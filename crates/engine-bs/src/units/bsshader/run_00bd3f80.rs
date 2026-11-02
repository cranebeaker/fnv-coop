//! `BSShader/run_00bd3f80` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bd44c0` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BD44C0: u32 = 0x00bd_44c0;

// Translated from 00bd4750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`): the creator `GetShader` (`00b55560`) calls for
/// shader 4. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 6 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 7, 5, 1, 0), (3, 0xe, 0xd, 2, 0),
/// (4, 0xf, 0xe, 2, 0)), calls slot +0x94 with (0, 5, 0xa, 0x80000000, 3, 2, 0, 0) and slot
/// +0xb4. Then allocates 0x12c bytes and runs the constructor `00bd44c0` with the
/// declaration (a null allocation is used as it is), calls the shader's slots +0x4c, +0x144,
/// +0x150, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bd4750 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd4750(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 7, 5, 1, 0],
        [3, 0xe, 0xd, 2, 0],
        [4, 0xf, 0xe, 2, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 5u32, 0xau32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x12cu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD44C0, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shared shader constructor `00b79b00` (unnamed; `thiscall`, four stack words, each an
/// object kept by an `NiPointer`).
const SHADER_BASE_CONSTRUCT_00B79B00: u32 = 0x00b7_9b00;
/// The shader's vtable (`010bc070`; no RTTI locator before it).
const VTABLE_010BC070: u32 = 0x010b_c070;

// Translated from 00bd44c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`): the constructor of shader 4, `thiscall` with
/// one stack word (the declaration). Runs `00b79b00` with (the declaration, 0, 0, 0), stores
/// the vtable `010bc070`, constructs the `NiPointer` arrays at +0x8c (0x1e) and +0x104 (0xa;
/// both destructor `00b66f20`) and sets +0x68 to 5. Answers `this`.
// Translation incomplete: 00bd44c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the arrays) is not translated.
pub fn fn_00bd44c0(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, declaration, 0u32, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BC070);
    construct_pointer_array(e, base + 0x8c, 0x1e, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x104, 0xa, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 5);
    this
}

/// `ShadowLightShader::SetupPassConstants` (engine map, `00b7b450`, `thiscall`, `RET 4`).
const SHADOW_LIGHT_SETUP_PASS_CONSTANTS: u32 = 0x00b7_b450;

// Translated from 00bd3270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc070` slot +0xfc): a 5-byte jump to
/// `ShadowLightShader::SetupPassConstants` (`00b7b450`), passing on `this` and its one word.
pub fn fn_00bd3270(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    e.call(SHADOW_LIGHT_SETUP_PASS_CONSTANTS, &args![this, word])
        .u32()
}

/// CRT `_gcvt` (`00ecdd16`, `cdecl`: the `double`, the digit count, the buffer), the `float`
/// it is given (`011adbcc`, through `FLD`/`FSTP double`) and the pixel profile chooser
/// `00b4f380` (`cdecl`, a byte).
const GCVT: u32 = 0x00ec_dd16;
const SPECULAR_VALUE_011ADBCC: u32 = 0x011a_dbcc;
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// Strings of `00bd3c60` (read in Ghidra): the HLSL files `lighting\\1x\\p\\base.p.hlsl`,
/// `ambDiffDirTexture`, `ambDiffDirAndPt`, `diffuseDir`, `diffusePt`, `specularDir`,
/// `specularPt` (each `.p.hlsl`), the macros `VC`, `TEX`, `SPECMULT`, `FOG`, the empty string
/// and the name format `"STB1%03i.pso"`.
const BASE_PIXEL_010AF2D4: u32 = 0x010a_f2d4;
const AMB_DIFF_DIR_TEXTURE_PIXEL_010AF21C: u32 = 0x010a_f21c;
const AMB_DIFF_DIR_AND_PT_PIXEL_010AF1EC: u32 = 0x010a_f1ec;
const DIFFUSE_DIR_PIXEL_010AF294: u32 = 0x010a_f294;
const DIFFUSE_PT_PIXEL_010AF2B4: u32 = 0x010a_f2b4;
const SPECULAR_DIR_PIXEL_010AF244: u32 = 0x010a_f244;
const SPECULAR_PT_PIXEL_010AF268: u32 = 0x010a_f268;
const MACRO_VC_010AED64: u32 = 0x010a_ed64;
const MACRO_TEX_010AECAC: u32 = 0x010a_ecac;
const MACRO_SPECMULT_010AF288: u32 = 0x010a_f288;
const MACRO_FOG_010AECE0: u32 = 0x010a_ece0;
const EMPTY_01011584: u32 = 0x0101_1584;
const FORMAT_STB1_PIXEL_010BC060: u32 = 0x010b_c060;
/// The `00bd3c60` frame (saved registers up to the cookie), its `_gcvt` buffer, path and name
/// buffers, and its ten source blocks (0x4c bytes apart from +0x20).
const PIXEL_FRAME_BYTES: u32 = 0x684;
const PIXEL_NUMBER: u32 = 0x318;
const PIXEL_PATH: u32 = 0x478;
const PIXEL_NAME: u32 = 0x57c;
const PIXEL_BLOCKS: u32 = 0x20;

/// A source block filled as the loaders do: the words, then `memset` of `cleared` bytes after
/// them.
fn fill_block(e: &mut Engine, block: u32, words: &[u32], cleared: u32) {
    for (index, word) in words.iter().enumerate() {
        e.mem.set_u32(block + 4 * index as u32, *word);
    }
    let rest = block + 4 * words.len() as u32;
    e.call(MEMSET, &args![rest, 0u32, cleared]);
}

// Translated from 00bd3c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc070` slot +0x14c): loads the shader's ten pixel
/// shaders, no stack words. Formats the `float` at `011adbcc` with 12 digits (`_gcvt`) into a
/// stack buffer, fills ten source blocks (`base`; `ambDiffDirTexture` with `VC` twice;
/// `ambDiffDirAndPt`; `diffuseDir`; `diffusePt`; `base` with `TEX` and `VC`; `specularDir` and
/// `specularPt` with `SPECMULT` set to the formatted number; `base` with `FOG`; the rest of
/// each block cleared by `memset`), takes the pixel profile from `00b4f380(1)` and, for each
/// block in order, compiles it (`MakePath`, the name `"STB1%03i.pso"` with the block's index,
/// `CompilePixelShaderHLSL`) into the `NiPointer` at +0x104 + 4 * index, or clears that
/// pointer when the block has no file.
// Translation incomplete: 00bd3c60 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bd3c60(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(PIXEL_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        let number = frame + PIXEL_NUMBER;
        let value = x87_load32(e.global::<u32>(SPECULAR_VALUE_011ADBCC));
        e.call(GCVT, &args![value, 0xcu32, number]);
        let empty = EMPTY_01011584;
        let blocks: [(&[u32], u32); 10] = [
            (&[BASE_PIXEL_010AF2D4, 0], 0x44),
            (
                &[
                    AMB_DIFF_DIR_TEXTURE_PIXEL_010AF21C,
                    MACRO_VC_010AED64,
                    empty,
                    0,
                ],
                0x3c,
            ),
            (
                &[
                    AMB_DIFF_DIR_TEXTURE_PIXEL_010AF21C,
                    MACRO_VC_010AED64,
                    empty,
                    0,
                ],
                0x3c,
            ),
            (&[AMB_DIFF_DIR_AND_PT_PIXEL_010AF1EC, 0], 0x44),
            (&[DIFFUSE_DIR_PIXEL_010AF294, 0], 0x44),
            (&[DIFFUSE_PT_PIXEL_010AF2B4, 0], 0x44),
            (
                &[
                    BASE_PIXEL_010AF2D4,
                    MACRO_TEX_010AECAC,
                    empty,
                    MACRO_VC_010AED64,
                    empty,
                    0,
                ],
                0x34,
            ),
            (
                &[
                    SPECULAR_DIR_PIXEL_010AF244,
                    MACRO_SPECMULT_010AF288,
                    number,
                    0,
                ],
                0x3c,
            ),
            (
                &[
                    SPECULAR_PT_PIXEL_010AF268,
                    MACRO_SPECMULT_010AF288,
                    number,
                    0,
                ],
                0x3c,
            ),
            (&[BASE_PIXEL_010AF2D4, MACRO_FOG_010AECE0, empty, 0], 0x3c),
        ];
        for (index, (words, cleared)) in blocks.iter().enumerate() {
            let block = frame + PIXEL_BLOCKS + 0x4c * index as u32;
            fill_block(e, block, words, *cleared);
        }
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        for index in 0..10u32 {
            let block = frame + PIXEL_BLOCKS + 0x4c * index;
            let slot = this + 0x104 + 4 * index;
            let file = e.mem.u32(block);
            if file == 0 {
                ni_pointer_clear(e, slot);
                continue;
            }
            e.call(MAKE_PATH, &args![file, frame + PIXEL_PATH]);
            e.call(
                FORMAT_00406D00,
                &args![
                    frame + PIXEL_NAME,
                    0x104u32,
                    FORMAT_STB1_PIXEL_010BC060,
                    index
                ],
            );
            let shader = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![
                        this,
                        frame + PIXEL_PATH,
                        block + 4,
                        profile,
                        frame + PIXEL_NAME
                    ],
                )
                .u32();
            ni_pointer_assign(e, slot, shader);
        }
    });
}

/// `NiAVObject::GetProperty` (engine map, `00a59d30`, `thiscall`, one word).
const GET_PROPERTY: u32 = 0x00a5_9d30;

// Translated from 00bd3fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`; vtable `010bc070` slot +0x50), `thiscall`, `RET
/// 4` (the geometry; `this` is not read). Takes the geometry's property 3
/// (`NiAVObject::GetProperty`), keeps it only when its word +0x1c is 0xa (otherwise the null
/// pointer, which the code uses as it is), sets its word +0x58 to 4, runs its vtable slot
/// +0x9c with the geometry and answers true.
pub fn fn_00bd3fc0(e: &mut Engine, _this: Ptr, geometry: Ptr) -> bool {
    let geometry = geometry.addr();
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    let property = if property != 0 && e.mem.u32(property + 0x1c) == 0xa {
        property
    } else {
        0
    };
    let vtable = e.mem.u32(property);
    let target = e.mem.u32(vtable + 0x9c);
    e.mem.set_u32(property + 0x58, 4);
    e.call(target, &args![property, geometry]);
    true
}

/// The record whose address `00bd4560` answers (`01202348`).
const RECORD_01202348: u32 = 0x0120_2348;

// Translated from 00bd4560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc070` slot +8, where the Xbox PDB's `NiObject` order has
/// `GetRTTI`): answers the address `01202348`. No arguments.
pub fn fn_00bd4560(_e: &mut Engine) -> Ptr {
    Ptr::new(RECORD_01202348)
}

/// The shader's 0x1e global pass pointers (`01202350` to `012023c8`).
const PASSES_01202350: u32 = 0x0120_2350;
const PASSES_END_012023C8: u32 = 0x0120_23c8;
/// The `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor) and the base destructor `00b79de0` (unnamed, no stack words).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BASE_DESTRUCT_00B79DE0: u32 = 0x00b7_9de0;
/// `00be0870` (unnamed, unit `bsshader.cpp` neighbourhood; `thiscall`, no stack words).
const CALL_00BE0870: u32 = 0x00be_0870;

/// The shader's `NiPointer`s at +0x8c (0x1e) and +0x104 (0xa) released and cleared, then the
/// global passes (`01202350` to `012023c8`) dropped.
fn clear_shader_pointers(e: &mut Engine, base: u32) {
    for index in 0..0x1eu32 {
        ni_pointer_clear(e, base + 0x8c + 4 * index);
    }
    for index in 0..0xau32 {
        ni_pointer_clear(e, base + 0x104 + 4 * index);
    }
    let mut slot = PASSES_01202350;
    while slot < PASSES_END_012023C8 {
        pass_pointer_clear(e, slot);
        slot += 4;
    }
}

// Translated from 00bd4570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`): the destructor of the shader with the vtable
/// `010bc070`, no stack words. Stores the vtable, clears (releasing what they hold) the
/// `NiPointer`s +0x8c (0x1e) and +0x104 (0xa) and the global passes `01202350` to `012023c8`,
/// destroys the arrays +0x104 (0xa) and +0x8c (0x1e) (destructor `00b66f20`) and runs the base
/// destructor `00b79de0`.
// Translation incomplete: 00bd4570 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the arrays and the base) is not translated.
pub fn fn_00bd4570(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC070);
    clear_shader_pointers(e, base);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x104, 4u32, 0xau32, DESTRUCT_00B66F20],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x8c, 4u32, 0x1eu32, DESTRUCT_00B66F20],
    );
    e.call(BASE_DESTRUCT_00B79DE0, &args![base]);
}

// Translated from 00bd46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd3f80`; vtable `010bc070` slot +0x4c), no stack words.
/// Runs `00be0870`, clears (releasing what they hold) the `NiPointer`s +0x8c (0x1e) and +0x104
/// (0xa) and the global passes `01202350` to `012023c8`, and answers true.
pub fn fn_00bd46a0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(CALL_00BE0870, &args![base]);
    clear_shader_pointers(e, base);
    true
}

/// The shader manager's pass state (`0126f74c`, its +0x24 the table of constant blocks), the
/// pass number (`011f91e4`), the current record (`011f91e0`), the two words copied into the
/// blocks (`011f95e4`, `011f95e8`), `BSRenderedTexture::GetTexture` (engine map, `004bc320`,
/// `thiscall`, one word) and `00c03230` (unit `BSShader`, unnamed; `thiscall`, `RET 8`).
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
const CURRENT_PASS_011F91E4: u32 = 0x011f_91e4;
const CURRENT_RECORD_011F91E0: u32 = 0x011f_91e0;
const CONSTANTS_011F95E4: u32 = 0x011f_95e4;
const CONSTANTS_011F95E8: u32 = 0x011f_95e8;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
const TEXTURE_FOR_00C03230: u32 = 0x00c0_3230;

/// The constant block `index` (a byte offset into the table) of the pass state.
fn constant_block(e: &Engine, state: u32, offset: u32) -> u32 {
    let table = e.mem.u32(state + 0x24);
    e.mem.u32(table + offset)
}

// Translated from 00bd4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc070` slot +0x78), `thiscall`, `RET 4` (one word whose
/// object +0xc is used). By the pass number (`011f91e4`), the compiler's two switches (byte
/// maps `00bd49c8` and `00bd4ad8`, translated as `match` arms): for passes 0x7a, 0x7b, 0x9f,
/// 0xaa, 0xb5, 0xc0, 0xdf, 0xe8, 0xf1, 0xfa, 0x108, 0x114, 0x11f, 0x12a the word at the
/// object's +0xac's address goes to +8 of the pass state's (`0126f74c`) first constant block
/// and `00c03230(object, 0)` to the second's; for 0x7e, 0x130, 0x131, 0x137, 0x143, 0x158,
/// 0x159, 0x15f, 0x165, 0x16b, 0x171, 0x177 `00c03230(object, 0)` to the first; for 0x151
/// and 0x234 the +0xac word to the first. Then the words `011f95e4` and `011f95e8` go to +8 of
/// the blocks +0x18 and +0x1c (passes 0xaa, 0xc0, 0x114, 0x12a), +0x14 and +0x18 (0xe8, 0xfa)
/// or +4 and +8 (0x165). For pass 0x234 the block +0x18 also gets
/// `BSRenderedTexture::GetTexture(0)` (`004bc320`) on the word +0x10c of what the current
/// record's (`011f91e0`) +0xc object points to. Last, the shader's slot +0x108.
pub fn fn_00bd4890(e: &mut Engine, this: Ptr, source: Ptr) {
    let this = this.addr();
    let object = e.mem.u32(source.addr() + 0xc);
    let state = e.global::<u32>(PASS_STATE_0126F74C);
    let pass = e.global::<u32>(CURRENT_PASS_011F91E4);
    let first_case = match pass {
        0x234 => 2,
        0x7a | 0x7b | 0x9f | 0xaa | 0xb5 | 0xc0 | 0xdf | 0xe8 | 0xf1 | 0xfa | 0x108 | 0x114
        | 0x11f | 0x12a => 0,
        0x7e | 0x130 | 0x131 | 0x137 | 0x143 | 0x158 | 0x159 | 0x15f | 0x165 | 0x16b | 0x171
        | 0x177 => 1,
        0x151 => 2,
        _ => 3,
    };
    match first_case {
        0 => {
            let holder = e.mem.u32(object + 0xac);
            let block = constant_block(e, state, 0);
            let value = e.mem.u32(holder);
            e.mem.set_u32(block + 8, value);
            let block = constant_block(e, state, 4);
            let texture = e.call(TEXTURE_FOR_00C03230, &args![object, 0u32]).u32();
            e.mem.set_u32(block + 8, texture);
        }
        1 => {
            let block = constant_block(e, state, 0);
            let texture = e.call(TEXTURE_FOR_00C03230, &args![object, 0u32]).u32();
            e.mem.set_u32(block + 8, texture);
        }
        2 => {
            let holder = e.mem.u32(object + 0xac);
            let block = constant_block(e, state, 0);
            let value = e.mem.u32(holder);
            e.mem.set_u32(block + 8, value);
        }
        _ => {}
    }
    let second = match pass {
        0xaa | 0xc0 | 0x114 | 0x12a => Some((0x18u32, 0x1cu32)),
        0xe8 | 0xfa => Some((0x14, 0x18)),
        0x165 => Some((4, 8)),
        _ => None,
    };
    if let Some((first, next)) = second {
        let block = constant_block(e, state, first);
        let value = e.global::<u32>(CONSTANTS_011F95E4);
        e.mem.set_u32(block + 8, value);
        let block = constant_block(e, state, next);
        let value = e.global::<u32>(CONSTANTS_011F95E8);
        e.mem.set_u32(block + 8, value);
    }
    if pass == 0x234 {
        let table = e.mem.u32(state + 0x24);
        let record = e.global::<u32>(CURRENT_RECORD_011F91E0);
        let property = e.mem.u32(record + 0xc);
        let block = e.mem.u32(table + 0x18);
        let pointed = e.mem.u32(property);
        let holder = e.mem.u32(pointed + 0x10c);
        let texture = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![holder, 0u32])
            .u32();
        e.mem.set_u32(block + 8, texture);
    }
    e.vcall(this, 0x108, &args![]);
}

/// Strings of `00bd3280` (read in Ghidra): the 2x vertex files `lighting\2x\v\AD.v.hlsl`,
/// `ADTS`, `DiffusePt`, `Specular` and `SimpleShadow` (each `.v.hlsl`), the 1x vertex files
/// `lighting\1x\v\base.v.hlsl`, `ambDiffuseDirTexture`, `ambDiffuseDirAndPt`, `diffuseDir`,
/// `diffusePt`, `specularDir` and `specularPt` (each `.v.hlsl`), the macros `LIGHTS`,
/// `PROJ_SHADOW`, `SPECULAR`, `POINT`, `NUM_PT_LIGHTS`, `SHADOWMAP`, `TREE`, the values `"2"`
/// and `"3"`, the name formats `"STB1%03i.vso"` and `"STB2%03i.vso"` and the profiles `vs_1_1`
/// and `vs_2_0`.
const AD_VERTEX_010AEB54: u32 = 0x010a_eb54;
const ADTS_VERTEX_010AEBF4: u32 = 0x010a_ebf4;
const DIFFUSE_PT_VERTEX_010AEB34: u32 = 0x010a_eb34;
const SPECULAR_VERTEX_010AEB14: u32 = 0x010a_eb14;
const SIMPLE_SHADOW_VERTEX_010AE968: u32 = 0x010a_e968;
const BASE_VERTEX_010AEDA8: u32 = 0x010a_eda8;
const AMB_DIFFUSE_DIR_TEXTURE_VERTEX_010AECE4: u32 = 0x010a_ece4;
const AMB_DIFFUSE_DIR_AND_PT_VERTEX_010AECB8: u32 = 0x010a_ecb8;
const DIFFUSE_DIR_VERTEX_010AED68: u32 = 0x010a_ed68;
const DIFFUSE_PT_1X_VERTEX_010AED88: u32 = 0x010a_ed88;
const SPECULAR_DIR_VERTEX_010AED10: u32 = 0x010a_ed10;
const SPECULAR_PT_VERTEX_010AED34: u32 = 0x010a_ed34;
const MACRO_LIGHTS_010AEB98: u32 = 0x010a_eb98;
const MACRO_PROJ_SHADOW_010AEBA0: u32 = 0x010a_eba0;
const MACRO_SPECULAR_010AEB8C: u32 = 0x010a_eb8c;
const MACRO_POINT_010AEB0C: u32 = 0x010a_eb0c;
const MACRO_NUM_PT_LIGHTS_010AEAFC: u32 = 0x010a_eafc;
const MACRO_SHADOWMAP_010AE95C: u32 = 0x010a_e95c;
const MACRO_TREE_0101C544: u32 = 0x0101_c544;
const VALUE_2_01063F3C: u32 = 0x0106_3f3c;
const VALUE_3_01063F30: u32 = 0x0106_3f30;
const FORMAT_STB1_VERTEX_010BC050: u32 = 0x010b_c050;
const FORMAT_STB2_VERTEX_010BC040: u32 = 0x010b_c040;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
const PROFILE_VS_2_0_010A93E4: u32 = 0x010a_93e4;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
/// The `00bd3280` frame (saved registers up to the cookie) and its name and path buffers.
const VERTEX_FRAME_BYTES: u32 = 0xb10;
const VERTEX_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0x904,
    path: 0xa08,
};
/// The `00bd3280` source blocks (0x4c bytes apart: twenty 2x blocks from +0x1c, ten 1x blocks
/// from +0x60c), in the exe's order: the 1x blocks first, then the 2x blocks (the one at
/// +0x360 written out in full, without a `memset`).
const VERTEX_SOURCES: [ShaderSource; 30] = [
    ShaderSource {
        offset: 0x60c,
        words: &[BASE_VERTEX_010AEDA8, MACRO_TREE_0101C544, EMPTY_01011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x658,
        words: &[
            AMB_DIFFUSE_DIR_TEXTURE_VERTEX_010AECE4,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            MACRO_VC_010AED64,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6a4,
        words: &[
            AMB_DIFFUSE_DIR_TEXTURE_VERTEX_010AECE4,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            MACRO_VC_010AED64,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6f0,
        words: &[
            AMB_DIFFUSE_DIR_AND_PT_VERTEX_010AECB8,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x73c,
        words: &[
            DIFFUSE_DIR_VERTEX_010AED68,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x788,
        words: &[
            DIFFUSE_PT_1X_VERTEX_010AED88,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x7d4,
        words: &[
            BASE_VERTEX_010AEDA8,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            MACRO_VC_010AED64,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x820,
        words: &[
            SPECULAR_DIR_VERTEX_010AED10,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x86c,
        words: &[
            SPECULAR_PT_VERTEX_010AED34,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x8b8,
        words: &[
            BASE_VERTEX_010AEDA8,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            MACRO_FOG_010AECE0,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1c,
        words: &[
            AD_VERTEX_010AEB54,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            AD_VERTEX_010AEB54,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            AD_VERTEX_010AEB54,
            MACRO_LIGHTS_010AEB98,
            VALUE_3_01063F30,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            AD_VERTEX_010AEB54,
            MACRO_LIGHTS_010AEB98,
            VALUE_3_01063F30,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[ADTS_VERTEX_010AEBF4, MACRO_TREE_0101C544, EMPTY_01011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1e4,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x230,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x27c,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_SPECULAR_010AEB8C,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2c8,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_SPECULAR_010AEB8C,
            EMPTY_01011584,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x314,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_SPECULAR_010AEB8C,
            EMPTY_01011584,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x360,
        words: &[
            ADTS_VERTEX_010AEBF4,
            MACRO_SPECULAR_010AEB8C,
            EMPTY_01011584,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
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
        offset: 0x3ac,
        words: &[
            DIFFUSE_PT_VERTEX_010AEB34,
            MACRO_LIGHTS_010AEB98,
            VALUE_2_01063F3C,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3f8,
        words: &[
            DIFFUSE_PT_VERTEX_010AEB34,
            MACRO_LIGHTS_010AEB98,
            VALUE_3_01063F30,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x444,
        words: &[
            SPECULAR_VERTEX_010AEB14,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x490,
        words: &[
            SPECULAR_VERTEX_010AEB14,
            MACRO_PROJ_SHADOW_010AEBA0,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x4dc,
        words: &[
            SPECULAR_VERTEX_010AEB14,
            MACRO_POINT_010AEB0C,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x528,
        words: &[
            SPECULAR_VERTEX_010AEB14,
            MACRO_NUM_PT_LIGHTS_010AEAFC,
            VALUE_2_01063F3C,
            MACRO_POINT_010AEB0C,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x574,
        words: &[
            SPECULAR_VERTEX_010AEB14,
            MACRO_NUM_PT_LIGHTS_010AEAFC,
            VALUE_3_01063F30,
            MACRO_POINT_010AEB0C,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5c0,
        words: &[
            SIMPLE_SHADOW_VERTEX_010AE968,
            MACRO_SHADOWMAP_010AE95C,
            EMPTY_01011584,
            MACRO_TREE_0101C544,
            EMPTY_01011584,
            0,
        ],
        cleared: 0x34,
    },
];

// Translated from 00bd3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (vtable `010bc070` slot +0x148): shader 4's vertex shaders, no stack
/// words. Fills thirty source blocks on its stack (the 1x lighting files with `TREE` and
/// `VC`/`FOG`, the 2x files with `LIGHTS`, `PROJ_SHADOW`, `SPECULAR`, `POINT`,
/// `NUM_PT_LIGHTS`, `SHADOWMAP` and `TREE`; the rest of each block cleared by `memset`). Then
/// compiles the ten 1x blocks (`"STB1%03i.vso"` with the index, `vs_1_1`) into the
/// `NiPointer`s +0x8c to +0xb0, a block without a file clearing its pointer instead; and at
/// shader model (`011f91c0`) 2 or more the twenty 2x blocks (`"STB2%03i.vso"`, `vs_2_0`, no
/// check for a file) into +0xb4 to +0x100.
// Translation incomplete: 00bd3280 - the /GS cookie check (00ec408c) of the stack frame is not translated.
pub fn fn_00bd3280(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(VERTEX_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &VERTEX_SOURCES);
        for index in 0..10u32 {
            compile_indexed_or_clear(
                e,
                this,
                frame,
                &VERTEX_BUFFERS,
                0x60c + 0x4c * index,
                this + 0x8c + 4 * index,
                FORMAT_STB1_VERTEX_010BC050,
                index,
                PROFILE_VS_1_1_010A93EC,
                false,
            );
        }
        if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            return;
        }
        for index in 0..20u32 {
            let shader = compile_indexed(
                e,
                this,
                frame,
                &VERTEX_BUFFERS,
                0x1c + 0x4c * index,
                FORMAT_STB2_VERTEX_010BC040,
                index,
                PROFILE_VS_2_0_010A93E4,
                false,
            );
            ni_pointer_assign(e, this + 0xb4 + 4 * index, shader);
        }
    });
}

// Translated from 00bd3f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bc190`), `thiscall`, no stack words: calls the
/// vtable +0x144 and +0x15c, then, from shader model 2 on (`011f91c0`, signed), jumps through the
/// vtable +0x160.
pub fn fn_00bd3f80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base, 0x144, &args![]);
    e.vcall(base, 0x15c, &args![]);
    if e.global::<i32>(SHADER_MODEL_011F91C0) >= 2 {
        e.vcall(base, 0x160, &args![]);
    }
}

// Translated from 00bd4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010bc194`), `thiscall`, no stack words: calls the
/// shader's vtable +0x128 with every word of the table `01202350`..`012023c8` (the loop tests
/// after each call, signed).
pub fn fn_00bd4000(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut slot = PASSES_01202350;
    loop {
        let pass = e.mem.u32(slot);
        e.vcall(base, 0x128, &args![pass]);
        slot += 4;
        if (slot as i32) >= (PASSES_END_012023C8 as i32) {
            break;
        }
    }
}
const TARGETS_00BD4254: [u32; 24] = [
    0x00bd_413d,
    0x00bd_4148,
    0x00bd_4159,
    0x00bd_4168,
    0x00bd_40e7,
    0x00bd_40f2,
    0x00bd_4103,
    0x00bd_4112,
    0x00bd_4071,
    0x00bd_4082,
    0x00bd_4091,
    0x00bd_409c,
    0x00bd_40ad,
    0x00bd_40bc,
    0x00bd_40c7,
    0x00bd_40d8,
    0x00bd_4173,
    0x00bd_4184,
    0x00bd_411d,
    0x00bd_412e,
    0x00bd_4066,
    0x00bd_4193,
    0x00bd_419b,
    0x00bd_4217,
];
const BYTES_00BD42B4: [u8; 250] = [
    0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 23, 1, 2, 23, 23, 3, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 4, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 23, 5, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 6, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    7, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 23, 23, 23, 23, 23, 23, 8, 23, 23, 23, 23, 23, 23, 23, 23, 9, 23, 23, 23, 23, 23, 23, 23,
    23, 10, 23, 23, 23, 23, 23, 23, 23, 23, 11, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    12, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 13, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 14,
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 15, 23, 23, 23, 23, 23, 16, 17, 23, 23, 23, 23, 23, 18,
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 19, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 20, 23, 23, 23, 23, 23, 23, 21, 22,
];
const TARGETS_00BD43B0: [u32; 7] = [
    0x00bd_41dd,
    0x00bd_41e9,
    0x00bd_41f1,
    0x00bd_41ff,
    0x00bd_420b,
    0x00bd_41cf,
    0x00bd_4217,
];
const BYTES_00BD43CC: [u8; 242] = [
    0, 6, 6, 6, 6, 6, 1, 6, 6, 6, 6, 6, 2, 6, 6, 6, 6, 6, 3, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5,
];
const CASES_00BD4030: [(u32, u32); 30] = [
    (0x00bd_4066, 0x0120_2368),
    (0x00bd_4071, 0x0120_2378),
    (0x00bd_4082, 0x0120_237c),
    (0x00bd_4091, 0x0120_2380),
    (0x00bd_409c, 0x0120_2384),
    (0x00bd_40ad, 0x0120_2388),
    (0x00bd_40bc, 0x0120_238c),
    (0x00bd_40c7, 0x0120_2390),
    (0x00bd_40d8, 0x0120_2394),
    (0x00bd_40e7, 0x0120_2398),
    (0x00bd_40f2, 0x0120_239c),
    (0x00bd_4103, 0x0120_23a0),
    (0x00bd_4112, 0x0120_23a4),
    (0x00bd_411d, 0x0120_23a8),
    (0x00bd_412e, 0x0120_23ac),
    (0x00bd_413d, 0x0120_2350),
    (0x00bd_4148, 0x0120_2354),
    (0x00bd_4159, 0x0120_2358),
    (0x00bd_4168, 0x0120_235c),
    (0x00bd_4173, 0x0120_2360),
    (0x00bd_4184, 0x0120_2364),
    (0x00bd_4193, 0x0120_236c),
    (0x00bd_419b, 0x0120_2370),
    (0x00bd_41a9, 0x0120_23b0),
    (0x00bd_41cf, 0x0120_2374),
    (0x00bd_41dd, 0x0120_23b4),
    (0x00bd_41e9, 0x0120_23b8),
    (0x00bd_41f1, 0x0120_23bc),
    (0x00bd_41ff, 0x0120_23c0),
    (0x00bd_420b, 0x0120_23c4),
];

// Translated from 00bd4030 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00bd4030 to the `RET 4` at 00bd4250, its jump tables after
/// it): slot 60 (+0xf0) of the vtable `010bc070`, `thiscall` with one stack word: the pass. The
/// pass 0x15f and the passes 0x60 to 0x159 and 0x165 to 0x256 with a case in the jump tables
/// (read from the exe) put their pass word (`01202350` to `012023c4`) in `0126f74c`; then for
/// every pass the `NiPointer` at +0x28 takes the shader's +0x6c (inlined assignment).
pub fn fn_00bd4030(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let target = if pass > 0x15f {
        let case = pass.wrapping_sub(0x165);
        if case > 0xf1 {
            0x00bd_4217
        } else {
            TARGETS_00BD43B0[usize::from(BYTES_00BD43CC[case as usize])]
        }
    } else if pass == 0x15f {
        0x00bd_41a9
    } else {
        let case = pass.wrapping_sub(0x60);
        if case > 0xf9 {
            0x00bd_4217
        } else {
            TARGETS_00BD4254[usize::from(BYTES_00BD42B4[case as usize])]
        }
    };
    // 00bd4217 (the default) goes straight to the assignment.
    if let Some(global) = CASES_00BD4030
        .iter()
        .find(|(address, _)| *address == target)
        .map(|(_, global)| *global)
    {
        let word = e.global::<u32>(global);
        e.set_global::<u32>(PASS_STATE_0126F74C, word);
    }
    let shader = e.mem.u32(base + 0x6c);
    ni_pointer_assign(e, base + 0x28, shader);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_3f80, fn_00bd3f80(Ptr)),
        entry!(0x00bd_4000, fn_00bd4000(Ptr)),
        entry!(0x00bd_4030, fn_00bd4030(Ptr, u32)),
        entry!(0x00bd_44c0, fn_00bd44c0(Ptr, Ptr) -> Ptr),
        entry!(0x00bd_4750, fn_00bd4750() -> Ptr),
        entry!(0x00bd_3270, fn_00bd3270(Ptr, u32) -> u32),
        entry!(0x00bd_3c60, fn_00bd3c60(Ptr)),
        entry!(0x00bd_3280, fn_00bd3280(Ptr)),
        entry!(0x00bd_3fc0, fn_00bd3fc0(Ptr, Ptr) -> bool),
        entry!(0x00bd_4560, fn_00bd4560() -> Ptr),
        entry!(0x00bd_4570, fn_00bd4570(Ptr)),
        entry!(0x00bd_46a0, fn_00bd46a0(Ptr) -> bool),
        entry!(0x00bd_4890, fn_00bd4890(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd4750_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD44C0, &[]);
        let (shader, log) = run(&mut e, 0x00bd_4750);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 6, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (declaration_slot(0x8c), vec![d, 3, 0xe, 0xd, 2, 0]),
                (declaration_slot(0x8c), vec![d, 4, 0xf, 0xe, 2, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 5, 0xa, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x12c]),
                (SHADER_CONSTRUCT_00BD44C0, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x144), vec![SHADER]),
                (shader_slot(0x150), vec![SHADER]),
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
    fn fn_00bd44c0_builds_the_shader() {
        let mut e = rig();
        e.register(SHADER_BASE_CONSTRUCT_00B79B00, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x12c);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bd_44c0, &args![this, declaration]).u32(), this);
        assert_eq!(
            calls(&e, SHADER_BASE_CONSTRUCT_00B79B00),
            vec![vec![this, declaration, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010BC070);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 0x1e, DESTRUCT_00B66F20),
                pointer_array(this + 0x104, 0xa, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 5);
    }
}

#[cfg(test)]
mod shader_four_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn load_vertex(model: i32) -> (Engine, u32) {
        let mut e = loader_rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x12c);
        e.call(0x00bd_3280, &args![this]);
        (e, this)
    }

    #[test]
    fn fn_00bd3280_compiles_the_1x_and_2x_vertex_shaders() {
        let (e, this) = load_vertex(2);
        let first = e.mem.u32(this + 0x8c);
        assert_eq!(
            compiled(&e, first),
            Compiled {
                file: BASE_VERTEX_010AEDA8,
                output: FORMAT_STB1_VERTEX_010BC050,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                macros: macro_words(&[(MACRO_TREE_0101C544, EMPTY_01011584)]),
            }
        );
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x8c + 4 * 9)).macros,
            macro_words(&[
                (MACRO_TREE_0101C544, EMPTY_01011584),
                (MACRO_FOG_010AECE0, EMPTY_01011584)
            ])
        );
        let full = e.mem.u32(this + 0xb4 + 4 * 11);
        assert_eq!(
            compiled(&e, full),
            Compiled {
                file: ADTS_VERTEX_010AEBF4,
                output: FORMAT_STB2_VERTEX_010BC040,
                profile: PROFILE_VS_2_0_010A93E4,
                pixel: false,
                macros: macro_words(&[
                    (MACRO_SPECULAR_010AEB8C, EMPTY_01011584),
                    (MACRO_LIGHTS_010AEB98, VALUE_2_01063F3C),
                    (MACRO_PROJ_SHADOW_010AEBA0, EMPTY_01011584),
                    (MACRO_TREE_0101C544, EMPTY_01011584)
                ]),
            }
        );
        assert_eq!(
            compiled(&e, e.mem.u32(this + 0x100)).file,
            SIMPLE_SHADOW_VERTEX_010AE968
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        let expected: Vec<u32> = (0..10).chain(0..20).collect();
        assert_eq!(indices, expected);
        assert_eq!(calls(&e, MEMSET).len(), 29);
    }

    #[test]
    fn fn_00bd3280_skips_the_2x_shaders_below_model_2() {
        let (e, this) = load_vertex(1);
        assert_eq!(calls(&e, FORMAT_00406D00).len(), 10);
        assert_ne!(e.mem.u32(this + 0x8c), 0);
        assert!((0..20).all(|i| e.mem.u32(this + 0xb4 + 4 * i) == 0));
    }

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
    fn fn_00bd3270_jumps_to_setup_pass_constants() {
        let mut e = rig();
        e.register(SHADOW_LIGHT_SETUP_PASS_CONSTANTS, |_, a| {
            (a[1] + 1).into_ret()
        });
        assert_eq!(e.call(0x00bd_3270, &args![0x1000u32, 0x41u32]).u32(), 0x42);
        assert_eq!(
            calls(&e, SHADOW_LIGHT_SETUP_PASS_CONSTANTS),
            vec![vec![0x1000, 0x41]]
        );
    }

    #[test]
    fn fn_00bd3c60_compiles_the_ten_pixel_shaders() {
        let mut e = loader_rig();
        e.map(0x011a_d000, 0x1000);
        e.set_global(SPECULAR_VALUE_011ADBCC, 0.5f32);
        e.register(GCVT, |e, a| {
            assert_eq!(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32), 0.5);
            assert_eq!(a[2], 0xc);
            e.mem.set_u32(a[3], 0x2e30);
            a[3].into_ret()
        });
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e55_0000u32.into_ret());
        let this = e.mem.alloc(0x12c);
        // An old shader in the last slot is released for the new one.
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x104 + 4 * 9, old);
        e.call(0x00bd_3c60, &args![this]);
        let number = calls(&e, GCVT)[0][3];
        let shaders: Vec<u32> = (0..10).map(|i| e.mem.u32(this + 0x104 + 4 * i)).collect();
        assert_eq!(
            compiled(&e, shaders[0]),
            Compiled {
                file: BASE_PIXEL_010AF2D4,
                output: FORMAT_STB1_PIXEL_010BC060,
                profile: 0x0e55_0000,
                pixel: true,
                macros: macro_words(&[]),
            }
        );
        assert_eq!(
            compiled(&e, shaders[2]).macros,
            macro_words(&[(MACRO_VC_010AED64, EMPTY_01011584)])
        );
        assert_eq!(
            compiled(&e, shaders[6]).macros,
            macro_words(&[
                (MACRO_TEX_010AECAC, EMPTY_01011584),
                (MACRO_VC_010AED64, EMPTY_01011584)
            ])
        );
        assert_eq!(compiled(&e, shaders[8]).file, SPECULAR_PT_PIXEL_010AF268);
        assert_eq!(
            compiled(&e, shaders[8]).macros,
            macro_words(&[(MACRO_SPECMULT_010AF288, number)])
        );
        assert_eq!(
            compiled(&e, shaders[9]).macros,
            macro_words(&[(MACRO_FOG_010AECE0, EMPTY_01011584)])
        );
        let indices: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|a| a[3]).collect();
        assert_eq!(indices, (0..10).collect::<Vec<u32>>());
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert_eq!(calls(&e, MEMSET).len(), 10);
    }

    #[test]
    fn fn_00bd3fc0_sets_the_property_and_runs_its_slot() {
        let mut e = rig();
        let table = vtable(&mut e);
        noop(&mut e, &[SLOTS + 0x9c]);
        let property = e.mem.alloc(0x60);
        e.mem.set_u32(property, table);
        e.mem.set_u32(property + 0x1c, 0xa);
        e.register_double(GET_PROPERTY, move |_, a| {
            assert_eq!(a[1], 3);
            property.into_ret()
        });
        let geometry = e.mem.alloc(0x10);
        assert!(e.call(0x00bd_3fc0, &args![0u32, geometry]).bool());
        assert_eq!(e.mem.u32(property + 0x58), 4);
        assert_eq!(calls(&e, SLOTS + 0x9c), vec![vec![property, geometry]]);
        // Another kind of property: the code goes on with the null pointer (page 0 mapped
        // here with the test vtable).
        e.mem.set_u32(property + 0x1c, 9);
        e.map(0, 0x1000);
        e.mem.set_u32(0, table);
        e.call(0x00bd_3fc0, &args![0u32, geometry]);
        assert_eq!(e.mem.u32(0x58), 4);
        assert_eq!(calls(&e, SLOTS + 0x9c)[1], vec![0, geometry]);
    }

    #[test]
    fn fn_00bd4560_answers_its_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bd_4560, &args![]).u32(), RECORD_01202348);
    }

    fn shader_with_pointers(e: &mut Engine) -> (u32, u32, u32, u32) {
        e.map(0x0120_2000, 0x1000);
        let this = e.mem.alloc(0x12c);
        let first = object(e, 1);
        let last = object(e, 2);
        e.mem.set_u32(this + 0x8c, first);
        e.mem.set_u32(this + 0x104 + 4 * 9, last);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.set_global(PASSES_01202350 + 4, pass);
        (this, first, last, pass)
    }

    #[test]
    fn fn_00bd4570_releases_everything_and_destroys() {
        let mut e = rig();
        noop(&mut e, &[VECTOR_DESTRUCT, BASE_DESTRUCT_00B79DE0]);
        let (this, first, last, pass) = shader_with_pointers(&mut e);
        e.call(0x00bd_4570, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BC070);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        assert_eq!(e.mem.u32(this + 0x104 + 4 * 9), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(last + 4), 1);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.global::<u32>(PASSES_01202350 + 4), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x104, 4, 0xa, DESTRUCT_00B66F20],
                vec![this + 0x8c, 4, 0x1e, DESTRUCT_00B66F20],
            ]
        );
        assert_eq!(calls(&e, BASE_DESTRUCT_00B79DE0), vec![vec![this]]);
    }

    #[test]
    fn fn_00bd46a0_clears_the_pointers_and_answers_true() {
        let mut e = rig();
        noop(&mut e, &[CALL_00BE0870]);
        let (this, first, _, pass) = shader_with_pointers(&mut e);
        // A pass still held elsewhere keeps living.
        e.mem.set_u32(pass + 0x64, 2);
        assert!(e.call(0x00bd_46a0, &args![this]).bool());
        assert_eq!(calls(&e, CALL_00BE0870), vec![vec![this]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![first]]);
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        assert_eq!(e.global::<u32>(PASSES_01202350 + 4), 0);
    }

    /// The pass state with eight constant blocks, a shader whose slot +0x108 is a double, and
    /// an object whose +0xac points at 0x6666.
    fn pass_rig(pass: u32) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = rig();
        for page in [0x011f_9000u32, 0x0126_f000] {
            e.map(page, 0x1000);
        }
        let state = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x20);
        let blocks: Vec<u32> = (0..8).map(|_| e.mem.alloc(0xc)).collect();
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *block);
        }
        e.mem.set_u32(state + 0x24, table);
        e.set_global(PASS_STATE_0126F74C, state);
        e.set_global(CURRENT_PASS_011F91E4, pass);
        e.set_global(CONSTANTS_011F95E4, 0xaaaau32);
        e.set_global(CONSTANTS_011F95E8, 0xbbbbu32);
        let this = e.mem.alloc(0x12c);
        let table = vtable(&mut e);
        e.mem.set_u32(this, table);
        noop(&mut e, &[SLOTS + 0x108]);
        e.register(TEXTURE_FOR_00C03230, |_, _| 0x7777u32.into_ret());
        let object = e.mem.alloc(0xb0);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x6666);
        e.mem.set_u32(object + 0xac, holder);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, object);
        (e, this, source, blocks)
    }

    fn block_values(e: &Engine, blocks: &[u32]) -> Vec<u32> {
        blocks.iter().map(|b| e.mem.u32(b + 8)).collect()
    }

    #[test]
    fn fn_00bd4890_fills_the_blocks_by_pass() {
        let cases: [(u32, [u32; 8]); 6] = [
            (0x7a, [0x6666, 0x7777, 0, 0, 0, 0, 0, 0]),
            (0xaa, [0x6666, 0x7777, 0, 0, 0, 0, 0xaaaa, 0xbbbb]),
            (0xe8, [0x6666, 0x7777, 0, 0, 0, 0xaaaa, 0xbbbb, 0]),
            (0x165, [0x7777, 0xaaaa, 0xbbbb, 0, 0, 0, 0, 0]),
            (0x151, [0x6666, 0, 0, 0, 0, 0, 0, 0]),
            (0x178, [0; 8]),
        ];
        for (pass, expected) in cases {
            let (mut e, this, source, blocks) = pass_rig(pass);
            e.call(0x00bd_4890, &args![this, source]);
            assert_eq!(
                block_values(&e, &blocks),
                expected.to_vec(),
                "pass {pass:#x}"
            );
            assert_eq!(calls(&e, SLOTS + 0x108), vec![vec![this]]);
        }
    }

    #[test]
    fn fn_00bd4890_pass_0x234_takes_the_rendered_texture() {
        let (mut e, this, source, blocks) = pass_rig(0x234);
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, a| (a[0] + 1).into_ret());
        let record = e.mem.alloc(0x10);
        let property = e.mem.alloc(4);
        let pointed = e.mem.alloc(0x110);
        e.mem.set_u32(pointed + 0x10c, 0x4000);
        e.mem.set_u32(property, pointed);
        e.mem.set_u32(record + 0xc, property);
        e.set_global(CURRENT_RECORD_011F91E0, record);
        e.call(0x00bd_4890, &args![this, source]);
        assert_eq!(
            block_values(&e, &blocks),
            vec![0x6666, 0, 0, 0, 0, 0, 0x4001, 0]
        );
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![vec![0x4000, 0]]
        );
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{
        calls, engine, order, slot_target, vobject,
    };

    #[test]
    fn fn_00bd3f80_by_shader_model() {
        let mut e = engine(&[]);
        e.map(0x011f_9000, 0x1000);
        let this = vobject(&mut e, 8, &[]);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        e.call(0x00bd_3f80, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bd_3f80,
                slot_target(&e, this, 0x144),
                slot_target(&e, this, 0x15c)
            ]
        );
        e.set_global(SHADER_MODEL_011F91C0, 2i32);
        e.call_log = Some(vec![]);
        e.call(0x00bd_3f80, &args![this]);
        assert_eq!(order(&e)[3], slot_target(&e, this, 0x160));
    }

    #[test]
    fn fn_00bd4000_releases_the_table() {
        let mut e = engine(&[]);
        e.map(0x0120_2000, 0x1000);
        let this = vobject(&mut e, 8, &[]);
        e.set_global(PASSES_01202350, 5u32);
        e.set_global(PASSES_END_012023C8 - 4, 6u32);
        e.call(0x00bd_4000, &args![this]);
        let seen = calls(&e, slot_target(&e, this, 0x128));
        assert_eq!(seen.len(), 0x1e);
        assert_eq!((seen[0][1], seen[0x1d][1]), (5, 6));
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00bd4030_sets_the_pass_word_and_the_shader_pointer() {
        let mut e = Engine::new();
        for (import, step) in [(0x00fd_f1c0u32, 1u32), (0x00fd_f1c4, u32::MAX)] {
            e.register_double(import, move |e, a| {
                let v = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], v);
                v.into_ret()
            });
        }
        e.map(0x0120_2000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        for global in (0x0120_2350u32..=0x0120_23c4).step_by(4) {
            e.set_global::<u32>(global, global & 0xfff);
        }
        let shader = e.mem.alloc(8);
        e.mem.set_u32(shader + 4, 1);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x6c, shader);
        let cases: [(u32, u32); 30] = [
            (0x60, 0x350),
            (0x7a, 0x354),
            (0x7b, 0x358),
            (0x7e, 0x35c),
            (0x9f, 0x398),
            (0xaa, 0x39c),
            (0xb5, 0x3a0),
            (0xc0, 0x3a4),
            (0xdf, 0x378),
            (0xe8, 0x37c),
            (0xf1, 0x380),
            (0xfa, 0x384),
            (0x108, 0x388),
            (0x114, 0x38c),
            (0x11f, 0x390),
            (0x12a, 0x394),
            (0x130, 0x360),
            (0x131, 0x364),
            (0x137, 0x3a8),
            (0x143, 0x3ac),
            (0x151, 0x368),
            (0x158, 0x36c),
            (0x159, 0x370),
            (0x15f, 0x3b0),
            (0x165, 0x3b4),
            (0x16b, 0x3b8),
            (0x171, 0x3bc),
            (0x177, 0x3c0),
            (0x234, 0x3c4),
            (0x256, 0x374),
        ];
        for (pass, word) in cases {
            e.set_global::<u32>(PASS_STATE_0126F74C, 0);
            e.call(0x00bd_4030, &args![this, pass]);
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), word, "{pass:#x}");
        }
        // The pointer took the shader once (one reference added).
        assert_eq!(e.mem.u32(this + 0x28), shader);
        assert_eq!(e.mem.u32(shader + 4), 2);
        // Passes with no case still assign the pointer.
        e.mem.set_u32(this + 0x28, 0);
        for pass in [0x5fu32, 0x61, 0x15a, 0x160, 0x166, 0x257, 0xffff_ffff] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 7);
            e.call(0x00bd_4030, &args![this, pass]);
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 7, "{pass:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x28), shader);
        assert_eq!(e.mem.u32(shader + 4), 3);
    }
}
