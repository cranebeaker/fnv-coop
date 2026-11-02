//! `Havok SDK/hkpiterativelinearcastagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The large float `0x7f7fffee` (`0104da48`) used as "no hit yet".
const NO_HIT: u32 = 0x0104_da48;
/// The double `0.0` (`01012060`) the final test compares against.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The vtable pointer stored first in the local hit records (`010cebe0`).
const RECORD_VTABLE: u32 = 0x010c_ebe0;
/// `hkMotionState::hkMotionState` (Xbox PDB, `00d431b0`, thiscall, one pointer argument).
const MOTION_STATE_COPY: u32 = 0x00d4_31b0;

type Vec4 = [f32; 4];

fn ld(e: &Engine, at: u32) -> Vec4 {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn st(e: &mut Engine, at: u32, v: Vec4) {
    for (i, lane) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

fn copy16(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// An `FLD` of a float in memory (it quiets a signalling NaN).
fn load_float(e: &Engine, at: u32) -> f32 {
    x87_quiet(e.mem.f32(at))
}

/// The dot product of the first three lanes the way the exe sums them: lane 1 plus lane 0,
/// then lane 2 plus that.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2]];
    p[2] + (p[1] + p[0])
}

// Translated from 00daf540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpIterativeLinearCastAgent::linearCast` (Xbox PDB), thiscall `(this, body, other, input,
/// output, early_out)`; the callee pops five words after `this`. Unconfirmed names: `body`,
/// `other`, `input`, `output` and `early_out` are the identities the call pattern suggests.
/// The input is copied with its third float (`input[2]`) raised by `input[0x19]` and the
/// virtual function at slot `0x10` of `this` is asked for the closest point (arguments `body`,
/// `other`, the copy, a local hit record whose byte at +8 reports a hit and whose float at
/// +0x2c holds the distance `d`). With a hit: `early_out` (slot 4 of its vtable, when non-null)
/// is told if `input[2] > d`. With `s = dot(input + 0x50, record + 0x20)`, the cast returns
/// when `d + s > 0` or `s + input[0x18]` is not below zero. Otherwise, when the first float of
/// the list at `input[0x1a]` is not below `d`, the hit fraction `d / (d - (d + s))` (zero when
/// `d <= 0`) goes to `output` through slot 4 of its vtable unless it exceeds `output[1]`.
/// When it is below `d`, the cast iterates: a local `hkMotionState` is built from `body[2]`,
/// and for each list entry (count at `list[1]`, counting down) the state is moved along the
/// cast by the current fraction, the closest point is asked again, and the fraction is
/// refined by `d2 / -dot(input + 0x50, record2 + 0x20)`; the loop ends when the refinement
/// exceeds the allowed distance or `output[1]`, or when the list's first float is not below
/// `d2`; the last hit goes to `output` (slot 4).
///
/// Uninitialised stack words: the second hit record (`second` here, at `f + 0x84`) is only
/// initialised in its vtable word, hit byte and the two floats at +4 and +0x2c; the loop
/// reads its normal (`second + 0x20`, `MOVAPS` at `00daf7d1`) without testing the hit byte, so
/// when the closest-point query misses on the first pass the exe uses stack garbage there (its
/// position at +0x10 can reach `hit` through the copies at `00daf872` the same way). Zeros
/// stand in, which ends the loop (`slope < 0` fails); a garbage normal can take other paths.
// Translation incomplete: 00daf540 - reads uninitialised stack word at ESP+0xb0 (second hit record normal, 00daf7d1)
pub fn hkp_iterative_linear_cast_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body: Ptr,
    other: Ptr,
    input: Ptr,
    output: Ptr,
    early_out: Ptr,
) {
    let t = this.addr();
    let (a1, a2) = (body.addr(), other.addr());
    let inp = input.addr();
    let (out4, out5) = (output.addr(), early_out.addr());
    e.with_stack(0x200, |e, frame| {
        let f = frame.addr();
        let record = f + 0x54;
        let hit = f + 0x14;
        let copy = f + 0xc4;
        let motion = f + 0x114;
        let big = load_float(e, NO_HIT);
        let raise = load_float(e, inp + 0x64);
        for i in 0..20 {
            let word = e.mem.u32(inp + 4 * i);
            e.mem.set_u32(copy + 4 * i, word);
        }
        let raised = (f64::from(raise) + f64::from(load_float(e, copy + 8))) as f32;
        e.mem.set_f32(copy + 8, raised);
        e.mem.set_f32(record + 0x2c, big);
        e.mem.set_f32(record + 4, big);
        e.mem.set_u32(record, RECORD_VTABLE);
        e.mem.set_u8(record + 8, 0);
        e.vcall(t, 0x10, &args![a1, a2, copy, record]);
        if e.mem.u8(record + 8) == 0 {
            return;
        }
        let mut d = load_float(e, record + 0x2c);
        copy16(e, record + 0x10, hit);
        copy16(e, record + 0x20, hit + 0x10);
        e.mem.set_u32(hit + 0x20, a1);
        e.mem.set_u32(hit + 0x24, a2);
        let limit = load_float(e, inp + 8);
        if limit > d && out5 != 0 {
            e.vcall(out5, 4, &args![hit]);
            d = load_float(e, record + 0x2c);
        }
        let direction = ld(e, inp + 0x50);
        st(e, f + 0xb4, direction);
        let s = x87_quiet(dot3(direction, ld(e, record + 0x20)));
        let f1 = (f64::from(d) + f64::from(s)) as f32;
        if f1 > 0.0 {
            return;
        }
        let u = load_float(e, inp + 0x60);
        let reach = f64::from(s) + f64::from(u);
        let below_zero = reach < e.mem.f64(ZERO_DOUBLE);
        if !below_zero {
            return;
        }
        let list = e.mem.u32(inp + 0x68);
        if load_float(e, list) >= d {
            if 0.0 < d {
                let fraction = (f64::from(d) / (f64::from(d) - f64::from(f1))) as f32;
                if load_float(e, out4 + 4) < fraction {
                    return;
                }
                e.mem.set_f32(hit + 0x1c, fraction);
            } else {
                e.mem.set_f32(hit + 0x1c, 0.0);
            }
            e.vcall(out4, 4, &args![hit]);
            return;
        }
        let fraction = (f64::from(d) / (f64::from(d) - f64::from(f1))) as f32;
        e.mem.set_f32(hit + 0x1c, fraction);
        let second = f + 0x84;
        e.mem.set_u32(second, RECORD_VTABLE);
        e.mem.set_u8(second + 8, 0);
        e.mem.set_f32(second + 0x2c, big);
        e.mem.set_f32(second + 4, big);
        let source_motion = e.mem.u32(a1 + 8);
        e.call(MOTION_STATE_COPY, &args![motion, source_motion]);
        let body_record = f + 0x44;
        let first_word = e.mem.u32(a1);
        e.mem.set_u32(body_record + 0xc, a1);
        e.mem.set_u32(body_record + 8, motion);
        e.mem.set_u32(body_record, first_word);
        e.mem.set_u32(body_record + 4, 0xffff_ffff);
        let mut remaining = (e.mem.u32(list + 4) as i32).wrapping_sub(1);
        if remaining >= 0 {
            let mut step = ld(e, f + 0xb4);
            loop {
                e.mem.set_f32(second + 0x2c, big);
                e.mem.set_f32(second + 4, big);
                let current = load_float(e, hit + 0x1c);
                e.mem.set_u8(second + 8, 0);
                let state = e.mem.u32(a1 + 8);
                let position = ld(e, state + 0x30);
                let moved = [0, 1, 2, 3].map(|i| current * step[i] + position[i]);
                st(e, motion + 0x30, moved);
                e.vcall(t, 0x10, &args![body_record, a2, copy, second]);
                step = ld(e, inp + 0x50);
                let slope = x87_quiet(dot3(step, ld(e, second + 0x20)));
                let negative = slope < 0.0;
                if !negative {
                    return;
                }
                let drop = -slope;
                let product = (f64::from(current) * f64::from(drop)) as f32;
                let d2 = load_float(e, second + 0x2c);
                if f64::from(product) + f64::from(d2) > f64::from(drop) {
                    return;
                }
                let refined = (f64::from(current) + f64::from(d2) / f64::from(drop)) as f32;
                if load_float(e, out4 + 4) < refined {
                    return;
                }
                copy16(e, second + 0x20, hit + 0x10);
                e.mem.set_f32(hit + 0x1c, refined);
                copy16(e, second + 0x10, hit);
                let list_now = e.mem.u32(inp + 0x68);
                let first = load_float(e, list_now);
                if first < d2 {
                    remaining -= 1;
                    if remaining >= 0 {
                        continue;
                    }
                }
                break;
            }
        }
        e.vcall(out4, 4, &args![hit]);
    });
}

/// The x87 status bits (`AH` after `FNSTSW`) of an `FCOM` of `st0` with `operand`: `0x01`
/// (C0) below, `0x40` (C3) equal, `0x45` unordered, 0 above.
fn fcom_status(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0
    }
}

/// `JP` after `TEST AH,0x5`: the masked bits have even parity (none or both).
fn parity_even(status: u8) -> bool {
    (status & 5).count_ones() % 2 == 0
}

/// The product-sum `scale * direction + position` of `MULPS`/`ADDPS`, lane by lane.
fn step_lanes(scale: f32, direction: Vec4, position: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| scale * direction[i] + position[i])
}

// Translated from 00daf8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpIterativeLinearCastAgent::staticLinearCast` (Xbox PDB), cdecl `(body, other, input,
/// collector, start_collector)` (five words; the names are the roles the code gives them,
/// unconfirmed). The closest-point function is taken from the dispatcher at `input[0]`: the
/// byte at `dispatcher + (word(body[0] + 0xc) + 0xd) * 0x20 + word(other[0] + 0xc)` indexes
/// 0x14-byte records from `dispatcher + 0x9a8`, whose first word is the function (cdecl, four
/// words). It is called with a copy of the 0x50-byte input whose third float is raised by
/// `input[0x19]` and a local hit record (vtable `010cebe0`, distance `d` at +0x2c); without a
/// hit the cast ends. The output record (the hit's position and normal, `body`, `other`) goes
/// to `start_collector` (slot 4, when non-null) when `input[2] > d`. With
/// `s = dot(input + 0x50, normal)` and `f1 = d + s`, the cast ends unless `f1 <= 0` and
/// `s + input[0x18] < 0.0` (both pass on NaN). When the first float of the list at
/// `input[0x1a]` is not below `d`, the record goes to `collector` (slot 4) with its fraction
/// set to 0 when `d <= 0` (or NaN), else only when `collector[1] * (d - f1)` is not below `d`,
/// and then also with fraction 0: the exe stores the `FLDZ` there, it never divides on this path.
/// Otherwise it iterates: fraction `d / (d - f1)`, a local `hkMotionState` copied from
/// `body[2]` (`00d431b0`) under a local body record (`body[0]`, key `-1`, the state, `body`),
/// and for each list entry (count `list[1]`, counting down) the state's position is moved to
/// `fraction * direction + body[2]'s position`, the closest point is asked again (a miss ends
/// the cast without a report), the hit replaces the record with the current fraction, and
/// with `n = -dot(direction, new normal)`: the cast ends when `n` is not above 0 or
/// `fraction * n + d2 > n`; it reports when the list's first float is not below `d2`; else
/// the fraction becomes `fraction + d2 / n` and the cast ends when that exceeds
/// `collector[1]`. Running out of entries reports the record.
pub fn hkp_iterative_linear_cast_agent_static_linear_cast(
    e: &mut Engine,
    body: Ptr,
    other: Ptr,
    input: Ptr,
    collector: Ptr,
    start_collector: Ptr,
) {
    let (p1, p2) = (body.addr(), other.addr());
    let inp = input.addr();
    let (out4, out5) = (collector.addr(), start_collector.addr());
    // `s` below is the 16-byte-aligned stack frame after the three register pushes.
    e.with_stack(0x1d0, |e, frame| {
        let s = frame.addr();
        let record = s + 0x90;
        let hit = s + 0x60;
        let copy = s + 0xd0;
        let second = s + 0x30;
        let body_record = s + 0x20;
        let motion = s + 0x120;
        // 00daf8ec: FLD [0104da48], FST/FSTP into the record's two distance floats.
        let big = x87_store32(x87_load32(e.mem.u32(NO_HIT)));
        e.mem.set_u32(record + 0x2c, big);
        e.mem.set_u32(record + 4, big);
        let dispatcher = e.mem.u32(inp);
        let raise = x87_load32(e.mem.u32(inp + 0x64));
        for i in 0..20 {
            let word = e.mem.u32(inp + 4 * i);
            e.mem.set_u32(copy + 4 * i, word);
        }
        let raised = x87_add(raise, x87_m32(e.mem.u32(copy + 8)));
        e.mem.set_u32(copy + 8, x87_store32(raised));
        e.mem.set_u32(record, RECORD_VTABLE);
        e.mem.set_u8(record + 8, 0);
        let type_a = e.mem.u32(e.mem.u32(p1) + 0xc);
        let type_b = e.mem.u32(e.mem.u32(p2) + 0xc);
        let index_at = (type_a.wrapping_add(0xd) << 5)
            .wrapping_add(type_b)
            .wrapping_add(dispatcher);
        let index = u32::from(e.mem.u8(index_at));
        let function = e
            .mem
            .u32(dispatcher.wrapping_add(index * 0x14).wrapping_add(0x9a8));
        e.call(function, &args![p1, p2, copy, record]);
        if e.mem.u8(record + 8) == 0 {
            return;
        }
        // 00daf989
        let mut d = x87_load32(e.mem.u32(record + 0x2c));
        copy16(e, record + 0x20, hit + 0x10);
        let limit = x87_load32(e.mem.u32(inp + 8));
        e.mem.set_u32(hit + 0x20, p1);
        e.mem.set_u32(hit + 0x24, p2);
        copy16(e, record + 0x10, hit);
        if fcom_status(limit, d) & 0x41 == 0 && out5 != 0 {
            e.vcall(out5, 4, &args![hit]);
            d = x87_load32(e.mem.u32(record + 0x2c));
        }
        // 00daf9e0
        let direction = ld(e, inp + 0x50);
        st(e, s + 0xc0, direction);
        let slope_s = x87_load32(dot3(direction, ld(e, record + 0x20)).to_bits());
        let f1 = x87_load32(x87_store32(x87_add(d, slope_s)));
        if fcom_status(f1, 0.0) & 0x41 == 0 {
            return;
        }
        // 00dafa33
        let reach = x87_add(slope_s, x87_load32(e.mem.u32(inp + 0x60)));
        if fcom_status(reach, e.mem.f64(ZERO_DOUBLE)) & 1 == 0 {
            return;
        }
        let list = e.mem.u32(inp + 0x68);
        if fcom_status(x87_load32(e.mem.u32(list)), d) & 1 == 0 {
            // 00dafa59
            if !parity_even(fcom_status(0.0, d)) {
                // 00dafa65
                let product = x87_mul(x87_sub(d, f1), x87_load32(e.mem.u32(out4 + 4)));
                let q = x87_load32(x87_store32(product));
                if !parity_even(fcom_status(q, d)) {
                    return;
                }
            }
            // 00dafa87 / 00dafa9e: the FLDZ goes to the fraction.
            e.mem.set_u32(hit + 0x1c, x87_store32(0.0));
            e.vcall(out4, 4, &args![hit]);
            return;
        }
        // 00dafab9
        let source_motion = e.mem.u32(p1 + 8);
        e.mem.set_u32(second, RECORD_VTABLE);
        let mut fraction = x87_store32(x87_div(d, x87_sub(d, f1)));
        e.mem.set_u8(second + 8, 0);
        let big = x87_store32(x87_load32(e.mem.u32(NO_HIT)));
        e.mem.set_u32(second + 0x2c, big);
        e.mem.set_u32(second + 4, big);
        e.call(MOTION_STATE_COPY, &args![motion, source_motion]);
        let first_word = e.mem.u32(p1);
        e.mem.set_u32(body_record + 0xc, p1);
        e.mem.set_u32(body_record + 8, motion);
        e.mem.set_u32(body_record, first_word);
        e.mem.set_u32(body_record + 4, 0xffff_ffff);
        let mut remaining = (e.mem.u32(list + 4) as i32).wrapping_sub(1);
        if remaining < 0 {
            e.vcall(out4, 4, &args![hit]);
            return;
        }
        // 00dafb17: the direction copy, then input + 0x50 again from the second pass on.
        let mut step = ld(e, s + 0xc0);
        loop {
            // 00dafb20
            let big = x87_store32(x87_load32(e.mem.u32(NO_HIT)));
            let state = e.mem.u32(p1 + 8);
            e.mem.set_u32(second + 0x2c, big);
            e.mem.set_u8(second + 8, 0);
            e.mem.set_u32(second + 4, big);
            let position = ld(e, state + 0x30);
            let moved = step_lanes(f32::from_bits(fraction), step, position);
            st(e, motion + 0x30, moved);
            e.call(function, &args![body_record, p2, copy, second]);
            if e.mem.u8(second + 8) == 0 {
                return;
            }
            // 00dafb7d
            let current = x87_load32(fraction);
            let normal = ld(e, second + 0x20);
            step = ld(e, inp + 0x50);
            copy16(e, second + 0x10, hit);
            st(e, hit + 0x10, normal);
            e.mem.set_u32(hit + 0x1c, x87_store32(current));
            let slope = x87_load32(dot3(step, normal).to_bits());
            if fcom_status(slope, 0.0) & 1 == 0 {
                return;
            }
            // 00dafbd4: FCHS, through the stack slot.
            let drop = x87_load32(x87_store32(-slope));
            let product = x87_load32(x87_store32(x87_mul(drop, current)));
            let d2 = x87_load32(e.mem.u32(second + 0x2c));
            if fcom_status(x87_add(product, d2), drop) & 0x41 == 0 {
                return;
            }
            // 00dafbfd
            let first = x87_load32(e.mem.u32(e.mem.u32(inp + 0x68)));
            if fcom_status(first, d2) & 1 == 0 {
                break;
            }
            // 00dafc0b
            fraction = x87_store32(x87_add(current, x87_div(d2, drop)));
            let refined = x87_load32(fraction);
            let allowed = x87_load32(e.mem.u32(out4 + 4));
            if !parity_even(fcom_status(allowed, refined)) {
                return;
            }
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
        // 00dafc2f / 00dafc4b
        e.vcall(out4, 4, &args![hit]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00da_f540,
            hkp_iterative_linear_cast_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00da_f8e0,
            hkp_iterative_linear_cast_agent_static_linear_cast(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    const FIRST_FINDER: u32 = 0x00ee_1000;
    const COLLECTOR: u32 = 0x00ee_1001;
    const EARLY_OUT: u32 = 0x00ee_1002;

    /// What a collector callback saw: the hit record's vectors and the two words.
    type Seen = Rc<RefCell<Vec<(Vec4, Vec4, u32, u32)>>>;
    type Finder = Box<dyn Fn(&mut Engine, &[u32], u32)>;

    struct Rig {
        e: Engine,
        this: u32,
        body: u32,
        other: u32,
        input: u32,
        output: u32,
        early: u32,
        collected: Seen,
        early_calls: Rc<Cell<u32>>,
    }

    fn object_with_slot(e: &mut Engine, offset: u32, address: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + offset, address);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    /// `finder` runs for each closest-point request, with the request's words and its number.
    fn rig(list_first: f32, list_count: u32, finder: Finder) -> Rig {
        let mut e = test_engine();
        e.mem.set_f32(NO_HIT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let this = object_with_slot(&mut e, 0x10, FIRST_FINDER);
        let calls = Rc::new(Cell::new(0u32));
        let counter = Rc::clone(&calls);
        e.register_double(FIRST_FINDER, move |e, a| {
            let n = counter.get();
            counter.set(n + 1);
            finder(e, a, n);
            Ret::default()
        });
        let output = object_with_slot(&mut e, 4, COLLECTOR);
        e.mem.set_f32(output + 4, 1.0);
        let collected: Seen = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&collected);
        e.register_double(COLLECTOR, move |e, a| {
            let h = a[1];
            let record = (
                ld(e, h),
                ld(e, h + 0x10),
                e.mem.u32(h + 0x20),
                e.mem.u32(h + 0x24),
            );
            seen.borrow_mut().push(record);
            Ret::default()
        });
        let early = object_with_slot(&mut e, 4, EARLY_OUT);
        let early_calls = Rc::new(Cell::new(0u32));
        let early_counter = Rc::clone(&early_calls);
        e.register_double(EARLY_OUT, move |_, _| {
            early_counter.set(early_counter.get() + 1);
            Ret::default()
        });
        let motion = e.mem.alloc(0x40);
        st(&mut e, motion + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let shape = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        let other = e.mem.alloc(0x20);
        let list = e.mem.alloc(0x10);
        e.mem.set_f32(list, list_first);
        e.mem.set_u32(list + 4, list_count);
        let input = e.mem.alloc(0x80);
        e.mem.set_f32(input + 8, 2.0);
        st(&mut e, input + 0x50, [0.0, 0.0, -1.0, 0.0]);
        e.mem.set_f32(input + 0x60, 0.25);
        e.mem.set_f32(input + 0x64, 0.0);
        e.mem.set_u32(input + 0x68, list);
        Rig {
            e,
            this,
            body,
            other,
            input,
            output,
            early,
            collected,
            early_calls,
        }
    }

    fn run(r: &mut Rig, early: u32) {
        r.e.call(
            0x00da_f540,
            &args![r.this, r.body, r.other, r.input, r.output, early],
        );
    }

    /// A finder that reports a hit with distance `d` in the first request and `d2` after.
    fn hit_at(d: f32, d2: f32) -> Finder {
        Box::new(move |e, a, n| {
            let record = a[4];
            if n == 0 {
                e.mem.set_u8(record + 8, 1);
                st(e, record + 0x20, [0.0, 0.0, 1.0, d]);
                st(e, record + 0x10, [7.0, 7.0, 7.0, 7.0]);
            } else {
                st(e, record + 0x20, [0.0, 0.0, 1.0, d2]);
                st(e, record + 0x10, [8.0, 8.0, 8.0, 8.0]);
            }
        })
    }

    #[test]
    fn no_hit_leaves_without_a_callback() {
        let mut r = rig(2.0, 1, Box::new(|_, _, _| {}));
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
        assert_eq!(r.early_calls.get(), 0);
    }

    #[test]
    fn a_direct_hit_reports_the_fraction_and_tells_the_early_out() {
        let mut r = rig(2.0, 1, hit_at(0.5, 0.0));
        let early = r.early;
        run(&mut r, early);
        // d = 0.5, s = -1: the fraction is 0.5 / (0.5 - (-0.5)).
        let seen = r.collected.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [7.0; 4]);
        assert_eq!(seen[0].1, [0.0, 0.0, 1.0, 0.5]);
        assert_eq!(seen[0].2, r.body);
        assert_eq!(seen[0].3, r.other);
        // input[2] (2.0) exceeds d, so the early-out listener is told once.
        assert_eq!(r.early_calls.get(), 1);
    }

    #[test]
    fn a_hit_at_or_behind_the_start_reports_fraction_zero_and_a_far_one_is_dropped() {
        let mut r = rig(2.0, 1, hit_at(-0.2, 0.0));
        run(&mut r, 0);
        {
            let seen = r.collected.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].1[3], 0.0);
        }
        assert_eq!(r.early_calls.get(), 0);
        // The fraction 0.5 exceeds the collector's limit, so nothing is reported.
        let mut r = rig(2.0, 1, hit_at(0.5, 0.0));
        let output = r.output;
        r.e.mem.set_f32(output + 4, 0.25);
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
    }

    #[test]
    fn a_hit_moving_away_is_ignored() {
        // d + s > 0: d = 3.0 with s = -1 is still positive.
        let mut r = rig(5.0, 1, hit_at(3.0, 0.0));
        run(&mut r, 0);
        assert!(r.collected.borrow().is_empty());
    }

    #[test]
    fn the_iteration_moves_the_state_and_reports_the_refined_fraction() {
        type Moved = Rc<RefCell<Option<(Vec4, u32, bool)>>>;
        let moved: Moved = Rc::new(RefCell::new(None));
        let sink = Rc::clone(&moved);
        let finder = hit_at(0.5, 0.1);
        let mut r = rig(
            0.25,
            1,
            Box::new(move |e, a, n| {
                if n == 1 {
                    // The state copy sits right after the 80-byte input copy.
                    let state = a[3] + 0x50;
                    let body_record = a[1];
                    let seen = (
                        ld(e, state + 0x30),
                        e.mem.u32(body_record + 4),
                        e.mem.u32(body_record + 8) == state,
                    );
                    *sink.borrow_mut() = Some(seen);
                }
                finder(e, a, n);
            }),
        );
        let motion_copies = Rc::new(Cell::new(0u32));
        let counter = Rc::clone(&motion_copies);
        r.e.register_double(MOTION_STATE_COPY, move |_, _| {
            counter.set(counter.get() + 1);
            Ret::default()
        });
        run(&mut r, 0);
        assert_eq!(motion_copies.get(), 1);
        // 0.5 * (0, 0, -1) + (1, 2, 3).
        let (position, key, linked) = moved.borrow().unwrap();
        assert_eq!(position, [1.0, 2.0, 2.5, 0.0]);
        assert_eq!(key, 0xffff_ffff);
        assert!(linked);
        // d2 = 0.1, slope -1: the refined fraction is 0.5 + 0.1.
        let seen = r.collected.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [8.0; 4]);
        assert!((seen[0].1[3] - 0.6).abs() < 1e-6);
    }
}

#[cfg(test)]
mod static_cast_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const FINDER: u32 = 0x00ee_2000;
    const COLLECTOR: u32 = 0x00ee_2001;
    const START_COLLECTOR: u32 = 0x00ee_2002;

    /// What the collector saw: the record's position, its normal (fraction in lane 3), the two bodies.
    type Seen = Rc<RefCell<Vec<(Vec4, Vec4, u32, u32)>>>;
    /// The finder's behaviour: the request's words and the request's number.
    type Finder = Box<dyn Fn(&mut Engine, &[u32], u32)>;

    struct Rig {
        e: Engine,
        body: u32,
        other: u32,
        input: u32,
        collector: u32,
        start: u32,
        seen: Seen,
        started: Rc<Cell<u32>>,
        motion_copies: Rc<Cell<u32>>,
    }

    fn object_with_slot(e: &mut Engine, offset: u32, address: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + offset, address);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    fn rig(list_first: f32, list_count: u32, finder: Finder) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(NO_HIT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        // Dispatcher: shape types 1 and 2 pick byte 3 at +0x1c2, record 3's function at +0x9e4.
        let dispatcher = e.mem.alloc(0xa00);
        e.mem.set_u8(dispatcher + 0x1c2, 3);
        e.mem.set_u32(dispatcher + 0x9e4, FINDER);
        let calls = Rc::new(Cell::new(0u32));
        let counter = Rc::clone(&calls);
        e.register_double(FINDER, move |e, a| {
            let n = counter.get();
            counter.set(n + 1);
            finder(e, a, n);
            Ret::default()
        });
        let collector = object_with_slot(&mut e, 4, COLLECTOR);
        e.mem.set_f32(collector + 4, 1.0);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(COLLECTOR, move |e, a| {
            let h = a[1];
            let record = (
                ld(e, h),
                ld(e, h + 0x10),
                e.mem.u32(h + 0x20),
                e.mem.u32(h + 0x24),
            );
            sink.borrow_mut().push(record);
            Ret::default()
        });
        let start = object_with_slot(&mut e, 4, START_COLLECTOR);
        let started = Rc::new(Cell::new(0u32));
        let started_counter = Rc::clone(&started);
        e.register_double(START_COLLECTOR, move |_, _| {
            started_counter.set(started_counter.get() + 1);
            Ret::default()
        });
        let motion_copies = Rc::new(Cell::new(0u32));
        let copies = Rc::clone(&motion_copies);
        e.register_double(MOTION_STATE_COPY, move |_, _| {
            copies.set(copies.get() + 1);
            Ret::default()
        });
        let motion = e.mem.alloc(0x40);
        st(&mut e, motion + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let shape_a = e.mem.alloc(0x10);
        e.mem.set_u32(shape_a + 0xc, 1);
        let shape_b = e.mem.alloc(0x10);
        e.mem.set_u32(shape_b + 0xc, 2);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body, shape_a);
        e.mem.set_u32(body + 8, motion);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, shape_b);
        let list = e.mem.alloc(0x10);
        e.mem.set_f32(list, list_first);
        e.mem.set_u32(list + 4, list_count);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, dispatcher);
        e.mem.set_f32(input + 8, 2.0);
        st(&mut e, input + 0x50, [0.0, 0.0, -1.0, 0.0]);
        e.mem.set_f32(input + 0x60, 0.25);
        e.mem.set_f32(input + 0x64, 0.5);
        e.mem.set_u32(input + 0x68, list);
        Rig {
            e,
            body,
            other,
            input,
            collector,
            start,
            seen,
            started,
            motion_copies,
        }
    }

    fn run(r: &mut Rig, start: u32) {
        r.e.call(
            0x00da_f8e0,
            &args![r.body, r.other, r.input, r.collector, start],
        );
    }

    /// A finder reporting distance `d` first and `d2` after (`None`: the second request misses).
    fn hit_at(d: f32, d2: Option<f32>) -> Finder {
        Box::new(move |e, a, n| {
            let record = a[3];
            if n == 0 {
                e.mem.set_u8(record + 8, 1);
                st(e, record + 0x20, [0.0, 0.0, 1.0, d]);
                st(e, record + 0x10, [7.0; 4]);
            } else if let Some(d2) = d2 {
                e.mem.set_u8(record + 8, 1);
                st(e, record + 0x20, [0.0, 0.0, 1.0, d2]);
                st(e, record + 0x10, [8.0; 4]);
            }
        })
    }

    #[test]
    fn a_miss_reports_nothing_and_the_query_sees_the_raised_copy() {
        let raised = Rc::new(Cell::new(0.0f32));
        let sink = Rc::clone(&raised);
        let mut r = rig(
            2.0,
            1,
            Box::new(move |e, a, _| {
                sink.set(e.mem.f32(a[2] + 8));
            }),
        );
        let start = r.start;
        run(&mut r, start);
        // input[2] 2.0 raised by input[0x19] 0.5.
        assert_eq!(raised.get(), 2.5);
        assert!(r.seen.borrow().is_empty());
        assert_eq!(r.started.get(), 0);
    }

    #[test]
    fn the_direct_path_reports_fraction_zero_or_drops_the_hit() {
        // d = 0.5, s = -1, f1 = -0.5: (d - f1) * 1.0 is not below d, so fraction 0 is reported.
        let mut r = rig(2.0, 1, hit_at(0.5, None));
        let start = r.start;
        run(&mut r, start);
        {
            let seen = r.seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].0, [7.0; 4]);
            assert_eq!(seen[0].1, [0.0, 0.0, 1.0, 0.0]);
            assert_eq!((seen[0].2, seen[0].3), (r.body, r.other));
        }
        // input[2] (2.0) exceeds d: the start collector is told once.
        assert_eq!(r.started.get(), 1);
        // A collector limit of 0.25 makes (d - f1) * 0.25 below d: nothing.
        let mut r = rig(2.0, 1, hit_at(0.5, None));
        let collector = r.collector;
        r.e.mem.set_f32(collector + 4, 0.25);
        run(&mut r, 0);
        assert!(r.seen.borrow().is_empty());
        // d <= 0 reports fraction 0 whatever the limit.
        let mut r = rig(2.0, 1, hit_at(-0.25, None));
        let collector = r.collector;
        r.e.mem.set_f32(collector + 4, 0.0);
        run(&mut r, 0);
        assert_eq!(r.seen.borrow().len(), 1);
        // d + s > 0: moving away, nothing.
        let mut r = rig(5.0, 1, hit_at(3.0, None));
        run(&mut r, 0);
        assert!(r.seen.borrow().is_empty());
    }

    #[test]
    fn the_iteration_moves_the_state_and_reports_the_last_fraction() {
        type Moved = Rc<RefCell<Option<(Vec4, u32)>>>;
        let moved: Moved = Rc::new(RefCell::new(None));
        let sink = Rc::clone(&moved);
        let finder = hit_at(0.5, Some(0.1));
        let mut r = rig(
            0.25,
            1,
            Box::new(move |e, a, n| {
                if n == 1 {
                    let body_record = a[0];
                    let state = e.mem.u32(body_record + 8);
                    *sink.borrow_mut() = Some((ld(e, state + 0x30), e.mem.u32(body_record + 4)));
                }
                finder(e, a, n);
            }),
        );
        run(&mut r, 0);
        assert_eq!(r.motion_copies.get(), 1);
        // fraction 0.5 / (0.5 + 0.5); 0.5 * (0, 0, -1) + (1, 2, 3).
        let (position, key) = moved.borrow().unwrap();
        assert_eq!(position, [1.0, 2.0, 2.5, 0.0]);
        assert_eq!(key, 0xffff_ffff);
        // The list value 0.25 is not below d2 = 0.1: the second hit goes out with fraction 0.5.
        let seen = r.seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [8.0; 4]);
        assert_eq!(seen[0].1, [0.0, 0.0, 1.0, 0.5]);
    }

    #[test]
    fn the_iteration_refines_until_the_entries_or_the_limit_run_out() {
        // d2 = 0.4: refined 0.5 + 0.4 = 0.9 within 1.0; the one entry is used up, so the
        // record (fraction still 0.5) is reported.
        let mut r = rig(0.25, 1, hit_at(0.5, Some(0.4)));
        run(&mut r, 0);
        assert_eq!(r.seen.borrow().len(), 1);
        assert_eq!(r.seen.borrow()[0].1[3], 0.5);
        // A limit of 0.8 is below 0.9: nothing.
        let mut r = rig(0.25, 1, hit_at(0.5, Some(0.4)));
        let collector = r.collector;
        r.e.mem.set_f32(collector + 4, 0.8);
        run(&mut r, 0);
        assert!(r.seen.borrow().is_empty());
        // A miss in the iteration ends without a report.
        let mut r = rig(0.25, 1, hit_at(0.5, None));
        run(&mut r, 0);
        assert!(r.seen.borrow().is_empty());
        // No entries: the first hit goes out as it is (its normal lane 3 still d = 0.75).
        let mut r = rig(0.25, 0, hit_at(0.75, None));
        run(&mut r, 0);
        assert_eq!(r.motion_copies.get(), 1);
        let seen = r.seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, [7.0; 4]);
        assert_eq!(seen[0].1[3], 0.75);
    }
}
