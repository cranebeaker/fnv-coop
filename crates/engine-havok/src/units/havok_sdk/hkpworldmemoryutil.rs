//! `Havok SDK/hkpworldmemoryutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// Memory router (`00c85750`): its field `+0x10` is the allocator, `+0x18` the memory checker.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpWorld` functions (`ECX` the world): the critical section enter and leave (`00c91180`,
/// `00c91190`), `getMemUsageForIntegration` (`00c92b50`, one stack word: a three-word usage
/// block) and `getMemoryWatchDog` (`00c93570`).
const WORLD_LOCK_ENTER: u32 = 0x00c9_1180;
const WORLD_LOCK_LEAVE: u32 = 0x00c9_1190;
const GET_MEM_USAGE_FOR_INTEGRATION: u32 = 0x00c9_2b50;
const GET_MEMORY_WATCH_DOG: u32 = 0x00c9_3570;
/// Splits a simulation island (cdecl `(world, island)`).
const SPLIT_SIMULATION_ISLAND: u32 = 0x00cb_6e70;
/// Island check (`ECX` the island; result in `AL`): set when the watch dog may be asked to
/// handle the island.
const ISLAND_CAN_BE_HANDLED: u32 = 0x00d0_77d0;
/// `hkSetOutOfMemoryState` (cdecl, one word) and the global (`.data`) that records an out of
/// memory condition.
const SET_OUT_OF_MEMORY_STATE: u32 = 0x00d8_8040;
const OUT_OF_MEMORY_FLAG: u32 = 0x0126_9200;
/// Profile label of the record that opens the watch dog's handling in `watchHeapMemory`.
const PROFILE_WATCH_DOG: u32 = 0x010c_cb6c;

// Translated from 00d01bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldMemoryUtil::checkMemoryForIntegration` (Xbox PDB name, engine map): cdecl, one
/// world. Inside the world's critical section (`00c91180`, left by `00c91190`) it keeps asking
/// the memory checker (the router's object at `+0x18`, virtual function `0x20`) whether the
/// first word of the integration memory usage (`00c92b50`, a three-word block whose third word
/// is the island to blame) can be had. While not: an island whose flag bits `0xC` of the byte
/// at `+0x25` are set is split (`00cb6e70`) until the usage's island or its first word
/// changes, halving the world's word at `+0xB4` each time the split changed nothing and while
/// the island still has those bits; when still not available, an island that the check
/// `00d077d0` rejects gets bit 2 of its byte set and bit 3 cleared, else the watch dog
/// (`getMemoryWatchDog`, virtual function `0x14` with the island and zero) is asked and the
/// usage is read again. The world's word at `+0xB4` is restored on exit.
pub fn hkp_world_memory_util_check_memory_for_integration(e: &mut Engine, world: Ptr) {
    let w = world.addr();
    e.call(WORLD_LOCK_ENTER, &args![w]);
    let saved = e.mem.u32(w + 0xb4);
    e.with_stack(0xc, |e, usage| {
        let usage = usage.addr();
        e.call(GET_MEM_USAGE_FOR_INTEGRATION, &args![w, usage]);
        if !memory_available(e, usage) {
            loop {
                let mut island = e.mem.u32(usage + 8);
                let mut bits = (e.mem.u8(island + 0x25) >> 2) & 3;
                while bits != 0 {
                    let largest = e.mem.u32(usage);
                    let start = island;
                    if bits != 0 {
                        loop {
                            e.call(SPLIT_SIMULATION_ISLAND, &args![w, start]);
                            e.call(GET_MEM_USAGE_FOR_INTEGRATION, &args![w, usage]);
                            island = e.mem.u32(usage + 8);
                            if island != start || e.mem.u32(usage) != largest {
                                break;
                            }
                            let halved = e.mem.u32(w + 0xb4) >> 1;
                            e.mem.set_u32(w + 0xb4, halved);
                            if e.mem.u8(island + 0x25) & 0xc == 0 {
                                break;
                            }
                        }
                    }
                    bits = (e.mem.u8(island + 0x25) >> 2) & 3;
                }
                if !memory_available(e, usage) {
                    let island = e.mem.u32(usage + 8);
                    if !e.call(ISLAND_CAN_BE_HANDLED, &args![island]).bool() {
                        let flags = (e.mem.u8(island + 0x25) & 0xf7) | 4;
                        e.mem.set_u8(island + 0x25, flags);
                    } else {
                        let watch_dog = e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32();
                        e.vcall(watch_dog, 0x14, &args![island, 0u32]);
                        e.call(GET_MEM_USAGE_FOR_INTEGRATION, &args![w, usage]);
                    }
                }
                if memory_available(e, usage) {
                    break;
                }
            }
        }
    });
    e.mem.set_u32(w + 0xb4, saved);
    e.call(WORLD_LOCK_LEAVE, &args![w]);
}

/// Asks the router's memory checker (object at `+0x18`, virtual function `0x20`) whether the
/// first word of the usage block (`usage`) can be allocated.
fn memory_available(e: &mut Engine, usage: u32) -> bool {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let checker = e.mem.u32(router + 0x18);
    let bytes = e.mem.u32(usage);
    e.vcall(checker, 0x20, &args![bytes]).bool()
}

// Translated from 00d01ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, one world (unnamed): with the mode word at `+0x28` of the simulation (`world + 8`)
/// zero or 2, calls the simulation's virtual function `0x14`; then, with the mode (read
/// again) zero or 3, tail-jumps to its virtual function `0x18` and returns its result, else
/// returns the mode.
pub fn fn_00d01ce0(e: &mut Engine, world: Ptr) -> u32 {
    let w = world.addr();
    let simulation = e.mem.u32(w + 8);
    let mode = e.mem.u32(simulation + 0x28);
    if mode == 0 || mode == 2 {
        let simulation = e.mem.u32(w + 8);
        e.vcall(simulation, 0x14, &args![]);
    }
    let simulation = e.mem.u32(w + 8);
    let mode = e.mem.u32(simulation + 0x28);
    if mode == 0 || mode == 3 {
        e.vcall(simulation, 0x18, &args![]).u32()
    } else {
        mode
    }
}

// Translated from 00d01d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldMemoryUtil::watchHeapMemory` (Xbox PDB name, engine map): cdecl, one world. With a
/// memory watch dog (`00c93570`) asks the router's allocator (`+0x10`, virtual function
/// `0x24`) whether the watch dog's size (its virtual function `0xC`) is available; when not,
/// brackets a call of the watch dog's virtual function `0x10` (with the world) in profile
/// records (`010ccb6c` and the block end). Then, when the out of memory flag `01269200` is
/// set, ends with a tail call of `hkSetOutOfMemoryState(0)` (`00d88040`).
pub fn hkp_world_memory_util_watch_heap_memory(e: &mut Engine, world: Ptr) {
    let w = world.addr();
    if e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32() == 0 {
        return;
    }
    let watch_dog = e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32();
    let size = e.vcall(watch_dog, 0xc, &args![]).u32();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if !e.vcall(allocator, 0x24, &args![size]).bool() {
        profile_record(e, PROFILE_WATCH_DOG);
        let watch_dog = e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32();
        e.vcall(watch_dog, 0x10, &args![w]);
        profile_record(e, PROFILE_END);
    }
    if e.global::<u32>(OUT_OF_MEMORY_FLAG) != 0 {
        e.call(SET_OUT_OF_MEMORY_STATE, &args![0u32]);
    }
}

// Translated from 00d01df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldMemoryUtil::tryToRecoverFromMemoryErrors` (Xbox PDB name, engine map): cdecl, one
/// world. With a memory watch dog (`00c93570`) and a non-zero mode word at `+0x28` of the
/// simulation (`world + 8`), up to ten times: asks the watch dog's virtual function `0x10`
/// with the world, calls `hkSetOutOfMemoryState(0)` (`00d88040`) and `00d01ce0`, until the mode
/// word is zero.
pub fn hkp_world_memory_util_try_to_recover_from_memory_errors(e: &mut Engine, world: Ptr) {
    let w = world.addr();
    if e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32() == 0 {
        return;
    }
    let simulation = e.mem.u32(w + 8);
    if e.mem.u32(simulation + 0x28) == 0 {
        return;
    }
    let mut remaining = 10i32;
    loop {
        let before = remaining;
        remaining -= 1;
        if before <= 0 {
            break;
        }
        let watch_dog = e.call(GET_MEMORY_WATCH_DOG, &args![w]).u32();
        e.vcall(watch_dog, 0x10, &args![w]);
        e.call(SET_OUT_OF_MEMORY_STATE, &args![0u32]);
        e.call(0x00d0_1ce0, &args![w]);
        let simulation = e.mem.u32(w + 8);
        if e.mem.u32(simulation + 0x28) == 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d0_1bc0,
            hkp_world_memory_util_check_memory_for_integration(Ptr)
        ),
        entry!(0x00d0_1ce0, fn_00d01ce0(Ptr) -> u32),
        entry!(0x00d0_1d20, hkp_world_memory_util_watch_heap_memory(Ptr)),
        entry!(
            0x00d0_1df0,
            hkp_world_memory_util_try_to_recover_from_memory_errors(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;

    /// An engine with the exe's data range mapped, a world (returned) whose simulation has
    /// mode word `mode` and `+0xB4` word `0x55`, and doubles that log the world functions.
    fn setup(mode: u32) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let world = e.mem.alloc(0x100);
        let simulation = e.mem.alloc(0x40);
        e.mem.set_u32(world + 8, simulation);
        e.mem.set_u32(world + 0xb4, 0x55);
        e.mem.set_u32(simulation + 0x28, mode);
        log_callees(
            &mut e,
            &log,
            &[WORLD_LOCK_ENTER, WORLD_LOCK_LEAVE, SPLIT_SIMULATION_ISLAND],
        );
        (e, log, world)
    }

    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    /// A router whose memory checker (`+0x18`, slot `0x20`) answers from `answers` (the last
    /// one repeats) and whose allocator (`+0x10`, slot `0x24`) answers `allocator_ok`. Both
    /// log. Returns the router.
    fn memory(e: &mut Engine, log: &Log, answers: Vec<bool>, allocator_ok: bool) -> u32 {
        let router = e.mem.alloc(0x20);
        let checker = e.mem.alloc(0x10);
        let checker_table = e.mem.alloc(0x40);
        e.mem.set_u32(checker, checker_table);
        e.mem.set_u32(checker_table + 0x20, SLOT_BASE + 0x20);
        e.mem.set_u32(router + 0x18, checker);
        let allocator = e.mem.alloc(0x10);
        let allocator_table = e.mem.alloc(0x40);
        e.mem.set_u32(allocator, allocator_table);
        e.mem.set_u32(allocator_table + 0x24, SLOT_BASE + 0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let mut answers: VecDeque<bool> = answers.into();
        let sink = Rc::clone(log);
        e.register_double(SLOT_BASE + 0x20, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x20, a.to_vec()));
            let answer = if answers.len() > 1 {
                answers.pop_front().unwrap()
            } else {
                answers[0]
            };
            Ret {
                eax: answer as u32,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(log);
        e.register_double(SLOT_BASE + 0x24, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x24, a.to_vec()));
            Ret {
                eax: allocator_ok as u32,
                ..Ret::default()
            }
        });
        router
    }

    /// `getMemUsageForIntegration` double: writes the next `(first, second, island)` of
    /// `usages` into the block (the last repeats).
    fn usages(e: &mut Engine, log: &Log, usages: Vec<[u32; 3]>) {
        let mut usages: VecDeque<[u32; 3]> = usages.into();
        let sink = Rc::clone(log);
        e.register_double(GET_MEM_USAGE_FOR_INTEGRATION, move |e, a| {
            sink.borrow_mut()
                .push((GET_MEM_USAGE_FOR_INTEGRATION, a.to_vec()));
            let usage = if usages.len() > 1 {
                usages.pop_front().unwrap()
            } else {
                usages[0]
            };
            for (i, word) in usage.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            Ret::default()
        });
    }

    /// A watch dog (returned) with logging virtual functions `0xC` (answers `size`), `0x10`
    /// and `0x14`, handed out by `getMemoryWatchDog`.
    fn watch_dog(e: &mut Engine, log: &Log, size: u32) -> u32 {
        let dog = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(dog, table);
        for slot in [0xc, 0x10, 0x14] {
            e.mem.set_u32(table + slot, SLOT_BASE + slot);
        }
        let sink = Rc::clone(log);
        e.register_double(SLOT_BASE + 0xc, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 0xc, a.to_vec()));
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        log_callees(e, log, &[SLOT_BASE + 0x10, SLOT_BASE + 0x14]);
        e.register_double(GET_MEMORY_WATCH_DOG, move |_, _| Ret {
            eax: dog,
            ..Ret::default()
        });
        dog
    }

    #[test]
    fn check_memory_for_integration_leaves_when_memory_is_available() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![true], true);
        usages(&mut e, &log, vec![[0x300, 0, 0x7000]]);
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(
            addresses(&log),
            vec![
                WORLD_LOCK_ENTER,
                GET_MEM_USAGE_FOR_INTEGRATION,
                SLOT_BASE + 0x20,
                WORLD_LOCK_LEAVE
            ]
        );
        let asked = calls_to(&log, SLOT_BASE + 0x20);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][1], 0x300);
        assert_eq!(e.mem.u32(world + 0xb4), 0x55);
    }

    #[test]
    fn check_memory_for_integration_marks_an_island_the_check_rejects() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![false, false, true], true);
        let island = e.mem.alloc(0x40);
        e.mem.set_u8(island + 0x25, 0x01);
        usages(&mut e, &log, vec![[0x300, 0, island]]);
        e.register(ISLAND_CAN_BE_HANDLED, |_, _| Ret::default());
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(e.mem.u8(island + 0x25), 0x05);
        assert!(calls_to(&log, SPLIT_SIMULATION_ISLAND).is_empty());
        assert_eq!(calls_to(&log, GET_MEM_USAGE_FOR_INTEGRATION).len(), 1);
        assert_eq!(e.mem.u32(world + 0xb4), 0x55);
    }

    #[test]
    fn check_memory_for_integration_asks_the_watch_dog_about_a_handled_island() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![false, false, true], true);
        let island = e.mem.alloc(0x40);
        e.mem.set_u8(island + 0x25, 0x01);
        usages(&mut e, &log, vec![[0x300, 0, island]]);
        e.register(ISLAND_CAN_BE_HANDLED, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let dog = watch_dog(&mut e, &log, 0);
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(e.mem.u8(island + 0x25), 0x01);
        assert_eq!(calls_to(&log, SLOT_BASE + 0x14), vec![vec![dog, island, 0]]);
        assert_eq!(calls_to(&log, GET_MEM_USAGE_FOR_INTEGRATION).len(), 2);
    }

    #[test]
    fn check_memory_for_integration_splits_until_the_largest_island_changes() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![false, true], true);
        let first = e.mem.alloc(0x40);
        e.mem.set_u8(first + 0x25, 0x04);
        let second = e.mem.alloc(0x40);
        usages(
            &mut e,
            &log,
            vec![[100, 0, first], [100, 0, first], [100, 0, second]],
        );
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(
            calls_to(&log, SPLIT_SIMULATION_ISLAND),
            vec![vec![world, first], vec![world, first]]
        );
        assert_eq!(e.mem.u32(world + 0xb4), 0x55);
        assert_eq!(calls_to(&log, GET_MEM_USAGE_FOR_INTEGRATION).len(), 3);
    }

    #[test]
    fn check_memory_for_integration_halves_the_world_word_while_splitting_changes_nothing() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![false, true], true);
        let island = e.mem.alloc(0x40);
        e.mem.set_u8(island + 0x25, 0x04);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        let mut calls = 0;
        e.register_double(GET_MEM_USAGE_FOR_INTEGRATION, move |e, a| {
            calls += 1;
            record.borrow_mut().push(e.mem.u32(a[0] + 0xb4));
            if calls == 3 {
                e.mem.set_u8(island + 0x25, 0);
            }
            e.mem.set_u32(a[1], 100);
            e.mem.set_u32(a[1] + 8, island);
            Ret::default()
        });
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(*seen.borrow(), vec![0x55, 0x55, 0x2a]);
        assert_eq!(e.mem.u32(world + 0xb4), 0x55);
    }

    #[test]
    fn check_memory_for_integration_goes_round_again_while_memory_is_missing() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![false, false, false, true, true], true);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        usages(&mut e, &log, vec![[100, 0, first], [100, 0, second]]);
        e.register(ISLAND_CAN_BE_HANDLED, |_, _| Ret::default());
        e.call(0x00d0_1bc0, &args![world]);
        assert_eq!(e.mem.u8(first + 0x25), 0x04);
        assert_eq!(
            calls_to(&log, SPLIT_SIMULATION_ISLAND),
            vec![vec![world, first]]
        );
        assert_eq!(calls_to(&log, SLOT_BASE + 0x20).len(), 5);
    }

    #[test]
    fn fn_00d01ce0_runs_the_simulation_steps_the_mode_asks_for() {
        let (mut e, log, world) = setup(0);
        let simulation = e.mem.u32(world + 8);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(simulation, table);
        e.mem.set_u32(table + 0x14, SLOT_BASE + 0x14);
        e.mem.set_u32(table + 0x18, SLOT_BASE + 0x18);
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0x14, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x14, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_BASE + 0x18, move |_, a| {
            sink.borrow_mut().push((SLOT_BASE + 0x18, a.to_vec()));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        // Mode 0: both functions, the result of the second.
        assert_eq!(e.call(0x00d0_1ce0, &args![world]).u32(), 0x77);
        assert_eq!(
            *log.borrow(),
            vec![
                (SLOT_BASE + 0x14, vec![simulation]),
                (SLOT_BASE + 0x18, vec![simulation])
            ]
        );
        // Mode 2: only the first, and the mode is returned.
        log.borrow_mut().clear();
        e.mem.set_u32(simulation + 0x28, 2);
        assert_eq!(e.call(0x00d0_1ce0, &args![world]).u32(), 2);
        assert_eq!(addresses(&log), vec![SLOT_BASE + 0x14]);
        // Mode 3: only the second.
        log.borrow_mut().clear();
        e.mem.set_u32(simulation + 0x28, 3);
        assert_eq!(e.call(0x00d0_1ce0, &args![world]).u32(), 0x77);
        assert_eq!(addresses(&log), vec![SLOT_BASE + 0x18]);
        // Mode 1: neither.
        log.borrow_mut().clear();
        e.mem.set_u32(simulation + 0x28, 1);
        assert_eq!(e.call(0x00d0_1ce0, &args![world]).u32(), 1);
        assert!(log.borrow().is_empty());
    }

    /// An engine whose profiling stream (returned) has room for two records.
    fn profiled(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(0x30);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x18);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    #[test]
    fn watch_heap_memory_without_a_watch_dog_does_nothing() {
        let (mut e, log, world) = setup(0);
        e.register(GET_MEMORY_WATCH_DOG, |_, _| Ret::default());
        e.set_global(OUT_OF_MEMORY_FLAG, 1u32);
        log_callees(&mut e, &log, &[SET_OUT_OF_MEMORY_STATE]);
        e.call(0x00d0_1d20, &args![world]);
        assert_eq!(addresses(&log), Vec::<u32>::new());
    }

    #[test]
    fn watch_heap_memory_brackets_the_watch_dog_when_the_allocator_declines() {
        let (mut e, log, world) = setup(0);
        let router = memory(&mut e, &log, vec![true], false);
        let allocator = e.mem.u32(router + 0x10);
        let dog = watch_dog(&mut e, &log, 0x1234);
        let stream = profiled(&mut e);
        let first_record = e.mem.u32(stream + 4);
        e.set_global(OUT_OF_MEMORY_FLAG, 1u32);
        log_callees(&mut e, &log, &[SET_OUT_OF_MEMORY_STATE]);
        e.call(0x00d0_1d20, &args![world]);
        assert_eq!(
            addresses(&log),
            vec![
                SLOT_BASE + 0xc,
                SLOT_BASE + 0x24,
                SLOT_BASE + 0x10,
                SET_OUT_OF_MEMORY_STATE
            ]
        );
        assert_eq!(calls_to(&log, SLOT_BASE + 0xc), vec![vec![dog]]);
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x24),
            vec![vec![allocator, 0x1234]]
        );
        assert_eq!(calls_to(&log, SLOT_BASE + 0x10), vec![vec![dog, world]]);
        assert_eq!(calls_to(&log, SET_OUT_OF_MEMORY_STATE), vec![vec![0]]);
        assert_eq!(e.mem.u32(first_record), PROFILE_WATCH_DOG);
        assert_eq!(e.mem.u32(first_record + 0xc), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), first_record + 0x18);
    }

    #[test]
    fn watch_heap_memory_leaves_the_watch_dog_alone_when_the_allocator_agrees() {
        let (mut e, log, world) = setup(0);
        memory(&mut e, &log, vec![true], true);
        watch_dog(&mut e, &log, 8);
        e.call(0x00d0_1d20, &args![world]);
        assert_eq!(addresses(&log), vec![SLOT_BASE + 0xc, SLOT_BASE + 0x24]);
    }

    #[test]
    fn try_to_recover_from_memory_errors_needs_a_watch_dog_and_a_mode() {
        let (mut e, log, world) = setup(0);
        e.register(GET_MEMORY_WATCH_DOG, |_, _| Ret::default());
        e.call(0x00d0_1df0, &args![world]);
        assert!(log.borrow().is_empty());
        watch_dog(&mut e, &log, 0);
        e.call(0x00d0_1df0, &args![world]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn try_to_recover_from_memory_errors_stops_when_the_mode_clears() {
        let (mut e, log, world) = setup(1);
        let dog = watch_dog(&mut e, &log, 0);
        log_callees(&mut e, &log, &[SET_OUT_OF_MEMORY_STATE]);
        let simulation = e.mem.u32(world + 8);
        let sink = Rc::clone(&log);
        let mut rounds = 0;
        e.register_double(0x00d0_1ce0, move |e, a| {
            sink.borrow_mut().push((0x00d0_1ce0, a.to_vec()));
            rounds += 1;
            if rounds == 3 {
                e.mem.set_u32(simulation + 0x28, 0);
            }
            Ret::default()
        });
        e.call(0x00d0_1df0, &args![world]);
        assert_eq!(calls_to(&log, SLOT_BASE + 0x10), vec![vec![dog, world]; 3]);
        assert_eq!(calls_to(&log, SET_OUT_OF_MEMORY_STATE), vec![vec![0]; 3]);
        assert_eq!(calls_to(&log, 0x00d0_1ce0), vec![vec![world]; 3]);
    }

    #[test]
    fn try_to_recover_from_memory_errors_gives_up_after_ten_rounds() {
        let (mut e, log, world) = setup(1);
        watch_dog(&mut e, &log, 0);
        log_callees(&mut e, &log, &[SET_OUT_OF_MEMORY_STATE, 0x00d0_1ce0]);
        e.call(0x00d0_1df0, &args![world]);
        assert_eq!(calls_to(&log, 0x00d0_1ce0).len(), 10);
    }
}
