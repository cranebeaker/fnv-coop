//! `Havok SDK/hkpcpubuildjacobiansjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `hkSimpleContactConstraintDataBuildJacobian` (Xbox PDB name per the engine map, `00d72190`):
/// cdecl `(atoms, input, flag dword, address of the output cursor)`; it reads the flag as a
/// byte.
const BUILD_CONTACT_JACOBIAN: u32 = 0x00d7_2190;
/// `hkSolverBuildJacobianFromAtomsNotContact` (Xbox PDB name per the engine map, `00d6f460`):
/// cdecl `(atoms, 16-bit word, input, address of the output cursor)`.
const BUILD_ATOMS_JACOBIAN: u32 = 0x00d6_f460;
/// The `double` 2^-23 (`010c9300`).
const EPSILON: u32 = 0x010c_9300;
/// Profiling label `"StBuildJacobians"`.
const LABEL_ST_BUILD_JACOBIANS: u32 = 0x010d_3ec8;

// Translated from 00d94a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuBuildJacobiansJob` (Xbox PDB name per the engine map): cdecl, three stack words: the
/// owner, a word that becomes `ECX` of the final call `00d04e80`, and the job. Writes the 16-byte
/// start record (`LtIntegrate`, `StBuildJacobians`), copies the 0x80 bytes at `owner + 0x10` to
/// a local input, and walks the `[block + 0x14]` constraint records (28 bytes each, from `block
/// + 0x18`, `block` = the word at `job + 0x20`). For each it fills the input: `+0x38`, `+0x3C`
/// from record `+0xC`, `+0x10`; `+0x30`, `+0x34` from the 16-bit words at `+0x18`, `+0x1A`
/// times 0x80 plus `[block + 8]`; `+0x6C`, `+0x70` those 16-bit words; `+0x60`, `+0x64`,
/// `+0x74` from record `+4`, `+8`, `+8`. A record whose atoms (the record's first word) start with
/// the 16-bit type 0x16 is built by `hkSimpleContactConstraintDataBuildJacobian` (flag 1),
/// any other by `hkSolverBuildJacobianFromAtomsNotContact` with the record's 16-bit word at
/// `+0x14`; both advance the output cursor, which starts at `[block + 0xC]`. At the end, when
/// the byte at `job + 0x28` and the word at `[block]` are both not 0, a gap of more than 0
/// bytes from the cursor to `[block + 0x10]` gets a marker: the gap size at `cursor + 4`,
/// `float(cursor) + 2^-23` (through `x87`, rounded to `float`) at `cursor` and the byte 2 at its
/// start; otherwise the word at the cursor is 0. Writes the closing record and ends with
/// `00d04e80`. Returns what that returns. (The three bytes above the flag byte the contact
/// build gets are padding; they are zero here.)
pub fn hk_cpu_build_jacobians_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_BUILD_JACOBIANS);
    let owner = owner.addr();
    let job = job.addr();
    let block = e.mem.u32(job + 0x20);
    e.with_stack(0x90, |e, frame| {
        let input = frame.addr();
        let cursor_cell = input + 0x80;
        let flag_cell = input + 0x84;
        for word in 0..0x20 {
            let value = e.mem.u32(owner + 0x10 + 4 * word);
            e.mem.set_u32(input + 4 * word, value);
        }
        let limit = e.mem.u32(block + 0x10);
        let count = e.mem.i32(block + 0x14);
        let base = e.mem.u32(block + 8);
        let cursor = e.mem.u32(block + 0xc);
        let mut record = block.wrapping_add(0x18);
        let marker = e.mem.u8(job + 0x28) != 0 && e.mem.u32(block) != 0;
        e.mem.set_u32(cursor_cell, cursor);
        for _ in 0..count.max(0) {
            let first = u32::from(e.mem.u16(record + 0x18));
            let second_index = u32::from(e.mem.u16(record + 0x1a));
            let word = e.mem.u32(record + 0xc);
            e.mem.set_u32(input + 0x38, word);
            let word = e.mem.u32(record + 0x10);
            e.mem.set_u32(input + 0x3c, word);
            e.mem.set_u32(input + 0x30, (first << 7).wrapping_add(base));
            e.mem
                .set_u32(input + 0x34, (second_index << 7).wrapping_add(base));
            e.mem.set_u32(input + 0x6c, first);
            e.mem.set_u32(input + 0x70, second_index);
            let word = e.mem.u32(record + 4);
            e.mem.set_u32(input + 0x60, word);
            let word = e.mem.u32(record + 8);
            e.mem.set_u32(input + 0x64, word);
            e.mem.set_u32(input + 0x74, word);
            let value = u32::from(e.mem.u16(record + 0x14));
            let atoms = e.mem.u32(record);
            record = record.wrapping_add(0x1c);
            if e.mem.u16(atoms) == 0x16 {
                e.mem.set_u8(flag_cell, 1);
                let flag = e.mem.u32(flag_cell);
                e.call(
                    BUILD_CONTACT_JACOBIAN,
                    &args![atoms, input, flag, cursor_cell],
                );
            } else {
                e.call(
                    BUILD_ATOMS_JACOBIAN,
                    &args![atoms, value, input, cursor_cell],
                );
            }
        }
        let cursor = e.mem.u32(cursor_cell);
        if marker {
            let gap = (limit as i32).wrapping_sub(cursor as i32);
            if gap > 0 {
                let old = x87_quiet(e.mem.f32(cursor));
                e.mem.set_u32(cursor + 4, gap as u32);
                let epsilon: f64 = e.global(EPSILON);
                e.mem.set_f32(cursor, (f64::from(old) + epsilon) as f32);
                e.mem.set_u8(cursor, 2);
            }
        } else {
            e.mem.set_u32(cursor, 0);
        }
    });
    profile_record(e, LABEL_JOB_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d94a80,
        hk_cpu_build_jacobians_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::calls;
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    /// Doubles for the two builders: they log their arguments with some words of the input
    /// (`+0x30`..`+0x3C`, `+0x60`..`+0x74`) and move the output cursor on by 0x10.
    fn builders(e: &mut Engine, rig: &JobRig) {
        for address in [BUILD_CONTACT_JACOBIAN, BUILD_ATOMS_JACOBIAN] {
            let sink = rig.log.clone();
            e.register_double(address, move |e, a| {
                let (input, cursor_cell) = if address == BUILD_CONTACT_JACOBIAN {
                    (a[1], a[3])
                } else {
                    (a[2], a[3])
                };
                let mut words = a.to_vec();
                for offset in [0x30, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x6c, 0x70, 0x74] {
                    words.push(e.mem.u32(input + offset));
                }
                sink.borrow_mut().push((address, words));
                let cursor = e.mem.u32(cursor_cell);
                e.mem.set_u32(cursor_cell, cursor + 0x10);
                Ret::default()
            });
        }
    }

    #[test]
    fn builds_each_record_and_marks_the_free_space() {
        let (mut e, rig) = job_rig();
        builders(&mut e, &rig);
        let owner = e.mem.alloc(0x100);
        for word in 0..0x20 {
            e.mem.set_u32(owner + 0x10 + 4 * word, 0x100 + word);
        }
        let output = e.mem.alloc(0x100);
        e.mem.set_f32(output + 0x20, 1.0);
        let contact = e.mem.alloc(0x10);
        e.mem.set_u16(contact, 0x16);
        let other = e.mem.alloc(0x10);
        e.mem.set_u16(other, 0x0b);
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 8, 0x1000);
        e.mem.set_u32(block + 0xc, output);
        e.mem.set_u32(block + 0x10, output + 0x80);
        e.mem.set_u32(block + 0x14, 2);
        for (slot, atoms) in [contact, other].into_iter().enumerate() {
            let record = block + 0x18 + 0x1c * slot as u32;
            e.mem.set_u32(record, atoms);
            e.mem.set_u32(record + 4, 0x40 + slot as u32);
            e.mem.set_u32(record + 8, 0x50 + slot as u32);
            e.mem.set_u32(record + 0xc, 0x60 + slot as u32);
            e.mem.set_u32(record + 0x10, 0x70 + slot as u32);
            e.mem.set_u16(record + 0x14, 0x14 + slot as u16);
            e.mem.set_u16(record + 0x18, 1 + slot as u16);
            e.mem.set_u16(record + 0x1a, 3 + slot as u16);
        }
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x20, block);
        e.mem.set_u8(job + 0x28, 1);
        let returned = e.call(0x00d9_4a80, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let contacts = calls(&rig.log, BUILD_CONTACT_JACOBIAN);
        let atoms = calls(&rig.log, BUILD_ATOMS_JACOBIAN);
        assert_eq!(contacts.len(), 1);
        assert_eq!(atoms.len(), 1);
        // The contact builder gets the atoms, the input, the flag byte 1 and the cursor cell.
        assert_eq!(contacts[0][0], contact);
        assert_eq!(contacts[0][2], 1);
        // Input fields for the first record, then the second.
        assert_eq!(
            contacts[0][4..],
            [0x1080, 0x1180, 0x60, 0x70, 0x40, 0x50, 1, 3, 0x50]
        );
        assert_eq!(atoms[0][..2], [other, 0x15]);
        assert_eq!(
            atoms[0][4..],
            [0x1100, 0x1200, 0x61, 0x71, 0x41, 0x51, 2, 4, 0x51]
        );
        // The input is a copy of the owner's, and both calls got the same cursor cell.
        assert_eq!(contacts[0][3], atoms[0][3]);
        // Two builds moved the cursor to +0x20, where the float 1.0 is; 0x60 bytes remain.
        assert_eq!(e.mem.u32(output + 0x24), 0x60);
        assert_eq!(e.mem.u32(output + 0x20), 0x3f80_0002);
        // Without the byte, the cursor word is cleared instead.
        e.mem.set_u8(job + 0x28, 0);
        e.mem.set_u32(output + 0x20, 0x1234_5678);
        e.call(0x00d9_4a80, &args![owner, 0x99u32, job]);
        assert_eq!(e.mem.u32(output + 0x20), 0);
    }

    #[test]
    fn a_full_buffer_gets_no_marker() {
        let (mut e, rig) = job_rig();
        builders(&mut e, &rig);
        let owner = e.mem.alloc(0x100);
        let output = e.mem.alloc(0x40);
        e.mem.set_u32(output, 0x3f80_0000);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 0xc, output);
        e.mem.set_u32(block + 0x10, output);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x20, block);
        e.mem.set_u8(job + 0x28, 1);
        e.call(0x00d9_4a80, &args![owner, 0u32, job]);
        // No records and no room: the word at the cursor is left alone.
        assert_eq!(e.mem.u32(output), 0x3f80_0000);
        assert_eq!(e.mem.u32(output + 4), 0);
    }
}
