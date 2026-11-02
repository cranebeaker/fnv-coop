//! `BSShader/debugshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// `DebugShader::DebugShader` (`00bd1950`), `::LoadShaders` (`00bd16f0`) and
/// `::LoadStagesAndPasses` (`00bd20e0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bd_1950;
const LOAD_SHADERS: u32 = 0x00bd_16f0;
const LOAD_STAGES_AND_PASSES: u32 = 0x00bd_20e0;

// Translated from 00bd23f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`) calls for
/// shader 9. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 2 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)) and calls slot +0xb4. Then allocates
/// 0xc0 bytes and runs the constructor (`00bd1950`; a null allocation is used as it is),
/// calls the shader's slot +0x4c, `LoadShaders` (`00bd16f0`), `LoadStagesAndPasses`
/// (`00bd20e0`), then slot +0xc0 with the declaration (no slot +0x11c here), and answers the
/// shader.
// Translation incomplete: 00bd23f0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn debug_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    e.call(LOAD_STAGES_AND_PASSES, &args![shader]);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The debug shader's vtable (`010bbd08`; no RTTI locator before it).
const DEBUG_SHADER_VTABLE_010BBD08: u32 = 0x010b_bd08;
/// The element constructor and destructor of the 0x10-pointer array at +0x6c (`006694e0`,
/// `0045cec0`).
const POINTER_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00bd1950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::DebugShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010bbd08`, constructs the 0x10
/// pointers at +0x6c (constructor `006694e0`, destructor `0045cec0`), clears the words +0xac
/// and +0xb0, constructs the `NiPointer` array at +0xb4 (2, destructor `00b66f20`), sets +0x68
/// to 10, then clears (releasing what they hold) the `NiPointer`s at +0xb0, +0xb4, +0xb8,
/// +0xac and +0x6c to +0xa8, and sets the byte +0x24 to 1. Answers `this`.
// Translation incomplete: 00bd1950 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the base and the arrays) is not translated.
pub fn debug_shader_debug_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, DEBUG_SHADER_VTABLE_010BBD08);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x6c,
            4u32,
            0x10u32,
            POINTER_CONSTRUCT_006694E0,
            POINTER_DESTRUCT_0045CEC0
        ],
    );
    e.mem.set_u32(base + 0xac, 0);
    e.mem.set_u32(base + 0xb0, 0);
    construct_pointer_array(e, base + 0xb4, 2, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 10);
    ni_pointer_clear(e, base + 0xb0);
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0xb4 + 4 * index);
    }
    ni_pointer_clear(e, base + 0xac);
    for index in 0..0x10u32 {
        ni_pointer_clear(e, base + 0x6c + 4 * index);
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"imagespace\\1x\\v\\base_old.v.hlsl"`
/// (`010bbce8`), `"imagespace\\1x\\p\\copy.p.hlsl"` (`010b0820`), the macro `"CHANNELMASK"`
/// (`010bbcc4`) with the value `""` (`01011584`), the compiled names `"DEBUG.vso"`,
/// `"DEBUG00.pso"`, `"DEBUG01.pso"` and the vertex profile `"vs_1_1"` (`010a93ec`).
const VERTEX_SOURCE_010BBCE8: u32 = 0x010b_bce8;
const PIXEL_SOURCE_010B0820: u32 = 0x010b_0820;
const MACRO_CHANNELMASK_010BBCC4: u32 = 0x010b_bcc4;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const OUTPUT_VSO_010BBCDC: u32 = 0x010b_bcdc;
const OUTPUT_PSO_010BBCD0: u32 = 0x010b_bcd0;
const OUTPUT_PSO_010BBCB8: u32 = 0x010b_bcb8;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// The pixel profile chooser `00b4f380` (unit `bsshadermanager.cpp`, unnamed; `cdecl`, a byte:
/// answers the profile string of a shader model setting).
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;

/// The source blocks of `LoadShaders`' frame (0x300 bytes up to the return address).
const VERTEX_BLOCK: ShaderSource = ShaderSource {
    offset: 0xa8,
    words: &[VERTEX_SOURCE_010BBCE8, 0, 0],
    cleared: 0x40,
};
const PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x5c,
    words: &[PIXEL_SOURCE_010B0820, 0, 0],
    cleared: 0x40,
};
const MASKED_PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x10,
    words: &[
        PIXEL_SOURCE_010B0820,
        MACRO_CHANNELMASK_010BBCC4,
        EMPTY_STRING_01011584,
        0,
    ],
    cleared: 0x3c,
};
const LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xf4,
    path: 0x1f8,
};

// Translated from 00bd16f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Takes the pixel profile
/// from `00b4f380(1)`, then fills each source block on its stack just before its compile: the
/// vertex file `base_old.v.hlsl` (no macros) as `"DEBUG.vso"` (`vs_1_1`) into the `NiPointer`
/// at +0xb0, the pixel file `copy.p.hlsl` (no macros) as `"DEBUG00.pso"` into +0xb4, and the
/// same pixel file with `CHANNELMASK` = `""` as `"DEBUG01.pso"` into +0xb8 (each step
/// `MakePath`, the name copied by `00406d00`, the compile, the assignment).
// Translation incomplete: 00bd16f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn debug_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let pixel_profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
    let steps = [
        (
            VERTEX_BLOCK,
            ShaderCompile {
                source: 0xa8,
                output: OUTPUT_VSO_010BBCDC,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                slot: 0xb0,
            },
        ),
        (
            PIXEL_BLOCK,
            ShaderCompile {
                source: 0x5c,
                output: OUTPUT_PSO_010BBCD0,
                profile: pixel_profile,
                pixel: true,
                slot: 0xb4,
            },
        ),
        (
            MASKED_PIXEL_BLOCK,
            ShaderCompile {
                source: 0x10,
                output: OUTPUT_PSO_010BBCB8,
                profile: pixel_profile,
                pixel: true,
                slot: 0xb8,
            },
        ),
    ];
    e.with_stack(0x300, |e, frame| {
        for (block, step) in steps {
            set_up_sources(e, frame.addr(), &[block]);
            compile_shaders(e, this.addr(), frame.addr(), &LOAD_SHADERS_BUFFERS, &[step]);
        }
    });
}

// Translated from 00bd20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::LoadStagesAndPasses` (Xbox PDB), `thiscall`, no stack words; answers 1. Gives
/// the pass pointer at +0x5c a new pass (`00e804d0`) with one stage (`CreateNewStage` into a
/// stack pointer, `SetupDefaultStage` index 0 with 3, 2, 1, `SetFilterMode(0)`, `SetStage`),
/// the vertex shader +0xb0 (pass +0x5c) and pixel shader +0xb4 (pass +0x44), the render states
/// 7 = 0, 0xe = 0, 0x1b = 0, 0x13 = 2, 0x14 = 2 (through the pass read for the one before) and
/// 0xa8 = 0xf; then drops the stage reference.
// Translation incomplete: 00bd20e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn debug_shader_load_stages_and_passes(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let slot = base + 0x5c;
    e.call(NEW_PASS_00E804D0, &args![slot]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let stage = e.mem.u32(local);
        e.call(SET_FILTER_MODE, &args![stage, 0u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + 0xb0);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + 0xb4);
        ni_pointer_assign(e, pass + 0x44, pixel);
        for (state, value) in [(7u32, 0u32), (0xe, 0), (0x1b, 0)] {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0x13, 2);
        pass_render_state(e, pass, 0x14, 2);
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0xa8, 0xf);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// The `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor), `BSShader::~BSShader` (engine map, `00be0f10`, no stack words) and the Ni free
/// (`00aa1460`, `cdecl`: block, size).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bd1ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `debugshader.cpp`): the debug shader's destructor, no stack
/// words. Stores the vtable `010bbd08`, clears (releasing what they hold) the `NiPointer`s
/// +0xb0, +0xb4, +0xb8, +0xac and +0x6c to +0xa8, destroys the array +0xb4 (2, destructor
/// `00b66f20`), releases +0xb0 and +0xac again when not null (they were just cleared),
/// destroys the 0x10 pointers at +0x6c (destructor `0045cec0`) and runs `BSShader::~BSShader`
/// (`00be0f10`).
// Translation incomplete: 00bd1ab0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the members and the base) is not translated.
pub fn fn_00bd1ab0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, DEBUG_SHADER_VTABLE_010BBD08);
    ni_pointer_clear(e, base + 0xb0);
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0xb4 + 4 * index);
    }
    ni_pointer_clear(e, base + 0xac);
    for index in 0..0x10u32 {
        ni_pointer_clear(e, base + 0x6c + 4 * index);
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0xb4, 4u32, 2u32, DESTRUCT_00B66F20],
    );
    for offset in [0xb0u32, 0xac] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_reference(e, held);
        }
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x6c, 4u32, 0x10u32, POINTER_DESTRUCT_0045CEC0],
    );
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The renderer object (`011f4748`), `2^32` (`float`, `01099378`, added to a negative
/// `FILD`), the constants of the debug rectangles (`float` `010181bc`, `double`s `0103c7c8`,
/// `01011588`, `01011598`, `0106b168`, `010760f0`), `NiScreenElementsData::SetRectangle`
/// (`00a88020`, `thiscall`, `RET 0x14`), `::UpdateBound` (`00a87900`, no stack words) and
/// `NiScreenElements::Draw` (`00a87790`, `thiscall`, `RET 4`) (engine map).
const RENDERER_OBJECT_011F4748: u32 = 0x011f_4748;
const TWO_POWER_32_01099378: u32 = 0x0109_9378;
const RECT_LEFT_010181BC: u32 = 0x0101_81bc;
const RECT_OFFSET_0103C7C8: u32 = 0x0103_c7c8;
const COLUMN_STEP_01011588: u32 = 0x0101_1588;
const ROW_STEP_01011598: u32 = 0x0101_1598;
const WIDTH_SCALE_0106B168: u32 = 0x0106_b168;
const HEIGHT_SCALE_010760F0: u32 = 0x0107_60f0;
const SET_RECTANGLE: u32 = 0x00a8_8020;
const UPDATE_BOUND: u32 = 0x00a8_7900;
const SCREEN_ELEMENTS_DRAW: u32 = 0x00a8_7790;
/// Bits of `1.0f`.
const ONE_BITS: u32 = 0x3f80_0000;

/// `FILD dword` of an unsigned value as the compiler converts it: `2^32` (the `float` at
/// `01099378`) added when the signed value is negative.
fn unsigned_load(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.global::<u32>(TWO_POWER_32_01099378)))
    } else {
        loaded
    }
}

/// The two ratios the debug quads are scaled by: `ratio / max(ratio_f, 1)` and
/// `1 / max(ratio_f, 1)` as the code's compares give them (a NaN ratio kept), where `ratio`
/// is the quotient as the FPU holds it and `ratio_f` the same stored as a `float`.
fn fitted(ratio: f64) -> (u32, u32) {
    let ratio_bits = x87_store32(ratio);
    let ratio_f = x87_load32(ratio_bits);
    // FCOM 1.0 with the ratio; TEST AH,0x41: below 1 takes 1.0, else (and NaN) the ratio.
    let first = if 1.0 > ratio_f { ONE_BITS } else { ratio_bits };
    let scaled = x87_store32(x87_div(ratio, x87_load32(first)));
    // FCOM the ratio with 1.0; TEST AH,5: below 1 takes 1.0, else (and NaN) the ratio.
    let second = if ratio_f < 1.0 { ONE_BITS } else { ratio_bits };
    (scaled, second)
}

// Translated from 00bd1cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `debugshader.cpp`; `00b55a10` calls it), `thiscall`, `RET 4`
/// (a target, or null for the screen). Draws the shader's sixteen textures (+0x6c) as quads
/// in a 4 by 4 grid through its screen elements (+0xac). The target's size is its +0x30
/// object's slots +0x94 and +0x98 (0 without one), the screen's the renderer's (`011f4748`)
/// slot +0xc8 object's slots +0x8c and +0x90 (with 0); from the height over the width the
/// fitted ratios give the rectangle (`010181bc`, the width ratio, the height ratio minus the
/// `double` `0103c7c8`, 0.0) the renderer's slot +0x190 gets. For each texture present, with
/// the loop index kept in the shader's word +0xbc: its own size gives the fitted ratios, the
/// quad's `SetRectangle(0, column * 0.5 - 1, 1 - row * 0.5, ...)` with the two ratios scaled by
/// the `double`s `0106b168` and `010760f0` (`00a88020` on the screen elements' +0xb8), then
/// `UpdateBound`, the screen elements' +0xc0 set to the shader, the shader's slot +0x50 with
/// them, the renderer's slot +0x190 with the rectangle when its word +0x200 is 1 or 2 and
/// its byte +0x208 is 1, `NiScreenElements::Draw` with the renderer, and the texture's
/// pointer cleared (releasing it) and +0xc0 cleared. The screen elements are held for the
/// loop as an `NiPointer`.
// Translation incomplete: 00bd1cf0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held screen elements) is not translated.
pub fn fn_00bd1cf0(e: &mut Engine, this: Ptr, target: Ptr) {
    let this = this.addr();
    let target = target.addr();
    let (width, height) = if target != 0 {
        let maps = e.mem.u32(target + 0x30);
        let width = if maps != 0 {
            e.vcall(maps, 0x94, &args![]).u32()
        } else {
            0
        };
        let width = x87_store32(unsigned_load(e, width));
        let maps = e.mem.u32(target + 0x30);
        let height = if maps != 0 {
            e.vcall(maps, 0x98, &args![]).u32()
        } else {
            0
        };
        (width, height)
    } else {
        let renderer = e.global::<u32>(RENDERER_OBJECT_011F4748);
        let screen = e.vcall(renderer, 0xc8, &args![]).u32();
        let width = e.vcall(screen, 0x8c, &args![0u32]).u32();
        let width = x87_store32(unsigned_load(e, width));
        let renderer = e.global::<u32>(RENDERER_OBJECT_011F4748);
        let screen = e.vcall(renderer, 0xc8, &args![]).u32();
        let height = e.vcall(screen, 0x90, &args![0u32]).u32();
        (width, height)
    };
    let height = x87_store32(unsigned_load(e, height));
    let ratio = x87_div(x87_load32(height), x87_m32(width));
    let (scaled, second) = fitted(ratio);
    let inverse = x87_store32(x87_div(1.0, x87_m32(second)));
    e.with_stack(0x10, |e, rectangle| {
        let rectangle = rectangle.addr();
        let left = x87_quiet_bits(e.global::<u32>(RECT_LEFT_010181BC));
        e.mem.set_u32(rectangle, left);
        e.mem.set_u32(rectangle + 4, x87_quiet_bits(scaled));
        let offset = f64::from_bits(e.global::<u64>(RECT_OFFSET_0103C7C8));
        let bottom = x87_store32(x87_sub(x87_load32(inverse), offset));
        e.mem.set_u32(rectangle + 8, bottom);
        e.mem.set_u32(rectangle + 0xc, 0);
        let screen = e.mem.u32(this + 0xac);
        if screen != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![screen.wrapping_add(4)]);
        }
        e.mem.set_u32(this + 0xbc, 0);
        loop {
            let index = e.mem.u32(this + 0xbc);
            let texture = e.mem.u32(this + 0x6c + 4 * index);
            if texture != 0 {
                let width = e.vcall(texture, 0x94, &args![]).u32();
                let width = x87_store32(unsigned_load(e, width));
                let index = e.mem.u32(this + 0xbc);
                let texture = e.mem.u32(this + 0x6c + 4 * index);
                let height = e.vcall(texture, 0x98, &args![]).u32();
                let height = x87_store32(unsigned_load(e, height));
                let ratio = x87_div(x87_load32(width), x87_m32(height));
                let (scaled, second) = fitted(ratio);
                // FDIVR ST(0),ST(1) (`D8 F9`): 1.0 over the loaded value.
                let inverse = x87_store32(x87_div(1.0, x87_load32(second)));
                let index = e.mem.u32(this + 0xbc);
                let row = x87_store32(unsigned_load(e, index >> 2));
                let column = x87_store32(unsigned_load(e, index & 3));
                let data = e.mem.u32(screen + 0xb8);
                let step = f64::from_bits(e.global::<u64>(COLUMN_STEP_01011588));
                // FSUB ST(0),ST(1) (`D8 E1`): minus the 1.0 below it.
                let x = x87_store32(x87_sub(x87_mul(x87_load32(column), step), 1.0));
                let step = f64::from_bits(e.global::<u64>(ROW_STEP_01011598));
                let y = x87_store32(x87_add(1.0, x87_mul(x87_load32(row), step)));
                let factor = f64::from_bits(e.global::<u64>(WIDTH_SCALE_0106B168));
                let w = x87_store32(x87_mul(x87_load32(scaled), factor));
                let factor = f64::from_bits(e.global::<u64>(HEIGHT_SCALE_010760F0));
                let h = x87_store32(x87_mul(x87_load32(inverse), factor));
                e.call(
                    SET_RECTANGLE,
                    &args![
                        data,
                        0u32,
                        x87_quiet_bits(x),
                        x87_quiet_bits(y),
                        x87_quiet_bits(w),
                        x87_quiet_bits(h)
                    ],
                );
                let data = e.mem.u32(screen + 0xb8);
                e.call(UPDATE_BOUND, &args![data]);
                e.mem.set_u32(screen + 0xc0, this);
                e.vcall(this, 0x50, &args![screen]);
                let renderer = e.global::<u32>(RENDERER_OBJECT_011F4748);
                let mode = e.mem.u32(renderer + 0x200);
                let renderer = if (mode == 1 || mode == 2) && e.mem.u8(renderer + 0x208) == 1 {
                    e.vcall(renderer, 0x190, &args![rectangle]);
                    e.global::<u32>(RENDERER_OBJECT_011F4748)
                } else {
                    renderer
                };
                e.call(SCREEN_ELEMENTS_DRAW, &args![screen, renderer]);
                let index = e.mem.u32(this + 0xbc);
                ni_pointer_clear(e, this + 0x6c + 4 * index);
                e.mem.set_u32(screen + 0xc0, 0);
            }
            let next = e.mem.u32(this + 0xbc).wrapping_add(1);
            e.mem.set_u32(this + 0xbc, next);
            if next >= 0x10 {
                break;
            }
        }
        if screen != 0 {
            release_reference(e, screen);
        }
    });
}

// Translated from 00bd23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `debugshader.cpp`): the scalar deleting destructor (vtable
/// `010bbd08` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00bd1ab0`) and, when bit 0
/// of the flags is set, frees the 0xc0 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bd23c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd1ab0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xc0u32]);
    }
    this
}

/// `00be0870` (unnamed, `thiscall`, no stack words; answers a bool),
/// `NiScreenElementsData::NiScreenElementsData` (`00a881e0`, `thiscall`, nine words),
/// `NiScreenElements::NiScreenElements` (`00a87810`, `thiscall`, the data),
/// `NiScreenElementsData::Insert` (`00a87ae0`, `thiscall`, three words) and `::SetTextures`
/// (`00a88130`, `thiscall`, six words), `NiAVObject::UpdateProperties` (`00a5a040`, no stack
/// words) and `00a59c60` (unit `NiMain`, unnamed; `thiscall`, the address of an update
/// record) (engine map).
const CALL_00BE0870: u32 = 0x00be_0870;
const SCREEN_ELEMENTS_DATA_CONSTRUCT: u32 = 0x00a8_81e0;
const SCREEN_ELEMENTS_CONSTRUCT: u32 = 0x00a8_7810;
const SCREEN_ELEMENTS_DATA_INSERT: u32 = 0x00a8_7ae0;
const SCREEN_ELEMENTS_DATA_SET_TEXTURES: u32 = 0x00a8_8130;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// The rectangle's `float`s (`01012054`, `010162c0`, `0104a494`), the `"ChannelMask"` name
/// (`010bbe44`) and the mask data (`011ae620`).
const RECT_LEFT_01012054: u32 = 0x0101_2054;
const RECT_WIDTH_010162C0: u32 = 0x0101_62c0;
const RECT_HEIGHT_0104A494: u32 = 0x0104_a494;
const CHANNEL_MASK_010BBE44: u32 = 0x010b_be44;
const CHANNEL_MASK_011AE620: u32 = 0x011a_e620;

// Translated from 00bd24c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::Initialize` (Xbox PDB), no stack words. Runs `00be0870` (its answer is the
/// answer); makes the screen elements (0xc4 bytes; their data 0x70 bytes,
/// `NiScreenElementsData(0, 0, 1, 1, 1, 4, 1, 2, 1)`, null when that allocation failed; the
/// elements null when theirs failed) into the `NiPointer` +0xac; on their data (+0xb8):
/// `Insert(4, 0, 0)`, `SetRectangle(0, ...)` with the `float`s `01012054`, 1.0, `010162c0`,
/// `0104a494`, `SetTextures(0, 0, 0.0, 0.0, 1.0, 1.0)`, `UpdateBound`; on the elements
/// `UpdateProperties` and `00a59c60` with a stack record (0.0 and five zero bytes). When the
/// pixel constant map (+0x30) is null, makes one (kind 2) with `"ChannelMask"` (register 5,
/// `011ae620`).
// Translation incomplete: 00bd24c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the allocations) is not translated.
pub fn debug_shader_initialize(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let result = e.call(CALL_00BE0870, &args![base]).bool();
    e.with_stack(0xc, |e, record| {
        let record = record.addr();
        e.mem.set_u32(record, 0);
        for offset in 4..9u32 {
            e.mem.set_u8(record + offset, 0);
        }
        let block = e.call(NI_ALLOCATE_00AA13E0, &args![0xc4u32]).u32();
        let elements = if block == 0 {
            0
        } else {
            let data_block = e.call(NI_ALLOCATE_00AA13E0, &args![0x70u32]).u32();
            let data = if data_block != 0 {
                e.call(
                    SCREEN_ELEMENTS_DATA_CONSTRUCT,
                    &args![data_block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
                )
                .u32()
            } else {
                0
            };
            e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![block, data]).u32()
        };
        ni_pointer_assign(e, base + 0xac, elements);
        let data = e.mem.u32(e.mem.u32(base + 0xac) + 0xb8);
        e.call(SCREEN_ELEMENTS_DATA_INSERT, &args![data, 4u32, 0u32, 0u32]);
        let data = e.mem.u32(e.mem.u32(base + 0xac) + 0xb8);
        let floats = [
            RECT_LEFT_01012054,
            0,
            RECT_WIDTH_010162C0,
            RECT_HEIGHT_0104A494,
        ]
        .map(|at| {
            if at == 0 {
                ONE_BITS
            } else {
                x87_quiet_bits(e.global::<u32>(at))
            }
        });
        e.call(
            SET_RECTANGLE,
            &args![data, 0u32, floats[0], floats[1], floats[2], floats[3]],
        );
        let data = e.mem.u32(e.mem.u32(base + 0xac) + 0xb8);
        e.call(
            SCREEN_ELEMENTS_DATA_SET_TEXTURES,
            &args![data, 0u32, 0u32, 0u32, 0u32, ONE_BITS, ONE_BITS],
        );
        let data = e.mem.u32(e.mem.u32(base + 0xac) + 0xb8);
        e.call(UPDATE_BOUND, &args![data]);
        let elements = e.mem.u32(base + 0xac);
        e.call(UPDATE_PROPERTIES, &args![elements]);
        let elements = e.mem.u32(base + 0xac);
        e.call(UPDATE_00A59C60, &args![elements, record]);
    });
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
        add_constant_entry(
            e,
            base,
            0x30,
            [
                CHANNEL_MASK_010BBE44,
                0x1000_0007,
                0,
                5,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                0x10,
                4,
                CHANNEL_MASK_011AE620,
                0,
            ],
        );
    }
    result
}

// Translated from 00bd1940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 72 of the vtable `010bbd08`, `thiscall`, no
/// stack words: a jump to `DebugShader::LoadShaders`.
pub fn fn_00bd1940(e: &mut Engine, this: Ptr) {
    debug_shader_load_shaders(e, this);
}

/// The current pass (`0126f74c`) and the texture index (`01202338`) `00bd1c50` reads.
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const TEXTURE_INDEX_01202338: u32 = 0x0120_2338;

// Translated from 00bd1c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `debugshader.cpp`), `thiscall`, seven words it does not
/// read: `this`'s slot +0xec; the pass at +0x5c gets the texture `this[0x6c + 4 * this[+0xbc]]`
/// on its first stage (+8) and the `NiPointer` `this[0xb4 + 4 * [01202338]]` at +0x44 (old one
/// released, new one held when they differ), and becomes current (`0126f74c`). Answers 0.
pub fn fn_00bd1c50(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    e.vcall(base, 0xec, &args![]);
    let pass = e.mem.u32(base + 0x5c);
    let stage = e.mem.u32(e.mem.u32(pass + 0x24));
    let selected = e.mem.u32(base + 0xbc);
    let texture = e.mem.u32(
        base.wrapping_add(selected.wrapping_mul(4))
            .wrapping_add(0x6c),
    );
    e.mem.set_u32(stage + 8, texture);
    let pass = e.mem.u32(base + 0x5c);
    let index = e.global::<u32>(TEXTURE_INDEX_01202338);
    let new = e
        .mem
        .u32(base.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xb4));
    let old = e.mem.u32(pass + 0x44);
    if old != new {
        if old != 0 {
            let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
            if left == 0 {
                e.vcall(old, 4, &args![]);
            }
        }
        e.mem.set_u32(pass + 0x44, new);
        if new != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new + 4]);
        }
    }
    let pass = e.mem.u32(base + 0x5c);
    e.set_global(CURRENT_PASS_0126F74C, pass);
    0
}

/// The two RTTI records `SetTextureToRender` looks for in the texture's chain (`011f444c`,
/// `011f4ac4`; by their place, unconfirmed which classes).
const RTTI_011F444C: u32 = 0x011f_444c;
const RTTI_011F4AC4: u32 = 0x011f_4ac4;

/// Whether `rtti` is in the RTTI chain the texture's slot +8 answers (base at +4).
fn has_rtti(e: &mut Engine, texture: u32, rtti: u32) -> bool {
    let mut at = e.vcall(texture, 8, &args![]).u32();
    while at != 0 {
        if at == rtti {
            return true;
        }
        at = e.mem.u32(at + 4);
    }
    false
}

// Translated from 00bd22c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugShader::SetTextureToRender` (Xbox PDB), `thiscall`, one word (a texture): finds the
/// first empty slot of the 16 texture `NiPointer`s at +0x6c (nothing without one); with a
/// texture whose RTTI chain (slot +8, base +4) has `011f444c`, or else `011f4ac4`, puts it in
/// that slot (old one released, new one held; nothing when it is already there).
pub fn debug_shader_set_texture_to_render(e: &mut Engine, this: Ptr, texture: Ptr) {
    let base = this.addr();
    let Some(slot) = (0..0x10u32).find(|index| e.mem.u32(base + 0x6c + 4 * index) == 0) else {
        return;
    };
    let texture = texture.addr();
    if texture == 0 {
        return;
    }
    if !has_rtti(e, texture, RTTI_011F444C) && !has_rtti(e, texture, RTTI_011F4AC4) {
        return;
    }
    let at = base + 0x6c + 4 * slot;
    let old = e.mem.u32(at);
    if old == texture {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(at, texture);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_1940, fn_00bd1940(Ptr)),
        entry!(
            0x00bd_1c50,
            fn_00bd1c50(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bd_22c0, debug_shader_set_texture_to_render(Ptr, Ptr)),
        entry!(0x00bd_1950, debug_shader_debug_shader(Ptr) -> Ptr),
        entry!(0x00bd_16f0, debug_shader_load_shaders(Ptr)),
        entry!(0x00bd_20e0, debug_shader_load_stages_and_passes(Ptr) -> bool),
        entry!(0x00bd_23f0, debug_shader_create_shader() -> Ptr),
        entry!(0x00bd_1ab0, fn_00bd1ab0(Ptr)),
        entry!(0x00bd_1cf0, fn_00bd1cf0(Ptr, Ptr)),
        entry!(0x00bd_23c0, fn_00bd23c0(Ptr, u32) -> Ptr),
        entry!(0x00bd_24c0, debug_shader_initialize(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn debug_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, LOAD_STAGES_AND_PASSES]);
        let (shader, log) = run(&mut e, 0x00bd_23f0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xc0]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (LOAD_STAGES_AND_PASSES, vec![SHADER]),
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
    fn debug_shader_constructor_builds_and_clears_the_pointers() {
        let mut e = rig();
        let this = e.mem.alloc(0xbc);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), DEBUG_SHADER_VTABLE_010BBD08);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                vec![
                    this + 0x6c,
                    4,
                    0x10,
                    POINTER_CONSTRUCT_006694E0,
                    POINTER_DESTRUCT_0045CEC0
                ],
                pointer_array(this + 0xb4, 2, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 10);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        // Everything just cleared: nothing to release.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn debug_shader_load_shaders_compiles_three_shaders_with_the_chosen_profile() {
        const PROFILE: u32 = 0x0ed0_0000;
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a[0], 1);
            PROFILE.into_ret()
        });
        let this = e.mem.alloc(0xbc);
        e.call(LOAD_SHADERS, &args![this]);
        let mask = macro_words(&[(MACRO_CHANNELMASK_010BBCC4, EMPTY_STRING_01011584)]);
        let expected = [
            (
                0xb0u32,
                VERTEX_SOURCE_010BBCE8,
                OUTPUT_VSO_010BBCDC,
                PROFILE_VS_1_1_010A93EC,
                false,
                [0; 8],
            ),
            (
                0xb4,
                PIXEL_SOURCE_010B0820,
                OUTPUT_PSO_010BBCD0,
                PROFILE,
                true,
                [0; 8],
            ),
            (
                0xb8,
                PIXEL_SOURCE_010B0820,
                OUTPUT_PSO_010BBCB8,
                PROFILE,
                true,
                mask,
            ),
        ];
        for (slot, file, output, profile, pixel, macros) in expected {
            let shader = e.mem.u32(this + slot);
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
        }
        let cleared: Vec<u32> = calls(&e, MEMSET).iter().map(|a| a[2]).collect();
        assert_eq!(cleared, vec![0x40, 0x40, 0x3c]);
    }

    #[test]
    fn debug_shader_load_stages_and_passes_builds_one_pass() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xbc);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0xb0, vertex);
        e.mem.set_u32(this + 0xb4, pixel);
        assert!(e.call(LOAD_STAGES_AND_PASSES, &args![this]).bool());
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x5c]]);
        let pass = e.mem.u32(this + 0x5c);
        assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 0)]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(
            render_states(&e, pass),
            vec![
                (7, 0),
                (0xe, 0),
                (0x1b, 0),
                (0x13, 2),
                (0x14, 2),
                (0xa8, 0xf)
            ]
        );
        let stage = calls(&e, PASS_SET_STAGE)[0][2];
        assert_eq!(e.mem.u32(stage + 0x60), 1);
    }
}

#[cfg(test)]
mod debug_function_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const SLOTS: u32 = 0x0e00_0000;
    const VTABLE: u32 = 0x0e80_0000;

    fn vtable(e: &mut Engine, n: u32) -> u32 {
        let table = VTABLE + n * 0x1000;
        let targets: Vec<u32> = (0..0x70u32).map(|i| SLOTS + n * 0x1000 + 4 * i).collect();
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
            0x0101_6000,
            0x0101_8000,
            0x0103_c000,
            0x0104_a000,
            0x0106_b000,
            0x0107_6000,
            0x0109_9000,
            0x011a_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TWO_POWER_32_01099378, 4_294_967_296.0f32);
        e.set_global(RECT_LEFT_010181BC, 0.05f32);
        e.set_global(RECT_OFFSET_0103C7C8, 0.05f64);
        e.set_global(COLUMN_STEP_01011588, 0.5f64);
        e.set_global(ROW_STEP_01011598, -0.5f64);
        e.set_global(WIDTH_SCALE_0106B168, 0.5f64);
        e.set_global(HEIGHT_SCALE_010760F0, -0.5f64);
    }

    #[test]
    fn fn_00bd1ab0_clears_the_members_and_destroys() {
        let mut e = rig();
        noop(&mut e, &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0xc0);
        let held = [object(&mut e, 1), object(&mut e, 1)];
        e.mem.set_u32(this + 0xac, held[0]);
        e.mem.set_u32(this + 0x6c + 4 * 15, held[1]);
        e.call(0x00bd_1ab0, &args![this]);
        assert_eq!(e.mem.u32(this), DEBUG_SHADER_VTABLE_010BBD08);
        assert_eq!(
            calls(&e, REFERENCE_DELETE),
            vec![vec![held[0]], vec![held[1]]]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0xb4, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 0x10, POINTER_DESTRUCT_0045CEC0],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bd23c0_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(
            &mut e,
            &[VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460],
        );
        let this = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x00bd_23c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xc0]]);
        e.call(0x00bd_23c0, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    /// A debug shader with the screen elements (vtable 0, data at +0xb8), textures (vtable 1)
    /// in the given slots with the given sizes, and a renderer (vtable 2) of mode `mode`.
    fn draw_rig(textures: &[(u32, u32, u32)], mode: u32) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = rig();
        map_globals(&mut e);
        let this = e.mem.alloc(0xc0);
        let table3 = vtable(&mut e, 3);
        e.mem.set_u32(this, table3);
        let screen = e.mem.alloc(0xc4);
        let table0 = vtable(&mut e, 0);
        e.mem.set_u32(screen, table0);
        e.mem.set_u32(screen + 4, 1);
        e.mem.set_u32(screen + 0xb8, 0x5555);
        e.mem.set_u32(this + 0xac, screen);
        let table1 = vtable(&mut e, 1);
        let mut made = vec![];
        for (index, width, height) in textures {
            let texture = object(&mut e, 1);
            e.mem.set_u32(texture, table1);
            e.mem.set_u32(texture + 0x40, *width);
            e.mem.set_u32(texture + 0x44, *height);
            e.mem.set_u32(this + 0x6c + 4 * index, texture);
            made.push(texture);
        }
        e.register(slot(1, 0x94), |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(slot(1, 0x98), |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        e.register(slot(1, 4), |_, _| Ret::default());
        let renderer = e.mem.alloc(0x210);
        let table2 = vtable(&mut e, 2);
        e.mem.set_u32(renderer, table2);
        e.mem.set_u32(renderer + 0x200, mode);
        e.mem.set_u8(renderer + 0x208, 1);
        e.set_global(RENDERER_OBJECT_011F4748, renderer);
        let output = e.mem.alloc(0x10);
        let table4 = vtable(&mut e, 4);
        e.mem.set_u32(output, table4);
        e.register_double(slot(2, 0xc8), move |_, _| output.into_ret());
        e.register(slot(4, 0x8c), |_, _| 800u32.into_ret());
        e.register(slot(4, 0x90), |_, _| 600u32.into_ret());
        noop(
            &mut e,
            &[
                slot(2, 0x190),
                slot(3, 0x50),
                SET_RECTANGLE,
                UPDATE_BOUND,
                SCREEN_ELEMENTS_DRAW,
            ],
        );
        (e, this, screen, made)
    }

    #[test]
    fn fn_00bd1cf0_draws_each_texture_in_its_grid_cell() {
        let (mut e, this, screen, textures) = draw_rig(&[(0, 64, 64), (5, 128, 64)], 1);
        e.call(0x00bd_1cf0, &args![this, 0u32]);
        let rectangles = calls(&e, SET_RECTANGLE);
        assert_eq!(rectangles.len(), 2);
        let floats = |a: &[u32]| {
            a[2..]
                .iter()
                .map(|b| f32::from_bits(*b))
                .collect::<Vec<_>>()
        };
        // Cell 0: column 0, row 0; a square texture: 0.5 by -0.5.
        assert_eq!(rectangles[0][..2], [0x5555, 0]);
        assert_eq!(floats(&rectangles[0]), vec![-1.0, 1.0, 0.5, -0.5]);
        // Cell 5: column 1, row 1; twice as wide: the width ratio 2 / 2 = 1, height 1 / 2.
        assert_eq!(floats(&rectangles[1]), vec![-0.5, 0.5, 0.5, -0.25]);
        // The renderer's slot +0x190 got the screen rectangle (mode 1, byte 1).
        let screen_rect = calls(&e, slot(2, 0x190));
        assert_eq!(screen_rect.len(), 2);
        assert_eq!(calls(&e, SCREEN_ELEMENTS_DRAW).len(), 2);
        assert_eq!(calls(&e, slot(3, 0x50)), vec![vec![this, screen]; 2]);
        // The textures were released and their pointers cleared; the loop index ends at 0x10.
        assert_eq!(
            calls(&e, slot(1, 4)),
            vec![vec![textures[0]], vec![textures[1]]]
        );
        assert_eq!(e.mem.u32(this + 0x6c + 4 * 5), 0);
        assert_eq!(e.mem.u32(this + 0xbc), 0x10);
        assert_eq!(e.mem.u32(screen + 0xc0), 0);
        // The screen elements were held for the loop and let go.
        assert_eq!(e.mem.u32(screen + 4), 1);
    }

    #[test]
    fn fn_00bd1cf0_uses_the_target_size_and_skips_the_renderer_hook() {
        let (mut e, this, _, _) = draw_rig(&[(3, 32, 32)], 3);
        let target = e.mem.alloc(0x40);
        let maps = e.mem.alloc(0x10);
        let table5 = vtable(&mut e, 5);
        e.mem.set_u32(maps, table5);
        e.mem.set_u32(target + 0x30, maps);
        e.register(slot(5, 0x94), |_, _| 100u32.into_ret());
        e.register(slot(5, 0x98), |_, _| 50u32.into_ret());
        e.call(0x00bd_1cf0, &args![this, target]);
        assert!(calls(&e, slot(2, 0x190)).is_empty());
        assert!(calls(&e, slot(2, 0xc8)).is_empty());
        let rectangles = calls(&e, SET_RECTANGLE);
        assert_eq!(f32::from_bits(rectangles[0][2]), 0.5);
        assert_eq!(f32::from_bits(rectangles[0][3]), 1.0);
    }

    #[test]
    fn debug_shader_initialize_builds_the_screen_elements_and_the_mask() {
        const MAP_SLOTS: u32 = 0x0e30_0000;
        const MAP_VTABLE: u32 = 0x0e31_0000;
        let mut e = rig();
        map_globals(&mut e);
        e.set_global(RECT_LEFT_01012054, -1.0f32);
        e.set_global(RECT_WIDTH_010162C0, 2.0f32);
        e.set_global(RECT_HEIGHT_0104A494, -2.0f32);
        e.register(CALL_00BE0870, |_, _| 1u32.into_ret());
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SCREEN_ELEMENTS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, a[1]);
            a[0].into_ret()
        });
        noop(
            &mut e,
            &[
                SCREEN_ELEMENTS_DATA_INSERT,
                SET_RECTANGLE,
                SCREEN_ELEMENTS_DATA_SET_TEXTURES,
                UPDATE_BOUND,
                UPDATE_PROPERTIES,
                UPDATE_00A59C60,
                MAP_SLOTS + 8,
            ],
        );
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xc0);
        assert!(e.call(0x00bd_24c0, &args![this]).bool());
        let elements = e.mem.u32(this + 0xac);
        let data = e.mem.u32(elements + 0xb8);
        assert_eq!(e.mem.u32(elements + 4), 1);
        let construct = calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT);
        assert_eq!(&construct[0][1..], &[0, 0, 1, 1, 1, 4, 1, 2, 1]);
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![data, 4, 0, 0]]
        );
        assert_eq!(
            calls(&e, SET_RECTANGLE),
            vec![vec![
                data,
                0,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                (-2.0f32).to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_TEXTURES),
            vec![vec![data, 0, 0, 0, 0, ONE_BITS, ONE_BITS]]
        );
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![elements]]);
        let update = calls(&e, UPDATE_00A59C60);
        assert_eq!(update[0][0], elements);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0][1], CHANNEL_MASK_010BBE44);
        assert_eq!(entries[0][9], CHANNEL_MASK_011AE620);
    }

    #[test]
    fn debug_shader_initialize_passes_a_null_data_when_its_allocation_fails() {
        let mut e = rig();
        map_globals(&mut e);
        e.register(CALL_00BE0870, |_, _| Ret::default());
        e.register_double(NI_ALLOCATE_00AA13E0, |e, a| {
            if a[0] == 0x70 {
                Ret::default()
            } else {
                e.mem.alloc(a[0]).into_ret()
            }
        });
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            assert_eq!(a[1], 0);
            let data = e.mem.alloc(0x10);
            e.mem.set_u32(a[0] + 0xb8, data);
            a[0].into_ret()
        });
        noop(
            &mut e,
            &[
                SCREEN_ELEMENTS_DATA_INSERT,
                SET_RECTANGLE,
                SCREEN_ELEMENTS_DATA_SET_TEXTURES,
                UPDATE_BOUND,
                UPDATE_PROPERTIES,
                UPDATE_00A59C60,
            ],
        );
        let this = e.mem.alloc(0xc0);
        let map = object(&mut e, 1);
        e.mem.set_u32(this + 0x30, map);
        assert!(!e.call(0x00bd_24c0, &args![this]).bool());
        assert!(calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT).is_empty());
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330).is_empty());
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

    const SLOTS: u32 = 0x0f10_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for offset in (0..0x100u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        for offset in (0..0x100u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00bd1940_runs_load_shaders() {
        use crate::units::bsshader::geometrydecalshader::shader_test_rig::loader_rig;
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0f20_0000u32.into_ret());
        let this = e.mem.alloc(0xd0);
        e.call(0x00bd_1940, &args![this]);
        assert!(!calls(&e, COMPILE_VERTEX_SHADER).is_empty());
    }

    #[test]
    fn fn_00bd1c50_sets_the_texture_and_pixel_shader() {
        let mut e = rig();
        let this = object(&mut e, 0xc0);
        let stage = e.mem.alloc(0x10);
        let stages = e.mem.alloc(4);
        e.mem.set_u32(stages, stage);
        let pass = e.mem.alloc(0x48);
        e.mem.set_u32(pass + 0x24, stages);
        e.mem.set_u32(this + 0x5c, pass);
        e.mem.set_u32(this + 0xbc, 2);
        e.mem.set_u32(this + 0x6c + 8, 0x7878);
        let old = object(&mut e, 8);
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(pass + 0x44, old);
        let new = object(&mut e, 8);
        e.set_global(TEXTURE_INDEX_01202338, 1u32);
        e.mem.set_u32(this + 0xb8, new);
        let answer = e
            .call(
                0x00bd_1c50,
                &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(calls(&e, SLOTS + 0xec), vec![vec![this]]);
        assert_eq!(e.mem.u32(stage + 8), 0x7878);
        assert_eq!(e.mem.u32(pass + 0x44), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![old]]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), pass);
    }

    #[test]
    fn set_texture_to_render_fills_the_first_empty_slot() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x6c, 1);
        let base_rtti = e.mem.alloc(8);
        e.mem.set_u32(base_rtti + 4, RTTI_011F4AC4);
        e.map(0x011f_4000, 0x1000);
        e.set_global(0x011f_4f00, base_rtti);
        e.register(SLOTS + 8, |e, _| e.mem.u32(0x011f_4f00).into_ret());
        let texture = object(&mut e, 8);
        e.call(0x00bd_22c0, &args![this, texture]);
        assert_eq!(e.mem.u32(this + 0x70), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        // A texture of another kind is not taken.
        e.mem.set_u32(base_rtti + 4, 0);
        let other = object(&mut e, 8);
        e.call(0x00bd_22c0, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        // All slots full: nothing.
        e.mem.set_u32(base_rtti + 4, RTTI_011F444C);
        for index in 0..16u32 {
            e.mem.set_u32(this + 0x6c + 4 * index, 9);
        }
        e.call(0x00bd_22c0, &args![this, other]);
        assert_eq!(e.mem.u32(other + 4), 0);
    }
}
