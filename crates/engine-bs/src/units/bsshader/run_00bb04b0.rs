//! `BSShader/run_00bb04b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bb04b0` (this unit's first function, unnamed, `thiscall`, no
/// words).
const SHADER_CONSTRUCT_00BB04B0: u32 = 0x00bb_04b0;

// Translated from 00bb0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x1e. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`, 3 entries, 2 streams), sets its entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)), calls slot +0x94 with (1, 0,
/// 6, 0x80000000, 2, 5, 1, 0) and slot +0xb4. Then allocates 0xac bytes and runs the
/// constructor `00bb04b0` (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x148, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bb0660 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bb0660(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 2u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![1u32, 0u32, 6u32, 0x8000_0000u32, 2u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BB04B0, &args![block]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// `BSShader::BSShader` (engine map, `00be2200`, `thiscall`, no stack words).
const BS_SHADER_CONSTRUCT: u32 = 0x00be_2200;
/// The shader's vtable (`010b8e80`; no RTTI locator before it).
const SHADER_VTABLE_010B8E80: u32 = 0x010b_8e80;
/// The three words copied to +0xa0 (`011a9b7c`, `011a9b80`, `011a9b84`).
const WORDS_011A9B7C: u32 = 0x011a_9b7c;

// Translated from 00bb04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the constructor of shader 0x1e, `thiscall`, no
/// stack words. Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010b8e80`, clears
/// the word +0x9c, stores 0.0 (`FLDZ`) at +0xa0, +0xa4, +0xa8, sets +0x68 to 0x12, stores
/// 0.0 at +0x90, +0x94, +0x98, then copies the words at `011a9b7c`, `011a9b80`, `011a9b84`
/// to +0xa0, +0xa4, +0xa8 and sets the byte +0x24 to 1. Answers `this`.
pub fn fn_00bb04b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_VTABLE_010B8E80);
    e.mem.set_u32(base + 0x9c, 0);
    for offset in [0xa0u32, 0xa4, 0xa8] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0x12);
    for offset in [0x90u32, 0x94, 0x98] {
        e.mem.set_f32(base + offset, 0.0);
    }
    for index in 0..3u32 {
        let word = e.global::<u32>(WORDS_011A9B7C + 4 * index);
        e.mem.set_u32(base + 0xa0 + 4 * index, word);
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The precipitation shader's global vertex shaders (`012005d0`, four `NiPointer`s), pixel
/// shaders (`012005c8`, two) and pass (`012005e0`).
const VERTEX_SHADERS_012005D0: u32 = 0x0120_05d0;
const PIXEL_SHADERS_012005C8: u32 = 0x0120_05c8;
const PASS_012005E0: u32 = 0x0120_05e0;
/// Strings of the precipitation loaders (read in Ghidra): `"precipitation\\precipitation.v.hlsl"`
/// (`010b8e28`), `"precipitation\\precipitation.p.hlsl"` (`010b8e5c`), the macros
/// `"BILLBOARD_UP"` (`010b8e18`), `"BILLBOARD_FACE"` (`010b8e08`), `"SNOW"` (`010b8e00`), the
/// formats `"PRECIP%03i.vso"` (`010b8df0`) and `"PRECIP%03i.pso"` (`010b8e4c`).
const PRECIPITATION_VERTEX_010B8E28: u32 = 0x010b_8e28;
const PRECIPITATION_PIXEL_010B8E5C: u32 = 0x010b_8e5c;
const MACRO_BILLBOARD_UP_010B8E18: u32 = 0x010b_8e18;
const MACRO_BILLBOARD_FACE_010B8E08: u32 = 0x010b_8e08;
const MACRO_SNOW_010B8E00: u32 = 0x010b_8e00;
const FORMAT_PRECIP_VERTEX_010B8DF0: u32 = 0x010b_8df0;
const FORMAT_PRECIP_PIXEL_010B8E4C: u32 = 0x010b_8e4c;
/// The vertex profile chooser `00b4f3e0` (no arguments) and the pixel one `00b4f380`
/// (`cdecl`, a byte) (unit `bsshadermanager.cpp`, unnamed; they answer a profile string).
const VERTEX_PROFILE_00B4F3E0: u32 = 0x00b4_f3e0;
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// The vertex loader's frame (saved registers up to the cookie) and buffers, and its four
/// source blocks.
const PRECIP_VERTEX_FRAME_BYTES: u32 = 0x350;
const PRECIP_VERTEX_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x144,
    path: 0x248,
};
const PRECIP_VERTEX_SOURCES: [ShaderSource; 4] = [
    ShaderSource {
        offset: 0x14,
        words: &[
            PRECIPITATION_VERTEX_010B8E28,
            MACRO_BILLBOARD_UP_010B8E18,
            0,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x60,
        words: &[
            PRECIPITATION_VERTEX_010B8E28,
            MACRO_BILLBOARD_FACE_010B8E08,
            0,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xac,
        words: &[
            PRECIPITATION_VERTEX_010B8E28,
            MACRO_BILLBOARD_UP_010B8E18,
            0,
            MACRO_SNOW_010B8E00,
            0,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xf8,
        words: &[
            PRECIPITATION_VERTEX_010B8E28,
            MACRO_BILLBOARD_FACE_010B8E08,
            0,
            MACRO_SNOW_010B8E00,
            0,
            0,
        ],
        cleared: 0x34,
    },
];

// Translated from 00bb01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b8e80` slot +0x140): the precipitation shader's vertex
/// shaders, no stack words. Fills four source blocks on its stack (`precipitation.v.hlsl` with
/// `BILLBOARD_UP`, `BILLBOARD_FACE`, and each with `SNOW`; the rest cleared by `memset`) and
/// compiles each (`MakePath`, the name `"PRECIP%03i.vso"` with the index, the profile from
/// `00b4f3e0` asked for each one, `CompileVertexShaderHLSL`) into the global `NiPointer`s at
/// `012005d0`.
// Translation incomplete: 00bb01c0 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bb01c0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(PRECIP_VERTEX_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &PRECIP_VERTEX_SOURCES);
        let path = frame + PRECIP_VERTEX_BUFFERS.path;
        let name = frame + PRECIP_VERTEX_BUFFERS.name;
        for (index, source) in PRECIP_VERTEX_SOURCES.iter().enumerate() {
            let index = index as u32;
            let block = frame + source.offset;
            let file = e.mem.u32(block);
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, FORMAT_PRECIP_VERTEX_010B8DF0, index],
            );
            let profile = e.call(VERTEX_PROFILE_00B4F3E0, &args![]).u32();
            let shader = e
                .call(
                    COMPILE_VERTEX_SHADER,
                    &args![this, path, block + 4, profile, name],
                )
                .u32();
            ni_pointer_assign(e, VERTEX_SHADERS_012005D0 + 4 * index, shader);
        }
    });
}

/// The pixel loader's frame and buffers, and its two source blocks.
const PRECIP_PIXEL_FRAME_BYTES: u32 = 0x2b8;
const PRECIP_PIXEL_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xac,
    path: 0x1b0,
};
const PRECIP_PIXEL_SOURCES: [ShaderSource; 2] = [
    ShaderSource {
        offset: 0x14,
        words: &[PRECIPITATION_PIXEL_010B8E5C, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x60,
        words: &[PRECIPITATION_PIXEL_010B8E5C, MACRO_SNOW_010B8E00, 0, 0],
        cleared: 0x3c,
    },
];

// Translated from 00bb0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b8e80` slot +0x144): the precipitation shader's pixel
/// shaders, no stack words. Fills two source blocks (`precipitation.p.hlsl` alone and with
/// `SNOW`) and compiles each (`MakePath`, `"PRECIP%03i.pso"` with the index, the profile from
/// `00b4f380(0)` asked for each one, `CompilePixelShaderHLSL`) into the global `NiPointer`s at
/// `012005c8`.
// Translation incomplete: 00bb0360 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bb0360(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(PRECIP_PIXEL_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &PRECIP_PIXEL_SOURCES);
        let path = frame + PRECIP_PIXEL_BUFFERS.path;
        let name = frame + PRECIP_PIXEL_BUFFERS.name;
        for (index, source) in PRECIP_PIXEL_SOURCES.iter().enumerate() {
            let index = index as u32;
            let block = frame + source.offset;
            let file = e.mem.u32(block);
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, FORMAT_PRECIP_PIXEL_010B8E4C, index],
            );
            let profile = e.call(PIXEL_PROFILE_00B4F380, &args![0u32]).u32();
            let shader = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![this, path, block + 4, profile, name],
                )
                .u32();
            ni_pointer_assign(e, PIXEL_SHADERS_012005C8 + 4 * index, shader);
        }
    });
}

/// The precipitation shader's global shaders cleared (releasing what they hold) and its global
/// pass dropped and cleared.
fn clear_precipitation_globals(e: &mut Engine) {
    for index in 0..4u32 {
        ni_pointer_clear(e, VERTEX_SHADERS_012005D0 + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, PIXEL_SHADERS_012005C8 + 4 * index);
    }
    pass_pointer_clear(e, PASS_012005E0);
}

/// `BSShader::~BSShader` (engine map, `00be0f10`, no stack words), `00be0870` (unnamed,
/// `thiscall`, no stack words; answers a bool) and the Ni free (`00aa1460`, `cdecl`).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const CALL_00BE0870: u32 = 0x00be_0870;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bb0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the precipitation shader's destructor, no stack
/// words. Stores the vtable `010b8e80`, clears the global shaders (`012005d0`, `012005c8`) and
/// drops and clears the global pass (`012005e0`), releases the object at +0x9c (the pointer is
/// left) and runs `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bb0550 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the pointer and the base) is not translated.
pub fn fn_00bb0550(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_VTABLE_010B8E80);
    clear_precipitation_globals(e);
    let held = e.mem.u32(base + 0x9c);
    if held != 0 {
        release_reference(e, held);
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The render states the precipitation pass gets, in order.
const PRECIP_RENDER_STATES: [(u32, u32); 6] =
    [(7, 1), (0xe, 0), (0x17, 4), (0x1b, 1), (0x13, 5), (0x14, 6)];

// Translated from 00bb0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`; vtable `010b8e80` slot +0x148), no stack words:
/// the precipitation pass. Gives the global pass (`012005e0`) a new pass (`00e804d0`), makes
/// a texture stage (`CreateNewStage` into a stack pointer cleared first,
/// `SetupDefaultStage(stage, 0, 3, 2, 1)`, `SetFilterMode(1)`) and puts it on the pass
/// (`NiD3DPass::SetStage` with the pass's word +0x14); the pass's vertex (+0x5c) and pixel
/// (+0x44) `NiPointer`s take the first global vertex and pixel shaders; the render states
/// (7, 1), (0xe, 0), (0x17, 4), (0x1b, 1), (0x13, 5), (0x14, 6), (0xf, 0), (0xa8, 7); then
/// the stage reference is dropped. Answers true.
// Translation incomplete: 00bb0770 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage) is not translated.
pub fn fn_00bb0770(e: &mut Engine, _this: Ptr) -> bool {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.call(NEW_PASS_00E804D0, &args![PASS_012005E0]);
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let stage = e.mem.u32(local);
        e.call(SET_FILTER_MODE, &args![stage, 1u32]);
        let stage = e.mem.u32(local);
        let pass = e.global::<u32>(PASS_012005E0);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.global::<u32>(PASS_012005E0);
        let vertex = e.global::<u32>(VERTEX_SHADERS_012005D0);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.global::<u32>(PASS_012005E0);
        let pixel = e.global::<u32>(PIXEL_SHADERS_012005C8);
        ni_pointer_assign(e, pass + 0x44, pixel);
        // The sixth state reuses the pass read for the fifth.
        let mut pass = 0;
        for (index, (state, value)) in PRECIP_RENDER_STATES.iter().enumerate() {
            if index != 5 {
                pass = e.global::<u32>(PASS_012005E0);
            }
            pass_render_state(e, pass, *state, *value);
        }
        for (state, value) in [(0xfu32, 0u32), (0xa8, 7)] {
            let pass = e.global::<u32>(PASS_012005E0);
            pass_render_state(e, pass, state, value);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

// Translated from 00bb09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`; vtable `010b8e80` slot +0x4c), no stack words.
/// Runs `00be0870`, clears the global shaders (`012005d0`, `012005c8`), drops and clears the
/// global pass (`012005e0`) and answers what `00be0870` answered.
pub fn fn_00bb09b0(e: &mut Engine, this: Ptr) -> bool {
    let result = e.call(CALL_00BE0870, &args![this]).bool();
    clear_precipitation_globals(e);
    result
}

// Translated from 00bb0a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the precipitation shader's scalar deleting
/// destructor (vtable `010b8e80` slot +0), `thiscall`, `RET 4`. Runs the destructor
/// (`00bb0550`) and, when bit 0 of the flags is set, frees the 0xac bytes (`00aa1460`).
/// Answers `this`.
pub fn fn_00bb0a50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb0550(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xacu32]);
    }
    this
}

/// The constant names of the precipitation `CreateConstantMaps` (read in Ghidra) and the
/// globals they point at.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const AMBIENT_COLOR_010B8FE4: u32 = 0x010b_8fe4;
const EYE_POS_010B8FDC: u32 = 0x010b_8fdc;
const VELOCITY_0103C35C: u32 = 0x0103_c35c;
const MIN_010943DC: u32 = 0x0109_43dc;
const MAX_010943E0: u32 = 0x0109_43e0;
const PARAMS_010B8FD4: u32 = 0x010b_8fd4;
const CAM_UP_010B8FCC: u32 = 0x010b_8fcc;
const EYE_POS_012005E4: u32 = 0x0120_05e4;
const CAM_UP_012005F0: u32 = 0x0120_05f0;

/// One `0x10000006` entry: one register of 0xc bytes at `data`.
fn point_entry(name: u32, register: u32, data: u32) -> [u32; 10] {
    [
        name,
        0x1000_0006,
        0,
        register,
        1,
        CONSTANT_EMPTY_NAME_01011584,
        0xc,
        4,
        data,
        0,
    ]
}

// Translated from 00bb0a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`; vtable `010b8e80` slot +0x11c): the
/// precipitation shader's `CreateConstantMaps`, no stack words. When the pixel constant map
/// (+0x30) is null, makes one (kind 2) with `"ambient color"` (+0xa0, register 0); when the
/// vertex one (+0x34) is null, makes one (kind 0) with `"WorldViewProjTranspose"`
/// (0x20000009, 0, 0, 4), `"eye pos"` (`012005e4`, 8), `"velocity"` (+0x6c, 9), `"min"`
/// (+0x78, 0xa), `"max"` (+0x84, 0xb), `"params"` (+0x90, 0xc) and `"cam up"` (`012005f0`,
/// 0xd), each a 0x10000006 entry of 0xc bytes.
// Translation incomplete: 00bb0a80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn fn_00bb0a80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
        add_constant_entry(
            e,
            base,
            0x30,
            point_entry(AMBIENT_COLOR_010B8FE4, 0, base + 0xa0),
        );
    }
    if e.mem.u32(base + 0x34) == 0 {
        make_constant_map(e, base, 0x34, 0);
        let entries = [
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
            point_entry(EYE_POS_010B8FDC, 8, EYE_POS_012005E4),
            point_entry(VELOCITY_0103C35C, 9, base + 0x6c),
            point_entry(MIN_010943DC, 0xa, base + 0x78),
            point_entry(MAX_010943E0, 0xb, base + 0x84),
            point_entry(PARAMS_010B8FD4, 0xc, base + 0x90),
            point_entry(CAM_UP_010B8FCC, 0xd, CAM_UP_012005F0),
        ];
        for words in entries {
            add_constant_entry(e, base, 0x34, words);
        }
    }
}

/// The four globals `00bb1390` writes.
const STORED_01200658: u32 = 0x0120_0658;
const STORED_01200668: u32 = 0x0120_0668;
const SUM_01200608: u32 = 0x0120_0608;
const SUM_0120060C: u32 = 0x0120_060c;

// Translated from 00bb1390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`), `cdecl`, four `float`s: the first two stored at
/// `01200658` and `01200668` (through `FLD`/`FSTP`), the last two added to the floats at
/// `01200608` and `0120060c`.
pub fn fn_00bb1390(e: &mut Engine, first: f32, second: f32, third: f32, fourth: f32) {
    e.set_global::<u32>(STORED_01200658, x87_quiet_bits(first.to_bits()));
    e.set_global::<u32>(STORED_01200668, x87_quiet_bits(second.to_bits()));
    let sum = x87_add(
        x87_load32(e.global::<u32>(SUM_01200608)),
        x87_m32(third.to_bits()),
    );
    e.set_global::<u32>(SUM_01200608, x87_store32(sum));
    let sum = x87_add(
        x87_load32(e.global::<u32>(SUM_0120060C)),
        x87_m32(fourth.to_bits()),
    );
    e.set_global::<u32>(SUM_0120060C, x87_store32(sum));
}

/// D3DX (engine map): `D3DXVec3TransformNormal` (`00ee6df2`), `D3DXVec3Normalize`
/// (`00ee6dec`), `D3DXVec3TransformCoord` (`00ee6df8`) (`stdcall`); and
/// `BSShaderLightingProperty::GetNumberofActiveLights` (engine map, `00b70790`, `thiscall`, a
/// 16-bit answer).
const D3DX_VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const D3DX_VEC3_NORMALIZE: u32 = 0x00ee_6dec;
const D3DX_VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;
const ACTIVE_LIGHTS: u32 = 0x00b7_0790;
/// The light direction (`011fa1f0`), position (`011fa180`) and range (`011fa200`) read, and
/// the constants written (`01200618`, `01200628`, `01200634`).
const LIGHT_DIRECTION_011FA1F0: u32 = 0x011f_a1f0;
const LIGHT_POSITION_011FA180: u32 = 0x011f_a180;
const LIGHT_RANGE_011FA200: u32 = 0x011f_a200;
const DIRECTION_01200618: u32 = 0x0120_0618;
const POSITION_01200628: u32 = 0x0120_0628;
const RANGE_01200634: u32 = 0x0120_0634;

// Translated from 00bb1960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`; the leaf shader's `SetupGeometry` `00bb25d0`
/// calls it), `thiscall`, `RET 0xc`: a lighting property, a matrix and a scale (`this` is not
/// read). The negated light direction (`011fa1f0`, three floats) transformed by the matrix
/// (`D3DXVec3TransformNormal`) and normalized (`D3DXVec3Normalize`) goes to `01200618` with
/// 0.0; when the property has active lights (`GetNumberofActiveLights`, 16 bits), the light
/// position (`011fa180`) transformed (`D3DXVec3TransformCoord`) goes to `01200628` and the
/// range (`011fa200`) over the scale to `01200634`.
pub fn fn_00bb1960(e: &mut Engine, _this: Ptr, property: Ptr, matrix: Ptr, scale: f32) {
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        let vector = frame + 4;
        let transformed = frame + 0x2c;
        for word in 0..3u32 {
            let value = x87_quiet_bits(e.global::<u32>(LIGHT_DIRECTION_011FA1F0 + 4 * word));
            e.mem.set_u32(vector + 4 * word, value ^ 0x8000_0000);
        }
        e.call(
            D3DX_VEC3_TRANSFORM_NORMAL,
            &args![transformed, vector, matrix],
        );
        e.call(D3DX_VEC3_NORMALIZE, &args![vector, transformed]);
        for word in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(vector + 4 * word));
            e.set_global::<u32>(DIRECTION_01200618 + 4 * word, value);
        }
        e.set_global::<u32>(DIRECTION_01200618 + 0xc, 0);
        if e.call(ACTIVE_LIGHTS, &args![property]).u16() == 0 {
            return;
        }
        let position = frame + 0x10;
        let moved = frame + 0x1c;
        for word in 0..3u32 {
            let value = x87_quiet_bits(e.global::<u32>(LIGHT_POSITION_011FA180 + 4 * word));
            e.mem.set_u32(position + 4 * word, value);
        }
        e.call(D3DX_VEC3_TRANSFORM_COORD, &args![moved, position, matrix]);
        for word in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(moved + 4 * word));
            e.set_global::<u32>(POSITION_01200628 + 4 * word, value);
        }
        let range = x87_load32(e.global::<u32>(LIGHT_RANGE_011FA200));
        let range = x87_store32(x87_div(range, x87_m32(scale.to_bits())));
        e.set_global::<u32>(RANGE_01200634, range);
    });
}

/// The object whose rows `00bb1a90` copies (`011f917c`) and the two blocks it fills
/// (`01200648`, `01200638`).
const CAMERA_011F917C: u32 = 0x011f_917c;
const ROW_01200648: u32 = 0x0120_0648;
const ROW_01200638: u32 = 0x0120_0638;

// Translated from 00bb1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`), no arguments. Copies (through `FLD`/`FSTP`)
/// the floats +0x6c, +0x78, +0x84 of the object at `011f917c` to `01200648` with 0.0 and
/// +0x70, +0x7c, +0x88 to `01200638` with 0.0.
pub fn fn_00bb1a90(e: &mut Engine) {
    let object = e.global::<u32>(CAMERA_011F917C);
    let first = [0x6cu32, 0x78, 0x84].map(|offset| x87_quiet_bits(e.mem.u32(object + offset)));
    let second = [0x70u32, 0x7c, 0x88].map(|offset| x87_quiet_bits(e.mem.u32(object + offset)));
    for (word, value) in first.iter().enumerate() {
        e.set_global::<u32>(ROW_01200648 + 4 * word as u32, *value);
    }
    e.set_global::<u32>(ROW_01200648 + 0xc, 0);
    for (word, value) in second.iter().enumerate() {
        e.set_global::<u32>(ROW_01200638 + 4 * word as u32, *value);
    }
    e.set_global::<u32>(ROW_01200638 + 0xc, 0);
}

/// The shader manager's pass state word (`0126f74c`).
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;

// Translated from 00bb1b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b9190` slot +0xf0), `thiscall`, `RET 4` (a pass
/// number). For pass 0x3e the pass state word `0126f74c` becomes the shader's +0x384, for
/// 0x3f its +0x388; other passes leave it.
pub fn fn_00bb1b20(e: &mut Engine, this: Ptr, pass: u32) {
    let offset = match pass {
        0x3e => 0x384,
        0x3f => 0x388,
        _ => return,
    };
    let state = e.mem.u32(this.addr() + offset);
    e.set_global::<u32>(PASS_STATE_0126F74C, state);
}

/// `BSShaderManager::GetFogProperty` (engine map, `00b55520`, `cdecl`, a byte), the byte it
/// is given (`011f91c4`) and the fog constants written (`011fa280`, `011fa290`).
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
const FOG_KIND_011F91C4: u32 = 0x011f_91c4;
const FOG_PARAM_011FA280: u32 = 0x011f_a280;
const FOG_COLOR_011FA290: u32 = 0x011f_a290;

// Translated from 00bb1b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010b9190` slot +0xfc), `thiscall`, `RET 4` (neither `this`
/// nor the word is read). With the fog property for the byte `011f91c4`
/// (`BSShaderManager::GetFogProperty`), when there is one: the fog parameters `011fa280` get
/// its +0x30, +0x30 minus +0x2c, +0x60 and 0.0, the fog colour `011fa290` its +0x20, +0x24,
/// +0x28 and 0.0 (each float through `FLD`/`FSTP`).
pub fn fn_00bb1b50(e: &mut Engine, _this: Ptr, _unused_1: u32) {
    let kind = u32::from(e.global::<u8>(FOG_KIND_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![kind]).u32();
    if fog == 0 {
        return;
    }
    let far = x87_quiet_bits(e.mem.u32(fog + 0x30));
    let color = [0x20u32, 0x24, 0x28].map(|offset| x87_quiet_bits(e.mem.u32(fog + offset)));
    let range = x87_store32(x87_sub(x87_load32(far), x87_m32(e.mem.u32(fog + 0x2c))));
    let power = x87_quiet_bits(e.mem.u32(fog + 0x60));
    for (word, value) in [far, range, power, 0].iter().enumerate() {
        e.set_global::<u32>(FOG_PARAM_011FA280 + 4 * word as u32, *value);
    }
    for (word, value) in [color[0], color[1], color[2], 0].iter().enumerate() {
        e.set_global::<u32>(FOG_COLOR_011FA290 + 4 * word as u32, *value);
    }
}

/// Strings of `00bb13d0` (read in Ghidra): `speedtree\\leaf.v.hlsl`, `speedtree\\leaf.p.hlsl`,
/// the macros `PT`, `FOG` and `TRANSPARENCYMS`, the empty string and the name formats
/// `"STLEAF%03i.vso"`, `"STLEAF%03i.pso"`, `"STLEAF2%03i.pso"` and `"STLEAF2%03iTMS.pso"`.
const LEAF_VERTEX_010B9178: u32 = 0x010b_9178;
const LEAF_PIXEL_010B915C: u32 = 0x010b_915c;
const LEAF_MACRO_PT_010B9174: u32 = 0x010b_9174;
const LEAF_MACRO_FOG_010AECE0: u32 = 0x010a_ece0;
const LEAF_MACRO_TRANSPARENCYMS_010B8AFC: u32 = 0x010b_8afc;
const LEAF_EMPTY_01011584: u32 = 0x0101_1584;
const FORMAT_LEAF_VERTEX_010B914C: u32 = 0x010b_914c;
const FORMAT_LEAF_PIXEL_010B9118: u32 = 0x010b_9118;
const FORMAT_LEAF2_PIXEL_010B9128: u32 = 0x010b_9128;
const FORMAT_LEAF2_TMS_PIXEL_010B9138: u32 = 0x010b_9138;
/// The vertex profile `vs_2_0`, the shader model setting (`011f91c0`), the byte `011f948b`
/// and the multisampling byte `011f9421`.
const LEAF_PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
const LEAF_SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const SWITCH_011F948B: u32 = 0x011f_948b;
const TRANSPARENCY_MULTISAMPLING_011F9421: u32 = 0x011f_9421;
/// The `00bb13d0` frame (saved registers up to the cookie) and its name and path buffers.
const LEAF_FRAME_BYTES: u32 = 0x3f8;
const LEAF_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x1ec,
    path: 0x2f0,
};
/// The `00bb13d0` source blocks (0x4c bytes apart: two pixel blocks from +0x24, four vertex
/// blocks from +0xbc), in the exe's order.
const LEAF_SOURCES: [ShaderSource; 6] = [
    ShaderSource {
        offset: 0xbc,
        words: &[LEAF_VERTEX_010B9178, 0, 0],
        cleared: 0x40,
    },
    ShaderSource {
        offset: 0x108,
        words: &[
            LEAF_VERTEX_010B9178,
            LEAF_MACRO_FOG_010AECE0,
            LEAF_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
    ShaderSource {
        offset: 0x154,
        words: &[
            LEAF_VERTEX_010B9178,
            LEAF_MACRO_PT_010B9174,
            LEAF_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
    ShaderSource {
        offset: 0x1a0,
        words: &[
            LEAF_VERTEX_010B9178,
            LEAF_MACRO_PT_010B9174,
            LEAF_EMPTY_01011584,
            LEAF_MACRO_FOG_010AECE0,
            LEAF_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x30,
    },
    ShaderSource {
        offset: 0x24,
        words: &[LEAF_PIXEL_010B915C, 0, 0],
        cleared: 0x40,
    },
    ShaderSource {
        offset: 0x70,
        words: &[
            LEAF_PIXEL_010B915C,
            LEAF_MACRO_FOG_010AECE0,
            LEAF_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
];

/// One shader model 2 leaf pixel shader: with `multisampling` the macro pair
/// (`TRANSPARENCYMS`, "") is written over the first pair of the block whose name is null and
/// the name format is `"STLEAF2%03iTMS.pso"`, otherwise `"STLEAF2%03i.pso"`; the name
/// formatted with the index, `MakePath` of the block's file, the pixel compile (`this`, path,
/// the block's macros, the profile, the name) into the `NiPointer` at `slot`.
#[allow(clippy::too_many_arguments)]
fn leaf_pixel_shader(
    e: &mut Engine,
    this: u32,
    frame: u32,
    block: u32,
    slot: u32,
    index: u32,
    profile: u32,
    multisampling: bool,
) {
    let block = frame + block;
    let format = if multisampling {
        let mut pair = block + 4;
        while e.mem.u32(pair) != 0 {
            pair += 8;
        }
        e.mem.set_u32(pair, LEAF_MACRO_TRANSPARENCYMS_010B8AFC);
        e.mem.set_u32(pair + 4, LEAF_EMPTY_01011584);
        FORMAT_LEAF2_TMS_PIXEL_010B9138
    } else {
        FORMAT_LEAF2_PIXEL_010B9128
    };
    let name = frame + LEAF_BUFFERS.name;
    let path = frame + LEAF_BUFFERS.path;
    e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
    let file = e.mem.u32(block);
    e.call(MAKE_PATH, &args![file, path]);
    let shader = e
        .call(
            COMPILE_PIXEL_SHADER,
            &args![this, path, block + 4, profile, name],
        )
        .u32();
    ni_pointer_assign(e, slot, shader);
}

// Translated from 00bb13d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function: the leaf shader's `LoadShaders`, no stack words. Fills six source
/// blocks on its stack (`leaf.v.hlsl` with none, `FOG`, `PT`, `PT` and `FOG`; `leaf.p.hlsl`
/// with none and `FOG`; the rest of each cleared by `memset`) and compiles the four vertex
/// blocks (`"STLEAF%03i.vso"` with the index, `vs_2_0`) into the `NiPointer`s +0x36c to
/// +0x378, a block without a file clearing its pointer instead. Takes the pixel profile from
/// `00b4f380(1)`. Below shader model (`011f91c0`) 2 the two pixel blocks are compiled the same
/// way (`"STLEAF%03i.pso"`) into +0x37c and +0x380. At model 2 or more, for each pixel block
/// with a file (a block without one clears its pointer): when the byte `011f948b` is set, two
/// compiles into the same pointer, each first flipping the byte `011f9421` (the first time
/// from a forced 1, so it is 0, then 1) and appending `TRANSPARENCYMS` when the flipped byte
/// is set; when it is clear, one compile, with `TRANSPARENCYMS` when `011f9421` is set.
// Translation incomplete: 00bb13d0 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bb13d0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(LEAF_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &LEAF_SOURCES);
        for index in 0..4u32 {
            compile_indexed_or_clear(
                e,
                this,
                frame,
                &LEAF_BUFFERS,
                0xbc + 0x4c * index,
                this + 0x36c + 4 * index,
                FORMAT_LEAF_VERTEX_010B914C,
                index,
                LEAF_PROFILE_VS_2_0_010A93E4,
                false,
            );
        }
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        if e.global::<i32>(LEAF_SHADER_MODEL_011F91C0) < 2 {
            for index in 0..2u32 {
                compile_indexed_or_clear(
                    e,
                    this,
                    frame,
                    &LEAF_BUFFERS,
                    0x24 + 0x4c * index,
                    this + 0x37c + 4 * index,
                    FORMAT_LEAF_PIXEL_010B9118,
                    index,
                    profile,
                    true,
                );
            }
            return;
        }
        for index in 0..2u32 {
            let block = 0x24 + 0x4c * index;
            let slot = this + 0x37c + 4 * index;
            if e.mem.u32(frame + block) == 0 {
                ni_pointer_clear(e, slot);
                continue;
            }
            if e.global::<u8>(SWITCH_011F948B) != 0 {
                let mut previous = 1u8;
                for _ in 0..2 {
                    let flipped = u8::from(previous == 0);
                    e.set_global(TRANSPARENCY_MULTISAMPLING_011F9421, flipped);
                    leaf_pixel_shader(e, this, frame, block, slot, index, profile, flipped != 0);
                    previous = e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421);
                }
            } else {
                let multisampling = e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421) != 0;
                leaf_pixel_shader(e, this, frame, block, slot, index, profile, multisampling);
            }
        }
    });
}

// Translated from 00bb0490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at
/// `010b8fa4`), `thiscall`, no stack words: calls the shader's own vtable +0x128 with its pass
/// (`012005e0`).
pub fn fn_00bb0490(e: &mut Engine, this: Ptr) {
    let pass = e.global::<u32>(PASS_012005E0);
    e.vcall(this.addr(), 0x128, &args![pass]);
}

/// The record `00bb0540` answers (`012005c0`).
const RECORD_012005C0: u32 = 0x0120_05c0;

// Translated from 00bb0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b8e88`, `ECX` unread):
/// answers the address `012005c0`.
pub fn fn_00bb0540(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RECORD_012005C0)
}

// Translated from 00bb0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b90d0`), `thiscall`
/// with one word: +0x94 becomes 0 when the word is 3, else 3 (`SUB 3`, `NEG`, `SBB`, `AND 3`).
pub fn fn_00bb0cf0(e: &mut Engine, this: Ptr, value: u32) {
    let result = if value.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.mem.set_u32(this.addr() + 0x94, result);
}

/// The record `00bb0d10` answers (`012005fc`, the property's RTTI record).
const RTTI_012005FC: u32 = 0x0120_05fc;

// Translated from 00bb0d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b9010`, `ECX` unread):
/// answers the address `012005fc`.
pub fn fn_00bb0d10(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RTTI_012005FC)
}

// Translated from 00bb0d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b90d4`), `thiscall`:
/// answers the word +0x94.
pub fn fn_00bb0d20(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x94)
}

// Translated from 00bb0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b90d8`), `thiscall`
/// with one word: stores it at +0x94.
pub fn fn_00bb0d30(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x94, value);
}

/// `BSShaderProperty::BSShaderProperty` (`00ba8e50`), its destructor (`00baa0e0`, unnamed,
/// `thiscall`) and its copy step `00ba8b10` (`thiscall`, two words), and this property's vtable
/// (`010b9008`).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const SHADER_PROPERTY_DESTRUCT_00BAA0E0: u32 = 0x00ba_a0e0;
const SHADER_PROPERTY_COPY_00BA8B10: u32 = 0x00ba_8b10;
const PROPERTY_VTABLE_010B9008: u32 = 0x010b_9008;

/// Three words copied from `source` to `target`.
fn copy_three(e: &mut Engine, target: u32, source: u32) {
    for index in 0..3 {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(target + 4 * index, word);
    }
}

// Translated from 00bb0d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the property constructor (vtable `010b9008`), `thiscall` with
/// seven words. Runs `BSShaderProperty::BSShaderProperty`, stores the float (through
/// `FLD`/`FSTP`) at +0x68 and the first word at +0x60, copies three words from each of the
/// pointers: the fourth word's to +0x6c, the fifth's to +0x78, the third's to +0x84; clears
/// +0x90 and +0x94, stores the sixth word at +0x9c and whether the seventh is non-zero at
/// +0x98. Answers `this`.
pub fn fn_00bb0d40(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    value: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
) -> Ptr {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![base]);
    e.mem.set_u32(base + 0x68, x87_quiet_bits(value));
    e.mem.set_u32(base + 0x60, first);
    e.mem.set_u32(base, PROPERTY_VTABLE_010B9008);
    copy_three(e, base + 0x6c, fourth);
    copy_three(e, base + 0x78, fifth);
    copy_three(e, base + 0x84, third);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0);
    e.mem.set_u32(base + 0x9c, sixth);
    e.mem.set_u32(base + 0x98, u32::from(seventh != 0));
    this
}

// Translated from 00bb0de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb0de0 - the C++ exception frame (FS:[0] chain, unwind states 1 to 0) is not translated.
/// Unnamed in the engine map: the property destructor, `thiscall`. Stores the vtable
/// `010b9008`, clears +0x60, releases the reference at +0x90 and clears it, then releases
/// +0x90 again when it is still set (the member's own destructor), and runs the
/// `BSShaderProperty` destructor (`00baa0e0`).
pub fn fn_00bb0de0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PROPERTY_VTABLE_010B9008);
    e.mem.set_u32(base + 0x60, 0);
    let held = e.mem.u32(base + 0x90);
    if held != 0 {
        release_reference(e, held);
        e.mem.set_u32(base + 0x90, 0);
    }
    let held = e.mem.u32(base + 0x90);
    if held != 0 {
        release_reference(e, held);
    }
    e.call(SHADER_PROPERTY_DESTRUCT_00BAA0E0, &args![base]);
}

// Translated from 00bb0e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010adf7c` and
/// `010b90bc`), `thiscall`: answers the word +0x90.
pub fn fn_00bb0e90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x90)
}

// Translated from 00bb0ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the property's scalar deleting destructor (vtable `010b9008`
/// slot +0), `thiscall`, `RET 4`. Runs `00bb0de0` and, with bit 0 of the flags, frees the 0xa0
/// bytes (`00aa1460`). Answers `this`.
pub fn fn_00bb0ea0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb0de0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xa0u32]);
    }
    this
}

// Translated from 00bb0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the property's copy into `target`, `thiscall` with two words
/// (the target, a word passed on). Runs `00ba8b10(target, word)` on `this`, copies +0x60 and
/// +0x6c..+0x8c, then the reference at +0x90 (when it differs: the target's old one released,
/// the source's taken with a count when not null) and the word +0x94.
pub fn fn_00bb0ed0(e: &mut Engine, this: Ptr, target: u32, word: u32) {
    let base = this.addr();
    e.call(SHADER_PROPERTY_COPY_00BA8B10, &args![base, target, word]);
    let value = e.mem.u32(base + 0x60);
    e.mem.set_u32(target + 0x60, value);
    for offset in (0x6cu32..=0x8c).step_by(4) {
        let value = e.mem.u32(base + offset);
        e.mem.set_u32(target + offset, value);
    }
    let old = e.mem.u32(target + 0x90);
    if old != e.mem.u32(base + 0x90) {
        if old != 0 {
            release_reference(e, old);
        }
        let new = e.mem.u32(base + 0x90);
        e.mem.set_u32(target + 0x90, new);
        if new != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new + 4]);
        }
    }
    let value = e.mem.u32(base + 0x94);
    e.mem.set_u32(target + 0x94, value);
}

/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, one word) and
/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, eight words).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;

// Translated from 00bb0fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb0fc0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// Unnamed in the engine map (vtable entry at `010b90a8`), `thiscall` with six words (a
/// geometry, a key, an unread word, a high key, two unread words). With the thread's allocation
/// context (+0x2b4) at 0x29: when the key differs from the word +0x38, makes the render-pass
/// array (`CheckCreateRenderPassArray(1)`), empties it (+0x10 of +0x3c), adds pass 0x2cf for
/// the geometry (`Add(geometry, 0x2cf, 1, 0, 0, 0, 0, 0)`) and stores `high << 8 | key` at
/// +0x38. Answers the array (+0x3c).
pub fn fn_00bb0fc0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    key: u32,
    _unused_2: u32,
    high: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let base = this.addr();
    with_allocation_context(e, 0x29, |e| {
        if e.mem.u32(base + 0x38) != key {
            e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![base, 1u32]);
            let array = e.mem.u32(base + 0x3c);
            e.mem.set_u32(array + 0x10, 0);
            let array = e.mem.u32(base + 0x3c);
            e.call(
                RENDER_PASS_ARRAY_ADD,
                &args![array, geometry, 0x2cfu32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            );
            e.mem.set_u32(base + 0x38, (high << 8) | key);
        }
        e.mem.u32(base + 0x3c)
    })
}

// Translated from 00bb1080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb1080 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
/// Unnamed in the engine map (vtable entry at `010b9050`), `thiscall` with one word: the
/// property's clone. With the thread's allocation context at 0x29, allocates 0xa0 bytes
/// (`00aa13e0`); when it succeeds, runs `BSShaderProperty::BSShaderProperty` on them, stores 0.0
/// at +0x68, the vtable `010b9008`, clears +0x60, +0x90, +0x9c, sets +0x98 to 1 and +0x1c to
/// 0x12. Then `00bb0ed0` copies `this` into it (with the word; a null block is passed as it
/// is). Answers the new property.
pub fn fn_00bb1080(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    with_allocation_context(e, 0x29, |e| {
        let block = e.call(NI_ALLOCATE, &args![0xa0u32]).u32();
        if block != 0 {
            e.call(SHADER_PROPERTY_CONSTRUCT, &args![block]);
            e.mem.set_f32(block + 0x68, 0.0);
            e.mem.set_u32(block, PROPERTY_VTABLE_010B9008);
            e.mem.set_u32(block + 0x60, 0);
            e.mem.set_u32(block + 0x90, 0);
            e.mem.set_u32(block + 0x98, 1);
            e.mem.set_u32(block + 0x9c, 0);
            e.mem.set_u32(block + 0x1c, 0x12);
        }
        fn_00bb0ed0(e, this, block, word);
        block
    })
}

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_FLAG, VIEWER_STRING_INT, VIEWER_STRING_TEXT,
};

/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), the viewer-string
/// makers `00c4bb40` (`cdecl`, one word) and `00a7d5a0` and `00a7db60` (`cdecl`, two words),
/// and the labels: `010b90ec`, `010ae79c`, `010ae344`, `010b90dc`.
const SHADER_PROPERTY_VIEWER_STRINGS: u32 = 0x00ba_9890;
const TEXTURE_NAME_00C4BB40: u32 = 0x00c4_bb40;
const VIEWER_STRING_POINTER_00A7D5A0: u32 = 0x00a7_d5a0;
const VIEWER_STRING_00A7DB60: u32 = 0x00a7_db60;
const LABEL_010B90EC: u32 = 0x010b_90ec;
const LABEL_010AE79C: u32 = 0x010a_e79c;
const LABEL_010AE344: u32 = 0x010a_e344;
const LABEL_010B90DC: u32 = 0x010b_90dc;

// Translated from 00bb1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b9068`): the
/// property's `GetViewerStrings`, `thiscall` with one word, the string array. Runs
/// `BSShaderProperty::GetViewerStrings`, then adds (the inlined `NiTArray` add): the class
/// name (`00a7d330` of `012005fc`), `00a7d4d0(010b90ec, +0x60)`; with a reference at +0x90,
/// `00a7d5e0(010ae79c, name)` with the name `00c4bb40` gives it, or `00a7d5a0(010ae79c,
/// reference)` without; when the vtable +0xcc answers 0, `00a7d370(010ae344, 1)`; and
/// `00a7db60(010b90dc, +0x98)`.
pub fn fn_00bb1160(e: &mut Engine, this: Ptr, strings: u32) {
    let base = this.addr();
    e.call(SHADER_PROPERTY_VIEWER_STRINGS, &args![base, strings]);
    let rtti = e.global::<u32>(RTTI_012005FC);
    let string = e.call(VIEWER_STRING, &args![rtti]).u32();
    viewer_strings_add(e, strings, string);
    let value = e.mem.u32(base + 0x60);
    let string = e
        .call(VIEWER_STRING_INT, &args![LABEL_010B90EC, value])
        .u32();
    viewer_strings_add(e, strings, string);
    let reference = e.mem.u32(base + 0x90);
    if reference != 0 {
        let name = e.call(TEXTURE_NAME_00C4BB40, &args![reference]).u32();
        let string = if name != 0 {
            e.call(VIEWER_STRING_TEXT, &args![LABEL_010AE79C, name])
                .u32()
        } else {
            let reference = e.mem.u32(base + 0x90);
            e.call(
                VIEWER_STRING_POINTER_00A7D5A0,
                &args![LABEL_010AE79C, reference],
            )
            .u32()
        };
        viewer_strings_add(e, strings, string);
    }
    if e.vcall(base, 0xcc, &args![]).u32() == 0 {
        let string = e
            .call(VIEWER_STRING_FLAG, &args![LABEL_010AE344, 1u32])
            .u32();
        viewer_strings_add(e, strings, string);
    }
    let value = e.mem.u32(base + 0x98);
    let string = e
        .call(VIEWER_STRING_00A7DB60, &args![LABEL_010B90DC, value])
        .u32();
    viewer_strings_add(e, strings, string);
}

/// `NiAVObject::UpdateProperties` (engine map, `00a5a040`, `thiscall`) and `00654640`
/// (unnamed, `thiscall` with one word).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const GEOMETRY_CALL_00654640: u32 = 0x0065_4640;

// Translated from 00bb12e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b90a4`), `thiscall`
/// with one word, a geometry. The block is the geometry's +0x9c (the exe calls
/// `NiAVObject::UpdateProperties` on the geometry when that address is null, then reads +0x14
/// of it as it is); its +0x14 object (null: answers false). The property's reference at +0x90
/// takes the texture (+8) of the first entry of that object's +0x20 array (null when there is
/// none); the vtable +0xd0 gets bits 12..13 of that entry's `u16` +4 (3 without an entry);
/// `00654640(geometry, object)`. Answers true.
pub fn fn_00bb12e0(e: &mut Engine, this: Ptr, geometry: u32) -> bool {
    let base = this.addr();
    let block = geometry.wrapping_add(0x9c);
    if block == 0 {
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![geometry]);
    }
    let object = e.mem.u32(block.wrapping_add(0x14));
    if object == 0 {
        return false;
    }
    let entries = e.mem.u32(object + 0x20);
    let entry = e.mem.u32(entries);
    let texture = if entry != 0 { e.mem.u32(entry + 8) } else { 0 };
    ni_pointer_assign(e, base + 0x90, texture);
    let entries = e.mem.u32(object + 0x20);
    let entry = e.mem.u32(entries);
    let mode = if entry != 0 {
        (u32::from(e.mem.u16(entry + 4)) >> 12) & 3
    } else {
        3
    };
    e.vcall(base, 0xd0, &args![mode]);
    e.call(GEOMETRY_CALL_00654640, &args![geometry, object]);
    true
}

// Translated from 00bb1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b92b4`), `thiscall`,
/// no stack words: calls the shader's own vtable +0x128 with its word +0x384.
pub fn fn_00bb1a70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pass = e.mem.u32(base + 0x384);
    e.vcall(base, 0x128, &args![pass]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_0490, fn_00bb0490(Ptr)),
        entry!(0x00bb_0540, fn_00bb0540(Ptr) -> Ptr),
        entry!(0x00bb_0cf0, fn_00bb0cf0(Ptr, u32)),
        entry!(0x00bb_0d10, fn_00bb0d10(Ptr) -> Ptr),
        entry!(0x00bb_0d20, fn_00bb0d20(Ptr) -> u32),
        entry!(0x00bb_0d30, fn_00bb0d30(Ptr, u32)),
        entry!(
            0x00bb_0d40,
            fn_00bb0d40(Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00bb_0de0, fn_00bb0de0(Ptr)),
        entry!(0x00bb_0e90, fn_00bb0e90(Ptr) -> u32),
        entry!(0x00bb_0ea0, fn_00bb0ea0(Ptr, u32) -> Ptr),
        entry!(0x00bb_0ed0, fn_00bb0ed0(Ptr, u32, u32)),
        entry!(
            0x00bb_0fc0,
            fn_00bb0fc0(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_1080, fn_00bb1080(Ptr, u32) -> u32),
        entry!(0x00bb_1160, fn_00bb1160(Ptr, u32)),
        entry!(0x00bb_12e0, fn_00bb12e0(Ptr, u32) -> bool),
        entry!(0x00bb_1a70, fn_00bb1a70(Ptr)),
        entry!(0x00bb_04b0, fn_00bb04b0(Ptr) -> Ptr),
        entry!(0x00bb_0660, fn_00bb0660() -> Ptr),
        entry!(0x00bb_01c0, fn_00bb01c0(Ptr)),
        entry!(0x00bb_0360, fn_00bb0360(Ptr)),
        entry!(0x00bb_0550, fn_00bb0550(Ptr)),
        entry!(0x00bb_0770, fn_00bb0770(Ptr) -> bool),
        entry!(0x00bb_09b0, fn_00bb09b0(Ptr) -> bool),
        entry!(0x00bb_0a50, fn_00bb0a50(Ptr, u32) -> Ptr),
        entry!(0x00bb_0a80, fn_00bb0a80(Ptr)),
        entry!(0x00bb_1390, fn_00bb1390(f32, f32, f32, f32)),
        entry!(0x00bb_13d0, fn_00bb13d0(Ptr)),
        entry!(0x00bb_1960, fn_00bb1960(Ptr, Ptr, Ptr, f32)),
        entry!(0x00bb_1a90, fn_00bb1a90()),
        entry!(0x00bb_1b20, fn_00bb1b20(Ptr, u32)),
        entry!(0x00bb_1b50, fn_00bb1b50(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bb0660_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BB04B0, &[]);
        let (shader, log) = run(&mut e, 0x00bb_0660);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 2]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 1, 0, 6, 0x8000_0000, 2, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xac]),
                (SHADER_CONSTRUCT_00BB04B0, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn fn_00bb04b0_sets_the_fields() {
        let mut e = Engine::new();
        e.register(BS_SHADER_CONSTRUCT, |_, a| a[0].into_ret());
        e.map(0x011a_9000, 0x1000);
        for index in 0..3u32 {
            e.set_global::<u32>(WORDS_011A9B7C + 4 * index, 0x100 + index);
        }
        let this = e.mem.alloc(0xac);
        e.mem.write(this, &[0xcc; 0xac]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_04b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010B8E80);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x68), 0x12);
        for offset in [0x90u32, 0x94, 0x98, 0x9c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0xa0), 0x100);
        assert_eq!(e.mem.u32(this + 0xa4), 0x101);
        assert_eq!(e.mem.u32(this + 0xa8), 0x102);
        let log = e.call_log.take().unwrap();
        let constructed: Vec<_> = log.iter().filter(|c| c.0 == BS_SHADER_CONSTRUCT).collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![this]);
    }
}

#[cfg(test)]
mod precipitation_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load_leaf(model: i32, switch: u8, multisampling: u8) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(LEAF_SHADER_MODEL_011F91C0, model);
        e.set_global(SWITCH_011F948B, switch);
        e.set_global(TRANSPARENCY_MULTISAMPLING_011F9421, multisampling);
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e55_0000u32.into_ret());
        let this = e.mem.alloc(0x390);
        e.call(0x00bb_13d0, &args![this]);
        (e, this)
    }

    #[test]
    fn fn_00bb13d0_compiles_twice_with_the_switch() {
        let (e, this) = load_leaf(2, 1, 0);
        let vertex = e.mem.u32(this + 0x36c + 4 * 3);
        assert_eq!(
            compiled(&e, vertex),
            Compiled {
                file: LEAF_VERTEX_010B9178,
                output: FORMAT_LEAF_VERTEX_010B914C,
                profile: LEAF_PROFILE_VS_2_0_010A93E4,
                pixel: false,
                macros: macro_words(&[
                    (LEAF_MACRO_PT_010B9174, LEAF_EMPTY_01011584),
                    (LEAF_MACRO_FOG_010AECE0, LEAF_EMPTY_01011584)
                ]),
            }
        );
        let outputs: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[2]).collect();
        assert_eq!(
            outputs[4..],
            [
                FORMAT_LEAF2_PIXEL_010B9128,
                FORMAT_LEAF2_TMS_PIXEL_010B9138,
                FORMAT_LEAF2_PIXEL_010B9128,
                FORMAT_LEAF2_TMS_PIXEL_010B9138
            ]
        );
        let pixel = e.mem.u32(this + 0x380);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: LEAF_PIXEL_010B915C,
                output: FORMAT_LEAF2_TMS_PIXEL_010B9138,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[
                    (LEAF_MACRO_FOG_010AECE0, LEAF_EMPTY_01011584),
                    (LEAF_MACRO_TRANSPARENCYMS_010B8AFC, LEAF_EMPTY_01011584)
                ]),
            }
        );
        assert_eq!(e.global::<u8>(TRANSPARENCY_MULTISAMPLING_011F9421), 1);
    }

    #[test]
    fn fn_00bb13d0_compiles_once_without_the_switch() {
        let (e, this) = load_leaf(2, 0, 1);
        assert_eq!(calls(&e, FORMAT_00406D00).len(), 6);
        let pixel = e.mem.u32(this + 0x37c);
        assert_eq!(
            compiled(&e, pixel).macros,
            macro_words(&[(LEAF_MACRO_TRANSPARENCYMS_010B8AFC, LEAF_EMPTY_01011584)])
        );
        let (e, this) = load_leaf(2, 0, 0);
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x37c)).output,
            FORMAT_LEAF2_PIXEL_010B9128
        );
    }

    #[test]
    fn fn_00bb13d0_uses_the_1x_names_below_model_2() {
        let (e, this) = load_leaf(1, 1, 0);
        assert_eq!(calls(&e, FORMAT_00406D00).len(), 6);
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x380)).output,
            FORMAT_LEAF_PIXEL_010B9118
        );
        assert_eq!(calls(&e, MEMSET).len(), 6);
    }

    fn map_globals(e: &mut Engine) {
        for page in [0x011f_9000u32, 0x011f_a000, 0x0120_0000, 0x0126_f000] {
            e.map(page, 0x1000);
        }
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00bb01c0_compiles_the_four_vertex_shaders_into_the_globals() {
        let mut e = loader_rig();
        map_globals(&mut e);
        e.register(VERTEX_PROFILE_00B4F3E0, |_, _| 0x0e56_0000u32.into_ret());
        let this = e.mem.alloc(0xac);
        e.call(0x00bb_01c0, &args![this]);
        let shaders: Vec<u32> = (0..4)
            .map(|i| e.global::<u32>(VERTEX_SHADERS_012005D0 + 4 * i))
            .collect();
        assert_eq!(
            compiled(&e, shaders[0]),
            Compiled {
                file: PRECIPITATION_VERTEX_010B8E28,
                output: FORMAT_PRECIP_VERTEX_010B8DF0,
                profile: 0x0e56_0000,
                pixel: false,
                macros: macro_words(&[(MACRO_BILLBOARD_UP_010B8E18, 0)]),
            }
        );
        assert_eq!(
            compiled(&e, shaders[3]).macros,
            macro_words(&[(MACRO_BILLBOARD_FACE_010B8E08, 0), (MACRO_SNOW_010B8E00, 0)])
        );
        assert_eq!(calls(&e, VERTEX_PROFILE_00B4F3E0).len(), 4);
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, vec![0, 1, 2, 3]);
    }

    #[test]
    fn fn_00bb0360_compiles_the_two_pixel_shaders_into_the_globals() {
        let mut e = loader_rig();
        map_globals(&mut e);
        e.register(PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a, &[0][..]);
            0x0e57_0000u32.into_ret()
        });
        let this = e.mem.alloc(0xac);
        e.call(0x00bb_0360, &args![this]);
        let second = e.global::<u32>(PIXEL_SHADERS_012005C8 + 4);
        assert_eq!(
            compiled(&e, second),
            Compiled {
                file: PRECIPITATION_PIXEL_010B8E5C,
                output: FORMAT_PRECIP_PIXEL_010B8E4C,
                profile: 0x0e57_0000,
                pixel: true,
                macros: macro_words(&[(MACRO_SNOW_010B8E00, 0)]),
            }
        );
        assert_eq!(calls(&e, PIXEL_PROFILE_00B4F380).len(), 2);
    }

    /// Globals holding one vertex shader (deleted on release), one pixel shader (kept) and a
    /// pass (deleted).
    fn set_globals(e: &mut Engine) -> (u32, u32, u32) {
        map_globals(e);
        let vertex = object(e, 1);
        let pixel = object(e, 2);
        e.set_global(VERTEX_SHADERS_012005D0 + 8, vertex);
        e.set_global(PIXEL_SHADERS_012005C8, pixel);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.set_global(PASS_012005E0, pass);
        (vertex, pixel, pass)
    }

    #[test]
    fn fn_00bb0550_clears_the_globals_and_destroys() {
        let mut e = rig();
        noop(&mut e, &[BS_SHADER_DESTRUCT]);
        let (vertex, pixel, pass) = set_globals(&mut e);
        let this = e.mem.alloc(0xac);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x9c, held);
        e.call(0x00bb_0550, &args![this]);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010B8E80);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex], vec![held]]);
        assert_eq!(e.mem.u32(pixel + 4), 1);
        assert_eq!(e.global::<u32>(PIXEL_SHADERS_012005C8), 0);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.global::<u32>(PASS_012005E0), 0);
        assert_eq!(e.mem.u32(this + 0x9c), held);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bb0a50_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(&mut e, &[BS_SHADER_DESTRUCT, NI_FREE_00AA1460]);
        map_globals(&mut e);
        let this = e.mem.alloc(0xac);
        assert_eq!(e.call(0x00bb_0a50, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xac]]);
        e.call(0x00bb_0a50, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bb09b0_clears_the_globals_and_answers_the_base() {
        for answer in [0u32, 1] {
            let mut e = rig();
            e.register_double(CALL_00BE0870, move |_, _| answer.into_ret());
            let (_, _, pass) = set_globals(&mut e);
            let this = e.mem.alloc(0xac);
            assert_eq!(e.call(0x00bb_09b0, &args![this]).bool(), answer != 0);
            assert_eq!(e.global::<u32>(VERTEX_SHADERS_012005D0 + 8), 0);
            assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        }
    }

    #[test]
    fn fn_00bb0770_sets_up_the_global_pass() {
        let mut e = pass_rig();
        map_globals(&mut e);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.set_global(VERTEX_SHADERS_012005D0, vertex);
        e.set_global(PIXEL_SHADERS_012005C8, pixel);
        assert!(e.call(0x00bb_0770, &args![0u32]).bool());
        let pass = e.global::<u32>(PASS_012005E0);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![PASS_012005E0]]);
        assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 1)]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(e.mem.u32(vertex + 4), 2);
        assert_eq!(
            render_states(&e, pass),
            vec![
                (7, 1),
                (0xe, 0),
                (0x17, 4),
                (0x1b, 1),
                (0x13, 5),
                (0x14, 6),
                (0xf, 0),
                (0xa8, 7)
            ]
        );
        let stage = calls(&e, PASS_SET_STAGE)[0][2];
        assert_eq!(e.mem.u32(stage + 0x60), 1);
    }

    #[test]
    fn fn_00bb0a80_makes_both_maps() {
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
        let this = e.mem.alloc(0xac);
        e.call(0x00bb_0a80, &args![this]);
        let pixel = e.mem.u32(this + 0x30);
        let vertex = e.mem.u32(this + 0x34);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 8);
        assert_eq!(
            entries[0],
            vec![
                pixel,
                AMBIENT_COLOR_010B8FE4,
                0x1000_0006,
                0,
                0,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                0xc,
                4,
                this + 0xa0,
                0
            ]
        );
        assert_eq!(entries[1][0], vertex);
        assert_eq!(entries[1][1], WORLD_VIEW_PROJ_TRANSPOSE_010A1E50);
        assert_eq!(entries[7][1], CAM_UP_010B8FCC);
        assert_eq!(entries[7][4], 0xd);
        assert_eq!(entries[7][9], CAM_UP_012005F0);
        assert_eq!(entries[3][9], this + 0x6c);
    }

    #[test]
    fn fn_00bb1390_stores_two_and_adds_two() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.set_global(SUM_01200608, 1.5f32);
        e.set_global(SUM_0120060C, -2.0f32);
        e.call(0x00bb_1390, &args![0x7f80_0001u32, 3.0f32, 0.25f32, 1.0f32]);
        assert_eq!(e.global::<u32>(STORED_01200658), 0x7fc0_0001);
        assert_eq!(e.global::<f32>(STORED_01200668), 3.0);
        assert_eq!(e.global::<f32>(SUM_01200608), 1.75);
        assert_eq!(e.global::<f32>(SUM_0120060C), -1.0);
    }

    #[test]
    fn fn_00bb1960_transforms_the_light() {
        for lights in [0u32, 2] {
            let mut e = rig();
            map_globals(&mut e);
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.set_global(LIGHT_DIRECTION_011FA1F0 + 4 * i as u32, *v);
                e.set_global(LIGHT_POSITION_011FA180 + 4 * i as u32, *v * 10.0);
            }
            e.set_global(LIGHT_RANGE_011FA200, 9.0f32);
            // The transforms copy, the normalize halves.
            e.register(D3DX_VEC3_TRANSFORM_NORMAL, |e, a| {
                let v = e.mem.bytes(a[1], 12);
                e.mem.write(a[0], &v);
                Ret::default()
            });
            e.register(D3DX_VEC3_TRANSFORM_COORD, |e, a| {
                let v = e.mem.bytes(a[1], 12);
                e.mem.write(a[0], &v);
                Ret::default()
            });
            e.register(D3DX_VEC3_NORMALIZE, |e, a| {
                for i in 0..3 {
                    let v = e.mem.f32(a[1] + 4 * i);
                    e.mem.set_f32(a[0] + 4 * i, v / 2.0);
                }
                Ret::default()
            });
            e.register_double(ACTIVE_LIGHTS, move |_, _| lights.into_ret());
            e.call(0x00bb_1960, &args![0u32, 0x4444u32, 0x5555u32, 3.0f32]);
            let direction: Vec<f32> = (0..4)
                .map(|i| e.global::<f32>(DIRECTION_01200618 + 4 * i))
                .collect();
            assert_eq!(direction, vec![-0.5, -1.0, -1.5, 0.0]);
            assert_eq!(calls(&e, ACTIVE_LIGHTS), vec![vec![0x4444]]);
            assert_eq!(calls(&e, D3DX_VEC3_TRANSFORM_NORMAL)[0][2], 0x5555);
            if lights == 0 {
                assert!(calls(&e, D3DX_VEC3_TRANSFORM_COORD).is_empty());
                assert_eq!(e.global::<f32>(RANGE_01200634), 0.0);
            } else {
                assert_eq!(e.global::<f32>(POSITION_01200628 + 8), 30.0);
                assert_eq!(e.global::<f32>(RANGE_01200634), 3.0);
            }
        }
    }

    #[test]
    fn fn_00bb1a90_copies_the_two_rows() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let object = e.mem.alloc(0x90);
        for offset in [0x6cu32, 0x70, 0x78, 0x7c, 0x84, 0x88] {
            e.mem.set_f32(object + offset, offset as f32);
        }
        e.set_global(CAMERA_011F917C, object);
        e.set_global(ROW_01200648 + 0xc, 7u32);
        e.call(0x00bb_1a90, &args![]);
        let row = |e: &Engine, at: u32| {
            (0..4)
                .map(|i| e.global::<f32>(at + 4 * i))
                .collect::<Vec<_>>()
        };
        assert_eq!(row(&e, ROW_01200648), vec![108.0, 120.0, 132.0, 0.0]);
        assert_eq!(row(&e, ROW_01200638), vec![112.0, 124.0, 136.0, 0.0]);
    }

    #[test]
    fn fn_00bb1b20_picks_the_pass_state() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let this = e.mem.alloc(0x390);
        e.mem.set_u32(this + 0x384, 0x3e3e);
        e.mem.set_u32(this + 0x388, 0x3f3f);
        e.call(0x00bb_1b20, &args![this, 0x3du32]);
        assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 0);
        e.call(0x00bb_1b20, &args![this, 0x3eu32]);
        assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 0x3e3e);
        e.call(0x00bb_1b20, &args![this, 0x3fu32]);
        assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 0x3f3f);
    }

    #[test]
    fn fn_00bb1b50_copies_the_fog_property() {
        let mut e = rig();
        map_globals(&mut e);
        e.set_global(FOG_KIND_011F91C4, 2u8);
        let fog = e.mem.alloc(0x64);
        for (offset, value) in [
            (0x20u32, 0.1f32),
            (0x24, 0.2),
            (0x28, 0.3),
            (0x2c, 1.0),
            (0x30, 5.0),
            (0x60, 2.0),
        ] {
            e.mem.set_f32(fog + offset, value);
        }
        e.register_double(GET_FOG_PROPERTY, move |_, a| {
            assert_eq!(a, &[2][..]);
            fog.into_ret()
        });
        e.call(0x00bb_1b50, &args![0u32, 0u32]);
        let row = |e: &Engine, at: u32| {
            (0..4)
                .map(|i| e.global::<f32>(at + 4 * i))
                .collect::<Vec<_>>()
        };
        assert_eq!(row(&e, FOG_PARAM_011FA280), vec![5.0, 4.0, 2.0, 0.0]);
        assert_eq!(row(&e, FOG_COLOR_011FA290), vec![0.1, 0.2, 0.3, 0.0]);
        // No fog property: nothing is written.
        e.register(GET_FOG_PROPERTY, |_, _| Ret::default());
        e.set_global(FOG_PARAM_011FA280, 9.0f32);
        e.call(0x00bb_1b50, &args![0u32, 0u32]);
        assert_eq!(e.global::<f32>(FOG_PARAM_011FA280), 9.0);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::{object, rig};
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, order, slot_target, vobject};

    #[test]
    fn fn_00bb0490_and_00bb1a70_hand_their_pass_to_the_shader() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.set_global(PASS_012005E0, 0x5e0u32);
        let shader = vobject(&mut e, 0x400, &[]);
        e.mem.set_u32(shader + 0x384, 0x384);
        e.call(0x00bb_0490, &args![shader]);
        e.call(0x00bb_1a70, &args![shader]);
        assert_eq!(
            calls(&e, slot_target(&e, shader, 0x128)),
            vec![vec![shader, 0x5e0], vec![shader, 0x384]]
        );
    }

    #[test]
    fn the_small_accessors() {
        let mut e = rig();
        let this = e.mem.alloc(0xa0);
        assert_eq!(e.call(0x00bb_0540, &args![this]).u32(), RECORD_012005C0);
        assert_eq!(e.call(0x00bb_0d10, &args![this]).u32(), RTTI_012005FC);
        e.call(0x00bb_0cf0, &args![this, 3u32]);
        assert_eq!(e.call(0x00bb_0d20, &args![this]).u32(), 0);
        e.call(0x00bb_0cf0, &args![this, 7u32]);
        assert_eq!(e.call(0x00bb_0d20, &args![this]).u32(), 3);
        e.call(0x00bb_0d30, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0x94), 0x55);
        e.mem.set_u32(this + 0x90, 0x90);
        assert_eq!(e.call(0x00bb_0e90, &args![this]).u32(), 0x90);
    }

    #[test]
    fn fn_00bb0d40_constructs_the_property() {
        let mut e = rig();
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xa0);
        let vectors: Vec<u32> = (0..3)
            .map(|v| {
                let p = e.mem.alloc(0xc);
                for i in 0..3 {
                    e.mem.set_u32(p + 4 * i, 0x10 * (v + 1) + i);
                }
                p
            })
            .collect();
        let result = e
            .call(
                0x00bb_0d40,
                &args![
                    this,
                    7u32,
                    0x7fa0_0000u32,
                    vectors[0],
                    vectors[1],
                    vectors[2],
                    0x66u32,
                    5u32
                ],
            )
            .u32();
        assert_eq!(result, this);
        assert_eq!(calls(&e, SHADER_PROPERTY_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), PROPERTY_VTABLE_010B9008);
        assert_eq!(e.mem.u32(this + 0x60), 7);
        assert_eq!(e.mem.u32(this + 0x68), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0x6c), 0x20);
        assert_eq!(e.mem.u32(this + 0x80), 0x32);
        assert_eq!(e.mem.u32(this + 0x84), 0x10);
        assert_eq!(e.mem.u32(this + 0x9c), 0x66);
        assert_eq!(e.mem.u32(this + 0x98), 1);
        e.call(
            0x00bb_0d40,
            &args![this, 7u32, 0u32, vectors[0], vectors[1], vectors[2], 0x66u32, 0u32],
        );
        assert_eq!(e.mem.u32(this + 0x98), 0);
    }

    #[test]
    fn fn_00bb0ea0_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        e.register(SHADER_PROPERTY_DESTRUCT_00BAA0E0, |_, _| Ret::default());
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let this = e.mem.alloc(0xa0);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x90, held);
        e.mem.set_u32(this + 0x60, 9);
        assert_eq!(e.call(0x00bb_0ea0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), PROPERTY_VTABLE_010B9008);
        assert_eq!((e.mem.u32(this + 0x60), e.mem.u32(this + 0x90)), (0, 0));
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_DESTRUCT_00BAA0E0),
            vec![vec![this]]
        );
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
        e.call(0x00bb_0ea0, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xa0]]);
        // Only one release for the one reference.
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00bb0ed0_copies_the_fields_and_the_reference() {
        let mut e = rig();
        e.register(SHADER_PROPERTY_COPY_00BA8B10, |_, _| Ret::default());
        let source = e.mem.alloc(0xa0);
        let target = e.mem.alloc(0xa0);
        for offset in (0x60u32..=0x8c).step_by(4) {
            e.mem.set_u32(source + offset, offset);
        }
        e.mem.set_u32(source + 0x94, 0x94);
        let new = object(&mut e, 1);
        let old = object(&mut e, 1);
        e.mem.set_u32(source + 0x90, new);
        e.mem.set_u32(target + 0x90, old);
        e.call(0x00bb_0ed0, &args![source, target, 3u32]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_00BA8B10),
            vec![vec![source, target, 3]]
        );
        assert_eq!(e.mem.u32(target + 0x60), 0x60);
        assert_eq!(e.mem.u32(target + 0x64), 0);
        assert_eq!(e.mem.u32(target + 0x8c), 0x8c);
        assert_eq!(e.mem.u32(target + 0x90), new);
        assert_eq!((e.mem.u32(new + 4), e.mem.u32(old + 4)), (2, 0));
        assert_eq!(e.mem.u32(target + 0x94), 0x94);
        // The same reference: no count changes.
        e.call(0x00bb_0ed0, &args![source, target, 3u32]);
        assert_eq!(e.mem.u32(new + 4), 2);
    }

    #[test]
    fn fn_00bb0fc0_rebuilds_the_array_for_a_new_key() {
        let mut e = rig();
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + 0x2b4), 0x29);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(array + 0x10, 5);
        e.mem.set_u32(this + 0x3c, array);
        e.mem.set_u32(this + 0x38, 4);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 7);
        let args = args![this, 0x900u32, 4u32, 0u32, 2u32, 0u32, 0u32];
        assert_eq!(e.call(0x00bb_0fc0, &args).u32(), array);
        assert!(calls(&e, RENDER_PASS_ARRAY_ADD).is_empty());
        let args = args![this, 0x900u32, 6u32, 0u32, 2u32, 0u32, 0u32];
        assert_eq!(e.call(0x00bb_0fc0, &args).u32(), array);
        assert_eq!(
            calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
            vec![vec![this, 1]]
        );
        assert_eq!(e.mem.u32(array + 0x10), 0);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![array, 0x900, 0x2cf, 1, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x38), 0x206);
        assert_eq!(e.mem.u32(tls + 0x2b4), 7);
    }

    #[test]
    fn fn_00bb1080_clones_into_a_new_property() {
        let mut e = rig();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SHADER_PROPERTY_COPY_00BA8B10, |_, _| Ret::default());
        let source = e.mem.alloc(0xa0);
        e.mem.set_u32(source + 0x94, 0x94);
        let made = e.call(0x00bb_1080, &args![source, 2u32]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xa0]]);
        assert_eq!(e.mem.u32(made), PROPERTY_VTABLE_010B9008);
        assert_eq!(e.mem.u32(made + 0x98), 1);
        assert_eq!(e.mem.u32(made + 0x1c), 0x12);
        assert_eq!(e.mem.u32(made + 0x94), 0x94);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_00BA8B10),
            vec![vec![source, made, 2]]
        );
    }

    #[test]
    fn fn_00bb1160_adds_the_viewer_strings() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.set_global(RTTI_012005FC, 0x5fcu32);
        for (target, answer) in [
            (SHADER_PROPERTY_VIEWER_STRINGS, 0u32),
            (VIEWER_STRING, 0x1001),
            (VIEWER_STRING_INT, 0x1002),
            (VIEWER_STRING_TEXT, 0x1003),
            (VIEWER_STRING_POINTER_00A7D5A0, 0x1004),
            (VIEWER_STRING_FLAG, 0x1005),
            (VIEWER_STRING_00A7DB60, 0x1006),
            (STRINGS_SET_SIZE, 0),
        ] {
            e.register_double(target, move |_, _| answer.into_ret());
        }
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x40 + 4 * a[1], word);
            Ret::default()
        });
        let strings = e.mem.alloc(0x80);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 4);
        let this = vobject(&mut e, 0xa0, &[(0xcc, 1)]);
        e.mem.set_u32(this + 0x60, 0x60);
        e.mem.set_u32(this + 0x98, 0x98);
        e.call(0x00bb_1160, &args![this, strings]);
        let added: Vec<u32> = (0..3).map(|i| e.mem.u32(strings + 0x40 + 4 * i)).collect();
        assert_eq!(added, vec![0x1001, 0x1002, 0x1006]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING), vec![vec![0x5fc]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_INT),
            vec![vec![LABEL_010B90EC, 0x60]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_00A7DB60),
            vec![vec![LABEL_010B90DC, 0x98]]
        );
        assert_eq!(calls(&e, STRINGS_SET_SIZE), vec![vec![strings, 6]]);
        // With a reference and the +0xcc answer 0: the reference line and the flag.
        let this = vobject(&mut e, 0xa0, &[]);
        e.mem.set_u32(this + 0x90, 0x9090);
        e.register(TEXTURE_NAME_00C4BB40, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00bb_1160, &args![this, strings]);
        assert_eq!(
            calls(&e, VIEWER_STRING_POINTER_00A7D5A0),
            vec![vec![LABEL_010AE79C, 0x9090]]
        );
        assert_eq!(calls(&e, VIEWER_STRING_FLAG), vec![vec![LABEL_010AE344, 1]]);
        e.register(TEXTURE_NAME_00C4BB40, |_, _| 0x4bb4u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00bb_1160, &args![this, strings]);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT),
            vec![vec![LABEL_010AE79C, 0x4bb4]]
        );
        assert!(order(&e).contains(&TEXTURE_NAME_00C4BB40));
    }

    #[test]
    fn fn_00bb12e0_takes_the_texture_of_the_geometry() {
        let mut e = rig();
        e.register(GEOMETRY_CALL_00654640, |_, _| Ret::default());
        let this = vobject(&mut e, 0xa0, &[]);
        let geometry = e.mem.alloc(0xc0);
        assert!(!e.call(0x00bb_12e0, &args![this, geometry]).bool());
        let holder = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xb0, holder);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(holder + 0x20, entries);
        // No entry: no texture and mode 3.
        assert!(e.call(0x00bb_12e0, &args![this, geometry]).bool());
        assert_eq!(calls(&e, slot_target(&e, this, 0xd0)), vec![vec![this, 3]]);
        assert_eq!(
            calls(&e, GEOMETRY_CALL_00654640),
            vec![vec![geometry, holder]]
        );
        let texture = object(&mut e, 0);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u16(entry + 4, 0x2fff);
        e.mem.set_u32(entry + 8, texture);
        e.mem.set_u32(entries, entry);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00bb_12e0, &args![this, geometry]).bool());
        assert_eq!(e.mem.u32(this + 0x90), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert_eq!(calls(&e, slot_target(&e, this, 0xd0)), vec![vec![this, 2]]);
    }
}
