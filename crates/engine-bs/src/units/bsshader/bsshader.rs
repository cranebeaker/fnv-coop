//! `BSShader/bsshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    construct_pointer_array, pass_pointer_clear, DESTRUCT_00B79930, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// CRT `_splitpath_s` (`00ec7f1e`), `strcat_s` (`00ec6bd2`), `_makepath_s` (`00ec825b`) and
/// `strcpy_s` (`00ec65a6`) (`cdecl`).
const SPLITPATH_S: u32 = 0x00ec_7f1e;
const STRCAT_S: u32 = 0x00ec_6bd2;
const MAKEPATH_S: u32 = 0x00ec_825b;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The shader root path `MakePath` splits (`011f9308`).
const SHADER_ROOT_011F9308: u32 = 0x011f_9308;

// Translated from 00be0c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::MakePath` (engine map), `cdecl` with two words: a shader source file name and
/// the 0x104-byte path buffer to fill. Nothing when the buffer is null. Otherwise splits the
/// file name (`_splitpath_s` into a drive (0x10), a folder (0x104), a name (0x80) and an
/// extension (8) on its stack), splits the shader root path at `011f9308` into the same drive
/// buffer and a second folder buffer, appends the file's folder to the root's (`strcat_s`) and
/// builds the path from the drive, the joined folder, the name and the extension
/// (`_makepath_s`, 0x104 bytes).
// Translation incomplete: 00be0c00 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bs_shader_make_path(e: &mut Engine, file: Ptr, path: Ptr) {
    if path.addr() == 0 {
        return;
    }
    e.with_stack(0x2a4, |e, frame| {
        let frame = frame.addr();
        let extension = frame;
        let drive = frame + 8;
        let name = frame + 0x18;
        let root_folder = frame + 0x98;
        let file_folder = frame + 0x19c;
        e.call(
            SPLITPATH_S,
            &args![
                file,
                drive,
                0x10u32,
                file_folder,
                0x104u32,
                name,
                0x80u32,
                extension,
                8u32
            ],
        );
        e.call(
            SPLITPATH_S,
            &args![
                SHADER_ROOT_011F9308,
                drive,
                0x10u32,
                root_folder,
                0x104u32,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        );
        e.call(STRCAT_S, &args![root_folder, 0x104u32, file_folder]);
        e.call(
            MAKEPATH_S,
            &args![path, 0x104u32, drive, root_folder, name, extension],
        );
    });
}

/// `NiD3DDefaultShader::NiD3DDefaultShader` (engine map, `00e7c6b0`, `thiscall`, no stack
/// words) and `BSShader`'s vtable (`010bc618`; no RTTI locator before it).
const DEFAULT_SHADER_CONSTRUCT_00E7C6B0: u32 = 0x00e7_c6b0;
const BS_SHADER_VTABLE_010BC618: u32 = 0x010b_c618;

// Translated from 00be2200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::BSShader` (engine map), `thiscall`, no stack words. Runs
/// `NiD3DDefaultShader::NiD3DDefaultShader`, stores the vtable `010bc618`, constructs the one
/// pass pointer at +0x5c (`eh vector constructor iterator` with `00b54620` and `00b79930`),
/// clears it (the pass's count at +0x64 dropped, deleted through `00e80190` at 0) and clears
/// the byte +0x64. Answers `this`.
// Translation incomplete: 00be2200 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the pass pointer) is not translated.
pub fn bs_shader_bs_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(DEFAULT_SHADER_CONSTRUCT_00E7C6B0, &args![base]);
    e.mem.set_u32(base, BS_SHADER_VTABLE_010BC618);
    construct_pointer_array(e, base + 0x5c, 1, DESTRUCT_00B79930);
    pass_pointer_clear(e, base + 0x5c);
    e.mem.set_u8(base + 0x64, 0);
    this
}

/// The compiled-file path maker `00be0dc0` (unit `bsshader.cpp`, unnamed; `cdecl`: buffer,
/// source path, compiled name, extension), the shader package's lookup `00b7fac0` (unnamed;
/// `thiscall` on the package at `011f9498`, the compiled name; answers an entry whose shader
/// bytes start at +0x104, or 0), the file time check `00afe7d0` (unnamed; `cdecl`: path,
/// 8-byte result, 0x20; answers a byte), `FileFinder::GetFile_ov2` (`00afdf20`, `cdecl`: path,
/// 0, 0x4000, 0x20), the file read `00462d80` (`thiscall` on the file: buffer, size), the
/// formatter `00406d00` (`cdecl`: buffer, size, format, words), `D3DXCompileShader`
/// (`00ee6e0a`, `stdcall`, ten words), the CRT's file open `00ec9a47` (`cdecl`: path, mode; the
/// engine map's name for it, `c_str`, is a folded one), `fwrite` (`00ecb086`), `fflush`
/// (`00ecda70`) and `_fclose` (`00ec9907`), `ArchiveManager::InvalidateFile` (`00af5e40`,
/// `cdecl`, the path), the package's shader maker `00b7fd90` (unnamed; `thiscall` on the
/// package at `011f9494`: name, bytes, size), the Ni allocation `00aa13e0` (`cdecl`, size),
/// `strncmp` (`00ec8a19`), the pixel profile chooser `00b4f380` (`cdecl`, a byte) and the
/// memory manager's free `00aa4060` (`thiscall` on `011f6238`, the block).
const COMPILED_PATH_00BE0DC0: u32 = 0x00be_0dc0;
const PACKAGE_FIND_00B7FAC0: u32 = 0x00b7_fac0;
const FILE_TIME_00AFE7D0: u32 = 0x00af_e7d0;
const GET_FILE: u32 = 0x00af_df20;
const FILE_READ_00462D80: u32 = 0x0046_2d80;
const FORMAT_00406D00: u32 = 0x0040_6d00;
const D3DX_COMPILE_SHADER: u32 = 0x00ee_6e0a;
const FILE_OPEN_00EC9A47: u32 = 0x00ec_9a47;
const FWRITE: u32 = 0x00ec_b086;
const FFLUSH: u32 = 0x00ec_da70;
const FCLOSE: u32 = 0x00ec_9907;
const INVALIDATE_FILE: u32 = 0x00af_5e40;
const PACKAGE_MAKE_SHADER_00B7FD90: u32 = 0x00b7_fd90;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const STRNCMP: u32 = 0x00ec_8a19;
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The settings and globals the compiles read: the shader package (`011f9498`), the byte that
/// turns on the source and cache time checks (`011ae638`), the byte that allows the
/// `"cache:\\"` retry (`011f94a7`), the cache root string (`011f9200`), the byte that adds the
/// `HDR` macro (`011f941e`), the package that makes shaders from bytes (`011f9494`) and the
/// renderer (`011f9508`; its Direct3D device at +0x288).
const SHADER_PACKAGE_011F9498: u32 = 0x011f_9498;
const CHECK_TIMES_011AE638: u32 = 0x011a_e638;
const CACHE_RETRY_011F94A7: u32 = 0x011f_94a7;
const CACHE_ROOT_011F9200: u32 = 0x011f_9200;
const HDR_MACRO_011F941E: u32 = 0x011f_941e;
const SHADER_MAKER_011F9494: u32 = 0x011f_9494;
const RENDERER_011F9508: u32 = 0x011f_9508;
/// Strings (read in Ghidra): `"vso"`, `"pso"`, `"cache:\\"`, `"HDR"`, `""`, `" /D"`, `"=1"`,
/// `"="`, `", "`, `"main"`, `"wb"`, `"ps_2_0"` (only its first four bytes compared), and the
/// messages.
const VSO_010BC868: u32 = 0x010b_c868;
const PSO_010BC8A0: u32 = 0x010b_c8a0;
const CACHE_ROOT_STRING_010BC860: u32 = 0x010b_c860;
const MACRO_HDR_010BC828: u32 = 0x010b_c828;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const DEFINE_010BC824: u32 = 0x010b_c824;
const EQUALS_ONE_010BC820: u32 = 0x010b_c820;
const EQUALS_01063EC8: u32 = 0x0106_3ec8;
const SEPARATOR_01012630: u32 = 0x0101_2630;
const ENTRY_MAIN_01094D90: u32 = 0x0109_4d90;
const MODE_WB_0106E798: u32 = 0x0106_e798;
const PS_2_0_010A93CC: u32 = 0x010a_93cc;
/// `"SHADER ERROR : error opening HLSL source file : %s"`.
const SOURCE_ERROR_010BC82C: u32 = 0x010b_c82c;
/// `"compiling : %s\n %s %s"` and `"%s\n %s\n %s"`.
const COMPILING_010BC808: u32 = 0x010b_c808;
const COMPILE_ERRORS_010BC7FC: u32 = 0x010b_c7fc;
/// `"invalid cache file : %s"`.
const INVALID_CACHE_010BC7E4: u32 = 0x010b_c7e4;
/// `"SHADER ERROR : error opening hlsl file : %s"`.
const HLSL_ERROR_010BC7B8: u32 = 0x010b_c7b8;
/// `"SHADER ERROR : failed to create vertex shader : %s"` and `".. pixel shader : %s"`.
const VERTEX_CREATE_ERROR_010BC784: u32 = 0x010b_c784;
const PIXEL_CREATE_ERROR_010BC86C: u32 = 0x010b_c86c;
/// `"SHADER ERROR : error opening cache file : %s"`.
const CACHE_ERROR_010BC754: u32 = 0x010b_c754;
/// The include handler's vtable (`010bc610`) the compile passes.
const INCLUDE_VTABLE_010BC610: u32 = 0x010b_c610;
/// `NiD3DVertexShader::NiD3DVertexShader` (`00be0b30`) and the pixel shader's constructor
/// `00be08f0` (unnamed) (`thiscall`, one word: the renderer).
const VERTEX_SHADER_CONSTRUCT_00BE0B30: u32 = 0x00be_0b30;
const PIXEL_SHADER_CONSTRUCT_00BE08F0: u32 = 0x00be_08f0;

/// What differs between `CompileVertexShaderHLSL` and `CompilePixelShaderHLSL`: the stack
/// frame (offsets of the buffers their callees see), the order of the time checks, the pixel
/// profile choice, the error macro list's loop, the cache write and the shader creation.
struct HlslCompile {
    frame: u32,
    extension: u32,
    cache_path: u32,
    cache_time: u32,
    source_time: u32,
    source_first: bool,
    source_error: u32,
    cache_error: u32,
    include: u32,
    macro_list: u32,
    command_line: u32,
    log: u32,
    constants: u32,
    handle: u32,
    pixel: bool,
    create_slot: u32,
    create_error: u32,
    object_size: u32,
    constructor: u32,
    object_slot: u32,
}

/// The vertex compile's frame (0x12ac bytes from its `ESP` after the saved registers).
const VERTEX_COMPILE: HlslCompile = HlslCompile {
    frame: 0x12ac,
    extension: VSO_010BC868,
    cache_path: 0x60,
    cache_time: 0x2c,
    source_time: 0x40,
    source_first: false,
    source_error: 0x164,
    cache_error: 0x77c,
    include: 0x58,
    macro_list: 0x268,
    command_line: 0x880,
    log: 0xd94,
    constants: 0x50,
    handle: 0x4c,
    pixel: false,
    create_slot: 0x16c,
    create_error: VERTEX_CREATE_ERROR_010BC784,
    object_size: 0x3c,
    constructor: VERTEX_SHADER_CONSTRUCT_00BE0B30,
    object_slot: 0x88,
};

/// The pixel compile's frame (0x12b0 bytes).
const PIXEL_COMPILE: HlslCompile = HlslCompile {
    frame: 0x12b0,
    extension: PSO_010BC8A0,
    cache_path: 0x64,
    cache_time: 0x30,
    source_time: 0x38,
    source_first: true,
    source_error: 0x67c,
    cache_error: 0x780,
    include: 0x5c,
    macro_list: 0x168,
    command_line: 0x884,
    log: 0xd98,
    constants: 0x58,
    handle: 0x54,
    pixel: true,
    create_slot: 0x1a8,
    create_error: PIXEL_CREATE_ERROR_010BC86C,
    object_size: 0x30,
    constructor: PIXEL_SHADER_CONSTRUCT_00BE08F0,
    object_slot: 0x80,
};

/// The compiled-code buffer and the error buffer the compile fills (`ID3DXBuffer`
/// pointers at +0x1c and +0x20 of both frames).
const COMPILED: u32 = 0x1c;
const ERRORS: u32 = 0x20;

/// `strcat_s(buffer, 0x514, text)`.
fn append(e: &mut Engine, buffer: u32, text: u32) {
    e.call(STRCAT_S, &args![buffer, 0x514u32, text]);
}

/// A non-empty macro value: not null and its first byte set.
fn has_value(e: &Engine, value: u32) -> bool {
    value != 0 && e.mem.u8(value) != 0
}

/// A file read whole (when its byte +0x2c is set): its size (vtable slot +0x28), a buffer of
/// that size from the memory manager, `00462d80` into it. Answers (buffer, size).
fn read_file(e: &mut Engine, file: u32) -> (u32, u32) {
    let size = e.vcall(file, 0x28, &args![]).u32();
    let buffer = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(FILE_READ_00462D80, &args![file, buffer, size]);
    (buffer, size)
}

/// `FileFinder::GetFile_ov2(path, 0, 0x4000, 0x20)`.
fn get_file(e: &mut Engine, path: u32) -> u32 {
    e.call(GET_FILE, &args![path, 0u32, 0x4000u32, 0x20u32])
        .u32()
}

/// A message formatted into one of the frame's 0x104-byte message buffers (the result 0).
fn report(e: &mut Engine, buffer: u32, format: u32, text: u32) -> u32 {
    e.call(FORMAT_00406D00, &args![buffer, 0x104u32, format, text]);
    0
}

/// `CompileVertexShaderHLSL` and `CompilePixelShaderHLSL`, as the exe runs them (see each).
fn compile_hlsl(
    e: &mut Engine,
    kind: &HlslCompile,
    path: u32,
    macros: u32,
    profile: u32,
    name: u32,
) -> u32 {
    e.with_stack(kind.frame, |e, frame| {
        let f = frame.addr();
        let cache_path = f + kind.cache_path;
        e.mem.set_u32(f + COMPILED, 0);
        e.mem.set_u32(f + ERRORS, 0);
        e.mem.set_u32(f + kind.constants, 0);
        e.call(
            COMPILED_PATH_00BE0DC0,
            &args![cache_path, path, name, kind.extension],
        );
        let mut use_cache = true;
        let mut cache_found = false;
        let mut packaged = 0u32;
        let package = e.global::<u32>(SHADER_PACKAGE_011F9498);
        if package != 0 {
            packaged = e.call(PACKAGE_FIND_00B7FAC0, &args![package, name]).u32();
        }
        if e.global::<u8>(CHECK_TIMES_011AE638) != 0 && packaged == 0 {
            for time in [kind.cache_time, kind.source_time] {
                e.mem.set_u32(f + time, 0);
                e.mem.set_u32(f + time + 4, 0);
            }
            let cache_time = f + kind.cache_time;
            let source_time = f + kind.source_time;
            let source_found;
            if kind.source_first {
                source_found = e
                    .call(FILE_TIME_00AFE7D0, &args![path, source_time, 0x20u32])
                    .u8()
                    != 0;
                cache_found = e
                    .call(FILE_TIME_00AFE7D0, &args![cache_path, cache_time, 0x20u32])
                    .u8()
                    != 0;
            } else {
                cache_found = e
                    .call(FILE_TIME_00AFE7D0, &args![cache_path, cache_time, 0x20u32])
                    .u8()
                    != 0;
                source_found = e
                    .call(FILE_TIME_00AFE7D0, &args![path, source_time, 0x20u32])
                    .u8()
                    != 0;
            }
            if !cache_found && e.global::<u8>(CACHE_RETRY_011F94A7) != 0 {
                // `CMP EDX,0x11f9200` with EDX 0 first: the string's address is never null,
                // so only its first byte decides.
                if e.global::<u8>(CACHE_ROOT_011F9200) == 0 {
                    e.call(
                        STRCPY_S,
                        &args![CACHE_ROOT_011F9200, 0x104u32, CACHE_ROOT_STRING_010BC860],
                    );
                    e.call(
                        COMPILED_PATH_00BE0DC0,
                        &args![cache_path, path, name, kind.extension],
                    );
                }
                cache_found = e
                    .call(FILE_TIME_00AFE7D0, &args![cache_path, cache_time, 0x20u32])
                    .u8()
                    != 0;
            }
            if !source_found {
                report(e, f + kind.source_error, SOURCE_ERROR_010BC82C, path);
            }
            if !cache_found {
                use_cache = false;
            }
        }
        let mut allocated = false;
        let mut data;
        let mut size = 0u32;
        if packaged != 0 {
            data = packaged.wrapping_add(0x104);
        } else if use_cache {
            let file = get_file(e, cache_path);
            if file == 0 {
                return report(e, f + kind.cache_error, CACHE_ERROR_010BC754, cache_path);
            }
            data = 0;
            if e.mem.u8(file + 0x2c) != 0 {
                (data, size) = read_file(e, file);
                allocated = true;
            }
            e.vcall(file, 0, &args![1u32]);
        } else {
            let mut appended = 0u32;
            if e.global::<u8>(HDR_MACRO_011F941E) != 0 {
                let mut slot = macros;
                while e.mem.u32(slot) != 0 {
                    slot += 8;
                }
                e.mem.set_u32(slot, MACRO_HDR_010BC828);
                e.mem.set_u32(slot + 4, EMPTY_STRING_01011584);
                appended = slot;
            }
            let mut compile_profile = profile;
            if kind.pixel
                && e.call(STRNCMP, &args![profile, PS_2_0_010A93CC, 4u32])
                    .u32()
                    == 0
            {
                compile_profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
                if compile_profile == 0 {
                    compile_profile = profile;
                }
            }
            let file = get_file(e, path);
            if file == 0 {
                return report(e, f + kind.source_error, HLSL_ERROR_010BC7B8, path);
            }
            if e.mem.u8(file + 0x2c) == 0 {
                e.vcall(file, 0, &args![1u32]);
                return report(e, f + kind.source_error, HLSL_ERROR_010BC7B8, path);
            }
            let (source, source_size) = read_file(e, file);
            let include = f + kind.include;
            e.mem.set_u32(include + 4, path);
            e.mem.set_u32(include, INCLUDE_VTABLE_010BC610);
            let list = f + kind.macro_list;
            let command = f + kind.command_line;
            e.mem.set_u8(list, 0);
            e.mem.set_u8(command, 0);
            for pair in 0..8u32 {
                let entry = macros + 8 * pair;
                let macro_name = e.mem.u32(entry);
                if macro_name == 0 {
                    continue;
                }
                append(e, list, macro_name);
                append(e, command, DEFINE_010BC824);
                let macro_name = e.mem.u32(entry);
                append(e, command, macro_name);
                if has_value(e, e.mem.u32(entry + 4)) {
                    append(e, list, EQUALS_01063EC8);
                    let value = e.mem.u32(entry + 4);
                    append(e, list, value);
                    append(e, command, EQUALS_01063EC8);
                    let value = e.mem.u32(entry + 4);
                    append(e, command, value);
                } else {
                    append(e, command, EQUALS_ONE_010BC820);
                }
                append(e, list, SEPARATOR_01012630);
            }
            e.call(
                FORMAT_00406D00,
                &args![
                    f + kind.log,
                    0x514u32,
                    COMPILING_010BC808,
                    path,
                    profile,
                    list
                ],
            );
            let result = e
                .call(
                    D3DX_COMPILE_SHADER,
                    &args![
                        source,
                        source_size,
                        macros,
                        include,
                        ENTRY_MAIN_01094D90,
                        compile_profile,
                        0u32,
                        f + COMPILED,
                        f + ERRORS,
                        f + kind.constants
                    ],
                )
                .u32();
            e.vcall(file, 0, &args![1u32]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, source]);
            let errors = e.mem.u32(f + ERRORS);
            if errors != 0 && e.mem.u32(f + COMPILED) == 0 {
                e.mem.set_u8(list, 0);
                for pair in 0..8u32 {
                    let entry = macros + 8 * pair;
                    let macro_name = e.mem.u32(entry);
                    if macro_name == 0 {
                        // The vertex loop stops at the first empty name, the pixel one goes on.
                        if kind.pixel {
                            continue;
                        }
                        break;
                    }
                    append(e, list, macro_name);
                    if has_value(e, e.mem.u32(entry + 4)) {
                        append(e, list, EQUALS_01063EC8);
                        let value = e.mem.u32(entry + 4);
                        append(e, list, value);
                    }
                    append(e, list, SEPARATOR_01012630);
                }
                let errors = e.mem.u32(f + ERRORS);
                let message = e.vcall(errors, 0xc, &args![]).u32();
                e.call(
                    FORMAT_00406D00,
                    &args![
                        f + kind.log,
                        0x514u32,
                        COMPILE_ERRORS_010BC7FC,
                        path,
                        list,
                        message
                    ],
                );
                let errors = e.mem.u32(f + ERRORS);
                e.vcall(errors, 8, &args![]);
                e.mem.set_u32(f + ERRORS, 0);
            }
            let compiled = e.mem.u32(f + COMPILED);
            if result == 0 && compiled != 0 {
                let output = e
                    .call(FILE_OPEN_00EC9A47, &args![cache_path, MODE_WB_0106E798])
                    .u32();
                if output != 0 || kind.pixel {
                    let compiled = e.mem.u32(f + COMPILED);
                    size = e.vcall(compiled, 0x10, &args![]).u32();
                    let compiled = e.mem.u32(f + COMPILED);
                    let bytes = e.vcall(compiled, 0xc, &args![]).u32();
                    e.call(FWRITE, &args![bytes, size, 1u32, output]);
                    if kind.pixel {
                        e.call(FFLUSH, &args![output]);
                    }
                    e.call(FCLOSE, &args![output]);
                    e.call(INVALIDATE_FILE, &args![cache_path]);
                } else {
                    e.call(
                        FORMAT_00406D00,
                        &args![
                            f + kind.source_error,
                            0xffu32,
                            INVALID_CACHE_010BC7E4,
                            cache_path
                        ],
                    );
                }
                let compiled = e.mem.u32(f + COMPILED);
                data = e.vcall(compiled, 0xc, &args![]).u32();
            } else {
                if !cache_found {
                    return 0;
                }
                let file = get_file(e, cache_path);
                if file == 0 {
                    return 0;
                }
                data = 0;
                if e.mem.u8(file + 0x2c) != 0 {
                    (data, size) = read_file(e, file);
                    allocated = true;
                }
                e.vcall(file, 0, &args![1u32]);
                if data == 0 {
                    return 0;
                }
            }
            if appended != 0 {
                e.mem.set_u32(appended, 0);
                e.mem.set_u32(appended + 4, 0);
            }
        }
        let mut made = 0u32;
        if data != 0 {
            let maker = e.global::<u32>(SHADER_MAKER_011F9494);
            if maker != 0 {
                e.call(
                    PACKAGE_MAKE_SHADER_00B7FD90,
                    &args![maker, name, data, size],
                );
            } else {
                let renderer = e.global::<u32>(RENDERER_011F9508);
                let device = e.mem.u32(renderer.wrapping_add(0x288));
                let handle = f + kind.handle;
                let failed = e
                    .vcall(device, kind.create_slot, &args![data, handle])
                    .u32();
                if failed != 0 {
                    return report(e, f + kind.cache_error, kind.create_error, cache_path);
                }
                let block = e.call(NI_ALLOCATE, &args![kind.object_size]).u32();
                made = if block != 0 {
                    let renderer = e.global::<u32>(RENDERER_011F9508);
                    e.call(kind.constructor, &args![block, renderer]).u32()
                } else {
                    0
                };
                let handle = e.mem.u32(handle);
                e.vcall(made, kind.object_slot, &args![handle]);
            }
        }
        if allocated {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, data]);
        }
        made
    })
}

// Translated from 00be0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::CompileVertexShaderHLSL` (engine map), `thiscall` with four stack words (`RET
/// 0x10`; `this` is not read): the HLSL source path, the macro pairs (name, value; eight are
/// walked), the profile and the compiled shader's name. Answers the vertex shader object, or
/// 0. The compiled file's path (`00be0dc0` with `"vso"`) is on the stack. A shader package
/// (`011f9498`) entry for the name (`00b7fac0`) supplies the bytes (at its +0x104) directly.
/// Otherwise, with the time checks on (`011ae638`): the cache's time (`00afe7d0`), then the
/// source's; a missing cache is looked up again under `"cache:\\"` (`011f9200`, set once
/// with `strcpy_s` and the path rebuilt) when `011f94a7` is set; a missing source formats
/// "error opening HLSL source file"; a missing cache means compiling, a found one (or the
/// checks off) loading the cache file. Compiling: the `HDR` macro is appended after the last
/// pair when `011f941e` is set; the source file is read (`GetFile_ov2`, size slot +0x28,
/// memory manager buffer, `00462d80`), the macro list and the `/D` command line are built
/// (`strcat_s`), "compiling : .." formatted, `D3DXCompileShader` run with an include handler
/// (`010bc610`, the path) and entry `"main"`, the file deleted (slot 0 with 1) and the buffer
/// freed. Errors without code: the macro list built again (up to the first empty name) and
/// formatted with the error text (slot +0xc), the error buffer released (slot +8). Success:
/// the code is written to the cache file (`"wb"`, `fwrite`, `_fclose`,
/// `ArchiveManager::InvalidateFile`; "invalid cache file" when it cannot be opened) and the
/// code buffer's bytes used; failure falls back to the cache file when it was found. The
/// appended macro is cleared again. The bytes then go to the package's maker (`00b7fd90` on
/// `011f9494`) when there is one, else to the device's `CreateVertexShader` (slot +0x16c;
/// failure formats "failed to create vertex shader" and answers 0), and a new
/// `NiD3DVertexShader` (0x3c bytes, `00be0b30` with the renderer) takes the handle (its slot
/// +0x88). A buffer read from the cache is freed at the end. Messages go to stack buffers
/// through `00406d00`.
// Translation incomplete: 00be0fe0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the shader allocation) and the /GS stack cookie check (00ec408c) are not translated.
pub fn bs_shader_compile_vertex_shader_hlsl(
    e: &mut Engine,
    _unused_this: Ptr,
    path: Ptr,
    macros: Ptr,
    profile: Ptr,
    name: Ptr,
) -> Ptr {
    let made = compile_hlsl(
        e,
        &VERTEX_COMPILE,
        path.addr(),
        macros.addr(),
        profile.addr(),
        name.addr(),
    );
    Ptr::new(made)
}

// Translated from 00be1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::CompilePixelShaderHLSL` (engine map), `thiscall` with four stack words (`RET
/// 0x10`; `this` is not read): path, macro pairs, profile, compiled name; answers the pixel
/// shader object, or 0. The same steps as `CompileVertexShaderHLSL` (`00be0fe0`) with these
/// differences: the compiled file's extension is `"pso"`; the time checks look at the source
/// first, then the cache; a profile starting with `"ps_2"` (`strncmp` with `"ps_2_0"`, 4) is
/// replaced by `00b4f380(1)` when that answers one (the log still shows the profile given);
/// the error macro list skips empty names instead of stopping; the cache file is written
/// without checking that it opened, with an `fflush` before `_fclose`; and the shader comes
/// from the device's `CreatePixelShader` (slot +0x1a8, "failed to create pixel shader"), a
/// 0x30-byte object built by `00be08f0` with the renderer taking the handle through its slot
/// +0x80. The stack buffers sit at the pixel frame's own offsets.
// Translation incomplete: 00be1750 - the C++ exception frame (FS:[0] chain, unwind state 0 for the shader allocation) and the /GS stack cookie check (00ec408c) are not translated.
pub fn bs_shader_compile_pixel_shader_hlsl(
    e: &mut Engine,
    _unused_this: Ptr,
    path: Ptr,
    macros: Ptr,
    profile: Ptr,
    name: Ptr,
) -> Ptr {
    let made = compile_hlsl(
        e,
        &PIXEL_COMPILE,
        path.addr(),
        macros.addr(),
        profile.addr(),
        name.addr(),
    );
    Ptr::new(made)
}

/// `NiD3DShaderProgram::NiD3DShaderProgram` (engine map, `00e897d0`, `thiscall`, two stack
/// words: the renderer and a program type) and `NiD3DVertexShader`'s vtable (`010ef87c`, RTTI
/// `.?AVNiD3DVertexShader@@`).
const SHADER_PROGRAM_CONSTRUCT_00E897D0: u32 = 0x00e8_97d0;
const VERTEX_SHADER_VTABLE_010EF87C: u32 = 0x010e_f87c;

// Translated from 00be0b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::NiD3DVertexShader` (engine map), `thiscall` with one stack word (the
/// renderer). Runs `NiD3DShaderProgram::NiD3DShaderProgram` with the renderer and 0, clears the
/// byte +0x2c and the words +0x30 to +0x38, stores the vtable `010ef87c` and answers `this`.
pub fn ni_d3d_vertex_shader_ni_d3d_vertex_shader(e: &mut Engine, this: Ptr, renderer: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_PROGRAM_CONSTRUCT_00E897D0,
        &args![base, renderer, 0u32],
    );
    e.mem.set_u8(base + 0x2c, 0);
    for offset in [0x30u32, 0x34, 0x38] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base, VERTEX_SHADER_VTABLE_010EF87C);
    this
}

/// The folder appended for the HDR shaders (the string at `010bc608`).
const HDR_FOLDER_010BC608: u32 = 0x010b_c608;

// Translated from 00be0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshader.cpp`): the compiled shader file's path, `cdecl`
/// with four words: the 0x104-byte buffer, the source path (only checked for null), the
/// compiled name and a fourth word the code does not read. Nothing when the buffer, the source
/// path or the name is null. Otherwise copies the cache root (`011f9200` when its first byte is
/// set, else the shader root `011f9308`) into the buffer (`strcpy_s`), appends the HDR folder
/// (`010bc608`) when `011f941e` is set and then the name (`strcat_s`).
pub fn fn_00be0dc0(e: &mut Engine, buffer: Ptr, source: Ptr, name: Ptr, _unused_3: u32) {
    let buffer = buffer.addr();
    if buffer == 0 || source.addr() == 0 {
        return;
    }
    if name.addr() == 0 {
        return;
    }
    let root = if e.global::<u8>(CACHE_ROOT_011F9200) != 0 {
        CACHE_ROOT_011F9200
    } else {
        SHADER_ROOT_011F9308
    };
    e.call(STRCPY_S, &args![buffer, 0x104u32, root]);
    if e.global::<u8>(HDR_MACRO_011F941E) != 0 {
        e.call(STRCAT_S, &args![buffer, 0x104u32, HDR_FOLDER_010BC608]);
    }
    e.call(STRCAT_S, &args![buffer, 0x104u32, name]);
}

/// The texture stage's state setters `00e88fc0` (`thiscall` on the stage's state block at
/// +0x10: state, value, a third word) and `00e890c0` (same block: sampler state, value, two more
/// words), the stage's `00e7de90` (`thiscall`, one word) and
/// `NiD3DTextureStage::SetFilterMode` (`00e7def0`, `thiscall`, one word) (unnamed but the last).
const STAGE_STATE_00E88FC0: u32 = 0x00e8_8fc0;
const SAMPLER_STATE_00E890C0: u32 = 0x00e8_90c0;
const STAGE_MODE_00E7DE90: u32 = 0x00e7_de90;
const SET_FILTER_MODE: u32 = 0x00e7_def0;

// Translated from 00be0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::SetupDefaultStage` (engine map), `cdecl` with five words: the texture stage, its
/// index, a word that picks the stage's mode, the sampler value and a filter flag (low byte).
/// Stores the index at +4 and sets the stage states (0xb: the index, 1: 1, 2: 2, 4: 1, 5: 2),
/// clears the byte +0x5e, sets state 0x18 to 0, the mode 0 when the third word is 3 and 3
/// otherwise (`00e7de90`), the sampler states 5, 6 and 7 to the sampler value, and the filter
/// mode 6 with the flag, 2 without.
pub fn bs_shader_setup_default_stage(
    e: &mut Engine,
    stage: Ptr,
    index: u32,
    mode: u32,
    sampler: u32,
    filter: bool,
) {
    let stage = stage.addr();
    let block = e.mem.u32(stage + 0x10);
    e.mem.set_u32(stage + 4, index);
    e.call(STAGE_STATE_00E88FC0, &args![block, 0xbu32, index, 0u32]);
    for (state, value) in [(1u32, 1u32), (2, 2), (4, 1), (5, 2)] {
        let block = e.mem.u32(stage + 0x10);
        e.call(STAGE_STATE_00E88FC0, &args![block, state, value, 0u32]);
    }
    let block = e.mem.u32(stage + 0x10);
    e.mem.set_u8(stage + 0x5e, 0);
    e.call(STAGE_STATE_00E88FC0, &args![block, 0x18u32, 0u32, 0u32]);
    let stage_mode = if mode == 3 { 0u32 } else { 3 };
    e.call(STAGE_MODE_00E7DE90, &args![stage, stage_mode]);
    for state in [5u32, 6, 7] {
        let block = e.mem.u32(stage + 0x10);
        e.call(
            SAMPLER_STATE_00E890C0,
            &args![block, state, sampler, 0u32, 0u32],
        );
    }
    let filter_mode = u32::from(filter) * 4 + 2;
    e.call(SET_FILTER_MODE, &args![stage, filter_mode]);
}

/// The pass type being rendered (`011f91e4`, as `bsbatchrenderer.cpp`'s code reads it) and the
/// render-state setters of `bsrenderstate_xenon.cpp` (`cdecl`, two words each): `00b97fa0`,
/// `BSRenderState::SetZWriteEnable` (`00b97e30`, engine map) and `00b97e80`.
const RENDERING_PASS_TYPE_011F91E4: u32 = 0x011f_91e4;
const RENDER_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const SET_Z_WRITE_ENABLE: u32 = 0x00b9_7e30;
const RENDER_STATE_00B97E80: u32 = 0x00b9_7e80;

// Translated from 00be0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable method (`thiscall`,
/// `this` not read) with one stack word it does not read (`RET 4`). Unless the pass type being
/// rendered (`011f91e4`) is 0x252 to 0x256: the render states `00b97fa0(1, 0)`,
/// `SetZWriteEnable(0, 0)` and `00b97e80(2, 0)`.
pub fn fn_00be0e50(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
    if (0x252..=0x256).contains(&pass_type) {
        return;
    }
    e.call(RENDER_STATE_00B97FA0, &args![1u32, 0u32]);
    e.call(SET_Z_WRITE_ENABLE, &args![0u32, 0u32]);
    e.call(RENDER_STATE_00B97E80, &args![2u32, 0u32]);
}

// Translated from 00be0e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), no arguments: the render
/// states `00b97fa0(0, 0)`, `SetZWriteEnable(1, 0)` and `00b97e80(3, 0)`.
pub fn fn_00be0e90(e: &mut Engine) {
    e.call(RENDER_STATE_00B97FA0, &args![0u32, 0u32]);
    e.call(SET_Z_WRITE_ENABLE, &args![1u32, 0u32]);
    e.call(RENDER_STATE_00B97E80, &args![3u32, 0u32]);
}

/// The pass being set up (`0126f74c`, the word the pass makers leave it in) and the two
/// per-item calls the list walks below make (unnamed, `cdecl`): `00b97d90` (the item's two
/// words, 0) and `00b98610` (the item's first word, 0).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const ITEM_STATE_00B97D90: u32 = 0x00b9_7d90;
const ITEM_STATE_00B98610: u32 = 0x00b9_8610;

/// The list the two walks go through: the pass's +0x30, its +0x10 the first node; each node is
/// two words then the next node at +8.
fn pass_list_head(e: &Engine) -> u32 {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let states = e.mem.u32(pass.wrapping_add(0x30));
    e.mem.u32(states.wrapping_add(0x10))
}

// Translated from 00be0eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable method (`thiscall`,
/// `this` not read) with one stack word it does not read (`RET 4`). For every node of the
/// current pass's list (`0126f74c`, +0x30, +0x10): `00b97d90(first word, second word, 0)`.
pub fn fn_00be0eb0(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let mut node = pass_list_head(e);
    while node != 0 {
        let second = e.mem.u32(node + 4);
        let first = e.mem.u32(node);
        e.call(ITEM_STATE_00B97D90, &args![first, second, 0u32]);
        node = e.mem.u32(node + 8);
    }
}

// Translated from 00be0ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable method (`thiscall`,
/// `this` not read) with one stack word it does not read (`RET 4`). For every node of the
/// current pass's list: `00b98610(first word, 0)`.
pub fn fn_00be0ee0(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let mut node = pass_list_head(e);
    while node != 0 {
        let first = e.mem.u32(node);
        e.call(ITEM_STATE_00B98610, &args![first, 0u32]);
        node = e.mem.u32(node + 8);
    }
}

/// The renderer method `00e75340` (`thiscall` on the renderer `011f9508`, one word: the shader;
/// unit `nixenonrenderer.cpp`), the `eh vector destructor iterator` (`00ec5fce`, `stdcall`:
/// array, element size, count, destructor) and the base destructor `00e7c540` (unnamed,
/// `thiscall`).
const RENDERER_METHOD_00E75340: u32 = 0x00e7_5340;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const BASE_DESTRUCT_00E7C540: u32 = 0x00e7_c540;

// Translated from 00be0f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShader::~BSShader` (engine map), `thiscall`, no stack words. Stores `BSShader`'s vtable
/// `010bc618`, drops the pass at +0x5c (its count at +0x64, deleted through `00e80190` at 0)
/// and clears it, gives the shader to the renderer's `00e75340`, destroys the one-element
/// pointer array at +0x5c (`00b79930`) and runs the base destructor `00e7c540`.
// Translation incomplete: 00be0f10 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the pointer array and the base) is not translated.
pub fn bs_shader_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BS_SHADER_VTABLE_010BC618);
    pass_pointer_clear(e, base + 0x5c);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.call(RENDERER_METHOD_00E75340, &args![renderer, base]);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x5c, 4u32, 1u32, DESTRUCT_00B79930],
    );
    e.call(BASE_DESTRUCT_00E7C540, &args![base]);
}

/// The pass's getter `00e7f7c0` (unnamed, `thiscall`, no stack words).
const PASS_GETTER_00E7F7C0: u32 = 0x00e7_f7c0;

// Translated from 00be1f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable slot (`thiscall`, `RET 4`):
/// neither `this` nor its one word is read. Takes the object `00e7f7c0` answers for the current pass (`0126f74c`); hands
/// its vtable slot +0x8c what the pass's +0x5c answers from slot +0x84 (with 0); then, when the
/// pass's +0x44 is set, its slot +0x7c with what that answers from slot +0x7c (with 0). The
/// getters take no stack word (`00e95d50`, plain `RET`): each pushed 0 stays on the stack as the
/// object's second word (`00e90e00`/`00e88870`, `RET 8`). Each
/// slot of the object is read before the call that feeds it, as in the exe.
pub fn fn_00be1f90(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let object = e.call(PASS_GETTER_00E7F7C0, &args![pass]).u32();
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let source = e.mem.u32(pass + 0x5c);
    let vtable = e.mem.u32(object);
    let value = e.vcall(source, 0x84, &args![]).u32();
    let target = e.mem.u32(vtable + 0x8c);
    e.call(target, &args![object, value, 0u32]);
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let source = e.mem.u32(pass + 0x44);
    if source != 0 {
        let vtable = e.mem.u32(object);
        let value = e.vcall(source, 0x7c, &args![]).u32();
        let target = e.mem.u32(vtable + 0x7c);
        e.call(target, &args![object, value, 0u32]);
    }
}

/// The render-state setters `00b97ff0` (`cdecl`, three words), `00b97f20` (three words) and
/// `00b97ed0` (two words) of `bsrenderstate_xenon.cpp`, and `00b984d0` (`bsbatchrenderer.cpp`,
/// two words) (unnamed).
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const RENDER_STATE_00B97F20: u32 = 0x00b9_7f20;
const RENDER_STATE_00B97ED0: u32 = 0x00b9_7ed0;
const BATCH_STATE_00B984D0: u32 = 0x00b9_84d0;

// Translated from 00be1ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshader.cpp`), a vtable slot (`thiscall`, `RET 4`; `this`
/// is not read) with one word: a block whose
/// first word is a property with flags at +0x18 and whose +0xc is an object with a register
/// word at +0x1c and two floats at +0x28 and +0x2c (the object counts as null when its +0x1c is
/// -1). Unless both floats are at least 1.0 (or unordered) and bit 0 of the flags is clear:
/// `00b97fa0(1, 0)` and then `00b97ff0` with flag bits 1-4 and 5-8 (with bit 0 set) or 6 and 7
/// (with it clear); else `00b97fa0(0, 0)`. A null object's floats are read at its null address,
/// as in the exe. The byte the exe stores from flag bit 9 into its own argument slot is never
/// read.
pub fn fn_00be1ff0(e: &mut Engine, _this: Ptr, block: Ptr) {
    let block = block.addr();
    let property = e.mem.u32(block);
    let object = e.mem.u32(block + 0xc);
    let object = if object == 0 || e.mem.u32(object + 0x1c) == 0xffff_ffff {
        0
    } else {
        object
    };
    let mut flagged = false;
    if property != 0 && e.mem.u8(property + 0x18) & 1 != 0 {
        flagged = true;
    }
    let first = e.mem.f32(object.wrapping_add(0x28));
    let mut plain = !(1.0f32 > first);
    if plain {
        let second = e.mem.f32(object.wrapping_add(0x2c));
        plain = !(1.0f32 > second) && !flagged;
    }
    if plain {
        e.call(RENDER_STATE_00B97FA0, &args![0u32, 0u32]);
        return;
    }
    e.call(RENDER_STATE_00B97FA0, &args![1u32, 0u32]);
    if flagged {
        let flags = u32::from(e.mem.u16(property + 0x18));
        e.call(
            RENDER_STATE_00B97FF0,
            &args![(flags >> 1) & 0xf, (flags >> 5) & 0xf, 0u32],
        );
    } else {
        e.call(RENDER_STATE_00B97FF0, &args![6u32, 7u32, 0u32]);
    }
}

// Translated from 00be20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable slot (`thiscall`, `RET 4`;
/// `this` is not read) with one word: a block whose first word is a property. `00b97f20` with bits 10-12 of the property's flags
/// (+0x18), its byte +0x1a and 0.
pub fn fn_00be20b0(e: &mut Engine, _this: Ptr, block: Ptr) {
    let property = e.mem.u32(block.addr());
    let flags = u32::from(e.mem.u16(property + 0x18));
    let byte = u32::from(e.mem.u8(property + 0x1a));
    e.call(RENDER_STATE_00B97F20, &args![(flags >> 10) & 7, byte, 0u32]);
}

// Translated from 00be20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (unit `bsshader.cpp` by address), a vtable slot (`thiscall`, `RET 8`;
/// `this` is not read) with two words: a block (first word a property, +0x10 another) and a flag (low byte). `00b984d0(3, 0)` when
/// the second property's flags (+0x18) have both bits 10 and 11 or the pass type being rendered
/// (`011f91e4`) is 0x56 or 0x57, else `00b984d0(1, 0)`. Then `00b97ed0` with the block's argument
/// word whose low byte the exe has replaced (0, or with the flag and a first property, bit 9 of
/// that property's flags) and 0: the upper three bytes stay those of the block address.
pub fn fn_00be20e0(e: &mut Engine, _this: Ptr, block: Ptr, flag: bool) {
    let block = block.addr();
    let mut set = false;
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
    if block != 0 {
        let second = e.mem.u32(block + 0x10);
        if second != 0 && e.mem.u16(second + 0x18) & 0xc00 == 0xc00 {
            set = true;
        }
    }
    if pass_type.wrapping_sub(0x56) <= 1 {
        set = true;
    }
    let mut low = 0u32;
    if flag && block != 0 {
        let first = e.mem.u32(block);
        if first != 0 {
            low = u32::from(e.mem.u16(first + 0x18) >> 9) & 1;
        }
    }
    e.call(BATCH_STATE_00B984D0, &args![u32::from(set) * 2 + 1, 0u32]);
    e.call(
        RENDER_STATE_00B97ED0,
        &args![(block & 0xffff_ff00) | low, 0u32],
    );
}

/// The pass's `00e7eb00` (unnamed, `thiscall`, one word) and the bit table `00be21b0` tests
/// against the pass's byte +0x4c (`011ae640`, one word per element).
const PASS_ELEMENT_00E7EB00: u32 = 0x00e7_eb00;
const ELEMENT_BITS_011AE640: u32 = 0x011a_e640;

// Translated from 00be2170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshader.cpp`), no arguments. For the current pass
/// (`0126f74c`): each element of its array (count +0x18, elements +0x24) up to the first null
/// one whose byte +0x64 and word +8 are set gets `00e7eb00(0)`.
pub fn fn_00be2170(e: &mut Engine) {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let count = e.mem.u32(pass + 0x18);
    let mut index = 0u32;
    while index < count {
        let elements = e.mem.u32(pass + 0x24);
        let element = e.mem.u32(elements.wrapping_add(index * 4));
        if element == 0 {
            return;
        }
        if e.mem.u8(element + 0x64) != 0 && e.mem.u32(element + 8) != 0 {
            e.call(PASS_ELEMENT_00E7EB00, &args![element, 0u32]);
        }
        index += 1;
    }
}

// Translated from 00be21b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshader.cpp`), no arguments. For the current pass: each
/// element index whose word in the table at `011ae640` shares a bit with the pass's byte +0x4c
/// (sign-extended): stops at a null element, else `00e7eb00(0)` when its word +8 is set.
pub fn fn_00be21b0(e: &mut Engine) {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let count = e.mem.u32(pass + 0x18);
    let mask = e.mem.u8(pass + 0x4c) as i8 as i32 as u32;
    let mut index = 0u32;
    while index < count {
        if e.mem.u32(ELEMENT_BITS_011AE640 + index * 4) & mask != 0 {
            let elements = e.mem.u32(pass + 0x24);
            let element = e.mem.u32(elements.wrapping_add(index * 4));
            if element == 0 {
                return;
            }
            if e.mem.u32(element + 8) != 0 {
                e.call(PASS_ELEMENT_00E7EB00, &args![element, 0u32]);
            }
        }
        index += 1;
    }
}

// Translated from 00be0ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it; in many shader vtables, the first at `010b00e4`
/// slot 73): `thiscall`, no stack words: `this`'s slot +0x128 with the pass at +0x5c; what it
/// answers in EAX, EDX and ST0 is this function's result.
pub fn fn_00be0ce0(e: &mut Engine, this: Ptr) -> Ret {
    let pass = e.mem.u32(this.addr() + 0x5c);
    e.vcall(this.addr(), 0x128, &args![pass])
}

/// The uniform-form entry of [`fn_00be0ce0`]: `this`; the call's registers pass through.
fn fn_00be0ce0_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00be0ce0(e, Ptr::new(a[0]))
}

// Translated from 00be0e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 1 of the include handler vtable `010bc610`
/// (`stdcall`, `this` and the data): `Close`, the data given back to the memory manager
/// (`Deallocate`). Answers 0 (`S_OK`).
pub fn fn_00be0e30(e: &mut Engine, _this: u32, data: u32) -> u32 {
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, data]);
    0
}

// Translated from 00be0fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it; in many shader vtables, the first at `010af420`
/// slot 74): `thiscall` (`this` not read), one word (a pass): clears the texture word (+8) of
/// each of the pass's stages (count +0x18, signed; array +0x24) whose +0xc is not 1.
pub fn fn_00be0fb0(e: &mut Engine, _this: Ptr, pass: u32) {
    if pass == 0 {
        return;
    }
    let count = e.mem.u32(pass + 0x18) as i32;
    let mut index = 0i32;
    while index < count {
        let stage = e
            .mem
            .u32(e.mem.u32(pass + 0x24).wrapping_add(4 * index as u32));
        if e.mem.u32(stage + 0xc) != 1 {
            e.mem.set_u32(stage + 8, 0);
        }
        index += 1;
    }
}

/// The include handler's path maker `00afe830` (unnamed; `cdecl`: base path, file name, out
/// buffer).
const INCLUDE_PATH_00AFE830: u32 = 0x00af_e830;

// Translated from 00be1eb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00be1eb0 - the /GS stack cookie check (00ec408c) is not translated.
/// Unnamed (Ghidra made no function of it): slot 0 of the include handler vtable `010bc610`
/// (`stdcall`, six words: `this`, the include type, the file name, the parent data, the data
/// and size outputs): `Open`. The path is made from `this`'s base path (+4) and the file name
/// (`00afe830`, a 0x104-byte buffer) and opened (`FileFinder::GetFile(path, 0, 0x4000,
/// 0x20)`); with no file, or one not open (byte +0x2c clear; then deleted through slot 0 with
/// 1), answers `E_FAIL` (0x80004005). Otherwise a block of the file's size (slot +0x28) from
/// the memory manager goes to `*data`, the file's read function (+8, `cdecl`: file, block,
/// size, the address of a 1, 1) fills it, its position (+4) moves on and `*size` gets the bytes
/// read; the file is deleted and 0 (`S_OK`) answered.
pub fn fn_00be1eb0(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
    file_name: u32,
    _unused_3: u32,
    data: Ptr,
    size: Ptr,
) -> u32 {
    let base_path = e.mem.u32(this + 4);
    e.with_stack(0x108, |e, frame| {
        let flag = frame.addr();
        let path = flag + 4;
        e.call(INCLUDE_PATH_00AFE830, &args![base_path, file_name, path]);
        let file = get_file(e, path);
        if file == 0 {
            return 0x8000_4005;
        }
        if e.mem.u8(file + 0x2c) == 0 {
            e.vcall(file, 0, &args![1u32]);
            return 0x8000_4005;
        }
        let bytes = e.vcall(file, 0x28, &args![]).u32();
        let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        e.mem.set_u32(data.addr(), block);
        let read = e.mem.u32(file + 8);
        e.mem.set_u32(flag, 1);
        let got = e.call(read, &args![file, block, bytes, flag, 1u32]).u32();
        let position = e.mem.u32(file + 4).wrapping_add(got);
        e.mem.set_u32(file + 4, position);
        e.mem.set_u32(size.addr(), got);
        e.vcall(file, 0, &args![1u32]);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00be_0ce0, fn_00be0ce0_entry as AbiFn),
        entry!(0x00be_0e30, fn_00be0e30(u32, u32) -> u32),
        entry!(0x00be_0fb0, fn_00be0fb0(Ptr, u32)),
        entry!(
            0x00be_1eb0,
            fn_00be1eb0(u32, u32, u32, u32, Ptr, Ptr) -> u32
        ),
        entry!(0x00be_0c00, bs_shader_make_path(Ptr, Ptr)),
        entry!(0x00be_2200, bs_shader_bs_shader(Ptr) -> Ptr),
        entry!(
            0x00be_0fe0,
            bs_shader_compile_vertex_shader_hlsl(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00be_1750,
            bs_shader_compile_pixel_shader_hlsl(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00be_0b30, ni_d3d_vertex_shader_ni_d3d_vertex_shader(Ptr, Ptr) -> Ptr),
        entry!(0x00be_0dc0, fn_00be0dc0(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00be_0cf0,
            bs_shader_setup_default_stage(Ptr, u32, u32, u32, bool)
        ),
        entry!(0x00be_0e50, fn_00be0e50(Ptr, u32)),
        entry!(0x00be_0e90, fn_00be0e90()),
        entry!(0x00be_0eb0, fn_00be0eb0(Ptr, u32)),
        entry!(0x00be_0ee0, fn_00be0ee0(Ptr, u32)),
        entry!(0x00be_0f10, bs_shader_destructor(Ptr)),
        entry!(0x00be_1f90, fn_00be1f90(Ptr, u32)),
        entry!(0x00be_1ff0, fn_00be1ff0(Ptr, Ptr)),
        entry!(0x00be_20b0, fn_00be20b0(Ptr, Ptr)),
        entry!(0x00be_20e0, fn_00be20e0(Ptr, Ptr, bool)),
        entry!(0x00be_2170, fn_00be2170()),
        entry!(0x00be_21b0, fn_00be21b0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        PASS_DELETE_00E80190, VECTOR_CONSTRUCT,
    };

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn make_path_joins_the_root_folder_and_the_file_folder() {
        let mut e = Engine::new();
        for target in [SPLITPATH_S, STRCAT_S, MAKEPATH_S] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let path = e.mem.alloc(0x104);
        e.call(0x00be_0c00, &args![0x0e00_0000u32, path]);
        let split = calls(&e, SPLITPATH_S);
        assert_eq!(split.len(), 2);
        let first = &split[0];
        let frame = first[7];
        assert_eq!(
            first,
            &vec![
                0x0e00_0000,
                frame + 8,
                0x10,
                frame + 0x19c,
                0x104,
                frame + 0x18,
                0x80,
                frame,
                8
            ]
        );
        assert_eq!(
            split[1],
            vec![
                SHADER_ROOT_011F9308,
                frame + 8,
                0x10,
                frame + 0x98,
                0x104,
                0,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            calls(&e, STRCAT_S),
            vec![vec![frame + 0x98, 0x104, frame + 0x19c]]
        );
        assert_eq!(
            calls(&e, MAKEPATH_S),
            vec![vec![
                path,
                0x104,
                frame + 8,
                frame + 0x98,
                frame + 0x18,
                frame
            ]]
        );
        // A null buffer: nothing.
        e.call(0x00be_0c00, &args![0x0e00_0000u32, 0u32]);
        assert_eq!(calls(&e, SPLITPATH_S).len(), 2);
    }

    #[test]
    fn bs_shader_constructor_clears_its_pass_pointer() {
        let mut e = Engine::new();
        e.register(DEFAULT_SHADER_CONSTRUCT_00E7C6B0, |_, a| a[0].into_ret());
        // The array constructor leaves a pass held once in the slot, so the clear drops it.
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 1);
        e.register_double(VECTOR_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(PASS_DELETE_00E80190, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00be_2200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_SHADER_VTABLE_010BC618);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u8(this + 0x65), 0xcc);
        assert_eq!(
            calls(&e, DEFAULT_SHADER_CONSTRUCT_00E7C6B0),
            vec![vec![this]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![this + 0x5c, 4, 1, 0x00b5_4620, DESTRUCT_00B79930]]
        );
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
    }

    /// Doubles' addresses: a source file, a cache file, the compiled code buffer, the error
    /// buffer, the device and the shader object (each with its own vtable).
    const FILE_VTABLE: u32 = 0x0e40_0000;
    const FILE_DELETE: u32 = 0x0e41_0000;
    const FILE_SIZE: u32 = 0x0e41_0028;
    const BUFFER_VTABLE: u32 = 0x0e42_0000;
    const BUFFER_RELEASE: u32 = 0x0e43_0008;
    const BUFFER_POINTER: u32 = 0x0e43_000c;
    const BUFFER_SIZE: u32 = 0x0e43_0010;
    const DEVICE_VTABLE: u32 = 0x0e44_0000;
    const CREATE_VERTEX: u32 = 0x0e45_016c;
    const CREATE_PIXEL: u32 = 0x0e45_01a8;
    const OBJECT_VTABLE: u32 = 0x0e46_0000;
    const OBJECT_SET_VERTEX: u32 = 0x0e47_0088;
    const OBJECT_SET_PIXEL: u32 = 0x0e47_0080;
    const PATH: u32 = 0x0e50_0000;
    const PROFILE: u32 = 0x0e50_0100;
    const NAME: u32 = 0x0e50_0200;
    const MACRO_A: u32 = 0x0e50_0300;
    const VALUE_A: u32 = 0x0e50_0310;
    const MACRO_B: u32 = 0x0e50_0320;
    const EMPTY: u32 = 0x0e50_0330;
    const CODE: u32 = 0x0e50_0400;
    const ERROR_TEXT: u32 = 0x0e50_0500;
    const OUTPUT_FILE: u32 = 0x0e50_0600;
    const RENDERER: u32 = 0x0e50_0700;
    const DEVICE: u32 = 0x0e50_0800;
    const HANDLE: u32 = 0x0e50_0900;

    /// The compile rig: time checks as given (`cache`, `source` say which files exist), the
    /// files readable (8 bytes each), `D3DXCompileShader` answering `result` with a code
    /// buffer (when `code`) and an error buffer (when `errors`), the device creating `HANDLE`.
    /// Answers the engine and the macro array (two pairs: `A=1`-like with a value, `B` with an
    /// empty value, then a null name, then a pair after it).
    fn compile_rig(
        checks: bool,
        cache: bool,
        result: u32,
        code: bool,
        errors: bool,
    ) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.map(0x0e50_0000, 0x1000);
        e.set_global(CHECK_TIMES_011AE638, u8::from(checks));
        e.set_global(RENDERER_011F9508, RENDERER);
        e.mem.write(VALUE_A, b"1\0");
        e.mem.write(EMPTY, b"\0");
        e.mem.write(PROFILE, b"ps_2_0\0");
        e.mem.set_u32(RENDERER + 0x288, DEVICE);
        e.mem.set_u32(DEVICE, DEVICE_VTABLE);
        let mut device = vec![0u32; 0x6b];
        device[0x5b] = CREATE_VERTEX;
        device[0x6a] = CREATE_PIXEL;
        e.put_vtable(DEVICE_VTABLE, &device);
        let mut file = vec![0u32; 0xb];
        file[0] = FILE_DELETE;
        file[0xa] = FILE_SIZE;
        e.put_vtable(FILE_VTABLE, &file);
        e.put_vtable(
            BUFFER_VTABLE,
            &[0, 0, BUFFER_RELEASE, BUFFER_POINTER, BUFFER_SIZE],
        );
        let mut object = vec![0u32; 0x23];
        object[0x20] = OBJECT_SET_PIXEL;
        object[0x22] = OBJECT_SET_VERTEX;
        e.put_vtable(OBJECT_VTABLE, &object);
        for target in [
            COMPILED_PATH_00BE0DC0,
            STRCPY_S,
            STRCAT_S,
            FORMAT_00406D00,
            FILE_READ_00462D80,
            FILE_DELETE,
            MEMORY_DEALLOCATE,
            FWRITE,
            FFLUSH,
            FCLOSE,
            INVALIDATE_FILE,
            BUFFER_RELEASE,
            OBJECT_SET_VERTEX,
            OBJECT_SET_PIXEL,
            PACKAGE_MAKE_SHADER_00B7FD90,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register_double(FILE_TIME_00AFE7D0, move |_, a| {
            let found = if a[0] == PATH { true } else { cache };
            u32::from(found).into_ret()
        });
        e.register(GET_FILE, |e, _| {
            let file = e.mem.alloc(0x30);
            e.mem.set_u32(file, FILE_VTABLE);
            e.mem.set_u8(file + 0x2c, 1);
            file.into_ret()
        });
        e.register(FILE_SIZE, |_, _| 8u32.into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register_double(D3DX_COMPILE_SHADER, move |e, a| {
            if code {
                let buffer = e.mem.alloc(8);
                e.mem.set_u32(buffer, BUFFER_VTABLE);
                e.mem.set_u32(a[7], buffer);
            }
            if errors {
                let buffer = e.mem.alloc(8);
                e.mem.set_u32(buffer, BUFFER_VTABLE);
                e.mem.set_u32(buffer + 4, 1);
                e.mem.set_u32(a[8], buffer);
            }
            result.into_ret()
        });
        e.register(BUFFER_POINTER, |e, a| {
            let text = if e.mem.u32(a[0] + 4) == 1 {
                ERROR_TEXT
            } else {
                CODE
            };
            text.into_ret()
        });
        e.register(BUFFER_SIZE, |_, _| 0x40u32.into_ret());
        e.register(FILE_OPEN_00EC9A47, |_, _| OUTPUT_FILE.into_ret());
        e.register(CREATE_VERTEX, |e, a| {
            e.mem.set_u32(a[2], HANDLE);
            0u32.into_ret()
        });
        e.register(CREATE_PIXEL, |e, a| {
            e.mem.set_u32(a[2], HANDLE);
            0u32.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for constructor in [
            VERTEX_SHADER_CONSTRUCT_00BE0B30,
            PIXEL_SHADER_CONSTRUCT_00BE08F0,
        ] {
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0], OBJECT_VTABLE);
                a[0].into_ret()
            });
        }
        e.register(STRNCMP, |e, a| {
            let same = e.mem.bytes(a[0], 4) == e.mem.bytes(a[1], 4);
            u32::from(!same).into_ret()
        });
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e50_0110u32.into_ret());
        e.map(0x010a_9000, 0x1000);
        e.mem.write(PS_2_0_010A93CC, b"ps_2_0\0");
        let macros = e.mem.alloc(0x40);
        let words = [MACRO_A, VALUE_A, MACRO_B, EMPTY, 0, 0, MACRO_A, 0];
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(macros + 4 * index as u32, *word);
        }
        e.call_log = Some(vec![]);
        (e, macros)
    }

    fn vertex(e: &mut Engine, macros: u32) -> u32 {
        e.call(0x00be_0fe0, &args![0u32, PATH, macros, PROFILE, NAME])
            .u32()
    }

    fn pixel(e: &mut Engine, macros: u32) -> u32 {
        e.call(0x00be_1750, &args![0u32, PATH, macros, PROFILE, NAME])
            .u32()
    }

    /// The order of the time checks, as (is the source, result buffer offset from the cache
    /// path) pairs.
    fn time_checks(e: &Engine, cache_path: u32) -> Vec<(bool, u32)> {
        calls(e, FILE_TIME_00AFE7D0)
            .iter()
            .map(|a| (a[0] == PATH, cache_path.wrapping_sub(a[1])))
            .collect()
    }

    #[test]
    fn vertex_compile_builds_the_macros_writes_the_cache_and_makes_the_shader() {
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        let shader = vertex(&mut e, macros);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert_eq!(
            calls(&e, COMPILED_PATH_00BE0DC0),
            vec![vec![cache_path, PATH, NAME, VSO_010BC868]]
        );
        let frame = cache_path - 0x60;
        // Cache first (result at +0x2c), then the source (+0x40).
        assert_eq!(
            time_checks(&e, cache_path),
            vec![(false, 0x34), (true, 0x20)]
        );
        assert_eq!(calls(&e, GET_FILE), vec![vec![PATH, 0, 0x4000, 0x20]]);
        let list = frame + 0x268;
        let command = frame + 0x880;
        let strcat: Vec<(u32, u32)> = calls(&e, STRCAT_S)
            .iter()
            .map(|a| {
                assert_eq!(a[1], 0x514);
                (a[0], a[2])
            })
            .collect();
        assert_eq!(
            strcat,
            vec![
                (list, MACRO_A),
                (command, DEFINE_010BC824),
                (command, MACRO_A),
                (list, EQUALS_01063EC8),
                (list, VALUE_A),
                (command, EQUALS_01063EC8),
                (command, VALUE_A),
                (list, SEPARATOR_01012630),
                (list, MACRO_B),
                (command, DEFINE_010BC824),
                (command, MACRO_B),
                (command, EQUALS_ONE_010BC820),
                (list, SEPARATOR_01012630),
                (list, MACRO_A),
                (command, DEFINE_010BC824),
                (command, MACRO_A),
                (command, EQUALS_ONE_010BC820),
                (list, SEPARATOR_01012630),
            ]
        );
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![
                frame + 0xd94,
                0x514,
                COMPILING_010BC808,
                PATH,
                PROFILE,
                list
            ]]
        );
        let compile = calls(&e, D3DX_COMPILE_SHADER);
        let source = calls(&e, FILE_READ_00462D80)[0][1];
        assert_eq!(
            compile,
            vec![vec![
                source,
                8,
                macros,
                frame + 0x58,
                ENTRY_MAIN_01094D90,
                PROFILE,
                0,
                frame + COMPILED,
                frame + ERRORS,
                frame + 0x50
            ]]
        );
        assert_eq!(calls(&e, FILE_DELETE).len(), 1);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, source]]
        );
        assert_eq!(
            calls(&e, FILE_OPEN_00EC9A47),
            vec![vec![cache_path, MODE_WB_0106E798]]
        );
        assert_eq!(calls(&e, FWRITE), vec![vec![CODE, 0x40, 1, OUTPUT_FILE]]);
        assert!(calls(&e, FFLUSH).is_empty());
        assert_eq!(calls(&e, FCLOSE), vec![vec![OUTPUT_FILE]]);
        assert_eq!(calls(&e, INVALIDATE_FILE), vec![vec![cache_path]]);
        assert_eq!(
            calls(&e, CREATE_VERTEX),
            vec![vec![DEVICE, CODE, frame + 0x4c]]
        );
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x3c]]);
        assert_eq!(
            calls(&e, VERTEX_SHADER_CONSTRUCT_00BE0B30),
            vec![vec![shader, RENDERER]]
        );
        assert_eq!(calls(&e, OBJECT_SET_VERTEX), vec![vec![shader, HANDLE]]);
    }

    #[test]
    fn vertex_compile_adds_the_hdr_macro_and_clears_it_again() {
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        e.set_global(HDR_MACRO_011F941E, 1u8);
        // The value `""` the macro gets is read.
        e.map(0x0101_1000, 0x1000);
        e.set_global(SHADER_MAKER_011F9494, 0x0e60_0000u32);
        let shader = vertex(&mut e, macros);
        // The package's maker takes the code; no object is made.
        assert_eq!(shader, 0);
        assert_eq!(
            calls(&e, PACKAGE_MAKE_SHADER_00B7FD90),
            vec![vec![0x0e60_0000, NAME, CODE, 0x40]]
        );
        let strcat = calls(&e, STRCAT_S);
        assert!(strcat.iter().any(|a| a[2] == MACRO_HDR_010BC828));
        assert_eq!(e.mem.u32(macros + 0x10), 0);
        assert_eq!(e.mem.u32(macros + 0x14), 0);
    }

    #[test]
    fn vertex_compile_errors_are_reported_and_the_missing_cache_gives_up() {
        let (mut e, macros) = compile_rig(true, false, 0x8000_4005, false, true);
        assert_eq!(vertex(&mut e, macros), 0);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        let frame = cache_path - 0x60;
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(
            formats[1],
            vec![
                frame + 0xd94,
                0x514,
                COMPILE_ERRORS_010BC7FC,
                PATH,
                frame + 0x268,
                ERROR_TEXT
            ]
        );
        assert_eq!(calls(&e, BUFFER_RELEASE).len(), 1);
        // The second list stops at the first empty name: A, =, 1, ", ", B, ", ".
        let second: Vec<u32> = calls(&e, STRCAT_S).iter().skip(18).map(|a| a[2]).collect();
        assert_eq!(
            second,
            vec![
                MACRO_A,
                EQUALS_01063EC8,
                VALUE_A,
                SEPARATOR_01012630,
                MACRO_B,
                SEPARATOR_01012630
            ]
        );
        assert!(calls(&e, FILE_OPEN_00EC9A47).is_empty());
        assert_eq!(calls(&e, GET_FILE).len(), 1);
        assert!(calls(&e, CREATE_VERTEX).is_empty());
    }

    #[test]
    fn vertex_compile_loads_the_cache_and_frees_it() {
        let (mut e, macros) = compile_rig(false, false, 0, true, false);
        let shader = vertex(&mut e, macros);
        assert_ne!(shader, 0);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert!(calls(&e, FILE_TIME_00AFE7D0).is_empty());
        assert_eq!(calls(&e, GET_FILE), vec![vec![cache_path, 0, 0x4000, 0x20]]);
        let bytes = calls(&e, FILE_READ_00462D80)[0][1];
        assert_eq!(calls(&e, CREATE_VERTEX)[0][1], bytes);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, bytes]]
        );
        assert!(calls(&e, D3DX_COMPILE_SHADER).is_empty());
    }

    #[test]
    fn vertex_compile_uses_a_packaged_shader_and_retries_the_cache_root() {
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        e.set_global(SHADER_PACKAGE_011F9498, 0x0e61_0000u32);
        e.register(PACKAGE_FIND_00B7FAC0, |_, _| 0x0e62_0000u32.into_ret());
        vertex(&mut e, macros);
        assert_eq!(
            calls(&e, PACKAGE_FIND_00B7FAC0),
            vec![vec![0x0e61_0000, NAME]]
        );
        assert!(calls(&e, FILE_TIME_00AFE7D0).is_empty());
        assert_eq!(calls(&e, CREATE_VERTEX)[0][1], 0x0e62_0104);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        // No package entry, the retry allowed: "cache:\\" set once and the path rebuilt.
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        e.set_global(CACHE_RETRY_011F94A7, 1u8);
        vertex(&mut e, macros);
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![vec![CACHE_ROOT_011F9200, 0x104, CACHE_ROOT_STRING_010BC860]]
        );
        assert_eq!(calls(&e, COMPILED_PATH_00BE0DC0).len(), 2);
        assert_eq!(calls(&e, FILE_TIME_00AFE7D0).len(), 3);
    }

    #[test]
    fn pixel_compile_picks_the_profile_and_writes_without_a_check() {
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        e.register(FILE_OPEN_00EC9A47, |_, _| 0u32.into_ret());
        let shader = pixel(&mut e, macros);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert_eq!(
            calls(&e, COMPILED_PATH_00BE0DC0)[0],
            vec![cache_path, PATH, NAME, PSO_010BC8A0]
        );
        let frame = cache_path - 0x64;
        // Source first (result at +0x38), then the cache (+0x30).
        assert_eq!(
            time_checks(&e, cache_path),
            vec![(true, 0x2c), (false, 0x34)]
        );
        assert_eq!(calls(&e, STRNCMP), vec![vec![PROFILE, PS_2_0_010A93CC, 4]]);
        assert_eq!(calls(&e, PIXEL_PROFILE_00B4F380), vec![vec![1]]);
        let compile = &calls(&e, D3DX_COMPILE_SHADER)[0];
        assert_eq!(compile[3], frame + 0x5c);
        assert_eq!(compile[5], 0x0e50_0110);
        assert_eq!(compile[9], frame + 0x58);
        assert_eq!(
            calls(&e, FORMAT_00406D00)[0],
            vec![
                frame + 0xd98,
                0x514,
                COMPILING_010BC808,
                PATH,
                PROFILE,
                frame + 0x168
            ]
        );
        // The open failed, yet the code is written and flushed through the null file.
        assert_eq!(calls(&e, FWRITE), vec![vec![CODE, 0x40, 1, 0]]);
        assert_eq!(calls(&e, FFLUSH), vec![vec![0]]);
        assert_eq!(calls(&e, FCLOSE), vec![vec![0]]);
        assert_eq!(
            calls(&e, CREATE_PIXEL),
            vec![vec![DEVICE, CODE, frame + 0x54]]
        );
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x30]]);
        assert_eq!(calls(&e, OBJECT_SET_PIXEL), vec![vec![shader, HANDLE]]);
    }

    #[test]
    fn pixel_compile_error_list_skips_empty_names() {
        let (mut e, macros) = compile_rig(true, false, 1, false, true);
        assert_eq!(pixel(&mut e, macros), 0);
        let second: Vec<u32> = calls(&e, STRCAT_S).iter().skip(18).map(|a| a[2]).collect();
        assert_eq!(
            second,
            vec![
                MACRO_A,
                EQUALS_01063EC8,
                VALUE_A,
                SEPARATOR_01012630,
                MACRO_B,
                SEPARATOR_01012630,
                MACRO_A,
                SEPARATOR_01012630
            ]
        );
    }

    #[test]
    fn compile_reports_missing_files_and_a_failed_create() {
        // The source cannot be found when compiling: the error goes to the frame's buffer.
        let (mut e, macros) = compile_rig(true, false, 0, true, false);
        e.register(GET_FILE, |_, _| 0u32.into_ret());
        assert_eq!(vertex(&mut e, macros), 0);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![
                cache_path - 0x60 + 0x164,
                0x104,
                HLSL_ERROR_010BC7B8,
                PATH
            ]]
        );
        // The cache file cannot be opened.
        let (mut e, macros) = compile_rig(false, false, 0, true, false);
        e.register(GET_FILE, |_, _| 0u32.into_ret());
        assert_eq!(pixel(&mut e, macros), 0);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![
                cache_path - 0x64 + 0x780,
                0x104,
                CACHE_ERROR_010BC754,
                cache_path
            ]]
        );
        // The device refuses the code.
        let (mut e, macros) = compile_rig(false, false, 0, true, false);
        e.register(CREATE_VERTEX, |_, _| 1u32.into_ret());
        assert_eq!(vertex(&mut e, macros), 0);
        let cache_path = calls(&e, COMPILED_PATH_00BE0DC0)[0][0];
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![
                cache_path - 0x60 + 0x77c,
                0x104,
                VERTEX_CREATE_ERROR_010BC784,
                cache_path
            ]]
        );
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn vertex_shader_constructor_clears_its_fields() {
        let mut e = Engine::new();
        e.register(SHADER_PROGRAM_CONSTRUCT_00E897D0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xcc; 0x3c]);
        assert_eq!(
            e.call(0x00be_0b30, &args![this, 0x0e00_0000u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VERTEX_SHADER_VTABLE_010EF87C);
        assert_eq!(e.mem.u8(this + 0x2c), 0);
        assert_eq!(e.mem.u8(this + 0x2d), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x30, 0xc), vec![0; 0xc]);
        assert_eq!(
            calls(&e, SHADER_PROGRAM_CONSTRUCT_00E897D0),
            vec![vec![this, 0x0e00_0000, 0]]
        );
    }

    #[test]
    fn compiled_path_uses_the_cache_root_and_the_hdr_folder() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(STRCAT_S, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let buffer = 0x0e10_0000u32;
        e.call(0x00be_0dc0, &args![buffer, 1u32, 0x0e20_0000u32, 0u32]);
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![vec![buffer, 0x104, SHADER_ROOT_011F9308]]
        );
        assert_eq!(calls(&e, STRCAT_S), vec![vec![buffer, 0x104, 0x0e20_0000]]);
        e.set_global(CACHE_ROOT_011F9200, b'c');
        e.set_global(HDR_MACRO_011F941E, 1u8);
        e.call(0x00be_0dc0, &args![buffer, 1u32, 0x0e20_0000u32, 0u32]);
        assert_eq!(
            calls(&e, STRCPY_S)[1],
            vec![buffer, 0x104, CACHE_ROOT_011F9200]
        );
        assert_eq!(
            calls(&e, STRCAT_S)[1..].to_vec(),
            vec![
                vec![buffer, 0x104, HDR_FOLDER_010BC608],
                vec![buffer, 0x104, 0x0e20_0000]
            ]
        );
        // Any null pointer: nothing.
        for words in [[0u32, 1, 2], [buffer, 0, 2], [buffer, 1, 0]] {
            e.call(0x00be_0dc0, &args![words[0], words[1], words[2], 0u32]);
        }
        assert_eq!(calls(&e, STRCPY_S).len(), 2);
    }
}

#[cfg(test)]
mod b054_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::PASS_DELETE_00E80190;

    fn rig(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for target in doubles {
            e.register(*target, |_, _| Ret::default());
        }
        e.map(0x011f_9000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The order of the logged calls to any of `addrs` (address and words).
    fn sequence(e: &Engine, addrs: &[u32]) -> Vec<(u32, Vec<u32>)> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| addrs.contains(&c.0))
            .cloned()
            .collect()
    }

    #[test]
    fn setup_default_stage_sets_the_states() {
        let mut e = rig(&[
            STAGE_STATE_00E88FC0,
            SAMPLER_STATE_00E890C0,
            STAGE_MODE_00E7DE90,
            SET_FILTER_MODE,
        ]);
        let stage = e.mem.alloc(0x60);
        e.mem.set_u32(stage + 0x10, 0x0e00_0010);
        e.mem.set_u8(stage + 0x5e, 9);
        e.call(0x00be_0cf0, &args![stage, 2u32, 3u32, 0x11u32, 1u32]);
        assert_eq!(e.mem.u32(stage + 4), 2);
        assert_eq!(e.mem.u8(stage + 0x5e), 0);
        let b = 0x0e00_0010;
        assert_eq!(
            calls(&e, STAGE_STATE_00E88FC0),
            vec![
                vec![b, 0xb, 2, 0],
                vec![b, 1, 1, 0],
                vec![b, 2, 2, 0],
                vec![b, 4, 1, 0],
                vec![b, 5, 2, 0],
                vec![b, 0x18, 0, 0]
            ]
        );
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![stage, 0]]);
        assert_eq!(
            calls(&e, SAMPLER_STATE_00E890C0),
            vec![
                vec![b, 5, 0x11, 0, 0],
                vec![b, 6, 0x11, 0, 0],
                vec![b, 7, 0x11, 0, 0]
            ]
        );
        assert_eq!(calls(&e, SET_FILTER_MODE), vec![vec![stage, 6]]);
        // Another mode word gives 3; no filter flag gives 2.
        e.call(0x00be_0cf0, &args![stage, 0u32, 1u32, 0u32, 0x100u32]);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90)[1], vec![stage, 3]);
        assert_eq!(calls(&e, SET_FILTER_MODE)[1], vec![stage, 2]);
    }

    #[test]
    fn fn_00be0e50_sets_the_states_outside_the_pass_types() {
        let states = [
            RENDER_STATE_00B97FA0,
            SET_Z_WRITE_ENABLE,
            RENDER_STATE_00B97E80,
        ];
        let mut e = rig(&states);
        for pass_type in [0x252u32, 0x256] {
            e.set_global(RENDERING_PASS_TYPE_011F91E4, pass_type);
            e.call(0x00be_0e50, &args![0x5555u32, 0u32]);
        }
        assert!(sequence(&e, &states).is_empty());
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x257u32);
        e.call(0x00be_0e50, &args![0x5555u32, 0u32]);
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x251u32);
        e.call(0x00be_0e50, &args![0x5555u32, 0u32]);
        let once = vec![
            (RENDER_STATE_00B97FA0, vec![1, 0]),
            (SET_Z_WRITE_ENABLE, vec![0, 0]),
            (RENDER_STATE_00B97E80, vec![2, 0]),
        ];
        let mut twice = once.clone();
        twice.extend(once);
        assert_eq!(sequence(&e, &states), twice);
    }

    #[test]
    fn fn_00be0e90_sets_the_states() {
        let states = [
            RENDER_STATE_00B97FA0,
            SET_Z_WRITE_ENABLE,
            RENDER_STATE_00B97E80,
        ];
        let mut e = rig(&states);
        e.call(0x00be_0e90, &args![]);
        assert_eq!(
            sequence(&e, &states),
            vec![
                (RENDER_STATE_00B97FA0, vec![0, 0]),
                (SET_Z_WRITE_ENABLE, vec![1, 0]),
                (RENDER_STATE_00B97E80, vec![3, 0]),
            ]
        );
    }

    /// The current pass with a list of two nodes ((1, 2), (3, 4)).
    fn pass_with_list(e: &mut Engine) {
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second, 3);
        e.mem.set_u32(second + 4, 4);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, 1);
        e.mem.set_u32(first + 4, 2);
        e.mem.set_u32(first + 8, second);
        let states = e.mem.alloc(0x14);
        e.mem.set_u32(states + 0x10, first);
        let pass = e.mem.alloc(0x34);
        e.mem.set_u32(pass + 0x30, states);
        e.set_global(CURRENT_PASS_0126F74C, pass);
    }

    #[test]
    fn fn_00be0eb0_walks_the_pass_list() {
        let mut e = rig(&[ITEM_STATE_00B97D90]);
        pass_with_list(&mut e);
        e.call(0x00be_0eb0, &args![0x5555u32, 0u32]);
        assert_eq!(
            calls(&e, ITEM_STATE_00B97D90),
            vec![vec![1, 2, 0], vec![3, 4, 0]]
        );
    }

    #[test]
    fn fn_00be0ee0_walks_the_pass_list() {
        let mut e = rig(&[ITEM_STATE_00B98610]);
        pass_with_list(&mut e);
        e.call(0x00be_0ee0, &args![0x5555u32, 0u32]);
        assert_eq!(calls(&e, ITEM_STATE_00B98610), vec![vec![1, 0], vec![3, 0]]);
    }

    #[test]
    fn bs_shader_destructor_drops_the_pass() {
        let mut e = rig(&[
            PASS_DELETE_00E80190,
            RENDERER_METHOD_00E75340,
            VECTOR_DESTRUCT,
            BASE_DESTRUCT_00E7C540,
        ]);
        e.set_global(RENDERER_011F9508, 0x0e00_0500u32);
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 1);
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x5c, pass);
        e.call(0x00be_0f10, &args![this]);
        assert_eq!(e.mem.u32(this), BS_SHADER_VTABLE_010BC618);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        let all = [
            PASS_DELETE_00E80190,
            RENDERER_METHOD_00E75340,
            VECTOR_DESTRUCT,
            BASE_DESTRUCT_00E7C540,
        ];
        assert_eq!(
            sequence(&e, &all),
            vec![
                (PASS_DELETE_00E80190, vec![pass]),
                (RENDERER_METHOD_00E75340, vec![0x0e00_0500, this]),
                (VECTOR_DESTRUCT, vec![this + 0x5c, 4, 1, DESTRUCT_00B79930]),
                (BASE_DESTRUCT_00E7C540, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_00be1f90_hands_the_pass_values_over() {
        let mut e = rig(&[]);
        let object = e.mem.alloc(4);
        e.put_vtable(0x0e10_0000, &[0; 0x24]);
        e.mem.set_u32(0x0e10_0000 + 0x8c, 0x0e11_008c);
        e.mem.set_u32(0x0e10_0000 + 0x7c, 0x0e11_007c);
        e.mem.set_u32(object, 0x0e10_0000);
        e.register(0x0e11_008c, |_, _| Ret::default());
        e.register(0x0e11_007c, |_, _| Ret::default());
        e.register_double(PASS_GETTER_00E7F7C0, move |_, _| object.into_ret());
        e.put_vtable(0x0e12_0000, &[0; 0x22]);
        e.mem.set_u32(0x0e12_0000 + 0x84, 0x0e13_0084);
        e.mem.set_u32(0x0e12_0000 + 0x7c, 0x0e13_007c);
        e.register(0x0e13_0084, |_, _| 0x84u32.into_ret());
        e.register(0x0e13_007c, |_, _| 0x7cu32.into_ret());
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x0e12_0000);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x5c, source);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        e.call(0x00be_1f90, &args![0u32, 0u32]);
        assert_eq!(calls(&e, PASS_GETTER_00E7F7C0), vec![vec![pass]]);
        assert_eq!(calls(&e, 0x0e13_0084), vec![vec![source]]);
        assert_eq!(calls(&e, 0x0e11_008c), vec![vec![object, 0x84, 0]]);
        assert!(calls(&e, 0x0e11_007c).is_empty());
        e.mem.set_u32(pass + 0x44, source);
        e.call(0x00be_1f90, &args![0u32, 0u32]);
        assert_eq!(calls(&e, 0x0e13_007c), vec![vec![source]]);
        assert_eq!(calls(&e, 0x0e11_007c), vec![vec![object, 0x7c, 0]]);
    }

    #[test]
    fn fn_00be1ff0_picks_the_states_by_floats_and_flags() {
        let mut e = rig(&[RENDER_STATE_00B97FA0, RENDER_STATE_00B97FF0]);
        let property = e.mem.alloc(0x1c);
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x1c, 0);
        e.mem.set_f32(object + 0x28, 1.0);
        e.mem.set_f32(object + 0x2c, 2.0);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, property);
        e.mem.set_u32(block + 0xc, object);
        e.call(0x00be_1ff0, &args![0u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FA0), vec![vec![0, 0]]);
        // A float below 1.0: on, with 6 and 7.
        e.mem.set_f32(object + 0x2c, 0.5);
        e.call(0x00be_1ff0, &args![0u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FA0)[1], vec![1, 0]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FF0), vec![vec![6, 7, 0]]);
        // NaN counts as at least 1.0; flag bit 0 gives the flag fields.
        e.mem.set_f32(object + 0x2c, f32::NAN);
        e.mem.set_u16(property + 0x18, 0x01e5);
        e.call(0x00be_1ff0, &args![0u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FF0)[1], vec![2, 0xf, 0]);
        e.mem.set_u16(property + 0x18, 0);
        e.call(0x00be_1ff0, &args![0u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FA0)[3], vec![0, 0]);
    }

    #[test]
    fn fn_00be20b0_passes_the_flag_fields() {
        let mut e = rig(&[RENDER_STATE_00B97F20]);
        let property = e.mem.alloc(0x1c);
        e.mem.set_u16(property + 0x18, 0x1c00 | 0x0400);
        e.mem.set_u8(property + 0x1a, 0x42);
        let block = e.mem.alloc(4);
        e.mem.set_u32(block, property);
        e.call(0x00be_20b0, &args![0u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97F20), vec![vec![7, 0x42, 0]]);
    }

    #[test]
    fn fn_00be20e0_keeps_the_block_bytes_above_the_flag() {
        let mut e = rig(&[BATCH_STATE_00B984D0, RENDER_STATE_00B97ED0]);
        let first = e.mem.alloc(0x1c);
        e.mem.set_u16(first + 0x18, 0x0200);
        let second = e.mem.alloc(0x1c);
        e.mem.set_u16(second + 0x18, 0x0c00);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, first);
        e.mem.set_u32(block + 0x10, second);
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x10u32);
        e.call(0x00be_20e0, &args![0u32, block, 1u32]);
        assert_eq!(calls(&e, BATCH_STATE_00B984D0), vec![vec![3, 0]]);
        assert_eq!(
            calls(&e, RENDER_STATE_00B97ED0),
            vec![vec![(block & 0xffff_ff00) | 1, 0]]
        );
        // Neither bit pair nor pass type: 1; without the flag the low byte is 0.
        e.mem.set_u16(second + 0x18, 0x0800);
        e.call(0x00be_20e0, &args![0u32, block, 0x100u32]);
        assert_eq!(calls(&e, BATCH_STATE_00B984D0)[1], vec![1, 0]);
        assert_eq!(
            calls(&e, RENDER_STATE_00B97ED0)[1],
            vec![block & 0xffff_ff00, 0]
        );
        // Pass type 0x57 sets it; a null block passes 0.
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x57u32);
        e.call(0x00be_20e0, &args![0u32, 0u32, 1u32]);
        assert_eq!(calls(&e, BATCH_STATE_00B984D0)[2], vec![3, 0]);
        assert_eq!(calls(&e, RENDER_STATE_00B97ED0)[2], vec![0, 0]);
    }

    /// The current pass with elements (byte +0x64, word +8), count `count`.
    fn pass_with_elements(e: &mut Engine, elements: &[(u8, u32)], count: u32) -> Vec<u32> {
        let block = e.mem.alloc(4 * count);
        let mut made = vec![];
        for (index, (byte, word)) in elements.iter().enumerate() {
            let element = e.mem.alloc(0x68);
            e.mem.set_u8(element + 0x64, *byte);
            e.mem.set_u32(element + 8, *word);
            e.mem.set_u32(block + 4 * index as u32, element);
            made.push(element);
        }
        let pass = e.mem.alloc(0x50);
        e.mem.set_u32(pass + 0x18, count);
        e.mem.set_u32(pass + 0x24, block);
        e.mem.set_u8(pass + 0x4c, 0x81);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        made
    }

    #[test]
    fn fn_00be2170_updates_the_marked_elements() {
        let mut e = rig(&[PASS_ELEMENT_00E7EB00]);
        let made = pass_with_elements(&mut e, &[(1, 1), (0, 1), (1, 0), (1, 5)], 6);
        e.call(0x00be_2170, &args![]);
        assert_eq!(
            calls(&e, PASS_ELEMENT_00E7EB00),
            vec![vec![made[0], 0], vec![made[3], 0]]
        );
    }

    #[test]
    fn fn_00be21b0_updates_the_elements_the_table_selects() {
        let mut e = rig(&[PASS_ELEMENT_00E7EB00]);
        let made = pass_with_elements(&mut e, &[(0, 1), (0, 1), (0, 0), (0, 1)], 5);
        // Index 0: bit 0; index 1: no bit; index 2: bit 31 (the sign-extended byte has it).
        e.set_global(ELEMENT_BITS_011AE640, 1u32);
        e.set_global(ELEMENT_BITS_011AE640 + 8, 0x8000_0000u32);
        e.set_global(ELEMENT_BITS_011AE640 + 12, 0x4000_0000u32);
        // Index 4 is selected but null: the walk stops there.
        e.set_global(ELEMENT_BITS_011AE640 + 16, 1u32);
        e.call(0x00be_21b0, &args![]);
        assert_eq!(
            calls(&e, PASS_ELEMENT_00E7EB00),
            vec![vec![made[0], 0], vec![made[3], 0]]
        );
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

    const SLOTS: u32 = 0x0f40_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x130);
        for offset in (0..0x130u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for offset in (0..0x130u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00be0ce0_and_00be0e30() {
        let mut e = rig();
        e.register(SLOTS + 0x128, |_, _| 6u32.into_ret());
        let this = object(&mut e, 0x60);
        e.mem.set_u32(this + 0x5c, 0x5c5c);
        assert_eq!(e.call(0x00be_0ce0, &args![this]).u32(), 6);
        assert_eq!(calls(&e, SLOTS + 0x128), vec![vec![this, 0x5c5c]]);
        assert_eq!(e.call(0x00be_0e30, &args![this, 0x9999u32]).u32(), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9999]]
        );
    }

    #[test]
    fn fn_00be0fb0_clears_stages_not_marked_1() {
        let mut e = rig();
        let stages: Vec<u32> = (0..3)
            .map(|i| {
                let stage = e.mem.alloc(0x10);
                e.mem.set_u32(stage + 8, 0x88);
                e.mem.set_u32(stage + 0xc, i);
                stage
            })
            .collect();
        let array = e.mem.alloc(12);
        for (i, stage) in stages.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *stage);
        }
        let pass = e.mem.alloc(0x28);
        e.mem.set_u32(pass + 0x18, 3);
        e.mem.set_u32(pass + 0x24, array);
        e.call(0x00be_0fb0, &args![0u32, pass]);
        let words: Vec<u32> = stages.iter().map(|s| e.mem.u32(s + 8)).collect();
        assert_eq!(words, vec![0, 0x88, 0]);
        e.call(0x00be_0fb0, &args![0u32, 0u32]);
    }

    #[test]
    fn fn_00be1eb0_reads_the_include() {
        let mut e = rig();
        e.register(INCLUDE_PATH_00AFE830, |_, _| Ret::default());
        e.register(0x0f41_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0x41);
            a[2].into_ret()
        });
        e.register(SLOTS + 0x28, |_, _| 0x20u32.into_ret());
        let file = object(&mut e, 0x30);
        e.mem.set_u32(file + 4, 0x100);
        e.mem.set_u32(file + 8, 0x0f41_0000);
        e.mem.set_u8(file + 0x2c, 1);
        e.map(0x0f42_0000, 0x10);
        e.set_global(0x0f42_0000, file);
        e.register(GET_FILE, |e, _| e.mem.u32(0x0f42_0000).into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x4444);
        let data = e.mem.alloc(4);
        let size = e.mem.alloc(4);
        let answer = e
            .call(0x00be_1eb0, &args![this, 0u32, 0x5555u32, 0u32, data, size])
            .u32();
        assert_eq!(answer, 0);
        let block = e.mem.u32(data);
        assert_eq!(e.mem.u8(block), 0x41);
        assert_eq!(e.mem.u32(size), 0x20);
        assert_eq!(e.mem.u32(file + 4), 0x120);
        assert_eq!(calls(&e, SLOTS), vec![vec![file, 1]]);
        let path = &calls(&e, INCLUDE_PATH_00AFE830)[0];
        assert_eq!(path[..2], [0x4444, 0x5555]);
        // A file not open: deleted, E_FAIL; no file: E_FAIL.
        e.mem.set_u8(file + 0x2c, 0);
        let answer = e
            .call(0x00be_1eb0, &args![this, 0u32, 0x5555u32, 0u32, data, size])
            .u32();
        assert_eq!(answer, 0x8000_4005);
        assert_eq!(calls(&e, SLOTS).len(), 2);
        e.set_global(0x0f42_0000, 0u32);
        let answer = e
            .call(0x00be_1eb0, &args![this, 0u32, 0x5555u32, 0u32, data, size])
            .u32();
        assert_eq!(answer, 0x8000_4005);
    }
}
