//! `Havok SDK/run_00d5f480` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The hash multiplier of the map (`0x9e3779b1`, written as `-0x61c8864f` in the exe) and the key
/// that marks an empty slot.
const HASH_MULTIPLIER: u32 = 0x9e37_79b1;
const EMPTY_KEY: u32 = 0xffff_ffff;

// Translated from 00d5f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Insert into an open-addressing hash map with 16-byte slots, a key and three words (thiscall,
/// `RET 0x14`: five stack words `allocator`, `key`, `first`, `second`, `third`; no map name, the
/// class is the `hkMap`-like container whose resize is `00d5f670`): the table is at `+0`, the
/// number of entries (with a flag bit at the top) at `+4` and the mask (size - 1) at `+8`. When
/// twice the entry count (the flag bit removed) is above the mask (signed) the map is first
/// resized with `00d5f670(allocator, 2 * mask + 2)`. The slot starts at `((key >> 4) *
/// 0x9e3779b1) & mask` and probes forward (wrapping with the mask) for the key or for a slot whose
/// key is `0xffffffff`; a new key adds one to the entry count; the slot gets `key` and the three
/// words at `+4`, `+8` and `+0xc` (`first`, `second`, `third`).
pub fn fn_00d5f480(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    key: u32,
    first: u32,
    second: u32,
    third: u32,
) {
    let me = this.addr();
    let entries = e.mem.u32(me + 4);
    let mask = e.mem.u32(me + 8);
    let doubled = (entries & 0x7fff_ffff).wrapping_add(entries & 0x7fff_ffff) as i32;
    if doubled > mask as i32 {
        let wanted = mask.wrapping_add(mask).wrapping_add(2);
        e.call(0x00d5_f670, &args![this, allocator, wanted]);
    }
    let mask = e.mem.u32(me + 8);
    let table = e.mem.u32(me);
    let mut index = (key >> 4).wrapping_mul(HASH_MULTIPLIER) & mask;
    let mut added = 1u32;
    if e.mem.u32(table.wrapping_add(index << 4)) != EMPTY_KEY {
        loop {
            if e.mem.u32(table.wrapping_add(index << 4)) == key {
                added = 0;
                break;
            }
            index = index.wrapping_add(1) & mask;
            if e.mem.u32(table.wrapping_add(index << 4)) == EMPTY_KEY {
                break;
            }
        }
    }
    let count = e.mem.u32(me + 4).wrapping_add(added);
    e.mem.set_u32(me + 4, count);
    let slot = index << 4;
    e.mem.set_u32(table.wrapping_add(slot), key);
    let values = e.mem.u32(me).wrapping_add(slot).wrapping_add(4);
    e.mem.set_u32(values, first);
    e.mem.set_u32(values + 4, second);
    e.mem.set_u32(values + 8, third);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d5_f480,
        fn_00d5f480(Ptr, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// A map with 8 slots (mask 7), all empty.
    fn map(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let table = e.mem.alloc(8 * 16);
        for slot in 0..8 {
            e.mem.set_u32(table + slot * 16, EMPTY_KEY);
        }
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 8, 7);
        (this, table)
    }

    fn home(key: u32) -> u32 {
        (key >> 4).wrapping_mul(HASH_MULTIPLIER) & 7
    }

    #[test]
    fn fn_00d5f480_inserts_overwrites_and_probes() {
        let mut e = Engine::new();
        e.register(0x00d5_f670, |_, _| Ret::default());
        let (this, table) = map(&mut e);
        // Keys with the same home slot: find two by search.
        let first_key = 0x40u32;
        let second_key = (0x41u32..0x1000)
            .map(|k| k << 4)
            .find(|k| home(*k) == home(first_key))
            .unwrap();
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, first_key, 1u32, 2u32, 3u32],
        );
        let slot = home(first_key);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(table + slot * 16), first_key);
        assert_eq!(
            [1u32, 2, 3].map(|word| e.mem.u32(table + slot * 16 + 4 * word)),
            [1, 2, 3]
        );
        // The same key again: no new entry, the words are replaced.
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, first_key, 7u32, 8u32, 9u32],
        );
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(table + slot * 16 + 4), 7);
        assert_eq!(e.mem.u32(table + slot * 16 + 12), 9);
        // A second key with the same home slot goes to the next slot.
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, second_key, 4u32, 5u32, 6u32],
        );
        assert_eq!(e.mem.u32(this + 4), 2);
        let next = (slot + 1) & 7;
        assert_eq!(e.mem.u32(table + next * 16), second_key);
        assert_eq!(e.mem.u32(table + next * 16 + 4), 4);
        // The flag bit of the count word is kept.
        e.mem.set_u32(this + 4, 0x8000_0002);
        let third_key = (0x1000u32..0x2000)
            .map(|k| k << 4)
            .find(|k| home(*k) == (slot + 4) & 7)
            .unwrap();
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, third_key, 1u32, 1u32, 1u32],
        );
        assert_eq!(e.mem.u32(this + 4), 0x8000_0003);
    }

    #[test]
    fn fn_00d5f480_resizes_first_when_the_map_is_more_than_half_full() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00d5_f670, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let (this, _) = map(&mut e);
        e.mem.set_u32(this + 4, 3);
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, 0x50u32, 0u32, 0u32, 0u32],
        );
        assert!(seen.borrow().is_empty());
        e.mem.set_u32(this + 4, 4);
        e.call(
            0x00d5_f480,
            &args![this, 0x7777u32, 0x60u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(*seen.borrow(), vec![vec![this, 0x7777, 16]]);
    }
}
