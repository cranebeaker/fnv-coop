//! `NiXenonRenderer/nixenonfragmentshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC build's class is `NiDX9FragmentShader` (RTTI of its vtable `010eff44`); the Xbox
//! PDB's `NiXenonFragmentShader` derives from `NiD3DShader`, and the PC vtable's slots up to
//! +0xe8 line up with that class's (`NiD3DShader`'s slot names below are the Xbox PDB's for
//! the same offset). Most of the virtuals work on the pass at `0126f74c` (the Xbox PDB has
//! a static `NiD3DShader::m_spCurrentPass`). Fields of the shader are read at their offsets:
//! `m_bInitialized` (+0x10), `m_pkD3DRenderer` (+0x18), `m_pkD3DRenderState` (+0x1c),
//! `m_uiCurrentPass` (+0x38) and `m_kDescriptor` (+0x3c, an
//! `NiFragmentShaderInstanceDescriptor` of 0x18 bytes), offsets as the PC code uses them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nixenonrenderer::nid3dpass::NiD3DPass;

/// The pass the shader virtuals work on (`0126f74c`).
const CURRENT_PASS: u32 = 0x0126_f74c;
/// The renderer (`011f4748`) given to the shader declaration and constant map constructors.
const RENDERER: u32 = 0x011f_4748;
/// The handle of the pooled empty `NiFixedString` (`0109b220`), never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Imported `InterlockedDecrement` (import slot `00fdf1c4`).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// Gamebryo's sized object release (`00aa1460`, `cdecl`: object, size) and allocation
/// (`00aa13e0`, `cdecl`: size).
const NI_DELETE: u32 = 0x00aa_1460;
const NI_NEW: u32 = 0x00aa_13e0;

/// `NiDX9FragmentShader`'s vtable (`010eff44`), `NiShader::NiShaderInstanceDescriptor`'s
/// (`010efeb4`), RTTI.
const FRAGMENT_SHADER_VTABLE: u32 = 0x010e_ff44;
const INSTANCE_DESCRIPTOR_VTABLE: u32 = 0x010e_feb4;
/// The records the `GetRTTI` slots return: `NiDX9FragmentShader`'s (`0126f9d8`),
/// `NiShader::NiShaderInstanceDescriptor`'s (`011f5d10`) and
/// `NiFragmentShaderInstanceDescriptor`'s (`01270918`).
const FRAGMENT_SHADER_RTTI: u32 = 0x0126_f9d8;
const INSTANCE_DESCRIPTOR_RTTI: u32 = 0x011f_5d10;
const FRAGMENT_INSTANCE_DESCRIPTOR_RTTI: u32 = 0x0127_0918;

/// The thunk targets (all `thiscall`): `00e81170` (seven stack words), `00e805b0` (seven),
/// `00e811d0` (eight), `00e812f0` (four), `00a8f3f0` (eight, unit `NiMain/nishader.cpp`),
/// `00e805f0` (seven), `00e80580` and `00e80590` (none), `00e81470` (none).
const PRE_PROCESS_PIPELINE_00E81170: u32 = 0x00e8_1170;
const SETUP_RENDERING_PASS_00E805B0: u32 = 0x00e8_05b0;
const SETUP_SHADER_PROGRAMS_00E811D0: u32 = 0x00e8_11d0;
const PREPARE_GEOMETRY_00E812F0: u32 = 0x00e8_12f0;
const POST_RENDER_00A8F3F0: u32 = 0x00a8_f3f0;
const POST_PROCESS_PIPELINE_00E805F0: u32 = 0x00e8_05f0;
const FIRST_PASS_00E80580: u32 = 0x00e8_0580;
const NEXT_PASS_00E80590: u32 = 0x00e8_0590;
const DESTROY_RENDERER_DATA_00E81470: u32 = 0x00e8_1470;
/// `NiD3DShaderInterface::Initialize` (engine map, `00e80570`, `thiscall`, no stack words).
const SHADER_INTERFACE_INITIALIZE: u32 = 0x00e8_0570;
/// `00e6fe30` and `00e6fbb0` (`thiscall` on the renderer; five and two stack words).
const RENDERER_SET_MODEL_TRANSFORM_00E6FE30: u32 = 0x00e6_fe30;
const RENDERER_00E6FBB0: u32 = 0x00e6_fbb0;
/// `00e807d0` (unit `nid3dshader.cpp`, `thiscall`, eight stack words).
const BASE_SETUP_TRANSFORMATIONS_00E807D0: u32 = 0x00e8_07d0;
/// `00e89060` (`thiscall`, three stack words: a state number, the address of the value, the
/// address of a flag byte; `bool`).
const STATE_LOOKUP_00E89060: u32 = 0x00e8_9060;
/// The function table `0126f738`, indexed by the word at +0x134 of the first argument of
/// `fn_00e8d3d0` (each entry `cdecl`, five words).
const PACKER_TABLE: u32 = 0x0126_f738;
/// `NiXenonShaderDeclaration::Create` (`00e76700`, `cdecl`: renderer, two words).
const SHADER_DECLARATION_CREATE: u32 = 0x00e7_6700;
/// `00e804d0` (`cdecl`, the address of a pass pointer it fills) and the pass delete
/// `00e80190` (`thiscall`, no stack words) run when a pass's count at +0x64 reaches 0.
const NEW_PASS_00E804D0: u32 = 0x00e8_04d0;
const PASS_DELETE_00E80190: u32 = 0x00e8_0190;
/// The pass's render-state group helpers: the allocation `00e7f5d0` (no arguments), the set
/// `00e7f430` (`thiscall` on the group, three words) and the remove `00e7f550` (`thiscall`,
/// one word); the sampler table `00e7f7c0` (no arguments) and the pass's set `00b71a10`
/// (`thiscall` on the pass, three words).
const RENDER_STATE_GROUP_NEW_00E7F5D0: u32 = 0x00e7_f5d0;
const RENDER_STATE_GROUP_SET_00E7F430: u32 = 0x00e7_f430;
const RENDER_STATE_GROUP_REMOVE_00E7F550: u32 = 0x00e7_f550;
const STATE_TABLE_00E7F7C0: u32 = 0x00e7_f7c0;
const PASS_SET_STATE_00B71A10: u32 = 0x00b7_1a10;
/// The two program setters `00b79950` (type 0) and `00b80600` (type 2), `thiscall` on the
/// pass, one word.
const PASS_SET_PROGRAM_TYPE_0_00B79950: u32 = 0x00b7_9950;
const PASS_SET_PROGRAM_TYPE_2_00B80600: u32 = 0x00b8_0600;
/// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (`00b7e330`, `thiscall`, two words) and
/// the pass's setters for its vertex (`00c06f30`) and pixel (`00c06ee0`) constant maps
/// (`thiscall`, one word).
const CONSTANT_MAP_CONSTRUCT: u32 = 0x00b7_e330;
const PASS_SET_VERTEX_CONSTANT_MAP_00C06F30: u32 = 0x00c0_6f30;
const PASS_SET_PIXEL_CONSTANT_MAP_00C06EE0: u32 = 0x00c0_6ee0;
/// `NiD3DTextureStage::CreateNewStage` (`00e7edf0`, `cdecl`, the address of a stage
/// pointer), `NiD3DPass::SetStage` (`00e802b0`, `thiscall`, two words) and the stage delete
/// `00e7ecd0` run when a stage's count at +0x60 reaches 0.
const CREATE_NEW_STAGE: u32 = 0x00e7_edf0;
const PASS_SET_STAGE: u32 = 0x00e8_02b0;
const STAGE_DELETE_00E7ECD0: u32 = 0x00e7_ecd0;
/// The two name lookups `AppendTextureSampler` tries (`cdecl`: the name, the address of the
/// index found; `bool`): `00a69d80` (a table at `011f49b0`) and `00a92960` (a table at
/// `011f5e9c`).
const LOOKUP_00A69D80: u32 = 0x00a6_9d80;
const LOOKUP_00A92960: u32 = 0x00a9_2960;
/// The per-stage step `00e8d890` (`thiscall`, three words) and the base `UpdatePipeline`
/// `00a8f3e0` (`thiscall`, seven words).
const STAGE_UPDATE_00E8D890: u32 = 0x00e8_d890;
const BASE_UPDATE_PIPELINE_00A8F3E0: u32 = 0x00a8_f3e0;
/// `NiD3DShader`'s destructor body (`00e806c0`, `thiscall`, no stack words).
const SHADER_DESTRUCTOR_00E806C0: u32 = 0x00e8_06c0;

/// The count of a stage (+0x60): down by one, and the stage deleted at 0.
fn release_stage(e: &mut Engine, stage: u32) {
    if stage != 0 {
        let count = e.mem.u32(stage + 0x60).wrapping_sub(1);
        e.mem.set_u32(stage + 0x60, count);
        if count == 0 {
            e.call(STAGE_DELETE_00E7ECD0, &args![stage]);
        }
    }
}

/// An `NiFixedString` handle let go as the code expands it: unless it is the empty string,
/// the count 8 bytes before it goes down (`InterlockedDecrement`).
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

// Translated from 00bd16e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9FragmentShader`'s slot +0x54, `PreProcessPipeline`'s in
/// the Xbox PDB; also in the vtable at `010bbd40`): a jump to `00e81170` with the same
/// `this` and seven words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd16e0(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    e.call(
        PRE_PROCESS_PIPELINE_00E81170,
        &args![this, a1, a2, a3, a4, a5, a6, a7],
    )
    .u32()
}

// Translated from 00e8d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9FragmentShader`'s last slot, +0xf0; one stack word,
/// never read): `true` when the shader is initialized (`m_bInitialized`, +0x10), otherwise
/// whether `NiD3DShaderInterface::Initialize` (`00e80570`) succeeds.
pub fn fn_00e8d230(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        return true;
    }
    e.call(SHADER_INTERFACE_INITIALIZE, &args![this]).u8() != 0
}

// Translated from 00e8d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x5c, `SetupRenderingPass`'s in the Xbox PDB): a jump to
/// `00e805b0` with the same `this` and seven words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d250(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    e.call(
        SETUP_RENDERING_PASS_00E805B0,
        &args![this, a1, a2, a3, a4, a5, a6, a7],
    )
    .u32()
}

// Translated from 00e8d260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x60, `SetupTransformations`' in the Xbox PDB; eight
/// stack words). Without a third word (`a3` null) it passes on to `00e807d0` with the same
/// words (the third as 0) and returns its result. Otherwise: on the first pass
/// (`m_uiCurrentPass`, +0x38, zero) the renderer (+0x18) gets `00e6fe30` with `a2`, `a7`,
/// 1, 3, 1; the render state (+0x1c) gets its slot +0x2c with the word at `a3` +0x24; the
/// renderer gets `00e6fbb0` with `a7`, 0; and the result is 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d260(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    if a3 == 0 {
        return e
            .call(
                BASE_SETUP_TRANSFORMATIONS_00E807D0,
                &args![this, a1, a2, 0u32, a4, a5, a6, a7, a8],
            )
            .u32();
    }
    let this = this.addr();
    if e.mem.u32(this + 0x38) == 0 {
        let renderer = e.mem.u32(this + 0x18);
        e.call(
            RENDERER_SET_MODEL_TRANSFORM_00E6FE30,
            &args![renderer, a2, a7, 1u32, 3u32, 1u32],
        );
    }
    let render_state = e.mem.u32(this + 0x1c);
    let word = u32::from(e.mem.u16(a3 + 0x24));
    e.vcall(render_state, 0x2c, &args![word]);
    let renderer = e.mem.u32(this + 0x18);
    e.call(RENDERER_00E6FBB0, &args![renderer, a7, 0u32]);
    0
}

// Translated from 00e8d2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x68, `SetupShaderPrograms`' in the Xbox PDB): a jump to
/// `00e811d0` with the same `this` and eight words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d2e0(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    e.call(
        SETUP_SHADER_PROGRAMS_00E811D0,
        &args![this, a1, a2, a3, a4, a5, a6, a7, a8],
    )
    .u32()
}

// Translated from 00e8d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x6c, `PrepareGeometryForRendering`'s in the Xbox PDB):
/// a jump to `00e812f0` with the same `this` and four words.
pub fn fn_00e8d2f0(e: &mut Engine, this: Ptr, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    e.call(PREPARE_GEOMETRY_00E812F0, &args![this, a1, a2, a3, a4])
        .u32()
}

// Translated from 00e8d300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x70, `PostRender`'s in the Xbox PDB): a jump to
/// `00a8f3f0` with the same `this` and eight words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d300(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    e.call(
        POST_RENDER_00A8F3F0,
        &args![this, a1, a2, a3, a4, a5, a6, a7, a8],
    )
    .u32()
}

// Translated from 00e8d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x74, `PostProcessPipeline`'s in the Xbox PDB): a jump to
/// `00e805f0` with the same `this` and seven words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d310(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    e.call(
        POST_PROCESS_PIPELINE_00E805F0,
        &args![this, a1, a2, a3, a4, a5, a6, a7],
    )
    .u32()
}

// Translated from 00e8d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x90, `FirstPass`' in the Xbox PDB): a jump to
/// `00e80580` with the same `this`.
pub fn fn_00e8d320(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIRST_PASS_00E80580, &args![this]).u32()
}

// Translated from 00e8d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x94, `NextPass`' in the Xbox PDB): a jump to
/// `00e80590` with the same `this`.
pub fn fn_00e8d330(e: &mut Engine, this: Ptr) -> u32 {
    e.call(NEXT_PASS_00E80590, &args![this]).u32()
}

// Translated from 00e8d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xcc, `DestroyRendererData`'s in the Xbox PDB): a jump
/// to `00e81470` with the same `this`.
pub fn fn_00e8d340(e: &mut Engine, this: Ptr) {
    e.call(DESTROY_RENDERER_DATA_00E81470, &args![this]);
}

// Translated from 00e8d350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xb8, `GetShaderInstanceDesc`'s in the Xbox PDB): the
/// address of the shader's descriptor (`m_kDescriptor`, +0x3c).
pub fn fn_00e8d350(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x3c))
}

// Translated from 00e8d360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::NiShaderInstanceDescriptor::GetRTTI` (engine map name): the address of the
/// record at `011f5d10`.
pub fn ni_shader_instance_descriptor_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(INSTANCE_DESCRIPTOR_RTTI)
}

// Translated from 00e8d370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::NiShaderInstanceDescriptor`'s scalar deleting destructor (engine map name):
/// stores the class's vtable (`010efeb4`) and, when `flags & 1`, gives the object to the
/// memory manager's free. Returns `this`.
pub fn ni_shader_instance_descriptor_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), INSTANCE_DESCRIPTOR_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e8d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called by `00e8d890` on a texture stage, no stack words): asks
/// the object at the stage's +0x10 for state 0xb (`00e89060`, with the addresses of a value
/// and of a flag byte, both zeroed first) and returns the flag byte when it answers `true`,
/// 0 otherwise.
pub fn fn_00e8d3a0(e: &mut Engine, this: Ptr) -> u8 {
    let object = e.mem.u32(this.addr() + 0x10);
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 4, 0);
        e.mem.set_u8(frame + 3, 0);
        let found = e
            .call(
                STATE_LOOKUP_00E89060,
                &args![object, 0xbu32, frame + 4, frame + 3],
            )
            .u8();
        let mask = if found != 0 { 0xff } else { 0 };
        mask & e.mem.u8(frame + 3)
    })
}

// Translated from 00e8d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words, called by `00e8d890`): calls the entry of
/// the table at `0126f738` chosen by the word at `owner` +0x134 with `owner` +0xf4,
/// `owner` +0x118 and the three other words, and passes its EAX back (the caller does not
/// read it).
pub fn fn_00e8d3d0(e: &mut Engine, owner: Ptr, a2: u32, a3: u32, a4: u32) -> u32 {
    let owner = owner.addr();
    let index = e.mem.u32(owner + 0x134);
    let function = e.mem.u32(PACKER_TABLE.wrapping_add(index.wrapping_mul(4)));
    e.call(
        function,
        &args![
            owner.wrapping_add(0xf4),
            owner.wrapping_add(0x118),
            a2,
            a3,
            a4
        ],
    )
    .u32()
}

// Translated from 00e8d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x9c, `CreateShaderDeclaration`'s in the Xbox PDB):
/// creates a shader declaration (`NiXenonShaderDeclaration::Create` with the renderer at
/// `011f4748` and the two words), gives it to the shader's slot +0xc0 (`SetShaderDecl`'s in
/// the Xbox PDB) and returns it.
pub fn fn_00e8d410(e: &mut Engine, this: Ptr, a1: u32, a2: u32) -> u32 {
    let renderer = e.mem.u32(RENDERER);
    let declaration = e
        .call(SHADER_DECLARATION_CREATE, &args![renderer, a1, a2])
        .u32();
    e.vcall(this.addr(), 0xc0, &args![declaration]);
    declaration
}

/// The pass's render-state group (+0x30), allocated (`00e7f5d0`) when it has none.
fn pass_render_state_group(e: &mut Engine, pass: u32) -> u32 {
    if e.mem.u32(pass + 0x30) == 0 {
        let group = e.call(RENDER_STATE_GROUP_NEW_00E7F5D0, &args![]).u32();
        e.mem.set_u32(pass + 0x30, group);
    }
    e.mem.u32(pass + 0x30)
}

// Translated from 00e8d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xa4, `SetAlphaOverride`'s in the Xbox PDB; six stack
/// words). `false` for a pass number other than 0 or without a current pass (`0126f74c`).
/// Otherwise the pass's render-state group (+0x30, allocated by `00e7f5d0` when missing)
/// gets state 0x1b set (`00e7f430`) to `enable` (`a2`, 0 or 1) with 1. When enabled, the
/// two states 0x13 and 0x14 follow: when their "keep" byte (`a3`, `a5`) is clear the pass
/// sets the state (`00b71a10`) to the entry of the table from `00e7f7c0` at
/// +0x20 + 4 × `a4` (`a6`) with 0, otherwise the group (when there is one) removes it
/// (`00e7f550`). `true` afterwards.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8d450(
    e: &mut Engine,
    _unused_this: Ptr,
    pass_number: u32,
    enable: u8,
    keep_source: u8,
    source: u32,
    keep_destination: u8,
    destination: u32,
) -> bool {
    if pass_number >= 1 {
        return false;
    }
    let pass = e.mem.u32(CURRENT_PASS);
    if pass == 0 {
        return false;
    }
    if enable == 0 {
        let group = pass_render_state_group(e, pass);
        e.call(
            RENDER_STATE_GROUP_SET_00E7F430,
            &args![group, 0x1bu32, 0u32, 1u32],
        );
        return true;
    }
    let group = pass_render_state_group(e, pass);
    e.call(
        RENDER_STATE_GROUP_SET_00E7F430,
        &args![group, 0x1bu32, 1u32, 1u32],
    );
    // 00e8d495: ECX is the current pass (ESI, `0126f74c`).
    let table = e.call(STATE_TABLE_00E7F7C0, &args![pass]).u32();
    for (state, keep, index) in [
        (0x13u32, keep_source, source),
        (0x14u32, keep_destination, destination),
    ] {
        if keep == 0 {
            let value = e
                .mem
                .u32(table.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x20));
            e.call(PASS_SET_STATE_00B71A10, &args![pass, state, value, 0u32]);
        } else {
            let group = e.mem.u32(pass + 0x30);
            if group != 0 {
                e.call(RENDER_STATE_GROUP_REMOVE_00E7F550, &args![group, state]);
            }
        }
    }
    true
}

// Translated from 00e8d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFragmentShaderInstanceDescriptor::GetRTTI` (engine map name): the address of the record
/// at `01270918`.
pub fn ni_fragment_shader_instance_descriptor_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(FRAGMENT_INSTANCE_DESCRIPTOR_RTTI)
}

// Translated from 00e8d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFragmentShaderInstanceDescriptor`'s scalar deleting destructor (engine map name): lets
/// go of the `NiFixedString` at +4 (its count down unless it is the empty string), stores the
/// base vtable (`010efeb4`) and, when `flags & 1`, gives the object to the memory manager's
/// free. Returns `this`.
pub fn ni_fragment_shader_instance_descriptor_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let name = e.mem.u32(this.addr() + 4);
    release_fixed_string(e, name);
    e.mem.set_u32(this.addr(), INSTANCE_DESCRIPTOR_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e8d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +8, `GetRTTI`'s): the address of `NiDX9FragmentShader`'s
/// record at `0126f9d8`.
pub fn fn_00e8d580(_e: &mut Engine) -> Ptr {
    Ptr::new(FRAGMENT_SHADER_RTTI)
}

// Translated from 00e8d590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xa0, `AppendRenderPass`'s in the Xbox PDB; one stack
/// word, the address of the new pass's number). `false` while a current pass (`0126f74c`)
/// exists. Otherwise a new pass is made into a local pointer (`00e804d0`), becomes the
/// current pass, the pass number is set to 0, the local pointer lets the pass go (its count
/// at +0x64 down, `00e80190` at 0) and the result is `true`.
pub fn fn_00e8d590(e: &mut Engine, _unused_this: Ptr, pass_number: Ptr) -> bool {
    if e.mem.u32(CURRENT_PASS) != 0 {
        return false;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(NEW_PASS_00E804D0, &args![local]);
        let pass = e.mem.u32(local.addr());
        e.mem.set_u32(CURRENT_PASS, pass);
        e.mem.set_u32(pass_number.addr(), 0);
        if pass != 0 {
            let count = e.mem.u32(pass + 0x64).wrapping_sub(1);
            e.mem.set_u32(pass + 0x64, count);
            if count == 0 {
                e.call(PASS_DELETE_00E80190, &args![pass]);
            }
        }
    });
    true
}

// Translated from 00e8d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xac, `SetGPUProgram`'s in the Xbox PDB; three stack
/// words, the third never read). `false` for a pass number other than 0 or without a current
/// pass. Otherwise the program goes to the pass by its type (the word at `program` +8):
/// type 0 through `00b79950`, type 2 through `00b80600`, other types nowhere; `true`.
pub fn fn_00e8d5f0(
    e: &mut Engine,
    _unused_this: Ptr,
    pass_number: u32,
    program: Ptr,
    _unused_3: u32,
) -> bool {
    if pass_number >= 1 {
        return false;
    }
    let pass = e.mem.u32(CURRENT_PASS);
    if pass == 0 {
        return false;
    }
    match e.mem.u32(program.addr() + 8) {
        0 => {
            e.call(PASS_SET_PROGRAM_TYPE_0_00B79950, &args![pass, program]);
        }
        2 => {
            e.call(PASS_SET_PROGRAM_TYPE_2_00B80600, &args![pass, program]);
        }
        _ => {}
    }
    true
}

// Translated from 00e8d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xb4, `AppendTextureSampler`'s in the Xbox PDB; five
/// stack words: pass number, the address of the stage number, the sampler name, a word never
/// read, the extra bits). `false` for a pass number other than 0 or without a current pass.
/// Otherwise a new stage is made (`NiD3DTextureStage::CreateNewStage` into a local pointer)
/// and given to the pass at its current stage (`NiD3DPass::SetStage`); when that fails the
/// stage is let go and the result is `false`. On success the stage number is the pass's
/// stage count less one and the name is looked up:
/// - in the first table (`00a69d80`): index 0 to 7 clear bits 24 to 27 of the stage's word at
///   +0xc and or in 0x2 to 0x9 at bit 20 (bits 20 to 23 are not cleared); 8 and 9 clear bits 20 to 27 and
///   set the extra bits with 0x1000000 or 0x2000000; any other index lets the stage go and
///   gives `false`;
/// - else in the second table (`00a92960`): the stage's 16-bit word at +0x5c becomes 0x700,
///   0x600, 0x500 or 0x800 for index 0 to 3 (0 for another index), with the extra bits ored
///   in;
/// - found in neither: nothing changes.
///
/// Except for the bad first-table index the stage is then let go (its count at +0x60 down,
/// `00e7ecd0` at 0) and the result is `true`.
pub fn fn_00e8d630(
    e: &mut Engine,
    _unused_this: Ptr,
    pass_number: u32,
    stage_number: Ptr,
    name: u32,
    _unused_4: u32,
    extra: u32,
) -> bool {
    if pass_number >= 1 {
        return false;
    }
    let pass = e.mem.u32(CURRENT_PASS);
    if pass == 0 {
        return false;
    }
    e.with_stack(8, |e, frame| {
        let stage_slot = frame.addr();
        let index_slot = stage_slot + 4;
        e.mem.set_u32(stage_slot, 0);
        e.call(CREATE_NEW_STAGE, &args![stage_slot]);
        let stage = e.mem.u32(stage_slot);
        let current_stage = e.get(Ptr::<NiD3DPass>::new(pass), NiD3DPass::m_uiCurrentStage);
        if e.call(PASS_SET_STAGE, &args![pass, current_stage, stage])
            .u32()
            != 0
        {
            release_stage(e, stage);
            return false;
        }
        let stage_count = e.get(Ptr::<NiD3DPass>::new(pass), NiD3DPass::m_uiStageCount);
        e.mem
            .set_u32(stage_number.addr(), stage_count.wrapping_sub(1));
        e.mem.set_u32(index_slot, 0);
        if e.call(LOOKUP_00A69D80, &args![name, index_slot]).u8() != 0 {
            let index = e.mem.u32(index_slot);
            let flags = e.mem.u32(stage + 0xc);
            let flags = match index {
                0..=7 => (flags & 0xf0ff_ffff) | ((index + 2) << 20),
                8 => (flags & 0xf00f_ffff) | extra | 0x0100_0000,
                9 => (flags & 0xf00f_ffff) | extra | 0x0200_0000,
                _ => {
                    release_stage(e, stage);
                    return false;
                }
            };
            e.mem.set_u32(stage + 0xc, flags);
        } else if e.call(LOOKUP_00A92960, &args![name, index_slot]).u8() != 0 {
            let filter: u32 = match e.mem.u32(index_slot) {
                0 => 0x700,
                1 => 0x600,
                2 => 0x500,
                3 => 0x800,
                _ => 0,
            };
            e.mem
                .set_u16(stage + 0x5c, ((filter & 0xff00) | extra) as u16);
        }
        release_stage(e, stage);
        true
    })
}

// Translated from 00e8dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiDX9FragmentShader`'s scalar deleting destructor (slot 0):
/// stores the class's vtable, lets go of the descriptor's `NiFixedString` (+0x40), gives the
/// descriptor (+0x3c) its base vtable `010efeb4`, runs `NiD3DShader`'s destructor body
/// (`00e806c0`) and, when `flags & 1`, frees the 0x54 bytes. Returns `this`.
pub fn fn_00e8dbe0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), FRAGMENT_SHADER_VTABLE);
    let name = e.mem.u32(this.addr() + 0x40);
    release_fixed_string(e, name);
    e.mem
        .set_u32(this.addr() + 0x3c, INSTANCE_DESCRIPTOR_VTABLE);
    e.call(SHADER_DESTRUCTOR_00E806C0, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x54u32]);
    }
    this
}

// Translated from 00e8dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x58, `UpdatePipeline`'s in the Xbox PDB; seven stack
/// words): with a current pass, each non-null stage of its array (`m_kStages`, +0x24, up to
/// `m_uiStageCount`, +0x18, re-read each time) gets `00e8d890` with the word at `a4` +0x14
/// and `a5`; then the base `00a8f3e0` runs with the same seven words, and its `EAX`
/// is the answer (the exe ends on that call; `UpdatePipeline` is `unsigned int` in the Xbox
/// PDB).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e8dc30(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    let pass = Ptr::<NiD3DPass>::new(e.mem.u32(CURRENT_PASS));
    if !pass.is_null() {
        let mut index = 0u32;
        while index < e.get(pass, NiD3DPass::m_uiStageCount) {
            let stages = e.get(pass, NiD3DPass::m_kStages_m_pBase);
            let stage = e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)));
            if stage != 0 {
                let word = e.mem.u32(a4.wrapping_add(0x14));
                e.call(STAGE_UPDATE_00E8D890, &args![this, stage, word, a5]);
            }
            index += 1;
        }
    }
    e.call(
        BASE_UPDATE_PIPELINE_00A8F3E0,
        &args![this, a1, a2, a3, a4, a5, a6, a7],
    )
    .u32()
}

// Translated from 00e8dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0xb0, `CreateShaderConstantMap`'s in the Xbox PDB; two
/// stack words: pass number, program kind). 0 for a pass number other than 0 or without a
/// current pass. Kind 0: the pass's vertex constant map (+0x48) when it has one, otherwise a
/// new `NiD3DShaderConstantMap` (0x44 bytes, constructed with the renderer at `011f4748` and
/// 0) given to the pass (`00c06f30`; 0 when the allocation fails, given all the same). Kind
/// 1: the same with the pixel constant map (+0x34), type 2 and `00c06ee0`. Any other kind:
/// 0.
pub fn fn_00e8dca0(e: &mut Engine, _unused_this: Ptr, pass_number: u32, kind: u32) -> u32 {
    if pass_number >= 1 {
        return 0;
    }
    let pass = e.mem.u32(CURRENT_PASS);
    if pass == 0 {
        return 0;
    }
    let (slot, map_type, setter) = match kind {
        0 => (0x48, 0u32, PASS_SET_VERTEX_CONSTANT_MAP_00C06F30),
        1 => (0x34, 2u32, PASS_SET_PIXEL_CONSTANT_MAP_00C06EE0),
        _ => return 0,
    };
    let existing = e.mem.u32(pass + slot);
    if existing != 0 {
        return existing;
    }
    let block = e.call(NI_NEW, &args![0x44u32]).u32();
    let map = if block != 0 {
        let renderer = e.mem.u32(RENDERER);
        e.call(CONSTANT_MAP_CONSTRUCT, &args![block, renderer, map_type])
            .u32()
    } else {
        0
    };
    e.call(setter, &args![pass, map]);
    map
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bd_16e0,
            fn_00bd16e0(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_d230, fn_00e8d230(Ptr, u32) -> bool),
        entry!(
            0x00e8_d250,
            fn_00e8d250(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_d260,
            fn_00e8d260(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_d2e0,
            fn_00e8d2e0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_d2f0, fn_00e8d2f0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(
            0x00e8_d300,
            fn_00e8d300(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_d310,
            fn_00e8d310(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_d320, fn_00e8d320(Ptr) -> u32),
        entry!(0x00e8_d330, fn_00e8d330(Ptr) -> u32),
        entry!(0x00e8_d340, fn_00e8d340(Ptr)),
        entry!(0x00e8_d350, fn_00e8d350(Ptr) -> Ptr),
        entry!(0x00e8_d360, ni_shader_instance_descriptor_get_rtti() -> Ptr),
        entry!(
            0x00e8_d370,
            ni_shader_instance_descriptor_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_d3a0, fn_00e8d3a0(Ptr) -> u8),
        entry!(0x00e8_d3d0, fn_00e8d3d0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e8_d410, fn_00e8d410(Ptr, u32, u32) -> u32),
        entry!(
            0x00e8_d450,
            fn_00e8d450(Ptr, u32, u8, u8, u32, u8, u32) -> bool
        ),
        entry!(
            0x00e8_d530,
            ni_fragment_shader_instance_descriptor_get_rtti() -> Ptr
        ),
        entry!(
            0x00e8_d540,
            ni_fragment_shader_instance_descriptor_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_d580, fn_00e8d580() -> Ptr),
        entry!(0x00e8_d590, fn_00e8d590(Ptr, Ptr) -> bool),
        entry!(0x00e8_d5f0, fn_00e8d5f0(Ptr, u32, Ptr, u32) -> bool),
        entry!(
            0x00e8_d630,
            fn_00e8d630(Ptr, u32, Ptr, u32, u32, u32) -> bool
        ),
        entry!(0x00e8_dbe0, fn_00e8dbe0(Ptr, u32) -> Ptr),
        entry!(
            0x00e8_dc30,
            fn_00e8dc30(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_dca0, fn_00e8dca0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the pages of the globals the shader reads mapped and the given doubles
    /// answering their first word plus one.
    fn engine(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for page in [0x0126_f000, 0x011f_4000, 0x0109_b000, 0x0127_0000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(RENDERER, 0x0e70_0000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0e7f_0000);
        for double in doubles {
            e.register(*double, |_, a| a.first().map_or(0, |w| w + 1).into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable has `target` at `slot`.
    fn object_with_slot(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x60);
        e.mem.set_u32(object, vtable);
        object
    }

    fn new_pass(e: &mut Engine) -> u32 {
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(CURRENT_PASS, pass);
        pass
    }

    #[test]
    fn fn_00bd16e0_jumps_with_seven_words() {
        let mut e = engine(&[PRE_PROCESS_PIPELINE_00E81170]);
        let words = args![0x10u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00bd_16e0, &words).u32(), 0x11);
        assert_eq!(calls_to(&e, PRE_PROCESS_PIPELINE_00E81170), vec![words]);
    }

    #[test]
    fn fn_00e8d230_initializes_only_when_needed() {
        let mut e = engine(&[]);
        e.register(SHADER_INTERFACE_INITIALIZE, |_, _| 0x100u32.into_ret());
        let this = e.mem.alloc(0x54);
        // Not initialized: the result is Initialize's AL (0x100 has AL 0).
        assert!(!e.call(0x00e8_d230, &args![this, 9u32]).bool());
        assert_eq!(calls_to(&e, SHADER_INTERFACE_INITIALIZE), vec![vec![this]]);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_d230, &args![this, 9u32]).bool());
        assert!(calls_to(&e, SHADER_INTERFACE_INITIALIZE).is_empty());
    }

    #[test]
    fn fn_00e8d250_jumps_with_seven_words() {
        let mut e = engine(&[SETUP_RENDERING_PASS_00E805B0]);
        let words = args![0x20u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_d250, &words).u32(), 0x21);
        assert_eq!(calls_to(&e, SETUP_RENDERING_PASS_00E805B0), vec![words]);
    }

    #[test]
    fn fn_00e8d260_without_a_property_passes_on_to_the_base() {
        let mut e = engine(&[BASE_SETUP_TRANSFORMATIONS_00E807D0]);
        let this = e.mem.alloc(0x54);
        let result = e
            .call(
                0x00e8_d260,
                &args![this, 1u32, 2u32, 0u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32();
        assert_eq!(result, this + 1);
        assert_eq!(
            calls_to(&e, BASE_SETUP_TRANSFORMATIONS_00E807D0),
            vec![vec![this, 1, 2, 0, 4, 5, 6, 7, 8]]
        );
    }

    #[test]
    fn fn_00e8d260_sets_the_transforms_on_the_first_pass_only() {
        let mut e = engine(&[RENDERER_SET_MODEL_TRANSFORM_00E6FE30, RENDERER_00E6FBB0]);
        e.register(0x0e50_0000, |_, _| Ret::default());
        let render_state = object_with_slot(&mut e, 0x2c, 0x0e50_0000);
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x18, 0x0e60_0000);
        e.mem.set_u32(this + 0x1c, render_state);
        let property = e.mem.alloc(0x28);
        e.mem.set_u16(property + 0x24, 0x1234);
        let words = args![this, 1u32, 2u32, property, 4u32, 5u32, 6u32, 7u32, 8u32];
        assert_eq!(e.call(0x00e8_d260, &words).u32(), 0);
        assert_eq!(
            calls_to(&e, RENDERER_SET_MODEL_TRANSFORM_00E6FE30),
            vec![vec![0x0e60_0000, 2, 7, 1, 3, 1]]
        );
        assert_eq!(calls_to(&e, 0x0e50_0000), vec![vec![render_state, 0x1234]]);
        assert_eq!(
            calls_to(&e, RENDERER_00E6FBB0),
            vec![vec![0x0e60_0000, 7, 0]]
        );
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_d260, &words);
        assert!(calls_to(&e, RENDERER_SET_MODEL_TRANSFORM_00E6FE30).is_empty());
        assert_eq!(calls_to(&e, RENDERER_00E6FBB0).len(), 1);
    }

    #[test]
    fn fn_00e8d2e0_jumps_with_eight_words() {
        let mut e = engine(&[SETUP_SHADER_PROGRAMS_00E811D0]);
        let words = args![0x30u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32];
        assert_eq!(e.call(0x00e8_d2e0, &words).u32(), 0x31);
        assert_eq!(calls_to(&e, SETUP_SHADER_PROGRAMS_00E811D0), vec![words]);
    }

    #[test]
    fn fn_00e8d2f0_jumps_with_four_words() {
        let mut e = engine(&[PREPARE_GEOMETRY_00E812F0]);
        let words = args![0x40u32, 1u32, 2u32, 3u32, 4u32];
        assert_eq!(e.call(0x00e8_d2f0, &words).u32(), 0x41);
        assert_eq!(calls_to(&e, PREPARE_GEOMETRY_00E812F0), vec![words]);
    }

    #[test]
    fn fn_00e8d300_jumps_with_eight_words() {
        let mut e = engine(&[POST_RENDER_00A8F3F0]);
        let words = args![0x50u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32];
        assert_eq!(e.call(0x00e8_d300, &words).u32(), 0x51);
        assert_eq!(calls_to(&e, POST_RENDER_00A8F3F0), vec![words]);
    }

    #[test]
    fn fn_00e8d310_jumps_with_seven_words() {
        let mut e = engine(&[POST_PROCESS_PIPELINE_00E805F0]);
        let words = args![0x60u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_d310, &words).u32(), 0x61);
        assert_eq!(calls_to(&e, POST_PROCESS_PIPELINE_00E805F0), vec![words]);
    }

    #[test]
    fn fn_00e8d320_jumps_to_first_pass() {
        let mut e = engine(&[FIRST_PASS_00E80580]);
        assert_eq!(e.call(0x00e8_d320, &args![0x70u32]).u32(), 0x71);
        assert_eq!(calls_to(&e, FIRST_PASS_00E80580), vec![vec![0x70]]);
    }

    #[test]
    fn fn_00e8d330_jumps_to_next_pass() {
        let mut e = engine(&[NEXT_PASS_00E80590]);
        assert_eq!(e.call(0x00e8_d330, &args![0x80u32]).u32(), 0x81);
        assert_eq!(calls_to(&e, NEXT_PASS_00E80590), vec![vec![0x80]]);
    }

    #[test]
    fn fn_00e8d340_jumps_to_destroy_renderer_data() {
        let mut e = engine(&[DESTROY_RENDERER_DATA_00E81470]);
        e.call(0x00e8_d340, &args![0x90u32]);
        assert_eq!(
            calls_to(&e, DESTROY_RENDERER_DATA_00E81470),
            vec![vec![0x90]]
        );
    }

    #[test]
    fn fn_00e8d350_returns_the_descriptor() {
        let mut e = engine(&[]);
        assert_eq!(e.call(0x00e8_d350, &args![0x1000u32]).u32(), 0x103c);
    }

    #[test]
    fn instance_descriptor_get_rtti_returns_the_record() {
        let mut e = engine(&[]);
        assert_eq!(
            e.call(0x00e8_d360, &args![]).u32(),
            INSTANCE_DESCRIPTOR_RTTI
        );
    }

    #[test]
    fn instance_descriptor_scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine(&[MEMORY_MANAGER_FREE]);
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00e8_d370, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), INSTANCE_DESCRIPTOR_VTABLE);
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        e.call(0x00e8_d370, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00e8d3a0_answers_the_flag_byte_when_found() {
        let mut e = engine(&[]);
        // The lookup double checks the zeroed locals, writes the flag byte and answers the
        // word of its object.
        e.register(STATE_LOOKUP_00E89060, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            assert_eq!(e.mem.u8(a[3]), 0);
            e.mem.set_u8(a[3], 0x5a);
            e.mem.u32(a[0]).into_ret()
        });
        let state = e.mem.alloc(4);
        let stage = e.mem.alloc(0x64);
        e.mem.set_u32(stage + 0x10, state);
        e.mem.set_u32(state, 1);
        assert_eq!(e.call(0x00e8_d3a0, &args![stage]).u8(), 0x5a);
        let lookup = calls_to(&e, STATE_LOOKUP_00E89060);
        assert_eq!(&lookup[0][..2], &[state, 0xb]);
        // AL 0 (0x100): not found, 0.
        e.mem.set_u32(state, 0x100);
        assert_eq!(e.call(0x00e8_d3a0, &args![stage]).u8(), 0);
    }

    #[test]
    fn fn_00e8d3d0_calls_the_table_entry() {
        let mut e = engine(&[0x0e40_0000]);
        e.mem.set_u32(PACKER_TABLE + 8, 0x0e40_0000);
        let owner = e.mem.alloc(0x140);
        e.mem.set_u32(owner + 0x134, 2);
        let result = e.call(0x00e8_d3d0, &args![owner, 5u32, 6u32, 7u32]).u32();
        assert_eq!(result, owner + 0xf5);
        assert_eq!(
            calls_to(&e, 0x0e40_0000),
            vec![vec![owner + 0xf4, owner + 0x118, 5, 6, 7]]
        );
    }

    #[test]
    fn fn_00e8d410_creates_and_sets_the_declaration() {
        let mut e = engine(&[SHADER_DECLARATION_CREATE]);
        e.register(0x0e50_0000, |_, _| Ret::default());
        let this = object_with_slot(&mut e, 0xc0, 0x0e50_0000);
        let result = e.call(0x00e8_d410, &args![this, 3u32, 4u32]).u32();
        assert_eq!(result, 0x0e70_0001);
        assert_eq!(
            calls_to(&e, SHADER_DECLARATION_CREATE),
            vec![vec![0x0e70_0000, 3, 4]]
        );
        assert_eq!(calls_to(&e, 0x0e50_0000), vec![vec![this, 0x0e70_0001]]);
    }

    fn alpha_engine() -> (Engine, u32) {
        let mut e = engine(&[
            RENDER_STATE_GROUP_SET_00E7F430,
            RENDER_STATE_GROUP_REMOVE_00E7F550,
            PASS_SET_STATE_00B71A10,
        ]);
        e.register(RENDER_STATE_GROUP_NEW_00E7F5D0, |_, _| {
            0x0e30_0000u32.into_ret()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x20 + 4 * 3, 0x33);
        e.mem.set_u32(table + 0x20 + 4 * 5, 0x55);
        e.register_double(STATE_TABLE_00E7F7C0, move |_, _| table.into_ret());
        let pass = new_pass(&mut e);
        (e, pass)
    }

    #[test]
    fn fn_00e8d450_refuses_other_passes_and_no_pass() {
        let (mut e, _) = alpha_engine();
        let words = args![0u32, 1u32, 1u8, 0u8, 0u32, 0u8, 0u32];
        assert!(!e.call(0x00e8_d450, &words).bool());
        e.mem.set_u32(CURRENT_PASS, 0);
        let words = args![0u32, 0u32, 1u8, 0u8, 0u32, 0u8, 0u32];
        assert!(!e.call(0x00e8_d450, &words).bool());
        assert!(calls_to(&e, RENDER_STATE_GROUP_SET_00E7F430).is_empty());
    }

    #[test]
    fn fn_00e8d450_disables_the_override() {
        let (mut e, pass) = alpha_engine();
        let words = args![0u32, 0u32, 0u8, 0u8, 3u32, 0u8, 5u32];
        assert!(e.call(0x00e8_d450, &words).bool());
        assert_eq!(e.mem.u32(pass + 0x30), 0x0e30_0000);
        assert_eq!(
            calls_to(&e, RENDER_STATE_GROUP_SET_00E7F430),
            vec![vec![0x0e30_0000, 0x1b, 0, 1]]
        );
        assert!(calls_to(&e, PASS_SET_STATE_00B71A10).is_empty());
    }

    #[test]
    fn fn_00e8d450_sets_or_removes_the_blend_states() {
        let (mut e, pass) = alpha_engine();
        // Both set from the table.
        let words = args![0u32, 0u32, 1u8, 0u8, 3u32, 0u8, 5u32];
        assert!(e.call(0x00e8_d450, &words).bool());
        assert_eq!(
            calls_to(&e, RENDER_STATE_GROUP_SET_00E7F430),
            vec![vec![0x0e30_0000, 0x1b, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, PASS_SET_STATE_00B71A10),
            vec![vec![pass, 0x13, 0x33, 0], vec![pass, 0x14, 0x55, 0]]
        );
        // Both kept: the group removes them.
        e.call_log = Some(vec![]);
        let words = args![0u32, 0u32, 1u8, 1u8, 3u32, 1u8, 5u32];
        assert!(e.call(0x00e8_d450, &words).bool());
        assert!(calls_to(&e, PASS_SET_STATE_00B71A10).is_empty());
        assert_eq!(
            calls_to(&e, RENDER_STATE_GROUP_REMOVE_00E7F550),
            vec![vec![0x0e30_0000, 0x13], vec![0x0e30_0000, 0x14]]
        );
        assert!(calls_to(&e, RENDER_STATE_GROUP_NEW_00E7F5D0).is_empty());
    }

    #[test]
    fn fragment_instance_descriptor_get_rtti_returns_the_record() {
        let mut e = engine(&[]);
        assert_eq!(
            e.call(0x00e8_d530, &args![]).u32(),
            FRAGMENT_INSTANCE_DESCRIPTOR_RTTI
        );
    }

    #[test]
    fn fragment_instance_descriptor_scalar_deleting_destructor_lets_the_name_go() {
        let mut e = engine(&[IMPORT_INTERLOCKED_DECREMENT, MEMORY_MANAGER_FREE]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 4, 0x0e20_0008);
        assert_eq!(e.call(0x00e8_d540, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e20_0000]]
        );
        assert_eq!(e.mem.u32(this), INSTANCE_DESCRIPTOR_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        // The empty string is not counted; no free without bit 0.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 4, 0x0e7f_0000);
        e.call(0x00e8_d540, &args![this, 0u32]);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
    }

    #[test]
    fn fn_00e8d580_returns_the_record() {
        let mut e = engine(&[]);
        assert_eq!(e.call(0x00e8_d580, &args![]).u32(), FRAGMENT_SHADER_RTTI);
    }

    #[test]
    fn fn_00e8d590_makes_the_current_pass() {
        let mut e = engine(&[PASS_DELETE_00E80190]);
        let made = e.mem.alloc(0x68);
        e.mem.set_u32(made + 0x64, 1);
        e.register_double(NEW_PASS_00E804D0, move |e, a| {
            e.mem.set_u32(a[0], made);
            Ret::default()
        });
        let number = e.mem.alloc(4);
        e.mem.set_u32(number, 7);
        assert!(e.call(0x00e8_d590, &args![0u32, number]).bool());
        assert_eq!(e.mem.u32(CURRENT_PASS), made);
        assert_eq!(e.mem.u32(number), 0);
        // The local pointer's count goes to 0 here: the pass delete runs.
        assert_eq!(e.mem.u32(made + 0x64), 0);
        assert_eq!(calls_to(&e, PASS_DELETE_00E80190), vec![vec![made]]);
        // With a current pass: nothing.
        e.call_log = Some(vec![]);
        e.mem.set_u32(number, 7);
        assert!(!e.call(0x00e8_d590, &args![0u32, number]).bool());
        assert_eq!(e.mem.u32(number), 7);
        assert!(calls_to(&e, NEW_PASS_00E804D0).is_empty());
    }

    #[test]
    fn fn_00e8d5f0_sets_the_program_by_type() {
        let mut e = engine(&[
            PASS_SET_PROGRAM_TYPE_0_00B79950,
            PASS_SET_PROGRAM_TYPE_2_00B80600,
        ]);
        let pass = new_pass(&mut e);
        let program = e.mem.alloc(0x10);
        for (kind, callee) in [
            (0u32, Some(PASS_SET_PROGRAM_TYPE_0_00B79950)),
            (1, None),
            (2, Some(PASS_SET_PROGRAM_TYPE_2_00B80600)),
        ] {
            e.call_log = Some(vec![]);
            e.mem.set_u32(program + 8, kind);
            assert!(e
                .call(0x00e8_d5f0, &args![0u32, 0u32, program, 9u32])
                .bool());
            let calls = calls_to(&e, PASS_SET_PROGRAM_TYPE_0_00B79950).len()
                + calls_to(&e, PASS_SET_PROGRAM_TYPE_2_00B80600).len();
            match callee {
                Some(callee) => assert_eq!(calls_to(&e, callee), vec![vec![pass, program]]),
                None => assert_eq!(calls, 0),
            }
        }
        assert!(!e
            .call(0x00e8_d5f0, &args![0u32, 1u32, program, 9u32])
            .bool());
        e.mem.set_u32(CURRENT_PASS, 0);
        assert!(!e
            .call(0x00e8_d5f0, &args![0u32, 0u32, program, 9u32])
            .bool());
    }

    /// A sampler engine: the new stage (count 1) goes into the pointer, `SetStage` answers
    /// `set_stage`, and the lookups answer from `first` / `second` (the index when found).
    fn sampler_engine(
        set_stage: u32,
        first: Option<u32>,
        second: Option<u32>,
    ) -> (Engine, u32, u32) {
        let mut e = engine(&[STAGE_DELETE_00E7ECD0]);
        let pass = new_pass(&mut e);
        e.mem.set_u32(pass + 0x14, 4);
        e.mem.set_u32(pass + 0x18, 6);
        let stage = e.mem.alloc(0x64);
        e.mem.set_u32(stage + 0xc, 0xffff_ffff);
        e.mem.set_u32(stage + 0x60, 1);
        e.register_double(CREATE_NEW_STAGE, move |e, a| {
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register_double(PASS_SET_STAGE, move |_, _| set_stage.into_ret());
        for (callee, answer) in [(LOOKUP_00A69D80, first), (LOOKUP_00A92960, second)] {
            e.register_double(callee, move |e, a| match answer {
                Some(index) => {
                    e.mem.set_u32(a[1], index);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            });
        }
        (e, pass, stage)
    }

    #[test]
    fn fn_00e8d630_refuses_other_passes_and_a_failed_stage() {
        let (mut e, pass, stage) = sampler_engine(1, Some(0), None);
        let number = e.mem.alloc(4);
        let words = args![0u32, 1u32, number, 0x0e10_0000u32, 0u32, 0u32];
        assert!(!e.call(0x00e8_d630, &words).bool());
        assert!(calls_to(&e, CREATE_NEW_STAGE).is_empty());
        let words = args![0u32, 0u32, number, 0x0e10_0000u32, 0u32, 0u32];
        assert!(!e.call(0x00e8_d630, &words).bool());
        assert_eq!(calls_to(&e, PASS_SET_STAGE), vec![vec![pass, 4, stage]]);
        assert_eq!(calls_to(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
        assert!(calls_to(&e, LOOKUP_00A69D80).is_empty());
    }

    #[test]
    fn fn_00e8d630_first_table_sets_the_address_mode_bits() {
        for (index, expected) in [
            (0u32, 0xf02f_ffffu32),
            (7, 0xf09f_ffff),
            (8, 0xf10f_ffff),
            (9, 0xf20f_ffff),
        ] {
            let (mut e, _, stage) = sampler_engine(0, Some(index), None);
            e.mem.set_u32(stage + 0xc, 0xff0f_ffff);
            let number = e.mem.alloc(4);
            let words = args![0u32, 0u32, number, 0x0e10_0000u32, 0u32, 0x30u32];
            assert!(e.call(0x00e8_d630, &words).bool());
            assert_eq!(e.mem.u32(number), 5);
            assert_eq!(e.mem.u32(stage + 0xc), expected);
            assert_eq!(calls_to(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
            assert!(calls_to(&e, LOOKUP_00A92960).is_empty());
        }
    }

    #[test]
    fn fn_00e8d630_bad_first_table_index_fails() {
        let (mut e, _, stage) = sampler_engine(0, Some(10), None);
        let number = e.mem.alloc(4);
        let words = args![0u32, 0u32, number, 0x0e10_0000u32, 0u32, 0u32];
        assert!(!e.call(0x00e8_d630, &words).bool());
        assert_eq!(e.mem.u32(stage + 0xc), 0xffff_ffff);
        assert_eq!(calls_to(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
    }

    #[test]
    fn fn_00e8d630_second_table_sets_the_filter_word() {
        for (index, expected) in [
            (0u32, 0x0712u16),
            (1, 0x0612),
            (2, 0x0512),
            (3, 0x0812),
            (4, 0x0012),
        ] {
            let (mut e, _, stage) = sampler_engine(0, None, Some(index));
            let number = e.mem.alloc(4);
            let words = args![0u32, 0u32, number, 0x0e10_0000u32, 0u32, 0x12u32];
            assert!(e.call(0x00e8_d630, &words).bool());
            assert_eq!(e.mem.u16(stage + 0x5c), expected);
            assert_eq!(e.mem.u32(stage + 0xc), 0xffff_ffff);
        }
        // Found nowhere: still true, nothing written.
        let (mut e, _, stage) = sampler_engine(0, None, None);
        e.mem.set_u16(stage + 0x5c, 0x4444);
        let number = e.mem.alloc(4);
        let words = args![0u32, 0u32, number, 0x0e10_0000u32, 0u32, 0x12u32];
        assert!(e.call(0x00e8_d630, &words).bool());
        assert_eq!(e.mem.u16(stage + 0x5c), 0x4444);
        assert_eq!(calls_to(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
    }

    #[test]
    fn fn_00e8dbe0_destroys_the_shader() {
        let mut e = engine(&[
            IMPORT_INTERLOCKED_DECREMENT,
            SHADER_DESTRUCTOR_00E806C0,
            NI_DELETE,
        ]);
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x40, 0x0e20_0008);
        assert_eq!(e.call(0x00e8_dbe0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FRAGMENT_SHADER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), INSTANCE_DESCRIPTOR_VTABLE);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e20_0000]]
        );
        assert_eq!(calls_to(&e, SHADER_DESTRUCTOR_00E806C0), vec![vec![this]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this, 0x54]]);
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x40, 0x0e7f_0000);
        e.call(0x00e8_dbe0, &args![this, 0u32]);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert!(calls_to(&e, NI_DELETE).is_empty());
    }

    #[test]
    fn fn_00e8dc30_updates_each_stage_then_the_base() {
        let mut e = engine(&[STAGE_UPDATE_00E8D890, BASE_UPDATE_PIPELINE_00A8F3E0]);
        let pass = new_pass(&mut e);
        let stages = e.mem.alloc(0xc);
        e.mem.set_u32(stages, 0x0e10_0000);
        e.mem.set_u32(stages + 4, 0);
        e.mem.set_u32(stages + 8, 0x0e10_0100);
        e.mem.set_u32(pass + 0x24, stages);
        e.mem.set_u32(pass + 0x18, 3);
        let geometry = e.mem.alloc(0x18);
        e.mem.set_u32(geometry + 0x14, 0x77);
        let words = args![0x0e60_0000u32, 1u32, 2u32, 3u32, geometry, 5u32, 6u32, 7u32];
        e.call(0x00e8_dc30, &words);
        assert_eq!(
            calls_to(&e, STAGE_UPDATE_00E8D890),
            vec![
                vec![0x0e60_0000, 0x0e10_0000, 0x77, 5],
                vec![0x0e60_0000, 0x0e10_0100, 0x77, 5]
            ]
        );
        assert_eq!(
            calls_to(&e, BASE_UPDATE_PIPELINE_00A8F3E0),
            vec![words.clone()]
        );
        // Without a pass only the base runs.
        e.mem.set_u32(CURRENT_PASS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_dc30, &words);
        assert!(calls_to(&e, STAGE_UPDATE_00E8D890).is_empty());
        assert_eq!(calls_to(&e, BASE_UPDATE_PIPELINE_00A8F3E0).len(), 1);
    }

    #[test]
    fn fn_00e8dc30_answers_the_base_eax() {
        let mut e = engine(&[STAGE_UPDATE_00E8D890]);
        e.register(BASE_UPDATE_PIPELINE_00A8F3E0, |_, _| 2u32.into_ret());
        e.mem.set_u32(CURRENT_PASS, 0);
        let words = args![0x0e60_0000u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_dc30, &words).u32(), 2);
    }

    #[test]
    fn fn_00e8dca0_makes_or_returns_the_constant_maps() {
        let mut e = engine(&[
            CONSTANT_MAP_CONSTRUCT,
            PASS_SET_VERTEX_CONSTANT_MAP_00C06F30,
            PASS_SET_PIXEL_CONSTANT_MAP_00C06EE0,
        ]);
        e.register(NI_NEW, |_, _| 0x0e50_0000u32.into_ret());
        let pass = new_pass(&mut e);
        // Kind 0, none yet: a new map of type 0.
        assert_eq!(
            e.call(0x00e8_dca0, &args![0u32, 0u32, 0u32]).u32(),
            0x0e50_0001
        );
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x44]]);
        assert_eq!(
            calls_to(&e, CONSTANT_MAP_CONSTRUCT),
            vec![vec![0x0e50_0000, 0x0e70_0000, 0]]
        );
        assert_eq!(
            calls_to(&e, PASS_SET_VERTEX_CONSTANT_MAP_00C06F30),
            vec![vec![pass, 0x0e50_0001]]
        );
        // Kind 1: type 2 and the pixel setter.
        e.call_log = Some(vec![]);
        e.call(0x00e8_dca0, &args![0u32, 0u32, 1u32]);
        assert_eq!(
            calls_to(&e, CONSTANT_MAP_CONSTRUCT),
            vec![vec![0x0e50_0000, 0x0e70_0000, 2]]
        );
        assert_eq!(
            calls_to(&e, PASS_SET_PIXEL_CONSTANT_MAP_00C06EE0),
            vec![vec![pass, 0x0e50_0001]]
        );
        // Existing maps come back as they are; another kind or pass gives 0.
        e.call_log = Some(vec![]);
        e.mem.set_u32(pass + 0x48, 0x0e11_0000);
        e.mem.set_u32(pass + 0x34, 0x0e22_0000);
        assert_eq!(
            e.call(0x00e8_dca0, &args![0u32, 0u32, 0u32]).u32(),
            0x0e11_0000
        );
        assert_eq!(
            e.call(0x00e8_dca0, &args![0u32, 0u32, 1u32]).u32(),
            0x0e22_0000
        );
        assert_eq!(e.call(0x00e8_dca0, &args![0u32, 0u32, 2u32]).u32(), 0);
        assert_eq!(e.call(0x00e8_dca0, &args![0u32, 1u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, NI_NEW).is_empty());
    }

    #[test]
    fn fn_00e8dca0_gives_a_failed_allocation_as_null() {
        let mut e = engine(&[PASS_SET_VERTEX_CONSTANT_MAP_00C06F30]);
        e.register(NI_NEW, |_, _| Ret::default());
        let pass = new_pass(&mut e);
        assert_eq!(e.call(0x00e8_dca0, &args![0u32, 0u32, 0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PASS_SET_VERTEX_CONSTANT_MAP_00C06F30),
            vec![vec![pass, 0]]
        );
    }
}
