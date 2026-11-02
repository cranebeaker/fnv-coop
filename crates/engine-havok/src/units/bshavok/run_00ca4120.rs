//! `BSHavok/run_00ca4120` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::MEMORY_MANAGER;

/// Matrix-times-vector routine (`00c7f640`, `this` = the 16-byte result, arguments the matrix
/// and the vector).
const TRANSFORM_POINT: u32 = 0x00c7_f640;
/// The float multiplied into every vertex coordinate.
const VERTEX_SCALE: u32 = 0x0126_82c8;
/// The floats the box corners start from: the largest value (for the minimum corner), the
/// smallest (for the maximum corner) and the one stored in the fourth lane of both.
const BOX_START_LOW: u32 = 0x0104_da48;
const BOX_START_HIGH: u32 = 0x0104_e148;
const BOX_FOURTH_LANE: u32 = 0x0101_1d78;
/// The tolerance `0.001` (`01017d00`) and `3.4028234663852886e38` as a `double` (`0104e138`).
const TOLERANCE: u32 = 0x0101_7d00;
const FLOAT_MAX_DOUBLE: u32 = 0x0104_e138;
/// `00cb2160(this, flag, a, tolerance)` (`RET 0xc`): writes a byte at `flag`, 1 when two
/// sets of vectors agree within the tolerance, and returns `flag`. Outside this unit.
const NEARLY_EQUAL: u32 = 0x00cb_2160;
/// Releases the buffer a block keeps at `+4` (`00aa10f0`, cdecl, one argument).
const RELEASE_BUFFER: u32 = 0x00aa_10f0;
/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the manager, one argument).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The vtable `fn_00ca4440` stores.
const RELEASE_VTABLE: u32 = 0x010c_759c;

/// `MINPS` for one lane: the destination when it is smaller, otherwise the source (so a NaN in
/// either gives the source).
fn min_lane(destination: f32, source: f32) -> f32 {
    if destination < source {
        destination
    } else {
        source
    }
}

/// `MAXPS` for one lane: the destination when it is larger, otherwise the source.
fn max_lane(destination: f32, source: f32) -> f32 {
    if destination > source {
        destination
    } else {
        source
    }
}

/// Reads the four floats at `address`.
fn read_lanes(e: &Engine, address: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|lane| e.mem.f32(address + 4 * lane))
}

/// Writes four floats to `address`.
fn write_lanes(e: &mut Engine, address: u32, lanes: [f32; 4]) {
    for (lane, value) in lanes.iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, *value);
    }
}

// Translated from 00ca4120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4120 - the /GS cookie check (00ec408c) is not modeled.
/// `fn_00ca4120` (unnamed in the Xbox PDB): takes the address of a 64-byte matrix. Copies the
/// matrix to `+0x20..+0x60`, starts the box at `+0x60` (minimum) and `+0x70` (maximum) from
/// the start floats, then, for each of the `+0xA0` entries of the 8-byte-stride array at
/// `+0x98`, takes the vertex list (`+0x20` of the entry's first word) of `+0x8`-many (16 bit)
/// vertices and for each vertex: scales its three coordinates by the float at 0x012682c8
/// (x87 multiply, stored as floats; the fourth lane is 0), multiplies the vector lane by lane
/// by the vector at `+0x80`, transforms it with the matrix (`00c7f640`) and folds the result
/// into the two corners with `MINPS` and `MAXPS`.
pub fn fn_00ca4120(e: &mut Engine, this: u32, matrix: u32) {
    for row in 0..4 {
        let lanes = read_lanes(e, matrix + 0x10 * row);
        write_lanes(e, this + 0x20 + 0x10 * row, lanes);
    }
    let low = e.mem.f32(BOX_START_LOW);
    let high = e.mem.f32(BOX_START_HIGH);
    let fourth = e.mem.f32(BOX_FOURTH_LANE);
    write_lanes(e, this + 0x60, [low, low, low, fourth]);
    write_lanes(e, this + 0x70, [high, high, high, fourth]);

    let count = e.mem.u32(this + 0xa0);
    for index in 0..count {
        let entry = e
            .mem
            .u32(e.mem.u32(this + 0x98).wrapping_add(index.wrapping_mul(8)));
        let vertices = e.mem.u32(entry + 0x20);
        let vertex_count = u32::from(e.mem.u16(entry + 8));
        for vertex in 0..vertex_count {
            let at = vertices.wrapping_add(12 * vertex);
            let scale = f64::from(x87_quiet(e.mem.f32(VERTEX_SCALE)));
            let scaled = [0, 4, 8]
                .map(|offset| (f64::from(x87_quiet(e.mem.f32(at + offset))) * scale) as f32);
            let factor = read_lanes(e, this + 0x80);
            let mut point = [
                factor[0] * scaled[0],
                factor[1] * scaled[1],
                factor[2] * scaled[2],
                factor[3] * 0.0,
            ];
            e.with_stack(0x20, |e, block| {
                write_lanes(e, block.addr(), point);
                e.call(
                    TRANSFORM_POINT,
                    &args![block.addr() + 0x10, matrix, block.addr()],
                );
                point = read_lanes(e, block.addr() + 0x10);
            });
            let lowest = read_lanes(e, this + 0x60);
            let highest = read_lanes(e, this + 0x70);
            write_lanes(
                e,
                this + 0x60,
                [0, 1, 2, 3].map(|lane| min_lane(lowest[lane], point[lane])),
            );
            write_lanes(
                e,
                this + 0x70,
                [0, 1, 2, 3].map(|lane| max_lane(highest[lane], point[lane])),
            );
        }
    }
}

// Translated from 00ca4280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4280 - the /GS cookie check 00ec408c is not modeled
/// Computes a bounding box (`bounds`: minimum at +0, maximum at +0x10) of the point lists
/// kept by `this` (ECX; the class is unconfirmed): `this + 0x98` is an array of 8-byte
/// entries (count at +0xa0) whose first word points to a record with a point array at +0x20
/// (12-byte points) and a 16-bit count at +8. Each point is scaled lane-wise by the vector
/// at `this + 0x80` (after a multiplication by the scale constant at `012682c8`), moved by
/// the transform `transform` through `00c7f640`, and folded into the box. When the float at
/// `this + 0x60` is not the largest float, the first step is a shortcut: if
/// `00cb2160(this + 0x20, transform, 0.001)` reports the vectors equal, the box is simply the
/// pair of vectors at `this + 0x60` and `this + 0x70`. Finally the box is grown by the float
/// at `this + 0x90` plus `margin` on every lane. The empty box starts at the two largest
/// floats. The stack protector check at the end is not modeled.
pub fn fn_00ca4280(e: &mut Engine, this: Ptr, transform: u32, margin: f32, bounds: Ptr) {
    let this = this.addr();
    let bounds = bounds.addr();
    let reference = e.mem.f32(this + 0x60);
    let largest = e.mem.f64(FLOAT_MAX_DOUBLE);
    let mut shortcut = false;
    if f64::from(reference) != largest {
        let tolerance = e.mem.u32(TOLERANCE);
        let equal = e.with_stack(4, |e, flag| {
            e.call(
                NEARLY_EQUAL,
                &args![this + 0x20, flag, transform, x87_quiet_bits(tolerance)],
            );
            e.mem.u8(flag.addr()) != 0
        });
        if equal {
            let low = read_lanes(e, this + 0x60);
            let high = read_lanes(e, this + 0x70);
            write_lanes(e, bounds, low);
            write_lanes(e, bounds + 0x10, high);
            shortcut = true;
        }
    }
    if !shortcut {
        let start_min = e.mem.f32(BOX_START_LOW);
        let start_max = e.mem.f32(BOX_START_HIGH);
        let zero = e.mem.f32(BOX_FOURTH_LANE);
        write_lanes(e, bounds, [start_min, start_min, start_min, zero]);
        write_lanes(e, bounds + 0x10, [start_max, start_max, start_max, zero]);
        let count = e.mem.u32(this + 0xa0);
        let mut index = 0u32;
        while index < count {
            let entry = e.mem.u32(e.mem.u32(this + 0x98) + index * 8);
            let mut point = e.mem.u32(entry + 0x20) + 8;
            let mut remaining = u32::from(e.mem.u16(entry + 8));
            while remaining != 0 {
                let scale = e.mem.f32(VERTEX_SCALE);
                let weights = read_lanes(e, this + 0x80);
                let scaled = [
                    (f64::from(e.mem.f32(point - 8)) * f64::from(scale)) as f32,
                    (f64::from(e.mem.f32(point - 4)) * f64::from(scale)) as f32,
                    (f64::from(scale) * f64::from(e.mem.f32(point))) as f32,
                    0.0,
                ];
                let product = [0, 1, 2, 3].map(|i| weights[i] * scaled[i]);
                e.with_stack(0x30, |e, local| {
                    let l = local.addr();
                    write_lanes(e, l, product);
                    e.call(TRANSFORM_POINT, &args![l + 0x10, transform, l]);
                    let world = read_lanes(e, l + 0x10);
                    let low = read_lanes(e, bounds);
                    let high = read_lanes(e, bounds + 0x10);
                    // MINPS / MAXPS: the second operand wins unless the first is strictly
                    // smaller / larger (so NaN lanes take the new point).
                    let low = [0, 1, 2, 3].map(|i| min_lane(low[i], world[i]));
                    let high = [0, 1, 2, 3].map(|i| max_lane(high[i], world[i]));
                    write_lanes(e, bounds, low);
                    write_lanes(e, bounds + 0x10, high);
                });
                point += 12;
                remaining -= 1;
            }
            index += 1;
        }
    }
    let grow = (f64::from(e.mem.f32(this + 0x90)) + f64::from(margin)) as f32;
    let low = read_lanes(e, bounds);
    let high = read_lanes(e, bounds + 0x10);
    write_lanes(e, bounds, low.map(|lane| lane - grow));
    write_lanes(e, bounds + 0x10, high.map(|lane| lane + grow));
}

// Translated from 00ca4440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca4440` (unnamed in the Xbox PDB; it has the shape of a scalar deleting destructor,
/// unconfirmed): stores the vtable 0x010c759c, releases the buffer at `+4` (`00aa10f0`) and,
/// when bit 0 of `flags` is set, frees the object itself with `MemoryManager::Deallocate`
/// (`00aa4060`). Answers `this`.
pub fn fn_00ca4440(e: &mut Engine, this: u32, flags: u32) -> u32 {
    let buffer = e.mem.u32(this + 4);
    e.mem.set_u32(this, RELEASE_VTABLE);
    e.call(RELEASE_BUFFER, &args![buffer]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca4120, fn_00ca4120(u32, u32)),
        entry!(0x00ca_4280, fn_00ca4280(Ptr, u32, f32, Ptr)),
        entry!(0x00ca4440, fn_00ca4440(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The transform double: the result is the vector unchanged.
    fn copy_vector(e: &mut Engine, a: &[u32]) -> Ret {
        let lanes = read_lanes(e, a[2]);
        write_lanes(e, a[0], lanes);
        Ret::default()
    }

    /// An engine with the constants mapped (scale 2, start floats +-1000, fourth lane 1) and an
    /// object whose `+0x80` vector is (1, 1, 1, 1), holding `entries` as (vertex count,
    /// vertices) pairs.
    fn setup(entries: &[(u16, Vec<[f32; 3]>)]) -> (Engine, u32, u32) {
        let mut e = logging_engine();
        for (address, value) in [
            (VERTEX_SCALE, 2.0f32),
            (BOX_START_LOW, 1000.0),
            (BOX_START_HIGH, -1000.0),
            (BOX_FOURTH_LANE, 1.0),
        ] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
        e.register(TRANSFORM_POINT, copy_vector);
        let this = e.mem.alloc(0x100);
        write_lanes(&mut e, this + 0x80, [1.0; 4]);
        let table = e.mem.alloc(8 * entries.len() as u32 + 8);
        e.mem.set_u32(this + 0x98, table);
        e.mem.set_u32(this + 0xa0, entries.len() as u32);
        for (i, (count, vertices)) in entries.iter().enumerate() {
            let entry = e.mem.alloc(0x40);
            let list = e.mem.alloc(12 * vertices.len() as u32 + 12);
            for (v, vertex) in vertices.iter().enumerate() {
                for (axis, value) in vertex.iter().enumerate() {
                    e.mem
                        .set_f32(list + 12 * v as u32 + 4 * axis as u32, *value);
                }
            }
            e.mem.set_u16(entry + 8, *count);
            e.mem.set_u32(entry + 0x20, list);
            e.mem.set_u32(table + 8 * i as u32, entry);
        }
        let matrix = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_f32(matrix + 4 * word, word as f32);
        }
        (e, this, matrix)
    }

    #[test]
    fn without_entries_only_the_matrix_and_the_start_corners_are_stored() {
        let (mut e, this, matrix) = setup(&[]);
        e.call(0x00ca_4120, &args![this, matrix]);
        for word in 0..16 {
            assert_eq!(e.mem.f32(this + 0x20 + 4 * word), word as f32);
        }
        assert_eq!(read_lanes(&e, this + 0x60), [1000.0, 1000.0, 1000.0, 1.0]);
        assert_eq!(
            read_lanes(&e, this + 0x70),
            [-1000.0, -1000.0, -1000.0, 1.0]
        );
        assert!(calls_to(&e, TRANSFORM_POINT).is_empty());
    }

    #[test]
    fn vertices_are_scaled_transformed_and_folded_into_the_corners() {
        let (mut e, this, matrix) = setup(&[
            (2, vec![[1.0, 2.0, 3.0], [-1.0, 5.0, 0.0]]),
            (0, vec![[100.0, 100.0, 100.0]]),
        ]);
        write_lanes(&mut e, this + 0x80, [1.0, 0.5, 1.0, 1.0]);
        e.call(0x00ca_4120, &args![this, matrix]);
        // Vertices become (2, 2, 6, 0) and (-2, 5, 0, 0); the empty entry adds nothing. The
        // fourth lane of the corners starts at 1 and meets 0.
        assert_eq!(read_lanes(&e, this + 0x60), [-2.0, 2.0, 0.0, 0.0]);
        assert_eq!(read_lanes(&e, this + 0x70), [2.0, 5.0, 6.0, 1.0]);
        let calls = calls_to(&e, TRANSFORM_POINT);
        assert_eq!(calls.len(), 2);
        assert!(calls.iter().all(|c| c[1] == matrix));
    }

    #[test]
    fn a_nan_vertex_lane_wins_both_corners() {
        let (mut e, this, matrix) = setup(&[(1, vec![[f32::NAN, 0.0, 0.0]])]);
        e.call(0x00ca_4120, &args![this, matrix]);
        assert!(e.mem.f32(this + 0x60).is_nan());
        assert!(e.mem.f32(this + 0x70).is_nan());
        assert_eq!(e.mem.f32(this + 0x64), 0.0);
    }

    #[test]
    fn destructor_stores_the_vtable_releases_the_buffer_and_frees_on_request() {
        let mut e = logging_engine();
        e.register(RELEASE_BUFFER, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, 0x7777);
        assert_eq!(e.call(0x00ca_4440, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RELEASE_VTABLE);
        assert_eq!(calls_to(&e, RELEASE_BUFFER), [[0x7777]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ca_4440, &args![this, 3u32]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [[MEMORY_MANAGER, this]]);
    }
}

#[cfg(test)]
mod bounds_tests {
    use super::*;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A box fed from one list of two points; the transform double adds 10 to every lane
    /// and the nearly-equal double reports `equal`.
    fn rig(equal: u8) -> (Engine, u32, u32) {
        let mut e = test_engine();
        e.mem.set_f32(BOX_START_LOW, 3.0e38);
        e.mem.set_f32(BOX_START_HIGH, -3.0e38);
        e.mem.set_f32(BOX_FOURTH_LANE, 0.0);
        e.mem.set_f32(VERTEX_SCALE, 2.0);
        e.mem.set_f64(FLOAT_MAX_DOUBLE, f64::from(f32::MAX));
        e.register_double(NEARLY_EQUAL, move |e, a| {
            e.mem.set_u8(a[1], equal);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(TRANSFORM_POINT, |e, a| {
            let v = read_lanes(e, a[2]);
            write_lanes(e, a[0], v.map(|lane| lane + 10.0));
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0x60, 1.0);
        write_lanes(&mut e, this + 0x60, [1.0, 2.0, 3.0, 4.0]);
        write_lanes(&mut e, this + 0x70, [5.0, 6.0, 7.0, 8.0]);
        write_lanes(&mut e, this + 0x80, [1.0, 1.0, 1.0, 1.0]);
        e.mem.set_f32(this + 0x90, 0.5);
        let points = e.mem.alloc(0x40);
        for (i, v) in [1.0f32, 2.0, 3.0, -1.0, 0.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(points + 4 * i as u32, v);
        }
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record + 0x20, points);
        e.mem.set_u16(record + 8, 2);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, record);
        e.mem.set_u32(this + 0x98, list);
        e.mem.set_u32(this + 0xa0, 1);
        let bounds = e.mem.alloc(0x20);
        (e, this, bounds)
    }

    #[test]
    fn folds_the_transformed_points_and_grows_the_box() {
        let (mut e, this, bounds) = rig(0);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        // points scaled by 2 then moved by 10: (12, 14, 16) and (8, 10, 20); grown by 0.75.
        assert_eq!(read_lanes(&e, bounds), [7.25, 9.25, 15.25, -0.75]);
        assert_eq!(read_lanes(&e, bounds + 0x10), [12.75, 14.75, 20.75, 10.75]);
    }

    #[test]
    fn the_shortcut_copies_the_stored_box() {
        let (mut e, this, bounds) = rig(1);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        assert_eq!(read_lanes(&e, bounds), [0.25, 1.25, 2.25, 3.25]);
        assert_eq!(read_lanes(&e, bounds + 0x10), [5.75, 6.75, 7.75, 8.75]);
        // The largest float skips the shortcut test altogether.
        let (mut e, this, bounds) = rig(1);
        e.mem.set_f32(this + 0x60, f32::MAX);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        assert_eq!(read_lanes(&e, bounds)[0], 7.25);
    }
}
