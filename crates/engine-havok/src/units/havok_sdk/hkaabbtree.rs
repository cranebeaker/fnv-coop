//! `Havok SDK/hkaabbtree.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its +0x10 word is the allocator object (virtual function
/// at +8 frees a block of a given size, at +0x10 frees an array's storage).
pub(crate) const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl: allocator, array, capacity, element
/// size) and `00c90940` (cdecl: allocator, array, element size; makes room for one more).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
pub(crate) const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The node accessors of the two node managers: `00d9a4c0` (`this` = manager, one word:
/// index; data + index * 0x30) and `00d9a4d0` (data + `u16` index * 0x14), both `RET 4`.
pub(crate) const NODE_AT_WIDE: u32 = 0x00d9_a4c0;
pub(crate) const NODE_AT_NARROW: u32 = 0x00d9_a4d0;
/// Root fix-up functions called by the builders: `00d9af30` (`this` = tree, root node, root
/// index, `0x7fffffff`; `RET 0xc`) and `00d9b450` (the same with `0x7fff`).
const FIX_UP_WIDE: u32 = 0x00d9_af30;
const FIX_UP_NARROW: u32 = 0x00d9_b450;
/// Floats read from the exe image: zero (`01011d78`), the half `0.5f` (`01016248`) and the
/// `0.5` double (`01011588`), the split tolerance `2^-22` (`010d409c`), the largest and
/// smallest start values of a bounds search (`0104da48`, `0104e148`), the sign mask
/// (`01017d10`, four words), the clamp limit (`011b6d70`, four floats) and the global zero
/// vector (`01267e30`, four floats).
pub(crate) const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_HALF: u32 = 0x0101_6248;
const DOUBLE_HALF: u32 = 0x0101_1588;
const SPLIT_TOLERANCE: u32 = 0x010d_409c;
const BOUNDS_START_MIN: u32 = 0x0104_da48;
const BOUNDS_START_MAX: u32 = 0x0104_e148;
const ABS_MASK: u32 = 0x0101_7d10;
const CLAMP_LIMIT: u32 = 0x011b_6d70;
pub(crate) const ZERO_VECTOR: u32 = 0x0126_7e30;
/// `FLOAT_ONE` (`01012068`): the scale lanes the default-constructed wide tree starts with.
pub(crate) const FLOAT_ONE: u32 = 0x0101_2068;
/// The constant vector of ones (`010c7170`) the narrow tree constructor takes its lane 3 from.
pub(crate) const ONES_VECTOR: u32 = 0x010c_7170;
/// Vtable pointers of the two tree classes.
const WIDE_TREE_VTABLE: u32 = 0x010d_40a4;
const NARROW_TREE_VTABLE: u32 = 0x010d_40ac;
/// "No node" in the parent field and root index of each node kind, and the leaf bit.
pub(crate) const NONE_WIDE: u32 = 0x7fff_ffff;
pub(crate) const NONE_NARROW: u16 = 0x7fff;
pub(crate) const LEAF_WIDE: u32 = 0x8000_0000;
pub(crate) const LEAF_NARROW: u16 = 0x8000;
/// Largest value a capacity word can hold (the top two bits are flags).
pub(crate) const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Four `float` lanes.
pub(crate) type Vec4 = [f32; 4];

pub(crate) fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// `MINPS dst, src` per lane: `dst` when it is less than `src`, else `src`.
pub(crate) fn min_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] < src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `MAXPS dst, src` per lane: `dst` when it is greater than `src`, else `src`.
pub(crate) fn max_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] > src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

pub(crate) fn add_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

pub(crate) fn sub_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

pub(crate) fn mul_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

/// `CMPLEPS` of `a` against `b` as lane masks, then the three low bits of `MOVMSKPS`.
pub(crate) fn all_three_le(a: Vec4, b: Vec4) -> bool {
    (0..3).all(|lane| a[lane] <= b[lane])
}

/// The allocator object: the word at +0x10 of the memory router's structure.
pub(crate) fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// `hkArray` growth to at least `wanted` elements of `element_size` bytes (array header: data,
/// size, capacity with flag bits in the top two): when the capacity is below `wanted`, calls
/// `_reserve` with the larger of twice the capacity and `wanted`.
pub(crate) fn reserve_at_least(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let alloc = allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let new_capacity = capacity.wrapping_mul(2).max(wanted);
        e.call(
            ARRAY_RESERVE,
            &args![alloc, array, new_capacity as u32, element_size],
        );
    }
}

/// Frees the storage of an array (header: data, size, capacity) the way the node managers and
/// the builders end: the allocator's function at +0x10 with (data, capacity * `element_size`),
/// only when the capacity word is not negative (storage owned by the array).
pub(crate) fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let alloc = allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            alloc,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

fn node_wide(e: &mut Engine, manager: u32, index: u32) -> u32 {
    e.call(NODE_AT_WIDE, &args![manager, index]).u32()
}

fn node_narrow(e: &mut Engine, manager: u32, index: u32) -> u32 {
    e.call(NODE_AT_NARROW, &args![manager, index]).u32()
}

// Translated from 00d9be20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Node manager reset (fastcall, `this` = manager, no stack arguments): clears the node
/// count (+4) and the free-list count (+0x10).
pub fn fn_00d9be20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
}

// Translated from 00d9be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::clear` (Xbox PDB): resets the node
/// manager at +0x1c, clears +0x10, sets the root index (+0x14) to `0x7fffffff` and clears the
/// leaf count (+0x18).
pub fn hk_aabb_tree_aabb_uint_clear(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    fn_00d9be20(e, Ptr::new(me + 0x1c));
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, NONE_WIDE);
    e.mem.set_u32(me + 0x18, 0);
}

// Translated from 00d9bed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `clear` of the 16-bit tree (fastcall, no stack arguments): like the wide one, but the
/// root index at +0x14 is the word `0x7fff`.
pub fn fn_00d9bed0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    fn_00d9be20(e, Ptr::new(me + 0x1c));
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u16(me + 0x14, NONE_NARROW);
    e.mem.set_u32(me + 0x18, 0);
}

// Translated from 00d9bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes a word onto the 16-bit node manager's free list (thiscall on the manager, `RET 4`;
/// the array header is at +0xC): grows it with `00c90940` (element size 2) when its size
/// equals its capacity, stores `value` and bumps the size.
pub fn fn_00d9bef0(e: &mut Engine, this: Ptr, value: u16) {
    let list = this.addr() + 0xc;
    let alloc = allocator(e);
    if e.mem.u32(list + 4) == e.mem.u32(list + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![alloc, list, 2u32]);
    }
    let data = e.mem.u32(list);
    let size = e.mem.u32(list + 4);
    e.mem
        .set_u16(data.wrapping_add(size.wrapping_mul(2)), value);
    e.mem.set_u32(list + 4, size.wrapping_add(1));
}

// Translated from 00d9c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A one-instruction thunk: jumps to `00d9bef0` (same arguments, `RET 4`).
pub fn fn_00d9c2d0(e: &mut Engine, this: Ptr, value: u16) {
    fn_00d9bef0(e, this, value);
}

// Translated from 00d9bf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::NodeManager::reserve` (Xbox
/// PDB), thiscall on the manager, `RET 0xC`: makes the node array (0x14-byte nodes; data +0,
/// count +4, capacity +8) hold `count` (the low word of the first argument) nodes, giving the
/// new ones an empty flag word (+0x10), and the free list (words; header +0xC) hold their
/// indices from the highest down to the first new one. When the word at `*root_index` is not
/// `0x7fff` the pointer at `*root_out` is refreshed to that node in the possibly moved array.
pub fn hk_aabb_tree_aabb16_node_manager_reserve(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    root_out: Ptr,
    root_index: Ptr,
) {
    let me = this.addr();
    let old_size = e.mem.i32(me + 4);
    let count = i32::from(count as u16);
    reserve_at_least(e, me, count, 0x14);
    let size = e.mem.u32(me + 4);
    let data = e.mem.u32(me);
    let mut node = data.wrapping_add(size.wrapping_mul(0x14));
    let mut left = count.wrapping_sub(size as i32);
    if left > 0 {
        loop {
            if node != 0 {
                e.mem.set_u16(node + 0x10, 0);
            }
            node = node.wrapping_add(0x14);
            left -= 1;
            if left == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(me + 4, count as u32);
    let list = me + 0xc;
    reserve_at_least(e, list, count, 2);
    let mut index = count - 1;
    if index >= old_size {
        loop {
            let list_data = e.mem.u32(list);
            let list_size = e.mem.u32(list + 4);
            e.mem.set_u16(
                list_data.wrapping_add(list_size.wrapping_mul(2)),
                index as u16,
            );
            e.mem.set_u32(list + 4, list_size.wrapping_add(1));
            index -= 1;
            if index < old_size {
                break;
            }
        }
    }
    let root = e.mem.u16(root_index.addr());
    if root != NONE_NARROW {
        let data = e.mem.u32(me);
        e.mem.set_u32(
            root_out.addr(),
            data.wrapping_add(u32::from(root).wrapping_mul(0x14)),
        );
    }
}

// Translated from 00d9c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wide tree's node manager `reserve` (thiscall on the manager, `RET 0xC`): like the 16-bit
/// one with 0x30-byte nodes whose word at +0x28 starts at 0, a `count` that is a full signed
/// word, a free list of dwords and the root index `*root_index` compared with `0x7fffffff`.
pub fn fn_00d9c450(e: &mut Engine, this: Ptr, count: i32, root_out: Ptr, root_index: Ptr) {
    let me = this.addr();
    let old_size = e.mem.i32(me + 4);
    reserve_at_least(e, me, count, 0x30);
    let size = e.mem.u32(me + 4);
    let data = e.mem.u32(me);
    let mut node = size.wrapping_mul(0x30).wrapping_add(data);
    let mut left = count.wrapping_sub(size as i32);
    if left > 0 {
        loop {
            if node != 0 {
                e.mem.set_u32(node + 0x28, 0);
            }
            node = node.wrapping_add(0x30);
            left -= 1;
            if left == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(me + 4, count as u32);
    let list = me + 0xc;
    reserve_at_least(e, list, count, 4);
    let mut index = count - 1;
    if index >= old_size {
        loop {
            let list_data = e.mem.u32(list);
            let list_size = e.mem.u32(list + 4);
            e.mem.set_u32(
                list_data.wrapping_add(list_size.wrapping_mul(4)),
                index as u32,
            );
            e.mem.set_u32(list + 4, list_size.wrapping_add(1));
            index -= 1;
            if index < old_size {
                break;
            }
        }
    }
    let root = e.mem.u32(root_index.addr());
    if root != NONE_WIDE {
        let data = e.mem.u32(me);
        e.mem
            .set_u32(root_out.addr(), root.wrapping_mul(0x30).wrapping_add(data));
    }
}

// Translated from 00d9c010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::NodeManager::~NodeManager` (Xbox
/// PDB), fastcall: empties the free list (header +0xC) and the node array (header +0) through
/// the allocator's function at +0x10 (dword entries, 0x30-byte nodes; only for storage the
/// array owns), leaving both with no data and the capacity word `0x80000000`.
pub fn hk_aabb_tree_aabb_uint_node_manager_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    release_array(e, me + 0xc, 4);
    release_array(e, me, 0x30);
}

// Translated from 00d9c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the 16-bit tree's node manager (fastcall): as the wide one with word
/// free-list entries and 0x14-byte nodes.
pub fn fn_00d9c090(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    release_array(e, me + 0xc, 2);
    release_array(e, me, 0x14);
}

// Translated from 00d9c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::hkAabbTree<hkAabbTreeData::
/// hkAabbTreeAabb_unsigned_int>` (Xbox PDB), fastcall: installs the vtable (`010d40a4`), empties
/// both node manager arrays (capacity words `0x80000000`), copies the global vector at
/// `01267e30` to +0x40 and +0x50, clears +0x10 and +0x18, sets the root index (+0x14) to
/// `0x7fffffff` and fills +0x60 and +0x70 with (one, one, one, zero) from `01012068` /
/// `01011d78`.
pub fn hk_aabb_tree_aabb_uint_construct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, WIDE_TREE_VTABLE);
    for at in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30] {
        let value = if at == 0x24 || at == 0x30 {
            0x8000_0000
        } else {
            0
        };
        e.mem.set_u32(me + at, value);
    }
    let global = [
        e.mem.u32(ZERO_VECTOR),
        e.mem.u32(ZERO_VECTOR + 4),
        e.mem.u32(ZERO_VECTOR + 8),
        e.mem.u32(ZERO_VECTOR + 12),
    ];
    for target in [0x40u32, 0x50] {
        for (lane, bits) in global.into_iter().enumerate() {
            e.mem.set_u32(me + target + 4 * lane as u32, bits);
        }
    }
    let one = e.mem.u32(FLOAT_ONE);
    let zero = e.mem.u32(FLOAT_ZERO);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x14, NONE_WIDE);
    for target in [0x60u32, 0x70] {
        for (lane, bits) in [one, one, one, zero].into_iter().enumerate() {
            e.mem.set_u32(me + target + 4 * lane as u32, bits);
        }
    }
}

// Translated from 00d9c180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit tree's constructor (thiscall, `RET 0xC`; three vector pointers): installs the
/// vtable (`010d40ac`), empties both manager arrays, copies the vectors to +0x40, +0x50 and
/// +0x60, computes +0x70 as the refined reciprocal of the third (lane 3 taken from the ones
/// vector at `010c7170`: `RCPPS` then `(r + r) - (r * x) * r`), clears +0x10 and +0x18 and sets the root word
/// (+0x14) to `0x7fff`.
/// Answers `this` (`MOV EAX,ECX`; the caller stores it, 00d08e53).
pub fn fn_00d9c180(e: &mut Engine, this: Ptr, offset: Ptr, extra: Ptr, scale: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, NARROW_TREE_VTABLE);
    for at in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30] {
        let value = if at == 0x24 || at == 0x30 {
            0x8000_0000
        } else {
            0
        };
        e.mem.set_u32(me + at, value);
    }
    let first = load_vec(e, offset.addr());
    store_vec(e, me + 0x40, first);
    let second = load_vec(e, extra.addr());
    store_vec(e, me + 0x50, second);
    let third = load_vec(e, scale.addr());
    store_vec(e, me + 0x60, third);
    let mut x = third;
    x[3] = e.mem.f32(ONES_VECTOR + 12);
    let r = x.map(rcp_estimate);
    let refined = sub_ps(add_ps(r, r), mul_ps(mul_ps(r, x), r));
    store_vec(e, me + 0x70, refined);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u16(me + 0x14, NONE_NARROW);
    this
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
pub(crate) fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

// Translated from 00d9c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::NodeManager::pushBack` (Xbox
/// PDB), thiscall on the manager, `RET 0xC`: when the free list (words at +0xC, count +0x10)
/// is empty first runs `reserve` for (low word of the node count + 0x80) nodes with
/// (`root_out`, `root_index`); then takes the last free index, stores the five dwords at
/// `node` into that node and returns the index.
pub fn hk_aabb_tree_aabb16_node_manager_push_back(
    e: &mut Engine,
    this: Ptr,
    root_out: Ptr,
    root_index: Ptr,
    node: Ptr,
) -> u32 {
    let me = this.addr();
    if e.mem.u32(me + 0x10) == 0 {
        let count = 0x80u16.wrapping_add(e.mem.u16(me + 4));
        hk_aabb_tree_aabb16_node_manager_reserve(e, this, u32::from(count), root_out, root_index);
    }
    let list_size = e.mem.u32(me + 0x10);
    let list = e.mem.u32(me + 0xc);
    let index = u32::from(
        e.mem
            .u16(list.wrapping_add(list_size.wrapping_mul(2)).wrapping_sub(2)),
    );
    e.mem.set_u32(me + 0x10, list_size.wrapping_sub(1));
    let target = e.mem.u32(me).wrapping_add(index.wrapping_mul(0x14));
    for word in 0..5u32 {
        let value = e.mem.u32(node.addr() + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
    index
}

// Translated from 00d9c280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::NodeManager::newNode` (Xbox
/// PDB), thiscall on the manager, `RET 0xC`: like `pushBack` without the copy; the taken free
/// index is stored as a word at `index_out` and the node's address is returned.
pub fn hk_aabb_tree_aabb16_node_manager_new_node(
    e: &mut Engine,
    this: Ptr,
    root_out: Ptr,
    root_index: Ptr,
    index_out: Ptr,
) -> u32 {
    let me = this.addr();
    if e.mem.u32(me + 0x10) == 0 {
        let count = 0x80u16.wrapping_add(e.mem.u16(me + 4));
        hk_aabb_tree_aabb16_node_manager_reserve(e, this, u32::from(count), root_out, root_index);
    }
    let list = e.mem.u32(me + 0xc);
    let list_size = e.mem.u32(me + 0x10);
    let index = e
        .mem
        .u16(list.wrapping_add(list_size.wrapping_mul(2)).wrapping_sub(2));
    e.mem.set_u16(index_out.addr(), index);
    e.mem.set_u32(me + 0x10, list_size.wrapping_sub(1));
    let index = e.mem.u16(index_out.addr());
    e.mem
        .u32(me)
        .wrapping_add(u32::from(index).wrapping_mul(0x14))
}

// Translated from 00d9cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wide node manager's `pushBack` (thiscall on the manager, `RET 0xC`): like the 16-bit
/// one with dword indices, a size + 0x80 `reserve`, 0x30-byte nodes and all twelve words of
/// `node` copied; returns the index.
pub fn fn_00d9cd80(e: &mut Engine, this: Ptr, root_out: Ptr, root_index: Ptr, node: Ptr) -> u32 {
    let me = this.addr();
    if e.mem.u32(me + 0x10) == 0 {
        let count = e.mem.u32(me + 4).wrapping_add(0x80);
        fn_00d9c450(e, this, count as i32, root_out, root_index);
    }
    let list = e.mem.u32(me + 0xc);
    let list_size = e.mem.u32(me + 0x10);
    let index = e
        .mem
        .u32(list.wrapping_add(list_size.wrapping_mul(4)).wrapping_sub(4));
    e.mem.set_u32(me + 0x10, list_size.wrapping_sub(1));
    let target = index.wrapping_mul(0x30).wrapping_add(e.mem.u32(me));
    for word in 0..12u32 {
        let value = e.mem.u32(node.addr() + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
    index
}

// Translated from 00d9ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wide node manager's `newNode` (thiscall on the manager, `RET 0xC`): takes a free index
/// (growing first when there is none), stores it at `index_out` and returns the node's address.
pub fn fn_00d9ce00(
    e: &mut Engine,
    this: Ptr,
    root_out: Ptr,
    root_index: Ptr,
    index_out: Ptr,
) -> u32 {
    let me = this.addr();
    if e.mem.u32(me + 0x10) == 0 {
        let count = e.mem.u32(me + 4).wrapping_add(0x80);
        fn_00d9c450(e, this, count as i32, root_out, root_index);
    }
    let list_size = e.mem.u32(me + 0x10);
    let list = e.mem.u32(me + 0xc);
    let index = e
        .mem
        .u32(list.wrapping_add(list_size.wrapping_mul(4)).wrapping_sub(4));
    e.mem.set_u32(index_out.addr(), index);
    e.mem.set_u32(me + 0x10, list_size.wrapping_sub(1));
    let index = e.mem.u32(index_out.addr());
    index.wrapping_mul(0x30).wrapping_add(e.mem.u32(me))
}

// Translated from 00d9c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::scalar deleting destructor` (Xbox
/// PDB), thiscall, `RET 4`: stores the vtable pointer, clears the tree, runs the node manager's
/// destructor (at +0x1c) and, when bit 0 of `flags` is set, frees the 0x80-byte object through
/// the allocator's function at +8. Returns `this`.
pub fn hk_aabb_tree_aabb_uint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, WIDE_TREE_VTABLE);
    hk_aabb_tree_aabb_uint_clear(e, this);
    hk_aabb_tree_aabb_uint_node_manager_destruct(e, Ptr::new(me + 0x1c));
    if flags & 1 != 0 {
        let alloc = allocator(e);
        e.vcall(alloc, 8, &args![this, 0x80u32]);
    }
    this
}

// Translated from 00d9c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::scalar deleting destructor`
/// (Xbox PDB): the same sequence for the 16-bit tree (vtable `010d40ac`).
pub fn hk_aabb_tree_aabb16_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, NARROW_TREE_VTABLE);
    fn_00d9bed0(e, this);
    fn_00d9c090(e, Ptr::new(me + 0x1c));
    if flags & 1 != 0 {
        let alloc = allocator(e);
        e.vcall(alloc, 8, &args![this, 0x80u32]);
    }
    this
}

/// The sign bit of `value` as the lowest bit of `MOVMSKPS`.
fn sign_bit(value: f32) -> u32 {
    value.to_bits() >> 31
}

/// The nearer-child test of the insertion walks: `(d1 - d0)` per lane, summed as
/// `lane2 + (lane1 + lane0)` (`SHUFPS`/`ADDSS`); the sign bit of the sum.
fn nearer_child_sign(d0: Vec4, d1: Vec4) -> u32 {
    let diff = sub_ps(d1, d0);
    sign_bit(diff[2] + (diff[1] + diff[0]))
}

// Translated from 00d9afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts the leaf `leaf` into the wide tree below node `root` (thiscall, `RET 0xC`;
/// `new_node` is a free internal node to join the leaf and the leaf it is paired with). Node
/// fields (0x30 bytes): bounds +0 / +0x10, children +0x20 / +0x24 (a leaf keeps its data at
/// +0x20), parent index and leaf bit 31 at +0x28, next-leaf link at +0x2C. Walks down from
/// `root` choosing the child whose bounds centre is nearer the leaf's (the sign of the summed
/// lane differences of squared offsets; a non-negative sum goes to child 1), then makes
/// `new_node` the parent of that node and the leaf (bounds = union, links updated), hangs it
/// in the old parent (or makes it the root: `this + 0x10` pointer, `this + 0x14` index) and
/// widens the bounds of every ancestor until one already contains the leaf.
pub fn fn_00d9afe0(e: &mut Engine, this: Ptr, root: u32, leaf: u32, new_node: u32) {
    let me = this.addr();
    let manager = me + 0x1c;
    let leaf_node = node_wide(e, manager, leaf);
    let joint = node_wide(e, manager, new_node);
    let leaf_min = load_vec(e, leaf_node);
    let leaf_max = load_vec(e, leaf_node + 0x10);
    let sum = add_ps(leaf_min, leaf_max);
    let mut current_index = root;
    let mut current = node_wide(e, manager, root);
    let mut direction = 0u32;
    while e.mem.u32(current + 0x28) & LEAF_WIDE == 0 {
        let first_index = e.mem.u32(current + 0x20);
        let first = node_wide(e, manager, first_index);
        let second_index = e.mem.u32(current + 0x24);
        let second = node_wide(e, manager, second_index);
        let first_offset = sub_ps(add_ps(load_vec(e, first + 0x10), load_vec(e, first)), sum);
        let second_offset = sub_ps(add_ps(load_vec(e, second + 0x10), load_vec(e, second)), sum);
        let d0 = mul_ps(first_offset, first_offset);
        let d1 = mul_ps(second_offset, second_offset);
        direction = !nearer_child_sign(d0, d1) & 1;
        current_index = e.mem.u32(current + 0x20 + direction * 4);
        current = node_wide(e, manager, current_index);
    }
    let parent_index = e.mem.u32(current + 0x28) & 0x7fff_ffff;
    let parent = if parent_index != NONE_WIDE {
        node_wide(e, manager, parent_index)
    } else {
        0
    };
    let union_min = min_ps(load_vec(e, leaf_node), load_vec(e, current));
    store_vec(e, joint, union_min);
    let union_max = max_ps(load_vec(e, leaf_node + 0x10), load_vec(e, current + 0x10));
    store_vec(e, joint + 0x10, union_max);
    e.mem.set_u32(joint + 0x20, current_index);
    e.mem.set_u32(joint + 0x24, leaf);
    let flags = e.mem.u32(current + 0x28) & LEAF_WIDE;
    e.mem.set_u32(current + 0x28, flags | new_node);
    let flags = e.mem.u32(leaf_node + 0x28) & LEAF_WIDE;
    e.mem.set_u32(leaf_node + 0x28, flags | new_node);
    let old_next = e.mem.u32(current + 0x2c);
    e.mem.set_u32(current + 0x2c, leaf);
    e.mem.set_u32(leaf_node + 0x2c, old_next);
    e.mem.set_u32(joint + 0x2c, old_next);
    if parent == 0 {
        e.mem.set_u32(me + 0x10, joint);
        let flags = e.mem.u32(joint + 0x28);
        e.mem.set_u32(joint + 0x28, flags | NONE_WIDE);
        e.mem.set_u32(me + 0x14, new_node);
        return;
    }
    e.mem.set_u32(parent + 0x20 + direction * 4, new_node);
    let flags = e.mem.u32(joint + 0x28) & LEAF_WIDE;
    e.mem.set_u32(joint + 0x28, flags | parent_index);
    if direction == 1 {
        let first_index = e.mem.u32(parent + 0x20);
        let mut walker = node_wide(e, manager, first_index);
        e.mem.set_u32(walker + 0x2c, new_node);
        while e.mem.u32(walker + 0x28) & LEAF_WIDE == 0 {
            let next_index = e.mem.u32(walker + 0x24);
            walker = node_wide(e, manager, next_index);
            e.mem.set_u32(walker + 0x2c, new_node);
        }
    }
    let mut ancestor = parent;
    loop {
        let ancestor_min = load_vec(e, ancestor);
        let ancestor_max = load_vec(e, ancestor + 0x10);
        let contained =
            all_three_le(ancestor_min, leaf_min) && all_three_le(leaf_max, ancestor_max);
        if contained {
            break;
        }
        store_vec(e, ancestor, min_ps(ancestor_min, leaf_min));
        store_vec(e, ancestor + 0x10, max_ps(ancestor_max, leaf_max));
        let up = e.mem.u32(ancestor + 0x28) & 0x7fff_ffff;
        if up == NONE_WIDE {
            break;
        }
        ancestor = node_wide(e, manager, up);
        if ancestor == 0 {
            break;
        }
    }
}

// Translated from 00d9b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the leaf `index` out of the wide tree (thiscall, `RET 8`): its parent joins node is
/// freed for reuse (its index is stored at `parent_out`) and the leaf's sibling replaces it in
/// the grandparent (or becomes the root: `this + 0x10` pointer, `this + 0x14` index). The
/// bounds of the ancestors are refitted (stopping at the first whose upper bound does not
/// change; the answer is that node, else the root pointer read before) and the next-leaf links
/// of the left neighbours are repaired. The exe reads through the parent pointer even when the
/// leaf is the root (parent absent), which faults there; this translation reads the same
/// addresses. Returns the node pointer described above.
pub fn fn_00d9b240(e: &mut Engine, this: Ptr, index: u32, parent_out: Ptr) -> u32 {
    let me = this.addr();
    let manager = me + 0x1c;
    let node = node_wide(e, manager, index);
    let parent_index = e.mem.u32(node + 0x28) & 0x7fff_ffff;
    let parent = if parent_index != NONE_WIDE {
        node_wide(e, manager, parent_index)
    } else {
        0
    };
    let grand_index = e.mem.u32(parent + 0x28) & 0x7fff_ffff;
    let grand = if grand_index != NONE_WIDE {
        node_wide(e, manager, grand_index)
    } else {
        0
    };
    let first_index = e.mem.u32(parent + 0x20);
    let first = node_wide(e, manager, first_index);
    let second_index = e.mem.u32(parent + 0x24);
    let second = node_wide(e, manager, second_index);
    let sibling = first ^ second ^ node;
    let sibling_index = first_index ^ second_index ^ index;
    e.mem.set_u32(parent_out.addr(), parent_index);
    let answer = if grand == 0 {
        e.mem.set_u32(me + 0x10, sibling);
        let flags = e.mem.u32(sibling + 0x28);
        e.mem.set_u32(sibling + 0x28, flags | NONE_WIDE);
        e.mem.set_u32(me + 0x14, sibling_index);
        e.mem.u32(me + 0x10)
    } else {
        let right_index = e.mem.u32(grand + 0x24);
        let right = node_wide(e, manager, right_index);
        let slot = u32::from(right == parent);
        e.mem.set_u32(grand + 0x20 + slot * 4, sibling_index);
        let flags = e.mem.u32(sibling + 0x28) & LEAF_WIDE;
        e.mem.set_u32(sibling + 0x28, flags | grand_index);
        let mut found = e.mem.u32(me + 0x10);
        if slot == 1 {
            let left_index = e.mem.u32(grand + 0x20);
            let mut walker = node_wide(e, manager, left_index);
            let is_leaf = e.mem.u32(walker + 0x28) & LEAF_WIDE != 0;
            e.mem.set_u32(walker + 0x2c, sibling_index);
            if !is_leaf {
                loop {
                    let next_index = e.mem.u32(walker + 0x24);
                    walker = node_wide(e, manager, next_index);
                    e.mem.set_u32(walker + 0x2c, sibling_index);
                    if e.mem.u32(walker + 0x28) & LEAF_WIDE != 0 {
                        break;
                    }
                }
            }
        }
        let mut ancestor = grand;
        loop {
            let right_index = e.mem.u32(ancestor + 0x24);
            let right = node_wide(e, manager, right_index);
            let left_index = e.mem.u32(ancestor + 0x20);
            let left = node_wide(e, manager, left_index);
            let new_min = min_ps(load_vec(e, left), load_vec(e, right));
            let new_max = max_ps(load_vec(e, left + 0x10), load_vec(e, right + 0x10));
            let old_max = load_vec(e, ancestor + 0x10);
            if (0..3).all(|lane| old_max[lane] == new_max[lane]) {
                found = ancestor;
                break;
            }
            store_vec(e, ancestor, new_min);
            store_vec(e, ancestor + 0x10, new_max);
            let up = e.mem.u32(ancestor + 0x28) & 0x7fff_ffff;
            if up == NONE_WIDE {
                break;
            }
            ancestor = node_wide(e, manager, up);
            if ancestor == 0 {
                break;
            }
        }
        found
    };
    let left_index = e.mem.u32(parent + 0x20);
    let left = node_wide(e, manager, left_index);
    if left == sibling {
        let is_leaf = e.mem.u32(sibling + 0x28) & LEAF_WIDE != 0;
        let next = e.mem.u32(parent + 0x2c);
        e.mem.set_u32(sibling + 0x2c, next);
        if !is_leaf {
            let mut walker = sibling;
            loop {
                let next_index = e.mem.u32(walker + 0x24);
                walker = node_wide(e, manager, next_index);
                e.mem.set_u32(walker + 0x2c, next);
                if e.mem.u32(walker + 0x28) & LEAF_WIDE != 0 {
                    break;
                }
            }
        }
    }
    answer
}

// Translated from 00d9be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the bounds of the wide tree's leaf `index` with the 0x20 bytes at `bounds`
/// (thiscall, `RET 8`). When the leaf is the root (`this + 0x14`) that is all; otherwise the
/// leaf is first taken out (`00d9b240`) and then inserted again at the root (`00d9afe0`),
/// reusing the freed joint node.
pub fn fn_00d9be50(e: &mut Engine, this: Ptr, index: u32, bounds: Ptr) {
    let me = this.addr();
    let manager = me + 0x1c;
    if e.mem.u32(me + 0x14) == index {
        let node = node_wide(e, manager, index);
        let max = load_vec(e, bounds.addr() + 0x10);
        store_vec(e, node + 0x10, max);
        let min = load_vec(e, bounds.addr());
        store_vec(e, node, min);
        return;
    }
    let joint = e.with_stack(4, |e, slot| {
        fn_00d9b240(e, this, index, slot);
        e.mem.u32(slot.addr())
    });
    let node = node_wide(e, manager, index);
    let max = load_vec(e, bounds.addr() + 0x10);
    let min = load_vec(e, bounds.addr());
    store_vec(e, node, min);
    store_vec(e, node + 0x10, max);
    let root = e.mem.u32(me + 0x14);
    fn_00d9afe0(e, this, root, index, joint);
}

// Translated from 00d9d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a leaf to the wide tree (thiscall, `RET 8`): builds a 0x30-byte node from the 0x20 bytes at `bounds`, `data` at +0x20
/// and the leaf bit at +0x28, pushes it into the node manager (`00d9cd80`) and returns its
/// index. The first leaf becomes the root (parent field `0x7fffffff`, next link
/// `0x7fffffff`); later ones are joined with a fresh node through `00d9afe0`. Counts the leaf
/// (`this + 0x18`). The node's words at +0x24 and +0x2C start as stack leftovers in the exe
/// (zero here); both are overwritten before use.
pub fn fn_00d9d110(e: &mut Engine, this: Ptr, bounds: Ptr, data: u32) -> u32 {
    let me = this.addr();
    let manager = me + 0x1c;
    let index = e.with_stack(0x30, |e, block| {
        let at = block.addr();
        let min = load_vec(e, bounds.addr());
        store_vec(e, at, min);
        let max = load_vec(e, bounds.addr() + 0x10);
        store_vec(e, at + 0x10, max);
        e.mem.set_u32(at + 0x20, data);
        e.mem.set_u32(at + 0x28, LEAF_WIDE);
        fn_00d9cd80(
            e,
            Ptr::new(manager),
            Ptr::new(me + 0x10),
            Ptr::new(me + 0x14),
            block,
        )
    });
    if e.mem.u32(me + 0x14) == NONE_WIDE {
        let node = node_wide(e, manager, index);
        let flags = e.mem.u32(node + 0x28);
        e.mem.set_u32(node + 0x28, flags | NONE_WIDE);
        e.mem.set_u32(node + 0x2c, NONE_WIDE);
        let count = e.mem.u32(me + 0x18);
        e.mem.set_u32(me + 0x18, count.wrapping_add(1));
        e.mem.set_u32(me + 0x10, node);
        e.mem.set_u32(me + 0x14, index);
        return index;
    }
    let joint = e.with_stack(4, |e, slot| {
        fn_00d9ce00(
            e,
            Ptr::new(manager),
            Ptr::new(me + 0x10),
            Ptr::new(me + 0x14),
            slot,
        );
        e.mem.u32(slot.addr())
    });
    let root = e.mem.u32(me + 0x14);
    fn_00d9afe0(e, this, root, index, joint);
    let count = e.mem.u32(me + 0x18);
    e.mem.set_u32(me + 0x18, count.wrapping_add(1));
    index
}

/// The six words of a narrow node's bounds (+0..+0xB): lanes 0..2 are the upper corner, 3..5
/// the lower corner.
pub(crate) fn narrow_bounds(e: &Engine, node: u32) -> [u16; 6] {
    std::array::from_fn(|word| e.mem.u16(node + 2 * word as u32))
}

/// The union of two word boxes: words 0..2 are upper bounds (larger wins), 3..5 lower bounds
/// (smaller wins).
fn union_words(a: [u16; 6], b: [u16; 6]) -> [u16; 6] {
    std::array::from_fn(|word| {
        if word < 3 {
            a[word].max(b[word])
        } else {
            a[word].min(b[word])
        }
    })
}

/// `(min + max) * 0.5` of a narrow node's bounds as the exe builds it from `FILD`: lanes
/// (w3 + w0, w4 + w1, w5 + w2, zero + zero), times the half constant.
fn narrow_centre(e: &Engine, bounds: [u16; 6]) -> Vec4 {
    let zero = e.mem.f32(FLOAT_ZERO);
    let half = e.mem.f32(FLOAT_HALF);
    let low = [
        f32::from(bounds[3]),
        f32::from(bounds[4]),
        f32::from(bounds[5]),
        zero,
    ];
    let high = [
        f32::from(bounds[0]),
        f32::from(bounds[1]),
        f32::from(bounds[2]),
        zero,
    ];
    mul_ps(add_ps(low, high), [half; 4])
}

// Translated from 00d9b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::addLeafInternal` (Xbox PDB),
/// thiscall, `RET 0xC`: the 16-bit counterpart of `00d9afe0` (nodes of 0x14 bytes: six bounds
/// words, children words at +0xC / +0xE, parent word and leaf bit 15 at +0x10, next-leaf link
/// word at +0x12; only the low words of the three arguments are used). The child whose centre
/// is nearer the leaf's is chosen by the sign of the same summed lane differences, but here a
/// negative sum goes to child 1.
pub fn hk_aabb_tree_aabb16_add_leaf_internal(
    e: &mut Engine,
    this: Ptr,
    root: u32,
    leaf: u32,
    new_node: u32,
) {
    let me = this.addr();
    let manager = me + 0x1c;
    let leaf_node = node_narrow(e, manager, leaf);
    let joint = node_narrow(e, manager, new_node);
    let leaf_words = narrow_bounds(e, leaf_node);
    let leaf_centre = narrow_centre(e, leaf_words);
    let new_node = new_node as u16;
    let leaf = leaf as u16;
    let mut current_index = u32::from(root as u16);
    let mut direction = 0u32;
    let mut current = node_narrow(e, manager, root);
    while e.mem.u16(current + 0x10) & LEAF_NARROW == 0 {
        let first_index = u32::from(e.mem.u16(current + 0xc));
        let first = node_narrow(e, manager, first_index);
        let second_index = u32::from(e.mem.u16(current + 0xe));
        let second = node_narrow(e, manager, second_index);
        let first_centre = narrow_centre(e, narrow_bounds(e, first));
        let second_centre = narrow_centre(e, narrow_bounds(e, second));
        let first_offset = sub_ps(first_centre, leaf_centre);
        let second_offset = sub_ps(second_centre, leaf_centre);
        let d0 = mul_ps(first_offset, first_offset);
        let d1 = mul_ps(second_offset, second_offset);
        direction = nearer_child_sign(d0, d1);
        current_index = u32::from(e.mem.u16(current + 0xc + direction * 2));
        current = node_narrow(e, manager, current_index);
    }
    let parent_index = e.mem.u16(current + 0x10) & NONE_NARROW;
    let parent = if parent_index != NONE_NARROW {
        node_narrow(e, manager, u32::from(parent_index))
    } else {
        0
    };
    let current_words = narrow_bounds(e, current);
    let union = union_words(leaf_words, current_words);
    for (word, value) in union.into_iter().enumerate() {
        e.mem.set_u16(joint + 2 * word as u32, value);
    }
    e.mem.set_u16(joint + 0xc, current_index as u16);
    e.mem.set_u16(joint + 0xe, leaf);
    let flags = e.mem.u16(current + 0x10) & LEAF_NARROW;
    e.mem.set_u16(current + 0x10, flags | new_node);
    let flags = e.mem.u16(leaf_node + 0x10) & LEAF_NARROW;
    e.mem.set_u16(leaf_node + 0x10, flags | new_node);
    let old_next = e.mem.u16(current + 0x12);
    e.mem.set_u16(current + 0x12, leaf);
    e.mem.set_u16(leaf_node + 0x12, old_next);
    e.mem.set_u16(joint + 0x12, old_next);
    if parent == 0 {
        e.mem.set_u32(me + 0x10, joint);
        let flags = e.mem.u16(joint + 0x10);
        e.mem.set_u16(joint + 0x10, flags | NONE_NARROW);
        e.mem.set_u16(me + 0x14, new_node);
        return;
    }
    e.mem.set_u16(parent + 0xc + direction * 2, new_node);
    let flags = e.mem.u16(joint + 0x10) & LEAF_NARROW;
    e.mem.set_u16(joint + 0x10, flags | parent_index);
    if direction == 1 {
        let first_index = u32::from(e.mem.u16(parent + 0xc));
        let mut walker = node_narrow(e, manager, first_index);
        e.mem.set_u16(walker + 0x12, new_node);
        while e.mem.u16(walker + 0x10) & LEAF_NARROW == 0 {
            let next_index = u32::from(e.mem.u16(walker + 0xe));
            walker = node_narrow(e, manager, next_index);
            e.mem.set_u16(walker + 0x12, new_node);
        }
    }
    let mut ancestor = parent;
    loop {
        let ancestor_words = narrow_bounds(e, ancestor);
        // The upper words must not be below the leaf's, the lower words not above them.
        let expand = (0..3).any(|word| {
            ancestor_words[word] < leaf_words[word]
                || leaf_words[word + 3] < ancestor_words[word + 3]
        });
        if !expand {
            return;
        }
        let merged = union_words(ancestor_words, leaf_words);
        for (word, value) in merged.into_iter().enumerate() {
            e.mem.set_u16(ancestor + 2 * word as u32, value);
        }
        let up = e.mem.u16(ancestor + 0x10) & NONE_NARROW;
        if up == NONE_NARROW {
            return;
        }
        ancestor = node_narrow(e, manager, u32::from(up));
        if ancestor == 0 {
            return;
        }
    }
}

// Translated from 00d9bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the leaf `index` (low word) out of the 16-bit tree (thiscall, `RET 8`): the counterpart
/// of `00d9b240`; the freed joint node's index is stored as a word at `parent_out`. The
/// refitting of ancestors compares all six bounds words and stops at the first ancestor whose
/// bounds do not change (the answer is then that ancestor, else the root pointer read
/// before). As in the wide version the exe reads through the parent pointer even for a root
/// leaf, which faults. Returns the pointer described above.
pub fn fn_00d9bb50(e: &mut Engine, this: Ptr, index: u32, parent_out: Ptr) -> u32 {
    let me = this.addr();
    let manager = me + 0x1c;
    let index16 = index as u16;
    let node = node_narrow(e, manager, u32::from(index16));
    let parent_index = e.mem.u16(node + 0x10) & NONE_NARROW;
    let parent = if parent_index != NONE_NARROW {
        node_narrow(e, manager, u32::from(parent_index))
    } else {
        0
    };
    let grand_index = e.mem.u16(parent + 0x10) & NONE_NARROW;
    let grand = if grand_index != NONE_NARROW {
        node_narrow(e, manager, u32::from(grand_index))
    } else {
        0
    };
    let second_index = e.mem.u16(parent + 0xe);
    let first_index = e.mem.u16(parent + 0xc);
    let first = node_narrow(e, manager, u32::from(first_index));
    let second = node_narrow(e, manager, u32::from(second_index));
    let sibling_index = first_index ^ second_index ^ index16;
    let sibling = first ^ second ^ node;
    e.mem.set_u16(parent_out.addr(), parent_index);
    let answer = if grand == 0 {
        e.mem.set_u32(me + 0x10, sibling);
        let flags = e.mem.u16(sibling + 0x10);
        e.mem.set_u16(sibling + 0x10, flags | NONE_NARROW);
        let root = e.mem.u32(me + 0x10);
        e.mem.set_u16(me + 0x14, sibling_index);
        root
    } else {
        let right_index = u32::from(e.mem.u16(grand + 0xe));
        let right = node_narrow(e, manager, right_index);
        let slot = u32::from(right == parent);
        e.mem.set_u16(grand + 0xc + slot * 2, sibling_index);
        let flags = e.mem.u16(sibling + 0x10) & LEAF_NARROW;
        e.mem.set_u16(sibling + 0x10, flags | grand_index);
        let mut found = e.mem.u32(me + 0x10);
        if slot == 1 {
            let left_index = u32::from(e.mem.u16(grand + 0xc));
            let mut walker = node_narrow(e, manager, left_index);
            e.mem.set_u16(walker + 0x12, sibling_index);
            while e.mem.u16(walker + 0x10) & LEAF_NARROW == 0 {
                let next_index = u32::from(e.mem.u16(walker + 0xe));
                walker = node_narrow(e, manager, next_index);
                e.mem.set_u16(walker + 0x12, sibling_index);
            }
        }
        let mut ancestor = grand;
        loop {
            let second_index = u32::from(e.mem.u16(ancestor + 0xe));
            let second = node_narrow(e, manager, second_index);
            let first_index = u32::from(e.mem.u16(ancestor + 0xc));
            let first = node_narrow(e, manager, first_index);
            let first_words = narrow_bounds(e, first);
            let second_words = narrow_bounds(e, second);
            let fitted = union_words(first_words, second_words);
            if narrow_bounds(e, ancestor) == fitted {
                found = ancestor;
                break;
            }
            for (word, value) in fitted.into_iter().enumerate() {
                e.mem.set_u16(ancestor + 2 * word as u32, value);
            }
            let up = e.mem.u16(ancestor + 0x10) & NONE_NARROW;
            if up == NONE_NARROW {
                break;
            }
            ancestor = node_narrow(e, manager, u32::from(up));
            if ancestor == 0 {
                break;
            }
        }
        found
    };
    let left_index = u32::from(e.mem.u16(parent + 0xc));
    let left = node_narrow(e, manager, left_index);
    if left == sibling {
        let next = e.mem.u16(parent + 0x12);
        e.mem.set_u16(sibling + 0x12, next);
        let mut walker = sibling;
        while e.mem.u16(walker + 0x10) & LEAF_NARROW == 0 {
            let next_index = u32::from(e.mem.u16(walker + 0xe));
            walker = node_narrow(e, manager, next_index);
            e.mem.set_u16(walker + 0x12, next);
        }
    }
    answer
}

// Translated from 00d9c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the leaf `index` from the 16-bit tree (thiscall, `RET 4`): when the root is not
/// itself the leaf the leaf is taken out (`00d9bb50`) and both the freed joint node and the
/// leaf node go back on the free list (`00d9c2d0`); a root leaf just empties the tree (root
/// word `0x7fff`, root pointer 0) and frees the leaf. Counts one leaf less (`this + 0x18`).
pub fn fn_00d9c7d0(e: &mut Engine, this: Ptr, index: u32) {
    let me = this.addr();
    let manager = Ptr::new(me + 0x1c);
    let root = e.mem.u32(me + 0x10);
    if e.mem.u16(root + 0x10) & LEAF_NARROW == 0 {
        let joint = e.with_stack(4, |e, slot| {
            fn_00d9bb50(e, this, index, slot);
            e.mem.u16(slot.addr())
        });
        fn_00d9c2d0(e, manager, joint);
        fn_00d9c2d0(e, manager, index as u16);
    } else {
        e.mem.set_u16(me + 0x14, NONE_NARROW);
        e.mem.set_u32(me + 0x10, 0);
        fn_00d9c2d0(e, manager, index as u16);
    }
    let count = e.mem.u32(me + 0x18);
    e.mem.set_u32(me + 0x18, count.wrapping_sub(1));
}

/// One lane of the quantisation `00d9c610` and `00d9ce50` apply to bounds: the clamped scaled
/// value rounded with `FISTP` (nearest, ties to even) and shifted right by 15.
fn quantise(lane: f32) -> u16 {
    (crate::x87::fistp(f64::from(lane)) as u32 >> 15) as u16
}

/// The six bounds words of a bounds pair (min at `min`, max at `max`) in the tree's grid:
/// `(offset + bound) * scale`, clamped between the zero vector and the limit (`MINPS` then
/// `MAXPS`) and quantised. Words 0..2 come from the max vector, 3..5 from the min vector.
fn grid_words(e: &Engine, tree: u32, min: Vec4, max: Vec4) -> [u16; 6] {
    let offset_min = load_vec(e, tree + 0x40);
    let offset_max = load_vec(e, tree + 0x50);
    let scale = load_vec(e, tree + 0x60);
    let limit = load_vec(e, CLAMP_LIMIT);
    let floor = load_vec(e, ZERO_VECTOR);
    let low = max_ps(min_ps(mul_ps(add_ps(offset_min, min), scale), limit), floor);
    let high = max_ps(min_ps(mul_ps(add_ps(max, offset_max), scale), limit), floor);
    [
        quantise(high[0]),
        quantise(high[1]),
        quantise(high[2]),
        quantise(low[0]),
        quantise(low[1]),
        quantise(low[2]),
    ]
}

// Translated from 00d9c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::add` (Xbox PDB), thiscall,
/// `RET 8`: quantises the 0x20-byte bounds at `bounds` into the tree's 16-bit grid (offsets at
/// +0x40 / +0x50, scale at +0x60), builds a leaf node (bounds words, `data` at +0xC, leaf bit
/// 15) and pushes it (`00d9c210`). The first leaf becomes the root (parent word `0x7fff`, next
/// link `0x7fff`); later ones are joined with a fresh node through `00d9b580`. Counts the
/// leaf (`this + 0x18`) and returns its index. The node's word at +0x12 starts as a stack
/// leftover in the exe (zero here); both paths overwrite it.
pub fn hk_aabb_tree_aabb16_add(e: &mut Engine, this: Ptr, bounds: Ptr, data: u32) -> u16 {
    let me = this.addr();
    let manager = me + 0x1c;
    let min = load_vec(e, bounds.addr());
    let max = load_vec(e, bounds.addr() + 0x10);
    let words = grid_words(e, me, min, max);
    let index = e.with_stack(0x14, |e, block| {
        let at = block.addr();
        for (word, value) in words.into_iter().enumerate() {
            e.mem.set_u16(at + 2 * word as u32, value);
        }
        e.mem.set_u32(at + 0xc, data);
        e.mem.set_u16(at + 0x10, LEAF_NARROW);
        hk_aabb_tree_aabb16_node_manager_push_back(
            e,
            Ptr::new(manager),
            Ptr::new(me + 0x10),
            Ptr::new(me + 0x14),
            block,
        ) as u16
    });
    if e.mem.u16(me + 0x14) == NONE_NARROW {
        let node = node_narrow(e, manager, u32::from(index));
        let flags = e.mem.u16(node + 0x10);
        e.mem.set_u16(node + 0x10, flags | NONE_NARROW);
        e.mem.set_u16(node + 0x12, NONE_NARROW);
        let count = e.mem.u32(me + 0x18);
        e.mem.set_u32(me + 0x18, count.wrapping_add(1));
        e.mem.set_u32(me + 0x10, node);
        e.mem.set_u16(me + 0x14, index);
        return index;
    }
    let joint = e.with_stack(4, |e, slot| {
        hk_aabb_tree_aabb16_node_manager_new_node(
            e,
            Ptr::new(manager),
            Ptr::new(me + 0x10),
            Ptr::new(me + 0x14),
            slot,
        );
        e.mem.u16(slot.addr())
    });
    let root = u32::from(e.mem.u16(me + 0x14));
    hk_aabb_tree_aabb16_add_leaf_internal(e, this, root, u32::from(index), u32::from(joint));
    let count = e.mem.u32(me + 0x18);
    e.mem.set_u32(me + 0x18, count.wrapping_add(1));
    index
}

// Translated from 00d9c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the wide tree from `node` (thiscall on the tree, `RET 8`; the second stack word is
/// unused) with a stack of next-leaf pointers kept in a 128-entry inline array that grows
/// through `00c90940`: for an inner node it goes to its first child and pushes the node of that
/// child's link at +0x2C (0 when the link is `0x7fffffff`); at a leaf it pops the stack. It
/// changes nothing but that stack. A null node does nothing. The exe pops a pushed 0 and then
/// reads through it, which faults; this reads the same address.
pub fn fn_00d9c360(e: &mut Engine, this: Ptr, node: u32, _unused_1: u32) {
    if node == 0 {
        return;
    }
    let manager = this.addr() + 0x1c;
    e.with_stack(0x210, |e, frame| {
        let header = frame.addr();
        e.mem.set_u32(header, header + 0x10);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0080);
        let mut current = node;
        loop {
            while e.mem.u32(current + 0x28) & LEAF_WIDE == 0 {
                let first_index = e.mem.u32(current + 0x20);
                current = node_wide(e, manager, first_index);
                let link = e.mem.u32(current + 0x2c);
                let next = if link == NONE_WIDE {
                    0
                } else {
                    node_wide(e, manager, link)
                };
                let alloc = allocator(e);
                if e.mem.u32(header + 4) == e.mem.u32(header + 8) & CAPACITY_MASK {
                    e.call(ARRAY_RESERVE_MORE, &args![alloc, header, 4u32]);
                }
                let data = e.mem.u32(header);
                let size = e.mem.u32(header + 4);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), next);
                e.mem.set_u32(header + 4, size.wrapping_add(1));
            }
            let size = e.mem.u32(header + 4);
            if size == 0 {
                break;
            }
            let data = e.mem.u32(header);
            current = e
                .mem
                .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
            e.mem.set_u32(header + 4, size - 1);
        }
        release_array(e, header, 4);
    });
}

// Translated from 00d9c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same walk over the 16-bit tree (thiscall on the tree, `RET 8`, second stack word
/// unused): leaf bit 15 of the word at +0x10, first child word at +0xC, link word at +0x12 (0
/// when `0x7fff`).
pub fn fn_00d9c520(e: &mut Engine, this: Ptr, node: u32, _unused_1: u32) {
    if node == 0 {
        return;
    }
    let manager = this.addr() + 0x1c;
    e.with_stack(0x210, |e, frame| {
        let header = frame.addr();
        e.mem.set_u32(header, header + 0x10);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0080);
        let mut current = node;
        loop {
            while e.mem.u16(current + 0x10) & LEAF_NARROW == 0 {
                let first_index = u32::from(e.mem.u16(current + 0xc));
                current = node_narrow(e, manager, first_index);
                let link = e.mem.u16(current + 0x12);
                let next = if link == NONE_NARROW {
                    0
                } else {
                    node_narrow(e, manager, u32::from(link))
                };
                let alloc = allocator(e);
                if e.mem.u32(header + 4) == e.mem.u32(header + 8) & CAPACITY_MASK {
                    e.call(ARRAY_RESERVE_MORE, &args![alloc, header, 4u32]);
                }
                let data = e.mem.u32(header);
                let size = e.mem.u32(header + 4);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), next);
                e.mem.set_u32(header + 4, size.wrapping_add(1));
            }
            let size = e.mem.u32(header + 4);
            if size == 0 {
                break;
            }
            let data = e.mem.u32(header);
            current = e
                .mem
                .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
            e.mem.set_u32(header + 4, size - 1);
        }
        release_array(e, header, 4);
    });
}

/// Which primitive layout a partition or build routine works on.
#[derive(Clone, Copy)]
enum Primitives {
    /// 0x20-byte float bounds, dword indices, 0x30-byte tree nodes.
    Wide,
    /// 0xC-byte word bounds, word indices, 0x14-byte tree nodes.
    Narrow,
}

impl Primitives {
    fn index_size(self) -> u32 {
        match self {
            Primitives::Wide => 4,
            Primitives::Narrow => 2,
        }
    }

    fn read_index(self, e: &Engine, base: u32, position: u32) -> u32 {
        let at = base.wrapping_add(position.wrapping_mul(self.index_size()));
        match self {
            Primitives::Wide => e.mem.u32(at),
            Primitives::Narrow => u32::from(e.mem.u16(at)),
        }
    }

    fn write_index(self, e: &mut Engine, base: u32, position: u32, value: u32) {
        let at = base.wrapping_add(position.wrapping_mul(self.index_size()));
        match self {
            Primitives::Wide => e.mem.set_u32(at, value),
            Primitives::Narrow => e.mem.set_u16(at, value as u16),
        }
    }

    /// The node of `index` through the node manager's accessor.
    fn node_at(self, e: &mut Engine, tree: u32, index: u32) -> u32 {
        match self {
            Primitives::Wide => node_wide(e, tree + 0x1c, index),
            Primitives::Narrow => node_narrow(e, tree + 0x1c, index),
        }
    }

    /// Takes a free node from the node manager (`newNode`); the index lands in `slot`.
    /// Returns the node address and its index.
    fn new_node(self, e: &mut Engine, tree: u32, slot: u32) -> (u32, u32) {
        let manager = Ptr::new(tree + 0x1c);
        let root_out = Ptr::new(tree + 0x10);
        let root_index = Ptr::new(tree + 0x14);
        match self {
            Primitives::Wide => {
                let node = fn_00d9ce00(e, manager, root_out, root_index, Ptr::new(slot));
                (node, e.mem.u32(slot))
            }
            Primitives::Narrow => {
                let node = hk_aabb_tree_aabb16_node_manager_new_node(
                    e,
                    manager,
                    root_out,
                    root_index,
                    Ptr::new(slot),
                );
                (node, u32::from(e.mem.u16(slot)))
            }
        }
    }

    /// Stores child number `which` of an inner node.
    fn set_child(self, e: &mut Engine, node: u32, which: u32, index: u32) {
        match self {
            Primitives::Wide => e.mem.set_u32(node + 0x20 + 4 * which, index),
            Primitives::Narrow => e.mem.set_u16(node + 0xc + 2 * which, index as u16),
        }
    }

    /// The first-primitive field of a work item (+0x20): a dword, or a word for narrow trees.
    fn item_start(self, e: &Engine, item: u32) -> u32 {
        match self {
            Primitives::Wide => e.mem.u32(item + 0x20),
            Primitives::Narrow => u32::from(e.mem.u16(item + 0x20)),
        }
    }

    fn set_item_start(self, e: &mut Engine, item: u32, start: u32) {
        match self {
            Primitives::Wide => e.mem.set_u32(item + 0x20, start),
            Primitives::Narrow => e.mem.set_u16(item + 0x20, start as u16),
        }
    }
}

/// The centre of primitive `index`: `0.5 * (max + min)` for float bounds, the same from the
/// word bounds converted to floats for the narrow layout.
fn primitive_centroid(e: &Engine, kind: Primitives, aabbs: u32, index: u32) -> Vec4 {
    match kind {
        Primitives::Wide => {
            let at = aabbs.wrapping_add(index << 5);
            let half = e.mem.f32(FLOAT_HALF);
            let sum = add_ps(load_vec(e, at + 0x10), load_vec(e, at));
            mul_ps([half; 4], sum)
        }
        Primitives::Narrow => {
            let at = aabbs.wrapping_add(index.wrapping_mul(0xc));
            narrow_centre(e, narrow_bounds(e, at))
        }
    }
}

/// Widens a (min, max) pair of the centroid boxes to include `centroid` (`MINPS`/`MAXPS`).
fn grow(bounds: &mut (Vec4, Vec4), centroid: Vec4) {
    bounds.0 = min_ps(bounds.0, centroid);
    bounds.1 = max_ps(bounds.1, centroid);
}

/// The shared body of the two partition routines `00d9a520` and `00d9a850`: splits the
/// primitives `start .. start + count` of the index array `*indices` in two groups about the
/// middle of the longest axis of the centroid box `bounds` (a Hoare partition on centroids)
/// and writes the centroid box of each group (`out1`, `out2`) and the group sizes. When the
/// box is too thin on that axis (the distance from the middle to either side is not above the
/// tolerance at `010d409c`) both outputs are the whole box and the halves are cut by count.
/// The x87 centre / gap arithmetic is done in double precision rounded to float at the same
/// points (`FSTP`), without modelling the extended-precision double rounding.
fn partition_primitives(
    e: &mut Engine,
    kind: Primitives,
    bounds: u32,
    aabbs: u32,
    indices: u32,
    start: i32,
    count: i32,
    outputs: (u32, u32),
    counts: (u32, u32),
) {
    let (out1, out2) = outputs;
    let (out_count1, out_count2) = counts;
    let start = match kind {
        Primitives::Wide => start,
        Primitives::Narrow => i32::from(start as u16),
    };
    let low = [e.mem.f32(BOUNDS_START_MIN); 4];
    let high = [e.mem.f32(BOUNDS_START_MAX); 4];
    for out in [out1, out2] {
        store_vec(e, out, low);
        store_vec(e, out + 0x10, high);
    }
    let box_min = load_vec(e, bounds);
    let box_max = load_vec(e, bounds + 0x10);
    let sums = add_ps(box_min, box_max);
    let span = sub_ps(box_max, box_min);
    let mask: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(ABS_MASK + 4 * lane as u32));
    let extent: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(span[lane].to_bits() & mask[lane]));
    let axis = if extent[1] > extent[0] {
        if extent[2] > extent[1] {
            2
        } else {
            1
        }
    } else if extent[2] > extent[0] {
        2
    } else {
        0
    };
    let centre = (f64::from(sums[axis]) * e.mem.f64(DOUBLE_HALF)) as f32;
    let tolerance = e.mem.f32(SPLIT_TOLERANCE);
    let upper_gap = (f64::from(box_max[axis]) - f64::from(centre)) as f32;
    let lower_gap = (f64::from(centre) - f64::from(box_min[axis])) as f32;
    let splits = tolerance < upper_gap && tolerance < lower_gap;
    if !splits {
        store_vec(e, out1, box_min);
        store_vec(e, out1 + 0x10, box_max);
        store_vec(e, out2, box_min);
        store_vec(e, out2 + 0x10, box_max);
        let first = count >> 1;
        e.mem.set_u32(out_count1, first as u32);
        e.mem.set_u32(out_count2, count.wrapping_sub(first) as u32);
        return;
    }
    let mut first_box = (low, high);
    let mut second_box = (low, high);
    let mut left = start;
    let mut right = start.wrapping_add(count).wrapping_sub(1);
    loop {
        // Left scan: primitives whose centre is not above the middle stay in the first group
        // (a NaN centre stops the scan).
        loop {
            let base = e.mem.u32(indices);
            let index = kind.read_index(e, base, left as u32);
            let centroid = primitive_centroid(e, kind, aabbs, index);
            let stays = matches!(
                centroid[axis].partial_cmp(&centre),
                Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
            );
            if !stays {
                break;
            }
            grow(&mut first_box, centroid);
            left = left.wrapping_add(1);
        }
        // Right scan: primitives whose centre is above the middle stay in the second group.
        loop {
            let base = e.mem.u32(indices);
            let index = kind.read_index(e, base, right as u32);
            let centroid = primitive_centroid(e, kind, aabbs, index);
            let stays = matches!(
                centroid[axis].partial_cmp(&centre),
                Some(std::cmp::Ordering::Greater)
            );
            if !stays {
                break;
            }
            grow(&mut second_box, centroid);
            right = right.wrapping_sub(1);
        }
        if left >= right {
            break;
        }
        let base = e.mem.u32(indices);
        let left_index = kind.read_index(e, base, left as u32);
        let right_index = kind.read_index(e, base, right as u32);
        kind.write_index(e, base, left as u32, right_index);
        kind.write_index(e, base, right as u32, left_index);
        let base = e.mem.u32(indices);
        let now_left = kind.read_index(e, base, left as u32);
        let now_right = kind.read_index(e, base, right as u32);
        let left_centroid = primitive_centroid(e, kind, aabbs, now_left);
        let right_centroid = primitive_centroid(e, kind, aabbs, now_right);
        grow(&mut first_box, left_centroid);
        grow(&mut second_box, right_centroid);
        right = right.wrapping_sub(1);
        left = left.wrapping_add(1);
        if left > right {
            break;
        }
    }
    store_vec(e, out1, first_box.0);
    store_vec(e, out1 + 0x10, first_box.1);
    store_vec(e, out2, second_box.0);
    store_vec(e, out2 + 0x10, second_box.1);
    let first = left.wrapping_sub(start);
    e.mem.set_u32(out_count1, first as u32);
    e.mem.set_u32(out_count2, count.wrapping_sub(first) as u32);
}

// Translated from 00d9a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits wide primitives (cdecl, nine stack arguments, caller cleans up): `bounds` is the
/// 0x20-byte centroid box of the primitives `start .. start + count` of the dword index array
/// whose data pointer is at `*indices`, `aabbs` the array of 0x20-byte primitive boxes. Writes
/// the centroid boxes of the two groups to `out1` / `out2` and their sizes to `out_count1` /
/// `out_count2`; see `partition_primitives`.
pub fn fn_00d9a520(
    e: &mut Engine,
    bounds: Ptr,
    aabbs: Ptr,
    indices: Ptr,
    start: i32,
    count: i32,
    out1: Ptr,
    out2: Ptr,
    out_count1: Ptr,
    out_count2: Ptr,
) {
    partition_primitives(
        e,
        Primitives::Wide,
        bounds.addr(),
        aabbs.addr(),
        indices.addr(),
        start,
        count,
        (out1.addr(), out2.addr()),
        (out_count1.addr(), out_count2.addr()),
    );
}

// Translated from 00d9a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The partition of the 16-bit tree builder (cdecl, nine stack arguments): primitives are
/// 0xC-byte word boxes, the index array holds words and only the low word of `start` is used.
pub fn fn_00d9a850(
    e: &mut Engine,
    bounds: Ptr,
    aabbs: Ptr,
    indices: Ptr,
    start: u32,
    count: i32,
    out1: Ptr,
    out2: Ptr,
    out_count1: Ptr,
    out_count2: Ptr,
) {
    partition_primitives(
        e,
        Primitives::Narrow,
        bounds.addr(),
        aabbs.addr(),
        indices.addr(),
        start as i32,
        count,
        (out1.addr(), out2.addr()),
        (out_count1.addr(), out_count2.addr()),
    );
}

/// Offsets inside the scratch block of `build_tree`: the primitive-index array header, the
/// work-item array header, the two group sizes, the new-node index slot, the work item being
/// processed, the two group boxes, the two child items and the inline storage of the work
/// array (64 items of 0x30 bytes).
const INDEX_HEADER: u32 = 0x00;
const WORK_HEADER: u32 = 0x10;
const GROUP_COUNTS: u32 = 0x20;
const NODE_INDEX_SLOT: u32 = 0x30;
const CURRENT_ITEM: u32 = 0x40;
const GROUP_BOXES: u32 = 0x70;
const CHILD_ITEMS: u32 = 0xb0;
const WORK_INLINE: u32 = 0x110;
const BUILD_FRAME: u32 = WORK_INLINE + 0x40 * 0x30;

/// Gives primitive `leaf` a leaf node: a fresh one from the node manager, or (`reuse`) the
/// node whose index the output index array already holds for it. Records the index in the
/// output array, copies the primitive's box and user data into the node, sets the leaf bit and
/// returns the node's index.
fn place_leaf(
    e: &mut Engine,
    kind: Primitives,
    tree: u32,
    sources: (u32, u32),
    out_indices: u32,
    reuse: bool,
    slot: u32,
    leaf: u32,
) -> u32 {
    let (aabbs, userdata) = sources;
    let (node, index) = if reuse {
        let data = e.mem.u32(out_indices);
        let index = kind.read_index(e, data, leaf);
        (kind.node_at(e, tree, index), index)
    } else {
        kind.new_node(e, tree, slot)
    };
    let data = e.mem.u32(out_indices);
    kind.write_index(e, data, leaf, index);
    let data_word = e.mem.u32(userdata.wrapping_add(leaf.wrapping_mul(4)));
    match kind {
        Primitives::Wide => {
            let from = aabbs.wrapping_add(leaf << 5);
            copy_words(e, from, node, 8);
            let flags = e.mem.u32(node + 0x28);
            e.mem.set_u32(node + 0x28, flags | LEAF_WIDE);
            e.mem.set_u32(node + 0x20, data_word);
        }
        Primitives::Narrow => {
            let from = aabbs.wrapping_add(leaf.wrapping_mul(0xc));
            copy_words(e, from, node, 3);
            e.mem.set_u32(node + 0xc, data_word);
            let flags = e.mem.u16(node + 0x10);
            e.mem.set_u16(node + 0x10, flags | LEAF_NARROW);
        }
    }
    index
}

/// `REP MOVSD`: copies `words` dwords.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

/// The shared body of the two tree builders `00d9c840` and `00d9d1c0`: gives every one of the
/// `count` primitives a leaf under `root_node`. Builds the index array 0..count, takes the box
/// of all centroids as the first work item and then repeatedly pops an item: a group of at
/// most two primitives becomes two leaves directly; a larger one is split by the partition
/// routine and each half becomes a leaf (one primitive), an inner node with a new work item
/// (more than one) or, for an empty half, whatever primitive sits at its position. The new
/// items are pushed second half first. `out_indices` points at the header of the array that
/// receives each primitive's node index (and, with `reuse`, supplies it).
fn build_tree(
    e: &mut Engine,
    kind: Primitives,
    tree: u32,
    sources: (u32, u32),
    count: i32,
    root_node: u32,
    out_indices: u32,
    reuse: bool,
) {
    let (aabbs, _) = sources;
    e.with_stack(BUILD_FRAME, |e, frame| {
        let f = frame.addr();
        let index_header = f + INDEX_HEADER;
        let work_header = f + WORK_HEADER;
        e.mem.set_u32(index_header, 0);
        e.mem.set_u32(index_header + 4, 0);
        e.mem.set_u32(index_header + 8, 0x8000_0000);
        reserve_at_least(e, index_header, count, kind.index_size());
        e.mem.set_u32(index_header + 4, count as u32);
        if count > 0 {
            for position in 0..count as u32 {
                let data = e.mem.u32(index_header);
                kind.write_index(e, data, position, position);
            }
        }
        let mut centre_box = (
            [e.mem.f32(BOUNDS_START_MIN); 4],
            [e.mem.f32(BOUNDS_START_MAX); 4],
        );
        if count > 0 {
            for position in 0..count as u32 {
                let centroid = primitive_centroid(e, kind, aabbs, position);
                grow(&mut centre_box, centroid);
            }
        }
        e.mem.set_u32(work_header, f + WORK_INLINE);
        e.mem.set_u32(work_header + 8, 0x8000_0040);
        let first_item = f + WORK_INLINE;
        store_vec(e, first_item, centre_box.0);
        store_vec(e, first_item + 0x10, centre_box.1);
        kind.set_item_start(e, first_item, 0);
        e.mem.set_u32(first_item + 0x24, count as u32);
        e.mem.set_u32(first_item + 0x28, root_node);
        e.mem.set_u32(work_header + 4, 1);
        let slot = f + NODE_INDEX_SLOT;
        let item = f + CURRENT_ITEM;
        let counts = f + GROUP_COUNTS;
        loop {
            let size = e.mem.u32(work_header + 4);
            if size == 0 {
                break;
            }
            let data = e.mem.u32(work_header);
            let top = data
                .wrapping_add(size.wrapping_mul(0x30))
                .wrapping_sub(0x30);
            copy_words(e, top, item, 12);
            e.mem.set_u32(work_header + 4, size.wrapping_sub(1));
            let item_count = e.mem.i32(item + 0x24);
            let node = e.mem.u32(item + 0x28);
            let start = kind.item_start(e, item);
            if item_count <= 2 {
                // Exactly two leaves are placed, whatever the count (the builders never queue
                // a smaller group).
                for which in 0..2u32 {
                    let data = e.mem.u32(index_header);
                    let leaf = kind.read_index(e, data, start.wrapping_add(which));
                    let index = place_leaf(e, kind, tree, sources, out_indices, reuse, slot, leaf);
                    kind.set_child(e, node, which, index);
                }
            } else {
                let out1 = f + GROUP_BOXES;
                let out2 = out1 + 0x20;
                partition_primitives(
                    e,
                    kind,
                    item,
                    aabbs,
                    index_header,
                    start as i32,
                    item_count,
                    (out1, out2),
                    (counts, counts + 4),
                );
                let first_count = e.mem.i32(counts);
                let mut running = 0u32;
                for which in 0..2u32 {
                    let group_count = e.mem.i32(counts + 4 * which);
                    let child = f + CHILD_ITEMS + 0x30 * which;
                    let index = if group_count > 1 {
                        let (child_node, child_index) = kind.new_node(e, tree, slot);
                        copy_words(e, out1 + 0x20 * which, child, 8);
                        let child_start = (which as i32)
                            .wrapping_mul(first_count)
                            .wrapping_add(start as i32);
                        kind.set_item_start(e, child, child_start as u32);
                        e.mem.set_u32(child + 0x24, group_count as u32);
                        e.mem.set_u32(child + 0x28, child_node);
                        child_index
                    } else {
                        let data = e.mem.u32(index_header);
                        let position = start.wrapping_add(running);
                        let leaf = kind.read_index(e, data, position);
                        place_leaf(e, kind, tree, sources, out_indices, reuse, slot, leaf)
                    };
                    kind.set_child(e, node, which, index);
                    running = running.wrapping_add(first_count as u32);
                }
                for which in (0..2u32).rev() {
                    if e.mem.i32(counts + 4 * which) > 1 {
                        let alloc = allocator(e);
                        let size = e.mem.u32(work_header + 4);
                        if size == e.mem.u32(work_header + 8) & CAPACITY_MASK {
                            e.call(ARRAY_RESERVE_MORE, &args![alloc, work_header, 0x30u32]);
                        }
                        let size = e.mem.u32(work_header + 4);
                        let data = e.mem.u32(work_header);
                        let target = data.wrapping_add(size.wrapping_mul(0x30));
                        // The exe skips the copy only for a null target (never, in practice).
                        if target != 0 {
                            copy_words(e, f + CHILD_ITEMS + 0x30 * which, target, 12);
                        }
                        e.mem.set_u32(work_header + 4, size.wrapping_add(1));
                    }
                }
            }
        }
        release_array(e, work_header, 0x30);
        release_array(e, index_header, kind.index_size());
    });
}

// Translated from 00d9c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the 16-bit tree's leaves for `count` primitives (thiscall, `RET 0x18`): `aabbs` are
/// 0xC-byte word boxes, `userdata` one dword per primitive, `root_node` the node that becomes
/// the parent of the first split, `out_indices` the header of the word array that receives the
/// node index of each primitive, and `reuse` (the byte at +0x1C) makes it reuse the nodes the
/// array already names instead of taking new ones. See `build_tree`.
pub fn fn_00d9c840(
    e: &mut Engine,
    this: Ptr,
    aabbs: Ptr,
    userdata: Ptr,
    count: i32,
    root_node: u32,
    out_indices: Ptr,
    reuse: bool,
) {
    build_tree(
        e,
        Primitives::Narrow,
        this.addr(),
        (aabbs.addr(), userdata.addr()),
        count,
        root_node,
        out_indices.addr(),
        reuse,
    );
}

// Translated from 00d9d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for the wide tree (thiscall, `RET 0x18`): 0x20-byte float boxes, dword node
/// indices in `out_indices`, 0x30-byte nodes.
pub fn fn_00d9d1c0(
    e: &mut Engine,
    this: Ptr,
    aabbs: Ptr,
    userdata: Ptr,
    count: i32,
    root_node: u32,
    out_indices: Ptr,
    reuse: bool,
) {
    build_tree(
        e,
        Primitives::Wide,
        this.addr(),
        (aabbs.addr(), userdata.addr()),
        count,
        root_node,
        out_indices.addr(),
        reuse,
    );
}

// Translated from 00d9ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb16_unsigned_short>::build` (Xbox PDB): builds the whole tree for `count` primitives (thiscall, `RET 0x10`): `aabbs` are
/// 0x20-byte float boxes (quantised into the tree's grid), `userdata` one dword each and
/// `out_indices` the header of the word array that receives each leaf's node index. Stores
/// the count (`this + 0x18`; zero does nothing else), sizes the output array, reserves
/// `2 * count - 1` nodes, takes the root node (flag word `0x7fff`), converts the boxes into a
/// temporary array of 0xC-byte word boxes and builds (`00d9c840`; one primitive is filled in
/// directly), then fixes the root up (`00d9b450`), walks the tree (`00d9c520`) and frees the
/// temporary array.
pub fn hk_aabb_tree_aabb16_build(
    e: &mut Engine,
    this: Ptr,
    aabbs: Ptr,
    userdata: Ptr,
    count: i32,
    out_indices: Ptr,
) {
    let me = this.addr();
    e.mem.set_u32(me + 0x18, count as u32);
    if count == 0 {
        return;
    }
    let out = out_indices.addr();
    reserve_at_least(e, out, count, 2);
    e.mem.set_u32(out + 4, count as u32);
    let manager = Ptr::new(me + 0x1c);
    let nodes = count.wrapping_mul(2).wrapping_sub(1);
    hk_aabb_tree_aabb16_node_manager_reserve(
        e,
        manager,
        nodes as u32,
        Ptr::new(me + 0x10),
        Ptr::new(me + 0x14),
    );
    let root = hk_aabb_tree_aabb16_node_manager_new_node(
        e,
        manager,
        Ptr::new(me + 0x10),
        Ptr::new(me + 0x14),
        Ptr::new(me + 0x14),
    );
    e.mem.set_u32(me + 0x10, root);
    let flags = e.mem.u16(root + 0x10);
    e.mem.set_u16(root + 0x10, flags | NONE_NARROW);
    e.with_stack(0x10, |e, header| {
        let header = header.addr();
        e.mem.set_u32(header, 0);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0000);
        reserve_at_least(e, header, count, 0xc);
        e.mem.set_u32(header + 4, count as u32);
        if count > 0 {
            for position in 0..count as u32 {
                let from = aabbs.addr().wrapping_add(position << 5);
                let min = load_vec(e, from);
                let max = load_vec(e, from + 0x10);
                let words = grid_words(e, me, min, max);
                let data = e.mem.u32(header);
                let to = data.wrapping_add(position.wrapping_mul(0xc));
                for (word, value) in words.into_iter().enumerate() {
                    e.mem.set_u16(to + 2 * word as u32, value);
                }
            }
        }
        let data = e.mem.u32(header);
        if count == 1 {
            let node = e.mem.u32(me + 0x10);
            let first = e.mem.u32(userdata.addr());
            e.mem.set_u32(node + 0xc, first);
            let flags = e.mem.u16(node + 0x10);
            e.mem.set_u16(node + 0x10, flags | LEAF_NARROW);
            let node = e.mem.u32(me + 0x10);
            e.mem.set_u16(node + 0x12, NONE_NARROW);
            copy_words(e, data, node, 3);
            let out_data = e.mem.u32(out);
            let root_index = e.mem.u16(me + 0x14);
            e.mem.set_u16(out_data, root_index);
        } else {
            let node = e.mem.u32(me + 0x10);
            fn_00d9c840(
                e,
                this,
                Ptr::new(data),
                userdata,
                count,
                node,
                out_indices,
                false,
            );
        }
        let root_index = u32::from(e.mem.u16(me + 0x14));
        let node = e.mem.u32(me + 0x10);
        e.call(
            FIX_UP_NARROW,
            &args![this, node, root_index, u32::from(NONE_NARROW)],
        );
        let node = e.mem.u32(me + 0x10);
        fn_00d9c520(e, this, node, 0);
        release_array(e, header, 0xc);
    });
}

// Translated from 00d9d650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTree<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::build` (Xbox PDB): builds the whole wide tree for `count` primitives (thiscall, `RET 0x10`): the same steps as
/// `00d9ce50` with the boxes copied unchanged into a temporary array of 0x20-byte entries,
/// dword node indices, the root flag `0x7fffffff` and the builder `00d9d1c0`, the fix-up
/// `00d9af30` and the walk `00d9c360`.
pub fn hk_aabb_tree_aabb_uint_build(
    e: &mut Engine,
    this: Ptr,
    aabbs: Ptr,
    userdata: Ptr,
    count: i32,
    out_indices: Ptr,
) {
    let me = this.addr();
    e.mem.set_u32(me + 0x18, count as u32);
    if count == 0 {
        return;
    }
    let out = out_indices.addr();
    reserve_at_least(e, out, count, 4);
    e.mem.set_u32(out + 4, count as u32);
    let manager = Ptr::new(me + 0x1c);
    let nodes = count.wrapping_mul(2).wrapping_sub(1);
    fn_00d9c450(e, manager, nodes, Ptr::new(me + 0x10), Ptr::new(me + 0x14));
    let root = fn_00d9ce00(
        e,
        manager,
        Ptr::new(me + 0x10),
        Ptr::new(me + 0x14),
        Ptr::new(me + 0x14),
    );
    e.mem.set_u32(me + 0x10, root);
    let flags = e.mem.u32(root + 0x28);
    e.mem.set_u32(root + 0x28, flags | NONE_WIDE);
    e.with_stack(0x10, |e, header| {
        let header = header.addr();
        e.mem.set_u32(header, 0);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0000);
        reserve_at_least(e, header, count, 0x20);
        e.mem.set_u32(header + 4, count as u32);
        if count > 0 {
            for position in 0..count as u32 {
                let data = e.mem.u32(header);
                copy_words(
                    e,
                    aabbs.addr().wrapping_add(position << 5),
                    data.wrapping_add(position << 5),
                    8,
                );
            }
        }
        let data = e.mem.u32(header);
        if count == 1 {
            let first = e.mem.u32(userdata.addr());
            let node = e.mem.u32(me + 0x10);
            let flags = e.mem.u32(node + 0x28);
            e.mem.set_u32(node + 0x28, flags | LEAF_WIDE);
            e.mem.set_u32(node + 0x20, first);
            let node = e.mem.u32(me + 0x10);
            e.mem.set_u32(node + 0x2c, NONE_WIDE);
            let node = e.mem.u32(me + 0x10);
            copy_words(e, data, node, 8);
            let out_data = e.mem.u32(out);
            let root_index = e.mem.u32(me + 0x14);
            e.mem.set_u32(out_data, root_index);
        } else {
            let node = e.mem.u32(me + 0x10);
            fn_00d9d1c0(
                e,
                this,
                Ptr::new(data),
                userdata,
                count,
                node,
                out_indices,
                false,
            );
        }
        let root_index = e.mem.u32(me + 0x14);
        let node = e.mem.u32(me + 0x10);
        e.call(FIX_UP_WIDE, &args![this, node, root_index, NONE_WIDE]);
        let node = e.mem.u32(me + 0x10);
        fn_00d9c360(e, this, node, 0);
        release_array(e, header, 0x20);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_a520,
            fn_00d9a520(Ptr, Ptr, Ptr, i32, i32, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d9_a850,
            fn_00d9a850(Ptr, Ptr, Ptr, u32, i32, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d9_afe0, fn_00d9afe0(Ptr, u32, u32, u32)),
        entry!(0x00d9_b240, fn_00d9b240(Ptr, u32, Ptr) -> u32),
        entry!(
            0x00d9_b580,
            hk_aabb_tree_aabb16_add_leaf_internal(Ptr, u32, u32, u32)
        ),
        entry!(0x00d9_bb50, fn_00d9bb50(Ptr, u32, Ptr) -> u32),
        entry!(0x00d9_be20, fn_00d9be20(Ptr)),
        entry!(0x00d9_be30, hk_aabb_tree_aabb_uint_clear(Ptr)),
        entry!(0x00d9_be50, fn_00d9be50(Ptr, u32, Ptr)),
        entry!(0x00d9_bed0, fn_00d9bed0(Ptr)),
        entry!(0x00d9_bef0, fn_00d9bef0(Ptr, u16)),
        entry!(
            0x00d9_bf30,
            hk_aabb_tree_aabb16_node_manager_reserve(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00d9_c010,
            hk_aabb_tree_aabb_uint_node_manager_destruct(Ptr)
        ),
        entry!(0x00d9_c090, fn_00d9c090(Ptr)),
        entry!(0x00d9_c110, hk_aabb_tree_aabb_uint_construct(Ptr)),
        entry!(0x00d9_c180, fn_00d9c180(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d9_c210, hk_aabb_tree_aabb16_node_manager_push_back(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d9_c280, hk_aabb_tree_aabb16_node_manager_new_node(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d9_c2d0, fn_00d9c2d0(Ptr, u16)),
        entry!(0x00d9_c2e0, hk_aabb_tree_aabb_uint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00d9_c320, hk_aabb_tree_aabb16_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00d9_c360, fn_00d9c360(Ptr, u32, u32)),
        entry!(0x00d9_c450, fn_00d9c450(Ptr, i32, Ptr, Ptr)),
        entry!(0x00d9_c520, fn_00d9c520(Ptr, u32, u32)),
        entry!(0x00d9_c610, hk_aabb_tree_aabb16_add(Ptr, Ptr, u32) -> u16),
        entry!(0x00d9_c7d0, fn_00d9c7d0(Ptr, u32)),
        entry!(0x00d9_c840, fn_00d9c840(Ptr, Ptr, Ptr, i32, u32, Ptr, bool)),
        entry!(0x00d9_cd80, fn_00d9cd80(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d9_ce00, fn_00d9ce00(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d9_ce50,
            hk_aabb_tree_aabb16_build(Ptr, Ptr, Ptr, i32, Ptr)
        ),
        entry!(0x00d9_d110, fn_00d9d110(Ptr, Ptr, u32) -> u32),
        entry!(0x00d9_d1c0, fn_00d9d1c0(Ptr, Ptr, Ptr, i32, u32, Ptr, bool)),
        entry!(
            0x00d9_d650,
            hk_aabb_tree_aabb_uint_build(Ptr, Ptr, Ptr, i32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Bounds = ([f32; 3], [f32; 3]);

    /// The allocator object's two virtual functions (free a block, free an array).
    const FREE_BLOCK: u32 = 0x00cc_0008;
    const FREE_ARRAY: u32 = 0x00cc_0010;

    struct Setup {
        e: Engine,
        log: Log,
        alloc: u32,
    }

    /// What the `_reserve` doubles do: a new block of `capacity` elements, filled with 0xAA, that
    /// takes over the elements the array holds; the capacity word becomes `capacity`.
    fn grow_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let bytes = capacity * element;
        let data = e.mem.alloc(bytes.max(4));
        for at in 0..bytes {
            e.mem.set_u8(data + at, 0xaa);
        }
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        if old != 0 {
            for at in 0..size * element {
                let byte = e.mem.u8(old + at);
                e.mem.set_u8(data + at, byte);
            }
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    fn setup() -> Setup {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(FLOAT_ZERO, 0.0);
        e.mem.set_f32(FLOAT_HALF, 0.5);
        e.mem.set_f64(DOUBLE_HALF, 0.5);
        e.mem.set_f32(SPLIT_TOLERANCE, 2.0f32.powi(-22));
        e.mem.set_f32(BOUNDS_START_MIN, f32::MAX);
        e.mem.set_f32(BOUNDS_START_MAX, -f32::MAX);
        e.mem.set_f32(FLOAT_ONE, 1.0);
        for lane in 0..4u32 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(CLAMP_LIMIT + 4 * lane, 2.0e9);
            e.mem.set_f32(ZERO_VECTOR + 4 * lane, 0.0);
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
        }
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(alloc, vtable);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_BLOCK, FREE_ARRAY, FIX_UP_WIDE, FIX_UP_NARROW] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            grow_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let more_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            more_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let capacity = ((e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2).max(4);
            grow_array(e, a[1], capacity, a[2]);
            Ret::default()
        });
        e.register_double(NODE_AT_WIDE, |e, a| Ret {
            eax: e.mem.u32(a[0]).wrapping_add(a[1].wrapping_mul(0x30)),
            ..Ret::default()
        });
        e.register_double(NODE_AT_NARROW, |e, a| Ret {
            eax: e.mem.u32(a[0]).wrapping_add(u32::from(a[1] as u16) * 0x14),
            ..Ret::default()
        });
        Setup { e, log, alloc }
    }

    /// The argument lists of every logged call to `address`.
    fn calls_to(s: &Setup, address: u32) -> Vec<Vec<u32>> {
        s.log
            .borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn new_wide_empty(s: &mut Setup) -> u32 {
        let tree = s.e.mem.alloc(0x80);
        hk_aabb_tree_aabb_uint_construct(&mut s.e, Ptr::new(tree));
        tree
    }

    fn new_wide(s: &mut Setup, nodes: i32) -> u32 {
        let tree = new_wide_empty(s);
        fn_00d9c450(
            &mut s.e,
            Ptr::new(tree + 0x1c),
            nodes,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        tree
    }

    fn new_narrow_empty(s: &mut Setup) -> u32 {
        let tree = s.e.mem.alloc(0x80);
        let vectors = s.e.mem.alloc(0x30);
        for lane in 0..4u32 {
            s.e.mem.set_f32(vectors + 0x20 + 4 * lane, 1.0);
        }
        fn_00d9c180(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(vectors),
            Ptr::new(vectors + 0x10),
            Ptr::new(vectors + 0x20),
        );
        tree
    }

    fn new_narrow(s: &mut Setup, nodes: u32) -> u32 {
        let tree = new_narrow_empty(s);
        hk_aabb_tree_aabb16_node_manager_reserve(
            &mut s.e,
            Ptr::new(tree + 0x1c),
            nodes,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        tree
    }

    fn wide_at(e: &Engine, tree: u32, index: u32) -> u32 {
        e.mem.u32(tree + 0x1c) + index * 0x30
    }

    fn narrow_at(e: &Engine, tree: u32, index: u32) -> u32 {
        e.mem.u32(tree + 0x1c) + index * 0x14
    }

    /// A box along x with the unit extent in y and z.
    fn span(low: f32, high: f32) -> Bounds {
        ([low, 0.0, 0.0], [high, 1.0, 1.0])
    }

    /// Writes a wide node: bounds, the two link dwords, the parent / leaf dword, the next link.
    fn put_wide(
        e: &mut Engine,
        tree: u32,
        index: u32,
        bounds: Bounds,
        links: (u32, u32),
        flags: u32,
        next: u32,
    ) {
        let node = wide_at(e, tree, index);
        for (lane, (low, high)) in bounds.0.iter().zip(bounds.1.iter()).enumerate() {
            e.mem.set_f32(node + 4 * lane as u32, *low);
            e.mem.set_f32(node + 0x10 + 4 * lane as u32, *high);
        }
        e.mem.set_u32(node + 0x20, links.0);
        e.mem.set_u32(node + 0x24, links.1);
        e.mem.set_u32(node + 0x28, flags);
        e.mem.set_u32(node + 0x2c, next);
    }

    fn set_wide_root(e: &mut Engine, tree: u32, index: u32) {
        let node = wide_at(e, tree, index);
        e.mem.set_u32(tree + 0x10, node);
        e.mem.set_u32(tree + 0x14, index);
    }

    /// Writes a narrow node: the six bounds words (maxima first), the two link words, the
    /// parent / leaf word and the next link.
    fn put_narrow(
        e: &mut Engine,
        tree: u32,
        index: u32,
        words: [u16; 6],
        links: (u16, u16),
        flags: u16,
        next: u16,
    ) {
        let node = narrow_at(e, tree, index);
        for (word, value) in words.into_iter().enumerate() {
            e.mem.set_u16(node + 2 * word as u32, value);
        }
        e.mem.set_u16(node + 0xc, links.0);
        e.mem.set_u16(node + 0xe, links.1);
        e.mem.set_u16(node + 0x10, flags);
        e.mem.set_u16(node + 0x12, next);
    }

    fn set_narrow_root(e: &mut Engine, tree: u32, index: u32) {
        let node = narrow_at(e, tree, index);
        e.mem.set_u32(tree + 0x10, node);
        e.mem.set_u16(tree + 0x14, index as u16);
    }

    fn lanes(e: &Engine, at: u32) -> [f32; 3] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    fn words(e: &Engine, node: u32) -> [u16; 6] {
        std::array::from_fn(|word| e.mem.u16(node + 2 * word as u32))
    }

    fn snapshot(e: &Engine, from: u32, dwords: u32) -> Vec<u32> {
        (0..dwords).map(|word| e.mem.u32(from + 4 * word)).collect()
    }

    /// Wide tree G(5) = [P(3) = [A(0), B(1)], C(2)] with next links A -> B -> C.
    fn three_leaf_wide(s: &mut Setup) -> u32 {
        let tree = new_wide(s, 8);
        let e = &mut s.e;
        put_wide(e, tree, 5, span(0.0, 21.0), (3, 2), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 3, span(0.0, 11.0), (0, 1), 5, 2);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 3, 1);
        put_wide(e, tree, 1, span(10.0, 11.0), (0, 0), LEAF_WIDE | 3, 2);
        put_wide(
            e,
            tree,
            2,
            span(20.0, 21.0),
            (0, 0),
            LEAF_WIDE | 5,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 5);
        tree
    }

    /// The same shape in the 16-bit layout (one unit of x is one word).
    fn three_leaf_narrow(s: &mut Setup) -> u32 {
        let tree = new_narrow(s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            5,
            [42, 2, 2, 0, 0, 0],
            (3, 2),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 3, [22, 2, 2, 0, 0, 0], (0, 1), 5, 2);
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 3, 1);
        put_narrow(e, tree, 1, [22, 2, 2, 20, 0, 0], (0, 0), LEAF_NARROW | 3, 2);
        put_narrow(
            e,
            tree,
            2,
            [42, 2, 2, 40, 0, 0],
            (0, 0),
            LEAF_NARROW | 5,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 5);
        tree
    }

    #[test]
    fn manager_reset_clears_the_node_and_free_counts() {
        let mut s = setup();
        let manager = s.e.mem.alloc(0x20);
        for at in [0u32, 4, 8, 0xc, 0x10, 0x14] {
            s.e.mem.set_u32(manager + at, 0x11 + at);
        }
        fn_00d9be20(&mut s.e, Ptr::new(manager));
        assert_eq!(s.e.mem.u32(manager + 4), 0);
        assert_eq!(s.e.mem.u32(manager + 0x10), 0);
        assert_eq!(s.e.mem.u32(manager), 0x11);
        assert_eq!(s.e.mem.u32(manager + 8), 0x19);
        assert_eq!(s.e.mem.u32(manager + 0xc), 0x1d);
        assert_eq!(s.e.mem.u32(manager + 0x14), 0x25);
    }

    #[test]
    fn clear_empties_the_wide_tree() {
        let mut s = setup();
        let tree = new_wide(&mut s, 4);
        s.e.mem.set_u32(tree + 0x10, 0x1234);
        s.e.mem.set_u32(tree + 0x14, 2);
        s.e.mem.set_u32(tree + 0x18, 7);
        hk_aabb_tree_aabb_uint_clear(&mut s.e, Ptr::new(tree));
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        assert_eq!(s.e.mem.u32(tree + 0x14), NONE_WIDE);
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        assert_eq!(s.e.mem.u32(tree + 0x20), 0);
        assert_eq!(s.e.mem.u32(tree + 0x2c), 0);
    }

    #[test]
    fn clear_empties_the_narrow_tree_and_keeps_the_high_word() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 4);
        s.e.mem.set_u32(tree + 0x10, 0x1234);
        s.e.mem.set_u32(tree + 0x14, 0xdead_0002);
        s.e.mem.set_u32(tree + 0x18, 7);
        fn_00d9bed0(&mut s.e, Ptr::new(tree));
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        assert_eq!(s.e.mem.u32(tree + 0x14), 0xdead_7fff);
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        assert_eq!(s.e.mem.u32(tree + 0x20), 0);
        assert_eq!(s.e.mem.u32(tree + 0x2c), 0);
    }

    #[test]
    fn free_list_push_grows_only_when_full() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 2);
        let manager = tree + 0x1c;
        // The list holds two words in a capacity of two: the first push has to grow it.
        fn_00d9bef0(&mut s.e, Ptr::new(manager), 7);
        let grown = calls_to(&s, ARRAY_RESERVE_MORE);
        assert_eq!(grown, vec![vec![s.alloc, manager + 0xc, 2]]);
        // The second goes through the thunk and finds room.
        fn_00d9c2d0(&mut s.e, Ptr::new(manager), 9);
        assert_eq!(calls_to(&s, ARRAY_RESERVE_MORE).len(), 1);
        let list = s.e.mem.u32(manager + 0xc);
        assert_eq!(s.e.mem.u32(manager + 0x10), 4);
        assert_eq!(s.e.mem.u16(list + 4), 7);
        assert_eq!(s.e.mem.u16(list + 6), 9);
        assert_eq!(s.e.mem.u16(list), 1);
        assert_eq!(s.e.mem.u16(list + 2), 0);
    }

    #[test]
    fn narrow_manager_reserve_adds_nodes_and_refreshes_the_root() {
        let mut s = setup();
        let tree = new_narrow_empty(&mut s);
        let manager = tree + 0x1c;
        hk_aabb_tree_aabb16_node_manager_reserve(
            &mut s.e,
            Ptr::new(manager),
            5,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE),
            vec![
                vec![s.alloc, manager, 5, 0x14],
                vec![s.alloc, manager + 0xc, 5, 2]
            ]
        );
        assert_eq!(s.e.mem.u32(manager + 4), 5);
        let nodes = s.e.mem.u32(manager);
        for index in 0..5 {
            // Only the flag word is cleared; the rest of the new node is untouched.
            assert_eq!(s.e.mem.u16(nodes + index * 0x14 + 0x10), 0);
            assert_eq!(s.e.mem.u16(nodes + index * 0x14 + 0x12), 0xaaaa);
        }
        assert_eq!(s.e.mem.u32(manager + 0x10), 5);
        let list = s.e.mem.u32(manager + 0xc);
        let order: Vec<u16> = (0..5).map(|i| s.e.mem.u16(list + 2 * i)).collect();
        assert_eq!(order, vec![4, 3, 2, 1, 0]);
        // The root pointer was not touched while the root word is "none".
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        // Growing again appends only the new indices and refreshes the root pointer.
        s.e.mem.set_u16(tree + 0x14, 2);
        hk_aabb_tree_aabb16_node_manager_reserve(
            &mut s.e,
            Ptr::new(manager),
            0x1_0008,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        assert_eq!(s.e.mem.u32(manager + 4), 8);
        let nodes = s.e.mem.u32(manager);
        assert_eq!(s.e.mem.u32(tree + 0x10), nodes + 2 * 0x14);
        let list = s.e.mem.u32(manager + 0xc);
        let tail: Vec<u16> = (5..8).map(|i| s.e.mem.u16(list + 2 * i)).collect();
        assert_eq!(tail, vec![7, 6, 5]);
        assert_eq!(s.e.mem.u32(manager + 0x10), 8);
    }

    #[test]
    fn wide_manager_reserve_adds_nodes_and_refreshes_the_root() {
        let mut s = setup();
        let tree = new_wide_empty(&mut s);
        let manager = tree + 0x1c;
        fn_00d9c450(
            &mut s.e,
            Ptr::new(manager),
            4,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE),
            vec![
                vec![s.alloc, manager, 4, 0x30],
                vec![s.alloc, manager + 0xc, 4, 4]
            ]
        );
        let nodes = s.e.mem.u32(manager);
        for index in 0..4 {
            assert_eq!(s.e.mem.u32(nodes + index * 0x30 + 0x28), 0);
            assert_eq!(s.e.mem.u32(nodes + index * 0x30 + 0x2c), 0xaaaa_aaaa);
        }
        let list = s.e.mem.u32(manager + 0xc);
        let order: Vec<u32> = (0..4).map(|i| s.e.mem.u32(list + 4 * i)).collect();
        assert_eq!(order, vec![3, 2, 1, 0]);
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        s.e.mem.set_u32(tree + 0x14, 1);
        fn_00d9c450(
            &mut s.e,
            Ptr::new(manager),
            6,
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
        );
        let nodes = s.e.mem.u32(manager);
        assert_eq!(s.e.mem.u32(tree + 0x10), nodes + 0x30);
        let list = s.e.mem.u32(manager + 0xc);
        assert_eq!(s.e.mem.u32(list + 16), 5);
        assert_eq!(s.e.mem.u32(list + 20), 4);
        assert_eq!(s.e.mem.u32(manager + 0x10), 6);
    }

    #[test]
    fn wide_manager_destructor_frees_only_owned_arrays() {
        let mut s = setup();
        // A fresh tree owns nothing: capacity words are 0x80000000.
        let fresh = new_wide_empty(&mut s);
        hk_aabb_tree_aabb_uint_node_manager_destruct(&mut s.e, Ptr::new(fresh + 0x1c));
        assert!(calls_to(&s, FREE_ARRAY).is_empty());
        let tree = new_wide(&mut s, 3);
        let manager = tree + 0x1c;
        let nodes = s.e.mem.u32(manager);
        let list = s.e.mem.u32(manager + 0xc);
        hk_aabb_tree_aabb_uint_node_manager_destruct(&mut s.e, Ptr::new(manager));
        assert_eq!(
            calls_to(&s, FREE_ARRAY),
            vec![vec![s.alloc, list, 12], vec![s.alloc, nodes, 0x90]]
        );
        for at in [0u32, 0xc] {
            assert_eq!(s.e.mem.u32(manager + at), 0);
            assert_eq!(s.e.mem.u32(manager + at + 4), 0);
            assert_eq!(s.e.mem.u32(manager + at + 8), 0x8000_0000);
        }
    }

    #[test]
    fn narrow_manager_destructor_frees_words_and_nodes() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 3);
        let manager = tree + 0x1c;
        let nodes = s.e.mem.u32(manager);
        let list = s.e.mem.u32(manager + 0xc);
        fn_00d9c090(&mut s.e, Ptr::new(manager));
        assert_eq!(
            calls_to(&s, FREE_ARRAY),
            vec![vec![s.alloc, list, 6], vec![s.alloc, nodes, 0x3c]]
        );
        assert_eq!(s.e.mem.u32(manager), 0);
        assert_eq!(s.e.mem.u32(manager + 8), 0x8000_0000);
        assert_eq!(s.e.mem.u32(manager + 0x14), 0x8000_0000);
    }

    #[test]
    fn wide_constructor_fills_the_defaults() {
        let mut s = setup();
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            s.e.mem.set_f32(ZERO_VECTOR + 4 * lane as u32, value);
        }
        let tree = s.e.mem.alloc(0x80);
        for at in 0..0x20 {
            s.e.mem.set_u32(tree + 4 * at, 0x5555_5555);
        }
        hk_aabb_tree_aabb_uint_construct(&mut s.e, Ptr::new(tree));
        assert_eq!(s.e.mem.u32(tree), WIDE_TREE_VTABLE);
        let manager: Vec<u32> = (0..6).map(|i| s.e.mem.u32(tree + 0x1c + 4 * i)).collect();
        assert_eq!(manager, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
        for at in [0x40u32, 0x50] {
            assert_eq!(s.e.mem.f32(tree + at), 1.0);
            assert_eq!(s.e.mem.f32(tree + at + 4), 2.0);
            assert_eq!(s.e.mem.f32(tree + at + 8), 3.0);
            assert_eq!(s.e.mem.f32(tree + at + 12), 4.0);
        }
        for at in [0x60u32, 0x70] {
            assert_eq!(lanes(&s.e, tree + at), [1.0, 1.0, 1.0]);
            assert_eq!(s.e.mem.f32(tree + at + 12), 0.0);
        }
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        assert_eq!(s.e.mem.u32(tree + 0x14), NONE_WIDE);
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
    }

    #[test]
    fn narrow_constructor_copies_vectors_and_refines_the_reciprocal() {
        let mut s = setup();
        let tree = s.e.mem.alloc(0x80);
        let vectors = s.e.mem.alloc(0x30);
        let values = [
            [1.0f32, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [2.0, 4.0, 0.5, 9.0],
        ];
        for (row, vector) in values.iter().enumerate() {
            for (lane, value) in vector.iter().enumerate() {
                s.e.mem
                    .set_f32(vectors + 0x10 * row as u32 + 4 * lane as u32, *value);
            }
        }
        fn_00d9c180(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(vectors),
            Ptr::new(vectors + 0x10),
            Ptr::new(vectors + 0x20),
        );
        assert_eq!(s.e.mem.u32(tree), NARROW_TREE_VTABLE);
        for (row, vector) in values.iter().enumerate() {
            for (lane, value) in vector.iter().enumerate() {
                assert_eq!(
                    s.e.mem
                        .f32(tree + 0x40 + 0x10 * row as u32 + 4 * lane as u32),
                    *value
                );
            }
        }
        // 1 / scale, lane 3 from the ones vector (RCPPS plus one Newton step: about 1e-7).
        for (lane, wanted) in [0.5f32, 0.25, 2.0, 1.0].into_iter().enumerate() {
            let got = s.e.mem.f32(tree + 0x70 + 4 * lane as u32);
            assert!((got - wanted).abs() <= wanted * 1.0e-5, "{lane}: {got}");
        }
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        assert_eq!(s.e.mem.u16(tree + 0x14), NONE_NARROW);
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        let manager: Vec<u32> = (0..6).map(|i| s.e.mem.u32(tree + 0x1c + 4 * i)).collect();
        assert_eq!(manager, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
    }

    #[test]
    fn narrow_constructor_answers_this() {
        let mut s = setup();
        let tree = s.e.mem.alloc(0x80);
        let vectors = s.e.mem.alloc(0x30);
        for lane in 0..12u32 {
            s.e.mem.set_f32(vectors + 4 * lane, 1.0);
        }
        // `MOV EAX,ECX` first; the caller stores the answer (00d08e53).
        let answered =
            s.e.call(
                0x00d9_c180,
                &args![tree, vectors, vectors + 0x10, vectors + 0x20],
            )
            .u32();
        assert_eq!(answered, tree);
    }

    #[test]
    fn narrow_push_back_takes_the_last_free_index_and_grows_when_empty() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 4);
        let manager = tree + 0x1c;
        let node = s.e.mem.alloc(0x14);
        for word in 0..5u32 {
            s.e.mem.set_u32(node + 4 * word, 0x100 + word);
        }
        let index = hk_aabb_tree_aabb16_node_manager_push_back(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(node),
        );
        assert_eq!(index, 0);
        assert_eq!(s.e.mem.u32(manager + 0x10), 3);
        let target = narrow_at(&s.e, tree, 0);
        assert_eq!(
            snapshot(&s.e, target, 5),
            vec![0x100, 0x101, 0x102, 0x103, 0x104]
        );
        assert_eq!(calls_to(&s, ARRAY_RESERVE).len(), 2);
        // With the free list empty, the node array grows by 0x80 first.
        s.e.mem.set_u32(manager + 0x10, 0);
        let index = hk_aabb_tree_aabb16_node_manager_push_back(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(node),
        );
        assert_eq!(index, 4);
        assert_eq!(s.e.mem.u32(manager + 4), 0x84);
        assert_eq!(calls_to(&s, ARRAY_RESERVE)[2][2], 0x84);
        assert_eq!(s.e.mem.u32(manager + 0x10), 0x7f);
    }

    #[test]
    fn narrow_new_node_returns_the_node_and_its_index() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 4);
        let manager = tree + 0x1c;
        let slot = s.e.mem.alloc(4);
        s.e.mem.set_u32(slot, 0xffff_ffff);
        let node = hk_aabb_tree_aabb16_node_manager_new_node(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(slot),
        );
        assert_eq!(s.e.mem.u32(slot), 0xffff_0000);
        assert_eq!(node, narrow_at(&s.e, tree, 0));
        assert_eq!(s.e.mem.u32(manager + 0x10), 3);
        s.e.mem.set_u32(manager + 0x10, 0);
        let node = hk_aabb_tree_aabb16_node_manager_new_node(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(slot),
        );
        assert_eq!(s.e.mem.u16(slot), 4);
        assert_eq!(node, narrow_at(&s.e, tree, 4));
        assert_eq!(calls_to(&s, ARRAY_RESERVE)[2][2], 0x84);
    }

    #[test]
    fn wide_push_back_copies_the_whole_node() {
        let mut s = setup();
        let tree = new_wide(&mut s, 4);
        let manager = tree + 0x1c;
        let node = s.e.mem.alloc(0x30);
        for word in 0..12u32 {
            s.e.mem.set_u32(node + 4 * word, 0x200 + word);
        }
        let index = fn_00d9cd80(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(node),
        );
        assert_eq!(index, 0);
        let target = wide_at(&s.e, tree, 0);
        let want: Vec<u32> = (0..12).map(|word| 0x200 + word).collect();
        assert_eq!(snapshot(&s.e, target, 12), want);
        assert_eq!(calls_to(&s, ARRAY_RESERVE).len(), 2);
        s.e.mem.set_u32(manager + 0x10, 0);
        let index = fn_00d9cd80(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(node),
        );
        assert_eq!(index, 4);
        assert_eq!(s.e.mem.u32(manager + 4), 0x84);
    }

    #[test]
    fn wide_new_node_returns_the_node_and_its_index() {
        let mut s = setup();
        let tree = new_wide(&mut s, 4);
        let manager = tree + 0x1c;
        let slot = s.e.mem.alloc(4);
        let node = fn_00d9ce00(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(slot),
        );
        assert_eq!(s.e.mem.u32(slot), 0);
        assert_eq!(node, wide_at(&s.e, tree, 0));
        assert_eq!(s.e.mem.u32(manager + 0x10), 3);
        s.e.mem.set_u32(manager + 0x10, 0);
        let node = fn_00d9ce00(
            &mut s.e,
            Ptr::new(manager),
            Ptr::new(tree + 0x10),
            Ptr::new(tree + 0x14),
            Ptr::new(slot),
        );
        assert_eq!(s.e.mem.u32(slot), 4);
        assert_eq!(node, wide_at(&s.e, tree, 4));
        assert_eq!(s.e.mem.u32(manager + 4), 0x84);
    }

    #[test]
    fn scalar_deleting_destructors_free_the_object_only_with_the_flag() {
        let mut s = setup();
        let wide = new_wide(&mut s, 2);
        s.e.mem.set_u32(wide + 0x18, 5);
        let got = hk_aabb_tree_aabb_uint_scalar_deleting_destructor(&mut s.e, Ptr::new(wide), 0);
        assert_eq!(got.addr(), wide);
        assert_eq!(s.e.mem.u32(wide), WIDE_TREE_VTABLE);
        assert_eq!(s.e.mem.u32(wide + 0x14), NONE_WIDE);
        assert_eq!(s.e.mem.u32(wide + 0x18), 0);
        assert_eq!(calls_to(&s, FREE_ARRAY).len(), 2);
        assert!(calls_to(&s, FREE_BLOCK).is_empty());
        let narrow = new_narrow(&mut s, 2);
        let got = hk_aabb_tree_aabb16_scalar_deleting_destructor(&mut s.e, Ptr::new(narrow), 3);
        assert_eq!(got.addr(), narrow);
        assert_eq!(s.e.mem.u32(narrow), NARROW_TREE_VTABLE);
        assert_eq!(s.e.mem.u16(narrow + 0x14), NONE_NARROW);
        assert_eq!(calls_to(&s, FREE_ARRAY).len(), 4);
        assert_eq!(calls_to(&s, FREE_BLOCK), vec![vec![s.alloc, narrow, 0x80]]);
    }

    #[test]
    fn wide_leaf_joins_a_root_leaf_under_a_new_root() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        put_wide(
            e,
            tree,
            0,
            span(0.0, 1.0),
            (0, 0),
            LEAF_WIDE | NONE_WIDE,
            NONE_WIDE,
        );
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | NONE_WIDE,
            77,
        );
        set_wide_root(e, tree, 0);
        fn_00d9afe0(e, Ptr::new(tree), 0, 1, 2);
        let joint = wide_at(e, tree, 2);
        assert_eq!(e.mem.u32(tree + 0x10), joint);
        assert_eq!(e.mem.u32(tree + 0x14), 2);
        assert_eq!(e.mem.u32(joint + 0x20), 0);
        assert_eq!(e.mem.u32(joint + 0x24), 1);
        assert_eq!(e.mem.u32(joint + 0x28), NONE_WIDE);
        assert_eq!(lanes(e, joint), [0.0, 0.0, 0.0]);
        assert_eq!(lanes(e, joint + 0x10), [11.0, 1.0, 1.0]);
        let first = wide_at(e, tree, 0);
        let second = wide_at(e, tree, 1);
        assert_eq!(e.mem.u32(first + 0x28), LEAF_WIDE | 2);
        assert_eq!(e.mem.u32(second + 0x28), LEAF_WIDE | 2);
        // The next-leaf links: root leaf -> new leaf -> whatever followed the root leaf.
        assert_eq!(e.mem.u32(first + 0x2c), 1);
        assert_eq!(e.mem.u32(second + 0x2c), NONE_WIDE);
        assert_eq!(e.mem.u32(joint + 0x2c), NONE_WIDE);
    }

    #[test]
    fn wide_leaf_takes_the_child_the_exe_selects_and_hangs_the_new_node_there() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        put_wide(e, tree, 3, span(0.0, 11.0), (0, 1), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 3, 1);
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | 3,
            NONE_WIDE,
        );
        put_wide(
            e,
            tree,
            2,
            span(10.25, 10.75),
            (0, 0),
            LEAF_WIDE | NONE_WIDE,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 3);
        // The leaf sits on the second child, but the squared-offset difference comes out
        // negative, which the exe turns into child 0 (the sign is inverted before use).
        fn_00d9afe0(e, Ptr::new(tree), 3, 2, 4);
        let joint = wide_at(e, tree, 4);
        let parent = wide_at(e, tree, 3);
        assert_eq!(e.mem.u32(parent + 0x20), 4);
        assert_eq!(e.mem.u32(parent + 0x24), 1);
        assert_eq!(e.mem.u32(joint + 0x20), 0);
        assert_eq!(e.mem.u32(joint + 0x24), 2);
        assert_eq!(e.mem.u32(joint + 0x28), 3);
        assert_eq!(lanes(e, joint), [0.0, 0.0, 0.0]);
        assert_eq!(lanes(e, joint + 0x10), [10.75, 1.0, 1.0]);
        let first = wide_at(e, tree, 0);
        assert_eq!(e.mem.u32(first + 0x28), LEAF_WIDE | 4);
        assert_eq!(e.mem.u32(first + 0x2c), 2);
        let leaf = wide_at(e, tree, 2);
        assert_eq!(e.mem.u32(leaf + 0x2c), 1);
        assert_eq!(e.mem.u32(joint + 0x2c), 1);
        // The old parent already contains the leaf: its bounds stay.
        assert_eq!(lanes(e, parent + 0x10), [11.0, 1.0, 1.0]);
    }

    #[test]
    fn wide_leaf_on_the_second_slot_repairs_links_and_widens_ancestors() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        // The root's bounds are stale (too small) so that the walk up has to widen them.
        let stale: Bounds = ([0.0, 0.0, 0.0], [0.5, 1.0, 1.0]);
        put_wide(e, tree, 3, stale, (0, 1), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 3, 1);
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | 3,
            NONE_WIDE,
        );
        put_wide(
            e,
            tree,
            2,
            span(0.25, 0.75),
            (0, 0),
            LEAF_WIDE | NONE_WIDE,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 3);
        fn_00d9afe0(e, Ptr::new(tree), 3, 2, 4);
        let parent = wide_at(e, tree, 3);
        let joint = wide_at(e, tree, 4);
        // The leaf is closest to child 0, which the exe pairs with child 1 (the farther one).
        assert_eq!(e.mem.u32(parent + 0x24), 4);
        assert_eq!(e.mem.u32(joint + 0x20), 1);
        assert_eq!(e.mem.u32(joint + 0x24), 2);
        // Walking the first subtree of the parent: its leaf now links to the new node.
        let first = wide_at(e, tree, 0);
        assert_eq!(e.mem.u32(first + 0x2c), 4);
        // The parent's bounds were widened to the leaf, and the walk stopped at the root.
        assert_eq!(lanes(e, parent + 0x10), [0.75, 1.0, 1.0]);
    }

    #[test]
    fn narrow_leaf_joins_a_root_leaf_under_a_new_root() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            0,
            [2, 2, 2, 0, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            77,
        );
        set_narrow_root(e, tree, 0);
        hk_aabb_tree_aabb16_add_leaf_internal(e, Ptr::new(tree), 0, 1, 2);
        let joint = narrow_at(e, tree, 2);
        assert_eq!(e.mem.u32(tree + 0x10), joint);
        assert_eq!(e.mem.u16(tree + 0x14), 2);
        assert_eq!(words(e, joint), [22, 2, 2, 0, 0, 0]);
        assert_eq!(e.mem.u16(joint + 0xc), 0);
        assert_eq!(e.mem.u16(joint + 0xe), 1);
        assert_eq!(e.mem.u16(joint + 0x10), NONE_NARROW);
        let first = narrow_at(e, tree, 0);
        let second = narrow_at(e, tree, 1);
        assert_eq!(e.mem.u16(first + 0x10), LEAF_NARROW | 2);
        assert_eq!(e.mem.u16(second + 0x10), LEAF_NARROW | 2);
        assert_eq!(e.mem.u16(first + 0x12), 1);
        assert_eq!(e.mem.u16(second + 0x12), NONE_NARROW);
        assert_eq!(e.mem.u16(joint + 0x12), NONE_NARROW);
    }

    #[test]
    fn narrow_leaf_goes_to_the_nearer_child() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            3,
            [22, 2, 2, 0, 0, 0],
            (0, 1),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 3, 1);
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | 3,
            NONE_NARROW,
        );
        put_narrow(
            e,
            tree,
            2,
            [21, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 3);
        hk_aabb_tree_aabb16_add_leaf_internal(e, Ptr::new(tree), 3, 2, 4);
        let parent = narrow_at(e, tree, 3);
        let joint = narrow_at(e, tree, 4);
        assert_eq!(e.mem.u16(parent + 0xc), 0);
        assert_eq!(e.mem.u16(parent + 0xe), 4);
        assert_eq!(e.mem.u16(joint + 0xc), 1);
        assert_eq!(e.mem.u16(joint + 0xe), 2);
        assert_eq!(e.mem.u16(joint + 0x10), 3);
        assert_eq!(words(e, joint), [22, 2, 2, 20, 0, 0]);
        let near = narrow_at(e, tree, 1);
        assert_eq!(e.mem.u16(near + 0x10), LEAF_NARROW | 4);
        assert_eq!(e.mem.u16(near + 0x12), 2);
        // Second slot: the first subtree's leaf now links to the new node.
        let first = narrow_at(e, tree, 0);
        assert_eq!(e.mem.u16(first + 0x12), 4);
    }

    #[test]
    fn narrow_leaf_widens_a_stale_parent() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            3,
            [21, 2, 2, 0, 0, 0],
            (0, 1),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 3, 1);
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | 3,
            NONE_NARROW,
        );
        put_narrow(
            e,
            tree,
            2,
            [23, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 3);
        hk_aabb_tree_aabb16_add_leaf_internal(e, Ptr::new(tree), 3, 2, 4);
        let parent = narrow_at(e, tree, 3);
        assert_eq!(words(e, parent), [23, 2, 2, 0, 0, 0]);
        // A parent that already holds the leaf is left alone (checked with a second leaf).
        put_narrow(
            e,
            tree,
            5,
            [3, 2, 2, 1, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            NONE_NARROW,
        );
        hk_aabb_tree_aabb16_add_leaf_internal(e, Ptr::new(tree), 3, 5, 6);
        assert_eq!(words(e, parent), [23, 2, 2, 0, 0, 0]);
    }

    #[test]
    fn wide_remove_leaf_replaces_the_parent_by_the_sibling() {
        let mut s = setup();
        let tree = three_leaf_wide(&mut s);
        let out = s.e.mem.alloc(4);
        let got = fn_00d9b240(&mut s.e, Ptr::new(tree), 1, Ptr::new(out));
        let e = &s.e;
        assert_eq!(e.mem.u32(out), 3);
        let grand = wide_at(e, tree, 5);
        assert_eq!(e.mem.u32(grand + 0x20), 0);
        assert_eq!(e.mem.u32(grand + 0x24), 2);
        let sibling = wide_at(e, tree, 0);
        assert_eq!(e.mem.u32(sibling + 0x28), LEAF_WIDE | 5);
        // The refit stops at the grandparent: its upper bound is unchanged.
        assert_eq!(got, grand);
        // The sibling inherits the removed parent's next link.
        assert_eq!(e.mem.u32(sibling + 0x2c), 2);
    }

    #[test]
    fn wide_remove_leaf_refits_stale_ancestors() {
        let mut s = setup();
        let tree = three_leaf_wide(&mut s);
        let grand = wide_at(&s.e, tree, 5);
        s.e.mem.set_f32(grand + 0x10, 99.0);
        s.e.mem.set_f32(grand, -5.0);
        let out = s.e.mem.alloc(4);
        let got = fn_00d9b240(&mut s.e, Ptr::new(tree), 1, Ptr::new(out));
        assert_eq!(lanes(&s.e, grand + 0x10), [21.0, 1.0, 1.0]);
        assert_eq!(lanes(&s.e, grand), [0.0, 0.0, 0.0]);
        // The walk ran out of ancestors: the answer is the root pointer read at the start.
        assert_eq!(got, grand);
    }

    #[test]
    fn wide_remove_leaf_under_the_root_makes_the_sibling_the_root() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        put_wide(e, tree, 2, span(0.0, 11.0), (0, 1), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 2, 1);
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | 2,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 2);
        let out = e.mem.alloc(4);
        let got = fn_00d9b240(e, Ptr::new(tree), 0, Ptr::new(out));
        let sibling = wide_at(e, tree, 1);
        assert_eq!(got, sibling);
        assert_eq!(e.mem.u32(tree + 0x10), sibling);
        assert_eq!(e.mem.u32(tree + 0x14), 1);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(e.mem.u32(sibling + 0x28), LEAF_WIDE | NONE_WIDE);
    }

    #[test]
    fn wide_remove_leaf_in_a_right_subtree_repairs_the_left_neighbour_link() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        put_wide(e, tree, 5, span(0.0, 21.0), (2, 3), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 2, span(20.0, 21.0), (0, 0), LEAF_WIDE | 5, 3);
        put_wide(e, tree, 3, span(0.0, 11.0), (0, 1), 5, NONE_WIDE);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 3, 1);
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | 3,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 5);
        let out = e.mem.alloc(4);
        let got = fn_00d9b240(e, Ptr::new(tree), 1, Ptr::new(out));
        let grand = wide_at(e, tree, 5);
        assert_eq!(e.mem.u32(grand + 0x24), 0);
        // The left neighbour of the removed parent now links to the sibling.
        let neighbour = wide_at(e, tree, 2);
        assert_eq!(e.mem.u32(neighbour + 0x2c), 0);
        let sibling = wide_at(e, tree, 0);
        assert_eq!(e.mem.u32(sibling + 0x2c), NONE_WIDE);
        assert_eq!(got, grand);
    }

    #[test]
    fn narrow_remove_leaf_replaces_the_parent_by_the_sibling() {
        let mut s = setup();
        let tree = three_leaf_narrow(&mut s);
        let out = s.e.mem.alloc(4);
        let got = fn_00d9bb50(&mut s.e, Ptr::new(tree), 1, Ptr::new(out));
        let e = &s.e;
        assert_eq!(e.mem.u16(out), 3);
        let grand = narrow_at(e, tree, 5);
        assert_eq!(e.mem.u16(grand + 0xc), 0);
        assert_eq!(e.mem.u16(grand + 0xe), 2);
        let sibling = narrow_at(e, tree, 0);
        assert_eq!(e.mem.u16(sibling + 0x10), LEAF_NARROW | 5);
        assert_eq!(e.mem.u16(sibling + 0x12), 2);
        assert_eq!(got, grand);
    }

    #[test]
    fn narrow_remove_leaf_refits_stale_ancestors() {
        let mut s = setup();
        let tree = three_leaf_narrow(&mut s);
        let grand = narrow_at(&s.e, tree, 5);
        s.e.mem.set_u16(grand, 99);
        s.e.mem.set_u16(grand + 6, 7);
        let out = s.e.mem.alloc(4);
        let got = fn_00d9bb50(&mut s.e, Ptr::new(tree), 1, Ptr::new(out));
        assert_eq!(words(&s.e, grand), [42, 2, 2, 0, 0, 0]);
        assert_eq!(got, grand);
    }

    #[test]
    fn narrow_remove_leaf_under_the_root_and_in_a_right_subtree() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            2,
            [22, 2, 2, 0, 0, 0],
            (0, 1),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 2, 1);
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | 2,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 2);
        let out = e.mem.alloc(4);
        let got = fn_00d9bb50(e, Ptr::new(tree), 0, Ptr::new(out));
        let sibling = narrow_at(e, tree, 1);
        assert_eq!(got, sibling);
        assert_eq!(e.mem.u32(tree + 0x10), sibling);
        assert_eq!(e.mem.u16(tree + 0x14), 1);
        assert_eq!(e.mem.u16(out), 2);
        assert_eq!(e.mem.u16(sibling + 0x10), LEAF_NARROW | NONE_NARROW);

        let tree = new_narrow(&mut s, 8);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            5,
            [42, 2, 2, 0, 0, 0],
            (2, 3),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 2, [42, 2, 2, 40, 0, 0], (0, 0), LEAF_NARROW | 5, 3);
        put_narrow(e, tree, 3, [22, 2, 2, 0, 0, 0], (0, 1), 5, NONE_NARROW);
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 3, 1);
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | 3,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 5);
        let got = fn_00d9bb50(e, Ptr::new(tree), 1, Ptr::new(out));
        let grand = narrow_at(e, tree, 5);
        assert_eq!(e.mem.u16(grand + 0xe), 0);
        let neighbour = narrow_at(e, tree, 2);
        assert_eq!(e.mem.u16(neighbour + 0x12), 0);
        let sibling = narrow_at(e, tree, 0);
        assert_eq!(e.mem.u16(sibling + 0x12), NONE_NARROW);
        assert_eq!(got, grand);
    }

    #[test]
    fn set_bounds_of_the_root_leaf_only_overwrites() {
        let mut s = setup();
        let tree = new_wide(&mut s, 4);
        put_wide(
            &mut s.e,
            tree,
            0,
            span(0.0, 1.0),
            (0, 0),
            LEAF_WIDE | NONE_WIDE,
            NONE_WIDE,
        );
        set_wide_root(&mut s.e, tree, 0);
        let bounds = s.e.mem.alloc(0x20);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0]
            .into_iter()
            .enumerate()
        {
            s.e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        fn_00d9be50(&mut s.e, Ptr::new(tree), 0, Ptr::new(bounds));
        let node = wide_at(&s.e, tree, 0);
        assert_eq!(lanes(&s.e, node), [1.0, 2.0, 3.0]);
        assert_eq!(lanes(&s.e, node + 0x10), [4.0, 5.0, 6.0]);
        assert_eq!(s.e.mem.u32(tree + 0x14), 0);
    }

    #[test]
    fn set_bounds_of_an_inner_leaf_removes_and_reinserts_it() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let e = &mut s.e;
        put_wide(e, tree, 2, span(0.0, 11.0), (0, 1), NONE_WIDE, NONE_WIDE);
        put_wide(e, tree, 0, span(0.0, 1.0), (0, 0), LEAF_WIDE | 2, 1);
        put_wide(
            e,
            tree,
            1,
            span(10.0, 11.0),
            (0, 0),
            LEAF_WIDE | 2,
            NONE_WIDE,
        );
        set_wide_root(e, tree, 2);
        let bounds = e.mem.alloc(0x20);
        for (lane, value) in [30.0f32, 0.0, 0.0, 0.0, 31.0, 1.0, 1.0, 0.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        fn_00d9be50(e, Ptr::new(tree), 0, Ptr::new(bounds));
        // The freed joint node (2) is reused as the new root over the sibling and the leaf.
        let joint = wide_at(e, tree, 2);
        assert_eq!(e.mem.u32(tree + 0x14), 2);
        assert_eq!(e.mem.u32(tree + 0x10), joint);
        assert_eq!(e.mem.u32(joint + 0x20), 1);
        assert_eq!(e.mem.u32(joint + 0x24), 0);
        assert_eq!(lanes(e, joint), [10.0, 0.0, 0.0]);
        assert_eq!(lanes(e, joint + 0x10), [31.0, 1.0, 1.0]);
        let moved = wide_at(e, tree, 0);
        assert_eq!(lanes(e, moved), [30.0, 0.0, 0.0]);
        assert_eq!(lanes(e, moved + 0x10), [31.0, 1.0, 1.0]);
    }

    #[test]
    fn wide_add_makes_the_first_leaf_the_root_and_joins_later_ones() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let bounds = s.e.mem.alloc(0x20);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0]
            .into_iter()
            .enumerate()
        {
            s.e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        let first = fn_00d9d110(&mut s.e, Ptr::new(tree), Ptr::new(bounds), 0xcafe);
        assert_eq!(first, 0);
        let node = wide_at(&s.e, tree, 0);
        assert_eq!(lanes(&s.e, node), [1.0, 2.0, 3.0]);
        assert_eq!(lanes(&s.e, node + 0x10), [4.0, 5.0, 6.0]);
        assert_eq!(s.e.mem.u32(node + 0x20), 0xcafe);
        assert_eq!(s.e.mem.u32(node + 0x28), LEAF_WIDE | NONE_WIDE);
        assert_eq!(s.e.mem.u32(node + 0x2c), NONE_WIDE);
        assert_eq!(s.e.mem.u32(tree + 0x10), node);
        assert_eq!(s.e.mem.u32(tree + 0x14), 0);
        assert_eq!(s.e.mem.u32(tree + 0x18), 1);
        for (lane, value) in [11.0f32, 2.0, 3.0, 0.0, 14.0, 5.0, 6.0, 0.0]
            .into_iter()
            .enumerate()
        {
            s.e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        let second = fn_00d9d110(&mut s.e, Ptr::new(tree), Ptr::new(bounds), 0xbeef);
        assert_eq!(second, 1);
        assert_eq!(s.e.mem.u32(tree + 0x18), 2);
        // The root is the new joint node (index 2) over both leaves.
        assert_eq!(s.e.mem.u32(tree + 0x14), 2);
        let joint = wide_at(&s.e, tree, 2);
        assert_eq!(s.e.mem.u32(tree + 0x10), joint);
        assert_eq!(lanes(&s.e, joint), [1.0, 2.0, 3.0]);
        assert_eq!(lanes(&s.e, joint + 0x10), [14.0, 5.0, 6.0]);
        assert_eq!(s.e.mem.u32(joint + 0x20), 0);
        assert_eq!(s.e.mem.u32(joint + 0x24), 1);
    }

    #[test]
    fn narrow_add_quantises_the_bounds_and_builds_the_tree() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let bounds = s.e.mem.alloc(0x20);
        for (lane, value) in [
            32768.0f32, 65536.0, 98304.0, 0.0, 131072.0, 163840.0, 196608.0, 0.0,
        ]
        .into_iter()
        .enumerate()
        {
            s.e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        let first = hk_aabb_tree_aabb16_add(&mut s.e, Ptr::new(tree), Ptr::new(bounds), 0xcafe);
        assert_eq!(first, 0);
        let node = narrow_at(&s.e, tree, 0);
        assert_eq!(words(&s.e, node), [4, 5, 6, 1, 2, 3]);
        assert_eq!(s.e.mem.u32(node + 0xc), 0xcafe);
        assert_eq!(s.e.mem.u16(node + 0x10), 0xffff);
        assert_eq!(s.e.mem.u16(node + 0x12), NONE_NARROW);
        assert_eq!(s.e.mem.u32(tree + 0x10), node);
        assert_eq!(s.e.mem.u16(tree + 0x14), 0);
        assert_eq!(s.e.mem.u32(tree + 0x18), 1);
        // A second box: the lower corner is clamped at zero, the upper rounded to the grid.
        for (lane, value) in [-5.0f32, 0.0, 0.0, 0.0, 65536.0, 32768.0, 32768.0, 0.0]
            .into_iter()
            .enumerate()
        {
            s.e.mem.set_f32(bounds + 4 * lane as u32, value);
        }
        let second = hk_aabb_tree_aabb16_add(&mut s.e, Ptr::new(tree), Ptr::new(bounds), 0xbeef);
        assert_eq!(second, 1);
        let leaf = narrow_at(&s.e, tree, 1);
        assert_eq!(words(&s.e, leaf), [2, 1, 1, 0, 0, 0]);
        assert_eq!(s.e.mem.u32(tree + 0x18), 2);
        assert_eq!(s.e.mem.u16(tree + 0x14), 2);
        let joint = narrow_at(&s.e, tree, 2);
        assert_eq!(s.e.mem.u32(tree + 0x10), joint);
        assert_eq!(words(&s.e, joint), [4, 5, 6, 0, 0, 0]);
        assert_eq!(s.e.mem.u16(joint + 0xc), 0);
        assert_eq!(s.e.mem.u16(joint + 0xe), 1);
    }

    #[test]
    fn narrow_remove_empties_a_single_leaf_tree_and_frees_two_nodes_otherwise() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let manager = tree + 0x1c;
        s.e.mem.set_u32(manager + 0x10, 0);
        put_narrow(
            &mut s.e,
            tree,
            0,
            [2, 2, 2, 0, 0, 0],
            (0, 0),
            LEAF_NARROW | NONE_NARROW,
            NONE_NARROW,
        );
        set_narrow_root(&mut s.e, tree, 0);
        s.e.mem.set_u32(tree + 0x18, 1);
        fn_00d9c7d0(&mut s.e, Ptr::new(tree), 0);
        assert_eq!(s.e.mem.u16(tree + 0x14), NONE_NARROW);
        assert_eq!(s.e.mem.u32(tree + 0x10), 0);
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        assert_eq!(s.e.mem.u32(manager + 0x10), 1);
        let list = s.e.mem.u32(manager + 0xc);
        assert_eq!(s.e.mem.u16(list), 0);

        let tree = new_narrow(&mut s, 8);
        let manager = tree + 0x1c;
        s.e.mem.set_u32(manager + 0x10, 0);
        let e = &mut s.e;
        put_narrow(
            e,
            tree,
            2,
            [22, 2, 2, 0, 0, 0],
            (0, 1),
            NONE_NARROW,
            NONE_NARROW,
        );
        put_narrow(e, tree, 0, [2, 2, 2, 0, 0, 0], (0, 0), LEAF_NARROW | 2, 1);
        put_narrow(
            e,
            tree,
            1,
            [22, 2, 2, 20, 0, 0],
            (0, 0),
            LEAF_NARROW | 2,
            NONE_NARROW,
        );
        set_narrow_root(e, tree, 2);
        e.mem.set_u32(tree + 0x18, 2);
        fn_00d9c7d0(e, Ptr::new(tree), 0);
        assert_eq!(e.mem.u32(tree + 0x18), 1);
        assert_eq!(e.mem.u32(manager + 0x10), 2);
        let list = e.mem.u32(manager + 0xc);
        assert_eq!(e.mem.u16(list), 2);
        assert_eq!(e.mem.u16(list + 2), 0);
        assert_eq!(e.mem.u32(tree + 0x10), narrow_at(e, tree, 1));
    }

    #[test]
    fn walks_leave_the_tree_alone_and_ignore_a_null_start() {
        let mut s = setup();
        let tree = three_leaf_wide(&mut s);
        let nodes = s.e.mem.u32(tree + 0x1c);
        let before = snapshot(&s.e, nodes, 12 * 8);
        let root = wide_at(&s.e, tree, 5);
        fn_00d9c360(&mut s.e, Ptr::new(tree), root, 0);
        fn_00d9c360(&mut s.e, Ptr::new(tree), 0, 0);
        assert_eq!(snapshot(&s.e, nodes, 12 * 8), before);
        let tree = three_leaf_narrow(&mut s);
        let nodes = s.e.mem.u32(tree + 0x1c);
        let before = snapshot(&s.e, nodes, 5 * 8);
        let root = narrow_at(&s.e, tree, 5);
        fn_00d9c520(&mut s.e, Ptr::new(tree), root, 0);
        fn_00d9c520(&mut s.e, Ptr::new(tree), 0, 0);
        assert_eq!(snapshot(&s.e, nodes, 5 * 8), before);
        // The inline stack is never freed and never grown for a shallow tree.
        assert!(calls_to(&s, FREE_ARRAY).is_empty());
        assert!(calls_to(&s, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn a_deep_wide_walk_grows_its_stack_and_frees_it() {
        let mut s = setup();
        let depth = 130u32;
        let tree = new_wide(&mut s, 280);
        let e = &mut s.e;
        let last_leaf = 2 * depth;
        for k in 0..depth {
            let child = if k + 1 < depth { k + 1 } else { last_leaf };
            put_wide(e, tree, k, span(0.0, 1.0), (child, depth + k), 0, NONE_WIDE);
            put_wide(
                e,
                tree,
                depth + k,
                span(0.0, 1.0),
                (0, 0),
                LEAF_WIDE,
                NONE_WIDE,
            );
        }
        put_wide(
            e,
            tree,
            last_leaf,
            span(0.0, 1.0),
            (0, 0),
            LEAF_WIDE,
            depth + depth - 1,
        );
        for k in 0..depth - 1 {
            // A left child links to its sibling leaf.
            let node = wide_at(e, tree, k + 1);
            e.mem.set_u32(node + 0x2c, depth + k);
        }
        let root = wide_at(e, tree, 0);
        fn_00d9c360(e, Ptr::new(tree), root, 0);
        assert_eq!(calls_to(&s, ARRAY_RESERVE_MORE).len(), 1);
        let freed = calls_to(&s, FREE_ARRAY);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 256 * 4);
    }

    #[test]
    fn a_deep_narrow_walk_grows_its_stack_and_frees_it() {
        let mut s = setup();
        let depth = 130u32;
        let tree = new_narrow(&mut s, 280);
        let e = &mut s.e;
        let last_leaf = 2 * depth;
        for k in 0..depth {
            let child = if k + 1 < depth { k + 1 } else { last_leaf };
            put_narrow(
                e,
                tree,
                k,
                [2, 2, 2, 0, 0, 0],
                (child as u16, (depth + k) as u16),
                0,
                NONE_NARROW,
            );
            put_narrow(
                e,
                tree,
                depth + k,
                [2, 2, 2, 0, 0, 0],
                (0, 0),
                LEAF_NARROW,
                NONE_NARROW,
            );
        }
        put_narrow(
            e,
            tree,
            last_leaf,
            [2, 2, 2, 0, 0, 0],
            (0, 0),
            LEAF_NARROW,
            (depth + depth - 1) as u16,
        );
        for k in 0..depth - 1 {
            let node = narrow_at(e, tree, k + 1);
            e.mem.set_u16(node + 0x12, (depth + k) as u16);
        }
        let root = narrow_at(e, tree, 0);
        fn_00d9c520(e, Ptr::new(tree), root, 0);
        assert_eq!(calls_to(&s, ARRAY_RESERVE_MORE).len(), 1);
        let freed = calls_to(&s, FREE_ARRAY);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 256 * 4);
    }

    /// Four wide primitives (unit boxes around the given centres) in memory, the index array
    /// header, the centroid box, and four output areas. Returns
    /// (aabbs, index header, bounds, out1, out2, counts).
    fn wide_partition_inputs(s: &mut Setup, centres: &[[f32; 3]], order: &[u32]) -> [u32; 6] {
        let e = &mut s.e;
        let aabbs = e.mem.alloc(0x20 * centres.len() as u32);
        let mut low = [f32::MAX; 3];
        let mut high = [-f32::MAX; 3];
        for (i, centre) in centres.iter().enumerate() {
            for (lane, value) in centre.iter().enumerate() {
                e.mem
                    .set_f32(aabbs + 0x20 * i as u32 + 4 * lane as u32, value - 0.5);
                e.mem.set_f32(
                    aabbs + 0x20 * i as u32 + 0x10 + 4 * lane as u32,
                    value + 0.5,
                );
                low[lane] = low[lane].min(*value);
                high[lane] = high[lane].max(*value);
            }
        }
        let indices = e.mem.alloc(4 * order.len() as u32);
        for (i, index) in order.iter().enumerate() {
            e.mem.set_u32(indices + 4 * i as u32, *index);
        }
        let header = e.mem.alloc(4);
        e.mem.set_u32(header, indices);
        let bounds = e.mem.alloc(0x20);
        for lane in 0..3u32 {
            e.mem.set_f32(bounds + 4 * lane, low[lane as usize]);
            e.mem.set_f32(bounds + 0x10 + 4 * lane, high[lane as usize]);
        }
        let out1 = e.mem.alloc(0x20);
        let out2 = e.mem.alloc(0x20);
        let counts = e.mem.alloc(8);
        [aabbs, header, bounds, out1, out2, counts]
    }

    fn run_wide_partition(
        s: &mut Setup,
        centres: &[[f32; 3]],
        order: &[u32],
    ) -> ([u32; 6], i32, i32) {
        let [aabbs, header, bounds, out1, out2, counts] = wide_partition_inputs(s, centres, order);
        fn_00d9a520(
            &mut s.e,
            Ptr::new(bounds),
            Ptr::new(aabbs),
            Ptr::new(header),
            0,
            centres.len() as i32,
            Ptr::new(out1),
            Ptr::new(out2),
            Ptr::new(counts),
            Ptr::new(counts + 4),
        );
        let first = s.e.mem.i32(counts);
        let second = s.e.mem.i32(counts + 4);
        ([aabbs, header, bounds, out1, out2, counts], first, second)
    }

    fn index_order(e: &Engine, header: u32, count: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..count).map(|i| e.mem.u32(data + 4 * i)).collect()
    }

    #[test]
    fn wide_partition_splits_about_the_middle_of_the_x_axis() {
        let mut s = setup();
        let centres = [
            [0.5, 0.5, 0.5],
            [1.5, 0.5, 0.5],
            [10.5, 0.5, 0.5],
            [11.5, 0.5, 0.5],
        ];
        let ([_, header, _, out1, out2, _], first, second) =
            run_wide_partition(&mut s, &centres, &[0, 1, 2, 3]);
        assert_eq!((first, second), (2, 2));
        assert_eq!(index_order(&s.e, header, 4), vec![0, 1, 2, 3]);
        assert_eq!(lanes(&s.e, out1), [0.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out1 + 0x10), [1.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out2), [10.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out2 + 0x10), [11.5, 0.5, 0.5]);
    }

    #[test]
    fn wide_partition_swaps_misplaced_primitives() {
        let mut s = setup();
        let centres = [
            [0.5, 0.5, 0.5],
            [1.5, 0.5, 0.5],
            [10.5, 0.5, 0.5],
            [11.5, 0.5, 0.5],
        ];
        let ([_, header, _, out1, out2, _], first, second) =
            run_wide_partition(&mut s, &centres, &[2, 1, 0, 3]);
        assert_eq!((first, second), (2, 2));
        assert_eq!(index_order(&s.e, header, 4), vec![0, 1, 2, 3]);
        assert_eq!(lanes(&s.e, out1), [0.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out1 + 0x10), [1.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out2), [10.5, 0.5, 0.5]);
        assert_eq!(lanes(&s.e, out2 + 0x10), [11.5, 0.5, 0.5]);
    }

    #[test]
    fn wide_partition_picks_the_longest_axis() {
        let mut s = setup();
        let on_y = [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 10.0, 0.0],
            [0.0, 11.0, 0.0],
        ];
        let (_, first, second) = run_wide_partition(&mut s, &on_y, &[3, 2, 1, 0]);
        assert_eq!((first, second), (2, 2));
        let on_z = [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 2.0],
            [0.0, 0.0, 30.0],
        ];
        let (parts, first, second) = run_wide_partition(&mut s, &on_z, &[0, 1, 2, 3]);
        assert_eq!((first, second), (3, 1));
        assert_eq!(lanes(&s.e, parts[4] + 0x10), [0.0, 0.0, 30.0]);
        // Equal extents on x and y with a longer z: also z.
        let on_x_then_z = [
            [0.0, 0.0, 0.0],
            [5.0, 5.0, 0.0],
            [5.0, 5.0, 100.0],
            [0.0, 0.0, 99.0],
        ];
        let (_, first, second) = run_wide_partition(&mut s, &on_x_then_z, &[0, 1, 2, 3]);
        assert_eq!((first, second), (2, 2));
    }

    #[test]
    fn wide_partition_of_a_degenerate_box_cuts_by_count() {
        let mut s = setup();
        let centres = [[2.0, 2.0, 2.0]; 5];
        let ([_, _, bounds, out1, out2, _], first, second) =
            run_wide_partition(&mut s, &centres, &[0, 1, 2, 3, 4]);
        assert_eq!((first, second), (2, 3));
        for out in [out1, out2] {
            assert_eq!(lanes(&s.e, out), lanes(&s.e, bounds));
            assert_eq!(lanes(&s.e, out + 0x10), lanes(&s.e, bounds + 0x10));
        }
    }

    /// Four narrow primitives (word boxes one unit each side of the centre) and the rest as
    /// for the wide inputs.
    fn narrow_partition_inputs(s: &mut Setup, centres: &[[u16; 3]], order: &[u16]) -> [u32; 6] {
        let e = &mut s.e;
        let aabbs = e.mem.alloc(0xc * centres.len() as u32);
        let mut low = [f32::MAX; 3];
        let mut high = [-f32::MAX; 3];
        for (i, centre) in centres.iter().enumerate() {
            for (lane, value) in centre.iter().enumerate() {
                e.mem
                    .set_u16(aabbs + 0xc * i as u32 + 2 * lane as u32, value + 1);
                e.mem
                    .set_u16(aabbs + 0xc * i as u32 + 6 + 2 * lane as u32, value - 1);
                low[lane] = low[lane].min(f32::from(*value));
                high[lane] = high[lane].max(f32::from(*value));
            }
        }
        let indices = e.mem.alloc(2 * order.len() as u32);
        for (i, index) in order.iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *index);
        }
        let header = e.mem.alloc(4);
        e.mem.set_u32(header, indices);
        let bounds = e.mem.alloc(0x20);
        for lane in 0..3u32 {
            e.mem.set_f32(bounds + 4 * lane, low[lane as usize]);
            e.mem.set_f32(bounds + 0x10 + 4 * lane, high[lane as usize]);
        }
        let out1 = e.mem.alloc(0x20);
        let out2 = e.mem.alloc(0x20);
        let counts = e.mem.alloc(8);
        [aabbs, header, bounds, out1, out2, counts]
    }

    #[test]
    fn narrow_partition_uses_the_low_word_of_start_and_swaps() {
        let mut s = setup();
        let centres = [[1, 1, 1], [3, 1, 1], [21, 1, 1], [23, 1, 1]];
        let [aabbs, header, bounds, out1, out2, counts] =
            narrow_partition_inputs(&mut s, &centres, &[2, 1, 0, 3]);
        fn_00d9a850(
            &mut s.e,
            Ptr::new(bounds),
            Ptr::new(aabbs),
            Ptr::new(header),
            0xabcd_0000,
            4,
            Ptr::new(out1),
            Ptr::new(out2),
            Ptr::new(counts),
            Ptr::new(counts + 4),
        );
        assert_eq!((s.e.mem.i32(counts), s.e.mem.i32(counts + 4)), (2, 2));
        let data = s.e.mem.u32(header);
        let order: Vec<u16> = (0..4).map(|i| s.e.mem.u16(data + 2 * i)).collect();
        assert_eq!(order, vec![0, 1, 2, 3]);
        assert_eq!(lanes(&s.e, out1), [1.0, 1.0, 1.0]);
        assert_eq!(lanes(&s.e, out1 + 0x10), [3.0, 1.0, 1.0]);
        assert_eq!(lanes(&s.e, out2), [21.0, 1.0, 1.0]);
        assert_eq!(lanes(&s.e, out2 + 0x10), [23.0, 1.0, 1.0]);
    }

    #[test]
    fn narrow_partition_of_a_degenerate_box_cuts_by_count() {
        let mut s = setup();
        let centres = [[4, 4, 4]; 5];
        let [aabbs, header, bounds, out1, out2, counts] =
            narrow_partition_inputs(&mut s, &centres, &[0, 1, 2, 3, 4]);
        fn_00d9a850(
            &mut s.e,
            Ptr::new(bounds),
            Ptr::new(aabbs),
            Ptr::new(header),
            1,
            3,
            Ptr::new(out1),
            Ptr::new(out2),
            Ptr::new(counts),
            Ptr::new(counts + 4),
        );
        assert_eq!((s.e.mem.i32(counts), s.e.mem.i32(counts + 4)), (1, 2));
        assert_eq!(lanes(&s.e, out2), lanes(&s.e, bounds));
        assert_eq!(lanes(&s.e, out1 + 0x10), lanes(&s.e, bounds + 0x10));
    }

    /// Three unit boxes in the wide layout (centres 0.5, 10.5, 11.5 on x) and their user data.
    fn wide_primitives(s: &mut Setup) -> (u32, u32) {
        let e = &mut s.e;
        let aabbs = e.mem.alloc(0x60);
        for (i, centre) in [0.5f32, 10.5, 11.5].into_iter().enumerate() {
            let at = aabbs + 0x20 * i as u32;
            for (lane, (low, high)) in [(centre - 0.5, centre + 0.5), (0.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .enumerate()
            {
                e.mem.set_f32(at + 4 * lane as u32, low);
                e.mem.set_f32(at + 0x10 + 4 * lane as u32, high);
            }
        }
        let userdata = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_u32(userdata + 4 * i, 0xa0 + i);
        }
        (aabbs, userdata)
    }

    #[test]
    fn wide_builder_with_no_primitives_only_stores_the_count() {
        let mut s = setup();
        let tree = new_wide(&mut s, 2);
        let out = s.e.mem.alloc(0x10);
        let calls_before = s.log.borrow().len();
        hk_aabb_tree_aabb_uint_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(0),
            Ptr::new(0),
            0,
            Ptr::new(out),
        );
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        assert_eq!(s.log.borrow().len(), calls_before);
    }

    #[test]
    fn wide_builder_with_one_primitive_fills_the_root_leaf() {
        let mut s = setup();
        let tree = new_wide_empty(&mut s);
        let (aabbs, userdata) = wide_primitives(&mut s);
        let out = s.e.mem.alloc(0x10);
        hk_aabb_tree_aabb_uint_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(aabbs),
            Ptr::new(userdata),
            1,
            Ptr::new(out),
        );
        assert_eq!(s.e.mem.u32(tree + 0x18), 1);
        assert_eq!(s.e.mem.u32(out + 4), 1);
        let data = s.e.mem.u32(out);
        assert_eq!(s.e.mem.u32(data), 0);
        let root = wide_at(&s.e, tree, 0);
        assert_eq!(s.e.mem.u32(tree + 0x10), root);
        assert_eq!(s.e.mem.u32(root + 0x20), 0xa0);
        assert_eq!(s.e.mem.u32(root + 0x28), LEAF_WIDE | NONE_WIDE);
        assert_eq!(s.e.mem.u32(root + 0x2c), NONE_WIDE);
        assert_eq!(lanes(&s.e, root), [0.0, 0.0, 0.0]);
        assert_eq!(lanes(&s.e, root + 0x10), [1.0, 1.0, 1.0]);
        assert_eq!(
            calls_to(&s, FIX_UP_WIDE),
            vec![vec![tree, root, 0, NONE_WIDE]]
        );
        // The temporary box array (one 0x20-byte entry) is released.
        assert_eq!(
            calls_to(&s, FREE_ARRAY).last().map(|call| call[2]),
            Some(0x20)
        );
    }

    #[test]
    fn wide_builder_splits_three_primitives_into_a_two_level_tree() {
        let mut s = setup();
        let tree = new_wide_empty(&mut s);
        let (aabbs, userdata) = wide_primitives(&mut s);
        let out = s.e.mem.alloc(0x10);
        hk_aabb_tree_aabb_uint_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(aabbs),
            Ptr::new(userdata),
            3,
            Ptr::new(out),
        );
        let data = s.e.mem.u32(out);
        let placed: Vec<u32> = (0..3).map(|i| s.e.mem.u32(data + 4 * i)).collect();
        assert_eq!(placed, vec![1, 3, 4]);
        let root = wide_at(&s.e, tree, 0);
        assert_eq!(s.e.mem.u32(root + 0x20), 1);
        assert_eq!(s.e.mem.u32(root + 0x24), 2);
        let inner = wide_at(&s.e, tree, 2);
        assert_eq!(s.e.mem.u32(inner + 0x20), 3);
        assert_eq!(s.e.mem.u32(inner + 0x24), 4);
        for (primitive, node) in [(0u32, 1u32), (1, 3), (2, 4)] {
            let leaf = wide_at(&s.e, tree, node);
            assert_eq!(s.e.mem.u32(leaf + 0x20), 0xa0 + primitive);
            assert_eq!(s.e.mem.u32(leaf + 0x28) & LEAF_WIDE, LEAF_WIDE);
            assert_eq!(lanes(&s.e, leaf), lanes(&s.e, aabbs + 0x20 * primitive));
        }
        assert_eq!(s.e.mem.u32(tree + 0x18), 3);
        assert_eq!(
            calls_to(&s, FIX_UP_WIDE),
            vec![vec![tree, root, 0, NONE_WIDE]]
        );
        let sizes: Vec<u32> = calls_to(&s, FREE_ARRAY)
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(sizes, vec![12, 0x60]);
    }

    #[test]
    fn wide_core_can_reuse_the_nodes_named_in_the_output_array() {
        let mut s = setup();
        let tree = new_wide(&mut s, 8);
        let (aabbs, userdata) = wide_primitives(&mut s);
        put_wide(
            &mut s.e,
            tree,
            0,
            span(0.0, 1.0),
            (0, 0),
            NONE_WIDE,
            NONE_WIDE,
        );
        let header = s.e.mem.alloc(0x10);
        let data = s.e.mem.alloc(8);
        s.e.mem.set_u32(header, data);
        s.e.mem.set_u32(data, 5);
        s.e.mem.set_u32(data + 4, 6);
        let free_before = s.e.mem.u32(tree + 0x1c + 0x10);
        let root = wide_at(&s.e, tree, 0);
        fn_00d9d1c0(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(aabbs),
            Ptr::new(userdata),
            2,
            root,
            Ptr::new(header),
            true,
        );
        assert_eq!(s.e.mem.u32(root + 0x20), 5);
        assert_eq!(s.e.mem.u32(root + 0x24), 6);
        let second = wide_at(&s.e, tree, 6);
        assert_eq!(s.e.mem.u32(second + 0x20), 0xa1);
        assert_eq!(s.e.mem.u32(second + 0x28) & LEAF_WIDE, LEAF_WIDE);
        assert_eq!(s.e.mem.u32(tree + 0x1c + 0x10), free_before);
    }

    /// Three narrow primitives from float boxes (x spans 1..2, 20..21, 22..23 grid cells).
    fn narrow_float_primitives(s: &mut Setup) -> (u32, u32) {
        let e = &mut s.e;
        let aabbs = e.mem.alloc(0x60);
        for (i, cell) in [1.0f32, 20.0, 22.0].into_iter().enumerate() {
            let at = aabbs + 0x20 * i as u32;
            for (lane, (low, high)) in [
                (cell * 32768.0, (cell + 1.0) * 32768.0),
                (0.0, 32768.0),
                (0.0, 32768.0),
            ]
            .into_iter()
            .enumerate()
            {
                e.mem.set_f32(at + 4 * lane as u32, low);
                e.mem.set_f32(at + 0x10 + 4 * lane as u32, high);
            }
        }
        let userdata = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_u32(userdata + 4 * i, 0xa0 + i);
        }
        (aabbs, userdata)
    }

    #[test]
    fn narrow_builder_with_no_primitives_only_stores_the_count() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 2);
        let out = s.e.mem.alloc(0x10);
        let calls_before = s.log.borrow().len();
        hk_aabb_tree_aabb16_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(0),
            Ptr::new(0),
            0,
            Ptr::new(out),
        );
        assert_eq!(s.e.mem.u32(tree + 0x18), 0);
        assert_eq!(s.log.borrow().len(), calls_before);
    }

    #[test]
    fn narrow_builder_with_one_primitive_fills_the_root_leaf() {
        let mut s = setup();
        let tree = new_narrow_empty(&mut s);
        let (aabbs, userdata) = narrow_float_primitives(&mut s);
        let out = s.e.mem.alloc(0x10);
        hk_aabb_tree_aabb16_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(aabbs),
            Ptr::new(userdata),
            1,
            Ptr::new(out),
        );
        assert_eq!(s.e.mem.u32(tree + 0x18), 1);
        assert_eq!(s.e.mem.u32(out + 4), 1);
        let data = s.e.mem.u32(out);
        assert_eq!(s.e.mem.u16(data), 0);
        let root = narrow_at(&s.e, tree, 0);
        assert_eq!(s.e.mem.u32(tree + 0x10), root);
        assert_eq!(words(&s.e, root), [2, 1, 1, 1, 0, 0]);
        assert_eq!(s.e.mem.u32(root + 0xc), 0xa0);
        assert_eq!(s.e.mem.u16(root + 0x10), 0xffff);
        assert_eq!(s.e.mem.u16(root + 0x12), NONE_NARROW);
        assert_eq!(
            calls_to(&s, FIX_UP_NARROW),
            vec![vec![tree, root, 0, u32::from(NONE_NARROW)]]
        );
        assert_eq!(
            calls_to(&s, FREE_ARRAY).last().map(|call| call[2]),
            Some(0xc)
        );
    }

    #[test]
    fn narrow_builder_splits_three_primitives_into_a_two_level_tree() {
        let mut s = setup();
        // A stand-in for the fix-up, which sets the link words of the tree: the walk that follows
        // reads the link of each first child (its next sibling); left at 0 they would point at
        // the root and the walk would never end.
        let fix_up_log = Rc::clone(&s.log);
        s.e.register_double(FIX_UP_NARROW, move |e, a| {
            fix_up_log.borrow_mut().push((FIX_UP_NARROW, a.to_vec()));
            let data = e.mem.u32(a[0] + 0x1c);
            e.mem.set_u16(data + 0x14 + 0x12, 2);
            e.mem.set_u16(data + 3 * 0x14 + 0x12, 4);
            Ret::default()
        });
        let tree = new_narrow_empty(&mut s);
        let (aabbs, userdata) = narrow_float_primitives(&mut s);
        let out = s.e.mem.alloc(0x10);
        hk_aabb_tree_aabb16_build(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(aabbs),
            Ptr::new(userdata),
            3,
            Ptr::new(out),
        );
        let data = s.e.mem.u32(out);
        let placed: Vec<u16> = (0..3).map(|i| s.e.mem.u16(data + 2 * i)).collect();
        assert_eq!(placed, vec![1, 3, 4]);
        let root = narrow_at(&s.e, tree, 0);
        assert_eq!(s.e.mem.u16(root + 0xc), 1);
        assert_eq!(s.e.mem.u16(root + 0xe), 2);
        let inner = narrow_at(&s.e, tree, 2);
        assert_eq!(s.e.mem.u16(inner + 0xc), 3);
        assert_eq!(s.e.mem.u16(inner + 0xe), 4);
        let want = [
            [2u16, 1, 1, 1, 0, 0],
            [21, 1, 1, 20, 0, 0],
            [23, 1, 1, 22, 0, 0],
        ];
        for (primitive, node) in [(0usize, 1u32), (1, 3), (2, 4)] {
            let leaf = narrow_at(&s.e, tree, node);
            assert_eq!(words(&s.e, leaf), want[primitive]);
            assert_eq!(s.e.mem.u32(leaf + 0xc), 0xa0 + primitive as u32);
            assert_eq!(s.e.mem.u16(leaf + 0x10) & LEAF_NARROW, LEAF_NARROW);
        }
        assert_eq!(
            calls_to(&s, FIX_UP_NARROW),
            vec![vec![tree, root, 0, u32::from(NONE_NARROW)]]
        );
        let sizes: Vec<u32> = calls_to(&s, FREE_ARRAY)
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(sizes, vec![6, 0x24]);
    }

    #[test]
    fn narrow_core_can_reuse_the_nodes_named_in_the_output_array() {
        let mut s = setup();
        let tree = new_narrow(&mut s, 8);
        let (_, userdata) = narrow_float_primitives(&mut s);
        // The core takes word boxes: convert two primitives by hand.
        let boxes = s.e.mem.alloc(0x18);
        for (i, source) in [[2u16, 1, 1, 1, 0, 0], [21, 1, 1, 20, 0, 0]]
            .iter()
            .enumerate()
        {
            for (word, value) in source.iter().enumerate() {
                s.e.mem
                    .set_u16(boxes + 0xc * i as u32 + 2 * word as u32, *value);
            }
        }
        put_narrow(&mut s.e, tree, 0, [0; 6], (0, 0), NONE_NARROW, NONE_NARROW);
        let header = s.e.mem.alloc(0x10);
        let data = s.e.mem.alloc(4);
        s.e.mem.set_u32(header, data);
        s.e.mem.set_u16(data, 5);
        s.e.mem.set_u16(data + 2, 6);
        let free_before = s.e.mem.u32(tree + 0x1c + 0x10);
        let root = narrow_at(&s.e, tree, 0);
        fn_00d9c840(
            &mut s.e,
            Ptr::new(tree),
            Ptr::new(boxes),
            Ptr::new(userdata),
            2,
            root,
            Ptr::new(header),
            true,
        );
        assert_eq!(s.e.mem.u16(root + 0xc), 5);
        assert_eq!(s.e.mem.u16(root + 0xe), 6);
        let second = narrow_at(&s.e, tree, 6);
        assert_eq!(words(&s.e, second), [21, 1, 1, 20, 0, 0]);
        assert_eq!(s.e.mem.u32(second + 0xc), 0xa1);
        assert_eq!(s.e.mem.u16(second + 0x10) & LEAF_NARROW, LEAF_NARROW);
        assert_eq!(s.e.mem.u32(tree + 0x1c + 0x10), free_before);
    }
}
