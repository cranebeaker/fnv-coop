//! `Havok SDK/hkpcpushaperaycastjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code computes in `f32`; floats the code moves through `FLD` / `FSTP` go through
//! `x87_quiet_bits`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpcpuworldraycastjob::{
    into_transform_frame, ADD_RAY_HIT_HOOK, ADD_RAY_HIT_TARGET, COLLECTOR_VTABLE, ONE,
};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, profile_record, store_vec, PROFILE_END};

/// Profiling label `"TtCpuShapeRayCast"`.
const LABEL_SHAPE_RAY_CAST: u32 = 0x010d_bb24;

// Translated from 00df94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuShapeRayCastJob` (Xbox PDB name per the engine map): cdecl, two stack words: a word that
/// becomes `ECX` of the final call `00d04e80`, and the job. Writes `TtCpuShapeRayCast`.
///
/// For each of the `[job + 0x28]` (signed) ray requests of 0x50 bytes at `[job + 0x24]`:
/// the request has the two end points at `+0` and `+0x10`, the list of hits (0x60-byte
/// entries) at `+0x40` with its capacity at `+0x44` and its count at `+0x48`, the number of
/// shapes at `+0x3C`, the pointer to the shape entries at `+0x38` and the collector flag at
/// `+0x4C`. It sets the hook `0126ec04` to `00e2b9b0`, makes an output block (1.0 at `+0x10`,
/// `0xFFFFFFFF` at `+0x14` and `+0x20`, 0 at `+0x40` and `+0x50`) and a collector (vtable
/// `010dbb04`, 1.0 at `+4`, the list in `+8` and `+0xC`, the capacity in `+0x10`, 0 at `+0x14`
/// and `+0x18`) on its stack and, for each shape entry whose object `[entry]` is not null, puts
/// the end points into the frame of the transform `[entry + 8]` (`into_transform_frame`, the
/// copy of the request's 48 bytes supplies the rest of the local ray). With the collector flag
/// clear it runs slot `0x20` of the object (a byte cell, the local ray, the output block) and, on a
/// hit, stores the output block (the normal rotated back into the world frame, the fraction
/// through `FLD` / `FSTP`, the words up to `+0x40`) in the next free hit entry
/// (`count < capacity`, the count goes up) or in the entry with the greatest fraction when that is
/// greater than the new one, and puts the shape entry pointer at `+0x50`. With the flag set it runs
/// slot `0x24` (the local ray, the shape entry, the collector) and afterwards copies the
/// collector's count (`+0x14`) to the request's count word. Ends with the end record and
/// `00d04e80` and returns what that returns. Words of the output block the object does not
/// write (and the first cell byte) are stack residue in the exe; here they start as zero and
/// keep their values from one shape entry to the next, as the exe's stack slots do.
pub fn hk_cpu_shape_ray_cast_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_SHAPE_RAY_CAST);
    let job = job.addr();
    e.with_stack(0xe0, |e, frame| {
        let b = frame.addr();
        let mut index = 0;
        let mut offset = 0u32;
        if e.mem.i32(job + 0x28) > 0 {
            loop {
                let request = e.mem.u32(job + 0x24).wrapping_add(offset);
                cast_request(e, request, b);
                offset = offset.wrapping_add(0x50);
                index += 1;
                if index >= e.mem.i32(job + 0x28) {
                    break;
                }
            }
        }
    });
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// One ray request of `hk_cpu_shape_ray_cast_job` on the frame `b`.
fn cast_request(e: &mut Engine, request: u32, b: u32) {
    e.mem.set_u32(b + 0x50, ONE);
    e.mem.set_u32(b + 0x28, ONE);
    e.mem.set_u32(b + 0x54, 0xffff_ffff);
    e.mem.set_u32(b + 0x60, 0xffff_ffff);
    let list = e.mem.u32(request + 0x40);
    e.mem.set_u32(b + 0x1c, list);
    e.mem.set_u32(b + 0x2c, list);
    e.mem.set_u32(b + 0x30, list);
    let capacity_word = e.mem.u32(request + 0x44);
    for word in 0..12 {
        let value = e.mem.u32(request + 4 * word);
        e.mem.set_u32(b + 0xa0 + 4 * word, value);
    }
    e.mem.set_u32(b + 0x34, capacity_word);
    e.set_global(ADD_RAY_HIT_HOOK, ADD_RAY_HIT_TARGET);
    e.mem.set_u32(b + 0x80, 0);
    e.mem.set_u32(b + 0x90, 0);
    e.mem.set_u32(b + 0x24, COLLECTOR_VTABLE);
    e.mem.set_u32(b + 0x38, 0);
    e.mem.set_u32(b + 0x3c, 0);
    if e.mem.i32(request + 0x3c) > 0 {
        let mut shape = 0;
        loop {
            let entries = e.mem.u32(request + 0x38);
            let entry = e.mem.u32(entries + 4 * shape);
            e.mem.set_u32(b + 0x20, entry);
            if e.mem.u32(entry) != 0 {
                cast_shape(e, request, entry, b);
            }
            shape += 1;
            if shape as i32 >= e.mem.i32(request + 0x3c) {
                break;
            }
        }
    }
    if e.mem.u8(request + 0x4c) != 0 {
        let count = e.mem.u32(b + 0x38);
        e.mem.set_u32(request + 0x48, count);
    }
}

/// One shape entry of a request: `entry` has a non-null object at `+0`.
fn cast_shape(e: &mut Engine, request: u32, entry: u32, b: u32) {
    let object = e.mem.u32(entry);
    let matrix = e.mem.u32(entry + 8);
    let end = into_transform_frame(e, matrix, load_vec(e, request + 0x10));
    store_vec(e, b + 0xb0, end);
    e.mem.set_u32(b + 0x50, ONE);
    let start = into_transform_frame(e, matrix, load_vec(e, request));
    store_vec(e, b + 0xa0, start);
    e.mem.set_u32(b + 0x90, 0);
    e.mem.set_u32(b + 0x54, 0xffff_ffff);
    e.mem.set_u32(b + 0x80, 0);
    e.mem.set_u32(b + 0x60, 0xffff_ffff);
    if e.mem.u8(request + 0x4c) != 0 {
        e.vcall(object, 0x24, &args![b + 0xa0, entry, b + 0x24]);
        return;
    }
    e.vcall(object, 0x20, &args![b + 0xf, b + 0xa0, b + 0x40]);
    if e.mem.u8(b + 0xf) == 0 {
        return;
    }
    let count = e.mem.i32(request + 0x48);
    let capacity = e.mem.i32(request + 0x44);
    let slot = if count < capacity {
        let slot = e.mem.u32(b + 0x1c);
        e.mem.set_i32(request + 0x48, count.wrapping_add(1));
        e.mem.set_u32(b + 0x1c, slot.wrapping_add(0x60));
        slot
    } else {
        let mut greatest = e.mem.u32(request + 0x40);
        let mut next = greatest.wrapping_add(0x60);
        for _ in 1..count {
            if e.mem.f32(greatest + 0x10) < e.mem.f32(next + 0x10) {
                greatest = next;
            }
            next = next.wrapping_add(0x60);
        }
        if e.mem.f32(greatest + 0x10) > e.mem.f32(b + 0x50) {
            greatest
        } else {
            return;
        }
    };
    if slot == 0 {
        return;
    }
    let normal = load_vec(e, b + 0x40);
    let rows = [
        load_vec(e, matrix),
        load_vec(e, matrix + 0x10),
        load_vec(e, matrix + 0x20),
    ];
    let rotated: [f32; 4] = std::array::from_fn(|lane| {
        (normal[0] * rows[0][lane] + normal[1] * rows[1][lane]) + normal[2] * rows[2][lane]
    });
    store_vec(e, b + 0x40, rotated);
    store_vec(e, slot, rotated);
    let fraction = x87_quiet_bits(e.mem.u32(b + 0x50));
    e.mem.set_u32(slot + 0x10, fraction);
    for word in (0x14..=0x40).step_by(4) {
        let value = e.mem.u32(b + 0x40 + word);
        e.mem.set_u32(slot + word, value);
    }
    e.mem.set_u32(slot + 0x50, entry);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00df94b0,
        hk_cpu_shape_ray_cast_job(u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};
    use crate::units::havok_sdk::hkpcpuworldraycastjob::test_support::{
        get_floats, object_with_slots, set_floats,
    };

    const CAST_SLOT: u32 = 0x0900_1020;
    const COLLECT_SLOT: u32 = 0x0900_1024;

    /// A job with one request: two shape entries (the first with a null object) over a
    /// transform that scales by 2, 3 and 4 and moves by 1, a list of three hit entries with
    /// capacity 2 and count 0, and the ray from `(3, 5, 7)` to `(5, 9, 13)` with the extra
    /// vector `(9, 8, 7, 6)` at `+0x20`.
    struct Shape {
        e: Engine,
        rig: JobRig,
        job: u32,
        request: u32,
        list: u32,
        entry: u32,
        object: u32,
    }

    fn shape_rig(collector_flag: u8) -> Shape {
        let (mut e, rig) = job_rig();
        let object = object_with_slots(&mut e, &[(0x20, CAST_SLOT), (0x24, COLLECT_SLOT)]);
        let matrix = e.mem.alloc(0x40);
        set_floats(&mut e, matrix, [2.0, 0.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x10, [0.0, 3.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x20, [0.0, 0.0, 4.0, 0.0]);
        set_floats(&mut e, matrix + 0x30, [1.0, 1.0, 1.0, 0.0]);
        let entries = e.mem.alloc(0x40);
        let skipped = e.mem.alloc(0x10);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, object);
        e.mem.set_u32(entry + 8, matrix);
        e.mem.set_u32(entries, skipped);
        e.mem.set_u32(entries + 4, entry);
        let list = e.mem.alloc(0x180);
        let request = e.mem.alloc(0x50);
        set_floats(&mut e, request, [3.0, 5.0, 7.0, 0.0]);
        set_floats(&mut e, request + 0x10, [5.0, 9.0, 13.0, 0.0]);
        set_floats(&mut e, request + 0x20, [9.0, 8.0, 7.0, 6.0]);
        e.mem.set_u32(request + 0x38, entries);
        e.mem.set_u32(request + 0x3c, 2);
        e.mem.set_u32(request + 0x40, list);
        e.mem.set_u32(request + 0x44, 2);
        e.mem.set_u8(request + 0x4c, collector_flag);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x24, request);
        e.mem.set_u32(job + 0x28, 1);
        Shape {
            e,
            rig,
            job,
            request,
            list,
            entry,
            object,
        }
    }

    /// Makes the cast of the object (slot `0x20`) log its words and the output block and answer
    /// `found` with the normal `(1, 2, 3)`, the fraction `fraction` and `0x1234` at `+0x14`.
    fn answer_cast(s: &mut Shape, found: u8, fraction: f32) {
        let sink = s.rig.log.clone();
        s.e.register_double(CAST_SLOT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..12).map(|word| e.mem.u32(a[2] + 4 * word)));
            words.extend([0x10, 0x14, 0x20, 0x40, 0x50].map(|offset| e.mem.u32(a[3] + offset)));
            sink.borrow_mut().push((CAST_SLOT, words));
            e.mem.set_u8(a[1], found);
            set_floats(e, a[3], [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(a[3] + 0x10, fraction);
            e.mem.set_u32(a[3] + 0x14, 0x1234);
            Ret::default()
        });
    }

    #[test]
    fn no_requests_only_write_the_records_and_finish() {
        let (mut e, rig) = job_rig();
        let job = e.mem.alloc(0x40);
        let returned = e.call(0x00df_94b0, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_SHAPE_RAY_CAST, PROFILE_END]
        );
    }

    #[test]
    fn a_hit_goes_into_the_next_free_entry_with_the_local_ray_and_rotated_normal() {
        let mut s = shape_rig(0);
        answer_cast(&mut s, 1, 0.5);
        let returned = s.e.call(0x00df_94b0, &args![0x99u32, s.job]).u32();
        s.rig.assert_finished(returned, 0x99, s.job);
        let casts = calls(&s.rig.log, CAST_SLOT);
        assert_eq!(casts.len(), 1);
        let cast = &casts[0];
        assert_eq!(cast[0], s.object);
        // The local ray: the start (4, 12, 24, 0), the end (8, 24, 48, 0) and the request's
        // third vector; the output block starts as the exe leaves it.
        let words: Vec<f32> = cast[4..16].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(
            words,
            vec![4.0, 12.0, 24.0, 0.0, 8.0, 24.0, 48.0, 0.0, 9.0, 8.0, 7.0, 6.0]
        );
        assert_eq!(cast[16..], [ONE, 0xffff_ffff, 0xffff_ffff, 0, 0]);
        let e = &s.e;
        assert_eq!(e.mem.u32(s.request + 0x48), 1);
        // The normal (1, 2, 3) through the rows: (2, 0, 0) + 2 * (0, 3, 0) + 3 * (0, 0, 4).
        assert_eq!(get_floats(e, s.list), [2.0, 6.0, 12.0, 0.0]);
        assert_eq!(e.mem.f32(s.list + 0x10), 0.5);
        assert_eq!(e.mem.u32(s.list + 0x14), 0x1234);
        assert_eq!(e.mem.u32(s.list + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(s.list + 0x50), s.entry);
        assert_eq!(e.global::<u32>(ADD_RAY_HIT_HOOK), ADD_RAY_HIT_TARGET);
        assert_eq!(
            s.rig.labels(&s.e, false),
            vec![LABEL_SHAPE_RAY_CAST, PROFILE_END]
        );
    }

    #[test]
    fn a_full_list_gives_way_only_when_its_greatest_fraction_is_greater() {
        let mut s = shape_rig(0);
        s.e.mem.set_u32(s.request + 0x48, 2);
        s.e.mem.set_f32(s.list + 0x10, 0.25);
        s.e.mem.set_f32(s.list + 0x60 + 0x10, 0.75);
        answer_cast(&mut s, 1, 0.5);
        s.e.call(0x00df_94b0, &args![0x99u32, s.job]);
        assert_eq!(s.e.mem.f32(s.list + 0x10), 0.25);
        assert_eq!(s.e.mem.f32(s.list + 0x60 + 0x10), 0.5);
        assert_eq!(s.e.mem.u32(s.list + 0x60 + 0x50), s.entry);
        assert_eq!(s.e.mem.u32(s.request + 0x48), 2);
        // A new fraction that is not smaller than the greatest leaves the list alone.
        s.e.mem.set_f32(s.list + 0x60 + 0x10, 0.75);
        s.e.mem.set_u32(s.list + 0x60 + 0x50, 0);
        answer_cast(&mut s, 1, 0.75);
        s.e.call(0x00df_94b0, &args![0x99u32, s.job]);
        assert_eq!(s.e.mem.f32(s.list + 0x60 + 0x10), 0.75);
        assert_eq!(s.e.mem.u32(s.list + 0x60 + 0x50), 0);
    }

    #[test]
    fn a_miss_stores_nothing() {
        let mut s = shape_rig(0);
        answer_cast(&mut s, 0, 0.5);
        s.e.call(0x00df_94b0, &args![0x99u32, s.job]);
        assert_eq!(s.e.mem.u32(s.request + 0x48), 0);
        assert_eq!(s.e.mem.u32(s.list + 0x50), 0);
    }

    #[test]
    fn with_the_collector_flag_the_collector_gets_the_local_ray_and_its_count_is_taken() {
        let mut s = shape_rig(1);
        let sink = s.rig.log.clone();
        s.e.register_double(COLLECT_SLOT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..7).map(|word| e.mem.u32(a[3] + 4 * word)));
            sink.borrow_mut().push((COLLECT_SLOT, words));
            e.mem.set_u32(a[3] + 0x14, 3);
            Ret::default()
        });
        log_double(&mut s.e, &s.rig.log.clone(), CAST_SLOT, 0);
        s.e.call(0x00df_94b0, &args![0x99u32, s.job]);
        let collected = calls(&s.rig.log, COLLECT_SLOT);
        assert_eq!(collected.len(), 1);
        let call = &collected[0];
        assert_eq!(call[0], s.object);
        assert_eq!(call[2], s.entry);
        assert_eq!(call[4..], [COLLECTOR_VTABLE, ONE, s.list, s.list, 2, 0, 0]);
        assert_eq!(s.e.mem.u32(s.request + 0x48), 3);
        assert!(calls(&s.rig.log, CAST_SLOT).is_empty());
    }
}
