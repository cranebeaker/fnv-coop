//! The searches: node keys and costs, the navmesh search's goal test, and the arrays, queues and
//! slots the A* searches keep their nodes in.

use super::layout::{
    AStarQueue, AStarSearch, Counter, InfoNode, InfoSearch, MeshEdge, NodeSlots, PathingRequest,
    Scale, SearchLocation, SimpleArray, SmootherNode, SortNode,
};
use super::point::{length_xy_squared, subtract};
use super::{
    fcom, fld, fld_global, fld_local, fstp, low_word, m32, m64_global, parity_even, FAMILY,
};
use crate::culling::layout::NiPoint3;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{fistp_chop64, x87_add, x87_div, x87_mul};
use engine_core::{args, typed_call, Engine};

/// The `double` 19 the scaled cost multiplies by (`010356e8`).
const NINETEEN: u32 = 0x0103_56e8;
/// The `double` 100 the request's radius has to pass (`01017a40`).
const HUNDRED: u32 = 0x0101_7a40;
/// The `double` 0.25 (`010290b0`).
const QUARTER: u32 = 0x0102_90b0;
/// The float -1 a new queue's best cost starts at (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The scrap heap's alignment word (`010a2720`).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;

/// Callees.
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const NODE_DATA: u32 = 0x0041_3f40;
const REQUEST_RADIUS: u32 = 0x009b_0bd0;
const LIST_ELEMENT: u32 = 0x005e_eda0;
const IS_FULL: u32 = 0x0043_8b90;
const GROWN_SIZE: u32 = 0x009a_3910;
const MOVE_INDICES: u32 = 0x005e_f6a0;
const MOVE_SLOTS: u32 = 0x006f_9eb0;
const CONSTRUCT_VERTICES: u32 = 0x0072_6bf0;
const CONSTRUCT_INDICES: u32 = 0x005e_f650;
const COPY_BLOCK: u32 = 0x0042_f5d0;
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const EDGE_ARRAY_INIT: u32 = 0x0069_ba70;
const PLACEMENT_NEW: u32 = 0x006e_6da0;
const SMOOTHER_EDGE_NEW: u32 = 0x006a_eb70;
const MEMSET: u32 = 0x0040_3d30;
const OPEN_LIST_NEW: u32 = 0x006a_8380;
const NEXT_EDGE_SLOT: u32 = 0x0097_8bc0;
const CONSTRUCT_EDGES: u32 = 0x0069_b9d0;
const EDGE_ASSIGN: u32 = 0x006d_bbb0;
const SOLUTION_NODE: u32 = 0x006e_8f40;
const TAG_INIT: u32 = 0x0044_e010;

/// The vtables the arrays' constructors and destructors install.
const INDEX_ARRAY_VTABLE: u32 = 0x0106_abe4;
const EDGE_ARRAY_VTABLE: u32 = 0x0106_b1e0;
const SEARCH_VTABLE: u32 = 0x0106_b650;
/// An array's vtable slots: allocate a buffer, free one.
const SLOT_ALLOCATE: u32 = 0x4;
const SLOT_FREE: u32 = 0x8;

// Typed from 006b9710
/// `SortNode::GetKey` (`thiscall(node)`): cost plus estimate, stored to a float.
pub fn sort_key(e: &mut Engine, this: Ptr<SortNode>) -> f32 {
    fstp(x87_add(
        fld(e, this, SortNode::cost),
        m32(e, this, SortNode::estimate),
    ))
}

// Typed from 006b9130
/// The second float (`thiscall(this)`), as `FLD` leaves it.
pub fn second_float(e: &mut Engine, this: Ptr<SortNode>) -> f64 {
    fld(e, this, SortNode::estimate)
}

// Typed from 0069ef80
/// `PathingRequest::GetRadius` (`thiscall(request)`), as `FLD` leaves it.
pub fn request_radius(e: &mut Engine, this: Ptr) -> f64 {
    fld(e, this.cast::<PathingRequest>(), PathingRequest::radius)
}

// Typed from 006a7f50
/// The first float (`thiscall(this)`), as `FLD` leaves it.
pub fn first_float(e: &mut Engine, this: Ptr<SortNode>) -> f64 {
    fld(e, this, SortNode::cost)
}

// Typed from 006b81d0
/// Whether two keys are equal (`thiscall(map, a, b)`).
pub fn keys_equal(_e: &mut Engine, _this: Ptr, a: u32, b: u32) -> u8 {
    u8::from(a == b)
}

// Typed from 00691e50
/// Whether two 16-bit keys are equal (`thiscall(map, a, b)`).
pub fn keys_equal_16(_e: &mut Engine, _this: Ptr, a: u16, b: u16) -> bool {
    a == b
}

// Typed from 006c6a60
/// `NiTMapBase::KeyToHashIndex` (`thiscall(map, key)`): `key % buckets` (`DIV`; none faults).
pub fn hash(e: &mut Engine, this: Ptr<Counter>, key: u32) -> u32 {
    key % e.get(this, Counter::buckets)
}

// Typed from 0049c4c0
/// The same for a 16-bit key (`thiscall(map, key)`).
pub fn hash_16(e: &mut Engine, this: Ptr<Counter>, key: u16) -> u32 {
    u32::from(key) % e.get(this, Counter::buckets)
}

// Typed from 006a94f0
/// A search node's key (`thiscall(search, mesh, triangle)`): the mesh's form id (`0084e3a0`) in the
/// high half, the triangle in the low.
pub fn node_key(e: &mut Engine, _this: Ptr, mesh: Ptr, index: u16) -> u32 {
    let id = typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, mesh));
    ((id & 0xffff) << 16) | u32::from(index)
}

// Typed from 0069e390
/// The first word of the object at +0x94 (`thiscall(this)`, `00559450`).
pub fn first_word_at_94(e: &mut Engine, this: Ptr) -> u32 {
    typed_call!(
        leaf::FAMILY,
        0x0055_9450,
        leaf::word_at_0(e, this.byte_add(0x94))
    )
}

// Typed from 006f4790
/// A node's cost scaled (`thiscall(scale, node)`): 0x13 when the scale is at most the node's key
/// ([`sort_key`]); else `key * 19 / scale` truncated to a 64-bit integer, its low word.
pub fn scaled_cost(e: &mut Engine, this: Ptr, node: Ptr<SortNode>) -> u32 {
    let this = this.cast::<Scale>();
    let key = typed_call!(FAMILY, 0x006b_9710, sort_key(e, node));
    // FLD key; FLD scale; FCOMPP; TEST AH,0x41; JP: on when the scale is above or unordered.
    if !parity_even(fcom(fld(e, this, Scale::value), fld_local(key)), 0x41) {
        return 0x13;
    }
    let v = x87_div(
        x87_mul(fld_local(key), m64_global(e, NINETEEN)),
        m32(e, this, Scale::value),
    );
    low_word(fistp_chop64(v))
}

// Typed from 006a1f90
/// `operator new` on a scrap heap (`cdecl(size, heap)`): `00aa54a0` with the heap's alignment.
pub fn scrap_new(e: &mut Engine, size: u32, heap: Ptr) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    e.call(SCRAP_ALLOCATE, &args![heap, size, alignment]).u32()
}

// Typed from 006a79c0
/// `NavMeshSearch::IsGoalNode` (Xbox PDB, `thiscall(search, node)`): with a request whose radius is
/// above 100, a node whose centre is within half the request's radius of the goal's in x and y
/// (squared against `r * (r * 0.25)`, `009b0bd0` read twice) and farther in x and y than in z; else the
/// node on the goal's mesh and triangle.
pub fn is_goal(e: &mut Engine, this: Ptr<AStarSearch>, node: Ptr) -> bool {
    let request = e.get(this, AStarSearch::request);
    if request != 0 {
        let r = typed_call!(FAMILY, 0x0069_ef80, request_radius(e, Ptr::new(request)));
        if fcom(r, m64_global(e, HUNDRED)) & 0x41 == 0 {
            let near = e.with_stack(0x24, |e, locals| {
                let node_center = locals.cast::<NiPoint3>();
                let goal_center = locals.byte_add(0xc).cast::<NiPoint3>();
                let difference = locals.byte_add(0x18).cast::<NiPoint3>();
                let data = e.call(NODE_DATA, &args![node]).u32();
                let c1 = typed_call!(
                    FAMILY,
                    0x006a_6eb0,
                    location_center(e, Ptr::new(data), node_center)
                );
                let goal = this.at(AStarSearch::goal);
                let c2 = typed_call!(FAMILY, 0x006a_6eb0, location_center(e, goal, goal_center));
                typed_call!(FAMILY, 0x0043_9ef0, subtract(e, c2, difference, c1));
                let xy = typed_call!(FAMILY, 0x0059_5c80, length_xy_squared(e, difference));
                let xy = fstp(fld_local(xy));
                let zz = fstp(x87_mul(
                    fld(e, difference, NiPoint3::z),
                    m32(e, difference, NiPoint3::z),
                ));
                let t = e.call(REQUEST_RADIUS, &args![request]).f64();
                let quarter = x87_mul(t, m64_global(e, QUARTER));
                let t2 = e.call(REQUEST_RADIUS, &args![request]).f64();
                let limit = x87_mul(t2, quarter);
                // FLD xy; FCOMPP against the limit: on when below.
                if parity_even(fcom(fld_local(xy), limit), 0x05) {
                    return false;
                }
                // FLD zz; FLD xy; FCOMPP: on when xy is above zz.
                fcom(fld_local(xy), fld_local(zz)) & 0x41 == 0
            });
            if near {
                return true;
            }
        }
    }
    let data = Ptr::<SearchLocation>::new(e.call(NODE_DATA, &args![node]).u32());
    if e.get(data, SearchLocation::mesh) != e.get(this.at(AStarSearch::goal), SearchLocation::mesh)
    {
        return false;
    }
    let data = Ptr::<SearchLocation>::new(e.call(NODE_DATA, &args![node]).u32());
    e.get(data, SearchLocation::triangle)
        == e.get(this.at(AStarSearch::goal), SearchLocation::triangle)
}

// Typed from 006b8c20
/// `NavMeshInfoSearch::IsGoalNode` (`thiscall(search, node)`): the node's info is the goal.
pub fn info_is_goal(e: &mut Engine, this: Ptr, node: Ptr) -> u32 {
    let goal = e.get(this.cast::<InfoSearch>(), InfoSearch::goal);
    u32::from(e.get(node.cast::<InfoNode>(), InfoNode::info) == goal)
}

// Typed from 006a6eb0
/// A search location's point as a new point (`thiscall(location, out)`), each float passed on.
pub fn location_center(
    e: &mut Engine,
    this: Ptr<SearchLocation>,
    out: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let p = this.at(SearchLocation::point);
    let z = fstp(fld(e, p, NiPoint3::z));
    let y = fstp(fld(e, p, NiPoint3::y));
    let x = fstp(fld(e, p, NiPoint3::x));
    typed_call!(leaf::FAMILY, 0x0041_6870, leaf::point3_new(e, out, x, y, z));
    out
}

// Typed from 006a7ad0
/// `BSSimpleArray::operator[]` (`thiscall(array, index)`, `00877a30`).
pub fn element(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    typed_call!(
        leaf::FAMILY,
        0x0087_7a30,
        leaf::array_element(e, this, index)
    )
}

// Typed from 0069dec0
/// Element `index` of a triangle list (`thiscall(list, index)`, `005eeda0`).
pub fn list_element(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(LIST_ELEMENT, &args![this, index]).u32()
}

// Typed from 006950d0
/// Entry `index` of an array of 8-byte entries (`thiscall(array, index)`), unchecked.
pub fn entry_8(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> u32 {
    e.get(this, SimpleArray::data)
        .wrapping_add(index.wrapping_mul(8))
}

// Typed from 006b3160
/// Entry `index` of an array of 0x14-byte entries (`thiscall(array, index)`), unchecked.
pub fn entry_14(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> u32 {
    index
        .wrapping_mul(0x14)
        .wrapping_add(e.get(this, SimpleArray::data))
}

// Typed from 006dad90
/// The same entry through [`entry_14`] (`thiscall(array, index)`).
pub fn entry_14_of(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> u32 {
    typed_call!(FAMILY, 0x006b_3160, entry_14(e, this, index))
}

// Typed from 006a8500
/// Frees an array's buffer (`thiscall(array)`): the vtable's free slot, then a null buffer.
pub fn free_data(e: &mut Engine, this: Ptr<SimpleArray>) {
    let data = e.get(this, SimpleArray::data);
    e.vcall(this.addr(), SLOT_FREE, &args![data]);
    e.set(this, SimpleArray::data, 0);
}

/// `BSSimpleArray::BSSimpleArray(capacity, size)` (Xbox PDB): all three words zero; the capacity
/// raised to the size; a buffer of the capacity from the vtable's allocate slot; `size` elements
/// constructed in it.
fn init_array(
    e: &mut Engine,
    this: Ptr<SimpleArray>,
    capacity: u32,
    size: u32,
    make_elements: impl FnOnce(&mut Engine, u32, u32),
) {
    e.set(this, SimpleArray::data, 0);
    e.set(this, SimpleArray::size, 0);
    e.set(this, SimpleArray::capacity, 0);
    let capacity = capacity.max(size);
    if capacity > 0 {
        let data = e.vcall(this.addr(), SLOT_ALLOCATE, &args![capacity]).u32();
        e.set(this, SimpleArray::data, data);
        e.set(this, SimpleArray::capacity, capacity);
    }
    if size > 0 {
        let data = e.get(this, SimpleArray::data);
        make_elements(e, data, size);
        e.set(this, SimpleArray::size, size);
    }
}

// Typed from 006b3eb0
/// `BSSimpleArray::BSSimpleArray` of the vertex arrays (`thiscall(array, capacity, size)`), the
/// elements made by `00726bf0`.
pub fn init_vertices(e: &mut Engine, this: Ptr<SimpleArray>, capacity: u32, size: u32) {
    init_array(e, this, capacity, size, |e, data, size| {
        e.call(CONSTRUCT_VERTICES, &args![this, data, size]);
    });
}

// Typed from 006b3f40
/// The same for the node arrays, the elements made by [`construct_12s`].
pub fn init_nodes(e: &mut Engine, this: Ptr<SimpleArray>, capacity: u32, size: u32) {
    init_array(e, this, capacity, size, |e, data, size| {
        typed_call!(
            FAMILY,
            0x006d_03d0,
            construct_12s(e, this.cast(), data, size)
        );
    });
}

// Typed from 00692290
/// The same for the index arrays, the elements made by `005ef650`.
pub fn init_indices(e: &mut Engine, this: Ptr<SimpleArray>, capacity: u32, size: u32) {
    init_array(e, this, capacity, size, |e, data, size| {
        e.call(CONSTRUCT_INDICES, &args![this, data, size]);
    });
}

/// The next free index of an array, growing it when full (`00438b90`): an empty one gets four from
/// the allocate slot, else the grown size (`009a3910`) and a move (`grow`); the count is raised.
fn next_of(e: &mut Engine, this: Ptr<SimpleArray>, grow: u32) -> u32 {
    if e.call(IS_FULL, &args![this]).u8() != 0 {
        if e.get(this, SimpleArray::capacity) == 0 {
            let n = 4u32;
            let data = e.vcall(this.addr(), SLOT_ALLOCATE, &args![n]).u32();
            e.set(this, SimpleArray::data, data);
            e.set(this, SimpleArray::capacity, n);
        } else {
            let n = e.call(GROWN_SIZE, &args![this]).u32();
            let count = e.get(this, SimpleArray::size);
            e.call(grow, &args![this, n, count]);
            e.set(this, SimpleArray::capacity, n);
        }
    }
    let size = e.get(this, SimpleArray::size).wrapping_add(1);
    e.set(this, SimpleArray::size, size);
    e.get(this, SimpleArray::size).wrapping_sub(1)
}

// Typed from 00692200
/// The next index of an index array (`thiscall(array)`), grown by `005ef6a0`.
pub fn next_index(e: &mut Engine, this: Ptr<SimpleArray>) -> u32 {
    next_of(e, this, MOVE_INDICES)
}

// Typed from 006b4060
/// The next index of a slot array (`thiscall(array)`), grown by `006f9eb0`.
pub fn next_slot(e: &mut Engine, this: Ptr<SimpleArray>) -> u32 {
    next_of(e, this, MOVE_SLOTS)
}

// Typed from 00691820
/// Appends a 16-bit value (`thiscall(array, value)`): the next index ([`next_index`]), the element
/// constructed (`005ef650`), the value copied; the index.
pub fn push_index(e: &mut Engine, this: Ptr<SimpleArray>, value: Ptr) -> u32 {
    let i = typed_call!(FAMILY, 0x0069_2200, next_index(e, this));
    let slot = e
        .get(this, SimpleArray::data)
        .wrapping_add(i.wrapping_mul(2));
    e.call(CONSTRUCT_INDICES, &args![this, slot, 1u32]);
    let v = e.mem.u16(value.addr());
    let slot = e
        .get(this, SimpleArray::data)
        .wrapping_add(i.wrapping_mul(2));
    e.mem.set_u16(slot, v);
    i
}

// Typed from 006918a0
/// Copies `count` 16-bit values out of a block (`thiscall(array, block, count)`, `0042f5d0`).
pub fn copy_indices(e: &mut Engine, _this: Ptr<SimpleArray>, block: u32, count: u32) -> u32 {
    e.call(COPY_BLOCK, &args![block, count << 1]).u32()
}

// Typed from 00691870
/// A buffer of `count` 16-bit values (`thiscall(array, count)`, `00401000`).
pub fn allocate_indices(e: &mut Engine, _this: Ptr<SimpleArray>, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count << 1]).u32()
}

// Typed from 0069af80
/// A buffer of `count` 0xc-byte entries (`thiscall(array, count)`, `00401000`).
pub fn allocate_entries(e: &mut Engine, _this: Ptr<SimpleArray>, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(0xc)]).u32()
}

// Typed from 006db090
/// Frees a buffer (`thiscall(array, block)`, `00401030`).
pub fn free_block(e: &mut Engine, _this: Ptr<SimpleArray>, block: u32) {
    e.call(FREE, &args![block]);
}

// Typed from 006917d0
/// `BSSimpleArray<unsigned short>::BSSimpleArray` (`thiscall(array)`): the vtable, then empty
/// ([`init_indices`]).
pub fn index_array_new(e: &mut Engine, this: Ptr<SimpleArray>) -> Ptr<SimpleArray> {
    e.set(this, SimpleArray::vtable, INDEX_ARRAY_VTABLE);
    typed_call!(FAMILY, 0x0069_2290, init_indices(e, this, 0, 0));
    this
}

// Typed from 00691800
/// Its destructor (`thiscall(array)`): the vtable, then `008454f0` frees the elements.
pub fn index_array_drop(e: &mut Engine, this: Ptr<SimpleArray>) {
    e.set(this, SimpleArray::vtable, INDEX_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Typed from 0069b2c0
/// The edge array's constructor (`thiscall(array)`): the vtable, then empty (`0069ba70`).
pub fn edge_array_new(e: &mut Engine, this: Ptr<SimpleArray>) -> Ptr<SimpleArray> {
    e.set(this, SimpleArray::vtable, EDGE_ARRAY_VTABLE);
    e.call(EDGE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Typed from 0069b2f0
/// Its destructor (`thiscall(array)`).
pub fn edge_array_drop(e: &mut Engine, this: Ptr<SimpleArray>) {
    e.set(this, SimpleArray::vtable, EDGE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

/// `count` placements of `size` bytes from `elements` on (`006e6da0`).
fn construct_each(e: &mut Engine, elements: u32, count: u32, size: u32) {
    for i in 0..count {
        let at = i.wrapping_mul(size).wrapping_add(elements);
        e.call(PLACEMENT_NEW, &args![size, at]);
    }
}

// Typed from 006a8a30
/// Constructs `count` 0x14-byte elements (`thiscall(array, elements, count)`).
pub fn construct_20s(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    construct_each(e, elements, count, 0x14);
}

// Typed from 006d03d0
/// Constructs `count` 0xc-byte elements (`thiscall(array, elements, count)`).
pub fn construct_12s(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    construct_each(e, elements, count, 0xc);
}

// Typed from 006b40f0
/// Constructs `count` 0x20-byte path-smoother edges (`thiscall(array, elements, count)`): each
/// placement that isn't null runs the constructor `006aeb70`.
pub fn construct_32s(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    for i in 0..count {
        let at = (i << 5).wrapping_add(elements);
        let p = e.call(PLACEMENT_NEW, &args![0x20u32, at]).u32();
        if p != 0 {
            e.call(SMOOTHER_EDGE_NEW, &args![p]);
        }
    }
}

// Typed from 006b3de0
/// The node slots' constructor (`thiscall(slots)`): the count zero, the slots cleared (`memset`).
pub fn slots_new(e: &mut Engine, this: Ptr) -> Ptr {
    e.set(this.cast::<NodeSlots>(), NodeSlots::count, 0);
    e.call(MEMSET, &args![this, 0u32, 0x1fdcu32]);
    this
}

// Typed from 006b9470
/// `AStarQueue::AStarQueue` (`thiscall(queue)`): the best cost -1, the slots cleared.
pub fn queue_new(e: &mut Engine, this: Ptr<AStarQueue>) -> Ptr<AStarQueue> {
    let best = fstp(fld_global(e, MINUS_ONE));
    e.set(this, AStarQueue::best, best);
    e.call(MEMSET, &args![this.byte_add(4), 0u32, 0x50u32]);
    this
}

// Typed from 006a7b10
/// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>::AStarSearch` (Xbox PDB, `thiscall(search)`):
/// the vtable, the open list (`006a8380`), the slots, the queue, three words and a flag cleared.
pub fn search_new(e: &mut Engine, this: Ptr<AStarSearch>) -> Ptr<AStarSearch> {
    e.set(this, AStarSearch::vtable, SEARCH_VTABLE);
    e.call(OPEN_LIST_NEW, &args![this.byte_add(4)]);
    typed_call!(FAMILY, 0x006b_3de0, slots_new(e, this.byte_add(0x14)));
    typed_call!(
        FAMILY,
        0x006b_9470,
        queue_new(e, this.at(AStarSearch::queue))
    );
    e.set(this, AStarSearch::unknown_2048, 0);
    e.set(this, AStarSearch::unknown_204c, 0);
    e.set(this, AStarSearch::unknown_2050, 0);
    e.set(this, AStarSearch::done, 0);
    this
}

// Typed from 006c9900
/// The address of the object's part at +0x5c (`thiscall(this)`).
pub fn at_5c(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x5c)
}

// Typed from 006a9540
/// The address of the request's part at +0x34 (`thiscall(request)`).
pub fn at_34(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x34)
}

// Typed from 006b05d0
/// A path smoother node's flag byte (`thiscall(node)`).
pub fn byte_at_40(e: &mut Engine, this: Ptr) -> u8 {
    e.get(this.cast::<SmootherNode>(), SmootherNode::flag)
}

// Typed from 006d4ca0
/// Whether two references name the same mesh and triangle (`thiscall(a, b)`).
pub fn edges_equal(e: &mut Engine, this: Ptr<MeshEdge>, other: Ptr<MeshEdge>) -> bool {
    e.get(this, MeshEdge::mesh) == e.get(other, MeshEdge::mesh)
        && e.get(this, MeshEdge::triangle) == e.get(other, MeshEdge::triangle)
}

// Typed from 006db4f0
/// Appends an edge (`thiscall(array, edge)`): the next slot (`00978bc0`), constructed
/// (`0069b9d0`), assigned (`006dbbb0`); the index.
pub fn append_edge(e: &mut Engine, this: Ptr<SimpleArray>, item: u32) -> u32 {
    let i = e.call(NEXT_EDGE_SLOT, &args![this]).u32();
    let slot = i
        .wrapping_mul(0xc)
        .wrapping_add(e.get(this, SimpleArray::data));
    e.call(CONSTRUCT_EDGES, &args![this, slot, 1u32]);
    let slot = i
        .wrapping_mul(0xc)
        .wrapping_add(e.get(this, SimpleArray::data));
    e.call(EDGE_ASSIGN, &args![slot, item]);
    i
}

// Typed from 006e78b0
/// Node `index` of a pathing solution (`thiscall(solution, index)`): `006e8f40` on its array at +8.
pub fn solution_node(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(SOLUTION_NODE, &args![this.byte_add(8), index]).u32()
}

// Typed from 006ecba0
/// A tagged word's constructor (`thiscall(this, mode, value)`): the value, then `0044e010` with the
/// mode byte.
pub fn tagged_new(e: &mut Engine, this: Ptr, mode: u8, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    e.call(TAG_INIT, &args![this, u32::from(mode)]);
    this
}
