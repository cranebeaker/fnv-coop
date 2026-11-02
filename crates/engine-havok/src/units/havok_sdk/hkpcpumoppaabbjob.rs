//! `Havok SDK/hkpcpumoppaabbjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// The memory router (`00c85750`); its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpMoppObbVirtualMachine::queryAabb` (Ghidra name; `ECX` = the machine object, three stack
/// words: the query box block, the MOPP code entry and the array that receives the keys).
const QUERY_AABB: u32 = 0x00dd_1da0;
/// Unnamed (cdecl, three words: the allocator, an array and the element size): makes room for
/// more elements in the array (the hkArray growth routine).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// Profiling label `"TtCollQueryMoppAabbQuery"`.
const LABEL_TT_MOPP_AABB_QUERY: u32 = 0x010d_b998;
/// The capacity bits of a capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The capacity word of an array without storage.
const EMPTY_CAPACITY: u32 = 0x8000_0000;

// Translated from 00df7010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuMoppAabbQueryJob` (Xbox PDB name per the engine map): cdecl, two stack words: a word
/// that becomes `ECX` of the final call `00d04e80`, and the job. Writes `TtCollQueryMoppAabbQuery`
/// and builds on its stack the query block (the 16 bytes at `job + 0x20` at `+0x10`, the word at
/// `job + 0x30` at `+0x20`, 0x200 at `+0x24` and `+0x28`, the byte 2 at `+0x2C`; the first 16
/// bytes of the block and the machine object, which the exe leaves unwritten, are zero here).
/// For each of the `[job + 0x3C]` 0x30-byte entries at `[job + 0x38]` it starts an array of
/// 4-byte keys over the buffer at `entry + 0x20` (size 0, capacity word `0x80001000`, not
/// owned), runs `00dd1da0` with the query block, the entry and the array, appends the key
/// `0xFFFFFFFF` (growing the array with `00c90940` first when its size equals its capacity),
/// then clears the size and frees the array's data (allocator slot `0x10`: data, 4 bytes per
/// capacity) unless the capacity word says it is not owned. Ends with `00d04e80` and returns
/// what it returns.
pub fn hk_cpu_mopp_aabb_query_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_TT_MOPP_AABB_QUERY);
    let job = job.addr();
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        // The query block, the array (data, size, capacity word) and the machine object.
        let block = f + 0x20;
        let array = f + 0x14;
        let machine = f + 0x60;
        for word in 0..4 {
            let value = e.mem.u32(job + 0x20 + 4 * word);
            e.mem.set_u32(block + 0x10 + 4 * word, value);
        }
        let limit = e.mem.u32(job + 0x30);
        e.mem.set_u32(block + 0x20, limit);
        e.mem.set_u32(block + 0x24, 0x200);
        e.mem.set_u32(block + 0x28, 0x200);
        e.mem.set_u8(block + 0x2c, 2);
        let mut index = 0;
        while index < e.mem.i32(job + 0x3c) {
            let entry = e.mem.u32(job + 0x38).wrapping_add(0x30 * index as u32);
            let keys = e.mem.u32(entry + 0x20);
            e.mem.set_u32(array, keys);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_1000);
            e.call(QUERY_AABB, &args![machine, block, entry, array]);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let size = e.mem.u32(array + 4);
            if size == e.mem.u32(array + 8) & CAPACITY_MASK {
                e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
            }
            let slot = e
                .mem
                .u32(array)
                .wrapping_add(e.mem.u32(array + 4).wrapping_mul(4));
            if slot != 0 {
                e.mem.set_u32(slot, 0xffff_ffff);
            }
            let grown = e.mem.u32(array + 4).wrapping_add(1);
            e.mem.set_u32(array + 4, grown);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = e.mem.u32(array + 8);
            e.mem.set_u32(array + 4, 0);
            if capacity as i32 >= 0 {
                let data = e.mem.u32(array);
                e.vcall(
                    allocator,
                    0x10,
                    &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
                );
            }
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 8, EMPTY_CAPACITY);
            index += 1;
        }
    });
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00df7010,
        hk_cpu_mopp_aabb_query_job(u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, logging_object};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::job_rig;

    const ALLOCATOR: u32 = 0x0900_0000;

    /// The entries of the job, each with its own key buffer; the job has the box words
    /// 1..=4 at `+0x20` and 9 at `+0x30`.
    fn job(e: &mut Engine, entries: u32, count: u32) -> u32 {
        let job = e.mem.alloc(0x40);
        for word in 0..4 {
            e.mem.set_u32(job + 0x20 + 4 * word, 1 + word);
        }
        e.mem.set_u32(job + 0x30, 9);
        e.mem.set_u32(job + 0x38, entries);
        e.mem.set_u32(job + 0x3c, count);
        job
    }

    #[test]
    fn collects_the_keys_of_each_entry_and_appends_the_terminator() {
        let (mut e, rig) = job_rig();
        let allocator = logging_object(&mut e, &rig.log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &rig.log, ARRAY_GROW, 0);
        let entries = e.mem.alloc(0x60);
        let buffers = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for (index, buffer) in buffers.iter().enumerate() {
            e.mem.set_u32(entries + 0x30 * index as u32 + 0x20, *buffer);
        }
        // The query adds one key to the array of the first entry and none to the second.
        let sink = rig.log.clone();
        e.register_double(QUERY_AABB, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|word| e.mem.u32(a[1] + 0x10 + 4 * word)));
            words.extend([
                e.mem.u32(a[1] + 0x20),
                e.mem.u32(a[1] + 0x24),
                e.mem.u32(a[1] + 0x28),
            ]);
            words.push(u32::from(e.mem.u8(a[1] + 0x2c)));
            words.extend([e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8)]);
            let first = sink.borrow().is_empty();
            sink.borrow_mut().push((QUERY_AABB, words));
            if first {
                let data = e.mem.u32(a[3]);
                e.mem.set_u32(data, 0x77);
                e.mem.set_u32(a[3] + 4, 1);
            }
            Ret::default()
        });
        let job = job(&mut e, entries, 2);
        let returned = e.call(0x00df_7010, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let queries = calls(&rig.log, QUERY_AABB);
        assert_eq!(queries.len(), 2);
        // The machine object, the query block, the entry and the array; then the block's
        // 16 bytes, limit, the two 0x200 words and the byte 2, and the array as it was passed.
        assert_eq!(queries[0][2], entries);
        assert_eq!(queries[1][2], entries + 0x30);
        assert_eq!(
            queries[0][4..],
            [1, 2, 3, 4, 9, 0x200, 0x200, 2, buffers[0], 0, 0x8000_1000]
        );
        assert_eq!(queries[1][12..], [buffers[1], 0, 0x8000_1000]);
        // The terminator goes after the key the query added; the buffers are not owned.
        assert_eq!(e.mem.u32(buffers[0]), 0x77);
        assert_eq!(e.mem.u32(buffers[0] + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(buffers[1]), 0xffff_ffff);
        assert!(calls(&rig.log, ARRAY_GROW).is_empty());
        assert!(calls(&rig.log, ALLOCATOR + 0x10).is_empty());
        assert_eq!(
            rig.labels(&e, false),
            vec![LABEL_TT_MOPP_AABB_QUERY, PROFILE_END]
        );
    }

    #[test]
    fn a_full_array_grows_and_its_new_storage_is_freed() {
        let (mut e, rig) = job_rig();
        let allocator = logging_object(&mut e, &rig.log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let entries = e.mem.alloc(0x30);
        let buffer = e.mem.alloc(0x4010);
        e.mem.set_u32(entries + 0x20, buffer);
        let grown = e.mem.alloc(0x8010);
        // The query fills the 0x1000-entry buffer; growing moves the keys to owned storage with
        // room for 0x2000.
        e.register_double(QUERY_AABB, |e, a| {
            e.mem.set_u32(a[3] + 4, 0x1000);
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(ARRAY_GROW, move |e, a| {
            sink.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 0x2000);
            Ret::default()
        });
        let job = job(&mut e, entries, 1);
        let returned = e.call(0x00df_7010, &args![0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert_eq!(calls(&rig.log, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&rig.log, ARRAY_GROW)[0][0], allocator);
        assert_eq!(calls(&rig.log, ARRAY_GROW)[0][2], 4);
        assert_eq!(e.mem.u32(grown + 0x4000), 0xffff_ffff);
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![allocator, grown, 0x8000]]
        );
    }
}
