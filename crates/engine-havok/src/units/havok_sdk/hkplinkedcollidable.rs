//! `Havok SDK/hkplinkedcollidable.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpLinkedCollidable` (Xbox PDB) is the collidable of an entity: its `m_collisionEntries`
//! (an `hkArray` of 8-byte `hkpLinkedCollidable::CollisionEntry`, the agent entry pointer
//! first, the partner collidable second) is at +0x50. A collidable finds its owner by the
//! signed byte at +0x10 (`m_ownerOffset`, Xbox PDB `hkpCollidable`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkarrayutil::HkArrayBase;

/// The thread's memory router (`00c85750`); word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkAlgorithm::quickSortRecursive` over the collision entries (`00d04200`, cdecl `(data,
/// low, high, comparison function)`).
const QUICK_SORT: u32 = 0x00d0_4200;
/// The comparison function it is given (a plain function, passed by address).
const ENTRY_COMPARISON: u32 = 0x00d0_81f0;
/// `hkpMotion::m_type` (Xbox PDB) of the owner, at +0xE8 of an `hkpEntity` (its `m_motion`
/// is at +0xE0), and the value of `MOTION_FIXED`.
const OWNER_MOTION_TYPE: u32 = 0xe8;
const MOTION_FIXED: u8 = 5;

layout! {
    /// `hkpLinkedCollidable` (Xbox PDB) as far as these functions read it.
    pub struct HkpLinkedCollidable: 0x60 {
        /// `m_ownerOffset` (Xbox PDB `hkpCollidable`): signed byte from the collidable to its
        /// owner.
        0x10 m_ownerOffset: i8,
        /// `m_collisionEntries` (Xbox PDB): `hkArray<CollisionEntry>`.
        0x50 m_collisionEntries: Inline<HkArrayBase>,
    }
}

// Translated from 00d081f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a "less than" comparison for a sort, cdecl, two stack words (nothing
/// popped): answers 1 when the word at +0x14 of the object at `a + 4` is below (unsigned) the same
/// word of `b`, else 0 (`CMP` / `SBB` / `NEG`). Ghidra made no function here; the address is pushed at 00d0821b.
pub fn fn_00d081f0(e: &mut Engine, a: u32, b: u32) -> u32 {
    let a_object = e.mem.u32(a + 4);
    let b_object = e.mem.u32(b + 4);
    u32::from(e.mem.u32(a_object + 0x14) < e.mem.u32(b_object + 0x14))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d081d0,
            hkp_linked_collidable_get_collision_entries_deterministic_unchecked(
                Ptr<HkpLinkedCollidable>,
            )
                -> Ptr<HkArrayBase>
        ),
        entry!(
            0x00d081e0,
            hkp_linked_collidable_get_collision_entries_deterministic_unchecked_ov2(
                Ptr<HkpLinkedCollidable>,
            )
                -> Ptr<
                HkArrayBase,
            >
        ),
        entry!(
            0x00d08210,
            hkp_linked_collidable_sort_entries(Ptr<HkpLinkedCollidable>)
        ),
        entry!(
            0x00d08260,
            hkp_linked_collidable_get_collision_entries_sorted(
                Ptr<HkpLinkedCollidable>,
                Ptr<HkArrayBase>,
            )
        ),
        entry!(0x00d0_81f0, fn_00d081f0(u32, u32) -> u32),
    ]
}

// Translated from 00d081d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinkedCollidable::getCollisionEntriesDeterministicUnchecked` (Xbox PDB, the first
/// overload; `LEA EAX,[ECX+0x50]`): the address of the collision entry array.
pub fn hkp_linked_collidable_get_collision_entries_deterministic_unchecked(
    _e: &mut Engine,
    this: Ptr<HkpLinkedCollidable>,
) -> Ptr<HkArrayBase> {
    this.at(HkpLinkedCollidable::m_collisionEntries)
}

// Translated from 00d081e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinkedCollidable::getCollisionEntriesDeterministicUnchecked` (Xbox PDB, the
/// overload the engine map names `_ov2`): the address of the collision entry array
/// (`this + 0x50`).
pub fn hkp_linked_collidable_get_collision_entries_deterministic_unchecked_ov2(
    _e: &mut Engine,
    this: Ptr<HkpLinkedCollidable>,
) -> Ptr<HkArrayBase> {
    this.at(HkpLinkedCollidable::m_collisionEntries)
}

// Translated from 00d08210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinkedCollidable::sortEntries` (Xbox PDB): sorts the collision entries with the quick
/// sort when there are two or more, then writes each position back into its agent entry:
/// the 16-bit word at +4 of the entry, or at +6 when this collidable is the one at the
/// entry's +0x14 (the entry's second collidable).
pub fn hkp_linked_collidable_sort_entries(e: &mut Engine, this: Ptr<HkpLinkedCollidable>) {
    let entries = this.at(HkpLinkedCollidable::m_collisionEntries);
    let size = e.get(entries, HkArrayBase::m_size);
    if size > 1 {
        let data = e.get(entries, HkArrayBase::m_data);
        e.call(QUICK_SORT, &args![data, 0i32, size - 1, ENTRY_COMPARISON]);
    }
    let mut index = 0i32;
    while index < e.get(entries, HkArrayBase::m_size) {
        let data = e.get(entries, HkArrayBase::m_data);
        let entry = e
            .mem
            .u32(data.addr().wrapping_add((index as u32).wrapping_mul(8)));
        let second = e.mem.u32(entry + 0x14);
        let word = 4 + 2 * u32::from(this.addr() == second);
        e.mem.set_u16(entry + word, index as u16);
        index += 1;
    }
}

// Translated from 00d08260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinkedCollidable::getCollisionEntriesSorted` (Xbox PDB), `RET 4`: copies the entries
/// into `out` (growing it to at least the entry count, by doubling its capacity if that is
/// larger), sets its size, and sorts the copy when the owner's motion type is fixed.
pub fn hkp_linked_collidable_get_collision_entries_sorted(
    e: &mut Engine,
    this: Ptr<HkpLinkedCollidable>,
    out: Ptr<HkArrayBase>,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let entries = this.at(HkpLinkedCollidable::m_collisionEntries);
    let count = e.get(entries, HkArrayBase::m_size);
    let mut existing = e.get(out, HkArrayBase::m_size);
    let allocator = e.mem.u32(router + 0x10);
    if count <= existing {
        existing = count;
    }
    let capacity = (e.get(out, HkArrayBase::m_capacityAndFlags) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, out, wanted, 8i32]);
    }
    let source = e.get(entries, HkArrayBase::m_data).addr();
    let destination = e.get(out, HkArrayBase::m_data).addr();
    // The entries already in `out` are overwritten, the rest are constructed in place.
    for index in 0..existing as u32 {
        let from = source.wrapping_add(index * 8);
        let to = destination.wrapping_add(index * 8);
        let first = e.mem.u32(from);
        e.mem.set_u32(to, first);
        let second = e.mem.u32(from + 4);
        e.mem.set_u32(to + 4, second);
    }
    for index in existing.max(0) as u32..count.max(0) as u32 {
        let from = source.wrapping_add(index * 8);
        let to = destination.wrapping_add(index * 8);
        if to != 0 {
            let first = e.mem.u32(from);
            e.mem.set_u32(to, first);
            let second = e.mem.u32(from + 4);
            e.mem.set_u32(to + 4, second);
        }
    }
    e.set(out, HkArrayBase::m_size, count);
    let owner_offset = e.get(this, HkpLinkedCollidable::m_ownerOffset);
    let owner = this.addr().wrapping_add(owner_offset as i32 as u32);
    if e.mem.u8(owner + OWNER_MOTION_TYPE) == MOTION_FIXED && count > 1 {
        let data = e.get(out, HkArrayBase::m_data);
        e.call(QUICK_SORT, &args![data, 0i32, count - 1, ENTRY_COMPARISON]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUTER: u32 = 0x0300_0000;
    const ALLOCATOR: u32 = 0x0300_0040;

    fn rig() -> (Engine, Ptr<HkpLinkedCollidable>) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(ROUTER, 0x100);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.register(MEMORY_ROUTER, |_, _| ROUTER.into_ret());
        e.register(QUICK_SORT, |_, _| Ret::default());
        // Reserve gives `out` room for exactly the asked count.
        e.register(ARRAY_RESERVE, |e, a| {
            let block = e.mem.alloc(8 * a[2].max(1));
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        // A collidable inside an owner: the owner is 0x20 bytes before it.
        let owner = e.mem.alloc(0x200);
        let collidable = Ptr::new(owner + 0x20);
        e.mem.set_i8(collidable.addr() + 0x10, -0x20);
        (e, collidable)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn fill_entries(e: &mut Engine, collidable: Ptr<HkpLinkedCollidable>, pairs: &[(u32, u32)]) {
        let entries = collidable.at(HkpLinkedCollidable::m_collisionEntries);
        let data = e.mem.alloc(8 * pairs.len() as u32);
        for (i, (first, second)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *first);
            e.mem.set_u32(data + 8 * i as u32 + 4, *second);
        }
        e.set(entries, HkArrayBase::m_data, Ptr::new(data));
        e.set(entries, HkArrayBase::m_size, pairs.len() as i32);
        e.set(entries, HkArrayBase::m_capacityAndFlags, pairs.len() as u32);
    }

    #[test]
    fn entries_accessor_is_the_embedded_array() {
        let (mut e, collidable) = rig();
        let result = e.call(0x00d0_81e0, &args![collidable]).u32();
        assert_eq!(result, collidable.addr() + 0x50);
    }

    #[test]
    fn sort_entries_numbers_the_agent_entries() {
        let (mut e, collidable) = rig();
        // Three agent entries: the middle one has this collidable as its second.
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        let c = e.mem.alloc(0x20);
        e.mem.set_u32(b + 0x14, collidable.addr());
        e.mem.set_u32(a + 0x14, 0x1234);
        e.mem.set_u32(c + 0x14, 0x1234);
        fill_entries(&mut e, collidable, &[(a, 0), (b, 0), (c, 0)]);
        e.call(0x00d0_8210, &args![collidable]);
        let data = e
            .get(
                collidable.at(HkpLinkedCollidable::m_collisionEntries),
                HkArrayBase::m_data,
            )
            .addr();
        assert_eq!(
            calls(&e, QUICK_SORT),
            vec![vec![data, 0, 2, ENTRY_COMPARISON]]
        );
        assert_eq!(e.mem.u16(a + 4), 0);
        assert_eq!(e.mem.u16(b + 6), 1);
        assert_eq!(e.mem.u16(b + 4), 0);
        assert_eq!(e.mem.u16(c + 4), 2);
        assert_eq!(e.mem.u16(c + 6), 0);
    }

    #[test]
    fn sorted_copy_grows_the_output_and_sorts_for_a_fixed_owner() {
        let (mut e, collidable) = rig();
        fill_entries(
            &mut e,
            collidable,
            &[(0x11, 0x21), (0x12, 0x22), (0x13, 0x23)],
        );
        let out = e.new_object::<HkArrayBase>();
        e.call(0x00d0_8260, &args![collidable, out]);
        // The output was empty: it is reserved for the three entries, 8 bytes each.
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![ALLOCATOR, out.addr(), 3, 8]]
        );
        let data = e.get(out, HkArrayBase::m_data).addr();
        assert_eq!(e.get(out, HkArrayBase::m_size), 3);
        assert_eq!(e.mem.u32(data + 16), 0x13);
        assert_eq!(e.mem.u32(data + 20), 0x23);
        // The owner is not fixed: no sort.
        assert!(calls(&e, QUICK_SORT).is_empty());

        // A fixed owner (motion type 5 at owner + 0xe8) sorts the copy.
        e.mem
            .set_u8(collidable.addr() - 0x20 + OWNER_MOTION_TYPE, MOTION_FIXED);
        e.call(0x00d0_8260, &args![collidable, out]);
        // `out` now has room (capacity 3 is not below the count): no second reserve.
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 1);
        assert_eq!(
            calls(&e, QUICK_SORT),
            vec![vec![data, 0, 2, ENTRY_COMPARISON]]
        );
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    #[test]
    fn the_first_overload_returns_the_entry_array_address() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        let got = e.call(0x00d081d0, &args![this]).u32();
        assert_eq!(got, this + 0x50);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    fn linked_object(e: &mut Engine, key: u32) -> u32 {
        let holder = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 4, object);
        e.mem.set_u32(object + 0x14, key);
        holder
    }

    #[test]
    fn fn_00d081f0_compares_the_keys_unsigned() {
        let mut e = Engine::new();
        for (a, b, expected) in [
            (1, 2, 1),
            (2, 1, 0),
            (2, 2, 0),
            (0xffff_ffff, 1, 0),
            (1, 0xffff_ffff, 1),
        ] {
            let first = linked_object(&mut e, a);
            let second = linked_object(&mut e, b);
            assert_eq!(e.call(0x00d0_81f0, &args![first, second]).u32(), expected);
        }
    }
}
