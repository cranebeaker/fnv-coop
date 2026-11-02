//! `Havok SDK/hkpvehicleraycastbatchingmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};
use crate::units::havok_sdk::hkpvehicleinstance::reserve_array;

// hkpVehicleRayCastBatchingManager has no type in the Xbox PDB, so the layout is described by the
// offsets the PC code uses (roles unconfirmed): `+8` the pointer to the vehicles (`+0xc` their
// count), `+0x14` a 16-bit count `n` of cast commands the buffer is sized for.

/// Memory router (`00c85750`): its field `+0` is the stack allocator's top, `+0x10` the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00e42b00`, the buffer layout (also called by this unit's functions).
const BUFFER_LAYOUT: u32 = 0x00e4_2b00;
/// The empty function (`RET`) that prepares a job block, called with the block in `ECX`.
const JOB_PREPARE: u32 = 0x00e5_cee0;
/// `hkJobQueue::addJob` (engine map name `addJob_ov2`; `ECX` the queue, then the job and `1`).
const JOB_QUEUE_ADD_JOB: u32 = 0x00d0_4b90;
/// `stepVehicleUsingWheelCollideOutput` of `hkpVehicleInstance` (`ECX` the vehicle).
const STEP_VEHICLE_USING_OUTPUT: u32 = 0x00e3_9030;
/// Profile labels (`.rdata`) of `buildAndAddCastJobs` ("TtBuildAndAddRaycastJobs", then
/// "TtBuildCommands") and of `stepVehiclesUsingCastResults` ("TtDoVehicles").
const BUILD_AND_ADD_LABEL: u32 = 0x010e_bec8;
const BUILD_COMMANDS_LABEL: u32 = 0x010e_beb8;
const STEP_VEHICLES_LABEL: u32 = 0x010e_bee4;

fn align_16(value: u32) -> u32 {
    value.wrapping_add(0xf) & 0xffff_fff0
}

// Translated from 00e42ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastBatchingManager::getBufferSize` (Xbox PDB name, engine map). Thiscall, one
/// stack word (`RET 4`): `count`. The size in bytes of the work buffer: `0x60` bytes and `0x50`
/// bytes per entry of the 16-bit count at `+0x14` (each rounded up to 16), `16 * count` rounded
/// up to 16, and the vehicle count at `+0xc` rounded up to 16.
pub fn hkp_vehicle_ray_cast_batching_manager_get_buffer_size(
    e: &mut Engine,
    this: Ptr,
    count: u32,
) -> u32 {
    let entries = u32::from(e.mem.u16(this.addr() + 0x14));
    let vehicles = e.mem.u32(this.addr() + 0xc);
    let outputs = align_16(entries.wrapping_mul(3) << 5);
    let commands = align_16(entries.wrapping_mul(5) << 4);
    let jobs = align_16(count << 4);
    outputs
        .wrapping_add(commands)
        .wrapping_add(jobs)
        .wrapping_add(align_16(vehicles))
}

// Translated from 00e42b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lays the work buffer out. Thiscall, three stack words (`RET 0xc`): `base` (the buffer),
/// `_unused_2` and `layout` (four words written): `layout[0] = base`, `layout[1] = base +
/// 0x50 * n` (the collision-output area, `n` the 16-bit count at `+0x14`; each size rounded up
/// to 16), `layout[2] = layout[1] + 0x60 * n` (one byte per vehicle) and `layout[3] = layout[2] +
/// the vehicle count at +0xc` rounded up to 16 (the job records).
pub fn fn_00e42b00(e: &mut Engine, this: Ptr, base: u32, _unused_2: u32, layout: u32) {
    let entries = u32::from(e.mem.u16(this.addr() + 0x14));
    let vehicles = e.mem.u32(this.addr() + 0xc);
    let outputs = base.wrapping_add(align_16(entries.wrapping_mul(5) << 4));
    e.mem.set_u32(layout + 4, outputs);
    let counts = outputs.wrapping_add(align_16(entries.wrapping_mul(3) << 5));
    e.mem.set_u32(layout, base);
    e.mem
        .set_u32(layout + 0xc, align_16(vehicles).wrapping_add(counts));
    e.mem.set_u32(layout + 8, counts);
}

// Translated from 00e42b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastBatchingManager::buildAndAddCastJobs` (Xbox PDB name, engine map). Thiscall,
/// six stack words (`RET 0x18`): `info` (its words at `+0x78` and `+0x7c` are used), `argument`,
/// `max_jobs`, `job_queue`, `job_data` and `buffer`. Between profile records (labels `010ebec8`,
/// `010ebeb8` and `010c41f8`) it lays the buffer out (`00e42b00`), has every vehicle's
/// wheel-collide object (the word at vehicle `+0x3c`, slot `0x30`) build its cast commands
/// (called with the vehicle, `info[0x7c]`, `argument` and the next free command and output
/// areas), keeps each vehicle's command count as a byte, and then splits the total over
/// `min(max_jobs, total)` jobs of `total / jobs` commands (the first `total % jobs` get one
/// more): every job block (`0x2c` bytes: bytes `0`, `3`, `1`, words `0x30`, `0xffff`, `job_data`,
/// the job-record pointer, `0`, `info[0x78]`, `90`, the first command, the command count) is
/// prepared (`00e5cee0`), the job-record word becomes `(count - 1) / 90 + 1` and the job is
/// added to `job_queue` (`00d04b90`, flag `1`). Returns the number of jobs. When no command was
/// built the exe divides by zero (`IDIV`), which faults; the translation panics there.
#[allow(clippy::too_many_arguments)]
pub fn hkp_vehicle_ray_cast_batching_manager_build_and_add_cast_jobs(
    e: &mut Engine,
    this: Ptr,
    info: u32,
    argument: u32,
    max_jobs: i32,
    job_queue: u32,
    job_data: u32,
    buffer: u32,
) -> i32 {
    let t = this.addr();
    profile_record(e, BUILD_AND_ADD_LABEL);
    let info_word = e.mem.u32(info + 0x7c);
    let vehicle_count = e.mem.i32(t + 0xc);
    e.with_stack(0x54, |e, frame| {
        let f = frame.addr();
        let layout = f + 0x14;
        let job = f + 0x24;
        e.call(BUFFER_LAYOUT, &args![t, buffer, max_jobs as u32, layout]);
        profile_record(e, BUILD_COMMANDS_LABEL);
        let mut total = 0i32;
        let mut commands = e.mem.u32(layout);
        let mut outputs = e.mem.u32(layout + 4);
        let counts = e.mem.u32(layout + 8);
        let records = e.mem.u32(layout + 0xc);
        if vehicle_count > 0 {
            let mut index = 0u32;
            loop {
                let vehicles = e.mem.u32(t + 8);
                let vehicle = e.mem.u32(vehicles.wrapping_add(index.wrapping_mul(4)));
                let wheel_collide = e.mem.u32(vehicle + 0x3c);
                let made = e
                    .vcall(
                        wheel_collide,
                        0x30,
                        &args![vehicle, info_word, argument, commands, outputs],
                    )
                    .i32();
                if made > 0 {
                    total = total.wrapping_add(made);
                    e.mem.set_u8(counts.wrapping_add(index), made as u8);
                    commands = commands.wrapping_add((made as u32).wrapping_mul(0x50));
                    outputs = outputs.wrapping_add((made as u32).wrapping_mul(0x60));
                } else {
                    e.mem.set_u8(counts.wrapping_add(index), 0);
                }
                index += 1;
                if index as i32 >= vehicle_count {
                    break;
                }
            }
        }
        profile_record(e, PROFILE_END);
        let mut jobs = max_jobs;
        if jobs >= total {
            jobs = total;
        }
        let quotient = total / jobs;
        let remainder = total % jobs;
        let mut cursor = e.mem.u32(layout);
        let mut record = records;
        if jobs > 0 {
            let mut index = 0i32;
            loop {
                let share = i32::from(index < remainder) + quotient;
                e.mem.set_u8(job, 0);
                e.mem.set_u8(job + 1, 3);
                e.mem.set_u8(job + 2, 1);
                e.mem.set_u16(job + 4, 0x30);
                e.mem.set_u16(job + 6, 0xffff);
                e.mem.set_u32(job + 0x10, job_data);
                e.mem.set_u32(job + 0x14, record);
                e.mem.set_u32(job + 0x18, 0);
                let word = e.mem.u32(info + 0x78);
                e.mem.set_u32(job + 0x1c, word);
                e.mem.set_u32(job + 0x20, 0x5a);
                e.mem.set_u32(job + 0x24, cursor);
                e.mem.set_u32(job + 0x28, share as u32);
                e.mem
                    .set_u32(record, ((share.wrapping_sub(1)) / 0x5a + 1) as u32);
                e.call(JOB_PREPARE, &args![job]);
                e.call(JOB_QUEUE_ADD_JOB, &args![job_queue, job, 1u32]);
                cursor = cursor.wrapping_add((share as u32).wrapping_mul(0x50));
                index += 1;
                record = record.wrapping_add(0x10);
                if index >= jobs {
                    break;
                }
            }
        }
        profile_record(e, PROFILE_END);
        jobs
    })
}

// Translated from 00e42da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleRayCastBatchingManager::stepVehiclesUsingCastResults` (Xbox PDB name, engine map).
/// Thiscall, three stack words (`RET 0xc`): `step_info`, `packed` and `buffer`. Between profile
/// records (labels `010ebee4` and `010c41f8`) it lays the buffer out (`00e42b00`) and takes
/// `0x600` bytes of the memory router's stack allocator for an array of `0x60`-byte collision
/// outputs. For every vehicle it sizes that array to the vehicle's wheel count (the byte at data
/// `+0x20`) and, for every wheel `j` (a signed byte), calls its wheel-collide object (vehicle
/// `+0x3c`) with `packed` whose low byte is replaced by `j`: slot `0x3c` (the vehicle, the word, the
/// command's word at `+0x40`, output `j`) when the vehicle's command count is non-zero and the
/// current command's word at `+0x48` is not null, else slot `0x40` (the vehicle, the word,
/// output `j`); then slot `0x2c` (the vehicle, the word, output `j`); the command cursor moves
/// on by `0x50` bytes when the vehicle has commands. Then `stepVehicleUsingWheelCollideOutput`
/// (`00e39030`) runs on (the vehicle, `step_info`, the array). Finally the stack top is
/// restored and the array's buffer freed through the allocator (slot `0x10`) when its capacity
/// word is not negative (it never is here).
pub fn hkp_vehicle_ray_cast_batching_manager_step_vehicles_using_cast_results(
    e: &mut Engine,
    this: Ptr,
    step_info: u32,
    packed: u32,
    buffer: u32,
) {
    let t = this.addr();
    profile_record(e, STEP_VEHICLES_LABEL);
    let vehicle_count = e.mem.i32(t + 0xc);
    e.with_stack(0x20, |e, frame| {
        let layout = frame.addr();
        let array = frame.addr() + 0x10;
        e.call(BUFFER_LAYOUT, &args![t, buffer, packed, layout]);
        let mut cursor = e.mem.u32(layout);
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        e.mem.set_u32(router, top.wrapping_add(0x600));
        e.mem.set_u32(array, top);
        e.mem.set_u32(array + 8, 0x8000_0010);
        if vehicle_count > 0 {
            let mut index = 0u32;
            loop {
                let vehicles = e.mem.u32(t + 8);
                let vehicle = e.mem.u32(vehicles.wrapping_add(index.wrapping_mul(4)));
                let wheels = u32::from(e.mem.u8(e.mem.u32(vehicle + 0x1c) + 0x20));
                reserve_array(e, array, wheels as i32, 0x60);
                e.mem.set_u32(array + 4, wheels);
                let wheels = u32::from(e.mem.u8(e.mem.u32(vehicle + 0x1c) + 0x20));
                let wheel_collide = e.mem.u32(vehicle + 0x3c);
                let mut wheel = 0i8;
                if wheels > 0 {
                    loop {
                        let word = (packed & 0xffff_ff00) | u32::from(wheel as u8);
                        let counts = e.mem.u32(layout + 8);
                        let made = e.mem.u8(counts.wrapping_add(index));
                        let offset = (i32::from(wheel) as u32).wrapping_mul(0x60);
                        if made != 0 && e.mem.u32(cursor + 0x48) != 0 {
                            let hit = e.mem.u32(cursor + 0x40);
                            let output = e.mem.u32(array).wrapping_add(offset);
                            e.vcall(wheel_collide, 0x3c, &args![vehicle, word, hit, output]);
                        } else {
                            let output = e.mem.u32(array).wrapping_add(offset);
                            e.vcall(wheel_collide, 0x40, &args![vehicle, word, output]);
                        }
                        let output = e.mem.u32(array).wrapping_add(offset);
                        e.vcall(wheel_collide, 0x2c, &args![vehicle, word, output]);
                        if made != 0 {
                            cursor = cursor.wrapping_add(0x50);
                        }
                        wheel = wheel.wrapping_add(1);
                        if i32::from(wheel) >= wheels as i32 {
                            break;
                        }
                    }
                }
                let vehicles = e.mem.u32(t + 8);
                let vehicle = e.mem.u32(vehicles.wrapping_add(index.wrapping_mul(4)));
                let outputs = e.mem.u32(array);
                e.call(
                    STEP_VEHICLE_USING_OUTPUT,
                    &args![vehicle, step_info, outputs],
                );
                index += 1;
                if index as i32 >= vehicle_count {
                    break;
                }
            }
        }
        profile_record(e, PROFILE_END);
        if top == e.mem.u32(array) {
            e.mem.set_u32(array + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, top);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let outputs = e.mem.u32(array);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(0x60);
            e.vcall(allocator, 0x10, &args![outputs, bytes]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_2ac0, hkp_vehicle_ray_cast_batching_manager_get_buffer_size(Ptr, u32) -> u32),
        entry!(0x00e4_2b00, fn_00e42b00(Ptr, u32, u32, u32)),
        entry!(0x00e4_2b50, hkp_vehicle_ray_cast_batching_manager_build_and_add_cast_jobs(Ptr, u32, u32, i32, u32, u32, u32) -> i32),
        entry!(
            0x00e4_2da0,
            hkp_vehicle_ray_cast_batching_manager_step_vehicles_using_cast_results(
                Ptr, u32, u32, u32
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;

    /// An engine with the exe's data range mapped and a profiling stream with no room.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let stream = e.mem.alloc(0x10);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A manager with `vehicles` entries (the wheel-collide object of vehicle `k` is the
    /// object `k + 1` of the returned list; its slots `0x30`, `0x3c`, `0x40` and `0x2c` are
    /// logging doubles, slot `0x30` returning `built[k]`).
    fn manager(e: &mut Engine, log: &Log, built: &[u32]) -> (u32, Vec<u32>, Vec<u32>) {
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, array);
        e.mem.set_u32(this + 0xc, built.len() as u32);
        e.mem.set_u16(this + 0x14, 4);
        let vtable = e.mem.alloc(0x100);
        for slot in [0x30u32, 0x3c, 0x40, 0x2c] {
            e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
            let sink = Rc::clone(log);
            let returns = built.to_vec();
            let mut known: Vec<u32> = Vec::new();
            e.register_double(SLOT_BASE + slot, move |_, a| {
                sink.borrow_mut().push((SLOT_BASE + slot, a.to_vec()));
                if !known.contains(&a[0]) {
                    known.push(a[0]);
                }
                let k = known.iter().position(|object| *object == a[0]).unwrap();
                Ret {
                    eax: if slot == 0x30 { returns[k] } else { 0 },
                    ..Ret::default()
                }
            });
        }
        let mut vehicles = Vec::new();
        let mut wheel_collides = Vec::new();
        for k in 0..built.len() as u32 {
            let vehicle = e.mem.alloc(0x40);
            let wheel_collide = e.mem.alloc(0x10);
            e.mem.set_u32(wheel_collide, vtable);
            e.mem.set_u32(vehicle + 0x3c, wheel_collide);
            e.mem.set_u32(array + 4 * k, vehicle);
            vehicles.push(vehicle);
            wheel_collides.push(wheel_collide);
        }
        (this, vehicles, wheel_collides)
    }

    #[test]
    fn get_buffer_size_adds_the_aligned_areas() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 0x14, 3);
        e.mem.set_u32(this + 0xc, 0x21);
        let got = e.call(0x00e4_2ac0, &args![this, 5u32]).u32();
        assert_eq!(got, 0x120 + 0xf0 + 0x50 + 0x30);
    }

    #[test]
    fn fn_00e42b00_lays_out_the_buffer() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 0x14, 3);
        e.mem.set_u32(this + 0xc, 0x21);
        let layout = e.mem.alloc(0x10);
        e.call(0x00e4_2b00, &args![this, 0x1000u32, 0u32, layout]);
        assert_eq!(e.mem.u32(layout), 0x1000);
        assert_eq!(e.mem.u32(layout + 4), 0x10f0);
        assert_eq!(e.mem.u32(layout + 8), 0x1210);
        assert_eq!(e.mem.u32(layout + 0xc), 0x1240);
    }

    /// Runs `buildAndAddCastJobs` for two vehicles that build `built` commands, returning the
    /// result, the log, the buffer, the vehicles' wheel-collide objects and the job fields
    /// the queue saw (job words `0..0x2c`, as the double read them).
    fn run_jobs(built: [u32; 2], max_jobs: i32) -> (i32, Log, u32, Vec<u32>, Vec<Vec<u32>>) {
        let (mut e, log) = setup();
        let (this, vehicles, wheel_collides) = manager(&mut e, &log, &built);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x78, 0x7878);
        e.mem.set_u32(info + 0x7c, 0x7c7c);
        let buffer = e.mem.alloc(0x4000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(JOB_PREPARE, move |_, a| {
            sink.borrow_mut().push((JOB_PREPARE, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        let jobs = Rc::clone(&seen);
        e.register_double(JOB_QUEUE_ADD_JOB, move |e, a| {
            sink.borrow_mut().push((JOB_QUEUE_ADD_JOB, a.to_vec()));
            let mut fields = vec![
                u32::from(e.mem.u8(a[1])),
                u32::from(e.mem.u8(a[1] + 1)),
                u32::from(e.mem.u8(a[1] + 2)),
                u32::from(e.mem.u16(a[1] + 4)),
                u32::from(e.mem.u16(a[1] + 6)),
            ];
            fields.extend([0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28].map(|k| e.mem.u32(a[1] + k)));
            jobs.borrow_mut().push(fields);
            Ret::default()
        });
        let got = e.call(
            0x00e4_2b50,
            &args![this, info, 0x55u32, max_jobs, 0x9000u32, 0x66u32, buffer],
        );
        let records = e.mem.u32(buffer + 0x140 + 0x180 + 0x10);
        assert_eq!(vehicles.len(), 2);
        let seen = seen.borrow().clone();
        (got.i32(), log, records, wheel_collides, seen)
    }

    #[test]
    fn build_and_add_cast_jobs_splits_the_commands_over_the_jobs() {
        let (jobs, log, _, wheel_collides, seen) = run_jobs([3, 0], 2);
        assert_eq!(jobs, 2);
        let built = |wheel_collide: u32| {
            log.borrow()
                .iter()
                .filter(|(a, args)| *a == SLOT_BASE + 0x30 && args[0] == wheel_collide)
                .map(|(_, args)| args[3..].to_vec())
                .collect::<Vec<_>>()
        };
        assert_eq!(built(wheel_collides[0]).len(), 1);
        assert_eq!(built(wheel_collides[1]).len(), 1);
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][..5], [0, 3, 1, 0x30, 0xffff]);
        assert_eq!(seen[0][5], 0x66);
        assert_eq!(seen[0][8..], [0x7878, 0x5a, seen[0][10], 2]);
        assert_eq!(seen[1][11], 1);
        assert_eq!(seen[1][10], seen[0][10] + 2 * 0x50);
        assert_eq!(seen[1][6], seen[0][6] + 0x10);
        assert_eq!(calls_to(&log, JOB_PREPARE).len(), 2);
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB).len(), 2);
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][0], 0x9000);
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][2], 1);
    }

    #[test]
    fn build_and_add_cast_jobs_hands_each_vehicle_the_next_free_areas() {
        let (jobs, log, records, wheel_collides, seen) = run_jobs([100, 3], 1);
        assert_eq!(jobs, 1);
        let calls = calls_to(&log, SLOT_BASE + 0x30);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][0], wheel_collides[0]);
        assert_eq!(calls[0][2..5], [0x7c7c, 0x55, calls[0][4]]);
        assert_eq!(calls[1][4], calls[0][4] + 100 * 0x50);
        assert_eq!(calls[1][5], calls[0][5] + 100 * 0x60);
        assert_eq!(seen[0][11], 103);
        assert_eq!(records, 2);
    }

    #[test]
    fn step_vehicles_using_cast_results_collides_every_wheel_and_steps() {
        let (mut e, log) = setup();
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x10);
        let vehicle = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, 2);
        e.mem.set_u32(vehicle + 0x1c, data);
        e.mem.set_u32(array, vehicle);
        e.mem.set_u32(this + 8, array);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u16(this + 0x14, 2);
        let vtable = e.mem.alloc(0x100);
        let wheel_collide = e.mem.alloc(0x10);
        e.mem.set_u32(wheel_collide, vtable);
        e.mem.set_u32(vehicle + 0x3c, wheel_collide);
        for slot in [0x3cu32, 0x40, 0x2c] {
            e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
            let sink = Rc::clone(&log);
            e.register_double(SLOT_BASE + slot, move |_, a| {
                sink.borrow_mut().push((SLOT_BASE + slot, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        e.register_double(STEP_VEHICLE_USING_OUTPUT, move |_, a| {
            sink.borrow_mut()
                .push((STEP_VEHICLE_USING_OUTPUT, a.to_vec()));
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        let top = e.mem.alloc(0x800);
        e.mem.set_u32(router, top);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let buffer = e.mem.alloc(0x400);
        // Layout for two entries and one vehicle: commands, outputs, one count byte.
        let counts = buffer + 0xa0 + 0xc0;
        e.mem.set_u8(counts, 1);
        e.mem.set_u32(buffer + 0x48, 0x1111);
        e.mem.set_u32(buffer + 0x40, 0x2222);
        e.call(0x00e4_2da0, &args![this, 0x4444u32, 0xaabb_ccddu32, buffer]);
        let output = |wheel: u32| top + wheel * 0x60;
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x3c),
            vec![vec![wheel_collide, vehicle, 0xaabb_cc00, 0x2222, output(0)]]
        );
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x40),
            vec![vec![wheel_collide, vehicle, 0xaabb_cc01, output(1)]]
        );
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x2c),
            vec![
                vec![wheel_collide, vehicle, 0xaabb_cc00, output(0)],
                vec![wheel_collide, vehicle, 0xaabb_cc01, output(1)]
            ]
        );
        assert_eq!(
            calls_to(&log, STEP_VEHICLE_USING_OUTPUT),
            vec![vec![vehicle, 0x4444, top]]
        );
        assert_eq!(e.mem.u32(router), top);
    }
}
