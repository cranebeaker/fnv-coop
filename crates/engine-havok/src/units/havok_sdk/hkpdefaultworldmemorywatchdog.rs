//! `Havok SDK/hkpdefaultworldmemorywatchdog.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpDefaultWorldMemoryWatchDog` is not in the Xbox PDB; the layout is read off the code
//! (descriptive, unconfirmed): `+8` is the amount of heap memory it asks for. The world has an
//! array of active islands (data `+0x28`, size `+0x2c`) and one of inactive islands (data
//! `+0x34`, size `+0x38`); an island has its world at `+0x18`, an array of entities (data `+0x48`,
//! size `+0x4c`) and an entity keeps a signed byte (a priority) at `+0xd0`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld::removeEntity` (Ghidra name; `this` = world, two stack words: the address of a
/// result byte it clears, and the entity).
const REMOVE_ENTITY: u32 = 0x00c9_1620;
/// The memory router (`00c85750`): its `+0x10` is an object whose slot `0x24` takes the amount
/// and answers in `AL`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const SLOT_MEMORY_CHECK: u32 = 0x24;
/// The starting threshold of `freeHeapMemoryTillRequestedAmountIsAvailable`.
const FIRST_THRESHOLD: i32 = 0x7f;

/// The call `00e48290` and `00e48300` make after each step: slot `0x24` of the router's `+0x10`
/// object with the amount; the answer is its low byte.
fn memory_check(e: &mut Engine, amount: u32) -> bool {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let object = e.mem.u32(router + 0x10);
    e.vcall(object, SLOT_MEMORY_CHECK, &args![amount]).u32() as u8 != 0
}

// Translated from 00e48260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMemoryWatchDog::getAmountOfFreeHeapMemoryRequested` (Ghidra name): the word
/// at `+8`.
pub fn hkp_default_world_memory_watch_dog_get_amount_of_free_heap_memory_requested(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00e48270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMemoryWatchDog::reduceConstraintsInIsland` (Ghidra name): `ECX` = watchdog
/// (unused), two stack words: the island and one never read. Removes the island's first entity
/// from the island's world (`hkpWorld::removeEntity`, whose first argument is the address of
/// the stack word that holds the island; the call clears its low byte).
pub fn hkp_default_world_memory_watch_dog_reduce_constraints_in_island(
    e: &mut Engine,
    _this: Ptr,
    island: Ptr,
    _unused_2: u32,
) {
    let entities = e.mem.u32(island.addr() + 0x48);
    let entity = e.mem.u32(entities);
    let world = e.mem.u32(island.addr() + 0x18);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), island.addr());
        e.call(REMOVE_ENTITY, &args![world, slot.addr(), entity]);
    });
}

// Translated from 00e48290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMemoryWatchDog::removeObjectsFromIsland` (Ghidra name), a `cdecl` function of
/// four words: the island, a priority threshold, the amount asked for and the address of a
/// signed word. Walks the island's entities from the last to the first. An entity whose
/// priority (the signed byte at `+0xd0`) is below the threshold raises the word to at least
/// that priority; any other is removed from the island's world (as in
/// `reduceConstraintsInIsland`), and the walk ends when the memory check (see `memory_check`)
/// answers false.
pub fn hkp_default_world_memory_watch_dog_remove_objects_from_island(
    e: &mut Engine,
    island: Ptr,
    threshold: i32,
    amount: u32,
    highest: Ptr,
) {
    let island = island.addr();
    let mut index = e.mem.i32(island + 0x4c).wrapping_sub(1);
    while index >= 0 {
        let entities = e.mem.u32(island + 0x48);
        let entity = e.mem.u32(entities + 4 * index as u32);
        let priority = i32::from(e.mem.u8(entity + 0xd0) as i8);
        if priority < threshold {
            let current = e.mem.i32(highest.addr());
            let kept = current.max(priority);
            e.mem.set_u32(highest.addr(), kept as u32);
        } else {
            let world = e.mem.u32(island + 0x18);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), island);
                e.call(REMOVE_ENTITY, &args![world, slot.addr(), entity]);
            });
            if !memory_check(e, amount) {
                return;
            }
        }
        index -= 1;
    }
}

/// The island at `index` of a world: an index of 0 or more is in the active array, a negative
/// one counts into the inactive array (-1 is its first entry, -2 the second, ...).
fn island_at(e: &Engine, world: u32, index: i32) -> u32 {
    if index >= 0 {
        e.mem.u32(e.mem.u32(world + 0x28) + 4 * index as u32)
    } else {
        let data = e.mem.u32(world + 0x34);
        e.mem
            .u32(data.wrapping_sub((index as u32).wrapping_mul(4).wrapping_add(4)))
    }
}

// Translated from 00e48300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMemoryWatchDog::freeHeapMemoryTillRequestedAmountIsAvailable` (Ghidra name):
/// `ECX` = watchdog, one stack word, the world. With the amount at `+8` and the threshold 0x7f
/// it walks the world's islands from the last active one down through the first, then the
/// inactive ones, running `removeObjectsFromIsland` on each (a word that starts at 0 collects
/// the highest priority below the threshold) and asking the memory check (see `memory_check`)
/// after each; the first true answer ends the function. When a whole pass finds nothing to stop
/// at, the highest priority seen becomes the next threshold and it goes round again, ending
/// when that is not above 0. The island count is read again at every step and may shrink.
pub fn hkp_default_world_memory_watch_dog_free_heap_memory_till_requested_amount_is_available(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
) {
    let amount = e.mem.u32(this.addr() + 8);
    let world = world.addr();
    e.with_stack(4, |e, highest| {
        let mut threshold = FIRST_THRESHOLD;
        loop {
            let mut index = e.mem.i32(world + 0x2c).wrapping_sub(1);
            let mut lowest = e.mem.i32(world + 0x38).wrapping_neg();
            e.mem.set_u32(highest.addr(), 0);
            if index < lowest {
                return;
            }
            loop {
                let active = e.mem.i32(world + 0x2c);
                if index >= active {
                    index = active.wrapping_sub(1);
                    if index < lowest {
                        return;
                    }
                }
                let island = island_at(e, world, index);
                hkp_default_world_memory_watch_dog_remove_objects_from_island(
                    e,
                    Ptr::new(island),
                    threshold,
                    amount,
                    highest,
                );
                if memory_check(e, amount) {
                    return;
                }
                lowest = e.mem.i32(world + 0x38).wrapping_neg();
                index = index.wrapping_sub(1);
                if index < lowest {
                    break;
                }
            }
            threshold = e.mem.i32(highest.addr());
            if threshold <= 0 {
                return;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e48260,
            hkp_default_world_memory_watch_dog_get_amount_of_free_heap_memory_requested(Ptr) -> u32
        ),
        entry!(
            0x00e48270,
            hkp_default_world_memory_watch_dog_reduce_constraints_in_island(Ptr, Ptr, u32)
        ),
        entry!(
            0x00e48290,
            hkp_default_world_memory_watch_dog_remove_objects_from_island(Ptr, i32, u32, Ptr)
        ),
        entry!(
            0x00e48300,
            hkp_default_world_memory_watch_dog_free_heap_memory_till_requested_amount_is_available(
                Ptr, Ptr,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHECK_DOUBLE: u32 = 0x0900_0024;

    /// What the removal double saw: the world, the word the result address held, the entity.
    type Removals = Rc<RefCell<Vec<(u32, u32, u32)>>>;
    type Answers = Rc<RefCell<Vec<bool>>>;

    /// The memory router whose object answers the check with `answers` in turn (it panics
    /// when they run out), and a removal double that records into the returned list.
    fn install(e: &mut Engine, answers: &[bool]) -> (Removals, Answers) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x24, CHECK_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, object);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let queue: Answers = Rc::new(RefCell::new(answers.to_vec()));
        let source = Rc::clone(&queue);
        e.register_double(CHECK_DOUBLE, move |_, a| {
            assert_eq!(a.len(), 2);
            let answer = source.borrow_mut().remove(0);
            Ret {
                eax: 0x1200 + u32::from(answer),
                ..Ret::default()
            }
        });
        let removals = Removals::default();
        let sink = Rc::clone(&removals);
        e.register_double(REMOVE_ENTITY, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2]));
            Ret::default()
        });
        (removals, queue)
    }

    /// An island of the world `world` whose entities have the given priorities.
    fn island(e: &mut Engine, world: u32, priorities: &[i8]) -> (u32, Vec<u32>) {
        let island = e.mem.alloc(0x60);
        e.mem.set_u32(island + 0x18, world);
        let array = e.mem.alloc(4 * priorities.len() as u32 + 4);
        let mut entities = Vec::new();
        for (slot, priority) in priorities.iter().enumerate() {
            let entity = e.mem.alloc(0xe0);
            e.mem.set_u8(entity + 0xd0, *priority as u8);
            e.mem.set_u32(array + 4 * slot as u32, entity);
            entities.push(entity);
        }
        e.mem.set_u32(island + 0x48, array);
        e.mem.set_u32(island + 0x4c, priorities.len() as u32);
        (island, entities)
    }

    #[test]
    fn requested_amount_is_the_word_at_eight() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        assert_eq!(e.call(0x00e4_8260, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn reducing_removes_the_first_entity_of_the_island() {
        let mut e = engine();
        let (removals, _) = install(&mut e, &[]);
        let (island, entities) = island(&mut e, 0x5000, &[1, 2]);
        e.call(0x00e4_8270, &args![0u32, island, 0u32]);
        assert_eq!(*removals.borrow(), vec![(0x5000, island, entities[0])]);
    }

    #[test]
    fn removing_walks_backwards_and_stops_when_the_check_fails() {
        let mut e = engine();
        let (removals, answers) = install(&mut e, &[true, false]);
        let (island, entities) = island(&mut e, 0x5000, &[5, 1, 7]);
        let highest = e.mem.alloc(4);
        e.call(0x00e4_8290, &args![island, 4u32, 0x99u32, highest]);
        // The last entity (7) goes, the middle one (1) is below the threshold, the first (5)
        // goes and the second check says stop.
        assert_eq!(
            *removals.borrow(),
            vec![(0x5000, island, entities[2]), (0x5000, island, entities[0])]
        );
        assert_eq!(e.mem.i32(highest), 1);
        assert!(answers.borrow().is_empty());
    }

    #[test]
    fn removing_ends_at_the_first_failed_check_even_with_entities_left() {
        let mut e = engine();
        let (removals, _) = install(&mut e, &[false]);
        let (high, entities) = island(&mut e, 0x5000, &[5, 9]);
        let highest = e.mem.alloc(4);
        e.mem.set_u32(highest, 3);
        e.call(0x00e4_8290, &args![high, 4u32, 0x99u32, highest]);
        assert_eq!(removals.borrow().len(), 1);
        assert_eq!(removals.borrow()[0].2, entities[1]);
        // A negative priority is compared as a signed byte and the word keeps its larger value.
        let (low, _) = island(&mut e, 0x5000, &[-2]);
        e.call(0x00e4_8290, &args![low, 4u32, 0x99u32, highest]);
        assert_eq!(e.mem.i32(highest), 3);
    }

    /// A world with the given active and inactive islands (the inactive array is read
    /// downwards from its data pointer plus four words).
    fn world(e: &mut Engine, active: &[u32], inactive: &[u32]) -> u32 {
        let world = e.mem.alloc(0x60);
        let list = e.mem.alloc(4 * active.len() as u32 + 4);
        for (slot, island) in active.iter().enumerate() {
            e.mem.set_u32(list + 4 * slot as u32, *island);
        }
        e.mem.set_u32(world + 0x28, list);
        e.mem.set_u32(world + 0x2c, active.len() as u32);
        let cold = e.mem.alloc(4 * inactive.len() as u32 + 4);
        for (slot, island) in inactive.iter().enumerate() {
            e.mem.set_u32(cold + 4 * slot as u32, *island);
        }
        // Entry -1 is `data[0]`, entry -2 is `data[1]`: the code subtracts `4 * index + 4`
        // from the data pointer, so the stored pointer is the first entry for index -1.
        e.mem.set_u32(world + 0x34, cold);
        e.mem.set_u32(world + 0x38, inactive.len() as u32);
        world
    }

    #[test]
    fn freeing_raises_the_threshold_and_stops_on_the_first_true_check() {
        let mut e = engine();
        // Pass 1 (threshold 0x7f): active island (3) -> false, inactive island (5) -> false,
        // so the threshold becomes 5. Pass 2: active island -> false, inactive island: its
        // entity is removed, the check inside the removal says false, then the outer check
        // says true.
        let (removals, answers) = install(&mut e, &[false, false, false, false, true]);
        let (active, _) = island(&mut e, 0, &[3]);
        let (inactive, entities) = island(&mut e, 0, &[5]);
        let populated = world(&mut e, &[active], &[inactive]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x77);
        e.call(0x00e4_8300, &args![this, populated]);
        assert_eq!(*removals.borrow(), vec![(0, inactive, entities[0])]);
        assert!(answers.borrow().is_empty());
    }

    #[test]
    fn freeing_ends_when_nothing_below_the_threshold_is_left() {
        let mut e = engine();
        let (removals, answers) = install(&mut e, &[false]);
        let (active, _) = island(&mut e, 0, &[0]);
        let populated = world(&mut e, &[active], &[]);
        let this = e.mem.alloc(0x10);
        e.call(0x00e4_8300, &args![this, populated]);
        // The only entity is below the threshold and has priority 0, so the highest seen is 0.
        assert!(removals.borrow().is_empty());
        assert!(answers.borrow().is_empty());
        // A world with no islands at all does nothing.
        let empty = world(&mut e, &[], &[]);
        e.call(0x00e4_8300, &args![this, empty]);
    }
}
