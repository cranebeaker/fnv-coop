//! `Havok SDK/hkpconvexradiusbuilder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexRadiusBuilder` (Xbox PDB name) turns a collision shape into display geometries (the
//! convex radius viewer draws them). The class is not in the Xbox PDB's type list, so its layout
//! is read off the code: the base is a reference counted object (`+4` size `u16`), `+8` is the
//! `float` radius tolerance below which a shape gets no display, and `+0xC` the display convex the
//! builder made for its list (`00de4340`). The user builder (`hkpUserConvexRadiusBuilder`) keeps an
//! `hkArray` (data `+8`, size `+0xC`, capacity and flags `+0x10`) of 8-byte entries `(function,
//! shape type)` that `buildShapeDisplay` walks for the shape types it does not know (the global
//! `01269ce4` points to it).
//!
//! The three helpers `00de3110`, `00de3200` and `00de3360` take registers: their first words here
//! are the register values (`EAX`, `ESI`, `ECX`), then the stack words. SSE code computes lane by
//! lane in `f32`; a sum of three squares is `z + (y + x)` the way the `SHUFPS`/`ADDSS` sequences add
//! it; `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`). x87 code rounds at each `float`
//! store, and a float copied through `FLD`/`FSTP` goes through [`x87_quiet`].
//!
//! Not translated: the C++ exception frames (none of these functions has one).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};
use crate::x87::rsqrt_estimate;

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl): `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The grow step of `hkArray::pushBack` (`00c90940`, cdecl): `(allocator, array, element size)`.
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkTransform::setMul` (`00cb2240`, `RET 8`): `this` = result, `(left, right)`.
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// `hkVector4::setTransformedPos` (`00c7f640`, `RET 8`): `this` = result, `(transform, point)`.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// Called on a new display geometry's `+0x10` with the transform (`00c6cd60`, `RET 4`).
const SET_DISPLAY_TRANSFORM: u32 = 0x00c6_cd60;
/// The display cylinder constructor (`00d58510`, `RET 0x14`): `this` = the new block, `(first end
/// point, second end point, radius, 9, 1)`.
const DISPLAY_CYLINDER_CONSTRUCTOR: u32 = 0x00d5_8510;
/// The display box constructor (`00de1f50`, `RET 4`): `this` = the new block, `(half extents)`.
const DISPLAY_BOX_CONSTRUCTOR: u32 = 0x00de_1f50;
/// The display convex constructor (`00de1d20`, `RET 4`): `this` = the new block, `(geometry)`.
const DISPLAY_CONVEX_CONSTRUCTOR: u32 = 0x00de_1d20;
/// `hkpTriangleUtil::isDegenerate` (`00ceb8d0`, cdecl): `(result byte, a, b, c, epsilon)`, answers
/// a pointer to the byte.
const TRIANGLE_IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// `hkGeometryUtility::createVerticesFromPlaneEquations` (`00d5a6a0`, cdecl): `(planes, vertices)`.
const CREATE_VERTICES_FROM_PLANES: u32 = 0x00d5_a6a0;
/// `hkGeometryUtility::createConvexGeometry` (`00d5a5e0`, cdecl): `(strided vertices, geometry)`.
const CREATE_CONVEX_GEOMETRY: u32 = 0x00d5_a5e0;
/// `hkpCylinderShape::getCylinderRadius` (`00d571a0`, answers it in `ST0`).
const CYLINDER_RADIUS: u32 = 0x00d5_71a0;
/// `hkpConvexVerticesShape::getPlaneEquations` (`00d58d50`): the shape's plane `hkArray`.
const GET_PLANE_EQUATIONS: u32 = 0x00d5_8d50;
/// `hkpConvexVerticesShape::getOriginalVertices` (`00d59200`): `(vertices array)`.
const GET_ORIGINAL_VERTICES: u32 = 0x00d5_9200;
/// The global `hkpUserConvexRadiusBuilder` pointer (`01269ce4`).
const USER_BUILDER: u32 = 0x0126_9ce4;
/// The identity transform `getAabb` is given (`01268370`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
/// The `hkBaseObject` vtable (`0102e388`) the user builder's destructor ends with.
const BASE_VTABLE: u32 = 0x0102_e388;

/// `float` constants: 0.0, 3.0, 0.5, -0.0, 1.0, -1.0, -1000.0 and the triangle epsilon.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const MINUS_ZERO: u32 = 0x010c_4990;
const ONE: u32 = 0x0101_2068;
const MINUS_ONE: u32 = 0x0101_2054;
const MINUS_THOUSAND: u32 = 0x0101_b31c;
const TRIANGLE_EPSILON: u32 = 0x0101_7d00;
/// The `double` the convex radius is subtracted from (`0.005`).
const RADIUS_MARGIN: u32 = 0x0102_3b88;
/// Vector constants: four 0.0 (the plane offsets start from lane 0 of it) and the sign mask.
const ZERO_VECTOR: u32 = 0x010c_7160;
const SIGN_MASK: u32 = 0x010c_44d0;

/// The capacity word of an empty `hkArray` that owns nothing.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// The mask of the capacity bits of an `hkArray`'s capacity-and-flags word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// `hkpShape::m_type` and the `float` radius every convex shape keeps at `+0x10`.
const SHAPE_TYPE: u32 = 0x0c;
const SHAPE_RADIUS: u32 = 0x10;
/// The builder's radius tolerance and its cached display convex.
const BUILDER_TOLERANCE: u32 = 0x08;
const BUILDER_DISPLAY_CONVEX: u32 = 0x0c;

/// `SHUFPS dst, src, imm` on four lanes: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
pub(crate) fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let pick = |from: Vec4, shift: u8| from[usize::from((imm >> shift) & 3)];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

pub(crate) fn mulps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

pub(crate) fn addps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

pub(crate) fn subps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// Lane 0 of the vector repeated (`SHUFPS v, v, 0`).
pub(crate) fn splat(value: f32) -> Vec4 {
    [value; 4]
}

/// `z + (y + x)`: the sum of lanes 0 to 2 the `SHUFPS 0x55`/`ADDSS`/`SHUFPS 0xaa`/`ADDSS` sequence
/// leaves in lane 0.
pub(crate) fn sum_of_three(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// The refined inverse square root of `sum`: `RSQRTSS`, one Newton step
/// `(half * r) * (three - (sum * r) * r)`, and zero when `sum == zero` (`CMPEQSS`/`ANDNPS`).
pub(crate) fn normalising_scale(sum: f32, zero: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    let value = (half * estimate) * (three - (sum * estimate) * estimate);
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// The allocator of the memory router (`router + 0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// `hkArray<void*>::pushBack`: the router, room for one more through `00c90940` when the size
/// equals the capacity, then the element.
fn push_back_pointer(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + size * 4, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// Frees the data of an `hkArray` of 16-byte elements unless its capacity word says it does not own
/// it, after setting its size to zero.
fn release_vector_array(e: &mut Engine, array: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 4],
        );
    }
}

/// Makes a block of `size` bytes from the allocator (`vtable + 4`).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let allocator = memory_allocator(e);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// The number of 16-byte elements in `bytes` (`CDQ`/`AND 0xF`/`ADD`/`SAR 4`).
fn sixteen_byte_elements(bytes: i32) -> i32 {
    (bytes + ((bytes >> 31) & 0xf)) >> 4
}

// Translated from 00de10e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUserConvexRadiusBuilder::~hkpUserConvexRadiusBuilder` scalar deleting destructor
/// (Xbox PDB): clears the entry count, frees the entry array when it owns its data (8 bytes per
/// entry), leaves the array empty, restores the base vtable, and frees the object (its `u16` size
/// at `+4`) when `flags & 1`. Answers `this`.
pub fn hkp_user_convex_radius_builder_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let this = this.addr();
    let allocator = memory_allocator(e);
    e.mem.set_u32(this + 0xc, 0);
    let capacity = e.mem.u32(this + 0x10);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(this + 8);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 3],
        );
    }
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0x10, EMPTY_CAPACITY);
    e.mem.set_u32(this, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(this + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    Ptr::new(this)
}

// Translated from 00de3090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets the builder's cached display convex (`+0xC`).
pub fn fn_00de3090(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + BUILDER_DISPLAY_CONVEX, 0);
}

// Translated from 00de3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_getPlaneToSurfaceDistance` (Xbox PDB name): the largest `w + dot(plane, vertex)` over every
/// plane equation and original vertex of the convex vertices shape, starting from `-1000.0`
/// (`ST0` answer). Register input: `shape` is `EAX`. The temporary vertex array is freed when it
/// owns its data.
pub fn get_plane_to_surface_distance(e: &mut Engine, shape: Ptr) -> f32 {
    let shape = shape.addr();
    let planes = e.call(GET_PLANE_EQUATIONS, &args![shape]).u32();
    e.with_stack(0xc, |e, vertices| {
        let vertices = vertices.addr();
        e.mem.set_u32(vertices + 8, EMPTY_CAPACITY);
        e.call(GET_ORIGINAL_VERTICES, &args![shape, vertices]);
        let mut farthest = x87_quiet(e.global::<f32>(MINUS_THOUSAND));
        let plane_count = e.mem.i32(planes + 4);
        let vertex_count = e.mem.i32(vertices + 4);
        let mut vertex = e.mem.u32(vertices);
        for _ in 0..vertex_count.max(0) {
            if plane_count > 0 {
                let point = load_vec(e, vertex);
                let mut plane = e.mem.u32(planes);
                for _ in 0..plane_count {
                    let products = mulps(load_vec(e, plane), point);
                    let dot = sum_of_three(products);
                    let w = e.mem.f32(plane + 0xc);
                    let value = (f64::from(w) + f64::from(dot)) as f32;
                    if farthest < value {
                        farthest = value;
                    }
                    plane += 0x10;
                }
            }
            vertex += 0x10;
        }
        release_vector_array(e, vertices);
        x87_quiet(farthest)
    })
}

// Translated from 00de3200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_addBoundingPlanes` (Xbox PDB name): asks the shape for its box (`getAabb` with the identity
/// transform and `radius` as the tolerance), makes room for six more planes in the plane array
/// and appends the six faces of that box: `(1,0,0,-max.x)`, `(0,1,0,-max.y)`, `(0,0,1,-max.z)`,
/// `(-1,0,0,min.x)`, `(0,-1,0,min.y)` and `(0,0,-1,min.z)`. Register input: `array` is `ESI`,
/// `shape` is `ECX`; `radius` is the stack word (the caller removes it).
pub fn add_bounding_planes(e: &mut Engine, array: Ptr, shape: Ptr, radius: f32) {
    let array = array.addr();
    let radius = x87_quiet(radius);
    e.with_stack(0x20, |e, aabb| {
        let aabb = aabb.addr();
        e.vcall(shape.addr(), 0x1c, &args![IDENTITY_TRANSFORM, radius, aabb]);
        let allocator = memory_allocator(e);
        let wanted = e.mem.u32(array + 4).wrapping_add(6) as i32;
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity < wanted {
            let doubled = capacity + capacity;
            let new_capacity = if wanted < doubled { doubled } else { wanted };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, new_capacity, 0x10u32],
            );
        }
        let size = e.mem.u32(array + 4) + 6;
        e.mem.set_u32(array + 4, size);
        let zero = e.global::<f32>(ZERO);
        let minus_zero = e.global::<f32>(MINUS_ZERO);
        let one = e.global::<f32>(ONE);
        let minus_one = e.global::<f32>(MINUS_ONE);
        let low = load_vec(e, aabb);
        let high = load_vec(e, aabb + 0x10);
        let planes = [
            [one, zero, zero, minus_zero - high[0]],
            [zero, one, zero, minus_zero - high[1]],
            [zero, zero, one, minus_zero - high[2]],
            [minus_one, zero, zero, low[0]],
            [zero, minus_one, zero, low[1]],
            [zero, zero, minus_one, low[2]],
        ];
        let data = e.mem.u32(array);
        for (index, plane) in planes.into_iter().enumerate() {
            store_vec(e, data + (size - 6 + index as u32) * 16, plane);
        }
    });
}

// Translated from 00de3360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_createConvexDisplayFromPlanes` (Xbox PDB name): turns the plane equations into vertices
/// (`createVerticesFromPlaneEquations`), moves them by `transform`, makes a geometry of them
/// (`createConvexGeometry`, from a strided vertex record `(data, count, 0x10)`) and a display
/// convex over it. Answers 0 (after freeing the vertices) when the planes give no vertex.
/// Register input: `planes` is `ECX`; `transform` is the stack word (the caller removes it).
pub fn create_convex_display_from_planes(e: &mut Engine, planes: Ptr, transform: Ptr) -> u32 {
    e.with_stack(0x24, |e, frame| {
        let vertices = frame.addr();
        let moved = vertices + 0xc;
        let strided = vertices + 0x18;
        e.mem.set_u32(vertices + 8, EMPTY_CAPACITY);
        e.call(CREATE_VERTICES_FROM_PLANES, &args![planes, vertices]);
        let count = e.mem.i32(vertices + 4);
        if count < 1 {
            release_vector_array(e, vertices);
            return 0;
        }
        e.mem.set_u32(moved + 8, EMPTY_CAPACITY);
        let allocator = memory_allocator(e);
        let capacity = (e.mem.u32(moved + 8) & CAPACITY_MASK) as i32;
        if capacity < count {
            let new_capacity = (capacity + capacity).max(count);
            e.call(
                ARRAY_RESERVE,
                &args![allocator, moved, new_capacity, 0x10u32],
            );
        }
        e.mem.set_i32(moved + 4, count);
        for index in 0..count as u32 {
            let target = e.mem.u32(moved) + index * 16;
            let source = e.mem.u32(vertices) + index * 16;
            e.call(SET_TRANSFORMED_POS, &args![target, transform, source]);
        }
        let geometry = allocate(e, 0x18);
        if geometry != 0 {
            for (offset, value) in [
                (0, 0),
                (4, 0),
                (8, EMPTY_CAPACITY),
                (0xc, 0),
                (0x10, 0),
                (0x14, EMPTY_CAPACITY),
            ] {
                e.mem.set_u32(geometry + offset, value);
            }
        }
        let moved_data = e.mem.u32(moved);
        let moved_size = e.mem.u32(moved + 4);
        e.mem.set_u32(strided, moved_data);
        e.mem.set_u32(strided + 4, moved_size);
        e.mem.set_u32(strided + 8, 0x10);
        e.call(CREATE_CONVEX_GEOMETRY, &args![strided, geometry]);
        let block = allocate(e, 0x60);
        e.mem.set_u16(block + 4, 0x60);
        let display = e
            .call(DISPLAY_CONVEX_CONSTRUCTOR, &args![block, geometry])
            .u32();
        release_vector_array(e, moved);
        e.mem.set_u32(moved, 0);
        e.mem.set_u32(moved + 8, EMPTY_CAPACITY);
        release_vector_array(e, vertices);
        display
    })
}

/// The convex radius a convex shape keeps at `+0x10`, as the x87 loads and stores it.
fn convex_radius(e: &Engine, shape: u32) -> f32 {
    x87_quiet(e.mem.f32(shape + SHAPE_RADIUS))
}

/// The cylinder's display: the two end points moved by `transform` and pulled inwards by the
/// convex radius, the display cylinder over them with the sum of both radii.
fn build_cylinder_display(e: &mut Engine, shape: u32, transform: u32, out: u32, radius: f32) {
    let cylinder_radius = e.call(CYLINDER_RADIUS, &args![shape]).f32();
    e.with_stack(0x20, |e, scratch| {
        let first = scratch.addr();
        let second = first + 0x10;
        e.call(SET_TRANSFORMED_POS, &args![first, transform, shape + 0x30]);
        e.call(SET_TRANSFORMED_POS, &args![second, transform, shape + 0x20]);
        let a = load_vec(e, first);
        let b = load_vec(e, second);
        let zero = e.global::<f32>(ZERO);
        let three = e.global::<f32>(THREE);
        let half = e.global::<f32>(HALF);
        let difference = subps(a, b);
        let sum = sum_of_three(mulps(difference, difference));
        let scale = normalising_scale(sum, zero, half, three);
        let shift = mulps(mulps(splat(scale), difference), splat(radius));
        store_vec(e, first, addps(shift, a));
        store_vec(e, second, subps(b, shift));
        let block = allocate(e, 0x90);
        e.mem.set_u16(block + 4, 0x90);
        let total = (f64::from(radius) + f64::from(cylinder_radius)) as f32;
        let display = e
            .call(
                DISPLAY_CYLINDER_CONSTRUCTOR,
                &args![block, first, second, total, 9u32, 1u32],
            )
            .u32();
        e.call(SET_DISPLAY_TRANSFORM, &args![display + 0x10, transform]);
        push_back_pointer(e, out, display);
    });
}

/// The box's display: half extents grown by the convex radius.
fn build_box_display(e: &mut Engine, shape: u32, transform: u32, out: u32, radius: f32) {
    let extents = addps(splat(radius), load_vec(e, shape + 0x20));
    e.with_stack(0x10, |e, scratch| {
        let scratch = scratch.addr();
        store_vec(e, scratch, extents);
        let block = allocate(e, 0x70);
        e.mem.set_u16(block + 4, 0x70);
        let display = e
            .call(DISPLAY_BOX_CONSTRUCTOR, &args![block, scratch])
            .u32();
        e.call(SET_DISPLAY_TRANSFORM, &args![display + 0x10, transform]);
        push_back_pointer(e, out, display);
    });
}

/// `left.yzx * right.zxy - left.zxy * right.yzx` (the `SHUFPS 0xC9`/`0xD2` cross product).
fn cross(left: Vec4, right: Vec4) -> Vec4 {
    subps(
        mulps(shufps(left, left, 0xc9), shufps(right, right, 0xd2)),
        mulps(shufps(left, left, 0xd2), shufps(right, right, 0xc9)),
    )
}

/// Sets the `w` lane of the face plane at `plane` to `0 - dot(((radius * n) + point) * n)` with
/// `n` its normal, copied through the x87.
fn set_face_offset(e: &mut Engine, plane: u32, radius: f32, point: Vec4) {
    let normal = load_vec(e, plane);
    let moved = mulps(addps(mulps(splat(radius), normal), point), normal);
    let zero = e.mem.f32(ZERO_VECTOR);
    e.mem
        .set_f32(plane + 0xc, x87_quiet(zero - sum_of_three(moved)));
}

/// A bisector plane: the sum of two stored planes, scaled by the refined `RSQRTSS` of its length,
/// its offset then set from `point`.
fn set_bisector_plane(
    e: &mut Engine,
    target: u32,
    (first, second): (u32, u32),
    radius: f32,
    point: Vec4,
) {
    let zero = e.global::<f32>(ZERO);
    let three = e.global::<f32>(THREE);
    let half = e.global::<f32>(HALF);
    let sum = addps(load_vec(e, first), load_vec(e, second));
    let scale = normalising_scale(sum_of_three(mulps(sum, sum)), zero, half, three);
    store_vec(e, target, mulps(splat(scale), sum));
    set_face_offset(e, target, radius, point);
}

/// Makes the plane array of the triangle at `array` (`{data, size, capacity}`): a block of
/// `0x80` bytes asked of the allocator through its buffer function, the size and capacity set to
/// the number of planes it holds (the capacity word `0x80000000` for none), the capacity widened
/// to eight when it is smaller, and the size set to eight.
fn make_eight_plane_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
    let allocator = memory_allocator(e);
    let (data, count) = e.with_stack(4, |e, cell| {
        let cell = cell.addr();
        e.mem.set_u32(cell, 0x80);
        let data = e.vcall(allocator, 0xc, &args![cell]).u32();
        (data, sixteen_byte_elements(e.mem.i32(cell)))
    });
    let capacity_word = if count == 0 {
        EMPTY_CAPACITY
    } else {
        count as u32
    };
    e.mem.set_u32(array, data);
    e.mem.set_i32(array + 4, count);
    e.mem.set_u32(array + 8, capacity_word);
    let allocator = memory_allocator(e);
    let capacity = (capacity_word & CAPACITY_MASK) as i32;
    if capacity < 8 {
        let new_capacity = if capacity + capacity > 8 {
            capacity + capacity
        } else {
            8
        };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, 0x10u32],
        );
    }
    e.mem.set_u32(array + 4, 8);
}

/// The triangle's display: eight planes (the two faces, the three sides and the three corner
/// bisectors) pushed out by the convex radius, then a display convex over them. A degenerate
/// triangle (`hkpTriangleUtil::isDegenerate` with the epsilon at `01017d00`) gets none.
fn build_triangle_display(e: &mut Engine, shape: u32, transform: u32, out: u32, radius: f32) {
    let (vertex_a, vertex_b, vertex_c) = (shape + 0x20, shape + 0x30, shape + 0x40);
    let epsilon = x87_quiet_bits(e.mem.u32(TRIANGLE_EPSILON));
    let degenerate = e.with_stack(1, |e, flag| {
        let arguments = args![flag, vertex_a, vertex_b, vertex_c, epsilon];
        let result = e.call(TRIANGLE_IS_DEGENERATE, &arguments).u32();
        e.mem.u8(result) != 0
    });
    if degenerate {
        return;
    }
    let a = load_vec(e, vertex_a);
    let b = load_vec(e, vertex_b);
    let c = load_vec(e, vertex_c);
    let zero = e.global::<f32>(ZERO);
    let three = e.global::<f32>(THREE);
    let half = e.global::<f32>(HALF);
    let direction = |from: Vec4, to: Vec4| {
        let difference = subps(to, from);
        let sum = sum_of_three(mulps(difference, difference));
        mulps(splat(normalising_scale(sum, zero, half, three)), difference)
    };
    let side_ab = direction(a, b);
    let side_bc = direction(b, c);
    let side_ca = direction(a, c);
    let normal = cross(side_ab, side_ca);
    e.with_stack(12, |e, array| {
        let array = array.addr();
        make_eight_plane_array(e, array);
        let data = e.mem.u32(array);
        let sign: Vec4 =
            std::array::from_fn(|lane| f32::from_bits(e.mem.u32(SIGN_MASK + 4 * lane as u32)));
        let flipped: Vec4 = std::array::from_fn(|lane| {
            f32::from_bits(normal[lane].to_bits() ^ sign[lane].to_bits())
        });
        let planes = [
            (normal, a),
            (flipped, a),
            (cross(side_ab, normal), a),
            (cross(side_bc, normal), b),
            (cross(normal, side_ca), c),
        ];
        for (index, (plane, point)) in planes.into_iter().enumerate() {
            let target = data + 0x10 * index as u32;
            store_vec(e, target, plane);
            set_face_offset(e, target, radius, point);
        }
        set_bisector_plane(e, data + 0x50, (data + 0x30, data + 0x20), radius, b);
        set_bisector_plane(e, data + 0x60, (data + 0x40, data + 0x30), radius, c);
        set_bisector_plane(e, data + 0x70, (data + 0x40, data + 0x20), radius, a);
        let display = create_convex_display_from_planes(e, Ptr::new(array), Ptr::new(transform));
        if display != 0 {
            push_back_pointer(e, out, display);
        }
        release_vector_array(e, array);
    });
}

/// The convex vertices shape's display: a copy of its plane equations, every offset moved inwards
/// by the convex radius when the farthest vertex is farther from the planes than `0.005 -
/// radius`, the six faces of its box added, and a display convex over them.
fn build_convex_vertices_display(
    e: &mut Engine,
    shape: u32,
    transform: u32,
    out: u32,
    radius: f32,
) {
    let planes = e.call(GET_PLANE_EQUATIONS, &args![shape]).u32();
    let surface = get_plane_to_surface_distance(e, Ptr::new(shape));
    e.with_stack(12, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array + 8, EMPTY_CAPACITY);
        let allocator = memory_allocator(e);
        let plane_count = e.mem.i32(planes + 4);
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity < plane_count {
            if e.mem.i32(array + 8) >= 0 {
                let data = e.mem.u32(array);
                e.vcall(allocator, 0x10, &args![data, (capacity as u32) << 4]);
            }
            let (data, count) = e.with_stack(4, |e, cell| {
                let cell = cell.addr();
                e.mem.set_u32(cell, (plane_count as u32) << 4);
                let data = e.vcall(allocator, 0xc, &args![cell]).u32();
                (data, sixteen_byte_elements(e.mem.i32(cell)))
            });
            e.mem.set_u32(array, data);
            e.mem.set_i32(array + 8, count);
        }
        e.mem.set_i32(array + 4, plane_count);
        let data = e.mem.u32(array);
        let source = e.mem.u32(planes);
        let count = plane_count.max(0) as u32;
        for index in 0..count {
            let plane = load_vec(e, source + index * 16);
            store_vec(e, data + index * 16, plane);
        }
        let margin = (e.mem.f64(RADIUS_MARGIN) - f64::from(radius)) as f32;
        if margin < surface {
            for index in 0..count {
                let w = data + index * 16 + 0xc;
                let moved = (f64::from(e.mem.f32(w)) - f64::from(radius)) as f32;
                e.mem.set_f32(w, moved);
            }
        }
        add_bounding_planes(e, Ptr::new(array), Ptr::new(shape), radius);
        let display = create_convex_display_from_planes(e, Ptr::new(array), Ptr::new(transform));
        if display != 0 {
            push_back_pointer(e, out, display);
        }
        release_vector_array(e, array);
    });
}

/// Builds the displays of every child of a shape container and appends them to `out`. The
/// children are asked for with `getChildShape(key, buffer)` into one `0x200`-byte buffer.
fn build_container_children(e: &mut Engine, this: u32, container: u32, transform: u32, out: u32) {
    e.with_stack(0x200, |e, buffer| {
        let buffer = buffer.addr();
        let mut key = e.vcall(container, 8, &args![]).i32();
        while key != -1 {
            let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
            build_child(e, this, child, transform, out);
            key = e.vcall(container, 0xc, &args![key]).i32();
        }
    });
}

fn build_child(e: &mut Engine, this: u32, child: u32, transform: u32, out: u32) {
    hkp_convex_radius_builder_build_shape_display(
        e,
        Ptr::new(this),
        Ptr::new(child),
        Ptr::new(transform),
        Ptr::new(out),
    );
}

/// Builds the display of a shape wrapped with a transform: the transform composed with `local`
/// (`setMul(parent, local)`), then the display of `child` under the result.
fn build_transformed_child(
    e: &mut Engine,
    this: u32,
    (child, local): (u32, u32),
    transform: u32,
    out: u32,
) {
    e.with_stack(0x40, |e, composed| {
        let composed = composed.addr();
        e.call(TRANSFORM_SET_MUL, &args![composed, transform, local]);
        build_child(e, this, child, composed, out);
    });
}

// Translated from 00de3520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusBuilder::buildShapeDisplay` (Xbox PDB): appends the display geometries of
/// `shape` (placed by `transform`) to the pointer array `out`. By the shape's type (`+0xC`):
/// the cylinder (2), triangle (3), box (4) and convex vertices (6) get a display when the
/// builder's tolerance (`+8`) is below their convex radius (`+0x10`); the types 7 to 10, 22 and
/// 23 recurse on every child their container (`vtable + 0x10`) gives; types 11, 12 and 15
/// compose their transform with the child's and recurse; 27 recurses on the shape it wraps; type
/// 20 recurses on every shape of its list when its radius is above the tolerance; types 1, 5, 13,
/// 21, 24 and 28 build nothing; every other type is looked up in the user builder's table (global
/// `01269ce4`) and handed to each function registered for it with `(shape, transform, out,
/// builder)`.
pub fn hkp_convex_radius_builder_build_shape_display(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    transform: Ptr,
    out: Ptr,
) {
    let (this, shape, transform, out) = (this.addr(), shape.addr(), transform.addr(), out.addr());
    let shape_type = e.mem.u32(shape + SHAPE_TYPE);
    let tolerance = e.mem.f32(this + BUILDER_TOLERANCE);
    match shape_type {
        2 | 3 | 4 | 6 => {
            let radius = convex_radius(e, shape);
            if tolerance < radius {
                match shape_type {
                    2 => build_cylinder_display(e, shape, transform, out, radius),
                    3 => build_triangle_display(e, shape, transform, out, radius),
                    4 => build_box_display(e, shape, transform, out, radius),
                    _ => build_convex_vertices_display(e, shape, transform, out, radius),
                }
            }
        }
        7..=10 | 22 | 23 => {
            let container = e.vcall(shape, 0x10, &args![]).u32();
            build_container_children(e, this, container, transform, out);
        }
        11 => e.with_stack(0x40, |e, local| {
            let local = local.addr();
            for (row, lane) in [(0, 0), (0x10, 1), (0x20, 2)] {
                for column in 0..4 {
                    let value = if column == lane { 1.0f32 } else { 0.0 };
                    e.mem.set_f32(local + row + 4 * column, value);
                }
            }
            let translation = load_vec(e, shape + 0x20);
            store_vec(e, local + 0x30, translation);
            let child = e.mem.u32(shape + 0x18);
            build_transformed_child(e, this, (child, local), transform, out);
        }),
        12 => {
            let child = e.mem.u32(shape + 0x18);
            build_transformed_child(e, this, (child, shape + 0x20), transform, out);
        }
        15 => {
            let child = e.mem.u32(shape + 0x14);
            build_transformed_child(e, this, (child, shape + 0x30), transform, out);
        }
        20 => {
            if tolerance < e.mem.f32(shape + SHAPE_RADIUS) {
                e.with_stack(0x200, |e, buffer| {
                    let buffer = buffer.addr();
                    let mut index = 0u32;
                    while (index as i32) < e.mem.i32(shape + 0x28) {
                        let container = e.mem.u32(shape + 0x20) + 0x10;
                        let keys = e.mem.u32(shape + 0x24);
                        let key = e.mem.u32(keys + index * 4);
                        let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
                        build_child(e, this, child, transform, out);
                        index += 1;
                    }
                });
            }
        }
        27 => {
            let child = e.mem.u32(shape + 0x10);
            build_child(e, this, child, transform, out);
        }
        1 | 5 | 13 | 21 | 24 | 28 => {}
        _ => {
            let mut list = e.mem.u32(USER_BUILDER);
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(list + 0xc) {
                let entries = e.mem.u32(list + 8);
                let function = e.mem.u32(entries + index * 8);
                let kind = e.mem.u32(entries + index * 8 + 4);
                if kind == shape_type {
                    e.call(function, &args![shape, transform, out, this]);
                    list = e.mem.u32(USER_BUILDER);
                }
                index += 1;
            }
        }
    }
}

// Translated from 00de4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the builder's cached display convex (`+0xC`), making it first when there is none: an
/// empty two-array geometry (`0x18` bytes), a display convex over it (`0x60` bytes, `u16` size
/// `0x60` at `+4`), stored in the builder and appended to the pointer array `list`.
pub fn fn_00de4340(e: &mut Engine, this: Ptr, list: Ptr) -> u32 {
    let this = this.addr();
    if e.mem.u32(this + BUILDER_DISPLAY_CONVEX) == 0 {
        let geometry = allocate(e, 0x18);
        if geometry != 0 {
            for (offset, value) in [
                (0, 0),
                (4, 0),
                (8, EMPTY_CAPACITY),
                (0xc, 0),
                (0x10, 0),
                (0x14, EMPTY_CAPACITY),
            ] {
                e.mem.set_u32(geometry + offset, value);
            }
        }
        let block = allocate(e, 0x60);
        e.mem.set_u16(block + 4, 0x60);
        let display = e
            .call(DISPLAY_CONVEX_CONSTRUCTOR, &args![block, geometry])
            .u32();
        e.mem.set_u32(this + BUILDER_DISPLAY_CONVEX, display);
        push_back_pointer(e, list.addr(), display);
    }
    e.mem.u32(this + BUILDER_DISPLAY_CONVEX)
}

// Translated from 00de4420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusBuilder::buildDisplayGeometries` (Xbox PDB): forgets the cached display
/// convex, empties the pointer array `out` and builds the display of `shape` placed by the
/// identity transform.
pub fn hkp_convex_radius_builder_build_display_geometries(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    out: Ptr,
) {
    e.with_stack(0x40, |e, identity| {
        let identity = identity.addr();
        for row in 0..3u32 {
            e.mem.set_f32(identity + 0x14 * row, 1.0);
        }
        fn_00de3090(e, this);
        e.mem.set_u32(out.addr() + 4, 0);
        hkp_convex_radius_builder_build_shape_display(e, this, shape, Ptr::new(identity), out);
    });
}

// Translated from 00de43e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x14 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, the vtable word `010d8224` at +0, two zero words at +8 and +0xc and `0x80000000` at +0x10 (an empty array that owns no storage). Returns the block. Ghidra made no function here; the address is stored in the data word at 011b8664.
pub fn fn_00de43e0(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010d_8224);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0x8000_0000);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00de10e0,
            hkp_user_convex_radius_builder_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00de3090, fn_00de3090(Ptr)),
        entry!(0x00de3110, get_plane_to_surface_distance(Ptr) -> f32),
        entry!(0x00de3200, add_bounding_planes(Ptr, Ptr, f32)),
        entry!(0x00de3360, create_convex_display_from_planes(Ptr, Ptr) -> u32),
        entry!(
            0x00de3520,
            hkp_convex_radius_builder_build_shape_display(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00de4340, fn_00de4340(Ptr, Ptr) -> u32),
        entry!(
            0x00de4420,
            hkp_convex_radius_builder_build_display_geometries(Ptr, Ptr, Ptr)
        ),
        entry!(0x00de_43e0, fn_00de43e0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATOR: u32 = 0x0900_0000;
    /// A shape type no case of the builder knows: only the user builder's functions see it.
    const USER_TYPE: u32 = 30;
    const USER_FUNCTION: u32 = 0x0910_0000;
    const OTHER_USER_FUNCTION: u32 = 0x0910_0100;

    /// A double at `address` that logs its arguments and answers what `body` computes.
    fn double(
        e: &mut Engine,
        log: &Log,
        address: u32,
        body: impl Fn(&mut Engine, &[u32]) -> u32 + 'static,
    ) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: body(e, a),
                ..Ret::default()
            }
        });
    }

    struct World {
        e: Engine,
        log: Log,
        allocator: u32,
        builder: u32,
        out: u32,
    }

    /// The constants, a memory router over a logging allocator (slot 4 hands out zeroed blocks,
    /// slot `0xC` the block for the size in the cell it is given), a reserve that really moves
    /// the array, a builder with tolerance `0.01` and an empty output array with room for 16.
    fn world() -> World {
        let mut e = engine();
        let log = new_log();
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(MINUS_ZERO, -0.0f32);
        e.set_global(ONE, 1.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(MINUS_THOUSAND, -1000.0f32);
        e.set_global(TRIANGLE_EPSILON, 0.001f32);
        e.set_global(RADIUS_MARGIN, 0.005f64);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        double(&mut e, &log, ALLOCATOR + 4, |e, a| e.mem.alloc(a[1]));
        double(&mut e, &log, ALLOCATOR + 0xc, |e, a| {
            let size = e.mem.u32(a[1]);
            e.mem.alloc(size)
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        double(&mut e, &log, ARRAY_RESERVE, |e, a| {
            let (array, capacity, size) = (a[1], a[2], a[3]);
            let data = e.mem.alloc(capacity * size);
            let old = e.mem.u32(array);
            let used = e.mem.u32(array + 4) * size;
            for offset in 0..used {
                let byte = e.mem.u8(old + offset);
                e.mem.set_u8(data + offset, byte);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            0
        });
        let builder = e.mem.alloc(0x20);
        e.mem.set_f32(builder + BUILDER_TOLERANCE, 0.01);
        let out = e.mem.alloc(0xc);
        let room = e.mem.alloc(0x40);
        e.mem.set_u32(out, room);
        e.mem.set_u32(out + 8, 16);
        World {
            e,
            log,
            allocator,
            builder,
            out,
        }
    }

    fn make_shape(e: &mut Engine, shape_type: u32, radius: f32) -> u32 {
        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(shape + SHAPE_TYPE, shape_type);
        e.mem.set_f32(shape + SHAPE_RADIUS, radius);
        shape
    }

    /// The user builder with a function for `USER_TYPE` (and another for the next type).
    fn user_builder(e: &mut Engine) {
        let list = e.mem.alloc(0x20);
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(entries, OTHER_USER_FUNCTION);
        e.mem.set_u32(entries + 4, USER_TYPE + 1);
        e.mem.set_u32(entries + 8, USER_FUNCTION);
        e.mem.set_u32(entries + 12, USER_TYPE);
        e.mem.set_u32(list + 8, entries);
        e.mem.set_u32(list + 0xc, 2);
        e.mem.set_u32(USER_BUILDER, list);
    }

    fn pointers(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    fn build(world: &mut World, shape: u32, transform: u32) {
        let (builder, out) = (world.builder, world.out);
        world
            .e
            .call(0x00de_3520, &args![builder, shape, transform, out]);
    }

    fn assert_close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-3,
                "{actual:?} != {expected:?}"
            );
        }
    }

    #[test]
    fn destructor_frees_the_entries_and_the_object_00de10e0() {
        let mut world = world();
        let allocator = world.allocator;
        let (e, log) = (&mut world.e, &world.log);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x18);
        e.mem.set_u32(this + 8, 0x1234);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, 5);
        let result = e.call(0x00de_10e0, &args![this, 1u32]).u32();
        assert_eq!(result, this);
        assert_eq!(
            calls(log, ALLOCATOR + 0x10),
            vec![vec![allocator, 0x1234, 40]]
        );
        assert_eq!(calls(log, ALLOCATOR + 8), vec![vec![allocator, this, 0x18]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), EMPTY_CAPACITY);
        // Data the array does not own is not freed, and without the flag the object stays.
        e.mem.set_u32(this + 8, 0x4321);
        e.mem.set_u32(this + 0x10, 0x8000_0005);
        e.call(0x00de_10e0, &args![this, 0u32]);
        assert_eq!(calls(log, ALLOCATOR + 0x10).len(), 1);
        assert_eq!(calls(log, ALLOCATOR + 8).len(), 1);
    }

    #[test]
    fn forgetting_the_display_convex_00de3090() {
        let mut world = world();
        let builder = world.builder;
        world.e.mem.set_u32(builder + 0xc, 0x77);
        world.e.call(0x00de_3090, &args![builder]);
        assert_eq!(world.e.mem.u32(builder + 0xc), 0);
    }

    #[test]
    fn plane_to_surface_distance_is_the_largest_offset_00de3110() {
        let mut world = world();
        let allocator = world.allocator;
        let (e, log) = (&mut world.e, &world.log);
        let planes = e.mem.alloc(12);
        let plane_data = e.mem.alloc(32);
        store_vec(e, plane_data, [0.0, 1.0, 0.0, -2.0]);
        store_vec(e, plane_data + 16, [1.0, 0.0, 0.0, -10.0]);
        e.mem.set_u32(planes, plane_data);
        e.mem.set_u32(planes + 4, 2);
        double(e, log, GET_PLANE_EQUATIONS, move |_, _| planes);
        double(e, log, GET_ORIGINAL_VERTICES, |e, a| {
            let data = e.mem.alloc(32);
            store_vec(e, data, [0.0, 5.0, 0.0, 0.0]);
            store_vec(e, data + 16, [1.0, 3.0, 0.0, 0.0]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 5);
            0
        });
        let shape = make_shape(e, 6, 0.1);
        let distance = e.call(0x00de_3110, &args![shape]).f32();
        assert_eq!(distance, 3.0);
        let freed = calls(log, ALLOCATOR + 0x10);
        assert_eq!(freed.len(), 1);
        assert_eq!((freed[0][0], freed[0][2]), (allocator, 80));
        // With no vertex the answer is the start value.
        double(e, log, GET_ORIGINAL_VERTICES, |_, _| 0);
        assert_eq!(e.call(0x00de_3110, &args![shape]).f32(), -1000.0);
    }

    #[test]
    fn bounding_planes_are_the_six_faces_of_the_box_00de3200() {
        let mut world = world();
        let allocator = world.allocator;
        let (e, log) = (&mut world.e, &world.log);
        let shape = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, 0x0920_0000);
        e.mem.set_u32(shape, vtable);
        double(e, log, 0x0920_0000, |e, a| {
            store_vec(e, a[3], [-1.0, -2.0, -3.0, 0.0]);
            store_vec(e, a[3] + 16, [4.0, 5.0, 6.0, 0.0]);
            0
        });
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(32);
        store_vec(e, data, [9.0, 9.0, 9.0, 9.0]);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 2);
        e.call(0x00de_3200, &args![array, shape, 0.25f32]);
        let aabb_call = &calls(log, 0x0920_0000)[0];
        assert_eq!(
            (aabb_call[0], aabb_call[1], aabb_call[2]),
            (shape, IDENTITY_TRANSFORM, 0.25f32.to_bits())
        );
        assert_eq!(
            calls(log, ARRAY_RESERVE),
            vec![vec![allocator, array, 7, 16]]
        );
        assert_eq!(e.mem.u32(array + 4), 7);
        let data = e.mem.u32(array);
        assert_eq!(load_vec(e, data), [9.0, 9.0, 9.0, 9.0]);
        let expected = [
            [1.0, 0.0, 0.0, -4.0],
            [0.0, 1.0, 0.0, -5.0],
            [0.0, 0.0, 1.0, -6.0],
            [-1.0, 0.0, 0.0, -1.0],
            [0.0, -1.0, 0.0, -2.0],
            [0.0, 0.0, -1.0, -3.0],
        ];
        for (index, plane) in expected.into_iter().enumerate() {
            assert_eq!(load_vec(e, data + 16 * (index as u32 + 1)), plane);
        }
    }

    /// Doubles for the plane-to-vertices path: two vertices (1,2,3) and (4,5,6), and a
    /// transformed copy that adds 10 to x.
    fn plane_doubles(world: &mut World) -> Seen {
        let record = seen();
        let noted = Rc::clone(&record);
        let (e, log) = (&mut world.e, &world.log);
        double(e, log, CREATE_VERTICES_FROM_PLANES, |e, a| {
            let data = e.mem.alloc(32);
            store_vec(e, data, [1.0, 2.0, 3.0, 0.0]);
            store_vec(e, data + 16, [4.0, 5.0, 6.0, 0.0]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            0
        });
        double(e, log, SET_TRANSFORMED_POS, |e, a| {
            let mut v = load_vec(e, a[2]);
            v[0] += 10.0;
            store_vec(e, a[0], v);
            0
        });
        double(e, log, CREATE_CONVEX_GEOMETRY, move |e, a| {
            let data = e.mem.u32(a[0]);
            let count = e.mem.u32(a[0] + 4);
            let mut notes = noted.borrow_mut();
            notes.push([count as f32, e.mem.u32(a[0] + 8) as f32, 0.0, 0.0]);
            for index in 0..count {
                notes.push(load_vec(e, data + 16 * index));
            }
            0
        });
        double(e, log, DISPLAY_CONVEX_CONSTRUCTOR, |_, _| 0x7777);
        record
    }

    #[test]
    fn display_from_planes_moves_the_vertices_and_wraps_them_00de3360() {
        let mut world = world();
        let record = plane_doubles(&mut world);
        let allocator = world.allocator;
        let (e, log) = (&mut world.e, &world.log);
        let planes = e.mem.alloc(12);
        let transform = e.mem.alloc(0x40);
        let display = e.call(0x00de_3360, &args![planes, transform]).u32();
        assert_eq!(display, 0x7777);
        let geometry_call = &calls(log, CREATE_CONVEX_GEOMETRY)[0];
        assert_eq!(
            *record.borrow(),
            vec![
                [2.0, 16.0, 0.0, 0.0],
                [11.0, 2.0, 3.0, 0.0],
                [14.0, 5.0, 6.0, 0.0]
            ]
        );
        // The geometry block was freed with its arrays empty (checked through the constructor).
        let constructor = &calls(log, DISPLAY_CONVEX_CONSTRUCTOR)[0];
        let geometry = constructor[1];
        assert_eq!(geometry, geometry_call[1]);
        assert_eq!(e.mem.u32(geometry + 8), EMPTY_CAPACITY);
        assert_eq!(e.mem.u32(geometry + 0x14), EMPTY_CAPACITY);
        assert_eq!(e.mem.u16(constructor[0] + 4), 0x60);
        assert_eq!(
            calls(log, ALLOCATOR + 4)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            vec![0x18, 0x60]
        );
        let moved = calls(log, SET_TRANSFORMED_POS);
        assert_eq!(moved.len(), 2);
        assert_eq!(moved[1][0], moved[0][0] + 16);
        assert_eq!(moved[0][1], transform);
        // Both temporary arrays were freed through the allocator (32 bytes each).
        assert_eq!(calls(log, ALLOCATOR + 0x10).len(), 2);
        assert!(calls(log, ALLOCATOR + 0x10)
            .iter()
            .all(|c| c[0] == allocator && c[2] == 32));
    }

    #[test]
    fn display_from_planes_answers_0_without_vertices_00de3360() {
        let mut world = world();
        let _ = plane_doubles(&mut world);
        let (e, log) = (&mut world.e, &world.log);
        double(e, log, CREATE_VERTICES_FROM_PLANES, |_, _| 0);
        let planes = e.mem.alloc(12);
        let transform = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00de_3360, &args![planes, transform]).u32(), 0);
        assert!(calls(log, DISPLAY_CONVEX_CONSTRUCTOR).is_empty());
        assert!(calls(log, ALLOCATOR + 4).is_empty());
    }

    #[test]
    fn types_that_build_nothing_and_the_user_builder_00de3520() {
        let mut w = world();
        user_builder(&mut w.e);
        double(&mut w.e, &w.log, USER_FUNCTION, |_, _| 0);
        double(&mut w.e, &w.log, OTHER_USER_FUNCTION, |_, _| 0);
        let transform = w.e.mem.alloc(0x40);
        for shape_type in [1, 5, 13, 21, 24, 28] {
            let shape = make_shape(&mut w.e, shape_type, 0.5);
            build(&mut w, shape, transform);
        }
        assert!(w.log.borrow().is_empty());
        let shape = make_shape(&mut w.e, USER_TYPE, 0.5);
        build(&mut w, shape, transform);
        assert_eq!(
            calls(&w.log, USER_FUNCTION),
            vec![vec![shape, transform, w.out, w.builder]]
        );
        assert!(calls(&w.log, OTHER_USER_FUNCTION).is_empty());
    }

    #[test]
    fn wrapper_and_transform_shapes_recurse_00de3520() {
        let mut w = world();
        user_builder(&mut w.e);
        double(&mut w.e, &w.log, USER_FUNCTION, |_, _| 0);
        double(&mut w.e, &w.log, TRANSFORM_SET_MUL, |_, _| 0);
        let transform = w.e.mem.alloc(0x40);
        let child = make_shape(&mut w.e, USER_TYPE, 0.5);
        // Type 27 hands the same transform on.
        let wrapper = make_shape(&mut w.e, 27, 0.0);
        w.e.mem.set_u32(wrapper + 0x10, child);
        build(&mut w, wrapper, transform);
        assert_eq!(
            calls(&w.log, USER_FUNCTION),
            vec![vec![child, transform, w.out, w.builder]]
        );
        // Types 12 and 15 compose the transform with their own at +0x20 / +0x30.
        for (shape_type, child_offset, local_offset) in [(12, 0x18, 0x20), (15, 0x14, 0x30)] {
            w.log.borrow_mut().clear();
            let shape = make_shape(&mut w.e, shape_type, 0.0);
            w.e.mem.set_u32(shape + child_offset, child);
            build(&mut w, shape, transform);
            let composed = calls(&w.log, TRANSFORM_SET_MUL)[0][0];
            assert_eq!(
                calls(&w.log, TRANSFORM_SET_MUL),
                vec![vec![composed, transform, shape + local_offset]]
            );
            assert_eq!(
                calls(&w.log, USER_FUNCTION),
                vec![vec![child, composed, w.out, w.builder]]
            );
        }
    }

    #[test]
    fn translation_shape_composes_an_identity_rotation_00de3520() {
        let mut w = world();
        user_builder(&mut w.e);
        double(&mut w.e, &w.log, USER_FUNCTION, |_, _| 0);
        let seen: Rc<RefCell<Vec<f32>>> = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        double(&mut w.e, &w.log, TRANSFORM_SET_MUL, move |e, a| {
            *record.borrow_mut() = (0..16).map(|word| e.mem.f32(a[2] + 4 * word)).collect();
            0
        });
        let transform = w.e.mem.alloc(0x40);
        let child = make_shape(&mut w.e, USER_TYPE, 0.5);
        let shape = make_shape(&mut w.e, 11, 0.0);
        w.e.mem.set_u32(shape + 0x18, child);
        store_vec(&mut w.e, shape + 0x20, [5.0, 6.0, 7.0, 8.0]);
        build(&mut w, shape, transform);
        assert_eq!(
            *seen.borrow(),
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 5.0, 6.0, 7.0, 8.0]
        );
        let composed = calls(&w.log, TRANSFORM_SET_MUL)[0][0];
        assert_eq!(
            calls(&w.log, USER_FUNCTION),
            vec![vec![child, composed, w.out, w.builder]]
        );
    }

    /// A container whose keys are 100 and 101 (then -1) with the children `first` and `second`.
    fn container(world: &mut World, first: u32, second: u32) -> u32 {
        const BASE: u32 = 0x0930_0000;
        let vtable = world.e.mem.alloc(0x20);
        for slot in [8, 0xc, 0x14] {
            world.e.mem.set_u32(vtable + slot, BASE + slot);
        }
        double(&mut world.e, &world.log, BASE + 8, |_, _| 100);
        double(&mut world.e, &world.log, BASE + 0xc, |_, a| {
            if a[1] == 100 {
                101
            } else {
                u32::MAX
            }
        });
        double(&mut world.e, &world.log, BASE + 0x14, move |_, a| {
            if a[1] == 100 {
                first
            } else {
                second
            }
        });
        let object = world.e.mem.alloc(0x20);
        world.e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn container_shapes_build_every_child_00de3520() {
        for shape_type in [7, 8, 9, 10, 22, 23] {
            let mut w = world();
            user_builder(&mut w.e);
            double(&mut w.e, &w.log, USER_FUNCTION, |_, _| 0);
            let transform = w.e.mem.alloc(0x40);
            let first = make_shape(&mut w.e, USER_TYPE, 0.5);
            let second = make_shape(&mut w.e, USER_TYPE, 0.5);
            let object = container(&mut w, first, second);
            let shape = make_shape(&mut w.e, shape_type, 0.5);
            let shape_vtable = w.e.mem.alloc(0x20);
            w.e.mem.set_u32(shape_vtable + 0x10, 0x0931_0000);
            w.e.mem.set_u32(shape, shape_vtable);
            double(&mut w.e, &w.log, 0x0931_0000, move |_, _| object);
            build(&mut w, shape, transform);
            let (out, builder) = (w.out, w.builder);
            assert_eq!(
                calls(&w.log, USER_FUNCTION),
                vec![
                    vec![first, transform, out, builder],
                    vec![second, transform, out, builder]
                ]
            );
            let buffers: Vec<u32> = calls(&w.log, 0x0930_0014).iter().map(|c| c[2]).collect();
            assert_eq!(buffers[0], buffers[1]);
        }
    }

    #[test]
    fn collection_shape_needs_a_radius_above_the_tolerance_00de3520() {
        for (radius, built) in [(0.5f32, 2), (0.001, 0)] {
            let mut w = world();
            user_builder(&mut w.e);
            double(&mut w.e, &w.log, USER_FUNCTION, |_, _| 0);
            let transform = w.e.mem.alloc(0x40);
            let first = make_shape(&mut w.e, USER_TYPE, 0.5);
            let second = make_shape(&mut w.e, USER_TYPE, 0.5);
            let object = container(&mut w, first, second);
            let shape = make_shape(&mut w.e, 20, radius);
            // The container is embedded 0x10 bytes into the object the shape points to.
            let holder = w.e.mem.alloc(0x30);
            let vtable = w.e.mem.u32(object);
            w.e.mem.set_u32(holder + 0x10, vtable);
            let keys = w.e.mem.alloc(8);
            w.e.mem.set_u32(keys, 100);
            w.e.mem.set_u32(keys + 4, 101);
            w.e.mem.set_u32(shape + 0x20, holder);
            w.e.mem.set_u32(shape + 0x24, keys);
            w.e.mem.set_u32(shape + 0x28, 2);
            build(&mut w, shape, transform);
            let children = calls(&w.log, USER_FUNCTION);
            assert_eq!(children.len(), built);
            if built == 2 {
                assert_eq!((children[0][0], children[1][0]), (first, second));
                assert_eq!(calls(&w.log, 0x0930_0014)[0][0], holder + 0x10);
            }
        }
    }

    type Seen = Rc<RefCell<Vec<Vec4>>>;

    fn seen() -> Seen {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn cylinder_display_is_pulled_in_by_the_radius_00de3520() {
        let mut w = world();
        let record = seen();
        let noted = Rc::clone(&record);
        w.e.register(CYLINDER_RADIUS, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        double(&mut w.e, &w.log, SET_TRANSFORMED_POS, |e, a| {
            let v = load_vec(e, a[2]);
            store_vec(e, a[0], v);
            0
        });
        double(
            &mut w.e,
            &w.log,
            DISPLAY_CYLINDER_CONSTRUCTOR,
            move |e, a| {
                noted.borrow_mut().push(load_vec(e, a[1]));
                noted.borrow_mut().push(load_vec(e, a[2]));
                0x5555_0000
            },
        );
        double(&mut w.e, &w.log, SET_DISPLAY_TRANSFORM, |_, _| 0);
        let transform = w.e.mem.alloc(0x40);
        let shape = make_shape(&mut w.e, 2, 0.25);
        store_vec(&mut w.e, shape + 0x30, [0.0, 0.0, 2.0, 0.0]);
        store_vec(&mut w.e, shape + 0x20, [0.0, 0.0, 0.0, 0.0]);
        build(&mut w, shape, transform);
        let ends = record.borrow().clone();
        assert_close(ends[0], [0.0, 0.0, 2.25, 0.0]);
        assert_close(ends[1], [0.0, 0.0, -0.25, 0.0]);
        let constructor = &calls(&w.log, DISPLAY_CYLINDER_CONSTRUCTOR)[0];
        assert_eq!(&constructor[3..], &[0.75f32.to_bits(), 9, 1]);
        assert_eq!(w.e.mem.u16(constructor[0] + 4), 0x90);
        assert_eq!(
            calls(&w.log, SET_DISPLAY_TRANSFORM),
            vec![vec![0x5555_0010, transform]]
        );
        assert_eq!(pointers(&w.e, w.out), vec![0x5555_0000]);
        // A tolerance at or above the radius builds nothing.
        w.e.mem.set_f32(w.builder + BUILDER_TOLERANCE, 0.25);
        build(&mut w, shape, transform);
        assert_eq!(pointers(&w.e, w.out).len(), 1);
    }

    #[test]
    fn box_display_has_the_half_extents_grown_by_the_radius_00de3520() {
        let mut w = world();
        let record = seen();
        let noted = Rc::clone(&record);
        double(&mut w.e, &w.log, DISPLAY_BOX_CONSTRUCTOR, move |e, a| {
            noted.borrow_mut().push(load_vec(e, a[1]));
            0x5656_0000
        });
        double(&mut w.e, &w.log, SET_DISPLAY_TRANSFORM, |_, _| 0);
        let transform = w.e.mem.alloc(0x40);
        let shape = make_shape(&mut w.e, 4, 0.25);
        store_vec(&mut w.e, shape + 0x20, [1.0, 2.0, 3.0, 0.5]);
        build(&mut w, shape, transform);
        assert_eq!(*record.borrow(), vec![[1.25, 2.25, 3.25, 0.75]]);
        let constructor = &calls(&w.log, DISPLAY_BOX_CONSTRUCTOR)[0];
        assert_eq!(w.e.mem.u16(constructor[0] + 4), 0x70);
        assert_eq!(
            calls(&w.log, SET_DISPLAY_TRANSFORM),
            vec![vec![0x5656_0010, transform]]
        );
        assert_eq!(pointers(&w.e, w.out), vec![0x5656_0000]);
    }

    /// Makes `CREATE_VERTICES_FROM_PLANES` note the eight planes it is given and answer no
    /// vertex, so the display of a plane list is never made.
    fn note_planes(world: &mut World) -> Seen {
        let record = seen();
        let noted = Rc::clone(&record);
        double(
            &mut world.e,
            &world.log,
            CREATE_VERTICES_FROM_PLANES,
            move |e, a| {
                let data = e.mem.u32(a[0]);
                for index in 0..e.mem.u32(a[0] + 4) {
                    noted.borrow_mut().push(load_vec(e, data + 16 * index));
                }
                0
            },
        );
        record
    }

    #[test]
    fn triangle_display_has_eight_planes_00de3520() {
        let mut w = world();
        let record = note_planes(&mut w);
        let flag_result = Rc::new(RefCell::new(0u8));
        let flag = Rc::clone(&flag_result);
        double(&mut w.e, &w.log, TRIANGLE_IS_DEGENERATE, move |e, a| {
            e.mem.set_u8(a[0], *flag.borrow());
            a[0]
        });
        let transform = w.e.mem.alloc(0x40);
        let shape = make_shape(&mut w.e, 3, 0.25);
        store_vec(&mut w.e, shape + 0x20, [0.0, 0.0, 0.0, 0.0]);
        store_vec(&mut w.e, shape + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut w.e, shape + 0x40, [0.0, 1.0, 0.0, 0.0]);
        build(&mut w, shape, transform);
        let degenerate = &calls(&w.log, TRIANGLE_IS_DEGENERATE)[0];
        assert_eq!(
            &degenerate[1..],
            &[shape + 0x20, shape + 0x30, shape + 0x40, 0.001f32.to_bits()]
        );
        let planes = record.borrow().clone();
        assert_eq!(planes.len(), 8);
        assert_close(planes[0], [0.0, 0.0, 1.0, -0.25]);
        assert_close(planes[1], [0.0, 0.0, -1.0, -0.25]);
        assert_close(planes[2], [0.0, -1.0, 0.0, -0.25]);
        assert_close(
            planes[3],
            [
                std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2,
                0.0,
                -0.9571,
            ],
        );
        assert_close(planes[4], [-1.0, 0.0, 0.0, -0.25]);
        let bisector = planes[5];
        let length = (bisector[0] * bisector[0] + bisector[1] * bisector[1]).sqrt();
        assert!((length - 1.0).abs() < 1e-3);
        // The plane array was freed (eight 16-byte planes), nothing was pushed.
        let freed = calls(&w.log, ALLOCATOR + 0x10);
        assert_eq!((freed[0][0], freed[0][2]), (w.allocator, 128));
        assert!(pointers(&w.e, w.out).is_empty());
        // A degenerate triangle makes nothing.
        *flag_result.borrow_mut() = 1;
        w.log.borrow_mut().clear();
        build(&mut w, shape, transform);
        assert!(calls(&w.log, ALLOCATOR + 0xc).is_empty());
    }

    /// A convex vertices shape with the planes `y <= 1` and `y >= -1` whose only vertex is
    /// `(0, vertex_y, 0)` and whose box is `[-1,-2,-3]..[4,5,6]`; builds it and answers the planes
    /// the display is made from.
    fn convex_vertices_planes(vertex_y: f32) -> Seen {
        let mut w = world();
        let record = note_planes(&mut w);
        let planes = w.e.mem.alloc(12);
        let plane_data = w.e.mem.alloc(32);
        store_vec(&mut w.e, plane_data, [0.0, 1.0, 0.0, -1.0]);
        store_vec(&mut w.e, plane_data + 16, [0.0, -1.0, 0.0, -1.0]);
        w.e.mem.set_u32(planes, plane_data);
        w.e.mem.set_u32(planes + 4, 2);
        double(&mut w.e, &w.log, GET_PLANE_EQUATIONS, move |_, _| planes);
        double(&mut w.e, &w.log, GET_ORIGINAL_VERTICES, move |e, a| {
            let data = e.mem.alloc(16);
            store_vec(e, data, [0.0, vertex_y, 0.0, 0.0]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 1);
            0
        });
        let shape = make_shape(&mut w.e, 6, 0.25);
        let vtable = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(vtable + 0x1c, 0x0920_0000);
        w.e.mem.set_u32(shape, vtable);
        double(&mut w.e, &w.log, 0x0920_0000, |e, a| {
            store_vec(e, a[3], [-1.0, -2.0, -3.0, 0.0]);
            store_vec(e, a[3] + 16, [4.0, 5.0, 6.0, 0.0]);
            0
        });
        let transform = w.e.mem.alloc(0x40);
        build(&mut w, shape, transform);
        let freed = calls(&w.log, ALLOCATOR + 0x10);
        assert_eq!(freed.last().map(|c| c[2]), Some(8 * 16));
        record
    }

    #[test]
    fn convex_vertices_display_moves_the_planes_in_by_the_radius_00de3520() {
        let moved = convex_vertices_planes(3.0).borrow().clone();
        assert_eq!(moved.len(), 8);
        assert_eq!(moved[0], [0.0, 1.0, 0.0, -1.25]);
        assert_eq!(moved[1], [0.0, -1.0, 0.0, -1.25]);
        assert_eq!(moved[2], [1.0, 0.0, 0.0, -4.0]);
        assert_eq!(moved[7], [0.0, 0.0, -1.0, -3.0]);
        // Vertices too close to the planes leave their offsets alone.
        let kept = convex_vertices_planes(0.5).borrow().clone();
        assert_eq!(kept[0], [0.0, 1.0, 0.0, -1.0]);
        assert_eq!(kept[1], [0.0, -1.0, 0.0, -1.0]);
    }

    #[test]
    fn display_convex_is_made_once_00de4340() {
        let mut w = world();
        double(&mut w.e, &w.log, DISPLAY_CONVEX_CONSTRUCTOR, |_, _| 0xd15);
        double(&mut w.e, &w.log, ARRAY_GROW, |e, a| {
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            0
        });
        let list = w.e.mem.alloc(12);
        assert_eq!(w.e.call(0x00de_4340, &args![w.builder, list]).u32(), 0xd15);
        assert_eq!(w.e.mem.u32(w.builder + 0xc), 0xd15);
        assert_eq!(pointers(&w.e, list), vec![0xd15]);
        assert_eq!(calls(&w.log, ARRAY_GROW), vec![vec![w.allocator, list, 4]]);
        let constructor = &calls(&w.log, DISPLAY_CONVEX_CONSTRUCTOR)[0];
        assert_eq!(w.e.mem.u16(constructor[0] + 4), 0x60);
        assert_eq!(w.e.mem.u32(constructor[1] + 8), EMPTY_CAPACITY);
        assert_eq!(w.e.call(0x00de_4340, &args![w.builder, list]).u32(), 0xd15);
        assert_eq!(calls(&w.log, DISPLAY_CONVEX_CONSTRUCTOR).len(), 1);
        assert_eq!(pointers(&w.e, list).len(), 1);
    }

    #[test]
    fn build_display_geometries_starts_from_the_identity_00de4420() {
        let mut w = world();
        user_builder(&mut w.e);
        let record = Rc::new(RefCell::new(Vec::new()));
        let noted = Rc::clone(&record);
        double(&mut w.e, &w.log, USER_FUNCTION, move |e, a| {
            *noted.borrow_mut() = (0..16).map(|word| e.mem.f32(a[1] + 4 * word)).collect();
            0
        });
        w.e.mem.set_u32(w.builder + 0xc, 0x99);
        w.e.mem.set_u32(w.out + 4, 5);
        let shape = make_shape(&mut w.e, USER_TYPE, 0.5);
        w.e.call(0x00de_4420, &args![w.builder, shape, w.out]);
        assert_eq!(w.e.mem.u32(w.builder + 0xc), 0);
        assert_eq!(w.e.mem.u32(w.out + 4), 0);
        assert_eq!(
            *record.borrow(),
            vec![1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(calls(&w.log, USER_FUNCTION)[0][0], shape);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00de43e0_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00de_43e0, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(e.mem.u16(block + 4), 0x14);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010d_8224);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0x8000_0000);
    }
}
