//! `fallout shared/tesidlemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ffbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600b90`, which stores the vtable `0104a060` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00600b90`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_005ffbd0(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0b90, &args![this]);
}

// Translated from 00600d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600d90`, a destructor with an exception frame
/// that stores the vtable `0104a070` and, when the byte at `+0x10` is set, frees the items
/// of the chains in the table at `+8`, and returns nothing. Parameters: one word, `this`;
/// the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00600d90`'s disassembly ends with its exception-frame unlink and returns no value
/// (Ghidra types it void).
pub fn fn_00600d70(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0d90, &args![this]);
}

// Translated from 00600b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104a040` (RTTI `.?AV?$BSTCaseInsensitiveStringMap@PAVIDLE_ANIM_ROOT@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00600d70`. One word, plain `RET`, returns nothing.
pub fn fn_00600b00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_a040);
    fn_00600d70(e, this);
}

/// `00406d30` (unnamed in the engine map): `cdecl`, three stack words (destination, size,
/// source): copies a string into a sized buffer.
const STRING_COPY: u32 = 0x0040_6d30;
/// `0040ab30` (`strrchr`, `magicitem.rs`): `cdecl`, the last occurrence of a character.
const STRRCHR: u32 = 0x0040_ab30;
/// `00853130` (unnamed in the engine map, `bgssaveloadreferencesmap.cpp`): `thiscall`, `RET 8`;
/// looks the key string up in a map and stores the entry's value through the second word,
/// answering whether it was found.
const MAP_LOOKUP: u32 = 0x0085_3130;
/// `"IdleAnims"`.
const IDLE_ANIMS_TEXT: u32 = 0x0101_70ec;
/// `00600560` (unnamed in the engine map): `thiscall`, `RET 4`; the root idle array registered
/// for a path, or null.
const GET_ROOT_IDLE_ARRAY: u32 = 0x0060_0560;
/// `00401000` (unnamed in the engine map): `cdecl`, one stack word; allocates that many bytes.
const ALLOCATE: u32 = 0x0040_1000;
/// `005ff890` (unnamed in the engine map): `thiscall`, `RET 4`; constructs a root idle array for
/// a path.
const ROOT_IDLE_ARRAY_CONSTRUCTOR: u32 = 0x005f_f890;
/// `005ff950` (unnamed in the engine map): `thiscall`, `RET 4`; hands the array its file name
/// list.
const ADD_ROOT_FILENAME_LIST: u32 = 0x005f_f950;
/// `00447850` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 8`; the
/// decompiler calls it `ModelLoader::CopyFilenameList(list, 0)`.
const MODEL_LOADER_COPY_FILENAME_LIST: u32 = 0x0044_7850;
/// `00447330` (unnamed in the engine map): `thiscall` on the `ModelLoader`; the decompiler calls
/// it `ModelLoader::BuildKFFileList(path, 1, 0, 0)`.
const MODEL_LOADER_BUILD_KF_FILE_LIST: u32 = 0x0044_7330;
/// `00470200` (unnamed in the engine map): `thiscall`, `RET 8`; registers the root idle array
/// under a path.
const REGISTER_ROOT_IDLE_ARRAY: u32 = 0x0047_0200;
/// The `ModelLoader` singleton (pointer global).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The byte global that is set while the file name list of a root is being built.
const BUILDING_ROOT_FLAG: u32 = 0x011c_b71c;

// Translated from 00600700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600700 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// `TESIdleManager::GetRootFilenameList` (Xbox PDB): `thiscall`, `RET 8` (a path and an index).
/// For a non-empty path, copies it into a 0x80-byte buffer, replaces its file name (everything
/// after the last backslash) with `"IdleAnims"`, looks that key up in the manager's map
/// (`00853130`) and answers the pointer stored at `index * 4 + 0x38` of the entry found; null
/// otherwise. The decompiler shows the lookup receiving the wrong arguments; the disassembly
/// passes the manager as `this`, the key and the address of the result.
pub fn tes_idle_manager_get_root_filename_list(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    index: u32,
) -> u32 {
    let mut result = 0;
    if !path.is_null() && e.mem.i8(path.addr()) != 0 {
        let buffer = e.mem.alloc(0x90);
        let entry_cell = e.mem.alloc(4);
        e.mem.set_u32(entry_cell, 0);
        e.call(STRING_COPY, &args![buffer, 0x80u32, path]);
        let separator = e.call(STRRCHR, &args![buffer, 0x5cu32]).u32();
        let room = 0x80u32
            .wrapping_sub(separator.wrapping_sub(buffer))
            .wrapping_sub(1);
        e.call(
            STRING_COPY,
            &args![separator.wrapping_add(1), room, IDLE_ANIMS_TEXT],
        );
        let found = e.call(MAP_LOOKUP, &args![this, buffer, entry_cell]).bool();
        let entry = e.mem.u32(entry_cell);
        if found && entry != 0 {
            result = e
                .mem
                .u32(entry.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x38));
        }
        e.mem.free(entry_cell);
        e.mem.free(buffer);
    }
    result
}

// Translated from 00600170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600170 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00efab6c, state variable) and the /GS stack-cookie check (00ec408c) are not translated
/// `TESIdleManager::AddRootIdleArray` (Xbox PDB): `thiscall`, `RET 0xc` (a path, an optional
/// file name list, a force byte). Does nothing (answers null) when the global flag at
/// `011cb71c` is set and `force` is zero, when the path is null or empty. Otherwise answers the
/// root idle array already registered for the path (`00600560`); when there is none, builds one
/// (`0x68` bytes, constructed with the path): from the given list copied by the `ModelLoader`
/// after copying the path into a 0x80-byte buffer whose file name becomes `"IdleAnims"` and
/// registering it under that buffer; or, without a list, from the `ModelLoader`'s animation file
/// list for the path (the global flag is set around that) and registering it under the path.
/// Answers the new array. The allocation result is checked for null as the exe does.
pub fn tes_idle_manager_add_root_idle_array(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    filenames: Ptr,
    force: u8,
) -> u32 {
    let mut result = 0;
    let building = e.global::<u8>(BUILDING_ROOT_FLAG);
    if (building == 0 || force != 0) && !path.is_null() && e.mem.i8(path.addr()) != 0 {
        result = e.call(GET_ROOT_IDLE_ARRAY, &args![this, path]).u32();
        if result == 0 {
            if !filenames.is_null() {
                let buffer = e.mem.alloc(0x90);
                e.call(STRING_COPY, &args![buffer, 0x80u32, path]);
                let separator = e.call(STRRCHR, &args![buffer, 0x5cu32]).u32();
                let room = 0x80u32
                    .wrapping_sub(separator.wrapping_sub(buffer))
                    .wrapping_sub(1);
                e.call(
                    STRING_COPY,
                    &args![separator.wrapping_add(1), room, IDLE_ANIMS_TEXT],
                );
                let array = new_root_idle_array(e, path);
                let loader = e.global::<u32>(MODEL_LOADER);
                let list = e
                    .call(
                        MODEL_LOADER_COPY_FILENAME_LIST,
                        &args![loader, filenames, 0u32],
                    )
                    .u32();
                e.call(ADD_ROOT_FILENAME_LIST, &args![array, list]);
                e.call(REGISTER_ROOT_IDLE_ARRAY, &args![this, buffer, array]);
                e.mem.free(buffer);
                result = array;
            } else {
                let array = new_root_idle_array(e, path);
                e.set_global::<u8>(BUILDING_ROOT_FLAG, 1);
                let loader = e.global::<u32>(MODEL_LOADER);
                let list = e
                    .call(
                        MODEL_LOADER_BUILD_KF_FILE_LIST,
                        &args![loader, path, 1u32, 0u32, 0u32],
                    )
                    .u32();
                e.call(ADD_ROOT_FILENAME_LIST, &args![array, list]);
                e.set_global::<u8>(BUILDING_ROOT_FLAG, 0);
                e.call(REGISTER_ROOT_IDLE_ARRAY, &args![this, path, array]);
                result = array;
            }
        }
    }
    result
}

/// `new RootIdleArray(path)`: allocates `0x68` bytes (`00401000`) and, if that worked,
/// constructs the array in them (`005ff890`); null otherwise.
fn new_root_idle_array(e: &mut Engine, path: Ptr) -> u32 {
    let block = e.call(ALLOCATE, &args![0x68u32]).u32();
    if block != 0 {
        e.call(ROOT_IDLE_ARRAY_CONSTRUCTOR, &args![block, path])
            .u32()
    } else {
        0
    }
}

/// `00863db0` (`thiscall` on a form array, no stack words): clears its entries (count at
/// +0xC) and sets the words +0xC and +0x10 to 0.
const FORM_ARRAY_CLEAR_00863DB0: u32 = 0x0086_3db0;

// Translated from 00600030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; called from the map's
/// `ClearData`): for each root of the map, deletes the forms of its `IdleArray` and then
/// its `LooseIdleArray` (virtual `+0x10` with 1, the count read once), clearing each array
/// right after its forms (`00863db0`). The roots and their file lists stay.
pub fn fn_00600030(e: &mut Engine, this: Ptr) {
    for_each_root(e, this, |e, root, _| {
        delete_forms(e, root);
        e.call(FORM_ARRAY_CLEAR_00863DB0, &args![root]);
        delete_forms(e, root + ROOT_LOOSE_IDLE_ARRAY);
        e.call(
            FORM_ARRAY_CLEAR_00863DB0,
            &args![root + ROOT_LOOSE_IDLE_ARRAY],
        );
    });
}

/// The used item's level (`01199c8c`).
const USED_ITEM_LEVEL: u32 = 0x0119_9c8c;

// Translated from 006005c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006005c0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESIdleManager::GetRootIdleArray_ov2` (Xbox PDB; `RET 4`: a model):
/// null without a model; otherwise the root idle array of the model's idle
/// directory (`TESModel::GetModel`, `TESIdleManager::GetIdleDirName` into a
/// 0x104-byte stack buffer, `TESIdleManager::GetRootIdleArray`).
pub fn tes_idle_manager_get_root_idle_array_ov2(e: &mut Engine, this: Ptr, model: u32) -> u32 {
    if model == 0 {
        return 0;
    }
    e.with_stack(0x108, |e, directory| {
        let path = e.call(MODEL_GET_MODEL, &args![model]).u32();
        tes_idle_manager_get_idle_dir_name(e, Ptr::new(path), directory);
        tes_idle_manager_get_root_idle_array(e, this, directory)
    })
}

// Translated from 00600910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The used item's level (the word at `01199c8c`; the engine map has no
/// name; `TESIdleManager::SetUsedItemLevel` sets it).
pub fn fn_00600910(e: &mut Engine) -> u32 {
    e.global::<u32>(USED_ITEM_LEVEL)
}

// Translated from 00600920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::SetUsedItemLevel` (Xbox PDB; cdecl, one word): the
/// word at `01199c8c`.
pub fn tes_idle_manager_set_used_item_level(e: &mut Engine, level: u32) {
    e.set_global(USED_ITEM_LEVEL, level);
}

// Translated from 00600940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::SetUsedItemActivate` (Xbox PDB; cdecl, one word whose
/// low byte is stored): the byte at `011cb6a8`.
pub fn tes_idle_manager_set_used_item_activate(e: &mut Engine, activate: u8) {
    e.set_global(USED_ITEM_ACTIVATE, activate);
}

/// The scalar deleting destructors below: the body, then the free
/// (`00401030`) when bit 0 of the flags is set. Returns the object.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, body: fn(&mut Engine, Ptr)) -> Ptr {
    body(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00600d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<IDLE_ANIM_ROOT *>` scalar deleting
/// destructor (engine map; `RET 4`: the flags): [`fn_00600b00`], then the
/// free when bit 0 is set.
pub fn bst_case_insensitive_string_map_idle_anim_root_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00600b00)
}

// Translated from 00600ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<char const *,IDLE_ANIM_ROOT *>>,char const
/// *,IDLE_ANIM_ROOT *>` scalar deleting destructor (engine map; `RET 4`):
/// [`fn_00600eb0`], then the free when bit 0 is set.
pub fn ni_t_map_base_idle_anim_root_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00600eb0)
}

// Translated from 00600f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<char const *,IDLE_ANIM_ROOT *>` scalar deleting destructor
/// (engine map; `RET 4`): [`fn_00600e50`], then the free when bit 0 is
/// set.
pub fn ni_t_map_idle_anim_root_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00600e50)
}

// Translated from 00600f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<char const *,IDLE_ANIM_ROOT *>,IDLE_ANIM_ROOT
/// *>` scalar deleting destructor (engine map; `RET 4`): [`fn_00600d90`],
/// then the free when bit 0 is set.
pub fn ni_t_string_template_map_idle_anim_root_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, fn_00600d90)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0060_05c0,
            tes_idle_manager_get_root_idle_array_ov2(Ptr, u32) -> u32
        ),
        entry!(0x0060_0910, fn_00600910() -> u32),
        entry!(0x0060_0920, tes_idle_manager_set_used_item_level(u32)),
        entry!(0x0060_0940, tes_idle_manager_set_used_item_activate(u8)),
        entry!(
            0x0060_0d40,
            bst_case_insensitive_string_map_idle_anim_root_p_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0060_0ee0,
            ni_t_map_base_idle_anim_root_p_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0060_0f10,
            ni_t_map_idle_anim_root_p_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0060_0f40,
            ni_t_string_template_map_idle_anim_root_p_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0060_0030, fn_00600030(Ptr)),
        entry!(0x0060_0700, tes_idle_manager_get_root_filename_list(Ptr, Ptr, u32) -> u32),
        entry!(0x0060_0170, tes_idle_manager_add_root_idle_array(Ptr, Ptr, Ptr, u8) -> u32),
        entry!(0x0060_0b00, fn_00600b00(Ptr)),
        entry!(0x005f_fbd0, fn_005ffbd0(Ptr)),
        entry!(0x0060_0d70, fn_00600d70(Ptr)),
        entry!(0x005f_f890, fn_005ff890(Ptr, Ptr) -> Ptr),
        entry!(0x005f_f950, idle_anim_root_add_root_filename_list(Ptr, Ptr)),
        entry!(0x005f_faf0, fn_005ffaf0(Ptr) -> Ptr),
        entry!(0x005f_fb60, fn_005ffb60(Ptr)),
        entry!(0x005f_fbf0, fn_005ffbf0(Ptr, u32)),
        entry!(0x005f_fc20, fn_005ffc20(Ptr)),
        entry!(0x005f_fd20, fn_005ffd20(Ptr)),
        entry!(0x005f_ff90, fn_005fff90(Ptr, u32) -> Ptr),
        entry!(0x005f_ffc0, fn_005fffc0(Ptr)),
        entry!(
            0x0060_03c0,
            tes_idle_manager_rebuild_root_filename_lists(Ptr)
        ),
        entry!(
            0x0060_04f0,
            tes_idle_manager_add_root_idle_array_ov2(Ptr, Ptr) -> u32
        ),
        entry!(
            0x0060_0560,
            tes_idle_manager_get_root_idle_array(Ptr, Ptr) -> u32
        ),
        entry!(
            0x0060_0630,
            tes_idle_manager_get_loose_idle_array(Ptr, Ptr) -> u32
        ),
        entry!(
            0x0060_0690,
            tes_idle_manager_get_loose_idle_array_ov2(Ptr, Ptr) -> u32
        ),
        entry!(0x0060_07f0, tes_idle_manager_get_idle_dir_name(Ptr, Ptr)),
        entry!(0x0060_08f0, tes_idle_manager_get_used_item() -> u32),
        entry!(0x0060_0900, tes_idle_manager_set_used_item(u32)),
        entry!(0x0060_0930, fn_00600930() -> u8),
        entry!(
            0x0060_0950,
            tes_idle_manager_get_idle_to_play(Ptr, Ptr, u32) -> u32
        ),
        entry!(0x0060_0ad0, fn_00600ad0(Ptr, u32, u8) -> Ptr),
        entry!(0x0060_0d10, fn_00600d10(Ptr, u32, u32) -> Ptr),
        entry!(0x0060_0d90, fn_00600d90(Ptr)),
        entry!(0x0060_0e50, fn_00600e50(Ptr)),
        entry!(0x0060_0eb0, fn_00600eb0(Ptr)),
    ]
}

/// `IDLE_ANIM_ROOT` (Xbox PDB), 0x68 bytes: `IdleArray` (`NiFormArray`,
/// `+0`), `LooseIdleArray` (`+0x18`), `IdleDir` (`BSStringT`, `+0x30`) and
/// `pKFFileList` (`+0x38`: twelve `BSSimpleList<const char*>*`, one per
/// animation group kind). The PC code uses the same offsets.
const ROOT_LOOSE_IDLE_ARRAY: u32 = 0x18;
const ROOT_IDLE_DIR: u32 = 0x30;
const ROOT_KF_FILE_LISTS: u32 = 0x38;
/// The number of `pKFFileList` slots.
const ROOT_KF_FILE_LIST_COUNT: u32 = 12;

/// `TESIdleManager` (Xbox PDB): `IdleArrayMap` at `+0`, `bInitialized` at
/// `+0x14`, `LoadedIdleArray` (`NiTPrimitiveArray<TESIdleForm*>`) at
/// `+0x18`.
const MANAGER_INITIALIZED: u32 = 0x14;
const MANAGER_LOADED_IDLE_ARRAY: u32 = 0x18;
/// `TESIdleManager::pUsedItem` (Xbox PDB static).
const USED_ITEM: u32 = 0x011c_b6a4;
/// The byte static after `iUsedItemLevel` (`bActivate` in the Xbox PDB's
/// order of the statics; the PC address is not confirmed by a name).
const USED_ITEM_ACTIVATE: u32 = 0x011c_b6a8;

/// `NiFormArray` constructor and destructor bodies (`0058b3f0`,
/// `0058b450`, thiscall).
const FORM_ARRAY_CONSTRUCT: u32 = 0x0058_b3f0;
const FORM_ARRAY_DESTRUCT: u32 = 0x0058_b450;
/// `NiFormArray` calls (thiscall(1)): `00864280` and `00600ab0`.
const FORM_ARRAY_CALL_864280: u32 = 0x0086_4280;
const FORM_ARRAY_CALL_600AB0: u32 = 0x0060_0ab0;
/// The array's count (`0084e3a0`) and `NiFormArray::GetFormAt` (engine map).
const FORM_ARRAY_COUNT: u32 = 0x0084_e3a0;
const FORM_ARRAY_GET_FORM_AT: u32 = 0x004d_6170;
/// `BSStringT` constructor, set (thiscall(text, 0)) and destructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `memset` (`00403d30`, cdecl(destination, value, size)).
const MEMSET: u32 = 0x0040_3d30;
/// `TESAnimGroup::ParseAnimGroupFilename` (engine map; cdecl(name, out,
/// group out, byte out)).
const PARSE_ANIM_GROUP_FILENAME: u32 = 0x005f_38d0;
/// `strrchr` (`00ec6e30`) and `_stricmp` (`00ec68e4`), cdecl.
const STRRCHR_CRT: u32 = 0x00ec_6e30;
const STRICMP: u32 = 0x00ec_68e4;
/// `"1HPLeft.kf"`, `"1HPFastLeft.kf"`, `"1HMLeft.kf"`, `"1HMFastLeft.kf"`:
/// the file names of group kind 4 that go to kind 0.
const LEFT_HAND_NAMES: [u32; 4] = [0x0104_a030, 0x0104_a020, 0x0104_a014, 0x0104_a004];
/// The animation group kind those names are moved out of.
const LEFT_HAND_KIND: u32 = 4;
/// The `BSSimpleList` construction on a new 8-byte node (`0096a2d0`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The list's add (`005ae3d0`, thiscall(item address)), end test
/// (`008256d0`), item address (`006815c0`) and head removal (`0063f7b0`).
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list's scalar deleting destructor (`004702f0`, `unplaced/run_00470040`).
const LIST_DELETE: u32 = 0x0047_02f0;
/// `operator delete` (cdecl(block)).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The string map's constructor (`00600fd0`, thiscall(size, copy flag),
/// `run_00600f70`) and the `NiTArray<TESIdleForm*>` one (`00601000`).
const STRING_MAP_CONSTRUCT: u32 = 0x0060_0fd0;
const IDLE_FORM_ARRAY_CONSTRUCT: u32 = 0x0060_1000;
/// `BSTCaseInsensitiveStringMap<IDLE_ANIM_ROOT*>` and
/// `NiTPrimitiveArray<TESIdleForm*>` vtables (RTTI).
const ROOT_MAP_VTABLE: u32 = 0x0104_a040;
const IDLE_FORM_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0104_a068;
/// `NiTStringTemplateMap<..>`, `NiTMap<const char*,IDLE_ANIM_ROOT*>` and
/// `NiTMapBase<..>` vtables (RTTI).
const STRING_TEMPLATE_MAP_VTABLE: u32 = 0x0104_a070;
const NI_T_MAP_VTABLE: u32 = 0x0104_a090;
const NI_T_MAP_BASE_VTABLE: u32 = 0x0104_a0b0;
/// The map's `RemoveAll` (`00438af0`, thiscall) and the free of a block
/// (`00aa10f0`, cdecl).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const MAP_FREE: u32 = 0x00aa_10f0;
/// The map's first position (`004b9ba0`) and its step (`006b7f20`,
/// thiscall(position address, key out, value out)).
const MAP_FIRST: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
/// `NiTPrimitiveArray<TESIdleForm*>` calls on `LoadedIdleArray`: its size
/// (`00658930`), an element's address (`00877a30`, thiscall(index)),
/// `0096ae90` (thiscall(index, value address)), `00600bc0`, `005e03d0`,
/// `0096a610` (thiscall(item address)).
const IDLE_ARRAY_SIZE: u32 = 0x0065_8930;
const IDLE_ARRAY_ELEMENT: u32 = 0x0087_7a30;
const IDLE_ARRAY_SET_AT: u32 = 0x0096_ae90;
const IDLE_ARRAY_CALL_600BC0: u32 = 0x0060_0bc0;
const IDLE_ARRAY_CALL_5E03D0: u32 = 0x005e_03d0;
const IDLE_ARRAY_ADD: u32 = 0x0096_a610;
/// `004013e0` (thiscall on a form, bool) and `TESIdleForm::InitParents`
/// (engine map).
const FORM_TEST_4013E0: u32 = 0x0040_13e0;
const IDLE_FORM_INIT_PARENTS: u32 = 0x005f_edb0;
/// `TESModel::GetModel` (engine map) and `TESObjectREFR::GetModel`
/// (engine map).
const MODEL_GET_MODEL: u32 = 0x0048_95b0;
const REFERENCE_GET_MODEL: u32 = 0x0057_15d0;
/// The idle form checks of `GetIdleToPlay`: `005ff2e0` (thiscall(actor,
/// word, process part)), `005ff1d0` (bool) and
/// `TESIdleForm::CheckIdleFile` (engine map, bool).
const IDLE_FORM_PICK: u32 = 0x005f_f2e0;
const IDLE_FORM_TEST_5FF1D0: u32 = 0x005f_f1d0;
const IDLE_FORM_CHECK_IDLE_FILE: u32 = 0x005f_f280;
/// `00406d50` (`strcat_s`-shaped, cdecl(destination, size, source)) and
/// `004812f0` (cdecl(text, part): where `part` occurs in `text`, or null)
/// and the C string length (`0044a670`).
const STRING_APPEND: u32 = 0x0040_6d50;
const STRING_FIND: u32 = 0x0048_12f0;
const C_STRING_LENGTH: u32 = 0x0044_a670;
/// The 0x104-byte path buffers on the stack.
const PATH_SIZE: u32 = 0x104;

// Translated from 005ff890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ff890 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `IDLE_ANIM_ROOT` constructor (thiscall(path)): both form arrays
/// (`0058b3f0`) and the directory string (`004037b0`, then set to `path`),
/// `00864280(1)` and `00600ab0(1)` on each array, and the twelve file list
/// pointers cleared (`memset`). Returns `this`.
pub fn fn_005ff890(e: &mut Engine, this: Ptr, path: Ptr) -> Ptr {
    let base = this.addr();
    e.call(FORM_ARRAY_CONSTRUCT, &args![this]);
    e.call(FORM_ARRAY_CONSTRUCT, &args![base + ROOT_LOOSE_IDLE_ARRAY]);
    e.call(STRING_CONSTRUCT, &args![base + ROOT_IDLE_DIR]);
    e.call(STRING_SET, &args![base + ROOT_IDLE_DIR, path, 0u32]);
    e.call(FORM_ARRAY_CALL_864280, &args![this, 1u32]);
    e.call(FORM_ARRAY_CALL_600AB0, &args![this, 1u32]);
    e.call(
        FORM_ARRAY_CALL_864280,
        &args![base + ROOT_LOOSE_IDLE_ARRAY, 1u32],
    );
    e.call(
        FORM_ARRAY_CALL_600AB0,
        &args![base + ROOT_LOOSE_IDLE_ARRAY, 1u32],
    );
    e.call(MEMSET, &args![base + ROOT_KF_FILE_LISTS, 0u32, 0x30u32]);
    this
}

// Translated from 005ff950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ff950 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `IDLE_ANIM_ROOT::AddRootFilenameList` (Xbox PDB): moves every file name
/// of `list` (null: nothing) into the root's file list of its animation
/// group kind (`TESAnimGroup::ParseAnimGroupFilename`; kind 4 files named
/// `1HPLeft.kf`, `1HPFastLeft.kf`, `1HMLeft.kf` or `1HMFastLeft.kf` after
/// their last backslash, compared without case, go to kind 0), making the
/// kind's list (an 8-byte `BSSimpleList`) when it has none; then deletes
/// `list` (`004702f0(1)`).
pub fn idle_anim_root_add_root_filename_list(e: &mut Engine, this: Ptr, list: Ptr) {
    if list.is_null() {
        return;
    }
    let base = this.addr();
    e.with_stack(4, |e, name| {
        e.with_stack(4, |e, out| {
            e.with_stack(4, |e, kind| {
                e.with_stack(4, |e, flag| {
                    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                        let item = e.call(LIST_ITEM, &args![list]).u32();
                        let file = e.mem.u32(item);
                        e.mem.set_u32(name.addr(), file);
                        e.call(PARSE_ANIM_GROUP_FILENAME, &args![file, out, kind, flag]);
                        if e.mem.u32(kind.addr()) == LEFT_HAND_KIND {
                            let file = e.mem.u32(name.addr());
                            let separator = e.call(STRRCHR_CRT, &args![file, 0x5cu32]).u32();
                            if separator != 0 {
                                let file_name = separator.wrapping_add(1);
                                let mut left_hand = false;
                                for text in LEFT_HAND_NAMES {
                                    if e.call(STRICMP, &args![file_name, text]).u32() == 0 {
                                        left_hand = true;
                                        break;
                                    }
                                }
                                if left_hand {
                                    e.mem.set_u32(kind.addr(), 0);
                                }
                            }
                        }
                        let group = e.mem.u32(kind.addr());
                        let slot = base
                            .wrapping_add(ROOT_KF_FILE_LISTS)
                            .wrapping_add(group.wrapping_mul(4));
                        if e.mem.u32(slot) == 0 {
                            let block = e.call(ALLOCATE, &args![8u32]).u32();
                            let made = if block == 0 {
                                0
                            } else {
                                e.call(LIST_CONSTRUCT, &args![block]).u32()
                            };
                            e.mem.set_u32(slot, made);
                        }
                        let target = e.mem.u32(slot);
                        e.call(LIST_ADD, &args![target, name]);
                        e.call(LIST_REMOVE_HEAD, &args![list]);
                    }
                });
            });
        });
    });
    // The list was checked for null on entry; the code tests it again.
    if !list.is_null() {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

// Translated from 005ffaf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ffaf0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `TESIdleManager` constructor: the root map (`00600ad0(0x25, 1)`), the
/// loaded idle array (`00600d10(0, 1)` on `+0x18`) and `bInitialized`
/// cleared. Returns `this`.
pub fn fn_005ffaf0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00600ad0(e, this, 0x25, 1);
    fn_00600d10(e, Ptr::new(this.addr() + MANAGER_LOADED_IDLE_ARRAY), 0, 1);
    e.mem.set_u8(this.addr() + MANAGER_INITIALIZED, 0);
    this
}

// Translated from 005ffb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ffb60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `TESIdleManager` destructor body: releases everything (`005ffd20`),
/// then the loaded idle array's (`005ffbd0`) and the map's (`00600b00`)
/// destructors.
pub fn fn_005ffb60(e: &mut Engine, this: Ptr) {
    fn_005ffd20(e, this);
    fn_005ffbd0(e, Ptr::new(this.addr() + MANAGER_LOADED_IDLE_ARRAY));
    fn_00600b00(e, this);
}

// Translated from 005ffbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a non-null `idle` to the loaded idle array (`0096a610` with the
/// address of the argument word).
pub fn fn_005ffbf0(e: &mut Engine, this: Ptr, idle: u32) {
    if idle == 0 {
        return;
    }
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), idle);
        e.call(
            IDLE_ARRAY_ADD,
            &args![this.addr() + MANAGER_LOADED_IDLE_ARRAY, item],
        );
    });
}

// Translated from 005ffc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the loaded idles: each slot of the loaded idle array whose
/// form is null or for which `004013e0` says so is cleared (`0096ae90` with
/// a 0 word), the others get their virtual `+0x88`; then each idle for which
/// `TESIdleForm::InitParents` says so is cleared; then `00600bc0` on the
/// array and `bInitialized` set. The count is read once.
pub fn fn_005ffc20(e: &mut Engine, this: Ptr) {
    let array = this.addr() + MANAGER_LOADED_IDLE_ARRAY;
    let count = e.call(IDLE_ARRAY_SIZE, &args![array]).u32();
    let clear = |e: &mut Engine, index: u32| {
        e.with_stack(4, |e, zero| {
            e.mem.set_u32(zero.addr(), 0);
            e.call(IDLE_ARRAY_SET_AT, &args![array, index, zero]);
        });
    };
    let mut index = 0u32;
    while index < count {
        let slot = e.call(IDLE_ARRAY_ELEMENT, &args![array, index]).u32();
        let idle = e.mem.u32(slot);
        if idle == 0 || e.call(FORM_TEST_4013E0, &args![idle]).bool() {
            clear(e, index);
        } else {
            e.vcall(idle, 0x88, &args![]);
        }
        index += 1;
    }
    let mut index = 0u32;
    while index < count {
        let slot = e.call(IDLE_ARRAY_ELEMENT, &args![array, index]).u32();
        let idle = e.mem.u32(slot);
        if idle != 0 && e.call(IDLE_FORM_INIT_PARENTS, &args![idle]).bool() {
            clear(e, index);
        }
        index += 1;
    }
    e.call(IDLE_ARRAY_CALL_600BC0, &args![array]);
    e.mem.set_u8(this.addr() + MANAGER_INITIALIZED, 1);
}

/// Each form of a form array (count read once) is deleted through its
/// virtual `+0x10` with 1 when not null.
fn delete_forms(e: &mut Engine, array: u32) {
    let count = e.call(FORM_ARRAY_COUNT, &args![array]).u32();
    let mut index = 0u32;
    while index < count {
        let form = e.call(FORM_ARRAY_GET_FORM_AT, &args![array, index]).u32();
        if form != 0 {
            e.vcall(form, 0x10, &args![1u32]);
        }
        index += 1;
    }
}

/// Empties a root's twelve file lists: each name is deleted
/// (`operator delete`) and removed from its list, the list deleted
/// (`004702f0(1)`); with `clear`, the slot is cleared afterwards.
fn release_file_lists(e: &mut Engine, root: u32, clear: bool) {
    let mut index = 0u32;
    while index < ROOT_KF_FILE_LIST_COUNT {
        let slot = root
            .wrapping_add(ROOT_KF_FILE_LISTS)
            .wrapping_add(index.wrapping_mul(4));
        if e.mem.u32(slot) != 0 {
            loop {
                let list = e.mem.u32(slot);
                if e.call(LIST_IS_EMPTY, &args![list]).bool() {
                    break;
                }
                let item = e.call(LIST_ITEM, &args![list]).u32();
                let name = e.mem.u32(item);
                e.call(OPERATOR_DELETE, &args![name]);
                let list = e.mem.u32(slot);
                e.call(LIST_REMOVE_HEAD, &args![list]);
            }
            let list = e.mem.u32(slot);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
            if clear {
                e.mem.set_u32(slot, 0);
            }
        }
        index += 1;
    }
}

/// Walks the root map: `f(root, key)` for each entry with a root (the key
/// and value words cleared before each step).
fn for_each_root(e: &mut Engine, this: Ptr, mut f: impl FnMut(&mut Engine, u32, u32)) {
    let first = e.call(MAP_FIRST, &args![this]).u32();
    e.with_stack(4, |e, position| {
        e.mem.set_u32(position.addr(), first);
        e.with_stack(4, |e, key| {
            e.with_stack(4, |e, value| {
                while e.mem.u32(position.addr()) != 0 {
                    e.mem.set_u32(value.addr(), 0);
                    e.mem.set_u32(key.addr(), 0);
                    e.call(MAP_NEXT, &args![this, position, key, value]);
                    let root = e.mem.u32(value.addr());
                    if root != 0 {
                        let name = e.mem.u32(key.addr());
                        f(e, root, name);
                    }
                }
            });
        });
    });
}

// Translated from 005ffd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases everything the manager holds: for each root of the map, the
/// forms of both its arrays (virtual `+0x10` with 1), its file lists, and
/// the root itself (`005fff90(1)`); then the loaded idles (virtual `+0x10`
/// with 1), `005e03d0` on the loaded idle array and the map's `RemoveAll`.
pub fn fn_005ffd20(e: &mut Engine, this: Ptr) {
    for_each_root(e, this, |e, root, _| {
        delete_forms(e, root);
        delete_forms(e, root + ROOT_LOOSE_IDLE_ARRAY);
        release_file_lists(e, root, false);
        if root != 0 {
            fn_005fff90(e, Ptr::new(root), 1);
        }
    });
    let array = this.addr() + MANAGER_LOADED_IDLE_ARRAY;
    let count = e.call(IDLE_ARRAY_SIZE, &args![array]).u32();
    let mut index = 0u32;
    while index < count {
        let slot = e.call(IDLE_ARRAY_ELEMENT, &args![array, index]).u32();
        let idle = e.mem.u32(slot);
        if idle != 0 {
            e.vcall(idle, 0x10, &args![1u32]);
        }
        index += 1;
    }
    e.call(IDLE_ARRAY_CALL_5E03D0, &args![array]);
    e.call(MAP_REMOVE_ALL, &args![this]);
}

// Translated from 005fff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IDLE_ANIM_ROOT` scalar deleting destructor: the destructor body
/// (`005fffc0`), then `operator delete(this)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_005fff90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fffc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005fffc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fffc0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `IDLE_ANIM_ROOT` destructor body: the directory string, then the
/// loose and the main form array (`0058b450`).
pub fn fn_005fffc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(STRING_DESTRUCT, &args![base + ROOT_IDLE_DIR]);
    e.call(FORM_ARRAY_DESTRUCT, &args![base + ROOT_LOOSE_IDLE_ARRAY]);
    e.call(FORM_ARRAY_DESTRUCT, &args![this]);
}

// Translated from 006003c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::RebuildRootFilenameLists` (Xbox PDB): for each root of
/// the map, empties its file lists (each slot cleared), then hands it the
/// `ModelLoader`'s animation file list for its key
/// (`BuildKFFileList(key, 1, 0, 0)`, `AddRootFilenameList`).
pub fn tes_idle_manager_rebuild_root_filename_lists(e: &mut Engine, this: Ptr) {
    for_each_root(e, this, |e, root, name| {
        release_file_lists(e, root, true);
        let loader = e.global::<u32>(MODEL_LOADER);
        let list = e
            .call(
                MODEL_LOADER_BUILD_KF_FILE_LIST,
                &args![loader, name, 1u32, 0u32, 0u32],
            )
            .ptr::<()>();
        idle_anim_root_add_root_filename_list(e, Ptr::new(root), list);
    });
}

// Translated from 006004f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006004f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESIdleManager::AddRootIdleArray` (Xbox PDB overload `_ov2`): for a
/// non-null idle model, its idle directory name (`GetIdleDirName` of
/// `TESModel::GetModel`) into a stack buffer and `AddRootIdleArray(name, 0,
/// 0)`; null otherwise.
pub fn tes_idle_manager_add_root_idle_array_ov2(e: &mut Engine, this: Ptr, model: Ptr) -> u32 {
    if model.is_null() {
        return 0;
    }
    e.with_stack(PATH_SIZE, |e, name| {
        let path = e.call(MODEL_GET_MODEL, &args![model]).ptr::<()>();
        tes_idle_manager_get_idle_dir_name(e, path, name);
        tes_idle_manager_add_root_idle_array(e, this, name, Ptr::NULL, 0)
    })
}

/// The map lookup (`00853130`) of a non-empty path: the value found, or
/// null.
fn lookup_root(e: &mut Engine, this: Ptr, path: Ptr) -> u32 {
    if path.is_null() || e.mem.i8(path.addr()) == 0 {
        return 0;
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), 0);
        let found = e.call(MAP_LOOKUP, &args![this, path, value]).bool();
        if found {
            e.mem.u32(value.addr())
        } else {
            0
        }
    })
}

// Translated from 00600560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::GetRootIdleArray` (Xbox PDB): the root registered for a
/// non-empty `path`, null otherwise.
pub fn tes_idle_manager_get_root_idle_array(e: &mut Engine, this: Ptr, path: Ptr) -> u32 {
    lookup_root(e, this, path)
}

// Translated from 00600630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::GetLooseIdleArray` (Xbox PDB): the loose idle array
/// (`+0x18`) of the root registered for a non-empty `path`, null
/// otherwise.
pub fn tes_idle_manager_get_loose_idle_array(e: &mut Engine, this: Ptr, path: Ptr) -> u32 {
    let root = lookup_root(e, this, path);
    if root == 0 {
        0
    } else {
        root + ROOT_LOOSE_IDLE_ARRAY
    }
}

// Translated from 00600690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600690 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESIdleManager::GetLooseIdleArray` (Xbox PDB overload `_ov2`): for a
/// non-null idle model, the loose idle array of its idle directory; null
/// otherwise.
pub fn tes_idle_manager_get_loose_idle_array_ov2(e: &mut Engine, this: Ptr, model: Ptr) -> u32 {
    if model.is_null() {
        return 0;
    }
    e.with_stack(PATH_SIZE, |e, name| {
        let path = e.call(MODEL_GET_MODEL, &args![model]).ptr::<()>();
        tes_idle_manager_get_idle_dir_name(e, path, name);
        tes_idle_manager_get_loose_idle_array(e, this, name)
    })
}

// Translated from 006007f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006007f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESIdleManager::GetIdleDirName` (Xbox PDB; cdecl(model path, out)):
/// `out` is emptied; for a non-empty path, a copy of it is cut after its
/// `IdleAnims` part when it has one (`004812f0`), else after its last
/// backslash with `IdleAnims` appended (nothing without a backslash), and
/// copied into `out` (0x104 bytes).
pub fn tes_idle_manager_get_idle_dir_name(e: &mut Engine, model: Ptr, out: Ptr) {
    e.mem.set_u8(out.addr(), 0);
    if model.is_null() || e.mem.i8(model.addr()) == 0 {
        return;
    }
    e.with_stack(PATH_SIZE, |e, copy| {
        e.call(STRING_COPY, &args![copy, PATH_SIZE, model]);
        let part = e.call(STRING_FIND, &args![copy, IDLE_ANIMS_TEXT]).u32();
        if part != 0 {
            let length = e.call(C_STRING_LENGTH, &args![IDLE_ANIMS_TEXT]).u32();
            e.mem.set_u8(part.wrapping_add(length), 0);
            e.call(STRING_COPY, &args![out, PATH_SIZE, copy]);
        } else {
            let separator = e.call(STRRCHR, &args![copy, 0x5cu32]).u32();
            if separator != 0 {
                e.mem.set_u8(separator.wrapping_add(1), 0);
                e.call(STRING_COPY, &args![out, PATH_SIZE, copy]);
                e.call(STRING_APPEND, &args![out, PATH_SIZE, IDLE_ANIMS_TEXT]);
            }
        }
    });
}

// Translated from 006008f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::GetUsedItem` (Xbox PDB): `pUsedItem`.
pub fn tes_idle_manager_get_used_item(e: &mut Engine) -> u32 {
    e.global::<u32>(USED_ITEM)
}

// Translated from 00600900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESIdleManager::SetUsedItem` (Xbox PDB; cdecl(item)).
pub fn tes_idle_manager_set_used_item(e: &mut Engine, item: u32) {
    e.set_global(USED_ITEM, item);
}

// Translated from 00600930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011cb6a8` (in `AL`), the static `00600940`
/// (`TESIdleManager::SetUsedItemActivate`, Xbox PDB) sets.
pub fn fn_00600930(e: &mut Engine) -> u8 {
    e.global::<u8>(USED_ITEM_ACTIVATE)
}

// Translated from 00600950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600950 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESIdleManager::GetIdleToPlay` (Xbox PDB; thiscall(actor, word)): for a
/// non-null actor, the root of its model's idle directory; each of the
/// root's idles (until one answers) is asked through `005ff2e0(actor,
/// word, part)` with `part` the actor's virtual `+0x1E4` plus 0x134 (null
/// when that is null). An answer for which `005ff1d0` says yes but
/// `TESIdleForm::CheckIdleFile` says no becomes null.
pub fn tes_idle_manager_get_idle_to_play(e: &mut Engine, this: Ptr, actor: Ptr, word: u32) -> u32 {
    let mut result = 0u32;
    if !actor.is_null() {
        e.with_stack(PATH_SIZE, |e, name| {
            let path = e.call(REFERENCE_GET_MODEL, &args![actor]).ptr::<()>();
            tes_idle_manager_get_idle_dir_name(e, path, name);
            let process = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
            let root = tes_idle_manager_get_root_idle_array(e, this, name);
            if root != 0 {
                let count = e.call(FORM_ARRAY_COUNT, &args![root]).u32();
                let mut index = 0u32;
                while index < count {
                    let idle = e.call(FORM_ARRAY_GET_FORM_AT, &args![root, index]).u32();
                    if idle != 0 {
                        let part = if process == 0 { 0 } else { process + 0x134 };
                        result = e
                            .call(IDLE_FORM_PICK, &args![idle, actor, word, part])
                            .u32();
                    }
                    if result != 0 {
                        break;
                    }
                    index += 1;
                }
            }
        });
    }
    if result != 0
        && e.call(IDLE_FORM_TEST_5FF1D0, &args![result]).bool()
        && !e.call(IDLE_FORM_CHECK_IDLE_FILE, &args![result]).bool()
    {
        result = 0;
    }
    result
}

// Translated from 00600ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTCaseInsensitiveStringMap<IDLE_ANIM_ROOT*>` constructor
/// (thiscall(size, copy flag)): the `NiTStringMap` one (`00600fd0`), then
/// its own vtable. Returns `this`.
pub fn fn_00600ad0(e: &mut Engine, this: Ptr, size: u32, copy: u8) -> Ptr {
    e.call(STRING_MAP_CONSTRUCT, &args![this, size, u32::from(copy)]);
    e.mem.set_u32(this.addr(), ROOT_MAP_VTABLE);
    this
}

// Translated from 00600d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPrimitiveArray<TESIdleForm*>` constructor (thiscall(two words)):
/// the `NiTArray` one (`00601000`), then its own vtable. Returns `this`.
pub fn fn_00600d10(e: &mut Engine, this: Ptr, max_size: u32, grow_by: u32) -> Ptr {
    e.call(IDLE_FORM_ARRAY_CONSTRUCT, &args![this, max_size, grow_by]);
    e.mem.set_u32(this.addr(), IDLE_FORM_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00600d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600d90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `NiTStringTemplateMap<..>` destructor body: its vtable; when it
/// copies its keys (the byte at `+0x10`), each key of each bucket chain
/// (`+8`, `+4` buckets; items linked through `+0`, key at `+4`) is freed
/// (`00aa10f0`); then the `NiTMap` destructor body (`00600e50`).
pub fn fn_00600d90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, STRING_TEMPLATE_MAP_VTABLE);
    if e.mem.u8(base + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(base + 4) {
            let table = e.mem.u32(base + 8);
            let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let current = item;
                item = e.mem.u32(current);
                let key = e.mem.u32(current.wrapping_add(4));
                e.call(MAP_FREE, &args![key]);
            }
            bucket = bucket.wrapping_add(1);
        }
    }
    fn_00600e50(e, this);
}

// Translated from 00600e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600e50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `NiTMap<const char*,IDLE_ANIM_ROOT*>` destructor body: its vtable,
/// `RemoveAll`, then the `NiTMapBase` destructor body (`00600eb0`).
pub fn fn_00600e50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00600eb0(e, this);
}

// Translated from 00600eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..>` destructor body: its vtable, `RemoveAll`, then the
/// bucket table (`+8`) freed (`00aa10f0`).
pub fn fn_00600eb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(MAP_FREE, &args![table]);
}

#[cfg(test)]
mod thunk_tests {
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

    #[test]
    fn fwd_a03_005ffbd0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0b90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_fbd0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0b90), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00600d70_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0d90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0d70, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0d90), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00600b00_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0060_0d90, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0060_0b00, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0060_0d90)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_a040);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_a040,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text.as_bytes());
        address
    }

    /// Doubles for the string helpers, working on game memory.
    fn register_strings(e: &mut Engine) {
        e.map(IDLE_ANIMS_TEXT, 16);
        e.mem.set_cstr(IDLE_ANIMS_TEXT, b"IdleAnims");
        e.register(STRING_COPY, |e, a| {
            let mut source = e.mem.cstr(a[2]);
            source.truncate(a[1] as usize - 1);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| u32::from(*b) == a[1]) {
                Some(index) => (a[0] + index as u32).into_ret(),
                None => Ret::default(),
            }
        });
    }

    #[test]
    fn at_00600700_looks_the_idle_anims_key_up_and_indexes_the_entry() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        let keys: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let entry = e.mem.alloc(0x60);
        e.mem.set_u32(entry + 0x38 + 8, 0xfeed_0002);
        let sink = keys.clone();
        e.register_double(MAP_LOOKUP, move |e, a| {
            sink.borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
            e.mem.set_u32(a[2], entry);
            1u32.into_ret()
        });
        let manager = e.mem.alloc(0x20);
        let path = put_text(&mut e, "meshes\\characters\\_1stperson\\idle.kf");
        let result = e.call(0x0060_0700, &args![manager, path, 2u32]).u32();
        assert_eq!(result, 0xfeed_0002);
        assert_eq!(
            *keys.borrow(),
            vec!["meshes\\characters\\_1stperson\\IdleAnims".to_string()]
        );
        let lookup = &calls(&e, MAP_LOOKUP)[0];
        assert_eq!(lookup[0], manager);
    }

    #[test]
    fn at_00600700_answers_null_for_missing_paths_and_lookups() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        e.register(MAP_LOOKUP, |_, _| Ret::default());
        let manager = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0060_0700, &args![manager, 0u32, 0u32]).u32(), 0);
        let empty = put_text(&mut e, "");
        assert_eq!(e.call(0x0060_0700, &args![manager, empty, 0u32]).u32(), 0);
        assert!(calls(&e, MAP_LOOKUP).is_empty());
        let path = put_text(&mut e, "a\\b.kf");
        // Not found.
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
        // Found, but the entry is null.
        e.register(MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
        // Not found even though the entry cell was filled in.
        let entry = e.mem.alloc(0x60);
        e.register_double(MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], entry);
            Ret::default()
        });
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
    }

    fn rig_add() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        e.map(MODEL_LOADER, 4);
        e.map(BUILDING_ROOT_FLAG, 4);
        e.set_global(MODEL_LOADER, 0x00c0_ffee);
        let manager = e.mem.alloc(0x20);
        let array_block = e.mem.alloc(0x68);
        e.register(GET_ROOT_IDLE_ARRAY, |_, _| Ret::default());
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a, [0x68]);
            array_block.into_ret()
        });
        e.register(ROOT_IDLE_ARRAY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ADD_ROOT_FILENAME_LIST, |_, _| Ret::default());
        e.register(MODEL_LOADER_COPY_FILENAME_LIST, |_, _| 0x1111u32.into_ret());
        e.register(MODEL_LOADER_BUILD_KF_FILE_LIST, |_, _| 0x2222u32.into_ret());
        e.register(REGISTER_ROOT_IDLE_ARRAY, |_, _| Ret::default());
        (e, manager, array_block)
    }

    #[test]
    fn at_00600170_answers_the_registered_array_without_building() {
        let (mut e, manager, _) = rig_add();
        e.register(GET_ROOT_IDLE_ARRAY, |_, _| 0x7777u32.into_ret());
        let path = put_text(&mut e, "a\\b.kf");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u32]).u32(),
            0x7777
        );
        assert_eq!(calls(&e, GET_ROOT_IDLE_ARRAY), vec![vec![manager, path]]);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn at_00600170_does_nothing_for_a_missing_path_or_while_building() {
        let (mut e, manager, _) = rig_add();
        assert_eq!(
            e.call(0x0060_0170, &args![manager, 0u32, 0u32, 0u32]).u32(),
            0
        );
        let empty = put_text(&mut e, "");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, empty, 0u32, 1u8]).u32(),
            0
        );
        let path = put_text(&mut e, "a\\b.kf");
        e.set_global::<u8>(BUILDING_ROOT_FLAG, 1);
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32(),
            0
        );
        assert!(calls(&e, GET_ROOT_IDLE_ARRAY).is_empty());
        // Forced, the flag is ignored.
        let array = e.call(0x0060_0170, &args![manager, path, 0u32, 1u8]).u32();
        assert_ne!(array, 0);
        assert_eq!(calls(&e, GET_ROOT_IDLE_ARRAY).len(), 1);
    }

    #[test]
    fn at_00600170_builds_the_array_from_a_given_file_list() {
        let (mut e, manager, block) = rig_add();
        let path = put_text(&mut e, "meshes\\actors\\idle.kf");
        let list = e.mem.alloc(8);
        let array = e.call(0x0060_0170, &args![manager, path, list, 0u8]).u32();
        assert_eq!(array, block);
        assert_eq!(
            calls(&e, ROOT_IDLE_ARRAY_CONSTRUCTOR),
            vec![vec![block, path]]
        );
        assert_eq!(
            calls(&e, MODEL_LOADER_COPY_FILENAME_LIST),
            vec![vec![0x00c0_ffee, list, 0]]
        );
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![block, 0x1111]]);
        let register = &calls(&e, REGISTER_ROOT_IDLE_ARRAY)[0];
        assert_eq!(register[0], manager);
        assert_eq!(register[2], block);
        assert_eq!(
            String::from_utf8(e.mem.cstr(register[1])).unwrap(),
            "meshes\\actors\\IdleAnims"
        );
        assert!(calls(&e, MODEL_LOADER_BUILD_KF_FILE_LIST).is_empty());
        assert_eq!(e.global::<u8>(BUILDING_ROOT_FLAG), 0);
    }

    #[test]
    fn at_00600170_builds_the_array_from_the_animation_files_without_a_list() {
        let (mut e, manager, block) = rig_add();
        let path = put_text(&mut e, "meshes\\actors\\idle.kf");
        let flag_seen = Rc::new(RefCell::new(Vec::new()));
        let sink = flag_seen.clone();
        e.register_double(MODEL_LOADER_BUILD_KF_FILE_LIST, move |e, _| {
            sink.borrow_mut().push(e.global::<u8>(BUILDING_ROOT_FLAG));
            0x2222u32.into_ret()
        });
        let array = e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32();
        assert_eq!(array, block);
        assert_eq!(*flag_seen.borrow(), vec![1]);
        assert_eq!(e.global::<u8>(BUILDING_ROOT_FLAG), 0);
        assert_eq!(
            calls(&e, MODEL_LOADER_BUILD_KF_FILE_LIST),
            vec![vec![0x00c0_ffee, path, 1, 0, 0]]
        );
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![block, 0x2222]]);
        assert_eq!(
            calls(&e, REGISTER_ROOT_IDLE_ARRAY),
            vec![vec![manager, path, block]]
        );
        assert!(calls(&e, MODEL_LOADER_COPY_FILENAME_LIST).is_empty());
    }

    #[test]
    fn at_00600170_a_failed_allocation_registers_a_null_array() {
        let (mut e, manager, _) = rig_add();
        e.register(ALLOCATE, |_, _| Ret::default());
        let path = put_text(&mut e, "a\\b.kf");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32(),
            0
        );
        assert!(calls(&e, ROOT_IDLE_ARRAY_CONSTRUCTOR).is_empty());
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![0, 0x2222]]);
        assert_eq!(
            calls(&e, REGISTER_ROOT_IDLE_ARRAY),
            vec![vec![manager, path, 0]]
        );
    }
}

#[cfg(test)]
mod b016_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn text(e: &mut Engine, value: &str) -> u32 {
        let address = e.mem.alloc(value.len() as u32 + 1);
        e.mem.set_cstr(address, value.as_bytes());
        address
    }

    /// A model of the simple list calls: a list is a word holding the
    /// address of a vector of item words (count first).
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| {
            ((e.mem.u32(a[0]) == 0) as u32).into_ret()
        });
        e.register(LIST_ITEM, |_, a| (a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let count = e.mem.u32(a[0]);
            for i in 1..count {
                let next = e.mem.u32(a[0] + 4 + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, next);
            }
            e.mem.set_u32(a[0], count - 1);
            Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            let count = e.mem.u32(a[0]);
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4 + 4 * count, item);
            e.mem.set_u32(a[0], count + 1);
            Ret::default()
        });
        e.register(ALLOCATE, |e, _| e.mem.alloc(0x40).into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let list = e.mem.alloc(0x40);
        e.mem.set_u32(list, items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *item);
        }
        list
    }

    #[test]
    fn root_constructor_and_destructors() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_ARRAY_CONSTRUCT,
                FORM_ARRAY_DESTRUCT,
                STRING_CONSTRUCT,
                STRING_SET,
                STRING_DESTRUCT,
                FORM_ARRAY_CALL_864280,
                FORM_ARRAY_CALL_600AB0,
                MEMSET,
                OPERATOR_DELETE,
            ],
        );
        let root = e.mem.alloc(0x68);
        e.call_log = Some(vec![]);
        let back = e.call(0x005f_f890, &args![root, 0x1234u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, root);
        assert_eq!(
            calls(&log, FORM_ARRAY_CONSTRUCT),
            vec![vec![root], vec![root + 0x18]]
        );
        assert_eq!(calls(&log, STRING_SET), vec![vec![root + 0x30, 0x1234, 0]]);
        assert_eq!(
            calls(&log, FORM_ARRAY_CALL_600AB0),
            vec![vec![root, 1], vec![root + 0x18, 1]]
        );
        assert_eq!(calls(&log, MEMSET), vec![vec![root + 0x38, 0, 0x30]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_ff90, &args![root, 1u32]).u32(), root);
        e.call(0x005f_ff90, &args![root, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRING_DESTRUCT), vec![vec![root + 0x30]; 2]);
        assert_eq!(
            calls(&log, FORM_ARRAY_DESTRUCT)[..2],
            [vec![root + 0x18], vec![root]]
        );
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![root]]);
    }

    #[test]
    fn add_root_filename_list_sorts_names_by_kind() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // The kind is the first byte of the name minus '0'.
        e.register(PARSE_ANIM_GROUP_FILENAME, |e, a| {
            let kind = u32::from(e.mem.u8(a[0])) - u32::from(b'0');
            e.mem.set_u32(a[2], kind);
            Ret::default()
        });
        e.register(STRRCHR_CRT, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| *b == a[1] as u8) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(STRICMP, |e, a| {
            let x = e.mem.cstr(a[0]).to_ascii_lowercase();
            let y = e.mem.cstr(a[1]).to_ascii_lowercase();
            ((x != y) as u32).into_ret()
        });
        e.map(0x0104_a000, 0x1000);
        for (address, name) in LEFT_HAND_NAMES.iter().zip([
            "1HPLeft.kf",
            "1HPFastLeft.kf",
            "1HMLeft.kf",
            "1HMFastLeft.kf",
        ]) {
            e.mem.set_cstr(*address, name.as_bytes());
        }
        let a = text(&mut e, "2\\walk.kf");
        let b = text(&mut e, "4\\1hmfastleft.KF");
        let c = text(&mut e, "4\\other.kf");
        let d = text(&mut e, "2\\run.kf");
        let list = list_of(&mut e, &[a, b, c, d]);
        let root = e.mem.alloc(0x68);
        e.call_log = Some(vec![]);
        e.call(0x005f_f950, &args![root, list]);
        let log = e.call_log.take().unwrap();
        let kind2 = e.mem.u32(root + 0x38 + 8);
        let kind0 = e.mem.u32(root + 0x38);
        let kind4 = e.mem.u32(root + 0x38 + 16);
        assert_eq!(e.mem.u32(kind2), 2);
        assert_eq!(e.mem.u32(kind2 + 4), a);
        assert_eq!(e.mem.u32(kind2 + 8), d);
        assert_eq!(e.mem.u32(kind0 + 4), b);
        assert_eq!(e.mem.u32(kind4 + 4), c);
        assert_eq!(calls(&log, ALLOCATE).len(), 3);
        assert_eq!(calls(&log, LIST_DELETE), vec![vec![list, 1]]);
        // A null list does nothing.
        e.call_log = Some(vec![]);
        e.call(0x005f_f950, &args![root, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn manager_constructor_destructor_and_add() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                STRING_MAP_CONSTRUCT,
                IDLE_FORM_ARRAY_CONSTRUCT,
                IDLE_ARRAY_ADD,
                MAP_FIRST,
                IDLE_ARRAY_SIZE,
                IDLE_ARRAY_CALL_5E03D0,
                MAP_REMOVE_ALL,
                0x0060_0b90,
                0x0060_0d90,
            ],
        );
        let manager = e.mem.alloc(0x28);
        e.mem.set_u8(manager + 0x14, 1);
        e.call_log = Some(vec![]);
        let back = e.call(0x005f_faf0, &args![manager]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, manager);
        assert_eq!(
            calls(&log, STRING_MAP_CONSTRUCT),
            vec![vec![manager, 0x25, 1]]
        );
        assert_eq!(
            calls(&log, IDLE_FORM_ARRAY_CONSTRUCT),
            vec![vec![manager + 0x18, 0, 1]]
        );
        assert_eq!(e.mem.u32(manager), ROOT_MAP_VTABLE);
        assert_eq!(e.mem.u32(manager + 0x18), IDLE_FORM_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u8(manager + 0x14), 0);

        e.call_log = Some(vec![]);
        e.call(0x005f_fbf0, &args![manager, 0u32]);
        e.call(0x005f_fbf0, &args![manager, 0x77u32]);
        e.call(0x005f_fb60, &args![manager]);
        let log = e.call_log.take().unwrap();
        let add = calls(&log, IDLE_ARRAY_ADD);
        assert_eq!(add.len(), 1);
        assert_eq!(add[0][0], manager + 0x18);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![manager]]);
        assert_eq!(calls(&log, 0x0060_0b90), vec![vec![manager + 0x18]]);
        assert_eq!(calls(&log, 0x0060_0d90), vec![vec![manager]]);
    }

    /// An idle array of the given forms (null entries allowed); each form has
    /// a vtable whose slots go to `0x7100_0000 + slot`.
    fn idle_array(e: &mut Engine, forms: &[bool]) -> Vec<u32> {
        let vtable = e.mem.alloc(0x100);
        for slot in (0..0x100u32).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x7100_0000 + slot);
            e.register(0x7100_0000 + slot, |_, _| Ret::default());
        }
        let slots = e.mem.alloc(4 * forms.len() as u32);
        let mut made = vec![];
        for (i, present) in forms.iter().enumerate() {
            let form = if *present {
                let form = e.mem.alloc(0x10);
                e.mem.set_u32(form, vtable);
                form
            } else {
                0
            };
            e.mem.set_u32(slots + 4 * i as u32, form);
            made.push(form);
        }
        let count = forms.len() as u32;
        e.register_double(IDLE_ARRAY_SIZE, move |_, _| count.into_ret());
        e.register_double(IDLE_ARRAY_ELEMENT, move |_, a| {
            (slots + 4 * a[1]).into_ret()
        });
        made
    }

    #[test]
    fn initialising_clears_bad_idles_and_marks_the_manager() {
        let mut e = Engine::new();
        quiet(&mut e, &[IDLE_ARRAY_SET_AT, IDLE_ARRAY_CALL_600BC0]);
        let forms = idle_array(&mut e, &[true, false, true]);
        let deleted = forms[2];
        e.register_double(FORM_TEST_4013E0, move |_, a| {
            ((a[0] == deleted) as u32).into_ret()
        });
        let first = forms[0];
        e.register_double(IDLE_FORM_INIT_PARENTS, move |_, a| {
            ((a[0] == first) as u32).into_ret()
        });
        let manager = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x005f_fc20, &args![manager]);
        let log = e.call_log.take().unwrap();
        let cleared: Vec<u32> = calls(&log, IDLE_ARRAY_SET_AT)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(cleared, vec![1, 2, 0]);
        assert_eq!(calls(&log, 0x7100_0088), vec![vec![forms[0]]]);
        assert_eq!(
            calls(&log, IDLE_ARRAY_CALL_600BC0),
            vec![vec![manager + 0x18]]
        );
        assert_eq!(e.mem.u8(manager + 0x14), 1);
    }

    /// One root in the map under the key "anims\\IdleAnims".
    fn map_with_root(e: &mut Engine, root: u32) -> u32 {
        let key = text(e, "anims\\IdleAnims");
        e.register(MAP_FIRST, |_, _| 1u32.into_ret());
        e.register_double(MAP_NEXT, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], root);
            Ret::default()
        });
        key
    }

    #[test]
    fn releasing_deletes_roots_lists_and_idles() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        quiet(
            &mut e,
            &[
                IDLE_ARRAY_CALL_5E03D0,
                MAP_REMOVE_ALL,
                STRING_DESTRUCT,
                FORM_ARRAY_DESTRUCT,
            ],
        );
        let forms = idle_array(&mut e, &[true]);
        e.register_double(FORM_ARRAY_COUNT, |_, _| 1u32.into_ret());
        let form = forms[0];
        e.register_double(FORM_ARRAY_GET_FORM_AT, move |_, _| form.into_ret());
        let root = e.mem.alloc(0x68);
        let names = list_of(&mut e, &[0x0aa0, 0x0bb0]);
        e.mem.set_u32(root + 0x38 + 12, names);
        map_with_root(&mut e, root);
        let manager = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x005f_fd20, &args![manager]);
        let log = e.call_log.take().unwrap();
        // Both arrays' form and the loaded idle: three deletes.
        assert_eq!(calls(&log, 0x7100_0010).len(), 3);
        assert_eq!(
            calls(&log, OPERATOR_DELETE)[..2],
            [vec![0x0aa0], vec![0x0bb0]]
        );
        assert_eq!(calls(&log, LIST_DELETE), vec![vec![names, 1]]);
        // The slot is not cleared here; the root is deleted.
        assert_eq!(e.mem.u32(root + 0x38 + 12), names);
        assert_eq!(calls(&log, OPERATOR_DELETE)[2], vec![root]);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![manager]]);
    }

    #[test]
    fn rebuilding_reloads_each_roots_lists() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(PARSE_ANIM_GROUP_FILENAME, |_, _| Ret::default());
        let root = e.mem.alloc(0x68);
        let names = list_of(&mut e, &[0x0aa0]);
        e.mem.set_u32(root + 0x38 + 4, names);
        let key = map_with_root(&mut e, root);
        let fresh = list_of(&mut e, &[0x0cc0]);
        e.register_double(MODEL_LOADER_BUILD_KF_FILE_LIST, move |_, _| {
            fresh.into_ret()
        });
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x0ddd_0000u32);
        let manager = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x0060_03c0, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(root + 0x38 + 4), 0);
        assert_eq!(
            calls(&log, MODEL_LOADER_BUILD_KF_FILE_LIST),
            vec![vec![0x0ddd_0000, key, 1, 0, 0]]
        );
        let kind0 = e.mem.u32(root + 0x38);
        assert_eq!(e.mem.u32(kind0 + 4), 0x0cc0);
    }

    fn lookup_doubles(e: &mut Engine, root: u32) {
        e.register_double(MAP_LOOKUP, move |e, a| {
            let wanted = e.mem.cstr(a[1]) == b"meshes\\IdleAnims";
            if wanted {
                e.mem.set_u32(a[2], root);
            }
            (wanted as u32).into_ret()
        });
        e.register(STRING_COPY, |e, a| {
            let mut source = e.mem.cstr(a[2]);
            source.truncate(a[1] as usize - 1);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        e.register(STRING_FIND, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            match haystack
                .windows(needle.len())
                .position(|w| w == needle.as_slice())
            {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| *b == a[1] as u8) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(C_STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.map(IDLE_ANIMS_TEXT, 16);
        e.mem.set_cstr(IDLE_ANIMS_TEXT, b"IdleAnims");
    }

    #[test]
    fn idle_dir_name_cuts_after_idle_anims_or_the_last_backslash() {
        let mut e = Engine::new();
        lookup_doubles(&mut e, 0);
        let out = e.mem.alloc(0x104);
        let model = text(&mut e, "meshes\\IdleAnims\\sub\\a.kf");
        e.call(0x0060_07f0, &args![model, out]);
        assert_eq!(e.mem.cstr(out), b"meshes\\IdleAnims");
        let model = text(&mut e, "meshes\\actor.nif");
        e.call(0x0060_07f0, &args![model, out]);
        assert_eq!(e.mem.cstr(out), b"meshes\\IdleAnims");
        let model = text(&mut e, "actor.nif");
        e.call(0x0060_07f0, &args![model, out]);
        assert_eq!(e.mem.cstr(out), b"");
        e.call(0x0060_07f0, &args![0u32, out]);
        assert_eq!(e.mem.cstr(out), b"");
    }

    #[test]
    fn root_and_loose_arrays_by_path_and_by_model() {
        let mut e = Engine::new();
        let root = e.mem.alloc(0x68);
        lookup_doubles(&mut e, root);
        e.map(0x011c_b000, 0x1000);
        let path = text(&mut e, "meshes\\IdleAnims");
        assert_eq!(e.call(0x0060_0560, &args![0x100u32, path]).u32(), root);
        assert_eq!(
            e.call(0x0060_0630, &args![0x100u32, path]).u32(),
            root + 0x18
        );
        let other = text(&mut e, "x");
        assert_eq!(e.call(0x0060_0560, &args![0x100u32, other]).u32(), 0);
        assert_eq!(e.call(0x0060_0630, &args![0x100u32, other]).u32(), 0);
        let empty = text(&mut e, "");
        assert_eq!(e.call(0x0060_0560, &args![0x100u32, empty]).u32(), 0);
        let model_path = text(&mut e, "meshes\\actor.nif");
        e.register_double(MODEL_GET_MODEL, move |_, _| model_path.into_ret());
        assert_eq!(
            e.call(0x0060_0690, &args![0x100u32, 0x5000u32]).u32(),
            root + 0x18
        );
        assert_eq!(e.call(0x0060_0690, &args![0x100u32, 0u32]).u32(), 0);
        // The add finds the registered root.
        assert_eq!(e.call(0x0060_04f0, &args![0x100u32, 0x5000u32]).u32(), root);
        assert_eq!(e.call(0x0060_04f0, &args![0x100u32, 0u32]).u32(), 0);
    }

    #[test]
    fn used_item_statics() {
        let mut e = Engine::new();
        e.map(0x011c_b000, 0x1000);
        e.call(0x0060_0900, &args![0x1234u32]);
        assert_eq!(e.call(0x0060_08f0, &args![]).u32(), 0x1234);
        e.set_global(USED_ITEM_ACTIVATE, 0x0000_0101u32);
        assert_eq!(e.call(0x0060_0930, &args![]).u32() & 0xff, 1);
    }

    #[test]
    fn idle_to_play_asks_each_idle_until_one_answers() {
        let mut e = Engine::new();
        let root = e.mem.alloc(0x68);
        lookup_doubles(&mut e, root);
        let model_path = text(&mut e, "meshes\\actor.nif");
        e.register_double(REFERENCE_GET_MODEL, move |_, _| model_path.into_ret());
        e.register(0x0012_31e4, |_, _| 0x0900u32.into_ret());
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x1e4, 0x0012_31e4);
        let actor = e.mem.alloc(0x10);
        e.mem.set_u32(actor, vtable);
        e.register(FORM_ARRAY_COUNT, |_, _| 3u32.into_ret());
        e.register(FORM_ARRAY_GET_FORM_AT, |_, a| (0x0a00 + a[1]).into_ret());
        // The second idle answers.
        e.register(IDLE_FORM_PICK, |_, a| {
            ((a[0] == 0x0a01) as u32 * 0x0eee).into_ret()
        });
        e.register(IDLE_FORM_TEST_5FF1D0, |_, _| 1u32.into_ret());
        e.register(IDLE_FORM_CHECK_IDLE_FILE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let idle = e.call(0x0060_0950, &args![0x100u32, actor, 7u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(idle, 0x0eee);
        assert_eq!(
            calls(&log, IDLE_FORM_PICK),
            vec![
                vec![0x0a00, actor, 7, 0x0a34],
                vec![0x0a01, actor, 7, 0x0a34]
            ]
        );
        // A file check that fails drops the answer.
        e.register(IDLE_FORM_CHECK_IDLE_FILE, |_, _| Ret::default());
        assert_eq!(e.call(0x0060_0950, &args![0x100u32, actor, 7u32]).u32(), 0);
        assert_eq!(e.call(0x0060_0950, &args![0x100u32, 0u32, 7u32]).u32(), 0);
    }

    #[test]
    fn map_destructors_free_keys_and_tables() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, MAP_FREE]);
        let map = e.mem.alloc(0x14);
        let table = e.mem.alloc(8);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 4, 0x0b0b);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x0a0a);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u8(map + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x0060_0d90, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MAP_FREE),
            vec![vec![0x0a0a], vec![0x0b0b], vec![table]]
        );
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![map], vec![map]]);
        assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_VTABLE);
        // Without key copies only the table is freed.
        e.mem.set_u8(map + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_0d90, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MAP_FREE), vec![vec![table]]);
    }

    #[test]
    fn map_and_array_constructors_store_their_vtables() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                STRING_MAP_CONSTRUCT,
                IDLE_FORM_ARRAY_CONSTRUCT,
                MAP_REMOVE_ALL,
                MAP_FREE,
            ],
        );
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_0ad0, &args![this, 9u32, 0x101u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ROOT_MAP_VTABLE);
        assert_eq!(e.call(0x0060_0d10, &args![this, 3u32, 4u32]).u32(), this);
        assert_eq!(e.mem.u32(this), IDLE_FORM_PRIMITIVE_ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        // Only the low byte of the copy flag goes on.
        assert_eq!(calls(&log, STRING_MAP_CONSTRUCT), vec![vec![this, 9, 1]]);
        assert_eq!(
            calls(&log, IDLE_FORM_ARRAY_CONSTRUCT),
            vec![vec![this, 3, 4]]
        );
        e.call(0x0060_0e50, &args![this]);
        assert_eq!(e.mem.u32(this), NI_T_MAP_BASE_VTABLE);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The forms' deleting slot (vtable +0x10) as the test places it.
    const DELETE_SLOT: u32 = 0x0090_0010;

    #[test]
    fn fn_00600030_deletes_and_clears_both_arrays_of_each_root() {
        let mut e = Engine::new();
        // Two roots: the second step answers a null root, which is skipped.
        let first_root = e.mem.alloc(0x68);
        let steps = vec![(2u32, first_root), (3, 0), (0, 0)];
        let mut steps = steps.into_iter();
        e.register(MAP_FIRST, |_, _| 1u32.into_ret());
        e.register_double(MAP_NEXT, move |e, a| {
            let (next, root) = steps.next().unwrap();
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[3], root);
            Ret::default()
        });
        // Each array holds two slots: a form and a null.
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, DELETE_SLOT]);
        let form = e.mem.alloc(4);
        e.mem.set_u32(form, 0x0090_1000);
        e.register(FORM_ARRAY_COUNT, |_, _| 2u32.into_ret());
        e.register_double(FORM_ARRAY_GET_FORM_AT, move |_, a| {
            (if a[1] == 0 { form } else { 0 }).into_ret()
        });
        let log: Log = Rc::new(RefCell::new(vec![]));
        for address in [DELETE_SLOT, FORM_ARRAY_CLEAR_00863DB0] {
            let record = log.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x30);
        e.call(0x0060_0030, &args![this]);
        let got = log.borrow().clone();
        assert_eq!(
            got,
            vec![
                (DELETE_SLOT, vec![form, 1]),
                (FORM_ARRAY_CLEAR_00863DB0, vec![first_root]),
                (DELETE_SLOT, vec![form, 1]),
                (FORM_ARRAY_CLEAR_00863DB0, vec![first_root + 0x18]),
            ]
        );
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_text(e: &mut Engine, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *b);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
    }

    fn text(e: &Engine, at: u32) -> Vec<u8> {
        let mut out = vec![];
        let mut at = at;
        while e.mem.u8(at) != 0 {
            out.push(e.mem.u8(at));
            at += 1;
        }
        out
    }

    #[test]
    fn root_idle_array_of_a_model_looks_up_its_idle_directory() {
        let mut e = Engine::new();
        let path = e.mem.alloc(0x20);
        put_text(&mut e, path, b"meshes\\IdleAnims\\x.kf");
        e.register_double(MODEL_GET_MODEL, move |_, _| path.into_ret());
        e.register(STRING_COPY, |e, a| {
            let source = text(e, a[2]);
            put_text(e, a[0], &source);
            Ret::default()
        });
        // "IdleAnims" is found after "meshes\\".
        e.register(STRING_FIND, |_, a| (a[0] + 7).into_ret());
        e.register(C_STRING_LENGTH, |_, _| 9u32.into_ret());
        e.register(MAP_LOOKUP, |e, a| {
            let key = text(e, a[1]);
            if key == b"meshes\\IdleAnims" {
                e.mem.set_u32(a[2], 0x7777);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let manager = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_05c0, &args![manager, 0x4000u32]).u32(),
            0x7777
        );
        assert_eq!(calls(&e, MODEL_GET_MODEL), vec![vec![0x4000]]);
        assert_eq!(calls(&e, MAP_LOOKUP)[0][0], manager);
        // Without a model: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_05c0, &args![manager, 0u32]).u32(), 0);
        assert!(calls(&e, MODEL_GET_MODEL).is_empty());
    }

    #[test]
    fn used_item_level_and_activate() {
        let mut e = Engine::new();
        e.map(0x0119_9000, 0x1000);
        e.map(0x011c_b000, 0x1000);
        e.call(0x0060_0920, &args![17u32]);
        assert_eq!(e.call(0x0060_0910, &args![]).u32(), 17);
        e.call(0x0060_0940, &args![0x1234_5601u32]);
        assert_eq!(e.global::<u8>(USED_ITEM_ACTIVATE), 1);
        assert_eq!(e.global::<u8>(USED_ITEM_ACTIVATE + 1), 0);
    }

    #[test]
    fn scalar_deleting_destructors_run_their_bodies() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            e.register(0x0060_0d90, |_, _| Ret::default());
            e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
            e.register(MAP_FREE, |_, _| Ret::default());
            let map = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0060_0d40, &args![map, flags]).u32(), map);
            // 00600b00: its vtable, then 00600d90 through 00600d70.
            assert_eq!(calls(&e, 0x0060_0d90), vec![vec![map]]);
            assert_eq!(e.call(0x0060_0ee0, &args![map, flags]).u32(), map);
            assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_VTABLE);
            assert_eq!(e.call(0x0060_0f10, &args![map, flags]).u32(), map);
            assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 3);
            e.mem.set_u8(map + 0x10, 0);
            assert_eq!(e.call(0x0060_0f40, &args![map, flags]).u32(), map);
            assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 5);
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), 4 * flags as usize);
        }
    }
}
