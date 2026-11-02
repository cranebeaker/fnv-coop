//! `BSShader/run_00bcc570` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The constructors of the three shaders made here (unnamed, `thiscall`): `00bd12c0` and
/// `00bcf7f0` (`RET 8`: two declarations), `00bcd390` (`RET 4`: one declaration).
const SHADER_CONSTRUCT_00BD12C0: u32 = 0x00bd_12c0;
const SHADER_CONSTRUCT_00BCF7F0: u32 = 0x00bc_f7f0;
const SHADER_CONSTRUCT_00BCD390: u32 = 0x00bc_d390;

/// The six entries (vtable slot +0x8c, five words each) of the six-entry declarations here.
const SIX_ENTRIES: [[u32; 5]; 6] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 5, 4, 3, 0],
    [3, 7, 5, 1, 0],
    [4, 0xe, 0xd, 2, 0],
    [5, 0xf, 0xe, 2, 0],
];
/// The eight entries of the second declaration of `00bd14b0` and `00bcfb00`.
const EIGHT_ENTRIES: [[u32; 5]; 8] = [
    [0, 0, 0, 2, 0],
    [1, 1, 1, 3, 0],
    [2, 2, 2, 4, 0],
    [3, 3, 3, 2, 0],
    [4, 5, 4, 3, 0],
    [5, 7, 5, 1, 0],
    [6, 0xe, 0xd, 2, 0],
    [7, 0xf, 0xe, 2, 0],
];

/// The shared body of `00bd14b0` and `00bcfb00` (each a separate function in the exe, with
/// the same code but for the size and the constructor): two declarations (6 and 8 entries,
/// 1 stream each) made one after the other, then filled and ended (slot +0xb4) in that
/// order; the shader made with both, its slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0
/// with the first declaration.
fn two_declaration_shader(e: &mut Engine, size: u32, construct: u32) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 8u32, 1u32])
        .u32();
    for entry in SIX_ENTRIES {
        e.vcall(first, 0x8c, &entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in EIGHT_ENTRIES {
        e.vcall(second, 0x8c, &entry);
    }
    e.vcall(second, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![size]).u32();
    let shader = if block != 0 {
        e.call(construct, &args![block, first, second]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

// Translated from 00bd14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xd. No arguments. Makes two declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`: 6 entries and 8 entries, 1 stream each), sets the first's entries
/// through its vtable slot +0x8c ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7,
/// 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0)) and ends it (slot +0xb4), then the
/// second's ((0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0), (4, 5, 4, 3,
/// 0), (5, 7, 5, 1, 0), (6, 0xe, 0xd, 2, 0), (7, 0xf, 0xe, 2, 0)) and ends it. Allocates 0xec
/// bytes and runs the constructor `00bd12c0` with both (a null allocation is used as it is),
/// calls the shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the first
/// declaration, and answers the shader.
// Translation incomplete: 00bd14b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd14b0(e: &mut Engine) -> Ptr {
    two_declaration_shader(e, 0xec, SHADER_CONSTRUCT_00BD12C0)
}

// Translated from 00bcfb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xe. The same steps as `00bd14b0` (two declarations of 6 and 8 entries with the
/// same entries), with 0x128 bytes and the constructor `00bcf7f0`.
// Translation incomplete: 00bcfb00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcfb00(e: &mut Engine) -> Ptr {
    two_declaration_shader(e, 0x128, SHADER_CONSTRUCT_00BCF7F0)
}

// Translated from 00bcd680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xf. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 6 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (the six of `00bd14b0`'s first declaration) and calls slot +0xb4. Then allocates 0x160
/// bytes and runs the constructor `00bcd390` with the declaration (a null allocation is used
/// as it is), calls the shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the
/// declaration, and answers the shader.
// Translation incomplete: 00bcd680 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcd680(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    for entry in SIX_ENTRIES {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x160u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BCD390, &args![block, declaration])
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
/// The vtables of the three shaders (`010bb7a8`, `010bb980`, `010bbb50`; no RTTI locators
/// before them).
const VTABLE_010BB7A8: u32 = 0x010b_b7a8;
const VTABLE_010BB980: u32 = 0x010b_b980;
const VTABLE_010BBB50: u32 = 0x010b_bb50;

// Translated from 00bcd390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bcd680` runs,
/// `thiscall` with one stack word (the declaration). Runs `00b79b00` with (the declaration,
/// 0, 0, 0), stores the vtable `010bb7a8`, constructs the `NiPointer` arrays at +0x8c (0x14)
/// and +0xdc (0x21; both destructor `00b66f20`), sets +0x68 to 3 and clears the byte +0x64,
/// assigns the declaration to the `NiPointer` at +0x6c, clears the `NiPointer` at +0x70 and
/// the word +0x60. Answers `this`.
// Translation incomplete: 00bcd390 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bcd390(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, declaration, 0u32, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BB7A8);
    construct_pointer_array(e, base + 0x8c, 0x14, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xdc, 0x21, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 3);
    e.mem.set_u8(base + 0x64, 0);
    ni_pointer_assign(e, base + 0x6c, declaration.addr());
    ni_pointer_clear(e, base + 0x70);
    e.mem.set_u32(base + 0x60, 0);
    this
}

// Translated from 00bcf7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bcfb00` runs,
/// `thiscall` with two stack words (the two declarations). Runs `00b79b00` with (first,
/// second, 0, 0), stores the vtable `010bb980`, constructs the `NiPointer` arrays at +0x8c
/// (0x1a) and +0xf4 (0xd; both destructor `00b66f20`), sets +0x68 to 4 and clears the byte
/// +0x64, assigns the declarations to the `NiPointer`s at +0x6c and +0x70 and clears the word
/// +0x60. Answers `this`.
// Translation incomplete: 00bcf7f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bcf7f0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, first, second, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BB980);
    construct_pointer_array(e, base + 0x8c, 0x1a, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xf4, 0xd, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 4);
    e.mem.set_u8(base + 0x64, 0);
    ni_pointer_assign(e, base + 0x6c, first.addr());
    ni_pointer_assign(e, base + 0x70, second.addr());
    e.mem.set_u32(base + 0x60, 0);
    this
}

// Translated from 00bd12c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bd14b0` runs,
/// `thiscall` with two stack words (the two declarations). Runs `00b79b00` with (first,
/// second, 0, 0), stores the vtable `010bbb50`, constructs the `NiPointer` arrays at +0x8c (2,
/// destructor `00b79930`), +0x94 (7), +0xb0 (3), +0xbc (7) and +0xd8 (3; destructor
/// `00b66f20`), clears the words +0xe4 and +0xe8, sets +0x68 to 2, assigns the declarations to
/// the `NiPointer`s at +0xe4 and +0xe8, then the object +0xe8 holds to the `NiPointer` at
/// +0x28, and clears the word +0x60. Answers `this`.
// Translation incomplete: 00bd12c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the base and the arrays) is not translated.
pub fn fn_00bd12c0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, first, second, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BBB50);
    construct_pointer_array(e, base + 0x8c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x94, 7, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xb0, 3, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xbc, 7, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xd8, 3, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0xe4, 0);
    e.mem.set_u32(base + 0xe8, 0);
    e.mem.set_u32(base + 0x68, 2);
    ni_pointer_assign(e, base + 0xe4, first.addr());
    ni_pointer_assign(e, base + 0xe8, second.addr());
    let kept = e.mem.u32(base + 0xe8);
    ni_pointer_assign(e, base + 0x28, kept);
    e.mem.set_u32(base + 0x60, 0);
    this
}

/// The shader quality word (`011f91c0`).
const SHADER_LEVEL_011F91C0: u32 = 0x011f_91c0;
/// The pass pointer that `fn_00bcce40` and `fn_00bcfd20` set (`0126f74c`).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
/// `00aa1460` (`cdecl`, block, size): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The shared shader destructor `00b79de0` and the shared reinitializer `00be0870` (both
/// unnamed, `thiscall`, no stack words).
const SHADER_BASE_DESTRUCT_00B79DE0: u32 = 0x00b7_9de0;
const SHADER_REINITIALIZE_00BE0870: u32 = 0x00be_0870;
/// The pass setters of `shadowlightshader.cpp` the `SetupTextures`-like functions call
/// (`00b7c150`, `00b7c170`, `00b7cfb0`, unnamed, `thiscall`, one stack word).
const SET_FIRST_STAGE_00B7C150: u32 = 0x00b7_c150;
const SET_LAST_STAGE_00B7C170: u32 = 0x00b7_c170;
const SET_STAGES_00B7CFB0: u32 = 0x00b7_cfb0;
/// `00b7af80` (unnamed, `thiscall`, one stack word), `BSShaderManager::GetShader` (`00b55560`,
/// `cdecl`, one word), `00a5f860` (unnamed, `thiscall` on an `NiPointer`, one stack word) and
/// `00c663d0` (unnamed, engine map unit `bhkworld`, `thiscall`, one stack word).
const SET_PASS_00B7AF80: u32 = 0x00b7_af80;
const GET_SHADER: u32 = 0x00b5_5560;
const POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
const COPY_00C663D0: u32 = 0x00c6_63d0;
/// `004dc060` (unnamed, engine map unit `renderer.cpp`, `cdecl`, one word).
const RENDERER_QUERY_004DC060: u32 = 0x004d_c060;

/// One numbered compile loop of the shader loaders: for each index, the block at
/// `first_file + 0x4c * index` of the frame (its file name word, then its macros), skipped
/// when `skip_null` and the file name is null; `MakePath` into the path buffer, the format
/// (with the index) into the name buffer, the compile and the result assigned to the shader's
/// `NiPointer` at `slot + 4 * index`.
struct NumberedCompile {
    first_file: u32,
    slot: u32,
    format: u32,
    profile: u32,
    pixel: bool,
    path: u32,
    name: u32,
    skip_null: bool,
}

fn compile_numbered(
    e: &mut Engine,
    this: u32,
    frame: u32,
    step: &NumberedCompile,
    indices: std::ops::Range<u32>,
) {
    for index in indices {
        let block = frame + step.first_file + 0x4c * index;
        let file = e.mem.u32(block);
        if step.skip_null && file == 0 {
            continue;
        }
        let path = frame + step.path;
        let name = frame + step.name;
        e.call(MAKE_PATH, &args![file, path]);
        e.call(FORMAT_00406D00, &args![name, 0x104u32, step.format, index]);
        let compile = if step.pixel {
            COMPILE_PIXEL_SHADER
        } else {
            COMPILE_VERTEX_SHADER
        };
        let shader = e
            .call(compile, &args![this, path, block + 4, step.profile, name])
            .u32();
        ni_pointer_assign(e, this + step.slot + 4 * index, shader);
    }
}

/// The source blocks `fn_00bcc750` writes (frame offsets from the frame base after the four
/// saved registers), in the exe's order.
const SOURCES_00BCC750: [ShaderSource; 20] = [
    ShaderSource {
        offset: 0x1c,
        words: &[0x010a_ebf4, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            0x010a_ebf4,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            0x010a_ebf4,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            0x010a_ebf4,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[
            0x010a_ebf4,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            0x010a_ebf4,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1e4,
        words: &[
            0x010a_ebf4,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x27c,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2c8,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x314,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x360,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3ac,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3f8,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x444,
        words: &[0x010b_b784, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x490,
        words: &[0x010a_eb14, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x4dc,
        words: &[
            0x010a_eb14,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x528,
        words: &[
            0x010a_eb14,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x574,
        words: &[
            0x010a_eb14,
            0x010a_eafc,
            0x0106_3f3c,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5c0,
        words: &[
            0x010a_eb14,
            0x010a_eafc,
            0x0106_3f30,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x230,
        words: &[
            0x010a_ebf4,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
];
/// The source blocks of `fn_00bcd7c0`.
const SOURCES_00BCD7C0: [ShaderSource; 33] = [
    ShaderSource {
        offset: 0x20,
        words: &[0x010a_f114, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6c,
        words: &[
            0x010a_f114,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eb6c,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb8,
        words: &[
            0x010a_f114,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x104,
        words: &[
            0x010a_f114,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x150,
        words: &[
            0x010a_f114,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x19c,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1e8,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x234,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x2cc,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x318,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x364,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3fc,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x448,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x494,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x52c,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x578,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5c4,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x65c,
        words: &[
            0x010a_f090,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6a8,
        words: &[
            0x010a_f090,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6f4,
        words: &[0x010b_b92c, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x740,
        words: &[0x010a_f070, 0x010b_a0bc, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x78c,
        words: &[
            0x010a_f070,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x7d8,
        words: &[
            0x010a_f070,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x824,
        words: &[
            0x010a_f070,
            0x010a_eafc,
            0x0106_3f3c,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x870,
        words: &[
            0x010a_f070,
            0x010a_eafc,
            0x0106_3f30,
            0x010a_eb0c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x8bc,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x280,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x3b0,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x4e0,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x610,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x908,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x954,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x9a0,
        words: &[
            0x010a_f114,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010b_a0bc,
            0x0101_1584,
            0x010a_ec48,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
];
/// The source blocks of `fn_00bce950`.
const SOURCES_00BCE950: [ShaderSource; 26] = [
    ShaderSource {
        offset: 0x1c,
        words: &[0x010a_ebf4, 0x010a_ed54, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            0x010a_ebf4,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            0x010a_ebf4,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            0x010a_ebf4,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[
            0x010a_ebf4,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            0x010a_ebf4,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1e4,
        words: &[
            0x010a_ebf4,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x27c,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2c8,
        words: &[
            0x010a_eb54,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x314,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3ac,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3f8,
        words: &[
            0x010a_eb54,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x444,
        words: &[
            0x010a_eb54,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x4dc,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x528,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x574,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x5c0,
        words: &[
            0x010a_eb34,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x60c,
        words: &[
            0x010a_eb70,
            0x010a_eb98,
            0x0106_3ee8,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x658,
        words: &[
            0x010a_eb70,
            0x010a_eb98,
            0x0106_3ee8,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x6a4,
        words: &[
            0x010a_eb70,
            0x010a_eb98,
            0x0106_3f24,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6f0,
        words: &[
            0x010a_eb70,
            0x010a_eb98,
            0x0106_3f24,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
            0,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x73c,
        words: &[
            0x010a_eb70,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f24,
            0x010a_ed54,
            0,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x230,
        words: &[
            0x010a_ebf4,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x360,
        words: &[
            0x010a_eb54,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x490,
        words: &[
            0x010a_eb54,
            0x010a_ec10,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
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
    ShaderSource {
        offset: 0x788,
        words: &[
            0x010a_eb70,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f24,
            0x010a_ec10,
            0x0101_1584,
            0x010a_ed54,
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
];
/// The source blocks of `fn_00bcf320`.
const SOURCES_00BCF320: [ShaderSource; 13] = [
    ShaderSource {
        offset: 0x1c,
        words: &[0x010a_f114, 0x010a_ed54, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            0x010a_f114,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            0x010a_f114,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1e4,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x230,
        words: &[
            0x010a_f0b0,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0x010a_eba0,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x27c,
        words: &[
            0x010a_f090,
            0x010a_eb98,
            0x0106_3f3c,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2c8,
        words: &[
            0x010a_f090,
            0x010a_eb98,
            0x0106_3f30,
            0x010a_ed54,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x314,
        words: &[0x010a_f0c8, 0x010a_eb98, 0x0106_3ee8, 0x010a_ed54, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x360,
        words: &[0x010a_f0c8, 0x010a_eb98, 0x0106_3f24, 0x010a_ed54, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3ac,
        words: &[
            0x010a_f0c8,
            0x010a_eb8c,
            0x0101_1584,
            0x010a_eb98,
            0x0106_3f24,
            0x010a_ed54,
            0,
            0,
        ],
        cleared: 0x2c,
    },
];
/// The source blocks of `fn_00bd0540`.
const SOURCES_00BD0540: [ShaderSource; 14] = [
    ShaderSource {
        offset: 0x230,
        words: &[0x010b_bb38, 0x010b_bb30, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x27c,
        words: &[
            0x010b_bb38,
            0x010b_bb30,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2c8,
        words: &[
            0x010b_bb38,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb20,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x314,
        words: &[
            0x010b_bb38,
            0x010b_bb30,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x360,
        words: &[
            0x010b_bb38,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb20,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3ac,
        words: &[
            0x010b_bb38,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3f8,
        words: &[
            0x010b_bb38,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1c,
        words: &[0x010b_bb08, 0x010b_bb30, 0x0101_1584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x68,
        words: &[
            0x010b_bb08,
            0x010b_bb30,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb4,
        words: &[
            0x010b_bb08,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb20,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x100,
        words: &[
            0x010b_bb08,
            0x010b_bb30,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14c,
        words: &[
            0x010b_bb08,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb20,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x198,
        words: &[
            0x010b_bb08,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0x010b_bb1c,
            0x0101_1584,
            0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1e4,
        words: &[
            0x010b_bb08,
            0x010b_bb28,
            0x0101_1584,
            0x010b_bb2c,
            0x0101_1584,
            0,
        ],
        cleared: 0x34,
    },
];

// Translated from 00bcc750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the vertex shader
/// loader of the shader `fn_00bcd390` builds): writes its twenty source blocks (`PAR2*`
/// files, macro pairs, cleared tails) into a stack frame and, when `011f91c0` is at least 2,
/// compiles each (`MakePath`, `PAR2%03i.vso` with the index, `CompileVertexShaderHLSL` with
/// `vs_2_0`) into the `NiPointer`s `+0x8c` onwards.
// Translation incomplete: 00bcc750 - the /GS cookie check (00ec408c) of its stack frame is not translated.
pub fn fn_00bcc750(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x818, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &SOURCES_00BCC750);
        if e.global::<i32>(SHADER_LEVEL_011F91C0) >= 2 {
            let step = NumberedCompile {
                first_file: 0x1c,
                slot: 0x8c,
                format: 0x010b_b774,
                profile: 0x010a_93e4,
                pixel: false,
                path: 0x60c,
                name: 0x710,
                skip_null: false,
            };
            compile_numbered(e, base, frame, &step, 0..0x14);
        }
    });
}

// Translated from 00bcd7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the pixel shader
/// loader of the same shader): writes its 33 source blocks and, when `011f91c0` is at least 2,
/// compiles the first 29 (`PAR2%03i.pso`, `CompilePixelShaderHLSL` with `ps_2_0`) into the
/// `NiPointer`s `+0xdc` onwards, and the last four as well when it is at least 5.
// Translation incomplete: 00bcd7c0 - the /GS cookie check (00ec408c) of its stack frame is not translated.
pub fn fn_00bcd7c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0xbf8, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &SOURCES_00BCD7C0);
        let level = e.global::<i32>(SHADER_LEVEL_011F91C0);
        if level < 2 {
            return;
        }
        let step = NumberedCompile {
            first_file: 0x20,
            slot: 0xdc,
            format: 0x010b_b91c,
            profile: 0x010a_93cc,
            pixel: true,
            path: 0x9ec,
            name: 0xaf0,
            skip_null: false,
        };
        compile_numbered(e, base, frame, &step, 0..0x1d);
        if level >= 5 {
            compile_numbered(e, base, frame, &step, 0x1d..0x21);
        }
    });
}

// Translated from 00bce950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the vertex shader
/// loader of the shader `fn_00bcf7f0` builds): writes its 26 source blocks and, when
/// `011f91c0` is at least 2, compiles each with a file name (`SKIN2%03i.vso`, `vs_2_0`) into
/// the `NiPointer`s `+0x8c` onwards.
// Translation incomplete: 00bce950 - the /GS cookie check (00ec408c) of its stack frame is not translated.
pub fn fn_00bce950(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x9e0, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &SOURCES_00BCE950);
        if e.global::<i32>(SHADER_LEVEL_011F91C0) >= 2 {
            let step = NumberedCompile {
                first_file: 0x1c,
                slot: 0x8c,
                format: 0x010b_b95c,
                profile: 0x010a_93e4,
                pixel: false,
                path: 0x8d8,
                name: 0x7d4,
                skip_null: true,
            };
            compile_numbered(e, base, frame, &step, 0..0x1a);
        }
    });
}

// Translated from 00bcf320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the pixel shader
/// loader of the same shader): writes its 13 source blocks and, when `011f91c0` is at least 2,
/// compiles each with a file name (`SKIN2%03i.pso`, `ps_2_0`) into the `NiPointer`s `+0xf4`
/// onwards.
// Translation incomplete: 00bcf320 - the /GS cookie check (00ec408c) of its stack frame is not translated.
pub fn fn_00bcf320(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x604, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &SOURCES_00BCF320);
        if e.global::<i32>(SHADER_LEVEL_011F91C0) >= 2 {
            let step = NumberedCompile {
                first_file: 0x1c,
                slot: 0xf4,
                format: 0x010b_b96c,
                profile: 0x010a_93cc,
                pixel: true,
                path: 0x4fc,
                name: 0x3f8,
                skip_null: true,
            };
            compile_numbered(e, base, frame, &step, 0..0xd);
        }
    });
}

// Translated from 00bd0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the vertex shader
/// loader of the shader `fn_00bd12c0` builds): writes its 14 source blocks, compiles the seven
/// from `+0x230` with a file name (`HAIR1%03i.vso`, `vs_1_1`) into the `NiPointer`s `+0x94`
/// onwards, then, when `011f91c0` is at least 2, the seven from `+0x1c` (`HAIR2%03i.vso`,
/// `vs_2_0`) into `+0xbc` onwards.
// Translation incomplete: 00bd0540 - the /GS cookie check (00ec408c) of its stack frame is not translated.
pub fn fn_00bd0540(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x650, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &SOURCES_00BD0540);
        let first = NumberedCompile {
            first_file: 0x230,
            slot: 0x94,
            format: 0x010b_baf8,
            profile: 0x010a_93ec,
            pixel: false,
            path: 0x548,
            name: 0x444,
            skip_null: true,
        };
        compile_numbered(e, base, frame, &first, 0..7);
        if e.global::<i32>(SHADER_LEVEL_011F91C0) >= 2 {
            let second = NumberedCompile {
                first_file: 0x1c,
                slot: 0xbc,
                format: 0x010b_bae8,
                profile: 0x010a_93e4,
                pixel: false,
                path: 0x548,
                name: 0x444,
                skip_null: false,
            };
            compile_numbered(e, base, frame, &second, 0..7);
        }
    });
}

/// The pass being drawn (`011f91e4`) and the texture setters of the slot +0x78 functions (unit
/// `shadowlightshader.cpp`, `stdcall`): `00b7c0a0` (object, stage, texture), `00b7c0e0` (object,
/// stage, word), `00b7c120` (stage), `00b7c1f0` and `00b7c230` (object, stage, index, flag), the
/// base `SetupGeometryTextures` `00b7d200` (`thiscall`, the block) and the empty `00d3dd00`
/// (`thiscall`, four words).
const PASS_011F91E4: u32 = 0x011f_91e4;
const STAGE_TEXTURE_00B7C0A0: u32 = 0x00b7_c0a0;
const STAGE_TEXTURE_00B7C0E0: u32 = 0x00b7_c0e0;
const STAGE_TEXTURE_00B7C120: u32 = 0x00b7_c120;
const STAGE_TEXTURE_00B7C1F0: u32 = 0x00b7_c1f0;
const STAGE_TEXTURE_00B7C230: u32 = 0x00b7_c230;
const SETUP_GEOMETRY_TEXTURES_00B7D200: u32 = 0x00b7_d200;
const EMPTY_00D3DD00: u32 = 0x00d3_dd00;

/// One texture setter call of the slot +0x78 functions, on the block's property.
#[derive(Clone, Copy)]
enum StageCall {
    /// `00b7c0a0(property, stage, texture)`.
    A(u32, u32),
    /// `00b7c0e0(property, stage, word)`.
    B(u32, u32),
    /// `00b7c1f0(property, stage, index, flag)`.
    F(u32, u32, u32),
    /// `00b7c230(property, stage, word, flag)`.
    T(u32, u32, u32),
    /// `00d3dd00` on the shader with (property, a, b, c).
    N(u32, u32, u32),
    /// `00b7c120(stage)`.
    R(u32),
}

/// Runs the setter calls in order, then the shader's slot +0x108.
fn stage_calls(e: &mut Engine, this: u32, property: u32, calls: &[StageCall]) {
    for call in calls {
        match *call {
            StageCall::A(stage, texture) => {
                e.call(STAGE_TEXTURE_00B7C0A0, &args![property, stage, texture]);
            }
            StageCall::B(stage, word) => {
                e.call(STAGE_TEXTURE_00B7C0E0, &args![property, stage, word]);
            }
            StageCall::F(stage, index, flag) => {
                e.call(STAGE_TEXTURE_00B7C1F0, &args![property, stage, index, flag]);
            }
            StageCall::T(stage, word, flag) => {
                e.call(STAGE_TEXTURE_00B7C230, &args![property, stage, word, flag]);
            }
            StageCall::N(a, b, c) => {
                e.call(EMPTY_00D3DD00, &args![this, property, a, b, c]);
            }
            StageCall::R(stage) => {
                e.call(STAGE_TEXTURE_00B7C120, &args![stage]);
            }
        }
    }
    e.vcall(this, 0x108, &args![]);
}

/// The jump table of `00bce500` for the passes 0x99 to 0x174 (the byte table at `00bce6e8` into
/// the 10 code addresses at `00bce6c0`), read from the exe.
const TARGETS_00BCE6C0: [u32; 10] = [
    0x00bc_e5dc,
    0x00bc_e5ce,
    0x00bc_e552,
    0x00bc_e544,
    0x00bc_e597,
    0x00bc_e589,
    0x00bc_e621,
    0x00bc_e636,
    0x00bc_e647,
    0x00bc_e6b2,
];
const BYTES_00BCE6E8: [u8; 220] = [
    0, 1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0, 1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0, 1, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 0, 1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    2, 3, 9, 9, 9, 9, 9, 9, 9, 2, 3, 9, 9, 9, 9, 9, 9, 9, 2, 3, 9, 9, 9, 9, 9, 9, 9, 2, 3, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 4, 9, 5, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 4, 5, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    4, 5, 9, 9, 9, 9, 9, 9, 9, 9, 9, 4, 5, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 6, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 6, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7, 9, 9, 9, 9, 9,
    9, 9, 9, 8, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9, 9, 8,
];

// Translated from 00bce500 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00bce500 to the `RET 4` at 00bce6bc, its jump tables after
/// it): slot 30 (+0x78, `SetupGeometryTextures`) of the vtable `010bb7a8`, `thiscall` with one
/// stack word: a block whose +0xc is the shader property. By the pass `011f91e4`: 0x231 to 0x233
/// put texture 0 on stage 0 (`00b7c0a0`) and the rendered texture on stage 6 (`00b7c120`);
/// 0x99 to 0x174 through the jump table one of nine fixed lists of `00b7c0a0`, `00b7c0e0`,
/// `00b7c1f0`, `00b7c230` and `00d3dd00` calls; after any of these the shader's slot +0x108.
/// Every other pass (and the table's default) runs the base `00b7d200` with the block.
pub fn fn_00bce500(e: &mut Engine, this: Ptr, block: Ptr) {
    use StageCall::{A, B, F, N, R, T};
    let base = this.addr();
    let pass = e.global::<u32>(PASS_011F91E4);
    let property = e.mem.u32(block.addr() + 0xc);
    let target = if pass > 0x233 {
        0x00bc_e6b2
    } else if pass >= 0x231 {
        0x00bc_e68c
    } else {
        let case = pass.wrapping_sub(0x99);
        if case > 0xdb {
            0x00bc_e6b2
        } else {
            TARGETS_00BCE6C0[usize::from(BYTES_00BCE6E8[case as usize])]
        }
    };
    let calls: &[StageCall] = match target {
        0x00bc_e544 => &[F(3, 0, 1), A(0, 0), B(1, 0), T(2, 0, 1)],
        0x00bc_e552 => &[A(0, 0), B(1, 0), T(2, 0, 1)],
        0x00bc_e589 => &[F(4, 0, 1), A(0, 0), B(1, 0), T(3, 0, 1)],
        0x00bc_e597 => &[A(0, 0), B(1, 0), T(3, 0, 1)],
        0x00bc_e5ce => &[F(4, 0, 1), A(0, 0), B(1, 0), T(3, 0, 1), N(2, 0, 1)],
        0x00bc_e5dc => &[A(0, 0), B(1, 0), T(3, 0, 1), N(2, 0, 1)],
        // 00bce621 joins 00bce562 (the `00b7c0e0` call) with stage 0.
        0x00bc_e621 => &[A(1, 0), B(0, 0), T(2, 0, 1)],
        // 00bce636 joins 00bce56a (the `00b7c230` call).
        0x00bc_e636 => &[A(0, 0), T(2, 0, 1)],
        0x00bc_e647 => &[A(1, 0), B(0, 0), T(2, 0, 1), N(6, 0, 1)],
        0x00bc_e68c => &[A(0, 0), R(6)],
        _ => {
            e.call(SETUP_GEOMETRY_TEXTURES_00B7D200, &args![base, block]);
            return;
        }
    };
    stage_calls(e, base, property, calls);
}

/// The jump table of `00bd02e0` for the passes 0xc5 to 0x142 (the byte table at `00bd03a0` into
/// the 4 code addresses at `00bd0390`), read from the exe.
const TARGETS_00BD0390: [u32; 4] = [0x00bd_032c, 0x00bd_030c, 0x00bd_0356, 0x00bd_0381];
const BYTES_00BD03A0: [u8; 126] = [
    0, 3, 3, 0, 3, 3, 3, 0, 3, 3, 0, 3, 3, 3, 0, 3, 3, 0, 3, 3, 3, 3, 1, 3, 3, 1, 3, 3, 3, 3, 3, 1,
    3, 3, 1, 3, 3, 3, 3, 3, 1, 3, 3, 1, 3, 3, 3, 3, 3, 1, 3, 3, 1, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3,
    3, 3, 0, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 0, 3,
    3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 3, 2,
];

// Translated from 00bd02e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00bd02e0 to the `RET 4` at 00bd038b, its jump tables after
/// it): slot 30 (+0x78, `SetupGeometryTextures`) of the vtable `010bb980`, `thiscall` with one
/// stack word: a block whose +0xc is the shader property. By the pass `011f91e4` (0xc5 to 0x142
/// through the jump table) one of three fixed lists of `00b7c0a0`, `00b7c0e0` and `00b7c1f0`
/// calls, then the shader's slot +0x108; every other pass runs the base `00b7d200`.
pub fn fn_00bd02e0(e: &mut Engine, this: Ptr, block: Ptr) {
    use StageCall::{A, B, F};
    let base = this.addr();
    let pass = e.global::<u32>(PASS_011F91E4);
    let property = e.mem.u32(block.addr() + 0xc);
    let case = pass.wrapping_sub(0xc5);
    let target = if case > 0x7d {
        0x00bd_0381
    } else {
        TARGETS_00BD0390[usize::from(BYTES_00BD03A0[case as usize])]
    };
    let calls: &[StageCall] = match target {
        // 00bd032c joins 00bd035a (`00b7c0e0` with 3, 1) and 00bd0362.
        0x00bd_032c => &[A(0, 0), B(1, 0), A(2, 1), B(3, 1), F(4, 0, 1)],
        // 00bd030c joins 00bd0368 (`00b7c1f0` with 3).
        0x00bd_030c => &[A(0, 0), B(1, 0), F(3, 0, 1)],
        0x00bd_0356 => &[B(0, 0), F(4, 0, 1)],
        _ => {
            e.call(SETUP_GEOMETRY_TEXTURES_00B7D200, &args![base, block]);
            return;
        }
    };
    stage_calls(e, base, property, calls);
}

// Translated from 00bcc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: a pass number; the
/// texture setup of the `fn_00bcd390` shader). By the exe's jump table (`00bcc600`, byte map
/// `00bcc614` for 0x99..0x174): numbers 0x99, 0x9a, 0xaf, 0xb0, 0xda, 0xec, 0x101, 0x103, 0x119,
/// 0x11a, 0x134, 0x140, 0x153, 0x15c, 0x168, 0x16e and 0x174 do nothing; 0xa4, 0xa5, 0xba, 0xbb,
/// 0x10e, 0x10f, 0x124 and 0x125 call `00b7c150` with 6 then `00b7c170` with 7; 0xe2, 0xe3, 0xf4
/// and 0xf5 the same with 5 and 6; 0x162 with 4 and 5; every other number goes to `00b7cfb0`.
pub fn fn_00bcc590(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let stages = match pass {
        0x99 | 0x9a | 0xaf | 0xb0 | 0xda | 0xec | 0x101 | 0x103 | 0x119 | 0x11a | 0x134 | 0x140
        | 0x153 | 0x15c | 0x168 | 0x16e | 0x174 => return,
        0xa4 | 0xa5 | 0xba | 0xbb | 0x10e | 0x10f | 0x124 | 0x125 => Some((6u32, 7u32)),
        0xe2 | 0xe3 | 0xf4 | 0xf5 => Some((5, 6)),
        0x162 => Some((4, 5)),
        _ => None,
    };
    match stages {
        Some((first, last)) => {
            e.call(SET_FIRST_STAGE_00B7C150, &args![base, first]);
            e.call(SET_LAST_STAGE_00B7C170, &args![base, last]);
        }
        None => {
            e.call(SET_STAGES_00B7CFB0, &args![base, pass]);
        }
    }
}

// Translated from 00bce800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: a pass number; the
/// texture setup of the `fn_00bcf7f0` shader). By the exe's jump table (`00bce858`, byte map
/// `00bce868` for 0xc5..0x142): numbers 0xc5, 0xc8, 0xcc, 0xcf, 0xd3, 0xd6, 0xdb, 0xde, 0xed,
/// 0xf0, 0x100, 0x107, 0x118, 0x11e, 0x133, 0x136, 0x13f and 0x142 do nothing; 0xe4, 0xe7, 0xf6
/// and 0xf9 call `00b7c150` with 5 then `00b7c170` with 6; 0x10d, 0x113, 0x123 and 0x129 the same
/// with 6 and 7; every other number goes to `00b7cfb0`.
pub fn fn_00bce800(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let stages = match pass {
        0xc5 | 0xc8 | 0xcc | 0xcf | 0xd3 | 0xd6 | 0xdb | 0xde | 0xed | 0xf0 | 0x100 | 0x107
        | 0x118 | 0x11e | 0x133 | 0x136 | 0x13f | 0x142 => return,
        0xe4 | 0xe7 | 0xf6 | 0xf9 => Some((5u32, 6u32)),
        0x10d | 0x113 | 0x123 | 0x129 => Some((6, 7)),
        _ => None,
    };
    match stages {
        Some((first, last)) => {
            e.call(SET_FIRST_STAGE_00B7C150, &args![base, first]);
            e.call(SET_LAST_STAGE_00B7C170, &args![base, last]);
        }
        None => {
            e.call(SET_STAGES_00B7CFB0, &args![base, pass]);
        }
    }
}

/// The pass of `fn_00bcce40`'s table (`01202230`..`012022b0`) for a pass number, from the
/// exe's two jump tables (`00bcd184` with byte map `00bcd1cc` for 1..0xf5, `00bcd2c4` with
/// byte map `00bcd308` for 0x102..0x174) and its direct test of 0x101.
fn pass_slot_00bcce40(pass: u32) -> Option<u32> {
    let slot = match pass {
        0xd9 => 0x0120_2230,
        0xda => 0x0120_2234,
        0xe2 => 0x0120_2238,
        0xe3 => 0x0120_223c,
        0xeb => 0x0120_2240,
        0xec => 0x0120_2244,
        0xf4 => 0x0120_2248,
        0xf5 => 0x0120_224c,
        0x101 => 0x0120_2250,
        0x102 => 0x0120_2254,
        0x103 => 0x0120_2258,
        0x10e => 0x0120_225c,
        0x10f => 0x0120_2260,
        0x119 => 0x0120_2264,
        0x11a => 0x0120_2268,
        0x124 => 0x0120_226c,
        0x125 => 0x0120_2270,
        0x99 => 0x0120_2274,
        0x9a => 0x0120_2278,
        0xa4 => 0x0120_227c,
        0xa5 => 0x0120_2280,
        0xaf => 0x0120_2284,
        0xb0 => 0x0120_2288,
        0xba => 0x0120_228c,
        0xbb => 0x0120_2290,
        0x134 => 0x0120_2294,
        0x140 => 0x0120_2298,
        0x153 => 0x0120_229c,
        0x15c => 0x0120_22a0,
        0x162 => 0x0120_22a4,
        0x168 => 0x0120_22a8,
        0x16e => 0x0120_22ac,
        0x174 => 0x0120_22b0,
        _ => return None,
    };
    Some(slot)
}

/// The fallback both pass selectors share: `00b7af80` with the number, then the slot +0xf0 of
/// shader 1 (`GetShader`) with it.
fn other_pass(e: &mut Engine, this: u32, pass: u32) {
    e.call(SET_PASS_00B7AF80, &args![this, pass]);
    let shader = e.call(GET_SHADER, &args![1u32]).u32();
    e.vcall(shader, 0xf0, &args![pass]);
}

// Translated from 00bcce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: a pass number; the
/// pass selection of the `fn_00bcd390` shader): assigns the declaration `+0x6c` to the
/// `NiPointer` `+0x28`; then a number of its table (`pass_slot_00bcce40`) makes that pass
/// current (`0126f74c`); 1 and 2 run the fallback (`00b7af80`, shader 1's slot +0xf0) and assign
/// `+0x6c` to `+0x28` again through `00a5f860`; any other number runs the fallback.
pub fn fn_00bcce40(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let declaration = e.mem.u32(base + 0x6c);
    ni_pointer_assign(e, base + 0x28, declaration);
    if let Some(slot) = pass_slot_00bcce40(pass) {
        let current = e.mem.u32(slot);
        e.mem.set_u32(CURRENT_PASS_0126F74C, current);
        return;
    }
    other_pass(e, base, pass);
    if pass == 1 || pass == 2 {
        let declaration = e.mem.u32(base + 0x6c);
        e.call(POINTER_ASSIGN_00A5F860, &args![base + 0x28, declaration]);
    }
}

// Translated from 00bcd380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, two stack words never read): answers
/// the declaration `+0x6c`.
pub fn fn_00bcd380(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    e.mem.u32(this.addr() + 0x6c)
}

// Translated from 00bcd490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no register or stack word read): answers `012022b4`, the end of
/// the pass table `01202230`.
pub fn fn_00bcd490(_e: &mut Engine) -> u32 {
    0x0120_22b4
}

/// The parts the two- and four-array shader destructors and reinitializers share: the
/// `NiPointer` arrays cleared (each element released and zeroed), then the pass table between
/// `table` and `end` cleared (`pass_pointer_clear`).
fn clear_pointer_arrays(e: &mut Engine, arrays: &[(u32, u32)]) {
    for (array, count) in arrays {
        for index in 0..*count {
            ni_pointer_clear(e, array + index * 4);
        }
    }
}

fn clear_pass_table(e: &mut Engine, table: u32, end: u32) {
    let mut slot = table;
    while slot < end {
        pass_pointer_clear(e, slot);
        slot += 4;
    }
}

/// The destructor body `00bcd4a0` and `00bcf920` share: the vtable, both arrays cleared, the
/// pass table cleared, the arrays' destructors (second first) and the base destructor.
fn destroy_two_array_shader(
    e: &mut Engine,
    this: u32,
    vtable: u32,
    second: (u32, u32),
    table: (u32, u32),
    first_count: u32,
) {
    e.mem.set_u32(this, vtable);
    clear_pointer_arrays(
        e,
        &[(this + 0x8c, first_count), (this + second.0, second.1)],
    );
    clear_pass_table(e, table.0, table.1);
    e.call(
        VECTOR_DESTRUCT,
        &args![this + second.0, 4u32, second.1, DESTRUCT_00B66F20],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![this + 0x8c, 4u32, first_count, DESTRUCT_00B66F20],
    );
    e.call(SHADER_BASE_DESTRUCT_00B79DE0, &args![this]);
}

// Translated from 00bcd4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the destructor of the
/// `fn_00bcd390` shader): stores the vtable `010bb7a8`, clears the `NiPointer`s `+0x8c` (0x14)
/// and `+0xdc` (0x21), clears the pass table `01202230`..`012022b4`, destroys both arrays
/// (`00b66f20` on each element, `+0xdc` first) and runs the base destructor `00b79de0`.
// Translation incomplete: 00bcd4a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the arrays) is not translated.
pub fn fn_00bcd4a0(e: &mut Engine, this: Ptr) {
    destroy_two_array_shader(
        e,
        this.addr(),
        VTABLE_010BB7A8,
        (0xdc, 0x21),
        (0x0120_2230, 0x0120_22b4),
        0x14,
    );
}

// Translated from 00bcd5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words): `00be0870`, then the
/// `NiPointer`s `+0x8c` (0x14) and `+0xdc` (0x21) and the pass table `01202230`..`012022b4`
/// cleared. Answers `true`.
pub fn fn_00bcd5d0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(SHADER_REINITIALIZE_00BE0870, &args![base]);
    clear_pointer_arrays(e, &[(base + 0x8c, 0x14), (base + 0xdc, 0x21)]);
    clear_pass_table(e, 0x0120_2230, 0x0120_22b4);
    true
}

// Translated from 00bce7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: the flags; the scalar
/// deleting destructor of the `fn_00bcd390` shader): `fn_00bcd4a0`, then, with bit 0, the
/// object freed with size 0x160. Answers `this`.
pub fn fn_00bce7d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bcd4a0(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x160u32]);
    }
    this
}

// Translated from 00bcf910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no register or stack word read): answers `01202328`, the end of
/// the pass table `012022c0`.
pub fn fn_00bcf910(_e: &mut Engine) -> u32 {
    0x0120_2328
}

// Translated from 00bcf920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the destructor of the
/// `fn_00bcf7f0` shader): the body of `fn_00bcd4a0` with the vtable `010bb980`, the arrays
/// `+0x8c` (0x1a) and `+0xf4` (0xd) and the pass table `012022c0`..`01202328`.
// Translation incomplete: 00bcf920 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the arrays) is not translated.
pub fn fn_00bcf920(e: &mut Engine, this: Ptr) {
    destroy_two_array_shader(
        e,
        this.addr(),
        VTABLE_010BB980,
        (0xf4, 0xd),
        (0x0120_22c0, 0x0120_2328),
        0x1a,
    );
}

// Translated from 00bcfa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words): `00be0870`, then the
/// `NiPointer`s `+0x8c` (0x1a) and `+0xf4` (0xd) and the pass table `012022c0`..`01202328`
/// cleared. Answers `true`.
pub fn fn_00bcfa50(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(SHADER_REINITIALIZE_00BE0870, &args![base]);
    clear_pointer_arrays(e, &[(base + 0x8c, 0x1a), (base + 0xf4, 0xd)]);
    clear_pass_table(e, 0x0120_22c0, 0x0120_2328);
    true
}

/// The pass of `fn_00bcfd20`'s table (`012022c0`..`01202324`) for a pass number and whether
/// the declaration assigned after it is the second (`+0x70`) rather than the first (`+0x6c`),
/// from the exe's jump tables (`00bd0128` with byte map `00bd016c` for 1..0xf9, `00bd0268` with
/// byte map `00bd0298` for 0x107..0x142) and its direct test of 0x100.
fn pass_slot_00bcfd20(pass: u32) -> Option<(u32, bool)> {
    let entry = match pass {
        0xdb => (0x0120_22c0, false),
        0xde => (0x0120_22c4, true),
        0xe4 => (0x0120_22c8, false),
        0xe7 => (0x0120_22cc, true),
        0xed => (0x0120_22d0, false),
        0xf0 => (0x0120_22d4, true),
        0xf6 => (0x0120_22d8, false),
        0xf9 => (0x0120_22dc, true),
        0x100 => (0x0120_22e0, false),
        0x107 => (0x0120_22e4, true),
        0x10d => (0x0120_22e8, false),
        0x113 => (0x0120_22ec, true),
        0x118 => (0x0120_22f0, false),
        0x11e => (0x0120_22f4, true),
        0x123 => (0x0120_22f8, false),
        0x129 => (0x0120_22fc, true),
        0x133 => (0x0120_2300, false),
        0x136 => (0x0120_2304, true),
        0x13f => (0x0120_2308, false),
        0x142 => (0x0120_230c, true),
        0xc5 => (0x0120_2310, false),
        0xc8 => (0x0120_2314, true),
        0xcc => (0x0120_2318, false),
        0xcf => (0x0120_231c, true),
        0xd3 => (0x0120_2320, false),
        0xd6 => (0x0120_2324, true),
        _ => return None,
    };
    Some(entry)
}

// Translated from 00bcfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: a pass number; the
/// pass selection of the `fn_00bcf7f0` shader): a number of its table makes that pass current
/// (`0126f74c`) and assigns its declaration (`+0x6c` or `+0x70`) to `+0x28` through `00a5f860`;
/// 1 and 2 run the fallback then assign `+0x6c`, 3 the fallback then `+0x70`; any other number
/// runs the fallback only. Then, for 0x250 and 0x251, the `NiPointer`s `+0x30` and `+0x34` get
/// `+0x7c` and `+0x80`; for every other number `00c663d0` copies `+0x84` into `+0x30` and `+0x88`
/// into `+0x34`.
pub fn fn_00bcfd20(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let declaration = match pass_slot_00bcfd20(pass) {
        Some((slot, second)) => {
            let current = e.mem.u32(slot);
            e.mem.set_u32(CURRENT_PASS_0126F74C, current);
            Some(second)
        }
        None => {
            other_pass(e, base, pass);
            match pass {
                1 | 2 => Some(false),
                3 => Some(true),
                _ => None,
            }
        }
    };
    if let Some(second) = declaration {
        let offset = if second { 0x70 } else { 0x6c };
        let object = e.mem.u32(base + offset);
        e.call(POINTER_ASSIGN_00A5F860, &args![base + 0x28, object]);
    }
    // 0x100 jumps past the 0x250/0x251 test, which it fails anyway.
    if pass == 0x250 || pass == 0x251 {
        let source = base + 0x7c;
        assign_slot(e, base + 0x30, source);
        assign_slot(e, base + 0x34, base + 0x80);
        return;
    }
    e.call(COPY_00C663D0, &args![base + 0x30, base + 0x84]);
    e.call(COPY_00C663D0, &args![base + 0x34, base + 0x88]);
}

/// An `NiPointer` at `dest` given the object the word at `source` holds, reading the source
/// again after releasing the old object (as the inlined code does).
fn assign_slot(e: &mut Engine, dest: u32, source: u32) {
    let old = e.mem.u32(dest);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(dest, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00bd0420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, two stack words: the geometry and one
/// never read): answers the declaration `+0x70` when the geometry's `+0xbc` (its skin) is set,
/// else `+0x6c`.
pub fn fn_00bd0420(e: &mut Engine, this: Ptr, geometry: u32, _unused_2: u32) -> u32 {
    let base = this.addr();
    if e.mem.u32(geometry + 0xbc) != 0 {
        e.mem.u32(base + 0x70)
    } else {
        e.mem.u32(base + 0x6c)
    }
}

// Translated from 00bd0440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: the flags; the scalar
/// deleting destructor of the `fn_00bcf7f0` shader): `fn_00bcf920`, then, with bit 0, the
/// object freed with size 0x128. Answers `this`.
pub fn fn_00bd0440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bcf920(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x128u32]);
    }
    this
}

// Translated from 00bd0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words): calls its own slot
/// +0x148, then (a tail jump) slot +0x14c.
pub fn fn_00bd0470(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base, 0x148, &args![]);
    e.vcall(base, 0x14c, &args![]);
}

/// The three texture stage setups after the first of each `fn_00bd0ae0` pass: (index, the
/// three `SetupDefaultStage` words).
const STAGES_00BD0AE0: [(u32, [u32; 3]); 5] = [
    (0, [1, 2, 1]),
    (1, [1, 2, 1]),
    (2, [1, 2, 1]),
    (3, [3, 2, 1]),
    (4, [1, 2, 1]),
];
/// The globals `fn_00bd0ae0` sets at the end.
const GLOBALS_00BD0AE0: [(u32, u32); 4] = [
    (0x011f_cf84, 0x482),
    (0x011f_cf88, 0x482),
    (0x011f_c3a4, 0x2c),
    (0x011f_c3a8, 0x3c),
];

/// One stage of a `fn_00bd0ae0` pass: `CreateNewStage` on the stage pointer, then
/// `SetupDefaultStage(stage, index, setup)` and `NiD3DPass::SetStage` on the pass the slot
/// holds (its `+0x14`, the stage).
fn add_stage(e: &mut Engine, pass_slot: u32, local: u32, index: u32, setup: [u32; 3]) {
    e.call(CREATE_NEW_STAGE, &args![local]);
    let stage = e.mem.u32(local);
    e.call(
        SETUP_DEFAULT_STAGE,
        &args![stage, index, setup[0], setup[1], setup[2]],
    );
    let pass = e.mem.u32(pass_slot);
    let stage = e.mem.u32(local);
    let position = e.mem.u32(pass + 0x14);
    e.call(PASS_SET_STAGE, &args![pass, position, stage]);
}

// Translated from 00bd0ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the pass setup of the
/// `fn_00bd12c0` shader): for each of its two pass pointers (`+0x8c`, `+0x90`) gets a new pass
/// (`00e804d0`) and adds stages 0..4 to it through one stage pointer on the stack (cleared once
/// per pass; setups (1, 2, 1) but (3, 2, 1) for stage 3), and stage 5 (3, 2, 1) when `004dc060`
/// with 0 answers at least 2; the stage pointer is released after. Then sets `011fcf84` and
/// `011fcf88` to 0x482, `011fc3a4` to 0x2c and `011fc3a8` to 0x3c. Answers `true`.
// Translation incomplete: 00bd0ae0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bd0ae0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for pass_index in 0..2u32 {
            let pass_slot = base + 0x8c + pass_index * 4;
            e.call(NEW_PASS_00E804D0, &args![pass_slot]);
            e.mem.set_u32(local, 0);
            for (index, setup) in STAGES_00BD0AE0 {
                add_stage(e, pass_slot, local, index, setup);
            }
            if e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32() as i32 >= 2 {
                add_stage(e, pass_slot, local, 5, [3, 2, 1]);
            }
            let stage = e.mem.u32(local);
            stage_release(e, stage);
        }
    });
    for (address, value) in GLOBALS_00BD0AE0 {
        e.mem.set_u32(address, value);
    }
    true
}

/// The arrays of the `fn_00bd12c0` shader: (offset, count) of each `NiPointer` array.
const ARRAYS_00BD12C0: [(u32, u32); 4] = [(0x94, 7), (0xbc, 7), (0xb0, 3), (0xd8, 3)];

// Translated from 00bd0ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words; the destructor of the
/// `fn_00bd12c0` shader): stores the vtable `010bbb50`, clears the `NiPointer` arrays `+0x94`
/// (7), `+0xbc` (7), `+0xb0` (3), `+0xd8` (3), the two pass pointers `+0x8c`, the `NiPointer`s
/// `+0xe4` and `+0xe8` and the word `+0x60`; runs the member destructors of `+0xe8` and `+0xe4`,
/// the arrays' destructors (`+0xd8`, `+0xbc`, `+0xb0`, `+0x94` with `00b66f20`, `+0x8c` with
/// `00b79930`) and the base destructor `00b79de0`.
// Translation incomplete: 00bd0ce0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the members) is not translated.
pub fn fn_00bd0ce0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BBB50);
    let arrays: Vec<(u32, u32)> = ARRAYS_00BD12C0
        .iter()
        .map(|(offset, count)| (base + offset, *count))
        .collect();
    clear_pointer_arrays(e, &arrays);
    clear_pass_table(e, base + 0x8c, base + 0x94);
    ni_pointer_clear(e, base + 0xe4);
    ni_pointer_clear(e, base + 0xe8);
    e.mem.set_u32(base + 0x60, 0);
    for offset in [0xe8u32, 0xe4] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release_reference(e, object);
        }
    }
    for (offset, count) in [(0xd8u32, 3u32), (0xbc, 7), (0xb0, 3), (0x94, 7)] {
        e.call(
            VECTOR_DESTRUCT,
            &args![base + offset, 4u32, count, DESTRUCT_00B66F20],
        );
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x8c, 4u32, 2u32, DESTRUCT_00B79930],
    );
    e.call(SHADER_BASE_DESTRUCT_00B79DE0, &args![base]);
}

// Translated from 00bd0f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words): `00be0870`, then the
/// `NiPointer` arrays `+0x94` (7), `+0xbc` (7), `+0xb0` (3), `+0xd8` (3) and the two pass
/// pointers `+0x8c` cleared. Answers `true`.
pub fn fn_00bd0f70(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.call(SHADER_REINITIALIZE_00BE0870, &args![base]);
    let arrays: Vec<(u32, u32)> = ARRAYS_00BD12C0
        .iter()
        .map(|(offset, count)| (base + offset, *count))
        .collect();
    clear_pointer_arrays(e, &arrays);
    clear_pass_table(e, base + 0x8c, base + 0x94);
    true
}

// Translated from 00bd1090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, no stack words): copies shader 1's
/// (`GetShader`) objects: its `+0x30` and `+0x34` held in local `NiPointer`s, assigned to this
/// shader's `+0x34` and `+0x30`, then its `+0x7c`, `+0x80`, `+0x84` and `+0x88` assigned to the
/// same offsets here; the locals released at the end (`+0x34`'s first).
// Translation incomplete: 00bd1090 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the local NiPointers) is not translated.
pub fn fn_00bd1090(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let source = e.call(GET_SHADER, &args![1u32]).u32();
    let first = e.mem.u32(source + 0x30);
    if first != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![first + 4]);
    }
    let second = e.mem.u32(source + 0x34);
    if second != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![second + 4]);
    }
    ni_pointer_assign(e, base + 0x34, second);
    ni_pointer_assign(e, base + 0x30, first);
    for offset in [0x7cu32, 0x80, 0x84, 0x88] {
        let object = e.mem.u32(source + offset);
        ni_pointer_assign(e, base + offset, object);
    }
    if second != 0 {
        release_reference(e, second);
    }
    if first != 0 {
        release_reference(e, first);
    }
}

// Translated from 00bd1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`; `thiscall`, one stack word: the flags; the scalar
/// deleting destructor of the `fn_00bd12c0` shader): `fn_00bd0ce0`, then, with bit 0, the
/// object freed with size 0xec. Answers `this`.
pub fn fn_00bd1480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd0ce0(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0xecu32]);
    }
    this
}

/// `00b71b90` (thiscall, no stack words): the shared tail the renderer-data functions below
/// jump to.
const SHADER_00B71B90: u32 = 0x00b7_1b90;
/// `00e81880` (thiscall, `RET 4`): the jump target of `00bd16d0`.
const JUMP_TARGET_00BD16D0: u32 = 0x00e8_1880;
/// The record `00bd0f60` answers (`01202330`).
const RTTI_01202330: u32 = 0x0120_2330;

/// Each object of the `count` pointers at `this + offset` that is set gets its virtual slot
/// `slot` (no stack words).
fn call_each(e: &mut Engine, this: u32, offset: u32, count: u32, slot: u32) {
    for index in 0..count {
        let object = e.mem.u32(this + offset + 4 * index);
        if object != 0 {
            e.vcall(object, slot, &args![]);
        }
    }
}

/// The shader's virtual slot +0x128 with each word of the pass table from `table` up to
/// `end` (read as it goes).
fn each_pass(e: &mut Engine, this: u32, table: u32, end: u32) {
    let mut slot = table;
    while (slot as i32) < (end as i32) {
        let pass = e.mem.u32(slot);
        e.vcall(this, 0x128, &args![pass]);
        slot = slot.wrapping_add(4);
    }
}

// Translated from 00bcc570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; slot +0x120 of the vtables at `010bb7a8` and
/// `010bb980`): the shader's own virtual slot +0x144, then a jump to its slot +0x158.
pub fn fn_00bcc570(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x144, &args![]);
    e.vcall(this.addr(), 0x158, &args![]);
}

// Translated from 00bcc6f0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// Unnamed in the engine map (fastcall; slot +0xd0 of the vtable at `010bb7a8`, the
/// [`fn_00bcd390`] shader): the 0x14 objects at +0x8c get their virtual slot +0xa0 and the
/// 0x21 at +0xdc their slot +0x88 (null ones skipped), then a jump to `00b71b90`.
pub fn fn_00bcc6f0(e: &mut Engine, this: Ptr) {
    call_each(e, this.addr(), 0x8c, 0x14, 0xa0);
    call_each(e, this.addr(), 0xdc, 0x21, 0x88);
    e.call(SHADER_00B71B90, &args![this]);
}

// Translated from 00bcce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; slot +0x124 of the vtable at `010bb7a8`): the shader's
/// virtual slot +0x128 with each word of the pass table `01202230`..`012022b4`.
pub fn fn_00bcce10(e: &mut Engine, this: Ptr) {
    each_pass(e, this.addr(), 0x0120_2230, 0x0120_22b4);
}

// Translated from 00bce8f0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// Unnamed in the engine map (fastcall; slot +0xd0 of the vtable at `010bb980`, the
/// [`fn_00bcf7f0`] shader): the 0x1a objects at +0x8c get their virtual slot +0xa0 and the
/// 0xd at +0xf4 their slot +0x88, then a jump to `00b71b90`.
pub fn fn_00bce8f0(e: &mut Engine, this: Ptr) {
    call_each(e, this.addr(), 0x8c, 0x1a, 0xa0);
    call_each(e, this.addr(), 0xf4, 0xd, 0x88);
    e.call(SHADER_00B71B90, &args![this]);
}

// Translated from 00bcf7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; slot +0x124 of the vtable at `010bb980`): the shader's
/// virtual slot +0x128 with each word of the pass table `012022c0`..`01202328`.
pub fn fn_00bcf7c0(e: &mut Engine, this: Ptr) {
    each_pass(e, this.addr(), 0x0120_22c0, 0x0120_2328);
}

// Translated from 00bd0490 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// Unnamed in the engine map (fastcall; slot +0x120 of the vtable at `010bbb50`, the
/// [`fn_00bd12c0`] shader): a jump to the shader's own virtual slot +0x144.
pub fn fn_00bd0490(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x144, &args![]);
}

// Translated from 00bd04a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; slot +0xd0 of the vtable at `010bbb50`): with a word at
/// +0x28, the 7 objects at +0x94 and the 7 at +0xbc get their virtual slot +0xa0; then, three
/// times, the objects at +0xb0 + 4i and +0xd8 + 4i their slot +0x88 (null ones skipped), and a
/// jump to `00b71b90`.
pub fn fn_00bd04a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        call_each(e, base, 0x94, 7, 0xa0);
        call_each(e, base, 0xbc, 7, 0xa0);
    }
    for index in 0..3u32 {
        call_each(e, base, 0xb0 + 4 * index, 1, 0x88);
        call_each(e, base, 0xd8 + 4 * index, 1, 0x88);
    }
    e.call(SHADER_00B71B90, &args![this]);
}

// Translated from 00bd0cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; slot +0x124 of the vtable at `010bbb50`): the shader's
/// virtual slot +0x128 with each word of the pass table `011fdff8`..`011fe000`.
pub fn fn_00bd0cb0(e: &mut Engine, this: Ptr) {
    each_pass(e, this.addr(), 0x011f_dff8, 0x011f_e000);
}

// Translated from 00bd0f60 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `GetRTTI` of the vtable at `010bbb50` (its slot +8): the address of the record at
/// `01202330`.
pub fn fn_00bd0f60(_e: &mut Engine) -> Ptr {
    Ptr::new(RTTI_01202330)
}

// Translated from 00bd16d0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// Unnamed in the engine map (slot +0x50 of the vtable at `010bbd08`): a 5-byte jump to
/// `00e81880` (thiscall, `RET 4`) with `this` and its one stack word, answering what it
/// answers.
pub fn fn_00bd16d0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    e.call(JUMP_TARGET_00BD16D0, &args![this, word]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_c570, fn_00bcc570(Ptr)),
        entry!(0x00bc_c6f0, fn_00bcc6f0(Ptr)),
        entry!(0x00bc_ce10, fn_00bcce10(Ptr)),
        entry!(0x00bc_e8f0, fn_00bce8f0(Ptr)),
        entry!(0x00bc_f7c0, fn_00bcf7c0(Ptr)),
        entry!(0x00bd_0490, fn_00bd0490(Ptr)),
        entry!(0x00bd_04a0, fn_00bd04a0(Ptr)),
        entry!(0x00bd_0cb0, fn_00bd0cb0(Ptr)),
        entry!(0x00bd_0f60, fn_00bd0f60() -> Ptr),
        entry!(0x00bd_16d0, fn_00bd16d0(Ptr, u32) -> u32),
        entry!(0x00bc_d390, fn_00bcd390(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_f7f0, fn_00bcf7f0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bd_12c0, fn_00bd12c0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bd_14b0, fn_00bd14b0() -> Ptr),
        entry!(0x00bc_fb00, fn_00bcfb00() -> Ptr),
        entry!(0x00bc_d680, fn_00bcd680() -> Ptr),
        entry!(0x00bc_c590, fn_00bcc590(Ptr, u32)),
        entry!(0x00bc_c750, fn_00bcc750(Ptr)),
        entry!(0x00bc_ce40, fn_00bcce40(Ptr, u32)),
        entry!(0x00bc_d380, fn_00bcd380(Ptr, u32, u32) -> u32),
        entry!(0x00bc_d490, fn_00bcd490() -> u32),
        entry!(0x00bc_d4a0, fn_00bcd4a0(Ptr)),
        entry!(0x00bc_d5d0, fn_00bcd5d0(Ptr) -> bool),
        entry!(0x00bc_d7c0, fn_00bcd7c0(Ptr)),
        entry!(0x00bc_e7d0, fn_00bce7d0(Ptr, u32) -> Ptr),
        entry!(0x00bc_e800, fn_00bce800(Ptr, u32)),
        entry!(0x00bc_e500, fn_00bce500(Ptr, Ptr)),
        entry!(0x00bd_02e0, fn_00bd02e0(Ptr, Ptr)),
        entry!(0x00bc_e950, fn_00bce950(Ptr)),
        entry!(0x00bc_f320, fn_00bcf320(Ptr)),
        entry!(0x00bc_f910, fn_00bcf910() -> u32),
        entry!(0x00bc_f920, fn_00bcf920(Ptr)),
        entry!(0x00bc_fa50, fn_00bcfa50(Ptr) -> bool),
        entry!(0x00bc_fd20, fn_00bcfd20(Ptr, u32)),
        entry!(0x00bd_0420, fn_00bd0420(Ptr, u32, u32) -> u32),
        entry!(0x00bd_0440, fn_00bd0440(Ptr, u32) -> Ptr),
        entry!(0x00bd_0470, fn_00bd0470(Ptr)),
        entry!(0x00bd_0540, fn_00bd0540(Ptr)),
        entry!(0x00bd_0ae0, fn_00bd0ae0(Ptr) -> bool),
        entry!(0x00bd_0ce0, fn_00bd0ce0(Ptr)),
        entry!(0x00bd_0f70, fn_00bd0f70(Ptr) -> bool),
        entry!(0x00bd_1090, fn_00bd1090(Ptr)),
        entry!(0x00bd_1480, fn_00bd1480(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    fn entries(declaration: u32, list: &[[u32; 5]]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|entry| {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                (declaration_slot(0x8c), words)
            })
            .collect()
    }

    fn two_declarations(creator: u32, construct: u32, size: u32) {
        let mut e = rig(construct, &[]);
        let (shader, log) = run(&mut e, creator);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        let mut want = vec![
            (DECLARATION_CREATE, vec![RENDERER, 6, 1]),
            (DECLARATION_CREATE, vec![RENDERER, 8, 1]),
        ];
        want.extend(entries(a, &SIX_ENTRIES));
        want.push((declaration_slot(0xb4), vec![a]));
        want.extend(entries(b, &EIGHT_ENTRIES));
        want.push((declaration_slot(0xb4), vec![b]));
        want.push((NI_ALLOCATE, vec![size]));
        want.push((construct, vec![SHADER, a, b]));
        for slot in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, a]));
        assert_eq!(log, want);
    }

    #[test]
    fn fn_00bd14b0_makes_two_declarations_then_the_shader() {
        two_declarations(0x00bd_14b0, SHADER_CONSTRUCT_00BD12C0, 0xec);
    }

    #[test]
    fn fn_00bcfb00_makes_two_declarations_then_the_shader() {
        two_declarations(0x00bc_fb00, SHADER_CONSTRUCT_00BCF7F0, 0x128);
    }

    #[test]
    fn fn_00bcd680_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BCD390, &[]);
        let (shader, log) = run(&mut e, 0x00bc_d680);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        let mut want = vec![(DECLARATION_CREATE, vec![RENDERER, 6, 1])];
        want.extend(entries(d, &SIX_ENTRIES));
        want.push((declaration_slot(0xb4), vec![d]));
        want.push((NI_ALLOCATE, vec![0x160]));
        want.push((SHADER_CONSTRUCT_00BCD390, vec![SHADER, d]));
        for slot in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, d]));
        assert_eq!(log, want);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn engine() -> (Engine, u32) {
        let mut e = rig();
        e.register(SHADER_BASE_CONSTRUCT_00B79B00, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        for offset in [0x28u32, 0x6c, 0x70, 0xe4, 0xe8] {
            e.mem.set_u32(this + offset, 0);
        }
        (e, this)
    }

    #[test]
    fn fn_00bcd390_builds_the_shader() {
        let (mut e, this) = engine();
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_d390, &args![this, declaration]).u32(), this);
        assert_eq!(
            calls(&e, SHADER_BASE_CONSTRUCT_00B79B00),
            vec![vec![this, declaration, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010BB7A8);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 0x14, DESTRUCT_00B66F20),
                pointer_array(this + 0xdc, 0x21, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 3);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x6c), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }

    #[test]
    fn fn_00bcf7f0_builds_the_shader() {
        let (mut e, this) = engine();
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_f7f0, &args![this, first, second]).u32(), this);
        assert_eq!(
            calls(&e, SHADER_BASE_CONSTRUCT_00B79B00),
            vec![vec![this, first, second, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010BB980);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 0x1a, DESTRUCT_00B66F20),
                pointer_array(this + 0xf4, 0xd, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 4);
        assert_eq!(e.mem.u32(this + 0x6c), first);
        assert_eq!(e.mem.u32(this + 0x70), second);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }

    #[test]
    fn fn_00bd12c0_builds_the_shader_and_shares_the_second_declaration() {
        let (mut e, this) = engine();
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x28, old);
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        assert_eq!(e.call(0x00bd_12c0, &args![this, first, second]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BBB50);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x94, 7, DESTRUCT_00B66F20),
                pointer_array(this + 0xb0, 3, DESTRUCT_00B66F20),
                pointer_array(this + 0xbc, 7, DESTRUCT_00B66F20),
                pointer_array(this + 0xd8, 3, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 2);
        assert_eq!(e.mem.u32(this + 0xe4), first);
        assert_eq!(e.mem.u32(this + 0xe8), second);
        assert_eq!(e.mem.u32(this + 0x28), second);
        assert_eq!(e.mem.u32(second + 4), 2);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }
}

#[cfg(test)]
mod b053_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const STUB: AbiFn = |_, _| Ret::default();
    /// A test vtable whose slot `+n` is the address `SLOTS + n`.
    const VTABLE: u32 = 0x0e80_0000;
    const SLOTS: u32 = 0x0e81_0000;

    fn engine() -> Engine {
        let mut e = rig();
        e.map(0x011f_9000, 0x4000);
        e.map(0x0120_2000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.register(VECTOR_DESTRUCT, STUB);
        e.register(NI_SIZED_DELETE, STUB);
        e.register(SHADER_BASE_DESTRUCT_00B79DE0, STUB);
        e.register(SHADER_REINITIALIZE_00BE0870, STUB);
        let slots: Vec<u32> = (0..0x60).map(|slot| SLOTS + slot * 4).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, STUB);
        }
        e
    }

    fn shader(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this, VTABLE);
        this
    }

    /// Runs a loader with doubles for its callees; answers the compile calls.
    fn run_loader(address: u32, level: u32) -> (Engine, u32, Vec<Vec<u32>>) {
        let mut e = engine();
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![0u8; a[2] as usize]);
            Ret::default()
        });
        e.register(MAKE_PATH, STUB);
        e.register(FORMAT_00406D00, STUB);
        e.register(COMPILE_VERTEX_SHADER, |e, _| e.mem.alloc(8).into_ret());
        e.register(COMPILE_PIXEL_SHADER, |e, _| e.mem.alloc(8).into_ret());
        e.mem.set_u32(SHADER_LEVEL_011F91C0, level);
        let this = shader(&mut e);
        e.call(address, &args![this]);
        let mut compiles = calls(&e, COMPILE_VERTEX_SHADER);
        compiles.extend(calls(&e, COMPILE_PIXEL_SHADER));
        (e, this, compiles)
    }

    #[test]
    fn fn_00bcc750_compiles_twenty_vertex_shaders() {
        let (e, this, compiles) = run_loader(0x00bc_c750, 2);
        assert_eq!(calls(&e, MEMSET).len(), 19);
        assert_eq!(compiles.len(), 20);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats[3][2..], [0x010b_b774, 3]);
        let paths = calls(&e, MAKE_PATH);
        assert_eq!(paths[0][0], 0x010a_ebf4);
        assert_eq!(paths[14][0], 0x010b_b784);
        assert_eq!(compiles[0][3], 0x010a_93e4);
        // Each result is held by its NiPointer.
        for index in 0..20u32 {
            let held = e.mem.u32(this + 0x8c + index * 4);
            assert_ne!(held, 0);
            assert_eq!(e.mem.u32(held + 4), 1);
        }
        let (e, _, compiles) = run_loader(0x00bc_c750, 1);
        assert!(compiles.is_empty());
        assert_eq!(calls(&e, MEMSET).len(), 19);
    }

    #[test]
    fn fn_00bcd7c0_compiles_the_last_four_from_level_5() {
        let (e, this, compiles) = run_loader(0x00bc_d7c0, 2);
        assert_eq!(compiles.len(), 0x1d);
        assert_eq!(compiles[0][3], 0x010a_93cc);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        let (e, this, compiles) = run_loader(0x00bc_d7c0, 5);
        assert_eq!(compiles.len(), 0x21);
        assert_ne!(e.mem.u32(this + 0x15c), 0);
        assert_eq!(calls(&e, FORMAT_00406D00)[0x20][2..], [0x010b_b91c, 0x20]);
    }

    #[test]
    fn fn_00bce950_skips_blocks_without_a_file() {
        let (e, this, compiles) = run_loader(0x00bc_e950, 3);
        let files: Vec<u32> = SOURCES_00BCE950
            .iter()
            .filter(|source| source.words[0] != 0)
            .map(|source| source.words[0])
            .collect();
        assert_eq!(compiles.len(), files.len());
        let formats = calls(&e, FORMAT_00406D00);
        assert!(formats.iter().all(|words| words[2] == 0x010b_b95c));
        let held = (0..0x1a)
            .filter(|i| e.mem.u32(this + 0x8c + i * 4) != 0)
            .count();
        assert_eq!(held, files.len());
    }

    #[test]
    fn fn_00bcf320_compiles_skin_pixel_shaders() {
        let (e, this, compiles) = run_loader(0x00bc_f320, 2);
        assert!(!compiles.is_empty());
        assert!(compiles.len() <= 13);
        assert!(calls(&e, COMPILE_VERTEX_SHADER).is_empty());
        assert_ne!(e.mem.u32(this + 0xf4), 0);
        let (_, _, compiles) = run_loader(0x00bc_f320, 0);
        assert!(compiles.is_empty());
    }

    #[test]
    fn fn_00bd0540_compiles_the_first_seven_always() {
        let (e, this, compiles) = run_loader(0x00bd_0540, 0);
        assert!(!compiles.is_empty());
        assert!(compiles.iter().all(|words| words[3] == 0x010a_93ec));
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        let (e, this, compiles) = run_loader(0x00bd_0540, 2);
        let late: Vec<_> = compiles.iter().filter(|w| w[3] == 0x010a_93e4).collect();
        assert_eq!(late.len(), 7);
        assert_ne!(e.mem.u32(this + 0xbc), 0);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats.last().unwrap()[2..], [0x010b_bae8, 6]);
    }

    fn stage_calls(address: u32, pass: u32) -> Vec<(u32, Vec<u32>)> {
        let mut e = engine();
        e.register(SET_FIRST_STAGE_00B7C150, STUB);
        e.register(SET_LAST_STAGE_00B7C170, STUB);
        e.register(SET_STAGES_00B7CFB0, STUB);
        let this = shader(&mut e);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, pass]);
        e.call_log.unwrap().into_iter().skip(1).collect()
    }

    #[test]
    fn fn_00bcc590_picks_the_stages_by_pass() {
        assert!(stage_calls(0x00bc_c590, 0x99).is_empty());
        assert!(stage_calls(0x00bc_c590, 0x174).is_empty());
        let log = stage_calls(0x00bc_c590, 0xa4);
        assert_eq!((log[0].0, log[0].1[1]), (SET_FIRST_STAGE_00B7C150, 6));
        assert_eq!((log[1].0, log[1].1[1]), (SET_LAST_STAGE_00B7C170, 7));
        assert_eq!(stage_calls(0x00bc_c590, 0xf5)[0].1[1], 5);
        assert_eq!(stage_calls(0x00bc_c590, 0x162)[1].1[1], 5);
        let log = stage_calls(0x00bc_c590, 0x98);
        assert_eq!((log[0].0, log[0].1[1]), (SET_STAGES_00B7CFB0, 0x98));
        assert_eq!(stage_calls(0x00bc_c590, 0x9b)[0].0, SET_STAGES_00B7CFB0);
    }

    #[test]
    fn fn_00bce800_picks_the_stages_by_pass() {
        assert!(stage_calls(0x00bc_e800, 0xc5).is_empty());
        assert!(stage_calls(0x00bc_e800, 0x142).is_empty());
        assert_eq!(stage_calls(0x00bc_e800, 0xe4)[0].1[1], 5);
        assert_eq!(stage_calls(0x00bc_e800, 0x129)[1].1[1], 7);
        assert_eq!(stage_calls(0x00bc_e800, 0x143)[0].0, SET_STAGES_00B7CFB0);
    }

    fn select(address: u32, pass: u32) -> (Engine, u32) {
        let mut e = engine();
        for callee in [SET_PASS_00B7AF80, POINTER_ASSIGN_00A5F860, COPY_00C663D0] {
            e.register(callee, STUB);
        }
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, VTABLE);
        e.register_double(GET_SHADER, move |_, _| other.into_ret());
        for index in 0..0x40u32 {
            e.mem.set_u32(0x0120_2230 + index * 4, 0x5000 + index);
        }
        let this = shader(&mut e);
        let declarations = [object(&mut e, 1), object(&mut e, 1)];
        e.mem.set_u32(this + 0x6c, declarations[0]);
        e.mem.set_u32(this + 0x70, declarations[1]);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, pass]);
        (e, this)
    }

    #[test]
    fn fn_00bcce40_selects_the_pass() {
        let (e, this) = select(0x00bc_ce40, 0x101);
        assert_eq!(e.mem.u32(CURRENT_PASS_0126F74C), 0x5008);
        let declaration = e.mem.u32(this + 0x6c);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 2);
        let (e, _) = select(0x00bc_ce40, 0x174);
        assert_eq!(e.mem.u32(CURRENT_PASS_0126F74C), 0x5020);
        let (e, this) = select(0x00bc_ce40, 2);
        assert_eq!(calls(&e, SET_PASS_00B7AF80), vec![vec![this, 2]]);
        assert_eq!(calls(&e, SLOTS + 0xf0).len(), 1);
        assert_eq!(calls(&e, POINTER_ASSIGN_00A5F860).len(), 1);
        let (e, _) = select(0x00bc_ce40, 0xf6);
        assert_eq!(calls(&e, SET_PASS_00B7AF80).len(), 1);
        assert!(calls(&e, POINTER_ASSIGN_00A5F860).is_empty());
        assert_eq!(e.mem.u32(CURRENT_PASS_0126F74C), 0);
    }

    #[test]
    fn fn_00bcfd20_selects_the_pass_and_copies() {
        let (e, this) = select(0x00bc_fd20, 0xde);
        assert_eq!(e.mem.u32(CURRENT_PASS_0126F74C), 0x5025);
        let second = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, POINTER_ASSIGN_00A5F860),
            vec![vec![this + 0x28, second]]
        );
        assert_eq!(
            calls(&e, COPY_00C663D0),
            vec![
                vec![this + 0x30, this + 0x84],
                vec![this + 0x34, this + 0x88]
            ]
        );
        let (e, this) = select(0x00bc_fd20, 3);
        assert_eq!(calls(&e, SET_PASS_00B7AF80), vec![vec![this, 3]]);
        assert_eq!(
            calls(&e, POINTER_ASSIGN_00A5F860)[0][1],
            e.mem.u32(this + 0x70)
        );
        let (e, this) = select(0x00bc_fd20, 0x250);
        assert!(calls(&e, POINTER_ASSIGN_00A5F860).is_empty());
        assert!(calls(&e, COPY_00C663D0).is_empty());
        assert_eq!(e.mem.u32(this + 0x30), e.mem.u32(this + 0x7c));
    }

    #[test]
    fn small_answers() {
        let mut e = engine();
        let this = shader(&mut e);
        e.mem.set_u32(this + 0x6c, 0x11);
        e.mem.set_u32(this + 0x70, 0x22);
        assert_eq!(e.call(0x00bc_d380, &args![this, 0u32, 0u32]).u32(), 0x11);
        assert_eq!(e.call(0x00bc_d490, &args![]).u32(), 0x0120_22b4);
        assert_eq!(e.call(0x00bc_f910, &args![]).u32(), 0x0120_2328);
        let geometry = e.mem.alloc(0xc0);
        assert_eq!(
            e.call(0x00bd_0420, &args![this, geometry, 0u32]).u32(),
            0x11
        );
        e.mem.set_u32(geometry + 0xbc, 1);
        assert_eq!(
            e.call(0x00bd_0420, &args![this, geometry, 0u32]).u32(),
            0x22
        );
        e.call(0x00bd_0470, &args![this]);
        let log: Vec<u32> = e.call_log.clone().unwrap().iter().map(|c| c.0).collect();
        assert!(log.ends_with(&[0x00bd_0470, SLOTS + 0x148, SLOTS + 0x14c]));
    }

    #[test]
    fn two_array_destructors_and_reinitializers() {
        let mut e = engine();
        e.register(PASS_DELETE_00E80190, STUB);
        let this = shader(&mut e);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x8c + 4, held);
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(0x0120_2230, pass);
        assert_eq!(e.call(0x00bc_e7d0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BB7A8);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u32(0x0120_2230), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0xdc, 4, 0x21, DESTRUCT_00B66F20],
                vec![this + 0x8c, 4, 0x14, DESTRUCT_00B66F20],
            ]
        );
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![this, 0x160]]);
        e.call_log = Some(vec![]);
        e.call(0x00bd_0440, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), VTABLE_010BB980);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT)[0],
            vec![this + 0xf4, 4, 0xd, DESTRUCT_00B66F20]
        );
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![this, 0x128]]);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0xdc + 0x80, held);
        assert!(e.call(0x00bc_d5d0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x15c), 0);
        assert_eq!(calls(&e, SHADER_REINITIALIZE_00BE0870), vec![vec![this]]);
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 2);
        e.mem.set_u32(0x0120_2324, pass);
        assert!(e.call(0x00bc_fa50, &args![this]).bool());
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        assert_eq!(e.mem.u32(0x0120_2324), 0);
    }

    #[test]
    fn fn_00bd0ae0_sets_up_both_passes() {
        let mut e = engine();
        e.register(NEW_PASS_00E804D0, |e, a| {
            let pass = e.mem.alloc(0x68);
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(CREATE_NEW_STAGE, |e, a| {
            let stage = e.mem.alloc(0x64);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(SETUP_DEFAULT_STAGE, STUB);
        e.register(PASS_SET_STAGE, STUB);
        e.register(STAGE_DELETE_00E7ECD0, STUB);
        e.register(RENDERER_QUERY_004DC060, |_, _| 2u32.into_ret());
        let this = shader(&mut e);
        assert!(e.call(0x00bd_0ae0, &args![this]).bool());
        let setups = calls(&e, SETUP_DEFAULT_STAGE);
        assert_eq!(setups.len(), 12);
        assert_eq!(setups[3][1..], [3, 3, 2, 1]);
        assert_eq!(setups[5][1..], [5, 3, 2, 1]);
        assert_eq!(setups[6][1..], [0, 1, 2, 1]);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0).len(), 2);
        assert_eq!(calls(&e, PASS_SET_STAGE).len(), 12);
        assert_eq!(e.mem.u32(0x011f_cf84), 0x482);
        assert_eq!(e.mem.u32(0x011f_c3a8), 0x3c);
        e.register(RENDERER_QUERY_004DC060, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00bd_0ae0, &args![this]);
        assert_eq!(calls(&e, SETUP_DEFAULT_STAGE).len(), 10);
    }

    #[test]
    fn fn_00bd0ce0_destroys_the_hair_shader() {
        let mut e = engine();
        e.register(PASS_DELETE_00E80190, STUB);
        let this = shader(&mut e);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0xd8 + 8, held);
        let first = object(&mut e, 1);
        e.mem.set_u32(this + 0xe4, first);
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(this + 0x90, pass);
        e.mem.set_u32(this + 0x60, 9);
        assert_eq!(e.call(0x00bd_1480, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BBB50);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held], vec![first]]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        let destructs = calls(&e, VECTOR_DESTRUCT);
        assert_eq!(destructs.len(), 5);
        assert_eq!(destructs[4], vec![this + 0x8c, 4, 2, DESTRUCT_00B79930]);
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![this, 0xec]]);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x94, held);
        assert!(e.call(0x00bd_0f70, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x94), 0);
    }

    #[test]
    fn fn_00bd1090_copies_shader_1_objects() {
        let mut e = engine();
        let source = e.mem.alloc(0x90);
        let objects: Vec<u32> = (0..6).map(|_| object(&mut e, 1)).collect();
        for (offset, object) in [0x30u32, 0x34, 0x7c, 0x80, 0x84, 0x88].iter().zip(&objects) {
            e.mem.set_u32(source + offset, *object);
        }
        e.register_double(GET_SHADER, move |_, _| source.into_ret());
        let this = shader(&mut e);
        e.call(0x00bd_1090, &args![this]);
        for (offset, object) in [0x30u32, 0x34, 0x7c, 0x80, 0x84, 0x88].iter().zip(&objects) {
            assert_eq!(e.mem.u32(this + offset), *object);
            assert_eq!(e.mem.u32(object + 4), 2);
        }
        assert!(calls(&e, REFERENCE_DELETE).is_empty());
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    /// One double for every virtual slot used here: it records (object, slot) in the log
    /// through a slot-specific address.
    fn slot_address(slot: u32) -> u32 {
        0x03f2_0000 + slot
    }

    /// An object whose vtable maps every slot below 0x200 to `slot_address(slot)`.
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(vtable + slot, slot_address(slot));
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for slot in (0..0x200u32).step_by(4) {
            e.register(slot_address(slot), |_, _| Ret::default());
        }
        e.register(SHADER_00B71B90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// The (object, slot) pairs of the virtual calls made, in order.
    fn slot_calls(e: &Engine) -> Vec<(u32, u32)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| (0x03f2_0000..0x03f2_0200).contains(&c.0))
            .map(|c| (c.1[0], c.0 - 0x03f2_0000))
            .collect()
    }

    #[test]
    fn slot_120_runs_slots_144_then_158() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x00bc_c570, &args![this]);
        assert_eq!(slot_calls(&e), vec![(this, 0x144), (this, 0x158)]);
    }

    fn renderer_data_case(address: u32, arrays: [(u32, u32, u32); 2]) {
        let mut e = engine();
        let this = object(&mut e);
        let mut expected = vec![];
        for (offset, count, slot) in arrays {
            for index in 0..count {
                // Every other pointer is null.
                if index % 2 == 0 {
                    let member = object(&mut e);
                    e.mem.set_u32(this + offset + 4 * index, member);
                    expected.push((member, slot));
                }
            }
        }
        e.call(address, &args![this]);
        assert_eq!(slot_calls(&e), expected);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(SHADER_00B71B90, vec![this]));
    }

    #[test]
    fn first_shader_renderer_data_walks_both_arrays() {
        renderer_data_case(0x00bc_c6f0, [(0x8c, 0x14, 0xa0), (0xdc, 0x21, 0x88)]);
    }

    #[test]
    fn second_shader_renderer_data_walks_both_arrays() {
        renderer_data_case(0x00bc_e8f0, [(0x8c, 0x1a, 0xa0), (0xf4, 0xd, 0x88)]);
    }

    fn pass_table_case(address: u32, table: u32, end: u32) {
        let mut e = engine();
        e.mem.map(table, end - table);
        for slot in (table..end).step_by(4) {
            e.mem.set_u32(slot, slot ^ 0x5555);
        }
        let this = object(&mut e);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        let passes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == slot_address(0x128))
            .map(|c| c.1[1])
            .collect();
        let wanted: Vec<u32> = (table..end).step_by(4).map(|slot| slot ^ 0x5555).collect();
        assert_eq!(passes, wanted);
    }

    #[test]
    fn first_shader_reloads_each_pass_of_its_table() {
        pass_table_case(0x00bc_ce10, 0x0120_2230, 0x0120_22b4);
    }

    #[test]
    fn second_shader_reloads_each_pass_of_its_table() {
        pass_table_case(0x00bc_f7c0, 0x0120_22c0, 0x0120_2328);
    }

    #[test]
    fn third_shader_reloads_each_pass_of_its_table() {
        pass_table_case(0x00bd_0cb0, 0x011f_dff8, 0x011f_e000);
    }

    #[test]
    fn third_shader_slot_120_jumps_to_slot_144() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x00bd_0490, &args![this]);
        assert_eq!(slot_calls(&e), vec![(this, 0x144)]);
    }

    #[test]
    fn third_shader_renderer_data_skips_the_first_arrays_without_the_word() {
        let mut e = engine();
        let this = object(&mut e);
        let mut members = vec![];
        for offset in (0x94..0xe4u32).step_by(4) {
            let member = object(&mut e);
            e.mem.set_u32(this + offset, member);
            members.push((offset, member));
        }
        let at = |offset: u32| members.iter().find(|m| m.0 == offset).unwrap().1;
        e.call(0x00bd_04a0, &args![this]);
        let tail: Vec<(u32, u32)> = [0xb0u32, 0xd8, 0xb4, 0xdc, 0xb8, 0xe0]
            .iter()
            .map(|offset| (at(*offset), 0x88))
            .collect();
        assert_eq!(slot_calls(&e), tail);
        // With the word at +0x28: the 14 slot +0xa0 calls first.
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bd_04a0, &args![this]);
        let calls = slot_calls(&e);
        assert_eq!(calls.len(), 20);
        assert_eq!(calls[0], (at(0x94), 0xa0));
        assert_eq!(calls[13], (at(0xd4), 0xa0));
        assert_eq!(calls[14..], tail[..]);
    }

    #[test]
    fn third_shader_get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bd_0f60, &args![]).u32(), RTTI_01202330);
    }

    #[test]
    fn thunk_00bd16d0_forwards_and_answers() {
        let mut e = Engine::new();
        e.register(JUMP_TARGET_00BD16D0, |_, a| (a[0] + a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00bd_16d0, &args![0x1000u32, 0x22u32]).u32(),
            0x1022
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|c| c.0 == JUMP_TARGET_00BD16D0).count(),
            1
        );
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    const SLOT_108: u32 = 0x0f00_0108;
    const SETTERS: [u32; 8] = [
        STAGE_TEXTURE_00B7C0A0,
        STAGE_TEXTURE_00B7C0E0,
        STAGE_TEXTURE_00B7C120,
        STAGE_TEXTURE_00B7C1F0,
        STAGE_TEXTURE_00B7C230,
        SETUP_GEOMETRY_TEXTURES_00B7D200,
        EMPTY_00D3DD00,
        SLOT_108,
    ];

    /// A shader whose slot +0x108 is recorded, a block whose +0xc is the property, and the
    /// setters doubled.
    fn rig() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let mut slots = [0u32; 0x43];
        slots[0x108 / 4] = SLOT_108;
        e.put_vtable(0x0f00_1000, &slots);
        for setter in SETTERS {
            e.register(setter, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0f00_1000);
        let block = e.mem.alloc(0x10);
        let property = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0xc, property);
        e.call_log = Some(vec![]);
        (e, this, block, property)
    }

    fn run(e: &mut Engine, function: u32, this: u32, block: u32, pass: u32) -> Calls {
        e.set_global::<u32>(PASS_011F91E4, pass);
        e.call_log = Some(vec![]);
        e.call(function, &args![this, block]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| SETTERS.contains(a))
            .collect()
    }

    #[test]
    fn fn_00bce500_sets_the_stage_textures_by_pass() {
        let (mut e, this, block, p) = rig();
        let a = |s: u32, t: u32| (STAGE_TEXTURE_00B7C0A0, vec![p, s, t]);
        let b = |s: u32, w: u32| (STAGE_TEXTURE_00B7C0E0, vec![p, s, w]);
        let f = |s: u32| (STAGE_TEXTURE_00B7C1F0, vec![p, s, 0, 1]);
        let t = |s: u32| (STAGE_TEXTURE_00B7C230, vec![p, s, 0, 1]);
        let n = |s: u32| (EMPTY_00D3DD00, vec![this, p, s, 0, 1]);
        let v = (SLOT_108, vec![this]);
        let base = vec![(SETUP_GEOMETRY_TEXTURES_00B7D200, vec![this, block])];
        let cases: Vec<(u32, Calls)> = vec![
            (0x99, vec![a(0, 0), b(1, 0), t(3), n(2), v.clone()]),
            (0x9a, vec![f(4), a(0, 0), b(1, 0), t(3), n(2), v.clone()]),
            (0xd9, vec![a(0, 0), b(1, 0), t(2), v.clone()]),
            (0xda, vec![f(3), a(0, 0), b(1, 0), t(2), v.clone()]),
            (0x101, vec![a(0, 0), b(1, 0), t(3), v.clone()]),
            (0x103, vec![f(4), a(0, 0), b(1, 0), t(3), v.clone()]),
            (0x134, vec![a(1, 0), b(0, 0), t(2), v.clone()]),
            (0x153, vec![a(0, 0), t(2), v.clone()]),
            (0x15c, vec![a(1, 0), b(0, 0), t(2), n(6), v.clone()]),
            (0x174, vec![a(1, 0), b(0, 0), t(2), n(6), v.clone()]),
            (
                0x232,
                vec![a(0, 0), (STAGE_TEXTURE_00B7C120, vec![6]), v.clone()],
            ),
            (0x9b, base.clone()),
            (0x98, base.clone()),
            (0x175, base.clone()),
            (0x230, base.clone()),
            (0x234, base),
        ];
        for (pass, expected) in cases {
            assert_eq!(
                run(&mut e, 0x00bc_e500, this, block, pass),
                expected,
                "{pass:#x}"
            );
        }
    }

    #[test]
    fn fn_00bd02e0_sets_the_stage_textures_by_pass() {
        let (mut e, this, block, p) = rig();
        let a = |s: u32, t: u32| (STAGE_TEXTURE_00B7C0A0, vec![p, s, t]);
        let b = |s: u32, w: u32| (STAGE_TEXTURE_00B7C0E0, vec![p, s, w]);
        let f = |s: u32| (STAGE_TEXTURE_00B7C1F0, vec![p, s, 0, 1]);
        let v = (SLOT_108, vec![this]);
        let base = vec![(SETUP_GEOMETRY_TEXTURES_00B7D200, vec![this, block])];
        let cases: Vec<(u32, Calls)> = vec![
            (
                0xc5,
                vec![a(0, 0), b(1, 0), a(2, 1), b(3, 1), f(4), v.clone()],
            ),
            (0xdb, vec![a(0, 0), b(1, 0), f(3), v.clone()]),
            (0x133, vec![b(0, 0), f(4), v.clone()]),
            (0xc6, base.clone()),
            (0xc4, base.clone()),
            (0x143, base),
        ];
        for (pass, expected) in cases {
            assert_eq!(
                run(&mut e, 0x00bd_02e0, this, block, pass),
                expected,
                "{pass:#x}"
            );
        }
    }
}
