//! `fallout shared/pathfinding/navmeshportal.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Whether the file's data needs byte swapping (`00401680`, thiscall on the
/// `TESFile`, a `bool` in AL).
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// Byte-swaps the word at its first argument (`00401080`, cdecl, two words).
const SWAP_WORD: u32 = 0x0040_1080;
/// Byte-swaps the `u16` at its first argument (`00407a90`, cdecl, two words).
const SWAP_HALF: u32 = 0x0040_7a90;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: the form id's address and
/// the file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TES::GetNavMeshInfoMap` (`0045af00`, thiscall on `TES`, no stack word).
const TES_GET_NAVMESH_INFO_MAP: u32 = 0x0045_af00;
/// `00693d70` (`navmesh_loadsave.cpp`, thiscall on the map
/// `TES::GetNavMeshInfoMap` returns, `RET 4`: the form id).
const NAVMESH_FROM_INFO_MAP: u32 = 0x0069_3d70;
/// `NavMeshTriangleEdgePortal` (Xbox PDB), as the PC code writes it: the
/// navmesh pointer at +0, the triangle index (`u16`) at +4.
const NAVMESH: u32 = 0x00;
const TRIANGLE: u32 = 0x04;

// Translated from 0069dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangleEdgePortal::Load` (Xbox PDB; thiscall, `RET 0xc`): reads
/// the portal from the record data at `data` of `file`. The navmesh form id
/// (word at `data`, byte-swapped when the file swaps) is resolved through
/// `TESForm::AddCompileIndex`; zero gives no navmesh, otherwise the navmesh
/// comes from `TES::GetNavMeshInfoMap` and `00693d70`. The triangle index is
/// the `u16` at `data + 4`, byte-swapped in place when the file swaps. The
/// first stack word is never read.
pub fn nav_mesh_triangle_edge_portal_load(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    file: Ptr,
    data: Ptr,
) {
    let base = this.addr();
    let form_id = e.mem.u32(data.addr());
    let navmesh = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), form_id);
        if e.call(FILE_SWAPS_ENDIAN, &args![file]).u32() & 0xff != 0 {
            e.call(SWAP_WORD, &args![local, 0u32]);
        }
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    });
    if navmesh == 0 {
        e.mem.set_u32(base + NAVMESH, 0);
    } else {
        let tes = e.global::<u32>(TES_SINGLETON);
        // 0069e026: `0045af00` takes no stack word (plain RET); the pushed form id
        // is the word of `00693d70` (RET 4).
        let map = e.call(TES_GET_NAVMESH_INFO_MAP, &args![tes]).u32();
        let pointer = e.call(NAVMESH_FROM_INFO_MAP, &args![map, navmesh]).u32();
        e.mem.set_u32(base + NAVMESH, pointer);
    }
    let triangle = e.mem.u16(data.addr().wrapping_add(4));
    e.mem.set_u16(base + TRIANGLE, triangle);
    if e.call(FILE_SWAPS_ENDIAN, &args![file]).u32() & 0xff != 0 {
        e.call(SWAP_HALF, &args![base + TRIANGLE, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0069_dfd0,
        nav_mesh_triangle_edge_portal_load(Ptr, u32, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn setup(e: &mut Engine, swaps: bool) {
        if swaps {
            e.register(FILE_SWAPS_ENDIAN, |_, _| Ret {
                eax: 0xffff_ff01,
                ..Ret::default()
            });
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| Ret {
                eax: 0xffff_ff00,
                ..Ret::default()
            });
        }
        e.register(SWAP_WORD, |e, a| {
            let v = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e.register(SWAP_HALF, |e, a| {
            let v = e.mem.u16(a[0]).swap_bytes();
            e.mem.set_u16(a[0], v);
            Ret::default()
        });
        // The compile index: adds 0x0100_0000 unless the id is zero.
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let v = e.mem.u32(a[0]);
            if v != 0 {
                e.mem.set_u32(a[0], v + 0x0100_0000);
            }
            Ret::default()
        });
        e.map(TES_SINGLETON, 4);
        e.mem.set_u32(TES_SINGLETON, 0x0777_0000);
        e.register(TES_GET_NAVMESH_INFO_MAP, |_, _| Ret {
            eax: 0x5000_0000,
            ..Ret::default()
        });
        e.register(NAVMESH_FROM_INFO_MAP, |_, a| Ret {
            eax: (a[0] ^ a[1]) + 1,
            ..Ret::default()
        });
    }

    #[test]
    fn load_resolves_the_navmesh_and_keeps_the_triangle() {
        let mut e = Engine::new();
        setup(&mut e, false);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xdddd_dddd);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0000_1234);
        e.mem.set_u16(data + 4, 0x0102);
        e.call_log = Some(vec![]);
        e.call(0x0069_dfd0, &args![this, 9u32, 0x0888_0000u32, data]);
        assert_eq!(e.mem.u32(this), (0x0100_1234 ^ 0x5000_0000) + 1);
        assert_eq!(e.mem.u32(this + 4), 0xdddd_0102);
        assert_eq!(
            calls_to(&e, TES_GET_NAVMESH_INFO_MAP),
            vec![vec![0x0777_0000]]
        );
        assert_eq!(
            calls_to(&e, NAVMESH_FROM_INFO_MAP),
            vec![vec![0x5000_0000, 0x0100_1234]]
        );
        assert!(calls_to(&e, SWAP_WORD).is_empty());
        assert!(calls_to(&e, SWAP_HALF).is_empty());
        assert_eq!(calls_to(&e, FILE_SWAPS_ENDIAN).len(), 2);
    }

    #[test]
    fn load_swaps_both_fields_and_a_zero_id_gives_no_navmesh() {
        let mut e = Engine::new();
        setup(&mut e, true);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1111_1111);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0);
        e.mem.set_u16(data + 4, 0x0102);
        e.call_log = Some(vec![]);
        e.call(0x0069_dfd0, &args![this, 0u32, 0x0888_0000u32, data]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0x0201);
        // The record data itself is left as it was.
        assert_eq!(e.mem.u16(data + 4), 0x0102);
        assert!(calls_to(&e, TES_GET_NAVMESH_INFO_MAP).is_empty());
        assert_eq!(calls_to(&e, SWAP_HALF), vec![vec![this + 4, 0]]);
        // A swapped non-zero id.
        e.mem.set_u32(data, 0x3412_0000);
        e.call(0x0069_dfd0, &args![this, 0u32, 0x0888_0000u32, data]);
        assert_eq!(e.mem.u32(this), (0x0100_1234 ^ 0x5000_0000) + 1);
    }
}
