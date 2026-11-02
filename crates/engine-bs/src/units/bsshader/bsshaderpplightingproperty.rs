//! `BSShader/bsshaderpplightingproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, ni_pointer_clear, release_reference, IMPORT_INTERLOCKED_DECREMENT,
    IMPORT_INTERLOCKED_INCREMENT, MEMORY_ALLOCATE, MEMORY_MANAGER, VECTOR_CONSTRUCT,
};

// `BSShaderPPLightingProperty` (Xbox PDB, size 0x110; 0x104 on PC, the size its
// `CreateObject` allocates). The PC offsets this file uses, read from the code (the Xbox PDB's
// offsets from `+0x84` on are 8 higher): `+0x20`/`+0x24` the shader flags, `+0x28` the alpha,
// `+0x2c` the fade alpha, `+0x30` the environment map scale, `+0x34` the camera distance,
// `+0x38` the last render pass state, `+0x3c`..`+0x54` the render pass arrays, `+0x58` the
// shader number, `+0x74` the "light list changed" byte, `+0x84`..`+0xa0` two colours, `+0xa4`
// the texture set, `+0xa8` the texture count, `+0xac` the six texture arrays (one per kind),
// `+0xc4`/`+0xcc` the two byte arrays, `+0xc8` the last texture index, `+0xd0` the tangent
// space data, `+0xd4`, `+0xd8` the clamp mode, `+0xdc` the texture effect data, `+0xe0`..
// `+0xec` four floats and `+0x100` the depth render pass.

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one stack word).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// The Ni allocation `00aa13e0` (`cdecl`, size) and the sized delete `00aa1460` (`cdecl`,
/// block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The `NiPointer` element constructor and destructor of the texture arrays (`006694e0`,
/// `0045cec0`).
const TEXTURE_POINTER_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const TEXTURE_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;

/// The four distance settings the fades read: `011f9454`/`011f9458` (near and far of the
/// first fade), `011f945c`/`011f9460` and `011f9464`/`011f9468` (the other two), and the
/// `011f946c`/`011f9470` pair `GetRenderPasses` uses.
const FADE_NEAR_011F9454: u32 = 0x011f_9454;
const FADE_FAR_011F9458: u32 = 0x011f_9458;
const FADE_NEAR_011F945C: u32 = 0x011f_945c;
const FADE_FAR_011F9460: u32 = 0x011f_9460;
const FADE_NEAR_011F9464: u32 = 0x011f_9464;
const FADE_FAR_011F9468: u32 = 0x011f_9468;
const FADE_NEAR_011F946C: u32 = 0x011f_946c;
const FADE_FAR_011F9470: u32 = 0x011f_9470;
/// The renderer singleton word (`011f4748`).
const RENDERER_011F4748: u32 = 0x011f_4748;
/// The shader quality word (`011f91c0`).
const SHADER_LEVEL_011F91C0: u32 = 0x011f_91c0;

/// `004a2020` (unnamed, engine map unit `bgsdecalmanager.cpp`, `thiscall`, one stack word: a
/// shader flag number; answers a byte).
const HAS_FLAG_004A2020: u32 = 0x004a_2020;
/// `NiAVObject::GetProperty` (engine map, `00a59d30`, `thiscall`, one stack word) and
/// `NiAVObject::UpdateProperties` (`00a5a040`, `thiscall`, none).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The `NiPointer` assignment `0066b0d0` (unnamed, `thiscall` on the pointer, one stack word).
const POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;

/// Sets `bit` in the flag word at `this + offset`, clearing the last render pass state
/// (`+0x38`) first when the bit was clear: the inlined `SetFlag(bit, true)`.
fn set_flag(e: &mut Engine, this: u32, offset: u32, bit: u32) {
    if e.mem.u32(this + offset) & bit == 0 {
        e.mem.set_u32(this + 0x38, 0);
    }
    let flags = e.mem.u32(this + offset) | bit;
    e.mem.set_u32(this + offset, flags);
}

/// Clears `bit` in the flag word at `this + offset`, clearing `+0x38` first when the bit was
/// set: the inlined `SetFlag(bit, false)`.
fn clear_flag(e: &mut Engine, this: u32, offset: u32, bit: u32) {
    if e.mem.u32(this + offset) & bit != 0 {
        e.mem.set_u32(this + 0x38, 0);
    }
    let flags = e.mem.u32(this + offset) & !bit;
    e.mem.set_u32(this + offset, flags);
}

/// An `NiPointer` at `dest` assigned the object the `NiPointer` at `source` holds, as the
/// copy code has it: nothing when they already match; otherwise the old object (when not
/// null) released, the source read again and stored, and its count incremented when not null.
fn assign_from(e: &mut Engine, dest: u32, source: u32) {
    let old = e.mem.u32(dest);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(dest, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

/// A texture array of `count` `NiPointer`s from the memory manager (`size` bytes: the count
/// word, then the elements), the count stored in front and the elements constructed (`eh
/// vector constructor iterator`, `006694e0`/`0045cec0`). Answers the elements, or 0 when the
/// allocation failed.
fn new_texture_array(e: &mut Engine, size: u32, count: u32) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            block + 4,
            4u32,
            count,
            TEXTURE_POINTER_CONSTRUCT_006694E0,
            TEXTURE_POINTER_DESTRUCT_0045CEC0
        ],
    );
    block + 4
}

/// The allocation size for a texture array of `count` elements, as the exe computes it
/// (`count * 4` then `+ 4`, each giving all ones on overflow).
fn texture_array_size(count: u32) -> u32 {
    count
        .checked_mul(4)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX)
}

/// A texture array destroyed when not null: its elements (count in the word before them)
/// through the `eh vector destructor iterator` with `0045cec0`, then the block freed.
fn delete_texture_array(e: &mut Engine, array: u32) {
    if array == 0 {
        return;
    }
    let count = e.mem.u32(array - 4);
    e.call(
        VECTOR_DESTRUCT,
        &args![array, 4u32, count, TEXTURE_POINTER_DESTRUCT_0045CEC0],
    );
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, array - 4]);
}

/// The memory manager's free on a block.
fn memory_free(e: &mut Engine, block: u32) {
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
}

/// Whether a texture is a normal-map-like kind: its `+0x24` (the texture's data) is not null,
/// that `+ 0x14` is not 0, and the word at `+0x18` of it is 5, 6 or 1.
fn texture_kind_flag(e: &Engine, texture: u32) -> Option<u8> {
    if texture == 0 {
        return None;
    }
    let data = e.mem.u32(texture + 0x24);
    if data == 0 || data.wrapping_add(0x14) == 0 {
        return None;
    }
    let kind = e.mem.u32(data.wrapping_add(0x18));
    Some(u8::from(kind == 5 || kind == 6 || kind == 1))
}

// Translated from 00b66760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): answers 1 when the word is 0xfc or 0x102,
/// else 0.
pub fn fn_00b66760(_e: &mut Engine, value: u32) -> u32 {
    u32::from(value == 0xfc || value == 0x102)
}

/// The vtables `fn_00b66850` stores (`010ae0c0`, then `0101dce4`) and the count it lowers
/// (`011f4400`, through `InterlockedDecrement`).
const VTABLE_010AE0C0: u32 = 0x010a_e0c0;
const VTABLE_0101DCE4: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;

// Translated from 00b66850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a destructor body): stores the vtable
/// `010ae0c0`; when the byte `+8` (owns its buffers) is set, frees the words `+0xc` and `+0x10`
/// through the memory manager (each when not null), clearing each after; stores the vtable
/// `0101dce4` and lowers the count `011f4400` (`InterlockedDecrement`).
pub fn fn_00b66850(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010AE0C0);
    for offset in [0xcu32, 0x10] {
        if e.mem.u8(base + 8) != 0 {
            let block = e.mem.u32(base + offset);
            if block != 0 {
                memory_free(e, block);
            }
        }
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base, VTABLE_0101DCE4);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00b66ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::LoadTextureSet` (Xbox PDB), `thiscall` with two stack words
/// (the first never read; the second a texture set, the property's own `+0xa4` when null):
/// clears the first `NiPointer` of each of the six texture arrays (`+0xac`); then, when the
/// set is not null, calls its slot +0x90 with each kind number 0..5 and that kind's array, and
/// when the first array's first texture is not null, its slot +0x8c with 0.
pub fn bs_shader_pp_lighting_property_load_texture_set(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    texture_set: u32,
) {
    let base = this.addr();
    let set = if texture_set != 0 {
        texture_set
    } else {
        e.mem.u32(base + 0xa4)
    };
    let arrays = base + 0xac;
    for kind in 0..6u32 {
        let array = e.mem.u32(arrays + kind * 4);
        let texture = e.mem.u32(array);
        if texture != 0 {
            release_reference(e, texture);
            e.mem.set_u32(array, 0);
        }
    }
    // The exe tests the arrays' address (`this + 0xac`), never 0 for a real object.
    if arrays == 0 || set == 0 {
        return;
    }
    for kind in 0..6u32 {
        let array = e.mem.u32(arrays + kind * 4);
        e.vcall(set, 0x90, &args![kind, array]);
    }
    let first = e.mem.u32(arrays);
    if e.mem.u32(first) != 0 {
        e.vcall(set, 0x8c, &args![0u32]);
    }
}

/// The fade the float functions compute between `near` and `far`, with the camera distance
/// `distance` (`+0x34`): 1.0 when `near` is above the distance, 0.0 when the distance is at
/// least `far`, otherwise `1 - min(ratio, 1)` (inlined) with `ratio = (distance - near) /
/// (far - near)` kept as a float. Each compare is the exe's: `near > distance` ordered, the
/// distance below `far` or unordered, `1 < ratio` ordered.
fn fade(distance: u32, near: u32, far: u32) -> f32 {
    let (distance_value, near_value, far_value) = (
        f32::from_bits(distance),
        f32::from_bits(near),
        f32::from_bits(far),
    );
    if near_value > distance_value {
        return 1.0;
    }
    if distance_value >= far_value {
        return 0.0;
    }
    let ratio = x87_store32(x87_div(
        x87_sub(x87_load32(distance), x87_load32(near)),
        x87_sub(x87_load32(far), x87_load32(near)),
    ));
    if 1.0 < f32::from_bits(ratio) {
        // The clamped ratio (1.0) is stored, then subtracted from 1.0.
        return f32::from_bits(x87_store32(x87_sub(1.0, 1.0)));
    }
    f32::from_bits(x87_store32(x87_sub(1.0, x87_m32(ratio))))
}

// Translated from 00b66b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers a float in `ST0`): 1.0
/// unless bit 0 of the flags `+0x20` is set, bit 0x800000 of `+0x24` is clear and the far
/// distance `011f9458` is above 0 (ordered); then the fade between `011f9454` and `011f9458`
/// at the camera distance `+0x34`.
pub fn fn_00b66b80(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let far = e.global::<u32>(FADE_FAR_011F9458);
    let enabled = e.mem.u32(base + 0x20) & 1 != 0
        && e.mem.u32(base + 0x24) & 0x0080_0000 == 0
        && 0.0 < f32::from_bits(far);
    if !enabled {
        return 1.0;
    }
    let near = e.global::<u32>(FADE_NEAR_011F9454);
    fade(e.mem.u32(base + 0x34), near, far)
}

/// `0040ebd0` (`cdecl`, two floats; answers a float): the helper `fn_00b66c40` uses for its
/// last fade.
const FLOAT_HELPER_0040EBD0: u32 = 0x0040_ebd0;

// Translated from 00b66c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers a float in `ST0`). With the
/// flags `+0x20` and bit 0x800000 of `+0x24` clear: when bit 0x20000 is set and `011f9468` is
/// above 0, the fade between `011f9464` and `011f9468`; otherwise when bit 0x80 is set and
/// `011f9460` is above 0, 1.0 when `011f945c` is above the distance, 0.0 when the distance is
/// at least `011f9460`, else `1 - f(1.0, ratio)` with the helper `0040ebd0`; otherwise 1.0.
pub fn fn_00b66c40(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let flags = e.mem.u32(base + 0x20);
    let far = e.global::<u32>(FADE_FAR_011F9460);
    let unlocked = |e: &Engine| e.mem.u32(base + 0x24) & 0x0080_0000 == 0;
    let second = flags & 0x80 != 0 && unlocked(e) && 0.0 < f32::from_bits(far);
    let first_far = e.global::<u32>(FADE_FAR_011F9468);
    let first = flags & 0x0002_0000 != 0 && unlocked(e) && f32::from_bits(first_far) > 0.0;
    let distance = e.mem.u32(base + 0x34);
    if first {
        let near = e.global::<u32>(FADE_NEAR_011F9464);
        return fade(distance, near, first_far);
    }
    if !second {
        return 1.0;
    }
    let near = e.global::<u32>(FADE_NEAR_011F945C);
    let (distance_value, near_value) = (f32::from_bits(distance), f32::from_bits(near));
    if near_value > distance_value {
        return 1.0;
    }
    if distance_value >= f32::from_bits(far) {
        return 0.0;
    }
    let ratio = x87_store32(x87_div(
        x87_sub(x87_load32(distance), x87_load32(near)),
        x87_sub(x87_load32(far), x87_load32(near)),
    ));
    let ratio = x87_store32(x87_load32(ratio));
    let picked = e.call(FLOAT_HELPER_0040EBD0, &args![1.0f32, ratio]).st0;
    f32::from_bits(x87_store32(x87_sub(1.0, picked)))
}

// Translated from 00b66db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): for each texture kind 0..5 and each
/// index below the count its slot +0xe8 answers (16 bits, asked again after each index), takes
/// the texture its slot +0xf4 answers for (kind, index) and, when not null, hands it to the
/// renderer (`011f4748`, read once) through the renderer's slot +0xe0 with (texture, 0, 0).
pub fn fn_00b66db0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let renderer = e.global::<u32>(RENDERER_011F4748);
    for kind in 0..6u32 {
        let mut index = 0u32;
        let count = e.vcall(base, 0xe8, &args![]).u32() as u16;
        if count == 0 {
            continue;
        }
        loop {
            let texture = e.vcall(base, 0xf4, &args![kind, index]).u32();
            if texture != 0 {
                e.vcall(renderer, 0xe0, &args![texture, 0u32, 0u32]);
            }
            index += 1;
            let count = e.vcall(base, 0xe8, &args![]).u32() as u16;
            if index as i32 >= i32::from(count) {
                break;
            }
        }
    }
}

/// `BSShaderProperty::RenderPass::RenderPass` (engine map, `00ba8ec0`, `cdecl`, six words).
const RENDER_PASS_CONSTRUCT: u32 = 0x00ba_8ec0;

// Translated from 00b66e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the geometry): the depth render pass
/// (`+0x100`), made on first use: 0x10 bytes from the memory manager made into a
/// `RenderPass` (`00ba8ec0` with the block, the geometry, the pass type, 1, 0, 0), the type
/// 0x1e8 when bit 2 of the flags `+0x20` is set, else 0x1e9 when bit 0x1000 is set, else 0x1e7
/// (null when the allocation fails). Answers the word `+0x100`.
// Translation incomplete: 00b66e20 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the allocation) is not translated.
pub fn fn_00b66e20(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x100) == 0 {
        let flags = e.mem.u32(base + 0x20);
        let kind = if flags & 2 != 0 {
            0x1e8u32
        } else if flags & 0x1000 != 0 {
            0x1e9
        } else {
            0x1e7
        };
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        let pass = if block != 0 {
            e.call(
                RENDER_PASS_CONSTRUCT,
                &args![block, geometry, kind, 1u32, 0u32, 0u32],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x100, pass);
    }
    e.mem.u32(base + 0x100)
}

// Translated from 00b66f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPointer`, no stack words): the `NiPointer`
/// destructor. When it holds an object, the object's count at `+4` goes down
/// (`InterlockedDecrement`) and at zero the object is deleted through its slot +4 (a tail
/// jump).
pub fn fn_00b66f20(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        release_reference(e, object);
    }
}

/// The base constructor `00b71720` (unnamed, engine map unit `bsshaderlightingproperty.cpp`,
/// `thiscall`, none) and the vtable `010ae0d0` of the property.
const LIGHTING_PROPERTY_CONSTRUCT_00B71720: u32 = 0x00b7_1720;
const VTABLE_010AE0D0: u32 = 0x010a_e0d0;
/// The byte `011fa00c` that makes new properties start with flag 0x2000, and the float
/// `0102918c` stored at `+0xe8`.
const DEFAULT_FLAG_011FA00C: u32 = 0x011f_a00c;
const INITIAL_E8_0102918C: u32 = 0x0102_918c;

// Translated from 00b66f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the `BSShaderPPLightingProperty`
/// constructor): the base constructor `00b71720`, the vtable `010ae0d0`, the floats
/// `+0x84`..`+0xa0` 0, the words `+0xa4`, `+0xd0`, `+0xdc` 0, the texture count `+0xa8` 2; for
/// each of the six kinds a texture array of that count (0 when the allocation fails) whose
/// held textures below the count are released; the byte arrays `+0xcc` (0, 0) and `+0xc4`
/// (0x1e, 0x1e) of two bytes; the `NiPointer`s `+0xd0` and `+0xdc` cleared, `+0xe4` 0, the float
/// `+0xe0` 0, `+0xd8` 3, `+0xd4` 0; flag 0x2000 set (clearing `+0x38` when it was clear) when
/// the byte `011fa00c` is set; `+0x100` 0, the float `+0xe8` from `0102918c`, `+0x1c` 8 and the
/// float `+0xec` 1.0. Answers `this`.
// Translation incomplete: 00b66f50 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the base and the arrays) is not translated.
pub fn fn_00b66f50(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(LIGHTING_PROPERTY_CONSTRUCT_00B71720, &args![base]);
    e.mem.set_u32(base, VTABLE_010AE0D0);
    for offset in (0x84u32..=0xa0).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0xa4, 0);
    e.mem.set_u32(base + 0xd0, 0);
    e.mem.set_u32(base + 0xdc, 0);
    e.mem.set_u16(base + 0xa8, 2);
    for kind in 0..6u32 {
        let count = u32::from(e.mem.u16(base + 0xa8));
        let array = new_texture_array(e, texture_array_size(count), count);
        let slot = base + 0xac + kind * 4;
        e.mem.set_u32(slot, array);
        if e.mem.u16(base + 0xa8) == 0 {
            continue;
        }
        let mut index = 0u32;
        loop {
            let element = e.mem.u32(slot).wrapping_add(index.wrapping_mul(4));
            let texture = e.mem.u32(element);
            if texture != 0 {
                release_reference(e, texture);
                e.mem.set_u32(element, 0);
            }
            index += 1;
            if index as i32 >= i32::from(e.mem.u16(base + 0xa8)) {
                break;
            }
        }
    }
    let bytes = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 2u32]).u32();
    e.mem.set_u32(base + 0xcc, bytes);
    e.mem.set_u8(bytes, 0);
    let bytes = e.mem.u32(base + 0xcc);
    e.mem.set_u8(bytes + 1, 0);
    let bytes = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 2u32]).u32();
    e.mem.set_u32(base + 0xc4, bytes);
    e.mem.set_u8(bytes, 0x1e);
    let bytes = e.mem.u32(base + 0xc4);
    e.mem.set_u8(bytes + 1, 0x1e);
    ni_pointer_clear(e, base + 0xd0);
    ni_pointer_clear(e, base + 0xdc);
    e.mem.set_u32(base + 0xe4, 0);
    e.mem.set_f32(base + 0xe0, 0.0);
    e.mem.set_u32(base + 0xd8, 3);
    e.mem.set_u32(base + 0xd4, 0);
    if e.global::<u8>(DEFAULT_FLAG_011FA00C) != 0 {
        set_flag(e, base, 0x20, 0x2000);
    }
    e.mem.set_u32(base + 0x100, 0);
    let value = e.global::<u32>(INITIAL_E8_0102918C);
    e.mem.set_u32(base + 0xe8, x87_quiet_bits(value));
    e.mem.set_u32(base + 0x1c, 8);
    e.mem.set_f32(base + 0xec, 1.0);
    this
}

// Translated from 00b671b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no register or stack word read): answers `011fa010` (a vtable
/// slot answering the address of a static object).
pub fn fn_00b671b0(_e: &mut Engine) -> u32 {
    0x011f_a010
}

// Translated from 00b671c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word `+0xd8` (the
/// texture clamp mode).
pub fn fn_00b671c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xd8)
}

// Translated from 00b671e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the 16-bit texture count
/// `+0xa8`.
pub fn fn_00b671e0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xa8)
}

// Translated from 00b671f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the 16-bit word `+0xc8`.
pub fn fn_00b671f0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xc8)
}

// Translated from 00b67200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three stack words: kind, index, texture): assigns
/// the texture to the `NiPointer` at that index of that kind's array (`+0xac`).
pub fn fn_00b67200(e: &mut Engine, this: Ptr, kind: u32, index: u32, texture: u32) {
    let array = e.mem.u32(
        this.addr()
            .wrapping_add(kind.wrapping_mul(4))
            .wrapping_add(0xac),
    );
    ni_pointer_assign(e, array.wrapping_add(index.wrapping_mul(4)), texture);
}

// Translated from 00b67260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: kind, index): answers the texture
/// at that index of that kind's array (`+0xac`).
pub fn fn_00b67260(e: &mut Engine, this: Ptr, kind: u32, index: u32) -> u32 {
    let array = e.mem.u32(
        this.addr()
            .wrapping_add(kind.wrapping_mul(4))
            .wrapping_add(0xac),
    );
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00b67290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: index, texture): assigns the
/// texture at that index of the first kind's array (`+0xac`).
pub fn fn_00b67290(e: &mut Engine, this: Ptr, index: u32, texture: u32) {
    let array = e.mem.u32(this.addr() + 0xac);
    ni_pointer_assign(e, array.wrapping_add(index.wrapping_mul(4)), texture);
}

// Translated from 00b672e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: index, texture): assigns the
/// texture at that index of the second kind's array (`+0xb0`).
pub fn fn_00b672e0(e: &mut Engine, this: Ptr, index: u32, texture: u32) {
    let array = e.mem.u32(this.addr() + 0xb0);
    ni_pointer_assign(e, array.wrapping_add(index.wrapping_mul(4)), texture);
}

/// `BSShaderProperty::RenderPass::~RenderPass` (engine map, `00ba8cd0`, `thiscall`) and the
/// base destructor `00b71410` (unnamed, `thiscall`).
const RENDER_PASS_DESTRUCT: u32 = 0x00ba_8cd0;
const LIGHTING_PROPERTY_DESTRUCT_00B71410: u32 = 0x00b7_1410;

// Translated from 00b67380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the `BSShaderPPLightingProperty`
/// destructor): stores the vtable `010ae0d0`; when the first texture array is not null,
/// releases and clears every held texture below the count `+0xa8` of each of the six arrays
/// and destroys each array, then frees the byte arrays `+0xcc` and `+0xc4`. Then clears the
/// `NiPointer`s `+0xdc` and `+0xd0` (`+0xd4` 0 and the float `+0xe0` 0 between), destroys the
/// depth render pass `+0x100` when not null (`~RenderPass`, then freed), runs the member
/// destructors of `+0xdc`, `+0xd0` and `+0xa4` (each released when not null) and the base
/// destructor `00b71410`.
// Translation incomplete: 00b67380 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the members) is not translated.
pub fn fn_00b67380(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010AE0D0);
    let arrays = base + 0xac;
    if e.mem.u32(arrays) != 0 {
        let mut index = 0u32;
        for kind in 0..6u32 {
            let slot = arrays + kind * 4;
            if e.mem.u16(base + 0xa8) != 0 {
                loop {
                    let element = e.mem.u32(slot).wrapping_add(index.wrapping_mul(4));
                    let texture = e.mem.u32(element);
                    if texture != 0 {
                        release_reference(e, texture);
                        e.mem.set_u32(element, 0);
                    }
                    index += 1;
                    if index as i32 >= i32::from(e.mem.u16(base + 0xa8)) {
                        break;
                    }
                }
            }
            let array = e.mem.u32(slot);
            index = 0;
            delete_texture_array(e, array);
        }
        let bytes = e.mem.u32(base + 0xcc);
        memory_free(e, bytes);
        let bytes = e.mem.u32(base + 0xc4);
        memory_free(e, bytes);
    }
    ni_pointer_clear(e, base + 0xdc);
    e.mem.set_u32(base + 0xd4, 0);
    e.mem.set_f32(base + 0xe0, 0.0);
    ni_pointer_clear(e, base + 0xd0);
    let pass = e.mem.u32(base + 0x100);
    if pass != 0 {
        e.call(RENDER_PASS_DESTRUCT, &args![pass]);
        memory_free(e, pass);
    }
    for offset in [0xdcu32, 0xd0, 0xa4] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release_reference(e, object);
        }
    }
    e.call(LIGHTING_PROPERTY_DESTRUCT_00B71410, &args![base]);
}

// Translated from 00b67580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the flags; the scalar deleting
/// destructor of the 0x14-byte object `fn_00b66850` destroys): runs `fn_00b66850` and, when bit
/// 0 is set, frees the object with size 0x14. Answers `this`.
pub fn fn_00b67580(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b66850(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

/// `00b702b0` (unnamed, `thiscall`, two stack words: the clone, the clone map).
const COPY_TO_00B702B0: u32 = 0x00b7_02b0;

// Translated from 00b67610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the clone map; the property's
/// `CreateClone`): allocates 0x104 bytes (`00aa13e0`), constructs a property in them
/// (`fn_00b66f50`, null when the allocation fails), runs `00b702b0` with (the clone, the
/// word), then the slot +0xd0 of `this` with the clone. Answers the clone.
// Translation incomplete: 00b67610 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b67610(e: &mut Engine, this: Ptr, clone_map: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x104u32]).u32();
    let clone = if block != 0 {
        fn_00b66f50(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(COPY_TO_00B702B0, &args![this, clone, clone_map]);
    e.vcall(this.addr(), 0xd0, &args![clone]);
    clone
}

// Translated from 00b676a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the destination property; the
/// property's `CopyMembers`): when the destination's texture count differs, its six texture
/// arrays are destroyed and made again with this count, its byte arrays freed and allocated
/// again with this count (one byte each), and the count copied. Then for each index below this
/// count, each kind's texture is assigned to the destination's and the two bytes copied; the
/// `NiPointer`s `+0xd0`, `+0xdc` and `+0xa4` assigned, and the words `+0xc8` (16 bits), `+0xd8`,
/// `+0xe4`, `+0xd4`, `+0x84`..`+0x90` and the floats `+0xe0`, `+0x30`, `+0xe8`, `+0xec` copied.
// Translation incomplete: 00b676a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b676a0(e: &mut Engine, this: Ptr, dest: Ptr) {
    let source = this.addr();
    let dest = dest.addr();
    if e.mem.u16(dest + 0xa8) != e.mem.u16(source + 0xa8) {
        for kind in 0..6u32 {
            let slot = dest + 0xac + kind * 4;
            let array = e.mem.u32(slot);
            delete_texture_array(e, array);
            let count = u32::from(e.mem.u16(source + 0xa8));
            let array = new_texture_array(e, texture_array_size(count), count);
            e.mem.set_u32(slot, array);
        }
        let bytes = e.mem.u32(dest + 0xcc);
        memory_free(e, bytes);
        let bytes = e.mem.u32(dest + 0xc4);
        memory_free(e, bytes);
        let count = u32::from(e.mem.u16(source + 0xa8));
        let bytes = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, count]).u32();
        e.mem.set_u32(dest + 0xcc, bytes);
        let count = u32::from(e.mem.u16(source + 0xa8));
        let bytes = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, count]).u32();
        e.mem.set_u32(dest + 0xc4, bytes);
        let count = e.mem.u16(source + 0xa8);
        e.mem.set_u16(dest + 0xa8, count);
    }
    if e.mem.u16(source + 0xa8) != 0 {
        let mut index = 0u32;
        loop {
            for kind in 0..6u32 {
                let offset = 0xac + kind * 4;
                let dest_slot = e.mem.u32(dest + offset).wrapping_add(index.wrapping_mul(4));
                let source_slot = e
                    .mem
                    .u32(source + offset)
                    .wrapping_add(index.wrapping_mul(4));
                assign_from(e, dest_slot, source_slot);
            }
            let byte = e.mem.u8(e.mem.u32(source + 0xcc).wrapping_add(index));
            let at = e.mem.u32(dest + 0xcc).wrapping_add(index);
            e.mem.set_u8(at, byte);
            let byte = e.mem.u8(e.mem.u32(source + 0xc4).wrapping_add(index));
            let at = e.mem.u32(dest + 0xc4).wrapping_add(index);
            e.mem.set_u8(at, byte);
            index += 1;
            if index as i32 >= i32::from(e.mem.u16(source + 0xa8)) {
                break;
            }
        }
    }
    assign_from(e, dest + 0xd0, source + 0xd0);
    let word = e.mem.u16(source + 0xc8);
    e.mem.set_u16(dest + 0xc8, word);
    let word = e.mem.u32(source + 0xd8);
    e.mem.set_u32(dest + 0xd8, word);
    assign_from(e, dest + 0xdc, source + 0xdc);
    let float = x87_quiet_bits(e.mem.u32(source + 0xe0));
    e.mem.set_u32(dest + 0xe0, float);
    for offset in [0xe4u32, 0xd4, 0x84, 0x88, 0x8c, 0x90] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(dest + offset, word);
    }
    for offset in [0x30u32, 0xe8, 0xec] {
        let float = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(dest + offset, float);
    }
    assign_from(e, dest + 0xa4, source + 0xa4);
}

/// `BSShaderManager::CreateTangentSpaceSimple` (engine map, `00b54b60`, `cdecl`, one word),
/// the RTTI `011f4aac` its geometry's skin must not have, the empty texture `011f9528`, and the
/// warning printer `005b5e40` (`cdecl`: format, name) with its two formats (`010ae238`,
/// `010ae1f0`).
const CREATE_TANGENT_SPACE_SIMPLE: u32 = 0x00b5_4b60;
const RTTI_011F4AAC: u32 = 0x011f_4aac;
const EMPTY_TEXTURE_011F9528: u32 = 0x011f_9528;
const WARN_005B5E40: u32 = 0x005b_5e40;
const FORMAT_010AE238: u32 = 0x010a_e238;
const FORMAT_010AE1F0: u32 = 0x010a_e1f0;

// Translated from 00b67a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::SetupGeometry` (Xbox PDB), `thiscall` with one stack word (the
/// geometry); answers `true`. Runs `UpdateProperties` when the geometry's `+0x9c` address is
/// null; sets flag 0x20 of `+0x24` when the geometry data's (`+0xb8`) word `+0x28` is not 0 or
/// the flag is already set, else clears it; makes the tangent space data
/// (`CreateTangentSpaceSimple`, assigned to `+0xd0` through `0066b0d0`) when the geometry has
/// data, the property none yet, flag 0x1000 is clear, the data's `+0x30` is null or not of RTTI
/// `011f4aac`, and the top nibble of the data's 16-bit `+0xc` is 0; with flag 0x100000 assigns
/// `011f9528` to the first texture and calls slot +0xe4 with 3; with flag 0x40000 takes the
/// geometry's property 0 (`GetProperty`) and, when found and `011f91c0` is below 5, drops bit
/// 0 of its 16-bit `+0x18` and sets its byte `+0x1a` to 0x80; calls slot +0x108; then prints
/// the warning `010ae238` (flags 0x800 and 0x10000000 clear, the fourth kind's array `+0xb8`
/// holding a first texture) or `010ae1f0` (either flag set, no such texture) with the
/// geometry's name (`+8`).
pub fn bs_shader_pp_lighting_property_setup_geometry(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
) -> bool {
    let base = this.addr();
    if geometry.wrapping_add(0x9c) == 0 {
        e.call(UPDATE_PROPERTIES, &args![geometry]);
    }
    let data = e.mem.u32(geometry + 0xb8);
    let already = e.mem.u32(base + 0x24) & 0x20 != 0;
    if e.mem.u32(data.wrapping_add(0x28)) != 0 || already {
        set_flag(e, base, 0x24, 0x20);
    } else {
        // Reached with the flag clear only (the jump at 00b67a7d skips the store).
        if already {
            e.mem.set_u32(base + 0x38, 0);
        }
        let flags = e.mem.u32(base + 0x24) & !0x20;
        e.mem.set_u32(base + 0x24, flags);
    }
    let data = e.mem.u32(geometry + 0xb8);
    if data != 0 && e.mem.u32(base + 0xd0) == 0 && e.mem.u32(base + 0x20) & 0x1000 == 0 {
        let skin = e.mem.u32(data + 0x30);
        let excluded = skin != 0 && e.vcall(skin, 8, &args![]).u32() == RTTI_011F4AAC;
        if !excluded && e.mem.u16(data + 0xc) & 0xf000 == 0 {
            let tangents = e.call(CREATE_TANGENT_SPACE_SIMPLE, &args![geometry]).u32();
            e.call(POINTER_ASSIGN_0066B0D0, &args![base + 0xd0, tangents]);
        }
    }
    if e.mem.u32(base + 0x20) & 0x0010_0000 != 0 {
        let first = e.mem.u32(base + 0xac);
        let empty = e.global::<u32>(EMPTY_TEXTURE_011F9528);
        ni_pointer_assign(e, first, empty);
        e.vcall(base, 0xe4, &args![3u32]);
    }
    if e.mem.u32(base + 0x20) & 0x0004_0000 != 0 {
        let property = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
        if property != 0 && e.global::<i32>(SHADER_LEVEL_011F91C0) < 5 {
            let bits = e.mem.u16(property + 0x18) & 0xfffe;
            e.mem.set_u16(property + 0x18, bits);
            e.mem.set_u8(property + 0x1a, 0x80);
        }
    }
    e.vcall(base, 0x108, &args![]);
    let flags = e.mem.u32(base + 0x20);
    let blended = flags & 0x800;
    if blended == 0 && flags & 0x1000_0000 == 0 {
        let second = e.mem.u32(base + 0xb8);
        if e.mem.u32(second) != 0 {
            let name = e.mem.u32(geometry + 8);
            e.call(WARN_005B5E40, &args![FORMAT_010AE238, name]);
            return true;
        }
    }
    if flags & 0x1000_0000 != 0 || blended != 0 {
        let second = e.mem.u32(base + 0xb8);
        if e.mem.u32(second) == 0 {
            let name = e.mem.u32(geometry + 8);
            e.call(WARN_005B5E40, &args![FORMAT_010AE1F0, name]);
        }
    }
    true
}

/// The callees of `GetRenderPasses` (engine map names where it has them; `thiscall` unless
/// said otherwise).
const CHECK_CREATE_DEPTH_MAP_PASSES: u32 = 0x00ba_9660;
const ADD_DEPTH_MAP_PASS_2X: u32 = 0x00bd_f650;
const CHECK_CREATE_CONST_ALPHA_PASSES: u32 = 0x00ba_9740;
const ADD_CONST_ALPHA_PASS_00BDA060: u32 = 0x00bd_a060;
const CHECK_CREATE_LOCAL_MAP_PASSES_00BA96D0: u32 = 0x00ba_96d0;
const ADD_LOCAL_MAP_PASS_2X: u32 = 0x00bd_e170;
const CHECK_CREATE_SILHOUETTE_PASSES_00BA9820: u32 = 0x00ba_9820;
const ADD_SILHOUETTE_PASS_00BDF6C0: u32 = 0x00bd_f6c0;
const CHECK_CREATE_RENDER_PASSES: u32 = 0x00ba_95e0;
/// `005bb9b0` (unnamed, `thiscall` on the accumulator's `+0x194`, one stack word: a vector it
/// fills), `00457990` (unnamed, `thiscall` on a vector: its length in `ST0`),
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, `cdecl`), `004b3ab0` (unnamed, `cdecl`, five
/// floats, answers in `ST0`), `BSShaderProperty::SetAlpha` (`00ba8ab0`, one float),
/// `BSShaderLightingProperty::ResortLights` (`00b70390`, one stack word) and `005bf7b0`
/// (unnamed, `cdecl`, answers a byte).
const CAMERA_POSITION_005BB9B0: u32 = 0x005b_b9b0;
const VECTOR_LENGTH_00457990: u32 = 0x0045_7990;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const INTERPOLATE_004B3AB0: u32 = 0x004b_3ab0;
const SET_ALPHA: u32 = 0x00ba_8ab0;
const RESORT_LIGHTS: u32 = 0x00b7_0390;
const DEPTH_PASSES_WANTED_005BF7B0: u32 = 0x005b_f7b0;
/// The default bound (`011f4288`) used when the geometry has none at `+0x20`.
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;

/// The byte the depth-pass adders get, written into the low byte of the caller's flag word as
/// the exe writes it into that argument's stack slot: 1 when the geometry's property 0 (or the
/// geometry's `+0x9c` word when there is none) exists and has bit 9 of its 16-bit `+0x18`, else
/// 0.
fn depth_flag(e: &mut Engine, geometry: u32, flag_word: u32) -> u32 {
    let mut property = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
    if property == 0 {
        property = e.mem.u32(geometry + 0x9c);
    }
    let set = property != 0 && (e.mem.u16(property + 0x18) >> 9) & 1 != 0;
    (flag_word & 0xffff_ff00) | u32::from(set)
}

/// The first pass of a pass array (`+4`, count `+0x10`) gets the renderer's `+8` object's
/// word `+0x1a0` stored at its `+0xc` target when the pass's byte `+9` is set.
fn refresh_first_pass(e: &mut Engine, array: u32) {
    let pass = e.mem.u32(e.mem.u32(array + 4));
    if e.mem.u8(pass + 9) != 0 {
        let renderer = e.global::<u32>(RENDERER_011F4748);
        let value = e.mem.u32(e.mem.u32(renderer + 8) + 0x1a0);
        let target = e.mem.u32(pass + 0xc);
        e.mem.set_u32(target, value);
    }
}

/// The threshold check of `GetRenderPasses` for one fade distance `limit`: the last render
/// pass state (`+0x38`) is cleared when the camera distance crossed it since the last call
/// (`limit` above the new distance and at most the old one, or at most the new one and above
/// the old one; every compare ordered).
fn clear_on_crossing(e: &mut Engine, this: u32, limit: f32, previous: f32) {
    let current = e.mem.f32(this + 0x34);
    if limit > current {
        if limit <= previous {
            e.mem.set_u32(this + 0x38, 0);
        }
        return;
    }
    if current >= limit && limit > previous {
        e.mem.set_u32(this + 0x38, 0);
    }
}

// Translated from 00b67be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::GetRenderPasses` (Xbox PDB), `thiscall` with six stack words:
/// the geometry, a pass word, the address of a 16-bit count, the pass type, the accumulator and
/// a flag word (its low byte). Answers a render pass array.
///
/// Types 7 (depth map, `+0x40`), 8 (constant alpha, `+0x44`), 9 (local map, `+0x48`) and 0xf
/// (`+0x54`) make their array and add their pass. Any other type rebuilds the main array
/// (`+0x3c`, key `type << 8 | word`): when one of the fades is on and the accumulator's bytes
/// `+0x164`/`+0x165` are clear, the camera distance `+0x34` is measured again (the bound
/// centre's distance to the camera, less its radius, scaled by the accumulator's `+0x110`) and
/// the state `+0x38` cleared when a fade distance was crossed; the alpha is set (with the
/// geometry's material alpha and, for the `011f946c`/`011f9470` fade, `004b3ab0`); then unless
/// the state matches the key, the lights are resorted when needed and the passes are added
/// through slot +0x114 (`011f91c0` is 1) or +0x118 (2 or more), a depth pass when `005bf7b0`
/// asks, the last pass marked, and one 0xfc/0x102 pass number lowered by one.
pub fn bs_shader_pp_lighting_property_get_render_passes(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    pass_word: u32,
    count_out: u32,
    pass_type: u32,
    accumulator: u32,
    flag_word: u32,
) -> u32 {
    let base = this.addr();
    let mut flag_word = flag_word;
    let properties = geometry.wrapping_add(0x9c);
    let measured_elsewhere =
        e.mem.u8(accumulator + 0x165) != 0 || e.mem.u8(accumulator + 0x164) != 0;
    let key = (pass_type << 8) | pass_word;
    if pass_type == 7 {
        let flags = e.mem.u32(base + 0x20);
        if flags & 0x8000 != 0 || flags & 0x0001_0000 != 0 {
            return 0;
        }
        e.call(CHECK_CREATE_DEPTH_MAP_PASSES, &args![base, 1u32]);
        let array = e.mem.u32(base + 0x40);
        if e.mem.u32(array + 0x10) != 0 {
            refresh_first_pass(e, array);
        } else if flag_word as u8 != 0 && properties != 0 {
            flag_word = depth_flag(e, geometry, flag_word);
            let has_skin = u32::from(e.mem.u32(geometry + 0xbc) != 0);
            e.call(
                ADD_DEPTH_MAP_PASS_2X,
                &args![base, geometry, has_skin, flag_word, accumulator],
            );
        }
        return e.mem.u32(base + 0x40);
    }
    if pass_type == 8 {
        let flags = e.mem.u32(base + 0x20);
        if flags & 0x8000 == 0 && flags & 0x0001_0000 == 0 && e.mem.u32(base + 0x24) & 0x10 == 0 {
            e.call(CHECK_CREATE_CONST_ALPHA_PASSES, &args![base, 1u32]);
            let array = e.mem.u32(base + 0x44);
            if e.mem.u32(array + 0x10) != 0 {
                refresh_first_pass(e, array);
            }
            let array = e.mem.u32(base + 0x44);
            if e.mem.u32(array + 0x10) == 0 && flag_word as u8 != 0 && properties != 0 {
                flag_word = depth_flag(e, geometry, flag_word);
                let has_skin = u32::from(e.mem.u32(geometry + 0xbc) != 0);
                e.call(
                    ADD_CONST_ALPHA_PASS_00BDA060,
                    &args![base, geometry, has_skin, flag_word],
                );
            }
        }
        return e.mem.u32(base + 0x44);
    }
    if pass_type == 9 {
        e.call(CHECK_CREATE_LOCAL_MAP_PASSES_00BA96D0, &args![base, 1u32]);
        let flags = e.mem.u32(base + 0x20);
        let two = u32::from(flags & 2 != 0);
        let empty = u32::from(flags & 0x0010_0000 != 0);
        e.call(ADD_LOCAL_MAP_PASS_2X, &args![base, geometry, two, empty]);
        return e.mem.u32(base + 0x48);
    }
    if pass_type == 0xf {
        e.call(CHECK_CREATE_SILHOUETTE_PASSES_00BA9820, &args![base, 1u32]);
        let array = e.mem.u32(base + 0x54);
        if e.mem.u32(array + 0x10) == 0 {
            let has_skin = u32::from(e.mem.u32(geometry + 0xbc) != 0);
            e.call(
                ADD_SILHOUETTE_PASS_00BDF6C0,
                &args![base, geometry, has_skin],
            );
        }
        return e.mem.u32(base + 0x54);
    }
    e.call(CHECK_CREATE_RENDER_PASSES, &args![base, u32::MAX]);
    let previous = f32::from_bits(x87_quiet_bits(e.mem.u32(base + 0x34)));
    let flags = e.mem.u32(base + 0x20);
    let second_fade = flags & 0x80 != 0 && 0.0 < e.global::<f32>(FADE_FAR_011F9460);
    let first_fade = flags & 0x0002_0000 != 0 && 0.0 < e.global::<f32>(FADE_FAR_011F9468);
    let main_fade = flags & 1 != 0 && 0.0 < e.global::<f32>(FADE_FAR_011F9458);
    let lod_on = flags & 0x0400_0000 != 0
        || e.call(HAS_FLAG_004A2020, &args![base, 0x1bu32]).u32() as u8 == 1;
    let lod_fade = lod_on
        && 0.0 < e.global::<f32>(FADE_FAR_011F9470)
        && e.call(HAS_FLAG_004A2020, &args![base, 0x23u32]).u32() as u8 == 0;
    let mut lod_fade_kept = lod_fade;
    let mut bound = e.mem.u32(geometry + 0x20);
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        // The bound copy (centre and radius) at `frame + 0x20`, the camera position at `+0`,
        // the difference at `+0xc`.
        for word in 0..4u32 {
            let value = e.mem.u32(bound + word * 4);
            e.mem.set_u32(frame + 0x20 + word * 4, value);
        }
        if (first_fade || second_fade || main_fade || lod_fade) && !measured_elsewhere {
            let camera = e.mem.u32(accumulator + 0x194);
            e.call(CAMERA_POSITION_005BB9B0, &args![camera, frame]);
            for axis in 0..3u32 {
                let centre = e.mem.u32(frame + 0x20 + axis * 4);
                let eye = e.mem.u32(frame + axis * 4);
                let difference = x87_store32(x87_sub(x87_load32(centre), x87_m32(eye)));
                e.mem.set_u32(frame + 0xc + axis * 4, difference);
            }
            let length = e.call(VECTOR_LENGTH_00457990, &args![frame + 0xc]).st0;
            let radius = e.mem.u32(frame + 0x2c);
            let distance = x87_store32(x87_sub(length, x87_m32(radius)));
            e.mem.set_u32(base + 0x34, distance);
            let accumulated = e.call(GET_ACCUMULATOR, &args![]).u32();
            if e.mem.u32(accumulated + 8) != 0 {
                let accumulated = e.call(GET_ACCUMULATOR, &args![]).u32();
                let shader = e.mem.u32(accumulated + 8);
                let scale = e.mem.u32(shader + 0x110);
                let distance =
                    x87_store32(x87_mul(x87_load32(scale), x87_m32(e.mem.u32(base + 0x34))));
                e.mem.set_u32(base + 0x34, distance);
            }
            let current = e.mem.f32(base + 0x34);
            if previous != current {
                if first_fade {
                    clear_on_crossing(e, base, e.global::<f32>(FADE_FAR_011F9468), previous);
                } else if second_fade {
                    clear_on_crossing(e, base, e.global::<f32>(FADE_FAR_011F9460), previous);
                } else if lod_fade {
                    clear_on_crossing(e, base, e.global::<f32>(FADE_FAR_011F9470), previous);
                }
                if main_fade {
                    clear_on_crossing(e, base, e.global::<f32>(FADE_FAR_011F9458), previous);
                }
            }
        }
        if properties != 0 {
            let material = e.mem.u32(geometry + 0xa4);
            if material == 0 {
                let alpha = x87_quiet_bits(e.mem.u32(base + 0x2c));
                e.call(SET_ALPHA, &args![base, alpha]);
            } else {
                let alpha = x87_store32(x87_mul(
                    x87_load32(e.mem.u32(material + 0x3c)),
                    x87_m32(e.mem.u32(base + 0x2c)),
                ));
                let mut chosen = alpha;
                if lod_fade {
                    let far = e.global::<u32>(FADE_FAR_011F9470);
                    let current = e.mem.f32(base + 0x34);
                    if f32::from_bits(far) != 0.0 && current >= f32::from_bits(far) {
                        lod_fade_kept = false;
                    } else {
                        let near = e.global::<u32>(FADE_NEAR_011F946C);
                        let (near_value, far_value) = (f32::from_bits(near), f32::from_bits(far));
                        if near_value != 0.0
                            && 0.0 != far_value
                            && current >= near_value
                            && current < far_value
                        {
                            let current_bits = x87_quiet_bits(e.mem.u32(base + 0x34));
                            let faded = e
                                .call(
                                    INTERPOLATE_004B3AB0,
                                    &args![
                                        0u32,
                                        1.0f32,
                                        x87_quiet_bits(far),
                                        x87_quiet_bits(near),
                                        current_bits
                                    ],
                                )
                                .st0;
                            chosen = x87_store32(faded);
                        }
                    }
                }
                e.call(SET_ALPHA, &args![base, x87_quiet_bits(chosen)]);
            }
        }
        if e.mem.u32(base + 0x38) == key && flag_word as u8 != 0 && e.mem.u8(base + 0x74) == 0 {
            return;
        }
        e.mem.set_u8(base + 0x74, 0);
        let has = |e: &mut Engine, number: u32| {
            e.call(HAS_FLAG_004A2020, &args![base, number]).u32() as u8 != 0
        };
        let resort = ((e.mem.u32(base + 0x20) & 0x0400_0000 != 0 || has(e, 5))
            && (has(e, 0xb) || has(e, 0x1c)))
            || has(e, 5)
            || e.mem.u8(e.mem.u32(geometry + 0x9c) + 0x18) & 1 != 0;
        if resort {
            e.call(RESORT_LIGHTS, &args![base, frame + 0x20]);
        }
        let flag = flag_word as u8;
        if flag == 1 {
            let passes = e.mem.u32(base + 0x3c);
            e.mem.set_u32(passes + 0x10, 0);
            if e.mem.f32(base + 0x28) == 0.0 {
                return;
            }
            if lod_fade && !lod_fade_kept {
                return;
            }
            e.mem.set_u32(base + 0x38, key);
        } else {
            e.mem.set_u16(count_out, 0);
        }
        let level = e.global::<i32>(SHADER_LEVEL_011F91C0);
        let slot = if level == 1 {
            Some(0x114u32)
        } else if level >= 2 {
            Some(0x118)
        } else {
            None
        };
        if let Some(slot) = slot {
            let lights = e.mem.u32(accumulator + 0x19c);
            e.vcall(
                base,
                slot,
                &args![
                    geometry,
                    pass_word,
                    count_out,
                    lights,
                    accumulator,
                    flag_word
                ],
            );
        }
        let depth = e.call(DEPTH_PASSES_WANTED_005BF7B0, &args![]).u32() as u8 != 0;
        let mut mark_last = true;
        if depth {
            if flag == 0 {
                mark_last = false;
            } else if properties != 0 {
                let depth_word = depth_flag(e, geometry, flag_word);
                let skin = e.mem.u32(geometry + 0xbc);
                e.call(CHECK_CREATE_DEPTH_MAP_PASSES, &args![base, 1u32]);
                let array = e.mem.u32(base + 0x40);
                e.mem.set_u32(array + 0x10, 0);
                e.call(
                    ADD_DEPTH_MAP_PASS_2X,
                    &args![
                        base,
                        geometry,
                        u32::from(skin != 0),
                        depth_word,
                        accumulator
                    ],
                );
            }
        }
        if mark_last && flag != 0 {
            let passes = e.mem.u32(base + 0x3c);
            let count = e.mem.u32(passes + 0x10);
            if count > 0 {
                let list = e.mem.u32(passes + 4);
                let last = e
                    .mem
                    .u32(list.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
                if last != 0 {
                    e.mem.set_u8(last + 8, 1);
                }
            }
        }
        let passes = e.mem.u32(base + 0x3c);
        let count = e.mem.u32(passes + 0x10);
        if count > 1 {
            let first = e.mem.u32(e.mem.u32(passes + 4));
            let number = u32::from(e.mem.u16(first + 4));
            if fn_00b66760(e, number) as u8 != 0 {
                let lowered = e.mem.u16(first + 4).wrapping_add(0xffff);
                e.mem.set_u16(first + 4, lowered);
                return;
            }
        }
        if count > 2 {
            let mut list_owner = passes;
            let mut index = 0u32;
            loop {
                let pass = e.mem.u32(e.mem.u32(list_owner + 4).wrapping_add(index * 4));
                let number = e.mem.u16(pass + 4);
                if number == 0xfc || number == 0x102 {
                    e.mem.set_u16(pass + 4, number.wrapping_sub(1));
                    list_owner = e.mem.u32(base + 0x3c);
                    index = e.mem.u32(list_owner + 0x10);
                }
                index = index.wrapping_add(1);
                let count = e.mem.u32(e.mem.u32(base + 0x3c) + 0x10);
                if index >= count {
                    break;
                }
            }
        }
    });
    e.mem.u32(base + 0x3c)
}

// Translated from 00b68450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): grows the texture arrays to ten
/// when the count `+0xa8` is below 10. For each kind a new array of ten (0x2c bytes; its held
/// textures below the old count released), then each old texture below the old count is
/// assigned to the new array, the old array destroyed and the new one stored. The byte arrays
/// `+0xcc` and `+0xc4` are freed and allocated again with ten bytes (their contents not
/// copied), and the count set to 10.
// Translation incomplete: 00b68450 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b68450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u16(base + 0xa8) >= 0xa {
        return;
    }
    let mut fresh = [0u32; 6];
    for slot in fresh.iter_mut() {
        let array = new_texture_array(e, 0x2c, 0xa);
        *slot = array;
        if e.mem.u16(base + 0xa8) != 0 {
            let mut element = array;
            let mut index = 0u32;
            loop {
                let texture = e.mem.u32(element);
                if texture != 0 {
                    release_reference(e, texture);
                    e.mem.set_u32(element, 0);
                }
                index += 1;
                element = element.wrapping_add(4);
                if index as i32 >= i32::from(e.mem.u16(base + 0xa8)) {
                    break;
                }
            }
        }
    }
    for (kind, array) in fresh.iter().enumerate() {
        let slot = base + 0xac + kind as u32 * 4;
        if e.mem.u16(base + 0xa8) != 0 {
            let mut index = 0u32;
            loop {
                let dest = array.wrapping_add(index * 4);
                let source = e.mem.u32(slot).wrapping_add(index * 4);
                assign_from(e, dest, source);
                index += 1;
                if index as i32 >= i32::from(e.mem.u16(base + 0xa8)) {
                    break;
                }
            }
        }
        let old = e.mem.u32(slot);
        delete_texture_array(e, old);
        e.mem.set_u32(slot, *array);
    }
    let bytes = e.mem.u32(base + 0xcc);
    memory_free(e, bytes);
    let bytes = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xau32])
        .u32();
    let old = e.mem.u32(base + 0xc4);
    e.mem.set_u32(base + 0xcc, bytes);
    memory_free(e, old);
    let bytes = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xau32])
        .u32();
    e.mem.set_u32(base + 0xc4, bytes);
    e.mem.set_u16(base + 0xa8, 0xa);
}

// Translated from 00b68660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: an index and a texture set): grows
/// the arrays to ten when needed (`fn_00b68450`), clears both bytes at the index; for each kind
/// loads the texture at the index from the set (its slot +0x90 with the kind and the element's
/// address) or, without a set, releases and clears it; for kind 1 sets the byte `+0xcc` at the
/// index to the texture's normal-map kind (5, 6 or 1). Then the 16-bit `+0xc8` becomes one less
/// than the number of leading non-null textures of the first kind (at most 9), or 0.
pub fn fn_00b68660(e: &mut Engine, this: Ptr, index: u32, texture_set: u32) {
    let base = this.addr();
    if e.mem.u16(base + 0xa8) < 0xa {
        fn_00b68450(e, this);
    }
    let bytes = e.mem.u32(base + 0xcc);
    e.mem.set_u8(index.wrapping_add(bytes), 0);
    let bytes = e.mem.u32(base + 0xc4);
    e.mem.set_u8(index.wrapping_add(bytes), 0);
    for kind in 0..6u32 {
        let element = e
            .mem
            .u32(base + 0xac + kind * 4)
            .wrapping_add(index.wrapping_mul(4));
        if texture_set != 0 {
            e.vcall(texture_set, 0x90, &args![kind, element]);
        } else {
            ni_pointer_clear(e, element);
        }
        if kind == 1 {
            let array = e.mem.u32(base + 0xac + kind * 4);
            let texture = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
            if let Some(flag) = texture_kind_flag(e, texture) {
                let bytes = e.mem.u32(base + 0xcc);
                e.mem.set_u8(index.wrapping_add(bytes), flag);
            }
        }
    }
    let first = e.mem.u32(base + 0xac);
    let mut leading = 0u16;
    loop {
        if e.mem.u32(first.wrapping_add(u32::from(leading) * 4)) == 0 {
            break;
        }
        leading += 1;
        if leading >= 9 {
            break;
        }
    }
    let last = if leading > 0 { leading - 1 } else { 0 };
    e.mem.set_u16(base + 0xc8, last);
}

/// The callees of `fn_00b68880`: the specular/skin shader property constructor `00bb4540`
/// (unnamed, `thiscall`, one stack word), `004391c0` (unnamed, engine map unit
/// `multiboundmarkerdata.cpp`, `thiscall`), `0045bad0` (unnamed, `cdecl`, two words) and its two
/// first words `012007b0` and `011f3728`; the bytes `011f94a6` and `011f9422`.
const SKIN_PROPERTY_CONSTRUCT_00BB4540: u32 = 0x00bb_4540;
const PROPERTY_CONSTRUCT_004391C0: u32 = 0x0043_91c0;
const LIST_HAS_0045BAD0: u32 = 0x0045_bad0;
const LIST_012007B0: u32 = 0x0120_07b0;
const LIST_011F3728: u32 = 0x011f_3728;
const SWITCH_011F94A6: u32 = 0x011f_94a6;
const SWITCH_011F9422: u32 = 0x011f_9422;

/// Allocates 0x108 bytes (`00aa13e0`) and, when that succeeds, runs `00bb4540` on them with
/// the property; answers the result (0 when the allocation fails).
fn make_skin_property(e: &mut Engine, this: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x108u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(SKIN_PROPERTY_CONSTRUCT_00BB4540, &args![block, this])
        .u32()
}

// Translated from 00b68880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three stack words of which only the first, the
/// geometry, is read): picks the shader number `+0x58` or makes a replacement property.
/// No geometry, or one whose slot +0x30 answers non-zero: `+0x58` 0, answers 0. Flag 2 of
/// `+0x24` or 0x4000 of `+0x20`: `+0x58` 1, answers 0. With flag 0x800: when `011f94a6` is set
/// and flag 0x1c (`004a2020`), the list `012007b0` (`0045bad0`) or `011f9422` holds, answers a new
/// `00bb4540` property; else `+0x58` 0xf and flag 0x20 set when flag 0x1a or 0x1b holds. Without
/// it: flag 0x12 makes the new property; else flag 0x13, the geometry's property 0 (made with
/// `004391c0`, bit 0 of its 16-bit `+0x18` cleared, when missing and flag 0x13 holds) decides
/// between flag 0x20 and a new property (`011f9422` set); flags 0x1a/0x1b make one or set flag
/// 0x20; and with `011f94a6` set each controller of the geometry (`+0xc`, next `+0x30`) goes to
/// `0045bad0` with `011f3728`. Answers 0 or the new property.
// Translation incomplete: 00b68880 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the allocations) is not translated.
pub fn fn_00b68880(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    let base = this.addr();
    let has = |e: &mut Engine, number: u32| {
        e.call(HAS_FLAG_004A2020, &args![base, number]).u32() as u8 != 0
    };
    if geometry == 0 || e.vcall(geometry, 0x30, &args![]).u32() != 0 {
        e.mem.set_u32(base + 0x58, 0);
        return 0;
    }
    if geometry.wrapping_add(0x9c) == 0 {
        e.call(UPDATE_PROPERTIES, &args![geometry]);
    }
    if e.mem.u32(base + 0x24) & 2 != 0 || e.mem.u32(base + 0x20) & 0x4000 != 0 {
        e.mem.set_u32(base + 0x58, 1);
        return 0;
    }
    if e.mem.u32(base + 0x20) & 0x800 != 0 {
        if e.global::<u8>(SWITCH_011F94A6) != 0
            && (has(e, 0x1c)
                || e.call(LIST_HAS_0045BAD0, &args![LIST_012007B0, base]).u32() as u8 != 0
                || e.global::<u8>(SWITCH_011F9422) != 0)
        {
            return make_skin_property(e, base);
        }
        e.mem.set_u32(base + 0x58, 0xf);
        if has(e, 0x1a) || has(e, 0x1b) {
            set_flag(e, base, 0x20, 0x20);
        }
        return 0;
    }
    if has(e, 0x12) {
        return make_skin_property(e, base);
    }
    let wants = has(e, 0x13);
    let mut property = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
    let mut skip_to_end = false;
    if property == 0 {
        if !wants {
            skip_to_end = true;
        } else {
            let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
            property = if block != 0 {
                e.call(PROPERTY_CONSTRUCT_004391C0, &args![block]).u32()
            } else {
                0
            };
            let bits = e.mem.u16(property.wrapping_add(0x18)) & 0xfffe;
            e.mem.set_u16(property.wrapping_add(0x18), bits);
        }
    }
    if !skip_to_end {
        if e.mem.u8(property + 0x18) & 1 != 0 || has(e, 0x1a) || has(e, 0x1b) {
            set_flag(e, base, 0x20, 0x20);
        }
        if e.mem.u8(property + 0x18) & 1 != 0 && e.global::<u8>(SWITCH_011F9422) != 0 {
            return make_skin_property(e, base);
        }
    }
    if has(e, 0x1a) || has(e, 0x1b) {
        if e.global::<u8>(SWITCH_011F9422) != 0 {
            return make_skin_property(e, base);
        }
        set_flag(e, base, 0x20, 0x20);
    }
    if e.global::<u8>(SWITCH_011F94A6) != 0 {
        let mut controller = e.mem.u32(geometry + 0xc);
        while controller != 0 {
            e.call(LIST_HAS_0045BAD0, &args![LIST_011F3728, controller]);
            controller = e.mem.u32(controller + 0x30);
        }
    }
    0
}

// Translated from 00b68b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): unless flag 0x20000 of `+0x20` is
/// set, sets flag 1 when flag 0x80 is set or the second kind's first texture is of a
/// normal-map kind (5, 6 or 1), else clears it; reads the second array's second word (unused);
/// and sets the byte `+0xcc` of each index below the count to its texture's kind flag (when
/// the texture and its data are there). Then clears flag 0x100 (and `+0x38` when it was set).
pub fn fn_00b68b10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x20) & 0x0002_0000 == 0 {
        let mut wanted = u8::from(e.mem.u32(base + 0x20) & 0x80 != 0);
        let first = e.mem.u32(e.mem.u32(base + 0xb0));
        if let Some(flag) = texture_kind_flag(e, first) {
            wanted |= flag;
        }
        if wanted != 0 {
            set_flag(e, base, 0x20, 1);
        } else {
            clear_flag(e, base, 0x20, 1);
        }
        let second = e.mem.u32(base + 0xb0);
        let _ = e.mem.u32(second + 4);
        if e.mem.u16(base + 0xa8) != 0 {
            let mut index = 0u32;
            loop {
                let texture = e.mem.u32(e.mem.u32(base + 0xb0).wrapping_add(index * 4));
                if let Some(flag) = texture_kind_flag(e, texture) {
                    let bytes = e.mem.u32(base + 0xcc);
                    e.mem.set_u8(index.wrapping_add(bytes), flag);
                }
                index += 1;
                if index as i32 >= i32::from(e.mem.u16(base + 0xa8)) {
                    break;
                }
            }
        }
    }
    clear_flag(e, base, 0x20, 0x100);
}

/// The default texture (`011f9510`) `fn_00b68d30` sets.
const DEFAULT_TEXTURE_011F9510: u32 = 0x011f_9510;

// Translated from 00b68d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): when the second kind's first
/// texture is null, calls slot +0x100 with (0, the texture `011f9510`).
pub fn fn_00b68d30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let second = e.mem.u32(base + 0xb0);
    if e.mem.u32(second) == 0 {
        let texture = e.global::<u32>(DEFAULT_TEXTURE_011F9510);
        e.vcall(base, 0x100, &args![0u32, texture]);
    }
}

// Translated from 00b68d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; the property's `CreateObject`): allocates
/// 0x104 bytes (`00aa13e0`) and constructs a property in them (`fn_00b66f50`); answers it, or
/// 0 when the allocation fails.
// Translation incomplete: 00b68d50 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b68d50(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x104u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00b66f50(e, Ptr::new(block)).addr()
}

/// The base `LoadBinary` `00b70260` (unnamed, `thiscall`, one stack word) and the stream's
/// `00a645b0` (unit `nistream.cpp`, `thiscall`, none).
const BASE_LOAD_BINARY_00B70260: u32 = 0x00b7_0260;
const STREAM_STEP_00A645B0: u32 = 0x00a6_45b0;

/// One read through the stream's reader (`+0x24c`): its function at `+8` called with (reader,
/// destination, size, the address of the count word, 1) and the answer added to the reader's
/// `+4`.
fn stream_read(e: &mut Engine, stream: u32, dest: u32, size: u32, count: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read = e.mem.u32(reader + 8);
    let done = e.call(read, &args![reader, dest, size, count, 1u32]).u32();
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
}

// Translated from 00b68dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the stream; the property's
/// `LoadBinary`): the base `00b70260`, then reads four bytes into `+0xd8`, runs `00a645b0`,
/// reads one byte into the argument's stack slot (dropped) when the stream's version `+4` is
/// below 0x17, and reads four bytes each into `+0xe0`, `+0xe4`, `+0xe8` and `+0xec`. The count
/// words handed to the reader are the argument's stack slot (4) and a local (1).
pub fn fn_00b68dc0(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(BASE_LOAD_BINARY_00B70260, &args![base, stream]);
    e.with_stack(8, |e, frame| {
        let slot = frame.addr();
        let local = slot + 4;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(slot, 4);
        stream_read(e, stream, base + 0xd8, 4, slot);
        e.call(STREAM_STEP_00A645B0, &args![stream]);
        if e.mem.u32(stream + 4) < 0x17 {
            e.mem.set_u32(local, 1);
            stream_read(e, stream, slot, 1, local);
        }
        for offset in [0xe0u32, 0xe4, 0xe8, 0xec] {
            e.mem.set_u32(slot, 4);
            stream_read(e, stream, base + offset, 4, slot);
        }
    });
}

// Translated from 00b68fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word: the flags; the property's scalar
/// deleting destructor): runs `fn_00b67380` and, when bit 0 is set, frees the object with size
/// 0x104. Answers `this`.
pub fn fn_00b68fb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b67380(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x104u32]);
    }
    this
}

/// `BSShaderProperty::PostLinkObject` (engine map, `00ba8be0`, `thiscall`, one stack word),
/// `BSShaderTextureSet::BSShaderTextureSet` (`0051d0d0`, `thiscall`) and the double `01012060`
/// the float `+0xe0` is kept above.
const SHADER_PROPERTY_POST_LINK_OBJECT: u32 = 0x00ba_8be0;
const TEXTURE_SET_CONSTRUCT: u32 = 0x0051_d0d0;
const LOWER_LIMIT_01012060: u32 = 0x0101_2060;

// Translated from 00b68fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::PostLinkObject` (Xbox PDB), `thiscall` with one stack word (the
/// stream): `BSShaderProperty::PostLinkObject`; unless flag 0x1000000 of `+0x20` is set, makes
/// the texture set `+0xa4` when there is none (0x38 bytes, `BSShaderTextureSet`, assigned
/// through `0066b0d0`) and runs `LoadTextureSet` with (the stream's `+0xe0`, 0). Then keeps the
/// float `+0xe0` between the double `01012060` (0.0 below it) and 1.0 (also for an unordered
/// value).
// Translation incomplete: 00b68fe0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_pp_lighting_property_post_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(SHADER_PROPERTY_POST_LINK_OBJECT, &args![base, stream]);
    if e.mem.u32(base + 0x20) & 0x0100_0000 == 0 {
        if e.mem.u32(base + 0xa4) == 0 {
            let block = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
            let set = if block != 0 {
                e.call(TEXTURE_SET_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.call(POINTER_ASSIGN_0066B0D0, &args![base + 0xa4, set]);
        }
        bs_shader_pp_lighting_property_load_texture_set(e, this, stream + 0xe0, 0);
    }
    let value = x87_quiet_bits(e.mem.u32(base + 0xe0));
    let limit = e.global::<f64>(LOWER_LIMIT_01012060);
    let kept = if f64::from(f32::from_bits(value)) < limit {
        0.0f32.to_bits()
    } else if 1.0 > f32::from_bits(value) {
        value
    } else {
        1.0f32.to_bits()
    };
    e.mem.set_u32(base + 0xe0, kept);
}

/// The callees of `fn_00b69be0` (engine map names): `NiGeometryData::ReplaceData` (`00a66bb0`,
/// `thiscall`, seven stack words), `NiObjectNET::GetExtraData` (`00a5bdd0`, one),
/// `NiAdditionalGeometryData::NiAdditionalGeometryData` (`00537bf0`, one), its
/// `SetDataBlockCount` (`00a72ca0`), `SetDataStreamCount` (`00a730e0`), `SetDataBlock`
/// (`00a72bb0`, four), `SetDataStream` (`00a726a0`, seven) and `NiObjectNET::RemoveExtraData`
/// (`00a5be90`, one); the extra data name `011f94bc`.
const REPLACE_DATA: u32 = 0x00a6_6bb0;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const ADDITIONAL_DATA_CONSTRUCT: u32 = 0x0053_7bf0;
const SET_DATA_BLOCK_COUNT: u32 = 0x00a7_2ca0;
const SET_DATA_STREAM_COUNT: u32 = 0x00a7_30e0;
const SET_DATA_BLOCK: u32 = 0x00a7_2bb0;
const SET_DATA_STREAM: u32 = 0x00a7_26a0;
const REMOVE_EXTRA_DATA: u32 = 0x00a5_be90;
const EXTRA_DATA_NAME_011F94BC: u32 = 0x011f_94bc;

// Translated from 00b69be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSShaderPPLightingProperty::CreateRendererSpecificProperty` in the engine map (from a
/// call graph; unconfirmed), `thiscall` with one stack word (the geometry). With tangent space
/// data (`+0xd0`, both its `+0xc` and `+0x10` set) and a geometry data (`+0xb8`) whose top nibble
/// of `+0xc` is 0: a buffer of three times the vertices' normals (count `+8`, 12 bytes each)
/// gets the normals (`+0x24`), then the data's `+0x10` and `+0xc` arrays; the old normals are
/// freed and `ReplaceData` installs it. Then, when the geometry has an extra data list (`+0x18`)
/// with the entry `011f94bc`: its `+0xc` goes to `+0xd4` (flag 0x4000 set when not 0, cleared
/// otherwise), a new `NiAdditionalGeometryData` (0x2c bytes) gets two blocks and streams (block
/// 1 the entry's data, `count << 5` bytes; streams 0 and 1 of four floats at 0 and 0x10, stride
/// 0x20), the entry's `+0x10` and `+0xc` are cleared and it is removed. Answers the new data, or
/// 0.
// Translation incomplete: 00b69be0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b69be0(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let base = this.addr();
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = u32::from(e.mem.u16(data + 8));
    let tangents = e.mem.u32(base + 0xd0);
    if tangents != 0
        && e.mem.u32(tangents + 0xc) != 0
        && e.mem.u32(tangents + 0x10) != 0
        && e.mem.u16(data + 0xc) & 0xf000 == 0
    {
        let size = (vertices * 3).checked_mul(0xc).unwrap_or(u32::MAX);
        let buffer = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        let first_array = e.mem.u32(data + 0x24);
        let tangents = e.mem.u32(base + 0xd0);
        let binormals = e.mem.u32(tangents + 0x10);
        let tangent_list = e.mem.u32(tangents + 0xc);
        let part = vertices * 12;
        e.call(MEMCPY, &args![buffer, first_array, part]);
        e.call(MEMCPY, &args![buffer + part, binormals, part]);
        e.call(MEMCPY, &args![buffer + vertices * 24, tangent_list, part]);
        memory_free(e, first_array);
        let keep = u32::from(e.mem.u16(data + 0xc) & 1);
        let (word_2c, word_28, word_20) = (
            e.mem.u32(data + 0x2c),
            e.mem.u32(data + 0x28),
            e.mem.u32(data + 0x20),
        );
        e.call(
            REPLACE_DATA,
            &args![data, vertices, word_20, buffer, word_28, word_2c, keep, 0x1000u32],
        );
    }
    let extra = e.mem.u32(geometry + 0x18);
    if extra == 0 {
        return 0;
    }
    let name = e.global::<u32>(EXTRA_DATA_NAME_011F94BC);
    let entry = e.call(GET_EXTRA_DATA, &args![extra, name]).u32();
    if entry == 0 {
        return 0;
    }
    let block_data = e.mem.u32(entry + 0xc);
    e.mem.set_u32(base + 0xd4, block_data);
    if block_data != 0 {
        set_flag(e, base, 0x20, 0x4000);
    } else {
        clear_flag(e, base, 0x20, 0x4000);
    }
    let block = e.call(NI_ALLOCATE, &args![0x2cu32]).u32();
    let added = if block != 0 {
        e.call(ADDITIONAL_DATA_CONSTRUCT, &args![block, vertices])
            .u32()
    } else {
        0
    };
    e.call(SET_DATA_BLOCK_COUNT, &args![added, 2u32]);
    e.call(SET_DATA_STREAM_COUNT, &args![added, 2u32]);
    let block_data = e.mem.u32(base + 0xd4);
    e.call(
        SET_DATA_BLOCK,
        &args![added, 1u32, block_data, vertices << 5, 0u32],
    );
    e.call(
        SET_DATA_STREAM,
        &args![added, 0u32, 1u32, 0u32, 4u32, vertices, 0x10u32, 0x20u32],
    );
    e.call(
        SET_DATA_STREAM,
        &args![added, 1u32, 1u32, 0x10u32, 4u32, vertices, 0x10u32, 0x20u32],
    );
    e.mem.set_u32(entry + 0x10, 0);
    e.mem.set_u32(entry + 0xc, 0);
    let name = e.global::<u32>(EXTRA_DATA_NAME_011F94BC);
    e.call(REMOVE_EXTRA_DATA, &args![geometry, name]);
    added
}

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_FLAG, VIEWER_STRING_FLOAT, VIEWER_STRING_INT,
    VIEWER_STRING_TEXT,
};

/// The vtable `00b669f0` stores first (`010ae0c8`).
const VTABLE_010AE0C8: u32 = 0x010a_e0c8;
/// The flag setter `00441130` (`thiscall`: bit index, byte value), the float setter
/// `00b66700` (`thiscall`, one `float`) and `_ftol2` (`00ec62c0`, the value in ST0).
const SET_FLAG_00441130: u32 = 0x0044_1130;
const SET_VALUE_00B66700: u32 = 0x00b6_6700;
const FTOL: u32 = 0x00ec_62c0;
/// The virtual slots (byte offsets) `SetRefractionRecurse` calls: `IsTriBasedGeom` (+0x1c)
/// and `IsNode` (+0xc).
const SLOT_IS_TRI_BASED_GEOM: u32 = 0x1c;
const SLOT_IS_NODE: u32 = 0x0c;
/// The shader globals `00b683e0` reads: a byte (`011f9427`), a word (`011f95e4`), the shader
/// flags (`011f91d8`), the shader level (`011f91c0`) and the two flag words it chooses from
/// (`010a91d8` when the four allow it, else `010a91d4`).
const BYTE_SWITCH_011F9427: u32 = 0x011f_9427;
const SWITCH_011F95E4: u32 = 0x011f_95e4;
const SHADER_FLAGS_011F91D8: u32 = 0x011f_91d8;
const PASS_FLAGS_010A91D8: u32 = 0x010a_91d8;
const PASS_FLAGS_010A91D4: u32 = 0x010a_91d4;
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, no words).
const GET_ACCUMULATOR_00B4F5C0: u32 = 0x00b4_f5c0;
/// `BSShaderProperty`'s water-fog array check (`00ba97b0`, `thiscall`, one word: the size)
/// and `RenderPassArray::Add` (`00ba9ee0`, `thiscall`, eight words).
const CHECK_CREATE_WATER_FOG_PASSES_00BA97B0: u32 = 0x00ba_97b0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The base `SaveBinary` (`00bb7160`, `thiscall`, one word: the stream).
const BASE_SAVE_BINARY_00BB7160: u32 = 0x00bb_7160;
/// `BSShaderLightingProperty::GetViewerStrings` (`00b71450`, `thiscall`, one word), the
/// class-name record (`011fa010`), `BSUtilities::GetTextureName` (`00c4bb40`, `cdecl`, one
/// word), the viewer-string makers of a name and an object (`00a7d5a0`) and of a name and a
/// 16-bit value (`00a7d490`), and the string array's `Add` (`0096a610`, `thiscall`, the
/// address of the element).
const LIGHTING_GET_VIEWER_STRINGS: u32 = 0x00b7_1450;
const CLASS_RTTI_011FA010: u32 = 0x011f_a010;
const GET_TEXTURE_NAME: u32 = 0x00c4_bb40;
const VIEWER_STRING_OBJECT: u32 = 0x00a7_d5a0;
const VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const STRINGS_ADD_0096A610: u32 = 0x0096_a610;
/// The labels `GetViewerStrings` uses (read in Ghidra).
const LABEL_BASE_DIFF: u32 = 0x010a_e428;
const LABEL_BASE_NORMAL: u32 = 0x010a_e41c;
const LABEL_MULTI_DIFF: u32 = 0x010a_e410;
const LABEL_MULTI_NORMAL: u32 = 0x010a_e400;
const LABEL_LANDSCAPE_TEXTURING: u32 = 0x010a_e3ec;
const LABEL_LANDSCAPE_TEXTURES: u32 = 0x010a_e3d8;
const LABEL_BASE_TEX: u32 = 0x010a_e3cc;
const LABEL_NORMAL_MAP: u32 = 0x010a_e3c0;
const LABEL_SPECULAR: u32 = 0x010a_e3b4;
const LABEL_GLOW_MAP: u32 = 0x010a_e3a8;
const LABEL_HEIGHTMAP: u32 = 0x010a_e394;
const LABEL_GLOW_SKIN_HAIR: u32 = 0x010a_e370;
const LABEL_ENVMAP: u32 = 0x010a_e360;
const LABEL_ENVMAP_MASK: u32 = 0x010a_e34c;
const LABEL_CLAMP: u32 = 0x010a_e344;
const LABEL_REFRACTION_POWER: u32 = 0x010a_e330;
const LABEL_REFRACTION_PERIOD: u32 = 0x010a_e31c;
const LABEL_SPECULAR_LOD: u32 = 0x010a_e30c;
const LABEL_ENVMAP_LOD: u32 = 0x010a_e300;
const LABEL_TEX_EFFECT_DATA: u32 = 0x010a_e2f0;
/// The texture effect data lines: the offset of each `float` and its label (`"Fill Color
/// R"`, `"Fill Color G"`, `"Fill Color R"` again, `"Fill Color A"`, `"Edge Color R"`,
/// `"Edge Color G"`, `"Edge Color R"` again, `"Edge Color A"`, `"Edge Falloff"`), as the
/// exe has them.
const TEX_EFFECT_LINES: [(u32, u32); 9] = [
    (0xc, 0x010a_e2e0),
    (0x10, 0x010a_e2d0),
    (0x14, 0x010a_e2e0),
    (0x18, 0x010a_e2c0),
    (0x1c, 0x010a_e2b0),
    (0x20, 0x010a_e2a0),
    (0x24, 0x010a_e2b0),
    (0x28, 0x010a_e290),
    (0x54, 0x010a_e280),
];

// Translated from 00b669f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b669f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` without words: the destructor of a 0x74-byte
/// reference-counted object (vtable `010ae0c8`; [`fn_00b675a0`] deletes it). Puts its vtable
/// back, releases and clears the `NiPointer`s at `+8` and `+0x6c`, then runs the members'
/// own destructors (each releases its pointer again when it is not null: both are null by
/// then), stores the base vtable `0101dce4` and lowers the object count at `011f4400`.
pub fn fn_00b669f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010AE0C8);
    ni_pointer_clear(e, base + 8);
    ni_pointer_clear(e, base + 0x6c);
    let member = e.mem.u32(base + 0x6c);
    if member != 0 {
        release_reference(e, member);
    }
    let member = e.mem.u32(base + 8);
    if member != 0 {
        release_reference(e, member);
    }
    e.mem.set_u32(base, VTABLE_0101DCE4);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00b671d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 57 of the vtable at `010ae0d0`), `thiscall`
/// with one word: stores it in the texture clamp mode `+0xd8` ([`fn_00b671c0`] answers it).
pub fn fn_00b671d0(e: &mut Engine, this: Ptr, mode: u32) {
    e.mem.set_u32(this.addr() + 0xd8, mode);
}

// Translated from 00b67280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 45 of the vtable at `010ae0d0`), `thiscall`
/// without words: answers the first texture of the first kind's array (`+0xac`).
pub fn fn_00b67280(e: &mut Engine, this: Ptr) -> u32 {
    let array = e.mem.u32(this.addr() + 0xac);
    e.mem.u32(array)
}

// Translated from 00b67330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 65 of the vtable at `010ae0d0`), `thiscall`
/// with two words (index, texture): assigns the texture at that index of the third kind's
/// array (`+0xb4`).
pub fn fn_00b67330(e: &mut Engine, this: Ptr, index: u32, texture: u32) {
    let array = e.mem.u32(this.addr() + 0xb4);
    ni_pointer_assign(e, array.wrapping_add(index.wrapping_mul(4)), texture);
}

// Translated from 00b675a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of [`fn_00b669f0`]'s object: that
/// destructor, then the sized delete (`00aa1460` with 0x74) when bit 0 of the flags is set.
/// Answers `this`.
pub fn fn_00b675a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b669f0(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x74u32]);
    }
    this
}

// Translated from 00b675c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::SetTextureEffectData` (engine map; the body matches):
/// assigns the object to the `NiPointer` at `+0xdc` and clears `iLastRenderPassState`.
pub fn bs_shader_pp_lighting_property_set_texture_effect_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
) {
    ni_pointer_assign(e, this.addr() + 0xdc, data);
    e.mem.set_u32(this.addr() + 0x38, 0);
}

// Translated from 00b683e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 41 of the vtable at `010ae0d0`), `thiscall`
/// with one word (a geometry): asks this property for its render passes (slot +0xa0 with the
/// geometry, a flag word, the address of a zeroed local word, 0, the accumulator
/// `00b4f5c0` answers, 0) and answers the low 16 bits of that local word. The flag word is
/// the one at `010a91d8` when the byte at `011f9427` is clear, the word at `011f95e4` set,
/// bit 5 of the shader flags (`011f91d8`) set and the shader level (`011f91c0`) 2 or more;
/// else the one at `010a91d4`.
pub fn fn_00b683e0(e: &mut Engine, this: Ptr, geometry: u32) -> u16 {
    let allowed = e.mem.u8(BYTE_SWITCH_011F9427) == 0
        && e.mem.u32(SWITCH_011F95E4) != 0
        && e.mem.u8(SHADER_FLAGS_011F91D8) & 0x20 != 0
        && e.mem.i32(SHADER_LEVEL_011F91C0) >= 2;
    let flags = if allowed {
        e.mem.u32(PASS_FLAGS_010A91D8)
    } else {
        e.mem.u32(PASS_FLAGS_010A91D4)
    };
    e.with_stack(4, |e, local| {
        let accumulator = e.call(GET_ACCUMULATOR_00B4F5C0, &args![]).u32();
        e.vcall(
            this.addr(),
            0xa0,
            &args![geometry, flags, local, 0u32, accumulator, 0u32],
        );
        e.mem.u16(local.addr())
    })
}

// Translated from 00b68770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::SetRefractionRecurse` (engine map; the body matches):
/// `cdecl`, six words: an object, a flag word, the refraction power (`float`), a byte, the
/// refraction period (`float`) and a byte. For a non-null object: a tri-based geometry
/// (slot +0x1c non-zero) whose shader property (`+0xa8`) has an `iShaderPropertyType` of 8
/// to 12 gets bit 15 set to the flag word (`00441130`), bit 16 to "flag and first byte",
/// bit 36 to "flag and second byte", the power through `00b66700` and, with the first byte
/// set, the period truncated to an integer (`_ftol2`) at `+0xe4`. Any other object that is
/// a node (slot +0xc) passes the six words on to each non-null child (the count at `+0xa6`
/// read once; the `float`s pushed through `FLD`/`FSTP`).
pub fn bs_shader_pp_lighting_property_set_refraction_recurse(
    e: &mut Engine,
    object: u32,
    flag: u32,
    power: u32,
    fire: u32,
    period: u32,
    second: u32,
) {
    if object == 0 {
        return;
    }
    if e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
        let property = e.mem.u32(object.wrapping_add(0xa8));
        if property == 0 {
            return;
        }
        let kind = e.mem.i32(property.wrapping_add(0x1c));
        if !(8..=0xc).contains(&kind) {
            return;
        }
        e.call(SET_FLAG_00441130, &args![property, 0xfu32, flag]);
        let both = u32::from(flag as u8 != 0 && fire as u8 != 0);
        e.call(SET_FLAG_00441130, &args![property, 0x10u32, both]);
        let both = u32::from(flag as u8 != 0 && second as u8 != 0);
        e.call(SET_FLAG_00441130, &args![property, 0x24u32, both]);
        e.call(SET_VALUE_00B66700, &args![property, x87_quiet_bits(power)]);
        if fire as u8 != 0 {
            let whole = e.call(FTOL, &args![x87_load32(period)]).u32();
            e.mem.set_u32(property.wrapping_add(0xe4), whole);
        }
        return;
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
    let mut index = 0u32;
    while index < count {
        let buffer = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_pp_lighting_property_set_refraction_recurse(
                e,
                child,
                flag,
                x87_quiet_bits(power),
                fire,
                x87_quiet_bits(period),
                second,
            );
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00b68c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 46 of the vtable at `010ae0d0`), `thiscall`
/// with one word (a geometry): the water-fog pass list. Null when the `float` at `+0xe0` is
/// above 0.0. Otherwise the list (`+0x50`) is made with one pass when missing (`00ba97b0`)
/// and, when it is empty, gets one pass for the geometry (`00ba9ee0`, first pass, no
/// lights): 0x2d8 plus 6 when bit 5 of the second flag word is set, plus 1 when bit 9 of the
/// word `+0x18` of the geometry's `+0x9c` object is set, and plus 0xc more of those with
/// bit 1 of the first flag word (0x2d8 to 0x2eb). Answers the list.
pub fn fn_00b68c10(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let base = this.addr();
    let refraction = f32::from_bits(e.mem.u32(base + 0xe0));
    if 0.0 < refraction {
        return 0;
    }
    e.call(CHECK_CREATE_WATER_FOG_PASSES_00BA97B0, &args![this, 1u32]);
    let list = e.mem.u32(base + 0x50);
    if e.mem.u32(list.wrapping_add(0x10)) != 0 {
        return e.mem.u32(base + 0x50);
    }
    let state = e.mem.u32(geometry.wrapping_add(0x9c));
    let skinned = state != 0 && (e.mem.u16(state.wrapping_add(0x18)) >> 9) & 1 != 0;
    let second = e.mem.u8(base + 0x24) & 0x20 != 0;
    let pass_enum: u32 = if e.mem.u8(base + 0x20) & 2 != 0 {
        match (second, skinned) {
            (true, true) => 0x2eb,
            (true, false) => 0x2e5,
            (false, true) => 0x2df,
            (false, false) => 0x2d9,
        }
    } else {
        match (second, skinned) {
            (true, true) => 0x2ea,
            (true, false) => 0x2e4,
            (false, true) => 0x2de,
            (false, false) => 0x2d8,
        }
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass_enum, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    e.mem.u32(base + 0x50)
}

/// One write through the stream's file (`[stream + 0x250]`): its write function at file
/// `+0xc` (`cdecl`: file, buffer, 4, the address of a word holding 4, 1); the answer is
/// added to the file's word at `+4`.
fn stream_write_word(e: &mut Engine, stream: u32, buffer: u32) {
    let file = e.mem.u32(stream.wrapping_add(0x250));
    let write = e.mem.u32(file.wrapping_add(0xc));
    let n = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write, &args![file, buffer, 4u32, size, 1u32]).u32()
    });
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(n));
}

// Translated from 00b68ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (a stream; the `SaveBinary` that pairs
/// with [`fn_00b68dc0`]): the base save (`00bb7160`), the clamp mode `+0xd8`, the texture set
/// `+0xa4` as a link (the stream's slot +0x30), then the four words `+0xe0` to `+0xec`, each
/// word written through the stream's file.
pub fn fn_00b68ed0(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(BASE_SAVE_BINARY_00BB7160, &args![this, stream]);
    stream_write_word(e, stream, base + 0xd8);
    let set = e.mem.u32(base + 0xa4);
    e.vcall(stream, 0x30, &args![set]);
    for offset in [0xe0u32, 0xe4, 0xe8, 0xec] {
        stream_write_word(e, stream, base + offset);
    }
}

/// The viewer string of the texture in slot `index` of the array at `offset`: through its
/// name (`00c4bb40`) when it has one (`00a7d5e0`), else through the object (`00a7d5a0`,
/// the slot read again).
fn texture_line(e: &mut Engine, this: u32, offset: u32, index: u32, label: u32) -> u32 {
    let slot = |e: &Engine| {
        e.mem
            .u32(this.wrapping_add(offset))
            .wrapping_add(index.wrapping_mul(4))
    };
    let texture = e.mem.u32(slot(e));
    let name = e.call(GET_TEXTURE_NAME, &args![texture]).u32();
    if name != 0 {
        e.call(VIEWER_STRING_TEXT, &args![label, name]).u32()
    } else {
        let texture = e.mem.u32(slot(e));
        e.call(VIEWER_STRING_OBJECT, &args![label, texture]).u32()
    }
}

/// The texture in slot `index` of the array at `offset`.
fn texture_at(e: &Engine, this: u32, offset: u32, index: u32) -> u32 {
    let array = e.mem.u32(this.wrapping_add(offset));
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

/// The string array's own `Add` (`0096a610`) with the string in a local word.
fn strings_append(e: &mut Engine, strings: u32, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(STRINGS_ADD_0096A610, &args![strings, local]);
    });
}

/// A `float` viewer line (`00a7d550`), the value through `FLD`/`FSTP`, added inline.
fn float_line(e: &mut Engine, strings: u32, label: u32, bits: u32) {
    let text = e
        .call(VIEWER_STRING_FLOAT, &args![label, x87_quiet_bits(bits)])
        .u32();
    viewer_strings_add(e, strings, text);
}

// Translated from 00b690d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::GetViewerStrings` (engine map; the body matches):
/// `thiscall`, one word (the strings). The `BSShaderLightingProperty` strings (`00b71450`),
/// the class name, then the textures: without the landscape flag (bit 14 of the first flag
/// word) the base diffuse and normal maps and, when present, the second diffuse and normal
/// maps; with it, the landscape flag line, the texture count (`+0xa8`) and for each of the
/// nine layers that has a base texture its base texture, normal map, specular flag (the
/// byte array `+0xcc`) and glow map. Then the heightmap (`+0xb8`), glow/skin (`+0xb4`),
/// environment map (`+0xbc`) and its mask (`+0xc0`) when present; "clamp" when slot +0xe0
/// answers 0; the refraction power (`+0xe0`) with bit 15 of the first flag word, or with bit
/// 16 the power and the period (`+0xe4`); the specular LOD (slot +0x10c) with bit 0; the
/// environment map LOD (slot +0x110) with bit 7; and with texture effect data (`+0xdc`) the
/// object and its nine `float`s. Lines are added inline or through `0096a610` as the exe
/// does for each.
pub fn bs_shader_pp_lighting_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let base = this.addr();
    e.call(LIGHTING_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI_011FA010);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
    if e.mem.u32(base + 0x20) & 0x4000 == 0 {
        let text = texture_line(e, base, 0xac, 0, LABEL_BASE_DIFF);
        viewer_strings_add(e, strings, text);
        let text = texture_line(e, base, 0xb0, 0, LABEL_BASE_NORMAL);
        viewer_strings_add(e, strings, text);
        if texture_at(e, base, 0xac, 1) != 0 {
            let text = texture_line(e, base, 0xac, 1, LABEL_MULTI_DIFF);
            strings_append(e, strings, text);
        }
        if texture_at(e, base, 0xb0, 1) != 0 {
            let text = texture_line(e, base, 0xb0, 1, LABEL_MULTI_NORMAL);
            strings_append(e, strings, text);
        }
    } else {
        let text = e
            .call(VIEWER_STRING_FLAG, &args![LABEL_LANDSCAPE_TEXTURING, 1u32])
            .u32();
        viewer_strings_add(e, strings, text);
        let count = u32::from(e.mem.u16(base + 0xa8));
        let text = e
            .call(VIEWER_STRING_SHORT, &args![LABEL_LANDSCAPE_TEXTURES, count])
            .u32();
        viewer_strings_add(e, strings, text);
        for layer in 0..9u32 {
            if texture_at(e, base, 0xac, layer) == 0 {
                continue;
            }
            let text = texture_line(e, base, 0xac, layer, LABEL_BASE_TEX);
            viewer_strings_add(e, strings, text);
            if texture_at(e, base, 0xb0, layer) != 0 {
                let text = texture_line(e, base, 0xb0, layer, LABEL_NORMAL_MAP);
                strings_append(e, strings, text);
            }
            let speculars = e.mem.u32(base + 0xcc);
            if e.mem.u8(speculars.wrapping_add(layer)) != 0 {
                let text = e
                    .call(VIEWER_STRING_FLAG, &args![LABEL_SPECULAR, 1u32])
                    .u32();
                viewer_strings_add(e, strings, text);
            }
            if texture_at(e, base, 0xb4, layer) != 0 {
                let text = texture_line(e, base, 0xb4, layer, LABEL_GLOW_MAP);
                strings_append(e, strings, text);
            }
        }
    }
    for (offset, label) in [
        (0xb8u32, LABEL_HEIGHTMAP),
        (0xb4, LABEL_GLOW_SKIN_HAIR),
        (0xbc, LABEL_ENVMAP),
        (0xc0, LABEL_ENVMAP_MASK),
    ] {
        if texture_at(e, base, offset, 0) != 0 {
            let text = texture_line(e, base, offset, 0, label);
            viewer_strings_add(e, strings, text);
        }
    }
    if e.vcall(base, 0xe0, &args![]).u32() == 0 {
        let text = e.call(VIEWER_STRING_FLAG, &args![LABEL_CLAMP, 1u32]).u32();
        viewer_strings_add(e, strings, text);
    }
    let flags = e.mem.u32(base + 0x20);
    if flags & 0x8000 != 0 {
        let power = e.mem.u32(base + 0xe0);
        float_line(e, strings, LABEL_REFRACTION_POWER, power);
    } else if flags & 0x10000 != 0 {
        let power = e.mem.u32(base + 0xe0);
        float_line(e, strings, LABEL_REFRACTION_POWER, power);
        let period = e.mem.u32(base + 0xe4);
        let text = e
            .call(VIEWER_STRING_INT, &args![LABEL_REFRACTION_PERIOD, period])
            .u32();
        viewer_strings_add(e, strings, text);
    }
    if e.mem.u8(base + 0x20) & 1 != 0 {
        let lod = e.vcall(base, 0x10c, &args![]).f64();
        float_line(e, strings, LABEL_SPECULAR_LOD, x87_store32(lod));
    }
    if e.mem.u32(base + 0x20) & 0x80 != 0 {
        let lod = e.vcall(base, 0x110, &args![]).f64();
        float_line(e, strings, LABEL_ENVMAP_LOD, x87_store32(lod));
    }
    let data = e.mem.u32(base + 0xdc);
    if data == 0 {
        return;
    }
    let text = e
        .call(VIEWER_STRING_OBJECT, &args![LABEL_TEX_EFFECT_DATA, data])
        .u32();
    viewer_strings_add(e, strings, text);
    for (offset, label) in TEX_EFFECT_LINES {
        let data = e.mem.u32(base + 0xdc);
        let value = e.mem.u32(data.wrapping_add(offset));
        float_line(e, strings, label, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_6760, fn_00b66760(u32) -> u32),
        entry!(0x00b6_6850, fn_00b66850(Ptr)),
        entry!(
            0x00b6_6ad0,
            bs_shader_pp_lighting_property_load_texture_set(Ptr, u32, u32)
        ),
        entry!(0x00b6_6b80, fn_00b66b80(Ptr) -> f32),
        entry!(0x00b6_6c40, fn_00b66c40(Ptr) -> f32),
        entry!(0x00b6_6db0, fn_00b66db0(Ptr)),
        entry!(0x00b6_6e20, fn_00b66e20(Ptr, u32) -> u32),
        entry!(0x00b6_6f20, fn_00b66f20(Ptr)),
        entry!(0x00b6_6f50, fn_00b66f50(Ptr) -> Ptr),
        entry!(0x00b6_71b0, fn_00b671b0() -> u32),
        entry!(0x00b6_71c0, fn_00b671c0(Ptr) -> u32),
        entry!(0x00b6_71e0, fn_00b671e0(Ptr) -> u16),
        entry!(0x00b6_71f0, fn_00b671f0(Ptr) -> u16),
        entry!(0x00b6_7200, fn_00b67200(Ptr, u32, u32, u32)),
        entry!(0x00b6_7260, fn_00b67260(Ptr, u32, u32) -> u32),
        entry!(0x00b6_7290, fn_00b67290(Ptr, u32, u32)),
        entry!(0x00b6_72e0, fn_00b672e0(Ptr, u32, u32)),
        entry!(0x00b6_7380, fn_00b67380(Ptr)),
        entry!(0x00b6_7580, fn_00b67580(Ptr, u32) -> Ptr),
        entry!(0x00b6_7610, fn_00b67610(Ptr, u32) -> u32),
        entry!(0x00b6_76a0, fn_00b676a0(Ptr, Ptr)),
        entry!(
            0x00b6_7a40,
            bs_shader_pp_lighting_property_setup_geometry(Ptr, u32) -> bool
        ),
        entry!(
            0x00b6_7be0,
            bs_shader_pp_lighting_property_get_render_passes(Ptr, u32, u32, u32, u32, u32, u32)
                -> u32
        ),
        entry!(0x00b6_8450, fn_00b68450(Ptr)),
        entry!(0x00b6_8660, fn_00b68660(Ptr, u32, u32)),
        entry!(0x00b6_8880, fn_00b68880(Ptr, u32, u32, u32) -> u32),
        entry!(0x00b6_8b10, fn_00b68b10(Ptr)),
        entry!(0x00b6_8d30, fn_00b68d30(Ptr)),
        entry!(0x00b6_8d50, fn_00b68d50() -> u32),
        entry!(0x00b6_8dc0, fn_00b68dc0(Ptr, u32)),
        entry!(0x00b6_8fb0, fn_00b68fb0(Ptr, u32) -> Ptr),
        entry!(
            0x00b6_8fe0,
            bs_shader_pp_lighting_property_post_link_object(Ptr, u32)
        ),
        entry!(0x00b6_9be0, fn_00b69be0(Ptr, u32) -> u32),
        entry!(0x00b6_69f0, fn_00b669f0(Ptr)),
        entry!(0x00b6_71d0, fn_00b671d0(Ptr, u32)),
        entry!(0x00b6_7280, fn_00b67280(Ptr) -> u32),
        entry!(0x00b6_7330, fn_00b67330(Ptr, u32, u32)),
        entry!(0x00b6_75a0, fn_00b675a0(Ptr, u32) -> Ptr),
        entry!(
            0x00b6_75c0,
            bs_shader_pp_lighting_property_set_texture_effect_data(Ptr, u32)
        ),
        entry!(0x00b6_83e0, fn_00b683e0(Ptr, u32) -> u16),
        entry!(
            0x00b6_8770,
            bs_shader_pp_lighting_property_set_refraction_recurse(u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00b6_8c10, fn_00b68c10(Ptr, u32) -> u32),
        entry!(0x00b6_8ed0, fn_00b68ed0(Ptr, u32)),
        entry!(
            0x00b6_90d0,
            bs_shader_pp_lighting_property_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const STUB: AbiFn = |_, _| Ret::default();
    /// The property test vtable: slot `+n` is the address `SLOTS + n`.
    const VTABLE: u32 = 0x0e80_0000;
    const SLOTS: u32 = 0x0e81_0000;

    fn engine() -> Engine {
        let mut e = rig();
        e.map(0x011f_4000, 0x7000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_9000, 0x1000);
        e.register(MEMORY_FREE, STUB);
        e.register(VECTOR_DESTRUCT, STUB);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SIZED_DELETE, STUB);
        let slots: Vec<u32> = (0..0x48).map(|slot| SLOTS + slot * 4).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, STUB);
        }
        e
    }

    /// A property with the test vtable, six texture arrays of `count` and both byte arrays.
    fn property(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this, VTABLE);
        e.mem.set_u16(this + 0xa8, count as u16);
        for kind in 0..6 {
            let block = e.mem.alloc(4 + count * 4);
            e.mem.set_u32(block, count);
            e.mem.set_u32(this + 0xac + kind * 4, block + 4);
        }
        let bytes = e.mem.alloc(16);
        e.mem.set_u32(this + 0xcc, bytes);
        let bytes = e.mem.alloc(16);
        e.mem.set_u32(this + 0xc4, bytes);
        this
    }

    fn texture(e: &mut Engine, this: u32, kind: u32, index: u32, count: u32) -> u32 {
        let texture = object(e, count);
        let array = e.mem.u32(this + 0xac + kind * 4);
        e.mem.set_u32(array + index * 4, texture);
        texture
    }

    #[test]
    fn fn_00b66760_matches_two_pass_numbers() {
        let mut e = Engine::new();
        for (value, want) in [(0xfcu32, 1u32), (0x102, 1), (0xfd, 0), (0, 0)] {
            assert_eq!(e.call(0x00b6_6760, &args![value]).u32(), want);
        }
    }

    #[test]
    fn fn_00b66850_frees_owned_buffers() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 8, 1);
        e.mem.set_u32(this + 0xc, 0x100);
        e.mem.set_u32(this + 0x10, 0x200);
        e.mem.set_u32(OBJECT_COUNT_011F4400, 3);
        e.call(0x00b6_6850, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, 0x100], vec![MEMORY_MANAGER, 0x200]]
        );
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        assert_eq!(e.mem.u32(this), VTABLE_0101DCE4);
        assert_eq!(e.mem.u32(OBJECT_COUNT_011F4400), 2);
        // Buffers not owned: only cleared.
        e.mem.set_u8(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00b6_6850, &args![this]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn load_texture_set_releases_then_loads_every_kind() {
        let mut e = engine();
        let this = property(&mut e, 2);
        let old = texture(&mut e, this, 3, 0, 1);
        let set = e.mem.alloc(8);
        e.mem.set_u32(set, VTABLE);
        e.mem.set_u32(this + 0xa4, set);
        e.register(SLOTS + 0x90, |e, a| {
            if a[1] == 0 {
                e.mem.set_u32(a[2], 0x5555);
            }
            Ret::default()
        });
        e.call(0x00b6_6ad0, &args![this, 0x77u32, 0u32]);
        let array = e.mem.u32(this + 0xac + 12);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        let loads = calls(&e, SLOTS + 0x90);
        assert_eq!(loads.len(), 6);
        for (kind, words) in loads.iter().enumerate() {
            let array = e.mem.u32(this + 0xac + kind as u32 * 4);
            assert_eq!(words, &vec![set, kind as u32, array]);
        }
        assert_eq!(calls(&e, SLOTS + 0x8c), vec![vec![set, 0]]);
    }

    #[test]
    fn fn_00b66b80_fades_with_the_distance() {
        let mut e = engine();
        let this = property(&mut e, 2);
        e.mem.set_f32(FADE_NEAR_011F9454, 100.0);
        e.mem.set_f32(FADE_FAR_011F9458, 300.0);
        e.mem.set_f32(this + 0x34, 150.0);
        // Flag 1 clear: 1.0.
        assert_eq!(e.call(0x00b6_6b80, &args![this]).st0, 1.0);
        e.mem.set_u32(this + 0x20, 1);
        assert_eq!(e.call(0x00b6_6b80, &args![this]).st0, 0.75);
        e.mem.set_f32(this + 0x34, 50.0);
        assert_eq!(e.call(0x00b6_6b80, &args![this]).st0, 1.0);
        e.mem.set_f32(this + 0x34, 300.0);
        assert_eq!(e.call(0x00b6_6b80, &args![this]).st0, 0.0);
        e.mem.set_f32(this + 0x34, f32::NAN);
        assert!(e.call(0x00b6_6b80, &args![this]).st0.is_nan());
        e.mem.set_u32(this + 0x24, 0x0080_0000);
        assert_eq!(e.call(0x00b6_6b80, &args![this]).st0, 1.0);
    }

    #[test]
    fn fn_00b66c40_picks_the_first_fade_on() {
        let mut e = engine();
        e.register(FLOAT_HELPER_0040EBD0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        let this = property(&mut e, 2);
        e.mem.set_f32(FADE_NEAR_011F945C, 0.0);
        e.mem.set_f32(FADE_FAR_011F9460, 100.0);
        e.mem.set_f32(FADE_NEAR_011F9464, 10.0);
        e.mem.set_f32(FADE_FAR_011F9468, 20.0);
        e.mem.set_f32(this + 0x34, 15.0);
        assert_eq!(e.call(0x00b6_6c40, &args![this]).st0, 1.0);
        e.mem.set_u32(this + 0x20, 0x80);
        assert_eq!(e.call(0x00b6_6c40, &args![this]).st0 as f32, 0.85);
        assert_eq!(
            calls(&e, FLOAT_HELPER_0040EBD0),
            vec![vec![1.0f32.to_bits(), 0.15f32.to_bits()]]
        );
        e.mem.set_u32(this + 0x20, 0x0002_0080);
        assert_eq!(e.call(0x00b6_6c40, &args![this]).st0, 0.5);
        e.mem.set_f32(this + 0x34, 25.0);
        assert_eq!(e.call(0x00b6_6c40, &args![this]).st0, 0.0);
        e.mem.set_f32(this + 0x34, 5.0);
        assert_eq!(e.call(0x00b6_6c40, &args![this]).st0, 1.0);
    }

    #[test]
    fn fn_00b66db0_hands_every_texture_to_the_renderer() {
        let mut e = engine();
        let this = property(&mut e, 2);
        e.register(SLOTS + 0xe8, |_, _| 2u32.into_ret());
        e.register(SLOTS + 0xf4, |_, a| {
            if a[1] == 1 && a[2] == 1 {
                0u32.into_ret()
            } else {
                (0x100 + a[1] * 0x10 + a[2]).into_ret()
            }
        });
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, VTABLE);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        e.call(0x00b6_6db0, &args![this]);
        let handed = calls(&e, SLOTS + 0xe0);
        assert_eq!(handed.len(), 11);
        assert_eq!(handed[0], vec![renderer, 0x100, 0, 0]);
        assert_eq!(handed[2], vec![renderer, 0x110, 0, 0]);
        assert_eq!(handed[10], vec![renderer, 0x151, 0, 0]);
    }

    #[test]
    fn fn_00b66e20_makes_the_depth_pass_once() {
        let mut e = engine();
        e.register(RENDER_PASS_CONSTRUCT, |_, a| a[0].into_ret());
        let this = property(&mut e, 2);
        for (flags, kind) in [(2u32, 0x1e8u32), (0x1000, 0x1e9), (0, 0x1e7)] {
            e.mem.set_u32(this + 0x100, 0);
            e.mem.set_u32(this + 0x20, flags);
            e.call_log = Some(vec![]);
            let pass = e.call(0x00b6_6e20, &args![this, 0x44u32]).u32();
            assert_ne!(pass, 0);
            assert_eq!(e.mem.u32(this + 0x100), pass);
            assert_eq!(
                calls(&e, RENDER_PASS_CONSTRUCT),
                vec![vec![pass, 0x44, kind, 1, 0, 0]]
            );
        }
        e.call_log = Some(vec![]);
        let pass = e.mem.u32(this + 0x100);
        assert_eq!(e.call(0x00b6_6e20, &args![this, 0x44u32]).u32(), pass);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn fn_00b66f20_releases_the_held_object() {
        let mut e = engine();
        let held = object(&mut e, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, held);
        e.call(0x00b6_6f20, &args![slot]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        e.mem.set_u32(slot, 0);
        e.call(0x00b6_6f20, &args![slot]);
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 1);
    }

    #[test]
    fn fn_00b66f50_constructs_the_property() {
        let mut e = engine();
        e.register(LIGHTING_PROPERTY_CONSTRUCT_00B71720, STUB);
        e.mem.set_u8(DEFAULT_FLAG_011FA00C, 1);
        e.mem.set_u32(INITIAL_E8_0102918C, 0.5f32.to_bits());
        let this = e.mem.alloc(0x110);
        e.mem.write(this, &[0xcc; 0x110]);
        assert_eq!(e.call(0x00b6_6f50, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010AE0D0);
        assert_eq!(e.mem.u16(this + 0xa8), 2);
        for kind in 0..6 {
            let array = e.mem.u32(this + 0xac + kind * 4);
            assert_eq!(e.mem.u32(array - 4), 2);
            assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0));
        }
        let constructs = calls(&e, VECTOR_CONSTRUCT);
        assert_eq!(constructs.len(), 6);
        assert_eq!(
            constructs[0][1..],
            [
                4,
                2,
                TEXTURE_POINTER_CONSTRUCT_006694E0,
                TEXTURE_POINTER_DESTRUCT_0045CEC0
            ]
        );
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|w| w[1]).collect();
        assert_eq!(sizes, vec![0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 2, 2]);
        let bytes = e.mem.u32(this + 0xcc);
        assert_eq!((e.mem.u8(bytes), e.mem.u8(bytes + 1)), (0, 0));
        let bytes = e.mem.u32(this + 0xc4);
        assert_eq!((e.mem.u8(bytes), e.mem.u8(bytes + 1)), (0x1e, 0x1e));
        for offset in [0xa4u32, 0xd0, 0xdc, 0xe4, 0xd4, 0x100, 0xe0, 0x84, 0xa0] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xd8), 3);
        assert_eq!(e.mem.u32(this + 0x20) & 0x2000, 0x2000);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.f32(this + 0xe8), 0.5);
        assert_eq!(e.mem.u32(this + 0x1c), 8);
        assert_eq!(e.mem.f32(this + 0xec), 1.0);
    }

    #[test]
    fn getters_and_setters_reach_their_fields() {
        let mut e = engine();
        let this = property(&mut e, 2);
        assert_eq!(e.call(0x00b6_71b0, &args![]).u32(), 0x011f_a010);
        e.mem.set_u32(this + 0xd8, 7);
        assert_eq!(e.call(0x00b6_71c0, &args![this]).u32(), 7);
        assert_eq!(e.call(0x00b6_71e0, &args![this]).u32(), 2);
        e.mem.set_u16(this + 0xc8, 5);
        assert_eq!(e.call(0x00b6_71f0, &args![this]).u32(), 5);
        let held = object(&mut e, 0);
        e.call(0x00b6_7200, &args![this, 2u32, 1u32, held]);
        let array = e.mem.u32(this + 0xac + 8);
        assert_eq!(e.mem.u32(array + 4), held);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(e.call(0x00b6_7260, &args![this, 2u32, 1u32]).u32(), held);
        e.call(0x00b6_7290, &args![this, 1u32, held]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0xac) + 4), held);
        e.call(0x00b6_72e0, &args![this, 0u32, held]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0xb0)), held);
        assert_eq!(e.mem.u32(held + 4), 3);
    }

    #[test]
    fn fn_00b67380_destroys_the_property() {
        let mut e = engine();
        e.register(RENDER_PASS_DESTRUCT, STUB);
        e.register(LIGHTING_PROPERTY_DESTRUCT_00B71410, STUB);
        let this = property(&mut e, 2);
        let held = texture(&mut e, this, 4, 1, 1);
        let tangents = object(&mut e, 1);
        e.mem.set_u32(this + 0xd0, tangents);
        let set = object(&mut e, 2);
        e.mem.set_u32(this + 0xa4, set);
        e.mem.set_u32(this + 0x100, 0x7000);
        e.call(0x00b6_7380, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010AE0D0);
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            vec![vec![held], vec![tangents]]
        );
        assert_eq!(calls(&e, VECTOR_DESTRUCT).len(), 6);
        assert_eq!(calls(&e, MEMORY_FREE).len(), 6 + 2 + 1);
        assert_eq!(calls(&e, RENDER_PASS_DESTRUCT), vec![vec![0x7000]]);
        assert_eq!(e.mem.u32(set + 4), 1);
        assert_eq!(
            calls(&e, LIGHTING_PROPERTY_DESTRUCT_00B71410),
            vec![vec![this]]
        );
    }

    #[test]
    fn deleting_destructors_free_on_bit_0() {
        let mut e = engine();
        e.register(LIGHTING_PROPERTY_DESTRUCT_00B71410, STUB);
        let small = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00b6_7580, &args![small, 1u32]).u32(), small);
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![small, 0x14]]);
        let this = property(&mut e, 2);
        e.call(0x00b6_8fb0, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_SIZED_DELETE).len(), 1);
        e.call(0x00b6_8fb0, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_SIZED_DELETE)[1], vec![this, 0x104]);
    }

    #[test]
    fn fn_00b67610_clones_into_a_new_property() {
        let mut e = engine();
        e.register(LIGHTING_PROPERTY_CONSTRUCT_00B71720, STUB);
        e.register(COPY_TO_00B702B0, STUB);
        let this = property(&mut e, 2);
        let clone = e.call(0x00b6_7610, &args![this, 0x99u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_010AE0D0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x104]]);
        assert_eq!(calls(&e, COPY_TO_00B702B0), vec![vec![this, clone, 0x99]]);
        assert_eq!(calls(&e, SLOTS + 0xd0), vec![vec![this, clone]]);
    }

    #[test]
    fn fn_00b676a0_copies_textures_and_fields() {
        let mut e = engine();
        let source = property(&mut e, 2);
        let dest = property(&mut e, 1);
        let held = texture(&mut e, source, 5, 1, 1);
        let bytes = e.mem.u32(source + 0xcc);
        e.mem.set_u8(bytes + 1, 9);
        let tangents = object(&mut e, 1);
        e.mem.set_u32(source + 0xd0, tangents);
        e.mem.set_u32(source + 0xd8, 4);
        e.mem.set_f32(source + 0xec, 2.5);
        e.mem.set_u32(source + 0x8c, 0x1234);
        e.call(0x00b6_76a0, &args![source, dest]);
        assert_eq!(e.mem.u16(dest + 0xa8), 2);
        assert_eq!(calls(&e, VECTOR_DESTRUCT).len(), 6);
        let array = e.mem.u32(dest + 0xac + 20);
        assert_eq!(e.mem.u32(array + 4), held);
        assert_eq!(e.mem.u32(held + 4), 2);
        assert_eq!(e.mem.u8(e.mem.u32(dest + 0xcc) + 1), 9);
        assert_eq!(e.mem.u32(dest + 0xd0), tangents);
        assert_eq!(e.mem.u32(dest + 0xd8), 4);
        assert_eq!(e.mem.f32(dest + 0xec), 2.5);
        assert_eq!(e.mem.u32(dest + 0x8c), 0x1234);
        // Same count: the arrays are kept.
        e.call_log = Some(vec![]);
        e.call(0x00b6_76a0, &args![source, dest]);
        assert!(calls(&e, VECTOR_DESTRUCT).is_empty());
        assert_eq!(e.mem.u32(held + 4), 2);
    }

    fn geometry(e: &mut Engine) -> (u32, u32) {
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry, VTABLE);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        let properties = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, properties);
        e.mem.set_u32(geometry + 8, 0x5150);
        (geometry, data)
    }

    #[test]
    fn setup_geometry_sets_flags_and_tangents() {
        let mut e = engine();
        e.register(CREATE_TANGENT_SPACE_SIMPLE, |_, _| 0x7777u32.into_ret());
        e.register(POINTER_ASSIGN_0066B0D0, STUB);
        e.register(WARN_005B5E40, STUB);
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        let this = property(&mut e, 2);
        let (geometry, data) = geometry(&mut e);
        e.mem.set_u32(data + 0x28, 1);
        e.mem.set_u32(this + 0x38, 9);
        assert!(e.call(0x00b6_7a40, &args![this, geometry]).bool());
        assert_eq!(e.mem.u32(this + 0x24), 0x20);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(
            calls(&e, POINTER_ASSIGN_0066B0D0),
            vec![vec![this + 0xd0, 0x7777]]
        );
        assert_eq!(calls(&e, SLOTS + 0x108), vec![vec![this]]);
        // No texture in the second array, flags clear: no warning.
        assert!(calls(&e, WARN_005B5E40).is_empty());
        // Flag 0x800 without the texture: the second warning.
        e.mem.set_u32(data + 0x28, 0);
        e.mem.set_u32(this + 0x20, 0x0010_0800);
        e.mem.set_u32(EMPTY_TEXTURE_011F9528, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_7a40, &args![this, geometry]);
        assert_eq!(e.mem.u32(this + 0x24), 0x20);
        assert_eq!(calls(&e, SLOTS + 0xe4), vec![vec![this, 3]]);
        assert_eq!(
            calls(&e, WARN_005B5E40),
            vec![vec![FORMAT_010AE1F0, 0x5150]]
        );
        // The flag clears when neither holds; a second texture gives the first warning.
        e.mem.set_u32(this + 0x24, 0);
        e.mem.set_u32(this + 0x20, 0);
        texture(&mut e, this, 3, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_7a40, &args![this, geometry]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(
            calls(&e, WARN_005B5E40),
            vec![vec![FORMAT_010AE238, 0x5150]]
        );
    }

    fn passes_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        for callee in [
            CHECK_CREATE_DEPTH_MAP_PASSES,
            ADD_DEPTH_MAP_PASS_2X,
            CHECK_CREATE_CONST_ALPHA_PASSES,
            ADD_CONST_ALPHA_PASS_00BDA060,
            CHECK_CREATE_LOCAL_MAP_PASSES_00BA96D0,
            ADD_LOCAL_MAP_PASS_2X,
            CHECK_CREATE_SILHOUETTE_PASSES_00BA9820,
            ADD_SILHOUETTE_PASS_00BDF6C0,
            CHECK_CREATE_RENDER_PASSES,
            CAMERA_POSITION_005BB9B0,
            SET_ALPHA,
            RESORT_LIGHTS,
        ] {
            e.register(callee, STUB);
        }
        e.register(HAS_FLAG_004A2020, |_, _| 0u32.into_ret());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(DEPTH_PASSES_WANTED_005BF7B0, |_, _| 0u32.into_ret());
        let this = property(&mut e, 2);
        for offset in [0x3cu32, 0x40, 0x44, 0x48, 0x54] {
            let array = e.mem.alloc(0x14);
            let list = e.mem.alloc(0x10);
            e.mem.set_u32(array + 4, list);
            e.mem.set_u32(this + offset, array);
        }
        let (geometry, _) = geometry(&mut e);
        let accumulator = e.mem.alloc(0x200);
        (e, this, geometry, accumulator)
    }

    fn get_passes(
        e: &mut Engine,
        this: u32,
        geometry: u32,
        kind: u32,
        accumulator: u32,
        flag: u32,
    ) -> u32 {
        let count = e.mem.alloc(4);
        e.call(
            0x00b6_7be0,
            &args![this, geometry, 3u32, count, kind, accumulator, flag],
        )
        .u32()
    }

    #[test]
    fn get_render_passes_for_the_special_types() {
        let (mut e, this, geometry, accumulator) = passes_engine();
        let properties = e.mem.u32(geometry + 0x9c);
        e.mem.set_u16(properties + 0x18, 0x200);
        let depth = e.mem.u32(this + 0x40);
        assert_eq!(
            get_passes(&mut e, this, geometry, 7, accumulator, 0xab01),
            depth
        );
        assert_eq!(
            calls(&e, ADD_DEPTH_MAP_PASS_2X),
            vec![vec![this, geometry, 0, 0xab01, accumulator]]
        );
        // Without the property bit the flag's low byte goes to 0.
        e.mem.set_u16(properties + 0x18, 0);
        e.call_log = Some(vec![]);
        get_passes(&mut e, this, geometry, 7, accumulator, 0xab01);
        assert_eq!(calls(&e, ADD_DEPTH_MAP_PASS_2X)[0][3], 0xab00);
        e.mem.set_u32(this + 0x20, 0x8000);
        assert_eq!(get_passes(&mut e, this, geometry, 7, accumulator, 1), 0);
        e.mem.set_u32(this + 0x20, 2);
        e.call_log = Some(vec![]);
        assert_eq!(
            get_passes(&mut e, this, geometry, 9, accumulator, 1),
            e.mem.u32(this + 0x48)
        );
        assert_eq!(
            calls(&e, ADD_LOCAL_MAP_PASS_2X),
            vec![vec![this, geometry, 1, 0]]
        );
        e.mem.set_u32(geometry + 0xbc, 5);
        assert_eq!(
            get_passes(&mut e, this, geometry, 0xf, accumulator, 1),
            e.mem.u32(this + 0x54)
        );
        assert_eq!(
            calls(&e, ADD_SILHOUETTE_PASS_00BDF6C0),
            vec![vec![this, geometry, 1]]
        );
        e.mem.set_u32(this + 0x20, 0);
        assert_eq!(
            get_passes(&mut e, this, geometry, 8, accumulator, 1),
            e.mem.u32(this + 0x44)
        );
        assert_eq!(
            calls(&e, ADD_CONST_ALPHA_PASS_00BDA060),
            vec![vec![this, geometry, 1, 0]]
        );
    }

    #[test]
    fn get_render_passes_for_the_depth_array_refreshes_its_first_pass() {
        let (mut e, this, geometry, accumulator) = passes_engine();
        let depth = e.mem.u32(this + 0x40);
        e.mem.set_u32(depth + 0x10, 1);
        let pass = e.mem.alloc(0x10);
        let target = e.mem.alloc(4);
        e.mem.set_u8(pass + 9, 1);
        e.mem.set_u32(pass + 0xc, target);
        e.mem.set_u32(e.mem.u32(depth + 4), pass);
        let renderer = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x1a4);
        e.mem.set_u32(renderer + 8, inner);
        e.mem.set_u32(inner + 0x1a0, 0x4321);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        assert_eq!(get_passes(&mut e, this, geometry, 7, accumulator, 1), depth);
        assert_eq!(e.mem.u32(target), 0x4321);
        assert!(calls(&e, ADD_DEPTH_MAP_PASS_2X).is_empty());
    }

    #[test]
    fn get_render_passes_builds_the_main_array() {
        let (mut e, this, geometry, accumulator) = passes_engine();
        e.mem.set_f32(this + 0x28, 1.0);
        e.mem.set_f32(this + 0x2c, 0.5);
        e.mem.set_u32(SHADER_LEVEL_011F91C0, 2);
        e.register(SLOTS + 0x118, |e, a| {
            // Two passes: the first numbered 0xfc.
            let this = a[0];
            let passes = e.mem.u32(this + 0x3c);
            let list = e.mem.u32(passes + 4);
            for index in 0..2 {
                let pass = e.mem.alloc(0x10);
                e.mem.set_u16(pass + 4, 0xfc + index as u16);
                e.mem.set_u32(list + index * 4, pass);
            }
            e.mem.set_u32(passes + 0x10, 2);
            Ret::default()
        });
        let passes = e.mem.u32(this + 0x3c);
        assert_eq!(
            get_passes(&mut e, this, geometry, 2, accumulator, 1),
            passes
        );
        assert_eq!(calls(&e, SET_ALPHA), vec![vec![this, 0.5f32.to_bits()]]);
        let added = calls(&e, SLOTS + 0x118);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], geometry);
        assert_eq!(added[0][4], 0);
        assert_eq!(added[0][6], 1);
        assert_eq!(e.mem.u32(this + 0x38), 0x203);
        let list = e.mem.u32(passes + 4);
        let first = e.mem.u32(list);
        // The first pass (0xfc) lowered by one; the last one marked.
        assert_eq!(e.mem.u16(first + 4), 0xfb);
        assert_eq!(e.mem.u8(e.mem.u32(list + 4) + 8), 1);
        // The state matches the key now: nothing is rebuilt.
        e.call_log = Some(vec![]);
        get_passes(&mut e, this, geometry, 2, accumulator, 1);
        assert!(calls(&e, SLOTS + 0x118).is_empty());
        // A zero alpha stops before the passes; a flag 0 clears the count word.
        e.mem.set_u32(this + 0x38, 0);
        e.mem.set_f32(this + 0x28, 0.0);
        get_passes(&mut e, this, geometry, 2, accumulator, 1);
        assert!(calls(&e, SLOTS + 0x118).is_empty());
        let count = e.mem.alloc(4);
        e.mem.set_u16(count, 7);
        e.call(
            0x00b6_7be0,
            &args![this, geometry, 3u32, count, 2u32, accumulator, 0u32],
        );
        assert_eq!(e.mem.u16(count), 0);
        assert_eq!(calls(&e, SLOTS + 0x118).len(), 1);
    }

    #[test]
    fn get_render_passes_measures_the_distance_and_clears_on_crossing() {
        let (mut e, this, geometry, accumulator) = passes_engine();
        e.register(CAMERA_POSITION_005BB9B0, |e, a| {
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            Ret::default()
        });
        e.register(VECTOR_LENGTH_00457990, |e, a| {
            f64::from(e.mem.f32(a[0])).into_ret()
        });
        e.register(GET_ACCUMULATOR, |e, _| {
            let block = e.mem.alloc(0x10);
            block.into_ret()
        });
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_f32(FADE_FAR_011F9458, 100.0);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound, 150.0);
        e.mem.set_f32(bound + 0xc, 10.0);
        e.mem.set_u32(geometry + 0x20, bound);
        e.mem.set_f32(this + 0x34, 90.0);
        e.mem.set_u32(this + 0x38, 0x55);
        e.mem.set_f32(this + 0x28, 0.0);
        get_passes(&mut e, this, geometry, 2, accumulator, 1);
        assert_eq!(e.mem.f32(this + 0x34), 140.0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        // Measured again on the same side: kept.
        e.mem.set_u32(this + 0x38, 0x55);
        get_passes(&mut e, this, geometry, 2, accumulator, 1);
        assert_eq!(e.mem.u32(this + 0x38), 0x55);
    }

    #[test]
    fn fn_00b68450_grows_the_arrays_to_ten() {
        let mut e = engine();
        let this = property(&mut e, 2);
        let held = texture(&mut e, this, 0, 1, 1);
        e.call(0x00b6_8450, &args![this]);
        assert_eq!(e.mem.u16(this + 0xa8), 10);
        let array = e.mem.u32(this + 0xac);
        assert_eq!(e.mem.u32(array - 4), 10);
        assert_eq!(e.mem.u32(array + 4), held);
        assert_eq!(e.mem.u32(held + 4), 2);
        assert_eq!(calls(&e, VECTOR_DESTRUCT).len(), 6);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|w| w[1]).collect();
        assert_eq!(sizes, vec![0x2c, 0x2c, 0x2c, 0x2c, 0x2c, 0x2c, 0xa, 0xa]);
        e.call_log = Some(vec![]);
        e.call(0x00b6_8450, &args![this]);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn fn_00b68660_loads_one_index() {
        let mut e = engine();
        let this = property(&mut e, 10);
        for index in 0..3 {
            texture(&mut e, this, 0, index, 1);
        }
        let normal = texture(&mut e, this, 1, 4, 1);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 0x18, 6);
        e.mem.set_u32(normal + 0x24, data);
        e.call(0x00b6_8660, &args![this, 4u32, 0u32]);
        let array = e.mem.u32(this + 0xb0);
        assert_eq!(e.mem.u32(array + 16), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![normal]]);
        assert_eq!(e.mem.u16(this + 0xc8), 2);
        // With a set the textures are loaded through it.
        let set = e.mem.alloc(4);
        e.mem.set_u32(set, VTABLE);
        e.register(SLOTS + 0x90, |e, a| {
            if a[1] == 1 {
                let texture = e.mem.alloc(0x28);
                let data = e.mem.alloc(0x20);
                e.mem.set_u32(data + 0x18, 5);
                e.mem.set_u32(texture + 0x24, data);
                e.mem.set_u32(a[2], texture);
            }
            Ret::default()
        });
        e.call(0x00b6_8660, &args![this, 4u32, set]);
        assert_eq!(calls(&e, SLOTS + 0x90).len(), 6);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0xcc) + 4), 1);
    }

    #[test]
    fn fn_00b68880_picks_the_shader() {
        let mut e = engine();
        e.register(SKIN_PROPERTY_CONSTRUCT_00BB4540, |_, a| a[0].into_ret());
        e.register(LIST_HAS_0045BAD0, |_, _| 0u32.into_ret());
        e.register(HAS_FLAG_004A2020, |_, a| u32::from(a[1] == 0x1a).into_ret());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        let this = property(&mut e, 2);
        let (geometry, _) = geometry(&mut e);
        assert_eq!(e.call(0x00b6_8880, &args![this, 0u32, 0u32, 0u32]).u32(), 0);
        e.mem.set_u32(this + 0x24, 2);
        assert_eq!(
            e.call(0x00b6_8880, &args![this, geometry, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0x58), 1);
        e.mem.set_u32(this + 0x24, 0);
        e.mem.set_u32(this + 0x20, 0x800);
        assert_eq!(
            e.call(0x00b6_8880, &args![this, geometry, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0x58), 0xf);
        assert_eq!(e.mem.u32(this + 0x20), 0x820);
        // Without flag 0x800 and with the switch, flag 0x1a makes the new property.
        e.mem.set_u32(this + 0x20, 0);
        e.mem.set_u8(SWITCH_011F9422, 1);
        let made = e
            .call(0x00b6_8880, &args![this, geometry, 0u32, 0u32])
            .u32();
        assert_ne!(made, 0);
        assert_eq!(
            calls(&e, SKIN_PROPERTY_CONSTRUCT_00BB4540),
            vec![vec![made, this]]
        );
        // The switch off: flag 0x20 set, the controllers listed.
        e.mem.set_u8(SWITCH_011F9422, 0);
        e.mem.set_u8(SWITCH_011F94A6, 1);
        let controller = e.mem.alloc(0x34);
        e.mem.set_u32(geometry + 0xc, controller);
        assert_eq!(
            e.call(0x00b6_8880, &args![this, geometry, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0x20), 0x20);
        assert_eq!(
            calls(&e, LIST_HAS_0045BAD0),
            vec![vec![LIST_011F3728, controller]]
        );
    }

    #[test]
    fn fn_00b68880_makes_a_property_for_flag_0x13() {
        let mut e = engine();
        e.register(HAS_FLAG_004A2020, |_, a| u32::from(a[1] == 0x13).into_ret());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(PROPERTY_CONSTRUCT_004391C0, |e, a| {
            e.mem.set_u16(a[0] + 0x18, 0xffff);
            a[0].into_ret()
        });
        let this = property(&mut e, 2);
        let (geometry, _) = geometry(&mut e);
        assert_eq!(
            e.call(0x00b6_8880, &args![this, geometry, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x1c]]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }

    #[test]
    fn fn_00b68b10_sets_the_normal_map_flags() {
        let mut e = engine();
        let this = property(&mut e, 2);
        let normal = texture(&mut e, this, 1, 1, 1);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 0x18, 1);
        e.mem.set_u32(normal + 0x24, data);
        e.mem.set_u32(this + 0x20, 0x101);
        e.mem.set_u32(this + 0x38, 5);
        e.call(0x00b6_8b10, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0xcc) + 1), 1);
        e.mem.set_u32(this + 0x20, 0x80);
        e.call(0x00b6_8b10, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0x81);
        e.mem.set_u32(this + 0x20, 0x0002_0100);
        e.call(0x00b6_8b10, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0x0002_0000);
    }

    #[test]
    fn fn_00b68d30_sets_the_default_texture_when_missing() {
        let mut e = engine();
        let this = property(&mut e, 2);
        e.mem.set_u32(DEFAULT_TEXTURE_011F9510, 0x3333);
        e.call(0x00b6_8d30, &args![this]);
        assert_eq!(calls(&e, SLOTS + 0x100), vec![vec![this, 0, 0x3333]]);
        texture(&mut e, this, 1, 0, 1);
        e.call(0x00b6_8d30, &args![this]);
        assert_eq!(calls(&e, SLOTS + 0x100).len(), 1);
    }

    #[test]
    fn fn_00b68d50_creates_a_property() {
        let mut e = engine();
        e.register(LIGHTING_PROPERTY_CONSTRUCT_00B71720, STUB);
        let made = e.call(0x00b6_8d50, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE_010AE0D0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b6_8d50, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00b68dc0_reads_the_fields() {
        let mut e = engine();
        e.register(BASE_LOAD_BINARY_00B70260, STUB);
        e.register(STREAM_STEP_00A645B0, STUB);
        e.register(0x0e90_0000, |e, a| {
            let size = a[2];
            assert_eq!(e.mem.u32(a[3]), size);
            for byte in 0..size {
                e.mem.set_u8(a[1] + byte, 0x11 * (byte as u8 + 1));
            }
            size.into_ret()
        });
        let this = property(&mut e, 2);
        let stream = e.mem.alloc(0x250);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, 0x0e90_0000);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 4, 0x16);
        e.call(0x00b6_8dc0, &args![this, stream]);
        assert_eq!(
            calls(&e, BASE_LOAD_BINARY_00B70260),
            vec![vec![this, stream]]
        );
        let reads = calls(&e, 0x0e90_0000);
        let targets: Vec<(u32, u32)> = reads.iter().map(|w| (w[1], w[2])).collect();
        assert_eq!(targets[0], (this + 0xd8, 4));
        assert_eq!(targets[1].1, 1);
        assert_eq!(
            targets[2..],
            [
                (this + 0xe0, 4),
                (this + 0xe4, 4),
                (this + 0xe8, 4),
                (this + 0xec, 4)
            ]
        );
        assert_eq!(e.mem.u32(reader + 4), 21);
        assert_eq!(e.mem.u32(this + 0xd8), 0x4433_2211);
        // A newer stream skips the byte.
        e.mem.set_u32(stream + 4, 0x17);
        e.call_log = Some(vec![]);
        e.call(0x00b6_8dc0, &args![this, stream]);
        assert_eq!(calls(&e, 0x0e90_0000).len(), 5);
    }

    #[test]
    fn post_link_object_makes_the_set_and_clamps() {
        let mut e = engine();
        e.register(SHADER_PROPERTY_POST_LINK_OBJECT, STUB);
        e.register(TEXTURE_SET_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            a[0].into_ret()
        });
        e.register(POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.mem.set_f64(LOWER_LIMIT_01012060, 0.0);
        let this = property(&mut e, 2);
        e.mem.set_f32(this + 0xe0, 1.5);
        e.call(0x00b6_8fe0, &args![this, 0x4000u32]);
        let set = e.mem.u32(this + 0xa4);
        assert_ne!(set, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x38]]);
        assert_eq!(e.mem.f32(this + 0xe0), 1.0);
        for (value, want) in [(-0.5f32, 0.0f32), (0.25, 0.25), (f32::NAN, 1.0)] {
            e.mem.set_f32(this + 0xe0, value);
            e.call(0x00b6_8fe0, &args![this, 0x4000u32]);
            assert_eq!(e.mem.f32(this + 0xe0), want);
        }
        assert_eq!(calls(&e, NI_ALLOCATE).len(), 1);
        e.mem.set_u32(this + 0x20, 0x0100_0000);
        e.call_log = Some(vec![]);
        e.call(0x00b6_8fe0, &args![this, 0x4000u32]);
        assert!(calls(&e, SLOTS + 0x90).is_empty());
    }

    #[test]
    fn fn_00b69be0_merges_tangents_and_makes_the_extra_data() {
        let mut e = engine();
        for callee in [
            MEMCPY,
            REPLACE_DATA,
            SET_DATA_BLOCK_COUNT,
            SET_DATA_STREAM_COUNT,
            SET_DATA_BLOCK,
            SET_DATA_STREAM,
            REMOVE_EXTRA_DATA,
        ] {
            e.register(callee, STUB);
        }
        e.register(ADDITIONAL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let entry = e.mem.alloc(0x14);
        e.mem.set_u32(entry + 0xc, 0x6000);
        e.register_double(GET_EXTRA_DATA, move |_, _| entry.into_ret());
        e.mem.set_u32(EXTRA_DATA_NAME_011F94BC, 0x77);
        let this = property(&mut e, 2);
        let (geometry, data) = geometry(&mut e);
        e.mem.set_u16(data + 8, 4);
        e.mem.set_u16(data + 0xc, 1);
        e.mem.set_u32(data + 0x24, 0x100);
        let tangents = e.mem.alloc(0x14);
        e.mem.set_u32(tangents + 0xc, 0x200);
        e.mem.set_u32(tangents + 0x10, 0x300);
        e.mem.set_u32(this + 0xd0, tangents);
        e.mem.set_u32(geometry + 0x18, 0x9000);
        let added = e.call(0x00b6_9be0, &args![this, geometry]).u32();
        let buffer = calls(&e, MEMORY_ALLOCATE)[0][1];
        assert_eq!(buffer, 4 * 3 * 12);
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies.len(), 3);
        assert_eq!(
            (copies[0][1], copies[1][1], copies[2][1]),
            (0x100, 0x300, 0x200)
        );
        assert_eq!(copies[1][0] - copies[0][0], 48);
        assert_eq!(calls(&e, REPLACE_DATA)[0][6..], [1, 0x1000]);
        assert_eq!(e.mem.u32(this + 0xd4), 0x6000);
        assert_eq!(e.mem.u32(this + 0x20), 0x4000);
        assert_eq!(calls(&e, GET_EXTRA_DATA), vec![vec![0x9000, 0x77]]);
        assert_eq!(calls(&e, ADDITIONAL_DATA_CONSTRUCT), vec![vec![added, 4]]);
        assert_eq!(
            calls(&e, SET_DATA_BLOCK),
            vec![vec![added, 1, 0x6000, 0x80, 0]]
        );
        assert_eq!(
            calls(&e, SET_DATA_STREAM),
            vec![
                vec![added, 0, 1, 0, 4, 4, 0x10, 0x20],
                vec![added, 1, 1, 0x10, 4, 4, 0x10, 0x20],
            ]
        );
        assert_eq!((e.mem.u32(entry + 0xc), e.mem.u32(entry + 0x10)), (0, 0));
        assert_eq!(calls(&e, REMOVE_EXTRA_DATA), vec![vec![geometry, 0x77]]);
        // No extra data list: 0.
        e.mem.set_u32(geometry + 0x18, 0);
        e.mem.set_u32(this + 0xd0, 0);
        assert_eq!(e.call(0x00b6_9be0, &args![this, geometry]).u32(), 0);
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Reference counts on the word at +4: decrement, increment, and a destroyed object
    /// recorded through its slot +4 (`0x0f00_0004`).
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x2000);
        e.map(0x010a_9000, 0x1000);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.put_vtable(0x0f00_2000, &[0, 0x0f00_0004]);
        e.call_log = Some(vec![]);
        e
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_2000);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A string array (64 slots) with doubles for the size change and the two stores.
    fn strings(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(4 * 64);
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, 64);
        e.mem.set_u16(this + 0xe, 2);
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        let store = |e: &mut Engine, a: &[u32], index: u32| {
            let base = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[a.len() - 1]);
            e.mem.set_u32(base + 4 * index, v);
            if index >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, index as u16 + 1);
            }
        };
        e.register_double(STRINGS_SET_AT, move |e, a| {
            store(e, a, a[1]);
            Ret::default()
        });
        e.register_double(STRINGS_ADD_0096A610, move |e, a| {
            let index = u32::from(e.mem.u16(a[0] + 0xa));
            store(e, a, index);
            Ret::default()
        });
        this
    }

    fn slots(e: &Engine, strings: u32) -> Vec<u32> {
        let base = e.mem.u32(strings + 4);
        (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00b669f0_releases_both_pointers_and_lowers_the_count() {
        let mut e = rig();
        let first = counted(&mut e, 1);
        let second = counted(&mut e, 2);
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 8, first);
        e.mem.set_u32(this + 0x6c, second);
        e.mem.set_u32(OBJECT_COUNT_011F4400, 5);
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.call(0x00b6_75a0, &args![this, 1u32]);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0x6c)), (0, 0));
        assert_eq!(e.mem.u32(this), VTABLE_0101DCE4);
        assert_eq!(e.mem.u32(OBJECT_COUNT_011F4400), 4);
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![this, 0x74]]);
        e.call(0x00b6_75a0, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_SIZED_DELETE).len(), 1);
        assert_eq!(e.mem.u32(OBJECT_COUNT_011F4400), 3);
    }

    #[test]
    fn small_accessors_read_and_assign() {
        let mut e = rig();
        let this = e.mem.alloc(0x104);
        e.call(0x00b6_71d0, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0xd8), 3);
        let first = e.mem.alloc(0x24);
        let third = e.mem.alloc(0x24);
        e.mem.set_u32(first, 0x77);
        e.mem.set_u32(this + 0xac, first);
        e.mem.set_u32(this + 0xb4, third);
        assert_eq!(e.call(0x00b6_7280, &args![this]).u32(), 0x77);
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 0);
        e.mem.set_u32(third + 8, old);
        e.call(0x00b6_7330, &args![this, 2u32, new]);
        assert_eq!(e.mem.u32(third + 8), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![old]]);
    }

    #[test]
    fn set_texture_effect_data_assigns_and_resets_the_pass_state() {
        let mut e = rig();
        let this = e.mem.alloc(0x104);
        let data = counted(&mut e, 0);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00b6_75c0, &args![this, data]);
        assert_eq!((e.mem.u32(this + 0xdc), e.mem.u32(data + 4)), (data, 1));
        assert_eq!(e.mem.u32(this + 0x38), 0);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00b6_75c0, &args![this, data]);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + 0x38), 0);
    }

    #[test]
    fn fn_00b683e0_asks_for_passes_with_the_chosen_flags() {
        let mut e = rig();
        e.mem.set_u32(PASS_FLAGS_010A91D8, 0xd8);
        e.mem.set_u32(PASS_FLAGS_010A91D4, 0xd4);
        e.register(GET_ACCUMULATOR_00B4F5C0, |_, _| 0xacc0u32.into_ret());
        let mut slots = [0u32; 0x29];
        slots[0xa0 / 4] = 0x0f00_00a0;
        e.put_vtable(0x0f00_3000, &slots);
        e.register(0x0f00_00a0, |e, a| {
            e.mem.set_u32(a[3], 0x1234_5678);
            0xffffu32.into_ret()
        });
        let this = e.mem.alloc(0x104);
        e.mem.set_u32(this, 0x0f00_3000);
        assert_eq!(e.call(0x00b6_83e0, &args![this, 0x55u32]).u16(), 0x5678);
        e.mem.set_u32(SWITCH_011F95E4, 1);
        e.mem.set_u8(SHADER_FLAGS_011F91D8, 0x20);
        e.mem.set_u32(SHADER_LEVEL_011F91C0, 2);
        e.call(0x00b6_83e0, &args![this, 0x55u32]);
        e.mem.set_u8(BYTE_SWITCH_011F9427, 1);
        e.call(0x00b6_83e0, &args![this, 0x55u32]);
        let asked = calls(&e, 0x0f00_00a0);
        assert_eq!(asked[0][..3], [this, 0x55, 0xd4]);
        assert_eq!(asked[0][4..], [0, 0xacc0, 0]);
        assert_eq!(asked[1][2], 0xd8);
        assert_eq!(asked[2][2], 0xd4);
    }

    #[test]
    fn set_refraction_recurse_sets_lit_geometry_through_nodes() {
        let mut e = rig();
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_TRI_BASED_GEOM as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(SET_FLAG_00441130, |_, _| Ret::default());
        e.register(SET_VALUE_00B66700, |_, _| Ret::default());
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        let geometry = |e: &mut Engine, kind: u32| {
            let g = e.mem.alloc(0xb0);
            e.mem.set_u32(g, 0x0f00_e100);
            e.mem.set_u32(g + 0x10, 1);
            let p = e.mem.alloc(0x104);
            e.mem.set_u32(p + 0x1c, kind);
            e.mem.set_u32(g + 0xa8, p);
            (g, p)
        };
        let (lit, lit_property) = geometry(&mut e, 8);
        let (last, last_property) = geometry(&mut e, 12);
        let (other, _) = geometry(&mut e, 13);
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, 0x0f00_e100);
        e.mem.set_u32(node + 0x14, node);
        let buffer = e.mem.alloc(0x10);
        for (i, child) in [lit, 0, last, other].iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *child);
        }
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, 4);
        let words = args![node, 0x101u32, 0x7f80_0001u32, 1u32, 2.75f32, 0u32];
        e.call(0x00b6_8770, &words);
        let flags = calls(&e, SET_FLAG_00441130);
        assert_eq!(
            flags[..3],
            [
                vec![lit_property, 0xf, 0x101],
                vec![lit_property, 0x10, 1],
                vec![lit_property, 0x24, 0]
            ]
        );
        assert_eq!(flags.len(), 6);
        assert_eq!(
            calls(&e, SET_VALUE_00B66700),
            vec![
                vec![lit_property, 0x7fc0_0001],
                vec![last_property, 0x7fc0_0001]
            ]
        );
        assert_eq!(e.mem.u32(lit_property + 0xe4), 2);
        assert_eq!(e.mem.u32(last_property + 0xe4), 2);
        // Without the first byte: no period, and "flag and byte" bits off.
        e.call(
            0x00b6_8770,
            &args![lit, 0x100u32, 1.0f32, 0u32, 9.0f32, 1u32],
        );
        let flags = calls(&e, SET_FLAG_00441130);
        assert_eq!(
            flags[6..],
            [
                vec![lit_property, 0xf, 0x100],
                vec![lit_property, 0x10, 0],
                vec![lit_property, 0x24, 0]
            ]
        );
        assert_eq!(e.mem.u32(lit_property + 0xe4), 2);
    }

    #[test]
    fn fn_00b68c10_adds_the_water_fog_pass() {
        let mut e = rig();
        let list = e.mem.alloc(0x14);
        e.register_double(CHECK_CREATE_WATER_FOG_PASSES_00BA97B0, move |e, a| {
            if e.mem.u32(a[0] + 0x50) == 0 {
                e.mem.set_u32(a[0] + 0x50, list);
            }
            Ret::default()
        });
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let this = e.mem.alloc(0x104);
        let geometry = e.mem.alloc(0xb0);
        e.mem.set_f32(this + 0xe0, 0.5);
        assert_eq!(e.call(0x00b6_8c10, &args![this, geometry]).u32(), 0);
        assert!(calls(&e, CHECK_CREATE_WATER_FOG_PASSES_00BA97B0).is_empty());
        let cases: [(u8, u8, u16, u32); 8] = [
            (0, 0, 0, 0x2d8),
            (0, 0, 0x200, 0x2de),
            (0, 0x20, 0, 0x2e4),
            (0, 0x20, 0x200, 0x2ea),
            (2, 0, 0, 0x2d9),
            (2, 0, 0x200, 0x2df),
            (2, 0x20, 0, 0x2e5),
            (2, 0x20, 0x200, 0x2eb),
        ];
        let state = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, state);
        for (first, second, skin, expected) in cases {
            e.mem.set_u32(this + 0xe0, 0x7fc0_0000);
            e.mem.set_u8(this + 0x20, first);
            e.mem.set_u8(this + 0x24, second);
            e.mem.set_u16(state + 0x18, skin);
            assert_eq!(e.call(0x00b6_8c10, &args![this, geometry]).u32(), list);
            let added = calls(&e, RENDER_PASS_ARRAY_ADD).pop().unwrap();
            assert_eq!(added, vec![list, geometry, expected, 1, 0, 0, 0, 0, 0]);
        }
        // A list with passes is answered as it is.
        e.mem.set_u32(list + 0x10, 1);
        e.mem.set_f32(this + 0xe0, 0.0);
        assert_eq!(e.call(0x00b6_8c10, &args![this, geometry]).u32(), list);
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD).len(), 8);
    }

    #[test]
    fn fn_00b68ed0_saves_the_clamp_mode_set_and_four_words() {
        let mut e = rig();
        e.register(BASE_SAVE_BINARY_00BB7160, |_, _| Ret::default());
        e.register(0x0f00_0300, |_, _| 4u32.into_ret());
        e.register(0x0f00_0330, |_, _| Ret::default());
        let mut slots = [0u32; 0xd];
        slots[0x30 / 4] = 0x0f00_0330;
        e.put_vtable(0x0f00_3300, &slots);
        let file = e.mem.alloc(0x20);
        e.mem.set_u32(file + 0xc, 0x0f00_0300);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, 0x0f00_3300);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(0x104);
        e.mem.set_u32(this + 0xa4, 0xa4a4);
        e.call(0x00b6_8ed0, &args![this, stream]);
        assert_eq!(
            calls(&e, BASE_SAVE_BINARY_00BB7160),
            vec![vec![this, stream]]
        );
        assert_eq!(calls(&e, 0x0f00_0330), vec![vec![stream, 0xa4a4]]);
        let written: Vec<u32> = calls(&e, 0x0f00_0300).iter().map(|w| w[1] - this).collect();
        assert_eq!(written, vec![0xd8, 0xe0, 0xe4, 0xe8, 0xec]);
        assert_eq!(e.mem.u32(file + 4), 20);
    }

    #[test]
    fn get_viewer_strings_lists_textures_and_values() {
        let mut e = rig();
        e.mem.set_u32(CLASS_RTTI_011FA010, 0x0c1a_0000);
        e.register(LIGHTING_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| {
            (0x1000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_FLAG, |_, a| {
            (0x2000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_FLOAT, |_, a| {
            (0x3000_0000 | (a[0] & 0xfff) | (f32::from_bits(a[1]) as u32) << 12).into_ret()
        });
        e.register(VIEWER_STRING_INT, |_, a| {
            (0x4000_0000 | (a[0] & 0xfff) | a[1] << 12).into_ret()
        });
        e.register(VIEWER_STRING_SHORT, |_, a| {
            (0x5000_0000 | (a[0] & 0xfff) | a[1] << 12).into_ret()
        });
        e.register(VIEWER_STRING_TEXT, |_, a| {
            (0x6000_0000 | (a[0] & 0xfff) | (a[1] & 0xff) << 12).into_ret()
        });
        e.register(VIEWER_STRING_OBJECT, |_, a| {
            (0x7000_0000 | (a[0] & 0xfff) | (a[1] & 0xff) << 12).into_ret()
        });
        // Textures 0x..1 have a name (their low byte + 0x80), the others none.
        e.register(GET_TEXTURE_NAME, |_, a| {
            if a[0] & 1 != 0 {
                (a[0] & 0xff) + 0x80
            } else {
                0
            }
            .into_ret()
        });
        let mut vt = [0u32; 0x45];
        vt[0xe0 / 4] = 0x0f00_00e0;
        vt[0x10c / 4] = 0x0f00_010c;
        vt[0x110 / 4] = 0x0f00_0110;
        e.put_vtable(0x0f00_4000, &vt);
        e.register(0x0f00_00e0, |_, _| Ret::default());
        e.register(0x0f00_010c, |_, _| 2.0f32.into_ret());
        e.register(0x0f00_0110, |_, _| 3.0f32.into_ret());
        let this = e.mem.alloc(0x104);
        e.mem.set_u32(this, 0x0f00_4000);
        let mut arrays = vec![];
        for kind in 0..6u32 {
            let array = e.mem.alloc(4 * 9);
            e.mem.set_u32(this + 0xac + 4 * kind, array);
            arrays.push(array);
        }
        e.mem.set_u32(arrays[0], 0x11);
        e.mem.set_u32(arrays[1], 0x22);
        e.mem.set_u32(arrays[1] + 4, 0x23);
        e.mem.set_u32(arrays[4], 0x45);
        e.mem.set_u32(this + 0x20, 0x0001_0081);
        e.mem.set_f32(this + 0xe0, 1.0);
        e.mem.set_u32(this + 0xe4, 7);
        let data = e.mem.alloc(0x60);
        for (offset, _) in TEX_EFFECT_LINES {
            e.mem.set_f32(data + offset, offset as f32);
        }
        e.mem.set_u32(this + 0xdc, data);
        let strings = strings(&mut e);
        e.call(0x00b6_90d0, &args![this, strings]);
        let mut expected = vec![
            0x1000_0000,
            0x6000_0000 | (LABEL_BASE_DIFF & 0xfff) | 0x91 << 12,
            0x7000_0000 | (LABEL_BASE_NORMAL & 0xfff) | 0x22 << 12,
            0x6000_0000 | (LABEL_MULTI_NORMAL & 0xfff) | 0xa3 << 12,
            0x6000_0000 | (LABEL_ENVMAP & 0xfff) | 0xc5 << 12,
            0x2000_0000 | (LABEL_CLAMP & 0xffff),
            0x3000_1000 | (LABEL_REFRACTION_POWER & 0xfff),
            0x4000_7000 | (LABEL_REFRACTION_PERIOD & 0xfff),
            0x3000_2000 | (LABEL_SPECULAR_LOD & 0xfff),
            0x3000_3000 | (LABEL_ENVMAP_LOD & 0xfff),
            0x7000_0000 | (LABEL_TEX_EFFECT_DATA & 0xfff) | (data & 0xff) << 12,
        ];
        for (offset, label) in TEX_EFFECT_LINES {
            expected.push(0x3000_0000 | (label & 0xfff) | offset << 12);
        }
        expected[0] |= 0x0c1a_0000 & 0xffff;
        assert_eq!(slots(&e, strings), expected);
        assert_eq!(calls(&e, STRINGS_ADD_0096A610).len(), 1);
        // Landscape texturing: the layer lines.
        e.mem.set_u32(this + 0x20, 0x4000);
        e.mem.set_u16(this + 0xa8, 2);
        e.mem.set_u32(this + 0xdc, 0);
        let speculars = e.mem.alloc(9);
        e.mem.set_u8(speculars + 1, 1);
        e.mem.set_u32(this + 0xcc, speculars);
        e.mem.set_u32(arrays[0] + 4, 0x13);
        e.mem.set_u32(arrays[2] + 4, 0x34);
        e.register(0x0f00_00e0, |_, _| 1u32.into_ret());
        let strings = self::strings(&mut e);
        e.call(0x00b6_90d0, &args![this, strings]);
        assert_eq!(
            slots(&e, strings),
            vec![
                // The label word 0x0c1a_0000 has no low 16 bits, so only the kind remains.
                0x1000_0000,
                0x2000_0000 | (LABEL_LANDSCAPE_TEXTURING & 0xffff),
                0x5000_2000 | (LABEL_LANDSCAPE_TEXTURES & 0xfff),
                0x6000_0000 | (LABEL_BASE_TEX & 0xfff) | 0x91 << 12,
                0x7000_0000 | (LABEL_NORMAL_MAP & 0xfff) | 0x22 << 12,
                0x6000_0000 | (LABEL_BASE_TEX & 0xfff) | 0x93 << 12,
                0x6000_0000 | (LABEL_NORMAL_MAP & 0xfff) | 0xa3 << 12,
                0x2000_0000 | (LABEL_SPECULAR & 0xffff),
                0x7000_0000 | (LABEL_GLOW_MAP & 0xfff) | 0x34 << 12,
                0x6000_0000 | (LABEL_ENVMAP & 0xfff) | 0xc5 << 12,
            ]
        );
    }
}
