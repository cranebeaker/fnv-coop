//! `BSHavok/bhkmultithreadedsystem.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkMultithreadedSystem` owns the Havok job queue (`+0x14`, a 0x1c0-byte
//! object) and job thread pool (`+0x18`, a 0x148-byte object) that
//! `InitMTSystem` builds and `00cbf2f0` tears down. Both functions mark the
//! thread block's word at `+0x2b4` with 0x10 while they run (the same
//! allocator-context word the audio methods set) and restore it afterwards.

#[allow(unused_imports)]
use crate::prelude::*;

/// The word of the thread block (`e.tls()`) that these methods set while they run.
const THREAD_CONTEXT: u32 = 0x2b4;
/// The value stored in [`THREAD_CONTEXT`] while `InitMTSystem` and the teardown run.
const THREAD_CONTEXT_VALUE: u32 = 0x10;
/// `00c85750()`: the object whose `+0x10` is the heap allocator Havok allocates through.
const HAVOK_ALLOCATOR_HOLDER: u32 = 0x00c8_5750;
/// Allocator vtable slot `+0x04`: `allocate(size)`.
const ALLOCATOR_ALLOCATE: u32 = 0x04;
/// Allocator vtable slot `+0x08`: `free(block, size)` (the job queue's block).
const ALLOCATOR_FREE_SIZED: u32 = 0x08;
/// Allocator vtable slot `+0x10`: `free(block, size)` (array storage).
const ALLOCATOR_FREE_ARRAY: u32 = 0x10;
/// `00d055c0(queue)`: the job queue's cleanup before it is freed.
const JOB_QUEUE_CLEANUP: u32 = 0x00d0_55c0;
/// `00c905b0(pool)`: the job thread pool's cleanup.
const JOB_THREAD_POOL_CLEANUP: u32 = 0x00c9_05b0;
/// `0045cfe0(this, name, flag)`: the `SettingT` constructor that
/// `bhkMultithreadedSystem` starts from.
const SETTING_CONSTRUCTOR: u32 = 0x0045_cfe0;
/// The setting's name argument (address of the `iNumThreads` text).
const NUM_THREADS_SETTING_NAME: u32 = 0x010c_9244;
/// `0044f4f0(this)`: the destructor of the `SettingT` base.
const SETTING_DESTRUCTOR: u32 = 0x0044_f4f0;
/// `00d2d380(out)`: constructs the job thread pool's creation info (0x20 bytes).
const JOB_POOL_INFO_CONSTRUCTOR: u32 = 0x00d2_d380;
/// `00d2d110(thread_pool, info)`: constructs the 0x148-byte job thread pool; returns it.
const JOB_THREAD_POOL_CONSTRUCTOR: u32 = 0x00d2_d110;
/// `00d05450(out)`: constructs the 0x1c-byte job queue creation info.
const JOB_QUEUE_INFO_CONSTRUCTOR: u32 = 0x00d0_5450;
/// `00d05490(queue, info)`: constructs the 0x1c0-byte job queue; returns it.
const JOB_QUEUE_CONSTRUCTOR: u32 = 0x00d0_5490;
/// `00d2cca0(queue)` (cdecl): stores the job function address in a run of globals.
const JOB_QUEUE_INSTALL: u32 = 0x00d2_cca0;
/// `GetCurrentThreadId`, through its import slot.
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;

layout! {
    /// `bhkMultithreadedSystem` (Xbox PDB), 0x1c bytes.
    pub struct BhkMultithreadedSystem: 0x1c {
        /// The word at +4 of the `SettingT` base (`iNumThreads`, Xbox PDB):
        /// the first word of the job thread pool's creation info.
        0x04 iNumThreadsValue: u32,
        /// `uiMTResourceOwner` (Xbox PDB): the id of the thread that holds the multithreaded resources.
        0x0c uiMTResourceOwner: u32,
        /// `pCurrentMTWorld` (Xbox PDB): `bhkWorld*`.
        0x10 pCurrentMTWorld: Ptr,
        /// `pJobQueue` (Xbox PDB): `hkJobQueue*`, 0x1c0 bytes.
        0x14 pJobQueue: Ptr,
        /// `pJobThreadPool` (Xbox PDB): `hkJobThreadPool*`, 0x148 bytes.
        0x18 pJobThreadPool: Ptr,
    }
}

/// The allocator `00c85750()` hands out: `[holder + 0x10]`.
fn havok_allocator(e: &mut Engine) -> u32 {
    let holder = e.call(HAVOK_ALLOCATOR_HOLDER, &args![]).u32();
    e.mem.u32(holder + 0x10)
}

/// Frees the storage of an `hkArray` of 4-byte elements the way the inlined
/// destructors do: when the capacity word is not negative, `storage` is
/// released through the array-free slot with `(capacity & 0x3fffffff) * 4`
/// bytes.
fn free_word_array(e: &mut Engine, allocator: u32, storage: u32, capacity_word: i32) {
    if capacity_word >= 0 {
        let size = ((capacity_word as u32) & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, ALLOCATOR_FREE_ARRAY, &args![storage, size]);
    }
}

// Translated from 00cbf2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the multithreaded system's job queue and job thread pool when
/// `uiMTResourceOwner` is set: cleans up and frees the queue (0x1c0 bytes),
/// cleans up the pool, clears both pointers and the owner. Runs with the
/// thread block's `+0x2b4` set to 0x10 and restores it.
// Translation incomplete: 00cbf2f0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00cbf2f0(e: &mut Engine, this: Ptr<BhkMultithreadedSystem>) {
    let context = e.tls() + THREAD_CONTEXT;
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, THREAD_CONTEXT_VALUE);
    if e.get(this, BhkMultithreadedSystem::uiMTResourceOwner) != 0 {
        let queue = e.get(this, BhkMultithreadedSystem::pJobQueue);
        if !queue.is_null() {
            e.call(JOB_QUEUE_CLEANUP, &args![queue]);
            let allocator = havok_allocator(e);
            e.vcall(allocator, ALLOCATOR_FREE_SIZED, &args![queue, 0x1c0u32]);
        }
        let pool = e.get(this, BhkMultithreadedSystem::pJobThreadPool);
        e.set(this, BhkMultithreadedSystem::pJobQueue, Ptr::NULL);
        if !pool.is_null() {
            e.call(JOB_THREAD_POOL_CLEANUP, &args![pool]);
        }
        e.set(this, BhkMultithreadedSystem::pJobThreadPool, Ptr::NULL);
    }
    e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 0);
    e.mem.set_u32(context, previous);
}

// Translated from 00cbf400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMultithreadedSystem::bhkMultithreadedSystem` (Xbox PDB): constructs
/// the `SettingT` base (`0045cfe0`, name at `010c9244`, flag 1) and clears
/// the owner, world, queue and pool. Returns `this`.
pub fn bhk_multithreaded_system_bhk_multithreaded_system(
    e: &mut Engine,
    this: Ptr<BhkMultithreadedSystem>,
) -> Ptr<BhkMultithreadedSystem> {
    e.call(
        SETTING_CONSTRUCTOR,
        &args![this, NUM_THREADS_SETTING_NAME, 1u32],
    );
    e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 0);
    e.set(this, BhkMultithreadedSystem::pCurrentMTWorld, Ptr::NULL);
    e.set(this, BhkMultithreadedSystem::pJobQueue, Ptr::NULL);
    e.set(this, BhkMultithreadedSystem::pJobThreadPool, Ptr::NULL);
    this
}

// Translated from 00cbf430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkMultithreadedSystem` destructor body: releases the job queue and
/// pool (`00cbf2f0`), then runs the `SettingT` base destructor (`0044f4f0`).
// Translation incomplete: 00cbf430 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00cbf430(e: &mut Engine, this: Ptr<BhkMultithreadedSystem>) {
    fn_00cbf2f0(e, this);
    e.call(SETTING_DESTRUCTOR, &args![this]);
}

// Translated from 00cbf490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees an `hkArray` of `hkArray<4-byte>` that sits at `+0xc` of `this`
/// (data `+0xc`, size `+0x10`, capacity word `+0x14`; elements are 12 bytes:
/// data, size, capacity word). Every element's size is cleared and its
/// storage freed (unless its capacity word is negative), then data and
/// capacity are reset; the outer storage is freed the same way (12 bytes per
/// capacity unit) and reset.
pub fn fn_00cbf490(e: &mut Engine, this: Ptr) {
    let allocator = havok_allocator(e);
    let data = e.mem.u32(this.addr() + 0xc);
    let count = e.mem.i32(this.addr() + 0x10);
    let mut element = data;
    for _ in 0..count.max(0) {
        let element_allocator = havok_allocator(e);
        let capacity = e.mem.i32(element + 8);
        e.mem.set_u32(element + 4, 0);
        let storage = e.mem.u32(element);
        free_word_array(e, element_allocator, storage, capacity);
        e.mem.set_u32(element, 0);
        e.mem.set_u32(element + 8, 0x8000_0000);
        element = element.wrapping_add(0xc);
    }
    let capacity = e.mem.i32(this.addr() + 0x14);
    e.mem.set_u32(this.addr() + 0x10, 0);
    if capacity >= 0 {
        let size = ((capacity as u32) & 0x3fff_ffff).wrapping_mul(12);
        e.vcall(allocator, ALLOCATOR_FREE_ARRAY, &args![data, size]);
    }
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x14, 0x8000_0000);
}

// Translated from 00cbf540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMultithreadedSystem::InitMTSystem` (Xbox PDB): releases any previous
/// queue and pool, records the id at `*thread_id` as the resource owner, then
/// builds the 0x148-byte job thread pool (from a creation info whose first
/// word is the setting's value) and the 0x1c0-byte job queue (from a
/// creation info whose capacity field is 10), installs the queue's job
/// functions, and frees the temporaries. Runs with the thread block's
/// `+0x2b4` set to 0x10 and restores it. The second stack word is never read.
// Translation incomplete: 00cbf540 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bhk_multithreaded_system_init_mt_system(
    e: &mut Engine,
    this: Ptr<BhkMultithreadedSystem>,
    thread_id: Ptr,
    _unused_1: u32,
) {
    let context = e.tls() + THREAD_CONTEXT;
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, THREAD_CONTEXT_VALUE);
    fn_00cbf2f0(e, this);
    let owner = e.mem.u32(thread_id.addr());
    e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, owner);

    // The pool's creation info: 0x20 bytes, first word the setting's value;
    // its `hkArray` of words is at +0x10 (data), +0x14 (size), +0x18 (capacity word).
    // The exe's frame (exception record, `SUB ESP,0x40`, three pushes and the cookie): 0x5c
    // bytes, the pool info at +0x30 (00cbf5a0) and the queue info at +0x14 (00cbf5ea).
    e.with_stack(0x5c, |e, frame| {
        let frame = frame.addr();
        let pool_info = frame + 0x30;
        e.call(JOB_POOL_INFO_CONSTRUCTOR, &args![pool_info]);
        let num_threads = e.get(this, BhkMultithreadedSystem::iNumThreadsValue);
        e.mem.set_u32(pool_info, num_threads);

        let allocator = havok_allocator(e);
        let pool = e
            .vcall(allocator, ALLOCATOR_ALLOCATE, &args![0x148u32])
            .u32();
        e.mem.set_u16(pool + 4, 0x148);
        let pool = e
            .call(JOB_THREAD_POOL_CONSTRUCTOR, &args![pool, pool_info])
            .u32();
        e.set(this, BhkMultithreadedSystem::pJobThreadPool, Ptr::new(pool));

        // The queue's creation info: 0x1c bytes, capacity field (+0x18) 10; its
        // `hkArray` of 12-byte elements is at +0xc.
        let queue_info = frame + 0x14;
        e.call(JOB_QUEUE_INFO_CONSTRUCTOR, &args![queue_info]);
        e.mem.set_u32(queue_info + 0x18, 0xa);

        let allocator = havok_allocator(e);
        let queue = e
            .vcall(allocator, ALLOCATOR_ALLOCATE, &args![0x1c0u32])
            .u32();
        let queue = if queue != 0 {
            e.call(JOB_QUEUE_CONSTRUCTOR, &args![queue, queue_info])
                .u32()
        } else {
            0
        };
        e.set(this, BhkMultithreadedSystem::pJobQueue, Ptr::new(queue));
        e.call(JOB_QUEUE_INSTALL, &args![queue]);

        fn_00cbf490(e, Ptr::new(queue_info));

        // The pool info's word array is released without being reset.
        let allocator = havok_allocator(e);
        let storage = e.mem.u32(pool_info + 0x10);
        let capacity = e.mem.i32(pool_info + 0x18);
        e.mem.set_u32(pool_info + 0x14, 0);
        free_word_array(e, allocator, storage, capacity);
    });
    e.mem.set_u32(context, previous);
}

// Translated from 00cbf6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMultithreadedSystem::AcquireMTResources` (Xbox PDB): when the calling
/// thread is not already the resource owner, runs `InitMTSystem` with a
/// local holding the thread id (and passes on its one stack argument).
pub fn bhk_multithreaded_system_acquire_mt_resources(
    e: &mut Engine,
    this: Ptr<BhkMultithreadedSystem>,
    arg: u32,
) {
    let current = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    if e.get(this, BhkMultithreadedSystem::uiMTResourceOwner) != current {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), current);
            bhk_multithreaded_system_init_mt_system(e, this, local, arg);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbf2f0, fn_00cbf2f0(Ptr<BhkMultithreadedSystem>)),
        entry!(
            0x00cbf400,
            bhk_multithreaded_system_bhk_multithreaded_system(
                Ptr<BhkMultithreadedSystem>,
            )
                -> Ptr<BhkMultithreadedSystem>
        ),
        entry!(0x00cbf430, fn_00cbf430(Ptr<BhkMultithreadedSystem>)),
        entry!(0x00cbf490, fn_00cbf490(Ptr)),
        entry!(
            0x00cbf540,
            bhk_multithreaded_system_init_mt_system(Ptr<BhkMultithreadedSystem>, Ptr, u32)
        ),
        entry!(
            0x00cbf6b0,
            bhk_multithreaded_system_acquire_mt_resources(Ptr<BhkMultithreadedSystem>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const HOLDER: u32 = 0x7000_0000;
    const ALLOCATOR: u32 = 0x7000_1000;
    const ALLOCATOR_VTABLE: u32 = 0x7000_2000;
    const ALLOCATE_DOUBLE: u32 = 0x7100_0004;
    const FREE_SIZED_DOUBLE: u32 = 0x7100_0008;
    const FREE_ARRAY_DOUBLE: u32 = 0x7100_0010;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The allocator holder (`00c85750`) with an allocator whose slots are doubles.
    fn allocator_fixture() -> Engine {
        let mut e = Engine::new();
        e.map(HOLDER, 0x20);
        e.mem.set_u32(HOLDER + 0x10, ALLOCATOR);
        e.map(ALLOCATOR, 8);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, ALLOCATE_DOUBLE, FREE_SIZED_DOUBLE, 0, FREE_ARRAY_DOUBLE],
        );
        e.register(HAVOK_ALLOCATOR_HOLDER, |_, _| Ret {
            eax: HOLDER,
            ..Ret::default()
        });
        e.register(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(FREE_SIZED_DOUBLE, |_, _| Ret::default());
        e.register(FREE_ARRAY_DOUBLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn teardown_frees_queue_and_pool_only_when_owned() {
        let mut e = allocator_fixture();
        e.register(JOB_QUEUE_CLEANUP, |_, _| Ret::default());
        e.register(JOB_THREAD_POOL_CLEANUP, |_, _| Ret::default());
        let this = e.new_object::<BhkMultithreadedSystem>();
        e.set(this, BhkMultithreadedSystem::pJobQueue, Ptr::new(0x5000));
        e.set(
            this,
            BhkMultithreadedSystem::pJobThreadPool,
            Ptr::new(0x6000),
        );
        let context = e.tls() + THREAD_CONTEXT;
        e.mem.set_u32(context, 0x77);

        // Not owned: nothing is released, the owner stays clear.
        e.call(0x00cbf2f0, &args![this]);
        assert!(calls_to(&e, JOB_QUEUE_CLEANUP).is_empty());
        assert_eq!(
            e.get(this, BhkMultithreadedSystem::pJobQueue).addr(),
            0x5000
        );
        assert_eq!(e.mem.u32(context), 0x77);

        e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 9);
        e.call(0x00cbf2f0, &args![this]);
        assert_eq!(calls_to(&e, JOB_QUEUE_CLEANUP), vec![vec![0x5000]]);
        assert_eq!(
            calls_to(&e, FREE_SIZED_DOUBLE),
            vec![vec![ALLOCATOR, 0x5000, 0x1c0]]
        );
        assert_eq!(calls_to(&e, JOB_THREAD_POOL_CLEANUP), vec![vec![0x6000]]);
        assert!(e.get(this, BhkMultithreadedSystem::pJobQueue).is_null());
        assert!(e
            .get(this, BhkMultithreadedSystem::pJobThreadPool)
            .is_null());
        assert_eq!(e.get(this, BhkMultithreadedSystem::uiMTResourceOwner), 0);
        assert_eq!(e.mem.u32(context), 0x77);
    }

    #[test]
    fn teardown_skips_missing_queue_and_pool() {
        let mut e = allocator_fixture();
        let this = e.new_object::<BhkMultithreadedSystem>();
        e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 9);
        e.call(0x00cbf2f0, &args![this]);
        assert!(calls_to(&e, JOB_QUEUE_CLEANUP).is_empty());
        assert!(calls_to(&e, JOB_THREAD_POOL_CLEANUP).is_empty());
        assert_eq!(e.get(this, BhkMultithreadedSystem::uiMTResourceOwner), 0);
    }

    #[test]
    fn constructor_builds_the_setting_and_clears_the_fields() {
        let mut e = allocator_fixture();
        e.register(SETTING_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<BhkMultithreadedSystem>();
        e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 1);
        e.set(this, BhkMultithreadedSystem::pCurrentMTWorld, Ptr::new(2));
        e.set(this, BhkMultithreadedSystem::pJobQueue, Ptr::new(3));
        e.set(this, BhkMultithreadedSystem::pJobThreadPool, Ptr::new(4));
        let result = e.call(0x00cbf400, &args![this]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(
            calls_to(&e, SETTING_CONSTRUCTOR),
            vec![vec![this.addr(), 0x010c_9244, 1]]
        );
        assert_eq!(e.get(this, BhkMultithreadedSystem::uiMTResourceOwner), 0);
        assert!(e
            .get(this, BhkMultithreadedSystem::pCurrentMTWorld)
            .is_null());
        assert!(e.get(this, BhkMultithreadedSystem::pJobQueue).is_null());
        assert!(e
            .get(this, BhkMultithreadedSystem::pJobThreadPool)
            .is_null());
    }

    #[test]
    fn destructor_releases_then_runs_the_base_destructor() {
        let mut e = allocator_fixture();
        e.register(SETTING_DESTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<BhkMultithreadedSystem>();
        e.call(0x00cbf430, &args![this]);
        assert_eq!(calls_to(&e, SETTING_DESTRUCTOR), vec![vec![this.addr()]]);
    }

    #[test]
    fn array_of_arrays_frees_unflagged_storage_and_resets() {
        let mut e = allocator_fixture();
        let this = e.mem.alloc(0x20);
        let elements = e.mem.alloc(0x24);
        // Element 0: owned storage, capacity 3 words. Element 1: borrowed
        // (negative capacity word). Element 2: owned, capacity 0.
        e.mem.set_u32(elements, 0xa000);
        e.mem.set_u32(elements + 4, 2);
        e.mem.set_u32(elements + 8, 3);
        e.mem.set_u32(elements + 0xc, 0xb000);
        e.mem.set_u32(elements + 0x10, 1);
        e.mem.set_u32(elements + 0x14, 0x8000_0004);
        e.mem.set_u32(elements + 0x18, 0xc000);
        e.mem.set_u32(elements + 0x1c, 0);
        e.mem.set_u32(elements + 0x20, 0);
        e.mem.set_u32(this + 0xc, elements);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 4);
        e.call(0x00cbf490, &args![this]);
        assert_eq!(
            calls_to(&e, FREE_ARRAY_DOUBLE),
            vec![
                vec![ALLOCATOR, 0xa000, 12],
                vec![ALLOCATOR, 0xc000, 0],
                vec![ALLOCATOR, elements, 48],
            ]
        );
        for element in 0..3 {
            let at = elements + 0xc * element;
            assert_eq!(e.mem.u32(at), 0);
            assert_eq!(e.mem.u32(at + 4), 0);
            assert_eq!(e.mem.u32(at + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn array_of_arrays_with_no_elements_and_flagged_storage_frees_nothing() {
        let mut e = allocator_fixture();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x9000);
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_u32(this + 0x14, 0x8000_0002);
        e.call(0x00cbf490, &args![this]);
        assert!(calls_to(&e, FREE_ARRAY_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn init_builds_the_pool_and_the_queue() {
        let mut e = allocator_fixture();
        let context = e.tls() + THREAD_CONTEXT;
        e.mem.set_u32(context, 0x55);
        let seen_context = Rc::new(Cell::new(0));
        let seen_capacity = Rc::new(Cell::new(0));
        let seen_threads = Rc::new(Cell::new(0));
        e.register(JOB_POOL_INFO_CONSTRUCTOR, |e, a| {
            // The pool info's word array: storage 0xd000, capacity word 2.
            e.mem.set_u32(a[0] + 0x10, 0xd000);
            e.mem.set_u32(a[0] + 0x18, 2);
            Ret::default()
        });
        let threads = seen_threads.clone();
        e.register_double(JOB_THREAD_POOL_CONSTRUCTOR, move |e, a| {
            threads.set(e.mem.u32(a[1]));
            assert_eq!(e.mem.u16(a[0] + 4), 0x148);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(JOB_QUEUE_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x8000_0000);
            Ret::default()
        });
        let capacity = seen_capacity.clone();
        e.register_double(JOB_QUEUE_CONSTRUCTOR, move |e, a| {
            capacity.set(e.mem.u32(a[1] + 0x18));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let context_during = seen_context.clone();
        e.register_double(JOB_QUEUE_INSTALL, move |e, _| {
            let at = e.tls() + THREAD_CONTEXT;
            context_during.set(e.mem.u32(at));
            Ret::default()
        });

        let this = e.new_object::<BhkMultithreadedSystem>();
        e.set(this, BhkMultithreadedSystem::iNumThreadsValue, 6);
        let thread_id = e.mem.alloc(4);
        e.mem.set_u32(thread_id, 0x1234);
        e.call(0x00cbf540, &args![this, thread_id, 0u32]);

        assert_eq!(
            e.get(this, BhkMultithreadedSystem::uiMTResourceOwner),
            0x1234
        );
        let pool = e.get(this, BhkMultithreadedSystem::pJobThreadPool);
        let queue = e.get(this, BhkMultithreadedSystem::pJobQueue);
        assert!(!pool.is_null() && !queue.is_null());
        assert_eq!(seen_threads.get(), 6);
        assert_eq!(seen_capacity.get(), 0xa);
        assert_eq!(seen_context.get(), 0x10);
        assert_eq!(e.mem.u32(context), 0x55);
        assert_eq!(calls_to(&e, JOB_QUEUE_INSTALL), vec![vec![queue.addr()]]);
        let allocations = calls_to(&e, ALLOCATE_DOUBLE);
        assert_eq!(
            allocations,
            vec![vec![ALLOCATOR, 0x148], vec![ALLOCATOR, 0x1c0]]
        );
        // The pool info's word array is freed (2 words = 8 bytes).
        assert_eq!(
            calls_to(&e, FREE_ARRAY_DOUBLE),
            vec![vec![ALLOCATOR, 0xd000, 8]]
        );
    }

    #[test]
    fn init_with_a_failed_queue_allocation_installs_a_null_queue() {
        let mut e = allocator_fixture();
        e.register(JOB_POOL_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x18, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_THREAD_POOL_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(JOB_QUEUE_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_QUEUE_INSTALL, |_, _| Ret::default());
        // The 0x1c0-byte allocation fails.
        e.register(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: if a[1] == 0x1c0 { 0 } else { e.mem.alloc(a[1]) },
            ..Ret::default()
        });
        let this = e.new_object::<BhkMultithreadedSystem>();
        let thread_id = e.mem.alloc(4);
        e.call(0x00cbf540, &args![this, thread_id, 0u32]);
        assert!(calls_to(&e, JOB_QUEUE_CONSTRUCTOR).is_empty());
        assert!(e.get(this, BhkMultithreadedSystem::pJobQueue).is_null());
        assert_eq!(calls_to(&e, JOB_QUEUE_INSTALL), vec![vec![0]]);
    }

    #[test]
    fn init_keeps_both_infos_at_the_exe_frame_offsets() {
        let mut e = allocator_fixture();
        e.register(JOB_POOL_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x18, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_THREAD_POOL_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(JOB_QUEUE_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_QUEUE_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(JOB_QUEUE_INSTALL, |_, _| Ret::default());
        let this = e.new_object::<BhkMultithreadedSystem>();
        let thread_id = e.mem.alloc(4);
        let frame = e.next_stack_local(0x5c);
        e.call(0x00cbf540, &args![this, thread_id, 0u32]);
        // The pool info at +0x30, the queue info at +0x14.
        assert_eq!(
            calls_to(&e, JOB_POOL_INFO_CONSTRUCTOR),
            vec![vec![frame + 0x30]]
        );
        assert_eq!(
            calls_to(&e, JOB_QUEUE_INFO_CONSTRUCTOR),
            vec![vec![frame + 0x14]]
        );
    }

    #[test]
    fn acquire_runs_init_only_for_a_new_owner() {
        let mut e = allocator_fixture();
        e.register(GET_CURRENT_THREAD_ID, |_, _| Ret {
            eax: 0x44,
            ..Ret::default()
        });
        e.register(JOB_POOL_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x18, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_THREAD_POOL_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(JOB_QUEUE_INFO_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x8000_0000);
            Ret::default()
        });
        e.register(JOB_QUEUE_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(JOB_QUEUE_INSTALL, |_, _| Ret::default());
        let this = e.new_object::<BhkMultithreadedSystem>();
        e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 0x44);
        e.call(0x00cbf6b0, &args![this, 0u32]);
        assert!(calls_to(&e, JOB_POOL_INFO_CONSTRUCTOR).is_empty());

        e.set(this, BhkMultithreadedSystem::uiMTResourceOwner, 0x45);
        e.register(JOB_QUEUE_CLEANUP, |_, _| Ret::default());
        e.register(JOB_THREAD_POOL_CLEANUP, |_, _| Ret::default());
        e.call(0x00cbf6b0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, JOB_POOL_INFO_CONSTRUCTOR).len(), 1);
        assert_eq!(e.get(this, BhkMultithreadedSystem::uiMTResourceOwner), 0x44);
    }
}
