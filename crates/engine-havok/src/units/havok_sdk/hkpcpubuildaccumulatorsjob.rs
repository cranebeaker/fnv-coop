//! `Havok SDK/hkpcpubuildaccumulatorsjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (Xbox PDB name per the engine map,
/// `00d29830`): cdecl, five words: `owner +
/// 0xE0`, the motion list, its count, the size 0xE0 and the accumulators.
const APPLY_FORCES_AND_BUILD_ACCUMULATORS: u32 = 0x00d2_9830;
/// Profiling label `"StBuildAccumulators"`.
const LABEL_ST_BUILD_ACCUMULATORS: u32 = 0x010d_3ef4;

// Translated from 00d94df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuBuildAccumulatorsJob` (Xbox PDB name per the engine map): cdecl, three stack words: the
/// owner, a word that becomes `ECX` of the final call `00d04e80`, and the job. Writes the
/// 16-byte start record (`LtIntegrate`, `StBuildAccumulators`), calls `00d29830` with `owner +
/// 0xE0`, the motions `[job + 0x20] + 4 * u16(job + 0x24)`, the count `u16(job + 0x26)`, 0xE0
/// and the accumulators `[[job + 0x18] + 0x30] + 0x80 * (u16(job + 0x24) + 1)`, writes the
/// closing record and ends with `00d04e80`. Returns what that returns.
pub fn hk_cpu_build_accumulators_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_BUILD_ACCUMULATORS);
    let job = job.addr();
    let index = u32::from(e.mem.u16(job + 0x24));
    let set = e.mem.u32(job + 0x18);
    let accumulators = (index + 1)
        .wrapping_shl(7)
        .wrapping_add(e.mem.u32(set + 0x30));
    let motions = e.mem.u32(job + 0x20).wrapping_add(index.wrapping_mul(4));
    let count = u32::from(e.mem.u16(job + 0x26));
    e.call(
        APPLY_FORCES_AND_BUILD_ACCUMULATORS,
        &args![
            owner.addr().wrapping_add(0xe0),
            motions,
            count,
            0xe0u32,
            accumulators
        ],
    );
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d94df0,
        hk_cpu_build_accumulators_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::job_rig;

    #[test]
    fn builds_the_accumulators_for_the_slice_of_motions() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, APPLY_FORCES_AND_BUILD_ACCUMULATORS, 0);
        let set = e.mem.alloc(0x40);
        e.mem.set_u32(set + 0x30, 0x4000);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, set);
        e.mem.set_u32(job + 0x20, 0x5000);
        e.mem.set_u16(job + 0x24, 2);
        e.mem.set_u16(job + 0x26, 5);
        let returned = e.call(0x00d9_4df0, &args![0x1000u32, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(
            calls(&rig.log, APPLY_FORCES_AND_BUILD_ACCUMULATORS),
            vec![vec![0x10e0, 0x5008, 5, 0xe0, 0x4180]]
        );
        assert_eq!(
            rig.labels(&e, true),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_BUILD_ACCUMULATORS,
                LABEL_JOB_END
            ]
        );
    }
}
