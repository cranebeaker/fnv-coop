//! `(unplaced)/run_00bc5a80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word): allocates.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map, `00b7e330`, `thiscall`,
/// `RET 8`: the word at `+0x18` and the map's kind).
const CONSTANT_MAP_CONSTRUCTOR: u32 = 0x00b7_e330;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The constant names (`010babec` "FogProps", `010a1e50` "WorldViewProjTranspose",
/// `010bb0ac` "fvars0") and the word `01011584` each entry is given.
const FOG_PROPS: u32 = 0x010b_abec;
const WORLD_VIEW_PROJ_TRANSPOSE: u32 = 0x010a_1e50;
const FVARS0: u32 = 0x010b_b0ac;
const ENTRY_DATA_01011584: u32 = 0x0101_1584;

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// A new constant map of `kind` (0x44 bytes, null when the allocation fails) assigned to the
/// counted pointer at `holder`.
fn make_map(e: &mut Engine, this: u32, holder: u32, kind: u32) {
    let block = e.call(ALLOCATE, &args![0x44u32]).u32();
    let map = if block == 0 {
        0
    } else {
        let owner = e.mem.u32(this + 0x18);
        e.call(CONSTANT_MAP_CONSTRUCTOR, &args![block, owner, kind])
            .u32()
    };
    let old = e.mem.u32(holder);
    if old != map {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(holder, map);
        if map != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![map + 4]);
        }
    }
}

// Translated from 00bc5a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a virtual slot of the vtable at `010bb07c`; `ECX` only): makes
/// the shader's two constant maps when missing. Without a map at `+0x30`, one of kind 2
/// (`00b7e330` with the word at `+0x18`) is made, counted, and given the entry "FogProps"
/// (its slot 8 with `0x10000007, 0, 0, 1, 01011584, 0x10, 4, this + 0x98, 0`). Without one
/// at `+0x34`, one of kind 0 gets "WorldViewProjTranspose" (slot `0x68` with
/// `0x20000009, 0, 01011584, 0, 0, 0, 0`) and "fvars0" (slot 8 with `0x10000007, 0, 4, 1,
/// 01011584, 0x10, 4, this + 0x88, 0`). A failed allocation assigns null and the entry call
/// then goes through it, as in the exe.
// Translation incomplete: 00bc5a80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) and the /GS cookie word are not translated.
pub fn fn_00bc5a80(e: &mut Engine, this: Ptr) {
    let shader = this.addr();
    if e.mem.u32(shader + 0x30) == 0 {
        make_map(e, shader, shader + 0x30, 2);
        let map = e.mem.u32(shader + 0x30);
        e.vcall(
            map,
            8,
            &args![
                FOG_PROPS,
                0x1000_0007u32,
                0u32,
                0u32,
                1u32,
                ENTRY_DATA_01011584,
                0x10u32,
                4u32,
                shader + 0x98,
                0u32
            ],
        );
    }
    if e.mem.u32(shader + 0x34) == 0 {
        make_map(e, shader, shader + 0x34, 0);
        let map = e.mem.u32(shader + 0x34);
        e.vcall(
            map,
            0x68,
            &args![
                WORLD_VIEW_PROJ_TRANSPOSE,
                0x2000_0009u32,
                0u32,
                ENTRY_DATA_01011584,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        );
        let map = e.mem.u32(shader + 0x34);
        e.vcall(
            map,
            8,
            &args![
                FVARS0,
                0x1000_0007u32,
                0u32,
                4u32,
                1u32,
                ENTRY_DATA_01011584,
                0x10u32,
                4u32,
                shader + 0x88,
                0u32
            ],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_5a80, fn_00bc5a80(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP_VTABLE: u32 = 0x0300_0000;
    const ADD_ENTRY: u32 = 0x0300_1000;
    const ADD_MATRIX: u32 = 0x0300_2000;
    const DELETE_THIS: u32 = 0x0300_3000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x1b];
        slots[1] = DELETE_THIS;
        slots[2] = ADD_ENTRY;
        slots[0x68 / 4] = ADD_MATRIX;
        e.put_vtable(MAP_VTABLE, &slots);
        for address in [ADD_ENTRY, ADD_MATRIX, DELETE_THIS] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn both_maps_are_made_with_their_entries() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x18, 0x4400);
        e.call(0x00bc_5a80, &args![this]);
        let (fog, vertex) = (e.mem.u32(this + 0x30), e.mem.u32(this + 0x34));
        assert_ne!(fog, 0);
        assert_ne!(vertex, 0);
        assert_eq!((e.mem.u32(fog + 4), e.mem.u32(vertex + 4)), (1, 1));
        let made = calls(&e, CONSTANT_MAP_CONSTRUCTOR);
        assert_eq!((made[0][1], made[0][2], made[1][2]), (0x4400, 2, 0));
        let entries = calls(&e, ADD_ENTRY);
        assert_eq!(
            entries[0],
            vec![
                fog,
                FOG_PROPS,
                0x1000_0007,
                0,
                0,
                1,
                ENTRY_DATA_01011584,
                0x10,
                4,
                this + 0x98,
                0
            ]
        );
        assert_eq!(
            entries[1],
            vec![
                vertex,
                FVARS0,
                0x1000_0007,
                0,
                4,
                1,
                ENTRY_DATA_01011584,
                0x10,
                4,
                this + 0x88,
                0
            ]
        );
        assert_eq!(
            calls(&e, ADD_MATRIX),
            vec![vec![
                vertex,
                WORLD_VIEW_PROJ_TRANSPOSE,
                0x2000_0009,
                0,
                ENTRY_DATA_01011584,
                0,
                0,
                0,
                0
            ]]
        );
        // Present maps are kept.
        e.call_log = Some(vec![]);
        e.call(0x00bc_5a80, &args![this]);
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCTOR).is_empty());
        assert!(calls(&e, ADD_ENTRY).is_empty());
    }
}
