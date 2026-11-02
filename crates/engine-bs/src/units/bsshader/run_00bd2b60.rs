//! `BSShader/run_00bd2b60` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1f while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_SHADER_5: u32 = 0x1f;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bd2b60` (this unit's first function, unnamed, `thiscall`, no
/// words) and the shader method `00bd2d50` (`thiscall`, no words) called directly.
const SHADER_CONSTRUCT_00BD2B60: u32 = 0x00bd_2b60;
const SHADER_METHOD_00BD2D50: u32 = 0x00bd_2d50;

// Translated from 00bd2e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): the creator `GetShader` (`00b55560`) calls for
/// shader 5. No arguments. With the thread's allocation context (+0x2b4 of the thread block)
/// at 0x1f (put back at the end): makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 5 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5,
/// 1, 0)), calls slot +0x94 with (0, 4, 8, 0x80000000, 3, 2, 0, 0) and slot +0xb4. Then
/// allocates 0xa8 bytes and runs the constructor `00bd2b60` (a null allocation is used as it
/// is), calls the shader's slots +0x4c and +0x13c, the method `00bd2d50`, slot +0x11c, then
/// +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bd2e70 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn fn_00bd2e70(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_SHADER_5);
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
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 4u32, 8u32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xa8u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD2B60, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.vcall(shader, 0x13c, &args![]);
    e.call(SHADER_METHOD_00BD2D50, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bbeb8`; no RTTI locator before it).
const VTABLE_010BBEB8: u32 = 0x010b_beb8;

// Translated from 00bd2b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): the constructor of shader 5, `thiscall`, no
/// stack words. Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010bbeb8`,
/// constructs the `NiPointer` arrays at +0x6c (4) and +0x7c (2; both destructor `00b66f20`),
/// clears the word +0x84, stores 0.0 at +0x88 to +0xa4 and sets +0x68 to 8. Answers `this`.
// Translation incomplete: 00bd2b60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the arrays) is not translated.
pub fn fn_00bd2b60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010BBEB8);
    construct_pointer_array(e, base + 0x6c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 2, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x84, 0);
    for offset in (0x88u32..=0xa4).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 8);
    this
}

// Translated from 00bd2d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): shader 5's stages and passes (the creator
/// `00bd2e70` calls it after the vtable slot +0x13c), `thiscall`, no stack words. Gives the
/// pass pointer at +0x84 a new pass (`00e804d0`); makes one texture stage (`CreateNewStage`
/// into a stack pointer, `SetupDefaultStage` with index 0 and 3, 2, 1, no filter mode) and puts
/// it on the pass (`NiD3DPass::SetStage` with the pass's word at +0x14); assigns the shader's
/// +0x6c to the pass's vertex shader `NiPointer` (+0x5c) and +0x7c to its pixel shader
/// `NiPointer` (+0x44); then drops the stage reference.
// Translation incomplete: 00bd2d50 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bd2d50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let slot = base + 0x84;
    e.call(NEW_PASS_00E804D0, &args![slot]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + 0x6c);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + 0x7c);
        ni_pointer_assign(e, pass + 0x44, pixel);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
}

/// Strings of `00bd2730` (read in Ghidra): `"speedtree\\frond.v.hlsl"` (`010bbe9c`),
/// `"speedtree\\frond.p.hlsl"` (`010bbe84`), the macros `"FOG"` (`010aece0`) and `"PT"`
/// (`010b9174`), the empty string, the formats `"STFROND%03i.vso"` (`010bbe74`),
/// `"STFROND2%03i.pso"` (`010bbe60`) and `"STFROND%03i.pso"` (`010bbe50`) and the vertex
/// profile `"vs_1_1"` (`010a93ec`).
const FROND_VERTEX_010BBE9C: u32 = 0x010b_be9c;
const FROND_PIXEL_010BBE84: u32 = 0x010b_be84;
const MACRO_FOG_010AECE0: u32 = 0x010a_ece0;
const MACRO_PT_010B9174: u32 = 0x010b_9174;
const FROND_EMPTY_01011584: u32 = 0x0101_1584;
const FORMAT_FROND_VERTEX_010BBE74: u32 = 0x010b_be74;
const FORMAT_FROND2_PIXEL_010BBE60: u32 = 0x010b_be60;
const FORMAT_FROND_PIXEL_010BBE50: u32 = 0x010b_be50;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// The pixel profile chooser `00b4f380` (`cdecl`, a byte) and the shader model setting
/// (`011f91c0`).
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
/// The `00bd2730` frame (saved registers up to the cookie) and its buffers.
const FROND_FRAME_BYTES: u32 = 0x3f0;
const FROND_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x2e8,
    path: 0x1e4,
};
/// The six source blocks in the exe's order of filling (four vertex, two pixel).
const FROND_SOURCES: [ShaderSource; 6] = [
    ShaderSource {
        offset: 0x1c,
        words: &[FROND_VERTEX_010BBE9C, 0, 0],
        cleared: 0x40,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            FROND_VERTEX_010BBE9C,
            MACRO_FOG_010AECE0,
            FROND_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            FROND_VERTEX_010BBE9C,
            MACRO_PT_010B9174,
            FROND_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            FROND_VERTEX_010BBE9C,
            MACRO_PT_010B9174,
            FROND_EMPTY_01011584,
            MACRO_FOG_010AECE0,
            FROND_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x30,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[FROND_PIXEL_010BBE84, 0, 0],
        cleared: 0x40,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            FROND_PIXEL_010BBE84,
            MACRO_FOG_010AECE0,
            FROND_EMPTY_01011584,
            0,
            0,
        ],
        cleared: 0x38,
    },
];

// Translated from 00bd2730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bbeb8` slot +0x13c): shader 5's `LoadShaders`, no stack
/// words. Fills six source blocks on its stack (`frond.v.hlsl` with none, `FOG`, `PT`, `PT`
/// and `FOG`; `frond.p.hlsl` with none and `FOG`; the rest of each cleared by `memset`);
/// compiles the four vertex blocks (`"STFROND%03i.vso"` with the index, `vs_1_1`) into the
/// `NiPointer`s +0x6c to +0x78; takes the pixel profile from `00b4f380(1)` and compiles the
/// two pixel blocks into +0x7c and +0x80, named `"STFROND2%03i.pso"` at shader model
/// (`011f91c0`) 2 or more and `"STFROND%03i.pso"` below. A block without a file clears its
/// pointer instead.
// Translation incomplete: 00bd2730 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bd2730(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(FROND_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &FROND_SOURCES);
        for index in 0..4u32 {
            compile_indexed_or_clear(
                e,
                this,
                frame,
                &FROND_BUFFERS,
                0x1c + 0x4c * index,
                this + 0x6c + 4 * index,
                FORMAT_FROND_VERTEX_010BBE74,
                index,
                PROFILE_VS_1_1_010A93EC,
                false,
            );
        }
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        let format = if e.global::<i32>(SHADER_MODEL_011F91C0) >= 2 {
            FORMAT_FROND2_PIXEL_010BBE60
        } else {
            FORMAT_FROND_PIXEL_010BBE50
        };
        for index in 0..2u32 {
            compile_indexed_or_clear(
                e,
                this,
                frame,
                &FROND_BUFFERS,
                0x14c + 0x4c * index,
                this + 0x7c + 4 * index,
                format,
                index,
                profile,
                true,
            );
        }
    });
}

/// The `eh vector destructor iterator`
/// (`00ec5fce`, `stdcall`: array, element size, count, destructor), `BSShader::~BSShader`
/// (engine map, `00be0f10`, no stack words) and the Ni free (`00aa1460`, `cdecl`: block, size).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bd2c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): shader 5's destructor, no stack words. Stores
/// the vtable `010bbeb8`, clears (releasing what they hold) the `NiPointer`s +0x6c to +0x78
/// and +0x7c, +0x80, drops the pass at +0x84 (its count at +0x64 down by one, deleted through
/// `00e80190` at 0; the pointer is left as it is), destroys the arrays +0x7c (2) and +0x6c (4)
/// (destructor `00b66f20`) and runs `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bd2c40 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the pass, the arrays and the base) is not translated.
pub fn fn_00bd2c40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BBEB8);
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x6c + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x7c + 4 * index);
    }
    let pass = e.mem.u32(base + 0x84);
    if pass != 0 {
        let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
        e.mem.set_u32(pass + 0x64, left);
        if left == 0 {
            e.call(PASS_DELETE_00E80190, &args![pass]);
        }
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x7c, 4u32, 2u32, DESTRUCT_00B66F20],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x6c, 4u32, 4u32, DESTRUCT_00B66F20],
    );
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

// Translated from 00bd2fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): shader 5's scalar deleting destructor
/// (vtable `010bbeb8` slot +0), `thiscall`, `RET 4`. Runs the destructor (`00bd2c40`) and,
/// when bit 0 of the flags is set, frees the 0xa8 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bd2fd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd2c40(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0xa8u32]);
    }
    this
}

// Translated from 00bd2720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010bb530`, `010bbfd8`,
/// `010bc380`), `thiscall`, no stack words (the same vtable position as `00bc0920`, whose slot
/// takes none): a jump through the shader's vtable +0x13c.
pub fn fn_00bd2720(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x13c, &args![]);
}

// Translated from 00bd2b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bbfdc`), `thiscall`,
/// no stack words: calls the shader's vtable +0x128 with its word +0x84.
pub fn fn_00bd2b40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pass = e.mem.u32(base + 0x84);
    e.vcall(base, 0x128, &args![pass]);
}

/// The record `00bd2c30` answers (`0120233c`).
const RECORD_0120233C: u32 = 0x0120_233c;

// Translated from 00bd2c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bbec0`, `ECX` unread):
/// answers the address `0120233c`.
pub fn fn_00bd2c30(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RECORD_0120233C)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_2720, fn_00bd2720(Ptr)),
        entry!(0x00bd_2b40, fn_00bd2b40(Ptr)),
        entry!(0x00bd_2c30, fn_00bd2c30(Ptr) -> Ptr),
        entry!(0x00bd_2d50, fn_00bd2d50(Ptr)),
        entry!(0x00bd_2b60, fn_00bd2b60(Ptr) -> Ptr),
        entry!(0x00bd_2e70, fn_00bd2e70() -> Ptr),
        entry!(0x00bd_2730, fn_00bd2730(Ptr)),
        entry!(0x00bd_2c40, fn_00bd2c40(Ptr)),
        entry!(0x00bd_2fd0, fn_00bd2fd0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd2e70_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD2B60, &[SHADER_METHOD_00BD2D50]);
        let (shader, log) = run(&mut e, 0x00bd_2e70);
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
                (
                    declaration_slot(0x94),
                    vec![d, 0, 4, 8, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xa8]),
                (SHADER_CONSTRUCT_00BD2B60, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (SHADER_METHOD_00BD2D50, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_SHADER_5);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bd2b60_builds_the_shader() {
        let mut e = rig();
        let this = e.mem.alloc(0xa8);
        e.mem.write(this, &[0xcc; 0xa8]);
        assert_eq!(e.call(0x00bd_2b60, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010BBEB8);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x84u32..=0xa4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 8);
    }
}

#[cfg(test)]
mod stages_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bd2d50_builds_one_pass_with_one_stage() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xa8);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0x6c, vertex);
        e.mem.set_u32(this + 0x7c, pixel);
        e.call(SHADER_METHOD_00BD2D50, &args![this]);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x84]]);
        let pass = e.mem.u32(this + 0x84);
        let set = calls(&e, PASS_SET_STAGE);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][..2], [pass, 0]);
        let stage = set[0][2];
        assert_eq!(
            calls(&e, SETUP_DEFAULT_STAGE),
            vec![vec![stage, 0, 3, 2, 1]]
        );
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(e.mem.u32(vertex + 4), 2);
    }
}

#[cfg(test)]
mod frond_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load(model: i32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e55_0000u32.into_ret());
        let this = e.mem.alloc(0xa8);
        e.call(0x00bd_2730, &args![this]);
        (e, this)
    }

    #[test]
    fn fn_00bd2730_compiles_four_vertex_and_two_pixel_shaders() {
        let (e, this) = load(2);
        let vertex: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 0x6c + 4 * i)).collect();
        assert_eq!(
            compiled(&e, vertex[0]),
            Compiled {
                file: FROND_VERTEX_010BBE9C,
                output: FORMAT_FROND_VERTEX_010BBE74,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                macros: macro_words(&[]),
            }
        );
        assert_eq!(
            compiled(&e, vertex[3]).macros,
            macro_words(&[
                (MACRO_PT_010B9174, FROND_EMPTY_01011584),
                (MACRO_FOG_010AECE0, FROND_EMPTY_01011584)
            ])
        );
        let pixel = e.mem.u32(this + 0x80);
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: FROND_PIXEL_010BBE84,
                output: FORMAT_FROND2_PIXEL_010BBE60,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[(MACRO_FOG_010AECE0, FROND_EMPTY_01011584)]),
            }
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, vec![0, 1, 2, 3, 0, 1]);
        assert_eq!(calls(&e, MEMSET).len(), 6);
    }

    #[test]
    fn fn_00bd2730_names_the_pixel_shaders_by_model() {
        let (e, this) = load(1);
        let pixel = e.mem.u32(this + 0x7c);
        assert_eq!(compiled(&e, pixel).output, FORMAT_FROND_PIXEL_010BBE50);
    }

    #[test]
    fn fn_00bd2c40_releases_drops_the_pass_and_destroys() {
        let mut e = rig();
        e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xa8);
        let shader = object(&mut e, 1);
        e.mem.set_u32(this + 0x70, shader);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x84, pass);
        e.call(0x00bd_2c40, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BBEB8);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shader]]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        // The pass pointer itself is left.
        assert_eq!(e.mem.u32(this + 0x84), pass);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x7c, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 4, DESTRUCT_00B66F20],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bd2fd0_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        for addr in [VECTOR_DESTRUCT, BS_SHADER_DESTRUCT, NI_FREE_00AA1460] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xa8);
        assert_eq!(e.call(0x00bd_2fd0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
        e.call(0x00bd_2fd0, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0xa8]]);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{
        calls, engine, slot_target, vobject,
    };

    #[test]
    fn the_small_slot_functions() {
        let mut e = engine(&[]);
        let this = vobject(&mut e, 0x90, &[]);
        e.mem.set_u32(this + 0x84, 0x84);
        e.call(0x00bd_2720, &args![this]);
        e.call(0x00bd_2b40, &args![this]);
        assert_eq!(calls(&e, slot_target(&e, this, 0x13c)), vec![vec![this]]);
        assert_eq!(
            calls(&e, slot_target(&e, this, 0x128)),
            vec![vec![this, 0x84]]
        );
        assert_eq!(e.call(0x00bd_2c30, &args![this]).u32(), RECORD_0120233C);
    }
}
