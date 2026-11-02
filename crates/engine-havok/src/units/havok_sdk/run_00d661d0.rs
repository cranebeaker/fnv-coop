//! `Havok SDK/run_00d661d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d66220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_6080`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d66220(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_6080, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d661e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d66220`. `00d66220` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d661e0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6220, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d661f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d66220`. `00d66220` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d661f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6220, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d66200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d66220`. `00d66220` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66200(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6220, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d66210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x2c (`SUB ECX,0x2c`) and jumps to `00d66220`. `00d66220` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66210(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6220, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c93860` pushes it on the array at world `+0x134` and `00c917f0` zeroes the first entry of
/// that array that equals it; `00c93920` (Xbox PDB name `hkpWorld::addWorldPostSimulationListener`)
/// pushes it on the array at `+0x164` and `00c918e0` zeroes the first entry of that one that
/// equals it.
const WORLD_ADD_PHANTOM_LISTENER: u32 = 0x00c9_3860;
const WORLD_REMOVE_PHANTOM_LISTENER: u32 = 0x00c9_17f0;
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// `hkpWorldViewerBase`'s constructor (Xbox PDB, `00ddccb0`, thiscall, one argument: the process
/// contexts) and destructor (`00ddcc60`, thiscall, no arguments; it answers its `EAX`).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The process id the shape display viewer was registered under (`01268fe0`, answered by
/// `00d661d0`) and the one the phantom display viewer was registered under (`01268ff0`, kept by
/// `00d66af0` and answered by `00d66b10`, which the removal in `00d66390` hands on).
const SHAPE_VIEWER_PROCESS_ID: u32 = 0x0126_8fe0;
const PHANTOM_REGISTERED_TAG: u32 = 0x0126_8ff0;
/// The vtable pointers of the phantom display viewer (RTTI: `hkpPhantomDisplayViewer`): the
/// live ones at `+0`, `+8`, `+0x20`, `+0x28` and `+0x2c`, and the two listener base class ones
/// (`0104e3d0`: `hkpPhantomListener`; `010ccf7c`: the post-simulation listener's).
const VTABLE_VIEWER: u32 = 0x010d_1bdc;
const VTABLE_PROCESS: u32 = 0x010d_1bbc;
const VTABLE_FILTER: u32 = 0x010d_1bac;
const VTABLE_PHANTOM_LISTENER: u32 = 0x010d_1b94;
const VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_1b84;
const PHANTOM_LISTENER_BASE_VTABLE: u32 = 0x0104_e3d0;
const POST_SIMULATION_LISTENER_BASE_VTABLE: u32 = 0x010c_cf7c;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d661d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id of the shape display viewer's registration (`01268fe0`): slot 1 of the
/// vtable `010d1ae4` (RTTI: `hkpShapeDisplayViewer`); `ECX` unused.
pub fn fn_00d661d0(e: &mut Engine) -> u32 {
    e.global(SHAPE_VIEWER_PROCESS_ID)
}

/// Calls the method at vtable offset `slot` of the sub-object `listener` with every phantom of
/// the array at `world + 0x110` (data; count at `+0x114`, read again each time).
fn notify_phantoms(e: &mut Engine, listener: u32, world: u32, slot: u32) {
    if e.mem.i32(world.wrapping_add(0x114)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let list = e.mem.u32(world.wrapping_add(0x110));
        let phantom = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.vcall(listener, slot, &args![phantom]);
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(world.wrapping_add(0x114)) {
            break;
        }
    }
}

// Translated from 00d66250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the phantom display viewer (`this` may be null) taking a world (the removal
/// side): takes the listeners `this + 0x28` and `this + 0x2c` (0 for a null `this`) away from
/// the world (`00c917f0`, `00c918e0`), then calls the method at vtable offset 8 of the
/// sub-object `this + 0x28` with every phantom of the world's array at `+0x110` (count at
/// `+0x114`).
pub fn fn_00d66250(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    e.call(WORLD_REMOVE_PHANTOM_LISTENER, &args![world, first]);
    let second = if at != 0 { at.wrapping_add(0x2c) } else { 0 };
    e.call(WORLD_REMOVE_POST_SIMULATION_LISTENER, &args![world, second]);
    notify_phantoms(e, at.wrapping_add(0x28), world, 8);
}

// Translated from 00d662c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d66250` for a world coming in: adds the listeners (`00c93860`, `00c93920`) and
/// calls the method at vtable offset 4 of the sub-object `this + 0x28` with every phantom.
pub fn fn_00d662c0(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    e.call(WORLD_ADD_PHANTOM_LISTENER, &args![world, first]);
    let second = if at != 0 { at.wrapping_add(0x2c) } else { 0 };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, second]);
    notify_phantoms(e, at.wrapping_add(0x28), world, 4);
}

// Translated from 00d66330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00d662c0` on `this - 8` once for each world of the container at `[this + 0x1c]` (a
/// count at `+0x70`, an array of world pointers at `+0x6c`; both read again after every call).
/// Does nothing without a container or without worlds.
pub fn fn_00d66330(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 || e.mem.i32(container.wrapping_add(0x70)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
        fn_00d662c0(e, Ptr::new(at.wrapping_sub(8)), Ptr::new(world));
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
            break;
        }
    }
}

// Translated from 00d66370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d66250` with the same stack word.
pub fn fn_00d66370(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d66250(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d66380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d662c0` with the same stack word.
pub fn fn_00d66380(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d662c0(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d66390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `phantom` out of the array of phantoms at `this + 8` (data, size at `+0xc`): the first
/// match is replaced by the last element and the size is lowered; then the object at
/// `[this - 0x10]` gets its method at vtable offset 0x14 called with `phantom + 0x10`, the
/// word at `01268ff0` (the registration `00d66af0` stores) and the word at `phantom + 0x10`.
/// Nothing happens when the array is empty or holds no match.
pub fn fn_00d66390(e: &mut Engine, this: Ptr, phantom: u32) {
    let at = this.addr();
    let size = e.mem.i32(at.wrapping_add(0xc));
    if size <= 0 {
        return;
    }
    let data = e.mem.u32(at.wrapping_add(8));
    let mut index = 0i32;
    loop {
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == phantom {
            break;
        }
        index = index.wrapping_add(1);
        if index >= size {
            return;
        }
    }
    // The search cannot end on a negative index; the exe tests it all the same.
    if index < 0 {
        return;
    }
    let last = e.mem.u32(at.wrapping_add(0xc)).wrapping_sub(1);
    e.mem.set_u32(at.wrapping_add(0xc), last);
    if last as i32 != index {
        let data = e.mem.u32(at.wrapping_add(8));
        let moved = e.mem.u32(data.wrapping_add(last.wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
    let value = e.mem.u32(phantom.wrapping_add(0x10));
    let handler = e.mem.u32(at.wrapping_sub(0x10));
    let tag = e.mem.u32(PHANTOM_REGISTERED_TAG);
    e.vcall(
        handler,
        0x14,
        &args![phantom.wrapping_add(0x10), tag, value],
    );
}

// Translated from 00d66430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the phantom display viewer (RTTI: `hkpPhantomDisplayViewer`, vtables
/// `010d1bdc` and the four after it): runs the `hkpWorldViewerBase` constructor (`00ddccb0`,
/// with the process `contexts`), stores the listener base vtables (`+0x28`, `+0x2c`), the
/// vtables (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`) and an empty array of phantoms at `+0x30`
/// (data, size, capacity word `0x80000000`). Answers `this`.
pub fn fn_00d66430(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem
        .set_u32(at.wrapping_add(0x28), PHANTOM_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem
        .set_u32(at.wrapping_add(0x28), VTABLE_PHANTOM_LISTENER);
    e.mem
        .set_u32(at.wrapping_add(0x2c), VTABLE_POST_SIMULATION_LISTENER);
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x34), 0);
    e.mem.set_u32(at.wrapping_add(0x38), 0x8000_0000);
    this
}

// Translated from 00d66490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the phantom display viewer (`ECX` = `this`): puts its vtable pointers
/// back, and when the viewer has a world container (`+0x24`) with worlds runs `00d66250` on
/// `this` with each of the `[container + 0x70]` entries of the array at `[container + 0x6c]`;
/// empties the array at `+0x30` (freeing its data through the router's allocator, slot `0x10`,
/// when the capacity word at `+0x38` is not negative), stores the listener base vtables
/// (`+0x2c`: post-simulation listener, `+0x28`: phantom listener) and ends in the
/// `hkpWorldViewerBase` destructor (`00ddcc60`), whose `EAX` it answers.
pub fn fn_00d66490(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let container = e.mem.u32(at.wrapping_add(0x24));
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem
        .set_u32(at.wrapping_add(0x28), VTABLE_PHANTOM_LISTENER);
    e.mem
        .set_u32(at.wrapping_add(0x2c), VTABLE_POST_SIMULATION_LISTENER);
    if container != 0 && e.mem.i32(container.wrapping_add(0x70)) > 0 {
        let mut index = 0u32;
        loop {
            let container = e.mem.u32(at.wrapping_add(0x24));
            let worlds = e.mem.u32(container.wrapping_add(0x6c));
            let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
            fn_00d66250(e, this, Ptr::new(world));
            let container = e.mem.u32(at.wrapping_add(0x24));
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = e.mem.u32(at.wrapping_add(0x38));
    e.mem.set_u32(at.wrapping_add(0x34), 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(at.wrapping_add(0x30));
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x38), 0x8000_0000);
    e.mem
        .set_u32(at.wrapping_add(0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x28), PHANTOM_LISTENER_BASE_VTABLE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_61d0, fn_00d661d0() -> u32),
        entry!(0x00d6_6250, fn_00d66250(Ptr, Ptr)),
        entry!(0x00d6_62c0, fn_00d662c0(Ptr, Ptr)),
        entry!(0x00d6_6330, fn_00d66330(Ptr)),
        entry!(0x00d6_6370, fn_00d66370(Ptr, Ptr)),
        entry!(0x00d6_6380, fn_00d66380(Ptr, Ptr)),
        entry!(0x00d6_6390, fn_00d66390(Ptr, u32)),
        entry!(0x00d6_6430, fn_00d66430(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_6490, fn_00d66490(Ptr) -> u32),
        entry!(0x00d6_6220, deleting_destructor_00d66220(Ptr, u8) -> Ptr),
        entry!(0x00d6_61e0, fn_00d661e0(Ptr, u32) -> u32),
        entry!(0x00d6_61f0, fn_00d661f0(Ptr, u32) -> u32),
        entry!(0x00d6_6200, fn_00d66200(Ptr, u32) -> u32),
        entry!(0x00d6_6210, fn_00d66210(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d66220_frees_only_when_bit_zero_is_set() {
        check(0x00d6_6220, 0x00d6_6080);
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
    fn le81_thunk_00d661e0() {
        assert_eq!(
            check(0x00d6_61e0, 0x00d6_6220, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d661f0() {
        assert_eq!(
            check(0x00d6_61f0, 0x00d6_6220, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66200() {
        assert_eq!(
            check(0x00d6_6200, 0x00d6_6220, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66210() {
        assert_eq!(
            check(0x00d6_6210, 0x00d6_6220, 0x2c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod phantom_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ROUTER: u32 = 0x0900_0100;
    const ALLOCATOR: u32 = 0x0900_0120;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0140;
    const FREE_ARRAY: u32 = 0x0900_0203;
    /// A listener sub-object vtable with callbacks in slots 4 and 8, and a display handler
    /// vtable with one in slot 0x14.
    const LISTENER_VTABLE: u32 = 0x0900_0160;
    const LISTENER_ADDED: u32 = 0x0900_0204;
    const LISTENER_REMOVED: u32 = 0x0900_0205;
    const HANDLER: u32 = 0x0900_0180;
    const HANDLER_VTABLE: u32 = 0x0900_01a0;
    const HANDLER_CALL: u32 = 0x0900_0206;

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

    fn rig() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        let log: Log = Rc::default();
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, FREE_ARRAY);
        e.mem.set_u32(LISTENER_VTABLE + 4, LISTENER_ADDED);
        e.mem.set_u32(LISTENER_VTABLE + 8, LISTENER_REMOVED);
        e.mem.set_u32(HANDLER, HANDLER_VTABLE);
        e.mem.set_u32(HANDLER_VTABLE + 0x14, HANDLER_CALL);
        record(&mut e, &log, MEMORY_ROUTER, ROUTER);
        for address in [
            FREE_ARRAY,
            LISTENER_ADDED,
            LISTENER_REMOVED,
            HANDLER_CALL,
            WORLD_ADD_PHANTOM_LISTENER,
            WORLD_REMOVE_PHANTOM_LISTENER,
            WORLD_ADD_POST_SIMULATION_LISTENER,
            WORLD_REMOVE_POST_SIMULATION_LISTENER,
            WORLD_VIEWER_CONSTRUCT,
        ] {
            record(&mut e, &log, address, 0);
        }
        record(&mut e, &log, WORLD_VIEWER_DESTRUCT, 0x99);
        (e, log)
    }

    fn calls(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a != MEMORY_ROUTER)
            .cloned()
            .collect()
    }

    /// A viewer whose sub-object at `+0x28` has the callback vtable.
    fn viewer(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, LISTENER_VTABLE);
        this
    }

    /// A world with the phantoms `phantoms` (array at `+0x110`, count at `+0x114`).
    fn world(e: &mut Engine, phantoms: &[u32]) -> u32 {
        let world = e.mem.alloc(0x120);
        let data = e.mem.alloc(4 * phantoms.len() as u32 + 4);
        for (i, p) in phantoms.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *p);
        }
        e.mem.set_u32(world + 0x110, data);
        e.mem.set_u32(world + 0x114, phantoms.len() as u32);
        world
    }

    #[test]
    fn fn_00d661d0_answers_the_registered_process_id() {
        let (mut e, _) = rig();
        e.set_global(SHAPE_VIEWER_PROCESS_ID, 0x4242u32);
        assert_eq!(e.call(0x00d6_61d0, &args![]).u32(), 0x4242);
    }

    #[test]
    fn fn_00d66250_removes_the_listeners_and_notifies_slot_8() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        let world = world(&mut e, &[0x11, 0x22]);
        e.call(0x00d6_6250, &args![this, world]);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_PHANTOM_LISTENER, vec![world, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![world, this + 0x2c]
                ),
                (LISTENER_REMOVED, vec![this + 0x28, 0x11]),
                (LISTENER_REMOVED, vec![this + 0x28, 0x22]),
            ]
        );
        // No phantoms: only the two removals.
        log.borrow_mut().clear();
        let empty = self::world(&mut e, &[]);
        e.call(0x00d6_6250, &args![this, empty]);
        assert_eq!(calls(&log).len(), 2);
        // A null viewer hands 0 to both.
        log.borrow_mut().clear();
        e.call(0x00d6_6250, &args![0u32, empty]);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_PHANTOM_LISTENER, vec![empty, 0]),
                (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![empty, 0]),
            ]
        );
    }

    #[test]
    fn fn_00d662c0_adds_the_listeners_and_notifies_slot_4() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        let world = world(&mut e, &[0x11, 0x22]);
        e.call(0x00d6_62c0, &args![this, world]);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_ADD_PHANTOM_LISTENER, vec![world, this + 0x28]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![world, this + 0x2c]),
                (LISTENER_ADDED, vec![this + 0x28, 0x11]),
                (LISTENER_ADDED, vec![this + 0x28, 0x22]),
            ]
        );
    }

    #[test]
    fn fn_00d66330_adds_every_world_on_this_minus_8() {
        let (mut e, log) = rig();
        let viewer_base = viewer(&mut e);
        let this = viewer_base + 8;
        let first = world(&mut e, &[]);
        let second = world(&mut e, &[]);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x1c, container);
        e.call(0x00d6_6330, &args![this]);
        let adds: Vec<_> = calls(&log)
            .into_iter()
            .filter(|(a, _)| *a == WORLD_ADD_PHANTOM_LISTENER)
            .collect();
        assert_eq!(
            adds,
            vec![
                (WORLD_ADD_PHANTOM_LISTENER, vec![first, viewer_base + 0x28]),
                (WORLD_ADD_PHANTOM_LISTENER, vec![second, viewer_base + 0x28]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(container + 0x70, 0);
        e.call(0x00d6_6330, &args![this]);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00d6_6330, &args![this]);
        assert!(calls(&log).is_empty());
    }

    #[test]
    fn fn_00d66370_and_00d66380_run_the_callbacks_on_this_minus_0x20() {
        let (mut e, log) = rig();
        let base = viewer(&mut e);
        let world = world(&mut e, &[]);
        e.call(0x00d6_6370, &args![base + 0x20, world]);
        e.call(0x00d6_6380, &args![base + 0x20, world]);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_PHANTOM_LISTENER, vec![world, base + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![world, base + 0x2c]
                ),
                (WORLD_ADD_PHANTOM_LISTENER, vec![world, base + 0x28]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![world, base + 0x2c]),
            ]
        );
    }

    /// The listener sub-object at `this`: array of phantoms at `+8`, a display handler at
    /// `this - 0x10`.
    fn phantom_list(e: &mut Engine, phantoms: &[u32]) -> u32 {
        let block = e.mem.alloc(0x40);
        let this = block + 0x10;
        let data = e.mem.alloc(4 * phantoms.len() as u32 + 4);
        for (i, p) in phantoms.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *p);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, phantoms.len() as u32);
        e.mem.set_u32(this - 0x10, HANDLER);
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let data = e.mem.u32(this + 8);
        (0..e.mem.u32(this + 0xc))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00d66390_swaps_the_last_phantom_in_and_tells_the_display_handler() {
        let (mut e, log) = rig();
        e.set_global(PHANTOM_REGISTERED_TAG, 0x55u32);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x10, 0x71);
        e.mem.set_u32(second + 0x10, 0x72);
        let this = phantom_list(&mut e, &[first, second, third]);
        e.call(0x00d6_6390, &args![this, first]);
        assert_eq!(items(&e, this), vec![third, second]);
        assert_eq!(
            calls(&log),
            vec![(HANDLER_CALL, vec![HANDLER, first + 0x10, 0x55, 0x71])]
        );
        // The last one goes by lowering the size only.
        log.borrow_mut().clear();
        e.call(0x00d6_6390, &args![this, second]);
        assert_eq!(items(&e, this), vec![third]);
        assert_eq!(
            calls(&log),
            vec![(HANDLER_CALL, vec![HANDLER, second + 0x10, 0x55, 0x72])]
        );
        // Not in the array, or an empty array: nothing happens.
        log.borrow_mut().clear();
        e.call(0x00d6_6390, &args![this, first]);
        let empty = phantom_list(&mut e, &[]);
        e.call(0x00d6_6390, &args![empty, first]);
        assert!(calls(&log).is_empty());
        assert_eq!(items(&e, this), vec![third]);
    }

    #[test]
    fn fn_00d66430_sets_up_the_phantom_viewer() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x60);
        let got = e.call(0x00d6_6430, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), VTABLE_PHANTOM_LISTENER);
        assert_eq!(e.mem.u32(this + 0x2c), VTABLE_POST_SIMULATION_LISTENER);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(
            calls(&log),
            vec![(WORLD_VIEWER_CONSTRUCT, vec![this, 0x1234])]
        );
    }

    #[test]
    fn fn_00d66490_removes_every_world_and_frees_the_array() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        let first = world(&mut e, &[]);
        let second = world(&mut e, &[]);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x24, container);
        let data = e.mem.alloc(16);
        e.mem.set_u32(this + 0x30, data);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 4);
        let got = e.call(0x00d6_6490, &args![this]).u32();
        assert_eq!(got, 0x99);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_PHANTOM_LISTENER, vec![first, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![first, this + 0x2c]
                ),
                (WORLD_REMOVE_PHANTOM_LISTENER, vec![second, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![second, this + 0x2c]
                ),
                (FREE_ARRAY, vec![ALLOCATOR, data, 16]),
                (WORLD_VIEWER_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 0x28), PHANTOM_LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn fn_00d66490_without_a_container_keeps_a_borrowed_array() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 0x8000_0004);
        e.call(0x00d6_6490, &args![this]);
        assert_eq!(calls(&log), vec![(WORLD_VIEWER_DESTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
    }
}
