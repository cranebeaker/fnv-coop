//! `BSShader/skyshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `SkyShader::SkyShader` (`00b8a1f0`), `::LoadShaders` (`00b874f0`) and
/// `::LoadStagesAndPasses` (`00b887c0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_a1f0;
const LOAD_SHADERS: u32 = 0x00b8_74f0;
const LOAD_STAGES_AND_PASSES: u32 = 0x00b8_87c0;

// Translated from 00b8a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0xa. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 3 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 5, 4, 3, 0), (2, 7, 5, 1, 0)) and calls slot
/// +0xb4. Then allocates 0xf0 bytes and runs the constructor (`00b8a1f0`; a null allocation
/// is used as it is), calls the shader's slot +0x4c, `LoadShaders` (`00b874f0`),
/// `LoadStagesAndPasses` (`00b887c0`), slot +0x11c, then +0xc0 with the declaration, and
/// answers the shader.
// Translation incomplete: 00b8a390 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn sky_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 5, 4, 3, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xf0u32]).u32();
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

/// The sky shader's vtable (`010afe18`; no RTTI locator before it).
const SKY_SHADER_VTABLE_010AFE18: u32 = 0x010a_fe18;
/// The four words copied (`011a9bd0`) and the seven four-word blocks they go to, in the
/// exe's order of their first words.
const COLOR_011A9BD0: u32 = 0x011a_9bd0;
const COLOR_BLOCKS: [u32; 7] = [
    0x011f_f910,
    0x011f_f920,
    0x011f_f930,
    0x011f_f8e0,
    0x011f_f8f0,
    0x011f_f900,
    0x011f_f8c8,
];

// Translated from 00b8a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::SkyShader` (Xbox PDB), `thiscall`, no stack words. Runs `BSShader::BSShader`
/// (`00be2200`), stores the vtable `010afe18`, constructs the `NiPointer` arrays at +0x6c (0xb,
/// destructor `00b79930`), +0x98 (0xb) and +0xc4 (8; destructor `00b66f20`), sets +0x68 to
/// 0xb, copies the four words at `011a9bd0` into each of the blocks at `011ff910`,
/// `011ff920`, `011ff930`, `011ff8e0`, `011ff8f0`, `011ff900` and `011ff8c8`, and sets the
/// byte +0x24 to 1. Answers `this`.
// Translation incomplete: 00b8a1f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn sky_shader_sky_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SKY_SHADER_VTABLE_010AFE18);
    construct_pointer_array(e, base + 0x6c, 0xb, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x98, 0xb, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xc4, 8, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 0xb);
    let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(COLOR_011A9BD0 + offset));
    for block in COLOR_BLOCKS {
        e.set_global::<u32>(block, color[0]);
    }
    for block in COLOR_BLOCKS {
        for word in 1..4u32 {
            e.set_global::<u32>(block + 4 * word, color[word as usize]);
        }
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"sky\\v\\sky.v.hlsl"` (`010afe04`),
/// `"sky\\v\\sky_quad.v.hlsl"` (`010afcbc`), `"sky\\p\\sky.p.hlsl"` (`010afd50`); the macros
/// `"TEX"`, `"DEPTH_VALUE"` (with `"0.99995f"`, `"1.0f"` or `"0.999999"`), `"HORIZFADE"`,
/// `"OCCLUSION"`, `"CLOUDS"`, `"SI"`, `"SI_CLOUDS"`, `"SI_SUN"`; the compiled names from
/// `"SKY.vso"` to `"SKYSICLOUDS.pso"` (see the steps).
const SKY_VERTEX_SOURCE_010AFE04: u32 = 0x010a_fe04;
const SKY_QUAD_VERTEX_SOURCE_010AFCBC: u32 = 0x010a_fcbc;
const SKY_PIXEL_SOURCE_010AFD50: u32 = 0x010a_fd50;
const SKY_MACRO_TEX_010AECAC: u32 = 0x010a_ecac;
const MACRO_DEPTH_VALUE_010AFDE4: u32 = 0x010a_fde4;
const DEPTH_0_99995F_010AFDD8: u32 = 0x010a_fdd8;
const DEPTH_1_0F_010AFDC4: u32 = 0x010a_fdc4;
const DEPTH_0_999999_010AFCFC: u32 = 0x010a_fcfc;
const MACRO_HORIZFADE_010AFDB8: u32 = 0x010a_fdb8;
const MACRO_OCCLUSION_010AFD9C: u32 = 0x010a_fd9c;
const MACRO_CLOUDS_010AFD88: u32 = 0x010a_fd88;
const MACRO_SI_010AEC48: u32 = 0x010a_ec48;
const MACRO_SI_CLOUDS_010AFCE4: u32 = 0x010a_fce4;
const MACRO_SI_SUN_010AFC98: u32 = 0x010a_fc98;
const SKY_EMPTY_STRING_01011584: u32 = 0x0101_1584;
/// The vertex profile chooser `00b4f3e0` (no arguments) and the pixel one `00b4f380` (`cdecl`,
/// a byte; given 0) (unit `bsshadermanager.cpp`, unnamed; they answer a profile string).
const VERTEX_PROFILE_00B4F3E0: u32 = 0x00b4_f3e0;
const SKY_PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// CRT `_gcvt` (`00ecdd16`, `cdecl`: the `double`, the digit count, the buffer) and the two
/// `float`s it is given (`011ff8ac`, `011ff8b0`, each through `FLD`).
const GCVT: u32 = 0x00ec_dd16;
const SUN_VALUE_011FF8AC: u32 = 0x011f_f8ac;
const SI_VALUE_011FF8B0: u32 = 0x011f_f8b0;
/// The `LoadShaders` frame (0x8d4 bytes up to the return address), its name and path buffers
/// and the `_gcvt` buffer.
const SKY_FRAME_BYTES: u32 = 0x8d4;
const SKY_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x568,
    path: 0x66c,
};
const SKY_NUMBER_BUFFER: u32 = 0x770;

/// One step of the sky `LoadShaders`: the `float` formatted by `_gcvt` first (0 for none), the
/// source block filled first (with the word that receives the number buffer's address, if
/// any), the file word's frame offset (or the file itself when `file_value` is set), the
/// compiled name, vertex or pixel, the macros' frame offset and the shader field.
struct SkyStep {
    gcvt: u32,
    block: ShaderSource,
    number_word: Option<u32>,
    file_offset: u32,
    file_value: bool,
    output: u32,
    pixel: bool,
    macros: u32,
    slot: u32,
}

/// The sky steps in the exe's order.
const SKY_STEPS: [SkyStep; 18] = [
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x4d0,
            words: &[SKY_VERTEX_SOURCE_010AFE04, 0, 0],
            cleared: 0x40,
        },
        number_word: None,
        file_offset: 0x4d0,
        file_value: false,
        output: 0x010a_fdfc,
        pixel: false,
        macros: 0x4d4,
        slot: 0x98,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x308,
            words: &[SKY_VERTEX_SOURCE_010AFE04, SKY_MACRO_TEX_010AECAC, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x308,
        file_value: false,
        output: 0x010a_fdf0,
        pixel: false,
        macros: 0x30c,
        slot: 0x9c,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x224,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_99995F_010AFDD8,
                SKY_MACRO_TEX_010AECAC,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x224,
        file_value: false,
        output: 0x010a_fdcc,
        pixel: false,
        macros: 0x228,
        slot: 0xa0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0xf4,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_1_0F_010AFDC4,
                MACRO_HORIZFADE_010AFDB8,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0xf4,
        file_value: false,
        output: 0x010a_fda8,
        pixel: false,
        macros: 0xf8,
        slot: 0xa8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x484,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_OCCLUSION_010AFD9C,
                0,
                0,
                0,
            ],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x484,
        file_value: false,
        output: 0x010a_fd90,
        pixel: false,
        macros: 0x488,
        slot: 0xac,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0xa8,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                SKY_MACRO_TEX_010AECAC,
                0,
                MACRO_CLOUDS_010AFD88,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0xa8,
        file_value: false,
        output: 0x010a_fd78,
        pixel: false,
        macros: 0xac,
        slot: 0xb0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x5c,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_HORIZFADE_010AFDB8,
                0,
                SKY_MACRO_TEX_010AECAC,
                0,
                MACRO_CLOUDS_010AFD88,
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
                0,
                0,
                0,
            ],
            cleared: 0,
        },
        number_word: None,
        file_offset: 0xa8,
        file_value: false,
        output: 0x010a_fd64,
        pixel: false,
        macros: 0x60,
        slot: 0xb4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x51c,
            words: &[SKY_PIXEL_SOURCE_010AFD50, 0, 0],
            cleared: 0x40,
        },
        number_word: None,
        file_offset: 0x51c,
        file_value: false,
        output: 0x010a_fd48,
        pixel: true,
        macros: 0x520,
        slot: 0xc4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x270,
            words: &[SKY_PIXEL_SOURCE_010AFD50, SKY_MACRO_TEX_010AECAC, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x270,
        file_value: false,
        output: 0x010a_fd3c,
        pixel: true,
        macros: 0x274,
        slot: 0xc8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x2bc,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_HORIZFADE_010AFDB8,
                0,
                SKY_MACRO_TEX_010AECAC,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x270,
        file_value: false,
        output: 0x010a_fd2c,
        pixel: true,
        macros: 0x2c0,
        slot: 0xcc,
    },
    SkyStep {
        gcvt: SUN_VALUE_011FF8AC,
        block: ShaderSource {
            offset: 0x438,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_OCCLUSION_010AFD9C, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x438,
        file_value: false,
        output: 0x010a_fd1c,
        pixel: true,
        macros: 0x43c,
        slot: 0xd0,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x3a0,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_HORIZFADE_010AFDB8, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x3a0,
        file_value: false,
        output: 0x010a_fd08,
        pixel: true,
        macros: 0x3a4,
        slot: 0xd4,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x1d8,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_999999_010AFCFC,
                MACRO_SI_010AEC48,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x1d8,
        file_value: false,
        output: 0x010a_fcf0,
        pixel: false,
        macros: 0x1dc,
        slot: 0xb8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x10,
            words: &[
                SKY_VERTEX_SOURCE_010AFE04,
                MACRO_DEPTH_VALUE_010AFDE4,
                DEPTH_0_999999_010AFCFC,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_CLOUDS_010AFCE4,
                SKY_EMPTY_STRING_01011584,
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
                0,
                0,
            ],
            cleared: 0,
        },
        number_word: None,
        file_offset: SKY_VERTEX_SOURCE_010AFE04,
        file_value: true,
        output: 0x010a_fcd4,
        pixel: false,
        macros: 0x14,
        slot: 0xbc,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x3ec,
            words: &[SKY_QUAD_VERTEX_SOURCE_010AFCBC, MACRO_SI_010AEC48, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: None,
        file_offset: 0x3ec,
        file_value: false,
        output: 0x010a_fcac,
        pixel: false,
        macros: 0x3f0,
        slot: 0xc0,
    },
    SkyStep {
        gcvt: SI_VALUE_011FF8B0,
        block: ShaderSource {
            offset: 0x354,
            words: &[SKY_PIXEL_SOURCE_010AFD50, MACRO_SI_010AEC48, 0, 0, 0],
            cleared: 0x38,
        },
        number_word: Some(2),
        file_offset: 0x354,
        file_value: false,
        output: 0x010a_fca0,
        pixel: true,
        macros: 0x358,
        slot: 0xe0,
    },
    SkyStep {
        gcvt: SUN_VALUE_011FF8AC,
        block: ShaderSource {
            offset: 0x140,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_SUN_010AFC98,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: Some(2),
        file_offset: 0x140,
        file_value: false,
        output: 0x010a_fc88,
        pixel: true,
        macros: 0x144,
        slot: 0xd8,
    },
    SkyStep {
        gcvt: 0,
        block: ShaderSource {
            offset: 0x18c,
            words: &[
                SKY_PIXEL_SOURCE_010AFD50,
                MACRO_SI_010AEC48,
                0,
                MACRO_SI_CLOUDS_010AFCE4,
                0,
                0,
                0,
            ],
            cleared: 0x30,
        },
        number_word: None,
        file_offset: 0x18c,
        file_value: false,
        output: 0x010a_fc78,
        pixel: true,
        macros: 0x190,
        slot: 0xdc,
    },
];

// Translated from 00b874f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Eighteen steps in the exe's
/// order (see `SKY_STEPS`), each filling one source block on its stack (the sky, sky-quad and
/// sky pixel HLSL files with their macros; two blocks written word by word without
/// `memset`) and compiling it: `MakePath` of the step's file word (two steps take another
/// block's word, one the file string itself), the compiled name copied by `00406d00`, the
/// profile from `00b4f3e0()` for vertex shaders or `00b4f380(0)` for pixel shaders, the
/// compile with the step's macros, the result assigned to the shader's `NiPointer` (+0x98 to
/// +0xe0). Three steps first format a `float` (`011ff8ac` or `011ff8b0`, through the FPU) with
/// `_gcvt(value, 12, buffer)`; two of them give the buffer's address as the `SI` macro's value
/// (the first one's text is not used).
// Translation incomplete: 00b874f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn sky_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(SKY_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        let numbers = frame + SKY_NUMBER_BUFFER;
        let path = frame + SKY_BUFFERS.path;
        let name = frame + SKY_BUFFERS.name;
        for step in &SKY_STEPS {
            if step.gcvt != 0 {
                let value = x87_load32(e.global::<u32>(step.gcvt));
                e.call(GCVT, &args![value, 0xcu32, numbers]);
            }
            set_up_sources(e, frame, core::slice::from_ref(&step.block));
            if let Some(index) = step.number_word {
                e.mem
                    .set_u32(frame + step.block.offset + 4 * index, numbers);
            }
            let file = if step.file_value {
                step.file_offset
            } else {
                e.mem.u32(frame + step.file_offset)
            };
            e.call(MAKE_PATH, &args![file, path]);
            e.call(FORMAT_00406D00, &args![name, 0x104u32, step.output]);
            let (compile, profile) = if step.pixel {
                let profile = e.call(SKY_PIXEL_PROFILE_00B4F380, &args![0u32]).u32();
                (COMPILE_PIXEL_SHADER, profile)
            } else {
                let profile = e.call(VERTEX_PROFILE_00B4F3E0, &args![]).u32();
                (COMPILE_VERTEX_SHADER, profile)
            };
            let shader = e
                .call(
                    compile,
                    &args![base, path, frame + step.macros, profile, name],
                )
                .u32();
            ni_pointer_assign(e, base + step.slot, shader);
        }
    });
}

/// One pass of `SkyShader::LoadStagesAndPasses` (immediates of its straight-line code): the
/// pass pointer's offset, the number of texture stages, the render states (state, value,
/// whether the exe sets it through the pass it read for the state before), and the vertex and
/// pixel shader `NiPointer`s it takes.
type SkyPass = (u32, u32, &'static [(u32, u32, bool)], u32, u32);

/// The sky passes in the exe's order.
const SKY_PASSES: [SkyPass; 11] = [
    (
        0x6c,
        1,
        &[
            (0x1b, 1, false),
            (0x13, 2, false),
            (0x14, 2, true),
            (0xe, 0, false),
            (0xa8, 0, false),
        ],
        0xac,
        0xd0,
    ),
    (
        0x8c,
        1,
        &[
            (0xf, 0, false),
            (0x1b, 0, false),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xc0,
        0xe0,
    ),
    (
        0x90,
        1,
        &[
            (0x1b, 1, false),
            (0x13, 2, false),
            (0x14, 2, true),
            (0xf, 0, false),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xb8,
        0xd8,
    ),
    (
        0x94,
        2,
        &[
            (0xf, 0, false),
            (0x1b, 1, false),
            (0x13, 1, false),
            (0x14, 6, true),
            (7, 1, false),
            (0xe, 0, false),
            (0x17, 4, false),
            (0xa8, 8, false),
        ],
        0xbc,
        0xdc,
    ),
    (
        0x70,
        2,
        &[
            (7, 0, false),
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 2, true),
            (0x34, 0, false),
            (0xf, 0, false),
            (0xa8, 7, false),
        ],
        0x9c,
        0xc8,
    ),
    (
        0x78,
        1,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 2, true),
            (0xa8, 7, false),
        ],
        0xa8,
        0xd4,
    ),
    (
        0x7c,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0xb0,
        0xc8,
    ),
    (
        0x80,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0xb4,
        0xcc,
    ),
    (
        0x84,
        2,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0x9c,
        0xc8,
    ),
    (
        0x74,
        2,
        &[
            (0xf, 1, false),
            (0x18, 0, false),
            (0x19, 5, false),
            (0x1b, 1, false),
            (0x13, 1, false),
            (0x14, 2, true),
        ],
        0xa0,
        0xc8,
    ),
    (
        0x88,
        0,
        &[
            (0xe, 0, false),
            (0x1b, 1, false),
            (0x13, 5, false),
            (0x14, 6, true),
            (0xa8, 7, false),
        ],
        0x98,
        0xc4,
    ),
];

// Translated from 00b887c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers 1. For each
/// of the eleven pass pointers in the exe's order (see `SKY_PASSES`): when the pointer is null,
/// a new pass (`00e804d0`), its texture stages (one or two, made into one stack pointer by
/// `CreateNewStage` without dropping the one before, `SetupDefaultStage` with the stage's
/// index and 3, 2, 1, no filter mode, `SetStage` with the pass's word at +0x14) and its render
/// states (the pass read again for each but 0x14, which goes through the pass read for 0x13);
/// then, whether or not the pass was made, its vertex (+0x5c) and pixel (+0x44) shader
/// `NiPointer`s are assigned from the shader's. Finally the stage reference is dropped.
// Translation incomplete: 00b887c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn sky_shader_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (pass_offset, stages, states, vertex, pixel) in SKY_PASSES {
            let slot = base + pass_offset;
            if e.mem.u32(slot) == 0 {
                e.call(NEW_PASS_00E804D0, &args![slot]);
                for index in 0..stages {
                    e.call(CREATE_NEW_STAGE, &args![local]);
                    let stage = e.mem.u32(local);
                    e.call(SETUP_DEFAULT_STAGE, &args![stage, index, 3u32, 2u32, 1u32]);
                    let pass = e.mem.u32(slot);
                    let stage = e.mem.u32(local);
                    let position = e.mem.u32(pass + 0x14);
                    e.call(PASS_SET_STAGE, &args![pass, position, stage]);
                }
                let mut pass = e.mem.u32(slot);
                for &(state, value, reuse) in states {
                    if !reuse {
                        pass = e.mem.u32(slot);
                    }
                    pass_render_state(e, pass, state, value);
                }
            }
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// The `eh vector destructor iterator` (`00ec5fce`, `stdcall`), `BSShader::~BSShader` (engine
/// map, `00be0f10`, no stack words), the Ni free (`00aa1460`, `cdecl`: block, size) and
/// `00be0870` (unnamed, `thiscall`, no stack words; answers a bool).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const CALL_00BE0870: u32 = 0x00be_0870;

// Translated from 00b88670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `skyshader.cpp`): the sky shader's destructor, no stack
/// words. Stores the vtable `010afe18`, drops and clears the 0xb passes at +0x6c, clears
/// (releasing what they hold) the `NiPointer`s +0x98 (0xb) and +0xc4 (8), destroys the arrays
/// +0xc4 (8) and +0x98 (0xb) (destructor `00b66f20`) and +0x6c (0xb, `00b79930`) and runs
/// `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00b88670 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the arrays and the base) is not translated.
pub fn fn_00b88670(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SKY_SHADER_VTABLE_010AFE18);
    for index in 0..0xbu32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..0xbu32 {
        ni_pointer_clear(e, base + 0x98 + 4 * index);
    }
    for index in 0..8u32 {
        ni_pointer_clear(e, base + 0xc4 + 4 * index);
    }
    for (offset, count, destructor) in [
        (0xc4u32, 8u32, DESTRUCT_00B66F20),
        (0x98, 0xb, DESTRUCT_00B66F20),
        (0x6c, 0xb, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The record whose address `00b887b0` answers (`011ff8d8`).
const RECORD_011FF8D8: u32 = 0x011f_f8d8;

// Translated from 00b887b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010afe18` slot +8, where the Xbox PDB's `NiObject` order has
/// `GetRTTI`): answers the address `011ff8d8`. No arguments.
pub fn fn_00b887b0(_e: &mut Engine) -> Ptr {
    Ptr::new(RECORD_011FF8D8)
}

// Translated from 00b89970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010afe18` slot +0x4c), no stack words. Runs `00be0870`,
/// drops and clears the first pass (+0x6c) and answers what `00be0870` answered.
pub fn fn_00b89970(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let result = e.call(CALL_00BE0870, &args![base]).bool();
    pass_pointer_clear(e, base + 0x6c);
    result
}

/// The shader manager's pass state (`0126f74c`, its +0x24 the table of constant blocks), the
/// pass number (`011f91e4`), and the texture-stage calls `00e7de90` and `00e7eb00` (unit
/// `NiXenonRenderer`, unnamed; `thiscall` on a block, one word each).
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
const CURRENT_PASS_011F91E4: u32 = 0x011f_91e4;
const STAGE_CALL_00E7DE90: u32 = 0x00e7_de90;
const STAGE_CALL_00E7EB00: u32 = 0x00e7_eb00;

/// A texture set on a constant block: its +8 the texture, then `00e7de90` with the property's
/// slot +0xcc answer and `00e7eb00` with 0.
fn set_block_texture(e: &mut Engine, block: u32, texture: u32, property: u32) {
    e.mem.set_u32(block + 8, texture);
    let mode = e.vcall(property, 0xcc, &args![]).u32();
    e.call(STAGE_CALL_00E7DE90, &args![block, mode]);
    e.call(STAGE_CALL_00E7EB00, &args![block, 0u32]);
}

// Translated from 00b89cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010afe18` slot +0x78), `thiscall`, `RET 4` (the property
/// state; `this` is not read). With the state's +0xc property: its texture +0x70, when set,
/// goes to the pass state's (`0126f74c`) first constant block; the second block (when there
/// is one) gets the property's +0x80, or when that is null and the pass number (`011f91e4`)
/// is 0x2d4, 0x23c, 0x23a, 0x2d2, 0x23d or 0x23e its +0x70 (when set). Each through
/// `00e7de90` (with the property's slot +0xcc) and `00e7eb00(0)`.
pub fn fn_00b89cc0(e: &mut Engine, _this: Ptr, state: Ptr) {
    let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
    let property = e.mem.u32(state.addr() + 0xc);
    let texture = e.mem.u32(property + 0x70);
    if texture != 0 {
        let block = e.mem.u32(e.mem.u32(pass_state + 0x24));
        set_block_texture(e, block, texture, property);
    }
    let mut texture = e.mem.u32(property + 0x80);
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    if texture == 0 {
        if !matches!(pass, 0x2d4 | 0x23c | 0x23a | 0x2d2 | 0x23d | 0x23e) {
            return;
        }
        texture = e.mem.u32(property + 0x70);
        if texture == 0 {
            return;
        }
    }
    let block = e.mem.u32(e.mem.u32(pass_state + 0x24) + 4);
    if block != 0 {
        set_block_texture(e, block, texture, property);
    }
}

/// The current pass record (`011f91e0`), the default bound (`011f4288`), the shader-manager
/// getter `00b4f5d0` (unit `bsshadermanager.cpp`, no arguments), the camera object
/// (`011f95d8`, its translation +0x8c), the renderer's `00e6fbb0` (`thiscall` on the
/// shader's +0x18, `RET 8`), and the sky constants and inputs.
const CURRENT_RECORD_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const MANAGER_STATE_00B4F5D0: u32 = 0x00b4_f5d0;
const CAMERA_011F95D8: u32 = 0x011f_95d8;
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const SUN_POSITION_011FA2A0: u32 = 0x011f_a2a0;
const ORIGIN_011F474C: u32 = 0x011f_474c;
const EYE_POSITION_011FF8C8: u32 = 0x011f_f8c8;
const SKY_HEIGHT_011FF8A8: u32 = 0x011f_f8a8;
const SKY_DIVISOR_01015A40: u32 = 0x0101_5a40;
const BLEND_VALUES_011FF8B4: u32 = 0x011f_f8b4;
const FOG_SWITCH_011F941E: u32 = 0x011f_941e;
const FOG_VALUE_011AD87C: u32 = 0x011a_d87c;
const COLOR_0_011FF910: u32 = 0x011f_f910;
const COLORS_011FF8E0: u32 = 0x011f_f8e0;
/// The render-state setter `00b97ff0` (unit `bsrenderstate_xenon.cpp`, `cdecl`, three words).
const SET_RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
/// Bits of `1.0f`.
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b89d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010afe18` slot +0x7c), `thiscall`, `RET 4` (the property
/// state). With the current record's geometry: when the shader manager's state (`00b4f5d0`)
/// has its byte +0x164 set, the renderer's `00e6fbb0` gets a stack copy of the world
/// transform moved by minus the camera's (`011f95d8`) translation and plus that of the state's
/// +8 object when there is one; otherwise the world transform itself. The eye position
/// `011ff8c8` becomes the sun position (`011fa2a0`) minus `011f474c` (then 0.0; on pass 5 its
/// z is minus the `float` `011ff8a8` over the `double` `01015a40`). For property kind 3
/// (+0x8c) the shader's +0xe8 and +0xe4 take the property's +0x84 and the blend value
/// `011ff8b4[u16 +0x88]`, otherwise +0xe8 is 0.0; with the byte `011f941e` +0xec takes
/// `011ad87c`. The colours `011ff910` (the compiler's switch on the kind, byte map
/// `00b8a18c`): kinds 0, 1, 3, 5, 6, 7 the property's four words +0x60; kind 2 the twelve
/// words at `011ff8e0`; other kinds 0.0 0.0 0.0 1.0 0.0 0.0 0.0 (the word `011ff92c` left)
/// 0.0 0.0 0.0. Then the slot +0x78 of the shader's +0x30 and +0x34 objects with the pass
/// state's +0x44 and +0x5c objects and the geometry's words, `00b97ff0(6, 0, 0)` for kinds 0
/// and 6, and the shader's +0x1c object's slot +0x24 with the state's +0x10 object when its
/// byte +0x18 has bit 0.
pub fn fn_00b89d80(e: &mut Engine, this: Ptr, state: Ptr) {
    let this = this.addr();
    let state = state.addr();
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
    let geometry = e.mem.u32(record);
    let mut bound = e.mem.u32(geometry + 0x20);
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    let world = geometry + 0x68;
    let passes = e.mem.u32(e.mem.u32(geometry + 0xb8) + 0x34);
    let word_bc = e.mem.u32(geometry + 0xbc);
    let property = e.mem.u32(state + 0xc);
    let manager = e.call(MANAGER_STATE_00B4F5D0, &args![]).u32();
    if e.mem.u8(manager + 0x164) != 0 {
        e.with_stack(0x34, |e, copy| {
            let copy = copy.addr();
            for word in 0..0xdu32 {
                let value = e.mem.u32(world + 4 * word);
                e.mem.set_u32(copy + 4 * word, value);
            }
            let camera = e.global::<u32>(CAMERA_011F95D8);
            let mut moved = [0u32; 3];
            for (index, slot) in moved.iter_mut().enumerate() {
                let offset = 4 * index as u32;
                let value = x87_sub(
                    x87_load32(e.mem.u32(copy + 0x24 + offset)),
                    x87_m32(e.mem.u32(camera + 0x8c + offset)),
                );
                *slot = x87_store32(value);
            }
            let manager = e.call(MANAGER_STATE_00B4F5D0, &args![]).u32();
            let object = e.mem.u32(manager + 8);
            if object != 0 {
                for (index, slot) in moved.iter_mut().enumerate() {
                    let offset = 4 * index as u32;
                    let value = x87_add(
                        x87_load32(e.mem.u32(object + 0x8c + offset)),
                        x87_m32(*slot),
                    );
                    *slot = x87_store32(value);
                }
            }
            for (index, value) in moved.iter().enumerate() {
                e.mem.set_u32(copy + 0x24 + 4 * index as u32, *value);
            }
            let renderer = e.mem.u32(this + 0x18);
            e.call(RENDERER_CALL_00E6FBB0, &args![renderer, copy, 0u32]);
        });
    } else {
        let renderer = e.mem.u32(this + 0x18);
        e.call(RENDERER_CALL_00E6FBB0, &args![renderer, world, 0u32]);
    }
    let mut eye = [0u32; 3];
    for (index, slot) in eye.iter_mut().enumerate() {
        let offset = 4 * index as u32;
        let value = x87_sub(
            x87_load32(e.global::<u32>(SUN_POSITION_011FA2A0 + offset)),
            x87_m32(e.global::<u32>(ORIGIN_011F474C + offset)),
        );
        *slot = x87_store32(value);
    }
    for (index, value) in eye.iter().enumerate() {
        e.set_global::<u32>(
            EYE_POSITION_011FF8C8 + 4 * index as u32,
            x87_quiet_bits(*value),
        );
    }
    e.set_global::<u32>(EYE_POSITION_011FF8C8 + 0xc, 0);
    if pass == 5 {
        let height = x87_load32(e.global::<u32>(SKY_HEIGHT_011FF8A8));
        let divisor = f64::from_bits(e.global::<u64>(SKY_DIVISOR_01015A40));
        let z = x87_store32(x87_div(-height, divisor));
        e.set_global::<u32>(EYE_POSITION_011FF8C8 + 8, z);
    }
    let kind = e.mem.u32(property + 0x8c);
    if kind == 3 {
        let value = x87_quiet_bits(e.mem.u32(property + 0x84));
        e.mem.set_u32(this + 0xe8, value);
        let index = u32::from(e.mem.u16(property + 0x88));
        let blend = x87_quiet_bits(e.global::<u32>(BLEND_VALUES_011FF8B4 + 4 * index));
        e.mem.set_u32(this + 0xe4, blend);
    } else {
        e.mem.set_u32(this + 0xe8, 0);
    }
    if e.global::<u8>(FOG_SWITCH_011F941E) != 0 {
        let value = x87_quiet_bits(e.global::<u32>(FOG_VALUE_011AD87C));
        e.mem.set_u32(this + 0xec, value);
    }
    let kind = e.mem.u32(property + 0x8c);
    // The compiler's switch: byte map `00b8a18c` (0 0 1 0 2 0 0 0) over the jump table
    // `00b8a180`.
    match kind {
        0 | 1 | 3 | 5 | 6 | 7 => {
            for word in 0..4u32 {
                let value = e.mem.u32(property + 0x60 + 4 * word);
                e.set_global::<u32>(COLOR_0_011FF910 + 4 * word, value);
            }
        }
        2 => {
            for word in 0..12u32 {
                let value = e.global::<u32>(COLORS_011FF8E0 + 4 * word);
                e.set_global::<u32>(COLOR_0_011FF910 + 4 * word, value);
            }
        }
        _ => {
            for word in [0u32, 1, 2] {
                e.set_global::<u32>(COLOR_0_011FF910 + 4 * word, 0);
            }
            e.set_global::<u32>(COLOR_0_011FF910 + 0xc, ONE_BITS);
            for word in [4u32, 5, 6, 8, 9, 10] {
                e.set_global::<u32>(COLOR_0_011FF910 + 4 * word, 0);
            }
        }
    }
    for (slot, entry) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let target = e.mem.u32(this + slot);
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
                constants, geometry, word_bc, 0u32, passes, state, 0u32, world, bound, 0u32, 1u32
            ],
        );
    }
    let kind = e.mem.u32(property + 0x8c);
    if kind == 0 || kind == 6 {
        e.call(SET_RENDER_STATE_00B97FF0, &args![6u32, 0u32, 0u32]);
    }
    let object = e.mem.u32(state + 0x10);
    if object != 0 && e.mem.u8(object + 0x18) & 1 != 0 {
        let holder = e.mem.u32(this + 0x1c);
        e.vcall(holder, 0x24, &args![object]);
    }
}

/// The render-state setter `00b98070` (unit `bsrenderstate_xenon.cpp`, `cdecl`, two words).
const SET_RENDER_STATE_00B98070: u32 = 0x00b9_8070;

// Translated from 00b8a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `skyshader.cpp`; vtable `010afe18` slot +0x8c),
/// `thiscall`, `RET 4` (the property state; `this` is not read). For property kinds (+0x8c of
/// the state's +0xc) 0 and 6 `00b97ff0(6, 7, 0)`; when the state's +0x10 object has bit 0 of
/// its byte +0x18, `00b98070(0, 0)`.
pub fn fn_00b8a1a0(e: &mut Engine, _this: Ptr, state: Ptr) {
    let state = state.addr();
    let kind = e.mem.u32(e.mem.u32(state + 0xc) + 0x8c);
    if kind == 0 || kind == 6 {
        e.call(SET_RENDER_STATE_00B97FF0, &args![6u32, 7u32, 0u32]);
    }
    let object = e.mem.u32(state + 0x10);
    if object != 0 && e.mem.u8(object + 0x18) & 1 != 0 {
        e.call(SET_RENDER_STATE_00B98070, &args![0u32, 0u32]);
    }
}

// Translated from 00b8a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `skyshader.cpp`): the scalar deleting destructor (vtable
/// `010afe18` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00b88670`) and, when bit 0
/// of the flags is set, frees the 0xf0 bytes (`00aa1460`). Answers `this`.
pub fn fn_00b8a360(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b88670(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xf0u32]);
    }
    this
}

// Translated from 00b884b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`skyshader.cpp`), `thiscall`, no stack words: calls the shader's
/// vtable +0x128 with each of the eleven pass pointers from +0x6c, in order.
pub fn fn_00b884b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..0xb {
        let pass = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![pass]);
    }
}

// Translated from 00b89960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShader::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: `LoadShaders`, then
/// `LoadStagesAndPasses` (a tail jump; the Xbox PDB declares the method `void`).
pub fn sky_shader_reload_shaders(e: &mut Engine, this: Ptr) {
    sky_shader_load_shaders(e, this);
    sky_shader_load_stages_and_passes(e, this);
}

/// The quad `00b899a0` builds: the corner coordinates (-1.0 at `01012054`, 0.99999899 at
/// `010aff54`, 1.0 from `FLD1`; through `FST`), the colour copied into its four colours
/// (`011a9bd0`, `COLOR_011A9BD0`) and the bound's radius (`01013974`).
const MINUS_ONE_01012054: u32 = 0x0101_2054;
const NEAR_ONE_010AFF54: u32 = 0x010a_ff54;
const RADIUS_01013974: u32 = 0x0101_3974;
/// `vector constructor iterator` (`00401050`, `stdcall`: array, size, count, constructor) and
/// the `NiColorA` constructor it runs (`004a7800`).
const ARRAY_CONSTRUCT_00401050: u32 = 0x0040_1050;
const COLOR_CONSTRUCT_004A7800: u32 = 0x004a_7800;
/// `NiTriShape::NiTriShape` (engine map, `00a74410`, `thiscall` with nine words).
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// `BSShaderData::BSShaderData` (`00baf3c0`) and `BSShaderData::Convert` (`00bb0180`).
const SHADER_DATA_CONSTRUCT: u32 = 0x00ba_f3c0;
const SHADER_DATA_CONVERT: u32 = 0x00bb_0180;
/// The `BSShaderData` local's size on the stack (up to the cookie).
const SHADER_DATA_BYTES: u32 = 0xb8;

// Translated from 00b899a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b899a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`skyshader.cpp`), `cdecl` with one word, an `NiPointer` slot.
/// Allocates (`MemoryManager::Allocate`) four corners (0x30 bytes: (-1, -1, n), (1, -1, n),
/// (1, 1, n), (-1, 1, n) with n = 0.99999899), six triangle indices (0, 1, 2, 3, 0, 2) and
/// four colours (built with `00401050`/`004a7800`, a null block used as it is) copied from
/// `011a9bd0`; makes an `NiTriShape` of 0xc4 bytes (`00aa13e0`; null when the allocation
/// fails) of 4 vertices and 2 triangles and takes a reference. Its bound (+0xb8) gets centre
/// 0.0 and the radius at `01013974`. A local `BSShaderData` (`00baf3c0`, its +0 set to 6 and
/// +0x90 to 4) converts it (`Convert(shape, 1)`); the slot takes the shape and the local
/// reference is released.
pub fn fn_00b899a0(e: &mut Engine, slot: u32) {
    let corners = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
        .u32();
    let minus = x87_quiet_bits(e.global::<u32>(MINUS_ONE_01012054));
    let near = x87_quiet_bits(e.global::<u32>(NEAR_ONE_010AFF54));
    let one = 1.0f32.to_bits();
    for (index, word) in [
        minus, minus, near, one, minus, near, one, one, near, minus, one, near,
    ]
    .iter()
    .enumerate()
    {
        e.mem.set_u32(corners + 4 * index as u32, *word);
    }
    let indices = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    for (index, value) in [0u16, 1, 2, 3, 0, 2].iter().enumerate() {
        e.mem.set_u16(indices + 2 * index as u32, *value);
    }
    let colours = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    if colours != 0 {
        e.call(
            ARRAY_CONSTRUCT_00401050,
            &args![colours, 0x10u32, 4u32, COLOR_CONSTRUCT_004A7800],
        );
    }
    for colour in 0..4 {
        for word in 0..4 {
            let value = e.global::<u32>(COLOR_011A9BD0 + 4 * word);
            e.mem.set_u32(colours + 0x10 * colour + 4 * word, value);
        }
    }
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            NI_TRI_SHAPE_CONSTRUCT,
            &args![block, 4u32, corners, 0u32, colours, 0u32, 0u32, 0u32, 2u32, indices],
        )
        .u32()
    } else {
        0
    };
    if shape != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![shape + 4]);
    }
    let bound = e.mem.u32(shape.wrapping_add(0xb8));
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_f32(bound + offset, 0.0);
    }
    let radius = x87_quiet_bits(e.global::<u32>(RADIUS_01013974));
    e.mem.set_u32(bound + 0x1c, radius);
    e.with_stack(SHADER_DATA_BYTES, |e, data| {
        let data = data.addr();
        e.call(SHADER_DATA_CONSTRUCT, &args![data]);
        e.mem.set_u32(data, 6);
        e.mem.set_u32(data + 0x90, 4);
        e.call(SHADER_DATA_CONVERT, &args![data, shape, 1u32]);
    });
    ni_pointer_assign(e, slot, shape);
    release_reference(e, shape);
}
/// `00b884e0`'s jump table for the passes 0x23a to 0x2d5 (the byte table at `00b885d0` into the
/// 10 code addresses at `00b885a8`), read from the exe.
const TARGETS_00B885A8: [u32; 10] = [
    0x00b8_859a,
    0x00b8_8565,
    0x00b8_858b,
    0x00b8_8571,
    0x00b8_857d,
    0x00b8_855a,
    0x00b8_852e,
    0x00b8_853c,
    0x00b8_854b,
    0x00b8_85a3,
];
const BYTES_00B885D0: [u8; 156] = [
    0, 1, 2, 3, 4, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 5, 6, 7, 8,
];
const CASES_00B884E0: [(u32, u32); 11] = [
    (0x00b8_84f6, 0x6c),
    (0x00b8_8501, 0x88),
    (0x00b8_852e, 0x90),
    (0x00b8_853c, 0x94),
    (0x00b8_854b, 0x8c),
    (0x00b8_855a, 0x70),
    (0x00b8_8565, 0x78),
    (0x00b8_8571, 0x7c),
    (0x00b8_857d, 0x80),
    (0x00b8_858b, 0x84),
    (0x00b8_859a, 0x74),
];

// Translated from 00b884e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00b884e0 to the `RET 4` at 00b885a3, its jump tables after
/// it): slot 60 (+0xf0) of the sky shader's vtable `010afe18`, `thiscall` with one stack word: the
/// pass. Pass 9 puts the shader's +0x6c in `0126f74c`, 0x239 its +0x88, and the passes 0x23a to
/// 0x2d5 with a case in the jump table (read from the exe) their word (+0x70 to +0x94); other
/// passes: nothing.
pub fn fn_00b884e0(e: &mut Engine, this: Ptr, pass: u32) {
    let target = if pass > 0x239 {
        let case = pass.wrapping_sub(0x23a);
        if case > 0x9b {
            return;
        }
        TARGETS_00B885A8[usize::from(BYTES_00B885D0[case as usize])]
    } else if pass == 0x239 {
        0x00b8_8501
    } else if pass == 9 {
        0x00b8_84f6
    } else {
        return;
    };
    // 00b885a3, the default, only returns.
    let Some(offset) = CASES_00B884E0
        .iter()
        .find(|(address, _)| *address == target)
        .map(|(_, offset)| *offset)
    else {
        return;
    };
    let word = e.mem.u32(this.addr() + offset);
    e.set_global::<u32>(PASS_STATE_0126F74C, word);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_84b0, fn_00b884b0(Ptr)),
        entry!(0x00b8_84e0, fn_00b884e0(Ptr, u32)),
        entry!(0x00b8_9960, sky_shader_reload_shaders(Ptr)),
        entry!(0x00b8_99a0, fn_00b899a0(u32)),
        entry!(0x00b8_8670, fn_00b88670(Ptr)),
        entry!(0x00b8_87b0, fn_00b887b0() -> Ptr),
        entry!(0x00b8_9970, fn_00b89970(Ptr) -> bool),
        entry!(0x00b8_9cc0, fn_00b89cc0(Ptr, Ptr)),
        entry!(0x00b8_9d80, fn_00b89d80(Ptr, Ptr)),
        entry!(0x00b8_a1a0, fn_00b8a1a0(Ptr, Ptr)),
        entry!(0x00b8_a360, fn_00b8a360(Ptr, u32) -> Ptr),
        entry!(0x00b8_87c0, sky_shader_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00b8_74f0, sky_shader_load_shaders(Ptr)),
        entry!(0x00b8_a1f0, sky_shader_sky_shader(Ptr) -> Ptr),
        entry!(0x00b8_a390, sky_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn sky_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00b8_a390);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xf0]),
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
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn sky_shader_constructor_builds_the_shader_and_copies_the_color() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        for word in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BD0 + 4 * word, 0x30 + word);
        }
        let this = e.mem.alloc(0xe4);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SKY_SHADER_VTABLE_010AFE18);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0xb, DESTRUCT_00B79930),
                pointer_array(this + 0x98, 0xb, DESTRUCT_00B66F20),
                pointer_array(this + 0xc4, 8, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0xb);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        for block in [
            0x011f_f8c8u32,
            0x011f_f8e0,
            0x011f_f8f0,
            0x011f_f900,
            0x011f_f910,
            0x011f_f920,
            0x011f_f930,
        ] {
            for word in 0..4u32 {
                assert_eq!(e.global::<u32>(block + 4 * word), 0x30 + word);
            }
        }
        assert_eq!(e.global::<u32>(0x011f_f8d8), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const VERTEX_PROFILE: u32 = 0x0f40_0000;
    const PIXEL_PROFILE: u32 = 0x0f40_0010;

    #[test]
    fn sky_shader_load_shaders_compiles_eighteen_shaders() {
        let mut e = loader_rig();
        e.register(VERTEX_PROFILE_00B4F3E0, |_, _| VERTEX_PROFILE.into_ret());
        e.register(SKY_PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a[0], 0);
            PIXEL_PROFILE.into_ret()
        });
        e.register(GCVT, |e, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            assert_eq!(a[2], 0xc);
            e.mem.set_u32(a[3], value as u32);
            Ret::default()
        });
        e.map(0x011f_f000, 0x1000);
        e.set_global::<f32>(SUN_VALUE_011FF8AC, 3.0);
        e.set_global::<f32>(SI_VALUE_011FF8B0, 5.0);
        let this = e.mem.alloc(0xe4);
        e.call(LOAD_SHADERS, &args![this]);
        let mut slots: Vec<u32> = SKY_STEPS.iter().map(|s| s.slot).collect();
        slots.sort_unstable();
        assert_eq!(
            slots,
            (0x98u32..=0xe0)
                .step_by(4)
                .filter(|s| *s != 0xa4)
                .collect::<Vec<_>>()
        );
        for step in &SKY_STEPS {
            let seen = compiled(&e, e.mem.u32(this + step.slot));
            assert_eq!(seen.output, step.output);
            assert_eq!(seen.pixel, step.pixel);
            let profile = if step.pixel {
                PIXEL_PROFILE
            } else {
                VERTEX_PROFILE
            };
            assert_eq!(seen.profile, profile);
        }
        // The cloud-fade vertex shader takes the file of the clouds block and the macros of
        // its own full block.
        let fade = compiled(&e, e.mem.u32(this + 0xb4));
        assert_eq!(fade.file, SKY_VERTEX_SOURCE_010AFE04);
        assert_eq!(
            fade.macros,
            macro_words(&[
                (MACRO_HORIZFADE_010AFDB8, 0),
                (SKY_MACRO_TEX_010AECAC, 0),
                (MACRO_CLOUDS_010AFD88, 0)
            ])
        );
        let quad = compiled(&e, e.mem.u32(this + 0xc0));
        assert_eq!(quad.file, SKY_QUAD_VERTEX_SOURCE_010AFCBC);
        // The SI pixel shaders see the number buffer's address as the SI value: the compile
        // double kept the macro words, so the value is a frame address holding the number.
        let si = compiled(&e, e.mem.u32(this + 0xe0));
        assert_eq!(si.macros[0], MACRO_SI_010AEC48);
        assert_ne!(si.macros[1], 0);
        let gcvt = calls(&e, GCVT);
        assert_eq!(gcvt.len(), 3);
        let bits = 3.0f64.to_bits();
        assert_eq!(gcvt[0][..2], [bits as u32, (bits >> 32) as u32]);
        assert_eq!(gcvt[1][3], si.macros[1]);
        assert_eq!(calls(&e, MEMSET).len(), 16);
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn sky_shader_load_stages_and_passes_builds_the_missing_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xe4);
        for offset in (0x98u32..=0xe0).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        // One pass exists already: it only gets its shaders.
        let kept = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x88, kept);
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        let made: Vec<u32> = calls(&e, NEW_PASS_00E804D0).iter().map(|a| a[0]).collect();
        let order = [
            0x6cu32, 0x8c, 0x90, 0x94, 0x70, 0x78, 0x7c, 0x80, 0x84, 0x74,
        ];
        assert_eq!(made, order.iter().map(|o| this + o).collect::<Vec<_>>());
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 16);
        let setups: Vec<u32> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(setups, vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 61);
        let first = e.mem.u32(this + 0x6c);
        assert_eq!(
            render_states(&e, first),
            vec![(0x1b, 1), (0x13, 2), (0x14, 2), (0xe, 0), (0xa8, 0)]
        );
        assert_eq!(e.mem.u32(first + 0x5c), e.mem.u32(this + 0xac));
        assert_eq!(e.mem.u32(first + 0x44), e.mem.u32(this + 0xd0));
        let fade = e.mem.u32(this + 0x74);
        assert_eq!(
            render_states(&e, fade),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x13, 1),
                (0x14, 2)
            ]
        );
        assert_eq!(e.mem.u32(kept + 0x5c), e.mem.u32(this + 0x98));
        assert_eq!(e.mem.u32(kept + 0x44), e.mem.u32(this + 0xc4));
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != kept));
    }
}

#[cfg(test)]
mod sky_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

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
            0x0101_5000u32,
            0x011a_d000,
            0x011f_4000,
            0x011f_9000,
            0x011f_a000,
            0x011f_f000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
    }

    #[test]
    fn fn_00b88670_drops_the_passes_and_destroys() {
        let mut e = rig();
        noop(&mut e, &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0xf0);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x6c + 4 * 10, pass);
        let shader = object(&mut e, 1);
        e.mem.set_u32(this + 0xc4 + 4 * 7, shader);
        e.call(0x00b8_8670, &args![this]);
        assert_eq!(e.mem.u32(this), SKY_SHADER_VTABLE_010AFE18);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shader]]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0xc4, 4, 8, DESTRUCT_00B66F20],
                vec![this + 0x98, 4, 0xb, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 0xb, DESTRUCT_00B79930],
            ]
        );
    }

    #[test]
    fn fn_00b8a360_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(
            &mut e,
            &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460],
        );
        let this = e.mem.alloc(0xf0);
        assert_eq!(e.call(0x00b8_a360, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xf0]]);
        e.call(0x00b8_a360, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00b887b0_answers_its_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b8_87b0, &args![]).u32(), RECORD_011FF8D8);
    }

    #[test]
    fn fn_00b89970_drops_the_first_pass_and_answers_the_base() {
        let mut e = rig();
        e.register(CALL_00BE0870, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0xf0);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x6c, pass);
        assert!(e.call(0x00b8_9970, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
    }

    /// The pass state with two constant blocks, a property state whose +0xc property (vtable
    /// 1) answers 9 from its slot +0xcc.
    fn texture_rig(pass: u32) -> (Engine, u32, u32, [u32; 2]) {
        let mut e = rig();
        map_globals(&mut e);
        let pass_state = e.mem.alloc(0x60);
        let table = e.mem.alloc(8);
        let blocks = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        e.mem.set_u32(table, blocks[0]);
        e.mem.set_u32(table + 4, blocks[1]);
        e.mem.set_u32(pass_state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        e.set_global(CURRENT_PASS_011F91E4, pass);
        let property = e.mem.alloc(0x90);
        let table1 = vtable(&mut e, 1);
        e.mem.set_u32(property, table1);
        e.register(slot(1, 0xcc), |_, _| 9u32.into_ret());
        noop(&mut e, &[STAGE_CALL_00E7DE90, STAGE_CALL_00E7EB00]);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        (e, state, property, blocks)
    }

    #[test]
    fn fn_00b89cc0_sets_the_sky_textures() {
        let (mut e, state, property, blocks) = texture_rig(0x10);
        e.mem.set_u32(property + 0x70, 0x7070);
        e.mem.set_u32(property + 0x80, 0x8080);
        e.call(0x00b8_9cc0, &args![0u32, state]);
        assert_eq!(e.mem.u32(blocks[0] + 8), 0x7070);
        assert_eq!(e.mem.u32(blocks[1] + 8), 0x8080);
        assert_eq!(
            calls(&e, STAGE_CALL_00E7DE90),
            vec![vec![blocks[0], 9], vec![blocks[1], 9]]
        );
        assert_eq!(
            calls(&e, STAGE_CALL_00E7EB00),
            vec![vec![blocks[0], 0], vec![blocks[1], 0]]
        );
        // No +0x80 texture: the +0x70 one is used again only on the listed passes.
        let (mut e, state, property, blocks) = texture_rig(0x23d);
        e.mem.set_u32(property + 0x70, 0x7070);
        e.call(0x00b8_9cc0, &args![0u32, state]);
        assert_eq!(e.mem.u32(blocks[1] + 8), 0x7070);
        let (mut e, state, property, blocks) = texture_rig(0x23b);
        e.mem.set_u32(property + 0x70, 0x7070);
        e.call(0x00b8_9cc0, &args![0u32, state]);
        assert_eq!(e.mem.u32(blocks[1] + 8), 0);
        assert_eq!(calls(&e, STAGE_CALL_00E7DE90).len(), 1);
    }

    /// A rig for `00b89d80`: the record's geometry, a shader manager state, a camera and the
    /// sky property of `kind`.
    fn draw_rig(kind: u32, moved: u8) -> (Engine, u32, u32, u32, u32) {
        let mut e = rig();
        map_globals(&mut e);
        let this = e.mem.alloc(0xf0);
        e.mem.set_u32(this + 0x18, 0x1818);
        let record = e.mem.alloc(0x10);
        let geometry = e.mem.alloc(0xc0);
        let passes = e.mem.alloc(0x38);
        e.mem.set_u32(passes + 0x34, 3);
        e.mem.set_u32(geometry + 0xb8, passes);
        e.mem.set_u32(geometry + 0xbc, 0xbcbc);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(geometry + 0x8c + 4 * i as u32, *v);
        }
        e.mem.set_u32(record, geometry);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        let manager = e.mem.alloc(0x168);
        e.mem.set_u8(manager + 0x164, moved);
        e.register_double(MANAGER_STATE_00B4F5D0, move |_, _| manager.into_ret());
        let camera = e.mem.alloc(0x98);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(camera + 0x8c + 4 * i as u32, *v);
        }
        e.set_global(CAMERA_011F95D8, camera);
        for (i, v) in [5.0f32, 6.0, 7.0].iter().enumerate() {
            e.set_global(SUN_POSITION_011FA2A0 + 4 * i as u32, *v);
            e.set_global(ORIGIN_011F474C + 4 * i as u32, 1.0f32);
        }
        let property = e.mem.alloc(0x90);
        e.mem.set_u32(property + 0x8c, kind);
        e.mem.set_f32(property + 0x84, 0.5);
        e.mem.set_u16(property + 0x88, 1);
        for i in 0..4u32 {
            e.mem.set_u32(property + 0x60 + 4 * i, 0x600 + i);
        }
        e.set_global(BLEND_VALUES_011FF8B4 + 4, 0.25f32);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        let pass_state = e.mem.alloc(0x60);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        noop(&mut e, &[RENDERER_CALL_00E6FBB0, SET_RENDER_STATE_00B97FF0]);
        (e, this, state, geometry, property)
    }

    #[test]
    fn fn_00b89d80_moves_the_world_by_the_camera_and_sets_the_colours() {
        let (mut e, this, state, geometry, _) = draw_rig(3, 1);
        e.register(RENDERER_CALL_00E6FBB0, |e, a| {
            // The copy's translation: world minus the camera.
            let moved: Vec<f32> = (0..3).map(|i| e.mem.f32(a[1] + 0x24 + 4 * i)).collect();
            assert_eq!(moved, vec![9.0, 18.0, 27.0]);
            Ret::default()
        });
        e.call(0x00b8_9d80, &args![this, state]);
        let call = calls(&e, RENDERER_CALL_00E6FBB0);
        assert_ne!(call[0][1], geometry + 0x68);
        let eye: Vec<f32> = (0..4)
            .map(|i| e.global::<f32>(EYE_POSITION_011FF8C8 + 4 * i))
            .collect();
        assert_eq!(eye, vec![4.0, 5.0, 6.0, 0.0]);
        assert_eq!(e.mem.f32(this + 0xe8), 0.5);
        assert_eq!(e.mem.f32(this + 0xe4), 0.25);
        let colours: Vec<u32> = (0..4)
            .map(|i| e.global::<u32>(COLOR_0_011FF910 + 4 * i))
            .collect();
        assert_eq!(colours, vec![0x600, 0x601, 0x602, 0x603]);
        assert!(calls(&e, SET_RENDER_STATE_00B97FF0).is_empty());
    }

    #[test]
    fn fn_00b89d80_plain_world_and_the_other_kinds() {
        let (mut e, this, state, geometry, _) = draw_rig(4, 0);
        e.set_global(CURRENT_PASS_011F91E4, 5u32);
        e.set_global(SKY_HEIGHT_011FF8A8, 7.0f32);
        e.set_global(SKY_DIVISOR_01015A40, 2.0f64);
        e.set_global(COLOR_0_011FF910 + 0x1c, 0x1234u32);
        e.mem.set_f32(this + 0xe8, 9.0);
        e.call(0x00b8_9d80, &args![this, state]);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0),
            vec![vec![0x1818, geometry + 0x68, 0]]
        );
        assert_eq!(e.global::<f32>(EYE_POSITION_011FF8C8 + 8), -3.5);
        assert_eq!(e.mem.f32(this + 0xe8), 0.0);
        let colours: Vec<u32> = (0..12)
            .map(|i| e.global::<u32>(COLOR_0_011FF910 + 4 * i))
            .collect();
        assert_eq!(
            colours,
            vec![0, 0, 0, ONE_BITS, 0, 0, 0, 0x1234, 0, 0, 0, 0]
        );
        // Kind 2 copies the twelve words; kind 0 sets the render state.
        let (mut e, this, state, _, _) = draw_rig(2, 0);
        for i in 0..12u32 {
            e.set_global(COLORS_011FF8E0 + 4 * i, 0x900 + i);
        }
        e.call(0x00b8_9d80, &args![this, state]);
        assert_eq!(e.global::<u32>(COLOR_0_011FF910 + 0x2c), 0x90b);
        let (mut e, this, state, _, _) = draw_rig(0, 0);
        let holder = e.mem.alloc(8);
        let table = vtable(&mut e, 2);
        e.mem.set_u32(holder, table);
        e.mem.set_u32(this + 0x1c, holder);
        noop(&mut e, &[slot(2, 0x24)]);
        let object = e.mem.alloc(0x1c);
        e.mem.set_u8(object + 0x18, 1);
        e.mem.set_u32(state + 0x10, object);
        e.call(0x00b8_9d80, &args![this, state]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00B97FF0), vec![vec![6, 0, 0]]);
        assert_eq!(calls(&e, slot(2, 0x24)), vec![vec![holder, object]]);
    }

    #[test]
    fn fn_00b8a1a0_sets_the_render_states() {
        let mut e = rig();
        noop(
            &mut e,
            &[SET_RENDER_STATE_00B97FF0, SET_RENDER_STATE_00B98070],
        );
        let property = e.mem.alloc(0x90);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        e.mem.set_u32(property + 0x8c, 6);
        let object = e.mem.alloc(0x1c);
        e.mem.set_u8(object + 0x18, 3);
        e.mem.set_u32(state + 0x10, object);
        e.call(0x00b8_a1a0, &args![0u32, state]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00B97FF0), vec![vec![6, 7, 0]]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00B98070), vec![vec![0, 0]]);
        e.mem.set_u32(property + 0x8c, 1);
        e.mem.set_u8(object + 0x18, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b8_a1a0, &args![0u32, state]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{order, slot_target, vobject};

    #[test]
    fn fn_00b884b0_hands_each_pass_to_the_shader() {
        let mut e = rig();
        let this = vobject(&mut e, 0xe4, &[]);
        for index in 0..0xb {
            e.mem.set_u32(this + 0x6c + 4 * index, 0x100 + index);
        }
        e.call(0x00b8_84b0, &args![this]);
        let seen = calls(&e, slot_target(&e, this, 0x128));
        let expected: Vec<Vec<u32>> = (0..0xb).map(|i| vec![this, 0x100 + i]).collect();
        assert_eq!(seen, expected);
    }

    #[test]
    fn sky_shader_reload_shaders_loads_then_builds_the_passes() {
        let mut e = pass_rig();
        e.register(VERTEX_PROFILE_00B4F3E0, |_, _| 0x0f40_0000u32.into_ret());
        e.register(SKY_PIXEL_PROFILE_00B4F380, |_, _| 0x0f40_0010u32.into_ret());
        e.register(GCVT, |_, _| Ret::default());
        e.map(0x011f_f000, 0x1000);
        let this = e.mem.alloc(0xe4);
        e.call(0x00b8_9960, &args![this]);
        let log = order(&e);
        let compiles = log
            .iter()
            .filter(|a| **a == COMPILE_VERTEX_SHADER || **a == COMPILE_PIXEL_SHADER)
            .count();
        assert_eq!(compiles, 18);
        let last_compile = log
            .iter()
            .rposition(|a| *a == COMPILE_VERTEX_SHADER || *a == COMPILE_PIXEL_SHADER)
            .unwrap();
        let first_pass = log.iter().position(|a| *a == NEW_PASS_00E804D0).unwrap();
        assert!(last_compile < first_pass);
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 11);
    }

    #[test]
    fn fn_00b899a0_builds_the_quad_and_stores_it() {
        let mut e = rig();
        for page in [0x0101_2000u32, 0x0101_3000, 0x010a_f000, 0x011a_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE_01012054, -1.0f32);
        e.set_global(NEAR_ONE_010AFF54, 0.999_999f32);
        e.set_global(RADIUS_01013974, 1000.0f32);
        for word in 0..4u32 {
            e.set_global(COLOR_011A9BD0 + 4 * word, 0x30 + word);
        }
        e.register(ARRAY_CONSTRUCT_00401050, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_TRI_SHAPE_CONSTRUCT, |e, a| {
            let bound = e.mem.alloc(0x20);
            e.mem.set_u32(a[0], REFERENCE_VTABLE);
            e.mem.set_u32(a[0] + 0xb8, bound);
            a[0].into_ret()
        });
        e.register(SHADER_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(SHADER_DATA_CONVERT, |e, a| {
            assert_eq!((e.mem.u32(a[0]), e.mem.u32(a[0] + 0x90)), (6, 4));
            Ret::default()
        });
        let old = object(&mut e, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        e.call(0x00b8_99a0, &args![slot]);
        let shape = e.mem.u32(slot);
        let made = calls(&e, NI_TRI_SHAPE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let words = &made[0];
        assert_eq!(words[0], shape);
        assert_eq!(
            (words[1], words[3], words[5], words[6], words[7], words[8]),
            (4, 0, 0, 0, 0, 2)
        );
        let corners: Vec<f32> = (0..12).map(|i| e.mem.f32(words[2] + 4 * i)).collect();
        let n = 0.999_999f32;
        assert_eq!(
            corners,
            vec![-1.0, -1.0, n, 1.0, -1.0, n, 1.0, 1.0, n, -1.0, 1.0, n]
        );
        let indices: Vec<u16> = (0..6).map(|i| e.mem.u16(words[9] + 2 * i)).collect();
        assert_eq!(indices, vec![0, 1, 2, 3, 0, 2]);
        assert_eq!(e.mem.u32(words[4] + 0x34), 0x31);
        assert_eq!(
            calls(&e, ARRAY_CONSTRUCT_00401050),
            vec![vec![words[4], 0x10, 4, COLOR_CONSTRUCT_004A7800]]
        );
        let bound = e.mem.u32(shape + 0xb8);
        assert_eq!(e.mem.f32(bound + 0x1c), 1000.0);
        assert_eq!(
            calls(&e, SHADER_DATA_CONVERT)[0][1..].to_vec(),
            vec![shape, 1]
        );
        // The slot holds the one reference left; the old object was released.
        assert_eq!(e.mem.u32(shape + 4), 1);
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00b884e0_puts_the_pass_word_in_the_state() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0xa0);
        for offset in (0x6cu32..0xa0).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        for (pass, offset) in [
            (9u32, 0x6cu32),
            (0x239, 0x88),
            (0x23a, 0x74),
            (0x23b, 0x78),
            (0x23c, 0x84),
            (0x2d5, 0x8c),
        ] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 0);
            e.call(0x00b8_84e0, &args![this, pass]);
            assert_eq!(
                e.global::<u32>(PASS_STATE_0126F74C),
                0x1000 + offset,
                "{pass:#x}"
            );
        }
        for pass in [8u32, 10, 0x238, 0x23f, 0x29e, 0x2d6] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 7);
            e.call(0x00b8_84e0, &args![this, pass]);
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 7, "{pass:#x}");
        }
    }
}
