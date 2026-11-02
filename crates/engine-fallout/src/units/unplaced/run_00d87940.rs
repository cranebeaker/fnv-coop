//! `(unplaced)/run_00d87940` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of KERNEL32 `CreateThread` (`00fdf124`, stdcall, six words).
const CREATE_THREAD: u32 = 0x00fd_f124;
/// The import slot of KERNEL32 `RaiseException` (`00fdf154`, stdcall, four words).
const RAISE_EXCEPTION: u32 = 0x00fd_f154;
/// The exception code of the debugger's "name this thread" convention (`MS_VC_EXCEPTION`).
const THREAD_NAME_EXCEPTION: u32 = 0x406d_1388;
/// `dwType` of its `THREADNAME_INFO`.
const THREAD_NAME_INFO_TYPE: u32 = 0x1000;

// Translated from 00d87940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d87940 - the FS:[0] SEH frame (scope table 0117f7d8, handler 00ece104) that swallows the thread-name exception is not modelled
/// No map name. Thiscall with four stack words (`RET 0x10`): starts a thread and names it for
/// the debugger. `this` is the thread object: it gets the handle at `+0`, the thread id at `+8`
/// and zero at `+0xc`. The words are the start routine, its parameter, the name (a C string
/// address) and the stack size. Its caller `00d2d110` passes `hkWorkerThreadFunc` (Ghidra's
/// label) as the start routine.
///
/// `CreateThread(NULL, stack_size, start, parameter, 0, &id)`: the id out parameter is the very
/// stack slot that held `parameter` (`[EBP + 0xc]`), so a `CreateThread` that leaves it alone
/// leaves `parameter` as the id. The handle is stored first; a null handle returns 1 and touches
/// nothing else. Otherwise the id and a cleared `+0xc` are stored and a `THREADNAME_INFO`
/// `{ 0x1000, name, id, 0 }` is raised as exception `0x406d1388` (`RaiseException(code, 0, 4,
/// &info)`) inside a `__try` whose handler just continues; the result is 0.
pub fn fn_00d87940(
    e: &mut Engine,
    this: Ptr,
    start: u32,
    parameter: u32,
    name: Ptr,
    stack_size: u32,
) -> u32 {
    let (handle, thread_id) = e.with_stack(4, |e, id_slot| {
        // The parameter's own stack slot doubles as the thread-id out parameter.
        e.mem.set_u32(id_slot.addr(), parameter);
        let handle = e
            .call(
                CREATE_THREAD,
                &args![0u32, stack_size, start, parameter, 0u32, id_slot],
            )
            .u32();
        (handle, e.mem.u32(id_slot.addr()))
    });
    e.mem.set_u32(this.addr(), handle);
    if handle == 0 {
        return 1;
    }
    e.mem.set_u32(this.addr().wrapping_add(8), thread_id);
    e.mem.set_u32(this.addr().wrapping_add(0xc), 0);
    e.with_stack(16, |e, info| {
        e.mem.set_u32(info.addr(), THREAD_NAME_INFO_TYPE);
        e.mem.set_u32(info.addr().wrapping_add(4), name.addr());
        e.mem.set_u32(info.addr().wrapping_add(8), thread_id);
        e.mem.set_u32(info.addr().wrapping_add(12), 0);
        e.call(
            RAISE_EXCEPTION,
            &args![THREAD_NAME_EXCEPTION, 0u32, 4u32, info],
        );
    });
    0
}

/// `GetCurrentThreadId` import slot (`kernel32`, no arguments).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;

// Translated from 00d87a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the calling thread's id (`GetCurrentThreadId`) as a 64-bit
/// value (`EDX` cleared).
pub fn fn_00d87a40(e: &mut Engine) -> u64 {
    let id = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    u64::from(id)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_7940, fn_00d87940(Ptr, u32, u32, Ptr, u32) -> u32),
        entry!(0x00d8_7a40, fn_00d87a40() -> u64),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn thread_id_is_widened_with_a_zero_high_word() {
        let mut e = Engine::new();
        e.register(GET_CURRENT_THREAD_ID, |_, _| Ret {
            eax: 0x1234,
            edx: 0xdead,
            ..Ret::default()
        });
        let r = e.call(0x00d8_7a40, &args![]);
        assert_eq!((r.eax, r.edx), (0x1234, 0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles over both imports: `CreateThread` answers `handle` and writes `id` to the id
    /// slot (unless `id` is `None`); `RaiseException` copies the four info words it is given
    /// into the log. Returns the log of (slot, words) for each call, info words appended.
    fn setup(e: &mut Engine, handle: u32, id: Option<u32>) -> Log {
        let log: Log = Rc::default();
        let create_log = log.clone();
        e.register_double(CREATE_THREAD, move |e, a| {
            create_log.borrow_mut().push((CREATE_THREAD, a.to_vec()));
            if let Some(id) = id {
                e.mem.set_u32(a[5], id);
            }
            Ret {
                eax: handle,
                ..Ret::default()
            }
        });
        let raise_log = log.clone();
        e.register_double(RAISE_EXCEPTION, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|i| e.mem.u32(a[3] + 4 * i)));
            raise_log.borrow_mut().push((RAISE_EXCEPTION, words));
            Ret::default()
        });
        log
    }

    #[test]
    fn creates_the_thread_records_it_and_names_it() {
        let mut e = Engine::new();
        let log = setup(&mut e, 0x4444, Some(0x1abc));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        let result = e.call(
            0x00d8_7940,
            &args![this, 0x00d8_0000u32, 0x5555u32, 0x0123_4560u32, 0x20000u32],
        );
        assert_eq!(result.u32(), 0);
        assert_eq!(e.mem.u32(this), 0x4444);
        assert_eq!(e.mem.u32(this + 8), 0x1abc);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        let create = &log[0];
        assert_eq!(create.0, CREATE_THREAD);
        // security, stack size, start routine, parameter, flags, id out (a stack slot)
        assert_eq!(create.1[..5], [0, 0x20000, 0x00d8_0000, 0x5555, 0]);
        let raise = &log[1];
        assert_eq!(raise.0, RAISE_EXCEPTION);
        // code, flags, argument count, pointer, then the THREADNAME_INFO words
        assert_eq!(raise.1[..3], [0x406d_1388, 0, 4]);
        assert_eq!(raise.1[4..], [0x1000, 0x0123_4560, 0x1abc, 0]);
    }

    #[test]
    fn a_failed_create_stores_only_the_null_handle_and_returns_one() {
        let mut e = Engine::new();
        let log = setup(&mut e, 0, Some(0x77));
        let this = e.mem.alloc(0x10);
        for offset in [0, 4, 8, 12] {
            e.mem.set_u32(this + offset, 0x9999_9999);
        }
        let result = e.call(0x00d8_7940, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(result.u32(), 1);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0x9999_9999);
        assert_eq!(e.mem.u32(this + 8), 0x9999_9999);
        assert_eq!(e.mem.u32(this + 12), 0x9999_9999);
        assert_eq!(log.borrow().len(), 1, "no exception is raised");
    }

    #[test]
    fn an_untouched_id_slot_still_holds_the_parameter() {
        let mut e = Engine::new();
        let log = setup(&mut e, 0x4444, None);
        let this = e.mem.alloc(0x10);
        e.call(0x00d8_7940, &args![this, 1u32, 0x5555u32, 0x100u32, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0x5555);
        assert_eq!(log.borrow()[1].1[4 + 2], 0x5555);
    }
}
