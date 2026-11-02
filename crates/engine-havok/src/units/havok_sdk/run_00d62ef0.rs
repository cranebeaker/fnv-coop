//! `Havok SDK/run_00d62ef0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorldViewerBase`'s constructor (Xbox PDB, `00ddccb0`, thiscall, one argument: the process
/// contexts) and destructor (`00ddcc60`, thiscall, no arguments; it answers its `EAX`).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The vtable pointers of the snapshot viewer (RTTI: `hkpWorldSnapshotViewer`): the live ones
/// at `+0`, `+8` and `+0x20`.
const VTABLE_VIEWER: u32 = 0x010d_1720;
const VTABLE_PROCESS: u32 = 0x010d_1700;
const VTABLE_FILTER: u32 = 0x010d_16f0;

// Translated from 00d62ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the viewer whose vtables are `010d1720`, `010d1700` and `010d16f0` (RTTI:
/// `hkpWorldSnapshotViewer`): runs the `hkpWorldViewerBase` constructor (`00ddccb0`, with the
/// process `contexts`), stores the three vtable pointers (`+0`, `+8`, `+0x20`) and the byte
/// `flag` at `+0x28`. Answers `this`.
pub fn fn_00d62ef0(e: &mut Engine, this: Ptr, contexts: Ptr, flag: u8) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem.set_u8(at.wrapping_add(0x28), flag);
    this
}

// Translated from 00d62fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the method at vtable offset 4 of the sub-object at `this + 0x18` once for each world
/// of the container at `[this + 0x1c]` (a count at `+0x70`, an array of world pointers at
/// `+0x6c`; both read again after every call). Does nothing without a container or without
/// worlds.
pub fn fn_00d62fa0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 || e.mem.i32(container.wrapping_add(0x70)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
        e.vcall(at.wrapping_add(0x18), 4, &args![world]);
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
            break;
        }
    }
}

// Translated from 00d62fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the snapshot viewer (`ECX` = `this`): puts the three vtable pointers back
/// and ends in the `hkpWorldViewerBase` destructor (`00ddcc60`), whose `EAX` it answers.
pub fn fn_00d62fe0(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

/// Allocates `size` bytes the way the create functions do: the thread's memory router (`00c85750`)
/// has the allocator at `+0x10`; its virtual slot `4` hands out the block, whose size goes into
/// the 16-bit word at `+4`.
fn allocate_agent_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00d62f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `contexts`; code in a viewer registration function stores its
/// address at `00d63006`): the create function of the snapshot viewer with the flag byte 0.
/// Allocates 0x2c bytes (`allocate_agent_block`), runs the constructor `00d62ef0` on the block
/// with `contexts` and the flag 0 and answers the constructed viewer plus 8 (the sub-object at
/// `+8`), or 0 when the constructor answers null.
pub fn fn_00d62f20(e: &mut Engine, contexts: Ptr) -> u32 {
    let block = allocate_agent_block(e, 0x2c);
    let viewer = fn_00d62ef0(e, Ptr::new(block), contexts, 0);
    if viewer.is_null() {
        return 0;
    }
    viewer.addr().wrapping_add(8)
}

// Translated from 00d62f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `contexts`; code in a viewer registration function stores its
/// address at `00d6301b`): the create function of the snapshot viewer with the flag byte 1, as
/// `00d62f20` otherwise.
pub fn fn_00d62f60(e: &mut Engine, contexts: Ptr) -> u32 {
    let block = allocate_agent_block(e, 0x2c);
    let viewer = fn_00d62ef0(e, Ptr::new(block), contexts, 1);
    if viewer.is_null() {
        return 0;
    }
    viewer.addr().wrapping_add(8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d62f20, fn_00d62f20(Ptr) -> u32),
        entry!(0x00d62f60, fn_00d62f60(Ptr) -> u32),
        entry!(0x00d6_2ef0, fn_00d62ef0(Ptr, Ptr, u8) -> Ptr),
        entry!(0x00d6_2fa0, fn_00d62fa0(Ptr)),
        entry!(0x00d6_2fe0, fn_00d62fe0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod snapshot_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn install_allocator(e: &mut Engine, block: u32) -> Rc<RefCell<Vec<u32>>> {
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0950_0000, &[0, 0x0950_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0950_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0950_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        sizes
    }

    #[test]
    fn fn_00d62f20_makes_a_viewer_with_the_flag_cleared() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_VIEWER_CONSTRUCT, 0);
        let sizes = install_allocator(&mut e, 0x0800_0000);
        e.mem.set_u8(0x0800_0028, 0x7f);
        let viewer = e.call(0x00d6_2f20, &args![0x1234u32]).u32();
        assert_eq!(viewer, 0x0800_0008);
        assert_eq!(*sizes.borrow(), vec![0x2c]);
        assert_eq!(e.mem.u16(0x0800_0004), 0x2c);
        assert_eq!(e.mem.u32(0x0800_0000), VTABLE_VIEWER);
        assert_eq!(e.mem.u8(0x0800_0028), 0);
        assert_eq!(
            *log.borrow(),
            vec![(WORLD_VIEWER_CONSTRUCT, vec![0x0800_0000, 0x1234])]
        );
    }

    #[test]
    fn fn_00d62f60_makes_a_viewer_with_the_flag_set() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_VIEWER_CONSTRUCT, 0);
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let viewer = e.call(0x00d6_2f60, &args![0x1234u32]).u32();
        assert_eq!(viewer, 0x0800_0008);
        assert_eq!(*sizes.borrow(), vec![0x2c]);
        assert_eq!(e.mem.u8(0x0800_0028), 1);
        assert_eq!(e.mem.u32(0x0800_0020), VTABLE_FILTER);
    }

    #[test]
    fn fn_00d62f20_answers_zero_when_the_allocator_gives_no_block() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_VIEWER_CONSTRUCT, 0);
        install_allocator(&mut e, 0);
        e.map(0, 0x100);
        // The constructor still answers `this`, the null block, so the answer is 0.
        assert_eq!(e.call(0x00d6_2f20, &args![0x1234u32]).u32(), 0);
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00d62ef0_stores_the_vtables_and_the_flag_byte() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_VIEWER_CONSTRUCT, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, 0xffff_ffff);
        let got = e
            .call(0x00d6_2ef0, &args![this, 0x1234u32, 0x101u32])
            .ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        // Only the low byte of the stack word is stored, at +0x28.
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(e.mem.u32(this + 0x28), 0xffff_ff01);
        assert_eq!(
            *log.borrow(),
            vec![(WORLD_VIEWER_CONSTRUCT, vec![this, 0x1234])]
        );
    }

    #[test]
    fn fn_00d62fa0_calls_the_slot_for_each_world() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, 0x0900_0011, 0);
        e.map(0x0900_0000, 0x100);
        e.mem.set_u32(0x0900_000c, 0x0900_0011);
        let this = e.mem.alloc(0x40);
        // The sub-object at +0x18 has the vtable at 0x09000008 (slot 4 at 0x0900000c).
        e.mem.set_u32(this + 0x18, 0x0900_0008);
        let container = e.mem.alloc(0x80);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1111);
        e.mem.set_u32(list + 4, 0x2222);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x1c, container);
        e.call(0x00d6_2fa0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0011, vec![this + 0x18, 0x1111]),
                (0x0900_0011, vec![this + 0x18, 0x2222]),
            ]
        );
        // No worlds, or no container: nothing is called.
        log.borrow_mut().clear();
        e.mem.set_u32(container + 0x70, 0);
        e.call(0x00d6_2fa0, &args![this]);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00d6_2fa0, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d62fe0_puts_the_vtables_back_and_answers_the_base_destructor() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_VIEWER_DESTRUCT, 0x77);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00d6_2fe0, &args![this]).u32(), 0x77);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(*log.borrow(), vec![(WORLD_VIEWER_DESTRUCT, vec![this])]);
    }
}
