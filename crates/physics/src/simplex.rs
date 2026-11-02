//! The character proxy's "simplex solver": given a velocity and the surfaces
//! the character is touching, it finds how far the character can move this
//! step and what velocity it ends up with.
//!
//! This is a translation of Havok 7.1's solver as compiled into
//! FalloutNV.exe 1.4.0.525 (PC). Names come from the Xbox PDB. Each function
//! cites the PC address it was translated from.
//!
//! How it works, in short: every surface is a plane (a normal and a distance)
//! that may itself be moving. The solver walks forward in time. At each step
//! it finds the first plane the character would reach, moves up to it, adds
//! that plane to a small set of "active" planes (at most four), and then
//! works out a new velocity that slides along all active planes, with
//! friction. Planes that stop mattering are dropped from the set again.
//!
//! Floating point: the game mixes SSE (single precision) and the x87 unit.
//! Single x87 operations whose result is stored straight back as a float
//! give the same answer as plain `f32` arithmetic, so those are written as
//! `f32`. Where the x87 unit keeps a chain of operations in a register
//! before storing, the chain is computed here in `f64`.
//! unresolved: if the game runs the x87 unit in 24-bit precision mode (as
//! Direct3D 9 does by default), those chains round to `f32` at every step
//! instead; the two only differ in the last bit in rare cases.

// The comparisons are written the way the machine code tests its flags, so
// that NaN takes the same branch as in the game; hence the negated forms.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

/// A surface the character touches (`hkpSurfaceConstraintInfo`, 0x40 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SurfaceConstraintInfo {
    /// Normal in x, y, z; distance in w (a point p is on the plane when
    /// n·p + w = 0).
    pub plane: [f32; 4],
    /// The surface's own velocity.
    pub velocity: [f32; 4],
    pub static_friction: f32,
    pub extra_up_static_friction: f32,
    pub extra_down_static_friction: f32,
    pub dynamic_friction: f32,
    /// Higher priority planes win when two cannot both be satisfied.
    pub priority: i32,
}

/// What happened between the character and one surface
/// (`hkpSurfaceConstraintInteraction`, 0x10 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SurfaceConstraintInteraction {
    pub touched: bool,
    /// Never written by the solver itself.
    pub stopped: bool,
    /// How long the character moved while this plane was active.
    pub surface_time: f32,
    /// Extra distance allowed before this plane counts as reached again.
    pub penalty_distance: f32,
    /// 0 = free; 1 = given up on by the three-plane solve; 2 = given up on
    /// by the two-plane solve. Planes with a non-zero status are not picked
    /// up again during this solve.
    pub status: i32,
}

/// Input to [`solve`] (`hkpSimplexSolverInput`, 0x50 bytes).
pub struct SimplexSolverInput<'a> {
    pub position: [f32; 4],
    pub velocity: [f32; 4],
    /// The largest velocity, per axis, a solved surface velocity may have.
    pub max_surface_velocity: [f32; 4],
    pub up_vector: [f32; 4],
    pub delta_time: f32,
    /// The solver stops once it has moved for longer than this.
    pub min_delta_time: f32,
    pub constraints: &'a [SurfaceConstraintInfo],
}

/// Output of [`solve`] (`hkpSimplexSolverOutput`, 0x30 bytes).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SimplexSolverOutput {
    pub position: [f32; 4],
    pub velocity: [f32; 4],
    /// How long the character moved for.
    pub delta_time: f32,
    /// One entry per constraint, in the same order.
    pub plane_interactions: Vec<SurfaceConstraintInteraction>,
}

type V4 = [f32; 4];

/// `1.1920929e-07` (float at `0104da4c`, 2^-23).
const EPS: f32 = 1.192_092_9e-7;
/// `2^-23` as a double (`010c9300`), added to the penalty distance.
const EPS_D: f64 = 1.192_092_895_507_812_5e-7;
/// `0.0001f` stored as a double (`01032980`).
const MIN_STEP: f32 = 0.0001;
/// `0.001f` stored as a double (`01016978`).
const DYN_RATIO: f32 = 0.001;
/// `-0.001` (float at `0104ffb8`).
const TEST_TOL: f32 = -0.001;

fn add4(a: V4, b: V4) -> V4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn sub4(a: V4, b: V4) -> V4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

fn mul4(a: V4, b: V4) -> V4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// `s` times every lane of `v`.
fn scale4(s: f32, v: V4) -> V4 {
    [s * v[0], s * v[1], s * v[2], s * v[3]]
}

/// x·x' + y·y' + z·z', summed as the game does: (x + y) + z.
fn dot3(a: V4, b: V4) -> f32 {
    let p = mul4(a, b);
    (p[1] + p[0]) + p[2]
}

/// a × b; the w lane is a.w·b.w - b.w·a.w (zero for finite values).
#[allow(clippy::eq_op)]
fn cross(a: V4, b: V4) -> V4 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        a[3] * b[3] - b[3] * a[3],
    ]
}

/// True when any of x, y, z of `v` is larger in size than the same lane of
/// `max`.
fn exceeds(v: V4, max: V4) -> bool {
    (0..3).any(|i| max[i] < v[i].abs())
}

/// The solver's working state on the stack of `00d24360`.
///
/// Layout in the game: four active-plane entries of three dwords each at
/// +0x00 (constraint index, constraint pointer, interaction pointer), the
/// active count at +0x30, the time moved so far at +0x34, the input pointer
/// at +0x38 and the output pointer at +0x3c. Index and both pointers always
/// move together, so an entry here is just the constraint index.
struct SolverInfo<'i, 'a> {
    entries: [usize; 4],
    count: usize,
    total_time: f32,
    input: &'i SimplexSolverInput<'a>,
    output: SimplexSolverOutput,
}

impl SolverInfo<'_, '_> {
    fn c(&self, i: usize) -> &SurfaceConstraintInfo {
        &self.input.constraints[i]
    }

    fn set_status(&mut self, i: usize, status: i32) {
        self.output.plane_interactions[i].status = status;
    }

    fn status(&self, i: usize) -> i32 {
        self.output.plane_interactions[i].status
    }
}

/// `00d23380` hkSimplexSolverSortInfo (Xbox PDB).
///
/// Orders the active planes by priority, lowest first; planes of equal
/// priority are ordered by how fast the surface moves, slowest first (equal
/// speeds are swapped too).
fn sort_info(info: &mut SolverInfo) {
    let count = info.count;
    if count < 2 {
        return;
    }
    for a in 0..count - 1 {
        for b in a + 1..count {
            let ca = info.c(info.entries[a]);
            let cb = info.c(info.entries[b]);
            let swap = if ca.priority < cb.priority {
                false
            } else if ca.priority != cb.priority {
                true
            } else {
                let va = dot3(ca.velocity, ca.velocity);
                let vb = dot3(cb.velocity, cb.velocity);
                vb <= va
            };
            if swap {
                info.entries.swap(a, b);
            }
        }
    }
}

/// `00d23480` hkSimplexSolverSolveTest1d (Xbox PDB).
///
/// True when `velocity` still moves into the plane (relative to the plane's
/// own motion) by more than 0.001.
fn solve_test_1d(c: &SurfaceConstraintInfo, velocity: V4) -> bool {
    let d = dot3(sub4(velocity, c.velocity), c.plane);
    TEST_TOL > d || d.is_nan()
}

/// `00d234e0` hkSimplexSolverSolve1d (Xbox PDB).
///
/// Slides `vin` along one plane with friction and returns the result.
/// Static friction can stop the sideways part and the up/down-slope part
/// separately; dynamic friction below 1 scales the slide back up towards
/// the original speed.
fn solve_1d(input: &SimplexSolverInput, c: &SurfaceConstraintInfo, vin: V4) -> V4 {
    let n = c.plane;
    let cvel = c.velocity;
    let sf = c.static_friction;
    let df = c.dynamic_friction;

    let rel = sub4(vin, cvel);
    let dot_n = dot3(n, rel);
    let rel_len2 = dot3(rel, rel);
    let mut tangent = sub4(rel, scale4(dot_n, n));
    let dot_n2 = dot_n * dot_n;
    let up_dot = dot3(input.up_vector, tangent);
    let extra = if 0.0 < up_dot {
        c.extra_up_static_friction
    } else {
        c.extra_down_static_friction
    };

    if 0.0 < extra {
        // Split the slide into a sideways part (along up × normal) and an
        // up/down-slope part, and hold each with static friction.
        let mut t = 0.0f32;
        let side_raw = cross(input.up_vector, n);
        let len2 = dot3(side_raw, side_raw);
        let mut side = side_raw;
        if len2 > EPS {
            let inv = 1.0 / len2.sqrt();
            side = scale4(inv, side_raw);
            let dot_c = dot3(side, tangent);
            t = dot_c;
            let sf2 = sf * sf;
            if ((dot_c * dot_c) as f64) <= sf2 as f64 * dot_n2 as f64 {
                t = 0.0;
                tangent = sub4(tangent, scale4(dot_c, side));
            }
        }
        let e = extra as f64 + sf as f64;
        let e2 = (e * e) as f32;
        let q = e2 as f64 * dot_n2 as f64;
        let tt = t as f64;
        let u = ((rel_len2 as f64 - tt * tt) - dot_n2 as f64) as f32;
        if (u as f64) <= q {
            if t == 0.0 {
                return cvel;
            }
            tangent = scale4(t, side);
        }
    } else {
        let sf2 = sf * sf;
        let p = (sf2 as f64 + 1.0) * dot_n2 as f64;
        if (rel_len2 as f64) <= p {
            return cvel;
        }
    }

    // Dynamic friction.
    if !(df < 1.0) {
        return add4(cvel, tangent);
    }
    let tl2 = dot3(tangent, tangent);
    if !(tl2 >= EPS) {
        return add4(cvel, tangent);
    }
    if !((rel_len2 as f64 * DYN_RATIO as f64) < tl2 as f64) {
        return add4(cvel, tangent);
    }
    let k = ((rel_len2 as f64 / tl2 as f64) as f32).sqrt();
    let dfd = df as f64;
    let m = (dfd + k as f64 * (1.0 - dfd)) as f32;
    let mut v = scale4(m, tangent);
    let d = dot3(n, v);
    v = sub4(v, scale4(d, n));
    add4(cvel, v)
}

/// `00d23850` hkSimplexSolverSolve2d (Xbox PDB).
///
/// Slides `vin` along the crease where planes `i0` and `i1` meet, with
/// their combined friction. If the planes are (nearly) parallel or the
/// crease would move too fast, both get status 2 and are solved one at a
/// time instead, the higher priority one last.
fn solve_2d(info: &mut SolverInfo, i0: usize, i1: usize, vin: V4) -> V4 {
    let input = info.input;
    let c0 = &input.constraints[i0];
    let c1 = &input.constraints[i1];
    let n0 = c0.plane;
    let n1 = c1.plane;
    let axis = cross(n0, n1);
    let len2 = dot3(axis, axis);

    if !(len2 <= EPS) {
        let inv = 1.0 / len2.sqrt();
        let dir = scale4(inv, axis);
        let a = cross(dir, n0);
        let b = cross(n1, dir);
        let d1 = dot3(c1.velocity, n1);
        let d0 = dot3(c0.velocity, n0);
        let s = dot3(add4(c0.velocity, c1.velocity), dir);
        let half = 0.5 * s;
        // The crease's own velocity: matches both planes' speeds along
        // their normals, and their average speed along the crease.
        let v = scale4(
            inv,
            add4(add4(scale4(half, axis), scale4(d0, b)), scale4(d1, a)),
        );
        if !exceeds(v, input.max_surface_velocity) {
            let rel = sub4(vin, v);
            let r = dot3(rel, rel);
            let u = dot3(input.up_vector, dir);
            let t = dot3(rel, dir);
            let sf_sum = c0.static_friction + c1.static_friction;
            let x = if t as f64 * u as f64 > 0.0 {
                c0.extra_up_static_friction as f64 + c1.extra_up_static_friction as f64
            } else {
                c0.extra_down_static_friction as f64 + c1.extra_down_static_friction as f64
            };
            let f = (x * u as f64 + sf_sum as f64) as f32;
            let f2 = f * 0.5;
            let dh = (0.5 * (c0.dynamic_friction as f64 + c1.dynamic_friction as f64)) as f32;
            let tt = t as f64 * t as f64;
            let t2 = tt as f32;
            let f22 = f2 * f2;
            if (r as f64 - t2 as f64) * f22 as f64 >= t2 as f64 {
                // Static friction holds: move with the crease.
                return v;
            }
            let mut along = t;
            if dh < 1.0 && (DYN_RATIO as f64 * r as f64) < tt {
                let inv_t = (1.0 / t).abs();
                let k = r.sqrt() * inv_t;
                let dhd = dh as f64;
                let m = (dhd + k as f64 * (1.0 - dhd)) as f32;
                along = m * t;
            }
            return add4(scale4(along, dir), v);
        }
    }

    info.set_status(i0, 2);
    info.set_status(i1, 2);
    if c0.priority > c1.priority {
        let w = solve_1d(input, c1, vin);
        solve_1d(input, c0, w)
    } else {
        let w = solve_1d(input, c0, vin);
        solve_1d(input, c1, w)
    }
}

/// `00d23ca0` hkSimplexSolverSolve3d (Xbox PDB).
///
/// The velocity that satisfies all three planes at once (the corner where
/// they meet). If the planes are (nearly) dependent or that velocity is too
/// fast, all three get status 1 and the three pairs are solved with
/// [`solve_2d`] in turn; with `sort` set, the active planes are sorted first
/// and the first three are used instead of `ia`, `ib`, `ic`.
fn solve_3d(
    info: &mut SolverInfo,
    mut ia: usize,
    mut ib: usize,
    mut ic: usize,
    sort: bool,
    vin: V4,
) -> V4 {
    let input = info.input;
    let ca = &input.constraints[ia];
    let cb = &input.constraints[ib];
    let cc = &input.constraints[ic];
    let bc = cross(cb.plane, cc.plane);
    let ca_x = cross(cc.plane, ca.plane);
    let ab = cross(ca.plane, cb.plane);
    let det = dot3(ca.plane, bc);

    if det.abs() >= EPS || det.is_nan() {
        let dc = dot3(cc.velocity, cc.plane);
        let db = dot3(cb.velocity, cb.plane);
        let da = dot3(ca.velocity, ca.plane);
        let inv = 1.0 / det;
        let v = scale4(
            inv,
            add4(add4(scale4(da, bc), scale4(db, ca_x)), scale4(dc, ab)),
        );
        if !exceeds(v, input.max_surface_velocity) {
            return v;
        }
    }

    if sort {
        sort_info(info);
        ia = info.entries[0];
        ib = info.entries[1];
        ic = info.entries[2];
    }
    info.set_status(ia, 1);
    info.set_status(ib, 1);
    info.set_status(ic, 1);
    // The game re-checks the active count between calls; nothing here
    // changes it, so all three always run.
    let count = info.count;
    let mut out = solve_2d(info, ia, ib, vin);
    if count == info.count {
        out = solve_2d(info, ia, ic, out);
    }
    if count == info.count {
        out = solve_2d(info, ib, ic, out);
    }
    out
}

/// `00d23f50` hkSimplexSolverExamineActivePlanes (Xbox PDB).
///
/// Works out the output velocity from the active planes, dropping planes
/// that the velocity no longer pushes into. Called with the newest plane
/// last in the set.
fn examine_active_planes(info: &mut SolverInfo) {
    let input = info.input;
    let vin = input.velocity;
    'outer: loop {
        match info.count {
            1 => {
                let c = info.c(info.entries[0]);
                info.output.velocity = solve_1d(input, c, vin);
                return;
            }
            2 => {
                let v = solve_1d(input, info.c(info.entries[1]), vin);
                if !solve_test_1d(info.c(info.entries[0]), v) {
                    info.output.velocity = v;
                    info.entries[0] = info.entries[1];
                    info.count = 1;
                    return;
                }
                let (e0, e1) = (info.entries[0], info.entries[1]);
                info.output.velocity = solve_2d(info, e0, e1, vin);
                return;
            }
            3 => {
                let e2 = info.entries[2];
                let v = solve_1d(input, info.c(e2), vin);
                if !solve_test_1d(info.c(info.entries[0]), v)
                    && !solve_test_1d(info.c(info.entries[1]), v)
                {
                    info.output.velocity = v;
                    info.entries[0] = e2;
                    info.count = 1;
                    continue;
                }
                for i in 0..2 {
                    let m = solve_2d(info, info.entries[i], e2, vin);
                    let other = info.entries[1 - i];
                    if !solve_test_1d(info.c(other), m) {
                        info.entries[0] = info.entries[i];
                        info.entries[1] = info.entries[2];
                        info.count -= 1;
                        continue 'outer;
                    }
                }
                let (e0, e1) = (info.entries[0], info.entries[1]);
                info.output.velocity = solve_3d(info, e0, e1, e2, true, vin);
                return;
            }
            4 => {
                sort_info(info);
                // Try each set of three that includes the newest plane;
                // keep the first whose answer does not push into the
                // fourth.
                for j in 0..3 {
                    let k = j + 1;
                    let a = info.entries[k % 3];
                    let b = info.entries[(k + 1) % 3];
                    let e3 = info.entries[3];
                    let n = solve_3d(info, a, b, e3, false, vin);
                    if !solve_test_1d(info.c(info.entries[j]), n) {
                        info.entries[j] = info.entries[2];
                        info.entries[2] = info.entries[3];
                        info.count = 3;
                        continue 'outer;
                    }
                }
                // None works: run all four triples in turn, then drop the
                // plane with the highest status.
                let [c0, c1, c2, c3] = info.entries;
                let count = info.count;
                let mut v = vin;
                v = solve_3d(info, c0, c1, c2, false, v);
                if count == info.count {
                    v = solve_3d(info, c0, c1, c3, false, v);
                }
                if count == info.count {
                    v = solve_3d(info, c0, c2, c3, false, v);
                }
                if count == info.count {
                    v = solve_3d(info, c1, c2, c3, false, v);
                }
                info.output.velocity = v;

                let mut max = 0;
                for k in 0..4 {
                    let s = info.status(info.entries[k]);
                    if !(max > s) {
                        max = s;
                    }
                }
                if let Some(k) = (0..4).find(|&k| info.status(info.entries[k]) == max) {
                    info.entries[k] = info.entries[3];
                }
                info.count -= 1;
                for k in 0..3 {
                    info.set_status(info.entries[k], 0);
                }
            }
            _ => return,
        }
    }
}

/// `00d24360` hkSimplexSolverSolve (Xbox PDB).
///
/// Moves the character from `input.position` with `input.velocity` for up to
/// `input.delta_time`, sliding along the constraint planes.
///
/// It returns early once the time moved exceeds `input.min_delta_time`, so
/// `delta_time` in the output may be shorter than the input's. If the input
/// time is negative the game leaves the output time as the caller had it;
/// here that is 0.
pub fn solve(input: &SimplexSolverInput) -> SimplexSolverOutput {
    let n = input.constraints.len();
    let mut info = SolverInfo {
        entries: [0; 4],
        count: 0,
        total_time: 0.0,
        input,
        output: SimplexSolverOutput {
            position: input.position,
            velocity: input.velocity,
            delta_time: 0.0,
            plane_interactions: vec![SurfaceConstraintInteraction::default(); n],
        },
    };
    let mut remaining = input.delta_time;
    if !(remaining >= 0.0) {
        return info.output;
    }
    loop {
        // Find the first plane the character reaches in the time left.
        let mut best: Option<usize> = None;
        let mut best_time = remaining;
        for i in 0..n {
            if (info.count >= 1 && info.entries[0] == i)
                || (info.count >= 2 && info.entries[1] == i)
                || (info.count >= 3 && info.entries[2] == i)
                || info.output.plane_interactions[i].status != 0
            {
                continue;
            }
            let c = &input.constraints[i];
            let speed = -dot3(sub4(info.output.velocity, c.velocity), c.plane);
            if speed <= 0.0 {
                continue;
            }
            // Distance to the plane where it is now (it moves with its own
            // velocity); touching or inside counts as zero.
            let p = add4(scale4(-info.total_time, c.velocity), info.output.position);
            let pn = mul4(p, c.plane);
            let mut dist = (pn[2] + c.plane[3]) + (pn[1] + pn[0]);
            if dist <= EPS {
                dist = 0.0;
            }
            let dist = info.output.plane_interactions[i].penalty_distance + dist;
            if speed as f64 * best_time as f64 > dist as f64 {
                best_time = dist / speed;
                best = Some(i);
            }
        }

        if best_time > MIN_STEP {
            info.total_time += best_time;
            info.output.position = add4(
                scale4(best_time, info.output.velocity),
                info.output.position,
            );
            remaining -= best_time;
            for k in 0..info.count {
                let it = &mut info.output.plane_interactions[info.entries[k]];
                it.surface_time += best_time;
                it.touched = true;
            }
            info.output.delta_time = info.total_time;
            if input.min_delta_time < info.total_time {
                return info.output;
            }
        }

        let Some(hit) = best else {
            info.output.delta_time = input.delta_time;
            return info.output;
        };
        info.entries[info.count] = hit;
        info.count += 1;
        let it = &mut info.output.plane_interactions[hit];
        it.penalty_distance = ((it.penalty_distance as f64 + EPS_D) * 2.0) as f32;
        examine_active_planes(&mut info);

        if !(remaining >= 0.0) {
            return info.output;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plane(nx: f32, ny: f32, nz: f32, w: f32) -> SurfaceConstraintInfo {
        SurfaceConstraintInfo {
            plane: [nx, ny, nz, w],
            dynamic_friction: 1.0,
            ..Default::default()
        }
    }

    fn input<'a>(
        position: V4,
        velocity: V4,
        constraints: &'a [SurfaceConstraintInfo],
    ) -> SimplexSolverInput<'a> {
        SimplexSolverInput {
            position,
            velocity,
            max_surface_velocity: [10.0, 10.0, 10.0, 0.0],
            up_vector: [0.0, 0.0, 1.0, 0.0],
            delta_time: 0.1,
            min_delta_time: 0.0,
            constraints,
        }
    }

    fn near(a: V4, b: V4) -> bool {
        (0..3).all(|i| (a[i] - b[i]).abs() < 1e-5)
    }

    #[test]
    fn simplex_no_constraints_moves_freely() {
        let inp = input([1.0, 2.0, 3.0, 0.0], [2.0, 0.0, -1.0, 0.0], &[]);
        let out = solve(&inp);
        assert!(near(out.position, [1.2, 2.0, 2.9, 0.0]));
        assert_eq!(out.velocity, inp.velocity);
        assert_eq!(out.delta_time, 0.1);
        // With min_delta_time at least dt it runs to the end instead.
        let mut inp = inp;
        inp.min_delta_time = 1.0;
        let out = solve(&inp);
        assert!(near(out.position, [1.2, 2.0, 2.9, 0.0]));
        assert_eq!(out.delta_time, 0.1);
    }

    #[test]
    fn simplex_floor_slide_removes_downward_velocity() {
        let cs = [plane(0.0, 0.0, 1.0, 0.0)];
        let out = solve(&input([0.0; 4], [1.0, 0.0, -1.0, 0.0], &cs));
        assert!(
            near(out.velocity, [1.0, 0.0, 0.0, 0.0]),
            "{:?}",
            out.velocity
        );
        assert!(
            near(out.position, [0.1, 0.0, 0.0, 0.0]),
            "{:?}",
            out.position
        );
        assert_eq!(out.delta_time, 0.1);
        let it = out.plane_interactions[0];
        assert!(it.touched);
        assert!((it.surface_time - 0.1).abs() < 1e-6);
        assert_eq!(it.penalty_distance, 2.0 * EPS);
        assert_eq!(it.status, 0);
    }

    #[test]
    fn simplex_corner_with_floor_stops() {
        let cs = [
            plane(1.0, 0.0, 0.0, 0.0),
            plane(0.0, 1.0, 0.0, 0.0),
            plane(0.0, 0.0, 1.0, 0.0),
        ];
        // Straight into the crease of two walls: stops (the floor is not
        // approached).
        let out = solve(&input([0.0; 4], [-1.0, -1.0, 0.0, 0.0], &cs));
        assert!(near(out.velocity, [0.0; 4]), "{:?}", out.velocity);
        assert!(near(out.position, [0.0; 4]));
        assert!(!out.plane_interactions[2].touched);
        // Also moving down: all three planes become active and the
        // three-plane solve gives zero.
        let out = solve(&input([0.0; 4], [-1.0, -1.0, -1.0, 0.0], &cs));
        assert!(near(out.velocity, [0.0; 4]), "{:?}", out.velocity);
        assert!(near(out.position, [0.0; 4]));
        assert!(out
            .plane_interactions
            .iter()
            .all(|i| i.touched && i.status == 0));
    }

    #[test]
    fn simplex_far_plane_not_reached() {
        let cs = [plane(0.0, 0.0, 1.0, 0.0)];
        let out = solve(&input([0.0, 0.0, 10.0, 0.0], [0.0, 0.0, -1.0, 0.0], &cs));
        assert!(near(out.position, [0.0, 0.0, 9.9, 0.0]));
        assert_eq!(out.velocity, [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(
            out.plane_interactions[0],
            SurfaceConstraintInteraction::default()
        );
    }

    #[test]
    fn simplex_moving_plane_pushes() {
        let mut wall = plane(1.0, 0.0, 0.0, 0.0);
        wall.velocity = [1.0, 0.0, 0.0, 0.0];
        let cs = [wall];
        let out = solve(&input([0.0; 4], [0.0; 4], &cs));
        assert!(
            near(out.velocity, [1.0, 0.0, 0.0, 0.0]),
            "{:?}",
            out.velocity
        );
        assert!(
            near(out.position, [0.1, 0.0, 0.0, 0.0]),
            "{:?}",
            out.position
        );
        assert!(out.plane_interactions[0].touched);
    }

    #[test]
    fn simplex_four_planes_terminates() {
        let s = 1.0 / 3.0f32.sqrt();
        let cs = [
            plane(1.0, 0.0, 0.0, 0.0),
            plane(0.0, 1.0, 0.0, 0.0),
            plane(0.0, 0.0, 1.0, 0.0),
            plane(s, s, s, 0.0),
        ];
        let out = solve(&input([0.0; 4], [-1.0, -1.0, -1.0, 0.0], &cs));
        assert!(near(out.velocity, [0.0; 4]), "{:?}", out.velocity);
        assert!(near(out.position, [0.0; 4]));
    }
}
