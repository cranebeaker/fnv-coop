//! `BSShader/watershader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1d while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_WATER: u32 = 0x1d;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `WaterShader::WaterShader` (`00b86bd0`), `WaterShader::LoadShaders` (`00b827c0`) and the
/// method `00b83dd0` of this unit (engine map; all `thiscall`, no words).
const CONSTRUCT: u32 = 0x00b8_6bd0;
const LOAD_SHADERS: u32 = 0x00b8_27c0;
const WATER_METHOD_00B83DD0: u32 = 0x00b8_3dd0;

// Translated from 00b86de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 0x11. No arguments. With the thread's allocation context (+0x2b4 of the thread
/// block) at 0x1d (put back at the end): makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 4 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 4, 0), (3, 7, 5, 1, 0)) and calls slot +0xb4. Then allocates 0x250
/// bytes and runs the constructor (`00b86bd0`; a null allocation is used as it is), calls
/// the shader's slot +0x4c, `LoadShaders` (`00b827c0`), `00b83dd0`, slot +0x11c, then +0xc0
/// with the declaration, and answers the shader.
// Translation incomplete: 00b86de0 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn water_shader_create_shader(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_WATER);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 4, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x250u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(WATER_METHOD_00B83DD0, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The water shader's vtable (`010af9e0`; no RTTI locator before it).
const WATER_SHADER_VTABLE_010AF9E0: u32 = 0x010a_f9e0;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
/// The globals the constructor sets: the shader itself (`011ff370`), four `float`s
/// (`011ff458`..`011ff464`) and the flags and counters at `011ff100`..`011ff11a`.
const WATER_SHADER_011FF370: u32 = 0x011f_f370;
const FLOATS_011FF458: u32 = 0x011f_f458;

// Translated from 00b86bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::WaterShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010af9e0`, constructs the `NiPointer`
/// arrays at +0x6c (0x4a, destructor `00b79930`) and +0x1a0 (0x26, destructor `00b66f20`) with
/// the words +0x194 to +0x19c cleared between them, clears +0x238 to +0x24c, sets +0x68 to 0xd
/// and the byte +0x24 to 1, clears (releasing what they hold) the `NiPointer`s at +0x23c,
/// +0x244, +0x238 and +0x248, keeps the shader in `011ff370`, stores 0.0 in the `float`s
/// `011ff458` to `011ff464`, clears the bytes `011ff104`, `011ff103`, `011ff102` and, unless the
/// shader model setting `011f91c0` is above 2 (signed), `011ff100`; then clears the 16-bit
/// `011ff118`, the byte `011ff11a`, the word `011ff110` and the 16-bit `011ff114`. Answers
/// `this`.
// Translation incomplete: 00b86bd0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xb for the base and the arrays) is not translated.
pub fn water_shader_water_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, WATER_SHADER_VTABLE_010AF9E0);
    construct_pointer_array(e, base + 0x6c, 0x4a, DESTRUCT_00B79930);
    for offset in [0x194u32, 0x198, 0x19c] {
        e.mem.set_u32(base + offset, 0);
    }
    construct_pointer_array(e, base + 0x1a0, 0x26, DESTRUCT_00B66F20);
    for offset in (0x238u32..=0x24c).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0xd);
    e.mem.set_u8(base + 0x24, 1);
    for offset in [0x23cu32, 0x244, 0x238, 0x248] {
        ni_pointer_clear(e, base + offset);
    }
    let above_model_2 = e.global::<i32>(SHADER_MODEL_011F91C0) > 2;
    e.set_global::<u32>(WATER_SHADER_011FF370, base);
    for index in 0..4u32 {
        e.set_global::<f32>(FLOATS_011FF458 + 4 * index, 0.0);
    }
    for flag in [0x011f_f104u32, 0x011f_f103, 0x011f_f102] {
        e.set_global::<u8>(flag, 0);
    }
    if !above_model_2 {
        e.set_global::<u8>(0x011f_f100, 0);
    }
    e.set_global::<u16>(0x011f_f118, 0);
    e.set_global::<u8>(0x011f_f11a, 0);
    e.set_global::<u32>(0x011f_f110, 0);
    e.set_global::<u16>(0x011f_f114, 0);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): the vertex file `"water\\2_ab\\v\\water.v.hlsl"`
/// (`010afc20`) with the macros `"VERTEX_COLORS"` (`010afc00`) and `"LIGHTING"` (`010afbe4`),
/// the names `"WATER000.vso"` .. `"WATER002.vso"` and the profile `"vs_1_1"` (`010a93ec`); the
/// pixel files `"water\\2_ab\\p\\water.p.hlsl"` (`010afbb8`, shader model above 4) or
/// `"water\\2_0\\p\\water.p.hlsl"` (`010afb9c`) and `"water\\2_ab\\p\\waterlighting.p.hlsl"`
/// (`010afb78`), copied into stack buffers; the pixel macros `"WATER"`, `"REFLECTIONS"`,
/// `"REFRACTIONS"`, `"DEPTH"`, `"FOG"`, `"INTERIORWATER"`, `"UNDERWATER"`, `"WADING"`,
/// `"STENCIL"`, `"LOD"`, each with the value `""` (`01011584`); the name format
/// `"WATER%03i.pso"` (`010afb1c`); the pixel profiles `"ps_2_0"`, `"ps_2_a"`, `"ps_2_b"`,
/// `"ps_3_0"`.
const W_VERTEX_SOURCE_010AFC20: u32 = 0x010a_fc20;
const W_MACRO_VERTEX_COLORS_010AFC00: u32 = 0x010a_fc00;
const W_MACRO_LIGHTING_010AFBE4: u32 = 0x010a_fbe4;
const W_PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const W_PIXEL_SOURCE_2_AB_010AFBB8: u32 = 0x010a_fbb8;
const W_PIXEL_SOURCE_2_0_010AFB9C: u32 = 0x010a_fb9c;
const W_LIGHTING_SOURCE_010AFB78: u32 = 0x010a_fb78;
const W_PIXEL_NAME_FORMAT_010AFB1C: u32 = 0x010a_fb1c;
const W_EMPTY: u32 = 0x0101_1584;
const W_WATER: u32 = 0x0102_3014;
const W_REFLECTIONS: u32 = 0x010a_fb6c;
const W_REFRACTIONS: u32 = 0x010a_fb58;
const W_DEPTH: u32 = 0x010a_fb64;
const W_FOG: u32 = 0x010a_ece0;
const W_INTERIORWATER: u32 = 0x010a_fb48;
const W_UNDERWATER: u32 = 0x010a_fb3c;
const W_WADING: u32 = 0x010a_fb34;
const W_STENCIL: u32 = 0x010a_fb2c;
const W_LOD: u32 = 0x0103_9578;
/// The pixel profiles by shader model: `"ps_3_0"` (`010a93b4`, model 7 and above), `"ps_2_b"`
/// (`010a93bc`, 6), `"ps_2_a"` (`010a93c4`, 5), `"ps_2_0"` (`010a93cc`, otherwise).
const W_PROFILE_PS_3_0_010A93B4: u32 = 0x010a_93b4;
const W_PROFILE_PS_2_B_010A93BC: u32 = 0x010a_93bc;
const W_PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;
const W_PROFILE_PS_2_0_010A93CC: u32 = 0x010a_93cc;
/// The `LoadShaders` frame (0x1070 bytes up to the return address) and its buffers: name
/// +0xd58, path +0xe5c, the pixel file +0xc58 and the lighting file +0xf60 (0x100 each).
const W_FRAME_BYTES: u32 = 0x1070;
const W_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xd58,
    path: 0xe5c,
};
const W_PIXEL_FILE_BUFFER: u32 = 0xc58;
const W_LIGHTING_FILE_BUFFER: u32 = 0xf60;
/// The vertex steps: the source block, the compiled name and the shader field.
const W_VERTEX_STEPS: [(ShaderSource, u32, u32); 3] = [
    (
        ShaderSource {
            offset: 0xb74,
            words: &[W_VERTEX_SOURCE_010AFC20, 0, 0, 0, 0],
            cleared: 0x38,
        },
        0x010a_fc10,
        0x194,
    ),
    (
        ShaderSource {
            offset: 0xc0c,
            words: &[
                W_VERTEX_SOURCE_010AFC20,
                W_MACRO_VERTEX_COLORS_010AFC00,
                W_EMPTY,
                0,
            ],
            cleared: 0x3c,
        },
        0x010a_fbf0,
        0x198,
    ),
    (
        ShaderSource {
            offset: 0xbc0,
            words: &[
                W_VERTEX_SOURCE_010AFC20,
                W_MACRO_LIGHTING_010AFBE4,
                W_EMPTY,
                0,
            ],
            cleared: 0x3c,
        },
        0x010a_fbd4,
        0x19c,
    ),
];

/// One pixel source block of the water `LoadShaders`: the lighting file or the water file in
/// its first word, the macro words after it and the bytes cleared after those.
struct WaterPixelBlock {
    lighting: bool,
    macros: &'static [u32],
    cleared: u32,
}

/// The 0x26 pixel source blocks, 0x4c bytes apart from +0x2c.
const W_PIXEL_BLOCKS: [WaterPixelBlock; 38] = [
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_DEPTH, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_REFRACTIONS, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_REFLECTIONS, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x34,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
        ],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_UNDERWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
        ],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER, W_EMPTY, W_WADING, W_EMPTY, W_DEPTH, W_EMPTY, W_FOG, W_EMPTY, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_WADING, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_DEPTH,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        cleared: 0x0,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFRACTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_INTERIORWATER,
            W_EMPTY,
            W_WADING,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[
            W_WATER,
            W_EMPTY,
            W_REFLECTIONS,
            W_EMPTY,
            W_LOD,
            W_EMPTY,
            W_FOG,
            W_EMPTY,
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
    WaterPixelBlock {
        lighting: false,
        macros: &[W_WATER, W_EMPTY, W_LOD, W_EMPTY, W_FOG, W_EMPTY, 0],
        cleared: 0x2c,
    },
    WaterPixelBlock {
        lighting: false,
        macros: &[W_STENCIL, W_EMPTY, 0],
        cleared: 0x3c,
    },
    WaterPixelBlock {
        lighting: true,
        macros: &[0],
        cleared: 0x44,
    },
    WaterPixelBlock {
        lighting: true,
        macros: &[W_WADING, W_EMPTY, 0],
        cleared: 0x3c,
    },
];

// Translated from 00b827c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. With the thread
/// allocation context (+0x2b4 of the thread block) at 0x1d (put back at the end): compiles the
/// three vertex shaders (`"WATER000.vso"` .. `"WATER002.vso"`, `vs_1_1`, without macros, with
/// `VERTEX_COLORS` and with `LIGHTING`) into the `NiPointer`s at +0x194, +0x198, +0x19c; picks
/// the pixel profile from the shader model setting (`011f91c0`: 7 and above `ps_3_0`, 6
/// `ps_2_b`, 5 `ps_2_a`, else `ps_2_0`); copies the water pixel file (the `2_ab` one above model
/// 4, else the `2_0` one) and the lighting file into 0x100-byte stack buffers (`00406d00`);
/// fills the 0x26 pixel source blocks (the buffers' addresses as file names, their macro sets,
/// the rest of some blocks cleared by `memset`) and compiles each whose file word is set as
/// `"WATER%03i.pso"` with its index into the `NiPointer` at +0x1a0 + 4 * index.
// Translation incomplete: 00b827c0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS stack cookie check (00ec408c) are not translated.
pub fn water_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_WATER);
    e.with_stack(W_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        let path = frame + W_BUFFERS.path;
        let name = frame + W_BUFFERS.name;
        for (block, output, slot) in W_VERTEX_STEPS {
            set_up_sources(e, frame, core::slice::from_ref(&block));
            let step = ShaderCompile {
                source: block.offset,
                output,
                profile: W_PROFILE_VS_1_1_010A93EC,
                pixel: false,
                slot,
            };
            compile_shaders(e, base, frame, &W_BUFFERS, &[step]);
        }
        let model = e.global::<i32>(SHADER_MODEL_011F91C0);
        let profile = if model >= 7 {
            W_PROFILE_PS_3_0_010A93B4
        } else if model >= 6 {
            W_PROFILE_PS_2_B_010A93BC
        } else if model >= 5 {
            W_PROFILE_PS_2_A_010A93C4
        } else {
            W_PROFILE_PS_2_0_010A93CC
        };
        let pixel_file = frame + W_PIXEL_FILE_BUFFER;
        let lighting_file = frame + W_LIGHTING_FILE_BUFFER;
        let water_source = if model > 4 {
            W_PIXEL_SOURCE_2_AB_010AFBB8
        } else {
            W_PIXEL_SOURCE_2_0_010AFB9C
        };
        e.call(FORMAT_00406D00, &args![pixel_file, 0x100u32, water_source]);
        e.call(
            FORMAT_00406D00,
            &args![lighting_file, 0x100u32, W_LIGHTING_SOURCE_010AFB78],
        );
        for (index, block) in W_PIXEL_BLOCKS.iter().enumerate() {
            let at = frame + 0x2c + 0x4c * index as u32;
            let file = if block.lighting {
                lighting_file
            } else {
                pixel_file
            };
            e.mem.set_u32(at, file);
            for (word, value) in block.macros.iter().enumerate() {
                e.mem.set_u32(at + 4 + 4 * word as u32, *value);
            }
            if block.cleared != 0 {
                let rest = at + 4 + 4 * block.macros.len() as u32;
                e.call(MEMSET, &args![rest, 0u32, block.cleared]);
            }
        }
        for index in 0..0x26u32 {
            let source = frame + 0x2c + 0x4c * index;
            let file = e.mem.u32(source);
            if file == 0 {
                continue;
            }
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, W_PIXEL_NAME_FORMAT_010AFB1C, index],
            );
            let shader = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![base, path, source + 4, profile, name],
                )
                .u32();
            ni_pointer_assign(e, base + 0x1a0 + 4 * index, shader);
        }
    });
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
}

/// The shader setters `00b79950` (vertex: the pass +0x5c) and `00b80600` (pixel: the pass
/// +0x44) (unnamed; `thiscall` on the pass, the shader) and the blend setter `00b80040`
/// (unnamed; `thiscall` on the pass, three words: the states 0x13 and 0x14, an unread word).
const SET_VERTEX_SHADER_00B79950: u32 = 0x00b7_9950;
const SET_PIXEL_SHADER_00B80600: u32 = 0x00b8_0600;
const SET_BLEND_00B80040: u32 = 0x00b8_0040;

/// How a pass of the second part of `00b83dd0` gets its shaders: both `NiPointer`
/// assignments expanded inline, the vertex one inline and the pixel one through `00b80600`,
/// or both through the setters.
#[derive(Clone, Copy)]
enum WaterAssign {
    Inline,
    InlineVertex,
    Helpers,
}

/// The second part of `00b83dd0`, in the exe's order: the pass pointer, the vertex and pixel
/// shader `NiPointer`s it takes, the word stored after them and its value, and how the
/// shaders are assigned.
const WATER_ASSIGNMENTS: [(u32, u32, u32, u32, u32, WaterAssign); 74] = [
    (0x6c, 0x194, 0x1a0, 0x011f_f120, 0x3e, WaterAssign::Inline),
    (0x70, 0x194, 0x1a4, 0x011f_f124, 0x3e, WaterAssign::Inline),
    (0x74, 0x194, 0x1a8, 0x011f_f128, 0x3e, WaterAssign::Inline),
    (0x78, 0x194, 0x1ac, 0x011f_f12c, 0x3e, WaterAssign::Inline),
    (0x7c, 0x194, 0x1b0, 0x011f_f130, 0x2e, WaterAssign::Inline),
    (0x80, 0x194, 0x1b4, 0x011f_f134, 0x2e, WaterAssign::Inline),
    (0x84, 0x194, 0x1b8, 0x011f_f138, 0x2e, WaterAssign::Inline),
    (0x88, 0x194, 0x1bc, 0x011f_f13c, 0x2e, WaterAssign::Inline),
    (0x8c, 0x194, 0x1c0, 0x011f_f140, 0x1e, WaterAssign::Inline),
    (0x90, 0x194, 0x1c4, 0x011f_f144, 0x1e, WaterAssign::Inline),
    (0x94, 0x194, 0x1c8, 0x011f_f148, 0x1e, WaterAssign::Inline),
    (0x98, 0x194, 0x1cc, 0x011f_f14c, 0x1e, WaterAssign::Inline),
    (0x9c, 0x194, 0x1d0, 0x011f_f150, 0xe, WaterAssign::Inline),
    (0xa0, 0x194, 0x1d4, 0x011f_f154, 0xe, WaterAssign::Inline),
    (0xa4, 0x194, 0x1d8, 0x011f_f158, 0xe, WaterAssign::Inline),
    (0xa8, 0x194, 0x1dc, 0x011f_f15c, 0xe, WaterAssign::Inline),
    (0xac, 0x194, 0x1e0, 0x011f_f160, 0xe, WaterAssign::Inline),
    (0xb0, 0x194, 0x1e4, 0x011f_f164, 0x3e, WaterAssign::Inline),
    (0xb4, 0x194, 0x1e8, 0x011f_f168, 0x3e, WaterAssign::Inline),
    (0xb8, 0x194, 0x1ec, 0x011f_f16c, 0x3e, WaterAssign::Inline),
    (0xbc, 0x194, 0x1f0, 0x011f_f170, 0x3e, WaterAssign::Inline),
    (0xc0, 0x194, 0x1f4, 0x011f_f174, 0x2e, WaterAssign::Inline),
    (0xc4, 0x194, 0x1f8, 0x011f_f178, 0x2e, WaterAssign::Inline),
    (0xc8, 0x194, 0x1fc, 0x011f_f17c, 0x2e, WaterAssign::Inline),
    (0xcc, 0x194, 0x200, 0x011f_f180, 0x2e, WaterAssign::Inline),
    (0xd0, 0x194, 0x204, 0x011f_f184, 0x1e, WaterAssign::Inline),
    (0xd4, 0x194, 0x208, 0x011f_f188, 0x1e, WaterAssign::Inline),
    (0xd8, 0x194, 0x20c, 0x011f_f18c, 0x1e, WaterAssign::Inline),
    (0xdc, 0x194, 0x210, 0x011f_f190, 0x1e, WaterAssign::Inline),
    (0xe0, 0x194, 0x214, 0x011f_f190, 0xe, WaterAssign::Inline),
    (0xe4, 0x194, 0x218, 0x011f_f198, 0xe, WaterAssign::Inline),
    (0xe8, 0x194, 0x21c, 0x011f_f19c, 0xe, WaterAssign::Inline),
    (0xec, 0x194, 0x220, 0x011f_f1a0, 0xe, WaterAssign::Inline),
    (0xf0, 0x194, 0x224, 0x011f_f1a4, 0x2e, WaterAssign::Inline),
    (0xf4, 0x194, 0x228, 0x011f_f1a8, 0x2e, WaterAssign::Inline),
    (0xf8, 0x194, 0x22c, 0x011f_f1ac, 0x0, WaterAssign::Inline),
    (0xfc, 0x19c, 0x230, 0x011f_f1b0, 0x4e, WaterAssign::Inline),
    (0x100, 0x19c, 0x234, 0x011f_f1b4, 0x4e, WaterAssign::Inline),
    (0x104, 0x198, 0x1a0, 0x011f_f1b8, 0x3e, WaterAssign::Inline),
    (0x108, 0x198, 0x1a4, 0x011f_f1bc, 0x3e, WaterAssign::Inline),
    (0x10c, 0x198, 0x1a8, 0x011f_f1c0, 0x3e, WaterAssign::Inline),
    (0x110, 0x198, 0x1ac, 0x011f_f1c4, 0x3e, WaterAssign::Inline),
    (0x114, 0x198, 0x1b0, 0x011f_f1c8, 0x2e, WaterAssign::Inline),
    (0x118, 0x198, 0x1b4, 0x011f_f1cc, 0x2e, WaterAssign::Inline),
    (0x11c, 0x198, 0x1b8, 0x011f_f1d0, 0x2e, WaterAssign::Inline),
    (0x120, 0x198, 0x1bc, 0x011f_f1d4, 0x2e, WaterAssign::Inline),
    (0x124, 0x198, 0x1c0, 0x011f_f1d8, 0x1e, WaterAssign::Inline),
    (0x128, 0x198, 0x1c4, 0x011f_f1dc, 0x1e, WaterAssign::Inline),
    (0x12c, 0x198, 0x1c8, 0x011f_f1e0, 0x1e, WaterAssign::Inline),
    (0x130, 0x198, 0x1cc, 0x011f_f1e4, 0x1e, WaterAssign::Inline),
    (
        0x134,
        0x198,
        0x1d0,
        0x011f_f1e8,
        0xe,
        WaterAssign::InlineVertex,
    ),
    (0x138, 0x198, 0x1d4, 0x011f_f1ec, 0xe, WaterAssign::Helpers),
    (0x13c, 0x198, 0x1d8, 0x011f_f1f0, 0xe, WaterAssign::Helpers),
    (0x140, 0x198, 0x1dc, 0x011f_f1f4, 0xe, WaterAssign::Helpers),
    (0x144, 0x198, 0x1e0, 0x011f_f1f8, 0xe, WaterAssign::Helpers),
    (0x148, 0x198, 0x1e4, 0x011f_f1fc, 0x3e, WaterAssign::Helpers),
    (0x14c, 0x198, 0x1e8, 0x011f_f200, 0x3e, WaterAssign::Helpers),
    (0x150, 0x198, 0x1ec, 0x011f_f204, 0x3e, WaterAssign::Helpers),
    (0x154, 0x198, 0x1f0, 0x011f_f208, 0x3e, WaterAssign::Helpers),
    (0x158, 0x198, 0x1f4, 0x011f_f20c, 0x2e, WaterAssign::Helpers),
    (0x15c, 0x198, 0x1f8, 0x011f_f210, 0x2e, WaterAssign::Helpers),
    (0x160, 0x198, 0x1fc, 0x011f_f214, 0x2e, WaterAssign::Helpers),
    (0x164, 0x198, 0x200, 0x011f_f218, 0x2e, WaterAssign::Helpers),
    (0x168, 0x194, 0x204, 0x011f_f21c, 0x1e, WaterAssign::Helpers),
    (0x16c, 0x194, 0x208, 0x011f_f220, 0x1e, WaterAssign::Helpers),
    (0x170, 0x194, 0x20c, 0x011f_f224, 0x1e, WaterAssign::Helpers),
    (0x174, 0x194, 0x210, 0x011f_f228, 0x1e, WaterAssign::Helpers),
    (0x178, 0x194, 0x214, 0x011f_f228, 0xe, WaterAssign::Helpers),
    (0x17c, 0x194, 0x218, 0x011f_f230, 0xe, WaterAssign::Helpers),
    (0x180, 0x194, 0x21c, 0x011f_f234, 0xe, WaterAssign::Helpers),
    (0x184, 0x194, 0x220, 0x011f_f238, 0xe, WaterAssign::Helpers),
    (0x188, 0x198, 0x22c, 0x011f_f23c, 0x0, WaterAssign::Helpers),
    (0x18c, 0x19c, 0x230, 0x011f_f240, 0x4e, WaterAssign::Helpers),
    (0x190, 0x19c, 0x234, 0x011f_f244, 0x4e, WaterAssign::Helpers),
];

/// A render state set on the pass the slot holds, read again first.
fn water_state(e: &mut Engine, slot: u32, state: u32, value: u32) {
    let pass = e.mem.u32(slot);
    pass_render_state(e, pass, state, value);
}

// Translated from 00b83dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `watershader.cpp`): the water shader's stages and passes
/// (the creator `00b86de0` runs it after `LoadShaders`), `thiscall`, no stack words; answers 1.
///
/// For each index from 0 to 0x49 whose pass pointer (+0x6c + 4 * index) is null: a new pass
/// (`00e804d0`) with five texture stages (made into one stack pointer cleared first,
/// `SetupDefaultStage` with the stage index and 3, 2, 1 (stages 2 and 3: 1, 2, 1),
/// `SetFilterMode(1)`, `SetStage`), the render states 0xf = 1, 0x18 = 0, 0x19 = 5; unless the
/// index is 0x21, 0x1b = 1 and then 0x13 = 5, 0x14 = 2 (through the same pass read) for
/// indices 0x24, 0x48, 0x25, 0x49, else the blend setter `00b80040(5, 6, 0)`; for indices
/// 0x11 to 0x1c and 0x37 to 0x42 the state 0x34 = 1, else for 0 to 0x1c, 0x26 to 0x42, 0x24 and
/// 0x48 the states 0x34 = 1, 0x38 = 6, 0x39 = 1, 0x3a = 1, 0x37 = 1; for indices 0x23 and 0x47
/// the states 0xa8 = 0, 0xe = 0, 0x34 = 1, 0x38 = 3, 0x39 = 0xff, 0x37 = 3, 0x3a = 0; then the
/// stage reference is dropped.
///
/// Then, for the 0x4a passes in order (see `WATER_ASSIGNMENTS`), the vertex shader (+0x5c) and
/// pixel shader (+0x44) assigned (inline or through `00b79950` / `00b80600`) and a word of the
/// table at `011ff120` set (the exe writes `011ff190` and `011ff228` twice and never
/// `011ff194` and `011ff22c`).
// Translation incomplete: 00b83dd0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00b83dd0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in 0..0x4au32 {
            let slot = base + 0x6c + 4 * index;
            if e.mem.u32(slot) != 0 {
                continue;
            }
            e.call(NEW_PASS_00E804D0, &args![slot]);
            e.mem.set_u32(local, 0);
            for (stage_index, setup) in [
                (0u32, [3u32, 2, 1]),
                (1, [3, 2, 1]),
                (2, [1, 2, 1]),
                (3, [1, 2, 1]),
                (4, [3, 2, 1]),
            ] {
                e.call(CREATE_NEW_STAGE, &args![local]);
                let stage = e.mem.u32(local);
                e.call(
                    SETUP_DEFAULT_STAGE,
                    &args![stage, stage_index, setup[0], setup[1], setup[2]],
                );
                let stage = e.mem.u32(local);
                e.call(SET_FILTER_MODE, &args![stage, 1u32]);
                let pass = e.mem.u32(slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            }
            water_state(e, slot, 0xf, 1);
            water_state(e, slot, 0x18, 0);
            water_state(e, slot, 0x19, 5);
            if index != 0x21 {
                if matches!(index, 0x24 | 0x48 | 0x25 | 0x49) {
                    water_state(e, slot, 0x1b, 1);
                    let pass = e.mem.u32(slot);
                    pass_render_state(e, pass, 0x13, 5);
                    pass_render_state(e, pass, 0x14, 2);
                } else {
                    water_state(e, slot, 0x1b, 1);
                    let pass = e.mem.u32(slot);
                    e.call(SET_BLEND_00B80040, &args![pass, 5u32, 6u32, 0u32]);
                }
            }
            if (0x11..=0x1c).contains(&index) || index.wrapping_sub(0x37) <= 0xb {
                water_state(e, slot, 0x34, 1);
            } else if (0..=0x1c).contains(&index)
                || (0x26..=0x42).contains(&index)
                || index == 0x24
                || index == 0x48
            {
                water_state(e, slot, 0x34, 1);
                water_state(e, slot, 0x38, 6);
                water_state(e, slot, 0x39, 1);
                water_state(e, slot, 0x3a, 1);
                water_state(e, slot, 0x37, 1);
            }
            if index == 0x23 || index == 0x47 {
                for (state, value) in [
                    (0xa8u32, 0u32),
                    (0xe, 0),
                    (0x34, 1),
                    (0x38, 3),
                    (0x39, 0xff),
                    (0x37, 3),
                    (0x3a, 0),
                ] {
                    water_state(e, slot, state, value);
                }
            }
            let stage = e.mem.u32(local);
            stage_release(e, stage);
        }
    });
    for (pass_offset, vertex, pixel, word, value, assign) in WATER_ASSIGNMENTS {
        let slot = base + pass_offset;
        match assign {
            WaterAssign::Inline => {
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + vertex);
                ni_pointer_assign(e, pass + 0x5c, shader);
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + pixel);
                ni_pointer_assign(e, pass + 0x44, shader);
            }
            WaterAssign::InlineVertex => {
                let pass = e.mem.u32(slot);
                let shader = e.mem.u32(base + vertex);
                ni_pointer_assign(e, pass + 0x5c, shader);
                let shader = e.mem.u32(base + pixel);
                let pass = e.mem.u32(slot);
                e.call(SET_PIXEL_SHADER_00B80600, &args![pass, shader]);
            }
            WaterAssign::Helpers => {
                let shader = e.mem.u32(base + vertex);
                let pass = e.mem.u32(slot);
                e.call(SET_VERTEX_SHADER_00B79950, &args![pass, shader]);
                let shader = e.mem.u32(base + pixel);
                let pass = e.mem.u32(slot);
                e.call(SET_PIXEL_SHADER_00B80600, &args![pass, shader]);
            }
        }
        e.set_global::<u32>(word, value);
    }
    true
}

/// The `eh vector destructor iterator` (`00ec5fce`, `stdcall`), `BSShader::~BSShader` (engine
/// map, `00be0f10`, no stack words), the Ni free (`00aa1460`, `cdecl`: block, size) and
/// `00be0870` (unnamed, `thiscall`, no stack words; answers a bool).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const CALL_00BE0870: u32 = 0x00be_0870;
/// The four default words (`011a9bd0`) and the two global blocks the water shader resets with
/// them (`011ff4c8`, 0xb0 bytes, and `011ff578`, 0x1f0 bytes).
const DEFAULT_WORDS_011A9BD0: u32 = 0x011a_9bd0;
const BLOCK_011FF4C8: u32 = 0x011f_f4c8;
const BLOCK_011FF578: u32 = 0x011f_f578;
/// The water shader's global constant entries (`011ff4a0` to `011ff4c4`).
const ENTRIES_011FF4A0: u32 = 0x011f_f4a0;
const ENTRIES_011FF4AC: u32 = 0x011f_f4ac;
const ENTRIES_END_011FF4C4: u32 = 0x011f_f4c4;

/// `REP MOVSD` of `count` words from `source` to `destination`, word by word upwards (the
/// blocks overlap, so the first words repeat through the destination).
fn move_words_up(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for index in 0..count {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(destination + 4 * index, word);
    }
}

// Translated from 00b824b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `watershader.cpp`): the water shader's destructor, no stack
/// words. Stores the vtable `010af9e0`; clears (releasing what they hold) the `NiPointer`s
/// +0x194, +0x198, +0x19c; drops and clears the 0x4a passes at +0x6c; clears the 0x26
/// `NiPointer`s at +0x1a0; fills the global blocks `011ff4c8` (0x2c words) and `011ff578` (0x7c
/// words) with the four words at `011a9bd0` (four set, then `REP MOVSD` up the block); then
/// the members' destructors release +0x24c down to +0x238 (not cleared before), destroy the
/// array +0x1a0 (0x26, destructor `00b66f20`), release +0x19c, +0x198, +0x194 again when not
/// null, destroy the array +0x6c (0x4a, `00b79930`), and `BSShader::~BSShader` (`00be0f10`)
/// runs.
// Translation incomplete: 00b824b0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xb for the members and the base) is not translated.
pub fn fn_00b824b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WATER_SHADER_VTABLE_010AF9E0);
    for offset in [0x194u32, 0x198, 0x19c] {
        ni_pointer_clear(e, base + offset);
    }
    for index in 0..0x4au32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..0x26u32 {
        ni_pointer_clear(e, base + 0x1a0 + 4 * index);
    }
    let words = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(DEFAULT_WORDS_011A9BD0 + offset));
    e.set_global::<u32>(BLOCK_011FF4C8 + 0xc, words[3]);
    e.set_global::<u32>(BLOCK_011FF4C8 + 4, words[1]);
    e.set_global::<u32>(BLOCK_011FF578 + 4, words[1]);
    let last = e.global::<u32>(DEFAULT_WORDS_011A9BD0 + 0xc);
    e.set_global::<u32>(BLOCK_011FF4C8, words[0]);
    e.set_global::<u32>(BLOCK_011FF4C8 + 8, words[2]);
    move_words_up(e, BLOCK_011FF4C8 + 0x10, BLOCK_011FF4C8, 0x28);
    e.set_global::<u32>(BLOCK_011FF578, words[0]);
    e.set_global::<u32>(BLOCK_011FF578 + 8, words[2]);
    e.set_global::<u32>(BLOCK_011FF578 + 0xc, last);
    move_words_up(e, BLOCK_011FF578 + 0x10, BLOCK_011FF578, 0x78);
    for offset in [0x24cu32, 0x248, 0x244, 0x240, 0x23c, 0x238] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_reference(e, held);
        }
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x1a0, 4u32, 0x26u32, DESTRUCT_00B66F20],
    );
    for offset in [0x19cu32, 0x198, 0x194] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_reference(e, held);
        }
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x6c, 4u32, 0x4au32, DESTRUCT_00B79930],
    );
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The record whose address `00b827b0` answers (`011ff498`).
const RECORD_011FF498: u32 = 0x011f_f498;

// Translated from 00b827b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010af9e0` slot +8, where the Xbox PDB's `NiObject` order has
/// `GetRTTI`): answers the address `011ff498`. No arguments.
pub fn fn_00b827b0(_e: &mut Engine) -> Ptr {
    Ptr::new(RECORD_011FF498)
}

// Translated from 00b861d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010af9e0` slot +0x4c), no stack words. Runs `00be0870`;
/// clears (releasing what they hold) +0x194, +0x198, +0x19c, drops and clears the 0x4a passes
/// at +0x6c, clears the 0x26 `NiPointer`s at +0x1a0 and the global entries `011ff4a0` to
/// `011ff4c4`; fills the global blocks `011ff4c8` and `011ff578` with the four words at
/// `011a9bd0`; answers what `00be0870` answered.
pub fn fn_00b861d0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let result = e.call(CALL_00BE0870, &args![base]).bool();
    for offset in [0x194u32, 0x198, 0x19c] {
        ni_pointer_clear(e, base + offset);
    }
    for index in 0..0x4au32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..0x26u32 {
        ni_pointer_clear(e, base + 0x1a0 + 4 * index);
    }
    let mut slot = ENTRIES_011FF4A0;
    while slot < ENTRIES_011FF4AC {
        ni_pointer_clear(e, slot);
        slot += 4;
    }
    let mut slot = ENTRIES_011FF4AC;
    while slot < ENTRIES_END_011FF4C4 {
        ni_pointer_clear(e, slot);
        slot += 4;
    }
    let words = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(DEFAULT_WORDS_011A9BD0 + offset));
    e.set_global::<u32>(BLOCK_011FF4C8, words[0]);
    e.set_global::<u32>(BLOCK_011FF4C8 + 8, words[2]);
    e.set_global::<u32>(BLOCK_011FF4C8 + 0xc, words[3]);
    e.set_global::<u32>(BLOCK_011FF4C8 + 4, words[1]);
    move_words_up(e, BLOCK_011FF4C8 + 0x10, BLOCK_011FF4C8, 0x28);
    e.set_global::<u32>(BLOCK_011FF578, words[0]);
    e.set_global::<u32>(BLOCK_011FF578 + 8, words[2]);
    e.set_global::<u32>(BLOCK_011FF578 + 0xc, words[3]);
    e.set_global::<u32>(BLOCK_011FF578 + 4, words[1]);
    move_words_up(e, BLOCK_011FF578 + 0x10, BLOCK_011FF578, 0x78);
    result
}

/// The shader manager's pass state (`0126f74c`, its +0x24 the table of constant blocks), the
/// pass number (`011f91e4`), the default texture (`011f9510`), the texture `011f943c`
/// (another rendered texture) and `BSRenderedTexture::GetTexture` (engine map, `004bc320`,
/// `thiscall`, one word).
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
const CURRENT_PASS_011F91E4: u32 = 0x011f_91e4;
const DEFAULT_TEXTURE_011F9510: u32 = 0x011f_9510;
const RENDERED_TEXTURE_011F943C: u32 = 0x011f_943c;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;

/// The pass state's constant block at `offset` of its table.
fn water_block(e: &Engine, state: u32, offset: u32) -> u32 {
    e.mem.u32(e.mem.u32(state + 0x24) + offset)
}

// Translated from 00b863a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010af9e0` slot +0x78), `thiscall`, `RET 4` (the property
/// state). Clears +8 of the pass state's (`0126f74c`) first five constant blocks; on passes
/// other than 0x288 and 0x289, with the state's +0xc water property: block 0 gets its +0x13c
/// rendered texture (`GetTexture(0)`) when set and its byte +0x80 is set, else the default
/// texture `011f9510`; block 1 the texture of `011f943c` when set and the byte +0x81 is set;
/// block 2 its +0x138 texture when set, else the default; block 3 its +0x148 when the byte +0x60
/// is set and it is not null; block 4 its +0x144 texture when set and the byte +0x63 is set;
/// then the shader's slot +0x108 runs.
pub fn fn_00b863a0(e: &mut Engine, this: Ptr, state: Ptr) {
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    let pass_state = e.global::<u32>(PASS_STATE_0126F74C);
    let property = e.mem.u32(state.addr() + 0xc);
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        let block = water_block(e, pass_state, offset);
        e.mem.set_u32(block + 8, 0);
    }
    if pass == 0x288 || pass == 0x289 {
        return;
    }
    let texture = e.mem.u32(property + 0x13c);
    if texture != 0 && e.mem.u8(property + 0x80) != 0 {
        let block = water_block(e, pass_state, 0);
        let value = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture, 0u32])
            .u32();
        e.mem.set_u32(block + 8, value);
    } else {
        let block = water_block(e, pass_state, 0);
        let value = e.global::<u32>(DEFAULT_TEXTURE_011F9510);
        e.mem.set_u32(block + 8, value);
    }
    let texture = e.global::<u32>(RENDERED_TEXTURE_011F943C);
    if texture != 0 && e.mem.u8(property + 0x81) != 0 {
        let block = water_block(e, pass_state, 4);
        let value = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture, 0u32])
            .u32();
        e.mem.set_u32(block + 8, value);
    }
    let texture = e.mem.u32(property + 0x138);
    if texture != 0 {
        let block = water_block(e, pass_state, 8);
        let value = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture, 0u32])
            .u32();
        e.mem.set_u32(block + 8, value);
    } else {
        let block = water_block(e, pass_state, 8);
        let value = e.global::<u32>(DEFAULT_TEXTURE_011F9510);
        e.mem.set_u32(block + 8, value);
    }
    if e.mem.u8(property + 0x60) != 0 {
        let texture = e.mem.u32(property + 0x148);
        if texture != 0 {
            let block = water_block(e, pass_state, 0xc);
            e.mem.set_u32(block + 8, texture);
        }
    }
    let texture = e.mem.u32(property + 0x144);
    if texture != 0 && e.mem.u8(property + 0x63) != 0 {
        let block = water_block(e, pass_state, 0x10);
        let value = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture, 0u32])
            .u32();
        e.mem.set_u32(block + 8, value);
    }
    e.vcall(this.addr(), 0x108, &args![]);
}

/// The current record (`011f91e0`), the default bound (`011f4288`), the renderer's `00e6fbb0`
/// (`thiscall` on the shader's +0x18, `RET 8`), the sun position (`011fa2a0`) and origin
/// (`011f474c`), the doubles `10.0` (`01020758`) and `1.0` (`01012070`), the `float`
/// `011ff108`, the extra block `011ff398`, `BSShaderManager::GetFogProperty` (engine map,
/// `00b55520`, `cdecl`, the byte `011f91c4`), `2^32` (`float`, `01099378`), the render-state
/// setters of `bsrenderstate_xenon.cpp` (`00b98070`, `00b98180`, `00b980c0`, `00b98230`,
/// `cdecl`) and the water constants written.
const CURRENT_RECORD_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const SUN_POSITION_011FA2A0: u32 = 0x011f_a2a0;
const ORIGIN_011F474C: u32 = 0x011f_474c;
const TEN_01020758: u32 = 0x0102_0758;
const ONE_01012070: u32 = 0x0101_2070;
const WATER_VALUE_011FF108: u32 = 0x011f_f108;
const EXTRA_011FF398: u32 = 0x011f_f398;
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
const FOG_KIND_011F91C4: u32 = 0x011f_91c4;
const TWO_POWER_32_01099378: u32 = 0x0109_9378;
const RENDER_STATE_00B98070: u32 = 0x00b9_8070;
const RENDER_STATE_00B98180: u32 = 0x00b9_8180;
const RENDER_STATE_00B980C0: u32 = 0x00b9_80c0;
const RENDER_STATE_00B98230: u32 = 0x00b9_8230;
const COMMON_011FF588: u32 = 0x011f_f588;
const ORIGIN_COPY_011FF558: u32 = 0x011f_f558;
const COMMON_011FF548: u32 = 0x011f_f548;
const COLOR_011FF568: u32 = 0x011f_f568;
const LIGHT_COUNT_011FF658: u32 = 0x011f_f658;
const LIGHT_COLORS_011FF6E8: u32 = 0x011f_f6e8;
/// Bits of `1.0f`.
const WATER_ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b864d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010af9e0` slot +0x7c), `thiscall`, `RET 4` (the property
/// state). With the current record's geometry and the state's +0xc water property: runs the
/// renderer's `00e6fbb0` with the world transform; fills the pixel constants from `011ff588`:
/// the sun position (`011fa2a0`) minus the origin (`011f474c`) with 0.0, the property's twelve
/// words +0x88, its floats +0x118 (`011ff5c8`) and +0x100 (`011ff5d0`), its +0x104 over 10.0
/// raised to 1.0 when below (`011ff5d4`), its +0x6c, +0x70, +0xd0 and the `float` `011ff108`
/// (`011ff5d8` on), the origin (`011ff5e8` and `011ff558`), its words +0xb8 to +0xc4
/// (`011ff5f8`); with a fog property (`GetFogProperty`): its far distance, far minus near, the
/// water property's +0x74, +0x78, its colour +0x20..+0x28 and the property's +0x120
/// (`011ff608`), its power (`011ff5cc`); the property's words +0xd8 to +0xe4 and the eight
/// words at `011ff398` (`011ff628`), its +0x124 four times (`011ff548`); the render states for
/// the pass (none of these for passes 0x288 and 0x289, which only get `00b98230` with the
/// property's +0x84; with the byte +0x60 set: `00b98070(1, 0)`, `00b98180(2, 0xff, +0x84, 0)`,
/// `00b980c0(0, 0, 3, 0)` (0 in the third word for passes 0x29a and 0x2ab) and
/// `00b98230(0xff, 0)`); its byte +0x82 four times as a `float` (`011ff568`); its light count
/// +0x130 as an unsigned `float` (`011ff658`); for each light of its list (+0x128) the
/// position (+0x8c) with its +0xe0 at `011ff668` + 0x10 * n and its colour (+0xd4..+0xdc times
/// its +0xc4, times the property's +0xd4) with 1.0 at `011ff6e8` + 0x10 * n; and last the slot
/// +0x78 of the shader's +0x30 and +0x34 objects with the pass state's +0x44 and +0x5c objects
/// and the geometry's words.
pub fn fn_00b864d0(e: &mut Engine, this: Ptr, state: Ptr) {
    let this = this.addr();
    let state = state.addr();
    let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    let geometry = e.mem.u32(record);
    let mut bound = e.mem.u32(geometry + 0x20);
    if bound == 0 {
        bound = DEFAULT_BOUND_011F4288;
    }
    let world = geometry + 0x68;
    let passes = e.mem.u32(e.mem.u32(geometry + 0xb8) + 0x34);
    let word_bc = e.mem.u32(geometry + 0xbc);
    let renderer = e.mem.u32(this + 0x18);
    e.call(RENDERER_CALL_00E6FBB0, &args![renderer, world, 0u32]);
    let mut sun = [0u32; 3];
    for (index, slot) in sun.iter_mut().enumerate() {
        let offset = 4 * index as u32;
        let value = x87_sub(
            x87_load32(e.global::<u32>(SUN_POSITION_011FA2A0 + offset)),
            x87_m32(e.global::<u32>(ORIGIN_011F474C + offset)),
        );
        *slot = x87_store32(value);
    }
    for (index, value) in sun.iter().enumerate() {
        e.set_global::<u32>(COMMON_011FF588 + 4 * index as u32, x87_quiet_bits(*value));
    }
    e.set_global::<u32>(COMMON_011FF588 + 0xc, 0);
    let property = e.mem.u32(state + 0xc);
    for word in 0..12u32 {
        let value = e.mem.u32(property + 0x88 + 4 * word);
        e.set_global::<u32>(COMMON_011FF588 + 0x10 + 4 * word, value);
    }
    let value = x87_quiet_bits(e.mem.u32(property + 0x118));
    e.set_global::<u32>(COMMON_011FF588 + 0x40, value);
    let value = x87_quiet_bits(e.mem.u32(property + 0x100));
    e.set_global::<u32>(COMMON_011FF588 + 0x48, value);
    let ten = f64::from_bits(e.global::<u64>(TEN_01020758));
    let scaled = x87_store32(x87_div(x87_load32(e.mem.u32(property + 0x104)), ten));
    let one = f64::from_bits(e.global::<u64>(ONE_01012070));
    // FCOM double; TEST AH,5: below the double the value becomes 1.0 (FLD1).
    let scaled = if x87_load32(scaled) < one {
        WATER_ONE_BITS
    } else {
        scaled
    };
    let origin = [0u32, 4, 8].map(|offset| e.global::<u32>(ORIGIN_011F474C + offset));
    e.set_global::<u32>(COMMON_011FF588 + 0x4c, scaled);
    for (index, offset) in [0x6cu32, 0x70, 0xd0].iter().enumerate() {
        let value = x87_quiet_bits(e.mem.u32(property + offset));
        e.set_global::<u32>(COMMON_011FF588 + 0x50 + 4 * index as u32, value);
    }
    let value = x87_quiet_bits(e.global::<u32>(WATER_VALUE_011FF108));
    e.set_global::<u32>(COMMON_011FF588 + 0x5c, value);
    for (index, value) in origin.iter().enumerate() {
        let value = x87_quiet_bits(*value);
        e.set_global::<u32>(COMMON_011FF588 + 0x60 + 4 * index as u32, value);
        e.set_global::<u32>(ORIGIN_COPY_011FF558 + 4 * index as u32, value);
    }
    for word in 0..4u32 {
        let value = e.mem.u32(property + 0xb8 + 4 * word);
        e.set_global::<u32>(COMMON_011FF588 + 0x70 + 4 * word, value);
    }
    let kind = u32::from(e.global::<u8>(FOG_KIND_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![kind]).u32();
    if fog != 0 {
        let far = x87_quiet_bits(e.mem.u32(fog + 0x30));
        let color = [0x20u32, 0x24, 0x28].map(|offset| e.mem.u32(fog + offset));
        let power = x87_quiet_bits(e.mem.u32(fog + 0x60));
        let range = x87_store32(x87_sub(x87_load32(far), x87_m32(e.mem.u32(fog + 0x2c))));
        let first = x87_quiet_bits(e.mem.u32(property + 0x74));
        let second = x87_quiet_bits(e.mem.u32(property + 0x78));
        let last = x87_quiet_bits(e.mem.u32(property + 0x120));
        let values = [
            far,
            range,
            first,
            second,
            x87_quiet_bits(color[0]),
            x87_quiet_bits(color[1]),
            x87_quiet_bits(color[2]),
            last,
        ];
        for (index, value) in values.iter().enumerate() {
            e.set_global::<u32>(COMMON_011FF588 + 0x80 + 4 * index as u32, *value);
        }
        e.set_global::<u32>(COMMON_011FF588 + 0x44, power);
    }
    for word in 0..4u32 {
        let value = e.mem.u32(property + 0xd8 + 4 * word);
        e.set_global::<u32>(COMMON_011FF588 + 0xa0 + 4 * word, value);
    }
    for word in 0..8u32 {
        let value = e.global::<u32>(EXTRA_011FF398 + 4 * word);
        e.set_global::<u32>(COMMON_011FF588 + 0xb0 + 4 * word, value);
    }
    let value = x87_quiet_bits(e.mem.u32(property + 0x124));
    for word in 0..4u32 {
        e.set_global::<u32>(COMMON_011FF548 + 4 * word, value);
    }
    let stencil = e.mem.u32(property + 0x84);
    if pass == 0x288 || pass == 0x289 {
        e.call(RENDER_STATE_00B98230, &args![stencil, 0u32]);
    } else if e.mem.u8(property + 0x60) != 0 {
        e.call(RENDER_STATE_00B98070, &args![1u32, 0u32]);
        let stencil = e.mem.u32(property + 0x84);
        e.call(RENDER_STATE_00B98180, &args![2u32, 0xffu32, stencil, 0u32]);
        let third = if pass == 0x29a || pass == 0x2ab {
            0u32
        } else {
            3
        };
        e.call(RENDER_STATE_00B980C0, &args![0u32, 0u32, third, 0u32]);
        e.call(RENDER_STATE_00B98230, &args![0xffu32, 0u32]);
    }
    let shade = x87_store32(f64::from(e.mem.u8(property + 0x82)));
    for word in 0..4u32 {
        e.set_global::<u32>(COLOR_011FF568 + 4 * word, shade);
    }
    let count = e.mem.u32(property + 0x130);
    let mut lights = f64::from(count as i32);
    if (count as i32) < 0 {
        lights = x87_add(lights, x87_m32(e.global::<u32>(TWO_POWER_32_01099378)));
    }
    e.set_global::<u32>(LIGHT_COUNT_011FF658, x87_store32(lights));
    let mut item = e.mem.u32(property + 0x128);
    let mut cursor = LIGHT_COLORS_011FF6E8;
    while item != 0 {
        let light = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if light == 0 {
            continue;
        }
        for (index, offset) in [0x8cu32, 0x90, 0x94, 0xe0].iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(light + offset));
            e.set_global::<u32>(cursor - 0x80 + 4 * index as u32, value);
        }
        let scale = x87_load32(x87_quiet_bits(e.mem.u32(light + 0xc4)));
        let red = x87_load32(e.mem.u32(light + 0xd4));
        let green = e.mem.u32(light + 0xd8);
        let blue = e.mem.u32(light + 0xdc);
        // FMULP ST2 (`DE CA`), FMUL ST(0),ST(1) (`D8 C9`), FMUL dword.
        let red = x87_store32(x87_mul(red, scale));
        let green = x87_store32(x87_mul(x87_load32(green), scale));
        let blue = x87_store32(x87_mul(scale, x87_m32(blue)));
        let factor = x87_load32(x87_quiet_bits(e.mem.u32(property + 0xd4)));
        let red = x87_store32(x87_mul(x87_load32(red), factor));
        let green = x87_store32(x87_mul(factor, x87_m32(green)));
        let blue = x87_store32(x87_mul(factor, x87_m32(blue)));
        for (index, value) in [red, green, blue].iter().enumerate() {
            e.set_global::<u32>(cursor + 4 * index as u32, x87_quiet_bits(*value));
        }
        e.set_global::<u32>(cursor + 0xc, WATER_ONE_BITS);
        cursor += 0x10;
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
}

/// The render-state setters `00b98480` and `00b98070` (`cdecl`, two words).
const RENDER_STATE_00B98480: u32 = 0x00b9_8480;

// Translated from 00b86b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010af9e0` slot +0x8c), `thiscall`, `RET 4` (the property
/// state; `this` is not read). With the state's +0xc property: `00b98480(0, 0)` when it is set
/// and has flag 0x10000 (+0x24); `00b98070(0, 0)` when the pass number (`011f91e4`) is neither
/// 0x288 nor 0x289 and the property's byte +0x60 is set (read without a null check).
pub fn fn_00b86b80(e: &mut Engine, _this: Ptr, state: Ptr) {
    let property = e.mem.u32(state.addr() + 0xc);
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    if property != 0 && e.mem.u32(property + 0x24) & 0x1_0000 != 0 {
        e.call(RENDER_STATE_00B98480, &args![0u32, 0u32]);
    }
    if pass != 0x288 && pass != 0x289 && e.mem.u8(property + 0x60) != 0 {
        e.call(RENDER_STATE_00B98070, &args![0u32, 0u32]);
    }
}

// Translated from 00b86db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `watershader.cpp`): the scalar deleting destructor (vtable
/// `010af9e0` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00b824b0`) and, when bit 0
/// of the flags is set, frees the 0x250 bytes (`00aa1460`). Answers `this`.
pub fn fn_00b86db0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b824b0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x250u32]);
    }
    this
}

/// The constant names of the water `CreateConstantMaps` (read in Ghidra), the data and the
/// global `NiPointer`s the entries are kept in.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const WORLD_TRANSPOSE_010A1F08: u32 = 0x010a_1f08;
const COMMON_DATA_010AFC60: u32 = 0x010a_fc60;
const VAR_AMOUNTS_010AFC54: u32 = 0x010a_fc54;
const FOG_DATA_010AFC4C: u32 = 0x010a_fc4c;
const DEPTH_DATA_010AF5B0: u32 = 0x010a_f5b0;
const SUN_LIGHT_DATA_010AFC3C: u32 = 0x010a_fc3c;
const LIGHT_DATA_010AF62C: u32 = 0x010a_f62c;

/// The map at `this + slot`'s entry for `name` (its slot +0x70) kept in the global `NiPointer`
/// `global`.
fn keep_water_entry(e: &mut Engine, this: u32, slot: u32, name: u32, global: u32) {
    let map = e.mem.u32(this + slot);
    let entry = e.vcall(map, 0x70, &args![name]).u32();
    ni_pointer_assign(e, global, entry);
}

// Translated from 00b86f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WaterShader::CreateConstantMaps` (Xbox PDB), no stack words. With the thread's allocation
/// context at 0x1d (put back at the end): when the vertex constant map (+0x34) is null, makes
/// one (kind 0) with `"WorldViewProjTranspose"` (slot +0x68: 0x20000009, 0, `""`) and
/// `"WorldTranspose"` (slot +0x68: 0x20000009, 4, `""`) and `"CommonData"` (0x10000009,
/// register 8, 3 registers, 0x30 bytes at `011ff548`), each entry kept in `011ff4a0`,
/// `011ff4a4`, `011ff4a8`; when the pixel map (+0x30) is null, makes one (kind 2) with
/// `"CommonData"` (register 1, 7 registers at `011ff588`), `"VarAmounts"` (8, `011ff5f8`),
/// `"FogData"` (0x1000000c, 9, 2 registers at `011ff608`), `"DepthData"` (0xb, `011ff628`),
/// `"SunLightData"` (0x1000000c, 0xc, 2 at `011ff638`) and `"LightData"` (0x10000009, 0xe,
/// 0x11 registers at `011ff658`), kept in `011ff4ac` to `011ff4c0`.
// Translation incomplete: 00b86f20 - the C++ exception frame (FS:[0] chain, unwind states 1 and 2 for the allocations) is not translated.
pub fn water_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let empty = CONSTANT_EMPTY_NAME_01011584;
    with_allocation_context(e, CONTEXT_WATER, |e| {
        if e.mem.u32(base + 0x34) == 0 {
            make_constant_map(e, base, 0x34, 0);
            for (name, register, global) in [
                (WORLD_VIEW_PROJ_TRANSPOSE_010A1E50, 0u32, ENTRIES_011FF4A0),
                (WORLD_TRANSPOSE_010A1F08, 4, ENTRIES_011FF4A0 + 4),
            ] {
                let map = e.mem.u32(base + 0x34);
                e.vcall(
                    map,
                    0x68,
                    &args![
                        name,
                        0x2000_0009u32,
                        register,
                        empty,
                        0u32,
                        0u32,
                        0u32,
                        0u32
                    ],
                );
                keep_water_entry(e, base, 0x34, name, global);
            }
            add_constant_entry(
                e,
                base,
                0x34,
                [
                    COMMON_DATA_010AFC60,
                    0x1000_0009,
                    0,
                    8,
                    3,
                    empty,
                    0x30,
                    4,
                    COMMON_011FF548,
                    0,
                ],
            );
            keep_water_entry(e, base, 0x34, COMMON_DATA_010AFC60, ENTRIES_011FF4A0 + 8);
        }
        if e.mem.u32(base + 0x30) != 0 {
            return;
        }
        make_constant_map(e, base, 0x30, 2);
        let entries = [
            (
                COMMON_DATA_010AFC60,
                0x1000_0009u32,
                1u32,
                7u32,
                0x70u32,
                COMMON_011FF588,
            ),
            (
                VAR_AMOUNTS_010AFC54,
                0x1000_0007,
                8,
                1,
                0x10,
                COMMON_011FF588 + 0x70,
            ),
            (
                FOG_DATA_010AFC4C,
                0x1000_000c,
                9,
                2,
                0x20,
                COMMON_011FF588 + 0x80,
            ),
            (
                DEPTH_DATA_010AF5B0,
                0x1000_0007,
                0xb,
                1,
                0x10,
                COMMON_011FF588 + 0xa0,
            ),
            (
                SUN_LIGHT_DATA_010AFC3C,
                0x1000_000c,
                0xc,
                2,
                0x20,
                COMMON_011FF588 + 0xb0,
            ),
            (
                LIGHT_DATA_010AF62C,
                0x1000_0009,
                0xe,
                0x11,
                0x110,
                LIGHT_COUNT_011FF658,
            ),
        ];
        for (index, (name, kind, register, count, size, data)) in entries.iter().enumerate() {
            add_constant_entry(
                e,
                base,
                0x30,
                [
                    *name, *kind, 0, *register, *count, empty, *size, 4, *data, 0,
                ],
            );
            keep_water_entry(e, base, 0x30, *name, ENTRIES_011FF4AC + 4 * index as u32);
        }
    });
}

// Translated from 00b861c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `WaterShader::LoadShaders` (`00b827c0`), then the
/// tail call to `00b83dd0` on `this`, whose answer it passes on.
pub fn fn_00b861c0(e: &mut Engine, this: Ptr) -> bool {
    water_shader_load_shaders(e, this);
    fn_00b83dd0(e, this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_61c0, fn_00b861c0(Ptr) -> bool),
        entry!(0x00b8_24b0, fn_00b824b0(Ptr)),
        entry!(0x00b8_27b0, fn_00b827b0() -> Ptr),
        entry!(0x00b8_61d0, fn_00b861d0(Ptr) -> bool),
        entry!(0x00b8_63a0, fn_00b863a0(Ptr, Ptr)),
        entry!(0x00b8_64d0, fn_00b864d0(Ptr, Ptr)),
        entry!(0x00b8_6b80, fn_00b86b80(Ptr, Ptr)),
        entry!(0x00b8_6db0, fn_00b86db0(Ptr, u32) -> Ptr),
        entry!(0x00b8_6f20, water_shader_create_constant_maps(Ptr)),
        entry!(0x00b8_3dd0, fn_00b83dd0(Ptr) -> bool),
        entry!(0x00b8_27c0, water_shader_load_shaders(Ptr)),
        entry!(0x00b8_6bd0, water_shader_water_shader(Ptr) -> Ptr),
        entry!(0x00b8_6de0, water_shader_create_shader() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn water_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, WATER_METHOD_00B83DD0]);
        let (shader, log) = run(&mut e, 0x00b8_6de0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 4, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x250]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (WATER_METHOD_00B83DD0, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_WATER);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.mem.write(0x011f_f100, &[0xee; 0x20]);
        e.mem.write(FLOATS_011FF458, &[0xee; 0x10]);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x250);
        e.mem.write(this, &[0xcc; 0x250]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        (e, this)
    }

    #[test]
    fn water_shader_constructor_builds_the_shader_and_resets_the_globals() {
        let (e, this) = construct(2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_VTABLE_010AF9E0);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x4a, DESTRUCT_00B79930),
                pointer_array(this + 0x1a0, 0x26, DESTRUCT_00B66F20),
            ]
        );
        for offset in [
            0x194u32, 0x198, 0x19c, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0xd);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.global::<u32>(WATER_SHADER_011FF370), this);
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(FLOATS_011FF458 + 4 * index), 0);
        }
        for byte in [0x011f_f100u32, 0x011f_f102, 0x011f_f103, 0x011f_f104] {
            assert_eq!(e.global::<u8>(byte), 0, "{byte:#x}");
        }
        assert_eq!(e.global::<u8>(0x011f_f101), 0xee);
        assert_eq!(e.global::<u16>(0x011f_f118), 0);
        assert_eq!(e.global::<u8>(0x011f_f11a), 0);
        assert_eq!(e.global::<u32>(0x011f_f110), 0);
        assert_eq!(e.global::<u16>(0x011f_f114), 0);
        assert_eq!(e.global::<u8>(0x011f_f116), 0xee);
    }

    #[test]
    fn water_shader_constructor_keeps_byte_011ff100_above_model_2() {
        let (e, _) = construct(3);
        assert_eq!(e.global::<u8>(0x011f_f100), 0xee);
        assert_eq!(e.global::<u8>(0x011f_f102), 0);
    }
}

#[cfg(test)]
mod load_shaders_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load(model: i32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 0x33);
        e.register(FORMAT_00406D00, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            if a[1] == 0x100 {
                // The file buffers keep the source string address for the test.
                e.mem.set_u32(a[0] + 4, 0x5eed);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x240);
        e.call(LOAD_SHADERS, &args![this]);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0x33);
        (e, this)
    }

    #[test]
    fn water_shader_load_shaders_compiles_three_vertex_and_38_pixel_shaders() {
        let (e, this) = load(5);
        let expected_vertex = [
            (0x194u32, 0x010a_fc10u32, macro_words(&[])),
            (
                0x198,
                0x010a_fbf0,
                macro_words(&[(W_MACRO_VERTEX_COLORS_010AFC00, W_EMPTY)]),
            ),
            (
                0x19c,
                0x010a_fbd4,
                macro_words(&[(W_MACRO_LIGHTING_010AFBE4, W_EMPTY)]),
            ),
        ];
        for (slot, output, macros) in expected_vertex {
            let seen = compiled(&e, e.mem.u32(this + slot));
            assert_eq!(
                seen,
                Compiled {
                    file: W_VERTEX_SOURCE_010AFC20,
                    output,
                    profile: W_PROFILE_VS_1_1_010A93EC,
                    pixel: false,
                    macros
                }
            );
        }
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats[3][1..], [0x100, W_PIXEL_SOURCE_2_AB_010AFBB8]);
        assert_eq!(formats[4][1..], [0x100, W_LIGHTING_SOURCE_010AFB78]);
        let pixel_names: Vec<&Vec<u32>> = formats.iter().skip(5).collect();
        assert_eq!(pixel_names.len(), 38);
        for (index, format) in pixel_names.iter().enumerate() {
            assert_eq!(format[2..], [W_PIXEL_NAME_FORMAT_010AFB1C, index as u32]);
        }
        let first = compiled(&e, e.mem.u32(this + 0x1a0));
        assert_eq!(first.profile, W_PROFILE_PS_2_A_010A93C4);
        assert!(first.pixel);
        assert_eq!(
            first.macros,
            [
                W_WATER,
                W_EMPTY,
                W_REFLECTIONS,
                W_EMPTY,
                W_REFRACTIONS,
                W_EMPTY,
                W_DEPTH,
                W_EMPTY
            ]
        );
        // MakePath got the stack buffer holding the pixel file, then the lighting file for the
        // last two blocks.
        let paths = calls(&e, MAKE_PATH);
        assert_eq!(paths.len(), 41);
        let water_buffer = paths[3][0];
        let lighting_buffer = paths[40][0];
        assert_ne!(water_buffer, lighting_buffer);
        assert!(paths[3..39].iter().all(|a| a[0] == water_buffer));
        assert_eq!(paths[39][0], lighting_buffer);
        for index in 0..38u32 {
            assert_ne!(e.mem.u32(this + 0x1a0 + 4 * index), 0);
        }
        let last = compiled(&e, e.mem.u32(this + 0x1a0 + 4 * 37));
        assert_eq!(last.macros[..2], [W_WADING, W_EMPTY]);
    }

    #[test]
    fn water_shader_load_shaders_profile_and_file_follow_the_shader_model() {
        for (model, profile, source) in [
            (7, W_PROFILE_PS_3_0_010A93B4, W_PIXEL_SOURCE_2_AB_010AFBB8),
            (6, W_PROFILE_PS_2_B_010A93BC, W_PIXEL_SOURCE_2_AB_010AFBB8),
            (4, W_PROFILE_PS_2_0_010A93CC, W_PIXEL_SOURCE_2_0_010AFB9C),
            (1, W_PROFILE_PS_2_0_010A93CC, W_PIXEL_SOURCE_2_0_010AFB9C),
        ] {
            let (e, this) = load(model);
            assert_eq!(compiled(&e, e.mem.u32(this + 0x1a0)).profile, profile);
            assert_eq!(calls(&e, FORMAT_00406D00)[3][2], source);
        }
    }
}

#[cfg(test)]
mod stages_and_passes_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn run(existing: Option<u32>) -> (Engine, u32) {
        let mut e = pass_rig();
        e.register(SET_BLEND_00B80040, |_, _| Ret::default());
        e.register(SET_VERTEX_SHADER_00B79950, |_, _| Ret::default());
        e.register(SET_PIXEL_SHADER_00B80600, |_, _| Ret::default());
        e.map(0x011f_f000, 0x1000);
        let this = e.mem.alloc(0x250);
        for offset in (0x194u32..=0x234).step_by(4) {
            let shader = object(&mut e, 1);
            e.mem.set_u32(this + offset, shader);
        }
        if let Some(offset) = existing {
            let pass = e.mem.alloc(0x70);
            e.mem.set_u32(this + offset, pass);
        }
        assert!(e.call(WATER_METHOD_00B83DD0, &args![this]).bool());
        (e, this)
    }

    #[test]
    fn fn_00b83dd0_builds_seventy_four_passes() {
        let (e, this) = run(None);
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 0x4a);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 5 * 0x4a);
        let setups: Vec<Vec<u32>> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .take(5)
            .map(|a| a[1..].to_vec())
            .collect();
        assert_eq!(
            setups,
            vec![
                vec![0, 3, 2, 1],
                vec![1, 3, 2, 1],
                vec![2, 1, 2, 1],
                vec![3, 1, 2, 1],
                vec![4, 3, 2, 1]
            ]
        );
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 521);
        assert_eq!(calls(&e, SET_BLEND_00B80040).len(), 69);
        let pass = |index: u32| e.mem.u32(this + 0x6c + 4 * index);
        assert_eq!(
            render_states(&e, pass(0)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x34, 1),
                (0x38, 6),
                (0x39, 1),
                (0x3a, 1),
                (0x37, 1)
            ]
        );
        assert_eq!(
            render_states(&e, pass(0x11)),
            vec![(0xf, 1), (0x18, 0), (0x19, 5), (0x1b, 1), (0x34, 1)]
        );
        assert_eq!(
            render_states(&e, pass(0x21)),
            vec![(0xf, 1), (0x18, 0), (0x19, 5)]
        );
        assert_eq!(
            render_states(&e, pass(0x23)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0xa8, 0),
                (0xe, 0),
                (0x34, 1),
                (0x38, 3),
                (0x39, 0xff),
                (0x37, 3),
                (0x3a, 0)
            ]
        );
        assert_eq!(
            render_states(&e, pass(0x24)),
            vec![
                (0xf, 1),
                (0x18, 0),
                (0x19, 5),
                (0x1b, 1),
                (0x13, 5),
                (0x14, 2),
                (0x34, 1),
                (0x38, 6),
                (0x39, 1),
                (0x3a, 1),
                (0x37, 1)
            ]
        );
        assert_eq!(calls(&e, SET_BLEND_00B80040)[0], vec![pass(0), 5, 6, 0]);
        // The second part: inline for the first passes, the setters for the last ones.
        assert_eq!(e.mem.u32(pass(0) + 0x5c), e.mem.u32(this + 0x194));
        assert_eq!(e.mem.u32(pass(0) + 0x44), e.mem.u32(this + 0x1a0));
        assert_eq!(e.global::<u32>(0x011f_f120), 0x3e);
        assert_eq!(e.global::<u32>(0x011f_f190), 0xe);
        assert_eq!(e.global::<u32>(0x011f_f194), 0);
        assert_eq!(e.global::<u32>(0x011f_f244), 0x4e);
        assert_eq!(calls(&e, SET_PIXEL_SHADER_00B80600).len(), 24);
        assert_eq!(calls(&e, SET_VERTEX_SHADER_00B79950).len(), 23);
        assert_eq!(
            calls(&e, SET_PIXEL_SHADER_00B80600)[0],
            vec![e.mem.u32(this + 0x134), e.mem.u32(this + 0x1d0)]
        );
        assert_eq!(
            e.mem.u32(e.mem.u32(this + 0x134) + 0x5c),
            e.mem.u32(this + 0x198)
        );
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn fn_00b83dd0_keeps_an_existing_pass() {
        let (e, this) = run(Some(0x70));
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 0x49);
        let kept = e.mem.u32(this + 0x70);
        assert!(calls(&e, PASS_SET_STAGE).iter().all(|a| a[0] != kept));
        assert_eq!(e.mem.u32(kept + 0x5c), e.mem.u32(this + 0x194));
        assert_eq!(e.mem.u32(kept + 0x44), e.mem.u32(this + 0x1a4));
    }
}

#[cfg(test)]
mod water_function_tests {
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
            0x0101_2000u32,
            0x0102_0000,
            0x0109_9000,
            0x011a_9000,
            0x011f_4000,
            0x011f_9000,
            0x011f_a000,
            0x011f_f000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        for (i, word) in [0x10u32, 0x11, 0x12, 0x13].iter().enumerate() {
            e.set_global(DEFAULT_WORDS_011A9BD0 + 4 * i as u32, *word);
        }
    }

    fn assert_blocks_reset(e: &Engine) {
        for word in 0..0x2cu32 {
            assert_eq!(
                e.global::<u32>(BLOCK_011FF4C8 + 4 * word),
                0x10 + (word & 3),
                "{word}"
            );
        }
        for word in 0..0x7cu32 {
            assert_eq!(
                e.global::<u32>(BLOCK_011FF578 + 4 * word),
                0x10 + (word & 3),
                "{word}"
            );
        }
    }

    #[test]
    fn fn_00b824b0_releases_resets_and_destroys() {
        let mut e = rig();
        map_globals(&mut e);
        noop(&mut e, &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0x250);
        let first = object(&mut e, 1);
        e.mem.set_u32(this + 0x194, first);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x6c + 4 * 0x49, pass);
        let member = object(&mut e, 2);
        e.mem.set_u32(this + 0x240, member);
        e.call(0x00b8_24b0, &args![this]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_VTABLE_010AF9E0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![first]]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        // The member pointer is released but not cleared.
        assert_eq!(e.mem.u32(member + 4), 1);
        assert_eq!(e.mem.u32(this + 0x240), member);
        assert_blocks_reset(&e);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x1a0, 4, 0x26, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 0x4a, DESTRUCT_00B79930],
            ]
        );
    }

    #[test]
    fn fn_00b86db0_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        map_globals(&mut e);
        noop(
            &mut e,
            &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460],
        );
        let this = e.mem.alloc(0x250);
        assert_eq!(e.call(0x00b8_6db0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x250]]);
        e.call(0x00b8_6db0, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00b827b0_answers_its_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b8_27b0, &args![]).u32(), RECORD_011FF498);
    }

    #[test]
    fn fn_00b861d0_clears_the_shader_and_the_globals() {
        let mut e = rig();
        map_globals(&mut e);
        e.register(CALL_00BE0870, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x250);
        let texture = object(&mut e, 1);
        e.mem.set_u32(this + 0x1a0 + 4 * 0x25, texture);
        let entry = object(&mut e, 1);
        e.set_global(ENTRIES_011FF4AC + 0x14, entry);
        assert!(e.call(0x00b8_61d0, &args![this]).bool());
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            vec![vec![texture], vec![entry]]
        );
        assert_eq!(e.global::<u32>(ENTRIES_011FF4AC + 0x14), 0);
        assert_blocks_reset(&e);
    }

    /// The pass state with five constant blocks and a water property in the state's +0xc.
    fn texture_rig(pass: u32) -> (Engine, u32, u32, u32, Vec<u32>) {
        let mut e = rig();
        map_globals(&mut e);
        let pass_state = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x14);
        let blocks: Vec<u32> = (0..5).map(|_| e.mem.alloc(0xc)).collect();
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *block);
            e.mem.set_u32(block + 8, 0x77);
        }
        e.mem.set_u32(pass_state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        e.set_global(CURRENT_PASS_011F91E4, pass);
        e.set_global(DEFAULT_TEXTURE_011F9510, 0xdefau32);
        let this = e.mem.alloc(0x250);
        let table0 = vtable(&mut e, 0);
        e.mem.set_u32(this, table0);
        noop(&mut e, &[slot(0, 0x108)]);
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, a| (a[0] + 1).into_ret());
        let property = e.mem.alloc(0x150);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        (e, this, state, property, blocks)
    }

    #[test]
    fn fn_00b863a0_sets_the_water_textures() {
        let (mut e, this, state, property, blocks) = texture_rig(0x10);
        e.mem.set_u32(property + 0x13c, 0x1000);
        e.mem.set_u8(property + 0x80, 1);
        e.set_global(RENDERED_TEXTURE_011F943C, 0x2000u32);
        e.mem.set_u8(property + 0x81, 1);
        e.mem.set_u8(property + 0x60, 1);
        e.mem.set_u32(property + 0x148, 0x4444);
        e.mem.set_u32(property + 0x144, 0x5000);
        e.mem.set_u8(property + 0x63, 1);
        e.call(0x00b8_63a0, &args![this, state]);
        let values: Vec<u32> = blocks.iter().map(|b| e.mem.u32(b + 8)).collect();
        assert_eq!(values, vec![0x1001, 0x2001, 0xdefa, 0x4444, 0x5001]);
        assert_eq!(calls(&e, slot(0, 0x108)), vec![vec![this]]);
    }

    #[test]
    fn fn_00b863a0_clears_only_on_the_stencil_passes() {
        let (mut e, this, state, _, blocks) = texture_rig(0x289);
        e.call(0x00b8_63a0, &args![this, state]);
        let values: Vec<u32> = blocks.iter().map(|b| e.mem.u32(b + 8)).collect();
        assert_eq!(values, vec![0; 5]);
        assert!(calls(&e, slot(0, 0x108)).is_empty());
        // Without the flags: the defaults where the code has them, nothing elsewhere.
        let (mut e, this, state, property, blocks) = texture_rig(0x10);
        e.mem.set_u32(property + 0x13c, 0x1000);
        e.mem.set_u32(property + 0x138, 0x3000);
        e.call(0x00b8_63a0, &args![this, state]);
        let values: Vec<u32> = blocks.iter().map(|b| e.mem.u32(b + 8)).collect();
        assert_eq!(values, vec![0xdefa, 0, 0x3001, 0, 0]);
    }

    #[test]
    fn fn_00b86b80_sets_the_render_states() {
        let mut e = rig();
        map_globals(&mut e);
        noop(&mut e, &[RENDER_STATE_00B98480, RENDER_STATE_00B98070]);
        let property = e.mem.alloc(0x70);
        e.mem.set_u32(property + 0x24, 0x1_0000);
        e.mem.set_u8(property + 0x60, 1);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        e.set_global(CURRENT_PASS_011F91E4, 0x10u32);
        e.call(0x00b8_6b80, &args![0u32, state]);
        assert_eq!(calls(&e, RENDER_STATE_00B98480), vec![vec![0, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B98070), vec![vec![0, 0]]);
        e.set_global(CURRENT_PASS_011F91E4, 0x288u32);
        e.mem.set_u32(property + 0x24, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b8_6b80, &args![0u32, state]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A rig for `00b864d0`: the record's geometry, a water property with two lights (the
    /// second item without a light) and a fog property when `fog`.
    fn draw_rig(pass: u32, fog: bool) -> (Engine, u32, u32, u32) {
        let mut e = rig();
        map_globals(&mut e);
        e.set_global(CURRENT_PASS_011F91E4, pass);
        e.set_global(TEN_01020758, 10.0f64);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(TWO_POWER_32_01099378, 4_294_967_296.0f32);
        for (i, v) in [5.0f32, 6.0, 7.0].iter().enumerate() {
            e.set_global(SUN_POSITION_011FA2A0 + 4 * i as u32, *v);
            e.set_global(ORIGIN_011F474C + 4 * i as u32, 1.0f32);
        }
        let this = e.mem.alloc(0x250);
        e.mem.set_u32(this + 0x18, 0x1818);
        let record = e.mem.alloc(0x10);
        let geometry = e.mem.alloc(0xc0);
        let passes = e.mem.alloc(0x38);
        e.mem.set_u32(passes + 0x34, 3);
        e.mem.set_u32(geometry + 0xb8, passes);
        e.mem.set_u32(record, geometry);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        let property = e.mem.alloc(0x140);
        e.mem.set_f32(property + 0x104, 5.0);
        e.mem.set_f32(property + 0xd4, 2.0);
        e.mem.set_u8(property + 0x82, 3);
        e.mem.set_u32(property + 0x84, 0x8484);
        e.mem.set_u32(property + 0x130, 2);
        let light = e.mem.alloc(0xe4);
        for (offset, value) in [
            (0x8cu32, 1.0f32),
            (0x90, 2.0),
            (0x94, 3.0),
            (0xe0, 4.0),
            (0xc4, 0.5),
            (0xd4, 2.0),
            (0xd8, 4.0),
            (0xdc, 6.0),
        ] {
            e.mem.set_f32(light + offset, value);
        }
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 8, light);
        e.mem.set_u32(property + 0x128, items[0]);
        let state = e.mem.alloc(0x14);
        e.mem.set_u32(state + 0xc, property);
        let pass_state = e.mem.alloc(0x60);
        e.set_global(PASS_STATE_0126F74C, pass_state);
        noop(
            &mut e,
            &[
                RENDERER_CALL_00E6FBB0,
                RENDER_STATE_00B98070,
                RENDER_STATE_00B98180,
                RENDER_STATE_00B980C0,
                RENDER_STATE_00B98230,
            ],
        );
        let fog_property = if fog {
            let object = e.mem.alloc(0x64);
            for (offset, value) in [
                (0x20u32, 0.1f32),
                (0x24, 0.2),
                (0x28, 0.3),
                (0x2c, 1.0),
                (0x30, 9.0),
                (0x60, 2.0),
            ] {
                e.mem.set_f32(object + offset, value);
            }
            object
        } else {
            0
        };
        e.register_double(GET_FOG_PROPERTY, move |_, _| fog_property.into_ret());
        (e, this, state, property)
    }

    #[test]
    fn fn_00b864d0_fills_the_water_constants() {
        let (mut e, this, state, property) = draw_rig(0x10, true);
        e.mem.set_u8(property + 0x60, 1);
        e.call(0x00b8_64d0, &args![this, state]);
        let floats = |e: &Engine, at: u32, n: u32| {
            (0..n)
                .map(|i| e.global::<f32>(at + 4 * i))
                .collect::<Vec<_>>()
        };
        assert_eq!(floats(&e, COMMON_011FF588, 4), vec![4.0, 5.0, 6.0, 0.0]);
        // 5 / 10 is below 1: 1.0.
        assert_eq!(e.global::<f32>(COMMON_011FF588 + 0x4c), 1.0);
        assert_eq!(floats(&e, COMMON_011FF588 + 0x60, 3), vec![1.0, 1.0, 1.0]);
        assert_eq!(floats(&e, COMMON_011FF588 + 0x80, 2), vec![9.0, 8.0]);
        assert_eq!(floats(&e, COMMON_011FF588 + 0x90, 3), vec![0.1, 0.2, 0.3]);
        assert_eq!(e.global::<f32>(COMMON_011FF588 + 0x44), 2.0);
        assert_eq!(floats(&e, COLOR_011FF568, 4), vec![3.0; 4]);
        assert_eq!(e.global::<f32>(LIGHT_COUNT_011FF658), 2.0);
        assert_eq!(
            floats(&e, LIGHT_COLORS_011FF6E8 - 0x80, 4),
            vec![1.0, 2.0, 3.0, 4.0]
        );
        // (2, 4, 6) * 0.5 * 2.
        assert_eq!(
            floats(&e, LIGHT_COLORS_011FF6E8, 4),
            vec![2.0, 4.0, 6.0, 1.0]
        );
        assert_eq!(e.global::<u32>(LIGHT_COLORS_011FF6E8 + 0x10), 0);
        assert_eq!(calls(&e, RENDER_STATE_00B98070), vec![vec![1, 0]]);
        assert_eq!(
            calls(&e, RENDER_STATE_00B98180),
            vec![vec![2, 0xff, 0x8484, 0]]
        );
        assert_eq!(calls(&e, RENDER_STATE_00B980C0), vec![vec![0, 0, 3, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B98230), vec![vec![0xff, 0]]);
    }

    #[test]
    fn fn_00b864d0_stencil_and_special_passes() {
        let (mut e, this, state, property) = draw_rig(0x288, false);
        e.mem.set_u8(property + 0x60, 1);
        e.mem.set_f32(property + 0x104, 20.0);
        e.call(0x00b8_64d0, &args![this, state]);
        assert_eq!(e.global::<f32>(COMMON_011FF588 + 0x4c), 2.0);
        assert_eq!(calls(&e, RENDER_STATE_00B98230), vec![vec![0x8484, 0]]);
        assert!(calls(&e, RENDER_STATE_00B98070).is_empty());
        let (mut e, this, state, property) = draw_rig(0x29a, false);
        e.mem.set_u8(property + 0x60, 1);
        e.call(0x00b8_64d0, &args![this, state]);
        assert_eq!(calls(&e, RENDER_STATE_00B980C0), vec![vec![0, 0, 0, 0]]);
    }

    #[test]
    fn water_create_constant_maps_keeps_every_entry() {
        const MAP_SLOTS: u32 = 0x0e30_0000;
        const MAP_VTABLE: u32 = 0x0e31_0000;
        let mut e = rig();
        map_globals(&mut e);
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        noop(&mut e, &[MAP_SLOTS + 8, MAP_SLOTS + 0x68]);
        e.register(MAP_SLOTS + 0x70, |e, _| {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, REFERENCE_VTABLE);
            entry.into_ret()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 4);
        let this = e.mem.alloc(0x250);
        e.call(0x00b8_6f20, &args![this]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 4);
        assert_eq!(calls(&e, MAP_SLOTS + 0x68).len(), 2);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 7);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(
            entries[6],
            vec![
                pixel,
                LIGHT_DATA_010AF62C,
                0x1000_0009,
                0,
                0xe,
                0x11,
                CONSTANT_EMPTY_NAME_01011584,
                0x110,
                4,
                LIGHT_COUNT_011FF658,
                0
            ]
        );
        let mut global = ENTRIES_011FF4A0;
        while global < ENTRIES_END_011FF4C4 - 4 {
            let entry = e.global::<u32>(global);
            assert_ne!(entry, 0, "{global:#x}");
            assert_eq!(e.mem.u32(entry + 4), 1);
            global += 4;
        }
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00b861c0_loads_the_shaders_then_builds_the_passes() {
        let mut e = pass_rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 5);
        e.register(FORMAT_00406D00, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        e.register(SET_BLEND_00B80040, |_, _| Ret::default());
        e.register(SET_VERTEX_SHADER_00B79950, |_, _| Ret::default());
        e.register(SET_PIXEL_SHADER_00B80600, |_, _| Ret::default());
        let this = e.mem.alloc(0x250);
        assert!(e.call(0x00b8_61c0, &args![this]).bool());
        assert_eq!(calls(&e, COMPILE_VERTEX_SHADER).len(), 3);
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 0x4a);
        // The shaders exist before the first pass is made.
        let log = e.call_log.as_ref().unwrap();
        let last_compile = log
            .iter()
            .rposition(|c| c.0 == COMPILE_PIXEL_SHADER)
            .unwrap();
        let first_pass = log.iter().position(|c| c.0 == NEW_PASS_00E804D0).unwrap();
        assert!(last_compile < first_pass);
    }
}
