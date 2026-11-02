//! `Havok SDK/hkpvehiclecastbatchingmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpVehicleCastBatchingManager` is an `hkpVehicleManager` (`hkpvehiclemanager.rs`) that
//! also keeps the total number of wheels of its vehicles as a 16-bit count at `+0x14`. Its
//! virtual slots used here are `0x1C` (update before collision detection), `0x20` (the size of
//! the scratch memory the cast needs, given the fourth word), `0x24` (the batched cast) and
//! `0x28` (the work after it). The profiling records use the host clock for `RDTSC` (see
//! `run_00cf5070`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

layout! {
    /// `hkpVehicleCastBatchingManager` (Xbox PDB name; the count's name is unconfirmed).
    pub struct HkpVehicleCastBatchingManager: 0x18 {
        /// The registered vehicles' data pointer (the `hkpVehicleManager` base).
        0x08 m_registered_vehicles: Ptr,
        /// The array's size.
        0x0c m_registered_vehicles_size: i32,
        /// The total number of wheels of the registered vehicles.
        0x14 m_total_num_wheels: u16,
    }
}

/// `hkpVehicleManager::addVehicle` and `removeVehicle` (Xbox PDB, `this` = the manager, then
/// the vehicle).
const MANAGER_ADD_VEHICLE: u32 = 0x00e4_26f0;
const MANAGER_REMOVE_VEHICLE: u32 = 0x00e4_26b0;
/// `hkpVehicleInstance::updateBeforeCollisionDetection` (Xbox PDB, `this` = the vehicle).
const VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION: u32 = 0x00e3_7940;
/// The memory router (`00c85750`, no arguments): its first word is the top of its stack
/// allocator and its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::alignedAlloc` and `alignedFree` (Xbox PDB, cdecl `(allocator, 4, 0x80)` and
/// `(allocator, block)` here).
const ALIGNED_ALLOC: u32 = 0x00c8_5620;
const ALIGNED_FREE: u32 = 0x00c8_5660;
/// `hkSemaphore::hkSemaphore` (Xbox PDB, `this`, then 0 and 1000), `hkSemaphore::acquire` and
/// `hkSemaphore::~hkSemaphore` (Xbox PDB, `this` only).
const SEMAPHORE_CONSTRUCTOR: u32 = 0x00d0_4350;
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
const SEMAPHORE_DESTRUCTOR: u32 = 0x00d0_4370;
/// `hkJobQueue::processAllJobs` (Xbox PDB, `this` = the queue).
const PROCESS_ALL_JOBS: u32 = 0x00d0_51b0;
/// The world's two read-only locks: `00c911a0` (in `hkpworld.cpp`, the map gives no name) and
/// `hkpWorld::unlockReadOnly` (Xbox PDB, `00c911b0`), `this` = the world.
const WORLD_LOCK_READ_ONLY: u32 = 0x00c9_11a0;
const WORLD_UNLOCK_READ_ONLY: u32 = 0x00c9_11b0;
/// Profiling labels: the start label of `stepVehiclesSynchronously` (`010ebe9c`) and the shared
/// end label.
const LABEL_STEP: u32 = 0x010e_be9c;
const LABEL_END: u32 = 0x010c_41f8;

// Translated from 00e428d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleCastBatchingManager::addVehicle` (Xbox PDB): the base `addVehicle`
/// (`00e426f0`), then the vehicle's wheel count (the byte at `+0x20` of its data at `+0x1C`)
/// is added to the manager's 16-bit count (wrapping at 16 bits).
pub fn add_vehicle(e: &mut Engine, this: Ptr<HkpVehicleCastBatchingManager>, vehicle: Ptr) {
    e.call(MANAGER_ADD_VEHICLE, &args![this, vehicle]);
    let data = e.mem.u32(vehicle.addr() + 0x1c);
    let wheels = u16::from(e.mem.u8(data + 0x20));
    let total = e.get(this, HkpVehicleCastBatchingManager::m_total_num_wheels);
    e.set(
        this,
        HkpVehicleCastBatchingManager::m_total_num_wheels,
        total.wrapping_add(wheels),
    );
}

// Translated from 00e428f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleCastBatchingManager::removeVehicle` (Xbox PDB): the base `removeVehicle`
/// (`00e426b0`, which may free the vehicle: the exe reads it afterwards all the same), then the
/// vehicle's wheel count is taken off the manager's 16-bit count (wrapping at 16 bits).
pub fn remove_vehicle(e: &mut Engine, this: Ptr<HkpVehicleCastBatchingManager>, vehicle: Ptr) {
    e.call(MANAGER_REMOVE_VEHICLE, &args![this, vehicle]);
    let data = e.mem.u32(vehicle.addr() + 0x1c);
    let wheels = u16::from(e.mem.u8(data + 0x20));
    let total = e.get(this, HkpVehicleCastBatchingManager::m_total_num_wheels);
    e.set(
        this,
        HkpVehicleCastBatchingManager::m_total_num_wheels,
        total.wrapping_sub(wheels),
    );
}

// Translated from 00e42910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleCastBatchingManager::updateBeforeCollisionDetection` (Xbox PDB): runs
/// `hkpVehicleInstance::updateBeforeCollisionDetection` (`00e37940`) on every registered
/// vehicle (the count is signed and read once).
pub fn update_before_collision_detection(e: &mut Engine, this: Ptr<HkpVehicleCastBatchingManager>) {
    let count = e.get(
        this,
        HkpVehicleCastBatchingManager::m_registered_vehicles_size,
    );
    let mut i = 0i32;
    while i < count {
        let data = e
            .get(this, HkpVehicleCastBatchingManager::m_registered_vehicles)
            .addr();
        let vehicle = e.mem.u32(data.wrapping_add((i as u32).wrapping_mul(4)));
        e.call(VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, &args![vehicle]);
        i += 1;
    }
}

// Translated from 00e42940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A small `hkpvehiclecastbatchingmanager.cpp` helper (the map gives it no name), cdecl with
/// one word, which `stepVehiclesSynchronously` passes the world: answers `0x100` when the word
/// at `+0x20` of the object at `+0x7C` of it is 2, else 0.
pub fn fn_00e42940(e: &mut Engine, world: Ptr) -> u32 {
    let simulation = e.mem.u32(world.addr() + 0x7c);
    if e.mem.u32(simulation + 0x20) == 2 {
        0x100
    } else {
        0
    }
}

// Translated from 00e42960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleCastBatchingManager::stepVehiclesSynchronously` (Xbox PDB), `(this, world,
/// thread pool, job queue, fourth word)` (the roles are read off the callees: `hkpWorld`'s
/// read-only unlock, `hkJobQueue::processAllJobs`; the thread pool's slots `0xC` and `0x10`):
/// writes a profile record, then
/// 1. runs the manager's slot `0x1C` (update before collision detection);
/// 2. asks slot `0x20` with the fourth word for the scratch size, rounds it up to 128 and takes
///    that many bytes from the router's stack (the first word of the router; its old value is
///    the scratch memory's address);
/// 3. allocates 0x80 bytes through the router's allocator (`alignedAlloc` with 4 and 0x80) and
///    constructs a semaphore (0, 1000) in them when the allocation worked;
/// 4. asks `00e42940` about the world and calls slot `0x24` with `(world, that answer, fourth
///    word, job queue, semaphore, scratch)`, then locks the world (`00c911a0`);
/// 5. has the thread pool process `(job queue, 0xB)` (slot `0xC`), runs `processAllJobs` on
///    the queue, calls the thread pool's slot `0x10`, acquires the semaphore, unlocks the world
///    (`00c911b0`), destroys the semaphore and frees it (`alignedFree`); the semaphore calls are
///    made even when the allocation failed (a null `this`, as in the exe);
/// 6. calls slot `0x28` with `(world + 0x1D0, fourth word, scratch)`, gives the router's stack
///    back and writes the end record.
pub fn step_vehicles_synchronously(
    e: &mut Engine,
    this: Ptr<HkpVehicleCastBatchingManager>,
    world: Ptr,
    thread_pool: Ptr,
    job_queue: Ptr,
    fourth: u32,
) {
    profile_record(e, LABEL_STEP);
    e.vcall(this.addr(), 0x1c, &args![]);
    let needed = e.vcall(this.addr(), 0x20, &args![fourth]).u32();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let scratch = e.mem.u32(router);
    let taken = needed.wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, taken.wrapping_add(scratch));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let semaphore = e
        .call(ALIGNED_ALLOC, &args![allocator, 4u32, 0x80u32])
        .u32();
    if semaphore != 0 {
        e.call(SEMAPHORE_CONSTRUCTOR, &args![semaphore, 0u32, 0x3e8u32]);
    }
    let flag = e.call(0x00e4_2940, &args![world]).u32();
    e.vcall(
        this.addr(),
        0x24,
        &args![world, flag, fourth, job_queue, semaphore, scratch],
    );
    e.call(WORLD_LOCK_READ_ONLY, &args![world]);
    e.vcall(thread_pool.addr(), 0xc, &args![job_queue, 0xbu32]);
    e.call(PROCESS_ALL_JOBS, &args![job_queue]);
    e.vcall(thread_pool.addr(), 0x10, &args![]);
    e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
    e.call(WORLD_UNLOCK_READ_ONLY, &args![world]);
    e.call(SEMAPHORE_DESTRUCTOR, &args![semaphore]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(ALIGNED_FREE, &args![allocator, semaphore]);
    e.vcall(
        this.addr(),
        0x28,
        &args![world.addr() + 0x1d0, fourth, scratch],
    );
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, scratch);
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_28d0,
            add_vehicle(Ptr<HkpVehicleCastBatchingManager>, Ptr)
        ),
        entry!(
            0x00e4_28f0,
            remove_vehicle(Ptr<HkpVehicleCastBatchingManager>, Ptr)
        ),
        entry!(
            0x00e4_2910,
            update_before_collision_detection(Ptr<HkpVehicleCastBatchingManager>)
        ),
        entry!(0x00e4_2940, fn_00e42940(Ptr) -> u32),
        entry!(
            0x00e4_2960,
            step_vehicles_synchronously(Ptr<HkpVehicleCastBatchingManager>, Ptr, Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, profile_labels, Log,
    };

    /// A vehicle with `wheels` wheels (the byte at `+0x20` of its data at `+0x1C`).
    fn vehicle(e: &mut Engine, wheels: u8) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, wheels);
        let vehicle = e.mem.alloc(0x40);
        e.mem.set_u32(vehicle + 0x1c, data);
        vehicle
    }

    #[test]
    fn adding_a_vehicle_adds_its_wheels() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, MANAGER_ADD_VEHICLE, 0);
        let manager = e.mem.alloc(0x18);
        e.mem.set_u16(manager + 0x14, 0xfffe);
        let car = vehicle(&mut e, 4);
        e.call(0x00e4_28d0, &args![manager, car]);
        assert_eq!(calls(&log, MANAGER_ADD_VEHICLE), vec![vec![manager, car]]);
        // 0xfffe + 4 wraps in 16 bits.
        assert_eq!(e.mem.u16(manager + 0x14), 2);
        // The neighbouring bytes are not touched.
        assert_eq!(e.mem.u16(manager + 0x16), 0);
    }

    #[test]
    fn removing_a_vehicle_takes_its_wheels_off() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, MANAGER_REMOVE_VEHICLE, 0);
        let manager = e.mem.alloc(0x18);
        e.mem.set_u16(manager + 0x14, 6);
        let car = vehicle(&mut e, 4);
        e.call(0x00e4_28f0, &args![manager, car]);
        assert_eq!(
            calls(&log, MANAGER_REMOVE_VEHICLE),
            vec![vec![manager, car]]
        );
        assert_eq!(e.mem.u16(manager + 0x14), 2);
        // 2 - 4 wraps in 16 bits.
        e.call(0x00e4_28f0, &args![manager, car]);
        assert_eq!(e.mem.u16(manager + 0x14), 0xfffe);
    }

    #[test]
    fn updating_runs_every_vehicle() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, 0);
        let manager = e.mem.alloc(0x18);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x11);
        e.mem.set_u32(list + 4, 0x22);
        e.mem.set_u32(manager + 8, list);
        e.mem.set_u32(manager + 0xc, 2);
        e.call(0x00e4_2910, &args![manager]);
        assert_eq!(
            calls(&log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION),
            vec![vec![0x11], vec![0x22]]
        );
        // A negative count (signed compare) runs nothing.
        e.mem.set_i32(manager + 0xc, -1);
        e.call(0x00e4_2910, &args![manager]);
        assert_eq!(
            calls(&log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION).len(),
            2
        );
    }

    #[test]
    fn the_simulation_check_answers_0x100_for_type_2() {
        let mut e = engine();
        let simulation = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x7c, simulation);
        for (kind, expected) in [(2u32, 0x100u32), (0, 0), (1, 0), (3, 0)] {
            e.mem.set_u32(simulation + 0x20, kind);
            assert_eq!(e.call(0x00e4_2940, &args![world]).u32(), expected);
        }
    }

    /// Everything `stepVehiclesSynchronously` talks to, as logging doubles at the addresses
    /// `0x0900_0000 + slot`.
    struct Rig {
        e: Engine,
        log: Log,
        manager: u32,
        world: u32,
        thread_pool: u32,
        job_queue: u32,
        router: u32,
        allocator: u32,
        stream: u32,
        records: u32,
    }

    fn rig(semaphore: u32, needed: u32) -> Rig {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        // The manager and the thread pool: vtables of logging doubles.
        let manager_vtable = e.mem.alloc(0x40);
        for slot in [0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(manager_vtable + slot, 0x0900_0000 + slot);
            log_double(&mut e, &log, 0x0900_0000 + slot, 0);
        }
        let sink = std::rc::Rc::clone(&log);
        e.register_double(0x0900_0020, move |_, a| {
            sink.borrow_mut().push((0x0900_0020, a.to_vec()));
            Ret {
                eax: needed,
                ..Ret::default()
            }
        });
        let manager = e.mem.alloc(0x20);
        e.mem.set_u32(manager, manager_vtable);
        let pool_vtable = e.mem.alloc(0x40);
        for slot in [0xc, 0x10] {
            e.mem.set_u32(pool_vtable + slot, 0x0900_0100 + slot);
            log_double(&mut e, &log, 0x0900_0100 + slot, 0);
        }
        let thread_pool = e.mem.alloc(0x10);
        e.mem.set_u32(thread_pool, pool_vtable);
        let world = e.mem.alloc(0x200);
        let simulation = e.mem.alloc(0x40);
        e.mem.set_u32(simulation + 0x20, 2);
        e.mem.set_u32(world + 0x7c, simulation);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, 0x5000);
        let allocator = 0x7777_0000;
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            ALIGNED_FREE,
            SEMAPHORE_CONSTRUCTOR,
            SEMAPHORE_ACQUIRE,
            SEMAPHORE_DESTRUCTOR,
            PROCESS_ALL_JOBS,
            WORLD_LOCK_READ_ONLY,
            WORLD_UNLOCK_READ_ONLY,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        log_double(&mut e, &log, ALIGNED_ALLOC, semaphore);
        Rig {
            e,
            log,
            manager,
            world,
            thread_pool,
            job_queue: 0x4444,
            router,
            allocator,
            stream,
            records,
        }
    }

    #[test]
    fn stepping_synchronously_runs_the_whole_sequence() {
        let mut rig = rig(0x6000, 0x81);
        rig.e.call(
            0x00e4_2960,
            &args![
                rig.manager,
                rig.world,
                rig.thread_pool,
                rig.job_queue,
                0x99u32
            ],
        );
        let log = &rig.log;
        // The scratch memory starts at the router's old top (0x5000); slot 0x20 asked for 0x81
        // bytes, rounded to 0x100 while it ran.
        assert_eq!(calls(log, 0x0900_001c), vec![vec![rig.manager]]);
        assert_eq!(calls(log, 0x0900_0020), vec![vec![rig.manager, 0x99]]);
        assert_eq!(
            calls(log, ALIGNED_ALLOC),
            vec![vec![rig.allocator, 4, 0x80]]
        );
        assert_eq!(
            calls(log, SEMAPHORE_CONSTRUCTOR),
            vec![vec![0x6000, 0, 0x3e8]]
        );
        // The world's simulation type is 2: the flag is 0x100.
        assert_eq!(
            calls(log, 0x0900_0024),
            vec![vec![
                rig.manager,
                rig.world,
                0x100,
                0x99,
                rig.job_queue,
                0x6000,
                0x5000
            ]]
        );
        assert_eq!(calls(log, WORLD_LOCK_READ_ONLY), vec![vec![rig.world]]);
        assert_eq!(
            calls(log, 0x0900_010c),
            vec![vec![rig.thread_pool, rig.job_queue, 0xb]]
        );
        assert_eq!(calls(log, PROCESS_ALL_JOBS), vec![vec![rig.job_queue]]);
        assert_eq!(calls(log, 0x0900_0110), vec![vec![rig.thread_pool]]);
        assert_eq!(calls(log, SEMAPHORE_ACQUIRE), vec![vec![0x6000]]);
        assert_eq!(calls(log, WORLD_UNLOCK_READ_ONLY), vec![vec![rig.world]]);
        assert_eq!(calls(log, SEMAPHORE_DESTRUCTOR), vec![vec![0x6000]]);
        assert_eq!(calls(log, ALIGNED_FREE), vec![vec![rig.allocator, 0x6000]]);
        assert_eq!(
            calls(log, 0x0900_0028),
            vec![vec![rig.manager, rig.world + 0x1d0, 0x99, 0x5000]]
        );
        // The router's top is back where it was and the two profile records were written.
        assert_eq!(rig.e.mem.u32(rig.router), 0x5000);
        assert_eq!(
            profile_labels(&rig.e, rig.stream, rig.records),
            vec![LABEL_STEP, LABEL_END]
        );
    }

    #[test]
    fn stepping_synchronously_takes_the_rounded_scratch_size_while_it_runs() {
        let mut rig = rig(0x6000, 0x81);
        let router = rig.router;
        let seen = std::rc::Rc::new(std::cell::RefCell::new(0));
        let sink = std::rc::Rc::clone(&seen);
        rig.e.register_double(0x0900_0024, move |e, _| {
            *sink.borrow_mut() = e.mem.u32(router);
            Ret::default()
        });
        rig.e.call(
            0x00e4_2960,
            &args![rig.manager, rig.world, rig.thread_pool, rig.job_queue, 0u32],
        );
        assert_eq!(*seen.borrow(), 0x5000 + 0x100);
    }

    #[test]
    fn stepping_synchronously_skips_the_constructor_for_a_failed_allocation() {
        let mut rig = rig(0, 0);
        rig.e.call(
            0x00e4_2960,
            &args![rig.manager, rig.world, rig.thread_pool, rig.job_queue, 0u32],
        );
        assert!(calls(&rig.log, SEMAPHORE_CONSTRUCTOR).is_empty());
        // The other semaphore calls are made with the null pointer.
        assert_eq!(calls(&rig.log, SEMAPHORE_ACQUIRE), vec![vec![0]]);
        assert_eq!(calls(&rig.log, SEMAPHORE_DESTRUCTOR), vec![vec![0]]);
        assert_eq!(calls(&rig.log, ALIGNED_FREE), vec![vec![rig.allocator, 0]]);
    }

    #[test]
    fn stepping_synchronously_passes_flag_zero_for_other_simulations() {
        let mut rig = rig(0x6000, 0);
        let simulation = rig.e.mem.u32(rig.world + 0x7c);
        rig.e.mem.set_u32(simulation + 0x20, 1);
        rig.e.call(
            0x00e4_2960,
            &args![rig.manager, rig.world, rig.thread_pool, rig.job_queue, 0u32],
        );
        assert_eq!(calls(&rig.log, 0x0900_0024)[0][2], 0);
    }
}
