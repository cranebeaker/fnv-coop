//! `Havok SDK/hkpboxboxmanifold.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBoxBoxManifold` (the Xbox PDB has no record of the Havok classes, so the field names
//! below are unconfirmed): up to eight 4-byte entries at `+0` (the first two bytes of an entry
//! identify its contact point; the word at `+2` of an entry is cleared by `fn_00db0920`), a
//! byte at `+0x20`, the entry count at `+0x21` and a byte at `+0x22`.

#[allow(unused_imports)]
use crate::prelude::*;

const COUNT: u32 = 0x21;
const FLAG: u32 = 0x22;

// Translated from 00db0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxManifold::removePoint` (engine map): clears the byte at `+0x22`, moves the last
/// entry (`+0x21` counts them) over entry `index` and drops the count. The index is not
/// checked and the count wraps as a byte.
pub fn hkp_box_box_manifold_remove_point(e: &mut Engine, this: Ptr, index: u32) {
    let manifold = this.addr();
    e.mem.set_u8(manifold + FLAG, 0);
    let count = u32::from(e.mem.u8(manifold + COUNT));
    let last = e
        .mem
        .u32(manifold.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
    e.mem
        .set_u32(manifold.wrapping_add(index.wrapping_mul(4)), last);
    let count = e.mem.u8(manifold + COUNT).wrapping_sub(1);
    e.mem.set_u8(manifold + COUNT, count);
}

// Translated from 00db0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the manifold (no engine-map name; `__fastcall`): the words at `+2`, `+6`, `+0xA`,
/// `+0xE`, `+0x12`, `+0x16`, `+0x1A` and `+0x1E` and the bytes at `+0x20`, `+0x21` and `+0x22`
/// become 0. Returns `this` (still in `EAX`).
pub fn fn_00db0920(e: &mut Engine, this: Ptr) -> Ptr {
    let manifold = this.addr();
    for offset in [2, 0xa, 0x12, 0x1a, 6, 0xe, 0x16, 0x1e] {
        e.mem.set_u16(manifold + offset, 0);
    }
    e.mem.set_u8(manifold + COUNT, 0);
    e.mem.set_u8(manifold + 0x20, 0);
    e.mem.set_u8(manifold + FLAG, 0);
    this
}

// Translated from 00db09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxManifold::findInManifold` (engine map): looks for an entry whose first two
/// bytes equal those of `key`, from the last entry down to the first; writes 1 (found) or 0 to
/// the byte at `found` and returns `found`.
pub fn hkp_box_box_manifold_find_in_manifold(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    key: Ptr,
) -> Ptr {
    let manifold = this.addr();
    let mut index = i32::from(e.mem.u8(manifold + COUNT)) - 1;
    while index >= 0 {
        let entry = manifold.wrapping_add((index as u32).wrapping_mul(4));
        if e.mem.u8(entry) == e.mem.u8(key.addr())
            && e.mem.u8(entry + 1) == e.mem.u8(key.addr() + 1)
        {
            e.mem.set_u8(found.addr(), 1);
            return found;
        }
        index -= 1;
    }
    e.mem.set_u8(found.addr(), 0);
    found
}

// Translated from 00db0960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxManifold::addPoint` (engine map): returns the new entry's index after storing
/// the word at `key` there and counting it, or `0xffffffff` when the manifold already holds
/// more than 8 entries, an entry with the same first two bytes is there (the exe passes
/// the address of its own `key` stack slot as the result byte of `findInManifold`) or the
/// manifold is full (8 entries). The first two stack words are never read.
pub fn hkp_box_box_manifold_add_point(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    key: Ptr,
) -> u32 {
    let manifold = this.addr();
    let count = e.mem.u8(manifold + COUNT);
    if count > 8 {
        return u32::MAX;
    }
    let found = e.with_stack(4, |e, slot| {
        let result = hkp_box_box_manifold_find_in_manifold(e, this, slot, key);
        e.mem.u8(result.addr()) != 0
    });
    if !found && u32::from(count) < 8 {
        let word = e.mem.u32(key.addr());
        e.mem.set_u32(manifold + u32::from(count) * 4, word);
        let next = e.mem.u8(manifold + COUNT).wrapping_add(1);
        e.mem.set_u8(manifold + COUNT, next);
        return u32::from(count);
    }
    u32::MAX
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db_0900, hkp_box_box_manifold_remove_point(Ptr, u32)),
        entry!(0x00db_0920, fn_00db0920(Ptr) -> Ptr),
        entry!(0x00db_0960, hkp_box_box_manifold_add_point(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00db_09b0, hkp_box_box_manifold_find_in_manifold(Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    /// A manifold block whose entry `i` is `[id_a, id_b, i, 0]` for the given ids.
    fn manifold(e: &mut Engine, ids: &[(u8, u8)]) -> u32 {
        let manifold = e.mem.alloc(0x30);
        for (i, (a, b)) in ids.iter().enumerate() {
            e.mem.set_u8(manifold + 4 * i as u32, *a);
            e.mem.set_u8(manifold + 4 * i as u32 + 1, *b);
            e.mem.set_u16(manifold + 4 * i as u32 + 2, 0x100 + i as u16);
        }
        e.mem.set_u8(manifold + COUNT, ids.len() as u8);
        manifold
    }

    fn key(e: &mut Engine, a: u8, b: u8) -> u32 {
        let key = e.mem.alloc(8);
        e.mem.set_u8(key, a);
        e.mem.set_u8(key + 1, b);
        e.mem.set_u16(key + 2, 0x5a5a);
        key
    }

    #[test]
    fn remove_point_moves_the_last_entry_into_the_hole() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[(1, 2), (3, 4), (5, 6)]);
        e.mem.set_u8(m + FLAG, 1);
        e.call(0x00db_0900, &args![m, 0u32]);
        assert_eq!(e.mem.u32(m), e.mem.u32(m + 8));
        assert_eq!(e.mem.u8(m + COUNT), 2);
        assert_eq!(e.mem.u8(m + FLAG), 0);
        assert_eq!(e.mem.u32(m + 4), 0x0101_0403);
    }

    #[test]
    fn remove_point_of_the_only_entry_copies_it_onto_itself() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[(7, 8)]);
        let before = e.mem.u32(m);
        e.call(0x00db_0900, &args![m, 0u32]);
        assert_eq!(e.mem.u32(m), before);
        assert_eq!(e.mem.u8(m + COUNT), 0);
    }

    #[test]
    fn clear_zeroes_the_listed_fields_and_returns_this() {
        let mut e = Engine::new();
        let m = e.mem.alloc(0x30);
        for offset in 0..0x30 {
            e.mem.set_u8(m + offset, 0xff);
        }
        assert_eq!(e.call(0x00db_0920, &args![m]).u32(), m);
        for offset in [2u32, 6, 0xa, 0xe, 0x12, 0x16, 0x1a, 0x1e] {
            assert_eq!(e.mem.u16(m + offset), 0);
        }
        assert_eq!(
            [e.mem.u8(m + 0x20), e.mem.u8(m + 0x21), e.mem.u8(m + 0x22)],
            [0, 0, 0]
        );
        assert_eq!(e.mem.u8(m), 0xff);
        assert_eq!(e.mem.u8(m + 1), 0xff);
        assert_eq!(e.mem.u8(m + 0x23), 0xff);
    }

    #[test]
    fn find_in_manifold_matches_both_id_bytes() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[(1, 2), (1, 3), (4, 2)]);
        let found = e.mem.alloc(4);
        let hit = key(&mut e, 1, 3);
        assert_eq!(e.call(0x00db_09b0, &args![m, found, hit]).u32(), found);
        assert_eq!(e.mem.u8(found), 1);
        let miss = key(&mut e, 4, 3);
        assert_eq!(e.call(0x00db_09b0, &args![m, found, miss]).u32(), found);
        assert_eq!(e.mem.u8(found), 0);
        let first = key(&mut e, 1, 2);
        e.call(0x00db_09b0, &args![m, found, first]);
        assert_eq!(e.mem.u8(found), 1);
    }

    #[test]
    fn find_in_manifold_of_an_empty_manifold_finds_nothing() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[]);
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 9);
        let probe = key(&mut e, 0, 0);
        e.call(0x00db_09b0, &args![m, found, probe]);
        assert_eq!(e.mem.u8(found), 0);
    }

    #[test]
    fn add_point_appends_a_new_id_and_returns_its_index() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[(1, 2)]);
        let fresh = key(&mut e, 9, 9);
        let got = e.call(0x00db_0960, &args![m, 0u32, 0u32, fresh]).u32();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u8(m + COUNT), 2);
        assert_eq!(e.mem.u32(m + 4), e.mem.u32(fresh));
    }

    #[test]
    fn add_point_refuses_a_known_id() {
        let mut e = Engine::new();
        let m = manifold(&mut e, &[(1, 2)]);
        let known = key(&mut e, 1, 2);
        let got = e.call(0x00db_0960, &args![m, 0u32, 0u32, known]).u32();
        assert_eq!(got, u32::MAX);
        assert_eq!(e.mem.u8(m + COUNT), 1);
    }

    #[test]
    fn add_point_refuses_when_the_manifold_is_full_or_over_full() {
        let mut e = Engine::new();
        let ids: Vec<(u8, u8)> = (0..8).map(|i| (i, i)).collect();
        let m = manifold(&mut e, &ids);
        let fresh = key(&mut e, 9, 9);
        assert_eq!(
            e.call(0x00db_0960, &args![m, 0u32, 0u32, fresh]).u32(),
            u32::MAX
        );
        assert_eq!(e.mem.u8(m + COUNT), 8);
        e.mem.set_u8(m + COUNT, 9);
        assert_eq!(
            e.call(0x00db_0960, &args![m, 0u32, 0u32, fresh]).u32(),
            u32::MAX
        );
        assert_eq!(e.mem.u8(m + COUNT), 9);
    }
}
