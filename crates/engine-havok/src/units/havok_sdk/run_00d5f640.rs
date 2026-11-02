//! `Havok SDK/run_00d5f640` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The key that marks an empty slot of the map.
const EMPTY_KEY: u32 = 0xffff_ffff;
/// `00d5f480`, the insert the resize calls for every entry (thiscall, five stack words).
const MAP_INSERT: u32 = 0x00d5_f480;

// Translated from 00d5f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the hash map with 16-byte slots (thiscall, `RET 4`: one stack word `allocator`,
/// an object with a vtable; no map name): runs the base destructor `00dc3980` on `this` and, when
/// the top bit of the count word at `+4` is clear, frees the table at `+0` through slot 8 of
/// `allocator` with `(mask + 1) * 16` bytes (`mask` is the word at `+8`).
pub fn fn_00d5f640(e: &mut Engine, this: Ptr, allocator: u32) {
    let me = this.addr();
    e.call(0x00dc_3980, &args![this]);
    if e.mem.u32(me + 4) & 0x8000_0000 == 0 {
        let mask = e.mem.u32(me + 8);
        let table = e.mem.u32(me);
        e.vcall(allocator, 8, &args![table, mask.wrapping_add(1) << 4]);
    }
}

// Translated from 00d5f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resize of the hash map with 16-byte slots (thiscall, `RET 8`: two stack words `allocator`, an
/// object with a vtable, and `wanted`; no map name): makes a new table of `max(wanted, 8)` slots
/// through slot 4 of `allocator` (`16 * slots` bytes), sets every key to `0xffffffff`, the entry
/// count to 0 and the mask to `slots - 1`; every old slot that is not empty is inserted again with
/// `00d5f480(allocator, key, +4, +8, +0xc)`; and when the top bit of the old count word was clear
/// the old table is freed through slot 8 of `allocator` with `16 * (old mask + 1)` bytes.
pub fn fn_00d5f670(e: &mut Engine, this: Ptr, allocator: u32, wanted: i32) {
    let me = this.addr();
    let slots = if wanted > 8 { wanted } else { 8 };
    let owned = e.mem.u32(me + 4) & 0x8000_0000;
    let old_table = e.mem.u32(me);
    let old_mask = e.mem.u32(me + 8);
    let old_slots = old_mask.wrapping_add(1);
    let table = e.vcall(allocator, 4, &args![(slots as u32) << 4]).u32();
    e.mem.set_u32(me, table);
    if slots > 0 {
        for slot in 0..slots as u32 {
            let table = e.mem.u32(me);
            e.mem.set_u32(table.wrapping_add(slot * 16), EMPTY_KEY);
        }
    }
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, (slots as u32).wrapping_sub(1));
    if (old_slots as i32) > 0 {
        let mut slot = old_table;
        for _ in 0..old_slots {
            let key = e.mem.u32(slot);
            if key != EMPTY_KEY {
                let first = e.mem.u32(slot + 4);
                let second = e.mem.u32(slot + 8);
                let third = e.mem.u32(slot + 0xc);
                e.call(
                    MAP_INSERT,
                    &args![this, allocator, key, first, second, third],
                );
            }
            slot = slot.wrapping_add(0x10);
        }
    }
    if owned == 0 {
        e.vcall(allocator, 8, &args![old_table, old_slots << 4]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_f640, fn_00d5f640(Ptr, u32)),
        entry!(0x00d5_f670, fn_00d5f670(Ptr, u32, i32)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An allocator whose slot 4 allocates (and logs) and slot 8 frees (and logs).
    fn allocator(e: &mut Engine) -> (u32, Log) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, 0x0900_0141);
        e.mem.set_u32(vtable + 8, 0x0900_0142);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let allocations = Rc::clone(&log);
        e.register_double(0x0900_0141, move |e, a| {
            allocations.borrow_mut().push((0x0900_0141, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let frees = Rc::clone(&log);
        e.register_double(0x0900_0142, move |_, a| {
            frees.borrow_mut().push((0x0900_0142, a.to_vec()));
            Ret::default()
        });
        (allocator, log)
    }

    #[test]
    fn fn_00d5f640_frees_the_table_when_it_owns_it() {
        let mut e = Engine::new();
        let (allocator, log) = allocator(&mut e);
        e.register(0x00dc_3980, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x7000);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u32(this + 8, 7);
        e.call(0x00d5_f640, &args![this, allocator]);
        assert_eq!(
            *log.borrow(),
            vec![(0x0900_0142, vec![allocator, 0x7000, 128])]
        );
        // With the top bit set the table is not freed.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 4, 0x8000_0005);
        e.call(0x00d5_f640, &args![this, allocator]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d5f670_rebuilds_the_table_and_inserts_the_old_entries() {
        for (owned, wanted) in [(true, 4i32), (false, 40)] {
            let mut e = Engine::new();
            let (allocator, log) = allocator(&mut e);
            let inserts = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let insert_log = Rc::clone(&inserts);
            e.register_double(MAP_INSERT, move |_, a| {
                insert_log.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            let this = e.mem.alloc(0x10);
            let old = e.mem.alloc(4 * 16);
            for slot in 0..4 {
                e.mem.set_u32(old + slot * 16, EMPTY_KEY);
            }
            for (slot, key) in [(0u32, 0x10u32), (2, 0x20)] {
                e.mem.set_u32(old + slot * 16, key);
                for word in 1..4 {
                    e.mem.set_u32(old + slot * 16 + word * 4, key + word);
                }
            }
            e.mem.set_u32(this, old);
            e.mem.set_u32(this + 4, if owned { 2 } else { 0x8000_0002 });
            e.mem.set_u32(this + 8, 3);
            e.call(0x00d5_f670, &args![this, allocator, wanted as u32]);
            let slots = wanted.max(8) as u32;
            // The allocation, the new table and its state.
            assert_eq!(log.borrow()[0], (0x0900_0141, vec![allocator, slots * 16]));
            let table = e.mem.u32(this);
            assert_ne!(table, old);
            for slot in 0..slots {
                assert_eq!(e.mem.u32(table + slot * 16), EMPTY_KEY);
            }
            assert_eq!(e.mem.u32(this + 4), 0);
            assert_eq!(e.mem.u32(this + 8), slots - 1);
            // The two used old entries are inserted again, in slot order.
            assert_eq!(
                *inserts.borrow(),
                vec![
                    vec![this, allocator, 0x10, 0x11, 0x12, 0x13],
                    vec![this, allocator, 0x20, 0x21, 0x22, 0x23],
                ]
            );
            // The old table is freed only when it was owned.
            let frees: Vec<_> = log
                .borrow()
                .iter()
                .filter(|(address, _)| *address == 0x0900_0142)
                .cloned()
                .collect();
            if owned {
                assert_eq!(frees, vec![(0x0900_0142, vec![allocator, old, 64])]);
            } else {
                assert!(frees.is_empty());
            }
        }
    }
}
