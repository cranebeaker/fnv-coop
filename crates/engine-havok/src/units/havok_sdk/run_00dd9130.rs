//! `Havok SDK/run_00dd9130` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dd9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_91f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dd9210(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_91f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The vtable of `hkIArchive` (RTTI `.?AVhkIArchive@@`) and of `hkBaseObject` (`0102e388`).
const IARCHIVE_VTABLE: u32 = 0x010d_7c80;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The reference-count functions of `hkReferencedObject`, each `thiscall` on the object with no
/// stack words: add (`00c90510`) and remove (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// A global object (`01268364`) whose virtual function at `+0xc` takes a name and answers a
/// stream reader.
const READER_FACTORY: u32 = 0x0126_8364;
/// `hkMemoryStreamReader::hkMemoryStreamReader` (Xbox PDB, `00e05b40`, `thiscall`, `RET 0xc`:
/// the buffer, its size, a mode word); answers `this`.
const MEMORY_STREAM_READER: u32 = 0x00e0_5b40;
/// The import slots of `EnterCriticalSection` (`00fdf05c`) and `LeaveCriticalSection`
/// (`00fdf100`), and `hkString::strCmp` (Xbox PDB, `00cb0fa0`, cdecl: two texts).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const STRING_COMPARE: u32 = 0x00cb_0fa0;

// Translated from 00dd9130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `reader`, `flag`): a constructor of
/// `hkIArchive` (the vtable's RTTI name) over an existing stream reader. Sets the reference count
/// (`u16` at `+6`) to 1, the vtable, the reader at `+8` and the low byte of `flag` at `+0xc`, and
/// adds a reference to the reader (`00c90510`). Answers `this`.
pub fn fn_00dd9130(e: &mut Engine, this: u32, reader: u32, flag: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, IARCHIVE_VTABLE);
    e.mem.set_u32(this + 8, reader);
    e.mem.set_u8(this + 0xc, flag as u8);
    e.call(ADD_REFERENCE, &args![reader]);
    this
}

// Translated from 00dd9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `name`, `flag`): the `hkIArchive`
/// constructor that opens a reader by name. Sets the reference count to 1, the vtable and the
/// low byte of `flag` at `+0xc`, then asks the global object at `01268364` (virtual function at
/// `+0xc`) for a reader for `name` and stores it at `+8`. Answers `this`.
pub fn fn_00dd9160(e: &mut Engine, this: u32, name: u32, flag: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, IARCHIVE_VTABLE);
    e.mem.set_u8(this + 0xc, flag as u8);
    let factory: u32 = e.global(READER_FACTORY);
    let reader = e.vcall(factory, 0xc, &args![name]).u32();
    e.mem.set_u32(this + 8, reader);
    this
}

// Translated from 00dd91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `this`, `buffer`, `size`, `flag`): the
/// `hkIArchive` constructor over a block of memory. Sets the reference count to 1, the vtable
/// and the low byte of `flag` at `+0xc`, takes 0x1c bytes from the memory router's allocator
/// (word at `+0x10`, virtual function `+4`), stores 0x1c in the `u16` at its `+4`, constructs an
/// `hkMemoryStreamReader` in it over `(buffer, size, 2)` (`00e05b40`) and stores what that
/// answers at `+8`. Answers `this`.
pub fn fn_00dd91a0(e: &mut Engine, this: u32, buffer: u32, size: u32, flag: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, IARCHIVE_VTABLE);
    e.mem.set_u8(this + 0xc, flag as u8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    let reader = e
        .call(MEMORY_STREAM_READER, &args![block, buffer, size, 2u32])
        .u32();
    e.mem.set_u32(this + 8, reader);
    this
}

// Translated from 00dd91f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`): the `hkIArchive` destructor body. Sets the
/// vtable, removes the reference to the reader when there is one (`00c905b0`) and leaves the
/// `hkBaseObject` vtable.
pub fn fn_00dd91f0(e: &mut Engine, this: u32) {
    let reader = e.mem.u32(this + 8);
    e.mem.set_u32(this, IARCHIVE_VTABLE);
    if reader != 0 {
        e.call(REMOVE_REFERENCE, &args![reader]);
    }
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
}

// The four functions below work on an object with a table of 12-byte entries at the pointer at
// `+0xc` (count at `+0x10`) and the critical section at the pointer at `+0x18`. An entry holds a
// text pointer in its first word (bit 0 is a flag the lookups mask off), a function in its second
// and a third word they do not touch. The engine map has no names for them; they sit beside
// `hkProcessFactory::registerProcess` (`00dd9380`).

// Translated from 00dd9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `index`). Inside the critical section
/// answers the first word of entry `index` with bit 0 cleared (the entry's text pointer).
pub fn fn_00dd9240(e: &mut Engine, this: u32, index: u32) -> u32 {
    let section = e.mem.u32(this + 0x18);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let table = e.mem.u32(this + 0xc);
    let word = e.mem.u32(table.wrapping_add(index.wrapping_mul(0xc)));
    let section = e.mem.u32(this + 0x18);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    word & 0xffff_fffe
}

// Translated from 00dd9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `name`). Inside the critical section
/// looks for the first entry whose text (first word with bit 0 cleared) equals `name`
/// (`hkString::strCmp`, `00cb0fa0`, answering 0); an entry with no text matches only a null
/// `name`, and a null `name` matches no entry that has text. Answers its index, or -1.
pub fn fn_00dd9270(e: &mut Engine, this: u32, name: u32) -> i32 {
    let section = e.mem.u32(this + 0x18);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let mut index = 0i32;
    if e.mem.i32(this + 0x10) > 0 {
        let mut offset = 0u32;
        loop {
            let table = e.mem.u32(this + 0xc);
            let text = e.mem.u32(table.wrapping_add(offset)) & 0xffff_fffe;
            let found = if text == 0 {
                name == 0
            } else if name == 0 {
                false
            } else {
                e.call(STRING_COMPARE, &args![text, name]).i32() == 0
            };
            if found {
                let section = e.mem.u32(this + 0x18);
                e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                return index;
            }
            index += 1;
            offset = offset.wrapping_add(0xc);
            if index >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
    let section = e.mem.u32(this + 0x18);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    -1
}

// Translated from 00dd92f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `index`, `argument`). Inside the
/// critical section calls the function in the second word of entry `index` (cdecl, one word:
/// `argument`) and answers what it answers.
pub fn fn_00dd92f0(e: &mut Engine, this: u32, index: u32, argument: u32) -> u32 {
    let section = e.mem.u32(this + 0x18);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let table = e.mem.u32(this + 0xc);
    let function = e.mem.u32(table.wrapping_add(index.wrapping_mul(0xc)) + 4);
    let result = e.call(function, &args![argument]).u32();
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    result
}

// Translated from 00dd9330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `name`, `argument`). Inside the
/// critical section finds the entry for `name` (`00dd9270`) and answers what `00dd92f0` makes of
/// it with `argument`; 0 when there is no such entry.
pub fn fn_00dd9330(e: &mut Engine, this: u32, name: u32, argument: u32) -> u32 {
    let section = e.mem.u32(this + 0x18);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let index = fn_00dd9270(e, this, name);
    if index < 0 {
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return 0;
    }
    let result = fn_00dd92f0(e, this, index as u32, argument);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_9210, deleting_destructor_00dd9210(Ptr, u8) -> Ptr),
        entry!(0x00dd9130, fn_00dd9130(u32, u32, u32) -> u32),
        entry!(0x00dd9160, fn_00dd9160(u32, u32, u32) -> u32),
        entry!(0x00dd91a0, fn_00dd91a0(u32, u32, u32, u32) -> u32),
        entry!(0x00dd91f0, fn_00dd91f0(u32)),
        entry!(0x00dd9240, fn_00dd9240(u32, u32) -> u32),
        entry!(0x00dd9270, fn_00dd9270(u32, u32) -> i32),
        entry!(0x00dd92f0, fn_00dd92f0(u32, u32, u32) -> u32),
        entry!(0x00dd9330, fn_00dd9330(u32, u32, u32) -> u32),
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
    fn deleting_destructor_00dd9210_frees_only_when_bit_zero_is_set() {
        check(0x00dd_9210, 0x00dd_91f0);
    }
}

#[cfg(test)]
mod a07_archive_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    fn logging(e: &mut Engine, name: &'static str, address: u32, answer: u32) -> Log {
        let log: Log = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((name, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn the_archive_over_a_reader_references_it() {
        let mut e = Engine::new();
        let log = logging(&mut e, "add", ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00dd_9130, &args![this, 0x4242u32, 0x1ffu32]).u32(),
            this
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), IARCHIVE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x4242);
        assert_eq!(e.mem.u8(this + 0xc), 0xff);
        assert_eq!(*log.borrow(), vec![("add", vec![0x4242])]);
    }

    #[test]
    fn the_archive_by_name_asks_the_global_for_a_reader() {
        let mut e = Engine::new();
        e.map(READER_FACTORY, 4);
        let factory = e.mem.alloc(8);
        e.put_vtable(0x0078_0000, &[0, 0, 0, 0x0078_0100]);
        e.mem.set_u32(factory, 0x0078_0000);
        e.set_global(READER_FACTORY, factory);
        e.register_double(0x0078_0100, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00dd_9160, &args![this, 0x5000u32, 1u32]).u32(),
            this
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), IARCHIVE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x5001);
        assert_eq!(e.mem.u8(this + 0xc), 1);
    }

    #[test]
    fn the_archive_over_memory_builds_a_memory_stream_reader() {
        let mut e = Engine::new();
        let allocator = e.mem.alloc(8);
        e.put_vtable(0x0078_1000, &[0, 0x0078_1100]);
        e.mem.set_u32(allocator, 0x0078_1000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let block = e.mem.alloc(0x20);
        // The allocation double answers `block`.
        let sizes: Log = Rc::default();
        let seen = Rc::clone(&sizes);
        e.register_double(0x0078_1100, move |_, a| {
            seen.borrow_mut().push(("allocate", a[1..].to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let constructed = logging(&mut e, "reader", MEMORY_STREAM_READER, 0x7777);
        let this = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00dd_91a0, &args![this, 0x9000u32, 0x40u32, 0u32])
                .u32(),
            this
        );
        assert_eq!(*sizes.borrow(), vec![("allocate", vec![0x1c])]);
        assert_eq!(e.mem.u16(block + 4), 0x1c);
        assert_eq!(
            *constructed.borrow(),
            vec![("reader", vec![block, 0x9000, 0x40, 2])]
        );
        assert_eq!(e.mem.u32(this + 8), 0x7777);
        assert_eq!(e.mem.u32(this), IARCHIVE_VTABLE);
        assert_eq!(e.mem.u8(this + 0xc), 0);
    }

    #[test]
    fn the_archive_destructor_releases_its_reader() {
        let mut e = Engine::new();
        let log = logging(&mut e, "remove", REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x4242);
        e.call(0x00dd_91f0, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00dd_91f0, &args![this]);
        assert_eq!(*log.borrow(), vec![("remove", vec![0x4242])]);
    }

    /// An object with three entries: texts "alpha" (bit 0 set), "beta" and no text, whose
    /// functions answer `argument + 100`, `+ 200` and `+ 300`; the critical section calls are
    /// logged.
    fn table(e: &mut Engine) -> (u32, Log) {
        let this = e.mem.alloc(0x40);
        let entries = e.mem.alloc(0x30);
        let alpha = e.mem.alloc(8);
        e.mem.set_cstr(alpha, b"alpha");
        let beta = e.mem.alloc(8);
        e.mem.set_cstr(beta, b"beta");
        e.mem.set_u32(entries, alpha | 1);
        e.mem.set_u32(entries + 0xc, beta);
        e.mem.set_u32(entries + 0x18, 0);
        for (i, f) in [0x0079_0100u32, 0x0079_0200, 0x0079_0300]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(entries + 0xc * i as u32 + 4, f);
            let add = 100 * (i as u32 + 1);
            e.register_double(f, move |_, a| Ret {
                eax: a[0] + add,
                ..Ret::default()
            });
        }
        let section = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, entries);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x18, section);
        let log: Log = Rc::default();
        for (name, slot) in [
            ("enter", ENTER_CRITICAL_SECTION),
            ("leave", LEAVE_CRITICAL_SECTION),
        ] {
            let seen = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                seen.borrow_mut().push((name, vec![a[0]]));
                Ret::default()
            });
        }
        e.register(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: u32::from(x != y),
                ..Ret::default()
            }
        });
        (this, log)
    }

    #[test]
    fn the_text_of_an_entry_is_read_inside_the_critical_section_without_its_flag() {
        let mut e = Engine::new();
        let (this, log) = table(&mut e);
        let section = e.mem.u32(this + 0x18);
        let entries = e.mem.u32(this + 0xc);
        let alpha = e.mem.u32(entries) & !1;
        assert_eq!(e.call(0x00dd_9240, &args![this, 0u32]).u32(), alpha);
        assert_eq!(e.call(0x00dd_9240, &args![this, 2u32]).u32(), 0);
        assert_eq!(
            log.borrow()[..2],
            [("enter", vec![section]), ("leave", vec![section])]
        );
    }

    #[test]
    fn entries_are_found_by_text_with_null_matching_the_empty_one() {
        let mut e = Engine::new();
        let (this, _) = table(&mut e);
        let find = |e: &mut Engine, text: &[u8]| {
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, text);
            e.call(0x00dd_9270, &args![this, name]).i32()
        };
        assert_eq!(find(&mut e, b"alpha"), 0);
        assert_eq!(find(&mut e, b"beta"), 1);
        assert_eq!(find(&mut e, b"gamma"), -1);
        assert_eq!(e.call(0x00dd_9270, &args![this, 0u32]).i32(), 2);
        // An empty table finds nothing.
        e.mem.set_u32(this + 0x10, 0);
        assert_eq!(e.call(0x00dd_9270, &args![this, 0u32]).i32(), -1);
    }

    #[test]
    fn the_function_of_an_entry_is_called_with_the_argument() {
        let mut e = Engine::new();
        let (this, log) = table(&mut e);
        assert_eq!(e.call(0x00dd_92f0, &args![this, 1u32, 5u32]).u32(), 205);
        assert_eq!(log.borrow().len(), 2);
        log.borrow_mut().clear();
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"beta");
        assert_eq!(e.call(0x00dd_9330, &args![this, name, 7u32]).u32(), 207);
        // The lookup and the call are nested inside the outer critical section.
        let names: Vec<&str> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            names,
            vec!["enter", "enter", "leave", "enter", "leave", "leave"]
        );
        e.mem.set_cstr(name, b"nothing");
        assert_eq!(e.call(0x00dd_9330, &args![this, name, 7u32]).u32(), 0);
    }
}
