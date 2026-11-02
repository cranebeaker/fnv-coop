//! `NiCollision/niobbroot.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiOBBRoot` (vtable `010f1b44` by its RTTI) on PC: `m_fDeltaTime` +4, `m_pTree` +8 (an
//! `NiOBBNode`, whose virtual slots are the deleting destructor +0, `IsLeaf` +4, `TestCollision` +8,
//! `FindCollisions` +0xc and `FindIntersections` +0x10), `m_bTransformChanged` +0xc, `m_velocity`
//! +0x10, `m_xform` +0x1c and `m_uiChangeStamp` +0x50 (the Xbox PDB's names; the PC offsets from
//! the code).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiOBBRoot`'s vtable (`010f1b44`).
const NI_OBB_ROOT_VTABLE: u32 = 0x010f_1b44;
/// The memory manager instance and its `Deallocate` (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00eb5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBRoot::_scalar_deleting_destructor_` (Xbox PDB): sets the vtable, deletes the tree (its
/// virtual slot +0 with 1) when there is one, and frees `this` through the memory manager when bit
/// 0 of `flags` is set. Answers `this`.
pub fn ni_obb_root_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let tree = e.mem.u32(base + 8);
    e.mem.set_u32(base, NI_OBB_ROOT_VTABLE);
    if tree != 0 {
        e.vcall(tree, 0, &args![1u32]);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00eb5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBRoot::TestCollision` (Xbox PDB): false against itself; otherwise the tree's
/// `TestCollision` (virtual slot +8) with the other root's tree, the four words, then this root's
/// and the other root's change stamps (+0x50), and its answer.
pub fn ni_obb_root_test_collision(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> bool {
    if this == other {
        return false;
    }
    let tree = e.mem.u32(this.addr() + 8);
    let stamp = e.mem.u32(this.addr() + 0x50);
    let other_stamp = e.mem.u32(other.addr() + 0x50);
    let other_tree = e.mem.u32(other.addr() + 8);
    e.vcall(
        tree,
        8,
        &args![other_tree, first, second, third, fourth, stamp, other_stamp],
    )
    .bool()
}

// Translated from 00eb6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBRoot::FindCollisions` (Xbox PDB): 0 against itself; otherwise the tree's
/// `FindCollisions` (virtual slot +0xc) with the other root's tree, the first six words, this
/// root's and the other root's change stamps (+0x50) and the seventh word, and its answer.
pub fn ni_obb_root_find_collisions(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
) -> u32 {
    if this == other {
        return 0;
    }
    let tree = e.mem.u32(this.addr() + 8);
    let stamp = e.mem.u32(this.addr() + 0x50);
    let other_stamp = e.mem.u32(other.addr() + 0x50);
    let other_tree = e.mem.u32(other.addr() + 8);
    e.vcall(
        tree,
        0xc,
        &args![
            other_tree,
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
            stamp,
            other_stamp,
            seventh
        ],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_5fc0,
            ni_obb_root_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00eb_5ff0,
            ni_obb_root_test_collision(Ptr, Ptr, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00eb_6030,
            ni_obb_root_find_collisions(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A root whose tree's slots +0, +8 and +0xc are test functions; change stamp `stamp`.
    fn root(e: &mut Engine, stamp: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable, 0x00ab_3000);
        e.mem.set_u32(vtable + 8, 0x00ab_3008);
        e.mem.set_u32(vtable + 0xc, 0x00ab_300c);
        let tree = e.mem.alloc(0x8c);
        e.mem.set_u32(tree, vtable);
        let root = e.mem.alloc(0x54);
        e.mem.set_u32(root + 8, tree);
        e.mem.set_u32(root + 0x50, stamp);
        (root, tree)
    }

    #[test]
    fn deleting_destructor_deletes_the_tree() {
        let mut e = Engine::new();
        e.register(0x00ab_3000, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (this, tree) = root(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_5fc0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_OBB_ROOT_VTABLE);
        assert_eq!(calls_to(&e, 0x00ab_3000), vec![vec![tree, 1]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00eb_5fc0, &args![this, 0u32]);
        assert!(calls_to(&e, 0x00ab_3000).is_empty());
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
    }

    #[test]
    fn collisions_go_to_the_trees() {
        let mut e = Engine::new();
        e.register(0x00ab_3008, |_, _| 1u32.into_ret());
        e.register(0x00ab_300c, |_, _| 5u32.into_ret());
        let (a, a_tree) = root(&mut e, 11);
        let (b, b_tree) = root(&mut e, 22);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00eb_5ff0, &args![a, b, 1u32, 2u32, 3u32, 4u32])
            .bool());
        assert_eq!(
            calls_to(&e, 0x00ab_3008),
            vec![vec![a_tree, b_tree, 1, 2, 3, 4, 11, 22]]
        );
        assert!(!e
            .call(0x00eb_5ff0, &args![a, a, 1u32, 2u32, 3u32, 4u32])
            .bool());
        let words = args![a, b, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00eb_6030, &words).u32(), 5);
        assert_eq!(
            calls_to(&e, 0x00ab_300c),
            vec![vec![a_tree, b_tree, 1, 2, 3, 4, 5, 6, 11, 22, 7]]
        );
        let words = args![b, b, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00eb_6030, &words).u32(), 0);
    }
}
