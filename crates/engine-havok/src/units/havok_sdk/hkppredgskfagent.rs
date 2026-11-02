//! `Havok SDK/hkppredgskfagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use std::cmp::Ordering;

use super::hkppredgskcylinderagent3::{copy_rows, transform_points};
use super::hkpsphereshape::{
    load_bits, load_vec, profile_record, store_bits, store_vec, sum3, zip_with, Vec4, PROFILE_END,
};

/// `hkSweptTransformUtil::lerp2` (`00cf1510`, cdecl: the swept transform plus `0x40`, the time
/// as float bits, the 0x40-byte output).
const LERP_TRANSFORM: u32 = 0x00cf_1510;
/// `hkpGskBaseAgent::calcSeparatingNormal` (`00dac020`, cdecl: the two body blocks, a float, the
/// cache block, the vector).
const CALC_SEPARATING_NORMAL: u32 = 0x00da_c020;
/// `hkpGsk::exitAndExportCacheImpl` (`00da9790`, `this` = the cache block, one word, `RET 4`).
const EXIT_AND_EXPORT_CACHE: u32 = 0x00da_9790;
/// `hkpGskfAgent::processCollisionNoTim` (`00daccd0`, `this` = the agent, four words,
/// `RET 0x10`).
const PROCESS_NO_TIM: u32 = 0x00da_ccd0;
/// `hk4dGskCollideCalcToi` (`00db60c0`, cdecl: the input block, three floats, two words of the
/// agent, the output).
const COLLIDE_CALC_TOI: u32 = 0x00db_60c0;
/// The manifold finish (`00dae3a0`, cdecl `(contacts, work points, 0, output, agent word 2)`).
const MANIFOLD_FINISH: u32 = 0x00da_e3a0;
/// `hkGskManifold_cleanup` (`00dad640`, cdecl `(contacts, agent word 2, output word 1)`).
const MANIFOLD_CLEANUP: u32 = 0x00da_d640;
/// The profiling label `processCollision` starts with (`TtGsk`); it ends with `PROFILE_END`.
const PROFILE_BEGIN: u32 = 0x010c_f22c;
/// The double `0.5` the first clamp test multiplies by.
const HALF_F64: u32 = 0x0101_1588;

/// The frame is addressed like the exe's: an offset from the stack pointer once the frame is
/// set up and the three registers pushed, which is `0xC` below the frame's base.
const fn esp(offset: u32) -> u32 {
    offset - 0xc
}

/// The second body's block `(shape, word 1, swept transform output, body)`.
const BLOCK_B: u32 = esp(0x20);
/// The first body's block.
const BLOCK_A: u32 = esp(0x30);
/// The relative velocity vector (the clamp is its fourth lane).
const VELOCITY: u32 = esp(0x40);
/// The second body's swept transform at the query time, and, once the time-of-impact path
/// starts, the five-word input block of the time-of-impact step.
const LERP_B: u32 = esp(0x60);
const TOI_INPUT: u32 = esp(0x60);
/// The clamp and the velocity copy of the time-of-impact input.
const TOI_CLAMP: u32 = esp(0xc0);
const TOI_VELOCITY: u32 = esp(0xd0);
/// The cache block (six counts and flags) followed by the first body's support points; the
/// contact path uses the same space for the world-space points.
const WORK: u32 = esp(0xe0);
const POINTS_A: u32 = esp(0x100);
const POINTS_B: u32 = esp(0x1c0);
/// The first body's swept transform at the query time.
const LERP_A: u32 = esp(0x260);
const FRAME: u32 = 0x394;

/// The words of one `processCollision` call and its frame.
#[derive(Clone, Copy)]
struct Process {
    this: u32,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
    frame: u32,
}

impl Process {
    /// `hkpGskfAgent::processCollisionNoTim` on the same agent and bodies.
    fn without_time_of_impact(self, e: &mut Engine) {
        e.call(
            PROCESS_NO_TIM,
            &args![self.this, self.body_a, self.body_b, self.input, self.output],
        );
    }

    /// Copies the float at `from` to `to` through the x87 stack (a quiet copy).
    fn copy_quiet(self, e: &mut Engine, from: u32, to: u32) {
        let bits = x87_quiet_bits(e.mem.u32(from));
        e.mem.set_u32(to, bits);
    }
}

/// The part of `processCollision` that runs when the stored time differs from the query time
/// and the parameters ask for continuous queries: both bodies' swept transforms at the query
/// time, the body blocks, the cache block and both support point sets, the separating normal
/// and, when it reports a cache to export, that export.
fn advance_to_query_time(e: &mut Engine, p: Process) {
    let f = p.frame;
    let shape_a = e.mem.u32(p.body_a);
    let shape_b = e.mem.u32(p.body_b);
    e.mem.set_u32(f + BLOCK_A + 8, f + LERP_A);
    e.mem.set_u32(f + BLOCK_A + 4, e.mem.u32(p.body_a + 4));
    e.mem.set_u32(f + BLOCK_A, shape_a);
    e.mem.set_u32(f + BLOCK_B + 8, f + LERP_B);
    e.mem.set_u32(f + BLOCK_B, shape_b);
    e.mem.set_u32(f + BLOCK_B + 0xc, p.body_b);
    e.mem.set_u32(f + BLOCK_A + 0xc, p.body_a);
    e.mem.set_u32(f + BLOCK_B + 4, e.mem.u32(p.body_b + 4));
    let time = x87_quiet_bits(e.mem.u32(p.input + 0x50));
    let motion_a = e.mem.u32(p.body_a + 8);
    e.call(LERP_TRANSFORM, &args![motion_a + 0x40, time, f + LERP_A]);
    let time = x87_quiet_bits(e.mem.u32(p.input + 0x50));
    let motion_b = e.mem.u32(p.body_b + 8);
    e.call(LERP_TRANSFORM, &args![motion_b + 0x40, time, f + LERP_B]);
    let counts = u32::from(e.mem.u8(p.this + 0x16));
    e.mem.set_u32(f + WORK, u32::from(e.mem.u8(p.this + 0x14)));
    e.mem
        .set_u32(f + WORK + 4, u32::from(e.mem.u8(p.this + 0x15)));
    e.mem.set_u32(f + WORK + 8, counts & 0xf);
    e.mem.set_u32(f + WORK + 0xc, counts >> 4);
    e.mem.set_u32(f + WORK + 0x10, 0);
    e.mem.set_u32(f + WORK + 0x14, 0);
    let count_a = e.mem.u32(f + WORK);
    e.vcall(shape_a, 0x3c, &args![p.this + 0xc, count_a, f + POINTS_A]);
    let count_b = e.mem.u32(f + WORK + 4);
    let count_a = e.mem.u32(f + WORK);
    e.vcall(
        shape_b,
        0x3c,
        &args![p.this + 0xc + 2 * count_a, count_b, f + POINTS_B],
    );
    let params = e.mem.u32(p.input + 0x60);
    let tolerance = x87_quiet_bits(e.mem.u32(params));
    e.call(
        CALC_SEPARATING_NORMAL,
        &args![f + BLOCK_A, f + BLOCK_B, tolerance, f + WORK, p.this + 0x20],
    );
    if e.mem.u32(f + WORK + 0x14) != 0 {
        e.call(EXIT_AND_EXPORT_CACHE, &args![f + WORK, p.this + 0xc]);
    }
}

/// The contact path of `processCollision`: the support points of both shapes in world space
/// and the manifold finish. The first test of the contact count tells whether there are any
/// points to transform at all.
fn contact_points(e: &mut Engine, p: Process) {
    let f = p.frame;
    let shape_a = e.mem.u32(p.body_a);
    let shape_b = e.mem.u32(p.body_b);
    let tolerance = x87_quiet(e.mem.f32(p.input + 8));
    let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
    let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
    let contacts = p.this + 0x30;
    let count = e.mem.u8(contacts + 2);
    let separation = load_bits(e, p.this + 0x20);
    store_bits(e, f + esp(0x1e0), separation);
    let sum = ((f64::from(radius_a) + f64::from(tolerance)) + f64::from(radius_b)) as f32;
    // The square of the sum is stored too, and nothing reads it back.
    e.mem
        .set_f32(f + esp(0x1fc), (f64::from(sum) * f64::from(sum)) as f32);
    let work = f + WORK;
    if count != 0 {
        let after_points = contacts + u32::from(count) * 8 + 4;
        let count_a = e.mem.u8(contacts);
        e.vcall(
            shape_a,
            0x3c,
            &args![after_points, u32::from(count_a), work],
        );
        let matrix_a = e.mem.u32(p.body_a + 8);
        let rows = copy_rows(
            e,
            matrix_a,
            [f + esp(0x30), f + esp(0x20), f + esp(0x50), f + esp(0x40)],
        );
        let count_a = e.mem.u8(contacts);
        transform_points(e, work, count_a, &rows);
        let count_b = e.mem.u8(contacts + 1);
        let second_work = work + (u32::from(count_a) << 4);
        e.vcall(
            shape_b,
            0x3c,
            &args![
                after_points + 2 * u32::from(count_a),
                u32::from(count_b),
                second_work
            ],
        );
        let matrix_b = e.mem.u32(p.body_b + 8);
        let rows = copy_rows(
            e,
            matrix_b,
            [f + esp(0x40), f + esp(0x50), f + esp(0x30), f + esp(0x20)],
        );
        let count_b = e.mem.u8(contacts + 1);
        transform_points(e, second_work, count_b, &rows);
    }
    let owner = e.mem.u32(p.this + 8);
    e.call(
        MANIFOLD_FINISH,
        &args![contacts, work, 0u32, p.output, owner],
    );
}

// Translated from 00d42020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPredGskfAgent::processCollision` (the name Ghidra shows), `this` in `ECX` and four stack
/// words (the callee pops them): `(this, body A, body B, input, output)`. The agent's words
/// `0x18` (the stored time), `0x1c` and `0x2c` (the clamp state), `0x20` (a vector) and
/// `0x30` (the manifold: three counts) drive it; the input holds the query times (`0x50`,
/// `0x54`), the step (`0x58`), the tolerance (`8`) and the parameters (`0x60`).
///
/// If the stored time differs from the query time the query is moved to it (for parameters
/// that ask for continuous queries; otherwise the stored time is just made the next time and
/// the agent's plain step runs). Then the relative velocity of the two swept transforms gives a
/// clamp; when it passes the first two tests it is stored and the manifold cleaned up, when the
/// time of impact is wanted and the limits are above the clamp the time-of-impact step runs,
/// and when the clamp is under the third limit the contact points are built; otherwise the
/// plain step runs.
pub fn pred_gskf_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, PROFILE_BEGIN);
    e.with_stack(FRAME, |e, frame| {
        let p = Process {
            this: this.addr(),
            body_a: body_a.addr(),
            body_b: body_b.addr(),
            input: input.addr(),
            output: output.addr(),
            frame: frame.addr(),
        };
        process_collision_body(e, p);
    });
    profile_record(e, PROFILE_END);
}

fn process_collision_body(e: &mut Engine, p: Process) {
    let f = p.frame;
    let stored = f64::from(e.mem.f32(p.this + 0x18));
    let query = f64::from(e.mem.f32(p.input + 0x50));
    if stored != query {
        let params = e.mem.u32(p.input + 0x60);
        if e.mem.u32(params + 0x10) == 0 {
            p.copy_quiet(e, p.input + 0x54, p.this + 0x18);
            p.without_time_of_impact(e);
            return;
        }
        advance_to_query_time(e, p);
    }
    p.copy_quiet(e, p.input + 0x54, p.this + 0x18);
    let motion_a = e.mem.u32(p.body_a + 8);
    let motion_b = e.mem.u32(p.body_b + 8);
    let step = x87_quiet(e.mem.f32(p.input + 0x58));
    let scale_a = (f64::from(e.mem.f32(motion_a + 0x5c)) * f64::from(step)) as f32;
    let scale_b = (f64::from(step) * f64::from(e.mem.f32(motion_b + 0x5c))) as f32;
    let drift_b = zip_with(
        load_vec(e, motion_b + 0x50),
        load_vec(e, motion_b + 0x40),
        |a, b| a - b,
    );
    let drift_a = zip_with(
        load_vec(e, motion_a + 0x40),
        load_vec(e, motion_a + 0x50),
        |a, b| a - b,
    );
    let term_b = (f64::from(e.mem.f32(motion_b + 0x9c)) * f64::from(scale_b))
        * f64::from(e.mem.f32(motion_b + 0xa0));
    let term_a = (f64::from(e.mem.f32(motion_a + 0x9c)) * f64::from(scale_a))
        * f64::from(e.mem.f32(motion_a + 0xa0));
    let mut velocity: Vec4 =
        std::array::from_fn(|lane| scale_b * drift_b[lane] + drift_a[lane] * scale_a);
    store_vec(e, f + VELOCITY, velocity);
    let angular = (term_b + term_a) as f32;
    e.mem.set_f32(f + VELOCITY + 0xc, angular);
    velocity = load_vec(e, f + VELOCITY);
    let axis = load_vec(e, p.this + 0x20);
    let projected = sum3(zip_with(velocity, axis, |a, b| a * b));
    let distance = e.mem.f32(p.this + 0x2c);
    let clamp = ((f64::from(distance) - f64::from(projected)) - f64::from(angular)) as f32;
    let clamp64 = f64::from(clamp);
    let params = e.mem.u32(p.input + 0x60);
    let half: f64 = e.global(HALF_F64);
    if f64::from(e.mem.f32(params)) < clamp64
        && f64::from(e.mem.f32(p.this + 0x1c)) * half < clamp64
    {
        e.mem.set_f32(p.this + 0x2c, clamp);
        if e.mem.u8(p.this + 0x32) != 0 {
            let owner = e.mem.u32(p.this + 8);
            let word = e.mem.u32(p.output + 4);
            e.call(MANIFOLD_CLEANUP, &args![p.this + 0x30, owner, word]);
        }
        return;
    }
    if e.mem.u32(params + 0x10) != 0 {
        let owner = e.mem.u32(p.this + 8);
        e.mem.set_f32(f + TOI_CLAMP, clamp);
        let held = x87_quiet(e.mem.f32(p.this + 0x2c));
        e.mem.set_u32(f + TOI_INPUT + 0x10, owner);
        e.mem.set_u32(f + TOI_INPUT + 8, 0);
        e.mem.set_u32(f + TOI_INPUT, p.body_a);
        e.mem.set_u32(f + TOI_INPUT + 4, p.body_b);
        e.mem.set_u32(f + TOI_INPUT + 0xc, p.input);
        store_vec(e, f + TOI_VELOCITY, velocity);
        let growth = f64::from(e.mem.f32(p.this + 0x1c));
        let low = (f64::from(e.mem.f32(params + 0x18)) * growth) as f32;
        let high = ((f64::from(e.mem.f32(params + 0x1c)) * growth) + f64::from(held)) as f32;
        let first = if high > low { low } else { high };
        let reached = matches!(
            f64::from(first).partial_cmp(&clamp64),
            Some(Ordering::Less | Ordering::Equal)
        );
        if !reached {
            let low = (f64::from(e.mem.f32(params + 0x28)) * growth) as f32;
            let high = (f64::from(held) + (f64::from(e.mem.f32(params + 0x2c)) * growth)) as f32;
            let second = if high > low { low } else { high };
            let growth_bits = x87_quiet_bits(e.mem.u32(p.this + 0x1c));
            e.call(
                COLLIDE_CALC_TOI,
                &args![
                    f + TOI_INPUT,
                    growth_bits,
                    first,
                    second,
                    p.this + 0xc,
                    p.this + 0x20,
                    p.output
                ],
            );
            p.without_time_of_impact(e);
            return;
        }
    }
    if f64::from(e.mem.f32(params + 0xc)) < clamp64 {
        e.mem.set_f32(p.this + 0x2c, clamp);
        contact_points(e, p);
        return;
    }
    p.without_time_of_impact(e);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d4_2020,
        pred_gskf_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod a05_gskf_agent_tests {
    use super::super::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use super::*;

    const POINTS_A: u32 = 0x0900_0200;
    const POINTS_B: u32 = 0x0900_0210;
    const PROCESS: u32 = 0x00d4_2020;
    const UNQUIET_NAN: u32 = 0x7f80_0001;

    /// An agent holding the stored time `1`, a growth of `4`, the vector `(2, 0, 0, 0)`, the
    /// distance `10`, the owner word `0x5555` and the counts `2`, `1` (cache) and `2`, `1`, `0`
    /// (manifold); two bodies, one translating by `(10, 20, 30)` and drifting by `-1` along x
    /// with weights that give the angular term `1.5`, the other scaling by two; an input with
    /// the same query time, next time `2`, step `1` and tolerance `0.1`; parameters with the
    /// continuous flag off. With these the clamp is `10.5`, which passes the first tests.
    struct Rig {
        e: Engine,
        stream: u32,
        this: u32,
        body: [u32; 2],
        shape: [u32; 2],
        input: u32,
        output: u32,
        params: u32,
        seen: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            e.map(0x0101_1000, 0x1000);
            e.map(0x0126_8000, 0x1000);
            e.mem.set_f64(HALF_F64, 0.5);
            let stream = e.mem.alloc(0x100);
            e.mem.set_u32(stream + 4, stream + 0x20);
            e.mem.set_u32(stream + 0xc, stream + 0xe0);
            e.mem.set_u32(PROFILE_SLOT, 3);
            e.register_double(TLS_GET_VALUE, move |_, _| Ret {
                eax: stream,
                ..Ret::default()
            });
            let rows_a = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [10.0, 20.0, 30.0, 0.0],
            ];
            let rows_b = [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0; 4],
            ];
            let mut body = [0; 2];
            let mut shape = [0; 2];
            for index in 0..2 {
                let vtable = e.mem.alloc(0x80);
                e.mem.set_u32(vtable + 0x3c, [POINTS_A, POINTS_B][index]);
                shape[index] = e.mem.alloc(0x40);
                e.mem.set_u32(shape[index], vtable);
                let motion = e.mem.alloc(0x100);
                let rows = [rows_a, rows_b][index];
                for (row, v) in rows.into_iter().enumerate() {
                    store_vec(&mut e, motion + 16 * row as u32, v);
                }
                let (scale, weight, damping) = [(1.0, 2.0, 0.5), (0.5, 4.0, 0.25)][index];
                if index == 0 {
                    store_vec(&mut e, motion + 0x50, [1.0, 0.0, 0.0, 0.0]);
                }
                e.mem.set_f32(motion + 0x5c, scale);
                e.mem.set_f32(motion + 0x9c, weight);
                e.mem.set_f32(motion + 0xa0, damping);
                body[index] = e.mem.alloc(0x20);
                e.mem.set_u32(body[index], shape[index]);
                e.mem.set_u32(body[index] + 4, 0xa4 + index as u32);
                e.mem.set_u32(body[index] + 8, motion);
            }
            e.register(POINTS_A, |e, a| {
                for k in 0..a[2] {
                    store_vec(e, a[3] + 16 * k, [k as f32 + 1.0, 2.0, 3.0, 0.0]);
                }
                Ret::default()
            });
            e.register(POINTS_B, |e, a| {
                for k in 0..a[2] {
                    store_vec(e, a[3] + 16 * k, [k as f32 + 10.0, 2.0, 3.0, 0.0]);
                }
                Ret::default()
            });
            let params = e.mem.alloc(0x40);
            e.mem.set_f32(params, 1.0);
            e.mem.set_f32(params + 0xc, 0.5);
            e.mem.set_f32(params + 0x18, 1.0);
            e.mem.set_f32(params + 0x1c, 0.5);
            e.mem.set_f32(params + 0x28, 2.0);
            e.mem.set_f32(params + 0x2c, 1.0);
            let input = e.mem.alloc(0x80);
            e.mem.set_f32(input + 8, 0.1);
            e.mem.set_f32(input + 0x50, 1.0);
            e.mem.set_f32(input + 0x54, 2.0);
            e.mem.set_f32(input + 0x58, 1.0);
            e.mem.set_u32(input + 0x60, params);
            let this = e.mem.alloc(0x80);
            e.mem.set_f32(this + 0x18, 1.0);
            e.mem.set_f32(this + 0x1c, 4.0);
            store_vec(&mut e, this + 0x20, [2.0, 0.0, 0.0, 0.0]);
            e.mem.set_f32(this + 0x2c, 10.0);
            e.mem.set_u32(this + 8, 0x5555);
            e.mem.set_u8(this + 0x14, 2);
            e.mem.set_u8(this + 0x15, 1);
            e.mem.set_u8(this + 0x16, 0x31);
            e.mem.set_u8(this + 0x30, 2);
            e.mem.set_u8(this + 0x31, 1);
            let output = e.mem.alloc(0x40);
            e.mem.set_u32(output + 4, 0x4444);
            for address in [
                MANIFOLD_CLEANUP,
                COLLIDE_CALC_TOI,
                MANIFOLD_FINISH,
                PROCESS_NO_TIM,
                EXIT_AND_EXPORT_CACHE,
                LERP_TRANSFORM,
                CALC_SEPARATING_NORMAL,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            let seen = e.mem.alloc(0x100);
            Rig {
                e,
                stream,
                this,
                body,
                shape,
                input,
                output,
                params,
                seen,
            }
        }

        fn run(&mut self) {
            self.e.call(
                PROCESS,
                &args![
                    self.this,
                    self.body[0],
                    self.body[1],
                    self.input,
                    self.output
                ],
            );
        }

        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            let log = self.e.call_log.as_ref().unwrap();
            log.iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, args)| args.clone())
                .collect()
        }

        fn words(&self, address: u32, count: u32) -> Vec<u32> {
            (0..count)
                .map(|n| self.e.mem.u32(address + 4 * n))
                .collect()
        }
    }

    #[test]
    fn a_new_time_without_continuous_parameters_just_steps() {
        let mut r = Rig::new();
        r.e.mem.set_f32(r.this + 0x18, 0.5);
        r.e.mem.set_u32(r.input + 0x54, UNQUIET_NAN);
        r.run();
        assert_eq!(r.e.mem.u32(r.this + 0x18), 0x7fc0_0001);
        assert_eq!(
            r.calls(PROCESS_NO_TIM),
            vec![vec![r.this, r.body[0], r.body[1], r.input, r.output]]
        );
        assert!(r.calls(LERP_TRANSFORM).is_empty());
        assert_eq!(r.e.mem.u32(r.stream + 4), r.stream + 0x20 + 24);
        assert_eq!(r.e.mem.u32(r.stream + 0x20), PROFILE_BEGIN);
        assert_eq!(r.e.mem.u32(r.stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn a_clamp_above_both_limits_is_stored() {
        let mut r = Rig::new();
        r.run();
        assert_eq!(r.e.mem.f32(r.this + 0x2c), 10.5);
        assert_eq!(r.e.mem.f32(r.this + 0x18), 2.0);
        assert!(r.calls(MANIFOLD_CLEANUP).is_empty());
        assert!(r.calls(PROCESS_NO_TIM).is_empty());
    }

    #[test]
    fn a_stored_clamp_cleans_the_manifold_when_it_has_contacts() {
        let mut r = Rig::new();
        r.e.mem.set_u8(r.this + 0x32, 2);
        r.run();
        assert_eq!(
            r.calls(MANIFOLD_CLEANUP),
            vec![vec![r.this + 0x30, 0x5555, 0x4444]]
        );
    }

    #[test]
    fn a_new_time_moves_both_bodies_and_asks_for_the_separating_normal() {
        let mut r = Rig::new();
        let seen = r.seen;
        r.e.mem.set_f32(r.this + 0x18, 0.5);
        r.e.mem.set_u32(r.params + 0x10, 1);
        r.e.register_double(CALC_SEPARATING_NORMAL, move |e, a| {
            for word in 0..4 {
                e.mem.set_u32(seen + 4 * word, e.mem.u32(a[0] + 4 * word));
                e.mem
                    .set_u32(seen + 0x10 + 4 * word, e.mem.u32(a[1] + 4 * word));
            }
            for word in 0..6 {
                e.mem
                    .set_u32(seen + 0x20 + 4 * word, e.mem.u32(a[3] + 4 * word));
            }
            e.mem.set_u32(a[3] + 0x14, 1);
            Ret::default()
        });
        r.run();
        let lerps = r.calls(LERP_TRANSFORM);
        let motion = |body: u32| r.e.mem.u32(body + 8) + 0x40;
        assert_eq!(lerps.len(), 2);
        assert_eq!(lerps[0][..2], [motion(r.body[0]), 1.0f32.to_bits()]);
        assert_eq!(lerps[1][..2], [motion(r.body[1]), 1.0f32.to_bits()]);
        assert_eq!(lerps[0][2] - lerps[1][2], 0x200);
        assert_eq!(
            r.words(r.seen, 4),
            vec![r.shape[0], 0xa4, lerps[0][2], r.body[0]]
        );
        assert_eq!(
            r.words(r.seen + 0x10, 4),
            vec![r.shape[1], 0xa5, lerps[1][2], r.body[1]]
        );
        assert_eq!(r.words(r.seen + 0x20, 6), vec![2, 1, 1, 3, 0, 0]);
        let normal = r.calls(CALC_SEPARATING_NORMAL);
        assert_eq!(normal.len(), 1);
        let cache = normal[0][3];
        assert_eq!(normal[0][2], 1.0f32.to_bits());
        assert_eq!(normal[0][4], r.this + 0x20);
        assert_eq!(
            r.calls(POINTS_A),
            vec![vec![r.shape[0], r.this + 0xc, 2, cache + 0x20]]
        );
        assert_eq!(
            r.calls(POINTS_B),
            vec![vec![r.shape[1], r.this + 0xc + 4, 1, cache + 0xe0]]
        );
        assert_eq!(
            r.calls(EXIT_AND_EXPORT_CACHE),
            vec![vec![cache, r.this + 0xc]]
        );
        assert_eq!(r.e.mem.f32(r.this + 0x2c), 10.5);
        assert_eq!(r.e.mem.u32(r.this + 0x18), 2.0f32.to_bits());
    }

    #[test]
    fn a_new_time_without_a_cache_to_export_does_not_export() {
        let mut r = Rig::new();
        r.e.mem.set_f32(r.this + 0x18, 0.5);
        r.e.mem.set_u32(r.params + 0x10, 1);
        r.run();
        assert_eq!(r.calls(CALC_SEPARATING_NORMAL).len(), 1);
        assert!(r.calls(EXIT_AND_EXPORT_CACHE).is_empty());
    }

    /// Makes the clamp `1.0` (the distance `0.5`) and fails the first test (limit `5`).
    fn low_clamp(r: &mut Rig) {
        r.e.mem.set_f32(r.this + 0x2c, 0.5);
        r.e.mem.set_f32(r.params, 5.0);
    }

    #[test]
    fn limits_above_the_clamp_run_the_time_of_impact_step_then_the_plain_step() {
        let mut r = Rig::new();
        low_clamp(&mut r);
        r.e.mem.set_u32(r.params + 0x10, 1);
        let seen = r.seen;
        r.e.register_double(COLLIDE_CALC_TOI, move |e, a| {
            for word in 0..0x20 {
                e.mem.set_u32(seen + 4 * word, e.mem.u32(a[0] + 4 * word));
            }
            Ret::default()
        });
        r.run();
        let toi = r.calls(COLLIDE_CALC_TOI);
        assert_eq!(toi.len(), 1);
        assert_eq!(
            toi[0][1..],
            [
                4.0f32.to_bits(),
                2.5f32.to_bits(),
                4.5f32.to_bits(),
                r.this + 0xc,
                r.this + 0x20,
                r.output
            ]
        );
        assert_eq!(
            r.words(r.seen, 5),
            vec![r.body[0], r.body[1], 0, r.input, 0x5555]
        );
        assert_eq!(r.e.mem.f32(r.seen + 0x60), 1.0);
        assert_eq!(load_vec(&r.e, r.seen + 0x70), [-1.0, 0.0, 0.0, 1.5]);
        assert_eq!(r.calls(PROCESS_NO_TIM).len(), 1);
        assert_eq!(r.e.mem.f32(r.this + 0x2c), 0.5);
    }

    #[test]
    fn limits_within_the_clamp_skip_the_time_of_impact_step() {
        let mut r = Rig::new();
        low_clamp(&mut r);
        r.e.mem.set_u32(r.params + 0x10, 1);
        r.e.mem.set_f32(r.params + 0x18, 0.1);
        r.e.mem.set_f32(r.params + 0xc, 5.0);
        r.run();
        assert!(r.calls(COLLIDE_CALC_TOI).is_empty());
        assert_eq!(r.calls(PROCESS_NO_TIM).len(), 1);
    }

    #[test]
    fn a_clamp_above_the_third_limit_builds_the_contact_points() {
        let mut r = Rig::new();
        low_clamp(&mut r);
        r.e.mem.set_u8(r.this + 0x32, 1);
        let seen = r.seen;
        r.e.register_double(MANIFOLD_FINISH, move |e, a| {
            for word in 0..12 {
                e.mem.set_u32(seen + 4 * word, e.mem.u32(a[1] + 4 * word));
            }
            Ret::default()
        });
        r.run();
        assert_eq!(r.e.mem.f32(r.this + 0x2c), 1.0);
        let after = r.this + 0x30 + 8 + 4;
        let a = r.calls(POINTS_A);
        let b = r.calls(POINTS_B);
        assert_eq!(a[0][..3], [r.shape[0], after, 2]);
        assert_eq!(b[0][..3], [r.shape[1], after + 4, 1]);
        assert_eq!(b[0][3], a[0][3] + 32);
        let points: Vec<f32> = (0..12).map(|n| r.e.mem.f32(r.seen + 4 * n)).collect();
        assert_eq!(
            points,
            vec![11.0, 22.0, 33.0, 0.0, 12.0, 22.0, 33.0, 0.0, 20.0, 4.0, 6.0, 0.0]
        );
        assert_eq!(
            r.calls(MANIFOLD_FINISH),
            vec![vec![r.this + 0x30, a[0][3], 0, r.output, 0x5555]]
        );
        assert!(r.calls(PROCESS_NO_TIM).is_empty());
    }

    #[test]
    fn a_clamp_above_the_third_limit_without_contacts_still_finishes_the_manifold() {
        let mut r = Rig::new();
        low_clamp(&mut r);
        r.run();
        assert!(r.calls(POINTS_A).is_empty());
        assert_eq!(r.calls(MANIFOLD_FINISH).len(), 1);
    }

    #[test]
    fn a_clamp_under_the_third_limit_runs_the_plain_step() {
        let mut r = Rig::new();
        low_clamp(&mut r);
        r.e.mem.set_f32(r.params + 0xc, 5.0);
        r.run();
        assert_eq!(r.calls(PROCESS_NO_TIM).len(), 1);
        assert!(r.calls(MANIFOLD_FINISH).is_empty());
        assert_eq!(r.e.mem.f32(r.this + 0x2c), 0.5);
    }
}
