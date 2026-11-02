//! `Havok SDK/run_00d41ec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`); the allocator is the word at router + 0x10, and its
/// virtual function at offset 4 allocates a block of the given size.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpGskfAgent::hkpGskfAgent` (Xbox PDB, `00dacbd0`, `this` and three words).
const GSKF_AGENT_CONSTRUCT: u32 = 0x00da_cbd0;
/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB, `00dabf10`, `this` and three words).
const GSK_BASE_AGENT_CONSTRUCT: u32 = 0x00da_bf10;
/// The vtable the factory stores into the larger agent after construction (`010ceb94`;
/// unconfirmed class).
const LARGE_AGENT_VTABLE: u32 = 0x010c_eb94;

// Translated from 00d41f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent factory (cdecl, four words `a, b, _, flag`; the third word is not read). The class is
/// unconfirmed. With a non-zero `flag` it allocates 0x80 bytes, stores the size at +4, runs
/// the `hkpGskfAgent` constructor (`00dacbd0`) with `(a, b, flag)` and then stores vtable
/// `010ceb94` over the one the constructor wrote. With a zero `flag` it allocates 0x30
/// bytes, stores the size at +4 and runs the `hkpGskBaseAgent` constructor (`00dabf10`) with
/// `(a, b, 0)`. Returns the new object (the constructor's return in the second case).
pub fn fn_00d41f50(e: &mut Engine, a: u32, b: u32, _unused_2: u32, flag: u32) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if flag != 0 {
        let block = e.vcall(allocator, 4, &args![0x80u32]).u32();
        e.mem.set_u16(block + 4, 0x80);
        e.call(GSKF_AGENT_CONSTRUCT, &args![block, a, b, flag]);
        e.mem.set_u32(block, LARGE_AGENT_VTABLE);
        return Ptr::new(block);
    }
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block + 4, 0x30);
    e.call(GSK_BASE_AGENT_CONSTRUCT, &args![block, a, b, 0u32])
        .ptr()
}

// Translated from 00d41ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `record`): fills a record of function words: `+0` = `0x00d418d0`,
/// `+0x2c` = `0x00d41a30`, `+0x28` = `0x00d41930`, `+8` = `0x00d41860`, `+0xc` = `0x00d417b0`,
/// `+0x10` = `0x00d417f0`, `+0x14` = `0x00d41830`, `+4` = `0x00d418b0`, and the byte at `+0x30` = 1.
pub fn fn_00d41ec0(e: &mut Engine, record: Ptr) {
    let r = record.addr();
    e.mem.set_u32(r, 0x00d4_18d0);
    e.mem.set_u32(r + 0x2c, 0x00d4_1a30);
    e.mem.set_u32(r + 0x28, 0x00d4_1930);
    e.mem.set_u32(r + 8, 0x00d4_1860);
    e.mem.set_u32(r + 0xc, 0x00d4_17b0);
    e.mem.set_u32(r + 0x10, 0x00d4_17f0);
    e.mem.set_u32(r + 0x14, 0x00d4_1830);
    e.mem.set_u32(r + 4, 0x00d4_18b0);
    e.mem.set_u8(r + 0x30, 1);
}

// Translated from 00d41f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `queue`, `first`, `second`; descriptions, not confirmed): builds
/// a 0x40-byte record on the stack (zero bytes at `+0x30` .. `+0x32` and zero words at `+0x18` ..
/// `+0x28`, then `00d41ec0`; the rest uninitialised in the exe, zero here) and registers it with
/// `00cfbd60` on `queue` with `(record, first, second)`.
pub fn fn_00d41f00(e: &mut Engine, queue: Ptr, first: u32, second: u32) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x30, 0);
        e.mem.set_u8(r + 0x31, 0);
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(r + offset, 0);
        }
        fn_00d41ec0(e, record);
        e.call(0x00cf_bd60, &args![queue, record, first, second]);
    });
}

// Translated from 00d41fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers a 0x14-byte record
/// built on the stack (function words `+0` = `0x00d41f50`, `+4` = `0x00dac2b0`, `+8` =
/// `0x00dac450`, `+0xc` = `0x00daf8e0`, the byte at `+0x10` = 0 and at `+0x11` = 1) with
/// `00cfbbe0` on `queue` with `(record, 0x12, 0x12)`. The padding bytes at `+0x12` are
/// uninitialised in the exe (zero here).
pub fn fn_00d41fd0(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r, 0x00d4_1f50);
        e.mem.set_u32(r + 4, 0x00da_c2b0);
        e.mem.set_u32(r + 8, 0x00da_c450);
        e.mem.set_u32(r + 0xc, 0x00da_f8e0);
        e.mem.set_u8(r + 0x10, 0);
        e.mem.set_u8(r + 0x11, 1);
        e.call(0x00cf_bbe0, &args![queue, record, 0x12u32, 0x12u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_1f50, fn_00d41f50(u32, u32, u32, u32) -> Ptr),
        entry!(0x00d4_1ec0, fn_00d41ec0(Ptr)),
        entry!(0x00d4_1f00, fn_00d41f00(Ptr, u32, u32)),
        entry!(0x00d4_1fd0, fn_00d41fd0(Ptr)),
    ]
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

    const ALLOC_SLOT_DOUBLE: u32 = 0x0900_0004;

    fn rig(e: &mut Engine, block: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            ALLOC_SLOT_DOUBLE,
            GSKF_AGENT_CONSTRUCT,
            GSK_BASE_AGENT_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: block,
                    ..Ret::default()
                }
            });
        }
        log
    }

    #[test]
    fn a_non_zero_flag_builds_the_large_agent() {
        let mut e = test_engine();
        let block = e.mem.alloc(0x100);
        let log = rig(&mut e, block);
        let got = e.call(0x00d4_1f50, &args![5u32, 6u32, 7u32, 9u32]);
        assert_eq!(got.u32(), block);
        assert_eq!(e.mem.u16(block + 4), 0x80);
        assert_eq!(e.mem.u32(block), LARGE_AGENT_VTABLE);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1], 0x80);
        assert_eq!(calls[1], (GSKF_AGENT_CONSTRUCT, vec![block, 5, 6, 9]));
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn a_zero_flag_builds_the_base_agent() {
        let mut e = test_engine();
        let block = e.mem.alloc(0x100);
        let log = rig(&mut e, block);
        let got = e.call(0x00d4_1f50, &args![5u32, 6u32, 7u32, 0u32]);
        assert_eq!(got.u32(), block);
        assert_eq!(e.mem.u16(block + 4), 0x30);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1], 0x30);
        assert_eq!(calls[1], (GSK_BASE_AGENT_CONSTRUCT, vec![block, 5, 6, 0]));
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    const RECORD: [u32; 13] = [
        0x00d4_18d0,
        0x00d4_18b0,
        0x00d4_1860,
        0x00d4_17b0,
        0x00d4_17f0,
        0x00d4_1830,
        0,
        0,
        0,
        0,
        0x00d4_1930,
        0x00d4_1a30,
        1,
    ];

    #[test]
    fn fn_00d41ec0_fills_the_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x40);
        e.call(0x00d4_1ec0, &args![record]);
        let words: Vec<u32> = (0..13).map(|word| e.mem.u32(record + word * 4)).collect();
        assert_eq!(words, RECORD.to_vec());
    }

    #[test]
    fn fn_00d41f00_registers_the_record_with_the_two_numbers() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bd60, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..13 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_1f00, &args![queue, 6u32, 9u32]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [6, 9]);
        assert_eq!(seen[4..], RECORD);
    }

    #[test]
    fn fn_00d41fd0_registers_the_record() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..5 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_1fd0, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[2..4], [0x12, 0x12]);
        assert_eq!(
            seen[4..],
            [
                0x00d4_1f50,
                0x00da_c2b0,
                0x00da_c450,
                0x00da_f8e0,
                0x0000_0100
            ]
        );
    }
}
