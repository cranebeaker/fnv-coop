//! `NiCollision/run_00eab9d0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSphereBV`'s vtable (`010f1854`); its slot 0 is `NiSphereBV::LoadBinary` (`00eab860`).
const NI_SPHERE_BV_VTABLE: u32 = 0x010f_1854;
/// The memory manager instance (`this` of its allocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The three words at `011f426c` a new sphere volume takes as its center.
const DEFAULT_CENTER_011F426C: u32 = 0x011f_426c;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// A stored float loaded back (`FSTP` then `FLD`).
fn round(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `b - a` per component of two 3-float vectors, each stored.
fn difference(e: &Engine, b: u32, a: u32) -> [f64; 3] {
    [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, b.wrapping_add(offset)),
            operand(e, a.wrapping_add(offset)),
        ))
    })
}

// Translated from 00eab9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eab9d0`, cdecl): whether two moving spheres (sphere volume layout: center at
/// +4, radius at +0x10) touch within `time`. With the relative velocity `v = velocity1 -
/// velocity0` and offset `d = center1 - center0` (each component stored), `a = |v|^2` and
/// `c = |d|^2` (stored) and `r2 = (radius0 + radius1)^2` (stored): when `a > 0` and the spheres
/// approach (`b = v . d`, stored, at or below zero) the closest approach is `c * a - b^2`
/// against `a * r2` when it falls within the time (`-time * a <= b`), else the distance at the
/// end `(time * a + 2b) * time + c` against `r2`; otherwise `c` against `r2`. Answers whether
/// the distance is at most `r2`.
pub fn fn_00eab9d0(
    e: &mut Engine,
    time: f32,
    sphere0: Ptr,
    velocity0: Ptr,
    sphere1: Ptr,
    velocity1: Ptr,
) -> bool {
    let first = sphere0.addr();
    let second = sphere1.addr();
    let [vx, vy, vz] = difference(e, velocity1.addr(), velocity0.addr());
    let a = round(x87_add(
        x87_add(x87_mul(vy, vy), x87_mul(vx, vx)),
        x87_mul(vz, vz),
    ));
    let [dx, dy, dz] = difference(e, second.wrapping_add(4), first.wrapping_add(4));
    let c = x87_store32(x87_add(
        x87_add(x87_mul(dy, dy), x87_mul(dx, dx)),
        x87_mul(dz, dz),
    ));
    let reach = round(x87_add(
        load(e, first.wrapping_add(0x10)),
        operand(e, second.wrapping_add(0x10)),
    ));
    let r2 = x87_store32(x87_mul(reach, reach));
    if a > 0.0 {
        let b = round(x87_add(
            x87_add(x87_mul(vx, dx), x87_mul(vy, dy)),
            x87_mul(vz, dz),
        ));
        if b <= 0.0 {
            let t = x87_load32(time.to_bits());
            if x87_mul(-t, a) <= b {
                let closest = x87_sub(x87_mul(x87_load32(c), a), x87_mul(b, b));
                return x87_mul(a, x87_m32(r2)) >= closest;
            }
            let end = x87_add(
                x87_mul(x87_add(x87_mul(t, a), x87_add(b, b)), t),
                x87_m32(c),
            );
            return x87_load32(r2) >= end;
        }
    }
    x87_load32(r2) >= x87_load32(c)
}

// Translated from 00eac170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eac170`, cdecl): makes a sphere volume and loads it from `stream`. Allocates
/// 0x14 bytes from the memory manager; a block gets the `NiSphereBV` vtable, the center from the
/// three words at `011f426c` and the radius 1.0. Then calls the volume's vtable slot 0
/// (`LoadBinary`) with `stream`, also when the allocation failed (the exe then reads the vtable
/// at address 0). Answers the volume.
pub fn fn_00eac170(e: &mut Engine, stream: Ptr) -> Ptr {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    if block != 0 {
        e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
        for offset in [0u32, 4] {
            let word = e.mem.u32(DEFAULT_CENTER_011F426C + offset);
            e.mem.set_u32(block + 4 + offset, word);
        }
        e.mem.set_u32(block + 0x10, 1.0f32.to_bits());
        let z = e.mem.u32(DEFAULT_CENTER_011F426C + 8);
        e.mem.set_u32(block + 0xc, z);
    }
    e.vcall(block, 0, &args![stream]);
    Ptr::new(block)
}

/// The double at `0101dae8` (about `1e-5`) the end-of-time distance may exceed the reach by.
const TOLERANCE_0101DAE8: u32 = 0x0101_dae8;
/// The double 0.5 at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// The float at `0104da4c` (`2^-23` in the image): the squared distance at or below which the
/// touching spheres get the default normal.
const SMALL_0104DA4C: u32 = 0x0104_da4c;
/// The three words at `011a946c`: that default normal.
const DEFAULT_NORMAL_011A946C: u32 = 0x011a_946c;
/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;
/// The CRT square root (`00ec6040`): the value in ST0 (a leading `f64`), the root in ST0.
const CRT_SQRT: u32 = 0x00ec_6040;

/// Writes three words.
fn write3(e: &mut Engine, at: u32, words: [u32; 3]) {
    for (i, word) in (0u32..).zip(words) {
        e.mem.set_u32(at.wrapping_add(4 * i), word);
    }
}

/// `out = -from`, each coordinate loaded, negated (`FCHS`) and stored.
fn write_negated(e: &mut Engine, from: u32, out: u32) {
    let words = [0u32, 4, 8].map(|i| x87_store32(-load(e, from.wrapping_add(i))));
    write3(e, out, words);
}

/// The spheres' contact once they touch from the start: the contact normal from `normal` (into
/// `first`, normalized by `004a0c10`) and its opposite into `second`.
fn write_normals(e: &mut Engine, normal: [u32; 3], first: u32, second: u32) {
    write3(e, first, normal);
    e.call(UNITIZE, &args![first]);
    write_negated(e, first, second);
}

// Translated from 00eabb20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `fn_00e98ff0` stores it first in the second table of sphere
/// tests; cdecl, ten words): where two moving spheres (center +4, radius +0x10) first touch
/// within `time`, with the time (`out_time`), the contact point and, when `normals` (a byte)
/// is set, the two contact normals. With `v = velocity1 - velocity0`, `d = center1 - center0`,
/// `a = |v|^2`, `c = |d|^2`, `r = radius0 + radius1` and `r2 = r^2` (each stored): when `a > 0`
/// and the spheres approach (`b = v . d` not above 0), a closest approach after `time` must
/// end within `r2` plus the double at `0101dae8`; then with `q = c - r2` and
/// `disc = b^2 - a q` (stored), a negative discriminant misses; `q <= 0` touches at time 0 (the
/// point halfway between the centers; the normal `d` normalized, or the default at `011a946c`
/// when `c` is at most the float at `0104da4c`); otherwise the time is
/// `-(sqrt(disc) + b) / a` (`00ec6040`) clamped to `[0, time]`, the point lies on the moved
/// offset at `radius0 / r` from the moved first center, and the normal is the moved offset.
/// Without approach the spheres must already touch (`r2 >= c`): time 0, the point on the
/// slower sphere's surface toward the other and the normals along the centers (always written).
/// Answers whether they touch.
#[allow(clippy::too_many_arguments)]
pub fn fn_00eabb20(
    e: &mut Engine,
    time: f32,
    sphere0: Ptr,
    velocity0: Ptr,
    sphere1: Ptr,
    velocity1: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    normals: u8,
    out_normal0: Ptr,
    out_normal1: Ptr,
) -> bool {
    let (s0, s1) = (sphere0.addr(), sphere1.addr());
    let (v0, v1) = (velocity0.addr(), velocity1.addr());
    let (time_out, point_out) = (out_time.addr(), out_point.addr());
    let (n0, n1) = (out_normal0.addr(), out_normal1.addr());
    let [vx, vy, vz] = difference(e, v1, v0);
    let a = round(x87_add(
        x87_mul(vz, vz),
        x87_add(x87_mul(vy, vy), x87_mul(vx, vx)),
    ));
    let [dx, dy, dz] = difference(e, s1.wrapping_add(4), s0.wrapping_add(4));
    let offset = [dx, dy, dz].map(x87_store32);
    let c = round(x87_add(
        x87_add(x87_mul(dx, dx), x87_mul(dy, dy)),
        x87_mul(dz, dz),
    ));
    let reach = round(x87_add(
        load(e, s1.wrapping_add(0x10)),
        operand(e, s0.wrapping_add(0x10)),
    ));
    let r2 = round(x87_mul(reach, reach));
    let t = x87_load32(time.to_bits());
    let b = if a > 0.0 {
        let s = x87_add(x87_mul(vx, dx), x87_mul(vy, dy));
        Some(round(x87_add(x87_mul(vz, dz), s)))
    } else {
        None
    };
    if let Some(b) = b.filter(|b| *b <= 0.0) {
        if !(x87_mul(-t, a) <= b) {
            // The closest approach comes after `time`: the distance at its end.
            let x = x87_add(x87_mul(a, t), x87_add(b, b));
            let end = x87_add(x87_mul(t, x), c);
            let tolerance = f64::from_bits(e.mem.u64(TOLERANCE_0101DAE8));
            if !(x87_add(tolerance, r2) >= end) {
                return false;
            }
        }
        let q = round(x87_sub(c, r2));
        let disc = round(x87_sub(x87_mul(b, b), x87_mul(a, q)));
        if disc < 0.0 || disc.is_nan() {
            return false;
        }
        if q <= 0.0 {
            // Touching from the start.
            e.mem.set_u32(time_out, 0);
            let half = x87_load64(e.mem.u64(HALF_01011588));
            let sum = [4u32, 8, 0xc].map(|i| {
                round(x87_add(
                    load(e, s0.wrapping_add(i)),
                    operand(e, s1.wrapping_add(i)),
                ))
            });
            let point = [
                x87_store32(x87_mul(sum[0], half)),
                x87_store32(x87_mul(sum[1], half)),
                x87_store32(x87_mul(half, sum[2])),
            ];
            write3(e, point_out, point);
            if normals != 0 {
                let normal = if c > operand(e, SMALL_0104DA4C) {
                    offset
                } else {
                    [0u32, 4, 8].map(|i| e.mem.u32(DEFAULT_NORMAL_011A946C + i))
                };
                write_normals(e, normal, n0, n1);
            }
            return true;
        }
        let root = round(e.call(CRT_SQRT, &args![disc]).f64());
        let first = round(-x87_div(x87_add(root, b), a));
        e.mem.set_u32(time_out, x87_store32(first));
        if 0.0 > first {
            e.mem.set_u32(time_out, 0);
        } else if t < first {
            e.mem.set_u32(time_out, x87_store32(t));
        }
        let at = load(e, time_out);
        let moved = [
            round(x87_mul(at, vx)),
            round(x87_mul(vy, at)),
            round(x87_mul(at, vz)),
        ];
        let moved_offset = [0usize, 1, 2].map(|i| round(x87_add(moved[i], [dx, dy, dz][i])));
        let share = round(x87_div(load(e, s0.wrapping_add(0x10)), reach));
        let toward = [
            round(x87_mul(moved_offset[0], share)),
            round(x87_mul(moved_offset[1], share)),
            round(x87_mul(share, moved_offset[2])),
        ];
        let moved0 = [0u32, 4, 8].map(|i| round(x87_mul(at, operand(e, v0.wrapping_add(i)))));
        let center0 = [0u32, 1, 2].map(|i| {
            round(x87_add(
                load(e, s0.wrapping_add(4 + 4 * i)),
                moved0[i as usize],
            ))
        });
        let point = [0usize, 1, 2].map(|i| x87_store32(x87_add(center0[i], toward[i])));
        write3(e, point_out, point);
        if normals != 0 {
            write_normals(e, moved_offset.map(x87_store32), n0, n1);
        }
        return true;
    }
    // Not approaching: they must touch already.
    if !(r2 >= c) {
        return false;
    }
    e.mem.set_u32(time_out, 0);
    let speed_squared = |e: &Engine, v: u32| {
        let [x, y, z] = [0u32, 4, 8].map(|i| load(e, v.wrapping_add(i)));
        round(x87_add(
            x87_mul(z, z),
            x87_add(x87_mul(x, x), x87_mul(y, y)),
        ))
    };
    let w1 = speed_squared(e, v1);
    let w0 = speed_squared(e, v0);
    let (moving, still, normal_at, opposite) = if w0 < w1 {
        (s1, s0, n0, n1)
    } else {
        (s0, s1, n1, n0)
    };
    let normal = [4u32, 8, 0xc].map(|i| {
        x87_store32(x87_sub(
            load(e, moving.wrapping_add(i)),
            operand(e, still.wrapping_add(i)),
        ))
    });
    write3(e, normal_at, normal);
    e.call(UNITIZE, &args![normal_at]);
    let radius = round(load(e, still.wrapping_add(0x10)));
    let point = if w0 < w1 {
        let along = [
            round(x87_mul(load(e, normal_at), radius)),
            round(x87_mul(load(e, normal_at + 4), radius)),
            round(x87_mul(radius, operand(e, normal_at + 8))),
        ];
        [0u32, 1, 2].map(|i| {
            x87_store32(x87_add(
                load(e, still.wrapping_add(4 + 4 * i)),
                along[i as usize],
            ))
        })
    } else {
        let along = [
            round(x87_mul(radius, operand(e, normal_at))),
            round(x87_mul(load(e, normal_at + 4), radius)),
            round(x87_mul(radius, operand(e, normal_at + 8))),
        ];
        [
            x87_store32(x87_add(along[0], operand(e, still.wrapping_add(4)))),
            x87_store32(x87_add(load(e, still.wrapping_add(8)), along[1])),
            x87_store32(x87_add(load(e, still.wrapping_add(0xc)), along[2])),
        ]
    };
    write3(e, point_out, point);
    write_negated(e, normal_at, opposite);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ea_b9d0,
            fn_00eab9d0(f32, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ea_c170, fn_00eac170(Ptr) -> Ptr),
        entry!(
            0x00ea_bb20,
            fn_00eabb20(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u8, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    #[test]
    fn moving_spheres_touch_within_the_time() {
        let mut e = Engine::new();
        let first = floats(&mut e, &[0.0, 0.0, 0.0, 0.0, 1.0]);
        let second = floats(&mut e, &[0.0, 10.0, 0.0, 0.0, 1.0]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let toward = floats(&mut e, &[-4.0, 0.0, 0.0]);
        let away = floats(&mut e, &[4.0, 0.0, 0.0]);
        let mut test = |time: f32, v1: u32| {
            e.call(0x00ea_b9d0, &args![time, first, still, second, v1])
                .bool()
        };
        // Apart and still, moving away, too slow, then fast enough (the end and the middle).
        assert!(!test(1.0, still));
        assert!(!test(5.0, away));
        assert!(!test(1.0, toward));
        assert!(test(2.0, toward));
        assert!(test(5.0, toward));
        // Touching at rest.
        let near = floats(&mut e, &[0.0, 2.0, 0.0, 0.0, 1.0]);
        let words = args![0.0f32, first, still, near, still];
        assert!(e.call(0x00ea_b9d0, &words).bool());
    }

    #[test]
    fn create_from_stream_loads_the_new_sphere() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.map(0x011f_4000, 0x3000);
        e.mem.set_u32(NI_SPHERE_BV_VTABLE, 0x00ab_5030);
        for (axis, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem
                .set_f32(DEFAULT_CENTER_011F426C + 4 * axis as u32, *value);
        }
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(0x14).into_ret());
        e.register(0x00ab_5030, |e, a| {
            assert_eq!(e.mem.f32(a[0] + 0x10), 1.0);
            assert_eq!(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ea_c170, &args![0x77u32]).u32();
        assert_eq!(e.mem.u32(made), NI_SPHERE_BV_VTABLE);
        let center: Vec<f32> = (0..3).map(|i| e.mem.f32(made + 4 + 4 * i)).collect();
        assert_eq!(center, vec![1.0, 2.0, 3.0]);
        let loads = e
            .call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(addr, _)| *addr == 0x00ab_5030)
            .count();
        assert_eq!(loads, 1);
    }
}

#[cfg(test)]
mod find_tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + 4 * i, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn setup(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0104_d000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.mem.set_f64(TOLERANCE_0101DAE8, 1e-5);
        e.mem.set_f32(SMALL_0104DA4C, 1.0e-7);
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(UNITIZE, |e, a| {
            let v = read(e, a[0], 3);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, x) in (0u32..).zip(v) {
                e.mem.set_f32(a[0] + 4 * i, x / length);
            }
            Ret::default()
        });
    }

    /// Runs the test with sphere 0 at the origin and sphere 1 at `x1` (radii 1), velocity 1
    /// `(vx1, 0, 0)`; answers (hit, time, point, normal0, normal1).
    #[allow(clippy::type_complexity)]
    fn find(e: &mut Engine, x1: f32, vx1: f32) -> (bool, f32, Vec<f32>, Vec<f32>, Vec<f32>) {
        let s0 = floats(e, &[0.0, 0.0, 0.0, 0.0, 1.0]);
        let s1 = floats(e, &[0.0, x1, 0.0, 0.0, 1.0]);
        let v0 = floats(e, &[0.0, 0.0, 0.0]);
        let v1 = floats(e, &[vx1, 0.0, 0.0]);
        let outs = floats(e, &[9.0; 10]);
        let hit = e
            .call(
                0x00ea_bb20,
                &args![
                    1.0f32,
                    s0,
                    v0,
                    s1,
                    v1,
                    outs,
                    outs + 4,
                    1u32,
                    outs + 16,
                    outs + 28
                ],
            )
            .bool();
        (
            hit,
            e.mem.f32(outs),
            read(e, outs + 4, 3),
            read(e, outs + 16, 3),
            read(e, outs + 28, 3),
        )
    }

    #[test]
    fn approaching_spheres_touch_at_their_first_contact() {
        let mut e = Engine::new();
        setup(&mut e);
        let (hit, time, point, n0, n1) = find(&mut e, 5.0, -4.0);
        assert!(hit);
        assert_eq!(time, 0.75);
        assert_eq!(point, vec![1.0, 0.0, 0.0]);
        assert_eq!(n0, vec![1.0, 0.0, 0.0]);
        assert_eq!(n1, vec![-1.0, 0.0, 0.0]);
        // Too far to reach within the time.
        assert!(!find(&mut e, 10.0, -4.0).0);
        // Already touching while approaching: time 0, the point halfway.
        let (hit, time, point, n0, _) = find(&mut e, 1.0, -1.0);
        assert!(hit);
        assert_eq!(time, 0.0);
        assert_eq!(point, vec![0.5, 0.0, 0.0]);
        assert_eq!(n0, vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn resting_spheres_touch_only_when_they_overlap() {
        let mut e = Engine::new();
        setup(&mut e);
        let (hit, time, point, n0, n1) = find(&mut e, 1.0, 0.0);
        assert!(hit);
        assert_eq!(time, 0.0);
        assert_eq!(point, vec![0.0, 0.0, 0.0]);
        assert_eq!(n1, vec![-1.0, 0.0, 0.0]);
        assert_eq!(n0, vec![1.0, 0.0, 0.0]);
        assert!(!find(&mut e, 3.0, 0.0).0);
    }
}
