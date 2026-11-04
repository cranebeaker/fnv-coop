//! `fallout/ai/ailineartaskthreadmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `AILinearTaskThreadManager` starts the AI linear task threads
//! (`AILinearTaskThread`, vtable `01085688`, derived from
//! `BSLinearTaskThread`, vtable `010856a0`; both RTTI names confirmed) and
//! the events the AI stages wait on. The PC layout differs from the Xbox
//! PDB's: two thread slots at +0x00, twelve events at +0x3C and eight at
//! +0x6C, as the code indexes them. `BSThread`'s members are 0x10 lower on
//! the PC (`StartProcessing` +0x30, `FinishedProcessing` +0x3C,
//! `bKillThread` +0x48).
//!
//! The semaphore wait and release are `004424e0` and `00442550` (Xbox PDB
//! names `WaitForSingleObject` and `ReleaseSemaphore`, `this` the
//! `BSSemaphore`). Windows functions are called through their import slots
//! (`CreateEventA` `00fdf0cc`, `CloseHandle` `00fdf07c`). Functions that
//! open a C++ exception frame (`FS:[0]`) carry a `Translation incomplete`
//! marker: the unwinding states are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots: `CreateEventA(security, manual reset, initial state,
/// name)` and `CloseHandle(handle)`.
const CREATE_EVENT: u32 = 0x00fd_f0cc;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `BSSemaphore` wait and release (`this` the semaphore).
const SEMAPHORE_WAIT: u32 = 0x0044_24e0;
const SEMAPHORE_RELEASE: u32 = 0x0044_2550;
/// `(setting)`: the address of a setting's value (+4; a zeroed scratch
/// word for a null setting).
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// The setting whose value above 2 selects two threads (name not
/// confirmed), and the two settings holding each thread's processor.
const THREAD_COUNT_SETTING: u32 = 0x011c_3ea4;
const FIRST_THREAD_PROCESSOR_SETTING: u32 = 0x011d_fb34;
const SECOND_THREAD_PROCESSOR_SETTING: u32 = 0x011d_fb18;
/// Thread task functions: one thread alone, or the first and second of two.
const SINGLE_THREAD_TASK: u32 = 0x008c_7bd0;
const FIRST_THREAD_TASK: u32 = 0x008c_7da0;
const SECOND_THREAD_TASK: u32 = 0x008c_7f50;
/// `"AI Linear Task Thread"`, `"AI Linear Task Thread 1"`,
/// `"AI Linear Task Thread 2"`, `"AI Linear Task Thread DEF %i"`.
const SINGLE_THREAD_NAME: u32 = 0x0108_561c;
const FIRST_THREAD_NAME: u32 = 0x0108_564c;
const SECOND_THREAD_NAME: u32 = 0x0108_5634;
const DEFAULT_NAME_FORMAT: u32 = 0x0108_5664;
/// The threads' stack size argument.
const THREAD_STACK_SIZE: u32 = 0x10000;
/// `(thread)`: the word at +0x24 of a `BSThread` (`m_ThreadID` by the
/// layout shift; map name `D3DTexture_LockRect` is folded code), stored at
/// `011dfa10`.
const THREAD_ID: u32 = 0x0059_bb30;
const FIRST_THREAD_ID: u32 = 0x011d_fa10;
/// `operator new` (cdecl `(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BSThread::Initialize` (Xbox PDB) `(this, stack size, name, 0)`.
const THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `BSThread::SetThreadProcessor` (Xbox PDB) `(this, processor)`.
const THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
/// `BSThread::SetThreadPriority` (Xbox PDB) `(this, priority)`.
const THREAD_SET_PRIORITY: u32 = 0x00aa_6560;
/// `BSThread::SetName` (Xbox PDB) `(this, name)`.
const THREAD_SET_NAME: u32 = 0x00aa_6540;
/// cdecl `(buffer, size, format, ...)`: formatted print into a buffer.
const PRINT_TO_BUFFER: u32 = 0x0040_6d00;
/// `BSThread::BSThread` (Xbox PDB) `(this)`.
const THREAD_CONSTRUCT: u32 = 0x00aa_6360;
/// `(this)`: the `BSSemaphore` constructor (map: `loadingmenu.cpp`).
const SEMAPHORE_CONSTRUCT: u32 = 0x0078_d3b0;
/// `hkMemoryRouter::hkMemoryRouter` (Xbox PDB) `(this)`.
const MEMORY_ROUTER_CONSTRUCT: u32 = 0x00c8_5770;
/// Vtables (RTTI names confirmed).
const AI_LINEAR_TASK_THREAD_VTABLE: u32 = 0x0108_5688;
const BS_LINEAR_TASK_THREAD_VTABLE: u32 = 0x0108_56a0;

/// Thread slots, and the two event arrays.
const THREAD_SLOTS: u32 = 2;
const STAGE_EVENTS: u32 = 12;
const OTHER_EVENTS: u32 = 8;

layout! {
    /// `AILinearTaskThreadManager` (Xbox PDB name), PC layout as the code
    /// indexes it.
    pub struct AILinearTaskThreadManager: 0x8C {
        /// `AITaskThreadArray` (Xbox PDB): the first `AILinearTaskThread*`
        /// (two on the PC, at +0x00 and +0x04).
        0x00 AITaskThreadArray: Ptr,
        /// Twelve event handles from +0x3C (`CreateEventA`).
        0x3C StageEvents: u32,
        /// Eight event handles from +0x6C.
        0x6C OtherEvents: u32,
    }

    /// `BSLinearTaskThread` (Xbox PDB), PC offsets (0x10 below the PDB's).
    pub struct BSLinearTaskThread: 0x4C {
        /// Virtual function table pointer.
        0x00 vfptr: u32,
        /// `StartProcessing` (Xbox PDB): a `BSSemaphore`, 0xC bytes.
        0x30 StartProcessing: u32,
        /// `FinishedProcessing` (Xbox PDB): a `BSSemaphore`.
        0x3C FinishedProcessing: u32,
        /// `bKillThread` (Xbox PDB).
        0x48 bKillThread: bool,
    }

    /// `AILinearTaskThread` (Xbox PDB), 0x78 bytes on the PC (the
    /// allocation `AddThread` makes).
    pub struct AILinearTaskThread: 0x78 {
        /// Virtual function table pointer.
        0x00 vfptr: u32,
        /// `pfnTaskFunc` (Xbox PDB).
        0x4C pfnTaskFunc: u32,
        /// `ThreadMemory` (Xbox PDB): an `hkMemoryRouter`.
        0x50 ThreadMemory: u32,
    }
}

fn thread_slot(this: Ptr<AILinearTaskThreadManager>, index: u32) -> u32 {
    this.addr().wrapping_add(index.wrapping_mul(4))
}

// Translated from 008c7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AILinearTaskThreadManager::CreateThreads` (Xbox PDB): creates the twelve
/// and the eight events (manual reset, not signalled, unnamed), then one
/// thread, or two when the setting at `011c3ea4` is above 2, each on the
/// processor of its setting, and keeps the first thread's id (`0059bb30`)
/// at `011dfa10`.
pub fn ai_linear_task_thread_manager_create_threads(
    e: &mut Engine,
    this: Ptr<AILinearTaskThreadManager>,
) {
    for i in 0..STAGE_EVENTS {
        let event = e.call(CREATE_EVENT, &args![0u32, 1u32, 0u32, 0u32]).u32();
        e.mem.set_u32(this.addr() + 0x3c + 4 * i, event);
    }
    for i in 0..OTHER_EVENTS {
        let event = e.call(CREATE_EVENT, &args![0u32, 1u32, 0u32, 0u32]).u32();
        e.mem.set_u32(this.addr() + 0x6c + 4 * i, event);
    }
    let count = e.call(SETTING_VALUE, &args![THREAD_COUNT_SETTING]).u32();
    if e.mem.i32(count) > 2 {
        let processor = e
            .call(SETTING_VALUE, &args![FIRST_THREAD_PROCESSOR_SETTING])
            .u32();
        let processor = e.mem.u32(processor);
        ai_linear_task_thread_manager_add_thread(
            e,
            this,
            0,
            processor,
            FIRST_THREAD_TASK,
            Ptr::new(FIRST_THREAD_NAME),
            THREAD_STACK_SIZE,
        );
        let processor = e
            .call(SETTING_VALUE, &args![SECOND_THREAD_PROCESSOR_SETTING])
            .u32();
        let processor = e.mem.u32(processor);
        ai_linear_task_thread_manager_add_thread(
            e,
            this,
            1,
            processor,
            SECOND_THREAD_TASK,
            Ptr::new(SECOND_THREAD_NAME),
            THREAD_STACK_SIZE,
        );
    } else {
        let processor = e
            .call(SETTING_VALUE, &args![FIRST_THREAD_PROCESSOR_SETTING])
            .u32();
        let processor = e.mem.u32(processor);
        ai_linear_task_thread_manager_add_thread(
            e,
            this,
            0,
            processor,
            SINGLE_THREAD_TASK,
            Ptr::new(SINGLE_THREAD_NAME),
            THREAD_STACK_SIZE,
        );
    }
    let first = e.mem.u32(this.addr());
    let id = e.call(THREAD_ID, &args![first]).u32();
    e.set_global(FIRST_THREAD_ID, id);
}

// Translated from 008c73a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops and deletes the threads (kill flag and start signal `008c74b0`,
/// wait for it to finish `008c7490`, deleting destructor slot 0 with 1),
/// then closes all twenty event handles.
pub fn fn_008c73a0(e: &mut Engine, this: Ptr<AILinearTaskThreadManager>) {
    for i in 0..THREAD_SLOTS {
        let slot = thread_slot(this, i);
        if e.mem.u32(slot) == 0 {
            continue;
        }
        let thread = Ptr::new(e.mem.u32(slot));
        fn_008c74b0(e, thread);
        let thread = Ptr::new(e.mem.u32(slot));
        fn_008c7490(e, thread);
        let thread = e.mem.u32(slot);
        if thread != 0 {
            e.vcall(thread, 0x00, &args![1u32]);
        }
        e.mem.set_u32(slot, 0);
    }
    for i in 0..STAGE_EVENTS {
        let handle = e.mem.u32(this.addr() + 0x3c + 4 * i);
        e.call(CLOSE_HANDLE, &args![handle]);
    }
    for i in 0..OTHER_EVENTS {
        let handle = e.mem.u32(this.addr() + 0x6c + 4 * i);
        e.call(CLOSE_HANDLE, &args![handle]);
    }
}

// Translated from 008c7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the thread's `FinishedProcessing` semaphore.
pub fn fn_008c7490(e: &mut Engine, this: Ptr<BSLinearTaskThread>) {
    e.call(SEMAPHORE_WAIT, &args![this.addr() + 0x3c]);
}

// Translated from 008c74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bKillThread` and releases `StartProcessing`, so the thread's loop
/// ends.
pub fn fn_008c74b0(e: &mut Engine, this: Ptr<BSLinearTaskThread>) {
    e.set(this, BSLinearTaskThread::bKillThread, true);
    e.call(SEMAPHORE_RELEASE, &args![this.addr() + 0x30]);
}

// Translated from 008c74d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c74d0 - C++ exception frame (FS:[0] unwinding states) and /GS cookie check (00ec408c) not translated
/// `AILinearTaskThreadManager::AddThread` (Xbox PDB): for a free slot
/// `index` (signed, below 2) makes an `AILinearTaskThread` running `task`,
/// initializes it with `stack_size` and `name`, puts it on `processor` at
/// priority 0, and without a name names it `"AI Linear Task Thread DEF
/// <index>"` (a 0x104-byte stack buffer). The thread is set up through the
/// allocation's result even when it is null, as the game does.
pub fn ai_linear_task_thread_manager_add_thread(
    e: &mut Engine,
    this: Ptr<AILinearTaskThreadManager>,
    index: i32,
    processor: u32,
    task: u32,
    name: Ptr,
    stack_size: u32,
) {
    // The code tests `index < 3`, then `index < 2` (signed).
    if index >= 2 {
        return;
    }
    let slot = thread_slot(this, index as u32);
    if e.mem.u32(slot) != 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x78u32]).u32();
    let thread = if block == 0 {
        Ptr::NULL
    } else {
        ai_linear_task_thread_construct(e, Ptr::new(block), task)
    };
    e.mem.set_u32(slot, thread.addr());
    let thread = e.mem.u32(slot);
    e.call(THREAD_INITIALIZE, &args![thread, stack_size, name, 0u32]);
    let thread = e.mem.u32(slot);
    e.call(THREAD_SET_PROCESSOR, &args![thread, processor]);
    let thread = e.mem.u32(slot);
    e.call(THREAD_SET_PRIORITY, &args![thread, 0u32]);
    if name.is_null() {
        e.with_stack(0x104, |e, buffer| {
            e.call(
                PRINT_TO_BUFFER,
                &args![buffer, 0x104u32, DEFAULT_NAME_FORMAT, index],
            );
            let thread = e.mem.u32(slot);
            e.call(THREAD_SET_NAME, &args![thread, buffer]);
        });
    }
}

// Translated from 008c7630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c7630 - C++ exception frame (FS:[0] unwinding states) not translated
/// `AILinearTaskThread::AILinearTaskThread` (Xbox PDB): the
/// `BSLinearTaskThread` constructor (`008c76a0`), its vtable, the thread's
/// `hkMemoryRouter`, and `pfnTaskFunc = task`. Returns `this`.
pub fn ai_linear_task_thread_construct(
    e: &mut Engine,
    this: Ptr<AILinearTaskThread>,
    task: u32,
) -> Ptr<AILinearTaskThread> {
    fn_008c76a0(e, this.cast());
    e.set(
        this,
        AILinearTaskThread::vfptr,
        AI_LINEAR_TASK_THREAD_VTABLE,
    );
    e.call(MEMORY_ROUTER_CONSTRUCT, &args![this.addr() + 0x50]);
    e.set(this, AILinearTaskThread::pfnTaskFunc, task);
    this
}

// Translated from 008c76a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c76a0 - C++ exception frame (FS:[0] unwinding states) not translated
/// The `BSLinearTaskThread` constructor: `BSThread::BSThread`, its vtable,
/// the two semaphores, `bKillThread` cleared. Returns `this`.
pub fn fn_008c76a0(e: &mut Engine, this: Ptr<BSLinearTaskThread>) -> Ptr<BSLinearTaskThread> {
    e.call(THREAD_CONSTRUCT, &args![this]);
    e.set(
        this,
        BSLinearTaskThread::vfptr,
        BS_LINEAR_TASK_THREAD_VTABLE,
    );
    e.call(SEMAPHORE_CONSTRUCT, &args![this.addr() + 0x30]);
    e.call(SEMAPHORE_CONSTRUCT, &args![this.addr() + 0x3c]);
    e.set(this, BSLinearTaskThread::bKillThread, false);
    this
}

// Translated from 008c7720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSLinearTaskThread::ThreadProc` (Xbox PDB): `OnStartup` (slot 8), then
/// until `bKillThread` is set (checked before and after each wait on
/// `StartProcessing`) runs `RunLinearTasks` (slot 0x10) and releases
/// `FinishedProcessing`; finally `OnShutdown` (slot 0xC) and one more
/// release. Returns 0.
pub fn bs_linear_task_thread_thread_proc(e: &mut Engine, this: Ptr<BSLinearTaskThread>) -> u32 {
    e.vcall(this.addr(), 0x08, &args![]);
    while !e.get(this, BSLinearTaskThread::bKillThread) {
        e.call(SEMAPHORE_WAIT, &args![this.addr() + 0x30]);
        if e.get(this, BSLinearTaskThread::bKillThread) {
            break;
        }
        e.vcall(this.addr(), 0x10, &args![]);
        e.call(SEMAPHORE_RELEASE, &args![this.addr() + 0x3c]);
    }
    e.vcall(this.addr(), 0x0c, &args![]);
    e.call(SEMAPHORE_RELEASE, &args![this.addr() + 0x3c]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c7290,
            ai_linear_task_thread_manager_create_threads(Ptr<AILinearTaskThreadManager>)
        ),
        entry!(0x008c73a0, fn_008c73a0(Ptr<AILinearTaskThreadManager>)),
        entry!(0x008c7490, fn_008c7490(Ptr<BSLinearTaskThread>)),
        entry!(0x008c74b0, fn_008c74b0(Ptr<BSLinearTaskThread>)),
        entry!(
            0x008c74d0,
            ai_linear_task_thread_manager_add_thread(
                Ptr<AILinearTaskThreadManager>,
                i32,
                u32,
                u32,
                Ptr,
                u32,
            )
        ),
        entry!(
            0x008c7630,
            ai_linear_task_thread_construct(
                Ptr<AILinearTaskThread>,
                u32,
            ) -> Ptr<AILinearTaskThread>
        ),
        entry!(
            0x008c76a0,
            fn_008c76a0(Ptr<BSLinearTaskThread>) -> Ptr<BSLinearTaskThread>
        ),
        entry!(
            0x008c7720,
            bs_linear_task_thread_thread_proc(Ptr<BSLinearTaskThread>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Doubles (answering 0) for every callee outside this file; the
    /// settings' value getter answers `setting + 4`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            CLOSE_HANDLE,
            SEMAPHORE_WAIT,
            SEMAPHORE_RELEASE,
            THREAD_ID,
            OPERATOR_NEW,
            THREAD_INITIALIZE,
            THREAD_SET_PROCESSOR,
            THREAD_SET_PRIORITY,
            THREAD_SET_NAME,
            PRINT_TO_BUFFER,
            THREAD_CONSTRUCT,
            SEMAPHORE_CONSTRUCT,
            MEMORY_ROUTER_CONSTRUCT,
        ] {
            stub(&mut e, addr, 0);
        }
        e.register_double(CREATE_EVENT, {
            let mut next = 0x100u32;
            move |_, _| {
                next += 1;
                ret(next)
            }
        });
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        for page in [0x011c_3000, 0x011d_f000] {
            e.map(page, 0x1000);
        }
        e
    }

    #[test]
    fn create_threads_makes_events_and_one_thread() {
        let mut e = engine();
        let this: Ptr<AILinearTaskThreadManager> = e.new_object();
        e.set_global(THREAD_COUNT_SETTING + 4, 2i32);
        e.set_global(FIRST_THREAD_PROCESSOR_SETTING + 4, 3u32);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        stub(&mut e, THREAD_ID, 0x7171);
        e.call_log = Some(vec![]);
        e.call(0x008c_7290, &args![this]);
        let log = e.call_log.take().unwrap();
        let events = calls(&log, CREATE_EVENT);
        assert_eq!(events.len(), 20);
        assert!(events.iter().all(|a| a == &vec![0, 1, 0, 0]));
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0x101);
        assert_eq!(e.mem.u32(this.addr() + 0x68), 0x10c);
        assert_eq!(e.mem.u32(this.addr() + 0x6c), 0x10d);
        assert_eq!(e.mem.u32(this.addr() + 0x88), 0x114);
        // One thread with the single-thread task and name.
        let thread = e.mem.u32(this.addr());
        assert_ne!(thread, 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(thread + 0x4c), SINGLE_THREAD_TASK);
        assert_eq!(
            calls(&log, THREAD_INITIALIZE),
            vec![vec![thread, THREAD_STACK_SIZE, SINGLE_THREAD_NAME, 0]]
        );
        assert_eq!(calls(&log, THREAD_SET_PROCESSOR), vec![vec![thread, 3]]);
        assert_eq!(calls(&log, THREAD_ID), vec![vec![thread]]);
        assert_eq!(e.global::<u32>(FIRST_THREAD_ID), 0x7171);
    }

    #[test]
    fn create_threads_makes_two_threads_above_two() {
        let mut e = engine();
        let this: Ptr<AILinearTaskThreadManager> = e.new_object();
        e.set_global(THREAD_COUNT_SETTING + 4, 3i32);
        e.set_global(FIRST_THREAD_PROCESSOR_SETTING + 4, 3u32);
        e.set_global(SECOND_THREAD_PROCESSOR_SETTING + 4, 5u32);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.call_log = Some(vec![]);
        e.call(0x008c_7290, &args![this]);
        let log = e.call_log.take().unwrap();
        let first = e.mem.u32(this.addr());
        let second = e.mem.u32(this.addr() + 4);
        assert_eq!(e.mem.u32(first + 0x4c), FIRST_THREAD_TASK);
        assert_eq!(e.mem.u32(second + 0x4c), SECOND_THREAD_TASK);
        assert_eq!(
            calls(&log, THREAD_SET_PROCESSOR),
            vec![vec![first, 3], vec![second, 5]]
        );
        let init = calls(&log, THREAD_INITIALIZE);
        assert_eq!(init[0][2], FIRST_THREAD_NAME);
        assert_eq!(init[1][2], SECOND_THREAD_NAME);
    }

    #[test]
    fn stop_threads_kills_waits_deletes_and_closes_events() {
        let mut e = engine();
        let this: Ptr<AILinearTaskThreadManager> = e.new_object();
        let vtable = 0x0200_1000;
        e.put_vtable(vtable, &[0x7000_0000]);
        stub(&mut e, 0x7000_0000, 0);
        let thread = e.mem.alloc(0x78);
        e.mem.set_u32(thread, vtable);
        e.mem.set_u32(this.addr() + 4, thread);
        for i in 0..20 {
            e.mem.set_u32(this.addr() + 0x3c + 4 * i, 0x200 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x008c_73a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(thread + 0x48), 1);
        assert_eq!(calls(&log, SEMAPHORE_RELEASE), vec![vec![thread + 0x30]]);
        assert_eq!(calls(&log, SEMAPHORE_WAIT), vec![vec![thread + 0x3c]]);
        assert_eq!(calls(&log, 0x7000_0000), vec![vec![thread, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        let closed: Vec<u32> = calls(&log, CLOSE_HANDLE).iter().map(|a| a[0]).collect();
        assert_eq!(closed, (0..20).map(|i| 0x200 + i).collect::<Vec<u32>>());
    }

    #[test]
    fn wait_finished_waits_on_the_finished_semaphore() {
        let mut e = engine();
        let thread: Ptr<BSLinearTaskThread> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x008c_7490, &args![thread]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SEMAPHORE_WAIT),
            vec![vec![thread.addr() + 0x3c]]
        );
    }

    #[test]
    fn kill_sets_the_flag_and_releases_the_start_semaphore() {
        let mut e = engine();
        let thread: Ptr<BSLinearTaskThread> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x008c_74b0, &args![thread]);
        let log = e.call_log.take().unwrap();
        assert!(e.get(thread, BSLinearTaskThread::bKillThread));
        assert_eq!(
            calls(&log, SEMAPHORE_RELEASE),
            vec![vec![thread.addr() + 0x30]]
        );
    }

    #[test]
    fn add_thread_names_unnamed_threads_and_skips_taken_slots() {
        let mut e = engine();
        let this: Ptr<AILinearTaskThreadManager> = e.new_object();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.call_log = Some(vec![]);
        e.call(
            0x008c_74d0,
            &args![this, 1i32, 4u32, 0x8888u32, 0u32, 0x2000u32],
        );
        let log = e.call_log.take().unwrap();
        let thread = e.mem.u32(this.addr() + 4);
        assert_ne!(thread, 0);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x78]]);
        assert_eq!(
            calls(&log, THREAD_INITIALIZE),
            vec![vec![thread, 0x2000, 0, 0]]
        );
        assert_eq!(calls(&log, THREAD_SET_PRIORITY), vec![vec![thread, 0]]);
        let print = calls(&log, PRINT_TO_BUFFER);
        assert_eq!(&print[0][1..], &[0x104, DEFAULT_NAME_FORMAT, 1]);
        assert_eq!(
            calls(&log, THREAD_SET_NAME),
            vec![vec![thread, print[0][0]]]
        );

        // A taken slot (1) or an index of 2 or more: nothing allocated.
        for index in [1i32, 2, 5] {
            e.call_log = Some(vec![]);
            e.call(
                0x008c_74d0,
                &args![this, index, 4u32, 0x8888u32, 0u32, 0x2000u32],
            );
            assert!(calls(&e.call_log.take().unwrap(), OPERATOR_NEW).is_empty());
        }

        // A failed allocation: the null thread is still set up.
        stub(&mut e, OPERATOR_NEW, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x008c_74d0,
            &args![this, 0i32, 4u32, 0x8888u32, 0x1234u32, 0x2000u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, THREAD_INITIALIZE),
            vec![vec![0, 0x2000, 0x1234, 0]]
        );
        assert!(calls(&log, THREAD_SET_NAME).is_empty());
    }

    #[test]
    fn ai_linear_task_thread_constructor_builds_base_and_task() {
        let mut e = engine();
        let thread: Ptr<AILinearTaskThread> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x008c_7630, &args![thread, 0x8888u32]).u32();
        assert_eq!(back, thread.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, THREAD_CONSTRUCT), vec![vec![thread.addr()]]);
        assert_eq!(
            calls(&log, MEMORY_ROUTER_CONSTRUCT),
            vec![vec![thread.addr() + 0x50]]
        );
        assert_eq!(
            e.get(thread, AILinearTaskThread::vfptr),
            AI_LINEAR_TASK_THREAD_VTABLE
        );
        assert_eq!(e.get(thread, AILinearTaskThread::pfnTaskFunc), 0x8888);
    }

    #[test]
    fn bs_linear_task_thread_constructor_sets_up_the_semaphores() {
        let mut e = engine();
        let thread: Ptr<BSLinearTaskThread> = e.new_object();
        e.set(thread, BSLinearTaskThread::bKillThread, true);
        e.call_log = Some(vec![]);
        let back = e.call(0x008c_76a0, &args![thread]).u32();
        assert_eq!(back, thread.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SEMAPHORE_CONSTRUCT),
            vec![vec![thread.addr() + 0x30], vec![thread.addr() + 0x3c]]
        );
        assert_eq!(
            e.get(thread, BSLinearTaskThread::vfptr),
            BS_LINEAR_TASK_THREAD_VTABLE
        );
        assert!(!e.get(thread, BSLinearTaskThread::bKillThread));
    }

    #[test]
    fn thread_proc_runs_tasks_until_killed() {
        let mut e = engine();
        let thread: Ptr<BSLinearTaskThread> = e.new_object();
        let vtable = 0x0200_2000;
        e.put_vtable(vtable, &[0, 0, 0x7100_0008, 0x7100_000c, 0x7100_0010]);
        e.mem.set_u32(thread.addr(), vtable);
        for slot in [0x7100_0008, 0x7100_000c] {
            stub(&mut e, slot, 0);
        }
        // Two task runs; the third wait sets the kill flag.
        e.register_double(SEMAPHORE_WAIT, {
            let mut waits = 0;
            move |e, a| {
                waits += 1;
                if waits == 3 {
                    e.mem.set_u8(a[0] - 0x30 + 0x48, 1);
                }
                Ret::default()
            }
        });
        stub(&mut e, 0x7100_0010, 0);
        e.call_log = Some(vec![]);
        let result = e.call(0x008c_7720, &args![thread]).u32();
        assert_eq!(result, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x7100_0008).len(), 1);
        assert_eq!(calls(&log, 0x7100_0010).len(), 2);
        assert_eq!(calls(&log, 0x7100_000c).len(), 1);
        assert_eq!(calls(&log, SEMAPHORE_WAIT).len(), 3);
        // One release per task run, one at shutdown.
        assert_eq!(
            calls(&log, SEMAPHORE_RELEASE),
            vec![vec![thread.addr() + 0x3c]; 3]
        );

        // Killed before starting: no wait at all.
        e.set(thread, BSLinearTaskThread::bKillThread, true);
        e.call_log = Some(vec![]);
        e.call(0x008c_7720, &args![thread]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SEMAPHORE_WAIT).is_empty());
        assert!(calls(&log, 0x7100_0010).is_empty());
    }
}
