//! `Havok SDK/hkpphysicscontext.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Object layout, as far as these functions show it (`this` = the physics context):
//! `+0` and `+8` are vtables (the second one belongs to the embedded process context, whose
//! enabled word is at `+0xc` and whose two small arrays start at `+0x10` and `+0x34`); `+0x58`
//! (world deletion listener), `+0x5c` (entity listener), `+0x60` (phantom listener), `+0x64`
//! (constraint listener) and `+0x68` (action listener) are listener sub-objects; `+0x6c`
//! (data), `+0x70` (size), `+0x74` (capacity word) is the array of worlds, `+0x78`/`+0x7c`/
//! `+0x80` the array of 0xf0-byte world records and `+0x84`/`+0x88`/`+0x8c` the array of
//! context listeners. An array's capacity word has bit 31 set when the storage is not owned
//! (it is then never freed) and the capacity in the low 30 bits.
//!
//! The kinds of the four listener sub-objects follow from the names the listener thunks pass
//! on (`Entities`, `Phantoms`, `Constraints`, `Actions`); the names of the world functions
//! that take them are not confirmed in Ghidra.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator, whose virtual function
/// at offset `0x10` frees `(data, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArray::_reserveMore` (cdecl `(allocator, array, element size)`).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// The exe's `operator delete` (cdecl, one pointer).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Releases the reference to the object (`this`) the world's listener tables came as.
const RELEASE: u32 = 0x00c9_05b0;
/// The world's listener tables (`this` = world; returns a block with the entity array at
/// `+8`/`+0xc`, the constraint array at `+0x14`/`+0x18`, the action array at `+0x20`/`+0x24`
/// and the phantom array at `+0x2c`/`+0x30`, each (data, size)).
const WORLD_LISTENER_TABLES: u32 = 0x00c9_29e0;
/// The world calls that hook a listener sub-object (`this` = world, `(listener)`), by kind.
const REMOVE_ENTITY_LISTENER: u32 = 0x00c9_17a0;
const REMOVE_PHANTOM_LISTENER: u32 = 0x00c9_17f0;
const REMOVE_ACTION_LISTENER: u32 = 0x00c9_1700;
const REMOVE_CONSTRAINT_LISTENER: u32 = 0x00c9_1750;
const ADD_ENTITY_LISTENER: u32 = 0x00c9_3820;
const ADD_PHANTOM_LISTENER: u32 = 0x00c9_3860;
const ADD_ACTION_LISTENER: u32 = 0x00c9_37a0;
const ADD_CONSTRAINT_LISTENER: u32 = 0x00c9_37e0;
/// The world calls for the deletion listener (`this + 0x58`).
const REMOVE_DELETION_LISTENER: u32 = 0x00c9_1a70;
const ADD_DELETION_LISTENER: u32 = 0x00c9_3a60;
/// The two calls around changing a world (`this` = world, no arguments).
const LOCK_WORLD: u32 = 0x00c9_1180;
const UNLOCK_WORLD: u32 = 0x00c9_1190;
/// Fills a 0xf0-byte world record from the world (`this` = world, `(record)`) after the
/// constructor `00c90b80` (`this` = the record) ran on it.
const GET_WORLD_RECORD: u32 = 0x00c9_1c50;
const WORLD_RECORD_CONSTRUCTOR: u32 = 0x00c9_0b80;
/// The two calls on the object at `+0xc` (`this`) that announce and withdraw an object
/// (`00cd1330`: `(object, class record, name, extra)`; `00cd10a0`: `(object)`).
const ANNOUNCE_OBJECT: u32 = 0x00cd_1330;
const WITHDRAW_OBJECT: u32 = 0x00cd_10a0;

/// The class records and names the announce call gets for the four kinds of object and for
/// the world records.
const ENTITIES: (u32, u32) = (0x0126_88c8, 0x010c_41c4);
const PHANTOMS: (u32, u32) = (0x0126_902c, 0x010c_41a4);
const CONSTRAINTS: (u32, u32) = (0x0126_908c, 0x010c_b200);
const ACTIONS: (u32, u32) = (0x0126_90d4, 0x010c_41dc);
const WORLD_RECORDS: (u32, u32) = (0x0126_9104, 0x010c_b20c);

/// The vtables of the listener sub-objects before and after the derived class takes them
/// over, the process context's and the context's own.
const WORLD_DELETION_LISTENER_VTABLE: u32 = 0x010c_b1c4;
const ENTITY_LISTENER_BASE_VTABLE: u32 = 0x0102_f13c;
const PHANTOM_LISTENER_BASE_VTABLE: u32 = 0x0104_e3d0;
const CONSTRAINT_LISTENER_VTABLE: u32 = 0x010c_b1e4;
const ACTION_LISTENER_VTABLE: u32 = 0x010c_b1d4;
const PROCESS_CONTEXT_BASE_VTABLE: u32 = 0x010c_b220;
const CONTEXT_VTABLE: u32 = 0x010c_b2b4;
const PROCESS_CONTEXT_VTABLE: u32 = 0x010c_b2a4;
const DELETION_LISTENER_FINAL_VTABLE: u32 = 0x010c_b294;
const ENTITY_LISTENER_FINAL_VTABLE: u32 = 0x010c_b278;
const PHANTOM_LISTENER_FINAL_VTABLE: u32 = 0x010c_b260;
const CONSTRAINT_LISTENER_FINAL_VTABLE: u32 = 0x010c_b240;
const ACTION_LISTENER_FINAL_VTABLE: u32 = 0x010c_b230;
/// The vtable the destructor leaves in the object (`0102e388`).
const DESTROYED_VTABLE: u32 = 0x0102_e388;

/// The four listener kinds: where the world's tables keep the data and the size of the array
/// the kind is announced for, and where the listener sub-object sits in the context.
const LISTENER_KINDS: [(u32, u32, u32); 4] = [
    (0x08, 0x0c, 0x5c),
    (0x2c, 0x30, 0x60),
    (0x20, 0x24, 0x68),
    (0x14, 0x18, 0x64),
];

/// The index of `value` in the word array `array` (data at +0, size at +4); the size and the
/// data are read once, as the exe does.
fn find_word(e: &Engine, array: u32, value: u32) -> Option<u32> {
    let size = e.mem.i32(array + 4);
    if size <= 0 {
        return None;
    }
    let data = e.mem.u32(array);
    (0..size as u32).find(|index| e.mem.u32(data.wrapping_add(4 * index)) == value)
}

/// Appends `value` to the word array `array` (data +0, size +4, capacity word +8), growing it
/// through `_reserveMore` when the size has reached the capacity.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Removes the entry `index` of the word array `array` by moving the last entry into its place.
fn remove_word_swap_last(e: &mut Engine, array: u32, index: u32) {
    let size = e.mem.u32(array + 4).wrapping_sub(1);
    e.mem.set_u32(array + 4, size);
    if size != index {
        let data = e.mem.u32(array);
        let last = e.mem.u32(data.wrapping_add(4 * size));
        e.mem.set_u32(data.wrapping_add(4 * index), last);
    }
}

/// Calls the virtual function at offset `slot` of each listener sub-object of the context for
/// every entry in the four arrays of the world's listener tables.
fn notify_listener_kinds(e: &mut Engine, this: u32, tables: u32, slot: u32) {
    for (data_at, size_at, listener) in LISTENER_KINDS {
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(tables + size_at) {
            let data = e.mem.u32(tables + data_at);
            let item = e.mem.u32(data.wrapping_add(4 * index));
            e.vcall(this + listener, slot, &args![item]);
            index += 1;
        }
    }
}

/// Frees the storage of a word array the way the destructors do: through the allocator's
/// virtual function at offset 0x10 `(data, bytes)` unless the capacity word `capacity` has bit
/// 31 set. `element_size` is the size of an entry.
fn free_storage(e: &mut Engine, allocator: u32, data: u32, capacity: u32, element_size: u32) {
    if capacity & 0x8000_0000 == 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00cd1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldDeletionListener::_scalar_deleting_destructor_` (engine map): stores the vtable
/// `010cb1c4` and, when bit 0 of `flags` is set, deletes the object (`operator delete`).
/// Returns `this`.
pub fn hkp_world_deletion_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), WORLD_DELETION_LISTENER_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr()]);
    }
    this
}

// Translated from 00cd1e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpActionListener::_scalar_deleting_destructor_` (engine map): as above with the vtable
/// `010cb1d4`.
pub fn hkp_action_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), ACTION_LISTENER_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr()]);
    }
    this
}

// Translated from 00cd1eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintListener::_scalar_deleting_destructor_` (engine map): as above with the
/// vtable `010cb1e4`.
pub fn hkp_constraint_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), CONSTRAINT_LISTENER_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr()]);
    }
    this
}

/// The nineteen viewer registrations in the order `registerAllPhysicsProcesses` calls them
/// (every one is a function without arguments; the last is a tail call). The Ghidra names, in
/// the same order: broadphase, midphase, rigid body centre of mass, constraint, convex radius,
/// active contact point, inactive contact point, TOI contact point, TOI count, rigid body
/// inertia and mouse picking viewers; two without a name (`00d66af0`, `00d65950`); then the
/// simulation island, swept transform display, vehicle, world memory, world snapshot and
/// welding viewers.
const VIEWER_REGISTRATIONS: [u32; 19] = [
    0x00d6_ad40,
    0x00d6_a850,
    0x00d6_a180,
    0x00d6_9a00,
    0x00d6_8e40,
    0x00d6_88d0,
    0x00d6_8800,
    0x00d6_86f0,
    0x00d6_7e10,
    0x00d6_78c0,
    0x00d6_6c90,
    0x00d6_6af0,
    0x00d6_5950,
    0x00d6_51c0,
    0x00d6_4630,
    0x00d6_3da0,
    0x00d6_3400,
    0x00d6_3000,
    0x00d6_2e60,
];

// Translated from 00cd1ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsContext::registerAllPhysicsProcesses` (engine map): calls the nineteen
/// `registerViewer` functions in order (`VIEWER_REGISTRATIONS`).
pub fn hkp_physics_context_register_all_physics_processes(e: &mut Engine) {
    for registration in VIEWER_REGISTRATIONS {
        e.call(registration, &args![]);
    }
}

// Translated from 00cd1f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entity listener thunk (role from the name it passes, unconfirmed; `this` = the listener
/// sub-object at `+0x5c`, one stack word `object`): if the object at `this - 0x50` (the
/// context's `+0xc`) is not null, announces `object` to it with the `Entities` class record
/// and name and `object + 0x10` as the extra word.
pub fn fn_00cd1f30(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x50));
    if owner != 0 {
        e.call(
            ANNOUNCE_OBJECT,
            &args![owner, object, ENTITIES.0, ENTITIES.1, object + 0x10],
        );
    }
}

// Translated from 00cd1f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entity listener thunk (unconfirmed role; `this` at `+0x5c`, one stack word): if the object
/// at `this - 0x50` is not null, withdraws `object` through it (tail call).
pub fn fn_00cd1f60(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x50));
    if owner != 0 {
        e.call(WITHDRAW_OBJECT, &args![owner, object]);
    }
}

// Translated from 00cd1f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Phantom listener thunk (unconfirmed role; `this` at `+0x60`, one stack word): as
/// `fn_00cd1f30` with the owner at `this - 0x54` and the `Phantoms` class record and name.
pub fn fn_00cd1f70(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x54));
    if owner != 0 {
        e.call(
            ANNOUNCE_OBJECT,
            &args![owner, object, PHANTOMS.0, PHANTOMS.1, object + 0x10],
        );
    }
}

// Translated from 00cd1fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Phantom listener thunk (unconfirmed role; `this` at `+0x60`, one stack word): as
/// `fn_00cd1f60` with the owner at `this - 0x54`.
pub fn fn_00cd1fa0(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x54));
    if owner != 0 {
        e.call(WITHDRAW_OBJECT, &args![owner, object]);
    }
}

// Translated from 00cd1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constraint listener thunk (unconfirmed role; `this` at `+0x64`, one stack word
/// `constraint`): when the owner at `this - 0x58` is not null, the object at `constraint + 0xc`
/// is not null and its virtual function at offset `0x20` does not answer `0xb`, announces
/// `constraint` to the owner with the `Constraints` class record and name and 0 as the extra
/// word.
pub fn fn_00cd1fb0(e: &mut Engine, this: Ptr, constraint: u32) {
    let me = this.addr();
    if e.mem.u32(me.wrapping_sub(0x58)) == 0 {
        return;
    }
    let data = e.mem.u32(constraint + 0xc);
    if data == 0 {
        return;
    }
    if e.vcall(data, 0x20, &args![]).u32() != 0xb {
        let owner = e.mem.u32(me.wrapping_sub(0x58));
        e.call(
            ANNOUNCE_OBJECT,
            &args![owner, constraint, CONSTRAINTS.0, CONSTRAINTS.1, 0u32],
        );
    }
}

// Translated from 00cd1ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constraint listener thunk (unconfirmed role; `this` at `+0x64`, one stack word): the same
/// test as `fn_00cd1fb0`, then withdraws `constraint` through the owner at `this - 0x58`.
pub fn fn_00cd1ff0(e: &mut Engine, this: Ptr, constraint: u32) {
    let me = this.addr();
    if e.mem.u32(me.wrapping_sub(0x58)) == 0 {
        return;
    }
    let data = e.mem.u32(constraint + 0xc);
    if data == 0 {
        return;
    }
    if e.vcall(data, 0x20, &args![]).u32() != 0xb {
        let owner = e.mem.u32(me.wrapping_sub(0x58));
        e.call(WITHDRAW_OBJECT, &args![owner, constraint]);
    }
}

// Translated from 00cd2020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Action listener thunk (unconfirmed role; `this` at `+0x68`, one stack word): if the owner at
/// `this - 0x5c` is not null, announces `object` to it with the `Actions` class record and name
/// and 0 as the extra word.
pub fn fn_00cd2020(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x5c));
    if owner != 0 {
        e.call(
            ANNOUNCE_OBJECT,
            &args![owner, object, ACTIONS.0, ACTIONS.1, 0u32],
        );
    }
}

// Translated from 00cd2040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Action listener thunk (unconfirmed role; `this` at `+0x68`, one stack word): if the owner at
/// `this - 0x5c` is not null, withdraws `object` through it (tail call).
pub fn fn_00cd2040(e: &mut Engine, this: Ptr, object: u32) {
    let owner = e.mem.u32(this.addr().wrapping_sub(0x5c));
    if owner != 0 {
        e.call(WITHDRAW_OBJECT, &args![owner, object]);
    }
}

// Translated from 00cd2080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unhooks the context from `world` (unconfirmed name; `this` = the context, one stack word).
/// Does nothing unless `this + 0xc` and `world` are non-null. Removes the four listener
/// sub-objects (`+0x5c`, `+0x60`, `+0x68`, `+0x64`) from the world. If `world` is in the array
/// of worlds (`+0x6c`) and `this + 0xc` is still non-null, withdraws the matching 0xf0-byte
/// world record (`+0x78`) through `this + 0xc`, runs its destructor (virtual function 0 with 0),
/// decrements the record count (`+0x7c`) and copies the last record over the removed one. Then
/// tells every listener kind about every entry of the world's tables (virtual function at
/// offset 8 of the listener sub-object) and releases the tables.
pub fn fn_00cd2080(e: &mut Engine, this: Ptr, world: u32) {
    let me = this.addr();
    if e.mem.u32(me + 0xc) == 0 || world == 0 {
        return;
    }
    e.call(REMOVE_ENTITY_LISTENER, &args![world, me + 0x5c]);
    e.call(REMOVE_PHANTOM_LISTENER, &args![world, me + 0x60]);
    e.call(REMOVE_ACTION_LISTENER, &args![world, me + 0x68]);
    e.call(REMOVE_CONSTRAINT_LISTENER, &args![world, me + 0x64]);
    let found = find_word(e, me + 0x6c, world);
    let owner = e.mem.u32(me + 0xc);
    if let Some(index) = found.filter(|_| owner != 0) {
        let offset = index.wrapping_mul(0xf0);
        let record = e.mem.u32(me + 0x78).wrapping_add(offset);
        e.call(WITHDRAW_OBJECT, &args![owner, record]);
        let record = e.mem.u32(me + 0x78).wrapping_add(offset);
        e.vcall(record, 0, &args![0u32]);
        let count = e.mem.u32(me + 0x7c).wrapping_sub(1);
        e.mem.set_u32(me + 0x7c, count);
        if count != index {
            let data = e.mem.u32(me + 0x78);
            let source = count.wrapping_mul(0xf0).wrapping_add(data);
            let destination = data.wrapping_add(offset);
            for word in 0..0x3c {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(destination + 4 * word, value);
            }
        }
    }
    let tables = e.call(WORLD_LISTENER_TABLES, &args![world]).u32();
    notify_listener_kinds(e, me, tables, 8);
    e.call(RELEASE, &args![tables]);
}

// Translated from 00cd2210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `listener` from the array of context listeners (`+0x84`, size `+0x88`) by moving the
/// last entry into its place (unconfirmed name; `this` = the context, one stack word). Does
/// nothing when it is not in the array.
pub fn fn_00cd2210(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x84;
    if let Some(index) = find_word(e, array, listener) {
        remove_word_swap_last(e, array, index);
    }
}

// Translated from 00cd2260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `world` from the context (unconfirmed name; `this` = the context, one stack word).
/// If `world` is in the array of worlds (`+0x6c`, size `+0x70`): removes the deletion listener
/// sub-object (`this + 0x58`, or 0 for a null `this`) from the world, tells every context
/// listener (`+0x84`; virtual function at offset 8 with the world), unhooks the world
/// (`00cd2080`) and removes the entry from the array by moving the last one into its place.
pub fn fn_00cd2260(e: &mut Engine, this: Ptr, world: u32) {
    let me = this.addr();
    let Some(index) = find_word(e, me + 0x6c, world) else {
        return;
    };
    let deletion_listener = if me == 0 { 0 } else { me + 0x58 };
    e.call(REMOVE_DELETION_LISTENER, &args![world, deletion_listener]);
    let mut at = 0u32;
    while (at as i32) < e.mem.i32(me + 0x88) {
        let listener = e.mem.u32(e.mem.u32(me + 0x84).wrapping_add(4 * at));
        e.vcall(listener, 8, &args![world]);
        at += 1;
    }
    e.call(0x00cd_2080, &args![me, world]);
    remove_word_swap_last(e, me + 0x6c, index);
}

// Translated from 00cd22f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hooks the context to `world` (unconfirmed name; `this` = the context, one stack word). Does
/// nothing unless `this + 0xc` and `world` are non-null. Locks the world, adds the four listener
/// sub-objects to it, appends a world record (`+0x78`, 0xf0 bytes; grown through
/// `_reserveMore`, constructed with `00c90b80` when its address is not null, filled by
/// `00c91c50`) and announces it through `this + 0xc` with the `hkpWorldCinfo` class record and
/// name. Then tells every listener kind about every entry of the world's tables (virtual function
/// at offset 4 of the listener sub-object), releases the tables and unlocks the world.
pub fn fn_00cd22f0(e: &mut Engine, this: Ptr, world: u32) {
    let me = this.addr();
    if e.mem.u32(me + 0xc) == 0 || world == 0 {
        return;
    }
    e.call(LOCK_WORLD, &args![world]);
    e.call(ADD_ENTITY_LISTENER, &args![world, me + 0x5c]);
    e.call(ADD_PHANTOM_LISTENER, &args![world, me + 0x60]);
    e.call(ADD_ACTION_LISTENER, &args![world, me + 0x68]);
    e.call(ADD_CONSTRAINT_LISTENER, &args![world, me + 0x64]);
    let array = me + 0x78;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(RESERVE_MORE, &args![allocator, array, 0xf0u32]);
    }
    let size = e.mem.u32(array + 4);
    let next = size.wrapping_mul(0xf0).wrapping_add(e.mem.u32(array));
    if next != 0 {
        e.call(WORLD_RECORD_CONSTRUCTOR, &args![next]);
    }
    let size = e.mem.u32(array + 4);
    let record = size.wrapping_mul(0xf0).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(GET_WORLD_RECORD, &args![world, record]);
    let owner = e.mem.u32(me + 0xc);
    if owner != 0 {
        e.call(
            ANNOUNCE_OBJECT,
            &args![owner, record, WORLD_RECORDS.0, WORLD_RECORDS.1, 0u32],
        );
    }
    let tables = e.call(WORLD_LISTENER_TABLES, &args![world]).u32();
    notify_listener_kinds(e, me, tables, 4);
    e.call(RELEASE, &args![tables]);
    e.call(UNLOCK_WORLD, &args![world]);
}

// Translated from 00cd2460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk from the deletion listener sub-object (`this` at `+0x58`, one stack word `world`):
/// jumps to `fn_00cd2260` with `this - 0x58`.
pub fn fn_00cd2460(e: &mut Engine, this: Ptr, world: u32) {
    e.call(0x00cd_2260, &args![this.addr().wrapping_sub(0x58), world]);
}

// Translated from 00cd2470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `listener` to the array of context listeners (`+0x84`) unless it is already in it
/// (unconfirmed name; `this` = the context, one stack word).
pub fn fn_00cd2470(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x84;
    if find_word(e, array, listener).is_none() {
        push_word(e, array, listener);
    }
}

// Translated from 00cd24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Switches the process context's enabled word (`this` = the embedded process context at the
/// context's `+8`, one stack word `enabled`; unconfirmed name). If it was non-zero, unhooks every
/// world of the array at the context's `+0x6c` (`fn_00cd2080` on `this - 8`). Stores the new
/// word; if it is non-zero, hooks every world (`fn_00cd22f0`).
pub fn fn_00cd24d0(e: &mut Engine, this: Ptr, enabled: u32) {
    let me = this.addr();
    let context = me.wrapping_sub(8);
    if e.mem.u32(me + 4) != 0 {
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(me + 0x68) {
            let world = e.mem.u32(e.mem.u32(me + 0x64).wrapping_add(4 * index));
            e.call(0x00cd_2080, &args![context, world]);
            index += 1;
        }
    }
    e.mem.set_u32(me + 4, enabled);
    if enabled != 0 {
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(me + 0x68) {
            let world = e.mem.u32(e.mem.u32(me + 0x64).wrapping_add(4 * index));
            e.call(0x00cd_22f0, &args![context, world]);
            index += 1;
        }
    }
}

// Translated from 00cd2530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `world` to the context (unconfirmed name; `this` = the context, one stack word). Does
/// nothing when it is already in the array of worlds (`+0x6c`). Otherwise adds the deletion
/// listener sub-object (`this + 0x58`, or 0 for a null `this`) to the world, appends the world,
/// tells every context listener (`+0x84`; virtual function at offset 4 with the world) and hooks
/// the world (`fn_00cd22f0`).
pub fn fn_00cd2530(e: &mut Engine, this: Ptr, world: u32) {
    let me = this.addr();
    if find_word(e, me + 0x6c, world).is_some() {
        return;
    }
    let deletion_listener = if me == 0 { 0 } else { me + 0x58 };
    e.call(ADD_DELETION_LISTENER, &args![world, deletion_listener]);
    push_word(e, me + 0x6c, world);
    let mut at = 0u32;
    while (at as i32) < e.mem.i32(me + 0x88) {
        let listener = e.mem.u32(e.mem.u32(me + 0x84).wrapping_add(4 * at));
        e.vcall(listener, 4, &args![world]);
        at += 1;
    }
    e.call(0x00cd_22f0, &args![me, world]);
}

// Translated from 00cd25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The physics context's constructor (unconfirmed name; `__fastcall`, `this` in `ECX`, returned
/// in `EAX`). Sets the 16-bit word at `+6` to 1, builds the process context at `+8` (vtable,
/// enabled word 0, two arrays of capacity 6 with their inline storage at `+0x1c` and `+0x40`),
/// stores the base vtables of the listener sub-objects and then the final ones, the context's
/// vtables at `+0` and `+8`, and empty arrays (data 0, size 0, capacity word `0x80000000`) at
/// `+0x6c`, `+0x78` and `+0x84`.
pub fn fn_00cd25d0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, PROCESS_CONTEXT_BASE_VTABLE);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x10, me + 0x1c);
    e.mem.set_u32(me + 0x18, 0x8000_0006);
    e.mem.set_u32(me + 0x3c, 0x8000_0006);
    e.mem.set_u32(me + 0x38, 0);
    e.mem.set_u32(me + 0x34, me + 0x40);
    e.mem.set_u32(me + 0x58, WORLD_DELETION_LISTENER_VTABLE);
    e.mem.set_u32(me + 0x5c, ENTITY_LISTENER_BASE_VTABLE);
    e.mem.set_u32(me + 0x60, PHANTOM_LISTENER_BASE_VTABLE);
    e.mem.set_u32(me + 0x64, CONSTRAINT_LISTENER_VTABLE);
    e.mem.set_u32(me + 0x68, ACTION_LISTENER_VTABLE);
    e.mem.set_u32(me, CONTEXT_VTABLE);
    e.mem.set_u32(me + 8, PROCESS_CONTEXT_VTABLE);
    e.mem.set_u32(me + 0x58, DELETION_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x5c, ENTITY_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x60, PHANTOM_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x64, CONSTRAINT_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x68, ACTION_LISTENER_FINAL_VTABLE);
    for array in [0x6c, 0x78, 0x84] {
        e.mem.set_u32(me + array, 0);
        e.mem.set_u32(me + array + 4, 0);
        e.mem.set_u32(me + array + 8, 0x8000_0000);
    }
    this
}

// Translated from 00cd2690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The physics context's destructor (unconfirmed name; `__fastcall`, `this` in `ECX`). Stores
/// the context's vtables and the final listener vtables, removes every world from the last to
/// the first (`fn_00cd2260`), frees the array of context listeners (`+0x84`), runs the
/// destructor (virtual function 0 with 0) of each 0xf0-byte world record and frees that array
/// (`+0x78`), frees the array of worlds (`+0x6c`) — each only when the capacity word has bit 31
/// clear — stores the base vtables of the listener sub-objects, destroys the embedded process
/// context (`fn_00cd27f0` on `this + 8`) and stores the vtable `0102e388`.
pub fn fn_00cd2690(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let mut index = e.mem.i32(me + 0x70).wrapping_sub(1);
    e.mem.set_u32(me, CONTEXT_VTABLE);
    e.mem.set_u32(me + 8, PROCESS_CONTEXT_VTABLE);
    e.mem.set_u32(me + 0x58, DELETION_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x5c, ENTITY_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x60, PHANTOM_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x64, CONSTRAINT_LISTENER_FINAL_VTABLE);
    e.mem.set_u32(me + 0x68, ACTION_LISTENER_FINAL_VTABLE);
    while index >= 0 {
        let world = e
            .mem
            .u32(e.mem.u32(me + 0x6c).wrapping_add(4 * index as u32));
        e.call(0x00cd_2260, &args![me, world]);
        index = index.wrapping_sub(1);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x8c);
    e.mem.set_u32(me + 0x88, 0);
    let data = e.mem.u32(me + 0x84);
    free_storage(e, allocator, data, capacity, 4);
    e.mem.set_u32(me + 0x84, 0);
    e.mem.set_u32(me + 0x8c, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(me + 0x7c);
    let mut record = e.mem.u32(me + 0x78);
    for _ in 0..count.max(0) {
        e.vcall(record, 0, &args![0u32]);
        record = record.wrapping_add(0xf0);
    }
    let capacity = e.mem.u32(me + 0x80);
    e.mem.set_u32(me + 0x7c, 0);
    let data = e.mem.u32(me + 0x78);
    free_storage(e, allocator, data, capacity, 0xf0);
    e.mem.set_u32(me + 0x78, 0);
    e.mem.set_u32(me + 0x80, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x74);
    e.mem.set_u32(me + 0x70, 0);
    let data = e.mem.u32(me + 0x6c);
    free_storage(e, allocator, data, capacity, 4);
    e.mem.set_u32(me + 0x6c, 0);
    e.mem.set_u32(me + 0x74, 0x8000_0000);
    e.mem.set_u32(me + 0x68, ACTION_LISTENER_VTABLE);
    e.mem.set_u32(me + 0x64, CONSTRAINT_LISTENER_VTABLE);
    e.mem.set_u32(me + 0x60, PHANTOM_LISTENER_BASE_VTABLE);
    e.mem.set_u32(me + 0x5c, ENTITY_LISTENER_BASE_VTABLE);
    e.mem.set_u32(me + 0x58, WORLD_DELETION_LISTENER_VTABLE);
    e.call(0x00cd_27f0, &args![me + 8]);
    e.mem.set_u32(me, DESTROYED_VTABLE);
}

// Translated from 00cd27f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process context's destructor body (unconfirmed name; `__fastcall`, `this` = the process
/// context): stores the vtable `010cb220`, then empties and frees (unless the capacity word has
/// bit 31 set) the array at `+0x2c`/`+0x30`/`+0x34` and the array at `+8`/`+0xc`/`+0x10`,
/// leaving data 0 and capacity word `0x80000000`.
pub fn fn_00cd27f0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, PROCESS_CONTEXT_BASE_VTABLE);
    for (data_at, size_at, capacity_at) in [(0x2c, 0x30, 0x34), (0x8, 0xc, 0x10)] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(me + capacity_at);
        e.mem.set_u32(me + size_at, 0);
        if capacity & 0x8000_0000 == 0 {
            let data = e.mem.u32(me + data_at);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(me + data_at, 0);
        e.mem.set_u32(me + capacity_at, 0x8000_0000);
    }
}

// Translated from 00cd2870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcessContext::_scalar_deleting_destructor_` (engine map): runs the destructor body
/// (`fn_00cd27f0`) and, when bit 0 of `flags` is set, deletes the object. Returns `this`.
pub fn hk_process_context_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00cd_27f0, &args![this.addr()]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr()]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd_1e70,
            hkp_world_deletion_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cd_1e90,
            hkp_action_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cd_1eb0,
            hkp_constraint_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cd_1ed0,
            hkp_physics_context_register_all_physics_processes()
        ),
        entry!(0x00cd_1f30, fn_00cd1f30(Ptr, u32)),
        entry!(0x00cd_1f60, fn_00cd1f60(Ptr, u32)),
        entry!(0x00cd_1f70, fn_00cd1f70(Ptr, u32)),
        entry!(0x00cd_1fa0, fn_00cd1fa0(Ptr, u32)),
        entry!(0x00cd_1fb0, fn_00cd1fb0(Ptr, u32)),
        entry!(0x00cd_1ff0, fn_00cd1ff0(Ptr, u32)),
        entry!(0x00cd_2020, fn_00cd2020(Ptr, u32)),
        entry!(0x00cd_2040, fn_00cd2040(Ptr, u32)),
        entry!(0x00cd_2080, fn_00cd2080(Ptr, u32)),
        entry!(0x00cd_2210, fn_00cd2210(Ptr, u32)),
        entry!(0x00cd_2260, fn_00cd2260(Ptr, u32)),
        entry!(0x00cd_22f0, fn_00cd22f0(Ptr, u32)),
        entry!(0x00cd_2460, fn_00cd2460(Ptr, u32)),
        entry!(0x00cd_2470, fn_00cd2470(Ptr, u32)),
        entry!(0x00cd_24d0, fn_00cd24d0(Ptr, u32)),
        entry!(0x00cd_2530, fn_00cd2530(Ptr, u32)),
        entry!(0x00cd_25d0, fn_00cd25d0(Ptr) -> Ptr),
        entry!(0x00cd_2690, fn_00cd2690(Ptr)),
        entry!(0x00cd_27f0, fn_00cd27f0(Ptr)),
        entry!(
            0x00cd_2870,
            hk_process_context_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Stand-ins for the listener sub-objects' virtual functions (slot 4 and slot 8) by kind
    /// index (entity 0, phantom 1, action 2, constraint 3).
    const LISTENER_ADDED: u32 = 0x0900_0710;
    const LISTENER_REMOVED: u32 = 0x0900_0720;
    /// Stand-ins for the context listeners' virtual functions (slot 4 and slot 8).
    const CONTEXT_LISTENER_ADDED: u32 = 0x0900_0730;
    const CONTEXT_LISTENER_REMOVED: u32 = 0x0900_0731;
    /// The world records' destructor, the allocator's free function and the two answers of
    /// a constraint's data object.
    const RECORD_DESTRUCTOR: u32 = 0x0900_0740;
    const FREE_SLOT: u32 = 0x0900_0750;
    const KIND_PLAIN: u32 = 0x0900_0760;
    const KIND_SPECIAL: u32 = 0x0900_0761;

    const REMOVE_WORLD: u32 = 0x00cd_2260;
    const UNHOOK_WORLD: u32 = 0x00cd_2080;
    const HOOK_WORLD: u32 = 0x00cd_22f0;
    const DESTROY_PROCESS_CONTEXT: u32 = 0x00cd_27f0;

    /// Every outside callee of the unit.
    const OUTSIDE: [u32; 23] = [
        OPERATOR_DELETE,
        RELEASE,
        REMOVE_ENTITY_LISTENER,
        REMOVE_PHANTOM_LISTENER,
        REMOVE_ACTION_LISTENER,
        REMOVE_CONSTRAINT_LISTENER,
        ADD_ENTITY_LISTENER,
        ADD_PHANTOM_LISTENER,
        ADD_ACTION_LISTENER,
        ADD_CONSTRAINT_LISTENER,
        REMOVE_DELETION_LISTENER,
        ADD_DELETION_LISTENER,
        LOCK_WORLD,
        UNLOCK_WORLD,
        GET_WORLD_RECORD,
        WORLD_RECORD_CONSTRUCTOR,
        ANNOUNCE_OBJECT,
        WITHDRAW_OBJECT,
        LISTENER_ADDED,
        CONTEXT_LISTENER_ADDED,
        CONTEXT_LISTENER_REMOVED,
        RECORD_DESTRUCTOR,
        FREE_SLOT,
    ];

    struct Fx {
        e: Engine,
        /// The context.
        this: u32,
        /// The object at `this + 0xc`.
        owner: u32,
        world: u32,
        tables: u32,
        allocator: u32,
    }

    fn vtable_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        table
    }

    /// A context with an owner, the four listener sub-objects (vtable slots 4 and 8 point at
    /// the `LISTENER_ADDED + kind` and `LISTENER_REMOVED + kind` stand-ins), empty arrays, a
    /// world and its (empty) listener tables, and doubles for everything outside the unit. The
    /// growing function enlarges an array to twice its size, at least 4 entries.
    fn fx() -> Fx {
        let mut e = Engine::new();
        for address in OUTSIDE {
            e.register_double(address, |_, _| Ret::default());
        }
        for kind in 1..4 {
            e.register_double(LISTENER_ADDED + kind, |_, _| Ret::default());
        }
        for kind in 0..4 {
            e.register_double(LISTENER_REMOVED + kind, |_, _| Ret::default());
        }
        for address in VIEWER_REGISTRATIONS {
            e.register_double(address, |_, _| Ret::default());
        }
        let free_table = vtable_with(&mut e, &[(0x10, FREE_SLOT)]);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, free_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(RESERVE_MORE, |e, a| {
            let (array, element_size) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = (size * 2).max(4);
            let fresh = e.mem.alloc(capacity * element_size);
            let old = e.mem.u32(array);
            for word in 0..size * element_size / 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(fresh + 4 * word, value);
            }
            e.mem.set_u32(array, fresh);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, owner);
        for (kind, offset) in [(0, 0x5c), (1, 0x60), (2, 0x68), (3, 0x64)] {
            let table = vtable_with(
                &mut e,
                &[(4, LISTENER_ADDED + kind), (8, LISTENER_REMOVED + kind)],
            );
            e.mem.set_u32(this + offset, table);
        }
        let world = e.mem.alloc(0x10);
        let tables = e.mem.alloc(0x40);
        e.register_double(WORLD_LISTENER_TABLES, move |_, _| Ret {
            eax: tables,
            ..Ret::default()
        });
        Fx {
            e,
            this,
            owner,
            world,
            tables,
            allocator,
        }
    }

    impl Fx {
        /// Runs the function and returns its nested calls.
        fn run(&mut self, address: u32, words: &[u32]) -> Log {
            self.e.call_log = Some(vec![]);
            self.e.call(address, words);
            self.e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .skip(1)
                .collect()
        }

        /// A word array (data, size, capacity word) at `base` holding `items`, with room for
        /// four more.
        fn set_array(&mut self, base: u32, items: &[u32]) {
            let data = self.e.mem.alloc(4 * items.len() as u32 + 16);
            for (index, item) in items.iter().enumerate() {
                self.e.mem.set_u32(data + 4 * index as u32, *item);
            }
            self.e.mem.set_u32(base, data);
            self.e.mem.set_u32(base + 4, items.len() as u32);
            self.e.mem.set_u32(base + 8, items.len() as u32 + 4);
        }

        fn array(&self, base: u32) -> Vec<u32> {
            let data = self.e.mem.u32(base);
            (0..self.e.mem.u32(base + 4))
                .map(|index| self.e.mem.u32(data + 4 * index))
                .collect()
        }

        /// Two context listeners with the stand-ins in slots 4 and 8; returns them.
        fn add_context_listeners(&mut self) -> [u32; 2] {
            let table = vtable_with(
                &mut self.e,
                &[(4, CONTEXT_LISTENER_ADDED), (8, CONTEXT_LISTENER_REMOVED)],
            );
            let listeners = [self.e.mem.alloc(0x10), self.e.mem.alloc(0x10)];
            for listener in listeners {
                self.e.mem.set_u32(listener, table);
            }
            self.set_array(self.this + 0x84, &listeners);
            listeners
        }

        /// Fills the four arrays of the world's listener tables: entities, phantoms, actions,
        /// constraints.
        fn fill_tables(&mut self, items: [&[u32]; 4]) {
            for ((data_at, _, _), entries) in LISTENER_KINDS.into_iter().zip(items) {
                self.set_array(self.tables + data_at, entries);
            }
        }

        /// Three world records of 0xf0 bytes at `+0x78` (vtable, then a marker per dword).
        fn add_records(&mut self) -> u32 {
            let table = vtable_with(&mut self.e, &[(0, RECORD_DESTRUCTOR)]);
            let records = self.e.mem.alloc(4 * 0xf0);
            for record in 0..3u32 {
                let at = records + record * 0xf0;
                for word in 1..0x3c {
                    self.e
                        .mem
                        .set_u32(at + 4 * word, 0x1000 * (record + 1) + word);
                }
                self.e.mem.set_u32(at, table);
            }
            self.e.mem.set_u32(self.this + 0x78, records);
            self.e.mem.set_u32(self.this + 0x7c, 3);
            self.e.mem.set_u32(self.this + 0x80, 4);
            records
        }
    }

    #[test]
    fn the_deleting_destructors_store_their_vtables_and_delete_on_bit_zero() {
        for (address, vtable) in [
            (0x00cd_1e70, 0x010c_b1c4),
            (0x00cd_1e90, 0x010c_b1d4),
            (0x00cd_1eb0, 0x010c_b1e4),
        ] {
            let mut f = fx();
            let object = f.e.mem.alloc(0x10);
            for flags in [0u32, 1, 2, 3] {
                f.e.mem.set_u32(object, 0);
                let log = f.run(address, &args![object, flags]);
                assert_eq!(f.e.mem.u32(object), vtable);
                let expected: Log = if flags & 1 != 0 {
                    vec![(OPERATOR_DELETE, vec![object])]
                } else {
                    vec![]
                };
                assert_eq!(log, expected);
            }
            assert_eq!(f.e.call(address, &args![object, 0u32]).u32(), object);
        }
    }

    #[test]
    fn the_process_context_destructor_with_delete_runs_the_body_then_deletes() {
        let mut f = fx();
        f.e.register_double(DESTROY_PROCESS_CONTEXT, |_, _| Ret::default());
        let object = f.e.mem.alloc(0x10);
        for (flags, deleted) in [(0u32, false), (1, true), (2, false)] {
            let log = f.run(0x00cd_2870, &args![object, flags]);
            let mut expected: Log = vec![(DESTROY_PROCESS_CONTEXT, vec![object])];
            if deleted {
                expected.push((OPERATOR_DELETE, vec![object]));
            }
            assert_eq!(log, expected);
        }
        assert_eq!(f.e.call(0x00cd_2870, &args![object, 0u32]).u32(), object);
    }

    #[test]
    fn the_viewers_are_registered_in_order() {
        let mut f = fx();
        let log = f.run(0x00cd_1ed0, &args![]);
        let expected: Log = VIEWER_REGISTRATIONS
            .map(|address| (address, vec![]))
            .to_vec();
        assert_eq!(log, expected);
    }

    #[test]
    fn the_entity_and_phantom_thunks_pass_the_object_on_through_the_owner() {
        let mut f = fx();
        let (this, owner, object) = (f.this, f.owner, 0x4000u32);
        let cases = [
            (0x00cd_1f30, this + 0x5c, ENTITIES),
            (0x00cd_1f70, this + 0x60, PHANTOMS),
        ];
        for (address, sub, (class, name)) in cases {
            let log = f.run(address, &args![sub, object]);
            let expected = vec![(
                ANNOUNCE_OBJECT,
                vec![owner, object, class, name, object + 0x10],
            )];
            assert_eq!(log, expected);
        }
        for (address, sub) in [(0x00cd_1f60, this + 0x5c), (0x00cd_1fa0, this + 0x60)] {
            let log = f.run(address, &args![sub, object]);
            assert_eq!(log, vec![(WITHDRAW_OBJECT, vec![owner, object])]);
        }
        f.e.mem.set_u32(this + 0xc, 0);
        for (address, sub) in [
            (0x00cd_1f30, this + 0x5c),
            (0x00cd_1f60, this + 0x5c),
            (0x00cd_1f70, this + 0x60),
            (0x00cd_1fa0, this + 0x60),
            (0x00cd_2020, this + 0x68),
            (0x00cd_2040, this + 0x68),
        ] {
            assert!(f.run(address, &args![sub, object]).is_empty());
        }
    }

    #[test]
    fn the_action_thunks_pass_the_object_on_through_the_owner() {
        let mut f = fx();
        let (this, owner, object) = (f.this, f.owner, 0x5000u32);
        let log = f.run(0x00cd_2020, &args![this + 0x68, object]);
        let expected = vec![(
            ANNOUNCE_OBJECT,
            vec![owner, object, ACTIONS.0, ACTIONS.1, 0],
        )];
        assert_eq!(log, expected);
        let log = f.run(0x00cd_2040, &args![this + 0x68, object]);
        assert_eq!(log, vec![(WITHDRAW_OBJECT, vec![owner, object])]);
    }

    #[test]
    fn the_constraint_thunks_skip_constraints_whose_data_answers_eleven() {
        let mut f = fx();
        f.e.register_double(KIND_PLAIN, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        f.e.register_double(KIND_SPECIAL, |_, _| Ret {
            eax: 0xb,
            ..Ret::default()
        });
        let plain_table = vtable_with(&mut f.e, &[(0x20, KIND_PLAIN)]);
        let special_table = vtable_with(&mut f.e, &[(0x20, KIND_SPECIAL)]);
        let (plain_data, special_data) = (f.e.mem.alloc(0x10), f.e.mem.alloc(0x10));
        f.e.mem.set_u32(plain_data, plain_table);
        f.e.mem.set_u32(special_data, special_table);
        let (plain, special, bare) = (
            f.e.mem.alloc(0x20),
            f.e.mem.alloc(0x20),
            f.e.mem.alloc(0x20),
        );
        f.e.mem.set_u32(plain + 0xc, plain_data);
        f.e.mem.set_u32(special + 0xc, special_data);
        let (sub, owner) = (f.this + 0x64, f.owner);
        let log = f.run(0x00cd_1fb0, &args![sub, plain]);
        assert_eq!(
            log,
            vec![
                (KIND_PLAIN, vec![plain_data]),
                (
                    ANNOUNCE_OBJECT,
                    vec![owner, plain, CONSTRAINTS.0, CONSTRAINTS.1, 0]
                ),
            ]
        );
        let log = f.run(0x00cd_1ff0, &args![sub, plain]);
        assert_eq!(
            log,
            vec![
                (KIND_PLAIN, vec![plain_data]),
                (WITHDRAW_OBJECT, vec![owner, plain])
            ]
        );
        for address in [0x00cd_1fb0, 0x00cd_1ff0] {
            let log = f.run(address, &args![sub, special]);
            assert_eq!(log, vec![(KIND_SPECIAL, vec![special_data])]);
            assert!(f.run(address, &args![sub, bare]).is_empty());
        }
        f.e.mem.set_u32(f.this + 0xc, 0);
        for address in [0x00cd_1fb0, 0x00cd_1ff0] {
            assert!(f.run(address, &args![sub, plain]).is_empty());
        }
    }

    #[test]
    fn a_listener_is_removed_by_moving_the_last_one_into_its_place() {
        let mut f = fx();
        let this = f.this;
        for (target, expected) in [
            (10u32, vec![30, 20]),
            (20, vec![10, 30]),
            (30, vec![10, 20]),
            (99, vec![10, 20, 30]),
        ] {
            f.set_array(this + 0x84, &[10, 20, 30]);
            f.e.call(0x00cd_2210, &args![this, target]);
            assert_eq!(f.array(this + 0x84), expected, "{target}");
        }
        f.set_array(this + 0x84, &[]);
        f.e.call(0x00cd_2210, &args![this, 10u32]);
        assert_eq!(f.array(this + 0x84), Vec::<u32>::new());
    }

    #[test]
    fn a_listener_is_added_once_growing_the_array_when_it_is_full() {
        let mut f = fx();
        let (this, allocator) = (f.this, f.allocator);
        f.set_array(this + 0x84, &[10]);
        f.e.mem.set_u32(this + 0x8c, 1);
        let log = f.run(0x00cd_2470, &args![this, 20u32]);
        assert_eq!(
            log,
            vec![
                (MEMORY_ROUTER, vec![]),
                (RESERVE_MORE, vec![allocator, this + 0x84, 4])
            ]
        );
        assert_eq!(f.array(this + 0x84), vec![10, 20]);
        let log = f.run(0x00cd_2470, &args![this, 30u32]);
        assert_eq!(log, vec![(MEMORY_ROUTER, vec![])]);
        assert_eq!(f.array(this + 0x84), vec![10, 20, 30]);
        assert!(f.run(0x00cd_2470, &args![this, 10u32]).is_empty());
        assert_eq!(f.array(this + 0x84), vec![10, 20, 30]);
    }

    #[test]
    fn removing_a_world_tells_the_listeners_and_unhooks_it() {
        let mut f = fx();
        f.e.register_double(UNHOOK_WORLD, |_, _| Ret::default());
        let (this, world) = (f.this, f.world);
        let listeners = f.add_context_listeners();
        f.set_array(this + 0x6c, &[0x111, world, 0x333]);
        let log = f.run(REMOVE_WORLD, &args![this, world]);
        assert_eq!(
            log,
            vec![
                (REMOVE_DELETION_LISTENER, vec![world, this + 0x58]),
                (CONTEXT_LISTENER_REMOVED, vec![listeners[0], world]),
                (CONTEXT_LISTENER_REMOVED, vec![listeners[1], world]),
                (UNHOOK_WORLD, vec![this, world]),
            ]
        );
        assert_eq!(f.array(this + 0x6c), vec![0x111, 0x333]);
        assert!(f.run(REMOVE_WORLD, &args![this, world]).is_empty());
        f.set_array(this + 0x6c, &[world]);
        f.run(REMOVE_WORLD, &args![this, world]);
        assert_eq!(f.array(this + 0x6c), Vec::<u32>::new());
    }

    #[test]
    fn the_deletion_listener_thunk_passes_the_context_on() {
        let mut f = fx();
        f.e.register_double(REMOVE_WORLD, |_, _| Ret::default());
        let (this, world) = (f.this, f.world);
        let log = f.run(0x00cd_2460, &args![this + 0x58, world]);
        assert_eq!(log, vec![(REMOVE_WORLD, vec![this, world])]);
    }

    #[test]
    fn adding_a_world_hooks_the_deletion_listener_and_tells_the_listeners() {
        let mut f = fx();
        f.e.register_double(HOOK_WORLD, |_, _| Ret::default());
        let (this, world, allocator) = (f.this, f.world, f.allocator);
        let listeners = f.add_context_listeners();
        f.set_array(this + 0x6c, &[0x111]);
        let log = f.run(0x00cd_2530, &args![this, world]);
        assert_eq!(
            log,
            vec![
                (ADD_DELETION_LISTENER, vec![world, this + 0x58]),
                (MEMORY_ROUTER, vec![]),
                (CONTEXT_LISTENER_ADDED, vec![listeners[0], world]),
                (CONTEXT_LISTENER_ADDED, vec![listeners[1], world]),
                (HOOK_WORLD, vec![this, world]),
            ]
        );
        assert_eq!(f.array(this + 0x6c), vec![0x111, world]);
        assert!(f.run(0x00cd_2530, &args![this, world]).is_empty());
        // A full array of worlds is grown first.
        f.set_array(this + 0x6c, &[0x111]);
        f.e.mem.set_u32(this + 0x74, 1);
        let log = f.run(0x00cd_2530, &args![this, world]);
        assert_eq!(log[2], (RESERVE_MORE, vec![allocator, this + 0x6c, 4]));
        assert_eq!(f.array(this + 0x6c), vec![0x111, world]);
    }

    #[test]
    fn switching_the_process_context_unhooks_and_hooks_every_world() {
        for (before, after, unhooked, hooked) in [
            (1u32, 1u32, true, true),
            (1, 0, true, false),
            (0, 1, false, true),
            (0, 0, false, false),
        ] {
            let mut f = fx();
            f.e.register_double(UNHOOK_WORLD, |_, _| Ret::default());
            f.e.register_double(HOOK_WORLD, |_, _| Ret::default());
            let this = f.this;
            f.set_array(this + 0x6c, &[0x111, 0x222]);
            f.e.mem.set_u32(this + 0xc, before);
            let log = f.run(0x00cd_24d0, &args![this + 8, after]);
            let mut expected: Log = vec![];
            for (flag, address) in [(unhooked, UNHOOK_WORLD), (hooked, HOOK_WORLD)] {
                if flag {
                    expected.push((address, vec![this, 0x111]));
                    expected.push((address, vec![this, 0x222]));
                }
            }
            assert_eq!(log, expected, "{before} {after}");
            assert_eq!(f.e.mem.u32(this + 0xc), after);
        }
    }

    #[test]
    fn unhooking_a_world_removes_its_record_and_tells_the_listeners() {
        let mut f = fx();
        let (this, world, owner, tables) = (f.this, f.world, f.owner, f.tables);
        f.set_array(this + 0x6c, &[0x111, world, 0x333]);
        let records = f.add_records();
        f.fill_tables([&[0xe1, 0xe2], &[0xf1], &[], &[0xc1, 0xc2, 0xc3]]);
        let log = f.run(UNHOOK_WORLD, &args![this, world]);
        assert_eq!(
            log,
            vec![
                (REMOVE_ENTITY_LISTENER, vec![world, this + 0x5c]),
                (REMOVE_PHANTOM_LISTENER, vec![world, this + 0x60]),
                (REMOVE_ACTION_LISTENER, vec![world, this + 0x68]),
                (REMOVE_CONSTRAINT_LISTENER, vec![world, this + 0x64]),
                (WITHDRAW_OBJECT, vec![owner, records + 0xf0]),
                (RECORD_DESTRUCTOR, vec![records + 0xf0, 0]),
                (WORLD_LISTENER_TABLES, vec![world]),
                (LISTENER_REMOVED, vec![this + 0x5c, 0xe1]),
                (LISTENER_REMOVED, vec![this + 0x5c, 0xe2]),
                (LISTENER_REMOVED + 1, vec![this + 0x60, 0xf1]),
                (LISTENER_REMOVED + 3, vec![this + 0x64, 0xc1]),
                (LISTENER_REMOVED + 3, vec![this + 0x64, 0xc2]),
                (LISTENER_REMOVED + 3, vec![this + 0x64, 0xc3]),
                (RELEASE, vec![tables]),
            ]
        );
        assert_eq!(f.e.mem.u32(this + 0x7c), 2);
        // The last record moved into the place of the removed one.
        for word in 1..0x3c {
            assert_eq!(f.e.mem.u32(records + 0xf0 + 4 * word), 0x3000 + word);
            assert_eq!(f.e.mem.u32(records + 4 * word), 0x1000 + word);
        }
    }

    #[test]
    fn unhooking_the_last_record_or_an_unknown_world_copies_nothing() {
        let mut f = fx();
        let (this, world) = (f.this, f.world);
        f.set_array(this + 0x6c, &[0x111, 0x222, world]);
        let records = f.add_records();
        let log = f.run(UNHOOK_WORLD, &args![this, world]);
        assert!(log.contains(&(RECORD_DESTRUCTOR, vec![records + 2 * 0xf0, 0])));
        assert_eq!(f.e.mem.u32(this + 0x7c), 2);
        assert_eq!(f.e.mem.u32(records + 2 * 0xf0 + 8), 0x3000 + 2);
        // A world that is not in the array: the listeners are still unhooked, no record goes.
        f.set_array(this + 0x6c, &[0x111]);
        let log = f.run(UNHOOK_WORLD, &args![this, world]);
        assert_eq!(log.len(), 6);
        assert_eq!(f.e.mem.u32(this + 0x7c), 2);
        // Nothing happens for a null world or while the owner is null.
        assert!(f.run(UNHOOK_WORLD, &args![this, 0u32]).is_empty());
        f.e.mem.set_u32(this + 0xc, 0);
        assert!(f.run(UNHOOK_WORLD, &args![this, world]).is_empty());
    }

    #[test]
    fn unhooking_keeps_the_record_when_the_owner_vanishes_meanwhile() {
        let mut f = fx();
        let (this, world) = (f.this, f.world);
        f.set_array(this + 0x6c, &[world]);
        f.add_records();
        f.e.register_double(REMOVE_CONSTRAINT_LISTENER, move |e, _| {
            e.mem.set_u32(this + 0xc, 0);
            Ret::default()
        });
        let log = f.run(UNHOOK_WORLD, &args![this, world]);
        assert!(!log.iter().any(|call| call.0 == RECORD_DESTRUCTOR));
        assert_eq!(f.e.mem.u32(this + 0x7c), 3);
        assert_eq!(log.last().map(|call| call.0), Some(RELEASE));
    }

    #[test]
    fn hooking_a_world_appends_and_announces_a_record() {
        let mut f = fx();
        let (this, world, owner, tables) = (f.this, f.world, f.owner, f.tables);
        let records = f.add_records();
        f.fill_tables([&[0xe1], &[], &[0xa1, 0xa2], &[]]);
        let log = f.run(HOOK_WORLD, &args![this, world]);
        let record = records + 3 * 0xf0;
        assert_eq!(
            log,
            vec![
                (LOCK_WORLD, vec![world]),
                (ADD_ENTITY_LISTENER, vec![world, this + 0x5c]),
                (ADD_PHANTOM_LISTENER, vec![world, this + 0x60]),
                (ADD_ACTION_LISTENER, vec![world, this + 0x68]),
                (ADD_CONSTRAINT_LISTENER, vec![world, this + 0x64]),
                (MEMORY_ROUTER, vec![]),
                (WORLD_RECORD_CONSTRUCTOR, vec![record]),
                (GET_WORLD_RECORD, vec![world, record]),
                (
                    ANNOUNCE_OBJECT,
                    vec![owner, record, WORLD_RECORDS.0, WORLD_RECORDS.1, 0]
                ),
                (WORLD_LISTENER_TABLES, vec![world]),
                (LISTENER_ADDED, vec![this + 0x5c, 0xe1]),
                (LISTENER_ADDED + 2, vec![this + 0x68, 0xa1]),
                (LISTENER_ADDED + 2, vec![this + 0x68, 0xa2]),
                (RELEASE, vec![tables]),
                (UNLOCK_WORLD, vec![world]),
            ]
        );
        assert_eq!(f.e.mem.u32(this + 0x7c), 4);
    }

    #[test]
    fn hooking_a_world_grows_a_full_record_array_and_may_skip_the_announcement() {
        let mut f = fx();
        let (this, world, allocator) = (f.this, f.world, f.allocator);
        f.add_records();
        f.e.mem.set_u32(this + 0x80, 3);
        f.e.register_double(GET_WORLD_RECORD, move |e, _| {
            e.mem.set_u32(this + 0xc, 0);
            Ret::default()
        });
        let log = f.run(HOOK_WORLD, &args![this, world]);
        assert_eq!(log[6], (RESERVE_MORE, vec![allocator, this + 0x78, 0xf0]));
        assert!(!log.iter().any(|call| call.0 == ANNOUNCE_OBJECT));
        let grown = f.e.mem.u32(this + 0x78);
        assert_eq!(log[7], (WORLD_RECORD_CONSTRUCTOR, vec![grown + 3 * 0xf0]));
        assert_eq!(f.e.mem.u32(this + 0x7c), 4);
        // The grown array kept the old records.
        assert_eq!(f.e.mem.u32(grown + 0xf0 + 4), 0x2001);
    }

    #[test]
    fn a_record_at_address_zero_is_not_constructed() {
        let mut f = fx();
        let (this, world) = (f.this, f.world);
        f.e.register_double(RESERVE_MORE, |_, _| Ret::default());
        let log = f.run(HOOK_WORLD, &args![this, world]);
        assert!(!log.iter().any(|call| call.0 == WORLD_RECORD_CONSTRUCTOR));
        assert!(log.contains(&(GET_WORLD_RECORD, vec![world, 0])));
        assert_eq!(f.e.mem.u32(this + 0x7c), 1);
    }

    #[test]
    fn hooking_needs_an_owner_and_a_world() {
        let mut f = fx();
        let (this, world) = (f.this, f.world);
        assert!(f.run(HOOK_WORLD, &args![this, 0u32]).is_empty());
        f.e.mem.set_u32(this + 0xc, 0);
        assert!(f.run(HOOK_WORLD, &args![this, world]).is_empty());
    }

    #[test]
    fn the_constructor_builds_the_vtables_and_the_empty_arrays() {
        let mut f = fx();
        let object = f.e.mem.alloc(0x100);
        for word in 0..0x40 {
            f.e.mem.set_u32(object + 4 * word, 0xdead_beef);
        }
        assert_eq!(f.e.call(0x00cd_25d0, &args![object]).u32(), object);
        let word = |f: &Fx, at: u32| f.e.mem.u32(object + at);
        assert_eq!(f.e.mem.u16(object + 6), 1);
        assert_eq!(word(&f, 0), CONTEXT_VTABLE);
        assert_eq!(word(&f, 8), PROCESS_CONTEXT_VTABLE);
        assert_eq!(word(&f, 0xc), 0);
        assert_eq!(word(&f, 0x10), object + 0x1c);
        assert_eq!(word(&f, 0x14), 0);
        assert_eq!(word(&f, 0x18), 0x8000_0006);
        assert_eq!(word(&f, 0x34), object + 0x40);
        assert_eq!(word(&f, 0x38), 0);
        assert_eq!(word(&f, 0x3c), 0x8000_0006);
        assert_eq!(word(&f, 0x58), 0x010c_b294);
        assert_eq!(word(&f, 0x5c), 0x010c_b278);
        assert_eq!(word(&f, 0x60), 0x010c_b260);
        assert_eq!(word(&f, 0x64), 0x010c_b240);
        assert_eq!(word(&f, 0x68), 0x010c_b230);
        for array in [0x6c, 0x78, 0x84] {
            assert_eq!(word(&f, array), 0);
            assert_eq!(word(&f, array + 4), 0);
            assert_eq!(word(&f, array + 8), 0x8000_0000);
        }
        // Words the constructor leaves alone.
        assert_eq!(f.e.mem.u16(object + 4), 0xbeef);
        assert_eq!(word(&f, 0x1c), 0xdead_beef);
    }

    #[test]
    fn the_destructor_removes_the_worlds_and_frees_owned_arrays() {
        let mut f = fx();
        f.e.register_double(REMOVE_WORLD, |_, _| Ret::default());
        f.e.register_double(DESTROY_PROCESS_CONTEXT, |_, _| Ret::default());
        let (this, allocator) = (f.this, f.allocator);
        f.add_context_listeners();
        f.set_array(this + 0x6c, &[0x111, 0x222, 0x333]);
        let records = f.add_records();
        let listeners_data = f.e.mem.u32(this + 0x84);
        let worlds_data = f.e.mem.u32(this + 0x6c);
        let log = f.run(0x00cd_2690, &args![this]);
        assert_eq!(
            log,
            vec![
                (REMOVE_WORLD, vec![this, 0x333]),
                (REMOVE_WORLD, vec![this, 0x222]),
                (REMOVE_WORLD, vec![this, 0x111]),
                (MEMORY_ROUTER, vec![]),
                (FREE_SLOT, vec![allocator, listeners_data, 6 * 4]),
                (MEMORY_ROUTER, vec![]),
                (RECORD_DESTRUCTOR, vec![records, 0]),
                (RECORD_DESTRUCTOR, vec![records + 0xf0, 0]),
                (RECORD_DESTRUCTOR, vec![records + 2 * 0xf0, 0]),
                (FREE_SLOT, vec![allocator, records, 4 * 0xf0]),
                (MEMORY_ROUTER, vec![]),
                (FREE_SLOT, vec![allocator, worlds_data, 7 * 4]),
                (DESTROY_PROCESS_CONTEXT, vec![this + 8]),
            ]
        );
        for (data_at, size_at, capacity_at) in
            [(0x6c, 0x70, 0x74), (0x78, 0x7c, 0x80), (0x84, 0x88, 0x8c)]
        {
            assert_eq!(f.e.mem.u32(this + data_at), 0);
            assert_eq!(f.e.mem.u32(this + size_at), 0);
            assert_eq!(f.e.mem.u32(this + capacity_at), 0x8000_0000);
        }
        assert_eq!(f.e.mem.u32(this), DESTROYED_VTABLE);
        assert_eq!(f.e.mem.u32(this + 8), PROCESS_CONTEXT_VTABLE);
        assert_eq!(f.e.mem.u32(this + 0x58), 0x010c_b1c4);
        assert_eq!(f.e.mem.u32(this + 0x5c), 0x0102_f13c);
        assert_eq!(f.e.mem.u32(this + 0x60), 0x0104_e3d0);
        assert_eq!(f.e.mem.u32(this + 0x64), 0x010c_b1e4);
        assert_eq!(f.e.mem.u32(this + 0x68), 0x010c_b1d4);
    }

    #[test]
    fn the_destructor_keeps_storage_it_does_not_own() {
        let mut f = fx();
        f.e.register_double(DESTROY_PROCESS_CONTEXT, |_, _| Ret::default());
        let this = f.this;
        let storage = f.e.mem.alloc(0x40);
        for (data_at, capacity_at) in [(0x6c, 0x74), (0x78, 0x80), (0x84, 0x8c)] {
            f.e.mem.set_u32(this + data_at, storage);
            f.e.mem.set_u32(this + capacity_at, 0x8000_0006);
        }
        let log = f.run(0x00cd_2690, &args![this]);
        let expected: Log = vec![
            (MEMORY_ROUTER, vec![]),
            (MEMORY_ROUTER, vec![]),
            (MEMORY_ROUTER, vec![]),
            (DESTROY_PROCESS_CONTEXT, vec![this + 8]),
        ];
        assert_eq!(log, expected);
        assert_eq!(f.e.mem.u32(this + 0x6c), 0);
    }

    #[test]
    fn the_process_context_body_frees_both_arrays() {
        let mut f = fx();
        let (object, allocator) = (f.e.mem.alloc(0x40), f.allocator);
        let (first, second) = (f.e.mem.alloc(0x40), f.e.mem.alloc(0x40));
        f.e.mem.set_u32(object + 0x2c, first);
        f.e.mem.set_u32(object + 0x30, 3);
        f.e.mem.set_u32(object + 0x34, 6);
        f.e.mem.set_u32(object + 8, second);
        f.e.mem.set_u32(object + 0xc, 2);
        f.e.mem.set_u32(object + 0x10, 0x4000_0005);
        let log = f.run(DESTROY_PROCESS_CONTEXT, &args![object]);
        assert_eq!(
            log,
            vec![
                (MEMORY_ROUTER, vec![]),
                (FREE_SLOT, vec![allocator, first, 6 * 4]),
                (MEMORY_ROUTER, vec![]),
                (FREE_SLOT, vec![allocator, second, 5 * 4]),
            ]
        );
        assert_eq!(f.e.mem.u32(object), 0x010c_b220);
        for (data_at, size_at, capacity_at) in [(0x2c, 0x30, 0x34), (8, 0xc, 0x10)] {
            assert_eq!(f.e.mem.u32(object + data_at), 0);
            assert_eq!(f.e.mem.u32(object + size_at), 0);
            assert_eq!(f.e.mem.u32(object + capacity_at), 0x8000_0000);
        }
        // Inline storage (bit 31 set) is not freed.
        f.e.mem.set_u32(object + 0x34, 0x8000_0006);
        f.e.mem.set_u32(object + 0x10, 0x8000_0006);
        let log = f.run(DESTROY_PROCESS_CONTEXT, &args![object]);
        assert_eq!(log.iter().filter(|call| call.0 == FREE_SLOT).count(), 0);
    }
}
