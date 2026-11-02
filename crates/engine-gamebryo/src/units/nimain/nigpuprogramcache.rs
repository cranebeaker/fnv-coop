//! `NiMain/nigpuprogramcache.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, assign_ref_counted, interlocked_decrement, interlocked_increment,
    release_fixed_string, ALLOCATE, EMPTY_FIXED_STRING, GET_FIXED_STRING,
};

// Translated from 00ee9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ee9a60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x3c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_gpu_program_cache_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ee_9a60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

/// The slot `00aa1ab0` jumps through.
const POINTER_CELL_00AA1AB0: u32 = 0x011a_c230;

// Translated from 00aa1ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011ac230]`: `NiGPUProgramCache::ValidateDirectory` (engine map). The slot holds a function pointer (`0x011ac230` is a data cell, not an import slot); the 1 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 00ee8d2f (`ADD ESP,0x4`); the cell starts out holding `00aa1670`, a function that returns `AL`.
pub fn fn_00aa1ab0(e: &mut Engine, path: Ptr) -> bool {
    let target = e.global::<u32>(POINTER_CELL_00AA1AB0);
    e.call(target, &args![path]).bool()
}

/// Vtables (by their RTTI): `NiRefObject` (`0101dce4`), `NiGPUProgramCache` (`011013b4`),
/// `NiGPUProgramCache::NiGPUProgramDesc` (`010f0134`), the program map's
/// `NiTFixedStringMapBase<..>` (`01101378`) and `NiTFixedStringMap<..>` (`01101388`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_GPU_PROGRAM_CACHE_VTABLE: u32 = 0x0110_13b4;
const PROGRAM_DESC_VTABLE: u32 = 0x010f_0134;
const MAP_BASE_VTABLE: u32 = 0x0110_1378;
const MAP_VTABLE: u32 = 0x0110_1388;
/// `NiRefObject::ms_uiObjects` (the counted object total).
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The memory manager object, its allocate (`00aa3e40`) and free (`00aa4060`), both
/// `thiscall` with one word; the array allocation and free (`00aa1070`, `00aa10f0`,
/// `cdecl`); `_memset` (`00ec61c0`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;
const FREE_ARRAY: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x00ec_61c0;
/// The program map's `RemoveAll` (`00aa01f0`, `thiscall`; the engine map names a folded
/// body of another instance).
const MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// `00a644a0` (unnamed, `cdecl`, one word): what the destructor hands the valid-target
/// set's element array to.
const FREE_STRING_ARRAY_00A644A0: u32 = 0x00a6_44a0;
/// `00a63ae0` (unit `nistream.cpp`, `cdecl`, a count): the fixed-string array allocation the
/// constructor makes for the valid-target set.
const NEW_STRING_ARRAY_00A63AE0: u32 = 0x00a6_3ae0;
/// `NiPath::IsRelative` (`00aa2cc0`, `cdecl`, a path), `NiPath::GetCurrentWorkingDirectory`
/// (`00aa2d10`, `cdecl`: buffer, size) and `NiPath::ConvertToAbsolute` (`00aa2ba0`, `cdecl`:
/// out, size, path, base).
const PATH_IS_RELATIVE: u32 = 0x00aa_2cc0;
const PATH_GET_CURRENT_WORKING_DIRECTORY: u32 = 0x00aa_2d10;
const PATH_CONVERT_TO_ABSOLUTE: u32 = 0x00aa_2ba0;
/// `00aa1bb0` (unnamed, `cdecl`, a path, answering `AL`): what is tried when the working
/// directory does not validate.
const CREATE_DIRECTORY_00AA1BB0: u32 = 0x00aa_1bb0;
/// `00a2e750` (`thiscall` on a fixed string, a C string; the engine map's unit is a folded
/// one): the fixed string's assignment from a C string.
const FIXED_STRING_ASSIGN_00A2E750: u32 = 0x00a2_e750;
/// `NiFile::Access` (`00aa1a30`, `cdecl`: path, mode) and `NiFile::GetFile` (`00aa1a20`,
/// `cdecl`: path, mode, buffer size).
const FILE_ACCESS: u32 = 0x00aa_1a30;
const FILE_GET_FILE: u32 = 0x00aa_1a20;
/// `NiStricmp` (`00aa1360`, `cdecl`: two C strings).
const NI_STRICMP: u32 = 0x00aa_1360;
/// The stream loads `00a8d8b0` (`cdecl`: stream, buffer, count of bytes), `00a57580`
/// (`cdecl`: stream, address of a byte) and `00a575b0` (`cdecl`: stream, address of a
/// word), and the header save `00e984d0` (`cdecl`: stream, text, count), all unnamed.
const LOAD_BYTES_00A8D8B0: u32 = 0x00a8_d8b0;
const LOAD_FLAG_00A57580: u32 = 0x00a5_7580;
const LOAD_VERSION_00A575B0: u32 = 0x00a5_75b0;
const SAVE_BYTES_00E984D0: u32 = 0x00e9_84d0;
/// `NiGPUProgramCache::ms_bOverwriteDeprecatedCacheFiles` (Xbox PDB static).
const OVERWRITE_DEPRECATED_CACHE_FILES: u32 = 0x011c_2fdd;
/// The literal `"NiGPUProgram Cache File"`.
const TEXT_CACHE_FILE: u32 = 0x0110_1394;
/// The cache file version `Save` writes and `Load` requires at least.
const CACHE_FILE_VERSION: u32 = 5;
/// `NiMaterialResource::GetVariable` (`00eea510`) and the other resource getters
/// `NiBSplineBasisData::GetDegree3Basis` (`00a62660`, a folded body), `00eea520`,
/// `00be0ad0`, `00aa1ac0` (folded name `NiD3DShader::GetShaderDecl`), `00dbbc00` and
/// `00eea530`, all `thiscall` without stack words.
const RESOURCE_GET_VARIABLE: u32 = 0x00ee_a510;
const RESOURCE_GETTER_00A62660: u32 = 0x00a6_2660;
const RESOURCE_GETTER_00EEA520: u32 = 0x00ee_a520;
const RESOURCE_GETTER_00BE0AD0: u32 = 0x00be_0ad0;
const RESOURCE_GETTER_00AA1AC0: u32 = 0x00aa_1ac0;
const RESOURCE_GETTER_00DBBC00: u32 = 0x00db_bc00;
const RESOURCE_GETTER_00EEA530: u32 = 0x00ee_a530;
/// The resource's constructor (`00eea580`, `thiscall`) and the setters `LoadEntry` runs on
/// it: by address of a fixed string `00a5b950`, `00eea650`, `00eea690`, `00eea6d0`,
/// `00eea710`; by value `NiD3DShaderInterface::SetD3DRenderState` (`00eea540`, a folded
/// name), `00eea550`, `00eea560`, `00eea570`; all `thiscall`, one word.
const RESOURCE_CONSTRUCT_00EEA580: u32 = 0x00ee_a580;
const RESOURCE_STRING_SETTERS: [u32; 5] = [
    0x00a5_b950,
    0x00ee_a650,
    0x00ee_a690,
    0x00ee_a6d0,
    0x00ee_a710,
];
const RESOURCE_WORD_SETTERS: [u32; 4] = [0x00ee_a540, 0x00ee_a550, 0x00ee_a560, 0x00ee_a570];
/// `NiTSet<NiPointer<NiMaterialResource>>::Add` (`00e8ea60`, `thiscall`, the address of
/// the pointer), the program pointer's assignment `00a5f860` (`thiscall`, a program) and
/// the program map's `SetAt` `00e8e1d0` (`thiscall`: the address of the name, a counted
/// descriptor passed by value).
const RESOURCE_SET_ADD: u32 = 0x00e8_ea60;
const PROGRAM_ASSIGN_00A5F860: u32 = 0x00a5_f860;
const MAP_SET_AT_00E8E1D0: u32 = 0x00e8_e1d0;
/// `NiGPUProgramCache` (Xbox PDB): `m_kCachedPrograms` (+0x8, a fixed-string map: hash size
/// +0x4, table +0x8, count +0xc of its own), `m_kFilename` (+0x18), `m_kWorkingDir` (+0x1c),
/// `m_kPathAndFilename` (+0x20), `m_kShaderProfile` (+0x24), `m_kValidTargets` (+0x28),
/// `m_uiVersion` (+0x34), `m_bAutoWriteCacheToDisk` (+0x38), `m_bLoaded` (+0x39),
/// `m_bLocked` (+0x3a).
const CACHED_PROGRAMS: u32 = 0x8;
const FILE_NAME: u32 = 0x18;
const WORKING_DIRECTORY: u32 = 0x1c;
const PATH_AND_FILE_NAME: u32 = 0x20;
const SHADER_PROFILE: u32 = 0x24;
const VALID_TARGETS: u32 = 0x28;
const VERSION: u32 = 0x34;
const LOADED: u32 = 0x39;
const LOCKED: u32 = 0x3a;
/// `NiBinaryStream`: the position (+4), `m_pfnRead` (+8) and `m_pfnWrite` (+0xc).
const POSITION: u32 = 0x4;
const READ: u32 = 0x8;
const WRITE: u32 = 0xc;

/// Releases a counted object (count at +4, its slot +4 at zero) when it is not null.
fn release_object(e: &mut Engine, object: u32) {
    if object != 0 && interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The fixed-string handle for a C string (`00a5b690`).
fn fixed_string(e: &mut Engine, text: u32) -> u32 {
    e.call(GET_FIXED_STRING, &args![text]).u32()
}

/// One transfer through a binary stream's read (+8) or write (+0xc) function: `bytes` at
/// `data`, the component size word at `size_word` set to `size`, one component; the
/// position advanced by the result.
fn transfer(
    e: &mut Engine,
    stream: u32,
    slot: u32,
    data: u32,
    bytes: u32,
    size_word: u32,
    size: u32,
) {
    let function = e.mem.u32(stream.wrapping_add(slot));
    e.mem.set_u32(size_word, size);
    let done = e
        .call(function, &args![stream, data, bytes, size_word, 1u32])
        .u32();
    let position = e.mem.u32(stream.wrapping_add(POSITION)).wrapping_add(done);
    e.mem.set_u32(stream.wrapping_add(POSITION), position);
}

/// Writes the word `value` from a stack local (4 bytes, component size 4).
fn write_word(e: &mut Engine, stream: u32, value: u32) {
    e.with_stack(8, |e, local| {
        e.mem.set_u32(local.addr(), value);
        transfer(e, stream, WRITE, local.addr(), 4, local.addr() + 4, 4);
    });
}

/// The C string's length (`strlen`, the exe's inlined byte loop).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length += 1;
    }
    length
}

/// The inlined `strcmp` equality `PruneUnusedConstants` makes.
fn same_text(e: &Engine, a: u32, b: u32) -> bool {
    let mut index = 0u32;
    loop {
        let x = e.mem.u8(a.wrapping_add(index));
        if x != e.mem.u8(b.wrapping_add(index)) {
            return false;
        }
        if x == 0 {
            return true;
        }
        index += 1;
    }
}

/// Validates the working directory (`fn_00aa1ab0`) or else tries `00aa1bb0` on it; false
/// when it is the empty string.
fn working_directory_ready(e: &mut Engine, this: u32) -> bool {
    let directory = e.mem.u32(this + WORKING_DIRECTORY);
    if directory == e.mem.u32(EMPTY_FIXED_STRING) {
        return false;
    }
    if fn_00aa1ab0(e, Ptr::new(directory)) {
        return true;
    }
    let directory = e.mem.u32(this + WORKING_DIRECTORY);
    e.call(CREATE_DIRECTORY_00AA1BB0, &args![directory]).bool()
}

// Translated from 00ee8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::GetValidCompileTargetsForPlatform` (Xbox PDB): the address of
/// `m_kValidTargets` (+0x28).
pub fn ni_gpu_program_cache_get_valid_compile_targets_for_platform(
    _e: &mut Engine,
    this: Ptr,
) -> Ptr {
    Ptr::new(this.addr().wrapping_add(VALID_TARGETS))
}

// Translated from 00ee8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::GetShaderProfile` (Xbox PDB): the address of `m_kShaderProfile`
/// (+0x24).
pub fn ni_gpu_program_cache_get_shader_profile(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(SHADER_PROFILE))
}

// Translated from 00ee8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by the Xbox PDB layout `GetWorkingDirectory`: the address of
/// `m_kWorkingDir` (+0x1c).
pub fn fn_00ee8a90(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(WORKING_DIRECTORY))
}

// Translated from 00ee8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::IsLoaded` (Xbox PDB): `m_bLoaded` (+0x39).
pub fn ni_gpu_program_cache_is_loaded(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + LOADED) != 0
}

// Translated from 00ee8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::IsLocked` (Xbox PDB): `m_bLocked` (+0x3a).
pub fn ni_gpu_program_cache_is_locked(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + LOCKED) != 0
}

// Translated from 00ee8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the program map); by its body the map's first
/// position: the first non-null bucket of the table (+0x8) among the hash size (+0x4), or
/// null.
pub fn fn_00ee8ac0(e: &mut Engine, map: Ptr) -> u32 {
    let size = e.mem.u32(map.addr() + 4);
    let table = e.mem.u32(map.addr() + 8);
    (0..size)
        .map(|index| e.mem.u32(table.wrapping_add(4 * index)))
        .find(|item| *item != 0)
        .unwrap_or(0)
}

// Translated from 00ee8af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee8af0 - the /GS cookie check (00ec408c) guarding the two 0x104-byte path buffers is not modeled
/// `NiGPUProgramCache::SetWorkingDirectory` (Xbox PDB; `thiscall`, the address of a fixed
/// string): keeps a counted copy of the old directory. A relative directory is made
/// absolute against the current working directory (`NiPath::GetCurrentWorkingDirectory`
/// into one 0x104-byte buffer, `NiPath::ConvertToAbsolute` into another, assigned with
/// `00a2e750`); either failure releases the copy and answers false. Otherwise the handle is
/// assigned as it is (counted). When the directory changed, `ComputeFullFilename` (slot
/// +0x80) runs; the copy is released and the answer is true.
pub fn ni_gpu_program_cache_set_working_directory(
    e: &mut Engine,
    this: Ptr,
    directory: Ptr,
) -> bool {
    let t = this.addr();
    let slot = t + WORKING_DIRECTORY;
    let held = e.mem.u32(slot);
    acquire_fixed_string(e, held);
    let given = e.mem.u32(directory.addr());
    let old = e.mem.u32(slot);
    let relative =
        given != e.mem.u32(EMPTY_FIXED_STRING) && e.call(PATH_IS_RELATIVE, &args![given]).bool();
    if relative {
        let made = e.with_stack(0x208, |e, buffers| {
            let current = buffers.addr();
            let absolute = current + 0x104;
            if !e
                .call(
                    PATH_GET_CURRENT_WORKING_DIRECTORY,
                    &args![current, 0x104u32],
                )
                .bool()
            {
                return false;
            }
            let given = e.mem.u32(directory.addr());
            if e.call(
                PATH_CONVERT_TO_ABSOLUTE,
                &args![absolute, 0x104u32, given, current],
            )
            .u32()
                == 0
            {
                return false;
            }
            e.call(FIXED_STRING_ASSIGN_00A2E750, &args![slot, absolute]);
            true
        });
        if !made {
            release_fixed_string(e, old);
            return false;
        }
    } else {
        let given = e.mem.u32(directory.addr());
        if e.mem.u32(slot) != given {
            acquire_fixed_string(e, given);
            let current = e.mem.u32(slot);
            release_fixed_string(e, current);
            e.mem.set_u32(slot, given);
        }
    }
    if old != e.mem.u32(slot) {
        e.vcall(t, 0x80, &args![]);
    }
    release_fixed_string(e, old);
    true
}

// Translated from 00ee8c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee8c10 - the /GS cookie check (00ec408c) guarding the 0x104-byte path buffer is not modeled
/// `NiGPUProgramCache::ComputeFullFilename` (Xbox PDB): without a working directory or a
/// file name, `m_kPathAndFilename` (+0x20) becomes the null string's handle when it is
/// set (the old one released) and `m_bLoaded` is cleared. Otherwise, with a counted copy of
/// the old path, the file name is made absolute against the working directory
/// (`NiPath::ConvertToAbsolute` into a 0x104-byte buffer) and assigned (unless the handle
/// already is the buffer's address); a changed path runs `Clear` (slot +0x70); the copy is
/// released.
pub fn ni_gpu_program_cache_compute_full_filename(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    if e.mem.u32(t + WORKING_DIRECTORY) == empty || e.mem.u32(t + FILE_NAME) == empty {
        let old = e.mem.u32(t + PATH_AND_FILE_NAME);
        if old != 0 {
            let handle = fixed_string(e, 0);
            e.mem.set_u32(t + PATH_AND_FILE_NAME, handle);
            release_fixed_string(e, old);
        }
        e.mem.set_u8(t + LOADED, 0);
        return;
    }
    let held = e.mem.u32(t + PATH_AND_FILE_NAME);
    acquire_fixed_string(e, held);
    let directory = e.mem.u32(t + WORKING_DIRECTORY);
    let name = e.mem.u32(t + FILE_NAME);
    let old = e.mem.u32(t + PATH_AND_FILE_NAME);
    e.with_stack(0x104, |e, buffer| {
        e.call(
            PATH_CONVERT_TO_ABSOLUTE,
            &args![buffer, 0x104u32, name, directory],
        );
        let current = e.mem.u32(t + PATH_AND_FILE_NAME);
        if current != buffer.addr() {
            let handle = fixed_string(e, buffer.addr());
            e.mem.set_u32(t + PATH_AND_FILE_NAME, handle);
            release_fixed_string(e, current);
        }
    });
    if old != e.mem.u32(t + PATH_AND_FILE_NAME) {
        e.vcall(t, 0x70, &args![]);
    }
    release_fixed_string(e, old);
}

// Translated from 00ee8d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): false without a working directory; true when it
/// validates (`00aa1ab0`); otherwise what `00aa1bb0` answers for it.
pub fn fn_00ee8d20(e: &mut Engine, this: Ptr) -> bool {
    working_directory_ready(e, this.addr())
}

// Translated from 00ee8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::AppendEntry` (Xbox PDB; `thiscall`: a descriptor and the address of
/// its name): false without a path; a missing file (`NiFile::Access` mode 0) is created
/// first by `Save` (slot +0x6c) when the path is accessible in mode 1 (false otherwise or
/// when `Save` fails). The file is opened (`NiFile::GetFile`, mode 2, 0x8000 bytes; false
/// when null), and when its slot +4 answers exactly 1 the entry is written (`SaveEntry`,
/// slot +0x88: file, descriptor, name) and its answer returned; the file is deleted (slot
/// 0 with 1) either way.
pub fn ni_gpu_program_cache_append_entry(e: &mut Engine, this: Ptr, desc: Ptr, name: Ptr) -> bool {
    let t = this.addr();
    let path = e.mem.u32(t + PATH_AND_FILE_NAME);
    if path == e.mem.u32(EMPTY_FIXED_STRING) {
        return false;
    }
    if !e.call(FILE_ACCESS, &args![path, 0u32]).bool() {
        let path = e.mem.u32(t + PATH_AND_FILE_NAME);
        if !e.call(FILE_ACCESS, &args![path, 1u32]).bool() || !e.vcall(t, 0x6c, &args![]).bool() {
            return false;
        }
    }
    let path = e.mem.u32(t + PATH_AND_FILE_NAME);
    let file = e.call(FILE_GET_FILE, &args![path, 2u32, 0x8000u32]).u32();
    if file == 0 {
        return false;
    }
    if e.vcall(file, 4, &args![]).u8() != 1 {
        e.vcall(file, 0, &args![1u32]);
        return false;
    }
    let saved = e.vcall(t, 0x88, &args![file, desc, name]).bool();
    e.vcall(file, 0, &args![1u32]);
    saved
}

// Translated from 00ee8e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a fixed-string set: elements +0, allocated
/// count +4, used count +8); by its body the set's `RemoveAll`: each non-null element
/// becomes the null string's handle (the old one released), then the used count is 0.
pub fn fn_00ee8e10(e: &mut Engine, set: Ptr) {
    let s = set.addr();
    let mut index = 0u32;
    while index < e.mem.u32(s + 4) {
        let slot = e.mem.u32(s).wrapping_add(4 * index);
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = fixed_string(e, 0);
            e.mem.set_u32(slot, handle);
            release_fixed_string(e, old);
        }
        index += 1;
    }
    e.mem.set_u32(s + 8, 0);
}

// Translated from 00ee8e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::Clear` (Xbox PDB): empties the program map (`RemoveAll`), clears
/// `m_bLoaded` and answers true.
pub fn ni_gpu_program_cache_clear(e: &mut Engine, this: Ptr) -> bool {
    e.call(MAP_REMOVE_ALL, &args![this.addr() + CACHED_PROGRAMS]);
    e.mem.set_u8(this.addr() + LOADED, 0);
    true
}

// Translated from 00ee8e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMapBase<..NiPointer<NiGPUProgramDesc>..>::GetAt` (Xbox PDB; `thiscall`:
/// the address of the key handle, the address of a pointer): the bucket is the handle
/// modulo the hash size (+4); the first item (next +0, key +4, value +8) with that handle
/// gives its value to the pointer (counted assignment) and the answer is true; none
/// answers false.
pub fn ni_t_fixed_string_map_base_get_at(e: &mut Engine, map: Ptr, key: Ptr, value: Ptr) -> bool {
    let handle = e.mem.u32(key.addr());
    let bucket = handle % e.mem.u32(map.addr() + 4);
    let table = e.mem.u32(map.addr() + 8);
    let mut item = e.mem.u32(table.wrapping_add(4 * bucket));
    while item != 0 {
        if handle == e.mem.u32(item + 4) {
            let found = e.mem.u32(item + 8);
            assign_ref_counted(e, value.addr(), found);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 00ee8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the program map: the address of a position, of
/// a key and of a value); by its body `GetNext`: the item at the position gives its key
/// (assigned, counted) and value (counted assignment); the position moves to the item's
/// next one, or else to the first non-null bucket after the item's own (the key modulo the
/// hash size, plus one), or null.
pub fn fn_00ee8f10(e: &mut Engine, map: Ptr, position: Ptr, key: Ptr, value: Ptr) {
    let item = e.mem.u32(position.addr());
    let handle = e.mem.u32(item + 4);
    if e.mem.u32(key.addr()) != handle {
        acquire_fixed_string(e, handle);
        let old = e.mem.u32(key.addr());
        release_fixed_string(e, old);
        e.mem.set_u32(key.addr(), handle);
    }
    let found = e.mem.u32(item + 8);
    assign_ref_counted(e, value.addr(), found);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let size = e.mem.u32(map.addr() + 4);
    let mut bucket = e.mem.u32(item + 4) % size + 1;
    let table = e.mem.u32(map.addr() + 8);
    while bucket < size {
        let candidate = e.mem.u32(table.wrapping_add(4 * bucket));
        if candidate != 0 {
            e.mem.set_u32(position.addr(), candidate);
            return;
        }
        bucket += 1;
    }
    e.mem.set_u32(position.addr(), 0);
}

// Translated from 00ee8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a counted-pointer set: elements +0, used count
/// +8; one word, the index); by its body the ordered removal: each element from the index
/// on takes the next one (counted assignment) up to the last, the used count drops by one
/// and the old last element is released and cleared.
pub fn fn_00ee8fe0(e: &mut Engine, set: Ptr, index: u32) {
    let s = set.addr();
    let mut position = index;
    while position < e.mem.u32(s + 8).wrapping_sub(1) {
        let slot = e.mem.u32(s).wrapping_add(4 * position);
        let current = e.mem.u32(slot);
        if current != e.mem.u32(slot + 4) {
            release_object(e, current);
            let next = e.mem.u32(slot + 4);
            e.mem.set_u32(slot, next);
            if next != 0 {
                interlocked_increment(e, next + 4);
            }
        }
        position += 1;
    }
    let used = e.mem.u32(s + 8).wrapping_sub(1);
    e.mem.set_u32(s + 8, used);
    let slot = e.mem.u32(s).wrapping_add(4 * used);
    let last = e.mem.u32(slot);
    if last != 0 {
        release_object(e, last);
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00ee9070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the program map base's scalar deleting
/// destructor (`thiscall`, the flags): the vtable `01101378`, `RemoveAll`, the table freed
/// (`00aa10f0`), and with bit 0 the map freed through the memory manager. Answers the map.
pub fn fn_00ee9070(e: &mut Engine, map: Ptr, flags: u32) -> Ptr {
    let m = map.addr();
    e.mem.set_u32(m, MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let table = e.mem.u32(m + 8);
    e.call(FREE_ARRAY, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, map]);
    }
    map
}

// Translated from 00ee90b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiPointer<NiGPUProgramDesc>>::_scalar_deleting_destructor_` (Xbox
/// PDB): the vtable `01101388` and `RemoveAll`, then the base's (`01101378`, `RemoveAll`
/// again, the table freed), and with bit 0 the map freed through the memory manager.
/// Answers the map.
pub fn ni_t_fixed_string_map_scalar_deleting_destructor(
    e: &mut Engine,
    map: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(map.addr(), MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    fn_00ee9070(e, map, flags)
}

// Translated from 00ee90f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee90f0 - the /GS cookie check (00ec408c) guarding the 0x400-byte flag buffer is not modeled
/// `NiGPUProgramCache::PruneUnusedConstants` (Xbox PDB: static; two stack words: a program and
/// a resource set): with a 0x400-byte flag per resource (cleared with `_memset`), every
/// variable the program names (slot +0x48 the count, asked again each round, +0x4c the
/// name) marks the first resource whose variable handle (`GetVariable`) is that name, or
/// compares equal as text; then from the last resource down every unmarked one is removed
/// (`00ee8fe0`). Answers true.
/// On the PC it is a vtable slot (31, no direct callers), so it is called with `ECX` set:
/// `this` comes first, unread.
pub fn ni_gpu_program_cache_prune_unused_constants(
    e: &mut Engine,
    _this: Ptr,
    program: Ptr,
    resources: Ptr,
) -> bool {
    let p = program.addr();
    let r = resources.addr();
    let used = e.mem.u32(r + 8);
    e.with_stack(0x400, |e, flags| {
        let flags = flags.addr();
        e.call(MEMSET, &args![flags, 0u32, 0x400u32]);
        let mut variable = 0u32;
        if e.vcall(p, 0x48, &args![]).u32() != 0 {
            loop {
                let name = e.vcall(p, 0x4c, &args![variable]).u32();
                for index in 0..used {
                    let resource = e.mem.u32(e.mem.u32(r).wrapping_add(4 * index));
                    if resource == 0 {
                        continue;
                    }
                    let handle = e.call(RESOURCE_GET_VARIABLE, &args![resource]).u32();
                    let handle = e.mem.u32(handle);
                    if handle == name || (name != 0 && handle != 0 && same_text(e, handle, name)) {
                        e.mem.set_u8(flags.wrapping_add(index), 1);
                        break;
                    }
                }
                variable += 1;
                if variable >= e.vcall(p, 0x48, &args![]).u32() {
                    break;
                }
            }
        }
        let mut index = used.wrapping_sub(1) as i32;
        while index >= 0 {
            if e.mem.u8(flags.wrapping_add(index as u32)) == 0 {
                fn_00ee8fe0(e, resources, index as u32);
            }
            index -= 1;
        }
    });
    true
}

// Translated from 00ee9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (static, two words: a binary stream and a C string); by its
/// body `SaveString`: a null string writes a 4-byte 0; otherwise its length plus one (4
/// bytes, component size 4), then that many bytes (component size 1). Answers true.
pub fn fn_00ee9230(e: &mut Engine, stream: Ptr, text: Ptr) -> bool {
    let s = stream.addr();
    if text.is_null() {
        write_word(e, s, 0);
        return true;
    }
    let length = string_length(e, text.addr()) + 1;
    write_word(e, s, length);
    e.with_stack(4, |e, size| {
        transfer(e, s, WRITE, text.addr(), length, size.addr(), 1);
    });
    true
}

// Translated from 00ee92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::LoadString` (Xbox PDB; static, three words: a binary stream, a
/// buffer, its size): false for a zero size or a null buffer; otherwise the length (4
/// bytes, read into the size's stack word) and, for a length of 0, an empty buffer; else
/// that many bytes (`00a8d8b0`) and a 0 byte one past them (at length + 1, as the exe
/// writes it). Answers true.
pub fn ni_gpu_program_cache_load_string(
    e: &mut Engine,
    stream: Ptr,
    buffer: Ptr,
    size: u32,
) -> bool {
    if size == 0 || buffer.is_null() {
        return false;
    }
    let s = stream.addr();
    let b = buffer.addr();
    let length = e.with_stack(8, |e, words| {
        let w = words.addr();
        e.mem.set_u32(w, 0);
        transfer(e, s, READ, w, 4, w + 4, 4);
        e.mem.u32(w)
    });
    if length == 0 {
        e.mem.set_u8(b, 0);
        return true;
    }
    e.call(LOAD_BYTES_00A8D8B0, &args![s, b, length]);
    e.mem.set_u8(b.wrapping_add(length).wrapping_add(1), 0);
    true
}

// Translated from 00ee9350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee9350 - the /GS cookie check (00ec408c) guarding the 0x200-byte text buffer is not modeled
/// `NiGPUProgramCache::Load` (Xbox PDB): false when the working directory is not ready
/// (empty, not valid and not creatable); true at once when loaded. The cache file
/// (`NiFile::GetFile`, mode 0, 0x8000 bytes; false when null; deleted and false when its
/// slot +4 is not exactly 1) must start with `"NiGPUProgram Cache File"` (read with its
/// terminator, `NiStricmp`), then a byte and a version: below 5 the file is dropped and,
/// with `ms_bOverwriteDeprecatedCacheFiles`, a fresh one saved (slot +0x6c), answering
/// false. The platform (slot +0x64), the code id (+0x60) and the shader profile (+0x54's
/// handle) must match the stored strings (`LoadString` with 0x200 bytes). Then entries are
/// loaded (`LoadEntry`, slot +0x8c) while the file's position (+0xc) is below its size
/// (+0x1c; both asked again each round), false and the file dropped on a failure; an entry
/// flagged as out of date makes a save follow under the same static. Loaded is set.
pub fn ni_gpu_program_cache_load(e: &mut Engine, this: Ptr) -> bool {
    let t = this.addr();
    if !working_directory_ready(e, t) {
        return false;
    }
    if e.mem.u8(t + LOADED) != 0 {
        return true;
    }
    let path = e.mem.u32(t + PATH_AND_FILE_NAME);
    let file = e.call(FILE_GET_FILE, &args![path, 0u32, 0x8000u32]).u32();
    if file == 0 {
        return false;
    }
    let delete = |e: &mut Engine| {
        e.vcall(file, 0, &args![1u32]);
    };
    if e.vcall(file, 4, &args![]).u8() != 1 {
        delete(e);
        return false;
    }
    // The frame: +6 the out-of-date flag, +7 the byte, +8 the version, +0xc the buffer.
    e.with_stack(0x20c, |e, frame| {
        let f = frame.addr();
        let buffer = f + 0xc;
        let length = string_length(e, TEXT_CACHE_FILE) + 1;
        e.call(LOAD_BYTES_00A8D8B0, &args![file, buffer, length]);
        if e.call(NI_STRICMP, &args![buffer, TEXT_CACHE_FILE]).u32() != 0 {
            delete(e);
            return false;
        }
        e.call(LOAD_FLAG_00A57580, &args![file, f + 7]);
        e.call(LOAD_VERSION_00A575B0, &args![file, f + 8]);
        if e.mem.u32(f + 8) < CACHE_FILE_VERSION {
            delete(e);
            if e.mem.u8(OVERWRITE_DEPRECATED_CACHE_FILES) != 0 {
                e.vcall(t, 0x6c, &args![]);
            }
            return false;
        }
        for slot in [0x64u32, 0x60, 0x54] {
            ni_gpu_program_cache_load_string(e, Ptr::new(file), Ptr::new(buffer), 0x200);
            let mut expected = e.vcall(t, slot, &args![]).u32();
            if slot == 0x54 {
                expected = e.mem.u32(expected);
            }
            if e.call(NI_STRICMP, &args![buffer, expected]).u32() != 0 {
                delete(e);
                return false;
            }
        }
        let mut stale = false;
        let mut size = e.vcall(file, 0x1c, &args![]).u32();
        while e.vcall(file, 0xc, &args![]).u32() < size {
            e.mem.set_u8(f + 6, 0);
            if !e.vcall(t, 0x8c, &args![file, f + 6]).bool() {
                delete(e);
                return false;
            }
            if e.mem.u8(f + 6) != 0 {
                stale = true;
            }
            size = e.vcall(file, 0x1c, &args![]).u32();
        }
        delete(e);
        e.mem.set_u8(t + LOADED, 1);
        if stale && e.mem.u8(OVERWRITE_DEPRECATED_CACHE_FILES) != 0 {
            e.vcall(t, 0x6c, &args![]);
        }
        true
    })
}

// Translated from 00ee9620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::Save` (Xbox PDB): false when the working directory is not ready;
/// the cache file (`NiFile::GetFile`, mode 1, 0x8000 bytes; false when null, and false
/// without deleting it when its slot +4 is not exactly 1) gets the header text with its
/// terminator (`00e984d0`), a byte 1, the version 5, the platform (slot +0x64), the code id
/// (+0x60) and the shader profile (+0x54's handle) as strings (`00ee9230`), then each map
/// entry (`GetFirstPos`, `GetNext` with a key and a value that are released after) through
/// `SaveEntry` (slot +0x88: file, descriptor, address of the key). The file is deleted and
/// the answer is true.
pub fn ni_gpu_program_cache_save(e: &mut Engine, this: Ptr) -> bool {
    let t = this.addr();
    if !working_directory_ready(e, t) {
        return false;
    }
    let path = e.mem.u32(t + PATH_AND_FILE_NAME);
    let file = e.call(FILE_GET_FILE, &args![path, 1u32, 0x8000u32]).u32();
    if file == 0 || e.vcall(file, 4, &args![]).u8() != 1 {
        return false;
    }
    let length = string_length(e, TEXT_CACHE_FILE) + 1;
    e.call(SAVE_BYTES_00E984D0, &args![file, TEXT_CACHE_FILE, length]);
    e.with_stack(8, |e, locals| {
        let l = locals.addr();
        e.mem.set_u8(l, 1);
        transfer(e, file, WRITE, l, 1, l + 4, 1);
    });
    write_word(e, file, CACHE_FILE_VERSION);
    let platform = e.vcall(t, 0x64, &args![]).u32();
    fn_00ee9230(e, Ptr::new(file), Ptr::new(platform));
    let code = e.vcall(t, 0x60, &args![]).u32();
    fn_00ee9230(e, Ptr::new(file), Ptr::new(code));
    let profile = e.vcall(t, 0x54, &args![]).u32();
    let profile = e.mem.u32(profile);
    fn_00ee9230(e, Ptr::new(file), Ptr::new(profile));
    let map = Ptr::new(t + CACHED_PROGRAMS);
    let first = fn_00ee8ac0(e, map);
    e.with_stack(0xc, |e, locals| {
        // +0 the key, +4 the value, +8 the position.
        let l = locals.addr();
        e.mem.set_u32(l + 8, first);
        while e.mem.u32(l + 8) != 0 {
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            e.mem.set_u32(l, empty);
            e.mem.set_u32(l + 4, 0);
            fn_00ee8f10(e, map, Ptr::new(l + 8), Ptr::new(l), Ptr::new(l + 4));
            let desc = e.mem.u32(l + 4);
            e.vcall(t, 0x88, &args![file, desc, l]);
            let key = e.mem.u32(l);
            release_fixed_string(e, key);
            release_object(e, desc);
        }
    });
    e.vcall(file, 0, &args![1u32]);
    true
}

/// Saves the string whose handle sits at `at` (`00ee9230`).
fn save_handle(e: &mut Engine, file: Ptr, at: u32) {
    let text = e.mem.u32(at);
    fn_00ee9230(e, file, Ptr::new(text));
}

// Translated from 00ee97e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::SaveEntry` (Xbox PDB; `thiscall`: a file, a descriptor, the address
/// of the name): the version (+0x34, 4 bytes), the name (`00ee9230`), the program through
/// `SaveGPUProgram` (slot +0x90: file, the descriptor's program +8; false when it fails),
/// the resource count (+0x14) and for each resource (+0xc's array) five strings (the
/// getters `00a62660`, `00eea4f0`, `00eea500`, `GetVariable`, `00eea520`, each a handle
/// address) and four words (`00be0ad0`, `00aa1ac0`, `00dbbc00`, `00eea530`). Answers true.
pub fn ni_gpu_program_cache_save_entry(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
    desc: Ptr,
    name: Ptr,
) -> bool {
    let t = this.addr();
    let f = file.addr();
    let d = desc.addr();
    e.with_stack(4, |e, size| {
        transfer(e, f, WRITE, t + VERSION, 4, size.addr(), 4);
    });
    let handle = e.mem.u32(name.addr());
    fn_00ee9230(e, file, Ptr::new(handle));
    let program = e.mem.u32(d + 8);
    if !e.vcall(t, 0x90, &args![file, program]).bool() {
        return false;
    }
    let count = e.mem.u32(d + 0x14);
    write_word(e, f, count);
    for index in 0..count {
        let resource = e.mem.u32(e.mem.u32(d + 0xc).wrapping_add(4 * index));
        let at = e.call(RESOURCE_GETTER_00A62660, &args![resource]).u32();
        save_handle(e, file, at);
        let at = fn_00eea4f0(e, Ptr::new(resource)).addr();
        save_handle(e, file, at);
        let at = fn_00eea500(e, Ptr::new(resource)).addr();
        save_handle(e, file, at);
        let at = e.call(RESOURCE_GET_VARIABLE, &args![resource]).u32();
        save_handle(e, file, at);
        let at = e.call(RESOURCE_GETTER_00EEA520, &args![resource]).u32();
        save_handle(e, file, at);
        for getter in [
            RESOURCE_GETTER_00BE0AD0,
            RESOURCE_GETTER_00AA1AC0,
            RESOURCE_GETTER_00DBBC00,
            RESOURCE_GETTER_00EEA530,
        ] {
            let value = e.call(getter, &args![resource]).u32();
            write_word(e, f, value);
        }
    }
    true
}

// Translated from 00ee99b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiPointer<NiGPUProgramDesc>>::NewItem` (Xbox PDB): a 0xc-byte item
/// from the memory manager with its value (+8) and key (+4) cleared. A failed allocation
/// still writes the key word at address 4, as the exe does.
pub fn ni_t_fixed_string_map_new_item(e: &mut Engine, _this: Ptr) -> Ptr {
    let item = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if item == 0 {
        e.mem.set_u32(4, 0);
        return Ptr::new(0);
    }
    e.mem.set_u32(item + 8, 0);
    e.mem.set_u32(item + 4, 0);
    Ptr::new(item)
}

// Translated from 00ee99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiPointer<NiGPUProgramDesc>>::DeleteItem` (Xbox PDB; `thiscall`, the
/// item): a set key becomes the null string's handle (the old released), a set value is
/// released and cleared; then (for a non-null item) the item's destructor releases the
/// value and the key again and the item goes back to the memory manager.
pub fn ni_t_fixed_string_map_delete_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    let i = item.addr();
    let key = e.mem.u32(i.wrapping_add(4));
    if key != 0 {
        let handle = fixed_string(e, 0);
        e.mem.set_u32(i.wrapping_add(4), handle);
        release_fixed_string(e, key);
    }
    let value = e.mem.u32(i.wrapping_add(8));
    if value != 0 {
        release_object(e, value);
        e.mem.set_u32(i.wrapping_add(8), 0);
    }
    if i != 0 {
        let value = e.mem.u32(i + 8);
        release_object(e, value);
        let key = e.mem.u32(i + 4);
        release_fixed_string(e, key);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, item]);
    }
}

// Translated from 00ee9a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the `NiGPUProgramCache` destructor body
/// (`thiscall`): the vtable `011013b4`, the valid targets emptied (`00ee8e10`) and their
/// array handed to `00a644a0`, the four fixed strings (+0x24, +0x20, +0x1c, +0x18)
/// released, the program map destroyed (its vtables, `RemoveAll` twice, the table freed),
/// then the `NiRefObject` vtable and one object less in `ms_uiObjects`.
pub fn fn_00ee9a60(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, NI_GPU_PROGRAM_CACHE_VTABLE);
    fn_00ee8e10(e, Ptr::new(t + VALID_TARGETS));
    let strings = e.mem.u32(t + VALID_TARGETS);
    e.call(FREE_STRING_ARRAY_00A644A0, &args![strings]);
    for offset in [
        SHADER_PROFILE,
        PATH_AND_FILE_NAME,
        WORKING_DIRECTORY,
        FILE_NAME,
    ] {
        let handle = e.mem.u32(t + offset);
        release_fixed_string(e, handle);
    }
    let map = t + CACHED_PROGRAMS;
    e.mem.set_u32(map, MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let table = e.mem.u32(map + 8);
    e.call(FREE_ARRAY, &args![table]);
    e.mem.set_u32(t, NI_REF_OBJECT_VTABLE);
    interlocked_decrement(e, REF_OBJECT_COUNT);
}

// Translated from 00ee9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::FindCachedProgram` (Xbox PDB; `thiscall`: a name, a resource set):
/// the name's handle (the empty string's for null, not counted) is looked up in the program
/// map (`GetAt`); a found descriptor adds each of its resources (+0xc, count +0x14) to the
/// set (`NiTSet::Add`) and answers its program (+8); the descriptor and the handle are
/// released. Not found answers null.
pub fn ni_gpu_program_cache_find_cached_program(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    resources: Ptr,
) -> Ptr {
    let t = this.addr();
    let handle = if name.is_null() {
        e.mem.u32(EMPTY_FIXED_STRING)
    } else {
        fixed_string(e, name.addr())
    };
    let program = e.with_stack(8, |e, locals| {
        let l = locals.addr();
        e.mem.set_u32(l, handle);
        e.mem.set_u32(l + 4, 0);
        let map = Ptr::new(t + CACHED_PROGRAMS);
        if !ni_t_fixed_string_map_base_get_at(e, map, Ptr::new(l), Ptr::new(l + 4)) {
            let desc = e.mem.u32(l + 4);
            release_object(e, desc);
            return 0;
        }
        let desc = e.mem.u32(l + 4);
        let mut index = 0u32;
        while index < e.mem.u32(desc + 0x14) {
            let entry = e.mem.u32(desc + 0xc).wrapping_add(4 * index);
            e.call(RESOURCE_SET_ADD, &args![resources, entry]);
            index += 1;
        }
        let program = e.mem.u32(desc + 8);
        release_object(e, desc);
        program
    });
    release_fixed_string(e, handle);
    Ptr::new(program)
}

// Translated from 00ee9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtables the `NiGPUProgramCache` constructor
/// (`thiscall`: the version, the auto-write flag, the locked flag): the `NiRefObject` base
/// (count 0, one object more), the vtable `011013b4`, the program map (37 buckets, a 0x94-byte
/// table from `00aa1070` cleared, no items), the four fixed strings empty, the valid targets
/// with ten fixed strings (`00a63ae0`) and none used, the version and the two flags kept and
/// loaded clear. Answers `this`.
pub fn fn_00ee9bf0(e: &mut Engine, this: Ptr, version: u32, auto_write: u32, locked: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(t + 4, 0);
    interlocked_increment(e, REF_OBJECT_COUNT);
    e.mem.set_u32(t, NI_GPU_PROGRAM_CACHE_VTABLE);
    let map = t + CACHED_PROGRAMS;
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let table = e.call(ALLOCATE_ARRAY, &args![0x94u32]).u32();
    e.mem.set_u32(map + 8, table);
    let bytes = e.mem.u32(map + 4).wrapping_mul(4);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    e.mem.set_u32(map, MAP_VTABLE);
    for offset in [
        FILE_NAME,
        WORKING_DIRECTORY,
        PATH_AND_FILE_NAME,
        SHADER_PROFILE,
    ] {
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(t + offset, empty);
    }
    let strings = e.call(NEW_STRING_ARRAY_00A63AE0, &args![0xau32]).u32();
    e.mem.set_u32(t + VALID_TARGETS, strings);
    e.mem.set_u32(t + VALID_TARGETS + 8, 0);
    e.mem.set_u32(t + VALID_TARGETS + 4, 0xa);
    e.mem.set_u32(t + VERSION, version);
    e.mem.set_u8(t + LOADED, 0);
    e.mem.set_u8(t + 0x38, auto_write as u8);
    e.mem.set_u8(t + LOCKED, locked as u8);
    this
}

/// `LoadEntry`'s string read: the length into the frame's +0x14 (cleared first, component
/// size word +0x10 set to 4), then for a length of 0 an empty buffer at +0x54, else that
/// many bytes into it (component size 1) and a 0 byte at length + 1.
fn load_entry_string(e: &mut Engine, stream: u32, frame: u32) {
    e.mem.set_u32(frame + 0x14, 0);
    transfer(e, stream, READ, frame + 0x14, 4, frame + 0x10, 4);
    let length = e.mem.u32(frame + 0x14);
    if length == 0 {
        e.mem.set_u8(frame + 0x54, 0);
        return;
    }
    transfer(e, stream, READ, frame + 0x54, length, frame + 0x10, 1);
    let length = e.mem.u32(frame + 0x14);
    e.mem
        .set_u8(frame.wrapping_add(0x55).wrapping_add(length), 0);
}

// Translated from 00ee9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee9cc0 - the /GS cookie check (00ec408c) guarding the 0x200-byte name buffer is not modeled
/// `NiGPUProgramCache::LoadEntry` (Xbox PDB; `thiscall`: a binary stream, the address of
/// the out-of-date flag), on the exe's frame (the locals at their offsets): the version and
/// the name (length, then `00a8d8b0`) are read and the name's handle made. A version other
/// than this cache's sets the flag and skips the entry, as does a name already in the map
/// (`GetAt`). A kept entry gets a new descriptor (0x18 bytes: the `NiRefObject` base, vtable
/// `010f0134`, cleared, not counted). `LoadGPUProgram` (slot +0x94: stream, address of the
/// name, address of the program, the skip flag as a word whose low byte the callee reads;
/// its upper bytes are the exe's uninitialised stack, 0 here) must succeed (else the name is
/// released and false answered); a kept descriptor takes the program (`00a5f860`). Each of
/// the resources then gets five strings and four words: kept, a new resource (0x2c bytes,
/// `00eea580`) takes them through its setters and joins the descriptor's set
/// (`NiTSet::Add`, counted around the call); skipped, they are read and dropped. A kept
/// descriptor with a program goes into the map (`00e8e1d0`, counted by value). The name is
/// released; the answer is true.
pub fn ni_gpu_program_cache_load_entry(e: &mut Engine, this: Ptr, stream: Ptr, flag: Ptr) -> bool {
    let s = stream.addr();
    e.with_stack(0x258, |e, frame| {
        let f = frame.addr();
        e.mem.set_u32(f + 0x30, flag.addr());
        e.mem.set_u32(f + 0x28, this.addr());
        transfer(e, s, READ, f + 0x4c, 4, f + 0x1c, 4);
        e.mem.set_u32(f + 0x18, 0);
        transfer(e, s, READ, f + 0x18, 4, f + 0x1c, 4);
        let length = e.mem.u32(f + 0x18);
        if length == 0 {
            e.mem.set_u8(f + 0x54, 0);
        } else {
            e.call(LOAD_BYTES_00A8D8B0, &args![s, f + 0x54, length]);
            let length = e.mem.u32(f + 0x18);
            e.mem.set_u8(f.wrapping_add(0x55).wrapping_add(length), 0);
        }
        let name = fixed_string(e, f + 0x54);
        e.mem.set_u32(f + 0x2c, name);
        e.mem.set_u8(f + 0x20, 0);
        let t = this.addr();
        if e.mem.u32(f + 0x4c) != e.mem.u32(t + VERSION) {
            let out = e.mem.u32(f + 0x30);
            e.mem.set_u8(out, 1);
            e.mem.set_u8(f + 0x20, 1);
        }
        e.mem.set_u32(f + 0x18, 0);
        e.mem.set_u32(f + 0x50, t + CACHED_PROGRAMS);
        if ni_t_fixed_string_map_base_get_at(
            e,
            Ptr::new(t + CACHED_PROGRAMS),
            Ptr::new(f + 0x2c),
            Ptr::new(f + 0x18),
        ) {
            e.mem.set_u8(f + 0x20, 1);
        }
        let found = e.mem.u32(f + 0x18);
        release_object(e, found);
        e.mem.set_u32(f + 0x18, 0);
        if e.mem.u8(f + 0x20) == 0 {
            let desc = e.call(ALLOCATE, &args![0x18u32]).u32();
            if desc != 0 {
                e.mem.set_u32(desc, NI_REF_OBJECT_VTABLE);
                e.mem.set_u32(desc + 4, 0);
                interlocked_increment(e, REF_OBJECT_COUNT);
                e.mem.set_u32(desc, PROGRAM_DESC_VTABLE);
                for offset in [8u32, 0xc, 0x10, 0x14] {
                    e.mem.set_u32(desc + offset, 0);
                }
            }
            e.mem.set_u32(f + 0x18, desc);
        }
        let t = e.mem.u32(f + 0x28);
        let skip = e.mem.u8(f + 0x20);
        e.mem.set_u32(f + 0x3c, 0);
        if !e
            .vcall(t, 0x94, &args![s, f + 0x2c, f + 0x3c, u32::from(skip)])
            .bool()
        {
            let name = e.mem.u32(f + 0x2c);
            release_fixed_string(e, name);
            return false;
        }
        if e.mem.u8(f + 0x20) == 0 {
            let desc = e.mem.u32(f + 0x18);
            let program = e.mem.u32(f + 0x3c);
            e.call(
                PROGRAM_ASSIGN_00A5F860,
                &args![desc.wrapping_add(8), program],
            );
        }
        e.mem.set_u32(f + 0x34, 0);
        transfer(e, s, READ, f + 0x34, 4, f + 0x1c, 4);
        e.mem.set_u32(f + 0x1c, 0);
        while e.mem.u32(f + 0x1c) < e.mem.u32(f + 0x34) {
            if e.mem.u8(f + 0x20) != 0 {
                for _ in 0..5 {
                    load_entry_string(e, s, f);
                }
                for _ in 0..4 {
                    transfer(e, s, READ, f + 0x28, 4, f + 0x10, 4);
                }
            } else {
                let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
                let resource = if block == 0 {
                    0
                } else {
                    e.call(RESOURCE_CONSTRUCT_00EEA580, &args![block]).u32()
                };
                for (slot, setter) in [0x38u32, 0x40, 0x44, 0x48, 0x30]
                    .into_iter()
                    .zip(RESOURCE_STRING_SETTERS)
                {
                    load_entry_string(e, s, f);
                    let handle = fixed_string(e, f + 0x54);
                    e.mem.set_u32(f + slot, handle);
                    e.call(setter, &args![resource, f + slot]);
                    let handle = e.mem.u32(f + slot);
                    release_fixed_string(e, handle);
                }
                for setter in RESOURCE_WORD_SETTERS {
                    transfer(e, s, READ, f + 0x24, 4, f + 0x10, 4);
                    let value = e.mem.u32(f + 0x24);
                    e.call(setter, &args![resource, value]);
                }
                e.mem.set_u32(f + 0x10, resource);
                if resource != 0 {
                    interlocked_increment(e, resource + 4);
                }
                let desc = e.mem.u32(f + 0x18);
                e.call(RESOURCE_SET_ADD, &args![desc.wrapping_add(0xc), f + 0x10]);
                release_object(e, resource);
            }
            let next = e.mem.u32(f + 0x1c) + 1;
            e.mem.set_u32(f + 0x1c, next);
        }
        if e.mem.u8(f + 0x20) == 0 {
            let desc = e.mem.u32(f + 0x18);
            if e.mem.u32(desc.wrapping_add(8)) != 0 {
                interlocked_increment(e, desc.wrapping_add(4));
                let map = e.mem.u32(f + 0x50);
                e.call(MAP_SET_AT_00E8E1D0, &args![map, f + 0x2c, desc]);
            }
        }
        let name = e.mem.u32(f + 0x2c);
        release_fixed_string(e, name);
        true
    })
}

// Translated from 00eea4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigpuprogramcache.cpp`; `thiscall`): the address of the
/// object's +0xc (`SaveEntry` uses it on a material resource as the handle of its second
/// string).
pub fn fn_00eea4f0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xc))
}

// Translated from 00eea500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigpuprogramcache.cpp`; `thiscall`): the address of the
/// object's +0x10 (`SaveEntry` uses it on a material resource as the handle of its third
/// string).
pub fn fn_00eea500(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x10))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee9ca0, ni_gpu_program_cache_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00aa_1ab0, fn_00aa1ab0(Ptr) -> bool),
        entry!(
            0x00ee_8a70,
            ni_gpu_program_cache_get_valid_compile_targets_for_platform(Ptr) -> Ptr
        ),
        entry!(0x00ee_8a80, ni_gpu_program_cache_get_shader_profile(Ptr) -> Ptr),
        entry!(0x00ee_8a90, fn_00ee8a90(Ptr) -> Ptr),
        entry!(0x00ee_8aa0, ni_gpu_program_cache_is_loaded(Ptr) -> bool),
        entry!(0x00ee_8ab0, ni_gpu_program_cache_is_locked(Ptr) -> bool),
        entry!(0x00ee_8ac0, fn_00ee8ac0(Ptr) -> u32),
        entry!(
            0x00ee_8af0,
            ni_gpu_program_cache_set_working_directory(Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_8c10, ni_gpu_program_cache_compute_full_filename(Ptr)),
        entry!(0x00ee_8d20, fn_00ee8d20(Ptr) -> bool),
        entry!(
            0x00ee_8d60,
            ni_gpu_program_cache_append_entry(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_8e10, fn_00ee8e10(Ptr)),
        entry!(0x00ee_8e70, ni_gpu_program_cache_clear(Ptr) -> bool),
        entry!(
            0x00ee_8e90,
            ni_t_fixed_string_map_base_get_at(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_8f10, fn_00ee8f10(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ee_8fe0, fn_00ee8fe0(Ptr, u32)),
        entry!(0x00ee_9070, fn_00ee9070(Ptr, u32) -> Ptr),
        entry!(
            0x00ee_90b0,
            ni_t_fixed_string_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ee_90f0,
            ni_gpu_program_cache_prune_unused_constants(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_9230, fn_00ee9230(Ptr, Ptr) -> bool),
        entry!(
            0x00ee_92d0,
            ni_gpu_program_cache_load_string(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00ee_9350, ni_gpu_program_cache_load(Ptr) -> bool),
        entry!(0x00ee_9620, ni_gpu_program_cache_save(Ptr) -> bool),
        entry!(
            0x00ee_97e0,
            ni_gpu_program_cache_save_entry(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_99b0, ni_t_fixed_string_map_new_item(Ptr) -> Ptr),
        entry!(0x00ee_99d0, ni_t_fixed_string_map_delete_item(Ptr, Ptr)),
        entry!(0x00ee_9a60, fn_00ee9a60(Ptr)),
        entry!(
            0x00ee_9b10,
            ni_gpu_program_cache_find_cached_program(Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00ee_9bf0, fn_00ee9bf0(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00ee_9cc0,
            ni_gpu_program_cache_load_entry(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ee_a4f0, fn_00eea4f0(Ptr) -> Ptr),
        entry!(0x00ee_a500, fn_00eea500(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_gpu_program_cache_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ee_9a60, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ee9ca0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ee9ca0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x3c]]);
        assert_eq!(calls_to(&e, 0x00ee_9a60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod a06_thunk_tests {
    use super::*;

    /// Calls `thunk` with `words`, with the data cell `cell` holding the
    /// address of a double that returns `0xdeadbeef`; checks that the double
    /// ran once with exactly those words and that the thunk returned `expected`.
    fn pointer(thunk: u32, cell: u32, words: &[u32], expected: u32) {
        const TARGET: u32 = 0x00aa_1b60;
        let mut e = Engine::new();
        e.map(cell & !0xfff, 0x1000);
        e.set_global(cell, TARGET);
        e.register(TARGET, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_00aa1ab0() {
        pointer(0x00aa_1ab0, POINTER_CELL_00AA1AB0, &[0x1001], 1);
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The test vtable: slot `n` is the double at `slot(n)`, answering 1 unless a test
    /// registers another.
    const VTABLE: u32 = 0x0e10_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }
    /// The binary-stream functions of the test streams.
    const READ_DOUBLE: u32 = 0x0e20_0008;
    const WRITE_DOUBLE: u32 = 0x0e20_000c;

    type Chunks = Rc<RefCell<VecDeque<Vec<u8>>>>;
    type Written = Rc<RefCell<Vec<(Vec<u8>, u32)>>>;

    struct Rig {
        e: Engine,
        input: Chunks,
        written: Written,
    }

    fn rig() -> Rig {
        let mut e = engine_with_counting();
        let slots: Vec<u32> = (0..0x30).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x30u32 {
            e.register(slot(4 * i), |_, _| 1u32.into_ret());
        }
        // Fixed strings: a fresh handle (count 1 at -8) holding a copy of the text.
        e.register(GET_FIXED_STRING, |e, a| {
            let block = e.mem.alloc(0x40);
            e.mem.set_u32(block, 1);
            if a[0] != 0 {
                for i in 0..0x30 {
                    let byte = e.mem.u8(a[0] + i);
                    e.mem.set_u8(block + 8 + i, byte);
                    if byte == 0 {
                        break;
                    }
                }
            }
            (block + 8).into_ret()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        for double in [
            MEMORY_MANAGER_FREE,
            FREE_ARRAY,
            MEMSET,
            MAP_REMOVE_ALL,
            FREE_STRING_ARRAY_00A644A0,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let input: Chunks = Rc::default();
        let feed = input.clone();
        e.register_double(READ_DOUBLE, move |e, a| {
            let chunk = feed
                .borrow_mut()
                .pop_front()
                .expect("a read with nothing to give");
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            a[2].into_ret()
        });
        let written: Written = Rc::default();
        let sink = written.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.bytes(a[1], a[2]), e.mem.u32(a[3])));
            a[2].into_ret()
        });
        e.call_log = Some(vec![]);
        Rig { e, input, written }
    }

    impl Rig {
        /// A binary stream (with the test vtable) using the read and write doubles.
        fn stream(&mut self) -> u32 {
            let stream = self.e.mem.alloc(0x20);
            self.e.mem.set_u32(stream, VTABLE);
            self.e.mem.set_u32(stream + READ, READ_DOUBLE);
            self.e.mem.set_u32(stream + WRITE, WRITE_DOUBLE);
            stream
        }
        fn feed(&self, chunk: &[u8]) {
            self.input.borrow_mut().push_back(chunk.to_vec());
        }
        fn feed_word(&self, word: u32) {
            self.feed(&word.to_le_bytes());
        }
        fn writes(&self) -> Vec<(Vec<u8>, u32)> {
            self.written.borrow().clone()
        }
        /// A C string in test memory.
        fn text(&mut self, text: &str) -> u32 {
            let block = self.e.mem.alloc(text.len() as u32 + 1);
            self.e.mem.write(block, text.as_bytes());
            block
        }
        /// A counted fixed-string handle (count `count` at -8) for `text`.
        fn handle(&mut self, text: &str, count: u32) -> u32 {
            let block = self.e.mem.alloc(text.len() as u32 + 9);
            self.e.mem.set_u32(block, count);
            self.e.mem.write(block + 8, text.as_bytes());
            block + 8
        }
        /// A counted object (the test vtable, count `count`).
        fn object(&mut self, size: u32, count: u32) -> u32 {
            let object = self.e.mem.alloc(size.max(8));
            self.e.mem.set_u32(object, VTABLE);
            self.e.mem.set_u32(object + 4, count);
            object
        }
        /// A cache object with the test vtable, every fixed string empty, version 7.
        fn cache(&mut self) -> u32 {
            let this = self.e.mem.alloc(0x3c);
            self.e.mem.set_u32(this, VTABLE);
            let empty = self.e.mem.u32(EMPTY_FIXED_STRING);
            for offset in [
                FILE_NAME,
                WORKING_DIRECTORY,
                PATH_AND_FILE_NAME,
                SHADER_PROFILE,
            ] {
                self.e.mem.set_u32(this + offset, empty);
            }
            self.e.mem.set_u32(this + VERSION, 7);
            this
        }
        /// A counted handle (count 1) whose value falls in `bucket` of `size` buckets.
        fn bucket_handle(&mut self, bucket: u32, size: u32) -> u32 {
            let block = self.e.mem.alloc(size + 0x20);
            let start = block + 8;
            let handle = start + (bucket + size - start % size) % size;
            self.e.mem.set_u32(handle - 8, 1);
            handle
        }
        fn count(&self, handle: u32) -> u32 {
            self.e.mem.u32(handle - 8)
        }
    }

    /// A program map at `map` with `buckets` buckets holding `(bucket, key, value)` items
    /// (pushed to the front of their bucket in order).
    fn map(e: &mut Engine, map: u32, buckets: u32, items: &[(u32, u32, u32)]) -> Vec<u32> {
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        items
            .iter()
            .map(|&(bucket, key, value)| {
                let item = e.mem.alloc(0xc);
                let head = e.mem.u32(table + 4 * bucket);
                e.mem.set_u32(item, head);
                e.mem.set_u32(item + 4, key);
                e.mem.set_u32(item + 8, value);
                e.mem.set_u32(table + 4 * bucket, item);
                item
            })
            .collect()
    }

    #[test]
    fn accessors_answer_fields() {
        let mut r = rig();
        let this = r.cache();
        let e = &mut r.e;
        assert_eq!(e.call(0x00ee_8a70, &args![this]).u32(), this + 0x28);
        assert_eq!(e.call(0x00ee_8a80, &args![this]).u32(), this + 0x24);
        assert_eq!(e.call(0x00ee_8a90, &args![this]).u32(), this + 0x1c);
        assert_eq!(e.call(0x00ee_a4f0, &args![this]).u32(), this + 0xc);
        assert_eq!(e.call(0x00ee_a500, &args![this]).u32(), this + 0x10);
        e.mem.set_u8(this + LOADED, 1);
        assert!(e.call(0x00ee_8aa0, &args![this]).bool());
        assert!(!e.call(0x00ee_8ab0, &args![this]).bool());
        e.mem.set_u8(this + LOCKED, 2);
        assert!(e.call(0x00ee_8ab0, &args![this]).bool());
    }

    #[test]
    fn first_position_is_the_first_used_bucket() {
        let mut r = rig();
        let m = r.e.mem.alloc(0x10);
        assert_eq!(r.e.call(0x00ee_8ac0, &args![m]).u32(), 0);
        let items = map(&mut r.e, m, 5, &[(3, 1, 0), (2, 1, 0)]);
        assert_eq!(r.e.call(0x00ee_8ac0, &args![m]).u32(), items[1]);
    }

    #[test]
    fn set_working_directory_assigns_or_makes_absolute() {
        let mut r = rig();
        let this = r.cache();
        let old = r.handle("old", 1);
        r.e.mem.set_u32(this + WORKING_DIRECTORY, old);
        let new = r.handle("new", 1);
        let given = r.e.mem.alloc(4);
        r.e.mem.set_u32(given, new);
        r.e.register(PATH_IS_RELATIVE, |_, _| 0u32.into_ret());
        assert!(r.e.call(0x00ee_8af0, &args![this, given]).bool());
        assert_eq!(r.e.mem.u32(this + WORKING_DIRECTORY), new);
        assert_eq!((r.count(old), r.count(new)), (0, 2));
        assert_eq!(calls_to(&r.e, slot(0x80)), vec![vec![this]]);
        // The same directory again: no recompute.
        assert!(r.e.call(0x00ee_8af0, &args![this, given]).bool());
        assert_eq!(calls_to(&r.e, slot(0x80)).len(), 1);
        assert_eq!(r.count(new), 2);
        // Relative: made absolute and assigned from the buffer.
        r.e.register(PATH_IS_RELATIVE, |_, _| 1u32.into_ret());
        r.e.register(PATH_GET_CURRENT_WORKING_DIRECTORY, |_, _| 1u32.into_ret());
        r.e.register(PATH_CONVERT_TO_ABSOLUTE, |_, _| 1u32.into_ret());
        let other = r.handle("other", 1);
        r.e.register_double(FIXED_STRING_ASSIGN_00A2E750, move |e, a| {
            e.mem.set_u32(a[0], other);
            Ret::default()
        });
        assert!(r.e.call(0x00ee_8af0, &args![this, given]).bool());
        let absolute = calls_to(&r.e, PATH_CONVERT_TO_ABSOLUTE)[0].clone();
        let current = calls_to(&r.e, PATH_GET_CURRENT_WORKING_DIRECTORY)[0][0];
        assert_eq!(absolute[1..], [0x104, new, current]);
        assert_eq!(
            calls_to(&r.e, FIXED_STRING_ASSIGN_00A2E750),
            vec![vec![this + WORKING_DIRECTORY, absolute[0]]]
        );
        assert_eq!(calls_to(&r.e, slot(0x80)).len(), 2);
        // A failing conversion answers false with the copy released.
        r.e.register(PATH_CONVERT_TO_ABSOLUTE, |_, _| 0u32.into_ret());
        let before = r.count(other);
        assert!(!r.e.call(0x00ee_8af0, &args![this, given]).bool());
        assert_eq!(r.count(other), before);
        r.e.register(PATH_GET_CURRENT_WORKING_DIRECTORY, |_, _| 0u32.into_ret());
        assert!(!r.e.call(0x00ee_8af0, &args![this, given]).bool());
        assert_eq!(calls_to(&r.e, PATH_CONVERT_TO_ABSOLUTE).len(), 2);
    }

    #[test]
    fn compute_full_filename_joins_or_clears() {
        let mut r = rig();
        let this = r.cache();
        let path = r.handle("path", 1);
        r.e.mem.set_u32(this + PATH_AND_FILE_NAME, path);
        r.e.mem.set_u8(this + LOADED, 1);
        r.e.call(0x00ee_8c10, &args![this]);
        assert_eq!(r.count(path), 0);
        assert_ne!(r.e.mem.u32(this + PATH_AND_FILE_NAME), path);
        assert_eq!(r.e.mem.u8(this + LOADED), 0);
        let directory = r.handle("dir", 1);
        let name = r.handle("cache", 1);
        r.e.mem.set_u32(this + WORKING_DIRECTORY, directory);
        r.e.mem.set_u32(this + FILE_NAME, name);
        r.e.register(PATH_CONVERT_TO_ABSOLUTE, |e, a| {
            e.mem.write(a[0], b"dir/cache\0");
            1u32.into_ret()
        });
        r.e.call(0x00ee_8c10, &args![this]);
        let converted = calls_to(&r.e, PATH_CONVERT_TO_ABSOLUTE)[0].clone();
        assert_eq!(converted[1..], [0x104, name, directory]);
        let joined = r.e.mem.u32(this + PATH_AND_FILE_NAME);
        assert_eq!(r.e.mem.bytes(joined, 9), b"dir/cache".to_vec());
        assert_eq!(calls_to(&r.e, slot(0x70)), vec![vec![this]]);
    }

    #[test]
    fn working_directory_check_validates_or_creates() {
        let mut r = rig();
        let this = r.cache();
        assert!(!r.e.call(0x00ee_8d20, &args![this]).bool());
        let directory = r.handle("dir", 1);
        r.e.mem.set_u32(this + WORKING_DIRECTORY, directory);
        r.e.map(0x011a_c000, 0x1000);
        r.e.set_global(POINTER_CELL_00AA1AB0, 0x0e30_0000u32);
        r.e.register(0x0e30_0000, |_, _| 0u32.into_ret());
        r.e.register(CREATE_DIRECTORY_00AA1BB0, |_, _| 0x101u32.into_ret());
        assert!(r.e.call(0x00ee_8d20, &args![this]).bool());
        assert_eq!(
            calls_to(&r.e, CREATE_DIRECTORY_00AA1BB0),
            vec![vec![directory]]
        );
        r.e.register(0x0e30_0000, |_, _| 1u32.into_ret());
        assert!(r.e.call(0x00ee_8d20, &args![this]).bool());
        assert_eq!(calls_to(&r.e, CREATE_DIRECTORY_00AA1BB0).len(), 1);
    }

    #[test]
    fn append_entry_saves_into_the_file() {
        let mut r = rig();
        let this = r.cache();
        let file = r.object(0x10, 1);
        assert!(!r.e.call(0x00ee_8d60, &args![this, 1u32, 2u32]).bool());
        let path = r.handle("path", 1);
        r.e.mem.set_u32(this + PATH_AND_FILE_NAME, path);
        r.e.register(FILE_ACCESS, |_, a| u32::from(a[1] == 0).into_ret());
        r.e.register_double(FILE_GET_FILE, move |_, _| file.into_ret());
        r.e.register(slot(0x88), |_, _| 0x0e01u32.into_ret());
        assert!(r.e.call(0x00ee_8d60, &args![this, 0x11u32, 0x22u32]).bool());
        assert_eq!(calls_to(&r.e, FILE_GET_FILE), vec![vec![path, 2, 0x8000]]);
        assert_eq!(
            calls_to(&r.e, slot(0x88)),
            vec![vec![this, file, 0x11, 0x22]]
        );
        assert_eq!(calls_to(&r.e, slot(0)), vec![vec![file, 1]]);
        // A missing file is created by Save first; a failing Save stops.
        r.e.register(FILE_ACCESS, |_, a| u32::from(a[1] == 1).into_ret());
        r.e.register(slot(0x6c), |_, _| 0u32.into_ret());
        assert!(!r.e.call(0x00ee_8d60, &args![this, 0x11u32, 0x22u32]).bool());
        assert_eq!(calls_to(&r.e, slot(0x6c)), vec![vec![this]]);
        // A file that is not good is deleted.
        r.e.register(slot(0x6c), |_, _| 1u32.into_ret());
        r.e.register(slot(4), |_, _| 2u32.into_ret());
        assert!(!r.e.call(0x00ee_8d60, &args![this, 0x11u32, 0x22u32]).bool());
        assert_eq!(calls_to(&r.e, slot(0)).len(), 2);
        assert_eq!(calls_to(&r.e, slot(0x88)).len(), 1);
    }

    #[test]
    fn string_set_remove_all_and_clear() {
        let mut r = rig();
        let set = r.e.mem.alloc(0xc);
        let elements = r.e.mem.alloc(0xc);
        let a = r.handle("a", 2);
        r.e.mem.set_u32(elements, a);
        r.e.mem.set_u32(set, elements);
        r.e.mem.set_u32(set + 4, 3);
        r.e.mem.set_u32(set + 8, 1);
        r.e.call(0x00ee_8e10, &args![set]);
        assert_eq!(r.count(a), 1);
        assert_ne!(r.e.mem.u32(elements), a);
        assert_eq!(r.e.mem.u32(elements + 4), 0);
        assert_eq!(r.e.mem.u32(set + 8), 0);
        let this = r.cache();
        r.e.mem.set_u8(this + LOADED, 1);
        assert!(r.e.call(0x00ee_8e70, &args![this]).bool());
        assert_eq!(r.e.mem.u8(this + LOADED), 0);
        assert_eq!(calls_to(&r.e, MAP_REMOVE_ALL), vec![vec![this + 8]]);
    }

    #[test]
    fn get_at_finds_the_key_and_counts_the_value() {
        let mut r = rig();
        let m = r.e.mem.alloc(0x10);
        let desc = r.object(0x18, 1);
        map(&mut r.e, m, 5, &[(2, 7, 0x55), (2, 12, desc)]);
        let key = r.e.mem.alloc(4);
        let value = r.e.mem.alloc(4);
        r.e.mem.set_u32(key, 12);
        assert!(r.e.call(0x00ee_8e90, &args![m, key, value]).bool());
        assert_eq!(r.e.mem.u32(value), desc);
        assert_eq!(r.e.mem.u32(desc + 4), 2);
        r.e.mem.set_u32(key, 17);
        assert!(!r.e.call(0x00ee_8e90, &args![m, key, value]).bool());
    }

    #[test]
    fn get_next_walks_chains_then_buckets() {
        let mut r = rig();
        let m = r.e.mem.alloc(0x10);
        let (a, b, c) = (
            r.bucket_handle(1, 7),
            r.bucket_handle(1, 7),
            r.bucket_handle(5, 7),
        );
        let items = map(&mut r.e, m, 7, &[(1, a, 0), (1, b, 0), (5, c, 0)]);
        let position = r.e.mem.alloc(4);
        let key = r.e.mem.alloc(4);
        let value = r.e.mem.alloc(4);
        let empty = r.e.mem.u32(EMPTY_FIXED_STRING);
        r.e.mem.set_u32(key, empty);
        r.e.mem.set_u32(position, items[1]);
        r.e.call(0x00ee_8f10, &args![m, position, key, value]);
        assert_eq!(r.e.mem.u32(key), b);
        assert_eq!(r.count(b), 2);
        assert_eq!(r.e.mem.u32(position), items[0]);
        // The end of bucket 1's chain moves on to bucket 5.
        r.e.call(0x00ee_8f10, &args![m, position, key, value]);
        assert_eq!(r.e.mem.u32(position), items[2]);
        // The last item ends the walk; the key moved on to it.
        assert_eq!(r.e.mem.u32(key), a);
        assert_eq!((r.count(a), r.count(b)), (2, 1));
        r.e.call(0x00ee_8f10, &args![m, position, key, value]);
        assert_eq!(r.e.mem.u32(position), 0);
    }

    #[test]
    fn ordered_removal_shifts_and_releases() {
        let mut r = rig();
        let objects: Vec<u32> = (0..3).map(|_| r.object(8, 1)).collect();
        let elements = r.e.mem.alloc(0xc);
        for (i, o) in objects.iter().enumerate() {
            r.e.mem.set_u32(elements + 4 * i as u32, *o);
        }
        let set = r.e.mem.alloc(0xc);
        r.e.mem.set_u32(set, elements);
        r.e.mem.set_u32(set + 8, 3);
        r.e.call(0x00ee_8fe0, &args![set, 0u32]);
        assert_eq!(r.e.mem.u32(elements), objects[1]);
        assert_eq!(r.e.mem.u32(elements + 4), objects[2]);
        assert_eq!(r.e.mem.u32(elements + 8), 0);
        assert_eq!(r.e.mem.u32(set + 8), 2);
        assert_eq!(calls_to(&r.e, slot(4)), vec![vec![objects[0]]]);
        assert_eq!(r.e.mem.u32(objects[1] + 4), 1);
        assert_eq!(r.e.mem.u32(objects[2] + 4), 1);
    }

    #[test]
    fn map_destructors_empty_and_free() {
        let mut r = rig();
        let m = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(m + 8, 0x0e40_0000);
        assert_eq!(r.e.call(0x00ee_90b0, &args![m, 1u32]).u32(), m);
        assert_eq!(r.e.mem.u32(m), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&r.e, MAP_REMOVE_ALL), vec![vec![m]; 2]);
        assert_eq!(calls_to(&r.e, FREE_ARRAY), vec![vec![0x0e40_0000]]);
        assert_eq!(
            calls_to(&r.e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, m]]
        );
        r.e.call(0x00ee_9070, &args![m, 0u32]);
        assert_eq!(calls_to(&r.e, MAP_REMOVE_ALL).len(), 3);
        assert_eq!(calls_to(&r.e, MEMORY_MANAGER_FREE).len(), 1);
    }

    #[test]
    fn prune_removes_resources_no_variable_names() {
        let mut r = rig();
        let names = [r.text("alpha"), r.text("beta")];
        let program = r.object(8, 1);
        r.e.register(slot(0x48), |_, _| 2u32.into_ret());
        r.e.register_double(slot(0x4c), move |_, a| names[a[1] as usize].into_ret());
        // Resources: "beta" (as other text), null, "gamma", "alpha" (the same pointer).
        let beta = r.text("beta");
        let gamma = r.text("gamma");
        let variables = [beta, 0, gamma, names[0]];
        let elements = r.e.mem.alloc(0x10);
        let mut resources = vec![];
        for (i, variable) in variables.iter().enumerate() {
            let resource = if i == 1 { 0 } else { r.object(0x10, 1) };
            if resource != 0 {
                r.e.mem.set_u32(resource + 8, *variable);
            }
            r.e.mem.set_u32(elements + 4 * i as u32, resource);
            resources.push(resource);
        }
        r.e.register(RESOURCE_GET_VARIABLE, |_, a| (a[0] + 8).into_ret());
        let set = r.e.mem.alloc(0xc);
        r.e.mem.set_u32(set, elements);
        r.e.mem.set_u32(set + 8, 4);
        assert!(r
            .e
            .call(0x00ee_90f0, &args![0x5555u32, program, set])
            .bool());
        assert_eq!(r.e.mem.u32(set + 8), 2);
        assert_eq!(r.e.mem.u32(elements), resources[0]);
        assert_eq!(r.e.mem.u32(elements + 4), resources[3]);
        assert_eq!(calls_to(&r.e, MEMSET)[0][1..], [0, 0x400]);
    }

    #[test]
    fn strings_save_and_load_with_their_length() {
        let mut r = rig();
        let stream = r.stream();
        let text = r.text("abc");
        assert!(r.e.call(0x00ee_9230, &args![stream, text]).bool());
        assert!(r.e.call(0x00ee_9230, &args![stream, 0u32]).bool());
        assert_eq!(
            r.writes(),
            vec![
                (4u32.to_le_bytes().to_vec(), 4),
                (b"abc\0".to_vec(), 1),
                (0u32.to_le_bytes().to_vec(), 4)
            ]
        );
        assert_eq!(r.e.mem.u32(stream + POSITION), 12);
        // Loading: the length, then the bytes through 00a8d8b0 and a 0 one past them.
        let buffer = r.e.mem.alloc(0x10);
        r.e.mem.write(buffer, &[0xcc; 0x10]);
        r.e.register(LOAD_BYTES_00A8D8B0, |e, a| {
            e.mem.write(a[1], &vec![b'x'; a[2] as usize]);
            Ret::default()
        });
        r.feed_word(3);
        assert!(r
            .e
            .call(0x00ee_92d0, &args![stream, buffer, 0x10u32])
            .bool());
        assert_eq!(r.e.mem.bytes(buffer, 5), vec![b'x', b'x', b'x', 0xcc, 0]);
        r.feed_word(0);
        assert!(r
            .e
            .call(0x00ee_92d0, &args![stream, buffer, 0x10u32])
            .bool());
        assert_eq!(r.e.mem.u8(buffer), 0);
        assert!(!r.e.call(0x00ee_92d0, &args![stream, buffer, 0u32]).bool());
        assert!(!r.e.call(0x00ee_92d0, &args![stream, 0u32, 1u32]).bool());
    }

    /// A cache whose working directory validates and whose path is set, with a cache file
    /// (the test vtable) handed out by `GetFile`.
    fn ready_cache(r: &mut Rig) -> (u32, u32) {
        let this = r.cache();
        let directory = r.handle("dir", 1);
        let path = r.handle("path", 1);
        r.e.mem.set_u32(this + WORKING_DIRECTORY, directory);
        r.e.mem.set_u32(this + PATH_AND_FILE_NAME, path);
        r.e.map(0x011a_c000, 0x1000);
        r.e.set_global(POINTER_CELL_00AA1AB0, 0x0e30_0000u32);
        r.e.register(0x0e30_0000, |_, _| 1u32.into_ret());
        let file = r.stream();
        r.e.register_double(FILE_GET_FILE, move |_, _| file.into_ret());
        (this, file)
    }

    #[test]
    fn load_checks_the_header_and_reads_entries() {
        let mut r = rig();
        let (this, file) = ready_cache(&mut r);
        r.e.map(0x011c_2000, 0x1000);
        r.e.map(0x0110_1000, 0x1000);
        r.e.mem.write(TEXT_CACHE_FILE, b"NiGPUProgram Cache File\0");
        r.e.register(LOAD_BYTES_00A8D8B0, |_, _| Ret::default());
        r.e.register(NI_STRICMP, |_, _| 0u32.into_ret());
        r.e.register(LOAD_FLAG_00A57580, |_, _| Ret::default());
        r.e.register(LOAD_VERSION_00A575B0, |e, a| {
            e.mem.set_u32(a[1], 5);
            Ret::default()
        });
        // The file's position climbs by one per entry up to its size of 2.
        r.e.register(slot(0x1c), |_, _| 2u32.into_ret());
        r.e.register(slot(0xc), |e, a| {
            let at = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, at + 1);
            at.into_ret()
        });
        // The second entry reports itself out of date.
        r.e.register(slot(0x8c), |e, a| {
            if e.mem.u32(a[1] + 0x10) == 2 {
                e.mem.set_u8(a[2], 1);
            }
            1u32.into_ret()
        });
        let profile = r.e.mem.alloc(4);
        r.e.register_double(slot(0x54), move |_, _| profile.into_ret());
        for _ in 0..3 {
            r.feed_word(0);
        }
        r.e.mem.set_u8(OVERWRITE_DEPRECATED_CACHE_FILES, 1);
        assert!(r.e.call(0x00ee_9350, &args![this]).bool());
        assert_eq!(r.e.mem.u8(this + LOADED), 1);
        assert_eq!(calls_to(&r.e, slot(0x8c)).len(), 2);
        assert_eq!(calls_to(&r.e, slot(0x6c)), vec![vec![this]]);
        assert_eq!(calls_to(&r.e, LOAD_BYTES_00A8D8B0)[0][2], 24);
        assert_eq!(calls_to(&r.e, FILE_GET_FILE)[0][1..], [0, 0x8000]);
        assert_eq!(calls_to(&r.e, slot(0)), vec![vec![file, 1]]);
        // Loaded: true at once.
        assert!(r.e.call(0x00ee_9350, &args![this]).bool());
        assert_eq!(calls_to(&r.e, FILE_GET_FILE).len(), 1);
        // An old version drops the file and saves anew.
        r.e.mem.set_u8(this + LOADED, 0);
        r.e.register(LOAD_VERSION_00A575B0, |e, a| {
            e.mem.set_u32(a[1], 4);
            Ret::default()
        });
        assert!(!r.e.call(0x00ee_9350, &args![this]).bool());
        assert_eq!(calls_to(&r.e, slot(0x6c)).len(), 2);
        // A wrong header is refused.
        r.e.register(NI_STRICMP, |_, _| 1u32.into_ret());
        assert!(!r.e.call(0x00ee_9350, &args![this]).bool());
        assert_eq!(calls_to(&r.e, LOAD_VERSION_00A575B0).len(), 2);
    }

    #[test]
    fn save_writes_the_header_and_every_entry() {
        let mut r = rig();
        let (this, file) = ready_cache(&mut r);
        r.e.map(0x0110_1000, 0x1000);
        r.e.mem.write(TEXT_CACHE_FILE, b"NiGPUProgram Cache File\0");
        r.e.register(SAVE_BYTES_00E984D0, |_, _| Ret::default());
        let platform = r.text("xenon");
        let code = r.text("id");
        let profile = r.handle("vs_3_0", 1);
        let profile_at = r.e.mem.alloc(4);
        r.e.mem.set_u32(profile_at, profile);
        r.e.register_double(slot(0x64), move |_, _| platform.into_ret());
        r.e.register_double(slot(0x60), move |_, _| code.into_ret());
        r.e.register_double(slot(0x54), move |_, _| profile_at.into_ret());
        let desc = r.object(0x18, 1);
        let key = r.handle("prog", 1);
        map(&mut r.e, this + CACHED_PROGRAMS, 3, &[(key % 3, key, desc)]);
        assert!(r.e.call(0x00ee_9620, &args![this]).bool());
        assert_eq!(
            calls_to(&r.e, SAVE_BYTES_00E984D0),
            vec![vec![file, TEXT_CACHE_FILE, 24]]
        );
        let writes = r.writes();
        assert_eq!(writes[0], (vec![1], 1));
        assert_eq!(writes[1], (5u32.to_le_bytes().to_vec(), 4));
        assert_eq!(writes[3], (b"xenon\0".to_vec(), 1));
        assert_eq!(writes[7], (b"vs_3_0\0".to_vec(), 1));
        let entries = calls_to(&r.e, slot(0x88));
        assert_eq!(entries[0][..3], [this, file, desc]);
        // The key and the value were counted for the call and released after.
        assert_eq!((r.count(key), r.e.mem.u32(desc + 4)), (1, 1));
        assert_eq!(calls_to(&r.e, slot(0)), vec![vec![file, 1]]);
        // A file that is not good answers false without being deleted.
        r.e.register(slot(4), |_, _| 0u32.into_ret());
        assert!(!r.e.call(0x00ee_9620, &args![this]).bool());
        assert_eq!(calls_to(&r.e, slot(0)).len(), 1);
    }

    #[test]
    fn save_entry_writes_the_program_and_resources() {
        let mut r = rig();
        let this = r.cache();
        let file = r.stream();
        let resource = r.object(0x30, 1);
        let texts: Vec<u32> = ["n", "s", "t", "v", "d"]
            .iter()
            .map(|t| r.text(t))
            .collect();
        for (offset, text) in [
            (0x14u32, texts[0]),
            (0xc, texts[1]),
            (0x10, texts[2]),
            (0x18, texts[3]),
            (0x1c, texts[4]),
        ] {
            r.e.mem.set_u32(resource + offset, text);
        }
        r.e.register(RESOURCE_GETTER_00A62660, |_, a| (a[0] + 0x14).into_ret());
        r.e.register(RESOURCE_GET_VARIABLE, |_, a| (a[0] + 0x18).into_ret());
        r.e.register(RESOURCE_GETTER_00EEA520, |_, a| (a[0] + 0x1c).into_ret());
        for (getter, value) in [
            (RESOURCE_GETTER_00BE0AD0, 0x10u32),
            (RESOURCE_GETTER_00AA1AC0, 0x20),
            (RESOURCE_GETTER_00DBBC00, 0x30),
            (RESOURCE_GETTER_00EEA530, 0x40),
        ] {
            r.e.register_double(getter, move |_, _| value.into_ret());
        }
        let desc = r.object(0x18, 1);
        let elements = r.e.mem.alloc(4);
        r.e.mem.set_u32(elements, resource);
        r.e.mem.set_u32(desc + 8, 0x0e50_0000);
        r.e.mem.set_u32(desc + 0xc, elements);
        r.e.mem.set_u32(desc + 0x14, 1);
        let name = r.handle("entry", 1);
        let name_at = r.e.mem.alloc(4);
        r.e.mem.set_u32(name_at, name);
        assert!(r
            .e
            .call(0x00ee_97e0, &args![this, file, desc, name_at])
            .bool());
        assert_eq!(
            calls_to(&r.e, slot(0x90)),
            vec![vec![this, file, 0x0e50_0000]]
        );
        let writes = r.writes();
        assert_eq!(writes[0], (7u32.to_le_bytes().to_vec(), 4));
        assert_eq!(writes[2], (b"entry\0".to_vec(), 1));
        assert_eq!(writes[3], (1u32.to_le_bytes().to_vec(), 4));
        let strings: Vec<Vec<u8>> = writes[4..14]
            .iter()
            .skip(1)
            .step_by(2)
            .map(|w| w.0.clone())
            .collect();
        assert_eq!(
            strings,
            vec![
                b"n\0".to_vec(),
                b"s\0".to_vec(),
                b"t\0".to_vec(),
                b"v\0".to_vec(),
                b"d\0".to_vec()
            ]
        );
        let words: Vec<Vec<u8>> = writes[14..].iter().map(|w| w.0.clone()).collect();
        assert_eq!(
            words,
            [0x10u32, 0x20, 0x30, 0x40]
                .map(|v| v.to_le_bytes().to_vec())
                .to_vec()
        );
        // A failing program save stops after the name.
        r.e.register(slot(0x90), |_, _| 0u32.into_ret());
        let before = r.writes().len();
        assert!(!r
            .e
            .call(0x00ee_97e0, &args![this, file, desc, name_at])
            .bool());
        assert_eq!(r.writes().len(), before + 3);
    }

    #[test]
    fn map_items_are_made_and_deleted() {
        let mut r = rig();
        let item = r.e.call(0x00ee_99b0, &args![0x0e60_0000u32]).u32();
        assert_eq!(
            calls_to(&r.e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xc]]
        );
        assert_eq!((r.e.mem.u32(item + 4), r.e.mem.u32(item + 8)), (0, 0));
        let key = r.handle("k", 2);
        let value = r.object(8, 1);
        r.e.mem.set_u32(item + 4, key);
        r.e.mem.set_u32(item + 8, value);
        r.e.call(0x00ee_99d0, &args![0x0e60_0000u32, item]);
        assert_eq!(r.count(key), 1);
        assert_eq!(calls_to(&r.e, slot(4)), vec![vec![value]]);
        assert_eq!(
            calls_to(&r.e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, item]]
        );
    }

    #[test]
    fn constructor_and_destructor_body() {
        let mut r = rig();
        r.e.register(NEW_STRING_ARRAY_00A63AE0, |_, _| 0x0e70_0000u32.into_ret());
        let this = r.e.mem.alloc(0x3c);
        let back =
            r.e.call(0x00ee_9bf0, &args![this, 9u32, 0x101u32, 0x200u32]);
        assert_eq!(back.u32(), this);
        assert_eq!(r.e.mem.u32(this), NI_GPU_PROGRAM_CACHE_VTABLE);
        assert_eq!(r.e.mem.u32(this + 8), MAP_VTABLE);
        assert_eq!(r.e.mem.u32(this + 0xc), 0x25);
        assert_eq!(calls_to(&r.e, ALLOCATE_ARRAY), vec![vec![0x94]]);
        let table = r.e.mem.u32(this + 0x10);
        assert_eq!(calls_to(&r.e, MEMSET), vec![vec![table, 0, 0x94]]);
        let empty = r.e.mem.u32(EMPTY_FIXED_STRING);
        assert_eq!(r.e.mem.u32(this + WORKING_DIRECTORY), empty);
        assert_eq!(
            (
                r.e.mem.u32(this + 0x28),
                r.e.mem.u32(this + 0x2c),
                r.e.mem.u32(this + 0x30)
            ),
            (0x0e70_0000, 0xa, 0)
        );
        assert_eq!(r.e.mem.u32(this + VERSION), 9);
        assert_eq!([0x38, 0x39, 0x3a].map(|o| r.e.mem.u8(this + o)), [1, 0, 0]);
        assert_eq!(r.e.mem.u32(REF_OBJECT_COUNT), 1);
        // The destructor body undoes it.
        let path = r.handle("p", 2);
        r.e.mem.set_u32(this + PATH_AND_FILE_NAME, path);
        r.e.mem.set_u32(this + 0x2c, 0);
        r.e.call(0x00ee_9a60, &args![this]);
        assert_eq!(r.count(path), 1);
        assert_eq!(r.e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(r.e.mem.u32(this + 8), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&r.e, MAP_REMOVE_ALL), vec![vec![this + 8]; 2]);
        assert_eq!(
            calls_to(&r.e, FREE_STRING_ARRAY_00A644A0),
            vec![vec![0x0e70_0000]]
        );
        assert_eq!(calls_to(&r.e, FREE_ARRAY), vec![vec![table]]);
        assert_eq!(r.e.mem.u32(REF_OBJECT_COUNT), 0);
    }

    #[test]
    fn find_cached_program_adds_the_resources() {
        let mut r = rig();
        let this = r.cache();
        let desc = r.object(0x18, 1);
        let elements = r.e.mem.alloc(8);
        r.e.mem.set_u32(desc + 8, 0x0e50_0000);
        r.e.mem.set_u32(desc + 0xc, elements);
        r.e.mem.set_u32(desc + 0x14, 2);
        r.e.register(RESOURCE_SET_ADD, |_, _| Ret::default());
        let name = r.text("prog");
        // GET_FIXED_STRING hands out a fresh handle; make the map hold the next one.
        let handle = r.handle("prog", 1);
        r.e.register_double(GET_FIXED_STRING, move |_, _| handle.into_ret());
        map(&mut r.e, this + CACHED_PROGRAMS, 1, &[(0, handle, desc)]);
        let found = r.e.call(0x00ee_9b10, &args![this, name, 0x0e60_0000u32]);
        assert_eq!(found.u32(), 0x0e50_0000);
        assert_eq!(
            calls_to(&r.e, RESOURCE_SET_ADD),
            vec![vec![0x0e60_0000, elements], vec![0x0e60_0000, elements + 4]]
        );
        assert_eq!(r.e.mem.u32(desc + 4), 1);
        assert_eq!(r.count(handle), 0);
        // A null name looks up the empty string: not found.
        assert_eq!(
            r.e.call(0x00ee_9b10, &args![this, 0u32, 0x0e60_0000u32])
                .u32(),
            0
        );
    }

    /// Feeds one entry: version, name, then `count` resources of five strings and four
    /// words.
    fn feed_entry(r: &Rig, version: u32, name: &[u8], count: u32) {
        r.feed_word(version);
        r.feed_word(name.len() as u32);
        r.feed_word(count);
        for i in 0..count {
            for text in [&b"a"[..], b"bc", b"", b"d", b"e"] {
                r.feed_word(text.len() as u32);
                if !text.is_empty() {
                    r.feed(text);
                }
            }
            for word in 0..4 {
                r.feed_word(0x100 * i + word);
            }
        }
    }

    #[test]
    fn load_entry_makes_a_descriptor_with_its_resources() {
        let mut r = rig();
        let this = r.cache();
        map(&mut r.e, this + CACHED_PROGRAMS, 3, &[]);
        let stream = r.stream();
        r.e.register(LOAD_BYTES_00A8D8B0, |e, a| {
            e.mem.write(a[1], &vec![b'p'; a[2] as usize]);
            Ret::default()
        });
        r.e.register(slot(0x94), |e, a| {
            e.mem.set_u32(a[3], 0x0e50_0000);
            1u32.into_ret()
        });
        r.e.register(RESOURCE_CONSTRUCT_00EEA580, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u32(a[0] + 4, 1);
            a[0].into_ret()
        });
        for setter in RESOURCE_STRING_SETTERS
            .into_iter()
            .chain(RESOURCE_WORD_SETTERS)
        {
            r.e.register(setter, |_, _| Ret::default());
        }
        for double in [RESOURCE_SET_ADD, PROGRAM_ASSIGN_00A5F860] {
            r.e.register(double, |_, _| Ret::default());
        }
        r.e.register(MAP_SET_AT_00E8E1D0, |e, a| {
            assert_eq!(e.mem.u32(a[2] + 4), 1);
            Ret::default()
        });
        r.e.register(PROGRAM_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The stream double answers the count read later: feed in order.
        r.feed_word(7);
        r.feed_word(3);
        r.feed_word(1);
        for text in [&b"a"[..], b"bc", b"", b"d", b"e"] {
            r.feed_word(text.len() as u32);
            if !text.is_empty() {
                r.feed(text);
            }
        }
        for word in 0..4u32 {
            r.feed_word(0x30 + word);
        }
        let flag = r.e.mem.alloc(4);
        assert!(r.e.call(0x00ee_9cc0, &args![this, stream, flag]).bool());
        assert_eq!(r.e.mem.u8(flag), 0);
        assert!(r.input.borrow().is_empty());
        assert_eq!(calls_to(&r.e, LOAD_BYTES_00A8D8B0)[0][2], 3);
        let load = calls_to(&r.e, slot(0x94));
        assert_eq!((load[0][0], load[0][1], load[0][4]), (this, stream, 0));
        let added = calls_to(&r.e, RESOURCE_SET_ADD);
        assert_eq!(added.len(), 1);
        let set_at = calls_to(&r.e, MAP_SET_AT_00E8E1D0);
        assert_eq!(set_at[0][0], this + CACHED_PROGRAMS);
        let desc = set_at[0][2];
        assert_eq!(r.e.mem.u32(desc), PROGRAM_DESC_VTABLE);
        assert_eq!(added[0][0], desc + 0xc);
        let words: Vec<u32> = RESOURCE_WORD_SETTERS
            .iter()
            .map(|s| calls_to(&r.e, *s)[0][1])
            .collect();
        assert_eq!(words, vec![0x30, 0x31, 0x32, 0x33]);
        assert_eq!(calls_to(&r.e, RESOURCE_STRING_SETTERS[1]).len(), 1);
    }

    #[test]
    fn load_entry_skips_an_old_version_and_fails_with_the_program() {
        let mut r = rig();
        let this = r.cache();
        map(&mut r.e, this + CACHED_PROGRAMS, 3, &[]);
        let stream = r.stream();
        r.e.register(LOAD_BYTES_00A8D8B0, |_, _| Ret::default());
        feed_entry(&r, 6, b"", 2);
        let flag = r.e.mem.alloc(4);
        assert!(r.e.call(0x00ee_9cc0, &args![this, stream, flag]).bool());
        assert_eq!(r.e.mem.u8(flag), 1);
        assert!(r.input.borrow().is_empty());
        assert_eq!(calls_to(&r.e, slot(0x94))[0][4], 1);
        assert!(calls_to(&r.e, ALLOCATE).is_empty());
        // The program load failing answers false before the resources.
        r.e.register(slot(0x94), |_, _| 0u32.into_ret());
        r.feed_word(7);
        r.feed_word(0);
        assert!(!r.e.call(0x00ee_9cc0, &args![this, stream, flag]).bool());
        assert!(r.input.borrow().is_empty());
    }
}
