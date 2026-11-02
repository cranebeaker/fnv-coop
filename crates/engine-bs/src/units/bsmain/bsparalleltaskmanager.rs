//! `BSMain/bsparalleltaskmanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`this` of `MemoryManager::Allocate`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (Xbox PDB): `thiscall(size)`.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSLinearTaskThread`'s constructor (`008c76a0`, `thiscall`, no stack words).
const LINEAR_TASK_THREAD_CONSTRUCT: u32 = 0x008c_76a0;
/// `sprintf_s(buffer, size, format, ...)` (`cdecl`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `BSThread::Initialize` (Xbox PDB): `thiscall(stack size, name, suspended)`.
const THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `BSThread::SetName` (Xbox PDB): `thiscall(name)`.
const THREAD_SET_NAME: u32 = 0x00aa_6540;
/// `BSThread::SetThreadProcessor` (Xbox PDB): `thiscall(processor)`.
const THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
/// `BSThread::SetThreadPriority` (Xbox PDB): `thiscall(priority)`.
const THREAD_SET_PRIORITY: u32 = 0x00aa_6560;
/// The vtable stored in each thread (`010c1af0`; RTTI name
/// `.?AVBSParallelTaskThread@@`, read through its complete object locator).
const PARALLEL_TASK_THREAD_VTABLE: u32 = 0x010c_1af0;
/// `"Parallel Task Thread %i"`.
const THREAD_NAME_FORMAT: u32 = 0x010c_1ad4;
/// A `.data` table of words, one per parallel task thread: the number in
/// the thread's name and the processor it is given (index 0 holds 5 in the
/// exe's initial data).
const THREAD_PROCESSORS: u32 = 0x011a_f714;
/// The number of threads: the loop bound the code compares against
/// (`CMP [index],1; JNC`).
const THREAD_COUNT: u32 = 1;
/// Size of a `BSParallelTaskThread` on the PC (`0x5C`; the Xbox PDB's 0x6C
/// less the 0x10 the PC's `BSThread` is smaller).
const PARALLEL_TASK_THREAD_SIZE: u32 = 0x5c;
/// `BSParallelTaskThread::pManager` (Xbox PDB `+0x5C`), at `+0x4C` on the PC.
const THREAD_MANAGER: u32 = 0x4c;
/// The name buffer on the stack (`0x104` bytes).
const NAME_SIZE: u32 = 0x104;
/// The thread's stack size passed to `BSThread::Initialize`.
const THREAD_STACK_SIZE: u32 = 0x8000;
/// The priority passed to `BSThread::SetThreadPriority`.
const THREAD_PRIORITY: u32 = 0xf;

layout! {
    /// `BSParallelTaskManager` (Xbox PDB), 0x10 bytes; the PC constructor
    /// clears the same four fields at the same offsets.
    pub struct BSParallelTaskManager: 0x10 {
        /// `pThreads` (Xbox PDB): `BSParallelTaskThread**`.
        0x00 pThreads: u32,
        /// `pTaskFunc` (Xbox PDB).
        0x04 pTaskFunc: u32,
        /// `pData` (Xbox PDB).
        0x08 pData: u32,
        /// `bRunningTask` (Xbox PDB).
        0x0C bRunningTask: bool,
    }
}

// Translated from 00c44980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c44980 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `BSParallelTaskManager::BSParallelTaskManager` (Xbox PDB): clears the four
/// fields, allocates the thread pointer array (4 bytes) and, for each of the
/// one thread, allocates a `BSParallelTaskThread` (0x5C bytes; when the
/// allocation succeeds: the `BSLinearTaskThread` constructor `008c76a0`, its
/// vtable and `pManager = this`; null otherwise), stores it, formats its name
/// `"Parallel Task Thread %i"` with the thread's word of the table at
/// `011af714` into a 0x104-byte stack buffer, and starts it:
/// `BSThread::Initialize(0x8000, name, 0)`, `SetName(name)`,
/// `SetThreadProcessor(table word)`, `SetThreadPriority(0xF)`, each on the
/// pointer read back from the array. Returns `this`.
///
/// The code also tests the constant 1 (`MOV ECX,1; TEST ECX,ECX; JZ`) before
/// allocating the array; it is never zero, so that jump to the end is never
/// taken.
pub fn bs_parallel_task_manager_constructor(
    e: &mut Engine,
    this: Ptr<BSParallelTaskManager>,
) -> Ptr<BSParallelTaskManager> {
    e.set(this, BSParallelTaskManager::pThreads, 0);
    e.set(this, BSParallelTaskManager::pTaskFunc, 0);
    e.set(this, BSParallelTaskManager::pData, 0);
    e.set(this, BSParallelTaskManager::bRunningTask, false);
    let threads = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    e.set(this, BSParallelTaskManager::pThreads, threads);
    let mut index = 0u32;
    while index < THREAD_COUNT {
        let block = e
            .call(
                MEMORY_ALLOCATE,
                &args![MEMORY_MANAGER, PARALLEL_TASK_THREAD_SIZE],
            )
            .u32();
        let thread = if block == 0 {
            0
        } else {
            e.call(LINEAR_TASK_THREAD_CONSTRUCT, &args![block]);
            e.mem.set_u32(block, PARALLEL_TASK_THREAD_VTABLE);
            e.mem.set_u32(block + THREAD_MANAGER, this.addr());
            block
        };
        let threads = e.get(this, BSParallelTaskManager::pThreads);
        e.mem
            .set_u32(threads.wrapping_add(index.wrapping_mul(4)), thread);
        let table_word = THREAD_PROCESSORS.wrapping_add(index.wrapping_mul(4));
        e.with_stack(NAME_SIZE, |e, name| {
            let number = e.mem.u32(table_word);
            e.call(
                SPRINTF_S,
                &args![name, NAME_SIZE, THREAD_NAME_FORMAT, number],
            );
            let thread = thread_at(e, this, index);
            e.call(
                THREAD_INITIALIZE,
                &args![thread, THREAD_STACK_SIZE, name, 0u32],
            );
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_NAME, &args![thread, name]);
            let processor = e.mem.u32(table_word);
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_PROCESSOR, &args![thread, processor]);
            let thread = thread_at(e, this, index);
            e.call(THREAD_SET_PRIORITY, &args![thread, THREAD_PRIORITY]);
        });
        index += 1;
    }
    this
}

/// `this->pThreads[index]`, read again from memory as the code does before
/// each call.
fn thread_at(e: &mut Engine, this: Ptr<BSParallelTaskManager>, index: u32) -> u32 {
    let threads = e.get(this, BSParallelTaskManager::pThreads);
    e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)))
}

/// `MemoryManager::Deallocate` (Xbox PDB): `thiscall(block)`.
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `DoThreadSetup` (engine map, cdecl(Havok thread memory)).
const DO_THREAD_SETUP: u32 = 0x00c8_0130;
/// `DoThreadShutdown` (engine map, cdecl(Havok thread memory)).
const DO_THREAD_SHUTDOWN: u32 = 0x00c8_0240;
/// `BSLinearTaskThread`'s destructor body (`008c77c0`, thiscall).
const LINEAR_TASK_THREAD_DESTRUCTOR: u32 = 0x008c_77c0;
/// The thread call the manager's destructor makes first (`008c74b0`,
/// thiscall, `ailineartaskthreadmanager.cpp`).
const LINEAR_TASK_THREAD_CALL_8C74B0: u32 = 0x008c_74b0;
/// Import slot of `ReleaseSemaphore` (stdcall(handle, count, previous)).
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// Import slot of `InterlockedIncrement` (stdcall(address)).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// Import slot of `InterlockedDecrement` (stdcall(address)).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Import slot of `WaitForSingleObject` (stdcall(handle, timeout)).
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// The task function `00c44c90` hands to `00c44c40`.
const SPLIT_TASK: u32 = 0x00c4_4d90;

/// `BSParallelTaskThread` fields on the PC (the Xbox PDB's offsets less
/// 0x10): `BSThread::m_ThreadID` (Xbox `+0x34`).
const THREAD_ID: u32 = 0x24;
/// `BSLinearTaskThread::StartProcessing` (`BSSemaphore`, Xbox `+0x40`):
/// its `iCount`, then `hSemaphore` at +4.
const START_PROCESSING: u32 = 0x30;
/// `BSLinearTaskThread::FinishedProcessing` (Xbox `+0x4C`).
const FINISHED_PROCESSING: u32 = 0x3c;
/// `BSParallelTaskThread::HavokThreadMemory` (Xbox `+0x60`): router, stack
/// buffer, thread id.
const HAVOK_THREAD_MEMORY: u32 = 0x50;

// Translated from 00c448f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskThread::OnStartup` (Xbox PDB): clears the Havok thread
/// memory's router and stack buffer, copies the thread id into it and runs
/// `DoThreadSetup` on it.
pub fn bs_parallel_task_thread_on_startup(e: &mut Engine, this: Ptr) {
    let memory = this.addr().wrapping_add(HAVOK_THREAD_MEMORY);
    e.mem.set_u32(memory, 0);
    e.mem.set_u32(memory.wrapping_add(4), 0);
    let id = e.mem.u32(this.addr().wrapping_add(THREAD_ID));
    e.mem.set_u32(memory.wrapping_add(8), id);
    e.call(DO_THREAD_SETUP, &args![memory]);
}

// Translated from 00c44940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskThread::OnShutdown` (Xbox PDB): `DoThreadShutdown` on the
/// Havok thread memory.
pub fn bs_parallel_task_thread_on_shutdown(e: &mut Engine, this: Ptr) {
    let memory = this.addr().wrapping_add(HAVOK_THREAD_MEMORY);
    e.call(DO_THREAD_SHUTDOWN, &args![memory]);
}

// Translated from 00c44960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskThread::RunLinearTasks` (Xbox PDB): runs the manager's
/// task (`BSParallelTaskManager::RunTask` on `pManager`).
pub fn bs_parallel_task_thread_run_linear_tasks(e: &mut Engine, this: Ptr) {
    let manager = e.mem.u32(this.addr().wrapping_add(THREAD_MANAGER));
    bs_parallel_task_manager_run_task(e, Ptr::new(manager));
}

// Translated from 00c44b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's destructor body: for the one thread, calls `008c74b0` on
/// it, waits for its `FinishedProcessing` semaphore (infinite timeout) and
/// decrements its count, deletes it (virtual `+0` with 1) when it is not
/// null and clears its slot; then frees the thread array.
pub fn fn_00c44b70(e: &mut Engine, this: Ptr<BSParallelTaskManager>) {
    let mut index = 0u32;
    while index < THREAD_COUNT {
        let thread = thread_at(e, this, index);
        e.call(LINEAR_TASK_THREAD_CALL_8C74B0, &args![thread]);
        let thread = thread_at(e, this, index);
        let semaphore = thread.wrapping_add(FINISHED_PROCESSING);
        let handle = e.mem.u32(semaphore.wrapping_add(4));
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, u32::MAX]);
        e.call(INTERLOCKED_DECREMENT, &args![semaphore]);
        let thread = thread_at(e, this, index);
        if thread != 0 {
            e.vcall(thread, 0, &args![1u32]);
        }
        let threads = e.get(this, BSParallelTaskManager::pThreads);
        e.mem
            .set_u32(threads.wrapping_add(index.wrapping_mul(4)), 0);
        index += 1;
    }
    let threads = e.get(this, BSParallelTaskManager::pThreads);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, threads]);
}

// Translated from 00c44c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `function(data)` on every thread and this one: sets `bRunningTask`,
/// stores the function and its data, starts the threads (`00c44cd0`), runs
/// the task here (`RunTask`), waits for the threads (`00c44d30`) and clears
/// `bRunningTask`.
pub fn fn_00c44c40(e: &mut Engine, this: Ptr<BSParallelTaskManager>, function: u32, data: u32) {
    e.set(this, BSParallelTaskManager::bRunningTask, true);
    e.set(this, BSParallelTaskManager::pTaskFunc, function);
    e.set(this, BSParallelTaskManager::pData, data);
    fn_00c44cd0(e, this);
    bs_parallel_task_manager_run_task(e, this);
    fn_00c44d30(e, this);
    e.set(this, BSParallelTaskManager::bRunningTask, false);
}

// Translated from 00c44c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the split task `00c44d90` on `work` across the threads
/// (`00c44c40(00c44d90, work)`).
pub fn fn_00c44c90(e: &mut Engine, this: Ptr<BSParallelTaskManager>, work: u32) {
    fn_00c44c40(e, this, SPLIT_TASK, work);
}

// Translated from 00c44cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskManager::RunTask` (Xbox PDB): `pTaskFunc(pData)` (cdecl).
pub fn bs_parallel_task_manager_run_task(e: &mut Engine, this: Ptr<BSParallelTaskManager>) {
    let data = e.get(this, BSParallelTaskManager::pData);
    let function = e.get(this, BSParallelTaskManager::pTaskFunc);
    e.call(function, &args![data]);
}

// Translated from 00c44cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the threads: for the one thread, releases its `StartProcessing`
/// semaphore once and increments its count.
pub fn fn_00c44cd0(e: &mut Engine, this: Ptr<BSParallelTaskManager>) {
    let mut index = 0u32;
    while index < THREAD_COUNT {
        let thread = thread_at(e, this, index);
        let semaphore = thread.wrapping_add(START_PROCESSING);
        let handle = e.mem.u32(semaphore.wrapping_add(4));
        e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
        e.call(INTERLOCKED_INCREMENT, &args![semaphore]);
        index += 1;
    }
}

// Translated from 00c44d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits for the threads: for the one thread, waits for its
/// `FinishedProcessing` semaphore (infinite timeout) and decrements its
/// count.
pub fn fn_00c44d30(e: &mut Engine, this: Ptr<BSParallelTaskManager>) {
    let mut index = 0u32;
    while index < THREAD_COUNT {
        let thread = thread_at(e, this, index);
        let semaphore = thread.wrapping_add(FINISHED_PROCESSING);
        let handle = e.mem.u32(semaphore.wrapping_add(4));
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, u32::MAX]);
        e.call(INTERLOCKED_DECREMENT, &args![semaphore]);
        index += 1;
    }
}

// Translated from 00c44d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The split task (cdecl(work)): `work` holds a function, its data, an item
/// count and a shared counter (+0, +4, +8, +0xC). Each runner takes the next
/// item with `InterlockedIncrement` on the counter and calls
/// `function(data, item)` (cdecl) for items below the count, until the
/// taken item reaches the count.
pub fn fn_00c44d90(e: &mut Engine, work: Ptr) {
    let work = work.addr();
    let mut item = 0u32;
    while item < e.mem.u32(work.wrapping_add(8)) {
        let taken = e
            .call(INTERLOCKED_INCREMENT, &args![work.wrapping_add(0xc)])
            .u32();
        item = taken.wrapping_sub(1);
        if item < e.mem.u32(work.wrapping_add(8)) {
            let data = e.mem.u32(work.wrapping_add(4));
            let function = e.mem.u32(work);
            e.call(function, &args![data, item]);
        }
    }
}

// Translated from 00c44df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskThread::scalar deleting destructor` (Xbox PDB): the
/// `BSLinearTaskThread` destructor body (`008c77c0`), then frees `this` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_parallel_task_thread_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(LINEAR_TASK_THREAD_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c44980,
            bs_parallel_task_manager_constructor(
                Ptr<BSParallelTaskManager>,
            ) -> Ptr<BSParallelTaskManager>
        ),
        entry!(0x00c4_48f0, bs_parallel_task_thread_on_startup(Ptr)),
        entry!(0x00c4_4940, bs_parallel_task_thread_on_shutdown(Ptr)),
        entry!(0x00c4_4960, bs_parallel_task_thread_run_linear_tasks(Ptr)),
        entry!(0x00c4_4b70, fn_00c44b70(Ptr<BSParallelTaskManager>)),
        entry!(
            0x00c4_4c40,
            fn_00c44c40(Ptr<BSParallelTaskManager>, u32, u32)
        ),
        entry!(0x00c4_4c90, fn_00c44c90(Ptr<BSParallelTaskManager>, u32)),
        entry!(
            0x00c4_4cb0,
            bs_parallel_task_manager_run_task(Ptr<BSParallelTaskManager>)
        ),
        entry!(0x00c4_4cd0, fn_00c44cd0(Ptr<BSParallelTaskManager>)),
        entry!(0x00c4_4d30, fn_00c44d30(Ptr<BSParallelTaskManager>)),
        entry!(0x00c4_4d90, fn_00c44d90(Ptr)),
        entry!(
            0x00c4_4df0,
            bs_parallel_task_thread_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    const DOUBLED: [u32; 6] = [
        LINEAR_TASK_THREAD_CONSTRUCT,
        SPRINTF_S,
        THREAD_INITIALIZE,
        THREAD_SET_NAME,
        THREAD_SET_PROCESSOR,
        THREAD_SET_PRIORITY,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in DOUBLED {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(0x011a_f000, 0x1000);
        e.mem.set_u32(THREAD_PROCESSORS, 5);
        e
    }

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_makes_and_starts_one_thread() {
        let mut e = engine();
        let manager: Ptr<BSParallelTaskManager> = e.new_object();
        e.set(manager, BSParallelTaskManager::pTaskFunc, 0x1234);
        e.set(manager, BSParallelTaskManager::pData, 0x5678);
        e.set(manager, BSParallelTaskManager::bRunningTask, true);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_4980, &args![manager]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, manager.addr());
        assert_eq!(e.get(manager, BSParallelTaskManager::pTaskFunc), 0);
        assert_eq!(e.get(manager, BSParallelTaskManager::pData), 0);
        assert!(!e.get(manager, BSParallelTaskManager::bRunningTask));
        assert_eq!(
            calls(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4], vec![MEMORY_MANAGER, 0x5c]]
        );
        let threads = e.get(manager, BSParallelTaskManager::pThreads);
        assert_ne!(threads, 0);
        let thread = e.mem.u32(threads);
        assert_ne!(thread, 0);
        assert_eq!(
            calls(&log, LINEAR_TASK_THREAD_CONSTRUCT),
            vec![vec![thread]]
        );
        assert_eq!(e.mem.u32(thread), PARALLEL_TASK_THREAD_VTABLE);
        assert_eq!(e.mem.u32(thread + 0x4c), manager.addr());
        let format = &calls(&log, SPRINTF_S)[0];
        let name = format[0];
        assert_eq!(&format[1..], &[0x104, THREAD_NAME_FORMAT, 5]);
        assert_eq!(
            calls(&log, THREAD_INITIALIZE),
            vec![vec![thread, 0x8000, name, 0]]
        );
        assert_eq!(calls(&log, THREAD_SET_NAME), vec![vec![thread, name]]);
        assert_eq!(calls(&log, THREAD_SET_PROCESSOR), vec![vec![thread, 5]]);
        assert_eq!(calls(&log, THREAD_SET_PRIORITY), vec![vec![thread, 0xf]]);
    }

    #[test]
    fn constructor_stores_null_when_the_thread_allocation_fails() {
        let mut e = engine();
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, 0xdead_beef);
        e.register_double(MEMORY_ALLOCATE, move |_, a| Ret {
            eax: if a[1] == 4 { array } else { 0 },
            ..Ret::default()
        });
        let manager: Ptr<BSParallelTaskManager> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c4_4980, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(manager, BSParallelTaskManager::pThreads), array);
        assert_eq!(e.mem.u32(array), 0);
        assert!(calls(&log, LINEAR_TASK_THREAD_CONSTRUCT).is_empty());
        // The calls still go to the (null) pointer read back from the array.
        assert_eq!(calls(&log, THREAD_INITIALIZE)[0][0], 0);
        assert_eq!(calls(&log, THREAD_SET_PRIORITY), vec![vec![0, 0xf]]);
    }

    /// A manager with one thread (0x5C bytes, its semaphore handles 0x11
    /// and 0x22) and the synchronisation imports doubled.
    fn manager_with_thread(e: &mut Engine) -> (Ptr<BSParallelTaskManager>, u32) {
        for addr in [
            RELEASE_SEMAPHORE,
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            WAIT_FOR_SINGLE_OBJECT,
            MEMORY_FREE,
            LINEAR_TASK_THREAD_CALL_8C74B0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let manager: Ptr<BSParallelTaskManager> = e.new_object();
        let threads = e.mem.alloc(4);
        let thread = e.mem.alloc(0x5c);
        e.mem.set_u32(thread + START_PROCESSING + 4, 0x11);
        e.mem.set_u32(thread + FINISHED_PROCESSING + 4, 0x22);
        e.mem.set_u32(thread + THREAD_MANAGER, manager.addr());
        e.mem.set_u32(threads, thread);
        e.set(manager, BSParallelTaskManager::pThreads, threads);
        (manager, thread)
    }

    #[test]
    fn on_startup_and_shutdown_set_up_the_havok_memory() {
        let mut e = engine();
        e.register(DO_THREAD_SETUP, |_, _| Ret::default());
        e.register(DO_THREAD_SHUTDOWN, |_, _| Ret::default());
        let thread = e.mem.alloc(0x5c);
        e.mem.set_u32(thread + THREAD_ID, 0x99);
        e.mem.set_u32(thread + 0x50, 7);
        e.mem.set_u32(thread + 0x54, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c4_48f0, &args![thread]);
        e.call(0x00c4_4940, &args![thread]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(thread + 0x50), 0);
        assert_eq!(e.mem.u32(thread + 0x54), 0);
        assert_eq!(e.mem.u32(thread + 0x58), 0x99);
        assert_eq!(calls(&log, DO_THREAD_SETUP), vec![vec![thread + 0x50]]);
        assert_eq!(calls(&log, DO_THREAD_SHUTDOWN), vec![vec![thread + 0x50]]);
    }

    #[test]
    fn run_linear_tasks_runs_the_managers_task() {
        let mut e = engine();
        let (manager, thread) = manager_with_thread(&mut e);
        e.register(0x0012_3450, |_, _| Ret::default());
        e.set(manager, BSParallelTaskManager::pTaskFunc, 0x0012_3450);
        e.set(manager, BSParallelTaskManager::pData, 0x777);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4960, &args![thread]);
        e.call(0x00c4_4cb0, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0012_3450), vec![vec![0x777], vec![0x777]]);
    }

    #[test]
    fn destructor_stops_deletes_and_frees() {
        let mut e = engine();
        let (manager, thread) = manager_with_thread(&mut e);
        e.register(0x0012_3460, |_, _| Ret::default());
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0012_3460);
        e.mem.set_u32(thread, vtable);
        let threads = e.get(manager, BSParallelTaskManager::pThreads);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4b70, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, LINEAR_TASK_THREAD_CALL_8C74B0),
            vec![vec![thread]]
        );
        assert_eq!(
            calls(&log, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x22, u32::MAX]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            vec![vec![thread + 0x3c]]
        );
        assert_eq!(calls(&log, 0x0012_3460), vec![vec![thread, 1]]);
        assert_eq!(e.mem.u32(threads), 0);
        assert_eq!(
            calls(&log, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, threads]]
        );
    }

    #[test]
    fn running_a_task_starts_runs_and_waits() {
        let mut e = engine();
        let (manager, thread) = manager_with_thread(&mut e);
        e.register(0x0012_3470, |e, a| {
            // The flag is set while the task runs.
            let manager = Ptr::<BSParallelTaskManager>::new(e.mem.u32(a[0]));
            assert!(e.get(manager, BSParallelTaskManager::bRunningTask));
            Ret::default()
        });
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, manager.addr());
        e.call_log = Some(vec![]);
        e.call(0x00c4_4c40, &args![manager, 0x0012_3470u32, data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), vec![vec![0x11, 1, 0]]);
        assert_eq!(
            calls(&log, INTERLOCKED_INCREMENT),
            vec![vec![thread + 0x30]]
        );
        assert_eq!(calls(&log, 0x0012_3470), vec![vec![data]]);
        assert_eq!(
            calls(&log, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x22, u32::MAX]]
        );
        assert!(!e.get(manager, BSParallelTaskManager::bRunningTask));
        assert_eq!(
            e.get(manager, BSParallelTaskManager::pTaskFunc),
            0x0012_3470
        );
        assert_eq!(e.get(manager, BSParallelTaskManager::pData), data);
    }

    #[test]
    fn split_run_hands_the_split_task_and_its_work() {
        let mut e = engine();
        let (manager, _) = manager_with_thread(&mut e);
        e.register(SPLIT_TASK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_4c90, &args![manager, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SPLIT_TASK), vec![vec![0x4444]]);
    }

    #[test]
    fn start_and_wait_touch_each_threads_semaphores() {
        let mut e = engine();
        let (manager, thread) = manager_with_thread(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4cd0, &args![manager]);
        e.call(0x00c4_4d30, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RELEASE_SEMAPHORE), vec![vec![0x11, 1, 0]]);
        assert_eq!(
            calls(&log, INTERLOCKED_INCREMENT),
            vec![vec![thread + 0x30]]
        );
        assert_eq!(
            calls(&log, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x22, u32::MAX]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            vec![vec![thread + 0x3c]]
        );
    }

    #[test]
    fn split_task_takes_items_until_the_count() {
        let mut e = engine();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(0x0012_3480, |_, _| Ret::default());
        let work = e.mem.alloc(16);
        e.mem.set_u32(work, 0x0012_3480);
        e.mem.set_u32(work + 4, 0xabc);
        e.mem.set_u32(work + 8, 3);
        // Another runner already took item 0.
        e.mem.set_u32(work + 12, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4d90, &args![work]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0012_3480),
            vec![vec![0xabc, 1], vec![0xabc, 2]]
        );
        // The last increment takes item 3, which ends the loop.
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT).len(), 3);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        e.register(LINEAR_TASK_THREAD_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_4df0, &args![0x5000u32, 0u32]).u32();
        e.call(0x00c4_4df0, &args![0x5000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x5000);
        assert_eq!(calls(&log, LINEAR_TASK_THREAD_DESTRUCTOR).len(), 2);
        assert_eq!(calls(&log, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0x5000]]);
    }
}
