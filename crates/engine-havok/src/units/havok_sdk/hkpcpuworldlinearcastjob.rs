//! `Havok SDK/hkpcpuworldlinearcastjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, profile_record, store_vec, zip_with, HALF_F32, PROFILE_END,
};

/// Profiling label `"TtCollQueryWorldLinearCast"`.
const LABEL_TT_LINEAR_CAST: u32 = 0x010d_b9ec;
/// `float` constant: the largest float below infinity (`0x7f7fffee`).
const FLOAT_MAX: u32 = 0x0104_da48;
/// `float` constant: 2^-23.
const FLOAT_EPSILON: u32 = 0x0104_da4c;
/// Four floats of 1.0.
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// The vtable of the collector object the function builds (scalar deleting destructor first).
const COLLECTOR_VTABLE: u32 = 0x0108_f288;
/// The vtables of the four listener objects.
const LISTENER_VTABLES: [u32; 4] = [0x010d_b9e4, 0x010d_b9d0, 0x010d_b9c4, 0x010d_b9b8];
/// Unnamed (cdecl, seven words: an object, the box, the sweep, a listener, a float and two
/// zeros).
const CAST_WITH_LISTENER_A: u32 = 0x00d1_cc50;
/// Unnamed (cdecl, seven words, like `00d1cc50`).
const CAST_WITH_LISTENER_B: u32 = 0x00d1_c2b0;
/// Unnamed (cdecl, five words: the address of an output byte, an object, the sweep, a listener
/// and a float).
const CAST_WITH_LISTENER_C: u32 = 0x00df_2750;

// Layout of the stack frame the function builds, relative to the lowest word its code uses.
/// The collector object (vtable, the distance 0x7f7fffee, two copies of the entry's word at
/// `+0x24`, the entry's word at `+0x28`, the hit word).
const COLLECTOR: u32 = 0x20;
/// The box the shape's virtual function writes (minimum, maximum; 32 bytes).
const BOX: u32 = 0x40;
/// The sweep made from the box (three vectors).
const SWEEP: u32 = 0x60;
/// The four listener objects (vtable words).
const LISTENERS: u32 = 0xa0;
/// The offset vector (entry origin minus the body position) the sweep is moved by.
const OFFSET: u32 = 0x130;
/// The copy of the sweep the function keeps (extent, centre plus offset, centre).
const SWEEP_COPY: u32 = 0x140;

// Translated from 00df7340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuWorldLinearCastJob` (Xbox PDB name per the engine map): cdecl, two stack words: a word
/// that becomes `ECX` of the final call `00d04e80`, and the job. Writes
/// `TtCollQueryWorldLinearCast`, builds the objects the casts use on its stack (a collector
/// with the distance 0x7f7fffee, four listener objects with the group's word at `+0xC` and the
/// collector, the words of the group at `[job + 0x18]` at `+0` to `+0x14`, `+0x20` to `+0x4F`
/// and `+0x70`) and for each of the `[job + 0x28]` 0x30-byte entries at `[job + 0x24]` resets the
/// collector for it, takes the entry's origin minus the vector at `+0x30` of the matrix `[[entry
/// + 0x20] + 8]`, its length (`SQRTSS` of the sum of the three squares, added as `y + x` first
/// and `z` last) and the entry's words at `+0x10` and `+0x14`, and calls the virtual function at
/// slot `0x1C` of `[[entry + 0x20]]` with `[[entry + 0x20] + 8]`, the float at `entry + 0x14`
/// and a 32-byte box. With no listener at `job + 0x2C` the sweep (box centre times 0.5, plus the
/// offset, and half extent) goes to `00d1cc50` and `00d1c2b0` (when `job + 0x30` or `+0x34` is
/// set, with the listeners at `+0xA8` and `+0xAC`) and to `00df2750` (when `job + 0x38` is
/// set, listener at `+0xA4`); with a listener the interpolated vector `(1 - 0.5) * min + 0.5 *
/// max`, that plus the offset, and half the extent go to its virtual function at `0x6C` with the
/// listener at `+0xA0`. The hit word of the collector goes to the entry's word at `+0x2C`. The
/// frame words the exe leaves unwritten are zero here. Ends with `00d04e80` and returns what it
/// returns.
pub fn hk_cpu_world_linear_cast_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_TT_LINEAR_CAST);
    let job = job.addr();
    let group = e.mem.u32(job + 0x18);
    let float_max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
    let epsilon = x87_quiet_bits(e.mem.u32(FLOAT_EPSILON));
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        e.mem.set_u32(f + COLLECTOR, COLLECTOR_VTABLE);
        e.mem.set_u32(f + COLLECTOR + 4, float_max);
        e.mem.set_u32(f + COLLECTOR + 0x10, 1);
        e.mem.set_u32(f + 0x120, epsilon);
        for (index, vtable) in LISTENER_VTABLES.iter().enumerate() {
            e.mem.set_u32(f + LISTENERS + 4 * index as u32, *vtable);
        }
        let shared = e.mem.u32(group + 0xc);
        e.mem.set_u32(f + 0xb0, shared);
        e.mem.set_u32(f + 0xb4, f + COLLECTOR);
        for word in 0..6 {
            let value = e.mem.u32(group + 4 * word);
            e.mem.set_u32(f + 0xc0 + 4 * word, value);
        }
        for word in 0..12 {
            let value = e.mem.u32(group + 0x20 + 4 * word);
            e.mem.set_u32(f + 0xe0 + 4 * word, value);
        }
        let last = e.mem.u32(group + 0x70);
        e.mem.set_u32(f + 0x128, last);
        let mut index = 0;
        while index < e.mem.i32(job + 0x28) {
            let entry = e.mem.u32(job + 0x24).wrapping_add(0x30 * index as u32);
            let seed = e.mem.u32(entry + 0x24);
            let third = e.mem.u32(entry + 0x28);
            e.mem.set_u32(f + COLLECTOR + 4, float_max);
            e.mem.set_u32(f + COLLECTOR, COLLECTOR_VTABLE);
            e.mem.set_u32(f + COLLECTOR + 8, seed);
            e.mem.set_u32(f + COLLECTOR + 0xc, seed);
            e.mem.set_u32(f + COLLECTOR + 0x10, third);
            e.mem.set_u32(f + COLLECTOR + 0x14, 0);
            let body = e.mem.u32(entry + 0x20);
            let transform = e.mem.u32(body + 8);
            let position = load_vec(e, transform + 0x30);
            let origin = load_vec(e, entry);
            e.mem.set_u32(f + 0xb8, body);
            let body = e.mem.u32(entry + 0x20);
            let shape = e.mem.u32(body);
            let shape_word = e.mem.u32(shape + 0xc);
            e.mem.set_u32(f + 0xbc, shape_word);
            let fourth = x87_quiet_bits(e.mem.u32(entry + 0x10));
            e.mem.set_u32(f + 0x120, fourth);
            let offset = zip_with(origin, position, |a, b| a - b);
            let fifth = x87_quiet_bits(e.mem.u32(entry + 0x14));
            store_vec(e, f + 0x110, offset);
            store_vec(e, f + OFFSET, offset);
            e.mem.set_u32(f + 0x38, fifth);
            let squares = zip_with(offset, offset, |a, b| a * b);
            let length = (squares[2] + (squares[1] + squares[0])).sqrt();
            e.mem.set_f32(f + 0x3c, length);
            let length_bits = x87_quiet_bits(e.mem.u32(f + 0x3c));
            e.mem.set_u32(f + 0x124, length_bits);
            let copied = x87_quiet_bits(e.mem.u32(f + 0x38));
            e.mem.set_u32(f + 0xc8, copied);
            let body = e.mem.u32(entry + 0x20);
            let argument = x87_quiet_bits(e.mem.u32(entry + 0x14));
            let listener = e.mem.u32(job + 0x2c);
            let shape = e.mem.u32(body);
            let shape_argument = e.mem.u32(body + 8);
            e.vcall(shape, 0x1c, &args![shape_argument, argument, f + BOX]);
            let half = e.mem.f32(HALF_F32);
            let minimum = load_vec(e, f + BOX);
            let maximum = load_vec(e, f + BOX + 0x10);
            if listener == 0 {
                let halves = [half; 4];
                let sum = zip_with(maximum, minimum, |a, b| a + b);
                let centre = zip_with(halves, sum, |a, b| a * b);
                let moved = zip_with(centre, load_vec(e, f + OFFSET), |a, b| a + b);
                let difference = zip_with(maximum, minimum, |a, b| a - b);
                let extent = zip_with(halves, difference, |a, b| a * b);
                store_vec(e, f + SWEEP_COPY + 0x20, centre);
                store_vec(e, f + SWEEP_COPY + 0x10, moved);
                store_vec(e, f + SWEEP_COPY, extent);
                let first_target = e.mem.u32(job + 0x30);
                let second_target = e.mem.u32(job + 0x34);
                if first_target != 0 || second_target != 0 {
                    store_vec(e, f + SWEEP, centre);
                    store_vec(e, f + SWEEP + 0x10, moved);
                    store_vec(e, f + SWEEP + 0x20, extent);
                    e.call(
                        CAST_WITH_LISTENER_A,
                        &args![
                            first_target,
                            f + BOX,
                            f + SWEEP,
                            f + LISTENERS + 8,
                            0u32,
                            0u32,
                            0u32
                        ],
                    );
                    e.call(
                        CAST_WITH_LISTENER_B,
                        &args![
                            second_target,
                            f + BOX,
                            f + SWEEP,
                            f + LISTENERS + 0xc,
                            0u32,
                            0u32,
                            0u32
                        ],
                    );
                }
                let third_target = e.mem.u32(job + 0x38);
                if third_target != 0 {
                    for (from, to) in [(0x20, 0), (0x10, 0x10), (0, 0x20)] {
                        let value = load_vec(e, f + SWEEP_COPY + from);
                        store_vec(e, f + SWEEP + to, value);
                    }
                    e.call(
                        CAST_WITH_LISTENER_C,
                        &args![f + 0x1f, third_target, f + SWEEP, f + LISTENERS + 4, 0u32],
                    );
                }
            } else {
                e.mem.set_u32(f + 0x90, 0);
                let ones = load_vec(e, ONES_VECTOR);
                let halves = [half; 4];
                let weights = zip_with(ones, halves, |a, b| a - b);
                let near = zip_with(weights, minimum, |a, b| a * b);
                let far = zip_with(maximum, halves, |a, b| a * b);
                let blend = zip_with(near, far, |a, b| a + b);
                store_vec(e, f + SWEEP, blend);
                let moved = zip_with(blend, load_vec(e, f + OFFSET), |a, b| a + b);
                store_vec(e, f + SWEEP + 0x10, moved);
                let difference = zip_with(maximum, minimum, |a, b| a - b);
                let extent = zip_with(halves, difference, |a, b| a * b);
                store_vec(e, f + SWEEP + 0x20, extent);
                e.mem.set_u32(f + 0x90, 0);
                e.vcall(listener, 0x6c, &args![f + SWEEP, f + LISTENERS]);
            }
            let hit = e.mem.u32(f + COLLECTOR + 0x14);
            e.mem.set_u32(entry + 0x2c, hit);
            index += 1;
        }
    });
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00df7340,
        hk_cpu_world_linear_cast_job(u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, logging_object, Log,
    };
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    const SHAPE: u32 = 0x0900_0000;
    const LISTENER: u32 = 0x0910_0000;

    /// An object whose vtable has only the slot `slot`, which points at `base + slot` (where
    /// the test registers a double).
    fn object_with_slot(e: &mut Engine, base: u32, slot: u32) -> u32 {
        let vtable = e.mem.alloc(slot + 4);
        e.mem.set_u32(vtable + slot, base + slot);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The offsets in the frame whose words the shape's virtual function call logs.
    const WATCHED: [u32; 16] = [
        0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0xb0, 0xb4, 0xb8, 0xbc, 0xc8, 0x120, 0x124,
        0x128,
    ];

    struct Scene {
        e: Engine,
        rig: JobRig,
        job: u32,
        entry: u32,
        body: u32,
    }

    /// A job with one entry. The entry has the origin (1, 2, 3, 4), the floats 0.25 and 8.0 at
    /// `+0x10` and `+0x14`, the words 0x2424 and 0x2828 at `+0x24` and `+0x28` and a body whose
    /// position is (0.5, 0.5, 0.5, 0.5) and whose shape has the word 0xC0C at `+0xC`; the shape's
    /// virtual function at `0x1C` logs the frame and answers the box (-1, -2, -3, 0) to (3, 2, 1,
    /// 0). The exe's constant for 0.5 is replaced by `half`.
    fn scene(half: f32) -> Scene {
        let (mut e, rig) = job_rig();
        e.mem.set_u32(FLOAT_MAX, 0x7f7f_ffee);
        e.mem.set_u32(FLOAT_EPSILON, 0x3400_0000);
        e.mem.set_f32(HALF_F32, half);
        for lane in 0..4 {
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
        }
        let shape = logging_object(&mut e, &rig.log, SHAPE);
        e.mem.set_u32(shape + 0xc, 0xc0c);
        let sink = rig.log.clone();
        e.register_double(SHAPE + 0x1c, move |e, a| {
            let frame = a[3] - BOX;
            let mut words = a.to_vec();
            words.extend(WATCHED.iter().map(|offset| e.mem.u32(frame + offset)));
            for base in [0x110, 0x130] {
                words.extend((0..4).map(|lane| e.mem.u32(frame + base + 4 * lane)));
            }
            sink.borrow_mut().push((SHAPE + 0x1c, words));
            for (lane, (low, high)) in [(-1.0f32, 3.0f32), (-2.0, 2.0), (-3.0, 1.0), (0.0, 0.0)]
                .into_iter()
                .enumerate()
            {
                e.mem.set_f32(a[3] + 4 * lane as u32, low);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane as u32, high);
            }
            Ret::default()
        });
        let group = e.mem.alloc(0x80);
        for word in 0..32 {
            e.mem.set_u32(group + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(group + 0x70, 0x777);
        let transform = e.mem.alloc(0x40);
        for lane in 0..4 {
            e.mem.set_f32(transform + 0x30 + 4 * lane, 0.5);
        }
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, transform);
        let entry = e.mem.alloc(0x30);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(entry + 4 * lane as u32, value);
        }
        e.mem.set_f32(entry + 0x10, 0.25);
        e.mem.set_f32(entry + 0x14, 8.0);
        e.mem.set_u32(entry + 0x20, body);
        e.mem.set_u32(entry + 0x24, 0x2424);
        e.mem.set_u32(entry + 0x28, 0x2828);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, group);
        e.mem.set_u32(job + 0x24, entry);
        e.mem.set_u32(job + 0x28, 1);
        Scene {
            e,
            rig,
            job,
            entry,
            body,
        }
    }

    /// A double at `address` that logs its arguments and then the 12 words of the sweep (the
    /// argument 2); with `hit` it stores that word in the collector of the frame the sweep is in.
    fn sweep_double(e: &mut Engine, log: &Log, address: u32, hit: Option<u32>) {
        let sink = log.clone();
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..12).map(|word| e.mem.u32(a[2] + 4 * word)));
            sink.borrow_mut().push((address, words));
            if let Some(hit) = hit {
                e.mem.set_u32(a[2] - SWEEP + COLLECTOR + 0x14, hit);
            }
            Ret::default()
        });
    }

    fn bits(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    #[test]
    fn a_cast_without_a_listener_sweeps_the_box_through_the_three_targets() {
        let mut s = scene(0.5);
        sweep_double(&mut s.e, &s.rig.log, CAST_WITH_LISTENER_A, Some(0x9999));
        sweep_double(&mut s.e, &s.rig.log, CAST_WITH_LISTENER_B, None);
        sweep_double(&mut s.e, &s.rig.log, CAST_WITH_LISTENER_C, None);
        s.e.mem.set_u32(s.job + 0x30, 0x3000);
        s.e.mem.set_u32(s.job + 0x34, 0x3400);
        s.e.mem.set_u32(s.job + 0x38, 0x3800);
        let returned = s.e.call(0x00df_7340, &args![0x99u32, s.job]).u32();
        s.rig.assert_finished(returned, 0x99, s.job);
        // The shape call sees the collector reset for the entry, the words copied from the
        // entry and the offset (1, 2, 3, 4) - (0.5, 0.5, 0.5, 0.5) with its length.
        let offset = [0.5f32, 1.5, 2.5, 3.5];
        let length = (6.25f32 + (2.25 + 0.25)).sqrt();
        let shapes = calls(&s.rig.log, SHAPE + 0x1c);
        assert_eq!(shapes.len(), 1);
        let frame = shapes[0][3] - BOX;
        let shape = s.e.mem.u32(s.body);
        let transform = s.e.mem.u32(s.body + 8);
        assert_eq!(shapes[0][..3], [shape, transform, 8.0f32.to_bits()]);
        let mut expected = vec![
            COLLECTOR_VTABLE,
            0x7f7f_ffee,
            0x2424,
            0x2424,
            0x2828,
            0,
            8.0f32.to_bits(),
            length.to_bits(),
            0x103,
            frame + COLLECTOR,
            s.body,
            0xc0c,
            8.0f32.to_bits(),
            0.25f32.to_bits(),
            length.to_bits(),
            0x777,
        ];
        expected.extend(bits(offset));
        expected.extend(bits(offset));
        assert_eq!(shapes[0][4..], expected[..]);
        // The sweep: centre (1, 0, -1, 0), that plus the offset, and the half extent.
        let mut sweep = bits([1.0, 0.0, -1.0, 0.0]);
        sweep.extend(bits([1.5, 1.5, 1.5, 3.5]));
        sweep.extend(bits([2.0, 2.0, 2.0, 0.0]));
        for (address, first, listener) in [
            (CAST_WITH_LISTENER_A, 0x3000, 8),
            (CAST_WITH_LISTENER_B, 0x3400, 0xc),
        ] {
            let cast = calls(&s.rig.log, address);
            assert_eq!(cast.len(), 1);
            assert_eq!(
                cast[0][..7],
                [
                    first,
                    frame + BOX,
                    frame + SWEEP,
                    frame + LISTENERS + listener,
                    0,
                    0,
                    0
                ]
            );
            assert_eq!(cast[0][7..], sweep[..]);
        }
        let cast = calls(&s.rig.log, CAST_WITH_LISTENER_C);
        assert_eq!(cast.len(), 1);
        assert_eq!(
            cast[0][..5],
            [
                frame + 0x1f,
                0x3800,
                frame + SWEEP,
                frame + LISTENERS + 4,
                0
            ]
        );
        assert_eq!(cast[0][5..], sweep[..]);
        // The word the first cast left in the collector goes to the entry.
        assert_eq!(s.e.mem.u32(s.entry + 0x2c), 0x9999);
        assert_eq!(
            s.rig.labels(&s.e, false),
            vec![LABEL_TT_LINEAR_CAST, PROFILE_END]
        );
    }

    #[test]
    fn a_cast_with_a_listener_hands_it_the_interpolated_sweep() {
        let mut s = scene(0.25);
        for address in [
            CAST_WITH_LISTENER_A,
            CAST_WITH_LISTENER_B,
            CAST_WITH_LISTENER_C,
        ] {
            log_double(&mut s.e, &s.rig.log, address, 0);
        }
        let listener = object_with_slot(&mut s.e, LISTENER, 0x6c);
        let sink = s.rig.log.clone();
        s.e.register_double(LISTENER + 0x6c, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..12).map(|word| e.mem.u32(a[1] + 4 * word)));
            words.push(e.mem.u32(a[1] + 0x30));
            sink.borrow_mut().push((LISTENER + 0x6c, words));
            e.mem.set_u32(a[2] - LISTENERS + COLLECTOR + 0x14, 0x7777);
            Ret::default()
        });
        s.e.mem.set_u32(s.job + 0x2c, listener);
        s.e.mem.set_u32(s.job + 0x30, 0x3000);
        s.e.mem.set_u32(s.job + 0x38, 0x3800);
        let returned = s.e.call(0x00df_7340, &args![0x99u32, s.job]).u32();
        s.rig.assert_finished(returned, 0x99, s.job);
        let heard = calls(&s.rig.log, LISTENER + 0x6c);
        assert_eq!(heard.len(), 1);
        let frame = heard[0][2] - LISTENERS;
        assert_eq!(heard[0][..3], [listener, frame + SWEEP, frame + LISTENERS]);
        // 0.75 * minimum + 0.25 * maximum, that plus the offset and a quarter of the extent;
        // the word after the sweep is zero.
        let mut sweep = bits([0.0, -1.0, -2.0, 0.0]);
        sweep.extend(bits([0.5, 0.5, 0.5, 3.5]));
        sweep.extend(bits([1.0, 1.0, 1.0, 0.0]));
        sweep.push(0);
        assert_eq!(heard[0][3..], sweep[..]);
        for address in [
            CAST_WITH_LISTENER_A,
            CAST_WITH_LISTENER_B,
            CAST_WITH_LISTENER_C,
        ] {
            assert!(calls(&s.rig.log, address).is_empty());
        }
        assert_eq!(s.e.mem.u32(s.entry + 0x2c), 0x7777);
    }

    #[test]
    fn without_targets_only_the_shape_is_asked() {
        let mut s = scene(0.5);
        for address in [
            CAST_WITH_LISTENER_A,
            CAST_WITH_LISTENER_B,
            CAST_WITH_LISTENER_C,
        ] {
            log_double(&mut s.e, &s.rig.log, address, 0);
        }
        let returned = s.e.call(0x00df_7340, &args![0x99u32, s.job]).u32();
        s.rig.assert_finished(returned, 0x99, s.job);
        assert_eq!(calls(&s.rig.log, SHAPE + 0x1c).len(), 1);
        for address in [
            CAST_WITH_LISTENER_A,
            CAST_WITH_LISTENER_B,
            CAST_WITH_LISTENER_C,
        ] {
            assert!(calls(&s.rig.log, address).is_empty());
        }
        assert_eq!(s.e.mem.u32(s.entry + 0x2c), 0);
    }
}
