//! `Havok SDK/hkpphysicsdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dc_ecc0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4c8e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dc_ecc0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpPhysicsSystem` functions (`this` = the system, one argument each, all `RET 4`):
/// `addRigidBody` (`00d064a0`), `addAction` (`00d06590`), `addConstraint` (`00d06540`) and
/// `addPhantom` (`00d064f0`); `removeRigidBody` (`00d06130`), `removeAction` (`00d061f0`) and
/// `removeConstraint` (`00d061b0`) take an index. The constructor is `00d065e0` (`this` only,
/// returns `this`). A system keeps its rigid bodies at `+8` (data) / `+0xc` (size), its
/// constraints at `+0x14` / `+0x18`, its actions at `+0x20` / `+0x24` and its phantoms at
/// `+0x2c` / `+0x30`, an `hkStringPtr` name at `+0x38`, a user data word at `+0x3c` and the
/// active byte at `+0x40`.
const SYSTEM_CONSTRUCT: u32 = 0x00d0_65e0;
const SYSTEM_ADD_RIGID_BODY: u32 = 0x00d0_64a0;
const SYSTEM_ADD_ACTION: u32 = 0x00d0_6590;
const SYSTEM_ADD_CONSTRAINT: u32 = 0x00d0_6540;
const SYSTEM_ADD_PHANTOM: u32 = 0x00d0_64f0;
const SYSTEM_REMOVE_RIGID_BODY: u32 = 0x00d0_6130;
const SYSTEM_REMOVE_ACTION: u32 = 0x00d0_61f0;
const SYSTEM_REMOVE_CONSTRAINT: u32 = 0x00d0_61b0;
/// `hkpEntity::isActive` (`00c9bff0`, `this` = the entity, a scratch byte; returns a pointer to
/// the result byte).
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// `hkStringPtr::operator=(const char*)` (`00cb0c10`, `this` = the string pointer, the text).
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// `hkString::strCasecmp` (`00cb0fe0`, cdecl `(a, b)`).
const STR_CASECMP: u32 = 0x00cb_0fe0;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpWorldCinfo`'s constructor (`00c90b80`, `this`; the object is 0xf0 bytes), the function
/// that applies one to a world (`00c91c50`, `this` = the world, the cinfo) and another world
/// function filling an array (`00c93b30`, `this` = the world, the array at `+0xc`; both
/// unnamed).
const WORLD_CINFO_CONSTRUCT: u32 = 0x00c9_0b80;
const WORLD_APPLY_CINFO: u32 = 0x00c9_1c50;
const WORLD_FILL_SYSTEMS: u32 = 0x00c9_3b30;
/// `hkpWorld`'s constructor (`00c95a80`, `this` = the 0x350-byte block, the cinfo, the
/// broad-phase size `0x111d4`; returns the world), `hkpAgentRegisterUtil::registerAllAgents`
/// (`00cbfd20`, cdecl `(dispatcher)`), `hkpWorld::addPhysicsSystem` (`00c97f80`, `this` = the
/// world, the system), `hkpSaveContactPointsUtil::loadContactPoints` (`00dce7b0`, cdecl
/// `(options, system, world)`), `hkpWorld::updateCollisionFilterOnWorld` (`00c96ff0`, `this` =
/// the world, `(0, 1)`) and the function that fills a new system for `00d4c2d0` (`00dcda40`,
/// cdecl `(options, world, system)`).
const WORLD_CONSTRUCT: u32 = 0x00c9_5a80;
const REGISTER_ALL_AGENTS: u32 = 0x00cb_fd20;
const WORLD_ADD_PHYSICS_SYSTEM: u32 = 0x00c9_7f80;
const LOAD_CONTACT_POINTS: u32 = 0x00dc_e7b0;
const WORLD_UPDATE_COLLISION_FILTER: u32 = 0x00c9_6ff0;
const POPULATE_SYSTEM: u32 = 0x00dc_da40;
/// The vtables `hkpPhysicsData` stores (`010cf9a4`, then `0102e388` at the end of its
/// destructor), the cinfo's (`0102f0ec`, left on the stack copy before its references are
/// given back) and the physics system with contacts' (`010d07f4`).
const PHYSICS_DATA_VTABLE: u32 = 0x010c_f9a4;
const BASE_VTABLE: u32 = 0x0102_e388;
const CINFO_VTABLE: u32 = 0x0102_f0ec;
const SYSTEM_WITH_CONTACTS_VTABLE: u32 = 0x010d_07f4;
/// The names `splitPhysicsSystems` gives the systems it makes.
const NAME_UNCONSTRAINED: u32 = 0x010d_0848;
const NAME_FIXED: u32 = 0x010d_0834;
const NAME_KEYFRAMED: u32 = 0x010d_081c;
const NAME_CONSTRAINED: u32 = 0x010d_0808;
const NAME_PHANTOMS: u32 = 0x010c_41a4;
/// The motion type byte (`+0xe8` of an entity) of keyframed and fixed bodies.
const MOTION_KEYFRAMED: u8 = 4;
const MOTION_FIXED: u8 = 5;

/// The memory router's heap allocator.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A block of `size` bytes from the heap allocator (slot 4) with its 16-bit size at `+4`.
fn allocate_object(e: &mut Engine, size: u32) -> u32 {
    let allocator = heap_allocator(e);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// The result byte of `hkpEntity::isActive`.
fn entity_active_byte(e: &mut Engine, entity: u32) -> u8 {
    e.with_stack(4, |e, scratch| {
        let result = e.call(ENTITY_IS_ACTIVE, &args![entity, scratch]).u32();
        e.mem.u8(result)
    })
}

/// Index of `word` in the word array whose data pointer is at `array` and size at `array + 4`
/// (read once), or -1.
fn index_of_word(e: &Engine, array: u32, word: u32) -> i32 {
    let count = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    (0..count.max(0))
        .find(|index| {
            e.mem
                .u32(data.wrapping_add((*index as u32).wrapping_mul(4)))
                == word
        })
        .unwrap_or(-1)
}

/// Appends `word` to the pointer array at `array`, growing it first (`00c90940`) when it is
/// full.
fn append_word(e: &mut Engine, array: u32, word: u32) {
    let allocator = heap_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), word);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d4bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `tryMoveEntity` (engine map; a helper of `splitPhysicsSystems`, cdecl with the destination
/// system in `ESI`, so the parameters are in that order): `destination`, then `source` and
/// `entity` on the stack. Does nothing when `entity` is already in the destination's rigid
/// bodies (`+8` / `+0xc`) or is not in the source's. Otherwise sets the destination's active
/// byte (`+0x40`) to 1 when it was set or the entity is active (`hkpEntity::isActive`), adds
/// the entity to the destination (`00d064a0`) and removes it from the source by index
/// (`00d06130`).
pub fn try_move_entity(e: &mut Engine, destination: Ptr, source: Ptr, entity: u32) {
    let (destination, source) = (destination.addr(), source.addr());
    let source_index = index_of_word(e, source + 8, entity);
    if index_of_word(e, destination + 8, entity) != -1 || source_index == -1 {
        return;
    }
    let active = e.mem.u8(destination + 0x40) != 0 || entity_active_byte(e, entity) != 0;
    e.mem.set_u8(destination + 0x40, u8::from(active));
    e.call(SYSTEM_ADD_RIGID_BODY, &args![destination, entity]);
    e.call(SYSTEM_REMOVE_RIGID_BODY, &args![source, source_index]);
}

// Translated from 00d4c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (looks like a find-by-name over the physics systems): `ECX` = the physics data and
/// the name on the stack. Walks the array at `+0xc` (size `+0x10`, re-read each time) and
/// returns the first entry whose `hkStringPtr` at `+0x38` (low bit cleared) is not null and
/// equals `name` without regard to case (`hkString::strCasecmp`); 0 when there is none.
pub fn fn_00d4c000(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let me = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(me + 0x10) {
        let entry = e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
        let text = e.mem.u32(entry + 0x38) & 0xffff_fffe;
        if text != 0 && e.call(STR_CASECMP, &args![text, name]).i32() == 0 {
            return e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
        }
        index += 1;
    }
    0
}

// Translated from 00d4c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsData::createWorld` (engine map): `ECX` = the physics data (world cinfo at `+8`,
/// array of physics systems at `+0xc` / `+0x10`) and `register_agents` on the stack. Builds a
/// default cinfo on the stack (`00c90b80`), makes an `hkpWorld` (0x350 bytes, constructor
/// `00c95a80` with the stored cinfo, or the default one when `+8` is null, and `0x111d4`),
/// registers all agents with the world's dispatcher (`+0x80`) when `register_agents` is set,
/// adds each system (`00c97f80`) and, when the system's virtual function at `0x10` (asked for
/// a result byte in a stack scratch) answers non-zero, loads its contact points (`00dce7b0`
/// with the options `00 00 | 0 | 01 01 00`). If a cinfo is stored and its word at `+0x58` is
/// non-null, runs `updateCollisionFilterOnWorld(0, 1)`. Finally gives the default cinfo's
/// references (`+0x6c`, `+0x5c`, `+0x58`) back and returns the world.
pub fn hkp_physics_data_create_world(e: &mut Engine, this: Ptr, register_agents: u8) -> Ptr {
    let me = this.addr();
    let world = e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let default_info = frame + 0x20;
        e.call(WORLD_CINFO_CONSTRUCT, &args![default_info]);
        let has_info = e.mem.u32(me + 8) != 0;
        let block = allocate_object(e, 0x350);
        let info = if has_info {
            e.mem.u32(me + 8)
        } else {
            default_info
        };
        let world = e
            .call(WORLD_CONSTRUCT, &args![block, info, 0x111d4u32])
            .u32();
        if register_agents != 0 {
            let dispatcher = e.mem.u32(world + 0x80);
            e.call(REGISTER_ALL_AGENTS, &args![dispatcher]);
        }
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(me + 0x10) {
            let system = e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
            e.call(WORLD_ADD_PHYSICS_SYSTEM, &args![world, system]);
            let system = e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
            let saves = e.vcall(system, 0x10, &args![frame + 0x13]).u32();
            if e.mem.u8(saves) != 0 {
                let options = frame + 0x14;
                e.mem.set_u8(options, 0);
                e.mem.set_u8(options + 1, 0);
                e.mem.set_u32(options + 4, 0);
                e.mem.set_u8(options + 8, 1);
                e.mem.set_u8(options + 9, 1);
                e.mem.set_u8(options + 0xa, 0);
                let system = e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
                e.call(LOAD_CONTACT_POINTS, &args![options, system, world]);
            }
            index += 1;
        }
        let stored = e.mem.u32(me + 8);
        if stored != 0 && e.mem.u32(stored + 0x58) != 0 {
            e.call(WORLD_UPDATE_COLLISION_FILTER, &args![world, 0u32, 1u32]);
        }
        e.mem.set_u32(default_info, CINFO_VTABLE);
        let held = e.mem.u32(default_info + 0x6c);
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        let held = e.mem.u32(default_info + 0x5c);
        e.mem.set_u32(default_info + 0x6c, 0);
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        let held = e.mem.u32(default_info + 0x58);
        e.mem.set_u32(default_info + 0x5c, 0);
        if held != 0 {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
        world
    });
    Ptr::new(world)
}

// Translated from 00d4c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no map name; `ECX` = the physics data, no stack words): the 16-bit
/// reference count at `+6` is 1, the vtable `010cf9a4` is set, the cinfo at `+8` is null and the
/// array of systems (`+0xc` / `+0x10` / `+0x14`) is empty and not owned (`0x80000000`).
pub fn fn_00d4c220(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, PHYSICS_DATA_VTABLE);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, 0x8000_0000);
    e.mem.set_u32(me + 8, 0);
}

// Translated from 00d4c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsData::~hkpPhysicsData` (engine map, `__fastcall`): sets the vtable `010cf9a4`,
/// releases the cinfo at `+8` if there is one, releases every system of the array at `+0xc`
/// (size `+0x10`, re-read each time), gives the array's storage back to the heap allocator
/// (slot `0x10`, `(data, capacity * 4)`) unless its capacity word has bit 31 set, resets it
/// (data null, capacity word `0x80000000`) and leaves the base vtable `0102e388`.
pub fn hkp_physics_data_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let info = e.mem.u32(me + 8);
    e.mem.set_u32(me, PHYSICS_DATA_VTABLE);
    if info != 0 {
        e.call(REMOVE_REFERENCE, &args![info]);
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(me + 0x10) {
        let system = e.mem.u32(e.mem.u32(me + 0xc) + index * 4);
        e.call(REMOVE_REFERENCE, &args![system]);
        index += 1;
    }
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(me + 0x14);
    e.mem.set_u32(me + 0x10, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xc);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x14, 0x8000_0000);
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00d4c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (looks like a populate-from-world function): `ECX` = the physics data, then
/// `world` and `save_contacts` (a byte) on the stack. Makes the cinfo at `+8` when there is
/// none (0xf0 bytes from the heap allocator, constructed with `00c90b80`), has the world fill
/// it (`00c91c50(world, cinfo)`) and the array at `+0xc` (`00c93b30(world, array)`), and, with
/// `save_contacts`, makes a 0x50-byte system (`00d065e0`, vtable `010d07f4`, its array at
/// `+0x44` empty and not owned, active byte 0) that `00dcda40` fills from the world (cdecl
/// `(options, world, system)`, the options a zeroed 8-byte stack block) and appends it to the
/// array (growing it first with `00c90940` when it is full).
pub fn fn_00d4c2d0(e: &mut Engine, this: Ptr, world: u32, save_contacts: u8) {
    let me = this.addr();
    if e.mem.u32(me + 8) == 0 {
        let block = allocate_object(e, 0xf0);
        let info = e.call(WORLD_CINFO_CONSTRUCT, &args![block]).u32();
        e.mem.set_u32(me + 8, info);
    }
    let info = e.mem.u32(me + 8);
    e.call(WORLD_APPLY_CINFO, &args![world, info]);
    let systems = me + 0xc;
    e.call(WORLD_FILL_SYSTEMS, &args![world, systems]);
    if save_contacts != 0 {
        e.with_stack(8, |e, options| {
            let system = allocate_object(e, 0x50);
            e.call(SYSTEM_CONSTRUCT, &args![system]);
            e.mem.set_u32(system, SYSTEM_WITH_CONTACTS_VTABLE);
            e.mem.set_u32(system + 0x44, 0);
            e.mem.set_u32(system + 0x48, 0);
            e.mem.set_u32(system + 0x4c, 0x8000_0000);
            e.call(POPULATE_SYSTEM, &args![options, world, system]);
            e.mem.set_u8(system + 0x40, 0);
            append_word(e, systems, system);
        });
    }
}

/// A new `hkpPhysicsSystem` (0x44 bytes from the heap allocator, constructed with `00d065e0`,
/// which returns the system).
fn new_physics_system(e: &mut Engine) -> u32 {
    let block = allocate_object(e, 0x44);
    e.call(SYSTEM_CONSTRUCT, &args![block]).u32()
}

/// Names `system` (`hkStringPtr::operator=`) and gives it the user data word of `like`.
fn name_physics_system(e: &mut Engine, system: u32, name: u32, like: u32) {
    e.call(STRING_PTR_ASSIGN, &args![system + 0x38, name]);
    let user_data = e.mem.u32(like + 0x3c);
    e.mem.set_u32(system + 0x3c, user_data);
}

// Translated from 00d4c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsData::splitPhysicsSystems` (engine map; cdecl, two stack words: the `source`
/// system and the `output` record). Copies everything in `source` (rigid bodies, actions,
/// constraints and phantoms) into a working system `all`, makes the systems "Unconstrained
/// Rigid Bodies", "Fixed Rigid Bodies" and "Keyframed Rigid Bodies" (user data of `all`,
/// active byte 0), and then, while `all` has rigid bodies, takes the first one into a new
/// "Constrained System" (active byte copied from `hkpEntity::isActive`) and keeps scanning
/// that system's bodies: every constraint of `all` that has the body at either end moves into
/// the system (`00d06540`, then the body at its other end moves too unless it is null or its
/// motion type (byte `+0xe8`) is 5, by `try_move_entity`, then `00d061b0` removes it from
/// `all`), and so does every action whose entity array (virtual function `0x10`, filling an
/// array in a stack block) contains the body (the action's other entities move as for
/// constraints; the scan of the entities continues with the next action's entry at the same
/// index, as the exe does without a break); the scan starts again from the first body while
/// anything moved. A system left with no constraints and no actions is given back: its one
/// body goes into the keyframed (motion 4), fixed (motion 5) or unconstrained system, whose
/// active byte becomes 1 when it was set or the body is active (a body with a null word at
/// `+0xcc` counts as active; fixed bodies leave the byte alone), and the system is destroyed
/// (virtual function 0 with 1); otherwise it is appended to the array at `output + 0x10`.
/// At the end `output` gets the fixed system at `+0`, the keyframed one at `+4`, the
/// unconstrained one at `+8` (each only when it has rigid bodies, else it is destroyed and the
/// word is 0), and at `+0xc` a "Phantoms" system holding `all`'s phantoms (0 when there are
/// none); `all` is destroyed.
pub fn hkp_physics_data_split_physics_systems(e: &mut Engine, source: Ptr, output: Ptr) {
    let (source, output) = (source.addr(), output.addr());
    e.with_stack(0x10, |e, entities| {
        let entities = entities.addr();
        let all = new_physics_system(e);
        for (array, adder) in [
            (8, SYSTEM_ADD_RIGID_BODY),
            (0x20, SYSTEM_ADD_ACTION),
            (0x14, SYSTEM_ADD_CONSTRAINT),
            (0x2c, SYSTEM_ADD_PHANTOM),
        ] {
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(source + array + 4) {
                let item = e.mem.u32(e.mem.u32(source + array) + index * 4);
                e.call(adder, &args![all, item]);
                index += 1;
            }
        }
        let unconstrained = new_physics_system(e);
        name_physics_system(e, unconstrained, NAME_UNCONSTRAINED, all);
        e.mem.set_u8(unconstrained + 0x40, 0);
        let fixed = new_physics_system(e);
        name_physics_system(e, fixed, NAME_FIXED, all);
        e.mem.set_u8(fixed + 0x40, 0);
        let keyframed = new_physics_system(e);
        name_physics_system(e, keyframed, NAME_KEYFRAMED, all);
        e.mem.set_u8(keyframed + 0x40, 0);
        while e.mem.i32(all + 0xc) > 0 {
            let group = new_physics_system(e);
            name_physics_system(e, group, NAME_CONSTRAINED, all);
            let first_body = e.mem.u32(e.mem.u32(all + 8));
            let active = entity_active_byte(e, first_body);
            e.mem.set_u8(group + 0x40, active);
            let first_body = e.mem.u32(e.mem.u32(all + 8));
            e.call(SYSTEM_ADD_RIGID_BODY, &args![group, first_body]);
            e.call(SYSTEM_REMOVE_RIGID_BODY, &args![all, 0u32]);
            loop {
                let mut moved = false;
                let mut body_index = 0i32;
                if e.mem.i32(group + 0xc) > 0 {
                    loop {
                        let body = e.mem.u32(e.mem.u32(group + 8) + body_index as u32 * 4);
                        let mut constraint = 0i32;
                        if e.mem.i32(all + 0x18) > 0 {
                            loop {
                                let current =
                                    e.mem.u32(e.mem.u32(all + 0x14) + constraint as u32 * 4);
                                if e.mem.u32(current + 0x14) == body
                                    || e.mem.u32(current + 0x18) == body
                                {
                                    moved = true;
                                    e.call(SYSTEM_ADD_CONSTRAINT, &args![group, current]);
                                    let current =
                                        e.mem.u32(e.mem.u32(all + 0x14) + constraint as u32 * 4);
                                    let other = e.mem.u32(current + 0x18)
                                        ^ e.mem.u32(current + 0x14)
                                        ^ body;
                                    if other != 0 && e.mem.u8(other + 0xe8) != MOTION_FIXED {
                                        try_move_entity(e, Ptr::new(group), Ptr::new(all), other);
                                    }
                                    e.call(SYSTEM_REMOVE_CONSTRAINT, &args![all, constraint]);
                                } else {
                                    constraint += 1;
                                }
                                if constraint >= e.mem.i32(all + 0x18) {
                                    break;
                                }
                            }
                        }
                        let mut action = 0i32;
                        if e.mem.i32(all + 0x24) > 0 {
                            loop {
                                e.mem.set_u32(entities, 0);
                                e.mem.set_u32(entities + 4, 0);
                                e.mem.set_u32(entities + 8, 0x8000_0000);
                                let current = e.mem.u32(e.mem.u32(all + 0x20) + action as u32 * 4);
                                e.vcall(current, 0x10, &args![entities]);
                                let mut limit = e.mem.i32(entities + 4);
                                let mut found = false;
                                if limit > 0 {
                                    let mut position = 0i32;
                                    loop {
                                        let data = e.mem.u32(entities);
                                        if e.mem.u32(data + position as u32 * 4) == body {
                                            let current = e
                                                .mem
                                                .u32(e.mem.u32(all + 0x20) + action as u32 * 4);
                                            e.call(SYSTEM_ADD_ACTION, &args![group, current]);
                                            e.call(SYSTEM_REMOVE_ACTION, &args![all, action]);
                                            found = true;
                                            limit = e.mem.i32(entities + 4);
                                            let mut other_index = 0i32;
                                            if limit > 0 {
                                                loop {
                                                    let data = e.mem.u32(entities);
                                                    let other =
                                                        e.mem.u32(data + other_index as u32 * 4);
                                                    if other != body
                                                        && e.mem.u8(other + 0xe8) != MOTION_FIXED
                                                    {
                                                        try_move_entity(
                                                            e,
                                                            Ptr::new(group),
                                                            Ptr::new(all),
                                                            other,
                                                        );
                                                        limit = e.mem.i32(entities + 4);
                                                    }
                                                    other_index += 1;
                                                    if other_index >= limit {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        position += 1;
                                        if position >= limit {
                                            break;
                                        }
                                    }
                                }
                                if !found {
                                    action += 1;
                                }
                                let allocator = heap_allocator(e);
                                let capacity = e.mem.u32(entities + 8);
                                e.mem.set_u32(entities + 4, 0);
                                if (capacity as i32) >= 0 {
                                    let data = e.mem.u32(entities);
                                    e.vcall(
                                        allocator,
                                        0x10,
                                        &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
                                    );
                                }
                                let more = action < e.mem.i32(all + 0x24);
                                e.mem.set_u32(entities, 0);
                                e.mem.set_u32(entities + 8, 0x8000_0000);
                                if !more {
                                    break;
                                }
                            }
                        }
                        body_index += 1;
                        if body_index >= e.mem.i32(group + 0xc) {
                            break;
                        }
                    }
                }
                if !moved {
                    break;
                }
            }
            if e.mem.u32(group + 0x18) == 0 && e.mem.u32(group + 0x24) == 0 {
                let body = e.mem.u32(e.mem.u32(group + 8));
                let active = e.mem.u32(body + 0xcc) == 0 || entity_active_byte(e, body) != 0;
                let motion = e.mem.u8(body + 0xe8);
                let target = if motion == MOTION_KEYFRAMED {
                    let was = e.mem.u8(keyframed + 0x40) != 0;
                    e.mem.set_u8(keyframed + 0x40, u8::from(was || active));
                    keyframed
                } else if motion == MOTION_FIXED {
                    fixed
                } else {
                    let was = e.mem.u8(unconstrained + 0x40) != 0;
                    e.mem.set_u8(unconstrained + 0x40, u8::from(was || active));
                    unconstrained
                };
                e.call(SYSTEM_ADD_RIGID_BODY, &args![target, body]);
                e.vcall(group, 0, &args![1u32]);
            } else {
                append_word(e, output + 0x10, group);
            }
        }
        for (slot, system) in [(4, keyframed), (0, fixed), (8, unconstrained)] {
            if e.mem.u32(system + 0xc) != 0 {
                e.mem.set_u32(output + slot, system);
            } else {
                e.vcall(system, 0, &args![1u32]);
                e.mem.set_u32(output + slot, 0);
            }
        }
        if e.mem.i32(all + 0x30) > 0 {
            let phantoms = new_physics_system(e);
            name_physics_system(e, phantoms, NAME_PHANTOMS, all);
            e.mem.set_u32(output + 0xc, phantoms);
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(all + 0x30) {
                let item = e.mem.u32(e.mem.u32(all + 0x2c) + index * 4);
                e.call(SYSTEM_ADD_PHANTOM, &args![phantoms, item]);
                index += 1;
            }
        } else {
            e.mem.set_u32(output + 0xc, 0);
        }
        e.vcall(all, 0, &args![1u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_bf70, try_move_entity(Ptr, Ptr, u32)),
        entry!(0x00d4_c000, fn_00d4c000(Ptr, u32) -> u32),
        entry!(
            0x00d4_c0c0,
            hkp_physics_data_create_world(Ptr, u8) -> Ptr
        ),
        entry!(0x00d4_c220, fn_00d4c220(Ptr)),
        entry!(0x00d4_c250, hkp_physics_data_destruct(Ptr)),
        entry!(0x00d4_c2d0, fn_00d4c2d0(Ptr, u32, u8)),
        entry!(
            0x00d4_c3b0,
            hkp_physics_data_split_physics_systems(Ptr, Ptr)
        ),
        entry!(0x00d4_c8e0, deleting_destructor_00d4c8e0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d4c8e0_frees_only_when_bit_zero_is_set() {
        check(0x00d4_c8e0, 0x00dc_ecc0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_ALLOCATE: u32 = 0x0900_0101;
    const SLOT_FREE: u32 = 0x0900_0102;
    const SYSTEM_DESTRUCT: u32 = 0x0900_0103;
    const ACTION_ENTITIES: u32 = 0x0900_0104;
    const SYSTEM_SAVES: u32 = 0x0900_0105;

    struct Fixture {
        e: Engine,
        log: Log,
        allocator: u32,
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(at, _)| *at).collect()
    }

    /// Makes `address` a double that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &Log, address: u32, result: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    /// An engine with a memory router whose allocator hands out zeroed blocks (slot 4) and
    /// records its frees (slot `0x10`).
    fn fixture() -> Fixture {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, SLOT_ALLOCATE);
        e.mem.set_u32(table + 0x10, SLOT_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_ALLOCATE, move |e, a| {
            sink.borrow_mut().push((SLOT_ALLOCATE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        record(&mut e, &log, SLOT_FREE, 0);
        // `_reserveMore`: a fresh block of 16 words, the old words copied, capacity 16.
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let (old, size) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            let room = e.mem.alloc(64);
            for index in 0..size {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(room + 4 * index, value);
            }
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        Fixture { e, log, allocator }
    }

    /// Appends `word` to the word array at `array` (data, size), in a fresh block.
    fn push_word(e: &mut Engine, array: u32, word: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let fresh = e.mem.alloc(4 * (size + 1));
        for index in 0..size {
            let value = e.mem.u32(old + 4 * index);
            e.mem.set_u32(fresh + 4 * index, value);
        }
        e.mem.set_u32(fresh + 4 * size, word);
        e.mem.set_u32(array, fresh);
        e.mem.set_u32(array + 4, size + 1);
    }

    /// Removes the word at `index` from the array, the later ones moving down.
    fn remove_word(e: &mut Engine, array: u32, index: u32) {
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        for position in index..size - 1 {
            let next = e.mem.u32(data + 4 * (position + 1));
            e.mem.set_u32(data + 4 * position, next);
        }
        e.mem.set_u32(array + 4, size - 1);
    }

    fn words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    /// A physics system object with the given rigid bodies.
    fn system_with(e: &mut Engine, bodies: &[u32]) -> u32 {
        let system = e.mem.alloc(0x100);
        for body in bodies {
            push_word(e, system + 8, *body);
        }
        system
    }

    /// An entity with motion type `motion` and the word `island` at `+0xcc`.
    fn entity(e: &mut Engine, motion: u8, island: u32) -> u32 {
        let entity = e.mem.alloc(0x100);
        e.mem.set_u8(entity + 0xe8, motion);
        e.mem.set_u32(entity + 0xcc, island);
        entity
    }

    /// `hkpEntity::isActive` answering from `active`; logs the entity.
    fn active_entities(f: &mut Fixture, active: Vec<u32>) {
        let sink = Rc::clone(&f.log);
        f.e.register_double(ENTITY_IS_ACTIVE, move |e, a| {
            sink.borrow_mut().push((ENTITY_IS_ACTIVE, vec![a[0]]));
            e.mem.set_u8(a[1], u8::from(active.contains(&a[0])));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    /// Doubles that model the physics system functions on real arrays (rigid bodies at `+8`,
    /// constraints at `+0x14`, actions at `+0x20`, phantoms at `+0x2c`), a constructor that
    /// gives each system a vtable whose destructor is logged and the user data word `0x5eed`, and the
    /// name assignment.
    fn system_model(f: &mut Fixture) {
        let table = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(table, SYSTEM_DESTRUCT);
        record(&mut f.e, &f.log, SYSTEM_DESTRUCT, 0);
        let sink = Rc::clone(&f.log);
        f.e.register_double(SYSTEM_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((SYSTEM_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], table);
            e.mem.set_u32(a[0] + 0x3c, 0x5eed);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for (address, array) in [
            (SYSTEM_ADD_RIGID_BODY, 8),
            (SYSTEM_ADD_ACTION, 0x20),
            (SYSTEM_ADD_CONSTRAINT, 0x14),
            (SYSTEM_ADD_PHANTOM, 0x2c),
        ] {
            let sink = Rc::clone(&f.log);
            f.e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                push_word(e, a[0] + array, a[1]);
                Ret::default()
            });
        }
        for (address, array) in [
            (SYSTEM_REMOVE_RIGID_BODY, 8),
            (SYSTEM_REMOVE_ACTION, 0x20),
            (SYSTEM_REMOVE_CONSTRAINT, 0x14),
        ] {
            let sink = Rc::clone(&f.log);
            f.e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                remove_word(e, a[0] + array, a[1]);
                Ret::default()
            });
        }
        let sink = Rc::clone(&f.log);
        f.e.register_double(STRING_PTR_ASSIGN, move |e, a| {
            sink.borrow_mut()
                .push((STRING_PTR_ASSIGN, vec![a[0] - 0x38, a[1]]));
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn try_move_entity_moves_an_active_entity_and_marks_the_destination() {
        let mut f = fixture();
        system_model(&mut f);
        active_entities(&mut f, vec![0x22]);
        let source = system_with(&mut f.e, &[0x11, 0x22]);
        let destination = system_with(&mut f.e, &[0x33]);
        f.e.call(0x00d4_bf70, &args![destination, source, 0x22u32]);
        assert_eq!(
            calls_to(&f.log, SYSTEM_ADD_RIGID_BODY),
            vec![vec![destination, 0x22]]
        );
        assert_eq!(
            calls_to(&f.log, SYSTEM_REMOVE_RIGID_BODY),
            vec![vec![source, 1]]
        );
        assert_eq!(f.e.mem.u8(destination + 0x40), 1);
        assert_eq!(words(&f.e, source + 8), vec![0x11]);
        assert_eq!(words(&f.e, destination + 8), vec![0x33, 0x22]);
    }

    #[test]
    fn try_move_entity_keeps_an_inactive_destination_inactive() {
        let mut f = fixture();
        system_model(&mut f);
        active_entities(&mut f, vec![]);
        let source = system_with(&mut f.e, &[0x11]);
        let destination = system_with(&mut f.e, &[]);
        f.e.call(0x00d4_bf70, &args![destination, source, 0x11u32]);
        assert_eq!(f.e.mem.u8(destination + 0x40), 0);
        assert_eq!(calls_to(&f.log, ENTITY_IS_ACTIVE), vec![vec![0x11]]);
        assert_eq!(words(&f.e, destination + 8), vec![0x11]);
        // An already active destination does not ask.
        let source = system_with(&mut f.e, &[0x44]);
        f.e.mem.set_u8(destination + 0x40, 1);
        f.e.call(0x00d4_bf70, &args![destination, source, 0x44u32]);
        assert_eq!(calls_to(&f.log, ENTITY_IS_ACTIVE).len(), 1);
        assert_eq!(f.e.mem.u8(destination + 0x40), 1);
    }

    #[test]
    fn try_move_entity_does_nothing_for_known_or_missing_entities() {
        let mut f = fixture();
        system_model(&mut f);
        active_entities(&mut f, vec![0x11]);
        let source = system_with(&mut f.e, &[0x11, 0x22]);
        let destination = system_with(&mut f.e, &[0x22]);
        // Already in the destination.
        f.e.call(0x00d4_bf70, &args![destination, source, 0x22u32]);
        // Not in the source.
        f.e.call(0x00d4_bf70, &args![destination, source, 0x99u32]);
        assert!(f.log.borrow().is_empty());
        assert_eq!(words(&f.e, source + 8), vec![0x11, 0x22]);
    }

    fn named_entry(e: &mut Engine, name: &[u8], flag: u32) -> u32 {
        let text = e.mem.alloc(32);
        e.mem.set_cstr(text, name);
        let entry = e.mem.alloc(0x50);
        e.mem.set_u32(entry + 0x38, text | flag);
        entry
    }

    #[test]
    fn find_by_name_ignores_case_and_unnamed_entries() {
        let mut f = fixture();
        let casecmp_log = Rc::clone(&f.log);
        f.e.register_double(STR_CASECMP, move |e, a| {
            casecmp_log.borrow_mut().push((STR_CASECMP, a.to_vec()));
            let (left, right) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: u32::from(!left.eq_ignore_ascii_case(&right)),
                ..Ret::default()
            }
        });
        let unnamed = unnamed_entry(&mut f.e);
        let first = named_entry(&mut f.e, b"Fixed Rigid Bodies", 1);
        let second = named_entry(&mut f.e, b"Constrained System", 0);
        let data = f.e.mem.alloc(16);
        for (index, entry) in [unnamed, first, second].iter().enumerate() {
            f.e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        let this = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(this + 0xc, data);
        f.e.mem.set_u32(this + 0x10, 3);
        let wanted = f.e.mem.alloc(32);
        f.e.mem.set_cstr(wanted, b"CONSTRAINED system");
        assert_eq!(f.e.call(0x00d4_c000, &args![this, wanted]).u32(), second);
        // The unnamed entry was skipped without asking; the first was compared.
        assert_eq!(calls_to(&f.log, STR_CASECMP).len(), 2);
        f.e.mem.set_cstr(wanted, b"nothing");
        assert_eq!(f.e.call(0x00d4_c000, &args![this, wanted]).u32(), 0);
        f.e.mem.set_u32(this + 0x10, 0);
        assert_eq!(f.e.call(0x00d4_c000, &args![this, wanted]).u32(), 0);
    }

    /// An entry whose name word is 1 (a null text with the owned flag set).
    fn unnamed_entry(e: &mut Engine) -> u32 {
        let entry = e.mem.alloc(0x50);
        e.mem.set_u32(entry + 0x38, 1);
        entry
    }

    #[test]
    fn the_constructor_makes_an_empty_data_object() {
        let mut f = fixture();
        let this = f.e.mem.alloc(0x20);
        for offset in [8, 0xc, 0x10] {
            f.e.mem.set_u32(this + offset, 0x77);
        }
        f.e.call(0x00d4_c220, &args![this]);
        assert_eq!(f.e.mem.u16(this + 6), 1);
        assert_eq!(f.e.mem.u32(this), PHYSICS_DATA_VTABLE);
        assert_eq!(f.e.mem.u32(this + 8), 0);
        assert_eq!(f.e.mem.u32(this + 0xc), 0);
        assert_eq!(f.e.mem.u32(this + 0x10), 0);
        assert_eq!(f.e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn the_destructor_releases_the_cinfo_and_the_systems_and_frees_the_array() {
        let mut f = fixture();
        record(&mut f.e, &f.log, REMOVE_REFERENCE, 0);
        let this = f.e.mem.alloc(0x20);
        let data = f.e.mem.alloc(16);
        f.e.mem.set_u32(data, 0xa1);
        f.e.mem.set_u32(data + 4, 0xa2);
        f.e.mem.set_u32(this + 8, 0xc1);
        f.e.mem.set_u32(this + 0xc, data);
        f.e.mem.set_u32(this + 0x10, 2);
        f.e.mem.set_u32(this + 0x14, 4);
        f.e.call(0x00d4_c250, &args![this]);
        assert_eq!(
            f.log.borrow().clone(),
            vec![
                (REMOVE_REFERENCE, vec![0xc1]),
                (REMOVE_REFERENCE, vec![0xa1]),
                (REMOVE_REFERENCE, vec![0xa2]),
                (SLOT_FREE, vec![f.allocator, data, 16]),
            ]
        );
        assert_eq!(f.e.mem.u32(this), BASE_VTABLE);
        assert_eq!(f.e.mem.u32(this + 0xc), 0);
        assert_eq!(f.e.mem.u32(this + 0x10), 0);
        assert_eq!(f.e.mem.u32(this + 0x14), 0x8000_0000);
        // Without a cinfo and with storage that is not owned, nothing is released or freed.
        f.log.borrow_mut().clear();
        let this = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(this + 0x14, 0x8000_0004);
        f.e.call(0x00d4_c250, &args![this]);
        assert!(f.log.borrow().is_empty());
        assert_eq!(f.e.mem.u32(this), BASE_VTABLE);
    }

    struct WorldRig {
        f: Fixture,
        this: u32,
        world: u32,
        systems: Vec<u32>,
    }

    /// A physics data with two systems (the first saves contact points, the second does not),
    /// and doubles for everything `createWorld` calls.
    fn world_rig(stored_info: u32) -> WorldRig {
        let mut f = fixture();
        let world = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(world + 0x80, 0xd15);
        let block_log = Rc::clone(&f.log);
        f.e.register_double(WORLD_CONSTRUCT, move |_, a| {
            block_log.borrow_mut().push((WORLD_CONSTRUCT, a.to_vec()));
            Ret {
                eax: world,
                ..Ret::default()
            }
        });
        let info_log = Rc::clone(&f.log);
        f.e.register_double(WORLD_CINFO_CONSTRUCT, move |e, a| {
            info_log
                .borrow_mut()
                .push((WORLD_CINFO_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + 0x6c, 0x61);
            e.mem.set_u32(a[0] + 0x5c, 0x62);
            e.mem.set_u32(a[0] + 0x58, 0x63);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [
            REGISTER_ALL_AGENTS,
            WORLD_ADD_PHYSICS_SYSTEM,
            WORLD_UPDATE_COLLISION_FILTER,
            REMOVE_REFERENCE,
        ] {
            record(&mut f.e, &f.log, address, 0);
        }
        let options_log = Rc::clone(&f.log);
        f.e.register_double(LOAD_CONTACT_POINTS, move |e, a| {
            let bytes = e.mem.bytes(a[0], 12);
            let mut seen = vec![a[1], a[2]];
            seen.extend(bytes.iter().map(|b| u32::from(*b)));
            options_log.borrow_mut().push((LOAD_CONTACT_POINTS, seen));
            Ret::default()
        });
        let table = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(table + 0x10, SYSTEM_SAVES);
        let systems: Vec<u32> = (0..2)
            .map(|_| {
                let system = f.e.mem.alloc(0x40);
                f.e.mem.set_u32(system, table);
                system
            })
            .collect();
        let saving = systems[0];
        f.e.register_double(SYSTEM_SAVES, move |e, a| {
            e.mem.set_u8(a[1], u8::from(a[0] == saving));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(this + 8, stored_info);
        let data = f.e.mem.alloc(8);
        f.e.mem.set_u32(data, systems[0]);
        f.e.mem.set_u32(data + 4, systems[1]);
        f.e.mem.set_u32(this + 0xc, data);
        f.e.mem.set_u32(this + 0x10, 2);
        WorldRig {
            f,
            this,
            world,
            systems,
        }
    }

    #[test]
    fn create_world_with_a_stored_cinfo_registers_adds_loads_and_updates_the_filter() {
        let mut rig = world_rig(0);
        let cinfo = rig.f.e.mem.alloc(0x100);
        rig.f.e.mem.set_u32(cinfo + 0x58, 0x7777);
        rig.f.e.mem.set_u32(rig.this + 8, cinfo);
        let (this, world) = (rig.this, rig.world);
        let got = rig.f.e.call(0x00d4_c0c0, &args![this, 1u32]).u32();
        assert_eq!(got, world);
        let log = &rig.f.log;
        assert_eq!(
            order(log),
            vec![
                WORLD_CINFO_CONSTRUCT,
                SLOT_ALLOCATE,
                WORLD_CONSTRUCT,
                REGISTER_ALL_AGENTS,
                WORLD_ADD_PHYSICS_SYSTEM,
                LOAD_CONTACT_POINTS,
                WORLD_ADD_PHYSICS_SYSTEM,
                WORLD_UPDATE_COLLISION_FILTER,
                REMOVE_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_REFERENCE,
            ]
        );
        assert_eq!(
            calls_to(log, SLOT_ALLOCATE),
            vec![vec![rig.f.allocator, 0x350]]
        );
        let construct = &calls_to(log, WORLD_CONSTRUCT)[0];
        assert_eq!(construct[1..], [cinfo, 0x111d4]);
        assert_eq!(calls_to(log, REGISTER_ALL_AGENTS), vec![vec![0xd15]]);
        assert_eq!(
            calls_to(log, WORLD_ADD_PHYSICS_SYSTEM),
            vec![vec![world, rig.systems[0]], vec![world, rig.systems[1]]]
        );
        let load = &calls_to(log, LOAD_CONTACT_POINTS)[0];
        assert_eq!(load[..2], [rig.systems[0], world]);
        assert_eq!(load[2..], [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0]);
        assert_eq!(
            calls_to(log, WORLD_UPDATE_COLLISION_FILTER),
            vec![vec![world, 0, 1]]
        );
        // The default cinfo's references go back in the order +0x6c, +0x5c, +0x58.
        assert_eq!(
            calls_to(log, REMOVE_REFERENCE),
            vec![vec![0x61], vec![0x62], vec![0x63]]
        );
    }

    #[test]
    fn create_world_without_a_cinfo_uses_the_default_one_and_skips_the_optional_steps() {
        let mut rig = world_rig(0);
        let (this, world) = (rig.this, rig.world);
        rig.f.e.mem.set_u32(this + 0x10, 0);
        let got = rig.f.e.call(0x00d4_c0c0, &args![this, 0u32]).u32();
        assert_eq!(got, world);
        let log = &rig.f.log;
        let default_info = calls_to(log, WORLD_CINFO_CONSTRUCT)[0][0];
        assert_eq!(calls_to(log, WORLD_CONSTRUCT)[0][1], default_info);
        assert!(calls_to(log, REGISTER_ALL_AGENTS).is_empty());
        assert!(calls_to(log, WORLD_UPDATE_COLLISION_FILTER).is_empty());
        assert!(calls_to(log, WORLD_ADD_PHYSICS_SYSTEM).is_empty());
        assert_eq!(calls_to(log, REMOVE_REFERENCE).len(), 3);
        // A stored cinfo whose filter word is null does not update the filter.
        let cinfo = rig.f.e.mem.alloc(0x100);
        rig.f.e.mem.set_u32(this + 8, cinfo);
        log.borrow_mut().clear();
        rig.f.e.call(0x00d4_c0c0, &args![this, 0u32]);
        assert!(calls_to(log, WORLD_UPDATE_COLLISION_FILTER).is_empty());
    }

    #[test]
    fn populate_makes_the_cinfo_and_the_contact_system_and_appends_it() {
        let mut f = fixture();
        let info_log = Rc::clone(&f.log);
        f.e.register_double(WORLD_CINFO_CONSTRUCT, move |_, a| {
            info_log
                .borrow_mut()
                .push((WORLD_CINFO_CONSTRUCT, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [WORLD_APPLY_CINFO, WORLD_FILL_SYSTEMS] {
            record(&mut f.e, &f.log, address, 0);
        }
        let construct_log = Rc::clone(&f.log);
        f.e.register_double(SYSTEM_CONSTRUCT, move |_, a| {
            construct_log
                .borrow_mut()
                .push((SYSTEM_CONSTRUCT, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let populate_log = Rc::clone(&f.log);
        f.e.register_double(POPULATE_SYSTEM, move |e, a| {
            populate_log.borrow_mut().push((
                POPULATE_SYSTEM,
                vec![e.mem.u32(a[0]), e.mem.u32(a[0] + 4), a[1], a[2]],
            ));
            Ret::default()
        });
        let grow_log = Rc::clone(&f.log);
        f.e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(16);
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let this = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(this + 0xc, 0);
        f.e.mem.set_u32(this + 0x14, 0x8000_0000);
        f.e.call(0x00d4_c2d0, &args![this, 0x4242u32, 1u32]);
        let log = &f.log;
        let info = f.e.mem.u32(this + 8);
        assert_ne!(info, 0);
        assert_eq!(f.e.mem.u16(info + 4), 0xf0);
        assert_eq!(
            order(log),
            vec![
                SLOT_ALLOCATE,
                WORLD_CINFO_CONSTRUCT,
                WORLD_APPLY_CINFO,
                WORLD_FILL_SYSTEMS,
                SLOT_ALLOCATE,
                SYSTEM_CONSTRUCT,
                POPULATE_SYSTEM,
                ARRAY_RESERVE_MORE,
            ]
        );
        assert_eq!(calls_to(log, WORLD_APPLY_CINFO), vec![vec![0x4242, info]]);
        assert_eq!(
            calls_to(log, WORLD_FILL_SYSTEMS),
            vec![vec![0x4242, this + 0xc]]
        );
        let system = calls_to(log, SYSTEM_CONSTRUCT)[0][0];
        assert_eq!(f.e.mem.u16(system + 4), 0x50);
        assert_eq!(f.e.mem.u32(system), SYSTEM_WITH_CONTACTS_VTABLE);
        assert_eq!(f.e.mem.u32(system + 0x4c), 0x8000_0000);
        assert_eq!(f.e.mem.u8(system + 0x40), 0);
        let populate = &calls_to(log, POPULATE_SYSTEM)[0];
        assert_eq!(populate[..], [0, 0, 0x4242, system]);
        assert_eq!(
            calls_to(log, ARRAY_RESERVE_MORE),
            vec![vec![f.allocator, this + 0xc, 4]]
        );
        assert_eq!(words(&f.e, this + 0xc), vec![system]);
        // With a cinfo and no contact saving, only the two world calls happen.
        log.borrow_mut().clear();
        f.e.call(0x00d4_c2d0, &args![this, 0x4343u32, 0u32]);
        assert_eq!(order(log), vec![WORLD_APPLY_CINFO, WORLD_FILL_SYSTEMS]);
        assert_eq!(calls_to(log, WORLD_APPLY_CINFO), vec![vec![0x4343, info]]);
    }

    /// An action whose entities (`entities`) are reported through virtual function `0x10` in an
    /// array whose capacity word is the count (so that the storage is given back).
    fn action(f: &mut Fixture, entities: &[u32]) -> u32 {
        let block = f.e.mem.alloc(4 * entities.len() as u32 + 4);
        for (index, entity) in entities.iter().enumerate() {
            f.e.mem.set_u32(block + 4 * index as u32, *entity);
        }
        let table = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(table + 0x10, ACTION_ENTITIES);
        let action = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(action, table);
        f.e.mem.set_u32(action + 8, block);
        f.e.mem.set_u32(action + 0xc, entities.len() as u32);
        action
    }

    fn constraint(e: &mut Engine, a: u32, b: u32) -> u32 {
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0x14, a);
        e.mem.set_u32(constraint + 0x18, b);
        constraint
    }

    fn splitting(f: &mut Fixture, active: Vec<u32>) {
        system_model(f);
        active_entities(f, active);
        f.e.register_double(ACTION_ENTITIES, move |e, a| {
            let (data, count) = (e.mem.u32(a[0] + 8), e.mem.u32(a[0] + 0xc));
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, count);
            Ret::default()
        });
    }

    #[test]
    fn splitting_groups_connected_bodies_and_sorts_the_rest_by_motion() {
        let mut f = fixture();
        splitting(&mut f, vec![]);
        let b1 = entity(&mut f.e, 1, 1);
        let b2 = entity(&mut f.e, 1, 1);
        let b3 = entity(&mut f.e, 1, 1);
        let b4 = entity(&mut f.e, 1, 1);
        let b5 = entity(&mut f.e, 1, 0);
        let b6 = entity(&mut f.e, MOTION_FIXED, 1);
        let b7 = entity(&mut f.e, MOTION_KEYFRAMED, 1);
        active_entities(&mut f, vec![b1]);
        let c1 = constraint(&mut f.e, b1, b2);
        let c2 = constraint(&mut f.e, b3, b2);
        let a1 = action(&mut f, &[b3, b4]);
        let phantom = 0x9999u32;
        let source = system_with(&mut f.e, &[b1, b2, b3, b4, b5, b6, b7]);
        push_word(&mut f.e, source + 0x14, c1);
        push_word(&mut f.e, source + 0x14, c2);
        push_word(&mut f.e, source + 0x20, a1);
        push_word(&mut f.e, source + 0x2c, phantom);
        let output = f.e.mem.alloc(0x40);
        f.e.call(0x00d4_c3b0, &args![source, output]);
        let log = &f.log;
        let [fixed, keyframed, unconstrained, phantoms] =
            [0, 4, 8, 0xc].map(|offset| f.e.mem.u32(output + offset));
        // The source is only read; its copy is destroyed at the end.
        assert_eq!(words(&f.e, source + 8), vec![b1, b2, b3, b4, b5, b6, b7]);
        assert_eq!(words(&f.e, fixed + 8), vec![b6]);
        assert_eq!(words(&f.e, keyframed + 8), vec![b7]);
        assert_eq!(words(&f.e, unconstrained + 8), vec![b5]);
        assert_eq!(words(&f.e, phantoms + 0x2c), vec![phantom]);
        let groups = words(&f.e, output + 0x10);
        assert_eq!(groups.len(), 1);
        assert_eq!(words(&f.e, groups[0] + 8), vec![b1, b2, b3, b4]);
        assert_eq!(words(&f.e, groups[0] + 0x14), vec![c1, c2]);
        assert_eq!(words(&f.e, groups[0] + 0x20), vec![a1]);
        assert_eq!(f.e.mem.u8(groups[0] + 0x40), 1);
        assert_eq!(f.e.mem.u32(groups[0] + 0x3c), 0x5eed);
        // Active bytes: b5 has no island word so it counts as active; the keyframed body is
        // asked and inactive; the fixed system's byte is never touched.
        assert_eq!(f.e.mem.u8(unconstrained + 0x40), 1);
        assert_eq!(f.e.mem.u8(keyframed + 0x40), 0);
        assert_eq!(f.e.mem.u8(fixed + 0x40), 0);
        assert_eq!(
            calls_to(log, ENTITY_IS_ACTIVE),
            [b1, b5, b6, b6, b7, b7].map(|b| vec![b]).to_vec()
        );
        let names: Vec<u32> = calls_to(log, STRING_PTR_ASSIGN)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(
            names,
            vec![
                NAME_UNCONSTRAINED,
                NAME_FIXED,
                NAME_KEYFRAMED,
                NAME_CONSTRAINED,
                NAME_CONSTRAINED,
                NAME_CONSTRAINED,
                NAME_CONSTRAINED,
                NAME_PHANTOMS,
            ]
        );
        // The three lone groups and the working copy are destroyed with argument 1; the
        // group of bodies and the three output systems are not.
        let destroyed: Vec<Vec<u32>> = calls_to(log, SYSTEM_DESTRUCT);
        assert_eq!(destroyed.len(), 4);
        assert!(destroyed.iter().all(|call| call[1] == 1));
        assert!(!destroyed.iter().any(|call| call[0] == groups[0]));
        // The action's entity array (capacity 2) was given back once per scan.
        assert!(!calls_to(log, SLOT_FREE).is_empty());
    }

    #[test]
    fn splitting_leaves_fixed_neighbours_in_place() {
        let mut f = fixture();
        splitting(&mut f, vec![]);
        let body = entity(&mut f.e, 1, 1);
        let wall = entity(&mut f.e, MOTION_FIXED, 1);
        let joint = constraint(&mut f.e, body, wall);
        let source = system_with(&mut f.e, &[body, wall]);
        push_word(&mut f.e, source + 0x14, joint);
        let output = f.e.mem.alloc(0x40);
        f.e.call(0x00d4_c3b0, &args![source, output]);
        let groups = words(&f.e, output + 0x10);
        assert_eq!(groups.len(), 1);
        assert_eq!(words(&f.e, groups[0] + 8), vec![body]);
        assert_eq!(words(&f.e, groups[0] + 0x14), vec![joint]);
        let fixed = f.e.mem.u32(output);
        assert_eq!(words(&f.e, fixed + 8), vec![wall]);
        // Nothing but the group and the fixed system has bodies.
        assert_eq!(f.e.mem.u32(output + 4), 0);
        assert_eq!(f.e.mem.u32(output + 8), 0);
        assert_eq!(f.e.mem.u32(output + 0xc), 0);
    }

    #[test]
    fn splitting_an_empty_system_gives_back_every_output_system() {
        let mut f = fixture();
        splitting(&mut f, vec![]);
        let source = system_with(&mut f.e, &[]);
        let output = f.e.mem.alloc(0x40);
        for offset in [0, 4, 8, 0xc] {
            f.e.mem.set_u32(output + offset, 0x1111);
        }
        f.e.call(0x00d4_c3b0, &args![source, output]);
        for offset in [0, 4, 8, 0xc] {
            assert_eq!(f.e.mem.u32(output + offset), 0);
        }
        // The three empty systems and the working copy are destroyed.
        assert_eq!(calls_to(&f.log, SYSTEM_DESTRUCT).len(), 4);
        assert!(calls_to(&f.log, SYSTEM_ADD_RIGID_BODY).is_empty());
        assert_eq!(f.e.mem.u32(output + 0x14), 0);
    }

    #[test]
    fn an_action_with_a_body_twice_moves_two_actions() {
        // The exe does not stop after the first match: the action that took the removed one's
        // place at the same index is matched by the second entry as well.
        let mut f = fixture();
        splitting(&mut f, vec![]);
        let body = entity(&mut f.e, 1, 1);
        let first = action(&mut f, &[body, body]);
        let second = action(&mut f, &[body]);
        let source = system_with(&mut f.e, &[body]);
        push_word(&mut f.e, source + 0x20, first);
        push_word(&mut f.e, source + 0x20, second);
        let output = f.e.mem.alloc(0x40);
        f.e.call(0x00d4_c3b0, &args![source, output]);
        let groups = words(&f.e, output + 0x10);
        assert_eq!(groups.len(), 1);
        assert_eq!(words(&f.e, groups[0] + 0x20), vec![first, second]);
        assert_eq!(
            calls_to(&f.log, SYSTEM_REMOVE_ACTION),
            vec![vec![calls_to(&f.log, SYSTEM_CONSTRUCT)[0][0], 0]; 2]
        );
    }
}
