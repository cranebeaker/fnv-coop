//! `BSHavok/run_00cbd700` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A bounded collection of 0x70-byte records kept by the largest float at `+0x1c`: when the
//! collection is full, a new record replaces the record with the largest value if its own value
//! is smaller, and the largest value is published at `+4` of the collection. The exe data
//! names none of the three functions (the Xbox PDB calls them `range`); `0x01268430` holds the
//! address of the second one, which Ghidra's `addCdPoint` (`009afca0`) calls through that word
//! with the new record and its own `this`. What the records are is not confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00cb9810::chain_end;

/// Size of one record.
const RECORD_SIZE: u32 = 0x70;
/// The word `fn_00cbd960` fills with the address of `fn_00cbd7a0`.
const CALLBACK_WORD: u32 = 0x0126_8430;

// Translated from 00cbd700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cbd700` (unnamed in the Xbox PDB): copies a record. The exe takes the arguments in
/// registers (`EAX` = `source`, `EDX` = `destination`) and clobbers `ECX`; it is registered
/// here with those two words in that order. Copies the 0x20 bytes at the start, then: the last
/// link of the chain of words at `+0xC` starting from the word at `source + 0x20` goes to
/// `destination + 0x20`, and the one for `source + 0x24` to `destination + 0x48`; the word at
/// `+4` of the first links go to `destination + 0x24` and `+0x4c`. Then the path from the
/// `source + 0x24` link is stored as words from `destination + 0x50`: with `n` the number of
/// links after the first one, the word at index `n` is -1 and the word at index `k` is the
/// `+4` word of link `n - 1 - k`.
pub fn fn_00cbd700(e: &mut Engine, source: u32, destination: u32) {
    for word in 0..8 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
    let first = e.mem.u32(source + 0x20);
    let second = e.mem.u32(source + 0x24);
    let first_end = chain_end(e, first);
    e.mem.set_u32(destination + 0x20, first_end);
    let second_end = chain_end(e, second);
    e.mem.set_u32(destination + 0x48, second_end);
    let first_value = e.mem.u32(first + 4);
    e.mem.set_u32(destination + 0x24, first_value);
    let second_value = e.mem.u32(second + 4);
    e.mem.set_u32(destination + 0x4c, second_value);
    let mut length = 0u32;
    let mut link = e.mem.u32(second + 0xc);
    while link != 0 {
        link = e.mem.u32(link + 0xc);
        length += 1;
    }
    e.mem.set_u32(destination + 0x50 + length * 4, 0xffff_ffff);
    let mut node = second;
    for index in (0..length).rev() {
        let value = e.mem.u32(node + 4);
        e.mem.set_u32(destination + 0x50 + index * 4, value);
        node = e.mem.u32(node + 0xc);
    }
}

/// The record from `base` (inclusive) to `end` (exclusive) whose float at `+0x1c` is the
/// largest; a later record wins only when the best so far is smaller than it (a NaN never
/// wins). Records are scanned in order, as the exe's four-at-a-time loop and its tail do.
fn largest_record(e: &Engine, base: u32, end: u32) -> u32 {
    let mut best = base;
    let mut at = base.wrapping_add(RECORD_SIZE);
    while at < end {
        if e.mem.f32(best + 0x1c) < e.mem.f32(at + 0x1c) {
            best = at;
        }
        at = at.wrapping_add(RECORD_SIZE);
    }
    best
}

// Translated from 00cbd7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cbd7a0` (unnamed in the Xbox PDB; cdecl, two words): takes a record `item` and the
/// collection `holder` (the limit at `+0x10`, the count at `+0x14`, the next free record at
/// `+0xC`, the records from `+8`). Does nothing when the limit is 0. When the count (signed)
/// is below the limit, copies the record to the next free slot (`fn_00cbd700`), counts it and
/// moves the free pointer on by 0x70. Otherwise finds the stored record with the largest float
/// at `+0x1c` ([`largest_record`]) and, when that float is greater than the one of `item`,
/// replaces that record by `item` and finds the largest again. In the full case it ends by
/// storing the largest float at `+4` of the collection (through the x87 stack).
pub fn fn_00cbd7a0(e: &mut Engine, item: u32, holder: u32) {
    let limit = e.mem.i32(holder + 0x10);
    if limit == 0 {
        return;
    }
    let count = e.mem.i32(holder + 0x14);
    if count < limit {
        let slot = e.mem.u32(holder + 0xc);
        fn_00cbd700(e, item, slot);
        let counted = e.mem.u32(holder + 0x14).wrapping_add(1);
        e.mem.set_u32(holder + 0x14, counted);
        let next = e.mem.u32(holder + 0xc).wrapping_add(RECORD_SIZE);
        e.mem.set_u32(holder + 0xc, next);
        return;
    }
    let base = e.mem.u32(holder + 8);
    let end = base.wrapping_add((count as u32).wrapping_mul(RECORD_SIZE));
    let mut best = largest_record(e, base, end);
    if e.mem.f32(best + 0x1c) > e.mem.f32(item + 0x1c) {
        fn_00cbd700(e, item, best);
        let base = e.mem.u32(holder + 8);
        best = largest_record(e, base, end);
    }
    let bits = x87_quiet_bits(e.mem.u32(best + 0x1c));
    e.mem.set_u32(holder + 4, bits);
}

// Translated from 00cbd960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cbd960` (unnamed in the Xbox PDB): takes no arguments and stores the address of
/// `fn_00cbd7a0` (0x00cbd7a0) in the word at 0x01268430.
pub fn fn_00cbd960(e: &mut Engine) {
    e.set_global(CALLBACK_WORD, 0x00cb_d7a0u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbd700, fn_00cbd700(u32, u32)),
        entry!(0x00cbd7a0, fn_00cbd7a0(u32, u32)),
        entry!(0x00cbd960, fn_00cbd960()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record whose float at `+0x1c` is `value`, with two one-link chains at `+0x20` and
    /// `+0x24` whose `+4` words are `value`-derived markers.
    fn record(e: &mut Engine, value: f32) -> u32 {
        let record = e.mem.alloc(0x80);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_f32(record + 0x1c, value);
        e.mem.set_u32(record + 0x20, first);
        e.mem.set_u32(record + 0x24, second);
        record
    }

    fn values_of(e: &Engine, holder: u32) -> Vec<f32> {
        let records = e.mem.u32(holder + 8);
        (0..e.mem.u32(holder + 0x14))
            .map(|i| e.mem.f32(records + 0x70 * i + 0x1c))
            .collect()
    }

    #[test]
    fn a_record_copy_follows_the_chains() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x40);
        let a1 = e.mem.alloc(0x10);
        let a2 = e.mem.alloc(0x10);
        let b0 = e.mem.alloc(0x10);
        let b1 = e.mem.alloc(0x10);
        let b2 = e.mem.alloc(0x10);
        for (node, next, marker) in [
            (a1, a2, 11),
            (a2, 0, 12),
            (b0, b1, 21),
            (b1, b2, 22),
            (b2, 0, 23),
        ] {
            e.mem.set_u32(node + 0xc, next);
            e.mem.set_u32(node + 4, marker);
        }
        e.mem.set_u32(source, 0x1234);
        e.mem.set_u32(source + 0x1c, 0x5678);
        e.mem.set_u32(source + 0x20, a1);
        e.mem.set_u32(source + 0x24, b0);
        let destination = e.mem.alloc(0x80);
        e.call(0x00cb_d700, &args![source, destination]);
        assert_eq!(e.mem.u32(destination), 0x1234);
        assert_eq!(e.mem.u32(destination + 0x1c), 0x5678);
        assert_eq!(e.mem.u32(destination + 0x20), a2);
        assert_eq!(e.mem.u32(destination + 0x24), 11);
        assert_eq!(e.mem.u32(destination + 0x48), b2);
        assert_eq!(e.mem.u32(destination + 0x4c), 21);
        assert_eq!(e.mem.u32(destination + 0x50), 22);
        assert_eq!(e.mem.u32(destination + 0x54), 21);
        assert_eq!(e.mem.u32(destination + 0x58), 0xffff_ffff);
    }

    #[test]
    fn a_record_copy_of_single_links_ends_the_path_at_once() {
        let mut e = Engine::new();
        let source = record(&mut e, 1.0);
        let destination = e.mem.alloc(0x80);
        e.call(0x00cb_d700, &args![source, destination]);
        assert_eq!(e.mem.u32(destination + 0x50), 0xffff_ffff);
        assert_eq!(e.mem.u32(destination + 0x20), e.mem.u32(source + 0x20));
    }

    #[test]
    fn nothing_happens_without_a_limit() {
        let mut e = Engine::new();
        let holder = e.mem.alloc(0x20);
        let item = record(&mut e, 1.0);
        e.call(0x00cb_d7a0, &args![item, holder]);
        assert_eq!(e.mem.u32(holder + 0x14), 0);
        assert_eq!(e.mem.u32(holder + 0xc), 0);
    }

    #[test]
    fn records_are_appended_until_the_limit() {
        let mut e = Engine::new();
        let holder = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x70 * 3);
        e.mem.set_u32(holder + 8, records);
        e.mem.set_u32(holder + 0xc, records);
        e.mem.set_i32(holder + 0x10, 3);
        for value in [5.0, 2.0] {
            let item = record(&mut e, value);
            e.call(0x00cb_d7a0, &args![item, holder]);
        }
        assert_eq!(e.mem.u32(holder + 0x14), 2);
        assert_eq!(e.mem.u32(holder + 0xc), records + 0xe0);
        assert_eq!(values_of(&e, holder), [5.0, 2.0]);
        // The threshold word is only set once the collection is full.
        assert_eq!(e.mem.u32(holder + 4), 0);
    }

    /// Fills a collection of limit 6 (so the scan covers the four-at-a-time loop and the tail)
    /// with the given values through the free-pointer path.
    fn full_collection(e: &mut Engine, values: &[f32]) -> u32 {
        let holder = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x70 * values.len() as u32);
        e.mem.set_u32(holder + 8, records);
        e.mem.set_u32(holder + 0xc, records);
        e.mem.set_i32(holder + 0x10, values.len() as i32);
        for value in values {
            let item = record(e, *value);
            e.call(0x00cb_d7a0, &args![item, holder]);
        }
        holder
    }

    #[test]
    fn a_smaller_record_replaces_the_largest_and_the_threshold_follows() {
        let mut e = Engine::new();
        let holder = full_collection(&mut e, &[3.0, 9.0, 1.0, 7.0, 8.0, 2.0]);
        assert_eq!(e.mem.u32(holder + 0x14), 6);
        let item = record(&mut e, 4.0);
        e.call(0x00cb_d7a0, &args![item, holder]);
        assert_eq!(values_of(&e, holder), [3.0, 4.0, 1.0, 7.0, 8.0, 2.0]);
        assert_eq!(e.mem.f32(holder + 4), 8.0);
    }

    #[test]
    fn a_larger_or_equal_record_is_dropped_but_the_threshold_is_stored() {
        let mut e = Engine::new();
        let holder = full_collection(&mut e, &[3.0, 9.0, 1.0]);
        for value in [9.0, 10.0] {
            let item = record(&mut e, value);
            e.call(0x00cb_d7a0, &args![item, holder]);
            assert_eq!(values_of(&e, holder), [3.0, 9.0, 1.0]);
            assert_eq!(e.mem.f32(holder + 4), 9.0);
        }
    }

    #[test]
    fn a_nan_record_is_dropped() {
        let mut e = Engine::new();
        let holder = full_collection(&mut e, &[3.0, 9.0]);
        let item = record(&mut e, f32::NAN);
        e.call(0x00cb_d7a0, &args![item, holder]);
        assert_eq!(values_of(&e, holder), [3.0, 9.0]);
    }

    #[test]
    fn the_registration_routine_stores_the_callback_address() {
        let mut e = Engine::new();
        e.map(CALLBACK_WORD, 4);
        e.call(0x00cb_d960, &args![]);
        assert_eq!(e.global::<u32>(CALLBACK_WORD), 0x00cb_d7a0);
    }
}
