//! `Havok SDK/run_00d74b70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d74b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(entries, count, group)`: for each of the `count` entry pointers (signed, at least 1
/// to do anything), takes the two objects at `+0x14` and `+0x18` and, when both exist and both
/// have a non-zero word at `+0x10`, rewrites the 32-bit word at `+0x2c` of each: the fields at
/// bits 5-9 and 10-14 of each are completed (a zero field at bits 5-9 gets the next number from a
/// counter that starts at 0 and runs over the whole call, and a zero field at bits 10-14 is
/// replaced by the other object's bits 5-9, or by its own bits 10-14 when the first object's
/// field is not zero), and `group << 6` is put above them; bits 0-4 are kept. The class is
/// unconfirmed.
pub fn fn_00d74b70(e: &mut Engine, entries: u32, count: i32, group: u32) {
    let mut counter = 0u32;
    let mut index = 0i32;
    if count <= 0 {
        return;
    }
    loop {
        let entry = e
            .mem
            .u32(entries.wrapping_add((index as u32).wrapping_mul(4)));
        let first = e.mem.u32(entry.wrapping_add(0x14));
        let second = e.mem.u32(entry.wrapping_add(0x18));
        if first != 0
            && e.mem.u32(first.wrapping_add(0x10)) != 0
            && second != 0
            && e.mem.u32(second.wrapping_add(0x10)) != 0
        {
            let first_word = e.mem.u32(first.wrapping_add(0x2c));
            let second_word = e.mem.u32(second.wrapping_add(0x2c));
            let mut first_middle = (first_word >> 5) & 0x1f;
            let mut second_middle = (second_word >> 5) & 0x1f;
            let mut first_high = (first_word >> 10) & 0x1f;
            let mut second_high = (second_word >> 10) & 0x1f;
            let first_low = first_word & 0x1f;
            let second_low = second_word & 0x1f;
            if first_middle == 0 {
                first_middle = counter;
                counter = counter.wrapping_add(1);
            }
            if second_middle == 0 {
                second_middle = counter;
                counter = counter.wrapping_add(1);
            }
            if first_high == 0 {
                first_high = second_middle;
            } else {
                second_high = first_middle;
            }
            let shifted = group << 6;
            let first_result = (((shifted | first_high) << 5 | first_middle) << 5) | first_low;
            let second_result = (((shifted | second_high) << 5 | second_middle) << 5) | second_low;
            e.mem.set_u32(first.wrapping_add(0x2c), first_result);
            e.mem.set_u32(second.wrapping_add(0x2c), second_result);
        }
        index += 1;
        if index >= count {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d7_4b70, fn_00d74b70(u32, i32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// An object with a non-zero word at `+0x10` and the given word at `+0x2c`.
    fn object(e: &mut Engine, word: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x2c, word);
        object
    }

    fn entry(e: &mut Engine, first: u32, second: u32) -> u32 {
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0x14, first);
        e.mem.set_u32(entry + 0x18, second);
        entry
    }

    fn list(e: &mut Engine, entries: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * entries.len() as u32);
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *entry);
        }
        data
    }

    #[test]
    fn zero_fields_get_fresh_numbers_from_a_counter_shared_by_the_whole_call() {
        let mut e = Engine::new();
        // Entry 0: both middle fields zero -> 0 and 1; first high field zero -> second middle.
        let a0 = object(&mut e, 0x03);
        let b0 = object(&mut e, 0x04);
        // Entry 1: first middle zero -> 2, second middle 7; first high 9 -> second high from
        // the first middle.
        let a1 = object(&mut e, (9 << 10) | 0x1f);
        let b1 = object(&mut e, (7 << 5) | (6 << 10) | 0x01);
        let entries = [entry(&mut e, a0, b0), entry(&mut e, a1, b1)];
        let data = list(&mut e, &entries);
        e.call(0x00d7_4b70, &args![data, 2i32, 5u32]);
        // Entry 0: counter 0 and 1; first_high = second_middle = 1; second_high = own 0.
        assert_eq!(e.mem.u32(a0 + 0x2c), (((5 << 6 | 1) << 5) << 5) | 0x03);
        assert_eq!(e.mem.u32(b0 + 0x2c), (((5 << 6) << 5 | 1) << 5) | 0x04);
        // Entry 1: first middle takes 2; first_high stays 9, second_high becomes 2.
        assert_eq!(e.mem.u32(a1 + 0x2c), (((5 << 6 | 9) << 5 | 2) << 5) | 0x1f);
        assert_eq!(e.mem.u32(b1 + 0x2c), (((5 << 6 | 2) << 5 | 7) << 5) | 0x01);
    }

    #[test]
    fn entries_missing_an_object_or_its_flag_word_are_left_alone() {
        let mut e = Engine::new();
        let flagless = object(&mut e, 0x20);
        e.mem.set_u32(flagless + 0x10, 0);
        let good = object(&mut e, 0x40);
        let other = object(&mut e, 0x40);
        let entries = [
            entry(&mut e, 0, good),
            entry(&mut e, good, 0),
            entry(&mut e, flagless, good),
            entry(&mut e, good, flagless),
        ];
        let data = list(&mut e, &entries);
        e.call(0x00d7_4b70, &args![data, 4i32, 1u32]);
        assert_eq!(e.mem.u32(good + 0x2c), 0x40);
        assert_eq!(e.mem.u32(flagless + 0x2c), 0x20);
        assert_eq!(e.mem.u32(other + 0x2c), 0x40);
    }

    #[test]
    fn a_count_of_zero_or_less_does_nothing() {
        let mut e = Engine::new();
        let a = object(&mut e, 0);
        let b = object(&mut e, 0);
        let entries = [entry(&mut e, a, b)];
        let data = list(&mut e, &entries);
        e.call(0x00d7_4b70, &args![data, 0i32, 1u32]);
        e.call(0x00d7_4b70, &args![data, -3i32, 1u32]);
        assert_eq!(e.mem.u32(a + 0x2c), 0);
    }
}
