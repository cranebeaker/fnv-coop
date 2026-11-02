//! `BSShader/run_00bd4d00` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bd5670` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BD5670: u32 = 0x00bd_5670;

// Translated from 00bd5920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the creator `GetShader` (`00b55560`) calls for
/// shader 3. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 4 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 5, 4, 3, 0), (2, 7, 5, 1, 0)), calls slot +0x94
/// with (0, 3, 6, 0x80000000, 0, 5, 1, 0) and slot +0xb4. Then allocates 0x100 bytes and runs
/// the constructor `00bd5670` with the declaration (a null allocation is used as it is),
/// calls the shader's slots +0x4c, +0x13c, +0x11c, +0x140, then +0xc0 with the declaration,
/// and answers the shader.
// Translation incomplete: 00bd5920 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd5920(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 5, 4, 3, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 3u32, 6u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x100u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD5670, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x11c, 0x140] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bc260`; no RTTI locator before it).
const VTABLE_010BC260: u32 = 0x010b_c260;
/// The four words copied to +0xa0 (`011a9bd0`) and the shader model setting (`011f91c0`).
const COLOR_011A9BD0: u32 = 0x011a_9bd0;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bd5670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the constructor the creator `00bd5970` runs,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010bc260`, constructs the `NiPointer` arrays at +0x6c (1, destructor
/// `00b79930`), +0x7c (4) and +0x8c (2; destructor `00b66f20`), stores 0.0 at +0xa0 to +0xfc,
/// sets +0x68 to 7, clears (releasing what they hold) the `NiPointer`s at +0x7c to +0x88 and
/// +0x8c, +0x90, assigns the declaration to the `NiPointer` at +0x28, stores 0.0 at +0x70,
/// copies the four words at `011a9bd0` to +0xa0, sets the 16-bit count at +0x9c to 0xe4 (0x50
/// when the setting at `011f91c0` is below 2, signed), allocates that many 0x10-byte elements
/// from the memory manager into +0x94 and clears +0x98. Answers `this`.
// Translation incomplete: 00bd5670 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn fn_00bd5670(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC260);
    construct_pointer_array(e, base + 0x6c, 1, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x7c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x8c, 2, DESTRUCT_00B66F20);
    for offset in (0xa0u32..=0xfc).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 7);
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x7c + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x8c + 4 * index);
    }
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    e.mem.set_f32(base + 0x70, 0.0);
    for index in 0..4u32 {
        let word = e.global::<u32>(COLOR_011A9BD0 + 4 * index);
        e.mem.set_u32(base + 0xa0 + 4 * index, word);
    }
    let count: u16 = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
        0x50
    } else {
        0xe4
    };
    e.mem.set_u16(base + 0x9c, count);
    let count = e.mem.u16(base + 0x9c);
    let elements = allocate_array(e, u32::from(count), 0x10);
    e.mem.set_u32(base + 0x94, elements);
    e.mem.set_u32(base + 0x98, 0);
    this
}

/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, one word), the
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor), `BSShader::~BSShader` (engine map, `00be0f10`, no stack words) and the Ni
/// free (`00aa1460`, `cdecl`: block, size).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
/// The destructor of the shader with the vtable `010bc070` (`00bd4570`, unit `run_00bd3f80`,
/// no stack words).
const DESTRUCT_00BD4570: u32 = 0x00bd_4570;

// Translated from 00bd4d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the scalar deleting destructor of the shader
/// with the vtable `010bc070` (its slot +0), `thiscall`, `RET 4`. Runs its destructor
/// (`00bd4570`) and, when bit 0 of the flags is set, frees the 0x12c bytes (`00aa1460`).
/// Answers `this`.
pub fn fn_00bd4d00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00BD4570, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x12cu32]);
    }
    this
}

/// `SpeedTreeShader::SetShaderConstants` (Xbox PDB, `00bb2f50`, `cdecl`: the map, a count).
const SET_SHADER_CONSTANTS_00BB2F50: u32 = 0x00bb_2f50;

/// `target`'s `NiPointer` takes the object of `source`'s, as the code expands it: nothing when
/// they already hold the same object; otherwise the old one is released when not null and the
/// source is read again, stored and held when not null.
fn copy_pointer(e: &mut Engine, target: u32, source: u32) {
    let old = e.mem.u32(target);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(target, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00bd4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`; vtable `010bc070` slot +0x11c, a
/// `CreateConstantMaps`), no stack words. Makes the vertex constant map (+0x34, kind 0) and
/// the pixel one (+0x30, kind 2) when they are null (no entries), copies them into the
/// `NiPointer`s +0x84 (pixel) and +0x88 (vertex) and runs
/// `SpeedTreeShader::SetShaderConstants` (`00bb2f50`) with the vertex map and 0x2a.
// Translation incomplete: 00bd4d30 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn fn_00bd4d30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) == 0 {
        make_constant_map(e, base, 0x34, 0);
    }
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
    }
    copy_pointer(e, base + 0x84, base + 0x30);
    copy_pointer(e, base + 0x88, base + 0x34);
    let map = e.mem.u32(base + 0x34);
    e.call(SET_SHADER_CONSTANTS_00BB2F50, &args![map, 0x2au32]);
}

/// Strings of the distant LOD `LoadShaders` (read in Ghidra):
/// `"tallgrass\\1x\\v\\DistantLOD.v.hlsl"` (`010bc23c`), `"tallgrass\\1x\\p\\highDetail.p.hlsl"`
/// (`010b8b7c`), the macros `"BILLBOARD"` (`010b8ba0`), `"VS_2_0"` (`010b8bc4`), `"PS_2_0"`
/// (`010b8b74`); the formats `"DISTLOD1%03i.vso"` (`010bc228`), `"DISTLOD1%03i.pso"`
/// (`010bc214`), `"DISTLOD2%03i.vso"` (`010bc200`), `"DISTLOD2%03i.pso"` (`010bc1ec`); the
/// profiles `"vs_1_1"` (`010a93ec`) and `"vs_2_0"` (`010a93e4`).
const DISTANT_LOD_VERTEX_010BC23C: u32 = 0x010b_c23c;
const HIGH_DETAIL_PIXEL_010B8B7C: u32 = 0x010b_8b7c;
const MACRO_BILLBOARD_010B8BA0: u32 = 0x010b_8ba0;
const MACRO_VS_2_0_010B8BC4: u32 = 0x010b_8bc4;
const MACRO_PS_2_0_010B8B74: u32 = 0x010b_8b74;
const FORMAT_LOD1_VERTEX_010BC228: u32 = 0x010b_c228;
const FORMAT_LOD1_PIXEL_010BC214: u32 = 0x010b_c214;
const FORMAT_LOD2_VERTEX_010BC200: u32 = 0x010b_c200;
const FORMAT_LOD2_PIXEL_010BC1EC: u32 = 0x010b_c1ec;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
/// The pixel profile chooser `00b4f380` (`cdecl`, a byte; answers a profile string).
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// The frame (from the saved registers up to the cookie) and its path and name buffers.
const LOD_FRAME_BYTES: u32 = 0x3f0;
const LOD_PATH: u32 = 0x1e4;
const LOD_NAME: u32 = 0x2e8;
/// The six source blocks in the exe's order of filling.
const LOD_SOURCES: [ShaderSource; 6] = [
    ShaderSource {
        offset: 0xb4,
        words: &[DISTANT_LOD_VERTEX_010BC23C, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x100,
        words: &[DISTANT_LOD_VERTEX_010BC23C, MACRO_BILLBOARD_010B8BA0, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[DISTANT_LOD_VERTEX_010BC23C, MACRO_VS_2_0_010B8BC4, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            DISTANT_LOD_VERTEX_010BC23C,
            MACRO_VS_2_0_010B8BC4,
            0,
            MACRO_BILLBOARD_010B8BA0,
            0,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1c,
        words: &[HIGH_DETAIL_PIXEL_010B8B7C, 0, 0],
        cleared: 0x40,
    },
    ShaderSource {
        offset: 0x68,
        words: &[HIGH_DETAIL_PIXEL_010B8B7C, MACRO_PS_2_0_010B8B74, 0, 0, 0],
        cleared: 0x38,
    },
];

/// One compile of the distant LOD loader: `MakePath` of the block's file into the path
/// buffer, the name formatted with the index, the compile (`this`, path, the block's macros,
/// profile, name); answers the compiled shader.
#[allow(clippy::too_many_arguments)]
fn lod_compile(
    e: &mut Engine,
    this: u32,
    frame: u32,
    block: u32,
    format: u32,
    index: u32,
    profile: u32,
    pixel: bool,
) -> u32 {
    let file = e.mem.u32(frame + block);
    e.call(MAKE_PATH, &args![file, frame + LOD_PATH]);
    e.call(
        FORMAT_00406D00,
        &args![frame + LOD_NAME, 0x104u32, format, index],
    );
    let compile = if pixel {
        COMPILE_PIXEL_SHADER
    } else {
        COMPILE_VERTEX_SHADER
    };
    e.call(
        compile,
        &args![
            this,
            frame + LOD_PATH,
            frame + block + 4,
            profile,
            frame + LOD_NAME
        ],
    )
    .u32()
}

// Translated from 00bd4f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc260` slot +0x13c): the distant LOD shader's
/// `LoadShaders`, no stack words. Fills six source blocks on its stack (`DistantLOD.v.hlsl`
/// with no macros, `BILLBOARD`, `VS_2_0`, and both; `highDetail.p.hlsl` with none and
/// `PS_2_0`; the rest of each block cleared by `memset`) and takes the pixel profile from
/// `00b4f380(1)`. At shader model 1 (`011f91c0`): the vertex shaders of the first two blocks
/// (`"DISTLOD1%03i.vso"` with 0 and 1, profile `vs_1_1`) into the `NiPointer`s +0x7c and
/// +0x80 and the pixel shader of the first pixel block (`"DISTLOD1%03i.pso"` with 0) into
/// +0x8c; otherwise the vertex shaders of the `VS_2_0` blocks (`"DISTLOD2%03i.vso"` with 2 and
/// 3, `vs_2_0`) into +0x84 and +0x88 and the pixel shader of the `PS_2_0` block
/// (`"DISTLOD2%03i.pso"` with 1) into +0x90. Each file goes through `BSShader::MakePath` and
/// each name through the formatter `00406d00` first.
// Translation incomplete: 00bd4f30 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bd4f30(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(LOD_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &LOD_SOURCES);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        if e.global::<i32>(SHADER_MODEL_011F91C0) == 1 {
            for (index, block) in [0xb4u32, 0x100].iter().enumerate() {
                let index = index as u32;
                let shader = lod_compile(
                    e,
                    this,
                    frame,
                    *block,
                    FORMAT_LOD1_VERTEX_010BC228,
                    index,
                    PROFILE_VS_1_1_010A93EC,
                    false,
                );
                ni_pointer_assign(e, this + 0x7c + 4 * index, shader);
            }
            let shader = lod_compile(
                e,
                this,
                frame,
                0x1c,
                FORMAT_LOD1_PIXEL_010BC214,
                0,
                profile,
                true,
            );
            ni_pointer_assign(e, this + 0x8c, shader);
        } else {
            for (index, block) in [(2u32, 0x14cu32), (3, 0x198)] {
                let shader = lod_compile(
                    e,
                    this,
                    frame,
                    block,
                    FORMAT_LOD2_VERTEX_010BC200,
                    index,
                    PROFILE_VS_2_0_010A93E4,
                    false,
                );
                ni_pointer_assign(e, this + 0x84 + 4 * (index - 2), shader);
            }
            let shader = lod_compile(
                e,
                this,
                frame,
                0x68,
                FORMAT_LOD2_PIXEL_010BC1EC,
                1,
                profile,
                true,
            );
            ni_pointer_assign(e, this + 0x90, shader);
        }
    });
}

// Translated from 00bd5330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the distant LOD shader's destructor, no stack
/// words. Stores the vtable `010bc260`, clears (releasing what they hold) the `NiPointer`s
/// +0x7c to +0x88 and +0x8c, +0x90, clears +0x98 and gives the instance array (+0x94) back to
/// the memory manager (`00aa4060`), destroys the arrays at +0x8c (2) and +0x7c (4) (destructor
/// `00b66f20`) and +0x6c (1, `00b79930`) and runs `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bd5330 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the arrays and the base) is not translated.
pub fn fn_00bd5330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC260);
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x7c + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x8c + 4 * index);
    }
    let instances = e.mem.u32(base + 0x94);
    e.mem.set_u32(base + 0x98, 0);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, instances]);
    for (offset, count, destructor) in [
        (0x8cu32, 2u32, DESTRUCT_00B66F20),
        (0x7c, 4, DESTRUCT_00B66F20),
        (0x6c, 1, DESTRUCT_00B79930),
    ] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, destructor],
        );
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00bd5490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): sets up the distant LOD shader's pass (+0x6c),
/// no stack words. A texture stage (on the stack, cleared first) made by
/// `NiD3DTextureStage::CreateNewStage`, `BSShader::SetupDefaultStage(stage, 0, 3, 2, 1)`,
/// `NiD3DPass::SetStage` with the pass's word +0x14 and the stage; the pass's `NiPointer`s
/// +0x5c and +0x44 take the shader's +0x7c and +0x8c; the render states (0x18, 10) (shader
/// model 2 or more only), (0xa8, 7), (0x1b, 0), (7, 1), (0x17, 4) and (0xe, 1); then the
/// stage reference is dropped.
// Translation incomplete: 00bd5490 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage) is not translated.
pub fn fn_00bd5490(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let pass = e.mem.u32(base + 0x6c);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.mem.u32(base + 0x6c);
        let texture = e.mem.u32(base + 0x7c);
        ni_pointer_assign(e, pass + 0x5c, texture);
        let pass = e.mem.u32(base + 0x6c);
        let texture = e.mem.u32(base + 0x8c);
        ni_pointer_assign(e, pass + 0x44, texture);
        if e.global::<i32>(SHADER_MODEL_011F91C0) >= 2 {
            let pass = e.mem.u32(base + 0x6c);
            pass_render_state(e, pass, 0x18, 10);
        }
        for (state, value) in [(0xa8u32, 7u32), (0x1b, 0), (7, 1), (0x17, 4), (0xe, 1)] {
            let pass = e.mem.u32(base + 0x6c);
            pass_render_state(e, pass, state, value);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
}

// Translated from 00bd58f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the distant LOD shader's scalar deleting
/// destructor (vtable `010bc260` slot +0), `thiscall`, `RET 4`. Runs the destructor
/// (`00bd5330`) and, when bit 0 of the flags is set, frees the 0x100 bytes (`00aa1460`).
/// Answers `this`.
pub fn fn_00bd58f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd5330(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x100u32]);
    }
    this
}

// Translated from 00bd5a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`; vtable `010bc260` slot +0x140), no stack
/// words. Gives the pass pointer +0x6c a new pass (`00e804d0`), sets it up (`00bd5490`) and
/// answers true.
pub fn fn_00bd5a40(e: &mut Engine, this: Ptr) -> bool {
    e.call(NEW_PASS_00E804D0, &args![this.addr() + 0x6c]);
    fn_00bd5490(e, this);
    true
}

/// `00e814b0` (unnamed, `thiscall`, no stack words).
const RENDERER_CALL_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00bd4ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bc330`), `thiscall`, no stack words. With the
/// word +0x28 set, calls the vtable +0xa0 of each of the four objects from +0x7c that is set;
/// then the vtable +0x88 of each of the two from +0x8c that is set; then jumps to `00e814b0`.
pub fn fn_00bd4ed0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        for index in 0..4 {
            let object = e.mem.u32(base + 0x7c + 4 * index);
            if object != 0 {
                e.vcall(object, 0xa0, &args![]);
            }
        }
    }
    for index in 0..2 {
        let object = e.mem.u32(base + 0x8c + 4 * index);
        if object != 0 {
            e.vcall(object, 0x88, &args![]);
        }
    }
    e.call(RENDERER_CALL_00E814B0, &args![base]);
}

// Translated from 00bd5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc384`), `thiscall`,
/// no stack words: calls the shader's vtable +0x128 with its word +0x6c.
pub fn fn_00bd5320(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pass = e.mem.u32(base + 0x6c);
    e.vcall(base, 0x128, &args![pass]);
}

/// The record `00bd5460` answers (`012023c8`).
const RECORD_012023C8: u32 = 0x0120_23c8;

// Translated from 00bd5460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc268`, `ECX` unread):
/// answers the address `012023c8`.
pub fn fn_00bd5460(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RECORD_012023C8)
}

// Translated from 00bd5470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc398`), `thiscall`
/// with two words: stores the first at +0x74 and the low byte of the second at +0x78.
pub fn fn_00bd5470(e: &mut Engine, this: Ptr, word: u32, flag: u32) {
    e.mem.set_u32(this.addr() + 0x74, word);
    e.mem.set_u8(this.addr() + 0x78, flag as u8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_4ed0, fn_00bd4ed0(Ptr)),
        entry!(0x00bd_5320, fn_00bd5320(Ptr)),
        entry!(0x00bd_5460, fn_00bd5460(Ptr) -> Ptr),
        entry!(0x00bd_5470, fn_00bd5470(Ptr, u32, u32)),
        entry!(0x00bd_5670, fn_00bd5670(Ptr, Ptr) -> Ptr),
        entry!(0x00bd_5920, fn_00bd5920() -> Ptr),
        entry!(0x00bd_4d00, fn_00bd4d00(Ptr, u32) -> Ptr),
        entry!(0x00bd_4d30, fn_00bd4d30(Ptr)),
        entry!(0x00bd_4f30, fn_00bd4f30(Ptr)),
        entry!(0x00bd_5330, fn_00bd5330(Ptr)),
        entry!(0x00bd_5490, fn_00bd5490(Ptr)),
        entry!(0x00bd_58f0, fn_00bd58f0(Ptr, u32) -> Ptr),
        entry!(0x00bd_5a40, fn_00bd5a40(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd5920_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD5670, &[]);
        let (shader, log) = run(&mut e, 0x00bd_5920);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 3, 6, 0x8000_0000, 0, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x100]),
                (SHADER_CONSTRUCT_00BD5670, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
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
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BD0 + 4 * index, 0x40 + index);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bd_5670, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bd5670_builds_the_shader() {
        let (e, this, declaration) = construct(2);
        assert_eq!(e.mem.u32(this), VTABLE_010BC260);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 1, DESTRUCT_00B79930),
                pointer_array(this + 0x7c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x8c, 2, DESTRUCT_00B66F20),
            ]
        );
        for index in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0xa0 + 4 * index), 0x40 + index);
        }
        for offset in (0xb0u32..=0xfc).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x68), 7);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u16(this + 0x9c), 0xe4);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40]]
        );
        assert_eq!(e.mem.u32(this + 0x98), 0);
    }

    #[test]
    fn fn_00bd5670_small_count_below_model_2() {
        let (e, this, _) = construct(1);
        assert_eq!(e.mem.u16(this + 0x9c), 0x50);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x500]]
        );
    }
}

#[cfg(test)]
mod distant_lod_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00bd4d00_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        noop(&mut e, &[DESTRUCT_00BD4570, NI_FREE_00AA1460]);
        let this = e.mem.alloc(0x12c);
        assert_eq!(e.call(0x00bd_4d00, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
        e.call(0x00bd_4d00, &args![this, 1u32]);
        assert_eq!(calls(&e, DESTRUCT_00BD4570), vec![vec![this]; 2]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x12c]]);
    }

    #[test]
    fn fn_00bd4d30_makes_the_maps_and_copies_them() {
        const MAP_VTABLE: u32 = 0x0e31_0000;
        let mut e = rig();
        e.put_vtable(MAP_VTABLE, &[0, REFERENCE_DELETE]);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        noop(&mut e, &[SET_SHADER_CONSTANTS_00BB2F50]);
        let this = e.mem.alloc(0x12c);
        // An old object in +0x84 (released, deleted) and none in +0x88.
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x84, old);
        e.call(0x00bd_4d30, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_ne!(vertex, 0);
        assert_eq!(e.mem.u32(this + 0x84), pixel);
        assert_eq!(e.mem.u32(this + 0x88), vertex);
        assert_eq!(e.mem.u32(vertex + 4), 2);
        assert_eq!(e.mem.u32(pixel + 4), 2);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert_eq!(
            calls(&e, SET_SHADER_CONSTANTS_00BB2F50),
            vec![vec![vertex, 0x2a]]
        );
        // Again: nothing new is made, the copies are already equal.
        e.call_log = Some(vec![]);
        e.call(0x00bd_4d30, &args![this]);
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
    }

    fn load(model: i32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        e.register(PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a, &[1][..]);
            0x0e55_0000u32.into_ret()
        });
        let this = e.mem.alloc(0x100);
        e.call(0x00bd_4f30, &args![this]);
        (e, this)
    }

    #[test]
    fn fn_00bd4f30_compiles_the_model_1_shaders() {
        let (e, this) = load(1);
        let vertex = [e.mem.u32(this + 0x7c), e.mem.u32(this + 0x80)];
        assert_eq!(
            compiled(&e, vertex[0]),
            Compiled {
                file: DISTANT_LOD_VERTEX_010BC23C,
                output: FORMAT_LOD1_VERTEX_010BC228,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                macros: macro_words(&[]),
            }
        );
        assert_eq!(
            compiled(&e, vertex[1]).macros,
            macro_words(&[(MACRO_BILLBOARD_010B8BA0, 0)])
        );
        let pixel = e.mem.u32(this + 0x8c);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: HIGH_DETAIL_PIXEL_010B8B7C,
                output: FORMAT_LOD1_PIXEL_010BC214,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[]),
            }
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, vec![0, 1, 0]);
        for offset in [0x84u32, 0x88, 0x90] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(vertex[0] + 4), 1);
    }

    #[test]
    fn fn_00bd4f30_compiles_the_model_2_shaders() {
        let (e, this) = load(3);
        let vertex = [e.mem.u32(this + 0x84), e.mem.u32(this + 0x88)];
        assert_eq!(
            compiled(&e, vertex[0]),
            Compiled {
                file: DISTANT_LOD_VERTEX_010BC23C,
                output: FORMAT_LOD2_VERTEX_010BC200,
                profile: PROFILE_VS_2_0_010A93E4,
                pixel: false,
                macros: macro_words(&[(MACRO_VS_2_0_010B8BC4, 0)]),
            }
        );
        assert_eq!(
            compiled(&e, vertex[1]).macros,
            macro_words(&[(MACRO_VS_2_0_010B8BC4, 0), (MACRO_BILLBOARD_010B8BA0, 0)])
        );
        let pixel = e.mem.u32(this + 0x90);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: HIGH_DETAIL_PIXEL_010B8B7C,
                output: FORMAT_LOD2_PIXEL_010BC1EC,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[(MACRO_PS_2_0_010B8B74, 0)]),
            }
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, vec![2, 3, 1]);
        assert_eq!(e.mem.u32(this + 0x7c), 0);
    }

    #[test]
    fn fn_00bd5330_releases_the_pointers_and_destroys() {
        let mut e = rig();
        noop(
            &mut e,
            &[MEMORY_DEALLOCATE, VECTOR_DESTRUCT, BS_SHADER_DESTRUCT],
        );
        let this = e.mem.alloc(0x100);
        let textures = [object(&mut e, 1), object(&mut e, 2)];
        e.mem.set_u32(this + 0x80, textures[0]);
        e.mem.set_u32(this + 0x90, textures[1]);
        e.mem.set_u32(this + 0x94, 0x7000);
        e.mem.set_u32(this + 0x98, 5);
        e.call(0x00bd_5330, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BC260);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![textures[0]]]);
        assert_eq!(e.mem.u32(textures[1] + 4), 1);
        assert_eq!(e.mem.u32(this + 0x98), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7000]]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x8c, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x7c, 4, 4, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 1, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    fn set_up_pass(model: i32) -> (Engine, u32, u32) {
        let mut e = pass_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x100);
        let textures = [object(&mut e, 1), object(&mut e, 1)];
        e.mem.set_u32(this + 0x7c, textures[0]);
        e.mem.set_u32(this + 0x8c, textures[1]);
        e.call(0x00bd_5a40, &args![this]);
        let pass = e.mem.u32(this + 0x6c);
        (e, this, pass)
    }

    #[test]
    fn fn_00bd5490_sets_up_the_stage_textures_and_states() {
        let (e, this, pass) = set_up_pass(2);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x6c]]);
        let setups = calls(&e, SETUP_DEFAULT_STAGE);
        assert_eq!(&setups[0][1..], &[0, 3, 2, 1]);
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        let set = calls(&e, PASS_SET_STAGE);
        assert_eq!(set.len(), 1);
        assert_eq!(&set[0][..2], &[pass, 0]);
        // Held by the pass once; the local reference was dropped.
        assert_eq!(e.mem.u32(set[0][2] + 0x60), 1);
        assert_eq!(e.mem.u32(pass + 0x5c), e.mem.u32(this + 0x7c));
        assert_eq!(e.mem.u32(pass + 0x44), e.mem.u32(this + 0x8c));
        assert_eq!(
            render_states(&e, pass),
            vec![
                (0x18, 10),
                (0xa8, 7),
                (0x1b, 0),
                (7, 1),
                (0x17, 4),
                (0xe, 1)
            ]
        );
    }

    #[test]
    fn fn_00bd5490_leaves_out_state_0x18_below_model_2() {
        let (e, _, pass) = set_up_pass(1);
        assert_eq!(
            render_states(&e, pass),
            vec![(0xa8, 7), (0x1b, 0), (7, 1), (0x17, 4), (0xe, 1)]
        );
    }

    #[test]
    fn fn_00bd58f0_destroys_and_frees_with_bit_0() {
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
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00bd_58f0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x100]]);
        e.call(0x00bd_58f0, &args![this, 0u32]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bd5a40_answers_true() {
        let mut e = pass_rig();
        e.map(0x011f_9000, 0x1000);
        let this = e.mem.alloc(0x100);
        assert!(e.call(0x00bd_5a40, &args![this]).bool());
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{
        calls, engine, order, slot_target, vobject,
    };

    #[test]
    fn fn_00bd4ed0_updates_the_set_objects() {
        let mut e = engine(&[RENDERER_CALL_00E814B0]);
        let this = e.mem.alloc(0x98);
        let a = vobject(&mut e, 8, &[]);
        let b = vobject(&mut e, 8, &[]);
        e.mem.set_u32(this + 0x88, a);
        e.mem.set_u32(this + 0x90, b);
        e.call(0x00bd_4ed0, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bd_4ed0,
                slot_target(&e, b, 0x88),
                RENDERER_CALL_00E814B0
            ]
        );
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bd_4ed0, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bd_4ed0,
                slot_target(&e, a, 0xa0),
                slot_target(&e, b, 0x88),
                RENDERER_CALL_00E814B0
            ]
        );
    }

    #[test]
    fn the_small_slot_functions() {
        let mut e = engine(&[]);
        let this = vobject(&mut e, 0x80, &[]);
        e.mem.set_u32(this + 0x6c, 0x6c);
        e.call(0x00bd_5320, &args![this]);
        assert_eq!(
            calls(&e, slot_target(&e, this, 0x128)),
            vec![vec![this, 0x6c]]
        );
        assert_eq!(e.call(0x00bd_5460, &args![this]).u32(), RECORD_012023C8);
        e.call(0x00bd_5470, &args![this, 0x1234u32, 0x0201u32]);
        assert_eq!(e.mem.u32(this + 0x74), 0x1234);
        assert_eq!(e.mem.u8(this + 0x78), 1);
    }
}
