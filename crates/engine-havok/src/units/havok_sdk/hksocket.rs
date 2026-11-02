//! `Havok SDK/hksocket.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkSocket` (RTTI vtable `010ce658`) is an abstract socket whose stream adapters sit inside the
//! object: the reader adapter (`hkSocket::ReaderAdapter`, vtable `010ce5fc`) at `+8` and the
//! writer adapter (`hkSocket::WriterAdapter`, vtable `010ce630`) at `+0x14`; each adapter's `+8`
//! points back at the socket. The socket's own slots `0xc`, `0x14` and `0x18` (pure in
//! `hkSocket`) are the `isOk`, receive and send the adapters forward to.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the socket objects use.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkBsdSocket`'s constructor (`00da30a0`, `thiscall`, one word: the descriptor, or -1).
const BSD_SOCKET_CONSTRUCTOR: u32 = 0x00da_30a0;
/// The byte that says the network layer has been started (`01268a44`) and the function pointer
/// that starts it (`011b3de8`, `hkBsdNetworkInit`, no arguments).
const NETWORK_STARTED: u32 = 0x0126_8a44;
const NETWORK_INIT: u32 = 0x011b_3de8;
/// The vtables the constructor stores: the socket's, the reader adapter's, the writer adapter's.
const SOCKET_VTABLE: u32 = 0x010c_e658;
const READER_ADAPTER_VTABLE: u32 = 0x010c_e5fc;
const WRITER_ADAPTER_VTABLE: u32 = 0x010c_e630;
/// The `hkBaseObject` vtable a destroyed socket is left with (`0102e388`).
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

/// Calls `slot` of the socket the adapter at `this` forwards to (`this + 8`) in a loop until
/// `count` bytes have gone through or a call moves none, as both adapters do: each call is given
/// the rest of `buffer` and how many bytes remain; the byte counts add up. Answers the total when
/// a call moves 0 bytes, else `count`; a `count` of 0 or less answers `count` without a call.
fn transfer_all(e: &mut Engine, this: u32, buffer: u32, count: i32, slot: u32) -> i32 {
    let mut total: i32 = 0;
    if count > 0 {
        loop {
            let socket = e.mem.u32(this + 8);
            let moved = e
                .vcall(
                    socket,
                    slot,
                    &args![buffer.wrapping_add(total as u32), count.wrapping_sub(total)],
                )
                .i32();
            total = total.wrapping_add(moved);
            if moved == 0 {
                return total;
            }
            if total >= count {
                break;
            }
        }
    }
    count
}

// Translated from 00d2b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket::ReaderAdapter::read` (engine map name), `thiscall`, `RET 8`: reads `count` bytes into
/// `buffer` by calling slot `0x14` of the socket at `this + 8` with the rest of the buffer and the
/// bytes still wanted, until it has them all; returns `count`, or the total so far when a call
/// reads 0 bytes.
pub fn hk_socket_reader_adapter_read(e: &mut Engine, this: u32, buffer: u32, count: i32) -> i32 {
    transfer_all(e, this, buffer, count, 0x14)
}

// Translated from 00d2b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket::ReaderAdapter::isOk` (engine map name), `thiscall`, `RET 4`: asks the socket at
/// `this + 8` (slot `0xc`) to write its `bool` to `out`; returns `out`.
pub fn hk_socket_reader_adapter_is_ok(e: &mut Engine, this: u32, out: u32) -> u32 {
    let socket = e.mem.u32(this + 8);
    e.vcall(socket, 0xc, &args![out]);
    out
}

// Translated from 00d2b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket::WriterAdapter::write` (engine map name), `thiscall`, `RET 8`: the same loop as the
/// reader's with slot `0x18` (send): writes `count` bytes of `buffer`; returns `count`, or the
/// total so far when a call writes 0 bytes.
pub fn hk_socket_writer_adapter_write(e: &mut Engine, this: u32, buffer: u32, count: i32) -> i32 {
    transfer_all(e, this, buffer, count, 0x18)
}

// Translated from 00d2b800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket::WriterAdapter::isOk` (engine map name), `thiscall`, `RET 4`: the same as the
/// reader's: slot `0xc` of the socket at `this + 8` writes its `bool` to `out`; returns `out`.
pub fn hk_socket_writer_adapter_is_ok(e: &mut Engine, this: u32, out: u32) -> u32 {
    let socket = e.mem.u32(this + 8);
    e.vcall(socket, 0xc, &args![out]);
    out
}

// Translated from 00d2b820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. No arguments: allocates a 0x24-byte block through the memory router's allocator
/// (slot 4), stores its size at `+4` and constructs the socket in it (`hkBsdSocket`'s
/// constructor `00da30a0` with the descriptor -1). Returns what that constructor answers.
pub fn fn_00d2b820(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x24u32]).u32();
    e.mem.set_u16(block + 4, 0x24);
    e.call(BSD_SOCKET_CONSTRUCTOR, &args![block, 0xffff_ffffu32])
        .u32()
}

// Translated from 00d2b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket`'s constructor (no map name; the vtables stored are RTTI-confirmed `hkSocket`,
/// `hkSocket::ReaderAdapter` and `hkSocket::WriterAdapter`), `ECX` = `this`, no stack words. Stores the
/// socket vtable at `+0`, the reader adapter's at `+8` and the writer adapter's at `+0x14`, the
/// word 1 at `+6`, `+0xe` and `+0x1a` (16 bits each) and points both adapters back at the
/// socket (`+0x10`, `+0x1c`). The first time any socket is made (byte `01268a44` clear) and
/// when the function pointer at `011b3de8` is set, calls it to start the network layer and sets the
/// byte. Returns `this`.
pub fn hk_socket_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, SOCKET_VTABLE);
    e.mem.set_u32(base + 8, READER_ADAPTER_VTABLE);
    e.mem.set_u16(base + 0xe, 1);
    e.mem.set_u16(base + 0x1a, 1);
    e.mem.set_u32(base + 0x14, WRITER_ADAPTER_VTABLE);
    e.mem.set_u32(base + 0x10, base);
    e.mem.set_u32(base + 0x1c, base);
    if e.mem.u8(NETWORK_STARTED) == 0 {
        let init = e.mem.u32(NETWORK_INIT);
        if init != 0 {
            e.call(init, &args![]);
            e.mem.set_u8(NETWORK_STARTED, 1);
        }
    }
    this
}

// Translated from 00d2b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 9, `+0x24`, of the `hkSocket` vtable `010ce658`, so `ECX` is the socket
/// but is not read). `RET 0xc`: three stack words that are not read. Returns 1.
pub fn fn_00d2b8b0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    1
}

// Translated from 00d2b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSocket::_scalar_deleting_destructor_` (engine map name), `thiscall`, `RET 4`: stores the
/// `hkBaseObject` vtable at `+0x14`, `+8` and `+0` and, when bit 0 of `flags` is set, frees the
/// block through the memory router's allocator (slot 8) with the 16-bit size at `this + 4`.
/// Returns `this`.
pub fn hk_socket_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x14, BASE_OBJECT_VTABLE);
    e.mem.set_u32(base + 8, BASE_OBJECT_VTABLE);
    e.mem.set_u32(base, BASE_OBJECT_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(base + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2b740, hk_socket_reader_adapter_read(u32, u32, i32) -> i32),
        entry!(0x00d2b790, hk_socket_reader_adapter_is_ok(u32, u32) -> u32),
        entry!(0x00d2b7b0, hk_socket_writer_adapter_write(u32, u32, i32) -> i32),
        entry!(0x00d2b800, hk_socket_writer_adapter_is_ok(u32, u32) -> u32),
        entry!(0x00d2b820, fn_00d2b820() -> u32),
        entry!(0x00d2b850, hk_socket_constructor(Ptr) -> Ptr),
        entry!(0x00d2b8b0, fn_00d2b8b0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00d2b8c0, hk_socket_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A socket (vtable slots `0xc`, `0x14`, `0x18`) with an adapter at `+8`; the receive and
    /// send doubles answer the byte counts in `answers`, one per call, logging
    /// `(pointer, remaining)`.
    fn socket_with_adapter(e: &mut Engine, answers: &'static [i32]) -> (u32, Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for slot in [0x14u32, 0x18] {
            let log = Rc::clone(&calls);
            let mut next = 0;
            e.register_double(0x0c00_0000 + slot, move |_, a| {
                log.borrow_mut().push((a[1], a[2]));
                let n = answers.get(next).copied().unwrap_or(0);
                next += 1;
                Ret {
                    eax: n as u32,
                    ..Ret::default()
                }
            });
        }
        e.register_double(0x0c00_000c, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        for slot in [0xcu32, 0x14, 0x18] {
            e.mem.set_u32(vtable + slot, 0x0c00_0000 + slot);
        }
        let socket = e.mem.alloc(0x40);
        e.mem.set_u32(socket, vtable);
        let adapter = e.mem.alloc(0x10);
        e.mem.set_u32(adapter + 8, socket);
        (adapter, calls)
    }

    #[test]
    fn reader_loops_until_everything_is_read() {
        let mut e = Engine::new();
        let (adapter, calls) = socket_with_adapter(&mut e, &[3, 5, 2]);
        let got = e.call(0x00d2_b740, &args![adapter, 0x1000u32, 10i32]).i32();
        assert_eq!(got, 10);
        assert_eq!(
            *calls.borrow(),
            vec![(0x1000, 10), (0x1003, 7), (0x1008, 2)]
        );
    }

    #[test]
    fn reader_stops_with_the_total_when_a_read_moves_nothing() {
        let mut e = Engine::new();
        let (adapter, calls) = socket_with_adapter(&mut e, &[4, 0]);
        let got = e.call(0x00d2_b740, &args![adapter, 0x1000u32, 10i32]).i32();
        assert_eq!(got, 4);
        assert_eq!(calls.borrow().len(), 2);
    }

    #[test]
    fn reader_answers_count_without_a_call_for_a_count_of_zero_or_less() {
        let mut e = Engine::new();
        let (adapter, calls) = socket_with_adapter(&mut e, &[9]);
        assert_eq!(
            e.call(0x00d2_b740, &args![adapter, 0x1000u32, 0i32]).i32(),
            0
        );
        assert_eq!(
            e.call(0x00d2_b740, &args![adapter, 0x1000u32, -5i32]).i32(),
            -5
        );
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn reader_counts_an_over_long_read_as_done() {
        let mut e = Engine::new();
        // One read of more than asked for ends the loop; the answer is still the count asked for.
        let (adapter, calls) = socket_with_adapter(&mut e, &[12]);
        let got = e.call(0x00d2_b740, &args![adapter, 0x1000u32, 10i32]).i32();
        assert_eq!(got, 10);
        assert_eq!(calls.borrow().len(), 1);
    }

    #[test]
    fn writer_uses_the_send_slot_with_the_same_loop() {
        let mut e = Engine::new();
        let (adapter, calls) = socket_with_adapter(&mut e, &[6, 0]);
        let got = e.call(0x00d2_b7b0, &args![adapter, 0x2000u32, 9i32]).i32();
        assert_eq!(got, 6);
        assert_eq!(*calls.borrow(), vec![(0x2000, 9), (0x2006, 3)]);
        let (adapter, _) = socket_with_adapter(&mut e, &[9]);
        assert_eq!(
            e.call(0x00d2_b7b0, &args![adapter, 0x2000u32, 9i32]).i32(),
            9
        );
        assert_eq!(
            e.call(0x00d2_b7b0, &args![adapter, 0x2000u32, 0i32]).i32(),
            0
        );
    }

    #[test]
    fn is_ok_of_both_adapters_forwards_the_result_pointer() {
        for address in [0x00d2_b790u32, 0x00d2_b800] {
            let mut e = Engine::new();
            let (adapter, _) = socket_with_adapter(&mut e, &[]);
            let out = e.mem.alloc(4);
            assert_eq!(e.call(address, &args![adapter, out]).u32(), out);
            assert_eq!(e.mem.u8(out), 1);
        }
    }

    #[test]
    fn fn_00d2b820_makes_a_socket_with_descriptor_minus_one() {
        let mut e = Engine::new();
        let allocator_vtable = e.mem.alloc(0x20);
        e.register_double(0x0c10_0004, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.mem.set_u32(allocator_vtable + 4, 0x0c10_0004);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(BSD_SOCKET_CONSTRUCTOR, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e.call(0x00d2_b820, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let made = log.iter().find(|c| c.0 == BSD_SOCKET_CONSTRUCTOR).unwrap();
        assert_eq!(made.1[1], 0xffff_ffff);
        assert_eq!(got, made.1[0] + 1);
        assert_eq!(e.mem.u16(made.1[0] + 4), 0x24);
    }

    #[test]
    fn constructor_stores_the_vtables_and_starts_the_network_once() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x011b_3000, 0x1000);
        let this = e.mem.alloc(0x24);
        let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let count = Rc::clone(&started);
        e.register_double(0x0c20_0000, move |_, _| {
            *count.borrow_mut() += 1;
            Ret::default()
        });
        e.mem.set_u32(NETWORK_INIT, 0x0c20_0000);
        assert_eq!(e.call(0x00d2_b850, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SOCKET_VTABLE);
        assert_eq!(e.mem.u32(this + 8), READER_ADAPTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), WRITER_ADAPTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), this);
        assert_eq!(e.mem.u32(this + 0x1c), this);
        for offset in [6, 0xe, 0x1a] {
            assert_eq!(e.mem.u16(this + offset), 1);
        }
        assert_eq!(*started.borrow(), 1);
        assert_eq!(e.mem.u8(NETWORK_STARTED), 1);
        // The second socket finds the network started.
        let second = e.mem.alloc(0x24);
        e.call(0x00d2_b850, &args![second]);
        assert_eq!(*started.borrow(), 1);
    }

    #[test]
    fn constructor_without_an_init_function_leaves_the_byte_clear() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x011b_3000, 0x1000);
        let this = e.mem.alloc(0x24);
        e.call(0x00d2_b850, &args![this]);
        assert_eq!(e.mem.u8(NETWORK_STARTED), 0);
    }

    #[test]
    fn fn_00d2b8b0_answers_1() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d2_b8b0, &args![0x10u32, 1u32, 2u32, 3u32]).u32(),
            1
        );
    }

    #[test]
    fn destructor_stores_the_base_vtables_and_frees_on_request() {
        let mut e = Engine::new();
        let freed: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&freed);
        e.register_double(0x0c30_0008, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0c30_0008);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 4, 0x24);
        assert_eq!(e.call(0x00d2_b8c0, &args![this, 0xfeu32]).u32(), this);
        for offset in [0, 8, 0x14] {
            assert_eq!(e.mem.u32(this + offset), BASE_OBJECT_VTABLE);
        }
        assert!(freed.borrow().is_empty());
        assert_eq!(e.call(0x00d2_b8c0, &args![this, 1u32]).u32(), this);
        assert_eq!(*freed.borrow(), vec![vec![allocator, this, 0x24]]);
    }
}
