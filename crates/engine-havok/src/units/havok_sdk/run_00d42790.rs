//! `Havok SDK/run_00d42790` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The last node of the chain that starts at `start` and follows the word at `+0xc` until it is 0.
fn last_in_chain_00d42790(e: &Engine, start: u32) -> u32 {
    let mut node = start;
    loop {
        let next = e.mem.u32(node + 0xc);
        if next == 0 {
            return node;
        }
        node = next;
    }
}

// Translated from 00d42790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, `RET 0xc`: three stack words `first`, `second`, `third`; no map name,
/// class unconfirmed): `+6` (`u16`) = 1, `+8` = `third`, the vtable word `+0` = `0x010c_f238`,
/// `+0xc` = the last node of the chain from `first` (following the word at `+0xc`), `+0x10` = the
/// last node of the chain from `second`, `+0x1c` = the word at `+0xc` of `*first`, `+0x20` = the
/// word at `+0xc` of `*second`. Returns `this`.
pub fn fn_00d42790(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, third);
    e.mem.set_u32(me, 0x010c_f238);
    let first_last = last_in_chain_00d42790(e, first);
    e.mem.set_u32(me + 0xc, first_last);
    let second_last = last_in_chain_00d42790(e, second);
    e.mem.set_u32(me + 0x10, second_last);
    let first_head = e.mem.u32(first);
    let word = e.mem.u32(first_head + 0xc);
    e.mem.set_u32(me + 0x1c, word);
    let second_head = e.mem.u32(second);
    let word = e.mem.u32(second_head + 0xc);
    e.mem.set_u32(me + 0x20, word);
    this
}

// Translated from 00d42960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers one 0x14-byte
/// record (function words `+0` = `0x00d42800`, `+4` = `0x00d42730`, `+8` = `0x00d42760`, `+0xc` =
/// `0x00d42780`, bytes `+0x10` = 0 and `+0x11` = 1) with `00cfbbe0` on `queue` twice: with
/// `(record, 0x1d, 0xffffffff)` and with `(record, 0xffffffff, 0x1d)`. The padding bytes at `+0x12`
/// are uninitialised in the exe (zero here).
pub fn fn_00d42960(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x10, 0);
        e.mem.set_u32(r, 0x00d4_2800);
        e.mem.set_u32(r + 4, 0x00d4_2730);
        e.mem.set_u32(r + 8, 0x00d4_2760);
        e.mem.set_u32(r + 0xc, 0x00d4_2780);
        e.mem.set_u8(r + 0x11, 1);
        e.call(0x00cf_bbe0, &args![queue, record, 0x1du32, 0xffff_ffffu32]);
        e.call(0x00cf_bbe0, &args![queue, record, 0xffff_ffffu32, 0x1du32]);
    });
}

// Translated from 00d42800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (an agent creation function, cdecl, four stack words, nothing popped):
/// `body_a`, `body_b`, `input` and `manager`. Allocates 0x24 bytes through the memory router's
/// allocator (`00c85750`; virtual function at +4 of the object at router + 0x10), stores the size
/// as a 16-bit word at +4 and runs the constructor `00d42790` on it with `body_a`, `body_b` and
/// `manager`. When the word at +0x1c of the new object is 0x1d, the virtual function at +0x2c of
/// the shape of `body_a` (the word at +0 of the body) is called on that shape with the outermost
/// parent of `body_a`, the outermost parent of `body_b` (each found by following the word at +0xc
/// until it is 0) and `input`, and the shape is stored at +0x14; when the word at +0x20 is 0x1d
/// the same happens with the roles swapped (shape of `body_b`, parents `body_b`'s then `body_a`'s)
/// and the shape is stored at +0x18. Returns the new object. Ghidra made no function here; the address is an immediate operand at 00d42978.
pub fn fn_00d42800(e: &mut Engine, body_a: u32, body_b: u32, input: u32, manager: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x24u32]).u32();
    e.mem.set_u16(block + 4, 0x24);
    let object = e
        .call(0x00d4_2790, &args![block, body_a, body_b, manager])
        .u32();
    if e.mem.u32(object + 0x1c) == 0x1d {
        let shape = e.mem.u32(body_a);
        let outer_b = outermost_parent_00d42800(e, body_b);
        let outer_a = outermost_parent_00d42800(e, body_a);
        e.vcall(shape, 0x2c, &args![outer_a, outer_b, input]);
        e.mem.set_u32(object + 0x14, shape);
    }
    if e.mem.u32(object + 0x20) == 0x1d {
        let shape = e.mem.u32(body_b);
        let outer_a = outermost_parent_00d42800(e, body_a);
        let outer_b = outermost_parent_00d42800(e, body_b);
        e.vcall(shape, 0x2c, &args![outer_b, outer_a, input]);
        e.mem.set_u32(object + 0x18, shape);
    }
    object
}

/// The last link of the chain of parent words (at +0xc) that starts at `body`.
fn outermost_parent_00d42800(e: &Engine, body: u32) -> u32 {
    let mut current = body;
    let mut next = e.mem.u32(current + 0xc);
    while next != 0 {
        current = next;
        next = e.mem.u32(current + 0xc);
    }
    current
}
// Translated from 00d42780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. It is the word at
/// `+0xc` of the function record that `00d42790`'s caller registers with the agent registry. Ghidra made no function here; the address is an immediate operand at 00d42990.
pub fn fn_00d42780(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_2790, fn_00d42790(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d4_2960, fn_00d42960(Ptr)),
        entry!(0x00d4_2800, fn_00d42800(u32, u32, u32, u32) -> u32),
        entry!(0x00d4_2780, fn_00d42780()),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// A chain of `length` nodes (`+0xc` links, the last 0) and the first node.
    fn chain(e: &mut Engine, length: u32) -> Vec<u32> {
        let nodes: Vec<u32> = (0..length).map(|_| e.mem.alloc(0x20)).collect();
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 0xc, pair[1]);
        }
        nodes
    }

    #[test]
    fn fn_00d42790_finds_the_ends_of_both_chains() {
        let mut e = Engine::new();
        let first_chain = chain(&mut e, 3);
        let second_chain = chain(&mut e, 1);
        // `*first` and `*second` are objects whose `+0xc` words are copied.
        let first_head = e.mem.alloc(0x20);
        e.mem.set_u32(first_head + 0xc, 0x1111);
        let second_head = e.mem.alloc(0x20);
        e.mem.set_u32(second_head + 0xc, 0x2222);
        e.mem.set_u32(first_chain[0], first_head);
        e.mem.set_u32(second_chain[0], second_head);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x5555);
        let result = e
            .call(
                0x00d4_2790,
                &args![this, first_chain[0], second_chain[0], 0x9999u32],
            )
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x9999);
        assert_eq!(e.mem.u32(this), 0x010c_f238);
        assert_eq!(e.mem.u32(this + 0xc), first_chain[2]);
        assert_eq!(e.mem.u32(this + 0x10), second_chain[0]);
        assert_eq!(e.mem.u32(this + 0x14), 0x5555);
        assert_eq!(e.mem.u32(this + 0x1c), 0x1111);
        assert_eq!(e.mem.u32(this + 0x20), 0x2222);
    }

    #[test]
    fn fn_00d42960_registers_the_record_twice() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..5 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_2960, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let record = [
            0x00d4_2800,
            0x00d4_2730,
            0x00d4_2760,
            0x00d4_2780,
            0x0000_0100,
        ];
        assert_eq!(seen[0][2..4], [0x1d, 0xffff_ffff]);
        assert_eq!(seen[0][4..], record);
        assert_eq!(seen[1][2..4], [0xffff_ffff, 0x1d]);
        assert_eq!(seen[1][4..], record);
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

    /// A collidable with a shape whose virtual function at +0x2c logs its words, and an optional parent.
    fn shaped_body(
        e: &mut Engine,
        parent: u32,
        log: &Rc<RefCell<Vec<Vec<u32>>>>,
        slot_address: u32,
    ) -> (u32, u32) {
        let body = e.mem.alloc(0x20);
        let shape = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x2c, slot_address);
        e.mem.set_u32(shape, vtable);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 0xc, parent);
        let sink = Rc::clone(log);
        e.register_double(slot_address, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (body, shape)
    }

    /// With both type words 0x1d each shape is asked with the outermost parents, in swapped order.
    #[test]
    fn fn_00d42800_asks_both_shapes_with_the_outermost_parents() {
        let (mut e, sizes) = allocator_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let top_a = e.mem.alloc(0x20);
        let middle_a = e.mem.alloc(0x20);
        e.mem.set_u32(middle_a + 0xc, top_a);
        let (body_a, shape_a) = shaped_body(&mut e, middle_a, &log, 0x0900_0007);
        let (body_b, shape_b) = shaped_body(&mut e, 0, &log, 0x0900_0008);
        e.register_double(0x00d4_2790, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, 0x1d);
            e.mem.set_u32(a[0] + 0x20, 0x1d);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e
            .call(0x00d4_2800, &args![body_a, body_b, 0x55u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x24]);
        assert_eq!(e.mem.u16(object + 4), 0x24);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![shape_a, top_a, body_b, 0x55],
                vec![shape_b, body_b, top_a, 0x55],
            ]
        );
        assert_eq!(e.mem.u32(object + 0x14), shape_a);
        assert_eq!(e.mem.u32(object + 0x18), shape_b);
    }

    /// Other type words skip the calls and leave the fields alone; the constructor gets the right words.
    #[test]
    fn fn_00d42800_skips_the_shape_calls_for_other_types() {
        let (mut e, _) = allocator_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let (body_a, _) = shaped_body(&mut e, 0, &log, 0x0900_0007);
        let (body_b, _) = shaped_body(&mut e, 0, &log, 0x0900_0008);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x00d4_2790, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0] + 0x1c, 0x1c);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e
            .call(0x00d4_2800, &args![body_a, body_b, 0x55u32, 0x7777u32])
            .u32();
        assert!(log.borrow().is_empty());
        assert_eq!(*seen.borrow(), vec![vec![object, body_a, body_b, 0x7777]]);
        assert_eq!(e.mem.u32(object + 0x14), 0);
        assert_eq!(e.mem.u32(object + 0x18), 0);
    }
    #[test]
    fn fn_00d42780_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00d4_2780, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
