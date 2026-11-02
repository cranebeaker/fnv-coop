//! `Havok SDK/run_00d39840` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d39840 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers a group of six
/// functions with `00cfbd60` on `queue`, with `(record, 5, 3)`. The record, 0x40 bytes on the stack,
/// holds the function words `+0` = `0x00d390a0`, `+4` = `0x00d39130`, `+8` = `0x00d390e0`, `+0xc` =
/// `0x00d39030`, `+0x10` = `0x00d39060`, `+0x14` = `0x00d39090`, `+0x28` = `0x00d39150`, `+0x2c` =
/// `0x00d39390`, zero words at `+0x18` .. `+0x24` and zero bytes at `+0x30` .. `+0x32`; the rest is
/// uninitialised in the exe (zero here).
pub fn fn_00d39840(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x31, 0);
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(r + offset, 0);
        }
        e.mem.set_u8(r + 0x30, 0);
        for (offset, function) in [
            (0u32, 0x00d3_90a0u32),
            (0x2c, 0x00d3_9390),
            (0x28, 0x00d3_9150),
            (0x08, 0x00d3_90e0),
            (0x0c, 0x00d3_9030),
            (0x10, 0x00d3_9060),
            (0x14, 0x00d3_9090),
            (0x04, 0x00d3_9130),
        ] {
            e.mem.set_u32(r + offset, function);
        }
        e.call(0x00cf_bd60, &args![queue, record, 5u32, 3u32]);
    });
}

// Translated from 00d398c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first and third are never read (`_unused_0`, `_unused_2`), the second is the second collidable (`body_b`) and the last is the contact manager, `manager`. Allocates 0x28 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc, +0xe and +0x10 and the vtable word `010ceefc` at +0. Then calls `00cd72a0` with the data at +0x20 of the shape (the word at +0 of the collidable) and the address of the block + 0x14. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3ab66.
pub fn fn_00d398c0(
    e: &mut Engine,
    _unused_0: u32,
    body_b: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(block + 4, 0x28);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    e.mem.set_u16(block + 0xc, 0xffff);
    e.mem.set_u16(block + 0xe, 0xffff);
    e.mem.set_u16(block + 0x10, 0xffff);
    e.mem.set_u32(block, 0x010c_eefc);
    let shape = e.mem.u32(body_b);
    e.call(0x00cd_72a0, &args![shape + 0x20, block + 0x14]);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_9840, fn_00d39840(Ptr)),
        entry!(0x00d3_98c0, fn_00d398c0(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d39840_registers_the_group() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bd60, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..0x0c {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            for offset in [0x30, 0x31, 0x32] {
                words.push(u32::from(e.mem.u8(a[1] + offset)));
            }
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d3_9840, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [5, 3]);
        assert_eq!(
            seen[4..],
            [
                0x00d3_90a0,
                0x00d3_9130,
                0x00d3_90e0,
                0x00d3_9030,
                0x00d3_9060,
                0x00d3_9090,
                0,
                0,
                0,
                0,
                0x00d3_9150,
                0x00d3_9390,
                0,
                0,
                0
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00d398c0_fills_the_block_and_copies_the_shape_data() {
        let (mut e, sizes) = allocator_engine();
        let shape = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cd_72a0, move |e, a| {
            // The block already holds the first vtable word while the data is copied.
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1] - 0x14)));
            Ret::default()
        });
        let block = e
            .call(0x00d3_98c0, &args![0x1000u32, body, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x28]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![shape + 0x20, block + 0x14], 0x010c_eefc)]
        );
        assert_eq!(e.mem.u16(block + 4), 0x28);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_eefc);
        for offset in [0xc, 0xe, 0x10] {
            assert_eq!(e.mem.u16(block + offset), 0xffff);
        }
    }
}
