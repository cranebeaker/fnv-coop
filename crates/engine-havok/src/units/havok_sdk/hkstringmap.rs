//! `Havok SDK/hkstringmap.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions belong to `hkCachedHashMap<hkStringMapOperations, hkContainerHeapAllocator>`:
//! a string-keyed open-addressing hash table. `m_elem` points at `hashMod + 1` slots of three
//! words laid out as three parallel tables: the stored hash words (the empty marker is
//! `0xffffffff`), then the key pointers, then the values. The hash of a key is
//! `h = h * 31 + (signed byte)` over its bytes, masked with `0x7fffffff`; a key's home slot is
//! `hash & hashMod` and collisions probe the next slot. The table is grown (`resizeTable`) by
//! `insert` when twice the element count exceeds `hashMod`.
//!
//! An *iterator* is a slot index: `getIterator` finds the first occupied slot and `getNext`
//! the next; `isValid` is `index <= hashMod`, so the sentinel one past the end is
//! `hashMod + 1`. Keys are compared with `hkString::strCmp` (`00cb0fa0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The calling thread's memory router (`00c85750`); its word at `+0x10` is the allocator whose
/// vtable slots are used here: `+4` allocates `(size)`, `+8` frees `(block, size)`.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkString::strCmp` (`00cb0fa0`, cdecl `(left, right)`, 0 when equal) and `memSet`
/// (`00cb1280`, cdecl `(destination, value, size)`).
const STR_CMP: u32 = 0x00cb_0fa0;
const MEM_SET: u32 = 0x00cb_1280;
/// The marker of an empty hash slot.
const EMPTY: u32 = 0xffff_ffff;
/// Bytes per slot: three words.
const SLOT_BYTES: u32 = 0xc;

layout! {
    /// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (Xbox PDB), 0x10
    /// bytes there (the empty `m_ops` member pads it; the code reads only the first three
    /// words).
    pub struct HkCachedHashMap: 0x10 {
        /// `m_elem` (Xbox PDB): the three parallel tables.
        0x00 m_elem: u32,
        /// `m_numElems` (Xbox PDB).
        0x04 m_numElems: i32,
        /// `m_hashMod` (Xbox PDB): the slot count minus one.
        0x08 m_hashMod: i32,
    }
}

/// The calling thread's allocator, the word at router `+0x10`.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The key hash the code computes: `h = h * 31 + (signed byte)` over the string, before the
/// `0x7fffffff` mask.
fn string_hash(e: &Engine, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut address = key;
    loop {
        let byte = e.mem.u8(address);
        if byte == 0 {
            break;
        }
        hash = (byte as i8 as i32 as u32).wrapping_add(hash.wrapping_mul(31));
        address = address.wrapping_add(1);
    }
    hash
}

/// The address of word `index` of the elements.
fn word_address(e: &Engine, this: Ptr<HkCachedHashMap>, index: u32) -> u32 {
    e.get(this, HkCachedHashMap::m_elem)
        .wrapping_add(index.wrapping_mul(4))
}

fn hash_mod(e: &Engine, this: Ptr<HkCachedHashMap>) -> u32 {
    e.get(this, HkCachedHashMap::m_hashMod) as u32
}

// Translated from 00d87410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::getIterator` (engine map name), `__fastcall` on the map, no stack words:
/// the first occupied slot (its hash word is not `0xffffffff`), or `hashMod + 1` when there is
/// none; 0 when `hashMod` is negative (no scan).
pub fn hk_cached_hash_map_get_iterator(e: &mut Engine, this: Ptr<HkCachedHashMap>) -> i32 {
    let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
    let mut index = 0i32;
    if hash_mod < 0 {
        return index;
    }
    loop {
        if e.mem.u32(word_address(e, this, index as u32)) != EMPTY {
            return index;
        }
        index += 1;
        if index > hash_mod {
            return index;
        }
    }
}

// Translated from 00d87430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::getKey` (engine map name), thiscall, one stack word `index`: the key
/// pointer of that slot, the word at `elem[hashMod + index + 1]`.
pub fn hk_cached_hash_map_get_key(e: &mut Engine, this: Ptr<HkCachedHashMap>, index: i32) -> u32 {
    let word = e
        .get(this, HkCachedHashMap::m_hashMod)
        .wrapping_add(index)
        .wrapping_add(1);
    e.mem.u32(word_address(e, this, word as u32))
}

// Translated from 00d87440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::getValue` (engine map name), thiscall, one stack word `index`: the value
/// of that slot, the word at `elem[index + 2 * hashMod + 2]`.
pub fn hk_cached_hash_map_get_value(e: &mut Engine, this: Ptr<HkCachedHashMap>, index: i32) -> u32 {
    let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
    let word = index.wrapping_add(hash_mod.wrapping_mul(2)).wrapping_add(2);
    e.mem.u32(word_address(e, this, word as u32))
}

// Translated from 00d87480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::getNext` (engine map name), thiscall, one stack word `index`: the next
/// occupied slot after `index`, or `hashMod + 1` when there is none (more precisely: the
/// first index above `index` that is occupied or past `hashMod`).
pub fn hk_cached_hash_map_get_next(e: &mut Engine, this: Ptr<HkCachedHashMap>, index: i32) -> i32 {
    let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
    let mut next = index.wrapping_add(1);
    if next > hash_mod {
        return next;
    }
    loop {
        if e.mem.u32(word_address(e, this, next as u32)) != EMPTY {
            return next;
        }
        next = next.wrapping_add(1);
        if next > hash_mod {
            return next;
        }
    }
}

// Translated from 00d874b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::isValid` (engine map name), thiscall, two stack words `(hkBool result,
/// index)`: stores whether `index <= hashMod` (signed). Returns the `hkBool` pointer.
pub fn hk_cached_hash_map_is_valid(
    e: &mut Engine,
    this: Ptr<HkCachedHashMap>,
    out: Ptr,
    index: i32,
) -> Ptr {
    let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
    e.mem.set_u8(out.addr(), u8::from(index <= hash_mod));
    out
}

// Translated from 00d874d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::insert` (engine map name), thiscall, two stack words `(key, value)`:
/// hashes the key; when twice the element count exceeds `hashMod` (signed) the table is first
/// resized to `hashMod * 2 + 2` slots (`resizeTable`). The probe starts at `hash & hashMod`
/// and walks (`+ 1`, masked) until an empty slot, or a slot with the same hash word whose key
/// compares equal (`strCmp == 0`): an empty slot counts a new element, an equal key is
/// overwritten. The hash word, key pointer and value are stored. (The exe leaves the value
/// in EAX.)
pub fn hk_cached_hash_map_insert(e: &mut Engine, this: Ptr<HkCachedHashMap>, key: Ptr, value: u32) {
    let hash = string_hash(e, key.addr()) & 0x7fff_ffff;
    let count = e.get(this, HkCachedHashMap::m_numElems);
    let mask = e.get(this, HkCachedHashMap::m_hashMod);
    if count.wrapping_add(count) > mask {
        fn_00d87860(e, this, mask.wrapping_mul(2).wrapping_add(2));
    }
    let mut slot = hash_mod(e, this) & hash;
    if e.mem.u32(word_address(e, this, slot)) != EMPTY {
        loop {
            if e.mem.u32(word_address(e, this, slot)) == hash {
                let key_word = hash_mod(e, this).wrapping_add(slot).wrapping_add(1);
                let stored = e.mem.u32(word_address(e, this, key_word));
                if e.call(STR_CMP, &args![key, stored]).i32() == 0 {
                    break;
                }
            }
            slot = slot.wrapping_add(1) & hash_mod(e, this);
            if e.mem.u32(word_address(e, this, slot)) == EMPTY {
                let count = e.get(this, HkCachedHashMap::m_numElems);
                e.set(this, HkCachedHashMap::m_numElems, count.wrapping_add(1));
                break;
            }
        }
    } else {
        let count = e.get(this, HkCachedHashMap::m_numElems);
        e.set(this, HkCachedHashMap::m_numElems, count.wrapping_add(1));
    }
    let mask = hash_mod(e, this);
    let hash_address = word_address(e, this, slot);
    e.mem.set_u32(hash_address, hash);
    let key_address = word_address(e, this, mask.wrapping_add(slot).wrapping_add(1));
    e.mem.set_u32(key_address, key.addr());
    let value_address = word_address(
        e,
        this,
        slot.wrapping_add(mask.wrapping_mul(2)).wrapping_add(2),
    );
    e.mem.set_u32(value_address, value);
}

// Translated from 00d87580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the key lookup; thiscall, one stack word `key`): the slot of
/// `key`: the probe starts at `hash & hashMod` and ends at an empty slot, which answers
/// `hashMod + 1`, or at a slot with the same hash word and an equal key (`strCmp == 0`),
/// which answers its index.
pub fn fn_00d87580(e: &mut Engine, this: Ptr<HkCachedHashMap>, key: Ptr) -> i32 {
    let hash = string_hash(e, key.addr()) & 0x7fff_ffff;
    let mut slot = hash_mod(e, this) & hash;
    let mut found = e.mem.u32(word_address(e, this, slot));
    loop {
        if found == EMPTY {
            return e.get(this, HkCachedHashMap::m_hashMod).wrapping_add(1);
        }
        if found == hash {
            let key_word = hash_mod(e, this).wrapping_add(slot).wrapping_add(1);
            let stored = e.mem.u32(word_address(e, this, key_word));
            if e.call(STR_CMP, &args![key, stored]).i32() == 0 {
                return slot as i32;
            }
        }
        slot = slot.wrapping_add(1) & hash_mod(e, this);
        found = e.mem.u32(word_address(e, this, slot));
    }
}

// Translated from 00d87600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (find or insert; thiscall, two stack words `(key, value)`):
/// looks `key` up (`00d87580`); when the slot is valid (`isValid`) returns its stored value
/// (`getValue`), otherwise inserts `(key, value)` and returns `value`.
pub fn fn_00d87600(e: &mut Engine, this: Ptr<HkCachedHashMap>, key: Ptr, value: u32) -> u32 {
    let index = fn_00d87580(e, this, key);
    let valid = e.with_stack(4, |e, out| {
        let out = hk_cached_hash_map_is_valid(e, this, out, index);
        e.mem.u8(out.addr()) != 0
    });
    if valid {
        return hk_cached_hash_map_get_value(e, this, index);
    }
    hk_cached_hash_map_insert(e, this, key, value);
    value
}

// Translated from 00d87650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the lookup that gives the value through a pointer; thiscall,
/// two stack words `(key, out)`): when `key` is present stores its value at `out` and returns
/// 0; otherwise returns 1 and leaves `out` alone.
pub fn fn_00d87650(e: &mut Engine, this: Ptr<HkCachedHashMap>, key: Ptr, out: Ptr) -> u32 {
    let index = fn_00d87580(e, this, key);
    let valid = e.with_stack(4, |e, flag| {
        let flag = hk_cached_hash_map_is_valid(e, this, flag, index);
        e.mem.u8(flag.addr()) != 0
    });
    if valid {
        let value = hk_cached_hash_map_get_value(e, this, index);
        e.mem.set_u32(out.addr(), value);
        return 0;
    }
    1
}

// Translated from 00d876a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (removes the entry at a slot; thiscall, one stack word
/// `hole`): the element count drops by one and the slot's hash word is emptied. The cluster
/// start is found by walking back (`slot - 1`, masked) from the hole to the nearest empty
/// slot, plus one (`cluster`); then each following slot of the run (`hole + 1`, masked, until
/// an empty one) whose home slot (`hash & hashMod`, `home`) does not lie in the cyclic range
/// `(hole, slot]` is moved into the hole (hash word, key and value; its own hash word
/// emptied) and becomes the new hole. The three conditions are the code's unsigned compares.
pub fn fn_00d876a0(e: &mut Engine, this: Ptr<HkCachedHashMap>, hole: u32) {
    let count = e.get(this, HkCachedHashMap::m_numElems);
    e.set(this, HkCachedHashMap::m_numElems, count.wrapping_sub(1));
    let hole_address = word_address(e, this, hole);
    e.mem.set_u32(hole_address, EMPTY);
    let mask = hash_mod(e, this);
    let mut walk = mask.wrapping_add(hole) & mask;
    if e.mem.u32(word_address(e, this, walk)) != EMPTY {
        loop {
            walk = walk.wrapping_add(mask) & mask;
            if e.mem.u32(word_address(e, this, walk)) == EMPTY {
                break;
            }
        }
    }
    let cluster = walk.wrapping_add(1) & mask;
    let mut hole = hole;
    let mut slot = hole.wrapping_add(1) & mask;
    if e.mem.u32(word_address(e, this, slot)) == EMPTY {
        return;
    }
    loop {
        let hash = e.mem.u32(word_address(e, this, slot));
        let home = hash & mask;
        let before_cluster = slot < cluster || home <= hole;
        let after_hole = hole <= slot || (home <= hole && slot < home);
        let outside = home <= hole || cluster <= home;
        if before_cluster && after_hole && outside {
            let hole_hash = word_address(e, this, hole);
            e.mem.set_u32(hole_hash, hash);
            let mask = hash_mod(e, this);
            let key = e.mem.u32(word_address(
                e,
                this,
                mask.wrapping_add(slot).wrapping_add(1),
            ));
            let hole_key = word_address(e, this, mask.wrapping_add(hole).wrapping_add(1));
            e.mem.set_u32(hole_key, key);
            let mask = hash_mod(e, this);
            let double = mask.wrapping_add(mask);
            let value = e.mem.u32(word_address(
                e,
                this,
                double.wrapping_add(slot).wrapping_add(2),
            ));
            let hole_value = word_address(e, this, double.wrapping_add(hole).wrapping_add(2));
            e.mem.set_u32(hole_value, value);
            let slot_hash = word_address(e, this, slot);
            e.mem.set_u32(slot_hash, EMPTY);
            hole = slot;
        }
        slot = slot.wrapping_add(1) & hash_mod(e, this);
        if e.mem.u32(word_address(e, this, slot)) == EMPTY {
            return;
        }
    }
}

// Translated from 00d87760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::clear` (engine map name), `__fastcall`: empties every slot's hash word
/// (the loop runs while the index is below `hashMod + 1`, signed) and sets the element count
/// to 0.
pub fn hk_cached_hash_map_clear(e: &mut Engine, this: Ptr<HkCachedHashMap>) {
    let mut index = 0i32;
    let mut limit = e.get(this, HkCachedHashMap::m_hashMod).wrapping_add(1);
    if limit > 0 {
        loop {
            let address = word_address(e, this, index as u32);
            e.mem.set_u32(address, EMPTY);
            limit = e.get(this, HkCachedHashMap::m_hashMod).wrapping_add(1);
            index = index.wrapping_add(1);
            if index >= limit {
                break;
            }
        }
    }
    e.set(this, HkCachedHashMap::m_numElems, 0);
}

// Translated from 00d87790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::hkCachedHashMap` (engine map name), `thiscall` with one stack word it
/// never reads (`RET 4` at `00d877ca`; every caller pushes one): allocates `0xc0` bytes
/// from the calling thread's allocator (slot `+4`), fills the first `0x40` bytes with `0xff`
/// (`memSet`, the 16 hash words), no elements, `hashMod` 15. Returns `this`.
pub fn hk_cached_hash_map_hk_cached_hash_map(
    e: &mut Engine,
    this: Ptr<HkCachedHashMap>,
    _unused: u32,
) -> Ptr<HkCachedHashMap> {
    let allocator = allocator(e);
    let block = e.vcall(allocator, 4, &args![0xc0u32]).u32();
    e.set(this, HkCachedHashMap::m_elem, block);
    e.call(MEM_SET, &args![block, 0xffu32, 0x40u32]);
    e.set(this, HkCachedHashMap::m_numElems, 0);
    e.set(this, HkCachedHashMap::m_hashMod, 0xf);
    this
}

// Translated from 00d877d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::~hkCachedHashMap` (engine map name), `__fastcall`: frees the table
/// through the calling thread's allocator (slot `+8`) with its size, `(hashMod + 1) * 12`.
pub fn hk_cached_hash_map_dtor(e: &mut Engine, this: Ptr<HkCachedHashMap>) {
    let allocator = allocator(e);
    let slots = e.get(this, HkCachedHashMap::m_hashMod).wrapping_add(1);
    let elem = e.get(this, HkCachedHashMap::m_elem);
    e.vcall(
        allocator,
        8,
        &args![elem, slots.wrapping_mul(SLOT_BYTES as i32)],
    );
}

// Translated from 00d87800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::getWithDefault` (engine map name), thiscall, two stack words `(key,
/// default)`: the lookup `00d87650` into a local that starts as the default; returns that
/// local (the value when found, the default otherwise).
pub fn hk_cached_hash_map_get_with_default(
    e: &mut Engine,
    this: Ptr<HkCachedHashMap>,
    key: Ptr,
    default: u32,
) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), default);
        fn_00d87650(e, this, key, slot);
        e.mem.u32(slot.addr())
    })
}

// Translated from 00d87820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::remove` (engine map name), thiscall, one stack word `key`: when `key` is
/// present removes its slot (`00d876a0`) and returns 0, otherwise returns 1.
pub fn hk_cached_hash_map_remove(e: &mut Engine, this: Ptr<HkCachedHashMap>, key: Ptr) -> u32 {
    let index = fn_00d87580(e, this, key);
    let valid = e.with_stack(4, |e, flag| {
        let flag = hk_cached_hash_map_is_valid(e, this, flag, index);
        e.mem.u8(flag.addr()) != 0
    });
    if valid {
        fn_00d876a0(e, this, index as u32);
        return 0;
    }
    1
}

// Translated from 00d87860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCachedHashMap::resizeTable` (engine map name), thiscall, one stack word `slots`: takes
/// the old table and its slot count, allocates `slots * 12` bytes from the calling thread's
/// allocator (slot `+4`), fills the first `slots * 4` bytes with `0xff` (`memSet`), sets
/// `hashMod = slots - 1` and no elements, then inserts every occupied old slot's key and value
/// (`insert`) and frees the old table (slot `+8`) with its size, `oldSlots * 12`.
pub fn fn_00d87860(e: &mut Engine, this: Ptr<HkCachedHashMap>, slots: i32) {
    let old_slots = e.get(this, HkCachedHashMap::m_hashMod).wrapping_add(1);
    let old = e.get(this, HkCachedHashMap::m_elem);
    let allocator = allocator(e);
    let block = e
        .vcall(allocator, 4, &args![slots.wrapping_mul(SLOT_BYTES as i32)])
        .u32();
    e.set(this, HkCachedHashMap::m_elem, block);
    e.call(MEM_SET, &args![block, 0xffu32, slots.wrapping_mul(4)]);
    e.set(this, HkCachedHashMap::m_hashMod, slots.wrapping_sub(1));
    e.set(this, HkCachedHashMap::m_numElems, 0);
    if old_slots > 0 {
        for index in 0..old_slots as u32 {
            let hash_address = old.wrapping_add(index.wrapping_mul(4));
            if e.mem.u32(hash_address) != EMPTY {
                let key = e.mem.u32(
                    old.wrapping_add(old_slots as u32 * 4)
                        .wrapping_add(index.wrapping_mul(4)),
                );
                let value = e.mem.u32(
                    old.wrapping_add(
                        index
                            .wrapping_add((old_slots as u32).wrapping_mul(2))
                            .wrapping_mul(4),
                    ),
                );
                hk_cached_hash_map_insert(e, this, Ptr::new(key), value);
            }
        }
    }
    e.vcall(
        allocator,
        8,
        &args![old, old_slots.wrapping_mul(SLOT_BYTES as i32)],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d8_7410,
            hk_cached_hash_map_get_iterator(Ptr<HkCachedHashMap>) -> i32
        ),
        entry!(
            0x00d8_7430,
            hk_cached_hash_map_get_key(Ptr<HkCachedHashMap>, i32) -> u32
        ),
        entry!(
            0x00d8_7440,
            hk_cached_hash_map_get_value(Ptr<HkCachedHashMap>, i32) -> u32
        ),
        entry!(
            0x00d8_7480,
            hk_cached_hash_map_get_next(Ptr<HkCachedHashMap>, i32) -> i32
        ),
        entry!(
            0x00d8_74b0,
            hk_cached_hash_map_is_valid(Ptr<HkCachedHashMap>, Ptr, i32) -> Ptr
        ),
        entry!(
            0x00d8_74d0,
            hk_cached_hash_map_insert(Ptr<HkCachedHashMap>, Ptr, u32)
        ),
        entry!(0x00d8_7580, fn_00d87580(Ptr<HkCachedHashMap>, Ptr) -> i32),
        entry!(
            0x00d8_7600,
            fn_00d87600(Ptr<HkCachedHashMap>, Ptr, u32) -> u32
        ),
        entry!(
            0x00d8_7650,
            fn_00d87650(Ptr<HkCachedHashMap>, Ptr, Ptr) -> u32
        ),
        entry!(0x00d8_76a0, fn_00d876a0(Ptr<HkCachedHashMap>, u32)),
        entry!(0x00d8_7760, hk_cached_hash_map_clear(Ptr<HkCachedHashMap>)),
        entry!(
            0x00d8_7790,
            hk_cached_hash_map_hk_cached_hash_map(
                Ptr<HkCachedHashMap>,
                u32,
            ) -> Ptr<HkCachedHashMap>
        ),
        entry!(0x00d8_77d0, hk_cached_hash_map_dtor(Ptr<HkCachedHashMap>)),
        entry!(
            0x00d8_7800,
            hk_cached_hash_map_get_with_default(Ptr<HkCachedHashMap>, Ptr, u32) -> u32
        ),
        entry!(
            0x00d8_7820,
            hk_cached_hash_map_remove(Ptr<HkCachedHashMap>, Ptr) -> u32
        ),
        entry!(0x00d8_7860, fn_00d87860(Ptr<HkCachedHashMap>, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Blocks = Rc<RefCell<Vec<(u32, i32)>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0050;
    const FREE_SLOT: u32 = 0x0900_0051;

    /// Doubles for the callees: the router (its allocator allocates from the game heap and
    /// logs frees), `strCmp` (a real byte comparison) and `memSet` (real). Returns the free
    /// log.
    fn install(e: &mut Engine) -> Blocks {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE_SLOT, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let freed: Blocks = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&freed);
        e.register_double(FREE_SLOT, move |_, a| {
            seen.borrow_mut().push((a[1], a[2] as i32));
            Ret::default()
        });
        e.register(STR_CMP, |e, a| {
            let left = e.mem.cstr(a[0]);
            let right = e.mem.cstr(a[1]);
            Ret {
                eax: match left.cmp(&right) {
                    std::cmp::Ordering::Less => -1i32 as u32,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                },
                ..Ret::default()
            }
        });
        e.register(MEM_SET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        freed
    }

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 4);
        e.mem.write(address, text);
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    fn new_map(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d8_7790, &args![this, 0u32]).u32(), this);
        this
    }

    fn insert(e: &mut Engine, map: u32, key: u32, value: u32) {
        e.call(0x00d8_74d0, &args![map, key, value]);
    }

    fn find(e: &mut Engine, map: u32, key: u32) -> i32 {
        e.call(0x00d8_7580, &args![map, key]).i32()
    }

    #[test]
    fn constructor_makes_sixteen_empty_slots() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        assert_eq!(e.mem.i32(map + 4), 0);
        assert_eq!(e.mem.i32(map + 8), 15);
        let elem = e.mem.u32(map);
        for slot in 0..16 {
            assert_eq!(e.mem.u32(elem + 4 * slot), 0xffff_ffff);
        }
        // The key and value tables are untouched.
        assert_eq!(e.mem.u32(elem + 0x40), 0);
    }

    #[test]
    fn insert_then_find_get_value_and_overwrite() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        let one = put_string(&mut e, b"alpha");
        let two = put_string(&mut e, b"beta");
        insert(&mut e, map, one, 111);
        insert(&mut e, map, two, 222);
        assert_eq!(e.mem.i32(map + 4), 2);
        let slot = find(&mut e, map, one);
        assert!((0..=15).contains(&slot));
        assert_eq!(e.call(0x00d8_7440, &args![map, slot]).u32(), 111);
        assert_eq!(e.call(0x00d8_7430, &args![map, slot]).u32(), one);
        // The same key by another pointer: the value is replaced, the count unchanged, and
        // the key pointer is the new one.
        let again = put_string(&mut e, b"alpha");
        insert(&mut e, map, again, 333);
        assert_eq!(e.mem.i32(map + 4), 2);
        assert_eq!(find(&mut e, map, one), slot);
        assert_eq!(e.call(0x00d8_7440, &args![map, slot]).u32(), 333);
        assert_eq!(e.call(0x00d8_7430, &args![map, slot]).u32(), again);
        // A missing key answers hashMod + 1.
        let missing = put_string(&mut e, b"gamma");
        assert_eq!(find(&mut e, map, missing), 16);
    }

    #[test]
    fn hash_is_thirty_one_times_plus_signed_bytes() {
        let mut e = Engine::new();
        install(&mut e);
        let key = put_string(&mut e, b"ab");
        assert_eq!(string_hash(&e, key), 97 * 31 + 98);
        // A byte above 0x7f counts as negative.
        let high = put_string(&mut e, &[0x80]);
        assert_eq!(string_hash(&e, high), 0xffff_ff80);
        let map = new_map(&mut e);
        insert(&mut e, map, high, 5);
        let slot = find(&mut e, map, high) as u32;
        // The stored hash word is masked with 0x7fffffff and the home slot is hash & hashMod.
        assert_eq!(e.mem.u32(e.mem.u32(map) + 4 * slot), 0x7fff_ff80);
        assert_eq!(slot, 0x7fff_ff80 & 15);
    }

    #[test]
    fn iteration_visits_every_occupied_slot_once() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        assert_eq!(e.call(0x00d8_7410, &args![map]).i32(), 16);
        let names = ["a", "bb", "ccc", "dddd", "eeeee"];
        for (i, name) in names.iter().enumerate() {
            let key = put_string(&mut e, name.as_bytes());
            insert(&mut e, map, key, 10 + i as u32);
        }
        let mut seen = Vec::new();
        let mut index = e.call(0x00d8_7410, &args![map]).i32();
        let valid = e.mem.alloc(4);
        loop {
            e.call(0x00d8_74b0, &args![map, valid, index]);
            if e.mem.u8(valid) == 0 {
                break;
            }
            let key = e.call(0x00d8_7430, &args![map, index]).u32();
            let value = e.call(0x00d8_7440, &args![map, index]).u32();
            seen.push((String::from_utf8(e.mem.cstr(key)).unwrap(), value));
            index = e.call(0x00d8_7480, &args![map, index]).i32();
        }
        seen.sort();
        let expected: Vec<(String, u32)> = names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.to_string(), 10 + i as u32))
            .collect();
        assert_eq!(seen, expected);
    }

    #[test]
    fn get_iterator_and_next_with_a_negative_hash_mod() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x10);
        e.mem.set_i32(map + 8, -1);
        assert_eq!(e.call(0x00d8_7410, &args![map]).i32(), 0);
        assert_eq!(e.call(0x00d8_7480, &args![map, 5i32]).i32(), 6);
    }

    #[test]
    fn is_valid_compares_the_index_with_hash_mod_signed() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x10);
        e.mem.set_i32(map + 8, 15);
        let out = e.mem.alloc(4);
        for (index, expected) in [(0i32, 1u8), (15, 1), (16, 0), (-3, 1)] {
            assert_eq!(e.call(0x00d8_74b0, &args![map, out, index]).u32(), out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn inserting_many_keys_grows_the_table_and_keeps_every_entry() {
        let mut e = Engine::new();
        let freed = install(&mut e);
        let map = new_map(&mut e);
        let first_table = e.mem.u32(map);
        let mut keys = Vec::new();
        for i in 0..24u32 {
            let key = put_string(&mut e, format!("key-{i}").as_bytes());
            keys.push(key);
            insert(&mut e, map, key, 1000 + i);
        }
        // 16 slots hold until twice the count passes 15 (the eighth insert resizes to 32).
        assert!(e.mem.i32(map + 8) >= 31);
        assert_ne!(e.mem.u32(map), first_table);
        assert_eq!(e.mem.i32(map + 4), 24);
        // The first table went back to the allocator with its size, 16 * 12.
        assert_eq!(freed.borrow()[0], (first_table, 0xc0));
        let valid = e.mem.alloc(4);
        for (i, key) in keys.iter().enumerate() {
            let slot = find(&mut e, map, *key);
            e.call(0x00d8_74b0, &args![map, valid, slot]);
            assert_eq!(e.mem.u8(valid), 1, "key {i}");
            assert_eq!(
                e.call(0x00d8_7440, &args![map, slot]).u32(),
                1000 + i as u32
            );
        }
    }

    #[test]
    fn get_with_default_and_the_pointer_lookup() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        let key = put_string(&mut e, b"present");
        let other = put_string(&mut e, b"absent");
        insert(&mut e, map, key, 77);
        assert_eq!(e.call(0x00d8_7800, &args![map, key, 5u32]).u32(), 77);
        assert_eq!(e.call(0x00d8_7800, &args![map, other, 5u32]).u32(), 5);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        assert_eq!(e.call(0x00d8_7650, &args![map, key, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 77);
        e.mem.set_u32(out, 0xdead);
        assert_eq!(e.call(0x00d8_7650, &args![map, other, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0xdead);
    }

    #[test]
    fn find_or_insert_returns_the_stored_value_or_inserts_the_given_one() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        let key = put_string(&mut e, b"k");
        assert_eq!(e.call(0x00d8_7600, &args![map, key, 9u32]).u32(), 9);
        assert_eq!(e.mem.i32(map + 4), 1);
        // Present now: the stored value wins over the new one.
        assert_eq!(e.call(0x00d8_7600, &args![map, key, 10u32]).u32(), 9);
        assert_eq!(e.mem.i32(map + 4), 1);
    }

    #[test]
    fn remove_deletes_the_entry_and_keeps_colliding_ones_findable() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        // Keys 16 apart in hash collide in a 16-slot table: single characters whose codes
        // differ by 16 ('A' = 0x41 and 'Q' = 0x51) share the home slot, and so do 'a'/'q'.
        let mut keys = Vec::new();
        for (i, name) in [b"A", b"Q", b"a", b"q", b"B", b"R"].iter().enumerate() {
            let key = put_string(&mut e, *name);
            insert(&mut e, map, key, 100 + i as u32);
            keys.push(key);
        }
        let valid = e.mem.alloc(4);
        // Remove each in turn; the rest stay findable with their values.
        for removed in 0..keys.len() {
            assert_eq!(e.call(0x00d8_7820, &args![map, keys[removed]]).u32(), 0);
            assert_eq!(e.mem.i32(map + 4), (keys.len() - removed - 1) as i32);
            // Removing it again fails.
            assert_eq!(e.call(0x00d8_7820, &args![map, keys[removed]]).u32(), 1);
            for (i, key) in keys.iter().enumerate() {
                let slot = find(&mut e, map, *key);
                e.call(0x00d8_74b0, &args![map, valid, slot]);
                if i > removed {
                    assert_eq!(e.mem.u8(valid), 1, "key {i} after removing {removed}");
                    assert_eq!(e.call(0x00d8_7440, &args![map, slot]).u32(), 100 + i as u32);
                } else {
                    assert_eq!(e.mem.u8(valid), 0, "key {i} still present");
                }
            }
        }
    }

    #[test]
    fn clear_empties_every_slot_and_the_count() {
        let mut e = Engine::new();
        install(&mut e);
        let map = new_map(&mut e);
        for name in [b"x", b"y", b"z"] {
            let key = put_string(&mut e, name);
            insert(&mut e, map, key, 1);
        }
        e.call(0x00d8_7760, &args![map]);
        assert_eq!(e.mem.i32(map + 4), 0);
        assert_eq!(e.call(0x00d8_7410, &args![map]).i32(), 16);
        // A map with a negative hashMod: nothing to clear, the count still resets.
        let odd = e.mem.alloc(0x10);
        e.mem.set_i32(odd + 4, 3);
        e.mem.set_i32(odd + 8, -1);
        e.call(0x00d8_7760, &args![odd]);
        assert_eq!(e.mem.i32(odd + 4), 0);
    }

    #[test]
    fn destructor_frees_the_table_with_its_size() {
        let mut e = Engine::new();
        let freed = install(&mut e);
        let map = new_map(&mut e);
        let table = e.mem.u32(map);
        e.call(0x00d8_77d0, &args![map]);
        assert_eq!(*freed.borrow(), vec![(table, 16 * 12)]);
    }
}
