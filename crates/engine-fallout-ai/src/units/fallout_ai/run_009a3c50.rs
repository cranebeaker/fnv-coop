//! `fallout/ai/run_009a3c50` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two `BSSimpleArray` template members (their vtables' slots, Xbox PDB:
//! `+0x4` `_Allocate`, `+0xc` `_Reallocate`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `BSSimpleArray::_Allocate`'s vtable slot (one word: the capacity).
const SLOT_ALLOCATE: u32 = 0x4;
/// `BSSimpleArray::_Reallocate`'s vtable slot (two words: buffer, capacity).
const SLOT_REALLOCATE: u32 = 0xc;
/// `006a8a30`: constructs items in place (`thiscall`: buffer, count).
const CONSTRUCT_ITEMS: u32 = 0x006a_8a30;
/// `009a3b20`: moves items to a new buffer (`thiscall`: new buffer, old
/// buffer, count).
const MOVE_ITEMS: u32 = 0x009a_3b20;
/// `006a8500`: frees the old buffer (`thiscall`).
const FREE_BUFFER: u32 = 0x006a_8500;

// Translated from 009a3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: gives the array a buffer of `capacity` items. Without a
/// buffer it allocates one (and stores the capacity); with one whose
/// reserved size equals `count` it reallocates it; otherwise it allocates
/// a new one, moves `count` items into it (`009a3b20`) and frees the old
/// (`006a8500`). The reserved size is only stored in the first case.
pub fn fn_009a3c50(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    let this_word = this.addr();
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        let buffer = e.vcall(this_word, SLOT_ALLOCATE, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let old = e.get(this, BSSimpleArray::pBuffer);
        let buffer = e
            .vcall(this_word, SLOT_REALLOCATE, &args![old, capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
    } else {
        let buffer = e.vcall(this_word, SLOT_ALLOCATE, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(MOVE_ITEMS, &args![this, buffer, old, count]);
        e.call(FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
    }
}

// Translated from 009a3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: sets the array up for `size` items with room for the
/// larger of `reserve` and `size`: all three fields cleared, a buffer
/// allocated (slot `_Allocate`) when that room is not 0, and `size` items
/// constructed (`006a8a30`) when `size` is not 0.
pub fn fn_009a3d00(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let room = if size > reserve { size } else { reserve };
    if room != 0 {
        let buffer = e.vcall(this.addr(), SLOT_ALLOCATE, &args![room]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, room);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(CONSTRUCT_ITEMS, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

/// `004300a0` (unnamed): stores 0.0 in the float at `this` (`ECX` only) and returns
/// `this`.
const CLEAR_FLOAT: u32 = 0x0043_00a0;

// Translated from 009a3d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): constructs an 8-byte item by clearing its
/// float at `+4` (`004300a0`); returns `this`. `+0` is left as it is.
pub fn fn_009a3d90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CLEAR_FLOAT, &args![this.addr() + 0x4]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_3c50, fn_009a3c50(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x009a_3d00, fn_009a3d00(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x009a_3d90, fn_009a3d90(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn array(e: &mut Engine) -> Ptr<BSSimpleArray> {
        let this = e.new_object::<BSSimpleArray>();
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr(), table);
        e.mem.set_u32(table + SLOT_ALLOCATE, 0x7a00_0004);
        e.mem.set_u32(table + SLOT_REALLOCATE, 0x7a00_000c);
        e.register(0x7a00_0004, |_, a| (0x9000 + a[1]).into_ret());
        e.register(0x7a00_000c, |_, a| (a[1] + a[2]).into_ret());
        for addr in [CONSTRUCT_ITEMS, MOVE_ITEMS, FREE_BUFFER] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    #[test]
    fn setup_reserves_the_larger_and_constructs_items() {
        let mut e = Engine::new();
        let this = array(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009a_3d00, &args![this, 4u32, 6u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9006);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 6);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 6);
        assert_eq!(
            calls_to(&e, CONSTRUCT_ITEMS),
            vec![vec![this.addr(), 0x9006, 6]]
        );
        e.call_log = Some(vec![]);
        e.call(0x009a_3d00, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert!(calls_to(&e, 0x7a00_0004).is_empty());
        assert!(calls_to(&e, CONSTRUCT_ITEMS).is_empty());
    }

    #[test]
    fn grow_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        let this = array(&mut e);
        e.call(0x009a_3c50, &args![this, 8u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9008);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        // Count equal to the reserved size: reallocated in place.
        e.call(0x009a_3c50, &args![this, 0x10u32, 8u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9018);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        // Otherwise: a new buffer, the items moved, the old freed.
        e.call_log = Some(vec![]);
        e.call(0x009a_3c50, &args![this, 0x20u32, 3u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9020);
        assert_eq!(
            calls_to(&e, MOVE_ITEMS),
            vec![vec![this.addr(), 0x9020, 0x9018, 3]]
        );
        assert_eq!(calls_to(&e, FREE_BUFFER), vec![vec![this.addr()]]);
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn fn_009a3d90_clears_the_float_after_the_first_word() {
        let mut e = Engine::new();
        e.register(CLEAR_FLOAT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let item = e.mem.alloc(8);
        e.mem.write(item, &[0xcd; 8]);
        assert_eq!(e.call(0x009a_3d90, &args![item]).u32(), item);
        assert_eq!(e.mem.u32(item), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(item + 4), 0);
    }
}
