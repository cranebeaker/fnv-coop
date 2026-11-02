//! `Havok SDK/hkptoicountviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorldViewerBase`'s constructor (`00ddccb0`, `this`, `(world)`).
const VIEWER_BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMapBase<unsigned long, unsigned long>::reserve` (`00cb1fe0`, `this` = the map, `(allocator,
/// count)`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// The process tag word (`01269000`) that `registerViewer` stores, the process factory pointer
/// (`01269cd0`) and `registerProcess` (`00dd9380`, `this` = the factory, `(name, creator)`).
const PROCESS_TAG: u32 = 0x0126_9000;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;

// Translated from 00d67ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiCountViewer` constructor (engine map), `(this, world)`: the base constructor
/// (`00ddccb0`) runs, then the vtables are stored: first `0x0104dd7c` at `+0x28`, `0x010ccf7c`
/// at `+0x2c` and `0x0102f13c` at `+0x30` (all overwritten at once), then `0x010d1e6c` at `+0`,
/// `0x010d1e4c` at `+8`, `0x010d1e3c` at `+0x20`, `0x010d1e1c` at `+0x28`, `0x010d1e0c` at `+0x2c`
/// and `0x010d1df0` at `+0x30`. The map at `+0x34` is emptied (data 0, size 0, capacity word
/// `0xffffffff`) and reserves 0 entries with the router's allocator (`00cb1fe0`). Returns `this`.
pub fn hkp_toi_count_viewer_construct(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let object = this.addr();
    e.call(VIEWER_BASE_CONSTRUCT, &args![this, world]);
    e.mem.set_u32(object + 0x28, 0x0104_dd7c);
    e.mem.set_u32(object + 0x2c, 0x010c_cf7c);
    e.mem.set_u32(object + 0x30, 0x0102_f13c);
    e.mem.set_u32(object, 0x010d_1e6c);
    e.mem.set_u32(object + 8, 0x010d_1e4c);
    e.mem.set_u32(object + 0x20, 0x010d_1e3c);
    e.mem.set_u32(object + 0x28, 0x010d_1e1c);
    e.mem.set_u32(object + 0x2c, 0x010d_1e0c);
    e.mem.set_u32(object + 0x30, 0x010d_1df0);
    let map = object + 0x34;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    this
}

// Translated from 00d67e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiCountViewer::registerViewer` (engine map): registers the process `TOI counter` with the
/// factory (`01269cd0`, creator `00d67d60`) and stores the id it returns at `01269000`.
pub fn hkp_toi_count_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, 0x010d_1de0u32, 0x00d6_7d60u32],
        )
        .u32();
    e.mem.set_u32(PROCESS_TAG, id);
}

// Translated from 00d67d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x40 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d67ce0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d67e16.
pub fn fn_00d67d60(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x40]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let object = e.call(0x00d6_7ce0, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_7ce0, hkp_toi_count_viewer_construct(Ptr, u32) -> Ptr),
        entry!(0x00d6_7e10, hkp_toi_count_viewer_register_viewer()),
        entry!(0x00d6_7d60, fn_00d67d60(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};

    #[test]
    fn the_constructor_stores_the_vtables_and_reserves_an_empty_map() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, VIEWER_BASE_CONSTRUCT, 0);
        log_double(&mut e, &log, MAP_RESERVE, 0);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x7700);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x34, 9);
        e.mem.set_u32(this + 0x38, 9);
        let got = e.call(0x00d6_7ce0, &args![this, 0x4242u32]).u32();
        assert_eq!(got, this);
        assert_eq!(calls(&log, VIEWER_BASE_CONSTRUCT), vec![vec![this, 0x4242]]);
        assert_eq!(calls(&log, MAP_RESERVE), vec![vec![this + 0x34, 0x7700, 0]]);
        let words: Vec<u32> = [0, 8, 0x20, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                0x010d_1e6c,
                0x010d_1e4c,
                0x010d_1e3c,
                0x010d_1e1c,
                0x010d_1e0c,
                0x010d_1df0,
                0,
                0,
                0xffff_ffff
            ]
        );
    }

    #[test]
    fn register_viewer_stores_the_id_the_factory_returns() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REGISTER_PROCESS, 0x4d);
        e.mem.set_u32(PROCESS_FACTORY, 0x5000);
        e.call(0x00d6_7e10, &args![]);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0x5000, 0x010d_1de0, 0x00d6_7d60]]
        );
        assert_eq!(e.mem.u32(PROCESS_TAG), 0x4d);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

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

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00d67d60_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_7ce0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_7d60, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x40]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x40);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d67d60_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_7ce0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_7d60, &args![0x1234u32]).u32(), 0);
    }
}
