//! `NiMain/nirenderer.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderer::ms_RTTI` (the class's `NiRTTI` record).
const NI_RENDERER_RTTI: u32 = 0x011f_4758;

// Translated from 00a62b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_renderer_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RENDERER_RTTI)
}

// Translated from 00a63230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a62b60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x280 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_renderer_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_2b60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x280u32]);
    }
    this
}

/// Import slots of `DeleteCriticalSection` and `InterlockedDecrement` (`stdcall`, one word).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `011f4748`: the global `00a62b60` clears.
const RENDERER_GLOBAL: u32 = 0x011f_4748;
/// `00a5d3d0` (unit `NiMain/niobject.cpp`): reached with a `JMP` (same `this`).
const OBJECT_TAIL: u32 = 0x00a5_d3d0;

// Translated from 00a62b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nirenderer.cpp`, `thiscall`, no stack word): sets the
/// vtable `0109bb94`, clears `011f4748`, deletes the critical sections at +0x180, +0x100 and
/// +0x80 (in that order), drops one reference of the objects at +0x14 and +8 when set
/// (`InterlockedDecrement` of the count at +4; at zero their vtable slot +4 runs), and
/// continues in `00a5d3d0` (a `JMP`; its result is passed on).
pub fn fn_00a62b60(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_bb94);
    e.mem.set_u32(RENDERER_GLOBAL, 0);
    for offset in [0x180u32, 0x100, 0x80] {
        e.call(DELETE_CRITICAL_SECTION, &args![base + offset]);
    }
    for offset in [0x14u32, 8] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            let count = e
                .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32();
            if count == 0 {
                e.vcall(object, 4, &args![]);
            }
        }
    }
    e.call(OBJECT_TAIL, &args![this]).u32()
}

/// `004a0c10` (`thiscall` on a three-float point, no stack word, nothing in ST0): the
/// point's unitize (other units name it so).
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// `004168a0` (`thiscall` on a 3x3 matrix; angle, x, y, z as floats; `RET 0x10`): sets
/// the matrix from an angle and an axis.
const MATRIX_FROM_ANGLE_AXIS: u32 = 0x0041_68a0;
/// `004b4500` (`thiscall` on a 3x3 matrix; result, point; `RET 8`): the matrix times the
/// point, written to `result`, which it answers.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `00a58510` (`cdecl`, one float; result in ST0): the arc cosine clamped to `[-1, 1]`.
const CLAMPED_ACOS: u32 = 0x00a5_8510;
/// CRT `acos` taking and returning ST0 (`00eca1d0`).
const CRT_ACOS: u32 = 0x00ec_a1d0;
/// `double` -1.0.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `double` 0.5.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `float` pi (`0x40490fdb`).
const PI_FLOAT: u32 = 0x0109_bb88;
/// `float` 0.1 (`0x3dcccccd`).
const TENTH_FLOAT: u32 = 0x0101_e2bc;

/// `FLD dword [a]`.
fn load(e: &Engine, a: u32) -> f64 {
    x87_load32(e.mem.u32(a))
}

/// A `float` memory operand at `a`.
fn operand(e: &Engine, a: u32) -> f64 {
    x87_m32(e.mem.u32(a))
}

/// `FSTP dword [a]`.
fn store(e: &mut Engine, a: u32, v: f64) {
    e.mem.set_u32(a, x87_store32(v));
}

/// `FLD dword [a]; FSTP dword [..]`: the word the exe pushes for the float at `a`.
fn pushed(e: &Engine, a: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(a))
}

// Translated from 00a62be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nirenderer.cpp`, `cdecl`, seven words): builds a
/// rectangle of half-size `size / 2` around `origin`, facing a direction.
///
/// `frame` holds three point pointers (`p`, `u`, `w`). With neither `p` nor `target` it
/// answers false and writes nothing. Otherwise it fills `direction` and two axes `r`, `q`:
/// - `p` set, no `target`: `direction` is `p`; the matrix from angle and axis (`004168a0`)
///   of the unitized `p x direction` and the angle `acos(p . direction)` (pi when the cosine
///   is `<= -1` or NaN, 0 when `>= 1`) turns `u` into `r` and `w` into `q`;
/// - no `p`: `direction` is `target - origin`, `q` the unitized cross product of
///   `(0, 0.1, 1)` (unitized) and `direction`, `r` the unitized cross product of `q` and
///   `direction`;
/// - both: `direction` is `p + unitize(target - origin)`, unitized; the matrix from the
///   unitized `p x direction` and the clamped arc cosine (`00a58510`) of `p . direction`
///   turns `u` into `r` and `w` into `q` (`004b4500`).
///
/// Then `direction` is unitized again; `corners` gets `origin + h q`, `origin - h q`,
/// `origin + h r`, `origin - h r` (twelve floats, `h = size * 0.5` as a float) and `axes`
/// gets `r` then `q` (six floats). Answers true. Every product and sum is x87 in the exe's
/// operand order; the locals the exe passes by address live in one stack block laid out as
/// its frame (`r` at +0x10, `q` at +0x1c, the matrix at +0x40, the product's result at
/// +0x34).
pub fn fn_00a62be0(
    e: &mut Engine,
    frame: Ptr,
    origin: Ptr,
    target: Ptr,
    size: f32,
    corners: Ptr,
    axes: Ptr,
    direction: Ptr,
) -> bool {
    let first = e.mem.u32(frame.addr());
    let target = target.addr();
    if first == 0 && target == 0 {
        return false;
    }
    let frame = frame.addr();
    let o = origin.addr();
    let d = direction.addr();
    e.with_stack(0x54, |e, block| {
        let k = |offset: u32| block.addr() + offset - 0x10;
        let m = k(0x40);
        if target == 0 {
            for i in 0..3 {
                let word = e.mem.u32(first + 4 * i);
                e.mem.set_u32(d + 4 * i, word);
            }
            let p = e.mem.u32(frame);
            let c0 = x87_sub(
                x87_mul(load(e, p + 8), operand(e, d + 4)),
                x87_mul(load(e, p + 4), operand(e, d + 8)),
            );
            store(e, k(0x1c), c0);
            let c1 = x87_sub(
                x87_mul(load(e, d + 8), operand(e, p)),
                x87_mul(load(e, p + 8), operand(e, d)),
            );
            store(e, k(0x20), c1);
            let c2 = x87_sub(
                x87_mul(load(e, p + 4), operand(e, d)),
                x87_mul(load(e, d + 4), operand(e, p)),
            );
            store(e, k(0x24), c2);
            e.call(POINT_UNITIZE, &args![k(0x1c)]);
            let dot = x87_add(
                x87_add(
                    x87_mul(load(e, p + 4), operand(e, d + 4)),
                    x87_mul(load(e, p), operand(e, d)),
                ),
                x87_mul(load(e, p + 8), operand(e, d + 8)),
            );
            let cosine = x87_load32(x87_store32(dot));
            // FCOM qword -1.0; TEST AH,0x41: below, equal or unordered take pi; then
            // FLD1; FCOMP: 1 below or equal (or unordered) takes 0.
            let angle = if !(cosine > e.mem.f64(MINUS_ONE_DOUBLE)) {
                load(e, PI_FLOAT)
            } else if !(1.0 > cosine) {
                0.0
            } else {
                x87_load32(x87_store32(e.call(CRT_ACOS, &args![cosine]).f64()))
            };
            let angle = x87_quiet_bits(x87_store32(angle));
            let axis = [pushed(e, k(0x1c)), pushed(e, k(0x20)), pushed(e, k(0x24))];
            e.call(
                MATRIX_FROM_ANGLE_AXIS,
                &args![m, angle, axis[0], axis[1], axis[2]],
            );
            let mm = |e: &Engine, i: u32| load(e, m + 4 * i);
            let mo = |e: &Engine, i: u32| operand(e, m + 4 * i);
            let u = e.mem.u32(frame + 4);
            let r0 = x87_add(
                x87_mul(load(e, u + 8), mm(e, 2)),
                x87_add(
                    x87_mul(load(e, u + 4), mm(e, 1)),
                    x87_mul(load(e, u), mm(e, 0)),
                ),
            );
            store(e, k(0x1c), r0);
            let r1 = x87_add(
                x87_add(
                    x87_mul(load(e, u), mm(e, 3)),
                    x87_mul(load(e, u + 4), mm(e, 4)),
                ),
                x87_mul(load(e, u + 8), mo(e, 5)),
            );
            store(e, k(0x20), r1);
            let r1_bits = e.mem.u32(k(0x20));
            e.mem.set_u32(k(0x14), r1_bits);
            let r2 = x87_add(
                x87_add(
                    x87_mul(load(e, u + 4), mo(e, 7)),
                    x87_mul(load(e, u), mo(e, 6)),
                ),
                x87_mul(load(e, u + 8), mo(e, 8)),
            );
            let r0_bits = e.mem.u32(k(0x1c));
            e.mem.set_u32(k(0x10), r0_bits);
            store(e, k(0x24), r2);
            let r2_bits = e.mem.u32(k(0x24));
            e.mem.set_u32(k(0x18), r2_bits);
            let w = e.mem.u32(frame + 8);
            let q0 = x87_add(
                x87_mul(mm(e, 2), load(e, w + 8)),
                x87_add(
                    x87_mul(mm(e, 1), load(e, w + 4)),
                    x87_mul(mm(e, 0), load(e, w)),
                ),
            );
            store(e, k(0x28), q0);
            let q1 = x87_add(
                x87_add(
                    x87_mul(mm(e, 3), load(e, w)),
                    x87_mul(mm(e, 4), operand(e, w + 4)),
                ),
                x87_mul(load(e, w + 8), mo(e, 5)),
            );
            store(e, k(0x2c), q1);
            let q2 = x87_add(
                x87_add(
                    x87_mul(mm(e, 6), operand(e, w)),
                    x87_mul(mm(e, 7), operand(e, w + 4)),
                ),
                x87_mul(load(e, w + 8), mo(e, 8)),
            );
            let q0_bits = e.mem.u32(k(0x28));
            e.mem.set_u32(k(0x1c), q0_bits);
            store(e, k(0x30), q2);
            let q1_bits = e.mem.u32(k(0x2c));
            let q2_bits = e.mem.u32(k(0x30));
            e.mem.set_u32(k(0x24), q2_bits);
            e.mem.set_u32(k(0x20), q1_bits);
        } else if first == 0 {
            let t0 = x87_store32(x87_sub(load(e, target), operand(e, o)));
            let t1 = x87_store32(x87_sub(load(e, target + 4), operand(e, o + 4)));
            let t2 = x87_store32(x87_sub(load(e, target + 8), operand(e, o + 8)));
            e.mem.set_u32(d, t0);
            e.mem.set_u32(d + 4, t1);
            e.mem.set_u32(d + 8, t2);
            // The up vector (0, 0.1, 1).
            e.mem.set_u32(k(0x10), x87_store32(0.0));
            let tenth = pushed(e, TENTH_FLOAT);
            e.mem.set_u32(k(0x14), tenth);
            e.mem.set_u32(k(0x18), x87_store32(1.0));
            e.call(POINT_UNITIZE, &args![k(0x10)]);
            let x = x87_sub(
                x87_mul(load(e, d + 8), load(e, k(0x14))),
                x87_mul(load(e, d + 4), load(e, k(0x18))),
            );
            store(e, k(0x1c), x);
            let y = x87_sub(
                x87_mul(load(e, k(0x18)), load(e, d)),
                x87_mul(load(e, d + 8), load(e, k(0x10))),
            );
            store(e, k(0x20), y);
            let z = x87_sub(
                x87_mul(load(e, k(0x10)), load(e, d + 4)),
                x87_mul(load(e, k(0x14)), operand(e, d)),
            );
            store(e, k(0x24), z);
            e.call(POINT_UNITIZE, &args![k(0x1c)]);
            let x = x87_sub(
                x87_mul(load(e, d + 8), load(e, k(0x20))),
                x87_mul(load(e, d + 4), load(e, k(0x24))),
            );
            let y = x87_sub(
                x87_mul(load(e, k(0x24)), load(e, d)),
                x87_mul(load(e, d + 8), load(e, k(0x1c))),
            );
            let z = x87_sub(
                x87_mul(load(e, k(0x1c)), load(e, d + 4)),
                x87_mul(load(e, k(0x20)), operand(e, d)),
            );
            store(e, k(0x14), y);
            store(e, k(0x10), x);
            store(e, k(0x18), z);
            e.call(POINT_UNITIZE, &args![k(0x10)]);
        } else {
            store(e, k(0x1c), x87_sub(load(e, target), operand(e, o)));
            store(e, k(0x20), x87_sub(load(e, target + 4), operand(e, o + 4)));
            store(e, k(0x24), x87_sub(load(e, target + 8), operand(e, o + 8)));
            e.call(POINT_UNITIZE, &args![k(0x1c)]);
            let s0 = x87_store32(x87_add(load(e, first), operand(e, k(0x1c))));
            let s1 = x87_store32(x87_add(load(e, first + 4), operand(e, k(0x20))));
            let s2 = x87_store32(x87_add(load(e, first + 8), operand(e, k(0x24))));
            e.mem.set_u32(d, s0);
            e.mem.set_u32(d + 4, s1);
            e.mem.set_u32(d + 8, s2);
            e.call(POINT_UNITIZE, &args![d]);
            let p = e.mem.u32(frame);
            let c0 = x87_sub(
                x87_mul(load(e, p + 8), operand(e, d + 4)),
                x87_mul(load(e, d + 8), operand(e, p + 4)),
            );
            store(e, k(0x1c), c0);
            let c1 = x87_sub(
                x87_mul(load(e, d + 8), operand(e, p)),
                x87_mul(load(e, p + 8), operand(e, d)),
            );
            store(e, k(0x20), c1);
            let c2 = x87_sub(
                x87_mul(load(e, d), operand(e, p + 4)),
                x87_mul(load(e, p), operand(e, d + 4)),
            );
            store(e, k(0x24), c2);
            e.call(POINT_UNITIZE, &args![k(0x1c)]);
            let axis = [pushed(e, k(0x1c)), pushed(e, k(0x20)), pushed(e, k(0x24))];
            let dot = x87_add(
                x87_add(
                    x87_mul(load(e, p + 4), operand(e, d + 4)),
                    x87_mul(load(e, p), operand(e, d)),
                ),
                x87_mul(load(e, p + 8), operand(e, d + 8)),
            );
            let cosine = x87_quiet_bits(x87_store32(dot));
            let angle = e.call(CLAMPED_ACOS, &args![cosine]).f64();
            let angle = x87_store32(angle);
            e.call(
                MATRIX_FROM_ANGLE_AXIS,
                &args![m, angle, axis[0], axis[1], axis[2]],
            );
            let u = e.mem.u32(frame + 4);
            let result = e.call(MATRIX_TIMES_POINT, &args![m, k(0x34), u]).u32();
            for i in 0..3 {
                let word = e.mem.u32(result + 4 * i);
                e.mem.set_u32(k(0x10) + 4 * i, word);
            }
            let w = e.mem.u32(frame + 8);
            let result = e.call(MATRIX_TIMES_POINT, &args![m, k(0x34), w]).u32();
            for i in 0..3 {
                let word = e.mem.u32(result + 4 * i);
                e.mem.set_u32(k(0x1c) + 4 * i, word);
            }
        }
        e.call(POINT_UNITIZE, &args![d]);

        let half = x87_mul(x87_load32(size.to_bits()), e.mem.f64(HALF_DOUBLE));
        let h = x87_load32(x87_store32(half));
        let c = corners.addr();
        let q: Vec<f64> = (0..3).map(|i| load(e, k(0x1c) + 4 * i)).collect();
        let a0 = x87_store32(x87_mul(h, q[0]));
        let a1 = x87_store32(x87_mul(q[1], h));
        let a2 = x87_store32(x87_mul(h, q[2]));
        let v = x87_store32(x87_add(load(e, o), x87_m32(a0)));
        e.mem.set_u32(c, v);
        let v = x87_store32(x87_add(x87_load32(a1), operand(e, o + 4)));
        e.mem.set_u32(c + 4, v);
        let v = x87_store32(x87_add(load(e, o + 8), x87_m32(a2)));
        e.mem.set_u32(c + 8, v);
        let b0 = x87_store32(x87_mul(q[0], h));
        let b1 = x87_store32(x87_mul(q[1], h));
        let b2 = x87_store32(x87_mul(q[2], h));
        for (i, b) in [b0, b1, b2].into_iter().enumerate() {
            let i = i as u32;
            let v = x87_store32(x87_sub(load(e, o + 4 * i), x87_m32(b)));
            e.mem.set_u32(c + 0xc + 4 * i, v);
        }
        let r: Vec<f64> = (0..3).map(|i| load(e, k(0x10) + 4 * i)).collect();
        let c0 = x87_store32(x87_mul(h, r[0]));
        let c1 = x87_store32(x87_mul(r[1], h));
        let c2 = x87_store32(x87_mul(h, r[2]));
        let v = x87_store32(x87_add(load(e, o), x87_m32(c0)));
        e.mem.set_u32(c + 0x18, v);
        let v = x87_store32(x87_add(x87_load32(c1), operand(e, o + 4)));
        e.mem.set_u32(c + 0x1c, v);
        let v = x87_store32(x87_add(load(e, o + 8), x87_m32(c2)));
        e.mem.set_u32(c + 0x20, v);
        let d0 = x87_store32(x87_mul(r[0], h));
        let d1 = x87_store32(x87_mul(r[1], h));
        let d2 = x87_store32(x87_mul(h, r[2]));
        for (i, b) in [d0, d1, d2].into_iter().enumerate() {
            let i = i as u32;
            let v = x87_store32(x87_sub(load(e, o + 4 * i), x87_m32(b)));
            e.mem.set_u32(c + 0x24 + 4 * i, v);
        }
        let a = axes.addr();
        for (i, offset) in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24]
            .into_iter()
            .enumerate()
        {
            let word = e.mem.u32(k(offset));
            e.mem.set_u32(a + 4 * i as u32, word);
        }
    });
    true
}

/// `NiObject::GetViewerStrings` (Xbox PDB, `thiscall`, the string array).
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a63260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, then appends
/// `NiGetViewerString` of the class name (the first word of `NiRenderer::ms_RTTI`,
/// `011f4758`) to `strings`: when the count (+0xa) is not below the capacity (+8) the array
/// first grows by `m_usGrowBy` (+0xe, `0096ad30`), then `0096ae90` stores the string held in
/// the `strings` argument's stack word at the old count.
pub fn ni_renderer_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_RENDERER_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        let count = e.mem.u16(strings.addr() + 0xa) as u32;
        let capacity = e.mem.u16(strings.addr() + 8) as u32;
        if count >= capacity {
            let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
            e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
        }
        e.call(ARRAY_SET_AT, &args![strings, count, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a62b50, ni_renderer_get_rtti() -> Ptr),
        entry!(0x00a63230, ni_renderer_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_2b60, fn_00a62b60(Ptr) -> u32),
        entry!(
            0x00a6_2be0,
            fn_00a62be0(Ptr, Ptr, Ptr, f32, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a6_3260, ni_renderer_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_renderer_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a62b50, &args![]).u32(), NI_RENDERER_RTTI);
    }

    #[test]
    fn ni_renderer_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_2b60, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a63230, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a63230, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x280]]);
        assert_eq!(calls_to(&e, 0x00a6_2b60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a62b60_deletes_the_locks_and_releases_both_objects() {
        let mut e = Engine::new();
        e.map(RENDERER_GLOBAL & !0xfff, 0x1000);
        e.mem.set_u32(RENDERER_GLOBAL, 5);
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        // The object at +0x14 keeps a reference, the one at +8 loses its last one.
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            e.mem.u32(a[0]).wrapping_sub(1).into_ret()
        });
        e.register(OBJECT_TAIL, |_, _| 0x99u32.into_ret());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x200);
        let kept = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        for (object, count) in [(kept, 2u32), (last, 1)] {
            e.mem.set_u32(object, 0x0150_0000);
            e.mem.set_u32(object + 4, count);
        }
        e.mem.set_u32(this + 0x14, kept);
        e.mem.set_u32(this + 8, last);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_2b60, &args![this]).u32(), 0x99);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (DELETE_CRITICAL_SECTION, vec![this + 0x180]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x100]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (INTERLOCKED_DECREMENT, vec![kept + 4]),
                (INTERLOCKED_DECREMENT, vec![last + 4]),
                (0x0150_1000, vec![last]),
                (OBJECT_TAIL, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0109_bb94);
        assert_eq!(e.mem.u32(RENDERER_GLOBAL), 0);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// Doubles: the unitize leaves the point as it is, the matrix from angle and axis
    /// writes the identity, the matrix product writes `point + 100` and answers its result,
    /// the arc cosines answer 0.5 and 0.75.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_a000, 0x0101_e000, 0x0109_b000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(MINUS_ONE_DOUBLE, -1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_u32(PI_FLOAT, 0x4049_0fdb);
        e.mem.set_f32(TENTH_FLOAT, 0.1);
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        e.register(MATRIX_FROM_ANGLE_AXIS, |e, a| {
            for i in 0..9 {
                let one = if i % 4 == 0 { 1.0 } else { 0.0 };
                e.mem.set_f32(a[0] + 4 * i, one);
            }
            Ret::default()
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) + 100.0;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(CRT_ACOS, |_, _| 0.5f64.into_ret());
        e.register(CLAMPED_ACOS, |_, _| 0.75f64.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn point(e: &mut Engine, v: [f32; 3]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, value) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *value);
        }
        p
    }

    fn floats(e: &Engine, p: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(p + 4 * i)).collect()
    }

    /// The corners for half-size `h` around `o` along `q` then `r`, as the exe orders its
    /// operations.
    fn corners(o: [f32; 3], q: [f32; 3], r: [f32; 3], h: f32) -> Vec<f32> {
        let mut out = vec![];
        out.extend((0..3).map(|i| o[i] + h * q[i]));
        out.extend((0..3).map(|i| o[i] - q[i] * h));
        out.extend((0..3).map(|i| o[i] + h * r[i]));
        out.extend((0..3).map(|i| o[i] - r[i] * h));
        out
    }

    struct Run {
        e: Engine,
        ok: bool,
        corners: u32,
        axes: u32,
        direction: u32,
    }

    fn run(p: Option<[f32; 3]>, target: Option<[f32; 3]>, origin: [f32; 3]) -> Run {
        let mut e = engine();
        let frame = e.mem.alloc(12);
        if let Some(p) = p {
            let p = point(&mut e, p);
            e.mem.set_u32(frame, p);
        }
        let u = point(&mut e, [1.0, 2.0, 3.0]);
        let w = point(&mut e, [4.0, 5.0, 6.0]);
        e.mem.set_u32(frame + 4, u);
        e.mem.set_u32(frame + 8, w);
        let target = target.map_or(0, |t| point(&mut e, t));
        let origin = point(&mut e, origin);
        let corners = e.mem.alloc(48);
        let axes = e.mem.alloc(24);
        let direction = e.mem.alloc(12);
        let ok = e
            .call(
                0x00a6_2be0,
                &args![frame, origin, target, 4.0f32, corners, axes, direction],
            )
            .bool();
        Run {
            e,
            ok,
            corners,
            axes,
            direction,
        }
    }

    #[test]
    fn without_p_or_target_answers_false() {
        let run = run(None, None, [0.0; 3]);
        assert!(!run.ok);
        assert!(calls_to(&run.e, POINT_UNITIZE).is_empty());
        assert_eq!(floats(&run.e, run.direction, 3), vec![0.0; 3]);
    }

    #[test]
    fn p_alone_rotates_by_the_angle_of_p_with_itself() {
        // |p|^2 = 4: not below 1, angle 0.
        let run = run(Some([2.0, 0.0, 0.0]), None, [10.0, 20.0, 30.0]);
        let e = &run.e;
        assert!(run.ok);
        assert_eq!(floats(e, run.direction, 3), vec![2.0, 0.0, 0.0]);
        let rotation = calls_to(e, MATRIX_FROM_ANGLE_AXIS);
        assert_eq!(rotation[0][1..], [0, 0, 0, 0]);
        // The identity keeps r = u and q = w.
        let r = [1.0, 2.0, 3.0];
        let q = [4.0, 5.0, 6.0];
        assert_eq!(
            floats(e, run.corners, 12),
            corners([10.0, 20.0, 30.0], q, r, 2.0)
        );
        assert_eq!(floats(e, run.axes, 6), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // The cross product was unitized, then the direction.
        let unitized = calls_to(e, POINT_UNITIZE);
        assert_eq!(unitized.len(), 2);
        assert_eq!(unitized[1], vec![run.direction]);
        assert!(calls_to(e, CRT_ACOS).is_empty());
    }

    #[test]
    fn p_alone_takes_acos_inside_the_range_and_pi_below_or_nan() {
        let inside = run(Some([0.5, 0.0, 0.0]), None, [0.0; 3]);
        assert_eq!(calls_to(&inside.e, CRT_ACOS), vec![args![0.25f64]]);
        assert_eq!(
            calls_to(&inside.e, MATRIX_FROM_ANGLE_AXIS)[0][1],
            0.5f32.to_bits()
        );
        let nan = run(Some([f32::NAN, 0.0, 0.0]), None, [0.0; 3]);
        assert!(calls_to(&nan.e, CRT_ACOS).is_empty());
        assert_eq!(calls_to(&nan.e, MATRIX_FROM_ANGLE_AXIS)[0][1], 0x4049_0fdb);
    }

    #[test]
    fn target_alone_builds_the_axes_from_the_up_vector() {
        let run = run(None, Some([11.0, 20.0, 30.0]), [10.0, 20.0, 30.0]);
        let e = &run.e;
        assert!(run.ok);
        assert_eq!(floats(e, run.direction, 3), vec![1.0, 0.0, 0.0]);
        // With the unitize a no-op and direction (1, 0, 0): q = (0, 1, -0.1) and
        // r = (0, -0.1, -1).
        let q = [0.0, 1.0, -0.1f32];
        let r = [0.0, -0.1f32, -1.0];
        assert_eq!(
            floats(e, run.axes, 6),
            vec![r[0], r[1], r[2], q[0], q[1], q[2]]
        );
        assert_eq!(
            floats(e, run.corners, 12),
            corners([10.0, 20.0, 30.0], q, r, 2.0)
        );
        let unitized = calls_to(e, POINT_UNITIZE);
        assert_eq!(unitized.len(), 4);
        assert_eq!(unitized[3], vec![run.direction]);
        assert!(calls_to(e, MATRIX_FROM_ANGLE_AXIS).is_empty());
    }

    #[test]
    fn p_and_target_rotate_u_and_w_by_the_clamped_angle() {
        let run = run(
            Some([0.0, 1.0, 0.0]),
            Some([12.0, 20.0, 30.0]),
            [10.0, 20.0, 30.0],
        );
        let e = &run.e;
        assert!(run.ok);
        // Direction: p + (target - origin) = (2, 1, 0) (the unitize is a no-op).
        assert_eq!(floats(e, run.direction, 3), vec![2.0, 1.0, 0.0]);
        // p . direction = 1.
        assert_eq!(calls_to(e, CLAMPED_ACOS), vec![vec![1.0f32.to_bits()]]);
        let rotation = calls_to(e, MATRIX_FROM_ANGLE_AXIS);
        // Angle 0.75; axis (d0 p1 - p0 d1 last) = (0, 0, 2).
        let axis: Vec<f32> = rotation[0][2..]
            .iter()
            .map(|b| f32::from_bits(*b))
            .collect();
        assert_eq!(rotation[0][1], 0.75f32.to_bits());
        assert_eq!(axis, vec![0.0, 0.0, 2.0]);
        let products = calls_to(e, MATRIX_TIMES_POINT);
        assert_eq!(products.len(), 2);
        assert_eq!(products[0][0], rotation[0][0]);
        assert_eq!(products[0][1], products[1][1]);
        let r = [101.0, 102.0, 103.0];
        let q = [104.0, 105.0, 106.0];
        assert_eq!(
            floats(e, run.axes, 6),
            vec![r[0], r[1], r[2], q[0], q[1], q[2]]
        );
        assert_eq!(
            floats(e, run.corners, 12),
            corners([10.0, 20.0, 30.0], q, r, 2.0)
        );
    }

    #[test]
    fn viewer_strings_append_the_class_name() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_RENDERER_RTTI, 0x0109_b000);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x5000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 5);
        e.call(0x00a6_3260, &args![0x7777u32, strings]);
        assert_eq!(
            calls_to(&e, OBJECT_GET_VIEWER_STRINGS),
            vec![vec![0x7777, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_b000]]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 8]]);
        assert_eq!(calls_to(&e, ARRAY_SET_AT)[0][..2], [strings, 3]);
        // Below the capacity, no growth.
        e.mem.set_u16(strings + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a6_3260, &args![0x7777u32, strings]);
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
    }
}
