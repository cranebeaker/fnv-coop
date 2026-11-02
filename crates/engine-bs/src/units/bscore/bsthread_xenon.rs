//! `BSCore/bsthread_xenon.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSThread` wraps a Win32 thread: a critical section guards the handle,
//! `Initialize` creates the thread suspended or running and resumes it, the
//! thread runs the object's `ThreadProc` (vtable slot 1) through the start
//! routine `00aa64d0`. Win32 functions are called through their import
//! slots.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSThread`'s vtable (`.?AVBSThread@@` through its RTTI locator).
pub(crate) const BS_THREAD_VTABLE: u32 = 0x010a_27d4;

/// Import slot of `InitializeCriticalSection`.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// Import slot of `DeleteCriticalSection`.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// Import slot of `EnterCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// Import slot of `LeaveCriticalSection`.
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Import slot of `CloseHandle`.
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// Import slot of `GetCurrentThread`.
const GET_CURRENT_THREAD: u32 = 0x00fd_f120;
/// Import slot of `GetCurrentThreadId`.
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// Import slot of `CreateThread`.
const CREATE_THREAD: u32 = 0x00fd_f124;
/// Import slot of `ResumeThread`.
const RESUME_THREAD: u32 = 0x00fd_f158;
/// Import slot of `SetThreadPriority` (Win32).
const WIN32_SET_THREAD_PRIORITY: u32 = 0x00fd_f11c;

/// The thread start routine `Initialize` passes to `CreateThread`.
const THREAD_START: u32 = 0x00aa_64d0;
/// `CREATE_SUSPENDED`, the creation flag `Initialize` passes when asked.
const CREATE_SUSPENDED: u32 = 4;

layout! {
    /// `BSThread` (Xbox PDB), 0x40 bytes on the Xbox. On the PC the critical
    /// section is a 0x18-byte `CRITICAL_SECTION`, so the fields after it
    /// sit 0x10 lower than the PDB says (offsets from the PC code).
    pub struct BSThread: 0x30 {
        /// The vtable (`~BSThread`, `ThreadProc`).
        0x00 vtable: u32,
        /// `CriticalSection` (Xbox PDB): a `CRITICAL_SECTION` (0x18 bytes on PC).
        0x04 CriticalSection: u32,
        /// `m_ThreadHandle` (Xbox PDB).
        0x1C m_ThreadHandle: u32,
        /// `m_ParentHandle` (Xbox PDB): `GetCurrentThread()` of the creator.
        0x20 m_ParentHandle: u32,
        /// `m_ThreadID` (Xbox PDB): written by `CreateThread`.
        0x24 m_ThreadID: u32,
        /// `m_ParentID` (Xbox PDB): `GetCurrentThreadId()` of the creator.
        0x28 m_ParentID: u32,
        /// `bThreadIsActive` (Xbox PDB).
        0x2C bThreadIsActive: bool,
    }
}

fn critical_section(this: Ptr<BSThread>) -> u32 {
    this.addr().wrapping_add(BSThread::CriticalSection.off)
}

// Translated from 00aa6360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSThread::BSThread` (Xbox PDB): sets the vtable, initializes the
/// critical section and clears the handles, ids and the active flag.
pub fn bs_thread_bs_thread(e: &mut Engine, this: Ptr<BSThread>) -> Ptr<BSThread> {
    e.set(this, BSThread::vtable, BS_THREAD_VTABLE);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![critical_section(this)]);
    e.set(this, BSThread::m_ThreadHandle, 0);
    e.set(this, BSThread::m_ParentHandle, 0);
    e.set(this, BSThread::m_ThreadID, 0);
    e.set(this, BSThread::m_ParentID, 0);
    e.set(this, BSThread::bThreadIsActive, false);
    this
}

// Translated from 00aa63c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa63c0 - the FS:[0] exception-unwinding frame is not translated
/// The `BSThread` destructor body (no name in the engine map; the vtable it
/// stores is `BSThread`'s): resets the vtable, closes the thread handle
/// (`00aa64f0`) and deletes the critical section.
pub fn fn_00aa63c0(e: &mut Engine, this: Ptr<BSThread>) {
    e.set(this, BSThread::vtable, BS_THREAD_VTABLE);
    fn_00aa64f0(e, this);
    e.call(DELETE_CRITICAL_SECTION, &args![critical_section(this)]);
}

// Translated from 00aa6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSThread::Initialize` (Xbox PDB): if no thread exists yet, records the
/// creator's thread handle and id, creates the thread (`stack_size`, start
/// routine `00aa64d0` with this object, suspended when `create_suspended`),
/// then under the critical section sets the priority to -1, marks the
/// thread active and resumes it. The second word is never read.
pub fn bs_thread_initialize(
    e: &mut Engine,
    this: Ptr<BSThread>,
    stack_size: u32,
    _unused_2: u32,
    create_suspended: bool,
) {
    if e.get(this, BSThread::m_ThreadHandle) != 0 {
        return;
    }
    let parent_handle = e.call(GET_CURRENT_THREAD, &[]).u32();
    e.set(this, BSThread::m_ParentHandle, parent_handle);
    let parent_id = e.call(GET_CURRENT_THREAD_ID, &[]).u32();
    e.set(this, BSThread::m_ParentID, parent_id);
    let flags = if create_suspended {
        CREATE_SUSPENDED
    } else {
        0
    };
    let thread_id_out = this.addr().wrapping_add(BSThread::m_ThreadID.off);
    let handle = e
        .call(
            CREATE_THREAD,
            &args![0u32, stack_size, THREAD_START, this, flags, thread_id_out],
        )
        .u32();
    e.set(this, BSThread::m_ThreadHandle, handle);
    e.call(ENTER_CRITICAL_SECTION, &args![critical_section(this)]);
    bs_thread_set_thread_priority(e, this, -1);
    e.set(this, BSThread::bThreadIsActive, true);
    let handle = e.get(this, BSThread::m_ThreadHandle);
    e.call(RESUME_THREAD, &args![handle]);
    e.call(LEAVE_CRITICAL_SECTION, &args![critical_section(this)]);
}

// Translated from 00aa64d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The thread start routine (`__stdcall`, one word): calls the thread
/// object's `ThreadProc` (vtable +0x04) and returns its result as the
/// thread's exit code.
pub fn fn_00aa64d0(e: &mut Engine, thread: Ptr<BSThread>) -> u32 {
    e.vcall(thread.addr(), 0x04, &[]).u32()
}

// Translated from 00aa64f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Closes the thread: if there is a handle, clears the active flag, then
/// under the critical section closes the handle and clears it.
pub fn fn_00aa64f0(e: &mut Engine, this: Ptr<BSThread>) {
    if e.get(this, BSThread::m_ThreadHandle) == 0 {
        return;
    }
    e.set(this, BSThread::bThreadIsActive, false);
    e.call(ENTER_CRITICAL_SECTION, &args![critical_section(this)]);
    let handle = e.get(this, BSThread::m_ThreadHandle);
    e.call(CLOSE_HANDLE, &args![handle]);
    e.set(this, BSThread::m_ThreadHandle, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![critical_section(this)]);
}

// Translated from 00aa6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSThread::SetName` (Xbox PDB): does nothing on the PC (one word, unread).
pub fn bs_thread_set_name(_e: &mut Engine, _this: Ptr<BSThread>, _unused_1: u32) {}

// Translated from 00aa6550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSThread::SetThreadProcessor` (Xbox PDB): does nothing on the PC (one
/// word, unread).
pub fn bs_thread_set_thread_processor(_e: &mut Engine, _this: Ptr<BSThread>, _unused_1: u32) {}

// Translated from 00aa6560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSThread::SetThreadPriority` (Xbox PDB): sets the Win32 priority of the
/// thread when it has a handle.
pub fn bs_thread_set_thread_priority(e: &mut Engine, this: Ptr<BSThread>, priority: i32) {
    let handle = e.get(this, BSThread::m_ThreadHandle);
    if handle != 0 {
        e.call(WIN32_SET_THREAD_PRIORITY, &args![handle, priority]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa6360,
            bs_thread_bs_thread(Ptr<BSThread>) -> Ptr<BSThread>
        ),
        entry!(0x00aa63c0, fn_00aa63c0(Ptr<BSThread>)),
        entry!(
            0x00aa6430,
            bs_thread_initialize(Ptr<BSThread>, u32, u32, bool)
        ),
        entry!(0x00aa64d0, fn_00aa64d0(Ptr<BSThread>) -> u32),
        entry!(0x00aa64f0, fn_00aa64f0(Ptr<BSThread>)),
        entry!(0x00aa6540, bs_thread_set_name(Ptr<BSThread>, u32)),
        entry!(
            0x00aa6550,
            bs_thread_set_thread_processor(Ptr<BSThread>, u32)
        ),
        entry!(
            0x00aa6560,
            bs_thread_set_thread_priority(Ptr<BSThread>, i32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMPORTS: [u32; 10] = [
        INITIALIZE_CRITICAL_SECTION,
        DELETE_CRITICAL_SECTION,
        ENTER_CRITICAL_SECTION,
        LEAVE_CRITICAL_SECTION,
        CLOSE_HANDLE,
        GET_CURRENT_THREAD,
        GET_CURRENT_THREAD_ID,
        CREATE_THREAD,
        RESUME_THREAD,
        WIN32_SET_THREAD_PRIORITY,
    ];

    /// Doubles for the Win32 imports: `GetCurrentThread` returns 0xFFFFFFFE,
    /// `GetCurrentThreadId` 77, `CreateThread` 0x1234 and writes id 99.
    fn thread_engine() -> Engine {
        let mut e = Engine::new();
        for slot in IMPORTS {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(GET_CURRENT_THREAD, |_, _| 0xFFFF_FFFEu32.into_ret());
        e.register(GET_CURRENT_THREAD_ID, |_, _| 77u32.into_ret());
        e.register(CREATE_THREAD, |e, a| {
            e.mem.set_u32(a[5], 99);
            0x1234u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| IMPORTS.contains(a))
            .collect()
    }

    #[test]
    fn constructor_clears_fields_and_initializes_the_section() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        for off in (0..0x30).step_by(4) {
            e.mem.set_u32(t.addr() + off, 0xAAAA_AAAA);
        }
        let r = e.call(0x00aa6360, &args![t]).u32();
        assert_eq!(r, t.addr());
        assert_eq!(e.get(t, BSThread::vtable), BS_THREAD_VTABLE);
        assert_eq!(e.get(t, BSThread::m_ThreadHandle), 0);
        assert_eq!(e.get(t, BSThread::m_ParentHandle), 0);
        assert_eq!(e.get(t, BSThread::m_ThreadID), 0);
        assert_eq!(e.get(t, BSThread::m_ParentID), 0);
        assert!(!e.get(t, BSThread::bThreadIsActive));
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![t.addr() + 4]]
        );
    }

    #[test]
    fn destructor_closes_the_handle_then_deletes_the_section() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.set(t, BSThread::m_ThreadHandle, 0x55);
        e.set(t, BSThread::bThreadIsActive, true);
        e.call(0x00aa63c0, &args![t]);
        assert_eq!(e.get(t, BSThread::vtable), BS_THREAD_VTABLE);
        assert_eq!(e.get(t, BSThread::m_ThreadHandle), 0);
        assert!(!e.get(t, BSThread::bThreadIsActive));
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x55]]);
        assert_eq!(
            order(&e),
            vec![
                ENTER_CRITICAL_SECTION,
                CLOSE_HANDLE,
                LEAVE_CRITICAL_SECTION,
                DELETE_CRITICAL_SECTION
            ]
        );
    }

    #[test]
    fn close_without_a_handle_does_nothing() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.set(t, BSThread::bThreadIsActive, true);
        e.call(0x00aa64f0, &args![t]);
        assert!(e.get(t, BSThread::bThreadIsActive));
        assert!(order(&e).is_empty());
    }

    #[test]
    fn initialize_creates_and_resumes_the_thread() {
        for (suspended, flags) in [(true, 4u32), (false, 0)] {
            let mut e = thread_engine();
            let t: Ptr<BSThread> = e.new_object();
            e.call(0x00aa6430, &args![t, 0x10000u32, 0xDEADu32, suspended]);
            assert_eq!(e.get(t, BSThread::m_ParentHandle), 0xFFFF_FFFE);
            assert_eq!(e.get(t, BSThread::m_ParentID), 77);
            assert_eq!(e.get(t, BSThread::m_ThreadID), 99);
            assert_eq!(e.get(t, BSThread::m_ThreadHandle), 0x1234);
            assert!(e.get(t, BSThread::bThreadIsActive));
            assert_eq!(
                calls(&e, CREATE_THREAD),
                vec![vec![
                    0,
                    0x10000,
                    THREAD_START,
                    t.addr(),
                    flags,
                    t.addr() + 0x24
                ]]
            );
            assert_eq!(
                calls(&e, WIN32_SET_THREAD_PRIORITY),
                vec![vec![0x1234, u32::MAX]]
            );
            assert_eq!(calls(&e, RESUME_THREAD), vec![vec![0x1234]]);
            assert_eq!(
                order(&e),
                vec![
                    GET_CURRENT_THREAD,
                    GET_CURRENT_THREAD_ID,
                    CREATE_THREAD,
                    ENTER_CRITICAL_SECTION,
                    WIN32_SET_THREAD_PRIORITY,
                    RESUME_THREAD,
                    LEAVE_CRITICAL_SECTION
                ]
            );
        }
    }

    #[test]
    fn initialize_with_a_thread_does_nothing() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.set(t, BSThread::m_ThreadHandle, 0x77);
        e.call(0x00aa6430, &args![t, 0u32, 0u32, false]);
        assert!(order(&e).is_empty());
        assert!(!e.get(t, BSThread::bThreadIsActive));
    }

    #[test]
    fn start_routine_runs_thread_proc() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.set(t, BSThread::vtable, 0x0300_0000);
        e.register(0x0300_1000, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x00aa64d0, &args![t]).u32(), t.addr() + 1);
    }

    #[test]
    fn set_name_and_processor_do_nothing() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.call(0x00aa6540, &args![t, 1u32]);
        e.call(0x00aa6550, &args![t, 2u32]);
        assert!(order(&e).is_empty());
        assert_eq!(e.mem.bytes(t.addr(), 0x30), vec![0; 0x30]);
    }

    #[test]
    fn set_thread_priority_needs_a_handle() {
        let mut e = thread_engine();
        let t: Ptr<BSThread> = e.new_object();
        e.call(0x00aa6560, &args![t, 2i32]);
        assert!(calls(&e, WIN32_SET_THREAD_PRIORITY).is_empty());
        e.set(t, BSThread::m_ThreadHandle, 0x42);
        e.call(0x00aa6560, &args![t, 2i32]);
        assert_eq!(calls(&e, WIN32_SET_THREAD_PRIORITY), vec![vec![0x42, 2]]);
    }
}
