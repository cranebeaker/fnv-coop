//! `Havok SDK/hkpcpusolveconstraintsjobs.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every job function here takes three stack words (cdecl): the owner, a word that becomes
//! `ECX` of the final call `00d04e80`, and the job; the owner's first word points to the solver
//! state whose parts at `+0x1e0` the functions pass on.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `00d89ed0`, cdecl `(state + 0x1e0, job + 0x20, job + 0x24)` (no name in the engine map).
const APPLY_GRAVITY: u32 = 0x00d8_9ed0;
/// `hkSolveConstraintsByTheSteps` (Xbox PDB name per the engine map, `00d8fb70`): cdecl, six
/// words.
const SOLVE_CONSTRAINTS_BY_THE_STEPS: u32 = 0x00d8_fb70;
/// `00d89f80` (no name in the engine map), cdecl `(state + 0x1e0, job + 0x28, job + 0x20, job +
/// 0x24, address of a byte flag)`.
const INTEGRATE_VELOCITIES: u32 = 0x00d8_9f80;
/// `hkExportImpulsesAndRhs` (Xbox PDB name per the engine map, `00df0680`): cdecl, four words.
const EXPORT_IMPULSES_AND_RHS: u32 = 0x00df_0680;
/// `hkSolveConstraints` (Xbox PDB name per the engine map, `00d8d030`): cdecl, four words.
const SOLVE_CONSTRAINTS: u32 = 0x00d8_d030;
/// `hkJobQueue::addJob` (Xbox PDB name per the engine map, `00d04b50`): `ECX` = the queue, two
/// stack words (the job, 0).
pub(crate) const ADD_JOB: u32 = 0x00d0_4b50;
/// Profiling labels: `"StAppGrav"`, `"StSolve"`, `"StIntVel"`, `"StSolverExport"`.
const LABEL_ST_APP_GRAV: u32 = 0x010d_3e98;
const LABEL_ST_SOLVE: u32 = 0x010d_3ea4;
const LABEL_ST_INT_VEL: u32 = 0x010d_3eac;
const LABEL_ST_SOLVER_EXPORT: u32 = 0x010d_3eb8;

/// The solver state the owner's first word points to.
fn solver_state(e: &Engine, owner: Ptr) -> u32 {
    e.mem.u32(owner.addr())
}

/// Zeroes `blocks` blocks of 16 bytes from `at`.
fn zero_blocks(e: &mut Engine, at: u32, blocks: u32) {
    for block in 0..blocks {
        for word in 0..4 {
            e.mem.set_u32(at.wrapping_add(16 * block + 4 * word), 0);
        }
    }
}

// Translated from 00d944d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuSolveApplyGravityJob` (Xbox PDB name per the engine map): cdecl, three stack words (see
/// the module comment). Writes the 16-byte start record (`LtIntegrate`, `StAppGrav`), calls
/// `00d89ed0` with `state + 0x1e0` and the job's words at `+0x20` and `+0x24`, writes the
/// closing record and ends with `00d04e80`. Returns what that returns.
pub fn hk_cpu_solve_apply_gravity_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_APP_GRAV);
    let job = job.addr();
    let state = solver_state(e, owner);
    let first = e.mem.u32(job + 0x20);
    let second_word = e.mem.u32(job + 0x24);
    e.call(
        APPLY_GRAVITY,
        &args![state.wrapping_add(0x1e0), first, second_word],
    );
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

// Translated from 00d94570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, three stack words (see the module comment). Writes the
/// 16-byte start record (`LtIntegrate`, `StSolve`), stores the solver state's word at `+0x2f8`
/// in `job + 0x24` and, when `job + 0x20` is 0, zeroes `(size + 15) / 16` blocks of 16 bytes at
/// `[block + 0x10]` where `block` is the job's word at `+0x2c` and `size` its word at `+0x18`.
/// Then calls `hkSolveConstraintsByTheSteps` with `state + 0x1e0`, `job + 0x20`, `job + 0x28`
/// and the words of `block` at `+0xc`, `+8` and `+0x10`, writes the closing record and ends
/// with `00d04e80`. Returns what that returns.
pub fn fn_00d94570(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_SOLVE);
    let job = job.addr();
    let block = e.mem.u32(job + 0x2c);
    let word_c = e.mem.u32(block + 0xc);
    let word_8 = e.mem.u32(block + 8);
    let data = e.mem.u32(block + 0x10);
    let state = solver_state(e, owner);
    let word_2f8 = e.mem.u32(state + 0x2f8);
    e.mem.set_u32(job + 0x24, word_2f8);
    if e.mem.u32(job + 0x20) == 0 {
        let blocks = e.mem.u32(block + 0x18).wrapping_add(0xf) >> 4;
        zero_blocks(e, data, blocks);
    }
    let flag = e.mem.u32(job + 0x28);
    let selector = e.mem.u32(job + 0x20);
    e.call(
        SOLVE_CONSTRAINTS_BY_THE_STEPS,
        &args![
            state.wrapping_add(0x1e0),
            selector,
            flag,
            word_c,
            word_8,
            data
        ],
    );
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

// Translated from 00d94670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuSolveIntegrateVelocitiesJob` (Xbox PDB name per the engine map): cdecl, three stack
/// words (see the module comment). Writes the 16-byte start record (`LtIntegrate`, `StIntVel`),
/// calls `00d89f80` with `state + 0x1e0`, the job's words at `+0x28`, `+0x20` and `+0x24` and the
/// address of a byte flag that starts as 0, stores the flag at `job + 0x2c`, writes the closing
/// record and ends with `00d04e80`. Returns what that returns.
pub fn hk_cpu_solve_integrate_velocities_job(
    e: &mut Engine,
    owner: Ptr,
    second: u32,
    job: Ptr,
) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_INT_VEL);
    let job = job.addr();
    let state = solver_state(e, owner);
    let flag = e.with_stack(4, |e, cell| {
        let words = [
            state.wrapping_add(0x1e0),
            e.mem.u32(job + 0x28),
            e.mem.u32(job + 0x20),
            e.mem.u32(job + 0x24),
        ];
        e.call(
            INTEGRATE_VELOCITIES,
            &args![words[0], words[1], words[2], words[3], cell],
        );
        e.mem.u8(cell.addr())
    });
    e.mem.set_u8(job + 0x2c, flag);
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

// Translated from 00d94730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, three stack words (see the module comment). Writes the
/// 16-byte start record (`LtIntegrate`, `StSolverExport`), calls `hkExportImpulsesAndRhs` with
/// `state + 0x1e0` and the words at `+0x10`, `+0xc`, `+8` of the block `job + 0x20` points to,
/// writes the closing record and ends with `00d04e80`. Returns what that returns.
pub fn fn_00d94730(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_SOLVER_EXPORT);
    let job = job.addr();
    let block = e.mem.u32(job + 0x20);
    let state = solver_state(e, owner);
    let words = [
        e.mem.u32(block + 0x10),
        e.mem.u32(block + 0xc),
        e.mem.u32(block + 8),
    ];
    e.call(
        EXPORT_IMPULSES_AND_RHS,
        &args![state.wrapping_add(0x1e0), words[0], words[1], words[2]],
    );
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

// Translated from 00d947e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuSolveConstraintsJob` (Xbox PDB name per the engine map): cdecl, three stack words (see
/// the module comment). Writes the 16-byte start record (`LtIntegrate`, `StSolve`), zeroes
/// `(4 * [job + 0x38] + 15) / 16` blocks of 16 bytes at `[job + 0x30] + [job + 0x20]`, calls
/// `hkSolveConstraints` with `state + 0x1e0`, `[job + 0x2c] + [job + 0x20]`, `[job + 0x28] +
/// [job + 0x20]` and the zeroed address. It then queues a follow-up job through
/// `hkJobQueue::addJob` (`ECX` = the second word): a 0x40-byte block with the type bytes 0xB, 0,
/// 2, the size 0x40 at `+4`, 0xFFFF at `+6`, the job's 16-bit word at `+0x10` and words at
/// `+0x14`, `+0x1c`, the set (`[job + 0x18]`) at `+0x18`, the set's words at `+0x78`, `+0x14`
/// and its 16-bit word at `+0x54` in `+0x20`, `+0x30`, `+0x34`; the bytes the exe leaves
/// unwritten (padding) are zero here. Then writes the record `StSolverExport`, calls
/// `hkExportImpulsesAndRhs` with `state + 0x1e0` and the set's words at `+0x40`, `+0x3c`,
/// `+0x30`, writes the closing record and ends with `00d04e80`. Returns what that returns.
pub fn hk_cpu_solve_constraints_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_SOLVE);
    let job = job.addr();
    let base = e.mem.u32(job + 0x20);
    let by_the_steps = e.mem.u32(job + 0x28).wrapping_add(base);
    let constraints = e.mem.u32(job + 0x2c).wrapping_add(base);
    let zeroed = e.mem.u32(job + 0x30).wrapping_add(base);
    let blocks = e.mem.u32(job + 0x38).wrapping_mul(4).wrapping_add(0xf) >> 4;
    zero_blocks(e, zeroed, blocks);
    let state = solver_state(e, owner);
    e.call(
        SOLVE_CONSTRAINTS,
        &args![state.wrapping_add(0x1e0), constraints, by_the_steps, zeroed],
    );
    let set = e.mem.u32(job + 0x18);
    e.with_stack(0x44, |e, follow_up| {
        let at = follow_up.addr();
        let job_word = e.mem.u16(job + 0x10);
        e.mem.set_u16(at + 4, 0x40);
        e.mem.set_u16(at + 0x10, job_word);
        let word = e.mem.u32(job + 0x14);
        e.mem.set_u32(at + 0x14, word);
        let word = e.mem.u32(set + 0x78);
        e.mem.set_u32(at + 0x20, word);
        e.mem.set_u16(at + 6, 0xffff);
        let word = e.mem.u32(job + 0x1c);
        e.mem.set_u32(at + 0x1c, word);
        let word = e.mem.u32(set + 0x14);
        e.mem.set_u32(at + 0x30, word);
        e.mem.set_u16(at + 0x36, 0);
        e.mem.set_u8(at + 0x3c, 0);
        let word = e.mem.u16(set + 0x54);
        e.mem.set_u8(at, 0xb);
        e.mem.set_u8(at + 1, 0);
        e.mem.set_u8(at + 2, 2);
        e.mem.set_u32(at + 0x18, set);
        e.mem.set_u16(at + 0x34, word);
        e.call(ADD_JOB, &args![second, at, 0u32]);
    });
    profile_record(e, LABEL_ST_SOLVER_EXPORT);
    let words = [
        e.mem.u32(set + 0x30),
        e.mem.u32(set + 0x3c),
        e.mem.u32(set + 0x40),
    ];
    e.call(
        EXPORT_IMPULSES_AND_RHS,
        &args![state.wrapping_add(0x1e0), words[2], words[1], words[0]],
    );
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d944d0,
            hk_cpu_solve_apply_gravity_job(Ptr, u32, Ptr) -> u32
        ),
        entry!(0x00d94570, fn_00d94570(Ptr, u32, Ptr) -> u32),
        entry!(
            0x00d94670,
            hk_cpu_solve_integrate_velocities_job(Ptr, u32, Ptr) -> u32
        ),
        entry!(0x00d94730, fn_00d94730(Ptr, u32, Ptr) -> u32),
        entry!(
            0x00d947e0,
            hk_cpu_solve_constraints_job(Ptr, u32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    /// The solver state (0x300 bytes, with 0xAB at `+0x2f8`) and the owner pointing to it.
    fn owner(e: &mut Engine) -> (u32, u32) {
        let state = e.mem.alloc(0x300);
        e.mem.set_u32(state + 0x2f8, 0xab);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner, state);
        (owner, state)
    }

    /// Checks the records of a job that opens with the start record `(LtIntegrate, label)`.
    fn assert_records(rig: &JobRig, e: &Engine, label: u32, middle: &[u32]) {
        let mut expected = vec![LABEL_LT_INTEGRATE, label];
        expected.extend_from_slice(middle);
        expected.push(LABEL_JOB_END);
        assert_eq!(rig.labels(e, true), expected);
    }

    #[test]
    fn apply_gravity_passes_the_two_job_words() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, APPLY_GRAVITY, 0);
        let (owner, state) = owner(&mut e);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x20, 0x21);
        e.mem.set_u32(job + 0x24, 0x22);
        let returned = e.call(0x00d9_44d0, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(
            calls(&rig.log, APPLY_GRAVITY),
            vec![vec![state + 0x1e0, 0x21, 0x22]]
        );
        assert_records(&rig, &e, LABEL_ST_APP_GRAV, &[]);
    }

    #[test]
    fn fn_00d94570_zeroes_the_block_only_without_a_selector() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, SOLVE_CONSTRAINTS_BY_THE_STEPS, 0);
        let (owner, state) = owner(&mut e);
        let data = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(data + 4 * word, 0xdead_beef);
        }
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 8, 0x81);
        e.mem.set_u32(block + 0xc, 0x82);
        e.mem.set_u32(block + 0x10, data);
        // 0x25 bytes round up to three blocks of 16.
        e.mem.set_u32(block + 0x18, 0x25);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x28, 0x28);
        e.mem.set_u32(job + 0x2c, block);
        let returned = e.call(0x00d9_4570, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(e.mem.u32(job + 0x24), 0xab);
        assert_eq!(e.mem.u32(data + 0x2c), 0);
        assert_eq!(e.mem.u32(data + 0x30), 0xdead_beef);
        assert_eq!(
            calls(&rig.log, SOLVE_CONSTRAINTS_BY_THE_STEPS),
            vec![vec![state + 0x1e0, 0, 0x28, 0x82, 0x81, data]]
        );
        assert_records(&rig, &e, LABEL_ST_SOLVE, &[]);
        // With a selector nothing is zeroed.
        e.mem.set_u32(data, 0x1234);
        e.mem.set_u32(job + 0x20, 7);
        e.call(0x00d9_4570, &args![owner, 0x99u32, job]);
        assert_eq!(e.mem.u32(data), 0x1234);
    }

    #[test]
    fn integrate_velocities_reports_the_flag_the_callee_sets() {
        let (mut e, rig) = job_rig();
        let (owner, state) = owner(&mut e);
        let log = rig.log.clone();
        e.register_double(INTEGRATE_VELOCITIES, move |e, a| {
            // The flag starts as 0.
            let mut words = a.to_vec();
            words.push(u32::from(e.mem.u8(a[4])));
            log.borrow_mut().push((INTEGRATE_VELOCITIES, words));
            e.mem.set_u8(a[4], 1);
            Ret::default()
        });
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x20, 0x21);
        e.mem.set_u32(job + 0x24, 0x22);
        e.mem.set_u32(job + 0x28, 0x23);
        let returned = e.call(0x00d9_4670, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let seen = calls(&rig.log, INTEGRATE_VELOCITIES);
        assert_eq!(seen[0][..4], [state + 0x1e0, 0x23, 0x21, 0x22]);
        assert_eq!(seen[0][5], 0);
        assert_eq!(e.mem.u8(job + 0x2c), 1);
        assert_records(&rig, &e, LABEL_ST_INT_VEL, &[]);
    }

    #[test]
    fn fn_00d94730_exports_the_three_block_words() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, EXPORT_IMPULSES_AND_RHS, 0);
        let (owner, state) = owner(&mut e);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 8, 0x31);
        e.mem.set_u32(block + 0xc, 0x32);
        e.mem.set_u32(block + 0x10, 0x33);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x20, block);
        let returned = e.call(0x00d9_4730, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(
            calls(&rig.log, EXPORT_IMPULSES_AND_RHS),
            vec![vec![state + 0x1e0, 0x33, 0x32, 0x31]]
        );
        assert_records(&rig, &e, LABEL_ST_SOLVER_EXPORT, &[]);
    }

    #[test]
    fn solve_constraints_queues_the_follow_up_job_and_exports() {
        let (mut e, rig) = job_rig();
        log_double(&mut e, &rig.log, SOLVE_CONSTRAINTS, 0);
        log_double(&mut e, &rig.log, EXPORT_IMPULSES_AND_RHS, 0);
        let sink = rig.log.clone();
        e.register_double(ADD_JOB, move |e, a| {
            // The queued job: its 0x40 bytes, as words.
            let mut words = a.to_vec();
            words.extend((0..16).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((ADD_JOB, words));
            Ret::default()
        });
        let (owner, state) = owner(&mut e);
        let area = e.mem.alloc(0x100);
        for word in 0..0x40 {
            e.mem.set_u32(area + 4 * word, 0xdead_beef);
        }
        let set = e.mem.alloc(0x80);
        e.mem.set_u32(set + 0x14, 0x4141);
        e.mem.set_u32(set + 0x30, 0x4343);
        e.mem.set_u32(set + 0x3c, 0x4444);
        e.mem.set_u32(set + 0x40, 0x4545);
        e.mem.set_u16(set + 0x54, 0x5454);
        e.mem.set_u32(set + 0x78, 0x7878);
        let job = e.mem.alloc(0x60);
        e.mem.set_u16(job + 0x10, 0x1010);
        e.mem.set_u32(job + 0x14, 0x1414);
        e.mem.set_u32(job + 0x18, set);
        e.mem.set_u32(job + 0x1c, 0x1c1c);
        e.mem.set_u32(job + 0x20, area);
        e.mem.set_u32(job + 0x28, 0x10);
        e.mem.set_u32(job + 0x2c, 0x20);
        e.mem.set_u32(job + 0x30, 0x40);
        // Five words round up to two blocks of 16 bytes.
        e.mem.set_u32(job + 0x38, 5);
        let returned = e.call(0x00d9_47e0, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(e.mem.u32(area + 0x40 + 0x20), 0xdead_beef);
        assert_eq!(e.mem.u32(area + 0x40 + 0x1c), 0);
        assert_eq!(
            calls(&rig.log, SOLVE_CONSTRAINTS),
            vec![vec![state + 0x1e0, area + 0x20, area + 0x10, area + 0x40]]
        );
        let queued = calls(&rig.log, ADD_JOB);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][0], 0x99);
        assert_eq!(queued[0][2], 0);
        let words = &queued[0][3..];
        assert_eq!(words[0], 0x0002_000b);
        assert_eq!(words[1], 0xffff_0040);
        assert_eq!(words[4], 0x1010);
        assert_eq!(words[5], 0x1414);
        assert_eq!(words[6], set);
        assert_eq!(words[7], 0x1c1c);
        assert_eq!(words[8], 0x7878);
        assert_eq!(words[0xc], 0x4141);
        assert_eq!(words[0xd], 0x5454);
        assert_eq!(words[0xf], 0);
        assert_eq!(
            calls(&rig.log, EXPORT_IMPULSES_AND_RHS),
            vec![vec![state + 0x1e0, 0x4545, 0x4444, 0x4343]]
        );
        assert_records(&rig, &e, LABEL_ST_SOLVE, &[LABEL_ST_SOLVER_EXPORT]);
    }
}
