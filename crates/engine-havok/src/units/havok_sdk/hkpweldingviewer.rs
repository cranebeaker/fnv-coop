//! `Havok SDK/hkpweldingviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d62ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_2720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d62ec0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_2720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d62e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d62ec0`. `00d62ec0` (`RET 4`, which the Xbox PDB names `hkpWeldingViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d62e90(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_2ec0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d62ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d62ec0`. `00d62ec0` (`RET 4`, which the Xbox PDB names `hkpWeldingViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d62ea0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_2ec0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d62eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d62ec0`. `00d62ec0` (`RET 4`, which the Xbox PDB names `hkpWeldingViewer::_vector_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d62eb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_2ec0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

/// The process factory (`.data`) `registerViewer` registers with.
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
/// The welding viewer's process identifier (the result of `registerProcess`, `.data`).
const PROCESS_ID: u32 = 0x0126_8fbc;
/// `hkProcessFactory::registerProcess` (`ECX` the factory, then the name and the creator).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
/// The name `registerViewer` registers (`"WeldingViewer"`) and the creator function it passes.
const WELDING_VIEWER_NAME: u32 = 0x010d_15e0;
const WELDING_VIEWER_CREATOR: u32 = 0x00d6_2e20;
/// An `hkpWorld` function called with an entity in `ECX` and the viewer's sub-object at `+0x28`
/// (roles unconfirmed: the destructor runs it for every entity of the world).
const ENTITY_CALL: u32 = 0x00c9_18e0;
/// `hkpWorldViewerBase::~hkpWorldViewerBase` (`ECX` the object; reached by a tail jump).
const WORLD_VIEWER_BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// The vtables the destructor stores (`.rdata`): `+0`, `+8`, `+0x20`, `+0x28`, then the base
/// table at `+0x28` the destructor ends with.
const VIEWER_VTABLE: u32 = 0x010d_1634;
const VIEWER_VTABLE_8: u32 = 0x010d_1614;
const VIEWER_VTABLE_20: u32 = 0x010d_1604;
const VIEWER_VTABLE_28: u32 = 0x010d_15f4;
const BASE_VTABLE_28: u32 = 0x010c_cf7c;

// Translated from 00d60b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldViewerBase::getConsumableCommands` (Xbox PDB name, engine map). Two stack words
/// (`RET 8`; `ECX` is not read): `first` and `second`, pointers to words that are set to zero.
pub fn hkp_world_viewer_base_get_consumable_commands(
    e: &mut Engine,
    _this: Ptr,
    first: u32,
    second: u32,
) {
    e.mem.set_u32(first, 0);
    e.mem.set_u32(second, 0);
}

// Translated from 00d62720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The welding viewer's destructor (fastcall, no stack word; the engine map has no name for it,
/// and `00d62ec0` calls it): stores the vtables (`+0` `010d1634`, `+8` `010d1614`, `+0x20`
/// `010d1604`, `+0x28` `010d15f4`), then for every entity of the world at `+0x24` (the array at
/// world `+0x6c`, count at `+0x70`; both re-read each time) calls `00c918e0` with the entity
/// and the sub-object `this + 0x28`, puts the base table `010ccf7c` at `+0x28` and ends with a
/// tail jump to the `hkpWorldViewerBase` destructor (`00ddcc60`, `ECX` the object).
pub fn fn_00d62720(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, VIEWER_VTABLE);
    e.mem.set_u32(t + 8, VIEWER_VTABLE_8);
    e.mem.set_u32(t + 0x20, VIEWER_VTABLE_20);
    e.mem.set_u32(t + 0x28, VIEWER_VTABLE_28);
    let world = e.mem.u32(t + 0x24);
    if world != 0 && e.mem.i32(world + 0x70) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(t + 0x24);
            let entities = e.mem.u32(world + 0x6c);
            let entity = e.mem.u32(entities.wrapping_add(index.wrapping_mul(4)));
            e.call(ENTITY_CALL, &args![entity, t + 0x28]);
            let world = e.mem.u32(t + 0x24);
            index += 1;
            if index as i32 >= e.mem.i32(world + 0x70) {
                break;
            }
        }
    }
    e.mem.set_u32(t + 0x28, BASE_VTABLE_28);
    e.call(WORLD_VIEWER_BASE_DESTRUCT, &args![t]);
}

// Translated from 00d62e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWeldingViewer::registerViewer` (Xbox PDB name, engine map): registers the process
/// `"WeldingViewer"` (`010d15e0`) with the creator `00d62e20` in the process factory (the global
/// `01269cd0`, `ECX`; `00dd9380`) and keeps the result in the global `01268fbc`.
pub fn hkp_welding_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let process = e
        .call(
            REGISTER_PROCESS,
            &args![factory, WELDING_VIEWER_NAME, WELDING_VIEWER_CREATOR],
        )
        .u32();
    e.set_global(PROCESS_ID, process);
}

// Translated from 00d62e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the welding viewer's process identifier (the global `01268fbc` that `registerViewer`
/// fills); the engine map has no name for it.
pub fn fn_00d62e80(e: &mut Engine) -> u32 {
    e.global(PROCESS_ID)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_2ec0, deleting_destructor_00d62ec0(Ptr, u8) -> Ptr),
        entry!(0x00d6_2e90, fn_00d62e90(Ptr, u32) -> u32),
        entry!(0x00d6_2ea0, fn_00d62ea0(Ptr, u32) -> u32),
        entry!(0x00d6_2eb0, fn_00d62eb0(Ptr, u32) -> u32),
        entry!(
            0x00d6_0b90,
            hkp_world_viewer_base_get_consumable_commands(Ptr, u32, u32)
        ),
        entry!(0x00d6_2720, fn_00d62720(Ptr)),
        entry!(0x00d6_2e60, hkp_welding_viewer_register_viewer()),
        entry!(0x00d6_2e80, fn_00d62e80() -> u32),
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
    fn deleting_destructor_00d62ec0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_2ec0, 0x00d6_2720);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d62e90() {
        assert_eq!(
            check(0x00d6_2e90, 0x00d6_2ec0, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d62ea0() {
        assert_eq!(
            check(0x00d6_2ea0, 0x00d6_2ec0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d62eb0() {
        assert_eq!(
            check(0x00d6_2eb0, 0x00d6_2ec0, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles that only log their arguments (and return zero) for each address.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn get_consumable_commands_clears_both_words() {
        let mut e = Engine::new();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x1111);
        e.mem.set_u32(second, 0x2222);
        e.call(0x00d6_0b90, &args![0u32, first, second]);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(second), 0);
    }

    #[test]
    fn fn_00d62720_releases_every_entity_and_chains_to_the_base() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_callees(&mut e, &log, &[ENTITY_CALL, WORLD_VIEWER_BASE_DESTRUCT]);
        let this = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x80);
        let entities = e.mem.alloc(8);
        e.mem.set_u32(entities, 0xe1);
        e.mem.set_u32(entities + 4, 0xe2);
        e.mem.set_u32(world + 0x6c, entities);
        e.mem.set_u32(world + 0x70, 2);
        e.mem.set_u32(this + 0x24, world);
        e.call(0x00d6_2720, &args![this]);
        assert_eq!(
            calls_to(&log, ENTITY_CALL),
            vec![vec![0xe1, this + 0x28], vec![0xe2, this + 0x28]]
        );
        assert_eq!(calls_to(&log, WORLD_VIEWER_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x010d_1634);
        assert_eq!(e.mem.u32(this + 8), 0x010d_1614);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_1604);
        assert_eq!(e.mem.u32(this + 0x28), 0x010c_cf7c);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00d6_2720, &args![this]);
        assert!(calls_to(&log, ENTITY_CALL).is_empty());
        assert_eq!(calls_to(&log, WORLD_VIEWER_BASE_DESTRUCT).len(), 1);
    }

    #[test]
    fn register_viewer_keeps_the_process_id() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(PROCESS_FACTORY, 0x1234u32);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            sink.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
        e.call(0x00d6_2e60, &args![]);
        assert_eq!(
            calls_to(&log, REGISTER_PROCESS),
            vec![vec![0x1234, 0x010d_15e0, 0x00d6_2e20]]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x55);
    }

    #[test]
    fn fn_00d62e80_returns_the_process_id() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(PROCESS_ID, 0x66u32);
        assert_eq!(e.call(0x00d6_2e80, &args![]).u32(), 0x66);
    }
}
