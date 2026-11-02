//! `Havok SDK/run_00d27f70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These are the methods of the union-find structure of `hkunionfind.cpp` (the unit's
//! neighbour `00d282c0` is named `hkUnionFind::reindex` in the Xbox PDB and uses the same two
//! fields; the PDB has no `hkUnionFind` type, so the class and the names of the functions
//! here are unconfirmed and they are named by address). `+0` points to an `hkArray<int>`
//! (whose first word is the data pointer) holding, for each node, either its parent node
//! (zero or positive) or the negated size of its group when it is a root, and `+4` is the
//! number of nodes. Roots are always the smallest index of their group.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// The union-find structure (0x8 bytes; the class is not in the Xbox PDB).
    pub struct HkUnionFind: 0x8 {
        /// Pointer to the `hkArray<int>` of parents and negated group sizes.
        0x00 m_parents: Ptr,
        /// Number of nodes.
        0x04 m_numNodes: i32,
    }
}

/// Memory router (`00c85750`): `+0xC` is the allocator the temporary table comes from (slot
/// `+0xC` allocates `(&bytes)` and updates the byte count, slot `+0x10` frees `(block,
/// bytes)`), `+0x10` the allocator arrays grow through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (cdecl `(allocator, array, element size)`): grows an `hkArray` by one chunk.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `capacityAndFlags` mask and the value of an array that owns no memory.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NO_OWNED_MEMORY: u32 = 0x8000_0000;

/// The data pointer of the parents array.
fn parents(e: &Engine, this: Ptr<HkUnionFind>) -> u32 {
    let array = e.get(this, HkUnionFind::m_parents);
    e.mem.u32(array.addr())
}

/// Address of `index` in an array of 32-bit words.
fn word(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(4))
}

fn read(e: &Engine, base: u32, index: i32) -> i32 {
    e.mem.i32(word(base, index))
}

fn write(e: &mut Engine, base: u32, index: i32, value: i32) {
    e.mem.set_i32(word(base, index), value);
}

// Translated from 00d27f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: remembers the parents array and the node count and makes every node a root
/// of its own group (parent entry -1).
pub fn fn_00d27f70(e: &mut Engine, this: Ptr<HkUnionFind>, array: Ptr, count: i32) {
    e.set(this, HkUnionFind::m_parents, array);
    e.set(this, HkUnionFind::m_numNodes, count);
    let mut index = 0;
    while index < count {
        let data = parents(e, this);
        write(e, data, index, -1);
        index += 1;
    }
}

// Translated from 00d28030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joins the groups of nodes `a` and `b`: finds both roots, compressing the paths walked,
/// and, when they differ, makes the smaller root the parent of the larger one and adds the
/// larger group's (negated) size to it.
pub fn fn_00d28030(e: &mut Engine, this: Ptr<HkUnionFind>, a: i32, b: i32) {
    let mut data = parents(e, this);
    let mut cursor_a = a;
    let parent_of_a = read(e, data, a);
    if parent_of_a >= 0 {
        let mut root = parent_of_a;
        let mut next = parent_of_a;
        while next >= 0 {
            root = next;
            next = read(e, data, root);
        }
        loop {
            let old = read(e, data, cursor_a);
            write(e, data, cursor_a, root);
            data = parents(e, this);
            cursor_a = old;
            if read(e, data, old) < 0 {
                break;
            }
        }
    }
    data = parents(e, this);
    let mut cursor_b = b;
    let parent_of_b = read(e, data, b);
    if parent_of_b >= 0 {
        let mut root = parent_of_b;
        let mut next = parent_of_b;
        while next >= 0 {
            root = next;
            next = read(e, data, root);
        }
        loop {
            let old = read(e, data, cursor_b);
            write(e, data, cursor_b, root);
            data = parents(e, this);
            cursor_b = old;
            if read(e, data, old) < 0 {
                break;
            }
        }
    }
    if cursor_a == cursor_b {
        return;
    }
    data = parents(e, this);
    let size_a = read(e, data, cursor_a);
    if cursor_a < cursor_b {
        let size_b = read(e, data, cursor_b);
        write(e, data, cursor_a, size_a.wrapping_add(size_b));
        data = parents(e, this);
        write(e, data, cursor_b, cursor_a);
    } else {
        let size_b = read(e, data, cursor_b);
        write(e, data, cursor_b, size_b.wrapping_add(size_a));
        data = parents(e, this);
        write(e, data, cursor_a, cursor_b);
    }
}

// Translated from 00d280e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flattens the forest: every node's entry is replaced by its root (entries that are
/// negative, the roots, are left alone).
pub fn fn_00d280e0(e: &mut Engine, this: Ptr<HkUnionFind>) {
    let start = parents(e, this);
    let count = e.get(this, HkUnionFind::m_numNodes);
    let end = word(start, count);
    let mut cursor = start;
    while cursor != end {
        let entry = e.mem.i32(cursor);
        if entry >= 0 && read(e, start, entry) >= 0 {
            loop {
                let entry = e.mem.i32(cursor);
                let grand = read(e, start, entry);
                e.mem.set_i32(cursor, grand);
                let data = parents(e, this);
                let entry = e.mem.i32(cursor);
                if read(e, data, entry) < 0 {
                    break;
                }
            }
        }
        cursor = cursor.wrapping_add(4);
    }
}

// Translated from 00d28120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Numbers the groups: flattens the forest, then walks the nodes in order; every root
/// appends its group size (the negated entry) to the `hkArray<int>` `sizes` and its entry
/// becomes the group's number (0, 1, 2, ... in the order of the roots), every other node
/// takes the number its root has already been given.
pub fn fn_00d28120(e: &mut Engine, this: Ptr<HkUnionFind>, sizes: Ptr) {
    fn_00d280e0(e, this);
    let array = sizes.addr();
    let mut group = 0;
    let mut index = 0;
    while index < e.get(this, HkUnionFind::m_numNodes) {
        let data = parents(e, this);
        let entry = read(e, data, index);
        if entry < 0 {
            let size = entry.wrapping_neg();
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
            }
            let count = e.mem.u32(array + 4);
            let slot = word(e.mem.u32(array), count as i32);
            e.mem.set_i32(slot, size);
            e.mem.set_u32(array + 4, count.wrapping_add(1));
            let data = parents(e, this);
            write(e, data, index, group);
            group += 1;
        } else {
            let root_entry = read(e, data, entry);
            write(e, data, index, root_entry);
        }
        index += 1;
    }
}

// Translated from 00d281b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the biggest group to number 0: finds the first largest entry of the `hkArray<int>`
/// `sizes` (compared as signed integers, from index 1 on); when it is not entry 0, builds a
/// table that exchanges 0 and that index, swaps those two entries of `sizes`, renumbers every
/// node's entry through the table and returns the index; otherwise returns 0.
pub fn fn_00d281b0(e: &mut Engine, this: Ptr<HkUnionFind>, sizes: Ptr) -> i32 {
    let count = e.mem.i32(sizes.addr() + 4);
    let data = e.mem.u32(sizes.addr());
    let mut biggest = read(e, data, 0);
    let mut best = 0;
    let mut index = 1;
    if count <= index {
        return 0;
    }
    while index < count {
        let size = read(e, data, index);
        if biggest < size {
            biggest = size;
            best = index;
        }
        index += 1;
    }
    if best == 0 {
        return 0;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let (table, capacity_flags) = if count == 0 {
        (0, NO_OWNED_MEMORY)
    } else {
        // The exe's frame (`SUB ESP,0x8`, four pushes): 0x18 bytes, the byte count at +0x10
        // (00d2820e).
        let (table, granted) = e.with_stack(0x18, |e, frame| {
            let bytes = frame.addr() + 0x10;
            e.mem.set_i32(bytes, count.wrapping_mul(4));
            let table = e.vcall(allocator, 0xc, &args![bytes]).u32();
            (table, e.mem.i32(bytes))
        });
        // Signed division by 4 (`CDQ; AND EDX, 3; ADD; SAR 2`).
        let quarter = (granted.wrapping_add((granted >> 31) & 3) >> 2) as u32;
        (
            table,
            if quarter == 0 {
                NO_OWNED_MEMORY
            } else {
                quarter
            },
        )
    };
    let mut position = 0;
    while position < count {
        e.mem.set_i32(word(table, position), position);
        position += 1;
    }
    e.mem.set_i32(table, best);
    write(e, table, best, 0);
    let sizes_data = e.mem.u32(sizes.addr());
    let first = read(e, sizes_data, 0);
    let moved = read(e, sizes_data, best);
    write(e, sizes_data, best, first);
    let sizes_data = e.mem.u32(sizes.addr());
    write(e, sizes_data, 0, moved);
    let mut node = 0;
    while node < e.get(this, HkUnionFind::m_numNodes) {
        let data = parents(e, this);
        let entry = read(e, data, node);
        let renumbered = read(e, table, entry);
        write(e, data, node, renumbered);
        node += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    if (capacity_flags as i32) >= 0 {
        let bytes = (capacity_flags & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![table, bytes]);
    }
    best
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_7f70, fn_00d27f70(Ptr<HkUnionFind>, Ptr, i32)),
        entry!(0x00d2_8030, fn_00d28030(Ptr<HkUnionFind>, i32, i32)),
        entry!(0x00d2_80e0, fn_00d280e0(Ptr<HkUnionFind>)),
        entry!(0x00d2_8120, fn_00d28120(Ptr<HkUnionFind>, Ptr)),
        entry!(0x00d2_81b0, fn_00d281b0(Ptr<HkUnionFind>, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_DOUBLE: u32 = 0x0900_0100;
    const FREE_DOUBLE: u32 = 0x0900_0110;

    fn engine() -> Engine {
        Engine::new()
    }

    /// A union-find over `entries` with its parents array.
    fn forest(e: &mut Engine, entries: &[i32]) -> (u32, u32) {
        let data = e.mem.alloc(entries.len() as u32 * 4);
        for (i, v) in entries.iter().enumerate() {
            e.mem.set_i32(data + 4 * i as u32, *v);
        }
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, entries.len() as u32);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, array);
        e.mem.set_u32(this + 4, entries.len() as u32);
        (this, data)
    }

    fn entries(e: &Engine, data: u32, count: u32) -> Vec<i32> {
        (0..count).map(|i| e.mem.i32(data + 4 * i)).collect()
    }

    #[test]
    fn constructor_makes_every_node_a_root() {
        let mut e = engine();
        let data = e.mem.alloc(12);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, data);
        let this = e.mem.alloc(8);
        e.call(0x00d2_7f70, &args![this, array, 3i32]);
        assert_eq!(e.mem.u32(this), array);
        assert_eq!(e.mem.i32(this + 4), 3);
        assert_eq!(entries(&e, data, 3), vec![-1, -1, -1]);
        // Zero nodes writes nothing.
        e.call(0x00d2_7f70, &args![this, array, 0i32]);
        assert_eq!(entries(&e, data, 3), vec![-1, -1, -1]);
    }

    #[test]
    fn join_merges_into_the_smaller_root_and_compresses_paths() {
        let mut e = engine();
        let (this, data) = forest(&mut e, &[-1, -1, -1, -1, -1]);
        e.call(0x00d2_8030, &args![this, 1i32, 3i32]);
        assert_eq!(entries(&e, data, 5), vec![-1, -2, -1, 1, -1]);
        // Joining the same group again changes nothing.
        e.call(0x00d2_8030, &args![this, 3i32, 1i32]);
        assert_eq!(entries(&e, data, 5), vec![-1, -2, -1, 1, -1]);
        e.call(0x00d2_8030, &args![this, 3i32, 4i32]);
        assert_eq!(entries(&e, data, 5), vec![-1, -3, -1, 1, 1]);
        e.call(0x00d2_8030, &args![this, 0i32, 4i32]);
        assert_eq!(entries(&e, data, 5), vec![-4, 0, -1, 1, 1]);
        // 4 -> 1 -> 0: the walk from 4 compresses both links to the root.
        e.call(0x00d2_8030, &args![this, 2i32, 4i32]);
        assert_eq!(entries(&e, data, 5), vec![-5, 0, 0, 1, 0]);
        // A larger index first still joins into the smaller root.
        let (this, data) = forest(&mut e, &[-1, -1, -1]);
        e.call(0x00d2_8030, &args![this, 2i32, 0i32]);
        assert_eq!(entries(&e, data, 3), vec![-2, -1, 0]);
    }

    #[test]
    fn flatten_points_every_node_at_its_root() {
        let mut e = engine();
        let (this, data) = forest(&mut e, &[-4, 0, 1, 2, -1, 4]);
        e.call(0x00d2_80e0, &args![this]);
        assert_eq!(entries(&e, data, 6), vec![-4, 0, 0, 0, -1, 4]);
    }

    /// Installs a double for the memory router whose `+0x10` allocator is `0x7777` and logs
    /// the calls of `00c90940` (which does not grow anything).
    fn array_router(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(ARRAY_RESERVE_MORE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x7777);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        calls
    }

    #[test]
    fn numbering_groups_collects_their_sizes() {
        let mut e = engine();
        let calls = array_router(&mut e);
        let (this, data) = forest(&mut e, &[-2, 0, -3, 2, 2, -1]);
        let sizes_data = e.mem.alloc(16);
        let sizes = e.mem.alloc(12);
        e.mem.set_u32(sizes, sizes_data);
        e.mem.set_u32(sizes + 8, 4);
        e.call(0x00d2_8120, &args![this, sizes]);
        assert_eq!(entries(&e, data, 6), vec![0, 0, 1, 1, 1, 2]);
        assert_eq!(e.mem.u32(sizes + 4), 3);
        assert_eq!(entries(&e, sizes_data, 3), vec![2, 3, 1]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn numbering_groups_grows_a_full_sizes_array() {
        let mut e = engine();
        let calls = array_router(&mut e);
        let (this, _) = forest(&mut e, &[-1, -1]);
        let sizes_data = e.mem.alloc(16);
        let sizes = e.mem.alloc(12);
        e.mem.set_u32(sizes, sizes_data);
        // Size 0 with capacity 0 (the flag bit marks memory that is not owned): full.
        e.mem.set_u32(sizes + 8, 0x8000_0000);
        e.call(0x00d2_8120, &args![this, sizes]);
        // The stand-in grows nothing, so the array stays "full" only for the first root.
        assert_eq!(*calls.borrow(), vec![vec![0x7777, sizes, 4]]);
        assert_eq!(e.mem.u32(sizes + 4), 2);
    }

    /// A router whose `+0xC` allocator allocates the requested bytes (slot `0xC`) and logs
    /// its frees (slot `0x10`).
    fn temp_allocator(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(e.mem.u32(a[1])),
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_DOUBLE, move |e, a| {
            let table: Vec<u32> = (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            free_log
                .borrow_mut()
                .push(vec![a[2], table[0], table[1], table[2]]);
            Ret::default()
        });
        log
    }

    #[test]
    fn moving_the_biggest_group_to_the_front_renumbers_the_nodes() {
        let mut e = engine();
        let frees = temp_allocator(&mut e);
        let (this, data) = forest(&mut e, &[0, 1, 2, 0, 2, 1]);
        let (sizes_owner, sizes_data) = forest(&mut e, &[1, 5, 3]);
        let sizes = e.mem.u32(sizes_owner);
        // The byte count sits at +0x10 of the exe's 0x18-byte frame (the allocator doubles it).
        let frame = e.next_stack_local(0x18);
        let got = e.call(0x00d2_81b0, &args![this, sizes]).i32();
        assert_eq!(got, 1);
        assert!(e.mem.u32(frame + 0x10) >= 12);
        assert_eq!(entries(&e, sizes_data, 3), vec![5, 1, 3]);
        assert_eq!(entries(&e, data, 6), vec![1, 0, 2, 1, 2, 0]);
        // The table (1, 0, 2) is freed with its three words.
        assert_eq!(*frees.borrow(), vec![vec![12, 1, 0, 2]]);
    }

    #[test]
    fn moving_the_biggest_group_does_nothing_when_it_is_already_first() {
        let mut e = engine();
        let _ = temp_allocator(&mut e);
        let (this, data) = forest(&mut e, &[0, 1, 2]);
        let (owner, sizes_data) = forest(&mut e, &[5, 5, 3]);
        let sizes = e.mem.u32(owner);
        assert_eq!(e.call(0x00d2_81b0, &args![this, sizes]).i32(), 0);
        assert_eq!(entries(&e, sizes_data, 3), vec![5, 5, 3]);
        assert_eq!(entries(&e, data, 3), vec![0, 1, 2]);
        // A single group has nothing to move either.
        let (owner, _) = forest(&mut e, &[9]);
        let sizes = e.mem.u32(owner);
        assert_eq!(e.call(0x00d2_81b0, &args![this, sizes]).i32(), 0);
    }
}
