//! `Havok SDK/hkpdefaultworldmaintenancemgr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpDefaultWorldMaintenanceMgr` is not in the Xbox PDB, so the layout is read off the code
//! (descriptive, unconfirmed): `+8` is a time the manager restarts the world from and `+0xc` the
//! interval between two maintenance passes. The world's own fields used here are `+8` (an
//! object whose vtable slot `0x38` takes a `float`; fields `+0x14`, `+0x18`, `+0x20`), `+0x28`
//! and `+0x2c` (an array of islands, each with an array of entities at `+0x48` and `+0x4c`;
//! an entity's `float` at `+0x13c`), `+0x78` (an object whose `+0x50` holds a copy of the step
//! info) and `+0x1d0` (the world's step info: start time, end time, delta time and inverse delta
//! time, four `float`s). x87 code rounds at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpbvagent::profile_start_record;
use super::hkpsphereshape::profile_record;

/// The vtable the constructor installs.
const VTABLE: u32 = 0x010c_d34c;
/// The `float`s the constructor copies into `+8` and `+0xc`.
const DEFAULT_TIME: u32 = 0x010c_d368;
const DEFAULT_INTERVAL: u32 = 0x010c_d364;
/// `hkpWorldOperationUtil::splitSimulationIslands` (Ghidra name; cdecl, the world).
const SPLIT_SIMULATION_ISLANDS: u32 = 0x00cb_6f30;
/// The island routine `resetWorldTime` runs on every island (cdecl: island, the step info's end
/// time, the world's end time, the word at world `+0x78`).
const ISLAND_ROUTINE: u32 = 0x00cc_0ed0;
/// The `float` -1.0.
const MINUS_ONE: u32 = 0x0101_2054;
/// Profiling labels: the two begin pairs of `performMaintenance` (the first label, then the
/// second), the label before the time reset, the one for `performMaintenanceNoSplit`, and the
/// end label.
const LABEL_FIRST: u32 = 0x010c_d380;
const LABEL_MAINTENANCE: u32 = 0x010c_d378;
const LABEL_NO_SPLIT: u32 = 0x010c_d36c;
const LABEL_END: u32 = 0x0107_6f38;
/// The slot of the simulation object that takes the time moved.
const SLOT_TIME_MOVED: u32 = 0x38;

// Translated from 00d0b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMaintenanceMgr::init` (Ghidra name): `ECX` = manager, one stack word, the
/// world. Copies the `float` at `+8` into the `float`s at `+0x14` and `+0x18` of the object at
/// the world's `+8`.
pub fn hkp_default_world_maintenance_mgr_init(e: &mut Engine, this: Ptr, world: Ptr) {
    let time = x87_quiet_bits(e.mem.u32(this.addr() + 8));
    let simulation = e.mem.u32(world.addr() + 8);
    e.mem.set_u32(simulation + 0x14, time);
    e.mem.set_u32(simulation + 0x18, time);
}

// Translated from 00d0b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no name in the engine map; `ECX` = object, nothing on the stack): copies
/// the `float`s at `010cd368` and `010cd364` into `+8` and `+0xc`, sets the 16-bit word at `+6`
/// to 1 and the vtable `010cd34c`. Returns `this`.
pub fn fn_00d0b250(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    let time = x87_quiet_bits(e.mem.u32(DEFAULT_TIME));
    e.mem.set_u32(me + 8, time);
    e.mem.set_u16(me + 6, 1);
    let interval = x87_quiet_bits(e.mem.u32(DEFAULT_INTERVAL));
    e.mem.set_u32(me + 0xc, interval);
    e.mem.set_u32(me, VTABLE);
    this
}

/// Whether the step's first `float` has reached the interval at `+0xc`: the x87 compare of
/// `[this + 0xc]` with it leaves "greater" and "unordered" to the skip branch (`JP` after
/// `TEST AH, 0x41`), so equal or less runs the pass.
fn pass_is_due(e: &Engine, this: u32, step: u32) -> bool {
    let interval = e.mem.f32(this + 0xc);
    let reached = e.mem.f32(step);
    interval <= reached
}

// Translated from 00d0b280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMaintenanceMgr::performMaintenance` (Ghidra name): `ECX` = manager, two
/// stack words, the world and the step info. Between begin and end profiling records it splits
/// the simulation islands of the world; when the step info's first `float` is not below the
/// interval at `+0xc` (see `pass_is_due`) it also runs `resetWorldTime` under its own record.
pub fn hkp_default_world_maintenance_mgr_perform_maintenance(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    step: Ptr,
) {
    profile_start_record(e, LABEL_FIRST, LABEL_MAINTENANCE);
    e.call(SPLIT_SIMULATION_ISLANDS, &args![world]);
    if pass_is_due(e, this.addr(), step.addr()) {
        profile_record(e, LABEL_NO_SPLIT);
        hkp_default_world_maintenance_mgr_reset_world_time(e, this, world, step);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d0b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMaintenanceMgr::performMaintenanceNoSplit` (Ghidra name): the same as
/// `performMaintenance` without the island split and the middle record.
pub fn hkp_default_world_maintenance_mgr_perform_maintenance_no_split(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    step: Ptr,
) {
    profile_start_record(e, LABEL_FIRST, LABEL_NO_SPLIT);
    if pass_is_due(e, this.addr(), step.addr()) {
        hkp_default_world_maintenance_mgr_reset_world_time(e, this, world, step);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d0b3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultWorldMaintenanceMgr::resetWorldTime` (Ghidra name): `ECX` = manager, two stack
/// words, the world and the step info (four `float`s: start, end, delta, inverse delta).
/// Rebuilds the world's step info at `+0x1d0` from the manager's `+8` time (start = that time,
/// delta = the step's end minus start, end = delta plus the time, inverse delta = 1 / delta);
/// the time moved `d` (the world's start minus the step's start) is added to `+0x14` and,
/// unless it is -1.0, `+0x20` of the world's `+8` object and to the `float` at `+0x13c` of
/// every entity of every island; the world's start is copied to its `+0x18`. The island routine
/// `00cc0ed0` then runs on every island, the `+8` object's slot `0x38` gets `d`, and the new
/// step info is copied word for word into the step and into the object at the world's `+0x78`
/// (at `+0x50`).
pub fn hkp_default_world_maintenance_mgr_reset_world_time(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    step: Ptr,
) {
    let (world, step) = (world.addr(), step.addr());
    let step_start = e.mem.f32(step);
    let step_end = e.mem.f32(step + 4);
    let delta = (f64::from(step_end) - f64::from(step_start)) as f32;
    let time_bits = x87_quiet_bits(e.mem.u32(this.addr() + 8));
    e.mem.set_u32(world + 0x1d0, time_bits);
    let time = f32::from_bits(e.mem.u32(this.addr() + 8));
    e.mem
        .set_f32(world + 0x1d4, (f64::from(delta) + f64::from(time)) as f32);
    e.mem.set_f32(world + 0x1d8, delta);
    e.mem
        .set_f32(world + 0x1dc, (1.0 / f64::from(delta)) as f32);
    let moved = (f64::from(e.mem.f32(world + 0x1d0)) - f64::from(step_start)) as f32;
    let simulation = e.mem.u32(world + 8);
    let sum = f64::from(moved) + f64::from(e.mem.f32(simulation + 0x14));
    e.mem.set_f32(simulation + 0x14, sum as f32);
    let start_bits = x87_quiet_bits(e.mem.u32(world + 0x1d0));
    e.mem.set_u32(simulation + 0x18, start_bits);
    let minus_one: f32 = e.global(MINUS_ONE);
    if minus_one != e.mem.f32(simulation + 0x20) {
        let sum = f64::from(e.mem.f32(simulation + 0x20)) + f64::from(moved);
        e.mem.set_f32(simulation + 0x20, sum as f32);
    }
    let mut index = 0u32;
    while (e.mem.i32(world + 0x2c)) > index as i32 {
        let island = e.mem.u32(e.mem.u32(world + 0x28) + 4 * index);
        let mut entity_index = 0u32;
        while e.mem.i32(island + 0x4c) > entity_index as i32 {
            let entity = e.mem.u32(e.mem.u32(island + 0x48) + 4 * entity_index);
            let sum = f64::from(moved) + f64::from(e.mem.f32(entity + 0x13c));
            e.mem.set_f32(entity + 0x13c, sum as f32);
            entity_index += 1;
        }
        let args_world_end = x87_quiet_bits(e.mem.u32(world + 0x1d4));
        let args_step_end = x87_quiet_bits(e.mem.u32(step + 4));
        let routine_word = e.mem.u32(world + 0x78);
        e.call(
            ISLAND_ROUTINE,
            &args![island, args_step_end, args_world_end, routine_word],
        );
        index += 1;
    }
    e.vcall(simulation, SLOT_TIME_MOVED, &args![moved]);
    let object = e.mem.u32(world + 0x78);
    for word in 0..4 {
        let value = e.mem.u32(world + 0x1d0 + 4 * word);
        e.mem.set_u32(step + 4 * word, value);
        e.mem.set_u32(object + 0x50 + 4 * word, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0b230, hkp_default_world_maintenance_mgr_init(Ptr, Ptr)),
        entry!(0x00d0b250, fn_00d0b250(Ptr) -> Ptr),
        entry!(
            0x00d0b280,
            hkp_default_world_maintenance_mgr_perform_maintenance(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d0b350,
            hkp_default_world_maintenance_mgr_perform_maintenance_no_split(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d0b3e0,
            hkp_default_world_maintenance_mgr_reset_world_time(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, Log,
    };

    /// The manager: time 10.0 at `+8`, interval 1.0 at `+0xc`.
    fn manager(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 8, 10.0);
        e.mem.set_f32(this + 0xc, 1.0);
        this
    }

    /// A step info: start, end, delta, inverse delta.
    fn step_info(e: &mut Engine, start: f32, end: f32) -> u32 {
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step, start);
        e.mem.set_f32(step + 4, end);
        step
    }

    /// A world with a simulation object (`+0x14` = 1.0, `+0x20` = `late`, a logging slot
    /// `0x38`), a holder at `+0x78`, and the given islands (each a list of entity `float`s).
    fn world(e: &mut Engine, log: &Log, late: f32, islands: &[&[f32]]) -> (u32, u32, u32) {
        let world = e.mem.alloc(0x200);
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x38, 0x0900_0038);
        log_double(e, log, 0x0900_0038, 0);
        let simulation = e.mem.alloc(0x40);
        e.mem.set_u32(simulation, vtable);
        e.mem.set_f32(simulation + 0x14, 1.0);
        e.mem.set_f32(simulation + 0x20, late);
        e.mem.set_u32(world + 8, simulation);
        let list = e.mem.alloc(8 * islands.len() as u32 + 8);
        for (index, entities) in islands.iter().enumerate() {
            let island = e.mem.alloc(0x60);
            let array = e.mem.alloc(8 * entities.len() as u32 + 8);
            for (slot, value) in entities.iter().enumerate() {
                let entity = e.mem.alloc(0x150);
                e.mem.set_f32(entity + 0x13c, *value);
                e.mem.set_u32(array + 4 * slot as u32, entity);
            }
            e.mem.set_u32(island + 0x48, array);
            e.mem.set_u32(island + 0x4c, entities.len() as u32);
            e.mem.set_u32(list + 4 * index as u32, island);
        }
        e.mem.set_u32(world + 0x28, list);
        e.mem.set_u32(world + 0x2c, islands.len() as u32);
        let holder = e.mem.alloc(0x80);
        e.mem.set_u32(world + 0x78, holder);
        (world, simulation, holder)
    }

    #[test]
    fn init_copies_the_time_into_the_simulation() {
        let mut e = engine();
        let this = manager(&mut e);
        let log = new_log();
        let (world, simulation, _) = world(&mut e, &log, 0.0, &[]);
        e.call(0x00d0_b230, &args![this, world]);
        assert_eq!(e.mem.f32(simulation + 0x14), 10.0);
        assert_eq!(e.mem.f32(simulation + 0x18), 10.0);
    }

    #[test]
    fn constructor_takes_its_defaults_from_the_data() {
        let mut e = engine();
        e.mem.set_f32(DEFAULT_TIME, 0.25);
        e.mem.set_f32(DEFAULT_INTERVAL, 0.5);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d0_b250, &args![this]).u32(), this);
        assert_eq!(e.mem.f32(this + 8), 0.25);
        assert_eq!(e.mem.f32(this + 0xc), 0.5);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_d34c);
    }

    #[test]
    fn reset_world_time_moves_every_clock_by_the_difference() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let this = manager(&mut e);
        let log = new_log();
        let (world, simulation, holder) = world(&mut e, &log, 3.0, &[&[1.0, 2.0], &[]]);
        let first_island = e.mem.u32(e.mem.u32(world + 0x28));
        let second_island = e.mem.u32(e.mem.u32(world + 0x28) + 4);
        log_double(&mut e, &log, ISLAND_ROUTINE, 0);
        let step = step_info(&mut e, 2.0, 2.5);
        e.call(0x00d0_b3e0, &args![this, world, step]);
        let expected = [10.0f32, 10.5, 0.5, 2.0];
        for (word, value) in expected.into_iter().enumerate() {
            assert_eq!(e.mem.f32(world + 0x1d0 + 4 * word as u32), value);
            assert_eq!(e.mem.f32(step + 4 * word as u32), value);
            assert_eq!(e.mem.f32(holder + 0x50 + 4 * word as u32), value);
        }
        // The time moved is 10 - 2 = 8.
        assert_eq!(e.mem.f32(simulation + 0x14), 9.0);
        assert_eq!(e.mem.f32(simulation + 0x18), 10.0);
        assert_eq!(e.mem.f32(simulation + 0x20), 11.0);
        let entities = e.mem.u32(first_island + 0x48);
        assert_eq!(e.mem.f32(e.mem.u32(entities) + 0x13c), 9.0);
        assert_eq!(e.mem.f32(e.mem.u32(entities + 4) + 0x13c), 10.0);
        // The step info is rewritten before the routine runs; its end time arrives as 10.5.
        assert_eq!(
            calls(&log, ISLAND_ROUTINE),
            vec![
                vec![first_island, 2.5f32.to_bits(), 10.5f32.to_bits(), holder],
                vec![second_island, 2.5f32.to_bits(), 10.5f32.to_bits(), holder],
            ]
        );
        assert_eq!(
            calls(&log, 0x0900_0038),
            vec![vec![simulation, 8.0f32.to_bits()]]
        );
    }

    #[test]
    fn reset_world_time_leaves_a_minus_one_late_time_alone() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let this = manager(&mut e);
        let log = new_log();
        let (world, simulation, _) = world(&mut e, &log, -1.0, &[]);
        let step = step_info(&mut e, 2.0, 2.5);
        e.call(0x00d0_b3e0, &args![this, world, step]);
        assert_eq!(e.mem.f32(simulation + 0x20), -1.0);
        assert_eq!(calls(&log, 0x0900_0038).len(), 1);
    }

    #[test]
    fn perform_maintenance_splits_then_resets_when_the_interval_is_reached() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let (stream, records) = monitor(&mut e, 0x80);
        let this = manager(&mut e);
        let log = new_log();
        let (world, _, _) = world(&mut e, &log, 3.0, &[]);
        log_double(&mut e, &log, SPLIT_SIMULATION_ISLANDS, 0);
        let step = step_info(&mut e, 2.0, 2.5);
        e.call(0x00d0_b280, &args![this, world, step]);
        assert_eq!(calls(&log, SPLIT_SIMULATION_ISLANDS), vec![vec![world]]);
        assert_eq!(calls(&log, 0x0900_0038).len(), 1);
        assert_eq!(e.mem.u32(records), LABEL_FIRST);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_MAINTENANCE);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_NO_SPLIT);
        assert_eq!(e.mem.u32(records + 0x1c), LABEL_END);
        assert_eq!(e.mem.u32(stream + 4), records + 0x28);
        // A step that has not reached the interval only splits.
        let early = step_info(&mut e, 0.5, 0.75);
        e.mem.set_u32(stream + 4, records);
        e.call(0x00d0_b280, &args![this, world, early]);
        assert_eq!(calls(&log, 0x0900_0038).len(), 1);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_END);
        assert_eq!(e.mem.u32(stream + 4), records + 0x1c);
    }

    #[test]
    fn perform_maintenance_no_split_only_resets() {
        let mut e = engine();
        e.set_global(MINUS_ONE, -1.0f32);
        let (stream, records) = monitor(&mut e, 0x80);
        let this = manager(&mut e);
        let log = new_log();
        let (world, _, _) = world(&mut e, &log, 3.0, &[]);
        log_double(&mut e, &log, SPLIT_SIMULATION_ISLANDS, 0);
        let step = step_info(&mut e, 2.0, 2.5);
        e.call(0x00d0_b350, &args![this, world, step]);
        assert!(calls(&log, SPLIT_SIMULATION_ISLANDS).is_empty());
        assert_eq!(calls(&log, 0x0900_0038).len(), 1);
        assert_eq!(e.mem.u32(records), LABEL_FIRST);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_NO_SPLIT);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_END);
        assert_eq!(e.mem.u32(stream + 4), records + 0x1c);
        let early = step_info(&mut e, 0.5, 0.75);
        e.call(0x00d0_b350, &args![this, world, early]);
        assert_eq!(calls(&log, 0x0900_0038).len(), 1);
    }
}
