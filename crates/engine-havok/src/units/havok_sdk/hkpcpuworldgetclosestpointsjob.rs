//! `Havok SDK/hkpcpuworldgetclosestpointsjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// Profiling label `"TtCollQueryWorldGetClosestPoints"`.
const LABEL_TT_GET_CLOSEST_POINTS: u32 = 0x010d_b954;
/// `float` constant: the largest float below infinity (`0x7f7fffee`).
const FLOAT_MAX: u32 = 0x0104_da48;
/// `float` constant: 2^-23.
const FLOAT_EPSILON: u32 = 0x0104_da4c;
/// `double` constant: 0.5.
const HALF: u32 = 0x0101_1588;
/// The vtable of the collector object the function builds (scalar deleting destructor first).
const COLLECTOR_VTABLE: u32 = 0x0108_f288;
/// The vtables of the three listener sub-objects.
const LISTENER_VTABLES: [u32; 3] = [0x010d_b94c, 0x010d_b940, 0x010d_b92c];
/// Unnamed (cdecl, three words: an object, the points and a listener sub-object).
const COLLECT_WITH_LISTENER_A: u32 = 0x00d1_d500;
/// Unnamed (cdecl, three words: an object, the points and a listener sub-object).
const COLLECT_WITH_LISTENER_B: u32 = 0x00d1_b9b0;

// Layout of the stack frame the function builds, relative to the lowest word its code uses.
/// The collector object (vtable, the distance 0x7f7fffee, two copies of the entry's second word,
/// the entry's third word, the hit flag).
const COLLECTOR: u32 = 0x18;
/// The three listener sub-objects (vtable words), then the shared word and the collector pointer.
const LISTENERS: u32 = 0x40;
/// The words taken from the group and the entry for the callee.
const INPUT: u32 = 0x4c;
/// The output buffer of the first virtual call (44 bytes).
const POINTS: u32 = 0xd0;

// Translated from 00df6b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuWorldGetClosestPointsJob` (Xbox PDB name per the engine map): cdecl, two stack words: a
/// word that becomes `ECX` of the final call `00d04e80`, and the job. Writes `TtCollQueryWorld
/// GetClosestPoints`, builds the objects the queries use on its stack (a collector with the
/// distance 0x7f7fffee, three listener objects with the group's word at `+0xC` and the
/// collector, the words of the group at `[job + 0x18]` that the queries read at `+0` to `+0x14`,
/// `+0x20` to `+0x4F` and `+0x70`, the callbacks at `job + 0x20`, `+0x24` and `+0x28`) and, for each
/// of the `[job + 0x38]` 16-byte entries at `[job + 0x34]`, resets the collector for it, calls the
/// virtual function at slot `0x1C` of the object `[[entry]]` with `[[entry] + 8]`, the float
/// `[job + 0x2C] - [group + 8] * 0.5` and the points buffer, then the callback object's
/// virtual function at `0x74` (when `[job + 0x20]` is set), `00d1d500` (`[job + 0x24]`) and
/// `00d1b9b0` (`[job + 0x28]`), and stores the collector's hit word in the entry's fourth word.
/// The frame words the exe leaves unwritten are zero here. Ends with `00d04e80` and returns what it
/// returns.
pub fn hk_cpu_world_get_closest_points_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_TT_GET_CLOSEST_POINTS);
    let job = job.addr();
    let group = e.mem.u32(job + 0x18);
    let float_max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
    let epsilon = x87_quiet_bits(e.mem.u32(FLOAT_EPSILON));
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        e.mem.set_u32(f + COLLECTOR, COLLECTOR_VTABLE);
        e.mem.set_u32(f + COLLECTOR + 4, float_max);
        e.mem.set_u32(f + 0xc0, epsilon);
        e.mem.set_u32(f + COLLECTOR + 0x10, 1);
        for (index, vtable) in LISTENER_VTABLES.iter().enumerate() {
            e.mem.set_u32(f + LISTENERS + 4 * index as u32, *vtable);
        }
        let shared = e.mem.u32(group + 0xc);
        e.mem.set_u32(f + INPUT, shared);
        e.mem.set_u32(f + INPUT + 4, f + COLLECTOR);
        for word in 0..6 {
            let value = e.mem.u32(group + 4 * word);
            e.mem.set_u32(f + 0x60 + 4 * word, value);
        }
        for word in 0..12 {
            let value = e.mem.u32(group + 0x20 + 4 * word);
            e.mem.set_u32(f + 0x80 + 4 * word, value);
        }
        let last = e.mem.u32(group + 0x70);
        e.mem.set_u32(f + 0xc8, last);
        let listener = e.mem.u32(job + 0x20);
        let first_callback = e.mem.u32(job + 0x24);
        let second_callback = e.mem.u32(job + 0x28);
        e.mem.set_u32(f + 0x34, listener);
        e.mem.set_u32(f + 0x30, first_callback);
        e.mem.set_u32(f + 0x3c, second_callback);
        let mut index = 0;
        while index < e.mem.i32(job + 0x38) {
            let entry = e.mem.u32(job + 0x34).wrapping_add(16 * index as u32);
            let (second_word, third_word) = (e.mem.u32(entry + 4), e.mem.u32(entry + 8));
            e.mem.set_u32(f + COLLECTOR + 4, float_max);
            e.mem.set_u32(f + COLLECTOR + 0x10, third_word);
            e.mem.set_u32(f + COLLECTOR, COLLECTOR_VTABLE);
            e.mem.set_u32(f + COLLECTOR + 8, second_word);
            e.mem.set_u32(f + COLLECTOR + 0xc, second_word);
            e.mem.set_u32(f + COLLECTOR + 0x14, 0);
            let target = e.mem.u32(entry);
            let object = e.mem.u32(target);
            let width = e.mem.f32(group + 8);
            let distance = (f64::from(e.mem.f32(job + 0x2c))
                - f64::from(width) * e.global::<f64>(HALF)) as f32;
            let argument = e.mem.u32(target + 8);
            e.vcall(object, 0x1c, &args![argument, distance, f + POINTS]);
            let copied = x87_quiet_bits(e.mem.u32(job + 0x2c));
            e.mem.set_u32(f + 0x68, copied);
            let target = e.mem.u32(entry);
            e.mem.set_u32(f + 0x54, target);
            let object_word = e.mem.u32(object + 0xc);
            e.mem.set_u32(f + 0x58, object_word);
            let listener = e.mem.u32(f + 0x34);
            if listener != 0 {
                e.vcall(listener, 0x74, &args![f + POINTS, f + LISTENERS]);
            }
            let object = e.mem.u32(f + 0x30);
            if object != 0 {
                e.call(
                    COLLECT_WITH_LISTENER_A,
                    &args![object, f + POINTS, f + LISTENERS + 4],
                );
            }
            let object = e.mem.u32(f + 0x3c);
            if object != 0 {
                e.call(
                    COLLECT_WITH_LISTENER_B,
                    &args![object, f + POINTS, f + LISTENERS + 8],
                );
            }
            let hit = e.mem.u32(f + COLLECTOR + 0x14);
            e.mem.set_u32(entry + 0xc, hit);
            index += 1;
        }
    });
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00df6b50,
        hk_cpu_world_get_closest_points_job(u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, logging_object, Log,
    };
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    const OBJECT: u32 = 0x0900_0000;
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

    /// The offsets in the frame whose words the doubles log.
    const WATCHED: [u32; 23] = [
        0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x60, 0x64,
        0x68, 0x6c, 0x70, 0x74, 0x80, 0xac, 0xc0, 0xc8,
    ];

    /// A double at `address` that logs its arguments and then the watched words of the frame
    /// whose points buffer is the argument `points`; with `hit` it also sets the collector's hit
    /// word to `hit` plus the number of calls so far.
    fn frame_double(e: &mut Engine, log: &Log, address: u32, points: usize, hit: Option<u32>) {
        let sink = log.clone();
        e.register_double(address, move |e, a| {
            let frame = a[points] - POINTS;
            let mut words = a.to_vec();
            words.extend(WATCHED.iter().map(|offset| e.mem.u32(frame + offset)));
            let before = sink
                .borrow()
                .iter()
                .filter(|call| call.0 == address)
                .count() as u32;
            sink.borrow_mut().push((address, words));
            if let Some(hit) = hit {
                e.mem.set_u32(frame + COLLECTOR + 0x14, hit + before);
            }
            Ret::default()
        });
    }

    fn rig() -> (Engine, JobRig) {
        let (mut e, rig) = job_rig();
        e.mem.set_u32(FLOAT_MAX, 0x7f7f_ffee);
        e.mem.set_u32(FLOAT_EPSILON, 0x3400_0000);
        e.mem.set_f64(HALF, 0.5);
        (e, rig)
    }

    /// A group with the words the job copies: word 2 is the float 2.0.
    fn group(e: &mut Engine) -> u32 {
        let group = e.mem.alloc(0x80);
        for word in 0..32 {
            e.mem.set_u32(group + 4 * word, 0x100 + word);
        }
        e.mem.set_f32(group + 8, 2.0);
        e.mem.set_u32(group + 0x70, 0x777);
        group
    }

    /// An entry whose first word is a target with the query object `object`, a word `0xc0c`
    /// at `+0xC` in the object and `0xaa` at `+8` in the target.
    fn entry(e: &mut Engine, at: u32, object: u32, words: (u32, u32)) -> u32 {
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target, object);
        e.mem.set_u32(target + 8, 0xaa);
        e.mem.set_u32(object + 0xc, 0xc0c);
        e.mem.set_u32(at, target);
        e.mem.set_u32(at + 4, words.0);
        e.mem.set_u32(at + 8, words.1);
        e.mem.set_u32(at + 0xc, 0xdead);
        target
    }

    #[test]
    fn queries_each_entry_and_stores_the_hit_word() {
        let (mut e, rig) = rig();
        let query_object = logging_object(&mut e, &rig.log, OBJECT);
        let listener = object_with_slot(&mut e, LISTENER, 0x74);
        frame_double(&mut e, &rig.log, OBJECT + 0x1c, 3, None);
        frame_double(&mut e, &rig.log, LISTENER + 0x74, 1, None);
        frame_double(&mut e, &rig.log, COLLECT_WITH_LISTENER_A, 1, Some(0x9000));
        frame_double(&mut e, &rig.log, COLLECT_WITH_LISTENER_B, 1, None);
        let group = group(&mut e);
        let entries = e.mem.alloc(0x20);
        let first = entry(&mut e, entries, query_object, (0x31, 0x32));
        entry(&mut e, entries + 0x10, query_object, (0x41, 0x42));
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, group);
        e.mem.set_u32(job + 0x20, listener);
        e.mem.set_u32(job + 0x24, 0x2400);
        e.mem.set_u32(job + 0x28, 0x2800);
        e.mem.set_f32(job + 0x2c, 5.0);
        e.mem.set_u32(job + 0x34, entries);
        e.mem.set_u32(job + 0x38, 2);
        let returned = e.call(0x00df_6b50, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        // The query gets the target's word, 5.0 - 2.0 * 0.5 and the points buffer.
        let queries = calls(&rig.log, OBJECT + 0x1c);
        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0][..3], [query_object, 0xaa, 4.0f32.to_bits()]);
        let frame = queries[0][3] - POINTS;
        // The collector is reset for each entry, with the entry's second and third words.
        assert_eq!(
            queries[0][4..],
            [
                COLLECTOR_VTABLE,
                0x7f7f_ffee,
                0x31,
                0x31,
                0x32,
                0,
                0x010d_b94c,
                0x010d_b940,
                0x010d_b92c,
                0x103,
                frame + 0x18,
                0,
                0,
                0x100,
                0x101,
                2.0f32.to_bits(),
                0x103,
                0x104,
                0x105,
                0x108,
                0x113,
                0x3400_0000,
                0x777
            ]
        );
        assert_eq!(queries[1][4 + 2], 0x41);
        assert_eq!(queries[1][4 + 4], 0x42);
        // The callbacks run after the query, with the entry and the object's word filled in.
        let listeners = calls(&rig.log, LISTENER + 0x74);
        assert_eq!(listeners.len(), 2);
        assert_eq!(
            listeners[0][..3],
            [listener, frame + POINTS, frame + LISTENERS]
        );
        assert_eq!(listeners[0][3 + 10], frame + 0x18);
        assert_eq!(listeners[0][3 + 11], first);
        assert_eq!(listeners[0][3 + 12], 0xc0c);
        assert_eq!(listeners[0][3 + 15], 5.0f32.to_bits());
        let collected = calls(&rig.log, COLLECT_WITH_LISTENER_A);
        assert_eq!(collected.len(), 2);
        assert_eq!(
            collected[0][..3],
            [0x2400, frame + POINTS, frame + LISTENERS + 4]
        );
        let collected = calls(&rig.log, COLLECT_WITH_LISTENER_B);
        assert_eq!(collected.len(), 2);
        assert_eq!(
            collected[0][..3],
            [0x2800, frame + POINTS, frame + LISTENERS + 8]
        );
        // The word the first callback left in the collector is stored in each entry.
        assert_eq!(e.mem.u32(entries + 0xc), 0x9000);
        assert_eq!(e.mem.u32(entries + 0x1c), 0x9001);
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_TT_GET_CLOSEST_POINTS, PROFILE_END]
        );
    }

    #[test]
    fn without_callbacks_the_hit_word_is_zero() {
        let (mut e, rig) = rig();
        let query_object = logging_object(&mut e, &rig.log, OBJECT);
        log_double(&mut e, &rig.log, COLLECT_WITH_LISTENER_A, 0);
        let group = group(&mut e);
        let entries = e.mem.alloc(0x10);
        entry(&mut e, entries, query_object, (0x31, 0x32));
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, group);
        e.mem.set_u32(job + 0x34, entries);
        e.mem.set_u32(job + 0x38, 1);
        let returned = e.call(0x00df_6b50, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(calls(&rig.log, OBJECT + 0x1c).len(), 1);
        assert!(calls(&rig.log, COLLECT_WITH_LISTENER_A).is_empty());
        assert_eq!(e.mem.u32(entries + 0xc), 0);
    }

    #[test]
    fn with_no_entries_it_only_finishes() {
        let (mut e, rig) = rig();
        let group = group(&mut e);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, group);
        let returned = e.call(0x00df_6b50, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
    }
}
