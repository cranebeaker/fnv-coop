//! `Havok SDK/hkgeometryutility.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, call_on_st0, compare_flags, float_to_x87, load_bits, load_vec4, mul4,
    refined_inverse_length, rotate_c9, rotate_d2, store_vec4, sub4, sum3, test_parity_even,
    x87_to_float, Vec4, X87, X87_SQRT,
};

/// `hkgpConvexHull` functions (all `thiscall` on the hull, a local of 0x14 bytes): the
/// constructor (`00d76ca0`), `BuildConfig`'s constructor (`00d74d40`, on a local of 0x10
/// bytes), `build_ov2` (`00d77200`: vertices, configuration; answers -1 when it fails),
/// `generateGeometry` (`00d76410`: 0, geometry), `buildIndices` (`00d76990`: a `float`, 0),
/// `fetchPlanes` (`00d76250`: planes) and the destructor (`00d76940`).
const CONVEX_HULL_CONSTRUCTOR: u32 = 0x00d7_6ca0;
const BUILD_CONFIG_CONSTRUCTOR: u32 = 0x00d7_4d40;
const CONVEX_HULL_BUILD: u32 = 0x00d7_7200;
const CONVEX_HULL_GENERATE_GEOMETRY: u32 = 0x00d7_6410;
const CONVEX_HULL_BUILD_INDICES: u32 = 0x00d7_6990;
const CONVEX_HULL_FETCH_PLANES: u32 = 0x00d7_6250;
const CONVEX_HULL_DESTRUCTOR: u32 = 0x00d7_6940;
/// The `float` `createConvexGeometry`'s second form hands to `buildIndices` (`0106ea1c`).
const BUILD_INDICES_FLOAT: u32 = 0x0106_ea1c;
/// `hkPlaneEquationUtil::getConvexVerticesFromPlaneEquations` (`00dd6270`, cdecl: planes, out).
const CONVEX_VERTICES_FROM_PLANES: u32 = 0x00dd_6270;
/// The memory router getter (`00c85750`, its word at `+0x10` is the allocator), the router's
/// `easyAlloc` (`00c85690`, cdecl: allocator, bytes) and `easyFree` (`00c856c0`, cdecl:
/// allocator, block).
const MEMORY_ROUTER_GETTER_UTILITY: u32 = 0x00c8_5750;
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `0.5` as a `float` (`01016248`) and `1.0` (`01012068`), the exe's zero (`01011d78`), the
/// sign mask `0x80000000` in every lane (`010c44d0`), the starting minimum and maximum of
/// `00d5ae60` (`010d1014`, `010d1010`).
const HALF_FLOAT: u32 = 0x0101_6248;
const ONE_FLOAT_UTILITY: u32 = 0x0101_2068;
const ZERO_FLOAT: u32 = 0x0101_1d78;
const SIGN_MASK: u32 = 0x010c_44d0;
const START_MINIMUM: u32 = 0x010d_1014;
const START_MAXIMUM: u32 = 0x010d_1010;
/// `0.5` as a `double` (`01011588`) and `1.0` (`01012070`).
const HALF_AS_DOUBLE: u32 = 0x0101_1588;
const ONE_AS_DOUBLE: u32 = 0x0101_2070;

// Translated from 00d5a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtility::createConvexGeometry` (Xbox PDB), cdecl (`vertices`, `geometry`): builds
/// a convex hull of the vertices (a local `hkgpConvexHull` and `BuildConfig`; the byte at `+0`
/// of the configuration is set to 1) and, unless `build` answers -1, has it generate the
/// geometry (`generateGeometry(0, geometry)`). The hull is destroyed at the end.
pub fn hk_geometry_utility_create_convex_geometry(e: &mut Engine, vertices: u32, geometry: u32) {
    e.with_stack(0x14, |e, hull| {
        e.with_stack(0x10, |e, config| {
            e.call(CONVEX_HULL_CONSTRUCTOR, &args![hull]);
            e.call(BUILD_CONFIG_CONSTRUCTOR, &args![config]);
            e.mem.set_u8(config.addr(), 1);
            let built = e
                .call(CONVEX_HULL_BUILD, &args![hull, vertices, config])
                .i32();
            if built != -1 {
                e.call(CONVEX_HULL_GENERATE_GEOMETRY, &args![hull, 0u32, geometry]);
            }
            e.call(CONVEX_HULL_DESTRUCTOR, &args![hull]);
        });
    });
}

// Translated from 00d5a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtility::createConvexGeometry_ov2` (Xbox PDB), cdecl (`vertices`, `geometry`,
/// `planes`): as the first form, and after `generateGeometry` also `buildIndices` with the
/// `float` at `0106ea1c` and 0, and `fetchPlanes(planes)`.
pub fn hk_geometry_utility_create_convex_geometry_ov2(
    e: &mut Engine,
    vertices: u32,
    geometry: u32,
    planes: u32,
) {
    e.with_stack(0x14, |e, hull| {
        e.with_stack(0x10, |e, config| {
            e.call(CONVEX_HULL_CONSTRUCTOR, &args![hull]);
            e.call(BUILD_CONFIG_CONSTRUCTOR, &args![config]);
            e.mem.set_u8(config.addr(), 1);
            let built = e
                .call(CONVEX_HULL_BUILD, &args![hull, vertices, config])
                .i32();
            if built != -1 {
                e.call(CONVEX_HULL_GENERATE_GEOMETRY, &args![hull, 0u32, geometry]);
                let value = x87_to_float(float_to_x87(e.mem.f32(BUILD_INDICES_FLOAT)));
                e.call(CONVEX_HULL_BUILD_INDICES, &args![hull, value, 0u32]);
                e.call(CONVEX_HULL_FETCH_PLANES, &args![hull, planes]);
            }
            e.call(CONVEX_HULL_DESTRUCTOR, &args![hull]);
        });
    });
}

// Translated from 00d5a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtility::createVerticesFromPlaneEquations` (Xbox PDB), cdecl (`planes`, `out`): a
/// tail call of `hkPlaneEquationUtil::getConvexVerticesFromPlaneEquations` (`00dd6270`) with
/// the same two words; answers what it answers.
pub fn hk_geometry_utility_create_vertices_from_plane_equations(
    e: &mut Engine,
    planes: u32,
    out: u32,
) -> u32 {
    e.call(CONVEX_VERTICES_FROM_PLANES, &args![planes, out])
        .u32()
}

// Translated from 00d5ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `a` (`EAX`) and `b` (`ECX`) point at `float`s; the stack word is `epsilon`. True when the
/// magnitude of the difference (x87: the difference is stored as a `float`, then its
/// absolute value, then compared with `epsilon`) is strictly below `epsilon`.
pub fn fn_00d5ad60(e: &mut Engine, a: u32, b: u32, epsilon: f32) -> bool {
    let mut x = X87::new();
    x.fld_f32(e.mem.f32(a));
    x.sub_m(float_to_x87(e.mem.f32(b)));
    let difference = x.fstp_f32();
    x.fld_f32(difference);
    x.abs();
    let magnitude = x.fstp_f32();
    x.fld_f32(magnitude);
    x.fld_f32(epsilon);
    let flags = x.fcom(1);
    x.pop();
    x.pop();
    flags & 0x41 == 0
}

// Translated from 00d5ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The vector is in a register: `vector` (`EAX`, a pointer to three
/// `float`s); the stack word is `epsilon`. Takes the absolute values of x, y and z (through
/// the x87 stack), the largest of them (the later of x and y wins ties, `z` replaces the
/// maximum only when strictly larger; NaNs as the x87 flags give) and answers true when
/// `|largest - 1.0|` (x87 double constant at `01012070`, each step stored as a `float`) is
/// strictly below `epsilon`.
pub fn fn_00d5ad90(e: &mut Engine, vector: u32, epsilon: f32) -> bool {
    let mut x = X87::new();
    let mut absolute = |e: &Engine, offset: u32| {
        x.fld_f32(e.mem.f32(vector + offset));
        x.abs();
        let stored = x.fstp_f32();
        x.fld_f32(stored);
        x.fstp_f32()
    };
    let abs_z = absolute(e, 8);
    let abs_y = absolute(e, 4);
    let abs_x = absolute(e, 0);
    x.fld_f32(abs_x);
    x.fld_f32(abs_y);
    let flags = x.fcom(1);
    if test_parity_even(flags, 5) {
        x.fstp_st(1);
    } else {
        x.pop();
    }
    let mut largest = x.fstp_f32();
    x.fld_f32(abs_z);
    x.fld_f32(largest);
    let flags = x.fcom(1);
    x.pop();
    if test_parity_even(flags, 5) {
        x.pop();
    } else {
        largest = x.fstp_f32();
    }
    x.fld_f32(largest);
    x.sub_m(e.mem.f64(ONE_AS_DOUBLE));
    let difference = x.fstp_f32();
    x.fld_f32(difference);
    x.abs();
    let magnitude = x.fstp_f32();
    x.fld_f32(magnitude);
    x.fld_f32(epsilon);
    let flags = x.fcom(1);
    x.pop();
    x.pop();
    flags & 0x41 == 0
}

// Translated from 00d5bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `out` (`EAX`) and `a` (`ECX`, a 16-byte vector); the stack words are `b` (a pointer to a
/// 16-byte vector) and `margin`. Writes at `out` the bounding sphere of the two points: the
/// centre `a + h` with `h = 0.5 * (b - a)` and, in the fourth lane, `|h|` (lanes 0 to 2,
/// `SQRTSS`) plus `margin`, added on the x87 stack and stored as a `float`.
pub fn fn_00d5bb10(e: &mut Engine, out: u32, a: u32, b: u32, margin: f32) {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let half = [e.mem.f32(HALF_FLOAT); 4];
    let h = mul4(half, sub4(vb, va));
    let length = sum3(mul4(h, h)).sqrt();
    let mut x = X87::new();
    x.fld_f32(length);
    x.add_m(float_to_x87(margin));
    store_vec4(e, out, add4(va, h));
    let radius = x.fstp_f32();
    e.mem.set_f32(out + 0xc, radius);
}

// Translated from 00d5bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `out` (`EAX`) and `a` (`ECX`); the stack words are `b`, `c` (pointers to 16-byte vectors)
/// and `margin`. Writes at `out` the circumscribed sphere of the triangle `a`, `b`, `c`: with
/// `u = b - a`, `v = c - a` and `n = u x v` the centre is
/// `a + ((n x u) * |v|^2 + (v x n) * |u|^2) * k` where `k` is `0.5 / |n|^2` (the double at
/// `01011588` divided by the squared length on the x87 stack, stored as a `float`), and the
/// fourth lane is the offset's length (`SQRTSS`) plus `margin`.
pub fn fn_00d5bb80(e: &mut Engine, out: u32, a: u32, b: u32, c: u32, margin: f32) {
    let va = load_vec4(e, a);
    let u = sub4(load_vec4(e, b), va);
    let v = sub4(load_vec4(e, c), va);
    let n = sub4(
        mul4(rotate_c9(u), rotate_d2(v)),
        mul4(rotate_d2(u), rotate_c9(v)),
    );
    let squared = sum3(mul4(n, n));
    let mut x = X87::new();
    x.fld_f32(squared);
    x.divr_m(e.mem.f64(HALF_AS_DOUBLE));
    let k = x.fstp_f32();
    let vv = sum3(mul4(v, v));
    let uu = sum3(mul4(u, u));
    let first = mul4(
        sub4(
            mul4(rotate_c9(n), rotate_d2(u)),
            mul4(rotate_d2(n), rotate_c9(u)),
        ),
        [vv; 4],
    );
    let second = mul4(
        sub4(
            mul4(rotate_d2(n), rotate_c9(v)),
            mul4(rotate_c9(n), rotate_d2(v)),
        ),
        [uu; 4],
    );
    let numerator = add4(add4(first, [0.0; 4]), second);
    let offset = mul4(numerator, [k; 4]);
    let radius = sum3(mul4(offset, offset)).sqrt();
    x.fld_f32(radius);
    x.add_m(float_to_x87(margin));
    store_vec4(e, out, add4(va, offset));
    let result = x.fstp_f32();
    e.mem.set_f32(out + 0xc, result);
}

// Translated from 00d5c060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtility::createMinBoundingSphere` (Xbox PDB), cdecl (`out`, `vertices`, `epsilon`):
/// `vertices` is a strided vertex list (the first word the data, `+4` the number of
/// vertices, `+8` the stride in bytes, rounded down to a multiple of 4). The x, y and z of
/// every vertex (through the x87 stack) are copied into a block of 16 bytes per vertex
/// taken from the memory router (`easyAlloc`), `00d5beb0(out, block, count, 0, epsilon)`
/// computes the sphere and the block is given back (`easyFree`). Answers `out`.
pub fn hk_geometry_utility_create_min_bounding_sphere(
    e: &mut Engine,
    out: u32,
    vertices: u32,
    epsilon: f32,
) -> u32 {
    let count = e.mem.u32(vertices + 4);
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e
        .call(EASY_ALLOC, &args![allocator, count.wrapping_shl(4)])
        .u32();
    let stride_words = {
        let stride = e.mem.i32(vertices + 8);
        let adjusted = stride.wrapping_add(((stride >> 31) as u32 & 3) as i32);
        (adjusted >> 2) as u32
    };
    let stride = stride_words.wrapping_mul(4);
    let mut index = 0i32;
    if e.mem.i32(vertices + 4) > 0 {
        let mut source = 0u32;
        let mut target = block;
        loop {
            for lane in 0..3 {
                let data = e.mem.u32(vertices);
                let value = x87_to_float(float_to_x87(
                    e.mem.f32(data.wrapping_add(source).wrapping_add(4 * lane)),
                ));
                e.mem.set_f32(target + 4 * lane, value);
            }
            index += 1;
            target = target.wrapping_add(0x10);
            source = source.wrapping_add(stride);
            if index >= e.mem.i32(vertices + 4) {
                break;
            }
        }
    }
    let radius = x87_to_float(float_to_x87(epsilon));
    let count = e.mem.u32(vertices + 4);
    fn_00d5beb0(e, out, block, count, 0u32, radius);
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![allocator, block]);
    out
}

// Translated from 00d5c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtility::createGeometryFromPlaneEquations` (Xbox PDB), cdecl (`planes`,
/// `geometry`): gets the vertices of the convex shape of the planes into a local array
/// (`00dd6270`); when there are some, builds a strided vertex record over them (the word
/// `+0` the data, `+4` the count, `+8` the stride 0x10) and has `createConvexGeometry` make
/// the geometry; with none, the two words at `geometry + 0x10` and `+4` are cleared. The
/// local array's memory is given back through the memory router's allocator (slot `0x10`).
pub fn hk_geometry_utility_create_geometry_from_plane_equations(
    e: &mut Engine,
    planes: u32,
    geometry: u32,
) {
    e.with_stack(0xc, |e, array| {
        let at = array.addr();
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
        e.call(CONVEX_VERTICES_FROM_PLANES, &args![planes, array]);
        let count = e.mem.i32(at + 4);
        if count > 0 {
            let data = e.mem.u32(at);
            e.with_stack(0xc, |e, strided| {
                let record = strided.addr();
                e.mem.set_u32(record + 4, count as u32);
                e.mem.set_u32(record, data);
                e.mem.set_u32(record + 8, 0x10);
                hk_geometry_utility_create_convex_geometry(e, record, geometry);
            });
        } else {
            e.mem.set_u32(geometry + 0x10, 0);
            e.mem.set_u32(geometry + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(at + 8);
        e.mem.set_u32(at + 4, 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & 0x3fff_ffff) << 4;
            let data = e.mem.u32(at);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00d5ac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `result`, `matrix`, `out_x`, `out_y`, `out_z`): runs the
/// axes solver (`00d5a6e0(result, matrix, 0x1e)`). When the first byte of `result` is 0
/// the three outputs are the unit axes `(1, 0, 0, 0)`, `(0, 1, 0, 0)` and `(0, 0, 1, 0)`
/// (the exe's `1.0` at `01012068` and zero at `01011d78`). Otherwise they are the three
/// rows of `matrix` (16-byte vectors at `+0`, `+0x10` and `+0x20`), and when the triple
/// product `(x x y) . z` is negative (an x87 compare with 0.0) the third has all its sign
/// bits flipped (the mask at `010c44d0`). Answers `result`.
pub fn fn_00d5ac50(
    e: &mut Engine,
    result: u32,
    matrix: u32,
    out_x: u32,
    out_y: u32,
    out_z: u32,
) -> u32 {
    fn_00d5a6e0(e, result, matrix, 0x1e);
    if e.mem.u8(result) == 0 {
        let zero = e.mem.f32(ZERO_FLOAT);
        let one = e.mem.f32(ONE_FLOAT_UTILITY);
        store_vec4(e, out_x, [one, zero, zero, zero]);
        store_vec4(e, out_y, [zero, one, zero, zero]);
        store_vec4(e, out_z, [zero, zero, one, zero]);
        return result;
    }
    for (offset, target) in [(0u32, out_x), (0x10, out_y), (0x20, out_z)] {
        for lane in 0..4 {
            let word = e.mem.u32(matrix + offset + 4 * lane);
            e.mem.set_u32(target + 4 * lane, word);
        }
    }
    let row_x = load_vec4(e, out_x);
    let row_y = load_vec4(e, out_y);
    let row_z = load_vec4(e, out_z);
    let cross = sub4(
        mul4(rotate_c9(row_x), rotate_d2(row_y)),
        mul4(rotate_d2(row_x), rotate_c9(row_y)),
    );
    let triple = sum3(mul4(cross, row_z));
    let flags = compare_flags(0.0, float_to_x87(triple));
    if flags & 0x41 == 0 {
        for lane in 0..4 {
            let mask = e.mem.u32(SIGN_MASK + 4 * lane);
            let word = e.mem.u32(out_z + 4 * lane) ^ mask;
            e.mem.set_u32(out_z + 4 * lane, word);
        }
    }
    result
}

// Translated from 00d5ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `vertices` (`ECX`, the first of the points: three `float`s each) and `basis` (`EDI`, three
/// 16-byte vectors at `+0`, `+0x10`, `+0x20`); the stack words are `count`, `stride`
/// (bytes), `half_extents` and `centre` (pointers to 16-byte outputs). The loop runs
/// `count / 3` times (the exe's signed division by 3) and takes three points per run (the
/// stride is added after each); each point `p` (zero in the fourth lane) gives three dot
/// products with the basis rows (`row * p`, and `p * row` for the third point of a run)
/// kept as per-axis minimum and maximum (they start at `010d1014` and `010d1010` in every
/// lane; an x87 compare replaces the minimum only when strictly larger than the dot and the
/// maximum only when strictly smaller). Afterwards `half_extents` gets `0.5 * (max - min)`
/// and `centre` the box centre in the world: with `c = 0.5 * (max + min)` it is
/// `c.z * row2 + (c.y * row1 + c.x * row0)`.
pub fn fn_00d5ae60(
    e: &mut Engine,
    vertices: u32,
    basis: u32,
    count: u32,
    stride: u32,
    half_extents: u32,
    centre: u32,
) {
    let product = i64::from(count as i32) * 0x5555_5556i64;
    let high = (product >> 32) as i32;
    let runs = high.wrapping_add(((high as u32) >> 31) as i32);
    let mut minimum = [e.mem.f32(START_MINIMUM); 4];
    let mut maximum = [e.mem.f32(START_MAXIMUM); 4];
    let zero = e.mem.f32(ZERO_FLOAT);
    if runs > 0 {
        let mut point_at = vertices;
        for _ in 0..runs {
            for point_index in 0..3 {
                let p = [
                    e.mem.f32(point_at),
                    e.mem.f32(point_at + 4),
                    e.mem.f32(point_at + 8),
                    zero,
                ];
                point_at = point_at.wrapping_add(stride);
                for axis in 0..3usize {
                    let row = load_vec4(e, basis + 0x10 * axis as u32);
                    let products = if point_index == 2 {
                        mul4(p, row)
                    } else {
                        mul4(row, p)
                    };
                    let dot = float_to_x87(sum3(products));
                    if compare_flags(float_to_x87(minimum[axis]), dot) & 0x41 == 0 {
                        minimum[axis] = x87_to_float(dot);
                    }
                    if !test_parity_even(compare_flags(float_to_x87(maximum[axis]), dot), 5) {
                        maximum[axis] = x87_to_float(dot);
                    }
                }
            }
        }
    }
    let half = [e.mem.f32(HALF_FLOAT); 4];
    let extent = sub4(maximum, minimum);
    let centre_local = mul4(half, add4(maximum, minimum));
    store_vec4(e, half_extents, mul4(half, extent));
    let row0 = load_vec4(e, basis);
    let row1 = load_vec4(e, basis + 0x10);
    let row2 = load_vec4(e, basis + 0x20);
    let first = mul4([centre_local[0]; 4], row0);
    let second = add4(mul4([centre_local[1]; 4], row1), first);
    let third = add4(mul4([centre_local[2]; 4], row2), second);
    store_vec4(e, centre, third);
}

/// `3.0` as a `float` (`01017718`) and the `float` `-1.0` (`01012054`).
const THREE_FLOAT: u32 = 0x0101_7718;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `0.0` as a `double` (`01012060`), `3.0` (`01021928`), `5.0` (`01020998`) and the growth
/// factor of the search, `1.2000000476837158` (`010290f0`).
const ZERO_AS_DOUBLE: u32 = 0x0101_2060;
const THREE_AS_DOUBLE: u32 = 0x0102_1928;
const FIVE_AS_DOUBLE: u32 = 0x0102_0998;
const GROWTH_AS_DOUBLE: u32 = 0x0102_90f0;
/// The `float`s of the optimiser: the first step (`01017d00`, `0.001`), the epsilon of the
/// gradient (`010141cc`, `0.0001`) and the step below which it stops (`010d1018`).
const FIRST_STEP: u32 = 0x0101_7d00;
const GRADIENT_EPSILON: u32 = 0x0101_41cc;
const STOP_STEP: u32 = 0x010d_1018;

/// `(refined 1 / |v|) * v` over four lanes: the inverse length is `RSQRTSS` of the squared
/// length (lanes 0 to 2) refined once, 0 when that squared length equals the exe's zero.
fn normalised(v: Vec4, zero: f32, half: f32, three: f32) -> Vec4 {
    let inverse = refined_inverse_length(sum3(mul4(v, v)), zero, half, three);
    mul4([inverse; 4], v)
}

/// Copies sixteen bytes the way a `MOVAPS` pair does.
fn copy_vec4(e: &mut Engine, from: u32, to: u32) {
    let v = load_vec4(e, from);
    store_vec4(e, to, v);
}

// Translated from 00d5b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `rows` (`EAX`: two 16-byte vectors) and `stride` (`EDX`, bytes between the points); the
/// stack words are `vertices` and `count`. Builds three orthonormal rows from the two: the
/// first row scaled by its refined inverse length (`RSQRTSS`, 0 for a zero squared length;
/// `0.5`, `3.0` and the zero come from `01016248`, `01017718` and `01011d78`), the second
/// reduced by its dot product with the *unscaled* first row (the dot goes through the x87
/// stack as a `float`) and scaled the same way, and the third the scaled cross product of the
/// unscaled rows. `00d5ae60` measures the points against those rows (the half extents and
/// the centre go to locals) and the answer in `ST0` is `(hy * hx) * hz`, the product of the
/// three half extents, stored as a `float`.
pub fn fn_00d5b0c0(e: &mut Engine, rows: u32, stride: u32, vertices: u32, count: u32) -> f32 {
    let r0 = load_vec4(e, rows);
    let r1 = load_vec4(e, rows + 0x10);
    let zero = e.mem.f32(ZERO_FLOAT);
    let half = e.mem.f32(HALF_FLOAT);
    let three = e.mem.f32(THREE_FLOAT);
    let n0 = normalised(r0, zero, half, three);
    let dot = x87_quiet(sum3(mul4(r1, r0)));
    let reduced = sub4(r1, mul4([dot; 4], r0));
    let n1 = normalised(reduced, zero, half, three);
    let r1_again = load_vec4(e, rows + 0x10);
    let cross = sub4(
        mul4(rotate_d2(r1_again), rotate_c9(r0)),
        mul4(rotate_c9(r1_again), rotate_d2(r0)),
    );
    let n2 = normalised(cross, zero, half, three);
    e.with_stack(0x50, |e, frame| {
        let basis = frame.addr();
        let half_extents = basis + 0x30;
        let centre = basis + 0x40;
        store_vec4(e, basis, n0);
        store_vec4(e, basis + 0x10, n1);
        store_vec4(e, basis + 0x20, n2);
        // The three rows are not touched between the stores and the call (in the exe, the
        // half extents and the centre are the words the code left at its locals).
        fn_00d5ae60(e, vertices, basis, count, stride, half_extents, centre);
        let mut x = X87::new();
        x.fld_f32(e.mem.f32(half_extents + 4));
        x.mul_m(float_to_x87(e.mem.f32(half_extents)));
        x.mul_m(float_to_x87(e.mem.f32(half_extents + 8)));
        x87_quiet(x.fstp_f32())
    })
}

// Translated from 00d5b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The rows are in a register: `params` (`EAX`, two 16-byte
/// vectors); the stack words are `vertices`, `count`, `stride`, `epsilon` and `gradient` (two
/// 16-byte vectors). The gradient of `00d5b0c0` with respect to the six used lanes of the two
/// rows: with a copy of the rows, each lane in turn gets `epsilon` added (through the x87
/// stack, as a `float`), `00d5b0c0` is run over the copy and `(value - base) * (1 / epsilon)`
/// (x87, stored as a `float`) goes to the same lane of `gradient`, after which the lane is
/// restored from `params`. Each gradient row is then multiplied by `-1.0` (`01012054`, all
/// four lanes, so lane 3 flips too) and when the squared length of the six lanes (the sum of
/// the two rows' `lane2 + (lane1 + lane0)`) is above 1.0 (x87 compare, NaN counts as not
/// above) both rows are scaled by `1 / sqrt(total)` (`_CIsqrt`, stored as a `float`).
pub fn fn_00d5b2a0(
    e: &mut Engine,
    params: u32,
    vertices: u32,
    count: u32,
    stride: u32,
    epsilon: f32,
    gradient: u32,
) {
    e.with_stack(0x20, |e, work| {
        let w = work.addr();
        copy_vec4(e, params, w);
        copy_vec4(e, params + 0x10, w + 0x10);
        let base = x87_quiet(fn_00d5b0c0(e, params, stride, vertices, count));
        let mut x = X87::new();
        x.fld_f32(epsilon);
        x.fld1();
        x.divrp(1);
        let inverse = x.fstp_f32();
        for row in 0..2u32 {
            for lane in 0..3u32 {
                let slot = w + 0x10 * row + 4 * lane;
                x.fld_f32(epsilon);
                x.add_m(float_to_x87(e.mem.f32(slot)));
                let bumped = x.fstp_f32();
                e.mem.set_f32(slot, bumped);
                let value = x87_quiet(fn_00d5b0c0(e, w, stride, vertices, count));
                x.fld_f32(value);
                x.sub_m(float_to_x87(base));
                x.mul_m(float_to_x87(inverse));
                let slope = x.fstp_f32();
                e.mem.set_f32(gradient + 0x10 * row + 4 * lane, slope);
                x.fld_f32(e.mem.f32(params + 0x10 * row + 4 * lane));
                let restored = x.fstp_f32();
                e.mem.set_f32(slot, restored);
            }
            let minus_one = e.mem.f32(MINUS_ONE_FLOAT);
            let current = load_vec4(e, gradient + 0x10 * row);
            store_vec4(e, gradient + 0x10 * row, mul4([minus_one; 4], current));
        }
        let row0 = load_vec4(e, gradient);
        let row1 = load_vec4(e, gradient + 0x10);
        let total = sum3(mul4(row0, row0)) + sum3(mul4(row1, row1));
        x.fld1();
        x.fld_f32(total);
        let flags = x.fcom(1);
        x.fstp_st(1);
        if flags & 0x41 != 0 {
            x.pop();
            assert!(x.is_empty());
            return;
        }
        call_on_st0(e, &mut x, X87_SQRT);
        let length = x.fstp_f32();
        x.fld_f32(length);
        x.fld1();
        x.divrp(1);
        let scale = x.fstp_f32();
        store_vec4(e, gradient, mul4([scale; 4], row0));
        let second = load_vec4(e, gradient + 0x10);
        store_vec4(e, gradient + 0x10, mul4([scale; 4], second));
        assert!(x.is_empty());
    });
}

// Translated from 00d5b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `work` (`ECX`, three 16-byte vectors) and `source` (`EDX`, two 16-byte vectors); the
/// stack words are `delta` (two 16-byte vectors) and `t`. Each of the two rows becomes
/// `t * delta + source` (all four lanes, `MULPS` then `ADDPS`), after which the rows are
/// made orthonormal the way `00d5b0c0` does it (the dot product goes through the x87 stack
/// as a `float`; the refined inverse lengths use `01016248`, `01017718` and `01011d78`):
/// row 0 is scaled, row 1 loses its dot with the scaled row 0 and is scaled, and row 2
/// becomes the scaled cross product `row0 x row1`.
pub fn fn_00d5b440(e: &mut Engine, work: u32, source: u32, delta: u32, t: f32) {
    for row in 0..2u32 {
        let at = work + 0x10 * row;
        copy_vec4(e, source + 0x10 * row, at);
        let moved = mul4([t; 4], load_vec4(e, delta + 0x10 * row));
        let sum = add4(moved, load_vec4(e, at));
        store_vec4(e, at, sum);
    }
    let zero = e.mem.f32(ZERO_FLOAT);
    let half = e.mem.f32(HALF_FLOAT);
    let three = e.mem.f32(THREE_FLOAT);
    let w0 = load_vec4(e, work);
    let n0 = normalised(w0, zero, half, three);
    let w1 = load_vec4(e, work + 0x10);
    store_vec4(e, work, n0);
    let dot = x87_quiet(sum3(mul4(n0, w1)));
    let reduced = sub4(load_vec4(e, work + 0x10), mul4([dot; 4], n0));
    store_vec4(e, work + 0x10, reduced);
    let n1 = normalised(load_vec4(e, work + 0x10), zero, half, three);
    store_vec4(e, work + 0x10, n1);
    let cross = sub4(
        mul4(rotate_d2(n1), rotate_c9(n0)),
        mul4(rotate_c9(n1), rotate_d2(n0)),
    );
    store_vec4(e, work + 0x20, cross);
    let n2 = normalised(load_vec4(e, work + 0x20), zero, half, three);
    store_vec4(e, work + 0x20, n2);
}

// Translated from 00d5b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `rows` (`ESI`) and `stride` (`EDI`); the stack words are `vertices`, `count`, `step`
/// (`float`), `delta` and `iterations` (signed). A line search along `delta` for the lowest
/// value of `00d5b0c0`: it moves the rows by `step` (`00d5b440` into a local of three rows,
/// then `00d5b0c0` of that) and compares with the value at the start.
///
/// When the start is above the stepped value the step is grown by the double at `010290f0`
/// (1.2) until the value is above the best one (or `iterations - 1` growths ran, which
/// answers the last step). When it is not, the step is halved toward 0 (`(t + 0.0) * 0.5`,
/// the double at `01012060` is 0) until the value is below the start and below the value
/// at the first step (too many halvings answer -1.0). The three step positions and values
/// that result must be ordered (step at least 0.0, high at least low, the start value at
/// least the best and the value at the high step at least the best), otherwise the answer is
/// -1.0 (`01012054`). Then `iterations` golden-section steps (`r = (3 - sqrt(5)) * 0.5` from
/// the doubles at `01021928` and `01020998`, `_CIsqrt`) shrink the bracket and the answer is
/// the lower step. Every value is kept as a `float` (the exe's frame slots), the compares
/// are the x87 flag tests with NaN included.
pub fn fn_00d5b640(
    e: &mut Engine,
    rows: u32,
    stride: u32,
    vertices: u32,
    count: u32,
    step: f32,
    delta: u32,
    iterations: i32,
) -> f32 {
    e.with_stack(0x30, |e, work| {
        let w = work.addr();
        let move_to = |e: &mut Engine, t: f32| {
            fn_00d5b440(e, w, rows, delta, t);
        };
        let value_at =
            |e: &mut Engine| -> f32 { x87_quiet(fn_00d5b0c0(e, w, stride, vertices, count)) };
        let failed = |e: &Engine| x87_quiet(e.mem.f32(MINUS_ONE_FLOAT));
        let load = float_to_x87;
        let step = x87_quiet(step);
        // The locals of the exe's frame: the best value (`+0x14`), the low and high steps
        // (`+0x10`, `+0x18`), the value at the high step (`+0x1c`), the start value (`+0x20`)
        // and the left end of the bracket (`+0x24`).
        let mut best;
        let mut low;
        let mut high;
        let mut high_value;
        let mut left = 0.0f32;
        let start = x87_quiet(fn_00d5b0c0(e, rows, stride, vertices, count));
        move_to(e, step);
        let stepped = value_at(e);
        let start_above = compare_flags(load(start), load(stepped)) & 0x41 == 0;
        if start_above {
            low = step;
            high = step;
            best = stepped;
            high_value = 0.0f32;
            if iterations > 0 {
                let mut counter = 0i32;
                loop {
                    let mut x = X87::new();
                    x.fld_f32(high);
                    x.mul_m(e.mem.f64(GROWTH_AS_DOUBLE));
                    high = x.fstp_f32();
                    move_to(e, high);
                    high_value = value_at(e);
                    if compare_flags(load(high_value), load(best)) & 0x41 == 0 {
                        break;
                    }
                    best = x87_quiet(high_value);
                    low = x87_quiet(high);
                    if counter == iterations.wrapping_sub(1) {
                        return x87_quiet(high);
                    }
                    counter += 1;
                    if counter >= iterations {
                        break;
                    }
                }
            }
        } else {
            high = step;
            low = step;
            high_value = stepped;
            best = 0.0f32;
            if iterations > 0 {
                let mut counter = 0i32;
                loop {
                    let mut x = X87::new();
                    x.fld_f32(low);
                    x.add_m(e.mem.f64(ZERO_AS_DOUBLE));
                    x.mul_m(e.mem.f64(HALF_AS_DOUBLE));
                    low = x.fstp_f32();
                    move_to(e, low);
                    best = value_at(e);
                    if compare_flags(load(start), load(best)) & 0x41 == 0
                        && compare_flags(load(stepped), load(best)) & 0x41 == 0
                    {
                        break;
                    }
                    if counter == iterations.wrapping_sub(1) {
                        return failed(e);
                    }
                    counter += 1;
                    if counter >= iterations {
                        break;
                    }
                }
            }
        }
        // The common checks (each fails with -1.0 when the first value is below the second).
        let zero = e.mem.f64(ZERO_AS_DOUBLE);
        let ordered = test_parity_even(compare_flags(load(low), zero), 5)
            && test_parity_even(compare_flags(load(high), load(low)), 5)
            && test_parity_even(compare_flags(load(start), load(best)), 5)
            && test_parity_even(compare_flags(load(high_value), load(best)), 5);
        if !ordered {
            return failed(e);
        }
        let mut x = X87::new();
        x.fld(e.mem.f64(FIVE_AS_DOUBLE));
        call_on_st0(e, &mut x, X87_SQRT);
        let root = x.fstp_f32();
        x.fld_f32(root);
        x.subr_m(e.mem.f64(THREE_AS_DOUBLE));
        x.mul_m(e.mem.f64(HALF_AS_DOUBLE));
        let ratio = x.fstp_f32();
        if iterations <= 0 {
            return x87_quiet(low);
        }
        let mut remaining = iterations;
        loop {
            let mut x = X87::new();
            x.fld_f32(ratio);
            x.fld_f32(low);
            x.fld_st(0);
            x.sub_m(load(left));
            let lower_gap = x.fstp_f32();
            x.fld_f32(high);
            x.sub(0, 1);
            let upper_gap = x.fstp_f32();
            x.fld_f32(lower_gap);
            x.fld_f32(upper_gap);
            let flags = x.fcom(1);
            if test_parity_even(flags, 5) {
                x.fstp_st(1);
                x.mulp(2);
                x.addp(1);
                let t = x.fstp_f32();
                move_to(e, t);
                let value = value_at(e);
                if compare_flags(load(best), load(value)) & 0x41 != 0 {
                    high = x87_quiet(t);
                } else {
                    left = x87_quiet(low);
                    low = x87_quiet(t);
                    best = x87_quiet(value);
                }
            } else {
                x.pop();
                x.mulp(2);
                x.subrp(1);
                let t = x.fstp_f32();
                move_to(e, t);
                let value = value_at(e);
                if compare_flags(load(best), load(value)) & 0x41 != 0 {
                    left = x87_quiet(t);
                } else {
                    high = x87_quiet(low);
                    low = x87_quiet(t);
                    best = x87_quiet(value);
                }
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        x87_quiet(low)
    })
}

// Translated from 00d5b990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d5b990 - lane 3 of the three rows comes from a stack word the exe never stores; not reproduced.
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `work` (`EAX`, the three rows the search leaves its answer in) and `stride` (`ECX`); the
/// stack words are `vertices`, `count`, `source` (three 16-byte vectors) and `iterations`
/// (signed). Copies the `source` rows to `work`, takes the value of `00d5b0c0` at `source` and
/// answers it at once when `iterations` is not above 0. Otherwise it runs up to `iterations`
/// rounds. Each round saves the rows, computes the gradient with `00d5b2a0` (epsilon `0.0001`
/// at `010141cc`, into a local), runs the line search `00d5b640` along it (the first step is
/// the `float` at `01017d00`, `0.001`; the later ones are the previous answer) and answers
/// the best value when the search failed (a negative answer, NaN does not count). The rows are
/// moved by the answer along the gradient (`00d5b440`); when the new value is above the old
/// one the saved rows come back and the old value is answered, when the answer is below the
/// `float` at `010d1018` the new value is answered, and the last round answers the new value.
///
/// Lane 3 of the three rows: the machine code and this translation agree in lanes 0 to 2 only;
/// in the exe lane 3 comes out of a stack word the code never stores (which one is
/// unconfirmed), so it is not reproduced here.
pub fn fn_00d5b990(
    e: &mut Engine,
    work: u32,
    stride: u32,
    vertices: u32,
    count: u32,
    source: u32,
    iterations: i32,
) -> f32 {
    let mut step = x87_quiet(e.mem.f32(FIRST_STEP));
    let mut best = x87_quiet(fn_00d5b0c0(e, source, stride, vertices, count));
    for row in 0..3u32 {
        copy_vec4(e, source + 0x10 * row, work + 0x10 * row);
    }
    if iterations <= 0 {
        return best;
    }
    e.with_stack(0x50, |e, frame| {
        let gradient = frame.addr();
        let saved = gradient + 0x20;
        let mut round = 0i32;
        loop {
            let epsilon = x87_quiet(e.mem.f32(GRADIENT_EPSILON));
            for row in 0..3u32 {
                copy_vec4(e, work + 0x10 * row, saved + 0x10 * row);
            }
            fn_00d5b2a0(e, work, vertices, count, stride, epsilon, gradient);
            let answer = x87_quiet(fn_00d5b640(
                e, work, stride, vertices, count, step, gradient, iterations,
            ));
            step = answer;
            if !test_parity_even(compare_flags(float_to_x87(answer), 0.0), 5) {
                return best;
            }
            fn_00d5b440(e, work, work, gradient, answer);
            let old = best;
            best = x87_quiet(fn_00d5b0c0(e, work, stride, vertices, count));
            if compare_flags(float_to_x87(best), float_to_x87(old)) & 0x41 == 0 {
                for row in 0..3u32 {
                    copy_vec4(e, saved + 0x10 * row, work + 0x10 * row);
                }
                return old;
            }
            let stop = x87_quiet(e.mem.f32(STOP_STEP));
            if compare_flags(float_to_x87(stop), float_to_x87(step)) & 0x41 == 0 {
                return best;
            }
            round += 1;
            if round >= iterations {
                return best;
            }
        }
    })
}

// Translated from 00d5bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `out` (`EAX`) and `a` (`ECX`, a 16-byte vector); the stack words are `b`, `c`, `d`
/// (pointers to 16-byte vectors) and `margin`. Writes at `out` the sphere through the four
/// points. With `u1 = b - a`, `u2 = c - a`, `u3 = d - a` the determinant `u1 . (u2 x u3)` is
/// worked out on the x87 stack (each product of two lanes, in the order of the code) and
/// stored as a `float`, and `k = 0.5 / determinant` (the double at `01011588`, stored as a
/// `float`). The centre offset is
/// `((u1 x u2) * |u3|^2 + 0 + (u3 x u1) * |u2|^2 + (u2 x u3) * |u1|^2) * k` and the fourth
/// lane is the offset's length (`SQRTSS`) plus `margin` (x87, stored as a `float`); the
/// first three lanes are `a` plus the offset.
pub fn fn_00d5bce0(e: &mut Engine, out: u32, a: u32, b: u32, c: u32, d: u32, margin: f32) {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vd = load_vec4(e, d);
    let vc = load_vec4(e, c);
    let u2 = sub4(vc, va);
    let u3 = sub4(vd, va);
    let u1 = sub4(vb, va);
    let s3 = sum3(mul4(u3, u3));
    let s2 = sum3(mul4(u2, u2));
    let s1 = sum3(mul4(u1, u1));
    let mut x = X87::new();
    x.fld_f32(u3[2]);
    x.fld_st(0);
    x.fld_f32(u2[1]);
    x.fld_st(0);
    x.mulp(2);
    x.fld_f32(u3[1]);
    x.fld_st(0);
    x.fld_f32(u2[2]);
    x.fld_st(0);
    x.mulp(2);
    x.fxch(4);
    x.subrp(1);
    x.mul_m(float_to_x87(u1[0]));
    x.fld_f32(u2[0]);
    x.fld_st(0);
    x.mulp(6);
    x.fld_f32(u3[0]);
    x.fld_st(0);
    x.mulp(6);
    x.fxch(6);
    x.subrp(5);
    x.fld_f32(u1[1]);
    x.mulp(5);
    x.fxch(1);
    x.subrp(4);
    x.mulp(1);
    x.fxch(3);
    x.mulp(1);
    x.subp(2);
    x.fld_f32(u1[2]);
    x.mulp(2);
    x.addp(1);
    let determinant = x.fstp_f32();
    x.fld_f32(determinant);
    x.divr_m(e.mem.f64(HALF_AS_DOUBLE));
    let k = x.fstp_f32();
    let cross = |left: Vec4, right: Vec4| {
        sub4(
            mul4(rotate_c9(left), rotate_d2(right)),
            mul4(rotate_d2(left), rotate_c9(right)),
        )
    };
    let first = mul4(cross(u1, u2), [s3; 4]);
    let with_zero = add4(first, [0.0; 4]);
    let second = mul4(cross(u3, u1), [s2; 4]);
    let third = mul4(cross(u2, u3), [s1; 4]);
    let offset = mul4(add4(add4(with_zero, second), third), [k; 4]);
    let radius = sum3(mul4(offset, offset)).sqrt();
    x.fld_f32(radius);
    x.add_m(float_to_x87(margin));
    store_vec4(e, out, add4(va, offset));
    let result = x.fstp_f32();
    e.mem.set_f32(out + 0xc, result);
}

// Translated from 00d5beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out`, `points`, `count`, `support`, `margin`): the sphere
/// through the `support` points that sit just below `points` (16-byte vectors at
/// `points - 0x10`, `points - 0x20`, ...) widened to hold the `count` points from `points` on.
/// The starting sphere is the first support point with the radius `margin` (the `ST0` copy
/// of it) for 1, the zero vector with the radius `-1.0` (`01012054`) for none and the sphere
/// of two or three points (`00d5bb10`, `00d5bb80`) for 2 and 3; with more than three the sphere
/// of four (`00d5bce0` on `points - 0x10` to `points - 0x40`) is answered at once. It is
/// stored at `out`. For each point in turn the distance to the centre (lanes 0 to 2,
/// `SQRTSS`) is compared with the radius (x87 compares: only a radius strictly below the
/// distance counts): the point moves to the front of the list (the points before it shift one
/// place up) and the function recurses (`points + 0x10`, the number of points before it,
/// `support + 1`) into a local sphere that becomes the new sphere at `out`. Answers `out`.
pub fn fn_00d5beb0(
    e: &mut Engine,
    out: u32,
    points: u32,
    count: u32,
    support: u32,
    margin: f32,
) -> u32 {
    let margin = x87_quiet(margin);
    e.with_stack(0x10, |e, sphere| {
        let at = sphere.addr();
        let below = |n: u32| points.wrapping_sub(0x10 * n);
        if support > 3 {
            fn_00d5bce0(e, at, below(1), below(2), below(3), below(4), margin);
            copy_vec4(e, at, out);
            return;
        }
        let initial = match support {
            0 => [0.0, 0.0, 0.0, x87_quiet(e.mem.f32(MINUS_ONE_FLOAT))],
            1 => {
                let mut point = load_vec4(e, below(1));
                point[3] = margin;
                point
            }
            2 => {
                fn_00d5bb10(e, at, below(1), below(2), margin);
                load_vec4(e, at)
            }
            _ => {
                fn_00d5bb80(e, at, below(1), below(2), below(3), margin);
                load_vec4(e, at)
            }
        };
        store_vec4(e, out, initial);
        let count = count as i32;
        let mut index = 0i32;
        let mut current = points;
        while index < count {
            let centre = load_vec4(e, out);
            let point = load_vec4(e, current);
            let offset = sub4(point, centre);
            let distance = sum3(mul4(offset, offset)).sqrt();
            let radius = e.mem.f32(out + 0xc);
            let flags = compare_flags(float_to_x87(radius), float_to_x87(distance));
            if !test_parity_even(flags, 5) {
                let mut cursor = current;
                for _ in 0..index {
                    let lower = cursor.wrapping_sub(0x10);
                    let moved = load_vec4(e, cursor);
                    copy_vec4(e, lower, cursor);
                    store_vec4(e, lower, moved);
                    cursor = lower;
                }
                fn_00d5beb0(
                    e,
                    at,
                    points.wrapping_add(0x10),
                    index as u32,
                    support.wrapping_add(1),
                    margin,
                );
                copy_vec4(e, at, out);
            }
            index += 1;
            current = current.wrapping_add(0x10);
        }
    });
    out
}

/// `00d5d660` (`thiscall` on a two-word state, one stack word: the 16-byte vector to fill), the
/// `float` that scales it (`01013ea4`, `0.01`), `0.5`, `9.0` (`0102e438`) and `12.0`
/// (`01035710`) as `double`s, and one third as a `float` (`0106c2e0`).
const RANDOM_VECTOR: u32 = 0x00d5_d660;
const JITTER_SCALE: u32 = 0x0101_3ea4;
const NINE_AS_DOUBLE: u32 = 0x0102_e438;
const TWELVE_AS_DOUBLE: u32 = 0x0103_5710;
const THIRD_FLOAT: u32 = 0x0106_c2e0;
/// `hkArrayUtil::_reserve` (`00c908c0`: allocator, array, count, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00d5c1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getMeanAndCovariance` (Xbox PDB), cdecl (`vertices`, `count`, `stride`, `covariance`, `mean`,
/// `jitter`). Walks the `count / 3` triangles of the vertex list (three floats per vertex,
/// `stride` bytes apart; each vertex becomes a vector with the exe's zero in lane 3) and
/// answers the area-weighted mean at `mean` (a vector) and the covariance at `covariance`
/// (three vectors). A local `hkArray` of 16-byte vectors takes each triangle's centroid
/// (reserved through the memory router's allocator with `_reserve`, given back at the end).
///
/// Per triangle `a`, `b`, `c` the area is `0.5 * |(a - b) x (a - c)|` (the cross product from the
/// `SHUFPS` rotations, its squared length summed lanes 2, 1, 0, `SQRTSS`, then the x87 half
/// stored as a `float`); with `jitter` set `a - b` becomes `(a + 0.01 * r) - b` for the vector `r`
/// that `00d5d660` makes from a state of two words, both `count`. When the area is above 0
/// the centroid is `(b + a + c) * (1/3)`, the area is added to the running total (x87,
/// stored as a `float`), `area * centroid` to the weighted sum and every entry `[r][k]` of the
/// covariance gets `(((g_k * 9.0) * g_r + a_r * a_k) + b_r * b_k) + c_r * c_k) * area` (x87). A
/// triangle whose area is not above 0 (a NaN counts) stores the same centroid and adds
/// `((b_k * b_r + a_r * a_k) + c_k * c_r)` unweighted. With no triangles, or a total area not
/// above 0, the mean is 0 and the covariance the identity. Otherwise the mean is the weighted
/// sum times `1 / total` (`float`) and each covariance entry becomes
/// `c / 12.0 - (m_r * m_k) * total` (x87, stored as a `float`).
pub fn get_mean_and_covariance(
    e: &mut Engine,
    vertices: u32,
    count: i32,
    stride: u32,
    covariance: u32,
    mean: u32,
    jitter: u8,
) {
    let triangles = count / 3;
    let zero = e.mem.f32(ZERO_FLOAT);
    e.with_stack(0x40, |e, frame| {
        let array = frame.addr();
        let generator = array + 0x10;
        let random = array + 0x20;
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = e.mem.i32(array + 4);
        let wanted = size.wrapping_add(triangles);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let doubled = capacity.wrapping_add(capacity);
            let new_capacity = if wanted < doubled { doubled } else { wanted };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, new_capacity, 0x10u32],
            );
        }
        e.mem.set_i32(array + 4, size.wrapping_add(triangles));
        let mut record = e.mem.u32(array).wrapping_add((size as u32) << 4);
        let zero_vector = [0.0f32; 4];
        store_vec4(e, covariance, zero_vector);
        store_vec4(e, covariance + 0x10, zero_vector);
        store_vec4(e, covariance + 0x20, zero_vector);
        let mut total = 0.0f32;
        let mut weighted = zero_vector;
        if triangles <= 0 {
            // Nothing to walk: the identity covariance and a zero mean.
            store_vec4(e, mean, zero_vector);
            for (offset, one) in [(0u32, 1.0f32), (0x14, 1.0), (0x28, 1.0)] {
                e.mem.set_f32(covariance + offset, one);
            }
        } else {
            let mut vertex = vertices;
            let nine = e.mem.f64(NINE_AS_DOUBLE);
            let third = e.mem.f32(THIRD_FLOAT);
            for _ in 0..triangles {
                let mut read = |e: &Engine| {
                    let v = [
                        e.mem.f32(vertex),
                        e.mem.f32(vertex + 4),
                        e.mem.f32(vertex + 8),
                        zero,
                    ];
                    vertex = vertex.wrapping_add(stride);
                    v
                };
                let a = read(e);
                let b = read(e);
                let c = read(e);
                let ab = if jitter != 0 {
                    e.mem.set_u32(generator, count as u32);
                    e.mem.set_u32(generator + 4, count as u32);
                    e.call(RANDOM_VECTOR, &args![generator, random]);
                    let scale = e.mem.f32(JITTER_SCALE);
                    let r = load_vec4(e, random);
                    sub4(add4(mul4([scale; 4], r), a), b)
                } else {
                    sub4(a, b)
                };
                let ac = sub4(a, c);
                let normal = sub4(
                    mul4(rotate_d2(ac), rotate_c9(ab)),
                    mul4(rotate_c9(ac), rotate_d2(ab)),
                );
                let length = sum3(mul4(normal, normal)).sqrt();
                let mut x = X87::new();
                x.fld_f32(length);
                x.mul_m(e.mem.f64(HALF_AS_DOUBLE));
                let area = x.fstp_f32();
                let sum = add4(b, a);
                let positive = compare_flags(float_to_x87(area), 0.0) & 0x41 == 0;
                let centroid = mul4([third; 4], add4(sum, c));
                if positive {
                    store_vec4(e, record, centroid);
                    x.fld_f32(area);
                    x.add_m(float_to_x87(total));
                    total = x.fstp_f32();
                    weighted = add4(mul4([area; 4], centroid), weighted);
                    for k in 0..3usize {
                        for r in 0..3usize {
                            x.fld_f32(centroid[k]);
                            x.mul_m(nine);
                            x.mul_m(float_to_x87(centroid[r]));
                            x.fld_f32(a[r]);
                            x.mul_m(float_to_x87(a[k]));
                            x.addp(1);
                            x.fld_f32(b[r]);
                            x.mul_m(float_to_x87(b[k]));
                            x.addp(1);
                            x.fld_f32(c[k]);
                            x.mul_m(float_to_x87(c[r]));
                            x.addp(1);
                            x.mul_m(float_to_x87(area));
                            let slot = covariance + 0x10 * r as u32 + 4 * k as u32;
                            x.add_m(float_to_x87(e.mem.f32(slot)));
                            let value = x.fstp_f32();
                            e.mem.set_f32(slot, value);
                        }
                    }
                } else {
                    store_vec4(e, record, centroid);
                    for k in 0..3usize {
                        for r in 0..3usize {
                            x.fld_f32(b[k]);
                            x.mul_m(float_to_x87(b[r]));
                            x.fld_f32(a[r]);
                            x.mul_m(float_to_x87(a[k]));
                            x.addp(1);
                            x.fld_f32(c[k]);
                            x.mul_m(float_to_x87(c[r]));
                            x.addp(1);
                            let slot = covariance + 0x10 * r as u32 + 4 * k as u32;
                            x.add_m(float_to_x87(e.mem.f32(slot)));
                            let value = x.fstp_f32();
                            e.mem.set_f32(slot, value);
                        }
                    }
                }
                assert!(x.is_empty());
                record = record.wrapping_add(0x10);
            }
            if test_parity_even(compare_flags(float_to_x87(total), 0.0), 0x41) {
                let mut x = X87::new();
                x.fld_f32(total);
                x.fld1();
                x.divrp(1);
                let inverse = x.fstp_f32();
                let mean_vector = mul4([inverse; 4], weighted);
                let twelve = e.mem.f64(TWELVE_AS_DOUBLE);
                for k in 0..3usize {
                    for r in 0..3usize {
                        let slot = covariance + 0x10 * r as u32 + 4 * k as u32;
                        x.fld_f32(e.mem.f32(slot));
                        x.div_m(twelve);
                        x.fld_f32(mean_vector[r]);
                        x.mul_m(float_to_x87(mean_vector[k]));
                        x.mul_m(float_to_x87(total));
                        x.subp(1);
                        let value = x.fstp_f32();
                        e.mem.set_f32(slot, value);
                    }
                }
                store_vec4(e, mean, mean_vector);
            } else {
                store_vec4(e, mean, zero_vector);
                store_vec4(e, covariance, zero_vector);
                store_vec4(e, covariance + 0x10, zero_vector);
                store_vec4(e, covariance + 0x20, zero_vector);
                for (offset, one) in [(0u32, 1.0f32), (0x14, 1.0), (0x28, 1.0)] {
                    e.mem.set_f32(covariance + offset, one);
                }
            }
        }
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let flags = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(array);
            let bytes = (flags & 0x3fff_ffff) << 4;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

/// `00d5c060`'s epsilon `1e-5` (`010718c0`, a `float`, also the bound of the perpendicularity
/// test), the factor `1.001` (`010d1020`, a `double`), the `float` `0.001` (`01017d00`) and
/// the absolute-value mask (`01017d10`).
const SPHERE_EPSILON: u32 = 0x0107_18c0;
const BOX_FACTOR: u32 = 0x010d_1020;
const CORNER_TOLERANCE: u32 = 0x0101_7d00;
const ABS_MASK_UTILITY: u32 = 0x0101_7d10;
/// `hkArrayUtil::_reserveMore` (`00c90940`: allocator, array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Routines of other units that `getIsBoxGeometry` calls, all unconfirmed and called as the code
/// does: `00cd4f10` (`thiscall` on the three rows, one `float`), `00c7f700` (`thiscall` on a
/// vector: a rotation, the vector), `00cb21e0` (`thiscall` on a transform: a transform) and
/// `00c7f640` (`thiscall` on a vector: a transform, the vector).
const ROTATION_FIX: u32 = 0x00cd_4f10;
const ROTATE_VECTOR: u32 = 0x00c7_f700;
const COPY_TRANSFORM: u32 = 0x00cb_21e0;
const TRANSFORM_VECTOR: u32 = 0x00c7_f640;

/// Gives back what `getIsBoxGeometry` allocated: the corner array's data (when it owns it, the
/// capacity word being non-negative) and then the 0x80-byte block (`block_flags` is its
/// capacity word), both through slot `0x10` of the router's allocator.
fn release_box_buffers(e: &mut Engine, array: u32, block: u32, block_flags: u32) {
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (flags & 0x3fff_ffff) << 4;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if (block_flags as i32) >= 0 {
        let bytes = (block_flags & 0x3fff_ffff) << 4;
        e.vcall(allocator, 0x10, &args![block, bytes]);
    }
}

// Translated from 00d5c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getIsBoxGeometry` (Xbox PDB), cdecl (`list`, `half_extents`, `transform`, `tolerance`;
/// answers a byte): `list` is `{data, count}` of 16-byte vectors. The minimum bounding
/// sphere of the points (`createMinBoundingSphere` with the epsilon at `010718c0` over a
/// strided record of stride 0x10) gives a centre and radius; the points whose distance to the
/// centre is within `tolerance` of the radius (x87: `|distance - radius| < tolerance`) are
/// collected into a local `hkArray` (grown with `_reserveMore`); exactly 8 of them must be
/// found, else the answer is 0. They are copied to a 0x80-byte block taken from the memory
/// router's allocator (slot `0xc`), and the code looks for the two nearest neighbours of
/// the first corner (the edges `e01`, `e02` start as `c0 - c1` and `c0 - c2` and are replaced
/// from `c3` to `c7` by squared-distance compares), requires `e01 . e02` to be at most the
/// `float` `1e-5`, and builds three orthonormal rows (`e01`, `e02` and their cross product,
/// each scaled by its refined inverse length) which `00cd4f10` fixes up (argument `0.01`)
/// and `00c7f700` applies to the eight corners. The bounds of the rotated corners must put
/// every coordinate of every corner on the lower or upper bound (`00d5ad60`, epsilon `0.001`).
/// `half_extents` gets `0.5 * (max - min)` (lane 3 is half of what it held) and the negated
/// centre goes into a local transform (the three rows and `-c`) that `00cb21e0` copies to
/// `transform`; when all three rows are axis-aligned unit vectors (`00d5ad90` with
/// `tolerance`) the half extents are rotated, made absolute (lane 3 cleared) and the rotation
/// of `transform` becomes the identity. The local transform is then refreshed from `transform`
/// and every input point, transformed (`00c7f640`), must lie within `1.001` times the half
/// extents (a NaN passes). Answers true when all of it holds. The buffers are given back on
/// every path.
pub fn get_is_box_geometry(
    e: &mut Engine,
    list: u32,
    half_extents: u32,
    transform: u32,
    tolerance: f32,
) -> bool {
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr();
        let array = base;
        let size_variable = base + 0x10;
        let strided = base + 0x20;
        let sphere_at = base + 0x30;
        let rows = base + 0x40;
        let output = base + 0x70;
        let local_transform = base + 0x80;
        let bounds_at = base + 0xc0;
        let data = e.mem.u32(list);
        let count = e.mem.i32(list + 4);
        e.mem.set_u32(strided, data);
        e.mem.set_i32(strided + 4, count);
        e.mem.set_u32(strided + 8, 0x10);
        let epsilon = x87_quiet(e.mem.f32(SPHERE_EPSILON));
        hk_geometry_utility_create_min_bounding_sphere(e, sphere_at, strided, epsilon);
        let sphere = load_vec4(e, sphere_at);
        e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]);
        e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]);
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(size_variable, 0x80);
        let block = e.vcall(allocator, 0xc, &args![size_variable]).u32();
        let granted = e.mem.i32(size_variable);
        let adjusted = granted.wrapping_add(((granted >> 31) as u32 & 0xf) as i32);
        let units = adjusted >> 4;
        let block_flags = if units == 0 {
            0x8000_0000
        } else {
            units as u32
        };
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        if count > 0 {
            let mut offset = 0u32;
            for _ in 0..count {
                let point = e.mem.u32(list).wrapping_add(offset);
                let v = load_vec4(e, point);
                let d = sub4(v, sphere);
                let distance = sum3(mul4(d, d)).sqrt();
                let mut x = X87::new();
                x.fld_f32(distance);
                x.sub_m(float_to_x87(sphere[3]));
                let difference = x.fstp_f32();
                x.fld_f32(difference);
                x.abs();
                let magnitude = x.fstp_f32();
                x.fld_f32(magnitude);
                x.fld_f32(tolerance);
                let flags = x.fcom(1);
                x.pop();
                x.pop();
                if flags & 0x41 == 0 {
                    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
                    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
                    let mut size = e.mem.u32(array + 4);
                    if size == capacity {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
                        size = e.mem.u32(array + 4);
                    }
                    let target = e.mem.u32(array).wrapping_add(size << 4);
                    store_vec4(e, target, v);
                    e.mem.set_u32(array + 4, size.wrapping_add(1));
                }
                offset = offset.wrapping_add(0x10);
            }
        }
        if count <= 0 || e.mem.i32(array + 4) != 8 {
            release_box_buffers(e, array, block, block_flags);
            return false;
        }
        let corners = e.mem.u32(array);
        for k in 0..8u32 {
            copy_vec4(e, corners + 0x10 * k, block + 0x10 * k);
        }
        // The two nearest neighbours of the first corner among the others.
        let corner = |e: &Engine, k: u32| load_vec4(e, block + 0x10 * k);
        let mut e01 = sub4(corner(e, 0), corner(e, 1));
        let mut e02 = sub4(corner(e, 0), corner(e, 2));
        for k in 3..8u32 {
            let delta = sub4(corner(e, 0), corner(e, k));
            let distance = sum3(mul4(delta, delta));
            let s1 = sum3(mul4(e01, e01));
            let s2 = sum3(mul4(e02, e02));
            if compare_flags(float_to_x87(s1), float_to_x87(distance)) & 0x41 == 0 {
                if compare_flags(float_to_x87(s2), float_to_x87(s1)) & 0x41 == 0 {
                    e02 = e01;
                }
                e01 = delta;
            } else if compare_flags(float_to_x87(s2), float_to_x87(distance)) & 0x41 == 0 {
                e02 = delta;
            }
        }
        let dot = sum3(mul4(e02, e01));
        let limit = e.mem.f32(SPHERE_EPSILON);
        if !test_parity_even(compare_flags(float_to_x87(limit), float_to_x87(dot)), 5) {
            release_box_buffers(e, array, block, block_flags);
            return false;
        }
        // The three orthonormal rows.
        let zero = e.mem.f32(ZERO_FLOAT);
        let half = e.mem.f32(HALF_FLOAT);
        let three = e.mem.f32(THREE_FLOAT);
        let normal = sub4(
            mul4(rotate_d2(e02), rotate_c9(e01)),
            mul4(rotate_c9(e02), rotate_d2(e01)),
        );
        let inverse1 = refined_inverse_length(sum3(mul4(e01, e01)), zero, half, three);
        let inverse2 = refined_inverse_length(sum3(mul4(e02, e02)), zero, half, three);
        let inverse3 = refined_inverse_length(sum3(mul4(normal, normal)), zero, half, three);
        store_vec4(e, rows, mul4([inverse1; 4], e01));
        store_vec4(e, rows + 0x10, mul4([inverse2; 4], e02));
        store_vec4(e, rows + 0x20, mul4([inverse3; 4], normal));
        let small = x87_quiet(e.mem.f32(JITTER_SCALE));
        e.call(ROTATION_FIX, &args![rows, small]);
        for k in 0..8u32 {
            let vertex = block + 0x10 * k;
            e.call(ROTATE_VECTOR, &args![vertex, rows, vertex]);
        }
        // The bounds of the rotated corners, in the exe's frame slots (the minimum and the
        // maximum of an axis are 4 bytes apart).
        for axis in 0..3u32 {
            let first = x87_quiet(e.mem.f32(block + 4 * axis));
            e.mem.set_f32(bounds_at + 8 * axis, first);
            e.mem.set_f32(bounds_at + 8 * axis + 4, first);
        }
        for k in 1..8u32 {
            for axis in 0..3u32 {
                let value = e.mem.f32(block + 0x10 * k + 4 * axis);
                let minimum = e.mem.f32(bounds_at + 8 * axis);
                let maximum = e.mem.f32(bounds_at + 8 * axis + 4);
                if !test_parity_even(compare_flags(float_to_x87(value), float_to_x87(minimum)), 5) {
                    e.mem.set_f32(bounds_at + 8 * axis, x87_quiet(value));
                } else if compare_flags(float_to_x87(value), float_to_x87(maximum)) & 0x41 == 0 {
                    e.mem.set_f32(bounds_at + 8 * axis + 4, x87_quiet(value));
                }
            }
        }
        let corner_epsilon = x87_quiet(e.mem.f32(CORNER_TOLERANCE));
        for k in 0..8u32 {
            for axis in 0..3u32 {
                let coordinate = block + 0x10 * k + 4 * axis;
                let lower = bounds_at + 8 * axis;
                if !fn_00d5ad60(e, coordinate, lower, corner_epsilon)
                    && !fn_00d5ad60(e, coordinate, lower + 4, corner_epsilon)
                {
                    release_box_buffers(e, array, block, block_flags);
                    return false;
                }
            }
        }
        // The half extents and the centre.
        let bound = |e: &Engine, axis: u32| {
            (
                e.mem.f32(bounds_at + 8 * axis),
                e.mem.f32(bounds_at + 8 * axis + 4),
            )
        };
        let mut extents = load_vec4(e, half_extents);
        let mut centre = [0.0f32, 0.0, 0.0, zero];
        let half_double = e.mem.f64(HALF_AS_DOUBLE);
        for axis in 0..3u32 {
            let (minimum, maximum) = bound(e, axis);
            extents[axis as usize] = x87_to_float(float_to_x87(maximum) - float_to_x87(minimum));
            centre[axis as usize] =
                x87_to_float((float_to_x87(minimum) + float_to_x87(maximum)) * half_double);
        }
        store_vec4(e, half_extents, mul4([half; 4], extents));
        let minus_one = e.mem.f32(MINUS_ONE_FLOAT);
        let negated = mul4(centre, [minus_one; 4]);
        for k in 0..3u32 {
            copy_vec4(e, rows + 0x10 * k, local_transform + 0x10 * k);
        }
        store_vec4(e, local_transform + 0x30, negated);
        e.call(COPY_TRANSFORM, &args![transform, local_transform]);
        let tolerance_copy = x87_quiet(tolerance);
        let axis_aligned = (0..3u32).all(|k| fn_00d5ad90(e, rows + 0x10 * k, tolerance_copy));
        if axis_aligned {
            e.call(ROTATE_VECTOR, &args![half_extents, transform, half_extents]);
            e.mem.set_f32(half_extents + 0xc, 0.0);
            let absolute = load_vec4(e, half_extents);
            let mask = load_bits(e, ABS_MASK_UTILITY);
            let masked = [0, 1, 2, 3].map(|i| f32::from_bits(absolute[i].to_bits() & mask[i]));
            store_vec4(e, half_extents, masked);
            store_vec4(e, transform, [1.0, 0.0, 0.0, 0.0]);
            store_vec4(e, transform + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec4(e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        }
        e.call(COPY_TRANSFORM, &args![local_transform, transform]);
        // Every input point must be inside.
        if count > 0 {
            let mut offset = 0u32;
            let factor = e.mem.f64(BOX_FACTOR);
            for _ in 0..count {
                let point = e.mem.u32(list).wrapping_add(offset);
                e.call(TRANSFORM_VECTOR, &args![output, local_transform, point]);
                for axis in 0..3u32 {
                    let mut x = X87::new();
                    x.fld_f32(e.mem.f32(output + 4 * axis));
                    x.abs();
                    let magnitude = x.fstp_f32();
                    x.fld_f32(magnitude);
                    x.fld_f32(e.mem.f32(half_extents + 4 * axis));
                    x.mul_m(factor);
                    let flags = x.fcom(1);
                    if !test_parity_even(flags, 5) {
                        release_box_buffers(e, array, block, block_flags);
                        return false;
                    }
                }
                offset = offset.wrapping_add(0x10);
            }
        }
        release_box_buffers(e, array, block, block_flags);
        true
    })
}

/// `1e-6` as a `float` (`010959f4`, the size under which an off-diagonal entry counts as zero),
/// `1.0` and `-1.0` as `double`s (`01012070`, `0101a6b0`) and `_CIsqrt` (`00ec6040`).
const OFF_DIAGONAL_LIMIT: u32 = 0x0109_59f4;
const MINUS_ONE_AS_DOUBLE: u32 = 0x0101_a6b0;

/// One probe of the search of `00d5a6e0`: `|e[index]|` (the off-diagonal slot at `+0x38 + 4 * index`
/// of the frame at `s`) goes to the slot at `+0x30` and is compared with `1e-6` the way `FCOMP`
/// does; true when it is below.
fn off_diagonal_is_small(e: &mut Engine, s: u32, index: i32) -> bool {
    let value = e
        .mem
        .f32(s.wrapping_add((index.wrapping_mul(4).wrapping_add(0x38)) as u32));
    let mut y = X87::new();
    y.fld_f32(value);
    y.abs();
    let magnitude = y.fstp_f32();
    e.mem.set_f32(s + 0x30, magnitude);
    let limit = e.mem.f32(OFF_DIAGONAL_LIMIT);
    let flags = compare_flags(float_to_x87(magnitude), float_to_x87(limit));
    !test_parity_even(flags, 5)
}

// Translated from 00d5a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `result`, `matrix`, `iterations`): a symmetric 3 x 3
/// eigen solver over the vectors at `matrix` (rows of 16 bytes, lanes 0 to 2 used). First
/// every entry is divided by the largest row length (`SQRTSS` of lanes 2, 1, 0 summed; the
/// divide happens when that length is not exactly 0). Then a Householder step makes the
/// matrix tridiagonal: with `m20 != 0` the reflection built from `r = sqrt(m10^2 + m20^2)`
/// (`_CIsqrt`, stored as a `float`) leaves the diagonal at the frame slots `d0 = m00`, `d1`,
/// `d2` and the off-diagonal at `e0 = r`, `e1`, and the rows of `matrix` hold the reflection
/// `[1 0 0; 0 c s; 0 s -c]` (`c = m10 / r`, `s = m20 / r`); with `m20 == 0` the diagonal is
/// `m00, m11, m22`, the off-diagonal `m10`, `m21` and the rows are the identity (lane 3 is
/// zeroed). Then for each of three diagonal positions `l` an implicit QL iteration (at most
/// `iterations` rounds per position) rotates the rows of `matrix`: it looks for the first
/// `m >= l` with `|e[m]| < 1e-6`, stops at `m == l`, otherwise builds the shift
/// `g = (d[l+1] - d[l]) / (2 e[l])`, `r = sqrt(g^2 + 1)`, and rotates from `m - 1` down to `l`
/// (the `|f| >= |g|` branches pick `c` and `s` by `-g/f` or `-f/g`, `_CIsqrt` and the
/// `double` `-1.0`), updating `d`, `e` and the rows (each product through the x87 stack, each
/// value stored as a `float` in the exe's frame slots, which are kept at their offsets
/// below). The byte at `result` is 0 when some position used up its `iterations` rounds, 1
/// otherwise.
///
/// The exe's frame (offsets from the stack pointer after the three pushes): `+0x14`, `+0x18`,
/// `+0x1c`, `+0x24`, `+0x2c`, `+0x30`, `+0x34` hold temporaries, `+0x20` is `l`, `+0x28` the
/// round counter, `+0x2c` the count of positions left, `+0x38` / `+0x3c` / `+0x40` are `e0` /
/// `e1` / `e2` and `+0x44` / `+0x48` / `+0x4c` are `d0` / `d1` / `d2`.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00d5a6e0(e: &mut Engine, result: u32, matrix: u32, iterations: i32) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let s = frame.addr();
        let load = |e: &Engine, offset: u32| e.mem.f32(s + offset);
        let store = |e: &mut Engine, offset: u32, value: f32| e.mem.set_f32(s + offset, value);
        let entry = |row: u32, lane: u32| matrix + 0x10 * row + 4 * lane;
        let mut x = X87::new();
        x.fldz();
        store(e, 0x24, x.fst_f32());
        x.fld_f32(load(e, 0x24));
        // The largest row length.
        for row in 0..3u32 {
            let v = load_vec4(e, matrix + 0x10 * row);
            let length = sum3(mul4(v, v)).sqrt();
            store(e, 0x30, length);
            x.fld_f32(length);
            let flags = x.fcom(1);
            if flags & 0x41 != 0 {
                x.pop();
            } else {
                x.fstp_st(1);
                let value = x.fstp_f32();
                store(e, 0x24, value);
                x.fld_f32(load(e, 0x24));
            }
        }
        x.fld_st(1);
        let flags = x.fcom(1);
        x.pop();
        if test_parity_even(flags, 0x44) {
            for lane in 0..3u32 {
                for row in 0..3u32 {
                    x.fld_f32(e.mem.f32(entry(row, lane)));
                    x.div(0, 1);
                    let value = x.fstp_f32();
                    e.mem.set_f32(entry(row, lane), value);
                }
            }
        }
        x.pop();
        let flags = x.fcom_m(float_to_x87(e.mem.f32(entry(2, 0))));
        x.fld_f32(e.mem.f32(entry(0, 0)));
        let copy = x.fstp_f32();
        store(e, 0x44, copy);
        if !test_parity_even(flags, 0x44) {
            // The m20 == 0 case.
            x.fld_f32(e.mem.f32(entry(1, 1)));
            let value = x.fstp_f32();
            store(e, 0x48, value);
            x.fld_f32(e.mem.f32(entry(2, 2)));
            let value = x.fstp_f32();
            store(e, 0x4c, value);
            x.fld_f32(e.mem.f32(entry(1, 0)));
            let value = x.fstp_f32();
            store(e, 0x38, value);
            x.fld_f32(e.mem.f32(entry(2, 1)));
            for row in 0..3u32 {
                store_vec4(e, matrix + 0x10 * row, [0.0; 4]);
            }
            let value = x.fstp_f32();
            store(e, 0x3c, value);
            let value = x.fstp_f32();
            store(e, 0x40, value);
            x.fld1();
            let one = x.fst_f32();
            e.mem.set_f32(entry(0, 0), one);
            e.mem.set_f32(entry(1, 1), x.fst_f32());
        } else {
            let value = x.fstp_f32();
            store(e, 0x40, value);
            x.fld_f32(e.mem.f32(entry(2, 0)));
            x.fld_f32(e.mem.f32(entry(1, 0)));
            x.mul(0, 0);
            x.fld_st(1);
            x.mulp(2);
            x.addp(1);
            let value = x.fstp_f32();
            store(e, 0x30, value);
            x.fld_f32(load(e, 0x30));
            call_on_st0(e, &mut x, X87_SQRT);
            let value = x.fstp_f32();
            store(e, 0x30, value);
            x.fld_f32(load(e, 0x30));
            let value = x.fstp_f32();
            store(e, 0x30, value);
            x.fld_f32(e.mem.f32(entry(1, 0)));
            x.fld_f32(load(e, 0x30));
            x.fld_st(0);
            x.divp(2);
            x.fxch(1);
            let value = x.fstp_f32();
            store(e, 0x2c, value);
            x.fld_f32(e.mem.f32(entry(2, 0)));
            x.div(0, 1);
            let value = x.fstp_f32();
            store(e, 0x30, value);
            x.fld_f32(e.mem.f32(entry(2, 2)));
            x.sub_m(float_to_x87(e.mem.f32(entry(1, 1))));
            x.fld_f32(load(e, 0x30));
            x.fld_st(0);
            x.mulp(2);
            x.fld_f32(load(e, 0x2c));
            x.fld_st(0);
            x.add(0, 1);
            x.mul_m(float_to_x87(e.mem.f32(entry(2, 1))));
            x.addp(3);
            x.fxch(2);
            let value = x.fstp_f32();
            store(e, 0x30, value);
            x.fld_f32(load(e, 0x30));
            x.fld_st(0);
            x.mul(0, 2);
            x.fld_f32(e.mem.f32(entry(1, 1)));
            x.add(0, 1);
            let value = x.fstp_f32();
            store(e, 0x48, value);
            x.subr_m(float_to_x87(e.mem.f32(entry(2, 2))));
            let value = x.fstp_f32();
            store(e, 0x4c, value);
            x.fxch(3);
            let value = x.fstp_f32();
            store(e, 0x38, value);
            x.fld_f32(e.mem.f32(entry(2, 1)));
            x.fld_st(2);
            x.mulp(4);
            x.subrp(3);
            x.fxch(2);
            let value = x.fstp_f32();
            store(e, 0x3c, value);
            x.fld1();
            e.mem.set_f32(entry(0, 0), x.fstp_f32());
            x.fldz();
            e.mem.set_f32(entry(1, 0), x.fst_f32());
            e.mem.set_f32(entry(2, 0), x.fst_f32());
            e.mem.set_f32(entry(0, 1), x.fst_f32());
            e.mem.set_f32(entry(0, 2), x.fstp_f32());
            e.mem.set_f32(entry(1, 1), x.fst_f32());
            x.fxch(1);
            e.mem.set_f32(entry(2, 1), x.fst_f32());
            e.mem.set_f32(entry(1, 2), x.fstp_f32());
            x.chs();
        }
        e.mem.set_f32(entry(2, 2), x.fstp_f32());
        // The QL iterations.
        x.fld_f32(e.mem.f32(OFF_DIAGONAL_LIMIT));
        let mut l = 0i32;
        e.mem.set_i32(s + 0x20, l);
        e.mem.set_i32(s + 0x2c, 2);
        loop {
            // 00d5a893: a new position.
            let mut round = 0i32;
            e.mem.set_i32(s + 0x28, round);
            if iterations > 0 {
                'rounds: loop {
                    // 00d5a8a5: look for a small off-diagonal entry.
                    let mut m = l;
                    'search: {
                        if e.mem.i32(s + 0x2c) >= 4 {
                            // The unrolled search (only reached with four or more left).
                            loop {
                                if off_diagonal_is_small(e, s, m) {
                                    break 'search;
                                }
                                if off_diagonal_is_small(e, s, m + 1) {
                                    m += 1;
                                    break 'search;
                                }
                                if off_diagonal_is_small(e, s, m + 2) {
                                    m += 2;
                                    break 'search;
                                }
                                if off_diagonal_is_small(e, s, m + 3) {
                                    m += 3;
                                    break 'search;
                                }
                                m += 4;
                                if m > -2 {
                                    break;
                                }
                            }
                        }
                        if m <= 1 {
                            loop {
                                if off_diagonal_is_small(e, s, m) {
                                    break 'search;
                                }
                                m += 1;
                                if m > 1 {
                                    break 'search;
                                }
                            }
                        }
                    }
                    // 00d5a941.
                    if m == l {
                        break 'rounds;
                    }
                    x.pop();
                    let slot_d = |k: i32| (0x44 + 4 * k) as u32;
                    let slot_e = |k: i32| (0x38 + 4 * k) as u32;
                    x.fld_f32(load(e, slot_d(l + 1)));
                    x.sub_m(float_to_x87(load(e, slot_d(l))));
                    x.fld_f32(load(e, slot_e(l)));
                    x.add(0, 0);
                    x.divp(1);
                    let g = x.fstp_f32();
                    store(e, 0x18, g);
                    x.fld_f32(load(e, 0x18));
                    x.mul(0, 0);
                    x.add_m(e.mem.f64(ONE_AS_DOUBLE));
                    let value = x.fstp_f32();
                    store(e, 0x30, value);
                    x.fld_f32(load(e, 0x30));
                    call_on_st0(e, &mut x, X87_SQRT);
                    let value = x.fstp_f32();
                    store(e, 0x30, value);
                    x.fld_f32(load(e, 0x30));
                    let root = x.fstp_f32();
                    store(e, 0x14, root);
                    x.fldz();
                    x.fld_f32(load(e, 0x18));
                    let flags = x.fcom(1);
                    x.fld_f32(load(e, 0x14));
                    if test_parity_even(flags, 5) {
                        x.addp(1);
                    } else {
                        x.subp(1);
                    }
                    x.divr_m(float_to_x87(load(e, slot_e(l))));
                    x.fld_f32(load(e, slot_d(m)));
                    x.sub_m(float_to_x87(load(e, slot_d(l))));
                    let mut i = m - 1;
                    x.addp(1);
                    let value = x.fstp_f32();
                    store(e, 0x18, value);
                    x.fld1();
                    store(e, 0x1c, x.fst_f32());
                    let value = x.fstp_f32();
                    store(e, 0x14, value);
                    store(e, 0x24, x.fst_f32());
                    if i >= l {
                        x.pop();
                        x.fld_f32(load(e, 0x1c));
                        x.fld_f32(load(e, 0x14));
                        let mut row_pointer =
                            matrix.wrapping_add(((i as u32) << 4).wrapping_add(0x14));
                        loop {
                            x.fld_f32(load(e, slot_e(i)));
                            x.mulp(2);
                            x.fxch(1);
                            let f = x.fstp_f32();
                            store(e, 0x30, f);
                            x.mul_m(float_to_x87(load(e, slot_e(i))));
                            let b = x.fstp_f32();
                            store(e, 0x34, b);
                            x.fld_f32(load(e, 0x30));
                            x.fld_st(0);
                            x.abs();
                            let value = x.fstp_f32();
                            store(e, 0x1c, value);
                            x.fld_f32(load(e, 0x1c));
                            x.fld_f32(load(e, 0x18));
                            x.fld_st(0);
                            x.abs();
                            let value = x.fstp_f32();
                            store(e, 0x1c, value);
                            x.fld_f32(load(e, 0x1c));
                            let flags = x.fcom(2);
                            x.pop();
                            x.fstp_st(1);
                            if !test_parity_even(flags, 5) {
                                x.chs();
                                x.divrp(1);
                                let value = x.fstp_f32();
                                store(e, 0x18, value);
                                x.fld_f32(load(e, 0x18));
                                x.mul(0, 0);
                                x.add_m(e.mem.f64(ONE_AS_DOUBLE));
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x1c));
                                call_on_st0(e, &mut x, X87_SQRT);
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x1c));
                                x.divr_m(e.mem.f64(MINUS_ONE_AS_DOUBLE));
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x18));
                                x.fld_f32(load(e, 0x1c));
                                x.fld_st(0);
                                x.mulp(2);
                                x.fxch(1);
                                let value = x.fstp_f32();
                                store(e, 0x14, value);
                                x.chs();
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x30));
                                x.fld_f32(load(e, 0x1c));
                                x.fld_st(0);
                                x.divp(2);
                                x.fxch(1);
                                let value = x.fstp_f32();
                                store(e, slot_e(i + 1), value);
                                x.fld_f32(load(e, 0x14));
                            } else {
                                x.fxch(1);
                                x.chs();
                                x.divrp(1);
                                let value = x.fstp_f32();
                                store(e, 0x30, value);
                                x.fld_f32(load(e, 0x30));
                                x.mul(0, 0);
                                x.add_m(e.mem.f64(ONE_AS_DOUBLE));
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x1c));
                                call_on_st0(e, &mut x, X87_SQRT);
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.fld_f32(load(e, 0x1c));
                                x.divr_m(e.mem.f64(MINUS_ONE_AS_DOUBLE));
                                let value = x.fstp_f32();
                                store(e, 0x14, value);
                                x.fld_f32(load(e, 0x30));
                                x.fld_f32(load(e, 0x14));
                                x.fld_st(0);
                                x.mulp(2);
                                x.fxch(1);
                                let value = x.fstp_f32();
                                store(e, 0x1c, value);
                                x.chs();
                                let value = x.fstp_f32();
                                store(e, 0x14, value);
                                x.fld_f32(load(e, 0x18));
                                x.fld_f32(load(e, 0x14));
                                x.fld_st(0);
                                x.divp(2);
                                x.fxch(1);
                                let value = x.fstp_f32();
                                store(e, slot_e(i + 1), value);
                                x.fld_f32(load(e, 0x1c));
                                x.fxch(1);
                            }
                            // 00d5aaf0: the new diagonal entries and the rotation of the rows.
                            x.fld_f32(load(e, slot_d(i + 1)));
                            i -= 1;
                            x.sub_m(float_to_x87(load(e, 0x24)));
                            let value = x.fstp_f32();
                            store(e, 0x18, value);
                            x.fld_f32(load(e, slot_d(i + 1)));
                            x.fld_f32(load(e, 0x18));
                            x.fld_st(0);
                            x.subp(2);
                            x.fld_st(3);
                            x.mulp(2);
                            x.fld_f32(load(e, 0x34));
                            x.fld_st(0);
                            x.add(0, 1);
                            x.mul(0, 4);
                            x.addp(3);
                            x.fxch(2);
                            let value = x.fstp_f32();
                            store(e, 0x14, value);
                            x.fld_st(3);
                            x.fld_f32(load(e, 0x14));
                            x.fld_st(0);
                            x.mulp(2);
                            x.fxch(1);
                            let value = x.fstp_f32();
                            store(e, 0x24, value);
                            x.fld_f32(load(e, 0x24));
                            x.addp(2);
                            x.fxch(1);
                            let value = x.fstp_f32();
                            store(e, slot_d(i + 2), value);
                            x.mul(0, 2);
                            x.subrp(1);
                            let value = x.fstp_f32();
                            store(e, 0x18, value);
                            // The three lanes of the two rows.
                            for (upper, lower) in [(-4i32, -0x14i32), (0, -0x10), (4, -0xc)] {
                                let upper_at = row_pointer.wrapping_add(upper as u32);
                                let lower_at = row_pointer.wrapping_add(lower as u32);
                                x.fld_f32(e.mem.f32(upper_at));
                                let value = x.fstp_f32();
                                store(e, 0x14, value);
                                x.fld_f32(e.mem.f32(lower_at));
                                x.mul(0, 2);
                                x.fld_st(1);
                                x.fld_f32(load(e, 0x14));
                                x.fld_st(0);
                                x.mulp(2);
                                x.fxch(2);
                                x.addp(1);
                                let value = x.fstp_f32();
                                e.mem.set_f32(upper_at, value);
                                x.fld_f32(e.mem.f32(lower_at));
                                x.mul(0, 2);
                                x.fld_st(3);
                                x.mulp(2);
                                x.subrp(1);
                                let value = x.fstp_f32();
                                e.mem.set_f32(lower_at, value);
                            }
                            row_pointer = row_pointer.wrapping_sub(0x10);
                            if i < e.mem.i32(s + 0x20) {
                                break;
                            }
                        }
                        x.pop();
                        x.pop();
                        x.fldz();
                    }
                    // 00d5abd9.
                    x.fld_f32(load(e, slot_d(l)));
                    x.sub_m(float_to_x87(load(e, 0x24)));
                    let value = x.fstp_f32();
                    store(e, slot_d(l), value);
                    x.fld_f32(load(e, 0x18));
                    let value = x.fstp_f32();
                    store(e, slot_e(l), value);
                    round += 1;
                    let value = x.fstp_f32();
                    store(e, slot_e(m), value);
                    x.fld_f32(e.mem.f32(OFF_DIAGONAL_LIMIT));
                    e.mem.set_i32(s + 0x28, round);
                    if round >= iterations {
                        break 'rounds;
                    }
                }
            }
            // 00d5ac0d.
            if round == iterations {
                x.pop();
                e.mem.set_u8(result, 0);
                return;
            }
            let remaining = e.mem.i32(s + 0x2c) - 1;
            l += 1;
            e.mem.set_i32(s + 0x20, l);
            e.mem.set_i32(s + 0x2c, remaining);
            if remaining <= -1 {
                x.pop();
                e.mem.set_u8(result, 1);
                return;
            }
        }
    });
    result
}

/// `hkGeometry`'s destructor (`00cce530`, `thiscall`), the local unit axes, the `float` `0.01`
/// that `getIsBoxGeometry` takes as its tolerance, and the memory router's bump word.
const GEOMETRY_DESTRUCTOR: u32 = 0x00cc_e530;

/// Gives back what `calcObb` took from the memory router: the bump pointer is put back to
/// `saved`, the local array's data is freed through the allocator (slot `0x10`) when the
/// capacity word says it owns it, and the local `hkGeometry` is destroyed.
fn release_obb_buffers(e: &mut Engine, s: u32) {
    let array = s + 0xc;
    let geometry = s + 0x24;
    let saved = e.mem.u32(s + 0x18);
    if saved == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    e.mem.set_u32(router, saved);
    let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (flags & 0x3fff_ffff) << 4;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array + 8, 0x8000_0000);
    e.mem.set_u32(array, 0);
    e.call(GEOMETRY_DESTRUCTOR, &args![geometry]);
}

// Translated from 00d5d270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d5d270 - in mode 2, lane 3 of the half extents and transform vectors is an uninitialised stack word in the exe; not reproduced.
/// `hkGeometryUtility::calcObb` (Xbox PDB), cdecl (`vertices`, `half_extents`, `transform`,
/// `mode`). Builds the convex hull of the strided `vertices` into a local `hkGeometry`
/// (`createConvexGeometry`) and flattens its triangles into a list of 3 vectors per triangle
/// (a local `hkArray` whose memory is a 128-byte-rounded bump allocation from the router's
/// first word, given back at the end). When the hull is a box (`getIsBoxGeometry` over the
/// hull's vertices with the tolerance `0.01` at `01013ea4`) its `half_extents` and
/// `transform` are left as that function made them. Otherwise the extents of the list are
/// measured in the identity basis (`00d5ae60`, its half extents `h0` and centre `c0`) and
/// the volume `hy * hx * hz` kept; `getMeanAndCovariance` (no jitter) and the axes solver
/// (`00d5ac50`) give a principal basis whose extents `h1`, `c1` and volume are measured the same
/// way. With `mode == 2` each basis is first refined by the optimiser `00d5b990` (20
/// rounds, the volume it answers replacing the old one) and its extents measured again. The
/// basis with the smaller volume is chosen (the principal one unless its volume is above the
/// identity's): `half_extents` gets its half extents and `transform` its three rows and
/// the centre in the fourth vector.
///
/// With `mode == 2` the machine code agrees with this translation in lanes 0 to 2 of
/// `half_extents` and of the `transform` vectors only; lane 3 there is a stack word the exe's
/// code never stores (which one is unconfirmed), so it is not reproduced. With the other modes
/// all four lanes agree.
pub fn hk_geometry_utility_calc_obb(
    e: &mut Engine,
    vertices: u32,
    half_extents: u32,
    transform: u32,
    mode: i32,
) {
    e.with_stack(0x120, |e, frame| {
        let s = frame.addr();
        let array = s + 0xc;
        let geometry = s + 0x24;
        let basis_identity = s + 0x80;
        let basis_principal = s + 0xb0;
        let covariance = s + 0xe0;
        for offset in [0u32, 0xc] {
            e.mem.set_u32(geometry + offset, 0);
            e.mem.set_u32(geometry + offset + 4, 0);
            e.mem.set_u32(geometry + offset + 8, 0x8000_0000);
        }
        hk_geometry_utility_create_convex_geometry(e, vertices, geometry);
        let triangles = e.mem.i32(geometry + 0x10);
        let total = triangles.wrapping_mul(3);
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let bump = e.mem.u32(router);
        let rounded = (((total as u32) << 4).wrapping_add(0x7f)) & 0xffff_ff80;
        e.mem.set_u32(router, rounded.wrapping_add(bump));
        let count_now = e.mem.i32(geometry + 0x10);
        e.mem.set_u32(array + 8, (total as u32) | 0x8000_0000);
        e.mem.set_u32(array, bump);
        e.mem.set_u32(s + 0x18, bump);
        let total = count_now.wrapping_mul(3);
        let router = e.call(MEMORY_ROUTER_GETTER_UTILITY, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < total {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if total < doubled { doubled } else { total };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
        }
        e.mem.set_i32(array + 4, total);
        if e.mem.i32(geometry + 0x10) > 0 {
            for t in 0..e.mem.u32(geometry + 0x10) {
                for k in 0..3u32 {
                    let triangle = e.mem.u32(geometry + 0xc).wrapping_add(0x10 * t);
                    let index = e.mem.u32(triangle + 4 * k);
                    let source = e.mem.u32(geometry).wrapping_add(index << 4);
                    let target = e.mem.u32(array).wrapping_add(0x30 * t + 0x10 * k);
                    copy_vec4(e, source, target);
                }
            }
        }
        let tolerance = x87_quiet(e.mem.f32(JITTER_SCALE));
        if get_is_box_geometry(e, geometry, half_extents, transform, tolerance) {
            release_obb_buffers(e, s);
            return;
        }
        let count = e.mem.i32(array + 4);
        let points = e.mem.u32(array);
        let zero = e.mem.f32(ZERO_FLOAT);
        let one = e.mem.f32(ONE_FLOAT_UTILITY);
        store_vec4(e, basis_identity, [one, zero, zero, zero]);
        store_vec4(e, basis_identity + 0x10, [zero, one, zero, zero]);
        store_vec4(e, basis_identity + 0x20, [zero, zero, one, zero]);
        fn_00d5ae60(
            e,
            points,
            basis_identity,
            count as u32,
            0x10,
            s + 0x50,
            s + 0x70,
        );
        let volume = |e: &Engine, half: u32| {
            let mut x = X87::new();
            x.fld_f32(e.mem.f32(half + 4));
            x.mul_m(float_to_x87(e.mem.f32(half)));
            x.mul_m(float_to_x87(e.mem.f32(half + 8)));
            x.fstp_f32()
        };
        let first_volume = volume(e, s + 0x50);
        e.mem.set_f32(s + 0x3c, first_volume);
        get_mean_and_covariance(e, points, count, 0x10, covariance, s + 0x60, 0);
        fn_00d5ac50(
            e,
            s + 0x23,
            covariance,
            basis_principal,
            basis_principal + 0x10,
            basis_principal + 0x20,
        );
        fn_00d5ae60(
            e,
            points,
            basis_principal,
            count as u32,
            0x10,
            s + 0x40,
            s + 0x60,
        );
        let second_volume = volume(e, s + 0x40);
        e.mem.set_f32(s + 0x1c, second_volume);
        if mode == 2 {
            let refined = fn_00d5b990(
                e,
                basis_identity,
                0x10,
                points,
                count as u32,
                basis_identity,
                0x14,
            );
            e.mem.set_f32(s + 0x3c, x87_quiet(refined));
            fn_00d5ae60(
                e,
                points,
                basis_identity,
                count as u32,
                0x10,
                s + 0x50,
                s + 0x70,
            );
            let refined = fn_00d5b990(
                e,
                basis_principal,
                0x10,
                points,
                count as u32,
                basis_principal,
                0x14,
            );
            e.mem.set_f32(s + 0x1c, x87_quiet(refined));
            fn_00d5ae60(
                e,
                points,
                basis_principal,
                count as u32,
                0x10,
                s + 0x40,
                s + 0x60,
            );
        }
        let identity_volume = e.mem.f32(s + 0x3c);
        let principal_volume = e.mem.f32(s + 0x1c);
        let flags = compare_flags(
            float_to_x87(principal_volume),
            float_to_x87(identity_volume),
        );
        let (half, centre, basis) = if flags & 0x41 == 0 {
            (s + 0x50, s + 0x70, basis_identity)
        } else {
            (s + 0x40, s + 0x60, basis_principal)
        };
        copy_vec4(e, half, half_extents);
        for row in 0..3u32 {
            copy_vec4(e, basis + 0x10 * row, transform + 0x10 * row);
        }
        copy_vec4(e, centre, transform + 0x30);
        release_obb_buffers(e, s);
    });
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5a5e0,
            hk_geometry_utility_create_convex_geometry(u32, u32)
        ),
        entry!(
            0x00d5a630,
            hk_geometry_utility_create_convex_geometry_ov2(u32, u32, u32)
        ),
        entry!(0x00d5a6a0, hk_geometry_utility_create_vertices_from_plane_equations(u32, u32) -> u32),
        entry!(0x00d5ac50, fn_00d5ac50(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d5ad60, fn_00d5ad60(u32, u32, f32) -> bool),
        entry!(0x00d5ad90, fn_00d5ad90(u32, f32) -> bool),
        entry!(0x00d5ae60, fn_00d5ae60(u32, u32, u32, u32, u32, u32)),
        entry!(0x00d5bb10, fn_00d5bb10(u32, u32, u32, f32)),
        entry!(0x00d5bb80, fn_00d5bb80(u32, u32, u32, u32, f32)),
        entry!(0x00d5c060, hk_geometry_utility_create_min_bounding_sphere(u32, u32, f32) -> u32),
        entry!(
            0x00d5c110,
            hk_geometry_utility_create_geometry_from_plane_equations(u32, u32)
        ),
        entry!(0x00d5b0c0, fn_00d5b0c0(u32, u32, u32, u32) -> f32),
        entry!(0x00d5b2a0, fn_00d5b2a0(u32, u32, u32, u32, f32, u32)),
        entry!(0x00d5b440, fn_00d5b440(u32, u32, u32, f32)),
        entry!(0x00d5b640, fn_00d5b640(u32, u32, u32, u32, f32, u32, i32) -> f32),
        entry!(0x00d5b990, fn_00d5b990(u32, u32, u32, u32, u32, i32) -> f32),
        entry!(0x00d5bce0, fn_00d5bce0(u32, u32, u32, u32, u32, f32)),
        entry!(0x00d5beb0, fn_00d5beb0(u32, u32, u32, u32, f32) -> u32),
        entry!(0x00d5a6e0, fn_00d5a6e0(u32, u32, i32) -> u32),
        entry!(
            0x00d5c1a0,
            get_mean_and_covariance(u32, i32, u32, u32, u32, u8)
        ),
        entry!(0x00d5c6d0, get_is_box_geometry(u32, u32, u32, f32) -> bool),
        entry!(0x00d5d270, hk_geometry_utility_calc_obb(u32, u32, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0106_e000,
            0x010c_4000,
            0x010d_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(ONE_FLOAT_UTILITY, 1.0);
        e.mem.set_f64(HALF_AS_DOUBLE, 0.5);
        e.mem.set_f64(ONE_AS_DOUBLE, 1.0);
        e.mem.set_f32(BUILD_INDICES_FLOAT, 0.75);
        e.mem.set_f32(START_MINIMUM, 1e30);
        e.mem.set_f32(START_MAXIMUM, -1e30);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    fn logging(e: &mut Engine, addresses: &[u32]) -> Log {
        let log: Log = Rc::default();
        for &address in addresses {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn hull_doubles(e: &mut Engine, built: i32) -> Log {
        let log = logging(
            e,
            &[
                CONVEX_HULL_CONSTRUCTOR,
                BUILD_CONFIG_CONSTRUCTOR,
                CONVEX_HULL_GENERATE_GEOMETRY,
                CONVEX_HULL_BUILD_INDICES,
                CONVEX_HULL_FETCH_PLANES,
                CONVEX_HULL_DESTRUCTOR,
            ],
        );
        let sink = Rc::clone(&log);
        e.register_double(CONVEX_HULL_BUILD, move |e, a| {
            // The configuration's first byte is already 1 when the hull is built.
            let mut words = a.to_vec();
            words.push(u32::from(e.mem.u8(a[2])));
            sink.borrow_mut().push((CONVEX_HULL_BUILD, words));
            Ret {
                eax: built as u32,
                ..Ret::default()
            }
        });
        log
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|c| c.0).collect()
    }

    #[test]
    fn the_convex_geometry_is_generated_only_when_the_hull_builds() {
        let mut e = engine();
        let log = hull_doubles(&mut e, 0);
        e.call(0x00d5_a5e0, &args![0x7000u32, 0x8000u32]);
        assert_eq!(
            addresses(&log),
            vec![
                CONVEX_HULL_CONSTRUCTOR,
                BUILD_CONFIG_CONSTRUCTOR,
                CONVEX_HULL_BUILD,
                CONVEX_HULL_GENERATE_GEOMETRY,
                CONVEX_HULL_DESTRUCTOR
            ]
        );
        let calls = log.borrow().clone();
        let hull = calls[0].1[0];
        assert_eq!(calls[2].1, vec![hull, 0x7000, calls[1].1[0], 1]);
        assert_eq!(calls[3].1, vec![hull, 0, 0x8000]);
        assert_eq!(calls[4].1, vec![hull]);
        // A failed build skips the geometry.
        let mut e = engine();
        let log = hull_doubles(&mut e, -1);
        e.call(0x00d5_a5e0, &args![0x7000u32, 0x8000u32]);
        assert_eq!(
            addresses(&log),
            vec![
                CONVEX_HULL_CONSTRUCTOR,
                BUILD_CONFIG_CONSTRUCTOR,
                CONVEX_HULL_BUILD,
                CONVEX_HULL_DESTRUCTOR
            ]
        );
    }

    #[test]
    fn the_second_form_also_builds_indices_and_fetches_planes() {
        let mut e = engine();
        let log = hull_doubles(&mut e, 0);
        e.call(0x00d5_a630, &args![0x7000u32, 0x8000u32, 0x9000u32]);
        let calls = log.borrow().clone();
        let hull = calls[0].1[0];
        assert_eq!(
            addresses(&log),
            vec![
                CONVEX_HULL_CONSTRUCTOR,
                BUILD_CONFIG_CONSTRUCTOR,
                CONVEX_HULL_BUILD,
                CONVEX_HULL_GENERATE_GEOMETRY,
                CONVEX_HULL_BUILD_INDICES,
                CONVEX_HULL_FETCH_PLANES,
                CONVEX_HULL_DESTRUCTOR
            ]
        );
        assert_eq!(calls[4].1, vec![hull, 0.75f32.to_bits(), 0]);
        assert_eq!(calls[5].1, vec![hull, 0x9000]);
        let mut e = engine();
        let log = hull_doubles(&mut e, -1);
        e.call(0x00d5_a630, &args![0x7000u32, 0x8000u32, 0x9000u32]);
        assert_eq!(log.borrow().len(), 4);
    }

    #[test]
    fn vertices_from_planes_is_the_plane_utility() {
        let mut e = engine();
        e.register(CONVEX_VERTICES_FROM_PLANES, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00d5_a6a0, &args![3u32, 4u32]).u32(), 7);
    }

    #[test]
    fn the_float_comparisons_are_strict_and_nan_fails() {
        let mut e = engine();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_f32(a, 1.0);
        e.mem.set_f32(b, 1.25);
        assert!(e.call(0x00d5_ad60, &args![a, b, 0.5f32]).bool());
        assert!(!e.call(0x00d5_ad60, &args![a, b, 0.25f32]).bool());
        assert!(!e.call(0x00d5_ad60, &args![a, b, f32::NAN]).bool());
        // The vector check: |largest absolute coordinate - 1| below epsilon.
        let v = e.mem.alloc(16);
        let check = |e: &mut Engine, x: f32, y: f32, z: f32, epsilon: f32| {
            store_vec4(e, v, [x, y, z, 0.0]);
            e.call(0x00d5_ad90, &args![v, epsilon]).bool()
        };
        assert!(check(&mut e, 0.5, -1.0, 0.25, 0.001));
        assert!(!check(&mut e, 0.5, 0.9, 0.25, 0.05));
        assert!(check(&mut e, 0.5, 0.9, 0.25, 0.2));
        assert!(check(&mut e, 0.1, 0.2, -1.5, 1.0));
        assert!(!check(&mut e, 2.0, 1.0, 0.0, 0.5));
        assert!(!check(&mut e, 1.0, 1.0, 1.0, 0.0));
    }

    #[test]
    fn the_two_point_sphere_adds_the_margin_to_half_the_distance() {
        let mut e = engine();
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00d5_bb10, &args![out, a, b, 0.5f32]);
        assert_eq!(load_vec4(&e, out), [1.0, 0.0, 0.0, 1.5]);
    }

    #[test]
    fn the_circumscribed_sphere_of_a_right_triangle() {
        let mut e = engine();
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 2.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00d5_bb80, &args![out, a, b, c, 0.25f32]);
        let sphere = load_vec4(&e, out);
        assert_eq!(sphere[..3], [1.0, 1.0, 0.0]);
        assert!((sphere[3] - (2.0f32.sqrt() + 0.25)).abs() < 1e-6);
    }

    #[test]
    fn the_geometry_from_planes_builds_from_the_vertices_or_clears_the_counts() {
        let mut e = engine();
        e.put_vtable(0x4400_7000, &[0, 0, 0, 0, 0x4400_7001]);
        let frees: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = Rc::clone(&frees);
        e.register_double(0x4400_7001, move |_, a| {
            sink.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_7000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_GETTER_UTILITY, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The plane utility fills the array with two vertices (capacity 2, owned).
        e.register(CONVEX_VERTICES_FROM_PLANES, |e, a| {
            let data = e.mem.alloc(32);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        let strided: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = Rc::clone(&strided);
        e.register_double(CONVEX_HULL_BUILD, move |e, a| {
            sink.borrow_mut()
                .extend([e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)]);
            Ret::default()
        });
        for address in [
            CONVEX_HULL_CONSTRUCTOR,
            BUILD_CONFIG_CONSTRUCTOR,
            CONVEX_HULL_GENERATE_GEOMETRY,
            CONVEX_HULL_DESTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 4, 7);
        e.mem.set_u32(geometry + 0x10, 9);
        e.call(0x00d5_c110, &args![0x6000u32, geometry]);
        let seen = strided.borrow().clone();
        assert_eq!((seen[1], seen[2]), (2, 0x10));
        assert_eq!(*frees.borrow(), vec![vec![seen[0], 32]]);
        assert_eq!(
            (e.mem.u32(geometry + 4), e.mem.u32(geometry + 0x10)),
            (7, 9)
        );
        // No vertices: the geometry's two counts are cleared and nothing is built.
        e.register(CONVEX_VERTICES_FROM_PLANES, |_, _| Ret::default());
        strided.borrow_mut().clear();
        e.call(0x00d5_c110, &args![0x6000u32, geometry]);
        assert!(strided.borrow().is_empty());
        assert_eq!(
            (e.mem.u32(geometry + 4), e.mem.u32(geometry + 0x10)),
            (0, 0)
        );
    }

    #[test]
    fn the_box_in_the_basis_takes_three_points_per_run() {
        let mut e = engine();
        let basis = e.mem.alloc(0x30);
        for (i, row) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            store_vec4(&mut e, basis + 0x10 * i as u32, row);
        }
        let points = e.mem.alloc(0x40);
        for (i, p) in [
            [0.0f32, 0.0, 0.0],
            [2.0, 4.0, 6.0],
            [1.0, 1.0, 1.0],
            [100.0, 100.0, 100.0],
        ]
        .into_iter()
        .enumerate()
        {
            store_vec4(&mut e, points + 16 * i as u32, [p[0], p[1], p[2], 5.0]);
        }
        let half = e.mem.alloc(16);
        let centre = e.mem.alloc(16);
        // 4 points make one run (4 / 3): the fourth is not looked at.
        e.call(
            0x00d5_ae60,
            &args![points, basis, 4u32, 16u32, half, centre],
        );
        let extents = load_vec4(&e, half);
        assert_eq!(extents[..3], [1.0, 2.0, 3.0]);
        assert_eq!(extents[3], -1e30);
        assert_eq!(load_vec4(&e, centre), [1.0, 2.0, 3.0, 0.0]);
        // Fewer than 3 points make no run: the start values remain.
        e.call(
            0x00d5_ae60,
            &args![points, basis, 2u32, 16u32, half, centre],
        );
        assert_eq!(load_vec4(&e, half), [-1e30; 4]);
    }

    #[test]
    fn the_minimum_bounding_sphere_copies_the_strided_points() {
        let mut e = numeric_engine();
        let (_, frees) = memory_system(&mut e);
        let data = e.mem.alloc(0x60);
        for i in 0..3u32 {
            let scale = (i + 1) as f32;
            store_vec4(
                &mut e,
                data + 16 * i,
                [scale, 2.0 * scale, 3.0 * scale, 99.0],
            );
        }
        let strided = e.mem.alloc(12);
        e.mem.set_u32(strided, data);
        e.mem.set_u32(strided + 4, 3);
        // A stride of 0x12 rounds down to 0x10.
        e.mem.set_u32(strided + 8, 0x12);
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00d5_c060, &args![out, strided, 0.125f32]).u32(),
            out
        );
        // The points are collinear: the sphere is the one of the two ends (centre (2, 4, 6),
        // half the distance) widened by the margin.
        let sphere = load_vec4(&e, out);
        assert_close(sphere[0], 2.0, 1e-3);
        assert_close(sphere[1], 4.0, 1e-3);
        assert_close(sphere[2], 6.0, 1e-3);
        assert_close(sphere[3], 56.0f32.sqrt() / 2.0 + 0.125, 1e-3);
        // The block of 16 bytes per point went back through `easyFree`.
        assert_eq!(frees.borrow().len(), 1);
    }

    #[test]
    fn the_axes_are_the_solved_rows_made_right_handed() {
        let mut e = numeric_engine();
        // The exe's sqrt/acos error path (a NaN or negative argument) sets errno, a word of
        // the thread's data, which the bare engine does not have: doubled.
        let errno = e.mem.alloc(4);
        e.register_double(0x00ec_85e3, move |_, _| errno.into_ret());
        let record = e.mem.alloc(4);
        let original = [
            [4.0f32, 1.0, 2.0, 0.0],
            [1.0, 3.0, 0.0, 0.0],
            [2.0, 0.0, 1.0, 0.0],
        ];
        let matrix = rows_at(&mut e, &original);
        let outputs = [e.mem.alloc(16), e.mem.alloc(16), e.mem.alloc(16)];
        assert_eq!(
            e.call(
                0x00d5_ac50,
                &args![record, matrix, outputs[0], outputs[1], outputs[2]]
            )
            .u32(),
            record
        );
        assert_eq!(e.mem.u8(record), 1);
        let axes = outputs.map(|at| load_vec4(&e, at));
        assert_eigenvectors(&axes, &original);
        // The reflection of the first step leaves the solved rows left-handed: the third has
        // its signs flipped, so that (x cross y) . z is +1.
        let [x, y, z] = axes;
        let cross = [
            x[1] * y[2] - x[2] * y[1],
            x[2] * y[0] - x[0] * y[2],
            x[0] * y[1] - x[1] * y[0],
        ];
        assert_close(
            cross[0] * z[0] + cross[1] * z[1] + cross[2] * z[2],
            1.0,
            1e-4,
        );
        // A diagonal matrix keeps its rows (a right-handed identity): the third is not flipped.
        let diagonal = rows_at(
            &mut e,
            &[
                [3.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        e.call(
            0x00d5_ac50,
            &args![record, diagonal, outputs[0], outputs[1], outputs[2]],
        );
        assert_eq!(load_vec4(&e, outputs[0]), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, outputs[1]), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, outputs[2]), [0.0, 0.0, 1.0, 0.0]);
        // A matrix that never converges (a NaN entry is never small) leaves the flag 0 and the
        // axes default to the unit axes.
        let unsolvable = rows_at(&mut e, &[[f32::NAN; 4]; 3]);
        for at in outputs {
            store_vec4(&mut e, at, [2.0; 4]);
        }
        e.mem.set_u8(record, 9);
        e.call(
            0x00d5_ac50,
            &args![record, unsolvable, outputs[0], outputs[1], outputs[2]],
        );
        assert_eq!(e.mem.u8(record), 0);
        assert_eq!(load_vec4(&e, outputs[0]), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, outputs[1]), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, outputs[2]), [0.0, 0.0, 1.0, 0.0]);
    }

    type Frees = Rc<RefCell<Vec<(u32, u32)>>>;

    fn numeric_engine() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_3000,
            0x0101_4000,
            0x0101_7000,
            0x0101_a000,
            0x0102_0000,
            0x0102_1000,
            0x0102_9000,
            0x0102_e000,
            0x0103_5000,
            0x0106_c000,
            0x0107_1000,
            0x0109_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        e.mem.set_f64(THREE_AS_DOUBLE, 3.0);
        e.mem.set_f64(FIVE_AS_DOUBLE, 5.0);
        e.mem.set_f64(GROWTH_AS_DOUBLE, f64::from(1.2f32));
        e.mem.set_f64(NINE_AS_DOUBLE, 9.0);
        e.mem.set_f64(TWELVE_AS_DOUBLE, 12.0);
        e.mem.set_f64(MINUS_ONE_AS_DOUBLE, -1.0);
        e.mem.set_f64(BOX_FACTOR, f64::from(1.001f32));
        e.mem.set_f32(FIRST_STEP, 0.001);
        e.mem.set_f32(GRADIENT_EPSILON, 0.0001);
        e.mem.set_f32(STOP_STEP, f32::from_bits(0x35a0_0000));
        e.mem.set_f32(JITTER_SCALE, 0.01);
        e.mem.set_f32(THIRD_FLOAT, f32::from_bits(0x3eaa_aaab));
        e.mem.set_f32(SPHERE_EPSILON, 0.00001);
        e.mem.set_f32(OFF_DIAGONAL_LIMIT, 0.000001);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK_UTILITY + 4 * lane, 0x7fff_ffff);
        }
        e
    }

    /// The memory router the code reaches through `00c85750`: its allocator hands out blocks
    /// (`alloc(&size)`, slot `0xc`) and logs what is given back (`free(data, bytes)`, slot
    /// `0x10`; `easyFree` logs the block with 0 bytes); `_reserve` and `_reserveMore` grow an
    /// `hkArray`. The router's first word is the bump pointer into a 16 KiB arena.
    fn memory_system(e: &mut Engine) -> (u32, Frees) {
        let frees: Frees = Rc::default();
        e.put_vtable(0x4400_7000, &[0, 0, 0, 0x4400_7001, 0x4400_7002]);
        e.register(0x4400_7001, |e, a| {
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&frees);
        e.register_double(0x4400_7002, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e.register(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let sink = Rc::clone(&frees);
        e.register_double(EASY_FREE, move |_, a| {
            sink.borrow_mut().push((a[1], 0));
            Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_7000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        let arena = e.mem.alloc(0x4000);
        e.mem.set_u32(router, arena);
        e.register_double(MEMORY_ROUTER_GETTER_UTILITY, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let (array, capacity, element) = (a[1], a[2], a[3]);
            grow_array(e, array, capacity, element);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let array = a[1];
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) * 2;
            grow_array(e, array, capacity.max(4), a[2]);
            Ret::default()
        });
        (router, frees)
    }

    fn grow_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let data = e.mem.alloc(capacity * element);
        let old = e.mem.u32(array);
        for i in 0..e.mem.u32(array + 4) * element {
            let byte = e.mem.u8(old + i);
            e.mem.set_u8(data + i, byte);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Doubles for the hull builder: the geometry the hull generates is `vertices` with
    /// `triangles` (indices at `+0`, `+4`, `+8` of 16-byte entries).
    fn hull_of(e: &mut Engine, vertices: &[Vec4], triangles: &[[u32; 3]]) {
        for address in [
            CONVEX_HULL_CONSTRUCTOR,
            BUILD_CONFIG_CONSTRUCTOR,
            CONVEX_HULL_DESTRUCTOR,
            GEOMETRY_DESTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CONVEX_HULL_BUILD, |_, _| Ret::default());
        let vertices = vertices.to_vec();
        let triangles = triangles.to_vec();
        e.register_double(CONVEX_HULL_GENERATE_GEOMETRY, move |e, a| {
            let geometry = a[2];
            let count = vertices.len() as u32;
            let data = e.mem.alloc(16 * count);
            for (i, v) in vertices.iter().enumerate() {
                store_vec4(e, data + 16 * i as u32, *v);
            }
            e.mem.set_u32(geometry, data);
            e.mem.set_u32(geometry + 4, count);
            e.mem.set_u32(geometry + 8, 0x8000_0000 | count);
            let total = triangles.len() as u32;
            let list = e.mem.alloc(16 * total);
            for (i, triangle) in triangles.iter().enumerate() {
                for (k, index) in triangle.iter().enumerate() {
                    e.mem.set_u32(list + 16 * i as u32 + 4 * k as u32, *index);
                }
            }
            e.mem.set_u32(geometry + 0xc, list);
            e.mem.set_u32(geometry + 0x10, total);
            e.mem.set_u32(geometry + 0x14, 0x8000_0000 | total);
            Ret::default()
        });
    }

    fn rows_at(e: &mut Engine, rows: &[Vec4]) -> u32 {
        let at = e.mem.alloc(0x10 * rows.len() as u32 + 0x10);
        for (i, row) in rows.iter().enumerate() {
            store_vec4(e, at + 0x10 * i as u32, *row);
        }
        at
    }

    /// The corners of a square on the diagonals of the x and y axes at z = 0 and two points
    /// above and below its middle: the box is 1 x 1 x 1 (half extents) in the world axes and
    /// `cos^2` of the angle in axes turned about z by up to 45 degrees (the extreme points are
    /// all at z = 0, so tilting a row out of the plane changes nothing at first order).
    const SQUARE: [[f32; 3]; 6] = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
    ];

    fn square_points(e: &mut Engine) -> u32 {
        let points: Vec<Vec4> = SQUARE.iter().map(|p| [p[0], p[1], p[2], 0.0]).collect();
        rows_at(e, &points)
    }

    fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    /// The product of the half extents of `points` in the frame `00d5b0c0` builds from two rows
    /// (in `f64`, with the same steps).
    fn reference_volume(rows: [[f64; 3]; 2], points: &[[f32; 3]]) -> f64 {
        let unit = |v: [f64; 3]| {
            let length = dot3(v, v).sqrt();
            [v[0] / length, v[1] / length, v[2] / length]
        };
        let n0 = unit(rows[0]);
        let along = dot3(rows[1], rows[0]);
        let n1 = unit([
            rows[1][0] - along * rows[0][0],
            rows[1][1] - along * rows[0][1],
            rows[1][2] - along * rows[0][2],
        ]);
        let n2 = unit([
            rows[0][1] * rows[1][2] - rows[0][2] * rows[1][1],
            rows[0][2] * rows[1][0] - rows[0][0] * rows[1][2],
            rows[0][0] * rows[1][1] - rows[0][1] * rows[1][0],
        ]);
        let mut volume = 1.0;
        for axis in [n0, n1, n2] {
            let projections: Vec<f64> = points
                .iter()
                .map(|p| dot3([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])], axis))
                .collect();
            let high = projections.iter().cloned().fold(f64::MIN, f64::max);
            let low = projections.iter().cloned().fold(f64::MAX, f64::min);
            volume *= (high - low) / 2.0;
        }
        volume
    }

    fn row3(row: Vec4) -> [f64; 3] {
        [f64::from(row[0]), f64::from(row[1]), f64::from(row[2])]
    }

    fn assert_close(actual: f32, expected: f32, tolerance: f32) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} is not within {tolerance} of {expected}"
        );
    }

    /// Three unit rows at right angles, each an eigenvector of the symmetric matrix `a`.
    fn assert_eigenvectors(axes: &[Vec4; 3], a: &[Vec4; 3]) {
        for (i, v) in axes.iter().enumerate() {
            assert_close(sum3(mul4(*v, *v)), 1.0, 1e-4);
            for w in &axes[i + 1..] {
                assert_close(sum3(mul4(*v, *w)), 0.0, 1e-4);
            }
            let image = [0, 1, 2].map(|k| sum3(mul4(a[k], *v)));
            let value = image[0] * v[0] + image[1] * v[1] + image[2] * v[2];
            for (got, lane) in image.iter().zip(v) {
                assert_close(*got, value * lane, 1e-3);
            }
        }
    }

    #[test]
    fn the_volume_is_the_product_of_the_half_extents_in_the_orthonormal_frame() {
        let mut e = numeric_engine();
        let points = square_points(&mut e);
        let volume = |e: &mut Engine, rows: [Vec4; 2]| {
            let at = rows_at(e, &rows);
            e.call(0x00d5_b0c0, &args![at, 16u32, points, 6u32]).f32()
        };
        let identity = volume(&mut e, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]);
        assert_close(identity, 1.0, 1e-5);
        // The second row loses its part along the first: (-0.5, 0.5, 0) is left of (0, 1, 0).
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let turned = volume(&mut e, [[h, h, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]);
        assert_close(turned, 0.5, 1e-4);
        // A longer first row is only a direction.
        let scaled = volume(&mut e, [[3.0 * h, 3.0 * h, 0.0, 0.0], [-h, h, 0.0, 0.0]]);
        assert_close(scaled, 0.5, 1e-4);
        // A zero first row has no direction: nothing is measured.
        assert_eq!(volume(&mut e, [[0.0; 4], [0.0, 1.0, 0.0, 0.0]]), 0.0);
    }

    #[test]
    fn the_gradient_of_the_volume_is_negated_and_limited_to_one() {
        let mut e = numeric_engine();
        let points = square_points(&mut e);
        let angle = 10.0f64.to_radians();
        let (c, s) = (angle.cos() as f32, angle.sin() as f32);
        let params = rows_at(&mut e, &[[c, s, 0.0, 0.0], [-s, c, 0.0, 0.0]]);
        let gradient = e.mem.alloc(0x20);
        e.call(
            0x00d5_b2a0,
            &args![params, points, 6u32, 16u32, 0.0001f32, gradient],
        );
        // The same forward differences in f64.
        let base_rows = [
            [f64::from(c), f64::from(s), 0.0],
            [-f64::from(s), f64::from(c), 0.0],
        ];
        let base = reference_volume(base_rows, &SQUARE);
        let epsilon = f64::from(0.0001f32);
        let mut expected = [[0.0f64; 3]; 2];
        for (row, values) in expected.iter_mut().enumerate() {
            for (lane, value) in values.iter_mut().enumerate() {
                let mut bumped = base_rows;
                bumped[row][lane] += epsilon;
                *value = -(reference_volume(bumped, &SQUARE) - base) / epsilon;
            }
        }
        let squared: f64 = expected.iter().map(|r| dot3(*r, *r)).sum();
        let scale = if squared > 1.0 {
            1.0 / squared.sqrt()
        } else {
            1.0
        };
        for (row, wanted) in expected.iter().enumerate() {
            let found = row3(load_vec4(&e, gradient + 0x10 * row as u32));
            for (lane, (got, want)) in found.iter().zip(wanted).enumerate() {
                assert!(
                    (got - want * scale).abs() < 0.03,
                    "row {row} lane {lane}: {got} against {}",
                    want * scale
                );
            }
        }
        // The volume falls toward larger angles: the y lane of the first row has the sign
        // of the descent.
        assert!(load_vec4(&e, gradient)[1] > 0.0);
    }

    #[test]
    fn the_moved_rows_are_made_orthonormal_again() {
        let mut e = numeric_engine();
        let work = e.mem.alloc(0x30);
        let source = rows_at(&mut e, &[[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]);
        let delta = rows_at(&mut e, &[[0.0, 1.0, 0.0, 0.0], [-1.0, 0.0, 0.0, 0.0]]);
        // t = 1 turns the rows to 45 degrees.
        e.call(0x00d5_b440, &args![work, source, delta, 1.0f32]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let expected = [[h, h, 0.0, 0.0], [-h, h, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]];
        for (k, row) in expected.into_iter().enumerate() {
            let found = load_vec4(&e, work + 0x10 * k as u32);
            for (got, want) in found.iter().zip(row) {
                assert_close(*got, want, 1e-6);
            }
        }
        // t = 0 leaves the source rows (as unit vectors).
        e.call(0x00d5_b440, &args![work, source, delta, 0.0f32]);
        let first = load_vec4(&e, work);
        assert_close(first[0], 1.0, 1e-6);
        assert_close(first[1], 0.0, 1e-6);
    }

    #[test]
    fn the_line_search_finds_the_lowest_volume_along_the_rows_delta() {
        let mut e = numeric_engine();
        let points = square_points(&mut e);
        let rows = rows_at(&mut e, &[[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]);
        let delta = rows_at(&mut e, &[[0.0, 1.0, 0.0, 0.0], [-1.0, 0.0, 0.0, 0.0]]);
        // The volume is 1 / (1 + t^2) up to t = 1 (45 degrees) and rises again after it.
        let step = e
            .call(
                0x00d5_b640,
                &args![rows, 16u32, points, 6u32, 0.1f32, delta, 30i32],
            )
            .f32();
        assert_close(step, 1.0, 0.01);
        // Starting at the lowest volume (the 45-degree frame) every step rises, the step is
        // halved until the rounds are used up and the answer is -1.
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let lowest = rows_at(&mut e, &[[h, h, 0.0, 0.0], [-h, h, 0.0, 0.0]]);
        let step = e
            .call(
                0x00d5_b640,
                &args![lowest, 16u32, points, 6u32, 0.1f32, delta, 5i32],
            )
            .f32();
        assert_eq!(step, -1.0);
    }

    #[test]
    fn the_optimiser_turns_the_rows_to_the_points() {
        let mut e = numeric_engine();
        let points = square_points(&mut e);
        let angle = 10.0f64.to_radians();
        let (c, s) = (angle.cos() as f32, angle.sin() as f32);
        let source = rows_at(
            &mut e,
            &[[c, s, 0.0, 0.0], [-s, c, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
        );
        let work = e.mem.alloc(0x30);
        // No rounds: the value at the start, the rows copied.
        let start = e
            .call(0x00d5_b990, &args![work, 16u32, points, 6u32, source, 0i32])
            .f32();
        assert_close(start, c * c, 1e-3);
        for k in 0..3 {
            assert_eq!(
                load_vec4(&e, work + 0x10 * k),
                load_vec4(&e, source + 0x10 * k)
            );
        }
        // Rounds of gradient steps lower it toward the 45-degree frame (0.5).
        let best = e
            .call(
                0x00d5_b990,
                &args![work, 16u32, points, 6u32, source, 20i32],
            )
            .f32();
        assert!(best < 0.6, "{best}");
        assert!(best > 0.499, "{best}");
        let rows = [row3(load_vec4(&e, work)), row3(load_vec4(&e, work + 0x10))];
        let measured = reference_volume(rows, &SQUARE) as f32;
        assert_close(measured, best, 2e-3);
    }

    #[test]
    fn the_four_point_sphere_is_the_circumsphere() {
        let mut e = numeric_engine();
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 2.0, 0.0, 0.0]);
        let d = vector(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00d5_bce0, &args![out, a, b, c, d, 0.25f32]);
        let sphere = load_vec4(&e, out);
        for centre in &sphere[..3] {
            assert_close(*centre, 1.0, 1e-5);
        }
        assert_close(sphere[3], 3.0f32.sqrt() + 0.25, 1e-5);
    }

    #[test]
    fn the_recursive_sphere_holds_every_point() {
        let mut e = numeric_engine();
        // A right-angled corner: the circle of the three far points holds the origin.
        let points = rows_at(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
            ],
        );
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00d5_beb0, &args![out, points, 4u32, 0u32, 0.0f32])
                .u32(),
            out
        );
        let sphere = load_vec4(&e, out);
        for centre in &sphere[..3] {
            assert_close(*centre, 2.0 / 3.0, 2e-3);
        }
        assert_close(sphere[3], (24.0f32 / 9.0).sqrt(), 2e-3);
        // Zero points: the zero vector with radius -1.
        assert_eq!(
            e.call(0x00d5_beb0, &args![out, points, 0u32, 0u32, 0.5f32])
                .u32(),
            out
        );
        assert_eq!(load_vec4(&e, out), [0.0, 0.0, 0.0, -1.0]);
        // One support point (the one just below `points`): its place with the margin as the
        // radius.
        let first = load_vec4(&e, points);
        e.call(0x00d5_beb0, &args![out, points + 0x10, 0u32, 1u32, 0.5f32]);
        assert_eq!(load_vec4(&e, out), [first[0], first[1], first[2], 0.5]);
    }

    #[test]
    fn the_symmetric_eigen_solver_finds_the_axes() {
        let mut e = numeric_engine();
        let record = e.mem.alloc(4);
        let original = [
            [4.0f32, 1.0, 2.0, 0.0],
            [1.0, 3.0, 0.0, 0.0],
            [2.0, 0.0, 1.0, 0.0],
        ];
        let matrix = rows_at(&mut e, &original);
        e.call(0x00d5_a6e0, &args![record, matrix, 30i32]);
        assert_eq!(e.mem.u8(record), 1);
        let axes = [0, 1, 2].map(|k| load_vec4(&e, matrix + 0x10 * k));
        assert_eigenvectors(&axes, &original);
        // Too few rounds: the flag stays 0.
        let matrix = rows_at(&mut e, &original);
        e.mem.set_u8(record, 7);
        e.call(0x00d5_a6e0, &args![record, matrix, 0i32]);
        assert_eq!(e.mem.u8(record), 0);
        // A diagonal matrix is already solved: the rows are the unit axes.
        let diagonal = rows_at(
            &mut e,
            &[
                [3.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        e.mem.set_u8(record, 0);
        e.call(0x00d5_a6e0, &args![record, diagonal, 30i32]);
        assert_eq!(e.mem.u8(record), 1);
        assert_eq!(load_vec4(&e, diagonal), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, diagonal + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, diagonal + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00d5a6e0_answers_its_result_pointer() {
        let mut e = numeric_engine();
        let record = e.mem.alloc(4);
        let original = [
            [4.0f32, 1.0, 2.0, 0.0],
            [1.0, 3.0, 0.0, 0.0],
            [2.0, 0.0, 1.0, 0.0],
        ];
        let matrix = rows_at(&mut e, &original);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00d5_a6e0, &args![record, matrix, 30i32]).u32(),
            args![record][0]
        );
    }

    #[test]
    fn the_mean_and_covariance_are_weighted_by_the_triangle_areas() {
        let mut e = numeric_engine();
        let (_, frees) = memory_system(&mut e);
        let vertices = rows_at(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 7.0],
                [2.0, 0.0, 0.0, 7.0],
                [0.0, 2.0, 0.0, 7.0],
            ],
        );
        let covariance = e.mem.alloc(0x30);
        let mean = e.mem.alloc(16);
        e.call(
            0x00d5_c1a0,
            &args![vertices, 3i32, 16u32, covariance, mean, 0u32],
        );
        let third = 2.0f32 / 3.0;
        let found = load_vec4(&e, mean);
        assert_close(found[0], third, 1e-6);
        assert_close(found[1], third, 1e-6);
        assert_close(found[2], 0.0, 1e-6);
        // The triangle's area is 2: the entries are those of the moments of the triangle
        // times its area, less the mean's product.
        let expected = [
            [4.0f32 / 9.0, -2.0 / 9.0, 0.0],
            [-2.0 / 9.0, 4.0 / 9.0, 0.0],
            [0.0, 0.0, 0.0],
        ];
        for (r, row) in expected.into_iter().enumerate() {
            let found = load_vec4(&e, covariance + 0x10 * r as u32);
            for (k, value) in row.into_iter().enumerate() {
                assert_close(found[k], value, 1e-5);
            }
        }
        // The centroid array's memory (one 16-byte entry) was given back.
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(frees.borrow()[0].1, 0x10);
        // Fewer than three vertices: no triangle, the identity and a zero mean.
        e.call(
            0x00d5_c1a0,
            &args![vertices, 2i32, 16u32, covariance, mean, 0u32],
        );
        assert_eq!(load_vec4(&e, mean), [0.0; 4]);
        assert_eq!(load_vec4(&e, covariance), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, covariance + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, covariance + 0x20), [0.0, 0.0, 1.0, 0.0]);
        // A triangle without area adds nothing: the same answer.
        let flat = rows_at(&mut e, &[[1.0; 4], [1.0; 4], [1.0; 4]]);
        e.call(
            0x00d5_c1a0,
            &args![flat, 3i32, 16u32, covariance, mean, 0u32],
        );
        assert_eq!(load_vec4(&e, mean), [0.0; 4]);
        assert_eq!(load_vec4(&e, covariance), [1.0, 0.0, 0.0, 0.0]);
    }

    /// Doubles for the transform helpers `getIsBoxGeometry` calls without a name: the
    /// rotation by the rows (`00c7f700`: out, rows, vector), the copy of a transform
    /// (`00cb21e0`: to, from), the rotation fix (`00cd4f10`, left alone) and the transform of
    /// a point (`00c7f640`: out, transform, point).
    fn transform_doubles(e: &mut Engine) {
        fn apply(e: &mut Engine, out: u32, rows: u32, point: u32, translate: bool) {
            let p = load_vec4(e, point);
            let mut result = [0.0f32; 4];
            for (k, slot) in result.iter_mut().take(3).enumerate() {
                let row = load_vec4(e, rows + 0x10 * k as u32);
                *slot = row[0] * p[0] + row[1] * p[1] + row[2] * p[2];
                if translate {
                    *slot += e.mem.f32(rows + 0x30 + 4 * k as u32);
                }
            }
            store_vec4(e, out, result);
        }
        e.register(0x00cd_4f10, |_, _| Ret::default());
        e.register(ROTATE_VECTOR, |e, a| {
            apply(e, a[0], a[1], a[2], false);
            Ret::default()
        });
        e.register(TRANSFORM_VECTOR, |e, a| {
            apply(e, a[0], a[1], a[2], true);
            Ret::default()
        });
        e.register(COPY_TRANSFORM, |e, a| {
            for k in 0..4 {
                let v = load_vec4(e, a[1] + 0x10 * k);
                store_vec4(e, a[0] + 0x10 * k, v);
            }
            Ret::default()
        });
    }

    /// The corners of the box with half extents (1, 2, 3) about (1, 2, 3), the maximum
    /// corner first, then the centre.
    fn box_points() -> Vec<Vec4> {
        let mut points = Vec::new();
        for sx in [1.0f32, -1.0] {
            for sy in [1.0f32, -1.0] {
                for sz in [1.0f32, -1.0] {
                    points.push([1.0 + sx, 2.0 + 2.0 * sy, 3.0 + 3.0 * sz, 0.0]);
                }
            }
        }
        points.push([1.0, 2.0, 3.0, 0.0]);
        points
    }

    #[test]
    fn eight_points_on_a_sphere_with_right_angles_are_a_box() {
        let mut e = numeric_engine();
        let (_, frees) = memory_system(&mut e);
        transform_doubles(&mut e);
        let points = box_points();
        let data = rows_at(&mut e, &points);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, points.len() as u32);
        let half = rows_at(&mut e, &[[0.0, 0.0, 0.0, 4.0]]);
        let transform = e.mem.alloc(0x40);
        assert!(e
            .call(0x00d5_c6d0, &args![list, half, transform, 0.01f32])
            .bool());
        let found = load_vec4(&e, half);
        assert_close(found[0], 1.0, 1e-4);
        assert_close(found[1], 2.0, 1e-4);
        assert_close(found[2], 3.0, 1e-4);
        // The rows were axis-aligned: the rotation is the identity and the extents are
        // made absolute with lane 3 cleared.
        assert_eq!(found[3], 0.0);
        assert_eq!(load_vec4(&e, transform), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, transform + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, transform + 0x20), [0.0, 0.0, 1.0, 0.0]);
        let translation = load_vec4(&e, transform + 0x30);
        assert_close(translation[0], -1.0, 1e-4);
        assert_close(translation[1], -2.0, 1e-4);
        assert_close(translation[2], -3.0, 1e-4);
        // The sphere's point block (`easyFree`), the corner array (grown to 8 entries) and the
        // 0x80-byte block were given back.
        let sizes: Vec<u32> = frees.borrow().iter().map(|f| f.1).collect();
        assert_eq!(sizes, vec![0, 0x80, 0x80]);
    }

    #[test]
    fn other_than_eight_points_on_the_sphere_is_not_a_box() {
        let mut e = numeric_engine();
        let (_, frees) = memory_system(&mut e);
        transform_doubles(&mut e);
        let data = rows_at(&mut e, &[[0.0; 4], [2.0, 0.0, 0.0, 0.0]]);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, 2);
        let half = e.mem.alloc(16);
        let transform = e.mem.alloc(0x40);
        assert!(!e
            .call(0x00d5_c6d0, &args![list, half, transform, 0.01f32])
            .bool());
        // The sphere's block, the two collected points' array (4 entries) and the 0x80-byte
        // block were given back.
        let sizes: Vec<u32> = frees.borrow().iter().map(|f| f.1).collect();
        assert_eq!(sizes, vec![0, 0x40, 0x80]);
        // Eight points on a sphere whose nearest edges are not at right angles (a square
        // antiprism) fail the perpendicularity test.
        let mut points = Vec::new();
        for (z, turn) in [(0.6f64, 0.0f64), (-0.6, 45.0)] {
            for k in 0..4 {
                let angle = (turn + 90.0 * f64::from(k)).to_radians();
                points.push([
                    (0.8 * angle.cos()) as f32,
                    (0.8 * angle.sin()) as f32,
                    z as f32,
                    0.0,
                ]);
            }
        }
        let data = rows_at(&mut e, &points);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, points.len() as u32);
        assert!(!e
            .call(0x00d5_c6d0, &args![list, half, transform, 0.01f32])
            .bool());
    }

    #[test]
    fn the_obb_of_a_box_hull_is_the_box() {
        let mut e = numeric_engine();
        let (router, frees) = memory_system(&mut e);
        transform_doubles(&mut e);
        let corners = box_points();
        hull_of(
            &mut e,
            &corners[..8],
            &[
                [0, 1, 2],
                [1, 3, 2],
                [4, 5, 6],
                [5, 7, 6],
                [0, 1, 4],
                [1, 5, 4],
                [2, 3, 6],
                [3, 7, 6],
                [0, 2, 4],
                [2, 6, 4],
                [1, 3, 5],
                [3, 7, 5],
            ],
        );
        let bump = e.mem.u32(router);
        let half = e.mem.alloc(16);
        let transform = e.mem.alloc(0x40);
        e.call(0x00d5_d270, &args![0x7000u32, half, transform, 0i32]);
        let found = load_vec4(&e, half);
        assert_close(found[0], 1.0, 1e-4);
        assert_close(found[1], 2.0, 1e-4);
        assert_close(found[2], 3.0, 1e-4);
        assert_eq!(load_vec4(&e, transform), [1.0, 0.0, 0.0, 0.0]);
        // The router's bump pointer is back where it started.
        assert_eq!(e.mem.u32(router), bump);
        assert!(!frees.borrow().is_empty());
    }

    #[test]
    fn the_obb_of_other_hulls_is_the_smaller_of_the_two_bases() {
        let mut e = numeric_engine();
        let (router, _) = memory_system(&mut e);
        transform_doubles(&mut e);
        let vertices: [Vec4; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 0.0, 0.0],
            [1.0, 1.0, 3.0, 0.0],
        ];
        let triangles = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];
        hull_of(&mut e, &vertices, &triangles);
        let bump = e.mem.u32(router);
        let mut volumes = Vec::new();
        for mode in [0i32, 2] {
            let half = e.mem.alloc(16);
            let transform = e.mem.alloc(0x40);
            e.call(0x00d5_d270, &args![0x7000u32, half, transform, mode]);
            assert_eq!(e.mem.u32(router), bump);
            let extents = load_vec4(&e, half);
            volumes.push(extents[0] * extents[1] * extents[2]);
            // The box holds every vertex.
            let rows = [0, 1, 2].map(|k| load_vec4(&e, transform + 0x10 * k));
            let centre = load_vec4(&e, transform + 0x30);
            for p in &vertices {
                for k in 0..3 {
                    let d = (p[0] - centre[0]) * rows[k][0]
                        + (p[1] - centre[1]) * rows[k][1]
                        + (p[2] - centre[2]) * rows[k][2];
                    assert!(
                        d.abs() <= extents[k] * 1.001 + 1e-4,
                        "mode {mode} axis {k}: {d} outside {}",
                        extents[k]
                    );
                }
            }
        }
        // The box in the world axes is 1.5 x 1 x 1.5 (volume 2.25): neither basis is larger.
        assert!(volumes[0] <= 2.25 * 1.001, "{}", volumes[0]);
        assert!(volumes[1] <= volumes[0] * 1.001, "{volumes:?}");
    }
}
