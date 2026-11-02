//! `Havok SDK/run_00d64650` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d649c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_48c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d649c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_48c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d64980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d649c0`. `00d649c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d64980(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_49c0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d64990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d649c0`. `00d649c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d64990(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_49c0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d649a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x2c (`SUB ECX,0x2c`) and jumps to `00d649c0`. `00d649c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d649a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_49c0, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

// Translated from 00d649b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d649c0`. `00d649c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d649b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_49c0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

use super::run_00d51760::push_word_with;

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c93820` pushes it on the array at world `+0x128` and `00c917a0` zeroes the first entry of
/// that array that equals it; `00c93920` (Xbox PDB name `hkpWorld::addWorldPostSimulationListener`)
/// pushes it on the array at `+0x164` and `00c918e0` zeroes the first entry of that one that
/// equals it.
const WORLD_ADD_ISLAND_LISTENER: u32 = 0x00c9_3820;
const WORLD_REMOVE_ISLAND_LISTENER: u32 = 0x00c9_17a0;
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// The two methods of the swept-transform display viewer's own source file (thiscall on the
/// viewer, one argument): `00d63ff0` finds the index of the world in the array of per-world
/// records at `+0x30` (size at `+0x34`; the record's first word is the world), or -1, and
/// `00d641f0` takes down the display state of the record at an index.
const FIND_WORLD_RECORD: u32 = 0x00d6_3ff0;
const TAKE_DOWN_WORLD_RECORD: u32 = 0x00d6_41f0;
/// `hkpWorldViewerBase`'s constructor (Xbox PDB, `00ddccb0`, thiscall, one argument: the process
/// contexts) and destructor (`00ddcc60`, thiscall, no arguments; it answers its `EAX`).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The id this viewer's process was registered under (`01268fd0`).
const SWEPT_VIEWER_PROCESS_ID: u32 = 0x0126_8fd0;
/// The vtable pointers of the swept-transform display viewer (RTTI:
/// `hkpSweptTransformDisplayViewer`): the live ones at `+0`, `+8`, `+0x20`, `+0x28` and `+0x2c`,
/// where `0102f13c` is the vtable of the listener base class (RTTI: `hkpEntityListener`) and
/// `010ccf7c` the one of `hkpWorldPostSimulationListener`.
const SWEPT_VTABLE_VIEWER: u32 = 0x010d_1918;
const SWEPT_VTABLE_PROCESS: u32 = 0x010d_18f8;
const SWEPT_VTABLE_FILTER: u32 = 0x010d_18e8;
const SWEPT_VTABLE_ENTITY_LISTENER: u32 = 0x010d_18cc;
const SWEPT_VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_18bc;
const ENTITY_LISTENER_BASE_VTABLE: u32 = 0x0102_f13c;
const POST_SIMULATION_LISTENER_BASE_VTABLE: u32 = 0x010c_cf7c;
/// The vtable pointers of the simulation island viewer (RTTI: `hkpSimulationIslandViewer`):
/// `+0`, `+8`, `+0x20` and the listener's at `+0x28`.
const ISLAND_VTABLE_VIEWER: u32 = 0x010d_1a28;
const ISLAND_VTABLE_PROCESS: u32 = 0x010d_1a08;
const ISLAND_VTABLE_FILTER: u32 = 0x010d_19f8;
const ISLAND_VTABLE_LISTENER: u32 = 0x010d_19e8;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Calls the method at vtable offset `slot` of the sub-object `listener` with every element of the
/// array of entity pointers at `island + 0x48` (data, size at `+4`, both read again each time).
pub(crate) fn replay_entities(e: &mut Engine, listener: u32, island: u32, slot: u32) {
    let array = island.wrapping_add(0x48);
    if e.mem.i32(array.wrapping_add(4)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let data = e.mem.u32(array);
        let entity = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        e.vcall(listener, slot, &args![entity]);
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(array.wrapping_add(4)) {
            break;
        }
    }
}

// Translated from 00d64650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the swept-transform display viewer (`this` may be null) taking a world: hands the
/// listeners `this + 0x28` and `this + 0x2c` (0 for a null `this`) to the world
/// (`00c93820`, `00c93920`), makes a 0x10-byte record through the router's allocator (slot 4,
/// an empty array in it at `+4`, capacity word `0x80000000`), stores the world in its first word and
/// keeps the record in the array at `this + 0x30`; then calls the method at vtable offset 4 of
/// the sub-object `this + 0x28` with every entity of every island in the world's two island
/// arrays (`world + 0x28`, count `+0x2c`; `world + 0x34`, count `+0x38`) and of the island at
/// `world + 0x20` (the entity array of an island is at `+0x48`). The exe stores the world
/// through the record's address even when the allocation failed (address 0).
pub fn fn_00d64650(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    e.call(WORLD_ADD_ISLAND_LISTENER, &args![world, first]);
    let second = if at != 0 { at.wrapping_add(0x2c) } else { 0 };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, second]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let node = e.vcall(allocator, 4, &args![0x10u32]).u32();
    let mut record = 0;
    if node != 0 {
        e.mem.set_u32(node.wrapping_add(4), 0);
        e.mem.set_u32(node.wrapping_add(8), 0);
        e.mem.set_u32(node.wrapping_add(0xc), 0x8000_0000);
        record = node;
    }
    e.mem.set_u32(record, world);
    push_word_with(e, at.wrapping_add(0x30), record, 0x10);
    let listener = at.wrapping_add(0x28);
    let mut index = 0u32;
    if e.mem.i32(world.wrapping_add(0x2c)) > 0 {
        loop {
            let islands = e.mem.u32(world.wrapping_add(0x28));
            let island = e.mem.u32(islands.wrapping_add(index.wrapping_mul(4)));
            replay_entities(e, listener, island, 4);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(world.wrapping_add(0x2c)) {
                break;
            }
        }
    }
    let mut index = 0u32;
    if e.mem.i32(world.wrapping_add(0x38)) > 0 {
        loop {
            let islands = e.mem.u32(world.wrapping_add(0x34));
            let island = e.mem.u32(islands.wrapping_add(index.wrapping_mul(4)));
            replay_entities(e, listener, island, 4);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(world.wrapping_add(0x38)) {
                break;
            }
        }
    }
    let island = e.mem.u32(world.wrapping_add(0x20));
    replay_entities(e, listener, island, 4);
}

// Translated from 00d647b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00d64650` on `this - 8` once for each world of the container at `[this + 0x1c]` (a
/// count at `+0x70`, an array of world pointers at `+0x6c`; both read again after every call).
/// Does nothing without a container or without worlds.
pub fn fn_00d647b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 || e.mem.i32(container.wrapping_add(0x70)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
        fn_00d64650(e, Ptr::new(at.wrapping_sub(8)), Ptr::new(world));
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
            break;
        }
    }
}

// Translated from 00d647f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d64650` with the same stack word.
pub fn fn_00d647f0(e: &mut Engine, this: Ptr, world: Ptr) {
    fn_00d64650(e, Ptr::new(this.addr().wrapping_sub(0x20)), world);
}

// Translated from 00d64800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes down the record at `index` of the array at `this + 0x30`: removes the listeners
/// (`this + 0x28`, 0 for a null `this`, from the world in the record: `00c917a0`;
/// `this + 0x2c` from it: `00c918e0`), takes the display state down (`00d641f0` on `this`
/// with the index), and when the record exists empties its array (freeing the storage through
/// the router's allocator, slot `0x10`, when the capacity word is not negative: the data and
/// `(capacity & 0x3fffffff) * 4` bytes) and frees the 0x10-byte record (slot 8). Then lowers the
/// count at `+0x34` and moves the last record into the hole.
pub fn fn_00d64800(e: &mut Engine, this: Ptr, index: i32) {
    remove_record(e, this, index, TAKE_DOWN_WORLD_RECORD);
}

/// The body of `00d64800` and `00d65fc0` (the same compiled code with another take-down routine,
/// `take_down`).
pub(crate) fn remove_record(e: &mut Engine, this: Ptr, index: i32, take_down: u32) {
    let at = this.addr();
    let offset = (index as u32).wrapping_mul(4);
    let first = if at != 0 { at.wrapping_add(0x28) } else { 0 };
    let records = e.mem.u32(at.wrapping_add(0x30));
    let node = e.mem.u32(records.wrapping_add(offset));
    let world = e.mem.u32(node);
    e.call(WORLD_REMOVE_ISLAND_LISTENER, &args![world, first]);
    let records = e.mem.u32(at.wrapping_add(0x30));
    let node = e.mem.u32(records.wrapping_add(offset));
    let world = e.mem.u32(node);
    e.call(
        WORLD_REMOVE_POST_SIMULATION_LISTENER,
        &args![world, at.wrapping_add(0x2c)],
    );
    e.call(take_down, &args![this, index]);
    let records = e.mem.u32(at.wrapping_add(0x30));
    let node = e.mem.u32(records.wrapping_add(offset));
    if node != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        let capacity = e.mem.u32(node.wrapping_add(0xc));
        e.mem.set_u32(node.wrapping_add(8), 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(node.wrapping_add(4));
            let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(node.wrapping_add(4), 0);
        e.mem.set_u32(node.wrapping_add(0xc), 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.vcall(allocator, 8, &args![node, 0x10u32]);
    }
    let count = e.mem.u32(at.wrapping_add(0x34)).wrapping_sub(1);
    e.mem.set_u32(at.wrapping_add(0x34), count);
    if count as i32 != index {
        let records = e.mem.u32(at.wrapping_add(0x30));
        let last = e.mem.u32(records.wrapping_add(count.wrapping_mul(4)));
        e.mem.set_u32(records.wrapping_add(offset), last);
    }
}

// Translated from 00d648c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the swept-transform display viewer (`ECX` = `this`): puts its vtable
/// pointers back (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`), takes down every record of the array
/// at `this + 0x30` from the last to the first (`00d64800`), empties the array (freeing its data
/// through the allocator, slot `0x10`, when the capacity word at `+0x38` is not negative),
/// stores the listener base vtables (`+0x2c`: post-simulation listener, `+0x28`: entity
/// listener) and ends in the `hkpWorldViewerBase` destructor (`00ddcc60`), whose `EAX` it
/// answers.
pub fn fn_00d648c0(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let mut index = e.mem.u32(at.wrapping_add(0x34)).wrapping_sub(1);
    e.mem.set_u32(at, SWEPT_VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), SWEPT_VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), SWEPT_VTABLE_FILTER);
    e.mem
        .set_u32(at.wrapping_add(0x28), SWEPT_VTABLE_ENTITY_LISTENER);
    e.mem
        .set_u32(at.wrapping_add(0x2c), SWEPT_VTABLE_POST_SIMULATION_LISTENER);
    while (index as i32) >= 0 {
        fn_00d64800(e, this, index as i32);
        index = index.wrapping_sub(1);
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
        .set_u32(at.wrapping_add(0x28), ENTITY_LISTENER_BASE_VTABLE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

// Translated from 00d64950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole viewer) taking a world: finds
/// the world's record (`00d63ff0` on the whole viewer) and, when the index is not negative,
/// takes it down (`00d64800`).
pub fn fn_00d64950(e: &mut Engine, this: Ptr, world: Ptr) {
    let whole = Ptr::new(this.addr().wrapping_sub(0x20));
    let index = e.call(FIND_WORLD_RECORD, &args![whole, world]).i32();
    if index >= 0 {
        fn_00d64800(e, whole, index);
    }
}

// Translated from 00d64970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id of the swept-transform display viewer's registration (`01268fd0`;
/// virtual method, `ECX` unused).
pub fn fn_00d64970(e: &mut Engine) -> u32 {
    e.global(SWEPT_VIEWER_PROCESS_ID)
}

/// The listener the thunks below pass: `this + 8` when the object they are adjusted to (`this -
/// 0x20`) is not null, otherwise 0.
fn adjusted_listener(this: Ptr) -> u32 {
    if this.addr().wrapping_sub(0x20) != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    }
}

// Translated from 00d649f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole object): adds the listener
/// `this + 8` (0 when the whole object would be null) to the world's post-simulation listeners
/// (`00c93920` on `world`, which comes as the one stack word).
pub fn fn_00d649f0(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, listener]);
}

// Translated from 00d64a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d649f0`, removing the listener (`00c918e0`).
pub fn fn_00d64a20(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(
        WORLD_REMOVE_POST_SIMULATION_LISTENER,
        &args![world, listener],
    );
}

// Translated from 00d64a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the simulation island viewer (RTTI: `hkpSimulationIslandViewer`, vtables
/// `010d1a28` and the three after it): runs the `hkpWorldViewerBase` constructor (`00ddccb0`,
/// with the process `contexts`), stores the listener base vtable at `+0x28`, the vtables (`+0`,
/// `+8`, `+0x20`, the listener's at `+0x28`), the bytes 1 at `+0x2c` and `+0x2d`, two empty
/// arrays (`+0x30` and `+0x3c`: data, size, capacity word `0x80000000`) and, when a world was
/// found (`+0x24`) with worlds, registers the listener (`this + 0x28`, `00c93920`) with each of
/// the `[world + 0x70]` entries of the array at `[world + 0x6c]`. Answers `this`.
pub fn fn_00d64a50(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem
        .set_u32(at.wrapping_add(0x28), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem.set_u32(at, ISLAND_VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), ISLAND_VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), ISLAND_VTABLE_FILTER);
    e.mem.set_u32(at.wrapping_add(0x28), ISLAND_VTABLE_LISTENER);
    e.mem.set_u8(at.wrapping_add(0x2c), 1);
    e.mem.set_u8(at.wrapping_add(0x2d), 1);
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x34), 0);
    e.mem.set_u32(at.wrapping_add(0x38), 0x8000_0000);
    e.mem.set_u32(at.wrapping_add(0x3c), 0);
    e.mem.set_u32(at.wrapping_add(0x40), 0);
    e.mem.set_u32(at.wrapping_add(0x44), 0x8000_0000);
    let world = e.mem.u32(at.wrapping_add(0x24));
    if world != 0 && e.mem.i32(world.wrapping_add(0x70)) > 0 {
        let listener = at.wrapping_add(0x28);
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(at.wrapping_add(0x24));
            let worlds = e.mem.u32(world.wrapping_add(0x6c));
            let target = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
            e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![target, listener]);
            let world = e.mem.u32(at.wrapping_add(0x24));
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(world.wrapping_add(0x70)) {
                break;
            }
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_4650, fn_00d64650(Ptr, Ptr)),
        entry!(0x00d6_47b0, fn_00d647b0(Ptr)),
        entry!(0x00d6_47f0, fn_00d647f0(Ptr, Ptr)),
        entry!(0x00d6_4800, fn_00d64800(Ptr, i32)),
        entry!(0x00d6_48c0, fn_00d648c0(Ptr) -> u32),
        entry!(0x00d6_4950, fn_00d64950(Ptr, Ptr)),
        entry!(0x00d6_4970, fn_00d64970() -> u32),
        entry!(0x00d6_49f0, fn_00d649f0(Ptr, Ptr)),
        entry!(0x00d6_4a20, fn_00d64a20(Ptr, Ptr)),
        entry!(0x00d6_4a50, fn_00d64a50(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_49c0, deleting_destructor_00d649c0(Ptr, u8) -> Ptr),
        entry!(0x00d6_4980, fn_00d64980(Ptr, u32) -> u32),
        entry!(0x00d6_4990, fn_00d64990(Ptr, u32) -> u32),
        entry!(0x00d6_49a0, fn_00d649a0(Ptr, u32) -> u32),
        entry!(0x00d6_49b0, fn_00d649b0(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d649c0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_49c0, 0x00d6_48c0);
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
    fn le81_thunk_00d64980() {
        assert_eq!(
            check(0x00d6_4980, 0x00d6_49c0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d64990() {
        assert_eq!(
            check(0x00d6_4990, 0x00d6_49c0, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d649a0() {
        assert_eq!(
            check(0x00d6_49a0, 0x00d6_49c0, 0x2c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d649b0() {
        assert_eq!(
            check(0x00d6_49b0, 0x00d6_49c0, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod swept_viewer_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ROUTER: u32 = 0x0900_0100;
    const ALLOCATOR: u32 = 0x0900_0120;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0140;
    const ALLOCATE: u32 = 0x0900_0201;
    const FREE_BLOCK: u32 = 0x0900_0202;
    const FREE_ARRAY: u32 = 0x0900_0203;
    const LISTENER_VTABLE: u32 = 0x0900_0160;
    const LISTENER_CALLBACK: u32 = 0x0900_0204;

    struct Rig {
        e: Engine,
        log: Log,
        found: Rc<Cell<i32>>,
        fail_allocation: Rc<Cell<bool>>,
    }

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

    /// An engine with the router, an allocator (slot 4 hands out blocks, slots 8 and 0x10 are
    /// logged), the listener callback of a viewer's sub-object, and doubles for every outside
    /// function the viewer functions call.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        let log: Log = Rc::default();
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 4, ALLOCATE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 8, FREE_BLOCK);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, FREE_ARRAY);
        e.mem.set_u32(LISTENER_VTABLE + 4, LISTENER_CALLBACK);
        record(&mut e, &log, MEMORY_ROUTER, ROUTER);
        let fail_allocation = Rc::new(Cell::new(false));
        let seen = Rc::clone(&log);
        let fail = Rc::clone(&fail_allocation);
        e.register_double(ALLOCATE, move |e, a| {
            seen.borrow_mut().push((ALLOCATE, a.to_vec()));
            Ret {
                eax: if fail.get() { 0 } else { e.mem.alloc(a[1]) },
                ..Ret::default()
            }
        });
        record(&mut e, &log, FREE_BLOCK, 0);
        record(&mut e, &log, FREE_ARRAY, 0);
        record(&mut e, &log, LISTENER_CALLBACK, 0);
        for address in [
            WORLD_ADD_ISLAND_LISTENER,
            WORLD_REMOVE_ISLAND_LISTENER,
            WORLD_ADD_POST_SIMULATION_LISTENER,
            WORLD_REMOVE_POST_SIMULATION_LISTENER,
            TAKE_DOWN_WORLD_RECORD,
            WORLD_VIEWER_CONSTRUCT,
        ] {
            record(&mut e, &log, address, 0);
        }
        record(&mut e, &log, WORLD_VIEWER_DESTRUCT, 0x99);
        let found = Rc::new(Cell::new(-1));
        let seen = Rc::clone(&log);
        let index = Rc::clone(&found);
        e.register_double(FIND_WORLD_RECORD, move |_, a| {
            seen.borrow_mut().push((FIND_WORLD_RECORD, a.to_vec()));
            Ret {
                eax: index.get() as u32,
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(0x00c9_0940, move |e, a| {
            seen.borrow_mut().push((0x00c9_0940, a.to_vec()));
            let array = a[1];
            let data = e.mem.alloc(64);
            let size = e.mem.u32(array + 4);
            for i in 0..size {
                let word = e.mem.u32(e.mem.u32(array) + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 16);
            Ret::default()
        });
        Rig {
            e,
            log,
            found,
            fail_allocation,
        }
    }

    /// An island with the entities `entities` in the array at `+0x48`.
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

    /// A world with one island at `+0x20`, two in the array at `+0x28` and one in the array
    /// at `+0x34`.
    fn world(e: &mut Engine) -> u32 {
        let world = e.mem.alloc(0x80);
        let single = island(e, &[0x31, 0x32]);
        let first = island(e, &[0x11]);
        let second = island(e, &[0x21, 0x22]);
        let third = island(e, &[]);
        let list_a = e.mem.alloc(8);
        e.mem.set_u32(list_a, first);
        e.mem.set_u32(list_a + 4, second);
        let list_b = e.mem.alloc(4);
        e.mem.set_u32(list_b, third);
        e.mem.set_u32(world + 0x20, single);
        e.mem.set_u32(world + 0x28, list_a);
        e.mem.set_u32(world + 0x2c, 2);
        e.mem.set_u32(world + 0x34, list_b);
        e.mem.set_u32(world + 0x38, 1);
        world
    }

    fn viewer(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, LISTENER_VTABLE);
        this
    }

    #[test]
    fn fn_00d64650_keeps_a_record_and_replays_every_entity() {
        let mut rig = rig();
        let world = world(&mut rig.e);
        let this = viewer(&mut rig.e);
        rig.e.call(0x00d6_4650, &args![this, world]);
        let log = rig.log.borrow();
        let calls: Vec<(u32, Vec<u32>)> = log
            .iter()
            .filter(|(a, _)| *a != MEMORY_ROUTER)
            .cloned()
            .collect();
        let listener = this + 0x28;
        assert_eq!(
            calls,
            vec![
                (WORLD_ADD_ISLAND_LISTENER, vec![world, this + 0x28]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![world, this + 0x2c]),
                (ALLOCATE, vec![ALLOCATOR, 0x10]),
                (0x00c9_0940, vec![ALLOCATOR, this + 0x30, 4]),
                (LISTENER_CALLBACK, vec![listener, 0x11]),
                (LISTENER_CALLBACK, vec![listener, 0x21]),
                (LISTENER_CALLBACK, vec![listener, 0x22]),
                (LISTENER_CALLBACK, vec![listener, 0x31]),
                (LISTENER_CALLBACK, vec![listener, 0x32]),
            ]
        );
        drop(log);
        // The record: the world, an empty array with the "not owned" capacity word.
        assert_eq!(rig.e.mem.u32(this + 0x34), 1);
        let record = rig.e.mem.u32(rig.e.mem.u32(this + 0x30));
        assert_eq!(rig.e.mem.u32(record), world);
        assert_eq!(rig.e.mem.u32(record + 4), 0);
        assert_eq!(rig.e.mem.u32(record + 8), 0);
        assert_eq!(rig.e.mem.u32(record + 0xc), 0x8000_0000);
    }

    #[test]
    #[should_panic]
    fn fn_00d64650_stores_through_address_zero_when_the_allocation_fails() {
        let mut rig = rig();
        rig.fail_allocation.set(true);
        let world = world(&mut rig.e);
        let this = viewer(&mut rig.e);
        rig.e.call(0x00d6_4650, &args![this, world]);
    }

    #[test]
    fn fn_00d647b0_runs_00d64650_for_each_world_on_this_minus_8() {
        let mut rig = rig();
        // The sub-object at this + 0x1c holds the container; the viewer is `this - 8`.
        let viewer_base = viewer(&mut rig.e);
        let this = viewer_base + 8;
        let first = world(&mut rig.e);
        let second = world(&mut rig.e);
        let list = rig.e.mem.alloc(8);
        rig.e.mem.set_u32(list, first);
        rig.e.mem.set_u32(list + 4, second);
        let container = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(container + 0x70, 2);
        rig.e.mem.set_u32(container + 0x6c, list);
        rig.e.mem.set_u32(this + 0x1c, container);
        rig.e.call(0x00d6_47b0, &args![this]);
        let adds: Vec<Vec<u32>> = rig
            .log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == WORLD_ADD_ISLAND_LISTENER)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(
            adds,
            vec![
                vec![first, viewer_base + 0x28],
                vec![second, viewer_base + 0x28]
            ]
        );
        // Without worlds or without a container nothing runs.
        rig.log.borrow_mut().clear();
        rig.e.mem.set_u32(container + 0x70, 0);
        rig.e.call(0x00d6_47b0, &args![this]);
        rig.e.mem.set_u32(this + 0x1c, 0);
        rig.e.call(0x00d6_47b0, &args![this]);
        assert!(rig.log.borrow().is_empty());
    }

    #[test]
    fn fn_00d647f0_runs_00d64650_on_this_minus_0x20() {
        let mut rig = rig();
        let world = world(&mut rig.e);
        let base = viewer(&mut rig.e);
        rig.e.call(0x00d6_47f0, &args![base + 0x20, world]);
        let first = rig.log.borrow()[0].clone();
        assert_eq!(first, (WORLD_ADD_ISLAND_LISTENER, vec![world, base + 0x28]));
    }

    /// A viewer whose array at `+0x30` holds records for `worlds`, each with a data block of
    /// `capacity` words (the capacity word as given).
    fn viewer_with_records(
        e: &mut Engine,
        worlds: &[u32],
        capacity: u32,
    ) -> (u32, Vec<(u32, u32)>) {
        let this = e.mem.alloc(0x60);
        let data = e.mem.alloc(4 * worlds.len() as u32 + 4);
        let mut records = Vec::new();
        for (i, world) in worlds.iter().enumerate() {
            let node = e.mem.alloc(0x10);
            let block = e.mem.alloc(16);
            e.mem.set_u32(node, *world);
            e.mem.set_u32(node + 4, block);
            e.mem.set_u32(node + 8, 2);
            e.mem.set_u32(node + 0xc, capacity);
            e.mem.set_u32(data + 4 * i as u32, node);
            records.push((node, block));
        }
        e.mem.set_u32(this + 0x30, data);
        e.mem.set_u32(this + 0x34, worlds.len() as u32);
        e.mem.set_u32(this + 0x38, worlds.len() as u32 + 1);
        (this, records)
    }

    fn calls(rig: &Rig) -> Vec<(u32, Vec<u32>)> {
        rig.log
            .borrow()
            .iter()
            .filter(|(a, _)| *a != MEMORY_ROUTER)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_00d64800_takes_a_record_down_and_moves_the_last_into_the_hole() {
        let mut rig = rig();
        let (this, records) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2, 0xa3], 3);
        rig.e.call(0x00d6_4800, &args![this, 0u32]);
        assert_eq!(
            calls(&rig),
            vec![
                (WORLD_REMOVE_ISLAND_LISTENER, vec![0xa1, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![0xa1, this + 0x2c]
                ),
                (TAKE_DOWN_WORLD_RECORD, vec![this, 0]),
                // 3 words of owned storage, then the 0x10-byte record itself.
                (FREE_ARRAY, vec![ALLOCATOR, records[0].1, 12]),
                (FREE_BLOCK, vec![ALLOCATOR, records[0].0, 0x10]),
            ]
        );
        assert_eq!(rig.e.mem.u32(this + 0x34), 2);
        let data = rig.e.mem.u32(this + 0x30);
        assert_eq!(rig.e.mem.u32(data), records[2].0);
        assert_eq!(rig.e.mem.u32(data + 4), records[1].0);
        // The record's own array was reset before it was freed.
        assert_eq!(rig.e.mem.u32(records[0].0 + 4), 0);
        assert_eq!(rig.e.mem.u32(records[0].0 + 8), 0);
        assert_eq!(rig.e.mem.u32(records[0].0 + 0xc), 0x8000_0000);
    }

    #[test]
    fn fn_00d64800_the_last_record_is_just_dropped_and_borrowed_storage_is_kept() {
        let mut rig = rig();
        let (this, records) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2], 0x8000_0002);
        rig.e.call(0x00d6_4800, &args![this, 1u32]);
        let seen = calls(&rig);
        // No array free: the capacity word is negative.
        assert!(seen.iter().all(|(a, _)| *a != FREE_ARRAY));
        assert_eq!(seen[3], (FREE_BLOCK, vec![ALLOCATOR, records[1].0, 0x10]));
        assert_eq!(rig.e.mem.u32(this + 0x34), 1);
        let data = rig.e.mem.u32(this + 0x30);
        assert_eq!(rig.e.mem.u32(data), records[0].0);
    }

    #[test]
    fn fn_00d648c0_takes_every_record_down_from_the_last() {
        let mut rig = rig();
        let (this, _) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2], 2);
        let data = rig.e.mem.u32(this + 0x30);
        let got = rig.e.call(0x00d6_48c0, &args![this]).u32();
        assert_eq!(got, 0x99);
        let order: Vec<u32> = calls(&rig)
            .iter()
            .filter(|(a, _)| *a == TAKE_DOWN_WORLD_RECORD)
            .map(|(_, a)| a[1])
            .collect();
        assert_eq!(order, vec![1, 0]);
        assert_eq!(rig.e.mem.u32(this), SWEPT_VTABLE_VIEWER);
        assert_eq!(rig.e.mem.u32(this + 8), SWEPT_VTABLE_PROCESS);
        assert_eq!(rig.e.mem.u32(this + 0x20), SWEPT_VTABLE_FILTER);
        assert_eq!(rig.e.mem.u32(this + 0x28), ENTITY_LISTENER_BASE_VTABLE);
        assert_eq!(
            rig.e.mem.u32(this + 0x2c),
            POST_SIMULATION_LISTENER_BASE_VTABLE
        );
        // The array of records is freed: its capacity word is the viewer's (3 words, owned).
        assert!(calls(&rig).contains(&(FREE_ARRAY, vec![ALLOCATOR, data, 12])));
        assert_eq!(rig.e.mem.u32(this + 0x30), 0);
        assert_eq!(rig.e.mem.u32(this + 0x34), 0);
        assert_eq!(rig.e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(
            calls(&rig).last().unwrap(),
            &(WORLD_VIEWER_DESTRUCT, vec![this])
        );
    }

    #[test]
    fn fn_00d648c0_without_records_only_resets_the_array() {
        let mut rig = rig();
        let this = rig.e.mem.alloc(0x60);
        rig.e.mem.set_u32(this + 0x38, 0x8000_0000);
        rig.e.call(0x00d6_48c0, &args![this]);
        let seen = calls(&rig);
        assert!(seen.iter().all(|(a, _)| *a != TAKE_DOWN_WORLD_RECORD));
        assert!(seen.iter().all(|(a, _)| *a != FREE_ARRAY));
        assert_eq!(rig.e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn fn_00d64950_takes_down_the_record_of_a_known_world() {
        let mut rig = rig();
        let (base, _) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2], 2);
        rig.found.set(1);
        rig.e.call(0x00d6_4950, &args![base + 0x20, 0xa2u32]);
        let seen = calls(&rig);
        assert_eq!(seen[0], (FIND_WORLD_RECORD, vec![base, 0xa2]));
        assert!(seen.contains(&(TAKE_DOWN_WORLD_RECORD, vec![base, 1])));
        // A world that is not found (-1) takes nothing down.
        rig.log.borrow_mut().clear();
        rig.found.set(-1);
        rig.e.call(0x00d6_4950, &args![base + 0x20, 0xa9u32]);
        assert_eq!(calls(&rig), vec![(FIND_WORLD_RECORD, vec![base, 0xa9])]);
    }

    #[test]
    fn fn_00d64970_answers_the_registered_process_id() {
        let mut rig = rig();
        rig.e.set_global(SWEPT_VIEWER_PROCESS_ID, 0x1234u32);
        assert_eq!(rig.e.call(0x00d6_4970, &args![]).u32(), 0x1234);
    }

    #[test]
    fn fn_00d649f0_and_00d64a20_pass_the_adjusted_listener() {
        let mut rig = rig();
        rig.e.call(0x00d6_49f0, &args![0x2020u32, 0x5000u32]);
        rig.e.call(0x00d6_49f0, &args![0x20u32, 0x5000u32]);
        rig.e.call(0x00d6_4a20, &args![0x2020u32, 0x5000u32]);
        rig.e.call(0x00d6_4a20, &args![0x20u32, 0x5000u32]);
        assert_eq!(
            calls(&rig),
            vec![
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x5000, 0x2028]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x5000, 0]),
                (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![0x5000, 0x2028]),
                (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![0x5000, 0]),
            ]
        );
    }

    #[test]
    fn fn_00d64a50_sets_up_the_island_viewer_and_registers_with_each_world() {
        let mut rig = rig();
        let world = rig.e.mem.alloc(0x80);
        let list = rig.e.mem.alloc(8);
        rig.e.mem.set_u32(list, 0x1000);
        rig.e.mem.set_u32(list + 4, 0x1001);
        rig.e.mem.set_u32(world + 0x70, 2);
        rig.e.mem.set_u32(world + 0x6c, list);
        let this = rig.e.mem.alloc(0x60);
        rig.e.register_double(WORLD_VIEWER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x24, world);
            e.mem.set_u32(a[0] + 0x28, 0xdead);
            Ret::default()
        });
        let got = rig.e.call(0x00d6_4a50, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(rig.e.mem.u32(this), ISLAND_VTABLE_VIEWER);
        assert_eq!(rig.e.mem.u32(this + 8), ISLAND_VTABLE_PROCESS);
        assert_eq!(rig.e.mem.u32(this + 0x20), ISLAND_VTABLE_FILTER);
        assert_eq!(rig.e.mem.u32(this + 0x28), ISLAND_VTABLE_LISTENER);
        assert_eq!(rig.e.mem.u8(this + 0x2c), 1);
        assert_eq!(rig.e.mem.u8(this + 0x2d), 1);
        assert_eq!(rig.e.mem.u32(this + 0x38), 0x8000_0000);
        assert_eq!(rig.e.mem.u32(this + 0x44), 0x8000_0000);
        assert_eq!(
            calls(&rig),
            vec![
                (
                    WORLD_ADD_POST_SIMULATION_LISTENER,
                    vec![0x1000, this + 0x28]
                ),
                (
                    WORLD_ADD_POST_SIMULATION_LISTENER,
                    vec![0x1001, this + 0x28]
                ),
            ]
        );
    }
}
