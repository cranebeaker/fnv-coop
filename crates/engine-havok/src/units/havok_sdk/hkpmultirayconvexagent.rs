//! `Havok SDK/hkpmultirayconvexagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMultiRayConvexAgent` (field names describe what the code does and are unconfirmed): a
//! collision agent between a multi-ray shape (first body) and another shape (second body).
//! The agent keeps, at +0xC, an array of 16-bit ids (data pointer, size at +0x10, capacity
//! and flags word at +0x14, inline storage for four from +0x18), one id per ray, which names
//! the contact the ray's last hit was reported as (0xFFFF for none), and at +8 the object
//! that owns the contacts (virtual function at +0xC creates one, at +0x14 removes one).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The memory router (`00c85750`, no arguments): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkTransform::setMulInverseMul` (Xbox PDB, `00cb22f0`, `this` = result, `(a, b)`).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `hkVector4::setTransformedPos` (`00c7f640`) and `setRotatedDir` (`00c7f700`), `this` =
/// result, `(transform, vector)`.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// Registers a pair of agent functions with the collision dispatcher (`00cfbbe0`, `this` =
/// dispatcher, `(functions, type a, type b)`).
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// Timer labels: `"TtmultiRay-cvx"`, `"TtmultiRay-cnvx-getPen"` and the end label.
const LABEL_RAY_CONVEX: u32 = 0x010c_eda8;
const LABEL_GET_PENETRATIONS: u32 = 0x010c_edb8;
const LABEL_END: u32 = 0x010c_41f8;
/// The agent's vtable (`010cedd4`) and the base vtable its destructor leaves (`0102e388`).
const AGENT_VTABLE: u32 = 0x010c_edd4;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The vtables of the small collector objects the symmetric agent functions build on the
/// stack: the closest-points one (`010ce7fc`) and the penetrations one (`010ce81c`).
const CLOSEST_COLLECTOR_VTABLE: u32 = 0x010c_e7fc;
const PENETRATION_COLLECTOR_VTABLE: u32 = 0x010c_e81c;
/// Constants in the exe image: four 1.0 (`010ec7f0`), the sign bit of the first three lanes
/// (`010c4c00`), the double 1.0 (`01012070`) and the largest finite float but one
/// (`0104da48`).
const ONES: u32 = 0x010e_c7f0;
const SIGN_MASK: u32 = 0x010c_4c00;
const DOUBLE_ONE: u32 = 0x0101_2070;
const NEARLY_FLOAT_MAX: u32 = 0x0104_da48;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, at: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, at: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, value);
    }
}

/// The four columns of the transform at `at`.
fn load_columns(e: &Engine, at: u32) -> [Vec4; 4] {
    [0, 0x10, 0x20, 0x30].map(|offset| load_vec(e, at + offset))
}

/// `x * column0 + y * column1 + z * column2 + column3`, summed `(y*c1 + x*c0) + (z*c2 + c3)`.
fn transform_point(columns: &[Vec4; 4], v: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        (v[1] * columns[1][lane] + v[0] * columns[0][lane])
            + (v[2] * columns[2][lane] + columns[3][lane])
    })
}

/// The same without the last column, summed `(y*c1 + x*c0) + z*c2`.
fn rotate_direction(columns: &[Vec4; 4], v: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        (v[1] * columns[1][lane] + v[0] * columns[0][lane]) + v[2] * columns[2][lane]
    })
}

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte timer record `(label, time stamp)` to the thread's monitor stream
/// (`TlsGetValue` of the index at `0126835c`; cursor at +4, limit at +0xC) while the cursor is
/// below the limit.
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

/// The allocator of the memory router.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Transforms the ray (two vectors) at `ray` with `columns` and stores the start at
/// `start_at` and the end at `end_at`.
fn store_transformed_ray(
    e: &mut Engine,
    ray: u32,
    columns: &[Vec4; 4],
    start_at: u32,
    end_at: u32,
) {
    let start = transform_point(columns, load_vec(e, ray));
    store_vec(e, start_at, start);
    let end = transform_point(columns, load_vec(e, ray + 0x10));
    store_vec(e, end_at, end);
}

// Translated from 00d36fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::cleanup` (Xbox PDB), thiscall `(this, arg)`, `RET 4`: tells the
/// owner (virtual function at +0x14 of the object at +8) to remove each contact whose id in
/// the array is not 0xFFFF, passing `(id, arg)`, then deletes the agent through its virtual
/// function at +0 with 1.
pub fn hkp_multi_ray_convex_agent_cleanup(e: &mut Engine, this: Ptr, arg: u32) {
    let t = this.addr();
    let size = e.mem.u32(t + 0x10) as i32;
    for i in 0..size.max(0) as u32 {
        let id = e.mem.u16(e.mem.u32(t + 0xc) + 2 * i);
        if id != 0xffff {
            let owner = e.mem.u32(t + 8);
            e.vcall(owner, 0x14, &args![u32::from(id), arg]);
        }
    }
    e.vcall(t, 0, &args![1u32]);
}

/// The body of `getClosestPoints` (`00d37010`, and the same code at `00d37280`): sets up the
/// frame the exe has (the relative transform of the two bodies at `+0x80`, the ray input at
/// `+0x20` (start, end, then two zero words at `+0x40`), the contact point handed to the
/// collector at `+0x50` (position, normal and distance, then the two body pointers at
/// `+0x70`) and the ray output at `+0xc0` with its fraction at `+0xd0`) and casts each of the
/// multi-ray shape's rays at the second body's shape.
fn closest_points(e: &mut Engine, body_a: u32, body_b: u32, collector: u32) {
    monitor_record(e, LABEL_RAY_CONVEX);
    let shape_b = e.mem.u32(body_b);
    let transform_b = e.mem.u32(body_b + 8);
    let transform_a = e.mem.u32(body_a + 8);
    let shape_a = e.mem.u32(body_a);
    e.with_stack(0x110, |e, frame| {
        let s = frame.addr();
        e.mem.set_u32(s + 0x1c, shape_b);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![s + 0x80, transform_b, transform_a],
        );
        let count = e.mem.u32(shape_a + 0x14) as i32;
        let mut ray = e.mem.u32(shape_a + 0x10);
        e.mem.set_u32(s + 0x70, body_a);
        e.mem.set_u32(s + 0x74, body_b);
        e.mem.set_u32(s + 0x40, 0);
        e.mem.set_u32(s + 0x44, 0);
        for _ in 0..count.max(0) {
            let columns = load_columns(e, s + 0x80);
            e.mem.set_f32(s + 0xd0, 1.0);
            store_transformed_ray(e, ray, &columns, s + 0x20, s + 0x30);
            e.mem.set_u32(s + 0xd4, 0xffff_ffff);
            e.mem.set_u32(s + 0xe0, 0xffff_ffff);
            e.mem.set_u32(s + 0x100, 0);
            e.vcall(shape_b, 0x20, &args![s + 0x17, s + 0x20, s + 0xc0]);
            if e.mem.u8(s + 0x17) != 0 {
                let fraction = x87_quiet(e.mem.f32(s + 0xd0));
                let start = load_vec(e, s + 0x20);
                let end = load_vec(e, s + 0x30);
                let ones = load_vec(e, ONES);
                let along: Vec4 = std::array::from_fn(|lane| {
                    (ones[lane] - fraction) * start[lane] + fraction * end[lane]
                });
                let body_columns = load_columns(e, transform_b);
                let point = transform_point(&body_columns, along);
                store_vec(e, s + 0x50, point);
                let normal = rotate_direction(&body_columns, load_vec(e, s + 0xc0));
                store_vec(e, s + 0x60, normal);
                let length = f64::from(x87_quiet(e.mem.f32(ray + 0xc)));
                let margin = f64::from(x87_quiet(e.mem.f32(shape_a + 0x1c)));
                let distance =
                    ((f64::from(fraction) - e.mem.f64(DOUBLE_ONE)) * length + margin) as f32;
                e.mem.set_f32(s + 0x6c, distance);
                e.vcall(collector, 4, &args![s + 0x50]);
            }
            ray = ray.wrapping_add(0x20);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d37010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::getClosestPoints` (Xbox PDB), `(body_a, body_b, input,
/// collector)` with the third word unread, `RET 0x10`: a timer scope around casting every ray
/// of the multi-ray shape of `body_a` (transformed into the second body's space with
/// `hkTransform::setMulInverseMul`) at the shape of `body_b` (virtual function at +0x20,
/// `(hit byte, ray input, ray output)`); each hit becomes a contact point (position, normal
/// and a distance of `(fraction - 1) * ray length + shape margin`, in the first body's
/// frame) given to the collector's virtual function at +4.
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn hkp_multi_ray_convex_agent_get_closest_points(
    e: &mut Engine,
    _this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    _unused_2: u32,
    collector: Ptr,
) {
    closest_points(e, body_a.addr(), body_b.addr(), collector.addr());
}

// Translated from 00d37280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the same code as `getClosestPoints` (`00d37010`) with the plain `RET`
/// of a cdecl function; the symmetric-agent code calls it with a collector built on the stack.
pub fn fn_00d37280(e: &mut Engine, body_a: Ptr, body_b: Ptr, _unused_2: u32, collector: Ptr) {
    closest_points(e, body_a.addr(), body_b.addr(), collector.addr());
}

// Translated from 00d374f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::staticGetPenetrations` (Xbox PDB), cdecl `(body_a, body_b, input,
/// collector)` with the third word unread: a timer scope (`"TtmultiRay-cnvx-getPen"`) around
/// casting each ray (transformed as in `getClosestPoints`, same ray input and output records
/// with the transform at `+0x50` and the output at `+0x90`) at the shape of `body_b` until
/// the collector's byte at +4 is set; each hit tells the collector (virtual function at +4)
/// `(body_a, body_b)`. A negative ray count runs until that byte is set, as in the exe.
pub fn hkp_multi_ray_convex_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_2: u32,
    collector: Ptr,
) {
    let (a, b, c) = (body_a.addr(), body_b.addr(), collector.addr());
    monitor_record(e, LABEL_GET_PENETRATIONS);
    let shape_a = e.mem.u32(a);
    let shape_b = e.mem.u32(b);
    let transform_b = e.mem.u32(b + 8);
    let transform_a = e.mem.u32(a + 8);
    e.with_stack(0xe0, |e, frame| {
        let s = frame.addr();
        e.mem.set_u32(s + 0x1c, shape_b);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![s + 0x50, transform_b, transform_a],
        );
        let mut remaining = e.mem.u32(shape_a + 0x14);
        let mut ray = e.mem.u32(shape_a + 0x10);
        e.mem.set_f32(s + 0xa0, 1.0);
        e.mem.set_u32(s + 0x40, 0);
        e.mem.set_u32(s + 0x44, 0);
        e.mem.set_u32(s + 0xa4, 0xffff_ffff);
        e.mem.set_u32(s + 0xd0, 0);
        e.mem.set_u32(s + 0xb0, 0xffff_ffff);
        while remaining != 0 {
            remaining -= 1;
            if e.mem.u8(c + 4) != 0 {
                break;
            }
            let columns = load_columns(e, s + 0x50);
            store_transformed_ray(e, ray, &columns, s + 0x20, s + 0x30);
            e.vcall(shape_b, 0x20, &args![s + 0x1b, s + 0x20, s + 0x90]);
            if e.mem.u8(s + 0x1b) != 0 {
                e.vcall(c, 4, &args![a, b]);
            }
            ray = ray.wrapping_add(0x20);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d37690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::getPenetrations` (Xbox PDB), `(body_a, body_b, input,
/// collector)`, `RET 0x10`: calls `staticGetPenetrations` (`00d374f0`) with the same four
/// words.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_multi_ray_convex_agent_get_penetrations(
    e: &mut Engine,
    _this: u32,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    collector: Ptr,
) {
    hkp_multi_ray_convex_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

// Translated from 00d376b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; the agent's constructor (unconfirmed), thiscall `(this, body, _,
/// _, owner)`, `RET 0x10`, returns `this`: stores the owner at +8, the vtable (`010cedd4`),
/// the word 1 at +6 and an array of 16-bit ids at +0xC (inline storage at +0x18, capacity
/// word `0x80000004`), grows it through `hkArrayUtil::_reserve` (element size 2) to the larger
/// of the ray count of the body's shape (word +0x14) and twice the capacity, and sets one
/// 0xFFFF id per ray.
pub fn fn_00d376b0(
    e: &mut Engine,
    this: Ptr,
    body: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    owner: u32,
) -> Ptr {
    let t = this.addr();
    let array = t + 0xc;
    e.mem.set_u32(t + 8, owner);
    e.mem.set_u32(t, AGENT_VTABLE);
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(array, array + 0xc);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0004);
    let shape = e.mem.u32(body.addr());
    let count = e.mem.u32(shape + 0x14) as i32;
    let heap = allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_mul(2);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![heap, array, wanted, 2u32]);
    }
    e.mem.set_u32(array + 4, count as u32);
    for i in 0..count.max(0) as u32 {
        let data = e.mem.u32(array);
        e.mem.set_u16(data + 2 * i, 0xffff);
    }
    this
}

// Translated from 00d377d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; registers the agent with the collision dispatcher (unconfirmed),
/// cdecl `(dispatcher)`: builds a record of four function addresses and two bytes on the
/// stack and registers it twice with `00cfbbe0`: `(0x00d37780, 0x00d37860, 0x00d378a0,
/// 0x00d3ac80, 1, 1)` for the type pair (0x12, 0x18) and `(0x00d37740, 0x00d374f0,
/// 0x00d37280, 0x00daf8e0, 0, 1)` for (0x18, 0x12). The two padding bytes after the record's
/// flags are never written (the callee copies them as they are; nothing reads them).
pub fn fn_00d377d0(e: &mut Engine, dispatcher: Ptr) {
    let records = [
        (
            [0x00d3_7780u32, 0x00d3_7860, 0x00d3_78a0, 0x00d3_ac80],
            [1u8, 1],
            (0x12u32, 0x18u32),
        ),
        (
            [0x00d3_7740, 0x00d3_74f0, 0x00d3_7280, 0x00da_f8e0],
            [0, 1],
            (0x18, 0x12),
        ),
    ];
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        for (functions, flags, (type_a, type_b)) in records {
            for (i, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(r + 4 * i as u32, function);
            }
            e.mem.set_u8(r + 0x10, flags[0]);
            e.mem.set_u8(r + 0x11, flags[1]);
            e.call(REGISTER_AGENT, &args![dispatcher, r, type_a, type_b]);
        }
    });
}

// Translated from 00d37860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; cdecl `(a0, a1, a2, a3)`: builds a collector object on the stack (vtable
/// `010ce81c`, a byte 0 at +4 and `a3` at +8; the other three bytes of that word are never
/// written) and calls `staticGetPenetrations` (`00d374f0`) with `(a1, a0, a2, collector)`
/// (the two bodies swapped).
pub fn fn_00d37860(e: &mut Engine, a0: Ptr, a1: Ptr, a2: u32, a3: u32) {
    e.with_stack(0xc, |e, collector| {
        let c = collector.addr();
        e.mem.set_u32(c + 8, a3);
        e.mem.set_u8(c + 4, 0);
        e.mem.set_u32(c, PENETRATION_COLLECTOR_VTABLE);
        hkp_multi_ray_convex_agent_static_get_penetrations(e, a1, a0, a2, collector);
    });
}

// Translated from 00d378a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; cdecl `(a0, a1, a2, a3)`: builds a collector object on the stack
/// (vtable `010ce7fc`, the float at `0104da48` (an `FLD`/`FSTP` copy) at +4 and `a3` at +8)
/// and calls `00d37280` (the cdecl copy of `getClosestPoints`) with `(a1, a0, a2, collector)`.
pub fn fn_00d378a0(e: &mut Engine, a0: Ptr, a1: Ptr, a2: u32, a3: u32) {
    e.with_stack(0xc, |e, collector| {
        let c = collector.addr();
        let limit = x87_quiet_bits(e.mem.u32(NEARLY_FLOAT_MAX));
        e.mem.set_u32(c + 8, a3);
        e.mem.set_u32(c + 4, limit);
        e.mem.set_u32(c, CLOSEST_COLLECTOR_VTABLE);
        fn_00d37280(e, a1, a0, a2, collector);
    });
}

// Translated from 00d378e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::processCollision` (Xbox PDB), thiscall `(this, body_a, body_b,
/// input, output)`, `RET 0x10`: like `getClosestPoints` (same frame, the ray output at
/// `+0xa0`), but each ray keeps its contact: a ray that hits writes a contact point at the
/// output's cursor (word +0 of `output`: position, normal and distance at +0x10, with
/// `hkVector4::setTransformedPos` and `setRotatedDir` of the second body's transform and the
/// distance at +0x1c), asks the owner (virtual function at +0xC, `(body_a, body_b, input,
/// output, 0, contact)`) to create its contact when the ray has none and, when it has one,
/// advances the cursor by 0x30 and stores the id at +0x20 of the contact point; a ray that
/// misses tells the owner (virtual function at +0x14, `(id, word +4 of output)`) to remove
/// its contact and clears the id.
pub fn hkp_multi_ray_convex_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let (t, a, b, o) = (this.addr(), body_a.addr(), body_b.addr(), output.addr());
    monitor_record(e, LABEL_RAY_CONVEX);
    let shape_a = e.mem.u32(a);
    let shape_b = e.mem.u32(b);
    let transform_b = e.mem.u32(b + 8);
    let transform_a = e.mem.u32(a + 8);
    e.with_stack(0xe8, |e, frame| {
        let s = frame.addr();
        e.mem.set_u32(s + 0x1c, shape_a);
        e.mem.set_u32(s + 0x18, shape_b);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![s + 0x50, transform_b, transform_a],
        );
        let count = e.mem.u32(shape_a + 0x14) as i32;
        e.mem.set_u32(s + 0x40, 0);
        e.mem.set_u32(s + 0x44, 0);
        let mut ray = e.mem.u32(shape_a + 0x10);
        for index in 0..count.max(0) as u32 {
            let columns = load_columns(e, s + 0x50);
            e.mem.set_f32(s + 0xb0, 1.0);
            store_transformed_ray(e, ray, &columns, s + 0x20, s + 0x30);
            e.mem.set_u32(s + 0xb4, 0xffff_ffff);
            e.mem.set_u32(s + 0xc0, 0xffff_ffff);
            e.mem.set_u32(s + 0xe0, 0);
            e.vcall(shape_b, 0x20, &args![s + 0x13, s + 0x20, s + 0xa0]);
            let ids = e.mem.u32(t + 0xc);
            if e.mem.u8(s + 0x13) == 0 {
                let id = e.mem.u16(ids + 2 * index);
                if id != 0xffff {
                    let cause = e.mem.u32(o + 4);
                    let owner = e.mem.u32(t + 8);
                    e.vcall(owner, 0x14, &args![u32::from(id), cause]);
                    let ids = e.mem.u32(t + 0xc);
                    e.mem.set_u16(ids + 2 * index, 0xffff);
                }
            } else {
                let fraction = e.mem.f32(s + 0xb0);
                let contact = e.mem.u32(o);
                let ones = load_vec(e, ONES);
                let start = load_vec(e, s + 0x20);
                let end = load_vec(e, s + 0x30);
                let along: Vec4 = std::array::from_fn(|lane| {
                    (ones[lane] - fraction) * start[lane] + fraction * end[lane]
                });
                store_vec(e, s + 0x90, along);
                e.call(SET_TRANSFORMED_POS, &args![contact, transform_b, s + 0x90]);
                e.call(
                    SET_ROTATED_DIR,
                    &args![contact + 0x10, transform_b, s + 0xa0],
                );
                let length = f64::from(x87_quiet(e.mem.f32(ray + 0xc)));
                let margin = f64::from(x87_quiet(e.mem.f32(shape_a + 0x1c)));
                let stored = x87_quiet(e.mem.f32(s + 0xb0));
                let distance =
                    ((f64::from(stored) - e.mem.f64(DOUBLE_ONE)) * length + margin) as f32;
                e.mem.set_f32(contact + 0x1c, distance);
                let ids = e.mem.u32(t + 0xc);
                if e.mem.u16(ids + 2 * index) == 0xffff {
                    let owner = e.mem.u32(t + 8);
                    let created = e
                        .vcall(owner, 0xc, &args![a, b, input.addr(), o, 0u32, contact])
                        .u16();
                    let ids = e.mem.u32(t + 0xc);
                    e.mem.set_u16(ids + 2 * index, created);
                }
                let ids = e.mem.u32(t + 0xc);
                let id = e.mem.u16(ids + 2 * index);
                if id != 0xffff {
                    let cursor = e.mem.u32(o);
                    e.mem.set_u32(o, cursor.wrapping_add(0x30));
                    e.mem.set_u32(contact + 0x20, u32::from(id));
                }
            }
            ray = ray.wrapping_add(0x20);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d37b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiRayConvexAgent>::getClosestPoints` (Xbox PDB), a virtual
/// method with `RET 0x10`, stack words `(a0, a1, a2, a3)`: builds a collector object on the stack (vtable
/// `010ce7fc`, the float at `0104da48` (an `FLD`/`FSTP` copy) at +4 and `a3` at +8) and calls
/// `getClosestPoints` (`00d37010`) with `(a1, a0, a2, collector)` (the bodies swapped).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads; it
/// stays in `ECX` for the call to `00d37010`.
pub fn hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_get_closest_points(
    e: &mut Engine,
    this: u32,
    a0: Ptr,
    a1: Ptr,
    a2: u32,
    a3: u32,
) {
    e.with_stack(0xc, |e, collector| {
        let c = collector.addr();
        e.mem.set_u32(c + 8, a3);
        let limit = x87_quiet_bits(e.mem.u32(NEARLY_FLOAT_MAX));
        e.mem.set_u32(c + 4, limit);
        e.mem.set_u32(c, CLOSEST_COLLECTOR_VTABLE);
        hkp_multi_ray_convex_agent_get_closest_points(e, Ptr::new(this), a1, a0, a2, collector);
    });
}

// Translated from 00d37bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiRayConvexAgent>::getPenetrations` (Xbox PDB), a virtual
/// method with `RET 0x10`, stack words `(a0, a1, a2, a3)`: builds a collector object on the stack (vtable
/// `010ce81c`, a byte 0 at +4 and `a3` at +8; the other three bytes of that word are never
/// written) and calls `getPenetrations` (`00d37690`) with `(a1, a0, a2, collector)`.
/// Virtual method: the first word is `this` (`ECX` in the exe), left in `ECX` for the callee.
pub fn hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_get_penetrations(
    e: &mut Engine,
    this: u32,
    a0: Ptr,
    a1: Ptr,
    a2: u32,
    a3: u32,
) {
    e.with_stack(0xc, |e, collector| {
        let c = collector.addr();
        e.mem.set_u32(c + 8, a3);
        e.mem.set_u8(c + 4, 0);
        e.mem.set_u32(c, PENETRATION_COLLECTOR_VTABLE);
        hkp_multi_ray_convex_agent_get_penetrations(e, this, a1, a0, a2, collector);
    });
}

// Translated from 00d37c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiRayConvexAgent>::processCollision` (Xbox PDB),
/// thiscall `(this, a0, a1, a2, output)`, `RET 0x10` (the exe leaves ECX alone and so passes
/// `this` on): runs `processCollision` (`00d378e0`) with the bodies swapped `(a1, a0, a2,
/// output)`, then turns each contact point it appended (from the cursor before to the cursor
/// after, 0x30 bytes each) around: the position moves by `distance * normal` in all four
/// lanes and the normal's first three lanes lose their sign. When the float at
/// `output + 0x3030` changed the call also negates the three lanes of the vector at
/// `output + 0x3020`, swaps the nibbles of the byte at `+0x304a`, exchanges the bytes at
/// `+0x3048` and `+0x3049`, swaps the 16-bit words at `+0x3040` and
/// `+0x3040 + 2 * (sum of those bytes - 1)` and, when both bytes were 1 and 2 (their AND is 2),
/// the words at `+0x3042` and `+0x3044`.
pub fn hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    a0: Ptr,
    a1: Ptr,
    a2: Ptr,
    output: Ptr,
) {
    let o = output.addr();
    let before = x87_quiet(e.mem.f32(o + 0x3030));
    let mut at = e.mem.u32(o);
    hkp_multi_ray_convex_agent_process_collision(e, this, a1, a0, a2, output);
    while at < e.mem.u32(o) {
        let position = load_vec(e, at);
        let normal = load_vec(e, at + 0x10);
        let moved: Vec4 = std::array::from_fn(|lane| normal[3] * normal[lane] + position[lane]);
        store_vec(e, at, moved);
        for lane in 0..4 {
            let word = e.mem.u32(at + 0x10 + 4 * lane) ^ e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(at + 0x10 + 4 * lane, word);
        }
        at = at.wrapping_add(0x30);
    }
    let after = x87_quiet(e.mem.f32(o + 0x3030));
    if before != after {
        for lane in 0..4 {
            let word = e.mem.u32(o + 0x3020 + 4 * lane) ^ e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(o + 0x3020 + 4 * lane, word);
        }
        let nibbles = e.mem.u8(o + 0x304a);
        let swapped = (nibbles >> 4) | ((nibbles & 0xf) << 4);
        let first = e.mem.u8(o + 0x3049);
        e.mem.set_u8(o + 0x304a, swapped);
        let second = e.mem.u8(o + 0x3048);
        e.mem.set_u8(o + 0x3049, second);
        e.mem.set_u8(o + 0x3048, first);
        let index = u32::from(first)
            .wrapping_add(u32::from(second))
            .wrapping_sub(1);
        let word = e.mem.u16(o + 0x3040);
        let other_at = (o + 0x3040).wrapping_add(index.wrapping_mul(2));
        let other = e.mem.u16(other_at);
        e.mem.set_u16(o + 0x3040, other);
        e.mem.set_u16(other_at, word);
        if e.mem.u8(o + 0x3048) & e.mem.u8(o + 0x3049) == 2 {
            let low = e.mem.u16(o + 0x3042);
            let high = e.mem.u16(o + 0x3044);
            e.mem.set_u16(o + 0x3042, high);
            e.mem.set_u16(o + 0x3044, low);
        }
    }
}

// Translated from 00d37d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayConvexAgent::_scalar_deleting_destructor_` (Xbox PDB), thiscall `(this,
/// flags)`, `RET 4`, returns `this`: empties the id array (size 0; storage that is owned,
/// capacity word non-negative, is freed through the allocator's virtual function at +0x10 with
/// `(data, capacity * 2)`; then data 0, capacity word `0x80000000`), restores the base vtable
/// (`0102e388`) and, when bit 0 of `flags` is set, frees the agent itself through the
/// allocator's virtual function at +8 with `(this, the word at +4)`.
pub fn hkp_multi_ray_convex_agent_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let t = this.addr();
    let heap = allocator(e);
    let capacity_word = e.mem.u32(t + 0x14);
    e.mem.set_u32(t + 0x10, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(t + 0xc);
        e.vcall(
            heap,
            0x10,
            &args![data, (capacity_word & 0x3fff_ffff).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(t + 0xc, 0);
    e.mem.set_u32(t + 0x14, 0x8000_0000);
    e.mem.set_u32(t, BASE_VTABLE);
    if flags & 1 != 0 {
        let heap = allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(heap, 8, &args![t, size]);
    }
    this
}

/// Allocates `size` bytes the way the agent create functions do: the thread's memory router
/// (`00c85750`) has the allocator at `+0x10`; its virtual slot `4` hands out the block, whose
/// size goes into the 16-bit word at `+4`.
fn allocate_agent_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00d37740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `owner`): a create
/// function of the multi-ray / convex pair (code at `00d3781d` stores its address in an agent
/// record). Allocates the 0x20-byte agent (`allocate_agent_block`) and runs the constructor
/// `00d376b0` on it with the four words in this order. Answers what the constructor answers,
/// the agent.
pub fn fn_00d37740(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: u32, owner: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x20);
    fn_00d376b0(e, Ptr::new(agent), body_a, body_b.addr(), input, owner).addr()
}

// Translated from 00d37780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `owner`): the create
/// function of the flipped multi-ray / convex agent (code at `00d377e3` stores its address in an
/// agent record). Allocates the 0x20-byte agent (`allocate_agent_block`), runs the constructor
/// `00d376b0` with the two bodies exchanged (`body_b`, `body_a`, `input`, `owner`) and stores
/// the vtable `010cee10`. Answers the agent.
pub fn fn_00d37780(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: u32, owner: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x20);
    fn_00d376b0(e, Ptr::new(agent), body_b, body_a.addr(), input, owner);
    e.mem.set_u32(agent, 0x010c_ee10);
    agent
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d37740, fn_00d37740(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00d37780, fn_00d37780(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00d3_6fc0, hkp_multi_ray_convex_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_7010,
            hkp_multi_ray_convex_agent_get_closest_points(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d3_7280, fn_00d37280(Ptr, Ptr, u32, Ptr)),
        entry!(
            0x00d3_74f0,
            hkp_multi_ray_convex_agent_static_get_penetrations(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d3_7690,
            hkp_multi_ray_convex_agent_get_penetrations(u32, Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d3_76b0, fn_00d376b0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d3_77d0, fn_00d377d0(Ptr)),
        entry!(0x00d3_7860, fn_00d37860(Ptr, Ptr, u32, u32)),
        entry!(0x00d3_78a0, fn_00d378a0(Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_78e0,
            hkp_multi_ray_convex_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_7b90,
            hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_get_closest_points(
                u32, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d3_7bd0,
            hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_get_penetrations(
                u32, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d3_7c10,
            hkp_symmetric_agent_linear_cast_multi_ray_convex_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(
            0x00d3_7d30,
            hkp_multi_ray_convex_agent_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn install_allocator(e: &mut Engine, block: u32) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        sizes
    }

    /// A body whose shape's word at `+0x14` (the ray count) is `rays`.
    fn body_with_rays(e: &mut Engine, rays: u32) -> u32 {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0x14, rays);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        body
    }

    #[test]
    fn fn_00d37740_allocates_the_agent_and_runs_the_constructor_with_the_bodies_in_order() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let a = body_with_rays(&mut e, 1);
        let b = body_with_rays(&mut e, 3);
        let agent = e.call(0x00d3_7740, &args![a, b, 0u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x20]);
        assert_eq!(e.mem.u16(agent + 4), 0x20);
        assert_eq!(e.mem.u32(agent), AGENT_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        // The constructor sizes the id array by the first body's ray count.
        assert_eq!(e.mem.u32(agent + 0x10), 1);
    }

    #[test]
    fn fn_00d37780_exchanges_the_bodies_and_stores_the_flipped_vtable() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let a = body_with_rays(&mut e, 1);
        let b = body_with_rays(&mut e, 3);
        let agent = e.call(0x00d3_7780, &args![a, b, 0u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x20]);
        assert_eq!(e.mem.u32(agent), 0x010c_ee10);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent + 0x10), 3);
    }

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    const CAST: u32 = 0x0900_0010;
    const COLLECT: u32 = 0x0900_0020;
    const CREATE: u32 = 0x0900_0030;
    const REMOVE: u32 = 0x0900_0040;
    const DESTRUCT: u32 = 0x0900_0050;
    const FREE_BLOCK: u32 = 0x0900_0060;
    const FREE_OBJECT: u32 = 0x0900_0070;

    /// `(address, argument words)` of every call a logging double received.
    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// What a ray cast saw: the two vectors of the ray input, the words after them, the
    /// output record before the call and where the three pointers lie to each other.
    #[derive(Debug, PartialEq)]
    struct Cast {
        start: Vec4,
        end: Vec4,
        input_words: [u32; 2],
        fraction: f32,
        output_words: [u32; 3],
        hit_to_input: u32,
        input_to_output: u32,
    }

    /// The argument words of each call a double received.
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A hit of a scripted ray cast: the normal and the fraction.
    type Hit = (Vec4, f32);

    struct Rig {
        e: Engine,
        log: Log,
        casts: Rc<RefCell<Vec<Cast>>>,
        script: Rc<RefCell<Vec<Option<Hit>>>>,
        timer: u32,
        timer_records: u32,
        transform_a: u32,
        transform_b: u32,
    }

    /// The transform the double of `setMulInverseMul` writes: no rotation and the
    /// translation (10, 20, 30).
    const RELATIVE: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [10.0, 20.0, 30.0, 1.0],
    ];

    /// The second body's transform: (x, y, z) -> (-y, x, z) + (100, 200, 300).
    const BODY_B: [Vec4; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [100.0, 200.0, 300.0, 1.0],
    ];

    fn store_columns(e: &mut Engine, at: u32, columns: [Vec4; 4]) {
        for (i, column) in columns.into_iter().enumerate() {
            store_vec(e, at + 0x10 * i as u32, column);
        }
    }

    /// An engine with the exe constants, the timer stream, a logging double for the matrix
    /// callee and a ray cast double that follows a script.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_vec(&mut e, ONES, [1.0; 4]);
        for (lane, word) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(SIGN_MASK + 4 * lane as u32, word);
        }
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        e.mem.set_u32(NEARLY_FLOAT_MAX, 0x7f7f_ffee);
        let timer = e.mem.alloc(0x20);
        let timer_records = e.mem.alloc(0x60);
        e.mem.set_u32(timer + 4, timer_records);
        e.mem.set_u32(timer + 0xc, timer_records + 0x60);
        e.set_global(MONITOR_SLOT, 7u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [7]);
            Ret {
                eax: timer,
                ..Ret::default()
            }
        });
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, move |e, a| {
            sink.borrow_mut()
                .push((TRANSFORM_SET_MUL_INVERSE_MUL, a.to_vec()));
            store_columns(e, a[0], RELATIVE);
            Ret::default()
        });
        let casts: Rc<RefCell<Vec<Cast>>> = Rc::new(RefCell::new(Vec::new()));
        let script: Rc<RefCell<Vec<Option<Hit>>>> = Rc::new(RefCell::new(Vec::new()));
        let (seen, plan) = (Rc::clone(&casts), Rc::clone(&script));
        e.register_double(CAST, move |e, a| {
            let mut seen = seen.borrow_mut();
            let index = seen.len();
            seen.push(Cast {
                start: load_vec(e, a[2]),
                end: load_vec(e, a[2] + 0x10),
                input_words: [e.mem.u32(a[2] + 0x20), e.mem.u32(a[2] + 0x24)],
                fraction: e.mem.f32(a[3] + 0x10),
                output_words: [
                    e.mem.u32(a[3] + 0x14),
                    e.mem.u32(a[3] + 0x20),
                    e.mem.u32(a[3] + 0x40),
                ],
                hit_to_input: a[2].wrapping_sub(a[1]),
                input_to_output: a[3].wrapping_sub(a[2]),
            });
            match plan.borrow().get(index).copied().flatten() {
                Some((normal, fraction)) => {
                    e.mem.set_u8(a[1], 1);
                    store_vec(e, a[3], normal);
                    e.mem.set_f32(a[3] + 0x10, fraction);
                }
                None => e.mem.set_u8(a[1], 0),
            }
            Ret::default()
        });
        let transform_a = e.mem.alloc(0x40);
        let transform_b = e.mem.alloc(0x40);
        store_columns(&mut e, transform_b, BODY_B);
        Rig {
            e,
            log,
            casts,
            script,
            timer,
            timer_records,
            transform_a,
            transform_b,
        }
    }

    impl Rig {
        /// An object whose first word points to a table with `address` in slot `slot`.
        fn object_with(&mut self, slot: u32, address: u32) -> u32 {
            let table = self.e.mem.alloc(0x80);
            self.e.mem.set_u32(table + slot, address);
            let object = self.e.mem.alloc(0x40);
            self.e.mem.set_u32(object, table);
            object
        }

        /// A double that logs its calls and answers `eax`.
        fn log_double(&mut self, address: u32, eax: u32) {
            let sink = Rc::clone(&self.log);
            self.e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }

        /// The calls to `address` so far.
        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(at, _)| *at == address)
                .map(|(_, a)| a.clone())
                .collect()
        }

        /// The first body has the rays `rays` (start, end) and the margin `margin`, the second
        /// is the shape the rays are cast at. Returns the two bodies.
        fn bodies(&mut self, rays: &[[f32; 8]], margin: f32) -> (u32, u32) {
            let data = self.e.mem.alloc(0x20 * rays.len() as u32 + 0x20);
            for (i, ray) in rays.iter().enumerate() {
                for (lane, value) in ray.iter().enumerate() {
                    self.e
                        .mem
                        .set_f32(data + 0x20 * i as u32 + 4 * lane as u32, *value);
                }
            }
            let ray_shape = self.e.mem.alloc(0x40);
            self.e.mem.set_u32(ray_shape + 0x10, data);
            self.e.mem.set_u32(ray_shape + 0x14, rays.len() as u32);
            self.e.mem.set_f32(ray_shape + 0x1c, margin);
            let ray_body = self.e.mem.alloc(0x10);
            self.e.mem.set_u32(ray_body, ray_shape);
            self.e.mem.set_u32(ray_body + 8, self.transform_a);
            let target_shape = self.object_with(0x20, CAST);
            let target_body = self.e.mem.alloc(0x10);
            self.e.mem.set_u32(target_body, target_shape);
            self.e.mem.set_u32(target_body + 8, self.transform_b);
            (ray_body, target_body)
        }

        fn set_script(&mut self, hits: &[Option<Hit>]) {
            *self.script.borrow_mut() = hits.to_vec();
        }

        /// The labels of the timer records written so far.
        fn timer_labels(&self) -> Vec<u32> {
            let cursor = self.e.mem.u32(self.timer + 4);
            (self.timer_records..cursor)
                .step_by(0xc)
                .map(|at| self.e.mem.u32(at))
                .collect()
        }
    }

    const TWO_RAYS: [[f32; 8]; 2] = [
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        [0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 2.0],
    ];

    /// The ray input the cast double sees for the two rays: the rays moved by (10, 20, 30).
    fn moved_rays() -> Vec<(Vec4, Vec4)> {
        vec![
            ([11.0, 22.0, 33.0, 1.0], [15.0, 26.0, 37.0, 1.0]),
            ([10.0, 20.0, 30.0, 1.0], [11.0, 21.0, 31.0, 1.0]),
        ]
    }

    fn seen_rays(r: &Rig) -> Vec<(Vec4, Vec4)> {
        r.casts.borrow().iter().map(|c| (c.start, c.end)).collect()
    }

    #[test]
    fn cleanup_removes_the_contacts_that_exist_and_then_deletes_the_agent() {
        let mut r = rig();
        let owner = r.object_with(0x14, REMOVE);
        r.log_double(REMOVE, 0);
        r.log_double(DESTRUCT, 0);
        let this = r.object_with(0, DESTRUCT);
        let ids = r.e.mem.alloc(8);
        for (i, id) in [5u16, 0xffff, 7].into_iter().enumerate() {
            r.e.mem.set_u16(ids + 2 * i as u32, id);
        }
        r.e.mem.set_u32(this + 8, owner);
        r.e.mem.set_u32(this + 0xc, ids);
        r.e.mem.set_u32(this + 0x10, 3);
        r.e.call(0x00d3_6fc0, &args![this, 0x77u32]);
        assert_eq!(r.calls(REMOVE), [[owner, 5, 0x77], [owner, 7, 0x77]]);
        assert_eq!(r.calls(DESTRUCT), [[this, 1]]);
        // An empty (or negative) size only deletes.
        r.log.borrow_mut().clear();
        r.e.mem.set_u32(this + 0x10, 0xffff_fffe);
        r.e.call(0x00d3_6fc0, &args![this, 0x77u32]);
        assert!(r.calls(REMOVE).is_empty());
        assert_eq!(r.calls(DESTRUCT), [[this, 1]]);
    }

    /// A collector whose function at +4 logs its arguments and, when it gets a contact
    /// point (two words), the ten words of it.
    fn collector_with_snapshot(r: &mut Rig) -> (u32, Words) {
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        r.e.register_double(COLLECT, move |e, a| {
            let mut words = a.to_vec();
            if a.len() == 2 {
                words.extend((0..10).map(|i| e.mem.u32(a[1] + 4 * i)));
            }
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let collector = r.object_with(4, COLLECT);
        (collector, seen)
    }

    /// `this` is the leading word of the virtual (thiscall) copy; the cdecl copy has none.
    fn check_closest_points(address: u32, this: &[u32]) {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(&TWO_RAYS, 0.5);
        r.set_script(&[Some(([1.0, 2.0, 3.0, 0.5], 0.25)), None]);
        let (collector, seen) = collector_with_snapshot(&mut r);
        let mut words = this.to_vec();
        words.extend(args![ray_body, target_body, 0u32, collector]);
        r.e.call(address, &words);
        assert_eq!(seen_rays(&r), moved_rays());
        for cast in r.casts.borrow().iter() {
            // The output is set up again for each ray: fraction 1, two keys of -1 and a zero.
            assert_eq!(cast.fraction, 1.0);
            assert_eq!(cast.output_words, [0xffff_ffff, 0xffff_ffff, 0]);
            assert_eq!(cast.input_words, [0, 0]);
            assert_eq!((cast.hit_to_input, cast.input_to_output), (9, 0xa0));
        }
        let args = r.calls(TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(args.len(), 1);
        assert_eq!(args[0][1..], [r.transform_b, r.transform_a]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let contact = &seen[0][2..];
        let floats: Vec<f32> = contact[..8].iter().map(|w| f32::from_bits(*w)).collect();
        // The point (12, 23, 34) along the ray moved by the second body's transform, the
        // normal (1, 2, 3) turned by it and the distance (0.25 - 1) * 4 + 0.5.
        assert_eq!(floats, [77.0, 212.0, 334.0, 1.0, -2.0, 1.0, 3.0, -2.5]);
        assert_eq!(contact[8..], [ray_body, target_body]);
        assert_eq!(r.timer_labels(), [LABEL_RAY_CONVEX, LABEL_END]);
    }

    #[test]
    fn closest_points_reports_each_hit_as_a_contact_point() {
        check_closest_points(0x00d3_7010, &[0x5555]);
    }

    #[test]
    fn the_cdecl_copy_of_closest_points_does_the_same() {
        check_closest_points(0x00d3_7280, &[]);
    }

    #[test]
    fn closest_points_with_no_rays_only_writes_the_timer_records() {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(&[], 0.0);
        let (collector, seen) = collector_with_snapshot(&mut r);
        r.e.call(
            0x00d3_7010,
            &args![0x5555u32, ray_body, target_body, 0u32, collector],
        );
        assert!(r.casts.borrow().is_empty());
        assert!(seen.borrow().is_empty());
        assert_eq!(r.timer_labels(), [LABEL_RAY_CONVEX, LABEL_END]);
    }

    #[test]
    fn a_full_timer_stream_is_left_alone() {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(&[], 0.0);
        let (collector, _) = collector_with_snapshot(&mut r);
        r.e.mem.set_u32(r.timer + 4, r.timer_records + 0x60);
        r.e.call(
            0x00d3_7010,
            &args![0x5555u32, ray_body, target_body, 0u32, collector],
        );
        assert_eq!(r.e.mem.u32(r.timer + 4), r.timer_records + 0x60);
        assert_eq!(r.e.mem.u32(r.timer_records), 0);
    }

    #[test]
    fn penetrations_tell_the_collector_about_each_hit_until_it_stops() {
        let mut r = rig();
        let three = [TWO_RAYS[0], TWO_RAYS[1], TWO_RAYS[0]];
        let (ray_body, target_body) = r.bodies(&three, 0.5);
        let hit = Some(([0.0, 0.0, 1.0, 0.0], 0.25));
        r.set_script(&[hit, None, hit]);
        let (collector, seen) = collector_with_snapshot(&mut r);
        r.e.call(0x00d3_74f0, &args![ray_body, target_body, 0u32, collector]);
        assert_eq!(r.casts.borrow().len(), 3);
        assert_eq!(
            *seen.borrow(),
            [
                vec![collector, ray_body, target_body],
                vec![collector, ray_body, target_body]
            ]
        );
        let casts = r.casts.borrow();
        // The output is only set up once: the fraction a cast leaves stays for the next one.
        assert_eq!(casts[0].fraction, 1.0);
        assert_eq!(casts[1].fraction, 0.25);
        assert_eq!(casts[0].output_words, [0xffff_ffff, 0xffff_ffff, 0]);
        assert_eq!((casts[0].hit_to_input, casts[0].input_to_output), (5, 0x70));
        assert_eq!(r.timer_labels(), [LABEL_GET_PENETRATIONS, LABEL_END]);
    }

    #[test]
    fn penetrations_stop_when_the_collector_is_done() {
        let mut r = rig();
        let three = [TWO_RAYS[0]; 3];
        let (ray_body, target_body) = r.bodies(&three, 0.5);
        let hit = Some(([0.0, 0.0, 1.0, 0.0], 0.5));
        r.set_script(&[hit, hit, hit]);
        let collector = r.object_with(4, COLLECT);
        r.e.register_double(COLLECT, |e, a| {
            e.mem.set_u8(a[0] + 4, 1);
            Ret::default()
        });
        r.e.call(0x00d3_74f0, &args![ray_body, target_body, 0u32, collector]);
        assert_eq!(r.casts.borrow().len(), 1);
        // A collector that is done at the start sees no cast at all.
        r.casts.borrow_mut().clear();
        r.e.call(0x00d3_74f0, &args![ray_body, target_body, 0u32, collector]);
        assert!(r.casts.borrow().is_empty());
    }

    #[test]
    fn the_penetrations_wrapper_passes_its_words_on() {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(&TWO_RAYS, 0.5);
        r.set_script(&[Some(([0.0, 0.0, 1.0, 0.0], 0.5)), None]);
        let (collector, seen) = collector_with_snapshot(&mut r);
        r.e.call(
            0x00d3_7690,
            &args![VCALL_THIS, ray_body, target_body, 0u32, collector],
        );
        assert_eq!(*seen.borrow(), [vec![collector, ray_body, target_body]]);
        assert_eq!(r.casts.borrow().len(), 2);
    }

    #[test]
    fn the_constructor_makes_an_id_per_ray_and_grows_the_array_when_needed() {
        // (rays, the capacity asked for)
        let cases = [
            (0u32, None),
            (3, None),
            (4, None),
            (6, Some(8u32)),
            (8, Some(8)),
            (9, Some(9)),
        ];
        for (count, requested) in cases {
            let mut r = rig();
            let allocator = r.e.mem.alloc(0x10);
            let router = r.e.mem.alloc(0x20);
            r.e.mem.set_u32(router + 0x10, allocator);
            r.e.register_double(MEMORY_ROUTER, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            let sink = Rc::clone(&r.log);
            r.e.register_double(ARRAY_RESERVE, move |e, a| {
                sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
                let block = e.mem.alloc(a[2] * a[3]);
                e.mem.set_u32(a[1], block);
                e.mem.set_u32(a[1] + 8, a[2]);
                Ret::default()
            });
            let rays = vec![[0.0; 8]; count as usize];
            let (body, _) = r.bodies(&rays, 0.0);
            let this = r.e.mem.alloc(0x40);
            let returned =
                r.e.call(0x00d3_76b0, &args![this, body, 1u32, 2u32, 0x1234u32]);
            assert_eq!(returned.u32(), this);
            assert_eq!(r.e.mem.u32(this), AGENT_VTABLE);
            assert_eq!(r.e.mem.u16(this + 6), 1);
            assert_eq!(r.e.mem.u32(this + 8), 0x1234);
            assert_eq!(r.e.mem.u32(this + 0x10), count);
            let data = r.e.mem.u32(this + 0xc);
            match requested {
                None => {
                    assert!(r.calls(ARRAY_RESERVE).is_empty());
                    assert_eq!(data, this + 0x18);
                    assert_eq!(r.e.mem.u32(this + 0x14), 0x8000_0004);
                }
                Some(size) => {
                    assert_eq!(r.calls(ARRAY_RESERVE), [[allocator, this + 0xc, size, 2]]);
                    assert_ne!(data, this + 0x18);
                }
            }
            for i in 0..count {
                assert_eq!(r.e.mem.u16(data + 2 * i), 0xffff, "{count} rays, id {i}");
            }
        }
    }

    #[test]
    fn the_registration_hands_over_two_records() {
        let mut r = rig();
        let sink = Rc::clone(&r.log);
        r.e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|i| e.mem.u32(a[1] + 4 * i)));
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            sink.borrow_mut().push((REGISTER_AGENT, words));
            Ret::default()
        });
        let dispatcher = r.e.mem.alloc(0x10);
        r.e.call(0x00d3_77d0, &args![dispatcher]);
        let calls = r.calls(REGISTER_AGENT);
        assert_eq!(calls.len(), 2);
        let record = calls[0][1];
        assert_eq!(
            calls[0],
            [
                dispatcher,
                record,
                0x12,
                0x18,
                0x00d3_7780,
                0x00d3_7860,
                0x00d3_78a0,
                0x00d3_ac80,
                1,
                1
            ]
        );
        assert_eq!(
            calls[1],
            [
                dispatcher,
                record,
                0x18,
                0x12,
                0x00d3_7740,
                0x00d3_74f0,
                0x00d3_7280,
                0x00da_f8e0,
                0,
                1
            ]
        );
    }

    /// What the collector double of a wrapper saw: its arguments and the words of the collector
    /// object built on the stack.
    fn wrapper_collector(r: &mut Rig, vtable: u32) -> Words {
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        r.e.register_double(COLLECT, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)]);
            sink.borrow_mut().push(words);
            Ret::default()
        });
        r.e.mem.set_u32(vtable + 4, COLLECT);
        seen
    }

    #[test]
    fn the_wrappers_build_a_penetration_collector_and_swap_the_bodies() {
        for address in [0x00d3_7860u32, 0x00d3_7bd0] {
            let mut r = rig();
            let (ray_body, target_body) = r.bodies(&TWO_RAYS, 0.5);
            r.set_script(&[Some(([0.0, 0.0, 1.0, 0.0], 0.5)), None]);
            let seen = wrapper_collector(&mut r, PENETRATION_COLLECTOR_VTABLE);
            // The multi-ray body is the second word, the shape it is cast at the first.
            // The vtable entry takes `this` first; its static twin does not.
            let mut words = Vec::new();
            if address == 0x00d3_7bd0 {
                words.push(VCALL_THIS);
            }
            words.extend([target_body, ray_body, 5u32, 0xabcdu32]);
            r.e.call(address, &words);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1, "{address:#x}");
            assert_eq!(seen[0][1..3], [ray_body, target_body]);
            assert_eq!(seen[0][3..], [PENETRATION_COLLECTOR_VTABLE, 0, 0xabcd]);
        }
    }

    #[test]
    fn the_wrappers_build_a_closest_points_collector_and_swap_the_bodies() {
        for address in [0x00d3_78a0u32, 0x00d3_7b90] {
            let mut r = rig();
            let (ray_body, target_body) = r.bodies(&TWO_RAYS, 0.5);
            r.set_script(&[Some(([0.0, 0.0, 1.0, 0.0], 0.5)), None]);
            let seen = wrapper_collector(&mut r, CLOSEST_COLLECTOR_VTABLE);
            // The vtable entry takes `this` first; its static twin does not.
            let mut words = Vec::new();
            if address == 0x00d3_7b90 {
                words.push(VCALL_THIS);
            }
            words.extend([target_body, ray_body, 5u32, 0xabcdu32]);
            r.e.call(address, &words);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1, "{address:#x}");
            assert_eq!(
                seen[0][2..],
                [CLOSEST_COLLECTOR_VTABLE, 0x7f7f_ffee, 0xabcd]
            );
            assert_eq!(r.casts.borrow().len(), 2);
        }
    }

    #[test]
    fn the_wrapper_copies_the_float_through_the_x87_stack() {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(&TWO_RAYS, 0.5);
        r.set_script(&[Some(([0.0, 0.0, 1.0, 0.0], 0.5)), None]);
        r.e.mem.set_u32(NEARLY_FLOAT_MAX, 0x7f80_0001);
        let seen = wrapper_collector(&mut r, CLOSEST_COLLECTOR_VTABLE);
        r.e.call(0x00d3_78a0, &args![target_body, ray_body, 5u32, 0u32]);
        assert_eq!(seen.borrow()[0][3], 0x7fc0_0001);
    }

    /// A processing rig: an agent with the ids `ids` (size = their number), an owner whose
    /// create function answers the values of `created` in turn and whose remove function
    /// is logged, and an output record with room for contact points.
    struct Collision {
        r: Rig,
        this: u32,
        ids: u32,
        owner: u32,
        output: u32,
        contacts: u32,
        ray_body: u32,
        target_body: u32,
    }

    fn collision(
        rays: &[[f32; 8]],
        ids: &[u16],
        created: &[u16],
        hits: &[Option<Hit>],
    ) -> Collision {
        let mut r = rig();
        let (ray_body, target_body) = r.bodies(rays, 0.25);
        r.set_script(hits);
        let table = r.e.mem.alloc(0x80);
        r.e.mem.set_u32(table + 0x14, REMOVE);
        r.e.mem.set_u32(table + 0xc, CREATE);
        let owner = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(owner, table);
        r.log_double(REMOVE, 0);
        let sink = Rc::clone(&r.log);
        // The double outlives `created`, so it owns a copy.
        #[allow(clippy::unnecessary_to_owned)]
        let mut answers = created.to_vec().into_iter();
        r.e.register_double(CREATE, move |_, a| {
            sink.borrow_mut().push((CREATE, a.to_vec()));
            Ret {
                eax: u32::from(answers.next().expect("no answer left")),
                ..Ret::default()
            }
        });
        for (address, marker) in [
            (SET_TRANSFORMED_POS, [7.0f32, 8.0, 9.0, 1.0]),
            (SET_ROTATED_DIR, [4.0, 5.0, 6.0, 0.0]),
        ] {
            let sink = Rc::clone(&r.log);
            r.e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|i| e.mem.u32(a[2] + 4 * i)));
                sink.borrow_mut().push((address, words));
                store_vec(e, a[0], marker);
                Ret::default()
            });
        }
        let this = r.e.mem.alloc(0x40);
        let stored = r.e.mem.alloc(2 * ids.len() as u32 + 2);
        for (i, id) in ids.iter().enumerate() {
            r.e.mem.set_u16(stored + 2 * i as u32, *id);
        }
        r.e.mem.set_u32(this + 8, owner);
        r.e.mem.set_u32(this + 0xc, stored);
        r.e.mem.set_u32(this + 0x10, ids.len() as u32);
        let output = r.e.mem.alloc(0x3100);
        let contacts = r.e.mem.alloc(0x30 * 4 + 0x10);
        r.e.mem.set_u32(output, contacts);
        r.e.mem.set_u32(output + 4, 0xcafe);
        Collision {
            r,
            this,
            ids: stored,
            owner,
            output,
            contacts,
            ray_body,
            target_body,
        }
    }

    impl Collision {
        fn id(&self, index: u32) -> u16 {
            self.r.e.mem.u16(self.ids + 2 * index)
        }

        fn contact_floats(&self, index: u32) -> Vec<f32> {
            read(&self.r.e, self.contacts + 0x30 * index, 8)
        }
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn processing_creates_keeps_and_removes_contacts() {
        let rays = [TWO_RAYS[0], TWO_RAYS[1], TWO_RAYS[0]];
        let hit = Some(([1.0, 2.0, 3.0, 0.0], 0.5));
        let mut c = collision(&rays, &[0xffff, 7, 5], &[3], &[hit, None, hit]);
        let (this, o, ray_body, target_body) = (c.this, c.output, c.ray_body, c.target_body);
        c.r.e.call(
            0x00d3_78e0,
            &args![this, ray_body, target_body, 0x4444u32, o],
        );
        // Ray 0 hit and had no contact: created (answer 3). Ray 1 missed: its contact 7 was
        // removed. Ray 2 hit and keeps 5.
        assert_eq!([c.id(0), c.id(1), c.id(2)], [3, 0xffff, 5]);
        assert_eq!(
            c.r.calls(CREATE),
            [[c.owner, ray_body, target_body, 0x4444, o, 0, c.contacts]]
        );
        assert_eq!(c.r.calls(REMOVE), [[c.owner, 7, 0xcafe]]);
        assert_eq!(c.r.e.mem.u32(o), c.contacts + 0x60);
        assert_eq!(c.r.e.mem.u32(c.contacts + 0x20), 3);
        assert_eq!(c.r.e.mem.u32(c.contacts + 0x50), 5);
        // The distance is (0.5 - 1) * ray length + the margin: -2 + 0.25 for the first ray
        // (length 4).
        let first = c.contact_floats(0);
        assert_eq!(first, [7.0, 8.0, 9.0, 1.0, 4.0, 5.0, 6.0, -1.75]);
        assert_eq!(c.contact_floats(1)[7], -1.75);
        // The matrix calls get the second body's transform and, for the position, the point
        // halfway along the moved ray.
        let position = c.r.calls(SET_TRANSFORMED_POS);
        assert_eq!(position.len(), 2);
        assert_eq!(position[0][..2], [c.contacts, c.r.transform_b]);
        let halfway: Vec<f32> = position[0][3..]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(halfway, [13.0, 24.0, 35.0, 1.0]);
        let direction = c.r.calls(SET_ROTATED_DIR);
        assert_eq!(direction[0][..2], [c.contacts + 0x10, c.r.transform_b]);
        let normal: Vec<f32> = direction[0][3..]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(normal, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(c.r.timer_labels(), [LABEL_RAY_CONVEX, LABEL_END]);
        for cast in c.r.casts.borrow().iter() {
            assert_eq!(cast.fraction, 1.0);
            assert_eq!((cast.hit_to_input, cast.input_to_output), (0xd, 0x80));
        }
    }

    #[test]
    fn a_contact_the_owner_could_not_create_does_not_advance_the_cursor() {
        let hit = Some(([1.0, 2.0, 3.0, 0.0], 0.5));
        let mut c = collision(&TWO_RAYS[..1], &[0xffff], &[0xffff], &[hit]);
        let (this, o, a, b) = (c.this, c.output, c.ray_body, c.target_body);
        c.r.e.call(0x00d3_78e0, &args![this, a, b, 0u32, o]);
        assert_eq!(c.id(0), 0xffff);
        assert_eq!(c.r.e.mem.u32(o), c.contacts);
        assert_eq!(c.r.e.mem.u32(c.contacts + 0x20), 0);
        // The owner's answer is a 16-bit value: the high half of EAX does not count.
        let mut c = collision(&TWO_RAYS[..1], &[0xffff], &[0xffff], &[hit]);
        c.r.e.register_double(CREATE, |_, _| Ret {
            eax: 0x1234_0009,
            ..Ret::default()
        });
        let (this, o, a, b) = (c.this, c.output, c.ray_body, c.target_body);
        c.r.e.call(0x00d3_78e0, &args![this, a, b, 0u32, o]);
        assert_eq!(c.id(0), 9);
    }

    #[test]
    fn a_miss_without_a_contact_does_nothing() {
        let mut c = collision(&TWO_RAYS[..1], &[0xffff], &[], &[None]);
        let (this, o, a, b) = (c.this, c.output, c.ray_body, c.target_body);
        c.r.e.call(0x00d3_78e0, &args![this, a, b, 0u32, o]);
        assert!(c.r.calls(REMOVE).is_empty());
        assert_eq!(c.r.e.mem.u32(o), c.contacts);
    }

    /// The symmetric version: one hit with the contact the double matrix calls write
    /// ((7, 8, 9, 1) and the normal (4, 5, 6) with the distance -0.75 in its fourth lane).
    fn symmetric(change_flag: bool, bytes: [u8; 3]) -> Collision {
        let hit = Some(([1.0, 2.0, 3.0, 0.0], 0.5));
        let ray = [[0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 2.0]];
        let mut c = collision(&ray, &[0xffff], &[3], &[hit]);
        let o = c.output;
        store_vec(&mut c.r.e, o + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        for (i, value) in bytes.into_iter().enumerate() {
            c.r.e.mem.set_u8(o + 0x3048 + i as u32, value);
        }
        for (i, value) in [0xaa00u16, 0xbb11, 0xcc22, 0xdd33].into_iter().enumerate() {
            c.r.e.mem.set_u16(o + 0x3040 + 2 * i as u32, value);
        }
        c.r.e.mem.set_f32(o + 0x3030, 1.5);
        if change_flag {
            // The owner's create function is where something else may write.
            c.r.e.register_double(CREATE, move |e, _| {
                e.mem.set_f32(o + 0x3030, 2.5);
                Ret {
                    eax: 3,
                    ..Ret::default()
                }
            });
        }
        let (this, a, b) = (c.this, c.ray_body, c.target_body);
        // The symmetric function swaps the bodies: the multi-ray body is the second word.
        c.r.e.call(0x00d3_7c10, &args![this, b, a, 0u32, o]);
        c
    }

    #[test]
    fn the_symmetric_version_turns_the_new_contacts_around() {
        let c = symmetric(false, [1, 2, 0x12]);
        // Distance -0.75: the position moves by -0.75 * (4, 5, 6, -0.75), the first three
        // lanes of the normal lose their sign and its fourth lane stays.
        assert_eq!(
            c.contact_floats(0),
            [4.0, 4.25, 4.5, 1.5625, -4.0, -5.0, -6.0, -0.75]
        );
        // The flag float did not change: nothing else moved.
        let o = c.output;
        assert_eq!(read(&c.r.e, o + 0x3020, 4), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            [
                c.r.e.mem.u8(o + 0x3048),
                c.r.e.mem.u8(o + 0x3049),
                c.r.e.mem.u8(o + 0x304a)
            ],
            [1, 2, 0x12]
        );
    }

    #[test]
    fn a_changed_float_swaps_the_bookkeeping_fields() {
        let c = symmetric(true, [1, 2, 0x12]);
        let (e, o) = (&c.r.e, c.output);
        assert_eq!(read(e, o + 0x3020, 4), [-1.0, -2.0, -3.0, 4.0]);
        // Nibbles swapped, the two bytes exchanged.
        assert_eq!(
            [
                e.mem.u8(o + 0x3048),
                e.mem.u8(o + 0x3049),
                e.mem.u8(o + 0x304a)
            ],
            [2, 1, 0x21]
        );
        // Index 2 + 1 - 1 = 2: the first and the third word trade places. The AND of the
        // bytes (2 & 1) is not 2.
        assert_eq!(
            [
                e.mem.u16(o + 0x3040),
                e.mem.u16(o + 0x3042),
                e.mem.u16(o + 0x3044),
                e.mem.u16(o + 0x3046)
            ],
            [0xcc22, 0xbb11, 0xaa00, 0xdd33]
        );
    }

    #[test]
    fn equal_bytes_of_two_swap_the_second_pair_of_words_as_well() {
        let c = symmetric(true, [2, 2, 0x34]);
        let (e, o) = (&c.r.e, c.output);
        // Index 2 + 2 - 1 = 3: the first and the fourth word trade places; the bytes are 2 and
        // 2, so the second and third trade places too.
        assert_eq!(
            [
                e.mem.u16(o + 0x3040),
                e.mem.u16(o + 0x3042),
                e.mem.u16(o + 0x3044),
                e.mem.u16(o + 0x3046)
            ],
            [0xdd33, 0xcc22, 0xbb11, 0xaa00]
        );
        assert_eq!(e.mem.u8(o + 0x304a), 0x43);
    }

    #[test]
    fn the_destructor_frees_owned_storage_and_the_object_when_asked() {
        let mut r = rig();
        let table = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(table + 0x10, FREE_BLOCK);
        r.e.mem.set_u32(table + 8, FREE_OBJECT);
        let allocator = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(allocator, table);
        let router = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(router + 0x10, allocator);
        r.e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        r.log_double(FREE_BLOCK, 0);
        r.log_double(FREE_OBJECT, 0);
        let this = r.e.mem.alloc(0x40);
        r.e.mem.set_u16(this + 4, 0x30);
        r.e.mem.set_u32(this + 0xc, 0x5000);
        r.e.mem.set_u32(this + 0x10, 3);
        r.e.mem.set_u32(this + 0x14, 6);
        let returned = r.e.call(0x00d3_7d30, &args![this, 1u32]);
        assert_eq!(returned.u32(), this);
        assert_eq!(r.calls(FREE_BLOCK), [[allocator, 0x5000, 12]]);
        assert_eq!(r.calls(FREE_OBJECT), [[allocator, this, 0x30]]);
        assert_eq!(r.e.mem.u32(this), BASE_VTABLE);
        assert_eq!(r.e.mem.u32(this + 0xc), 0);
        assert_eq!(r.e.mem.u32(this + 0x10), 0);
        assert_eq!(r.e.mem.u32(this + 0x14), 0x8000_0000);
        // Borrowed storage is not freed, and an even flags word keeps the object.
        r.log.borrow_mut().clear();
        r.e.mem.set_u32(this + 0xc, 0x6000);
        r.e.mem.set_u32(this + 0x14, 0x8000_0004);
        r.e.call(0x00d3_7d30, &args![this, 2u32]);
        assert!(r.calls(FREE_BLOCK).is_empty());
        assert!(r.calls(FREE_OBJECT).is_empty());
        assert_eq!(r.e.mem.u32(this + 0xc), 0);
    }
}
