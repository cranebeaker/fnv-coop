//! `Havok SDK/hkppaircollisionfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The filter keeps a map from 64-bit keys to 64-bit counters at `+0x30` (data `+0x30`, size
//! `+0x34`, `+0x38` the highest index; `hkMapBase<unsigned __int64, unsigned __int64>`, whose
//! functions `00cb1cb0`, `00cb1bc0`, `00cb2000`, `00cb1c60`, `00cb2030` and `00cb1e40` are
//! called here) of the pairs of entities it disables collisions between: the key of a pair is
//! `(smaller << 32) + larger` of the two words taken as signed numbers extended to 64 bits and
//! ordered as unsigned 64-bit numbers (the exe's `_allmul` `00ecc300` is a 64-bit multiply by
//! `1 << 32` there).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMapBase::getWithDefault` (`00cb1cb0`, `this` = the map, key low and high, default low and
/// high; returns the 64-bit value in `EDX:EAX`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1cb0;
/// `hkMapBase::insert` (`00cb1bc0`, `this` = the map, allocator, key low and high, value low and
/// high).
const MAP_INSERT: u32 = 0x00cb_1bc0;
/// `hkMapBase::remove` by key (`00cb2000`, `this` = the map, key low and high).
const MAP_ERASE: u32 = 0x00cb_2000;
/// `hkMapBase::findKey` (`00cb1c60`, `this` = the map, key low and high; returns the index).
const MAP_FIND_KEY: u32 = 0x00cb_1c60;
/// `hkMapBase::reserve` (`00cb2030`, `this` = the map, allocator, 0) and
/// `clearAndDeallocate` (`00cb1e40`, `this` = the map, allocator).
const MAP_RESERVE: u32 = 0x00cb_2030;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1e40;
/// The base class constructor (`00cbfba0`, `this`), the empty call the destructor makes on the
/// map (`00a29680`, `this` = the map: the PC build compiled it to a bare `RET`), and the
/// reference count functions (`00c90510` takes a reference, `00c905b0` releases one).
const BASE_CONSTRUCT: u32 = 0x00cb_fba0;
const EMPTY_CALL: u32 = 0x00a2_9680;
const ADD_REFERENCE: u32 = 0x00c9_0510;
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;

/// The filter: an object `+0x28` / `+0x34` of which hold the optional second filters, and
/// the pair map at `+0x30`.
const FILTER_OBJECT: u32 = 0x28;
const MAP: u32 = 0x30;
const REFERENCED: u32 = 0x3c;
/// The vtables the constructor and destructor set at `+0`, `+8`, `+0xc`, `+0x10` and `+0x14`.
const VTABLES: [(u32, u32); 5] = [
    (0x00, 0x010d_a0c8),
    (0x08, 0x010d_a0bc),
    (0x0c, 0x010d_a0a8),
    (0x10, 0x010d_a09c),
    (0x14, 0x010d_a090),
];
/// The vtables the destructor leaves.
const FINAL_VTABLES: [(u32, u32); 5] = [
    (0x14, 0x010c_3b8c),
    (0x10, 0x010c_3bac),
    (0x0c, 0x010c_3b98),
    (0x08, 0x010c_3b80),
    (0x00, 0x0102_e388),
];

/// The 64-bit key of a pair of signed words: the one compared as an unsigned 64-bit number
/// (after sign extension) as the smaller goes in the high half and the other is added to it.
fn pair_key(a: u32, b: u32) -> u64 {
    let first = i64::from(a as i32) as u64;
    let second = i64::from(b as i32) as u64;
    if first < second {
        (first << 32).wrapping_add(second)
    } else {
        (second << 32).wrapping_add(first)
    }
}

/// The two words of a 64-bit number, low first.
fn halves(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
}

/// `getWithDefault(key, 0)` on the map at `map`.
fn count_of(e: &mut Engine, map: u32, key: u64) -> u64 {
    let (low, high) = halves(key);
    let value = e.call(MAP_GET_WITH_DEFAULT, &args![map, low, high, 0u32, 0u32]);
    (u64::from(value.edx) << 32) | u64::from(value.eax)
}

/// `insert(allocator, key, value)` with the router's allocator.
fn store_count(e: &mut Engine, map: u32, key: u64, value: u64) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let (low, high) = halves(key);
    let (value_low, value_high) = halves(value);
    e.call(
        MAP_INSERT,
        &args![map, allocator, low, high, value_low, value_high],
    );
}

// Translated from 00def120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPairCollisionFilter::isCollisionEnabled` (engine map): `ECX` = filter and three stack
/// words (`out`, `a`, `b`). If the object at `+0x28` is not null, asks its interface at `+0x14`
/// (vtable slot 4: a scratch word for the result, `a`, `b`) for a pointer to a result byte, and
/// when that byte is 0 stores 0 at `out` and stops; otherwise stores 1 at `out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn hkp_pair_collision_filter_is_collision_enabled(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    a: u32,
    b: u32,
) -> u32 {
    let object = e.mem.u32(this.addr() + FILTER_OBJECT);
    if object != 0 {
        // The scratch hkBool is the frame's argument slot (`ESP+0x14` at the call): the slot
        // writes its answer there and answers its address, and the byte is tested through that
        // address, so the test is made while it is there. (Scoped to the slot call alone, the
        // byte was read from a freed block.)
        let enabled = e.with_stack(4, |e, scratch| {
            let result = e.vcall(object + 0x14, 4, &args![scratch, a, b]).u32();
            e.mem.u8(result) != 0
        });
        if !enabled {
            e.mem.set_u8(out, 0);
            return out;
        }
    }
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00def160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor (no map name; `ECX` = filter, no stack words). Sets the five vtables
/// (`010da0c8`, `010da0bc`, `010da0a8`, `010da09c`, `010da090` at `+0`, `+8`, `+0xc`, `+0x10`,
/// `+0x14`), releases the object at `+0x3c` if there is one (`00c905b0`), clears and
/// deallocates the pair map (`00cb1e40` with the router's allocator), calls `00a29680` on the
/// map, and leaves the vtables `010c3b8c` (`+0x14`), `010c3bac` (`+0x10`), `010c3b98` (`+0xc`),
/// `010c3b80` (`+8`) and `0102e388` (`+0`).
pub fn fn_00def160(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let referenced = e.mem.u32(me + REFERENCED);
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(me + offset, vtable);
    }
    if referenced != 0 {
        e.call(RELEASE_REFERENCE, &args![referenced]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![me + MAP, allocator]);
    e.call(EMPTY_CALL, &args![me + MAP]);
    for (offset, vtable) in FINAL_VTABLES {
        e.mem.set_u32(me + offset, vtable);
    }
}

// Translated from 00def1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ECX` = filter and two stack words, `a` and `b` (signed words).
/// Takes one off the counter of the pair (see `pair_key`) in the map at `+0x30`, which is 0 for
/// a pair that is not in it: a counter that is 0 stays so and 0 is returned; a counter of 1 is
/// removed from the map (`00cb2000`) and 0 returned; any other counter is stored again
/// decreased by one (`00cb1bc0`) and its low half returned.
pub fn fn_00def1e0(e: &mut Engine, this: Ptr, a: u32, b: u32) -> u32 {
    let map = this.addr() + MAP;
    let key = pair_key(a, b);
    let count = count_of(e, map, key);
    if count == 0 {
        return 0;
    }
    let remaining = count.wrapping_sub(1);
    if remaining != 0 {
        store_count(e, map, key, remaining);
        return remaining as u32;
    }
    let (low, high) = halves(key);
    e.call(MAP_ERASE, &args![map, low, high]);
    0
}

// Translated from 00def2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ECX` = filter and two stack words, `a` and `b`. Adds one to the
/// counter of the pair (`pair_key`) in the map at `+0x30` (0 if it is not there) and stores it
/// again (`00cb1bc0`); returns the low half of the new counter.
pub fn fn_00def2b0(e: &mut Engine, this: Ptr, a: u32, b: u32) -> u32 {
    let map = this.addr() + MAP;
    let key = pair_key(a, b);
    let count = count_of(e, map, key).wrapping_add(1);
    store_count(e, map, key, count);
    count as u32
}

// Translated from 00def350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no map name): `ECX` = filter and one stack word, `object`. Runs the base
/// constructor (`00cbfba0`), sets the five vtables as the destructor does, empties the pair
/// map (data 0, size 0, `+0x38` = `0xffffffff`) and reserves it with 0 elements
/// (`00cb2030`, the router's allocator), stores `object` at `+0x3c`, 5 at `+0x20`, and takes a
/// reference on `object` (`00c90510`) if it is not null. Returns `this`.
pub fn fn_00def350(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let me = this.addr();
    e.call(BASE_CONSTRUCT, &args![me]);
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(me + offset, vtable);
    }
    let map = me + MAP;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.mem.set_u32(me + REFERENCED, object);
    e.mem.set_u32(me + 0x20, 5);
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    this
}

// Translated from 00def3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ECX` = the filter base (the object `isCollisionEnabled_ov2`
/// works on at `this - 8`) and three stack words (`out`, `a`, `b`, two entity pointers). If
/// either pointer is 0, stores 1 at `out`. Otherwise looks the pair (`pair_key`, the pointers
/// taken as signed words) up in the map at `+0x30` (`00cb1c60`) and stores 1 if the index found
/// is (signed) greater than the word at `+0x38`, else 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00def3d0(e: &mut Engine, this: Ptr, out: u32, a: u32, b: u32) -> u32 {
    if a != 0 && b != 0 {
        let map = this.addr() + MAP;
        let (low, high) = halves(pair_key(a, b));
        let index = e.call(MAP_FIND_KEY, &args![map, low, high]).u32();
        let highest = e.mem.u32(map + 8);
        e.mem.set_u8(out, u8::from(index as i32 > highest as i32));
        return out;
    }
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00def450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPairCollisionFilter::isCollisionEnabled_ov2` (engine map): `ECX` = the sub-object at
/// `+8` of the filter and three stack words (`out`, `a`, `b`). If the object at `+0x34` (of
/// the sub-object, so `+0x2c` of the filter) is not null, asks its interface at `+8` (vtable
/// slot 4: scratch word, `a`, `b`) for a result byte and when it is 0 stores 0 at `out`
/// and returns `out`. Otherwise each of `a` and `b` is turned into the object it stands for
/// (the pointer plus the signed byte at `+0x10` when the byte at `+0x18` is 1, else 0) and
/// [`fn_00def3d0`] runs on `this - 8` with `out` and both. Returns `out`.
pub fn hkp_pair_collision_filter_is_collision_enabled_ov2(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    a: u32,
    b: u32,
) -> u32 {
    let me = this.addr();
    let object = e.mem.u32(me + 0x34);
    if object != 0 {
        // The scratch hkBool is the frame's argument slot (`ESP+0x20` at the call), tested
        // through the address the slot answers while it is there, as in `00def120`.
        let enabled = e.with_stack(4, |e, scratch| {
            let result = e.vcall(object + 8, 4, &args![scratch, a, b]).u32();
            e.mem.u8(result) != 0
        });
        if !enabled {
            e.mem.set_u8(out, 0);
            return out;
        }
    }
    let entity_of = |e: &Engine, handle: u32| {
        if e.mem.u8(handle + 0x18) == 1 {
            handle.wrapping_add(i32::from(e.mem.i8(handle + 0x10)) as u32)
        } else {
            0
        }
    };
    let first = entity_of(e, a);
    let second = entity_of(e, b);
    fn_00def3d0(e, Ptr::new(me.wrapping_sub(8)), out, first, second);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00def120,
            hkp_pair_collision_filter_is_collision_enabled(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x00def160, fn_00def160(Ptr)),
        entry!(0x00def1e0, fn_00def1e0(Ptr, u32, u32) -> u32),
        entry!(0x00def2b0, fn_00def2b0(Ptr, u32, u32) -> u32),
        entry!(0x00def350, fn_00def350(Ptr, u32) -> Ptr),
        entry!(0x00def3d0, fn_00def3d0(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00def450,
            hkp_pair_collision_filter_is_collision_enabled_ov2(Ptr, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Counters = Rc<RefCell<HashMap<u64, u64>>>;

    const ROUTER: u32 = 0x0030_1000;
    const ALLOCATOR: u32 = 0x0030_2000;
    const FAKE_ENABLED: u32 = 0x7000_0010;
    const FAKE_WRITES_ITS_SCRATCH: u32 = 0x7000_0020;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ROUTER, 0x20);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        e
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// Doubles of the map functions on a map from keys to counters.
    fn counter_map(e: &mut Engine) -> Counters {
        let counters: Counters = Rc::default();
        let get = Rc::clone(&counters);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
            let key = (u64::from(a[2]) << 32) | u64::from(a[1]);
            let value = get.borrow().get(&key).copied().unwrap_or(0);
            Ret {
                eax: value as u32,
                edx: (value >> 32) as u32,
                ..Ret::default()
            }
        });
        let insert = Rc::clone(&counters);
        e.register_double(MAP_INSERT, move |_, a| {
            let key = (u64::from(a[3]) << 32) | u64::from(a[2]);
            let value = (u64::from(a[5]) << 32) | u64::from(a[4]);
            insert.borrow_mut().insert(key, value);
            Ret::default()
        });
        let erase = Rc::clone(&counters);
        e.register_double(MAP_ERASE, move |_, a| {
            let key = (u64::from(a[2]) << 32) | u64::from(a[1]);
            erase.borrow_mut().remove(&key);
            Ret::default()
        });
        counters
    }

    #[test]
    fn the_pair_key_puts_the_smaller_word_in_the_high_half() {
        assert_eq!(pair_key(3, 9), 0x0000_0003_0000_0009);
        assert_eq!(pair_key(9, 3), 0x0000_0003_0000_0009);
        assert_eq!(pair_key(5, 5), 0x0000_0005_0000_0005);
        // Negative words are extended to 64 bits and ordered as unsigned numbers: -1 is the
        // larger one and is added with its sign extension.
        assert_eq!(pair_key(-1i32 as u32, 2), 0x0000_0001_ffff_ffff);
        assert_eq!(pair_key(2, -1i32 as u32), 0x0000_0001_ffff_ffff);
    }

    #[test]
    fn counting_a_pair_up_and_down_uses_the_symmetric_key() {
        let mut e = engine();
        let counters = counter_map(&mut e);
        let filter = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00de_f2b0, &args![filter, 7u32, 3u32]).u32(), 1);
        assert_eq!(e.call(0x00de_f2b0, &args![filter, 3u32, 7u32]).u32(), 2);
        assert_eq!(counters.borrow().get(&0x0000_0003_0000_0007), Some(&2));
        assert_eq!(e.call(0x00de_f1e0, &args![filter, 7u32, 3u32]).u32(), 1);
        assert_eq!(counters.borrow().get(&0x0000_0003_0000_0007), Some(&1));
        // Down to zero removes the pair; a pair that is not there stays at 0.
        assert_eq!(e.call(0x00de_f1e0, &args![filter, 3u32, 7u32]).u32(), 0);
        assert!(counters.borrow().is_empty());
        assert_eq!(e.call(0x00de_f1e0, &args![filter, 3u32, 7u32]).u32(), 0);
        assert!(counters.borrow().is_empty());
    }

    #[test]
    fn counters_keep_their_high_half() {
        let mut e = engine();
        let counters = counter_map(&mut e);
        let filter = e.mem.alloc(0x60);
        counters
            .borrow_mut()
            .insert(0x0000_0004_0000_0005, 0x1_0000_0000);
        assert_eq!(
            e.call(0x00de_f1e0, &args![filter, 4u32, 5u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(
            counters.borrow().get(&0x0000_0004_0000_0005),
            Some(&0xffff_ffff)
        );
        counters
            .borrow_mut()
            .insert(0x0000_0004_0000_0005, 0xffff_ffff);
        assert_eq!(e.call(0x00de_f2b0, &args![filter, 4u32, 5u32]).u32(), 0);
        assert_eq!(
            counters.borrow().get(&0x0000_0004_0000_0005),
            Some(&0x1_0000_0000)
        );
    }

    #[test]
    fn the_insert_uses_the_router_s_allocator() {
        let mut e = engine();
        counter_map(&mut e);
        let filter = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00de_f2b0, &args![filter, 1u32, 2u32]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_INSERT),
            vec![args![filter + MAP, ALLOCATOR, 2u32, 1u32, 1u32, 0u32].to_vec()]
        );
    }

    #[test]
    fn looking_up_a_pair_compares_the_index_with_the_highest_one() {
        let mut e = engine();
        let filter = e.mem.alloc(0x60);
        let out = e.mem.alloc(4);
        e.mem.set_u32(filter + MAP + 8, 7);
        for (index, expected) in [(7u32, 0u8), (8, 1), (0, 0), (0xffff_ffff, 0)] {
            e.register_double(MAP_FIND_KEY, move |_, _| Ret {
                eax: index,
                ..Ret::default()
            });
            e.mem.set_u8(out, 9);
            e.call_log = Some(vec![]);
            e.call(0x00de_f3d0, &args![filter, out, 10u32, 20u32]);
            let log: Log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u8(out), expected, "index {index:#x}");
            assert_eq!(
                calls_to(&log, MAP_FIND_KEY),
                vec![args![filter + MAP, 20u32, 10u32].to_vec()]
            );
        }
    }

    #[test]
    fn a_null_entity_is_always_enabled() {
        let mut e = engine();
        e.register(MAP_FIND_KEY, |_, _| Ret::default());
        let filter = e.mem.alloc(0x60);
        let out = e.mem.alloc(4);
        for (a, b) in [(0u32, 5u32), (5, 0), (0, 0)] {
            e.mem.set_u8(out, 9);
            e.call_log = Some(vec![]);
            e.call(0x00de_f3d0, &args![filter, out, a, b]);
            let log: Log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u8(out), 1);
            assert!(calls_to(&log, MAP_FIND_KEY).is_empty());
        }
    }

    /// A filter object whose interface (at `object + offset`) answers `answer`.
    fn answering_object(e: &mut Engine, offset: u32, answer: u8) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, FAKE_ENABLED);
        e.mem.set_u32(object + offset, vtable);
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, answer);
        e.register(FAKE_ENABLED, |e, _| {
            // The test puts the byte's address in a fixed global instead of a capture.
            Ret {
                eax: e.mem.u32(0x0030_3000),
                ..Ret::default()
            }
        });
        e.map(0x0030_3000, 4);
        e.mem.set_u32(0x0030_3000, byte);
        object
    }

    #[test]
    fn is_collision_enabled_asks_the_second_filter() {
        let mut e = engine();
        let filter = e.mem.alloc(0x60);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(0x00de_f120, &args![filter, out, 1u32, 2u32]);
        assert_eq!(e.mem.u8(out), 1);
        let object = answering_object(&mut e, 0x14, 0);
        e.mem.set_u32(filter + FILTER_OBJECT, object);
        e.call_log = Some(vec![]);
        e.call(0x00de_f120, &args![filter, out, 1u32, 2u32]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(out), 0);
        let asked = calls_to(&log, FAKE_ENABLED);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], object + 0x14);
        assert_eq!(asked[0][2..], [1, 2]);
        let object = answering_object(&mut e, 0x14, 1);
        e.mem.set_u32(filter + FILTER_OBJECT, object);
        e.call(0x00de_f120, &args![filter, out, 1u32, 2u32]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn fn_00def120_answers_its_result_pointer() {
        let mut e = engine();
        let filter = e.mem.alloc(0x60);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00de_f120, &args![filter, out, 1u32, 2u32]).u32(),
            args![out][0]
        );
    }

    /// Entity handles: byte `+0x18` = `flag`, byte `+0x10` = -0x20, so the entity is
    /// `handle - 0x20`.
    fn handle(e: &mut Engine, flag: u8) -> u32 {
        let base = e.mem.alloc(0x100);
        e.mem.set_u8(base + 0x80 + 0x18, flag);
        e.mem.set_i8(base + 0x80 + 0x10, -0x20);
        base + 0x80
    }

    #[test]
    fn the_second_overload_turns_handles_into_entities_for_the_lookup() {
        let mut e = engine();
        e.register_double(MAP_FIND_KEY, |_, _| Ret::default());
        let base = e.mem.alloc(0x60);
        let sub = base + 8;
        let out = e.mem.alloc(4);
        // A handle whose flag is not 1 stands for no entity, so the lookup is skipped.
        let (first, second) = (handle(&mut e, 1), handle(&mut e, 0));
        e.mem.set_u8(out, 9);
        e.call_log = Some(vec![]);
        let got = e.call(0x00de_f450, &args![sub, out, first, second]).u32();
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        assert!(calls_to(&log, MAP_FIND_KEY).is_empty());
        // Two real entities are looked up by their pair key.
        let second = handle(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00de_f450, &args![sub, out, first, second]);
        let log: Log = e.call_log.take().unwrap();
        let (low, high) = halves(pair_key(first - 0x20, second - 0x20));
        assert_eq!(
            calls_to(&log, MAP_FIND_KEY),
            vec![args![base + MAP, low, high].to_vec()]
        );
    }

    #[test]
    fn the_second_overload_stops_when_the_second_filter_refuses() {
        let mut e = engine();
        e.register_double(MAP_FIND_KEY, |_, _| Ret::default());
        let base = e.mem.alloc(0x60);
        let sub = base + 8;
        let out = e.mem.alloc(4);
        let (first, second) = (handle(&mut e, 1), handle(&mut e, 1));
        e.mem.set_u8(out, 9);
        let object = answering_object(&mut e, 8, 0);
        e.mem.set_u32(sub + 0x34, object);
        e.call_log = Some(vec![]);
        let got = e.call(0x00de_f450, &args![sub, out, first, second]).u32();
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 0);
        assert!(calls_to(&log, MAP_FIND_KEY).is_empty());
        // An accepting second filter lets the lookup run.
        let object = answering_object(&mut e, 8, 1);
        e.mem.set_u32(sub + 0x34, object);
        e.call_log = Some(vec![]);
        e.call(0x00de_f450, &args![sub, out, first, second]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MAP_FIND_KEY).len(), 1);
    }

    /// A filter object whose interface (at `object + offset`) writes `answer` into the scratch
    /// hkBool it is given and answers that scratch, as the exe's `isCollisionEnabled`s do.
    fn writing_object(e: &mut Engine, offset: u32, answer: u8) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, FAKE_WRITES_ITS_SCRATCH);
        e.mem.set_u32(object + offset, vtable);
        e.register_double(FAKE_WRITES_ITS_SCRATCH, move |e, a| {
            e.mem.set_u8(a[1], answer);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        object
    }

    /// Both overloads test the hkBool the second filter writes while the scratch is still
    /// there (the exe's is an argument slot of the frame). With the freed-block check a scratch
    /// freed before the test would read as the fill word's low byte.
    #[test]
    fn the_second_filter_s_answer_is_read_from_a_live_scratch() {
        for answer in [0u8, 1] {
            let mut e = engine();
            e.mem.enable_freed_check(true);
            let out = e.mem.alloc(4);
            // 00def120: the answer is the result.
            let filter = e.mem.alloc(0x60);
            let object = writing_object(&mut e, 0x14, answer);
            e.mem.set_u32(filter + FILTER_OBJECT, object);
            e.mem.set_u8(out, 9);
            e.call(0x00de_f120, &args![filter, out, 1u32, 2u32]);
            assert_eq!(e.mem.u8(out), answer, "00def120, answer {answer}");
            // 00def450: an accepting second filter goes on to the lookup, which handles that
            // stand for no entity answer 1.
            let base = e.mem.alloc(0x60);
            let sub = base + 8;
            let object = writing_object(&mut e, 8, answer);
            e.mem.set_u32(sub + 0x34, object);
            let (first, second) = (handle(&mut e, 0), handle(&mut e, 0));
            e.mem.set_u8(out, 9);
            e.call(0x00de_f450, &args![sub, out, first, second]);
            assert_eq!(e.mem.u8(out), answer, "00def450, answer {answer}");
        }
    }

    #[test]
    fn the_constructor_and_destructor_manage_the_map_and_the_reference() {
        let mut e = engine();
        for address in [
            BASE_CONSTRUCT,
            MAP_RESERVE,
            ADD_REFERENCE,
            RELEASE_REFERENCE,
            MAP_CLEAR_AND_DEALLOCATE,
            EMPTY_CALL,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let filter = e.mem.alloc(0x60);
        e.mem.set_u32(filter + MAP + 8, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00de_f350, &args![filter, 0x5150u32]).u32(), filter);
        let log: Log = e.call_log.take().unwrap();
        let order: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (BASE_CONSTRUCT, args![filter].to_vec()),
                (MEMORY_ROUTER, args![].to_vec()),
                (MAP_RESERVE, args![filter + MAP, ALLOCATOR, 0u32].to_vec()),
                (ADD_REFERENCE, args![0x5150u32].to_vec()),
            ]
        );
        for (offset, vtable) in VTABLES {
            assert_eq!(e.mem.u32(filter + offset), vtable);
        }
        assert_eq!(e.mem.u32(filter + MAP), 0);
        assert_eq!(e.mem.u32(filter + MAP + 4), 0);
        assert_eq!(e.mem.u32(filter + MAP + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(filter + REFERENCED), 0x5150);
        assert_eq!(e.mem.u32(filter + 0x20), 5);
        e.call_log = Some(vec![]);
        e.call(0x00de_f160, &args![filter]);
        let log: Log = e.call_log.take().unwrap();
        let order: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (RELEASE_REFERENCE, args![0x5150u32].to_vec()),
                (MEMORY_ROUTER, args![].to_vec()),
                (
                    MAP_CLEAR_AND_DEALLOCATE,
                    args![filter + MAP, ALLOCATOR].to_vec()
                ),
                (EMPTY_CALL, args![filter + MAP].to_vec()),
            ]
        );
        for (offset, vtable) in FINAL_VTABLES {
            assert_eq!(e.mem.u32(filter + offset), vtable);
        }
    }

    #[test]
    fn a_null_object_is_neither_referenced_nor_released() {
        let mut e = engine();
        for address in [
            BASE_CONSTRUCT,
            MAP_RESERVE,
            ADD_REFERENCE,
            RELEASE_REFERENCE,
            MAP_CLEAR_AND_DEALLOCATE,
            EMPTY_CALL,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let filter = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00de_f350, &args![filter, 0u32]);
        e.call(0x00de_f160, &args![filter]);
        let log: Log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ADD_REFERENCE).is_empty());
        assert!(calls_to(&log, RELEASE_REFERENCE).is_empty());
    }
}
