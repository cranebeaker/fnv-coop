//! `BSShader/imagespaceshaderparam.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The clean-up `00b8a7e0` (this unit, unnamed; `thiscall`, no stack words) the function
/// starts with.
const CLEAN_UP_00B8A7E0: u32 = 0x00b8_a7e0;
/// The Ni allocation `00aa13e0` (`cdecl`, size) and
/// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map, `00b7e330`, `thiscall`: the
/// renderer and a kind word).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const CONSTANT_MAP_CONSTRUCT: u32 = 0x00b7_e330;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// The memory manager singleton (`011f6238`), its allocate (`00aa3e40`) and deallocate
/// (`00aa4060`) (`thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `"Constant Group"` (`010aff90`) and `""` (`01011584`), the strings the maps get.
const CONSTANT_GROUP_010AFF90: u32 = 0x010a_ff90;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;

/// `NiPointer<T>::operator=(T *)` expanded inline.
fn assign_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

/// A constant map of the given kind (0x44 bytes, `00b7e330` with the renderer `011f9508`; null
/// when the allocation fails) assigned to the `NiPointer` at `slot`.
fn make_constant_map(e: &mut Engine, slot: u32, kind: u32) {
    let block = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
    let map = if block != 0 {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call(CONSTANT_MAP_CONSTRUCT, &args![block, renderer, kind])
            .u32()
    } else {
        0
    };
    assign_pointer(e, slot, map);
}

/// A group of `count` registers: the count stored at `count_slot`; when it is above 0
/// (signed) the old buffer at `buffer_slot` is freed (when set), a new one of `count * 4`
/// four-byte words (`operator new[]` size check) allocated into it and the map at `map_slot`
/// told through its vtable slot +8: ("Constant Group", 0x10000000, 0, 0, count, "", count *
/// 16, 4, buffer, 0).
fn make_constant_group(e: &mut Engine, base: u32, count: u32, slots: (u32, u32, u32)) {
    let (count_slot, buffer_slot, map_slot) = slots;
    e.mem.set_u32(base + count_slot, count);
    if (count as i32) <= 0 {
        return;
    }
    let old = e.mem.u32(base + buffer_slot);
    if old != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    }
    let elements = e.mem.u32(base + count_slot).wrapping_mul(4);
    let product = u64::from(elements) * 4;
    let bytes = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    let buffer = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let count = e.mem.u32(base + count_slot);
    let map = e.mem.u32(base + map_slot);
    e.mem.set_u32(base + buffer_slot, buffer);
    e.vcall(
        map,
        8,
        &args![
            CONSTANT_GROUP_010AFF90,
            0x1000_0000u32,
            0u32,
            0u32,
            count,
            EMPTY_STRING_01011584,
            count.wrapping_shl(4),
            4u32,
            buffer,
            0u32
        ],
    );
}

// Translated from 00b8ab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderParam::InitConstantMap` (Xbox PDB), `thiscall` with two stack words: the
/// vertex and pixel register counts. Runs the clean-up `00b8a7e0`, makes the vertex constant
/// map (kind 0) into the `NiPointer` at +0x4c and the pixel one (kind 2) into +0x50
/// (`NiD3DShaderConstantMap::NiD3DShaderConstantMap` with the renderer `011f9508` on 0x44
/// bytes). Then the vertex group: +0x40 = first + 3, buffer at +0x3c, map +0x4c; and the pixel
/// group: +0x48 = second, buffer at +0x44, map +0x50; a group with a count above 0 gets a new
/// buffer of 16 bytes per register (the old one freed) and is added to its map as
/// `"Constant Group"` (vtable slot +8).
// Translation incomplete: 00b8ab10 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn image_space_shader_param_init_constant_map(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    pixel_count: u32,
) {
    let base = this.addr();
    e.call(CLEAN_UP_00B8A7E0, &args![base]);
    make_constant_map(e, base + 0x4c, 0);
    make_constant_map(e, base + 0x50, 2);
    make_constant_group(e, base, vertex_count.wrapping_add(3), (0x40, 0x3c, 0x4c));
    make_constant_group(e, base, pixel_count, (0x48, 0x44, 0x50));
}

// Translated from 00b8a7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceshaderparam.cpp`): the clean-up the constant map
/// set-up (`00b8ab10`) starts with, `thiscall`, no stack words. The constant maps held at +0x4c
/// and +0x50 are released (`InterlockedDecrement` on their +4, deleted through vtable slot +4
/// at 0) and cleared; the register buffers at +0x3c and +0x44 are given back to the memory
/// manager (`00aa4060` on `011f6238`) and cleared when set.
pub fn fn_00b8a7e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for slot in [0x4cu32, 0x50] {
        let map = e.mem.u32(base + slot);
        if map != 0 {
            let left = e
                .call(IMPORT_INTERLOCKED_DECREMENT, &args![map.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(map, 4, &args![]);
            }
            e.mem.set_u32(base + slot, 0);
        }
    }
    for slot in [0x3cu32, 0x44] {
        let buffer = e.mem.u32(base + slot);
        if buffer != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
            e.mem.set_u32(base + slot, 0);
        }
    }
}

/// `4294967296.0` (`float`, `01099378`): added to a negative `FILD` to read
/// the word as unsigned.
const TWO_TO_32_FLOAT: u32 = 0x0109_9378;
/// `0.5` (`double`, `01011588`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The byte `00b8a7c0` tests (`011f9440`).
const FLAG_011F9440: u32 = 0x011f_9440;
/// `_memcpy` (engine map), cdecl (destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// `_memset` (engine map), cdecl (destination, byte, size).
const MEMSET: u32 = 0x00ec_61c0;
/// `NiTMapBase<..., unsigned int, ...>::SetAt` (engine map; shared body),
/// thiscall (key, value), `RET 8`.
const MAP_SET_AT: u32 = 0x0084_4700;
/// The vtable `ImageSpaceShaderParam`'s constructor and destructor store
/// (`010aff8c`; no RTTI locator before it).
const SHADER_PARAM_VTABLE: u32 = 0x010a_ff8c;
/// `ImageSpaceEffectParam::ImageSpaceEffectParam` (engine map), thiscall.
const EFFECT_PARAM_CONSTRUCT: u32 = 0x004e_ba20;
/// The base destructor (`teswater.rs`), thiscall.
const EFFECT_PARAM_DESTRUCT: u32 = 0x004e_ba90;
/// The map's destructor body (+0x78), thiscall.
const MAP_DESTRUCT_006E2150: u32 = 0x006e_2150;
/// The Gamebryo `malloc` and `free` (`run_00aa1010.rs`), cdecl.
const NI_MALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// The compiler's array destructor helper (engine map: `eh vector
/// destructor iterator`), stdcall (array, element size, count, element
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The `NiPointer` destructor (`run_0045cb90.rs`), passed as an address.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Vtables (RTTI): `NiTArray<NiTexturingProperty::FilterMode,
/// NiTMallocInterface<...>>`, `NiTArray<NiPointer<NiTexture>,
/// NiTNewInterface<...>>`, `NiTObjectArray<NiPointer<NiTexture>>`,
/// `NiTPrimitiveArray<NiTexturingProperty::FilterMode>`,
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, unsigned int>>, ...>` and
/// `NiTMap<unsigned int, unsigned int>`.
const FILTER_MODE_ARRAY_VTABLE: u32 = 0x0102_32c4;
const TEXTURE_ARRAY_VTABLE: u32 = 0x0103_d3e8;
const TEXTURE_OBJECT_ARRAY_VTABLE: u32 = 0x0103_d3f0;
const FILTER_MODE_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0102_32cc;
const MAP_BASE_VTABLE: u32 = 0x0106_cf88;
const MAP_VTABLE: u32 = 0x0106_cf68;

/// `FILD dword` of an unsigned word: the signed value, plus `2^32` (the
/// `float` at `01099378`) when it is negative.
fn unsigned_to_x87(e: &Engine, word: u32) -> f64 {
    let value = f64::from(word as i32);
    if (word as i32) < 0 {
        x87_add(value, x87_m32(e.global::<u32>(TWO_TO_32_FLOAT)))
    } else {
        value
    }
}

// Translated from 00b8a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderParam::SetTexOffset` (engine map): for `index` below 3,
/// writes the texture's half-texel offset to vertex constant `index` (the
/// buffer at +0x3C, 16 bytes each): `(0.5 / width, 0.5 / height, 0, 0)`
/// when `index` is 0 or the byte at `+0x54 + index` is set, else all 0.
pub fn image_space_shader_param_set_tex_offset(
    e: &mut Engine,
    this: Ptr,
    index: i32,
    width: u32,
    height: u32,
) {
    if index >= 3 {
        return;
    }
    let (x, y) = if index == 0
        || e.mem
            .u8(this.addr().wrapping_add(index as u32).wrapping_add(0x54))
            != 0
    {
        let half: f64 = e.global(HALF_DOUBLE);
        let width = unsigned_to_x87(e, width);
        let x = x87_store32(x87_div(half, width));
        let height = unsigned_to_x87(e, height);
        let y = x87_store32(x87_div(half, height));
        (x, y)
    } else {
        (0.0f32.to_bits(), 0.0f32.to_bits())
    };
    let buffer = e.mem.u32(this.addr() + 0x3c);
    let slot = buffer.wrapping_add((index as u32).wrapping_mul(16));
    e.mem.set_u32(slot, x87_quiet_bits(x));
    e.mem.set_u32(slot + 4, x87_quiet_bits(y));
    e.mem.set_u32(slot + 8, 0.0f32.to_bits());
    e.mem.set_u32(slot + 0xc, 0.0f32.to_bits());
}

// Translated from 00b8a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderParam::SetPixelConstant` (engine map): the four floats
/// into pixel constant `index` (the buffer at +0x44, 16 bytes each).
pub fn image_space_shader_param_set_pixel_constant(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let buffer = e.mem.u32(this.addr() + 0x44);
    let slot = buffer.wrapping_add(index.wrapping_mul(16));
    e.mem.set_u32(slot, x87_quiet_bits(x.to_bits()));
    e.mem.set_u32(slot + 4, x87_quiet_bits(y.to_bits()));
    e.mem.set_u32(slot + 8, x87_quiet_bits(z.to_bits()));
    e.mem.set_u32(slot + 0xc, x87_quiet_bits(w.to_bits()));
}

// Translated from 00b8a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x28 becomes 7 when the byte at
/// `011f9440` is set, else 0.
pub fn fn_00b8a7c0(e: &mut Engine, this: Ptr) {
    let value = if e.global::<u8>(FLAG_011F9440) != 0 {
        7
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 0x28, value);
}

// Translated from 00b8a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies `size` bytes from `source` into pixel
/// constant `index` onwards (the buffer at +0x44, 16 bytes each).
pub fn fn_00b8a8b0(e: &mut Engine, this: Ptr, index: u32, source: u32, size: u32) {
    let destination = (index << 4).wrapping_add(e.mem.u32(this.addr() + 0x44));
    e.call(MEMCPY, &args![destination, source, size]);
}

// Translated from 00b8a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetAt(key, value)` on the
/// `NiTMap<unsigned int, unsigned int>` at +0x78 (a jump to `00844700`).
pub fn fn_00b8a8d0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    e.call(MAP_SET_AT, &args![this.addr() + 0x78, key, value]);
}

/// Releases the reference-counted object at `slot` (`InterlockedDecrement`
/// on its count at +4; at zero its vtable slot +4).
fn release_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object + 4])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b8a8e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b8a8e0 - the C++ exception frame (FS:[0] chain, unwind states 5 down to 0 between the members) is not translated.
/// The `ImageSpaceShaderParam` destructor (engine map unnamed): the vtable,
/// the clean-up `00b8a7e0`, the map at +0x78 (`006e2150`), the filter-mode
/// array at +0x68 (its vtable, its buffer +0x6C freed), the texture array
/// at +0x58 (its vtable; its buffer +0x5C, when set, destroyed as an
/// `operator new[]` array of `NiPointer`s: the count in the word before it,
/// then the block freed through the memory manager), the two constant maps
/// (+0x50, +0x4C) released, then the base destructor (`004eba90`).
pub fn fn_00b8a8e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_PARAM_VTABLE);
    fn_00b8a7e0(e, this);
    e.call(MAP_DESTRUCT_006E2150, &args![base + 0x78]);
    let filter_modes = e.mem.u32(base + 0x6c);
    e.mem.set_u32(base + 0x68, FILTER_MODE_ARRAY_VTABLE);
    e.call(NI_FREE, &args![filter_modes]);
    let textures = e.mem.u32(base + 0x5c);
    e.mem.set_u32(base + 0x58, TEXTURE_ARRAY_VTABLE);
    if textures != 0 {
        let count = e.mem.u32(textures - 4);
        e.call(
            VECTOR_DESTRUCT,
            &args![textures, 4u32, count, POINTER_DESTRUCT],
        );
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, textures - 4]);
    }
    release_pointer(e, base + 0x50);
    release_pointer(e, base + 0x4c);
    e.call(EFFECT_PARAM_DESTRUCT, &args![base]);
}

// Translated from 00b8a9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b8a9e0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 between the members) is not translated.
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (engine map): the base
/// (`ImageSpaceEffectParam`), the vtable, the words +0x24 and +0x28 and the
/// pointers +0x3C to +0x50 cleared, the floats +0x2C to +0x38 `0.0`, the
/// texture array at +0x58 (`NiTObjectArray`: buffer 0, sizes 0 and grow by
/// 1), the filter-mode array at +0x68 (`NiTPrimitiveArray`, the same), the
/// map at +0x78 (`NiTMap<unsigned int, unsigned int>`: 0x25 buckets of a
/// zeroed `00aa1070` block, count 0), and the halfword +0x54 and byte +0x56
/// cleared. Returns `this`.
pub fn image_space_shader_param_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_PARAM_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_PARAM_VTABLE);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0);
    for offset in [0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    for offset in [0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u16(base + 0x60, 0);
    e.mem.set_u16(base + 0x66, 1);
    e.mem.set_u16(base + 0x62, 0);
    e.mem.set_u16(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x58, TEXTURE_OBJECT_ARRAY_VTABLE);
    e.mem.set_u16(base + 0x70, 0);
    e.mem.set_u16(base + 0x76, 1);
    e.mem.set_u16(base + 0x72, 0);
    e.mem.set_u16(base + 0x74, 0);
    e.mem.set_u32(base + 0x6c, 0);
    e.mem
        .set_u32(base + 0x68, FILTER_MODE_PRIMITIVE_ARRAY_VTABLE);
    e.mem.set_u32(base + 0x78, MAP_BASE_VTABLE);
    e.mem.set_u32(base + 0x7c, 0x25);
    e.mem.set_u32(base + 0x84, 0);
    let buckets = e.call(NI_MALLOC, &args![0x94u32]).u32();
    e.mem.set_u32(base + 0x80, buckets);
    let size = e.mem.u32(base + 0x7c).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u32(base + 0x78, MAP_VTABLE);
    e.mem.set_u16(base + 0x54, 0);
    e.mem.set_u8(base + 0x56, 0);
    this
}

// Translated from 00b8aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderParam`'s scalar deleting destructor (engine map
/// unnamed): the destructor (`00b8a8e0`), then, with bit 0 of `flags`,
/// the object freed through the memory manager. Returns `this`.
pub fn fn_00b8aaf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8a8e0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b8a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, five words: an index and four floats): the four
/// floats into vertex constant `index + 3` (the buffer at +0x3c, 16 bytes each; the code
/// indexes `(index * 4 + 0xc) * 4`), each through `FLD`/`FSTP`.
pub fn fn_00b8a760(e: &mut Engine, this: Ptr, index: u32, x: f32, y: f32, z: f32, w: f32) {
    let buffer = e.mem.u32(this.addr() + 0x3c);
    let slot = buffer.wrapping_add(index.wrapping_mul(4).wrapping_add(0xc).wrapping_mul(4));
    e.mem.set_u32(slot, x87_quiet_bits(x.to_bits()));
    e.mem
        .set_u32(slot.wrapping_add(4), x87_quiet_bits(y.to_bits()));
    e.mem
        .set_u32(slot.wrapping_add(8), x87_quiet_bits(z.to_bits()));
    e.mem
        .set_u32(slot.wrapping_add(0xc), x87_quiet_bits(w.to_bits()));
}

// Translated from 00b8a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: an index and the address of four floats):
/// the four floats into pixel constant `index` (the buffer at +0x44, 16 bytes each). The code
/// reads the second, third and fourth floats first (each through `FLD`/`FSTP` into a stack
/// temporary), then copies the first and then the three temporaries (`FLD`/`FSTP` again).
pub fn fn_00b8a860(e: &mut Engine, this: Ptr, index: u32, source: u32) {
    let second = x87_quiet_bits(e.mem.u32(source.wrapping_add(4)));
    let buffer = e.mem.u32(this.addr() + 0x44);
    let third = x87_quiet_bits(e.mem.u32(source.wrapping_add(8)));
    let fourth = x87_quiet_bits(e.mem.u32(source.wrapping_add(0xc)));
    let slot = buffer.wrapping_add(index.wrapping_mul(16));
    let first = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(slot, first);
    e.mem.set_u32(slot.wrapping_add(4), x87_quiet_bits(second));
    e.mem.set_u32(slot.wrapping_add(8), x87_quiet_bits(third));
    e.mem
        .set_u32(slot.wrapping_add(0xc), x87_quiet_bits(fourth));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_a760, fn_00b8a760(Ptr, u32, f32, f32, f32, f32)),
        entry!(0x00b8_a860, fn_00b8a860(Ptr, u32, u32)),
        entry!(
            0x00b8_ab10,
            image_space_shader_param_init_constant_map(Ptr, u32, u32)
        ),
        entry!(0x00b8_a7e0, fn_00b8a7e0(Ptr)),
        entry!(
            0x00b8_a6e0,
            image_space_shader_param_set_tex_offset(Ptr, i32, u32, u32)
        ),
        entry!(
            0x00b8_a790,
            image_space_shader_param_set_pixel_constant(Ptr, u32, f32, f32, f32, f32)
        ),
        entry!(0x00b8_a7c0, fn_00b8a7c0(Ptr)),
        entry!(0x00b8_a8b0, fn_00b8a8b0(Ptr, u32, u32, u32)),
        entry!(0x00b8_a8d0, fn_00b8a8d0(Ptr, u32, u32)),
        entry!(0x00b8_a8e0, fn_00b8a8e0(Ptr)),
        entry!(0x00b8_a9e0, image_space_shader_param_construct(Ptr) -> Ptr),
        entry!(0x00b8_aaf0, fn_00b8aaf0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_GROUP: u32 = 0x0f00_0000;
    const DELETE: u32 = 0x0f00_0010;
    const MAP_VTABLE: u32 = 0x0f01_0000;
    const RENDERER: u32 = 0x0f02_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
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
        e.register(CLEAN_UP_00B8A7E0, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(ADD_GROUP, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(MAP_VTABLE, &[0, DELETE, ADD_GROUP]);
        e.map(0x011f_9000, 0x1000);
        e.set_global::<u32>(RENDERER_011F9508, RENDERER);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn init_constant_map_makes_both_maps_and_groups() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x3c, 0x1234);
        e.call(0x00b8_ab10, &args![this, 5u32, 2u32]);
        assert_eq!(calls(&e, CLEAN_UP_00B8A7E0), vec![vec![this]]);
        let vertex_map = e.mem.u32(this + 0x4c);
        let pixel_map = e.mem.u32(this + 0x50);
        assert_eq!(
            calls(&e, CONSTANT_MAP_CONSTRUCT),
            vec![vec![vertex_map, RENDERER, 0], vec![pixel_map, RENDERER, 2]]
        );
        assert_eq!(e.mem.u32(vertex_map + 4), 1);
        assert_eq!(e.mem.u32(this + 0x40), 8);
        assert_eq!(e.mem.u32(this + 0x48), 2);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1234]]
        );
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x80], vec![MEMORY_MANAGER, 0x20]]
        );
        let vertex_buffer = e.mem.u32(this + 0x3c);
        let pixel_buffer = e.mem.u32(this + 0x44);
        assert_eq!(
            calls(&e, ADD_GROUP),
            vec![
                vec![
                    vertex_map,
                    CONSTANT_GROUP_010AFF90,
                    0x1000_0000,
                    0,
                    0,
                    8,
                    EMPTY_STRING_01011584,
                    0x80,
                    4,
                    vertex_buffer,
                    0
                ],
                vec![
                    pixel_map,
                    CONSTANT_GROUP_010AFF90,
                    0x1000_0000,
                    0,
                    0,
                    2,
                    EMPTY_STRING_01011584,
                    0x20,
                    4,
                    pixel_buffer,
                    0
                ],
            ]
        );
    }

    #[test]
    fn init_constant_map_skips_empty_groups() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.call(0x00b8_ab10, &args![this, (-3i32) as u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, ADD_GROUP).is_empty());
        // A failed allocation leaves the map pointer null.
        let mut e = engine();
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x54);
        e.call(0x00b8_ab10, &args![this, (-3i32) as u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod clean_up_tests {
    use super::*;

    const DELETE: u32 = 0x0f10_0004;
    const VTABLE: u32 = 0x0f11_0000;

    #[test]
    fn fn_00b8a7e0_releases_the_maps_and_frees_the_buffers() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x54);
        let kept = e.mem.alloc(8);
        e.mem.set_u32(kept, VTABLE);
        e.mem.set_u32(kept + 4, 2);
        let dropped = e.mem.alloc(8);
        e.mem.set_u32(dropped, VTABLE);
        e.mem.set_u32(dropped + 4, 1);
        e.mem.set_u32(this + 0x4c, kept);
        e.mem.set_u32(this + 0x50, dropped);
        e.mem.set_u32(this + 0x3c, 0x0e00_1000);
        e.mem.set_u32(this + 0x44, 0);
        e.call(0x00b8_a7e0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(kept + 4), 1);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<_> = log.iter().filter(|c| c.0 == DELETE).collect();
        assert_eq!(deleted, vec![&(DELETE, vec![dropped])]);
        let freed: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_DEALLOCATE).collect();
        assert_eq!(
            freed,
            vec![&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x0e00_1000])]
        );
    }
}

#[cfg(test)]
mod shader_param_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_9000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global(TWO_TO_32_FLOAT, 4_294_967_296.0f32);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e
    }

    fn floats(e: &Engine, at: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(at + offset))
    }

    #[test]
    fn set_tex_offset_writes_half_texel_offsets() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x3c, buffer);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_f32(buffer + offset, 9.0);
        }
        e.call(0x00b8_a6e0, &args![this, 0u32, 4u32, 0xffff_ffffu32]);
        let expected_y = (0.5f64 / 4_294_967_295.0) as f32;
        assert_eq!(floats(&e, buffer), [0.125, expected_y, 0.0, 0.0]);
        // Index 1 without its byte: zeros; with it: the offsets.
        e.call(0x00b8_a6e0, &args![this, 1u32, 8u32, 2u32]);
        assert_eq!(floats(&e, buffer + 16), [0.0; 4]);
        e.mem.set_u8(this + 0x55, 1);
        e.call(0x00b8_a6e0, &args![this, 1u32, 8u32, 2u32]);
        assert_eq!(floats(&e, buffer + 16), [0.0625, 0.25, 0.0, 0.0]);
        // Index 3 and above: nothing.
        e.call(0x00b8_a6e0, &args![this, 3u32, 8u32, 2u32]);
        assert_eq!(floats(&e, buffer + 32), [9.0; 4]);
    }

    #[test]
    fn set_pixel_constant_writes_four_floats() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x44, buffer);
        e.call(
            0x00b8_a790,
            &args![this, 2u32, 1.0f32, 2.0f32, 3.0f32, -4.0f32],
        );
        assert_eq!(floats(&e, buffer + 32), [1.0, 2.0, 3.0, -4.0]);
    }

    #[test]
    fn fn_00b8a7c0_follows_the_flag() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x28, 3);
        e.call(0x00b8_a7c0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        e.set_global(FLAG_011F9440, 1u8);
        e.call(0x00b8_a7c0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 7);
    }

    #[test]
    fn fn_00b8a8b0_copies_into_the_pixel_constants() {
        let mut e = engine();
        e.register(MEMCPY, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x44, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00b8_a8b0, &args![this, 3u32, 0x6000u32, 0x20u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x5030, 0x6000, 0x20]]);
    }

    #[test]
    fn fn_00b8a8d0_sets_the_map_entry() {
        let mut e = engine();
        e.register(MAP_SET_AT, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x00b8_a8d0, &args![this, 5u32, 6u32]);
        assert_eq!(calls_to(&e, MAP_SET_AT), vec![vec![this + 0x78, 5, 6]]);
    }

    fn destructor_engine() -> Engine {
        let mut e = engine();
        for addr in [
            MAP_DESTRUCT_006E2150,
            NI_FREE,
            VECTOR_DESTRUCT,
            MEMORY_DEALLOCATE,
            EFFECT_PARAM_DESTRUCT,
            0x0000_7004,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        e
    }

    #[test]
    fn destructor_releases_every_member() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x90);
        let textures = e.mem.alloc(0x10) + 4;
        e.mem.set_u32(textures - 4, 3);
        e.mem.set_u32(this + 0x5c, textures);
        e.mem.set_u32(this + 0x6c, 0x7700);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, 0x0f00_0000);
        e.mem.set_u32(map + 4, 1);
        e.mem.set_u32(this + 0x50, map);
        e.call_log = Some(vec![]);
        e.call(0x00b8_aaf0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), SHADER_PARAM_VTABLE);
        assert_eq!(e.mem.u32(this + 0x58), TEXTURE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x68), FILTER_MODE_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, MAP_DESTRUCT_006E2150), vec![vec![this + 0x78]]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![textures, 4, 3, POINTER_DESTRUCT]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, textures - 4],
                vec![MEMORY_MANAGER, this]
            ]
        );
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![map]]);
        assert_eq!(calls_to(&e, EFFECT_PARAM_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn deleting_destructor_keeps_the_object_without_the_flag() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b8_aaf0, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, VECTOR_DESTRUCT).is_empty());
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        e.call(0x00b8_a8e0, &args![this]);
        assert_eq!(calls_to(&e, EFFECT_PARAM_DESTRUCT).len(), 2);
    }

    #[test]
    fn constructor_builds_the_members() {
        let mut e = engine();
        e.register(EFFECT_PARAM_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_MALLOC, |_, _| Ret {
            eax: 0x8800,
            ..Ret::default()
        });
        e.register(MEMSET, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        for offset in (0..0x88).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b8_a9e0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, EFFECT_PARAM_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SHADER_PARAM_VTABLE);
        for offset in (0x24..0x54).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x54) & 0x00ff_ffff, 0);
        assert_eq!(e.mem.u32(this + 0x58), TEXTURE_OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(e.mem.u16(this + 0x66), 1);
        assert_eq!(e.mem.u32(this + 0x68), FILTER_MODE_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 0x76), 1);
        assert_eq!(e.mem.u32(this + 0x78), MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x7c), 0x25);
        assert_eq!(e.mem.u32(this + 0x80), 0x8800);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(calls_to(&e, NI_MALLOC), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x8800, 0, 0x94]]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b8a760_writes_vertex_constant_index_plus_3() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let buffer = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x3c, buffer);
        e.call(
            0x00b8_a760,
            &args![
                this,
                1u32,
                1.5f32,
                -2.0f32,
                f32::from_bits(0x7f80_0001),
                4.0f32
            ],
        );
        let slot = buffer + 0x40;
        assert_eq!(e.mem.f32(slot), 1.5);
        assert_eq!(e.mem.f32(slot + 4), -2.0);
        // A signalling NaN comes out quiet.
        assert_eq!(e.mem.u32(slot + 8), 0x7fc0_0001);
        assert_eq!(e.mem.f32(slot + 0xc), 4.0);
    }

    #[test]
    fn fn_00b8a860_copies_four_floats_to_pixel_constant_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let buffer = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x44, buffer);
        let source = e.mem.alloc(0x10);
        for (i, v) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_f32(source + 4 * i as u32, *v);
        }
        e.mem.set_u32(source + 4, 0xff80_0002);
        e.call(0x00b8_a860, &args![this, 2u32, source]);
        let slot = buffer + 0x20;
        assert_eq!(e.mem.f32(slot), 0.25);
        assert_eq!(e.mem.u32(slot + 4), 0xffc0_0002);
        assert_eq!(e.mem.f32(slot + 8), 0.75);
        assert_eq!(e.mem.f32(slot + 0xc), 1.0);
    }
}
