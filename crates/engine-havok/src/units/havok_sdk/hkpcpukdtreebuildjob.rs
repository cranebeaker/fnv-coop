//! `Havok SDK/hkpcpukdtreebuildjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{copy_vec, profile_record, PROFILE_END};

/// `hkKdTreeBuildingUtils::buildTree` (Ghidra name; cdecl, seven words: the input block, three
/// words of the job, the count, a further job word and the address that receives one result
/// word).
const BUILD_TREE: u32 = 0x00d9_d880;
/// `hkKdTreeBuilder<...>::splitTreeBuilding` (Ghidra name; cdecl, seven words: the input block,
/// two job words, the count, the four output structures, a job word and the bounds block).
const SPLIT_TREE_BUILDING: u32 = 0x00df_37b0;
/// `hkKdTreeBuilderSparseWriter` constructor (Ghidra name; `ECX` = the writer, no stack words).
const SPARSE_WRITER_CONSTRUCT: u32 = 0x00df_3110;
/// `hkKdTreeBuilderSparseWriter::flush` (Ghidra name; `ECX` = the writer, no stack words).
const SPARSE_WRITER_FLUSH: u32 = 0x00df_3160;
/// `hkKdTreeBuilder<...>::buildTreeRecursive` (Ghidra name; cdecl, five words: the builder, the
/// input block, a job word and the addresses of two words; returns a count).
const BUILD_TREE_RECURSIVE: u32 = 0x00df_56a0;
/// `hkKdTreeRayCastJobUtil::initializeBuildSetupJobs` (Ghidra name; cdecl, six words: three job
/// words, a job word, the number of jobs and the first of the setup jobs).
const INITIALIZE_BUILD_SETUP_JOBS: u32 = 0x00e2_aa70;
/// `hkJobQueue::lockQueue` (Ghidra name; `ECX` = the queue, one stack word: a 380-byte scratch
/// area; returns the lock token).
pub(crate) const LOCK_QUEUE: u32 = 0x00d0_4400;
/// `hkJobQueue::addJobQueueLocked` (Ghidra name; `ECX` = the queue, three stack words: the
/// token, the job and 0).
pub(crate) const ADD_JOB_QUEUE_LOCKED: u32 = 0x00d0_4b00;
/// `hkJobQueue::unlockQueue` (Ghidra name; `ECX` = the queue, one stack word: the token).
pub(crate) const UNLOCK_QUEUE: u32 = 0x00d0_4420;
/// Four floats kept in the exe's data at `0x012684d0`, copied into the builders' structures.
const BUILDER_VECTOR: u32 = 0x0126_84d0;

/// Profiling label `"TtCpuKdTreeBuildFastDistributed"`.
const LABEL_FAST_DISTRIBUTED: u32 = 0x010d_ba54;
/// Profiling label `"TtCpuKdTreeBuildFastDistributedSub"`.
const LABEL_FAST_DISTRIBUTED_SUB: u32 = 0x010d_ba74;
/// Profiling label `"TtCpuKdTreeBuildCoord"`.
const LABEL_COORDINATOR: u32 = 0x010d_ba98;

/// Writes the first eight bytes of a job of `size` bytes with type byte `kind` at `job`: the
/// bytes `kind`, 3 and 1 at `+0..+2`, and the 16-bit words `size` and `0xFFFF` at `+4` and `+6`.
fn write_job_header(e: &mut Engine, job: u32, kind: u8, size: u16) {
    e.mem.set_u8(job, kind);
    e.mem.set_u8(job + 1, 3);
    e.mem.set_u8(job + 2, 1);
    e.mem.set_u16(job + 4, size);
    e.mem.set_u16(job + 6, 0xffff);
}

// Translated from 00df7e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuKdTreeBuildFastDistributedJob` (Xbox PDB name): cdecl, two stack words: a word that
/// becomes `ECX` of the final call `00d04e80` (it is also the job queue the sub-jobs go to), and
/// the job. Writes `TtCpuKdTreeBuildFastDistributed` and puts the words at `job + 0x20` and
/// `+0x24` in an input block on its stack. With fewer than 8 (signed) at `+0x38` it calls
/// `00d9d880` (the block, the words at `+0x28`, `+0x2C` and `+0x3C`, the count, the word at
/// `+0x40` and the address of a result word) and copies the result word into the four words at
/// `[job + 0x30]` unless that is null. With 8 or more it prepares four 0x50-byte output
/// structures (a pointer to a pair of result words, `0xFFFFFFFF` at `+8` and the four floats of
/// `0x012684d0` at `+0x40`) and a bounds block (16-bit words, three 0 then three `0xFFFF`), runs
/// `00df37b0` and builds four 0x60-byte jobs of type 4 on its stack. Job `k` gets the words at
/// `+0x10` and `+0x14` of the job, 0 at `+0x18`, the word at `+0x2C` at `+0x24`, words from its
/// output structure (the second word at `+4`, the pair at `+0x2C` and `+0x30`, the words at
/// `+0xC` and `+0x10`, the vector at `+0x20` copied to `+0x40`, the word at `+8` to `+0x54`),
/// the result pointer `[job + 0x30] + 4k` (0 when that is null) at `+0x50`, and a share at
/// `+0x58`: `n = [job + 0x40] / 4` (rounded toward zero) for the first three and
/// `[job + 0x40] - 3n` for the last. The entry pointer at `+0x20` is `[job + 0x28]` plus 8 bytes
/// times (8 plus the already advanced counters of the earlier jobs); the counters at `[job + 0x34]` each go up by
/// `n + (n & 1)` (before the next job reads them). It sets the word `[job + 0x14]` points to to
/// 4, then locks the queue (`00d04400`), adds the four jobs (`00d04b00`) and unlocks
/// (`00d04420`). Ends with `00d04e80` and returns what it returns. Frame words the exe leaves
/// unwritten are zero here.
pub fn hk_cpu_kd_tree_build_fast_distributed_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_FAST_DISTRIBUTED);
    let job = job.addr();
    e.with_stack(0x500, |e, frame| {
        let f = frame.addr();
        let input = f + 0x18;
        let result = f + 0x14;
        let first = e.mem.u32(job + 0x20);
        let next = e.mem.u32(job + 0x24);
        e.mem.set_u32(input, first);
        e.mem.set_u32(input + 4, next);
        let count = e.mem.i32(job + 0x38);
        if count < 8 {
            let args = args![
                input,
                e.mem.u32(job + 0x28),
                e.mem.u32(job + 0x2c),
                e.mem.u32(job + 0x3c),
                count,
                e.mem.u32(job + 0x40),
                result
            ];
            e.call(BUILD_TREE, &args);
            let results = e.mem.u32(job + 0x30);
            if results != 0 {
                for word in 0..4 {
                    let value = e.mem.u32(result);
                    e.mem.set_u32(results + 4 * word, value);
                }
            }
        } else {
            let bounds = f + 0x20;
            for offset in [0, 2, 4] {
                e.mem.set_u16(bounds + offset, 0);
                e.mem.set_u16(bounds + 8 + offset, 0xffff);
            }
            let outputs = f + 0x1d0;
            for k in 0..4 {
                let output = outputs + 0x50 * k;
                e.mem.set_u32(output, f + 0x1b0 + 8 * k);
                e.mem.set_u32(output + 8, 0xffff_ffff);
                copy_vec(e, BUILDER_VECTOR, output + 0x40);
            }
            let args = args![
                input,
                e.mem.u32(job + 0x28),
                e.mem.u32(job + 0x3c),
                count,
                outputs,
                e.mem.u32(job + 0x34),
                bounds
            ];
            e.call(SPLIT_TREE_BUILDING, &args);
            let total = e.mem.i32(job + 0x40);
            let share = total.wrapping_add((total >> 31) & 3) >> 2;
            let step = (share & 1).wrapping_add(share);
            let entries = e.mem.u32(job + 0x28);
            let counters = e.mem.u32(job + 0x34);
            let results = e.mem.u32(job + 0x30);
            let mut skipped = 8u32;
            for k in 0..4 {
                let block = f + 0x30 + 0x60 * k;
                let output = outputs + 0x50 * k;
                let pair = e.mem.u32(output);
                write_job_header(e, block, 4, 0x60);
                e.mem.set_u32(block + 0x18, 0);
                for offset in [0x10, 0x14] {
                    let value = e.mem.u32(job + offset);
                    e.mem.set_u32(block + offset, value);
                }
                e.mem
                    .set_u32(block + 0x20, entries.wrapping_add(skipped.wrapping_mul(8)));
                let value = e.mem.u32(job + 0x2c);
                e.mem.set_u32(block + 0x24, value);
                for (to, from) in [(0x28, 4), (0x34, 0xc), (0x38, 0x10), (0x54, 8)] {
                    let value = e.mem.u32(output + from);
                    e.mem.set_u32(block + to, value);
                }
                for (to, from) in [(0x2c, 0), (0x30, 4)] {
                    let value = e.mem.u32(pair + from);
                    e.mem.set_u32(block + to, value);
                }
                copy_vec(e, output + 0x20, block + 0x40);
                let slot = if results == 0 { 0 } else { results + 4 * k };
                e.mem.set_u32(block + 0x50, slot);
                let size = if k == 3 {
                    total.wrapping_sub(share.wrapping_mul(3))
                } else {
                    share
                };
                e.mem.set_i32(block + 0x58, size);
                let counter = counters + 4 * k;
                let advanced = e.mem.i32(counter).wrapping_add(step);
                e.mem.set_i32(counter, advanced);
                skipped = skipped.wrapping_add(advanced as u32);
            }
            let flag = e.mem.u32(job + 0x14);
            e.mem.set_u32(flag, 4);
            let token = e.call(LOCK_QUEUE, &args![second, f + 0x310]).u32();
            for k in 0..4 {
                e.call(
                    ADD_JOB_QUEUE_LOCKED,
                    &args![second, token, f + 0x30 + 0x60 * k, 0u32],
                );
            }
            e.call(UNLOCK_QUEUE, &args![second, token]);
        }
    });
    let done = e.call(JOB_DONE, &args![second, job, job, 0u32]).u32();
    profile_record(e, PROFILE_END);
    done
}

// Translated from 00df8440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuKdTreeBuildFastDistributedSubJob` (Xbox PDB name): cdecl, two stack words: a word that
/// becomes `ECX` of the final call `00d04e80`, and the job. Writes
/// `TtCpuKdTreeBuildFastDistributedSub` and builds the tree builder on its stack: a pointer to
/// the pair `{[job + 0x2C], [job + 0x30]}`, the words at `+0x28`, `+0x54`, `+0x34` and `+0x38`,
/// the 16 bytes at `+0x40`, the four floats of `0x012684d0` and a sparse writer (constructed by
/// `00df3110`, `ECX` = the writer, which the builder points to). It calls `00df56a0` with the
/// builder, the input block `{[job + 0x20]}`, the word at `+0x24`, the address of the word 1 and
/// the address of the word at `[job + 0x58]`, and stores the count it returns plus 2 in the word
/// `[job + 0x50]` points to. Then it runs `00d04e80`, flushes the writer (`00df3160`), writes the
/// end record and returns what `00d04e80` returned. Frame words the exe leaves unwritten are zero
/// here.
pub fn hk_cpu_kd_tree_build_fast_distributed_sub_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_FAST_DISTRIBUTED_SUB);
    let job = job.addr();
    let done = e.with_stack(0x1c0, |e, frame| {
        let f = frame.addr();
        for (to, from) in [
            (0x28, 0x2c),
            (0x2c, 0x30),
            (0x34, 0x28),
            (0x38, 0x54),
            (0x3c, 0x34),
            (0x40, 0x38),
        ] {
            let value = e.mem.u32(job + from);
            e.mem.set_u32(f + to, value);
        }
        e.mem.set_u32(f + 0x30, f + 0x28);
        copy_vec(e, job + 0x40, f + 0x50);
        copy_vec(e, BUILDER_VECTOR, f + 0x70);
        e.call(SPARSE_WRITER_CONSTRUCT, &args![f + 0x80]);
        let input = e.mem.u32(job + 0x20);
        e.mem.set_u32(f + 0x1c, input);
        e.mem.set_u32(f + 0x20, 1);
        let size = e.mem.u32(job + 0x58);
        e.mem.set_u32(f + 0x24, size);
        e.mem.set_u32(f + 0x60, f + 0x80);
        let args = args![
            f + 0x30,
            f + 0x1c,
            e.mem.u32(job + 0x24),
            f + 0x20,
            f + 0x24
        ];
        let built = e.call(BUILD_TREE_RECURSIVE, &args).u32();
        let out = e.mem.u32(job + 0x50);
        e.mem.set_u32(out, built.wrapping_add(2));
        let done = e.call(JOB_DONE, &args![second, job, job, 0u32]).u32();
        e.call(SPARSE_WRITER_FLUSH, &args![f + 0x80]);
        done
    });
    profile_record(e, PROFILE_END);
    done
}

// Translated from 00df8570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuKdTreeBuildCoordinatorJob` (Xbox PDB name): cdecl, two stack words: a word that becomes
/// `ECX` of the final call `00d04e80` (it is also the job queue the setup jobs go to), and the
/// job. Writes `TtCpuKdTreeBuildCoord`, prepares four 0x70-byte jobs of type 5 on its stack (the
/// header, and 0 at `+0x10`, `+0x14`, `+0x18`, `+0x30` and `+0x60`) and stores the smaller of
/// `[job + 0x38]` and 4 (signed) in the word `[job + 0x14]` points to. With a positive count it
/// copies the job's words at `+0x10`, `+0x14` and `+0x1C` to `+0x40` into the first prepared job
/// (all at the same offsets), sets its word at `+0x60` to 1, runs `00e2aa70` with the words at
/// `[job + 0x50]`, `+0x38`, `+0x20` and `+0x10`, the count and the prepared jobs, then locks the
/// queue (`00d04400`), adds the first `count` prepared jobs (`00d04b00`) and unlocks
/// (`00d04420`). Ends with `00d04e80` and returns what it returns. Frame words the exe leaves
/// unwritten are zero here.
pub fn hk_cpu_kd_tree_build_coordinator_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_COORDINATOR);
    let job = job.addr();
    e.with_stack(0x400, |e, frame| {
        let f = frame.addr();
        let jobs = f + 0x20;
        for k in 0..4 {
            let at = jobs + 0x70 * k;
            write_job_header(e, at, 5, 0x70);
            for offset in [0x10, 0x14, 0x18, 0x30, 0x60] {
                e.mem.set_u32(at + offset, 0);
            }
        }
        let count = e.mem.i32(job + 0x38).min(4);
        let flag = e.mem.u32(job + 0x14);
        e.mem.set_i32(flag, count);
        if count > 0 {
            for offset in [
                0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
            ] {
                let value = e.mem.u32(job + offset);
                e.mem.set_u32(jobs + offset, value);
            }
            e.mem.set_u32(jobs + 0x60, 1);
            let args = args![
                e.mem.u32(job + 0x50),
                e.mem.u32(job + 0x38),
                e.mem.u32(job + 0x20),
                e.mem.u32(job + 0x10),
                count,
                jobs
            ];
            e.call(INITIALIZE_BUILD_SETUP_JOBS, &args);
            let token = e.call(LOCK_QUEUE, &args![second, f + 0x1e0]).u32();
            for k in 0..count as u32 {
                e.call(
                    ADD_JOB_QUEUE_LOCKED,
                    &args![second, token, jobs + 0x70 * k, 0u32],
                );
            }
            e.call(UNLOCK_QUEUE, &args![second, token]);
        }
    });
    let done = e.call(JOB_DONE, &args![second, job, job, 0u32]).u32();
    profile_record(e, PROFILE_END);
    done
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00df7e20,
            hk_cpu_kd_tree_build_fast_distributed_job(u32, Ptr) -> u32
        ),
        entry!(
            0x00df8440,
            hk_cpu_kd_tree_build_fast_distributed_sub_job(u32, Ptr) -> u32
        ),
        entry!(
            0x00df8570,
            hk_cpu_kd_tree_build_coordinator_job(u32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    /// A job of 0x60 bytes with the given words.
    fn job_with_words(e: &mut Engine, words: &[(u32, u32)]) -> u32 {
        let job = e.mem.alloc(0x60);
        for (offset, value) in words {
            e.mem.set_u32(job + offset, *value);
        }
        job
    }

    /// The addresses of the doubles that ran, in order.
    fn order(rig: &JobRig) -> Vec<u32> {
        rig.log
            .borrow()
            .iter()
            .map(|(address, _)| *address)
            .collect()
    }

    #[test]
    fn a_small_count_builds_the_tree_directly_and_copies_the_result() {
        let (mut e, rig) = job_rig();
        let sink = rig.log.clone();
        e.register_double(BUILD_TREE, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            sink.borrow_mut().push((BUILD_TREE, words));
            e.mem.set_u32(a[6], 0xabc);
            Ret::default()
        });
        log_double(&mut e, &rig.log, SPLIT_TREE_BUILDING, 0);
        let results = e.mem.alloc(0x10);
        let job = job_with_words(
            &mut e,
            &[
                (0x20, 1),
                (0x24, 2),
                (0x28, 3),
                (0x2c, 4),
                (0x30, results),
                (0x38, 7),
                (0x3c, 5),
                (0x40, 6),
            ],
        );
        let returned = e.call(0x00df_7e20, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let builds = calls(&rig.log, BUILD_TREE);
        assert_eq!(builds.len(), 1);
        assert_eq!(builds[0][1..6], [3, 4, 5, 7, 6]);
        assert_eq!(builds[0][7..], [1, 2]);
        assert_eq!(builds[0][6] + 4, builds[0][0]);
        for word in 0..4 {
            assert_eq!(e.mem.u32(results + 4 * word), 0xabc);
        }
        assert!(calls(&rig.log, SPLIT_TREE_BUILDING).is_empty());
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_FAST_DISTRIBUTED, PROFILE_END]
        );
    }

    #[test]
    fn a_negative_count_is_small_and_a_null_result_pointer_is_not_written() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, BUILD_TREE, 0);
        log_double(&mut e, &rig.log, SPLIT_TREE_BUILDING, 0);
        let job = job_with_words(&mut e, &[(0x38, 0xffff_ffff)]);
        let returned = e.call(0x00df_7e20, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let builds = calls(&rig.log, BUILD_TREE);
        assert_eq!(builds.len(), 1);
        assert_eq!(builds[0][4], 0xffff_ffff);
        assert!(calls(&rig.log, SPLIT_TREE_BUILDING).is_empty());
    }

    /// Everything a run of the job with a count of 9 leaves behind.
    struct Split {
        e: Engine,
        rig: JobRig,
        second: u32,
        entries: u32,
        counters: u32,
        flag: u32,
        results: u32,
    }

    /// Runs the job with the counters 1, 2, 3, 4 and a total of 22; the doubles of the split
    /// give each output structure `k` the words `10 + k`, `20 + k` (pair), `30 + k`, `40 + k` and
    /// `50 + k` (at `+4`, `+0xC` and `+0x10`) and the vector `0x1000 * (k + 1) + lane`, and log
    /// the structures and the bounds they were called with.
    fn run_split(with_results: bool) -> Split {
        let (mut e, rig) = job_rig();
        for lane in 0..4 {
            e.mem.set_u32(BUILDER_VECTOR + 4 * lane, 0x100 + lane);
        }
        let sink = rig.log.clone();
        e.register_double(SPLIT_TREE_BUILDING, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4)]);
            for offset in [0, 2, 4, 8, 10, 12] {
                words.push(u32::from(e.mem.u16(a[6] + offset)));
            }
            for k in 0..4 {
                let output = a[4] + 0x50 * k;
                words.push(e.mem.u32(output + 8));
                words.extend((0..4).map(|lane| e.mem.u32(output + 0x40 + 4 * lane)));
                let pair = e.mem.u32(output);
                e.mem.set_u32(pair, 10 + k);
                e.mem.set_u32(pair + 4, 20 + k);
                e.mem.set_u32(output + 4, 30 + k);
                e.mem.set_u32(output + 0xc, 40 + k);
                e.mem.set_u32(output + 0x10, 50 + k);
                for lane in 0..4 {
                    e.mem
                        .set_u32(output + 0x20 + 4 * lane, 0x1000 * (k + 1) + lane);
                }
            }
            sink.borrow_mut().push((SPLIT_TREE_BUILDING, words));
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(ADD_JOB_QUEUE_LOCKED, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..0x18).map(|word| e.mem.u32(a[2] + 4 * word)));
            sink.borrow_mut().push((ADD_JOB_QUEUE_LOCKED, words));
            Ret::default()
        });
        log_double(&mut e, &rig.log, LOCK_QUEUE, 0x77);
        log_double(&mut e, &rig.log, UNLOCK_QUEUE, 0);
        log_double(&mut e, &rig.log, BUILD_TREE, 0);
        let entries = e.mem.alloc(0x10);
        let counters = e.mem.alloc(0x10);
        for k in 0..4 {
            e.mem.set_u32(counters + 4 * k, 1 + k);
        }
        let flag = e.mem.alloc(4);
        let results = if with_results { e.mem.alloc(0x10) } else { 0 };
        let job = job_with_words(
            &mut e,
            &[
                (0x10, 0x110),
                (0x14, flag),
                (0x20, 0x120),
                (0x24, 0x124),
                (0x28, entries),
                (0x2c, 0x12c),
                (0x30, results),
                (0x34, counters),
                (0x38, 9),
                (0x3c, 0x13c),
                (0x40, 22),
            ],
        );
        let returned = e.call(0x00df_7e20, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        Split {
            e,
            rig,
            second: 0x99,
            entries,
            counters,
            flag,
            results,
        }
    }

    #[test]
    fn a_large_count_splits_the_tree_and_queues_four_jobs() {
        let run = run_split(true);
        let (e, rig) = (&run.e, &run.rig);
        let splits = calls(&rig.log, SPLIT_TREE_BUILDING);
        assert_eq!(splits.len(), 1);
        let split = &splits[0];
        // The input block, the job words, the count, the structures, the counters, the bounds.
        assert_eq!(split[1..4], [run.entries, 0x13c, 9]);
        assert_eq!(split[4], split[0] + 0x1b8);
        assert_eq!(split[5], run.counters);
        assert_eq!(split[6], split[0] + 8);
        assert_eq!(split[7..9], [0x120, 0x124]);
        assert_eq!(split[9..15], [0, 0, 0, 0xffff, 0xffff, 0xffff]);
        for k in 0..4 {
            let at = 15 + 5 * k;
            assert_eq!(split[at], 0xffff_ffff);
            assert_eq!(split[at + 1..at + 5], [0x100, 0x101, 0x102, 0x103]);
        }
        // The counters went up by 6 each (n = 5, and 1 more because it is odd).
        for k in 0..4 {
            assert_eq!(e.mem.u32(run.counters + 4 * k), 7 + k);
        }
        assert_eq!(e.mem.u32(run.flag), 4);
        assert_eq!(
            order(rig),
            vec![
                SPLIT_TREE_BUILDING,
                LOCK_QUEUE,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                UNLOCK_QUEUE,
                JOB_DONE,
            ]
        );
        assert_eq!(calls(&rig.log, LOCK_QUEUE)[0][0], run.second);
        assert_eq!(calls(&rig.log, UNLOCK_QUEUE), vec![vec![run.second, 0x77]]);
        let adds = calls(&rig.log, ADD_JOB_QUEUE_LOCKED);
        // The entry offsets skip 8 entries plus the counters of the earlier jobs.
        let offsets = [8, 15, 23, 32];
        let shares = [5, 5, 5, 7];
        for (k, add) in adds.iter().enumerate() {
            let block = |offset: usize| add[4 + offset / 4];
            assert_eq!(add[..2], [run.second, 0x77]);
            assert_eq!(add[2], split[0] + 0x18 + 0x60 * k as u32);
            assert_eq!(add[3], 0);
            assert_eq!(block(0), 0x0001_0304);
            assert_eq!(block(4), 0xffff_0060);
            assert_eq!(block(0x10), 0x110);
            assert_eq!(block(0x14), run.flag);
            assert_eq!(block(0x18), 0);
            assert_eq!(block(0x20), run.entries + 8 * offsets[k]);
            assert_eq!(block(0x24), 0x12c);
            assert_eq!(block(0x28), 30 + k as u32);
            assert_eq!(block(0x2c), 10 + k as u32);
            assert_eq!(block(0x30), 20 + k as u32);
            assert_eq!(block(0x34), 40 + k as u32);
            assert_eq!(block(0x38), 50 + k as u32);
            for lane in 0..4 {
                assert_eq!(
                    block(0x40 + 4 * lane),
                    0x1000 * (k as u32 + 1) + lane as u32
                );
            }
            assert_eq!(block(0x50), run.results + 4 * k as u32);
            assert_eq!(block(0x54), 0xffff_ffff);
            assert_eq!(block(0x58), shares[k]);
        }
        assert_eq!(adds.len(), 4);
        assert_eq!(
            rig.labels(e, false),
            vec![LABEL_FAST_DISTRIBUTED, PROFILE_END]
        );
    }

    #[test]
    fn a_large_count_without_a_result_array_queues_jobs_without_result_pointers() {
        let run = run_split(false);
        let adds = calls(&run.rig.log, ADD_JOB_QUEUE_LOCKED);
        assert_eq!(adds.len(), 4);
        for add in &adds {
            assert_eq!(add[4 + 0x50 / 4], 0);
        }
        assert_eq!(run.e.mem.u32(run.flag), 4);
        assert!(calls(&run.rig.log, BUILD_TREE).is_empty());
    }

    #[test]
    fn the_sub_job_builds_the_tree_with_a_sparse_writer_and_flushes_it() {
        let (mut e, rig) = job_rig();
        for lane in 0..4 {
            e.mem.set_u32(BUILDER_VECTOR + 4 * lane, 0x100 + lane);
        }
        log_double(&mut e, &rig.log, SPARSE_WRITER_CONSTRUCT, 0);
        log_double(&mut e, &rig.log, SPARSE_WRITER_FLUSH, 0);
        let sink = rig.log.clone();
        e.register_double(BUILD_TREE_RECURSIVE, move |e, a| {
            let mut words = a.to_vec();
            // The builder (20 words from the pointer to the pair), the pair, the input word,
            // the 1 and the size word.
            words.extend((0..20).map(|word| e.mem.u32(a[0] + 4 * word)));
            let pair = e.mem.u32(a[0]);
            words.extend([e.mem.u32(pair), e.mem.u32(pair + 4)]);
            words.extend([e.mem.u32(a[1]), e.mem.u32(a[3]), e.mem.u32(a[4])]);
            sink.borrow_mut().push((BUILD_TREE_RECURSIVE, words));
            Ret {
                eax: 40,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(4);
        let job = e.mem.alloc(0x60);
        for (offset, value) in [
            (0x20, 0x120),
            (0x24, 0x124),
            (0x28, 0x128),
            (0x2c, 0x12c),
            (0x30, 0x130),
            (0x34, 0x134),
            (0x38, 0x138),
            (0x40, 0x140),
            (0x44, 0x144),
            (0x48, 0x148),
            (0x4c, 0x14c),
            (0x50, out),
            (0x54, 0x154),
            (0x58, 0x158),
        ] {
            e.mem.set_u32(job + offset, value);
        }
        let returned = e.call(0x00df_8440, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let builds = calls(&rig.log, BUILD_TREE_RECURSIVE);
        assert_eq!(builds.len(), 1);
        let build = &builds[0];
        // Arguments: the builder, the input word, the job word, and the addresses of 1 and size.
        assert_eq!(build[2], 0x124);
        assert_eq!(build[1] + 0x14, build[0]);
        assert_eq!(build[3] + 4, build[4]);
        let words = &build[5..25];
        assert_eq!(words[0], build[0] - 8);
        assert_eq!(words[1..5], [0x128, 0x154, 0x134, 0x138]);
        assert_eq!(words[8..12], [0x140, 0x144, 0x148, 0x14c]);
        assert_eq!(words[12], build[0] + 0x50);
        assert_eq!(words[16..20], [0x100, 0x101, 0x102, 0x103]);
        assert_eq!(build[25..27], [0x12c, 0x130]);
        assert_eq!(build[27..30], [0x120, 1, 0x158]);
        assert_eq!(e.mem.u32(out), 42);
        assert_eq!(
            calls(&rig.log, SPARSE_WRITER_CONSTRUCT),
            vec![vec![build[0] + 0x50]]
        );
        assert_eq!(
            calls(&rig.log, SPARSE_WRITER_FLUSH),
            vec![vec![build[0] + 0x50]]
        );
        assert_eq!(
            order(&rig),
            vec![
                SPARSE_WRITER_CONSTRUCT,
                BUILD_TREE_RECURSIVE,
                JOB_DONE,
                SPARSE_WRITER_FLUSH
            ]
        );
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_FAST_DISTRIBUTED_SUB, PROFILE_END]
        );
    }

    /// The offsets the coordinator copies from the job into the first setup job.
    const COPIED: [u32; 12] = [
        0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
    ];

    #[test]
    fn the_coordinator_prepares_at_most_four_setup_jobs_and_queues_them() {
        let (mut e, rig) = job_rig();
        let sink = rig.log.clone();
        e.register_double(INITIALIZE_BUILD_SETUP_JOBS, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..0x1c).map(|word| e.mem.u32(a[5] + 4 * word)));
            sink.borrow_mut().push((INITIALIZE_BUILD_SETUP_JOBS, words));
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(ADD_JOB_QUEUE_LOCKED, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..0x1c).map(|word| e.mem.u32(a[2] + 4 * word)));
            sink.borrow_mut().push((ADD_JOB_QUEUE_LOCKED, words));
            Ret::default()
        });
        log_double(&mut e, &rig.log, LOCK_QUEUE, 0x66);
        log_double(&mut e, &rig.log, UNLOCK_QUEUE, 0);
        let flag = e.mem.alloc(4);
        let job = e.mem.alloc(0x60);
        e.mem.set_u32(job + 0x14, flag);
        e.mem.set_u32(job + 0x38, 9);
        e.mem.set_u32(job + 0x50, 0x150);
        for offset in COPIED {
            if offset != 0x14 && offset != 0x38 {
                e.mem.set_u32(job + offset, 0x100 + offset);
            }
        }
        let returned = e.call(0x00df_8570, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(e.mem.u32(flag), 4);
        let setups = calls(&rig.log, INITIALIZE_BUILD_SETUP_JOBS);
        assert_eq!(setups.len(), 1);
        let setup = &setups[0];
        // The words of the job, the count 4 (not 9) and the first setup job.
        assert_eq!(setup[..5], [0x150, 9, 0x120, 0x110, 4]);
        let first = &setup[6..];
        assert_eq!(first[0], 0x0001_0305);
        assert_eq!(first[1], 0xffff_0070);
        for offset in COPIED {
            let expected = if offset == 0x14 {
                flag
            } else if offset == 0x38 {
                9
            } else {
                0x100 + offset
            };
            assert_eq!(first[offset as usize / 4], expected);
        }
        assert_eq!(first[0x18 / 4], 0);
        assert_eq!(first[0x60 / 4], 1);
        assert_eq!(
            order(&rig),
            vec![
                INITIALIZE_BUILD_SETUP_JOBS,
                LOCK_QUEUE,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                ADD_JOB_QUEUE_LOCKED,
                UNLOCK_QUEUE,
                JOB_DONE,
            ]
        );
        let adds = calls(&rig.log, ADD_JOB_QUEUE_LOCKED);
        for (k, add) in adds.iter().enumerate() {
            assert_eq!(add[..2], [0x99, 0x66]);
            assert_eq!(add[2], setup[5] + 0x70 * k as u32);
            assert_eq!(add[3], 0);
            assert_eq!(add[4], 0x0001_0305);
            assert_eq!(add[5], 0xffff_0070);
            // Only the first setup job got the job's words; the others keep zeros.
            assert_eq!(add[4 + 0x10 / 4] == 0, k > 0);
            assert_eq!(add[4 + 0x60 / 4], u32::from(k == 0));
        }
        assert_eq!(adds.len(), 4);
        assert_eq!(calls(&rig.log, UNLOCK_QUEUE), vec![vec![0x99, 0x66]]);
        assert_eq!(rig.labels(&e, false), vec![LABEL_COORDINATOR, PROFILE_END]);
    }

    #[test]
    fn the_coordinator_with_no_jobs_to_set_up_only_stores_the_count() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, INITIALIZE_BUILD_SETUP_JOBS, 0);
        log_double(&mut e, &rig.log, LOCK_QUEUE, 0x66);
        log_double(&mut e, &rig.log, ADD_JOB_QUEUE_LOCKED, 0);
        log_double(&mut e, &rig.log, UNLOCK_QUEUE, 0);
        let flag = e.mem.alloc(4);
        let job = job_with_words(&mut e, &[(0x14, flag), (0x38, 0xffff_fffd)]);
        let returned = e.call(0x00df_8570, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(e.mem.i32(flag), -3);
        assert_eq!(order(&rig), vec![JOB_DONE]);
    }
}
