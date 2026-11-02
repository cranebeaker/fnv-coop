//! `NiXenonRenderer/nid3dgpuprogramcache.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two `NiTSet<NiPointer<NiMaterialResource>, NiTNewInterface<..>>` members this unit
//! emits read the set at its Xbox PDB offsets (the PC code uses the same): `m_pBase` +0,
//! `m_uiAlloced` +4, `m_uiUsed` +8 (0xc bytes).

#[allow(unused_imports)]
use crate::prelude::*;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `NiPointer` array allocator `00c1d790` (unit `nipsysmeshemitter.cpp`, unnamed; `cdecl`,
/// the element count: an `operator new[]` block of constructed null pointers) and its
/// release `00a47600` (unnamed; `cdecl`, the array: releases every element and frees the
/// block).
const POINTER_ARRAY_NEW_00C1D790: u32 = 0x00c1_d790;
const POINTER_ARRAY_DELETE_00A47600: u32 = 0x00a4_7600;

/// `NiPointer<T>::operator=` from another pointer at `source`, as these two functions expand
/// it: nothing when the slot already holds what `source` holds; else the slot's object is
/// released (deleted through its vtable slot +4 when its count at +4 reaches 0), `source` is
/// read again, stored and its object counted when not null.
fn assign_from(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00e7cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `NiGPUProgramCache::IsNode`; the body is `XOR EAX,EAX; RET` (it
/// reads no register or stack word and answers 0), a body the linker shares with other
/// classes' methods, so the name is not checked. No parameters: callers pass `this` or
/// nothing, and neither is read.
pub fn fn_00e7cd40(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00e8e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTSet<NiPointer<NiMaterialResource>, ..>` reallocation (the engine map has no name;
/// it is the set's `Realloc`, called by `Add` `00e8ea60`), `thiscall`, one word: nothing when
/// `new_max` is `m_uiAlloced`. Else a new array of `new_max` pointers (`00c1d790`; none for 0)
/// takes the first `m_uiUsed` elements (each assigned, so counted once more; with no new array
/// nothing is copied), the old array is released (`00a47600`), and the set keeps the new array
/// and `new_max`. `m_uiUsed` is not changed.
pub fn ni_t_set_material_resource_realloc(e: &mut Engine, this: Ptr, new_max: u32) {
    let set = this.addr();
    if new_max == e.mem.u32(set + 4) {
        return;
    }
    let base = if new_max != 0 {
        let base = e.call(POINTER_ARRAY_NEW_00C1D790, &args![new_max]).u32();
        let mut index = 0u32;
        while index < e.mem.u32(set + 8) {
            let old = e.mem.u32(set).wrapping_add(index.wrapping_mul(4));
            assign_from(e, base.wrapping_add(index.wrapping_mul(4)), old);
            index += 1;
        }
        base
    } else {
        0
    };
    let old = e.mem.u32(set);
    e.call(POINTER_ARRAY_DELETE_00A47600, &args![old]);
    e.mem.set_u32(set, base);
    e.mem.set_u32(set + 4, new_max);
}

// Translated from 00e8ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiPointer<NiMaterialResource>, NiTNewInterface<..>>::Add` (Xbox PDB), `thiscall`, one
/// word (the address of the pointer to add): a full set grows first (`00e8e3a0` with twice
/// `m_uiAlloced`, or 1 when it is 0); then the element at `m_uiUsed` (the count goes up first)
/// is assigned from the pointer.
pub fn ni_t_set_material_resource_add(e: &mut Engine, this: Ptr, element: u32) {
    let set = this.addr();
    let alloced = e.mem.u32(set + 4);
    if e.mem.u32(set + 8) == alloced {
        let grown = if alloced != 0 {
            alloced.wrapping_add(alloced)
        } else {
            1
        };
        ni_t_set_material_resource_realloc(e, this, grown);
    }
    let used = e.mem.u32(set + 8);
    let slot = e.mem.u32(set).wrapping_add(used.wrapping_mul(4));
    e.mem.set_u32(set + 8, used.wrapping_add(1));
    assign_from(e, slot, element);
}

/// The shared empty `NiFixedString` (`0109b220`): strings equal to it are not counted (a
/// string's count word sits 8 bytes before its text).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `NiRefObject`'s vtable and `ms_uiObjects` (`011f4400`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
/// The vtables stored by the program description (`010f0134`) and the cache (`010f0094`)
/// constructors.
const PROGRAM_DESC_VTABLE_010F0134: u32 = 0x010f_0134;
const PROGRAM_CACHE_VTABLE_010F0094: u32 = 0x010f_0094;
/// The RTTI records `GetPlatformSpecificCodeID` (`010f0054`) and `NiGPUProgramCache::GetRTTI`
/// (`01270920`) answer.
const PLATFORM_CODE_ID_010F0054: u32 = 0x010f_0054;
const GPU_PROGRAM_CACHE_RTTI_01270920: u32 = 0x0127_0920;
/// The sized free `00aa1460` (`cdecl`: block, size), the Ni allocator `00aa1070`/`00aa13e0`
/// (`cdecl`: size) and the free `00aa10f0` (`cdecl`).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `NiGPUProgramCache`'s base destructor (`00ee9a60`), base constructor (`00ee9bf0`, `RET
/// 0xc`), `SetWorkingDirectory` (`00ee8af0`, `RET 4`), `Load` (`00ee9350`), `LoadString`
/// (`00ee92d0`, `RET 0xc`: file, buffer, size), the program-name save (`00ee9230`, `RET 8`:
/// file, name; both `stdcall`, the ECX the callers set is not read) and the debug-file
/// switch (`00ee8d20`).
const BASE_DESTRUCT_00EE9A60: u32 = 0x00ee_9a60;
const BASE_CONSTRUCT_00EE9BF0: u32 = 0x00ee_9bf0;
const SET_WORKING_DIRECTORY: u32 = 0x00ee_8af0;
const CACHE_LOAD: u32 = 0x00ee_9350;
const LOAD_STRING: u32 = 0x00ee_92d0;
const SAVE_NAME_00EE9230: u32 = 0x00ee_9230;
const WRITES_DEBUG_FILES_00EE8D20: u32 = 0x00ee_8d20;
/// `NiFixedString` from a C string (`00a5b690`, `cdecl`), its in-place constructor (`00438170`,
/// `thiscall`, `RET 4`) and destructor (`004381b0`, `thiscall`), `EqualsNoCase` (`0048af90`,
/// `thiscall`, `RET 4`) and `NiStricmp` (`00aa1360`, `cdecl`).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIXED_STRING_EQUALS_NO_CASE: u32 = 0x0048_af90;
const NI_STRICMP: u32 = 0x00aa_1360;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, ...).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The string array's allocation (`00a63ae0`, `cdecl`: count) and free (`00a644a0`,
/// `cdecl`).
const STRING_ARRAY_NEW_00A63AE0: u32 = 0x00a6_3ae0;
const STRING_ARRAY_FREE_00A644A0: u32 = 0x00a6_44a0;
/// `_strcpy_s` (`00ec65a6`), the character search (`00ec6e30`, `cdecl`: text, character; the
/// last one, by its use), `NiPath::Standardize` (`00aa2a00`), the directory checks
/// (`00aa1ab0`, `00aa1bb0`) and `NiFile::GetFile` (`00aa1a20`: name, mode, buffer size).
const STRCPY_S: u32 = 0x00ec_65a6;
const FIND_CHARACTER_00EC6E30: u32 = 0x00ec_6e30;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const VALIDATE_DIRECTORY: u32 = 0x00aa_1ab0;
const CREATE_DIRECTORY_00AA1BB0: u32 = 0x00aa_1bb0;
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;
/// The D3DX buffer creation (import thunk `00ee6e3a`, `stdcall`: size, out buffer).
const CREATE_BUFFER_00EE6E3A: u32 = 0x00ee_6e3a;
/// `NiD3DShaderProgramFactory::GetInstance` (`00e92eb0`), `InsertVertexShaderIntoMap`
/// (`00e937f0`, `RET 4`), `InsertPixelShaderIntoMap` (`00e93820`, `RET 4`),
/// `CreateVertexShaderFromBuffer` (`00e93850`, `cdecl`, 10 words) and
/// `CreatePixelShaderFromBuffer` (`00e938e0`, `cdecl`, 7 words).
const FACTORY_INSTANCE: u32 = 0x00e9_2eb0;
const INSERT_VERTEX_SHADER: u32 = 0x00e9_37f0;
const INSERT_PIXEL_SHADER: u32 = 0x00e9_3820;
const CREATE_VERTEX_SHADER: u32 = 0x00e9_3850;
const CREATE_PIXEL_SHADER: u32 = 0x00e9_38e0;
/// `00a5f860` (`thiscall`, `RET 4`): `NiPointer` assignment from a raw pointer.
const NI_POINTER_SET_00A5F860: u32 = 0x00a5_f860;
/// The renderer pointer `LoadGPUProgram` uses (`0126fa3c`) and the two shader type names it
/// compares with (`0126fa48` vertex, `0126fa40` pixel; `char*` globals).
const RENDERER_0126FA3C: u32 = 0x0126_fa3c;
const VERTEX_TYPE_NAME: u32 = 0x0126_fa48;
const PIXEL_TYPE_NAME: u32 = 0x0126_fa40;
/// The string literal passed with every shader creation (`0105c2b0`).
const SHADER_TEXT_0105C2B0: u32 = 0x0105_c2b0;
/// The formats: the program file name (`010f0080` with `010f005c`), the vertex (`010f015c`)
/// and pixel (`010f013c`) names, and the source (`010f014c`).
const FILE_NAME_FORMAT: u32 = 0x010f_0080;
const FILE_NAME_SUFFIX: u32 = 0x010f_005c;
const VERTEX_NAME_FORMAT: u32 = 0x010f_015c;
const PIXEL_NAME_FORMAT: u32 = 0x010f_013c;
const SOURCE_FORMAT: u32 = 0x010f_014c;
/// The profile names the constructor compares with and adds (string literals of the exe).
const PROFILE_10A93EC: u32 = 0x010a_93ec;
const PROFILE_10A93E4: u32 = 0x010a_93e4;
const PROFILE_10F0078: u32 = 0x010f_0078;
const PROFILE_10A93DC: u32 = 0x010a_93dc;
const PROFILE_10F0070: u32 = 0x010f_0070;
const PROFILE_10F0068: u32 = 0x010f_0068;
const PROFILE_10A93D4: u32 = 0x010a_93d4;
const PROFILE_10F0060: u32 = 0x010f_0060;
const PROFILE_10A93CC: u32 = 0x010a_93cc;
const PROFILE_10A93C4: u32 = 0x010a_93c4;
const PROFILE_10A93BC: u32 = 0x010a_93bc;
const PROFILE_10A93B4: u32 = 0x010a_93b4;

/// Counts a fixed string up unless it is the empty one.
fn string_add_ref(e: &mut Engine, text: u32) {
    if text != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![text.wrapping_sub(8)]);
    }
}

/// Counts a fixed string down unless it is the empty one.
fn string_release(e: &mut Engine, text: u32) {
    if text != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![text.wrapping_sub(8)]);
    }
}

/// `NiFixedString` assignment as the code expands it: nothing when the slot holds `text`;
/// otherwise `text` counted up, the old one down, and `text` stored.
fn string_assign(e: &mut Engine, slot: u32, text: u32) {
    if e.mem.u32(slot) == text {
        return;
    }
    string_add_ref(e, text);
    let old = e.mem.u32(slot);
    string_release(e, old);
    e.mem.set_u32(slot, text);
}

/// Releases a reference-counted object: its count at +4 down, deleted (vtable +4) at 0.
fn release_object(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// Counts a reference-counted object up.
fn add_ref_object(e: &mut Engine, object: u32) {
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// A NiFile's stream call through the function pointer at `offset` of the file (+8 read, +0xc
/// write; `cdecl`: file, data, size, the address of the element size, 1), its count added to
/// the file's position (+4).
fn file_stream(e: &mut Engine, file: u32, offset: u32, data: u32, size: u32, element: u32) {
    let function = e.mem.u32(file.wrapping_add(offset));
    let moved = e
        .call(function, &args![file, data, size, element, 1u32])
        .u32();
    let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(file.wrapping_add(4), position);
}

// Translated from 00e8dd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::GetPlatformSpecificCodeID` (Xbox PDB; `ECX` not read): the address
/// `010f0054`.
pub fn ni_d3d_gpu_program_cache_get_platform_specific_code_id(_e: &mut Engine, _this: Ptr) -> u32 {
    PLATFORM_CODE_ID_010F0054
}

// Translated from 00e8dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::SaveDebugHLSLFile` (Xbox PDB; `thiscall`, `RET 8`: file name, text):
/// false unless debug files are written (`00ee8d20`). The file name's directory (copied into
/// a 0x104-byte local, `NiPath::Standardize`, cut at the last `/` or else `\`) must exist or
/// be made (`00aa1ab0`, `00aa1bb0`); the file opened for writing (`NiFile::GetFile` with 1 and
/// 0x8000) gets the text (without its terminator, the file's write function at +0xc, element
/// size 1) and is deleted (vtable +0 with 1). True when written.
// Translation incomplete: 00e8dd70 - the /GS cookie check (00ec408c) is not translated.
pub fn ni_d3d_gpu_program_cache_save_debug_hlsl_file(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    text: u32,
) -> bool {
    if !e.call(WRITES_DEBUG_FILES_00EE8D20, &args![this]).bool() {
        return false;
    }
    e.with_stack(0x108, |e, locals| {
        let element = locals.addr();
        let directory = element + 4;
        e.call(STRCPY_S, &args![directory, 0x104u32, name]);
        e.call(PATH_STANDARDIZE, &args![directory]);
        let mut cut = e
            .call(FIND_CHARACTER_00EC6E30, &args![directory, 0x2fu32])
            .u32();
        if cut == 0 {
            cut = e
                .call(FIND_CHARACTER_00EC6E30, &args![directory, 0x5cu32])
                .u32();
        }
        if cut != 0 {
            e.mem.set_u8(cut, 0);
        }
        if !e.call(VALIDATE_DIRECTORY, &args![directory]).bool()
            && !e.call(CREATE_DIRECTORY_00AA1BB0, &args![directory]).bool()
        {
            return false;
        }
        let file = e
            .call(NI_FILE_GET_FILE, &args![name, 1u32, 0x8000u32])
            .u32();
        if file == 0 {
            return false;
        }
        let length = e.mem.cstr(text).len() as u32;
        e.mem.set_u32(element, 1);
        file_stream(e, file, 0xc, text, length, element);
        e.vcall(file, 0, &args![1u32]);
        true
    })
}

// Translated from 00e8de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiPointer<NiMaterialResource>>::RemoveAll` (by its shape; `ECX` only): releases and
/// clears every slot below `m_uiAlloced`, then `m_uiUsed` is 0.
pub fn fn_00e8de70(e: &mut Engine, this: Ptr) {
    let set = this.addr();
    let mut index = 0u32;
    while index < e.mem.u32(set.wrapping_add(4)) {
        let slot = e.mem.u32(set).wrapping_add(index.wrapping_mul(4));
        let object = e.mem.u32(slot);
        if object != 0 {
            release_object(e, object);
            e.mem.set_u32(slot, 0);
        }
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(set.wrapping_add(8), 0);
}

// Translated from 00e8ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The program map's `SetValue` (by its shape; `thiscall`, `RET 0xc`, `this` not read): the
/// item's key (+4) becomes the fixed string at `key` (counted), and its value (+8) the
/// program `value`, passed by value: assigned (counted) and its argument released at the end.
pub fn fn_00e8ded0(e: &mut Engine, _this: Ptr, item: u32, key: u32, value: u32) {
    let text = e.mem.u32(key);
    if e.mem.u32(item.wrapping_add(4)) != text {
        string_add_ref(e, text);
        let old = e.mem.u32(item.wrapping_add(4));
        string_release(e, old);
        e.mem.set_u32(item.wrapping_add(4), text);
    }
    let old = e.mem.u32(item.wrapping_add(8));
    if old != value {
        if old != 0 {
            release_object(e, old);
        }
        e.mem.set_u32(item.wrapping_add(8), value);
        if value == 0 {
            return;
        }
        add_ref_object(e, value);
    }
    if value != 0 {
        release_object(e, value);
    }
}

// Translated from 00e8df70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::SaveGPUProgram` (Xbox PDB; `thiscall`, `RET 8`: file, program):
/// false without a program. The program's name (vtable +8, its string) goes out first
/// (`00ee9230`), then the code size (vtable +0x60, 4 bytes, element size 4) and the code
/// (vtable +0x64, element size 1) through the file's write function at +0xc; both argument
/// words serve as the locals they are in the exe. True.
pub fn ni_d3d_gpu_program_cache_save_gpu_program(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    program: u32,
) -> bool {
    if program == 0 {
        return false;
    }
    let name_slot = e.vcall(program, 8, &args![]).u32();
    let name = e.mem.u32(name_slot);
    // 00e8df97: `00ee9230` is stdcall (RET 8) and does not read the ECX (the cache) the
    // caller sets.
    let _ = this;
    e.call(SAVE_NAME_00EE9230, &args![file, name]);
    let size = e.vcall(program, 0x60, &args![]).u32();
    let code = e.vcall(program, 0x64, &args![]).u32();
    e.with_stack(8, |e, slots| {
        let element = slots.addr();
        let size_slot = element + 4;
        e.mem.set_u32(size_slot, size);
        e.mem.set_u32(element, 4);
        file_stream(e, file, 0xc, size_slot, 4, element);
        let size = e.mem.u32(size_slot);
        e.mem.set_u32(element, 1);
        file_stream(e, file, 0xc, code, size, element);
    });
    true
}

// Translated from 00e8e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::LoadGPUProgram` (Xbox PDB; `thiscall`, `RET 0x10`: file, program
/// name, program out, skip): clears the output; reads the shader type (`LoadString` into a
/// 0x100-byte local) and returns false unless it is the vertex or pixel name; reads the code
/// size (4 bytes, element size 4, the file's read function at +8). With `skip` the code is
/// skipped (file vtable +8 with the size) and true answered. Otherwise a D3DX buffer of that
/// size is made (`00ee6e3a`; false on failure), the code read into it (its vtable +0xc), and
/// the renderer at `0126fa3c` creates the shader (vtable +0x24 for a vertex shader with the
/// extra zero words, +0x28 for a pixel shader) from the buffer, the size, the name, `0105c2b0`
/// and this cache's vtable +0x54 answer; a created shader goes into the factory's map
/// (`00e937f0` / `00e93820`). The buffer is released (vtable +8); true.
// Translation incomplete: 00e8e000 - the /GS cookie check (00ec408c) is not translated.
pub fn ni_d3d_gpu_program_cache_load_gpu_program(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
    output: u32,
    skip: u8,
) -> bool {
    e.mem.set_u32(output, 0);
    e.with_stack(0x110, |e, locals| {
        let size_slot = locals.addr();
        let buffer_slot = size_slot + 4;
        let element = size_slot + 8;
        let kind = size_slot + 0x10;
        e.mem.set_u32(size_slot, 0);
        // 00e8e049: `00ee92d0` is stdcall (RET 0xc) and does not read the ECX (the cache)
        // the caller sets.
        e.call(LOAD_STRING, &args![file, kind, 0x100u32]);
        let vertex_name = e.global::<u32>(VERTEX_TYPE_NAME);
        let vertex = if e.call(NI_STRICMP, &args![kind, vertex_name]).u32() == 0 {
            true
        } else {
            let pixel_name = e.global::<u32>(PIXEL_TYPE_NAME);
            if e.call(NI_STRICMP, &args![kind, pixel_name]).u32() != 0 {
                return false;
            }
            false
        };
        e.mem.set_u32(element, 4);
        file_stream(e, file, 8, size_slot, 4, element);
        let size = e.mem.u32(size_slot);
        if skip != 0 {
            e.vcall(file, 8, &args![size]);
            return true;
        }
        e.mem.set_u32(buffer_slot, 0);
        if e.call(CREATE_BUFFER_00EE6E3A, &args![size, buffer_slot])
            .i32()
            < 0
        {
            return false;
        }
        let buffer = e.mem.u32(buffer_slot);
        let size = e.mem.u32(size_slot);
        let data = e.vcall(buffer, 0xc, &args![]).u32();
        e.mem.set_u32(element, 1);
        file_stream(e, file, 8, data, size, element);
        let renderer = e.global::<u32>(RENDERER_0126FA3C);
        let extra_slot = e.vcall(this.addr(), 0x54, &args![]).u32();
        let extra = e.mem.u32(extra_slot);
        let text = e.mem.u32(name);
        let buffer = e.mem.u32(buffer_slot);
        let size = e.mem.u32(size_slot);
        let shader = if vertex {
            e.vcall(
                renderer,
                0x24,
                &args![
                    buffer,
                    size,
                    text,
                    SHADER_TEXT_0105C2B0,
                    extra,
                    0u32,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32()
        } else {
            e.vcall(
                renderer,
                0x28,
                &args![buffer, size, text, SHADER_TEXT_0105C2B0, extra, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(output, shader);
        if shader != 0 {
            let factory = e.call(FACTORY_INSTANCE, &args![]).u32();
            let insert = if vertex {
                INSERT_VERTEX_SHADER
            } else {
                INSERT_PIXEL_SHADER
            };
            e.call(insert, &args![factory, shader]);
        }
        let buffer = e.mem.u32(buffer_slot);
        e.vcall(buffer, 8, &args![]);
        true
    })
}

// Translated from 00e8e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The program map's `SetAt` (by its shape; `thiscall`, `RET 8`: the address of the fixed
/// string key, the program passed by value): the bucket is the key modulo the bucket count
/// (+4) in the table (+8); an item with the key gets the value (`00e8ded0` with a counted
/// copy), otherwise a new item (vtable +4) does and is linked at the bucket's head, the count
/// (+0xc) going up. The value argument is released at the end.
pub fn fn_00e8e1d0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let text = e.mem.u32(key);
    let bucket = text % e.mem.u32(map.wrapping_add(4));
    let table = e.mem.u32(map.wrapping_add(8));
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 && text != e.mem.u32(item.wrapping_add(4)) {
        item = e.mem.u32(item);
    }
    if item != 0 {
        if value != 0 {
            add_ref_object(e, value);
        }
        fn_00e8ded0(e, this, item, key, value);
    } else {
        let item = e.vcall(map, 4, &args![]).u32();
        if value != 0 {
            add_ref_object(e, value);
        }
        fn_00e8ded0(e, this, item, key, value);
        let table = e.mem.u32(map.wrapping_add(8));
        let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        e.mem.set_u32(item, head);
        let table = e.mem.u32(map.wrapping_add(8));
        e.mem
            .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), item);
        let count = e.mem.u32(map.wrapping_add(0xc)).wrapping_add(1);
        e.mem.set_u32(map.wrapping_add(0xc), count);
    }
    if value != 0 {
        release_object(e, value);
    }
}

// Translated from 00e8e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fixed-string array's `Realloc` (by its shape; `thiscall`, `RET 4`; base +0, allocated
/// +4, used +8): nothing for the current size; otherwise a new array (`00a63ae0`; none for 0)
/// takes the used strings (assigned, counted), the old one is freed (`00a644a0`) and the
/// array keeps the new one and its size.
pub fn fn_00e8e290(e: &mut Engine, this: Ptr, new_max: u32) {
    let array = this.addr();
    if new_max == e.mem.u32(array.wrapping_add(4)) {
        return;
    }
    let base = if new_max != 0 {
        let base = e.call(STRING_ARRAY_NEW_00A63AE0, &args![new_max]).u32();
        let mut index = 0u32;
        while index < e.mem.u32(array.wrapping_add(8)) {
            let text = e
                .mem
                .u32(e.mem.u32(array).wrapping_add(index.wrapping_mul(4)));
            string_assign(e, base.wrapping_add(index.wrapping_mul(4)), text);
            index = index.wrapping_add(1);
        }
        base
    } else {
        0
    };
    let old = e.mem.u32(array);
    e.call(STRING_ARRAY_FREE_00A644A0, &args![old]);
    e.mem.set_u32(array, base);
    e.mem.set_u32(array.wrapping_add(4), new_max);
}

// Translated from 00e8e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fixed-string array's `Add` (by its shape; `thiscall`, `RET 4`: the address of the
/// string): a full array grows (`00e8e290` with twice its size, or 1); the string is assigned
/// to the next slot, the used count going up first.
pub fn fn_00e8e330(e: &mut Engine, this: Ptr, text: u32) {
    let array = this.addr();
    let alloced = e.mem.u32(array.wrapping_add(4));
    if e.mem.u32(array.wrapping_add(8)) == alloced {
        let grown = if alloced != 0 {
            alloced.wrapping_add(alloced)
        } else {
            1
        };
        fn_00e8e290(e, this, grown);
    }
    let used = e.mem.u32(array.wrapping_add(8));
    let slot = e.mem.u32(array).wrapping_add(used.wrapping_mul(4));
    e.mem.set_u32(array.wrapping_add(8), used.wrapping_add(1));
    let text = e.mem.u32(text);
    string_assign(e, slot, text);
}

/// How the constructor adds a profile name to the array at +0x28.
#[derive(Clone, Copy)]
enum ProfileAdd {
    /// `00a5b690` on the literal, added, its result released.
    Created,
    /// Constructed in place (`00438170`), added, the local released by hand.
    Constructed,
    /// Constructed in place, added, then its destructor (`004381b0`).
    Destructed,
}

/// Adds the profile names at +0x28 in order, each the way `how` says.
fn add_profiles(e: &mut Engine, this: Ptr, names: &[u32], how: ProfileAdd) {
    let array = Ptr::new(this.addr().wrapping_add(0x28));
    for name in names {
        e.with_stack(4, |e, local| match how {
            ProfileAdd::Created => {
                let text = e.call(FIXED_STRING_FROM_TEXT, &args![*name]).u32();
                e.mem.set_u32(local.addr(), text);
                fn_00e8e330(e, array, local.addr());
                string_release(e, text);
            }
            ProfileAdd::Constructed => {
                let at = e.call(FIXED_STRING_CONSTRUCT, &args![local, *name]).u32();
                fn_00e8e330(e, array, at);
                let text = e.mem.u32(local.addr());
                string_release(e, text);
            }
            ProfileAdd::Destructed => {
                let at = e.call(FIXED_STRING_CONSTRUCT, &args![local, *name]).u32();
                fn_00e8e330(e, array, at);
                e.call(FIXED_STRING_DESTRUCT, &args![local]);
            }
        });
    }
}

/// The constructor's `"is the profile <literal>"` test: the same pointer, or a non-null one
/// equal ignoring case (`NiStricmp`).
fn is_profile(e: &mut Engine, profile: u32, literal: u32) -> bool {
    profile == literal || (profile != 0 && e.call(NI_STRICMP, &args![profile, literal]).u32() == 0)
}

/// `NiFixedString::EqualsNoCase` on the profile at +0x24.
fn profile_equals(e: &mut Engine, this: Ptr, literal: u32) -> bool {
    e.call(
        FIXED_STRING_EQUALS_NO_CASE,
        &args![this.addr().wrapping_add(0x24), literal],
    )
    .bool()
}

// Translated from 00e8e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::NiD3DGPUProgramCache` (Xbox PDB; `thiscall`, `RET 0x24`): the base
/// constructor (`00ee9bf0` with the first, sixth and eighth words), the vtable `010f0094`,
/// the type (+0x3c, third word), a byte (+0x40, seventh word) and the shader path (+0x44, the
/// fifth word as a fixed string). The program file name (`NiSprintf` of `010f0080` with the
/// path, the fourth word's string and `010f005c`) goes to +0x18; the working directory is the
/// second word (`00ee8af0`, the empty string without one); the fourth word's string is the
/// profile (+0x24). The profile then decides which profile names the array at +0x28 holds:
/// for type 0 the vertex profiles up to it (`010a93ec`, `010a93e4`, `010f0078`, `010a93dc`),
/// for type 2 the pixel profiles up to it (`010f0070` down to `010a93b4`), each added in the
/// order and the way the exe does. With the ninth byte and a working directory the cache
/// loads (`00ee9350`). Answers `this`.
// Translation incomplete: 00e8e450 - the /GS cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_gpu_program_cache_ni_d3d_gpu_program_cache(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    directory: u32,
    kind: u32,
    profile: u32,
    path: u32,
    sixth: u32,
    flag: u8,
    eighth: u32,
    load: u8,
) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00EE9BF0, &args![this, first, sixth, eighth]);
    e.mem.set_u32(base, PROGRAM_CACHE_VTABLE_010F0094);
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(base + 0x44, empty);
    e.mem.set_u8(base + 0x40, flag);
    e.mem.set_u32(base + 0x3c, kind);
    let old = e.mem.u32(base + 0x44);
    if old != path {
        let text = e.call(FIXED_STRING_FROM_TEXT, &args![path]).u32();
        e.mem.set_u32(base + 0x44, text);
        string_release(e, old);
    }
    e.with_stack(0x108, |e, locals| {
        let directory_slot = locals.addr();
        let file_name = directory_slot + 4;
        let profile_text = e.mem.u32(profile);
        e.call(
            NI_SPRINTF,
            &args![
                file_name,
                0x104u32,
                FILE_NAME_FORMAT,
                path,
                profile_text,
                FILE_NAME_SUFFIX
            ],
        );
        let old = e.mem.u32(base + 0x18);
        if old != file_name {
            let text = e.call(FIXED_STRING_FROM_TEXT, &args![file_name]).u32();
            e.mem.set_u32(base + 0x18, text);
            string_release(e, old);
        }
        let working = if directory == 0 {
            e.global::<u32>(EMPTY_FIXED_STRING)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![directory]).u32()
        };
        e.mem.set_u32(directory_slot, working);
        e.call(SET_WORKING_DIRECTORY, &args![this, directory_slot]);
        let working = e.mem.u32(directory_slot);
        string_release(e, working);
    });
    let profile_text = e.mem.u32(profile);
    string_assign(e, base + 0x24, profile_text);
    if kind == 0 {
        let current = e.mem.u32(base + 0x24);
        if is_profile(e, current, PROFILE_10A93EC) {
            add_profiles(e, this, &[PROFILE_10A93EC], ProfileAdd::Created);
        } else if is_profile(e, e.mem.u32(base + 0x24), PROFILE_10A93E4) {
            add_profiles(
                e,
                this,
                &[PROFILE_10A93E4, PROFILE_10A93EC],
                ProfileAdd::Created,
            );
        } else if profile_equals(e, this, PROFILE_10F0078) {
            add_profiles(
                e,
                this,
                &[PROFILE_10F0078, PROFILE_10A93E4, PROFILE_10A93EC],
                ProfileAdd::Constructed,
            );
        } else if profile_equals(e, this, PROFILE_10A93DC) {
            add_profiles(
                e,
                this,
                &[
                    PROFILE_10A93DC,
                    PROFILE_10F0078,
                    PROFILE_10A93E4,
                    PROFILE_10A93EC,
                ],
                ProfileAdd::Destructed,
            );
        }
    } else if kind == 2 {
        // The pixel chain from each starting profile to `010f0070`.
        const FROM_10A93D4: [u32; 3] = [PROFILE_10A93D4, PROFILE_10F0068, PROFILE_10F0070];
        let current = e.mem.u32(base + 0x24);
        if is_profile(e, current, PROFILE_10F0070) {
            add_profiles(e, this, &[PROFILE_10F0070], ProfileAdd::Created);
        } else if profile_equals(e, this, PROFILE_10F0068) {
            add_profiles(
                e,
                this,
                &[PROFILE_10F0068, PROFILE_10F0070],
                ProfileAdd::Constructed,
            );
        } else {
            let start: Option<&[u32]> = if profile_equals(e, this, PROFILE_10A93D4) {
                Some(&[])
            } else if profile_equals(e, this, PROFILE_10F0060) {
                Some(&[PROFILE_10F0060])
            } else if profile_equals(e, this, PROFILE_10A93CC) {
                Some(&[PROFILE_10A93CC, PROFILE_10F0060])
            } else if profile_equals(e, this, PROFILE_10A93C4) {
                Some(&[PROFILE_10A93C4, PROFILE_10A93CC, PROFILE_10F0060])
            } else if profile_equals(e, this, PROFILE_10A93BC) {
                Some(&[PROFILE_10A93BC, PROFILE_10A93CC, PROFILE_10F0060])
            } else if profile_equals(e, this, PROFILE_10A93B4) {
                Some(&[
                    PROFILE_10A93B4,
                    PROFILE_10A93C4,
                    PROFILE_10A93CC,
                    PROFILE_10F0060,
                ])
            } else {
                None
            };
            if let Some(start) = start {
                add_profiles(e, this, start, ProfileAdd::Destructed);
                add_profiles(e, this, &FROM_10A93D4, ProfileAdd::Destructed);
            }
        }
    }
    if load != 0 && directory != 0 {
        e.call(CACHE_LOAD, &args![this]);
    }
    this
}

// Translated from 00e8ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::GetRTTI` (engine map; `ECX` not read): the RTTI record at `01270920`.
pub fn ni_gpu_program_cache_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    GPU_PROGRAM_CACHE_RTTI_01270920
}

// Translated from 00e8ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache` scalar deleting destructor (engine map; `RET 4`): the shader path
/// (+0x44) released, the base destructor (`00ee9a60`), then with flag bit 0 the 0x48 bytes
/// freed (`00aa1460`). Answers `this`.
pub fn ni_d3d_gpu_program_cache_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let path = e.mem.u32(this.addr().wrapping_add(0x44));
    string_release(e, path);
    e.call(BASE_DESTRUCT_00EE9A60, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00e8ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::NiGPUProgramDesc` constructor (by its vtable `010f0134`; `ECX` only):
/// the `NiRefObject` part (vtable `0101dce4`, count 0, `ms_uiObjects` counted up), then the
/// vtable and the words at +8 to +0x14 cleared. Answers `this`.
pub fn fn_00e8ead0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    e.mem.set_u32(base, PROGRAM_DESC_VTABLE_010F0134);
    for offset in [8u32, 0xc, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00e8eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::NiGPUProgramDesc` scalar deleting destructor (engine map; `RET 4`): the
/// resource set at +0xc emptied (`00e8de70`) and its array released (`00a47600`), the
/// program at +8 released, the `NiRefObject` vtable and `ms_uiObjects` counted down; with flag
/// bit 0 the 0x18 bytes freed (`00aa1460`). Answers `this`.
pub fn ni_gpu_program_desc_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    fn_00e8de70(e, Ptr::new(base + 0xc));
    let array = e.mem.u32(base + 0xc);
    e.call(POINTER_ARRAY_DELETE_00A47600, &args![array]);
    let program = e.mem.u32(base + 8);
    if program != 0 {
        release_object(e, program);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.call(
        IMPORT_INTERLOCKED_DECREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00e8eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGPUProgramCache::GenerateProgram` (Xbox PDB; `thiscall`, `RET 0xc`: name, code,
/// resource set): nothing (null) while generation is off (+0x3a). The source (`NiSprintf` of
/// `010f014c` with the program name and the code into a block of the code's length + 0x208,
/// `00aa1070`) is compiled for type 0 or 2 (+0x3c) by the factory
/// (`CreateVertexShaderFromBuffer` / `CreatePixelShaderFromBuffer`) under a name made from
/// the vtable +0x58 answer's string, the path (+0x44), `name` and the vtable +0x60 answer
/// (`010f015c` / `010f013c`), and held (`00a5f860`). With +0x40 the source is saved (vtable
/// +0x98). A program the cache accepts (vtable +0x7c with the set) gets a description
/// (0x18 bytes, `00e8ead0`) holding the set's resources and the program, optionally handed to
/// vtable +0x84 with the name's fixed string, and is mapped under that name (`00e8e1d0` on
/// +8). The block is freed (`00aa10f0`). Answers the program after releasing the held
/// reference, or null after emptying the set (`00e8de70`).
// Translation incomplete: 00e8eb80 - the /GS cookie check (00ec408c) is not translated.
pub fn ni_d3d_gpu_program_cache_generate_program(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    code: u32,
    resources: u32,
) -> u32 {
    let base = this.addr();
    if e.mem.u8(base + 0x3a) != 0 {
        return 0;
    }
    e.with_stack(0x110, |e, locals| {
        let key_slot = locals.addr();
        let program_slot = key_slot + 4;
        let program_name = key_slot + 0x10;
        e.mem.set_u32(program_slot, 0);
        let length = e.mem.cstr(code).len() as u32;
        let size = length.wrapping_add(0x208);
        let block = e.call(NI_ALLOCATE_00AA1070, &args![size]).u32();
        let kind = e.mem.u32(base + 0x3c);
        if kind == 0 || kind == 2 {
            let target = e.vcall(base, 0x58, &args![]).u32();
            let version = e.vcall(base, 0x60, &args![]).u32();
            let target_text = e.mem.u32(target);
            let path = e.mem.u32(base + 0x44);
            let format = if kind == 0 {
                VERTEX_NAME_FORMAT
            } else {
                PIXEL_NAME_FORMAT
            };
            e.call(
                NI_SPRINTF,
                &args![
                    program_name,
                    0x104u32,
                    format,
                    target_text,
                    path,
                    name,
                    version
                ],
            );
            e.call(
                NI_SPRINTF,
                &args![block, size, SOURCE_FORMAT, program_name, code],
            );
            let source_length = e.mem.cstr(block).len() as u32;
            let profile = e.mem.u32(base + 0x24);
            let program = if kind == 0 {
                e.call(
                    CREATE_VERTEX_SHADER,
                    &args![
                        block,
                        source_length,
                        program_name,
                        program_name,
                        SHADER_TEXT_0105C2B0,
                        profile,
                        0u32,
                        0u32,
                        0u32,
                        1u32
                    ],
                )
                .u32()
            } else {
                e.call(
                    CREATE_PIXEL_SHADER,
                    &args![
                        block,
                        source_length,
                        program_name,
                        program_name,
                        SHADER_TEXT_0105C2B0,
                        profile,
                        1u32
                    ],
                )
                .u32()
            };
            e.call(NI_POINTER_SET_00A5F860, &args![program_slot, program]);
        }
        if e.mem.u8(base + 0x40) != 0 {
            e.vcall(base, 0x98, &args![program_name, block]);
        }
        let mut program = e.mem.u32(program_slot);
        if program != 0 && e.vcall(base, 0x7c, &args![program, resources]).bool() {
            let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x18u32]).u32();
            let description = if block == 0 {
                0
            } else {
                fn_00e8ead0(e, Ptr::new(block)).addr()
            };
            let mut index = 0u32;
            while index < e.mem.u32(resources.wrapping_add(8)) {
                let element = e.mem.u32(resources).wrapping_add(index.wrapping_mul(4));
                ni_t_set_material_resource_add(e, Ptr::new(description.wrapping_add(0xc)), element);
                index = index.wrapping_add(1);
            }
            let held = e.mem.u32(description.wrapping_add(8));
            let program_now = e.mem.u32(program_slot);
            if held != program_now {
                if held != 0 {
                    release_object(e, held);
                }
                let program_now = e.mem.u32(program_slot);
                e.mem.set_u32(description.wrapping_add(8), program_now);
                add_ref_object(e, program_now);
            }
            let key = if name == 0 {
                e.global::<u32>(EMPTY_FIXED_STRING)
            } else {
                e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
            };
            e.mem.set_u32(key_slot, key);
            if e.mem.u8(base + 0x38) != 0 {
                e.vcall(base, 0x84, &args![description, key_slot]);
            }
            if description != 0 {
                add_ref_object(e, description);
            }
            fn_00e8e1d0(e, Ptr::new(base + 8), key_slot, description);
            let key = e.mem.u32(key_slot);
            string_release(e, key);
            program = e.mem.u32(program_slot);
        }
        e.call(NI_FREE_00AA10F0, &args![block]);
        if program != 0 {
            release_object(e, program);
            program
        } else {
            fn_00e8de70(e, Ptr::new(resources));
            0
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e8_dd60,
            ni_d3d_gpu_program_cache_get_platform_specific_code_id(Ptr) -> u32
        ),
        entry!(
            0x00e8_dd70,
            ni_d3d_gpu_program_cache_save_debug_hlsl_file(Ptr, u32, u32) -> bool
        ),
        entry!(0x00e8_de70, fn_00e8de70(Ptr)),
        entry!(0x00e8_ded0, fn_00e8ded0(Ptr, u32, u32, u32)),
        entry!(
            0x00e8_df70,
            ni_d3d_gpu_program_cache_save_gpu_program(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e8_e000,
            ni_d3d_gpu_program_cache_load_gpu_program(Ptr, u32, u32, u32, u8) -> bool
        ),
        entry!(0x00e8_e1d0, fn_00e8e1d0(Ptr, u32, u32)),
        entry!(0x00e8_e290, fn_00e8e290(Ptr, u32)),
        entry!(0x00e8_e330, fn_00e8e330(Ptr, u32)),
        entry!(
            0x00e8_e450,
            ni_d3d_gpu_program_cache_ni_d3d_gpu_program_cache(
                Ptr, u32, u32, u32, u32, u32, u32, u8, u32, u8
            ) -> Ptr
        ),
        entry!(0x00e8_ea10, ni_gpu_program_cache_get_rtti(Ptr) -> u32),
        entry!(
            0x00e8_ea20,
            ni_d3d_gpu_program_cache_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_ead0, fn_00e8ead0(Ptr) -> Ptr),
        entry!(
            0x00e8_eb10,
            ni_gpu_program_desc_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_eb80,
            ni_d3d_gpu_program_cache_generate_program(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x00e7_cd40, fn_00e7cd40(Ptr) -> u32),
        entry!(0x00e8_e3a0, ni_t_set_material_resource_realloc(Ptr, u32)),
        entry!(0x00e8_ea60, ni_t_set_material_resource_add(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0e40_0000;
    const DELETE: u32 = 0x0e41_0000;

    /// Interlocked counts on memory, the array allocator handing out zeroed blocks (after a
    /// count word, as `operator new[]`) and a recording delete.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(POINTER_ARRAY_NEW_00C1D790, |e, a| {
            let block = e.mem.alloc(4 + 4 * a[0]);
            e.mem.set_u32(block, a[0]);
            (block + 4).into_ret()
        });
        e.register(POINTER_ARRAY_DELETE_00A47600, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A set of `alloced` slots holding `objects`.
    fn make_set(e: &mut Engine, alloced: u32, objects: &[u32]) -> u32 {
        let base = e.mem.alloc(4 * alloced.max(1));
        for (index, object) in objects.iter().enumerate() {
            e.mem.set_u32(base + 4 * index as u32, *object);
        }
        let set = e.mem.alloc(0xc);
        e.mem.set_u32(set, base);
        e.mem.set_u32(set + 4, alloced);
        e.mem.set_u32(set + 8, objects.len() as u32);
        set
    }

    #[test]
    fn fn_00e7cd40_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e7_cd40, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x00e7_cd40, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn fn_00e8e3a0_moves_the_elements_to_a_new_array() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        let set = make_set(&mut e, 2, &[first, second]);
        let old = e.mem.u32(set);
        e.call(0x00e8_e3a0, &args![set, 5u32]);
        let base = e.mem.u32(set);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![5]]);
        assert_eq!(calls(&e, POINTER_ARRAY_DELETE_00A47600), vec![vec![old]]);
        assert_eq!(e.mem.u32(base), first);
        assert_eq!(e.mem.u32(base + 4), second);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(first + 4), 2);
        assert_eq!(e.mem.u32(set + 4), 5);
        assert_eq!(e.mem.u32(set + 8), 2);
    }

    #[test]
    fn fn_00e8e3a0_to_zero_frees_the_array_and_same_size_does_nothing() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let set = make_set(&mut e, 2, &[first]);
        e.call(0x00e8_e3a0, &args![set, 2u32]);
        assert!(calls(&e, POINTER_ARRAY_DELETE_00A47600).is_empty());
        let old = e.mem.u32(set);
        e.call(0x00e8_e3a0, &args![set, 0u32]);
        assert!(calls(&e, POINTER_ARRAY_NEW_00C1D790).is_empty());
        assert_eq!(calls(&e, POINTER_ARRAY_DELETE_00A47600), vec![vec![old]]);
        assert_eq!(e.mem.u32(set), 0);
        assert_eq!(e.mem.u32(set + 4), 0);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert_eq!(e.mem.u32(first + 4), 1);
    }

    #[test]
    fn fn_00e8ea60_appends_and_grows_when_full() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let set = make_set(&mut e, 1, &[first]);
        let added = object(&mut e, 0);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, added);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![2]]);
        let base = e.mem.u32(set);
        assert_eq!(e.mem.u32(base), first);
        assert_eq!(e.mem.u32(base + 4), added);
        assert_eq!(e.mem.u32(added + 4), 1);
        assert_eq!(e.mem.u32(set + 4), 2);
        assert_eq!(e.mem.u32(set + 8), 2);
    }

    #[test]
    fn fn_00e8ea60_starts_an_empty_set_at_one_and_adds_null() {
        let mut e = engine();
        let set = make_set(&mut e, 0, &[]);
        let pointer = e.mem.alloc(4);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790), vec![vec![1]]);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // Room left: no growth, the slot's old object released.
        let stale = object(&mut e, 1);
        let set = make_set(&mut e, 4, &[]);
        let base = e.mem.u32(set);
        e.mem.set_u32(base, stale);
        let added = object(&mut e, 0);
        e.mem.set_u32(pointer, added);
        e.call(0x00e8_ea60, &args![set, pointer]);
        assert_eq!(calls(&e, POINTER_ARRAY_NEW_00C1D790).len(), 1);
        assert_eq!(calls(&e, DELETE), vec![vec![stale]]);
        assert_eq!(e.mem.u32(base), added);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OBJECT_VTABLE: u32 = 0x0e50_0000;
    const OBJECT_DELETE: u32 = 0x0e51_0004;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Interlocked counts on memory, reference-counted objects deleted through a recording
    /// slot, the empty string and `ms_uiObjects` mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(OBJECT_VTABLE, &[0, OBJECT_DELETE]);
        e.register(OBJECT_DELETE, |_, _| Ret::default());
        e.map(0x0109_b000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        let empty = fixed_string(&mut e, b"");
        e.set_global::<u32>(EMPTY_FIXED_STRING, empty);
        e.call_log = Some(vec![]);
        e
    }

    /// A fixed string: a count word 8 bytes before the text.
    fn fixed_string(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(8 + text.len() as u32 + 1);
        e.mem.set_cstr(block + 8, text);
        block + 8
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let o = e.mem.alloc(0x10);
        e.mem.set_u32(o, OBJECT_VTABLE);
        e.mem.set_u32(o + 4, count);
        o
    }

    #[test]
    fn the_constant_answers() {
        let mut e = engine();
        assert_eq!(e.call(0x00e8_dd60, &args![0u32]).u32(), 0x010f_0054);
        assert_eq!(e.call(0x00e8_ea10, &args![0u32]).u32(), 0x0127_0920);
    }

    #[test]
    fn save_debug_hlsl_file_writes_the_text() {
        let mut e = engine();
        e.register(WRITES_DEBUG_FILES_00EE8D20, |_, _| 0u32.into_ret());
        let name = e.mem.alloc(0x20);
        e.mem.set_cstr(name, b"shaders/debug/a.hlsl");
        let text = e.mem.alloc(0x10);
        e.mem.set_cstr(text, b"float4");
        assert!(!e.call(0x00e8_dd70, &args![0x10u32, name, text]).bool());
        e.register(WRITES_DEBUG_FILES_00EE8D20, |_, _| 1u32.into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        e.register(FIND_CHARACTER_00EC6E30, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|c| u32::from(*c) == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        let directories: Rc<RefCell<Vec<Vec<u8>>>> = Rc::default();
        let log = directories.clone();
        e.register_double(VALIDATE_DIRECTORY, move |e, a| {
            log.borrow_mut().push(e.mem.cstr(a[0]));
            1u32.into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file, OBJECT_VTABLE);
        e.mem.set_u32(file + 0xc, 0x0e52_0000);
        e.register(0x0e52_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            a[2].into_ret()
        });
        e.mem.set_u32(OBJECT_VTABLE, 0x0e51_0000);
        e.register(0x0e51_0000, |_, _| Ret::default());
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        assert!(e.call(0x00e8_dd70, &args![0x10u32, name, text]).bool());
        assert_eq!(*directories.borrow(), vec![b"shaders/debug".to_vec()]);
        assert_eq!(calls(&e, NI_FILE_GET_FILE), vec![vec![name, 1, 0x8000]]);
        assert_eq!(calls(&e, 0x0e52_0000)[0][..3], [file, text, 6]);
        assert_eq!(e.mem.u32(file + 4), 6);
        assert_eq!(calls(&e, 0x0e51_0000), vec![vec![file, 1]]);
        // A missing directory that cannot be made: false.
        e.register(VALIDATE_DIRECTORY, |_, _| 0u32.into_ret());
        e.register(CREATE_DIRECTORY_00AA1BB0, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e8_dd70, &args![0x10u32, name, text]).bool());
    }

    #[test]
    fn remove_all_releases_every_slot() {
        let mut e = engine();
        let last = object(&mut e, 1);
        let kept = object(&mut e, 2);
        let base = e.mem.alloc(0xc);
        e.mem.set_u32(base, last);
        e.mem.set_u32(base + 8, kept);
        let set = e.mem.alloc(0xc);
        e.mem.set_u32(set, base);
        e.mem.set_u32(set + 4, 3);
        e.mem.set_u32(set + 8, 3);
        e.call(0x00e8_de70, &args![set]);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![last]]);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(e.mem.bytes(base, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(set + 8), 0);
    }

    #[test]
    fn set_value_stores_the_key_and_value() {
        let mut e = engine();
        let old_key = fixed_string(&mut e, b"old");
        e.mem.set_u32(old_key - 8, 1);
        let new_key = fixed_string(&mut e, b"new");
        let key_slot = e.mem.alloc(4);
        e.mem.set_u32(key_slot, new_key);
        let old_value = object(&mut e, 1);
        let value = object(&mut e, 1);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, old_key);
        e.mem.set_u32(item + 8, old_value);
        e.call(0x00e8_ded0, &args![0u32, item, key_slot, value]);
        assert_eq!(e.mem.u32(item + 4), new_key);
        assert_eq!(e.mem.u32(new_key - 8), 1);
        assert_eq!(e.mem.u32(old_key - 8), 0);
        assert_eq!(e.mem.u32(item + 8), value);
        // Counted for the item, released for the argument.
        assert_eq!(e.mem.u32(value + 4), 1);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![old_value]]);
        // The same value again: only the argument's release.
        e.call(0x00e8_ded0, &args![0u32, item, key_slot, value]);
        assert_eq!(e.mem.u32(value + 4), 0);
    }

    #[test]
    fn save_gpu_program_writes_name_size_and_code() {
        let mut e = engine();
        assert!(!e.call(0x00e8_df70, &args![0x10u32, 0x20u32, 0u32]).bool());
        let program = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x1a];
        slots[2] = 0x0e53_0008;
        slots[0x18] = 0x0e53_0060;
        slots[0x19] = 0x0e53_0064;
        e.put_vtable(0x0e54_0000, &slots);
        e.mem.set_u32(program, 0x0e54_0000);
        let name_slot = e.mem.alloc(4);
        e.mem.set_u32(name_slot, 0x7777);
        e.register_double(0x0e53_0008, move |_, _| name_slot.into_ret());
        e.register(0x0e53_0060, |_, _| 0x30u32.into_ret());
        e.register(0x0e53_0064, |_, _| 0x9900u32.into_ret());
        e.register(SAVE_NAME_00EE9230, |_, _| Ret::default());
        let writes: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let log = writes.clone();
        e.register_double(0x0e52_0000, move |e, a| {
            log.borrow_mut()
                .push(vec![a[1], a[2], e.mem.u32(a[3]), a[4]]);
            a[2].into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 0xc, 0x0e52_0000);
        assert!(e.call(0x00e8_df70, &args![0x10u32, file, program]).bool());
        assert_eq!(calls(&e, SAVE_NAME_00EE9230), vec![vec![file, 0x7777]]);
        let writes = writes.borrow();
        assert_eq!(writes[0][1..], [4, 4, 1]);
        assert_eq!(writes[1], vec![0x9900, 0x30, 1, 1]);
        assert_eq!(e.mem.u32(file + 4), 0x34);
    }

    /// A cache object whose vtable slot +0x54 answers the address of the word 0x5454.
    fn cache(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x28];
        slots[0x54 / 4] = 0x0e55_0054;
        slots[0x58 / 4] = 0x0e55_0058;
        slots[0x60 / 4] = 0x0e55_0060;
        slots[0x7c / 4] = 0x0e55_007c;
        slots[0x84 / 4] = 0x0e55_0084;
        slots[0x98 / 4] = 0x0e55_0098;
        e.put_vtable(0x0e56_0000, &slots);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x5454);
        e.register_double(0x0e55_0054, move |_, _| word.into_ret());
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, 0x0e56_0000);
        this
    }

    fn reading_file(e: &mut Engine, kind: &'static [u8]) -> u32 {
        e.map(0x0126_f000, 0x1000);
        let vertex = e.mem.alloc(8);
        e.mem.set_cstr(vertex, b"vs");
        let pixel = e.mem.alloc(8);
        e.mem.set_cstr(pixel, b"ps");
        e.set_global::<u32>(VERTEX_TYPE_NAME, vertex);
        e.set_global::<u32>(PIXEL_TYPE_NAME, pixel);
        e.register_double(LOAD_STRING, move |e, a| {
            assert_eq!((a.len(), a[2]), (3, 0x100));
            e.mem.set_cstr(a[1], kind);
            Ret::default()
        });
        e.register(NI_STRICMP, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file, OBJECT_VTABLE);
        e.mem.set_u32(file + 8, 0x0e57_0000);
        e.register(0x0e57_0000, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x40);
            }
            a[2].into_ret()
        });
        file
    }

    #[test]
    fn load_gpu_program_creates_and_registers_a_vertex_shader() {
        let mut e = engine();
        let this = cache(&mut e);
        let file = reading_file(&mut e, b"vs");
        let buffer = e.mem.alloc(4);
        e.put_vtable(0x0e58_0000, &[0, 0, 0x0e58_0008, 0x0e58_000c]);
        e.mem.set_u32(buffer, 0x0e58_0000);
        e.register(0x0e58_0008, |_, _| Ret::default());
        e.register(0x0e58_000c, |_, _| 0xdddd_0000u32.into_ret());
        e.register_double(CREATE_BUFFER_00EE6E3A, move |e, a| {
            e.mem.set_u32(a[1], buffer);
            0u32.into_ret()
        });
        let renderer = e.mem.alloc(4);
        let mut slots = vec![0u32; 0xb];
        slots[9] = 0x0e59_0024;
        slots[10] = 0x0e59_0028;
        e.put_vtable(0x0e5a_0000, &slots);
        e.mem.set_u32(renderer, 0x0e5a_0000);
        e.set_global::<u32>(RENDERER_0126FA3C, renderer);
        e.register(0x0e59_0024, |_, _| 0x5ade_0000u32.into_ret());
        e.register(FACTORY_INSTANCE, |_, _| 0xfac0u32.into_ret());
        e.register(INSERT_VERTEX_SHADER, |_, _| Ret::default());
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x4e4e);
        let output = e.mem.alloc(4);
        e.mem.set_u32(output, 0xffff);
        let ok = e.call(0x00e8_e000, &args![this, file, name, output, 0u32]);
        assert!(ok.bool());
        assert_eq!(e.mem.u32(output), 0x5ade_0000);
        assert_eq!(calls(&e, CREATE_BUFFER_00EE6E3A)[0][0], 0x40);
        assert_eq!(
            calls(&e, 0x0e59_0024),
            vec![vec![
                renderer,
                buffer,
                0x40,
                0x4e4e,
                SHADER_TEXT_0105C2B0,
                0x5454,
                0,
                0,
                0,
                0
            ]]
        );
        assert_eq!(
            calls(&e, INSERT_VERTEX_SHADER),
            vec![vec![0xfac0, 0x5ade_0000]]
        );
        assert_eq!(calls(&e, 0x0e58_0008), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(file + 4), 0x44);
    }

    #[test]
    fn load_gpu_program_skips_and_refuses_other_types() {
        let mut e = engine();
        let this = cache(&mut e);
        let file = reading_file(&mut e, b"ps");
        let skip = 0x0e5b_0008;
        e.put_vtable(0x0e5c_0000, &[0, 0, skip]);
        e.mem.set_u32(file, 0x0e5c_0000);
        e.register(skip, |_, _| Ret::default());
        let output = e.mem.alloc(4);
        assert!(e
            .call(0x00e8_e000, &args![this, file, 0u32, output, 1u32])
            .bool());
        assert_eq!(calls(&e, skip), vec![vec![file, 0x40]]);
        let other = reading_file(&mut e, b"gs");
        assert!(!e
            .call(0x00e8_e000, &args![this, other, 0u32, output, 0u32])
            .bool());
    }

    /// A map of `buckets` empty buckets whose new item (vtable +4) is a fresh 0xc block.
    fn map(e: &mut Engine, buckets: u32) -> u32 {
        e.put_vtable(0x0e5d_0000, &[0, 0x0e5d_1004]);
        // A new item's key is the empty string.
        e.register(0x0e5d_1004, |e, _| {
            let item = e.mem.alloc(0xc);
            let empty = e.global::<u32>(EMPTY_FIXED_STRING);
            e.mem.set_u32(item + 4, empty);
            item.into_ret()
        });
        let table = e.mem.alloc(4 * buckets);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m, 0x0e5d_0000);
        e.mem.set_u32(m + 4, buckets);
        e.mem.set_u32(m + 8, table);
        m
    }

    #[test]
    fn set_at_adds_then_updates() {
        let mut e = engine();
        let m = map(&mut e, 4);
        let key = fixed_string(&mut e, b"k");
        let key_slot = e.mem.alloc(4);
        e.mem.set_u32(key_slot, key);
        let value = object(&mut e, 1);
        e.call(0x00e8_e1d0, &args![m, key_slot, value]);
        let table = e.mem.u32(m + 8);
        let item = e.mem.u32(table + 4 * (key % 4));
        assert_ne!(item, 0);
        assert_eq!(e.mem.u32(item + 4), key);
        assert_eq!(e.mem.u32(item + 8), value);
        assert_eq!(e.mem.u32(m + 0xc), 1);
        // The item's reference, less the argument's own release.
        assert_eq!(e.mem.u32(value + 4), 1);
        let other = object(&mut e, 1);
        e.call(0x00e8_e1d0, &args![m, key_slot, other]);
        assert_eq!(e.mem.u32(item + 8), other);
        assert_eq!(e.mem.u32(m + 0xc), 1);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![value]]);
    }

    fn string_array_engine() -> Engine {
        let mut e = engine();
        // New arrays hold empty strings.
        e.register(STRING_ARRAY_NEW_00A63AE0, |e, a| {
            let base = e.mem.alloc(4 * a[0]);
            let empty = e.global::<u32>(EMPTY_FIXED_STRING);
            for i in 0..a[0] {
                e.mem.set_u32(base + 4 * i, empty);
            }
            base.into_ret()
        });
        e.register(STRING_ARRAY_FREE_00A644A0, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_string_array_grows_and_counts() {
        let mut e = string_array_engine();
        let array = e.mem.alloc(0xc);
        let first = fixed_string(&mut e, b"a");
        let second = fixed_string(&mut e, b"b");
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, first);
        e.call(0x00e8_e330, &args![array, slot]);
        e.mem.set_u32(slot, second);
        e.call(0x00e8_e330, &args![array, slot]);
        assert_eq!(calls(&e, STRING_ARRAY_NEW_00A63AE0), vec![vec![1], vec![2]]);
        let base = e.mem.u32(array);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 4)), (first, second));
        // Counted once by the add and once by the copy into the grown array.
        assert_eq!(e.mem.u32(first - 8), 2);
        assert_eq!(e.mem.u32(second - 8), 1);
        assert_eq!((e.mem.u32(array + 4), e.mem.u32(array + 8)), (2, 2));
        // Same size: nothing; zero: freed, no new array.
        e.call(0x00e8_e290, &args![array, 2u32]);
        e.call(0x00e8_e290, &args![array, 0u32]);
        assert_eq!(calls(&e, STRING_ARRAY_NEW_00A63AE0).len(), 2);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(
            calls(&e, STRING_ARRAY_FREE_00A644A0).last().unwrap(),
            &vec![base]
        );
    }

    /// Doubles for the constructor: strings are their literal addresses (mapped), equality
    /// is by address.
    fn constructor_engine() -> (Engine, u32) {
        let mut e = string_array_engine();
        e.map(0x010a_9000, 0x1000);
        e.map(0x010f_0000, 0x1000);
        e.map(0x4000, 0x2000);
        for a in [NI_SPRINTF, SET_WORKING_DIRECTORY, CACHE_LOAD] {
            e.register(a, |_, _| Ret::default());
        }
        // The base constructor leaves the file name and profile empty strings.
        e.register(BASE_CONSTRUCT_00EE9BF0, |e, a| {
            let empty = e.global::<u32>(EMPTY_FIXED_STRING);
            e.mem.set_u32(a[0] + 0x18, empty);
            e.mem.set_u32(a[0] + 0x24, empty);
            Ret::default()
        });
        e.register(FIXED_STRING_FROM_TEXT, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(NI_STRICMP, |_, a| u32::from(a[0] != a[1]).into_ret());
        e.register(FIXED_STRING_EQUALS_NO_CASE, |e, a| {
            (e.mem.u32(a[0]) == a[1]).into_ret()
        });
        let this = e.mem.alloc(0x48);
        (e, this)
    }

    fn profiles(e: &Engine, this: u32) -> Vec<u32> {
        let base = e.mem.u32(this + 0x28);
        let used = e.mem.u32(this + 0x30);
        (0..used).map(|i| e.mem.u32(base + 4 * i)).collect()
    }

    fn construct(e: &mut Engine, this: u32, kind: u32, profile: u32, directory: u32, load: u32) {
        let profile_slot = e.mem.alloc(4);
        e.mem.set_u32(profile_slot, profile);
        let r = e.call(
            0x00e8_e450,
            &args![
                this,
                1u32,
                directory,
                kind,
                profile_slot,
                0x5000u32,
                6u32,
                0x107u32,
                8u32,
                load
            ],
        );
        assert_eq!(r.u32(), this);
    }

    #[test]
    fn the_constructor_lists_vertex_profiles() {
        let (mut e, this) = constructor_engine();

        construct(&mut e, this, 0, PROFILE_10A93E4, 0, 1);
        assert_eq!(e.mem.u32(this), PROGRAM_CACHE_VTABLE_010F0094);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u8(this + 0x40), 7);
        assert_eq!(e.mem.u32(this + 0x44), 0x5000);
        assert_eq!(e.mem.u32(this + 0x24), PROFILE_10A93E4);
        assert_eq!(profiles(&e, this), vec![PROFILE_10A93E4, PROFILE_10A93EC]);
        assert_eq!(
            calls(&e, BASE_CONSTRUCT_00EE9BF0),
            vec![vec![this, 1, 6, 8]]
        );
        // No working directory: no load.
        assert!(calls(&e, CACHE_LOAD).is_empty());
        let (mut e, this) = constructor_engine();

        construct(&mut e, this, 0, PROFILE_10A93DC, 0x5000, 1);
        assert_eq!(
            profiles(&e, this),
            vec![
                PROFILE_10A93DC,
                PROFILE_10F0078,
                PROFILE_10A93E4,
                PROFILE_10A93EC
            ]
        );
        assert_eq!(calls(&e, FIXED_STRING_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, CACHE_LOAD), vec![vec![this]]);
    }

    #[test]
    fn the_constructor_lists_pixel_profiles() {
        for (profile, expected) in [
            (PROFILE_10F0070, vec![PROFILE_10F0070]),
            (PROFILE_10F0068, vec![PROFILE_10F0068, PROFILE_10F0070]),
            (
                PROFILE_10A93BC,
                vec![
                    PROFILE_10A93BC,
                    PROFILE_10A93CC,
                    PROFILE_10F0060,
                    PROFILE_10A93D4,
                    PROFILE_10F0068,
                    PROFILE_10F0070,
                ],
            ),
            (
                PROFILE_10A93B4,
                vec![
                    PROFILE_10A93B4,
                    PROFILE_10A93C4,
                    PROFILE_10A93CC,
                    PROFILE_10F0060,
                    PROFILE_10A93D4,
                    PROFILE_10F0068,
                    PROFILE_10F0070,
                ],
            ),
            (0x010a_9100, vec![]),
        ] {
            let (mut e, this) = constructor_engine();
            construct(&mut e, this, 2, profile, 0, 0);
            assert_eq!(profiles(&e, this), expected);
        }
    }

    #[test]
    fn the_cache_destructor_and_the_description() {
        let mut e = engine();
        e.register(BASE_DESTRUCT_00EE9A60, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        e.register(POINTER_ARRAY_DELETE_00A47600, |_, _| Ret::default());
        let path = fixed_string(&mut e, b"p");
        e.mem.set_u32(path - 8, 2);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x44, path);
        assert_eq!(e.call(0x00e8_ea20, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(path - 8), 1);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x48]]);
        let desc = e.mem.alloc(0x18);
        e.mem.write(desc, &[0xee; 0x18]);
        assert_eq!(e.call(0x00e8_ead0, &args![desc]).u32(), desc);
        assert_eq!(e.mem.u32(desc), PROGRAM_DESC_VTABLE_010F0134);
        assert_eq!(e.mem.bytes(desc + 4, 0x14), vec![0; 0x14]);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT_011F4400), 1);
        let program = object(&mut e, 1);
        e.mem.set_u32(desc + 8, program);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_eb10, &args![desc, 0u32]).u32(), desc);
        assert_eq!(e.mem.u32(desc), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![program]]);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT_011F4400), 0);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
    }

    #[test]
    fn generate_program_maps_an_accepted_vertex_program() {
        let mut e = engine();
        let this = cache(&mut e);
        e.mem.set_u32(this + 0x3c, 0);
        e.mem.set_u8(this + 0x38, 1);
        // The program map is embedded at +8.
        let m = map(&mut e, 2);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(m + offset);
            e.mem.set_u32(this + 8 + offset, word);
        }
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, 0x7474);
        e.register_double(0x0e55_0058, move |_, _| target.into_ret());
        e.register(0x0e55_0060, |_, _| 3u32.into_ret());
        e.register(0x0e55_007c, |_, _| 1u32.into_ret());
        e.register(0x0e55_0084, |_, _| Ret::default());
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(NI_SPRINTF, |e, a| {
            e.mem.set_cstr(a[0], b"src");
            Ret::default()
        });
        let program = object(&mut e, 0);
        e.register_double(CREATE_VERTEX_SHADER, move |_, _| program.into_ret());
        e.register(NI_POINTER_SET_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            add_ref_object(e, a[1]);
            Ret::default()
        });
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x4b00u32.into_ret());
        e.map(0x4000, 0x1000);
        e.register(POINTER_ARRAY_NEW_00C1D790, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        e.register(POINTER_ARRAY_DELETE_00A47600, |_, _| Ret::default());
        let resource = object(&mut e, 1);
        let base = e.mem.alloc(4);
        e.mem.set_u32(base, resource);
        let resources = e.mem.alloc(0xc);
        e.mem.set_u32(resources, base);
        e.mem.set_u32(resources + 4, 1);
        e.mem.set_u32(resources + 8, 1);
        let name = e.mem.alloc(4);
        let code = e.mem.alloc(8);
        e.mem.set_cstr(code, b"code");
        let r = e
            .call(0x00e8_eb80, &args![this, name, code, resources])
            .u32();
        assert_eq!(r, program);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070), vec![vec![4 + 0x208]]);
        let created = calls(&e, CREATE_VERTEX_SHADER)[0].clone();
        assert_eq!(created[1], 3);
        assert_eq!(created[4..], [SHADER_TEXT_0105C2B0, 0, 0, 0, 0, 1]);
        let table = e.mem.u32(this + 0x10);
        // The key 0x4b00 hashes to bucket 0 (key % 2 buckets).
        let item = e.mem.u32(table);
        let description = e.mem.u32(item + 8);
        assert_eq!(e.mem.u32(description + 8), program);
        assert_eq!(e.mem.u32(item + 4), 0x4b00);
        // The description holds the program; the map holds the description.
        assert_eq!(e.mem.u32(program + 4), 1);
        assert_eq!(e.mem.u32(description + 4), 1);
        assert_eq!(e.mem.u32(resource + 4), 2);
        assert_eq!(calls(&e, 0x0e55_0084).len(), 1);
    }

    #[test]
    fn generate_program_without_a_program_empties_the_set() {
        let mut e = engine();
        let this = cache(&mut e);
        e.mem.set_u32(this + 0x3c, 1);
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        let resource = object(&mut e, 1);
        let base = e.mem.alloc(4);
        e.mem.set_u32(base, resource);
        let resources = e.mem.alloc(0xc);
        e.mem.set_u32(resources, base);
        e.mem.set_u32(resources + 4, 1);
        e.mem.set_u32(resources + 8, 1);
        let code = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00e8_eb80, &args![this, 0u32, code, resources])
                .u32(),
            0
        );
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![resource]]);
        assert_eq!(e.mem.u32(resources + 8), 0);
        // Generation off: nothing at all.
        e.mem.set_u8(this + 0x3a, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_eb80, &args![this, 0u32, code, resources])
                .u32(),
            0
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
