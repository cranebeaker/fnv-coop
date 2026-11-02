//! `BSShader/run_00bc4340` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bc4830` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BC4830: u32 = 0x00bc_4830;

// Translated from 00bc4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x1a. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`, 5 entries, 1 stream), sets its entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1,
/// 0)) and calls slot +0xb4. Then allocates 0xc0 bytes and runs the constructor `00bc4830`
/// with the declaration (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x140, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bc4af0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bc4af0(e: &mut Engine) -> Ptr {
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
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BC4830, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x140, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bad98`; no RTTI locator before it).
const SHADER_VTABLE_010BAD98: u32 = 0x010b_ad98;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00bc4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the constructor of shader 0x1a, `thiscall` with
/// one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`), stores the vtable
/// `010bad98`, clears the words +0xb4, +0xb8, +0xbc, sets +0x68 to 0x10, the byte +0x24 to 1
/// and +0x6c to 0x243; then the pass pointer at +0xb4 is cleared (its count at +0x64 dropped,
/// `00e80190` at 0), the declaration assigned to the `NiPointer` at +0x28, the `NiPointer`s
/// at +0xb8 and +0xbc cleared (each just set to 0, so these releases find nothing), 0.0
/// stored at +0x70 and 0x40 bytes from +0x74 cleared (`memset`). Answers `this`.
// Translation incomplete: 00bc4830 - the C++ exception frame (FS:[0] chain, unwind state 3 for the base and the pointers) is not translated.
pub fn fn_00bc4830(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_VTABLE_010BAD98);
    e.mem.set_u32(base + 0xb4, 0);
    e.mem.set_u32(base + 0xb8, 0);
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0x68, 0x10);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x243);
    pass_pointer_clear(e, base + 0xb4);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0xb8);
    ni_pointer_clear(e, base + 0xbc);
    e.mem.set_f32(base + 0x70, 0.0);
    e.call(MEMSET, &args![base + 0x74, 0u32, 0x40u32]);
    this
}

/// `00be0870` (unnamed, `thiscall`, no stack words).
const CALL_00BE0870: u32 = 0x00be_0870;

// Translated from 00bc4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (the vtables `010b80a8` and `010bad98` slot +0x4c): a 5-byte jump to
/// `00be0870`, passing on `this`; answers what it answers.
pub fn fn_00bc4600(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALL_00BE0870, &args![this]).u32()
}

/// The global `NiPointer` `00bc4610` clears (`01201fc4`).
const GLOBAL_POINTER_01201FC4: u32 = 0x0120_1fc4;

// Translated from 00bc4610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`), no arguments: clears the global `NiPointer` at
/// `01201fc4` (releasing what it holds).
pub fn fn_00bc4610(e: &mut Engine) {
    ni_pointer_clear(e, GLOBAL_POINTER_01201FC4);
}

/// Strings of the beam loader (read in Ghidra): `"beam\\v\\beam.v.hlsl"` (`010bad80`),
/// `"BEAM.vso"` (`010bad74`), `"beam\\p\\beam.p.hlsl"` (`010bad60`), `"BEAM.pso"`
/// (`010bad54`), the vertex profile `"vs_1_1"` (`010a93ec`); the pixel profile chooser
/// `00b4f380` (`cdecl`, a byte).
const BEAM_VERTEX_010BAD80: u32 = 0x010b_ad80;
const BEAM_VSO_010BAD74: u32 = 0x010b_ad74;
const BEAM_PIXEL_010BAD60: u32 = 0x010b_ad60;
const BEAM_PSO_010BAD54: u32 = 0x010b_ad54;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// The beam loader's frame (saved registers up to the cookie) and buffers.
const BEAM_FRAME_BYTES: u32 = 0x2b4;
const BEAM_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xa8,
    path: 0x1ac,
};

// Translated from 00bc4680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bad98` slot +0x13c): the beam shader's `LoadShaders`, no
/// stack words. Fills the vertex block (`beam.v.hlsl`, no macros, the rest cleared by
/// `memset`) and compiles it (`MakePath`, the name `"BEAM.vso"`, `vs_1_1`) into the `NiPointer`
/// +0xb8; then takes the pixel profile (`00b4f380(1)`), fills the pixel block
/// (`beam.p.hlsl`) and compiles it (`"BEAM.pso"`) into +0xbc.
// Translation incomplete: 00bc4680 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bc4680(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(BEAM_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(
            e,
            frame,
            &[ShaderSource {
                offset: 0x5c,
                words: &[BEAM_VERTEX_010BAD80, 0, 0],
                cleared: 0x40,
            }],
        );
        let vertex = ShaderCompile {
            source: 0x5c,
            output: BEAM_VSO_010BAD74,
            profile: PROFILE_VS_1_1_010A93EC,
            pixel: false,
            slot: 0xb8,
        };
        let shader = compile_shader(e, this, frame, &BEAM_BUFFERS, &vertex);
        ni_pointer_assign(e, this + vertex.slot, shader);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        set_up_sources(
            e,
            frame,
            &[ShaderSource {
                offset: 0x10,
                words: &[BEAM_PIXEL_010BAD60, 0, 0],
                cleared: 0x40,
            }],
        );
        let pixel = ShaderCompile {
            source: 0x10,
            output: BEAM_PSO_010BAD54,
            profile,
            pixel: true,
            slot: 0xbc,
        };
        let shader = compile_shader(e, this, frame, &BEAM_BUFFERS, &pixel);
        ni_pointer_assign(e, this + pixel.slot, shader);
    });
}

/// `BSShader::~BSShader` (engine map, `00be0f10`, no stack words) and the Ni free
/// (`00aa1460`, `cdecl`: block, size).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bc49a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the beam shader's destructor, no stack words.
/// Stores the vtable `010bad98`, clears the global `NiPointer` `01201fc4`, drops and clears
/// the pass +0xb4, clears the `NiPointer`s +0xb8 and +0xbc (releasing what they hold); then
/// the members' own destructors release +0xbc and +0xb8 and drop the pass +0xb4 again when
/// they are not null (they were just cleared), and `BSShader::~BSShader` (`00be0f10`) runs.
// Translation incomplete: 00bc49a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the members and the base) is not translated.
pub fn fn_00bc49a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_VTABLE_010BAD98);
    ni_pointer_clear(e, GLOBAL_POINTER_01201FC4);
    pass_pointer_clear(e, base + 0xb4);
    ni_pointer_clear(e, base + 0xb8);
    ni_pointer_clear(e, base + 0xbc);
    for offset in [0xbcu32, 0xb8] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_reference(e, held);
        }
    }
    let pass = e.mem.u32(base + 0xb4);
    if pass != 0 {
        let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
        e.mem.set_u32(pass + 0x64, left);
        if left == 0 {
            e.call(PASS_DELETE_00E80190, &args![pass]);
        }
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The stage call `00e890c0` (unit `NiXenonRenderer`, unnamed; `thiscall` on the stage's
/// +0x10, four words).
const STAGE_CALL_00E890C0: u32 = 0x00e8_90c0;
/// The beam pass's render states, in order.
const BEAM_RENDER_STATES: [(u32, u32); 10] = [
    (0x16, 1),
    (0xa8, 0xf),
    (0x1b, 1),
    (0x13, 5),
    (0x14, 6),
    (0xf, 0),
    (0x34, 0),
    (7, 1),
    (0x17, 4),
    (0xe, 0),
];

// Translated from 00bc4c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the beam pass, no stack words. Holds the pass
/// +0xb4 (its count +0x64 up by one when not null; the code goes on with it even when null).
/// When the pass has no stage yet (word +0x18 below 1): a stage (`CreateNewStage` into a
/// stack pointer cleared first), `SetupDefaultStage(stage, 0, 1, 2, 1)`, `00e890c0` on the
/// stage's +0x10 with (7, 0, 0, 0) and `NiD3DPass::SetStage` with the pass's word +0x14. The
/// pass's vertex (+0x5c) and pixel (+0x44) `NiPointer`s take the shader's +0xb8 and +0xbc;
/// the render states (0x16, 1), (0xa8, 0xf), (0x1b, 1), (0x13, 5), (0x14, 6), (0xf, 0),
/// (0x34, 0), (7, 1), (0x17, 4), (0xe, 0) are set on it; then the stage reference is dropped
/// and the pass let go (count down by one, `00e80190` at 0, without a null check).
// Translation incomplete: 00bc4c00 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage and the pass) is not translated.
pub fn fn_00bc4c00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        let pass = e.mem.u32(base + 0xb4);
        if pass != 0 {
            let count = e.mem.u32(pass + 0x64).wrapping_add(1);
            e.mem.set_u32(pass + 0x64, count);
        }
        if e.mem.u32(pass + 0x18) < 1 {
            e.call(CREATE_NEW_STAGE, &args![local]);
            let stage = e.mem.u32(local);
            e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 1u32, 2u32, 1u32]);
            let stage = e.mem.u32(local);
            let target = e.mem.u32(stage + 0x10);
            e.call(STAGE_CALL_00E890C0, &args![target, 7u32, 0u32, 0u32, 0u32]);
            let stage = e.mem.u32(local);
            let position = e.mem.u32(pass + 0x14);
            e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        }
        let vertex = e.mem.u32(base + 0xb8);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pixel = e.mem.u32(base + 0xbc);
        ni_pointer_assign(e, pass + 0x44, pixel);
        for (state, value) in BEAM_RENDER_STATES {
            pass_render_state(e, pass, state, value);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
        let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
        e.mem.set_u32(pass + 0x64, left);
        if left == 0 {
            e.call(PASS_DELETE_00E80190, &args![pass]);
        }
    });
}

// Translated from 00bc4e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the beam shader's scalar deleting destructor
/// (vtable `010bad98` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00bc49a0`) and,
/// when bit 0 of the flags is set, frees the 0xc0 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bc4e70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc49a0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xc0u32]);
    }
    this
}

// Translated from 00bc4ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`; vtable `010bad98` slot +0x140), no stack words.
/// Gives the pass pointer +0xb4 a new pass (`00e804d0`) when it is null, sets the pass up
/// (`00bc4c00`) and answers true.
pub fn fn_00bc4ea0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if e.mem.u32(base + 0xb4) == 0 {
        e.call(NEW_PASS_00E804D0, &args![base + 0xb4]);
    }
    fn_00bc4c00(e, this);
    true
}

/// The constant names of the beam `CreateConstantMaps` (read in Ghidra).
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const FVARS_010AF454: u32 = 0x010a_f454;

// Translated from 00bc4ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`; vtable `010bad98` slot +0x11c): the beam
/// shader's `CreateConstantMaps`, no stack words. When the vertex constant map (+0x34) is
/// null, makes one (kind 0) with `"WorldViewProjTranspose"` (0x20000009, 0, 0, 4) and
/// `"fVars"` (0x10000009, register 4, 4 registers, 0x40 bytes at +0x74); when the pixel one
/// (+0x30) is null, makes one (kind 2) without entries.
// Translation incomplete: 00bc4ed0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn fn_00bc4ed0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
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
        add_constant_entry(
            e,
            base,
            0x34,
            [
                FVARS_010AF454,
                0x1000_0009,
                0,
                4,
                4,
                CONSTANT_EMPTY_NAME_01011584,
                0x40,
                4,
                base + 0x74,
                0,
            ],
        );
    }
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
    }
}

// Translated from 00bc5030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`; vtable `010baf60` slot +0x13c): runs the
/// shader's slot +0x140 and then, as a tail jump, its slot +0x144, answering what that one
/// answers. No stack words.
pub fn fn_00bc5030(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.vcall(this, 0x140, &args![]);
    e.vcall(this, 0x144, &args![]).u32()
}

/// Strings of the volumetric fog loaders (read in Ghidra): `"vfog\\VolumetricFog.v.hlsl"`
/// (`010baf00`), `"vfog\\VolumetricFog.p.hlsl"` (`010baf40`), the macro `"SINGLE_PASS"`
/// (`010baf34`), the formats `"VolumetricFog%03d.vso"` (`010baee8`) and
/// `"VolumetricFog%03d.pso"` (`010baf1c`), the profiles `"vs_2_0"` (`010a93e4`), `"vs_3_0"`
/// (`010a93dc`), `"ps_2_a"` (`010a93c4`) and `"ps_3_0"` (`010a93b4`), and the byte that turns
/// the second shader of each on (`011ad80e`).
const FOG_VERTEX_010BAF00: u32 = 0x010b_af00;
const FOG_PIXEL_010BAF40: u32 = 0x010b_af40;
const MACRO_SINGLE_PASS_010BAF34: u32 = 0x010b_af34;
const FORMAT_FOG_VERTEX_010BAEE8: u32 = 0x010b_aee8;
const FORMAT_FOG_PIXEL_010BAF1C: u32 = 0x010b_af1c;
const PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
const PROFILE_VS_3_0_010A93DC: u32 = 0x010a_93dc;
const PROFILE_PS_2_A_010A93C4: u32 = 0x010a_93c4;
const PROFILE_PS_3_0_010A93B4: u32 = 0x010a_93b4;
const SECOND_SHADERS_011AD80E: u32 = 0x011a_d80e;
const FOG_EMPTY_01011584: u32 = 0x0101_1584;
/// The volumetric fog loaders' frame (saved registers up to the cookie) and buffers.
const FOG_FRAME_BYTES: u32 = 0x2bc;
const FOG_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xb0,
    path: 0x1b4,
};

/// The two volumetric fog compiles: block `index` (0x4c bytes apart from +0x18), the second
/// only with the byte `011ad80e` set, each with its own profile, into `slot + 4 * index`.
fn compile_fog_pair(e: &mut Engine, this: u32, frame: u32, format: u32, slot: u32, pixel: bool) {
    for index in 0..2u32 {
        if index == 1 && e.global::<u8>(SECOND_SHADERS_011AD80E) == 0 {
            continue;
        }
        let profile = e.mem.u32(frame + 0x10 + 4 * index);
        let shader = compile_indexed(
            e,
            this,
            frame,
            &FOG_BUFFERS,
            0x18 + 0x4c * index,
            format,
            index,
            profile,
            pixel,
        );
        ni_pointer_assign(e, this + slot + 4 * index, shader);
    }
}

// Translated from 00bc5070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010baf60` slot +0x140): the volumetric fog vertex shaders,
/// no stack words. Two blocks of `VolumetricFog.v.hlsl` without macros (the rest cleared by
/// `memset`) with the profiles `vs_2_0` and `vs_3_0`; each is compiled (`MakePath`,
/// `"VolumetricFog%03d.vso"` with the index) into the `NiPointer`s +0x78 and +0x7c, the second
/// only when the byte `011ad80e` is set.
// Translation incomplete: 00bc5070 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bc5070(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(FOG_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, PROFILE_VS_2_0_010A93E4);
        e.mem.set_u32(frame + 0x14, PROFILE_VS_3_0_010A93DC);
        set_up_sources(
            e,
            frame,
            &[
                ShaderSource {
                    offset: 0x18,
                    words: &[FOG_VERTEX_010BAF00, 0],
                    cleared: 0x44,
                },
                ShaderSource {
                    offset: 0x64,
                    words: &[FOG_VERTEX_010BAF00, 0],
                    cleared: 0x44,
                },
            ],
        );
        compile_fog_pair(e, this, frame, FORMAT_FOG_VERTEX_010BAEE8, 0x78, false);
    });
}

// Translated from 00bc51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010baf60` slot +0x144): the volumetric fog pixel shaders, no
/// stack words. `VolumetricFog.p.hlsl` without macros (profile `ps_2_a`) and with
/// `SINGLE_PASS` (`ps_3_0`), each compiled (`"VolumetricFog%03d.pso"` with the index) into the
/// `NiPointer`s +0x80 and +0x84, the second only when the byte `011ad80e` is set.
// Translation incomplete: 00bc51a0 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bc51a0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(FOG_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, PROFILE_PS_2_A_010A93C4);
        e.mem.set_u32(frame + 0x14, PROFILE_PS_3_0_010A93B4);
        set_up_sources(
            e,
            frame,
            &[
                ShaderSource {
                    offset: 0x18,
                    words: &[FOG_PIXEL_010BAF40, 0],
                    cleared: 0x44,
                },
                ShaderSource {
                    offset: 0x64,
                    words: &[
                        FOG_PIXEL_010BAF40,
                        MACRO_SINGLE_PASS_010BAF34,
                        FOG_EMPTY_01011584,
                        0,
                    ],
                    cleared: 0x3c,
                },
            ],
        );
        compile_fog_pair(e, this, frame, FORMAT_FOG_PIXEL_010BAF1C, 0x80, true);
    });
}

/// The property constructor `00bc3bb0` (unnamed, `thiscall`) and its copy `00bc3db0`
/// (unnamed, `thiscall`, two words).
const PROPERTY_CONSTRUCT_00BC3BB0: u32 = 0x00bc_3bb0;
const PROPERTY_COPY_00BC3DB0: u32 = 0x00bc_3db0;

// Translated from 00bc4340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc4340 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
/// Unnamed in the engine map: the creator `RegisterStreamables` registers under the name at
/// `010a9338` ("HairShaderProperty"), no arguments. Allocates 0x180 bytes (`00aa13e0`) and
/// answers what `00bc3bb0` answers for them, or null when the allocation fails.
pub fn fn_00bc4340(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x180u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(PROPERTY_CONSTRUCT_00BC3BB0, &args![block]).u32()
}

// Translated from 00bc43b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc43b0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
/// Unnamed in the engine map (vtable entry at `010bac40`), `thiscall` with one word: the clone.
/// With the thread's allocation context (+0x2b4) at 0x28, allocates 0x180 bytes, runs
/// `00bc3bb0` on them when that succeeds, and copies `this` into the result
/// (`00bc3db0(result, word)`, a null result passed as it is). Answers the result.
pub fn fn_00bc43b0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    with_allocation_context(e, 0x28, |e| {
        let block = e.call(NI_ALLOCATE, &args![0x180u32]).u32();
        let made = if block != 0 {
            e.call(PROPERTY_CONSTRUCT_00BC3BB0, &args![block]).u32()
        } else {
            0
        };
        e.call(PROPERTY_COPY_00BC3DB0, &args![this, made, word]);
        made
    })
}

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_FLAG,
};

/// `BSShaderPPLightingProperty::GetViewerStrings` (`00b690d0`), the pointer viewer string
/// `00a7d5a0` (`cdecl`, label and word), the property's RTTI record (`01201fb8`) and the labels
/// `010bad48` (the word +0x17c) and `010bad30`, `010bad20`, `010bad14` (bits 7, 8 and 9 of +0x24).
const PP_LIGHTING_VIEWER_STRINGS: u32 = 0x00b6_90d0;
const VIEWER_STRING_POINTER_00A7D5A0: u32 = 0x00a7_d5a0;
const RTTI_01201FB8: u32 = 0x0120_1fb8;
const LABEL_010BAD48: u32 = 0x010b_ad48;
const FLAG_LABELS: [(u32, u32); 3] = [
    (0x80, 0x010b_ad30),
    (0x100, 0x010b_ad20),
    (0x200, 0x010b_ad14),
];

// Translated from 00bc4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bac58`): the
/// property's `GetViewerStrings`, `thiscall` with one word, the string array. Runs
/// `BSShaderPPLightingProperty::GetViewerStrings`, then adds (the inlined `NiTArray` add) the
/// class name (`00a7d330` of `01201fb8`), `00a7d5a0(010bad48, +0x17c)` and for bits 7, 8 and 9
/// of +0x24 `00a7d370(label, bit set)`. The flag word is the stack slot of the last string with
/// its low byte replaced (`SETA`, byte store), as the exe pushes it.
pub fn fn_00bc4460(e: &mut Engine, this: Ptr, strings: u32) {
    let base = this.addr();
    e.call(PP_LIGHTING_VIEWER_STRINGS, &args![base, strings]);
    let rtti = e.global::<u32>(RTTI_01201FB8);
    let mut slot = e.call(VIEWER_STRING, &args![rtti]).u32();
    viewer_strings_add(e, strings, slot);
    let value = e.mem.u32(base + 0x17c);
    slot = e
        .call(
            VIEWER_STRING_POINTER_00A7D5A0,
            &args![LABEL_010BAD48, value],
        )
        .u32();
    viewer_strings_add(e, strings, slot);
    for (bit, label) in FLAG_LABELS {
        let set = e.mem.u32(base + 0x24) & bit != 0;
        let word = (slot & 0xffff_ff00) | u32::from(set);
        slot = e.call(VIEWER_STRING_FLAG, &args![label, word]).u32();
        viewer_strings_add(e, strings, slot);
    }
}

/// `BSShaderPPLightingProperty::SetupGeometry` (`00b67a40`, one word), `NiAVObject::
/// GetProperty` (`00a59d30`, one word) and `00654640` (unnamed, one word); all `thiscall`.
const PP_LIGHTING_SETUP_GEOMETRY: u32 = 0x00b6_7a40;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const USE_PROPERTY_00654640: u32 = 0x0065_4640;

// Translated from 00bc45d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bac94`), `thiscall`
/// with one word, a geometry: runs `BSShaderPPLightingProperty::SetupGeometry` (`this` passed on
/// in `ECX`), then hands the geometry's property 10 (`GetProperty`), when it has one, to
/// `00654640`. Answers what `SetupGeometry` answered.
pub fn fn_00bc45d0(e: &mut Engine, this: Ptr, geometry: u32) -> bool {
    let result = e
        .call(PP_LIGHTING_SETUP_GEOMETRY, &args![this, geometry])
        .bool();
    let property = e.call(GET_PROPERTY, &args![geometry, 10u32]).u32();
    if property != 0 {
        e.call(USE_PROPERTY_00654640, &args![geometry, property]);
    }
    result
}

/// `00e814b0` (unnamed, `thiscall`, no stack words).
const RENDERER_CALL_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00bc4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bae68`), `thiscall`, no stack words: with the
/// word +0x28 set, calls the vtable +0xa0 of the object +0xb8 and +0x88 of the object +0xbc;
/// then jumps to `00e814b0`.
pub fn fn_00bc4640(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        let object = e.mem.u32(base + 0xb8);
        e.vcall(object, 0xa0, &args![]);
        let object = e.mem.u32(base + 0xbc);
        e.vcall(object, 0x88, &args![]);
    }
    e.call(RENDERER_CALL_00E814B0, &args![base]);
}

// Translated from 00bc4810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010baebc`), `thiscall`,
/// no stack words: calls the shader's vtable +0x128 with its word +0xb4.
pub fn fn_00bc4810(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pass = e.mem.u32(base + 0xb4);
    e.vcall(base, 0x128, &args![pass]);
}

/// The record `00bc4970` answers (`01201fc8`).
const RECORD_01201FC8: u32 = 0x0120_1fc8;

// Translated from 00bc4970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bada0`, `ECX` unread):
/// answers the address `01201fc8`.
pub fn fn_00bc4970(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RECORD_01201FC8)
}

// Translated from 00bc4980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010b81e0`, `010ba4e8`,
/// `010baed0`), `thiscall` with two words: stores the first at +0x6c and the low byte of the
/// second at +0x64.
pub fn fn_00bc4980(e: &mut Engine, this: Ptr, word: u32, flag: u32) {
    e.mem.set_u32(this.addr() + 0x6c, word);
    e.mem.set_u8(this.addr() + 0x64, flag as u8);
}

// Translated from 00bc5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entries at `010b8fa0`, `010bb080`), `thiscall`, no stack
/// words: calls the vtable +0x13c, then jumps through the vtable +0x148.
pub fn fn_00bc5050(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x13c, &args![]);
    e.vcall(this.addr(), 0x148, &args![]);
}

// Translated from 00bc52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bb084`), `thiscall`, no stack words: calls the
/// shader's vtable +0x128 with each of the three pass pointers from +0x6c, in order.
pub fn fn_00bc52f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..3 {
        let pass = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![pass]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_4340, fn_00bc4340() -> u32),
        entry!(0x00bc_43b0, fn_00bc43b0(Ptr, u32) -> u32),
        entry!(0x00bc_4460, fn_00bc4460(Ptr, u32)),
        entry!(0x00bc_45d0, fn_00bc45d0(Ptr, u32) -> bool),
        entry!(0x00bc_4640, fn_00bc4640(Ptr)),
        entry!(0x00bc_4810, fn_00bc4810(Ptr)),
        entry!(0x00bc_4970, fn_00bc4970(Ptr) -> Ptr),
        entry!(0x00bc_4980, fn_00bc4980(Ptr, u32, u32)),
        entry!(0x00bc_5050, fn_00bc5050(Ptr)),
        entry!(0x00bc_52f0, fn_00bc52f0(Ptr)),
        entry!(0x00bc_4830, fn_00bc4830(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_4af0, fn_00bc4af0() -> Ptr),
        entry!(0x00bc_4600, fn_00bc4600(Ptr) -> u32),
        entry!(0x00bc_4610, fn_00bc4610()),
        entry!(0x00bc_4680, fn_00bc4680(Ptr)),
        entry!(0x00bc_49a0, fn_00bc49a0(Ptr)),
        entry!(0x00bc_4c00, fn_00bc4c00(Ptr)),
        entry!(0x00bc_4e70, fn_00bc4e70(Ptr, u32) -> Ptr),
        entry!(0x00bc_4ea0, fn_00bc4ea0(Ptr) -> bool),
        entry!(0x00bc_4ed0, fn_00bc4ed0(Ptr)),
        entry!(0x00bc_5030, fn_00bc5030(Ptr) -> u32),
        entry!(0x00bc_5070, fn_00bc5070(Ptr)),
        entry!(0x00bc_51a0, fn_00bc51a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bc4af0_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BC4830, &[]);
        let (shader, log) = run(&mut e, 0x00bc_4af0);
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
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xc0]),
                (SHADER_CONSTRUCT_00BC4830, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
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
    fn fn_00bc4830_builds_the_shader() {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xcc; 0xc0]);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x28, old);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_4830, &args![this, declaration]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BAD98);
        assert_eq!(e.mem.u32(this + 0x68), 0x10);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x243);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        for offset in [0xb4u32, 0xb8, 0xbc] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x74, 0, 0x40]]);
    }
}

#[cfg(test)]
mod beam_and_fog_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const SLOTS: u32 = 0x0e00_0000;
    const VTABLE: u32 = 0x0e80_0000;

    fn vtable(e: &mut Engine) -> u32 {
        let targets: Vec<u32> = (0..0x60u32).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(VTABLE, &targets);
        VTABLE
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00bc4600_jumps_to_00be0870() {
        let mut e = rig();
        e.register(CALL_00BE0870, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x00bc_4600, &args![0x1000u32]).u32(), 0x1001);
    }

    #[test]
    fn fn_00bc4610_clears_the_global_pointer() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        let held = object(&mut e, 1);
        e.set_global(GLOBAL_POINTER_01201FC4, held);
        e.call(0x00bc_4610, &args![]);
        assert_eq!(e.global::<u32>(GLOBAL_POINTER_01201FC4), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        // Already clear: nothing.
        e.call(0x00bc_4610, &args![]);
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 1);
    }

    #[test]
    fn fn_00bc4680_compiles_the_beam_shaders() {
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e55_0000u32.into_ret());
        let this = e.mem.alloc(0xc0);
        e.call(0x00bc_4680, &args![this]);
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0xb8)),
            Compiled {
                file: BEAM_VERTEX_010BAD80,
                output: BEAM_VSO_010BAD74,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                macros: macro_words(&[]),
            }
        );
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0xbc)),
            Compiled {
                file: BEAM_PIXEL_010BAD60,
                output: BEAM_PSO_010BAD54,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[]),
            }
        );
        // The pixel profile is asked for after the vertex compile.
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == COMPILE_VERTEX_SHADER || *a == PIXEL_PROFILE_00B4F380)
            .collect();
        assert_eq!(order, vec![COMPILE_VERTEX_SHADER, PIXEL_PROFILE_00B4F380]);
    }

    #[test]
    fn fn_00bc49a0_clears_the_members_and_destroys() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        noop(&mut e, &[BS_SHADER_DESTRUCT]);
        let this = e.mem.alloc(0xc0);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0xb4, pass);
        let vertex = object(&mut e, 1);
        e.mem.set_u32(this + 0xb8, vertex);
        e.call(0x00bc_49a0, &args![this]);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BAD98);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex]]);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bc4e70_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        noop(&mut e, &[BS_SHADER_DESTRUCT, NI_FREE_00AA1460]);
        let this = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x00bc_4e70, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xc0]]);
        e.call(0x00bc_4e70, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bc4ea0_makes_the_pass_once_and_sets_it_up() {
        let mut e = pass_rig();
        noop(&mut e, &[STAGE_CALL_00E890C0]);
        let this = e.mem.alloc(0xc0);
        let shaders = [object(&mut e, 1), object(&mut e, 1)];
        e.mem.set_u32(this + 0xb8, shaders[0]);
        e.mem.set_u32(this + 0xbc, shaders[1]);
        assert!(e.call(0x00bc_4ea0, &args![this]).bool());
        let pass = e.mem.u32(this + 0xb4);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0xb4]]);
        let setups = calls(&e, SETUP_DEFAULT_STAGE);
        assert_eq!(&setups[0][1..], &[0, 1, 2, 1]);
        let stage = calls(&e, PASS_SET_STAGE)[0][2];
        assert_eq!(
            calls(&e, STAGE_CALL_00E890C0),
            vec![vec![e.mem.u32(stage + 0x10), 7, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(pass + 0x5c), shaders[0]);
        assert_eq!(e.mem.u32(pass + 0x44), shaders[1]);
        assert_eq!(render_states(&e, pass), BEAM_RENDER_STATES.to_vec());
        // The pass was held and let go again; the stage is held by the pass only.
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        // Again: no new pass, and with its stage count +0x18 set no new stage.
        e.mem.set_u32(pass + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bc_4ea0, &args![this]);
        assert!(calls(&e, NEW_PASS_00E804D0).is_empty());
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert_eq!(render_states(&e, pass).len(), 10);
    }

    #[test]
    fn fn_00bc4c00_deletes_a_pass_it_held_alone() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xc0);
        let pass = e.mem.alloc(0x70);
        // Count 0 and a stage already: held to 1, let go to 0, deleted.
        e.mem.set_u32(pass + 0x18, 1);
        e.mem.set_u32(this + 0xb4, pass);
        e.call(0x00bc_4c00, &args![this]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
    }

    #[test]
    fn fn_00bc4ed0_makes_the_beam_maps() {
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
        let this = e.mem.alloc(0xc0);
        e.call(0x00bc_4ed0, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        assert_ne!(e.mem.u32(this + 0x30), 0);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[1],
            vec![
                vertex,
                FVARS_010AF454,
                0x1000_0009,
                0,
                4,
                4,
                CONSTANT_EMPTY_NAME_01011584,
                0x40,
                4,
                this + 0x74,
                0
            ]
        );
        let maps = calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330);
        assert_eq!((maps[0][2], maps[1][2]), (0, 2));
    }

    #[test]
    fn fn_00bc5030_runs_both_loaders() {
        let mut e = rig();
        let table = vtable(&mut e);
        noop(&mut e, &[SLOTS + 0x140]);
        e.register(SLOTS + 0x144, |_, _| 0x44u32.into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, table);
        assert_eq!(e.call(0x00bc_5030, &args![this]).u32(), 0x44);
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(order, vec![SLOTS + 0x140, SLOTS + 0x144]);
    }

    fn fog_load(addr: u32, second: u8) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011a_d000, 0x1000);
        e.set_global(SECOND_SHADERS_011AD80E, second);
        let this = e.mem.alloc(0x90);
        e.call(addr, &args![this]);
        (e, this)
    }

    #[test]
    fn fn_00bc5070_compiles_the_fog_vertex_shaders() {
        let (e, this) = fog_load(0x00bc_5070, 1);
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x78)),
            Compiled {
                file: FOG_VERTEX_010BAF00,
                output: FORMAT_FOG_VERTEX_010BAEE8,
                profile: PROFILE_VS_2_0_010A93E4,
                pixel: false,
                macros: macro_words(&[]),
            }
        );
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x7c)).profile,
            PROFILE_VS_3_0_010A93DC
        );
        let (e, this) = fog_load(0x00bc_5070, 0);
        assert_eq!(e.mem.u32(this + 0x7c), 0);
        assert_eq!(calls(&e, COMPILE_VERTEX_SHADER).len(), 1);
    }

    #[test]
    fn fn_00bc51a0_compiles_the_fog_pixel_shaders() {
        let (e, this) = fog_load(0x00bc_51a0, 1);
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x80)).profile,
            PROFILE_PS_2_A_010A93C4
        );
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x84)),
            Compiled {
                file: FOG_PIXEL_010BAF40,
                output: FORMAT_FOG_PIXEL_010BAF1C,
                profile: PROFILE_PS_3_0_010A93B4,
                pixel: true,
                macros: macro_words(&[(MACRO_SINGLE_PASS_010BAF34, FOG_EMPTY_01011584)]),
            }
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, vec![0, 1]);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::rig;
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, order, slot_target, vobject};

    #[test]
    fn fn_00bc4340_and_00bc43b0_create_and_clone() {
        let mut e = rig();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PROPERTY_CONSTRUCT_00BC3BB0, |_, a| (a[0] + 1).into_ret());
        e.register(PROPERTY_COPY_00BC3DB0, |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + 0x2b4), 0x28);
            Ret::default()
        });
        let made = e.call(0x00bc_4340, &args![]).u32();
        let block = calls(&e, PROPERTY_CONSTRUCT_00BC3BB0)[0][0];
        assert_eq!(made, block + 1);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x180]]);
        let clone = e.call(0x00bc_43b0, &args![0x500u32, 3u32]).u32();
        assert_eq!(
            calls(&e, PROPERTY_COPY_00BC3DB0),
            vec![vec![0x500, clone, 3]]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00bc_4340, &args![]).u32(), 0);
        assert_eq!(e.call(0x00bc_43b0, &args![0x500u32, 3u32]).u32(), 0);
        assert_eq!(calls(&e, PROPERTY_COPY_00BC3DB0)[1], vec![0x500, 0, 3]);
    }

    #[test]
    fn fn_00bc4460_adds_the_strings_and_flags() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.set_global(RTTI_01201FB8, 0xfb8u32);
        e.register(PP_LIGHTING_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, _| 0x1111_1100u32.into_ret());
        e.register(VIEWER_STRING_POINTER_00A7D5A0, |_, _| {
            0x2222_2200u32.into_ret()
        });
        e.register(VIEWER_STRING_FLAG, |_, a| (a[0] << 8).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x20 + 4 * a[1], word);
            Ret::default()
        });
        let strings = e.mem.alloc(0x40);
        e.mem.set_u16(strings + 8, 8);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x17c, 0x17c);
        e.mem.set_u32(this + 0x24, 0x280);
        e.call(0x00bc_4460, &args![this, strings]);
        assert_eq!(
            calls(&e, PP_LIGHTING_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_POINTER_00A7D5A0),
            vec![vec![LABEL_010BAD48, 0x17c]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_FLAG),
            vec![
                vec![0x010b_ad30, 0x2222_2201],
                vec![0x010b_ad20, 0x0bad_3000],
                vec![0x010b_ad14, 0x0bad_2001]
            ]
        );
        let added: Vec<u32> = (0..5).map(|i| e.mem.u32(strings + 0x20 + 4 * i)).collect();
        assert_eq!(
            added,
            vec![
                0x1111_1100,
                0x2222_2200,
                0x0bad_3000,
                0x0bad_2000,
                0x0bad_1400
            ]
        );
    }

    #[test]
    fn fn_00bc45d0_sets_up_then_uses_property_10() {
        let mut e = rig();
        e.register(PP_LIGHTING_SETUP_GEOMETRY, |_, _| 1u32.into_ret());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(USE_PROPERTY_00654640, |_, _| Ret::default());
        assert!(e.call(0x00bc_45d0, &args![0x10u32, 0x20u32]).bool());
        assert_eq!(
            calls(&e, PP_LIGHTING_SETUP_GEOMETRY),
            vec![vec![0x10, 0x20]]
        );
        assert_eq!(calls(&e, GET_PROPERTY), vec![vec![0x20, 10]]);
        assert!(calls(&e, USE_PROPERTY_00654640).is_empty());
        e.register(GET_PROPERTY, |_, _| 0x99u32.into_ret());
        e.register(PP_LIGHTING_SETUP_GEOMETRY, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00bc_45d0, &args![0x10u32, 0x20u32]).bool());
        assert_eq!(calls(&e, USE_PROPERTY_00654640), vec![vec![0x20, 0x99]]);
    }

    #[test]
    fn the_slot_callers() {
        let mut e = rig();
        e.register(RENDERER_CALL_00E814B0, |_, _| Ret::default());
        let this = vobject(&mut e, 0x200, &[]);
        let first = vobject(&mut e, 8, &[]);
        let second = vobject(&mut e, 8, &[]);
        e.mem.set_u32(this + 0xb8, first);
        e.mem.set_u32(this + 0xbc, second);
        e.call(0x00bc_4640, &args![this]);
        assert_eq!(order(&e), vec![0x00bc_4640, RENDERER_CALL_00E814B0]);
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bc_4640, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bc_4640,
                slot_target(&e, first, 0xa0),
                slot_target(&e, second, 0x88),
                RENDERER_CALL_00E814B0
            ]
        );
        e.mem.set_u32(this + 0xb4, 0xb4);
        for index in 0..3 {
            e.mem.set_u32(this + 0x6c + 4 * index, 0x6c + index);
        }
        e.call_log = Some(vec![]);
        e.call(0x00bc_4810, &args![this]);
        e.call(0x00bc_52f0, &args![this]);
        assert_eq!(
            calls(&e, slot_target(&e, this, 0x128)),
            vec![
                vec![this, 0xb4],
                vec![this, 0x6c],
                vec![this, 0x6d],
                vec![this, 0x6e]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00bc_5050, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bc_5050,
                slot_target(&e, this, 0x13c),
                slot_target(&e, this, 0x148)
            ]
        );
    }

    #[test]
    fn fn_00bc4970_and_00bc4980() {
        let mut e = rig();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00bc_4970, &args![this]).u32(), RECORD_01201FC8);
        e.call(0x00bc_4980, &args![this, 0x1234u32, 0x0102u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 0x1234);
        assert_eq!(e.mem.u32(this + 0x64), 2);
    }
}
