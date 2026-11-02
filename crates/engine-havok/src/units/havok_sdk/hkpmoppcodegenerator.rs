//! `Havok SDK/hkpmoppcodegenerator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e066c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_6650`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e066c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_6650, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree`
/// (`00c856c0`, cdecl `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkString::memSet` (`00cb1280`, cdecl `(destination, byte, size)`) and `memCpy`
/// (`00cb1260`, cdecl `(destination, source, size)`).
const MEM_SET: u32 = 0x00cb_1280;
const MEM_COPY: u32 = 0x00cb_1260;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The vtable of the generator (`010ea5b4`), of the compiled code (`0102e368`) and the base one.
const VTABLE_GENERATOR: u32 = 0x010e_a5b4;
const VTABLE_CODE: u32 = 0x0102_e368;
const VTABLE_BASE: u32 = 0x0102_e388;

// Translated from 00e06480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::validateJumpCommands` (Xbox PDB): does nothing in this build.
pub fn hkp_mopp_code_generator_validate_jump_commands(_e: &mut Engine, _this: Ptr) {}

// Translated from 00e06490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::resize` (Xbox PDB), `__fastcall`: doubles the code buffer (size at
/// `+8`, data at `+0x10`). The new buffer is filled with `0xCD` over its first half and gets
/// the old contents in its second half, the old one is freed.
pub fn hkp_mopp_code_generator_resize(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let doubled = e.mem.u32(me + 8).wrapping_add(e.mem.u32(me + 8));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let fresh = e.call(EASY_ALLOC, &args![allocator, doubled]).u32();
    let size = e.mem.u32(me + 8);
    e.call(MEM_SET, &args![fresh, 0xcdu32, size]);
    let size = e.mem.u32(me + 8);
    let old = e.mem.u32(me + 0x10);
    e.call(MEM_COPY, &args![size.wrapping_add(fresh), old, size]);
    let size = e.mem.u32(me + 8);
    let old = e.mem.u32(me + 0x10);
    e.mem.set_u32(me + 8, size.wrapping_add(size));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![allocator, old]);
    e.mem.set_u32(me + 0x10, fresh);
}

// Translated from 00e064f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::compileCode` (Xbox PDB), `__fastcall`: copies the `+0xC` bytes the
/// generator wrote at the end of its buffer (size `+8`, data `+0x10`) into a new 0x30-byte
/// code object, which has the vtable `0102e368`, the 16-bit words 0x30 and 1 at `+4` and
/// `+6`, a zero vector at `+0x10`, a byte array at `+0x20` and the byte 2 at `+0x2C`. The
/// array is made to hold the bytes (reserved twice, the second time by the doubling policy),
/// the buffer is freed and the generator is left with no data, nothing written and the size
/// set to the byte count. Returns the object.
pub fn hkp_mopp_code_generator_compile_code(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    let used = e.mem.u32(me + 0xc);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let object = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(object + 4, 0x30);
    e.mem.set_u16(object + 6, 1);
    e.mem.set_u32(object, VTABLE_CODE);
    let bytes = object + 0x20;
    e.mem.set_u32(bytes, 0);
    e.mem.set_u32(bytes + 4, 0);
    e.mem.set_u32(bytes + 8, 0x8000_0000);
    for word in 0..4 {
        e.mem.set_u32(object + 0x10 + 4 * word, 0);
    }
    e.mem.set_u8(object + 0x2c, 2);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(bytes + 8) & 0x3fff_ffff) as i32;
    if capacity < used as i32 {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, bytes, used, 1u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(bytes + 8) & 0x3fff_ffff) as i32;
    if capacity < used as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let count = if (used as i32) < doubled {
            doubled as u32
        } else {
            used
        };
        e.call(ARRAY_RESERVE, &args![allocator, bytes, count, 1u32]);
    }
    e.mem.set_u32(bytes + 4, used);
    let source = e
        .mem
        .u32(me + 0x10)
        .wrapping_sub(e.mem.u32(me + 0xc))
        .wrapping_add(e.mem.u32(me + 8));
    let data = e.mem.u32(bytes);
    e.call(MEM_COPY, &args![data, source, used]);
    let buffer = e.mem.u32(me + 0x10);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![allocator, buffer]);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 8, used);
    Ptr::new(object)
}

// Translated from 00e065f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::hkpMoppCodeGenerator` (Xbox PDB), `RET 4` with the buffer size:
/// sets the 16-bit word 1 at `+6`, the vtable `010ea5b4`, an empty array at `+0x14`, the size
/// at `+8`, a buffer of that many bytes from `easyAlloc` at `+0x10` and 0 at `+0xC`. Returns
/// `this`.
pub fn hkp_mopp_code_generator_constructor(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE_GENERATOR);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let buffer = e.call(EASY_ALLOC, &args![allocator, size]).u32();
    e.mem.set_u32(me + 8, size);
    e.mem.set_u32(me + 0x10, buffer);
    e.mem.set_u32(me + 0xc, 0);
    this
}

// Translated from 00e06650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCodeGenerator::~hkpMoppCodeGenerator` (Xbox PDB), `__fastcall`: frees the buffer at
/// `+0x10` when there is one, empties the array at `+0x14` (8-byte elements, size `+0x18`,
/// capacity word `+0x1C`; freed unless not owned) and ends with the base vtable.
pub fn hkp_mopp_code_generator_destructor(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let buffer = e.mem.u32(me + 0x10);
    e.mem.set_u32(me, VTABLE_GENERATOR);
    if buffer != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(EASY_FREE, &args![allocator, buffer]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x1c);
    e.mem.set_u32(me + 0x18, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x14);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(8)],
        );
    }
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x1c, 0x8000_0000);
    e.mem.set_u32(me, VTABLE_BASE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_66c0, deleting_destructor_00e066c0(Ptr, u8) -> Ptr),
        entry!(
            0x00e0_6480,
            hkp_mopp_code_generator_validate_jump_commands(Ptr)
        ),
        entry!(0x00e0_6490, hkp_mopp_code_generator_resize(Ptr)),
        entry!(
            0x00e0_64f0,
            hkp_mopp_code_generator_compile_code(Ptr) -> Ptr
        ),
        entry!(
            0x00e0_65f0,
            hkp_mopp_code_generator_constructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e0_6650, hkp_mopp_code_generator_destructor(Ptr)),
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
    fn deleting_destructor_00e066c0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_66c0, 0x00e0_6650);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0501;
    const FREE_SLOT: u32 = 0x0900_0502;

    fn push(log: &Log, row: Vec<u32>) {
        log.borrow_mut().push(row);
    }

    /// The memory router with an allocator (slot 4 allocates by value, slot `0x10` frees), and
    /// working stand-ins for the allocation helpers and the string functions, all logged.
    fn rig() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            push(&sink, vec![1, a[1]]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_SLOT, move |_, a| {
            push(&sink, vec![2, a[1], a[2]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(EASY_ALLOC, move |e, a| {
            push(&sink, vec![3, a[0], a[1]]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(EASY_FREE, move |_, a| {
            push(&sink, vec![4, a[0], a[1]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(MEM_SET, move |e, a| {
            push(&sink, vec![5, a[1], a[2]]);
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(MEM_COPY, move |e, a| {
            push(&sink, vec![6, a[2]]);
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        (e, log, allocator)
    }

    #[test]
    fn validating_jump_commands_does_nothing() {
        let (mut e, log, _allocator) = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00e06480, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn resize_doubles_the_buffer_and_keeps_the_old_contents_in_the_upper_half() {
        let (mut e, log, allocator) = rig();
        let this = e.mem.alloc(0x40);
        let old = e.mem.alloc(8);
        for i in 0..8 {
            e.mem.set_u8(old + i, 0x10 + i as u8);
        }
        e.mem.set_u32(this + 8, 8);
        e.mem.set_u32(this + 0x10, old);
        e.call(0x00e06490, &args![this]);
        let fresh = e.mem.u32(this + 0x10);
        assert_ne!(fresh, old);
        assert_eq!(e.mem.u32(this + 8), 16);
        let bytes: Vec<u8> = (0..16).map(|i| e.mem.u8(fresh + i)).collect();
        assert_eq!(
            bytes,
            vec![
                0xcd, 0xcd, 0xcd, 0xcd, 0xcd, 0xcd, 0xcd, 0xcd, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
                0x16, 0x17
            ]
        );
        assert_eq!(
            *log.borrow(),
            vec![
                vec![3, allocator, 16],
                vec![5, 0xcd, 8],
                vec![6, 8],
                vec![4, allocator, old],
            ]
        );
    }

    #[test]
    fn compile_code_copies_the_written_tail_into_a_new_code_object() {
        let (mut e, log, allocator) = rig();
        let reserves: Log = Rc::default();
        let sink = Rc::clone(&reserves);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            push(&sink, a.to_vec());
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let buffer = e.mem.alloc(0x40);
        for i in 0..5 {
            e.mem.set_u8(buffer + 0x3b + i, 1 + i as u8);
        }
        e.mem.set_u32(this + 8, 0x40);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x10, buffer);
        let object = e.call(0x00e064f0, &args![this]).u32();
        assert_eq!(e.mem.u32(object), VTABLE_CODE);
        assert_eq!(e.mem.u16(object + 4), 0x30);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u8(object + 0x2c), 2);
        assert_eq!(e.mem.u32(object + 0x24), 5);
        let data = e.mem.u32(object + 0x20);
        let copied: Vec<u8> = (0..5).map(|i| e.mem.u8(data + i)).collect();
        assert_eq!(copied, vec![1, 2, 3, 4, 5]);
        // The array was made to hold the 5 bytes once: the second check finds the room.
        assert_eq!(
            *reserves.borrow(),
            vec![vec![allocator, object + 0x20, 5, 1]]
        );
        assert_eq!(
            *log.borrow(),
            vec![vec![1, 0x30], vec![6, 5], vec![4, allocator, buffer]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    #[test]
    fn compile_code_reserves_again_by_doubling_when_the_first_reserve_falls_short() {
        let (mut e, _log, allocator) = rig();
        let reserves: Log = Rc::default();
        let sink = Rc::clone(&reserves);
        // A reserve that grants one byte less than asked the first time.
        e.register_double(ARRAY_RESERVE, move |e, a| {
            push(&sink, a.to_vec());
            let granted = if sink.borrow().len() == 1 {
                a[2] - 1
            } else {
                a[2]
            };
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, granted);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x40);
        e.mem.set_u32(this + 0xc, 6);
        e.mem.set_u32(this + 0x10, buffer);
        let object = e.call(0x00e064f0, &args![this]).u32();
        // Capacity 5 < 6: the second request is max(6, 2 * 5).
        assert_eq!(
            *reserves.borrow(),
            vec![
                vec![allocator, object + 0x20, 6, 1],
                vec![allocator, object + 0x20, 10, 1]
            ]
        );
    }

    #[test]
    fn the_constructor_takes_a_buffer_of_the_given_size() {
        let (mut e, log, allocator) = rig();
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00e065f0, &args![this, 0x100u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![vec![3, allocator, 0x100]]);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE_GENERATOR);
        assert_eq!(e.mem.u32(this + 8), 0x100);
        assert_ne!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
    }

    #[test]
    fn the_destructor_frees_the_buffer_and_the_owned_array() {
        let (mut e, log, allocator) = rig();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x4000);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x18, 3);
        e.mem.set_u32(this + 0x1c, 6);
        e.call(0x00e06650, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![vec![4, allocator, 0x4000], vec![2, 0x5000, 48]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_BASE);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        // No buffer and an array that is not owned: nothing is freed.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x1c, 0x8000_0006);
        e.call(0x00e06650, &args![this]);
        assert!(log.borrow().is_empty());
    }
}
