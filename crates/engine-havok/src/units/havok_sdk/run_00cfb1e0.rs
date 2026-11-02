//! `Havok SDK/run_00cfb1e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of a collision agent table (the class is unconfirmed): `this + 0xc` starts a table of
//! 8 x 8 pointers to reference-counted objects, entry `(i, j)` at `this + 0xc + (i + j * 8) *
//! 4`, which the exe fills with a reference counted object for every pair of the eight types.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c90510` (`this` only): adds a reference (under the reference-count lock).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `00c905b0` (`this` only): removes a reference.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// Replaces the table entry at `slot` by `object`: a reference is added to `object` first, then
/// one removed from the old entry (read from the slot after the add), then the slot is stored.
fn replace_entry(e: &mut Engine, slot: u32, object: u32) {
    e.call(ADD_REFERENCE, &args![object]);
    let old = e.mem.u32(slot);
    e.call(REMOVE_REFERENCE, &args![old]);
    e.mem.set_u32(slot, object);
}

// Translated from 00cfb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `+0x1e23`.
pub fn fn_00cfb1e0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1e23, value);
}

// Translated from 00cfb1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the table entries `(first, second)` and `(second, first)` (entry `(i, j)` at `this + 0xc +
/// (i + j * 8) * 4`) to `object`, each through `replace_entry` (the second entry is read after
/// the first was stored, so for `first == second` the object itself is the old entry).
pub fn fn_00cfb1f0(e: &mut Engine, this: Ptr, object: u32, first: u32, second: u32) {
    let table = this.addr() + 0xc;
    let slot = table.wrapping_add(first.wrapping_add(second.wrapping_mul(8)).wrapping_mul(4));
    replace_entry(e, slot, object);
    let slot = table.wrapping_add(second.wrapping_add(first.wrapping_mul(8)).wrapping_mul(4));
    replace_entry(e, slot, object);
}

// Translated from 00cfb240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets all eight entries `(index, k)` and all eight `(k, index)` of the table to `object`: for
/// `k` from 0 to 7, the entry at `this + 0xc + (index + k * 8) * 4` and then the one at `this +
/// 0xc + (index * 8 + k) * 4`, each through `replace_entry`.
pub fn fn_00cfb240(e: &mut Engine, this: Ptr, object: u32, index: u32) {
    let table = this.addr() + 0xc;
    let mut column = table.wrapping_add(index.wrapping_mul(4));
    let mut row = table.wrapping_add(index.wrapping_mul(0x20));
    for _ in 0..8 {
        replace_entry(e, column, object);
        replace_entry(e, row, object);
        row = row.wrapping_add(4);
        column = column.wrapping_add(0x20);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_b1e0, fn_00cfb1e0(Ptr, u8)),
        entry!(0x00cf_b1f0, fn_00cfb1f0(Ptr, u32, u32, u32)),
        entry!(0x00cf_b240, fn_00cfb240(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(&'static str, u32)>>>;

    /// A table object with 64 distinct old entries (`0x1000 + slot number`) and doubles for the
    /// two reference functions.
    fn rig() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(ADD_REFERENCE, move |_, a| {
            sink.borrow_mut().push(("add", a[0]));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().push(("remove", a[0]));
            Ret::default()
        });
        let this = e.mem.alloc(0x1e30);
        for slot in 0..64 {
            e.mem.set_u32(this + 0xc + 4 * slot, 0x1000 + slot);
        }
        (e, log, this)
    }

    fn entry(e: &Engine, this: u32, i: u32, j: u32) -> u32 {
        e.mem.u32(this + 0xc + 4 * (i + 8 * j))
    }

    #[test]
    fn byte_is_stored_at_the_fixed_offset() {
        let (mut e, _, this) = rig();
        e.call(0x00cf_b1e0, &args![this, 0x1ffu32]);
        assert_eq!(e.mem.u8(this + 0x1e23), 0xff);
    }

    /// Both mirrored entries get the object, each after an add and before a remove of the
    /// old one; a diagonal entry makes the object its own old entry for the second change.
    #[test]
    fn two_mirrored_entries_are_replaced_with_reference_counting() {
        let (mut e, log, this) = rig();
        e.call(0x00cf_b1f0, &args![this, 0x77u32, 2u32, 5u32]);
        assert_eq!(entry(&e, this, 2, 5), 0x77);
        assert_eq!(entry(&e, this, 5, 2), 0x77);
        assert_eq!(
            *log.borrow(),
            vec![
                ("add", 0x77),
                ("remove", 0x1000 + 2 + 5 * 8),
                ("add", 0x77),
                ("remove", 0x1000 + 5 + 2 * 8)
            ]
        );
        log.borrow_mut().clear();
        e.call(0x00cf_b1f0, &args![this, 0x88u32, 3u32, 3u32]);
        assert_eq!(entry(&e, this, 3, 3), 0x88);
        assert_eq!(
            *log.borrow(),
            vec![
                ("add", 0x88),
                ("remove", 0x1000 + 3 + 3 * 8),
                ("add", 0x88),
                ("remove", 0x88)
            ]
        );
    }

    /// A whole row and column are replaced, alternating column entry and row entry.
    #[test]
    fn a_row_and_a_column_are_replaced() {
        let (mut e, log, this) = rig();
        e.call(0x00cf_b240, &args![this, 0x99u32, 2u32]);
        for k in 0..8 {
            assert_eq!(entry(&e, this, 2, k), 0x99);
            assert_eq!(entry(&e, this, k, 2), 0x99);
        }
        assert_eq!(entry(&e, this, 1, 1), 0x1000 + 9);
        let log = log.borrow();
        assert_eq!(log.len(), 32);
        // The first pair of steps: column entry 0 (slot 2), then row entry 0 (slot 16).
        assert_eq!(log[0], ("add", 0x99));
        assert_eq!(log[1], ("remove", 0x1000 + 2));
        assert_eq!(log[3], ("remove", 0x1000 + 16));
        // The next pair: column entry 1 (slot 10), row entry 1 (slot 17).
        assert_eq!(log[5], ("remove", 0x1000 + 10));
        assert_eq!(log[7], ("remove", 0x1000 + 17));
    }
}
