//! `Havok SDK/hkpvehiclemanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpVehicleManager` (vtable `010e5b4c`; the `hkBaseObject` base is `0102e388`) keeps the
//! registered vehicles as an `hkArray<hkpVehicleInstance*>` at `+8` (data `+8`, size `+0xC`,
//! capacity and flags `+0x10`; bit 31 of the flags means the memory is not owned) and holds a
//! reference to each. `stepVehicles` uses a scratch array of 0x60-byte wheel records that
//! starts in 0x600 bytes taken from the top of the memory router's stack (the first word of
//! the router, advanced for the duration of the call).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleManager` (Xbox PDB layout of the registered vehicle array).
    pub struct HkpVehicleManager: 0x14 {
        /// The registered vehicles' data pointer.
        0x08 m_registered_vehicles: Ptr,
        /// The array's size.
        0x0c m_registered_vehicles_size: i32,
        /// The array's capacity and flags.
        0x10 m_registered_vehicles_capacity: u32,
    }
}

/// The memory router (`00c85750`, no arguments): its first word is the top of its stack
/// allocator and its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject` reference counting: add (`00c90510`) and remove (`00c905b0`), `this` =
/// the object (the map gives them no name).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, capacity, element size)`) and
/// the `hkarrayutil.cpp` function that makes room for one more element (`00c90940`, cdecl
/// `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpVehicleInstance::updateBeforeCollisionDetection` (Xbox PDB, `this` = the vehicle) and
/// `hkpVehicleInstance::stepVehicleUsingWheelCollideOutput` (Xbox PDB, `this` = the vehicle,
/// then the step info and the wheel records).
const VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION: u32 = 0x00e3_7940;
const VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT: u32 = 0x00e3_9030;
/// The vtables the destructor writes: the manager's and the base object's.
const MANAGER_VTABLE: u32 = 0x010e_5b4c;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The bytes `stepVehicles` takes from the router's stack, the capacity (in records) of the
/// scratch array that starts there, its flags (not owned, capacity 0x10) and the record size.
const SCRATCH_BYTES: u32 = 0x600;
const SCRATCH_FLAGS: u32 = 0x8000_0010;
const WHEEL_RECORD_SIZE: u32 = 0x60;

// Translated from 00e426b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleManager::removeVehicle` (Xbox PDB): looks the vehicle up in the array (the size
/// is signed); the size drops by one in any case, and the last element moves into the vehicle's
/// slot unless that slot is the new size. A vehicle that is not found has index -1, so the last
/// element is written to the word before the array's data (as the exe does). Then one
/// reference is removed from the vehicle (`00c905b0`).
pub fn remove_vehicle(e: &mut Engine, this: Ptr<HkpVehicleManager>, vehicle: Ptr) {
    let size = e.get(this, HkpVehicleManager::m_registered_vehicles_size);
    let data = e.get(this, HkpVehicleManager::m_registered_vehicles).addr();
    let mut index = 0i32;
    let mut found = false;
    if size > 0 {
        while index < size {
            if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == vehicle.addr() {
                found = true;
                break;
            }
            index += 1;
        }
    }
    if !found {
        index = -1;
    }
    let new_size = size.wrapping_sub(1);
    e.set(
        this,
        HkpVehicleManager::m_registered_vehicles_size,
        new_size,
    );
    if new_size != index {
        let data = e.get(this, HkpVehicleManager::m_registered_vehicles).addr();
        let last = e
            .mem
            .u32(data.wrapping_add((new_size as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), last);
    }
    e.call(REMOVE_REFERENCE, &args![vehicle]);
}

// Translated from 00e426f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleManager::addVehicle` (Xbox PDB): adds a reference to the vehicle (`00c90510`),
/// then appends it to the array. The memory router is asked first; when the size equals the
/// capacity (`+0x10` masked with `3FFFFFFF`) the array is grown through `00c90940` with the
/// router's allocator.
pub fn add_vehicle(e: &mut Engine, this: Ptr<HkpVehicleManager>, vehicle: Ptr) {
    e.call(ADD_REFERENCE, &args![vehicle]);
    let array = this.addr() + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), vehicle.addr());
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00e42740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's destructor (the map gives `00e42740` no name; that it is
/// `hkpVehicleManager::~hkpVehicleManager` is unconfirmed): sets the manager's vtable,
/// removes a reference from every registered vehicle (`00c905b0`), empties the array and, when
/// it owns its memory (bit 31 of the capacity word clear), frees it through the router's
/// allocator (slot `0x10`, with the data pointer and the capacity times 4). The array is then
/// reset (data 0, capacity word `80000000`) and the base object's vtable is set.
pub fn fn_00e42740(e: &mut Engine, this: Ptr<HkpVehicleManager>) {
    let count = e.get(this, HkpVehicleManager::m_registered_vehicles_size);
    e.mem.set_u32(this.addr(), MANAGER_VTABLE);
    let mut i = 0i32;
    while i < count {
        let data = e.get(this, HkpVehicleManager::m_registered_vehicles).addr();
        let vehicle = e.mem.u32(data.wrapping_add((i as u32).wrapping_mul(4)));
        e.call(REMOVE_REFERENCE, &args![vehicle]);
        i += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.get(this, HkpVehicleManager::m_registered_vehicles_capacity);
    e.set(this, HkpVehicleManager::m_registered_vehicles_size, 0);
    if (capacity as i32) >= 0 {
        let data = e.get(this, HkpVehicleManager::m_registered_vehicles);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.set(this, HkpVehicleManager::m_registered_vehicles, Ptr::new(0));
    e.set(
        this,
        HkpVehicleManager::m_registered_vehicles_capacity,
        0x8000_0000,
    );
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
}

// Translated from 00e427b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleManager::stepVehicles` (Xbox PDB), `(this, step info)`: takes 0x600 bytes from
/// the router's stack and makes them the storage of a local array of 0x60-byte wheel records
/// (capacity 0x10, not owned). For every registered vehicle (the count is read once, the
/// array each time): the wheel count is the byte at `data + 0x20` of the vehicle's data
/// (`+0x1C`); the local array is made at least that big (`_reserve` with the router's
/// allocator, `max(2 * capacity, wheels)` records, when its capacity is smaller) and its size
/// set to it; the vehicle's `updateBeforeCollisionDetection` (`00e37940`) runs, then its wheel
/// collider (`+0x3C`, slot `0x10`) is called with `(step info's time at +8, vehicle, records)`
/// and the vehicle is stepped with `00e39030` `(vehicle, step info, records)`. Afterwards the
/// router's stack is given back and, when the local array has moved to memory it owns (bit 31
/// clear), it is freed through the allocator (slot `0x10`, data and capacity times 0x60).
pub fn step_vehicles(e: &mut Engine, this: Ptr<HkpVehicleManager>, step_info: Ptr) {
    e.with_stack(0xc, |e, local| {
        step_vehicles_with(e, this, step_info, local.addr());
    });
}

/// [`step_vehicles`] with the local array (data, size, capacity and flags) at `array`.
fn step_vehicles_with(e: &mut Engine, this: Ptr<HkpVehicleManager>, step_info: Ptr, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(router, top.wrapping_add(SCRATCH_BYTES));
    let count = e.get(this, HkpVehicleManager::m_registered_vehicles_size);
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, SCRATCH_FLAGS);
    if count > 0 {
        let mut i = 0i32;
        loop {
            let list = e.get(this, HkpVehicleManager::m_registered_vehicles).addr();
            let vehicle = e.mem.u32(list.wrapping_add((i as u32).wrapping_mul(4)));
            let vehicle_data = e.mem.u32(vehicle + 0x1c);
            let wheels = i32::from(e.mem.u8(vehicle_data + 0x20));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
            if capacity < wheels {
                let doubled = capacity.wrapping_add(capacity);
                let wanted = if wheels < doubled { doubled } else { wheels };
                e.call(
                    ARRAY_RESERVE,
                    &args![allocator, array, wanted as u32, WHEEL_RECORD_SIZE],
                );
            }
            e.mem.set_i32(array + 4, wheels);
            e.call(VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, &args![vehicle]);
            let records = e.mem.u32(array);
            let collide = e.mem.u32(vehicle + 0x3c);
            let time = x87_quiet(e.mem.f32(step_info.addr() + 8));
            e.vcall(collide, 0x10, &args![time, vehicle, records]);
            let records = e.mem.u32(array);
            e.call(
                VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT,
                &args![vehicle, step_info, records],
            );
            i += 1;
            if i >= count {
                break;
            }
        }
        // Still in the router's stack space: the size is cleared (the store at the end makes
        // this one unobservable).
        if e.mem.u32(array) == top {
            e.mem.set_u32(array + 4, 0);
        }
    } else {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![
                data,
                (capacity & 0x3fff_ffff).wrapping_mul(WHEEL_RECORD_SIZE)
            ],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_26b0, remove_vehicle(Ptr<HkpVehicleManager>, Ptr)),
        entry!(0x00e4_26f0, add_vehicle(Ptr<HkpVehicleManager>, Ptr)),
        entry!(0x00e4_2740, fn_00e42740(Ptr<HkpVehicleManager>)),
        entry!(0x00e4_27b0, step_vehicles(Ptr<HkpVehicleManager>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    /// A manager over `vehicles`: an array with room for `capacity` entries (the flags word
    /// is `flags`). Returns `(manager, data)`.
    fn make_manager(e: &mut Engine, vehicles: &[u32], capacity: u32, flags: u32) -> (u32, u32) {
        let manager = e.mem.alloc(0x14);
        let data = e.mem.alloc(4 * capacity.max(vehicles.len() as u32 + 4));
        for (i, vehicle) in vehicles.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *vehicle);
        }
        e.mem.set_u32(manager + 8, data);
        e.mem.set_u32(manager + 0xc, vehicles.len() as u32);
        e.mem.set_u32(manager + 0x10, capacity | flags);
        (manager, data)
    }

    /// A router double: the first word `top`, the allocator `allocator` at `+0x10`.
    fn router(e: &mut Engine, top: u32, allocator: u32) -> u32 {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, top);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    #[test]
    fn removal_moves_the_last_vehicle_into_the_gap() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let (manager, data) = make_manager(&mut e, &[0x11, 0x22, 0x33], 4, 0);
        e.call(0x00e4_26b0, &args![manager, 0x11u32]);
        assert_eq!(e.mem.i32(manager + 0xc), 2);
        assert_eq!(e.mem.u32(data), 0x33);
        assert_eq!(e.mem.u32(data + 4), 0x22);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x11]]);
    }

    #[test]
    fn removal_of_the_last_vehicle_moves_nothing() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let (manager, data) = make_manager(&mut e, &[0x11, 0x22, 0x33], 4, 0);
        e.call(0x00e4_26b0, &args![manager, 0x33u32]);
        assert_eq!(e.mem.i32(manager + 0xc), 2);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [0x11, 0x22, 0x33]
        );
    }

    #[test]
    fn removal_of_an_unknown_vehicle_still_shrinks_the_array() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        // The data sits after one spare word: the exe writes the last element there.
        let (manager, data) = make_manager(&mut e, &[0x11, 0x22], 4, 0);
        let shifted = data + 4;
        e.mem.set_u32(shifted, 0x11);
        e.mem.set_u32(shifted + 4, 0x22);
        e.mem.set_u32(manager + 8, shifted);
        e.call(0x00e4_26b0, &args![manager, 0x99u32]);
        assert_eq!(e.mem.i32(manager + 0xc), 1);
        // Index -1 receives the element at the new size (index 1).
        assert_eq!(e.mem.u32(data), 0x22);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x99]]);
    }

    #[test]
    fn removal_from_an_empty_array_leaves_size_minus_one() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let (manager, _) = make_manager(&mut e, &[], 4, 0);
        e.call(0x00e4_26b0, &args![manager, 0x99u32]);
        // Index -1 equals the new size -1: nothing is moved.
        assert_eq!(e.mem.i32(manager + 0xc), -1);
    }

    #[test]
    fn adding_references_and_appends() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        log_double(&mut e, &log, ARRAY_RESERVE_MORE, 0);
        router(&mut e, 0, 0x7777_0000);
        let (manager, data) = make_manager(&mut e, &[0x11], 4, 0x8000_0000);
        e.call(0x00e4_26f0, &args![manager, 0x22u32]);
        assert_eq!(e.mem.u32(data + 4), 0x22);
        assert_eq!(e.mem.i32(manager + 0xc), 2);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x22]]);
        assert!(calls(&log, ARRAY_RESERVE_MORE).is_empty());
        // A full array (the size equals the capacity without the flag bits) is grown first.
        let (full, _) = make_manager(&mut e, &[0x11, 0x12], 2, 0x8000_0000);
        e.call(0x00e4_26f0, &args![full, 0x33u32]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE_MORE),
            vec![vec![0x7777_0000, full + 8, 4]]
        );
    }

    #[test]
    fn destruction_releases_every_vehicle_and_frees_owned_memory() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        log_double(&mut e, &log, 0x0900_0010, 0);
        router(&mut e, 0, allocator);
        let (manager, data) = make_manager(&mut e, &[0x11, 0x22], 6, 0x4000_0000);
        e.call(0x00e4_2740, &args![manager]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x11], vec![0x22]]);
        // The capacity is masked with 3FFFFFFF: 6 entries of 4 bytes.
        assert_eq!(calls(&log, 0x0900_0010), vec![vec![allocator, data, 24]]);
        assert_eq!(e.mem.u32(manager), 0x0102_e388);
        assert_eq!(e.mem.u32(manager + 8), 0);
        assert_eq!(e.mem.i32(manager + 0xc), 0);
        assert_eq!(e.mem.u32(manager + 0x10), 0x8000_0000);
    }

    #[test]
    fn destruction_keeps_memory_it_does_not_own() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        router(&mut e, 0, 0x7777_0000);
        let (manager, _) = make_manager(&mut e, &[0x11], 4, 0x8000_0000);
        e.call(0x00e4_2740, &args![manager]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x11]]);
        assert_eq!(e.mem.u32(manager + 8), 0);
    }

    /// A vehicle with `wheels` wheels and a wheel collider that logs its calls.
    fn vehicle(e: &mut Engine, wheels: u8, log: &Log) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, wheels);
        let collide_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(collide_vtable + 0x10, 0x0900_0010);
        let collide = e.mem.alloc(0x10);
        e.mem.set_u32(collide, collide_vtable);
        log_double(e, log, 0x0900_0010, 0);
        let vehicle = e.mem.alloc(0x60);
        e.mem.set_u32(vehicle + 0x1c, data);
        e.mem.set_u32(vehicle + 0x3c, collide);
        vehicle
    }

    #[test]
    fn stepping_runs_every_vehicle_with_the_scratch_records() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, 0);
        log_double(&mut e, &log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT, 0);
        log_double(&mut e, &log, ARRAY_RESERVE, 0);
        let allocator = 0x7777_0000;
        let router_address = router(&mut e, 0x5000, allocator);
        let first = vehicle(&mut e, 4, &log);
        let second = vehicle(&mut e, 8, &log);
        let (manager, _) = make_manager(&mut e, &[first, second], 2, 0);
        let step = e.mem.alloc(0x20);
        e.mem.set_f32(step + 8, 0.25);
        e.call(0x00e4_27b0, &args![manager, step]);
        assert_eq!(
            calls(&log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION),
            vec![vec![first], vec![second]]
        );
        // The records are at the router's old top; the wheel collider gets the time first.
        let collide: Vec<Vec<u32>> = calls(&log, 0x0900_0010);
        assert_eq!(collide.len(), 2);
        assert_eq!(collide[0][1..], [0.25f32.to_bits(), first, 0x5000]);
        assert_eq!(collide[1][1..], [0.25f32.to_bits(), second, 0x5000]);
        assert_eq!(
            calls(&log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT),
            vec![vec![first, step, 0x5000], vec![second, step, 0x5000]]
        );
        // 16 records fit: no reserve. The router's top is given back.
        assert!(calls(&log, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.u32(router_address), 0x5000);
    }

    #[test]
    fn stepping_takes_the_scratch_space_from_the_router() {
        let mut e = engine();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = new_log();
        let router_address = router(&mut e, 0x5000, 0x7777_0000);
        let sink = std::rc::Rc::clone(&seen);
        e.register_double(VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, move |e, _| {
            sink.borrow_mut().push(e.mem.u32(router_address));
            Ret::default()
        });
        log_double(&mut e, &log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT, 0);
        let one = vehicle(&mut e, 1, &log);
        let (manager, _) = make_manager(&mut e, &[one], 1, 0);
        let step = e.mem.alloc(0x20);
        e.call(0x00e4_27b0, &args![manager, step]);
        assert_eq!(*seen.borrow(), vec![0x5000 + 0x600]);
        assert_eq!(e.mem.u32(router_address), 0x5000);
    }

    #[test]
    fn stepping_a_big_vehicle_reserves_and_frees_the_array() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, VEHICLE_UPDATE_BEFORE_COLLISION_DETECTION, 0);
        log_double(&mut e, &log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT, 0);
        // The reserve double moves the array to owned memory with room for 32 records.
        let block = e.mem.alloc(0x100);
        let reserved = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = std::rc::Rc::clone(&reserved);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let free_log = new_log();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0210);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        log_double(&mut e, &free_log, 0x0900_0210, 0);
        router(&mut e, 0x5000, allocator);
        let big = vehicle(&mut e, 20, &log);
        let (manager, _) = make_manager(&mut e, &[big], 1, 0);
        let step = e.mem.alloc(0x20);
        e.call(0x00e4_27b0, &args![manager, step]);
        // 20 > 16: the new capacity is max(2 * 16, 20) = 32.
        let reserved = reserved.borrow();
        assert_eq!(reserved.len(), 1);
        assert_eq!(
            [reserved[0][0], reserved[0][2], reserved[0][3]],
            [allocator, 32, 0x60]
        );
        let step_calls = calls(&log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT);
        assert_eq!(step_calls, vec![vec![big, step, block]]);
        // The owned memory is freed with the capacity times 0x60.
        assert_eq!(
            calls(&free_log, 0x0900_0210),
            vec![vec![allocator, block, 32 * 0x60]]
        );
    }

    #[test]
    fn stepping_without_vehicles_only_gives_back_the_scratch_space() {
        let mut e = engine();
        let router_address = router(&mut e, 0x5000, 0x7777_0000);
        let (manager, _) = make_manager(&mut e, &[], 1, 0);
        let step = e.mem.alloc(0x20);
        e.call(0x00e4_27b0, &args![manager, step]);
        assert_eq!(e.mem.u32(router_address), 0x5000);
    }
}
