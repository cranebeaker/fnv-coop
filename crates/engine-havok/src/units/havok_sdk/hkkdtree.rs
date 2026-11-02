//! `Havok SDK/hkkdtree.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkKdTree` layout as the code uses it: an `hkArray` of 8-byte entries (data `+0`, size
//! `+4`, capacity and flags `+8`), `+0xc` -1 after a reset, `+0x10` the entry count of the
//! construction info, `+0x14` and `+0x18` two derived counts, an `hkArray` of 16-byte nodes
//! (data `+0x1c`, size `+0x20`, capacity and flags `+0x24`), and two 16-byte vectors at
//! `+0x30` and `+0x40`. The capacity word's bit 31 means the array does not own its memory.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router getter (`00c85750`): its words at `+0xc` and `+0x10` are allocators.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, count, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The `float`s `hkKdTreeCinfo`'s constructor splats into its two vectors: the largest
/// value (`0104da48`) and the lowest (`0104e148`).
const LARGEST_FLOAT: u32 = 0x0104_da48;
const LOWEST_FLOAT: u32 = 0x0104_e148;
/// The bit of an `hkArray` capacity word that says the memory is not owned.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d976d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCinfo::hkKdTreeCinfo` (Xbox PDB), `thiscall`: `+0` becomes -1, the bytes at `+4`
/// and `+5` become 2 and 1, the vector at `+0x10` the largest `float` (`0104da48`) in
/// every lane and the vector at `+0x20` the lowest (`0104e148`) in every lane. Answers
/// `this`.
pub fn hk_kd_tree_cinfo_hk_kd_tree_cinfo(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    let largest = e.mem.u32(LARGEST_FLOAT);
    e.mem.set_u32(at, 0xffff_ffff);
    e.mem.set_u8(at + 4, 2);
    e.mem.set_u8(at + 5, 1);
    for lane in 0..4 {
        e.mem.set_u32(at + 0x10 + 4 * lane, largest);
    }
    let lowest = e.mem.u32(LOWEST_FLOAT);
    for lane in 0..4 {
        e.mem.set_u32(at + 0x20 + 4 * lane, lowest);
    }
    this
}

// Translated from 00d97710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `key`): walks the 16-byte nodes (`+0x1c`,
/// `+0x20` of them, a signed count) for the first whose key, the 16-bit words at `+6` and
/// `+0xe` as the low and high half, equals `key`; the words at `+0`, `+2` and `+4` of that
/// node become 0xffff and those at `+8`, `+0xa` and `+0xc` become 0.
pub fn fn_00d97710(e: &mut Engine, this: Ptr, key: u32) {
    let count = e.mem.i32(this.addr() + 0x20);
    let mut node = e.mem.u32(this.addr() + 0x1c);
    let mut index = 0i32;
    while index < count {
        let low = u32::from(e.mem.u16(node + 6));
        let high = u32::from(e.mem.u16(node + 0xe));
        if low | high << 16 == key {
            for offset in [0, 2, 4] {
                e.mem.set_u16(node + offset, 0xffff);
            }
            for offset in [0xc, 0xa, 8] {
                e.mem.set_u16(node + offset, 0);
            }
            return;
        }
        index += 1;
        node = node.wrapping_add(0x10);
    }
}

// Translated from 00d97770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTree::initNodeArray` (Xbox PDB), `thiscall`, `RET 4` (`count`, raised to at least 2):
/// when the 8-byte entry array (`+0`) holds fewer than `count`, reserves room (twice the
/// capacity, or `count` when that is larger, through `hkArrayUtil::_reserve` with the memory
/// router's allocator at `+0x10`, element size 8) when the capacity is below `count`, and
/// gives the new entries the value `{0x7fffffff, 0}` (a null slot is skipped), then sets
/// the size. Afterwards the first entry is set to `{0x7fffffff, 0}` (when the data is not
/// null), its first word again and the first word of the second entry to `0x7fffffff`.
pub fn hk_kd_tree_init_node_array(e: &mut Engine, this: Ptr, count: i32) {
    let at = this.addr();
    let count = if count <= 2 { 2 } else { count };
    if e.mem.i32(at + 4) < count {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(at + 8) & CAPACITY_MASK) as i32;
        if capacity < count {
            let mut wanted = capacity.wrapping_add(capacity);
            if count >= wanted {
                wanted = count;
            }
            e.call(ARRAY_RESERVE, &args![allocator, at, wanted, 8u32]);
        }
        let size = e.mem.u32(at + 4);
        let data = e.mem.u32(at);
        let mut entry = data.wrapping_add(size.wrapping_mul(8));
        let mut missing = count.wrapping_sub(size as i32);
        if missing > 0 {
            loop {
                if entry != 0 {
                    e.mem.set_u32(entry, 0x7fff_ffff);
                    e.mem.set_u32(entry + 4, 0);
                }
                entry = entry.wrapping_add(8);
                missing = missing.wrapping_sub(1);
                if missing == 0 {
                    break;
                }
            }
        }
        e.mem.set_u32(at + 4, count as u32);
    }
    let first = e.mem.u32(at);
    if first != 0 {
        e.mem.set_u32(first, 0x7fff_ffff);
        e.mem.set_u32(first + 4, 0);
    }
    let first = e.mem.u32(at);
    e.mem.set_u32(first, 0x7fff_ffff);
    let first = e.mem.u32(at);
    e.mem.set_u32(first + 8, 0x7fff_ffff);
}

// Translated from 00d97850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTree::~hkKdTree` (Xbox PDB), `thiscall`: empties the node array (`+0x1c`, 16-byte
/// nodes) and then the entry array (`+0`, 8-byte entries): each size is cleared and, unless
/// the capacity word has the not-owned bit, the data is freed through the memory router's
/// allocator (`+0x10`, slot `0x10`: data, capacity times the element size), then the data
/// is cleared and the capacity word set to `0x80000000`.
pub fn hk_kd_tree_destructor(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(at + 0x24);
    e.mem.set_u32(at + 0x20, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & CAPACITY_MASK) << 4;
        let data = e.mem.u32(at + 0x1c);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at + 0x1c, 0);
    e.mem.set_u32(at + 0x24, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(at + 8);
    e.mem.set_u32(at + 4, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(8);
        let data = e.mem.u32(at);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at + 8, NOT_OWNED);
    e.mem.set_u32(at, 0);
}

// Translated from 00d978d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTree::reset` (Xbox PDB), `thiscall`, `RET 4` (`cinfo`, an `hkKdTreeCinfo`): sets
/// `+0xc` to -1 and copies the vectors at `cinfo + 0x10` and `+0x20` to `+0x30` and `+0x40`.
/// With `n = cinfo[0]`, `+0x14` becomes `2 * n + 8` when the byte `cinfo[5]` is set and
/// `2 * n` otherwise; `+0x18` becomes `n` divided (signed) by the byte `cinfo[4]`, or 0
/// when that is 0. The entry array is initialised for `+0x14` plus `+0x18` entries
/// (`hkKdTree::initNodeArray`), `n` is stored at `+0x10`, and the node array (`+0x1c`)
/// gets room for `n` nodes (twice its capacity when larger; `hkArrayUtil::_reserve`, the
/// router's allocator at `+0x10`, element size 0x10) and size `n`.
pub fn hk_kd_tree_reset(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    let at = this.addr();
    let info = cinfo.addr();
    e.mem.set_u32(at + 0xc, 0xffff_ffff);
    for lane in 0..4 {
        let first = e.mem.u32(info + 0x10 + 4 * lane);
        e.mem.set_u32(at + 0x30 + 4 * lane, first);
        let second = e.mem.u32(info + 0x20 + 4 * lane);
        e.mem.set_u32(at + 0x40 + 4 * lane, second);
    }
    let n = e.mem.u32(info);
    let doubled = if e.mem.u8(info + 5) != 0 {
        n.wrapping_add(n).wrapping_add(8)
    } else {
        n.wrapping_add(n)
    };
    e.mem.set_u32(at + 0x14, doubled);
    let divisor = i32::from(e.mem.u8(info + 4));
    let share = if divisor == 0 {
        0
    } else {
        (n as i32).wrapping_div(divisor)
    };
    e.mem.set_u32(at + 0x18, share as u32);
    hk_kd_tree_init_node_array(e, this, doubled.wrapping_add(share as u32) as i32);
    let n = e.mem.u32(info);
    e.mem.set_u32(at + 0x10, n);
    let nodes = at + 0x1c;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(nodes + 8) & CAPACITY_MASK) as i32;
    if capacity < n as i32 {
        let mut wanted = capacity.wrapping_add(capacity);
        if (n as i32) >= wanted {
            wanted = n as i32;
        }
        e.call(ARRAY_RESERVE, &args![allocator, nodes, wanted, 0x10u32]);
    }
    e.mem.set_u32(nodes + 4, n);
}

// Translated from 00d97970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTree::hkKdTree` (Xbox PDB), `thiscall`, `RET 4` (`cinfo`): both arrays start empty
/// (data and size 0, capacity word `0x80000000`), then `reset` (`00d978d0`) runs with
/// `cinfo`. Answers `this`.
pub fn hk_kd_tree_hk_kd_tree(e: &mut Engine, this: Ptr, cinfo: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, 0);
    e.mem.set_u32(at + 4, 0);
    e.mem.set_u32(at + 8, NOT_OWNED);
    e.mem.set_u32(at + 0x1c, 0);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u32(at + 0x24, NOT_OWNED);
    hk_kd_tree_reset(e, this, cinfo);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d976d0, hk_kd_tree_cinfo_hk_kd_tree_cinfo(Ptr) -> Ptr),
        entry!(0x00d97710, fn_00d97710(Ptr, u32)),
        entry!(0x00d97770, hk_kd_tree_init_node_array(Ptr, i32)),
        entry!(0x00d97850, hk_kd_tree_destructor(Ptr)),
        entry!(0x00d978d0, hk_kd_tree_reset(Ptr, Ptr)),
        entry!(0x00d97970, hk_kd_tree_hk_kd_tree(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_TARGET: u32 = 0x4400_5001;
    const ALLOCATOR_VTABLE: u32 = 0x4400_5100;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x2000);
        e.mem.set_u32(LARGEST_FLOAT, 0x7f7f_ffee);
        e.mem.set_u32(LOWEST_FLOAT, 0xff7f_ffee);
        e
    }

    /// A memory router whose allocator (`+0x10`) frees through a double (vtable slot
    /// `0x10`), and a reserve double that gives the array `count` entries of room.
    fn memory(e: &mut Engine) -> (Calls, Calls) {
        let frees: Calls = Rc::default();
        let reserves: Calls = Rc::default();
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, 0, 0, FREE_TARGET]);
        let sink = Rc::clone(&frees);
        e.register_double(FREE_TARGET, move |_, a| {
            sink.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&reserves);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        (frees, reserves)
    }

    #[test]
    fn the_cinfo_constructor_fills_its_defaults() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00d9_76d0, &args![cinfo]).u32(), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0xffff_ffff);
        assert_eq!((e.mem.u8(cinfo + 4), e.mem.u8(cinfo + 5)), (2, 1));
        for lane in 0..4 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * lane), 0x7f7f_ffee);
            assert_eq!(e.mem.u32(cinfo + 0x20 + 4 * lane), 0xff7f_ffee);
        }
    }

    #[test]
    fn the_node_with_the_key_is_cleared() {
        let mut e = engine();
        let tree = e.mem.alloc(0x50);
        let nodes = e.mem.alloc(0x30);
        e.mem.set_u32(tree + 0x1c, nodes);
        e.mem.set_u32(tree + 0x20, 3);
        for index in 0..3u32 {
            let node = nodes + index * 0x10;
            for offset in [0, 2, 4, 8, 0xa, 0xc] {
                e.mem.set_u16(node + offset, 0x1111);
            }
            e.mem.set_u16(node + 6, index as u16 + 1);
            e.mem.set_u16(node + 0xe, 0x20);
        }
        // Key = low word 2 | high word 0x20 << 16 matches the second node only.
        e.call(0x00d9_7710, &args![tree, 0x0020_0002u32]);
        let second = nodes + 0x10;
        for offset in [0, 2, 4] {
            assert_eq!(e.mem.u16(second + offset), 0xffff);
        }
        for offset in [8, 0xa, 0xc] {
            assert_eq!(e.mem.u16(second + offset), 0);
        }
        assert_eq!(e.mem.u16(nodes), 0x1111);
        assert_eq!(e.mem.u16(nodes + 0x20 + 8), 0x1111);
        // No match leaves everything; a zero count looks at nothing.
        e.call(0x00d9_7710, &args![tree, 0x99u32]);
        assert_eq!(e.mem.u16(nodes + 0x20), 0x1111);
        e.mem.set_u32(tree + 0x20, 0);
        e.call(0x00d9_7710, &args![tree, 0x0020_0001u32]);
        assert_eq!(e.mem.u16(nodes), 0x1111);
    }

    #[test]
    fn the_entry_array_is_reserved_and_filled_with_the_empty_entry() {
        let mut e = engine();
        let (_, reserves) = memory(&mut e);
        let tree = e.mem.alloc(0x50);
        e.mem.set_u32(tree + 8, NOT_OWNED);
        e.call(0x00d9_7770, &args![tree, 5u32]);
        let data = e.mem.u32(tree);
        assert_eq!(e.mem.u32(tree + 4), 5);
        assert_eq!(reserves.borrow().len(), 1);
        assert_eq!(reserves.borrow()[0][2..], [5, 8]);
        for index in 0..5 {
            assert_eq!(e.mem.u32(data + index * 8), 0x7fff_ffff);
            assert_eq!(e.mem.u32(data + index * 8 + 4), 0);
        }
        // A count below 2 is raised to 2; a large enough array is only re-marked.
        e.mem.set_u32(data + 4, 9);
        e.call(0x00d9_7770, &args![tree, 1u32]);
        assert_eq!(e.mem.u32(tree + 4), 5);
        assert_eq!(e.mem.u32(data + 4), 0);
        assert_eq!(reserves.borrow().len(), 1);
        // A fresh empty tree with count 0 gets 2 entries (room of 2).
        let other = e.mem.alloc(0x50);
        e.call(0x00d9_7770, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other + 4), 2);
        assert_eq!(reserves.borrow()[1][2..], [2, 8]);
    }

    #[test]
    fn the_destructor_frees_owned_arrays_only() {
        let mut e = engine();
        let (frees, _) = memory(&mut e);
        let tree = e.mem.alloc(0x50);
        for (offset, value) in [
            (0x1c, 0x6000),
            (0x20, 3),
            (0x24, 4),
            (0, 0x7000),
            (4, 2),
            (8, 6),
        ] {
            e.mem.set_u32(tree + offset, value);
        }
        e.call(0x00d9_7850, &args![tree]);
        assert_eq!(
            *frees.borrow(),
            vec![vec![0x6000, 0x40], vec![0x7000, 0x30]]
        );
        for (offset, value) in [
            (0x1c, 0),
            (0x20, 0),
            (0x24, NOT_OWNED),
            (0, 0),
            (4, 0),
            (8, NOT_OWNED),
        ] {
            assert_eq!(e.mem.u32(tree + offset), value);
        }
        // Not-owned arrays are not freed.
        frees.borrow_mut().clear();
        e.mem.set_u32(tree + 0x1c, 0x6000);
        e.call(0x00d9_7850, &args![tree]);
        assert!(frees.borrow().is_empty());
    }

    fn cinfo(e: &mut Engine, count: u32, divisor: u8, extra: u8) -> u32 {
        let cinfo = e.mem.alloc(0x30);
        e.mem.set_u32(cinfo, count);
        e.mem.set_u8(cinfo + 4, divisor);
        e.mem.set_u8(cinfo + 5, extra);
        for lane in 0..4 {
            e.mem.set_u32(cinfo + 0x10 + 4 * lane, 0x100 + lane);
            e.mem.set_u32(cinfo + 0x20 + 4 * lane, 0x200 + lane);
        }
        cinfo
    }

    #[test]
    fn reset_derives_the_counts_and_sizes_the_arrays() {
        let mut e = engine();
        let (_, reserves) = memory(&mut e);
        let tree = e.mem.alloc(0x50);
        e.mem.set_u32(tree + 8, NOT_OWNED);
        e.mem.set_u32(tree + 0x24, NOT_OWNED);
        let info = cinfo(&mut e, 8, 2, 1);
        e.call(0x00d9_78d0, &args![tree, info]);
        assert_eq!(e.mem.u32(tree + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(tree + 0x14), 24);
        assert_eq!(e.mem.u32(tree + 0x18), 4);
        assert_eq!(e.mem.u32(tree + 0x10), 8);
        assert_eq!(e.mem.u32(tree + 4), 28);
        assert_eq!(e.mem.u32(tree + 0x20), 8);
        assert_eq!(e.mem.u32(tree + 0x30), 0x100);
        assert_eq!(e.mem.u32(tree + 0x4c), 0x203);
        let asked: Vec<Vec<u32>> = reserves.borrow().iter().map(|a| a[2..].to_vec()).collect();
        assert_eq!(asked, vec![vec![28, 8], vec![8, 0x10]]);
        // Without the extra flag and with a zero divisor: 2 * count and no share.
        let info = cinfo(&mut e, 6, 0, 0);
        e.call(0x00d9_78d0, &args![tree, info]);
        assert_eq!(e.mem.u32(tree + 0x14), 12);
        assert_eq!(e.mem.u32(tree + 0x18), 0);
        // The entry array already holds 28, so its size stays; the nodes shrink to 6.
        assert_eq!(e.mem.u32(tree + 4), 28);
        assert_eq!(e.mem.u32(tree + 0x20), 6);
    }

    #[test]
    fn the_constructor_starts_empty_and_resets() {
        let mut e = engine();
        memory(&mut e);
        let tree = e.mem.alloc(0x50);
        e.mem.write(tree, &[0xcd; 0x50]);
        let info = cinfo(&mut e, 4, 1, 0);
        assert_eq!(e.call(0x00d9_7970, &args![tree, info]).u32(), tree);
        assert_eq!(e.mem.u32(tree + 0x14), 8);
        assert_eq!(e.mem.u32(tree + 0x18), 4);
        assert_eq!(e.mem.u32(tree + 4), 12);
        assert_eq!(e.mem.u32(tree + 0x20), 4);
        assert_eq!(e.mem.u32(tree + 0xc), 0xffff_ffff);
    }
}
