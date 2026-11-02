//! `Havok SDK/hkdefaulterror.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkMapBase::getWithDefault` (`00cb1930`, `this` = the map, `(key, default)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// The map's clear routine (`00cb1a60`, `this` = the map).
const MAP_CLEAR: u32 = 0x00cb_1a60;
/// The map's insert (`00cb1870`, `this` = the map, `(allocator, key, value)`).
const MAP_INSERT: u32 = 0x00cb_1870;
/// The map's erase (`00cb1fb0`, `this` = the map, `(key)`).
const MAP_ERASE: u32 = 0x00cb_1fb0;
/// The map's reserve (`00cb1fe0`, `this` = the map, `(allocator, capacity)`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkMapBase::clearAndDeallocate` (`00cb1a90`, `this` = the map, `(allocator)`).
const MAP_DEALLOCATE: u32 = 0x00cb_1a90;
/// The map's destructor (`00a29680`, `this` = the map).
const MAP_DESTRUCT: u32 = 0x00a2_9680;
/// `hkString::snprintf` (`00cb0f60`, cdecl `(buffer, size, format, ...)`).
const FORMAT_INTO: u32 = 0x00cb_0f60;
/// `hkArrayUtil` growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Makes the stack tracer (`00da2bc0`, answers the new object).
const STACK_TRACER_CREATE: u32 = 0x00da_2bc0;
/// `hkReferencedObject::removeReferenceLockUnchecked` (`00c903b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_03b0;
/// `hkStackTracer::getStackTrace` (`00da2b20`, `this` = the tracer, `(frames, capacity)`,
/// answers the frame count).
const GET_STACK_TRACE: u32 = 0x00da_2b20;
/// The stack trace printer (`00da2990`, `this` = the tracer, `(frames, count, callback,
/// user)`; the map names it `hkLeakDetectAllocator::quit`).
const PRINT_STACK_TRACE: u32 = 0x00da_2990;
/// The exe's global holding the stack tracer.
const STACK_TRACER: u32 = 0x0126_9a84;
/// The exe's format strings `"%s\n"` (`010ce5cc`) and `"%s(%d): [0x%08X] %s : %s\n"`
/// (`010ce5b0`), and the text `"Stack trace is:\n"` (`010ce59c`).
const FORMAT_TEXT: u32 = 0x010c_e5cc;
const FORMAT_LOCATED: u32 = 0x010c_e5b0;
const TRACE_HEADER: u32 = 0x010c_e59c;
/// The labels of the four severities: `"Report"`, `"Warning"`, `"Assert"` and `"Error"`, and
/// the empty text used above 3.
const LABEL_REPORT: u32 = 0x0104_df50;
const LABEL_WARNING: u32 = 0x0104_df48;
const LABEL_ASSERT: u32 = 0x0104_df40;
const LABEL_ERROR: u32 = 0x0104_df38;
const LABEL_NONE: u32 = 0x0101_1584;

// Translated from 00d2b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::isEnabled` (Xbox PDB; `this` = the error handler, then `(out, id)`,
/// `RET 8`): the ids that are switched off are the keys of the map at `+8`; stores 1 at the
/// byte `out` when `id` is not found there (looked up with the default 0) and 0 when it is.
/// Answers `out`.
pub fn hk_default_error_is_enabled(e: &mut Engine, this: Ptr, out: Ptr, id: u32) -> Ptr {
    let value = e
        .call(MAP_GET_WITH_DEFAULT, &args![this.addr() + 8, id, 0u32])
        .u32();
    e.mem.set_u8(out.addr(), u8::from(value == 0));
    out
}

// Translated from 00d2b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::enableAll` (Xbox PDB): jumps to the map's clear routine (`00cb1a60`) with
/// the map at `this + 8`.
pub fn hk_default_error_enable_all(e: &mut Engine, this: Ptr) {
    e.call(MAP_CLEAR, &args![this.addr() + 8]);
}

// Translated from 00d2b3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::showMessage` (Xbox PDB; `this` = the error handler, then `(label, id,
/// text, file, line, trace)`, `RET 0x18`, `trace` only counts as its low byte): formats the
/// message into a 0x800-byte local buffer with `hkString::snprintf` (`00cb0f60`): `"%s\n"`
/// with `text` when both `id` and `file` are 0, otherwise `"%s(%d): [0x%08X] %s : %s\n"` with
/// `file`, `line`, `id`, `label` and `text`. The last byte of the buffer is set to 0 and the
/// handler's callback (`+0x20`) is called with the buffer and the user word (`+0x24`). When
/// `trace` is not 0 it makes sure the stack tracer global (`01269a84`) exists (a new one from
/// `00da2bc0` replaces a missing one; one that appeared in the meantime is released with
/// `00c903b0`), captures up to 20 frames (`00da2b20`) and, when more than 2 came back, passes
/// `"Stack trace is:\n"` to the callback and has the tracer print the frames after the first
/// two (`00da2990`).
pub fn hk_default_error_show_message(
    e: &mut Engine,
    this: Ptr,
    label: u32,
    id: u32,
    text: u32,
    file: u32,
    line: u32,
    trace: u8,
) {
    let handler = this.addr();
    e.with_stack(0x800, |e, buffer| {
        let buffer = buffer.addr();
        if id == 0 && file == 0 {
            e.call(FORMAT_INTO, &args![buffer, 0x800u32, FORMAT_TEXT, text]);
        } else {
            e.call(
                FORMAT_INTO,
                &args![
                    buffer,
                    0x800u32,
                    FORMAT_LOCATED,
                    file,
                    line,
                    id,
                    label,
                    text
                ],
            );
        }
        let callback = e.mem.u32(handler + 0x20);
        let user = e.mem.u32(handler + 0x24);
        e.mem.set_u8(buffer + 0x7ff, 0);
        e.call(callback, &args![buffer, user]);
    });
    if trace == 0 {
        return;
    }
    if e.global::<u32>(STACK_TRACER) == 0 {
        let created = e.call(STACK_TRACER_CREATE, &args![]).u32();
        let current = e.global::<u32>(STACK_TRACER);
        if current != 0 {
            e.call(REMOVE_REFERENCE, &args![current]);
        }
        e.set_global(STACK_TRACER, created);
    }
    e.with_stack(0x50, |e, frames| {
        // 00d2b4a1: `00da2b20` (RET 8) is called with ECX = the tracer (01269a84, loaded at
        // 00d2b471 or set at 00d2b492).
        let tracer = e.global::<u32>(STACK_TRACER);
        let count = e
            .call(GET_STACK_TRACE, &args![tracer, frames, 0x14u32])
            .u32() as i32;
        if count > 2 {
            let callback = e.mem.u32(handler + 0x20);
            let user = e.mem.u32(handler + 0x24);
            e.call(callback, &args![TRACE_HEADER, user]);
            let callback = e.mem.u32(handler + 0x20);
            let user = e.mem.u32(handler + 0x24);
            let tracer = e.global::<u32>(STACK_TRACER);
            e.call(
                PRINT_STACK_TRACE,
                &args![
                    tracer,
                    frames.addr() + 8,
                    (count - 2) as u32,
                    callback,
                    user
                ],
            );
        }
    });
}

// Translated from 00d2b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::message` (Xbox PDB; `this` = the error handler, then `(severity, id,
/// text, file, line)`, `RET 0x14`): an `id` of -1 is replaced by the top of the section stack
/// (`+0x14`, size at `+0x18`) when that is not empty. Asks the handler's virtual method at
/// `+0x14` (`isEnabled`) about the id and answers 0 when it is off. Otherwise picks the label
/// by severity (0 `Report`, 1 `Warning`, 2 `Assert`, 3 `Error`, anything above an empty text)
/// and a trace flag (set for 2 and 3), calls `showMessage` (`00d2b3e0`) and answers 1 for the
/// severities 2 and 3, otherwise 0. The exe keeps the out byte of `isEnabled` in the stack
/// slot of the `id` argument, so the word handed to `showMessage` as the flag has the old
/// bytes of `id` above the flag byte; `showMessage` reads only the low byte.
pub fn hk_default_error_message(
    e: &mut Engine,
    this: Ptr,
    severity: u32,
    id: u32,
    text: u32,
    file: u32,
    line: u32,
) -> u32 {
    let handler = this.addr();
    let mut id = id;
    let sections = e.mem.u32(handler + 0x18);
    if id == 0xffff_ffff && sections != 0 {
        let stack = e.mem.u32(handler + 0x14);
        id = e
            .mem
            .u32(stack.wrapping_add(sections.wrapping_mul(4)).wrapping_sub(4));
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let out = e.vcall(handler, 0x14, &args![slot, id]).u32();
        if e.mem.u8(out) == 0 {
            return 0;
        }
        let (label, trace) = match severity {
            0 => (LABEL_REPORT, 0),
            1 => (LABEL_WARNING, 0),
            2 => (LABEL_ASSERT, 1),
            3 => (LABEL_ERROR, 1),
            _ => (LABEL_NONE, 0),
        };
        hk_default_error_show_message(e, this, label, id, text, file, line, trace);
        u32::from(severity == 2 || severity == 3)
    })
}

// Translated from 00d2b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::sectionEnd` (Xbox PDB): lowers the section stack size (`+0x18`) by one.
pub fn hk_default_error_section_end(e: &mut Engine, this: Ptr) {
    let size = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x18, size.wrapping_sub(1));
}

// Translated from 00d2b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::setEnabled` (Xbox PDB; `this` = the error handler, then `(id, enabled)`,
/// `RET 8`, `enabled` only counts as its low byte): switching an id on erases it from the map
/// at `+8` (`00cb1fb0`), switching it off inserts it with the value 1 (`00cb1870`, with the
/// memory router's allocator).
pub fn hk_default_error_set_enabled(e: &mut Engine, this: Ptr, id: u32, enabled: u8) {
    let map = this.addr() + 8;
    if enabled != 0 {
        e.call(MAP_ERASE, &args![map, id]);
    } else {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_INSERT, &args![map, allocator, id, 1u32]);
    }
}

// Translated from 00d2b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::sectionBegin` (Xbox PDB; `this` = the error handler, then two stack words
/// of which only the first is used, `RET 8`): pushes `id` on the section stack (the array at
/// `+0x14`: data, size, capacity word), growing it through `00c90940` first when the size
/// equals the capacity.
pub fn hk_default_error_section_begin(e: &mut Engine, this: Ptr, id: u32, _unused_1: u32) {
    let array = this.addr() + 0x14;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), id);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d2b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (`this` = the error handler, then `(callback, user)`, `RET 8`; the class
/// name is not in the map): sets the word at `+6` to 1, the vtable to `0x010CE5D4`, makes the
/// map at `+8` (`0`, `0`, `-1`, then reserve with the memory router's allocator and 0
/// through `00cb1fe0`), empties the section stack (`+0x14`: 0, 0, capacity word `0x80000000`)
/// and keeps the callback (`+0x20`) and its user word (`+0x24`). Answers `this`.
pub fn fn_00d2b640(e: &mut Engine, this: Ptr, callback: u32, user: u32) -> Ptr {
    let handler = this.addr();
    e.mem.set_u16(handler + 6, 1);
    e.mem.set_u32(handler, 0x010c_e5d4);
    let map = handler + 8;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.mem.set_u32(handler + 0x14, 0);
    e.mem.set_u32(handler + 0x18, 0);
    e.mem.set_u32(handler + 0x1c, 0x8000_0000);
    e.mem.set_u32(handler + 0x24, user);
    e.mem.set_u32(handler + 0x20, callback);
    this
}

// Translated from 00d2b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultError::~hkDefaultError` (Xbox PDB): empties the section stack (size 0; its
/// storage goes back to the memory router's allocator, virtual call at `+0x10` with the data
/// and the byte size, unless the capacity word marks the storage as not owned by bit 31;
/// data cleared, capacity word `0x80000000`), clears and deallocates the map at `+8`
/// (`00cb1a90`) and destroys it (`00a29680`), then stores the base vtable `0x0102E388`.
pub fn hk_default_error_destruct(e: &mut Engine, this: Ptr) {
    let handler = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(handler + 0x1c);
    e.mem.set_u32(handler + 0x18, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        let data = e.mem.u32(handler + 0x14);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(handler + 0x14, 0);
    e.mem.set_u32(handler + 0x1c, 0x8000_0000);
    let map = handler + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_DESTRUCT, &args![map]);
    e.mem.set_u32(handler, 0x0102_e388);
}

// Translated from 00d2b710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_b6b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2b710(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_b6b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_b3b0, hk_default_error_is_enabled(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d2_b3d0, hk_default_error_enable_all(Ptr)),
        entry!(
            0x00d2_b3e0,
            hk_default_error_show_message(Ptr, u32, u32, u32, u32, u32, u8)
        ),
        entry!(0x00d2_b4f0, hk_default_error_message(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d2_b5b0, hk_default_error_section_end(Ptr)),
        entry!(0x00d2_b5c0, hk_default_error_set_enabled(Ptr, u32, u8)),
        entry!(0x00d2_b600, hk_default_error_section_begin(Ptr, u32, u32)),
        entry!(0x00d2_b640, fn_00d2b640(Ptr, u32, u32) -> Ptr),
        entry!(0x00d2_b6b0, hk_default_error_destruct(Ptr)),
        entry!(0x00d2_b710, deleting_destructor_00d2b710(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const CALLBACK_DOUBLE: u32 = 0x0900_0021;
    const PRINTED_FREE_DOUBLE: u32 = 0x0900_0031;

    type Call = (u32, Vec<u32>);

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
    fn deleting_destructor_00d2b710_frees_only_when_bit_zero_is_set() {
        check(0x00d2_b710, 0x00d2_b6b0);
    }

    /// An engine with the stack tracer global mapped, a memory router whose allocator
    /// (`allocator`) has its free slot (`+0x10`) on a double, a zero-answering double for
    /// every outside callee and the call log on. Answers `(engine, allocator)`.
    fn error_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        // The page of the exe's text constants: the callback double reads the byte at +0x7ff of
        // every text it is given, the trace header included.
        e.map(0x010c_e000, 0x1000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, PRINTED_FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        for address in [
            MAP_GET_WITH_DEFAULT,
            MAP_CLEAR,
            MAP_INSERT,
            MAP_ERASE,
            MAP_RESERVE,
            MAP_DEALLOCATE,
            MAP_DESTRUCT,
            FORMAT_INTO,
            ARRAY_RESERVE_MORE,
            STACK_TRACER_CREATE,
            REMOVE_REFERENCE,
            GET_STACK_TRACE,
            PRINT_STACK_TRACE,
            CALLBACK_DOUBLE,
            PRINTED_FREE_DOUBLE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.call_log = Some(Vec::new());
        (e, allocator)
    }

    /// The logged calls to `addresses`, in order.
    fn calls_to(e: &Engine, addresses: &[u32]) -> Vec<Call> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addresses.contains(a))
            .cloned()
            .collect()
    }

    /// The argument words of every logged call to `address`.
    fn logged(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        calls_to(e, &[address])
            .into_iter()
            .map(|(_, words)| words)
            .collect()
    }

    /// A handler block whose vtable slot `+0x14` is `isEnabled` itself (`00d2b3b0`) and
    /// whose callback is [`CALLBACK_DOUBLE`] with the user word `0x7777`.
    fn handler(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, 0x00d2_b3b0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x20, CALLBACK_DOUBLE);
        e.mem.set_u32(this + 0x24, 0x7777);
        this
    }

    #[test]
    fn is_enabled_stores_whether_the_id_is_missing_from_the_map() {
        let (mut e, _) = error_engine();
        e.register(MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: u32::from(a[1] == 5),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00d2_b3b0, &args![this, out, 5u32]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.call(0x00d2_b3b0, &args![this, out, 6u32]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            logged(&e, MAP_GET_WITH_DEFAULT),
            vec![vec![this + 8, 5, 0], vec![this + 8, 6, 0]]
        );
    }

    #[test]
    fn enable_all_clears_the_map() {
        let (mut e, _) = error_engine();
        let this = e.mem.alloc(0x40);
        e.call(0x00d2_b3d0, &args![this]);
        assert_eq!(logged(&e, MAP_CLEAR), vec![vec![this + 8]]);
    }

    #[test]
    fn set_enabled_erases_on_and_inserts_off() {
        let (mut e, allocator) = error_engine();
        let this = e.mem.alloc(0x40);
        e.call(0x00d2_b5c0, &args![this, 0x1234u32, 0x100u32]);
        // Only the low byte counts: 0x100 is "off".
        assert!(logged(&e, MAP_ERASE).is_empty());
        assert_eq!(
            logged(&e, MAP_INSERT),
            vec![vec![this + 8, allocator, 0x1234, 1]]
        );
        e.call(0x00d2_b5c0, &args![this, 0x4321u32, 1u32]);
        assert_eq!(logged(&e, MAP_ERASE), vec![vec![this + 8, 0x4321]]);
        assert_eq!(logged(&e, MAP_INSERT).len(), 1);
    }

    #[test]
    fn section_end_lowers_the_size() {
        let (mut e, _) = error_engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 3);
        e.call(0x00d2_b5b0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x18), 2);
    }

    #[test]
    fn section_begin_pushes_the_id() {
        let (mut e, allocator) = error_engine();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(16);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x1c, 0x8000_0004);
        e.call(0x00d2_b600, &args![this, 0xabcdu32, 0x9999u32]);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(data + 4), 0xabcd);
        assert!(logged(&e, ARRAY_RESERVE_MORE).is_empty());
        // Full: the growth routine is asked first.
        e.mem.set_u32(this + 0x1c, 0x8000_0002);
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let grown = e.mem.alloc(32);
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00d2_b600, &args![this, 0x55u32, 0u32]);
        assert_eq!(
            logged(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, this + 0x14, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 3);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x14) + 8), 0x55);
    }

    #[test]
    fn construct_sets_up_the_handler() {
        let (mut e, allocator) = error_engine();
        let this = e.mem.alloc(0x40);
        for offset in (0..0x28).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(
            e.call(0x00d2_b640, &args![this, 0x1111u32, 0x2222u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_e5d4);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x20), 0x1111);
        assert_eq!(e.mem.u32(this + 0x24), 0x2222);
        assert_eq!(logged(&e, MAP_RESERVE), vec![vec![this + 8, allocator, 0]]);
    }

    #[test]
    fn destruct_gives_the_section_stack_and_the_map_back() {
        let (mut e, allocator) = error_engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x18, 3);
        e.mem.set_u32(this + 0x1c, 0x4000_0010);
        e.call(0x00d2_b6b0, &args![this]);
        assert_eq!(
            calls_to(&e, &[PRINTED_FREE_DOUBLE, MAP_DEALLOCATE, MAP_DESTRUCT]),
            vec![
                (PRINTED_FREE_DOUBLE, vec![allocator, 0x5000, 0x40]),
                (MAP_DEALLOCATE, vec![this + 8, allocator]),
                (MAP_DESTRUCT, vec![this + 8]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn destruct_keeps_storage_it_does_not_own() {
        let (mut e, _) = error_engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x1c, 0x8000_0010);
        e.call(0x00d2_b6b0, &args![this]);
        assert!(logged(&e, PRINTED_FREE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    /// Records the callback calls (buffer text length marker, user) in order.
    fn record_callback(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u8, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CALLBACK_DOUBLE, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u8(a[0] + 0x7ff), a[1]));
            Ret::default()
        });
        seen
    }

    /// `snprintf` double that fills the whole buffer with `b'x'`.
    fn fill_buffer(e: &mut Engine) {
        e.register(FORMAT_INTO, |e, a| {
            for offset in 0..a[1] {
                e.mem.set_u8(a[0] + offset, b'x');
            }
            Ret::default()
        });
    }

    #[test]
    fn show_message_without_id_and_file_prints_just_the_text() {
        let (mut e, _) = error_engine();
        fill_buffer(&mut e);
        let seen = record_callback(&mut e);
        let this = handler(&mut e);
        e.call(
            0x00d2_b3e0,
            &args![this, 0x11u32, 0u32, 0x33u32, 0u32, 0x55u32, 0u32],
        );
        let format = logged(&e, FORMAT_INTO);
        assert_eq!(format.len(), 1);
        let buffer = format[0][0];
        assert_eq!(format[0][1..], [0x800, FORMAT_TEXT, 0x33]);
        // The last byte of the buffer is cleared before the callback runs.
        assert_eq!(*seen.borrow(), vec![(buffer, 0, 0x7777)]);
        assert!(logged(&e, GET_STACK_TRACE).is_empty());
    }

    #[test]
    fn show_message_with_an_id_or_a_file_prints_the_location() {
        for (id, file) in [(0x22u32, 0u32), (0, 0x44), (0x22, 0x44)] {
            let (mut e, _) = error_engine();
            record_callback(&mut e);
            let this = handler(&mut e);
            e.call(
                0x00d2_b3e0,
                &args![this, 0x11u32, id, 0x33u32, file, 0x55u32, 0u32],
            );
            let format = logged(&e, FORMAT_INTO);
            assert_eq!(
                format[0][1..],
                [0x800, FORMAT_LOCATED, file, 0x55, id, 0x11, 0x33]
            );
        }
    }

    #[test]
    fn show_message_makes_a_missing_tracer_and_prints_the_frames_after_the_first_two() {
        let (mut e, _) = error_engine();
        let seen = record_callback(&mut e);
        e.register(STACK_TRACER_CREATE, |_, _| 0x6000u32.into_ret());
        e.register(GET_STACK_TRACE, |_, _| 5u32.into_ret());
        let this = handler(&mut e);
        e.call(
            0x00d2_b3e0,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 0x100u32],
        );
        // 0x100 has a clear low byte: no trace.
        assert!(logged(&e, GET_STACK_TRACE).is_empty());
        e.call(
            0x00d2_b3e0,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 0x101u32],
        );
        assert_eq!(e.global::<u32>(STACK_TRACER), 0x6000);
        assert!(logged(&e, REMOVE_REFERENCE).is_empty());
        let trace = logged(&e, GET_STACK_TRACE);
        assert_eq!(trace.len(), 1);
        assert_eq!(trace[0][0], 0x6000);
        let frames = trace[0][1];
        assert_eq!(trace[0][2], 0x14);
        assert_eq!(
            logged(&e, PRINT_STACK_TRACE),
            vec![vec![0x6000, frames + 8, 3, CALLBACK_DOUBLE, 0x7777]]
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 3);
        assert_eq!((seen[2].0, seen[2].2), (TRACE_HEADER, 0x7777));
    }

    #[test]
    fn show_message_releases_a_tracer_that_appeared_while_making_one() {
        let (mut e, _) = error_engine();
        record_callback(&mut e);
        e.register(STACK_TRACER_CREATE, |e, _| {
            e.set_global(STACK_TRACER, 0x6100u32);
            0x6000u32.into_ret()
        });
        let this = handler(&mut e);
        e.call(
            0x00d2_b3e0,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32],
        );
        assert_eq!(logged(&e, REMOVE_REFERENCE), vec![vec![0x6100]]);
        assert_eq!(e.global::<u32>(STACK_TRACER), 0x6000);
    }

    #[test]
    fn show_message_keeps_an_existing_tracer_and_skips_short_traces() {
        for (frames, printed) in [(2u32, false), (3, true)] {
            let (mut e, _) = error_engine();
            let seen = record_callback(&mut e);
            e.set_global(STACK_TRACER, 0x6200u32);
            e.register_double(GET_STACK_TRACE, move |_, _| frames.into_ret());
            let this = handler(&mut e);
            e.call(
                0x00d2_b3e0,
                &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32],
            );
            assert!(logged(&e, STACK_TRACER_CREATE).is_empty());
            assert_eq!(e.global::<u32>(STACK_TRACER), 0x6200);
            assert_eq!(logged(&e, PRINT_STACK_TRACE).len(), usize::from(printed));
            assert_eq!(seen.borrow().len(), 1 + usize::from(printed));
        }
    }

    /// `message` with every outside callee doubled and the handler's `isEnabled` answering
    /// "enabled" unless the map double says the id is off. Answers `(result, show calls)`.
    fn run_message(
        severity: u32,
        id: u32,
        sections: &[u32],
        off: &[u32],
    ) -> (u32, Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let (mut e, _) = error_engine();
        fill_buffer(&mut e);
        record_callback(&mut e);
        e.set_global(STACK_TRACER, 0x6200u32);
        e.register(GET_STACK_TRACE, |_, _| 4u32.into_ret());
        let off = off.to_vec();
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
            u32::from(off.contains(&a[1])).into_ret()
        });
        let this = handler(&mut e);
        let data = e.mem.alloc(16);
        for (i, section) in sections.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *section);
        }
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, sections.len() as u32);
        let result = e
            .call(
                0x00d2_b4f0,
                &args![this, severity, id, 0x33u32, 0x44u32, 0x55u32],
            )
            .u32();
        (
            result,
            logged(&e, FORMAT_INTO),
            logged(&e, PRINT_STACK_TRACE),
        )
    }

    #[test]
    fn message_labels_by_severity_and_answers_one_for_assert_and_error() {
        for (severity, label, answer, traced) in [
            (0u32, LABEL_REPORT, 0u32, false),
            (1, LABEL_WARNING, 0, false),
            (2, LABEL_ASSERT, 1, true),
            (3, LABEL_ERROR, 1, true),
            (4, LABEL_NONE, 0, false),
            (0xffff_ffff, LABEL_NONE, 0, false),
        ] {
            let (result, format, printed) = run_message(severity, 0x22, &[], &[]);
            assert_eq!(result, answer, "severity {severity:#x}");
            assert_eq!(format.len(), 1);
            assert_eq!(
                format[0][1..],
                [0x800, FORMAT_LOCATED, 0x44, 0x55, 0x22, label, 0x33]
            );
            assert_eq!(printed.len(), usize::from(traced), "severity {severity:#x}");
        }
    }

    #[test]
    fn message_does_nothing_for_a_disabled_id() {
        let (result, format, printed) = run_message(3, 0x22, &[], &[0x22]);
        assert_eq!(result, 0);
        assert!(format.is_empty());
        assert!(printed.is_empty());
    }

    #[test]
    fn message_replaces_an_id_of_minus_one_by_the_top_section() {
        let (result, format, _) = run_message(2, 0xffff_ffff, &[7, 9], &[]);
        assert_eq!(result, 1);
        assert_eq!(
            format[0][1..],
            [0x800, FORMAT_LOCATED, 0x44, 0x55, 9, LABEL_ASSERT, 0x33]
        );
        // The replacement id is what `isEnabled` is asked about.
        let (result, format, _) = run_message(2, 0xffff_ffff, &[7, 9], &[9]);
        assert_eq!((result, format.len()), (0, 0));
        // Without sections the -1 stays.
        let (_, format, _) = run_message(0, 0xffff_ffff, &[], &[]);
        assert_eq!(
            format[0][1..],
            [
                0x800,
                FORMAT_LOCATED,
                0x44,
                0x55,
                0xffff_ffff,
                LABEL_REPORT,
                0x33
            ]
        );
    }
}
