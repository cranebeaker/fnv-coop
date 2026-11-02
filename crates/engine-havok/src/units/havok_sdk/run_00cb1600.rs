//! `Havok SDK/run_00cb1600` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cb17b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cc_00e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cb17b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cc_00e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkOstream::hkOstream` (`00cc00b0`, `(this, flag)`), `fn_00cc01c0` (`(this, text)`, writes
/// text into the stream) and the stream destructor (`00cc00e0`), all in other units.
const OSTREAM_CONSTRUCT: u32 = 0x00cc_00b0;
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// `hkBufferedStreamWriter` constructor (`00d274c0`, `(this, buffer, size, clear)`).
const BUFFERED_WRITER_CONSTRUCT: u32 = 0x00d2_74c0;
/// Adds a reference to an object when the 16-bit memory size word at +4 is set (`00c903a0`,
/// `CMP word [ECX+4],0; JZ; INC word [ECX+6]`).
const ADD_REFERENCE: u32 = 0x00c9_03a0;
/// The global pointer to the Havok error reporter (`01268360`); its vtable offset 0xc reports
/// a message `(level, id, text, file, line)` and returns non-zero to request a break.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The source file string `.\System\Error\hkError.cpp` (`010c85ac`).
const ERROR_FILE: u32 = 0x010c_85ac;

// Translated from 00cb1600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkErrStream` (the RTTI name of the vtable `010c85a0` this stores; the
/// constructor role is read from the vtable store): runs the `hkOstream` constructor
/// (`00cc00b0`, flag 0), stores the vtable, and, when `buffer + size - 0x20` is non-zero,
/// builds a buffered stream writer (`00d274c0`) in the last 0x20 bytes of the buffer, writing
/// into the first `size - 0x20` bytes with clearing, stores it at +8 and adds a reference
/// to it (`00c903a0`); otherwise +8 is zero. `this` is ECX. Returns `this`. The third word
/// passed to `00d274c0` is the pointer `buffer + size - 0x20` with its low byte replaced by 1,
/// as the exe builds it on the stack.
pub fn hk_err_stream_construct(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> Ptr {
    let t = this.addr();
    e.call(OSTREAM_CONSTRUCT, &args![this, 0u32]);
    let end = buffer.wrapping_add(size).wrapping_sub(0x20);
    e.mem.set_u32(t, 0x010c_85a0);
    if end != 0 {
        let flag = (end & 0xffff_ff00) | 1;
        let writer = e
            .call(
                BUFFERED_WRITER_CONSTRUCT,
                &args![end, buffer, size.wrapping_sub(0x20), flag],
            )
            .u32();
        e.mem.set_u32(t + 8, writer);
        e.call(ADD_REFERENCE, &args![writer]);
    } else {
        e.mem.set_u32(t + 8, 0);
        e.call(ADD_REFERENCE, &args![0u32]);
    }
    this
}

// Translated from 00cb1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reports a Havok error, cdecl `(text)`: builds an `hkErrStream` (`00cb1600`) over a local
/// 0x200-byte buffer, writes `text` into it (`00cc01c0`), asks the error reporter's virtual
/// function at 0xc to report `(3, 0x2636fe25, buffer, ".\System\Error\hkError.cpp", 0x28)` and
/// executes `INT3` (a breakpoint trap, not a call; the translation panics there) when it
/// returns non-zero; then destroys the stream (`00cc00e0`). The function's name in the exe is
/// unknown.
pub fn fn_00cb1750(e: &mut Engine, text: u32) {
    e.with_stack(0x20c, |e, frame| {
        let s = frame.addr();
        hk_err_stream_construct(e, Ptr::new(s), s + 0xc, 0x200);
        e.call(OSTREAM_WRITE_TEXT, &args![s, text]);
        let reporter: u32 = e.global(ERROR_REPORTER);
        let wanted_break = e
            .vcall(
                reporter,
                0xc,
                &args![3u32, 0x2636_fe25u32, s + 0xc, ERROR_FILE, 0x28u32],
            )
            .u32();
        if wanted_break != 0 {
            panic!("00cb1750: the exe's INT3 trap (the error reporter asked for a break)");
        }
        e.call(OSTREAM_DESTRUCT, &args![s]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_17b0, deleting_destructor_00cb17b0(Ptr, u8) -> Ptr),
        entry!(0x00cb_1600, hk_err_stream_construct(Ptr, u32, u32) -> Ptr),
        entry!(0x00cb_1750, fn_00cb1750(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

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
    fn deleting_destructor_00cb17b0_frees_only_when_bit_zero_is_set() {
        check(0x00cb_17b0, 0x00cc_00e0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn the_stream_places_its_writer_at_the_end_of_the_buffer() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, OSTREAM_CONSTRUCT, 0);
        log_double(&mut e, &log, BUFFERED_WRITER_CONSTRUCT, 0x7777);
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cb_1600, &args![this, 0x1000_0000u32, 0x200u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_85a0);
        assert_eq!(e.mem.u32(this + 8), 0x7777);
        let calls = log.borrow();
        assert_eq!(calls[0], (OSTREAM_CONSTRUCT, vec![this, 0]));
        assert_eq!(
            calls[1],
            (
                BUFFERED_WRITER_CONSTRUCT,
                vec![0x1000_01e0, 0x1000_0000, 0x1e0, 0x1000_0101]
            )
        );
        assert_eq!(calls[2], (ADD_REFERENCE, vec![0x7777]));
    }

    #[test]
    fn a_zero_end_pointer_leaves_no_writer() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, OSTREAM_CONSTRUCT, 0);
        log_double(&mut e, &log, BUFFERED_WRITER_CONSTRUCT, 0x7777);
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 5);
        e.call(0x00cb_1600, &args![this, 0x20u32, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[1], (ADD_REFERENCE, vec![0]));
    }

    #[test]
    fn reporting_writes_the_text_and_does_not_break_when_not_asked() {
        reporting(0);
    }

    #[test]
    #[should_panic(expected = "00cb1750: the exe's INT3 trap")]
    fn reporting_traps_with_int3_when_asked() {
        reporting(1);
    }

    fn reporting(wanted: u32) {
        {
            let mut e = test_engine();
            let log: Log = Rc::new(RefCell::new(Vec::new()));
            log_double(&mut e, &log, OSTREAM_CONSTRUCT, 0);
            log_double(&mut e, &log, BUFFERED_WRITER_CONSTRUCT, 0x7777);
            log_double(&mut e, &log, ADD_REFERENCE, 0);
            log_double(&mut e, &log, OSTREAM_WRITE_TEXT, 0);
            log_double(&mut e, &log, OSTREAM_DESTRUCT, 0);
            log_double(&mut e, &log, 0x0900_000c, wanted);
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 0xc, 0x0900_000c);
            let reporter = e.mem.alloc(0x10);
            e.mem.set_u32(reporter, vtable);
            e.set_global(ERROR_REPORTER, reporter);
            e.call(0x00cb_1750, &args![0x5555u32]);
            let calls = log.borrow();
            let write = calls.iter().find(|c| c.0 == OSTREAM_WRITE_TEXT).unwrap();
            assert_eq!(write.1[1], 0x5555);
            let report = calls.iter().find(|c| c.0 == 0x0900_000c).unwrap();
            assert_eq!(report.1[0], reporter);
            assert_eq!(report.1[1..3], [3, 0x2636_fe25]);
            assert_eq!(report.1[4..], [ERROR_FILE, 0x28]);
            assert_eq!(calls.last().unwrap().0, OSTREAM_DESTRUCT);
        }
    }
}
