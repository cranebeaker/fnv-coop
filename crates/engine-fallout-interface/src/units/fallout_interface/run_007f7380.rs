//! `fallout/interface/run_007f7380` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// List nodes (`[item, next]`): the item slot (`006815c0`), the next node (`00726070`),
/// the next-node store (`006ecd40`, `RET 4`; the map's folded name is
/// `CombatProcedureAttackMelee::Initialize`) and the node's deleting destructor
/// (`004702f0`, `RET 4`).
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
/// The interface manager (`004b7210`) and `InterfaceManager::SetCurrentFocusTarget`
/// (`00715860`, `RET 0xc`), `Tile::TextToTrait` (`00a01860`, cdecl) of `"_num_filtered"`
/// (`0106f8d0`), the tile's float trait (`00a011b0`, `RET 4`, `ST0`) and its setter
/// (`00a012d0`, `RET 0xc`), 1.0 (`01012070`, `double`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const NUM_FILTERED_NAME: u32 = 0x0106_f8d0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The list box's flag test (`007242c0`, `RET 4`), its synchronisation
/// (`ListBox<unsigned int>::Synchronize`, `00726090`), the tile's release (`004459e0`,
/// `RET 4`) and `MemoryManager` free (`00401030`, cdecl).
const LIST_BOX_FLAG_007242C0: u32 = 0x0072_42c0;
const LIST_BOX_SYNCHRONIZE: u32 = 0x0072_6090;
const TILE_RELEASE_004459E0: u32 = 0x0044_59e0;
const FREE: u32 = 0x0040_1030;

// Translated from 007f7380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a list box, `RET 8`: the node to remove and the node before
/// it): false without a node. Removes the node's item: when it is the selected tile
/// (+0x10) the selection moves to the next item's tile (or the previous one's, when not
/// the same node) and the interface focuses it (`SetCurrentFocusTarget(tile, 0xfc3,
/// 1)`); with a parent tile (+0xc) and the item's byte +8 set, the parent's
/// `"_num_filtered"` is lowered by one; the item's tile is deleted (slot 0 with 1), with
/// flag 1 (`007242c0`) its +4 released (`004459e0`), and the item freed. The node is
/// unlinked: the list's own head (+4) takes the second node's item and next (the second
/// node deleted), or becomes empty; any other node is skipped by the previous one and
/// deleted. With flag 0 the box is synchronised. True.
pub fn fn_007f7380(e: &mut Engine, this: Ptr, node: u32, previous: u32) -> bool {
    let base = this.addr();
    if node == 0 {
        return false;
    }
    let item_of = |e: &mut Engine, node: u32| {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        e.mem.u32(slot)
    };
    let item = item_of(e, node);
    if e.mem.u32(item) == e.mem.u32(base + 0x10) {
        let selected = if e.call(NODE_NEXT, &args![node]).u32() != 0 {
            let next = e.call(NODE_NEXT, &args![node]).u32();
            let item = item_of(e, next);
            e.mem.u32(item)
        } else if previous != 0 && previous != node {
            let item = item_of(e, previous);
            e.mem.u32(item)
        } else {
            0
        };
        e.mem.set_u32(base + 0x10, selected);
        if e.mem.u32(base + 0x10) != 0 {
            let tile = e.mem.u32(base + 0x10);
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            e.call(
                SET_CURRENT_FOCUS_TARGET,
                &args![manager, tile, 0xfc3u32, 1u32],
            );
        }
    }
    if e.mem.u32(base + 0xc) != 0 {
        let item = item_of(e, node);
        if e.mem.u8(item + 8) != 0 {
            let which = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_NAME]).u32();
            let parent = e.mem.u32(base + 0xc);
            let count = e.call(TILE_GET_FLOAT, &args![parent, which]).f64();
            let lowered = x87_store32(x87_sub(count, e.global::<f64>(ONE_DOUBLE)));
            let which = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_NAME]).u32();
            let parent = e.mem.u32(base + 0xc);
            e.call(
                TILE_SET_FLOAT,
                &args![parent, which, x87_quiet_bits(lowered), 1u32],
            );
        }
    }
    let item = item_of(e, node);
    let tile = e.mem.u32(item);
    if tile != 0 {
        e.vcall(tile, 0, &args![1u32]);
    }
    if e.call(LIST_BOX_FLAG_007242C0, &args![this, 1u32]).bool() {
        let item = item_of(e, node);
        let other = e.mem.u32(item + 4);
        if other != 0 {
            e.call(TILE_RELEASE_004459E0, &args![other, 1u32]);
        }
    }
    let item = item_of(e, node);
    e.call(FREE, &args![item]);
    let head = if base != 0 { base + 4 } else { 0 };
    if node == head {
        if e.mem.u32(base + 8) != 0 {
            let second = e.mem.u32(base + 8);
            let after = e.call(NODE_NEXT, &args![second]).u32();
            e.mem.set_u32(base + 8, after);
            let slot = e.call(NODE_SELF, &args![second]).u32();
            let moved = e.mem.u32(slot);
            e.mem.set_u32(base + 4, moved);
            e.call(NODE_SET_NEXT, &args![second, 0u32]);
            if second != 0 {
                e.call(NODE_DELETE, &args![second, 1u32]);
            }
        } else {
            e.mem.set_u32(base + 4, 0);
        }
    } else {
        let after = e.call(NODE_NEXT, &args![node]).u32();
        e.call(NODE_SET_NEXT, &args![previous, after]);
        e.call(NODE_SET_NEXT, &args![node, 0u32]);
        // Always true here (a null node returned early); the code tests it again.
        if node != 0 {
            e.call(NODE_DELETE, &args![node, 1u32]);
        }
    }
    if e.call(LIST_BOX_FLAG_007242C0, &args![this, 0u32]).bool() {
        e.call(LIST_BOX_SYNCHRONIZE, &args![this]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007f_7380, fn_007f7380(Ptr, u32, u32) -> bool)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    const DELETE_SLOT: u32 = 0x0e00_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        for address in [
            SET_CURRENT_FOCUS_TARGET,
            TILE_SET_FLOAT,
            LIST_BOX_SYNCHRONIZE,
            TILE_RELEASE_004459E0,
            FREE,
            NODE_DELETE,
            DELETE_SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(INTERFACE_MANAGER, |_, _| 0x9000u32.into_ret());
        e.register(TEXT_TO_TRAIT, |_, _| 0xfeedu32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register(LIST_BOX_FLAG_007242C0, |_, a| {
            u32::from(a[1] == 1).into_ret()
        });
        e.put_vtable(VTABLE, &[DELETE_SLOT]);
        e
    }

    /// A list item: its tile (with the test vtable), a second word and the byte at +8.
    fn item(e: &mut Engine, filtered: u8) -> (u32, u32) {
        let tile = e.mem.alloc(4);
        e.mem.set_u32(tile, VTABLE);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, 0x4400);
        e.mem.set_u8(item + 8, filtered);
        (item, tile)
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn removes_the_head_and_moves_the_selection() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let (first, first_tile) = item(&mut e, 1);
        let (second, second_tile) = item(&mut e, 0);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this + 4, first);
        e.mem.set_u32(this + 8, node);
        e.mem.set_u32(this + 0xc, 0x7700);
        e.mem.set_u32(this + 0x10, first_tile);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_7380, &args![this, this + 4, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0x10), second_tile);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x9000, second_tile, 0xfc3, 1]]
        );
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x7700, 0xfeed, 4.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![first_tile, 1]]);
        assert_eq!(calls(&e, TILE_RELEASE_004459E0), vec![vec![0x4400, 1]]);
        assert_eq!(calls(&e, FREE), vec![vec![first]]);
        assert_eq!(e.mem.u32(this + 4), second);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert!(calls(&e, LIST_BOX_SYNCHRONIZE).is_empty());
        assert!(!e.call(0x007f_7380, &args![this, 0u32, 0u32]).bool());
    }

    #[test]
    fn removes_a_later_node() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let (first, _) = item(&mut e, 0);
        let (second, _) = item(&mut e, 0);
        let (third, _) = item(&mut e, 0);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, third);
        let middle = e.mem.alloc(8);
        e.mem.set_u32(middle, second);
        e.mem.set_u32(middle + 4, last);
        e.mem.set_u32(this + 4, first);
        e.mem.set_u32(this + 8, middle);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_7380, &args![this, middle, this + 4]).bool());
        assert_eq!(e.mem.u32(this + 8), last);
        assert_eq!(e.mem.u32(middle + 4), 0);
        assert_eq!(calls(&e, NODE_DELETE), vec![vec![middle, 1]]);
        assert!(calls(&e, SET_CURRENT_FOCUS_TARGET).is_empty());
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
    }
}
