//! `Havok SDK/hkmap.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to the 64-bit key, 64-bit value instance
//! `hkMapBase<unsigned __int64, unsigned __int64, hkMapOperations<unsigned
//! __int64>>` (the engine map names `insert`, `findKey`, `clearAndDeallocate`
//! and `resizeTable`; the other four have no name). The map is an open
//! addressing hash table: `m_elem` points at slots of 16 bytes (key low word,
//! key high word, value low word, value high word), an empty slot has both key
//! words `0xffffffff`, `m_hashMod` is the slot count minus one, and the
//! `m_numElems` word keeps the element count in its low 31 bits and, in bit 31
//! (`0x80000000`), the flag that the slots are not owned by the map (a buffer
//! given at construction), so they are never freed. A key hashes to
//! `(key low >> 4) * 0x9e3779b1 & m_hashMod`; collisions probe the next slot.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned __int64>>`
    /// (Xbox PDB), 0xC bytes.
    pub struct HkMapBase: 0xC {
        /// `m_elem` (Xbox PDB): the slots.
        0x00 m_elem: u32,
        /// `m_numElems` (Xbox PDB): the element count, bit 31 set when the slots are a
        /// buffer the map does not own.
        0x04 m_numElems: u32,
        /// `m_hashMod` (Xbox PDB): the slot count minus one.
        0x08 m_hashMod: i32,
    }
}

/// The hash multiplier `-0x61c8864f` (`0x9e3779b1`).
const HASH_MULTIPLIER: u32 = 0x9e37_79b1;
/// Bit 31 of `m_numElems`: the slots are not owned by the map.
const NOT_OWNED: u32 = 0x8000_0000;
/// A slot's key words are both this when it is empty.
const EMPTY: u32 = 0xffff_ffff;
/// The slot size.
const SLOT_SIZE: u32 = 0x10;

/// The slot address.
fn slot_address(e: &Engine, this: Ptr<HkMapBase>, slot: u32) -> u32 {
    e.get(this, HkMapBase::m_elem)
        .wrapping_add(slot.wrapping_mul(SLOT_SIZE))
}

/// Whether both key words are `0xffffffff` (`AND`, `CMP -1`).
fn is_empty(low: u32, high: u32) -> bool {
    low & high == EMPTY
}

/// The key's home slot before masking.
fn hash(key_low: u32) -> u32 {
    (key_low >> 4).wrapping_mul(HASH_MULTIPLIER)
}

/// Writes the two key words of a slot as the empty marker.
fn make_empty(e: &mut Engine, address: u32) {
    e.mem.set_u32(address, EMPTY);
    e.mem.set_u32(address + 4, EMPTY);
}

// Translated from 00cb1810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. The size in bytes of the slots for `count` elements:
/// the slot count is the smallest power of two that is at least 8 and at
/// least twice `count`, times 16 bytes. (Like the exe's signed loop, a
/// `count` above 0x20000000 never ends.)
pub fn fn_00cb1810(_e: &mut Engine, count: i32) -> u32 {
    let wanted = count.wrapping_add(count);
    let mut slots = 8i32;
    if wanted > slots {
        loop {
            slots = slots.wrapping_add(slots);
            if slots >= wanted {
                break;
            }
        }
    }
    (slots as u32) << 4
}

// Translated from 00cb1b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Sets the map up over a buffer of `byte_size` bytes at
/// `storage`: `m_elem` is the buffer, `m_numElems` is `0x80000000` (empty,
/// not owned), `m_hashMod` is the slot count minus one, and every slot is
/// marked empty.
pub fn fn_00cb1b80(e: &mut Engine, this: Ptr<HkMapBase>, storage: u32, byte_size: u32) {
    let slots = byte_size >> 4;
    e.set(this, HkMapBase::m_elem, storage);
    e.set(this, HkMapBase::m_numElems, NOT_OWNED);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1) as i32);
    for slot in 0..slots {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
}

// Translated from 00cb1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::insert` (Xbox PDB): stores the pair, replacing the value of an
/// equal key. First grows the table (`resizeTable`, through `allocator`, to
/// `2 * m_hashMod + 2` slots) when twice the element count exceeds
/// `m_hashMod`. The element count rises by one for a new key.
pub fn hk_map_base_insert(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key_low: u32,
    key_high: u32,
    value_low: u32,
    value_high: u32,
) {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let mask = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > mask {
        let slots = mask.wrapping_add(mask).wrapping_add(2);
        hk_map_base_resize_table(e, this, allocator, slots);
    }
    let mut slot = hash(key_low) & e.get(this, HkMapBase::m_hashMod) as u32;
    let mut added = 1u32;
    loop {
        let address = slot_address(e, this, slot);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if is_empty(low, high) {
            break;
        }
        if low == key_low && high == key_high {
            added = 0;
            break;
        }
        slot = slot.wrapping_add(1) & e.get(this, HkMapBase::m_hashMod) as u32;
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_add(added));
    let address = slot_address(e, this, slot);
    e.mem.set_u32(address, key_low);
    e.mem.set_u32(address + 4, key_high);
    e.mem.set_u32(address + 8, value_low);
    e.mem.set_u32(address + 0xc, value_high);
}

// Translated from 00cb1c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::findKey` (Xbox PDB): the slot index holding the key, or
/// `m_hashMod + 1` when it is not in the map (also when `m_hashMod` is not
/// positive).
pub fn hk_map_base_find_key(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
) -> u32 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let mut slot = hash(key_low) & mask as u32;
        loop {
            let address = slot_address(e, this, slot);
            let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
            if is_empty(low, high) {
                break;
            }
            if low == key_low && high == key_high {
                return slot;
            }
            slot = slot.wrapping_add(1) & mask as u32;
        }
    }
    (mask as u32).wrapping_add(1)
}

// Translated from 00cb1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Removes the element in slot `index`: empties the slot,
/// lowers the element count, then walks the slots that follow (up to the next
/// empty one) and moves back into the hole every element whose home slot
/// does not lie cyclically between the hole and itself, so that later
/// lookups still find them.
pub fn fn_00cb1d20(e: &mut Engine, this: Ptr<HkMapBase>, index: u32) {
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_sub(1));
    let address = slot_address(e, this, index);
    make_empty(e, address);
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;

    // The nearest empty slot before the hole, going backwards.
    let mut probe = mask.wrapping_add(index) & mask;
    loop {
        let address = slot_address(e, this, probe);
        if is_empty(e.mem.u32(address), e.mem.u32(address + 4)) {
            break;
        }
        probe = probe.wrapping_add(mask) & mask;
    }
    let first_after = probe.wrapping_add(1) & mask;

    let mut hole = index;
    let mut slot = index;
    loop {
        let mask = e.get(this, HkMapBase::m_hashMod) as u32;
        slot = slot.wrapping_add(1) & mask;
        let address = slot_address(e, this, slot);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if is_empty(low, high) {
            break;
        }
        let home = hash(low) & mask;
        // The three unsigned range tests of the exe, each of which keeps
        // the element where it is.
        let stays = (slot >= first_after && home > hole)
            || (slot < hole && (home > hole || home <= slot))
            || (home > hole && home < first_after);
        if !stays {
            let target = slot_address(e, this, hole);
            e.mem.set_u32(target, low);
            e.mem.set_u32(target + 4, high);
            let value_low = e.mem.u32(address + 8);
            let value_high = e.mem.u32(address + 0xc);
            e.mem.set_u32(target + 8, value_low);
            e.mem.set_u32(target + 0xc, value_high);
            make_empty(e, address);
            hole = slot;
        }
    }
}

// Translated from 00cb1e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Empties every slot and zeroes the element count, keeping
/// the not-owned flag in bit 31.
pub fn fn_00cb1e00(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    for slot in 0..slots.max(0) as u32 {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00cb1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::clearAndDeallocate` (Xbox PDB): empties the map, then, unless
/// the slots are not owned, returns them to `allocator` (vtable slot 8, the
/// slots and their size in bytes).
pub fn hk_map_base_clear_and_deallocate(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr) {
    fn_00cb1e00(e, this);
    if e.get(this, HkMapBase::m_numElems) & NOT_OWNED == 0 {
        let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1) as u32;
        let storage = e.get(this, HkMapBase::m_elem);
        e.vcall(allocator.addr(), 8, &args![storage, slots << 4]);
    }
}

// Translated from 00cb1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned
/// __int64>>::resizeTable` (Xbox PDB): gives the map `max(slots, 8)` slots
/// from `allocator` (vtable slot 4, the size in bytes), marks them empty,
/// clears the element count (and the not-owned flag), then inserts every
/// element of the old table again and, if the old slots were owned, returns
/// them to the allocator (vtable slot 8, the slots and their size).
pub fn hk_map_base_resize_table(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, slots: i32) {
    let slots = if slots > 8 { slots } else { 8 };
    let was_not_owned = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_storage = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let storage = e
        .vcall(allocator.addr(), 4, &args![(slots as u32) << 4])
        .u32();
    e.set(this, HkMapBase::m_elem, storage);
    for slot in 0..slots as u32 {
        let address = slot_address(e, this, slot);
        make_empty(e, address);
    }
    e.set(this, HkMapBase::m_numElems, 0);
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    for old in 0..old_slots.max(0) as u32 {
        let address = old_storage.wrapping_add(old * SLOT_SIZE);
        let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
        if !is_empty(low, high) {
            let value_low = e.mem.u32(address + 8);
            let value_high = e.mem.u32(address + 0xc);
            hk_map_base_insert(e, this, allocator, low, high, value_low, value_high);
        }
    }
    if was_not_owned == 0 {
        let size = (old_slots as u32) << 4;
        e.vcall(allocator.addr(), 8, &args![old_storage, size]);
    }
}

// The 32-bit instance, `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned
// long>>`: the same table as above with 8-byte slots (key, value); an empty slot has its key
// word `0xffffffff`. The functions below are its methods (and a few of the 64-bit instance's).

/// The slot size shift of the 32-bit instance (8 bytes) and of the 64-bit one (16 bytes).
const SLOT_SHIFT_32: u32 = 3;

/// The slot index holding `key` in a map with slots of `1 << slot_shift` bytes whose keys are
/// one word, or `m_hashMod + 1` when it is not there (also when `m_hashMod` is not positive).
pub(crate) fn find_key_word(e: &mut Engine, map: u32, key: u32, slot_shift: u32) -> u32 {
    let mask = e.mem.i32(map + 8);
    if mask > 0 {
        let slots = e.mem.u32(map);
        let mask = mask as u32;
        let mut slot = hash(key) & mask;
        let mut found = e.mem.u32(slots.wrapping_add(slot.wrapping_shl(slot_shift)));
        while found != EMPTY {
            if found == key {
                return slot;
            }
            slot = slot.wrapping_add(1) & mask;
            found = e.mem.u32(slots.wrapping_add(slot.wrapping_shl(slot_shift)));
        }
    }
    (mask as u32).wrapping_add(1)
}

/// Removes the element in slot `index` of a map with slots of `1 << slot_shift` bytes (key
/// word, then `value_words` value words): empties the slot, lowers the element count, then
/// walks the slots that follow (up to the next empty one) and moves back into the hole every
/// element whose home slot does not lie cyclically between the hole and itself, so that later
/// lookups still find them (the exe's three unsigned range tests, which all must pass for the
/// element to move).
pub(crate) fn remove_slot(e: &mut Engine, map: u32, index: u32, slot_shift: u32, value_words: u32) {
    let offset = |slot: u32| slot.wrapping_shl(slot_shift);
    let count = e.mem.u32(map + 4).wrapping_sub(1);
    e.mem.set_u32(map + 4, count);
    let slots = e.mem.u32(map);
    e.mem.set_u32(slots.wrapping_add(offset(index)), EMPTY);
    let slots = e.mem.u32(map);
    let mask = e.mem.u32(map + 8);
    // The nearest empty slot before the hole, going backwards.
    let mut probe = mask.wrapping_add(index) & mask;
    if e.mem.u32(slots.wrapping_add(offset(probe))) != EMPTY {
        loop {
            probe = probe.wrapping_add(mask) & mask;
            if e.mem.u32(slots.wrapping_add(offset(probe))) == EMPTY {
                break;
            }
        }
    }
    let first_after = probe.wrapping_add(1) & mask;
    let mut hole = index;
    let mut current = index.wrapping_add(1) & mask;
    if e.mem.u32(slots.wrapping_add(offset(current))) == EMPTY {
        return;
    }
    loop {
        let slots = e.mem.u32(map);
        let key = e.mem.u32(slots.wrapping_add(offset(current)));
        let home = hash(key) & mask;
        let moves = (current < first_after || home <= hole)
            && (hole <= current || (home <= hole && current < home))
            && (home <= hole || first_after <= home);
        if moves {
            e.mem.set_u32(slots.wrapping_add(offset(hole)), key);
            let slots = e.mem.u32(map);
            for word in 1..=value_words {
                let value = e.mem.u32(slots.wrapping_add(offset(current)) + 4 * word);
                e.mem
                    .set_u32(slots.wrapping_add(offset(hole)) + 4 * word, value);
            }
            let slots = e.mem.u32(map);
            e.mem.set_u32(slots.wrapping_add(offset(current)), EMPTY);
            hole = current;
        }
        let mask = e.mem.u32(map + 8);
        current = current.wrapping_add(1) & mask;
        let slots = e.mem.u32(map);
        if e.mem.u32(slots.wrapping_add(offset(current))) == EMPTY {
            break;
        }
    }
}

/// The slot bytes `reserve` and `00cb17f0` work out for `count` elements: twice `count`, taken
/// up to a power of two that is at least 8.
fn slot_count_for(count: i32) -> i32 {
    let wanted = count.wrapping_add(count);
    let mut slots = 8i32;
    if wanted > slots {
        loop {
            slots = slots.wrapping_add(slots);
            if slots >= wanted {
                break;
            }
        }
    }
    slots
}

// Translated from 00cb17e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::getMemSize` (Xbox
/// PDB), `fastcall`: the size of the slots in bytes, `(m_hashMod + 1) * 8`.
pub fn hk_map_base_u32_get_mem_size(e: &mut Engine, this: Ptr<HkMapBase>) -> u32 {
    (e.get(this, HkMapBase::m_hashMod) as u32)
        .wrapping_mul(8)
        .wrapping_add(8)
}

// Translated from 00cb17f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (cdecl `(count)`): the size in bytes of the slots of the 32-bit map for
/// `count` elements: the smallest power of two that is at least 8 and at least twice `count`,
/// times 8. (Like the exe's signed loop, a `count` above 0x20000000 never ends.)
pub fn fn_00cb17f0(_e: &mut Engine, count: i32) -> u32 {
    (slot_count_for(count) as u32) << SLOT_SHIFT_32
}

// Translated from 00cb1830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::hkMapBase` (Xbox PDB):
/// sets the map up over a buffer of `byte_size` bytes at `storage`: `m_elem` is the buffer,
/// `m_numElems` is `0x80000000` (empty, not owned), `m_hashMod` is the slot count (`byte_size / 8`)
/// minus one, and every slot's key is marked empty. Answers `this`.
pub fn hk_map_base_u32_construct(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    storage: u32,
    byte_size: u32,
) -> Ptr<HkMapBase> {
    let slots = byte_size >> 3;
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1) as i32);
    e.set(this, HkMapBase::m_elem, storage);
    e.set(this, HkMapBase::m_numElems, NOT_OWNED);
    if (slots as i32) > 0 {
        for slot in 0..slots {
            let storage = e.get(this, HkMapBase::m_elem);
            e.mem
                .set_u32(storage.wrapping_add(slot.wrapping_mul(8)), EMPTY);
        }
    }
    this
}

// Translated from 00cb1870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::insert` (Xbox PDB):
/// stores the pair, replacing the value of an equal key. First grows the table
/// (`resizeTable`, through `allocator`, to `2 * m_hashMod + 2` slots) when twice the element count
/// exceeds `m_hashMod`. The element count rises by one for a new key.
pub fn hk_map_base_u32_insert(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key: u32,
    value: u32,
) {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let mask = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > mask {
        let slots = mask.wrapping_add(mask).wrapping_add(2);
        hk_map_base_u32_resize_table(e, this, allocator, slots);
    }
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;
    let slots = e.get(this, HkMapBase::m_elem);
    let mut slot = hash(key) & mask;
    let mut added = 1u32;
    if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) != EMPTY {
        loop {
            if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) == key {
                added = 0;
                break;
            }
            slot = slot.wrapping_add(1) & mask;
            if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) == EMPTY {
                break;
            }
        }
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word.wrapping_add(added));
    e.mem.set_u32(slots.wrapping_add(slot.wrapping_mul(8)), key);
    let slots = e.get(this, HkMapBase::m_elem);
    e.mem
        .set_u32(slots.wrapping_add(slot.wrapping_mul(8)) + 4, value);
}

// Translated from 00cb18f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::findKey` (Xbox PDB):
/// the slot index holding the key, or `m_hashMod + 1` when it is not in the map (also when
/// `m_hashMod` is not positive).
pub fn hk_map_base_u32_find_key(e: &mut Engine, this: Ptr<HkMapBase>, key: u32) -> u32 {
    find_key_word(e, this.addr(), key, SLOT_SHIFT_32)
}

// Translated from 00cb1930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::getWithDefault` (Xbox
/// PDB), `RET 8`: the value stored for `key`, or `default` when the key is not in the map.
pub fn hk_map_base_u32_get_with_default(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key: u32,
    default: u32,
) -> u32 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let slots = e.get(this, HkMapBase::m_elem);
        let mask = mask as u32;
        let mut slot = hash(key) & mask;
        let mut found = e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8)));
        while found != EMPTY {
            if found == key {
                return e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8)) + 4);
            }
            slot = slot.wrapping_add(1) & mask;
            found = e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8)));
        }
    }
    default
}

// Translated from 00cb1980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::get` (Xbox PDB),
/// `RET 8`: when `key` is in the map stores its value through `out` and answers 0, else
/// answers 1.
pub fn hk_map_base_u32_get(e: &mut Engine, this: Ptr<HkMapBase>, key: u32, out: Ptr) -> u32 {
    let slot = hk_map_base_u32_find_key(e, this, key);
    if (slot as i32) <= e.get(this, HkMapBase::m_hashMod) {
        let slots = e.get(this, HkMapBase::m_elem);
        let value = e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8)) + 4);
        e.mem.set_u32(out.addr(), value);
        return 0;
    }
    1
}

// Translated from 00cb19b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::remove` (Xbox PDB),
/// `RET 4`: removes the element in slot `index` (see [`remove_slot`]).
pub fn hk_map_base_u32_remove(e: &mut Engine, this: Ptr<HkMapBase>, index: u32) {
    remove_slot(e, this.addr(), index, SLOT_SHIFT_32, 1);
}

// Translated from 00cb1a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (`fastcall`): empties every slot of the 32-bit map and zeroes the element
/// count, keeping the not-owned flag in bit 31.
pub fn fn_00cb1a60(e: &mut Engine, this: Ptr<HkMapBase>) {
    let slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let mut slot = 0i32;
    if slots > 0 {
        loop {
            let storage = e.get(this, HkMapBase::m_elem);
            e.mem
                .set_u32(storage.wrapping_add((slot as u32).wrapping_mul(8)), EMPTY);
            slot += 1;
            if slot >= slots {
                break;
            }
        }
    }
    let count_word = e.get(this, HkMapBase::m_numElems);
    e.set(this, HkMapBase::m_numElems, count_word & NOT_OWNED);
}

// Translated from 00cb1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::clearAndDeallocate`
/// (Xbox PDB), `RET 4`: empties the map (`00cb1a60`), then, unless the slots are not owned, returns
/// them to `allocator` (vtable slot 8, the slots and their size in bytes).
pub fn hk_map_base_u32_clear_and_deallocate(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr) {
    fn_00cb1a60(e, this);
    if e.get(this, HkMapBase::m_numElems) & NOT_OWNED == 0 {
        let bytes = hk_map_base_u32_get_mem_size(e, this);
        let storage = e.get(this, HkMapBase::m_elem);
        e.vcall(allocator.addr(), 8, &args![storage, bytes]);
    }
}

// Translated from 00cb1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::resizeTable` (Xbox
/// PDB), `RET 8` `(allocator, slots)`: gives the map `max(slots, 8)` slots from `allocator`
/// (vtable slot 4, the size in bytes), marks them empty, clears the element count (and the
/// not-owned flag), then inserts every element of the old table again and, if the old slots were
/// owned, returns them to the allocator (vtable slot 8, the slots and their size).
pub fn hk_map_base_u32_resize_table(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    slots: i32,
) {
    let slots = if slots > 8 { slots } else { 8 };
    let was_not_owned = e.get(this, HkMapBase::m_numElems) & NOT_OWNED;
    let old_storage = e.get(this, HkMapBase::m_elem);
    let old_slots = e.get(this, HkMapBase::m_hashMod).wrapping_add(1);
    let storage = e
        .vcall(allocator.addr(), 4, &args![(slots as u32) << SLOT_SHIFT_32])
        .u32();
    e.set(this, HkMapBase::m_elem, storage);
    let mut slot = 0i32;
    if slots > 0 {
        loop {
            let storage = e.get(this, HkMapBase::m_elem);
            e.mem
                .set_u32(storage.wrapping_add((slot as u32).wrapping_mul(8)), EMPTY);
            slot += 1;
            if slot >= slots {
                break;
            }
        }
    }
    e.set(this, HkMapBase::m_hashMod, slots.wrapping_sub(1));
    e.set(this, HkMapBase::m_numElems, 0);
    let mut old = 0i32;
    if old_slots > 0 {
        loop {
            let address = old_storage.wrapping_add((old as u32).wrapping_mul(8));
            let key = e.mem.u32(address);
            if key != EMPTY {
                let value = e.mem.u32(address + 4);
                hk_map_base_u32_insert(e, this, allocator, key, value);
            }
            old += 1;
            if old >= old_slots {
                break;
            }
        }
    }
    if was_not_owned == 0 {
        e.vcall(
            allocator.addr(),
            8,
            &args![old_storage, (old_slots as u32) << SLOT_SHIFT_32],
        );
    }
}

// Translated from 00cb1cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned __int64>>::getWithDefault`
/// (Xbox PDB), `RET 0x10` `(key low, key high, default low, default high)`, the 64-bit result in
/// `EDX:EAX`: the value stored for the key, or the default when the key is not in the map.
pub fn hk_map_base_u64_get_with_default(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    key_low: u32,
    key_high: u32,
    default_low: u32,
    default_high: u32,
) -> u64 {
    let mask = e.get(this, HkMapBase::m_hashMod);
    if mask > 0 {
        let slots = e.get(this, HkMapBase::m_elem);
        let mask = mask as u32;
        let mut slot = hash(key_low);
        loop {
            slot &= mask;
            let address = slots.wrapping_add(slot.wrapping_mul(SLOT_SIZE));
            let (low, high) = (e.mem.u32(address), e.mem.u32(address + 4));
            if is_empty(low, high) {
                break;
            }
            if low == key_low && high == key_high {
                let value_low = e.mem.u32(address + 8);
                let value_high = e.mem.u32(address + 0xc);
                return u64::from(value_low) | u64::from(value_high) << 32;
            }
            slot = slot.wrapping_add(1);
        }
    }
    u64::from(default_low) | u64::from(default_high) << 32
}

// Translated from 00cb1f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::findOrInsertKey` (Xbox
/// PDB), `RET 0xc` `(allocator, key, value)`: grows the table like `insert` when needed, then
/// stores the pair only when the key is not in the map yet (an equal key keeps its value).
/// Answers the key's slot (the iterator): `EAX` holds it at both `RET 0xc`s, and the one
/// caller, `findOrCopy`, counts the slot's value up through it (`00dc46e8`).
pub fn hk_map_base_u32_find_or_insert_key(
    e: &mut Engine,
    this: Ptr<HkMapBase>,
    allocator: Ptr,
    key: u32,
    value: u32,
) -> u32 {
    let count = (e.get(this, HkMapBase::m_numElems) & 0x7fff_ffff) as i32;
    let mask = e.get(this, HkMapBase::m_hashMod);
    if count.wrapping_add(count) > mask {
        let slots = mask.wrapping_add(mask).wrapping_add(2);
        hk_map_base_u32_resize_table(e, this, allocator, slots);
    }
    let mask = e.get(this, HkMapBase::m_hashMod) as u32;
    let slots = e.get(this, HkMapBase::m_elem);
    let mut slot = hash(key) & mask;
    if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) == key {
        return slot; // 00cb1f81 JZ 00cb1fa9 .. 00cb1fac RET 0xc: EAX is the home slot
    }
    loop {
        if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) == EMPTY {
            e.mem.set_u32(slots.wrapping_add(slot.wrapping_mul(8)), key);
            let slots = e.get(this, HkMapBase::m_elem);
            e.mem
                .set_u32(slots.wrapping_add(slot.wrapping_mul(8)) + 4, value);
            let count_word = e.get(this, HkMapBase::m_numElems);
            e.set(this, HkMapBase::m_numElems, count_word.wrapping_add(1));
            return slot; // 00cb1f99 .. 00cb1fac RET 0xc: EAX is still the slot
        }
        slot = slot.wrapping_add(1) & mask;
        if e.mem.u32(slots.wrapping_add(slot.wrapping_mul(8))) == key {
            return slot; // 00cb1f91 JNZ not taken .. 00cb1f96 RET 0xc: EAX is the slot
        }
    }
}

// Translated from 00cb1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (`RET 4`): removes `key` from the 32-bit map when it is there and answers 0,
/// else answers 1.
pub fn fn_00cb1fb0(e: &mut Engine, this: Ptr<HkMapBase>, key: u32) -> u32 {
    let slot = hk_map_base_u32_find_key(e, this, key);
    if (slot as i32) <= e.get(this, HkMapBase::m_hashMod) {
        hk_map_base_u32_remove(e, this, slot);
        return 0;
    }
    1
}

// Translated from 00cb1fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::reserve` (Xbox PDB),
/// `RET 8` `(allocator, count)`: rebuilds the table (`resizeTable`) with the smallest power of two
/// that is at least 8 and at least twice `count`, as the exe tail-jumps to it.
pub fn hk_map_base_u32_reserve(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, count: i32) {
    hk_map_base_u32_resize_table(e, this, allocator, slot_count_for(count));
}

// Translated from 00cb2000 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (`RET 8`): removes the 64-bit key `(key_low, key_high)` from the 64-bit map
/// when it is there and answers 0, else answers 1.
pub fn fn_00cb2000(e: &mut Engine, this: Ptr<HkMapBase>, key_low: u32, key_high: u32) -> u32 {
    let slot = hk_map_base_find_key(e, this, key_low, key_high);
    if (slot as i32) <= e.get(this, HkMapBase::m_hashMod) {
        fn_00cb1d20(e, this, slot);
        return 0;
    }
    1
}

// Translated from 00cb2030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMapBase<unsigned __int64,unsigned __int64,hkMapOperations<unsigned __int64>>::reserve` (Xbox
/// PDB), `RET 8` `(allocator, count)`: rebuilds the 64-bit map's table (`resizeTable`) with the
/// smallest power of two that is at least 8 and at least twice `count`, as the exe tail-jumps to it.
pub fn hk_map_base_u64_reserve(e: &mut Engine, this: Ptr<HkMapBase>, allocator: Ptr, count: i32) {
    hk_map_base_resize_table(e, this, allocator, slot_count_for(count));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb1810, fn_00cb1810(i32) -> u32),
        entry!(0x00cb1b80, fn_00cb1b80(Ptr<HkMapBase>, u32, u32)),
        entry!(
            0x00cb1bc0,
            hk_map_base_insert(Ptr<HkMapBase>, Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00cb1c60,
            hk_map_base_find_key(Ptr<HkMapBase>, u32, u32) -> u32
        ),
        entry!(0x00cb1d20, fn_00cb1d20(Ptr<HkMapBase>, u32)),
        entry!(0x00cb1e00, fn_00cb1e00(Ptr<HkMapBase>)),
        entry!(
            0x00cb1e40,
            hk_map_base_clear_and_deallocate(Ptr<HkMapBase>, Ptr)
        ),
        entry!(
            0x00cb1e70,
            hk_map_base_resize_table(Ptr<HkMapBase>, Ptr, i32)
        ),
        entry!(
            0x00cb17e0,
            hk_map_base_u32_get_mem_size(Ptr<HkMapBase>) -> u32
        ),
        entry!(0x00cb17f0, fn_00cb17f0(i32) -> u32),
        entry!(
            0x00cb1830,
            hk_map_base_u32_construct(Ptr<HkMapBase>, u32, u32) -> Ptr<HkMapBase>
        ),
        entry!(
            0x00cb1870,
            hk_map_base_u32_insert(Ptr<HkMapBase>, Ptr, u32, u32)
        ),
        entry!(
            0x00cb18f0,
            hk_map_base_u32_find_key(Ptr<HkMapBase>, u32) -> u32
        ),
        entry!(
            0x00cb1930,
            hk_map_base_u32_get_with_default(Ptr<HkMapBase>, u32, u32) -> u32
        ),
        entry!(
            0x00cb1980,
            hk_map_base_u32_get(Ptr<HkMapBase>, u32, Ptr) -> u32
        ),
        entry!(0x00cb19b0, hk_map_base_u32_remove(Ptr<HkMapBase>, u32)),
        entry!(0x00cb1a60, fn_00cb1a60(Ptr<HkMapBase>)),
        entry!(
            0x00cb1a90,
            hk_map_base_u32_clear_and_deallocate(Ptr<HkMapBase>, Ptr)
        ),
        entry!(
            0x00cb1ac0,
            hk_map_base_u32_resize_table(Ptr<HkMapBase>, Ptr, i32)
        ),
        entry!(
            0x00cb1cb0,
            hk_map_base_u64_get_with_default(Ptr<HkMapBase>, u32, u32, u32, u32) -> u64
        ),
        // Answers the slot: EAX at both RET 0xc (00cb1f96, 00cb1fac).
        entry!(
            0x00cb1f40,
            hk_map_base_u32_find_or_insert_key(Ptr<HkMapBase>, Ptr, u32, u32) -> u32
        ),
        entry!(0x00cb1fb0, fn_00cb1fb0(Ptr<HkMapBase>, u32) -> u32),
        entry!(
            0x00cb1fe0,
            hk_map_base_u32_reserve(Ptr<HkMapBase>, Ptr, i32)
        ),
        entry!(0x00cb2000, fn_00cb2000(Ptr<HkMapBase>, u32, u32) -> u32),
        entry!(
            0x00cb2030,
            hk_map_base_u64_reserve(Ptr<HkMapBase>, Ptr, i32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The allocator double's calls: (slot, first word, second word).
    type AllocatorLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// An allocator whose vtable slot 4 hands out blocks and slot 8 frees them;
    /// both are logged.
    fn allocator(e: &mut Engine) -> (Ptr, AllocatorLog) {
        let log: AllocatorLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        let allocate_log = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            allocate_log.borrow_mut().push((4, a[1], 0));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let free_log = Rc::clone(&log);
        e.register_double(0x0900_0008, move |_, a| {
            free_log.borrow_mut().push((8, a[1], a[2]));
            Ret::default()
        });
        (Ptr::new(object), log)
    }

    /// A map over a fresh `slots`-slot buffer it does not own.
    fn buffer_map(e: &mut Engine, slots: u32) -> Ptr<HkMapBase> {
        let this = e.new_object::<HkMapBase>();
        let storage = e.mem.alloc(slots * 16);
        e.call(0x00cb_1b80, &args![this, storage, slots * 16]);
        this
    }

    fn insert(e: &mut Engine, map: Ptr<HkMapBase>, alloc: Ptr, key: (u32, u32), value: (u32, u32)) {
        e.call(
            0x00cb_1bc0,
            &args![map, alloc, key.0, key.1, value.0, value.1],
        );
    }

    fn find(e: &mut Engine, map: Ptr<HkMapBase>, key: (u32, u32)) -> u32 {
        e.call(0x00cb_1c60, &args![map, key.0, key.1]).u32()
    }

    fn value_at(e: &Engine, map: Ptr<HkMapBase>, slot: u32) -> (u32, u32) {
        let address = slot_address(e, map, slot);
        (e.mem.u32(address + 8), e.mem.u32(address + 0xc))
    }

    /// `n` keys whose home slot in a table of `mask + 1` slots is `home`.
    fn keys_with_home(mask: u32, home: u32, n: usize) -> Vec<u32> {
        (0u32..)
            .map(|k| k << 4)
            .filter(|&key| hash(key) & mask == home)
            .take(n)
            .collect()
    }

    #[test]
    fn slot_bytes_is_a_power_of_two_slot_count() {
        let mut e = Engine::new();
        for (count, bytes) in [(0, 128), (4, 128), (5, 256), (100, 4096)] {
            assert_eq!(e.call(0x00cb_1810, &args![count]).u32(), bytes);
        }
    }

    #[test]
    fn buffer_setup_marks_every_slot_empty() {
        let mut e = Engine::new();
        let this = e.new_object::<HkMapBase>();
        let storage = e.mem.alloc(64);
        e.mem.write(storage, &[0u8; 64]);
        e.call(0x00cb_1b80, &args![this, storage, 64u32]);
        assert_eq!(e.get(this, HkMapBase::m_elem), storage);
        assert_eq!(e.get(this, HkMapBase::m_numElems), 0x8000_0000);
        assert_eq!(e.get(this, HkMapBase::m_hashMod), 3);
        for slot in 0..4 {
            assert_eq!(e.mem.u32(storage + 16 * slot), 0xffff_ffff);
            assert_eq!(e.mem.u32(storage + 16 * slot + 4), 0xffff_ffff);
        }
    }

    #[test]
    fn insert_and_find_key() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (111, 1));
        insert(&mut e, map, alloc, (0x20, 5), (222, 2));
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0002);
        let first = find(&mut e, map, (0x10, 0));
        let second = find(&mut e, map, (0x20, 5));
        assert_eq!(value_at(&e, map, first), (111, 1));
        assert_eq!(value_at(&e, map, second), (222, 2));
        // Same low word, other high word: not present; so is an absent key.
        assert_eq!(find(&mut e, map, (0x20, 6)), 8);
        assert_eq!(find(&mut e, map, (0x30, 0)), 8);
        // An equal key replaces the value and does not count again.
        insert(&mut e, map, alloc, (0x10, 0), (333, 3));
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0002);
        assert_eq!(value_at(&e, map, first), (333, 3));
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn find_key_in_a_map_without_slots_returns_one_past_the_mask() {
        let mut e = Engine::new();
        let map = e.new_object::<HkMapBase>();
        assert_eq!(find(&mut e, map, (0x10, 0)), 1);
    }

    #[test]
    fn colliding_keys_probe_the_next_slot() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        let keys = keys_with_home(7, 2, 3);
        for (i, key) in keys.iter().enumerate() {
            insert(&mut e, map, alloc, (*key, 0), (i as u32, 0));
        }
        let slots: Vec<u32> = keys.iter().map(|k| find(&mut e, map, (*k, 0))).collect();
        assert_eq!(slots, vec![2, 3, 4]);
    }

    #[test]
    fn insert_grows_a_buffer_map_and_keeps_the_buffer() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        // A 4-slot buffer map (mask 3): the third insert finds 2 * 2 > 3.
        let map = buffer_map(&mut e, 4);
        let old = e.get(map, HkMapBase::m_elem);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        insert(&mut e, map, alloc, (0x20, 0), (2, 0));
        assert!(log.borrow().is_empty());
        insert(&mut e, map, alloc, (0x30, 0), (3, 0));
        // 2 * 3 + 2 = 8 slots from the allocator (128 bytes), no free of the buffer.
        assert_eq!(*log.borrow(), vec![(4, 128, 0)]);
        assert_ne!(e.get(map, HkMapBase::m_elem), old);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 7);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 3);
        for (key, value) in [(0x10, 1), (0x20, 2), (0x30, 3)] {
            let slot = find(&mut e, map, (key, 0));
            assert_eq!(value_at(&e, map, slot), (value, 0));
        }
    }

    #[test]
    fn resize_frees_owned_slots_with_their_size() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        // An owned table: clear the flag as an earlier resize would have.
        e.set(map, HkMapBase::m_numElems, 1);
        let old = e.get(map, HkMapBase::m_elem);
        // Fewer than 8 slots still gets 8, by the clamp.
        e.call(0x00cb_1e70, &args![map, alloc, 3i32]);
        assert_eq!(*log.borrow(), vec![(4, 128, 0), (8, old, 128)]);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 7);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 1);
        let slot = find(&mut e, map, (0x10, 0));
        assert_eq!(value_at(&e, map, slot), (1, 0));
    }

    #[test]
    fn remove_moves_displaced_elements_back() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 16);
        // Three keys with home slot 5 (slots 5, 6, 7) and one with home slot 6
        // that probed to slot 8.
        let same = keys_with_home(15, 5, 3);
        let other = keys_with_home(15, 6, 1)[0];
        for (i, key) in same.iter().enumerate() {
            insert(&mut e, map, alloc, (*key, 0), (i as u32 + 10, 0));
        }
        insert(&mut e, map, alloc, (other, 0), (99, 0));
        assert_eq!(find(&mut e, map, (other, 0)), 8);
        // Removing the first one (slot 5) shifts the later ones back.
        e.call(0x00cb_1d20, &args![map, 5u32]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0003);
        assert_eq!(find(&mut e, map, (same[0], 0)), 16);
        for (i, key) in same.iter().enumerate().skip(1) {
            let slot = find(&mut e, map, (*key, 0));
            assert_eq!(value_at(&e, map, slot), (i as u32 + 10, 0));
        }
        let slot = find(&mut e, map, (other, 0));
        assert_eq!(value_at(&e, map, slot), (99, 0));
        // Every element is still reachable from its home slot without a gap.
        for key in [same[1], same[2], other] {
            assert_ne!(find(&mut e, map, (key, 0)), 16);
        }
    }

    #[test]
    fn remove_leaves_an_element_in_its_own_home_slot() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 16);
        let first = keys_with_home(15, 5, 1)[0];
        let in_place = keys_with_home(15, 6, 1)[0];
        insert(&mut e, map, alloc, (first, 0), (1, 0));
        insert(&mut e, map, alloc, (in_place, 0), (2, 0));
        assert_eq!(find(&mut e, map, (in_place, 0)), 6);
        e.call(0x00cb_1d20, &args![map, 5u32]);
        // The element at slot 6 has its home there: it must not move to 5.
        assert_eq!(find(&mut e, map, (in_place, 0)), 6);
        assert_eq!(find(&mut e, map, (first, 0)), 16);
    }

    #[test]
    fn clear_keeps_only_the_not_owned_flag() {
        let mut e = Engine::new();
        let (alloc, _) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        insert(&mut e, map, alloc, (0x10, 0), (1, 0));
        e.call(0x00cb_1e00, &args![map]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0x8000_0000);
        assert_eq!(find(&mut e, map, (0x10, 0)), 8);
        e.set(map, HkMapBase::m_numElems, 5);
        e.call(0x00cb_1e00, &args![map]);
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0);
    }

    #[test]
    fn clear_and_deallocate_frees_only_owned_slots() {
        let mut e = Engine::new();
        let (alloc, log) = allocator(&mut e);
        let map = buffer_map(&mut e, 8);
        e.call(0x00cb_1e40, &args![map, alloc]);
        assert!(log.borrow().is_empty());
        e.set(map, HkMapBase::m_numElems, 0);
        let storage = e.get(map, HkMapBase::m_elem);
        e.call(0x00cb_1e40, &args![map, alloc]);
        assert_eq!(*log.borrow(), vec![(8, storage, 128)]);
    }
}

#[cfg(test)]
mod instance_tests {
    use super::*;

    const ALLOCATOR_VTABLE: u32 = 0x0128_5000;
    const ALLOC: u32 = 0x0900_6001;
    const FREE: u32 = 0x0900_6002;

    /// An allocator object whose slot 4 allocates `a[1]` zeroed bytes and slot 8 frees; both are
    /// logged through `call_log`.
    fn engine() -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.put_vtable(ALLOCATOR_VTABLE, &[0, ALLOC, FREE]);
        e.register(ALLOC, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(FREE, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.call_log = Some(vec![]);
        (e, Ptr::new(allocator))
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A 32-bit map over a buffer of `slots` slots (not owned).
    fn map(e: &mut Engine, slots: u32) -> (Ptr<HkMapBase>, u32) {
        let map = Ptr::<HkMapBase>::new(e.mem.alloc(0xc));
        let storage = e.mem.alloc(slots * 8);
        let answer = e.call(0x00cb_1830, &args![map, storage, slots * 8]).u32();
        assert_eq!(answer, map.addr());
        (map, storage)
    }

    fn insert(e: &mut Engine, map: Ptr<HkMapBase>, allocator: Ptr, key: u32, value: u32) {
        e.call(0x00cb_1870, &args![map, allocator, key, value]);
    }

    /// The (slot, key, value) of every occupied slot.
    fn contents(e: &Engine, map: Ptr<HkMapBase>) -> Vec<(u32, u32, u32)> {
        let slots = e.get(map, HkMapBase::m_elem);
        (0..=e.get(map, HkMapBase::m_hashMod) as u32)
            .filter_map(|slot| {
                let key = e.mem.u32(slots + 8 * slot);
                (key != EMPTY).then(|| (slot, key, e.mem.u32(slots + 8 * slot + 4)))
            })
            .collect()
    }

    /// A key whose home slot in a table with `mask` is `home`, scanning upward from `from`.
    fn key_with_home(mask: u32, home: u32, from: u32) -> u32 {
        (from..).find(|key| hash(*key) & mask == home).unwrap()
    }

    #[test]
    fn size_helpers() {
        let (mut e, _) = engine();
        let (map, _) = map(&mut e, 16);
        assert_eq!(e.call(0x00cb_17e0, &args![map]).u32(), 128);
        let mut bytes = |count: i32| e.call(0x00cb_17f0, &args![count]).u32();
        assert_eq!(bytes(0), 64);
        assert_eq!(bytes(4), 64);
        assert_eq!(bytes(5), 128);
        assert_eq!(bytes(100), 2048);
    }

    #[test]
    fn the_constructor_sets_up_a_buffer_the_map_does_not_own() {
        let (mut e, _) = engine();
        let map = Ptr::<HkMapBase>::new(e.mem.alloc(0xc));
        let storage = e.mem.alloc(64);
        e.mem.write(storage, &[0x5a; 64]);
        let answer = e.call(0x00cb_1830, &args![map, storage, 64u32]).u32();
        assert_eq!(answer, map.addr());
        assert_eq!(e.get(map, HkMapBase::m_elem), storage);
        assert_eq!(e.get(map, HkMapBase::m_numElems), NOT_OWNED);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 7);
        for slot in 0..8 {
            assert_eq!(e.mem.u32(storage + 8 * slot), EMPTY);
            assert_eq!(e.mem.u32(storage + 8 * slot + 4), 0x5a5a_5a5a);
        }
    }

    #[test]
    fn insert_find_get_and_the_default_lookup() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        // Keys with one home slot probe along the table.
        for (i, key) in [0x100u32, 0x101, 0x102].into_iter().enumerate() {
            insert(&mut e, map, allocator, key, 10 + i as u32);
        }
        let home = hash(0x100) & 7;
        let found: Vec<u32> = [0x100u32, 0x101, 0x102]
            .iter()
            .map(|key| e.call(0x00cb_18f0, &args![map, *key]).u32())
            .collect();
        assert_eq!(found, vec![home, (home + 1) & 7, (home + 2) & 7]);
        // A missing key answers the slot count (m_hashMod + 1).
        assert_eq!(e.call(0x00cb_18f0, &args![map, 0x103u32]).u32(), 8);
        // Replacing the value of an equal key keeps the count.
        insert(&mut e, map, allocator, 0x101, 99);
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 3);
        assert_eq!(e.call(0x00cb_1930, &args![map, 0x101u32, 7u32]).u32(), 99);
        assert_eq!(e.call(0x00cb_1930, &args![map, 0x103u32, 7u32]).u32(), 7);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        assert_eq!(e.call(0x00cb_1980, &args![map, 0x102u32, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 12);
        assert_eq!(e.call(0x00cb_1980, &args![map, 0x103u32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 12);
    }

    #[test]
    fn a_map_without_slots_finds_nothing() {
        let (mut e, _) = engine();
        let map = Ptr::<HkMapBase>::new(e.mem.alloc(0xc));
        e.set(map, HkMapBase::m_hashMod, 0);
        assert_eq!(e.call(0x00cb_18f0, &args![map, 5u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_1930, &args![map, 5u32, 77u32]).u32(), 77);
    }

    #[test]
    fn insert_grows_the_table_when_it_is_more_than_half_full() {
        let (mut e, allocator) = engine();
        let (map, buffer) = map(&mut e, 8);
        for i in 0..5u32 {
            insert(&mut e, map, allocator, 0x40 + 0x10 * i, i);
        }
        // The fifth insert found 2 * 4 > 7 and grew the table to 16 slots; the old buffer was
        // not the map's, so it was not freed.
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 15);
        assert_eq!(calls(&e, ALLOC), vec![vec![allocator.addr(), 128]]);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(e.get(map, HkMapBase::m_numElems), 5);
        assert_ne!(e.get(map, HkMapBase::m_elem), buffer);
        assert_eq!(contents(&e, map).len(), 5);
        for i in 0..5u32 {
            let key = 0x40 + 0x10 * i;
            assert_eq!(e.call(0x00cb_1930, &args![map, key, 99u32]).u32(), i);
        }
        // The next growth (the ninth element) frees the owned 16-slot storage.
        let owned = e.get(map, HkMapBase::m_elem);
        for i in 5..9u32 {
            insert(&mut e, map, allocator, 0x40 + 0x10 * i, i);
        }
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 31);
        assert_eq!(calls(&e, FREE), vec![vec![allocator.addr(), owned, 128]]);
        assert_eq!(contents(&e, map).len(), 9);
    }

    #[test]
    fn removing_an_element_moves_later_ones_back_into_the_hole() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        let (a, b, c) = (0x100u32, 0x101, 0x102);
        for (i, key) in [a, b, c].into_iter().enumerate() {
            insert(&mut e, map, allocator, key, 1 + i as u32);
        }
        let home = hash(a) & 7;
        // Remove the middle one: the one after it, whose home is before the hole, moves up.
        let slot = e.call(0x00cb_18f0, &args![map, b]).u32();
        e.call(0x00cb_19b0, &args![map, slot]);
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 2);
        let mut expected = vec![(home, a, 1), ((home + 1) & 7, c, 3)];
        expected.sort();
        assert_eq!(contents(&e, map), expected);
    }

    #[test]
    fn removing_leaves_an_element_that_sits_in_its_own_home_slot() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        let (a, b) = (0x100u32, 0x101);
        let home = hash(a) & 7;
        // An element whose home is the third slot of the cluster.
        let d = key_with_home(7, (home + 2) & 7, 0x200);
        for (i, key) in [a, b, d].into_iter().enumerate() {
            insert(&mut e, map, allocator, key, 1 + i as u32);
        }
        let slot = e.call(0x00cb_18f0, &args![map, a]).u32();
        e.call(0x00cb_19b0, &args![map, slot]);
        // b moved into the hole; d stayed.
        let mut expected = vec![(home, b, 2), ((home + 2) & 7, d, 3)];
        expected.sort();
        assert_eq!(contents(&e, map), expected);
    }

    #[test]
    fn remove_by_key_answers_whether_the_key_was_there() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        insert(&mut e, map, allocator, 0x100, 5);
        assert_eq!(e.call(0x00cb_1fb0, &args![map, 0x101u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_1fb0, &args![map, 0x100u32]).u32(), 0);
        assert!(contents(&e, map).is_empty());
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 0);
    }

    #[test]
    fn clear_empties_the_slots_and_keeps_the_ownership_flag() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        insert(&mut e, map, allocator, 0x100, 5);
        e.call(0x00cb_1a60, &args![map]);
        assert!(contents(&e, map).is_empty());
        assert_eq!(e.get(map, HkMapBase::m_numElems), NOT_OWNED);
    }

    #[test]
    fn clear_and_deallocate_frees_only_storage_the_map_owns() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        insert(&mut e, map, allocator, 0x100, 5);
        e.call(0x00cb_1a90, &args![map, allocator]);
        assert!(calls(&e, FREE).is_empty());
        // After growing, the storage is owned.
        for i in 0..5u32 {
            insert(&mut e, map, allocator, 0x40 + 0x10 * i, i);
        }
        let owned = e.get(map, HkMapBase::m_elem);
        e.call(0x00cb_1a90, &args![map, allocator]);
        assert_eq!(calls(&e, FREE), vec![vec![allocator.addr(), owned, 128]]);
        assert!(contents(&e, map).is_empty());
        assert_eq!(e.get(map, HkMapBase::m_numElems), 0);
    }

    #[test]
    fn find_or_insert_key_keeps_the_value_of_an_existing_key() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        e.call(0x00cb_1f40, &args![map, allocator, 0x100u32, 1u32]);
        e.call(0x00cb_1f40, &args![map, allocator, 0x100u32, 2u32]);
        e.call(0x00cb_1f40, &args![map, allocator, 0x101u32, 3u32]);
        // The second key collides with the first and probes on; the first keeps its value.
        let home = hash(0x100) & 7;
        let mut expected = vec![(home, 0x100, 1), ((home + 1) & 7, 0x101, 3)];
        expected.sort();
        assert_eq!(contents(&e, map), expected);
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 2);
        // A key found after probing (not at the home slot) is found too.
        e.call(0x00cb_1f40, &args![map, allocator, 0x101u32, 9u32]);
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 2);
        // Enough keys grow the table like insert.
        for i in 0..4u32 {
            e.call(0x00cb_1f40, &args![map, allocator, 0x500 + 0x10 * i, i]);
        }
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 15);
    }

    #[test]
    fn find_or_insert_key_answers_the_slot_00cb1f40() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        let home = hash(0x100) & 7;
        let next = (home + 1) & 7;
        // Inserted at the home slot (00cb1fac), found there (00cb1f81); a colliding key is
        // inserted one slot on, then found after probing (00cb1f96).
        for (key, value, slot) in [
            (0x100u32, 1u32, home),
            (0x100, 2, home),
            (0x101, 3, next),
            (0x101, 4, next),
        ] {
            let answer = e
                .call(0x00cb_1f40, &args![map, allocator, key, value])
                .u32();
            assert_eq!(answer, slot);
        }
        let slots = e.get(map, HkMapBase::m_elem);
        assert_eq!(e.mem.u32(slots + 8 * next + 4), 3);
        // The answer indexes the pair (`findOrCopy` counts its value up through it, 00dc46e8),
        // also in a table the call has just grown.
        for key in [0x500u32, 0x510, 0x520, 0x530] {
            let answer = e.call(0x00cb_1f40, &args![map, allocator, key, 0u32]).u32();
            let slots = e.get(map, HkMapBase::m_elem);
            assert_eq!(e.mem.u32(slots + 8 * answer), key);
        }
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 15);
    }

    #[test]
    fn reserve_rebuilds_the_table_at_a_power_of_two() {
        let (mut e, allocator) = engine();
        let (map, _) = map(&mut e, 8);
        insert(&mut e, map, allocator, 0x100, 5);
        insert(&mut e, map, allocator, 0x110, 6);
        // Ten elements need 20 slots, taken up to 32.
        e.call(0x00cb_1fe0, &args![map, allocator, 10u32]);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 31);
        assert_eq!(calls(&e, ALLOC).last().unwrap()[1], 256);
        assert_eq!(contents(&e, map).len(), 2);
        assert_eq!(e.call(0x00cb_1930, &args![map, 0x110u32, 0u32]).u32(), 6);
    }

    /// A 64-bit map with the pairs ((1, 2) -> (3, 4)) and ((5, 6) -> (7, 8)).
    fn map64(e: &mut Engine, allocator: Ptr) -> Ptr<HkMapBase> {
        let map = Ptr::<HkMapBase>::new(e.mem.alloc(0xc));
        let storage = e.mem.alloc(8 * 16);
        e.call(0x00cb_1b80, &args![map, storage, 8 * 16u32]);
        for (key, value) in [((1u32, 2u32), (3u32, 4u32)), ((5, 6), (7, 8))] {
            e.call(
                0x00cb_1bc0,
                &args![map, allocator, key.0, key.1, value.0, value.1],
            );
        }
        map
    }

    #[test]
    fn the_64_bit_default_lookup_compares_both_key_words() {
        let (mut e, allocator) = engine();
        let map = map64(&mut e, allocator);
        let lookup = |e: &mut Engine, low: u32, high: u32| {
            e.call(0x00cb_1cb0, &args![map, low, high, 0xaau32, 0xbbu32])
                .u64()
        };
        assert_eq!(lookup(&mut e, 1, 2), 3 | 4 << 32);
        assert_eq!(lookup(&mut e, 5, 6), 7 | 8 << 32);
        // The same low word with another high word, and a key that is not there at all.
        assert_eq!(lookup(&mut e, 1, 3), 0xaa | 0xbb << 32);
        assert_eq!(lookup(&mut e, 9, 9), 0xaa | 0xbb << 32);
        // A map without slots answers the default.
        let empty = Ptr::<HkMapBase>::new(e.mem.alloc(0xc));
        assert_eq!(
            e.call(0x00cb_1cb0, &args![empty, 1u32, 2u32, 0x11u32, 0x22u32])
                .u64(),
            0x11 | 0x22 << 32
        );
    }

    #[test]
    fn the_64_bit_remove_by_key_and_reserve() {
        let (mut e, allocator) = engine();
        let map = map64(&mut e, allocator);
        assert_eq!(e.call(0x00cb_2000, &args![map, 1u32, 3u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_2000, &args![map, 1u32, 2u32]).u32(), 0);
        assert_eq!(e.get(map, HkMapBase::m_numElems) & 0x7fff_ffff, 1);
        let gone = e
            .call(0x00cb_1cb0, &args![map, 1u32, 2u32, 0u32, 0u32])
            .u64();
        assert_eq!(gone, 0);
        // Reserve for 10: 20 slots up to 32 (32 slots of 16 bytes).
        e.call(0x00cb_2030, &args![map, allocator, 10u32]);
        assert_eq!(e.get(map, HkMapBase::m_hashMod), 31);
        assert_eq!(calls(&e, ALLOC).last().unwrap()[1], 512);
        assert_eq!(
            e.call(0x00cb_1cb0, &args![map, 5u32, 6u32, 0u32, 0u32])
                .u64(),
            7 | 8 << 32
        );
    }
}
