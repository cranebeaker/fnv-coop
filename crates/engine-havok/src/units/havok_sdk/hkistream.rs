//! `Havok SDK/hkistream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dcca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_c160`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcca30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_c160, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The virtual table both constructors store (`010d7540`; `hkIstream`'s).
const ISTREAM_VTABLE: u32 = 0x010d_7540;
/// Callees outside this file: the memory stream reader's constructor (`00e05b40`,
/// `thiscall`, `RET 0xc`: buffer, length, mode), the other reader constructor
/// (`00d466e0`, `thiscall`, `RET 8`: one word, mode), the array reset that leaves an empty
/// string of size 1 (`00d61cc0`, `thiscall`), the whitespace skip (`00dcc1b0`, which takes
/// the reader in `EDI`: the register value is its first word), one character read
/// (`00cb1550`, `thiscall` on the reader, answers the character or -1) and
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: allocator, array, element size).
const MEMORY_STREAM_READER_CONSTRUCTOR: u32 = 0x00e0_5b40;
const STREAM_READER_CONSTRUCTOR: u32 = 0x00d4_66e0;
const STRING_BUFFER_RESET: u32 = 0x00d6_1cc0;
const SKIP_WHITESPACE: u32 = 0x00dc_c1b0;
const READ_CHARACTER: u32 = 0x00cb_1550;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;

/// Builds a reader object: `size` bytes from the memory router's allocator (slot 4), the
/// 16-bit size stored at `+4` of the block, then `constructor` run on it with `words`.
fn make_reader(e: &mut Engine, size: u32, constructor: u32, words: &[u32]) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    let mut arguments = vec![block];
    arguments.extend_from_slice(words);
    e.call(constructor, &arguments).u32()
}

// Translated from 00dcc6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkIstream::hkIstream` (Xbox PDB), `thiscall`, `RET 8` (`buffer`, `length`): stores 1 in
/// the word at `+6` and the virtual table (`010d7540`), allocates 0x1c bytes through the
/// memory router's allocator (slot 4), stores the size 0x1c at `+4` of the block and
/// constructs a memory stream reader in it (`00e05b40`: buffer, length, 2). The reader
/// becomes the word at `+8`. Returns `this`.
pub fn hk_istream_hk_istream(e: &mut Engine, this: Ptr, buffer: u32, length: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), ISTREAM_VTABLE);
    let reader = make_reader(
        e,
        0x1c,
        MEMORY_STREAM_READER_CONSTRUCTOR,
        &[buffer, length, 2],
    );
    e.mem.set_u32(this.addr() + 8, reader);
    this
}

// Translated from 00dcc6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A second `hkIstream` constructor (unnamed in the engine map; the class and the meaning
/// of its argument are unconfirmed), `thiscall`, `RET 4` (`source`): as `00dcc6a0`, but the
/// reader is 0x14 bytes and constructed by `00d466e0` with (`source`, 2). Returns `this`.
pub fn fn_00dcc6f0(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), ISTREAM_VTABLE);
    let reader = make_reader(e, 0x14, STREAM_READER_CONSTRUCTOR, &[source, 2]);
    e.mem.set_u32(this.addr() + 8, reader);
    this
}

/// Appends `value` to the `hkArray<char>` at `array` (data `+0`, size `+4`, capacity and
/// flags `+8`), growing it through `00c90940` with the memory router's word at `+0xc`
/// when it is full.
fn push_character(e: &mut Engine, array: u32, value: u8) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0xc);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 1u32]);
    }
    let size = e.mem.u32(array + 4);
    let at = size.wrapping_add(e.mem.u32(array));
    e.mem.set_u8(at, value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00dcc8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `out`, an `hkArray<char>`): reads one
/// whitespace-delimited word from the reader at `this + 8` into `out`. The array is reset
/// first (`00d61cc0`), whitespace is skipped (`00dcc1b0` with the reader), the reader's slot
/// `0x1c` is called with 1 and a character is read (`00cb1550`); every character that is
/// not -1, space, tab, carriage return or line feed is appended to the array (growing it
/// when full) and followed by slot `0x1c` with 1 and the next read. The reader's slot
/// `0x20` is then called and a 0 byte is appended. Returns `this`.
pub fn fn_00dcc8e0(e: &mut Engine, this: Ptr, out: u32) -> Ptr {
    e.call(STRING_BUFFER_RESET, &args![out]);
    let reader = e.mem.u32(this.addr() + 8);
    e.call(SKIP_WHITESPACE, &args![reader]);
    e.vcall(reader, 0x1c, &args![1u32]);
    let mut character = e.call(READ_CHARACTER, &args![reader]).i32();
    while character != -1
        && character != 0x20
        && character != 9
        && character != 0xd
        && character != 0xa
    {
        push_character(e, out, character as u8);
        e.vcall(reader, 0x1c, &args![1u32]);
        character = e.call(READ_CHARACTER, &args![reader]).i32();
    }
    e.vcall(reader, 0x20, &args![]);
    push_character(e, out, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_ca30, deleting_destructor_00dcca30(Ptr, u8) -> Ptr),
        entry!(0x00dcc6a0, hk_istream_hk_istream(Ptr, u32, u32) -> Ptr),
        entry!(0x00dcc6f0, fn_00dcc6f0(Ptr, u32) -> Ptr),
        entry!(0x00dcc8e0, fn_00dcc8e0(Ptr, u32) -> Ptr),
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
    fn deleting_destructor_00dcca30_frees_only_when_bit_zero_is_set() {
        check(0x00dc_ca30, 0x00dc_c160);
    }

    const ALLOCATOR_SLOT_TARGET: u32 = 0x4400_4001;
    const READER_VTABLE: u32 = 0x4400_4100;
    const READER_SLOT_MARK: u32 = 0x4400_4002;
    const READER_SLOT_RESET: u32 = 0x4400_4003;

    /// A router whose allocator (vtable slot 4) hands out `block`.
    fn router_with_allocator(e: &mut Engine, block: u32) {
        e.put_vtable(0x4400_4200, &[0, ALLOCATOR_SLOT_TARGET]);
        e.register_double(ALLOCATOR_SLOT_TARGET, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_4200);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(router + 0xc, 0x7777);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    #[test]
    fn the_constructors_build_their_readers() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        router_with_allocator(&mut e, block);
        e.register(MEMORY_STREAM_READER_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(STREAM_READER_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00dc_c6a0, &args![this, 0x5000u32, 0x30u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ISTREAM_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u16(block + 4), 0x1c);
        let log = e.call_log.take().unwrap();
        let made: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == MEMORY_STREAM_READER_CONSTRUCTOR)
            .map(|c| &c.1)
            .collect();
        assert_eq!(made, vec![&vec![block, 0x5000, 0x30, 2]]);
        let allocation: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == ALLOCATOR_SLOT_TARGET)
            .map(|c| &c.1)
            .collect();
        assert_eq!(allocation[0][1], 0x1c);
        // The second constructor builds 0x14 bytes with (source, 2).
        let other = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_c6f0, &args![other, 0x6000u32]).u32(), other);
        assert_eq!(e.mem.u16(block + 4), 0x14);
        assert_eq!(e.mem.u32(other + 8), block);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(STREAM_READER_CONSTRUCTOR, vec![block, 0x6000, 2])));
    }

    #[test]
    fn a_word_is_read_up_to_whitespace_and_terminated() {
        use std::cell::RefCell;
        use std::collections::VecDeque;
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        router_with_allocator(&mut e, block);
        e.register(STRING_BUFFER_RESET, |e, a| {
            // The reset leaves an empty string of size 1.
            e.mem.set_u32(a[0] + 4, 1);
            let data = e.mem.u32(a[0]);
            e.mem.set_u8(data, 0);
            Ret::default()
        });
        e.register(SKIP_WHITESPACE, |_, _| Ret::default());
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            // Moves the array to 8 bytes of room.
            let data = e.mem.alloc(8);
            let old = e.mem.u32(a[1]);
            for i in 0..e.mem.u32(a[1] + 4) {
                let byte = e.mem.u8(old + i);
                e.mem.set_u8(data + i, byte);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let script = std::rc::Rc::new(RefCell::new(VecDeque::from([
            i32::from(b'a'),
            i32::from(b'b'),
            0x20,
        ])));
        let source = std::rc::Rc::clone(&script);
        e.register_double(READ_CHARACTER, move |_, _| Ret {
            eax: source.borrow_mut().pop_front().unwrap_or(-1) as u32,
            ..Ret::default()
        });
        e.register(READER_SLOT_MARK, |_, _| Ret::default());
        e.register(READER_SLOT_RESET, |_, _| Ret::default());
        e.put_vtable(
            READER_VTABLE,
            &[0, 0, 0, 0, 0, 0, 0, READER_SLOT_MARK, READER_SLOT_RESET],
        );
        let reader = e.mem.alloc(8);
        e.mem.set_u32(reader, READER_VTABLE);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, reader);
        // An array with room for 2 bytes.
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(2);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_c8e0, &args![this, array]).u32(), this);
        let size = e.mem.u32(array + 4);
        let bytes = e.mem.bytes(e.mem.u32(array), size);
        assert_eq!(bytes, vec![0, b'a', b'b', 0]);
        let log = e.call_log.take().unwrap();
        let count = |address: u32| log.iter().filter(|c| c.0 == address).count();
        assert_eq!(count(READER_SLOT_MARK), 3);
        assert_eq!(count(READER_SLOT_RESET), 1);
        assert_eq!(count(ARRAY_RESERVE_MORE), 1);
        assert_eq!(count(READ_CHARACTER), 3);
        assert!(log.contains(&(SKIP_WHITESPACE, vec![reader])));
        // End of input (-1) also ends the word.
        e.call(0x00dc_c8e0, &args![this, array]);
        assert_eq!(e.mem.u32(array + 4), 2);
    }
}
