//! `Havok SDK/hkpcpufirejacobiansetupcallbackjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `hkSimpleContactConstraintData_fireCallbacks` (Xbox PDB name per the engine map, `00d92900`):
/// cdecl `(object, input, atoms, 2)`.
const FIRE_CALLBACKS: u32 = 0x00d9_2900;
/// The memory router (`00c85750`); its `+0x10` is the allocator whose slot `8` frees a block
/// (`(block, bytes)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Profiling label `"StConstraintCallbacks"`.
const LABEL_ST_CONSTRAINT_CALLBACKS: u32 = 0x010d_3edc;

// Translated from 00d94c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSingleThreadedJobsOnIsland::cpuFireJacobianSetupCallbackJob` (Xbox PDB name per the
/// engine map): cdecl, three stack words: the owner, a word that becomes `ECX` of the final
/// call `00d04e80`, and the job. Copies the 0x80 bytes at `owner + 0x10` to a local input. When
/// the set (`[job + 0x18]`) has a callback list (`+0x70`, count `+0x74`) it writes the 16-byte
/// start record (`LtIntegrate`, `StConstraintCallbacks`) and goes through the list, whose
/// entries are 8 bytes (an item, a record). For each item it fills the input: `+0x60` from the
/// item's first word and `+0x64` from its word at `+0x1C`; `+0x30` and `+0x34` from the
/// words at `+0xA4` of the item's words at `+4` and `+8` plus the set's word at `+0x30`;
/// `+0x38` and `+0x3C` from words at `+0x18` of the objects at `+0x14` and `+0x18` of the first
/// word's target. If the byte at `item + 0x12` has bit 0 or 3 set it walks from the item's word
/// at `+0xC` along the words at `+0x14` while the 16-bit type is at least 0x17, calls
/// `hkSimpleContactConstraintData_fireCallbacks` and, when the target's word at `+0x10`
/// changed, stores the new word in the record's first word and its 16-bit word at `+0x10` at the
/// record's `+0x14`. If bit 2 is set it calls the virtual function at `+0x38` of the target's
/// object at `+0xC` with the input. The list is then freed through the memory router's allocator
/// (slot 8) and cleared, and the closing record written. Ends with `00d04e80` either way.
/// Returns what that returns.
pub fn hk_cpu_fire_jacobian_setup_callback_job(
    e: &mut Engine,
    owner: Ptr,
    second: u32,
    job: Ptr,
) -> u32 {
    let owner = owner.addr();
    let job = job.addr();
    let set = e.mem.u32(job + 0x18);
    e.with_stack(0x80, |e, input| {
        let input = input.addr();
        for word in 0..0x20 {
            let value = e.mem.u32(owner + 0x10 + 4 * word);
            e.mem.set_u32(input + 4 * word, value);
        }
        if e.mem.i32(set + 0x74) > 0 {
            profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_CONSTRAINT_CALLBACKS);
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(set + 0x74) {
                let list = e.mem.u32(set + 0x70);
                let item = e.mem.u32(list.wrapping_add(index.wrapping_mul(8)));
                let target = e.mem.u32(item);
                let word = e.mem.u32(item + 0x1c);
                e.mem.set_u32(input + 0x60, target);
                e.mem.set_u32(input + 0x64, word);
                for (field, source) in [(0x30, item + 4), (0x34, item + 8)] {
                    let object = e.mem.u32(source);
                    let sum = e.mem.u32(object + 0xa4).wrapping_add(e.mem.u32(set + 0x30));
                    e.mem.set_u32(input + field, sum);
                }
                for (field, source) in [(0x38, 0x14), (0x3c, 0x18)] {
                    let object = e.mem.u32(e.mem.u32(item) + source);
                    let word = e.mem.u32(object + 0x18);
                    e.mem.set_u32(input + field, word);
                }
                let flags = e.mem.u8(item + 0x12);
                if flags & 9 != 0 {
                    let target = e.mem.u32(item);
                    let object = e.mem.u32(target + 0xc);
                    let before = e.mem.u32(target + 0x10);
                    let mut atoms = e.mem.u32(item + 0xc);
                    while e.mem.u16(atoms) >= 0x17 {
                        atoms = e.mem.u32(atoms + 0x14);
                    }
                    e.call(FIRE_CALLBACKS, &args![object, input, atoms, 2u32]);
                    let after = e.mem.u32(e.mem.u32(item) + 0x10);
                    if before != after {
                        let list = e.mem.u32(set + 0x70);
                        let record = e.mem.u32(list.wrapping_add(index.wrapping_mul(8) + 4));
                        e.mem.set_u32(record, after);
                        let word = e.mem.u16(after + 0x10);
                        e.mem.set_u16(record + 0x14, word);
                    }
                }
                if flags & 4 != 0 {
                    let object = e.mem.u32(e.mem.u32(item) + 0xc);
                    e.vcall(object, 0x38, &args![input]);
                }
                index += 1;
            }
            let count = e.mem.u32(set + 0x74);
            let list = e.mem.u32(set + 0x70);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![list, count.wrapping_mul(8)]);
            e.mem.set_u32(set + 0x70, 0);
            e.mem.set_u32(set + 0x74, 0);
            profile_record(e, LABEL_JOB_END);
        }
    });
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d94c40,
        hk_cpu_fire_jacobian_setup_callback_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, logging_object};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::job_rig;

    const ALLOCATOR: u32 = 0x0900_0000;
    const CALLBACKS: u32 = 0x0910_0000;

    /// A list item with the given flag byte whose constraint has the callback object, two
    /// entities and the atoms chain `head` (type 0x20) then `tail` (type 0x16); the item's
    /// bodies have the words 0x10 and 0x20 at `+0xA4`, and its record is separate.
    struct Item {
        item: u32,
        record: u32,
        target: u32,
        tail: u32,
    }

    fn item(e: &mut Engine, flags: u8, callbacks: u32) -> Item {
        let record = e.mem.alloc(0x20);
        let item = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x20);
        let entity_a = e.mem.alloc(0x20);
        let entity_b = e.mem.alloc(0x20);
        e.mem.set_u32(entity_a + 0x18, 0xa1);
        e.mem.set_u32(entity_b + 0x18, 0xb1);
        e.mem.set_u32(target + 0x14, entity_a);
        e.mem.set_u32(target + 0x18, entity_b);
        e.mem.set_u32(target + 0xc, callbacks);
        e.mem.set_u32(target + 0x10, 0x3333);
        let body_a = e.mem.alloc(0xc0);
        let body_b = e.mem.alloc(0xc0);
        e.mem.set_u32(body_a + 0xa4, 0x10);
        e.mem.set_u32(body_b + 0xa4, 0x20);
        let tail = e.mem.alloc(0x20);
        e.mem.set_u16(tail, 0x16);
        let head = e.mem.alloc(0x20);
        e.mem.set_u16(head, 0x20);
        e.mem.set_u32(head + 0x14, tail);
        e.mem.set_u32(item, target);
        e.mem.set_u32(item + 4, body_a);
        e.mem.set_u32(item + 8, body_b);
        e.mem.set_u32(item + 0xc, head);
        e.mem.set_u8(item + 0x12, flags);
        e.mem.set_u32(item + 0x1c, 0x77);
        Item {
            item,
            record,
            target,
            tail,
        }
    }

    #[test]
    fn without_a_callback_list_it_just_finishes() {
        let (mut e, rig) = job_rig();
        let owner = e.mem.alloc(0x100);
        let set = e.mem.alloc(0x80);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, set);
        let returned = e.call(0x00d9_4c40, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        assert!(rig.labels(&e, false).is_empty());
    }

    #[test]
    fn fires_the_callbacks_of_each_item_and_frees_the_list() {
        let (mut e, rig) = job_rig();
        let allocator = logging_object(&mut e, &rig.log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let callbacks = logging_object(&mut e, &rig.log, CALLBACKS);
        // Item 0 fires the contact callbacks, item 1 only calls the virtual function.
        let first = item(&mut e, 1, callbacks);
        let second = item(&mut e, 4, callbacks);
        // The firing changes the constraint's word at +0x10 to another body with the 16-bit
        // word 0x4d.
        let replacement = e.mem.alloc(0x20);
        e.mem.set_u16(replacement + 0x10, 0x4d);
        let sink = rig.log.clone();
        let changed = first.target;
        e.register_double(FIRE_CALLBACKS, move |e, a| {
            let mut words = a.to_vec();
            // The input as the call sees it: +0x30, +0x34, +0x38, +0x3c, +0x60, +0x64, +0x70.
            for offset in [0x30, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x70] {
                words.push(e.mem.u32(a[1] + offset));
            }
            sink.borrow_mut().push((FIRE_CALLBACKS, words));
            e.mem.set_u32(changed + 0x10, replacement);
            Ret::default()
        });
        let owner = e.mem.alloc(0x100);
        for word in 0..0x20 {
            e.mem.set_u32(owner + 0x10 + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(owner + 0x10 + 0x70, 0x7070);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list, first.item);
        e.mem.set_u32(list + 4, first.record);
        e.mem.set_u32(list + 8, second.item);
        e.mem.set_u32(list + 0xc, second.record);
        let set = e.mem.alloc(0x80);
        e.mem.set_u32(set + 0x30, 0x1000);
        e.mem.set_u32(set + 0x70, list);
        e.mem.set_u32(set + 0x74, 2);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, set);
        let returned = e.call(0x00d9_4c40, &args![owner, 0x99u32, job]).u32();
        rig.assert_finished(returned, 0x99, job);
        let fired = calls(&rig.log, FIRE_CALLBACKS);
        assert_eq!(fired.len(), 1);
        let input = fired[0][1];
        assert_eq!(fired[0][..4], [callbacks, input, first.tail, 2]);
        // The input fields built for the first item, and the copy of the owner's.
        assert_eq!(
            fired[0][4..],
            [0x1010, 0x1020, 0xa1, 0xb1, first.target, 0x77, 0x7070]
        );
        // The changed word goes to the record, with its 16-bit word.
        assert_eq!(e.mem.u32(first.record), replacement);
        assert_eq!(e.mem.u16(first.record + 0x14), 0x4d);
        assert_eq!(e.mem.u32(second.record), 0);
        assert_eq!(
            calls(&rig.log, CALLBACKS + 0x38),
            vec![vec![callbacks, input]]
        );
        // The list is freed (two entries of 8 bytes) and cleared.
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 8),
            vec![vec![allocator, list, 16]]
        );
        assert_eq!(e.mem.u32(set + 0x70), 0);
        assert_eq!(e.mem.u32(set + 0x74), 0);
        assert_eq!(
            rig.labels(&e, true),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_CONSTRAINT_CALLBACKS,
                LABEL_JOB_END
            ]
        );
    }
}
