//! `(unplaced)/run_00ec7ad2` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT `_memset` (`00ec61c0`, cdecl: destination, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// The import slot of KERNEL32 `IsDebuggerPresent` (`00fdf0f0`).
const IS_DEBUGGER_PRESENT: u32 = 0x00fd_f0f0;
/// The import slot of KERNEL32 `SetUnhandledExceptionFilter` (`00fdf180`, one word).
const SET_UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f180;
/// The import slot of KERNEL32 `UnhandledExceptionFilter` (`00fdf17c`, one word).
const UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f17c;
/// The import slot of KERNEL32 `GetCurrentProcess` (`00fdf110`).
const GET_CURRENT_PROCESS: u32 = 0x00fd_f110;
/// The import slot of KERNEL32 `TerminateProcess` (`00fdf178`, process, exit code).
const TERMINATE_PROCESS: u32 = 0x00fd_f178;
/// `00ed5bae` (no name; its body is `AND [01270a60], 0`, whatever argument it is given; the CRT
/// pushes a 2 here).
const CLEAR_01270A60: u32 = 0x00ed_5bae;
/// The exit code and exception code of the invalid-parameter failure
/// (`STATUS_INVALID_CRUNTIME_PARAMETER`).
const INVALID_CRUNTIME_PARAMETER: u32 = 0xc000_0417;

/// The frame `__invoke_watson` builds below `EBP` (0x328 bytes), as offsets from its start:
/// the `EXCEPTION_RECORD` (0x50 bytes), the `EXCEPTION_POINTERS` (record, context) and the
/// `CONTEXT` (0x2cc bytes).
const RECORD: u32 = 0;
const POINTERS: u32 = 0x50;
const CONTEXT: u32 = 0x58;
/// `EBP` is this far above the frame's start.
const EBP: u32 = 0x328;

// Translated from 00ec7b08 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec7b08 - the CONTEXT's registers (ECX, EDX, EBX, ESI, EDI, EBP, EIP from the return address, the segment registers and EFLAGS) are machine state the engine does not have and stay zero; the /GS cookie check (00ec408c) after TerminateProcess is unreachable and not translated
/// `__invoke_watson` (Ghidra label from its function ID; the CRT's invalid-parameter failure
/// path, cdecl, five words that are never read): builds an `EXCEPTION_RECORD` with code
/// `0xc0000417` and flags 1 (`_memset` clears everything after the code first) and a `CONTEXT`
/// with `ContextFlags` `0x10001`, `Eax` the context's own address and `Esp` the frame's `EBP + 4`,
/// all in a 0x328-byte frame. Then `IsDebuggerPresent`, `SetUnhandledExceptionFilter(NULL)` and
/// `UnhandledExceptionFilter(&{record, context})`; when that returns 0 and no debugger is present
/// it calls `00ed5bae(2)`. It ends with `TerminateProcess(GetCurrentProcess(), 0xc0000417)`.
/// Nothing returns from that, so the engine stops with a panic naming this address and the reason.
pub fn fn_00ec7b08(
    e: &mut Engine,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let frame = e.mem.alloc(EBP + 8);
    let ebp = frame + EBP;
    // AND [EBP - 0x328], 0 (the exception code), then the memset of the 0x4c bytes after it.
    e.mem.set_u32(frame + RECORD, 0);
    e.call(MEMSET, &args![frame + RECORD + 4, 0u32, 0x4cu32]);
    e.mem.set_u32(frame + POINTERS, frame + RECORD);
    e.mem.set_u32(frame + POINTERS + 4, frame + CONTEXT);
    e.mem.set_u32(frame + CONTEXT + 0xb0, frame + CONTEXT); // Eax
    e.mem.set_u32(frame + CONTEXT + 0xc4, ebp + 4); // Esp
    e.mem.set_u32(frame + CONTEXT, 0x0001_0001); // ContextFlags
    e.mem.set_u32(frame + RECORD, INVALID_CRUNTIME_PARAMETER);
    e.mem.set_u32(frame + RECORD + 4, 1); // ExceptionFlags
    let debugger_present = e.call(IS_DEBUGGER_PRESENT, &args![]).u32();
    e.call(SET_UNHANDLED_EXCEPTION_FILTER, &args![0u32]);
    let filter = e
        .call(UNHANDLED_EXCEPTION_FILTER, &args![frame + POINTERS])
        .u32();
    if filter == 0 && debugger_present == 0 {
        e.call(CLEAR_01270A60, &args![2u32]);
    }
    let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
    e.call(
        TERMINATE_PROCESS,
        &args![process, INVALID_CRUNTIME_PARAMETER],
    );
    panic!(
        "00ec7b08 __invoke_watson: TerminateProcess(GetCurrentProcess(), 0xc0000417): \
         the CRT invalid-parameter failure ended the process"
    );
}

/// The `.data` word `00ec7af9` sets (`__init_pointers` at `00eccf26` passes it its value).
const POINTER_0126FD9C: u32 = 0x0126_fd9c;

// Translated from 00ec7af9 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (a CRT pointer initialiser, cdecl, one word; `__init_pointers` calls it): stores
/// its argument in the word at `0126fd9c`.
pub fn fn_00ec7af9(e: &mut Engine, value: u32) {
    e.mem.set_u32(POINTER_0126FD9C, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_7b08, fn_00ec7b08(u32, u32, u32, u32, u32)),
        entry!(0x00ec_7af9, fn_00ec7af9(u32)),
        entry!(0x00ec_7ad2, bad_typeid_deleting_dtor(Ptr, u32) -> Ptr),
    ]
}

/// `std::bad_typeid`'s vtable (`010f1fd0`; RTTI `.?AVbad_typeid@std@@`).
const BAD_TYPEID_VTABLE: u32 = 0x010f_1fd0;
/// `std::exception::~exception` (engine map, `00ec79a1`; thiscall) and `operator delete`
/// (`00401030`, cdecl: the block).
const EXCEPTION_DTOR: u32 = 0x00ec_79a1;
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00ec7ad2 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of `std::bad_typeid` (slot 0 of its vtable, by its RTTI; unnamed
/// in the engine map): the vtable, `std::exception::~exception`, then with bit 0 of `flags`
/// `operator delete`. Returns `this`.
pub fn bad_typeid_deleting_dtor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BAD_TYPEID_VTABLE);
    e.call(EXCEPTION_DTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn destroys_and_deletes_with_bit_0() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(EXCEPTION_DTOR, |e, a| {
                assert_eq!(e.mem.u32(a[0]), BAD_TYPEID_VTABLE);
                Ret::default()
            });
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            let this = e.mem.alloc(0xc);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ec_7ad2, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (EXCEPTION_DTOR, vec![this]));
            assert_eq!(
                log.iter().filter(|c| c.0 == OPERATOR_DELETE).count(),
                flags as usize
            );
        }
    }
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    #[test]
    fn stores_the_argument_in_the_pointer_word() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.call(0x00ec_7af9, &args![0x1234_5678u32]);
        assert_eq!(e.mem.u32(POINTER_0126FD9C), 0x1234_5678);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles over everything `__invoke_watson` calls. `memset` really fills, the filter
    /// double answers `filter` and records the record and context words it is given,
    /// `IsDebuggerPresent` answers `debugger`.
    fn setup(e: &mut Engine, debugger: u32, filter: u32) -> Log {
        let log: Log = Rc::default();
        let l = log.clone();
        e.register_double(MEMSET, move |e, a| {
            l.borrow_mut().push((MEMSET, a.to_vec()));
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(IS_DEBUGGER_PRESENT, move |_, a| {
            l.borrow_mut().push((IS_DEBUGGER_PRESENT, a.to_vec()));
            Ret {
                eax: debugger,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(SET_UNHANDLED_EXCEPTION_FILTER, move |_, a| {
            l.borrow_mut()
                .push((SET_UNHANDLED_EXCEPTION_FILTER, a.to_vec()));
            Ret::default()
        });
        let l = log.clone();
        e.register_double(UNHANDLED_EXCEPTION_FILTER, move |e, a| {
            let pointers = a[0];
            let record = e.mem.u32(pointers);
            let context = e.mem.u32(pointers + 4);
            let words = vec![
                a[0],
                record,
                context,
                e.mem.u32(record),         // ExceptionCode
                e.mem.u32(record + 4),     // ExceptionFlags
                e.mem.u32(record + 8),     // ExceptionRecord
                e.mem.u32(record + 0x4c),  // last word of the record
                e.mem.u32(context),        // ContextFlags
                e.mem.u32(context + 0xb0), // Eax
                e.mem.u32(context + 0xc4), // Esp
            ];
            l.borrow_mut().push((UNHANDLED_EXCEPTION_FILTER, words));
            Ret {
                eax: filter,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(CLEAR_01270A60, move |_, a| {
            l.borrow_mut().push((CLEAR_01270A60, a.to_vec()));
            Ret::default()
        });
        let l = log.clone();
        e.register_double(GET_CURRENT_PROCESS, move |_, a| {
            l.borrow_mut().push((GET_CURRENT_PROCESS, a.to_vec()));
            Ret {
                eax: 0xffff_ffff,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(TERMINATE_PROCESS, move |_, a| {
            l.borrow_mut().push((TERMINATE_PROCESS, a.to_vec()));
            Ret::default()
        });
        log
    }

    fn run(e: &mut Engine) -> String {
        let result = catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00ec_7b08, &args![1u32, 2u32, 3u32, 4u32, 5u32])
        }));
        let payload = result.expect_err("the process ends: the engine panics");
        match payload.downcast_ref::<String>() {
            Some(text) => text.clone(),
            None => (*payload.downcast_ref::<&str>().unwrap()).to_string(),
        }
    }

    #[test]
    fn builds_the_record_and_context_and_ends_the_process() {
        let mut e = Engine::new();
        let log = setup(&mut e, 0, 0);
        let message = run(&mut e);
        assert!(message.contains("00ec7b08"));
        assert!(message.contains("0xc0000417"));
        let log = log.borrow();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                MEMSET,
                IS_DEBUGGER_PRESENT,
                SET_UNHANDLED_EXCEPTION_FILTER,
                UNHANDLED_EXCEPTION_FILTER,
                CLEAR_01270A60,
                GET_CURRENT_PROCESS,
                TERMINATE_PROCESS,
            ]
        );
        // memset(record + 4, 0, 0x4c)
        assert_eq!(log[0].1[1..], [0, 0x4c]);
        assert_eq!(log[2].1, vec![0]);
        let filter = &log[3].1;
        let (pointers, record, context) = (filter[0], filter[1], filter[2]);
        assert_eq!(pointers, record + POINTERS);
        assert_eq!(context, record + CONTEXT);
        assert_eq!(log[0].1[0], record + 4);
        // code, flags, nested record (cleared), last word (cleared), ContextFlags, Eax, Esp
        assert_eq!(filter[3], 0xc000_0417);
        assert_eq!(filter[4], 1);
        assert_eq!(filter[5], 0);
        assert_eq!(filter[6], 0);
        assert_eq!(filter[7], 0x0001_0001);
        assert_eq!(filter[8], context, "Eax holds the context's own address");
        assert_eq!(filter[9], record + EBP + 4, "Esp is EBP + 4");
        assert_eq!(log[4].1, vec![2]);
        assert_eq!(log[6].1, vec![0xffff_ffff, 0xc000_0417]);
    }

    #[test]
    fn a_debugger_or_a_handled_filter_skips_the_hook() {
        for (debugger, filter) in [(1, 0), (0, 1), (1, 1)] {
            let mut e = Engine::new();
            let log = setup(&mut e, debugger, filter);
            run(&mut e);
            let log = log.borrow();
            assert!(
                log.iter().all(|c| c.0 != CLEAR_01270A60),
                "debugger {debugger}, filter {filter}"
            );
            assert_eq!(log.last().unwrap().0, TERMINATE_PROCESS);
        }
    }
}
