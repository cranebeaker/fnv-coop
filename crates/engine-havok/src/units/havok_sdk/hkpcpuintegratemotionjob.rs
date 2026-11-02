//! `Havok SDK/hkpcpuintegratemotionjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `hkRigidMotionUtilApplyForcesAndStep` and `hkRigidMotionUtilApplyAccumulators` (Xbox PDB
/// names per the engine map, `00d28a30` and `00d29bf0`): cdecl, six words each: the solver
/// state at `+0x1e0`, the one at `+0x1d0`, the third (`+0x200` for the first function, the
/// accumulators for the second), the list of motions, their count and the size 0xE0.
pub(crate) const APPLY_FORCES_AND_STEP: u32 = 0x00d2_8a30;
const APPLY_ACCUMULATORS: u32 = 0x00d2_9bf0;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB name per the engine map, `00d1a330`):
/// cdecl `(world word, entities, count)`.
pub(crate) const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// Profiling label `"StIntegrateMotions"`.
const LABEL_ST_INTEGRATE_MOTIONS: u32 = 0x010d_3e84;

// Translated from 00d943d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuIntegrateMotionJob` (Xbox PDB name per the engine map): cdecl, three stack words: the
/// owner, a word that becomes `ECX` of the final call, and the job. Writes the 16-byte start
/// record (`LtIntegrate`, `StIntegrateMotions`), takes the job's motion list (`[[job + 0x18] +
/// 0x50] + 4 * u16(job + 0x36)`) and count (`u16(job + 0x34)`), runs
/// `hkRigidMotionUtilApplyForcesAndStep` when the byte at `job + 0x3C` is not 0 and
/// `hkRigidMotionUtilApplyAccumulators` (with the accumulators `[[job + 0x18] + 0x30] + 0x80 *
/// (u16(job + 0x36) + 1)`) otherwise, stores the result at `job + 0x38`, recalculates the
/// entities' AABBs (`entityBatchRecalcAabb` with the word at `[owner] + 0x78`), writes the
/// closing record and ends with `00d04e80`. Returns what that returns.
pub fn hk_cpu_integrate_motion_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_INTEGRATE_MOTIONS);
    let owner = owner.addr();
    let job = job.addr();
    let count = u32::from(e.mem.u16(job + 0x34));
    let index = u32::from(e.mem.u16(job + 0x36));
    let set = e.mem.u32(job + 0x18);
    let accumulators = (index + 1)
        .wrapping_shl(7)
        .wrapping_add(e.mem.u32(set + 0x30));
    let motions = e.mem.u32(set + 0x50).wrapping_add(index.wrapping_mul(4));
    let state = e.mem.u32(owner);
    let result = if e.mem.u8(job + 0x3c) != 0 {
        e.call(
            APPLY_FORCES_AND_STEP,
            &args![
                state.wrapping_add(0x1e0),
                state.wrapping_add(0x1d0),
                state.wrapping_add(0x200),
                motions,
                count,
                0xe0u32
            ],
        )
    } else {
        e.call(
            APPLY_ACCUMULATORS,
            &args![
                state.wrapping_add(0x1e0),
                state.wrapping_add(0x1d0),
                accumulators,
                motions,
                count,
                0xe0u32
            ],
        )
    };
    e.mem.set_u32(job + 0x38, result.u32());
    let world = e.mem.u32(e.mem.u32(owner) + 0x78);
    e.call(ENTITY_BATCH_RECALC_AABB, &args![world, motions, count]);
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d943d0,
        hk_cpu_integrate_motion_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::job_rig;

    #[test]
    fn integrates_with_accumulators_or_with_forces() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, APPLY_ACCUMULATORS, 0x66);
        log_double(&mut e, &rig.log, APPLY_FORCES_AND_STEP, 0x77);
        log_double(&mut e, &rig.log, ENTITY_BATCH_RECALC_AABB, 0);
        let state = e.mem.alloc(0x300);
        e.mem.set_u32(state + 0x78, 0x7777);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner, state);
        let set = e.mem.alloc(0x60);
        e.mem.set_u32(set + 0x30, 0x4000);
        e.mem.set_u32(set + 0x50, 0x5000);
        let job = e.mem.alloc(0x50);
        e.mem.set_u32(job + 0x18, set);
        e.mem.set_u16(job + 0x34, 3);
        e.mem.set_u16(job + 0x36, 2);
        let returned = e.call(0x00d9_43d0, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        // Entry 2 of the list, and the third block of accumulators after the first.
        assert_eq!(
            calls(&rig.log, APPLY_ACCUMULATORS),
            vec![vec![state + 0x1e0, state + 0x1d0, 0x4180, 0x5008, 3, 0xe0]]
        );
        assert!(calls(&rig.log, APPLY_FORCES_AND_STEP).is_empty());
        assert_eq!(e.mem.u32(job + 0x38), 0x66);
        assert_eq!(
            calls(&rig.log, ENTITY_BATCH_RECALC_AABB),
            vec![vec![0x7777, 0x5008, 3]]
        );
        assert_eq!(
            rig.labels(&e, true),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_INTEGRATE_MOTIONS,
                LABEL_JOB_END
            ]
        );
        // With the byte set, the other function runs.
        e.mem.set_u8(job + 0x3c, 1);
        e.call(0x00d9_43d0, &args![owner, 0x99u32, job]);
        assert_eq!(
            calls(&rig.log, APPLY_FORCES_AND_STEP),
            vec![vec![
                state + 0x1e0,
                state + 0x1d0,
                state + 0x200,
                0x5008,
                3,
                0xe0
            ]]
        );
        assert_eq!(e.mem.u32(job + 0x38), 0x77);
    }
}
