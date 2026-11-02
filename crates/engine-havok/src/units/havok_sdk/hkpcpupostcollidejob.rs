//! `Havok SDK/hkpcpupostcollidejob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{
    JOB_DONE, LABEL_JOB_END, LOCK_ISLAND, RUN_COMMANDS, UNLOCK_ISLAND,
};
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_SLOT, TLS_GET_VALUE};

/// Frees the result table of the job (`00d965b0`, `ECX` = the table, nothing on the stack): the
/// blocks it lists and the table itself, through the memory router's allocator (the counterpart
/// of `00d96530`; no name in the engine map).
const FREE_RESULTS: u32 = 0x00d9_65b0;
/// Profiling labels: `"LtNarrowPhase"` and `"StPostCollide"` (a start record), `"MinumCmds"`
/// (a record of a label and a `float`) and the closing label at `01076f38`.
const LABEL_LT_NARROW_PHASE: u32 = 0x010d_3de4;
const LABEL_ST_POST_COLLIDE: u32 = 0x010d_3dd4;
const LABEL_MINUM_CMDS: u32 = 0x010d_3dc8;

/// Appends a record of a label and a `float` to the thread's record stream (the time stamp slot
/// holds the value) while the cursor (`+4`) is below the limit (`+0xC`); the record is 8 bytes.
fn profile_value_record(e: &mut Engine, label: u32, value: i32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_f32(cursor + 4, value as f32);
        e.mem.set_u32(stream + 4, cursor + 8);
    }
}

// Translated from 00d931a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuPostCollideJob` (Xbox PDB name per the engine map): cdecl, three stack words: the
/// owner, a word that becomes `ECX` of the final call, and the job. Writes the 16-byte start
/// record (`LtNarrowPhase`, `StPostCollide`), locks the world (`job + 0x14`) for constraint
/// updates and folds the result blocks of the job (the table at `job + 0x20`: a count, then
/// block pointers from `+0xC`, indexed as 16-bit values) into it: the maximum of the first
/// words (signed) at `world + 8` and the sums of the next three words at `world + 0xC`,
/// `+0x10`, `+0x14`. A block with a command byte count at `+0x14` writes a `MinumCmds` record
/// of the count divided by 16 and runs the commands (`block + 0x10 + [block + 0x10]`). Then it
/// frees the table, unlocks the world, writes a closing record and returns 2, or, when the
/// second word is not 0, what `00d04e80` returns for the job. (The exe also loads the pointer
/// of the block four ahead as a cache hint; the value is not used.)
pub fn hk_cpu_post_collide_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    profile_start_record(e, LABEL_LT_NARROW_PHASE, LABEL_ST_POST_COLLIDE);
    let owner = owner.addr();
    let job = job.addr();
    let world = e.mem.u32(job + 0x14);
    let lock_object = e.mem.u32(owner);
    e.call(LOCK_ISLAND, &args![lock_object, world]);
    let count = e.mem.i32(e.mem.u32(job + 0x20));
    let mut index = 0i32;
    while index < count {
        let table = e.mem.u32(job + 0x20);
        let block = e.mem.u32(
            table
                .wrapping_add(0xc)
                .wrapping_add((i32::from(index as i16) * 4) as u32),
        );
        let widest = e.mem.i32(world + 8).max(e.mem.i32(block));
        e.mem.set_u32(world + 8, widest as u32);
        for (offset, word) in [(0xc, 4), (0x10, 8), (0x14, 0xc)] {
            let total = e
                .mem
                .u32(world + offset)
                .wrapping_add(e.mem.u32(block + word));
            e.mem.set_u32(world + offset, total);
        }
        let size = e.mem.u32(block + 0x14);
        if size != 0 {
            profile_value_record(e, LABEL_MINUM_CMDS, (size >> 4) as i32);
            let commands = block
                .wrapping_add(0x10)
                .wrapping_add(e.mem.u32(block + 0x10));
            let lock_object = e.mem.u32(owner);
            e.call(RUN_COMMANDS, &args![lock_object, commands, size]);
        }
        index += 1;
    }
    let table = e.mem.u32(job + 0x20);
    e.call(FREE_RESULTS, &args![table]);
    let lock_object = e.mem.u32(owner);
    e.call(UNLOCK_ISLAND, &args![lock_object, world]);
    profile_record(e, LABEL_JOB_END);
    if second != 0 {
        e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
    } else {
        2
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d931a0,
        hk_cpu_post_collide_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, Log,
    };

    /// The owner (its first word 0x1234), a world with the counters 3, 10, 20, 30, a job with
    /// the result blocks `blocks` (each: the four counters, the command offset word and the
    /// command byte count) and the doubles for everything the job calls. Returns `(owner,
    /// job, world, table, log)`.
    fn setup(e: &mut Engine, blocks: &[([u32; 4], u32)]) -> (u32, u32, u32, u32, Log) {
        let log = new_log();
        for address in [LOCK_ISLAND, UNLOCK_ISLAND, RUN_COMMANDS, FREE_RESULTS] {
            log_double(e, &log, address, 0);
        }
        log_double(e, &log, JOB_DONE, 0x55);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner, 0x1234);
        let world = e.mem.alloc(0x20);
        for (offset, value) in [(8, 3), (0xc, 10), (0x10, 20), (0x14, 30)] {
            e.mem.set_u32(world + offset, value);
        }
        let table = e.mem.alloc(0xc + 4 * blocks.len() as u32);
        e.mem.set_u32(table, blocks.len() as u32);
        for (slot, (counters, size)) in blocks.iter().enumerate() {
            let block = e.mem.alloc(0x80);
            for (word, counter) in counters.iter().enumerate() {
                e.mem.set_u32(block + 4 * word as u32, *counter);
            }
            e.mem.set_u32(block + 0x10, 8);
            e.mem.set_u32(block + 0x14, *size);
            e.mem.set_u32(table + 0xc + 4 * slot as u32, block);
        }
        let job = e.mem.alloc(0x30);
        e.mem.set_u32(job + 0x14, world);
        e.mem.set_u32(job + 0x20, table);
        (owner, job, world, table, log)
    }

    #[test]
    fn results_are_folded_into_the_world_under_the_lock() {
        let mut e = engine();
        let (owner, job, world, table, log) =
            setup(&mut e, &[([7, 1, 2, 3], 0x40), ([1, 1, 1, 1], 0)]);
        let (stream, records) = monitor(&mut e, 0x100);
        let first_block = e.mem.u32(table + 0xc);
        let returned = e.call(0x00d9_31a0, &args![owner, 0u32, job]).u32();
        assert_eq!(returned, 2);
        let totals: Vec<u32> = [8, 0xc, 0x10, 0x14]
            .iter()
            .map(|offset| e.mem.u32(world + offset))
            .collect();
        assert_eq!(totals, vec![7, 12, 23, 34]);
        // The order of the calls: lock, the commands of the first block, free, unlock.
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![LOCK_ISLAND, RUN_COMMANDS, FREE_RESULTS, UNLOCK_ISLAND]
        );
        assert_eq!(calls(&log, LOCK_ISLAND), vec![vec![0x1234, world]]);
        assert_eq!(
            calls(&log, RUN_COMMANDS),
            vec![vec![0x1234, first_block + 0x18, 0x40]]
        );
        assert_eq!(calls(&log, FREE_RESULTS), vec![vec![table]]);
        assert_eq!(calls(&log, UNLOCK_ISLAND), vec![vec![0x1234, world]]);
        // The records: the 16-byte start record, `MinumCmds` with 0x40 / 16 and the end.
        assert_eq!(e.mem.u32(records), LABEL_LT_NARROW_PHASE);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_ST_POST_COLLIDE);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_MINUM_CMDS);
        assert_eq!(e.mem.f32(records + 0x14), 4.0);
        assert_eq!(e.mem.u32(records + 0x18), LABEL_JOB_END);
        assert_eq!(e.mem.u32(stream + 4), records + 0x24);
    }

    #[test]
    fn a_nonzero_second_word_finishes_the_job() {
        let mut e = engine();
        let (owner, job, _, _, log) = setup(&mut e, &[]);
        monitor(&mut e, 0x100);
        let returned = e.call(0x00d9_31a0, &args![owner, 0x99u32, job]).u32();
        assert_eq!(returned, 0x55);
        assert_eq!(calls(&log, JOB_DONE), vec![vec![0x99, job, job, 0]]);
        // No blocks: no commands, but the table is still freed and the world unlocked.
        assert!(calls(&log, RUN_COMMANDS).is_empty());
        assert_eq!(calls(&log, FREE_RESULTS).len(), 1);
        assert_eq!(calls(&log, UNLOCK_ISLAND).len(), 1);
    }
}
