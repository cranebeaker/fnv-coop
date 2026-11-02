//! `BSShader/tallgrassshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1c while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_TALL_GRASS: u32 = 0x1c;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bacd00` (engine map unit `tallgrassshader.cpp`, `thiscall`,
/// `RET 4`: the declaration).
const TALL_GRASS_SHADER_CONSTRUCT_00BACD00: u32 = 0x00ba_cd00;

// Translated from 00bacf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 2. No arguments. With the thread's allocation context
/// (+0x2b4 of the thread block) at 0x1c (put back at the end): makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 5 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0)), calls slot +0x94 with (0, 4, 6, 0x80000000,
/// 0, 5, 1, 0) and slot +0xb4. Then allocates 0x1e0 bytes and runs the constructor `00bacd00`
/// with the declaration (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x11c, +0x140, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bacf00 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn fn_00bacf00(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_TALL_GRASS);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 5u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 3, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 4u32, 6u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x1e0u32]).u32();
    let shader = if block != 0 {
        e.call(
            TALL_GRASS_SHADER_CONSTRUCT_00BACD00,
            &args![block, declaration],
        )
        .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x11c, 0x140] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The tall grass shader's vtable (`010b8980`; no RTTI locator before it).
const TALL_GRASS_VTABLE_010B8980: u32 = 0x010b_8980;
/// The four words copied (`011a9bd0`..`011a9bdc` into `01200570`..`0120057c`).
const WORDS_011A9BD0: u32 = 0x011a_9bd0;
const WORDS_01200570: u32 = 0x0120_0570;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bacd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the tall grass shader's constructor,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010b8980`, constructs the `NiPointer` arrays at +0x6c (0x2c, destructor
/// `00b79930`), +0x128 (0x18) and +0x188 (7, both destructor `00b66f20`), stores 0.0 at
/// +0x1b0 to +0x1dc, sets +0x68 to 6, assigns the declaration to the `NiPointer` at +0x28,
/// stores 0.0 at +0x11c, copies the four words at `011a9bd0` to `01200570`, sets the 16-bit
/// count at +0x1ac to 0xe4 (0x50 when the setting at `011f91c0` is below 2, signed), allocates
/// that many 0x10-byte elements from the memory manager into +0x1a4 and clears +0x1a8.
/// Answers `this`.
// Translation incomplete: 00bacd00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn fn_00bacd00(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, TALL_GRASS_VTABLE_010B8980);
    construct_pointer_array(e, base + 0x6c, 0x2c, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x128, 0x18, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x188, 7, DESTRUCT_00B66F20);
    for offset in (0x1b0u32..=0x1dc).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 6);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    e.mem.set_f32(base + 0x11c, 0.0);
    for index in 0..4u32 {
        let word = e.global::<u32>(WORDS_011A9BD0 + 4 * index);
        e.set_global::<u32>(WORDS_01200570 + 4 * index, word);
    }
    let count: u16 = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
        0x50
    } else {
        0xe4
    };
    e.mem.set_u16(base + 0x1ac, count);
    let count = e.mem.u16(base + 0x1ac);
    let elements = allocate_array(e, u32::from(count), 0x10);
    e.mem.set_u32(base + 0x1a4, elements);
    e.mem.set_u32(base + 0x1a8, 0);
    this
}

/// The shader manager's pass state (`0126f74c`): its word +0x24 is a table of the constant
/// blocks the passes fill (+0, +4, +8, +0xc are read), +0x44 and +0x5c two more objects.
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
/// The two words `00bac8d0` copies into the constant blocks (`011f95e4`, `011f95e8`).
const CONSTANTS_011F95E4: u32 = 0x011f_95e4;
const CONSTANTS_011F95E8: u32 = 0x011f_95e8;
/// The current pass number (`011f91e4`) and the word holding the current pass record
/// (`011f91e0`; the record's +0 is the geometry being drawn).
const CURRENT_PASS_011F91E4: u32 = 0x011f_91e4;
const CURRENT_RECORD_011F91E0: u32 = 0x011f_91e0;
/// `BSRenderedTexture::GetTexture` (engine map, `004bc320`, `thiscall`, one word).
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, one word), the
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor), `BSShader::~BSShader` (engine map, `00be0f10`, no stack words) and the Ni
/// free (`00aa1460`, `cdecl`: block, size).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
/// CRT `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// Bits of `1.0f`.
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00baa570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the tall grass shader's
/// destructor, no stack words. Stores the vtable `010b8980`, clears +0x1a8, gives the
/// instance array (+0x1a4) back to the memory manager (`00aa4060`), destroys the `NiPointer`
/// arrays at +0x188 (7) and +0x128 (0x18) (destructor `00b66f20`) and +0x6c (0x2c, `00b79930`)
/// (`eh vector destructor iterator`) and runs `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00baa570 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the arrays and the base) is not translated.
pub fn fn_00baa570(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, TALL_GRASS_VTABLE_010B8980);
    let instances = e.mem.u32(base + 0x1a4);
    e.mem.set_u32(base + 0x1a8, 0);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, instances]);
    for (offset, count, destructor) in [
        (0x188u32, 7u32, DESTRUCT_00B66F20),
        (0x128, 0x18, DESTRUCT_00B66F20),
        (0x6c, 0x2c, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The size the scalar deleting destructor frees.
const TALL_GRASS_SIZE: u32 = 0x1e0;

// Translated from 00baced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the scalar deleting destructor,
/// `thiscall`, `RET 4`. Runs the destructor (`00baa570`) and, when bit 0 of the flags is set,
/// frees the 0x1e0 bytes (`00aa1460`). Answers `this`.
pub fn fn_00baced0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00baa570(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, TALL_GRASS_SIZE]);
    }
    this
}

// Translated from 00baaf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): copies the instance data into the
/// shader. `thiscall`, `RET 8`; the first word is not read, the second is the object whose
/// +0x7c holds the instance data. From that data: copies its `u16` count (+0x10) of 0x10-byte
/// elements from its array (+0x14) into the shader's instance array (+0x1a4) (`memcpy`); runs
/// the vtable slot +0xa4 of the object +0xb8 of the data's first word with a 16-bit count:
/// that object's `u16` +0x40 when the data's `u16` +0x12 equals the count, otherwise the
/// 16-bit product (`IMUL`) of the `u16` +0x34 of the data's object +8 and the `u16` +0x12;
/// and stores the `u16` +0x12 in the word +0x20 of the instance entry (+0x1a8) when there is
/// one.
pub fn fn_00baaf40(e: &mut Engine, this: Ptr, _unused_1: u32, holder: Ptr) {
    let base = this.addr();
    let data = e.mem.u32(holder.addr() + 0x7c);
    let source = e.mem.u32(data + 0x14);
    let count = e.mem.u16(data + 0x10);
    let target = e.mem.u32(base + 0x1a4);
    e.call(MEMCPY, &args![target, source, u32::from(count) << 4]);
    let active = e.mem.u16(data + 0x12);
    let object = e.mem.u32(e.mem.u32(data) + 0xb8);
    let vertices = if active == count {
        e.mem.u16(object + 0x40)
    } else {
        let per_instance = e.mem.u16(e.mem.u32(data + 8) + 0x34);
        per_instance.wrapping_mul(active)
    };
    e.vcall(object, 0xa4, &args![u32::from(vertices)]);
    let entry = e.mem.u32(base + 0x1a8);
    if entry != 0 {
        let active = e.mem.u16(data + 0x12);
        e.mem.set_u32(entry + 0x20, u32::from(active));
    }
}

/// D3DX (engine map): `D3DXMatrixInverse` (`00ee6dda`, `stdcall`: out, determinant,
/// matrix), `D3DXVec3TransformNormal` (`00ee6df2`, `stdcall`: out, vector, matrix) and
/// `D3DXVec3Normalize` (`00ee6dec`, `stdcall`: out, vector).
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
const D3DX_VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const D3DX_VEC3_NORMALIZE: u32 = 0x00ee_6dec;
/// The renderer's `00e6fbb0` (`thiscall` on the shader's +0x18, `RET 8`: the transform and 0).
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, no arguments),
/// `BSShaderAccumulator::IsShadowPass` (`00b630f0`, `cdecl`, the pass),
/// `BSShaderLightingProperty::GetNumberofActiveNonShadowLights` (`00b707d0`, `thiscall`, a
/// 16-bit answer) and `TallGrassShader::UpdatePointLights` (`00baa720`, `thiscall`, `RET
/// 0x10`) (engine map), the unnamed `00baa850` (`thiscall`, `RET 8`) and `004a0c10`
/// (`thiscall` on a point), and the base `SetupGeometry` (`00e807d0`, unit `nid3dshader.cpp`,
/// `thiscall`, `RET 0x20`).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const IS_SHADOW_PASS: u32 = 0x00b6_30f0;
const ACTIVE_NON_SHADOW_LIGHTS: u32 = 0x00b7_07d0;
const UPDATE_POINT_LIGHTS: u32 = 0x00ba_a720;
const SHADOW_SETUP_00BAA850: u32 = 0x00ba_a850;
const POINT_CALL_004A0C10: u32 = 0x004a_0c10;
const BASE_SETUP_GEOMETRY_00E807D0: u32 = 0x00e8_07d0;
/// The globals `00baac10` writes: the transformed up vector (`01200550`, `01200554`), the
/// light direction, ambient and diffuse blocks (`01200510`, `01200520`, `01200570`, four
/// floats each) and the byte that turns the point lights on (`011f9ef0`).
const UP_01200550: u32 = 0x0120_0550;
const LIGHT_DIRECTION_01200510: u32 = 0x0120_0510;
const LIGHT_AMBIENT_01200520: u32 = 0x0120_0520;
const POINT_LIGHTS_ON_011F9EF0: u32 = 0x011f_9ef0;

// Translated from 00baac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b8980` slot +0x60): the tall grass `SetupGeometry`,
/// `thiscall`, `RET 0x20` (eight words: the geometry, three words, the property state, a
/// word, the world transform and the bound, as `00bac9c0` passes them). Runs the renderer's
/// `00e6fbb0` (+0x18) with the transform and 0; builds on the stack the D3DX matrix of the
/// transform (rotation transposed and times the scale, translation, 1.0), inverts it
/// (`D3DXMatrixInverse`), transforms (0, 1, 0) by the inverse (`D3DXVec3TransformNormal`),
/// normalizes it (`D3DXVec3Normalize`) and stores its x and y at `01200550`. When the state's
/// word +0xc is an object whose word +0x1c is 4: from the accumulator's (`GetAccumulator`,
/// +0x194, +0xe0) light at +0xf8, when there is one, its direction (+0xf0, through `004a0c10`
/// on a copy) negated with 1.0 into `01200510`, its +0xd4 colour with 1.0 into `01200520` and
/// its +0xc8 colour with 1.0 into `01200570` (each float through `FLD`/`FSTP`); then on a
/// shadow pass (`IsShadowPass` of the pass number `011f91e4` read at the start) `00baa850`
/// with the second word and the transform, otherwise, with the byte `011f9ef0` set and active
/// non-shadow lights on that object, `UpdatePointLights` with it, the inverse matrix, the
/// transform's scale and the pass number. Answers the base `SetupGeometry` (`00e807d0`) with
/// the eight words.
pub fn fn_00baac10(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    state: u32,
    word_6: u32,
    transform: u32,
    bound: u32,
) -> u32 {
    let this = this.addr();
    let renderer = e.mem.u32(this + 0x18);
    e.call(RENDERER_CALL_00E6FBB0, &args![renderer, transform, 0u32]);
    e.with_stack(0xd0, |e, frame| {
        let frame = frame.addr();
        let matrix = frame + 0x44;
        let inverse = frame + 0x90;
        let up = frame + 0x34;
        let transformed = frame + 0x84;
        let scale_bits = e.mem.u32(transform + 0x30);
        // FLD scale, FMUL [rotation 0]; then each FLD rotation, FMUL scale.
        let first = x87_mul(x87_load32(scale_bits), x87_m32(e.mem.u32(transform)));
        e.mem.set_u32(matrix, x87_store32(first));
        let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
        e.mem.set_u32(frame + 0x40, pass);
        for (row, place) in [
            (3u32, 0x48u32),
            (6, 0x4c),
            (1, 0x54),
            (4, 0x58),
            (7, 0x5c),
            (2, 0x64),
            (5, 0x68),
            (8, 0x6c),
        ] {
            let value = x87_load32(e.mem.u32(transform + 4 * row));
            let product = x87_store32(x87_mul(value, x87_m32(scale_bits)));
            e.mem.set_u32(frame + place, product);
        }
        for (index, place) in [0x74u32, 0x78, 0x7c].iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(transform + 0x24 + 4 * index as u32));
            e.mem.set_u32(frame + place, value);
        }
        for place in [0x50u32, 0x60, 0x70] {
            e.mem.set_u32(frame + place, 0);
        }
        e.mem.set_u32(frame + 0x80, ONE_BITS);
        e.call(D3DX_MATRIX_INVERSE, &args![inverse, 0u32, matrix]);
        e.mem.set_u32(up, 0);
        e.mem.set_u32(up + 4, ONE_BITS);
        e.mem.set_u32(up + 8, 0);
        e.call(D3DX_VEC3_TRANSFORM_NORMAL, &args![transformed, up, inverse]);
        e.call(D3DX_VEC3_NORMALIZE, &args![up, transformed]);
        let x = x87_quiet_bits(e.mem.u32(up));
        e.set_global::<u32>(UP_01200550, x);
        let y = x87_quiet_bits(e.mem.u32(up + 4));
        e.set_global::<u32>(UP_01200550 + 4, y);

        let property = e.mem.u32(state + 0xc);
        if property == 0 || e.mem.u32(property + 0x1c) != 4 {
            return;
        }
        let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
        let lights = e.mem.u32(e.mem.u32(accumulator + 0x194) + 0xe0);
        let light = e.mem.u32(lights + 0xf8);
        if light != 0 {
            let direction = frame + 0x28;
            let z = e.mem.u32(light + 0xf8);
            e.mem.set_u32(direction + 8, z);
            let x = e.mem.u32(light + 0xf0);
            e.mem.set_u32(direction, x);
            let y = e.mem.u32(light + 0xf4);
            e.mem.set_u32(direction + 4, y);
            e.call(POINT_CALL_004A0C10, &args![direction]);
            for (source, target) in [(0xc8u32, 0x0120_0570u32), (0xd4, LIGHT_AMBIENT_01200520)] {
                for word in 0..3u32 {
                    let value = x87_quiet_bits(e.mem.u32(light + source + 4 * word));
                    e.set_global::<u32>(target + 4 * word, value);
                }
                e.set_global::<u32>(target + 0xc, ONE_BITS);
            }
            // FLD, FCHS, FSTP: the sign flipped after the load quiets a signalling NaN.
            for word in 0..3u32 {
                let value = x87_quiet_bits(e.mem.u32(direction + 4 * word)) ^ 0x8000_0000;
                e.set_global::<u32>(LIGHT_DIRECTION_01200510 + 4 * word, value);
            }
            e.set_global::<u32>(LIGHT_DIRECTION_01200510 + 0xc, ONE_BITS);
        }
        let pass = e.mem.u32(frame + 0x40);
        if e.call(IS_SHADOW_PASS, &args![pass]).bool() {
            e.call(SHADOW_SETUP_00BAA850, &args![this, word_2, transform]);
        } else if e.global::<u8>(POINT_LIGHTS_ON_011F9EF0) != 0
            && e.call(ACTIVE_NON_SHADOW_LIGHTS, &args![property]).u16() != 0
        {
            let scale = x87_quiet_bits(e.mem.u32(transform + 0x30));
            e.call(
                UPDATE_POINT_LIGHTS,
                &args![this, property, inverse, scale, pass],
            );
        }
    });
    e.call(
        BASE_SETUP_GEOMETRY_00E807D0,
        &args![this, geometry, word_2, word_3, word_4, state, word_6, transform, bound],
    )
    .u32()
}

// Translated from 00bac8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b8980` slot +0xf8), `thiscall`, `RET 4` (a pass number;
/// `this` is not read). For passes 0x1c to 0x25 and 0x262 to 0x26b the words `011f95e4` and
/// `011f95e8` go to +8 of the pass state's (`0126f74c`, read once) constant blocks +4 and +8;
/// for passes 0x30 to 0x39 and 0x276 to 0x27f to +8 of the blocks +8 and +0xc.
pub fn fn_00bac8d0(e: &mut Engine, _this: Ptr, pass: u32) {
    let state = e.global::<u32>(PASS_STATE_0126F74C);
    let set = |e: &mut Engine, block: u32, source: u32| {
        let table = e.mem.u32(state + 0x24);
        let constant = e.mem.u32(table + block);
        let value = e.global::<u32>(source);
        e.mem.set_u32(constant + 8, value);
    };
    if (0x1c..=0x25).contains(&pass) || pass.wrapping_sub(0x262) <= 9 {
        set(e, 4, CONSTANTS_011F95E4);
        set(e, 8, CONSTANTS_011F95E8);
    }
    if (0x30..=0x39).contains(&pass) || pass.wrapping_sub(0x276) <= 9 {
        set(e, 8, CONSTANTS_011F95E4);
        set(e, 0xc, CONSTANTS_011F95E8);
    }
}

// Translated from 00bac950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b8980` slot +0x78), `thiscall`, `RET 4` (one word, an
/// object whose +0xc it reads). Stores the answer of the vtable slot +0xb4 of that +0xc
/// object in +8 of the pass state's (`0126f74c`) first constant block; for pass numbers
/// (`011f91e4`, read first) 0x284 to 0x287 also stores `BSRenderedTexture::GetTexture(0)`
/// (`004bc320`) on the word +0x10c of what the first word of the current record's
/// (`011f91e0`) +0xc object points to, in +8 of the second block. Then the shader's slot
/// +0x108.
pub fn fn_00bac950(e: &mut Engine, this: Ptr, source: Ptr) {
    let state = e.global::<u32>(PASS_STATE_0126F74C);
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    let object = e.mem.u32(source.addr() + 0xc);
    let table = e.mem.u32(state + 0x24);
    let constant = e.mem.u32(table);
    let value = e.vcall(object, 0xb4, &args![]).u32();
    e.mem.set_u32(constant + 8, value);
    if pass.wrapping_sub(0x284) <= 3 {
        let state = e.global::<u32>(PASS_STATE_0126F74C);
        let table = e.mem.u32(state + 0x24);
        let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
        let property = e.mem.u32(record + 0xc);
        let constant = e.mem.u32(table + 4);
        let pointed = e.mem.u32(property);
        let texture_holder = e.mem.u32(pointed + 0x10c);
        let texture = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture_holder, 0u32])
            .u32();
        e.mem.set_u32(constant + 8, texture);
    }
    e.vcall(this.addr(), 0x108, &args![]);
}

/// The globals `00bac9c0` reads and writes.
const FULL_BRIGHT_011F91A7: u32 = 0x011f_91a7;
const DIFFUSE_011FA0C0: u32 = 0x011f_a0c0;
const AMBIENT_011F9EE0: u32 = 0x011f_9ee0;
const AMBIENT_01200560: u32 = 0x0120_0560;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
/// `255.0` (`double`, `0101e568`), `3600.0` (`double`, `01012640`), the cycle length (`double`
/// `0104eea0`, about 6.28318) and the time it scales (`011f940c`).
const BYTE_SCALE_0101E568: u32 = 0x0101_e568;
const SECONDS_PER_HOUR_01012640: u32 = 0x0101_2640;
const CYCLE_0104EEA0: u32 = 0x0104_eea0;
const GAME_SECONDS_011F940C: u32 = 0x011f_940c;
/// `_CIfmod` (`00ec913a`: `ST1` modulo `ST0`, both doubles; answers in `ST0`).
const CI_FMOD: u32 = 0x00ec_913a;
const WIND_PHASE_0120055C: u32 = 0x0120_055c;
const FLAGS_01200530: u32 = 0x0120_0530;
const FOG_SWITCH_011F941E: u32 = 0x011f_941e;
const FOG_VALUE_011F9188: u32 = 0x011f_9188;
const FOG_VALUE_01200580: u32 = 0x0120_0580;
const SKIP_011F9427: u32 = 0x011f_9427;
const RENDER_FLAGS_011F91D8: u32 = 0x011f_91d8;
const SHADOW_FADE_011FD890: u32 = 0x011f_d890;
const SHADOW_WORDS_011FD874: u32 = 0x011f_d874;
const SHADOW_VALUE_011ADCE0: u32 = 0x011a_dce0;
const SHADOW_VALUE_011FD880: u32 = 0x011f_d880;
const SHADOW_01200590: u32 = 0x0120_0590;
/// The render-state setter `00b97f20` (unit `bsrenderstate_xenon.cpp`, `cdecl`, three words).
const SET_ALPHA_STATE_00B97F20: u32 = 0x00b9_7f20;

// Translated from 00bac9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`; vtable `010b8980` slot +0x7c),
/// `thiscall`, `RET 4` (the property state). With the current record's (`011f91e0`)
/// geometry: runs the shader's slot +0x60 (`SetupGeometry`) with the geometry, its word +0xbc,
/// 0, the word +0x34 of its +0xb8, the state, 0, its transform (+0x68) and its bound (+0x20,
/// or `011f4288` when null). Then, with the state's +0xc object (the code reads it back from
/// the argument's stack word, which it overwrote with it): with the byte `011f91a7` set, 1.0
/// into the four floats at `011fa0c0`; the four words `011f9ee0` copied to `01200560`; the
/// alpha (+0x1d0..+0x1dc) from the state's first object (its byte +0x1a divided by 255.0, then
/// three 0.0) or the four words `011a9bd0`; the phase (+0x11c) as the time `011f940c` over
/// 3600, times the cycle (`0104eea0`), times the `float` +0x34 of the object's +0x7c, taken
/// modulo the cycle (`_CIfmod`) when not below it, and copied to `0120055c`; 1.0 or 0.0 (flag
/// 0x800 of the object's +0x24) into `01200530`, `01200534` and 1.0 into `01200538`; the
/// instance copy (`00baaf40`); with the byte `011f941e` the float `011f9188` into `01200580`;
/// with `011f9427` clear, `011f95e4` set and bit 5 of `011f91d8`: at a shader model
/// (`011f91c0`) of 2 or more the words `011fd874`, `011fd878`, `011adce0`, `011fd880` into
/// `01200590` when the truncated `011fd890` is nonzero. `00b97f20` runs with bits 10 to 12 of
/// the first object's `u16` +0x18, its byte +0x1a and 0 unless the model is 2 or more with
/// `011f941e` set. Last, the slot +0x78 of the shader's +0x30 and +0x34 objects with the pass
/// state's (`0126f74c`) +0x44 and +0x5c objects, the eight words of the first call, 0 and 1.
pub fn fn_00bac9c0(e: &mut Engine, this: Ptr, state: Ptr) {
    let base = this.addr();
    let state = state.addr();
    let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
    let object = e.mem.u32(record);
    let mut bound = e.mem.u32(object + 0x20);
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    let passes = e.mem.u32(e.mem.u32(object + 0xb8) + 0x34);
    let word_bc = e.mem.u32(object + 0xbc);
    let source = e.mem.u32(state + 0xc);
    e.vcall(
        base,
        0x60,
        &args![
            object,
            word_bc,
            0u32,
            passes,
            state,
            0u32,
            object + 0x68,
            bound
        ],
    );
    if e.global::<u8>(FULL_BRIGHT_011F91A7) != 0 {
        for word in 0..4u32 {
            e.set_global::<u32>(DIFFUSE_011FA0C0 + 4 * word, ONE_BITS);
        }
    }
    for word in 0..4u32 {
        let value = e.global::<u32>(AMBIENT_011F9EE0 + 4 * word);
        e.set_global::<u32>(AMBIENT_01200560 + 4 * word, value);
    }
    let first = e.mem.u32(state);
    if first != 0 {
        let alpha = f64::from(e.mem.u8(first + 0x1a));
        let scale = f64::from_bits(e.global::<u64>(BYTE_SCALE_0101E568));
        let alpha = x87_store32(x87_div(alpha, scale));
        e.mem.set_u32(base + 0x1d0, alpha);
        for offset in [0x1d4u32, 0x1d8, 0x1dc] {
            e.mem.set_u32(base + offset, 0);
        }
    } else {
        for word in 0..4u32 {
            let value = e.global::<u32>(WORDS_011A9BD0 + 4 * word);
            e.mem.set_u32(base + 0x1d0 + 4 * word, value);
        }
    }
    let seconds = x87_load32(e.global::<u32>(GAME_SECONDS_011F940C));
    let hours = x87_div(
        seconds,
        f64::from_bits(e.global::<u64>(SECONDS_PER_HOUR_01012640)),
    );
    let cycle = x87_load64(e.global::<u64>(CYCLE_0104EEA0));
    // FMUL ST(1),ST(0) (`DC C9`): the hours times the cycle; then FMULP by the float.
    let scaled = x87_mul(hours, cycle);
    let data = e.mem.u32(source + 0x7c);
    let speed = x87_load32(e.mem.u32(data + 0x34));
    let phase = x87_load32(x87_store32(x87_mul(scaled, speed)));
    e.mem.set_u32(base + 0x11c, x87_store32(phase));
    // FCOM ST(1); TEST AH,1: below the cycle (or unordered) keeps the phase.
    let below = phase < cycle || phase.is_nan() || cycle.is_nan();
    if !below {
        let remainder = e.call(CI_FMOD, &args![phase, cycle]).f64();
        let remainder = x87_load32(x87_store32(remainder));
        e.mem.set_u32(base + 0x11c, x87_store32(remainder));
    }
    let phase = x87_quiet_bits(e.mem.u32(base + 0x11c));
    e.set_global::<u32>(WIND_PHASE_0120055C, phase);
    let flag = if e.mem.u32(source + 0x24) & 0x800 != 0 {
        ONE_BITS
    } else {
        0
    };
    e.set_global::<u32>(FLAGS_01200530, flag);
    e.set_global::<u32>(FLAGS_01200530 + 4, flag);
    e.set_global::<u32>(FLAGS_01200530 + 8, ONE_BITS);
    fn_00baaf40(e, this, object, Ptr::new(source));
    let fog = e.global::<u8>(FOG_SWITCH_011F941E) != 0;
    if fog {
        let value = x87_quiet_bits(e.global::<u32>(FOG_VALUE_011F9188));
        e.set_global::<u32>(FOG_VALUE_01200580, value);
    }
    let model = e.global::<i32>(SHADER_MODEL_011F91C0);
    let shadows = e.global::<u8>(SKIP_011F9427) == 0
        && e.global::<u32>(CONSTANTS_011F95E4) != 0
        && e.global::<u8>(RENDER_FLAGS_011F91D8) & 0x20 != 0;
    if shadows && model >= 2 {
        let fade = X87Ext::load32(e.global::<u32>(SHADOW_FADE_011FD890));
        if x87_fistp64_chop(fade) as u32 != 0 {
            let sources = [
                SHADOW_WORDS_011FD874,
                SHADOW_WORDS_011FD874 + 4,
                SHADOW_VALUE_011ADCE0,
                SHADOW_VALUE_011FD880,
            ];
            for (word, source) in sources.iter().enumerate() {
                let value = x87_quiet_bits(e.global::<u32>(*source));
                e.set_global::<u32>(SHADOW_01200590 + 4 * word as u32, value);
            }
        }
    }
    if model < 2 || !fog {
        let first = e.mem.u32(state);
        let flags = u32::from(e.mem.u16(first + 0x18));
        let alpha = u32::from(e.mem.u8(first + 0x1a));
        e.call(
            SET_ALPHA_STATE_00B97F20,
            &args![(flags >> 10) & 7, alpha, 0u32],
        );
    }
    for (slot, entry) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let target = e.mem.u32(base + slot);
        if target == 0 {
            continue;
        }
        let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
        let constants = e.mem.u32(pass_state + entry);
        if constants == 0 {
            continue;
        }
        e.vcall(
            target,
            0x78,
            &args![
                constants,
                object,
                word_bc,
                0u32,
                passes,
                state,
                0u32,
                object + 0x68,
                bound,
                0u32,
                1u32
            ],
        );
    }
}

/// `00bac160` (unit `tallgrassshader.cpp`, no stack words): fills the passes.
const STAGES_AND_PASSES_00BAC160: u32 = 0x00ba_c160;

// Translated from 00bad060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`; vtable `010b8980` slot +0x140),
/// no stack words. With the thread's allocation context at 0x1c (put back at the end): gives
/// each of the 0x2c pass pointers at +0x6c a new pass (`00e804d0`), runs `00bac160` and
/// answers true.
// Translation incomplete: 00bad060 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_00bad060(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    with_allocation_context(e, CONTEXT_TALL_GRASS, |e| {
        for index in 0..0x2cu32 {
            e.call(NEW_PASS_00E804D0, &args![base + 0x6c + 4 * index]);
        }
        e.call(STAGES_AND_PASSES_00BAC160, &args![base]);
    });
    true
}

/// The constant names of `CreateConstantMaps`.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const SHADOW_PROJ_TRANSFORM_010AF648: u32 = 0x010a_f648;
const FOG_PARAM_010B8C54: u32 = 0x010b_8c54;
const FOG_COLOR_010B8C48: u32 = 0x010b_8c48;
const GROUPED_CONSTANTS_010B8C34: u32 = 0x010b_8c34;
const INSTANCE_DATA_010B8C24: u32 = 0x010b_8c24;
const POINT_LIGHT_COLOR_010B8C10: u32 = 0x010b_8c10;
const POINT_LIGHT_DATA_010B8BFC: u32 = 0x010b_8bfc;
const ALPHA_TEST_REF_010B8BEC: u32 = 0x010b_8bec;
/// The constant data the entries point at.
const SHADOW_PROJECTION_011FD884: u32 = 0x011f_d884;
const FOG_PARAM_011FA280: u32 = 0x011f_a280;
const FOG_COLOR_011FA290: u32 = 0x011f_a290;

/// One `0x10000007` entry: one register of 0x10 bytes at `data`.
fn vector_entry(name: u32, register: u32, data: u32) -> [u32; 10] {
    [
        name,
        0x1000_0007,
        0,
        register,
        1,
        CONSTANT_EMPTY_NAME_01011584,
        0x10,
        4,
        data,
        0,
    ]
}

// Translated from 00bad100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TallGrassShader::CreateConstantMaps` (Xbox PDB), no stack words. With the thread's
/// allocation context at 0x1c (put back at the end): when the vertex constant map (+0x34) is
/// null, makes one (kind 0) and adds `"WorldViewProjTranspose"` (0x20000009, 0, 9, 4),
/// `"ShadowProjTransform"` (`011fd884`, register 0xd), `"fogparam"` (`011fa280`, 0xf),
/// `"fogcolor"` (`011fa290`, 0xe), `"grouped constants"` (`01200510`, 9 registers, 0x90
/// bytes) and `"instance data"` (register 0x14, the `u16` count +0x1ac, 0x10 bytes each, the
/// array +0x1a4), and keeps the map's slot +0x70 answer for `"instance data"` in +0x1a8; when
/// the pixel constant map (+0x30) is null, makes one (kind 2) and adds `"point light color"`
/// (+0x1b0, register 2), `"point light data"` (+0x1c0, 4) and `"alpha test ref"` (+0x1d0, 3).
// Translation incomplete: 00bad100 - the C++ exception frame (FS:[0] chain, unwind states 1 and 2 for the allocations) is not translated.
pub fn tall_grass_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let empty = CONSTANT_EMPTY_NAME_01011584;
    with_allocation_context(e, CONTEXT_TALL_GRASS, |e| {
        if e.mem.u32(base + 0x34) == 0 {
            make_constant_map(e, base, 0x34, 0);
            let entries = [
                [
                    WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                    0x2000_0009,
                    0,
                    9,
                    4,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                vector_entry(
                    SHADOW_PROJ_TRANSFORM_010AF648,
                    0xd,
                    SHADOW_PROJECTION_011FD884,
                ),
                vector_entry(FOG_PARAM_010B8C54, 0xf, FOG_PARAM_011FA280),
                vector_entry(FOG_COLOR_010B8C48, 0xe, FOG_COLOR_011FA290),
                [
                    GROUPED_CONSTANTS_010B8C34,
                    0x1000_0009,
                    0,
                    0,
                    9,
                    empty,
                    0x90,
                    4,
                    LIGHT_DIRECTION_01200510,
                    0,
                ],
            ];
            for words in entries {
                add_constant_entry(e, base, 0x34, words);
            }
            let count = u32::from(e.mem.u16(base + 0x1ac));
            let instances = e.mem.u32(base + 0x1a4);
            add_constant_entry(
                e,
                base,
                0x34,
                [
                    INSTANCE_DATA_010B8C24,
                    0x1000_0009,
                    0,
                    0x14,
                    count,
                    empty,
                    count << 4,
                    4,
                    instances,
                    0,
                ],
            );
            let map = e.mem.u32(base + 0x34);
            let entry = e.vcall(map, 0x70, &args![INSTANCE_DATA_010B8C24]).u32();
            e.mem.set_u32(base + 0x1a8, entry);
        }
        if e.mem.u32(base + 0x30) == 0 {
            make_constant_map(e, base, 0x30, 2);
            for (name, register, offset) in [
                (POINT_LIGHT_COLOR_010B8C10, 2u32, 0x1b0u32),
                (POINT_LIGHT_DATA_010B8BFC, 4, 0x1c0),
                (ALPHA_TEST_REF_010B8BEC, 3, 0x1d0),
            ] {
                add_constant_entry(e, base, 0x30, vector_entry(name, register, base + offset));
            }
        }
    });
}

/// `004dc060` (unnamed, engine map unit `renderer.cpp`, `cdecl`, one word), the multisampling
/// word `011f9421` (a byte), the texture word `011f9534`, the shadow filter result `011f948c`
/// and the value of render state 0xc3 (`float`, `01200458`).
const RENDERER_QUERY_004DC060: u32 = 0x004d_c060;
const TRANSPARENCY_MULTISAMPLING_011F9421: u32 = 0x011f_9421;
const TEXTURE_011F9534: u32 = 0x011f_9534;
const SHADOW_FILTER_RESULT_011F948C: u32 = 0x011f_948c;
const STATE_VALUE_01200458: u32 = 0x0120_0458;

/// A stage made into the function's one stage pointer (`CreateNewStage` with its address)
/// and set up (`SetupDefaultStage` with the index and three words). Answers the stage.
fn new_default_stage(e: &mut Engine, holder: u32, index: u32, setup: [u32; 3]) -> u32 {
    e.call(CREATE_NEW_STAGE, &args![holder]);
    let stage = e.mem.u32(holder);
    e.call(
        SETUP_DEFAULT_STAGE,
        &args![stage, index, setup[0], setup[1], setup[2]],
    );
    stage
}

/// `NiD3DPass::SetStage` on the pass with its word +0x14 and the stage.
fn set_pass_stage(e: &mut Engine, pass: u32, stage: u32) {
    let position = e.mem.u32(pass + 0x14);
    e.call(PASS_SET_STAGE, &args![pass, position, stage]);
}

/// The fog states of the first two pass groups, set when `011f941e` is on and `011f9421`
/// off: (0x1b, 1), (0x13, 5), (0x14, 6).
fn fog_states(e: &mut Engine, pass: u32) {
    if e.global::<u8>(FOG_SWITCH_011F941E) == 0
        || e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421) != 0
    {
        return;
    }
    pass_render_state(e, pass, 0x1b, 1);
    pass_render_state(e, pass, 0x13, 5);
    pass_render_state(e, pass, 0x14, 6);
}

/// The pass's vertex shader (+0x5c) and pixel shader (+0x44) `NiPointer`s set.
fn pass_shaders(e: &mut Engine, pass: u32, vertex: u32, pixel: u32) {
    ni_pointer_assign(e, pass + 0x5c, vertex);
    ni_pointer_assign(e, pass + 0x44, pixel);
}

// Translated from 00bac160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`), `thiscall`, no stack words: the
/// passes of the tall grass shader, run by `00bad060`. With the thread's allocation context
/// at 0x1c (put back at the end) and one stage pointer kept for the whole function (dropped
/// at the end):
/// - passes 0 to 0x13 (+0x6c): a pass without stages (+0x18) gets stage 0 (3, 2, 0); passes
///   8 to 0xf with fewer than 3 stages get stages 1 and 2 (1, 2, 0), and those passes use
///   pixel shader 2, the others pixel shader 0. State (0x1b, 0); when `004dc060(0)` answers
///   2 or more, (0xf, 1), (0x19, 5), (0x18, 0xa) and the fog states. Vertex shader +0x128 +
///   4 * index.
/// - passes 0x14 to 0x27 (+0xbc): with fewer than 2 stages, stage 0 (3, 2, 0) and stage 1
///   (3, 2, 0) whose words +8 and +0xc become `011f9534` and 1; passes 0x1c to 0x23 with
///   fewer than 4 stages get stages 2 and 3 (1, 2, 0), and those passes use pixel shader 3,
///   the others pixel shader 1. When `004dc060(0)` answers below 2, (0xf, 0); otherwise
///   (0xf, 1), (0x19, 5), (0x18, 0xa) and the fog states. Vertex shaders again from +0x128.
/// - passes 0x28 to 0x2b (+0x10c): with fewer than 2 stages, stages 0 and 1 (3, 2, 0); states
///   (0x1b, 1), (0x13, 9), (0x14, 1), (0xe, 0), (0xc3, the `float` `01200458`), (0x98,
///   0x3f); vertex shader +0x178 + 4 * index, pixel shader +0x198 + 4 * `011f948c`.
///
/// Pixel shaders of the first two groups are at +0x188 + 4 * the number chosen; the fog
/// states are (0x1b, 1), (0x13, 5), (0x14, 6), set when `011f941e` is on and `011f9421` off.
// Translation incomplete: 00bac160 - the C++ exception frame (FS:[0] chain, unwind state 1) is not translated.
pub fn fn_00bac160(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    with_allocation_context(e, CONTEXT_TALL_GRASS, |e| {
        e.with_stack(4, |e, holder| {
            let holder = holder.addr();
            e.mem.set_u32(holder, 0);
            for index in 0..0x14u32 {
                let pass = e.mem.u32(base + 0x6c + 4 * index);
                if e.mem.u32(pass + 0x18) < 1 {
                    let stage = new_default_stage(e, holder, 0, [3, 2, 0]);
                    set_pass_stage(e, pass, stage);
                }
                let pixel = if (8..=0xf).contains(&index) {
                    if e.mem.u32(pass + 0x18) < 3 {
                        let stage = new_default_stage(e, holder, 1, [1, 2, 0]);
                        set_pass_stage(e, pass, stage);
                        let stage = new_default_stage(e, holder, 2, [1, 2, 0]);
                        set_pass_stage(e, pass, stage);
                    }
                    2
                } else {
                    0
                };
                pass_render_state(e, pass, 0x1b, 0);
                if e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32() as i32 >= 2 {
                    pass_render_state(e, pass, 0xf, 1);
                    pass_render_state(e, pass, 0x19, 5);
                    pass_render_state(e, pass, 0x18, 0xa);
                    fog_states(e, pass);
                }
                let vertex = e.mem.u32(base + 0x128 + 4 * index);
                let pixel = e.mem.u32(base + 0x188 + 4 * pixel);
                pass_shaders(e, pass, vertex, pixel);
            }
            for index in 0x14..0x28u32 {
                let pass = e.mem.u32(base + 0x6c + 4 * index);
                if e.mem.u32(pass + 0x18) < 2 {
                    let stage = new_default_stage(e, holder, 0, [3, 2, 0]);
                    set_pass_stage(e, pass, stage);
                    let stage = new_default_stage(e, holder, 1, [3, 2, 0]);
                    let texture = e.global::<u32>(TEXTURE_011F9534);
                    e.mem.set_u32(stage + 8, texture);
                    e.mem.set_u32(stage + 0xc, 1);
                    set_pass_stage(e, pass, stage);
                }
                let pixel = if (0x1c..=0x23).contains(&index) {
                    if e.mem.u32(pass + 0x18) < 4 {
                        let stage = new_default_stage(e, holder, 2, [1, 2, 0]);
                        set_pass_stage(e, pass, stage);
                        let stage = new_default_stage(e, holder, 3, [1, 2, 0]);
                        set_pass_stage(e, pass, stage);
                    }
                    3
                } else {
                    1
                };
                if (e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32() as i32) < 2 {
                    pass_render_state(e, pass, 0xf, 0);
                } else {
                    pass_render_state(e, pass, 0xf, 1);
                    pass_render_state(e, pass, 0x19, 5);
                    pass_render_state(e, pass, 0x18, 0xa);
                    fog_states(e, pass);
                }
                let vertex = e.mem.u32(base + 0x128 + 4 * (index - 0x14));
                let pixel = e.mem.u32(base + 0x188 + 4 * pixel);
                pass_shaders(e, pass, vertex, pixel);
            }
            for index in 0..4u32 {
                let pass = e.mem.u32(base + 0x10c + 4 * index);
                if e.mem.u32(pass + 0x18) < 2 {
                    let stage = new_default_stage(e, holder, 0, [3, 2, 0]);
                    set_pass_stage(e, pass, stage);
                    let stage = new_default_stage(e, holder, 1, [3, 2, 0]);
                    set_pass_stage(e, pass, stage);
                }
                pass_render_state(e, pass, 0x1b, 1);
                pass_render_state(e, pass, 0x13, 9);
                pass_render_state(e, pass, 0x14, 1);
                pass_render_state(e, pass, 0xe, 0);
                let value = x87_quiet_bits(e.global::<u32>(STATE_VALUE_01200458));
                pass_render_state(e, pass, 0xc3, value);
                pass_render_state(e, pass, 0x98, 0x3f);
                let vertex = e.mem.u32(base + 0x178 + 4 * index);
                let filter = e.global::<u32>(SHADOW_FILTER_RESULT_011F948C);
                let pixel = e.mem.u32(
                    base.wrapping_add(0x198)
                        .wrapping_add(filter.wrapping_mul(4)),
                );
                pass_shaders(e, pass, vertex, pixel);
            }
            let stage = e.mem.u32(holder);
            stage_release(e, stage);
        });
    });
}

/// Strings of `TallGrassShader::LoadShaders` (read in Ghidra): the files
/// `tallgrass\\1x\\v\\lowDetail.v.hlsl`, `tallgrass\\1x\\p\\highDetail.p.hlsl` and
/// `highDetail_1pt.p.hlsl`, the macros `VS_2_0`, `PS_2_0`, `VERTLIT`, `FIT_TO_SLOPE`,
/// `PTLIGHT`, `PROJ_SHADOW`, `BILLBOARD`, `SHADOWMAP`, `DEPTHBIAS`, `SAMPLE`, `PASSES` and
/// `TRANSPARENCYMS`, the values `"-0.7"`, `"1"`, `"2"`, `"4"` and the empty string.
const LOW_DETAIL_VERTEX_010B8BCC: u32 = 0x010b_8bcc;
const HIGH_DETAIL_PIXEL_010B8B7C: u32 = 0x010b_8b7c;
const HIGH_DETAIL_1PT_PIXEL_010B8B4C: u32 = 0x010b_8b4c;
const GRASS_MACRO_VS_2_0_010B8BC4: u32 = 0x010b_8bc4;
const GRASS_MACRO_PS_2_0_010B8B74: u32 = 0x010b_8b74;
const GRASS_MACRO_VERTLIT_010B8BBC: u32 = 0x010b_8bbc;
const GRASS_MACRO_FIT_TO_SLOPE_010B8BAC: u32 = 0x010b_8bac;
const GRASS_MACRO_PTLIGHT_010AEC18: u32 = 0x010a_ec18;
const GRASS_MACRO_PROJ_SHADOW_010AEBA0: u32 = 0x010a_eba0;
const GRASS_MACRO_BILLBOARD_010B8BA0: u32 = 0x010b_8ba0;
const GRASS_MACRO_SHADOWMAP_010AE95C: u32 = 0x010a_e95c;
const GRASS_MACRO_DEPTHBIAS_010AEE24: u32 = 0x010a_ee24;
const GRASS_MACRO_SAMPLE_010AEE54: u32 = 0x010a_ee54;
const GRASS_MACRO_PASSES_010AEE14: u32 = 0x010a_ee14;
const GRASS_MACRO_TRANSPARENCYMS_010B8AFC: u32 = 0x010b_8afc;
const GRASS_VALUE_DEPTH_BIAS_010B8B44: u32 = 0x010b_8b44;
const GRASS_VALUE_1_01063F48: u32 = 0x0106_3f48;
const GRASS_VALUE_2_01063F3C: u32 = 0x0106_3f3c;
const GRASS_VALUE_4_01063F24: u32 = 0x0106_3f24;
const GRASS_EMPTY_01011584: u32 = 0x0101_1584;
/// The name formats `"GRASS2%03i.vso"`, `"GRASS23x%03i.vso"`, `"GRASS2%03i.pso"`,
/// `"GRASS2%03iTMS.pso"`, `"GRASS23x%03i.pso"`, `"GRASS23x%03iTMS.pso"` and the profiles
/// `vs_2_0`, `vs_3_0` and `ps_3_0`.
const FORMAT_GRASS2_VERTEX_010B8B34: u32 = 0x010b_8b34;
const FORMAT_GRASS23X_VERTEX_010B8B20: u32 = 0x010b_8b20;
const FORMAT_GRASS2_PIXEL_010B8AEC: u32 = 0x010b_8aec;
const FORMAT_GRASS2_TMS_PIXEL_010B8B0C: u32 = 0x010b_8b0c;
const FORMAT_GRASS23X_PIXEL_010B8AC4: u32 = 0x010b_8ac4;
const FORMAT_GRASS23X_TMS_PIXEL_010B8AD8: u32 = 0x010b_8ad8;
const GRASS_PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
const GRASS_PROFILE_VS_3_0_010A93DC: u32 = 0x010a_93dc;
const GRASS_PROFILE_PS_3_0_010A93B4: u32 = 0x010a_93b4;
/// The pixel profile chooser `00b4f380` (`cdecl`, a byte), the byte `011f948b`, the shader
/// 3.0 support byte `011f94a4` and the grass shader 3.0 byte `011f948a`.
const GRASS_PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
const SWITCH_011F948B: u32 = 0x011f_948b;
const SHADER_30_SUPPORT_011F94A4: u32 = 0x011f_94a4;
const GRASS_VS_30_011F948A: u32 = 0x011f_948a;
/// The `LoadShaders` frame (saved registers up to the cookie) and its name and path buffers.
const GRASS_FRAME_BYTES: u32 = 0xb74;
const GRASS_NAME: u32 = 0x968;
const GRASS_PATH: u32 = 0xa6c;
/// The source blocks (0x4c bytes apart: seven pixel blocks from +0x34, twenty-four vertex
/// blocks from +0x248), in the exe's order: the vertex blocks first.
const GRASS_SOURCES: [ShaderSource; 31] = [
    ShaderSource {
        offset: 0x248,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x294,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2e0,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x32c,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x378,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3c4,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x410,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x45c,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x4a8,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x4f4,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x540,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x58c,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x5d8,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x624,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x670,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x6bc,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_VERTLIT_010B8BBC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x708,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x754,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x7a0,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x7ec,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_PTLIGHT_010AEC18,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x838,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x884,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x8d0,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x91c,
        words: &[
            LOW_DETAIL_VERTEX_010B8BCC,
            GRASS_MACRO_VS_2_0_010B8BC4,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_BILLBOARD_010B8BA0,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_FIT_TO_SLOPE_010B8BAC,
            GRASS_EMPTY_01011584,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x34,
        words: &[
            HIGH_DETAIL_PIXEL_010B8B7C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x80,
        words: &[
            HIGH_DETAIL_1PT_PIXEL_010B8B4C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xcc,
        words: &[
            HIGH_DETAIL_PIXEL_010B8B7C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x118,
        words: &[
            HIGH_DETAIL_1PT_PIXEL_010B8B4C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            GRASS_MACRO_PROJ_SHADOW_010AEBA0,
            GRASS_EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x164,
        words: &[
            HIGH_DETAIL_PIXEL_010B8B7C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_DEPTHBIAS_010AEE24,
            GRASS_VALUE_DEPTH_BIAS_010B8B44,
            GRASS_MACRO_SAMPLE_010AEE54,
            GRASS_VALUE_1_01063F48,
            GRASS_MACRO_PASSES_010AEE14,
            GRASS_VALUE_1_01063F48,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x1b0,
        words: &[
            HIGH_DETAIL_PIXEL_010B8B7C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_DEPTHBIAS_010AEE24,
            GRASS_VALUE_DEPTH_BIAS_010B8B44,
            GRASS_MACRO_SAMPLE_010AEE54,
            GRASS_VALUE_4_01063F24,
            GRASS_MACRO_PASSES_010AEE14,
            GRASS_VALUE_1_01063F48,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    ShaderSource {
        offset: 0x1fc,
        words: &[
            HIGH_DETAIL_PIXEL_010B8B7C,
            GRASS_MACRO_PS_2_0_010B8B74,
            0,
            GRASS_MACRO_SHADOWMAP_010AE95C,
            GRASS_EMPTY_01011584,
            GRASS_MACRO_DEPTHBIAS_010AEE24,
            GRASS_VALUE_DEPTH_BIAS_010B8B44,
            GRASS_MACRO_SAMPLE_010AEE54,
            GRASS_VALUE_4_01063F24,
            GRASS_MACRO_PASSES_010AEE14,
            GRASS_VALUE_2_01063F3C,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
];

/// Whether shader 3.0 is supported (`011f94a4`) and the grass 3.0 byte (`011f948a`) is set.
fn grass_shader_30(e: &Engine) -> bool {
    e.global::<u8>(SHADER_30_SUPPORT_011F94A4) != 0 && e.global::<u8>(GRASS_VS_30_011F948A) != 0
}

/// The grass 3.0 byte (`011f948a`) flipped.
fn flip_grass_shader_30(e: &mut Engine) {
    let flipped = u8::from(e.global::<u8>(GRASS_VS_30_011F948A) == 0);
    e.set_global(GRASS_VS_30_011F948A, flipped);
}

/// The macro pair (`TRANSPARENCYMS`, "") written over the first pair of the block's macros
/// (from `macros`) whose name is null.
fn append_transparency_macro(e: &mut Engine, macros: u32) {
    let mut pair = macros;
    while e.mem.u32(pair) != 0 {
        pair += 8;
    }
    e.mem.set_u32(pair, GRASS_MACRO_TRANSPARENCYMS_010B8AFC);
    e.mem.set_u32(pair + 4, GRASS_EMPTY_01011584);
}

/// The 24 vertex shaders: for each block `MakePath` of its file, the name `"GRASS2%03i.vso"`
/// with the index and `vs_2_0`, replaced by `"GRASS23x%03i.vso"` and `vs_3_0` when
/// [`grass_shader_30`]; the vertex compile into the `NiPointer` +0x128 + 4 * index.
fn grass_vertex_shaders(e: &mut Engine, this: u32, frame: u32) {
    let name = frame + GRASS_NAME;
    let path = frame + GRASS_PATH;
    for index in 0..0x18u32 {
        let block = frame + 0x248 + 0x4c * index;
        let file = e.mem.u32(block);
        e.call(MAKE_PATH, &args![file, path]);
        e.call(
            FORMAT_00406D00,
            &args![name, 0x104u32, FORMAT_GRASS2_VERTEX_010B8B34, index],
        );
        let mut profile = GRASS_PROFILE_VS_2_0_010A93E4;
        if grass_shader_30(e) {
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, FORMAT_GRASS23X_VERTEX_010B8B20, index],
            );
            profile = GRASS_PROFILE_VS_3_0_010A93DC;
        }
        let shader = e
            .call(
                COMPILE_VERTEX_SHADER,
                &args![this, path, block + 4, profile, name],
            )
            .u32();
        ni_pointer_assign(e, this + 0x128 + 4 * index, shader);
    }
}

/// One pixel compile of block `block` (its file `MakePath`ed, its macros from +4) with the
/// profile into the `NiPointer` at `slot`; the name is already formatted.
fn grass_pixel_compile(e: &mut Engine, this: u32, frame: u32, block: u32, slot: u32, profile: u32) {
    let shader = e
        .call(
            COMPILE_PIXEL_SHADER,
            &args![
                this,
                frame + GRASS_PATH,
                block + 4,
                profile,
                frame + GRASS_NAME
            ],
        )
        .u32();
    ni_pointer_assign(e, slot, shader);
}

// Translated from 00baafc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TallGrassShader::LoadShaders` (engine map; vtable `010b8980` slot +0x13c), no stack
/// words. With the thread's allocation context at 0x1c (put back at the end): fills 31 source
/// blocks on its stack (24 for `lowDetail.v.hlsl`, each with (`VS_2_0`, null) first, then
/// combinations of `VERTLIT`, `FIT_TO_SLOPE`, `PTLIGHT`, `PROJ_SHADOW`, `BILLBOARD` and
/// `SHADOWMAP`; 7 for `highDetail.p.hlsl` and `highDetail_1pt.p.hlsl` with (`PS_2_0`, null),
/// `PROJ_SHADOW`, or `SHADOWMAP` with `DEPTHBIAS` "-0.7", `SAMPLE` "1"/"4" and `PASSES`
/// "1"/"2"; the rest cleared by `memset` where a block is not written in full), takes the
/// pixel profile from `00b4f380(1)` and, at shader model (`011f91c0`) 1, stops there.
///
/// Otherwise compiles the 24 vertex shaders (`"GRASS2%03i.vso"`/`vs_2_0`, or
/// `"GRASS23x%03i.vso"`/`vs_3_0` when shader 3.0 (`011f94a4`) and the grass byte `011f948a`
/// are both set) into +0x128 to +0x184; when the byte `011f948b` and `011f94a4` are set,
/// twice, flipping `011f948a` before each round. Then the seven pixel shaders into +0x188 to
/// +0x1a0 (blocks 4 to 6 only when `004dc060(0)` answers 5 or more; their pointers are left
/// otherwise):
/// - without `011f948b`: when the byte `011f9421` is set, (`TRANSPARENCYMS`, "") is put on
///   the first free macro pair and the name is the `TMS` one; the name is `"GRASS23x..."`
///   and the profile becomes `ps_3_0` when both 3.0 bytes are set, else `"GRASS2..."`; one
///   compile.
/// - with `011f948b`: `011f9421` is set to 1, then four compiles into the same pointer, each
///   first flipping `011f9421` on the even rounds (so 0, 0, 1, 1) and `011f948a` every round;
///   a set `011f9421` appends `TRANSPARENCYMS` and uses `"GRASS2%03iTMS.pso"`, otherwise
///   `"GRASS2%03i.pso"`; when both 3.0 bytes are set the name is formatted again as
///   `"GRASS23x%03iTMS.pso"` or `"GRASS23x%03i.pso"` (by `011f9421`) and the profile becomes
///   `ps_3_0`, otherwise the profile is asked from `00b4f380(1)` again.
///
/// The profile chosen is kept from one compile to the next.
// Translation incomplete: 00baafc0 - the C++ exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) of the stack frame are not translated.
pub fn tall_grass_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    with_allocation_context(e, CONTEXT_TALL_GRASS, |e| {
        e.with_stack(GRASS_FRAME_BYTES, |e, frame| {
            let frame = frame.addr();
            set_up_sources(e, frame, &GRASS_SOURCES);
            let mut profile = e.call(GRASS_PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
            if e.global::<u32>(SHADER_MODEL_011F91C0) == 1 {
                return;
            }
            if e.global::<u8>(SWITCH_011F948B) != 0
                && e.global::<u8>(SHADER_30_SUPPORT_011F94A4) != 0
            {
                for _ in 0..2 {
                    flip_grass_shader_30(e);
                    grass_vertex_shaders(e, this, frame);
                }
            } else {
                grass_vertex_shaders(e, this, frame);
            }
            let name = frame + GRASS_NAME;
            let path = frame + GRASS_PATH;
            for index in 0..7u32 {
                if index >= 4 && (e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32() as i32) < 5 {
                    continue;
                }
                let block = frame + 0x34 + 0x4c * index;
                let slot = this + 0x188 + 4 * index;
                if e.global::<u8>(SWITCH_011F948B) == 0 {
                    let format = if e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421) != 0 {
                        append_transparency_macro(e, block + 4);
                        if grass_shader_30(e) {
                            FORMAT_GRASS23X_TMS_PIXEL_010B8AD8
                        } else {
                            FORMAT_GRASS2_TMS_PIXEL_010B8B0C
                        }
                    } else if grass_shader_30(e) {
                        FORMAT_GRASS23X_PIXEL_010B8AC4
                    } else {
                        FORMAT_GRASS2_PIXEL_010B8AEC
                    };
                    e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
                    if grass_shader_30(e) {
                        profile = GRASS_PROFILE_PS_3_0_010A93B4;
                    }
                    let file = e.mem.u32(block);
                    e.call(MAKE_PATH, &args![file, path]);
                    grass_pixel_compile(e, this, frame, block, slot, profile);
                    continue;
                }
                let mut multisampling = 1u8;
                e.set_global(TRANSPARENCY_MULTISAMPLING_011F9421, multisampling);
                for round in 0..4u32 {
                    if round > 0 {
                        multisampling = e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421);
                    }
                    if round % 2 == 0 {
                        multisampling = u8::from(multisampling == 0);
                        e.set_global(TRANSPARENCY_MULTISAMPLING_011F9421, multisampling);
                    }
                    flip_grass_shader_30(e);
                    let format = if multisampling != 0 {
                        append_transparency_macro(e, block + 4);
                        FORMAT_GRASS2_TMS_PIXEL_010B8B0C
                    } else {
                        FORMAT_GRASS2_PIXEL_010B8AEC
                    };
                    e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
                    let file = e.mem.u32(block);
                    e.call(MAKE_PATH, &args![file, path]);
                    if grass_shader_30(e) {
                        let format = if e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421) != 0 {
                            FORMAT_GRASS23X_TMS_PIXEL_010B8AD8
                        } else {
                            FORMAT_GRASS23X_PIXEL_010B8AC4
                        };
                        e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
                        profile = GRASS_PROFILE_PS_3_0_010A93B4;
                    } else {
                        profile = e.call(GRASS_PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
                    }
                    grass_pixel_compile(e, this, frame, block, slot, profile);
                }
            }
        });
    });
}

/// `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (`00b70600`, `thiscall`, `RET 4`:
/// an out word), `D3DXVec3TransformCoord` (`00ee6df8`, `stdcall`: out, vector, matrix),
/// `D3DXMatrixMultiply` (`00ee6de0`, `stdcall`: out, a, b), `D3DXMatrixTranspose`
/// (`00ee6dd4`, `stdcall`: out, matrix) and the matrix builder `00b71a40` (`cdecl`: out,
/// rotation, translation, scale).
const FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const D3DX_VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;
const D3DX_MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const D3DX_MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
const BUILD_MATRIX_00B71A40: u32 = 0x00b7_1a40;
/// The point light block `UpdatePointLights` fills (`01200540`, four floats).
const POINT_LIGHT_01200540: u32 = 0x0120_0540;
/// The camera offset `00baa850` adds to the translation (`011f474c`, three floats) and the
/// matrix blocks it fills (`01200510`, `01200520`, `01200540`, `01200570`, four floats each).
const OFFSET_011F474C: u32 = 0x011f_474c;
const MATRIX_ROWS_00BAA850: [u32; 4] = [0x0120_0510, 0x0120_0520, 0x0120_0540, 0x0120_0570];

// Translated from 00baa720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TallGrassShader::UpdatePointLights` (engine map; `thiscall`, `RET 0x10`: lighting
/// property, matrix, scale float, pass): the first active non-shadow light's object (+0xf8 of
/// what `00b70600` answers) gives its colour (+0xd4, three floats) with 1.0 at +0x1b0 of the
/// shader, its position (+0x8c) transformed by the matrix (`D3DXVec3TransformCoord`) and its
/// range (+0xe0; divided by the scale unless the pass is a shadow pass, `IsShadowPass`) into
/// `01200540`. Every float goes through `FLD`/`FSTP`; the range is kept in the first
/// argument's stack slot as in the exe.
pub fn tall_grass_shader_update_point_lights(
    e: &mut Engine,
    this: Ptr,
    property: u32,
    matrix: u32,
    scale: f32,
    pass: u32,
) {
    let base = this.addr();
    e.with_stack(0x20, |e, locals| {
        let out_word = locals.addr();
        let position = out_word + 4;
        let transformed = out_word + 0x10;
        e.mem.set_u32(out_word, 0);
        let light = e
            .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![property, out_word])
            .u32();
        let object = e.mem.u32(light.wrapping_add(0xf8));
        for (index, offset) in [0xd4u32, 0xd8, 0xdc].into_iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(object.wrapping_add(offset)));
            e.mem.set_u32(base + 0x1b0 + 4 * index as u32, value);
        }
        e.mem.set_u32(base + 0x1bc, x87_store32(1.0));
        for (index, offset) in [0x8cu32, 0x90, 0x94].into_iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(object.wrapping_add(offset)));
            e.mem.set_u32(position + 4 * index as u32, value);
        }
        e.call(
            D3DX_VEC3_TRANSFORM_COORD,
            &args![transformed, position, matrix],
        );
        let mut range = x87_quiet_bits(e.mem.u32(object.wrapping_add(0xe0)));
        if !e.call(IS_SHADOW_PASS, &args![pass]).bool() {
            range = x87_store32(x87_div(x87_load32(range), x87_m32(scale.to_bits())));
        }
        for index in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(transformed + 4 * index));
            e.mem.set_u32(POINT_LIGHT_01200540 + 4 * index, value);
        }
        e.mem
            .set_u32(POINT_LIGHT_01200540 + 0xc, x87_quiet_bits(range));
    });
}

// Translated from 00baa850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: a word, the world transform): the shadow
/// pass setup. Two identity matrices on the (16-byte aligned) stack; without the word the
/// first is built from the transform (`00b71a40` with its rotation, translation +0x24 and
/// scale +0x30), and the camera offset at `011f474c` is added to its translation row. It is
/// multiplied (`D3DXMatrixMultiply`) by the 16 floats at +0x10 of the object the current
/// record (`011f91e0` +0xc) holds, the product transposed (`D3DXMatrixTranspose`) into the
/// second and stored row by row at `01200510`, `01200520`, `01200540`, `01200570`. That
/// object's light (+0xf8) then gives its colour (+0xd4) with 1.0 at +0x1b0, its position
/// (+0x8c) at +0x1c0 and its range (+0xe0) at +0x1cc of the shader (every float through
/// `FLD`/`FSTP`).
pub fn fn_00baa850(e: &mut Engine, this: Ptr, word: u32, transform: u32) {
    let base = this.addr();
    let one = x87_store32(1.0);
    e.with_stack(0x110, |e, frame| {
        let second = frame.addr();
        let first = second + 0x58;
        let record_matrix = second + 0x98;
        let product = second + 0xd8;
        for matrix in [second, first] {
            for index in 0..16u32 {
                let value = if index % 5 == 0 { one } else { 0 };
                e.mem.set_u32(matrix + 4 * index, value);
            }
        }
        if word == 0 {
            let scale = x87_quiet_bits(e.mem.u32(transform.wrapping_add(0x30)));
            e.call(
                BUILD_MATRIX_00B71A40,
                &args![first, transform, transform.wrapping_add(0x24), scale],
            );
        }
        let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
        for index in 0..3u32 {
            let offset = e.global::<u32>(OFFSET_011F474C + 4 * index);
            let cell = first + 0x30 + 4 * index;
            let sum = x87_add(x87_load32(offset), x87_m32(e.mem.u32(cell)));
            e.mem.set_u32(cell, x87_store32(sum));
        }
        let holder = e.mem.u32(record.wrapping_add(0xc));
        let object = e.mem.u32(holder);
        for index in 0..16u32 {
            let word = e.mem.u32(object.wrapping_add(0x10 + 4 * index));
            e.mem.set_u32(record_matrix + 4 * index, word);
        }
        e.call(D3DX_MATRIX_MULTIPLY, &args![product, first, record_matrix]);
        for index in 0..16u32 {
            let word = e.mem.u32(product + 4 * index);
            e.mem.set_u32(second + 4 * index, word);
        }
        e.call(D3DX_MATRIX_TRANSPOSE, &args![second, second]);
        for (row, global) in MATRIX_ROWS_00BAA850.into_iter().enumerate() {
            for column in 0..4u32 {
                let value = x87_quiet_bits(e.mem.u32(second + 0x10 * row as u32 + 4 * column));
                e.mem.set_u32(global + 4 * column, value);
            }
        }
        let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
        let holder = e.mem.u32(record.wrapping_add(0xc));
        let light = e.mem.u32(e.mem.u32(holder).wrapping_add(0xf8));
        let read =
            |e: &mut Engine, offset: u32| x87_quiet_bits(e.mem.u32(light.wrapping_add(offset)));
        let color = [read(e, 0xd4), read(e, 0xd8), read(e, 0xdc)];
        let position = [read(e, 0x8c), read(e, 0x90), read(e, 0x94)];
        let range = read(e, 0xe0);
        // The exe also loads +0xe4 and +0xe8 into locals it never uses.
        let _ = (read(e, 0xe4), read(e, 0xe8));
        for (index, value) in color.into_iter().chain([one]).enumerate() {
            e.mem.set_u32(base + 0x1b0 + 4 * index as u32, value);
        }
        for (index, value) in position.into_iter().chain([range]).enumerate() {
            e.mem.set_u32(base + 0x1c0 + 4 * index as u32, value);
        }
    });
}

// Translated from 00bac8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): vtable `+0x13c` on `this`, then the tail call to
/// `00bac160` (the passes).
pub fn fn_00bac8b0(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x13c, &args![]);
    fn_00bac160(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_a720,
            tall_grass_shader_update_point_lights(Ptr, u32, u32, f32, u32)
        ),
        entry!(0x00ba_a850, fn_00baa850(Ptr, u32, u32)),
        entry!(0x00ba_c8b0, fn_00bac8b0(Ptr)),
        entry!(0x00ba_cd00, fn_00bacd00(Ptr, Ptr) -> Ptr),
        entry!(0x00ba_cf00, fn_00bacf00() -> Ptr),
        entry!(0x00ba_a570, fn_00baa570(Ptr)),
        entry!(0x00ba_ced0, fn_00baced0(Ptr, u32) -> Ptr),
        entry!(0x00ba_af40, fn_00baaf40(Ptr, u32, Ptr)),
        entry!(
            0x00ba_ac10,
            fn_00baac10(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ba_c8d0, fn_00bac8d0(Ptr, u32)),
        entry!(0x00ba_c950, fn_00bac950(Ptr, Ptr)),
        entry!(0x00ba_c9c0, fn_00bac9c0(Ptr, Ptr)),
        entry!(0x00ba_afc0, tall_grass_shader_load_shaders(Ptr)),
        entry!(0x00ba_c160, fn_00bac160(Ptr)),
        entry!(0x00ba_d060, fn_00bad060(Ptr) -> bool),
        entry!(0x00ba_d100, tall_grass_shader_create_constant_maps(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bacf00_makes_the_declaration_then_the_shader() {
        let mut e = rig(TALL_GRASS_SHADER_CONSTRUCT_00BACD00, &[]);
        let (shader, log) = run(&mut e, 0x00ba_cf00);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 5, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 4, 6, 0x8000_0000, 0, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x1e0]),
                (TALL_GRASS_SHADER_CONSTRUCT_00BACD00, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_TALL_GRASS);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(WORDS_011A9BD0 + 4 * index, 0x70 + index);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x1e0);
        e.mem.write(this, &[0xcc; 0x1e0]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 1);
        assert_eq!(e.call(0x00ba_cd00, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bacd00_builds_the_shader() {
        let (e, this, declaration) = construct(3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), TALL_GRASS_VTABLE_010B8980);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x2c, DESTRUCT_00B79930),
                pointer_array(this + 0x128, 0x18, DESTRUCT_00B66F20),
                pointer_array(this + 0x188, 7, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x1b0u32..=0x1dc).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x68), 6);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 2);
        assert_eq!(e.mem.u32(this + 0x11c), 0);
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(WORDS_01200570 + 4 * index), 0x70 + index);
        }
        assert_eq!(e.mem.u16(this + 0x1ac), 0xe4);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40]]
        );
        assert_ne!(e.mem.u32(this + 0x1a4), 0);
        assert_eq!(e.mem.u32(this + 0x1a8), 0);
    }

    #[test]
    fn fn_00bacd00_small_count_below_model_2() {
        let (e, this, _) = construct(1);
        assert_eq!(e.mem.u16(this + 0x1ac), 0x50);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x500]]
        );
        let (e, this, _) = construct(-7);
        assert_eq!(e.mem.u16(this + 0x1ac), 0x50);
    }
}

#[cfg(test)]
mod shader_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load_grass(model: u32, bytes: [u8; 4], query: u32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(SHADER_MODEL_011F91C0, model);
        e.set_global(SWITCH_011F948B, bytes[0]);
        e.set_global(TRANSPARENCY_MULTISAMPLING_011F9421, bytes[1]);
        e.set_global(SHADER_30_SUPPORT_011F94A4, bytes[2]);
        e.set_global(GRASS_VS_30_011F948A, bytes[3]);
        e.register(GRASS_PIXEL_PROFILE_00B4F380, |_, _| {
            0x0e55_0000u32.into_ret()
        });
        e.register_double(RENDERER_QUERY_004DC060, move |_, _| query.into_ret());
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 3);
        let this = e.mem.alloc(0x1e0);
        e.call(0x00ba_afc0, &args![this]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 3);
        (e, this)
    }

    #[test]
    fn load_shaders_compiles_3x_shaders_with_multisampling() {
        let (e, this) = load_grass(2, [0, 1, 1, 1], 5);
        let vertex = e.mem.u32(this + 0x128 + 4 * 0x17);
        assert_eq!(
            compiled(&e, vertex),
            Compiled {
                file: LOW_DETAIL_VERTEX_010B8BCC,
                output: FORMAT_GRASS23X_VERTEX_010B8B20,
                profile: GRASS_PROFILE_VS_3_0_010A93DC,
                pixel: false,
                macros: macro_words(&[
                    (GRASS_MACRO_VS_2_0_010B8BC4, 0),
                    (GRASS_MACRO_SHADOWMAP_010AE95C, GRASS_EMPTY_01011584),
                    (GRASS_MACRO_BILLBOARD_010B8BA0, GRASS_EMPTY_01011584),
                    (GRASS_MACRO_FIT_TO_SLOPE_010B8BAC, GRASS_EMPTY_01011584)
                ]),
            }
        );
        let pixel = e.mem.u32(this + 0x188 + 4 * 6);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: HIGH_DETAIL_PIXEL_010B8B7C,
                output: FORMAT_GRASS23X_TMS_PIXEL_010B8AD8,
                profile: GRASS_PROFILE_PS_3_0_010A93B4,
                pixel: true,
                macros: macro_words(&[
                    (GRASS_MACRO_PS_2_0_010B8B74, 0),
                    (GRASS_MACRO_SHADOWMAP_010AE95C, GRASS_EMPTY_01011584),
                    (
                        GRASS_MACRO_DEPTHBIAS_010AEE24,
                        GRASS_VALUE_DEPTH_BIAS_010B8B44
                    ),
                    (GRASS_MACRO_SAMPLE_010AEE54, GRASS_VALUE_4_01063F24)
                ]),
            }
        );
        assert_eq!(calls(&e, MEMSET).len(), 21);
        assert_eq!(calls(&e, COMPILE_VERTEX_SHADER).len(), 0x18);
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), 7);
    }

    #[test]
    fn load_shaders_switch_compiles_rounds() {
        let (e, this) = load_grass(2, [1, 0, 1, 0], 4);
        // Two vertex rounds: 3.0 names first, then the 2.0 ones are kept.
        assert_eq!(calls(&e, COMPILE_VERTEX_SHADER).len(), 0x30);
        let vertex = e.mem.u32(this + 0x128);
        assert_eq!(compiled(&e, vertex).output, FORMAT_GRASS2_VERTEX_010B8B34);
        assert_eq!(compiled(&e, vertex).profile, GRASS_PROFILE_VS_2_0_010A93E4);
        // Four pixel blocks, four compiles each; the last one is kept.
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), 16);
        let pixel = e.mem.u32(this + 0x188);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: HIGH_DETAIL_PIXEL_010B8B7C,
                output: FORMAT_GRASS2_TMS_PIXEL_010B8B0C,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[
                    (GRASS_MACRO_PS_2_0_010B8B74, 0),
                    (GRASS_MACRO_TRANSPARENCYMS_010B8AFC, GRASS_EMPTY_01011584),
                    (GRASS_MACRO_TRANSPARENCYMS_010B8AFC, GRASS_EMPTY_01011584)
                ]),
            }
        );
        let profiles: Vec<u32> = calls(&e, COMPILE_PIXEL_SHADER)[..4]
            .iter()
            .map(|a| a[3])
            .collect();
        assert_eq!(
            profiles,
            vec![
                GRASS_PROFILE_PS_3_0_010A93B4,
                0x0e55_0000,
                GRASS_PROFILE_PS_3_0_010A93B4,
                0x0e55_0000
            ]
        );
        assert_eq!(e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421), 1);
        assert!((4..7).all(|i| e.mem.u32(this + 0x188 + 4 * i) == 0));
    }

    #[test]
    fn load_shaders_stops_at_model_1() {
        let (e, _) = load_grass(1, [0, 0, 0, 0], 5);
        assert_eq!(calls(&e, MEMSET).len(), 21);
        assert!(calls(&e, COMPILE_VERTEX_SHADER).is_empty());
        assert!(calls(&e, COMPILE_PIXEL_SHADER).is_empty());
    }

    /// The tall grass passes with every shader slot holding an object: the 0x2c passes, the
    /// vertex shaders +0x128 to +0x184 and the pixel shaders +0x188 to +0x1a0.
    fn passes_rig(query: u32, stages: u32) -> (Engine, u32, Vec<u32>) {
        let mut e = pass_rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.register_double(RENDERER_QUERY_004DC060, move |_, _| query.into_ret());
        e.set_global(FOG_SWITCH_011F941E, 1u8);
        e.set_global(TEXTURE_011F9534, 0x0e77_0000u32);
        e.set_global(SHADOW_FILTER_RESULT_011F948C, 1u32);
        e.set_global(STATE_VALUE_01200458, 0.5f32);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 3);
        let this = e.mem.alloc(0x1e0);
        let passes: Vec<u32> = (0..0x2cu32)
            .map(|index| {
                let pass = e.mem.alloc(0x70);
                e.mem.set_u32(pass + 0x18, stages);
                e.mem.set_u32(this + 0x6c + 4 * index, pass);
                pass
            })
            .collect();
        for offset in (0x128u32..0x1a4).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        e.call(0x00ba_c160, &args![this]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 3);
        (e, this, passes)
    }

    /// The (index, three words) of each stage set on the pass, in order.
    fn set_stages(e: &Engine, pass: u32) -> Vec<(u32, [u32; 3])> {
        let setups = calls(e, SETUP_DEFAULT_STAGE);
        calls(e, PASS_SET_STAGE)
            .iter()
            .filter(|a| a[0] == pass)
            .map(|a| {
                let setup = setups.iter().find(|s| s[0] == a[2]).unwrap();
                (setup[1], [setup[2], setup[3], setup[4]])
            })
            .collect()
    }

    #[test]
    fn fn_00bac160_fills_the_three_pass_groups() {
        let (e, this, passes) = passes_rig(2, 0);
        let shader = |offset: u32| e.mem.u32(this + offset);
        assert_eq!(set_stages(&e, passes[0]), vec![(0, [3, 2, 0])]);
        assert_eq!(
            render_states(&e, passes[0]),
            vec![
                (0x1b, 0),
                (0xf, 1),
                (0x19, 5),
                (0x18, 0xa),
                (0x1b, 1),
                (0x13, 5),
                (0x14, 6)
            ]
        );
        assert_eq!(e.mem.u32(passes[0] + 0x5c), shader(0x128));
        assert_eq!(e.mem.u32(passes[0] + 0x44), shader(0x188));
        assert_eq!(
            set_stages(&e, passes[8]),
            vec![(0, [3, 2, 0]), (1, [1, 2, 0]), (2, [1, 2, 0])]
        );
        assert_eq!(e.mem.u32(passes[8] + 0x44), shader(0x190));
        assert_eq!(e.mem.u32(passes[0x13] + 0x5c), shader(0x128 + 0x13 * 4));
        // The second group: two stages, the second with the texture word.
        assert_eq!(
            set_stages(&e, passes[0x14]),
            vec![(0, [3, 2, 0]), (1, [3, 2, 0])]
        );
        let second = calls(&e, PASS_SET_STAGE)
            .iter()
            .filter(|a| a[0] == passes[0x14])
            .map(|a| a[2])
            .nth(1)
            .unwrap();
        assert_eq!(e.mem.u32(second + 8), 0x0e77_0000);
        assert_eq!(e.mem.u32(second + 0xc), 1);
        assert_eq!(e.mem.u32(passes[0x14] + 0x5c), shader(0x128));
        assert_eq!(e.mem.u32(passes[0x14] + 0x44), shader(0x18c));
        assert_eq!(set_stages(&e, passes[0x1c]).len(), 4);
        assert_eq!(e.mem.u32(passes[0x1c] + 0x44), shader(0x194));
        assert_eq!(
            render_states(&e, passes[0x14]),
            vec![
                (0xf, 1),
                (0x19, 5),
                (0x18, 0xa),
                (0x1b, 1),
                (0x13, 5),
                (0x14, 6)
            ]
        );
        // The last group.
        assert_eq!(
            render_states(&e, passes[0x28]),
            vec![
                (0x1b, 1),
                (0x13, 9),
                (0x14, 1),
                (0xe, 0),
                (0xc3, 0.5f32.to_bits()),
                (0x98, 0x3f)
            ]
        );
        assert_eq!(e.mem.u32(passes[0x2b] + 0x5c), shader(0x178 + 3 * 4));
        assert_eq!(e.mem.u32(passes[0x2b] + 0x44), shader(0x19c));
        // The last stage is still held by its pass: dropped, not deleted.
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn fn_00bac160_keeps_stages_and_sets_the_low_states() {
        let (e, _, passes) = passes_rig(1, 4);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert_eq!(render_states(&e, passes[8]), vec![(0x1b, 0)]);
        assert_eq!(render_states(&e, passes[0x1c]), vec![(0xf, 0)]);
        assert_eq!(render_states(&e, passes[0x2a]).len(), 6);
    }

    /// Test vtable targets: slot byte offset `o` of the table at `VTABLE + n * 0x1000` calls
    /// `SLOTS + n * 0x1000 + o`.
    const SLOTS: u32 = 0x0e00_0000;
    const VTABLE: u32 = 0x0e80_0000;

    fn vtable(e: &mut Engine, n: u32) -> u32 {
        let table = VTABLE + n * 0x1000;
        let targets: Vec<u32> = (0..0x60u32).map(|i| SLOTS + n * 0x1000 + 4 * i).collect();
        e.put_vtable(table, &targets);
        table
    }

    fn slot(n: u32, offset: u32) -> u32 {
        SLOTS + n * 0x1000 + offset
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn map_globals(e: &mut Engine) {
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_e000,
            0x0104_e000,
            0x011a_9000,
            0x011a_d000,
            0x011f_4000,
            0x011f_6000,
            0x011f_9000,
            0x011f_a000,
            0x011f_d000,
            0x0120_0000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
    }

    #[test]
    fn fn_00baa570_destroys_the_arrays_and_the_base() {
        let mut e = rig();
        noop(
            &mut e,
            &[MEMORY_DEALLOCATE, VECTOR_DESTRUCT, BS_SHADER_DESTRUCT],
        );
        let this = e.mem.alloc(0x1e0);
        e.mem.set_u32(this + 0x1a4, 0x1234);
        e.mem.set_u32(this + 0x1a8, 9);
        e.call(0x00ba_a570, &args![this]);
        assert_eq!(e.mem.u32(this), TALL_GRASS_VTABLE_010B8980);
        assert_eq!(e.mem.u32(this + 0x1a8), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1234]]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x188, 4, 7, DESTRUCT_00B66F20],
                vec![this + 0x128, 4, 0x18, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 0x2c, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00baced0_frees_only_with_bit_0() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                MEMORY_DEALLOCATE,
                VECTOR_DESTRUCT,
                BS_SHADER_DESTRUCT,
                NI_FREE_00AA1460,
            ],
        );
        let this = e.mem.alloc(0x1e0);
        assert_eq!(e.call(0x00ba_ced0, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
        e.call(0x00ba_ced0, &args![this, 3u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x1e0]]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT).len(), 2);
    }

    /// The instance-data holder: data at +0x7c with a count, an active count, the array, a
    /// first word whose +0xb8 is an object with vtable 1 and a `u16` +0x40, and an object +8
    /// with a `u16` +0x34.
    fn instance_holder(e: &mut Engine, count: u16, active: u16) -> (u32, u32, u32) {
        let holder = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 0x7c, data);
        e.mem.set_u16(data + 0x10, count);
        e.mem.set_u16(data + 0x12, active);
        e.mem.set_u32(data + 0x14, 0x5000);
        let owner = e.mem.alloc(0xc0);
        let object = e.mem.alloc(0x44);
        let table = vtable(e, 1);
        e.mem.set_u32(object, table);
        e.mem.set_u16(object + 0x40, 0x99);
        e.mem.set_u32(owner + 0xb8, object);
        e.mem.set_u32(data, owner);
        let partition = e.mem.alloc(0x38);
        e.mem.set_u16(partition + 0x34, 0x0102);
        e.mem.set_u32(data + 8, partition);
        (holder, data, object)
    }

    #[test]
    fn fn_00baaf40_copies_the_instances_and_sets_the_count() {
        let mut e = rig();
        noop(&mut e, &[MEMCPY, slot(1, 0xa4)]);
        let this = e.mem.alloc(0x1e0);
        e.mem.set_u32(this + 0x1a4, 0x6000);
        let entry = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1a8, entry);
        // All instances active: the object's own count.
        let (holder, _, object) = instance_holder(&mut e, 3, 3);
        e.call(0x00ba_af40, &args![this, 0u32, holder]);
        assert_eq!(calls(&e, MEMCPY), vec![vec![0x6000, 0x5000, 0x30]]);
        assert_eq!(calls(&e, slot(1, 0xa4)), vec![vec![object, 0x99]]);
        assert_eq!(e.mem.u32(entry + 0x20), 3);
        // Fewer active: the 16-bit product with the per-instance count.
        let (holder, _, object) = instance_holder(&mut e, 0x300, 0x200);
        e.mem.set_u32(this + 0x1a8, 0);
        e.call(0x00ba_af40, &args![this, 0u32, holder]);
        assert_eq!(calls(&e, slot(1, 0xa4))[1], vec![object, 0x0400]);
    }

    #[test]
    fn fn_00bac8d0_sets_the_constant_blocks_of_its_pass_ranges() {
        let mut e = rig();
        map_globals(&mut e);
        let state = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x10);
        let blocks: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc)).collect();
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *block);
        }
        e.mem.set_u32(state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, state);
        e.set_global(CONSTANTS_011F95E4, 0xaaaau32);
        e.set_global(CONSTANTS_011F95E8, 0xbbbbu32);
        let values = |e: &Engine| blocks.iter().map(|b| e.mem.u32(b + 8)).collect::<Vec<_>>();
        e.call(0x00ba_c8d0, &args![0u32, 0x1bu32]);
        assert_eq!(values(&e), vec![0; 4]);
        e.call(0x00ba_c8d0, &args![0u32, 0x26bu32]);
        assert_eq!(values(&e), vec![0, 0xaaaa, 0xbbbb, 0]);
        e.call(0x00ba_c8d0, &args![0u32, 0x30u32]);
        assert_eq!(values(&e), vec![0, 0xaaaa, 0xaaaa, 0xbbbb]);
        for block in &blocks {
            e.mem.set_u32(block + 8, 0);
        }
        e.call(0x00ba_c8d0, &args![0u32, 0x280u32]);
        assert_eq!(values(&e), vec![0; 4]);
    }

    #[test]
    fn fn_00bac950_stores_the_texture_constants() {
        let mut e = rig();
        map_globals(&mut e);
        let state = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x10);
        let blocks = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(table, blocks[0]);
        e.mem.set_u32(table + 4, blocks[1]);
        e.mem.set_u32(state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, state);
        let this = e.mem.alloc(0x10);
        let shader_table = vtable(&mut e, 0);
        e.mem.set_u32(this, shader_table);
        let texture_object = e.mem.alloc(0x10);
        let object_table = vtable(&mut e, 1);
        e.mem.set_u32(texture_object, object_table);
        e.register(slot(1, 0xb4), |_, _| 0x7777u32.into_ret());
        noop(&mut e, &[slot(0, 0x108)]);
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, a| (a[0] + 1).into_ret());
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, texture_object);
        // A record whose +0xc points at a word whose +0x10c holds 0x4000.
        let record = e.mem.alloc(0x10);
        let property = e.mem.alloc(4);
        let pointed = e.mem.alloc(0x110);
        e.mem.set_u32(pointed + 0x10c, 0x4000);
        e.mem.set_u32(property, pointed);
        e.mem.set_u32(record + 0xc, property);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        e.set_global(CURRENT_PASS_011F91E4, 0x283u32);
        e.call(0x00ba_c950, &args![this, source]);
        assert_eq!(e.mem.u32(blocks[0] + 8), 0x7777);
        assert_eq!(e.mem.u32(blocks[1] + 8), 0);
        e.set_global(CURRENT_PASS_011F91E4, 0x287u32);
        e.call(0x00ba_c950, &args![this, source]);
        assert_eq!(e.mem.u32(blocks[1] + 8), 0x4001);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![vec![0x4000, 0]]
        );
        assert_eq!(calls(&e, slot(0, 0x108)), vec![vec![this]; 2]);
    }

    #[test]
    fn fn_00bad060_makes_the_passes_in_the_context() {
        let mut e = rig();
        noop(&mut e, &[NEW_PASS_00E804D0]);
        e.register(STAGES_AND_PASSES_00BAC160, |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + 0x2b4), CONTEXT_TALL_GRASS);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 3);
        let this = e.mem.alloc(0x1e0);
        assert!(e.call(0x00ba_d060, &args![this]).bool());
        let passes = calls(&e, NEW_PASS_00E804D0);
        assert_eq!(passes.len(), 0x2c);
        assert_eq!(passes[0], vec![this + 0x6c]);
        assert_eq!(passes[0x2b], vec![this + 0x6c + 0x2b * 4]);
        assert_eq!(calls(&e, STAGES_AND_PASSES_00BAC160), vec![vec![this]]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 3);
    }

    #[test]
    fn create_constant_maps_makes_both_maps_once() {
        let mut e = rig();
        let map_table = vtable(&mut e, 2);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(CONSTANT_MAP_CONSTRUCT_00B7E330, move |e, a| {
            e.mem.set_u32(a[0], map_table);
            a[0].into_ret()
        });
        noop(&mut e, &[slot(2, 8)]);
        e.register(slot(2, 0x70), |_, _| 0x8888u32.into_ret());
        let this = e.mem.alloc(0x1e0);
        e.mem.set_u32(this + 0x18, 0x5151);
        e.mem.set_u16(this + 0x1ac, 0x50);
        e.mem.set_u32(this + 0x1a4, 0x9000);
        e.call(0x00ba_d100, &args![this]);
        let maps = calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330);
        assert_eq!(maps.len(), 2);
        assert_eq!(&maps[0][1..], &[0x5151, 0]);
        assert_eq!(&maps[1][1..], &[0x5151, 2]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(vertex + 4), 1);
        let entries = calls(&e, slot(2, 8));
        assert_eq!(entries.len(), 9);
        assert_eq!(
            entries[0],
            vec![
                vertex,
                WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                9,
                4,
                0,
                0,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            entries[5],
            vec![
                vertex,
                INSTANCE_DATA_010B8C24,
                0x1000_0009,
                0,
                0x14,
                0x50,
                CONSTANT_EMPTY_NAME_01011584,
                0x500,
                4,
                0x9000,
                0
            ]
        );
        assert_eq!(
            entries[8],
            vec![
                pixel,
                ALPHA_TEST_REF_010B8BEC,
                0x1000_0007,
                0,
                3,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                0x10,
                4,
                this + 0x1d0,
                0
            ]
        );
        assert_eq!(e.mem.u32(this + 0x1a8), 0x8888);
        // Both maps present: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00ba_d100, &args![this]);
        assert!(calls(&e, slot(2, 8)).is_empty());
    }

    #[test]
    fn create_constant_maps_keeps_a_null_map_when_the_allocation_fails() {
        let mut e = rig();
        e.register(NI_ALLOCATE_00AA13E0, |_, _| Ret::default());
        // The entries go through the null map's vtable: map page 0 with a table of doubles.
        e.map(0, 0x1000);
        let table = vtable(&mut e, 3);
        e.mem.set_u32(0, table);
        noop(&mut e, &[slot(3, 8), slot(3, 0x70)]);
        let this = e.mem.alloc(0x1e0);
        e.call(0x00ba_d100, &args![this]);
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330).is_empty());
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(calls(&e, slot(3, 8)).len(), 9);
    }

    /// A rig for `SetupGeometry`: identity-like transform with scale 2, the D3DX doubles, a
    /// property state whose +0xc object has type `kind`.
    fn setup_rig(kind: u32, shadow: bool) -> (Engine, u32, u32, u32) {
        let mut e = rig();
        map_globals(&mut e);
        noop(
            &mut e,
            &[
                RENDERER_CALL_00E6FBB0,
                D3DX_MATRIX_INVERSE,
                POINT_CALL_004A0C10,
                SHADOW_SETUP_00BAA850,
                UPDATE_POINT_LIGHTS,
            ],
        );
        e.register(BASE_SETUP_GEOMETRY_00E807D0, |_, _| 0x4242u32.into_ret());
        e.register(D3DX_VEC3_TRANSFORM_NORMAL, |e, a| {
            // out = the vector doubled.
            for i in 0..3 {
                let v = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v * 2.0);
            }
            Ret::default()
        });
        e.register(D3DX_VEC3_NORMALIZE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v / 2.0);
            }
            Ret::default()
        });
        e.register_double(IS_SHADOW_PASS, move |_, _| u32::from(shadow).into_ret());
        e.register(ACTIVE_NON_SHADOW_LIGHTS, |_, _| 0x1_0002u32.into_ret());
        let this = e.mem.alloc(0x1e0);
        e.mem.set_u32(this + 0x18, 0x5151);
        let transform = e.mem.alloc(0x34);
        for (i, v) in [
            1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 5.0, 6.0, 7.0, 2.0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_f32(transform + 4 * i as u32, *v);
        }
        let state = e.mem.alloc(0x10);
        let property = e.mem.alloc(0x20);
        e.mem.set_u32(property + 0x1c, kind);
        e.mem.set_u32(state + 0xc, property);
        e.set_global(CURRENT_PASS_011F91E4, 0x31u32);
        (e, this, transform, state)
    }

    #[test]
    fn fn_00baac10_builds_the_matrix_and_calls_the_base() {
        let (mut e, this, transform, state) = setup_rig(1, false);
        e.register(D3DX_MATRIX_INVERSE, |e, a| {
            let matrix: Vec<f32> = (0..16).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            assert_eq!(
                matrix,
                vec![
                    2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 5.0, 6.0, 7.0, 1.0
                ]
            );
            assert_eq!(a[1], 0);
            Ret::default()
        });
        let words = [1u32, 2, 3, 4, state, 6, transform, 8];
        let answer = e
            .call(
                0x00ba_ac10,
                &args![
                    this, words[0], words[1], words[2], words[3], words[4], words[5], words[6],
                    words[7]
                ],
            )
            .u32();
        assert_eq!(answer, 0x4242);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0),
            vec![vec![0x5151, transform, 0]]
        );
        // (0, 1, 0) doubled then halved.
        assert_eq!(e.global::<f32>(UP_01200550), 0.0);
        assert_eq!(e.global::<f32>(UP_01200550 + 4), 1.0);
        let mut base = vec![this];
        base.extend_from_slice(&words);
        assert_eq!(calls(&e, BASE_SETUP_GEOMETRY_00E807D0), vec![base]);
        // Not a type-4 property: no light work.
        assert!(calls(&e, IS_SHADOW_PASS).is_empty());
    }

    #[test]
    fn fn_00baac10_takes_the_light_and_runs_the_point_lights_or_the_shadow_setup() {
        for shadow in [false, true] {
            let (mut e, this, transform, state) = setup_rig(4, shadow);
            e.set_global(POINT_LIGHTS_ON_011F9EF0, 1u8);
            let accumulator = e.mem.alloc(0x198);
            let middle = e.mem.alloc(0xe4);
            let lights = e.mem.alloc(0xfc);
            let light = e.mem.alloc(0x100);
            e.mem.set_u32(accumulator + 0x194, middle);
            e.mem.set_u32(middle + 0xe0, lights);
            e.mem.set_u32(lights + 0xf8, light);
            for (i, v) in [0.1f32, 0.2, 0.3].iter().enumerate() {
                e.mem.set_f32(light + 0xc8 + 4 * i as u32, *v);
            }
            for (i, v) in [0.4f32, 0.5, 0.6].iter().enumerate() {
                e.mem.set_f32(light + 0xd4 + 4 * i as u32, *v);
            }
            for (i, v) in [1.0f32, -2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(light + 0xf0 + 4 * i as u32, *v);
            }
            e.register_double(GET_ACCUMULATOR, move |_, _| accumulator.into_ret());
            e.call(
                0x00ba_ac10,
                &args![this, 1u32, 2u32, 3u32, 4u32, state, 6u32, transform, 8u32],
            );
            let floats = |e: &Engine, at: u32| {
                (0..4)
                    .map(|i| e.global::<f32>(at + 4 * i))
                    .collect::<Vec<_>>()
            };
            assert_eq!(floats(&e, 0x0120_0570), vec![0.1, 0.2, 0.3, 1.0]);
            assert_eq!(floats(&e, LIGHT_AMBIENT_01200520), vec![0.4, 0.5, 0.6, 1.0]);
            assert_eq!(
                floats(&e, LIGHT_DIRECTION_01200510),
                vec![-1.0, 2.0, -3.0, 1.0]
            );
            assert_eq!(calls(&e, IS_SHADOW_PASS), vec![vec![0x31]]);
            if shadow {
                assert_eq!(
                    calls(&e, SHADOW_SETUP_00BAA850),
                    vec![vec![this, 2, transform]]
                );
                assert!(calls(&e, UPDATE_POINT_LIGHTS).is_empty());
            } else {
                let update = calls(&e, UPDATE_POINT_LIGHTS);
                assert_eq!(update.len(), 1);
                assert_eq!(update[0][0], this);
                assert_eq!(update[0][1], e.mem.u32(state + 0xc));
                assert_eq!(update[0][3], 2.0f32.to_bits());
                assert_eq!(update[0][4], 0x31);
                assert!(calls(&e, SHADOW_SETUP_00BAA850).is_empty());
            }
        }
    }

    /// A rig for `00bac9c0`: the current record's geometry, the property state with an alpha
    /// object and a source object with data.
    fn draw_rig() -> (Engine, u32, u32, u32) {
        let mut e = rig();
        map_globals(&mut e);
        let this = e.mem.alloc(0x1e0);
        let table = vtable(&mut e, 0);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 0x1a4, 0x6000);
        noop(
            &mut e,
            &[slot(0, 0x60), MEMCPY, SET_ALPHA_STATE_00B97F20, CI_FMOD],
        );
        let record = e.mem.alloc(0x10);
        let object = e.mem.alloc(0xc0);
        let passes = e.mem.alloc(0x38);
        e.mem.set_u32(passes + 0x34, 3);
        e.mem.set_u32(object + 0xb8, passes);
        e.mem.set_u32(object + 0xbc, 0xbcbc);
        e.mem.set_u32(record, object);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        let state = e.mem.alloc(0x10);
        let alpha = e.mem.alloc(0x1c);
        e.mem.set_u16(alpha + 0x18, 0x1c00);
        e.mem.set_u8(alpha + 0x1a, 51);
        e.mem.set_u32(state, alpha);
        let (source, data, _) = instance_holder(&mut e, 1, 1);
        noop(&mut e, &[slot(1, 0xa4)]);
        e.mem.set_f32(data + 0x34, 2.0);
        e.mem.set_u32(state + 0xc, source);
        e.set_global(BYTE_SCALE_0101E568, 255.0f64);
        e.set_global(SECONDS_PER_HOUR_01012640, 3600.0f64);
        e.set_global(CYCLE_0104EEA0, 6.0f64);
        e.set_global(GAME_SECONDS_011F940C, 3600.0f32);
        let pass_state = e.mem.alloc(0x60);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        (e, this, state, object)
    }

    #[test]
    fn fn_00bac9c0_sets_up_the_geometry_and_the_constants() {
        let (mut e, this, state, object) = draw_rig();
        e.set_global(FULL_BRIGHT_011F91A7, 1u8);
        e.set_global(AMBIENT_011F9EE0, 0x1111u32);
        e.call(0x00ba_c9c0, &args![this, state]);
        assert_eq!(
            calls(&e, slot(0, 0x60)),
            vec![vec![
                this,
                object,
                0xbcbc,
                0,
                3,
                state,
                0,
                object + 0x68,
                DEFAULT_BOUND_011F4288
            ]]
        );
        assert_eq!(e.global::<u32>(DIFFUSE_011FA0C0 + 0xc), ONE_BITS);
        assert_eq!(e.global::<u32>(AMBIENT_01200560), 0x1111);
        assert_eq!(e.mem.f32(this + 0x1d0), 0.2);
        assert_eq!(e.mem.u32(this + 0x1d4), 0);
        // 3600 / 3600 * 6 * 2 = 12, not below the cycle: fmod (the double answers 0).
        assert_eq!(calls(&e, CI_FMOD).len(), 1);
        let fmod = &calls(&e, CI_FMOD)[0];
        assert_eq!(
            f64::from_bits(u64::from(fmod[0]) | u64::from(fmod[1]) << 32),
            12.0
        );
        assert_eq!(e.mem.u32(this + 0x11c), 0);
        assert_eq!(e.global::<u32>(FLAGS_01200530), 0);
        assert_eq!(e.global::<u32>(FLAGS_01200530 + 8), ONE_BITS);
        // Model 0: the alpha state is set from the alpha object.
        assert_eq!(calls(&e, SET_ALPHA_STATE_00B97F20), vec![vec![7, 51, 0]]);
    }

    #[test]
    fn fn_00bac9c0_keeps_a_small_phase_and_skips_the_alpha_state_with_fog() {
        let (mut e, this, state, _) = draw_rig();
        e.set_global(GAME_SECONDS_011F940C, 900.0f32);
        e.mem.set_u32(state, 0);
        e.mem.set_u32(e.mem.u32(state + 0xc) + 0x24, 0x800);
        e.set_global(FOG_SWITCH_011F941E, 1u8);
        e.set_global(FOG_VALUE_011F9188, 0.75f32);
        e.set_global(SHADER_MODEL_011F91C0, 3i32);
        e.set_global(WORDS_011A9BD0, 0x2222u32);
        // Shadows on with a nonzero fade: the shadow words are copied.
        e.set_global(CONSTANTS_011F95E4, 1u32);
        e.set_global(RENDER_FLAGS_011F91D8, 0x20u8);
        e.set_global(SHADOW_FADE_011FD890, 1.5f32);
        e.set_global(SHADOW_WORDS_011FD874, 0.5f32);
        e.set_global(SHADOW_VALUE_011FD880, 0.25f32);
        // The two constant users.
        let users = [e.mem.alloc(8), e.mem.alloc(8)];
        let table = vtable(&mut e, 4);
        for user in users {
            e.mem.set_u32(user, table);
        }
        e.mem.set_u32(this + 0x30, users[0]);
        e.mem.set_u32(this + 0x34, users[1]);
        let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
        e.mem.set_u32(pass_state + 0x44, 0x4444);
        noop(&mut e, &[slot(4, 0x78)]);
        e.call(0x00ba_c9c0, &args![this, state]);
        // 900 / 3600 * 6 * 2 = 3, below the cycle.
        assert!(calls(&e, CI_FMOD).is_empty());
        assert_eq!(e.mem.f32(this + 0x11c), 3.0);
        assert_eq!(e.global::<f32>(WIND_PHASE_0120055C), 3.0);
        assert_eq!(e.mem.u32(this + 0x1d0), 0x2222);
        assert_eq!(e.global::<u32>(FLAGS_01200530 + 4), ONE_BITS);
        assert_eq!(e.global::<f32>(FOG_VALUE_01200580), 0.75);
        assert_eq!(e.global::<f32>(SHADOW_01200590), 0.5);
        assert_eq!(e.global::<f32>(SHADOW_01200590 + 0xc), 0.25);
        assert!(calls(&e, SET_ALPHA_STATE_00B97F20).is_empty());
        // Only the first user has its constants object.
        let used = calls(&e, slot(4, 0x78));
        assert_eq!(used.len(), 1);
        assert_eq!(
            &used[0][..3],
            &[
                users[0],
                0x4444,
                e.mem.u32(e.global::<u32>(CURRENT_RECORD_011F91E0))
            ]
        );
        assert_eq!(&used[0][10..], &[0, 1]);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// A light whose object (+0xf8) holds floats at the offsets the two setups read.
    fn light(e: &mut Engine) -> (u32, u32) {
        let object = e.mem.alloc(0x100);
        for (offset, value) in [
            (0xd4u32, 0.25f32),
            (0xd8, 0.5),
            (0xdc, 0.75),
            (0x8c, 10.0),
            (0x90, 20.0),
            (0x94, 30.0),
            (0xe0, 600.0),
        ] {
            e.mem.set_f32(object + offset, value);
        }
        let light = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, object);
        (light, object)
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn update_point_lights_fills_the_light_block() {
        for (shadow, range) in [(false, 300.0f32), (true, 600.0)] {
            let mut e = rig();
            e.map(0x0120_0000, 0x1000);
            let (light, _) = light(&mut e);
            e.register_double(FIRST_ACTIVE_NON_SHADOW_LIGHT, move |_, _| light.into_ret());
            e.register(D3DX_VEC3_TRANSFORM_COORD, |e, a| {
                for i in 0..3 {
                    let v = e.mem.f32(a[1] + 4 * i) + 1.0;
                    e.mem.set_f32(a[0] + 4 * i, v);
                }
                a[0].into_ret()
            });
            e.register_double(IS_SHADOW_PASS, move |_, _| u32::from(shadow).into_ret());
            let this = e.mem.alloc(0x1e0);
            e.call(
                0x00ba_a720,
                &args![this, 0x4000u32, 0x5000u32, 2.0f32, 7u32],
            );
            assert_eq!(floats(&e, this + 0x1b0, 4), vec![0.25, 0.5, 0.75, 1.0]);
            assert_eq!(
                floats(&e, POINT_LIGHT_01200540, 4),
                vec![11.0, 21.0, 31.0, range]
            );
            assert_eq!(calls(&e, IS_SHADOW_PASS), vec![vec![7]]);
            assert_eq!(calls(&e, D3DX_VEC3_TRANSFORM_COORD)[0][2], 0x5000);
            assert_eq!(calls(&e, FIRST_ACTIVE_NON_SHADOW_LIGHT)[0][0], 0x4000);
        }
    }

    fn shadow_setup(word: u32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        for (i, v) in [100.0f32, 200.0, 300.0].into_iter().enumerate() {
            e.set_global::<f32>(OFFSET_011F474C + 4 * i as u32, v);
        }
        // The record's object: its light at +0xf8 and a matrix at +0x10 with 2.0 on the
        // diagonal and 1.0 in the first three cells of the last row.
        let (_, lit) = light(&mut e);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xf8, lit);
        for i in 0..4 {
            e.mem.set_f32(object + 0x10 + 20 * i, 2.0);
        }
        for i in 12..15 {
            e.mem.set_f32(object + 0x10 + 4 * i, 1.0);
        }
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 0xc, holder);
        e.set_global::<u32>(CURRENT_RECORD_011F91E0, record);
        e.register(BUILD_MATRIX_00B71A40, |e, a| {
            e.mem.set_f32(a[0] + 0x30, 1.0);
            Ret::default()
        });
        // Element-wise, which is enough to follow the cells.
        e.register(D3DX_MATRIX_MULTIPLY, |e, a| {
            for i in 0..16 {
                let v = e.mem.f32(a[1] + 4 * i) * e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(D3DX_MATRIX_TRANSPOSE, |e, a| {
            let m: Vec<f32> = (0..16).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            for r in 0..4 {
                for c in 0..4 {
                    e.mem
                        .set_f32(a[0] + 4 * (4 * r + c), m[(4 * c + r) as usize]);
                }
            }
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x1e0);
        let transform = e.mem.alloc(0x34);
        e.call(0x00ba_a850, &args![this, word, transform]);
        (e, this)
    }

    #[test]
    fn fn_00baa850_stores_the_transposed_matrix_and_the_light() {
        let (e, this) = shadow_setup(0);
        assert_eq!(calls(&e, BUILD_MATRIX_00B71A40).len(), 1);
        assert_eq!(floats(&e, 0x0120_0510, 4), vec![2.0, 0.0, 0.0, 101.0]);
        assert_eq!(floats(&e, 0x0120_0520, 4), vec![0.0, 2.0, 0.0, 200.0]);
        assert_eq!(floats(&e, 0x0120_0540, 4), vec![0.0, 0.0, 2.0, 300.0]);
        assert_eq!(floats(&e, 0x0120_0570, 4), vec![0.0, 0.0, 0.0, 2.0]);
        assert_eq!(floats(&e, this + 0x1b0, 4), vec![0.25, 0.5, 0.75, 1.0]);
        assert_eq!(floats(&e, this + 0x1c0, 4), vec![10.0, 20.0, 30.0, 600.0]);
        // With the word set the identity is kept: only the offset is added.
        let (e, _) = shadow_setup(1);
        assert!(calls(&e, BUILD_MATRIX_00B71A40).is_empty());
        assert_eq!(floats(&e, 0x0120_0510, 4)[3], 100.0);
    }

    #[test]
    fn fn_00bac8b0_runs_the_slot_then_the_passes() {
        let mut e = pass_rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.register(RENDERER_QUERY_004DC060, |_, _| 2u32.into_ret());
        e.set_global(FOG_SWITCH_011F941E, 1u8);
        e.set_global(TEXTURE_011F9534, 0x0e77_0000u32);
        e.set_global(SHADOW_FILTER_RESULT_011F948C, 1u32);
        e.set_global(STATE_VALUE_01200458, 0.5f32);
        let this = e.mem.alloc(0x1e0);
        for index in 0..0x2cu32 {
            let pass = e.mem.alloc(0x70);
            e.mem.set_u32(this + 0x6c + 4 * index, pass);
        }
        for offset in (0x128u32..0x1a4).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        let mut slots = vec![0u32; 0x50];
        slots[0x13c / 4] = 0x0e60_013c;
        e.put_vtable(0x0e61_0000, &slots);
        e.mem.set_u32(this, 0x0e61_0000);
        e.register(0x0e60_013c, |_, _| Ret::default());
        e.call(0x00ba_c8b0, &args![this]);
        let log = e.call_log.as_ref().unwrap();
        let slot = log.iter().position(|c| c.0 == 0x0e60_013c).unwrap();
        let first_stage = log.iter().position(|c| c.0 == PASS_SET_STAGE).unwrap();
        assert!(slot < first_stage);
    }
}
