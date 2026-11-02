//! `NiXenonRenderer/nid3dshaderprogramcreatorasm.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Virtual slots named below are the Xbox PDB's: on the creator `LoadShaderCodeFromFile`
//! (+0x24) and `LoadShaderCodeFromBuffer` (+0x28); on `NiD3DVertexShader` `SetName` (+0x54),
//! `SetShaderProgramName` (+0x5c), `GetCode` (+0x64), `SetCode` (+0x68), `SetCreator` (+0x70),
//! `GetUsage` (+0x7c), `SetUsage` (+0x80), `SetShaderHandle` (+0x88), `GetVertexDeclaration`
//! (+0x8c), `SetVertexDeclaration` (+0x90) and `SetSoftwareVertexProcessing` (+0x98); on
//! `NiD3DPixelShader` `SetShaderHandle` (+0x80).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DShaderProgramCreator::CreateVertexShader` (`00e98890`, `thiscall` with six words) and
/// `CreatePixelShader` (`00e988d0`, `thiscall` with one), unit
/// `nid3dshaderprogramcreator_xenon.cpp`.
const CREATE_VERTEX_SHADER_00E98890: u32 = 0x00e9_8890;
const CREATE_PIXEL_SHADER_00E988D0: u32 = 0x00e9_88d0;

// Translated from 00e978d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::RecreateVertexShader` (engine map), `thiscall` with the
/// vertex shader. False when the shader has no code (`GetCode`). Else its vertex declaration
/// (`GetVertexDeclaration`, kept in the argument word the code reuses, whose address is passed
/// on) and usage (`GetUsage`) go with the code to `CreateVertexShader` (`00e98890`; the last
/// three words 0); false when that answers 0, else the handle is set (`SetShaderHandle`) and
/// true answered.
pub fn ni_d3d_shader_program_creator_asm_recreate_vertex_shader(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
) -> bool {
    let code = e.vcall(shader, 0x64, &args![]).u32();
    if code == 0 {
        return false;
    }
    let declaration = e.vcall(shader, 0x8c, &args![]).u32();
    let handle = e.with_stack(4, |e, word| {
        let word = word.addr();
        e.mem.set_u32(word, declaration);
        let usage = e.vcall(shader, 0x7c, &args![]).u32();
        e.call(
            CREATE_VERTEX_SHADER_00E98890,
            &args![this, code, word, usage, 0u32, 0u32, 0u32],
        )
        .u32()
    });
    if handle == 0 {
        return false;
    }
    e.vcall(shader, 0x88, &args![handle]);
    true
}

// Translated from 00e97940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::RecreatePixelShader` (engine map), `thiscall` with the pixel
/// shader: false when it has no code (`GetCode`, slot 0x64); else `CreatePixelShader`
/// (`00e988d0`) makes the handle from it (0: false), which is set (`SetShaderHandle`, slot
/// 0x80); true.
pub fn ni_d3d_shader_program_creator_asm_recreate_pixel_shader(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
) -> bool {
    let code = e.vcall(shader, 0x64, &args![]).u32();
    if code == 0 {
        return false;
    }
    let handle = e
        .call(CREATE_PIXEL_SHADER_00E988D0, &args![this, code])
        .u32();
    if handle == 0 {
        return false;
    }
    e.vcall(shader, 0x80, &args![handle]);
    true
}

/// The allocation `00aa13e0` (`cdecl`, the size), the program constructor
/// `NiD3DShaderProgram::NiD3DShaderProgram` (`00e897d0`, `thiscall`: the renderer, the program
/// type) with the creators' renderer (`0126fa2c`), the shader vtables (`NiD3DVertexShader`
/// `010ef87c`, `NiD3DPixelShader` `010ef7d4`, from their RTTI), `NiShaderFactory::ReportError`
/// (`00aa0020`, `cdecl`) and the two failure formats.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const PROGRAM_CONSTRUCT_00E897D0: u32 = 0x00e8_97d0;
const RENDERER_0126FA2C: u32 = 0x0126_fa2c;
const VERTEX_SHADER_VTABLE_010EF87C: u32 = 0x010e_f87c;
const PIXEL_SHADER_VTABLE_010EF7D4: u32 = 0x010e_f7d4;
const REPORT_ERROR_00AA0020: u32 = 0x00aa_0020;
const VERTEX_FAILED_010F1258: u32 = 0x010f_1258;
const PIXEL_FAILED_010F1280: u32 = 0x010f_1280;

/// A new vertex shader as the two vertex creators build it: 0x3c bytes (`00aa13e0`; 0 when
/// that fails), the program constructor with the renderer and type 0, the
/// `NiD3DVertexShader` vtable, the byte +0x2c and the words +0x30 to +0x38 cleared.
fn new_vertex_shader(e: &mut Engine) -> u32 {
    let shader = e.call(ALLOCATE_00AA13E0, &args![0x3cu32]).u32();
    if shader == 0 {
        return 0;
    }
    let renderer = e.mem.u32(RENDERER_0126FA2C);
    e.call(PROGRAM_CONSTRUCT_00E897D0, &args![shader, renderer, 0u32]);
    e.mem.set_u32(shader, VERTEX_SHADER_VTABLE_010EF87C);
    e.mem.set_u8(shader + 0x2c, 0);
    for offset in [0x30u32, 0x34, 0x38] {
        e.mem.set_u32(shader + offset, 0);
    }
    shader
}

/// A new pixel shader as the two pixel creators build it: 0x30 bytes (`00aa13e0`; 0 when that
/// fails), the program constructor with the renderer and type 2, the `NiD3DPixelShader`
/// vtable and the handle (+0x2c) cleared.
fn new_pixel_shader(e: &mut Engine) -> u32 {
    let shader = e.call(ALLOCATE_00AA13E0, &args![0x30u32]).u32();
    if shader == 0 {
        return 0;
    }
    let renderer = e.mem.u32(RENDERER_0126FA2C);
    e.call(PROGRAM_CONSTRUCT_00E897D0, &args![shader, renderer, 2u32]);
    e.mem.set_u32(shader, PIXEL_SHADER_VTABLE_010EF7D4);
    e.mem.set_u32(shader + 0x2c, 0);
    shader
}

/// The four words the code loaders fill, in the order of the creators' stack frame: the code
/// (+0), two words the loaders only clear (+4, +8) and the code size (+0xc) for the vertex
/// creators; the pixel creators keep the size at +4.
const CODE: u32 = 0;
const WORD_4: u32 = 4;
const WORD_8: u32 = 8;
const WORD_C: u32 = 0xc;

// Translated from 00e97980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::CreateVertexShaderFromFile` (engine map), `thiscall` with
/// eight words: the file name, the shader name, two words it does not read, the vertex
/// declaration, the usage, the software vertex processing flag and the error code passed to
/// the report. Makes a vertex shader ([`new_vertex_shader`]; 0 when that fails) and loads the
/// code (`LoadShaderCodeFromFile`: the file, the addresses of the code, size and two more
/// stack words, all cleared first, and the error code); failing, the shader is deleted (its
/// slot 0 with 1) and 0 answered. `CreateVertexShader` (`00e98890`) then gets the code, the
/// address of the declaration's argument word, the usage, the two other words and the flag; a
/// null handle is reported (`00aa0020` with 1, the error code, `"Failed CreateVertexShader call
/// on %s"` and the shader name) and the shader deleted (0 answered). Else the shader gets its
/// name, program name (the file), code and size, creator (`this`), handle, usage, declaration
/// (read back from its argument word) and flag, and is answered.
pub fn ni_d3d_shader_program_creator_asm_create_vertex_shader_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
    _unused_3: u32,
    _unused_4: u32,
    declaration: u32,
    usage: u32,
    software: u32,
    error: u32,
) -> u32 {
    let shader = new_vertex_shader(e);
    if shader == 0 {
        return 0;
    }
    e.with_stack(0x14, |e, frame| {
        let words = frame.addr();
        let declaration_word = words + 0x10;
        for offset in [CODE, WORD_4, WORD_8, WORD_C] {
            e.mem.set_u32(words + offset, 0);
        }
        e.mem.set_u32(declaration_word, declaration);
        let loaded = e
            .vcall(
                this.addr(),
                0x24,
                &args![
                    file,
                    words + CODE,
                    words + WORD_C,
                    words + WORD_8,
                    words + WORD_4,
                    error
                ],
            )
            .bool();
        if !loaded {
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        let code = e.mem.u32(words + CODE);
        let word_4 = e.mem.u32(words + WORD_4);
        let word_8 = e.mem.u32(words + WORD_8);
        let handle = e
            .call(
                CREATE_VERTEX_SHADER_00E98890,
                &args![
                    this,
                    code,
                    declaration_word,
                    usage,
                    word_8,
                    word_4,
                    software
                ],
            )
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, error, VERTEX_FAILED_010F1258, name],
            );
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        e.vcall(shader, 0x54, &args![name]);
        e.vcall(shader, 0x5c, &args![file]);
        let size = e.mem.u32(words + WORD_C);
        let code = e.mem.u32(words + CODE);
        e.vcall(shader, 0x68, &args![size, code]);
        e.vcall(shader, 0x70, &args![this]);
        e.vcall(shader, 0x88, &args![handle]);
        e.vcall(shader, 0x80, &args![usage]);
        let declaration = e.mem.u32(declaration_word);
        e.vcall(shader, 0x90, &args![declaration]);
        e.vcall(shader, 0x98, &args![software]);
        shader
    })
}

// Translated from 00e97af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::CreateVertexShaderFromBuffer` (engine map), `thiscall` with
/// nine words: the buffer, its size, the shader name, two words it does not read, the vertex
/// declaration, the usage, the software vertex processing flag and the error code. As
/// [`ni_d3d_shader_program_creator_asm_create_vertex_shader_from_file`], with the code loaded
/// by `LoadShaderCodeFromBuffer` (the buffer and its size first) and the program name set to
/// null.
pub fn ni_d3d_shader_program_creator_asm_create_vertex_shader_from_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    buffer_size: u32,
    name: u32,
    _unused_4: u32,
    _unused_5: u32,
    declaration: u32,
    usage: u32,
    software: u32,
    error: u32,
) -> u32 {
    let shader = new_vertex_shader(e);
    if shader == 0 {
        return 0;
    }
    e.with_stack(0x14, |e, frame| {
        let words = frame.addr();
        let declaration_word = words + 0x10;
        for offset in [CODE, WORD_4, WORD_8, WORD_C] {
            e.mem.set_u32(words + offset, 0);
        }
        e.mem.set_u32(declaration_word, declaration);
        let loaded = e
            .vcall(
                this.addr(),
                0x28,
                &args![
                    buffer,
                    buffer_size,
                    words + CODE,
                    words + WORD_C,
                    words + WORD_8,
                    words + WORD_4,
                    error
                ],
            )
            .bool();
        if !loaded {
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        let code = e.mem.u32(words + CODE);
        let word_4 = e.mem.u32(words + WORD_4);
        let word_8 = e.mem.u32(words + WORD_8);
        let handle = e
            .call(
                CREATE_VERTEX_SHADER_00E98890,
                &args![
                    this,
                    code,
                    declaration_word,
                    usage,
                    word_8,
                    word_4,
                    software
                ],
            )
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, error, VERTEX_FAILED_010F1258, name],
            );
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        e.vcall(shader, 0x54, &args![name]);
        e.vcall(shader, 0x5c, &args![0u32]);
        let size = e.mem.u32(words + WORD_C);
        let code = e.mem.u32(words + CODE);
        e.vcall(shader, 0x68, &args![size, code]);
        e.vcall(shader, 0x70, &args![this]);
        e.vcall(shader, 0x88, &args![handle]);
        e.vcall(shader, 0x80, &args![usage]);
        let declaration = e.mem.u32(declaration_word);
        e.vcall(shader, 0x90, &args![declaration]);
        e.vcall(shader, 0x98, &args![software]);
        shader
    })
}

// Translated from 00e97c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::CreatePixelShaderFromFile` (engine map), `thiscall` with five
/// words: the file name, the shader name, two words it does not read and the error code. Makes
/// a pixel shader ([`new_pixel_shader`]; 0 when that fails) and loads the code
/// (`LoadShaderCodeFromFile`: the file, the addresses of the code, size and two more stack
/// words, all cleared first, and the error code); failing, the shader is deleted (its slot 0
/// with 1) and 0 answered. `CreatePixelShader` (`00e988d0`) makes the handle from the code; a
/// null one is reported (`00aa0020` with 1, the error code, `"Failed CreatePixelShader call on
/// %s"` and the shader name) and the shader deleted. Else the shader gets its name, program name
/// (the file), code and size, creator (`this`) and handle, and is answered.
pub fn ni_d3d_shader_program_creator_asm_create_pixel_shader_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
    _unused_3: u32,
    _unused_4: u32,
    error: u32,
) -> u32 {
    let shader = new_pixel_shader(e);
    if shader == 0 {
        return 0;
    }
    e.with_stack(0x10, |e, frame| {
        let words = frame.addr();
        for offset in [CODE, WORD_4, WORD_8, WORD_C] {
            e.mem.set_u32(words + offset, 0);
        }
        let loaded = e
            .vcall(
                this.addr(),
                0x24,
                &args![
                    file,
                    words + CODE,
                    words + WORD_4,
                    words + WORD_C,
                    words + WORD_8,
                    error
                ],
            )
            .bool();
        if !loaded {
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        let code = e.mem.u32(words + CODE);
        let handle = e
            .call(CREATE_PIXEL_SHADER_00E988D0, &args![this, code])
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, error, PIXEL_FAILED_010F1280, name],
            );
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        e.vcall(shader, 0x54, &args![name]);
        e.vcall(shader, 0x5c, &args![file]);
        let size = e.mem.u32(words + WORD_4);
        let code = e.mem.u32(words + CODE);
        e.vcall(shader, 0x68, &args![size, code]);
        e.vcall(shader, 0x70, &args![this]);
        e.vcall(shader, 0x80, &args![handle]);
        shader
    })
}

// Translated from 00e97d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::CreatePixelShaderFromBuffer` (engine map), `thiscall` with six
/// words: the buffer, its size, the shader name, two words it does not read and the error code.
/// As [`ni_d3d_shader_program_creator_asm_create_pixel_shader_from_file`], with the code loaded
/// by `LoadShaderCodeFromBuffer` (the buffer and its size first) and the program name set to
/// null.
pub fn ni_d3d_shader_program_creator_asm_create_pixel_shader_from_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    buffer_size: u32,
    name: u32,
    _unused_4: u32,
    _unused_5: u32,
    error: u32,
) -> u32 {
    let shader = new_pixel_shader(e);
    if shader == 0 {
        return 0;
    }
    e.with_stack(0x10, |e, frame| {
        let words = frame.addr();
        for offset in [CODE, WORD_4, WORD_8, WORD_C] {
            e.mem.set_u32(words + offset, 0);
        }
        let loaded = e
            .vcall(
                this.addr(),
                0x28,
                &args![
                    buffer,
                    buffer_size,
                    words + CODE,
                    words + WORD_4,
                    words + WORD_C,
                    words + WORD_8,
                    error
                ],
            )
            .bool();
        if !loaded {
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        let code = e.mem.u32(words + CODE);
        let handle = e
            .call(CREATE_PIXEL_SHADER_00E988D0, &args![this, code])
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, error, PIXEL_FAILED_010F1280, name],
            );
            e.vcall(shader, 0, &args![1u32]);
            return 0;
        }
        e.vcall(shader, 0x54, &args![name]);
        e.vcall(shader, 0x5c, &args![0u32]);
        let size = e.mem.u32(words + WORD_4);
        let code = e.mem.u32(words + CODE);
        e.vcall(shader, 0x68, &args![size, code]);
        e.vcall(shader, 0x70, &args![this]);
        e.vcall(shader, 0x80, &args![handle]);
        shader
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_78d0,
            ni_d3d_shader_program_creator_asm_recreate_vertex_shader(Ptr, u32) -> bool
        ),
        entry!(
            0x00e9_7940,
            ni_d3d_shader_program_creator_asm_recreate_pixel_shader(Ptr, u32) -> bool
        ),
        entry!(
            0x00e9_7980,
            ni_d3d_shader_program_creator_asm_create_vertex_shader_from_file(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_7af0,
            ni_d3d_shader_program_creator_asm_create_vertex_shader_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_7c60,
            ni_d3d_shader_program_creator_asm_create_pixel_shader_from_file(
                Ptr, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_7d80,
            ni_d3d_shader_program_creator_asm_create_pixel_shader_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    const CREATOR_VTABLE: u32 = 0x0e00_0000;
    const LOAD_FROM_FILE: u32 = 0x0e01_0024;
    const LOAD_FROM_BUFFER: u32 = 0x0e01_0028;
    /// The shader slots the creators call, at `0x0e10_0000 + slot`.
    const SHADER_SLOT_BASE: u32 = 0x0e10_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn slot(offset: u32) -> u32 {
        SHADER_SLOT_BASE + offset
    }

    /// The shader calls (slot, words after `this`) in order.
    fn shader_calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 & 0xffff_f000 == SHADER_SLOT_BASE)
            .map(|c| (c.0 - SHADER_SLOT_BASE, c.1[1..].to_vec()))
            .collect()
    }

    /// The shader vtable at `vtable` with every slot the creators use doubled.
    fn shader_vtable(e: &mut Engine, vtable: u32) {
        let slots: Vec<u32> = (0..0x28).map(|i| slot(4 * i)).collect();
        e.put_vtable(vtable, &slots);
        for i in 0..0x28u32 {
            e.register(slot(4 * i), |_, _| Ret::default());
        }
    }

    /// A creator whose loaders write code 0x0e40_0000 and size 0x80 (answering `loads`), an
    /// allocator, a constructor and the vertex and pixel vtables.
    fn rig(loads: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(RENDERER_0126FA2C, 0x0e30_0000);
        let mut slots = vec![0u32; 11];
        slots[9] = LOAD_FROM_FILE;
        slots[10] = LOAD_FROM_BUFFER;
        e.put_vtable(CREATOR_VTABLE, &slots);
        shader_vtable(&mut e, VERTEX_SHADER_VTABLE_010EF87C);
        shader_vtable(&mut e, PIXEL_SHADER_VTABLE_010EF7D4);
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PROGRAM_CONSTRUCT_00E897D0, |_, _| Ret::default());
        e.register(REPORT_ERROR_00AA0020, |_, _| Ret::default());
        for (loader, first_out) in [(LOAD_FROM_FILE, 2usize), (LOAD_FROM_BUFFER, 3)] {
            e.register_double(loader, move |e, a| {
                // Every out word is cleared before the call.
                for out in &a[first_out..first_out + 4] {
                    assert_eq!(e.mem.u32(*out), 0);
                }
                e.mem.set_u32(a[first_out], 0x0e40_0000);
                // The size (the second out word) 0x80, the other two 0x81 and 0x82.
                for (i, out) in a[first_out + 1..first_out + 4].iter().enumerate() {
                    e.mem.set_u32(*out, 0x80 + i as u32);
                }
                u32::from(loads).into_ret()
            });
        }
        let creator = e.mem.alloc(0x10);
        e.mem.set_u32(creator, CREATOR_VTABLE);
        e.call_log = Some(vec![]);
        (e, creator)
    }

    #[test]
    fn recreate_vertex_shader_passes_code_declaration_and_usage() {
        let mut e = Engine::new();
        shader_vtable(&mut e, 0x0e20_0000);
        e.register(slot(0x64), |_, _| 0x0e40_0000u32.into_ret());
        e.register(slot(0x8c), |_, _| 0x0e41_0000u32.into_ret());
        e.register(slot(0x7c), |_, _| 3u32.into_ret());
        e.register(CREATE_VERTEX_SHADER_00E98890, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0e41_0000);
            0x0e42_0000u32.into_ret()
        });
        let shader = e.mem.alloc(0x3c);
        e.mem.set_u32(shader, 0x0e20_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_78d0, &args![0x0e50_0000u32, shader]).bool());
        let created = calls(&e, CREATE_VERTEX_SHADER_00E98890);
        assert_eq!(created[0][..2], [0x0e50_0000, 0x0e40_0000]);
        assert_eq!(created[0][3..], [3, 0, 0, 0]);
        assert_eq!(calls(&e, slot(0x88)), vec![vec![shader, 0x0e42_0000]]);
        // No handle: false; no code: false at once.
        e.register(CREATE_VERTEX_SHADER_00E98890, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e9_78d0, &args![0x0e50_0000u32, shader]).bool());
        e.register(slot(0x64), |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_78d0, &args![0x0e50_0000u32, shader]).bool());
        assert!(calls(&e, CREATE_VERTEX_SHADER_00E98890).is_empty());
    }

    #[test]
    fn recreate_pixel_shader_sets_the_new_handle() {
        let mut e = Engine::new();
        shader_vtable(&mut e, 0x0e20_0000);
        e.register(slot(0x64), |_, _| 0x0e40_0000u32.into_ret());
        e.register(CREATE_PIXEL_SHADER_00E988D0, |_, _| {
            0x0e43_0000u32.into_ret()
        });
        let shader = e.mem.alloc(0x30);
        e.mem.set_u32(shader, 0x0e20_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_7940, &args![0x0e50_0000u32, shader]).bool());
        assert_eq!(
            calls(&e, CREATE_PIXEL_SHADER_00E988D0),
            vec![vec![0x0e50_0000, 0x0e40_0000]]
        );
        assert_eq!(calls(&e, slot(0x80)), vec![vec![shader, 0x0e43_0000]]);
        e.register(CREATE_PIXEL_SHADER_00E988D0, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e9_7940, &args![0x0e50_0000u32, shader]).bool());
        e.register(slot(0x64), |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e9_7940, &args![0x0e50_0000u32, shader]).bool());
    }

    #[test]
    fn create_vertex_shader_from_file_fills_the_new_shader() {
        let (mut e, creator) = rig(true);
        e.register(CREATE_VERTEX_SHADER_00E98890, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0e60_0000);
            0x0e42_0000u32.into_ret()
        });
        let call = args![creator, 0x0e70_0000u32, 0x0e71_0000u32, 0x11u32, 0x22u32];
        let call = [call, args![0x0e60_0000u32, 5u32, 1u32, 9u32]].concat();
        let shader = e.call(0x00e9_7980, &call).u32();
        assert_ne!(shader, 0);
        assert_eq!(e.mem.u32(shader), VERTEX_SHADER_VTABLE_010EF87C);
        assert_eq!(
            calls(&e, PROGRAM_CONSTRUCT_00E897D0),
            vec![vec![shader, 0x0e30_0000, 0]]
        );
        let load = &calls(&e, LOAD_FROM_FILE)[0];
        assert_eq!((load[1], load[6]), (0x0e70_0000, 9));
        // Out words: code, the size (frame +0xc), +8, +4.
        let created = &calls(&e, CREATE_VERTEX_SHADER_00E98890)[0];
        assert_eq!(created[1], 0x0e40_0000);
        // The fourth out word (frame +4) got 0x82, the third (+8) 0x81, the size (+0xc) 0x80.
        assert_eq!(created[3..], [5, 0x81, 0x82, 1]);
        assert_eq!(
            shader_calls(&e),
            vec![
                (0x54, vec![0x0e71_0000]),
                (0x5c, vec![0x0e70_0000]),
                (0x68, vec![0x80, 0x0e40_0000]),
                (0x70, vec![creator]),
                (0x88, vec![0x0e42_0000]),
                (0x80, vec![5]),
                (0x90, vec![0x0e60_0000]),
                (0x98, vec![1]),
            ]
        );
        // A null handle: reported, the shader deleted, 0.
        let (mut e, creator) = rig(true);
        e.register(CREATE_VERTEX_SHADER_00E98890, |_, _| 0u32.into_ret());
        let call = [
            args![creator, 0x0e70_0000u32, 0x0e71_0000u32, 0u32, 0u32],
            args![0u32, 0u32, 0u32, 9u32],
        ]
        .concat();
        assert_eq!(e.call(0x00e9_7980, &call).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 9, VERTEX_FAILED_010F1258, 0x0e71_0000]]
        );
        assert_eq!(shader_calls(&e), vec![(0, vec![1])]);
        // The load failing: deleted, 0.
        let (mut e, creator) = rig(false);
        let call = [
            args![creator, 0x0e70_0000u32, 0x0e71_0000u32, 0u32, 0u32],
            args![0u32, 0u32, 0u32, 9u32],
        ]
        .concat();
        assert_eq!(e.call(0x00e9_7980, &call).u32(), 0);
        assert_eq!(shader_calls(&e), vec![(0, vec![1])]);
        assert!(calls(&e, CREATE_VERTEX_SHADER_00E98890).is_empty());
    }

    #[test]
    fn create_vertex_shader_from_buffer_loads_from_the_buffer() {
        let (mut e, creator) = rig(true);
        e.register(CREATE_VERTEX_SHADER_00E98890, |_, _| {
            0x0e42_0000u32.into_ret()
        });
        let call = [
            args![
                creator,
                0x0e72_0000u32,
                0x400u32,
                0x0e71_0000u32,
                0u32,
                0u32
            ],
            args![0x0e60_0000u32, 5u32, 1u32, 9u32],
        ]
        .concat();
        let shader = e.call(0x00e9_7af0, &call).u32();
        assert_ne!(shader, 0);
        let load = &calls(&e, LOAD_FROM_BUFFER)[0];
        assert_eq!((load[1], load[2], load[7]), (0x0e72_0000, 0x400, 9));
        let seen = shader_calls(&e);
        assert_eq!(seen[1], (0x5c, vec![0]));
        assert_eq!(seen[2], (0x68, vec![0x80, 0x0e40_0000]));
        assert_eq!(seen[6], (0x90, vec![0x0e60_0000]));
        // The allocation failing: 0 and nothing loaded.
        let (mut e, creator) = rig(true);
        e.register(ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        let mut call = call;
        call[0] = creator;
        assert_eq!(e.call(0x00e9_7af0, &call).u32(), 0);
        assert!(calls(&e, LOAD_FROM_BUFFER).is_empty());
    }

    #[test]
    fn create_pixel_shader_from_file_fills_the_new_shader() {
        let (mut e, creator) = rig(true);
        e.register(CREATE_PIXEL_SHADER_00E988D0, |_, _| {
            0x0e43_0000u32.into_ret()
        });
        let call = args![creator, 0x0e70_0000u32, 0x0e71_0000u32, 0u32, 0u32, 9u32];
        let shader = e.call(0x00e9_7c60, &call).u32();
        assert_ne!(shader, 0);
        assert_eq!(e.mem.u32(shader), PIXEL_SHADER_VTABLE_010EF7D4);
        assert_eq!(
            calls(&e, PROGRAM_CONSTRUCT_00E897D0),
            vec![vec![shader, 0x0e30_0000, 2]]
        );
        assert_eq!(
            calls(&e, CREATE_PIXEL_SHADER_00E988D0),
            vec![vec![creator, 0x0e40_0000]]
        );
        // The pixel creators read the size from the second out word they pass.
        assert_eq!(
            shader_calls(&e),
            vec![
                (0x54, vec![0x0e71_0000]),
                (0x5c, vec![0x0e70_0000]),
                (0x68, vec![0x80, 0x0e40_0000]),
                (0x70, vec![creator]),
                (0x80, vec![0x0e43_0000]),
            ]
        );
        let (mut e, creator) = rig(true);
        e.register(CREATE_PIXEL_SHADER_00E988D0, |_, _| 0u32.into_ret());
        let call = args![creator, 0x0e70_0000u32, 0x0e71_0000u32, 0u32, 0u32, 9u32];
        assert_eq!(e.call(0x00e9_7c60, &call).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 9, PIXEL_FAILED_010F1280, 0x0e71_0000]]
        );
        assert_eq!(shader_calls(&e), vec![(0, vec![1])]);
    }

    #[test]
    fn create_pixel_shader_from_buffer_loads_from_the_buffer() {
        let (mut e, creator) = rig(true);
        e.register(CREATE_PIXEL_SHADER_00E988D0, |_, _| {
            0x0e43_0000u32.into_ret()
        });
        let call = args![
            creator,
            0x0e72_0000u32,
            0x400u32,
            0x0e71_0000u32,
            0u32,
            0u32,
            9u32
        ];
        let shader = e.call(0x00e9_7d80, &call).u32();
        assert_ne!(shader, 0);
        let load = &calls(&e, LOAD_FROM_BUFFER)[0];
        assert_eq!((load[1], load[2], load[7]), (0x0e72_0000, 0x400, 9));
        let seen = shader_calls(&e);
        assert_eq!(seen[1], (0x5c, vec![0]));
        assert_eq!(seen[2], (0x68, vec![0x80, 0x0e40_0000]));
        let (mut e, creator) = rig(false);
        let call = args![
            creator,
            0x0e72_0000u32,
            0x400u32,
            0x0e71_0000u32,
            0u32,
            0u32,
            9u32
        ];
        assert_eq!(e.call(0x00e9_7d80, &call).u32(), 0);
        assert_eq!(shader_calls(&e), vec![(0, vec![1])]);
    }
}
