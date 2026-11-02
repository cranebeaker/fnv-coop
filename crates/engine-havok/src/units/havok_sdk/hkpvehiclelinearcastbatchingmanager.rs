//! `Havok SDK/hkpvehiclelinearcastbatchingmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpVehicleLinearCastBatchingManager` is an `hkpVehicleCastBatchingManager`
//! (`hkpvehiclecastbatchingmanager.rs`), so an `hkpVehicleManager` too: the registered vehicles
//! are an array at `+8` (data `+8`, size `+0xC`) and `+0x14` is the 16-bit total number of
//! wheels. A vehicle's wheel collider (`+0x3C`, an `hkpVehicleLinearCastWheelCollide`) tells the
//! manager how many linear casts it needs (slot `0x30`).
//!
//! One buffer holds everything a batch needs, in four regions whose sizes are rounded up to 16
//! bytes: the per-wheel records (0x50 bytes each), the per-cast records (0x40 and 0x70 bytes
//! each) and the job headers (16 bytes each). `00e43060` lays them out and returns the four
//! addresses; `getBufferSize` gives the total. The profiling records use the host clock for
//! `RDTSC` (see `run_00cf5070`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

layout! {
    /// `hkpVehicleLinearCastBatchingManager` (Xbox PDB name; the count's name is unconfirmed).
    pub struct HkpVehicleLinearCastBatchingManager: 0x18 {
        /// The registered vehicles' data pointer (the `hkpVehicleManager` base).
        0x08 m_registered_vehicles: Ptr,
        /// The array's size.
        0x0c m_registered_vehicles_size: i32,
        /// The total number of wheels of the registered vehicles.
        0x14 m_total_num_wheels: u16,
    }
}

/// The function that adds up the vehicles' cast counts (`00e42fe0`) and the one that lays out
/// the buffer (`00e43060`).
const TOTAL_CASTS: u32 = 0x00e4_2fe0;
const LAY_OUT_BUFFER: u32 = 0x00e4_3060;
/// `hkpVehicleInstance::stepVehicleUsingWheelCollideOutput` (Xbox PDB, `this` = the vehicle,
/// then the step info and the wheel records).
const VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT: u32 = 0x00e3_9030;
/// The memory router (`00c85750`, no arguments): its first word is the top of its stack
/// allocator and its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The job type's constructor (`00d05db0`, a bare `RET`) and `hkJobQueue::addJob_ov2` (Xbox PDB,
/// `this` = the queue, then the job and 1).
const JOB_CONSTRUCTOR: u32 = 0x00d0_5db0;
const ADD_JOB: u32 = 0x00d0_4b90;
/// Profiling labels of `buildAndAddCastJobs` (`010ebef4` for the start, `010ebeb8` for the
/// loop over the vehicles) and of `stepVehiclesUsingCastResults` (`010ebee4`), and the shared
/// end label.
const LABEL_BUILD: u32 = 0x010e_bef4;
const LABEL_BUILD_VEHICLES: u32 = 0x010e_beb8;
const LABEL_STEP: u32 = 0x010e_bee4;
const LABEL_END: u32 = 0x010c_41f8;
/// The floats `buildAndAddCastJobs` copies into every job (`0104da4c` and `01013ea4`).
const JOB_FLOAT_A: u32 = 0x0104_da4c;
const JOB_FLOAT_B: u32 = 0x0101_3ea4;
/// The scratch array of `stepVehiclesUsingCastResults`: 0x600 bytes of the router's stack,
/// capacity 0x10 records (flags: not owned) of 0x60 bytes.
const SCRATCH_BYTES: u32 = 0x600;
const SCRATCH_FLAGS: u32 = 0x8000_0010;
const WHEEL_RECORD_SIZE: u32 = 0x60;

/// `CDQ`/`IDIV`: the PC raises a divide error (the game stops) for a zero divisor and for
/// `i32::MIN / -1`.
fn idiv(dividend: i32, divisor: i32) -> (i32, i32) {
    match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
        (Some(quotient), Some(remainder)) => (quotient, remainder),
        _ => panic!(
            "integer divide error (#DE) in buildAndAddCastJobs 00e430b0: {dividend} / {divisor}"
        ),
    }
}

/// `value` rounded up to a multiple of 16.
fn round_up_16(value: u32) -> u32 {
    value.wrapping_add(0xf) & 0xffff_fff0
}

// Translated from 00e42fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of linear casts all vehicles need (the map gives `00e42fe0` no name): the sum, as
/// a signed word, of the answers of every registered vehicle's wheel collider (`+0x3C` of the
/// vehicle, slot `0x30`, no arguments). The vehicle count is read again after every vehicle.
pub fn fn_00e42fe0(e: &mut Engine, this: Ptr<HkpVehicleLinearCastBatchingManager>) -> i32 {
    let mut total = 0i32;
    let mut i = 0i32;
    if e.get(
        this,
        HkpVehicleLinearCastBatchingManager::m_registered_vehicles_size,
    ) > 0
    {
        loop {
            let list = e
                .get(
                    this,
                    HkpVehicleLinearCastBatchingManager::m_registered_vehicles,
                )
                .addr();
            let vehicle = e.mem.u32(list.wrapping_add((i as u32).wrapping_mul(4)));
            let collide = e.mem.u32(vehicle + 0x3c);
            let casts = e.vcall(collide, 0x30, &args![]).i32();
            i += 1;
            total = total.wrapping_add(casts);
            if i >= e.get(
                this,
                HkpVehicleLinearCastBatchingManager::m_registered_vehicles_size,
            ) {
                break;
            }
        }
    }
    total
}

// Translated from 00e43010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastBatchingManager::getBufferSize` (Xbox PDB), `(this, jobs)`: with `casts`
/// = the casts all vehicles need (`00e42fe0`) and `wheels` = the total wheel count, the size of
/// the buffer: `wheels * 0x50`, `casts * 0x70`, `casts * 0x40` and `jobs * 16`, each rounded up
/// to 16, added.
pub fn get_buffer_size(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastBatchingManager>,
    jobs: u32,
) -> u32 {
    let casts = e.call(TOTAL_CASTS, &args![this]).u32();
    let wheels = u32::from(e.get(
        this,
        HkpVehicleLinearCastBatchingManager::m_total_num_wheels,
    ));
    let wheel_records = round_up_16(wheels.wrapping_mul(0x50));
    let cast_records = round_up_16(casts.wrapping_mul(0x70));
    let cast_inputs = round_up_16(casts.wrapping_mul(0x40));
    let job_headers = round_up_16(jobs << 4);
    wheel_records
        .wrapping_add(cast_records)
        .wrapping_add(cast_inputs)
        .wrapping_add(job_headers)
}

// Translated from 00e43060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lays the buffer out (the map gives `00e43060` no name), `(this, base, _, regions)`; the
/// second stack word is not read. Writes four addresses to `regions`: `base`; `base +
/// round16(wheels * 0x50)`; that `+ round16(casts * 0x40)`; that `+ round16(casts * 0x70)`
/// (where the job headers start), with `casts` from `00e42fe0`.
pub fn fn_00e43060(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastBatchingManager>,
    base: u32,
    _unused_2: u32,
    regions: Ptr,
) {
    let casts = e.call(TOTAL_CASTS, &args![this]).u32();
    let wheels = u32::from(e.get(
        this,
        HkpVehicleLinearCastBatchingManager::m_total_num_wheels,
    ));
    let cast_inputs_at = base.wrapping_add(round_up_16(wheels.wrapping_mul(0x50)));
    let cast_records_at = cast_inputs_at.wrapping_add(round_up_16(casts.wrapping_shl(6)));
    let job_headers_at = cast_records_at.wrapping_add(round_up_16(casts.wrapping_mul(0x70)));
    e.mem.set_u32(regions.addr(), base);
    e.mem.set_u32(regions.addr() + 4, cast_inputs_at);
    e.mem.set_u32(regions.addr() + 8, cast_records_at);
    e.mem.set_u32(regions.addr() + 0xc, job_headers_at);
}

// Translated from 00e430b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastBatchingManager::buildAndAddCastJobs` (Xbox PDB), `(this, world, _,
/// max jobs, job queue, job word, buffer)`; the third stack word is not read. Returns the
/// number of jobs made. With profile records (`010ebef4`, `010ebeb8`, end labels):
/// 1. the buffer is laid out (`00e43060` with `buffer`) into four regions `a`, `b`, `c` and the
///    job headers;
/// 2. every registered vehicle's wheel collider (slot `0x38`) is called with `(vehicle, the word
///    at world + 0x7C, a, b, c)` and answers its number of casts; `a` moves on by wheels * 0x50,
///    `b` by casts * 0x40 and `c` by casts * 0x70, and the casts are added up;
/// 3. the number of jobs is `min(max jobs, casts)` (signed), a zero (division by zero stops the
///    game) is a divide error. The casts are split among the jobs: `casts / jobs` each plus one
///    for the first `casts % jobs` (signed);
/// 4. for every job a 0x40-byte job is built on the stack and added to the queue (`00d04b90`
///    with the job and 1): type bytes 3, 2, 2, size 0x40 (`+4`) and 0xFFFF (`+6`), the job word
///    (`+0x10`), the address of its header (`+0x14`), the word at `world + 0x78` (`+0x18`), 0,
///    the floats 0.0 and the two copies (`+0x20` to `+0x28`), 0xA, 0x80, the current `b`
///    (`+0x34`) and the job's casts (`+0x38`). The header word is the batch count
///    `(casts - 1) / 128 + 1` (a signed shift) and `b` moves on by casts * 0x40. The bytes the
///    exe leaves unwritten (`+3`, `+8` to `+0xF`, `+0x3C`) are zero here.
pub fn build_and_add_cast_jobs(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastBatchingManager>,
    world: Ptr,
    _unused_2: u32,
    max_jobs: i32,
    job_queue: Ptr,
    job_word: u32,
    buffer: u32,
) -> i32 {
    profile_record(e, LABEL_BUILD);
    let simulation = e.mem.u32(world.addr() + 0x7c);
    let count = e.get(
        this,
        HkpVehicleLinearCastBatchingManager::m_registered_vehicles_size,
    );
    let regions = e.with_stack(0x10, |e, regions| {
        e.call(LAY_OUT_BUFFER, &args![this, buffer, max_jobs, regions]);
        std::array::from_fn::<u32, 4, _>(|i| e.mem.u32(regions.addr() + 4 * i as u32))
    });
    profile_record(e, LABEL_BUILD_VEHICLES);
    let mut total = 0i32;
    let (mut region_a, mut region_b, mut region_c) = (regions[0], regions[1], regions[2]);
    let mut i = 0i32;
    if count > 0 {
        loop {
            let list = e
                .get(
                    this,
                    HkpVehicleLinearCastBatchingManager::m_registered_vehicles,
                )
                .addr();
            let vehicle = e.mem.u32(list.wrapping_add((i as u32).wrapping_mul(4)));
            let collide = e.mem.u32(vehicle + 0x3c);
            let casts = e
                .vcall(
                    collide,
                    0x38,
                    &args![vehicle, simulation, region_a, region_b, region_c],
                )
                .i32();
            let data = e.mem.u32(vehicle + 0x1c);
            let wheels = u32::from(e.mem.u8(data + 0x20));
            total = total.wrapping_add(casts);
            region_c = region_c.wrapping_add(casts.wrapping_mul(0x70) as u32);
            region_b = region_b.wrapping_add((casts as u32).wrapping_shl(6));
            i += 1;
            region_a = region_a.wrapping_add(wheels * 0x50);
            if i >= count {
                break;
            }
        }
    }
    profile_record(e, LABEL_END);
    let jobs = if max_jobs < total { max_jobs } else { total };
    let (per_job, extra) = idiv(total, jobs);
    let mut header = regions[3];
    let mut cast_inputs = regions[1];
    let mut job_index = 0i32;
    if jobs > 0 {
        loop {
            let casts = per_job.wrapping_add(i32::from(job_index < extra));
            let float_a = x87_quiet(e.mem.f32(JOB_FLOAT_A));
            let float_b = x87_quiet(e.mem.f32(JOB_FLOAT_B));
            let island = e.mem.u32(world.addr() + 0x78);
            let batches = casts
                .wrapping_sub(1)
                .wrapping_add((casts.wrapping_sub(1) >> 31) & 0x7f)
                >> 7;
            e.with_stack(0x40, |e, job| {
                let job = job.addr();
                e.mem.set_u8(job, 3);
                e.mem.set_u8(job + 1, 2);
                e.mem.set_u8(job + 2, 2);
                e.mem.set_u16(job + 4, 0x40);
                e.mem.set_u16(job + 6, 0xffff);
                e.mem.set_u32(job + 0x10, job_word);
                e.mem.set_u32(job + 0x14, header);
                e.mem.set_u32(job + 0x18, island);
                e.mem.set_u32(job + 0x1c, 0);
                e.mem.set_f32(job + 0x20, 0.0);
                e.mem.set_f32(job + 0x24, float_a);
                e.mem.set_f32(job + 0x28, float_b);
                e.mem.set_u32(job + 0x2c, 0xa);
                e.mem.set_u32(job + 0x30, 0x80);
                e.mem.set_u32(job + 0x34, cast_inputs);
                e.mem.set_i32(job + 0x38, casts);
                e.mem.set_i32(header, batches.wrapping_add(1));
                e.call(JOB_CONSTRUCTOR, &args![job]);
                e.call(ADD_JOB, &args![job_queue, job, 1u32]);
            });
            cast_inputs = cast_inputs.wrapping_add((casts as u32).wrapping_shl(6));
            job_index += 1;
            header = header.wrapping_add(0x10);
            if job_index >= jobs {
                break;
            }
        }
    }
    profile_record(e, LABEL_END);
    jobs
}

// Translated from 00e43320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleLinearCastBatchingManager::stepVehiclesUsingCastResults` (Xbox PDB), `(this, step
/// info, _, buffer)`; the third stack word is not read (the exe overwrites its slot). With the
/// profile records `010ebee4` and the end label:
/// 1. the buffer is laid out as in `buildAndAddCastJobs`; `b` is its second region;
/// 2. 0x600 bytes of the router's stack are the storage of a local array of 0x60-byte wheel
///    records (capacity 0x10, not owned);
/// 3. for every vehicle (the count is read once): the array is made at least as big as the
///    vehicle's wheel count (`_reserve` with the router's allocator, `max(2 * capacity,
///    wheels)` when smaller) and its size set to it; then for every wheel `j` of the collider
///    (`+0x3C`) with `j_word` = the buffer parameter's slot with its low byte replaced by `j`
///    (the exe keeps the counter in the parameter's stack slot and passes the whole word on):
///    slot `0x3C` `(j_word, b)` answers a count `x`; if `x != 0` slot `0x44` `(vehicle, j_word,
///    x, record)` runs, else slot `0x48` `(vehicle, j_word, record)`; then slots `0x2C`
///    `(vehicle, j_word, record)` and `0x34` `(j_word)`, whose answer times 0x40 moves `b`
///    on. `record` = the array's data + `j * 0x60`. Then the vehicle is stepped
///    (`00e39030` with the step info and the array's data);
/// 4. after the end record the router's stack is given back and, when the array lives in
///    memory it owns (bit 31 of the capacity word clear), it is freed through the allocator
///    (slot `0x10`, data and capacity times 0x60).
pub fn step_vehicles_using_cast_results(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastBatchingManager>,
    step_info: Ptr,
    unused_2: u32,
    buffer: u32,
) {
    profile_record(e, LABEL_STEP);
    let count = e.get(
        this,
        HkpVehicleLinearCastBatchingManager::m_registered_vehicles_size,
    );
    let regions = e.with_stack(0x10, |e, regions| {
        e.call(LAY_OUT_BUFFER, &args![this, buffer, unused_2, regions]);
        std::array::from_fn::<u32, 4, _>(|i| e.mem.u32(regions.addr() + 4 * i as u32))
    });
    let mut cast_inputs = regions[1];
    e.with_stack(0xc, |e, array| {
        step_with_array(
            e,
            this,
            step_info,
            buffer,
            count,
            &mut cast_inputs,
            array.addr(),
        );
    });
}

/// The rest of [`step_vehicles_using_cast_results`] with the local array at `array`.
fn step_with_array(
    e: &mut Engine,
    this: Ptr<HkpVehicleLinearCastBatchingManager>,
    step_info: Ptr,
    buffer: u32,
    count: i32,
    cast_inputs: &mut u32,
    array: u32,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(router, top.wrapping_add(SCRATCH_BYTES));
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, SCRATCH_FLAGS);
    if count > 0 {
        let mut i = 0i32;
        loop {
            let list = e
                .get(
                    this,
                    HkpVehicleLinearCastBatchingManager::m_registered_vehicles,
                )
                .addr();
            let vehicle = e.mem.u32(list.wrapping_add((i as u32).wrapping_mul(4)));
            let data = e.mem.u32(vehicle + 0x1c);
            let wheels = i32::from(e.mem.u8(data + 0x20));
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
            let collide = e.mem.u32(vehicle + 0x3c);
            if wheels > 0 {
                let mut wheel = 0u8;
                loop {
                    let wheel_word = (buffer & 0xffff_ff00) | u32::from(wheel);
                    let casts = e
                        .vcall(collide, 0x3c, &args![wheel_word, *cast_inputs])
                        .u32();
                    let records = e.mem.u32(array);
                    let record = records.wrapping_add(u32::from(wheel) * WHEEL_RECORD_SIZE);
                    if casts != 0 {
                        e.vcall(collide, 0x44, &args![vehicle, wheel_word, casts, record]);
                    } else {
                        e.vcall(collide, 0x48, &args![vehicle, wheel_word, record]);
                    }
                    let records = e.mem.u32(array);
                    let record = records.wrapping_add(u32::from(wheel) * WHEEL_RECORD_SIZE);
                    e.vcall(collide, 0x2c, &args![vehicle, wheel_word, record]);
                    let used = e.vcall(collide, 0x34, &args![wheel_word]).u32();
                    *cast_inputs = cast_inputs.wrapping_add(used.wrapping_shl(6));
                    wheel = wheel.wrapping_add(1);
                    if i32::from(wheel) >= wheels {
                        break;
                    }
                }
            }
            let records = e.mem.u32(array);
            let list = e
                .get(
                    this,
                    HkpVehicleLinearCastBatchingManager::m_registered_vehicles,
                )
                .addr();
            let stepped = e.mem.u32(list.wrapping_add((i as u32).wrapping_mul(4)));
            e.call(
                VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT,
                &args![stepped, step_info, records],
            );
            i += 1;
            if i >= count {
                break;
            }
        }
    }
    profile_record(e, LABEL_END);
    // The array is still in the router's stack space: its size is cleared (the store below
    // makes this one unobservable).
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
        entry!(
            0x00e4_2fe0,
            fn_00e42fe0(Ptr<HkpVehicleLinearCastBatchingManager>) -> i32
        ),
        entry!(
            0x00e4_3010,
            get_buffer_size(Ptr<HkpVehicleLinearCastBatchingManager>, u32) -> u32
        ),
        entry!(
            0x00e4_3060,
            fn_00e43060(Ptr<HkpVehicleLinearCastBatchingManager>, u32, u32, Ptr)
        ),
        entry!(
            0x00e4_30b0,
            build_and_add_cast_jobs(
                Ptr<HkpVehicleLinearCastBatchingManager>,
                Ptr,
                u32,
                i32,
                Ptr,
                u32,
                u32,
            ) -> i32
        ),
        entry!(
            0x00e4_3320,
            step_vehicles_using_cast_results(
                Ptr<HkpVehicleLinearCastBatchingManager>,
                Ptr,
                u32,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, profile_labels, Log,
    };

    /// A wheel collider whose slots `0x30` and `0x38` answer `casts` and log; the rest of its
    /// vtable (slots `0x2C`, `0x34`, `0x3C`, `0x44`, `0x48`) are logging doubles answering
    /// `casts` for `0x38`, 0 otherwise.
    fn collider(e: &mut Engine, log: &Log, base: u32, casts: u32) -> u32 {
        let vtable = e.mem.alloc(0x50);
        for slot in (0..0x50).step_by(4) {
            let answer = if slot == 0x30 || slot == 0x38 {
                casts
            } else {
                0
            };
            log_double(e, log, base + slot, answer);
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let collider = e.mem.alloc(0x10);
        e.mem.set_u32(collider, vtable);
        collider
    }

    /// A vehicle with `wheels` wheels and the collider.
    fn vehicle(e: &mut Engine, wheels: u8, collide: u32) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, wheels);
        let vehicle = e.mem.alloc(0x60);
        e.mem.set_u32(vehicle + 0x1c, data);
        e.mem.set_u32(vehicle + 0x3c, collide);
        vehicle
    }

    /// A manager over `vehicles` with the given total wheel count.
    fn make_manager(e: &mut Engine, vehicles: &[u32], total_wheels: u16) -> u32 {
        let manager = e.mem.alloc(0x18);
        let list = e.mem.alloc(4 * vehicles.len().max(1) as u32);
        for (i, vehicle) in vehicles.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *vehicle);
        }
        e.mem.set_u32(manager + 8, list);
        e.mem.set_u32(manager + 0xc, vehicles.len() as u32);
        e.mem.set_u16(manager + 0x14, total_wheels);
        manager
    }

    #[test]
    fn the_cast_total_adds_every_colliders_answer() {
        let mut e = engine();
        let log = new_log();
        let first = collider(&mut e, &log, 0x0900_0000, 3);
        let second = collider(&mut e, &log, 0x0900_1000, 4);
        let one = vehicle(&mut e, 2, first);
        let two = vehicle(&mut e, 2, second);
        let manager = make_manager(&mut e, &[one, two], 4);
        assert_eq!(e.call(0x00e4_2fe0, &args![manager]).i32(), 7);
        assert_eq!(calls(&log, 0x0900_0030), vec![vec![first]]);
        assert_eq!(calls(&log, 0x0900_1030), vec![vec![second]]);
        // No vehicles: zero.
        let empty = manager_empty(&mut e);
        assert_eq!(e.call(0x00e4_2fe0, &args![empty]).i32(), 0);
    }

    fn manager_empty(e: &mut Engine) -> u32 {
        make_manager(e, &[], 0)
    }

    #[test]
    fn the_buffer_size_rounds_each_region_up_to_16() {
        let mut e = engine();
        let log = new_log();
        let first = collider(&mut e, &log, 0x0900_0000, 3);
        let one = vehicle(&mut e, 3, first);
        let manager = make_manager(&mut e, &[one], 3);
        // wheels 3 * 0x50 = 240; casts 3 * 0x70 = 336, 3 * 0x40 = 192; jobs 5 * 16 = 80.
        assert_eq!(
            e.call(0x00e4_3010, &args![manager, 5u32]).u32(),
            240 + 336 + 192 + 80
        );
        // One wheel: 0x50 is a multiple of 16; one cast: 0x70 and 0x40 as well; one job 16.
        let single = collider(&mut e, &log, 0x0900_2000, 1);
        let car = vehicle(&mut e, 1, single);
        let small = make_manager(&mut e, &[car], 1);
        assert_eq!(
            e.call(0x00e4_3010, &args![small, 1u32]).u32(),
            0x50 + 0x70 + 0x40 + 0x10
        );
        // Zero jobs add nothing.
        assert_eq!(
            e.call(0x00e4_3010, &args![small, 0u32]).u32(),
            0x50 + 0x70 + 0x40
        );
    }

    #[test]
    fn the_buffer_layout_gives_four_regions() {
        let mut e = engine();
        let log = new_log();
        let first = collider(&mut e, &log, 0x0900_0000, 3);
        let one = vehicle(&mut e, 3, first);
        let manager = make_manager(&mut e, &[one], 3);
        let out = e.mem.alloc(0x10);
        e.call(0x00e4_3060, &args![manager, 0x1000u32, 0x77u32, out]);
        let regions: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(
            regions,
            vec![
                0x1000,
                0x1000 + 240,
                0x1000 + 240 + 192,
                0x1000 + 240 + 192 + 336
            ]
        );
    }

    /// The job queue: `00d04b90` copies the job it is given.
    type Jobs = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    struct BuildRig {
        e: Engine,
        log: Log,
        jobs: Jobs,
        world: u32,
        manager: u32,
        stream: u32,
        records: u32,
    }

    /// A rig for `buildAndAddCastJobs`: vehicles with `(wheels, casts)`.
    fn build_rig(vehicles: &[(u8, u32)]) -> BuildRig {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let mut cars = Vec::new();
        let mut wheels = 0u16;
        for (i, (count, casts)) in vehicles.iter().enumerate() {
            let collide = collider(&mut e, &log, 0x0900_0000 + 0x1000 * i as u32, *casts);
            cars.push(vehicle(&mut e, *count, collide));
            wheels += u16::from(*count);
        }
        let manager = make_manager(&mut e, &cars, wheels);
        let world = e.mem.alloc(0x100);
        let simulation = e.mem.alloc(0x40);
        e.mem.set_u32(world + 0x7c, simulation);
        e.mem.set_u32(world + 0x78, 0xabcd);
        e.mem.set_u32(JOB_FLOAT_A, 0x3400_0000);
        e.mem.set_u32(JOB_FLOAT_B, 0x3c23_d70a);
        log_double(&mut e, &log, JOB_CONSTRUCTOR, 0);
        let jobs: Jobs = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = std::rc::Rc::clone(&jobs);
        e.register_double(ADD_JOB, move |e, a| {
            let mut words: Vec<u32> = (0..16).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            words.insert(0, a[0]);
            words.insert(1, a[2]);
            sink.borrow_mut().push(words);
            Ret::default()
        });
        BuildRig {
            e,
            log,
            jobs,
            world,
            manager,
            stream,
            records,
        }
    }

    #[test]
    fn building_jobs_splits_the_casts_among_the_jobs() {
        // 5 + 3 = 8 casts in 3 jobs: 3, 3, 2 (8 / 3 = 2 remainder 2: the first two get one more).
        let mut rig = build_rig(&[(2, 5), (1, 3)]);
        let buffer = rig.e.mem.alloc(0x1000);
        let queue = 0x4444;
        let jobs = rig
            .e
            .call(
                0x00e4_30b0,
                &args![rig.manager, rig.world, 0u32, 3i32, queue, 0x55u32, buffer],
            )
            .i32();
        assert_eq!(jobs, 3);
        // wheels 3 * 0x50 = 240; b: 8 * 0x40 = 512; c: 8 * 0x70 = 896 (all multiples of 16).
        let b = buffer + 240;
        let headers = buffer + 240 + 512 + 896;
        let added = rig.jobs.borrow().clone();
        assert_eq!(added.len(), 3);
        let casts = [3u32, 3, 2];
        let mut cast_inputs = b;
        for (i, job) in added.iter().enumerate() {
            // [queue, 1, words of the job...]
            assert_eq!(job[..2], [queue, 1]);
            let words = &job[2..];
            assert_eq!(words[0] & 0xff, 3);
            assert_eq!((words[0] >> 8) & 0xff, 2);
            assert_eq!((words[0] >> 16) & 0xff, 2);
            assert_eq!(words[1], 0xffff_0040);
            assert_eq!(words[4], 0x55);
            assert_eq!(words[5], headers + 16 * i as u32);
            assert_eq!(words[6], 0xabcd);
            assert_eq!(words[7], 0);
            assert_eq!(words[8], 0);
            assert_eq!(words[9], 0x3400_0000);
            assert_eq!(words[10], 0x3c23_d70a);
            assert_eq!(words[11], 0xa);
            assert_eq!(words[12], 0x80);
            assert_eq!(words[13], cast_inputs);
            assert_eq!(words[14], casts[i]);
            cast_inputs += casts[i] * 0x40;
        }
        // The header words hold the batch counts: (3 - 1) / 128 + 1 = 1 for each.
        for i in 0..3 {
            assert_eq!(rig.e.mem.u32(headers + 16 * i), 1);
        }
        assert_eq!(calls(&rig.log, JOB_CONSTRUCTOR).len(), 3);
        // The colliders got the vehicle, the simulation word and the advancing regions.
        let simulation = rig.e.mem.u32(rig.world + 0x7c);
        let first = calls(&rig.log, 0x0900_0038);
        let second = calls(&rig.log, 0x0900_1038);
        assert_eq!(first.len(), 1);
        assert_eq!(first[0][2], simulation);
        assert_eq!(first[0][3..], [buffer, buffer + 240, buffer + 240 + 512]);
        // The second vehicle starts after the first one's two wheels (0xA0), 5 b records
        // (0x140) and 5 c records (0x230).
        assert_eq!(
            second[0][3..],
            [
                buffer + 0xa0,
                buffer + 240 + 0x140,
                buffer + 240 + 512 + 0x230
            ]
        );
    }

    #[test]
    fn building_jobs_writes_profile_records() {
        let mut rig = build_rig(&[(2, 5)]);
        let buffer = rig.e.mem.alloc(0x1000);
        rig.e.call(
            0x00e4_30b0,
            &args![rig.manager, rig.world, 0u32, 8i32, 0x4444u32, 0u32, buffer],
        );
        assert_eq!(
            profile_labels(&rig.e, rig.stream, rig.records),
            vec![LABEL_BUILD, LABEL_BUILD_VEHICLES, LABEL_END, LABEL_END]
        );
    }

    #[test]
    fn building_jobs_never_makes_more_jobs_than_casts() {
        let mut rig = build_rig(&[(2, 2)]);
        let buffer = rig.e.mem.alloc(0x1000);
        let jobs = rig
            .e
            .call(
                0x00e4_30b0,
                &args![rig.manager, rig.world, 0u32, 8i32, 0x4444u32, 0u32, buffer],
            )
            .i32();
        assert_eq!(jobs, 2);
        let added = rig.jobs.borrow().clone();
        assert_eq!(added.len(), 2);
        assert_eq!([added[0][2 + 14], added[1][2 + 14]], [1, 1]);
    }

    #[test]
    fn a_long_job_needs_several_batches() {
        // 300 casts in one job: (300 - 1) / 128 + 1 = 3 batches.
        let mut rig = build_rig(&[(1, 300)]);
        let buffer = rig.e.mem.alloc(0x10000);
        rig.e.call(
            0x00e4_30b0,
            &args![rig.manager, rig.world, 0u32, 1i32, 0x4444u32, 0u32, buffer],
        );
        let headers = buffer + 0x50 + 300 * 0x40 + 300 * 0x70;
        assert_eq!(rig.e.mem.u32(headers), 3);
    }

    #[test]
    #[should_panic(expected = "integer divide error")]
    fn building_jobs_without_casts_stops_on_the_division() {
        let mut rig = build_rig(&[(2, 0)]);
        let buffer = rig.e.mem.alloc(0x1000);
        rig.e.call(
            0x00e4_30b0,
            &args![rig.manager, rig.world, 0u32, 4i32, 0x4444u32, 0u32, buffer],
        );
    }

    /// A rig for `stepVehiclesUsingCastResults`.
    struct StepRig {
        e: Engine,
        log: Log,
        manager: u32,
        router: u32,
        stream: u32,
        records: u32,
    }

    fn step_rig(vehicles: &[(u8, u32)]) -> StepRig {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let mut cars = Vec::new();
        let mut wheels = 0u16;
        for (i, (count, casts)) in vehicles.iter().enumerate() {
            let collide = collider(&mut e, &log, 0x0900_0000 + 0x1000 * i as u32, *casts);
            cars.push(vehicle(&mut e, *count, collide));
            wheels += u16::from(*count);
        }
        let manager = make_manager(&mut e, &cars, wheels);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, 0x5000);
        e.mem.set_u32(router + 0x10, 0x7777_0000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT, 0);
        log_double(&mut e, &log, ARRAY_RESERVE, 0);
        StepRig {
            e,
            log,
            manager,
            router,
            stream,
            records,
        }
    }

    #[test]
    fn stepping_runs_the_collider_for_every_wheel() {
        // One vehicle with 2 wheels and 3 casts per vehicle (slot 0x38 answers 3, but the
        // per-wheel slots 0x3C and 0x34 answer 0).
        let mut rig = step_rig(&[(2, 3)]);
        let step = rig.e.mem.alloc(0x20);
        // The buffer's low byte is replaced by the wheel number in the word passed on.
        rig.e.call(
            0x00e4_3320,
            &args![rig.manager, step, 0x99u32, 0x1234_56ffu32],
        );
        let vehicle = rig.e.mem.u32(rig.e.mem.u32(rig.manager + 8));
        let log = &rig.log;
        // Slot 0x3C: (wheel word, b region); b = buffer + round16(2 * 0x50) = buffer + 160.
        let first_cast = calls(log, 0x0900_003c);
        assert_eq!(first_cast.len(), 2);
        assert_eq!(first_cast[0][1..], [0x1234_5600, 0x1234_56ff + 160]);
        assert_eq!(first_cast[1][1..], [0x1234_5601, 0x1234_56ff + 160]);
        // A zero answer takes slot 0x48 (vehicle, word, record); slot 0x44 is not used.
        let plain = calls(log, 0x0900_0048);
        assert_eq!(plain.len(), 2);
        assert_eq!(plain[0][1..3], [vehicle, 0x1234_5600]);
        assert_eq!(plain[1][1..3], [vehicle, 0x1234_5601]);
        // The records are 0x60 apart.
        assert_eq!(plain[1][3] - plain[0][3], 0x60);
        assert!(calls(log, 0x0900_0044).is_empty());
        // Slot 0x2C gets the same record, slot 0x34 the wheel word.
        let finish = calls(log, 0x0900_002c);
        assert_eq!(finish[0][3], plain[0][3]);
        assert_eq!(calls(log, 0x0900_0034)[1][1], 0x1234_5601);
        // The vehicle is stepped with the step info and the records.
        assert_eq!(
            calls(log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT),
            vec![vec![vehicle, step, plain[0][3]]]
        );
        assert_eq!(rig.e.mem.u32(rig.router), 0x5000);
        assert_eq!(
            profile_labels(&rig.e, rig.stream, rig.records),
            vec![LABEL_STEP, LABEL_END]
        );
    }

    #[test]
    fn stepping_uses_slot_0x44_for_wheels_with_casts() {
        let mut rig = step_rig(&[(1, 0)]);
        let vehicle = rig.e.mem.u32(rig.e.mem.u32(rig.manager + 8));
        let collide = rig.e.mem.u32(vehicle + 0x3c);
        // The collider's slot 0x3C answers 5 casts.
        let vtable = rig.e.mem.u32(collide);
        rig.e.mem.set_u32(vtable + 0x3c, 0x0900_8000);
        log_double(&mut rig.e, &rig.log, 0x0900_8000, 5);
        let step = rig.e.mem.alloc(0x20);
        rig.e
            .call(0x00e4_3320, &args![rig.manager, step, 0u32, 0x2000u32]);
        let with_casts = calls(&rig.log, 0x0900_0044);
        assert_eq!(with_casts.len(), 1);
        assert_eq!(with_casts[0][2..4], [0x2000, 5]);
        assert!(calls(&rig.log, 0x0900_0048).is_empty());
    }

    #[test]
    fn stepping_advances_the_cast_region_by_the_used_casts() {
        let mut rig = step_rig(&[(2, 0)]);
        let vehicle = rig.e.mem.u32(rig.e.mem.u32(rig.manager + 8));
        let collide = rig.e.mem.u32(vehicle + 0x3c);
        let vtable = rig.e.mem.u32(collide);
        // Slot 0x34 answers 2 casts used per wheel.
        rig.e.mem.set_u32(vtable + 0x34, 0x0900_8034);
        log_double(&mut rig.e, &rig.log, 0x0900_8034, 2);
        let step = rig.e.mem.alloc(0x20);
        rig.e
            .call(0x00e4_3320, &args![rig.manager, step, 0u32, 0x2000u32]);
        let first_cast = calls(&rig.log, 0x0900_003c);
        assert_eq!(first_cast[1][2] - first_cast[0][2], 2 << 6);
    }

    #[test]
    fn stepping_a_big_vehicle_reserves_and_frees_the_records() {
        let mut rig = step_rig(&[(20, 0)]);
        let block = rig.e.mem.alloc(0x100);
        let reserved = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = std::rc::Rc::clone(&reserved);
        rig.e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let free_log = new_log();
        let vtable = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(vtable + 0x10, 0x0900_f010);
        let allocator = rig.e.mem.alloc(0x10);
        rig.e.mem.set_u32(allocator, vtable);
        log_double(&mut rig.e, &free_log, 0x0900_f010, 0);
        rig.e.mem.set_u32(rig.router + 0x10, allocator);
        let step = rig.e.mem.alloc(0x20);
        rig.e
            .call(0x00e4_3320, &args![rig.manager, step, 0u32, 0x2000u32]);
        let reserved = reserved.borrow();
        assert_eq!(reserved.len(), 1);
        // 20 > 16: max(2 * 16, 20) = 32 records of 0x60.
        assert_eq!(
            [reserved[0][0], reserved[0][2], reserved[0][3]],
            [allocator, 32, 0x60]
        );
        assert_eq!(
            calls(&free_log, 0x0900_f010),
            vec![vec![allocator, block, 32 * 0x60]]
        );
    }

    #[test]
    fn stepping_without_vehicles_only_gives_back_the_scratch_space() {
        let mut rig = step_rig(&[]);
        let step = rig.e.mem.alloc(0x20);
        rig.e
            .call(0x00e4_3320, &args![rig.manager, step, 0u32, 0x2000u32]);
        assert_eq!(rig.e.mem.u32(rig.router), 0x5000);
        assert!(calls(&rig.log, VEHICLE_STEP_USING_WHEEL_COLLIDE_OUTPUT).is_empty());
    }
}
