//! The classes of the pathing family, once each, at their PC offsets, read from the functions that
//! use them. A class read only as far as this family's code reads it says so; its size is then where
//! the last field read ends, rounded to a word. Floats are `f32` fields: their bits load and store
//! unchanged until an x87 load in the code reads them.

use crate::culling::layout::{NiPoint3, NiPoint3Image};
use crate::image;
use engine_core::ptr::{Field, Inline};

image! {
    /// A two-float point or direction (an angle's cosine and sine, an arc's ends).
    pub struct Point2 / Point2Image: 0x8 {
        0x0 x: f32,
        0x4 y: f32,
    }

    /// `BSSimpleArray` (Xbox PDB): a vtable (whose slot 1 allocates and slot 2 frees), the buffer,
    /// the count in use and the count allocated.
    pub struct SimpleArray / SimpleArrayImage: 0x10 {
        0x0 vtable: u32,
        0x4 data: u32,
        0x8 size: u32,
        0xc capacity: u32,
    }

    /// `NavMeshTriangle` (Xbox PDB): three vertex indices, three edge links (another triangle of the
    /// mesh, or `0xffff`), and the flags (bit `n` of the low three: edge `n` leads to another mesh).
    pub struct NavMeshTriangle / NavMeshTriangleImage: 0x10 {
        0x0 vertex_0: u16,
        0x2 vertex_1: u16,
        0x4 vertex_2: u16,
        0x6 link_0: u16,
        0x8 link_1: u16,
        0xa link_2: u16,
        0xc flags: u32,
    }

    /// An edge's extra information: the navmesh info of the mesh on the other side and its triangle.
    pub struct EdgeInfo / EdgeInfoImage: 0xc {
        0x0 head: u32,
        0x4 info: u32,
        0x8 triangle: u16,
        0xa pad: u16,
    }

    /// The navmesh's grid of triangle lists: cells per side, a cell's width and depth, the box it
    /// covers and the cells (a triangle list of 0x10 bytes each, row by row).
    pub struct NavMeshGrid / NavMeshGridImage: 0x28 {
        0x00 side: u32,
        0x04 cell_x: f32,
        0x08 cell_y: f32,
        0x0c min_x: f32,
        0x10 min_y: f32,
        0x14 min_z: f32,
        0x18 max_x: f32,
        0x1c max_y: f32,
        0x20 max_z: f32,
        0x24 cells: u32,
    }

    /// `NavMesh` (Xbox PDB), as far as this family reads it: the water it lies under, the vertices
    /// (0xc bytes each), the triangles, the edges' extra information, the grid and its navmesh info.
    pub struct NavMesh / NavMeshImage: 0x108 {
        0x000 head: [u32; 9],
        0x024 water: u32,
        0x028 vertices: Inline<SimpleArray> = SimpleArrayImage,
        0x038 triangles: Inline<SimpleArray> = SimpleArrayImage,
        0x048 edge_infos: Inline<SimpleArray> = SimpleArrayImage,
        0x058 middle: [u32; 20],
        0x0a8 grid: Inline<NavMeshGrid> = NavMeshGridImage,
        0x0d0 tail: [u32; 13],
        0x104 info: u32,
    }

    /// `NavMeshInfo` (Xbox PDB), as far as this family reads it.
    pub struct NavMeshInfo / NavMeshInfoImage: 0x58 {
        0x00 vtable: u32,
        0x04 cell_id: u32,
        0x08 unknown_8: u32,
        0x0c cell_key: u32,
        0x10 unknown_10: [u32; 3],
        0x1c world_space: u32,
        0x20 unknown_20: [u32; 13],
        0x54 navmesh: u32,
    }

    /// A triangle of a mesh, with a word beside it: an edge's or a node's reference.
    pub struct MeshEdge / MeshEdgeImage: 0xc {
        0x0 mesh: u32,
        0x4 triangle: u16,
        0x6 pad: u16,
        0x8 extra: u32,
    }

    /// A point on an edge of the point-of-view search.
    pub struct EdgeLocation / EdgeLocationImage: 0x18 {
        0x00 edge: Inline<MeshEdge> = MeshEdgeImage,
        0x0c point: Inline<NiPoint3> = NiPoint3Image,
    }

    /// An arc of directions: its two ends and which way it turns.
    pub struct Arc / ArcImage: 0x14 {
        0x00 first: Inline<Point2> = Point2Image,
        0x08 second: Inline<Point2> = Point2Image,
        0x10 flag: u32,
    }

    /// Two references: a triangle of a mesh and an edge, with three words after them.
    pub struct EdgePair / EdgePairImage: 0x20 {
        0x00 mesh: u32,
        0x04 triangle: u16,
        0x06 pad: u16,
        0x08 edge: Inline<MeshEdge> = MeshEdgeImage,
        0x14 word_14: u32,
        0x18 word_18: u32,
        0x1c word_1c: u32,
    }

    /// A tangent of the sphere-tangent list (after its count word): which side, the direction and
    /// the second direction.
    pub struct Tangent / TangentImage: 0x14 {
        0x00 side: u32,
        0x04 x: f32,
        0x08 y: f32,
        0x0c x2: f32,
        0x10 y2: f32,
    }

    /// A sphere: a centre and a radius.
    pub struct Sphere / SphereImage: 0x10 {
        0x0 center: Inline<NiPoint3> = NiPoint3Image,
        0xc radius: f32,
    }

    /// A search node's cost so far and estimate left (the sort key is their sum).
    pub struct SortNode / SortNodeImage: 0x8 {
        0x0 cost: f32,
        0x4 estimate: f32,
    }

    /// A hash map as far as its hash reads it: the bucket count.
    pub struct Counter / CounterImage: 0x8 {
        0x0 vtable: u32,
        0x4 buckets: u32,
    }

    /// The A* search's best-node queue, as far as its constructor writes it.
    pub struct AStarQueue / AStarQueueImage: 0x54 {
        0x00 best: f32,
        0x04 slots: [u32; 20],
    }

    /// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` and the `NavMeshSearch` built on it (Xbox
    /// PDB), as far as this family reads them: the open list, the node slots, the queue, the
    /// request and the goal triangle.
    pub struct AStarSearch / AStarSearchImage: 0x2084 {
        0x0000 vtable: u32,
        0x0004 open: [u32; 4],
        0x0014 nodes: [u32; 0x7f8],
        0x1ff4 queue: Inline<AStarQueue> = AStarQueueImage,
        0x2048 unknown_2048: u32,
        0x204c unknown_204c: u32,
        0x2050 unknown_2050: u32,
        0x2054 done: u8,
        0x2055 pad: [u8; 3],
        0x2058 request: u32,
        0x205c unknown_205c: [u32; 5],
        0x2070 goal: Inline<SearchLocation> = SearchLocationImage,
    }

    /// A search location: a triangle of a mesh and a point.
    pub struct SearchLocation / SearchLocationImage: 0x14 {
        0x0 mesh: u32,
        0x4 triangle: u16,
        0x6 pad: u16,
        0x8 point: Inline<NiPoint3> = NiPoint3Image,
    }

    /// The node slots of the A* search, as far as their constructor writes them.
    pub struct NodeSlots / NodeSlotsImage: 0x1fe0 {
        0x0000 slots: [u32; 0x7f7],
        0x1fdc count: u32,
    }

    /// The navmesh-info search, as far as its goal test reads it.
    pub struct InfoSearch / InfoSearchImage: 0x2078 {
        0x0000 head: [u32; 0x81d],
        0x2074 goal: u32,
    }

    /// A node of the navmesh-info search, as far as the goal test reads it.
    pub struct InfoNode / InfoNodeImage: 0xc {
        0x0 head: [u32; 2],
        0x8 info: u32,
    }

    /// A float a cost is scaled against.
    pub struct Scale / ScaleImage: 0x4 {
        0x0 value: f32,
    }

    /// `PathingRequest` (Xbox PDB), as far as its radius.
    pub struct PathingRequest / PathingRequestImage: 0x6c {
        0x00 head: [u32; 26],
        0x68 radius: f32,
    }

    /// A path smoother node, as far as its flag byte.
    pub struct SmootherNode / SmootherNodeImage: 0x44 {
        0x00 head: [u32; 16],
        0x40 flag: u8,
        0x41 pad: [u8; 3],
    }

    /// `PathingLocation` (Xbox PDB): a point, the navmesh info and triangle it is on (or `0xffff`), the
    /// cell or world space it is in and its cell key, and flags.
    pub struct PathingLocation / PathingLocationImage: 0x28 {
        0x00 vtable: u32,
        0x04 point: Inline<NiPoint3> = NiPoint3Image,
        0x10 info: u32,
        0x14 unknown_14: u32,
        0x18 cell: u32,
        0x1c world_space: u32,
        0x20 cell_key: u32,
        0x24 triangle: u16,
        0x26 flags: u8,
        0x27 pad: u8,
    }
}

/// Vertex `i` of a triangle (the exe indexes the words from the start, `i` unchecked).
pub const fn vertex_field(i: u32) -> Field<NavMeshTriangle, u16> {
    Field::new(i.wrapping_mul(2))
}

/// Edge link `i` of a triangle.
pub const fn link_field(i: u32) -> Field<NavMeshTriangle, u16> {
    Field::new(6u32.wrapping_add(i.wrapping_mul(2)))
}

/// The vtable `PathingLocation`'s constructors install.
pub const PATHING_LOCATION_VTABLE: u32 = 0x0102_583c;
