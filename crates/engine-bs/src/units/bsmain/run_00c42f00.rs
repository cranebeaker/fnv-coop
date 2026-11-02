//! `BSMain/run_00c42f00` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two `BSTaskThread` methods without Xbox PDB names (the class and its
//! layout are in `bstaskmanager_xenon.rs`; the fields are read here at
//! their offsets).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `BSTaskThread` vtable (RTTI `.?AVBSTaskThread@@`).
const TASK_THREAD_VTABLE: u32 = 0x010c_1844;
/// Import slot of `SuspendThread` (stdcall(handle)).
const SUSPEND_THREAD: u32 = 0x00fd_f15c;
/// Import slot of `ResumeThread` (stdcall(handle)).
const RESUME_THREAD: u32 = 0x00fd_f158;
/// Import slot of `CloseHandle` (stdcall(handle)).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;

/// `BSTaskThread::hThreadHandle` (Xbox PDB) +0x04.
const THREAD_HANDLE: u32 = 0x04;
/// `BSTaskThread::Semaphore.hSemaphore` (Xbox PDB) +0x10.
const SEMAPHORE_HANDLE: u32 = 0x10;
/// `BSTaskThread::PauseSemaphore.hSemaphore` (Xbox PDB) +0x1C.
const PAUSE_SEMAPHORE_HANDLE: u32 = 0x1c;

// Translated from 00c42f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskThread`'s destructor body: stores the vtable, suspends the
/// thread and closes its handle, then the pause semaphore's and the work
/// semaphore's handles.
pub fn fn_00c42f00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TASK_THREAD_VTABLE);
    let thread = e.mem.u32(this.addr() + THREAD_HANDLE);
    e.call(SUSPEND_THREAD, &args![thread]);
    let thread = e.mem.u32(this.addr() + THREAD_HANDLE);
    e.call(CLOSE_HANDLE, &args![thread]);
    let pause = e.mem.u32(this.addr() + PAUSE_SEMAPHORE_HANDLE);
    e.call(CLOSE_HANDLE, &args![pause]);
    let work = e.mem.u32(this.addr() + SEMAPHORE_HANDLE);
    e.call(CLOSE_HANDLE, &args![work]);
}

// Translated from 00c42f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes the task thread (`ResumeThread` on `hThreadHandle`).
pub fn fn_00c42f50(e: &mut Engine, this: Ptr) {
    let thread = e.mem.u32(this.addr() + THREAD_HANDLE);
    e.call(RESUME_THREAD, &args![thread]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_2f00, fn_00c42f00(Ptr)),
        entry!(0x00c4_2f50, fn_00c42f50(Ptr)),
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

    fn engine_with_thread() -> (Engine, u32) {
        let mut e = Engine::new();
        for addr in [SUSPEND_THREAD, RESUME_THREAD, CLOSE_HANDLE] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + THREAD_HANDLE, 0x11);
        e.mem.set_u32(this + SEMAPHORE_HANDLE, 0x22);
        e.mem.set_u32(this + PAUSE_SEMAPHORE_HANDLE, 0x33);
        (e, this)
    }

    #[test]
    fn destructor_suspends_and_closes_the_handles() {
        let (mut e, this) = engine_with_thread();
        e.call_log = Some(vec![]);
        e.call(0x00c4_2f00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TASK_THREAD_VTABLE);
        assert_eq!(calls(&log, SUSPEND_THREAD), vec![vec![0x11]]);
        assert_eq!(
            calls(&log, CLOSE_HANDLE),
            vec![vec![0x11], vec![0x33], vec![0x22]]
        );
    }

    #[test]
    fn resume_resumes_the_thread() {
        let (mut e, this) = engine_with_thread();
        e.call_log = Some(vec![]);
        e.call(0x00c4_2f50, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESUME_THREAD), vec![vec![0x11]]);
    }
}
