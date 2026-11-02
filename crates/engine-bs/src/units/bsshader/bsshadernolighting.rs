//! `BSShader/bsshadernolighting.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `BSShaderNoLighting::BSShaderNoLighting` (`00bc8f80`, `thiscall`, `RET 0x30`: twelve
/// declarations), `::LoadShaders` (`00bc5c40`) and `::LoadStagesAndPasses` (`00bc77e0`; both
/// `thiscall`, no words) (engine map).
const CONSTRUCT: u32 = 0x00bc_8f80;
const LOAD_SHADERS: u32 = 0x00bc_5c40;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bc_77e0;

/// Each declaration's entry count as made (`Create`'s second word) and the entries it gets
/// through vtable slot +0x8c (five words each), in the exe's order.
const DECLARATIONS: [(u32, &[[u32; 5]]); 12] = [
    (2, &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]]),
    (2, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]]),
    (3, &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]]),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 3, 2, 0],
            [2, 5, 4, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 7, 5, 1, 0],
        ],
    ),
    (
        4,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 5, 4, 4, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 7, 5, 1, 0],
        ],
    ),
    (
        5,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
        ],
    ),
    (
        6,
        &[
            [0, 0, 0, 2, 0],
            [1, 1, 1, 3, 0],
            [2, 2, 2, 4, 0],
            [3, 1, 3, 2, 0],
            [4, 5, 4, 4, 0],
            [5, 7, 5, 1, 0],
        ],
    ),
];

// Translated from 00bc93c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x21. No arguments. Makes twelve declarations, one after the other
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508` read each time, entry
/// counts 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 1 stream each), then in the same order sets
/// each one's entries through its vtable slot +0x8c (the lists of `DECLARATIONS`, read from
/// the pushes) and ends it (slot +0xb4). Allocates 0x1b4 bytes and runs the constructor
/// (`00bc8f80`) with the twelve declarations in order (a null allocation is used as it is),
/// calls the shader's slot +0x4c, `LoadShaders` (`00bc5c40`), `LoadStagesAndPasses`
/// (`00bc77e0`), slot +0x11c, then +0xc0 with the first declaration, and answers the shader.
// Translation incomplete: 00bc93c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn bs_shader_no_lighting_create_shader(e: &mut Engine) -> Ptr {
    let mut made = Vec::with_capacity(DECLARATIONS.len());
    for (count, _) in DECLARATIONS {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        made.push(
            e.call(DECLARATION_CREATE, &args![renderer, count, 1u32])
                .u32(),
        );
    }
    for (declaration, (_, entries)) in made.iter().zip(DECLARATIONS) {
        for entry in entries {
            e.vcall(*declaration, 0x8c, entry);
        }
        e.vcall(*declaration, 0xb4, &args![]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x1b4u32]).u32();
    let shader = if block != 0 {
        let mut words = vec![block];
        words.extend_from_slice(&made);
        e.call(CONSTRUCT, &words).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![made[0]]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bb230`; no RTTI locator before it).
const NO_LIGHTING_VTABLE_010BB230: u32 = 0x010b_b230;
/// The four words copied (`011a9be0` into `01202010`).
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const COLOR_01202010: u32 = 0x0120_2010;

// Translated from 00bc8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::BSShaderNoLighting` (Xbox PDB), `thiscall` with twelve stack words
/// (`RET 0x30`), the twelve declarations. Runs `BSShader::BSShader` (`00be2200`), stores the
/// vtable `010bb230`, constructs the `NiPointer` arrays at +0x6c (0x1e, destructor
/// `00b79930`), +0xe4 (0x1c) and +0x154 (7; destructor `00b66f20`), clears +0x184 to +0x1b0,
/// sets +0x68 to 0x16, assigns the declarations in order to the `NiPointer`s at +0x184 to
/// +0x1b0, copies the four words at `011a9be0` to `01202010` and clears +0x170. Answers
/// `this`.
// Translation incomplete: 00bc8f80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xf for the base and the arrays) is not translated.
pub fn bs_shader_no_lighting_bs_shader_no_lighting(
    e: &mut Engine,
    this: Ptr,
    declaration_0: Ptr,
    declaration_1: Ptr,
    declaration_2: Ptr,
    declaration_3: Ptr,
    declaration_4: Ptr,
    declaration_5: Ptr,
    declaration_6: Ptr,
    declaration_7: Ptr,
    declaration_8: Ptr,
    declaration_9: Ptr,
    declaration_10: Ptr,
    declaration_11: Ptr,
) -> Ptr {
    let declarations = [
        declaration_0,
        declaration_1,
        declaration_2,
        declaration_3,
        declaration_4,
        declaration_5,
        declaration_6,
        declaration_7,
        declaration_8,
        declaration_9,
        declaration_10,
        declaration_11,
    ];
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NO_LIGHTING_VTABLE_010BB230);
    construct_pointer_array(e, base + 0x6c, 0x1e, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0xe4, 0x1c, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x154, 7, DESTRUCT_00B66F20);
    for index in 0..12u32 {
        e.mem.set_u32(base + 0x184 + 4 * index, 0);
    }
    e.mem.set_u32(base + 0x68, 0x16);
    for (index, declaration) in declarations.iter().enumerate() {
        ni_pointer_assign(e, base + 0x184 + 4 * index as u32, declaration.addr());
    }
    for word in [0u32, 8, 4, 0xc] {
        let value = e.global::<u32>(COLOR_011A9BE0 + word);
        e.set_global::<u32>(COLOR_01202010 + word, value);
    }
    e.mem.set_u32(base + 0x170, 0);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): the HLSL files `"lighting\1x\v\NoLighting.v.hlsl"`
/// (`010bb20c`), `"lighting\1x\v\particlesystem.v.hlsl"` (`010bb1e0`),
/// `"lighting\2x\v\waterfog.v.hlsl"` (`010bb1a8`), `"lighting\1x\p\NoLighting.p.hlsl"`
/// (`010bb178`) and `"lighting\2x\p\waterfog.p.hlsl"` (`010bb148`); the macros `"TEX"`,
/// `"VC"`, `"FALLOFF"`, `"SKIN"`, `"PSYS_SUBTEX"`, `"PSYS_STRIP"`, `"TEXCOORD"`,
/// `"PREMULT_ALPHA"`, `"ALPHATESTING"`, each with the value `""` (`01011584`); the vertex
/// name format `"NOLIGHT%03i.vso"` (`010bb198`) and profile `"vs_2_0"` (`010a93e4`); the pixel
/// names `"NOLIGHTTEX.pso"`, `"NOLIGHTVCONLY.pso"`, `"NOLIGHTTEXVC.pso"`,
/// `"NOLIGHTTEXVCPMA.pso"`, `"NOLIGHTMAT.pso"`, `"NOLIGHTWF.pso"`, `"NOLIGHTWFAt.pso"` and the
/// pixel profile `"ps_2_a"` (`010a93c4`).
const NL_VERTEX_SOURCE_010BB20C: u32 = 0x010b_b20c;
const PARTICLE_VERTEX_SOURCE_010BB1E0: u32 = 0x010b_b1e0;
const WATERFOG_VERTEX_SOURCE_010BB1A8: u32 = 0x010b_b1a8;
const NL_PIXEL_SOURCE_010BB178: u32 = 0x010b_b178;
const WATERFOG_PIXEL_SOURCE_010BB148: u32 = 0x010b_b148;
const MACRO_TEX_010AECAC: u32 = 0x010a_ecac;
const MACRO_VC_010AED64: u32 = 0x010a_ed64;
const MACRO_FALLOFF_010BB204: u32 = 0x010b_b204;
const MACRO_SKIN_010AEC10: u32 = 0x010a_ec10;
const MACRO_PSYS_SUBTEX_010BB1D4: u32 = 0x010b_b1d4;
const MACRO_PSYS_STRIP_010BB1C8: u32 = 0x010b_b1c8;
const MACRO_TEXCOORD_010AE930: u32 = 0x010a_e930;
const MACRO_PREMULT_ALPHA_010BB168: u32 = 0x010b_b168;
const MACRO_ALPHATESTING_010AEF00: u32 = 0x010a_ef00;
const NL_EMPTY_STRING_01011584: u32 = 0x0101_1584;
const VERTEX_NAME_FORMAT_010BB198: u32 = 0x010b_b198;
const PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
const NL_PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;
/// The compiled names of the seven pixel shaders, in compile order.
const PIXEL_OUTPUTS: [u32; 7] = [
    0x010b_b138,
    0x010b_b124,
    0x010b_b110,
    0x010b_b0fc,
    0x010b_b0ec,
    0x010b_b0dc,
    0x010b_b0cc,
];
/// The `LoadShaders` frame (0xc90 bytes up to the return address) and its buffers.
const NL_FRAME_BYTES: u32 = 0xc90;
const NL_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xb88,
    path: 0xa84,
};
/// The 0x1c vertex source blocks, 0x4c bytes apart from +0x19c (the loop's order).
const NL_VERTEX_BLOCKS: [ShaderSource; 28] = [
    ShaderSource {
        offset: 0x19c,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1e8,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x234,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x280,
        words: &[NL_VERTEX_SOURCE_010BB20C, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2cc,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x318,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x364,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3b0,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3fc,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x448,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x494,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x4e0,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x52c,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x578,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5c4,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
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
        offset: 0x610,
        words: &[
            NL_VERTEX_SOURCE_010BB20C,
            MACRO_FALLOFF_010BB204,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x65c,
        words: &[PARTICLE_VERTEX_SOURCE_010BB1E0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x6a8,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_SUBTEX_010BB1D4,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6f4,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_STRIP_010BB1C8,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x740,
        words: &[
            PARTICLE_VERTEX_SOURCE_010BB1E0,
            MACRO_PSYS_STRIP_010BB1C8,
            NL_EMPTY_STRING_01011584,
            MACRO_PSYS_SUBTEX_010BB1D4,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x78c,
        words: &[WATERFOG_VERTEX_SOURCE_010BB1A8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x7d8,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x824,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x870,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x8bc,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x908,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x954,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x9a0,
        words: &[
            WATERFOG_VERTEX_SOURCE_010BB1A8,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            MACRO_TEXCOORD_010AE930,
            NL_EMPTY_STRING_01011584,
            MACRO_SKIN_010AEC10,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
];
/// The seven pixel source blocks, in the order the exe fills and compiles them.
const NL_PIXEL_BLOCKS: [ShaderSource; 7] = [
    ShaderSource {
        offset: 0xb8,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x150,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6c,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x20,
        words: &[
            NL_PIXEL_SOURCE_010BB178,
            MACRO_PREMULT_ALPHA_010BB168,
            NL_EMPTY_STRING_01011584,
            MACRO_TEX_010AECAC,
            NL_EMPTY_STRING_01011584,
            MACRO_VC_010AED64,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xa38,
        words: &[NL_PIXEL_SOURCE_010BB178, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x9ec,
        words: &[WATERFOG_PIXEL_SOURCE_010BB148, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x104,
        words: &[
            WATERFOG_PIXEL_SOURCE_010BB148,
            MACRO_ALPHATESTING_010AEF00,
            NL_EMPTY_STRING_01011584,
            0,
        ],
        cleared: 0x3c,
    },
];

// Translated from 00bc5c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills the 0x1c
/// vertex source blocks on its stack (the no-lighting, particle-system and waterfog vertex
/// files with their macro sets; each block's rest cleared by `memset` except one written in
/// full), then compiles each block whose file word is set: `MakePath`, the name formatted from
/// `"NOLIGHT%03i.vso"` with the block's index (`00406d00`), `CompileVertexShaderHLSL` with
/// `"vs_2_0"`, the result assigned to the `NiPointer` at +0xe4 + 4 * index. Then fills the seven
/// pixel source blocks and compiles them in order (`"ps_2_a"`, the names `NOLIGHTTEX.pso` ..
/// `NOLIGHTWFAt.pso`) into the `NiPointer`s at +0x154 to +0x16c.
// Translation incomplete: 00bc5c40 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_shader_no_lighting_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(NL_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &NL_VERTEX_BLOCKS);
        let path = frame + NL_BUFFERS.path;
        let name = frame + NL_BUFFERS.name;
        for (index, block) in NL_VERTEX_BLOCKS.iter().enumerate() {
            let index = index as u32;
            let source = frame + block.offset;
            let file = e.mem.u32(source);
            if file == 0 {
                continue;
            }
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, VERTEX_NAME_FORMAT_010BB198, index],
            );
            let shader = e
                .call(
                    COMPILE_VERTEX_SHADER,
                    &args![base, path, source + 4, PROFILE_VS_2_0_010A93E4, name],
                )
                .u32();
            ni_pointer_assign(e, base + 0xe4 + 4 * index, shader);
        }
        set_up_sources(e, frame, &NL_PIXEL_BLOCKS);
        for (index, block) in NL_PIXEL_BLOCKS.iter().enumerate() {
            let step = ShaderCompile {
                source: block.offset,
                output: PIXEL_OUTPUTS[index],
                profile: NL_PROFILE_PS_2_A_010A93C4,
                pixel: true,
                slot: 0x154 + 4 * index as u32,
            };
            compile_shaders(e, base, frame, &NL_BUFFERS, &[step]);
        }
    });
}

/// The passes of `BSShaderNoLighting::LoadStagesAndPasses`, in the exe's order (immediates of
/// its straight-line code): the pass pointer's offset, whether the pass gets a texture stage,
/// the vertex and pixel shader `NiPointer`s of the shader it takes, and whether it gets the
/// render state 0xa8 = 3.
const NO_LIGHTING_PASSES: [(u32, bool, u32, u32, bool); 30] = [
    (0x6c, true, 0xe4, 0x154, false),
    (0x70, true, 0xec, 0x15c, false),
    (0x74, false, 0xe8, 0x158, false),
    (0x78, false, 0xf0, 0x164, false),
    (0x7c, true, 0xf4, 0x154, false),
    (0x80, true, 0xfc, 0x15c, false),
    (0x84, false, 0xf8, 0x158, false),
    (0x88, false, 0x100, 0x164, false),
    (0x8c, true, 0x104, 0x154, false),
    (0x90, true, 0x10c, 0x15c, false),
    (0x94, false, 0x108, 0x158, false),
    (0x98, false, 0x110, 0x164, false),
    (0x9c, true, 0x114, 0x154, false),
    (0xa0, true, 0x11c, 0x15c, false),
    (0xa4, false, 0x118, 0x158, false),
    (0xa8, false, 0x120, 0x164, false),
    (0xac, true, 0x124, 0x15c, false),
    (0xb0, true, 0x128, 0x15c, false),
    (0xb4, true, 0x124, 0x160, false),
    (0xb8, true, 0x128, 0x160, false),
    (0xbc, true, 0x12c, 0x15c, false),
    (0xc0, true, 0x130, 0x15c, false),
    (0xc4, false, 0x134, 0x168, true),
    (0xc8, true, 0x138, 0x16c, true),
    (0xcc, false, 0x13c, 0x168, true),
    (0xd0, true, 0x140, 0x16c, true),
    (0xd4, false, 0x144, 0x168, true),
    (0xd8, true, 0x148, 0x16c, true),
    (0xdc, false, 0x14c, 0x168, true),
    (0xe0, true, 0x150, 0x16c, true),
];

// Translated from 00bc77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers
/// 1. For each of the 30 pass pointers from +0x6c to +0xe0 in order: a new pass (`00e804d0`);
/// for 18 of them a texture stage made into one stack pointer (`CreateNewStage`; earlier
/// stages are not dropped before the next is made), `SetupDefaultStage` index 0 with 3, 2, 1
/// (no filter mode) and `SetStage` with the pass's word at +0x14; the pass's vertex shader
/// (+0x5c) and pixel shader (+0x44) `NiPointer`s assigned from the shader's (vertex +0xe4 to
/// +0x150, pixel +0x154 to +0x16c, see the table); for the last eight the render state 0xa8 = 3.
/// Finally the stage reference is dropped.
// Translation incomplete: 00bc77e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn bs_shader_no_lighting_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (pass_offset, stage, vertex, pixel, alpha_state) in NO_LIGHTING_PASSES {
            let slot = base + pass_offset;
            e.call(NEW_PASS_00E804D0, &args![slot]);
            if stage {
                e.call(CREATE_NEW_STAGE, &args![local]);
                let stage = e.mem.u32(local);
                e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
                let pass = e.mem.u32(slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            }
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + vertex);
            ni_pointer_assign(e, pass + 0x5c, shader);
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
            if alpha_state {
                let pass = e.mem.u32(slot);
                pass_render_state(e, pass, 0xa8, 3);
            }
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

use crate::units::bsshader::particleshader::constant_maps::{add_entry, make_constant_map};
use crate::units::bsshader::particleshader::reference_release;

/// The pass number of the frame being drawn (`011f91e4`).
const PASS_011F91E4: u32 = 0x011f_91e4;
/// The byte setting `00bc6de0` passes to `BSShaderManager::GetFogProperty` (`011f91c4`) and that
/// getter (engine map, `00b55520`, `cdecl`, one word).
const FOG_SETTING_011F91C4: u32 = 0x011f_91c4;
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
/// The fog constants `00bc6de0` and `00bc7100` write (`01202148` to `01202164`), the `float`
/// used when the fog property's two distances are 0 (`010ad6e0`) and the colour then copied
/// (`011a9bd0` to `011a9bdc`).
const FOG_01202148: u32 = 0x0120_2148;
const FOG_FAR_010AD6E0: u32 = 0x010a_d6e0;
const FOG_COLOR_011A9BD0: u32 = 0x011a_9bd0;

/// `NiPointer` assignment `00a5f860` (`thiscall` on the slot, one word) and the state word the
/// pass set-up leaves (`0126f74c`).
const NI_POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
/// The jump table of `00bc6970` for the passes 0x42 to 0x57 (`00bc6d50`, 22 code addresses) and
/// the one for 0x2dd to 0x2ef (the byte table at `00bc6dc8` into the 8 addresses at `00bc6da8`),
/// read from the exe.
const PASSES_00BC6D50: [u32; 22] = [
    0x00bc_699c,
    0x00bc_69d5,
    0x00bc_69b8,
    0x00bc_69f2,
    0x00bc_6a72,
    0x00bc_6ab1,
    0x00bc_6a92,
    0x00bc_6ad1,
    0x00bc_6a0e,
    0x00bc_6a4b,
    0x00bc_6a2b,
    0x00bc_6a6a,
    0x00bc_6af1,
    0x00bc_6b30,
    0x00bc_6b10,
    0x00bc_6b50,
    0x00bc_6b6f,
    0x00bc_6b8b,
    0x00bc_6bc1,
    0x00bc_6bf6,
    0x00bc_6bfe,
    0x00bc_6c33,
];
const TARGETS_00BC6DA8: [u32; 8] = [
    0x00bc_6cd4,
    0x00bc_6c78,
    0x00bc_6cf2,
    0x00bc_6c96,
    0x00bc_6d11,
    0x00bc_6cb5,
    0x00bc_6d30,
    0x00bc_6bbd,
];
const BYTES_00BC6DC8: [u8; 19] = [0, 7, 7, 7, 7, 1, 2, 7, 7, 7, 7, 3, 4, 7, 7, 7, 7, 5, 6];

/// What each case of `00bc6970` does: the shader word put in `0126f74c`, then either the word
/// assigned to the `NiPointer` at +0x28 or (`None`) that pointer released and cleared.
fn case_00bc6970(target: u32) -> Option<(u32, Option<u32>)> {
    let case = match target {
        0x00bc_699c => (0x6c, Some(0x184)),
        0x00bc_69b8 => (0x70, Some(0x18c)),
        0x00bc_69d5 => (0x74, Some(0x188)),
        0x00bc_69f2 => (0x78, Some(0x184)),
        0x00bc_6a0e => (0x7c, Some(0x190)),
        0x00bc_6a2b => (0x80, Some(0x198)),
        0x00bc_6a4b => (0x84, Some(0x194)),
        // 00bc6a6a: +0x88, then the store and assignment at 00bc6a11 (+0x190).
        0x00bc_6a6a => (0x88, Some(0x190)),
        0x00bc_6a72 => (0x8c, Some(0x19c)),
        0x00bc_6a92 => (0x90, Some(0x1a4)),
        0x00bc_6ab1 => (0x94, Some(0x1a0)),
        0x00bc_6ad1 => (0x98, Some(0x19c)),
        0x00bc_6af1 => (0x9c, Some(0x1a8)),
        0x00bc_6b10 => (0xa0, Some(0x1b0)),
        0x00bc_6b30 => (0xa4, Some(0x1ac)),
        0x00bc_6b50 => (0xa8, Some(0x1a8)),
        0x00bc_6b6f => (0xac, None),
        0x00bc_6b8b => (0xb0, None),
        0x00bc_6bc1 => (0xb4, None),
        // 00bc6bf6 and 00bc6c33 jump into 00bc6b8b's release after their own word.
        0x00bc_6bf6 => (0xb8, None),
        0x00bc_6bfe => (0xbc, None),
        0x00bc_6c33 => (0xc0, None),
        0x00bc_6c3e => (0xc4, Some(0x184)),
        0x00bc_6c78 => (0xc8, Some(0x184)),
        0x00bc_6c96 => (0xcc, Some(0x188)),
        0x00bc_6cb5 => (0xd0, Some(0x18c)),
        0x00bc_6cd4 => (0xd4, Some(0x19c)),
        0x00bc_6cf2 => (0xd8, Some(0x19c)),
        0x00bc_6d11 => (0xdc, Some(0x1a0)),
        0x00bc_6d30 => (0xe0, Some(0x1a4)),
        // 00bc6bbd: the default, nothing.
        _ => return None,
    };
    Some(case)
}

// Translated from 00bc6970 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00bc6970 to the `RET 4` at 00bc6d4b, its jump tables
/// after it): slot 60 (+0xf0) of the no-lighting shader's vtable `010bb230`, `thiscall` with one
/// stack word: the pass. For the passes 0x42 to 0x57, 0x2dc and 0x2dd/0x2e2/0x2e3/0x2e8/0x2e9/
/// 0x2ee/0x2ef (the compiler's jump tables, read from the exe) puts the pass's state word (+0x6c
/// to +0xe0) in `0126f74c`, then assigns the pass's shader word (+0x184 to +0x1b0) to the
/// `NiPointer` at +0x28 (`00a5f860`), or for 0x52 to 0x57 releases that pointer and clears it.
/// Other passes: nothing.
pub fn fn_00bc6970(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let target = if pass > 0x2dc {
        let case = pass.wrapping_sub(0x2dd);
        if case > 0x12 {
            return;
        }
        TARGETS_00BC6DA8[usize::from(BYTES_00BC6DC8[case as usize])]
    } else if pass == 0x2dc {
        0x00bc_6c3e
    } else {
        let case = pass.wrapping_sub(0x42);
        if case > 0x15 {
            return;
        }
        PASSES_00BC6D50[case as usize]
    };
    let Some((state, pointer)) = case_00bc6970(target) else {
        return;
    };
    let word = e.mem.u32(base + state);
    e.set_global::<u32>(PASS_STATE_0126F74C, word);
    match pointer {
        Some(offset) => {
            let value = e.mem.u32(base + offset);
            e.call(NI_POINTER_ASSIGN_00A5F860, &args![base + 0x28, value]);
        }
        None => ni_pointer_clear(e, base + 0x28),
    }
}

// Translated from 00bc6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the no-lighting shader's vtable `010bb230`, slot +0xfc),
/// `thiscall` with one stack word: the pass number. Sets the byte +0x8 of the object at +0x170
/// to 1 for passes 0x52 to 0x57 (else 0) and of the object at +0x178 for passes 0x4a to 0x51,
/// 0x56 and 0x57 (else 0). With the fog property (`GetFogProperty` with the byte `011f91c4`):
/// when its `float`s +0x30 and +0x2c are both 0.0 (`FUCOM`, NaN is not), `01202148` gets
/// `010ad6e0`, `0120214c` 0.0, `01202150` the property's +0x60, `01202154` 0.0 and `01202158` to
/// `01202164` the words `011a9bd0` to `011a9bdc`; otherwise `01202148` gets +0x30, `0120214c`
/// +0x30 - +0x2c (x87, rounded to a `float`), `01202150` +0x60, `01202154` 0.0, `01202158` to
/// `01202160` the property's +0x20 to +0x28 and `01202164` 1.0.
pub fn fn_00bc6de0(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let target = e.mem.u32(base + 0x170);
    let skinned = if pass.wrapping_sub(0x52) <= 5 { 1 } else { 0 };
    e.mem.set_u8(target + 8, skinned);
    let other = e.mem.u32(base + 0x178);
    let flag = if (0x4a..=0x51).contains(&pass) || pass == 0x56 || pass == 0x57 {
        1
    } else {
        0
    };
    e.mem.set_u8(other + 8, flag);
    let setting = u32::from(e.global::<u8>(FOG_SETTING_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![setting]).u32();
    if fog == 0 {
        return;
    }
    let near_bits = x87_quiet_bits(e.mem.u32(fog + 0x2c));
    let raw = [0x20u32, 0x24, 0x28].map(|offset| e.mem.u32(fog + offset));
    let far_bits = x87_quiet_bits(e.mem.u32(fog + 0x30));
    let range_bits = x87_quiet_bits(e.mem.u32(fog + 0x60));
    let far = x87_load32(far_bits);
    let near = x87_load32(near_bits);
    let words: [u32; 8] = if far == 0.0 && near == 0.0 {
        let fallback = x87_quiet_bits(e.global::<u32>(FOG_FAR_010AD6E0));
        let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(FOG_COLOR_011A9BD0 + offset));
        [
            fallback, 0, range_bits, 0, color[0], color[1], color[2], color[3],
        ]
    } else {
        [
            far_bits,
            x87_store32(x87_sub(far, near)),
            range_bits,
            0,
            x87_quiet_bits(raw[0]),
            x87_quiet_bits(raw[1]),
            x87_quiet_bits(raw[2]),
            1.0f32.to_bits(),
        ]
    };
    for (index, word) in words.iter().enumerate() {
        e.set_global::<u32>(FOG_01202148 + 4 * index as u32, *word);
    }
}

/// The 16-byte sub-texture records `00bc6fa0` copies (`01202048`) and CRT `memcpy` (`00ec44d0`,
/// `cdecl`).
const SUBTEX_01202048: u32 = 0x0120_2048;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00bc6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`), `thiscall` with two stack words (`RET
/// 8`): an object whose word +0xb8 holds the record (a byte count at +0x6c, the 16-byte entries
/// at +0x68), and a word the code never reads. With a non-zero count, copies count * 16 bytes of
/// entries to `01202048` (`memcpy`) and stores the count in +0x20 of the object at +0x170.
pub fn fn_00bc6fa0(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let record = e.mem.u32(owner.addr() + 0xb8);
    let count = e.mem.u8(record + 0x6c);
    if count != 0 {
        let source = e.mem.u32(record + 0x68);
        let count = u32::from(count);
        e.call(MEMCPY, &args![SUBTEX_01202048, source, count << 4]);
        let target = e.mem.u32(this.addr() + 0x170);
        e.mem.set_u32(target + 0x20, count);
    }
}

/// The material alpha (`0120201c`), the material colour (`01202010` to `01202018`), the setting
/// that always scales it (`011f941e`), the `double` 0.1 below which it is scaled (`0101ffa0`), the
/// colour it is compared with (`011f4998` to `011f49a0`) and the one that then replaces it
/// (`011a9b7c` to `011a9b84`).
const MATERIAL_ALPHA_0120201C: u32 = 0x0120_201c;
const MATERIAL_COLOR_01202010: u32 = 0x0120_2010;
const ALWAYS_SCALE_011F941E: u32 = 0x011f_941e;
const TENTH_0101FFA0: u32 = 0x0101_ffa0;
const COMPARED_011F4998: u32 = 0x011f_4998;
const REPLACEMENT_011A9B7C: u32 = 0x011a_9b7c;

// Translated from 00bc6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`), `thiscall` (the caller loads ECX;
/// `this` is not read) with two words (`RET 8`): the shader property (`float` alpha at +0x28)
/// and the material (a colour pointer at +0x34, else its own colour at +0x28; a scale at +0x40). `0120201c` gets the alpha when it is below 1.0,
/// else 1.0. The colour's three `float`s are multiplied by the scale when the setting
/// `011f941e` is set or all three are below 0.1 (each product rounded to a `float`), replaced
/// by `011a9b7c` to `011a9b84` when they then equal `011f4998` to `011f49a0`, and stored in
/// `01202010` to `01202018`.
pub fn fn_00bc6fe0(e: &mut Engine, _this: Ptr, property: Ptr, material: Ptr) {
    let alpha = x87_load32(e.mem.u32(property.addr() + 0x28));
    let stored = if 1.0 > alpha {
        x87_store32(alpha)
    } else {
        1.0f32.to_bits()
    };
    e.set_global::<u32>(MATERIAL_ALPHA_0120201C, stored);
    let material = material.addr();
    let mut source = e.mem.u32(material + 0x34);
    if source == 0 {
        source = material + 0x28;
    }
    let words = [0u32, 4, 8].map(|offset| e.mem.u32(source + offset));
    let mut color = words.map(x87_load32);
    let scale = if e.global::<u8>(ALWAYS_SCALE_011F941E) != 0 {
        true
    } else {
        let tenth = x87_load64(e.global::<u64>(TENTH_0101FFA0));
        tenth > color[0] && tenth > color[1] && tenth > color[2]
    };
    if scale {
        let factor = x87_load32(x87_quiet_bits(e.mem.u32(material + 0x40)));
        color = color.map(|value| x87_load32(x87_store32(x87_mul(value, factor))));
    }
    let compared =
        [0u32, 4, 8].map(|offset| x87_load32(e.global::<u32>(COMPARED_011F4998 + offset)));
    if compared[0] == color[0] && compared[1] == color[1] && compared[2] == color[2] {
        color =
            [0u32, 4, 8].map(|offset| x87_load32(e.global::<u32>(REPLACEMENT_011A9B7C + offset)));
    }
    for (index, value) in color.iter().enumerate() {
        e.set_global::<u32>(
            MATERIAL_COLOR_01202010 + 4 * index as u32,
            x87_store32(*value),
        );
    }
}

/// The light direction and colour `00bc7100` reads (`011fa2a0` to `011fa2a8`, `011f9604` to
/// `011f960c`, `011f9610`) and its fog far value (`011f9618`).
const DIRECTION_011FA2A0: u32 = 0x011f_a2a0;
const COLOR_011F9604: u32 = 0x011f_9604;
const OFFSET_011F9610: u32 = 0x011f_9610;
const FAR_011F9618: u32 = 0x011f_9618;

// Translated from 00bc7100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`), `thiscall`, no stack words. For passes
/// 0x2d8 to 0x2ef: sets the byte +0x8 of the object at +0x174, and the fog constants: `01202148`
/// = `011f9618`, `0120214c` to `01202154` = 0.0, `01202158` to `01202160` = `011f9604` to
/// `011f960c`, `01202164` = their dot product with `011fa2a0` to `011fa2a8` (rounded to a
/// `float`) minus `011f9610` (x87, in the exe's order).
pub fn fn_00bc7100(e: &mut Engine, this: Ptr) {
    let pass = e.global::<u32>(PASS_011F91E4);
    if pass.wrapping_sub(0x2d8) > 0x17 {
        return;
    }
    let target = e.mem.u32(this.addr() + 0x174);
    e.mem.set_u8(target + 8, 1);
    let direction =
        [0u32, 4, 8].map(|offset| x87_quiet_bits(e.global::<u32>(DIRECTION_011FA2A0 + offset)));
    let color = [0u32, 4, 8].map(|offset| e.global::<u32>(COLOR_011F9604 + offset));
    let far = x87_quiet_bits(e.global::<u32>(FAR_011F9618));
    let offset = e.global::<u32>(OFFSET_011F9610);
    let first = x87_mul(x87_load32(color[0]), x87_m32(direction[0]));
    let second = x87_mul(x87_load32(color[1]), x87_m32(direction[1]));
    let sum = x87_add(first, second);
    let third = x87_mul(x87_load32(color[2]), x87_m32(direction[2]));
    let dot = x87_store32(x87_add(sum, third));
    let last = x87_store32(x87_sub(x87_load32(dot), x87_m32(offset)));
    let words = [
        far,
        0,
        0,
        0,
        x87_quiet_bits(color[0]),
        x87_quiet_bits(color[1]),
        x87_quiet_bits(color[2]),
        last,
    ];
    for (index, word) in words.iter().enumerate() {
        e.set_global::<u32>(FOG_01202148 + 4 * index as u32, *word);
    }
}

/// `BSShader::~BSShader` (engine map, `00be0f10`), the compiler's `eh vector destructor
/// iterator` (`00ec5fce`, `stdcall`: array, element size, count, destructor) and the sized free
/// `00aa1460` (`cdecl`: block, size).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bc7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::~BSShaderNoLighting` (the body of the vtable `010bb230`'s scalar
/// deleting destructor; unnamed in the engine map), `thiscall`, no stack words. Stores the
/// vtable `010bb230`, clears the 0x1e pass pointers at +0x6c, the 0x1c vertex shader `NiPointer`s
/// at +0xe4, the 7 pixel shader ones at +0x154 and the 12 declarations at +0x184 to +0x1b0, and
/// the word +0x170; then the members' destructors run on what is left (the declarations from
/// +0x1b0 down, the arrays +0x154, +0xe4 with `00b66f20` and +0x6c with `00b79930`) and
/// `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bc7220 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xf for the members) is not translated.
pub fn fn_00bc7220(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NO_LIGHTING_VTABLE_010BB230);
    for index in 0..0x1eu32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..0x1cu32 {
        ni_pointer_clear(e, base + 0xe4 + 4 * index);
    }
    for index in 0..7u32 {
        ni_pointer_clear(e, base + 0x154 + 4 * index);
    }
    for index in 0..12u32 {
        ni_pointer_clear(e, base + 0x184 + 4 * index);
    }
    e.mem.set_u32(base + 0x170, 0);
    for index in (0..12u32).rev() {
        let object = e.mem.u32(base + 0x184 + 4 * index);
        reference_release(e, object);
    }
    for (array, count, destructor) in [
        (0x154u32, 7u32, DESTRUCT_00B66F20),
        (0xe4, 0x1c, DESTRUCT_00B66F20),
        (0x6c, 0x1e, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT_00EC5FCE,
            &args![base + array, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00bc7710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010bb230`, slot +0x8), `thiscall`, no stack
/// words: answers `01202040`.
pub fn fn_00bc7710(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2040
}

// Translated from 00bc7720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`; the vtable `010bb230`, slot +0x118),
/// `thiscall` with two stack words: a word the code never reads and the shader property. Answers
/// the declaration for the property's flags: bit 0x40 of +0x20 picks the second six, bit 0x20 of
/// the byte +0x24 the later ones (and with a word at +0x60 the last of them), bit 0x2 of +0x20
/// the second half: +0x184, +0x19c, +0x188, +0x1a0, +0x18c, +0x1a4 without bit 0x40, +0x190,
/// +0x1a8, +0x194, +0x1ac, +0x198, +0x1b0 with it.
pub fn fn_00bc7720(e: &mut Engine, this: Ptr, _unused_1: u32, property: Ptr) -> u32 {
    let base = this.addr();
    let property = property.addr();
    let flags = e.mem.u32(property + 0x20);
    let second_half = flags & 2 != 0;
    let textured = e.mem.u8(property + 0x24) & 0x20 != 0;
    let offset = if flags & 0x40 != 0 {
        if textured {
            if e.mem.u32(property + 0x60) != 0 {
                if second_half {
                    0x1b0
                } else {
                    0x198
                }
            } else if second_half {
                0x1ac
            } else {
                0x194
            }
        } else if second_half {
            0x1a8
        } else {
            0x190
        }
    } else if textured {
        if e.mem.u32(property + 0x60) != 0 {
            if second_half {
                0x1a4
            } else {
                0x18c
            }
        } else if second_half {
            0x1a0
        } else {
            0x188
        }
    } else if second_half {
        0x19c
    } else {
        0x184
    };
    e.mem.u32(base + offset)
}

/// The render state object (`0126f74c`) and the texture stage calls `00e7de90` and `00e7eb00`
/// (unnamed; `thiscall` on a stage, one word).
const RENDER_STATE_0126F74C: u32 = 0x0126_f74c;
const STAGE_CALL_00E7DE90: u32 = 0x00e7_de90;
const STAGE_CALL_00E7EB00: u32 = 0x00e7_eb00;

// Translated from 00bc8d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010bb230`, slot +0x78), `thiscall` with one
/// stack word: the geometry's set-up record (the property at +0xc). With stage 0 of the render
/// state (`0126f74c`, the stage array at +0x24) and a texture from the property's slot +0xb4:
/// the texture into the stage's +0x8, `00e7de90` with the property's slot +0xcc answer and
/// `00e7eb00(0)`.
pub fn fn_00bc8d40(e: &mut Engine, _this: Ptr, record: Ptr) {
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    let stages = e.mem.u32(state + 0x24);
    let stage = e.mem.u32(stages);
    if stage == 0 {
        return;
    }
    let property = e.mem.u32(record.addr() + 0xc);
    let texture = e.vcall(property, 0xb4, &args![]).u32();
    if texture != 0 {
        e.mem.set_u32(stage + 8, texture);
        let mode = e.vcall(property, 0xcc, &args![]).u32();
        e.call(STAGE_CALL_00E7DE90, &args![stage, mode]);
        e.call(STAGE_CALL_00E7EB00, &args![stage, 0u32]);
    }
}

/// The unnamed `cdecl` state setters `00b98480`, `BSRenderState::SetZWriteEnable` (engine map,
/// `00b97e30`), `00b97de0`, `00b98070`, `00b97fa0` and `00b97ff0`.
const STATE_CALL_00B98480: u32 = 0x00b9_8480;
const SET_Z_WRITE_ENABLE: u32 = 0x00b9_7e30;
const STATE_CALL_00B97DE0: u32 = 0x00b9_7de0;
const STATE_CALL_00B98070: u32 = 0x00b9_8070;
const STATE_CALL_00B97FA0: u32 = 0x00b9_7fa0;
const STATE_CALL_00B97FF0: u32 = 0x00b9_7ff0;

// Translated from 00bc8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010bb230`, slot +0x8c), `thiscall` with one
/// stack word: the geometry's set-up record. Undoes the set-up's states: `00b98480(0, 0)` when
/// the property (+0xc) has bit 0x10000 of +0x24; `SetZWriteEnable(1, 0)` without bit 0 of +0x24
/// and `00b97de0(1, 0)` without bit 31 of +0x20 (both read with no null test on the property);
/// `00b98070(0, 0)` when the object at +0x10 has flags bit 0.
pub fn fn_00bc8d90(e: &mut Engine, _this: Ptr, record: Ptr) {
    let record = record.addr();
    let property = e.mem.u32(record + 0xc);
    if property != 0 && e.mem.u32(property + 0x24) & 0x1_0000 != 0 {
        e.call(STATE_CALL_00B98480, &args![0u32, 0u32]);
    }
    if e.mem.u8(property + 0x24) & 1 == 0 {
        e.call(SET_Z_WRITE_ENABLE, &args![1u32, 0u32]);
    }
    if e.mem.u32(property + 0x20) & 0x8000_0000 == 0 {
        e.call(STATE_CALL_00B97DE0, &args![1u32, 0u32]);
    }
    let flags_holder = e.mem.u32(record + 0x10);
    if flags_holder != 0 && e.mem.u8(flags_holder + 0x18) & 1 != 0 {
        e.call(STATE_CALL_00B98070, &args![0u32, 0u32]);
    }
}

/// The texture matrix constants (`01201fe0` to `01202008`, three rows of three with a gap word
/// after each) and the matrix update `00a88cd0` (unnamed; `thiscall`).
const TEXTURE_MATRIX_01201FE0: u32 = 0x0120_1fe0;
const MATRIX_UPDATE_00A88CD0: u32 = 0x00a8_8cd0;

// Translated from 00bc8e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`), `thiscall` (the caller loads ECX;
/// `this` is not read) with one word (`RET 4`): the geometry's set-up record. The texture matrix
/// is the identity unless the record's +0x14 has a first texture (its +0x20 array) with a transform (+0xc): then that transform's nine `float`s
/// at +0x20 (after `00a88cd0` when its byte +0x1c is set). The rows go to `01201fe0`, `01201ff0`
/// and `01202000` (three `float`s each).
pub fn fn_00bc8e00(e: &mut Engine, _this: Ptr, record: Ptr) {
    let one = 1.0f32.to_bits();
    let mut matrix = [one, 0, 0, 0, one, 0, 0, 0, one];
    let textures = e.mem.u32(record.addr() + 0x14);
    if textures != 0 {
        let array = e.mem.u32(textures + 0x20);
        let first = e.mem.u32(array);
        if first != 0 {
            let transform = e.mem.u32(first + 0xc);
            if transform != 0 {
                if e.mem.u8(transform + 0x1c) != 0 {
                    e.call(MATRIX_UPDATE_00A88CD0, &args![transform]);
                }
                for (index, word) in matrix.iter_mut().enumerate() {
                    *word = e.mem.u32(transform + 0x20 + 4 * index as u32);
                }
            }
        }
    }
    for (index, word) in matrix.iter().enumerate() {
        let row = index as u32 / 3;
        let column = index as u32 % 3;
        e.set_global::<u32>(
            TEXTURE_MATRIX_01201FE0 + 0x10 * row + 4 * column,
            x87_quiet_bits(*word),
        );
    }
}

/// `00be1ff0` (unnamed; `thiscall`, `RET 4`): the shader's common set-up the no-lighting one
/// falls back to.
const SHADER_CALL_00BE1FF0: u32 = 0x00be_1ff0;

// Translated from 00bc8ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010bb230`, slot +0x80), `thiscall` with one
/// stack word: the geometry's set-up record. Without a flags object at the record's +0 with bit
/// 0 and for the passes 0x4a to 0x51: `00b97fa0(1, 0)` and `00b97ff0(6, 7, 0)`; otherwise
/// `00be1ff0` on `this` with the record. Answers the last call's `EAX`.
pub fn fn_00bc8ed0(e: &mut Engine, this: Ptr, record: Ptr) -> u32 {
    let pass = e.global::<u32>(PASS_011F91E4);
    let flags_holder = e.mem.u32(record.addr());
    let flagged = flags_holder != 0 && e.mem.u8(flags_holder + 0x18) & 1 != 0;
    if !flagged && pass.wrapping_sub(0x4a) <= 7 {
        e.call(STATE_CALL_00B97FA0, &args![1u32, 0u32]);
        return e.call(STATE_CALL_00B97FF0, &args![6u32, 7u32, 0u32]).u32();
    }
    e.call(SHADER_CALL_00BE1FF0, &args![this, record]).u32()
}

// Translated from 00bc9390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadernolighting.cpp`; slot 0 of the vtable `010bb230`): the
/// no-lighting shader's scalar deleting destructor, `thiscall`, one stack word (the flags). Runs
/// `00bc7220`; with bit 0 set, frees the 0x1b4 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bc9390(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc7220(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x1b4u32]);
    }
    this
}

/// The scene graph holder (`011f91e0`), the fallback for its +0x20 (`011f4288`), the renderer
/// call on the shader's +0x18 (`00e6fbb0`, `thiscall`, `RET 8`), `NiTransform::MakeIdentity`
/// (engine map, `00a86bf0`), the translation copied into the local transform (`011f474c` to
/// `011f4754`), the falloff constants (`01202030` to `0120203c`) and the alpha flags (`01202020`,
/// `01202024`).
const SCENE_HOLDER_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_011F4288: u32 = 0x011f_4288;
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const TRANSLATION_011F474C: u32 = 0x011f_474c;
const FALLOFF_CONSTANT_01202030: u32 = 0x0120_2030;
const ALPHA_FLAGS_01202020: u32 = 0x0120_2020;

// Translated from 00bc9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::SetupGeometryConstants` (engine map; the vtable `010bb230`, slot +0x7c),
/// `thiscall` with one stack word: the geometry's set-up record (+0 a flags object, +0x8 the
/// material, +0xc the property, +0x10 another flags object). With the scene object `O` of
/// `011f91e0`: `00e6fbb0` on the shader's +0x18 with (`O` + 0x68, or, when `O` has a word +0xbc,
/// a local identity transform with the translation `011f474c`, and 0); `00bc6fa0` for the
/// passes 0x52 to 0x57; the texture matrix (`00bc8e00`), the light fog constants (`00bc7100`) and
/// the material constants (`00bc6fe0`). The falloff `01202030` to `0120203c` gets the property's
/// +0x70 to +0x7c, except for the passes 0x56 and 0x57 without bit 0x40 of +0x20 (then 0.0).
/// The constant maps at +0x34 and +0x30 (with the state's +0x5c / +0x44 set) get their slot
/// +0x78 with eleven words, around the alpha flags `01202020` / `01202024` (1.0 for field 0 / 2
/// of the flags at +0 bits 5 to 8). Then `SetZWriteEnable(0, 0)` without bit 0 of +0x24,
/// `00b97de0(0, 0)` without bit 31 of +0x20, and the object at +0x1c's slot +0x24 with +0x10's
/// flags object when it has bit 0.
pub fn bs_shader_no_lighting_setup_geometry_constants(e: &mut Engine, this: Ptr, record: Ptr) {
    let base = this.addr();
    let record = record.addr();
    let pass = e.global::<u32>(PASS_011F91E4);
    let holder = e.global::<u32>(SCENE_HOLDER_011F91E0);
    let scene = e.mem.u32(holder);
    let mut fallback = e.mem.u32(scene + 0x20);
    if fallback == 0 {
        fallback = DEFAULT_011F4288;
    }
    let camera = e.mem.u32(scene + 0xb8);
    let camera_word = e.mem.u32(camera + 0x34);
    let property = e.mem.u32(record + 0xc);
    let scene_word = e.mem.u32(scene + 0xbc);
    let material = e.mem.u32(record + 8);
    e.with_stack(0x34, |e, transform| {
        let transform = transform.addr();
        let anchor = if scene_word != 0 {
            e.call(MAKE_IDENTITY, &args![transform]);
            for index in 0..3u32 {
                let word = e.global::<u32>(TRANSLATION_011F474C + 4 * index);
                e.mem.set_u32(transform + 0x24 + 4 * index, word);
            }
            transform
        } else {
            scene + 0x68
        };
        let target = e.mem.u32(base + 0x18);
        e.call(RENDERER_CALL_00E6FBB0, &args![target, anchor, 0u32]);
    });
    if pass.wrapping_sub(0x52) <= 5 {
        fn_00bc6fa0(e, this, Ptr::new(scene), scene + 0x68);
    }
    fn_00bc8e00(e, this, Ptr::new(record));
    fn_00bc7100(e, this);
    fn_00bc6fe0(e, this, Ptr::new(property), Ptr::new(material));
    let falloff = if pass.wrapping_sub(0x56) <= 1 && e.mem.u8(property + 0x20) & 0x40 == 0 {
        [0u32; 4]
    } else {
        [0x70u32, 0x74, 0x78, 0x7c].map(|offset| e.mem.u32(property + offset))
    };
    for (index, word) in falloff.iter().enumerate() {
        e.set_global::<u32>(FALLOFF_CONSTANT_01202030 + 4 * index as u32, *word);
    }
    let map_call = |e: &mut Engine, map_offset: u32, state_offset: u32| {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            return;
        }
        let state = e.global::<u32>(RENDER_STATE_0126F74C);
        let shader = e.mem.u32(state + state_offset);
        if shader == 0 {
            return;
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
                scene + 0x68,
                fallback,
                0u32,
                1u32
            ],
        );
    };
    map_call(e, 0x34, 0x5c);
    e.set_global::<u32>(ALPHA_FLAGS_01202020, 0);
    e.set_global::<u32>(ALPHA_FLAGS_01202020 + 4, 0);
    let flags_holder = e.mem.u32(record);
    if flags_holder != 0 {
        let field = (u32::from(e.mem.u16(flags_holder + 0x18)) >> 5) & 0xf;
        if field == 0 {
            e.set_global::<u32>(ALPHA_FLAGS_01202020, 1.0f32.to_bits());
        } else if field == 2 {
            e.set_global::<u32>(ALPHA_FLAGS_01202020 + 4, 1.0f32.to_bits());
        }
    }
    map_call(e, 0x30, 0x44);
    if e.mem.u8(property + 0x24) & 1 == 0 {
        e.call(SET_Z_WRITE_ENABLE, &args![0u32, 0u32]);
    }
    if e.mem.u32(property + 0x20) & 0x8000_0000 == 0 {
        e.call(STATE_CALL_00B97DE0, &args![0u32, 0u32]);
    }
    let other = e.mem.u32(record + 0x10);
    if other != 0 && e.mem.u8(other + 0x18) & 1 != 0 {
        let target = e.mem.u32(base + 0x1c);
        e.vcall(target, 0x24, &args![other]);
    }
}

/// Strings of `BSShaderNoLighting::CreateConstantMaps` (read in Ghidra): the pixel entries
/// `"matcolor"` (`010bb390`), `"matalpha"` (`010ba2bc`), `"fogcolor"` (`010b8c48`), `"waterfog"`
/// (`010ba288`); the vertex entries `"WorldViewProjTranspose"` (`010a1e50`),
/// `"WorldViewTranspose"` (`010a1ee0`), `"WorldTranspose"` (`010a1f08`), `"EyeDir"` (`010a189c`),
/// `"TextureTransform"` (`010bb37c`), `"fog"` (`0103bfc8`), `"falloff"` (`010bb374`), `"subtex"`
/// (`010bb36c`) and `"BoneMatrix3"` (`010a1cac`).
const NAME_MATCOLOR_010BB390: u32 = 0x010b_b390;
const NAME_MATALPHA_010BA2BC: u32 = 0x010b_a2bc;
const NAME_FOGCOLOR_010B8C48: u32 = 0x010b_8c48;
const NAME_WATERFOG_010BA288: u32 = 0x010b_a288;
const NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const NAME_WORLD_VIEW_TRANSPOSE_010A1EE0: u32 = 0x010a_1ee0;
const NAME_WORLD_TRANSPOSE_010A1F08: u32 = 0x010a_1f08;
const NAME_EYE_DIR_010A189C: u32 = 0x010a_189c;
const NAME_TEXTURE_TRANSFORM_010BB37C: u32 = 0x010b_b37c;
const NAME_FOG_0103BFC8: u32 = 0x0103_bfc8;
const NAME_FALLOFF_STRING_010BB374: u32 = 0x010b_b374;
const NAME_SUBTEX_010BB36C: u32 = 0x010b_b36c;
const NAME_BONE_MATRIX3_010A1CAC: u32 = 0x010a_1cac;
/// The water fog constants (`01202168`).
const WATERFOG_01202168: u32 = 0x0120_2168;

/// An entry with data (`kind`, register, register count, byte count, data) on the map at `slot`.
fn data_entry(e: &mut Engine, slot: u32, name: u32, kind: u32, layout: [u32; 3], data: u32) {
    let [register, registers, bytes] = layout;
    add_entry(
        e,
        slot,
        &[
            name,
            kind,
            0,
            register,
            registers,
            NL_EMPTY_STRING_01011584,
            bytes,
            4,
            data,
            0,
        ],
    );
}

/// A predefined matrix entry (0x20000009, the given extra word, register, register count) on
/// the map at `slot`, then, when `field` is set, the map's slot +0x70 asked for the entry by
/// name and its answer stored at `this + field`.
fn matrix_entry(e: &mut Engine, slot: u32, name: u32, words: [u32; 3], field: Option<u32>) {
    let [extra, register, registers] = words;
    add_entry(
        e,
        slot,
        &[name, 0x2000_0009, extra, register, registers, 0, 0, 0, 0, 0],
    );
    if let Some(target) = field {
        let map = e.mem.u32(slot);
        let entry = e.vcall(map, 0x70, &args![name]).u32();
        e.mem.set_u32(target, entry);
    }
}

// Translated from 00bc9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::CreateConstantMaps` (engine map), `thiscall`, no stack words. When the
/// `NiPointer` at +0x30 is null: a constant map (program type 2, the word at +0x18 as the
/// renderer) with `"matcolor"`, `"matalpha"`, `"fogcolor"` (0x10000007, registers 0 to 2, 0x10
/// bytes at `01202010`, `01202020`, `01202158`) and `"waterfog"` (0x10000009, register 5, 2
/// registers, 0x20 bytes at `01202168`). When the `NiPointer` at +0x34 is null: a constant map of
/// program type 0 with the matrices `"WorldViewProjTranspose"` (register 0), `"WorldViewTranspose"`
/// (0x24), `"WorldTranspose"` (8), `"EyeDir"` (0x29), 4 registers each, the entries
/// `"TextureTransform"` (0x10000008, register 4, 3 registers, 0x30 bytes at `01201fe0`), `"fog"`
/// (0x10000009, 0xd, 2, 0x20 at `01202148`), `"falloff"` (0x10000007, 0x28, 1, 0x10 at
/// `01202030`), `"subtex"` (0x10000009, 0xf, 0x10, 0x100 at `01202048`) and `"BoneMatrix3"`
/// (0x20000009 with 0x120000, register 0x40, 0x36 registers); the map's slot +0x70 answers for
/// `WorldViewTranspose`, `WorldTranspose`, `EyeDir`, `subtex` and `BoneMatrix3` are kept at
/// +0x178, +0x174, +0x17c, +0x170 and +0x180.
// Translation incomplete: 00bc9cc0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn bs_shader_no_lighting_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pixel_map = base + 0x30;
    if e.mem.u32(pixel_map) == 0 {
        make_constant_map(e, pixel_map, base + 0x18, 2);
        for (name, register, data) in [
            (NAME_MATCOLOR_010BB390, 0u32, COLOR_01202010),
            (NAME_MATALPHA_010BA2BC, 1, ALPHA_FLAGS_01202020),
            (NAME_FOGCOLOR_010B8C48, 2, FOG_01202148 + 0x10),
        ] {
            data_entry(e, pixel_map, name, 0x1000_0007, [register, 1, 0x10], data);
        }
        data_entry(
            e,
            pixel_map,
            NAME_WATERFOG_010BA288,
            0x1000_0009,
            [5, 2, 0x20],
            WATERFOG_01202168,
        );
    }
    let vertex_map = base + 0x34;
    if e.mem.u32(vertex_map) == 0 {
        make_constant_map(e, vertex_map, base + 0x18, 0);
        matrix_entry(
            e,
            vertex_map,
            NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
            [0, 0, 4],
            None,
        );
        matrix_entry(
            e,
            vertex_map,
            NAME_WORLD_VIEW_TRANSPOSE_010A1EE0,
            [0, 0x24, 4],
            Some(base + 0x178),
        );
        matrix_entry(
            e,
            vertex_map,
            NAME_WORLD_TRANSPOSE_010A1F08,
            [0, 8, 4],
            Some(base + 0x174),
        );
        matrix_entry(
            e,
            vertex_map,
            NAME_EYE_DIR_010A189C,
            [0, 0x29, 4],
            Some(base + 0x17c),
        );
        data_entry(
            e,
            vertex_map,
            NAME_TEXTURE_TRANSFORM_010BB37C,
            0x1000_0008,
            [4, 3, 0x30],
            TEXTURE_MATRIX_01201FE0,
        );
        data_entry(
            e,
            vertex_map,
            NAME_FOG_0103BFC8,
            0x1000_0009,
            [0xd, 2, 0x20],
            FOG_01202148,
        );
        data_entry(
            e,
            vertex_map,
            NAME_FALLOFF_STRING_010BB374,
            0x1000_0007,
            [0x28, 1, 0x10],
            FALLOFF_CONSTANT_01202030,
        );
        data_entry(
            e,
            vertex_map,
            NAME_SUBTEX_010BB36C,
            0x1000_0009,
            [0xf, 0x10, 0x100],
            SUBTEX_01202048,
        );
        let map = e.mem.u32(vertex_map);
        let entry = e.vcall(map, 0x70, &args![NAME_SUBTEX_010BB36C]).u32();
        e.mem.set_u32(base + 0x170, entry);
        matrix_entry(
            e,
            vertex_map,
            NAME_BONE_MATRIX3_010A1CAC,
            [0x12_0000, 0x40, 0x36],
            Some(base + 0x180),
        );
    }
}

// Translated from 00bc6940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshadernolighting.cpp`), `thiscall`, no stack words:
/// gives each of the 0x1e passes at +0x6c to +0xe0 to `this`'s vtable slot +0x128.
pub fn fn_00bc6940(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..0x1eu32 {
        let pass = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![pass]);
    }
}

// Translated from 00bc8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderNoLighting::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: `LoadShaders`,
/// then `LoadStagesAndPasses` (a tail jump: its answer is this function's).
pub fn bs_shader_no_lighting_reload_shaders(e: &mut Engine, this: Ptr) -> bool {
    bs_shader_no_lighting_load_shaders(e, this);
    bs_shader_no_lighting_load_stages_and_passes(e, this)
}

/// The renderer's `00e6fe30` (unnamed; `thiscall`, five words).
const RENDERER_CALL_00E6FE30: u32 = 0x00e6_fe30;

// Translated from 00bc8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 25 of the vtable `010bb230`, `thiscall`,
/// eight words of which the second (a geometry: +0x20, +0x28), third (skin data: +4, the u16
/// +0x20) and seventh are read. With skin data: the renderer's (`011f9508`)
/// `00e6fe30(geometry, seventh, 0, 3, 1)`, then the current pass's (`0126f74c`) vertex shader
/// (+0x5c) gets its slot +0x78 with the bone entry +0x180, the geometry's +0x28, the u16, the
/// geometry's +0x20 and the skin data's +4. Answers 0.
pub fn fn_00bc8f20(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    geometry: Ptr,
    skin: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    seventh: u32,
    _unused_7: u32,
) -> u32 {
    if skin.is_null() {
        return 0;
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.call(
        RENDERER_CALL_00E6FE30,
        &args![renderer, geometry, seventh, 0u32, 3u32, 1u32],
    );
    let skin_word = e.mem.u32(skin.addr() + 4);
    let pass = e.global::<u32>(RENDER_STATE_0126F74C);
    let shader = e.mem.u32(pass + 0x5c);
    let word_20 = e.mem.u32(geometry.addr() + 0x20);
    let bones = u32::from(e.mem.u16(skin.addr() + 0x20));
    let word_28 = e.mem.u32(geometry.addr() + 0x28);
    let entry = e.mem.u32(this.addr() + 0x180);
    e.vcall(
        shader,
        0x78,
        &args![entry, word_28, bones, word_20, skin_word],
    );
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_6940, fn_00bc6940(Ptr)),
        entry!(0x00bc_8d30, bs_shader_no_lighting_reload_shaders(Ptr) -> bool),
        entry!(
            0x00bc_8f20,
            fn_00bc8f20(Ptr, u32, Ptr, Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bc_77e0, bs_shader_no_lighting_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00bc_5c40, bs_shader_no_lighting_load_shaders(Ptr)),
        entry!(0x00bc_8f80, bs_shader_no_lighting_bs_shader_no_lighting(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00bc_93c0,
            bs_shader_no_lighting_create_shader() -> Ptr
        ),
        entry!(0x00bc_6970, fn_00bc6970(Ptr, u32)),
        entry!(0x00bc_6de0, fn_00bc6de0(Ptr, u32)),
        entry!(0x00bc_6fa0, fn_00bc6fa0(Ptr, Ptr, u32)),
        entry!(0x00bc_6fe0, fn_00bc6fe0(Ptr, Ptr, Ptr)),
        entry!(0x00bc_7100, fn_00bc7100(Ptr)),
        entry!(0x00bc_7220, fn_00bc7220(Ptr)),
        entry!(0x00bc_7710, fn_00bc7710(Ptr) -> u32),
        entry!(0x00bc_7720, fn_00bc7720(Ptr, u32, Ptr) -> u32),
        entry!(0x00bc_8d40, fn_00bc8d40(Ptr, Ptr)),
        entry!(0x00bc_8d90, fn_00bc8d90(Ptr, Ptr)),
        entry!(0x00bc_8e00, fn_00bc8e00(Ptr, Ptr)),
        entry!(0x00bc_8ed0, fn_00bc8ed0(Ptr, Ptr) -> u32),
        entry!(0x00bc_9390, fn_00bc9390(Ptr, u32) -> Ptr),
        entry!(
            0x00bc_9a50,
            bs_shader_no_lighting_setup_geometry_constants(Ptr, Ptr)
        ),
        entry!(0x00bc_9cc0, bs_shader_no_lighting_create_constant_maps(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn bs_shader_no_lighting_create_shader_makes_twelve_declarations_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bc_93c0);
        assert_eq!(shader, SHADER);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let mut want: Vec<(u32, Vec<u32>)> = [2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        // The entries as pushed in the exe, declaration by declaration.
        let lists: [&[[u32; 5]]; 12] = [
            &[[0, 0, 0, 2, 0], [1, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0]],
            &[[0, 0, 0, 2, 0], [1, 5, 4, 4, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 7, 5, 1, 0]],
            &[[0, 0, 0, 2, 0], [1, 1, 3, 2, 0], [2, 5, 4, 4, 0]],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 3, 2, 0],
                [2, 5, 4, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 5, 4, 4, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 7, 5, 1, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
            ],
            &[
                [0, 0, 0, 2, 0],
                [1, 1, 1, 3, 0],
                [2, 2, 2, 4, 0],
                [3, 1, 3, 2, 0],
                [4, 5, 4, 4, 0],
                [5, 7, 5, 1, 0],
            ],
        ];
        for (n, list) in lists.iter().enumerate() {
            let declaration = d(n as u32);
            for entry in list.iter() {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                want.push((declaration_slot(0x8c), words));
            }
            want.push((declaration_slot(0xb4), vec![declaration]));
        }
        want.push((NI_ALLOCATE, vec![0x1b4]));
        let mut construct = vec![SHADER];
        construct.extend((0..12).map(d));
        want.push((CONSTRUCT, construct));
        want.push((shader_slot(0x4c), vec![SHADER]));
        want.push((LOAD_SHADERS, vec![SHADER]));
        want.push((LOAD_STAGES_AND_PASSES, vec![SHADER]));
        want.push((shader_slot(0x11c), vec![SHADER]));
        want.push((shader_slot(0xc0), vec![SHADER, d(0)]));
        assert_eq!(log, want);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_constructor_keeps_the_twelve_declarations() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BE0 + 4 * index, 0x60 + index);
        }
        let this = e.mem.alloc(0x1b4);
        e.mem.write(this, &[0xcc; 0x1b4]);
        let declarations: Vec<u32> = (0..12).map(|_| object(&mut e, 0)).collect();
        let mut words = vec![this];
        words.extend_from_slice(&declarations);
        assert_eq!(e.call(CONSTRUCT, &words).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NO_LIGHTING_VTABLE_010BB230);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x1e, DESTRUCT_00B79930),
                pointer_array(this + 0xe4, 0x1c, DESTRUCT_00B66F20),
                pointer_array(this + 0x154, 7, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0x16);
        for (index, declaration) in declarations.iter().enumerate() {
            assert_eq!(e.mem.u32(this + 0x184 + 4 * index as u32), *declaration);
            assert_eq!(e.mem.u32(declaration + 4), 1);
        }
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(COLOR_01202010 + 4 * index), 0x60 + index);
        }
        assert_eq!(e.mem.u32(this + 0x170), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_load_shaders_compiles_28_vertex_and_7_pixel_shaders() {
        let mut e = loader_rig();
        let this = e.mem.alloc(0x1b4);
        e.call(LOAD_SHADERS, &args![this]);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 35);
        for (index, format) in formats[..28].iter().enumerate() {
            assert_eq!(format[2..], [VERTEX_NAME_FORMAT_010BB198, index as u32]);
        }
        let tex = (MACRO_TEX_010AECAC, NL_EMPTY_STRING_01011584);
        let vc = (MACRO_VC_010AED64, NL_EMPTY_STRING_01011584);
        let falloff = (MACRO_FALLOFF_010BB204, NL_EMPTY_STRING_01011584);
        let skin = (MACRO_SKIN_010AEC10, NL_EMPTY_STRING_01011584);
        // A few vertex shaders: index 0 (TEX), 3 (no macros), 14 (TEX VC FALLOFF SKIN, the
        // block written in full), 15 (FALLOFF SKIN), 16 (particles), 20 (waterfog).
        let expected = [
            (0usize, NL_VERTEX_SOURCE_010BB20C, macro_words(&[tex])),
            (3, NL_VERTEX_SOURCE_010BB20C, [0; 8]),
            (
                14,
                NL_VERTEX_SOURCE_010BB20C,
                macro_words(&[tex, vc, falloff, skin]),
            ),
            (15, NL_VERTEX_SOURCE_010BB20C, macro_words(&[falloff, skin])),
            (16, PARTICLE_VERTEX_SOURCE_010BB1E0, [0; 8]),
            (20, WATERFOG_VERTEX_SOURCE_010BB1A8, [0; 8]),
        ];
        for (index, file, macros) in expected {
            let shader = e.mem.u32(this + 0xe4 + 4 * index as u32);
            let seen = compiled(&e, shader);
            assert_eq!(seen.file, file, "{index}");
            assert_eq!(seen.output, VERTEX_NAME_FORMAT_010BB198);
            assert_eq!(seen.profile, PROFILE_VS_2_0_010A93E4);
            assert!(!seen.pixel);
            assert_eq!(seen.macros, macros, "{index}");
        }
        for index in 0..28u32 {
            assert_ne!(e.mem.u32(this + 0xe4 + 4 * index), 0);
        }
        let pixel_files = [
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            NL_PIXEL_SOURCE_010BB178,
            WATERFOG_PIXEL_SOURCE_010BB148,
            WATERFOG_PIXEL_SOURCE_010BB148,
        ];
        for (index, file) in pixel_files.iter().enumerate() {
            let shader = e.mem.u32(this + 0x154 + 4 * index as u32);
            let seen = compiled(&e, shader);
            assert_eq!(seen.file, *file);
            assert_eq!(seen.output, PIXEL_OUTPUTS[index]);
            assert_eq!(seen.profile, NL_PROFILE_PS_2_A_010A93C4);
            assert!(seen.pixel);
        }
        let premultiplied = compiled(&e, e.mem.u32(this + 0x160));
        assert_eq!(
            premultiplied.macros[..6],
            [
                MACRO_PREMULT_ALPHA_010BB168,
                NL_EMPTY_STRING_01011584,
                MACRO_TEX_010AECAC,
                NL_EMPTY_STRING_01011584,
                MACRO_VC_010AED64,
                NL_EMPTY_STRING_01011584
            ]
        );
        let alpha_tested = compiled(&e, e.mem.u32(this + 0x16c));
        assert_eq!(
            alpha_tested.macros[..2],
            [MACRO_ALPHATESTING_010AEF00, NL_EMPTY_STRING_01011584]
        );
        // 0x1c + 7 blocks, one of them written in full.
        assert_eq!(calls(&e, MEMSET).len(), 34);
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn bs_shader_no_lighting_load_stages_and_passes_builds_thirty_passes() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x1b4);
        for offset in (0xe4u32..=0x16c).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        let made: Vec<u32> = calls(&e, NEW_PASS_00E804D0).iter().map(|a| a[0]).collect();
        assert_eq!(
            made,
            (0x6cu32..=0xe0)
                .step_by(4)
                .map(|o| this + o)
                .collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 18);
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        for setup in calls(&e, SETUP_DEFAULT_STAGE) {
            assert_eq!(setup[1..], [0, 3, 2, 1]);
        }
        // Spot checks: a pass with a stage, one without, one with the render state.
        let first = e.mem.u32(this + 0x6c);
        assert_eq!(calls(&e, PASS_SET_STAGE)[0][..2], [first, 0]);
        assert_eq!(e.mem.u32(first + 0x5c), e.mem.u32(this + 0xe4));
        assert_eq!(e.mem.u32(first + 0x44), e.mem.u32(this + 0x154));
        let third = e.mem.u32(this + 0x74);
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != third));
        assert_eq!(e.mem.u32(third + 0x5c), e.mem.u32(this + 0xe8));
        assert_eq!(e.mem.u32(third + 0x44), e.mem.u32(this + 0x158));
        assert!(render_states(&e, third).is_empty());
        let last = e.mem.u32(this + 0xe0);
        assert_eq!(render_states(&e, last), vec![(0xa8, 3)]);
        assert_eq!(e.mem.u32(last + 0x5c), e.mem.u32(this + 0x150));
        assert_eq!(e.mem.u32(last + 0x44), e.mem.u32(this + 0x16c));
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 8);
        // Only the last stage's reference is dropped.
        let stages_set: Vec<u32> = calls(&e, PASS_SET_STAGE).iter().map(|a| a[2]).collect();
        assert_eq!(e.mem.u32(stages_set[17] + 0x60), 1);
        assert_eq!(e.mem.u32(stages_set[0] + 0x60), 2);
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;
    use crate::units::bsshader::particleshader::constant_map_rig::*;

    const SLOTS: u32 = 0x0e9f_0000;
    const VTABLE: u32 = 0x0e9e_0000;

    /// [`rig`] with the pages the functions write mapped, a test vtable whose slots are
    /// recorded no-ops, and no-op doubles for the state setters.
    fn engine() -> Engine {
        let mut e = rig();
        for page in [
            0x0120_1000u32,
            0x0120_2000,
            0x011f_9000,
            0x011f_a000,
            0x011f_4000,
            0x011a_9000,
            0x010a_d000,
            0x0101_f000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        let slots: Vec<u32> = (0..0x60u32).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        for target in [
            STATE_CALL_00B98480,
            SET_Z_WRITE_ENABLE,
            STATE_CALL_00B97DE0,
            STATE_CALL_00B98070,
            STATE_CALL_00B97FA0,
            STATE_CALL_00B97FF0,
            SHADER_CALL_00BE1FF0,
            STAGE_CALL_00E7DE90,
            STAGE_CALL_00E7EB00,
            MEMCPY,
            MATRIX_UPDATE_00A88CD0,
            BS_SHADER_DESTRUCT,
            VECTOR_DESTRUCT_00EC5FCE,
            NI_FREE_SIZED_00AA1460,
            RENDERER_CALL_00E6FBB0,
            MAKE_IDENTITY,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e
    }

    fn with_vtable(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// A shader with the two flag objects at +0x170 and +0x178 (and +0x174).
    fn shader(e: &mut Engine) -> u32 {
        let this = with_vtable(e, 0x1b4);
        for offset in [0x170u32, 0x174, 0x178] {
            let target = e.mem.alloc(0x24);
            e.mem.set_u32(this + offset, target);
        }
        this
    }

    fn flag(e: &Engine, this: u32, offset: u32) -> u8 {
        e.mem.u8(e.mem.u32(this + offset) + 8)
    }

    fn fog_words(e: &Engine) -> Vec<u32> {
        (0..8)
            .map(|i| e.global::<u32>(FOG_01202148 + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00bc6970_sets_the_state_and_the_shader_pointer() {
        let mut e = engine();
        e.register(NI_POINTER_ASSIGN_00A5F860, |_, _| Ret::default());
        let this = with_vtable(&mut e, 0x1b4);
        for offset in (0x6cu32..=0xe0).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        for offset in (0x184u32..=0x1b0).step_by(4) {
            e.mem.set_u32(this + offset, 0x2000 + offset);
        }
        let cases: [(u32, u32, u32); 31] = [
            (0x42, 0x6c, 0x184),
            (0x43, 0x74, 0x188),
            (0x44, 0x70, 0x18c),
            (0x45, 0x78, 0x184),
            (0x46, 0x8c, 0x19c),
            (0x47, 0x94, 0x1a0),
            (0x48, 0x90, 0x1a4),
            (0x49, 0x98, 0x19c),
            (0x4a, 0x7c, 0x190),
            (0x4b, 0x84, 0x194),
            (0x4c, 0x80, 0x198),
            (0x4d, 0x88, 0x190),
            (0x4e, 0x9c, 0x1a8),
            (0x4f, 0xa4, 0x1ac),
            (0x50, 0xa0, 0x1b0),
            (0x51, 0xa8, 0x1a8),
            (0x2dc, 0xc4, 0x184),
            (0x2dd, 0xd4, 0x19c),
            (0x2e2, 0xc8, 0x184),
            (0x2e3, 0xd8, 0x19c),
            (0x2e8, 0xcc, 0x188),
            (0x2e9, 0xdc, 0x1a0),
            (0x2ee, 0xd0, 0x18c),
            (0x2ef, 0xe0, 0x1a4),
            // The release cases (pointer word 0 below).
            (0x52, 0xac, 0),
            (0x53, 0xb0, 0),
            (0x54, 0xb4, 0),
            (0x55, 0xb8, 0),
            (0x56, 0xbc, 0),
            (0x57, 0xc0, 0),
            (0x58, 0, 0),
        ];
        for (pass, state, pointer) in cases {
            e.set_global::<u32>(PASS_STATE_0126F74C, 0);
            let before = calls(&e, NI_POINTER_ASSIGN_00A5F860).len();
            e.call(0x00bc_6970, &args![this, pass]);
            let expected = if state == 0 { 0 } else { 0x1000 + state };
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), expected, "{pass:#x}");
            let assigned = calls(&e, NI_POINTER_ASSIGN_00A5F860);
            if pointer == 0 {
                assert_eq!(assigned.len(), before, "{pass:#x}");
            } else {
                assert_eq!(assigned[before], vec![this + 0x28, 0x2000 + pointer]);
            }
        }
        // Passes with no case: nothing.
        for pass in [0x41u32, 0x2db, 0x2de, 0x2f0, 0xffff_ffff] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 7);
            e.call(0x00bc_6970, &args![this, pass]);
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 7, "{pass:#x}");
        }
        // A release with a held pointer: its count drops, at 0 slot +4, and +0x28 is cleared.
        let held = with_vtable(&mut e, 8);
        e.mem.set_u32(held + 4, 1);
        e.mem.set_u32(this + 0x28, held);
        e.call(0x00bc_6970, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![held]]);
    }

    #[test]
    fn fn_00bc6de0_sets_the_pass_flags_and_the_fog() {
        let mut e = engine();
        let this = shader(&mut e);
        let fog = e.mem.alloc(0x64);
        e.mem.set_f32(fog + 0x20, 0.25);
        e.mem.set_f32(fog + 0x24, 0.5);
        e.mem.set_f32(fog + 0x28, 0.75);
        e.mem.set_f32(fog + 0x2c, 100.0);
        e.mem.set_f32(fog + 0x30, 300.0);
        e.mem.set_f32(fog + 0x60, 9.0);
        e.register_double(GET_FOG_PROPERTY, move |_, _| fog.into_ret());
        e.set_global::<u8>(FOG_SETTING_011F91C4, 3);
        e.call(0x00bc_6de0, &args![this, 0x56u32]);
        assert_eq!(calls(&e, GET_FOG_PROPERTY), vec![vec![3]]);
        assert_eq!((flag(&e, this, 0x170), flag(&e, this, 0x178)), (1, 1));
        let f = |v: f32| v.to_bits();
        assert_eq!(
            fog_words(&e),
            vec![
                f(300.0),
                f(200.0),
                f(9.0),
                0,
                f(0.25),
                f(0.5),
                f(0.75),
                f(1.0)
            ]
        );
        // Both distances 0: the far value and the colour from the exe's data.
        e.mem.set_f32(fog + 0x2c, 0.0);
        e.mem.set_f32(fog + 0x30, -0.0);
        e.set_global::<f32>(FOG_FAR_010AD6E0, 1000.0);
        for index in 0..4u32 {
            e.set_global::<u32>(FOG_COLOR_011A9BD0 + 4 * index, 0x70 + index);
        }
        e.call(0x00bc_6de0, &args![this, 0x4au32]);
        assert_eq!((flag(&e, this, 0x170), flag(&e, this, 0x178)), (0, 1));
        assert_eq!(
            fog_words(&e),
            vec![f(1000.0), 0, f(9.0), 0, 0x70, 0x71, 0x72, 0x73]
        );
        // A NaN distance is not zero; pass 0x52 sets only the first flag.
        e.mem.set_f32(fog + 0x2c, f32::NAN);
        e.call(0x00bc_6de0, &args![this, 0x52u32]);
        assert_eq!((flag(&e, this, 0x170), flag(&e, this, 0x178)), (1, 0));
        assert_eq!(fog_words(&e)[0], (-0.0f32).to_bits());
        assert_eq!(fog_words(&e)[7], f(1.0));
    }

    #[test]
    fn fn_00bc6de0_without_fog_only_sets_the_flags() {
        let mut e = engine();
        e.register(GET_FOG_PROPERTY, |_, _| Ret::default());
        let this = shader(&mut e);
        e.set_global::<u32>(FOG_01202148, 0x55);
        e.call(0x00bc_6de0, &args![this, 0x58u32]);
        assert_eq!((flag(&e, this, 0x170), flag(&e, this, 0x178)), (0, 0));
        assert_eq!(e.global::<u32>(FOG_01202148), 0x55);
    }

    #[test]
    fn fn_00bc6fa0_copies_the_sub_textures() {
        let mut e = engine();
        let this = shader(&mut e);
        let owner = e.mem.alloc(0xbc);
        let record = e.mem.alloc(0x70);
        e.mem.set_u32(owner + 0xb8, record);
        e.mem.set_u32(record + 0x68, 0x0e40_0000);
        e.call(0x00bc_6fa0, &args![this, owner, 0u32]);
        assert!(calls(&e, MEMCPY).is_empty());
        e.mem.set_u8(record + 0x6c, 3);
        e.call(0x00bc_6fa0, &args![this, owner, 0u32]);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![vec![SUBTEX_01202048, 0x0e40_0000, 0x30]]
        );
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x170) + 0x20), 3);
    }

    fn material_color(e: &Engine) -> Vec<f32> {
        (0..3)
            .map(|i| e.global::<f32>(MATERIAL_COLOR_01202010 + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00bc6fe0_sets_the_material_alpha_and_colour() {
        let mut e = engine();
        e.set_global::<f64>(TENTH_0101FFA0, 0.1);
        let property = e.mem.alloc(0x2c);
        e.mem.set_f32(property + 0x28, 0.5);
        let material = e.mem.alloc(0x44);
        for (index, value) in [0.2f32, 0.4, 0.6].iter().enumerate() {
            e.mem.set_f32(material + 0x28 + 4 * index as u32, *value);
        }
        e.mem.set_f32(material + 0x40, 2.0);
        e.call(0x00bc_6fe0, &args![0u32, property, material]);
        assert_eq!(e.global::<f32>(MATERIAL_ALPHA_0120201C), 0.5);
        assert_eq!(material_color(&e), vec![0.2, 0.4, 0.6]);
        // Alpha from 1.0 up (and NaN) gives 1.0; a colour below 0.1 is scaled.
        e.mem.set_f32(property + 0x28, f32::NAN);
        for (index, value) in [0.05f32, 0.01, 0.0].iter().enumerate() {
            e.mem.set_f32(material + 0x28 + 4 * index as u32, *value);
        }
        e.call(0x00bc_6fe0, &args![0u32, property, material]);
        assert_eq!(e.global::<f32>(MATERIAL_ALPHA_0120201C), 1.0);
        assert_eq!(material_color(&e), vec![0.1, 0.02, 0.0]);
        // The setting scales always; the colour pointer at +0x34 wins; a colour equal to
        // `011f4998` is replaced.
        e.set_global::<u8>(ALWAYS_SCALE_011F941E, 1);
        let other = e.mem.alloc(0xc);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(other + 4 * index as u32, *value);
            e.set_global::<f32>(COMPARED_011F4998 + 4 * index as u32, 2.0 * value);
            e.set_global::<f32>(REPLACEMENT_011A9B7C + 4 * index as u32, 7.0 + *value);
        }
        e.mem.set_u32(material + 0x34, other);
        e.call(0x00bc_6fe0, &args![0u32, property, material]);
        assert_eq!(material_color(&e), vec![8.0, 9.0, 10.0]);
    }

    #[test]
    fn fn_00bc7100_sets_the_light_fog_for_its_passes() {
        let mut e = engine();
        let this = shader(&mut e);
        e.set_global::<u32>(PASS_011F91E4, 0x2d7);
        e.call(0x00bc_7100, &args![this]);
        assert_eq!(flag(&e, this, 0x174), 0);
        e.set_global::<u32>(PASS_011F91E4, 0x2ef);
        for (index, (direction, color)) in [(1.0f32, 2.0f32), (3.0, 4.0), (5.0, 6.0)]
            .iter()
            .enumerate()
        {
            e.set_global::<f32>(DIRECTION_011FA2A0 + 4 * index as u32, *direction);
            e.set_global::<f32>(COLOR_011F9604 + 4 * index as u32, *color);
        }
        e.set_global::<f32>(OFFSET_011F9610, 4.0);
        e.set_global::<f32>(FAR_011F9618, 500.0);
        e.call(0x00bc_7100, &args![this]);
        assert_eq!(flag(&e, this, 0x174), 1);
        let f = |v: f32| v.to_bits();
        // 2 + 12 + 30 = 44, minus 4.
        assert_eq!(
            fog_words(&e),
            vec![f(500.0), 0, 0, 0, f(2.0), f(4.0), f(6.0), f(40.0)]
        );
    }

    #[test]
    fn fn_00bc7220_destroys_the_members_and_keeps_the_memory() {
        let mut e = engine();
        let this = with_vtable(&mut e, 0x1b4);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x70, pass);
        let vertex = object(&mut e, 1);
        e.mem.set_u32(this + 0xe8, vertex);
        let pixel = object(&mut e, 2);
        e.mem.set_u32(this + 0x158, pixel);
        e.mem.set_u32(this + 0x170, 7);
        e.call(0x00bc_7220, &args![this]);
        assert_eq!(e.mem.u32(this), NO_LIGHTING_VTABLE_010BB230);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex]]);
        assert_eq!(e.mem.u32(pixel + 4), 1);
        assert_eq!(e.mem.u32(this + 0x170), 0);
        assert_eq!(calls(&e, VECTOR_DESTRUCT_00EC5FCE).len(), 3);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
    }

    #[test]
    fn fn_00bc9390_destroys_the_shader() {
        let mut e = engine();
        let this = with_vtable(&mut e, 0x1b4);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x70, pass);
        let vertex = object(&mut e, 1);
        e.mem.set_u32(this + 0xe8, vertex);
        let pixel = object(&mut e, 2);
        e.mem.set_u32(this + 0x158, pixel);
        let declaration = object(&mut e, 1);
        e.mem.set_u32(this + 0x1b0, declaration);
        e.mem.set_u32(this + 0x170, 7);
        assert_eq!(e.call(0x00bc_9390, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NO_LIGHTING_VTABLE_010BB230);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            vec![vec![vertex], vec![declaration]]
        );
        assert_eq!(e.mem.u32(pixel + 4), 1);
        assert_eq!(e.mem.u32(this + 0x170), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![
                vec![this + 0x154, 4, 7, DESTRUCT_00B66F20],
                vec![this + 0xe4, 4, 0x1c, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 0x1e, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x1b4]]);
    }

    #[test]
    fn fn_00bc7710_and_00bc7720_answer_their_words() {
        let mut e = engine();
        let this = with_vtable(&mut e, 0x1b4);
        for offset in (0x184u32..=0x1b0).step_by(4) {
            e.mem.set_u32(this + offset, offset);
        }
        assert_eq!(e.call(0x00bc_7710, &args![this]).u32(), 0x0120_2040);
        let property = e.mem.alloc(0x64);
        let mut seen = vec![];
        for flags in [0u32, 2, 0x40, 0x42] {
            for (textured, map) in [(0u8, 0u32), (0x20, 0), (0x20, 1)] {
                e.mem.set_u32(property + 0x20, flags);
                e.mem.set_u8(property + 0x24, textured);
                e.mem.set_u32(property + 0x60, map);
                seen.push(e.call(0x00bc_7720, &args![this, 0u32, property]).u32());
            }
        }
        assert_eq!(
            seen,
            vec![
                0x184, 0x188, 0x18c, 0x19c, 0x1a0, 0x1a4, 0x190, 0x194, 0x198, 0x1a8, 0x1ac, 0x1b0
            ]
        );
    }

    /// A render state object at `0126f74c` with stage 0 (and 1) and two state objects at +0x44
    /// and +0x5c.
    fn state(e: &mut Engine) -> (u32, u32) {
        let state = e.mem.alloc(0x60);
        e.set_global::<u32>(RENDER_STATE_0126F74C, state);
        let stages = e.mem.alloc(8);
        e.mem.set_u32(state + 0x24, stages);
        let stage = e.mem.alloc(0x70);
        e.mem.set_u32(stages, stage);
        for offset in [0x44u32, 0x5c] {
            let object = with_vtable(e, 8);
            e.mem.set_u32(state + offset, object);
        }
        (state, stage)
    }

    fn record(e: &mut Engine, property: u32) -> u32 {
        let record = e.mem.alloc(0x18);
        e.mem.set_u32(record + 0xc, property);
        record
    }

    #[test]
    fn fn_00bc8d40_sets_the_property_texture_on_stage_0() {
        let mut e = engine();
        let (_, stage) = state(&mut e);
        let property = with_vtable(&mut e, 0x30);
        let record = record(&mut e, property);
        e.call(0x00bc_8d40, &args![0u32, record]);
        assert!(calls(&e, STAGE_CALL_00E7DE90).is_empty());
        e.register(SLOTS + 0xb4, |_, _| 0x0e66_0000u32.into_ret());
        e.register(SLOTS + 0xcc, |_, _| 5u32.into_ret());
        e.call(0x00bc_8d40, &args![0u32, record]);
        assert_eq!(e.mem.u32(stage + 8), 0x0e66_0000);
        assert_eq!(calls(&e, STAGE_CALL_00E7DE90), vec![vec![stage, 5]]);
        assert_eq!(calls(&e, STAGE_CALL_00E7EB00), vec![vec![stage, 0]]);
    }

    #[test]
    fn fn_00bc8d90_undoes_the_states() {
        let mut e = engine();
        let property = e.mem.alloc(0x28);
        let record = record(&mut e, property);
        e.call(0x00bc_8d90, &args![0u32, record]);
        assert!(calls(&e, STATE_CALL_00B98480).is_empty());
        assert_eq!(calls(&e, SET_Z_WRITE_ENABLE), vec![vec![1, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B97DE0), vec![vec![1, 0]]);
        e.mem.set_u32(property + 0x24, 0x1_0001);
        e.mem.set_u32(property + 0x20, 0x8000_0000);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u8(flags + 0x18, 1);
        e.mem.set_u32(record + 0x10, flags);
        e.call(0x00bc_8d90, &args![0u32, record]);
        assert_eq!(calls(&e, STATE_CALL_00B98480), vec![vec![0, 0]]);
        assert_eq!(calls(&e, SET_Z_WRITE_ENABLE).len(), 1);
        assert_eq!(calls(&e, STATE_CALL_00B97DE0).len(), 1);
        assert_eq!(calls(&e, STATE_CALL_00B98070), vec![vec![0, 0]]);
    }

    #[test]
    fn fn_00bc8e00_copies_the_texture_matrix() {
        let mut e = engine();
        let record = e.mem.alloc(0x18);
        e.call(0x00bc_8e00, &args![0u32, record]);
        let f = |v: f32| v.to_bits();
        let read = |e: &Engine| -> Vec<u32> {
            (0..9u32)
                .map(|i| e.global::<u32>(TEXTURE_MATRIX_01201FE0 + 0x10 * (i / 3) + 4 * (i % 3)))
                .collect()
        };
        assert_eq!(read(&e), vec![f(1.0), 0, 0, 0, f(1.0), 0, 0, 0, f(1.0)]);
        let textures = e.mem.alloc(0x24);
        let array = e.mem.alloc(4);
        let first = e.mem.alloc(0x10);
        let transform = e.mem.alloc(0x44);
        e.mem.set_u32(record + 0x14, textures);
        e.mem.set_u32(textures + 0x20, array);
        e.mem.set_u32(array, first);
        e.mem.set_u32(first + 0xc, transform);
        e.mem.set_u8(transform + 0x1c, 1);
        for index in 0..9u32 {
            e.mem
                .set_f32(transform + 0x20 + 4 * index, index as f32 + 2.0);
        }
        e.mem.set_u32(transform + 0x20, 0x7f80_0001);
        e.call(0x00bc_8e00, &args![0u32, record]);
        assert_eq!(calls(&e, MATRIX_UPDATE_00A88CD0), vec![vec![transform]]);
        let mut want: Vec<u32> = (0..9).map(|i| f(i as f32 + 2.0)).collect();
        want[0] = 0x7fc0_0001;
        assert_eq!(read(&e), want);
    }

    #[test]
    fn fn_00bc8ed0_sets_the_pass_states_or_falls_back() {
        let mut e = engine();
        e.register(STATE_CALL_00B97FF0, |_, _| 0x11u32.into_ret());
        e.register(SHADER_CALL_00BE1FF0, |_, _| 0x22u32.into_ret());
        let record = e.mem.alloc(4);
        e.set_global::<u32>(PASS_011F91E4, 0x51);
        assert_eq!(
            e.call(0x00bc_8ed0, &args![0x0e10_0000u32, record]).u32(),
            0x11
        );
        assert_eq!(calls(&e, STATE_CALL_00B97FA0), vec![vec![1, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B97FF0), vec![vec![6, 7, 0]]);
        e.set_global::<u32>(PASS_011F91E4, 0x52);
        assert_eq!(
            e.call(0x00bc_8ed0, &args![0x0e10_0000u32, record]).u32(),
            0x22
        );
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u8(flags + 0x18, 1);
        e.mem.set_u32(record, flags);
        e.set_global::<u32>(PASS_011F91E4, 0x4a);
        e.call(0x00bc_8ed0, &args![0x0e10_0000u32, record]);
        assert_eq!(
            calls(&e, SHADER_CALL_00BE1FF0),
            vec![vec![0x0e10_0000, record], vec![0x0e10_0000, record]]
        );
    }

    #[test]
    fn setup_geometry_constants_runs_the_set_ups_and_the_maps() {
        let mut e = engine();
        e.set_global::<f64>(TENTH_0101FFA0, 0.1);
        let (state, _) = state(&mut e);
        let scene = e.mem.alloc(0xc0);
        let camera = e.mem.alloc(0x70);
        e.mem.set_u32(scene + 0xb8, camera);
        e.mem.set_u32(camera + 0x34, 0x34);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, scene);
        e.set_global::<u32>(SCENE_HOLDER_011F91E0, holder);
        e.set_global::<u32>(PASS_011F91E4, 0x56);
        let this = shader(&mut e);
        e.mem.set_u32(this + 0x18, 0x0e18_0000);
        let map = with_vtable(&mut e, 8);
        e.mem.set_u32(this + 0x34, map);
        let other_target = with_vtable(&mut e, 8);
        e.mem.set_u32(this + 0x1c, other_target);
        let property = e.mem.alloc(0x80);
        e.mem.set_f32(property + 0x28, 0.5);
        e.mem.set_u32(property + 0x70, 0x0e01_0000);
        let material = e.mem.alloc(0x44);
        let record = record(&mut e, property);
        e.mem.set_u32(record + 8, material);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u16(flags + 0x18, 2 << 5);
        e.mem.set_u32(record, flags);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u8(other + 0x18, 1);
        e.mem.set_u32(record + 0x10, other);
        e.call(0x00bc_9a50, &args![this, record]);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0),
            vec![vec![0x0e18_0000, scene + 0x68, 0]]
        );
        assert!(calls(&e, MAKE_IDENTITY).is_empty());
        assert_eq!(e.global::<f32>(MATERIAL_ALPHA_0120201C), 0.5);
        // Pass 0x56 without bit 0x40: the falloff is cleared.
        assert_eq!(e.global::<u32>(FALLOFF_CONSTANT_01202030), 0);
        let shader_5c = e.mem.u32(state + 0x5c);
        assert_eq!(
            calls(&e, SLOTS + 0x78),
            vec![vec![
                map,
                shader_5c,
                scene,
                0,
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
        assert_eq!(e.global::<u32>(ALPHA_FLAGS_01202020), 0);
        assert_eq!(e.global::<f32>(ALPHA_FLAGS_01202020 + 4), 1.0);
        assert_eq!(calls(&e, SET_Z_WRITE_ENABLE), vec![vec![0, 0]]);
        assert_eq!(calls(&e, STATE_CALL_00B97DE0), vec![vec![0, 0]]);
        assert_eq!(calls(&e, SLOTS + 0x24), vec![vec![other_target, other]]);
        // A scene word +0xbc: the local identity transform with the translation.
        e.mem.set_u32(scene + 0xbc, 0xbc);
        e.set_global::<u32>(PASS_011F91E4, 0x52);
        e.call(0x00bc_9a50, &args![this, record]);
        let transform = calls(&e, MAKE_IDENTITY)[0][0];
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0)[1],
            vec![0x0e18_0000, transform, 0]
        );
        assert_eq!(e.global::<u32>(FALLOFF_CONSTANT_01202030), 0x0e01_0000);
    }

    #[test]
    fn no_lighting_create_constant_maps_adds_the_entries() {
        let mut e = map_rig();
        let this = e.mem.alloc(0x1b4);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.call(0x00bc_9cc0, &args![this]);
        let pixel = e.mem.u32(this + 0x30);
        let vertex = e.mem.u32(this + 0x34);
        assert_eq!(map_made_with(&e, pixel), (0x0e50_0000, 2));
        assert_eq!(map_made_with(&e, vertex), (0x0e50_0000, 0));
        let empty = NL_EMPTY_STRING_01011584;
        assert_eq!(
            entries(&e, pixel),
            vec![
                vec![
                    NAME_MATCOLOR_010BB390,
                    0x1000_0007,
                    0,
                    0,
                    1,
                    empty,
                    0x10,
                    4,
                    COLOR_01202010,
                    0
                ],
                vec![
                    NAME_MATALPHA_010BA2BC,
                    0x1000_0007,
                    0,
                    1,
                    1,
                    empty,
                    0x10,
                    4,
                    ALPHA_FLAGS_01202020,
                    0
                ],
                vec![
                    NAME_FOGCOLOR_010B8C48,
                    0x1000_0007,
                    0,
                    2,
                    1,
                    empty,
                    0x10,
                    4,
                    0x0120_2158,
                    0
                ],
                vec![
                    NAME_WATERFOG_010BA288,
                    0x1000_0009,
                    0,
                    5,
                    2,
                    empty,
                    0x20,
                    4,
                    WATERFOG_01202168,
                    0
                ],
            ]
        );
        let added = entries(&e, vertex);
        let names: Vec<u32> = added.iter().map(|a| a[0]).collect();
        assert_eq!(
            names,
            vec![
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                NAME_WORLD_VIEW_TRANSPOSE_010A1EE0,
                NAME_WORLD_TRANSPOSE_010A1F08,
                NAME_EYE_DIR_010A189C,
                NAME_TEXTURE_TRANSFORM_010BB37C,
                NAME_FOG_0103BFC8,
                NAME_FALLOFF_STRING_010BB374,
                NAME_SUBTEX_010BB36C,
                NAME_BONE_MATRIX3_010A1CAC,
            ]
        );
        assert_eq!(
            added[1][..5],
            [NAME_WORLD_VIEW_TRANSPOSE_010A1EE0, 0x2000_0009, 0, 0x24, 4]
        );
        assert_eq!(
            added[4][..9],
            [
                NAME_TEXTURE_TRANSFORM_010BB37C,
                0x1000_0008,
                0,
                4,
                3,
                empty,
                0x30,
                4,
                TEXTURE_MATRIX_01201FE0
            ]
        );
        assert_eq!(
            added[8][..5],
            [
                NAME_BONE_MATRIX3_010A1CAC,
                0x2000_0009,
                0x12_0000,
                0x40,
                0x36
            ]
        );
        // The lookups by name, in order, and their answers kept.
        let lookups: Vec<u32> = calls(&e, MAP_LOOKUP).iter().map(|a| a[1]).collect();
        assert_eq!(
            lookups,
            vec![
                NAME_WORLD_VIEW_TRANSPOSE_010A1EE0,
                NAME_WORLD_TRANSPOSE_010A1F08,
                NAME_EYE_DIR_010A189C,
                NAME_SUBTEX_010BB36C,
                NAME_BONE_MATRIX3_010A1CAC
            ]
        );
        assert_eq!(
            e.mem.u32(this + 0x178),
            NAME_WORLD_VIEW_TRANSPOSE_010A1EE0 + 1
        );
        assert_eq!(e.mem.u32(this + 0x174), NAME_WORLD_TRANSPOSE_010A1F08 + 1);
        assert_eq!(e.mem.u32(this + 0x17c), NAME_EYE_DIR_010A189C + 1);
        assert_eq!(e.mem.u32(this + 0x170), NAME_SUBTEX_010BB36C + 1);
        assert_eq!(e.mem.u32(this + 0x180), NAME_BONE_MATRIX3_010A1CAC + 1);
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

    const SLOTS: u32 = 0x0ef0_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x130);
        for offset in (0..0x130u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        for offset in (0..0x130u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00bc6940_gives_the_passes_to_slot_0x128() {
        let mut e = rig();
        let this = object(&mut e, 0xe4);
        for index in 0..0x1eu32 {
            e.mem.set_u32(this + 0x6c + 4 * index, 0x200 + index);
        }
        e.call(0x00bc_6940, &args![this]);
        let given: Vec<u32> = calls(&e, SLOTS + 0x128).iter().map(|a| a[1]).collect();
        assert_eq!(given, (0x200..0x21e).collect::<Vec<u32>>());
    }

    #[test]
    fn reload_shaders_loads_then_builds_passes() {
        use crate::units::bsshader::geometrydecalshader::shader_test_rig::pass_rig;
        let mut e = pass_rig();
        let this = e.mem.alloc(0x1a0);
        let answer = e.call(0x00bc_8d30, &args![this]).bool();
        let direct = {
            let mut e = pass_rig();
            let this = e.mem.alloc(0x1a0);
            bs_shader_no_lighting_load_shaders(&mut e, Ptr::new(this));
            bs_shader_no_lighting_load_stages_and_passes(&mut e, Ptr::new(this))
        };
        assert_eq!(answer, direct);
        assert!(!calls(&e, COMPILE_VERTEX_SHADER).is_empty());
    }

    #[test]
    fn fn_00bc8f20_sets_the_bone_constants() {
        let mut e = rig();
        e.register(RENDERER_CALL_00E6FE30, |_, _| Ret::default());
        e.set_global(RENDERER_011F9508, 0x9508u32);
        let shader = object(&mut e, 4);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x5c, shader);
        e.set_global(RENDER_STATE_0126F74C, pass);
        let this = e.mem.alloc(0x184);
        e.mem.set_u32(this + 0x180, 0x1800);
        let geometry = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0x20, 0x20);
        e.mem.set_u32(geometry + 0x28, 0x28);
        let skin = e.mem.alloc(0x24);
        e.mem.set_u32(skin + 4, 0x44);
        e.mem.set_u16(skin + 0x20, 5);
        let words = |skin: u32| args![this, 0u32, geometry, skin, 0u32, 0u32, 0u32, 0x77u32, 0u32];
        assert_eq!(e.call(0x00bc_8f20, &words(skin)).u32(), 0);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FE30),
            vec![vec![0x9508, geometry, 0x77, 0, 3, 1]]
        );
        assert_eq!(
            calls(&e, SLOTS + 0x78),
            vec![vec![shader, 0x1800, 0x28, 5, 0x20, 0x44]]
        );
        e.call(0x00bc_8f20, &words(0));
        assert_eq!(calls(&e, SLOTS + 0x78).len(), 1);
    }
}
