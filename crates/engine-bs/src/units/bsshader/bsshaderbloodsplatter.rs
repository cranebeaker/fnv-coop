//! `BSShader/bsshaderbloodsplatter.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderBloodSplatter::BSShaderBloodSplatter` (`00b80d30`), `::LoadShaders`
/// (`00b80090`) and `::LoadStagesAndPasses` (`00b807d0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_0d30;
const LOAD_SHADERS: u32 = 0x00b8_0090;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_07d0;

// Translated from 00b80e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 7. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 2 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)) and calls slot +0xb4. Then
/// allocates 0x84 bytes and runs the constructor (`00b80d30`; a null allocation is used as it
/// is), calls the shader's slot +0x4c, `LoadShaders` (`00b80090`), `LoadStagesAndPasses`
/// (`00b807d0`), slot +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00b80e70 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_blood_splatter_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x84u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The blood splatter shader's vtable (`010af760`; no RTTI locator before it).
const BLOOD_SPLATTER_VTABLE_010AF760: u32 = 0x010a_f760;
/// The two `float`s the constructor loads (`010177e8`, `0101e2bc`) and the four it stores
/// (`011fed94`, `011fed98` the first, `011fed9c` 0.0, `011feda0` the second).
const VALUE_010177E8: u32 = 0x0101_77e8;
const VALUE_0101E2BC: u32 = 0x0101_e2bc;
const FLOATS_011FED94: u32 = 0x011f_ed94;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`: file name, load byte, the
/// `NiPointer` to fill, mipmap byte, cube byte), the texture name it is given (`010af89c`)
/// and the `NiPointer` it fills (`011fedc4`).
const GET_TEXTURE: u32 = 0x00b5_5840;
const TEXTURE_NAME_010AF89C: u32 = 0x010a_f89c;
const TEXTURE_POINTER_011FEDC4: u32 = 0x011f_edc4;

// Translated from 00b80d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::BSShaderBloodSplatter` (Xbox PDB), `thiscall`, no stack words.
/// Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010af760`, constructs the
/// `NiPointer` arrays at +0x6c (2, destructor `00b79930`), +0x74 and +0x7c (2 each,
/// destructor `00b66f20`), sets +0x68 to 0x17, stores the `float` at `010177e8` (through the
/// FPU, a signalling NaN quieted) in `011fed94` and `011fed98`, 0.0 in `011fed9c` and the
/// `float` at `0101e2bc` in `011feda0`, then has `GetTexture` (`00b55840`) fill the pointer
/// at `011fedc4` with the texture named at `010af89c` (load byte 1, mipmap byte 1, not a cube
/// map). Answers `this`.
// Translation incomplete: 00b80d30 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn bs_shader_blood_splatter_bs_shader_blood_splatter(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BLOOD_SPLATTER_VTABLE_010AF760);
    construct_pointer_array(e, base + 0x6c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x74, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 2, DESTRUCT_00B66F20);
    let first = x87_quiet_bits(e.global::<u32>(VALUE_010177E8));
    let second = x87_quiet_bits(e.global::<u32>(VALUE_0101E2BC));
    e.mem.set_u32(base + 0x68, 0x17);
    e.set_global::<u32>(FLOATS_011FED94, first);
    e.set_global::<u32>(FLOATS_011FED94 + 4, first);
    e.set_global::<f32>(FLOATS_011FED94 + 8, 0.0);
    e.set_global::<u32>(FLOATS_011FED94 + 0xc, second);
    e.call(
        GET_TEXTURE,
        &args![
            TEXTURE_NAME_010AF89C,
            1u32,
            TEXTURE_POINTER_011FEDC4,
            1u32,
            0u32
        ],
    );
    this
}
/// Strings of `LoadShaders` (read in Ghidra): `"BloodSplatter\\BloodSplatter.v.hlsl"`
/// (`010af738`), `"BloodSplatter\\BloodSplatter.p.hlsl"` (`010af714`), the macro `"FLARE"`
/// (`010af70c`) with the value `""` (`01011584`), the compiled names `"BS.vso"`, `"BSF.vso"`,
/// `"BS.pso"`, `"BSF.pso"` and the profiles `"vs_1_1"` (`010a93ec`), `"ps_2_a"` (`010a93c4`).
const VERTEX_SOURCE_010AF738: u32 = 0x010a_f738;
const PIXEL_SOURCE_010AF714: u32 = 0x010a_f714;
const MACRO_FLARE_010AF70C: u32 = 0x010a_f70c;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const OUTPUT_BS_VSO_010AF704: u32 = 0x010a_f704;
const OUTPUT_BSF_VSO_010AF6FC: u32 = 0x010a_f6fc;
const OUTPUT_BS_PSO_010AF6F4: u32 = 0x010a_f6f4;
const OUTPUT_BSF_PSO_010AF6EC: u32 = 0x010a_f6ec;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;

/// The source blocks of `LoadShaders`' frame (0x34c bytes up to the return address), in the order the exe fills them.
const LOAD_SHADERS_SOURCES: [ShaderSource; 4] = [
    ShaderSource {
        offset: 0xa8,
        words: &[VERTEX_SOURCE_010AF738, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0xf4,
        words: &[PIXEL_SOURCE_010AF714, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x10,
        words: &[
            VERTEX_SOURCE_010AF738,
            MACRO_FLARE_010AF70C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x5c,
        words: &[
            PIXEL_SOURCE_010AF714,
            MACRO_FLARE_010AF70C,
            EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
];
/// The compiled-name and path buffers of that frame.
const LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x140,
    path: 0x244,
};
/// The compile steps, in the exe's order.
const LOAD_SHADERS_STEPS: [ShaderCompile; 4] = [
    ShaderCompile {
        source: 0xa8,
        output: OUTPUT_BS_VSO_010AF704,
        profile: PROFILE_VS_1_1_010A93EC,
        pixel: false,
        slot: 0x74,
    },
    ShaderCompile {
        source: 0x10,
        output: OUTPUT_BSF_VSO_010AF6FC,
        profile: PROFILE_VS_1_1_010A93EC,
        pixel: false,
        slot: 0x78,
    },
    ShaderCompile {
        source: 0xf4,
        output: OUTPUT_BS_PSO_010AF6F4,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x7c,
    },
    ShaderCompile {
        source: 0x5c,
        output: OUTPUT_BSF_PSO_010AF6EC,
        profile: PROFILE_PS_2_A_010A93C4,
        pixel: true,
        slot: 0x80,
    },
];

// Translated from 00b80090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills four
/// source blocks on its stack (the vertex and pixel HLSL files, each without macros and with
/// the macro `FLARE` = `""`; the rest of each block cleared by `memset`), then compiles in
/// order: the plain vertex source as `"BS.vso"` (`vs_1_1`) into the `NiPointer` at +0x74, the
/// `FLARE` vertex source as `"BSF.vso"` into +0x78, the plain pixel source as `"BS.pso"`
/// (`ps_2_a`) into +0x7c and the `FLARE` pixel source as `"BSF.pso"` into +0x80. Each step:
/// `BSShader::MakePath` (`00be0c00`) of the file into the path buffer, the compiled name copied
/// by `00406d00`, `CompileVertexShaderHLSL` (`00be0fe0`) or `CompilePixelShaderHLSL`
/// (`00be1750`), the result assigned to the pointer.
// Translation incomplete: 00b80090 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_shader_blood_splatter_load_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0x34c, |e, frame| {
        set_up_sources(e, frame.addr(), &LOAD_SHADERS_SOURCES);
        compile_shaders(
            e,
            this.addr(),
            frame.addr(),
            &LOAD_SHADERS_BUFFERS,
            &LOAD_SHADERS_STEPS,
        );
    });
}
// Translated from 00b807d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words;
/// answers 1. Two passes, at +0x6c and +0x70: each gets a new pass (`00e804d0`), two default
/// stages (`SetupDefaultStage` with 3, 2, 1, filter mode 1), the render states 0x1b = 1,
/// 7 = 0, 0xe = 0, then 0x13 and 0x14 (1 and 3 for the first pass, 2 and 2 for the second;
/// the last one through the pass read for the one before), and its vertex shader (+0x5c) and
/// pixel shader (+0x44) `NiPointer`s assigned from the loader's shaders: +0x74 and +0x7c for
/// the first pass, +0x78 and +0x80 for the second.
// Translation incomplete: 00b807d0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage pointers) is not translated.
pub fn bs_shader_blood_splatter_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for (pass_offset, last_states, shaders) in [
        (0x6cu32, [1u32, 3], [0x74u32, 0x7c]),
        (0x70, [2, 2], [0x78, 0x80]),
    ] {
        let slot = base + pass_offset;
        e.call(NEW_PASS_00E804D0, &args![slot]);
        add_default_stages(e, slot, 2, [3, 2, 1], 1);
        for (state, value) in [(0x1bu32, 1u32), (7, 0), (0xe, 0)] {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0x13, last_states[0]);
        pass_render_state(e, pass, 0x14, last_states[1]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + shaders[0]);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + shaders[1]);
        ni_pointer_assign(e, pass + 0x44, pixel);
    }
    true
}

// Translated from 00b80600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`): a pass's pixel shader setter,
/// `thiscall` with one stack word (the shader): `NiPointer` assignment of the shader to the
/// pass's +0x44 (the old shader released, `InterlockedDecrement` on its +4 and deleted through
/// vtable slot +4 at 0; the new one held), nothing when it already holds it.
pub fn fn_00b80600(e: &mut Engine, this: Ptr, shader: Ptr) {
    ni_pointer_assign(e, this.addr() + 0x44, shader.addr());
}

/// The object the splatter direction test runs on (`011f91c8`), its test `00b5bca0` (unit
/// `shadowscenenode.cpp`, `thiscall`, `RET 0xc`: a point, a direction, the out point), the
/// projection `00a6fdb0` (unit `nicamera.cpp`, `thiscall`, `RET 0x14`: the point, three out
/// `float`s, a `float`), the `float` given to it (`010718c0`), the normalize `004a0c10`
/// (`thiscall` on a point), `2.0` (`double`, `01011590`) and the constants written (`011fed90`
/// the dot product, then `011fed94` to `011feda0`).
const SPLATTER_OBJECT_011F91C8: u32 = 0x011f_91c8;
const SPLATTER_TEST_00B5BCA0: u32 = 0x00b5_bca0;
const PROJECT_00A6FDB0: u32 = 0x00a6_fdb0;
const PROJECT_VALUE_010718C0: u32 = 0x0107_18c0;
const NORMALIZE_004A0C10: u32 = 0x004a_0c10;
const TWO_01011590: u32 = 0x0101_1590;
const DOT_011FED90: u32 = 0x011f_ed90;

// Translated from 00b803b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`), `cdecl`, one word (the
/// geometry). Stores 0.0 at `011fed90`; runs `00b5bca0` on the object at `011f91c8` with the
/// geometry's world translation (+0x8c), the first column of its world rotation (+0x68, +0x74,
/// +0x80, copied to the stack) and an out point. When that answers false the constants
/// `011fed94` to `011fed9c` become 0.0 and `011feda0` the `float` at `0101e2bc`. Otherwise:
/// the column (copied again) and the out point minus the translation are normalized
/// (`004a0c10`), their dot product goes to `011fed90`, and the projection `00a6fdb0` on the
/// geometry (the out point, three out `float`s, the first of them the argument's own stack
/// word, and the `float` at `010718c0`) runs; when it answers false the constants are set as
/// above, otherwise the first two outputs are clamped to [0, 1] (a NaN kept), and `011fed94`
/// to `011feda0` get twice the first minus 1, twice the second minus 1, the third and the
/// `float` at `0101e2bc`.
pub fn fn_00b803b0(e: &mut Engine, geometry: Ptr) {
    let geometry = geometry.addr();
    e.set_global::<u32>(DOT_011FED90, 0);
    // The frame from the locals to the argument word: L (second output), L+4 (third), L+8
    // (the difference), L+0x14 (the column), L+0x20 (the out point), L+0x34 (the argument).
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        let second = frame;
        let third = frame + 4;
        let difference = frame + 8;
        let column = frame + 0x14;
        let point = frame + 0x20;
        let first = frame + 0x34;
        e.mem.set_u32(first, geometry);
        let copy_column = |e: &mut Engine| {
            for (index, offset) in [0x68u32, 0x74, 0x80].iter().enumerate() {
                let value = x87_quiet_bits(e.mem.u32(geometry + offset));
                e.mem.set_u32(column + 4 * index as u32, value);
            }
        };
        copy_column(e);
        let object = e.global::<u32>(SPLATTER_OBJECT_011F91C8);
        let translation = geometry + 0x8c;
        let hit = e
            .call(
                SPLATTER_TEST_00B5BCA0,
                &args![object, translation, column, point],
            )
            .bool();
        let rest_constants = |e: &mut Engine| {
            for word in 0..3u32 {
                e.set_global::<u32>(FLOATS_011FED94 + 4 * word, 0);
            }
            let rest = x87_quiet_bits(e.global::<u32>(VALUE_0101E2BC));
            e.set_global::<u32>(FLOATS_011FED94 + 0xc, rest);
        };
        if !hit {
            rest_constants(e);
            return;
        }
        copy_column(e);
        for word in 0..3u32 {
            let value = x87_sub(
                x87_load32(e.mem.u32(point + 4 * word)),
                x87_m32(e.mem.u32(translation + 4 * word)),
            );
            e.mem.set_u32(difference + 4 * word, x87_store32(value));
        }
        e.call(NORMALIZE_004A0C10, &args![column]);
        e.call(NORMALIZE_004A0C10, &args![difference]);
        let product = |e: &Engine, word: u32| {
            x87_mul(
                x87_load32(e.mem.u32(difference + 4 * word)),
                x87_m32(e.mem.u32(column + 4 * word)),
            )
        };
        let y = product(e, 1);
        let x = product(e, 0);
        let sum = x87_add(y, x);
        let z = product(e, 2);
        let dot = x87_add(sum, z);
        e.set_global::<u32>(DOT_011FED90, x87_store32(dot));
        let value = x87_quiet_bits(e.global::<u32>(PROJECT_VALUE_010718C0));
        let projected = e
            .call(
                PROJECT_00A6FDB0,
                &args![geometry, point, first, second, third, value],
            )
            .bool();
        if !projected {
            rest_constants(e);
            return;
        }
        // FCOM against 0.0 and then 1.0 (TEST AH,5 / AH,0x41): below 0 becomes 0.0, above 1
        // becomes 1.0, a NaN is left as it is.
        let a = x87_load32(e.mem.u32(first));
        if a < 0.0 {
            e.mem.set_u32(first, 0);
        } else if 1.0 < a {
            e.mem.set_u32(first, ONE_BITS);
        }
        let b = x87_load32(e.mem.u32(second));
        if b < 0.0 {
            e.mem.set_u32(second, 0);
        } else if b > 1.0 {
            e.mem.set_u32(second, ONE_BITS);
        }
        let a = x87_load32(e.mem.u32(first));
        let two = x87_load64(e.global::<u64>(TWO_01011590));
        // FMUL ST(1),ST(0) (`DC C9`), then FSUB ST(2),ST(0) (`DC EA`) with 1.0.
        let first_constant = x87_sub(x87_mul(a, two), 1.0);
        e.set_global::<u32>(FLOATS_011FED94, x87_store32(first_constant));
        // FMUL by the second output, then FSUBRP (`DE E1`): the product minus 1.0.
        let second_constant = x87_sub(x87_mul(two, x87_m32(e.mem.u32(second))), 1.0);
        e.set_global::<u32>(FLOATS_011FED94 + 4, x87_store32(second_constant));
        let third_value = x87_quiet_bits(e.mem.u32(third));
        e.set_global::<u32>(FLOATS_011FED94 + 8, third_value);
        let rest = x87_quiet_bits(e.global::<u32>(VALUE_0101E2BC));
        e.set_global::<u32>(FLOATS_011FED94 + 0xc, rest);
    });
}

/// `BSShader::~BSShader` (engine map, `00be0f10`, no stack words), the `eh vector destructor
/// iterator` (`00ec5fce`, `stdcall`) and the Ni free (`00aa1460`, `cdecl`: block, size).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
/// Bits of `1.0f`.
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b80650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`): the blood splatter shader's
/// destructor, no stack words. Stores the vtable `010af760`, drops and clears the passes at
/// +0x6c and +0x70, clears (releasing what they hold) the `NiPointer`s +0x74, +0x78, +0x7c,
/// +0x80 and the texture at `011fedc4`, destroys the arrays +0x7c and +0x74 (2 each,
/// destructor `00b66f20`) and +0x6c (2, `00b79930`) and runs `BSShader::~BSShader`
/// (`00be0f10`).
// Translation incomplete: 00b80650 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the arrays and the base) is not translated.
pub fn fn_00b80650(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BLOOD_SPLATTER_VTABLE_010AF760);
    for index in 0..2u32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x74 + 4 * index);
    }
    ni_pointer_clear(e, TEXTURE_POINTER_011FEDC4);
    for (offset, destructor) in [
        (0x7cu32, DESTRUCT_00B66F20),
        (0x74, DESTRUCT_00B66F20),
        (0x6c, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, 2u32, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00b807c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (the blood splatter vtable `010af760` slot +0x118, and many other
/// shader vtables' slot +0x118): answers the word +0x28 (the declaration `NiPointer`).
/// `thiscall`, `RET 8`; neither stack word is read.
pub fn fn_00b807c0(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    e.mem.u32(this.addr() + 0x28)
}

// Translated from 00b80e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`): the scalar deleting
/// destructor (vtable `010af760` slot +0), `thiscall`, `RET 4`. Runs the destructor
/// (`00b80650`) and, when bit 0 of the flags is set, frees the 0x84 bytes (`00aa1460`).
/// Answers `this`.
pub fn fn_00b80e40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b80650(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x84u32]);
    }
    this
}

/// The constant names of `CreateConstantMaps` (read in Ghidra) and the data outside the
/// shader they point at.
const ALPHA_0103A454: u32 = 0x0103_a454;
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const LIGHTLOC_010AF8C4: u32 = 0x010a_f8c4;
const CTRL_010AF8BC: u32 = 0x010a_f8bc;
const PIXEL_DATA_011FEDB4: u32 = 0x011f_edb4;
const VERTEX_DATA_011FEDA4: u32 = 0x011f_eda4;

// Translated from 00b80f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::CreateConstantMaps` (Xbox PDB), no stack words. When the pixel
/// constant map (+0x30) is null, makes one (kind 2) and adds `"alpha"`
/// (0x10000007, register 0, `011fedb4`); when the vertex one (+0x34) is null, makes one (kind
/// 0) and adds `"WorldViewProjTranspose"` (0x20000009, 0, 0, 4), `"lightloc"` (register 4,
/// `011fed94`) and `"ctrl"` (register 5, `011feda4`).
// Translation incomplete: 00b80f50 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn bs_shader_blood_splatter_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vector = |name: u32, register: u32, data: u32| {
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
    };
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
        add_constant_entry(
            e,
            base,
            0x30,
            vector(ALPHA_0103A454, 0, PIXEL_DATA_011FEDB4),
        );
    }
    if e.mem.u32(base + 0x34) == 0 {
        make_constant_map(e, base, 0x34, 0);
        add_constant_entry(
            e,
            base,
            0x34,
            [
                WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                0,
                4,
                0,
                0,
                0,
                0,
                0,
            ],
        );
        add_constant_entry(e, base, 0x34, vector(LIGHTLOC_010AF8C4, 4, FLOATS_011FED94));
        add_constant_entry(
            e,
            base,
            0x34,
            vector(CTRL_010AF8BC, 5, VERTEX_DATA_011FEDA4),
        );
    }
}

// Translated from 00b80380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`), `thiscall`, no stack words:
/// gives each of the two passes at +0x6c and +0x70 to `this`'s vtable slot +0x128.
pub fn fn_00b80380(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x6cu32, 0x70] {
        let pass = e.mem.u32(base + offset);
        e.vcall(base, 0x128, &args![pass]);
    }
}

/// The current pass word (`0126f74c`) and the pass kind being set up (`011f91e4`; 0x2f0 and
/// 0x2f1 are the blood splatter's two).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const PASS_KIND_011F91E4: u32 = 0x011f_91e4;

// Translated from 00b805d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 60 of the vtable `010af760`, `thiscall`, one
/// word (the pass kind): kind 0x2f0 makes the pass at +0x70 current (`0126f74c`), kind 0x2f1
/// the pass at +0x6c; any other kind changes nothing.
pub fn fn_00b805d0(e: &mut Engine, this: Ptr, kind: u32) {
    let base = this.addr();
    match kind {
        0x2f0 => {
            let pass = e.mem.u32(base + 0x70);
            e.set_global(CURRENT_PASS_0126F74C, pass);
        }
        0x2f1 => {
            let pass = e.mem.u32(base + 0x6c);
            e.set_global(CURRENT_PASS_0126F74C, pass);
        }
        _ => {}
    }
}

/// What `00b807b0` answers: the address `011fedc8` (no RTTI locator for the vtable; the word
/// is the class's `NiRTTI` by its place in the vtable, unconfirmed).
const RECORD_011FEDC8: u32 = 0x011f_edc8;

// Translated from 00b807b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 2 of the vtable `010af760`, `thiscall`
/// without reading `this`: answers the address `011fedc8`.
pub fn fn_00b807b0(_e: &mut Engine, _this: Ptr) -> u32 {
    RECORD_011FEDC8
}

// Translated from 00b80b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatter::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// `LoadShaders`, then `LoadStagesAndPasses` (a tail jump: its answer is this function's).
pub fn bs_shader_blood_splatter_reload_shaders(e: &mut Engine, this: Ptr) -> bool {
    bs_shader_blood_splatter_load_shaders(e, this);
    bs_shader_blood_splatter_load_stages_and_passes(e, this)
}

/// The stage texture setter `00e7de90` (unnamed; `thiscall` on a stage, one word),
/// `ImageSpaceTexture::GetTexture` (engine map, `00ba3780`; `thiscall`, no stack words) on the
/// image-space texture at `012005a4`.
const STAGE_SET_00E7DE90: u32 = 0x00e7_de90;
const GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const IMAGE_SPACE_TEXTURE_012005A4: u32 = 0x0120_05a4;

/// The stage's texture word (+8) set to `texture`, then `00e7de90` on the stage with what the
/// shader property's slot +0xcc answers.
fn set_stage_texture(e: &mut Engine, stage: u32, texture: u32, property: u32) {
    e.mem.set_u32(stage + 8, texture);
    let value = e.vcall(property, 0xcc, &args![]).u32();
    e.call(STAGE_SET_00E7DE90, &args![stage, value]);
}

// Translated from 00b80b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`), `thiscall`, one word (the
/// geometry; its shader property at +0xc). For pass kind 0x2f1 (`011f91e4`) the two stages of
/// the pass at +0x6c (its stage array at +0x24) take the property's textures +0x60 and +0x64;
/// for kind 0x2f0 the first stage of the pass at +0x70 takes +0x60 and the second the
/// image-space texture at `012005a4` (`GetTexture`, asked twice) or, when it has none, the
/// texture at `011fedc4`. Each stage then gets `00e7de90` with the property's slot +0xcc. Last
/// `this`'s slot +0x108.
pub fn fn_00b80b30(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let base = this.addr();
    let property = e.mem.u32(geometry.addr() + 0xc);
    let kind = e.global::<u32>(PASS_KIND_011F91E4);
    if kind == 0x2f1 {
        let pass = e.mem.u32(base + 0x6c);
        for index in 0..2u32 {
            let stages = e.mem.u32(pass + 0x24);
            let stage = e.mem.u32(stages + 4 * index);
            let texture = e.mem.u32(property + 0x60 + 4 * index);
            set_stage_texture(e, stage, texture, property);
        }
    } else if kind == 0x2f0 {
        let pass = e.mem.u32(base + 0x70);
        let stage = e.mem.u32(e.mem.u32(pass + 0x24));
        let texture = e.mem.u32(property + 0x60);
        set_stage_texture(e, stage, texture, property);
        let second = e.mem.u32(e.mem.u32(pass + 0x24) + 4);
        let image = e
            .call(GET_TEXTURE_00BA3780, &args![IMAGE_SPACE_TEXTURE_012005A4])
            .u32();
        let texture = if image != 0 {
            e.call(GET_TEXTURE_00BA3780, &args![IMAGE_SPACE_TEXTURE_012005A4])
                .u32()
        } else {
            e.global::<u32>(TEXTURE_POINTER_011FEDC4)
        };
        set_stage_texture(e, second, texture, property);
    }
    e.vcall(base, 0x108, &args![]);
}

/// What `00b80c20` reads: the accumulator holder (`011f91e0`, its first word), the fallback
/// object `011f4288` for a null +0x20, `00e6fbb0` (unnamed; `thiscall` on the shader's +0x18,
/// two words) and the `float`s `011ad878` and `011ad874`; it writes the `float` at `011fedb4`
/// (the `"alpha"` constant).
const ACCUMULATOR_011F91E0: u32 = 0x011f_91e0;
const FALLBACK_011F4288: u32 = 0x011f_4288;
const FN_00E6FBB0: u32 = 0x00e6_fbb0;
const SCALE_011AD878: u32 = 0x011a_d878;
const SCALE_011AD874: u32 = 0x011a_d874;

// Translated from 00b80c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderbloodsplatter.cpp`), `thiscall`, one word (the
/// geometry; its shader property at +0xc). With A the accumulator (`*011f91e0`): `00e6fbb0`
/// on the shader's +0x18 with A + 0x68 and 0; the alpha constant `011fedb4` becomes
/// `*property[+0x6c] * property[+0x28] * 011ad878` (x87, stored to `float` after the first
/// product), times `011ad874 * 011fed90` for pass kind 0x2f0. Then each constant map (+0x30
/// with the current pass's +0x44, +0x34 with its +0x5c), when both are set, is given its slot
/// +0x78 with (that word, A, A[+0xbc], 0, A[+0xb8][+0x34], the geometry, 0, A + 0x68, A[+0x20]
/// or `011f4288` when null, 0, 1).
pub fn fn_00b80c20(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let base = this.addr();
    let accumulator = e.mem.u32(e.global::<u32>(ACCUMULATOR_011F91E0));
    let mut object = e.mem.u32(accumulator + 0x20);
    let list = accumulator + 0x68;
    if object == 0 {
        object = FALLBACK_011F4288;
    }
    let word_34 = e.mem.u32(e.mem.u32(accumulator + 0xb8) + 0x34);
    let word_bc = e.mem.u32(accumulator + 0xbc);
    let member = e.mem.u32(base + 0x18);
    e.call(FN_00E6FBB0, &args![member, list, 0u32]);
    let kind = e.global::<u32>(PASS_KIND_011F91E4);
    let property = e.mem.u32(geometry.addr() + 0xc);
    let value = e.mem.u32(e.mem.u32(property + 0x6c));
    let product = x87_store32(x87_mul(
        x87_load32(value),
        x87_m32(e.mem.u32(property + 0x28)),
    ));
    let alpha = x87_mul(
        x87_load32(product),
        x87_m32(e.global::<u32>(SCALE_011AD878)),
    );
    e.set_global(PIXEL_DATA_011FEDB4, x87_store32(alpha));
    if kind == 0x2f0 {
        let scaled = x87_mul(
            x87_load32(e.global::<u32>(SCALE_011AD874)),
            x87_m32(e.global::<u32>(DOT_011FED90)),
        );
        let alpha = x87_mul(scaled, x87_m32(e.global::<u32>(PIXEL_DATA_011FEDB4)));
        e.set_global(PIXEL_DATA_011FEDB4, x87_store32(alpha));
    }
    for (map_offset, pass_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let shader = e.mem.u32(pass + pass_offset);
        if shader == 0 {
            continue;
        }
        e.vcall(
            map,
            0x78,
            &args![
                shader,
                accumulator,
                word_bc,
                0u32,
                word_34,
                geometry,
                0u32,
                list,
                object,
                0u32,
                1u32
            ],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_0380, fn_00b80380(Ptr)),
        entry!(0x00b8_05d0, fn_00b805d0(Ptr, u32)),
        entry!(0x00b8_07b0, fn_00b807b0(Ptr) -> u32),
        entry!(0x00b8_0b20, bs_shader_blood_splatter_reload_shaders(Ptr) -> bool),
        entry!(0x00b8_0b30, fn_00b80b30(Ptr, Ptr)),
        entry!(0x00b8_0c20, fn_00b80c20(Ptr, Ptr)),
        entry!(0x00b8_0d30, bs_shader_blood_splatter_bs_shader_blood_splatter(Ptr) -> Ptr),
        entry!(0x00b8_0090, bs_shader_blood_splatter_load_shaders(Ptr)),
        entry!(0x00b8_07d0, bs_shader_blood_splatter_load_stages_and_passes(Ptr) -> bool),
        entry!(
            0x00b8_0e70,
            bs_shader_blood_splatter_create_shader() -> Ptr
        ),
        entry!(0x00b8_0600, fn_00b80600(Ptr, Ptr)),
        entry!(0x00b8_03b0, fn_00b803b0(Ptr)),
        entry!(0x00b8_0650, fn_00b80650(Ptr)),
        entry!(0x00b8_07c0, fn_00b807c0(Ptr, u32, u32) -> u32),
        entry!(0x00b8_0e40, fn_00b80e40(Ptr, u32) -> Ptr),
        entry!(
            0x00b8_0f50,
            bs_shader_blood_splatter_create_constant_maps(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_blood_splatter_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_0e70);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x84]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (LOAD_STAGES_AND_PASSES, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod b000h_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_blood_splatter_constructor_sets_the_globals_and_gets_the_texture() {
        let mut e = rig();
        e.register(GET_TEXTURE, |_, _| Ret::default());
        e.map(0x0101_7000, 0x8000);
        e.map(0x011f_e000, 0x1000);
        // A signalling NaN is quieted by the FPU load.
        e.set_global::<u32>(VALUE_010177E8, 0x7f80_0001);
        e.set_global::<f32>(VALUE_0101E2BC, 2.5);
        e.mem.write(FLOATS_011FED94, &[0xff; 0x10]);
        let this = e.mem.alloc(0x84);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BLOOD_SPLATTER_VTABLE_010AF760);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x74, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 2, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0x17);
        assert_eq!(e.global::<u32>(FLOATS_011FED94), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(FLOATS_011FED94 + 4), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(FLOATS_011FED94 + 8), 0);
        assert_eq!(e.global::<f32>(FLOATS_011FED94 + 0xc), 2.5);
        assert_eq!(
            calls(&e, GET_TEXTURE),
            vec![vec![
                TEXTURE_NAME_010AF89C,
                1,
                TEXTURE_POINTER_011FEDC4,
                1,
                0
            ]]
        );
    }

    #[test]
    fn bs_shader_blood_splatter_load_shaders_compiles_four_shaders() {
        let mut e = loader_rig();
        let this = e.mem.alloc(0x84);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x78, old);
        e.call(LOAD_SHADERS, &args![this]);
        let flare = macro_words(&[(MACRO_FLARE_010AF70C, EMPTY_STRING_01011584)]);
        let expected = [
            (
                0x74u32,
                VERTEX_SOURCE_010AF738,
                OUTPUT_BS_VSO_010AF704,
                false,
                [0; 8],
            ),
            (
                0x78,
                VERTEX_SOURCE_010AF738,
                OUTPUT_BSF_VSO_010AF6FC,
                false,
                flare,
            ),
            (
                0x7c,
                PIXEL_SOURCE_010AF714,
                OUTPUT_BS_PSO_010AF6F4,
                true,
                [0; 8],
            ),
            (
                0x80,
                PIXEL_SOURCE_010AF714,
                OUTPUT_BSF_PSO_010AF6EC,
                true,
                flare,
            ),
        ];
        for (slot, file, output, pixel, macros) in expected {
            let shader = e.mem.u32(this + slot);
            let profile = if pixel {
                PROFILE_PS_2_A_010A93C4
            } else {
                PROFILE_VS_1_1_010A93EC
            };
            assert_eq!(
                compiled(&e, shader),
                Compiled {
                    file,
                    output,
                    profile,
                    pixel,
                    macros
                }
            );
            assert_eq!(e.mem.u32(shader + 4), 1);
        }
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        let cleared: Vec<u32> = calls(&e, MEMSET).iter().map(|a| a[2]).collect();
        assert_eq!(cleared, vec![0x44, 0x44, 0x3c, 0x3c]);
    }

    #[test]
    fn bs_shader_blood_splatter_load_stages_and_passes_builds_two_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x84);
        let shaders: Vec<u32> = (0..4).map(|_| object(&mut e, 1)).collect();
        for (index, shader) in shaders.iter().enumerate() {
            e.mem.set_u32(this + 0x74 + 4 * index as u32, *shader);
        }
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x6c], vec![this + 0x70]]
        );
        for (pass_offset, last, vertex, pixel) in [
            (0x6cu32, (1u32, 3u32), 0usize, 2usize),
            (0x70, (2, 2), 1, 3),
        ] {
            let pass = e.mem.u32(this + pass_offset);
            assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 1), (1, [3, 2, 1], 1)]);
            assert_eq!(
                render_states(&e, pass),
                vec![(0x1b, 1), (7, 0), (0xe, 0), (0x13, last.0), (0x14, last.1)]
            );
            assert_eq!(e.mem.u32(pass + 0x5c), shaders[vertex]);
            assert_eq!(e.mem.u32(pass + 0x44), shaders[pixel]);
        }
        // Each stage is held by its pass only.
        for a in calls(&e, PASS_SET_STAGE) {
            assert_eq!(e.mem.u32(a[2] + 0x60), 1);
        }
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }
}

#[cfg(test)]
mod pixel_shader_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00b80600_assigns_the_pixel_shader() {
        let mut e = rig();
        let pass = e.mem.alloc(0x68);
        let old = object(&mut e, 1);
        let new = object(&mut e, 0);
        e.mem.set_u32(pass + 0x44, old);
        e.call(0x00b8_0600, &args![pass, new]);
        assert_eq!(e.mem.u32(pass + 0x44), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        e.call(0x00b8_0600, &args![pass, new]);
        assert_eq!(e.mem.u32(new + 4), 1);
        e.call(0x00b8_0600, &args![pass, 0u32]);
        assert_eq!(e.mem.u32(pass + 0x44), 0);
        assert_eq!(e.mem.u32(new + 4), 0);
    }
}

#[cfg(test)]
mod splatter_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn map_globals(e: &mut Engine) {
        for page in [
            0x0101_1000u32,
            0x0101_e000,
            0x0107_1000,
            0x011f_9000,
            0x011f_e000,
        ] {
            e.map(page, 0x1000);
        }
    }

    fn constants(e: &Engine) -> Vec<f32> {
        (0..4)
            .map(|i| e.global::<f32>(FLOATS_011FED94 + 4 * i))
            .collect()
    }

    /// A rig for `00b803b0`: the test answers `hit` with the out point (4, 2, 0), the
    /// projection answers `projected` with the outputs `outputs`.
    fn splatter_rig(hit: u32, projected: u32, outputs: [f32; 3]) -> (Engine, u32) {
        let mut e = rig();
        map_globals(&mut e);
        e.set_global(VALUE_0101E2BC, 0.75f32);
        e.set_global(TWO_01011590, 2.0f64);
        e.set_global(PROJECT_VALUE_010718C0, 0.125f32);
        e.set_global(SPLATTER_OBJECT_011F91C8, 0x5000u32);
        e.set_global(DOT_011FED90, 9.0f32);
        let geometry = e.mem.alloc(0x98);
        // Column (1, 0, 0), translation (1, 2, 0).
        e.mem.set_f32(geometry + 0x68, 1.0);
        e.mem.set_f32(geometry + 0x8c, 1.0);
        e.mem.set_f32(geometry + 0x90, 2.0);
        e.register_double(SPLATTER_TEST_00B5BCA0, move |e, a| {
            e.mem.set_f32(a[3], 4.0);
            e.mem.set_f32(a[3] + 4, 2.0);
            e.mem.set_f32(a[3] + 8, 0.0);
            hit.into_ret()
        });
        e.register(NORMALIZE_004A0C10, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, c) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, c / length);
            }
            Ret::default()
        });
        e.register_double(PROJECT_00A6FDB0, move |e, a| {
            // The first output is the argument's own word, which held the geometry.
            assert_eq!(e.mem.f32(a[2] + 4), 0.0);
            e.mem.set_f32(a[2], outputs[0]);
            e.mem.set_f32(a[3], outputs[1]);
            e.mem.set_f32(a[4], outputs[2]);
            assert_eq!(a[5], 0.125f32.to_bits());
            projected.into_ret()
        });
        (e, geometry)
    }

    #[test]
    fn fn_00b803b0_without_a_hit_sets_the_rest_values() {
        let (mut e, geometry) = splatter_rig(0, 1, [0.0; 3]);
        e.call(0x00b8_03b0, &args![geometry]);
        assert_eq!(e.global::<f32>(DOT_011FED90), 0.0);
        assert_eq!(constants(&e), vec![0.0, 0.0, 0.0, 0.75]);
        let test = calls(&e, SPLATTER_TEST_00B5BCA0);
        assert_eq!(&test[0][..2], &[0x5000, geometry + 0x8c]);
        assert!(calls(&e, PROJECT_00A6FDB0).is_empty());
    }

    #[test]
    fn fn_00b803b0_projects_and_clamps() {
        let (mut e, geometry) = splatter_rig(1, 1, [1.5, -0.5, 0.25]);
        e.call(0x00b8_03b0, &args![geometry]);
        // Out point minus translation is (3, 0, 0): along the column, dot 1.
        assert_eq!(e.global::<f32>(DOT_011FED90), 1.0);
        // 1.5 clamps to 1 (2 * 1 - 1 = 1), -0.5 to 0 (2 * 0 - 1 = -1).
        assert_eq!(constants(&e), vec![1.0, -1.0, 0.25, 0.75]);
        let projection = calls(&e, PROJECT_00A6FDB0);
        assert_eq!(projection[0][0], geometry);
        let (mut e, geometry) = splatter_rig(1, 1, [0.25, 0.5, 2.0]);
        e.call(0x00b8_03b0, &args![geometry]);
        assert_eq!(constants(&e), vec![-0.5, 0.0, 2.0, 0.75]);
    }

    #[test]
    fn fn_00b803b0_keeps_a_nan_output_and_handles_a_failed_projection() {
        let (mut e, geometry) = splatter_rig(1, 1, [f32::NAN, 3.0, 0.0]);
        e.call(0x00b8_03b0, &args![geometry]);
        assert!(constants(&e)[0].is_nan());
        assert_eq!(constants(&e)[1], 1.0);
        let (mut e, geometry) = splatter_rig(1, 0, [0.5; 3]);
        e.call(0x00b8_03b0, &args![geometry]);
        assert_eq!(e.global::<f32>(DOT_011FED90), 1.0);
        assert_eq!(constants(&e), vec![0.0, 0.0, 0.0, 0.75]);
    }

    #[test]
    fn fn_00b80650_drops_everything_and_destroys() {
        let mut e = rig();
        map_globals(&mut e);
        noop(&mut e, &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0x84);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 2);
        e.mem.set_u32(this + 0x70, pass);
        let shader = object(&mut e, 1);
        e.mem.set_u32(this + 0x80, shader);
        let texture = object(&mut e, 1);
        e.set_global(TEXTURE_POINTER_011FEDC4, texture);
        e.call(0x00b8_0650, &args![this]);
        assert_eq!(e.mem.u32(this), BLOOD_SPLATTER_VTABLE_010AF760);
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            vec![vec![shader], vec![texture]]
        );
        assert_eq!(e.global::<u32>(TEXTURE_POINTER_011FEDC4), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x7c, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x74, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 2, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00b807c0_answers_the_declaration_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x28, 0x2828);
        assert_eq!(e.call(0x00b8_07c0, &args![this, 1u32, 2u32]).u32(), 0x2828);
    }

    #[test]
    fn fn_00b80e40_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        map_globals(&mut e);
        noop(
            &mut e,
            &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460],
        );
        let this = e.mem.alloc(0x84);
        assert_eq!(e.call(0x00b8_0e40, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x84]]);
        e.call(0x00b8_0e40, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn create_constant_maps_makes_both_maps() {
        const MAP_SLOTS: u32 = 0x0e30_0000;
        const MAP_VTABLE: u32 = 0x0e31_0000;
        let mut e = rig();
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        noop(&mut e, &[MAP_SLOTS + 8]);
        let this = e.mem.alloc(0x84);
        e.call(0x00b8_0f50, &args![this]);
        let pixel = e.mem.u32(this + 0x30);
        let vertex = e.mem.u32(this + 0x34);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0][..2], [pixel, ALPHA_0103A454]);
        assert_eq!(entries[0][9], PIXEL_DATA_011FEDB4);
        assert_eq!(
            entries[1][..2],
            [vertex, WORLD_VIEW_PROJ_TRANSPOSE_010A1E50]
        );
        assert_eq!(entries[2][4], 4);
        assert_eq!(entries[2][9], FLOATS_011FED94);
        assert_eq!(entries[3][1], CTRL_010AF8BC);
        assert_eq!(entries[3][9], VERTEX_DATA_011FEDA4);
        let maps = calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330);
        assert_eq!(maps[0][2], 2);
        assert_eq!(maps[1][2], 0);
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

    /// Slot doubles: each `SLOTS + offset` is a no-op answering `0x900 + offset`.
    const SLOTS: u32 = 0x0e50_0000;

    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for offset in (0..0x200u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(0x84);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [0x011f_9000u32, 0x011f_e000, 0x0126_f000, 0x011a_d000] {
            e.map(page, 0x1000);
        }
        for offset in (0..0x200u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.register(SLOTS + 0xcc, |_, _| 0x9ccu32.into_ret());
        e.register(STAGE_SET_00E7DE90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00b80380_gives_both_passes_to_slot_0x128() {
        let mut e = rig();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x6c, 0x61);
        e.mem.set_u32(this + 0x70, 0x62);
        e.call(0x00b8_0380, &args![this]);
        assert_eq!(
            calls(&e, SLOTS + 0x128),
            vec![vec![this, 0x61], vec![this, 0x62]]
        );
    }

    #[test]
    fn fn_00b805d0_picks_the_pass_of_the_kind() {
        let mut e = rig();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x6c, 0x61);
        e.mem.set_u32(this + 0x70, 0x62);
        e.call(0x00b8_05d0, &args![this, 0x2f0u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x62);
        e.call(0x00b8_05d0, &args![this, 0x2f1u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x61);
        e.call(0x00b8_05d0, &args![this, 0x2f2u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x61);
        assert_eq!(e.call(0x00b8_07b0, &args![this]).u32(), 0x011f_edc8);
    }

    #[test]
    fn fn_00b80b20_loads_shaders_then_passes() {
        use crate::units::bsshader::geometrydecalshader::shader_test_rig::pass_rig;
        let mut e = pass_rig();
        let this = e.mem.alloc(0x84);
        assert!(e.call(0x00b8_0b20, &args![this]).bool());
        assert_eq!(calls(&e, COMPILE_VERTEX_SHADER).len(), 2);
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), 2);
        // The passes get the shaders just compiled.
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x6c], vec![this + 0x70]]
        );
        let pass = e.mem.u32(this + 0x6c);
        assert_eq!(e.mem.u32(pass + 0x5c), e.mem.u32(this + 0x74));
        let log = e.call_log.as_ref().unwrap();
        let last_compile = log
            .iter()
            .rposition(|(a, _)| *a == COMPILE_PIXEL_SHADER)
            .unwrap();
        let first_pass = log
            .iter()
            .position(|(a, _)| *a == NEW_PASS_00E804D0)
            .unwrap();
        assert!(last_compile < first_pass);
    }

    /// A stage array of two stages at the pass's +0x24.
    fn pass(e: &mut Engine) -> (u32, [u32; 2]) {
        let pass = e.mem.alloc(0x28);
        let stages = e.mem.alloc(8);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(stages, first);
        e.mem.set_u32(stages + 4, second);
        e.mem.set_u32(pass + 0x24, stages);
        (pass, [first, second])
    }

    fn geometry(e: &mut Engine) -> (u32, u32) {
        let property = object(e);
        e.mem.set_u32(property + 0x60, 0x160);
        e.mem.set_u32(property + 0x64, 0x164);
        let geometry = e.mem.alloc(0x10);
        e.mem.set_u32(geometry + 0xc, property);
        (geometry, property)
    }

    #[test]
    fn fn_00b80b30_kind_0x2f1_sets_both_stages() {
        let mut e = rig();
        let this = object(&mut e);
        let (pass, stages) = pass(&mut e);
        e.mem.set_u32(this + 0x6c, pass);
        let (geometry, property) = geometry(&mut e);
        e.set_global(PASS_KIND_011F91E4, 0x2f1u32);
        e.call(0x00b8_0b30, &args![this, geometry]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x160);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x164);
        assert_eq!(
            calls(&e, STAGE_SET_00E7DE90),
            vec![vec![stages[0], 0x9cc], vec![stages[1], 0x9cc]]
        );
        assert_eq!(calls(&e, SLOTS + 0xcc), vec![vec![property]; 2]);
        assert_eq!(calls(&e, SLOTS + 0x108), vec![vec![this]]);
    }

    #[test]
    fn fn_00b80b30_kind_0x2f0_uses_the_image_space_texture() {
        for image in [0u32, 0x7777] {
            let mut e = rig();
            e.map(0x0120_0000, 0x1000);
            let this = object(&mut e);
            let (pass, stages) = pass(&mut e);
            e.mem.set_u32(this + 0x70, pass);
            let (geometry, _) = geometry(&mut e);
            e.set_global(PASS_KIND_011F91E4, 0x2f0u32);
            e.set_global(TEXTURE_POINTER_011FEDC4, 0x8888u32);
            e.register_double(GET_TEXTURE_00BA3780, move |_, _| image.into_ret());
            e.call(0x00b8_0b30, &args![this, geometry]);
            assert_eq!(e.mem.u32(stages[0] + 8), 0x160);
            let want = if image != 0 { image } else { 0x8888 };
            assert_eq!(e.mem.u32(stages[1] + 8), want);
            let asked = calls(&e, GET_TEXTURE_00BA3780);
            assert_eq!(asked.len(), if image != 0 { 2 } else { 1 });
            assert!(asked.iter().all(|a| a[0] == IMAGE_SPACE_TEXTURE_012005A4));
            assert_eq!(calls(&e, STAGE_SET_00E7DE90).len(), 2);
        }
    }

    #[test]
    fn fn_00b80b30_other_kinds_only_call_slot_0x108() {
        let mut e = rig();
        let this = object(&mut e);
        let (geometry, _) = geometry(&mut e);
        e.set_global(PASS_KIND_011F91E4, 5u32);
        e.call(0x00b8_0b30, &args![this, geometry]);
        assert!(calls(&e, STAGE_SET_00E7DE90).is_empty());
        assert_eq!(calls(&e, SLOTS + 0x108), vec![vec![this]]);
    }

    /// `00b80c20`'s rig: the accumulator, a property (+0x28 2.0, `*+0x6c` 3.0), the current
    /// pass with shaders at +0x44 and +0x5c, the scales.
    fn constants_rig(kind: u32, fallback: bool) -> (Engine, u32, u32, u32) {
        let mut e = rig();
        let accumulator = e.mem.alloc(0xc0);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, accumulator);
        e.set_global(ACCUMULATOR_011F91E0, holder);
        if !fallback {
            e.mem.set_u32(accumulator + 0x20, 0x2020);
        }
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x34, 0x3434);
        e.mem.set_u32(accumulator + 0xb8, inner);
        e.mem.set_u32(accumulator + 0xbc, 0xbcbc);
        let (geometry, property) = geometry(&mut e);
        e.mem.set_f32(property + 0x28, 2.0);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 3.0);
        e.mem.set_u32(property + 0x6c, value);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x44, 0x4444);
        e.mem.set_u32(pass + 0x5c, 0x5c5c);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        e.set_global(SCALE_011AD878, 0.5f32);
        e.set_global(SCALE_011AD874, 4.0f32);
        e.set_global(DOT_011FED90, 0.25f32);
        e.set_global(PASS_KIND_011F91E4, kind);
        e.register(FN_00E6FBB0, |_, _| Ret::default());
        let this = object(&mut e);
        e.mem.set_u32(this + 0x18, 0x1818);
        (e, this, geometry, accumulator)
    }

    #[test]
    fn fn_00b80c20_sets_alpha_and_feeds_both_maps() {
        let (mut e, this, geometry, accumulator) = constants_rig(0x2f1, false);
        let pixel = object(&mut e);
        let vertex = object(&mut e);
        e.mem.set_u32(this + 0x30, pixel);
        e.mem.set_u32(this + 0x34, vertex);
        e.call(0x00b8_0c20, &args![this, geometry]);
        assert_eq!(
            calls(&e, FN_00E6FBB0),
            vec![vec![0x1818, accumulator + 0x68, 0]]
        );
        assert_eq!(e.global::<f32>(PIXEL_DATA_011FEDB4), 3.0);
        let fed = calls(&e, SLOTS + 0x78);
        let tail = |shader: u32| {
            vec![
                shader,
                accumulator,
                0xbcbc,
                0,
                0x3434,
                geometry,
                0,
                accumulator + 0x68,
                0x2020,
                0,
                1,
            ]
        };
        assert_eq!(fed.len(), 2);
        assert_eq!(fed[0][0], pixel);
        assert_eq!(fed[0][1..], tail(0x4444)[..]);
        assert_eq!(fed[1][0], vertex);
        assert_eq!(fed[1][1..], tail(0x5c5c)[..]);
    }

    #[test]
    fn fn_00b80c20_kind_0x2f0_scales_and_skips_missing_maps() {
        let (mut e, this, geometry, _) = constants_rig(0x2f0, true);
        let vertex = object(&mut e);
        e.mem.set_u32(this + 0x34, vertex);
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        e.mem.set_u32(pass + 0x5c, 0);
        e.call(0x00b8_0c20, &args![this, geometry]);
        // 3 * 4 * 0.25 = 3.
        assert_eq!(e.global::<f32>(PIXEL_DATA_011FEDB4), 3.0);
        assert!(calls(&e, SLOTS + 0x78).is_empty());
        // With the vertex shader there: the fallback object for a null +0x20.
        e.mem.set_u32(pass + 0x5c, 0x5c5c);
        e.call(0x00b8_0c20, &args![this, geometry]);
        assert_eq!(calls(&e, SLOTS + 0x78)[0][9], FALLBACK_011F4288);
    }
}
