//! `Havok SDK/run_00d67e30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d68230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_7fa0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d68230(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_7fa0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d681e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d68230`. `00d68230` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d681e0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8230, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d681f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d68230`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d681f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8230, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d68200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d68230`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68200(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8230, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d68210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x2c` (`SUB ECX,0x2c`) and jumps to `00d68230`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68210(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8230, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

// Translated from 00d68220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x30` (`SUB ECX,0x30`) and jumps to `00d68230`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68220(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8230, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

use super::run_00d51760::push_word_with;

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c91a20` zeroes the first entry of the array at world `+0x1a0` that equals it, `00c918e0`
/// the first of the post-simulation listeners (`+0x164`).
const WORLD_REMOVE_ISLAND_ACTIVATION_LISTENER: u32 = 0x00c9_1a20;
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// `hkpEntity` method of `hkpentity.cpp` (thiscall on the entity, one argument, the listener):
/// `00c9c670` removes the listener from the entity.
const ENTITY_REMOVE_LISTENER: u32 = 0x00c9_c670;
/// `hkMapBase<unsigned long, unsigned long, hkMapOperations<unsigned long>>` methods (thiscall on
/// the map): `findKey` (Xbox PDB, `00cb18f0`, one argument: the key; answers the slot index),
/// `clearAndDeallocate` (Xbox PDB, `00cb1a90`, one argument: the allocator) and `00cb1fb0` (one
/// argument, the key: removes it). `00a29680` is the plain `RET` the exe calls on the map
/// afterwards.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_REMOVE: u32 = 0x00cb_1fb0;
const EMPTY_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkStringBuf` helpers (thiscall on the buffer / cdecl): the constructor `00cbbd80` (a buffer
/// of `0xc + 0x80` bytes with its inline storage) and `00d61d00` (`buffer, format, ...`, the
/// printf into it).
const STRING_BUFFER_CONSTRUCT: u32 = 0x00cb_bd80;
const STRING_BUFFER_FORMAT: u32 = 0x00d6_1d00;
/// `hkpWorldViewerBase`'s destructor (Xbox PDB, `00ddcc60`, thiscall, no arguments; it answers its
/// `EAX`).
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The arrow drawing `00d67a50` (cdecl, eight words) and the largest-component finder `00d679c0`
/// (cdecl, one pointer), both in `run_00d678e0`.
const DRAW_ARROW: u32 = 0x00d6_7a50;
const LARGEST_COMPONENT: u32 = 0x00d6_79c0;
/// The format `"%d(%d)"` (`010d1e78`), the one at `01020764` (`"%d"`) and the float the text is
/// drawn with (`0101e2bc`, 0.1 in the exe).
const FORMAT_TWO_COUNTS: u32 = 0x010d_1e78;
const FORMAT_ONE_COUNT: u32 = 0x0102_0764;
const TEXT_SCALE: u32 = 0x0101_e2bc;
/// The vtable pointers of the TOI count viewer (RTTI: `hkpToiCountViewer`): the live ones at
/// `+0`, `+8`, `+0x20`, `+0x28`, `+0x2c` and `+0x30`, and the three listener base class ones the
/// destructor leaves (`0102f13c`: `hkpEntityListener`; `010ccf7c`: the post-simulation
/// listener's; `0104dd7c`).
const VTABLE_VIEWER: u32 = 0x010d_1e6c;
const VTABLE_PROCESS: u32 = 0x010d_1e4c;
const VTABLE_FILTER: u32 = 0x010d_1e3c;
const VTABLE_FIRST_LISTENER: u32 = 0x010d_1e1c;
const VTABLE_SECOND_LISTENER: u32 = 0x010d_1e0c;
const VTABLE_THIRD_LISTENER: u32 = 0x010d_1df0;
const ENTITY_LISTENER_BASE_VTABLE: u32 = 0x0102_f13c;
const POST_SIMULATION_LISTENER_BASE_VTABLE: u32 = 0x010c_cf7c;
const THIRD_LISTENER_BASE_VTABLE: u32 = 0x0104_dd7c;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// An empty slot of the map holds this key.
const EMPTY_KEY: u32 = 0xffff_ffff;

/// The index of the first slot at or after `index` whose key is not empty, scanning slots of 8
/// bytes from `slot` while the index stays at or below `hash_mod` (the exe's two scan loops).
fn skip_empty_slots(e: &Engine, mut index: i32, mut slot: u32, hash_mod: i32) -> i32 {
    loop {
        if e.mem.u32(slot) != EMPTY_KEY {
            return index;
        }
        index = index.wrapping_add(1);
        slot = slot.wrapping_add(8);
        if index > hash_mod {
            return index;
        }
    }
}

// Translated from 00d67e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the TOI count viewer's sub-object at `+0x20` (`this - 0x20` is the whole viewer;
/// the map of entities is at `this + 0x14`, its hash mask at `this + 0x1c`) taking a world: collects
/// every key of the map whose word at `+8` is the world into a local array of words (growing it
/// through the router's allocator, slot `0x10`), then for each takes the listener `this + 0x10`
/// (0 when the whole viewer would be null) off the entity (`00c9c670`) and removes it from the
/// map (`00cb1fb0`), frees the local array's storage when it was grown, and takes the
/// sub-object's listeners (`this + 8` from the array at world `+0x1a0`: `00c91a20`, `this +
/// 0xc` from the post-simulation listeners: `00c918e0`; each 0 for a null whole viewer) away
/// from the world.
pub fn fn_00d67e30(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let map = at.wrapping_add(0x14);
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array.wrapping_add(8), 0x8000_0000);
        let hash_mod = e.mem.i32(at.wrapping_add(0x1c));
        let mut index = 0i32;
        if hash_mod >= 0 {
            let data = e.mem.u32(map);
            index = skip_empty_slots(e, index, data, hash_mod);
        }
        let mut count = 0u32;
        if index <= e.mem.i32(at.wrapping_add(0x1c)) {
            loop {
                let data = e.mem.u32(map);
                let key = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8)));
                if e.mem.u32(key.wrapping_add(8)) == world {
                    push_word_with(e, array, key, 0x10);
                    count = e.mem.u32(array.wrapping_add(4));
                }
                let hash_mod = e.mem.i32(map.wrapping_add(8));
                let mut next = index.wrapping_add(1);
                if next <= hash_mod {
                    let slot = e.mem.u32(map).wrapping_add((next as u32).wrapping_mul(8));
                    next = skip_empty_slots(e, next, slot, hash_mod);
                }
                index = next;
                if index > e.mem.i32(at.wrapping_add(0x1c)) {
                    break;
                }
            }
        }
        let mut position = 0u32;
        if (count as i32) > 0 {
            loop {
                let data = e.mem.u32(array);
                let element = data.wrapping_add(position.wrapping_mul(4));
                let listener = if at.wrapping_sub(0x20) != 0 {
                    at.wrapping_add(0x10)
                } else {
                    0
                };
                let key = e.mem.u32(element);
                e.call(ENTITY_REMOVE_LISTENER, &args![key, listener]);
                let key = e.mem.u32(element);
                e.call(MAP_REMOVE, &args![map, key]);
                position = position.wrapping_add(1);
                if (position as i32) >= (count as i32) {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        let capacity = e.mem.u32(array.wrapping_add(8));
        e.mem.set_u32(array.wrapping_add(4), 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
    let whole = at.wrapping_sub(0x20);
    let first = if whole != 0 { at.wrapping_add(8) } else { 0 };
    e.call(
        WORLD_REMOVE_ISLAND_ACTIVATION_LISTENER,
        &args![world, first],
    );
    let second = if whole != 0 { at.wrapping_add(0xc) } else { 0 };
    e.call(WORLD_REMOVE_POST_SIMULATION_LISTENER, &args![world, second]);
}

// Translated from 00d67fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the TOI count viewer (`ECX` = `this`): puts its vtable pointers back
/// (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`, `+0x30`), runs `00d67e30` on `this + 0x20` with each
/// of the worlds of the container at `[this + 0x24]` (a count at `+0x70`, the array at `+0x6c`;
/// the container is read again after each call and the loop ends when it has gone null),
/// clears the map at `this + 0x34` (`clearAndDeallocate` with the router's allocator at
/// `+0x10`, then `00a29680`), stores the listener base vtables (`+0x30`, `+0x2c`, `+0x28`) and
/// ends in the `hkpWorldViewerBase` destructor (`00ddcc60`), whose `EAX` it answers.
pub fn fn_00d67fa0(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let sub_object = at.wrapping_add(0x20);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(sub_object, VTABLE_FILTER);
    e.mem.set_u32(at.wrapping_add(0x28), VTABLE_FIRST_LISTENER);
    e.mem.set_u32(at.wrapping_add(0x2c), VTABLE_SECOND_LISTENER);
    e.mem.set_u32(at.wrapping_add(0x30), VTABLE_THIRD_LISTENER);
    let mut index = 0i32;
    let mut container = e.mem.u32(at.wrapping_add(0x24));
    while container != 0 && index < e.mem.i32(container.wrapping_add(0x70)) {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e
            .mem
            .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
        fn_00d67e30(e, Ptr::new(sub_object), Ptr::new(world));
        index = index.wrapping_add(1);
        container = e.mem.u32(at.wrapping_add(0x24));
    }
    let map = at.wrapping_add(0x34);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(EMPTY_DESTRUCTOR, &args![map]);
    e.mem
        .set_u32(at.wrapping_add(0x30), ENTITY_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x28), THIRD_LISTENER_BASE_VTABLE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

// Translated from 00d68030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d68030 - the upper three bytes of the flag word pushed for the two calls are the stale ECX left by 00d61d00 (the exe stores only the low byte)
/// Shows the TOI count of one entity (`this` is the TOI count viewer; the map is at `this +
/// 0x34`, its hash mask at `this + 0x3c`): looks `entity` up (`00cb18f0`)
/// and, when it is in the map, formats the value's top byte `h` and low 24 bits `n` as
/// `"%d(%d)"` (`n`, `h`; `00d61d00`) when `h` is not zero and as `"%d"` (`n`) otherwise into a
/// local string buffer (`00cbbd80`); asks the object at `this + 8` for its process id (the
/// method at vtable offset 4, handing it the six words `(entity, face, text, 0xff800000, 0.1,
/// flag)`, which the exe leaves on its stack: the callee is a plain `RET` that ignores them);
/// draws the text with `00d67a50` (`[this + 0x18]`, the id, and those same six words: the float
/// is the one at `0101e2bc`, quiet through `FLD`/`FSTP`, and the last word has the low byte 1 and
/// above it the stale `ECX` of `00d61d00`, which is not modelled: zero here and read as a byte only); writes the count `n` back as the map value (the top byte cleared) and frees
/// the string buffer's storage when it is owned (the router's allocator at `+0xc`, the data
/// and its capacity). Nothing else happens when the entity is not in the map.
pub fn fn_00d68030(e: &mut Engine, this: Ptr, entity: u32, face: i32) {
    let at = this.addr();
    e.with_stack(0x90, |e, frame| {
        let slot = frame.addr();
        let buffer = slot.wrapping_add(4);
        let map = at.wrapping_add(0x34);
        let index = e.call(MAP_FIND_KEY, &args![map, entity]).i32();
        e.mem.set_u32(slot, index as u32);
        if index > e.mem.i32(at.wrapping_add(0x3c)) {
            return;
        }
        let data = e.mem.u32(map);
        let value = e.mem.u32(
            data.wrapping_add((index as u32).wrapping_mul(8))
                .wrapping_add(4),
        );
        let high = value >> 0x18;
        e.call(STRING_BUFFER_CONSTRUCT, &args![buffer]);
        let count = value & 0x00ff_ffff;
        if high != 0 {
            e.call(
                STRING_BUFFER_FORMAT,
                &args![buffer, FORMAT_TWO_COUNTS, count, high],
            );
        } else {
            e.call(
                STRING_BUFFER_FORMAT,
                &args![buffer, FORMAT_ONE_COUNT, count],
            );
        }
        let scale = x87_quiet_bits(e.mem.u32(TEXT_SCALE));
        // The word is the stale `ECX` (left behind by `00d61d00`) with its low byte set to 1; the
        // upper three bytes are not modelled and both users read the low byte only.
        let flag = 1u32;
        let text = e.mem.u32(buffer);
        let tag = e
            .vcall(
                at.wrapping_add(8),
                4,
                &args![entity, face, text, 0xff80_0000u32, scale, flag],
            )
            .u32();
        let handler = e.mem.u32(at.wrapping_add(0x18));
        e.call(
            DRAW_ARROW,
            &args![
                handler,
                tag,
                entity,
                face,
                text,
                0xff80_0000u32,
                f32::from_bits(scale),
                flag
            ],
        );
        let data = e.mem.u32(map);
        let index = e.mem.u32(slot);
        e.mem.set_u32(
            data.wrapping_add(index.wrapping_mul(8)).wrapping_add(4),
            count,
        );
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0xc));
        let capacity = e.mem.u32(buffer.wrapping_add(8));
        e.mem.set_u32(buffer.wrapping_add(4), 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(buffer);
            e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
        }
    });
}

// Translated from 00d68130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the TOI counts of a world's entities (`this` is the sub-object at `+0x2c` of the TOI
/// count viewer, whose vtable is `010d1e0c`; the viewer itself is `this - 0x2c`): finds the index
/// of the largest component of the vector at `world + 0x10` (`00d679c0`), then runs `00d68030` on `this - 0x2c` with every entity of every
/// island in the world's array at `+0x28` (count at `+0x2c`) and of the island at `+0x20`
/// (the entity array of an island is at `+0x48`), each with that index.
pub fn fn_00d68130(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let world = world.addr();
    let axis = e
        .call(LARGEST_COMPONENT, &args![world.wrapping_add(0x10)])
        .i32();
    let target = Ptr::new(at.wrapping_sub(0x2c));
    let mut index = 0i32;
    if e.mem.i32(world.wrapping_add(0x2c)) > 0 {
        loop {
            let islands = e.mem.u32(world.wrapping_add(0x28));
            let island = e
                .mem
                .u32(islands.wrapping_add((index as u32).wrapping_mul(4)));
            show_island(e, target, island, axis);
            index = index.wrapping_add(1);
            if index >= e.mem.i32(world.wrapping_add(0x2c)) {
                break;
            }
        }
    }
    let island = e.mem.u32(world.wrapping_add(0x20));
    show_island(e, target, island, axis);
}

/// Runs `00d68030` on `target` for every entity of the island (the array at `+0x48`, count at
/// `+0x4c`, read again each time).
fn show_island(e: &mut Engine, target: Ptr, island: u32, axis: i32) {
    let array = island.wrapping_add(0x48);
    if e.mem.i32(array.wrapping_add(4)) <= 0 {
        return;
    }
    let mut position = 0u32;
    loop {
        let data = e.mem.u32(array);
        let entity = e.mem.u32(data.wrapping_add(position.wrapping_mul(4)));
        fn_00d68030(e, target, entity, axis);
        position = position.wrapping_add(1);
        if (position as i32) >= e.mem.i32(array.wrapping_add(4)) {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_7e30, fn_00d67e30(Ptr, Ptr)),
        entry!(0x00d6_7fa0, fn_00d67fa0(Ptr) -> u32),
        entry!(0x00d6_8030, fn_00d68030(Ptr, u32, i32)),
        entry!(0x00d6_8130, fn_00d68130(Ptr, Ptr)),
        entry!(0x00d6_8230, deleting_destructor_00d68230(Ptr, u8) -> Ptr),
        entry!(0x00d6_81e0, fn_00d681e0(Ptr, u32) -> u32),
        entry!(0x00d6_81f0, fn_00d681f0(Ptr, u32) -> u32),
        entry!(0x00d6_8200, fn_00d68200(Ptr, u32) -> u32),
        entry!(0x00d6_8210, fn_00d68210(Ptr, u32) -> u32),
        entry!(0x00d6_8220, fn_00d68220(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d68230_frees_only_when_bit_zero_is_set() {
        check(0x00d6_8230, 0x00d6_7fa0);
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
    fn le81_thunk_00d681e0() {
        assert_eq!(
            check(0x00d6_81e0, 0x00d6_8230, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
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
    fn thunk_00d681f0() {
        assert_eq!(check(0x00d6_81f0, 0x00d6_8230, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d68200() {
        assert_eq!(check(0x00d6_8200, 0x00d6_8230, 0x28), 0xdead_beef);
    }

    #[test]
    fn thunk_00d68210() {
        assert_eq!(check(0x00d6_8210, 0x00d6_8230, 0x2c), 0xdead_beef);
    }

    #[test]
    fn thunk_00d68220() {
        assert_eq!(check(0x00d6_8220, 0x00d6_8230, 0x30), 0xdead_beef);
    }
}

#[cfg(test)]
mod toi_count_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ROUTER: u32 = 0x0900_0100;
    const ALLOCATOR: u32 = 0x0900_0120;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0140;
    const FREE_ARRAY: u32 = 0x0900_0203;
    const RESERVE: u32 = 0x00c9_0940;

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
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        // Both allocator slots of the router point at the same allocator.
        e.mem.set_u32(ROUTER + 0xc, ALLOCATOR);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, FREE_ARRAY);
        record(&mut e, &log, MEMORY_ROUTER, ROUTER);
        for address in [
            FREE_ARRAY,
            WORLD_REMOVE_ISLAND_ACTIVATION_LISTENER,
            WORLD_REMOVE_POST_SIMULATION_LISTENER,
            ENTITY_REMOVE_LISTENER,
            MAP_REMOVE,
            MAP_CLEAR_AND_DEALLOCATE,
            EMPTY_DESTRUCTOR,
        ] {
            record(&mut e, &log, address, 0);
        }
        record(&mut e, &log, WORLD_VIEWER_DESTRUCT, 0x99);
        let seen = Rc::clone(&log);
        e.register_double(RESERVE, move |e, a| {
            seen.borrow_mut().push((RESERVE, a.to_vec()));
            let array = a[1];
            let data = e.mem.alloc(16);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 4);
            Ret::default()
        });
        (e, log)
    }

    fn calls(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a != MEMORY_ROUTER)
            .cloned()
            .collect()
    }

    /// The sub-object at `+0x20` of a viewer whose map (data at `+0x14` of the sub-object, hash
    /// mask at `+0x1c`) has the slots `keys` (`EMPTY_KEY` for an empty one).
    fn sub_object(e: &mut Engine, keys: &[u32]) -> u32 {
        let block = e.mem.alloc(0x80);
        let this = block + 0x20;
        let data = e.mem.alloc(8 * keys.len() as u32 + 8);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *key);
        }
        e.mem.set_u32(this + 0x14, data);
        e.mem
            .set_u32(this + 0x1c, (keys.len() as u32).wrapping_sub(1));
        this
    }

    fn entity_of(e: &mut Engine, world: u32) -> u32 {
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, world);
        entity
    }

    #[test]
    fn fn_00d67e30_removes_the_entities_of_the_world_from_the_map() {
        let (mut e, log) = rig();
        let (world, other) = (0x7000, 0x7100);
        let first = entity_of(&mut e, world);
        let second = entity_of(&mut e, other);
        let third = entity_of(&mut e, world);
        let this = sub_object(
            &mut e,
            &[EMPTY_KEY, first, EMPTY_KEY, second, third, EMPTY_KEY],
        );
        e.call(0x00d6_7e30, &args![this, world]);
        let seen = calls(&log);
        let reserve = seen.iter().find(|(a, _)| *a == RESERVE).unwrap();
        assert_eq!(reserve.1[0], ALLOCATOR);
        assert_eq!(reserve.1[2], 4);
        // Each match is taken off the entity and out of the map, in map order.
        let removals: Vec<(u32, Vec<u32>)> = seen
            .iter()
            .filter(|(a, _)| *a == ENTITY_REMOVE_LISTENER || *a == MAP_REMOVE)
            .cloned()
            .collect();
        assert_eq!(
            removals,
            vec![
                (ENTITY_REMOVE_LISTENER, vec![first, this + 0x10]),
                (MAP_REMOVE, vec![this + 0x14, first]),
                (ENTITY_REMOVE_LISTENER, vec![third, this + 0x10]),
                (MAP_REMOVE, vec![this + 0x14, third]),
            ]
        );
        // The grown local array is freed (4 words), then the two listeners come off the world.
        let tail: Vec<_> = seen
            .iter()
            .skip_while(|(a, _)| *a != FREE_ARRAY)
            .map(|(a, v)| (*a, v[1..].to_vec()))
            .collect();
        assert_eq!(seen.iter().filter(|(a, _)| *a == FREE_ARRAY).count(), 1);
        assert_eq!(
            tail[1],
            (WORLD_REMOVE_ISLAND_ACTIVATION_LISTENER, vec![this + 8])
        );
        assert_eq!(
            tail[2],
            (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![this + 0xc])
        );
        assert_eq!(seen.last().unwrap().1[0], world);
    }

    #[test]
    fn fn_00d67e30_without_matches_frees_nothing() {
        let (mut e, log) = rig();
        let other = entity_of(&mut e, 0x7100);
        let this = sub_object(&mut e, &[other, EMPTY_KEY]);
        e.call(0x00d6_7e30, &args![this, 0x7000u32]);
        let seen = calls(&log);
        assert!(seen.iter().all(|(a, _)| *a != RESERVE && *a != FREE_ARRAY));
        assert!(seen.iter().all(|(a, _)| *a != MAP_REMOVE));
        assert_eq!(seen.len(), 2);
        // An empty map (mask -1): the scan is skipped altogether.
        log.borrow_mut().clear();
        let empty = sub_object(&mut e, &[]);
        e.call(0x00d6_7e30, &args![empty, 0x7000u32]);
        assert_eq!(calls(&log).len(), 2);
    }

    #[test]
    fn fn_00d67fa0_takes_the_worlds_off_clears_the_map_and_calls_the_base_destructor() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x80);
        // The map of the sub-object at +0x20: data at +0x34, hash mask -1 at +0x3c.
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0x34, data);
        e.mem.set_u32(this + 0x3c, 0xffff_ffff);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x7000);
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x70, 1);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x24, container);
        let got = e.call(0x00d6_7fa0, &args![this]).u32();
        assert_eq!(got, 0x99);
        assert_eq!(
            calls(&log),
            vec![
                (
                    WORLD_REMOVE_ISLAND_ACTIVATION_LISTENER,
                    vec![0x7000, this + 0x28]
                ),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![0x7000, this + 0x2c]
                ),
                (MAP_CLEAR_AND_DEALLOCATE, vec![this + 0x34, ALLOCATOR]),
                (EMPTY_DESTRUCTOR, vec![this + 0x34]),
                (WORLD_VIEWER_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), THIRD_LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), ENTITY_LISTENER_BASE_VTABLE);
        // Without a container only the map and the base destructor run.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00d6_7fa0, &args![this]);
        assert_eq!(calls(&log).len(), 3);
    }

    const PROCESS_OBJECT_VTABLE: u32 = 0x0900_0160;
    const GET_TAG: u32 = 0x0900_0204;
    const STRING_DATA_OFFSET: u32 = 0xc;

    /// A viewer for the text drawing: the map (slot 1 holds `value`), the process sub-object at
    /// `+8` whose slot 4 answers the tag `0x7777`, a handler at `+0x18`, and doubles for the string
    /// buffer, the lookup and the arrow drawing.
    fn text_rig(value: u32) -> (Engine, Log, u32) {
        let (mut e, log) = rig();
        e.mem.set_u32(PROCESS_OBJECT_VTABLE + 4, GET_TAG);
        record(&mut e, &log, GET_TAG, 0x7777);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 8, PROCESS_OBJECT_VTABLE);
        e.mem.set_u32(this + 0x18, 0x5ee5);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 8 + 4, value);
        e.mem.set_u32(this + 0x34, data);
        e.mem.set_u32(this + 0x3c, 3);
        e.mem.set_u32(TEXT_SCALE, 0.1f32.to_bits());
        let seen = Rc::clone(&log);
        e.register_double(MAP_FIND_KEY, move |_, a| {
            seen.borrow_mut().push((MAP_FIND_KEY, a.to_vec()));
            Ret {
                eax: if a[1] == 0xe1 { 1 } else { 4 },
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STRING_BUFFER_CONSTRUCT, move |e, a| {
            seen.borrow_mut()
                .push((STRING_BUFFER_CONSTRUCT, a.to_vec()));
            // An inline buffer: data at +0xc, size 1, capacity word 0x80000080.
            e.mem.set_u32(a[0], a[0] + STRING_DATA_OFFSET);
            e.mem.set_u32(a[0] + 4, 1);
            e.mem.set_u32(a[0] + 8, 0x8000_0080);
            Ret::default()
        });
        record(&mut e, &log, STRING_BUFFER_FORMAT, 0);
        record(&mut e, &log, DRAW_ARROW, 0);
        record(&mut e, &log, LARGEST_COMPONENT, 3);
        (e, log, this)
    }

    #[test]
    fn fn_00d68030_draws_the_two_counts_and_writes_the_low_count_back() {
        // Top byte 2, low 24 bits 5.
        let (mut e, log, this) = text_rig(0x0200_0005);
        e.call(0x00d6_8030, &args![this, 0xe1u32, 3i32]);
        let seen = calls(&log);
        let buffer = seen
            .iter()
            .find(|(a, _)| *a == STRING_BUFFER_CONSTRUCT)
            .unwrap()
            .1[0];
        assert_eq!(seen[0], (MAP_FIND_KEY, vec![this + 0x34, 0xe1]));
        assert!(seen.contains(&(STRING_BUFFER_FORMAT, vec![buffer, FORMAT_TWO_COUNTS, 5, 2])));
        // The process id is asked for with the six words the arrow drawing then reuses.
        let flag = 1u32;
        let scale = 0.1f32.to_bits();
        assert!(seen.contains(&(
            GET_TAG,
            vec![this + 8, 0xe1, 3, buffer + 0xc, 0xff80_0000, scale, flag]
        )));
        assert!(seen.contains(&(
            DRAW_ARROW,
            vec![
                0x5ee5,
                0x7777,
                0xe1,
                3,
                buffer + 0xc,
                0xff80_0000,
                scale,
                flag
            ]
        )));
        // The value's top byte is cleared.
        let data = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(data + 8 + 4), 5);
        // The inline buffer is not owned: nothing is freed.
        assert!(seen.iter().all(|(a, _)| *a != FREE_ARRAY));
        assert_eq!(e.mem.u32(buffer + 4), 0);
    }

    #[test]
    fn fn_00d68030_draws_one_count_and_frees_a_grown_buffer() {
        let (mut e, log, this) = text_rig(0x0000_0007);
        // The buffer's capacity word says the storage is owned (bit 31 clear): it is freed
        // through the router's allocator at +0xc with the data and the capacity.
        let seen = Rc::clone(&log);
        e.register_double(STRING_BUFFER_CONSTRUCT, move |e, a| {
            seen.borrow_mut()
                .push((STRING_BUFFER_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], 0x2222_0000);
            e.mem.set_u32(a[0] + 4, 1);
            e.mem.set_u32(a[0] + 8, 0x40);
            Ret::default()
        });
        e.call(0x00d6_8030, &args![this, 0xe1u32, 1i32]);
        let seen = calls(&log);
        let buffer = seen
            .iter()
            .find(|(a, _)| *a == STRING_BUFFER_CONSTRUCT)
            .unwrap()
            .1[0];
        assert!(seen.contains(&(STRING_BUFFER_FORMAT, vec![buffer, FORMAT_ONE_COUNT, 7])));
        assert!(seen.contains(&(FREE_ARRAY, vec![ALLOCATOR, 0x2222_0000, 0x40])));
        let data = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(data + 8 + 4), 7);
    }

    #[test]
    fn fn_00d68030_does_nothing_for_an_entity_that_is_not_in_the_map() {
        let (mut e, log, this) = text_rig(0x0200_0005);
        e.call(0x00d6_8030, &args![this, 0xe2u32, 3i32]);
        assert_eq!(calls(&log), vec![(MAP_FIND_KEY, vec![this + 0x34, 0xe2])]);
    }

    #[test]
    fn fn_00d68130_shows_every_entity_with_the_largest_axis_of_the_world() {
        let (mut e, log, viewer) = text_rig(0x0200_0005);
        let this = viewer + 0x2c;
        let island_entities = |e: &mut Engine, entities: &[u32]| {
            let island = e.mem.alloc(0x60);
            let data = e.mem.alloc(4 * entities.len() as u32 + 4);
            for (i, v) in entities.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *v);
            }
            e.mem.set_u32(island + 0x48, data);
            e.mem.set_u32(island + 0x4c, entities.len() as u32);
            island
        };
        let world = e.mem.alloc(0x80);
        let first = island_entities(&mut e, &[0xe2, 0xe3]);
        let second = island_entities(&mut e, &[0xe4]);
        let single = island_entities(&mut e, &[0xe5]);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(world + 0x28, list);
        e.mem.set_u32(world + 0x2c, 2);
        e.mem.set_u32(world + 0x20, single);
        e.call(0x00d6_8130, &args![this, world]);
        let seen = calls(&log);
        assert_eq!(seen[0], (LARGEST_COMPONENT, vec![world + 0x10]));
        // The double finds none of the entities (0xe2 and up are not 0xe1): each is one lookup.
        let lookups: Vec<u32> = seen
            .iter()
            .filter(|(a, _)| *a == MAP_FIND_KEY)
            .map(|(_, v)| v[1])
            .collect();
        assert_eq!(lookups, vec![0xe2, 0xe3, 0xe4, 0xe5]);
        assert!(seen
            .iter()
            .filter(|(a, _)| *a == MAP_FIND_KEY)
            .all(|(_, v)| v[0] == viewer + 0x34));
    }
}
