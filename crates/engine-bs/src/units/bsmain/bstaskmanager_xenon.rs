//! `BSMain/bstaskmanager_xenon.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `BSTaskThread` vtable (RTTI `.?AVBSTaskThread@@`).
const TASK_THREAD_VTABLE: u32 = 0x010c_1844;
/// Import slot of `CreateSemaphoreA` (stdcall(attributes, initial, maximum,
/// name)).
const CREATE_SEMAPHORE: u32 = 0x00fd_f0b4;
/// Import slot of `CreateThread` (stdcall(attributes, stack size, start,
/// parameter, flags, thread id out)).
const CREATE_THREAD: u32 = 0x00fd_f124;
/// `CREATE_SUSPENDED`, the flags the thread is created with.
const CREATE_SUSPENDED: u32 = 4;
/// The thread's entry point (`00c42da0`, `BSMain/run_00c42960`).
const THREAD_ENTRY: u32 = 0x00c4_2da0;
/// `SetThreadName` (engine map, `NiSystem/nithread_xenon.cpp`; cdecl(thread
/// id, name)).
const SET_THREAD_NAME: u32 = 0x00aa_2740;
/// `BSTaskThread`'s destructor body (`00c42f00`, `BSMain/run_00c42f00`).
const TASK_THREAD_DESTRUCTOR: u32 = 0x00c4_2f00;
/// The `MemoryManager` singleton and its free (thiscall(block)).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

layout! {
    /// `BSTaskThread` (Xbox PDB), 0x30 bytes; the PC constructor writes the
    /// same offsets.
    pub struct BSTaskThread: 0x30 {
        /// `hThreadHandle` (Xbox PDB).
        0x04 hThreadHandle: u32,
        /// `iThreadID` (Xbox PDB).
        0x08 iThreadID: u32,
        /// `Semaphore.iCount` (Xbox PDB, `BSSemaphore`).
        0x0C Semaphore_iCount: u32,
        /// `Semaphore.hSemaphore`.
        0x10 Semaphore_hSemaphore: u32,
        /// `Semaphore.iMaxCount`.
        0x14 Semaphore_iMaxCount: u32,
        /// `PauseSemaphore.iCount` (Xbox PDB).
        0x18 PauseSemaphore_iCount: u32,
        /// `PauseSemaphore.hSemaphore`.
        0x1C PauseSemaphore_hSemaphore: u32,
        /// `PauseSemaphore.iMaxCount`.
        0x20 PauseSemaphore_iMaxCount: u32,
        /// `HavokThreadMemory.pHavokThreadRouter` (Xbox PDB).
        0x24 HavokThreadMemory_pHavokThreadRouter: u32,
        /// `HavokThreadMemory.pHavokThreadStackBuffer`.
        0x28 HavokThreadMemory_pHavokThreadStackBuffer: u32,
        /// `HavokThreadMemory.iThreadID`.
        0x2C HavokThreadMemory_iThreadID: u32,
    }
}

// Translated from 00c42dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42dd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `BSTaskThread::BSTaskThread` (Xbox PDB): stores the vtable, makes the
/// work semaphore (count 0, maximum `0x7FFFFFFF`) and the pause semaphore
/// (count 1, maximum 1) with `CreateSemaphoreA`, creates the thread
/// suspended at `00c42da0` with `this` as its parameter (its id into
/// `iThreadID`), names it when `name` is not null (`SetThreadName`), clears
/// the Havok thread memory's router and stack buffer and copies the thread
/// id into it. The first stack word is not read. Returns `this`.
pub fn bs_task_thread_constructor(
    e: &mut Engine,
    this: Ptr<BSTaskThread>,
    _unused_1: u32,
    name: u32,
) -> Ptr<BSTaskThread> {
    e.mem.set_u32(this.addr(), TASK_THREAD_VTABLE);
    e.set(this, BSTaskThread::Semaphore_iCount, 0);
    e.set(this, BSTaskThread::Semaphore_iMaxCount, 0x7fff_ffff);
    let maximum = e.get(this, BSTaskThread::Semaphore_iMaxCount);
    let initial = e.get(this, BSTaskThread::Semaphore_iCount);
    let semaphore = e
        .call(CREATE_SEMAPHORE, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.set(this, BSTaskThread::Semaphore_hSemaphore, semaphore);
    e.set(this, BSTaskThread::PauseSemaphore_iCount, 1);
    e.set(this, BSTaskThread::PauseSemaphore_iMaxCount, 1);
    let maximum = e.get(this, BSTaskThread::PauseSemaphore_iMaxCount);
    let initial = e.get(this, BSTaskThread::PauseSemaphore_iCount);
    let semaphore = e
        .call(CREATE_SEMAPHORE, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.set(this, BSTaskThread::PauseSemaphore_hSemaphore, semaphore);
    let thread = e
        .call(
            CREATE_THREAD,
            &args![
                0u32,
                0u32,
                THREAD_ENTRY,
                this,
                CREATE_SUSPENDED,
                this.addr() + 8
            ],
        )
        .u32();
    e.set(this, BSTaskThread::hThreadHandle, thread);
    if name != 0 {
        let id = e.get(this, BSTaskThread::iThreadID);
        e.call(SET_THREAD_NAME, &args![id, name]);
    }
    e.set(this, BSTaskThread::HavokThreadMemory_pHavokThreadRouter, 0);
    e.set(
        this,
        BSTaskThread::HavokThreadMemory_pHavokThreadStackBuffer,
        0,
    );
    let id = e.get(this, BSTaskThread::iThreadID);
    e.set(this, BSTaskThread::HavokThreadMemory_iThreadID, id);
    this
}

// Translated from 00c3f130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskThread::scalar deleting destructor` (Xbox PDB): the destructor
/// body (`00c42f00`), then frees `this` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_task_thread_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSTaskThread>,
    flags: u32,
) -> Ptr<BSTaskThread> {
    e.call(TASK_THREAD_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_2dd0,
            bs_task_thread_constructor(Ptr<BSTaskThread>, u32, u32) -> Ptr<BSTaskThread>
        ),
        entry!(
            0x00c3_f130,
            bs_task_thread_scalar_deleting_destructor(Ptr<BSTaskThread>, u32) -> Ptr<BSTaskThread>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register_double(CREATE_SEMAPHORE, {
            let mut next = 0x100u32;
            move |_, _| {
                next += 1;
                next.into_ret()
            }
        });
        e.register(CREATE_THREAD, |e, a| {
            e.mem.set_u32(a[5], 0x77);
            0x200u32.into_ret()
        });
        e.register(SET_THREAD_NAME, |_, _| Ret::default());
        e.register(TASK_THREAD_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_makes_semaphores_and_a_suspended_thread() {
        let mut e = engine();
        let this: Ptr<BSTaskThread> = e.new_object();
        e.set(this, BSTaskThread::HavokThreadMemory_pHavokThreadRouter, 5);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c4_2dd0, &args![this, 0xdeadu32, 0x4000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), TASK_THREAD_VTABLE);
        assert_eq!(
            calls(&log, CREATE_SEMAPHORE),
            vec![vec![0, 0, 0x7fff_ffff, 0], vec![0, 1, 1, 0]]
        );
        assert_eq!(e.get(this, BSTaskThread::Semaphore_hSemaphore), 0x101);
        assert_eq!(e.get(this, BSTaskThread::PauseSemaphore_hSemaphore), 0x102);
        assert_eq!(
            calls(&log, CREATE_THREAD),
            vec![vec![0, 0, THREAD_ENTRY, this.addr(), 4, this.addr() + 8]]
        );
        assert_eq!(e.get(this, BSTaskThread::hThreadHandle), 0x200);
        assert_eq!(calls(&log, SET_THREAD_NAME), vec![vec![0x77, 0x4000]]);
        assert_eq!(
            e.get(this, BSTaskThread::HavokThreadMemory_pHavokThreadRouter),
            0
        );
        assert_eq!(e.get(this, BSTaskThread::HavokThreadMemory_iThreadID), 0x77);
    }

    #[test]
    fn constructor_without_a_name_does_not_name_the_thread() {
        let mut e = engine();
        let this: Ptr<BSTaskThread> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c4_2dd0, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SET_THREAD_NAME).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let back = e.call(0x00c3_f130, &args![0x5000u32, 2u32]).u32();
        e.call(0x00c3_f130, &args![0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x5000);
        assert_eq!(calls(&log, TASK_THREAD_DESTRUCTOR).len(), 2);
        assert_eq!(calls(&log, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0x5000]]);
    }
}
