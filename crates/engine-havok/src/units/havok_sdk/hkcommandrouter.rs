//! `Havok SDK/hkcommandrouter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkIArchive::readArrayGeneric` (`00dd8fe0`, `this` = the archive, `(destination, element
/// size, count)`).
const ARCHIVE_READ_ARRAY_GENERIC: u32 = 0x00dd_8fe0;
/// `hkDisplaySerializeIStream::readQuadVector4` (`00ddac60`, `this` = the archive,
/// `(destination)`).
const READ_QUAD_VECTOR4: u32 = 0x00dd_ac60;
/// `hkIArchive::isOk` (`00dd90e0`, `this` = the archive, `(out byte)`, answers the out pointer).
const ARCHIVE_IS_OK: u32 = 0x00dd_90e0;
/// `hkMapBase<unsigned long,unsigned long,...>::get` (`00cb1980`, `this` = the map,
/// `(key, out value)`, answers 0 when the key is there).
const MAP_GET: u32 = 0x00cb_1980;
/// The memory router (`00c85750`): its field `+0xC` is the allocator the message array uses.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The formatting function (`00d61d00`, cdecl `(array, format, value)`) that writes the text
/// into a byte array.
const FORMAT_INTO_ARRAY: u32 = 0x00d6_1d00;
/// The exe's format string `VDB: Found a command (%x) with no handler. Could corrupt the
/// stream.`.
const NO_HANDLER_FORMAT: u32 = 0x010d_7cb0;

// Translated from 00dd9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCommandRouter::consumeCommands` (Xbox PDB; `this` = the router, then `(out, archive)`,
/// `RET 8`). Reads command bytes from the archive (`hkIArchive::readArrayGeneric`, one byte at
/// a time into a four-byte local that keeps its other three bytes between rounds; they are
/// not initialised in the exe and the handlers read only the low byte, so the local starts at
/// zero here) until the byte is 0xF0 or the archive is not ok, then stores 1 at the byte
/// `out`. For a command other than 0xF0 it looks the byte up in the router's handler map (the
/// map is at `this + 8`, `00cb1980`); a found, non-null handler gets its virtual method at
/// slot `0xC` called with the whole command word. Otherwise: commands 0xB0 and 0xB1 first read
/// a quad vector (`00ddac60`) and 0xB0 also eight bytes; then a message array with 128 bytes
/// of local storage (capacity word 0x80000080) is made, `VDB: Found a command (%x) with no
/// handler. Could corrupt the stream.` is formatted into it (`00d61d00`, with the command
/// byte) and the array's storage is given back to the router's allocator (slot `0x10`, at
/// `+0xC` of the router) unless the capacity word marks it as local (bit 31). Its capacity
/// below 1 would grow it through `hkArrayUtil::_reserve` (`00c908c0`); the capacity is the
/// constant 0x80 here, so that branch cannot run.
pub fn hk_command_router_consume_commands(e: &mut Engine, this: Ptr, out: Ptr, archive: Ptr) {
    e.with_stack(0xe0, |e, frame| {
        let f = frame.addr();
        // The frame, as the exe lays it out: the isOk out byte (+0x1B), the handler out word
        // (+0x1C), the router (+0x20), the command word (+0x24), eight bytes (+0x28), a quad
        // vector (+0x30), the message array (+0x40: data, size, capacity word) and its local
        // storage (+0x4C).
        e.mem.set_u32(f + 0x20, this.addr());
        loop {
            e.call(
                ARCHIVE_READ_ARRAY_GENERIC,
                &args![archive, f + 0x24, 1u32, 1u32],
            );
            let command = e.mem.u32(f + 0x24);
            let command_byte = command as u8;
            if command_byte != 0xf0 {
                let key = u32::from(command_byte);
                let map = e.mem.u32(f + 0x20).wrapping_add(8);
                let missing = e.call(MAP_GET, &args![map, key, f + 0x1c]).u32();
                let handler = e.mem.u32(f + 0x1c);
                if missing == 0 && handler != 0 {
                    e.vcall(handler, 0xc, &args![command]);
                } else {
                    if command_byte == 0xb0 {
                        e.call(READ_QUAD_VECTOR4, &args![archive, f + 0x30]);
                        e.call(
                            ARCHIVE_READ_ARRAY_GENERIC,
                            &args![archive, f + 0x28, 8u32, 1u32],
                        );
                    } else if command_byte == 0xb1 {
                        e.call(READ_QUAD_VECTOR4, &args![archive, f + 0x30]);
                    }
                    report_missing_handler(e, f, key);
                }
            }
            let ok = e.call(ARCHIVE_IS_OK, &args![archive, f + 0x1b]).u32();
            if e.mem.u8(ok) == 0 || command_byte == 0xf0 {
                break;
            }
        }
        e.mem.set_u8(out.addr(), 1);
    });
}

/// The message half of the unhandled-command path of `consumeCommands`, on the router's
/// frame `f`: builds the array at `f + 0x40`, formats the text into it and frees its storage.
fn report_missing_handler(e: &mut Engine, f: u32, key: u32) {
    e.mem.set_u32(f + 0x40, f + 0x4c);
    e.mem.set_u32(f + 0x44, 0);
    e.mem.set_u32(f + 0x48, 0x8000_0080);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let mut capacity = (e.mem.u32(f + 0x48) & 0x3fff_ffff) as i32;
    if capacity < 1 {
        capacity = capacity.wrapping_add(capacity);
        if capacity <= 1 {
            capacity = 1;
        }
        e.call(ARRAY_RESERVE, &args![allocator, f + 0x40, capacity, 1u32]);
    }
    let data = e.mem.u32(f + 0x40);
    e.mem.set_u32(f + 0x44, 1);
    e.mem.set_u8(data, 0);
    let data = e.mem.u32(f + 0x40);
    e.mem.set_u8(data, 0);
    e.call(FORMAT_INTO_ARRAY, &args![f + 0x40, NO_HANDLER_FORMAT, key]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    e.mem.set_u32(f + 0x44, 0);
    let capacity_word = e.mem.u32(f + 0x48);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(f + 0x40);
        e.vcall(allocator, 0x10, &args![data, capacity_word & 0x3fff_ffff]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dd9660,
        hk_command_router_consume_commands(Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    const HANDLER_SLOT_DOUBLE: u32 = 0x0900_0031;
    const FREE_SLOT_DOUBLE: u32 = 0x0900_0032;
    const HANDLED_COMMAND: u32 = 0x41;

    /// Everything the router touches outside this file, doubled: the archive hands out the
    /// scripted bytes (`bytes`) one at a time and is ok while `ok` is set, the handler map
    /// knows only command 0x41 (its handler's slot `0xC` is `HANDLER_SLOT_DOUBLE`), and the
    /// router's allocator (at `+0xC`) frees through `FREE_SLOT_DOUBLE`.
    struct Fixture {
        e: Engine,
        this: u32,
        handler: u32,
        archive: u32,
        out: u32,
        ok: Rc<Cell<u8>>,
    }

    fn fixture(bytes: &[u8]) -> Fixture {
        let mut e = Engine::new();
        let queue: Rc<RefCell<VecDeque<u8>>> =
            Rc::new(RefCell::new(bytes.iter().copied().collect()));
        e.register_double(ARCHIVE_READ_ARRAY_GENERIC, move |e, a| {
            if a[2] == 1 {
                let next = queue
                    .borrow_mut()
                    .pop_front()
                    .expect("script ran out of bytes");
                e.mem.set_u8(a[1], next);
            }
            Ret::default()
        });
        e.register(READ_QUAD_VECTOR4, |_, _| Ret::default());
        let ok = Rc::new(Cell::new(1u8));
        let ok_flag = Rc::clone(&ok);
        e.register_double(ARCHIVE_IS_OK, move |e, a| {
            e.mem.set_u8(a[1], ok_flag.get());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let handler_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(handler_vtable + 0xc, HANDLER_SLOT_DOUBLE);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(handler, handler_vtable);
        e.register_double(MAP_GET, move |e, a| {
            if a[1] == HANDLED_COMMAND {
                e.mem.set_u32(a[2], handler);
                Ret::default()
            } else {
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            }
        });
        e.register(HANDLER_SLOT_DOUBLE, |_, _| Ret::default());
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator_vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, allocator_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(FREE_SLOT_DOUBLE, |_, _| Ret::default());
        e.register(FORMAT_INTO_ARRAY, |_, _| Ret::default());
        let archive = e.mem.alloc(0x10);
        let out = e.mem.alloc(8);
        e.call_log = Some(Vec::new());
        Fixture {
            e,
            this,
            handler,
            archive,
            out,
            ok,
        }
    }

    fn run(f: &mut Fixture) {
        f.e.call(0x00dd9660, &args![f.this, f.out, f.archive]);
    }

    fn calls(f: &Fixture, addr: u32) -> Vec<Vec<u32>> {
        f.e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn handled_command_goes_to_its_handler_and_0xf0_ends_the_loop() {
        let mut f = fixture(&[HANDLED_COMMAND as u8, 0xf0]);
        run(&mut f);
        assert_eq!(f.e.mem.u8(f.out), 1);
        assert_eq!(
            calls(&f, MAP_GET),
            vec![vec![f.this + 8, HANDLED_COMMAND, calls(&f, MAP_GET)[0][2]]]
        );
        assert_eq!(
            calls(&f, HANDLER_SLOT_DOUBLE),
            vec![vec![f.handler, HANDLED_COMMAND]]
        );
        assert!(calls(&f, FORMAT_INTO_ARRAY).is_empty());
        assert_eq!(calls(&f, ARCHIVE_READ_ARRAY_GENERIC).len(), 2);
        assert_eq!(calls(&f, ARCHIVE_IS_OK).len(), 2);
    }

    #[test]
    fn a_first_byte_of_0xf0_looks_nothing_up() {
        let mut f = fixture(&[0xf0]);
        run(&mut f);
        assert_eq!(f.e.mem.u8(f.out), 1);
        assert!(calls(&f, MAP_GET).is_empty());
        assert_eq!(calls(&f, ARCHIVE_IS_OK).len(), 1);
    }

    #[test]
    fn the_loop_also_ends_when_the_archive_is_not_ok() {
        let mut f = fixture(&[HANDLED_COMMAND as u8, HANDLED_COMMAND as u8]);
        f.ok.set(0);
        run(&mut f);
        assert_eq!(f.e.mem.u8(f.out), 1);
        assert_eq!(calls(&f, HANDLER_SLOT_DOUBLE).len(), 1);
        assert_eq!(calls(&f, ARCHIVE_READ_ARRAY_GENERIC).len(), 1);
    }

    #[test]
    fn unhandled_command_formats_the_message_into_local_storage() {
        let mut f = fixture(&[0x99, 0xf0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        f.e.register_double(FORMAT_INTO_ARRAY, move |e, a| {
            let data = e.mem.u32(a[0]);
            record.borrow_mut().push((
                data - a[0],
                e.mem.u32(a[0] + 4),
                e.mem.u32(a[0] + 8),
                e.mem.u8(data),
            ));
            Ret::default()
        });
        run(&mut f);
        // The array's storage is its own 128-byte local one (data 0xC after the array), holding
        // one byte of 0, its capacity word marks the storage as local, nothing is freed.
        assert_eq!(*seen.borrow(), vec![(0xc, 1, 0x8000_0080, 0)]);
        let formats = calls(&f, FORMAT_INTO_ARRAY);
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0][1..], [NO_HANDLER_FORMAT, 0x99]);
        assert!(calls(&f, FREE_SLOT_DOUBLE).is_empty());
        assert!(calls(&f, READ_QUAD_VECTOR4).is_empty());
    }

    #[test]
    fn a_message_array_that_grew_is_given_back_to_the_allocator() {
        let mut f = fixture(&[0x99, 0xf0]);
        let heap = f.e.mem.alloc(0x40);
        f.e.register_double(FORMAT_INTO_ARRAY, move |e, a| {
            e.mem.set_u32(a[0], heap);
            e.mem.set_u32(a[0] + 8, 0x40);
            Ret::default()
        });
        run(&mut f);
        let frees = calls(&f, FREE_SLOT_DOUBLE);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [heap, 0x40]);
        // The size word is reset before the storage goes back.
        let array = calls(&f, FORMAT_INTO_ARRAY)[0][0];
        assert_eq!(f.e.mem.u32(array + 4), 0);
    }

    #[test]
    fn commands_0xb0_and_0xb1_read_their_vector_payload_first() {
        let mut f = fixture(&[0xb0, 0xb1, 0xf0]);
        run(&mut f);
        let order: Vec<(u32, u32)> =
            f.e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, w)| {
                    *a == READ_QUAD_VECTOR4 || (*a == ARCHIVE_READ_ARRAY_GENERIC && w[2] == 8)
                })
                .map(|(a, w)| (*a, w[1]))
                .collect();
        let quad = calls(&f, READ_QUAD_VECTOR4)[0][1];
        let eight = calls(&f, ARCHIVE_READ_ARRAY_GENERIC)
            .iter()
            .find(|w| w[2] == 8)
            .map(|w| w[1])
            .unwrap();
        assert_eq!(
            order,
            vec![
                (READ_QUAD_VECTOR4, quad),
                (ARCHIVE_READ_ARRAY_GENERIC, eight),
                (READ_QUAD_VECTOR4, quad),
            ]
        );
        let eight_reads = calls(&f, ARCHIVE_READ_ARRAY_GENERIC)
            .iter()
            .filter(|w| w[2] == 8)
            .count();
        assert_eq!(eight_reads, 1);
        assert_eq!(calls(&f, FORMAT_INTO_ARRAY).len(), 2);
    }
}
