//! `Havok SDK/run_00d66ed0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d66f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_6cd0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d66f00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_6cd0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d66ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d66f00`. `00d66f00` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66ee0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6f00, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d66ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d66f00`. `00d66f00` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66ef0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6f00, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

use super::run_00d51760::push_word_with;
use super::run_00d64650::replay_entities;

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c93820` pushes it on the array at world `+0x128` and `00c917a0` zeroes the first entry of
/// that array that equals it; `00c93920` (Xbox PDB name `hkpWorld::addWorldPostSimulationListener`)
/// pushes it on the array at `+0x164` and `00c918e0` zeroes the first entry of that one that
/// equals it.
const WORLD_ADD_ISLAND_LISTENER: u32 = 0x00c9_3820;
const WORLD_REMOVE_ISLAND_LISTENER: u32 = 0x00c9_17a0;
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// `hkpWorldViewerBase`'s constructor (Xbox PDB, `00ddccb0`, thiscall, one argument: the process
/// contexts) and destructor (`00ddcc60`, thiscall, no arguments; it answers its `EAX`).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The id the mouse picking viewer's process was registered under (`01268ff8`).
const MOUSE_PICKING_PROCESS_ID: u32 = 0x0126_8ff8;
/// The vtable pointers of the rigid body inertia viewer (RTTI: `hkpRigidBodyInertiaViewer`): the
/// live ones at `+0`, `+8`, `+0x20`, `+0x28` and `+0x2c`, and the two listener base class ones
/// (`0102f13c`: `hkpEntityListener`; `010ccf7c`: the post-simulation listener's).
const INERTIA_VTABLE_VIEWER: u32 = 0x010d_1d80;
const INERTIA_VTABLE_PROCESS: u32 = 0x010d_1d60;
const INERTIA_VTABLE_FILTER: u32 = 0x010d_1d50;
const INERTIA_VTABLE_ENTITY_LISTENER: u32 = 0x010d_1d34;
const INERTIA_VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_1d24;
const ENTITY_LISTENER_BASE_VTABLE: u32 = 0x0102_f13c;
const POST_SIMULATION_LISTENER_BASE_VTABLE: u32 = 0x010c_cf7c;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d66ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01268ff8`: slot 1 of the vtable `010d1cc4` (RTTI:
/// `hkpMousePickingViewer`); `ECX` unused.
pub fn fn_00d66ed0(e: &mut Engine) -> u32 {
    e.global(MOUSE_PICKING_PROCESS_ID)
}

/// Calls the method at vtable offset `slot` of the sub-object `listener` with every entity of
/// every island of the world: the islands of the array at `world + 0x28` (count at `+0x2c`),
/// of the one at `world + 0x34` (count at `+0x38`) and the island at `world + 0x20` when it is
/// not null (the entity array of an island is at `+0x48`; counts are read again each time).
fn visit_islands(e: &mut Engine, listener: u32, world: u32, slot: u32) {
    for (list, count) in [(0x28u32, 0x2cu32), (0x34, 0x38)] {
        let mut index = 0u32;
        if e.mem.i32(world.wrapping_add(count)) > 0 {
            loop {
                let islands = e.mem.u32(world.wrapping_add(list));
                let island = e.mem.u32(islands.wrapping_add(index.wrapping_mul(4)));
                replay_entities(e, listener, island, slot);
                index = index.wrapping_add(1);
                if (index as i32) >= e.mem.i32(world.wrapping_add(count)) {
                    break;
                }
            }
        }
    }
    let island = e.mem.u32(world.wrapping_add(0x20));
    if island != 0 {
        replay_entities(e, listener, island, slot);
    }
}

// Translated from 00d66f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the rigid body viewers' shape taking a world (the removal side; `this` may be
/// null): takes the listeners `this + 0x28` and `this + 0x2c` (0 for a null `this`) away from
/// the world (`00c917a0`, `00c918e0`), then calls the method at vtable offset 8 of the
/// sub-object `this + 0x28` with every entity of every island of the world (the two island
/// arrays at `world + 0x28` and `world + 0x34`, and the island at `world + 0x20` if it is not
/// null).
pub fn fn_00d66f30(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    e.call(WORLD_REMOVE_ISLAND_LISTENER, &args![world, first]);
    let second = if at != 0 { at.wrapping_add(0x2c) } else { 0 };
    e.call(WORLD_REMOVE_POST_SIMULATION_LISTENER, &args![world, second]);
    visit_islands(e, at.wrapping_add(0x28), world, 8);
}

// Translated from 00d67020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d66f30` for a world coming in: adds the listeners (`00c93820`, `00c93920`) and calls
/// the method at vtable offset 4 of the sub-object `this + 0x28` with every entity.
pub fn fn_00d67020(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    e.call(WORLD_ADD_ISLAND_LISTENER, &args![world, first]);
    let second = if at != 0 { at.wrapping_add(0x2c) } else { 0 };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, second]);
    visit_islands(e, at.wrapping_add(0x28), world, 4);
}

// Translated from 00d67110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00d67020` on `this - 8` once for each world of the container at `[this + 0x1c]` (an
/// array of world pointers at `+0x6c`; the count at `+0x70` is read once). Does nothing without
/// a container or without worlds.
pub fn fn_00d67110(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 {
        return;
    }
    let count = e.mem.i32(container.wrapping_add(0x70));
    let mut index = 0i32;
    if count <= index {
        return;
    }
    loop {
        let container = e.mem.u32(at.wrapping_add(0x1c));
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e
            .mem
            .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
        fn_00d67020(e, Ptr::new(at.wrapping_sub(8)), Ptr::new(world));
        index = index.wrapping_add(1);
        if index >= count {
            break;
        }
    }
}

// Translated from 00d67150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d66f30` with the same stack word.
pub fn fn_00d67150(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d66f30(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d67160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d67020` with the same stack word.
pub fn fn_00d67160(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d67020(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d67170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `entity` from the array of entities at `this + 8` (data, size at `+0xc`): the first
/// match is replaced by the last element and the size is lowered. Nothing happens when the
/// array is empty or holds no match.
pub fn fn_00d67170(e: &mut Engine, this: Ptr, entity: u32) {
    let at = this.addr();
    let size = e.mem.i32(at.wrapping_add(0xc));
    if size <= 0 {
        return;
    }
    let data = e.mem.u32(at.wrapping_add(8));
    let mut index = 0i32;
    loop {
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == entity {
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
}

// Translated from 00d671b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `entity` to the array of entities at `this + 8` (data, size at `+0xc`, capacity word
/// at `+0x10`), growing it through the router's allocator (slot `0x10`) when it is full.
pub fn fn_00d671b0(e: &mut Engine, this: Ptr, entity: u32) {
    push_word_with(e, this.addr().wrapping_add(8), entity, 0x10);
}

// Translated from 00d671f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the rigid body inertia viewer (RTTI: `hkpRigidBodyInertiaViewer`, vtables
/// `010d1d80` and the four after it): runs the `hkpWorldViewerBase` constructor (`00ddccb0`,
/// with the process `contexts`), stores the listener base vtables (`+0x28`, `+0x2c`), the
/// vtables (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`) and two empty arrays (`+0x30` and `+0x3c`:
/// data, size, capacity word `0x80000000`). Answers `this`.
pub fn fn_00d671f0(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem
        .set_u32(at.wrapping_add(0x28), ENTITY_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem.set_u32(at, INERTIA_VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), INERTIA_VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), INERTIA_VTABLE_FILTER);
    e.mem
        .set_u32(at.wrapping_add(0x28), INERTIA_VTABLE_ENTITY_LISTENER);
    e.mem.set_u32(
        at.wrapping_add(0x2c),
        INERTIA_VTABLE_POST_SIMULATION_LISTENER,
    );
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x34), 0);
    e.mem.set_u32(at.wrapping_add(0x38), 0x8000_0000);
    e.mem.set_u32(at.wrapping_add(0x3c), 0);
    e.mem.set_u32(at.wrapping_add(0x40), 0);
    e.mem.set_u32(at.wrapping_add(0x44), 0x8000_0000);
    this
}

// Translated from 00d67250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the rigid body inertia viewer (`ECX` = `this`): puts its vtable pointers
/// back, and when the viewer has a world container (`+0x24`) runs `00d66f30` on `this` with
/// each of the `[container + 0x70]` entries (read once) of the array at `[container + 0x6c]`;
/// then destroys the `[this + 0x40]` elements of 0x70 bytes at `[this + 0x3c]` (the method at
/// vtable offset 0 of each, with 0 as its one argument), frees that array through the router's
/// allocator (slot `0x10`, `(capacity & 0x3fffffff) * 0x70` bytes) unless the capacity word at
/// `+0x44` has bit 31 set, does the same for the array of words at `+0x30` (capacity word
/// `+0x38`, 4 bytes per element; the router is asked again), stores the listener base vtables
/// (`+0x2c`: post-simulation listener, `+0x28`: entity listener) and ends in the
/// `hkpWorldViewerBase` destructor (`00ddcc60`), whose `EAX` it answers.
pub fn fn_00d67250(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let container = e.mem.u32(at.wrapping_add(0x24));
    e.mem.set_u32(at, INERTIA_VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), INERTIA_VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), INERTIA_VTABLE_FILTER);
    e.mem
        .set_u32(at.wrapping_add(0x28), INERTIA_VTABLE_ENTITY_LISTENER);
    e.mem.set_u32(
        at.wrapping_add(0x2c),
        INERTIA_VTABLE_POST_SIMULATION_LISTENER,
    );
    if container != 0 {
        let count = e.mem.i32(container.wrapping_add(0x70));
        let mut index = 0i32;
        if count > 0 {
            loop {
                let container = e.mem.u32(at.wrapping_add(0x24));
                let worlds = e.mem.u32(container.wrapping_add(0x6c));
                let world = e
                    .mem
                    .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
                fn_00d66f30(e, this, Ptr::new(world));
                index = index.wrapping_add(1);
                if index >= count {
                    break;
                }
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let elements = e.mem.i32(at.wrapping_add(0x40));
    let mut element = e.mem.u32(at.wrapping_add(0x3c));
    if elements > 0 {
        for _ in 0..elements {
            e.vcall(element, 0, &args![0u32]);
            element = element.wrapping_add(0x70);
        }
    }
    let capacity = e.mem.u32(at.wrapping_add(0x44));
    e.mem.set_u32(at.wrapping_add(0x40), 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(at.wrapping_add(0x3c));
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(0x70);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at.wrapping_add(0x3c), 0);
    e.mem.set_u32(at.wrapping_add(0x44), 0x8000_0000);
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
        .set_u32(at.wrapping_add(0x28), ENTITY_LISTENER_BASE_VTABLE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_6ed0, fn_00d66ed0() -> u32),
        entry!(0x00d6_6f30, fn_00d66f30(Ptr, Ptr)),
        entry!(0x00d6_7020, fn_00d67020(Ptr, Ptr)),
        entry!(0x00d6_7110, fn_00d67110(Ptr)),
        entry!(0x00d6_7150, fn_00d67150(Ptr, Ptr)),
        entry!(0x00d6_7160, fn_00d67160(Ptr, Ptr)),
        entry!(0x00d6_7170, fn_00d67170(Ptr, u32)),
        entry!(0x00d6_71b0, fn_00d671b0(Ptr, u32)),
        entry!(0x00d6_71f0, fn_00d671f0(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_7250, fn_00d67250(Ptr) -> u32),
        entry!(0x00d6_6f00, deleting_destructor_00d66f00(Ptr, u8) -> Ptr),
        entry!(0x00d6_6ee0, fn_00d66ee0(Ptr, u32) -> u32),
        entry!(0x00d6_6ef0, fn_00d66ef0(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d66f00_frees_only_when_bit_zero_is_set() {
        check(0x00d6_6f00, 0x00d6_6cd0);
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
    fn le81_thunk_00d66ee0() {
        assert_eq!(
            check(0x00d6_6ee0, 0x00d6_6f00, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66ef0() {
        assert_eq!(
            check(0x00d6_6ef0, 0x00d6_6f00, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod inertia_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ROUTER: u32 = 0x0900_0100;
    const ALLOCATOR: u32 = 0x0900_0120;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0140;
    const FREE_ARRAY: u32 = 0x0900_0203;
    const LISTENER_VTABLE: u32 = 0x0900_0160;
    const LISTENER_ADDED: u32 = 0x0900_0204;
    const LISTENER_REMOVED: u32 = 0x0900_0205;
    const ELEMENT_VTABLE: u32 = 0x0900_0180;
    const ELEMENT_DESTRUCTOR: u32 = 0x0900_0206;

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
        e.mem.set_u32(ELEMENT_VTABLE, ELEMENT_DESTRUCTOR);
        record(&mut e, &log, MEMORY_ROUTER, ROUTER);
        for address in [
            FREE_ARRAY,
            LISTENER_ADDED,
            LISTENER_REMOVED,
            ELEMENT_DESTRUCTOR,
            WORLD_ADD_ISLAND_LISTENER,
            WORLD_REMOVE_ISLAND_LISTENER,
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

    fn viewer(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, LISTENER_VTABLE);
        this
    }

    fn island(e: &mut Engine, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(4 * entities.len() as u32 + 4);
        for (i, v) in entities.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        island
    }

    /// A world with one island at `+0x20` (when `single` is given), two in the array at `+0x28`
    /// and one in the array at `+0x34`.
    fn world(e: &mut Engine, single: Option<&[u32]>) -> u32 {
        let world = e.mem.alloc(0x80);
        let first = island(e, &[0x11]);
        let second = island(e, &[0x21, 0x22]);
        let third = island(e, &[0x41]);
        let list_a = e.mem.alloc(8);
        e.mem.set_u32(list_a, first);
        e.mem.set_u32(list_a + 4, second);
        let list_b = e.mem.alloc(4);
        e.mem.set_u32(list_b, third);
        if let Some(entities) = single {
            let single = island(e, entities);
            e.mem.set_u32(world + 0x20, single);
        }
        e.mem.set_u32(world + 0x28, list_a);
        e.mem.set_u32(world + 0x2c, 2);
        e.mem.set_u32(world + 0x34, list_b);
        e.mem.set_u32(world + 0x38, 1);
        world
    }

    #[test]
    fn fn_00d66ed0_answers_the_registered_process_id() {
        let (mut e, _) = rig();
        e.set_global(MOUSE_PICKING_PROCESS_ID, 0x4242u32);
        assert_eq!(e.call(0x00d6_6ed0, &args![]).u32(), 0x4242);
    }

    #[test]
    fn fn_00d66f30_removes_the_listeners_and_notifies_slot_8_for_every_entity() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        let world = world(&mut e, Some(&[0x31, 0x32]));
        e.call(0x00d6_6f30, &args![this, world]);
        let listener = this + 0x28;
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_ISLAND_LISTENER, vec![world, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![world, this + 0x2c]
                ),
                (LISTENER_REMOVED, vec![listener, 0x11]),
                (LISTENER_REMOVED, vec![listener, 0x21]),
                (LISTENER_REMOVED, vec![listener, 0x22]),
                (LISTENER_REMOVED, vec![listener, 0x41]),
                (LISTENER_REMOVED, vec![listener, 0x31]),
                (LISTENER_REMOVED, vec![listener, 0x32]),
            ]
        );
    }

    #[test]
    fn fn_00d67020_adds_the_listeners_and_notifies_slot_4_and_skips_a_null_island() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        // No island at world + 0x20: that part is skipped (the exe tests for null here).
        let world = world(&mut e, None);
        e.call(0x00d6_7020, &args![this, world]);
        let listener = this + 0x28;
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_ADD_ISLAND_LISTENER, vec![world, this + 0x28]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![world, this + 0x2c]),
                (LISTENER_ADDED, vec![listener, 0x11]),
                (LISTENER_ADDED, vec![listener, 0x21]),
                (LISTENER_ADDED, vec![listener, 0x22]),
                (LISTENER_ADDED, vec![listener, 0x41]),
            ]
        );
    }

    #[test]
    fn fn_00d67110_adds_every_world_on_this_minus_8_with_the_count_read_once() {
        let (mut e, log) = rig();
        let viewer_base = viewer(&mut e);
        let this = viewer_base + 8;
        let first = world(&mut e, None);
        let second = world(&mut e, None);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x1c, container);
        e.call(0x00d6_7110, &args![this]);
        let adds: Vec<_> = calls(&log)
            .into_iter()
            .filter(|(a, _)| *a == WORLD_ADD_ISLAND_LISTENER)
            .collect();
        assert_eq!(
            adds,
            vec![
                (WORLD_ADD_ISLAND_LISTENER, vec![first, viewer_base + 0x28]),
                (WORLD_ADD_ISLAND_LISTENER, vec![second, viewer_base + 0x28]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(container + 0x70, 0);
        e.call(0x00d6_7110, &args![this]);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00d6_7110, &args![this]);
        assert!(calls(&log).is_empty());
    }

    #[test]
    fn fn_00d67150_and_00d67160_run_the_callbacks_on_this_minus_0x20() {
        let (mut e, log) = rig();
        let base = viewer(&mut e);
        let world = world(&mut e, None);
        e.mem.set_u32(world + 0x2c, 0);
        e.mem.set_u32(world + 0x38, 0);
        e.call(0x00d6_7150, &args![base + 0x20, world]);
        e.call(0x00d6_7160, &args![base + 0x20, world]);
        assert_eq!(
            calls(&log),
            vec![
                (WORLD_REMOVE_ISLAND_LISTENER, vec![world, base + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![world, base + 0x2c]
                ),
                (WORLD_ADD_ISLAND_LISTENER, vec![world, base + 0x28]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![world, base + 0x2c]),
            ]
        );
    }

    fn entity_array(e: &mut Engine, values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, values.len() as u32);
        e.mem.set_u32(this + 0x10, values.len() as u32 + 1);
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let data = e.mem.u32(this + 8);
        (0..e.mem.u32(this + 0xc))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00d67170_swaps_the_last_entity_into_the_hole() {
        let (mut e, _) = rig();
        let this = entity_array(&mut e, &[10, 20, 30]);
        e.call(0x00d6_7170, &args![this, 10u32]);
        assert_eq!(items(&e, this), vec![30, 20]);
        e.call(0x00d6_7170, &args![this, 20u32]);
        assert_eq!(items(&e, this), vec![30]);
        e.call(0x00d6_7170, &args![this, 99u32]);
        assert_eq!(items(&e, this), vec![30]);
        let empty = entity_array(&mut e, &[]);
        e.call(0x00d6_7170, &args![empty, 10u32]);
        assert_eq!(items(&e, empty), Vec::<u32>::new());
    }

    #[test]
    fn fn_00d671b0_appends_and_grows_when_full() {
        let (mut e, log) = rig();
        let this = entity_array(&mut e, &[10]);
        // Capacity 2, size 1: room for one more.
        e.call(0x00d6_71b0, &args![this, 20u32]);
        assert_eq!(items(&e, this), vec![10, 20]);
        assert!(calls(&log).is_empty());
        // Now full: the growth routine runs first (the double makes room for 8).
        e.register_double(0x00c9_0940, {
            let log = Rc::clone(&log);
            move |e, a| {
                log.borrow_mut().push((0x00c9_0940, a.to_vec()));
                let array = a[1];
                let data = e.mem.alloc(32);
                for i in 0..e.mem.u32(array + 4) {
                    let word = e.mem.u32(e.mem.u32(array) + 4 * i);
                    e.mem.set_u32(data + 4 * i, word);
                }
                e.mem.set_u32(array, data);
                e.mem.set_u32(array + 8, 8);
                Ret::default()
            }
        });
        e.call(0x00d6_71b0, &args![this, 30u32]);
        assert_eq!(items(&e, this), vec![10, 20, 30]);
        assert_eq!(
            calls(&log),
            vec![(0x00c9_0940, vec![ALLOCATOR, this + 8, 4])]
        );
    }

    #[test]
    fn fn_00d671f0_sets_up_the_inertia_viewer() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x60);
        let got = e.call(0x00d6_71f0, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), INERTIA_VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), INERTIA_VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), INERTIA_VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), INERTIA_VTABLE_ENTITY_LISTENER);
        assert_eq!(
            e.mem.u32(this + 0x2c),
            INERTIA_VTABLE_POST_SIMULATION_LISTENER
        );
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x44), 0x8000_0000);
        assert_eq!(
            calls(&log),
            vec![(WORLD_VIEWER_CONSTRUCT, vec![this, 0x1234])]
        );
    }

    #[test]
    fn fn_00d67250_removes_the_worlds_destroys_the_elements_and_frees_both_arrays() {
        let (mut e, log) = rig();
        let this = viewer(&mut e);
        let first = world(&mut e, None);
        // No islands: the removal only takes the listeners away.
        e.mem.set_u32(first + 0x2c, 0);
        e.mem.set_u32(first + 0x38, 0);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, first);
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x70, 1);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x24, container);
        // Two elements of 0x70 bytes with a vtable in their first word.
        let elements = e.mem.alloc(0xe0);
        e.mem.set_u32(elements, ELEMENT_VTABLE);
        e.mem.set_u32(elements + 0x70, ELEMENT_VTABLE);
        e.mem.set_u32(this + 0x3c, elements);
        e.mem.set_u32(this + 0x40, 2);
        e.mem.set_u32(this + 0x44, 3);
        let words = e.mem.alloc(16);
        e.mem.set_u32(this + 0x30, words);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 4);
        let got = e.call(0x00d6_7250, &args![this]).u32();
        assert_eq!(got, 0x99);
        let seen = calls(&log);
        // The world removal comes first (its listener calls and the entities).
        assert_eq!(seen[0].0, WORLD_REMOVE_ISLAND_LISTENER);
        let tail: Vec<_> = seen
            .iter()
            .skip_while(|(a, _)| *a != ELEMENT_DESTRUCTOR)
            .cloned()
            .collect();
        assert_eq!(
            tail,
            vec![
                (ELEMENT_DESTRUCTOR, vec![elements, 0]),
                (ELEMENT_DESTRUCTOR, vec![elements + 0x70, 0]),
                (FREE_ARRAY, vec![ALLOCATOR, elements, 3 * 0x70]),
                (FREE_ARRAY, vec![ALLOCATOR, words, 16]),
                (WORLD_VIEWER_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), INERTIA_VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 0x28), ENTITY_LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
        for offset in [0x30, 0x34, 0x3c, 0x40] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x44), 0x8000_0000);
    }

    #[test]
    fn fn_00d67250_keeps_borrowed_arrays() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 0x8000_0004);
        e.mem.set_u32(this + 0x44, 0x8000_0002);
        e.call(0x00d6_7250, &args![this]);
        assert_eq!(calls(&log), vec![(WORLD_VIEWER_DESTRUCT, vec![this])]);
    }
}
