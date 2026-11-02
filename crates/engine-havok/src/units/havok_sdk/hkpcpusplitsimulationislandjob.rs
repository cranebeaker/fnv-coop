//! `Havok SDK/hkpcpusplitsimulationislandjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// The splitting of the island (`00d942f0`, cdecl `(first word of the job at +0x14, island +
/// 0x5c)`); no name in the engine map.
pub(crate) const SPLIT_ISLAND: u32 = 0x00d9_42f0;
/// Profiling label `"TtSplitIsle"`.
const LABEL_TT_SPLIT_ISLE: u32 = 0x010d_3e78;

// Translated from 00d94340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuSplitSimulationIslandJob` (Xbox PDB name per the engine map): cdecl, three stack
/// words: the owner (not used), a word that becomes `ECX` of the final call, and the job. Writes
/// the record `TtSplitIsle`, calls `00d942f0` with the job's word at `+0x14` and its word at
/// `+0x18` plus `0x5C`, writes the closing record `Et` (`010c41f8`) and ends with `00d04e80`
/// (the job twice and 0). Returns what that returns.
pub fn hk_cpu_split_simulation_island_job(
    e: &mut Engine,
    _owner: Ptr,
    second: u32,
    job: Ptr,
) -> u32 {
    profile_record(e, LABEL_TT_SPLIT_ISLE);
    let job = job.addr();
    let first = e.mem.u32(job + 0x14);
    let island = e.mem.u32(job + 0x18);
    e.call(SPLIT_ISLAND, &args![first, island.wrapping_add(0x5c)]);
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d94340,
        hk_cpu_split_simulation_island_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::job_rig;

    #[test]
    fn splits_the_island_between_the_two_records() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, SPLIT_ISLAND, 0);
        let job = e.mem.alloc(0x30);
        e.mem.set_u32(job + 0x14, 0x111);
        e.mem.set_u32(job + 0x18, 0x2000);
        let returned = e.call(0x00d9_4340, &args![0u32, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(calls(&rig.log, SPLIT_ISLAND), vec![vec![0x111, 0x205c]]);
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_TT_SPLIT_ISLE, PROFILE_END]
        );
    }
}
