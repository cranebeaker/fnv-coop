//! `Havok SDK/run_00d68e60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d691d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_90d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d691d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_90d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d69190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d691d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d69190(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_91d0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d691a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d691d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d691a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_91d0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d691b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d691d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d691b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_91d0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d691c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x2c` (`SUB ECX,0x2c`) and jumps to `00d691d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d691c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_91d0, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

/// The allocator behind the memory router: `*(router + 0x10)`.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// `hkArray` growth (`00c90940`, cdecl: the allocator, the array, the element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// A listener-array add on the world (`hkpworld.cpp`): `this` the world, one word, the
/// listener subobject at `this + 0x28`. Its name is not confirmed (the engine map has none).
const WORLD_ADD_LISTENER_FIRST: u32 = 0x00c9_3820;
/// `hkpWorld::addWorldPostSimulationListener` (engine map name): one word, the listener.
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
/// The matching removal for the listener added by `00c93820` (name not confirmed).
const WORLD_REMOVE_LISTENER_FIRST: u32 = 0x00c9_17a0;
/// The matching removal for the listener added by `00c93920` (name not confirmed).
const WORLD_REMOVE_LISTENER_SECOND: u32 = 0x00c9_18e0;
/// `00d68a70` (`hkpconvexradiusviewer.cpp`): looks a world up in the viewer's world list.
const FIND_WORLD: u32 = 0x00d6_8a70;
/// `00d68b20` (`hkpconvexradiusviewer.cpp`): the per-world clean-up before a record goes.
const REMOVE_WORLD_GEOMETRIES: u32 = 0x00d6_8b20;
/// The base destructor tail-called by `00d690d0` (`hkpworldviewerbase.cpp`).
const WORLD_VIEWER_BASE_DESTRUCTOR: u32 = 0x00dd_cc60;

/// Calls the vtable slot at byte offset 4 of `listener` with each entity of the pointer array
/// at `entity_array` (data pointer at `+0`, count at `+4`). The data pointer and the count are
/// read again on every pass, as the exe does, because the callback may change them.
fn notify_each_entity(e: &mut Engine, listener: u32, entity_array: u32) {
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(entity_array + 4) {
        let data = e.mem.u32(entity_array);
        let entity = e.mem.u32(data + index * 4);
        e.vcall(listener, 4, &args![entity]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00d68e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::addWorld` (Xbox PDB name by its shape; the exe's RTTI confirms the
/// class of the vtable at `010d20f8`, the engine map names nothing): registers the listener subobjects at `this + 0x28` and
/// `this + 0x2c` with the world (`00c93820`, `00c93920`; a null `this` passes null), keeps a
/// new 16-byte record holding the world in the array at `this + 0x30` (data, count at
/// `+0x34`, capacity and flags at `+0x38`), and calls the slot at byte offset 4 of the
/// `this + 0x28` listener with every entity of the world's active islands (`+0x28`, count
/// `+0x2c`), inactive islands (`+0x34`, count `+0x38`) and fixed island (`+0x20`); an
/// island's entities are the array at `+0x48`, count at `+0x4c`.
pub fn fn_00d68e60(e: &mut Engine, this: Ptr, world: u32) {
    let this = this.addr();
    let listener_first = if this == 0 {
        0
    } else {
        this.wrapping_add(0x28)
    };
    e.call(WORLD_ADD_LISTENER_FIRST, &args![world, listener_first]);
    let listener_second = if this == 0 {
        0
    } else {
        this.wrapping_add(0x2c)
    };
    e.call(
        WORLD_ADD_POST_SIMULATION_LISTENER,
        &args![world, listener_second],
    );

    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x10u32]).u32();
    let mut record = 0;
    if block != 0 {
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 0xc, 0x8000_0000);
        record = block;
    }
    // A null block faults here in the exe too.
    e.mem.set_u32(record, world);

    let array = this.wrapping_add(0x30);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(this.wrapping_add(0x38)) & 0x3fff_ffff;
    if e.mem.u32(this.wrapping_add(0x34)) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let count = e.mem.u32(this.wrapping_add(0x34));
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(count.wrapping_mul(4)), record);
    e.mem
        .set_u32(this.wrapping_add(0x34), count.wrapping_add(1));

    // The listener the entities are announced to is always `this + 0x28`.
    let listener = this.wrapping_add(0x28);
    for (islands, islands_count) in [(0x28u32, 0x2cu32), (0x34, 0x38)] {
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(world + islands_count) {
            let data = e.mem.u32(world + islands);
            let island = e.mem.u32(data + index * 4);
            notify_each_entity(e, listener, island + 0x48);
            index = index.wrapping_add(1);
        }
    }
    let fixed_island = e.mem.u32(world + 0x20);
    if fixed_island != 0 {
        notify_each_entity(e, listener, fixed_island + 0x48);
    }
}

// Translated from 00d68fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::init` (Xbox PDB name; the vtable slot after the destructor): adds every world of the viewer's context (the object at `this + 0x1c`: world pointers at
/// `+0x6c`, count at `+0x70`) through `00d68e60` on `this - 8`. A null context, or none
/// with a world, does nothing.
pub fn fn_00d68fc0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let context = e.mem.u32(this.wrapping_add(0x1c));
    if context == 0 || e.mem.i32(context + 0x70) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let context = e.mem.u32(this.wrapping_add(0x1c));
        let data = e.mem.u32(context + 0x6c);
        let world = e.mem.u32(data + index * 4);
        fn_00d68e60(e, Ptr::new(this.wrapping_sub(8)), world);
        let context = e.mem.u32(this.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(context + 0x70) {
            break;
        }
    }
}

// Translated from 00d69000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::worldAddedCallback` (slot 1 of the vtable at `010d20c8`): a thunk
/// (`ADD ECX,-0x20; JMP 00d68e60`, so its `RET 4` is that function's) that adds `world` to the
/// viewer whose subobject at `+0x20` is `this`.
pub fn fn_00d69000(e: &mut Engine, this: Ptr, world: u32) {
    fn_00d68e60(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d69010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::removeWorld` (Xbox PDB name by its shape): removes world record `index` of the array at `this + 0x30`: takes the listeners at
/// `this + 0x28` (a null `this` passes null) and `this + 0x2c` off the record's world
/// (`00c917a0`, `00c918e0`), runs the per-world clean-up `00d68b20`, frees the record (its own
/// array first, unless bit 31 of the flags word at `+0xc` is set; then the 16 bytes), shrinks
/// the count and moves the last record into the freed slot.
pub fn fn_00d69010(e: &mut Engine, this: Ptr, index: u32) {
    let this = this.addr();
    let listener_first = if this == 0 {
        0
    } else {
        this.wrapping_add(0x28)
    };
    let slot = index.wrapping_mul(4);
    let record = e.mem.u32(this + 0x30).wrapping_add(slot);
    let world = e.mem.u32(e.mem.u32(record));
    e.call(WORLD_REMOVE_LISTENER_FIRST, &args![world, listener_first]);
    let record = e.mem.u32(this + 0x30).wrapping_add(slot);
    let world = e.mem.u32(e.mem.u32(record));
    e.call(
        WORLD_REMOVE_LISTENER_SECOND,
        &args![world, this.wrapping_add(0x2c)],
    );
    e.call(REMOVE_WORLD_GEOMETRIES, &args![this, index]);

    let record = e.mem.u32(e.mem.u32(this + 0x30).wrapping_add(slot));
    if record != 0 {
        let allocator = router_allocator(e);
        let flags = e.mem.u32(record + 0xc);
        e.mem.set_u32(record + 8, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(record + 4);
            let bytes = (flags & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 0xc, 0x8000_0000);
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![record, 0x10u32]);
    }
    let count = e.mem.u32(this + 0x34).wrapping_sub(1);
    e.mem.set_u32(this + 0x34, count);
    if count != index {
        let data = e.mem.u32(this + 0x30);
        let last = e.mem.u32(data.wrapping_add(count.wrapping_mul(4)));
        e.mem.set_u32(data.wrapping_add(slot), last);
    }
}

// Translated from 00d690d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::~hkpConvexRadiusViewer` (Xbox PDB; called by the scalar deleting destructor `00d691d0`): points the five vtables (`this`, `+8`, `+0x20`, `+0x28`, `+0x2c`) at the
/// viewer's, removes every world record from the last (`00d69010`), frees the record array
/// (unless bit 31 of the capacity word at `+0x38` is set), leaves the two listener vtables at
/// their base classes' and tail-calls the base destructor `00ddcc60` on `this`.
pub fn fn_00d690d0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = e.mem.u32(this + 0x34).wrapping_sub(1) as i32;
    e.mem.set_u32(this, 0x010d_20f8);
    e.mem.set_u32(this + 8, 0x010d_20d8);
    e.mem.set_u32(this + 0x20, 0x010d_20c8);
    e.mem.set_u32(this + 0x28, 0x010d_20ac);
    e.mem.set_u32(this + 0x2c, 0x010d_209c);
    while index >= 0 {
        fn_00d69010(e, Ptr::new(this), index as u32);
        index = index.wrapping_sub(1);
    }
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(this + 0x38);
    e.mem.set_u32(this + 0x34, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(this + 0x30);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_u32(this + 0x38, 0x8000_0000);
    e.mem.set_u32(this + 0x2c, 0x010c_cf7c);
    e.mem.set_u32(this + 0x28, 0x0102_f13c);
    e.call(WORLD_VIEWER_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00d69160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::worldRemovedCallback` (slot 2 of the vtable at `010d20c8`), on the subobject at `+0x20` (`this - 0x20` is the viewer): looks `world` up with
/// `00d68a70` and, when found (a non-negative index), removes its record with `00d69010`.
pub fn fn_00d69160(e: &mut Engine, this: Ptr, world: u32) {
    let viewer = this.addr().wrapping_sub(0x20);
    let index = e.call(FIND_WORLD, &args![viewer, world]).i32();
    if index >= 0 {
        fn_00d69010(e, Ptr::new(viewer), index as u32);
    }
}

// Translated from 00d69180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusViewer::getProcessTag` (slot 1 of the `hkProcess` vtable at `010d20d8`):
/// returns the static word at `01269010` (the Xbox PDB's static `m_tag`).
pub fn fn_00d69180(e: &mut Engine) -> u32 {
    e.mem.u32(0x0126_9010)
}

// Translated from 00d69200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintViewer::worldAddedCallback` (Xbox PDB name; the exe's RTTI names the class of the
/// vtable at `010d2194`, whose slot 1 this is): the `this` it gets is the subobject at `+0x20`.
/// Calls `hkpWorld::addWorldPostSimulationListener` (`00c93920`) on `world` with the listener
/// `this + 8`, or null when `this - 0x20` is null.
pub fn fn_00d69200(e: &mut Engine, this: Ptr, world: u32) {
    let listener = if this.addr().wrapping_sub(0x20) == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, listener]);
}

// Translated from 00d69230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintViewer::worldRemovedCallback` (Xbox PDB name; slot 2 of the same vtable): as
/// `00d69200` but through `00c918e0` (the removal paired with `00c93920`; the map has no name
/// for it): calls it on `world` with the listener `this + 8`, or null when `this - 0x20` is null.
pub fn fn_00d69230(e: &mut Engine, this: Ptr, world: u32) {
    let listener = if this.addr().wrapping_sub(0x20) == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(WORLD_REMOVE_LISTENER_SECOND, &args![world, listener]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_91d0, deleting_destructor_00d691d0(Ptr, u8) -> Ptr),
        entry!(0x00d6_9190, fn_00d69190(Ptr, u32) -> u32),
        entry!(0x00d6_91a0, fn_00d691a0(Ptr, u32) -> u32),
        entry!(0x00d6_91b0, fn_00d691b0(Ptr, u32) -> u32),
        entry!(0x00d6_91c0, fn_00d691c0(Ptr, u32) -> u32),
        entry!(0x00d6_8e60, fn_00d68e60(Ptr, u32)),
        entry!(0x00d6_8fc0, fn_00d68fc0(Ptr)),
        entry!(0x00d6_9000, fn_00d69000(Ptr, u32)),
        entry!(0x00d6_9010, fn_00d69010(Ptr, u32)),
        entry!(0x00d6_90d0, fn_00d690d0(Ptr)),
        entry!(0x00d6_9160, fn_00d69160(Ptr, u32)),
        entry!(0x00d6_9180, fn_00d69180() -> u32),
        entry!(0x00d6_9200, fn_00d69200(Ptr, u32)),
        entry!(0x00d6_9230, fn_00d69230(Ptr, u32)),
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
    fn deleting_destructor_00d691d0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_91d0, 0x00d6_90d0);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d69190() {
        assert_eq!(check(0x00d6_9190, 0x00d6_91d0, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d691a0() {
        assert_eq!(check(0x00d6_91a0, 0x00d6_91d0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d691b0() {
        assert_eq!(check(0x00d6_91b0, 0x00d6_91d0, 0x28), 0xdead_beef);
    }

    #[test]
    fn thunk_00d691c0() {
        assert_eq!(check(0x00d6_91c0, 0x00d6_91d0, 0x2c), 0xdead_beef);
    }
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    const ALLOC_SLOT: u32 = 0x0900_0001;
    const FREE_SLOT: u32 = 0x0900_0002;
    const BUFFER_FREE_SLOT: u32 = 0x0900_0003;
    const ENTITY_SLOT: u32 = 0x0900_0004;

    /// The recorded calls to `address`, each as its argument words.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Installs the memory router with an allocator whose three slots are doubles that do not
    /// change anything but allocation (slot 4 returns a fresh block of the asked size).
    fn install_router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        e.mem.set_u32(vtable + 0x10, BUFFER_FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOC_SLOT, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(FREE_SLOT, |_, _| Ret::default());
        e.register(BUFFER_FREE_SLOT, |_, _| Ret::default());
        allocator
    }

    /// An island whose entity array holds `entities`.
    fn island(e: &mut Engine, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x60);
        let data = e.mem.alloc(16);
        for (i, entity) in entities.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *entity);
        }
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        island
    }

    /// A pointer array holding `items`: returns the data pointer.
    fn pointer_array(e: &mut Engine, items: &[u32]) -> u32 {
        let data = e.mem.alloc(16);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        data
    }

    /// A viewer with a listener vtable whose slot 4 is `ENTITY_SLOT`, an empty record array
    /// of the given capacity, and the doubles every function here calls.
    fn viewer(e: &mut Engine, capacity: u32) -> u32 {
        install_router(e);
        let base = e.mem.alloc(0x60);
        let listener_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(listener_vtable + 4, ENTITY_SLOT);
        e.mem.set_u32(base + 0x28, listener_vtable);
        let data = e.mem.alloc(16);
        e.mem.set_u32(base + 0x30, data);
        e.mem.set_u32(base + 0x38, capacity);
        e.register(WORLD_ADD_LISTENER_FIRST, |_, _| Ret::default());
        e.register(WORLD_ADD_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.register(ENTITY_SLOT, |_, _| Ret::default());
        base
    }

    #[test]
    fn add_world_registers_listeners_keeps_a_record_and_announces_every_entity() {
        let mut e = Engine::new();
        let this = viewer(&mut e, 4);
        let active = [
            island(&mut e, &[0x11, 0x12]),
            island(&mut e, &[]),
            island(&mut e, &[0x13]),
        ];
        let inactive = [island(&mut e, &[0x21])];
        let fixed = island(&mut e, &[0x31, 0x32]);
        let world = e.mem.alloc(0x80);
        let active_data = pointer_array(&mut e, &active);
        let inactive_data = pointer_array(&mut e, &inactive);
        e.mem.set_u32(world + 0x28, active_data);
        e.mem.set_u32(world + 0x2c, 3);
        e.mem.set_u32(world + 0x34, inactive_data);
        e.mem.set_u32(world + 0x38, 1);
        e.mem.set_u32(world + 0x20, fixed);
        e.call_log = Some(vec![]);
        e.call(0x00d6_8e60, &args![this, world]);

        assert_eq!(
            calls(&e, WORLD_ADD_LISTENER_FIRST),
            [vec![world, this + 0x28]]
        );
        assert_eq!(
            calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER),
            [vec![world, this + 0x2c]]
        );
        let announced: Vec<u32> = calls(&e, ENTITY_SLOT).iter().map(|a| a[1]).collect();
        assert_eq!(announced, [0x11, 0x12, 0x13, 0x21, 0x31, 0x32]);
        assert!(calls(&e, ENTITY_SLOT).iter().all(|a| a[0] == this + 0x28));
        // The record: world first, the rest of the array header cleared, flags "not owned".
        assert_eq!(e.mem.u32(this + 0x34), 1);
        let record = e.mem.u32(e.mem.u32(this + 0x30));
        assert_eq!(e.mem.u32(record), world);
        assert_eq!(e.mem.u32(record + 4), 0);
        assert_eq!(e.mem.u32(record + 8), 0);
        assert_eq!(e.mem.u32(record + 0xc), 0x8000_0000);
        assert_eq!(calls(&e, ALLOC_SLOT).len(), 1);
        assert_eq!(calls(&e, ALLOC_SLOT)[0][1], 0x10);
    }

    #[test]
    fn add_world_grows_a_full_record_array_and_skips_a_missing_fixed_island() {
        let mut e = Engine::new();
        let this = viewer(&mut e, 0);
        let world = e.mem.alloc(0x80);
        // Count 0 equals capacity 0: the array is grown (the double supplies room).
        e.register(ARRAY_GROW, |e, a| {
            let room = e.mem.alloc(16);
            e.mem.set_u32(a[1], room);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00d68e60(&mut e, Ptr::new(this), world);
        let grows = calls(&e, ARRAY_GROW);
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0][1..], [this + 0x30, 4]);
        assert_eq!(e.mem.u32(this + 0x34), 1);
        assert!(calls(&e, ENTITY_SLOT).is_empty());
        // A set flag bit in the capacity word does not count as capacity.
        e.mem.set_u32(this + 0x38, 0xc000_0001);
        e.mem.set_u32(this + 0x34, 1);
        fn_00d68e60(&mut e, Ptr::new(this), world);
        assert_eq!(calls(&e, ARRAY_GROW).len(), 2);
    }

    #[test]
    fn init_adds_each_world_of_the_context_to_the_viewer_before_it() {
        let mut e = Engine::new();
        let base = viewer(&mut e, 4);
        let this = base + 8;
        // `viewer` set the array at base + 0x30; this - 8 is that base.
        let world_a = e.mem.alloc(0x80);
        let world_b = e.mem.alloc(0x80);
        let data = pointer_array(&mut e, &[world_a, world_b]);
        let context = e.mem.alloc(0x80);
        e.mem.set_u32(context + 0x6c, data);
        e.mem.set_u32(context + 0x70, 2);
        e.mem.set_u32(this + 0x1c, context);
        e.call_log = Some(vec![]);
        e.call(0x00d6_8fc0, &args![this]);
        assert_eq!(
            calls(&e, WORLD_ADD_LISTENER_FIRST),
            [vec![world_a, base + 0x28], vec![world_b, base + 0x28]]
        );
        assert_eq!(e.mem.u32(base + 0x34), 2);
    }

    #[test]
    fn init_does_nothing_without_a_context_or_without_worlds() {
        let mut e = Engine::new();
        let base = viewer(&mut e, 4);
        let this = base + 8;
        e.call_log = Some(vec![]);
        e.call(0x00d6_8fc0, &args![this]);
        let context = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x1c, context);
        e.call(0x00d6_8fc0, &args![this]);
        assert!(calls(&e, WORLD_ADD_LISTENER_FIRST).is_empty());
    }

    #[test]
    fn thunk_00d69000_adds_the_world_to_the_viewer_at_minus_0x20() {
        let mut e = Engine::new();
        let base = viewer(&mut e, 4);
        let world = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00d6_9000, &args![base + 0x20, world]);
        assert_eq!(
            calls(&e, WORLD_ADD_LISTENER_FIRST),
            [vec![world, base + 0x28]]
        );
    }

    /// Three records (worlds 0x100, 0x200, 0x300) in the viewer's array; the first owns a
    /// 3-word array, the second and third do not.
    fn viewer_with_records(e: &mut Engine) -> (u32, [u32; 3]) {
        let this = viewer(e, 4);
        let mut records = [0u32; 3];
        for (i, record) in records.iter_mut().enumerate() {
            *record = e.mem.alloc(0x10);
            e.mem.set_u32(*record, 0x100 * (i as u32 + 1));
            e.mem.set_u32(*record + 4, 0x5000 + i as u32);
        }
        e.mem.set_u32(records[0] + 0xc, 3);
        e.mem.set_u32(records[1] + 0xc, 0x8000_0000);
        e.mem.set_u32(records[2] + 0xc, 0x8000_0000);
        let data = e.mem.u32(this + 0x30);
        for (i, record) in records.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *record);
        }
        e.mem.set_u32(this + 0x34, 3);
        e.register(WORLD_REMOVE_LISTENER_FIRST, |_, _| Ret::default());
        e.register(WORLD_REMOVE_LISTENER_SECOND, |_, _| Ret::default());
        e.register(REMOVE_WORLD_GEOMETRIES, |_, _| Ret::default());
        (this, records)
    }

    #[test]
    fn remove_world_frees_an_owned_array_and_moves_the_last_record_in() {
        let mut e = Engine::new();
        let (this, records) = viewer_with_records(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00d6_9010, &args![this, 0u32]);
        assert_eq!(
            calls(&e, WORLD_REMOVE_LISTENER_FIRST),
            [vec![0x100, this + 0x28]]
        );
        assert_eq!(
            calls(&e, WORLD_REMOVE_LISTENER_SECOND),
            [vec![0x100, this + 0x2c]]
        );
        assert_eq!(calls(&e, REMOVE_WORLD_GEOMETRIES), [vec![this, 0]]);
        assert_eq!(calls(&e, BUFFER_FREE_SLOT).len(), 1);
        assert_eq!(calls(&e, BUFFER_FREE_SLOT)[0][1..], [0x5000, 12]);
        assert_eq!(calls(&e, FREE_SLOT)[0][1..], [records[0], 0x10]);
        assert_eq!(e.mem.u32(records[0] + 4), 0);
        assert_eq!(e.mem.u32(records[0] + 8), 0);
        assert_eq!(e.mem.u32(records[0] + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x34), 2);
        let data = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(data), records[2]);
        assert_eq!(e.mem.u32(data + 4), records[1]);
    }

    #[test]
    fn remove_world_of_the_last_record_keeps_the_others_and_skips_a_borrowed_array() {
        let mut e = Engine::new();
        let (this, records) = viewer_with_records(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00d6_9010, &args![this, 2u32]);
        assert!(calls(&e, BUFFER_FREE_SLOT).is_empty());
        assert_eq!(calls(&e, FREE_SLOT)[0][1..], [records[2], 0x10]);
        assert_eq!(e.mem.u32(this + 0x34), 2);
        let data = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(data), records[0]);
        assert_eq!(e.mem.u32(data + 4), records[1]);
        // The slot of the removed record is left as it was.
        assert_eq!(e.mem.u32(data + 8), records[2]);
    }

    #[test]
    fn remove_world_with_a_null_record_only_shrinks_the_array() {
        let mut e = Engine::new();
        let (this, records) = viewer_with_records(&mut e);
        // The first record is fetched through the array before it is null-checked; a null
        // record entry would fault, so null the pointer only after the world reads: the
        // clean-up double does it.
        let data = e.mem.u32(this + 0x30);
        e.register_double(REMOVE_WORLD_GEOMETRIES, move |e, _| {
            e.mem.set_u32(data + 4, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d6_9010, &args![this, 1u32]);
        assert!(calls(&e, FREE_SLOT).is_empty());
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert_eq!(e.mem.u32(data + 4), records[2]);
    }

    #[test]
    fn destructor_removes_the_records_last_first_and_hands_over_to_the_base() {
        let mut e = Engine::new();
        let (this, _records) = viewer_with_records(&mut e);
        let array = e.mem.u32(this + 0x30);
        e.register(WORLD_VIEWER_BASE_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_90d0, &args![this]);
        let cleaned: Vec<u32> = calls(&e, REMOVE_WORLD_GEOMETRIES)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(cleaned, [2, 1, 0]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x8000_0000);
        // Only the array of the viewer itself is freed last, with its 4-word capacity.
        let freed = calls(&e, BUFFER_FREE_SLOT);
        assert_eq!(freed.last().unwrap()[1..], [array, 16]);
        assert_eq!(e.mem.u32(this), 0x010d_20f8);
        assert_eq!(e.mem.u32(this + 8), 0x010d_20d8);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_20c8);
        assert_eq!(e.mem.u32(this + 0x28), 0x0102_f13c);
        assert_eq!(e.mem.u32(this + 0x2c), 0x010c_cf7c);
        assert_eq!(calls(&e, WORLD_VIEWER_BASE_DESTRUCTOR), [vec![this]]);
    }

    #[test]
    fn destructor_of_an_empty_viewer_with_a_borrowed_array_frees_nothing() {
        let mut e = Engine::new();
        let this = viewer(&mut e, 0x8000_0000);
        e.register(WORLD_VIEWER_BASE_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_90d0, &args![this]);
        assert!(calls(&e, BUFFER_FREE_SLOT).is_empty());
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn world_removed_thunk_removes_the_record_found_for_the_world() {
        let mut e = Engine::new();
        let (base, _records) = viewer_with_records(&mut e);
        e.register(FIND_WORLD, |_, a| Ret {
            eax: if a[1] == 0x200 { 1 } else { u32::MAX },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d6_9160, &args![base + 0x20, 0x200u32]);
        assert_eq!(calls(&e, FIND_WORLD), [vec![base, 0x200]]);
        assert_eq!(calls(&e, REMOVE_WORLD_GEOMETRIES), [vec![base, 1]]);
        e.call(0x00d6_9160, &args![base + 0x20, 0x999u32]);
        assert_eq!(calls(&e, REMOVE_WORLD_GEOMETRIES).len(), 1);
    }

    #[test]
    fn process_tag_getter_returns_the_static_word() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.set_global(0x0126_9010, 0xfeed_u32);
        assert_eq!(e.call(0x00d6_9180, &args![]).u32(), 0xfeed);
    }

    #[test]
    fn listener_thunks_pass_this_plus_8_or_null() {
        let mut e = Engine::new();
        e.register(WORLD_ADD_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.register(WORLD_REMOVE_LISTENER_SECOND, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_9200, &args![0x2000u32, 0x77u32]);
        e.call(0x00d6_9200, &args![0x20u32, 0x78u32]);
        e.call(0x00d6_9230, &args![0x2000u32, 0x79u32]);
        e.call(0x00d6_9230, &args![0x20u32, 0x7au32]);
        assert_eq!(
            calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER),
            [vec![0x77, 0x2008], vec![0x78, 0]]
        );
        assert_eq!(
            calls(&e, WORLD_REMOVE_LISTENER_SECOND),
            [vec![0x79, 0x2008], vec![0x7a, 0]]
        );
    }
}
