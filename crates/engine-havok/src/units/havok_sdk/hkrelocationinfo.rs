//! `Havok SDK/hkrelocationinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkRelocationInfo` (not in the Xbox PDB; layout from the PC code) holds four `hkArray`
//! records (data at `+0`, size at `+4`, capacity at `+8` with the top bit set for storage the
//! array does not own) at `+0x00` (8-byte elements), `+0x0C` (16 bytes), `+0x18` (8 bytes) and
//! `+0x24` (8 bytes), and a pointer at `+0x30` to a string map.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator object (virtual slot 8
/// gives a block back with its size, slot `0x10` gives back an array's storage).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkCachedHashMap<hkStringMapOperations>` pieces, `this` = the map in `ECX`: `getIterator`
/// (`00d87410`), `isValid` (`00d874b0`, `(out byte, iterator)`), `getKey` (`00d87430`,
/// `(iterator)`), `getNext` (`00d87480`, `(iterator)`), `clear` (`00d87760`) and the destructor
/// (`00d877d0`).
const MAP_ITERATOR: u32 = 0x00d8_7410;
const MAP_IS_VALID: u32 = 0x00d8_74b0;
const MAP_KEY: u32 = 0x00d8_7430;
const MAP_NEXT: u32 = 0x00d8_7480;
const MAP_CLEAR: u32 = 0x00d8_7760;
const MAP_DESTRUCTOR: u32 = 0x00d8_77d0;
/// `00cb1190` (cdecl `(string, allocator)`): gives a stored string back.
const STRING_FREE: u32 = 0x00cb_1190;

fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00e029d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStorageStringMap<int,hkContainerHeapAllocator>::clear` (Xbox PDB), `this` = the map:
/// gives back the stored key string of every entry (`00cb1190` with the allocator) walking
/// the map's iterator (`getIterator`, `isValid`, `getKey`, `getNext`), then `clear` on the map
/// (`00d87760`, a tail call).
pub fn hk_storage_string_map_clear(e: &mut Engine, this: Ptr) {
    let mut iterator = e.call(MAP_ITERATOR, &args![this]).u32();
    e.with_stack(4, |e, valid| {
        e.call(MAP_IS_VALID, &args![this, valid, iterator]);
        while e.mem.u8(valid.addr()) != 0 {
            let key = e.call(MAP_KEY, &args![this, iterator]).u32();
            let heap = allocator(e);
            e.call(STRING_FREE, &args![key, heap]);
            iterator = e.call(MAP_NEXT, &args![this, iterator]).u32();
            e.call(MAP_IS_VALID, &args![this, valid, iterator]);
        }
    });
    e.call(MAP_CLEAR, &args![this]);
}

/// Empties the `hkArray` record at `this + base` (data at `+base`, size `+base+4`, capacity
/// `+base+8`) of `element_size`-byte elements: size 0, the storage given back through the
/// allocator's slot `0x10` unless the top capacity bit is set, data 0, capacity `0x80000000`.
fn release_array(e: &mut Engine, this: Ptr, base: u32, element_size: u32) {
    let heap = allocator(e);
    let capacity = e.mem.u32(this.addr() + base + 8);
    e.mem.set_u32(this.addr() + base + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(this.addr() + base);
        e.vcall(
            heap,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(this.addr() + base, 0);
    e.mem.set_u32(this.addr() + base + 8, 0x8000_0000);
}

// Translated from 00e028e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRelocationInfo::~hkRelocationInfo` (Xbox PDB): when the string map at `+0x30` exists,
/// clears it ([`hk_storage_string_map_clear`]), runs its destructor (`00d877d0`) and gives its
/// 0x10-byte block back to the allocator (slot 8); then empties the four arrays in the order
/// `+0x24`, `+0x18`, `+0x0C`, `+0` (see `release_array`).
pub fn hk_relocation_info_destructor(e: &mut Engine, this: Ptr) {
    let map = e.mem.u32(this.addr() + 0x30);
    if map != 0 {
        hk_storage_string_map_clear(e, Ptr::new(map));
        e.call(MAP_DESTRUCTOR, &args![map]);
        let heap = allocator(e);
        e.vcall(heap, 8, &args![map, 0x10u32]);
    }
    release_array(e, this, 0x24, 8);
    release_array(e, this, 0x18, 8);
    release_array(e, this, 0x0c, 16);
    release_array(e, this, 0, 8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_28e0, hk_relocation_info_destructor(Ptr)),
        entry!(0x00e0_29d0, hk_storage_string_map_clear(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const FREE_BLOCK: u32 = 0x0900_0010;
    const FREE_ARRAY: u32 = 0x0900_0020;

    /// A router whose allocator logs both give-back calls.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_BLOCK, move |_, a| {
            seen.borrow_mut()
                .push(format!("block:{:x}:{:x}", a[1], a[2]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_ARRAY, move |_, a| {
            seen.borrow_mut()
                .push(format!("array:{:x}:{:x}", a[1], a[2]));
            Ret::default()
        });
        (e, log)
    }

    /// Doubles for the map: two entries with the keys 0x100 and 0x200, iterated by index.
    fn install_map(e: &mut Engine, log: &Log) {
        let seen = Rc::clone(log);
        e.register_double(MAP_ITERATOR, move |_, a| {
            seen.borrow_mut().push(format!("iterator:{:x}", a[0]));
            Ret::default()
        });
        e.register(MAP_IS_VALID, |e, a| {
            e.mem.set_u8(a[1], u8::from(a[2] < 2));
            Ret::default()
        });
        e.register(MAP_KEY, |_, a| Ret {
            eax: 0x100 * (a[1] + 1),
            ..Ret::default()
        });
        e.register(MAP_NEXT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let seen = Rc::clone(log);
        e.register_double(STRING_FREE, move |_, a| {
            seen.borrow_mut().push(format!("string:{:x}", a[0]));
            Ret::default()
        });
        let seen = Rc::clone(log);
        e.register_double(MAP_CLEAR, move |_, a| {
            seen.borrow_mut().push(format!("clear:{:x}", a[0]));
            Ret::default()
        });
    }

    #[test]
    fn clear_frees_every_key_and_clears_the_map() {
        let (mut e, log) = engine();
        install_map(&mut e, &log);
        let map = e.mem.alloc(0x20);
        e.call(0x00e0_29d0, &args![map]);
        assert_eq!(
            *log.borrow(),
            vec![
                format!("iterator:{map:x}"),
                "string:100".to_string(),
                "string:200".to_string(),
                format!("clear:{map:x}"),
            ]
        );
    }

    #[test]
    fn destructor_without_a_map_empties_the_four_arrays() {
        let (mut e, log) = engine();
        let this = e.mem.alloc(0x40);
        // Owned storage in the arrays at +0x24 (3 elements), +0x0C (2 of 16 bytes); the
        // others hold in-place storage.
        e.mem.set_u32(this + 0x24, 0xa000);
        e.mem.set_u32(this + 0x28, 3);
        e.mem.set_u32(this + 0x2c, 3);
        e.mem.set_u32(this + 0x18, 0xb000);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 0x8000_0004);
        e.mem.set_u32(this + 0x0c, 0xc000);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this, 0xd000);
        e.mem.set_u32(this + 4, 4);
        e.mem.set_u32(this + 8, 5);
        e.call(0x00e0_28e0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                "array:a000:18".to_string(),
                "array:c000:20".to_string(),
                "array:d000:28".to_string(),
            ]
        );
        for base in [0x24, 0x18, 0x0c, 0] {
            assert_eq!(e.mem.u32(this + base), 0);
            assert_eq!(e.mem.u32(this + base + 4), 0);
            assert_eq!(e.mem.u32(this + base + 8), 0x8000_0000);
        }
    }

    #[test]
    fn destructor_with_a_map_clears_destroys_and_frees_it_first() {
        let (mut e, log) = engine();
        install_map(&mut e, &log);
        let seen = Rc::clone(&log);
        e.register_double(MAP_DESTRUCTOR, move |_, a| {
            seen.borrow_mut().push(format!("destroy:{:x}", a[0]));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let map = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x30, map);
        for base in [0x24, 0x18, 0x0c, 0] {
            e.mem.set_u32(this + base + 8, 0x8000_0000);
        }
        e.call(0x00e0_28e0, &args![this]);
        let log = log.borrow();
        assert_eq!(log[0], format!("iterator:{map:x}"));
        assert_eq!(log[3], format!("clear:{map:x}"));
        assert_eq!(log[4], format!("destroy:{map:x}"));
        assert_eq!(log[5], format!("block:{map:x}:10"));
        assert_eq!(log.len(), 6);
    }
}
