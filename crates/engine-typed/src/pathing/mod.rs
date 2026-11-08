//! The pathing family (F4 of the migration plan, stage 1): the code the AI threads run most. The
//! census (measure/typed/t3, `threads.tsv`) puts the navmesh search on the tasklet threads 2 to 7
//! (start `00b023f0`) and the AI threads 17 and 18, with a little on the main thread: the navmesh's
//! triangle, vertex and edge getters, the matching edge across meshes, the triangle under a point and
//! its height, the search's node keys, goal test and arrays, the point-of-view search's angles, arcs
//! and sphere tangents, the pathing locations, and the point helpers all of them call.
//!
//! Stage 1: the objects stay in guest memory where the game made them and are read through their
//! [`layout`]. Each function stays registered at its exe address; a call by address inside the family
//! is a `typed_call!`.
//!
//! **Floats.** Every function here runs on the main thread and on other threads, so the family's row
//! is [`Threads::Both`] and its x87 code keeps the exact helpers ([`engine_core::x87`]), as the
//! culling family does: every load, operation and store is the exe's, in its order, with a memory
//! operand kept a memory operand. A comparison reads the status word the exe's `FCOM` sets
//! ([`fcom`]) and branches as its `TEST AH,mask` and jump do ([`parity_even`]), so unordered
//! operands take the exe's branch.

use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn, Engine};

use crate::culling::layout::NiPoint3;
pub(crate) use crate::culling::{fld, fld_local, fstp, m32, m32_local};
use engine_core::ptr::Ptr;
use layout::{
    AStarQueue, AStarSearch, Arc, Counter, EdgeLocation, EdgePair, MeshEdge, NavMesh, NavMeshGrid,
    NavMeshInfo, NavMeshTriangle, PathingLocation, Point2, SearchLocation, SimpleArray, SortNode,
    Sphere,
};

pub mod layout;
pub mod location;
pub mod navmesh;
pub mod point;
pub mod pov;
pub mod search;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(7);

/// `NV_TYPED=pathing`. Not past its gate.
pub static PATHING: Family = Family {
    name: "pathing",
    id: FAMILY,
    past_gate: false,
    threads: Threads::Both,
    funcs,
};

/// The `double` 0.0 at `01012060`, the operand of the zero tests.
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The float `FLT_MAX` at `01016970`.
pub(crate) const FLOAT_MAX: u32 = 0x0101_6970;

/// `FCOM`/`FCOMP`/`FCOMPP` of `ST0` against `source`: the condition bits in `AH` (`C0` 0x01, `C2`
/// 0x04, `C3` 0x40): none when greater, `C0` when less, `C3` when equal, all three when unordered.
#[inline(always)]
pub(crate) fn fcom(st0: f64, source: f64) -> u8 {
    if st0.is_nan() || source.is_nan() {
        0x45
    } else if st0 < source {
        0x01
    } else if st0 == source {
        0x40
    } else {
        0
    }
}

/// `TEST AH,mask` then `JP`: taken when the tested bits hold an even number of ones.
#[inline(always)]
pub(crate) fn parity_even(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// The low word of a 64-bit integer (what the exe reads back after `FISTP qword`).
#[inline(always)]
pub(crate) fn low_word(v: i64) -> u32 {
    let b = v.to_le_bytes();
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// `FLD dword` of a float in the exe's data.
#[inline(always)]
pub(crate) fn fld_global(e: &Engine, addr: u32) -> f64 {
    engine_core::x87::x87_load32(e.global::<u32>(addr))
}

/// A `double` in the exe's data as the memory operand of an x87 instruction.
#[inline(always)]
pub(crate) fn m64_global(e: &Engine, addr: u32) -> f64 {
    f64::from_bits(e.global::<u64>(addr))
}

/// `FLD dword; FSTP dword`: a float passed on through the x87 stack (a signalling NaN quieted).
#[inline(always)]
pub(crate) fn through(value: f32) -> f32 {
    fstp(fld_local(value))
}

/// Copies a point's three floats as words (`MOV`, never through the x87 stack).
#[inline(always)]
pub(crate) fn copy_point(e: &mut Engine, from: Ptr<NiPoint3>, to: Ptr<NiPoint3>) {
    for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
        let v = e.get(from, f);
        e.set(to, f, v);
    }
}

/// The family's functions, by exe address.
#[rustfmt::skip]
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        // Point helpers.
        entry!(0x0043_9ef0, point::subtract(Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x0043_9e90, point::add(Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x004a_3760, point::scaled(Ptr<NiPoint3>, f32, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x0043_9180, point::scale_by(Ptr<NiPoint3>, f32) -> Ptr<NiPoint3>),
        entry!(0x0063_c8a0, point::add_to(Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x0059_5c80, point::length_xy_squared(Ptr<NiPoint3>) -> f32),
        entry!(0x004a_7290, point::length_squared(Ptr<NiPoint3>) -> f32),
        entry!(0x0045_7990, point::length(Ptr<NiPoint3>) -> f32),
        entry!(0x0058_9850, point::length_2d(Ptr<Point2>) -> f32),
        entry!(0x0040_8840, point::abs(f32) -> f32),
        entry!(0x0040_19b0, point::root(f32) -> f32),
        entry!(0x0045_79e0, point::root_of(f32) -> f32),
        entry!(0x0040_19d0, point::root_rounded(f32) -> f32),
        entry!(0x004a_0bd0, point::negated(Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x0069_e0c0, point::on_circle(Ptr<Point2>, Ptr<NiPoint3>, f32) -> Ptr<NiPoint3>),
        // The navmesh.
        entry!(0x0055_82f0, navmesh::triangle(Ptr<NavMesh>, u16) -> Ptr<NavMeshTriangle>),
        entry!(0x0055_8dd0, navmesh::triangle_at(Ptr<SimpleArray>, u32) -> Ptr<NavMeshTriangle>),
        entry!(0x0055_82d0, navmesh::vertex(Ptr<NavMeshTriangle>, u32) -> u16),
        entry!(0x0055_8200, navmesh::triangle_count(Ptr<NavMesh>) -> u32),
        entry!(0x0055_8220, navmesh::center(Ptr<NavMesh>, Ptr<NiPoint3>, u16) -> Ptr<NiPoint3>),
        entry!(0x0068_f0a0, navmesh::vertex_point(Ptr<NavMesh>, u16) -> Ptr<NiPoint3>),
        entry!(0x006a_1440, navmesh::entry_12(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006a_7af0, navmesh::entry_12_of(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x0069_1140, navmesh::has_flags(Ptr<NavMeshTriangle>, u32) -> bool),
        entry!(0x0068_f1d0, navmesh::edge_flag(Ptr<NavMeshTriangle>, u32) -> bool),
        entry!(0x0069_3b70, navmesh::flag_40(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x0069_3b90, navmesh::flag_10(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x006a_5ec0, navmesh::flag_200(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x0068_f200, navmesh::has_link(Ptr<NavMeshTriangle>, i32) -> bool),
        entry!(0x0068_f2c0, navmesh::link(Ptr<NavMeshTriangle>, i32) -> u16),
        entry!(0x0068_f230, navmesh::edge_info(Ptr<NavMesh>, u16, i32) -> u32),
        entry!(0x0069_3d30, navmesh::edge_info_at(Ptr<NavMesh>, u16) -> u32),
        entry!(0x0068_efb0, navmesh::info(Ptr<NavMesh>) -> u32),
        entry!(0x0068_f460, navmesh::matching_edge(Ptr<NavMesh>, u16, i32, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0068_f670, navmesh::matching_edge_of(Ptr<NavMesh>, Ptr<MeshEdge>, Ptr) -> bool),
        entry!(0x0068_f3b0, navmesh::matching_edge_held(Ptr<NavMesh>, u16, i32, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0068_f0c0, navmesh::edge_points(Ptr<NavMesh>, u16, i32, Ptr<NiPoint3>, Ptr<NiPoint3>) -> bool),
        entry!(0x0068_efd0, navmesh::edge(Ptr<NavMesh>, Ptr, u16, i32) -> Ptr),
        entry!(0x0068_f040, navmesh::edge_vertices(Ptr<NavMesh>, Ptr, u16, i32) -> Ptr),
        entry!(0x0069_7980, navmesh::triangle_z(Ptr<NavMesh>, u16, Ptr<NiPoint3>, f32, f32, Ptr) -> bool),
        entry!(0x0069_8320, navmesh::crossed_edge(Ptr<NavMesh>, u16, i32, Ptr<Point2>, Ptr<Point2>) -> i32),
        entry!(0x0069_8060, navmesh::triangle_edge(Ptr<NavMesh>, u16, i32, Ptr<Point2>, Ptr<Point2>, Ptr<Point2>) -> i32),
        entry!(0x0069_6a50, navmesh::triangle_for(Ptr<NavMesh>, Ptr<NiPoint3>, u32) -> u16),
        entry!(0x0069_6cf0, navmesh::closest_triangle(Ptr<NavMesh>, Ptr<NiPoint3>, Ptr, Ptr) -> bool),
        entry!(0x0069_69e0, navmesh::grid_range(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x0069_67c0, navmesh::grid_cell(Ptr<NavMeshGrid>, u32, u32) -> u32),
        entry!(0x0069_d370, navmesh::grid_cell_at(Ptr<NavMeshGrid>, Ptr<NiPoint3>, Ptr)),
        entry!(0x0069_71e0, navmesh::plane_normal(Ptr<NavMesh>, Ptr<NiPoint3>, u16) -> Ptr<NiPoint3>),
        entry!(0x006c_d5c0, navmesh::water_can_cross(Ptr, u32, u32, u32, Ptr<NavMesh>, u16, u32) -> bool),
        entry!(0x006a_78d0, navmesh::referenced_triangle(Ptr<MeshEdge>) -> Ptr<NavMeshTriangle>),
        entry!(0x0069_ad00, navmesh::resolve(Ptr<NavMeshInfo>, Ptr) -> bool),
        entry!(0x0069_0890, navmesh::hold(Ptr, u32)),
        entry!(0x0069_0830, navmesh::info_world_space(Ptr<NavMeshInfo>) -> u32),
        entry!(0x0069_0800, navmesh::parent_world_space(Ptr<NavMeshInfo>) -> u32),
        entry!(0x0069_dfb0, navmesh::is_loaded(Ptr<NavMeshInfo>) -> bool),
        entry!(0x006a_1b60, navmesh::info_mesh(Ptr<NavMeshInfo>) -> u32),
        entry!(0x0069_3d70, navmesh::reference_by_id(Ptr, u32) -> u32),
        entry!(0x006b_77e0, navmesh::info_for_cell(Ptr, Ptr) -> u32),
        entry!(0x006b_7850, navmesh::info_in_world(Ptr, Ptr, u32) -> u32),
        entry!(0x006d_6f40, navmesh::potential_info(u32) -> u32),
        entry!(0x006d_6f60, navmesh::potential_info_in_world(u32, u32) -> u32),
        // The search.
        entry!(0x006b_9710, search::sort_key(Ptr<SortNode>) -> f32),
        entry!(0x006b_9130, search::second_float(Ptr<SortNode>) -> f64),
        entry!(0x0069_ef80, search::request_radius(Ptr) -> f64),
        entry!(0x006a_7f50, search::first_float(Ptr<SortNode>) -> f64),
        entry!(0x006b_81d0, search::keys_equal(Ptr, u32, u32) -> u8),
        entry!(0x0069_1e50, search::keys_equal_16(Ptr, u16, u16) -> bool),
        entry!(0x006c_6a60, search::hash(Ptr<Counter>, u32) -> u32),
        entry!(0x0049_c4c0, search::hash_16(Ptr<Counter>, u16) -> u32),
        entry!(0x006a_94f0, search::node_key(Ptr, Ptr, u16) -> u32),
        entry!(0x0069_e390, search::first_word_at_94(Ptr) -> u32),
        entry!(0x006f_4790, search::scaled_cost(Ptr, Ptr<SortNode>) -> u32),
        entry!(0x006a_1f90, search::scrap_new(u32, Ptr) -> u32),
        entry!(0x006a_79c0, search::is_goal(Ptr<AStarSearch>, Ptr) -> bool),
        entry!(0x006b_8c20, search::info_is_goal(Ptr, Ptr) -> u32),
        entry!(0x006a_6eb0, search::location_center(Ptr<SearchLocation>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x006a_7ad0, search::element(Ptr, u32) -> u32),
        entry!(0x0069_dec0, search::list_element(Ptr, u32) -> u32),
        entry!(0x0069_50d0, search::entry_8(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006b_3160, search::entry_14(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006d_ad90, search::entry_14_of(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006a_8500, search::free_data(Ptr<SimpleArray>)),
        entry!(0x006b_3eb0, search::init_vertices(Ptr<SimpleArray>, u32, u32)),
        entry!(0x006b_3f40, search::init_nodes(Ptr<SimpleArray>, u32, u32)),
        entry!(0x0069_2290, search::init_indices(Ptr<SimpleArray>, u32, u32)),
        entry!(0x0069_2200, search::next_index(Ptr<SimpleArray>) -> u32),
        entry!(0x006b_4060, search::next_slot(Ptr<SimpleArray>) -> u32),
        entry!(0x0069_1820, search::push_index(Ptr<SimpleArray>, Ptr) -> u32),
        entry!(0x0069_18a0, search::copy_indices(Ptr<SimpleArray>, u32, u32) -> u32),
        entry!(0x0069_1870, search::allocate_indices(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x0069_af80, search::allocate_entries(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006d_b090, search::free_block(Ptr<SimpleArray>, u32)),
        entry!(0x0069_17d0, search::index_array_new(Ptr<SimpleArray>) -> Ptr<SimpleArray>),
        entry!(0x0069_1800, search::index_array_drop(Ptr<SimpleArray>)),
        entry!(0x0069_b2c0, search::edge_array_new(Ptr<SimpleArray>) -> Ptr<SimpleArray>),
        entry!(0x0069_b2f0, search::edge_array_drop(Ptr<SimpleArray>)),
        entry!(0x006a_8a30, search::construct_20s(Ptr, u32, u32)),
        entry!(0x006d_03d0, search::construct_12s(Ptr, u32, u32)),
        entry!(0x006b_40f0, search::construct_32s(Ptr, u32, u32)),
        entry!(0x006b_3de0, search::slots_new(Ptr) -> Ptr),
        entry!(0x006b_9470, search::queue_new(Ptr<AStarQueue>) -> Ptr<AStarQueue>),
        entry!(0x006a_7b10, search::search_new(Ptr<AStarSearch>) -> Ptr<AStarSearch>),
        entry!(0x006c_9900, search::at_5c(u32) -> u32),
        entry!(0x006a_9540, search::at_34(Ptr) -> u32),
        entry!(0x006b_05d0, search::byte_at_40(Ptr) -> u8),
        entry!(0x006d_4ca0, search::edges_equal(Ptr<MeshEdge>, Ptr<MeshEdge>) -> bool),
        entry!(0x006d_b4f0, search::append_edge(Ptr<SimpleArray>, u32) -> u32),
        entry!(0x006e_78b0, search::solution_node(Ptr, u32) -> u32),
        entry!(0x006e_cba0, search::tagged_new(Ptr, u8, u32) -> Ptr),
        // The point-of-view search.
        entry!(0x0069_85f0, pov::copy_pair(Ptr<Point2>, Ptr<Point2>) -> Ptr<Point2>),
        entry!(0x0069_8620, pov::relative(Ptr<Point2>, Ptr<Point2>, Ptr<Point2>) -> Ptr<Point2>),
        entry!(0x006a_0e90, pov::rotated(Ptr<Point2>, Ptr<Point2>, Ptr<Point2>) -> Ptr<Point2>),
        entry!(0x0069_8680, pov::quadrant(Ptr<Point2>) -> u32),
        entry!(0x006a_0660, pov::arc_contains(Ptr<Arc>, Ptr<Point2>) -> bool),
        entry!(0x006a_04b0, pov::sphere_new(Ptr<Sphere>, Ptr<NiPoint3>, f32) -> Ptr<Sphere>),
        entry!(0x006a_04e0, pov::angle_new(Ptr<Point2>, f32) -> Ptr<Point2>),
        entry!(0x006a_05e0, pov::arc_new(Ptr<Arc>, Ptr<Point2>, Ptr<Point2>, u32) -> Ptr<Arc>),
        entry!(0x006a_0620, pov::arc_assign(Ptr<Arc>, Ptr<Arc>) -> Ptr<Arc>),
        entry!(0x006a_0480, pov::vertex_new(Ptr) -> Ptr),
        entry!(0x006a_0870, pov::tangents_new(Ptr) -> Ptr),
        entry!(0x006c_4350, pov::mesh_edge_new(Ptr<MeshEdge>, Ptr<MeshEdge>) -> Ptr<MeshEdge>),
        entry!(0x006b_c870, pov::mesh_edge_copy(Ptr<MeshEdge>, Ptr<MeshEdge>) -> Ptr<MeshEdge>),
        entry!(0x0069_a690, pov::triangle_ref_copy(Ptr<MeshEdge>, Ptr<MeshEdge>) -> Ptr<MeshEdge>),
        entry!(0x0069_a660, pov::triangle_ref_set(Ptr<MeshEdge>, u32, u16) -> Ptr<MeshEdge>),
        entry!(0x0069_a6c0, pov::mesh_edge_set(Ptr<MeshEdge>, u32, u16, u32) -> Ptr<MeshEdge>),
        entry!(0x006b_39a0, pov::edge_pair_copy(Ptr<EdgePair>, Ptr<EdgePair>) -> Ptr<EdgePair>),
        entry!(0x006a_0c10, pov::edge_location_new(Ptr<EdgeLocation>, Ptr<MeshEdge>, Ptr<NiPoint3>) -> Ptr<EdgeLocation>),
        entry!(0x006a_0e50, pov::edge_location_copy(Ptr<EdgeLocation>, Ptr<EdgeLocation>) -> Ptr<EdgeLocation>),
        entry!(0x006b_9b10, pov::distance_to_segment(Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr) -> f32),
        entry!(0x006b_a240, pov::intersection(Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> bool),
        entry!(0x006b_9e90, pov::crossing(Ptr<Point2>, Ptr<Point2>, Ptr<Point2>, Ptr<Point2>, Ptr<Point2>) -> bool),
        entry!(0x006b_9cf0, pov::height_in(Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr, f32, f32) -> bool),
        entry!(0x006b_ab00, pov::segment_within(Ptr<NiPoint3>, f32, Ptr<NiPoint3>, Ptr<NiPoint3>) -> bool),
        entry!(0x006b_b570, pov::sphere_tangents(Ptr<Sphere>, Ptr<Sphere>, u32, Ptr) -> u32),
        // Pathing locations.
        entry!(0x0044_1110, location::world_space(Ptr<PathingLocation>) -> u32),
        entry!(0x006a_9520, location::is_valid(Ptr<PathingLocation>) -> bool),
        entry!(0x006d_d640, location::mesh_and_triangle(Ptr<PathingLocation>, Ptr, Ptr) -> bool),
        entry!(0x006d_d610, location::mesh(Ptr<PathingLocation>, Ptr) -> bool),
        entry!(0x006d_d6a0, location::has_triangle(Ptr<PathingLocation>) -> bool),
        entry!(0x006d_d4f0, location::cell(Ptr<PathingLocation>) -> u32),
        entry!(0x006d_cbb0, location::new(Ptr<PathingLocation>) -> Ptr<PathingLocation>),
        entry!(0x006d_cee0, location::new_at(Ptr<PathingLocation>, u32, u32, u32) -> Ptr<PathingLocation>),
        entry!(0x006d_cf20, location::new_in(Ptr<PathingLocation>, Ptr<NiPoint3>, Ptr<NavMeshInfo>) -> Ptr<PathingLocation>),
        entry!(0x006d_cc40, location::copy(Ptr<PathingLocation>, Ptr<PathingLocation>) -> Ptr<PathingLocation>),
        entry!(0x006d_cce0, location::assign(Ptr<PathingLocation>, Ptr<PathingLocation>) -> Ptr<PathingLocation>),
        entry!(0x006d_5320, location::world_location_new(Ptr) -> Ptr),
    ]
}
