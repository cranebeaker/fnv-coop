//! `NiXenonRenderer/nid3dshaderprogramcreatorhlsl.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1460` (`cdecl`, two words: block, size): the Ni sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `00e89970` (`thiscall`, no stack words): the destructor body `00be0af0` runs (engine map:
/// `NiD3DPixelShader`'s, unconfirmed).
const PIXEL_SHADER_DESTRUCT: u32 = 0x00e8_9970;
/// `00e89b70` (`thiscall`, no stack words): the destructor body `00be0b70` runs (engine map:
/// `NiD3DVertexShader`'s, unconfirmed).
const VERTEX_SHADER_DESTRUCT: u32 = 0x00e8_9b70;

// Translated from 00be0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x1c`
/// (`NiD3DShaderProgram::m_pkCreator` in the Xbox PDB; the body is a one-instruction getter that
/// other classes can share).
pub fn fn_00be0ad0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1c)
}

// Translated from 00be0af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with
/// one stack word (the flags): runs the destructor `00e89970`, and when bit 0 of the flags is
/// set frees the object with size 0x30 (`00aa1460`). Answers `this`.
pub fn ni_d3d_pixel_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PIXEL_SHADER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

// Translated from 00be0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x2c`
/// (`NiD3DPixelShader::m_hShader` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0b10(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2c)
}

// Translated from 00be0b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::SetShaderHandle` (Xbox PDB, engine map), `thiscall` with one stack word:
/// stores it at `+0x2c` (`m_hShader`).
pub fn ni_d3d_pixel_shader_set_shader_handle(e: &mut Engine, this: Ptr, handle: u32) {
    e.mem.set_u32(this.addr() + 0x2c, handle);
}

// Translated from 00be0b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with
/// one stack word (the flags): runs the destructor `00e89b70`, and when bit 0 of the flags is
/// set frees the object with size 0x3c (`00aa1460`). Answers `this`.
pub fn ni_d3d_vertex_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(VERTEX_SHADER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 00be0b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x30`
/// (`NiD3DVertexShader::m_uiUsage` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0b90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x30)
}

// Translated from 00be0bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetShaderHandle` (Xbox PDB, engine map), `thiscall` with one stack
/// word: stores it at `+0x34` (`m_hShader`).
pub fn ni_d3d_vertex_shader_set_shader_handle(e: &mut Engine, this: Ptr, handle: u32) {
    e.mem.set_u32(this.addr() + 0x34, handle);
}

// Translated from 00be0bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x38`
/// (`NiD3DVertexShader::m_hDecl` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0bc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x38)
}

// Translated from 00be0be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::GetSoftwareVertexProcessing` (Xbox PDB, engine map), `thiscall`, no
/// stack words: answers the byte at `+0x2c` (`m_bSoftwareVP`) in `AL`.
pub fn ni_d3d_vertex_shader_get_software_vertex_processing(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x2c)
}

// Translated from 00cc0760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiD3DShaderProgram::GetName` in the engine map (from a call graph, so unconfirmed:
/// the body is a one-instruction getter many classes can share), `thiscall`, no stack words:
/// answers the word at `+0xc` (`m_pszName` in `NiD3DShaderProgram`, Xbox PDB).
pub fn fn_00cc0760(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc)
}

// Translated from 00e8bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::GetEntryPoint` (Xbox PDB, engine map), `thiscall`, no stack words:
/// answers the word at `+0x3c` (`m_pcEntryPoint`).
pub fn ni_d3d_hlsl_vertex_shader_get_entry_point(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3c)
}

// Translated from 00e95d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::GetShaderTarget` (Xbox PDB, engine map), `thiscall`, no stack words:
/// answers the word at `+0x34` (`m_pcShaderTarget`).
pub fn ni_d3d_hlsl_pixel_shader_get_shader_target(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x34)
}

/// `00aa1070` (`cdecl`, one word: the size): the Ni block allocation the name setters use.
const NI_ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00aa10f0` (`cdecl`, one word): frees a block from `00aa1070` (null is passed too).
const NI_FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni object allocation (`NiNew`).
const NI_NEW: u32 = 0x00aa_13e0;
/// `_strcpy_s` (`00ec65a6`, `cdecl`: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_memcpy` (`00ec44d0`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// `00404d80` (`cdecl`, three words: destination, size, source): calls `_strcpy_s` with them
/// and answers the destination (unit `effectitem.cpp` in the engine map, unnamed).
const COPY_STRING_00404D80: u32 = 0x0040_4d80;
/// `NiShaderFactory::ReportError` (`00aa0020`, engine map; `cdecl` with a format and its
/// arguments: 1, the recoverable flag, the format, then the format's words).
const REPORT_ERROR: u32 = 0x00aa_0020;
/// `NiD3DShaderProgram::NiD3DShaderProgram` (`00e897d0`, engine map; `thiscall`, two words:
/// the renderer and the program type, 0 for a vertex and 2 for a pixel shader here).
const SHADER_PROGRAM_CONSTRUCT: u32 = 0x00e8_97d0;
/// `00e971d0` (`thiscall`, no stack words): the destructor body the HLSL vertex shader's
/// deleting destructor runs (unnamed in the engine map).
const HLSL_VERTEX_SHADER_DESTRUCT: u32 = 0x00e9_71d0;
/// `00e98890` (`thiscall`, six words: code, address of the declaration, usage, 0, 0, the
/// software-processing flag): the creator's D3D vertex shader creation; answers the handle.
/// The engine map calls it `NiD3DShaderProgramCreator::CreateVertexShader` (from its callers).
const CREATE_D3D_VERTEX_SHADER: u32 = 0x00e9_8890;
/// `00e988d0` (`thiscall`, one word: the code): the creator's D3D pixel shader creation;
/// answers the handle (the engine map's `NiD3DShaderProgramCreator::CreatePixelShader`).
const CREATE_D3D_PIXEL_SHADER: u32 = 0x00e9_88d0;
/// `NiD3DShaderProgramFactory::ResolveShaderFileName` (`00e930a0`, engine map; `cdecl`:
/// file name, path buffer, buffer size; answers a byte).
const RESOLVE_SHADER_FILE_NAME: u32 = 0x00e9_30a0;
/// The import thunks (`stdcall`), named by the import table entries their slots point at:
/// `D3DXCompileShader` (`00ee6e0a`, slot `00fdf408`, ten words), `D3DXCompileShaderFromFileA`
/// (`00ee6e46`, slot `00fdf3e0`, nine words), `D3DXGetShaderConstantTable` (`00ee6e40`, slot
/// `00fdf3e4`, two words), `D3DXGetVertexShaderProfile` (`00ee6e2e`, slot `00fdf3f0`) and
/// `D3DXGetPixelShaderProfile` (`00ee6e28`, slot `00fdf3f4`), each with the device.
const D3DX_COMPILE_SHADER: u32 = 0x00ee_6e0a;
const D3DX_COMPILE_SHADER_FROM_FILE: u32 = 0x00ee_6e46;
const D3DX_GET_SHADER_CONSTANT_TABLE: u32 = 0x00ee_6e40;
const D3DX_GET_VERTEX_SHADER_PROFILE: u32 = 0x00ee_6e2e;
const D3DX_GET_PIXEL_SHADER_PROFILE: u32 = 0x00ee_6e28;
/// The creators' statics, in the Xbox PDB's order (`NiD3DShaderProgramCreator`):
/// `ms_uiUniversalShaderCreationFlags` (`0126fa24`, or-ed into the compile flags),
/// `ms_pkD3DDevice` (`0126fa28`, given to the profile queries) and `ms_pkD3DRenderer`
/// (`0126fa2c`, given to the program constructor).
const UNIVERSAL_CREATION_FLAGS: u32 = 0x0126_fa24;
const CREATOR_DEVICE: u32 = 0x0126_fa28;
const CREATOR_RENDERER: u32 = 0x0126_fa2c;
/// The empty string (`01011584`) the name setters compare with.
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"main"` (`01094d90`): the entry point used when none is given.
const DEFAULT_ENTRY_POINT: u32 = 0x0109_4d90;
/// Error formats: `"Failed CreateVertexShader call on %s\n"` (`010f1258`), `"Failed
/// CreatePixelShader call on %s"` (`010f1280`), `"Invalid shader file name"` (`010f12a8`),
/// the two compile failures from a file (`010f12c4` without, `010f1304` with the compiler's
/// messages), `"Failed to find shader program file %s"` (`010f1338`), `"Invalid shader
/// buffer"` (`010f1360`) and the two assembly failures from memory (`010f1378` without,
/// `010f13b8` with the messages).
const FAILED_CREATE_VERTEX_SHADER: u32 = 0x010f_1258;
const FAILED_CREATE_PIXEL_SHADER: u32 = 0x010f_1280;
const INVALID_SHADER_FILE_NAME: u32 = 0x010f_12a8;
const FAILED_COMPILE_FILE: u32 = 0x010f_12c4;
const FAILED_COMPILE_FILE_MESSAGES: u32 = 0x010f_1304;
const FAILED_FIND_FILE: u32 = 0x010f_1338;
const INVALID_SHADER_BUFFER: u32 = 0x010f_1360;
const FAILED_ASSEMBLE_BUFFER: u32 = 0x010f_1378;
const FAILED_ASSEMBLE_BUFFER_MESSAGES: u32 = 0x010f_13b8;
/// The vtables the creators install: `NiD3DHLSLVertexShader` (`010f119c`) and
/// `NiD3DHLSLPixelShader` (`010f10f4`; their deleting destructors are slot 0).
const HLSL_VERTEX_SHADER_VTABLE: u32 = 0x010f_119c;
const HLSL_PIXEL_SHADER_VTABLE: u32 = 0x010f_10f4;
/// RTTI records answered by the `GetRTTI` slots.
const PIXEL_SHADER_RTTI: u32 = 0x0126_f9ac;
const VERTEX_SHADER_RTTI: u32 = 0x0126_f9b4;
const HLSL_PIXEL_SHADER_RTTI: u32 = 0x0126_fa40;
const HLSL_VERTEX_SHADER_RTTI: u32 = 0x0126_fa48;

/// `IUnknown::Release` on a COM object: its vtable slot +8, `stdcall` with the object.
fn release(e: &mut Engine, object: u32) {
    e.vcall(object, 8, &[]);
}

/// The object's virtual slot 0 (the deleting destructor) with 1: destroys and frees it.
fn delete_object(e: &mut Engine, object: u32) {
    e.vcall(object, 0, &[1]);
}

/// The two string setters at `00be0920` and `00be09e0`, inlined there on the word at
/// `field`: a null name or one equal to the empty string (`01011584`, compared byte by byte)
/// frees the held block (`00aa10f0`, also when it is null) and clears it. Otherwise a held
/// block shorter than the name is freed and cleared, a missing one is allocated with the
/// name's length plus 1 (`00aa1070`), and the name is copied in with `_strcpy_s`.
fn set_program_string(e: &mut Engine, field: u32, name: u32) {
    if name == 0 || e.mem.cstr(name) == e.mem.cstr(EMPTY_STRING) {
        let held = e.mem.u32(field);
        e.call(NI_FREE_BLOCK, &args![held]);
        e.mem.set_u32(field, 0);
        return;
    }
    let size = e.mem.cstr(name).len() as u32 + 1;
    let held = e.mem.u32(field);
    if held != 0 && (e.mem.cstr(held).len() as u32) < size {
        e.call(NI_FREE_BLOCK, &args![held]);
        e.mem.set_u32(field, 0);
    }
    if e.mem.u32(field) == 0 {
        let block = e.call(NI_ALLOCATE_BLOCK, &args![size]).u32();
        e.mem.set_u32(field, block);
    }
    let block = e.mem.u32(field);
    e.call(STRCPY_S, &args![block, size, name]);
}

// Translated from 00be0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::SetName` (Xbox PDB, engine map), `thiscall`, one word: keeps a copy
/// of the name at `+0xc` (`m_pszName`), reusing the held block when it is long enough; a null
/// or empty name frees it and clears the field.
pub fn ni_d3d_shader_program_set_name(e: &mut Engine, this: Ptr, name: u32) {
    set_program_string(e, this.addr() + 0xc, name);
}

// Translated from 00be09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::SetShaderProgramName` (Xbox PDB, engine map), `thiscall`, one word:
/// as `SetName` on `+0x10` (`m_pszShaderProgramName`).
pub fn ni_d3d_shader_program_set_shader_program_name(e: &mut Engine, this: Ptr, name: u32) {
    set_program_string(e, this.addr() + 0x10, name);
}

// Translated from 00be0aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x18`
/// (`NiD3DShaderProgram::m_pvCode` in the Xbox PDB; the shader vtables' slot +0x64; the
/// one-instruction body is shared with other classes, which use it as another getter).
pub fn fn_00be0aa0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x18)
}

// Translated from 00be0ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::SetCode` (Xbox PDB, engine map), `thiscall`, two words: the size to
/// `+0x14` (`m_uiCodeSize`) and the code to `+0x18` (`m_pvCode`).
pub fn ni_d3d_shader_program_set_code(e: &mut Engine, this: Ptr, size: u32, code: u32) {
    e.mem.set_u32(this.addr() + 0x14, size);
    e.mem.set_u32(this.addr() + 0x18, code);
}

// Translated from 00be0ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::GetRTTI` (Xbox PDB, engine map): answers its RTTI record `0126f9ac`.
pub fn ni_d3d_pixel_shader_get_rtti(_e: &mut Engine) -> u32 {
    PIXEL_SHADER_RTTI
}

// Translated from 00be0b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::GetRTTI` (Xbox PDB, engine map): answers its RTTI record `0126f9b4`.
pub fn ni_d3d_vertex_shader_get_rtti(_e: &mut Engine) -> u32 {
    VERTEX_SHADER_RTTI
}

// Translated from 00be0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetUsage` (Xbox PDB, engine map), `thiscall`, one word: stores it at
/// `+0x30` (`m_uiUsage`).
pub fn ni_d3d_vertex_shader_set_usage(e: &mut Engine, this: Ptr, usage: u32) {
    e.mem.set_u32(this.addr() + 0x30, usage);
}

// Translated from 00be0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetVertexDeclaration` (Xbox PDB, engine map), `thiscall`, one word:
/// stores it at `+0x38` (`m_hDecl`).
pub fn ni_d3d_vertex_shader_set_vertex_declaration(e: &mut Engine, this: Ptr, declaration: u32) {
    e.mem.set_u32(this.addr() + 0x38, declaration);
}

// Translated from 00be0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetSoftwareVertexProcessing` (Xbox PDB, engine map), `thiscall`, one
/// word: stores its low byte at `+0x2c` (`m_bSoftwareVP`).
pub fn ni_d3d_vertex_shader_set_software_vertex_processing(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2c, value);
}

// Translated from 00e95ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::RecreateVertexShader` (Xbox PDB, engine map), `thiscall`,
/// one word (the shader). Takes the shader's code (its slot +0x64); without code answers 0.
/// Creates the D3D shader (`00e98890` with the code, the address of a local holding the
/// declaration from slot +0x8c, the usage from slot +0x7c, 0, 0, 0); a null handle answers 0.
/// Otherwise sets the handle (slot +0x88), reads the code's constant table
/// (`D3DXGetShaderConstantTable`) into the shader's own argument slot and, when that
/// succeeds, sets it (slot +0xb8). The word then in that slot is released when not null:
/// the table, or the shader itself when the call failed without writing it. Answers 1.
pub fn ni_d3d_shader_program_creator_hlsl_recreate_vertex_shader(
    e: &mut Engine,
    this: Ptr,
    shader: Ptr,
) -> bool {
    let shader = shader.addr();
    let code = e.vcall(shader, 0x64, &[]).u32();
    if code == 0 {
        return false;
    }
    let declaration = e.vcall(shader, 0x8c, &[]).u32();
    e.with_stack(8, |e, locals| {
        let declaration_slot = locals.addr();
        // The argument slot the exe reuses for the constant table: it holds the shader.
        let table_slot = locals.addr() + 4;
        e.mem.set_u32(declaration_slot, declaration);
        e.mem.set_u32(table_slot, shader);
        let usage = e.vcall(shader, 0x7c, &[]).u32();
        let handle = e
            .call(
                CREATE_D3D_VERTEX_SHADER,
                &args![this, code, declaration_slot, usage, 0u32, 0u32, 0u32],
            )
            .u32();
        if handle == 0 {
            return false;
        }
        e.vcall(shader, 0x88, &[handle]);
        let result = e
            .call(D3DX_GET_SHADER_CONSTANT_TABLE, &args![code, table_slot])
            .i32();
        if result >= 0 {
            let table = e.mem.u32(table_slot);
            e.vcall(shader, 0xb8, &[table]);
        }
        let table = e.mem.u32(table_slot);
        if table != 0 {
            release(e, table);
        }
        true
    })
}

// Translated from 00e95c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::RecreatePixelShader` (Xbox PDB, engine map), `thiscall`,
/// one word (the shader): as `RecreateVertexShader` with the pixel creation (`00e988d0` with
/// the code), the handle set through slot +0x80 and the constant table through slot +0xa0.
pub fn ni_d3d_shader_program_creator_hlsl_recreate_pixel_shader(
    e: &mut Engine,
    this: Ptr,
    shader: Ptr,
) -> bool {
    let shader = shader.addr();
    let code = e.vcall(shader, 0x64, &[]).u32();
    if code == 0 {
        return false;
    }
    let handle = e.call(CREATE_D3D_PIXEL_SHADER, &args![this, code]).u32();
    if handle == 0 {
        return false;
    }
    e.vcall(shader, 0x80, &[handle]);
    e.with_stack(4, |e, table_slot| {
        // The argument slot the exe reuses for the constant table: it holds the shader.
        let table_slot = table_slot.addr();
        e.mem.set_u32(table_slot, shader);
        let result = e
            .call(D3DX_GET_SHADER_CONSTANT_TABLE, &args![code, table_slot])
            .i32();
        if result >= 0 {
            let table = e.mem.u32(table_slot);
            e.vcall(shader, 0xa0, &[table]);
        }
        let table = e.mem.u32(table_slot);
        if table != 0 {
            release(e, table);
        }
    });
    true
}

// Translated from 00e95cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::GetRTTI` (Xbox PDB, engine map): answers its RTTI record
/// `0126fa40`.
pub fn ni_d3d_hlsl_pixel_shader_get_rtti(_e: &mut Engine) -> u32 {
    HLSL_PIXEL_SHADER_RTTI
}

// Translated from 00e95cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall`
/// with one stack word (the flags): runs the destructor (`00e96c90`), and when bit 0 of the
/// flags is set frees the object with size 0x3c (`00aa1460`). Answers `this`.
pub fn ni_d3d_hlsl_pixel_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e96c90(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 00e95cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::SetEntryPoint` (Xbox PDB, engine map), `thiscall`, one word: when
/// it differs from the held `+0x30` (`m_pcEntryPoint`), frees the held copy (`00aa10f0`) and
/// keeps a new copy (`00aa1070` with the length plus 1, `_strcpy_s`), or clears the field for
/// a null or empty string.
pub fn ni_d3d_hlsl_pixel_shader_set_entry_point(e: &mut Engine, this: Ptr, entry_point: u32) {
    replace_string(e, this.addr() + 0x30, entry_point, true);
}

/// The string setters `00e95cf0`, `00e95d60`, `00e95e20` and `00e95e80`, inlined there: when
/// `value` differs from the word at `field`, the held block is freed and a non-empty `value`
/// is copied into a new block. For a null or empty `value` the entry-point setters clear the
/// field; the shader-target setters leave the freed pointer in it (`clear_when_empty`).
fn replace_string(e: &mut Engine, field: u32, value: u32, clear_when_empty: bool) {
    let held = e.mem.u32(field);
    if held == value {
        return;
    }
    e.call(NI_FREE_BLOCK, &args![held]);
    if value != 0 && e.mem.u8(value) != 0 {
        let size = e.mem.cstr(value).len() as u32 + 1;
        let block = e.call(NI_ALLOCATE_BLOCK, &args![size]).u32();
        e.call(STRCPY_S, &args![block, size, value]);
        e.mem.set_u32(field, block);
    } else if clear_when_empty {
        e.mem.set_u32(field, 0);
    }
}

/// The constant-table setters `00e95dc0` and `00e95ef0`, inlined there: when `table` differs
/// from the word at `field`, the held table is released (slot +8, `stdcall`) when not null,
/// `table` is stored and, when not null, referenced (its slot +4, `stdcall`).
fn replace_table(e: &mut Engine, field: u32, table: u32) {
    let held = e.mem.u32(field);
    if held == table {
        return;
    }
    if held != 0 {
        release(e, held);
    }
    e.mem.set_u32(field, table);
    if table != 0 {
        e.vcall(table, 4, &[]);
    }
}

// Translated from 00e95d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::SetShaderTarget` (Xbox PDB, engine map), `thiscall`, one word: as
/// `SetEntryPoint` on `+0x34` (`m_pcShaderTarget`), except that a null or empty target leaves
/// the freed pointer in the field (the exe does not clear it).
pub fn ni_d3d_hlsl_pixel_shader_set_shader_target(e: &mut Engine, this: Ptr, target: u32) {
    replace_string(e, this.addr() + 0x34, target, false);
}

// Translated from 00e95dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::SetConstantTable` (Xbox PDB, engine map), `thiscall`, one word:
/// replaces the table at `+0x38` (`m_pkConstantTable`), releasing the old one and
/// referencing the new one.
pub fn ni_d3d_hlsl_pixel_shader_set_constant_table(e: &mut Engine, this: Ptr, table: u32) {
    replace_table(e, this.addr() + 0x38, table);
}

// Translated from 00e95df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::GetRTTI` (Xbox PDB, engine map): answers its RTTI record
/// `0126fa48`.
pub fn ni_d3d_hlsl_vertex_shader_get_rtti(_e: &mut Engine) -> u32 {
    HLSL_VERTEX_SHADER_RTTI
}

// Translated from 00e95e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall`
/// with one stack word (the flags): runs the destructor `00e971d0`, and when bit 0 of the
/// flags is set frees the object with size 0x48 (`00aa1460`). Answers `this`.
pub fn ni_d3d_hlsl_vertex_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(HLSL_VERTEX_SHADER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00e95e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::SetEntryPoint` (Xbox PDB, engine map), `thiscall`, one word: as
/// the pixel shader's on `+0x3c` (`m_pcEntryPoint`; cleared for a null or empty string).
pub fn ni_d3d_hlsl_vertex_shader_set_entry_point(e: &mut Engine, this: Ptr, entry_point: u32) {
    replace_string(e, this.addr() + 0x3c, entry_point, true);
}

// Translated from 00e95e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::SetShaderTarget` (Xbox PDB, engine map), `thiscall`, one word: as
/// the pixel shader's on `+0x40` (`m_pcShaderTarget`; a null or empty target leaves the freed
/// pointer in the field).
pub fn ni_d3d_hlsl_vertex_shader_set_shader_target(e: &mut Engine, this: Ptr, target: u32) {
    replace_string(e, this.addr() + 0x40, target, false);
}

// Translated from 00e95ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the HLSL vertex shader's slot
/// +0xb4): answers the word at `+0x44` (`NiD3DHLSLVertexShader::m_pkConstantTable` in the
/// Xbox PDB).
pub fn fn_00e95ee0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x44)
}

// Translated from 00e95ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::SetConstantTable` (Xbox PDB, engine map), `thiscall`, one word:
/// replaces the table at `+0x44`, releasing the old one and referencing the new one.
pub fn ni_d3d_hlsl_vertex_shader_set_constant_table(e: &mut Engine, this: Ptr, table: u32) {
    replace_table(e, this.addr() + 0x44, table);
}

/// The inlined `NiD3DHLSLVertexShader` constructor of the vertex creators: the program
/// constructor (`00e897d0` with the creators' renderer and type 0), then `+0x2c` (byte),
/// `+0x30`..`+0x38` cleared, the HLSL vertex shader vtable, `+0x3c`..`+0x44` cleared.
fn construct_hlsl_vertex_shader(e: &mut Engine, shader: u32) {
    let renderer = e.global::<u32>(CREATOR_RENDERER);
    e.call(SHADER_PROGRAM_CONSTRUCT, &args![shader, renderer, 0u32]);
    e.mem.set_u8(shader + 0x2c, 0);
    e.mem.set_u32(shader + 0x30, 0);
    e.mem.set_u32(shader + 0x34, 0);
    e.mem.set_u32(shader + 0x38, 0);
    e.mem.set_u32(shader, HLSL_VERTEX_SHADER_VTABLE);
    e.mem.set_u32(shader + 0x3c, 0);
    e.mem.set_u32(shader + 0x40, 0);
    e.mem.set_u32(shader + 0x44, 0);
}

/// The inlined `NiD3DHLSLPixelShader` constructor of the pixel creators: the program
/// constructor (`00e897d0` with the renderer and type 2), `+0x2c` cleared, the HLSL pixel
/// shader vtable, `+0x30`..`+0x38` cleared.
fn construct_hlsl_pixel_shader(e: &mut Engine, shader: u32) {
    let renderer = e.global::<u32>(CREATOR_RENDERER);
    e.call(SHADER_PROGRAM_CONSTRUCT, &args![shader, renderer, 2u32]);
    e.mem.set_u32(shader + 0x2c, 0);
    e.mem.set_u32(shader, HLSL_PIXEL_SHADER_VTABLE);
    e.mem.set_u32(shader + 0x30, 0);
    e.mem.set_u32(shader + 0x34, 0);
    e.mem.set_u32(shader + 0x38, 0);
}

/// The vertex creators' common tail once the D3D shader exists: name (slot +0x54), program
/// name (+0x5c), code (+0x68: size, code), creator (+0x70), handle (+0x88), usage (+0x80),
/// software processing (+0x98), entry point (+0xa8), target (+0xb0), constant table (+0xb8),
/// then the table released when not null.
fn fill_vertex_shader(
    e: &mut Engine,
    shader: u32,
    creator: u32,
    name: u32,
    program_name: u32,
    code: (u32, u32),
    handle: u32,
    usage: u32,
    software_vp: u32,
    entry_point: u32,
    shader_target: u32,
    table: u32,
) {
    e.vcall(shader, 0x54, &[name]);
    e.vcall(shader, 0x5c, &[program_name]);
    e.vcall(shader, 0x68, &[code.0, code.1]);
    e.vcall(shader, 0x70, &[creator]);
    e.vcall(shader, 0x88, &[handle]);
    e.vcall(shader, 0x80, &[usage]);
    e.vcall(shader, 0x98, &[software_vp]);
    e.vcall(shader, 0xa8, &[entry_point]);
    e.vcall(shader, 0xb0, &[shader_target]);
    let held = e.mem.u32(table);
    e.vcall(shader, 0xb8, &[held]);
    let held = e.mem.u32(table);
    if held != 0 {
        release(e, held);
    }
}

/// The pixel creators' common tail: name (slot +0x54), program name (+0x5c), code (+0x68:
/// size, code), creator (+0x70), handle (+0x80), entry point (+0x90), target (+0x98),
/// constant table (+0xa0), then the table released when not null.
fn fill_pixel_shader(
    e: &mut Engine,
    shader: u32,
    creator: u32,
    name: u32,
    program_name: u32,
    code: (u32, u32),
    handle: u32,
    entry_point: u32,
    shader_target: u32,
    table: u32,
) {
    e.vcall(shader, 0x54, &[name]);
    e.vcall(shader, 0x5c, &[program_name]);
    e.vcall(shader, 0x68, &[code.0, code.1]);
    e.vcall(shader, 0x70, &[creator]);
    e.vcall(shader, 0x80, &[handle]);
    e.vcall(shader, 0x90, &[entry_point]);
    e.vcall(shader, 0x98, &[shader_target]);
    let held = e.mem.u32(table);
    e.vcall(shader, 0xa0, &[held]);
    let held = e.mem.u32(table);
    if held != 0 {
        release(e, held);
    }
}

// Translated from 00e95f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreateVertexShaderFromD3DXBuffer` (Xbox PDB, engine map),
/// `thiscall`, nine words (`ID3DXBuffer*`, code size, name, entry point, target, declaration,
/// usage, software processing, recoverable). Allocates and constructs an HLSL vertex shader
/// (0x48 bytes; 0 when the allocation fails), reads the constant table of the buffer's code
/// (a failure answers 0 and leaves the shader allocated), defaults the entry point to
/// `"main"` and the target to the device's vertex profile, and creates the D3D shader
/// (`00e98890`: code, the declaration argument's address, usage, 0, 0, software processing).
/// Without a handle it reports `"Failed CreateVertexShader call on %s\n"` with the name,
/// deletes the shader and answers 0; otherwise it copies the buffer's code into a block of the
/// given size, fills the shader (program name 0) and answers it.
pub fn ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_d3dx_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    code_size: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    declaration: u32,
    usage: u32,
    software_vp: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x48u32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_vertex_shader(e, shader);
    e.with_stack(8, |e, locals| {
        let table = locals.addr();
        let declaration_slot = locals.addr() + 4;
        e.mem.set_u32(declaration_slot, declaration);
        e.mem.set_u32(table, 0);
        let code = e.vcall(buffer, 0xc, &[]).u32();
        let result = e
            .call(D3DX_GET_SHADER_CONSTANT_TABLE, &args![code, table])
            .i32();
        if result < 0 {
            return Ptr::NULL;
        }
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_VERTEX_SHADER_PROFILE);
        let code = e.vcall(buffer, 0xc, &[]).u32();
        let handle = e
            .call(
                CREATE_D3D_VERTEX_SHADER,
                &args![this, code, declaration_slot, usage, 0u32, 0u32, software_vp],
            )
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_VERTEX_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let block = e.call(NI_ALLOCATE_BLOCK, &args![code_size]).u32();
        let size = e.vcall(buffer, 0x10, &[]).u32();
        let source = e.vcall(buffer, 0xc, &[]).u32();
        e.call(MEMCPY, &args![block, source, size]);
        fill_vertex_shader(
            e,
            shader,
            this.addr(),
            name,
            0,
            (code_size, block),
            handle,
            usage,
            software_vp,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

// Translated from 00e96100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreatePixelShaderFromD3DXBuffer` (Xbox PDB, engine map),
/// `thiscall`, six words (`ID3DXBuffer*`, code size, name, entry point, target,
/// recoverable): as the vertex one with an HLSL pixel shader (0x3c bytes), the device's pixel
/// profile, the pixel creation (`00e988d0` with the code) and `"Failed CreatePixelShader call
/// on %s"`.
pub fn ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_d3dx_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    code_size: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x3cu32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_pixel_shader(e, shader);
    e.with_stack(4, |e, table| {
        let table = table.addr();
        e.mem.set_u32(table, 0);
        let code = e.vcall(buffer, 0xc, &[]).u32();
        let result = e
            .call(D3DX_GET_SHADER_CONSTANT_TABLE, &args![code, table])
            .i32();
        if result < 0 {
            return Ptr::NULL;
        }
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_PIXEL_SHADER_PROFILE);
        let code = e.vcall(buffer, 0xc, &[]).u32();
        let handle = e.call(CREATE_D3D_PIXEL_SHADER, &args![this, code]).u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_PIXEL_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let block = e.call(NI_ALLOCATE_BLOCK, &args![code_size]).u32();
        let size = e.vcall(buffer, 0x10, &[]).u32();
        let source = e.vcall(buffer, 0xc, &[]).u32();
        e.call(MEMCPY, &args![block, source, size]);
        fill_pixel_shader(
            e,
            shader,
            this.addr(),
            name,
            0,
            (code_size, block),
            handle,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

/// The compile failure path the two loaders share: with an error buffer whose text is not
/// null, the text is copied (`00aa1070` with the buffer's size, `00404d80`) and reported with
/// the entry point, `source` (a file only) and the copy; without an error buffer the failure
/// is reported with the entry point (and `source`). The error buffer is released when there
/// is one, the copy freed (`00aa10f0`, null too) and the shader buffer released when not null.
fn report_compile_failure(
    e: &mut Engine,
    shader_slot: u32,
    errors_slot: u32,
    entry_point: u32,
    source: Option<u32>,
    recoverable: u32,
    formats: (u32, u32),
) {
    let errors = e.mem.u32(errors_slot);
    let mut copy = 0;
    if errors != 0 {
        let text = e.vcall(errors, 0xc, &[]).u32();
        if text != 0 {
            let errors = e.mem.u32(errors_slot);
            let size = e.vcall(errors, 0x10, &[]).u32();
            copy = e.call(NI_ALLOCATE_BLOCK, &args![size]).u32();
            e.call(COPY_STRING_00404D80, &args![copy, size, text]);
            match source {
                Some(file) => e.call(
                    REPORT_ERROR,
                    &args![1u32, recoverable, formats.1, entry_point, file, copy],
                ),
                None => e.call(
                    REPORT_ERROR,
                    &args![1u32, recoverable, formats.1, entry_point, copy],
                ),
            };
        }
        let errors = e.mem.u32(errors_slot);
        release(e, errors);
    } else {
        match source {
            Some(file) => e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, formats.0, entry_point, file],
            ),
            None => e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, formats.0, entry_point],
            ),
        };
    }
    e.call(NI_FREE_BLOCK, &args![copy]);
    let shader_buffer = e.mem.u32(shader_slot);
    if shader_buffer != 0 {
        release(e, shader_buffer);
    }
}

/// The compile success path the two loaders share: the shader buffer's size goes to
/// `size_out`, a block of that size (`00aa1070`) to `code_out`, the buffer's code is copied
/// into it, the shader buffer is released and the error buffer too when not null.
fn keep_compiled_code(
    e: &mut Engine,
    shader_slot: u32,
    errors_slot: u32,
    code_out: u32,
    size_out: u32,
) {
    let shader_buffer = e.mem.u32(shader_slot);
    let size = e.vcall(shader_buffer, 0x10, &[]).u32();
    e.mem.set_u32(size_out, size);
    let block = e.call(NI_ALLOCATE_BLOCK, &args![size]).u32();
    e.mem.set_u32(code_out, block);
    let shader_buffer = e.mem.u32(shader_slot);
    let size = e.mem.u32(size_out);
    let source = e.vcall(shader_buffer, 0xc, &[]).u32();
    let block = e.mem.u32(code_out);
    e.call(MEMCPY, &args![block, source, size]);
    let shader_buffer = e.mem.u32(shader_slot);
    release(e, shader_buffer);
    let errors = e.mem.u32(errors_slot);
    if errors != 0 {
        release(e, errors);
    }
}

// Translated from 00e962a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e962a0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramCreatorHLSL::LoadShaderCodeFromFile` (Xbox PDB, engine map), `thiscall`,
/// seven words (file, entry point, target, code out, size out, constant table out,
/// recoverable). Clears the code and size, reports `"Invalid shader file name"` for a null or
/// empty file and `"Failed to find shader program file %s"` when the factory cannot resolve
/// it (`00e930a0` into a 0x104-byte path buffer), answering 0. Otherwise compiles the file
/// (`D3DXCompileShaderFromFileA`: path, 0, 0, entry point, target, the creator's flags at +4
/// or-ed with the universal ones, the shader and error buffers in locals, the table out). On
/// success it keeps a copy of the code and answers 1; on failure it reports (with the
/// compiler's text when there is some), releases the buffers and answers 0. The locals start
/// as the engine's stack block (zero) where the exe leaves whatever its stack held.
pub fn ni_d3d_shader_program_creator_hlsl_load_shader_code_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    entry_point: u32,
    shader_target: u32,
    code_out: u32,
    size_out: u32,
    table_out: u32,
    recoverable: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    if file == 0 || e.mem.u8(file) == 0 {
        e.call(
            REPORT_ERROR,
            &args![1u32, recoverable, INVALID_SHADER_FILE_NAME],
        );
        return false;
    }
    e.with_stack(0x10c, |e, locals| {
        let path = locals.addr();
        let shader_slot = locals.addr() + 0x104;
        let errors_slot = locals.addr() + 0x108;
        let found = e
            .call(RESOLVE_SHADER_FILE_NAME, &args![file, path, 0x104u32])
            .bool();
        if !found {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_FIND_FILE, file],
            );
            return false;
        }
        let flags = e.mem.u32(this.addr() + 4) | e.global::<u32>(UNIVERSAL_CREATION_FLAGS);
        let result = e
            .call(
                D3DX_COMPILE_SHADER_FROM_FILE,
                &args![
                    path,
                    0u32,
                    0u32,
                    entry_point,
                    shader_target,
                    flags,
                    shader_slot,
                    errors_slot,
                    table_out
                ],
            )
            .i32();
        if result >= 0 {
            keep_compiled_code(e, shader_slot, errors_slot, code_out, size_out);
            return true;
        }
        report_compile_failure(
            e,
            shader_slot,
            errors_slot,
            entry_point,
            Some(file),
            recoverable,
            (FAILED_COMPILE_FILE, FAILED_COMPILE_FILE_MESSAGES),
        );
        false
    })
}

// Translated from 00e964a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::LoadShaderCodeFromBuffer` (Xbox PDB, engine map),
/// `thiscall`, eight words (source, source size, entry point, target, code out, size out,
/// constant table out, recoverable). Clears the code and size; a null source or a zero size
/// reports `"Invalid shader buffer"` and answers 0. Otherwise compiles the source
/// (`D3DXCompileShader`: source, size, 0, 0, entry point, target, the creator's flags at +4
/// or-ed with the universal ones, then the shader and error buffers, which the exe writes
/// into its own code-out and size-out argument slots, and the table out). Success keeps a
/// copy of the code and answers 1; failure reports (with the compiler's text when there is
/// some), releases the buffers and answers 0. The two argument slots start with the out
/// pointers they held.
pub fn ni_d3d_shader_program_creator_hlsl_load_shader_code_from_buffer(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    source_size: u32,
    entry_point: u32,
    shader_target: u32,
    code_out: u32,
    size_out: u32,
    table_out: u32,
    recoverable: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    if source == 0 || source_size == 0 {
        e.call(
            REPORT_ERROR,
            &args![1u32, recoverable, INVALID_SHADER_BUFFER],
        );
        return false;
    }
    let flags = e.mem.u32(this.addr() + 4) | e.global::<u32>(UNIVERSAL_CREATION_FLAGS);
    e.with_stack(8, |e, slots| {
        let shader_slot = slots.addr();
        let errors_slot = slots.addr() + 4;
        e.mem.set_u32(shader_slot, code_out);
        e.mem.set_u32(errors_slot, size_out);
        let result = e
            .call(
                D3DX_COMPILE_SHADER,
                &args![
                    source,
                    source_size,
                    0u32,
                    0u32,
                    entry_point,
                    shader_target,
                    flags,
                    shader_slot,
                    errors_slot,
                    table_out
                ],
            )
            .i32();
        if result >= 0 {
            keep_compiled_code(e, shader_slot, errors_slot, code_out, size_out);
            return true;
        }
        report_compile_failure(
            e,
            shader_slot,
            errors_slot,
            entry_point,
            None,
            recoverable,
            (FAILED_ASSEMBLE_BUFFER, FAILED_ASSEMBLE_BUFFER_MESSAGES),
        );
        false
    })
}

/// The entry point, or `"main"` when it is null.
fn entry_point_or_main(entry_point: u32) -> u32 {
    if entry_point == 0 {
        DEFAULT_ENTRY_POINT
    } else {
        entry_point
    }
}

/// The target, or the device's profile (`D3DXGetVertexShaderProfile` or
/// `D3DXGetPixelShaderProfile` at `profile`) when it is null.
fn target_or_profile(e: &mut Engine, shader_target: u32, profile: u32) -> u32 {
    if shader_target == 0 {
        let device = e.global::<u32>(CREATOR_DEVICE);
        e.call(profile, &args![device]).u32()
    } else {
        shader_target
    }
}

// Translated from 00e96610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreateVertexShaderFromFile` (Xbox PDB, engine map),
/// `thiscall`, eight words (file, name, entry point, target, declaration, usage, software
/// processing, recoverable). Allocates and constructs an HLSL vertex shader (0 when that
/// fails), defaults the entry point and the target, and loads the code (`00e962a0` into
/// three locals: code, size, table); a failed load deletes the shader and answers 0. Then
/// creates the D3D shader (`00e98890`: code, the declaration argument's address, usage, 0, 0,
/// software processing): without a handle it reports `"Failed CreateVertexShader call on
/// %s\n"` with the name, deletes the shader and answers 0 (the code block and the table are
/// left as they are). Otherwise fills the shader (program name: the file) and answers it.
pub fn ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    declaration: u32,
    usage: u32,
    software_vp: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x48u32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_vertex_shader(e, shader);
    e.with_stack(0x10, |e, locals| {
        let (code, table, size) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        for local in [size, code, table] {
            e.mem.set_u32(local, 0);
        }
        let declaration_slot = locals.addr() + 0xc;
        e.mem.set_u32(declaration_slot, declaration);
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_VERTEX_SHADER_PROFILE);
        let loaded = ni_d3d_shader_program_creator_hlsl_load_shader_code_from_file(
            e,
            this,
            file,
            entry_point,
            shader_target,
            code,
            size,
            table,
            recoverable,
        );
        if !loaded {
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_block = e.mem.u32(code);
        let handle = e
            .call(
                CREATE_D3D_VERTEX_SHADER,
                &args![
                    this,
                    code_block,
                    declaration_slot,
                    usage,
                    0u32,
                    0u32,
                    software_vp
                ],
            )
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_VERTEX_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_pair = (e.mem.u32(size), e.mem.u32(code));
        fill_vertex_shader(
            e,
            shader,
            this.addr(),
            name,
            file,
            code_pair,
            handle,
            usage,
            software_vp,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

// Translated from 00e967d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreateVertexShaderFromBuffer` (Xbox PDB, engine map),
/// `thiscall`, nine words (source, source size, name, entry point, target, declaration,
/// usage, software processing, recoverable): as `CreateVertexShaderFromFile` with the code
/// loaded by `00e964a0` (source, size, entry point, target) and program name 0.
pub fn ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_buffer(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    source_size: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    declaration: u32,
    usage: u32,
    software_vp: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x48u32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_vertex_shader(e, shader);
    e.with_stack(0x10, |e, locals| {
        let (code, table, size) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        for local in [size, code, table] {
            e.mem.set_u32(local, 0);
        }
        let declaration_slot = locals.addr() + 0xc;
        e.mem.set_u32(declaration_slot, declaration);
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_VERTEX_SHADER_PROFILE);
        let loaded = ni_d3d_shader_program_creator_hlsl_load_shader_code_from_buffer(
            e,
            this,
            source,
            source_size,
            entry_point,
            shader_target,
            code,
            size,
            table,
            recoverable,
        );
        if !loaded {
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_block = e.mem.u32(code);
        let handle = e
            .call(
                CREATE_D3D_VERTEX_SHADER,
                &args![
                    this,
                    code_block,
                    declaration_slot,
                    usage,
                    0u32,
                    0u32,
                    software_vp
                ],
            )
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_VERTEX_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_pair = (e.mem.u32(size), e.mem.u32(code));
        fill_vertex_shader(
            e,
            shader,
            this.addr(),
            name,
            0,
            code_pair,
            handle,
            usage,
            software_vp,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

// Translated from 00e96990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreatePixelShaderFromFile` (Xbox PDB, engine map),
/// `thiscall`, five words (file, name, entry point, target, recoverable): allocates and
/// constructs an HLSL pixel shader (0 when that fails), defaults the entry point and the
/// target (the device's pixel profile), loads the code (`00e962a0`; a failure deletes the
/// shader and answers 0) and creates the D3D shader (`00e988d0` with the code). Without a
/// handle it reports `"Failed CreatePixelShader call on %s"` with the name, deletes the shader
/// and answers 0; otherwise fills the shader (program name: the file) and answers it.
pub fn ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x3cu32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_pixel_shader(e, shader);
    e.with_stack(0xc, |e, locals| {
        let (code, table, size) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        for local in [size, code, table] {
            e.mem.set_u32(local, 0);
        }
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_PIXEL_SHADER_PROFILE);
        let loaded = ni_d3d_shader_program_creator_hlsl_load_shader_code_from_file(
            e,
            this,
            file,
            entry_point,
            shader_target,
            code,
            size,
            table,
            recoverable,
        );
        if !loaded {
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_block = e.mem.u32(code);
        let handle = e
            .call(CREATE_D3D_PIXEL_SHADER, &args![this, code_block])
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_PIXEL_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_pair = (e.mem.u32(size), e.mem.u32(code));
        fill_pixel_shader(
            e,
            shader,
            this.addr(),
            name,
            file,
            code_pair,
            handle,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

// Translated from 00e96b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::CreatePixelShaderFromBuffer` (Xbox PDB, engine map),
/// `thiscall`, six words (source, source size, name, entry point, target, recoverable): as
/// `CreatePixelShaderFromFile` with the code loaded by `00e964a0` and program name 0.
pub fn ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_buffer(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    source_size: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    recoverable: u32,
) -> Ptr {
    let shader = e.call(NI_NEW, &args![0x3cu32]).u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    construct_hlsl_pixel_shader(e, shader);
    e.with_stack(0xc, |e, locals| {
        let (code, table, size) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        for local in [size, code, table] {
            e.mem.set_u32(local, 0);
        }
        let entry_point = entry_point_or_main(entry_point);
        let shader_target = target_or_profile(e, shader_target, D3DX_GET_PIXEL_SHADER_PROFILE);
        let loaded = ni_d3d_shader_program_creator_hlsl_load_shader_code_from_buffer(
            e,
            this,
            source,
            source_size,
            entry_point,
            shader_target,
            code,
            size,
            table,
            recoverable,
        );
        if !loaded {
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_block = e.mem.u32(code);
        let handle = e
            .call(CREATE_D3D_PIXEL_SHADER, &args![this, code_block])
            .u32();
        if handle == 0 {
            e.call(
                REPORT_ERROR,
                &args![1u32, recoverable, FAILED_CREATE_PIXEL_SHADER, name],
            );
            delete_object(e, shader);
            return Ptr::NULL;
        }
        let code_pair = (e.mem.u32(size), e.mem.u32(code));
        fill_pixel_shader(
            e,
            shader,
            this.addr(),
            name,
            0,
            code_pair,
            handle,
            entry_point,
            shader_target,
            table,
        );
        Ptr::new(shader)
    })
}

// Translated from 00e96c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiD3DHLSLPixelShader` destructor (`thiscall`, no stack words; unnamed in the engine
/// map, run by `00e95cd0`): installs the HLSL pixel shader vtable, frees the entry point
/// (`+0x30`) and target (`+0x34`) copies (`00aa10f0`, null too), releases and clears the
/// constant table (`+0x38`) when set, then runs `00e89970` (the pixel shader destructor body,
/// a tail jump).
pub fn fn_00e96c90(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let entry_point = e.mem.u32(t + 0x30);
    e.mem.set_u32(t, HLSL_PIXEL_SHADER_VTABLE);
    e.call(NI_FREE_BLOCK, &args![entry_point]);
    let target = e.mem.u32(t + 0x34);
    e.call(NI_FREE_BLOCK, &args![target]);
    let table = e.mem.u32(t + 0x38);
    if table != 0 {
        release(e, table);
        e.mem.set_u32(t + 0x38, 0);
    }
    e.call(PIXEL_SHADER_DESTRUCT, &args![this]);
}

/// The import slot `00ee6e2e` jumps through (`D3DXGetVertexShaderProfile`, see above).
const IMPORT_SLOT_00EE6E2E: u32 = 0x00fd_f3f0;

// Translated from 00ee6e2e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3f0]` (Ghidra labels the import
/// `D3DXGetVertexShaderProfile`; the engine map's `NiD3DHLSLVertexShader` name
/// is folded code). stdcall: the one stack word (the device; callers at
/// 00e6bddc and 00e9667f push one) goes unchanged to the import and its result
/// (the profile text) comes back.
pub fn fn_00ee6e2e(e: &mut Engine, device: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6E2E, &args![device]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00be_0920, ni_d3d_shader_program_set_name(Ptr, u32)),
        entry!(0x00ee_6e2e, fn_00ee6e2e(u32) -> u32),
        entry!(
            0x00be_09e0,
            ni_d3d_shader_program_set_shader_program_name(Ptr, u32)
        ),
        entry!(0x00be_0aa0, fn_00be0aa0(Ptr) -> u32),
        entry!(0x00be_0ab0, ni_d3d_shader_program_set_code(Ptr, u32, u32)),
        entry!(0x00be_0ae0, ni_d3d_pixel_shader_get_rtti() -> u32),
        entry!(0x00be_0b60, ni_d3d_vertex_shader_get_rtti() -> u32),
        entry!(0x00be_0ba0, ni_d3d_vertex_shader_set_usage(Ptr, u32)),
        entry!(
            0x00be_0bd0,
            ni_d3d_vertex_shader_set_vertex_declaration(Ptr, u32)
        ),
        entry!(
            0x00be_0bf0,
            ni_d3d_vertex_shader_set_software_vertex_processing(Ptr, u8)
        ),
        entry!(
            0x00e9_5ba0,
            ni_d3d_shader_program_creator_hlsl_recreate_vertex_shader(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_5c40,
            ni_d3d_shader_program_creator_hlsl_recreate_pixel_shader(Ptr, Ptr) -> bool
        ),
        entry!(0x00e9_5cc0, ni_d3d_hlsl_pixel_shader_get_rtti() -> u32),
        entry!(
            0x00e9_5cd0,
            ni_d3d_hlsl_pixel_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_5cf0,
            ni_d3d_hlsl_pixel_shader_set_entry_point(Ptr, u32)
        ),
        entry!(
            0x00e9_5d60,
            ni_d3d_hlsl_pixel_shader_set_shader_target(Ptr, u32)
        ),
        entry!(
            0x00e9_5dc0,
            ni_d3d_hlsl_pixel_shader_set_constant_table(Ptr, u32)
        ),
        entry!(0x00e9_5df0, ni_d3d_hlsl_vertex_shader_get_rtti() -> u32),
        entry!(
            0x00e9_5e00,
            ni_d3d_hlsl_vertex_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_5e20,
            ni_d3d_hlsl_vertex_shader_set_entry_point(Ptr, u32)
        ),
        entry!(
            0x00e9_5e80,
            ni_d3d_hlsl_vertex_shader_set_shader_target(Ptr, u32)
        ),
        entry!(0x00e9_5ee0, fn_00e95ee0(Ptr) -> u32),
        entry!(
            0x00e9_5ef0,
            ni_d3d_hlsl_vertex_shader_set_constant_table(Ptr, u32)
        ),
        entry!(
            0x00e9_5f20,
            ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_d3dx_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00e9_6100,
            ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_d3dx_buffer(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00e9_62a0,
            ni_d3d_shader_program_creator_hlsl_load_shader_code_from_file(
                Ptr, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(
            0x00e9_64a0,
            ni_d3d_shader_program_creator_hlsl_load_shader_code_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(
            0x00e9_6610,
            ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_file(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00e9_67d0,
            ni_d3d_shader_program_creator_hlsl_create_vertex_shader_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00e9_6990,
            ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_file(
                Ptr, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00e9_6b10,
            ni_d3d_shader_program_creator_hlsl_create_pixel_shader_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x00e9_6c90, fn_00e96c90(Ptr)),
        entry!(0x00be_0ad0, fn_00be0ad0(Ptr) -> u32),
        entry!(
            0x00be_0af0,
            ni_d3d_pixel_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00be_0b10, fn_00be0b10(Ptr) -> u32),
        entry!(0x00be_0b20, ni_d3d_pixel_shader_set_shader_handle(Ptr, u32)),
        entry!(
            0x00be_0b70,
            ni_d3d_vertex_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00be_0b90, fn_00be0b90(Ptr) -> u32),
        entry!(
            0x00be_0bb0,
            ni_d3d_vertex_shader_set_shader_handle(Ptr, u32)
        ),
        entry!(0x00be_0bc0, fn_00be0bc0(Ptr) -> u32),
        entry!(
            0x00be_0be0,
            ni_d3d_vertex_shader_get_software_vertex_processing(Ptr) -> u8
        ),
        entry!(0x00cc_0760, fn_00cc0760(Ptr) -> u32),
        entry!(0x00e8_bf90, ni_d3d_hlsl_vertex_shader_get_entry_point(Ptr) -> u32),
        entry!(0x00e9_5d50, ni_d3d_hlsl_pixel_shader_get_shader_target(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x48);
        for offset in (0..0x48u32).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        this
    }

    #[test]
    fn getters_read_their_words() {
        let mut e = Engine::new();
        let this = object(&mut e);
        for (address, offset) in [
            (0x00be_0ad0u32, 0x1cu32),
            (0x00be_0b10, 0x2c),
            (0x00be_0b90, 0x30),
            (0x00be_0bc0, 0x38),
            (0x00cc_0760, 0xc),
            (0x00e8_bf90, 0x3c),
            (0x00e9_5d50, 0x34),
        ] {
            assert_eq!(e.call(address, &args![this]).u32(), 0x1000 + offset);
        }
        e.mem.set_u8(this + 0x2c, 1);
        assert_eq!(e.call(0x00be_0be0, &args![this]).u32() & 0xff, 1);
        e.mem.set_u8(this + 0x2c, 0);
        assert_eq!(e.call(0x00be_0be0, &args![this]).u32() & 0xff, 0);
    }

    #[test]
    fn set_shader_handle_stores_at_its_offset() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0b20, &args![this, 0xabcdu32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xabcd);
        assert_eq!(e.mem.u32(this + 0x34), 0x1034);
        e.call(0x00be_0bb0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x34), 0x1234);
    }

    fn deleting(address: u32, destruct: u32, size: u32) {
        let mut e = Engine::new();
        e.register(destruct, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let this = object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
        assert_eq!(e.call(address, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let destructs = log.iter().filter(|(x, _)| *x == destruct).count();
        assert_eq!(destructs, 2);
        let deletes: Vec<_> = log.iter().filter(|(x, _)| *x == NI_SIZED_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![this, size]);
    }

    #[test]
    fn pixel_shader_deleting_destructor_frees_on_bit_0() {
        deleting(0x00be_0af0, PIXEL_SHADER_DESTRUCT, 0x30);
    }

    #[test]
    fn vertex_shader_deleting_destructor_frees_on_bit_0() {
        deleting(0x00be_0b70, VERTEX_SHADER_DESTRUCT, 0x3c);
    }

    /// Fake code addresses of the shaders' virtual slots (slot offset added) and of the COM
    /// objects' slots.
    const SLOT_BASE: u32 = 0x0066_0000;
    const COM_BASE: u32 = 0x0067_0000;
    const RENDERER: u32 = 0x0abc_0000;
    const DEVICE: u32 = 0x0abc_0004;
    const VERTEX_PROFILE: u32 = 0x0abc_0010;
    const PIXEL_PROFILE: u32 = 0x0abc_0020;
    const HANDLE: u32 = 0x0bad_0001;

    /// Doubles for the allocators, string and memory copies, the error report, the program
    /// constructor, the profiles and the shader vtables' slots (the shader getters +0x64,
    /// +0x7c and +0x8c read +0x18, +0x30 and +0x38; every other slot does nothing).
    fn creator_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_BLOCK, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(COPY_STRING_00404D80, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            a[0].into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(REPORT_ERROR, |_, _| Ret::default());
        e.register(SHADER_PROGRAM_CONSTRUCT, |_, _| Ret::default());
        e.register(D3DX_GET_VERTEX_SHADER_PROFILE, |_, _| {
            VERTEX_PROFILE.into_ret()
        });
        e.register(D3DX_GET_PIXEL_SHADER_PROFILE, |_, _| {
            PIXEL_PROFILE.into_ret()
        });
        e.register(CREATE_D3D_VERTEX_SHADER, |_, _| HANDLE.into_ret());
        e.register(CREATE_D3D_PIXEL_SHADER, |_, _| HANDLE.into_ret());
        e.register(PIXEL_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(HLSL_VERTEX_SHADER_DESTRUCT, |_, _| Ret::default());
        e.map(0x010f_1000, 0x1000);
        // The pixel vtable's slots up to +0xa4 end before the vertex vtable starts.
        for slot in (0..0xa8u32).step_by(4) {
            e.mem
                .set_u32(HLSL_PIXEL_SHADER_VTABLE + slot, SLOT_BASE + slot);
        }
        for slot in (0..0xc0u32).step_by(4) {
            e.mem
                .set_u32(HLSL_VERTEX_SHADER_VTABLE + slot, SLOT_BASE + slot);
            e.register(SLOT_BASE + slot, |_, _| Ret::default());
        }
        e.register(SLOT_BASE + 0x64, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(SLOT_BASE + 0x7c, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(SLOT_BASE + 0x8c, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        for slot in [4u32, 8] {
            e.register(COM_BASE + slot, |_, _| Ret::default());
        }
        e.register(COM_BASE + 0xc, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(COM_BASE + 0x10, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.map(0x0126_f000, 0x1000);
        e.set_global(CREATOR_RENDERER, RENDERER);
        e.set_global(CREATOR_DEVICE, DEVICE);
        e.set_global(UNIVERSAL_CREATION_FLAGS, 0x100u32);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u8(EMPTY_STRING, 0);
        e
    }

    /// A COM object (`ID3DXBuffer`-like) whose slot +0xc answers `pointer` and slot +0x10
    /// answers `size`; +4 and +8 do nothing.
    fn com_object(e: &mut Engine, pointer: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x14);
        for slot in (0..0x14u32).step_by(4) {
            e.mem.set_u32(vtable + slot, COM_BASE + slot);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 8, pointer);
        e.mem.set_u32(object + 0xc, size);
        object
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s);
        block
    }

    /// The argument lists of the logged calls to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The argument word of the single logged call to the shader slot `slot`.
    fn slot_argument(e: &Engine, slot: u32) -> Vec<u32> {
        let found = calls(e, SLOT_BASE + slot);
        assert_eq!(found.len(), 1, "slot {slot:#x}");
        found[0][1..].to_vec()
    }

    /// A shader object on the HLSL vertex vtable with code, usage and declaration words.
    fn vertex_shader(e: &mut Engine, code: u32) -> u32 {
        let shader = e.mem.alloc(0x48);
        e.mem.set_u32(shader, HLSL_VERTEX_SHADER_VTABLE);
        e.mem.set_u32(shader + 0x18, code);
        e.mem.set_u32(shader + 0x30, 0x55);
        e.mem.set_u32(shader + 0x38, 0x77);
        shader
    }

    #[test]
    fn set_name_copies_reuses_or_frees() {
        let mut e = creator_engine();
        let program = e.mem.alloc(0x2c);
        let long = text(&mut e, b"long name");
        let short = text(&mut e, b"abc");
        e.call_log = Some(vec![]);
        e.call(0x00be_0920, &args![program, long]);
        let first = e.mem.u32(program + 0xc);
        assert_eq!(e.mem.cstr(first), b"long name");
        assert_eq!(calls(&e, NI_ALLOCATE_BLOCK), vec![vec![10]]);
        // A shorter name reuses the block.
        e.call(0x00be_0920, &args![program, short]);
        assert_eq!(e.mem.u32(program + 0xc), first);
        assert_eq!(e.mem.cstr(first), b"abc");
        // A longer one than the held text ("abc") frees it and allocates anew.
        e.call(0x00be_0920, &args![program, long]);
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![first]]);
        assert_eq!(calls(&e, NI_ALLOCATE_BLOCK).len(), 2);
        // Empty and null names free and clear.
        let held = e.mem.u32(program + 0xc);
        let empty = text(&mut e, b"");
        e.call(0x00be_0920, &args![program, empty]);
        assert_eq!(e.mem.u32(program + 0xc), 0);
        assert_eq!(calls(&e, NI_FREE_BLOCK).last().unwrap(), &vec![held]);
        e.call(0x00be_0920, &args![program, 0u32]);
        assert_eq!(calls(&e, NI_FREE_BLOCK).last().unwrap(), &vec![0]);
    }

    #[test]
    fn set_shader_program_name_uses_its_field() {
        let mut e = creator_engine();
        let program = e.mem.alloc(0x2c);
        let name = text(&mut e, b"file.hlsl");
        e.call(0x00be_09e0, &args![program, name]);
        let held = e.mem.u32(program + 0x10);
        assert_eq!(e.mem.cstr(held), b"file.hlsl");
        assert_eq!(e.mem.u32(program + 0xc), 0);
        e.call(0x00be_09e0, &args![program, 0u32]);
        assert_eq!(e.mem.u32(program + 0x10), 0);
    }

    #[test]
    fn code_getter_reads_0x18() {
        let mut e = Engine::new();
        let this = object(&mut e);
        assert_eq!(e.call(0x00be_0aa0, &args![this]).u32(), 0x1018);
    }

    #[test]
    fn set_code_stores_size_and_code() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0ab0, &args![this, 0x40u32, 0x9000u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0x40);
        assert_eq!(e.mem.u32(this + 0x18), 0x9000);
    }

    #[test]
    fn rtti_getters_answer_their_records() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00be_0ae0, &[]).u32(), 0x0126_f9ac);
        assert_eq!(e.call(0x00be_0b60, &[]).u32(), 0x0126_f9b4);
        assert_eq!(e.call(0x00e9_5cc0, &[]).u32(), 0x0126_fa40);
        assert_eq!(e.call(0x00e9_5df0, &[]).u32(), 0x0126_fa48);
    }

    #[test]
    fn set_usage_stores_at_0x30() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0ba0, &args![this, 7u32]);
        assert_eq!(e.mem.u32(this + 0x30), 7);
    }

    #[test]
    fn set_vertex_declaration_stores_at_0x38() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0bd0, &args![this, 0x4444u32]);
        assert_eq!(e.mem.u32(this + 0x38), 0x4444);
    }

    #[test]
    fn set_software_vertex_processing_stores_the_low_byte() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0bf0, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 0x2c), 1);
        assert_eq!(e.mem.u8(this + 0x2d), 0x10);
    }

    #[test]
    fn constant_table_getter_reads_0x44() {
        let mut e = Engine::new();
        let this = object(&mut e);
        assert_eq!(e.call(0x00e9_5ee0, &args![this]).u32(), 0x1044);
    }

    #[test]
    fn recreate_vertex_shader_sets_handle_and_table() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        e.register_double(D3DX_GET_SHADER_CONSTANT_TABLE, move |e, a| {
            e.mem.set_u32(a[1], table);
            Ret::default()
        });
        e.register(CREATE_D3D_VERTEX_SHADER, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x77);
            HANDLE.into_ret()
        });
        let creator = e.mem.alloc(8);
        let shader = vertex_shader(&mut e, 0x5000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_5ba0, &args![creator, shader]).bool());
        let create = &calls(&e, CREATE_D3D_VERTEX_SHADER)[0];
        assert_eq!(create[..2], [creator, 0x5000]);
        assert_eq!(create[3..], [0x55, 0, 0, 0]);
        assert_eq!(slot_argument(&e, 0x88), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0xb8), vec![table]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![table]]);
    }

    #[test]
    fn recreate_vertex_shader_without_code_or_handle_answers_0() {
        let mut e = creator_engine();
        let creator = e.mem.alloc(8);
        let shader = vertex_shader(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_5ba0, &args![creator, shader]).bool());
        assert!(calls(&e, CREATE_D3D_VERTEX_SHADER).is_empty());
        e.register(CREATE_D3D_VERTEX_SHADER, |_, _| Ret::default());
        e.mem.set_u32(shader + 0x18, 0x5000);
        assert!(!e.call(0x00e9_5ba0, &args![creator, shader]).bool());
        assert!(calls(&e, SLOT_BASE + 0x88).is_empty());
    }

    #[test]
    fn recreate_vertex_shader_failed_table_releases_the_slot_word() {
        let mut e = creator_engine();
        // The table call fails without writing: the slot still holds the shader, which the
        // exe then releases through the shader's own slot +8.
        e.register(D3DX_GET_SHADER_CONSTANT_TABLE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        let creator = e.mem.alloc(8);
        let shader = vertex_shader(&mut e, 0x5000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_5ba0, &args![creator, shader]).bool());
        assert!(calls(&e, SLOT_BASE + 0xb8).is_empty());
        assert_eq!(calls(&e, SLOT_BASE + 8), vec![vec![shader]]);
    }

    #[test]
    fn recreate_pixel_shader_sets_handle_and_table() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        e.register_double(D3DX_GET_SHADER_CONSTANT_TABLE, move |e, a| {
            e.mem.set_u32(a[1], table);
            Ret::default()
        });
        let creator = e.mem.alloc(8);
        let shader = vertex_shader(&mut e, 0x5000);
        e.mem.set_u32(shader, HLSL_PIXEL_SHADER_VTABLE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_5c40, &args![creator, shader]).bool());
        assert_eq!(
            calls(&e, CREATE_D3D_PIXEL_SHADER),
            vec![vec![creator, 0x5000]]
        );
        assert_eq!(slot_argument(&e, 0x80), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0xa0), vec![table]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![table]]);
        // No handle: 0, nothing set.
        e.register(CREATE_D3D_PIXEL_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_5c40, &args![creator, shader]).bool());
        assert!(calls(&e, SLOT_BASE + 0x80).is_empty());
        // No code: 0 before the creation.
        e.mem.set_u32(shader + 0x18, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_5c40, &args![creator, shader]).bool());
        assert!(calls(&e, CREATE_D3D_PIXEL_SHADER).is_empty());
    }

    #[test]
    fn hlsl_pixel_shader_deleting_destructor_runs_the_destructor() {
        let mut e = creator_engine();
        let shader = e.mem.alloc(0x3c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5cd0, &args![shader, 1u32]).u32(), shader);
        assert_eq!(e.mem.u32(shader), HLSL_PIXEL_SHADER_VTABLE);
        assert_eq!(calls(&e, PIXEL_SHADER_DESTRUCT), vec![vec![shader]]);
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![shader, 0x3c]]);
        e.call(0x00e9_5cd0, &args![shader, 0u32]);
        assert_eq!(calls(&e, NI_SIZED_DELETE).len(), 1);
    }

    #[test]
    fn hlsl_vertex_shader_deleting_destructor_frees_on_bit_0() {
        deleting(0x00e9_5e00, HLSL_VERTEX_SHADER_DESTRUCT, 0x48);
    }

    /// The entry-point setters: a copy, the same pointer kept, cleared for an empty string.
    fn entry_point_setter(address: u32, offset: u32) {
        let mut e = creator_engine();
        let shader = e.mem.alloc(0x48);
        let main = text(&mut e, b"main");
        e.call_log = Some(vec![]);
        e.call(address, &args![shader, main]);
        let held = e.mem.u32(shader + offset);
        assert_eq!(e.mem.cstr(held), b"main");
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![0]]);
        e.call(address, &args![shader, held]);
        assert_eq!(calls(&e, NI_FREE_BLOCK).len(), 1);
        let empty = text(&mut e, b"");
        e.call(address, &args![shader, empty]);
        assert_eq!(calls(&e, NI_FREE_BLOCK)[1], vec![held]);
        assert_eq!(e.mem.u32(shader + offset), 0);
    }

    /// The target setters: as the entry-point ones, but a null target leaves the freed
    /// pointer in the field.
    fn target_setter(address: u32, offset: u32) {
        let mut e = creator_engine();
        let shader = e.mem.alloc(0x48);
        let target = text(&mut e, b"ps_3_0");
        e.call_log = Some(vec![]);
        e.call(address, &args![shader, target]);
        let held = e.mem.u32(shader + offset);
        assert_eq!(e.mem.cstr(held), b"ps_3_0");
        e.call(address, &args![shader, 0u32]);
        assert_eq!(calls(&e, NI_FREE_BLOCK)[1], vec![held]);
        assert_eq!(e.mem.u32(shader + offset), held);
    }

    #[test]
    fn hlsl_pixel_shader_set_entry_point_copies_or_clears() {
        entry_point_setter(0x00e9_5cf0, 0x30);
    }

    #[test]
    fn hlsl_pixel_shader_set_shader_target_keeps_the_freed_pointer() {
        target_setter(0x00e9_5d60, 0x34);
    }

    #[test]
    fn hlsl_vertex_shader_set_entry_point_copies_or_clears() {
        entry_point_setter(0x00e9_5e20, 0x3c);
    }

    #[test]
    fn hlsl_vertex_shader_set_shader_target_keeps_the_freed_pointer() {
        target_setter(0x00e9_5e80, 0x40);
    }

    /// The table setters: release the old, reference the new, nothing for the same one.
    fn table_setter(address: u32, offset: u32) {
        let mut e = creator_engine();
        let shader = e.mem.alloc(0x48);
        let first = com_object(&mut e, 0, 0);
        let second = com_object(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(address, &args![shader, first]);
        assert_eq!(e.mem.u32(shader + offset), first);
        assert_eq!(calls(&e, COM_BASE + 4), vec![vec![first]]);
        assert!(calls(&e, COM_BASE + 8).is_empty());
        e.call(address, &args![shader, first]);
        assert_eq!(calls(&e, COM_BASE + 4).len(), 1);
        e.call(address, &args![shader, second]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![first]]);
        e.call(address, &args![shader, 0u32]);
        assert_eq!(calls(&e, COM_BASE + 8)[1], vec![second]);
        assert_eq!(e.mem.u32(shader + offset), 0);
        assert_eq!(calls(&e, COM_BASE + 4).len(), 2);
    }

    #[test]
    fn hlsl_pixel_shader_set_constant_table_replaces_it() {
        table_setter(0x00e9_5dc0, 0x38);
    }

    #[test]
    fn hlsl_vertex_shader_set_constant_table_replaces_it() {
        table_setter(0x00e9_5ef0, 0x44);
    }

    /// An engine whose constant-table query writes a table object and succeeds.
    fn engine_with_table() -> (Engine, u32) {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        e.register_double(D3DX_GET_SHADER_CONSTANT_TABLE, move |e, a| {
            e.mem.set_u32(a[1], table);
            Ret::default()
        });
        (e, table)
    }

    #[test]
    fn create_vertex_shader_from_d3dx_buffer_builds_the_shader() {
        let (mut e, table) = engine_with_table();
        let code = text(&mut e, b"CODE");
        let buffer = com_object(&mut e, code, 4);
        let creator = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let shader = e
            .call(
                0x00e9_5f20,
                &args![creator, buffer, 4u32, 0x700u32, 0u32, 0u32, 0x900u32, 3u32, 1u32, 1u32],
            )
            .u32();
        assert_ne!(shader, 0);
        assert_eq!(e.mem.u32(shader), HLSL_VERTEX_SHADER_VTABLE);
        assert_eq!(
            calls(&e, SHADER_PROGRAM_CONSTRUCT),
            vec![vec![shader, RENDERER, 0]]
        );
        let create = &calls(&e, CREATE_D3D_VERTEX_SHADER)[0];
        assert_eq!(create[..2], [creator, code]);
        assert_eq!(create[3..], [3, 0, 0, 1]);
        assert_eq!(slot_argument(&e, 0x54), vec![0x700]);
        assert_eq!(slot_argument(&e, 0x5c), vec![0]);
        let set_code = slot_argument(&e, 0x68);
        assert_eq!(set_code[0], 4);
        assert_eq!(e.mem.cstr(set_code[1]), b"CODE");
        assert_eq!(slot_argument(&e, 0x70), vec![creator]);
        assert_eq!(slot_argument(&e, 0x88), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0x80), vec![3]);
        assert_eq!(slot_argument(&e, 0x98), vec![1]);
        assert_eq!(slot_argument(&e, 0xa8), vec![DEFAULT_ENTRY_POINT]);
        assert_eq!(slot_argument(&e, 0xb0), vec![VERTEX_PROFILE]);
        assert_eq!(slot_argument(&e, 0xb8), vec![table]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![table]]);
    }

    #[test]
    fn create_vertex_shader_from_d3dx_buffer_failures() {
        let (mut e, _) = engine_with_table();
        let buffer = com_object(&mut e, 0x5000, 4);
        let creator = e.mem.alloc(8);
        let words =
            args![creator, buffer, 4u32, 0x700u32, 0x710u32, 0x720u32, 0u32, 0u32, 0u32, 1u32];
        // No handle: reported with the name, the shader deleted.
        e.register(CREATE_D3D_VERTEX_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5f20, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, FAILED_CREATE_VERTEX_SHADER, 0x700]]
        );
        let delete = slot_argument(&e, 0);
        assert_eq!(delete, vec![1]);
        assert!(calls(&e, D3DX_GET_VERTEX_SHADER_PROFILE).is_empty());
        // The table query fails: 0, nothing created.
        e.register(D3DX_GET_SHADER_CONSTANT_TABLE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5f20, &words).u32(), 0);
        assert!(calls(&e, CREATE_D3D_VERTEX_SHADER).is_empty());
        assert!(calls(&e, SLOT_BASE).is_empty());
        // No memory: 0 at once.
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5f20, &words).u32(), 0);
        assert!(calls(&e, SHADER_PROGRAM_CONSTRUCT).is_empty());
    }

    #[test]
    fn create_pixel_shader_from_d3dx_buffer_builds_the_shader() {
        let (mut e, table) = engine_with_table();
        let code = text(&mut e, b"PS");
        let buffer = com_object(&mut e, code, 2);
        let creator = e.mem.alloc(8);
        let words = args![creator, buffer, 2u32, 0x700u32, 0u32, 0u32, 0u32];
        e.call_log = Some(vec![]);
        let shader = e.call(0x00e9_6100, &words).u32();
        assert_eq!(e.mem.u32(shader), HLSL_PIXEL_SHADER_VTABLE);
        assert_eq!(
            calls(&e, SHADER_PROGRAM_CONSTRUCT),
            vec![vec![shader, RENDERER, 2]]
        );
        assert_eq!(
            calls(&e, CREATE_D3D_PIXEL_SHADER),
            vec![vec![creator, code]]
        );
        assert_eq!(slot_argument(&e, 0x54), vec![0x700]);
        assert_eq!(slot_argument(&e, 0x5c), vec![0]);
        assert_eq!(slot_argument(&e, 0x68)[0], 2);
        assert_eq!(slot_argument(&e, 0x70), vec![creator]);
        assert_eq!(slot_argument(&e, 0x80), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0x90), vec![DEFAULT_ENTRY_POINT]);
        assert_eq!(slot_argument(&e, 0x98), vec![PIXEL_PROFILE]);
        assert_eq!(slot_argument(&e, 0xa0), vec![table]);
        // No handle: reported, deleted, 0.
        e.register(CREATE_D3D_PIXEL_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6100, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, FAILED_CREATE_PIXEL_SHADER, 0x700]]
        );
        assert_eq!(slot_argument(&e, 0), vec![1]);
    }

    #[test]
    fn load_shader_code_from_file_copies_the_compiled_code() {
        let mut e = creator_engine();
        let compiled = text(&mut e, b"BYTECODE");
        let shader_buffer = com_object(&mut e, compiled, 8);
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 1u32.into_ret());
        e.register_double(D3DX_COMPILE_SHADER_FROM_FILE, move |e, a| {
            e.mem.set_u32(a[6], shader_buffer);
            e.mem.set_u32(a[7], 0);
            Ret::default()
        });
        let creator = e.mem.alloc(8);
        e.mem.set_u32(creator + 4, 0x11);
        let file = text(&mut e, b"a.hlsl");
        let outs = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let words = args![
            creator,
            file,
            0x710u32,
            0x720u32,
            outs,
            outs + 4,
            outs + 8,
            1u32
        ];
        assert!(e.call(0x00e9_62a0, &words).bool());
        let compile = &calls(&e, D3DX_COMPILE_SHADER_FROM_FILE)[0];
        assert_eq!(compile[1..6], [0, 0, 0x710, 0x720, 0x111]);
        assert_eq!(compile[8], outs + 8);
        let resolve = &calls(&e, RESOLVE_SHADER_FILE_NAME)[0];
        assert_eq!(
            (resolve[0], resolve[1], resolve[2]),
            (file, compile[0], 0x104)
        );
        assert_eq!(e.mem.u32(outs + 4), 8);
        assert_eq!(e.mem.cstr(e.mem.u32(outs)), b"BYTECODE");
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![shader_buffer]]);
    }

    #[test]
    fn load_shader_code_from_file_reports_failures() {
        let mut e = creator_engine();
        let creator = e.mem.alloc(8);
        let outs = e.mem.alloc(12);
        e.mem.set_u32(outs, 0x99);
        e.mem.set_u32(outs + 4, 0x99);
        let file = text(&mut e, b"a.hlsl");
        let words = |file: u32| {
            args![
                creator,
                file,
                0x710u32,
                0x720u32,
                outs,
                outs + 4,
                outs + 8,
                0u32
            ]
        };
        // No file name.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_62a0, &words(0)).bool());
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, INVALID_SHADER_FILE_NAME]]
        );
        // The file is not found.
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_62a0, &words(file)).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, FAILED_FIND_FILE, file]]
        );
        // The compile fails with messages: the copy is reported and freed.
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 1u32.into_ret());
        let message = text(&mut e, b"error X");
        let errors = com_object(&mut e, message, 8);
        e.register_double(D3DX_COMPILE_SHADER_FROM_FILE, move |e, a| {
            e.mem.set_u32(a[6], 0);
            e.mem.set_u32(a[7], errors);
            0x8000_4005u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_62a0, &words(file)).bool());
        let report = &calls(&e, REPORT_ERROR)[0];
        assert_eq!(
            report[..5],
            [1, 0, FAILED_COMPILE_FILE_MESSAGES, 0x710, file]
        );
        assert_eq!(e.mem.cstr(report[5]), b"error X");
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![report[5]]]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![errors]]);
        // Without messages.
        e.register(D3DX_COMPILE_SHADER_FROM_FILE, |e, a| {
            e.mem.set_u32(a[6], 0);
            e.mem.set_u32(a[7], 0);
            0x8000_4005u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_62a0, &words(file)).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, FAILED_COMPILE_FILE, 0x710, file]]
        );
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![0]]);
        assert!(calls(&e, COM_BASE + 8).is_empty());
    }

    #[test]
    fn load_shader_code_from_buffer_compiles_into_the_argument_slots() {
        let mut e = creator_engine();
        let compiled = text(&mut e, b"BC");
        let shader_buffer = com_object(&mut e, compiled, 2);
        let message = text(&mut e, b"warning");
        let errors = com_object(&mut e, message, 8);
        e.register_double(D3DX_COMPILE_SHADER, move |e, a| {
            e.mem.set_u32(a[7], shader_buffer);
            e.mem.set_u32(a[8], errors);
            Ret::default()
        });
        let creator = e.mem.alloc(8);
        let outs = e.mem.alloc(12);
        let words = |source: u32, size: u32| {
            args![
                creator,
                source,
                size,
                0x710u32,
                0x720u32,
                outs,
                outs + 4,
                outs + 8,
                1u32
            ]
        };
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_64a0, &words(0x6000, 0x40)).bool());
        let compile = &calls(&e, D3DX_COMPILE_SHADER)[0];
        assert_eq!(compile[..7], [0x6000, 0x40, 0, 0, 0x710, 0x720, 0x100]);
        assert_eq!(compile[9], outs + 8);
        assert_eq!(e.mem.u32(outs + 4), 2);
        assert_eq!(e.mem.cstr(e.mem.u32(outs)), b"BC");
        assert_eq!(
            calls(&e, COM_BASE + 8),
            vec![vec![shader_buffer], vec![errors]]
        );
        // An invalid buffer is reported.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_64a0, &words(0x6000, 0)).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, INVALID_SHADER_BUFFER]]
        );
        // A failure with messages reports the entry point and the copy.
        e.register_double(D3DX_COMPILE_SHADER, move |e, a| {
            e.mem.set_u32(a[7], 0);
            e.mem.set_u32(a[8], errors);
            0x8000_4005u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_64a0, &words(0x6000, 0x40)).bool());
        let report = &calls(&e, REPORT_ERROR)[0];
        assert_eq!(report[..4], [1, 1, FAILED_ASSEMBLE_BUFFER_MESSAGES, 0x710]);
        assert_eq!(e.mem.cstr(report[4]), b"warning");
        // Without messages.
        e.register(D3DX_COMPILE_SHADER, |e, a| {
            e.mem.set_u32(a[7], 0);
            e.mem.set_u32(a[8], 0);
            0x8000_4005u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_64a0, &words(0x6000, 0x40)).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, FAILED_ASSEMBLE_BUFFER, 0x710]]
        );
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![0]]);
        assert!(calls(&e, COM_BASE + 8).is_empty());
    }

    /// Doubles under the two loaders: the file resolves, and both compilers write a shader
    /// buffer holding 8 bytes of `"COMPILED"` and the table and succeed (`succeed` false:
    /// they fail without buffers).
    fn loads(e: &mut Engine, table: u32, succeed: bool) {
        let compiled = text(e, b"COMPILED");
        let shader_buffer = com_object(e, compiled, 8);
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 1u32.into_ret());
        for (compiler, first_out) in [
            (D3DX_COMPILE_SHADER_FROM_FILE, 6usize),
            (D3DX_COMPILE_SHADER, 7),
        ] {
            e.register_double(compiler, move |e, a| {
                if succeed {
                    e.mem.set_u32(a[first_out], shader_buffer);
                    e.mem.set_u32(a[first_out + 1], 0);
                    e.mem.set_u32(a[first_out + 2], table);
                    Ret::default()
                } else {
                    e.mem.set_u32(a[first_out], 0);
                    e.mem.set_u32(a[first_out + 1], 0);
                    0x8000_4005u32.into_ret()
                }
            });
        }
    }

    /// The code block the shader got (slot +0x68) holds the compiled bytes.
    fn set_code(e: &Engine) -> Vec<u32> {
        let set_code = slot_argument(e, 0x68);
        assert_eq!(set_code[0], 8);
        assert_eq!(e.mem.bytes(set_code[1], 8), b"COMPILED");
        set_code
    }

    #[test]
    fn create_vertex_shader_from_file_builds_the_shader() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        loads(&mut e, table, true);
        let creator = e.mem.alloc(8);
        e.register(CREATE_D3D_VERTEX_SHADER, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x900);
            HANDLE.into_ret()
        });
        let file = text(&mut e, b"a.vsh");
        let words = args![creator, file, 0x700u32, 0x710u32, 0x720u32, 0x900u32, 5u32, 1u32, 1u32];
        e.call_log = Some(vec![]);
        let shader = e.call(0x00e9_6610, &words).u32();
        assert_eq!(e.mem.u32(shader), HLSL_VERTEX_SHADER_VTABLE);
        let compile = &calls(&e, D3DX_COMPILE_SHADER_FROM_FILE)[0];
        assert_eq!(compile[3..5], [0x710, 0x720]);
        let code = set_code(&e)[1];
        let create = &calls(&e, CREATE_D3D_VERTEX_SHADER)[0];
        assert_eq!(create[..2], [creator, code]);
        assert_eq!(create[3..], [5, 0, 0, 1]);
        assert_eq!(slot_argument(&e, 0x54), vec![0x700]);
        assert_eq!(slot_argument(&e, 0x5c), vec![file]);
        assert_eq!(slot_argument(&e, 0x70), vec![creator]);
        assert_eq!(slot_argument(&e, 0x88), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0x80), vec![5]);
        assert_eq!(slot_argument(&e, 0x98), vec![1]);
        assert_eq!(slot_argument(&e, 0xa8), vec![0x710]);
        assert_eq!(slot_argument(&e, 0xb0), vec![0x720]);
        assert_eq!(slot_argument(&e, 0xb8), vec![table]);
        assert_eq!(calls(&e, COM_BASE + 8).last().unwrap(), &vec![table]);
        // No handle: reported with the name, deleted.
        e.register(CREATE_D3D_VERTEX_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6610, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, FAILED_CREATE_VERTEX_SHADER, 0x700]]
        );
        assert_eq!(slot_argument(&e, 0), vec![1]);
        // A failed load deletes the shader; the defaults went to the compiler.
        loads(&mut e, table, false);
        e.call_log = Some(vec![]);
        let defaults = args![creator, file, 0x700u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x00e9_6610, &defaults).u32(), 0);
        let compile = &calls(&e, D3DX_COMPILE_SHADER_FROM_FILE)[0];
        assert_eq!(compile[3..5], [DEFAULT_ENTRY_POINT, VERTEX_PROFILE]);
        assert_eq!(slot_argument(&e, 0), vec![1]);
        assert!(calls(&e, CREATE_D3D_VERTEX_SHADER).is_empty());
    }

    #[test]
    fn create_vertex_shader_from_buffer_builds_the_shader() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        loads(&mut e, table, true);
        let creator = e.mem.alloc(8);
        let words = args![
            creator, 0x6000u32, 0x40u32, 0x700u32, 0x710u32, 0u32, 0x900u32, 5u32, 0u32, 1u32
        ];
        e.call_log = Some(vec![]);
        let shader = e.call(0x00e9_67d0, &words).u32();
        assert_ne!(shader, 0);
        let compile = &calls(&e, D3DX_COMPILE_SHADER)[0];
        assert_eq!(compile[..6], [0x6000, 0x40, 0, 0, 0x710, VERTEX_PROFILE]);
        let code = set_code(&e)[1];
        let create = &calls(&e, CREATE_D3D_VERTEX_SHADER)[0];
        assert_eq!(create[..2], [creator, code]);
        assert_eq!(create[3..], [5, 0, 0, 0]);
        assert_eq!(slot_argument(&e, 0x5c), vec![0]);
        assert_eq!(slot_argument(&e, 0xb0), vec![VERTEX_PROFILE]);
        assert_eq!(slot_argument(&e, 0xb8), vec![table]);
        // No handle: reported with the name.
        e.register(CREATE_D3D_VERTEX_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_67d0, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, FAILED_CREATE_VERTEX_SHADER, 0x700]]
        );
        // A failed load deletes the shader.
        loads(&mut e, table, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_67d0, &words).u32(), 0);
        assert_eq!(slot_argument(&e, 0), vec![1]);
        assert!(calls(&e, CREATE_D3D_VERTEX_SHADER).is_empty());
    }

    #[test]
    fn create_pixel_shader_from_file_builds_the_shader() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        loads(&mut e, table, true);
        let creator = e.mem.alloc(8);
        let file = text(&mut e, b"a.psh");
        let words = args![creator, file, 0x700u32, 0u32, 0u32, 1u32];
        e.call_log = Some(vec![]);
        let shader = e.call(0x00e9_6990, &words).u32();
        assert_eq!(e.mem.u32(shader), HLSL_PIXEL_SHADER_VTABLE);
        let compile = &calls(&e, D3DX_COMPILE_SHADER_FROM_FILE)[0];
        assert_eq!(compile[3..5], [DEFAULT_ENTRY_POINT, PIXEL_PROFILE]);
        let code = set_code(&e)[1];
        assert_eq!(
            calls(&e, CREATE_D3D_PIXEL_SHADER),
            vec![vec![creator, code]]
        );
        assert_eq!(slot_argument(&e, 0x54), vec![0x700]);
        assert_eq!(slot_argument(&e, 0x5c), vec![file]);
        assert_eq!(slot_argument(&e, 0x70), vec![creator]);
        assert_eq!(slot_argument(&e, 0x80), vec![HANDLE]);
        assert_eq!(slot_argument(&e, 0x90), vec![DEFAULT_ENTRY_POINT]);
        assert_eq!(slot_argument(&e, 0x98), vec![PIXEL_PROFILE]);
        assert_eq!(slot_argument(&e, 0xa0), vec![table]);
        // No handle.
        e.register(CREATE_D3D_PIXEL_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6990, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, FAILED_CREATE_PIXEL_SHADER, 0x700]]
        );
        assert_eq!(slot_argument(&e, 0), vec![1]);
        // A failed load.
        loads(&mut e, table, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6990, &words).u32(), 0);
        assert!(calls(&e, CREATE_D3D_PIXEL_SHADER).is_empty());
        assert_eq!(slot_argument(&e, 0), vec![1]);
    }

    #[test]
    fn create_pixel_shader_from_buffer_builds_the_shader() {
        let mut e = creator_engine();
        let table = com_object(&mut e, 0, 0);
        loads(&mut e, table, true);
        let creator = e.mem.alloc(8);
        let words = args![creator, 0x6000u32, 0x40u32, 0x700u32, 0x710u32, 0x720u32, 0u32];
        e.call_log = Some(vec![]);
        let shader = e.call(0x00e9_6b10, &words).u32();
        assert_ne!(shader, 0);
        let compile = &calls(&e, D3DX_COMPILE_SHADER)[0];
        assert_eq!(compile[..6], [0x6000, 0x40, 0, 0, 0x710, 0x720]);
        let code = set_code(&e)[1];
        assert_eq!(
            calls(&e, CREATE_D3D_PIXEL_SHADER),
            vec![vec![creator, code]]
        );
        assert_eq!(slot_argument(&e, 0x54), vec![0x700]);
        assert_eq!(slot_argument(&e, 0x5c), vec![0]);
        assert_eq!(slot_argument(&e, 0x98), vec![0x720]);
        // No handle.
        e.register(CREATE_D3D_PIXEL_SHADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6b10, &words).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, FAILED_CREATE_PIXEL_SHADER, 0x700]]
        );
        // A failed load.
        loads(&mut e, table, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_6b10, &words).u32(), 0);
        assert_eq!(slot_argument(&e, 0), vec![1]);
    }

    #[test]
    fn hlsl_pixel_shader_destructor_frees_its_strings_and_table() {
        let mut e = creator_engine();
        let shader = e.mem.alloc(0x3c);
        let table = com_object(&mut e, 0, 0);
        e.mem.set_u32(shader + 0x30, 0x7100);
        e.mem.set_u32(shader + 0x34, 0x7200);
        e.mem.set_u32(shader + 0x38, table);
        e.call_log = Some(vec![]);
        e.call(0x00e9_6c90, &args![shader]);
        assert_eq!(e.mem.u32(shader), HLSL_PIXEL_SHADER_VTABLE);
        assert_eq!(calls(&e, NI_FREE_BLOCK), vec![vec![0x7100], vec![0x7200]]);
        assert_eq!(calls(&e, COM_BASE + 8), vec![vec![table]]);
        assert_eq!(e.mem.u32(shader + 0x38), 0);
        assert_eq!(calls(&e, PIXEL_SHADER_DESTRUCT), vec![vec![shader]]);
        // Without a table nothing is released.
        e.call_log = Some(vec![]);
        e.call(0x00e9_6c90, &args![shader]);
        assert!(calls(&e, COM_BASE + 8).is_empty());
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    #[test]
    fn fn_00ee6e2e_passes_the_device_and_the_answer() {
        let mut e = Engine::new();
        e.register(IMPORT_SLOT_00EE6E2E, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_6e2e, &args![0x7000u32]).u32(), 0x7001);
        let log = e.call_log.take().unwrap();
        let imports: Vec<_> = log.iter().filter(|c| c.0 == IMPORT_SLOT_00EE6E2E).collect();
        assert_eq!(imports, vec![&(IMPORT_SLOT_00EE6E2E, vec![0x7000])]);
    }
}
