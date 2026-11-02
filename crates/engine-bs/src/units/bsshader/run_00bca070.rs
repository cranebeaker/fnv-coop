//! `BSShader/run_00bca070` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bcaca0` (unnamed, `thiscall`, `RET 8`: two declarations).
const SHADER_CONSTRUCT_00BCACA0: u32 = 0x00bc_aca0;

// Translated from 00bcae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x20. No arguments. Makes two declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`: 2 entries and 3 entries, 1 stream each), sets the first's entries
/// through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 0, 5, 1, 0)) and
/// ends it (slot +0xb4), then the second's ((0, 0, 0, 2, 0), (1, 0, 5, 1, 0), (2, 0, 4, 3,
/// 0)) and ends it. Allocates 0x90 bytes and runs the constructor `00bcaca0` with both (a
/// null allocation is used as it is), calls the shader's slots +0x4c, +0x13c, +0x148,
/// +0x14c, +0x11c, then +0xc0 with the first declaration, and answers the shader.
// Translation incomplete: 00bcae90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcae90(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 0, 5, 1, 0]] {
        e.vcall(first, 0x8c, &entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in [[0u32, 0, 0, 2, 0], [1, 0, 5, 1, 0], [2, 0, 4, 3, 0]] {
        e.vcall(second, 0x8c, &entry);
    }
    e.vcall(second, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BCACA0, &args![block, first, second])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x14c, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bb410`; no RTTI locator before it).
const VTABLE_010BB410: u32 = 0x010b_b410;
/// The eight `float`s set to 1.0 (`01202188`..`01202194`, `012021a8`..`012021b4`).
const ONES_01202188: u32 = 0x0120_2188;
const ONES_012021A8: u32 = 0x0120_21a8;

// Translated from 00bcaca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`): the constructor the creator `00bcaf00` runs,
/// `thiscall` with two stack words (the two declarations). Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010bb410`, constructs the `NiPointer` arrays at +0x6c (1,
/// destructor `00b79930`), +0x70 and +0x7c (3 each, destructor `00b66f20`), clears the words
/// +0x88 and +0x8c, sets the byte +0x64 to 1 and +0x68 to 0x14, assigns the declarations to the
/// `NiPointer`s at +0x88 and +0x8c and stores 1.0 in the `float`s `01202188` to `01202194` and
/// `012021a8` to `012021b4`. Answers `this`.
// Translation incomplete: 00bcaca0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the base and the arrays) is not translated.
pub fn fn_00bcaca0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010BB410);
    construct_pointer_array(e, base + 0x6c, 1, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x70, 3, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 3, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x88, 0);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u8(base + 0x64, 1);
    e.mem.set_u32(base + 0x68, 0x14);
    ni_pointer_assign(e, base + 0x88, first.addr());
    ni_pointer_assign(e, base + 0x8c, second.addr());
    for index in 0..4u32 {
        e.set_global::<f32>(ONES_01202188 + 4 * index, 1.0);
    }
    for index in 0..4u32 {
        e.set_global::<f32>(ONES_012021A8 + 4 * index, 1.0);
    }
    this
}

use crate::units::bsshader::particleshader::reference_release;

/// The `float` `00bca050` inverts (`011ad83c`); the result goes to `012021a8` (the first of
/// the four `float`s the constructor sets to 1.0).
const SOURCE_011AD83C: u32 = 0x011a_d83c;

// Translated from 00bca050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the shader's vtable `010bb410`, slot +0xfc), `thiscall`
/// with one stack word the code never reads (`RET 4`): stores `1.0 / [011ad83c]` (x87
/// `FDIVRP`, rounded to a `float`) in `012021a8`.
pub fn fn_00bca050(e: &mut Engine, _this: Ptr, _unused_1: u32) {
    let value = x87_load32(e.global::<u32>(SOURCE_011AD83C));
    let inverse = x87_store32(x87_div(1.0, value));
    e.set_global::<u32>(ONES_012021A8, inverse);
}

/// Strings of the vertex loader `00bca0d0` (read in Ghidra): `"tile\tile.v.hlsl"` (`010bb3cc`),
/// the macros `"TEXSCROLL"` (`010bb3c0`) and `"VC"` (`010aed64`) with `""` (`01011584`), the
/// name format `"TILE1%03i.vso"` (`010bb3b0`) and `"vs_1_1"` (`010a93ec`).
const TILE_VERTEX_SOURCE_010BB3CC: u32 = 0x010b_b3cc;
const MACRO_TEXSCROLL_010BB3C0: u32 = 0x010b_b3c0;
const MACRO_VC_010AED64: u32 = 0x010a_ed64;
const TILE_EMPTY_STRING_01011584: u32 = 0x0101_1584;
const TILE_VERTEX_NAME_FORMAT_010BB3B0: u32 = 0x010b_b3b0;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// Strings of the pixel loader `00bca240`: `"tile\tile.p.hlsl"` (`010bb3fc`), the macro
/// `"ALPHAMAP"` (`010bb3f0`) and the name format `"TILE1%03i.pso"` (`010bb3e0`).
const TILE_PIXEL_SOURCE_010BB3FC: u32 = 0x010b_b3fc;
const MACRO_ALPHAMAP_010BB3F0: u32 = 0x010b_b3f0;
const TILE_PIXEL_NAME_FORMAT_010BB3E0: u32 = 0x010b_b3e0;
/// `00b4f380` (unnamed, unit `bsshadermanager.cpp`; `cdecl`, one word): answers the pixel
/// profile string.
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;

/// One loader of the tile shader: the three source blocks written on the stack (the file and
/// a null word then 0x44 bytes cleared; the file, `second`, `""`, a null word then 0x3c bytes
/// cleared; the same with `"VC"`), `profile` asked for (when it is `None`, `00b4f380` with 1
/// after the blocks), then for each block whose file word is set: `MakePath`, the name formatted
/// from `format` with the block's index, the compile (vertex or pixel) and the result assigned
/// to the `NiPointer` at `slots + 4 * index`.
fn load_tile_shaders(
    e: &mut Engine,
    this: u32,
    frame: u32,
    layout: (u32, ShaderBuffers),
    sources: (u32, u32),
    format: u32,
    profile: Option<u32>,
    slots: u32,
) {
    let (first, buffers) = layout;
    let (file, second) = sources;
    for (index, (macro_name, cleared)) in
        [(0u32, 0x44u32), (second, 0x3c), (MACRO_VC_010AED64, 0x3c)]
            .iter()
            .enumerate()
    {
        let block = frame + first + 0x4c * index as u32;
        e.mem.set_u32(block, file);
        if index == 0 {
            e.mem.set_u32(block + 4, 0);
            e.call(MEMSET, &args![block + 8, 0u32, *cleared]);
        } else {
            e.mem.set_u32(block + 4, *macro_name);
            e.mem.set_u32(block + 8, TILE_EMPTY_STRING_01011584);
            e.mem.set_u32(block + 0xc, 0);
            e.call(MEMSET, &args![block + 0x10, 0u32, *cleared]);
        }
    }
    let (profile, pixel) = match profile {
        Some(profile) => (profile, false),
        None => (e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32(), true),
    };
    let path = frame + buffers.path;
    let name = frame + buffers.name;
    for index in 0..3u32 {
        let block = frame + first + 0x4c * index;
        let file = e.mem.u32(block);
        if file == 0 {
            continue;
        }
        e.call(MAKE_PATH, &args![file, path]);
        e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
        let compile = if pixel {
            COMPILE_PIXEL_SHADER
        } else {
            COMPILE_VERTEX_SHADER
        };
        let shader = e
            .call(compile, &args![this, path, block + 4, profile, name])
            .u32();
        ni_pointer_assign(e, this + slots + 4 * index, shader);
    }
}

/// The vertex loader's frame (0x308 bytes up to the return address), its first source block
/// and buffers.
const TILE_VERTEX_FRAME_BYTES: u32 = 0x308;
const TILE_VERTEX_LAYOUT: (u32, ShaderBuffers) = (
    0x18,
    ShaderBuffers {
        name: 0xfc,
        path: 0x200,
    },
);
/// The pixel loader's frame (0x30c bytes), its first source block and buffers.
const TILE_PIXEL_FRAME_BYTES: u32 = 0x30c;
const TILE_PIXEL_LAYOUT: (u32, ShaderBuffers) = (
    0x1c,
    ShaderBuffers {
        name: 0x100,
        path: 0x204,
    },
);

// Translated from 00bca0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the shader's vtable `010bb410`, slot +0x140), `thiscall`,
/// no stack words: the tile shader's vertex shaders. Compiles `"tile\tile.v.hlsl"` without
/// macros, with `TEXSCROLL` and with `VC` (`"TILE1%03i.vso"` with the index, `"vs_1_1"`) into
/// the `NiPointer`s at +0x70 to +0x78.
// Translation incomplete: 00bca0d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00bca0d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(TILE_VERTEX_FRAME_BYTES, |e, frame| {
        load_tile_shaders(
            e,
            base,
            frame.addr(),
            TILE_VERTEX_LAYOUT,
            (TILE_VERTEX_SOURCE_010BB3CC, MACRO_TEXSCROLL_010BB3C0),
            TILE_VERTEX_NAME_FORMAT_010BB3B0,
            Some(PROFILE_VS_1_1_010A93EC),
            0x70,
        );
    });
}

// Translated from 00bca240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the shader's vtable `010bb410`, slot +0x144), `thiscall`,
/// no stack words: the tile shader's pixel shaders. Writes the three blocks of
/// `"tile\tile.p.hlsl"` (no macros, `ALPHAMAP`, `VC`), asks `00b4f380` (with 1) for the profile
/// and compiles them (`"TILE1%03i.pso"` with the index) into the `NiPointer`s at +0x7c to
/// +0x84.
// Translation incomplete: 00bca240 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00bca240(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(TILE_PIXEL_FRAME_BYTES, |e, frame| {
        load_tile_shaders(
            e,
            base,
            frame.addr(),
            TILE_PIXEL_LAYOUT,
            (TILE_PIXEL_SOURCE_010BB3FC, MACRO_ALPHAMAP_010BB3F0),
            TILE_PIXEL_NAME_FORMAT_010BB3E0,
            None,
            0x7c,
        );
    });
}

// Translated from 00bca3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot +0x148), `thiscall`,
/// no stack words; answers true. Gives the pass pointer at +0x6c a new pass (`00e804d0`, no null
/// test), adds two texture stages through one stack pointer: stage 0 (`SetupDefaultStage(stage,
/// 0, 1, 2, 1)`, `SetStage`, then filter mode 1) and stage 1 (`SetupDefaultStage(stage, 1, 1,
/// 2, 1)`, `SetStage`; the first stage's reference is not dropped), sets the render states (7,
/// 0), (0xe, 0), (0xa8, 0xf) and drops the second stage's reference.
// Translation incomplete: 00bca3c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bca3c0(e: &mut Engine, this: Ptr) -> bool {
    let slot = this.addr() + 0x6c;
    e.call(NEW_PASS_00E804D0, &args![slot]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 1u32, 2u32, 1u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let stage = e.mem.u32(local);
        e.call(SET_FILTER_MODE, &args![stage, 1u32]);
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 1u32, 1u32, 2u32, 1u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        for (state, value) in [(7u32, 0u32), (0xe, 0), (0xa8, 0xf)] {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// The word `00bca500` stores the shader's pass in (`0126f74c`, the render state object the
/// set-ups below read).
const RENDER_STATE_0126F74C: u32 = 0x0126_f74c;

// Translated from 00bca500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the shader's vtable `010bb410`, slot +0xf0), `thiscall`
/// with one stack word the code never reads (`RET 4`): stores the word at +0x6c (the pass
/// pointer) in `0126f74c`.
pub fn fn_00bca500(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let pass = e.mem.u32(this.addr() + 0x6c);
    e.set_global::<u32>(RENDER_STATE_0126F74C, pass);
}

/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`), the compiler's `eh vector
/// destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count, destructor) and the
/// sized free `00aa1460` (`cdecl`: block, size).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bca510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`): the destructor of `00bcaca0`'s shader,
/// `thiscall`, no stack words. Stores the vtable `010bb410`, clears the `NiPointer`s at +0x70 to
/// +0x78 and +0x7c to +0x84 and the pass pointer at +0x6c, clears the byte +0x64, the
/// `NiPointer` at +0x88 and the word +0x60; the members' destructors then run on what is left
/// (+0x8c, +0x88, the arrays +0x7c and +0x70 through the `eh vector destructor iterator` with
/// `00b66f20`, +0x6c with `00b79930`) and `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bca510 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the members) is not translated.
pub fn fn_00bca510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BB410);
    for offset in [0x70u32, 0x74, 0x78, 0x7c, 0x80, 0x84] {
        ni_pointer_clear(e, base + offset);
    }
    pass_pointer_clear(e, base + 0x6c);
    e.mem.set_u8(base + 0x64, 0);
    ni_pointer_clear(e, base + 0x88);
    e.mem.set_u32(base + 0x60, 0);
    for offset in [0x8cu32, 0x88] {
        let object = e.mem.u32(base + offset);
        reference_release(e, object);
    }
    for (array, count, destructor) in [
        (0x7cu32, 3u32, DESTRUCT_00B66F20),
        (0x70, 3, DESTRUCT_00B66F20),
        (0x6c, 1, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT_00EC5FCE,
            &args![base + array, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00bca6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the shader's vtable `010bb410`, slot +0x8), `thiscall`,
/// no stack words: answers `012021b8`.
pub fn fn_00bca6b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_21b8
}

/// `00be0870` (unnamed, `thiscall`, no stack words).
const SHADER_CALL_00BE0870: u32 = 0x00be_0870;

// Translated from 00bca6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot +0x4c), `thiscall`,
/// no stack words; answers true. Runs `00be0870` on `this` (its answer is not used), clears the
/// `NiPointer`s at +0x70 to +0x78 and +0x7c to +0x84 and the pass pointer at +0x6c.
pub fn fn_00bca6c0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(SHADER_CALL_00BE0870, &args![base]);
    for offset in [0x70u32, 0x74, 0x78, 0x7c, 0x80, 0x84] {
        ni_pointer_clear(e, base + offset);
    }
    pass_pointer_clear(e, base + 0x6c);
    true
}

/// `00e7f7c0` (unnamed; `thiscall` on the render state object, no stack words; answers an
/// object), `00b79950` and `00b80600` (unnamed; `thiscall` on it, one word each), and the
/// texture stage calls `00e7de90` and `00e7eb00` (unnamed; `thiscall` on a stage, one word).
const RENDER_STATE_CALL_00E7F7C0: u32 = 0x00e7_f7c0;
const RENDER_STATE_SET_00B79950: u32 = 0x00b7_9950;
const RENDER_STATE_SET_00B80600: u32 = 0x00b8_0600;
const STAGE_CALL_00E7DE90: u32 = 0x00e7_de90;
const STAGE_CALL_00E7EB00: u32 = 0x00e7_eb00;

/// A texture stage held for the time of a set-up: the count at +0x60 goes up by one when the
/// stage is not null.
fn stage_hold(e: &mut Engine, stage: u32) {
    if stage != 0 {
        let count = e.mem.u32(stage + 0x60).wrapping_add(1);
        e.mem.set_u32(stage + 0x60, count);
    }
}

/// The shader object's slot `slot` (`PUSH 0; CALL`): the getters behind it (`00e95d50`, ...)
/// take no stack word (plain `RET`), so the pushed 0 stays on the stack and is the render
/// state's second word (`keep`) in the call after (`00e90e00`/`00e88870`, `RET 8`).
fn shader_of(e: &mut Engine, source: u32, slot: u32) -> u32 {
    e.vcall(source, slot, &args![]).u32()
}

// Translated from 00bca760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot +0x78), `thiscall`
/// with one stack word: the geometry's set-up record, whose word +0xc holds the property (bytes
/// +0x91, +0x92, +0x93, words +0x60, +0x64, +0x8c). Asks the render state object `0126f74c` for
/// an object (`00e7f7c0`). Without the byte +0x92: `00b79950` on the state with the vertex shader
/// +0x74 (byte +0x91 set) or +0x70, the object's slot +0x8c with the answer of slot +0x84 (with
/// 0) of the state's +0x5c, `00b80600` with the pixel shader +0x80 (word +0x64 set) or +0x7c, the
/// object's slot +0x7c with the answer of slot +0x7c (with 0) of the state's +0x44, and the
/// declaration +0x88; with the byte: the shaders +0x78 and +0x84, the same two slots, and the
/// declaration +0x8c. The shader's slot +0xc0 gets the declaration. Then the state's stages 0
/// and 1 (the array at its +0x24) are held; stage 0 gets `00e7de90` with 3 (byte +0x91 set) or
/// the word +0x8c, and, without the byte +0x93 and with a word +0x60, that word at +0x8 and
/// `00e7eb00(1)`; with a word +0x64, stage 1 gets `00e7de90(0)`, the word at +0x8 and
/// `00e7eb00(0)`, else its +0x8 is cleared when it is not null. Both stages are dropped (stage 1
/// first).
// Translation incomplete: 00bca760 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held stages) is not translated.
pub fn fn_00bca760(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let base = this.addr();
    let property = e.mem.u32(geometry.addr() + 0xc);
    let separate = e.mem.u8(property + 0x91);
    let second_texture = e.mem.u32(property + 0x64) != 0;
    let shared = e.mem.u8(property + 0x92);
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    let object = e.call(RENDER_STATE_CALL_00E7F7C0, &args![state]).u32();
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    let declaration = if shared == 0 {
        let vertex = if separate != 0 {
            e.mem.u32(base + 0x74)
        } else {
            e.mem.u32(base + 0x70)
        };
        e.call(RENDER_STATE_SET_00B79950, &args![state, vertex]);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let source = e.mem.u32(state + 0x5c);
        let value = shader_of(e, source, 0x84);
        e.vcall(object, 0x8c, &args![value, 0u32]);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let pixel = if second_texture {
            e.mem.u32(base + 0x80)
        } else {
            e.mem.u32(base + 0x7c)
        };
        e.call(RENDER_STATE_SET_00B80600, &args![state, pixel]);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let source = e.mem.u32(state + 0x44);
        let value = shader_of(e, source, 0x7c);
        e.vcall(object, 0x7c, &args![value, 0u32]);
        e.mem.u32(base + 0x88)
    } else {
        let vertex = e.mem.u32(base + 0x78);
        e.call(RENDER_STATE_SET_00B79950, &args![state, vertex]);
        let pixel = e.mem.u32(base + 0x84);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        e.call(RENDER_STATE_SET_00B80600, &args![state, pixel]);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let source = e.mem.u32(state + 0x5c);
        let value = shader_of(e, source, 0x84);
        e.vcall(object, 0x8c, &args![value, 0u32]);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let source = e.mem.u32(state + 0x44);
        let value = shader_of(e, source, 0x7c);
        e.vcall(object, 0x7c, &args![value, 0u32]);
        e.mem.u32(base + 0x8c)
    };
    e.vcall(base, 0xc0, &args![declaration]);
    let mode = if separate != 0 {
        3
    } else {
        e.mem.u32(property + 0x8c)
    };
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    let stages = e.mem.u32(state + 0x24);
    let first = e.mem.u32(stages);
    stage_hold(e, first);
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    let stages = e.mem.u32(state + 0x24);
    let second = e.mem.u32(stages + 4);
    stage_hold(e, second);
    e.call(STAGE_CALL_00E7DE90, &args![first, mode]);
    if e.mem.u8(property + 0x93) == 0 {
        let texture = e.mem.u32(property + 0x60);
        if texture != 0 {
            e.mem.set_u32(first + 8, texture);
            e.call(STAGE_CALL_00E7EB00, &args![first, 1u32]);
        }
    }
    let texture = e.mem.u32(property + 0x64);
    if texture != 0 {
        e.call(STAGE_CALL_00E7DE90, &args![second, 0u32]);
        e.mem.set_u32(second + 8, texture);
        e.call(STAGE_CALL_00E7EB00, &args![second, 0u32]);
    } else if second != 0 {
        e.mem.set_u32(second + 8, 0);
    }
    stage_release(e, second);
    stage_release(e, first);
}

/// The scene graph holder (`011f91e0`), the fallback for its word +0x20 (`011f4288`), the
/// scroll constants (`01202198` to `012021a4`), the setting enabling the viewport adjustment
/// (`011f9426`), the renderer (`011f4748`), the viewport `float`s (`011ad840` to `011ad84c`)
/// and the `float` 2^32 (`01099378`).
const SCENE_HOLDER_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_011F4288: u32 = 0x011f_4288;
const SCROLL_01202198: u32 = 0x0120_2198;
const ADJUST_011F9426: u32 = 0x011f_9426;
const RENDERER_011F4748: u32 = 0x011f_4748;
const VIEWPORT_011AD840: u32 = 0x011a_d840;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
/// `00e6fbb0` (unnamed; `thiscall` on the shader's +0x18, `RET 8`), the `_ftol2` helper
/// (`00ec62c0`, the value in `ST0`, a leading `f64` argument here), and the unnamed `cdecl`
/// state setters `00b984f0`, `00b97d20`, `00b98070`, `00b980c0` and `00b98180`.
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const FTOL: u32 = 0x00ec_62c0;
const STATE_CALL_00B984F0: u32 = 0x00b9_84f0;
const STATE_CALL_00B97D20: u32 = 0x00b9_7d20;
const STATE_CALL_00B98070: u32 = 0x00b9_8070;
const STATE_CALL_00B980C0: u32 = 0x00b9_80c0;
const STATE_CALL_00B98180: u32 = 0x00b9_8180;

/// An unsigned word as the x87 loads it: `FILD`, then 2^32 added when it is negative.
fn load_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)))
    } else {
        loaded
    }
}

// Translated from 00bca980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot +0x7c), `thiscall`
/// with one stack word: the geometry's set-up record (+0x8 an object with a `float` at +0x3c,
/// +0xc the property, +0x10 an object with flags at +0x18). Nothing without the scene holder
/// `011f91e0`. Else, with its object `O`: `00e6fbb0` on the shader's +0x18 with (`O` + 0x68, 0);
/// the property's `float`s +0x7c to +0x88 copied to `01202198` (byte +0x91 set) and its words
/// +0x68 to +0x74 to `01202188`, then `01202194` replaced by +0x78 times the `float` +0x3c of the
/// object at +0x8 (1.0 without one). The constant maps at +0x30 and +0x34, when set and the
/// state's +0x44 / +0x5c is set, get their slot +0x78 with eleven words. With the property's byte
/// +0xac: the words +0x9c to +0xa8 (the second and fourth recomputed from the viewport `float`s
/// and the renderer's slot +0xd4 targets' heights when `011f9426` is set and the renderer's slots
/// +0xcc and +0xc8 agree; `_ftol2`) go to `00b984f0(1, 0)` and `00b97d20`. With bit 0 of the
/// flags of +0x10's object: `00b98070(1, 0)`, `00b980c0` and `00b98180` with the fields of its
/// 16-bit flags and its words +0x1c, +0x20.
pub fn fn_00bca980(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let holder = e.global::<u32>(SCENE_HOLDER_011F91E0);
    if holder == 0 {
        return;
    }
    let base = this.addr();
    let record = geometry.addr();
    let scene = e.mem.u32(holder);
    let mut fallback = e.mem.u32(scene + 0x20);
    let anchor = scene + 0x68;
    if fallback == 0 {
        fallback = DEFAULT_011F4288;
    }
    let camera = e.mem.u32(scene + 0xb8);
    let camera_word = e.mem.u32(camera + 0x34);
    let scene_word = e.mem.u32(scene + 0xbc);
    let target = e.mem.u32(base + 0x18);
    let property = e.mem.u32(record + 0xc);
    e.call(RENDERER_CALL_00E6FBB0, &args![target, anchor, 0u32]);
    if e.mem.u8(property + 0x91) != 0 {
        for index in 0..4u32 {
            let value = x87_quiet_bits(e.mem.u32(property + 0x7c + 4 * index));
            e.set_global::<u32>(SCROLL_01202198 + 4 * index, value);
        }
    }
    for index in 0..4u32 {
        let value = e.mem.u32(property + 0x68 + 4 * index);
        e.set_global::<u32>(ONES_01202188 + 4 * index, value);
    }
    let mut scale = 1.0f32.to_bits();
    let source = e.mem.u32(record + 8);
    if source != 0 {
        scale = x87_quiet_bits(e.mem.u32(source + 0x3c));
    }
    let scaled = x87_store32(x87_mul(
        x87_load32(e.mem.u32(property + 0x78)),
        x87_m32(scale),
    ));
    e.set_global::<u32>(ONES_01202188 + 0xc, scaled);
    for (map_offset, state_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let shader = e.mem.u32(state + state_offset);
        if shader == 0 {
            continue;
        }
        e.vcall(
            map,
            0x78,
            &args![
                shader,
                scene,
                scene_word,
                0u32,
                camera_word,
                record,
                0u32,
                anchor,
                fallback,
                0u32,
                1u32
            ],
        );
    }
    if e.mem.u8(property + 0xac) != 0 {
        let left = e.mem.u32(property + 0x9c);
        let mut top = e.mem.u32(property + 0xa0);
        let right = e.mem.u32(property + 0xa4);
        let mut bottom = e.mem.u32(property + 0xa8);
        if e.global::<u8>(ADJUST_011F9426) != 0 {
            let renderer = e.global::<u32>(RENDERER_011F4748);
            let first = e.vcall(renderer, 0xcc, &args![]).u32();
            let second = e.vcall(renderer, 0xc8, &args![]).u32();
            if first == second {
                let low_bits = e.global::<u32>(VIEWPORT_011AD840 + 8);
                let high_bits = e.global::<u32>(VIEWPORT_011AD840 + 0xc);
                let renderer = e.global::<u32>(RENDERER_011F4748);
                let target = e.vcall(renderer, 0xd4, &args![]).u32();
                let height = e.mem.u32(target + 0xc);
                let renderer = e.global::<u32>(RENDERER_011F4748);
                let other = e.vcall(renderer, 0xd4, &args![]).u32();
                let low = x87_load32(low_bits);
                let high = x87_load32(high_bits);
                let span = x87_sub(low, high);
                let start = x87_mul(load_unsigned(e, height), high);
                let offset = x87_mul(f64::from(top as i32), span);
                top = e.call(FTOL, &args![x87_add(start, offset)]).u32();
                let other_height = e.mem.u32(other + 0xc);
                let start = x87_mul(high, load_unsigned(e, other_height));
                let offset = x87_mul(span, f64::from(bottom as i32));
                bottom = e.call(FTOL, &args![x87_add(offset, start)]).u32();
            }
        }
        e.call(STATE_CALL_00B984F0, &args![1u32, 0u32]);
        e.call(STATE_CALL_00B97D20, &args![left, top, right, bottom, 0u32]);
    }
    let flags_holder = e.mem.u32(record + 0x10);
    if flags_holder != 0 && e.mem.u8(flags_holder + 0x18) & 1 != 0 {
        e.call(STATE_CALL_00B98070, &args![1u32, 0u32]);
        let flags = u32::from(e.mem.u16(flags_holder + 0x18));
        e.call(
            STATE_CALL_00B980C0,
            &args![(flags >> 1) & 7, (flags >> 4) & 7, (flags >> 7) & 7, 0u32],
        );
        let second = e.mem.u32(flags_holder + 0x20);
        let flags = u32::from(e.mem.u16(flags_holder + 0x18));
        let first = e.mem.u32(flags_holder + 0x1c);
        e.call(
            STATE_CALL_00B98180,
            &args![flags >> 0xc, first, second, 0u32],
        );
    }
}

// Translated from 00bcac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot +0x8c), `thiscall`
/// with one stack word (the geometry's set-up record): `00b984f0(0, 0)` when the property's byte
/// +0xac is set and `00b98070(0, 0)` when the object at +0x10 has flags bit 0.
pub fn fn_00bcac60(e: &mut Engine, _this: Ptr, geometry: Ptr) {
    let record = geometry.addr();
    let property = e.mem.u32(record + 0xc);
    if e.mem.u8(property + 0xac) != 0 {
        e.call(STATE_CALL_00B984F0, &args![0u32, 0u32]);
    }
    let flags_holder = e.mem.u32(record + 0x10);
    if flags_holder != 0 && e.mem.u8(flags_holder + 0x18) & 1 != 0 {
        e.call(STATE_CALL_00B98070, &args![0u32, 0u32]);
    }
}

// Translated from 00bcae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bca070`; the vtable `010bb410`, slot 0): the shader's
/// scalar deleting destructor, `thiscall`, one stack word (the flags). Runs `00bca510`; with
/// bit 0 set, frees the 0x90 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bcae60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bca510(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x90u32]);
    }
    this
}

/// `00be08b0` (unit `bsshader.cpp` range, unnamed; `thiscall`, no stack words).
const SHADER_CALL_00BE08B0: u32 = 0x00be_08b0;

// Translated from 00bca070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bb4e0`), `thiscall`, no stack words. With the
/// word +0x28 set, calls the vtable +0xa0 of each of the three objects from +0x70 that is set;
/// then the vtable +0x88 of each of the three from +0x7c that is set; then jumps to `00be08b0`.
pub fn fn_00bca070(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        for index in 0..3 {
            let object = e.mem.u32(base + 0x70 + 4 * index);
            if object != 0 {
                e.vcall(object, 0xa0, &args![]);
            }
        }
    }
    for index in 0..3 {
        let object = e.mem.u32(base + 0x7c + 4 * index);
        if object != 0 {
            e.vcall(object, 0x88, &args![]);
        }
    }
    e.call(SHADER_CALL_00BE08B0, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_a070, fn_00bca070(Ptr)),
        entry!(0x00bc_aca0, fn_00bcaca0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bc_ae90, fn_00bcae90() -> Ptr),
        entry!(0x00bc_a050, fn_00bca050(Ptr, u32)),
        entry!(0x00bc_a0d0, fn_00bca0d0(Ptr)),
        entry!(0x00bc_a240, fn_00bca240(Ptr)),
        entry!(0x00bc_a3c0, fn_00bca3c0(Ptr) -> bool),
        entry!(0x00bc_a500, fn_00bca500(Ptr, u32)),
        entry!(0x00bc_a510, fn_00bca510(Ptr)),
        entry!(0x00bc_a6b0, fn_00bca6b0(Ptr) -> u32),
        entry!(0x00bc_a6c0, fn_00bca6c0(Ptr) -> bool),
        entry!(0x00bc_a760, fn_00bca760(Ptr, Ptr)),
        entry!(0x00bc_a980, fn_00bca980(Ptr, Ptr)),
        entry!(0x00bc_ac60, fn_00bcac60(Ptr, Ptr)),
        entry!(0x00bc_ae60, fn_00bcae60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bcae90_makes_two_declarations_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BCACA0, &[]);
        let (shader, log) = run(&mut e, 0x00bc_ae90);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (DECLARATION_CREATE, vec![RENDERER, 3, 1]),
                (declaration_slot(0x8c), vec![a, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![a, 1, 0, 5, 1, 0]),
                (declaration_slot(0xb4), vec![a]),
                (declaration_slot(0x8c), vec![b, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![b, 1, 0, 5, 1, 0]),
                (declaration_slot(0x8c), vec![b, 2, 0, 4, 3, 0]),
                (declaration_slot(0xb4), vec![b]),
                (NI_ALLOCATE, vec![0x90]),
                (SHADER_CONSTRUCT_00BCACA0, vec![SHADER, a, b]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x14c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, a]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bcaca0_builds_the_shader() {
        let mut e = rig();
        e.map(0x0120_2000, 0x1000);
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcc; 0x90]);
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_aca0, &args![this, first, second]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010BB410);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 1, DESTRUCT_00B79930),
                pointer_array(this + 0x70, 3, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 3, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u8(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x68), 0x14);
        assert_eq!(e.mem.u32(this + 0x88), first);
        assert_eq!(e.mem.u32(this + 0x8c), second);
        assert_eq!(e.mem.u32(second + 4), 1);
        for index in 0..4u32 {
            assert_eq!(e.global::<f32>(ONES_01202188 + 4 * index), 1.0);
            assert_eq!(e.global::<f32>(ONES_012021A8 + 4 * index), 1.0);
        }
        assert_eq!(e.global::<u32>(0x0120_2198), 0);
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bca050_stores_the_inverse() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global::<f32>(SOURCE_011AD83C, 4.0);
        e.call(0x00bc_a050, &args![0u32, 0u32]);
        assert_eq!(e.global::<f32>(ONES_012021A8), 0.25);
        e.set_global::<f32>(SOURCE_011AD83C, 0.0);
        e.call(0x00bc_a050, &args![0u32, 0u32]);
        assert_eq!(e.global::<f32>(ONES_012021A8), f32::INFINITY);
    }

    #[test]
    fn fn_00bca0d0_and_00bca240_compile_the_tile_shaders() {
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e77_0000u32.into_ret());
        let this = e.mem.alloc(0x90);
        e.call(0x00bc_a0d0, &args![this]);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 3);
        for (index, format) in formats.iter().enumerate() {
            assert_eq!(
                format[2..],
                [TILE_VERTEX_NAME_FORMAT_010BB3B0, index as u32]
            );
        }
        let empty = TILE_EMPTY_STRING_01011584;
        for (index, macros) in [
            [0u32; 8],
            [MACRO_TEXSCROLL_010BB3C0, empty, 0, 0, 0, 0, 0, 0],
            [MACRO_VC_010AED64, empty, 0, 0, 0, 0, 0, 0],
        ]
        .iter()
        .enumerate()
        {
            let seen = compiled(&e, e.mem.u32(this + 0x70 + 4 * index as u32));
            assert_eq!(seen.file, TILE_VERTEX_SOURCE_010BB3CC);
            assert_eq!(seen.profile, PROFILE_VS_1_1_010A93EC);
            assert!(!seen.pixel);
            assert_eq!(&seen.macros, macros);
        }
        assert!(calls(&e, PIXEL_PROFILE_00B4F380).is_empty());
        e.call(0x00bc_a240, &args![this]);
        assert_eq!(calls(&e, PIXEL_PROFILE_00B4F380), vec![vec![1]]);
        for index in 0..3u32 {
            let seen = compiled(&e, e.mem.u32(this + 0x7c + 4 * index));
            assert_eq!(seen.file, TILE_PIXEL_SOURCE_010BB3FC);
            assert_eq!(seen.profile, 0x0e77_0000);
            assert!(seen.pixel);
        }
        let alpha = compiled(&e, e.mem.u32(this + 0x80));
        assert_eq!(alpha.macros[..3], [MACRO_ALPHAMAP_010BB3F0, empty, 0]);
        assert_eq!(calls(&e, MEMSET).len(), 6);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats[5][2..], [TILE_PIXEL_NAME_FORMAT_010BB3E0, 2]);
    }

    #[test]
    fn fn_00bca3c0_makes_the_pass_with_two_stages() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x90);
        assert!(e.call(0x00bc_a3c0, &args![this]).bool());
        let pass = e.mem.u32(this + 0x6c);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x6c]]);
        let setups = calls(&e, SETUP_DEFAULT_STAGE);
        assert_eq!(setups[0][1..], [0, 1, 2, 1]);
        assert_eq!(setups[1][1..], [1, 1, 2, 1]);
        let set = calls(&e, PASS_SET_STAGE);
        assert_eq!(set.len(), 2);
        assert_eq!(set[1][..2], [pass, 1]);
        assert_eq!(calls(&e, SET_FILTER_MODE), vec![vec![set[0][2], 1]]);
        assert_eq!(render_states(&e, pass), vec![(7, 0), (0xe, 0), (0xa8, 0xf)]);
        // Only the second stage's reference is dropped.
        assert_eq!(e.mem.u32(set[0][2] + 0x60), 2);
        assert_eq!(e.mem.u32(set[1][2] + 0x60), 1);
    }

    #[test]
    fn fn_00bca500_stores_the_pass_and_00bca6b0_answers_the_constants() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x6c, 0x0e45_0000);
        e.call(0x00bc_a500, &args![this, 0u32]);
        assert_eq!(e.global::<u32>(RENDER_STATE_0126F74C), 0x0e45_0000);
        assert_eq!(e.call(0x00bc_a6b0, &args![this]).u32(), 0x0120_21b8);
    }

    /// A shader with six `NiPointer`s at +0x70 to +0x84 (count 1), a pass at +0x6c (count 1)
    /// and pointers at +0x88 (count 2) and +0x8c (count 1).
    fn shader(e: &mut Engine) -> (u32, Vec<u32>, u32) {
        let this = e.mem.alloc(0x90);
        let mut objects = vec![];
        for offset in (0x70u32..=0x8c).step_by(4) {
            let count = if offset == 0x88 { 2 } else { 1 };
            let object = object(e, count);
            e.mem.set_u32(this + offset, object);
            objects.push(object);
        }
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x6c, pass);
        (this, objects, pass)
    }

    #[test]
    fn fn_00bca510_destroys_the_members_and_keeps_the_memory() {
        let mut e = rig();
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCT_00EC5FCE, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        let (this, objects, pass) = shader(&mut e);
        e.mem.set_u8(this + 0x64, 1);
        e.mem.set_u32(this + 0x60, 7);
        e.call(0x00bc_a510, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BB410);
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 7);
        assert_eq!(e.mem.u32(objects[6] + 4), 1);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
    }

    #[test]
    fn fn_00bcae60_destroys_the_shader() {
        let mut e = rig();
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCT_00EC5FCE, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        let (this, objects, pass) = shader(&mut e);
        e.mem.set_u8(this + 0x64, 1);
        e.mem.set_u32(this + 0x60, 7);
        assert_eq!(e.call(0x00bc_ae60, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BB410);
        // The six arrays' objects and the one at +0x8c go; +0x88 keeps one reference.
        let mut deleted = objects[..6].to_vec();
        deleted.push(objects[7]);
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            deleted.iter().map(|o| vec![*o]).collect::<Vec<_>>()
        );
        assert_eq!(e.mem.u32(objects[6] + 4), 1);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![
                vec![this + 0x7c, 4, 3, DESTRUCT_00B66F20],
                vec![this + 0x70, 4, 3, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 1, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x90]]);
        e.call(0x00bc_ae60, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bca6c0_clears_the_shaders_and_the_pass() {
        let mut e = rig();
        e.register(SHADER_CALL_00BE0870, |_, _| Ret::default());
        let (this, objects, pass) = shader(&mut e);
        assert!(e.call(0x00bc_a6c0, &args![this]).bool());
        assert_eq!(calls(&e, SHADER_CALL_00BE0870), vec![vec![this]]);
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 6);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        for offset in (0x6cu32..=0x84).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x88), objects[6]);
    }

    const STATE_VTABLE: u32 = 0x0e30_0000;
    const STATE_SLOTS: u32 = 0x0e31_0000;

    /// A render state object at `0126f74c` with source objects at +0x44 and +0x5c (their slots
    /// +0x7c and +0x84 answer 0x44 and 0x5c), a stage array at +0x24 with two stages, and the
    /// object `00e7f7c0` answers (same vtable).
    fn state_rig() -> (Engine, u32, [u32; 2]) {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let slots: Vec<u32> = (0..0x40u32).map(|i| STATE_SLOTS + 4 * i).collect();
        e.put_vtable(STATE_VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(STATE_SLOTS + 0x7c, |_, _| 0x44u32.into_ret());
        e.register(STATE_SLOTS + 0x84, |_, _| 0x5cu32.into_ret());
        let state = e.mem.alloc(0x60);
        e.set_global::<u32>(RENDER_STATE_0126F74C, state);
        for offset in [0x44u32, 0x5c] {
            let source = e.mem.alloc(8);
            e.mem.set_u32(source, STATE_VTABLE);
            e.mem.set_u32(state + offset, source);
        }
        let stages = e.mem.alloc(8);
        e.mem.set_u32(state + 0x24, stages);
        let mut made = [0u32; 2];
        for (index, stage) in made.iter_mut().enumerate() {
            *stage = e.mem.alloc(0x70);
            e.mem.set_u32(*stage + 0x60, 1);
            e.mem.set_u32(stages + 4 * index as u32, *stage);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, STATE_VTABLE);
        e.register_double(RENDER_STATE_CALL_00E7F7C0, move |_, _| object.into_ret());
        for target in [
            RENDER_STATE_SET_00B79950,
            RENDER_STATE_SET_00B80600,
            STAGE_CALL_00E7DE90,
            STAGE_CALL_00E7EB00,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        (e, state, made)
    }

    /// A shader with the test vtable (slot +0xc0 recorded) and words 0x70 + offset at +0x70 to
    /// +0x8c.
    fn state_shader(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this, STATE_VTABLE);
        for offset in (0x70u32..=0x8c).step_by(4) {
            e.mem.set_u32(this + offset, 0x0e50_0000 + offset);
        }
        this
    }

    fn record(e: &mut Engine, property: u32) -> u32 {
        let record = e.mem.alloc(0x14);
        e.mem.set_u32(record + 0xc, property);
        record
    }

    #[test]
    fn fn_00bca760_sets_the_shaders_and_the_stages() {
        let (mut e, state, [first, second]) = state_rig();
        let this = state_shader(&mut e);
        let property = e.mem.alloc(0xb0);
        e.mem.set_u32(property + 0x8c, 6);
        e.mem.set_u32(property + 0x60, 0x0e61_0000);
        e.mem.set_u32(property + 0x64, 0x0e62_0000);
        let record = record(&mut e, property);
        e.call(0x00bc_a760, &args![this, record]);
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00B79950),
            vec![vec![state, 0x0e50_0070]]
        );
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00B80600),
            vec![vec![state, 0x0e50_0080]]
        );
        assert_eq!(calls(&e, STATE_SLOTS + 0x8c)[0][1], 0x5c);
        assert_eq!(
            calls(&e, STATE_SLOTS + 0x8c)[0][2],
            0,
            "the 0 left on the stack"
        );
        assert_eq!(
            calls(&e, STATE_SLOTS + 0x7c)[0],
            vec![e.mem.u32(state + 0x44)]
        );
        assert_eq!(calls(&e, STATE_SLOTS + 0x7c)[1][1..], [0x44, 0]);
        assert_eq!(calls(&e, STATE_SLOTS + 0xc0), vec![vec![this, 0x0e50_0088]]);
        assert_eq!(
            calls(&e, STAGE_CALL_00E7DE90),
            vec![vec![first, 6], vec![second, 0]]
        );
        assert_eq!(
            calls(&e, STAGE_CALL_00E7EB00),
            vec![vec![first, 1], vec![second, 0]]
        );
        assert_eq!(e.mem.u32(first + 8), 0x0e61_0000);
        assert_eq!(e.mem.u32(second + 8), 0x0e62_0000);
        assert_eq!(e.mem.u32(first + 0x60), 1);
        assert_eq!(e.mem.u32(second + 0x60), 1);
    }

    #[test]
    fn fn_00bca760_shared_and_separate_shaders() {
        let (mut e, state, [first, second]) = state_rig();
        let this = state_shader(&mut e);
        let property = e.mem.alloc(0xb0);
        e.mem.set_u8(property + 0x91, 1);
        e.mem.set_u8(property + 0x93, 1);
        e.mem.set_u32(property + 0x60, 0x0e61_0000);
        e.mem.set_u32(second + 8, 0x77);
        let record = record(&mut e, property);
        e.call(0x00bc_a760, &args![this, record]);
        assert_eq!(calls(&e, RENDER_STATE_SET_00B79950)[0][1], 0x0e50_0074);
        assert_eq!(calls(&e, RENDER_STATE_SET_00B80600)[0][1], 0x0e50_007c);
        assert_eq!(calls(&e, STAGE_CALL_00E7DE90), vec![vec![first, 3]]);
        assert!(calls(&e, STAGE_CALL_00E7EB00).is_empty());
        assert_eq!(e.mem.u32(second + 8), 0);
        // The byte +0x92: the shaders +0x78, +0x84 and the declaration +0x8c.
        e.mem.set_u8(property + 0x92, 1);
        e.call(0x00bc_a760, &args![this, record]);
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00B79950)[1],
            vec![state, 0x0e50_0078]
        );
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00B80600)[1],
            vec![state, 0x0e50_0084]
        );
        assert_eq!(calls(&e, STATE_SLOTS + 0xc0)[1], vec![this, 0x0e50_008c]);
    }

    fn scene_rig() -> (Engine, u32, u32, u32) {
        let (mut e, _, _) = state_rig();
        for page in [
            0x011f_9000u32,
            0x011f_4000,
            0x0120_2000,
            0x011a_d000,
            0x0109_9000,
        ] {
            e.map(page, 0x1000);
        }
        for target in [
            RENDERER_CALL_00E6FBB0,
            STATE_CALL_00B984F0,
            STATE_CALL_00B97D20,
            STATE_CALL_00B98070,
            STATE_CALL_00B980C0,
            STATE_CALL_00B98180,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32 as u32).into_ret()
        });
        let scene = e.mem.alloc(0xc0);
        let camera = e.mem.alloc(0x38);
        e.mem.set_u32(scene + 0xb8, camera);
        e.mem.set_u32(camera + 0x34, 0x34);
        e.mem.set_u32(scene + 0xbc, 0xbc);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, scene);
        e.set_global::<u32>(SCENE_HOLDER_011F91E0, holder);
        let this = state_shader(&mut e);
        e.mem.set_u32(this + 0x18, 0x0e18_0000);
        let property = e.mem.alloc(0xb0);
        (e, this, scene, property)
    }

    #[test]
    fn fn_00bca980_sets_the_constants_and_the_maps() {
        let (mut e, this, scene, property) = scene_rig();
        for (index, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0].iter().enumerate() {
            e.mem.set_f32(property + 0x68 + 4 * index as u32, *value);
        }
        e.mem.set_u8(property + 0x91, 1);
        e.mem.set_f32(property + 0x7c, 9.0);
        let record = record(&mut e, property);
        let source = e.mem.alloc(0x40);
        e.mem.set_f32(source + 0x3c, 0.5);
        e.mem.set_u32(record + 8, source);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, STATE_VTABLE);
        e.mem.set_u32(this + 0x34, map);
        e.call(0x00bc_a980, &args![this, record]);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0),
            vec![vec![0x0e18_0000, scene + 0x68, 0]]
        );
        assert_eq!(e.global::<f32>(SCROLL_01202198), 9.0);
        assert_eq!(e.global::<f32>(ONES_01202188), 1.0);
        assert_eq!(e.global::<f32>(ONES_01202188 + 8), 3.0);
        assert_eq!(e.global::<f32>(ONES_01202188 + 0xc), 2.5);
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let shader = e.mem.u32(state + 0x5c);
        assert_eq!(
            calls(&e, STATE_SLOTS + 0x78),
            vec![vec![
                map,
                shader,
                scene,
                0xbc,
                0,
                0x34,
                record,
                0,
                scene + 0x68,
                DEFAULT_011F4288,
                0,
                1
            ]]
        );
        assert!(calls(&e, STATE_CALL_00B984F0).is_empty());
        assert!(calls(&e, STATE_CALL_00B98070).is_empty());
        // Nothing without the holder.
        e.set_global::<u32>(SCENE_HOLDER_011F91E0, 0);
        e.call(0x00bc_a980, &args![this, record]);
        assert_eq!(calls(&e, RENDERER_CALL_00E6FBB0).len(), 1);
    }

    #[test]
    fn fn_00bca980_scissor_and_flags() {
        let (mut e, this, _, property) = scene_rig();
        e.mem.set_u8(property + 0xac, 1);
        for (index, value) in [10u32, 20, 30, 40].iter().enumerate() {
            e.mem.set_u32(property + 0x9c + 4 * index as u32, *value);
        }
        let record = record(&mut e, property);
        let flags = e.mem.alloc(0x24);
        // Fields 1 (bits 1-3), 2 (bits 4-6), 3 (bits 7-9), 5 (bits 12-15) and bit 0.
        e.mem
            .set_u16(flags + 0x18, 1 | (1 << 1) | (2 << 4) | (3 << 7) | (5 << 12));
        e.mem.set_u32(flags + 0x1c, 0x1c);
        e.mem.set_u32(flags + 0x20, 0x20);
        e.mem.set_u32(record + 0x10, flags);
        e.call(0x00bc_a980, &args![this, record]);
        assert_eq!(calls(&e, STATE_CALL_00B984F0), vec![vec![1, 0]]);
        assert_eq!(
            calls(&e, STATE_CALL_00B97D20),
            vec![vec![10, 20, 30, 40, 0]]
        );
        assert_eq!(calls(&e, STATE_CALL_00B98070), vec![vec![1, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B980C0), vec![vec![1, 2, 3, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B98180), vec![vec![5, 0x1c, 0x20, 0]]);
        // The viewport adjustment: two targets of heights 100 and 200, viewport 0.25 .. 0.75.
        e.set_global::<u8>(ADJUST_011F9426, 1);
        e.set_global::<f32>(VIEWPORT_011AD840 + 8, 0.75);
        e.set_global::<f32>(VIEWPORT_011AD840 + 0xc, 0.25);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, STATE_VTABLE);
        e.set_global::<u32>(RENDERER_011F4748, renderer);
        let mut heights = vec![200u32, 100];
        e.register_double(STATE_SLOTS + 0xd4, move |e, _| {
            let target = e.mem.alloc(0x10);
            e.mem.set_u32(target + 0xc, heights.pop().unwrap());
            target.into_ret()
        });
        e.call(0x00bc_a980, &args![this, record]);
        // 100 * 0.25 + 20 * 0.5 = 35; 0.25 * 200 + 0.5 * 40 = 70.
        assert_eq!(calls(&e, STATE_CALL_00B97D20)[1], vec![10, 35, 30, 70, 0]);
    }

    #[test]
    fn fn_00bcac60_undoes_the_states() {
        let mut e = rig();
        for target in [STATE_CALL_00B984F0, STATE_CALL_00B98070] {
            e.register(target, |_, _| Ret::default());
        }
        let property = e.mem.alloc(0xb0);
        let record = record(&mut e, property);
        e.call(0x00bc_ac60, &args![0u32, record]);
        assert!(calls(&e, STATE_CALL_00B984F0).is_empty());
        e.mem.set_u8(property + 0xac, 1);
        let flags = e.mem.alloc(0x20);
        e.mem.set_u8(flags + 0x18, 1);
        e.mem.set_u32(record + 0x10, flags);
        e.call(0x00bc_ac60, &args![0u32, record]);
        assert_eq!(calls(&e, STATE_CALL_00B984F0), vec![vec![0, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B98070), vec![vec![0, 0]]);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{
        engine, order, slot_target, vobject,
    };

    #[test]
    fn fn_00bca070_updates_the_set_objects() {
        let mut e = engine(&[SHADER_CALL_00BE08B0]);
        let this = e.mem.alloc(0x90);
        let a = vobject(&mut e, 8, &[]);
        let b = vobject(&mut e, 8, &[]);
        e.mem.set_u32(this + 0x74, a);
        e.mem.set_u32(this + 0x84, b);
        e.call(0x00bc_a070, &args![this]);
        assert_eq!(
            order(&e),
            vec![0x00bc_a070, slot_target(&e, b, 0x88), SHADER_CALL_00BE08B0]
        );
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bc_a070, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bc_a070,
                slot_target(&e, a, 0xa0),
                slot_target(&e, b, 0x88),
                SHADER_CALL_00BE08B0
            ]
        );
    }
}
