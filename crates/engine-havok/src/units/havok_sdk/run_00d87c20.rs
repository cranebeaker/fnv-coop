//! `Havok SDK/run_00d87c20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d87c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, two words (`RET 8`): calls virtual slot `+8` (when
/// `flag` is non-zero) or `+4` (when it is zero) of the object embedded at
/// `this + 0x40`, on that object, with `argument`. Returns the callee's EAX.
pub fn fn_00d87c20(e: &mut Engine, this: Ptr, argument: u32, flag: u8) -> u32 {
    let object = this.addr() + 0x40;
    let slot = if flag != 0 { 8 } else { 4 };
    e.vcall(object, slot, &args![argument]).u32()
}

/// `00cb1e00` (`hkmap.cpp`): `thiscall`, no stack words.
const MAP_CALL: u32 = 0x00cb_1e00;
/// `00c9c380` (`hkpentity.cpp`): `thiscall`, no stack words; a count.
const ENTITY_COUNT: u32 = 0x00c9_c380;
/// `00c9c390` (`hkpentity.cpp`): `thiscall`, one stack word, the index; the item at it.
const ENTITY_ITEM: u32 = 0x00c9_c390;
/// `00def2b0` and `00def1e0` (`hkppaircollisionfilter.cpp`): `thiscall`, two stack words.
const PAIR_FILTER_CHECK_FIRST: u32 = 0x00de_f2b0;
const PAIR_FILTER_CHECK_SECOND: u32 = 0x00de_f1e0;
/// `hkAgentNnMachine_FindAgent` (`00cfdd70`, engine map name): cdecl, two words.
const FIND_AGENT: u32 = 0x00cf_dd70;
/// `hkpWorldAgentUtil::removeAgentAndItsToiEvents` (`00cc1160`, engine map name): cdecl, one
/// word.
const REMOVE_AGENT: u32 = 0x00cc_1160;
/// `00c91120` and `00c91140` (`hkpworld.cpp`, jump thunks): `thiscall`, no stack words.
const WORLD_CALL_FIRST: u32 = 0x00c9_1120;
const WORLD_CALL_SECOND: u32 = 0x00c9_1140;
/// `hkpWorld::reenableCollisionBetweenEntityPair` (`00c913b0`, engine map name): `thiscall`,
/// two stack words.
const REENABLE_COLLISION: u32 = 0x00c9_13b0;
/// `00c91160` (`hkpworld.cpp`, a jump thunk): `thiscall`, one stack word, a pointer to a
/// 16-byte operation record; its callee pops the word.
const WORLD_QUEUE_OPERATION: u32 = 0x00c9_1160;
/// `00d87c20` (this file): `thiscall(this, argument, flag)`.
const NOTIFY_OBJECT: u32 = 0x00d8_7c20;

/// The ending shared by `00d87d70` and `00d87e30`: with the world `[entity + 8]`'s
/// reference count (`+0x94`) raised around `during`, and once it is back to zero, no pending
/// flag at `+0x9c` and the flags at `+0x8c`, `+0xa4` and `+0x90`, the two world calls
/// (`00c91120`, `00c91140`) the exe makes.
fn finish_with_world(e: &mut Engine, world: u32) {
    let count = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, count);
    if count != 0 {
        return;
    }
    if e.mem.u8(world + 0x9c) != 0 {
        return;
    }
    if e.mem.u32(world + 0x8c) != 0 {
        e.call(WORLD_CALL_FIRST, &args![world]);
    }
    if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
        e.call(WORLD_CALL_SECOND, &args![world]);
    }
}

// Translated from 00d87ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): runs `00cb1e00` on `this + 0x30`, then, for the world's
/// two island arrays (the one at `world + 0x28`, count at `+0x2c`, then the one at `+0x34`,
/// count at `+0x38`), for every island and each of its entities (array at `+0x48`, count at
/// `+0x4c`), asks the entity for its count (`00c9c380`) and, for each index below it, takes the
/// item (`00c9c390`) and passes it to the slot `+4` of the object embedded at `this + 0x40`.
/// The counts and data pointers are read again on every pass, as the exe does. The class is
/// unconfirmed.
pub fn fn_00d87ca0(e: &mut Engine, this: Ptr, world: u32) {
    let this = this.addr();
    e.call(MAP_CALL, &args![this.wrapping_add(0x30)]);
    for array in [world.wrapping_add(0x28), world.wrapping_add(0x34)] {
        let mut island_index = 0i32;
        while island_index < e.mem.i32(array + 4) {
            let data = e.mem.u32(array);
            let island = e
                .mem
                .u32(data.wrapping_add((island_index as u32).wrapping_mul(4)));
            let mut entity_index = 0i32;
            while entity_index < e.mem.i32(island + 0x4c) {
                let entities = e.mem.u32(island + 0x48);
                let entity = e
                    .mem
                    .u32(entities.wrapping_add((entity_index as u32).wrapping_mul(4)));
                let count = e.call(ENTITY_COUNT, &args![entity]).i32();
                let mut item_index = 0i32;
                while item_index < count {
                    let item = e.call(ENTITY_ITEM, &args![entity, item_index]).u32();
                    // The object embedded at `this + 0x40`: its vtable's slot `+4`.
                    e.vcall(this.wrapping_add(0x40), 4, &args![item]);
                    item_index += 1;
                }
                entity_index += 1;
            }
            island_index += 1;
        }
    }
}

// Translated from 00d87d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): `request` is a record with the object at `+0xc` (its
/// virtual slot `0x20` answers a type: 0xb ends the function), two bodies at `+0x14` and `+0x18`
/// (each needs a non-zero word at `+0x10`). `00def2b0` on `this - 0x40` with the two bodies must
/// answer 1; then `00cfdd70` on the two words at `+0x10` of the bodies finds an agent; with the
/// world `[first body + 8]` (non-null) the agent goes to `00cc1160` while the world's count at
/// `+0x94` is raised, and the world calls `00c91120` and `00c91140` follow under the
/// conditions of the exe. A null `request` does nothing. The class is unconfirmed.
pub fn fn_00d87d70(e: &mut Engine, this: Ptr, request: u32) {
    if request == 0 {
        return;
    }
    let object = e.mem.u32(request + 0xc);
    if e.vcall(object, 0x20, &args![]).u32() == 0xb {
        return;
    }
    let first = e.mem.u32(request + 0x14);
    if e.mem.u32(first + 0x10) == 0 {
        return;
    }
    let second = e.mem.u32(request + 0x18);
    if e.mem.u32(second + 0x10) == 0 {
        return;
    }
    let answer = e
        .call(
            PAIR_FILTER_CHECK_FIRST,
            &args![this.addr().wrapping_sub(0x40), first, second],
        )
        .u32();
    if answer != 1 {
        return;
    }
    let second_words = e.mem.u32(request + 0x18).wrapping_add(0x10);
    let first_words = e.mem.u32(request + 0x14).wrapping_add(0x10);
    let agent = e.call(FIND_AGENT, &args![first_words, second_words]).u32();
    if agent == 0 {
        return;
    }
    let world = e.mem.u32(first + 8);
    if world == 0 {
        return;
    }
    let count = e.mem.u32(world + 0x94).wrapping_add(1);
    e.mem.set_u32(world + 0x94, count);
    e.call(REMOVE_AGENT, &args![agent]);
    finish_with_world(e, world);
}

// Translated from 00d87e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): the same record checks as `00d87d70` (null record, the
/// object at `+0xc` of type 0xb, two bodies with a non-zero word at `+0x10`), then `00def1e0` on
/// `this - 0x40` with the two bodies: an answer above 0 ends it. When the word at `this - 4` is
/// not null its object's filter (`[[object + 8] + 4]`, called with the object at `+8`, an
/// out buffer and the two words at `+0x10` of the bodies) must give a non-zero byte. With the
/// world `[first body + 8]` (non-null) raised at `+0x94`, `hkpWorld::reenableCollisionBetween`
/// `EntityPair` (`00c913b0`) runs on the two bodies and the same world tail as `00d87d70`
/// follows. The class is unconfirmed.
pub fn fn_00d87e30(e: &mut Engine, this: Ptr, request: u32) {
    let this = this.addr();
    if request == 0 {
        return;
    }
    let object = e.mem.u32(request + 0xc);
    if e.vcall(object, 0x20, &args![]).u32() == 0xb {
        return;
    }
    let first = e.mem.u32(request + 0x14);
    if e.mem.u32(first + 0x10) == 0 {
        return;
    }
    let second = e.mem.u32(request + 0x18);
    if e.mem.u32(second + 0x10) == 0 {
        return;
    }
    let answer = e
        .call(
            PAIR_FILTER_CHECK_SECOND,
            &args![this.wrapping_sub(0x40), first, second],
        )
        .i32();
    if answer > 0 {
        return;
    }
    let holder = e.mem.u32(this.wrapping_sub(4));
    if holder != 0 {
        let holder = e.mem.u32(this.wrapping_sub(4));
        let filter = e.mem.u32(holder + 8);
        let function = e.mem.u32(filter + 4);
        // The result buffer is the exe's own argument slot of the caller.
        let enabled = e.with_stack(8, |e, buffer| {
            let flag = e
                .call(
                    function,
                    &args![
                        holder.wrapping_add(8),
                        buffer.addr(),
                        first.wrapping_add(0x10),
                        second.wrapping_add(0x10)
                    ],
                )
                .u32();
            e.mem.u8(flag) != 0
        });
        if !enabled {
            return;
        }
    }
    let world = e.mem.u32(first + 8);
    if world == 0 {
        return;
    }
    let count = e.mem.u32(world + 0x94).wrapping_add(1);
    e.mem.set_u32(world + 0x94, count);
    e.call(REENABLE_COLLISION, &args![world, first, second]);
    finish_with_world(e, world);
}

/// The 16-byte operation record the two switches below queue on the world: the byte `0x23`
/// at `+0`, the viewer `this - 0x40` at `+4`, the request's second word at `+8` and the
/// enable flag byte at `+0xc` (the other bytes of the record are never written by the exe).
fn queue_operation(e: &mut Engine, world: u32, viewer: u32, second_word: u32, flag: u8) {
    e.with_stack(0x10, |e, record| {
        let record = record.addr();
        e.mem.set_u8(record, 0x23);
        e.mem.set_u32(record + 4, viewer);
        e.mem.set_u32(record + 8, second_word);
        e.mem.set_u8(record + 0xc, flag);
        e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
    });
}

/// Shared body of `00d87f00` and `00d87f60`: `request` points to the pair `(world, word)`; a
/// world with a non-zero word at `+0x94` gets the operation record queued
/// ([`queue_operation`]), otherwise `00d87c20` is called on `this - 0x40` with the word and
/// the flag.
fn queue_or_notify(e: &mut Engine, this: u32, request: u32, flag: u8) {
    let world = e.mem.u32(request);
    let word = e.mem.u32(request + 4);
    if e.mem.u32(world + 0x94) != 0 {
        queue_operation(e, world, this.wrapping_sub(0x40), word, flag);
    } else {
        e.call(NOTIFY_OBJECT, &args![this.wrapping_sub(0x40), word, flag]);
    }
}

// Translated from 00d87f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): `request` is a pair `(world, word)`. When the world's
/// word at `+0x94` is non-zero a 16-byte record (`0x23`, `this - 0x40`, `word`, flag 1) is
/// queued with `00c91160` on the world; otherwise `00d87c20` is called on `this - 0x40` with
/// `word` and flag 1. The unwritten bytes of the exe's stack record are zero here. The class
/// is unconfirmed.
pub fn fn_00d87f00(e: &mut Engine, this: Ptr, request: u32) {
    queue_or_notify(e, this.addr(), request, 1);
}

// Translated from 00d87f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d87f00` with flag 0.
pub fn fn_00d87f60(e: &mut Engine, this: Ptr, request: u32) {
    queue_or_notify(e, this.addr(), request, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d87c20, fn_00d87c20(Ptr, u32, u8) -> u32),
        entry!(0x00d8_7ca0, fn_00d87ca0(Ptr, u32)),
        entry!(0x00d8_7d70, fn_00d87d70(Ptr, u32)),
        entry!(0x00d8_7e30, fn_00d87e30(Ptr, u32)),
        entry!(0x00d8_7f00, fn_00d87f00(Ptr, u32)),
        entry!(0x00d8_7f60, fn_00d87f60(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d87c20_picks_slot_8_for_a_set_flag_and_slot_4_otherwise() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_2000, &[0, 0x0090_5000, 0x0090_5100]);
        e.register(0x0090_5000, |_, a| Ret {
            eax: 1000 + a[1],
            ..Ret::default()
        });
        e.register(0x0090_5100, |_, a| Ret {
            eax: 2000 + a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x40, 0x0090_2000);
        assert_eq!(
            e.call(0x00d8_7c20, &args![Ptr::<()>::new(this), 7u32, 1u8])
                .u32(),
            2007
        );
        assert_eq!(
            e.call(0x00d8_7c20, &args![Ptr::<()>::new(this), 7u32, 0u8])
                .u32(),
            1007
        );
    }
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const TYPE_SLOT: u32 = 0x0900_0020;
    const NOTIFY_SLOT: u32 = 0x0900_0004;
    const FILTER_FUNCTION: u32 = 0x0900_0100;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Registers recording doubles over `addresses` (each answers 0) and returns their log.
    fn record(e: &mut Engine, addresses: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let calls = Rc::clone(&log);
            let address = *address;
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    fn pointer_array(e: &mut Engine, items: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        data
    }

    fn entity_with_items(e: &mut Engine, items: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 8, items);
        block
    }

    #[test]
    fn entities_of_both_island_arrays_are_visited_and_each_item_is_handed_on() {
        let mut e = Engine::new();
        let log = record(&mut e, &[MAP_CALL, NOTIFY_SLOT]);
        e.register(ENTITY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(ENTITY_ITEM, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        // Entities 1, 2, 3 have 2, 0 and 1 items; entity 4 has one.
        let entities = [
            entity_with_items(&mut e, 2),
            entity_with_items(&mut e, 0),
            entity_with_items(&mut e, 1),
            entity_with_items(&mut e, 1),
        ];
        let island = |e: &mut Engine, list: &[u32]| {
            let island = e.mem.alloc(0x60);
            let data = pointer_array(e, list);
            e.mem.set_u32(island + 0x48, data);
            e.mem.set_u32(island + 0x4c, list.len() as u32);
            island
        };
        let first_island = island(&mut e, &entities[..3]);
        let second_island = island(&mut e, &entities[3..]);
        let world = e.mem.alloc(0x80);
        let first_data = pointer_array(&mut e, &[first_island]);
        let second_data = pointer_array(&mut e, &[second_island]);
        e.mem.set_u32(world + 0x28, first_data);
        e.mem.set_u32(world + 0x2c, 1);
        e.mem.set_u32(world + 0x34, second_data);
        e.mem.set_u32(world + 0x38, 1);
        // The object at this + 0x40 has the double as slot 4.
        let this = e.mem.alloc(0x60);
        e.put_vtable(0x0900_0000, &[0, NOTIFY_SLOT]);
        e.mem.set_u32(this + 0x40, 0x0900_0000);
        e.call(0x00d8_7ca0, &args![this, world]);
        let calls = log.borrow();
        assert_eq!(calls[0], (MAP_CALL, vec![this + 0x30]));
        let items: Vec<u32> = calls[1..].iter().map(|(_, a)| a[1]).collect();
        assert_eq!(
            items,
            [entities[0], entities[0] + 1, entities[2], entities[3]]
        );
        assert!(calls[1..].iter().all(|(_, a)| a[0] == this + 0x40));
    }

    #[test]
    fn empty_island_arrays_only_run_the_map_call() {
        let mut e = Engine::new();
        let log = record(&mut e, &[MAP_CALL]);
        let world = e.mem.alloc(0x80);
        let this = e.mem.alloc(0x60);
        e.call(0x00d8_7ca0, &args![this, world]);
        assert_eq!(addresses(&log), [MAP_CALL]);
    }

    /// A request record whose object answers `ty`, with two bodies carrying the given words at
    /// `+0x10`, the first body pointing to `world` at `+8`.
    fn make_request(e: &mut Engine, ty: u32, flags: (u32, u32), world: u32) -> (u32, u32, u32) {
        e.put_vtable(0x0900_0000, &[0, 0, 0, 0, 0, 0, 0, 0, TYPE_SLOT]);
        e.register(TYPE_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0900_0000);
        e.mem.set_u32(object + 8, ty);
        let first = e.mem.alloc(0x40);
        e.mem.set_u32(first + 8, world);
        e.mem.set_u32(first + 0x10, flags.0);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(second + 0x10, flags.1);
        let request = e.mem.alloc(0x20);
        e.mem.set_u32(request + 0xc, object);
        e.mem.set_u32(request + 0x14, first);
        e.mem.set_u32(request + 0x18, second);
        (request, first, second)
    }

    fn make_world(e: &mut Engine) -> u32 {
        let world = e.mem.alloc(0xb0);
        e.mem.set_u32(world + 0x94, 0);
        world
    }

    const THIS: u32 = 0x0001_0040;

    #[test]
    fn disable_path_removes_the_agent_and_runs_the_world_tail() {
        let mut e = Engine::new();
        let world = make_world(&mut e);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        let (request, _, _) = make_request(&mut e, 3, (1, 1), world);
        let log = record(&mut e, &[WORLD_CALL_FIRST, WORLD_CALL_SECOND, REMOVE_AGENT]);
        e.register(PAIR_FILTER_CHECK_FIRST, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(FIND_AGENT, |_, _| Ret {
            eax: 0xa11,
            ..Ret::default()
        });
        e.call(0x00d8_7d70, &args![THIS, request]);
        assert_eq!(
            *log.borrow(),
            [
                (REMOVE_AGENT, vec![0xa11]),
                (WORLD_CALL_FIRST, vec![world]),
                (WORLD_CALL_SECOND, vec![world])
            ]
        );
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn disable_path_stops_early_in_each_of_its_checks() {
        let mut e = Engine::new();
        let world = make_world(&mut e);
        let log = record(&mut e, &[REMOVE_AGENT]);
        let check_answer = Rc::new(RefCell::new(1u32));
        let answer = Rc::clone(&check_answer);
        e.register_double(PAIR_FILTER_CHECK_FIRST, move |_, _| Ret {
            eax: *answer.borrow(),
            ..Ret::default()
        });
        let agent = Rc::new(RefCell::new(0xa11u32));
        let found = Rc::clone(&agent);
        e.register_double(FIND_AGENT, move |_, _| Ret {
            eax: *found.borrow(),
            ..Ret::default()
        });
        // A null request, the wrapped object of type 0xb, a body without its word.
        e.call(0x00d8_7d70, &args![THIS, 0u32]);
        let (typed, _, _) = make_request(&mut e, 0xb, (1, 1), world);
        e.call(0x00d8_7d70, &args![THIS, typed]);
        let (first_missing, _, _) = make_request(&mut e, 3, (0, 1), world);
        e.call(0x00d8_7d70, &args![THIS, first_missing]);
        let (second_missing, _, _) = make_request(&mut e, 3, (1, 0), world);
        e.call(0x00d8_7d70, &args![THIS, second_missing]);
        // The filter's answer must be exactly 1; no agent; no world.
        let (good, first, _) = make_request(&mut e, 3, (1, 1), world);
        *check_answer.borrow_mut() = 2;
        e.call(0x00d8_7d70, &args![THIS, good]);
        *check_answer.borrow_mut() = 1;
        *agent.borrow_mut() = 0;
        e.call(0x00d8_7d70, &args![THIS, good]);
        agent.replace(0xa11);
        e.mem.set_u32(first + 8, 0);
        e.call(0x00d8_7d70, &args![THIS, good]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn world_tail_stops_when_the_count_does_not_return_to_zero_or_a_flag_is_set() {
        let mut e = Engine::new();
        let log = record(&mut e, &[WORLD_CALL_FIRST, WORLD_CALL_SECOND, REMOVE_AGENT]);
        e.register(PAIR_FILTER_CHECK_FIRST, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(FIND_AGENT, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        // Count already 1: it ends at 1 after the decrement, so nothing else runs.
        let busy = make_world(&mut e);
        e.mem.set_u32(busy + 0x94, 1);
        e.mem.set_u32(busy + 0x8c, 1);
        let (request_busy, _, _) = make_request(&mut e, 3, (1, 1), busy);
        e.call(0x00d8_7d70, &args![THIS, request_busy]);
        assert_eq!(e.mem.u32(busy + 0x94), 1);
        // The pending byte stops it before the world calls.
        let pending = make_world(&mut e);
        e.mem.set_u32(pending + 0x8c, 1);
        e.mem.set_u8(pending + 0x9c, 1);
        let (request_pending, _, _) = make_request(&mut e, 3, (1, 1), pending);
        e.call(0x00d8_7d70, &args![THIS, request_pending]);
        // Only the second call's condition holds: flag word 0xa4 is 1 but 0x90 is zero.
        let partial = make_world(&mut e);
        e.mem.set_u32(partial + 0xa4, 1);
        let (request_partial, _, _) = make_request(&mut e, 3, (1, 1), partial);
        e.call(0x00d8_7d70, &args![THIS, request_partial]);
        // The first world call alone.
        let first_only = make_world(&mut e);
        e.mem.set_u32(first_only + 0x8c, 7);
        e.mem.set_u32(first_only + 0xa4, 2);
        e.mem.set_u32(first_only + 0x90, 1);
        let (request_first, _, _) = make_request(&mut e, 3, (1, 1), first_only);
        e.call(0x00d8_7d70, &args![THIS, request_first]);
        assert_eq!(
            addresses(&log),
            [
                REMOVE_AGENT,
                REMOVE_AGENT,
                REMOVE_AGENT,
                REMOVE_AGENT,
                WORLD_CALL_FIRST
            ]
        );
    }

    fn enable_setup(e: &mut Engine) -> (u32, u32, u32, u32, Log) {
        let world = make_world(e);
        let (request, first, second) = make_request(e, 3, (1, 1), world);
        let log = record(
            e,
            &[WORLD_CALL_FIRST, WORLD_CALL_SECOND, REENABLE_COLLISION],
        );
        e.register(PAIR_FILTER_CHECK_SECOND, |_, _| Ret::default());
        (world, request, first, second, log)
    }

    #[test]
    fn enable_path_without_a_filter_holder_reenables_the_pair() {
        let mut e = Engine::new();
        e.map(0x0000_0000, 0x0002_0000);
        let (world, request, first, second, log) = enable_setup(&mut e);
        e.call(0x00d8_7e30, &args![THIS, request]);
        assert_eq!(
            *log.borrow(),
            [(REENABLE_COLLISION, vec![world, first, second])]
        );
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn enable_path_asks_the_filter_when_the_holder_is_set() {
        let mut e = Engine::new();
        e.map(0x0000_0000, 0x0002_0000);
        let (world, request, first, second, log) = enable_setup(&mut e);
        // The holder at this - 4: `[[holder + 8] + 4]` is the filter function.
        let holder = e.mem.alloc(0x20);
        let filter = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 8, filter);
        e.mem.set_u32(filter + 4, FILTER_FUNCTION);
        e.mem.set_u32(THIS - 4, holder);
        let result = Rc::new(RefCell::new(0u8));
        let answer = Rc::clone(&result);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&seen);
        e.register_double(FILTER_FUNCTION, move |e, a| {
            calls.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], *answer.borrow());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(0x00d8_7e30, &args![THIS, request]);
        // The filter said no: nothing was reenabled.
        assert!(log.borrow().is_empty());
        assert_eq!(seen.borrow().len(), 1);
        let call = seen.borrow()[0].clone();
        assert_eq!(
            (call[0], call[2], call[3]),
            (holder + 8, first + 0x10, second + 0x10)
        );
        *result.borrow_mut() = 1;
        e.call(0x00d8_7e30, &args![THIS, request]);
        assert_eq!(
            *log.borrow(),
            [(REENABLE_COLLISION, vec![world, first, second])]
        );
    }

    #[test]
    fn enable_path_stops_on_a_positive_filter_answer_and_the_early_checks() {
        let mut e = Engine::new();
        e.map(0x0000_0000, 0x0002_0000);
        let (world, request, _, _, log) = enable_setup(&mut e);
        e.register(PAIR_FILTER_CHECK_SECOND, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.call(0x00d8_7e30, &args![THIS, request]);
        // Negative counts as "not above 0".
        e.register(PAIR_FILTER_CHECK_SECOND, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.call(0x00d8_7e30, &args![THIS, request]);
        assert_eq!(addresses(&log), [REENABLE_COLLISION]);
        e.call(0x00d8_7e30, &args![THIS, 0u32]);
        let (typed, _, _) = make_request(&mut e, 0xb, (1, 1), world);
        e.call(0x00d8_7e30, &args![THIS, typed]);
        let (no_word, _, _) = make_request(&mut e, 3, (1, 0), world);
        e.call(0x00d8_7e30, &args![THIS, no_word]);
        assert_eq!(addresses(&log).len(), 1);
    }

    fn pair(e: &mut Engine, world: u32, word: u32) -> u32 {
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, world);
        e.mem.set_u32(pair + 4, word);
        pair
    }

    #[test]
    fn queued_operation_or_direct_notification_depends_on_the_worlds_word() {
        let mut e = Engine::new();
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a[0], record));
            Ret::default()
        });
        let notified = record(&mut e, &[NOTIFY_OBJECT]);
        let busy = make_world(&mut e);
        e.mem.set_u32(busy + 0x94, 2);
        let idle = make_world(&mut e);
        let busy_pair = pair(&mut e, busy, 0x77);
        let idle_pair = pair(&mut e, idle, 0x88);
        e.call(0x00d8_7f00, &args![THIS, busy_pair]);
        e.call(0x00d8_7f60, &args![THIS, busy_pair]);
        e.call(0x00d8_7f00, &args![THIS, idle_pair]);
        e.call(0x00d8_7f60, &args![THIS, idle_pair]);
        assert_eq!(
            *calls.borrow(),
            [
                (busy, vec![0x23, THIS - 0x40, 0x77, 1]),
                (busy, vec![0x23, THIS - 0x40, 0x77, 0])
            ]
        );
        assert_eq!(
            *notified.borrow(),
            [
                (NOTIFY_OBJECT, vec![THIS - 0x40, 0x88, 1]),
                (NOTIFY_OBJECT, vec![THIS - 0x40, 0x88, 0])
            ]
        );
    }
}
