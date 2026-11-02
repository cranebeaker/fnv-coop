//! `Havok SDK/hkpgskbaseagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Timer labels: the start (`010cf22c`) and the end (`010c41f8`) of `staticGetClosestPoints`.
const LABEL_START: u32 = 0x010c_f22c;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkTransform::setMulInverseMul` (Xbox PDB, `00cb22f0`, `this` = result, `(a, b)`).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The two builders of the GSK cache (`00dafe70` when the second shape's type word at +0xc is
/// 3, else `00dafcd0`; `this` = the cache, `(shape a, shape b, transform)`; `RET 0xc`).
const GSK_INIT_TYPE_3: u32 = 0x00da_fe70;
const GSK_INIT_OTHER: u32 = 0x00da_fcd0;
/// The GSK solver (`this` = a local GSK object): `00daad40(shape a, shape b, transform,
/// result)` returns non-zero for no result, `00da9880(result, out)` turns a result into the
/// closest points and `00da9790(cache)` refreshes the cache.
const GSK_SOLVE: u32 = 0x00da_ad40;
const GSK_CLOSEST_POINTS: u32 = 0x00da_9880;
const GSK_REFRESH_CACHE: u32 = 0x00da_9790;
/// `hkVector4::setRotatedDir` (Xbox PDB name unconfirmed, `00c7f700`, `this` = result,
/// `(matrix, vector)`).
const ROTATE_VECTOR: u32 = 0x00c7_f700;
/// The `-1.0` float in the exe image (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte timer record `(label, time stamp)` to the thread's monitor stream while
/// its cursor (+4) is below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 come from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: [f32; 4], src: [f32; 4], imm: u32) -> [f32; 4] {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`: the low half of `src` becomes the high half of `dst`.
fn movlhps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`: the high half of `src` becomes the low half of `dst`.
fn movhlps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [src[2], src[3], dst[2], dst[3]]
}

// Translated from 00dabf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB; the vtable `010d51cc` it stores has the RTTI
/// name `hkpGskBaseAgent`). `this` is ECX; `body_a` and `body_b` are collision bodies (shape
/// at +0, motion at +8, parent chain at +0xc); `third` is stored at `this + 8`. Sets the
/// 16-bit word at +6 to 1, builds the relative transform of the two motions (`00cb22f0`) and
/// the GSK cache at `this + 0xc` (`00dafe70` when the second shape's word at +0xc is 3,
/// otherwise `00dafcd0`), clears the 16 bytes at +0x20, stores the `-1.0` constant at +0x2c and
/// +0x18, and puts at +0x1c the smaller of the floats at +0x4c of the root bodies of the two
/// chains (the second wins unless it is above the first, and when the compare is unordered).
/// Returns `this`.
pub fn hkp_gsk_base_agent_hkp_gsk_base_agent(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, third);
    e.mem.set_u32(t, 0x010d_51cc);
    let shape_b = e.mem.u32(body_b.addr());
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let shape_a = e.mem.u32(body_a.addr());
    e.with_stack(0x40, |e, transform| {
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let init = if e.mem.u32(shape_b + 0xc) == 3 {
            GSK_INIT_TYPE_3
        } else {
            GSK_INIT_OTHER
        };
        e.call(init, &args![t + 0xc, shape_a, shape_b, transform]);
    });
    let minus_one = x87_quiet(e.mem.f32(MINUS_ONE));
    set_lanes(e, t + 0x20, [0.0; 4]);
    e.mem.set_f32(t + 0x2c, minus_one);
    e.mem.set_f32(t + 0x18, minus_one);
    let mut root_a = body_a.addr();
    while e.mem.u32(root_a + 0xc) != 0 {
        root_a = e.mem.u32(root_a + 0xc);
    }
    let first = x87_quiet(e.mem.f32(root_a + 0x4c));
    let mut root_b = body_b.addr();
    while e.mem.u32(root_b + 0xc) != 0 {
        root_b = e.mem.u32(root_b + 0xc);
    }
    let second = x87_quiet(e.mem.f32(root_b + 0x4c));
    let smaller = if f64::from(second) > f64::from(first) {
        first
    } else {
        second
    };
    e.mem.set_f32(t + 0x1c, smaller);
    this
}

// Translated from 00dac450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::staticGetClosestPoints` (Xbox PDB), cdecl
/// `(body_a, body_b, input, collector)`; the roles are described from their use. Brackets the
/// work with timer records (start and end labels in the monitor stream). Builds the relative
/// transform of the two bodies' motions (`00cb22f0`), the GSK cache (`00dafe70` or `00dafcd0`,
/// as the constructor does) and asks both shapes (vtable offset `0x3c`) to convert the cache's
/// vertex ids into vertices; initialises a local GSK object from the cache's counts and
/// runs the solver (`00daad40`); when it succeeds, `00da9880` gives the closest points, which
/// are turned into body A's space (`00c7f700`). When the gap (the distance minus both shape
/// radii, the floats at +0x10 of the shapes) is below the float at `input + 8`, the contact
/// point and normal are put in body-local form, optionally refined by the second shape's
/// virtual function at offset `0x40` (only when its type word at +0xc is 3 and the word at
/// `input + 4` is set; a result of 0 drops the contact, 1 replaces the normal), and passed
/// to the collector's virtual function at offset 4 as a 16-float record (point, normal with
/// the gap in its last lane, the two bodies).
pub fn hkp_gsk_base_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    monitor_record(e, LABEL_START);
    let (a, b) = (body_a.addr(), body_b.addr());
    let motion_a = e.mem.u32(a + 8);
    let motion_b = e.mem.u32(b + 8);
    let shape_a = e.mem.u32(a);
    let shape_b = e.mem.u32(b);
    e.with_stack(0x290, |e, frame| {
        let x = frame.addr();
        e.mem.set_u32(x + 0xc, shape_a);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![x + 0x80, motion_a, motion_b],
        );
        let init = if e.mem.u32(shape_b + 0xc) == 3 {
            GSK_INIT_TYPE_3
        } else {
            GSK_INIT_OTHER
        };
        e.call(init, &args![x + 0x10, shape_a, shape_b, x + 0x80]);
        let count_a = u32::from(e.mem.u8(x + 0x18));
        let count_b = u32::from(e.mem.u8(x + 0x19));
        let flags = u32::from(e.mem.u8(x + 0x1a));
        e.mem.set_u32(x + 0x104, count_b);
        e.mem.set_u32(x + 0x108, flags & 0xf);
        e.mem.set_u32(x + 0x100, count_a);
        e.mem.set_u32(x + 0x10c, flags >> 4);
        e.mem.set_u32(x + 0x110, 0);
        e.mem.set_u32(x + 0x114, 0);
        e.vcall(shape_a, 0x3c, &args![x + 0x10, count_a, x + 0x120]);
        let second_ids = x + 0x10 + count_a * 2;
        e.vcall(shape_b, 0x3c, &args![second_ids, count_b, x + 0x1e0]);
        let failed = e
            .call(
                GSK_SOLVE,
                &args![x + 0x100, shape_a, shape_b, x + 0x80, x + 0xc0],
            )
            .u32();
        if failed == 0 {
            e.call(GSK_CLOSEST_POINTS, &args![x + 0x100, x + 0xc0, x + 0x30]);
            e.call(ROTATE_VECTOR, &args![x + 0x70, motion_a, x + 0x30]);
            let radius_a = e.mem.f32(shape_a + 0x10);
            let radius_b = e.mem.f32(shape_b + 0x10);
            let raw = e.mem.f32(x + 0x50);
            let gap = ((f64::from(raw) - f64::from(radius_a)) - f64::from(radius_b)) as f32;
            e.mem.set_f32(x + 0xc, gap);
            let gap = e.mem.f32(x + 0xc);
            e.mem.set_f32(x + 0x50, gap);
            let tolerance = e.mem.f32(input.addr() + 8);
            if f64::from(tolerance) > f64::from(gap) {
                emit_contact(
                    e,
                    x,
                    a,
                    b,
                    input.addr(),
                    collector.addr(),
                    shape_b,
                    gap,
                    radius_a,
                );
            }
        }
    });
    monitor_record(e, LABEL_END);
}

/// The tail of `staticGetClosestPoints` after the gap test (the part of the exe from
/// `00dac5e9`): `x` is the frame, `gap` the stored gap and `radius_a` the first shape's
/// radius.
#[allow(clippy::too_many_arguments)]
fn emit_contact(
    e: &mut Engine,
    x: u32,
    a: u32,
    b: u32,
    input: u32,
    collector: u32,
    shape_b: u32,
    gap: f32,
    radius_a: f32,
) {
    let normal = lanes(e, x + 0x30);
    let point = lanes(e, x + 0x40);
    let negative_radius = -x87_quiet(radius_a);
    let moved = [0, 1, 2, 3].map(|i| negative_radius * normal[i] + point[i]);
    set_lanes(e, x + 0x40, moved);
    let negative_gap = -gap;
    let local = [0, 1, 2, 3].map(|i| negative_gap * normal[i] + moved[i]);
    let offset = lanes(e, x + 0xb0);
    let delta = [0, 1, 2, 3].map(|i| local[i] - offset[i]);
    let (m0, m1, m2) = (lanes(e, x + 0x80), lanes(e, x + 0x90), lanes(e, x + 0xa0));
    let low = unpcklps(m0, m1);
    let high = unpckhps(m0, m1);
    let column0 = movlhps(low, m2);
    let column1 = shufps(movhlps(column0, low), m2, 0xd4);
    let column2 = shufps(high, m2, 0xe4);
    let by_y = shufps(delta, delta, 0x55);
    let by_x = shufps(delta, delta, 0x00);
    let by_z = shufps(delta, delta, 0xaa);
    let mut world = [0.0f32; 4];
    for i in 0..4 {
        world[i] = (by_y[i] * column1[i] + by_x[i] * column0[i]) + column2[i] * by_z[i];
    }
    set_lanes(e, x + 0x60, world);
    if e.mem.u32(shape_b + 0xc) == 3 && e.mem.u32(input + 4) != 0 {
        if e.mem.u32(x + 0x114) != 0 {
            e.call(GSK_REFRESH_CACHE, &args![x + 0x100, x + 0x10]);
        }
        let direction = lanes(e, x + 0x70);
        set_lanes(e, x + 0x20, direction);
        let second_motion = e.mem.u32(b + 8);
        let first_motion = e.mem.u32(a + 8);
        let count_a = u32::from(e.mem.u8(x + 0x18));
        let second_ids = x + 0x10 + count_a * 2;
        e.mem.set_u32(x + 0xc, second_motion);
        e.mem.set_u32(x + 0x1c, second_ids);
        let verdict = e
            .vcall(
                shape_b,
                0x40,
                &args![
                    second_ids,
                    x + 0x19,
                    x + 0x60,
                    second_motion,
                    e.mem.u32(a),
                    first_motion,
                    x + 0x20
                ],
            )
            .u32();
        if verdict == 0 {
            return;
        }
        if verdict == 1 {
            let refined = lanes(e, x + 0x20);
            set_lanes(e, x + 0x70, refined);
        }
    }
    let world = lanes(e, x + 0x60);
    let second_motion = e.mem.u32(b + 8);
    let rows = [
        lanes(e, second_motion),
        lanes(e, second_motion + 0x10),
        lanes(e, second_motion + 0x20),
        lanes(e, second_motion + 0x30),
    ];
    let spread_y = shufps(world, world, 0x55);
    let spread_x = shufps(world, world, 0x00);
    let spread_z = shufps(world, world, 0xaa);
    let mut moved_point = [0.0f32; 4];
    for i in 0..4 {
        let sum = (spread_y[i] * rows[1][i] + spread_x[i] * rows[0][i])
            + (spread_z[i] * rows[2][i] + rows[3][i]);
        moved_point[i] = sum;
    }
    set_lanes(e, x + 0xd0, moved_point);
    let normal = lanes(e, x + 0x70);
    set_lanes(e, x + 0xe0, normal);
    e.mem.set_f32(x + 0xec, x87_quiet(e.mem.f32(x + 0x50)));
    e.mem.set_u32(x + 0xf0, a);
    e.mem.set_u32(x + 0xf4, b);
    let table = e.mem.u32(collector);
    let callback = e.mem.u32(table + 4);
    e.call(callback, &args![collector, x + 0xd0]);
}

/// The labels of the penetration queries: `getPenetrations` writes a 16-byte record with
/// `010d5218` ("LtGsk") and a second label `010d5210` ("Sttim") at +0xc, then a 12-byte record
/// with `010d5204` ("StSepNormal"); its closing record is `01076f38`.
const LABEL_LT_GSK: u32 = 0x010d_5218;
const LABEL_STTIM: u32 = 0x010d_5210;
const LABEL_ST_SEP_NORMAL: u32 = 0x010d_5204;
const LABEL_PENETRATIONS_END: u32 = 0x0107_6f38;
/// The `0.0` float in the exe image (`01011d78`), the limit of `staticGetPenetrations`.
const ZERO_LIMIT: u32 = 0x0101_1d78;
/// `hkpGsk::getClosestPoint` (Xbox PDB, `00dabca0`), cdecl `(record, cache, out vector 2,
/// out vector 1)`; returns 0 when it found the points.
const GSK_GET_CLOSEST_POINT: u32 = 0x00da_bca0;

/// The 16-byte record `("LtGsk", time stamp, -, "Sttim")` of `getPenetrations`, appended while
/// the stream's cursor is below its limit; the cursor moves on by 0x10.
fn monitor_record_pair(e: &mut Engine) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, LABEL_LT_GSK);
        e.mem.set_u32(cursor + 0xc, LABEL_STTIM);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The three rows of a motion state's rotation (the 16-byte rows at +0, +0x10 and +0x20).
fn motion_rows(e: &Engine, motion: u32) -> [[f32; 4]; 3] {
    [
        lanes(e, motion),
        lanes(e, motion + 0x10),
        lanes(e, motion + 0x20),
    ]
}

/// `y * row1 + x * row0` plus `z * row2`, the single-precision `MULPS`/`ADDPS` sequence that
/// turns a vector into the rotated one.
fn rotate_by_rows(v: [f32; 4], rows: [[f32; 4]; 3]) -> [f32; 4] {
    [0usize, 1, 2, 3].map(|i| (v[1] * rows[1][i] + v[0] * rows[0][i]) + v[2] * rows[2][i])
}

/// The product with the transposed rotation, built in the exe from `UNPCKLPS`, `UNPCKHPS`,
/// `MOVLHPS`, `MOVHLPS` and `SHUFPS`.
fn rotate_by_columns(v: [f32; 4], rows: [[f32; 4]; 3]) -> [f32; 4] {
    let low = unpcklps(rows[0], rows[1]);
    let high = unpckhps(rows[0], rows[1]);
    let column0 = movlhps(low, rows[2]);
    let column1 = shufps(movhlps(column0, low), rows[2], 0xd4);
    let column2 = shufps(high, rows[2], 0xe4);
    let by_y = shufps(v, v, 0x55);
    let by_x = shufps(v, v, 0x00);
    let by_z = shufps(v, v, 0xaa);
    [0usize, 1, 2, 3].map(|i| (by_y[i] * column1[i] + by_x[i] * column0[i]) + column2[i] * by_z[i])
}

/// The distance between the shapes' surfaces: the solver's distance minus both radii, in
/// extended precision and stored as a float.
fn surface_gap(distance: f32, radius_a: f32, radius_b: f32) -> f32 {
    ((f64::from(distance) - f64::from(radius_a)) - f64::from(radius_b)) as f32
}

/// Fills the local GSK object the solver works on: the two vertex counts, the two halves of
/// the flag byte, a third word (1 or 0, per caller) and the cache-changed flag at +0x14 (0).
fn init_gsk_object(e: &mut Engine, object: u32, counts: (u32, u32), flags: u32, third: u32) {
    e.mem.set_u32(object, counts.0);
    e.mem.set_u32(object + 4, counts.1);
    e.mem.set_u32(object + 8, flags & 0xf);
    e.mem.set_u32(object + 0xc, flags >> 4);
    e.mem.set_u32(object + 0x10, third);
    e.mem.set_u32(object + 0x14, 0);
}

/// Asks both shapes (vtable offset `0x3c`) to turn the cache's vertex ids into vertices: the
/// first shape's ids start at `cache`, the second's follow after `count_a` ids of 2 bytes. The
/// counts are read back from the object, as the exe does.
fn convert_vertex_ids(
    e: &mut Engine,
    shapes: (u32, u32),
    cache: u32,
    object: u32,
    outputs: (u32, u32),
) {
    let first_count = e.mem.u32(object);
    e.vcall(shapes.0, 0x3c, &args![cache, first_count, outputs.0]);
    let count_b = e.mem.u32(object + 4);
    let count_a = e.mem.u32(object);
    e.vcall(
        shapes.1,
        0x3c,
        &args![cache + count_a * 2, count_b, outputs.1],
    );
}

// Translated from 00dabe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::processCollision` (Xbox PDB): takes `this` and four stack words (`RET
/// 0x10`) and does nothing.
pub fn hkp_gsk_base_agent_process_collision(
    _e: &mut Engine,
    _unused_this: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

// Translated from 00dabe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::cleanup` (Xbox PDB): with a non-null `this`, tail-calls the object's
/// virtual function at offset 0 (the scalar deleting destructor) with the word 1 in place of
/// the caller's stack word; otherwise returns.
pub fn hkp_gsk_base_agent_cleanup(e: &mut Engine, this: Ptr, _unused_2: u32) {
    if this.addr() != 0 {
        e.vcall(this.addr(), 0, &args![1u32]);
    }
}

// Translated from 00dabed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::warpTime` (Xbox PDB), `(this, old_time, new_time, third)`; the third
/// word is never read. When `old_time` equals the float at `this + 0x18` (an exact compare, so
/// a NaN is unequal), `new_time` replaces it; otherwise the float is reset to the `-1.0` of the
/// exe image (`01012054`) and the 16 bytes at `this + 0x20` are cleared.
pub fn hkp_gsk_base_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    old_time: f32,
    new_time: f32,
    _unused_4: u32,
) {
    let t = this.addr();
    if f64::from(old_time) == f64::from(e.mem.f32(t + 0x18)) {
        e.mem.set_f32(t + 0x18, x87_quiet(new_time));
    } else {
        let minus_one = x87_quiet(e.mem.f32(MINUS_ONE));
        e.mem.set_f32(t + 0x18, minus_one);
        set_lanes(e, t + 0x20, [0.0; 4]);
    }
}

// Translated from 00dac020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::calcSeparatingNormal` (Xbox PDB), cdecl `(body_a, body_b, third, gsk,
/// out)`; the third word is never read and `gsk` is the GSK object the solver (`00daad40`) runs
/// on. Builds the relative transform of the two bodies' motions (`00cb22f0`), runs the solver
/// (its return value is ignored) and writes into `out` the solver's vector rotated by body A's
/// motion, with the gap (the solver's distance minus both shapes' radii, the floats at +0x10)
/// in the last lane.
pub fn hkp_gsk_base_agent_calc_separating_normal(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_3: u32,
    gsk: Ptr,
    out: Ptr,
) {
    let (a, b) = (body_a.addr(), body_b.addr());
    let shape_b = e.mem.u32(b);
    let motion_b = e.mem.u32(b + 8);
    let motion_a = e.mem.u32(a + 8);
    let shape_a = e.mem.u32(a);
    e.with_stack(0x80, |e, frame| {
        let x = frame.addr();
        let (result, transform) = (x + 0x10, x + 0x20);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        e.call(
            GSK_SOLVE,
            &args![gsk.addr(), shape_a, shape_b, transform, result],
        );
        let distance = e.mem.f32(result + 0xc);
        let vector = lanes(e, result);
        let rows = motion_rows(e, e.mem.u32(a + 8));
        set_lanes(e, out.addr(), rotate_by_rows(vector, rows));
        let gap = surface_gap(
            distance,
            e.mem.f32(shape_a + 0x10),
            e.mem.f32(shape_b + 0x10),
        );
        e.mem.set_f32(out.addr() + 0xc, gap);
    });
}

// Translated from 00dac0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::getPenetrations` (Xbox PDB), `(this, body_a, body_b, third, collector)`;
/// the third word is never read. Brackets the work with timer records (`monitor_record_pair`,
/// the "StSepNormal" record and the closing `01076f38`). Builds the relative transform
/// (`00cb22f0`), a local GSK object from the cache at `this + 0xc` (counts at +0x14 and +0x15,
/// flag byte at +0x16), converts both shapes' vertex ids (vtable offset `0x3c`) and runs the
/// solver (`00daad40`); when it flags a changed cache (`object + 0x14`) the cache is refreshed
/// (`00da9790`). A failed solve clears the 16 bytes at `this + 0x20` and reports to the
/// collector's virtual function at offset 4 with the two bodies. A successful one stores the
/// rotated solver vector there, with the gap (distance minus both radii) in the last float, and
/// reports only when the gap is below 0.
pub fn hkp_gsk_base_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    _unused_4: u32,
    collector: Ptr,
) {
    let t = this.addr();
    monitor_record_pair(e);
    monitor_record(e, LABEL_ST_SEP_NORMAL);
    let (a, b) = (body_a.addr(), body_b.addr());
    let shape_b = e.mem.u32(b);
    let motion_b = e.mem.u32(b + 8);
    let motion_a = e.mem.u32(a + 8);
    let shape_a = e.mem.u32(a);
    e.with_stack(0x400, |e, frame| {
        let x = frame.addr();
        let (object, result, transform) = (x + 0x30, x + 0x20, x + 0x1b0);
        let cache = t + 0xc;
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let count_b = u32::from(e.mem.u8(t + 0x15));
        let count_a = u32::from(e.mem.u8(t + 0x14));
        let flags = u32::from(e.mem.u8(cache + 0xa));
        init_gsk_object(e, object, (count_a, count_b), flags, 1);
        convert_vertex_ids(e, (shape_a, shape_b), cache, object, (x + 0x50, x + 0x110));
        let failed = e
            .call(
                GSK_SOLVE,
                &args![object, shape_a, shape_b, transform, result],
            )
            .u32();
        if e.mem.u32(object + 0x14) != 0 {
            e.call(GSK_REFRESH_CACHE, &args![object, cache]);
        }
        if failed != 0 {
            set_lanes(e, t + 0x20, [0.0; 4]);
            e.vcall(collector.addr(), 4, &args![a, b]);
        } else {
            let vector = lanes(e, result);
            let gap = surface_gap(
                vector[3],
                e.mem.f32(shape_a + 0x10),
                e.mem.f32(shape_b + 0x10),
            );
            let rows = motion_rows(e, e.mem.u32(a + 8));
            set_lanes(e, t + 0x20, rotate_by_rows(vector, rows));
            e.mem.set_f32(t + 0x2c, gap);
            if f64::from(gap) < 0.0 {
                e.vcall(collector.addr(), 4, &args![a, b]);
            }
        }
    });
    monitor_record(e, LABEL_PENETRATIONS_END);
}

// Translated from 00dac2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::staticGetPenetrations` (Xbox PDB), cdecl `(body_a, body_b, third,
/// collector)`; the third word is never read. Brackets the work with timer records ("TtGsk",
/// `010cf22c`, and `010c41f8`). Builds the relative transform (`00cb22f0`), the GSK cache (`00dafe70`
/// when the second shape's type word at +0xc is 3, else `00dafcd0`) and a local GSK object from
/// it, converts both shapes' vertex ids (vtable offset `0x3c`) and runs the solver (`00daad40`).
/// A failed solve reports to the collector's virtual function at offset 4 with the two bodies;
/// a successful one reports only when the gap (the solver's distance minus both radii) is
/// below the float at `01011d78` (0.0 in the image).
pub fn hkp_gsk_base_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_3: u32,
    collector: Ptr,
) {
    monitor_record(e, LABEL_START);
    let (a, b) = (body_a.addr(), body_b.addr());
    let shape_b = e.mem.u32(b);
    let motion_b = e.mem.u32(b + 8);
    let motion_a = e.mem.u32(a + 8);
    let shape_a = e.mem.u32(a);
    let outcome = e.with_stack(0x400, |e, frame| {
        let x = frame.addr();
        let (cache, result, transform, object) = (x + 0x14, x + 0x20, x + 0x30, x + 0x70);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let init = if e.mem.u32(shape_b + 0xc) == 3 {
            GSK_INIT_TYPE_3
        } else {
            GSK_INIT_OTHER
        };
        e.call(init, &args![cache, shape_a, shape_b, transform]);
        let count_b = u32::from(e.mem.u8(cache + 9));
        let flags = u32::from(e.mem.u8(cache + 0xa));
        let count_a = u32::from(e.mem.u8(cache + 8));
        init_gsk_object(e, object, (count_a, count_b), flags, 1);
        convert_vertex_ids(e, (shape_a, shape_b), cache, object, (x + 0x90, x + 0x150));
        let failed = e
            .call(
                GSK_SOLVE,
                &args![object, shape_a, shape_b, transform, result],
            )
            .u32();
        (failed == 0).then(|| e.mem.f32(result + 0xc))
    });
    monitor_record(e, LABEL_END);
    let report = match outcome {
        None => true,
        Some(distance) => {
            let gap = surface_gap(
                distance,
                e.mem.f32(shape_a + 0x10),
                e.mem.f32(shape_b + 0x10),
            );
            f64::from(gap) < f64::from(e.mem.f32(ZERO_LIMIT))
        }
    };
    if report {
        e.vcall(collector.addr(), 4, &args![a, b]);
    }
}

// Translated from 00dac7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::getClosestPoint` (Xbox PDB), `(this, found, body_a, body_b, input, out)`:
/// `found` is a byte the function sets, `input` has the distance limit at +8 and `out` receives
/// the closest-point record. Builds the relative transform (`00cb22f0`) and a local GSK object
/// from the cache at `this + 0xc` (counts at +0x14 and +0x15, flag byte at +0x16), converts
/// both shapes' vertex ids (vtable offset `0x3c`) and runs the solver (`00daad40`, its return
/// value ignored); a changed-cache flag (`object + 0x14`) refreshes the cache (`00da9790`).
/// `00da9880` turns the solver result into the record at `out`: the vector at +0 is rotated
/// by body A's motion into `out + 0x40`, and the float at `out + 0x20` becomes the gap (minus
/// both radii). When the gap is below the limit the point at `out + 0x10` is pulled back by
/// the first radius along the vector, moved to the second body's space by the transpose of the
/// relative rotation (stored at `out + 0x30`) and `found` is 1; otherwise `found` is 0.
pub fn hkp_gsk_base_agent_get_closest_point(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    out: Ptr,
) {
    let t = this.addr();
    let (a, b) = (body_a.addr(), body_b.addr());
    let out = out.addr();
    let shape_a = e.mem.u32(a);
    let motion_a = e.mem.u32(a + 8);
    let shape_b = e.mem.u32(b);
    let motion_b = e.mem.u32(b + 8);
    e.with_stack(0x400, |e, frame| {
        let x = frame.addr();
        let (result, transform, object) = (x + 0x10, x + 0x20, x + 0x60);
        let cache = t + 0xc;
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let count_b = u32::from(e.mem.u8(t + 0x15));
        let flags = u32::from(e.mem.u8(t + 0x16));
        let count_a = u32::from(e.mem.u8(t + 0x14));
        init_gsk_object(e, object, (count_a, count_b), flags, 0);
        convert_vertex_ids(e, (shape_a, shape_b), cache, object, (x + 0x80, x + 0x140));
        e.call(
            GSK_SOLVE,
            &args![object, shape_a, shape_b, transform, result],
        );
        if e.mem.u32(object + 0x14) != 0 {
            e.call(GSK_REFRESH_CACHE, &args![object, cache]);
        }
        e.call(GSK_CLOSEST_POINTS, &args![object, result, out]);
        let normal = lanes(e, out);
        let rows = motion_rows(e, e.mem.u32(a + 8));
        set_lanes(e, out + 0x40, rotate_by_rows(normal, rows));
        let gap = surface_gap(
            e.mem.f32(out + 0x20),
            e.mem.f32(shape_a + 0x10),
            e.mem.f32(shape_b + 0x10),
        );
        e.mem.set_f32(out + 0x20, gap);
        if f64::from(e.mem.f32(input.addr() + 8)) > f64::from(gap) {
            let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
            let point = lanes(e, out + 0x10);
            let pulled = [0usize, 1, 2, 3].map(|i| point[i] - radius_a * normal[i]);
            set_lanes(e, out + 0x10, pulled);
            let negative_gap = -gap;
            let offset = lanes(e, transform + 0x30);
            let delta =
                [0usize, 1, 2, 3].map(|i| (negative_gap * normal[i] + pulled[i]) - offset[i]);
            let rotation = [
                lanes(e, transform),
                lanes(e, transform + 0x10),
                lanes(e, transform + 0x20),
            ];
            set_lanes(e, out + 0x30, rotate_by_columns(delta, rotation));
            e.mem.set_u8(found.addr(), 1);
        } else {
            e.mem.set_u8(found.addr(), 0);
        }
    });
}

// Translated from 00dac9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::getClosestPoints` (Xbox PDB), `(this, body_a, body_b, input, collector)`:
/// `input` has a float at +8 that is copied into the query record. Brackets the work with
/// timer records (`010cf22c` and `010c41f8`). Builds the relative transform (`00cb22f0`) and a
/// query record `(transform, motion of body A, shape A, shape B, the float)` and calls
/// `hkpGsk::getClosestPoint` (`00dabca0`) with it, the cache at `this + 0xc` and two output
/// vectors. On success the two vectors and the two bodies are put into a 0x28-byte record
/// that goes to the collector's virtual function at offset 4.
pub fn hkp_gsk_base_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    monitor_record(e, LABEL_START);
    let (a, b) = (body_a.addr(), body_b.addr());
    let motion_b = e.mem.u32(b + 8);
    let motion_a = e.mem.u32(a + 8);
    e.with_stack(0x100, |e, frame| {
        let x = frame.addr();
        let transform = x + 0x80;
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let shape_a = e.mem.u32(a);
        let shape_b = e.mem.u32(b);
        e.mem.set_u32(x + 0x24, shape_a);
        e.mem.set_u32(x + 0x1c, transform);
        e.mem.set_u32(x + 0x28, shape_b);
        let tolerance = x87_quiet(e.mem.f32(input.addr() + 8));
        e.mem.set_u32(x + 0x20, e.mem.u32(a + 8));
        e.mem.set_f32(x + 0x2c, tolerance);
        let missed = e
            .call(
                GSK_GET_CLOSEST_POINT,
                &args![x + 0x1c, this.addr() + 0xc, x + 0x40, x + 0x30],
            )
            .u32();
        if missed == 0 {
            for i in 0..4 {
                let first = e.mem.u32(x + 0x30 + 4 * i);
                e.mem.set_u32(x + 0x50 + 4 * i, first);
            }
            for i in 0..4 {
                let second = e.mem.u32(x + 0x40 + 4 * i);
                e.mem.set_u32(x + 0x60 + 4 * i, second);
            }
            e.mem.set_u32(x + 0x70, a);
            e.mem.set_u32(x + 0x74, b);
            e.vcall(collector.addr(), 4, &args![x + 0x50]);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00dabeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `this` in `ECX` and one stack word that is never read (`RET 4`,
/// `_unused_0`). Stores the float constant at `01012054` (through `FLD` / `FSTP`) at +0x18 and
/// zeroes the 16 bytes at +0x20. Ghidra holds it in the body of `00d33770`, whose `+0x84`
/// branch jumps here; it is also a vtable slot of `hkpPredGskfAgent` (`010cebb8`, slot 9).
pub fn fn_00dabeb0(e: &mut Engine, this: u32, _unused_0: u32) {
    let value: u32 = e.global(0x0101_2054);
    e.mem.set_u32(this + 0x18, x87_quiet_bits(value));
    e.mem.write(this + 0x20, &[0u8; 16]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_bf10, hkp_gsk_base_agent_hkp_gsk_base_agent(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00da_c450,
            hkp_gsk_base_agent_static_get_closest_points(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00da_be80,
            hkp_gsk_base_agent_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00da_be90, hkp_gsk_base_agent_cleanup(Ptr, u32)),
        entry!(
            0x00da_bed0,
            hkp_gsk_base_agent_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(
            0x00da_c020,
            hkp_gsk_base_agent_calc_separating_normal(Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00da_c0b0,
            hkp_gsk_base_agent_get_penetrations(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00da_c2b0,
            hkp_gsk_base_agent_static_get_penetrations(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00da_c7f0,
            hkp_gsk_base_agent_get_closest_point(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00da_c9e0,
            hkp_gsk_base_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00da_beb0, fn_00dabeb0(u32, u32)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// A body: shape at +0 (type word at shape + 0xc), motion at +8, parent at +0xc and the
    /// float at +0x4c.
    fn body(e: &mut Engine, shape_type: u32, motion: u32, parent: u32, value: f32) -> u32 {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, shape_type);
        let b = e.mem.alloc(0x60);
        e.mem.set_u32(b, shape);
        e.mem.set_u32(b + 8, motion);
        e.mem.set_u32(b + 0xc, parent);
        e.mem.set_f32(b + 0x4c, value);
        b
    }

    #[test]
    fn the_constructor_builds_the_cache_and_takes_the_smaller_root_value() {
        let mut e = test_engine();
        e.mem.set_f32(MINUS_ONE, -1.0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, GSK_INIT_TYPE_3, 0);
        log_double(&mut e, &log, GSK_INIT_OTHER, 0);
        let root_a = body(&mut e, 1, 0x100, 0, 5.0);
        let a = body(&mut e, 1, 0x1000, root_a, 99.0);
        let root_b = body(&mut e, 1, 0x200, 0, 2.0);
        let b = body(&mut e, 3, 0x2000, root_b, 99.0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, 0xdead);
        let got = e.call(0x00da_bf10, &args![this, a, b, 0x77u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_51cc);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        assert_eq!(lanes(&e, this + 0x20), [0.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(this + 0x18), -1.0);
        // The second root's value (2.0) is below the first's (5.0).
        assert_eq!(e.mem.f32(this + 0x1c), 2.0);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1..], [0x1000, 0x2000]);
        // The second shape has type 3, so the type-3 builder runs with (cache, shape a, shape b).
        assert_eq!(calls[1].0, GSK_INIT_TYPE_3);
        assert_eq!(calls[1].1[0], this + 0xc);
        assert_eq!(calls[1].1[1], e.mem.u32(a));
        assert_eq!(calls[1].1[2], e.mem.u32(b));
        assert_eq!(calls.len(), 2);
        // With the first value smaller, it wins instead.
        drop(calls);
        e.mem.set_f32(root_a + 0x4c, 1.0);
        let other = body(&mut e, 4, 0x2000, root_b, 0.0);
        e.call(0x00da_bf10, &args![this, a, other, 0x77u32]);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        assert_eq!(log.borrow().last().unwrap().0, GSK_INIT_OTHER);
    }

    /// Doubles for everything `staticGetClosestPoints` calls; returns the collector log.
    fn closest_points_rig(solver_result: u32) -> (Engine, Log, u32, u32, u32, u32) {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        // Monitor stream: a record buffer.
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.set_global(MONITOR_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        // The relative transform: identity rows, zero translation.
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, |e, a| {
            let rows = [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ];
            for (i, row) in rows.into_iter().enumerate() {
                set_lanes(e, a[0] + 0x10 * i as u32, row);
            }
            Ret::default()
        });
        // The cache builder: one id each side.
        e.register_double(GSK_INIT_OTHER, |e, a| {
            e.mem.set_u8(a[0] + 8, 1);
            e.mem.set_u8(a[0] + 9, 1);
            e.mem.set_u8(a[0] + 0xa, 0);
            Ret::default()
        });
        log_double(&mut e, &log, GSK_SOLVE, solver_result);
        e.register_double(GSK_CLOSEST_POINTS, |e, a| {
            set_lanes(e, a[2], [0.0, 0.0, 1.0, 0.0]);
            set_lanes(e, a[2] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(a[2] + 0x20, 0.5);
            Ret::default()
        });
        e.register_double(ROTATE_VECTOR, |e, a| {
            let v = lanes(e, a[2]);
            set_lanes(e, a[0], v);
            Ret::default()
        });
        // Shape vtables with the vertex conversion at 0x3c.
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x3c, 0x0900_003c);
        log_double(&mut e, &log, 0x0900_003c, 0);
        let make_shape = |e: &mut Engine, radius: f32| {
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, vtable);
            e.mem.set_f32(shape + 0x10, radius);
            e.mem.set_u32(shape + 0xc, 1);
            shape
        };
        let shape_a = make_shape(&mut e, 0.1);
        let shape_b = make_shape(&mut e, 0.2);
        let motion_a = e.mem.alloc(0x40);
        let motion_b = e.mem.alloc(0x40);
        for (i, row) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 0.0, 0.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            set_lanes(&mut e, motion_b + 0x10 * i as u32, row);
        }
        let a = e.mem.alloc(0x20);
        e.mem.set_u32(a, shape_a);
        e.mem.set_u32(a + 8, motion_a);
        let b = e.mem.alloc(0x20);
        e.mem.set_u32(b, shape_b);
        e.mem.set_u32(b + 8, motion_b);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 1.0);
        // The collector records the point record it receives.
        let collector_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(collector_vtable + 4, 0x0900_0004);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, collector_vtable);
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..16).map(|i| e.mem.u32(a[1] + 4 * i)));
            sink.borrow_mut().push((0x0900_0004, words));
            Ret::default()
        });
        (e, log, a, b, input, collector)
    }

    #[test]
    fn a_close_pair_reports_a_contact_point() {
        let (mut e, log, a, b, input, collector) = closest_points_rig(0);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        let calls = log.borrow();
        let record = calls.iter().find(|c| c.0 == 0x0900_0004).unwrap();
        let floats: Vec<f32> = record.1[2..].iter().map(|w| f32::from_bits(*w)).collect();
        // gap = 0.5 - 0.1 - 0.2; the world point is the point moved back by radius and gap
        // along the normal, then placed by body b's motion (translation 10 on x).
        let gap = ((f64::from(0.5f32) - f64::from(0.1f32)) - f64::from(0.2f32)) as f32;
        assert!((floats[0] - 11.0).abs() < 1e-5);
        assert!((floats[1] - 2.0).abs() < 1e-5);
        assert!((floats[2] - (3.0 - 0.1 - gap)).abs() < 1e-5);
        // The normal and the gap in the last lane.
        assert_eq!(floats[6], 1.0);
        assert_eq!(floats[7], gap);
        // The two bodies.
        assert_eq!(record.1[2 + 8], a);
        assert_eq!(record.1[2 + 9], b);
        // The monitor stream got two records (start and end labels).
        drop(calls);
        let stream = e.call(TLS_GET_VALUE, &args![3u32]).u32();
        let cursor = e.mem.u32(stream + 4);
        assert_eq!(cursor + 0x100 - e.mem.u32(stream + 0xc), 24);
    }

    #[test]
    fn a_failed_solve_or_a_wide_gap_reports_nothing() {
        let (mut e, log, a, b, input, collector) = closest_points_rig(1);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        assert!(log.borrow().iter().all(|c| c.0 != 0x0900_0004));
        // A tolerance below the gap drops the contact too.
        let (mut e, log, a, b, input, collector) = closest_points_rig(0);
        e.mem.set_f32(input + 8, 0.1);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        assert!(log.borrow().iter().all(|c| c.0 != 0x0900_0004));
    }
}

#[cfg(test)]
mod havok_agent_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SHAPE_VERTEX_CONVERSION: u32 = 0x0900_003c;
    const COLLECTOR_SLOT_4: u32 = 0x0900_0004;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// The argument lists of the logged calls to `address`, in order.
    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Bytes written to the monitor stream so far.
    fn monitor_used(e: &mut Engine) -> u32 {
        let stream = e.call(TLS_GET_VALUE, &args![3u32]).u32();
        e.mem.u32(stream + 4) + 0x100 - e.mem.u32(stream + 0xc)
    }

    /// The engine with doubles for everything the penetration and closest-point queries call.
    /// The solver returns `solver_ret`, writes the vector `[1, 2, 3, distance]` into its result
    /// and sets the cache-changed flag of its object to `cache_changed`. Returns the engine, the
    /// log, the two bodies and the collector. Body A has a motion that scales y by 2 and z by 3;
    /// the shape of body A has radius 0.1 and that of body B 0.2.
    fn rig(solver_ret: u32, distance: f32, cache_changed: bool) -> (Engine, Log, u32, u32, u32) {
        let mut e = test_engine();
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(ZERO_LIMIT, 0.0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.set_global(MONITOR_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, move |e, a| {
            sink.borrow_mut()
                .push((TRANSFORM_SET_MUL_INVERSE_MUL, a.to_vec()));
            let rows = [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.5, 0.0, 0.0, 0.0],
            ];
            for (i, row) in rows.into_iter().enumerate() {
                set_lanes(e, a[0] + 0x10 * i as u32, row);
            }
            Ret::default()
        });
        for init in [GSK_INIT_TYPE_3, GSK_INIT_OTHER] {
            let sink = Rc::clone(&log);
            e.register_double(init, move |e, a| {
                sink.borrow_mut().push((init, a.to_vec()));
                e.mem.set_u8(a[0] + 8, 2);
                e.mem.set_u8(a[0] + 9, 3);
                e.mem.set_u8(a[0] + 0xa, 0x53);
                Ret::default()
            });
        }
        log_double(&mut e, &log, GSK_REFRESH_CACHE, 0);
        log_double(&mut e, &log, SHAPE_VERTEX_CONVERSION, 0);
        let sink = Rc::clone(&log);
        e.register_double(GSK_SOLVE, move |e, a| {
            sink.borrow_mut().push((GSK_SOLVE, a.to_vec()));
            set_lanes(e, a[4], [1.0, 2.0, 3.0, distance]);
            e.mem.set_u32(a[0] + 0x14, u32::from(cache_changed));
            Ret {
                eax: solver_ret,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x3c, SHAPE_VERTEX_CONVERSION);
        let make_body = |e: &mut Engine, radius: f32, motion: u32| {
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, vtable);
            e.mem.set_f32(shape + 0x10, radius);
            e.mem.set_u32(shape + 0xc, 1);
            let body = e.mem.alloc(0x40);
            e.mem.set_u32(body, shape);
            e.mem.set_u32(body + 8, motion);
            body
        };
        let motion_a = e.mem.alloc(0x40);
        for (i, row) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            set_lanes(&mut e, motion_a + 0x10 * i as u32, row);
        }
        let motion_b = e.mem.alloc(0x40);
        let a = make_body(&mut e, 0.1, motion_a);
        let b = make_body(&mut e, 0.2, motion_b);
        let collector_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(collector_vtable + 4, COLLECTOR_SLOT_4);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, collector_vtable);
        log_double(&mut e, &log, COLLECTOR_SLOT_4, 0);
        (e, log, a, b, collector)
    }

    /// An agent object with the cache bytes (2 ids, 3 ids, flags 0x53) the queries read.
    fn agent(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u8(this + 0x14, 2);
        e.mem.set_u8(this + 0x15, 3);
        e.mem.set_u8(this + 0x16, 0x53);
        this
    }

    fn gap_of(distance: f32) -> f32 {
        ((f64::from(distance) - f64::from(0.1f32)) - f64::from(0.2f32)) as f32
    }

    #[test]
    fn process_collision_does_nothing() {
        let mut e = test_engine();
        e.call_log = Some(Vec::new());
        e.call(0x00da_be80, &args![0x1234u32, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn cleanup_calls_the_destructor_with_one_unless_null() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x0900_0000, 0);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0000);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.call(0x00da_be90, &args![0u32, 7u32]);
        assert!(log.borrow().is_empty());
        e.call(0x00da_be90, &args![this, 7u32]);
        assert_eq!(calls(&log, 0x0900_0000), vec![vec![this, 1]]);
    }

    #[test]
    fn warp_time_keeps_the_new_time_only_when_the_old_one_matches() {
        let mut e = test_engine();
        e.mem.set_f32(MINUS_ONE, -1.0);
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x18, 2.5);
        set_lanes(&mut e, this + 0x20, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00da_bed0, &args![this, 2.5f32, 7.0f32, 0u32]);
        assert_eq!(e.mem.f32(this + 0x18), 7.0);
        assert_eq!(lanes(&e, this + 0x20), [1.0, 2.0, 3.0, 4.0]);
        // A different old time resets the float to -1.0 and clears the vector.
        e.call(0x00da_bed0, &args![this, 2.5f32, 9.0f32, 0u32]);
        assert_eq!(e.mem.f32(this + 0x18), -1.0);
        assert_eq!(lanes(&e, this + 0x20), [0.0; 4]);
        // A NaN never matches.
        e.mem.set_f32(this + 0x18, f32::NAN);
        set_lanes(&mut e, this + 0x20, [1.0; 4]);
        e.call(0x00da_bed0, &args![this, f32::NAN, 9.0f32, 0u32]);
        assert_eq!(e.mem.f32(this + 0x18), -1.0);
        assert_eq!(lanes(&e, this + 0x20), [0.0; 4]);
    }

    #[test]
    fn calc_separating_normal_rotates_the_solver_vector_and_stores_the_gap() {
        let (mut e, log, a, b, _) = rig(5, 0.5, false);
        let gsk = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        e.mem.set_u32(out + 0xc, 0x7777);
        e.call(0x00da_c020, &args![a, b, 0u32, gsk, out]);
        let solves = calls(&log, GSK_SOLVE);
        assert_eq!(solves.len(), 1);
        assert_eq!(solves[0][0], gsk);
        assert_eq!(solves[0][1], e.mem.u32(a));
        assert_eq!(solves[0][2], e.mem.u32(b));
        let transforms = calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(transforms[0][1..], [e.mem.u32(a + 8), e.mem.u32(b + 8)]);
        assert_eq!(solves[0][3], transforms[0][0]);
        let out_lanes = lanes(&e, out);
        assert_eq!(out_lanes[..3], [1.0, 4.0, 9.0]);
        assert_eq!(out_lanes[3], gap_of(0.5));
    }

    #[test]
    fn get_penetrations_reports_a_failed_solve_and_clears_the_normal() {
        let (mut e, log, a, b, collector) = rig(1, 0.5, true);
        let this = agent(&mut e);
        set_lanes(&mut e, this + 0x20, [9.0; 4]);
        e.call(0x00da_c0b0, &args![this, a, b, 0u32, collector]);
        assert_eq!(lanes(&e, this + 0x20), [0.0; 4]);
        assert_eq!(calls(&log, COLLECTOR_SLOT_4), vec![vec![collector, a, b]]);
        // The cache was flagged as changed, so it is refreshed with the object and the cache.
        let refreshes = calls(&log, GSK_REFRESH_CACHE);
        assert_eq!(refreshes.len(), 1);
        assert_eq!(refreshes[0][1], this + 0xc);
        // The object has the counts 2 and 3, the flag halves 3 and 5 and the word 1.
        let solves = calls(&log, GSK_SOLVE);
        assert_eq!(refreshes[0][0], solves[0][0]);
        let object = solves[0][0];
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(object + 4 * i)).collect();
        assert_eq!(words, [2, 3, 3, 5, 1, 1]);
        // Both shapes convert their ids: the second's follow the first's two 2-byte ids.
        let conversions = calls(&log, SHAPE_VERTEX_CONVERSION);
        assert_eq!(conversions.len(), 2);
        assert_eq!(conversions[0][..3], [e.mem.u32(a), this + 0xc, 2]);
        assert_eq!(conversions[1][..3], [e.mem.u32(b), this + 0xc + 4, 3]);
        // Two timer records of 0x10 and 0xc bytes, then the closing one.
        assert_eq!(monitor_used(&mut e), 0x10 + 0xc + 0xc);
        let stream = e.call(TLS_GET_VALUE, &args![3u32]).u32();
        let first = e.mem.u32(stream + 4) - 0x28;
        assert_eq!(e.mem.u32(first), LABEL_LT_GSK);
        assert_eq!(e.mem.u32(first + 0xc), LABEL_STTIM);
        assert_eq!(e.mem.u32(first + 0x10), LABEL_ST_SEP_NORMAL);
        assert_eq!(e.mem.u32(first + 0x1c), LABEL_PENETRATIONS_END);
    }

    #[test]
    fn get_penetrations_reports_a_successful_solve_only_below_zero() {
        // A positive gap stores the normal and the gap and reports nothing.
        let (mut e, log, a, b, collector) = rig(0, 0.5, false);
        let this = agent(&mut e);
        e.call(0x00da_c0b0, &args![this, a, b, 0u32, collector]);
        assert!(calls(&log, COLLECTOR_SLOT_4).is_empty());
        assert!(calls(&log, GSK_REFRESH_CACHE).is_empty());
        let stored = lanes(&e, this + 0x20);
        assert_eq!(stored[..3], [1.0, 4.0, 9.0]);
        assert_eq!(stored[3], gap_of(0.5));
        // A negative gap reports.
        let (mut e, log, a, b, collector) = rig(0, 0.2, false);
        let this = agent(&mut e);
        e.call(0x00da_c0b0, &args![this, a, b, 0u32, collector]);
        assert_eq!(calls(&log, COLLECTOR_SLOT_4), vec![vec![collector, a, b]]);
        assert_eq!(e.mem.f32(this + 0x2c), gap_of(0.2));
        // A NaN gap does not report: the compare is ordered.
        let (mut e, log, a, b, collector) = rig(0, f32::NAN, false);
        let this = agent(&mut e);
        e.call(0x00da_c0b0, &args![this, a, b, 0u32, collector]);
        assert!(calls(&log, COLLECTOR_SLOT_4).is_empty());
    }

    #[test]
    fn static_get_penetrations_builds_the_cache_and_reports_by_gap() {
        // A failed solve reports; the second shape's type word 1 picks the general builder.
        let (mut e, log, a, b, collector) = rig(1, 0.5, false);
        e.call(0x00da_c2b0, &args![a, b, 0u32, collector]);
        assert_eq!(calls(&log, COLLECTOR_SLOT_4), vec![vec![collector, a, b]]);
        assert_eq!(calls(&log, GSK_INIT_OTHER).len(), 1);
        assert!(calls(&log, GSK_INIT_TYPE_3).is_empty());
        let init = &calls(&log, GSK_INIT_OTHER)[0];
        let cache = init[0];
        let solves = calls(&log, GSK_SOLVE);
        let object = solves[0][0];
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(object + 4 * i)).collect();
        assert_eq!(words, [2, 3, 3, 5, 1, 0]);
        let conversions = calls(&log, SHAPE_VERTEX_CONVERSION);
        assert_eq!(conversions[0][..3], [e.mem.u32(a), cache, 2]);
        assert_eq!(conversions[1][..3], [e.mem.u32(b), cache + 4, 3]);
        assert_eq!(monitor_used(&mut e), 0x18);
        // A positive gap does not report; a second shape of type 3 picks the other builder.
        let (mut e, log, a, b, collector) = rig(0, 0.5, false);
        let shape_b = e.mem.u32(b);
        e.mem.set_u32(shape_b + 0xc, 3);
        e.call(0x00da_c2b0, &args![a, b, 0u32, collector]);
        assert!(calls(&log, COLLECTOR_SLOT_4).is_empty());
        assert_eq!(calls(&log, GSK_INIT_TYPE_3).len(), 1);
        assert!(calls(&log, GSK_INIT_OTHER).is_empty());
        // A negative gap reports.
        let (mut e, log, a, b, collector) = rig(0, 0.2, false);
        e.call(0x00da_c2b0, &args![a, b, 0u32, collector]);
        assert_eq!(calls(&log, COLLECTOR_SLOT_4).len(), 1);
        // The limit is the float in the image: with 1.0 a gap of 0.2 reports.
        let (mut e, log, a, b, collector) = rig(0, 0.5, false);
        e.mem.set_f32(ZERO_LIMIT, 1.0);
        e.call(0x00da_c2b0, &args![a, b, 0u32, collector]);
        assert_eq!(calls(&log, COLLECTOR_SLOT_4).len(), 1);
    }

    /// The rig with the double of the converter that fills the closest-point record.
    fn closest_point_rig(cache_changed: bool) -> (Engine, Log, u32, u32) {
        let (mut e, log, a, b, _) = rig(0, 0.5, cache_changed);
        let sink = Rc::clone(&log);
        e.register_double(GSK_CLOSEST_POINTS, move |e, args| {
            sink.borrow_mut().push((GSK_CLOSEST_POINTS, args.to_vec()));
            set_lanes(e, args[2], [0.0, 0.0, 1.0, 0.0]);
            set_lanes(e, args[2] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(args[2] + 0x20, 0.5);
            Ret::default()
        });
        (e, log, a, b)
    }

    #[test]
    fn get_closest_point_builds_the_record_when_the_gap_is_below_the_limit() {
        let (mut e, log, a, b) = closest_point_rig(true);
        let this = agent(&mut e);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 1.0);
        let found = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x80);
        e.mem.set_u8(found, 0xaa);
        e.call(0x00da_c7f0, &args![this, found, a, b, input, out]);
        assert_eq!(e.mem.u8(found), 1);
        assert_eq!(calls(&log, GSK_REFRESH_CACHE).len(), 1);
        let gap = gap_of(0.5);
        assert_eq!(e.mem.f32(out + 0x20), gap);
        // The normal (0, 0, 1) rotated by body A's motion.
        assert_eq!(lanes(&e, out + 0x40), [0.0, 0.0, 3.0, 0.0]);
        // The point is pulled back by the first radius, then by the gap, and shifted by the
        // transform's translation (0.5 on x); the rotation is the identity.
        assert_eq!(lanes(&e, out + 0x10), [1.0, 2.0, 3.0 - 0.1, 0.0]);
        let local = lanes(&e, out + 0x30);
        assert!((local[0] - 0.5).abs() < 1e-6);
        assert!((local[1] - 2.0).abs() < 1e-6);
        assert!((local[2] - (3.0 - 0.1 - gap)).abs() < 1e-5);
        let solves = calls(&log, GSK_SOLVE);
        let object = solves[0][0];
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(object + 4 * i)).collect();
        assert_eq!(words, [2, 3, 3, 5, 0, 1]);
        assert_eq!(
            calls(&log, GSK_CLOSEST_POINTS)[0][..3],
            [object, solves[0][4], out]
        );
    }

    #[test]
    fn get_closest_point_reports_nothing_at_or_above_the_limit() {
        let (mut e, log, a, b) = closest_point_rig(false);
        let this = agent(&mut e);
        let input = e.mem.alloc(0x20);
        // The gap is 0.2 and the limit equals it: not below, so not found.
        e.mem.set_f32(input + 8, gap_of(0.5));
        let found = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x80);
        e.mem.set_u8(found, 0xaa);
        e.call(0x00da_c7f0, &args![this, found, a, b, input, out]);
        assert_eq!(e.mem.u8(found), 0);
        assert!(calls(&log, GSK_REFRESH_CACHE).is_empty());
        assert_eq!(e.mem.f32(out + 0x20), gap_of(0.5));
        assert_eq!(lanes(&e, out + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x30), [0.0; 4]);
    }

    #[test]
    fn get_closest_points_passes_the_two_vectors_and_bodies_to_the_collector() {
        let (mut e, log, a, b, collector) = rig(0, 0.5, false);
        let sink = Rc::clone(&log);
        e.register_double(GSK_GET_CLOSEST_POINT, move |e, args| {
            let mut words = args.to_vec();
            words.extend((0..5).map(|i| e.mem.u32(args[0] + 4 * i)));
            sink.borrow_mut().push((GSK_GET_CLOSEST_POINT, words));
            set_lanes(e, args[3], [1.0, 2.0, 3.0, 4.0]);
            set_lanes(e, args[2], [5.0, 6.0, 7.0, 8.0]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(COLLECTOR_SLOT_4, move |e, args| {
            let mut words = args.to_vec();
            words.extend((0..10).map(|i| e.mem.u32(args[1] + 4 * i)));
            sink.borrow_mut().push((COLLECTOR_SLOT_4, words));
            Ret::default()
        });
        let this = agent(&mut e);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 1.5);
        e.call(0x00da_c9e0, &args![this, a, b, input, collector]);
        let queries = calls(&log, GSK_GET_CLOSEST_POINT);
        assert_eq!(queries.len(), 1);
        // The query record: transform, body A's motion, the shapes and the float.
        let transform = calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL)[0][0];
        assert_eq!(queries[0][4], transform);
        assert_eq!(queries[0][5], e.mem.u32(a + 8));
        assert_eq!(queries[0][6], e.mem.u32(a));
        assert_eq!(queries[0][7], e.mem.u32(b));
        assert_eq!(f32::from_bits(queries[0][8]), 1.5);
        assert_eq!(queries[0][1], this + 0xc);
        let delivered = calls(&log, COLLECTOR_SLOT_4);
        assert_eq!(delivered.len(), 1);
        let record = &delivered[0][2..];
        let floats: Vec<f32> = record[..8].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(record[8..], [a, b]);
        assert_eq!(monitor_used(&mut e), 0x18);
    }

    #[test]
    fn get_closest_points_reports_nothing_when_the_query_misses() {
        let (mut e, log, a, b, collector) = rig(0, 0.5, false);
        log_double(&mut e, &log, GSK_GET_CLOSEST_POINT, 1);
        let this = agent(&mut e);
        let input = e.mem.alloc(0x20);
        e.call(0x00da_c9e0, &args![this, a, b, input, collector]);
        assert!(calls(&log, COLLECTOR_SLOT_4).is_empty());
        assert_eq!(monitor_used(&mut e), 0x18);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00dabeb0_stores_the_constant_and_clears_sixteen_bytes() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(0x0101_2054, 0.25f32);
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x55; 0x40]);
        e.call(0x00da_beb0, &args![this, 0u32]);
        assert_eq!(e.mem.f32(this + 0x18), 0.25);
        assert_eq!(e.mem.bytes(this + 0x20, 16), vec![0u8; 16]);
        assert_eq!(e.mem.u8(this + 0x17), 0x55);
        assert_eq!(e.mem.u8(this + 0x1c), 0x55);
        assert_eq!(e.mem.u8(this + 0x30), 0x55);
    }

    /// A signalling NaN constant is quieted by the `FLD` / `FSTP` pair.
    #[test]
    fn fn_00dabeb0_quiets_a_signalling_nan_constant() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(0x0101_2054, 0x7f80_0001u32);
        let this = e.mem.alloc(0x40);
        e.call(0x00da_beb0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0x7fc0_0001);
    }
}
