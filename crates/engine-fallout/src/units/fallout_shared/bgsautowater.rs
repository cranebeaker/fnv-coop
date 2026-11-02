//! `fallout shared/bgsautowater.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const FREE: u32 = 0x00aa_1460;
const ARRAY_FREE: u32 = 0x0040_1030;
const ARRAY_SET_SIZE: u32 = 0x0049_fb50;

// Translated from 0049db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first item in the node stored at `this + 0x28`, or null.
pub fn fn_0049db30(e: &mut Engine, this: Ptr) -> Ptr {
    let node = e.mem.u32(this.addr().wrapping_add(0x28));
    Ptr::new(if node == 0 { 0 } else { e.mem.u32(node + 4) })
}

// Translated from 0049da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Classifies a point against a plane as negative, on-plane, or positive.
/// Each x87 instruction of the distance (0049da8c-0049dab8) rounds at the
/// running thread's precision; the distance is stored to a `float` local.
pub fn fn_0049da80(e: &mut Engine, plane: Ptr, point: Ptr) -> u32 {
    let p = plane.addr();
    let q = point.addr();
    // 0049da8c-0049da9f: FLD/FMUL x, FLD/FMUL y, FADDP.
    let x = x87_mul(x87_load32(e.mem.u32(p)), x87_m32(e.mem.u32(q)));
    let y = x87_mul(
        x87_load32(e.mem.u32(p.wrapping_add(4))),
        x87_m32(e.mem.u32(q.wrapping_add(4))),
    );
    let sum = x87_add(x, y);
    // 0049daa4-0049daad: FLD/FMUL z, FADDP.
    let z = x87_mul(
        x87_load32(e.mem.u32(p.wrapping_add(8))),
        x87_m32(e.mem.u32(q.wrapping_add(8))),
    );
    let sum = x87_add(sum, z);
    // 0049dab2-0049dab8: FSUB the plane's +0xc, FSTP/FLD the float local.
    let distance = x87_store32(x87_sub(sum, x87_m32(e.mem.u32(p.wrapping_add(0xc)))));
    let distance = x87_load32(distance);
    let first_zero = e.global::<f64>(0x0101_2060);
    if distance < first_zero {
        return 2;
    }
    let second_zero = e.global::<f64>(0x0101_2060);
    if distance > second_zero {
        1
    } else {
        0
    }
}

// Translated from 0049db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0xa0 of the object stored at `this + 0x28`, or
/// zero when that pointer is null. GatherLineSegments passes GetLand's result.
pub fn fn_0049db00(e: &mut Engine, this: Ptr) -> f32 {
    let land = e.mem.u32(this.addr().wrapping_add(0x28));
    if land == 0 {
        0.0
    } else {
        // 0049db18-0049db28: FLD float, FSTP float local, FLD (a signalling NaN is quieted).
        x87_quiet(e.mem.f32(land.wrapping_add(0xa0)))
    }
}

// Translated from 0049c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater::GeneratePlaceableWaterForCell` (Xbox PDB): runs the three
/// auto-water generation passes when the cell's +0x24 flag has
/// bit 1 set, bracketed by the executable's TLS context guard.
// Translation incomplete: 0049c860 - C++ SEH unwind cleanup is omitted.
pub fn bg_auto_water_generate_placeable_water_for_cell(e: &mut Engine, cell: Ptr) {
    e.with_stack(4, |e, timer| {
        e.call(0x0040_4eb0, &args![timer, 0x1d, 1, 0x0101_e018, 0x53]);
        if cell.addr() != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
            let point_array = e.mem.u32(0x011c_57ac);
            if point_array != 0 {
                e.call(0x0084_54f0, &args![point_array, 1]);
            }
            let water_list = e.mem.u32(0x011c_57b0);
            if water_list != 0 {
                e.call(0x004e_d900, &args![water_list]);
            }
            bg_auto_water_gather_line_segments(e, cell);
            bg_auto_water_generate_polygons(e, cell);
            bg_auto_water_generate_placeable_water(e, cell);
        }
        e.call(0x0040_4ee0, &args![timer]);
    });
}

// Translated from 0049db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater::GeneratePolygons` (Xbox PDB): joins the accumulated line
/// segments into polygons and removes overlapping polygons from the list.
// Translation incomplete: 0049db60 - compiler SEH registration and exception unwind cleanup are omitted.
pub fn bg_auto_water_generate_polygons(e: &mut Engine, cell: Ptr) {
    e.with_stack(0x50, |e, scratch| {
        let timer = scratch;
        let first: Ptr = Ptr::new(scratch.addr() + 4);
        let second: Ptr = Ptr::new(scratch.addr() + 16);
        let offset: Ptr = Ptr::new(scratch.addr() + 28);
        let cursor: Ptr = Ptr::new(scratch.addr() + 40);
        let polygon_slot: Ptr = Ptr::new(scratch.addr() + 44);
        e.call(0x0040_4eb0, &args![timer, 0x1d, 1, 0x0101_e018, 0x20a]);
        loop {
            let points = e.mem.u32(0x011c_57ac);
            if e.call(0x0044_ddc0, &args![points]).u32() == 0 {
                break;
            }
            let allocation = e.call(0x0040_1000, &args![0x10]).u32();
            let chain = if allocation == 0 {
                0
            } else {
                e.call(0x0049_f1c0, &args![allocation]).u32()
            };
            for index in [0, 1] {
                let points = e.mem.u32(0x011c_57ac);
                let point = e.call(0x006a_7af0, &args![points, index]).u32();
                e.call(0x0049_f210, &args![chain, point]);
            }
            let points = e.mem.u32(0x011c_57ac);
            fn_0049f3c0(e, Ptr::new(points), 0, 2, 1);
            let mut extended = true;
            let mut closed = false;
            while extended {
                extended = false;
                let mut index = 0i32;
                loop {
                    let points = e.mem.u32(0x011c_57ac);
                    if index >= e.call(0x0044_ddc0, &args![points]).u32() as i32 {
                        break;
                    }
                    for (destination, point_index) in
                        [(first, index), (second, index.wrapping_add(1))]
                    {
                        let points = e.mem.u32(0x011c_57ac);
                        let point = e.call(0x006a_7af0, &args![points, point_index]).u32();
                        copy_point(e, destination, Ptr::new(point));
                    }
                    // The four extension cases are ordered: tail in either direction,
                    // then head in either direction. Every comparison reloads tolerance.
                    for (endpoint, candidate, end, opposite) in [
                        (first, second, 0x006c_ed90, 0x006c_ed70),
                        (second, first, 0x006c_ed90, 0x006c_ed70),
                        (first, second, 0x006c_ed70, 0x006c_ed90),
                        (second, first, 0x006c_ed70, 0x006c_ed90),
                    ] {
                        if !polygon_endpoint_equal(e, endpoint, chain, end) {
                            continue;
                        }
                        if polygon_endpoint_equal(e, candidate, chain, opposite) {
                            continue;
                        }
                        let tolerance = e.global::<f32>(0x0101_3ea4);
                        let neighbor = if end == 0x006c_ed90 {
                            let count = e.call(0x0044_ddc0, &args![chain]).u32();
                            e.call(0x006a_7af0, &args![chain, count.wrapping_sub(2)])
                                .u32()
                        } else {
                            e.call(0x006a_7af0, &args![chain, 1]).u32()
                        };
                        if e.call(0x0049_e2f0, &args![candidate, neighbor, tolerance])
                            .bool()
                        {
                            continue;
                        }
                        if end == 0x006c_ed90 {
                            e.call(0x0049_f210, &args![chain, candidate]);
                        } else {
                            ni_point3_array_insert(e, Ptr::new(chain), 0, candidate);
                        }
                        let points = e.mem.u32(0x011c_57ac);
                        fn_0049f3c0(e, Ptr::new(points), index as u32, 2, 1);
                        extended = true;
                        break;
                    }
                    if extended {
                        break;
                    }
                    let forward = polygon_endpoint_equal(e, first, chain, 0x006c_ed90)
                        && polygon_endpoint_equal(e, second, chain, 0x006c_ed70);
                    if forward
                        || (polygon_endpoint_equal(e, first, chain, 0x006c_ed70)
                            && polygon_endpoint_equal(e, second, chain, 0x006c_ed90))
                    {
                        closed = true;
                    }
                    index = index.wrapping_add(2);
                }
            }
            auto_water_cell_offset(e, cell, offset);
            if !closed {
                if chain != 0 {
                    e.vcall(chain, 0, &args![1]);
                }
                continue;
            }
            let allocation = e.call(0x0040_1000, &args![0x40]).u32();
            let polygon = if allocation == 0 {
                0
            } else {
                e.call(0x00c4_cba0, &args![allocation, chain]).u32()
            };
            if e.call(0x0059_bb30, &args![polygon]).u32() as i32 <= 2 {
                if polygon != 0 {
                    e.call(0x0049_e3e0, &args![polygon, 1]);
                }
            } else {
                e.call(0x00c4_cf00, &args![polygon]);
                e.call(0x00c4_cf80, &args![polygon]);
                if e.mem.u32(0x011c_57b0) == 0 {
                    let allocation = e.call(0x0040_1000, &args![12]).u32();
                    let list = if allocation == 0 {
                        0
                    } else {
                        e.call(0x0048_f200, &args![allocation]).u32()
                    };
                    e.mem.set_u32(0x011c_57b0, list);
                }
                e.mem.set_u32(polygon_slot.addr(), polygon);
                let list = e.mem.u32(0x011c_57b0);
                e.call(0x0076_b660, &args![list, polygon_slot]);
            }
        }
        if e.mem.u32(0x011c_57b0) != 0 {
            let list = e.mem.u32(0x011c_57b0);
            let mut outer = e.call(0x0055_9450, &args![list]).u32();
            while outer != 0 {
                let list = e.mem.u32(0x011c_57b0);
                let next_outer = e.call(0x007b_52d0, &args![list, outer]).u32();
                let list = e.mem.u32(0x011c_57b0);
                let payload = e.call(0x0063_17a0, &args![list, outer]).u32();
                let first_polygon = e.mem.u32(payload);
                let list = e.mem.u32(0x011c_57b0);
                let mut inner = e.call(0x0055_9450, &args![list]).u32();
                while inner != 0 {
                    let list = e.mem.u32(0x011c_57b0);
                    let next_inner = e.call(0x007b_52d0, &args![list, inner]).u32();
                    let list = e.mem.u32(0x011c_57b0);
                    let payload = e.call(0x0063_17a0, &args![list, inner]).u32();
                    let second_polygon = e.mem.u32(payload);
                    if first_polygon != second_polygon
                        && fn_0049eed0(e, Ptr::new(first_polygon), Ptr::new(second_polygon))
                    {
                        e.mem.set_u32(cursor.addr(), outer);
                        let list = e.mem.u32(0x011c_57b0);
                        fn_0049f590(e, Ptr::new(list), cursor);
                        break;
                    }
                    inner = next_inner;
                }
                outer = next_outer;
            }
        }
        e.call(0x0040_4ee0, &args![timer]);
    });
}

fn copy_point(e: &mut Engine, destination: Ptr, source: Ptr) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(source.addr().wrapping_add(offset));
        e.mem.set_u32(destination.addr().wrapping_add(offset), word);
    }
}

fn polygon_endpoint_equal(e: &mut Engine, first: Ptr, chain: u32, getter: u32) -> bool {
    // PC pushes the tolerance before calling the endpoint getter.
    let tolerance = e.global::<f32>(0x0101_3ea4);
    let second = e.call(getter, &args![chain]).u32();
    e.call(0x0049_e2f0, &args![first, second, tolerance]).bool()
}

// The PC callers leave water height and Y on the stack across GetLand calls;
// GetWorldOffsetForBlock takes only (land, output, 0), then 00416870 consumes XYZ.
fn auto_water_cell_offset(e: &mut Engine, cell: Ptr, output: Ptr) {
    let height = e.call(0x0054_71e0, &args![cell]).f32();
    e.with_stack(24, |e, points| {
        let land = e.call(0x0054_6fb0, &args![cell]).u32();
        let first = e.call(0x0053_5b30, &args![land, points, 0]).u32();
        let y = e.mem.f32(first + 4);
        let land = e.call(0x0054_6fb0, &args![cell]).u32();
        let second = e
            .call(0x0053_5b30, &args![land, points.addr() + 12, 0])
            .u32();
        let x = e.mem.f32(second);
        e.call(0x0041_6870, &args![output, x, y, height]);
    });
}

// Translated from 0049c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater::GatherLineSegments` (Xbox PDB): intersects each terrain
/// triangle with water, collects corner and boundary points, then sorts and
/// pairs the four boundary arrays into the global line-segment array.
// Translation incomplete: 0049c930 - compiler SEH registration and exception unwind cleanup are omitted.
pub fn bg_auto_water_gather_line_segments(e: &mut Engine, cell: Ptr) {
    let land = e.call(0x0054_6fb0, &args![cell]).u32();
    let height = e.call(0x0054_71e0, &args![cell]).f32();
    if e.mem.u32(0x011c_57ac) == 0 {
        let allocation = e.call(0x0040_1000, &args![0x10]).u32();
        let array = if allocation == 0 {
            0
        } else {
            e.call(0x0049_f1c0, &args![allocation]).u32()
        };
        e.mem.set_u32(0x011c_57ac, array);
    } else {
        let array = e.mem.u32(0x011c_57ac);
        e.call(0x0084_54f0, &args![array, 1]);
    }
    if land == 0 {
        return;
    }
    let loaded = e.call(0x0045_cb90, &args![land]).bool();
    if !loaded {
        e.call(0x0053_5b90, &args![land, 0]);
    }
    e.with_stack(0xa0, |e, scratch| {
        let boundaries = scratch;
        let temporary: Ptr = Ptr::new(scratch.addr() + 16);
        let vertices: [Ptr; 3] = [
            Ptr::new(scratch.addr() + 32),
            Ptr::new(scratch.addr() + 44),
            Ptr::new(scratch.addr() + 56),
        ];
        let water_point: Ptr = Ptr::new(scratch.addr() + 68);
        let normal: Ptr = Ptr::new(scratch.addr() + 80);
        let plane: Ptr = Ptr::new(scratch.addr() + 92);
        let intersection: Ptr = Ptr::new(scratch.addr() + 108);
        fn_0049f640(e, boundaries);
        for side in 0..4 {
            e.call(0x0049_f1c0, &args![temporary]);
            fn_0049f690(e, boundaries, side, temporary.addr());
            e.call(0x0049_f1f0, &args![temporary]);
        }
        let mut adjust_height = false;
        let mut land_height = 0.0;
        if e.call(0x0054_ddd0, &args![cell]).u32() != 0 {
            let world = e.call(0x0054_ddd0, &args![cell]).u32();
            if e.call(0x0058_6300, &args![world]).bool() {
                land_height = fn_0049db00(e, Ptr::new(land));
                adjust_height = true;
            }
        }
        let vertex_blocks = fn_0049db30(e, Ptr::new(land)).addr();
        for block in 0..4 {
            for triangle in 0..0x3fb {
                for (vertex, index_offset) in vertices.into_iter().zip([0, 2, 4]) {
                    let vertex_index = e.mem.u16(0x0118_ae90 + triangle * 2 + index_offset) as u32;
                    let buffer = e.mem.u32(vertex_blocks.wrapping_add(block * 4));
                    copy_point(e, vertex, Ptr::new(buffer.wrapping_add(vertex_index * 12)));
                }
                if adjust_height {
                    for vertex in vertices {
                        let z = (e.mem.f32(vertex.addr() + 8) as f64 + land_height as f64) as f32;
                        e.mem.set_f32(vertex.addr() + 8, z);
                    }
                }
                if !e.call(0x0043_9090, &args![vertices[0], vertices[1]]).bool()
                    || !e.call(0x0043_9090, &args![vertices[0], vertices[2]]).bool()
                    || !e.call(0x0043_9090, &args![vertices[1], vertices[2]]).bool()
                {
                    continue;
                }
                e.call(0x0041_6870, &args![water_point, 0.0f32, 0.0f32, height]);
                e.call(0x0041_6870, &args![normal, 0.0f32, 0.0f32, 1.0f32]);
                e.call(0x00a6_9990, &args![plane, normal, water_point]);
                let mut edge_bits = [0; 3];
                for (vertex, bits) in vertices.into_iter().zip(edge_bits.iter_mut()) {
                    let x = e.mem.f32(vertex.addr());
                    let y = e.mem.f32(vertex.addr() + 4);
                    let z = e.mem.f32(vertex.addr() + 8);
                    *bits = bg_auto_water_which_cell_edge(e, cell, x, y, z);
                }
                e.call(0x0068_15c0, &args![intersection]);
                let mut count = 0u16;
                let mut endpoint_seen = false;
                let mut coplanar_seen = false;
                for (edge_index, (a, b)) in [(0, 1), (0, 2), (1, 2)].into_iter().enumerate() {
                    let result = e
                        .call(
                            0x006b_9a10,
                            &args![vertices[a], vertices[b], water_point, normal, intersection],
                        )
                        .u32() as i32;
                    if result == 1 {
                        let first_equal = e
                            .call(0x0043_90c0, &args![vertices[a], intersection])
                            .bool();
                        // The third edge's first-endpoint match rejects unconditionally;
                        // the second-endpoint test is conditional on previous matches.
                        let mut accepted = true;
                        if edge_index == 2 {
                            if first_equal
                                || (e
                                    .call(0x0043_90c0, &args![vertices[b], intersection])
                                    .bool()
                                    && (endpoint_seen || coplanar_seen))
                            {
                                accepted = false;
                            }
                        } else {
                            let endpoint_equal = first_equal
                                || e.call(0x0043_90c0, &args![vertices[b], intersection])
                                    .bool();
                            if endpoint_equal {
                                if edge_index == 0 {
                                    endpoint_seen = true;
                                } else if endpoint_seen || coplanar_seen {
                                    accepted = false;
                                } else {
                                    endpoint_seen = true;
                                }
                            }
                        }
                        if accepted {
                            let points = e.mem.u32(0x011c_57ac);
                            e.call(0x0049_f210, &args![points, intersection]);
                            count = count.wrapping_add(1);
                            for side in 0..4 {
                                let mask = 1 << side;
                                if edge_bits[a] & mask != 0 && edge_bits[b] & mask != 0 {
                                    let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                                    e.call(0x0049_f210, &args![array, intersection]);
                                    break;
                                }
                            }
                        }
                    } else if result == -1 {
                        coplanar_seen = true;
                        if edge_index != 0 {
                            for _ in 0..count {
                                let points = e.mem.u32(0x011c_57ac);
                                fn_0049f510(e, Ptr::new(points));
                            }
                        }
                        for vertex in [vertices[a], vertices[b]] {
                            let points = e.mem.u32(0x011c_57ac);
                            e.call(0x0049_f210, &args![points, vertex]);
                        }
                        count = if edge_index == 0 {
                            count.wrapping_add(2)
                        } else {
                            2
                        };
                    }
                }
                if count != 2 {
                    for _ in 0..count {
                        let points = e.mem.u32(0x011c_57ac);
                        fn_0049f510(e, Ptr::new(points));
                    }
                }
                for (vertex, bits) in vertices.into_iter().zip(edge_bits) {
                    let sides = match bits {
                        9 => [0, 3],
                        10 => [1, 3],
                        5 => [0, 2],
                        6 => [1, 2],
                        _ => continue,
                    };
                    if fn_0049da80(e, plane, vertex) != 2 {
                        continue;
                    }
                    e.mem.set_f32(vertex.addr() + 8, height);
                    for side in sides {
                        let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                        if !fn_0049f190(e, Ptr::new(array), vertex.addr()) {
                            let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                            e.call(0x0049_f210, &args![array, vertex]);
                        }
                    }
                }
            }
        }
        if !loaded {
            e.call(0x0053_6d80, &args![land]);
        }
        for side in 0..4 {
            let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
            fn_0049f560(
                e,
                Ptr::new(array),
                if side < 2 { 0x0049_c800 } else { 0x0049_c7b0 },
            );
            let mut index = 0i32;
            loop {
                let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                let size = e.call(0x0044_ddc0, &args![array]).u32() as i32;
                if index >= size {
                    break;
                }
                let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                let size = e.call(0x0044_ddc0, &args![array]).u32() as i32;
                if index.wrapping_add(1) < size {
                    for point_index in [index, index.wrapping_add(1)] {
                        let array = e.call(0x0055_8dd0, &args![boundaries, side]).u32();
                        let point = e.call(0x006a_1440, &args![array, point_index]).u32();
                        let points = e.mem.u32(0x011c_57ac);
                        e.call(0x0049_f210, &args![points, point]);
                    }
                }
                index = index.wrapping_add(2);
            }
        }
        fn_0049f670(e, boundaries);
    });
}

// Translated from 0049e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater::GeneratePlaceableWater` (Xbox PDB): creates references and
/// water/stencil properties for polygons with scene geometry, derives bounds
/// from mesh vertices, attaches multibounds and initializes Havok once.
// Translation incomplete: 0049e410 - compiler SEH registration and exception unwind cleanup are omitted.
pub fn bg_auto_water_generate_placeable_water(e: &mut Engine, cell: Ptr) {
    e.with_stack(0x80, |e, scratch| {
        let timer = scratch;
        let offset: Ptr = Ptr::new(scratch.addr() + 4);
        let cursor: Ptr = Ptr::new(scratch.addr() + 16);
        let names: [Ptr; 2] = [Ptr::new(scratch.addr() + 20), Ptr::new(scratch.addr() + 24)];
        let extent: Ptr = Ptr::new(scratch.addr() + 28);
        let center: Ptr = Ptr::new(scratch.addr() + 40);
        let update: Ptr = Ptr::new(scratch.addr() + 52);
        e.call(0x0040_4eb0, &args![timer, 0x1d, 1, 0x0101_e018, 0x2cd]);
        if e.mem.u32(0x011c_57b0) != 0 {
            auto_water_cell_offset(e, cell, offset);
            let mut havok_initialized = false;
            let list = e.mem.u32(0x011c_57b0);
            let mut node = e.call(0x0055_9450, &args![list]).u32();
            while node != 0 {
                let list = e.mem.u32(0x011c_57b0);
                let next = e.call(0x007b_52d0, &args![list, node]).u32();
                let list = e.mem.u32(0x011c_57b0);
                let payload = e.call(0x0063_17a0, &args![list, node]).u32();
                let polygon = e.mem.u32(payload);
                if e.call(0x0056_c7f0, &args![polygon]).u32() != 0
                    && e.call(0x004a_8a90, &args![polygon]).u32() != 0
                {
                    for ((getter, string_address), name) in
                        [(0x0056_c7f0, 0x0101_e068), (0x004a_8a90, 0x0101_e05c)]
                            .into_iter()
                            .zip(names)
                    {
                        let string = e.call(0x0043_8170, &args![name, string_address]).u32();
                        let object = e.call(getter, &args![polygon]).u32();
                        e.call(0x00a5_b950, &args![object, string]);
                        e.call(0x0043_81b0, &args![name]);
                    }
                    let allocation = e.call(0x0040_1000, &args![0x68]).u32();
                    let reference = if allocation == 0 {
                        0
                    } else {
                        e.call(0x0055_a2f0, &args![allocation]).u32()
                    };
                    let allocation = e.call(0x00aa_13e0, &args![0x150]).u32();
                    let water_property = if allocation == 0 {
                        0
                    } else {
                        e.call(0x00b6_abb0, &args![allocation]).u32()
                    };
                    let mesh = e.call(0x004a_8a90, &args![polygon]).u32();
                    e.call(0x0043_9410, &args![mesh, water_property]);
                    let object = e.call(0x0056_c7f0, &args![polygon]).u32();
                    e.call(0x00b5_7e30, &args![object, 0, 0]);
                    let allocation = e.call(0x00aa_13e0, &args![0x24]).u32();
                    let stencil = if allocation == 0 {
                        Ptr::new(0)
                    } else {
                        fn_0049ec80(e, Ptr::new(allocation))
                    };
                    fn_0049ee30(e, stencil, 3);
                    let mesh = e.call(0x004a_8a90, &args![polygon]).u32();
                    e.call(0x0043_9410, &args![mesh, stencil]);
                    let water_type = e.call(0x0054_7770, &args![cell]).u32();
                    let form = e.call(0x0058_0130, &args![water_type]).u32();
                    e.call(0x0057_5690, &args![reference, form]);
                    let allocation = e.call(0x00aa_13e0, &args![0xe4]).u32();
                    let fade = if allocation == 0 {
                        0
                    } else {
                        e.call(0x00b4_e150, &args![allocation]).u32()
                    };
                    fn_0049ee60(e, Ptr::new(fade), 1);
                    e.vcall(reference, 0x1cc, &args![fade, 1]);
                    e.call(0x0044_0460, &args![fade, offset]);
                    fn_0049eea0(e, Ptr::new(reference), offset);
                    let allocation = e.call(0x00aa_13e0, &args![0x24]).u32();
                    let bounds = if allocation == 0 {
                        0
                    } else {
                        e.call(0x00c3_7ec0, &args![allocation]).u32()
                    };
                    let mut minimum = [0.0f32; 3];
                    let mut maximum = [0.0f32; 3];
                    let mesh = e.call(0x004a_8a90, &args![polygon]).u32();
                    let buffer = fn_0049ec60(e, Ptr::new(mesh));
                    let mut index = 0i32;
                    loop {
                        let count = e.call(0x0045_6650, &args![mesh]).u32() & 0xffff;
                        if index >= count as i32 {
                            break;
                        }
                        let point = buffer.wrapping_add((index as u32).wrapping_mul(12));
                        let values = [e.mem.f32(point), e.mem.f32(point + 4), e.mem.f32(point + 8)];
                        if index > 0 {
                            // Six calls run in XYZ minimum, then XYZ maximum order.
                            for axis in 0..3 {
                                minimum[axis] = e
                                    .call(0x0040_ebd0, &args![minimum[axis], values[axis]])
                                    .f32();
                            }
                            for axis in 0..3 {
                                maximum[axis] = e
                                    .call(0x0040_4010, &args![maximum[axis], values[axis]])
                                    .f32();
                            }
                        } else {
                            minimum = values;
                            maximum = values;
                        }
                        index = index.wrapping_add(1);
                    }
                    let mut half_extents = [0.0f32; 3];
                    for axis in [2, 1, 0] {
                        let divisor = e.global::<f64>(0x0101_1590);
                        let half = ((maximum[axis] as f64 - minimum[axis] as f64) / divisor) as f32;
                        half_extents[axis] = e.call(0x0040_8840, &args![half]).f32();
                    }
                    e.call(
                        0x0041_6870,
                        &args![extent, half_extents[0], half_extents[1], half_extents[2]],
                    );
                    let center_z = (maximum[2] as f64 - e.mem.f32(extent.addr() + 8) as f64) as f32;
                    let center_y = (maximum[1] as f64 - e.mem.f32(extent.addr() + 4) as f64) as f32;
                    let center_x = (maximum[0] as f64 - e.mem.f32(extent.addr()) as f64) as f32;
                    e.call(0x0041_6870, &args![center, center_x, center_y, center_z]);
                    e.call(0x0063_c8a0, &args![center, offset]);
                    e.vcall(bounds, 0xb8, &args![center]);
                    e.call(0x0043_9680, &args![bounds, extent]);
                    let allocation = e.call(0x00aa_13e0, &args![0x10]).u32();
                    let multibound = if allocation == 0 {
                        0
                    } else {
                        e.call(0x00c3_5e00, &args![allocation]).u32()
                    };
                    e.call(0x004a_ddc0, &args![multibound, bounds]);
                    let allocation = e.call(0x00aa_13e0, &args![0xb4]).u32();
                    let bound_node = if allocation == 0 {
                        0
                    } else {
                        e.call(0x00c4_6970, &args![allocation]).u32()
                    };
                    e.call(0x0043_9920, &args![bound_node, multibound]);
                    e.vcall(fade, 0xdc, &args![bound_node, 1]);
                    let object = e.call(0x0056_c7f0, &args![polygon]).u32();
                    e.vcall(bound_node, 0xdc, &args![object, 1]);
                    e.call(0x0043_d410, &args![update, 0.0f32, 0, 0]);
                    e.call(0x00a5_9c60, &args![fade, update]);
                    e.call(0x0054_8230, &args![cell, reference, 0]);
                    e.call(0x0048_4490, &args![reference]);
                    let enabled = e.call(0x0040_8d60, &args![0x011c_c5a8]).u32();
                    if e.mem.u8(enabled) != 0 && !havok_initialized {
                        e.call(0x0056_c8f0, &args![reference]);
                        havok_initialized = true;
                    }
                    if polygon != 0 {
                        e.call(0x0049_e3e0, &args![polygon, 1]);
                    }
                    e.mem.set_u32(cursor.addr(), node);
                    let list = e.mem.u32(0x011c_57b0);
                    fn_0049f590(e, Ptr::new(list), cursor);
                }
                node = next;
            }
        }
        e.call(0x0040_4ee0, &args![timer]);
    });
}

// Translated from 0049ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::GetRTTI` (Xbox PDB): returns its RTTI descriptor address.
pub fn ni_stencil_property_get_rtti(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(0x011f_4a0c)
}

// Translated from 0049ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty` scalar deleting destructor (Xbox PDB): destroys the
/// property and frees its 0x24-byte allocation when bit zero is set.
pub fn ni_stencil_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_stencil_property_destroy(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this, 0x24]);
    }
    this
}

// Translated from 0049ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty` destructor body (Xbox PDB): installs its vtable and
/// calls the base property destructor.
pub fn ni_stencil_property_destroy(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_e07c);
    e.call(0x0043_92c0, &args![this]);
}

// Translated from 0049edb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the 16-bit stencil function and the fixed mask/reference values.
pub fn fn_0049edb0(e: &mut Engine, this: Ptr, value: u16) {
    e.call(0x0043_9360, &args![this, value as u32, 0xf000, 0x0c]);
}

// Translated from 0049ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the 16-bit stencil operation and its fixed value/mask.
pub fn fn_0049ee10(e: &mut Engine, this: Ptr, value: u16) {
    e.call(0x0043_9360, &args![this, value as u32, 0x70, 4]);
}

// Translated from 0049ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the 16-bit stencil test and its fixed value/mask.
pub fn fn_0049ee30(e: &mut Engine, this: Ptr, value: u16) {
    e.call(0x0043_9360, &args![this, value as u32, 0xc00, 10]);
}

// Translated from 0049ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the 8-bit property flag to the sibling setter.
pub fn fn_0049ee60(e: &mut Engine, this: Ptr, value: u8) {
    fn_0049ee80(e, this, value);
}

// Translated from 0049ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards an 8-bit property value with the fixed 0x8000 mask.
pub fn fn_0049ee80(e: &mut Engine, this: Ptr, value: u8) {
    e.call(0x0043_b370, &args![this, value as u32, 0x8000]);
}

// Translated from 0049ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a narrowed byte value and the fixed flag to the property setter.
pub fn fn_0049ed90(e: &mut Engine, this: Ptr, value: u8) {
    e.call(0x0063_3ba0, &args![this, value as u32, 1]);
}

// Translated from 0049ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a narrowed 16-bit value with the fixed mask and operation.
pub fn fn_0049ede0(e: &mut Engine, this: Ptr, value: u16) {
    e.call(0x0043_9360, &args![this, value as u32, 0x380, 7]);
}

// Translated from 0049eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three point components into the auto-water property at `this+0x30`.
pub fn fn_0049eea0(e: &mut Engine, this: Ptr, point: Ptr) {
    for offset in [0, 4, 8] {
        let value = e.mem.u32(point.addr().wrapping_add(offset));
        e.mem
            .set_u32(this.addr().wrapping_add(0x30 + offset), value);
    }
}

// Translated from 0049eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts each indexed point through 00452dc0 and returns on the first match.
pub fn fn_0049eed0(e: &mut Engine, collection: Ptr, target: Ptr) -> bool {
    let mut index = 0i32;
    loop {
        let count = e.call(0x0059_bb30, &args![collection]).u32() as i32;
        if index >= count {
            return false;
        }
        let array = e.call(0x0055_9450, &args![collection]).u32();
        let point = e.call(0x006a_7af0, &args![array, index as u32]).u32();
        let equal = e.with_stack(0x18, |e, scratch| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(point.wrapping_add(offset));
                e.mem.set_u32(scratch.addr().wrapping_add(offset), word);
            }
            let pair: Ptr = Ptr::new(scratch.addr().wrapping_add(0xc));
            // 0049ef1a-0049ef24: y and x go out through FLD/FSTP float (a
            // signalling NaN is quieted).
            let x = x87_quiet_bits(e.mem.u32(scratch.addr()));
            let y = x87_quiet_bits(e.mem.u32(scratch.addr().wrapping_add(4)));
            e.call(0x0045_2dc0, &args![pair, x, y]);
            let x_word = e.mem.u32(pair.addr());
            let y_word = e.mem.u32(pair.addr().wrapping_add(4));
            e.call(0x00c4_e0a0, &args![target, x_word, y_word]).bool()
        });
        if equal {
            return true;
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0049ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the form associated with the save buffer embedded at `this+0xb8`.
pub fn fn_0049ec60(e: &mut Engine, this: Ptr) -> u32 {
    let buffer = e
        .call(0x0055_9450, &args![this.addr().wrapping_add(0xb8)])
        .u32();
    e.call(0x007a_f430, &args![buffer]).u32()
}

// Translated from 0049ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the 0x24-byte stencil property and applies its default settings.
// Translation incomplete: 0049ec80 - C++ SEH registration and unwind cleanup are omitted.
pub fn fn_0049ec80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0043_9260, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_e07c);
    e.mem.set_u16(this.addr().wrapping_add(0x18), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x1c), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x20), u32::MAX);
    fn_0049ed90(e, this, 0);
    fn_0049edb0(e, this, 4);
    e.call(0x0053_3fb0, &args![this, 0]);
    fn_0049ee10(e, this, 0);
    fn_0049ede0(e, this, 3);
    fn_0049ee30(e, this, 0);
    this
}

// Translated from 0049f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater` point-array membership check using the unit's exact
/// three-component point comparator and the search function at 0049f7d0.
pub fn fn_0049f190(e: &mut Engine, array: Ptr, comparison_value: u32) -> bool {
    let index = fn_0049f7d0(e, array, comparison_value, 0, 0x0049_f830);
    index != u32::MAX
}

// Translated from 0049f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPoint3, 1024>::Insert` (Xbox PDB): inserts a point,
/// growing through the array's allocator when its heap requires relocation.
pub fn ni_point3_array_insert(e: &mut Engine, this: Ptr, index: u32, point: Ptr) {
    let size = e.mem.u32(this.addr().wrapping_add(8));
    if index == size {
        e.call(0x0049_f210, &args![this, point]);
        return;
    }

    if e.call(0x0043_8b90, &args![this]).bool() {
        let growth = e.call(0x009a_3910, &args![this]).u32();
        let vtable = e.mem.u32(this.addr());
        let allocator = e.mem.u32(vtable + 4);
        let new_buffer = e.call(allocator, &args![this, growth]).u32();
        let old_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(0x006d_c6e0, &args![this, new_buffer, old_buffer, index]);
        e.call(
            0x0049_f8f0,
            &args![this, new_buffer.wrapping_add(index.wrapping_mul(12)), 1],
        );
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        let trailing_count = current_size.wrapping_sub(index);
        let old_tail = current_buffer.wrapping_add(index.wrapping_mul(12));
        let new_tail = new_buffer.wrapping_add(index.wrapping_mul(12).wrapping_add(12));
        e.call(
            0x006d_c6e0,
            &args![this, new_tail, old_tail, trailing_count],
        );
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(this.addr().wrapping_add(4), new_buffer);
        e.mem.set_u32(this.addr().wrapping_add(12), growth);
    } else {
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        let tail = current_buffer.wrapping_add(index.wrapping_mul(12));
        let trailing_count = current_size.wrapping_sub(index);
        e.call(
            0x006d_c6e0,
            &args![this, tail.wrapping_add(12), tail, trailing_count],
        );
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(
            0x0049_f8f0,
            &args![this, current_buffer.wrapping_add(index.wrapping_mul(12)), 1],
        );
    }

    let current_size = e.mem.u32(this.addr().wrapping_add(8));
    e.mem
        .set_u32(this.addr().wrapping_add(8), current_size.wrapping_add(1));
    let destination = e
        .mem
        .u32(this.addr().wrapping_add(4))
        .wrapping_add(index.wrapping_mul(12));
    for offset in [0, 4, 8] {
        let word = e.mem.u32(point.addr().wrapping_add(offset));
        e.mem.set_u32(destination.wrapping_add(offset), word);
    }
}

// Translated from 0049f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` points at `index`, optionally relocating the array first.
pub fn fn_0049f3c0(e: &mut Engine, this: Ptr, index: u32, count: u32, allow_relocation: u8) {
    if count >= e.mem.u32(this.addr().wrapping_add(8)) {
        e.call(0x0084_54f0, &args![this, allow_relocation]);
        return;
    }

    let capacity = allow_relocation != 0 && e.call(0x006f_3170, &args![this]).bool();
    if capacity {
        let growth = e.call(0x0086_9600, &args![this]).u32();
        let vtable = e.mem.u32(this.addr());
        let allocator = e.mem.u32(vtable.wrapping_add(4));
        let new_buffer = e.call(allocator, &args![this, growth]).u32();
        let old_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(0x006d_c6e0, &args![this, new_buffer, old_buffer, index]);

        let buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(
            0x0072_ba80,
            &args![this, buffer.wrapping_add(index.wrapping_mul(12)), count],
        );
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        let destination = new_buffer.wrapping_add(index.wrapping_mul(12));
        let source = current_buffer
            .wrapping_add(index.wrapping_mul(12))
            .wrapping_add(count.wrapping_mul(12));
        e.call(
            0x006d_c6e0,
            &args![this, destination, source, current_size.wrapping_sub(count)],
        );
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(this.addr().wrapping_add(4), new_buffer);
        e.mem.set_u32(this.addr().wrapping_add(12), growth);
    } else {
        let buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(
            0x0072_ba80,
            &args![this, buffer.wrapping_add(index.wrapping_mul(12)), count],
        );
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        let destination = current_buffer.wrapping_add(index.wrapping_mul(12));
        let source = destination.wrapping_add(count.wrapping_mul(12));
        let trailing_count = current_size.wrapping_sub(index).wrapping_sub(count);
        e.call(
            0x006d_c6e0,
            &args![this, destination, source, trailing_count],
        );
    }

    let current_size = e.mem.u32(this.addr().wrapping_add(8));
    e.mem.set_u32(
        this.addr().wrapping_add(8),
        current_size.wrapping_sub(count),
    );
}

// Translated from 0049f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unlinks the node referenced by `cursor`, using the list's head/tail helpers.
pub fn fn_0049f590(e: &mut Engine, this: Ptr, cursor: Ptr) -> u32 {
    let node = e.mem.u32(cursor.addr());
    if node == e.mem.u32(this.addr()) {
        let next = e.mem.u32(node);
        e.mem.set_u32(cursor.addr(), next);
        return e.call(0x007b_5390, &args![this]).u32();
    }
    if node == e.mem.u32(this.addr().wrapping_add(4)) {
        e.mem.set_u32(cursor.addr(), 0);
        return e.call(0x004e_df00, &args![this]).u32();
    }

    let previous = e.mem.u32(node.wrapping_add(4));
    let next = e.mem.u32(node);
    e.mem.set_u32(cursor.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(4), previous);
    }
    let payload = e.mem.u32(node.wrapping_add(8));
    e.call(0x004a_49e0, &args![this, node]);
    let size = e.mem.u32(this.addr().wrapping_add(8));
    e.mem
        .set_u32(this.addr().wrapping_add(8), size.wrapping_sub(1));
    payload
}

// Translated from 0049f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts one 16-byte nested-array element, using the class allocator when
/// the array's heap requires relocation.
pub fn fn_0049f690(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let size = e.mem.u32(this.addr().wrapping_add(8));
    if index == size {
        fn_0049faa0(e, this, value);
        return;
    }

    if e.call(0x0043_8b90, &args![this]).bool() {
        let growth = e.call(0x009a_3910, &args![this]).u32();
        let vtable = e.mem.u32(this.addr());
        let allocator = e.mem.u32(vtable.wrapping_add(4));
        let new_buffer = e.call(allocator, &args![this, growth]).u32();
        let old_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(0x006c_8bd0, &args![this, new_buffer, old_buffer, index]);
        e.call(
            ARRAY_SET_SIZE,
            &args![this, new_buffer.wrapping_add(index.wrapping_mul(16)), 1],
        );
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(
            0x006c_8bd0,
            &args![
                this,
                new_buffer
                    .wrapping_add(index.wrapping_mul(16))
                    .wrapping_add(16),
                current_buffer.wrapping_add(index.wrapping_mul(16)),
                current_size.wrapping_sub(index),
            ],
        );
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(this.addr().wrapping_add(4), new_buffer);
        e.mem.set_u32(this.addr().wrapping_add(12), growth);
    } else {
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        let tail = current_buffer.wrapping_add(index.wrapping_mul(16));
        e.call(
            0x006c_8bd0,
            &args![
                this,
                tail.wrapping_add(16),
                tail,
                current_size.wrapping_sub(index),
            ],
        );
        let current_buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(
            ARRAY_SET_SIZE,
            &args![this, current_buffer.wrapping_add(index.wrapping_mul(16)), 1],
        );
    }
    let current_size = e.mem.u32(this.addr().wrapping_add(8));
    e.mem
        .set_u32(this.addr().wrapping_add(8), current_size.wrapping_add(1));
    let destination = e
        .mem
        .u32(this.addr().wrapping_add(4))
        .wrapping_add(index.wrapping_mul(16));
    e.call(0x0049_f8b0, &args![destination, value]);
}

// Translated from 0049f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes and destroys the last `NiPoint3` when the array is nonempty.
pub fn fn_0049f510(e: &mut Engine, this: Ptr) {
    let size = e.mem.u32(this.addr().wrapping_add(8));
    if size != 0 {
        let item = e
            .mem
            .u32(this.addr().wrapping_add(4))
            .wrapping_add(size.wrapping_mul(12).wrapping_sub(12));
        e.call(0x0072_ba80, &args![this, item, 1]);
        let current_size = e.mem.u32(this.addr().wrapping_add(8));
        e.mem
            .set_u32(this.addr().wrapping_add(8), current_size.wrapping_sub(1));
    }
}

// Translated from 0049f560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the array's 12-byte points with the supplied cdecl comparator.
pub fn fn_0049f560(e: &mut Engine, this: Ptr, compare: u32) {
    let buffer = e.mem.u32(this.addr().wrapping_add(4));
    let size = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00ec_6f20, &args![buffer, size, 12, compare]);
}

// Translated from 0049f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the nested point array, installing its vtable and empty state.
pub fn fn_0049f640(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_e128);
    fn_0049fbf0(e, this, 0, 0);
    this
}

// Translated from 0049f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the nested point array while retaining its allocation.
pub fn fn_0049f670(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_e128);
    e.call(0x0049_faf0, &args![this, 1]);
}

// Translated from 0049f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the first 12-byte point accepted by the callback, starting at index.
pub fn fn_0049f7d0(
    e: &mut Engine,
    this: Ptr,
    comparison_value: u32,
    start: u32,
    compare: u32,
) -> u32 {
    let mut index = start;
    loop {
        if e.mem.u32(this.addr().wrapping_add(8)) <= index {
            return u32::MAX;
        }
        let buffer = e.mem.u32(this.addr().wrapping_add(4));
        if buffer != 0
            && e.call(
                compare,
                &args![
                    buffer.wrapping_add(index.wrapping_mul(12)),
                    comparison_value
                ],
            )
            .bool()
        {
            return index;
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0049f830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the engine's exact three-component point comparator with both pointers.
pub fn fn_0049f830(e: &mut Engine, first: u32, second: u32) -> bool {
    e.call(0x0043_90c0, &args![first, second]).bool()
}

// Translated from 0049ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAutoWater::WhichCellEdge` (Xbox PDB): returns edge bits for a point
/// equal to a cell corner or boundary. Its cdecl ABI is `(cell, x, y, z)`;
/// the decompiler omitted the fourth stack word, which callers clean up.
pub fn bg_auto_water_which_cell_edge(e: &mut Engine, cell: Ptr, x: f32, y: f32, z: f32) -> u32 {
    e.with_stack(0x50, |e, scratch| {
        let offset: Ptr = Ptr::new(scratch.addr());
        let point: Ptr = Ptr::new(scratch.addr().wrapping_add(0x10));
        let corners: [Ptr; 4] = [
            Ptr::new(scratch.addr().wrapping_add(0x20)),
            Ptr::new(scratch.addr().wrapping_add(0x2c)),
            Ptr::new(scratch.addr().wrapping_add(0x38)),
            Ptr::new(scratch.addr().wrapping_add(0x44)),
        ];

        let land = e.call(0x0054_6fb0, &args![cell]).u32();
        e.call(0x0053_5b30, &args![land, offset, 0]);
        for (address, value) in [
            (point.addr(), x),
            (point.addr().wrapping_add(4), y),
            (point.addr().wrapping_add(8), z),
        ] {
            e.mem.set_f32(address, value);
        }
        e.call(0x0063_c8a0, &args![point, offset]);

        for (corner_index, corner) in corners.into_iter().enumerate() {
            let y_index = e.call(0x0054_4c60, &args![cell]).u32();
            let y_units = y_index
                .wrapping_mul(0x1000)
                .wrapping_add(if corner_index >= 2 { 0x1000 } else { 0 });
            let y_coordinate = y_units as i32 as f32;
            let x_index = e.call(0x0054_4c30, &args![cell]).u32();
            let x_units = x_index
                .wrapping_mul(0x1000)
                .wrapping_add(if corner_index & 1 != 0 { 0x1000 } else { 0 });
            let x_coordinate = x_units as i32 as f32;
            e.call(0x0045_2dc0, &args![corner, x_coordinate, y_coordinate]);
        }

        let corner_matches = |e: &Engine, corner: Ptr| {
            e.mem.f32(corner.addr()) == e.mem.f32(point.addr())
                && e.mem.f32(corner.addr().wrapping_add(4))
                    == e.mem.f32(point.addr().wrapping_add(4))
        };
        if corner_matches(e, corners[0]) {
            9
        } else if corner_matches(e, corners[1]) {
            10
        } else if corner_matches(e, corners[2]) {
            5
        } else if corner_matches(e, corners[3]) {
            6
        } else if e.mem.f32(corners[0].addr()) == e.mem.f32(point.addr()) {
            1
        } else if e.mem.f32(corners[1].addr()) == e.mem.f32(point.addr()) {
            2
        } else if e.mem.f32(corners[0].addr().wrapping_add(4))
            == e.mem.f32(point.addr().wrapping_add(4))
        {
            8
        } else if e.mem.f32(corners[3].addr().wrapping_add(4))
            == e.mem.f32(point.addr().wrapping_add(4))
        {
            4
        } else {
            0
        }
    })
}

// Translated from 0049f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPoint3, 1024>` scalar deleting destructor (Xbox PDB).
pub fn ni_point3_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0049_f1f0, &args![this]);
    if flags & 1 != 0 {
        e.call(ARRAY_FREE, &args![this]);
    }
    this
}

// Translated from 0049f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSSimpleArray<NiPoint3>, 1024>` scalar deleting destructor
/// (Xbox PDB).
pub fn nested_point3_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0049f670(e, this);
    if flags & 1 != 0 {
        e.call(ARRAY_FREE, &args![this]);
    }
    this
}

// Translated from 0049faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a 16-byte element through the current array allocator and constructs it.
pub fn fn_0049faa0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let index = e.call(0x006f_31f0, &args![this]).u32();
    let storage = e
        .mem
        .u32(this.addr().wrapping_add(4))
        .wrapping_add(index.wrapping_mul(16));
    e.call(ARRAY_SET_SIZE, &args![this, storage, 1]);
    let element = e
        .mem
        .u32(this.addr().wrapping_add(4))
        .wrapping_add(index.wrapping_mul(16));
    e.call(0x0049_f8b0, &args![element, value]);
    index
}

// Translated from 0049fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the 16-byte array header, allocating at least `size` slots and
/// constructing each requested element.
pub fn fn_0049fbf0(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    e.mem.set_u32(this.addr().wrapping_add(8), 0);
    e.mem.set_u32(this.addr().wrapping_add(12), 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let vtable = e.mem.u32(this.addr());
        let allocator = e.mem.u32(vtable + 4);
        let buffer = e.call(allocator, &args![this, capacity]).u32();
        e.mem.set_u32(this.addr().wrapping_add(4), buffer);
        e.mem.set_u32(this.addr().wrapping_add(12), capacity);
    }
    if size != 0 {
        let buffer = e.mem.u32(this.addr().wrapping_add(4));
        e.call(ARRAY_SET_SIZE, &args![this, buffer, size]);
        e.mem.set_u32(this.addr().wrapping_add(8), size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_db30, fn_0049db30(Ptr) -> Ptr),
        entry!(0x0049_da80, fn_0049da80(Ptr, Ptr) -> u32),
        entry!(0x0049_db00, fn_0049db00(Ptr) -> f32),
        entry!(
            0x0049_c860,
            bg_auto_water_generate_placeable_water_for_cell(Ptr)
        ),
        entry!(0x0049_c930, bg_auto_water_gather_line_segments(Ptr)),
        entry!(0x0049_db60, bg_auto_water_generate_polygons(Ptr)),
        entry!(0x0049_e410, bg_auto_water_generate_placeable_water(Ptr)),
        entry!(0x0049_ed30, ni_stencil_property_get_rtti(Ptr) -> Ptr),
        entry!(0x0049_ed40, ni_stencil_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0049_ed70, ni_stencil_property_destroy(Ptr)),
        entry!(0x0049_edb0, fn_0049edb0(Ptr, u16)),
        entry!(0x0049_ee10, fn_0049ee10(Ptr, u16)),
        entry!(0x0049_ee30, fn_0049ee30(Ptr, u16)),
        entry!(0x0049_ee60, fn_0049ee60(Ptr, u8)),
        entry!(0x0049_ee80, fn_0049ee80(Ptr, u8)),
        entry!(0x0049_ed90, fn_0049ed90(Ptr, u8)),
        entry!(0x0049_ede0, fn_0049ede0(Ptr, u16)),
        entry!(0x0049_eea0, fn_0049eea0(Ptr, Ptr)),
        entry!(0x0049_eed0, fn_0049eed0(Ptr, Ptr) -> bool),
        entry!(0x0049_ec60, fn_0049ec60(Ptr) -> u32),
        entry!(0x0049_ec80, fn_0049ec80(Ptr) -> Ptr),
        entry!(0x0049_f190, fn_0049f190(Ptr, u32) -> bool),
        entry!(0x0049_f270, ni_point3_array_insert(Ptr, u32, Ptr)),
        entry!(0x0049_f3c0, fn_0049f3c0(Ptr, u32, u32, u8)),
        entry!(0x0049_f590, fn_0049f590(Ptr, Ptr) -> u32),
        entry!(0x0049_f690, fn_0049f690(Ptr, u32, u32)),
        entry!(0x0049_f510, fn_0049f510(Ptr)),
        entry!(0x0049_f560, fn_0049f560(Ptr, u32)),
        entry!(0x0049_ef60, bg_auto_water_which_cell_edge(Ptr, f32, f32, f32) -> u32),
        entry!(0x0049_f640, fn_0049f640(Ptr) -> Ptr),
        entry!(0x0049_f670, fn_0049f670(Ptr)),
        entry!(0x0049_f7d0, fn_0049f7d0(Ptr, u32, u32, u32) -> u32),
        entry!(0x0049_f830, fn_0049f830(u32, u32) -> bool),
        entry!(0x0049_f850, ni_point3_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0049_f880, nested_point3_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0049_faa0, fn_0049faa0(Ptr, u32) -> u32),
        entry!(0x0049_fbf0, fn_0049fbf0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn double(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
        e.call_log = Some(vec![]);
    }

    fn point_helpers(e: &mut Engine) {
        e.register(0x0041_6870, |e, a| {
            assert_eq!(a.len(), 4);
            for offset in 0..3 {
                e.mem.set_u32(a[0] + offset * 4, a[offset as usize + 1]);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x0049_f210, |e, a| {
            assert_eq!(a.len(), 2);
            let size = e.mem.u32(a[0] + 8);
            let destination = e.mem.u32(a[0] + 4) + size * 12;
            copy_point(e, Ptr::new(destination), Ptr::new(a[1]));
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        for address in [0x006a_7af0, 0x006a_1440] {
            e.register(address, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4) + a[1] * 12,
                ..Ret::default()
            });
        }
        e.register(0x0044_ddc0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(0x0084_54f0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(0x0055_8dd0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + a[1] * 16,
            ..Ret::default()
        });
        e.register(0x006d_c6e0, |e, a| {
            let words: Vec<_> = (0..a[3] * 3)
                .map(|index| e.mem.u32(a[2] + index * 4))
                .collect();
            for (index, value) in words.into_iter().enumerate() {
                e.mem.set_u32(a[1] + index as u32 * 4, value);
            }
            Ret::default()
        });
        for address in [
            0x006f_3170,
            0x0072_ba80,
            0x006a_8500,
            0x0043_8b90,
            0x0049_f8f0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(0x007b_52d0, |e, a| {
            assert_eq!(a.len(), 2);
            Ret {
                eax: e.mem.u32(a[1]),
                ..Ret::default()
            }
        });
        e.register(0x0063_17a0, |_, a| {
            assert_eq!(a.len(), 2);
            Ret {
                eax: a[1] + 8,
                ..Ret::default()
            }
        });
    }

    fn cell_offset_helpers(e: &mut Engine) {
        e.register(0x0054_71e0, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                st0: 7.0,
                ..Ret::default()
            }
        });
        e.register(0x0054_6fb0, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 0x1800,
                ..Ret::default()
            }
        });
        e.register(0x0053_5b30, |e, a| {
            assert_eq!(a.len(), 3);
            assert_eq!(a[0], 0x1800);
            assert_eq!(a[2], 0);
            for (offset, value) in [(0, 10.0), (4, 20.0), (8, 0.0)] {
                e.mem.set_f32(a[1] + offset, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn polygon_comparison_captures_tolerance_before_getter_and_refreshes_next_call() {
        let mut e = Engine::new();
        e.map(0x0101_3ea4, 4);
        e.mem.set_f32(0x0101_3ea4, 0.25);
        e.register(0x006c_ed90, |e, a| {
            assert_eq!(a, [0x2000]);
            let previous = e.mem.f32(0x0101_3ea4);
            e.mem.set_f32(0x0101_3ea4, previous * 2.0);
            Ret {
                eax: 0x3000,
                ..Ret::default()
            }
        });
        let mut comparisons = 0;
        e.register_double(0x0049_e2f0, move |_, a| {
            assert_eq!(a[0..2], [0x1000, 0x3000]);
            assert_eq!(a[2], [0.25f32, 0.5][comparisons].to_bits());
            comparisons += 1;
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert!(polygon_endpoint_equal(
            &mut e,
            Ptr::new(0x1000),
            0x2000,
            0x006c_ed90
        ));
        assert!(polygon_endpoint_equal(
            &mut e,
            Ptr::new(0x1000),
            0x2000,
            0x006c_ed90
        ));
    }

    #[test]
    fn polygon_cleanup_removes_first_overlapping_polygon_and_retains_saved_next() {
        let mut e = Engine::new();
        for base in [0x2000, 0x3000, 0x4000, 0x5000, 0x6000] {
            e.map(base, 0x1000);
        }
        e.map(0x011c_57ac, 8);
        e.mem.set_u32(0x011c_57ac, 0x2800); // Empty line array.
        e.mem.set_u32(0x011c_57b0, 0x2000);
        e.mem.set_u32(0x2000, 0x2100);
        e.mem.set_u32(0x2004, 0x2120);
        e.mem.set_u32(0x2008, 2);
        e.mem.set_u32(0x2100, 0x2120);
        e.mem.set_u32(0x2108, 0x3000);
        e.mem.set_u32(0x2120, 0);
        e.mem.set_u32(0x2124, 0x2100);
        e.mem.set_u32(0x2128, 0x4000);
        e.mem.set_u32(0x3000, 0x5000);
        e.mem.set_u32(0x3008, 1);
        e.mem.set_u32(0x5004, 0x6000);
        for (offset, value) in [(0, 3.0), (4, 4.0), (8, 5.0)] {
            e.mem.set_f32(0x6000 + offset, value);
        }
        point_helpers(&mut e);
        for address in [0x0040_4eb0, 0x0040_4ee0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0059_bb30, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(0x0045_2dc0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x00c4_e0a0, |_, a| {
            assert_eq!(a, [0x4000, 3.0f32.to_bits(), 4.0f32.to_bits()]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(0x007b_5390, |e, a| {
            assert_eq!(a, [0x2000]);
            e.mem.set_u32(0x2000, 0x2120);
            e.mem.set_u32(0x2124, 0);
            e.mem.set_u32(0x2008, 1);
            Ret {
                eax: 0x3000,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        bg_auto_water_generate_polygons(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x2000), 0x2120);
        assert_eq!(e.mem.u32(0x2008), 1);
        let calls = e.call_log.unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x007b_5390)
                .count(),
            1
        );
        assert_eq!(calls.last().unwrap().0, 0x0040_4ee0);
    }

    #[test]
    fn polygon_generation_extends_all_four_directions_closes_and_publishes() {
        // Four segment orientations select each ordered extension branch.
        for segments in [
            [[1, 2], [2, 3], [3, 1]],
            [[1, 2], [3, 2], [3, 1]],
            [[1, 2], [1, 3], [3, 2]],
            [[1, 2], [3, 1], [3, 2]],
        ] {
            let mut e = Engine::new();
            for base in [0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000] {
                e.map(base, 0x1000);
            }
            e.map(0x011c_57ac, 8);
            e.map(0x0101_3ea4, 4);
            e.mem.set_u32(0x011c_57ac, 0x2000);
            e.mem.set_u32(0x2004, 0x5000);
            e.mem.set_u32(0x2008, 6);
            e.mem.set_f32(0x0101_3ea4, 0.01);
            for (index, value) in segments.into_iter().flatten().enumerate() {
                e.mem.set_f32(0x5000 + index as u32 * 12, value as f32);
            }
            point_helpers(&mut e);
            cell_offset_helpers(&mut e);
            for address in [
                0x0040_4eb0,
                0x0040_4ee0,
                0x00c4_cf00,
                0x00c4_cf80,
                0x0049_f1f0,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            let mut arrays = 0;
            e.register_double(0x0040_1000, move |_, a| {
                let allocation = match a[0] {
                    16 => {
                        arrays += 1;
                        0x3000 + (arrays - 1) * 0x100
                    }
                    0x40 => 0x7000,
                    12 => 0x6000,
                    _ => panic!("unexpected size"),
                };
                Ret {
                    eax: allocation,
                    ..Ret::default()
                }
            });
            e.register(0x0049_f1c0, |e, a| {
                e.mem.set_u32(a[0], 0x3800);
                e.mem.set_u32(a[0] + 4, 0x4000 + (a[0] - 0x3000));
                e.mem.set_u32(a[0] + 8, 0);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            e.mem.set_u32(0x3800, 0x3900);
            e.register(0x3900, |_, a| {
                assert_eq!(a[1], 1);
                Ret::default()
            });
            e.register(0x006c_ed70, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            });
            e.register(0x006c_ed90, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4) + (e.mem.u32(a[0] + 8) - 1) * 12,
                ..Ret::default()
            });
            e.register(0x0049_e2f0, |e, a| {
                assert_eq!(a.len(), 3);
                assert_eq!(a[2], 0.01f32.to_bits());
                Ret {
                    eax: u32::from(e.mem.f32(a[0]) == e.mem.f32(a[1])),
                    ..Ret::default()
                }
            });
            e.register(0x00c4_cba0, |e, a| {
                assert_eq!(a, [0x7000, 0x3000]);
                assert_eq!(e.mem.u32(0x3008), 3);
                e.mem.set_u32(0x7004, e.mem.u32(0x3004));
                e.mem.set_u32(0x7008, 3);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            e.register(0x0059_bb30, |_, a| {
                assert_eq!(a, [0x7000]);
                Ret {
                    eax: 3,
                    ..Ret::default()
                }
            });
            e.register(0x0048_f200, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
            e.register(0x0076_b660, |e, a| {
                assert_eq!(a[0], 0x6000);
                assert_eq!(e.mem.u32(a[1]), 0x7000);
                e.mem.set_u32(0x6000, 0x6100);
                e.mem.set_u32(0x6100, 0);
                e.mem.set_u32(0x6108, 0x7000);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            bg_auto_water_generate_polygons(&mut e, Ptr::new(0x1000));
            assert_eq!(e.mem.u32(0x2008), 0);
            assert_eq!(e.mem.u32(0x011c_57b0), 0x6000);
            assert_eq!(e.mem.u32(0x6108), 0x7000);
            let calls = e.call_log.unwrap();
            assert_eq!(calls.last().unwrap().0, 0x0040_4ee0);
            assert_eq!(
                calls
                    .iter()
                    .filter(|(address, _)| *address == 0x00c4_cf00)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn terrain_gathering_intersects_one_triangle_adds_corners_and_pairs_boundaries() {
        for (results, point_count, boundary_count, pops, first_z) in [
            ([1i32, 1, 0], 4, 2, 0, 7.0),
            ([-1, 0, 0], 2, 1, 0, -1.0),
            ([1, -1, 0], 4, 2, 1, -1.0),
            ([1, 1, -1], 4, 2, 2, 2.0),
            ([1, 0, 0], 2, 2, 1, 7.0),
        ] {
            let mut e = edge_engine();
            for base in [
                0x1000, 0x2000, 0x7000, 0x8000, 0x9000, 0xa000, 0xb000, 0xc000, 0xd000,
            ] {
                e.map(base, 0x1000);
            }
            e.map(0x0118_ae90, 0x800);
            e.map(0x011c_57ac, 8);
            e.map(0x0101_2060, 8);
            e.mem.set_u32(0x011c_57ac, 0x2000);
            e.mem.set_u32(0x2004, 0xd000);
            e.mem.set_u32(0x1828, 0x1a00);
            e.mem.set_u32(0x1a04, 0x1b00);
            e.mem.set_f32(0x1aa0, 1.0);
            for block in 0..4 {
                e.mem.set_u32(0x1b00 + block * 4, 0x7000);
            }
            for (index, values) in [[-10.0, -20.0, -2.0], [-10.0, 80.0, 1.0], [90.0, 80.0, 1.0]]
                .into_iter()
                .enumerate()
            {
                for (axis, value) in values.into_iter().enumerate() {
                    e.mem
                        .set_f32(0x7000 + index as u32 * 12 + axis as u32 * 4, value);
                }
                e.mem.set_u16(0x0118_ae90 + index as u32 * 2, index as u16);
            }
            point_helpers(&mut e);
            cell_offset_helpers(&mut e);
            for address in [
                0x0053_5b90,
                0x0053_6d80,
                0x0049_f1f0,
                0x0049_faf0,
                0x0068_15c0,
                0x00ec_6f20,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.register(0x0045_cb90, |_, _| Ret::default());
            e.register(0x0054_ddd0, |_, _| Ret {
                eax: 0x1900,
                ..Ret::default()
            });
            e.register(0x0058_6300, |_, a| {
                assert_eq!(a, [0x1900]);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            });
            e.register(0x0049_f1c0, |e, a| {
                for offset in [0, 4, 8, 12] {
                    e.mem.set_u32(a[0] + offset, 0);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            e.register(0x006f_31f0, |e, a| {
                let index = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 4, 0x8000);
                e.mem.set_u32(a[0] + 8, index + 1);
                Ret {
                    eax: index,
                    ..Ret::default()
                }
            });
            e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
            e.register(0x0049_f8b0, |e, a| {
                let side = (a[0] - 0x8000) / 16;
                e.mem.set_u32(a[0] + 4, 0x9000 + side * 0x1000);
                e.mem.set_u32(a[0] + 8, 0);
                e.mem.set_u32(a[0] + 12, 64);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            let mut comparisons = 0;
            e.register_double(0x0043_9090, move |_, a| {
                assert_eq!(a.len(), 2);
                comparisons += 1;
                Ret {
                    eax: u32::from(comparisons <= 3),
                    ..Ret::default()
                }
            });
            e.register(0x0043_90c0, |_, _| Ret::default());
            e.register(0x00a6_9990, |e, a| {
                assert_eq!(a.len(), 3);
                copy_point(e, Ptr::new(a[0]), Ptr::new(a[1]));
                e.mem.set_f32(a[0] + 12, e.mem.f32(a[2] + 8));
                Ret::default()
            });
            let mut intersections = 0;
            e.register_double(0x006b_9a10, move |e, a| {
                assert_eq!(a.len(), 5);
                intersections += 1;
                if intersections <= 2 {
                    assert_eq!(e.mem.f32(a[0] + 8), -1.0);
                }
                if results[intersections - 1] == 1 {
                    for (axis, value) in [0.0, intersections as f32 * 30.0, 7.0]
                        .into_iter()
                        .enumerate()
                    {
                        e.mem.set_f32(a[4] + axis as u32 * 4, value);
                    }
                }
                Ret {
                    eax: results[intersections - 1] as u32,
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            bg_auto_water_gather_line_segments(&mut e, Ptr::new(0x1000));
            assert_eq!(e.mem.u32(0x2008), point_count);
            assert_eq!(e.mem.u32(0x8008), boundary_count); // Shared left boundary plus submerged corner.
            assert_eq!(e.mem.u32(0x8038), 1); // Unpaired bottom corner is retained only here.
            assert_eq!(e.mem.f32(0xd008), first_z);
            let calls = e.call_log.unwrap();
            let sorts: Vec<_> = calls
                .iter()
                .filter(|(address, _)| *address == 0x00ec_6f20)
                .map(|(_, a)| a[3])
                .collect();
            assert_eq!(sorts, [0x0049_c800, 0x0049_c800, 0x0049_c7b0, 0x0049_c7b0]);
            assert_eq!(
                calls
                    .iter()
                    .filter(|(address, _)| *address == 0x0053_6d80)
                    .count(),
                1
            );
            assert_eq!(calls.last().unwrap().0, 0x0049_faf0);
            assert_eq!(
                calls
                    .iter()
                    .filter(|(address, _)| *address == 0x0072_ba80)
                    .count(),
                pops
            );
        }
    }

    #[test]
    fn placeable_water_builds_scene_bounds_skips_invalid_polygon_and_initializes_havok_once() {
        let mut e = Engine::new();
        for base in (0x1000..=0xf000).step_by(0x1000) {
            e.map(base, 0x1000);
        }
        e.map(0x011c_57ac, 8);
        e.map(0x0101_1590, 8);
        e.mem.set_f64(0x0101_1590, 2.0);
        e.mem.set_u32(0x011c_57b0, 0x2000);
        e.mem.set_u32(0x2000, 0x2100);
        e.mem.set_u32(0x2004, 0x2140);
        e.mem.set_u32(0x2008, 3);
        for (node, next, previous, polygon) in [
            (0x2100, 0x2120, 0, 0x3000),
            (0x2120, 0x2140, 0x2100, 0x3100),
            (0x2140, 0, 0x2120, 0x3200),
        ] {
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, polygon);
        }
        point_helpers(&mut e);
        cell_offset_helpers(&mut e);
        e.register(0x0055_9450, |e, a| Ret {
            eax: if a[0] == 0x2000 {
                e.mem.u32(a[0])
            } else {
                0xd000 + (a[0] - 0x50b8) / 2
            },
            ..Ret::default()
        });
        e.register(0x007a_f430, |_, a| Ret {
            eax: 0xe000 + (a[0] - 0xd000),
            ..Ret::default()
        });
        for buffer in [0xe000, 0xe100] {
            for (index, values) in [[-2.0, -4.0, -6.0], [2.0, 4.0, 6.0]]
                .into_iter()
                .enumerate()
            {
                for (axis, value) in values.into_iter().enumerate() {
                    e.mem
                        .set_f32(buffer + index as u32 * 12 + axis as u32 * 4, value);
                }
            }
        }
        for address in [
            0x0040_4eb0,
            0x0040_4ee0,
            0x0043_81b0,
            0x0043_9410,
            0x0043_9260,
            0x0063_3ba0,
            0x0043_9360,
            0x0053_3fb0,
            0x0043_b370,
            0x0057_5690,
            0x0044_0460,
            0x0043_9680,
            0x004a_ddc0,
            0x0043_9920,
            0x0043_d410,
            0x00a5_9c60,
            0x0054_8230,
            0x0048_4490,
            0x0056_c8f0,
            0x0049_e3e0,
            0x004a_49e0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0056_c7f0, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                eax: if a[0] == 0x3100 {
                    0
                } else {
                    0x4000 + a[0] - 0x3000
                },
                ..Ret::default()
            }
        });
        e.register(0x004a_8a90, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                eax: 0x5000 + a[0] - 0x3000,
                ..Ret::default()
            }
        });
        e.register(0x0043_8170, |_, a| {
            assert_eq!(a.len(), 2);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x00a5_b950, |_, a| {
            assert_eq!(a.len(), 2);
            Ret::default()
        });
        e.register(0x00b5_7e30, |_, a| {
            assert_eq!(a.len(), 3);
            assert_eq!(a[1..], [0, 0]);
            Ret::default()
        });
        let mut references = 0;
        e.register_double(0x0040_1000, move |_, a| {
            assert_eq!(a, [0x68]);
            let allocation = 0x6000 + references * 0x100;
            references += 1;
            Ret {
                eax: allocation,
                ..Ret::default()
            }
        });
        e.register(0x0055_a2f0, |e, a| {
            e.mem.set_u32(a[0], 0xf200);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let mut allocations = [0u32; 6];
        e.register_double(0x00aa_13e0, move |_, a| {
            // 0x24 is used twice: stencil first, then the bound shape.
            let index = match a[0] {
                0x150 => 0,
                0x24 => {
                    if allocations[1] <= allocations[3] {
                        1
                    } else {
                        3
                    }
                }
                0xe4 => 2,
                0x10 => 4,
                0xb4 => 5,
                _ => panic!("unexpected size"),
            };
            let allocation = 0x7000 + index as u32 * 0x1000 + allocations[index] * 0x100;
            allocations[index] += 1;
            Ret {
                eax: allocation,
                ..Ret::default()
            }
        });
        for (address, vtable) in [
            (0x00b6_abb0, 0),
            (0x00b4_e150, 0xf000),
            (0x00c3_7ec0, 0xf400),
            (0x00c3_5e00, 0),
            (0x00c4_6970, 0xf600),
        ] {
            e.register_double(address, move |e, a| {
                assert_eq!(a.len(), 1);
                if vtable != 0 {
                    e.mem.set_u32(a[0], vtable);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        for (slot, function) in [
            (0xf0dc, 0xf100),
            (0xf3cc, 0xf300),
            (0xf4b8, 0xf500),
            (0xf6dc, 0xf700),
        ] {
            e.mem.set_u32(slot, function);
        }
        e.register(0xf100, |_, a| {
            assert_eq!(a.len(), 3);
            assert_eq!(a[2], 1);
            Ret::default()
        });
        e.register(0xf300, |_, a| {
            assert_eq!(a.len(), 3);
            assert_eq!(a[2], 1);
            Ret::default()
        });
        e.register(0xf500, |e, a| {
            assert_eq!(
                [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)],
                [11.5, 22.0, 7.0]
            );
            Ret::default()
        });
        e.register(0xf700, |_, a| {
            assert_eq!(a.len(), 3);
            assert_eq!(a[2], 1);
            Ret::default()
        });
        e.register(0x0054_7770, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 0x1600,
                ..Ret::default()
            }
        });
        e.register(0x0058_0130, |_, a| {
            assert_eq!(a, [0x1600]);
            Ret {
                eax: 0x1700,
                ..Ret::default()
            }
        });
        e.register(0x0045_6650, |_, _| Ret {
            eax: 0x1234_0002,
            ..Ret::default()
        });
        e.register(0x0040_ebd0, |_, a| Ret {
            st0: f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        e.register(0x0040_4010, |_, a| Ret {
            st0: f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        let mut absolute_calls = 0;
        e.register_double(0x0040_8840, move |e, a| {
            assert_eq!(a.len(), 1);
            let axis = absolute_calls % 3;
            assert_eq!(f32::from_bits(a[0]), [6.0, 2.0, 0.5][axis]);
            absolute_calls += 1;
            e.mem.set_f64(0x0101_1590, [4.0, 8.0, 2.0][axis]);
            Ret {
                st0: f32::from_bits(a[0]).abs() as f64,
                ..Ret::default()
            }
        });
        e.register(0x0063_c8a0, |e, a| {
            for offset in [0, 4, 8] {
                e.mem.set_f32(
                    a[0] + offset,
                    (e.mem.f32(a[0] + offset) as f64 + e.mem.f32(a[1] + offset) as f64) as f32,
                );
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.mem.set_u8(0x1500, 1);
        e.register(0x0040_8d60, |_, a| {
            assert_eq!(a, [0x011c_c5a8]);
            Ret {
                eax: 0x1500,
                ..Ret::default()
            }
        });
        e.register(0x007b_5390, |e, a| {
            assert_eq!(a, [0x2000]);
            let head = e.mem.u32(0x2000);
            let next = e.mem.u32(head);
            e.mem.set_u32(0x2000, next);
            e.mem.set_u32(next + 4, 0);
            e.mem.set_u32(0x2008, e.mem.u32(0x2008) - 1);
            Ret::default()
        });
        e.register(0x004e_df00, |e, a| {
            assert_eq!(a, [0x2000]);
            let tail = e.mem.u32(0x2004);
            let previous = e.mem.u32(tail + 4);
            e.mem.set_u32(previous, 0);
            e.mem.set_u32(0x2004, previous);
            e.mem.set_u32(0x2008, e.mem.u32(0x2008) - 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        bg_auto_water_generate_placeable_water(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x2000), 0x2120);
        assert_eq!(e.mem.u32(0x2004), 0x2120);
        assert_eq!(e.mem.u32(0x2008), 1);
        for reference in [0x6000, 0x6100] {
            assert_eq!(
                [
                    e.mem.f32(reference + 0x30),
                    e.mem.f32(reference + 0x34),
                    e.mem.f32(reference + 0x38)
                ],
                [10.0, 20.0, 7.0]
            );
        }
        let calls = e.call_log.unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0056_c8f0)
                .count(),
            1
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0054_8230)
                .count(),
            2
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0049_e3e0)
                .count(),
            2
        );
        assert_eq!(calls.last().unwrap().0, 0x0040_4ee0);
    }

    #[test]
    fn linked_node_accessor_returns_payload_or_null() {
        let mut e = Engine::new();
        e.map(0x1000, 0x30);
        e.map(0x2000, 8);
        e.mem.set_u32(0x1028, 0x2000);
        e.mem.set_u32(0x2004, 0x3456);
        assert_eq!(fn_0049db30(&mut e, Ptr::new(0x1000)).addr(), 0x3456);
        e.mem.set_u32(0x1028, 0);
        assert_eq!(fn_0049db30(&mut e, Ptr::new(0x1000)).addr(), 0);
    }

    #[test]
    fn plane_side_classifier_covers_both_sides_zero_and_unordered() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 12);
        e.map(0x0101_2060, 8);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x1000, 1.0);
        e.mem.set_f32(0x1004, 0.0);
        e.mem.set_f32(0x1008, 0.0);
        e.mem.set_f32(0x100c, 2.0);
        e.mem.set_f32(0x2000, 3.0);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 1);
        e.mem.set_f32(0x2000, 2.0);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 0);
        e.mem.set_f32(0x2000, 1.0);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 2);
        e.mem.set_f32(0x2000, f32::NAN);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 0);
        e.mem.set_f32(0x2000, 3.0);
        e.mem.set_f64(0x0101_2060, 1.0);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 0);
        e.mem.set_f64(0x0101_2060, 3.0);
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 2);
    }

    /// 0049da80 at the main thread's 24-bit precision: the FADDP of
    /// `1 * 1` and `1 * 2^-30` (0049da9f) rounds to 1, so the distance to the
    /// plane at 1 is 0 (on the plane); at 53 bits it stays 2^-30 (positive).
    #[test]
    fn plane_side_classifier_rounds_each_x87_operation_at_24_bit_precision() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 12);
        e.map(0x0101_2060, 8);
        e.mem.set_f64(0x0101_2060, 0.0);
        for (offset, value) in [(0u32, 1.0f32), (4, 1.0), (8, 0.0), (0xc, 1.0)] {
            e.mem.set_f32(0x1000 + offset, value);
        }
        for (offset, value) in [(0u32, 1.0f32), (4, 2f32.powi(-30)), (8, 0.0)] {
            e.mem.set_f32(0x2000 + offset, value);
        }
        assert_eq!(fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)), 1);
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        let at_24_bits = fn_0049da80(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
        assert_eq!(at_24_bits, 0);
    }

    #[test]
    fn cell_water_height_reads_land_field_or_returns_zero() {
        let mut e = Engine::new();
        e.map(0x1000, 0x30);
        e.map(0x2000, 0xa4);
        e.mem.set_u32(0x1028, 0x2000);
        e.mem.set_f32(0x20a0, 42.5);
        assert_eq!(fn_0049db00(&mut e, Ptr::new(0x1000)), 42.5);
        e.mem.set_u32(0x1028, 0);
        assert_eq!(fn_0049db00(&mut e, Ptr::new(0x1000)), 0.0);
    }

    /// 0049db00 (0049db18-0049db28): the land's +0xa0 float goes through
    /// `FLD`, `FSTP float` to the local and `FLD` back, so a signalling NaN
    /// comes back quieted.
    #[test]
    fn land_float_getter_quiets_a_signalling_nan_through_fld_fstp() {
        let mut e = Engine::new();
        e.map(0x1000, 0x30);
        e.map(0x2000, 0xa4);
        e.mem.set_u32(0x1028, 0x2000);
        e.mem.set_u32(0x20a0, 0x7f80_0001);
        assert_eq!(fn_0049db00(&mut e, Ptr::new(0x1000)).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn polygon_generation_empty_points_and_empty_water_list_only_closes_guard() {
        let mut e = Engine::new();
        e.map(0x011c_57ac, 4);
        e.map(0x011c_57b0, 4);
        e.mem.set_u32(0x011c_57ac, 0);
        e.mem.set_u32(0x011c_57b0, 0);
        double(&mut e, 0x0040_4eb0);
        double(&mut e, 0x0040_4ee0);
        e.register(0x0044_ddc0, |_, a| {
            assert_eq!(a, [0]);
            Ret::default()
        });
        bg_auto_water_generate_polygons(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
        assert_eq!(e.call_log.as_ref().unwrap()[0].0, 0x0040_4eb0);
        assert_eq!(e.call_log.as_ref().unwrap()[2].0, 0x0040_4ee0);
    }

    #[test]
    fn cell_generation_calls_predicate_and_direct_passes_in_order() {
        let mut e = Engine::new();
        e.map(0x1000, 0x28);
        e.map(0x011c_57ac, 4);
        e.map(0x011c_57b0, 4);
        e.mem.set_u32(0x011c_57ac, 0x2000);
        e.mem.set_u32(0x011c_57b0, 0x3000);
        for address in [
            0x0040_4eb0,
            0x0040_4ee0,
            0x0084_54f0,
            0x0054_6fb0,
            0x0054_71e0,
            0x0044_ddc0,
        ] {
            double(&mut e, address);
        }
        e.register(0x0045_18e0, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(0x004e_d900, |e, a| {
            assert_eq!(a, [0x3000]);
            e.mem.set_u32(0x011c_57b0, 0);
            Ret::default()
        });
        bg_auto_water_generate_placeable_water_for_cell(&mut e, Ptr::new(0x1000));
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(calls[1], (0x0045_18e0, vec![0x1000]));
        assert_eq!(calls[2], (0x0084_54f0, vec![0x2000, 1]));
        assert_eq!(calls[3], (0x004e_d900, vec![0x3000]));
        assert_eq!(calls[4], (0x0054_6fb0, vec![0x1000]));
        let guard_lines: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == 0x0040_4eb0)
            .map(|(_, args)| args[4])
            .collect();
        assert_eq!(guard_lines, [0x53, 0x20a, 0x2cd]);
        assert_eq!(calls.last().unwrap().0, 0x0040_4ee0);
        e.call_log = Some(vec![]);
        e.register(0x0045_18e0, |_, _| Ret::default());
        bg_auto_water_generate_placeable_water_for_cell(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
        e.call_log = Some(vec![]);
        bg_auto_water_generate_placeable_water_for_cell(&mut e, Ptr::new(0));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn nested_property_helpers_narrow_arguments_and_copy_point_words() {
        let mut e = Engine::new();
        e.map(0x1000, 0x40);
        e.map(0x2000, 12);
        e.mem.set_u32(0x2000, 0x1122_3344);
        e.mem.set_u32(0x2004, 0x5566_7788);
        e.mem.set_u32(0x2008, 0x99aa_bbcc);
        double(&mut e, 0x0063_3ba0);
        double(&mut e, 0x0043_9360);
        fn_0049ed90(&mut e, Ptr::new(0x1000), 0x23u8);
        fn_0049ede0(&mut e, Ptr::new(0x1000), 0x4567u16);
        fn_0049eea0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(e.mem.u32(0x1030), 0x1122_3344);
        assert_eq!(e.mem.u32(0x1034), 0x5566_7788);
        assert_eq!(e.mem.u32(0x1038), 0x99aa_bbcc);
        assert_eq!(
            e.call_log.as_ref().unwrap()[0],
            (0x0063_3ba0, vec![0x1000, 0x23, 1])
        );
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x0043_9360, vec![0x1000, 0x4567, 0x380, 7])
        );
    }

    #[test]
    fn property_setter_entries_narrow_stack_words_to_declared_types() {
        let mut e = Engine::new();
        double(&mut e, 0x0043_9360);
        double(&mut e, 0x0043_b370);
        e.call(0x0049_edb0, &args![0x1000, 0xabcd_1234u32]);
        e.call(0x0049_ee60, &args![0x1000, 0x1234_5678]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x0043_9360, vec![0x1000, 0x1234, 0xf000, 12])
        );
        assert_eq!(
            e.call_log.as_ref().unwrap()[3],
            (0x0043_b370, vec![0x1000, 0x78, 0x8000])
        );
    }

    #[test]
    fn embedded_save_buffer_resolves_form_through_both_calls() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, [0x10b8]);
            Ret {
                eax: 0x2000,
                ..Ret::default()
            }
        });
        e.register(0x007a_f430, |_, a| {
            assert_eq!(a, [0x2000]);
            Ret {
                eax: 0x5678,
                ..Ret::default()
            }
        });
        assert_eq!(fn_0049ec60(&mut e, Ptr::new(0x1000)), 0x5678);
    }

    #[test]
    fn indexed_point_predicate_refreshes_count_and_stops_at_first_match() {
        let mut e = Engine::new();
        e.map(0x3000, 12);
        e.map(0x3010, 12);
        e.mem.set_f32(0x3000, 1.0);
        e.mem.set_f32(0x3004, 2.0);
        e.mem.set_f32(0x3008, 3.0);
        e.mem.set_f32(0x3010, 4.0);
        e.mem.set_f32(0x3014, 5.0);
        e.mem.set_f32(0x3018, 6.0);
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 0x2000,
                ..Ret::default()
            }
        });
        e.register_double(0x0059_bb30, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        e.register(0x006a_7af0, |_, a| {
            assert_eq!(a[0], 0x2000);
            Ret {
                eax: if a[1] == 0 { 0x3000 } else { 0x3010 },
                ..Ret::default()
            }
        });
        e.register(0x0045_2dc0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x00c4_e0a0, |_, a| {
            assert_eq!(a[0], 0x4000);
            Ret {
                eax: u32::from(a == [0x4000, 4.0f32.to_bits(), 5.0f32.to_bits()]),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert!(fn_0049eed0(&mut e, Ptr::new(0x1000), Ptr::new(0x4000)));
        let call_addresses: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(
            call_addresses,
            [
                0x0059_bb30,
                0x0055_9450,
                0x006a_7af0,
                0x0045_2dc0,
                0x00c4_e0a0,
                0x0059_bb30,
                0x0055_9450,
                0x006a_7af0,
                0x0045_2dc0,
                0x00c4_e0a0
            ]
        );

        e.register_double(0x0059_bb30, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!fn_0049eed0(&mut e, Ptr::new(0x1000), Ptr::new(0x4000)));
        assert_eq!(e.call_log.as_ref().unwrap(), &[(0x0059_bb30, vec![0x1000])]);
    }

    /// 0049eed0 (0049ef1a-0049ef24): the point's x and y reach `00452dc0`
    /// through `FLD`/`FSTP float`, so a signalling NaN arrives quieted.
    #[test]
    fn indexed_point_predicate_quiets_a_signalling_nan_through_fld_fstp() {
        let mut e = Engine::new();
        e.map(0x3000, 12);
        e.mem.set_u32(0x3000, 0x7f80_0001);
        e.mem.set_f32(0x3004, 2.0);
        e.mem.set_f32(0x3008, 3.0);
        e.register(0x0059_bb30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x0055_9450, |_, _| Ret {
            eax: 0x2000,
            ..Ret::default()
        });
        e.register(0x006a_7af0, |_, _| Ret {
            eax: 0x3000,
            ..Ret::default()
        });
        e.register(0x0045_2dc0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        e.register(0x00c4_e0a0, |_, a| Ret {
            eax: u32::from(a[1..] == [0x7fc0_0001, 2.0f32.to_bits()]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(fn_0049eed0(&mut e, Ptr::new(0x1000), Ptr::new(0x4000)));
        let log = e.call_log.take().unwrap();
        let construct: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == 0x0045_2dc0)
            .collect();
        assert_eq!(construct.len(), 1);
        assert_eq!(construct[0].1[1..], [0x7fc0_0001, 2.0f32.to_bits()]);
    }

    #[test]
    fn stencil_constructor_sets_defaults_and_calls_base_and_mode_setters() {
        let mut e = Engine::new();
        e.map(0x1000, 0x24);
        for address in [0x0043_9260, 0x0063_3ba0, 0x0043_9360, 0x0053_3fb0] {
            double(&mut e, address);
        }
        assert_eq!(fn_0049ec80(&mut e, Ptr::new(0x1000)), Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1000), 0x0101_e07c);
        assert_eq!(e.mem.u16(0x1018), 0);
        assert_eq!(e.mem.u32(0x101c), 0);
        assert_eq!(e.mem.u32(0x1020), u32::MAX);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &[
                (0x0043_9260, vec![0x1000]),
                (0x0063_3ba0, vec![0x1000, 0, 1]),
                (0x0043_9360, vec![0x1000, 4, 0xf000, 12]),
                (0x0053_3fb0, vec![0x1000, 0]),
                (0x0043_9360, vec![0x1000, 0, 0x70, 4]),
                (0x0043_9360, vec![0x1000, 3, 0x380, 7]),
                (0x0043_9360, vec![0x1000, 0, 0xc00, 10]),
            ]
        );
    }

    #[test]
    fn rtti_getter_returns_exe_address() {
        assert_eq!(
            ni_stencil_property_get_rtti(&mut Engine::new(), Ptr::new(0x1000)).addr(),
            0x011f_4a0c
        );
    }

    fn edge_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0054_6fb0, |_, a| {
            assert_eq!(a, [0x1000]);
            Ret {
                eax: 0x2000,
                ..Ret::default()
            }
        });
        e.register(0x0053_5b30, |e, a| {
            assert_eq!(a[0], 0x2000);
            assert_eq!(a[2], 0);
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(0x0063_c8a0, |e, a| {
            for offset in [0, 4, 8] {
                let sum =
                    (e.mem.f32(a[0] + offset) as f64 + e.mem.f32(a[1] + offset) as f64) as f32;
                e.mem.set_f32(a[0] + offset, sum);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [0x0054_4c60, 0x0054_4c30] {
            e.register(address, |_, a| {
                assert_eq!(a, [0x1000]);
                Ret::default()
            });
        }
        e.register(0x0045_2dc0, |e, a| {
            e.mem.set_f32(a[0], f32::from_bits(a[1]));
            e.mem.set_f32(a[0] + 4, f32::from_bits(a[2]));
            e.mem.set_f32(a[0] + 8, 0.0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn which_cell_edge_preserves_corner_priority_edge_bits_and_exact_equality() {
        let mut e = edge_engine();
        let cell = Ptr::new(0x1000);
        for (x, y, expected) in [
            (0.0, 0.0, 9),
            (4096.0, 0.0, 10),
            (0.0, 4096.0, 5),
            (4096.0, 4096.0, 6),
            (0.0, 100.0, 1),
            (4096.0, 100.0, 2),
            (100.0, 0.0, 8),
            (100.0, 4096.0, 4),
            (100.0, 200.0, 0),
        ] {
            assert_eq!(
                bg_auto_water_which_cell_edge(&mut e, cell, x, y, 7.0),
                expected
            );
        }
        assert_eq!(
            bg_auto_water_which_cell_edge(&mut e, cell, f32::NAN, 0.0, 0.0),
            8
        );
        assert_eq!(
            bg_auto_water_which_cell_edge(&mut e, cell, f32::NAN, f32::NAN, 0.0),
            0
        );
    }

    #[test]
    fn which_cell_edge_uses_fourth_fresh_y_value_for_high_y_edge() {
        let mut e = edge_engine();
        let mut calls = 0u32;
        e.register_double(0x0054_4c60, move |_, a| {
            assert_eq!(a, [0x1000]);
            let value = calls;
            calls += 1;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        assert_eq!(
            bg_auto_water_which_cell_edge(&mut e, Ptr::new(0x1000), 100.0, 16_384.0, 0.0),
            4
        );
    }

    #[test]
    fn stencil_destructor_writes_vtable_calls_base_and_obeys_delete_flag() {
        let mut e = Engine::new();
        e.map(0x1000, 0x24);
        double(&mut e, 0x0043_92c0);
        double(&mut e, FREE);
        ni_stencil_property_scalar_deleting_destructor(&mut e, Ptr::new(0x1000), 0);
        assert_eq!(e.mem.u32(0x1000), 0x0101_e07c);
        assert_eq!(e.call_log.as_ref().unwrap(), &[(0x0043_92c0, vec![0x1000])]);
        e.call_log = Some(vec![]);
        ni_stencil_property_scalar_deleting_destructor(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (FREE, vec![0x1000, 0x24]));
    }

    macro_rules! setter_test {
        ($name:ident, $function:ident, $callee:expr, $value:expr, $tail:expr) => {
            #[test]
            fn $name() {
                let mut e = Engine::new();
                double(&mut e, $callee);
                $function(&mut e, Ptr::new(0x1000), $value);
                let mut expected = vec![0x1000, $value as u32];
                expected.extend($tail);
                assert_eq!(e.call_log.as_ref().unwrap(), &[($callee, expected)]);
            }
        };
    }

    setter_test!(
        stencil_function_forwards_fixed_arguments,
        fn_0049edb0,
        0x0043_9360,
        0x1234u16,
        vec![0xf000, 12]
    );
    setter_test!(
        stencil_operation_forwards_fixed_arguments,
        fn_0049ee10,
        0x0043_9360,
        0xabu16,
        vec![0x70, 4]
    );
    setter_test!(
        stencil_test_forwards_fixed_arguments,
        fn_0049ee30,
        0x0043_9360,
        0x55u16,
        vec![0xc00, 10]
    );
    setter_test!(
        byte_property_forwards_through_sibling,
        fn_0049ee60,
        0x0043_b370,
        0x5au8,
        vec![0x8000]
    );
    setter_test!(
        byte_property_uses_fixed_mask,
        fn_0049ee80,
        0x0043_b370,
        0xffu8,
        vec![0x8000]
    );

    #[test]
    fn point_search_returns_matching_index_and_empty_sentinel() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 3);
        e.register(0x3000, |_, a| Ret {
            eax: u32::from(a[0] == 0x2018),
            ..Ret::default()
        });
        assert_eq!(fn_0049f7d0(&mut e, Ptr::new(0x1000), 0, 0, 0x3000), 2);
        e.mem.set_u32(0x1008, 0);
        assert_eq!(
            fn_0049f7d0(&mut e, Ptr::new(0x1000), 0, 0, 0x3000),
            u32::MAX
        );
    }

    #[test]
    fn point_search_reloads_array_fields_after_the_callback_mutates_them() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 48);
        e.map(0x3000, 48);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 3);
        e.register_double(0x3000, |e, a| {
            if a[0] == 0x2000 {
                e.mem.set_u32(0x1004, 0x3000);
                e.mem.set_u32(0x1008, 2);
                Ret::default()
            } else {
                assert_eq!(a[0], 0x300c);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            }
        });
        assert_eq!(fn_0049f7d0(&mut e, Ptr::new(0x1000), 9, 0, 0x3000), 1);
    }

    #[test]
    fn membership_uses_the_point_comparator() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 1);
        e.register(0x0043_90c0, |_, a| Ret {
            eax: u32::from(a == [0x2000, 0xfeed]),
            ..Ret::default()
        });
        assert!(fn_0049f190(&mut e, Ptr::new(0x1000), 0xfeed));
    }

    #[test]
    fn point_insert_appends_at_size_and_shifts_when_inserting_inside() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 48);
        e.map(0x3000, 12);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 2);
        e.map(0x0049_f210, 4);
        e.map(0x0043_8b90, 4);
        e.map(0x006d_c6e0, 4);
        e.map(0x0049_f8f0, 4);
        e.map(0x006a_8500, 4);
        e.mem.set_u32(0x3000, 0xaaaa);
        e.mem.set_u32(0x3004, 0xbbbb);
        e.mem.set_u32(0x3008, 0xcccc);
        double(&mut e, 0x0049_f210);
        ni_point3_array_insert(&mut e, Ptr::new(0x1000), 2, Ptr::new(0x3000));
        assert_eq!(e.mem.u32(0x1008), 2);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &[(0x0049_f210, vec![0x1000, 0x3000])]
        );

        e.register(0x0043_8b90, |_, _| Ret::default());
        double(&mut e, 0x006d_c6e0);
        double(&mut e, 0x0049_f8f0);
        double(&mut e, 0x006a_8500);
        ni_point3_array_insert(&mut e, Ptr::new(0x1000), 1, Ptr::new(0x3000));
        assert_eq!(e.mem.u32(0x1008), 3);
        assert_eq!(
            [e.mem.u32(0x200c), e.mem.u32(0x2010), e.mem.u32(0x2014)],
            [0xaaaa, 0xbbbb, 0xcccc]
        );
    }

    #[test]
    fn point_insert_relocated_path_reloads_buffer_before_copy() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 48);
        e.map(0x3000, 48);
        e.map(0x4000, 12);
        e.map(0x5000, 8);
        e.mem.set_u32(0x1000, 0x5000);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 2);
        e.mem.set_u32(0x100c, 2);
        e.mem.set_u32(0x5004, 0x6000);
        e.mem.set_u32(0x4000, 0x1234);
        e.mem.set_u32(0x4004, 0x5678);
        e.mem.set_u32(0x4008, 0x9abc);
        double(&mut e, 0x0043_8b90);
        e.register(0x0043_8b90, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x6000, |_, _| Ret {
            eax: 0x3000,
            ..Ret::default()
        });
        e.register(0x009a_3910, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        for address in [0x006d_c6e0, 0x0049_f8f0, 0x006a_8500] {
            e.register(address, |_, _| Ret::default());
        }
        ni_point3_array_insert(&mut e, Ptr::new(0x1000), 1, Ptr::new(0x4000));
        assert_eq!(e.mem.u32(0x1004), 0x3000);
        assert_eq!(e.mem.u32(0x1008), 3);
        assert_eq!(
            [e.mem.u32(0x300c), e.mem.u32(0x3010), e.mem.u32(0x3014)],
            [0x1234, 0x5678, 0x9abc]
        );
    }

    #[test]
    fn point_insert_reloads_fields_after_in_place_callbacks() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        for base in [0x2000, 0x3000, 0x4000, 0x5000, 0x6000] {
            e.map(base, 48);
        }
        e.map(0x7000, 12);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 2);
        e.mem.set_u32(0x7000, 0xaaaa);
        e.mem.set_u32(0x7004, 0xbbbb);
        e.mem.set_u32(0x7008, 0xcccc);
        e.register_double(0x0043_8b90, |e, _| {
            e.mem.set_u32(0x1004, 0x4000);
            e.mem.set_u32(0x1008, 3);
            Ret::default()
        });
        e.register_double(0x006d_c6e0, |e, a| {
            assert_eq!(a, [0x1000, 0x4018, 0x400c, 2]);
            e.mem.set_u32(0x1004, 0x5000);
            e.mem.set_u32(0x1008, 4);
            Ret::default()
        });
        e.register_double(0x0049_f8f0, |e, a| {
            assert_eq!(a, [0x1000, 0x500c, 1]);
            e.mem.set_u32(0x1004, 0x6000);
            e.mem.set_u32(0x1008, 5);
            Ret::default()
        });
        ni_point3_array_insert(&mut e, Ptr::new(0x1000), 1, Ptr::new(0x7000));
        assert_eq!(e.mem.u32(0x1008), 6);
        assert_eq!(
            [e.mem.u32(0x600c), e.mem.u32(0x6010), e.mem.u32(0x6014)],
            [0xaaaa, 0xbbbb, 0xcccc]
        );
    }

    #[test]
    fn point_range_remove_reloads_size_and_buffer_after_destructor_callback() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 96);
        e.map(0x3000, 96);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 5);
        e.register_double(0x0072_ba80, |e, a| {
            assert_eq!(a, [0x1000, 0x200c, 1]);
            e.mem.set_u32(0x1004, 0x3000);
            e.mem.set_u32(0x1008, 7);
            Ret::default()
        });
        e.register(0x006d_c6e0, |_, a| {
            assert_eq!(a, [0x1000, 0x300c, 0x3018, 5]);
            Ret::default()
        });
        fn_0049f3c0(&mut e, Ptr::new(0x1000), 1, 1, 0);
        assert_eq!(e.mem.u32(0x1008), 6);
    }

    #[test]
    fn point_range_remove_relocation_keeps_the_exes_distinct_copy_count() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 96);
        e.map(0x3000, 96);
        e.map(0x4000, 96);
        e.map(0x5000, 8);
        e.mem.set_u32(0x1000, 0x5000);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 5);
        e.mem.set_u32(0x100c, 5);
        e.mem.set_u32(0x5004, 0x6000);
        e.register(0x006f_3170, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x0086_9600, |_, _| Ret {
            eax: 8,
            ..Ret::default()
        });
        e.register(0x6000, |_, _| Ret {
            eax: 0x3000,
            ..Ret::default()
        });
        let mut copies = 0;
        e.register_double(0x006d_c6e0, move |_, a| {
            copies += 1;
            if copies == 1 {
                assert_eq!(a, [0x1000, 0x3000, 0x2000, 1]);
            } else {
                assert_eq!(a, [0x1000, 0x300c, 0x4018, 6]);
            }
            Ret::default()
        });
        e.register_double(0x0072_ba80, |e, a| {
            assert_eq!(a, [0x1000, 0x200c, 1]);
            e.mem.set_u32(0x1004, 0x4000);
            e.mem.set_u32(0x1008, 7);
            Ret::default()
        });
        e.register_double(0x006a_8500, |e, _| {
            e.mem.set_u32(0x1008, 10);
            Ret::default()
        });
        fn_0049f3c0(&mut e, Ptr::new(0x1000), 1, 1, 1);
        assert_eq!(e.mem.u32(0x1004), 0x3000);
        assert_eq!(e.mem.u32(0x1008), 9);
        assert_eq!(e.mem.u32(0x100c), 8);
    }

    #[test]
    fn list_cursor_remove_unlinks_middle_node_and_returns_payload() {
        let mut e = Engine::new();
        e.map(0x1000, 12);
        e.map(0x2000, 12);
        e.map(0x3000, 12);
        e.map(0x4000, 12);
        e.map(0x5000, 4);
        e.mem.set_u32(0x1000, 0x2000);
        e.mem.set_u32(0x1004, 0x4000);
        e.mem.set_u32(0x1008, 3);
        e.mem.set_u32(0x2000, 0x3000);
        e.mem.set_u32(0x2004, 0);
        e.mem.set_u32(0x3000, 0x4000);
        e.mem.set_u32(0x3004, 0x2000);
        e.mem.set_u32(0x3008, 0xfeed);
        e.mem.set_u32(0x4000, 0);
        e.mem.set_u32(0x4004, 0x3000);
        e.mem.set_u32(0x5000, 0x3000);
        e.register(0x004a_49e0, |e, a| {
            assert_eq!(a, [0x1000, 0x3000]);
            e.mem.set_u32(0x3008, 0); // Return value was captured before cleanup.
            e.mem.set_u32(0x1008, 4); // The final decrement reads the current size.
            Ret::default()
        });
        assert_eq!(
            fn_0049f590(&mut e, Ptr::new(0x1000), Ptr::new(0x5000)),
            0xfeed
        );
        assert_eq!(e.mem.u32(0x5000), 0x4000);
        assert_eq!(e.mem.u32(0x2000), 0x4000);
        assert_eq!(e.mem.u32(0x4004), 0x2000);
        assert_eq!(e.mem.u32(0x1000), 0x2000);
        assert_eq!(e.mem.u32(0x1004), 0x4000);
        assert_eq!(e.mem.u32(0x1008), 3);
    }

    #[test]
    fn point_pop_is_noop_when_empty_and_destroys_last_when_nonempty() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 0x30);
        e.mem.set_u32(0x1004, 0x2000);
        e.register_double(0x0072_ba80, |e, _| {
            e.mem.set_u32(0x1008, 5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_0049f510(&mut e, Ptr::new(0x1000));
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.mem.set_u32(0x1008, 2);
        fn_0049f510(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1008), 4);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &[(0x0072_ba80, vec![0x1000, 0x200c, 1])]
        );
    }

    #[test]
    fn point_sort_passes_element_stride_and_callback() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 4);
        double(&mut e, 0x00ec_6f20);
        fn_0049f560(&mut e, Ptr::new(0x1000), 0x3000);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &[(0x00ec_6f20, vec![0x2000, 4, 12, 0x3000])]
        );
    }

    #[test]
    fn comparator_adapter_passes_both_points_and_returns_low_byte() {
        let mut e = Engine::new();
        e.register(0x0043_90c0, |_, a| {
            assert_eq!(a, [0x1111, 0x2222]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert!(fn_0049f830(&mut e, 0x1111, 0x2222));
    }

    #[test]
    fn nested_array_constructor_sets_vtable_and_empty_header() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        fn_0049f640(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1000), 0x0101_e128);
        assert_eq!(
            [e.mem.u32(0x1004), e.mem.u32(0x1008), e.mem.u32(0x100c)],
            [0; 3]
        );
    }

    #[test]
    fn nested_array_append_uses_index_and_constructs_element() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 48);
        e.mem.set_u32(0x1004, 0x2000);
        e.register(0x006f_31f0, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        double(&mut e, ARRAY_SET_SIZE);
        double(&mut e, 0x0049_f8b0);
        assert_eq!(fn_0049faa0(&mut e, Ptr::new(0x1000), 0x7788), 2);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x0049_f8b0, vec![0x2020, 0x7788])));
    }

    #[test]
    fn nested_array_destructor_uses_keep_allocation_mode() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        double(&mut e, 0x0049_faf0);
        fn_0049f670(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1000), 0x0101_e128);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &[(0x0049_faf0, vec![0x1000, 1])]
        );
    }

    #[test]
    fn nested_array_insert_shifts_and_constructs_one_slot() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.map(0x2000, 64);
        e.mem.set_u32(0x1004, 0x2000);
        e.mem.set_u32(0x1008, 2);
        e.register(0x0043_8b90, |_, _| Ret::default());
        double(&mut e, 0x006c_8bd0);
        double(&mut e, ARRAY_SET_SIZE);
        double(&mut e, 0x0049_f8b0);
        fn_0049f690(&mut e, Ptr::new(0x1000), 1, 0x1234);
        assert_eq!(e.mem.u32(0x1008), 3);
        assert_eq!(
            e.call_log.as_ref().unwrap().last().unwrap(),
            &(0x0049_f8b0, vec![0x2010, 0x1234])
        );
    }

    #[test]
    fn point_scalar_destructor_conditionally_deallocates() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        double(&mut e, 0x0049_f1f0);
        double(&mut e, ARRAY_FREE);
        ni_point3_array_scalar_deleting_destructor(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (ARRAY_FREE, vec![0x1000]));
    }

    #[test]
    fn nested_scalar_destructor_conditionally_deallocates() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        double(&mut e, 0x0049_faf0);
        double(&mut e, ARRAY_FREE);
        nested_point3_array_scalar_deleting_destructor(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (ARRAY_FREE, vec![0x1000]));
    }

    #[test]
    fn point_comparator_returns_low_byte_from_callee() {
        let mut e = Engine::new();
        e.register(0x0043_90c0, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        assert!(!fn_0049f830(&mut e, 0, 0));
    }

    #[test]
    fn array_initializer_zeroes_fields_when_no_capacity_or_size() {
        let mut e = Engine::new();
        e.map(0x1000, 16);
        e.mem.set_u32(0x1000, 0x3000);
        fn_0049fbf0(&mut e, Ptr::new(0x1000), 0, 0);
        assert_eq!(
            [e.mem.u32(0x1004), e.mem.u32(0x1008), e.mem.u32(0x100c)],
            [0; 3]
        );
    }
}
