//! `BSMain/iomanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const SLEEP: u32 = 0x00fd_f104;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;

// The Xbox PDB describes IOManager as 0x5C. The PC constructor 00c3da50
// initializes through +0x80, so its live object is at least 0x84. Keep PC
// accesses as verified raw offsets until the divergent layout is resolved.

fn word(e: &Engine, base: Ptr, offset: u32) -> u32 {
    e.mem.u32(base.addr().wrapping_add(offset))
}

fn set_word(e: &mut Engine, base: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(base.addr().wrapping_add(offset), value);
}

fn count_queued(e: &mut Engine, this: Ptr) -> u32 {
    let queue = word(e, this, 0x64);
    e.vcall(queue, 0x0c, &[]).u32()
}

fn external_count(e: &mut Engine, this: Ptr) -> u32 {
    let target = word(e, this, 0x6c);
    if target == 0 {
        0
    } else {
        e.call(target, &[]).u32()
    }
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![0x011f_6238u32, size]).u32()
}

fn array_bytes(count: u32) -> u32 {
    let bytes = count as u64 * 4;
    (bytes as u32) | if bytes > u32::MAX as u64 { u32::MAX } else { 0 }
}

// Translated from 00456520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOManager::LoadQueuedPriority` (Xbox PDB): load through priority 5.
pub fn io_manager_load_queued_priority(e: &mut Engine, this: Ptr) {
    fn_00c3dfa0(e, this, 5);
}

// Translated from 005a8040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at receiver +0x54. The engine map's `IOManager::Pause`
/// candidate (Xbox PDB) is unconfirmed for this PC body.
pub fn fn_005a8040(e: &mut Engine, this: Ptr, value: u32) {
    set_word(e, this, 0x54, value);
}

// Translated from 00c3db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys an IO manager's post-process queue and then its task manager base.
/// The game's SEH frame is omitted.
// Translation incomplete: 00c3db60 - SEH unwinding is not modeled.
pub fn fn_00c3db60(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_1604);
    let queue = word(e, this, 0x64);
    if queue != 0 {
        e.vcall(queue, 0, &args![1u32]);
    }
    fn_00c3fa70(e, this);
}

// Translated from 00c3df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOManager::RequeueTask` (Xbox PDB): replaces the priority byte in a task's 64-bit key.
pub fn io_manager_requeue_task(e: &mut Engine, this: Ptr, task: Ptr, priority: i32) {
    let low = word(e, task, 0x10);
    let high = word(e, task, 0x14);
    let key = (((high as u64) << 32) | low as u64) & 0xffff_ffff_ff00_ffff;
    let shifted = ((priority as i64) << 16) as u64;
    let result = key.wrapping_add(shifted);
    fn_00c3e690(e, this, task, result as u32, (result >> 32) as u32);
}

// Translated from 00c3dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains queued IO work up to the requested priority and reports percentage progress.
pub fn fn_00c3dfa0(e: &mut Engine, this: Ptr, priority: i32) {
    if word(e, this, 0x68) == 7 {
        return;
    }
    e.mem.set_u8(0x011a_f70c, 0);
    if word(e, this, 0x18) == 0 && count_queued(e, this) == 0 && external_count(e, this) == 0 {
        set_word(e, this, 0x68, 6);
        e.mem.set_u8(0x011a_f70c, 1);
        return;
    }
    set_word(e, this, 0x68, priority as u32);
    fn_00c3e1b0(e, this, priority, 0);
    let bucket_limit = (priority as u32).wrapping_mul(4).wrapping_add(3);
    let total = count_queued(e, this)
        .wrapping_add(external_count(e, this))
        .wrapping_add(fn_00c3e860(e, this, bucket_limit));
    let mut remaining = total;
    let mut processed = false;
    while remaining != 0 || !processed {
        // 00c3e0a9: both counts loaded exactly (`FILD qword`), `FDIVP` and `FMUL` by the
        // double at 01017a40 rounded to the thread's precision control, `FSTP` to a float.
        let progress_scale: f64 = e.global(0x0101_7a40);
        let fraction = x87_div(f64::from(total.wrapping_sub(remaining)), f64::from(total));
        let progress = x87_store32(x87_mul(fraction, progress_scale));
        let progress_callback = word(e, this, 0x78);
        if progress_callback != 0 {
            e.call(progress_callback, &args![progress]);
        }
        let previous_count = word(e, this, 0x18);
        if count_queued(e, this) < previous_count {
            e.call(SLEEP, &args![50u32]);
        }
        processed = e.call(0x00c3_dbf0, &args![this]).bool();
        remaining = count_queued(e, this)
            .wrapping_add(external_count(e, this))
            .wrapping_add(fn_00c3e860(e, this, bucket_limit));
    }
    set_word(e, this, 0x68, 6);
    let progress_callback = word(e, this, 0x78);
    if progress_callback != 0 {
        e.call(progress_callback, &args![e.global::<f32>(0x0101_6410)]);
    }
    e.mem.set_u8(0x011a_f70c, 1);
}

// Translated from 00c3e1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs queued tasks from a lower priority through the requested priority.
/// The game's SEH frame and an unreachable cleanup branch are omitted.
// Translation incomplete: 00c3e1b0 - SEH unwinding is not modeled.
pub fn fn_00c3e1b0(e: &mut Engine, this: Ptr, priority: i32, lower_bound: i32) {
    if priority <= lower_bound {
        return;
    }
    fn_00c3e310(e, this);
    e.with_stack(0x50, |e, local| {
        set_word(e, local, 0, 0x010c_1664);
        set_word(e, local, 8, 0);
        set_word(e, local, 0x10, 0);
        set_word(e, local, 0x14, 0);
        e.mem.set_u8(local.addr() + 0x18, 0);
        e.call(
            0x00c4_2060,
            &args![local, (lower_bound as u32).wrapping_mul(4).wrapping_add(4)],
        );
        while e.mem.u8(local.addr() + 0x18) & 2 == 0 {
            let key = local.addr() + 0x30;
            let task_ptr = local.addr() + 0x3c;
            e.mem.set_u32(key, 0);
            e.mem.set_u32(key + 4, 0);
            e.mem.set_u32(task_ptr, 0);
            let found = e
                .call(0x00c4_0e70, &args![this, local, key, task_ptr, 1u32])
                .bool();
            if found {
                let task_priority = (e.mem.u32(key) >> 16) & 0xff;
                if task_priority as i32 > priority {
                    let task = e.mem.u32(task_ptr);
                    if task != 0 {
                        e.call(0x0044_dd60, &args![task]);
                    }
                    break;
                }
                let task = e.mem.u32(task_ptr);
                e.vcall(task, 0x1c, &args![lower_bound]);
            }
            let task = e.mem.u32(task_ptr);
            if task != 0 {
                e.call(0x0044_dd60, &args![task]);
            }
        }
    });
    fn_00c3e340(e, this);
}

// Translated from 00c3e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries each queue until one accepts an item, then notifies the owner.
pub fn fn_00c3e420(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let mut index = 0;
    while index < word(e, this, 8) {
        let queues = word(e, this, 4);
        let queue = Ptr::new(e.mem.u32(queues.wrapping_add(index.wrapping_mul(4))));
        if fn_00c3e490(e, queue, item) {
            e.vcall(this.addr(), 8, &[]);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c3e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Acquires a queue activity token, resolves the item, and attempts insertion.
pub fn fn_00c3e490(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    e.call(0x0040_fbf0, &args![this.addr().wrapping_add(0x20), 0u32]);
    let target = e.call(0x0044_9f80, &args![this]).ptr();
    let accepted = fn_00c3ea60(e, target, item);
    let count = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, count);
    if count == 0 {
        set_word(e, this, 0x20, 0);
    }
    accepted
}

fn assign_ref(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old != value {
        if old != 0 {
            e.call(0x0044_dd60, &args![old]);
        }
        e.mem.set_u32(holder, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 8]);
        }
    }
}

fn assign_ref_from(e: &mut Engine, holder: u32, source: u32) {
    let old = e.mem.u32(holder);
    if old != e.mem.u32(source) {
        if old != 0 {
            e.call(0x0044_dd60, &args![old]);
        }
        let new_value = e.mem.u32(source);
        e.mem.set_u32(holder, new_value);
        let stored = e.mem.u32(holder);
        if stored != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![stored + 8]);
        }
    }
}

// Translated from 00c3ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes one node from a lock-free IO task queue into an owning pointer.
pub fn fn_00c3ea60(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    loop {
        let queue = word(e, this, 0);
        let head = e.mem.u32(queue + 4);
        let hazard_head = word(e, this, 4);
        e.mem.set_u32(hazard_head, head);
        if head != e.mem.u32(word(e, this, 0) + 4) {
            continue;
        }
        let tail = e.mem.u32(word(e, this, 0) + 8);
        let next = e.mem.u32(head);
        let hazard_next = word(e, this, 8);
        e.mem.set_u32(hazard_next, next);
        if head != e.mem.u32(word(e, this, 0) + 4) {
            continue;
        }
        if next == 0 {
            e.mem.set_u32(word(e, this, 4), 0);
            assign_ref(e, item.addr(), 0);
            return false;
        }
        if head == tail {
            e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![word(e, this, 0) + 8, next, tail],
            );
            continue;
        }
        let previous = e.mem.u32(item.addr());
        if previous != e.mem.u32(next + 4) {
            if previous != 0 {
                e.call(0x0044_dd60, &args![previous]);
            }
            let replacement = e.mem.u32(next + 4);
            e.mem.set_u32(item.addr(), replacement);
            let retained = e.mem.u32(item.addr());
            if retained != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![retained + 8]);
            }
        }
        if e.call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![word(e, this, 0) + 4, next, head],
        )
        .u32()
            != head
        {
            continue;
        }
        e.vcall(word(e, this, 0), 0x0c, &[]);
        assign_ref(e, next + 4, 0);
        e.mem.set_u32(word(e, this, 4), 0);
        e.mem.set_u32(word(e, this, 8), 0);
        fn_00c3f090(e, this, Ptr::new(head));
        return true;
    }
}

// Translated from 00c3f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a removed queue node to the interface's deferred deletion list.
pub fn fn_00c3f090(e: &mut Engine, this: Ptr, node: Ptr) {
    assign_ref(e, node.addr() + 4, 0);
    set_word(e, node, 4, word(e, this, 0x10));
    set_word(e, this, 0x10, node.addr());
    let count = word(e, this, 0x0c).wrapping_add(1);
    set_word(e, this, 0x0c, count);
    let queue = word(e, this, 0);
    if count == e.mem.u32(queue + 0x0c) {
        e.call(0x00c4_2b80, &args![this]);
    }
}

// Translated from 00c3e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces a task key if its state is idle or can be locked from state 1/2.
pub fn fn_00c3e690(e: &mut Engine, this: Ptr, task: Ptr, low: u32, high: u32) -> bool {
    let state = word(e, task, 0x0c) as i32;
    if state == 0 {
        set_word(e, task, 0x10, low);
        set_word(e, task, 0x14, high);
        return true;
    }
    if state >= 3
        || e.call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![task.addr() + 0x0c, 2u32, state],
        )
        .i32()
            != state
    {
        return false;
    }
    e.call(0x0044_e050, &args![this, task]);
    set_word(e, task, 0x10, low);
    set_word(e, task, 0x14, high);
    if e.call(
        INTERLOCKED_COMPARE_EXCHANGE,
        &args![task.addr() + 0x0c, 0u32, 2u32],
    )
    .u32()
        == 2
    {
        e.vcall(this.addr(), 0x48, &args![task]);
    }
    true
}

// Translated from 00c3e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the IO manager's base task manager, bucket counts, and worker threads.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3e4f0 - SEH unwinding is not modeled.
pub fn fn_00c3e4f0(
    e: &mut Engine,
    this: Ptr,
    bucket_count: u32,
    thread_count: u32,
    capacity: u32,
) -> Ptr {
    fn_00c3ec80(
        e,
        this,
        bucket_count.wrapping_add(thread_count),
        capacity,
        12,
    );
    set_word(e, this, 0, 0x010c_167c);
    e.mem.set_u8(this.addr() + 0x40, 0);
    set_word(e, this, 0x44, 0);
    set_word(e, this, 0x48, 0);
    set_word(e, this, 0x4c, thread_count);
    let counts = allocate(e, array_bytes(capacity));
    set_word(e, this, 0x54, counts);
    e.call(0x00ec_61c0, &args![counts, 0u32, capacity.wrapping_mul(4)]);
    set_word(e, this, 0x58, 0);
    let threads = allocate(e, array_bytes(thread_count));
    set_word(e, this, 0x50, threads);
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let storage = allocate(e, 0x34);
        let thread = if storage == 0 {
            0
        } else {
            bs_task_manager_thread_construct(e, Ptr::new(storage), this, index.wrapping_add(2))
                .addr()
        };
        let current_threads = word(e, this, 0x50);
        e.mem
            .set_u32(current_threads.wrapping_add(index.wrapping_mul(4)), thread);
        let current_thread = e
            .mem
            .u32(current_threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x00c4_2f50, &args![current_thread]);
        index = index.wrapping_add(1);
    }
    this
}

// Translated from 00c3e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a priority queue with one lock-free queue interface per bucket.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3e8b0 - SEH unwinding is not modeled.
pub fn fn_00c3e8b0(e: &mut Engine, this: Ptr, count: u32, node_limit: u32, queue: Ptr) -> Ptr {
    set_word(e, this, 0, 0x010c_16dc);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x0c, 0);
    set_word(e, this, 8, count);
    let slots = allocate(e, array_bytes(count));
    set_word(e, this, 4, slots);
    let mut index = 0;
    while index < word(e, this, 8) {
        let storage = allocate(e, 0x40);
        let interface = if storage == 0 {
            0
        } else {
            fn_00c3eeb0(e, Ptr::new(storage), node_limit, queue).addr()
        };
        let current_slots = word(e, this, 4);
        e.mem
            .set_u32(current_slots.wrapping_add(index.wrapping_mul(4)), interface);
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the base lock-free task map and its per-thread interface manager.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3ec80 - SEH unwinding is not modeled.
pub fn fn_00c3ec80(
    e: &mut Engine,
    this: Ptr,
    bucket_count: u32,
    capacity: u32,
    batch_size: u32,
) -> Ptr {
    set_word(e, this, 0, 0x010c_16f4);
    set_word(e, this, 0x20, 0);
    set_word(e, this, 0x24, 0);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x18, 0);
    set_word(e, this, 8, capacity);
    let referenced = allocate(e, array_bytes(capacity));
    if referenced != 0 {
        let mut remaining = capacity;
        let mut address = referenced;
        loop {
            remaining = remaining.wrapping_sub(1);
            if (remaining as i32) < 0 {
                break;
            }
            e.mem.set_u32(address, 0);
            address = address.wrapping_add(4);
        }
    }
    set_word(e, this, 0x0c, referenced);
    let buckets = allocate(e, array_bytes(bucket_count.wrapping_mul(3)));
    set_word(e, this, 4, buckets);
    set_word(e, this, 0x10, batch_size);
    let storage = allocate(e, 0x10);
    let interfaces = if storage == 0 {
        0
    } else {
        e.call(0x00c4_2180, &args![storage, bucket_count]).u32()
    };
    set_word(e, this, 0x14, interfaces);
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs one lock-free queue interface and its sentinel and retired-node storage.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3eeb0 - SEH unwinding is not modeled.
pub fn fn_00c3eeb0(e: &mut Engine, this: Ptr, node_limit: u32, queue: Ptr) -> Ptr {
    set_word(e, this, 0, 0x010c_1760);
    set_word(e, this, 0x20, 0);
    set_word(e, this, 0x24, 0);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x18, 0);
    let sentinel = allocate(e, 8);
    if sentinel != 0 {
        e.mem.set_u32(sentinel, 0);
        e.mem.set_u32(sentinel + 4, 0);
    }
    set_word(e, this, 4, sentinel);
    set_word(e, this, 8, sentinel);
    set_word(e, this, 0x0c, queue.addr());
    let retired = allocate(e, array_bytes(node_limit.wrapping_mul(2)));
    set_word(e, this, 0x10, retired);
    let storage = allocate(e, 0x10);
    let manager = if storage == 0 {
        0
    } else {
        e.call(0x00c4_2180, &args![storage, node_limit]).u32()
    };
    set_word(e, this, 0x14, manager);
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses each task thread on the first nested pause request.
pub fn fn_00c3e750(e: &mut Engine, this: Ptr) -> bool {
    e.mem.set_u8(0x0120_2df0, 0);
    if e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x48])
        .u32()
        != 1
    {
        return false;
    }
    while word(e, this, 0x44) != 0 {
        e.call(SLEEP, &args![1u32]);
    }
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x0044_6f70, &args![thread]);
        e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x44]);
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c3e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes every task thread when the nested pause count reaches zero.
pub fn fn_00c3e7d0(e: &mut Engine, this: Ptr) -> bool {
    if word(e, this, 0x48) == 0
        || e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x48])
            .u32()
            != 0
    {
        return false;
    }
    while word(e, this, 0x44) != word(e, this, 0x4c) {
        e.call(SLEEP, &args![1u32]);
    }
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x0044_6ff0, &args![thread]);
        e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x44]);
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c3e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the task threads and fires the callback only when the pause count becomes one.
pub fn fn_00c3e310(e: &mut Engine, this: Ptr) {
    if fn_00c3e750(e, this) && word(e, this, 0x7c) != 0 {
        let target = word(e, this, 0x7c);
        e.call(target, &[]);
    }
}

// Translated from 00c3e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes the task threads and fires the callback only when the pause count becomes zero.
pub fn fn_00c3e340(e: &mut Engine, this: Ptr) {
    if fn_00c3e7d0(e, this) && word(e, this, 0x80) != 0 {
        let target = word(e, this, 0x80);
        e.call(target, &[]);
    }
}

// Translated from 00c3e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the task manager and clears its queue.
pub fn fn_00c3e380(e: &mut Engine, this: Ptr) {
    fn_00c3fce0(e, this);
    let queue = Ptr::new(word(e, this, 0x64));
    fn_00c3ea10(e, queue);
}

// Translated from 00c3e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes a queue bucket from two bytes of a 64-bit key.
pub fn fn_00c3e3a0(_e: &mut Engine, _this: Ptr, low: u32, high: u32) -> i32 {
    let bucket = (low >> 16) & 0xff;
    let category = (((high >> 24) & 0xff) + 4) % 20;
    bucket.wrapping_mul(category) as i32
}

// Translated from 00c3e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the IO manager: destroy, optionally free, return this.
pub fn fn_00c3e3f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c3db60(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c3e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sum queue bucket counts from zero through the supplied inclusive index.
pub fn fn_00c3e860(e: &mut Engine, this: Ptr, last: u32) -> u32 {
    let mut sum = 0u32;
    let mut index = 0u32;
    loop {
        if index > last {
            break;
        }
        let counts = word(e, this, 0x54);
        sum = sum.wrapping_add(e.mem.u32(counts.wrapping_add(index.wrapping_mul(4))));
        index = index.wrapping_add(1);
    }
    sum
}

// Translated from 00c3ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the priority queue count and signals each owned queue.
pub fn fn_00c3ea10(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0x0c, 0);
    let mut index = 0;
    while index < word(e, this, 8) {
        let list = word(e, this, 4);
        let queue = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.call(0x00c4_1330, &args![queue, 0u32]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00c3ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskManagerThread<int64_t>::BSTaskManagerThread<int64_t>` (Xbox PDB).
pub fn bs_task_manager_thread_construct(
    e: &mut Engine,
    this: Ptr,
    manager: Ptr,
    thread_id: u32,
) -> Ptr {
    e.call(0x00c4_2dd0, &args![this, thread_id, 0x010c_1748u32]);
    set_word(e, this, 0, 0x010c_1740);
    set_word(e, this, 0x30, manager.addr());
    this
}

// Translated from 00c3f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<...>::IncrementCount` (Xbox PDB).
pub fn lock_free_map_increment_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.addr().wrapping_add(0x18)],
    );
}

// Translated from 00c3f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreePriorityQueue<...>::IncrementCount` (Xbox PDB).
pub fn lock_free_priority_queue_increment_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.addr().wrapping_add(0x0c)],
    );
}

// Translated from 00c3f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreePriorityQueue<...>::DecrementCount` (Xbox PDB).
pub fn lock_free_priority_queue_decrement_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.addr().wrapping_add(0x0c)],
    );
}

// Translated from 00c3f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets this thread's map interface, allocating it if TLS has none.
pub fn fn_00c3f750(e: &mut Engine, this: Ptr) -> Ptr {
    let manager = word(e, this, 0x14);
    let tls_slot = e.mem.u32(manager + 4);
    let interface = e.call(0x00fd_f044, &args![tls_slot]).u32();
    if interface != 0 {
        Ptr::new(interface)
    } else {
        e.call(0x00c4_1610, &args![manager, this]).ptr()
    }
}

// Translated from 00c3f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<...>::DecrementCount` (Xbox PDB).
pub fn lock_free_map_decrement_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.addr().wrapping_add(0x18)],
    );
}

// Translated from 00c3f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<NiPointer<IOTask>>::AllocateInterface` (Xbox PDB).
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f7a0 - SEH unwinding is not modeled.
pub fn lock_free_queue_allocate_interface(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    let storage = allocate(e, 0x14);
    let interface = if storage == 0 {
        0
    } else {
        let nodes = word(e, this, 0x10);
        let first = nodes.wrapping_add(index.wrapping_mul(8));
        e.call(0x0044_cd00, &args![storage, this, first, first + 4])
            .u32()
    };
    e.mem.set_u32(tls, previous);
    Ptr::new(interface)
}

// Translated from 00c3f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a lock-free task map's interface manager and allocated arrays.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f8c0 - SEH unwinding is not modeled.
pub fn fn_00c3f8c0(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_16f4);
    e.call(0x00c4_0b70, &args![this, 1u32]);
    let referenced = word(e, this, 0x0c);
    e.call(DEALLOCATE, &args![0x011f_6238u32, referenced]);
    let buckets = word(e, this, 4);
    e.call(DEALLOCATE, &args![0x011f_6238u32, buckets]);
    set_word(e, this, 0, 0x0101_7458);
}

// Translated from 00c3f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries a task in each priority queue until one accepts it.
/// The compiler passes an owning NiPointer by value to the virtual call.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f960 - SEH unwinding is not modeled.
pub fn fn_00c3f960(e: &mut Engine, this: Ptr, task: Ptr) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let mut accepted = false;
    let count = word(e, this, 8);
    for index in 0..count {
        if !task.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![task.addr() + 8]);
        }
        accepted = e.vcall(this.addr(), 0x18, &args![index, task]).bool();
        if accepted {
            break;
        }
    }
    let activity = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, activity);
    if activity == 0 {
        set_word(e, this, 0x20, 0);
    }
    if !task.is_null() {
        e.call(0x0044_dd60, &args![task]);
    }
    accepted
}

// Translated from 00c3fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys each task thread and the task manager's two arrays.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3fa70 - SEH unwinding is not modeled.
pub fn fn_00c3fa70(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_167c);
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        if thread != 0 {
            e.vcall(thread, 0, &args![1u32]);
        }
        index = index.wrapping_add(1);
    }
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 0x50)]);
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 0x54)]);
    fn_00c3f8c0(e, this);
}

// Translated from 00c3fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a task through the manager's virtual key, thread, and insertion methods.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3fb50 - SEH unwinding is not modeled.
pub fn fn_00c3fb50(e: &mut Engine, this: Ptr, task: Ptr) -> bool {
    set_word(e, task, 0x0c, 1);
    let low = word(e, task, 0x10);
    let high = word(e, task, 0x14);
    let bucket = e.vcall(this.addr(), 0x24, &args![low, high]).u32();
    let thread = e.vcall(this.addr(), 0x58, &args![task]).u32();
    let retained = e.with_stack(4, |e, retained| {
        e.mem.set_u32(retained.addr(), task.addr());
        if !task.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![task.addr() + 8]);
        }
        let updated_low = word(e, task, 0x10);
        let updated_high = word(e, task, 0x14);
        let result = e
            .vcall(
                this.addr(),
                0x10,
                &args![updated_low, updated_high, retained, 0u32],
            )
            .bool();
        let remaining = e.mem.u32(retained.addr());
        if remaining != 0 {
            e.call(0x0044_dd60, &args![remaining]);
        }
        result
    });
    if retained {
        let counts = word(e, this, 0x54);
        e.call(
            INTERLOCKED_INCREMENT,
            &args![counts.wrapping_add(bucket.wrapping_mul(4))],
        );
        e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x58]);
        let threads = word(e, this, 0x50);
        let target = e.mem.u32(threads.wrapping_add(thread.wrapping_mul(4)));
        e.call(0x0044_2600, &args![target]);
    }
    retained
}

// Translated from 00c3fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls a task's virtual method at slot +4.
pub fn fn_00c3fc80(e: &mut Engine, _this: Ptr, task: Ptr) {
    e.vcall(task.addr(), 4, &[]);
}

// Translated from 00c3fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a task between state transitions 4 and 5, then enqueues it.
pub fn fn_00c3fca0(e: &mut Engine, this: Ptr, task: Ptr) {
    set_word(e, task, 0x0c, 4);
    e.vcall(task.addr(), 8, &[]);
    set_word(e, task, 0x0c, 5);
    e.call(0x0044_e050, &args![this, task]);
}

// Translated from 00c3fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cancels queued tasks until a full pass finds none left.
/// The game's SEH frame and unreachable cleanup branch are omitted.
// Translation incomplete: 00c3fce0 - SEH unwinding is not modeled.
pub fn fn_00c3fce0(e: &mut Engine, this: Ptr) {
    let mut found_any = true;
    while found_any {
        found_any = false;
        e.with_stack(0x50, |e, cursor| {
            set_word(e, cursor, 0, 0x010c_1664);
            set_word(e, cursor, 8, 0);
            set_word(e, cursor, 0x10, 0);
            set_word(e, cursor, 0x14, 0);
            e.mem.set_u8(cursor.addr() + 0x18, 0);
            while e.mem.u8(cursor.addr() + 0x18) & 2 == 0 {
                let key = cursor.addr() + 0x30;
                let task_out = cursor.addr() + 0x3c;
                e.mem.set_u32(key, 0);
                e.mem.set_u32(key + 4, 0);
                e.mem.set_u32(task_out, 0);
                let found = e
                    .call(0x00c4_0e70, &args![this, cursor, key, task_out, 1u32])
                    .bool();
                let task = e.mem.u32(task_out);
                if found {
                    e.call(0x0044_ac40, &args![this, task, 0u32]);
                    found_any = true;
                }
                if task != 0 {
                    e.call(0x0044_dd60, &args![task]);
                }
            }
        });
    }
}

// Translated from 00c3fe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a queue iterator or request record's nine words.
pub fn fn_00c3fe00(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    for (index, value) in [first, second, third, fourth, 0, 0, 0, 0, 0]
        .iter()
        .copied()
        .enumerate()
    {
        set_word(e, this, index as u32 * 4, value);
    }
    this
}

// Translated from 00c3fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets a task from a marked map node and clears the caller's three hazard pointers.
pub fn fn_00c3fe80(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    task_out: Ptr,
) -> bool {
    let mut found = e
        .call(0x00c4_16a0, &args![this, first, second, third])
        .bool();
    if found {
        let node = word(e, this, 0x14) & !1;
        assign_ref_from(e, task_out.addr(), node + 8);
        if e.mem.u32(node + 0x0c) & 1 != 0 {
            found = false;
        }
    }
    for offset in [4, 8, 0x0c] {
        let hazard = word(e, this, offset);
        e.mem.set_u32(hazard, 0);
    }
    found
}

// Translated from 00c3ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a key and task into the lock-free map, or assigns an existing task.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3ffb0 - SEH unwinding is not modeled.
pub fn fn_00c3ffb0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    task: Ptr,
    assign_existing: bool,
) -> bool {
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    let mut new_node = 0;
    let mut result = true;
    loop {
        if e.call(0x00c4_16a0, &args![this, first, second, third])
            .bool()
        {
            if new_node != 0 {
                let held = e.mem.u32(new_node + 8);
                if held != 0 {
                    e.call(0x0044_dd60, &args![held]);
                }
                e.call(DEALLOCATE, &args![0x011f_6238u32, new_node]);
            }
            if assign_existing {
                let found = word(e, this, 0x14) & !1;
                assign_ref_from(e, found + 8, task.addr());
            } else {
                result = false;
            }
            break;
        }
        if new_node == 0 {
            new_node = allocate(e, 0x10);
            if new_node != 0 {
                let key = e.vcall(word(e, this, 0), 0x2c, &args![second, third]).u64();
                e.mem.set_u64(new_node, key);
                e.mem.set_u32(new_node + 8, 0);
                e.mem.set_u32(new_node + 0x0c, 0);
                fn_00c41050(e, Ptr::new(new_node + 8), task);
            }
        }
        let head = e.with_stack(4, |e, p| {
            e.mem.set_u32(p.addr(), word(e, this, 0x14) & !1);
            e.call(0x0044_e010, &args![p, 0u32]);
            e.mem.u32(p.addr())
        });
        e.mem.set_u32(new_node + 0x0c, head);
        let observed = e
            .call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![word(e, this, 0x10), new_node & !1, head],
            )
            .u32();
        if observed == head {
            e.vcall(word(e, this, 0), 0x3c, &[]);
            break;
        }
    }
    for offset in [4, 8, 0x0c] {
        e.mem.set_u32(word(e, this, offset), 0);
    }
    e.mem.set_u32(tls, previous);
    result
}

// Translated from 00c403d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks an existing map node for removal and unlinks it when the CAS succeeds.
pub fn fn_00c403d0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> bool {
    let mut result = false;
    loop {
        if !e
            .call(0x00c4_16a0, &args![this, first, second, third])
            .bool()
        {
            break;
        }
        let successor = word(e, this, 0x18) & !1;
        let (marked, normalized_successor) = e.with_stack(8, |e, p| {
            e.mem.set_u32(p.addr(), successor);
            e.call(0x0044_e010, &args![p, 0u32]);
            let refreshed = word(e, this, 0x18) & !1;
            e.mem.set_u32(p.addr() + 4, refreshed);
            e.call(0x0044_e010, &args![p.addr() + 4, 1u32]);
            let node = word(e, this, 0x14) & !1;
            let normalized_successor = e.mem.u32(p.addr());
            let observed = e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![node + 0x0c, e.mem.u32(p.addr() + 4), normalized_successor],
            );
            (observed.u32() == normalized_successor, normalized_successor)
        });
        if !marked {
            continue;
        }
        let replacement = word(e, this, 0x14) & !1;
        let expected = e.with_stack(4, |e, p| {
            e.mem.set_u32(p.addr(), replacement);
            let previous = e.mem.u32(p.addr());
            let observed = e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![word(e, this, 0x10), normalized_successor, previous],
            );
            observed.u32() == previous
        });
        if expected {
            e.call(0x00c4_1f40, &args![this, word(e, this, 0x14) & !1]);
        } else {
            e.call(0x00c4_16a0, &args![this, first, second, third]);
        }
        e.vcall(word(e, this, 0), 0x40, &[]);
        result = true;
        break;
    }
    for offset in [4, 8, 0x0c] {
        e.mem.set_u32(word(e, this, offset), 0);
    }
    result
}

// Translated from 00c40680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks and unlinks a map node selected by a two-word key.
pub fn fn_00c40680(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let mut result = false;
    loop {
        if !fn_00c41ac0(e, this, first, Ptr::new(second)) {
            break;
        }
        let successor = word(e, this, 0x18) & !1;
        let (marked, normalized_successor) = e.with_stack(8, |e, p| {
            e.mem.set_u32(p.addr(), successor);
            e.call(0x0044_e010, &args![p, 0u32]);
            e.mem.set_u32(p.addr() + 4, word(e, this, 0x18) & !1);
            e.call(0x0044_e010, &args![p.addr() + 4, 1u32]);
            let node = word(e, this, 0x14) & !1;
            let normalized_successor = e.mem.u32(p.addr());
            let observed = e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![node + 0x0c, e.mem.u32(p.addr() + 4), normalized_successor],
            );
            (observed.u32() == normalized_successor, normalized_successor)
        });
        if !marked {
            continue;
        }
        let unlinked = e.with_stack(4, |e, p| {
            e.mem.set_u32(p.addr(), word(e, this, 0x14) & !1);
            let expected = e.mem.u32(p.addr());
            e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![word(e, this, 0x10), normalized_successor, expected],
            )
            .u32()
                == expected
        });
        if unlinked {
            e.call(0x00c4_1f40, &args![this, word(e, this, 0x14) & !1]);
        } else {
            fn_00c41ac0(e, this, first, Ptr::new(second));
        }
        result = true;
        break;
    }
    for offset in [4, 8, 0x0c] {
        e.mem.set_u32(word(e, this, offset), 0);
    }
    result
}

// Translated from 00c40910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a pointer held in a single-word slot.
pub fn fn_00c40910(e: &mut Engine, this: Ptr) {
    let held = word(e, this, 0);
    if held != 0 {
        e.call(0x0044_dd60, &args![held]);
    }
}

// Translated from 00c40930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and optionally frees a lock-free task map.
pub fn fn_00c40930(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c3f8c0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c40960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and optionally frees an IO task manager.
pub fn fn_00c40960(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c3fa70(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c40990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the lock-free map iterator's vtable.
pub fn fn_00c40990(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_1670);
}

// Translated from 00c409b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreePriorityQueue<NiPointer<IOTask>>` scalar deleting destructor (Xbox PDB).
pub fn lock_free_priority_queue_delete(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c409e0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c409e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys each allocated priority queue interface and its pointer array.
pub fn fn_00c409e0(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_16dc);
    let mut index = 0;
    while index < word(e, this, 8) {
        let slots = word(e, this, 4);
        let interface = e.mem.u32(slots.wrapping_add(index.wrapping_mul(4)));
        if interface != 0 {
            e.vcall(interface, 0, &args![1u32]);
        }
        index = index.wrapping_add(1);
    }
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 4)]);
}

// Translated from 00c40a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<NiPointer<IOTask>>` scalar deleting destructor (Xbox PDB).
pub fn lock_free_queue_delete(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c40aa0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c40aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a lock-free queue's interface manager and node array.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c40aa0 - SEH unwinding is not modeled.
pub fn fn_00c40aa0(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_1760);
    fn_00c41330(e, this, true);
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 0x10)]);
    set_word(e, this, 0, 0x0101_7458);
}

// Translated from 00c40b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMapIterator::ClearKey` (Xbox PDB): clears its 64-bit key.
pub fn lock_free_map_iterator_clear_key(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0x10, 0);
    set_word(e, this, 0x14, 0);
}

// Translated from 00c40b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMapIterator` scalar deleting destructor (Xbox PDB).
pub fn lock_free_map_iterator_delete(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c40990(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c40b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a map's interface manager and all bucket chains, optionally replacing the manager.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c40b70 - SEH unwinding is not modeled.
pub fn fn_00c40b70(e: &mut Engine, this: Ptr, replace_manager: bool) {
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    let manager = word(e, this, 0x14);
    if manager != 0 {
        e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
        let capacity = e.mem.u32(word(e, this, 0x14));
        let manager = word(e, this, 0x14);
        if manager != 0 {
            fn_00c420d0(e, Ptr::new(manager));
            e.call(DEALLOCATE, &args![0x011f_6238u32, manager]);
        }
        set_word(e, this, 0x14, 0);
        set_word(e, this, 0x18, 0);
        let mut index = 0;
        while index < word(e, this, 8) {
            let bucket = word(e, this, 0x0c).wrapping_add(index.wrapping_mul(4));
            let mut node = e.mem.u32(bucket);
            e.mem.set_u32(bucket, 0);
            while node & !1 != 0 {
                let current = node & !1;
                node = e.mem.u32(current + 0x0c);
                e.mem.set_u32(current + 0x0c, 0);
                let held = e.mem.u32(current + 8);
                if held != 0 {
                    e.call(0x0044_dd60, &args![held]);
                    e.mem.set_u32(current + 8, 0);
                }
                e.call(0x00c4_1fe0, &args![this, current]);
            }
            index = index.wrapping_add(1);
        }
        if !replace_manager {
            let storage = allocate(e, 0x10);
            let replacement = if storage == 0 {
                0
            } else {
                e.call(0x00c4_2180, &args![storage, capacity]).u32()
            };
            set_word(e, this, 0x14, replacement);
        }
        let activity = word(e, this, 0x24).wrapping_sub(1);
        set_word(e, this, 0x24, activity);
        if activity == 0 {
            set_word(e, this, 0x20, 0);
        }
    }
    e.mem.set_u32(tls, previous);
}

// Translated from 00c40e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances a map iterator, returning a key and its task when found.
pub fn fn_00c40e70(
    e: &mut Engine,
    this: Ptr,
    iterator: Ptr,
    key_out: Ptr,
    value_out: Ptr,
    continue_buckets: bool,
) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let mut found = false;
    if e.mem.u8(iterator.addr() + 0x18) & 2 == 0 {
        loop {
            let first_in_bucket = e.mem.u8(iterator.addr() + 0x18) & 1 == 0;
            let bucket = word(e, iterator, 8);
            let interface = fn_00c3f750(e, this);
            found = if first_in_bucket {
                let found = fn_00c42380(
                    e,
                    interface,
                    bucket,
                    Ptr::new(iterator.addr() + 0x10),
                    value_out,
                );
                let flags = e.mem.u8(iterator.addr() + 0x18);
                e.mem.set_u8(iterator.addr() + 0x18, flags | 1);
                found
            } else {
                fn_00c42560(
                    e,
                    interface,
                    bucket,
                    Ptr::new(iterator.addr() + 0x10),
                    value_out,
                )
            };
            if found {
                set_word(e, key_out, 0, word(e, iterator, 0x10));
                set_word(e, key_out, 4, word(e, iterator, 0x14));
                break;
            }
            let next = word(e, iterator, 8).wrapping_add(1);
            set_word(e, iterator, 8, next);
            if next >= word(e, this, 8) {
                let flags = e.mem.u8(iterator.addr() + 0x18);
                e.mem.set_u8(iterator.addr() + 0x18, flags | 2);
                break;
            }
            if !continue_buckets {
                break;
            }
            let flags = e.mem.u8(iterator.addr() + 0x18);
            e.mem.set_u8(iterator.addr() + 0x18, flags & !1);
        }
    }
    let activity = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, activity);
    if activity == 0 {
        set_word(e, this, 0x20, 0);
    }
    found
}

// Translated from 00c41050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces an owning task pointer with the pointer in another slot.
pub fn fn_00c41050(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    assign_ref_from(e, this.addr(), source.addr());
    this
}

// Translated from 00c410b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskManagerThread<int64_t>::ThreadUpdate` (Xbox PDB): waits for tasks,
/// scans the manager map, and runs tasks moved from state 1 to state 3.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c410b0 - SEH unwinding is not modeled.
pub fn bs_task_manager_thread_update(e: &mut Engine, this: Ptr) {
    while e.mem.u8(word(e, this, 0x30) + 0x40) != 1 {
        e.call(0x00fd_f1c8, &args![word(e, this, 0x10), u32::MAX]);
        e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x0c]);
        e.with_stack(0x30, |e, scratch| {
            let iterator = scratch;
            let key = Ptr::new(scratch.addr() + 0x20);
            let task_slot = Ptr::new(scratch.addr() + 0x28);
            set_word(e, iterator, 0, 0x010c_1664);
            set_word(e, iterator, 8, 0);
            set_word(e, iterator, 0x10, 0);
            set_word(e, iterator, 0x14, 0);
            e.mem.set_u8(iterator.addr() + 0x18, 0);
            set_word(e, task_slot, 0, 0);
            let mut accepted_task = false;
            while e.mem.u8(iterator.addr() + 0x18) & 2 == 0 {
                let waited = e.call(0x00fd_f1c8, &args![word(e, this, 0x1c), 0u32]).u32();
                if waited == 0x102 {
                    e.call(0x00fd_f1c8, &args![word(e, this, 0x1c), u32::MAX]);
                    e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x18]);
                    fn_00c42060(e, iterator, 0);
                }
                set_word(e, task_slot, 0, 0);
                let map = Ptr::new(word(e, this, 0x30));
                let found = fn_00c40e70(e, map, iterator, key, task_slot, !accepted_task);
                let task = word(e, task_slot, 0);
                if !found {
                    if e.mem.u8(iterator.addr() + 0x18) & 2 == 0 {
                        fn_00c42060(e, iterator, 0);
                        accepted_task = false;
                    }
                } else if word(e, Ptr::new(task), 0x0c) == 1
                    && e.call(
                        INTERLOCKED_COMPARE_EXCHANGE,
                        &args![task + 0x0c, 3u32, 1u32],
                    )
                    .u32()
                        == 1
                {
                    accepted_task = true;
                    let manager = word(e, this, 0x30);
                    let current_task = word(e, task_slot, 0);
                    e.vcall(manager, 0x4c, &args![current_task]);
                    let manager = word(e, this, 0x30);
                    let current_task = word(e, task_slot, 0);
                    e.vcall(manager, 0x50, &args![current_task]);
                }
                let task = word(e, task_slot, 0);
                if task != 0 {
                    e.call(0x0044_dd60, &args![task]);
                    set_word(e, task_slot, 0, 0);
                }
                e.call(0x00fd_f1b4, &args![word(e, this, 0x1c), 1u32, 0u32]);
                e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x18]);
            }
            set_word(e, iterator, 0, 0x010c_1670);
        });
    }
}

// Translated from 00c41610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a per-thread interface slot and publishes it through Windows TLS.
pub fn fn_00c41610(e: &mut Engine, this: Ptr, owner: Ptr) -> Ptr {
    let index = e
        .call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x0c])
        .u32()
        .wrapping_sub(1);
    if index >= word(e, this, 0) {
        return Ptr::new(0);
    }
    let interface = e.vcall(owner.addr(), 4, &args![index]).ptr();
    let slots = word(e, this, 8);
    e.mem
        .set_u32(slots + index.wrapping_mul(8) + 4, interface.addr());
    let thread_id = e.call(0x00fd_f1d0, &[]).u32();
    let slots = word(e, this, 8);
    e.mem.set_u32(slots + index.wrapping_mul(8), thread_id);
    e.call(0x00fd_f04c, &args![word(e, this, 4), interface]);
    interface
}

// Translated from 00c42060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets an iterator through its virtual slot and assigns a bucket index.
pub fn fn_00c42060(e: &mut Engine, this: Ptr, bucket: u32) {
    e.vcall(this.addr(), 4, &[]);
    set_word(e, this, 8, bucket);
    let flags = e.mem.u8(this.addr() + 0x18);
    e.mem.set_u8(this.addr() + 0x18, flags & !3);
}

// Translated from 00c420d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a TLS interface manager's objects, slot array, and TLS index.
pub fn fn_00c420d0(e: &mut Engine, this: Ptr) {
    let mut index = 0;
    while index < word(e, this, 0) {
        let slots = word(e, this, 8);
        let interface = e.mem.u32(slots + index.wrapping_mul(8) + 4);
        if interface != 0 {
            e.call(0x00c4_2b40, &args![interface]);
            e.call(DEALLOCATE, &args![0x011f_6238u32, interface]);
        }
        index = index.wrapping_add(1);
    }
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 8)]);
    e.call(0x00fd_f050, &args![word(e, this, 4)]);
}

// Translated from 00c41330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a lock-free queue's node chain and optionally rebuilds its interface manager.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c41330 - SEH unwinding is not modeled.
pub fn fn_00c41330(e: &mut Engine, this: Ptr, keep_cleared: bool) {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    let capacity = e.mem.u32(word(e, this, 0x14));
    let manager = word(e, this, 0x14);
    if manager != 0 {
        e.call(0x00c4_22d0, &args![manager]);
        e.call(DEALLOCATE, &args![0x011f_6238u32, manager]);
    }
    set_word(e, this, 0x14, 0);
    set_word(e, this, 0x18, 0);
    while word(e, this, 4) != 0 {
        let node = word(e, this, 4);
        let next = e.mem.u32(node);
        let held = e.mem.u32(node + 4);
        if held != 0 {
            e.call(0x0044_dd60, &args![held]);
            e.mem.set_u32(node + 4, 0);
        }
        let node = word(e, this, 4);
        if node != 0 {
            let held = e.mem.u32(node + 4);
            if held != 0 {
                e.call(0x0044_dd60, &args![held]);
            }
            e.call(DEALLOCATE, &args![0x011f_6238u32, node]);
        }
        set_word(e, this, 4, next);
    }
    if !keep_cleared {
        let node = allocate(e, 8);
        if node != 0 {
            e.mem.set_u32(node, 0);
            e.mem.set_u32(node + 4, 0);
        }
        set_word(e, this, 4, node);
        set_word(e, this, 8, node);
        let storage = allocate(e, 0x10);
        let manager = if storage == 0 {
            0
        } else {
            e.call(0x00c4_2180, &args![storage, capacity]).u32()
        };
        set_word(e, this, 0x14, manager);
    }
    let activity = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, activity);
    if activity == 0 {
        set_word(e, this, 0x20, 0);
    }
    e.mem.set_u32(tls, previous);
}

// Translated from 00c41ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Searches one map bucket while helping unlink marked nodes.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c41ac0 - SEH unwinding is not modeled.
pub fn fn_00c41ac0(e: &mut Engine, this: Ptr, bucket: u32, task: Ptr) -> bool {
    'restart: loop {
        loop {
            let buckets = e.mem.u32(word(e, this, 0).wrapping_add(0x0c));
            let bucket_slot = buckets.wrapping_add(bucket.wrapping_mul(4));
            set_word(e, this, 0x10, bucket_slot);
            let head = e.mem.u32(bucket_slot);
            set_word(e, this, 0x14, head);
            e.mem.set_u32(word(e, this, 8), head & !1);
            let normalized = e.with_stack(4, |e, p| {
                e.mem.set_u32(p.addr(), head);
                e.call(0x0044_e010, &args![p, 0u32]);
                e.mem.u32(p.addr())
            });
            if e.mem.u32(word(e, this, 0x10)) == normalized {
                break;
            }
        }
        loop {
            let node = word(e, this, 0x14) & !1;
            if node == 0 {
                return false;
            }
            let next = e.mem.u32(node + 0x0c);
            set_word(e, this, 0x18, next);
            e.mem.set_u32(word(e, this, 4), next & !1);
            if next != e.mem.u32(node + 0x0c) {
                continue 'restart;
            }
            let held = e.mem.u32(node + 8);
            if held != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![held + 8]);
            }
            let head = word(e, this, 0x14) & !1;
            let normalized = e.with_stack(4, |e, p| {
                e.mem.set_u32(p.addr(), head);
                e.call(0x0044_e010, &args![p, 0u32]);
                e.mem.u32(p.addr())
            });
            if e.mem.u32(word(e, this, 0x10)) != normalized {
                if held != 0 {
                    e.call(0x0044_dd60, &args![held]);
                }
                continue 'restart;
            }
            if next & 1 == 0 {
                let target = word(e, task, 0);
                if held == target {
                    if held != 0 {
                        e.call(0x0044_dd60, &args![held]);
                    }
                    return true;
                }
                set_word(e, this, 0x10, node + 0x0c);
                e.mem.set_u32(word(e, this, 0x0c), node);
            } else {
                let observed = e.call(
                    INTERLOCKED_COMPARE_EXCHANGE,
                    &args![word(e, this, 0x10), next & !1, normalized],
                );
                if observed.u32() != normalized {
                    if held != 0 {
                        e.call(0x0044_dd60, &args![held]);
                    }
                    continue 'restart;
                }
                e.call(0x00c4_1f40, &args![this, node]);
            }
            let next = word(e, this, 0x18);
            set_word(e, this, 0x14, next);
            e.mem.set_u32(word(e, this, 8), next & !1);
            if held != 0 {
                e.call(0x0044_dd60, &args![held]);
            }
        }
    }
}

// Translated from 00c42380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a bucket's first unmarked node into the key and task outputs.
pub fn fn_00c42380(e: &mut Engine, this: Ptr, bucket: u32, key: Ptr, task: Ptr) -> bool {
    let found = loop {
        loop {
            let buckets = e.mem.u32(word(e, this, 0).wrapping_add(0x0c));
            let slot = buckets.wrapping_add(bucket.wrapping_mul(4));
            set_word(e, this, 0x10, slot);
            let head = e.mem.u32(slot);
            set_word(e, this, 0x14, head);
            e.mem.set_u32(word(e, this, 8), head & !1);
            if e.mem.u32(word(e, this, 0x10)) == head & !1 {
                break;
            }
        }
        let node = word(e, this, 0x14) & !1;
        if node == 0 {
            break false;
        }
        e.vcall(
            word(e, this, 0),
            0x30,
            &args![e.mem.u32(node), e.mem.u32(node + 4), key],
        );
        let node = word(e, this, 0x14) & !1;
        assign_ref_from(e, task.addr(), node + 8);
        if e.mem.u32((word(e, this, 0x14) & !1) + 0x0c) & 1 == 0 {
            break true;
        }
    };
    for offset in [4, 8, 0x0c] {
        e.mem.set_u32(word(e, this, offset), 0);
    }
    found
}

// Translated from 00c42560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances within a bucket using the map search and rejects marked successors.
pub fn fn_00c42560(e: &mut Engine, this: Ptr, bucket: u32, key: Ptr, task: Ptr) -> bool {
    let found = 'search: loop {
        while !e
            .call(
                0x00c4_16a0,
                &args![this, bucket, word(e, key, 0), word(e, key, 4)],
            )
            .bool()
        {
            let node = word(e, this, 0x14) & !1;
            if node == 0 {
                break 'search false;
            }
            if e.mem.u32(node + 0x0c) & 1 == 0 {
                e.vcall(
                    word(e, this, 0),
                    0x30,
                    &args![e.mem.u32(node), e.mem.u32(node + 4), key],
                );
                let node = word(e, this, 0x14) & !1;
                assign_ref_from(e, task.addr(), node + 8);
                if e.mem.u32((word(e, this, 0x14) & !1) + 0x0c) & 1 == 0 {
                    break 'search true;
                }
                assign_ref(e, task.addr(), 0);
            }
        }
        let node = word(e, this, 0x18) & !1;
        if node == 0 {
            break false;
        }
        if e.mem.u32(node + 0x0c) & 1 == 0 {
            e.vcall(
                word(e, this, 0),
                0x30,
                &args![e.mem.u32(node), e.mem.u32(node + 4), key],
            );
            let node = word(e, this, 0x18) & !1;
            assign_ref_from(e, task.addr(), node + 8);
            if e.mem.u32((word(e, this, 0x18) & !1) + 0x0c) & 1 == 0 {
                break true;
            }
            assign_ref(e, task.addr(), 0);
        }
    };
    for offset in [4, 8, 0x0c] {
        e.mem.set_u32(word(e, this, offset), 0);
    }
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045_6520, io_manager_load_queued_priority(Ptr)),
        entry!(0x005a_8040, fn_005a8040(Ptr, u32)),
        entry!(0x00c3_db60, fn_00c3db60(Ptr)),
        entry!(0x00c3_df40, io_manager_requeue_task(Ptr, Ptr, i32)),
        entry!(0x00c3_dfa0, fn_00c3dfa0(Ptr, i32)),
        entry!(0x00c3_e1b0, fn_00c3e1b0(Ptr, i32, i32)),
        entry!(0x00c3_e310, fn_00c3e310(Ptr)),
        entry!(0x00c3_e340, fn_00c3e340(Ptr)),
        entry!(0x00c3_e380, fn_00c3e380(Ptr)),
        entry!(0x00c3_e3a0, fn_00c3e3a0(Ptr, u32, u32) -> i32),
        entry!(0x00c3_e3f0, fn_00c3e3f0(Ptr, u32) -> Ptr),
        entry!(0x00c3_e420, fn_00c3e420(Ptr, Ptr) -> bool),
        entry!(0x00c3_e490, fn_00c3e490(Ptr, Ptr) -> bool),
        entry!(0x00c3_e4f0, fn_00c3e4f0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00c3_e690, fn_00c3e690(Ptr, Ptr, u32, u32) -> bool),
        entry!(0x00c3_e750, fn_00c3e750(Ptr) -> bool),
        entry!(0x00c3_e7d0, fn_00c3e7d0(Ptr) -> bool),
        entry!(0x00c3_e860, fn_00c3e860(Ptr, u32) -> u32),
        entry!(0x00c3_e8b0, fn_00c3e8b0(Ptr, u32, u32, Ptr) -> Ptr),
        entry!(0x00c3_ea10, fn_00c3ea10(Ptr)),
        entry!(0x00c3_ea60, fn_00c3ea60(Ptr, Ptr) -> bool),
        entry!(0x00c3_ec80, fn_00c3ec80(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00c3_ee70, bs_task_manager_thread_construct(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00c3_eeb0, fn_00c3eeb0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00c3_f090, fn_00c3f090(Ptr, Ptr)),
        entry!(0x00c3_f6f0, lock_free_map_increment_count(Ptr)),
        entry!(0x00c3_f710, lock_free_priority_queue_increment_count(Ptr)),
        entry!(0x00c3_f730, lock_free_priority_queue_decrement_count(Ptr)),
        entry!(0x00c3_f750, fn_00c3f750(Ptr) -> Ptr),
        entry!(0x00c3_f7a0, lock_free_queue_allocate_interface(Ptr, u32) -> Ptr),
        entry!(0x00c3_f8a0, lock_free_map_decrement_count(Ptr)),
        entry!(0x00c3_f8c0, fn_00c3f8c0(Ptr)),
        entry!(0x00c3_f960, fn_00c3f960(Ptr, Ptr) -> bool),
        entry!(0x00c3_fa70, fn_00c3fa70(Ptr)),
        entry!(0x00c3_fb50, fn_00c3fb50(Ptr, Ptr) -> bool),
        entry!(0x00c3_fc80, fn_00c3fc80(Ptr, Ptr)),
        entry!(0x00c3_fca0, fn_00c3fca0(Ptr, Ptr)),
        entry!(0x00c3_fce0, fn_00c3fce0(Ptr)),
        entry!(0x00c3_fe00, fn_00c3fe00(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00c3_fe80, fn_00c3fe80(Ptr, u32, u32, u32, Ptr) -> bool),
        entry!(0x00c3_ffb0, fn_00c3ffb0(Ptr, u32, u32, u32, Ptr, bool) -> bool),
        entry!(0x00c4_03d0, fn_00c403d0(Ptr, u32, u32, u32) -> bool),
        entry!(0x00c4_0680, fn_00c40680(Ptr, u32, u32) -> bool),
        entry!(0x00c4_0910, fn_00c40910(Ptr)),
        entry!(0x00c4_0930, fn_00c40930(Ptr, u32) -> Ptr),
        entry!(0x00c4_0960, fn_00c40960(Ptr, u32) -> Ptr),
        entry!(0x00c4_0990, fn_00c40990(Ptr)),
        entry!(0x00c4_09b0, lock_free_priority_queue_delete(Ptr, u32) -> Ptr),
        entry!(0x00c4_09e0, fn_00c409e0(Ptr)),
        entry!(0x00c4_0a70, lock_free_queue_delete(Ptr, u32) -> Ptr),
        entry!(0x00c4_0aa0, fn_00c40aa0(Ptr)),
        entry!(0x00c4_0b20, lock_free_map_iterator_clear_key(Ptr)),
        entry!(0x00c4_0b40, lock_free_map_iterator_delete(Ptr, u32) -> Ptr),
        entry!(0x00c4_0b70, fn_00c40b70(Ptr, bool)),
        entry!(0x00c4_0e70, fn_00c40e70(Ptr, Ptr, Ptr, Ptr, bool) -> bool),
        entry!(0x00c4_1050, fn_00c41050(Ptr, Ptr) -> Ptr),
        entry!(0x00c4_10b0, bs_task_manager_thread_update(Ptr)),
        entry!(0x00c4_1330, fn_00c41330(Ptr, bool)),
        entry!(0x00c4_1610, fn_00c41610(Ptr, Ptr) -> Ptr),
        entry!(0x00c4_1ac0, fn_00c41ac0(Ptr, u32, Ptr) -> bool),
        entry!(0x00c4_2060, fn_00c42060(Ptr, u32)),
        entry!(0x00c4_20d0, fn_00c420d0(Ptr)),
        entry!(0x00c4_2380, fn_00c42380(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x00c4_2560, fn_00c42560(Ptr, u32, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn one(_: &mut Engine, _: &[u32]) -> Ret {
        1u32.into_ret()
    }

    fn increment(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], value);
        value.into_ret()
    }

    fn decrement(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], value);
        value.into_ret()
    }

    fn compare_exchange(e: &mut Engine, a: &[u32]) -> Ret {
        let old = e.mem.u32(a[0]);
        if old == a[2] {
            e.mem.set_u32(a[0], a[1]);
        }
        old.into_ret()
    }

    fn alloc(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.alloc(a[1].max(1)).into_ret()
    }

    fn setup_atomics(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, increment);
        e.register(INTERLOCKED_DECREMENT, decrement);
        e.register(INTERLOCKED_COMPARE_EXCHANGE, compare_exchange);
    }

    fn setup_allocator(e: &mut Engine) {
        e.register(ALLOCATE, alloc);
        e.register(DEALLOCATE, no_op);
        e.register(0x00c4_2180, |_, a| a[0].into_ret());
    }

    fn setup_destructor(e: &mut Engine) {
        e.register(0x00c4_0b70, no_op);
        e.register(DEALLOCATE, no_op);
    }

    #[test]
    fn load_queued_priority_uses_five() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x90);
        set_word(&mut e, manager, 0x68, 7);
        io_manager_load_queued_priority(&mut e, manager);
        assert_eq!(word(&e, manager, 0x68), 7);
    }

    #[test]
    fn store_word_at_54() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x58);
        fn_005a8040(&mut e, manager, 0x1234);
        assert_eq!(word(&e, manager, 0x54), 0x1234);
    }

    #[test]
    fn destructor_resets_vtable_and_releases_base() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x90);
        fn_00c3db60(&mut e, manager);
        assert_eq!(word(&e, manager, 0), 0x0101_7458);
    }

    #[test]
    fn requeue_sets_priority_byte() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        set_word(&mut e, task, 0x10, 0x4455_6677);
        set_word(&mut e, task, 0x14, 0x1122_3344);
        io_manager_requeue_task(&mut e, manager, task, 3);
        assert_eq!(word(&e, task, 0x10), 0x4403_6677);
        assert_eq!(word(&e, task, 0x14), 0x1122_3344);
    }

    #[test]
    fn load_loop_stops_when_already_loading() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x68, 7);
        fn_00c3dfa0(&mut e, manager, 5);
        assert_eq!(word(&e, manager, 0x68), 7);
    }

    #[test]
    fn lower_priority_range_is_empty() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        fn_00c3e1b0(&mut e, manager, 2, 2);
    }

    #[test]
    fn priority_runner_reloads_task_pointer_after_virtual_call() {
        use std::cell::Cell;
        use std::rc::Rc;

        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        e.register(0x00c4_2060, no_op);
        e.register(0x0044_dd60, no_op);
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let replacement = block(&mut e, 4);
        e.put_vtable(0x7900_0000, &[0, 0, 0, 0, 0, 0, 0, 0x7900_001c]);
        set_word(&mut e, first, 0, 0x7900_0000);
        let out_slot = Rc::new(Cell::new(0u32));
        let saved_slot = out_slot.clone();
        let first_addr = first.addr();
        e.register_double(0x00c4_0e70, move |e, a| {
            saved_slot.set(a[3]);
            e.mem.set_u32(a[2], 0x0001_0000);
            e.mem.set_u32(a[3], first_addr);
            e.mem.set_u8(a[1] + 0x18, 2);
            1u32.into_ret()
        });
        let replacement_addr = replacement.addr();
        e.register_double(0x7900_001c, move |e, _| {
            e.mem.set_u32(out_slot.get(), replacement_addr);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3e1b0(&mut e, manager, 1, 0);
        let releases: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _)| *addr == 0x0044_dd60)
            .collect();
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].1[0], replacement_addr);
    }

    #[test]
    fn nested_pause_skips_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x48, 1);
        fn_00c3e310(&mut e, manager);
        assert_eq!(word(&e, manager, 0x48), 2);
    }

    #[test]
    fn nested_resume_skips_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x48, 2);
        fn_00c3e340(&mut e, manager);
        assert_eq!(word(&e, manager, 0x48), 1);
    }

    #[test]
    fn clear_manager_resets_empty_queue_count() {
        let mut e = Engine::new();
        e.register(0x00c4_0e70, |e, a| {
            e.mem.set_u8(a[1] + 0x18, 2);
            Ret::default()
        });
        let manager = block(&mut e, 0x84);
        let queue = block(&mut e, 0x10);
        set_word(&mut e, manager, 0x64, queue.addr());
        set_word(&mut e, queue, 0x0c, 17);
        fn_00c3e380(&mut e, manager);
        assert_eq!(word(&e, queue, 0x0c), 0);
    }

    #[test]
    fn bucket_formula_uses_key_bytes() {
        let mut e = Engine::new();
        assert_eq!(
            fn_00c3e3a0(&mut e, Ptr::new(0), 0x0003_0000, 0x0200_0000),
            18
        );
    }

    #[test]
    fn scalar_delete_returns_this() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x90);
        assert_eq!(fn_00c3e3f0(&mut e, manager, 0), manager);
    }

    #[test]
    fn empty_queue_scan_returns_false() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x10);
        assert!(!fn_00c3e420(&mut e, queue, Ptr::new(0)));
    }

    #[test]
    fn queue_scan_rechecks_count_after_pop_callback() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        let outer = block(&mut e, 0x10);
        let slots = block(&mut e, 8);
        let inner = block(&mut e, 0x28);
        let queue = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        let output = block(&mut e, 4);
        set_word(&mut e, outer, 4, slots.addr());
        set_word(&mut e, outer, 8, 2);
        set_word(&mut e, slots, 0, inner.addr());
        set_word(&mut e, inner, 0, queue.addr());
        set_word(&mut e, inner, 4, hazard_head.addr());
        set_word(&mut e, inner, 8, hazard_next.addr());
        set_word(&mut e, inner, 0x24, 1);
        set_word(&mut e, queue, 4, sentinel.addr());
        let outer_addr = outer.addr();
        e.register_double(0x0044_9f80, move |e, a| {
            e.mem.set_u32(outer_addr + 8, 1);
            a[0].into_ret()
        });
        assert!(!fn_00c3e420(&mut e, outer, output));
    }

    #[test]
    fn queue_pop_miss_releases_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        e.register(0x0044_9f80, |_, a| a[0].into_ret());
        let queue = block(&mut e, 0x28);
        let state = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        set_word(&mut e, queue, 0, state.addr());
        set_word(&mut e, queue, 4, hazard_head.addr());
        set_word(&mut e, queue, 8, hazard_next.addr());
        set_word(&mut e, state, 4, sentinel.addr());
        let item = block(&mut e, 4);
        set_word(&mut e, queue, 0x24, 1);
        assert!(!fn_00c3e490(&mut e, queue, item));
        assert_eq!(word(&e, queue, 0x24), 0);
    }

    #[test]
    fn task_manager_constructor_sets_thread_count() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        e.register(0x00ec_61c0, no_op);
        e.register(0x00c4_2f50, no_op);
        e.register(0x00c4_2dd0, no_op);
        let manager = block(&mut e, 0x5c);
        fn_00c3e4f0(&mut e, manager, 1, 1, 1);
        assert_eq!(word(&e, manager, 0x4c), 1);
        assert_ne!(word(&e, manager, 0x50), 0);
    }

    #[test]
    fn requeue_idle_task_writes_both_key_words() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        assert!(fn_00c3e690(&mut e, manager, task, 7, 8));
        assert_eq!(word(&e, task, 0x10), 7);
        assert_eq!(word(&e, task, 0x14), 8);
    }

    #[test]
    fn first_pause_returns_true() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        let manager = block(&mut e, 0x5c);
        assert!(fn_00c3e750(&mut e, manager));
        assert_eq!(word(&e, manager, 0x48), 1);
    }

    #[test]
    fn final_resume_returns_true() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let manager = block(&mut e, 0x5c);
        set_word(&mut e, manager, 0x48, 1);
        assert!(fn_00c3e7d0(&mut e, manager));
        assert_eq!(word(&e, manager, 0x48), 0);
    }

    #[test]
    fn bucket_sum_includes_last() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let counts = block(&mut e, 12);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, counts, 0, 2);
        set_word(&mut e, counts, 4, 3);
        set_word(&mut e, counts, 8, 5);
        assert_eq!(fn_00c3e860(&mut e, manager, 2), 10);
    }

    #[test]
    fn priority_queue_constructor_allocates_one_interface() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let queue = block(&mut e, 0x10);
        fn_00c3e8b0(&mut e, queue, 1, 1, Ptr::new(0));
        let slots = word(&e, queue, 4);
        assert_ne!(e.mem.u32(slots), 0);
    }

    #[test]
    fn reset_queue_signals_each_entry() {
        let mut e = Engine::new();
        e.register(0x00c4_1330, no_op);
        let queue = block(&mut e, 0x10);
        let items = block(&mut e, 4);
        set_word(&mut e, queue, 4, items.addr());
        set_word(&mut e, queue, 8, 1);
        set_word(&mut e, queue, 0x0c, 3);
        e.call_log = Some(vec![]);
        fn_00c3ea10(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, _)| *addr == 0x00c4_1330));
    }

    #[test]
    fn empty_lock_free_pop_clears_hazards() {
        let mut e = Engine::new();
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let first_hazard = block(&mut e, 4);
        let second_hazard = block(&mut e, 4);
        let output = block(&mut e, 4);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, interface, 4, first_hazard.addr());
        set_word(&mut e, interface, 8, second_hazard.addr());
        set_word(&mut e, queue, 4, sentinel.addr());
        assert!(!fn_00c3ea60(&mut e, interface, output));
        assert_eq!(word(&e, first_hazard, 0), 0);
    }

    #[test]
    fn map_constructor_sets_capacity_and_arrays() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let map = block(&mut e, 0x28);
        fn_00c3ec80(&mut e, map, 2, 1, 4);
        assert_eq!(word(&e, map, 8), 1);
        assert_ne!(word(&e, map, 4), 0);
    }

    #[test]
    fn thread_constructor_sets_manager() {
        let mut e = Engine::new();
        e.register(0x00c4_2dd0, no_op);
        let thread = block(&mut e, 0x34);
        let manager = block(&mut e, 4);
        assert_eq!(
            bs_task_manager_thread_construct(&mut e, thread, manager, 7),
            thread
        );
        assert_eq!(word(&e, thread, 0x30), manager.addr());
    }

    #[test]
    fn queue_interface_constructor_sets_sentinel() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let interface = block(&mut e, 0x40);
        fn_00c3eeb0(&mut e, interface, 2, Ptr::new(0x1234));
        assert_eq!(word(&e, interface, 4), word(&e, interface, 8));
        assert_eq!(word(&e, interface, 0x0c), 0x1234);
    }

    #[test]
    fn retired_node_is_linked_and_counted() {
        let mut e = Engine::new();
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let node = block(&mut e, 8);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, queue, 0x0c, 2);
        fn_00c3f090(&mut e, interface, node);
        assert_eq!(word(&e, interface, 0x10), node.addr());
        assert_eq!(word(&e, interface, 0x0c), 1);
    }

    #[test]
    fn map_count_increments_at_offset_18() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let map = block(&mut e, 0x1c);
        lock_free_map_increment_count(&mut e, map);
        assert_eq!(word(&e, map, 0x18), 1);
    }

    #[test]
    fn priority_queue_count_increments_at_offset_c() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let queue = block(&mut e, 0x10);
        lock_free_priority_queue_increment_count(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 1);
    }

    #[test]
    fn priority_queue_count_decrements_at_offset_c() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let queue = block(&mut e, 0x10);
        set_word(&mut e, queue, 0x0c, 2);
        lock_free_priority_queue_decrement_count(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 1);
    }

    #[test]
    fn existing_tls_interface_is_returned() {
        let mut e = Engine::new();
        e.register(0x00fd_f044, |_, _| 0x4321u32.into_ret());
        let map = block(&mut e, 0x18);
        let manager = block(&mut e, 8);
        set_word(&mut e, map, 0x14, manager.addr());
        set_word(&mut e, manager, 4, 5);
        assert_eq!(fn_00c3f750(&mut e, map).addr(), 0x4321);
    }

    #[test]
    fn allocate_interface_passes_adjacent_hazards() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        e.register(0x0044_cd00, |_, a| a[0].into_ret());
        let queue = block(&mut e, 0x14);
        let hazards = block(&mut e, 16);
        set_word(&mut e, queue, 0x10, hazards.addr());
        e.call_log = Some(vec![]);
        assert!(!lock_free_queue_allocate_interface(&mut e, queue, 1).is_null());
        let call = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(addr, _)| *addr == 0x0044_cd00)
            .unwrap();
        assert_eq!(call.1[2], hazards.addr() + 8);
        assert_eq!(call.1[3], hazards.addr() + 12);
    }

    #[test]
    fn map_count_decrements_at_offset_18() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let map = block(&mut e, 0x1c);
        set_word(&mut e, map, 0x18, 2);
        lock_free_map_decrement_count(&mut e, map);
        assert_eq!(word(&e, map, 0x18), 1);
    }

    #[test]
    fn map_destructor_resets_base_vtable() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let map = block(&mut e, 0x28);
        fn_00c3f8c0(&mut e, map);
        assert_eq!(word(&e, map, 0), 0x0101_7458);
    }

    #[test]
    fn empty_priority_scan_releases_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        let queue = block(&mut e, 0x28);
        set_word(&mut e, queue, 0x24, 1);
        assert!(!fn_00c3f960(&mut e, queue, Ptr::new(0)));
        assert_eq!(word(&e, queue, 0x24), 0);
    }

    #[test]
    fn manager_destructor_frees_arrays() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x5c);
        e.call_log = Some(vec![]);
        fn_00c3fa70(&mut e, manager);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(addr, _)| *addr == DEALLOCATE)
                .count(),
            4
        );
    }

    #[test]
    fn run_task_updates_bucket_and_thread_counts() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_dd60, no_op);
        e.register(0x0044_2600, no_op);
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        let counts = block(&mut e, 4);
        let threads = block(&mut e, 4);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x50, threads.addr());
        let mut slots = vec![0; 0x5c / 4];
        slots[0x10 / 4] = 0x7000_0010;
        slots[0x24 / 4] = 0x7000_0024;
        slots[0x58 / 4] = 0x7000_0058;
        e.put_vtable(0x7100_0000, &slots);
        set_word(&mut e, manager, 0, 0x7100_0000);
        e.register(0x7000_0010, one);
        e.register(0x7000_0024, no_op);
        e.register(0x7000_0058, no_op);
        assert!(fn_00c3fb50(&mut e, manager, task));
        assert_eq!(word(&e, counts, 0), 1);
        assert_eq!(word(&e, manager, 0x58), 1);
    }

    #[test]
    fn task_virtual_slot_four_is_called() {
        let mut e = Engine::new();
        let task = block(&mut e, 4);
        e.put_vtable(0x7200_0000, &[0, 0x7200_0004]);
        set_word(&mut e, task, 0, 0x7200_0000);
        e.register(0x7200_0004, no_op);
        e.call_log = Some(vec![]);
        fn_00c3fc80(&mut e, Ptr::new(0), task);
        assert_eq!(e.call_log.as_ref().unwrap()[0].0, 0x7200_0004);
    }

    #[test]
    fn task_finishes_in_state_five() {
        let mut e = Engine::new();
        let manager = block(&mut e, 4);
        let task = block(&mut e, 0x10);
        e.put_vtable(0x7300_0000, &[0, 0, 0x7300_0008]);
        set_word(&mut e, task, 0, 0x7300_0000);
        e.register(0x7300_0008, no_op);
        e.register(0x0044_e050, no_op);
        fn_00c3fca0(&mut e, manager, task);
        assert_eq!(word(&e, task, 0x0c), 5);
    }

    #[test]
    fn cancel_empty_queue_stops_after_one_pass() {
        let mut e = Engine::new();
        e.register(0x00c4_0e70, |e, a| {
            e.mem.set_u8(a[1] + 0x18, 2);
            Ret::default()
        });
        let manager = block(&mut e, 0x5c);
        fn_00c3fce0(&mut e, manager);
    }

    #[test]
    fn record_constructor_clears_remaining_words() {
        let mut e = Engine::new();
        let record = block(&mut e, 0x24);
        fn_00c3fe00(&mut e, record, 1, 2, 3, 4);
        assert_eq!(word(&e, record, 0x0c), 4);
        assert_eq!(word(&e, record, 0x20), 0);
    }

    #[test]
    fn failed_map_lookup_clears_three_hazards() {
        let mut e = Engine::new();
        e.register(0x00c4_16a0, no_op);
        let interface = block(&mut e, 0x18);
        let hazards = [block(&mut e, 4), block(&mut e, 4), block(&mut e, 4)];
        for (index, hazard) in hazards.iter().enumerate() {
            set_word(&mut e, interface, 4 + index as u32 * 4, hazard.addr());
            set_word(&mut e, *hazard, 0, 99);
        }
        let output = block(&mut e, 4);
        assert!(!fn_00c3fe80(&mut e, interface, 1, 2, 3, output));
        assert!(hazards.iter().all(|h| word(&e, *h, 0) == 0));
    }

    #[test]
    fn queued_count_reloads_queue_after_callback_swap() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let second = block(&mut e, 4);
        e.put_vtable(0x7400_0000, &[0, 0, 0, 0x7400_000c]);
        e.put_vtable(0x7500_0000, &[0, 0, 0, 0x7500_000c]);
        set_word(&mut e, first, 0, 0x7400_0000);
        set_word(&mut e, second, 0, 0x7500_0000);
        set_word(&mut e, manager, 0x64, first.addr());
        let manager_addr = manager.addr();
        let second_addr = second.addr();
        e.register_double(0x7400_000c, move |e, _| {
            e.mem.set_u32(manager_addr + 0x64, second_addr);
            1u32.into_ret()
        });
        e.register(0x7500_000c, |_, _| 2u32.into_ret());
        assert_eq!(count_queued(&mut e, manager), 1);
        assert_eq!(count_queued(&mut e, manager), 2);
    }

    #[test]
    fn external_count_callback_has_no_argument_words() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x6c, 0x7d00_0000);
        e.register(0x7d00_0000, |_, a| {
            assert!(a.is_empty());
            3u32.into_ret()
        });
        assert_eq!(external_count(&mut e, manager), 3);
    }

    #[test]
    fn loading_loop_uses_replaced_queue_and_exe_progress_scale() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let second = block(&mut e, 4);
        let counts = block(&mut e, 16);
        e.put_vtable(0x7a00_0000, &[0, 0, 0, 0x7a00_000c]);
        e.put_vtable(0x7b00_0000, &[0, 0, 0, 0x7b00_000c]);
        set_word(&mut e, first, 0, 0x7a00_0000);
        set_word(&mut e, second, 0, 0x7b00_0000);
        set_word(&mut e, manager, 0x64, first.addr());
        set_word(&mut e, manager, 0x18, 1);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x78, 0x7c00_0000);
        e.map(0x0101_7a40, 8);
        e.set_global(0x0101_7a40, 37.0f64);
        e.map(0x0101_6410, 4);
        e.set_global(0x0101_6410, 100.0f32);
        e.map(0x011a_f70c, 1);
        e.register(0x7a00_000c, |_, _| 2u32.into_ret());
        let mut second_reads = 0;
        e.register_double(0x7b00_000c, move |_, _| {
            second_reads += 1;
            (if second_reads <= 2 { 1u32 } else { 0u32 }).into_ret()
        });
        e.register(SLEEP, no_op);
        let mut process_calls = 0;
        e.register_double(0x00c3_dbf0, move |_, _| {
            process_calls += 1;
            (process_calls == 2).into_ret()
        });
        let manager_addr = manager.addr();
        let second_addr = second.addr();
        e.register_double(0x7c00_0000, move |e, a| {
            if a[0] != 100.0f32.to_bits() {
                e.mem.set_u32(manager_addr + 0x64, second_addr);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3dfa0(&mut e, manager, 0);
        assert_eq!(word(&e, manager, 0x68), 6);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, _)| *addr == 0x7b00_000c));
        let progress: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _)| *addr == 0x7c00_0000)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(
            progress,
            vec![0.0f32.to_bits(), 18.5f32.to_bits(), 100.0f32.to_bits()]
        );
    }

    #[test]
    fn loading_progress_rounds_at_the_thread_precision() {
        // 00c3e0bc: under 24-bit precision (the main thread once Direct3D has its device)
        // `FDIVP` and `FMUL` each round to 24 bits; 1/3 and 2/3 of 100.0 come out one float
        // above the plain double result (0x42055555, 0x42855555).
        let mut e = Engine::new();
        crate::x87::set_control_word(0x007f);
        let manager = block(&mut e, 0x84);
        let queue = block(&mut e, 4);
        let counts = block(&mut e, 16);
        let left = block(&mut e, 4);
        e.put_vtable(0x7a00_0000, &[0, 0, 0, 0x7a00_000c]);
        set_word(&mut e, queue, 0, 0x7a00_0000);
        set_word(&mut e, manager, 0x64, queue.addr());
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x78, 0x7c00_0000);
        set_word(&mut e, left, 0, 3);
        e.map(0x0101_7a40, 8);
        e.set_global(0x0101_7a40, 100.0f64);
        e.map(0x0101_6410, 4);
        e.set_global(0x0101_6410, 100.0f32);
        e.map(0x011a_f70c, 1);
        let cell = left.addr();
        e.register_double(0x7a00_000c, move |e, _| e.mem.u32(cell).into_ret());
        e.register_double(0x00c3_dbf0, move |e, _| {
            let value = e.mem.u32(cell).wrapping_sub(1);
            e.mem.set_u32(cell, value);
            1u32.into_ret()
        });
        e.register(0x7c00_0000, no_op);
        e.call_log = Some(vec![]);
        fn_00c3dfa0(&mut e, manager, 0);
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        let progress: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _)| *addr == 0x7c00_0000)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(
            progress,
            vec![0, 0x4205_5556, 0x4285_5556, 100.0f32.to_bits()]
        );
        assert_eq!(word(&e, manager, 0x68), 6);
    }

    #[test]
    fn map_constructor_signed_decrement_skips_high_bit_capacity() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, _| {
            let p = e.mem.alloc(16);
            e.mem.set_u32(p, 0xdead_beef);
            p.into_ret()
        });
        e.register(0x00c4_2180, |_, a| a[0].into_ret());
        let map = block(&mut e, 0x28);
        fn_00c3ec80(&mut e, map, 1, 0x8000_0001, 1);
        let first = word(&e, map, 0x0c);
        assert_eq!(e.mem.u32(first), 0xdead_beef);
    }

    #[test]
    fn queue_pop_rereads_item_after_release_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let head = block(&mut e, 8);
        let next = block(&mut e, 8);
        let previous_item = block(&mut e, 0x10);
        let first_item = block(&mut e, 0x10);
        let replacement_item = block(&mut e, 0x10);
        let output = block(&mut e, 4);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, interface, 4, hazard_head.addr());
        set_word(&mut e, interface, 8, hazard_next.addr());
        set_word(&mut e, queue, 4, head.addr());
        set_word(&mut e, queue, 8, next.addr());
        set_word(&mut e, queue, 0x0c, 2);
        set_word(&mut e, head, 0, next.addr());
        set_word(&mut e, next, 4, first_item.addr());
        set_word(&mut e, output, 0, previous_item.addr());
        e.put_vtable(0x7600_0000, &[0, 0, 0, 0x7600_000c]);
        set_word(&mut e, queue, 0, 0x7600_0000);
        e.register(0x7600_000c, no_op);
        let old_addr = previous_item.addr();
        let next_addr = next.addr();
        let replacement_addr = replacement_item.addr();
        e.register_double(0x0044_dd60, move |e, a| {
            if a[0] == old_addr {
                e.mem.set_u32(next_addr + 4, replacement_addr);
            }
            Ret::default()
        });
        assert!(fn_00c3ea60(&mut e, interface, output));
        assert_eq!(word(&e, output, 0), replacement_addr);
    }

    #[test]
    fn run_task_rereads_key_after_virtual_callbacks() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_dd60, no_op);
        e.register(0x0044_2600, no_op);
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        let counts = block(&mut e, 4);
        let threads = block(&mut e, 4);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x50, threads.addr());
        set_word(&mut e, task, 0x10, 1);
        set_word(&mut e, task, 0x14, 2);
        let mut slots = vec![0; 0x5c / 4];
        slots[0x10 / 4] = 0x7700_0010;
        slots[0x24 / 4] = 0x7700_0024;
        slots[0x58 / 4] = 0x7700_0058;
        e.put_vtable(0x7701_0000, &slots);
        set_word(&mut e, manager, 0, 0x7701_0000);
        let task_addr = task.addr();
        e.register_double(0x7700_0024, move |e, _| {
            e.mem.set_u32(task_addr + 0x10, 7);
            e.mem.set_u32(task_addr + 0x14, 8);
            0u32.into_ret()
        });
        e.register(0x7700_0058, no_op);
        e.register(0x7700_0010, |_, a| {
            assert_eq!((a[1], a[2]), (7, 8));
            1u32.into_ret()
        });
        assert!(fn_00c3fb50(&mut e, manager, task));
    }

    #[test]
    fn queue_constructor_rechecks_count_after_nested_constructor() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let queue = block(&mut e, 0x10);
        let queue_addr = queue.addr();
        e.register_double(0x00c4_2180, move |e, a| {
            e.mem.set_u32(queue_addr + 8, 1);
            a[0].into_ret()
        });
        fn_00c3e8b0(&mut e, queue, 2, 1, Ptr::new(0));
        let slots = word(&e, queue, 4);
        assert_ne!(e.mem.u32(slots), 0);
        assert_eq!(e.mem.u32(slots + 4), 0);
    }

    #[test]
    fn manager_destructor_reloads_thread_array_before_free() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x5c);
        let first_array = block(&mut e, 8);
        let second_array = block(&mut e, 8);
        let thread = block(&mut e, 4);
        set_word(&mut e, first_array, 0, thread.addr());
        set_word(&mut e, manager, 0x4c, 2);
        set_word(&mut e, manager, 0x50, first_array.addr());
        e.put_vtable(0x7800_0000, &[0x7800_0004]);
        set_word(&mut e, thread, 0, 0x7800_0000);
        let manager_addr = manager.addr();
        let replacement_addr = second_array.addr();
        e.register_double(0x7800_0004, move |e, _| {
            e.mem.set_u32(manager_addr + 0x4c, 1);
            e.mem.set_u32(manager_addr + 0x50, replacement_addr);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3fa70(&mut e, manager);
        let freed = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(addr, _)| *addr == DEALLOCATE)
            .unwrap();
        assert_eq!(freed.1[1], replacement_addr);
    }

    #[test]
    fn map_insert_existing_task_assigns_and_clears_hazards() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x00c4_16a0, one);
        let interface = block(&mut e, 0x1c);
        let node = block(&mut e, 0x10);
        let task = block(&mut e, 4);
        let held = block(&mut e, 0x0c);
        set_word(&mut e, task, 0, held.addr());
        set_word(&mut e, interface, 0x14, node.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
            set_word(&mut e, hazard, 0, 123);
        }
        let tls = e.tls() + 0x2b4;
        e.mem.set_u32(tls, 11);
        assert!(fn_00c3ffb0(&mut e, interface, 0, 0, 0, task, true));
        assert_eq!(word(&e, node, 8), held.addr());
        assert_eq!(e.mem.u32(tls), 11);
        for offset in [4, 8, 0x0c] {
            assert_eq!(e.mem.u32(word(&e, interface, offset)), 0);
        }
    }

    #[test]
    fn map_insert_uses_normalized_head_for_link_and_cas() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        setup_atomics(&mut e);
        e.register(0x00c4_16a0, no_op);
        let interface = block(&mut e, 0x1c);
        let owner = block(&mut e, 4);
        let head_slot = block(&mut e, 4);
        let old_head = block(&mut e, 0x10);
        let task = block(&mut e, 4);
        let mut slots = vec![0; 0x40 / 4];
        slots[0x2c / 4] = 0x7a10_002c;
        slots[0x3c / 4] = 0x7a10_003c;
        e.put_vtable(0x7a10_0000, &slots);
        set_word(&mut e, owner, 0, 0x7a10_0000);
        set_word(&mut e, interface, 0, owner.addr());
        set_word(&mut e, interface, 0x10, head_slot.addr());
        set_word(&mut e, head_slot, 0, old_head.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        e.register(0x7a10_002c, |_, _| Ret {
            eax: 4,
            edx: 5,
            st0: 0.0,
        });
        e.register(0x7a10_003c, no_op);
        let normalized = old_head.addr();
        e.register_double(0x0044_e010, move |e, a| {
            e.mem.set_u32(a[0], normalized);
            Ret::default()
        });
        assert!(fn_00c3ffb0(&mut e, interface, 0, 0, 0, task, false));
        let inserted = word(&e, head_slot, 0);
        assert_ne!(inserted, old_head.addr());
        assert_eq!(e.mem.u32(inserted + 0x0c), old_head.addr());
    }

    #[test]
    fn map_remove_missing_key_clears_hazards() {
        let mut e = Engine::new();
        e.register(0x00c4_16a0, no_op);
        let interface = block(&mut e, 0x1c);
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
            set_word(&mut e, hazard, 0, 77);
        }
        assert!(!fn_00c403d0(&mut e, interface, 1, 2, 3));
        for offset in [4, 8, 0x0c] {
            assert_eq!(e.mem.u32(word(&e, interface, offset)), 0);
        }
    }

    #[test]
    fn map_remove_reuses_saved_normalized_successor_after_cas() {
        let mut e = Engine::new();
        e.register(0x00c4_16a0, one);
        e.register(0x00c4_1f40, no_op);
        let interface = block(&mut e, 0x1c);
        let owner = block(&mut e, 4);
        let head_slot = block(&mut e, 4);
        let node = block(&mut e, 0x10);
        let successor = block(&mut e, 0x10);
        let other = block(&mut e, 0x10);
        let mut slots = vec![0; 0x44 / 4];
        slots[0x40 / 4] = 0x7a20_0040;
        e.put_vtable(0x7a20_0000, &slots);
        e.register(0x7a20_0040, no_op);
        set_word(&mut e, owner, 0, 0x7a20_0000);
        set_word(&mut e, interface, 0, owner.addr());
        set_word(&mut e, interface, 0x10, head_slot.addr());
        set_word(&mut e, interface, 0x14, node.addr());
        set_word(&mut e, interface, 0x18, successor.addr());
        set_word(&mut e, head_slot, 0, node.addr());
        set_word(&mut e, node, 0x0c, successor.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        let normalized = successor.addr();
        e.register_double(0x0044_e010, move |e, a| {
            e.mem.set_u32(a[0], normalized | a[1]);
            Ret::default()
        });
        let node_addr = node.addr();
        let interface_addr = interface.addr();
        let other_addr = other.addr();
        let mut calls = 0;
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |e, a| {
            calls += 1;
            if calls == 1 {
                assert_eq!((a[0], a[2]), (node_addr + 0x0c, normalized));
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(interface_addr + 0x18, other_addr);
                normalized.into_ret()
            } else {
                assert_eq!((a[1], a[2]), (normalized, node_addr));
                node_addr.into_ret()
            }
        });
        assert!(fn_00c403d0(&mut e, interface, 1, 2, 3));
    }

    #[test]
    fn map_remove_by_task_stops_at_empty_bucket() {
        let mut e = Engine::new();
        e.register(0x0044_e010, no_op);
        let interface = block(&mut e, 0x1c);
        let map = block(&mut e, 0x10);
        let buckets = block(&mut e, 4);
        let task = block(&mut e, 4);
        set_word(&mut e, interface, 0, map.addr());
        set_word(&mut e, map, 0x0c, buckets.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        assert!(!fn_00c40680(&mut e, interface, 0, task.addr()));
    }

    #[test]
    fn map_remove_by_task_keeps_successor_after_first_cas_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_dd60, no_op);
        e.register(0x00c4_1f40, no_op);
        let interface = block(&mut e, 0x1c);
        let map = block(&mut e, 0x10);
        let buckets = block(&mut e, 4);
        let node = block(&mut e, 0x10);
        let successor = block(&mut e, 0x10);
        let other = block(&mut e, 0x10);
        let held = block(&mut e, 0x0c);
        let task = block(&mut e, 4);
        set_word(&mut e, interface, 0, map.addr());
        set_word(&mut e, map, 0x0c, buckets.addr());
        set_word(&mut e, buckets, 0, node.addr());
        set_word(&mut e, node, 8, held.addr());
        set_word(&mut e, node, 0x0c, successor.addr());
        set_word(&mut e, task, 0, held.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        let normalized = successor.addr();
        let mut hazard_calls = 0;
        e.register_double(0x0044_e010, move |e, a| {
            hazard_calls += 1;
            if hazard_calls >= 3 {
                e.mem.set_u32(a[0], normalized | a[1]);
            }
            Ret::default()
        });
        let node_addr = node.addr();
        let interface_addr = interface.addr();
        let other_addr = other.addr();
        let mut cas_calls = 0;
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |e, a| {
            cas_calls += 1;
            if cas_calls == 1 {
                assert_eq!((a[0], a[2]), (node_addr + 0x0c, normalized));
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(interface_addr + 0x18, other_addr);
                normalized.into_ret()
            } else {
                assert_eq!((a[1], a[2]), (normalized, node_addr));
                node_addr.into_ret()
            }
        });
        assert!(fn_00c40680(&mut e, interface, 0, task.addr()));
    }

    #[test]
    fn task_pointer_cleanup_releases_non_null_pointer() {
        let mut e = Engine::new();
        e.register(0x0044_dd60, no_op);
        let slot = block(&mut e, 4);
        set_word(&mut e, slot, 0, 0x1234);
        e.call_log = Some(vec![]);
        fn_00c40910(&mut e, slot);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x0044_dd60, vec![0x1234])));
    }

    #[test]
    fn map_delete_returns_receiver_and_restores_vtable() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let map = block(&mut e, 0x28);
        assert_eq!(fn_00c40930(&mut e, map, 0), map);
        assert_eq!(word(&e, map, 0), 0x0101_7458);
    }

    #[test]
    fn manager_delete_returns_receiver_after_destructor() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x5c);
        assert_eq!(fn_00c40960(&mut e, manager, 0), manager);
        assert_eq!(word(&e, manager, 0), 0x0101_7458);
    }

    #[test]
    fn iterator_cleanup_restores_vtable() {
        let mut e = Engine::new();
        let iterator = block(&mut e, 0x1c);
        fn_00c40990(&mut e, iterator);
        assert_eq!(word(&e, iterator, 0), 0x010c_1670);
    }

    #[test]
    fn priority_queue_delete_frees_when_flag_set() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, no_op);
        let queue = block(&mut e, 0x0c);
        e.call_log = Some(vec![]);
        assert_eq!(lock_free_priority_queue_delete(&mut e, queue, 1), queue);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, args)| { *addr == DEALLOCATE && args[1] == queue.addr() }));
    }

    #[test]
    fn priority_queue_destructor_calls_interface_virtual_delete() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, no_op);
        let queue = block(&mut e, 0x0c);
        let slots = block(&mut e, 4);
        let interface = block(&mut e, 4);
        set_word(&mut e, queue, 4, slots.addr());
        set_word(&mut e, queue, 8, 1);
        set_word(&mut e, slots, 0, interface.addr());
        e.put_vtable(0x7900_0000, &[0x7900_0004]);
        set_word(&mut e, interface, 0, 0x7900_0000);
        e.register(0x7900_0004, no_op);
        e.call_log = Some(vec![]);
        fn_00c409e0(&mut e, queue);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x7900_0004, vec![interface.addr(), 1])));
    }

    fn setup_empty_queue_cleanup(e: &mut Engine, queue: Ptr) {
        e.register(0x0040_fbf0, no_op);
        e.register(0x00c4_22d0, no_op);
        e.register(DEALLOCATE, no_op);
        let manager = block(e, 0x10);
        set_word(e, queue, 0x14, manager.addr());
        set_word(e, queue, 0x24, 1);
    }

    #[test]
    fn queue_delete_frees_when_flag_set() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x28);
        setup_empty_queue_cleanup(&mut e, queue);
        e.call_log = Some(vec![]);
        assert_eq!(lock_free_queue_delete(&mut e, queue, 1), queue);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, args)| { *addr == DEALLOCATE && args[1] == queue.addr() }));
    }

    #[test]
    fn queue_destructor_clears_manager_and_vtable() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x28);
        setup_empty_queue_cleanup(&mut e, queue);
        fn_00c40aa0(&mut e, queue);
        assert_eq!(word(&e, queue, 0x14), 0);
        assert_eq!(word(&e, queue, 0), 0x0101_7458);
    }

    #[test]
    fn iterator_clear_key_zeros_both_words() {
        let mut e = Engine::new();
        let iterator = block(&mut e, 0x18);
        set_word(&mut e, iterator, 0x10, 2);
        set_word(&mut e, iterator, 0x14, 3);
        lock_free_map_iterator_clear_key(&mut e, iterator);
        assert_eq!(e.mem.u64(iterator.addr() + 0x10), 0);
    }

    #[test]
    fn iterator_delete_frees_when_flag_set() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, no_op);
        let iterator = block(&mut e, 0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(lock_free_map_iterator_delete(&mut e, iterator, 1), iterator);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, args)| { *addr == DEALLOCATE && args[1] == iterator.addr() }));
    }

    #[test]
    fn map_clear_drops_manager_and_zeroes_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        e.register(0x00fd_f050, no_op);
        e.register(DEALLOCATE, no_op);
        let map = block(&mut e, 0x28);
        let manager = block(&mut e, 0x10);
        set_word(&mut e, map, 0x14, manager.addr());
        set_word(&mut e, map, 0x24, 1);
        fn_00c40b70(&mut e, map, true);
        assert_eq!(word(&e, map, 0x14), 0);
        assert_eq!(word(&e, map, 0x24), 0);
    }

    #[test]
    fn exhausted_iterator_stays_exhausted_and_releases_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        let map = block(&mut e, 0x28);
        let iterator = block(&mut e, 0x1c);
        let key = block(&mut e, 8);
        let task = block(&mut e, 4);
        e.mem.set_u8(iterator.addr() + 0x18, 2);
        set_word(&mut e, map, 0x24, 1);
        assert!(!fn_00c40e70(&mut e, map, iterator, key, task, true));
        assert_eq!(word(&e, map, 0x24), 0);
    }

    #[test]
    fn iterator_selects_first_bucket_path_before_tls_callback() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        e.register_double(0x00c4_16a0, |_, _| {
            panic!("the continuation path ran after TLS changed the iterator flags")
        });
        let map = block(&mut e, 0x28);
        let manager = block(&mut e, 8);
        let interface = block(&mut e, 0x1c);
        let buckets = block(&mut e, 4);
        let iterator = block(&mut e, 0x1c);
        let key = block(&mut e, 8);
        let task = block(&mut e, 4);
        set_word(&mut e, map, 8, 1);
        set_word(&mut e, map, 0x0c, buckets.addr());
        set_word(&mut e, map, 0x14, manager.addr());
        set_word(&mut e, map, 0x24, 1);
        set_word(&mut e, manager, 4, 7);
        set_word(&mut e, interface, 0, map.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        let iterator_addr = iterator.addr();
        let interface_addr = interface.addr();
        e.register_double(0x00fd_f044, move |e, _| {
            e.mem.set_u8(iterator_addr + 0x18, 1);
            interface_addr.into_ret()
        });
        assert!(!fn_00c40e70(&mut e, map, iterator, key, task, true));
        assert_eq!(e.mem.u8(iterator.addr() + 0x18) & 2, 2);
    }

    #[test]
    fn pointer_assignment_reloads_source_after_release() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let holder = block(&mut e, 4);
        let source = block(&mut e, 4);
        let first = block(&mut e, 0x0c);
        let second = block(&mut e, 0x0c);
        set_word(&mut e, holder, 0, first.addr());
        set_word(&mut e, source, 0, first.addr() + 4);
        let source_addr = source.addr();
        let replacement = second.addr();
        e.register_double(0x0044_dd60, move |e, _| {
            e.mem.set_u32(source_addr, replacement);
            Ret::default()
        });
        assert_eq!(fn_00c41050(&mut e, holder, source), holder);
        assert_eq!(word(&e, holder, 0), replacement);
    }

    #[test]
    fn task_thread_stops_when_manager_requests_shutdown() {
        let mut e = Engine::new();
        let thread = block(&mut e, 0x34);
        let manager = block(&mut e, 0x44);
        set_word(&mut e, thread, 0x30, manager.addr());
        e.mem.set_u8(manager.addr() + 0x40, 1);
        bs_task_manager_thread_update(&mut e, thread);
    }

    #[test]
    fn task_thread_reloads_task_between_virtual_callbacks() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0040_fbf0, no_op);
        e.register(0x0044_dd60, no_op);
        e.register(0x00fd_f1c8, no_op);
        e.register(0x00fd_f1b4, no_op);
        let thread = block(&mut e, 0x34);
        let manager = block(&mut e, 0x44);
        let tls_manager = block(&mut e, 0x10);
        let interface = block(&mut e, 0x1c);
        let buckets = block(&mut e, 4);
        let node = block(&mut e, 0x10);
        let first_task = block(&mut e, 0x10);
        let second_task = block(&mut e, 0x10);
        set_word(&mut e, thread, 0x30, manager.addr());
        set_word(&mut e, manager, 8, 1);
        set_word(&mut e, manager, 0x14, tls_manager.addr());
        set_word(&mut e, manager, 0x24, 1);
        set_word(&mut e, manager, 0x0c, buckets.addr());
        set_word(&mut e, buckets, 0, node.addr());
        set_word(&mut e, interface, 0, manager.addr());
        set_word(&mut e, node, 8, first_task.addr());
        set_word(&mut e, first_task, 0x0c, 1);
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        let mut slots = vec![0; 0x54 / 4];
        slots[0x30 / 4] = 0x7a30_0030;
        slots[0x4c / 4] = 0x7a30_004c;
        slots[0x50 / 4] = 0x7a30_0050;
        e.put_vtable(0x7a30_0000, &slots);
        set_word(&mut e, manager, 0, 0x7a30_0000);
        e.register(0x7a30_0030, no_op);
        let interface_addr = interface.addr();
        e.register_double(0x00fd_f044, move |_, _| interface_addr.into_ret());
        // The update's 0x30-byte local (the iterator, the key at +0x20, the task slot at +0x28).
        let scratch = e.next_stack_local(0x30);
        let second_addr = second_task.addr();
        e.register_double(0x7a30_004c, move |e, a| {
            assert_eq!(a[1], first_task.addr());
            e.mem.set_u32(scratch + 0x28, second_addr);
            Ret::default()
        });
        let manager_addr = manager.addr();
        e.register_double(0x7a30_0050, move |e, a| {
            assert_eq!(a[1], second_addr);
            e.mem.set_u8(scratch + 0x18, 2);
            e.mem.set_u8(manager_addr + 0x40, 1);
            Ret::default()
        });
        bs_task_manager_thread_update(&mut e, thread);
    }

    #[test]
    fn queue_clear_drops_empty_manager_and_restores_tls() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x28);
        setup_empty_queue_cleanup(&mut e, queue);
        let tls = e.tls() + 0x2b4;
        e.mem.set_u32(tls, 19);
        fn_00c41330(&mut e, queue, true);
        assert_eq!(word(&e, queue, 0x14), 0);
        assert_eq!(word(&e, queue, 0x24), 0);
        assert_eq!(e.mem.u32(tls), 19);
    }

    #[test]
    fn queue_clear_keeps_captured_successor_but_reloads_head_for_delete() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x28);
        setup_empty_queue_cleanup(&mut e, queue);
        let first = block(&mut e, 8);
        let successor = block(&mut e, 8);
        let replacement = block(&mut e, 8);
        set_word(&mut e, queue, 4, first.addr());
        set_word(&mut e, first, 0, successor.addr());
        set_word(&mut e, first, 4, 0x1234);
        let queue_addr = queue.addr();
        let replacement_addr = replacement.addr();
        let mut changed = false;
        e.register_double(0x0044_dd60, move |e, _| {
            if !changed {
                e.mem.set_u32(queue_addr + 4, replacement_addr);
                changed = true;
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c41330(&mut e, queue, true);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(DEALLOCATE, vec![0x011f_6238, replacement.addr()])));
        assert!(log.contains(&(DEALLOCATE, vec![0x011f_6238, successor.addr()])));
        assert!(!log.contains(&(DEALLOCATE, vec![0x011f_6238, first.addr()])));
        assert_eq!(word(&e, queue, 4), 0);
    }

    #[test]
    fn interface_allocation_publishes_slot_and_thread_id() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let manager = block(&mut e, 0x10);
        let owner = block(&mut e, 4);
        let slots = block(&mut e, 8);
        let replacement_slots = block(&mut e, 8);
        let interface = block(&mut e, 4);
        set_word(&mut e, manager, 0, 1);
        set_word(&mut e, manager, 4, 9);
        set_word(&mut e, manager, 8, slots.addr());
        e.put_vtable(0x7f00_0000, &[0, 0x7f00_0004]);
        set_word(&mut e, owner, 0, 0x7f00_0000);
        let interface_addr = interface.addr();
        e.register_double(0x7f00_0004, move |_, _| interface_addr.into_ret());
        let manager_addr = manager.addr();
        let replacement_addr = replacement_slots.addr();
        e.register_double(0x00fd_f1d0, move |e, _| {
            e.mem.set_u32(manager_addr + 8, replacement_addr);
            44u32.into_ret()
        });
        e.register(0x00fd_f04c, no_op);
        assert_eq!(fn_00c41610(&mut e, manager, owner), interface);
        assert_eq!(word(&e, replacement_slots, 0), 44);
        assert_eq!(word(&e, slots, 4), interface.addr());
    }

    #[test]
    fn map_search_stops_at_empty_bucket() {
        let mut e = Engine::new();
        e.register(0x0044_e010, no_op);
        let interface = block(&mut e, 0x1c);
        let map = block(&mut e, 0x10);
        let buckets = block(&mut e, 4);
        let task = block(&mut e, 4);
        set_word(&mut e, interface, 0, map.addr());
        set_word(&mut e, map, 0x0c, buckets.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        assert!(!fn_00c41ac0(&mut e, interface, 0, task));
    }

    #[test]
    fn map_search_reloads_successor_after_release_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_e010, no_op);
        let interface = block(&mut e, 0x1c);
        let map = block(&mut e, 0x10);
        let buckets = block(&mut e, 4);
        let node = block(&mut e, 0x10);
        let successor = block(&mut e, 0x10);
        let held = block(&mut e, 0x0c);
        let target = block(&mut e, 4);
        set_word(&mut e, interface, 0, map.addr());
        set_word(&mut e, map, 0x0c, buckets.addr());
        set_word(&mut e, buckets, 0, node.addr());
        set_word(&mut e, node, 8, held.addr());
        set_word(&mut e, node, 0x0c, successor.addr());
        set_word(&mut e, target, 0, 9);
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        let interface_addr = interface.addr();
        e.register_double(0x0044_dd60, move |e, _| {
            e.mem.set_u32(interface_addr + 0x18, 0);
            Ret::default()
        });
        assert!(!fn_00c41ac0(&mut e, interface, 0, target));
        assert_eq!(word(&e, interface, 0x14), 0);
    }

    #[test]
    fn iterator_reset_clears_both_flags() {
        let mut e = Engine::new();
        let iterator = block(&mut e, 0x1c);
        e.put_vtable(0x7e00_0000, &[0, 0x7e00_0004]);
        set_word(&mut e, iterator, 0, 0x7e00_0000);
        e.register(0x7e00_0004, no_op);
        e.mem.set_u8(iterator.addr() + 0x18, 7);
        fn_00c42060(&mut e, iterator, 3);
        assert_eq!(word(&e, iterator, 8), 3);
        assert_eq!(e.mem.u8(iterator.addr() + 0x18), 4);
    }

    #[test]
    fn tls_manager_destroys_slots_and_frees_index() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, no_op);
        e.register(0x00fd_f050, no_op);
        let manager = block(&mut e, 0x10);
        set_word(&mut e, manager, 4, 7);
        e.call_log = Some(vec![]);
        fn_00c420d0(&mut e, manager);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x00fd_f050, vec![7])));
    }

    #[test]
    fn first_bucket_node_empty_clears_hazards() {
        let mut e = Engine::new();
        let interface = block(&mut e, 0x1c);
        let map = block(&mut e, 0x10);
        let buckets = block(&mut e, 4);
        let key = block(&mut e, 8);
        let task = block(&mut e, 4);
        set_word(&mut e, interface, 0, map.addr());
        set_word(&mut e, map, 0x0c, buckets.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
            set_word(&mut e, hazard, 0, 55);
        }
        assert!(!fn_00c42380(&mut e, interface, 0, key, task));
        for offset in [4, 8, 0x0c] {
            assert_eq!(e.mem.u32(word(&e, interface, offset)), 0);
        }
    }

    #[test]
    fn next_bucket_node_missing_clears_hazards() {
        let mut e = Engine::new();
        e.register(0x00c4_16a0, no_op);
        let interface = block(&mut e, 0x1c);
        let key = block(&mut e, 8);
        let task = block(&mut e, 4);
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
            set_word(&mut e, hazard, 0, 55);
        }
        assert!(!fn_00c42560(&mut e, interface, 0, key, task));
        for offset in [4, 8, 0x0c] {
            assert_eq!(e.mem.u32(word(&e, interface, offset)), 0);
        }
    }

    #[test]
    fn next_bucket_node_assigns_unmarked_successor() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x00c4_16a0, one);
        let interface = block(&mut e, 0x1c);
        let owner = block(&mut e, 4);
        let node = block(&mut e, 0x10);
        let held = block(&mut e, 0x0c);
        let key = block(&mut e, 8);
        let task = block(&mut e, 4);
        let mut slots = vec![0; 0x34 / 4];
        slots[0x30 / 4] = 0x7a40_0030;
        e.put_vtable(0x7a40_0000, &slots);
        e.register(0x7a40_0030, no_op);
        set_word(&mut e, owner, 0, 0x7a40_0000);
        set_word(&mut e, interface, 0, owner.addr());
        set_word(&mut e, interface, 0x18, node.addr());
        set_word(&mut e, node, 8, held.addr());
        for offset in [4, 8, 0x0c] {
            let hazard = block(&mut e, 4);
            set_word(&mut e, interface, offset, hazard.addr());
        }
        assert!(fn_00c42560(&mut e, interface, 0, key, task));
        assert_eq!(word(&e, task, 0), held.addr());
    }
}
