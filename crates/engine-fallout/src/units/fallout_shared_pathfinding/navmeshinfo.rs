//! `fallout shared/pathfinding/navmeshinfo.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMeshInfo` (0x5C bytes) describes one navmesh to the world: its id,
//! the navmeshes next to it, the navmeshes it prefers, the doors that
//! connect it, and the loaded mesh and bounds. The three lists are
//! `BSSimpleArray`s of pointers.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `00401460(destination, source, size)`: copy (cdecl).
const COPY: u32 = 0x0040_1460;
/// `00401680(file)`: whether the file being read has the other byte order.
const FILE_SWAPS: u32 = 0x0040_1680;
/// `00401080(address, word)`: reverses the bytes of the 32-bit value at
/// `address` (cdecl; the second word is never read).
const SWAP_32: u32 = 0x0040_1080;
/// `00485d50(&form id, file)`: `TESForm::AddCompileIndex` (the map name of
/// the callee the decompiler calls so): rewrites the form id in place with
/// the load order of `file` (cdecl).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `0045af00(tes)`: `TES::GetNavMeshInfoMap` (map name): the world's
/// navmesh info map; `ECX` is the global `TES` pointer at `011dea10`.
const GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// The global `TES *` (`TES::ms_pkTES`-like; held at `011dea10`).
const TES_POINTER: u32 = 0x011d_ea10;
/// `00693d70(map, form id)`: the `NavMeshInfo` of the map for a form id, or 0.
const MAP_FIND: u32 = 0x0069_3d70;
/// `006f2290(array, count, flag)`: `BSSimpleArray::SetSize`.
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `00877a30(array, index)`: the address of element `index`.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `0044ddc0(array)`: the element count of the array.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `006bf8f0(array, index, count)`: removes `count` elements from `index` on.
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// `004839c0(form id)`: the form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `005b5e40(format, ...)`: writes a line to the log (cdecl).
const PRINT_LINE: u32 = 0x005b_5e40;
/// `"NavMeshInfo %08x, Could not find Door %08x referenced by NavMeshInfo"`.
const MISSING_DOOR_FORMAT: u32 = 0x0106_bb08;
/// `006d3100(object, flags)`: scalar deleting destructor of the object at
/// `+0x54`.
const MESH_DESTROY: u32 = 0x006d_3100;
/// `006b4850(object, flags)`: scalar deleting destructor of the object at
/// `+0x58`.
const BOUNDS_DESTROY: u32 = 0x006b_4850;
/// `005e04f0(array)`: destructor body of the door array.
const DOORS_DESTRUCT: u32 = 0x005e_04f0;
/// `0069af10(array)`: destructor body of the two navmesh arrays.
const NAV_MESH_ARRAY_DESTRUCT: u32 = 0x0069_af10;
/// `operator delete(block)` (`00401030`, cdecl).
const DELETE: u32 = 0x0040_1030;

layout! {
    /// `NavMeshInfo` (Xbox PDB), 0x5C bytes; only the members this unit
    /// touches.
    pub struct NavMeshInfo: 0x5C {
        /// `NavMeshID` (Xbox PDB).
        0x00 NavMeshID: u32,
        /// `AdjacentNavMeshes` (Xbox PDB): `BSSimpleArray<NavMeshInfo *, 1024>`.
        0x24 AdjacentNavMeshes: Inline<BSSimpleArray>,
        /// `PreferredAdjacentNavMeshes` (Xbox PDB):
        /// `BSSimpleArray<NavMeshInfo *, 1024>`.
        0x34 PreferredAdjacentNavMeshes: Inline<BSSimpleArray>,
        /// `ConnectedDoors` (Xbox PDB): `BSSimpleArray<TESObjectREFR *, 1024>`.
        0x44 ConnectedDoors: Inline<BSSimpleArray>,
        /// `pNavMesh` (Xbox PDB): `NavMesh *`.
        0x54 pNavMesh: Ptr,
        /// `pBounds` (Xbox PDB): `NavMeshBounds *`.
        0x58 pBounds: Ptr,
    }
}

// Translated from 006b4790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b4790 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor body of `NavMeshInfo` (no Xbox PDB name; `006b6390` is the
/// deleting wrapper): destroys the mesh at `+0x54` and the bounds at
/// `+0x58` when present (`006d3100`, `006b4850`, flags 0 and 1), then the
/// door array at `+0x44` (`005e04f0`) and the two navmesh arrays at `+0x34`
/// and `+0x24` (`0069af10`). The exception-unwinding frame is not
/// translated.
pub fn fn_006b4790(e: &mut Engine, this: Ptr<NavMeshInfo>) {
    let mesh = e.get(this, NavMeshInfo::pNavMesh);
    if !mesh.is_null() {
        e.call(MESH_DESTROY, &args![mesh, 0u32]);
    }
    let bounds = e.get(this, NavMeshInfo::pBounds);
    if !bounds.is_null() {
        e.call(BOUNDS_DESTROY, &args![bounds, 1u32]);
    }
    let base = this.addr();
    e.call(DOORS_DESTRUCT, &args![base.wrapping_add(0x44)]);
    e.call(NAV_MESH_ARRAY_DESTRUCT, &args![base.wrapping_add(0x34)]);
    e.call(NAV_MESH_ARRAY_DESTRUCT, &args![base.wrapping_add(0x24)]);
}

/// Reads the 32-bit value at `address`, reverses its bytes when the file has
/// the other byte order, and has the file's load order applied to it.
fn read_form_id(e: &mut Engine, file: Ptr, address: u32) -> u32 {
    e.with_stack(4, |e, local| {
        let value = e.mem.u32(address);
        e.mem.set_u32(local.addr(), value);
        if e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(SWAP_32, &args![local, 0u32]);
        }
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    })
}

/// Reads the element count at `cursor` (copied into a stack word, reversed
/// when the file has the other byte order), sets the array's size to it
/// (`006f2290`, flag 1) and moves the cursor past it. Returns the count.
fn read_count(e: &mut Engine, array: u32, cursor: &mut u32, file: Ptr) -> u32 {
    let count = e.with_stack(4, |e, local| {
        e.call(COPY, &args![local, *cursor, 4u32]);
        if e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(SWAP_32, &args![local, 0u32]);
        }
        e.mem.u32(local.addr())
    });
    e.call(ARRAY_SET_SIZE, &args![array, count, 1u32]);
    *cursor = cursor.wrapping_add(4);
    count
}

/// Reads a list of navmesh ids into `array`, storing for each the
/// `NavMeshInfo` the map finds for it (0 for id 0).
fn read_nav_mesh_list(e: &mut Engine, array: u32, cursor: &mut u32, file: Ptr) {
    let count = read_count(e, array, cursor, file);
    for index in 0..count {
        let id = read_form_id(e, file, *cursor);
        if id == 0 {
            let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            e.mem.set_u32(slot, 0);
        } else {
            let tes = e.global::<u32>(TES_POINTER);
            let map = e.call(GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
            let info = e.call(MAP_FIND, &args![map, id]).u32();
            let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            e.mem.set_u32(slot, info);
        }
        *cursor = cursor.wrapping_add(4);
    }
}

// Translated from 006b51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo::SetFromPortalChunk` (Xbox PDB): reads the portal chunk at
/// `data` of `file` (every word reversed when `00401680` says the file has
/// the other byte order, every form id rewritten to the load order of
/// `file` by `00485d50`). First the adjacent navmeshes (`+0x24`): a count,
/// then that many ids, each stored as the `NavMeshInfo` the world's map
/// finds for it (0 for id 0). With `version >= 11` the same for the
/// preferred navmeshes (`+0x34`). Last the connected doors (`+0x44`): a
/// count, then that many ids stored as they are. Each list is sized by
/// `006f2290` first.
pub fn nav_mesh_info_set_from_portal_chunk(
    e: &mut Engine,
    this: Ptr<NavMeshInfo>,
    data: u32,
    file: Ptr,
    version: i32,
) {
    let base = this.addr();
    let mut cursor = data;
    read_nav_mesh_list(e, base.wrapping_add(0x24), &mut cursor, file);
    if version >= 11 {
        read_nav_mesh_list(e, base.wrapping_add(0x34), &mut cursor, file);
    }
    let doors = base.wrapping_add(0x44);
    let count = read_count(e, doors, &mut cursor, file);
    for index in 0..count {
        let id = read_form_id(e, file, cursor);
        let slot = e.call(ARRAY_ELEMENT, &args![doors, index]).u32();
        e.mem.set_u32(slot, id);
        cursor = cursor.wrapping_add(4);
    }
}

// Translated from 006b5490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo::InitDoorPointers` (Xbox PDB): replaces each form id in the
/// door array (`+0x44`) with the form `004839c0` finds for it. A door that
/// is not found is logged (`005b5e40`: the info's id and the door's id) and
/// removed from the array (`006bf8f0`), and the walk continues with the
/// element that took its place.
pub fn nav_mesh_info_init_door_pointers(e: &mut Engine, this: Ptr<NavMeshInfo>) {
    let doors = this.addr().wrapping_add(0x44);
    let mut count = e.call(ARRAY_COUNT, &args![doors]).u32();
    let mut index = 0u32;
    while index < count {
        let slot = e.call(ARRAY_ELEMENT, &args![doors, index]).u32();
        let door_id = e.mem.u32(slot);
        let form = e.call(LOOKUP_FORM, &args![door_id]).u32();
        let slot = e.call(ARRAY_ELEMENT, &args![doors, index]).u32();
        e.mem.set_u32(slot, form);
        let slot = e.call(ARRAY_ELEMENT, &args![doors, index]).u32();
        if e.mem.u32(slot) == 0 {
            let info_id = e.get(this, NavMeshInfo::NavMeshID);
            e.call(PRINT_LINE, &args![MISSING_DOOR_FORMAT, info_id, door_id]);
            e.call(ARRAY_REMOVE, &args![doors, index, 1u32]);
            count = count.wrapping_sub(1);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006b6390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfo::~NavMeshInfo` (Xbox PDB name of the map; this is the
/// scalar deleting destructor): runs the destructor body (`006b4790`) and,
/// when bit 0 of `flags` is set, frees the object. Returns `this`.
pub fn nav_mesh_info_destructor(
    e: &mut Engine,
    this: Ptr<NavMeshInfo>,
    flags: u32,
) -> Ptr<NavMeshInfo> {
    fn_006b4790(e, this);
    if flags & 1 != 0 {
        e.call(DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b4790, fn_006b4790(Ptr<NavMeshInfo>)),
        entry!(
            0x006b51a0,
            nav_mesh_info_set_from_portal_chunk(Ptr<NavMeshInfo>, u32, Ptr, i32)
        ),
        entry!(
            0x006b5490,
            nav_mesh_info_init_door_pointers(Ptr<NavMeshInfo>)
        ),
        entry!(
            0x006b6390,
            nav_mesh_info_destructor(Ptr<NavMeshInfo>, u32) -> Ptr<NavMeshInfo>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    #[test]
    fn destructor_body_releases_the_members_in_order() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                MESH_DESTROY,
                BOUNDS_DESTROY,
                DOORS_DESTRUCT,
                NAV_MESH_ARRAY_DESTRUCT,
            ],
        );
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.set(info, NavMeshInfo::pNavMesh, Ptr::new(0x7000));
        e.set(info, NavMeshInfo::pBounds, Ptr::new(0x7100));
        let base = info.addr();
        e.call_log = Some(vec![]);
        e.call(0x006b4790, &args![info]);
        assert_eq!(
            calls(&e)[1..],
            [
                (MESH_DESTROY, vec![0x7000, 0]),
                (BOUNDS_DESTROY, vec![0x7100, 1]),
                (DOORS_DESTRUCT, vec![base + 0x44]),
                (NAV_MESH_ARRAY_DESTRUCT, vec![base + 0x34]),
                (NAV_MESH_ARRAY_DESTRUCT, vec![base + 0x24]),
            ]
        );
    }

    #[test]
    fn destructor_body_skips_the_missing_members() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                MESH_DESTROY,
                BOUNDS_DESTROY,
                DOORS_DESTRUCT,
                NAV_MESH_ARRAY_DESTRUCT,
            ],
        );
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006b4790, &args![info]);
        assert_eq!(calls(&e).len(), 4);
    }

    #[test]
    fn deleting_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                MESH_DESTROY,
                BOUNDS_DESTROY,
                DOORS_DESTRUCT,
                NAV_MESH_ARRAY_DESTRUCT,
                DELETE,
            ],
        );
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.call_log = Some(vec![]);
        let result = e.call(0x006b6390, &args![info, 0u32]).ptr::<NavMeshInfo>();
        assert_eq!(result, info);
        assert!(calls(&e).iter().all(|c| c.0 != DELETE));
        e.call(0x006b6390, &args![info, 1u32]);
        assert_eq!(calls(&e).last().unwrap(), &(DELETE, vec![info.addr()]));
    }

    /// Doubles for a chunk read: array sizes allocate a buffer, byte swaps
    /// and `FILE_SWAPS` follow `swaps`, compile indexes add 0x0100_0000 to
    /// non-zero ids, the map gives back `id + 0x100`.
    fn chunk_doubles(e: &mut Engine, swaps: bool) {
        e.register_double(FILE_SWAPS, move |_, _| Ret {
            eax: u32::from(swaps),
            ..Ret::default()
        });
        e.register(SWAP_32, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let value = e.mem.u32(a[0]);
            if value != 0 {
                e.mem.set_u32(a[0], value | 0x0100_0000);
            }
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            let buffer = e.mem.alloc(a[1].max(1) * 4);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + 4 * a[1],
            ..Ret::default()
        });
        e.register(GET_NAV_MESH_INFO_MAP, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(MAP_FIND, |_, a| Ret {
            eax: a[1] + 0x100,
            ..Ret::default()
        });
        e.map(TES_POINTER, 4);
        e.set_global(TES_POINTER, 0x5000u32);
    }

    fn words(e: &Engine, array: u32) -> Vec<u32> {
        let count = e.mem.u32(array + 8);
        let buffer = e.mem.u32(array + 4);
        (0..count).map(|i| e.mem.u32(buffer + 4 * i)).collect()
    }

    fn chunk(e: &mut Engine, words: &[u32], swapped: bool) -> u32 {
        let data = e.mem.alloc(4 * words.len() as u32);
        for (i, w) in words.iter().enumerate() {
            let w = if swapped { w.swap_bytes() } else { *w };
            e.mem.set_u32(data + 4 * i as u32, w);
        }
        data
    }

    #[test]
    fn portal_chunk_of_an_old_version_has_two_lists() {
        let mut e = Engine::new();
        chunk_doubles(&mut e, false);
        // adjacent: 2 ids (7, 0); doors: 1 id (9).
        let data = chunk(&mut e, &[2, 7, 0, 1, 9], false);
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006b51a0, &args![info, data, 0x6000u32, 10u32]);
        let base = info.addr();
        assert_eq!(words(&e, base + 0x24), vec![0x0100_0007 + 0x100, 0]);
        assert_eq!(words(&e, base + 0x34), Vec::<u32>::new());
        assert_eq!(words(&e, base + 0x44), vec![0x0100_0009]);
        let log = calls(&e);
        assert_eq!(log[1], (COPY, vec![log[1].1[0], data, 4]));
        assert!(log.contains(&(ARRAY_SET_SIZE, vec![base + 0x24, 2, 1])));
        assert!(log.contains(&(ARRAY_SET_SIZE, vec![base + 0x44, 1, 1])));
        assert!(log.iter().all(|c| c.0 != SWAP_32));
        assert!(log.contains(&(GET_NAV_MESH_INFO_MAP, vec![0x5000])));
        assert!(log.contains(&(MAP_FIND, vec![0x5001, 0x0100_0007])));
    }

    #[test]
    fn portal_chunk_of_version_eleven_adds_the_preferred_list() {
        let mut e = Engine::new();
        chunk_doubles(&mut e, false);
        let data = chunk(&mut e, &[1, 7, 2, 8, 9, 0], false);
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.call(0x006b51a0, &args![info, data, 0x6000u32, 11u32]);
        let base = info.addr();
        assert_eq!(words(&e, base + 0x24), vec![0x0100_0007 + 0x100]);
        assert_eq!(
            words(&e, base + 0x34),
            vec![0x0100_0008 + 0x100, 0x0100_0009 + 0x100]
        );
        assert_eq!(words(&e, base + 0x44), Vec::<u32>::new());
    }

    #[test]
    fn portal_chunk_reverses_the_words_of_a_file_in_the_other_order() {
        let mut e = Engine::new();
        chunk_doubles(&mut e, true);
        let data = chunk(&mut e, &[1, 7, 1, 9], true);
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006b51a0, &args![info, data, 0x6000u32, 10u32]);
        let base = info.addr();
        assert_eq!(words(&e, base + 0x24), vec![0x0100_0007 + 0x100]);
        assert_eq!(words(&e, base + 0x44), vec![0x0100_0009]);
        let swaps = calls(&e).iter().filter(|c| c.0 == SWAP_32).count();
        assert_eq!(swaps, 4);
    }

    #[test]
    fn init_door_pointers_drops_the_doors_that_are_gone() {
        let mut e = Engine::new();
        let info: Ptr<NavMeshInfo> = e.new_object();
        e.set(info, NavMeshInfo::NavMeshID, 0x00ab_cdef);
        let doors = info.addr() + 0x44;
        let buffer = e.mem.alloc(12);
        for (i, id) in [0x10u32, 0x20, 0x30].iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *id);
        }
        e.mem.set_u32(doors + 4, buffer);
        e.mem.set_u32(doors + 8, 3);
        e.register(ARRAY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(ARRAY_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + 4 * a[1],
            ..Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] == 0x20 { 0 } else { a[0] + 1 },
            ..Ret::default()
        });
        silent(&mut e, &[PRINT_LINE]);
        e.register(ARRAY_REMOVE, |e, a| {
            // Shift the later elements down and shrink the array.
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            for i in a[1]..size - 1 {
                let next = e.mem.u32(buffer + 4 * (i + 1));
                e.mem.set_u32(buffer + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006b5490, &args![info]);
        assert_eq!(words(&e, doors), vec![0x11, 0x31]);
        let log = calls(&e);
        assert!(log.contains(&(PRINT_LINE, vec![MISSING_DOOR_FORMAT, 0x00ab_cdef, 0x20])));
        assert!(log.contains(&(ARRAY_REMOVE, vec![doors, 1, 1])));
        assert_eq!(log.iter().filter(|c| c.0 == LOOKUP_FORM).count(), 3);
    }
}
