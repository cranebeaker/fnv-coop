//! `Havok SDK/hkcpujobthreadpool.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkSemaphore` constructor (`00d04350`, `this`, `(initial count, maximum count)`).
const SEMAPHORE_CONSTRUCT: u32 = 0x00d0_4350;
/// `hkSemaphore` destructor (`00d04370`).
const SEMAPHORE_DESTRUCT: u32 = 0x00d0_4370;
/// `hkSemaphore::acquire` (`00d04380`).
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `hkSemaphore::release` (`00d04390`, `this`, `(count)`).
const SEMAPHORE_RELEASE: u32 = 0x00d0_4390;
/// The thread wrapper's constructor (`00d87910`, clears the handle at `this`).
const THREAD_CONSTRUCT: u32 = 0x00d8_7910;
/// The thread wrapper's handle closer (`00d87920`).
const THREAD_CLOSE: u32 = 0x00d8_7920;
/// The thread wrapper's creator (`00d87940`, `this`, `(function, argument, extra word, stack
/// size)`).
const THREAD_CREATE: u32 = 0x00d8_7940;
/// The thread wrapper's destructor (`00d87a70`).
const THREAD_DESTRUCT: u32 = 0x00d8_7a70;
/// `hkReferencedObject::setLockMode` (`00c90410`, cdecl `(mode)`).
const SET_LOCK_MODE: u32 = 0x00c9_0410;
/// The job queue's processing routine (`00d051b0`, `this` = the queue).
const JOB_QUEUE_PROCESS: u32 = 0x00d0_51b0;
/// `hkArrayUtil` growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The memory router's constructor (`00c85770`, clears its eight words).
const ROUTER_CONSTRUCT: u32 = 0x00c8_5770;
/// The memory system accessor (`00cbbfc0`, answers the memory system object).
const MEMORY_SYSTEM: u32 = 0x00cb_bfc0;
/// The memory system's per-thread start (`00cbb930`, cdecl `(router)`).
const MEMORY_INIT_THREAD: u32 = 0x00cb_b930;
/// The memory system's per-thread end (`00cbb990`).
const MEMORY_QUIT_THREAD: u32 = 0x00cb_b990;
/// Resizes the thread's monitor stream (`00cb0e50`, cdecl `(stream, size)`).
const MONITOR_RESIZE: u32 = 0x00cb_0e50;
/// Resets the thread's monitor stream (`00cb0da0`, cdecl `(stream)`).
const MONITOR_RESET: u32 = 0x00cb_0da0;
/// Import slot of `TlsGetValue`.
const API_TLS_GET_VALUE: u32 = 0x00fd_f044;
/// Import slot of `TlsSetValue`.
const API_TLS_SET_VALUE: u32 = 0x00fd_f04c;
/// Import slot of `GetSystemInfo`.
const API_GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
/// Import slot of `GetCurrentThread`.
const API_GET_CURRENT_THREAD: u32 = 0x00fd_f120;
/// Import slot of `SetThreadIdealProcessor`.
const API_SET_THREAD_IDEAL_PROCESSOR: u32 = 0x00fd_f12c;
/// The exe's global holding the TLS index of the thread number.
const THREAD_NUMBER_TLS_INDEX: u32 = 0x0126_87c8;
/// The exe's global holding the TLS index of the monitor stream.
const MONITOR_TLS_INDEX: u32 = 0x0126_835c;
/// The worker thread's entry function.
const WORKER_THREAD_FUNCTION: u32 = 0x00d2_cfb0;

/// The pool object: six 0x30-byte thread records start at `+8`; at `+0x128` is the block the
/// workers share (the job queue pointer, then a semaphore at `+0x12C`), at `+0x130` the thread
/// count, `+0x134` and `+0x138` two settings (the second is the monitor stream size) and
/// `+0x13C` the "processing" byte; `+0x140` and `+0x144` are the thread creation arguments.
const RECORDS: u32 = 8;
const RECORD_SIZE: u32 = 0x30;
const MAX_THREADS: u32 = 6;
const SHARED: u32 = 0x128;
const THREAD_COUNT: u32 = 0x130;
const PROCESSING: u32 = 0x13c;

/// The address of thread record `index` of the pool at `this`.
fn record(this: u32, index: u32) -> u32 {
    this.wrapping_add(RECORDS)
        .wrapping_add(index.wrapping_mul(RECORD_SIZE))
}

/// Calls `GetSystemInfo` and answers `dwNumberOfProcessors` (at `+0x14` of the 0x24-byte
/// structure).
fn processor_count(e: &mut Engine) -> i32 {
    e.with_stack(0x24, |e, info| {
        e.call(API_GET_SYSTEM_INFO, &args![info]);
        e.mem.i32(info.addr() + 0x14)
    })
}

/// Starts the thread of `record` (the thread wrapper at `+8`) running the worker function with
/// the record as its argument, and answers the creator's result.
fn create_thread(e: &mut Engine, this: u32, record: u32) -> u32 {
    let extra = e.mem.u32(this + 0x140);
    let stack_size = e.mem.u32(this + 0x144);
    e.call(
        THREAD_CREATE,
        &args![
            record + 8,
            WORKER_THREAD_FUNCTION,
            record,
            extra,
            stack_size
        ],
    )
    .u32()
}

/// The thread-local monitor stream of the calling thread.
fn monitor_stream(e: &mut Engine) -> u32 {
    let index = e.global::<u32>(MONITOR_TLS_INDEX);
    e.call(API_TLS_GET_VALUE, &args![index]).u32()
}

// Translated from 00d2cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the semaphore at `this + 4` (initial count 0, maximum 6); answers `this`.
pub fn fn_00d2cd80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SEMAPHORE_CONSTRUCT, &args![this.addr() + 4, 0u32, 6u32]);
    this
}

// Translated from 00d2cda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the thread record at `this`: constructs the thread wrapper (`+8`) and the
/// record's semaphore (`+0x24`, initial count 0, maximum 1), clears the monitor words
/// (`+0x28`, `+0x2C`) and the stop byte (`+0x20`), sets the thread number (`+0x18`) to -1 and
/// the shared block pointer (`+0`) to null; answers `this`. The clear-timer byte (`+0x21`) is
/// not touched.
pub fn fn_00d2cda0(e: &mut Engine, this: Ptr) -> Ptr {
    let r = this.addr();
    e.call(THREAD_CONSTRUCT, &args![r + 8]);
    e.call(SEMAPHORE_CONSTRUCT, &args![r + 0x24, 0u32, 1u32]);
    e.mem.set_u32(r + 0x28, 0);
    e.mem.set_u32(r + 0x2c, 0);
    e.mem.set_u8(r + 0x20, 0);
    e.mem.set_u32(r + 0x18, 0xffff_ffff);
    e.mem.set_u32(r, 0);
    this
}

// Translated from 00d2cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::processAllJobs` (Xbox PDB; `this` = the pool, then two stack words,
/// `RET 8`, the second unused): stores the job queue pointer in the shared block (`+0x128`),
/// sets the processing byte and releases every thread's semaphore once, last thread first.
pub fn hk_cpu_job_thread_pool_process_all_jobs(
    e: &mut Engine,
    this: Ptr,
    job_queue: u32,
    _unused_1: u32,
) {
    let pool = this.addr();
    e.mem.set_u32(pool + SHARED, job_queue);
    e.mem.set_u8(pool + PROCESSING, 1);
    let mut index = e.mem.i32(pool + THREAD_COUNT).wrapping_sub(1);
    while index >= 0 {
        e.call(
            SEMAPHORE_RELEASE,
            &args![record(pool, index as u32) + 0x24, 1u32],
        );
        index -= 1;
    }
}

// Translated from 00d2ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::isProcessing` (Xbox PDB): whether the processing byte (`+0x13C`) is
/// not zero.
pub fn hk_cpu_job_thread_pool_is_processing(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + PROCESSING) != 0
}

// Translated from 00d2ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::getNumThreads` (Xbox PDB): the thread count (`+0x130`).
pub fn hk_cpu_job_thread_pool_get_num_threads(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + THREAD_COUNT)
}

// Translated from 00d2ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops the last thread: lowers the thread count, sets the stop byte (`+0x20`) of that
/// thread's record, releases its semaphore once, acquires the shared semaphore (`+0x12C`) and
/// tail-calls the thread wrapper's handle closer (`00d87920`) on the record's wrapper
/// (`+8`), answering its result.
pub fn fn_00d2ce50(e: &mut Engine, this: Ptr) -> u32 {
    let pool = this.addr();
    let count = e.mem.u32(pool + THREAD_COUNT).wrapping_sub(1);
    e.mem.set_u32(pool + THREAD_COUNT, count);
    let r = record(pool, count);
    e.mem.set_u8(r + 0x20, 1);
    e.call(SEMAPHORE_RELEASE, &args![r + 0x24, 1u32]);
    e.call(SEMAPHORE_ACQUIRE, &args![pool + 0x12c]);
    e.call(THREAD_CLOSE, &args![r + 8]).u32()
}

// Translated from 00d2ce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::waitForCompletion` (Xbox PDB): when the processing byte is set,
/// acquires the shared semaphore (`+0x12C`) once per thread, then clears the byte.
pub fn hk_cpu_job_thread_pool_wait_for_completion(e: &mut Engine, this: Ptr) {
    let pool = this.addr();
    if e.mem.u8(pool + PROCESSING) != 0 {
        if e.mem.i32(pool + THREAD_COUNT) > 0 {
            let mut waited = 0;
            loop {
                e.call(SEMAPHORE_ACQUIRE, &args![pool + 0x12c]);
                waited += 1;
                if waited >= e.mem.i32(pool + THREAD_COUNT) {
                    break;
                }
            }
        }
        e.mem.set_u8(pool + PROCESSING, 0);
    }
}

// Translated from 00d2ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::clearTimerData` (Xbox PDB): for every thread, copies its monitor
/// begin word (`+0x28`) to its current word (`+0x2C`) and sets its clear-timer byte (`+0x21`).
pub fn hk_cpu_job_thread_pool_clear_timer_data(e: &mut Engine, this: Ptr) {
    let pool = this.addr();
    if e.mem.i32(pool + THREAD_COUNT) > 0 {
        let mut index = 0u32;
        loop {
            let r = record(pool, index);
            let begin = e.mem.u32(r + 0x28);
            e.mem.set_u32(r + 0x2c, begin);
            e.mem.set_u8(r + 0x21, 1);
            index += 1;
            if index as i32 >= e.mem.i32(pool + THREAD_COUNT) {
                break;
            }
        }
    }
}

// Translated from 00d2cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::~hkCpuJobThreadPool` (Xbox PDB): sets the vtable to the pool's own
/// (`0x010CE774`), waits for completion, then for every thread sets the stop byte and releases
/// its semaphore, acquires the shared semaphore once per thread, sets the lock mode to 0
/// (`00c90410`), destroys the shared semaphore and destroys all six records' semaphores and
/// thread wrappers, last record first. Ends with the base vtable `0x0102E388`.
pub fn hk_cpu_job_thread_pool_destruct(e: &mut Engine, this: Ptr) {
    let pool = this.addr();
    e.mem.set_u32(pool, 0x010c_e774);
    hk_cpu_job_thread_pool_wait_for_completion(e, this);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(pool + THREAD_COUNT) {
        let r = record(pool, index);
        e.mem.set_u8(r + 0x20, 1);
        e.call(SEMAPHORE_RELEASE, &args![r + 0x24, 1u32]);
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(pool + THREAD_COUNT) {
        e.call(SEMAPHORE_ACQUIRE, &args![pool + 0x12c]);
        index += 1;
    }
    e.call(SET_LOCK_MODE, &args![0u32]);
    e.call(SEMAPHORE_DESTRUCT, &args![pool + 0x12c]);
    for index in (0..MAX_THREADS).rev() {
        let r = record(pool, index);
        e.call(SEMAPHORE_DESTRUCT, &args![r + 0x24]);
        e.call(THREAD_DESTRUCT, &args![r + 8]);
    }
    e.mem.set_u32(pool, 0x0102_e388);
}

// Translated from 00d2cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2cfb0 - the exe sets bit 15 (flush-to-zero) of MXCSR before
// anything else; the engine has no floating-point control state, so that is not modelled.
/// `hkWorkerThreadFunc` (Xbox PDB; cdecl, `thread` = the thread record): the worker thread's
/// body. Stores its thread number in the thread-local slot, pins itself to its ideal
/// processor, builds a memory router on its own stack and registers it with the memory system
/// (virtual call at `+0xC` with the name `hkCpuJobThreadPool` and 3), starts the thread's
/// memory use (`00cbb930`), sizes the thread's monitor stream when the shared block asks for
/// it (`+0x10` above 0) and stores the stream's two words in the record (`+0x28`, `+0x2C`).
/// Then it works: acquires its own semaphore, and until the stop byte is set it resets the
/// monitor stream if asked (clear-timer byte), runs the job queue (`00d051b0`), refreshes
/// the current monitor word, releases the shared semaphore and acquires its own again. At the
/// end it ends the thread's memory use (`00cbb990`), gives the router back through the
/// memory system (virtual call at `+0x10` with 3), releases the shared semaphore once more
/// and answers 0.
pub fn hk_worker_thread_func(e: &mut Engine, thread: Ptr) -> u32 {
    let r = thread.addr();
    let shared = e.mem.u32(r);
    let number = e.mem.u32(r + 0x18);
    let number_index = e.global::<u32>(THREAD_NUMBER_TLS_INDEX);
    e.call(API_TLS_SET_VALUE, &args![number_index, number]);
    let ideal_processor = e.mem.u32(r + 0x1c);
    let current_thread = e.call(API_GET_CURRENT_THREAD, &args![]).u32();
    e.call(
        API_SET_THREAD_IDEAL_PROCESSOR,
        &args![current_thread, ideal_processor],
    );
    e.with_stack(0x20, |e, router| {
        let router = router.addr();
        e.call(ROUTER_CONSTRUCT, &args![router]);
        let memory_system = e.call(MEMORY_SYSTEM, &args![]).u32();
        e.vcall(memory_system, 0xc, &args![router, 0x010c_e79cu32, 3u32]);
        e.call(MEMORY_INIT_THREAD, &args![router]);
        if e.mem.i32(shared + 0x10) > 0 {
            let stream = monitor_stream(e);
            let size = e.mem.u32(shared + 0x10);
            e.call(MONITOR_RESIZE, &args![stream, size]);
        }
        let stream = monitor_stream(e);
        let begin = e.mem.u32(stream);
        e.mem.set_u32(r + 0x28, begin);
        let stream = monitor_stream(e);
        let current = e.mem.u32(stream + 4);
        e.mem.set_u32(r + 0x2c, current);
        e.call(SEMAPHORE_ACQUIRE, &args![r + 0x24]);
        while e.mem.u8(r + 0x20) == 0 {
            if e.mem.u8(r + 0x21) != 0 {
                let stream = monitor_stream(e);
                e.call(MONITOR_RESET, &args![stream]);
                let stream = monitor_stream(e);
                let current = e.mem.u32(stream + 4);
                e.mem.set_u32(r + 0x2c, current);
                e.mem.set_u8(r + 0x21, 0);
            }
            let queue = e.mem.u32(shared);
            e.call(JOB_QUEUE_PROCESS, &args![queue]);
            let stream = monitor_stream(e);
            let current = e.mem.u32(stream + 4);
            e.mem.set_u32(r + 0x2c, current);
            e.call(SEMAPHORE_RELEASE, &args![shared + 4, 1u32]);
            e.call(SEMAPHORE_ACQUIRE, &args![r + 0x24]);
        }
        e.call(MEMORY_QUIT_THREAD, &args![]);
        let memory_system = e.call(MEMORY_SYSTEM, &args![]).u32();
        e.vcall(memory_system, 0x10, &args![router, 3u32]);
    });
    e.call(SEMAPHORE_RELEASE, &args![shared + 4, 1u32]);
    0
}

// Translated from 00d2d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pool's constructor (`this` = the pool, then the creation info, `RET 4`; the class
/// name is not in the map): sets the word at `+6` to 1 and the vtable to `0x010CE774`,
/// initialises all six thread records (`00d2cda0`) and the shared semaphore (`00d2cd80`),
/// clears the processing byte, copies the creation info's words `+0x1C`, `+8`, `+4` and
/// `+0xC` to `+0x140`, `+0x144`, `+0x134` and `+0x138`, and stores the thread count (the info's
/// first word, 5 when it is above 6) at `+0x130`. Each thread to start gets its record set up
/// (shared block pointer, thread number from 1, empty monitor words, cleared stop and
/// clear-timer bytes) and its ideal processor: the entry of the info's array (`+0x10`, used when
/// the count at `+0x14` is above 0) for this thread, otherwise the thread number modulo the
/// processor count (`GetSystemInfo`); the thread is created through `00d87940`. Ends with the
/// lock mode set to 1 (`00c90410`); answers `this`.
pub fn fn_00d2d110(e: &mut Engine, this: Ptr, info: Ptr) -> Ptr {
    let pool = this.addr();
    let info = info.addr();
    e.mem.set_u16(pool + 6, 1);
    e.mem.set_u32(pool, 0x010c_e774);
    for index in 0..MAX_THREADS {
        fn_00d2cda0(e, Ptr::new(record(pool, index)));
    }
    fn_00d2cd80(e, Ptr::new(pool + SHARED));
    e.mem.set_u8(pool + PROCESSING, 0);
    let value = e.mem.u32(info + 0x1c);
    e.mem.set_u32(pool + 0x140, value);
    let value = e.mem.u32(info + 8);
    e.mem.set_u32(pool + 0x144, value);
    let value = e.mem.u32(info + 4);
    e.mem.set_u32(pool + 0x134, value);
    let value = e.mem.u32(info + 0xc);
    e.mem.set_u32(pool + 0x138, value);
    let mut count = e.mem.i32(info);
    if count > MAX_THREADS as i32 {
        count = 5;
    }
    e.mem.set_u32(pool + THREAD_COUNT, count as u32);
    let processors = processor_count(e);
    let mut offset = 0u32;
    let mut index = 0u32;
    while (index as i32) < count {
        let r = record(pool, index);
        e.mem.set_u32(r, pool + SHARED);
        e.mem.set_u32(r + 0x18, index + 1);
        e.mem.set_u32(r + 0x28, 0);
        e.mem.set_u32(r + 0x2c, 0);
        e.mem.set_u8(r + 0x20, 0);
        e.mem.set_u8(r + 0x21, 0);
        let ideal = if e.mem.i32(info + 0x14) > 0 {
            let array = e.mem.u32(info + 0x10);
            e.mem.u32(array.wrapping_add(offset))
        } else {
            // A processor count of 0 divides by zero (#DE) in the exe.
            ((index as i32 + 1) % processors) as u32
        };
        e.mem.set_u32(r + 0x1c, ideal);
        create_thread(e, pool, r);
        offset = offset.wrapping_add(4);
        index += 1;
    }
    e.call(SET_LOCK_MODE, &args![1u32]);
    this
}

// Translated from 00d2d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one worker thread (the class name is not in the map): sets up the record after the
/// last one (shared block pointer, empty monitor words, cleared stop and clear-timer bytes,
/// thread number = the current count + 1, ideal processor = that number modulo the processor
/// count from `GetSystemInfo`), creates its thread (`00d87940`) and raises the count. Answers
/// what the thread creator answered.
pub fn fn_00d2d240(e: &mut Engine, this: Ptr) -> u32 {
    let pool = this.addr();
    let count = e.mem.u32(pool + THREAD_COUNT);
    let r = record(pool, count);
    e.mem.set_u32(r, pool + SHARED);
    e.mem.set_u32(r + 0x28, 0);
    e.mem.set_u32(r + 0x2c, 0);
    e.mem.set_u8(r + 0x20, 0);
    e.mem.set_u8(r + 0x21, 0);
    e.mem.set_u32(r + 0x18, count.wrapping_add(1));
    let processors = processor_count(e);
    // A processor count of 0 divides by zero (#DE) in the exe.
    let ideal = (e.mem.i32(r + 0x18) % processors) as u32;
    e.mem.set_u32(r + 0x1c, ideal);
    let created = create_thread(e, pool, r);
    let count = e.mem.u32(pool + THREAD_COUNT);
    e.mem.set_u32(pool + THREAD_COUNT, count.wrapping_add(1));
    created
}

// Translated from 00d2d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::appendTimerData` (Xbox PDB; `this` = the pool, then `(array,
/// allocator)`, `RET 8`): for every thread appends its two monitor words (`+0x28`, `+0x2C`)
/// as an 8-byte element to the `hkArray` (data, size, capacity word), growing it through
/// `00c90940` (with the allocator, the array and 8) first whenever the size equals the
/// capacity.
pub fn hk_cpu_job_thread_pool_append_timer_data(
    e: &mut Engine,
    this: Ptr,
    array: Ptr,
    allocator: u32,
) {
    let pool = this.addr();
    let array = array.addr();
    if e.mem.i32(pool + THREAD_COUNT) > 0 {
        let mut index = 0u32;
        loop {
            if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
                e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
            }
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            let element = data.wrapping_add(size.wrapping_mul(8));
            e.mem.set_u32(array + 4, size.wrapping_add(1));
            let r = record(pool, index);
            let begin = e.mem.u32(r + 0x28);
            e.mem.set_u32(element, begin);
            let current = e.mem.u32(r + 0x2c);
            e.mem.set_u32(element + 4, current);
            index += 1;
            if index as i32 >= e.mem.i32(pool + THREAD_COUNT) {
                break;
            }
        }
    }
}

// Translated from 00d2d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPool::setNumThreads` (Xbox PDB; `this` = the pool, then the wanted count,
/// `RET 4`): limits the wanted count to 6, then adds threads (`00d2d240`) while there are too
/// few and stops threads (`00d2ce50`) while there are too many.
pub fn hk_cpu_job_thread_pool_set_num_threads(e: &mut Engine, this: Ptr, wanted: i32) {
    let pool = this.addr();
    let wanted = wanted.min(MAX_THREADS as i32);
    if e.mem.i32(pool + THREAD_COUNT) < wanted {
        loop {
            fn_00d2d240(e, this);
            if e.mem.i32(pool + THREAD_COUNT) >= wanted {
                break;
            }
        }
    }
    if e.mem.i32(pool + THREAD_COUNT) > wanted {
        loop {
            fn_00d2ce50(e, this);
            if e.mem.i32(pool + THREAD_COUNT) <= wanted {
                break;
            }
        }
    }
}

// Translated from 00d2d380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuJobThreadPoolCinfo::hkCpuJobThreadPoolCinfo` (Xbox PDB): the creation info's
/// defaults: thread count 1 (`+0`), `0x3D0900` (`+4`), 0 (`+8`, `+0xC`, `+0x10`, `+0x14`),
/// `0x80000000` (`+0x18`) and the pointer `0x010CE7B0` (`+0x1C`). Answers `this`.
pub fn hk_cpu_job_thread_pool_cinfo_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let info = this.addr();
    e.mem.set_u32(info, 1);
    e.mem.set_u32(info + 4, 0x003d_0900);
    e.mem.set_u32(info + 0xc, 0);
    e.mem.set_u32(info + 0x10, 0);
    e.mem.set_u32(info + 0x14, 0);
    e.mem.set_u32(info + 0x18, 0x8000_0000);
    e.mem.set_u32(info + 0x1c, 0x010c_e7b0);
    e.mem.set_u32(info + 8, 0);
    this
}

// Translated from 00d2d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_cf00`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2d3b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_cf00, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_cd80, fn_00d2cd80(Ptr) -> Ptr),
        entry!(0x00d2_cda0, fn_00d2cda0(Ptr) -> Ptr),
        entry!(
            0x00d2_cde0,
            hk_cpu_job_thread_pool_process_all_jobs(Ptr, u32, u32)
        ),
        entry!(0x00d2_ce30, hk_cpu_job_thread_pool_is_processing(Ptr) -> bool),
        entry!(0x00d2_ce40, hk_cpu_job_thread_pool_get_num_threads(Ptr) -> u32),
        entry!(0x00d2_ce50, fn_00d2ce50(Ptr) -> u32),
        entry!(0x00d2_ce90, hk_cpu_job_thread_pool_wait_for_completion(Ptr)),
        entry!(0x00d2_ced0, hk_cpu_job_thread_pool_clear_timer_data(Ptr)),
        entry!(0x00d2_cf00, hk_cpu_job_thread_pool_destruct(Ptr)),
        entry!(0x00d2_cfb0, hk_worker_thread_func(Ptr) -> u32),
        entry!(0x00d2_d110, fn_00d2d110(Ptr, Ptr) -> Ptr),
        entry!(0x00d2_d240, fn_00d2d240(Ptr) -> u32),
        entry!(
            0x00d2_d2c0,
            hk_cpu_job_thread_pool_append_timer_data(Ptr, Ptr, u32)
        ),
        entry!(
            0x00d2_d330,
            hk_cpu_job_thread_pool_set_num_threads(Ptr, i32)
        ),
        entry!(0x00d2_d380, hk_cpu_job_thread_pool_cinfo_construct(Ptr) -> Ptr),
        entry!(0x00d2_d3b0, deleting_destructor_00d2d3b0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const SYSTEM_SLOT_DOUBLE: u32 = 0x0900_0012;
    const SYSTEM_RELEASE_SLOT_DOUBLE: u32 = 0x0900_0013;
    const POOL_BYTES: u32 = 0x150;

    type Call = (u32, Vec<u32>);

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d2d3b0_frees_only_when_bit_zero_is_set() {
        check(0x00d2_d3b0, 0x00d2_cf00);
    }

    /// An engine with a zero-answering double for every outside callee of the pool code, the
    /// two TLS index globals set (7: thread number, 9: monitor stream) and the call log on.
    fn pool_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(THREAD_NUMBER_TLS_INDEX, 7u32);
        e.set_global(MONITOR_TLS_INDEX, 9u32);
        for address in [
            SEMAPHORE_CONSTRUCT,
            SEMAPHORE_DESTRUCT,
            SEMAPHORE_ACQUIRE,
            SEMAPHORE_RELEASE,
            THREAD_CONSTRUCT,
            THREAD_CLOSE,
            THREAD_CREATE,
            THREAD_DESTRUCT,
            SET_LOCK_MODE,
            JOB_QUEUE_PROCESS,
            ARRAY_RESERVE_MORE,
            ROUTER_CONSTRUCT,
            MEMORY_SYSTEM,
            MEMORY_INIT_THREAD,
            MEMORY_QUIT_THREAD,
            MONITOR_RESIZE,
            MONITOR_RESET,
            API_TLS_GET_VALUE,
            API_TLS_SET_VALUE,
            API_GET_SYSTEM_INFO,
            API_GET_CURRENT_THREAD,
            API_SET_THREAD_IDEAL_PROCESSOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(Vec::new());
        e
    }

    /// `GetSystemInfo` double that reports `processors` processors.
    fn report_processors(e: &mut Engine, processors: u32) {
        e.register_double(API_GET_SYSTEM_INFO, move |e, a| {
            e.mem.set_u32(a[0] + 0x14, processors);
            Ret::default()
        });
    }

    /// The logged calls to `addresses`, in order.
    fn calls_to(e: &Engine, addresses: &[u32]) -> Vec<Call> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addresses.contains(a))
            .cloned()
            .collect()
    }

    /// The argument words of every logged call to `address`.
    fn logged(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        calls_to(e, &[address])
            .into_iter()
            .map(|(_, words)| words)
            .collect()
    }

    /// A pool block with `count` threads.
    fn pool_with(e: &mut Engine, count: u32) -> u32 {
        let pool = e.mem.alloc(POOL_BYTES);
        e.mem.set_u32(pool + THREAD_COUNT, count);
        pool
    }

    #[test]
    fn cinfo_construct_stores_the_defaults() {
        let mut e = pool_engine();
        let info = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(info + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00d2_d380, &args![info]).u32(), info);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(info + i * 4)).collect();
        assert_eq!(
            words,
            vec![1, 0x003d_0900, 0, 0, 0, 0, 0x8000_0000, 0x010c_e7b0]
        );
    }

    #[test]
    fn shared_semaphore_construct_uses_zero_and_six() {
        let mut e = pool_engine();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d2_cd80, &args![this]).u32(), this);
        assert_eq!(logged(&e, SEMAPHORE_CONSTRUCT), vec![vec![this + 4, 0, 6]]);
    }

    #[test]
    fn record_init_clears_the_record_but_not_the_clear_timer_byte() {
        let mut e = pool_engine();
        let r = e.mem.alloc(0x30);
        e.mem.set_u32(r + 0x28, 5);
        e.mem.set_u32(r + 0x2c, 6);
        e.mem.set_u8(r + 0x20, 1);
        e.mem.set_u8(r + 0x21, 0x55);
        e.mem.set_u32(r + 0x18, 3);
        e.mem.set_u32(r, 9);
        assert_eq!(e.call(0x00d2_cda0, &args![r]).u32(), r);
        assert_eq!(e.mem.u32(r + 0x28), 0);
        assert_eq!(e.mem.u32(r + 0x2c), 0);
        assert_eq!(e.mem.u8(r + 0x20), 0);
        assert_eq!(e.mem.u8(r + 0x21), 0x55);
        assert_eq!(e.mem.u32(r + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(r), 0);
        assert_eq!(logged(&e, THREAD_CONSTRUCT), vec![vec![r + 8]]);
        assert_eq!(logged(&e, SEMAPHORE_CONSTRUCT), vec![vec![r + 0x24, 0, 1]]);
    }

    #[test]
    fn process_all_jobs_releases_every_thread_last_first() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 3);
        e.call(0x00d2_cde0, &args![pool, 0x1234u32, 0x9999u32]);
        assert_eq!(e.mem.u32(pool + 0x128), 0x1234);
        assert_eq!(e.mem.u8(pool + 0x13c), 1);
        assert_eq!(
            logged(&e, SEMAPHORE_RELEASE),
            vec![
                vec![pool + 8 + 0x60 + 0x24, 1],
                vec![pool + 8 + 0x30 + 0x24, 1],
                vec![pool + 8 + 0x24, 1],
            ]
        );
    }

    #[test]
    fn process_all_jobs_without_threads_only_sets_the_state() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 0);
        e.call(0x00d2_cde0, &args![pool, 0x77u32, 0u32]);
        assert_eq!(e.mem.u32(pool + 0x128), 0x77);
        assert_eq!(e.mem.u8(pool + 0x13c), 1);
        assert!(logged(&e, SEMAPHORE_RELEASE).is_empty());
    }

    #[test]
    fn is_processing_tests_the_whole_byte() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 0);
        assert_eq!(e.call(0x00d2_ce30, &args![pool]).u32(), 0);
        e.mem.set_u8(pool + 0x13c, 0x80);
        assert_eq!(e.call(0x00d2_ce30, &args![pool]).u32(), 1);
    }

    #[test]
    fn get_num_threads_answers_the_count() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 4);
        assert_eq!(e.call(0x00d2_ce40, &args![pool]).u32(), 4);
    }

    #[test]
    fn stop_last_thread_signals_waits_and_closes_it() {
        let mut e = pool_engine();
        e.register(THREAD_CLOSE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let pool = pool_with(&mut e, 3);
        let last = pool + 8 + 0x60;
        assert_eq!(e.call(0x00d2_ce50, &args![pool]).u32(), 0x77);
        assert_eq!(e.mem.u32(pool + 0x130), 2);
        assert_eq!(e.mem.u8(last + 0x20), 1);
        assert_eq!(
            calls_to(&e, &[SEMAPHORE_RELEASE, SEMAPHORE_ACQUIRE, THREAD_CLOSE]),
            vec![
                (SEMAPHORE_RELEASE, vec![last + 0x24, 1]),
                (SEMAPHORE_ACQUIRE, vec![pool + 0x12c]),
                (THREAD_CLOSE, vec![last + 8]),
            ]
        );
    }

    #[test]
    fn wait_for_completion_acquires_once_per_thread_and_clears_the_byte() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 3);
        e.call(0x00d2_ce90, &args![pool]);
        assert!(logged(&e, SEMAPHORE_ACQUIRE).is_empty());
        e.mem.set_u8(pool + 0x13c, 1);
        e.call(0x00d2_ce90, &args![pool]);
        assert_eq!(logged(&e, SEMAPHORE_ACQUIRE), vec![vec![pool + 0x12c]; 3]);
        assert_eq!(e.mem.u8(pool + 0x13c), 0);
        // Without threads the byte is still cleared.
        let empty = pool_with(&mut e, 0);
        e.mem.set_u8(empty + 0x13c, 1);
        e.call(0x00d2_ce90, &args![empty]);
        assert_eq!(logged(&e, SEMAPHORE_ACQUIRE).len(), 3);
        assert_eq!(e.mem.u8(empty + 0x13c), 0);
    }

    #[test]
    fn clear_timer_data_copies_begin_to_current_and_flags_the_threads() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 2);
        let first = pool + 8;
        let second = pool + 8 + 0x30;
        let third = pool + 8 + 0x60;
        e.mem.set_u32(first + 0x28, 0x10);
        e.mem.set_u32(second + 0x28, 0x20);
        e.mem.set_u32(third + 0x28, 0x30);
        e.call(0x00d2_ced0, &args![pool]);
        assert_eq!(e.mem.u32(first + 0x2c), 0x10);
        assert_eq!(e.mem.u32(second + 0x2c), 0x20);
        assert_eq!(e.mem.u32(third + 0x2c), 0);
        assert_eq!(e.mem.u8(first + 0x21), 1);
        assert_eq!(e.mem.u8(second + 0x21), 1);
        assert_eq!(e.mem.u8(third + 0x21), 0);
    }

    #[test]
    fn destructor_stops_the_threads_and_tears_the_records_down() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 2);
        e.mem.set_u8(pool + 0x13c, 1);
        e.call(0x00d2_cf00, &args![pool]);
        let shared = pool + 0x12c;
        let record_semaphore = |i: u32| pool + 8 + i * 0x30 + 0x24;
        let mut expected: Vec<Call> = vec![
            (SEMAPHORE_ACQUIRE, vec![shared]),
            (SEMAPHORE_ACQUIRE, vec![shared]),
            (SEMAPHORE_RELEASE, vec![record_semaphore(0), 1]),
            (SEMAPHORE_RELEASE, vec![record_semaphore(1), 1]),
            (SEMAPHORE_ACQUIRE, vec![shared]),
            (SEMAPHORE_ACQUIRE, vec![shared]),
            (SET_LOCK_MODE, vec![0]),
            (SEMAPHORE_DESTRUCT, vec![shared]),
        ];
        for i in (0..6).rev() {
            expected.push((SEMAPHORE_DESTRUCT, vec![record_semaphore(i)]));
            expected.push((THREAD_DESTRUCT, vec![pool + 8 + i * 0x30 + 8]));
        }
        assert_eq!(
            calls_to(
                &e,
                &[
                    SEMAPHORE_ACQUIRE,
                    SEMAPHORE_RELEASE,
                    SET_LOCK_MODE,
                    SEMAPHORE_DESTRUCT,
                    THREAD_DESTRUCT
                ]
            ),
            expected
        );
        assert_eq!(e.mem.u32(pool), 0x0102_e388);
        assert_eq!(e.mem.u8(pool + 0x13c), 0);
        assert_eq!(e.mem.u8(pool + 8 + 0x20), 1);
        assert_eq!(e.mem.u8(pool + 8 + 0x30 + 0x20), 1);
        assert_eq!(e.mem.u8(pool + 8 + 0x60 + 0x20), 0);
    }

    /// The addresses of the calls a worker thread makes, but not the TLS reads.
    const WORKER_CALLS: [u32; 14] = [
        API_TLS_SET_VALUE,
        API_GET_CURRENT_THREAD,
        API_SET_THREAD_IDEAL_PROCESSOR,
        ROUTER_CONSTRUCT,
        MEMORY_SYSTEM,
        SYSTEM_SLOT_DOUBLE,
        MEMORY_INIT_THREAD,
        MONITOR_RESIZE,
        SEMAPHORE_ACQUIRE,
        MONITOR_RESET,
        JOB_QUEUE_PROCESS,
        SEMAPHORE_RELEASE,
        MEMORY_QUIT_THREAD,
        SYSTEM_RELEASE_SLOT_DOUBLE,
    ];

    /// A worker's surroundings: the pool (the record is its first), a job queue at `0x2222`
    /// and a monitor stream (begin `0x100`, current `0x180`) that `TlsGetValue(9)` answers.
    /// Answers `(pool, record, stream, memory system)`.
    fn worker_setup(e: &mut Engine) -> (u32, u32, u32, u32) {
        let pool = pool_with(e, 1);
        let r = pool + 8;
        e.mem.set_u32(r, pool + 0x128);
        e.mem.set_u32(r + 0x18, 1);
        e.mem.set_u32(r + 0x1c, 3);
        e.mem.set_u32(pool + 0x128, 0x2222);
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream, 0x100);
        e.mem.set_u32(stream + 4, 0x180);
        e.register_double(API_TLS_GET_VALUE, move |_, a| Ret {
            eax: if a[0] == 9 { stream } else { 0 },
            ..Ret::default()
        });
        e.register(API_GET_CURRENT_THREAD, |_, _| 0x55u32.into_ret());
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SYSTEM_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x10, SYSTEM_RELEASE_SLOT_DOUBLE);
        let system = e.mem.alloc(8);
        e.mem.set_u32(system, vtable);
        e.register_double(MEMORY_SYSTEM, move |_, _| Ret {
            eax: system,
            ..Ret::default()
        });
        e.register(SYSTEM_SLOT_DOUBLE, |_, _| Ret::default());
        e.register(SYSTEM_RELEASE_SLOT_DOUBLE, |_, _| Ret::default());
        (pool, r, stream, system)
    }

    #[test]
    fn worker_thread_runs_one_round_with_a_monitor_reset() {
        let mut e = pool_engine();
        let (pool, r, stream, system) = worker_setup(&mut e);
        e.mem.set_u8(r + 0x21, 1);
        e.register_double(MONITOR_RESET, move |e, _| {
            e.mem.set_u32(stream + 4, 0x200);
            Ret::default()
        });
        e.register_double(JOB_QUEUE_PROCESS, move |e, _| {
            e.mem.set_u8(r + 0x20, 1);
            let current = e.mem.u32(stream + 4);
            e.mem.set_u32(stream + 4, current + 0x10);
            Ret::default()
        });
        assert_eq!(e.call(0x00d2_cfb0, &args![r]).u32(), 0);
        let router = logged(&e, ROUTER_CONSTRUCT)[0][0];
        let shared = pool + 0x128;
        let expected: Vec<Call> = vec![
            (API_TLS_SET_VALUE, vec![7, 1]),
            (API_GET_CURRENT_THREAD, vec![]),
            (API_SET_THREAD_IDEAL_PROCESSOR, vec![0x55, 3]),
            (ROUTER_CONSTRUCT, vec![router]),
            (MEMORY_SYSTEM, vec![]),
            (SYSTEM_SLOT_DOUBLE, vec![system, router, 0x010c_e79c, 3]),
            (MEMORY_INIT_THREAD, vec![router]),
            (SEMAPHORE_ACQUIRE, vec![r + 0x24]),
            (MONITOR_RESET, vec![stream]),
            (JOB_QUEUE_PROCESS, vec![0x2222]),
            (SEMAPHORE_RELEASE, vec![shared + 4, 1]),
            (SEMAPHORE_ACQUIRE, vec![r + 0x24]),
            (MEMORY_QUIT_THREAD, vec![]),
            (MEMORY_SYSTEM, vec![]),
            (SYSTEM_RELEASE_SLOT_DOUBLE, vec![system, router, 3]),
            (SEMAPHORE_RELEASE, vec![shared + 4, 1]),
        ];
        assert_eq!(calls_to(&e, &WORKER_CALLS), expected);
        assert_eq!(e.mem.u32(r + 0x28), 0x100);
        assert_eq!(e.mem.u32(r + 0x2c), 0x210);
        assert_eq!(e.mem.u8(r + 0x21), 0);
    }

    #[test]
    fn worker_thread_that_is_told_to_stop_at_once_skips_the_loop() {
        let mut e = pool_engine();
        let (pool, r, stream, _) = worker_setup(&mut e);
        e.mem.set_u32(pool + 0x138, 64);
        e.mem.set_u8(r + 0x20, 1);
        assert_eq!(e.call(0x00d2_cfb0, &args![r]).u32(), 0);
        assert_eq!(logged(&e, MONITOR_RESIZE), vec![vec![stream, 64]]);
        assert_eq!(logged(&e, SEMAPHORE_ACQUIRE), vec![vec![r + 0x24]]);
        assert!(logged(&e, JOB_QUEUE_PROCESS).is_empty());
        assert!(logged(&e, MONITOR_RESET).is_empty());
        assert_eq!(
            logged(&e, SEMAPHORE_RELEASE),
            vec![vec![pool + 0x128 + 4, 1]]
        );
        assert_eq!(e.mem.u32(r + 0x28), 0x100);
        assert_eq!(e.mem.u32(r + 0x2c), 0x180);
    }

    /// A creation info: `count` threads, settings `0x1111` (`+4`), stack size `0x2222`
    /// (`+8`), monitor size `0x3333` (`+0xC`), `0x4444` (`+0x1C`), and when `ideal` is given
    /// an array of ideal processors.
    fn creation_info(e: &mut Engine, count: u32, ideal: Option<[u32; 2]>) -> u32 {
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info, count);
        e.mem.set_u32(info + 4, 0x1111);
        e.mem.set_u32(info + 8, 0x2222);
        e.mem.set_u32(info + 0xc, 0x3333);
        e.mem.set_u32(info + 0x1c, 0x4444);
        if let Some(ideal) = ideal {
            let array = e.mem.alloc(8);
            e.mem.set_u32(array, ideal[0]);
            e.mem.set_u32(array + 4, ideal[1]);
            e.mem.set_u32(info + 0x10, array);
            e.mem.set_u32(info + 0x14, 2);
        }
        info
    }

    #[test]
    fn construct_starts_the_threads_with_ideal_processors_from_the_thread_number() {
        let mut e = pool_engine();
        report_processors(&mut e, 4);
        let pool = e.mem.alloc(POOL_BYTES);
        let info = creation_info(&mut e, 2, None);
        e.mem.set_u8(pool + 8 + 0x21, 0x77);
        e.mem.set_u8(pool + 0x13c, 0x66);
        assert_eq!(e.call(0x00d2_d110, &args![pool, info]).u32(), pool);
        assert_eq!(e.mem.u16(pool + 6), 1);
        assert_eq!(e.mem.u32(pool), 0x010c_e774);
        assert_eq!(e.mem.u8(pool + 0x13c), 0);
        assert_eq!(e.mem.u32(pool + 0x140), 0x4444);
        assert_eq!(e.mem.u32(pool + 0x144), 0x2222);
        assert_eq!(e.mem.u32(pool + 0x134), 0x1111);
        assert_eq!(e.mem.u32(pool + 0x138), 0x3333);
        assert_eq!(e.mem.u32(pool + 0x130), 2);
        let first = pool + 8;
        let second = first + 0x30;
        for (r, number, ideal) in [(first, 1, 1), (second, 2, 2)] {
            assert_eq!(e.mem.u32(r), pool + 0x128);
            assert_eq!(e.mem.u32(r + 0x18), number);
            assert_eq!(e.mem.u32(r + 0x1c), ideal);
        }
        assert_eq!(e.mem.u8(first + 0x21), 0);
        for index in 2..6 {
            let r = pool + 8 + index * 0x30;
            assert_eq!(e.mem.u32(r), 0);
            assert_eq!(e.mem.u32(r + 0x18), 0xffff_ffff);
        }
        let creates = vec![
            vec![first + 8, WORKER_THREAD_FUNCTION, first, 0x4444, 0x2222],
            vec![second + 8, WORKER_THREAD_FUNCTION, second, 0x4444, 0x2222],
        ];
        assert_eq!(logged(&e, THREAD_CREATE), creates);
        let mut expected: Vec<Call> = Vec::new();
        for index in 0..6 {
            let r = pool + 8 + index * 0x30;
            expected.push((THREAD_CONSTRUCT, vec![r + 8]));
            expected.push((SEMAPHORE_CONSTRUCT, vec![r + 0x24, 0, 1]));
        }
        expected.push((SEMAPHORE_CONSTRUCT, vec![pool + 0x12c, 0, 6]));
        expected.push((
            API_GET_SYSTEM_INFO,
            vec![logged(&e, API_GET_SYSTEM_INFO)[0][0]],
        ));
        expected.push((THREAD_CREATE, creates[0].clone()));
        expected.push((THREAD_CREATE, creates[1].clone()));
        expected.push((SET_LOCK_MODE, vec![1]));
        assert_eq!(
            calls_to(
                &e,
                &[
                    THREAD_CONSTRUCT,
                    SEMAPHORE_CONSTRUCT,
                    API_GET_SYSTEM_INFO,
                    THREAD_CREATE,
                    SET_LOCK_MODE
                ]
            ),
            expected
        );
    }

    #[test]
    fn construct_takes_the_ideal_processors_from_the_infos_array_when_it_has_one() {
        let mut e = pool_engine();
        report_processors(&mut e, 4);
        let pool = e.mem.alloc(POOL_BYTES);
        let info = creation_info(&mut e, 2, Some([7, 9]));
        e.call(0x00d2_d110, &args![pool, info]);
        assert_eq!(e.mem.u32(pool + 8 + 0x1c), 7);
        assert_eq!(e.mem.u32(pool + 8 + 0x30 + 0x1c), 9);
    }

    #[test]
    fn construct_limits_the_thread_count() {
        for (wanted, started) in [(0u32, 0u32), (6, 6), (7, 5), (9, 5)] {
            let mut e = pool_engine();
            report_processors(&mut e, 2);
            let pool = e.mem.alloc(POOL_BYTES);
            let info = creation_info(&mut e, wanted, None);
            e.call(0x00d2_d110, &args![pool, info]);
            assert_eq!(e.mem.u32(pool + 0x130), started);
            assert_eq!(logged(&e, THREAD_CREATE).len(), started as usize);
            assert_eq!(logged(&e, API_GET_SYSTEM_INFO).len(), 1);
            assert_eq!(logged(&e, SET_LOCK_MODE), vec![vec![1]]);
        }
    }

    #[test]
    fn add_thread_sets_up_the_next_record_and_raises_the_count() {
        for (processors, ideal) in [(3u32, 0u32), (2, 1)] {
            let mut e = pool_engine();
            report_processors(&mut e, processors);
            e.register(THREAD_CREATE, |_, _| 1u32.into_ret());
            let pool = pool_with(&mut e, 2);
            e.mem.set_u32(pool + 0x140, 0x4444);
            e.mem.set_u32(pool + 0x144, 0x2222);
            let r = pool + 8 + 0x60;
            e.mem.set_u8(r + 0x20, 9);
            e.mem.set_u8(r + 0x21, 9);
            e.mem.set_u32(r + 0x28, 5);
            e.mem.set_u32(r + 0x2c, 5);
            assert_eq!(e.call(0x00d2_d240, &args![pool]).u32(), 1);
            assert_eq!(e.mem.u32(pool + 0x130), 3);
            assert_eq!(e.mem.u32(r), pool + 0x128);
            assert_eq!(e.mem.u32(r + 0x18), 3);
            assert_eq!(e.mem.u32(r + 0x1c), ideal);
            assert_eq!(e.mem.u8(r + 0x20), 0);
            assert_eq!(e.mem.u8(r + 0x21), 0);
            assert_eq!(e.mem.u32(r + 0x28), 0);
            assert_eq!(e.mem.u32(r + 0x2c), 0);
            assert_eq!(
                logged(&e, THREAD_CREATE),
                vec![vec![r + 8, WORKER_THREAD_FUNCTION, r, 0x4444, 0x2222]]
            );
        }
    }

    #[test]
    fn append_timer_data_adds_one_element_per_thread() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 2);
        e.mem.set_u32(pool + 8 + 0x28, 0x10);
        e.mem.set_u32(pool + 8 + 0x2c, 0x11);
        e.mem.set_u32(pool + 8 + 0x30 + 0x28, 0x20);
        e.mem.set_u32(pool + 8 + 0x30 + 0x2c, 0x21);
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 0x8000_0004);
        e.call(0x00d2_d2c0, &args![pool, array, 0x5000u32]);
        assert_eq!(e.mem.u32(array + 4), 2);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(data + i * 4)).collect();
        assert_eq!(words, vec![0x10, 0x11, 0x20, 0x21]);
        assert!(logged(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn append_timer_data_grows_the_array_when_it_is_full() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 2);
        e.mem.set_u32(pool + 8 + 0x28, 0x10);
        e.mem.set_u32(pool + 8 + 0x2c, 0x11);
        e.mem.set_u32(pool + 8 + 0x30 + 0x28, 0x20);
        e.mem.set_u32(pool + 8 + 0x30 + 0x2c, 0x21);
        let array = e.mem.alloc(12);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0xaa);
        e.mem.set_u32(old + 4, 0xbb);
        e.mem.set_u32(array, old);
        e.mem.set_u32(array + 4, 1);
        // The capacity word carries flag bits in bits 30 and 31; only the low 30 count.
        e.mem.set_u32(array + 8, 0x8000_0001);
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let grown = e.mem.alloc(0x40);
            let size = e.mem.u32(a[1] + 4);
            for i in 0..size * 2 {
                let word = e.mem.u32(e.mem.u32(a[1]) + i * 4);
                e.mem.set_u32(grown + i * 4, word);
            }
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00d2_d2c0, &args![pool, array, 0x5000u32]);
        assert_eq!(logged(&e, ARRAY_RESERVE_MORE), vec![vec![0x5000, array, 8]]);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(data + i * 4)).collect();
        assert_eq!(words, vec![0xaa, 0xbb, 0x10, 0x11, 0x20, 0x21]);
    }

    #[test]
    fn append_timer_data_without_threads_leaves_the_array_alone() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 0);
        let array = e.mem.alloc(12);
        e.call(0x00d2_d2c0, &args![pool, array, 0x5000u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert!(logged(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn set_num_threads_adds_up_to_six() {
        let mut e = pool_engine();
        report_processors(&mut e, 4);
        let pool = pool_with(&mut e, 2);
        e.call(0x00d2_d330, &args![pool, 9u32]);
        assert_eq!(e.mem.u32(pool + 0x130), 6);
        let started: Vec<u32> = logged(&e, THREAD_CREATE).iter().map(|w| w[2]).collect();
        let expected: Vec<u32> = (2..6).map(|i| pool + 8 + i * 0x30).collect();
        assert_eq!(started, expected);
        assert!(logged(&e, THREAD_CLOSE).is_empty());
    }

    #[test]
    fn set_num_threads_stops_the_extra_threads_last_first() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 3);
        e.call(0x00d2_d330, &args![pool, 1u32]);
        assert_eq!(e.mem.u32(pool + 0x130), 1);
        assert_eq!(
            logged(&e, THREAD_CLOSE),
            vec![vec![pool + 8 + 0x60 + 8], vec![pool + 8 + 0x30 + 8]]
        );
        assert_eq!(e.mem.u8(pool + 8 + 0x60 + 0x20), 1);
        assert_eq!(e.mem.u8(pool + 8 + 0x30 + 0x20), 1);
        assert_eq!(e.mem.u8(pool + 8 + 0x20), 0);
        assert!(logged(&e, THREAD_CREATE).is_empty());
    }

    #[test]
    fn set_num_threads_with_the_current_count_does_nothing() {
        let mut e = pool_engine();
        let pool = pool_with(&mut e, 3);
        e.call(0x00d2_d330, &args![pool, 3u32]);
        assert_eq!(e.mem.u32(pool + 0x130), 3);
        assert!(logged(&e, THREAD_CREATE).is_empty());
        assert!(logged(&e, THREAD_CLOSE).is_empty());
    }
}
