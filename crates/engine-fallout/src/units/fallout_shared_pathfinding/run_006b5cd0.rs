//! `fallout shared/pathfinding/run_006b5cd0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `NavMeshInfoMap` form's destructor and its record loader (the
//! class's own file is `navmeshinfomap.cpp`; these three sit in a run of
//! their own in the engine map).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// The vtable of `NavMeshInfoMap` (RTTI `.?AVNavMeshInfoMap@@`).
const NAV_MESH_INFO_MAP_VTABLE: u32 = 0x0106_bb8c;
/// `006b6b20` (navmeshinfomap.cpp): empties the map's infos and maps.
const MAP_CLEAR_006B6B20: u32 = 0x006b_6b20;
/// `006b80d0` (navmeshinfomap.cpp): the destructor of the worldspace map at +0x2C.
const WORLD_MAP_DTOR_006B80D0: u32 = 0x006b_80d0;
/// `006b7fd0` (navmeshinfomap.cpp): the destructor of the info map at +0x1C.
const INFO_MAP_DTOR_006B7FD0: u32 = 0x006b_7fd0;
/// `TESForm::~TESForm` (`00483630`, engine map).
const TES_FORM_DTOR: u32 = 0x0048_3630;
/// `00404eb0` (unnamed in the engine map; `thiscall`, `RET 0x10`): a scope
/// object's constructor (with 0x2E, 1, the source file name and line 0x5C).
const SCOPE_ENTER_00404EB0: u32 = 0x0040_4eb0;
/// `00404ee0` (unnamed in the engine map): the scope object's destructor.
const SCOPE_LEAVE_00404EE0: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Pathfinding\NavMeshInfoMap.cpp"`.
const SOURCE_FILE_NAME: u32 = 0x0106_bcc8;
/// `004b9ba0` (unnamed in the engine map): a map's first position.
const MAP_FIRST_004B9BA0: u32 = 0x004b_9ba0;
/// `006b7f20` (navmeshinfomap.cpp; `RET 0xC`): the map iterator step
/// (iterator, key and value addresses).
const MAP_STEP_006B7F20: u32 = 0x006b_7f20;
/// `0044ddc0` (unnamed in the engine map): an array's size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `00877a30` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `0084e3a0` (unnamed in the engine map): a form's id word.
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `TESFile::GetTESForm` (`00472660`, engine map): the current record's type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, engine map; `thiscall`, `RET 4`).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the file's current chunk size.
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `004726f0` (unnamed in the engine map): moves to the next chunk (false at the end).
const ADVANCE_004726F0: u32 = 0x0047_26f0;
/// `00401680` (unnamed in the engine map): the file's byte at +0x299.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// `00401080` (unnamed in the engine map; cdecl, an address and a byte):
/// fixes up a word read while the file's byte is set.
const FIX_UP_00401080: u32 = 0x0040_1080;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `00853130` (unnamed in the engine map; `thiscall`, `RET 8`: key, the
/// address of the value): a map lookup.
const MAP_GET_AT_00853130: u32 = 0x0085_3130;
/// `NavMeshInfo::SetFromPortalChunk` (`006b51a0`, engine map; `RET 0xC`).
const SET_FROM_PORTAL_CHUNK: u32 = 0x006b_51a0;
/// The form's virtual slot (byte offset) given the editor id.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The form's virtual slot (byte offset) that reads the bounds chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// `006b46a0` (run_006b46a0): the navmesh info constructor.
const NAV_MESH_INFO_CTOR_006B46A0: u32 = 0x006b_46a0;
/// `006b4b50` (run_006b4850; `RET 0xC`): reads a navmesh info from the record bytes.
const NAV_MESH_INFO_READ_006B4B50: u32 = 0x006b_4b50;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`
/// (an info's key).
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `00693d70` (unnamed in the engine map; `thiscall`, `RET 4`): the info
/// already in the map under a key, or null.
const FIND_INFO_00693D70: u32 = 0x0069_3d70;
/// `NiTMapBase<..>::SetAt` (`00844700`, engine map; `thiscall`, `RET 8`).
const MAP_SET_AT_00844700: u32 = 0x0084_4700;
/// `0042fb00` (unnamed in the engine map): the constructor of the string local.
const STRING_CTOR_0042FB00: u32 = 0x0042_fb00;
/// `0042fa40` (unnamed in the engine map): the string local's destructor.
const STRING_DTOR_0042FA40: u32 = 0x0042_fa40;
/// `00690860` (unnamed in the engine map; `thiscall`, `RET 4`): fills the
/// string from the existing info; its byte result decides the `005a8040` call.
const INFO_NAME_00690860: u32 = 0x0069_0860;
/// `0069aee0` (unnamed in the engine map): the `BSSimpleArray` constructor.
const ARRAY_CTOR_0069AEE0: u32 = 0x0069_aee0;
/// `0069af10` (unnamed in the engine map): the `BSSimpleArray` destructor.
const ARRAY_DTOR_0069AF10: u32 = 0x0069_af10;
/// `005e04c0` (unnamed in the engine map): the constructor of the third array local.
const ARRAY_CTOR_005E04C0: u32 = 0x005e_04c0;
/// `005e04f0` (unnamed in the engine map): its destructor.
const ARRAY_DTOR_005E04F0: u32 = 0x005e_04f0;
/// `BSSimpleArray<unsigned_int_1024>::CopyTo<1024>` (`006e5330`, engine map;
/// `thiscall`, `RET 4`: the destination).
const ARRAY_COPY_TO: u32 = 0x006e_5330;
/// `006b5ab0` (unnamed in the engine map; `thiscall`, `RET 4`): merges the
/// new info with the existing one.
const INFO_MERGE_006B5AB0: u32 = 0x006b_5ab0;
/// `005a8040` (named `IOManager::Pause` in the engine map, a folded body;
/// `thiscall`, `RET 4`): given the string's characters.
const INFO_RENAME_005A8040: u32 = 0x005a_8040;
/// `006b6390` (`NavMeshInfo` scalar deleting destructor; `RET 4`).
const NAV_MESH_INFO_DELETE: u32 = 0x006b_6390;

// Translated from 006b5d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5d00 - its FS:[0] exception frame (the unwind states that destroy the maps and the form when a member destructor throws) is not translated
/// `NavMeshInfoMap` destructor body (unnamed in the engine map): the
/// class's vtable, `006b6b20` (empty everything), the destructors of the
/// worldspace map (+0x2C) and the info map (+0x1C), then `TESForm::~TESForm`.
pub fn fn_006b5d00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NAV_MESH_INFO_MAP_VTABLE);
    e.call(MAP_CLEAR_006B6B20, &args![this]);
    e.call(WORLD_MAP_DTOR_006B80D0, &args![this.addr() + 0x2c]);
    e.call(INFO_MAP_DTOR_006B7FD0, &args![this.addr() + 0x1c]);
    e.call(TES_FORM_DTOR, &args![this]);
}

// Translated from 006b5cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoMap` scalar deleting destructor (unnamed in the engine map;
/// `RET 4`): the destructor body `006b5d00`, then `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006b5cd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b5d00(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// Reads the current chunk into the shared buffer (grown when the chunk is
/// larger, signed comparison): returns the chunk size.
fn read_into_buffer(e: &mut Engine, file: Ptr, buffer: &mut u32, capacity: &mut i32) -> u32 {
    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
    if size as i32 > *capacity {
        e.call(DEALLOCATE, &args![*buffer]);
        *capacity = size as i32;
        *buffer = e.call(ALLOCATE, &args![*capacity as u32]).u32();
    }
    e.call(GET_CHUNK_DATA, &args![file, *buffer, size]);
    size
}

// Translated from 006b5d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5d80 - its FS:[0] exception frame (the unwind states of the scope object and the merge locals) and the /GS cookie check (00ec408c) are not translated
/// The `NavMeshInfoMap` record loader (unnamed in the engine map; a
/// virtual of the form; `thiscall`, `RET 4`; returns a byte). Inside a
/// scope object (`00404eb0` / `00404ee0`): when the byte at +0x3C is set,
/// every info's worldspace array (+0x44) has its forms replaced by their
/// ids (`0084e3a0`), and the byte is cleared. A 0x9B0-byte buffer is
/// allocated; a record type other than 0x38 returns false (the buffer is not
/// freed). Then `TESForm::LoadForm` and, with the version starting at 1,
/// chunk by chunk: `NVER` (the version word, fixed up when the file's byte
/// is set; below 5 sets the byte at +0x18), `EDID` (0x200 bytes into a
/// stack buffer of the chunk's size, the exe's `_alloca`; here the engine's
/// stack block; slot +0x134), `OBND` (slot +0xE0), `NVMI` (a new info
/// (0x5C bytes, `006b46a0`) read by `006b4b50`; when an info with its key
/// exists (`00693d70`) the two are merged through local copies of the
/// existing info's three arrays (+0x24, +0x34, +0x44), the existing info is
/// renamed when `00690860` says so, and the new info is deleted; otherwise
/// it is added under its key), and `NVCI` (the first word, an info id fixed
/// up and given its compile index, must be in the info map, or the loader
/// returns false; from version 5 the rest goes to
/// `NavMeshInfo::SetFromPortalChunk`). At the end the buffer is freed and
/// true returned.
pub fn fn_006b5d80(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    // Frame locals whose addresses the calls get: the scope object, the
    // version word, the map walk's iterator, key and value, the found info,
    // the NVCI id, the merge string and its three arrays.
    e.with_stack(0x70, |e, frame| {
        let f = frame.addr();
        let scope = f;
        let version = f + 4;
        let (it, key, value) = (f + 8, f + 0xc, f + 0x10);
        let found = f + 0x14;
        let id = f + 0x18;
        let name = f + 0x20;
        let (first, second, third) = (f + 0x30, f + 0x40, f + 0x50);
        e.call(
            SCOPE_ENTER_00404EB0,
            &args![scope, 0x2eu32, 1u32, SOURCE_FILE_NAME, 0x5cu32],
        );
        if e.mem.u8(base + 0x3c) != 0 {
            let info_map = base + 0x1c;
            let start = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
            e.mem.set_u32(it, start);
            while e.mem.u32(it) != 0 {
                e.call(MAP_STEP_006B7F20, &args![info_map, it, key, value]);
                let mut index: u32 = 0;
                loop {
                    let array = e.mem.u32(value).wrapping_add(0x44);
                    if index >= e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
                        break;
                    }
                    let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
                    let form = e.mem.u32(element);
                    let form_id = e.call(FORM_ID_0084E3A0, &args![form]).u32();
                    let array = e.mem.u32(value).wrapping_add(0x44);
                    let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
                    e.mem.set_u32(element, form_id);
                    index = index.wrapping_add(1);
                }
            }
            e.mem.set_u8(base + 0x3c, 0);
        }
        let mut capacity: i32 = 0x9b0;
        let mut buffer = e.call(ALLOCATE, &args![capacity as u32]).u32();
        if e.call(GET_TES_FORM, &args![file]).u8() != 0x38 {
            e.call(SCOPE_LEAVE_00404EE0, &args![scope]);
            return false;
        }
        // The form's own id, kept in a local the loader never reads again.
        e.call(FORM_ID_0084E3A0, &args![this]);
        e.call(TES_FORM_LOAD_FORM, &args![this, file]);
        e.mem.set_u32(version, 1);
        loop {
            match e.call(GET_TES_CHUNK, &args![file]).u32() {
                // NVCI
                0x4943_564e => {
                    read_into_buffer(e, file, &mut buffer, &mut capacity);
                    let word = e.mem.u32(buffer);
                    e.mem.set_u32(id, word);
                    if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                        e.call(FIX_UP_00401080, &args![id, 0u32]);
                    }
                    e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
                    let data = buffer.wrapping_add(4);
                    let info_id = e.mem.u32(id);
                    if e.call(MAP_GET_AT_00853130, &args![base + 0x1c, info_id, found])
                        .u8()
                        == 0
                    {
                        e.call(SCOPE_LEAVE_00404EE0, &args![scope]);
                        return false;
                    }
                    let level = e.mem.i32(version);
                    if level >= 5 {
                        let info = e.mem.u32(found);
                        e.call(
                            SET_FROM_PORTAL_CHUNK,
                            &args![info, data, file, level as u32],
                        );
                    }
                }
                // EDID
                0x4449_4445 => {
                    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
                    e.with_stack(size, |e, editor_id| {
                        e.call(GET_CHUNK_DATA, &args![file, editor_id, 0x200u32]);
                        e.vcall(base, SLOT_SET_EDITOR_ID, &args![editor_id]);
                    });
                }
                // OBND
                0x444e_424f => {
                    e.vcall(base, SLOT_LOAD_BOUNDS, &args![file]);
                }
                // NVMI
                0x494d_564e => {
                    read_into_buffer(e, file, &mut buffer, &mut capacity);
                    let block = e.call(ALLOCATE, &args![0x5cu32]).u32();
                    let info = if block != 0 {
                        e.call(NAV_MESH_INFO_CTOR_006B46A0, &args![block]).u32()
                    } else {
                        0
                    };
                    let level = e.mem.u32(version);
                    e.call(
                        NAV_MESH_INFO_READ_006B4B50,
                        &args![info, buffer, file, level],
                    );
                    let info_key = e.call(FIRST_OF_00559450, &args![info]).u32();
                    let existing = e.call(FIND_INFO_00693D70, &args![this, info_key]).u32();
                    if existing != 0 {
                        e.call(STRING_CTOR_0042FB00, &args![name]);
                        let rename = e.call(INFO_NAME_00690860, &args![existing, name]).u8();
                        e.call(ARRAY_CTOR_0069AEE0, &args![first]);
                        e.call(ARRAY_CTOR_0069AEE0, &args![second]);
                        e.call(ARRAY_CTOR_005E04C0, &args![third]);
                        e.call(ARRAY_COPY_TO, &args![existing + 0x24, first]);
                        e.call(ARRAY_COPY_TO, &args![existing + 0x34, second]);
                        e.call(ARRAY_COPY_TO, &args![existing + 0x44, third]);
                        e.call(INFO_MERGE_006B5AB0, &args![info, existing]);
                        if rename != 0 {
                            let characters = e.call(FIRST_OF_00559450, &args![name]).u32();
                            e.call(INFO_RENAME_005A8040, &args![existing, characters]);
                        }
                        e.call(ARRAY_COPY_TO, &args![first, existing + 0x24]);
                        e.call(ARRAY_COPY_TO, &args![second, existing + 0x34]);
                        e.call(ARRAY_COPY_TO, &args![third, existing + 0x44]);
                        if info != 0 {
                            e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
                        }
                        e.call(ARRAY_DTOR_005E04F0, &args![third]);
                        e.call(ARRAY_DTOR_0069AF10, &args![second]);
                        e.call(ARRAY_DTOR_0069AF10, &args![first]);
                        e.call(STRING_DTOR_0042FA40, &args![name]);
                    } else {
                        let info_key = e.call(FIRST_OF_00559450, &args![info]).u32();
                        e.call(MAP_SET_AT_00844700, &args![base + 0x1c, info_key, info]);
                    }
                }
                // NVER
                0x5245_564e => {
                    e.call(GET_CHUNK_DATA, &args![file, version, 0u32]);
                    if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                        e.call(FIX_UP_00401080, &args![version, 0u32]);
                    }
                    if e.mem.i32(version) < 5 {
                        e.mem.set_u8(base + 0x18, 1);
                    }
                }
                _ => {}
            }
            if e.call(ADVANCE_004726F0, &args![file]).u8() == 0 {
                break;
            }
        }
        e.call(DEALLOCATE, &args![buffer]);
        e.call(SCOPE_LEAVE_00404EE0, &args![scope]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_5cd0, fn_006b5cd0(Ptr, u32) -> Ptr),
        entry!(0x006b_5d00, fn_006b5d00(Ptr)),
        entry!(0x006b_5d80, fn_006b5d80(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn at_006b5cd0_and_006b5d00_destroy_in_order() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MAP_CLEAR_006B6B20,
                WORLD_MAP_DTOR_006B80D0,
                INFO_MAP_DTOR_006B7FD0,
                TES_FORM_DTOR,
                DEALLOCATE,
            ],
        );
        let this = e.mem.alloc(0x40);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006b_5cd0, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), NAV_MESH_INFO_MAP_VTABLE);
            let order: Vec<(u32, Vec<u32>)> =
                log.into_iter().filter(|c| c.0 != 0x006b_5cd0).collect();
            let mut expected = vec![
                (MAP_CLEAR_006B6B20, vec![this]),
                (WORLD_MAP_DTOR_006B80D0, vec![this + 0x2c]),
                (INFO_MAP_DTOR_006B7FD0, vec![this + 0x1c]),
                (TES_FORM_DTOR, vec![this]),
            ];
            if frees == 1 {
                expected.push((DEALLOCATE, vec![this]));
            }
            assert_eq!(order, expected);
        }
    }

    /// A file double as in the ragdoll loader's tests: index +0, count +4,
    /// record type +0x10, chunk types from +0x20, flag byte +0x299, chunk
    /// size +0x25C.
    fn file(e: &mut Engine, record: u32, chunks: &[u32], size: u32, flag: u8) -> u32 {
        let file = e.mem.alloc(0x300);
        e.mem.set_u32(file + 4, chunks.len() as u32);
        e.mem.set_u32(file + 0x10, record);
        for (i, chunk) in chunks.iter().enumerate() {
            e.mem.set_u32(file + 0x20 + 4 * i as u32, *chunk);
        }
        e.mem.set_u8(file + 0x299, flag);
        e.mem.set_u32(file + 0x25c, size);
        e.register(GET_TES_FORM, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(GET_TES_CHUNK, |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.u32(a[0] + 0x20 + 4 * index).into_ret()
        });
        e.register(ADVANCE_004726F0, |e, a| {
            let index = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], index);
            u32::from(index < e.mem.u32(a[0] + 4)).into_ret()
        });
        e.register(CHUNK_SIZE_00401660, |e, a| {
            e.mem.u32(a[0] + 0x25c).into_ret()
        });
        e.register(FILE_FLAG_00401680, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x299)).into_ret()
        });
        // Version chunks read 7; other chunks the word 0x0100_0042 first.
        e.register(GET_CHUNK_DATA, |e, a| {
            let index = e.mem.u32(a[0]);
            let word = if e.mem.u32(a[0] + 0x20 + 4 * index) == 0x5245_564e {
                e.mem.u32(a[0] + 0x2fc)
            } else {
                0x0100_0042
            };
            e.mem.set_u32(a[1], word);
            Ret::default()
        });
        e.mem.set_u32(file + 0x2fc, 7);
        file
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                SCOPE_ENTER_00404EB0,
                SCOPE_LEAVE_00404EE0,
                DEALLOCATE,
                TES_FORM_LOAD_FORM,
                FIX_UP_00401080,
                TES_FORM_ADD_COMPILE_INDEX,
                SET_FROM_PORTAL_CHUNK,
                NAV_MESH_INFO_READ_006B4B50,
                MAP_SET_AT_00844700,
                STRING_CTOR_0042FB00,
                STRING_DTOR_0042FA40,
                ARRAY_CTOR_0069AEE0,
                ARRAY_DTOR_0069AF10,
                ARRAY_CTOR_005E04C0,
                ARRAY_DTOR_005E04F0,
                ARRAY_COPY_TO,
                INFO_MERGE_006B5AB0,
                INFO_RENAME_005A8040,
                NAV_MESH_INFO_DELETE,
            ],
        );
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FORM_ID_0084E3A0, |_, a| (a[0] | 0x8000_0000).into_ret());
        e.register(NAV_MESH_INFO_CTOR_006B46A0, |_, a| a[0].into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    #[test]
    fn at_006b5d80_refuses_other_records() {
        let mut e = engine();
        let file = file(&mut e, 0x39, &[0x5245_564e], 4, 0);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_5d80, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        let scope = &calls_to(&log, SCOPE_ENTER_00404EB0)[0];
        assert_eq!(scope[1..], [0x2e, 1, SOURCE_FILE_NAME, 0x5c]);
        assert_eq!(calls_to(&log, SCOPE_LEAVE_00404EE0), vec![vec![scope[0]]]);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x9b0]]);
        assert!(calls_to(&log, DEALLOCATE).is_empty());
        assert!(calls_to(&log, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn at_006b5d80_reads_version_names_and_infos() {
        let mut e = engine();
        // No info has the new info's key (its first word, 0 here) yet.
        e.register(FIND_INFO_00693D70, |_, _| Ret::default());
        e.register(MAP_GET_AT_00853130, |e, a| {
            e.mem.set_u32(a[2], 0x0abc);
            1u32.into_ret()
        });
        let mut slots = vec![0u32; 0x4e];
        slots[0x38] = 0x0f07_1000;
        slots[0x4d] = 0x0f07_2000;
        e.put_vtable(0x0f07_0000, &slots);
        e.register(0x0f07_1000, |_, _| Ret::default());
        e.register(0x0f07_2000, |_, _| Ret::default());
        let chunks = [
            0x5245_564e, // NVER
            0x4449_4445, // EDID
            0x444e_424f, // OBND
            0x494d_564e, // NVMI
            0x4943_564e, // NVCI
            0x1111_1111, // other
        ];
        let file = file(&mut e, 0x38, &chunks, 0xa00, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0f07_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_5d80, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(calls_to(&log, FORM_ID_0084E3A0), vec![vec![this]]);
        assert_eq!(calls_to(&log, TES_FORM_LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0f07_1000), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0f07_2000).len(), 1);
        // The buffer grows for the 0xA00-byte NVMI chunk.
        let allocations = calls_to(&log, ALLOCATE);
        assert_eq!(allocations[..2], [vec![0x9b0], vec![0xa00]]);
        assert_eq!(allocations[2], vec![0x5c]);
        let read = &calls_to(&log, NAV_MESH_INFO_READ_006B4B50)[0];
        assert_eq!(read[2..], [file, 7]);
        let info = read[0];
        assert_eq!(
            calls_to(&log, MAP_SET_AT_00844700),
            vec![vec![this + 0x1c, 0, info]]
        );
        let get = &calls_to(&log, MAP_GET_AT_00853130)[0];
        assert_eq!(get[..2], [this + 0x1c, 0x0100_0042]);
        let portal = &calls_to(&log, SET_FROM_PORTAL_CHUNK)[0];
        assert_eq!(portal[0], 0x0abc);
        assert_eq!(portal[2..], [file, 7]);
        // Fix-ups for the version and the NVCI id, then compile index.
        assert_eq!(calls_to(&log, FIX_UP_00401080).len(), 2);
        assert_eq!(calls_to(&log, TES_FORM_ADD_COMPILE_INDEX)[0][1], file);
        // The grown buffer is the one freed at the end.
        assert_eq!(calls_to(&log, DEALLOCATE).len(), 2);
        assert_eq!(calls_to(&log, SCOPE_LEAVE_00404EE0).len(), 1);
    }

    #[test]
    fn at_006b5d80_merges_into_an_existing_info_and_marks_old_versions() {
        let mut e = engine();
        e.register(FIND_INFO_00693D70, |_, _| 0x7000u32.into_ret());
        e.register(INFO_NAME_00690860, |_, _| 1u32.into_ret());
        let file = file(&mut e, 0x38, &[0x5245_564e, 0x494d_564e], 0x10, 0);
        e.mem.set_u32(file + 0x2fc, 4);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_5d80, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this + 0x18), 1);
        let info = calls_to(&log, NAV_MESH_INFO_READ_006B4B50)[0][0];
        assert_eq!(
            calls_to(&log, INFO_MERGE_006B5AB0),
            vec![vec![info, 0x7000]]
        );
        let copies = calls_to(&log, ARRAY_COPY_TO);
        assert_eq!(copies.len(), 6);
        assert_eq!(copies[0][0], 0x7024);
        assert_eq!(copies[3][1], 0x7024);
        assert_eq!(calls_to(&log, INFO_RENAME_005A8040).len(), 1);
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![info, 1]]);
        assert!(calls_to(&log, MAP_SET_AT_00844700).is_empty());
        assert!(calls_to(&log, FIX_UP_00401080).is_empty());
    }

    #[test]
    fn at_006b5d80_fails_on_an_unknown_portal_info_and_rewrites_ids_first() {
        let mut e = engine();
        e.register(MAP_GET_AT_00853130, |_, _| Ret::default());
        // One info whose +0x44 array holds two forms.
        e.register(MAP_FIRST_004B9BA0, |_, _| 1u32.into_ret());
        e.register(MAP_STEP_006B7F20, |e, a| {
            e.mem.set_u32(a[3], 0x9000);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.register(ARRAY_SIZE_0044DDC0, |_, _| 2u32.into_ret());
        let forms = e.mem.alloc(8);
        e.register_double(ARRAY_AT_00877A30, move |_, a| (forms + 4 * a[1]).into_ret());
        e.mem.set_u32(forms, 0x11);
        e.mem.set_u32(forms + 4, 0x22);
        let file = file(&mut e, 0x38, &[0x4943_564e], 0x10, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 0x3c, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_5d80, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        assert_eq!(e.mem.u32(forms), 0x8000_0011);
        assert_eq!(e.mem.u32(forms + 4), 0x8000_0022);
        assert_eq!(calls_to(&log, ARRAY_SIZE_0044DDC0)[0], vec![0x9044]);
        assert_eq!(calls_to(&log, SCOPE_LEAVE_00404EE0).len(), 1);
        assert!(calls_to(&log, DEALLOCATE).is_empty());
        assert!(calls_to(&log, SET_FROM_PORTAL_CHUNK).is_empty());
    }
}
