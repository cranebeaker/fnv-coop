//! `BSHavok/bhkconstraintchain.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConstraintChain` is a `bhkConstraint` (vtable `010ca14c`) whose Havok constraint
//! (`+8`) is an `hkpConstraintChainInstance`: its chained entities are the `hkArray` at `+0x38`
//! (data pointer) and `+0x3c` (count). `hkConstraintChainCinfo` extends `hkConstraintCinfo`
//! (0x14 bytes: `pData` at `+4`, priority, the two entities) with the `hkArray` of entities at
//! `+0x14` (data), `+0x18` (count) and `+0x1c` (capacity in the low 30 bits, flags above).
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie check (`00ec408c`) of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStream::ReadMultipleLinkIDs` (Xbox PDB, `00a64670`, thiscall on the stream, no stack
/// arguments), `GetNumberOfLinkIDs` (`00a63460`) and `GetObjectFromLinkID` (`00a63430`); both
/// thiscall on the stream with no stack arguments.
const READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `hkConstraintCinfo::LoadBinary` (`00c8a230`) and `SaveBinary` (`00c8a280`); both thiscall
/// with the stream.
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
/// `bhkConstraint` (engine map names): `SaveBinary` (`00c89920`), `RegisterStreamables`
/// (`00c898e0`), `IsEqual` (`00c896c0`), `~bhkConstraint` (`00c89640`), `CopyCinfo`
/// (`00c89c80`), `CopyMembers` (`00c89730`, thiscall: the source and the process) and
/// `LinkObject` (`00c89860`); all thiscall with their arguments on the stack.
const CONSTRAINT_SAVE_BINARY: u32 = 0x00c8_9920;
const CONSTRAINT_REGISTER_STREAMABLES: u32 = 0x00c8_98e0;
const CONSTRAINT_IS_EQUAL: u32 = 0x00c8_96c0;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_LINK_OBJECT: u32 = 0x00c8_9860;
/// `bhkSerializable::Kill` (engine map name, `00c876a0`, thiscall, no stack arguments).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// The Havok constraint chain instance: constructor (`00dd0530`, thiscall on the block, one
/// stack argument: the constraint data) and `addEntity` (`00dd04d0`, thiscall, the entity).
const INSTANCE_CONSTRUCT: u32 = 0x00dd_0530;
const INSTANCE_ADD_ENTITY: u32 = 0x00dd_04d0;
/// Reference count decrement of a Havok object (`00c905b0`, thiscall, no stack arguments; the
/// engine map does not name it) and `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl:
/// allocator, array, capacity, element size).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The memory router (`00c85750`, no arguments): its field `+0x10` is the allocator, whose
/// virtual `+4` allocates (size) and `+0x10` frees (pointer, size in bytes).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `bhkWorld::Add_ov2` (engine map name, `00c6b320`) and `Remove_ov3` (`00c68d50`); both thiscall
/// on the `bhkWorld` with the Havok constraint as the one stack argument.
const WORLD_ADD: u32 = 0x00c6_b320;
const WORLD_REMOVE: u32 = 0x00c6_8d50;
/// A function of no effect called on a wrapper (`00a29680`: one `RET`).
const NO_EFFECT: u32 = 0x00a2_9680;
/// The Havok string stream: `hkOstream::hkOstream` (Xbox PDB, `00cc0500`, thiscall: buffer,
/// size, the static flag), `operator<<` for a string (`00cc01c0`, thiscall, the string) and the
/// destructor (`00cc00e0`), and the error reporter object at `01268360` whose virtual `+0xc`
/// reports a message (level, id, text, file, line).
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0500;
const OSTREAM_WRITE_STRING: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The text (`010c5e30`) and the file name (`010d0c28`) of the report, its id and its line.
const REPORT_TEXT: u32 = 0x010c_5e30;
const REPORT_FILE: u32 = 0x010d_0c28;
const REPORT_ID: u32 = 0x234f_24fc;
const REPORT_LINE: u32 = 0x168;
/// `bhkConstraintChain::iCount` (Xbox PDB static, `01268f80`) and the vtable (`010ca14c`).
const INSTANCE_COUNT: u32 = 0x0126_8f80;
const CONSTRAINT_CHAIN_VTABLE: u32 = 0x010c_a14c;
/// Per-thread word the functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// Size of the Havok constraint chain instance.
const INSTANCE_SIZE: u32 = 0x48;

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// The allocator of the memory router (`00c85750`, field `+0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes the `hkArray` at `array` hold `wanted` elements: when its capacity (low 30 bits of
/// `+8`) is below `wanted` (signed), reserves the larger of twice the capacity and `wanted`
/// (`hkArrayUtil::_reserve` with 4-byte elements); then stores `wanted` as the size (`+4`).
fn set_array_size(e: &mut Engine, array: u32, wanted: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < (wanted as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if (wanted as i32) < (doubled as i32) {
            doubled
        } else {
            wanted
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, new_capacity, 4u32]);
    }
    e.mem.set_u32(array + 4, wanted);
}

// Translated from 00d53200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintChainCinfo::LoadBinary` (Xbox PDB), `(stream)`: reads the link ids from the
/// stream (`00a64670`), then the base Cinfo load (`00c8a230`).
pub fn hk_constraint_chain_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(READ_MULTIPLE_LINK_IDS, &args![stream]);
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00d53220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d53220 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::~bhkConstraintChain` (Xbox PDB): stores the vtable `010ca14c`, runs
/// `bhkSerializable::Kill` (`00c876a0`), lowers the instance count (`01268f80`) and runs the
/// base destructor (`00c89640`).
pub fn bhk_constraint_chain_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CONSTRAINT_CHAIN_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(INSTANCE_COUNT).wrapping_sub(1);
    e.mem.set_u32(INSTANCE_COUNT, count);
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00d53290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::SaveBinary` (Xbox PDB): a 5-byte jump to `bhkConstraint::SaveBinary`
/// (`00c89920`), passing on `this` and the stream.
pub fn bhk_constraint_chain_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00d532a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::RegisterStreamables` (Xbox PDB), `(stream)`: for each chained entity of
/// the Havok constraint (`+8`) the wrapper object at the entity's `+0xc` is asked to register
/// (its virtual `+0x54` with the stream); then the base registration (`00c898e0`), whose result
/// is the answer.
pub fn bhk_constraint_chain_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    let object = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let mut index = 0;
    while index < e.mem.u32(object + 0x3c) as i32 {
        let entities = e.mem.u32(object + 0x38);
        let entity = e.mem.u32(entities + 4 * index as u32);
        let wrapper = if entity != 0 {
            e.mem.u32(entity + 0xc)
        } else {
            0
        };
        e.vcall(wrapper, 0x54, &args![stream]);
        index += 1;
    }
    e.call(CONSTRAINT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00d532f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::IsEqual` (Xbox PDB), `(other)`: false when the base comparison
/// (`00c896c0`) is; otherwise true when both Havok constraints (`+8`) have the same number of
/// chained entities and the same entity pointers.
pub fn bhk_constraint_chain_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(CONSTRAINT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let theirs = if other.addr() != 0 {
        e.mem.u32(other.addr() + 8)
    } else {
        0
    };
    let count = e.mem.u32(mine + 0x3c) as i32;
    let mut equal = count == e.mem.u32(theirs + 0x3c) as i32;
    let mut index = 0;
    while equal && index < count {
        let mine_entities = e.mem.u32(mine + 0x38);
        let their_entities = e.mem.u32(theirs + 0x38);
        let offset = 4 * index as u32;
        equal = e.mem.u32(mine_entities + offset) == e.mem.u32(their_entities + offset);
        index += 1;
    }
    equal
}

// Translated from 00d53360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintChainCinfo::SaveBinary` (Xbox PDB), `(stream)`: writes the entity count (`+0x18`,
/// 4 bytes through the stream's output object, `stream + 0x250`, function pointer at `+0xc`
/// called as `(output, buffer, size, &size, 1)`, advancing the object's position at `+4`),
/// writes the link id of the wrapper (`+0xc`) of each entity (the stream's virtual `+0x30`; 0
/// for a null entity), then the base Cinfo save (`00c8a280`).
pub fn hk_constraint_chain_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let writer = e.mem.u32(output + 0xc);
    let count = e.mem.u32(this.addr() + 0x18);
    // The count the write takes from, and the size cell holding 4.
    e.with_stack(8, |e, cells| {
        let count_cell = cells.addr();
        let size_cell = count_cell + 4;
        e.mem.set_u32(count_cell, count);
        e.mem.set_u32(size_cell, 4);
        let written = e
            .call(writer, &args![output, count_cell, 4u32, size_cell, 1u32])
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(written));
    });
    let mut index = 0u32;
    while index < count {
        let entities = e.mem.u32(this.addr() + 0x14);
        let entity = e.mem.u32(entities + 4 * index);
        let wrapper = if entity != 0 {
            e.mem.u32(entity + 0xc)
        } else {
            0
        };
        e.vcall(stream.addr(), 0x30, &args![wrapper]);
        index += 1;
    }
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00d533e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d533e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::Init` (Xbox PDB), `(cinfo)`: the argument is the `hkSerializableCinfo`
/// base of the Cinfo (`+4` of the `hkConstraintChainCinfo`; 0 stays 0). Inside the
/// `TLS + 0x2b4 = 0x10` scope, allocates an `hkpConstraintChainInstance` (0x48 bytes from the
/// memory router's allocator, the block's `+4` word set to 0x48), constructs it with the
/// Cinfo's `pData` (`+4`), adds each entity of the Cinfo's array (`+0x14`, count `+0x18`),
/// sets it as the Havok constraint (virtual `+0x8c`), releases the reference the construction
/// gave (`00c905b0`) and calls the virtual `+0xbc` with the argument.
pub fn bhk_constraint_chain_init(e: &mut Engine, this: Ptr, cinfo_base: Ptr) {
    let scope = enter_scope(e);
    let cinfo = if cinfo_base.addr() != 0 {
        cinfo_base.addr().wrapping_sub(4)
    } else {
        0
    };
    let data = e.mem.u32(cinfo + 4);
    let allocator = allocator(e);
    let block = e.vcall(allocator, 4, &args![INSTANCE_SIZE]).u32();
    e.mem.set_u16(block + 4, INSTANCE_SIZE as u16);
    let instance = e.call(INSTANCE_CONSTRUCT, &args![block, data]).u32();
    let mut index = 0;
    while index < e.mem.u32(cinfo + 0x18) as i32 {
        let entities = e.mem.u32(cinfo + 0x14);
        let entity = e.mem.u32(entities + 4 * index as u32);
        e.call(INSTANCE_ADD_ENTITY, &args![instance, entity]);
        index += 1;
    }
    e.vcall(this.addr(), 0x8c, &args![instance]);
    e.call(REMOVE_REFERENCE, &args![instance]);
    e.vcall(this.addr(), 0xbc, &args![cinfo_base]);
    leave_scope(e, scope);
}

// Translated from 00d534e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d534e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::Add` (Xbox PDB), `(world)`: inside the `TLS + 0x2b4 = 0x10` scope,
/// false when there is no Havok constraint (`+8`). With a world, when the world the chain
/// reports (virtual `+0x94`) is not the one the argument reports (its virtual `+0x94`), runs
/// virtual `+0xa0`, then virtual `+0xd8` with the world and 1, adds the Havok constraint to the
/// world (`00c6b320`) and answers true. When the two worlds are the same nothing is called; with no world argument
/// only virtual `+0xa0` runs. Both answer false.
pub fn bhk_constraint_chain_add(e: &mut Engine, this: Ptr, world: Ptr) -> bool {
    let scope = enter_scope(e);
    let object = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let mut added = false;
    if object != 0 {
        if world.addr() == 0 {
            e.vcall(this.addr(), 0xa0, &args![]);
        } else {
            let mine = e.vcall(this.addr(), 0x94, &args![]).u32();
            let theirs = e.vcall(world.addr(), 0x94, &args![]).u32();
            if mine != theirs {
                e.vcall(this.addr(), 0xa0, &args![]);
                e.vcall(this.addr(), 0xd8, &args![world, 1u32]);
                e.call(WORLD_ADD, &args![world, object]);
                added = true;
            }
        }
    }
    leave_scope(e, scope);
    added
}

// Translated from 00d535d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d535d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::Remove` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope, takes the
/// object the chain's virtual `+0x94` answers and its `bhkWorld` (`+0x350`); false when there is
/// none. Otherwise removes the Havok constraint (`+8`) from it (`00c68d50`), runs the chain's
/// virtual `+0xd8` with the world and 0, and answers the removal's result.
pub fn bhk_constraint_chain_remove(e: &mut Engine, this: Ptr) -> bool {
    let scope = enter_scope(e);
    let owner = e.vcall(this.addr(), 0x94, &args![]).u32();
    let world = if owner != 0 {
        e.mem.u32(owner + 0x350)
    } else {
        0
    };
    let mut removed = false;
    if world != 0 {
        let object = e.mem.u32(this.addr() + 8);
        removed = e.call(WORLD_REMOVE, &args![world, object]).bool();
        e.vcall(this.addr(), 0xd8, &args![world, 0u32]);
    }
    leave_scope(e, scope);
    removed
}

// Translated from 00d536a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d536a0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled; the third argument of the string stream constructor keeps the previous value of ECX in its upper three bytes (1 here).
/// `bhkConstraintChain::ProcessEntities` (Xbox PDB), `(world, add)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, goes through the chained entities of the Havok constraint
/// (`+8`). Answers true when an entity or its wrapper (`+0xc`) is missing while adding, else
/// false.
///
/// Adding: a missing entity or wrapper sets the answer and calls the chain's virtual `+0xc8`
/// with the `+0x24` word of the object the world's virtual `+0x94` answers (0 for none). Otherwise,
/// unless the wrapper's Havok entity (`+8`) is in a world (its `+8` word is set), the high
/// 16 bits of the entity's `+0x2c` word are taken from the `+0x2c` word of the object the
/// chain's virtual `+0xcc` answers. Then, when the world's `+0x18` word is 0: the wrapper's
/// virtual `+0x9c` runs with the world and, when the wrapper's virtual `+0x94` answers 0 and
/// its flags byte (`+0x10`) has neither bit 4 nor bit 5 set, a message is reported (`hkOstream`
/// on a 0x200-byte stack buffer holding the text `010c5e30`, sent to the error reporter's
/// virtual `+0xc` with level 1, id `234f24fc`, file `010d0c28`, line 0x168) and the wrapper's
/// virtual `+0xc8` runs with the world.
///
/// Removing: the chain's virtual `+0xc8` runs with 0 for each entity that is the `+0x24` word
/// of the object the world's virtual `+0x94` answers.
pub fn bhk_constraint_chain_process_entities(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    add: bool,
) -> bool {
    let scope = enter_scope(e);
    let mut missing = false;
    let object = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let mut index = 0;
    while index < e.mem.u32(object + 0x3c) as i32 {
        let entities = e.mem.u32(object + 0x38);
        let entity = e.mem.u32(entities + 4 * index as u32);
        if add {
            let wrapper = if entity != 0 {
                e.mem.u32(entity + 0xc)
            } else {
                0
            };
            if wrapper == 0 {
                missing = true;
                let owner = e.vcall(world.addr(), 0x94, &args![]).u32();
                let word = if owner != 0 {
                    e.mem.u32(owner + 0x24)
                } else {
                    0
                };
                e.vcall(this.addr(), 0xc8, &args![word]);
            } else {
                add_entity(e, this, world, entity, wrapper);
            }
        } else {
            let owner = e.vcall(world.addr(), 0x94, &args![]).u32();
            let word = if owner != 0 {
                e.mem.u32(owner + 0x24)
            } else {
                0
            };
            if entity == word {
                e.vcall(this.addr(), 0xc8, &args![0u32]);
            }
        }
        index += 1;
    }
    leave_scope(e, scope);
    missing
}

/// The adding branch of [`bhk_constraint_chain_process_entities`] for an entity with a wrapper.
fn add_entity(e: &mut Engine, this: Ptr, world: Ptr, entity: u32, wrapper: u32) {
    let havok_entity = e.mem.u32(wrapper + 8);
    let mut in_world = false;
    if havok_entity != 0 {
        e.call(NO_EFFECT, &args![wrapper]);
        in_world = e.mem.u32(havok_entity + 8) != 0;
        e.call(NO_EFFECT, &args![wrapper]);
    }
    if !in_world {
        let source = e.vcall(this.addr(), 0xcc, &args![]).u32();
        let high = e.mem.u32(source + 0x2c) & 0xffff_0000;
        let low = u32::from(e.mem.u16(entity + 0x2c));
        e.mem.set_u32(entity + 0x2c, high | low);
    }
    if e.mem.u32(world.addr() + 0x18) != 0 {
        return;
    }
    e.vcall(wrapper, 0x9c, &args![world]);
    let answer = e.vcall(wrapper, 0x94, &args![]).u32();
    if answer != 0 || e.mem.u8(wrapper + 0x10) & 0x30 != 0 {
        return;
    }
    // The hkOstream object (0xc bytes) and its 0x200-byte buffer.
    e.with_stack(0x20c, |e, frame| {
        let stream = frame.addr();
        let buffer = stream + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, 0x200u32, 1u32]);
        e.call(OSTREAM_WRITE_STRING, &args![stream, REPORT_TEXT]);
        let reporter = e.mem.u32(ERROR_REPORTER);
        e.vcall(
            reporter,
            0xc,
            &args![1u32, REPORT_ID, buffer, REPORT_FILE, REPORT_LINE],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
    e.vcall(wrapper, 0xc8, &args![world]);
}

// Translated from 00d53900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::CopyCinfo` (Xbox PDB), `(cinfo)`: the base copy (`00c89c80`), then,
/// with a Havok constraint (`+8`), makes the Cinfo's entity array (`+0x14`) as large as the
/// constraint's chained entities and copies their pointers.
pub fn bhk_constraint_chain_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    e.call(CONSTRAINT_COPY_CINFO, &args![this, cinfo]);
    if this.addr() == 0 {
        return;
    }
    let object = e.mem.u32(this.addr() + 8);
    if object == 0 {
        return;
    }
    let count = e.mem.u32(object + 0x3c);
    let array = cinfo.addr() + 0x14;
    set_array_size(e, array, count);
    let mut index = 0;
    while index < e.mem.u32(object + 0x3c) as i32 {
        let entities = e.mem.u32(object + 0x38);
        let entity = e.mem.u32(entities + 4 * index as u32);
        let target = e.mem.u32(array);
        e.mem.set_u32(target + 4 * index as u32, entity);
        index += 1;
    }
}

// Translated from 00d53970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d53970 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::LinkObject` (Xbox PDB), `(stream)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, makes the entity array of the Cinfo (the object the chain's virtual `+0xb4` answers,
/// minus 4; its array at `+0x14`) as large as the stream's link id count (`00a63460`) and
/// fills it with the Havok entity (`+8`) of the objects the stream links (`00a63430`, one per
/// element), then the base link (`00c89860`).
pub fn bhk_constraint_chain_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let scope = enter_scope(e);
    let count = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    // The virtual `+0xb4` takes the address of a stack word (the parameter's own slot).
    let answer = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), stream.addr());
        e.vcall(this.addr(), 0xb4, &args![cell]).u32()
    });
    let cinfo = if answer != 0 {
        answer.wrapping_sub(4)
    } else {
        0
    };
    let array = cinfo + 0x14;
    set_array_size(e, array, count);
    let mut index = 0u32;
    while index < count {
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let entity = e.mem.u32(object + 8);
        let data = e.mem.u32(array);
        e.mem.set_u32(data + 4 * index, entity);
        index += 1;
    }
    e.call(CONSTRAINT_LINK_OBJECT, &args![this, stream]);
    leave_scope(e, scope);
}

// Translated from 00d53a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d53a80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraintChain::CopyMembers` (Xbox PDB), `(source, process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, takes the Cinfo (the object the chain's virtual `+0xb4` answers,
/// minus 4), moves the wrapper (`+0xc`) of each of its entities into a temporary array and
/// empties its entity array (count 0). Then refills the Cinfo's array with the Havok entities
/// (`+8`) of the clones of those wrappers: for each, the cloning process's map (`process + 0`,
/// virtual `+4` the bucket index of the key, virtual `+8` key equality, buckets at `+8`, items
/// `(next, key, value)`) gives the clone when there is one, else the wrapper clones itself
/// (virtual `+0x48` with the process). Last the base copy (`00c89730`, with the source and the
/// process) and the temporary array's buffer is freed through the allocator (virtual `+0x10`:
/// pointer, byte size) unless it was never allocated.
pub fn bhk_constraint_chain_copy_members(e: &mut Engine, this: Ptr, source: Ptr, process: Ptr) {
    let scope = enter_scope(e);
    // The temporary `hkArray` (data, size, capacity and flags) starts empty with the "do not
    // deallocate" flag.
    e.with_stack(0xc, |e, temporary| {
        let temporary = temporary.addr();
        e.mem.set_u32(temporary, 0);
        e.mem.set_u32(temporary + 4, 0);
        e.mem.set_u32(temporary + 8, 0x8000_0000);
        // The virtual `+0xb4` takes the address of a bool the answer does not depend on.
        let answer = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
        let cinfo = if answer != 0 {
            answer.wrapping_sub(4)
        } else {
            0
        };
        let count = e.mem.u32(cinfo + 0x18);
        set_array_size(e, temporary, count);
        let mut index = 0;
        while index < e.mem.u32(cinfo + 0x18) as i32 {
            let entities = e.mem.u32(cinfo + 0x14);
            let entity = e.mem.u32(entities + 4 * index as u32);
            let wrapper = if entity != 0 {
                e.mem.u32(entity + 0xc)
            } else {
                0
            };
            let data = e.mem.u32(temporary);
            e.mem.set_u32(data + 4 * index as u32, wrapper);
            index += 1;
        }
        e.mem.set_u32(cinfo + 0x18, 0);
        let moved = e.mem.u32(temporary + 4);
        let array = cinfo + 0x14;
        set_array_size(e, array, moved);
        let mut index = 0;
        while index < e.mem.u32(temporary + 4) as i32 {
            let map = e.mem.u32(process.addr());
            let wrapper = e.mem.u32(e.mem.u32(temporary) + 4 * index as u32);
            let bucket = e.vcall(map, 4, &args![wrapper]).u32();
            let mut item = e.mem.u32(e.mem.u32(map + 8) + 4 * bucket);
            let mut clone = 0;
            while item != 0 {
                let key = e.mem.u32(item + 4);
                if e.vcall(map, 8, &args![wrapper, key]).bool() {
                    clone = e.mem.u32(item + 8);
                    break;
                }
                item = e.mem.u32(item);
            }
            if clone == 0 {
                let wrapper = e.mem.u32(e.mem.u32(temporary) + 4 * index as u32);
                clone = e.vcall(wrapper, 0x48, &args![process]).u32();
            }
            let entity = e.mem.u32(clone + 8);
            let data = e.mem.u32(array);
            e.mem.set_u32(data + 4 * index as u32, entity);
            index += 1;
        }
        e.call(CONSTRAINT_COPY_MEMBERS, &args![this, source, process]);
        let allocator = allocator(e);
        let capacity = e.mem.u32(temporary + 8);
        e.mem.set_u32(temporary + 4, 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            let data = e.mem.u32(temporary);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_3200, hk_constraint_chain_cinfo_load_binary(Ptr, Ptr)),
        entry!(0x00d5_3220, bhk_constraint_chain_destructor(Ptr)),
        entry!(0x00d5_3290, bhk_constraint_chain_save_binary(Ptr, Ptr)),
        entry!(0x00d5_32a0, bhk_constraint_chain_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00d5_32f0, bhk_constraint_chain_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00d5_3360, hk_constraint_chain_cinfo_save_binary(Ptr, Ptr)),
        entry!(0x00d5_33e0, bhk_constraint_chain_init(Ptr, Ptr)),
        entry!(0x00d5_34e0, bhk_constraint_chain_add(Ptr, Ptr) -> bool),
        entry!(0x00d5_35d0, bhk_constraint_chain_remove(Ptr) -> bool),
        entry!(0x00d5_36a0, bhk_constraint_chain_process_entities(Ptr, Ptr, bool) -> bool),
        entry!(0x00d5_3900, bhk_constraint_chain_copy_cinfo(Ptr, Ptr)),
        entry!(0x00d5_3970, bhk_constraint_chain_link_object(Ptr, Ptr)),
        entry!(
            0x00d5_3a80,
            bhk_constraint_chain_copy_members(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual slot targets: the vtable of the chain, of a wrapper, of the world and of
    // the stream, by the slot offset added.
    const THIS_VTABLE: u32 = 0x7001_0000;
    const WRAPPER_VTABLE: u32 = 0x7001_1000;
    const WORLD_VTABLE: u32 = 0x7001_2000;
    const STREAM_VTABLE: u32 = 0x7001_3000;
    const MAP_VTABLE: u32 = 0x7001_4000;
    const ALLOCATOR_VTABLE: u32 = 0x7001_5000;
    const THIS_SLOT: u32 = 0x7000_0000;
    const WRAPPER_SLOT: u32 = 0x7000_1000;
    const WORLD_SLOT: u32 = 0x7000_2000;
    const STREAM_SLOT: u32 = 0x7000_3000;
    const MAP_SLOT: u32 = 0x7000_4000;
    const ALLOCATOR_SLOT: u32 = 0x7000_5000;
    const ROUTER_OBJECT: u32 = 0x7100_0000;
    const WRITER: u32 = 0x7000_6000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// A vtable at `vtable` whose listed slots point at `fake_base + slot`.
    fn put_table(e: &mut Engine, vtable: u32, fake_base: u32, slots: &[u32]) {
        let size = slots.iter().max().map_or(0, |slot| slot / 4 + 1);
        let mut table = vec![0u32; size as usize];
        for slot in slots {
            table[(*slot / 4) as usize] = fake_base + slot;
        }
        e.put_vtable(vtable, &table);
    }

    /// The memory router with an allocator whose allocation (virtual `+4`) hands out zeroed
    /// blocks; `hkArrayUtil::_reserve` allocates the buffer and records the capacity.
    fn install_router(e: &mut Engine) -> u32 {
        let allocator = ROUTER_OBJECT + 0x20;
        put_table(e, ALLOCATOR_VTABLE, ALLOCATOR_SLOT, &[4, 0x10]);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, allocator);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOCATOR_SLOT + 4, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATOR_SLOT + 0x10, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        allocator
    }

    /// A chain object whose Havok constraint holds the entities `entities`.
    fn chain_with_entities(e: &mut Engine, entities: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x14);
        let object = e.mem.alloc(0x48);
        let data = e.mem.alloc(0x40);
        for (index, entity) in entities.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entity);
        }
        e.mem.set_u32(this + 8, object);
        e.mem.set_u32(object + 0x38, data);
        e.mem.set_u32(object + 0x3c, entities.len() as u32);
        (this, object)
    }

    #[test]
    fn hk_constraint_chain_cinfo_load_binary_reads_the_link_ids_then_the_base() {
        let mut e = logged_engine();
        e.register(READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        e.register(CINFO_LOAD_BINARY, |_, _| Ret::default());
        e.call(0x00d5_3200, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, READ_MULTIPLE_LINK_IDS), vec![vec![0x2000]]);
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![0x1000, 0x2000]]);
        let order = vec![READ_MULTIPLE_LINK_IDS, CINFO_LOAD_BINARY];
        assert_eq!(order_of(&e, &order), order);
    }

    #[test]
    fn bhk_constraint_chain_destructor_kills_counts_down_and_runs_the_base() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONSTRAINT_DESTRUCT, |_, _| Ret::default());
        e.mem.set_u32(INSTANCE_COUNT, 5);
        let this = e.mem.alloc(0x14);
        e.call(0x00d5_3220, &args![this]);
        assert_eq!(e.mem.u32(this), CONSTRAINT_CHAIN_VTABLE);
        assert_eq!(e.mem.u32(INSTANCE_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
        let order = vec![SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT];
        assert_eq!(order_of(&e, &order), order);
    }

    #[test]
    fn bhk_constraint_chain_save_binary_forwards_to_the_base() {
        let mut e = logged_engine();
        e.register(CONSTRAINT_SAVE_BINARY, |_, _| Ret::default());
        e.call(0x00d5_3290, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, CONSTRAINT_SAVE_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn bhk_constraint_chain_register_streamables_registers_each_entity_wrapper() {
        let mut e = logged_engine();
        put_table(&mut e, WRAPPER_VTABLE, WRAPPER_SLOT, &[0x54]);
        e.register(WRAPPER_SLOT + 0x54, |_, _| Ret::default());
        e.register(CONSTRAINT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        let mut entities = vec![];
        let mut wrappers = vec![];
        for _ in 0..2 {
            let wrapper = e.mem.alloc(0x20);
            e.mem.set_u32(wrapper, WRAPPER_VTABLE);
            let entity = e.mem.alloc(0x20);
            e.mem.set_u32(entity + 0xc, wrapper);
            entities.push(entity);
            wrappers.push(wrapper);
        }
        let (this, _) = chain_with_entities(&mut e, &entities);
        assert!(e.call(0x00d5_32a0, &args![this, 0x9000u32]).bool());
        assert_eq!(
            calls_to(&e, WRAPPER_SLOT + 0x54),
            vec![vec![wrappers[0], 0x9000], vec![wrappers[1], 0x9000]]
        );
        assert_eq!(
            calls_to(&e, CONSTRAINT_REGISTER_STREAMABLES),
            vec![vec![this, 0x9000]]
        );
        let order = vec![
            WRAPPER_SLOT + 0x54,
            WRAPPER_SLOT + 0x54,
            CONSTRAINT_REGISTER_STREAMABLES,
        ];
        assert_eq!(order_of(&e, &order), order);
        // The base answer is the answer; no entities means only the base runs.
        e.register(CONSTRAINT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        let (empty, _) = chain_with_entities(&mut e, &[]);
        clear_log(&mut e);
        assert!(!e.call(0x00d5_32a0, &args![empty, 0x9000u32]).bool());
        assert!(calls_to(&e, WRAPPER_SLOT + 0x54).is_empty());
    }

    #[test]
    fn bhk_constraint_chain_is_equal_compares_the_entity_pointers() {
        let mut e = logged_engine();
        e.register(CONSTRAINT_IS_EQUAL, |_, _| 1u32.into_ret());
        let (a, _) = chain_with_entities(&mut e, &[0x10, 0x20, 0x30]);
        let (b, _) = chain_with_entities(&mut e, &[0x10, 0x20, 0x30]);
        assert!(e.call(0x00d5_32f0, &args![a, b]).bool());
        assert_eq!(calls_to(&e, CONSTRAINT_IS_EQUAL), vec![vec![a, b]]);
        let (different, _) = chain_with_entities(&mut e, &[0x10, 0x20, 0x31]);
        assert!(!e.call(0x00d5_32f0, &args![a, different]).bool());
        let (shorter, _) = chain_with_entities(&mut e, &[0x10, 0x20]);
        assert!(!e.call(0x00d5_32f0, &args![a, shorter]).bool());
        let (empty, _) = chain_with_entities(&mut e, &[]);
        let (other_empty, _) = chain_with_entities(&mut e, &[]);
        assert!(e.call(0x00d5_32f0, &args![empty, other_empty]).bool());
        // A false base answer is the answer without looking at the entities.
        e.register(CONSTRAINT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00d5_32f0, &args![a, b]).bool());
    }

    #[test]
    fn hk_constraint_chain_cinfo_save_binary_writes_the_count_and_the_link_ids() {
        let mut e = logged_engine();
        put_table(&mut e, STREAM_VTABLE, STREAM_SLOT, &[0x30]);
        e.register(STREAM_SLOT + 0x30, |_, _| Ret::default());
        e.register(CINFO_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITER, |e, a| {
            // Keeps the count and the size cell the write was given.
            let count = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 0x10, count);
            let size = e.mem.u32(a[3]);
            e.mem.set_u32(a[0] + 0x14, size);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream, STREAM_VTABLE);
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output + 0xc, WRITER);
        e.mem.set_u32(output + 4, 10);
        e.mem.set_u32(stream + 0x250, output);
        let this = e.mem.alloc(0x30);
        let array = e.mem.alloc(0x10);
        let wrapper = e.mem.alloc(0x20);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, wrapper);
        e.mem.set_u32(array, entity);
        e.mem.set_u32(this + 0x14, array);
        e.mem.set_u32(this + 0x18, 2);
        e.call(0x00d5_3360, &args![this, stream]);
        let writes = calls_to(&e, WRITER);
        assert_eq!(writes.len(), 1);
        assert_eq!((writes[0][0], writes[0][2], writes[0][4]), (output, 4, 1));
        assert_eq!(e.mem.u32(output + 0x10), 2);
        assert_eq!(e.mem.u32(output + 0x14), 4);
        assert_eq!(e.mem.u32(output + 4), 14);
        // The second entity is null: its link id is written for 0.
        assert_eq!(
            calls_to(&e, STREAM_SLOT + 0x30),
            vec![vec![stream, wrapper], vec![stream, 0]]
        );
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![this, stream]]);
        let order = vec![
            WRITER,
            STREAM_SLOT + 0x30,
            STREAM_SLOT + 0x30,
            CINFO_SAVE_BINARY,
        ];
        assert_eq!(order_of(&e, &order), order);
        // No entities: the count and the base only.
        clear_log(&mut e);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00d5_3360, &args![this, stream]);
        assert!(calls_to(&e, STREAM_SLOT + 0x30).is_empty());
        assert_eq!(calls_to(&e, WRITER).len(), 1);
    }

    #[test]
    fn bhk_constraint_chain_init_creates_the_instance_from_the_cinfo() {
        let mut e = logged_engine();
        let allocator = install_router(&mut e);
        put_table(&mut e, THIS_VTABLE, THIS_SLOT, &[0x8c, 0xbc]);
        e.register(THIS_SLOT + 0x8c, |_, _| Ret::default());
        e.register(THIS_SLOT + 0xbc, |_, _| Ret::default());
        e.register(INSTANCE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(INSTANCE_ADD_ENTITY, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, THIS_VTABLE);
        let cinfo = e.mem.alloc(0x30);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo + 4, 0x5555);
        e.mem.set_u32(cinfo + 0x14, array);
        e.mem.set_u32(cinfo + 0x18, 2);
        e.mem.set_u32(array, 0xe1);
        e.mem.set_u32(array + 4, 0xe2);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00d5_33e0, &args![this, cinfo + 4]);
        assert_eq!(
            calls_to(&e, ALLOCATOR_SLOT + 4),
            vec![vec![allocator, 0x48]]
        );
        let constructs = calls_to(&e, INSTANCE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(constructs[0][1], 0x5555);
        assert_eq!(e.mem.u16(block + 4), 0x48);
        assert_eq!(
            calls_to(&e, INSTANCE_ADD_ENTITY),
            vec![vec![block, 0xe1], vec![block, 0xe2]]
        );
        assert_eq!(calls_to(&e, THIS_SLOT + 0x8c), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, THIS_SLOT + 0xbc), vec![vec![this, cinfo + 4]]);
        let order = vec![
            ALLOCATOR_SLOT + 4,
            INSTANCE_CONSTRUCT,
            INSTANCE_ADD_ENTITY,
            INSTANCE_ADD_ENTITY,
            THIS_SLOT + 0x8c,
            REMOVE_REFERENCE,
            THIS_SLOT + 0xbc,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // No entities: none added.
        clear_log(&mut e);
        e.mem.set_u32(cinfo + 0x18, 0);
        e.call(0x00d5_33e0, &args![this, cinfo + 4]);
        assert!(calls_to(&e, INSTANCE_ADD_ENTITY).is_empty());
        assert_eq!(calls_to(&e, THIS_SLOT + 0x8c).len(), 1);
    }

    /// A chain with the vtable slots the world functions use, and the world object.
    fn chain_and_world(e: &mut Engine) -> (u32, u32) {
        put_table(e, THIS_VTABLE, THIS_SLOT, &[0x94, 0xa0, 0xc8, 0xcc, 0xd8]);
        put_table(e, WORLD_VTABLE, WORLD_SLOT, &[0x94]);
        let (this, _) = chain_with_entities(e, &[]);
        e.mem.set_u32(this, THIS_VTABLE);
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(world, WORLD_VTABLE);
        (this, world)
    }

    #[test]
    fn bhk_constraint_chain_add_adds_the_constraint_to_a_different_world() {
        let mut e = logged_engine();
        let (this, world) = chain_and_world(&mut e);
        e.register(THIS_SLOT + 0x94, |_, _| 0x111u32.into_ret());
        e.register(WORLD_SLOT + 0x94, |_, _| 0x222u32.into_ret());
        e.register(THIS_SLOT + 0xa0, |_, _| Ret::default());
        e.register(THIS_SLOT + 0xd8, |_, _| Ret::default());
        e.register(WORLD_ADD, |_, _| Ret::default());
        let object = e.mem.u32(this + 8);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        assert!(e.call(0x00d5_34e0, &args![this, world]).bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xa0), vec![vec![this]]);
        assert_eq!(calls_to(&e, THIS_SLOT + 0xd8), vec![vec![this, world, 1]]);
        assert_eq!(calls_to(&e, WORLD_ADD), vec![vec![world, object]]);
        let order = vec![THIS_SLOT + 0xa0, THIS_SLOT + 0xd8, WORLD_ADD];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // The same world on both sides: nothing is called, answer false.
        clear_log(&mut e);
        e.register(WORLD_SLOT + 0x94, |_, _| 0x111u32.into_ret());
        assert!(!e.call(0x00d5_34e0, &args![this, world]).bool());
        assert!(calls_to(&e, THIS_SLOT + 0xa0).is_empty());
        assert!(calls_to(&e, WORLD_ADD).is_empty());
        // No world: only the virtual +0xa0.
        clear_log(&mut e);
        assert!(!e.call(0x00d5_34e0, &args![this, 0u32]).bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xa0).len(), 1);
        assert!(calls_to(&e, WORLD_ADD).is_empty());
        // No Havok constraint: nothing.
        clear_log(&mut e);
        e.mem.set_u32(this + 8, 0);
        assert!(!e.call(0x00d5_34e0, &args![this, world]).bool());
        assert!(calls_to(&e, THIS_SLOT + 0xa0).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_constraint_chain_remove_removes_the_constraint_from_its_world() {
        let mut e = logged_engine();
        let (this, _) = chain_and_world(&mut e);
        e.register(THIS_SLOT + 0x94, |e, _| {
            // The owner object, whose +0x350 is the world.
            let owner = e.mem.alloc(0x360);
            e.mem.set_u32(owner + 0x350, 0x4000);
            owner.into_ret()
        });
        e.register(THIS_SLOT + 0xd8, |_, _| Ret::default());
        e.register(WORLD_REMOVE, |_, _| 1u32.into_ret());
        let object = e.mem.u32(this + 8);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        assert!(e.call(0x00d5_35d0, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE), vec![vec![0x4000, object]]);
        assert_eq!(calls_to(&e, THIS_SLOT + 0xd8), vec![vec![this, 0x4000, 0]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // The removal's answer is the answer.
        e.register(WORLD_REMOVE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00d5_35d0, &args![this]).bool());
        // No world on the owner: nothing, false.
        clear_log(&mut e);
        e.register(THIS_SLOT + 0x94, |e, _| e.mem.alloc(0x360).into_ret());
        assert!(!e.call(0x00d5_35d0, &args![this]).bool());
        assert!(calls_to(&e, WORLD_REMOVE).is_empty());
        // No owner at all.
        e.register(THIS_SLOT + 0x94, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00d5_35d0, &args![this]).bool());
        assert!(calls_to(&e, WORLD_REMOVE).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    const REPORTER_VTABLE: u32 = 0x7001_6000;
    const REPORTER_SLOT: u32 = 0x7000_7000;
    const REPORTER_OBJECT: u32 = 0x7100_1000;

    /// A chain with two entities, each with a wrapper; the doubles of `ProcessEntities`.
    struct Entities {
        this: u32,
        world: u32,
        entities: Vec<u32>,
        wrappers: Vec<u32>,
        havok_entities: Vec<u32>,
    }

    fn entities_scene(e: &mut Engine) -> Entities {
        let (this, world) = chain_and_world(e);
        put_table(e, WRAPPER_VTABLE, WRAPPER_SLOT, &[0x94, 0x9c, 0xc8]);
        put_table(e, REPORTER_VTABLE, REPORTER_SLOT, &[0xc]);
        e.register(NO_EFFECT, |_, _| Ret::default());
        e.register(WRAPPER_SLOT + 0x9c, |_, _| Ret::default());
        e.register(WRAPPER_SLOT + 0x94, |_, _| 0u32.into_ret());
        e.register(WRAPPER_SLOT + 0xc8, |_, _| Ret::default());
        e.register(THIS_SLOT + 0xc8, |_, _| Ret::default());
        e.register(THIS_SLOT + 0xcc, |e, _| {
            let source = e.mem.alloc(0x40);
            e.mem.set_u32(source + 0x2c, 0xabcd_1234);
            source.into_ret()
        });
        e.register(OSTREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(OSTREAM_WRITE_STRING, |_, _| Ret::default());
        e.register(OSTREAM_DESTRUCT, |_, _| Ret::default());
        e.register(REPORTER_SLOT + 0xc, |_, _| Ret::default());
        e.map(REPORTER_OBJECT, 0x10);
        e.mem.set_u32(REPORTER_OBJECT, REPORTER_VTABLE);
        e.mem.set_u32(ERROR_REPORTER, REPORTER_OBJECT);
        let mut entities = vec![];
        let mut wrappers = vec![];
        let mut havok_entities = vec![];
        for _ in 0..2 {
            let havok = e.mem.alloc(0x20);
            let wrapper = e.mem.alloc(0x20);
            e.mem.set_u32(wrapper, WRAPPER_VTABLE);
            e.mem.set_u32(wrapper + 8, havok);
            let entity = e.mem.alloc(0x40);
            e.mem.set_u32(entity + 0xc, wrapper);
            e.mem.set_u32(entity + 0x2c, 0x1111_2222);
            entities.push(entity);
            wrappers.push(wrapper);
            havok_entities.push(havok);
        }
        let object = e.mem.u32(this + 8);
        let data = e.mem.u32(object + 0x38);
        e.mem.set_u32(data, entities[0]);
        e.mem.set_u32(data + 4, entities[1]);
        e.mem.set_u32(object + 0x3c, 2);
        Entities {
            this,
            world,
            entities,
            wrappers,
            havok_entities,
        }
    }

    #[test]
    fn bhk_constraint_chain_process_entities_prepares_the_entities_and_reports_unlinked_ones() {
        let mut e = logged_engine();
        let scene = entities_scene(&mut e);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        // The world's +0x18 word is set: only the filter words change. The first entity's
        // Havok entity is in a world, the second is not.
        e.mem.set_u32(scene.world + 0x18, 1);
        e.mem.set_u32(scene.havok_entities[0] + 8, 0x5000);
        assert!(!e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 1u32])
            .bool());
        assert_eq!(e.mem.u32(scene.entities[0] + 0x2c), 0x1111_2222);
        assert_eq!(e.mem.u32(scene.entities[1] + 0x2c), 0xabcd_2222);
        assert_eq!(
            calls_to(&e, NO_EFFECT),
            vec![vec![scene.wrappers[0]]; 2]
                .into_iter()
                .chain(vec![vec![scene.wrappers[1]]; 2])
                .collect::<Vec<_>>()
        );
        assert_eq!(calls_to(&e, THIS_SLOT + 0xcc), vec![vec![scene.this]]);
        assert!(calls_to(&e, WRAPPER_SLOT + 0x9c).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);

        // The world's +0x18 word is 0: the wrappers run +0x9c; with +0x94 answering 0 and the
        // flag bits clear a message is reported and +0xc8 runs.
        clear_log(&mut e);
        e.mem.set_u32(scene.world + 0x18, 0);
        e.mem.set_u8(scene.wrappers[0] + 0x10, 0x10);
        e.call(0x00d5_36a0, &args![scene.this, scene.world, 1u32]);
        assert_eq!(
            calls_to(&e, WRAPPER_SLOT + 0x9c),
            vec![
                vec![scene.wrappers[0], scene.world],
                vec![scene.wrappers[1], scene.world]
            ]
        );
        // Only the second wrapper reports.
        let constructs = calls_to(&e, OSTREAM_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let stream = constructs[0][0];
        assert_eq!(constructs[0][1], stream + 0xc);
        assert_eq!(constructs[0][2..], [0x200, 1]);
        assert_eq!(
            calls_to(&e, OSTREAM_WRITE_STRING),
            vec![vec![stream, REPORT_TEXT]]
        );
        assert_eq!(
            calls_to(&e, REPORTER_SLOT + 0xc),
            vec![vec![
                REPORTER_OBJECT,
                1,
                0x234f_24fc,
                stream + 0xc,
                REPORT_FILE,
                0x168
            ]]
        );
        assert_eq!(calls_to(&e, OSTREAM_DESTRUCT), vec![vec![stream]]);
        assert_eq!(
            calls_to(&e, WRAPPER_SLOT + 0xc8),
            vec![vec![scene.wrappers[1], scene.world]]
        );
        let order = vec![
            OSTREAM_CONSTRUCT,
            OSTREAM_WRITE_STRING,
            REPORTER_SLOT + 0xc,
            OSTREAM_DESTRUCT,
            WRAPPER_SLOT + 0xc8,
        ];
        assert_eq!(order_of(&e, &order), order);
        // A wrapper whose +0x94 answers non-zero reports nothing.
        clear_log(&mut e);
        e.register(WRAPPER_SLOT + 0x94, |_, _| 1u32.into_ret());
        e.call(0x00d5_36a0, &args![scene.this, scene.world, 1u32]);
        assert!(calls_to(&e, OSTREAM_CONSTRUCT).is_empty());
        e.register(WRAPPER_SLOT + 0x94, |_, _| 0u32.into_ret());

        // A missing wrapper sets the answer and runs the chain's +0xc8 with the +0x24 word of
        // the world's +0x94 object.
        clear_log(&mut e);
        e.register(WORLD_SLOT + 0x94, |e, _| {
            let object = e.mem.alloc(0x40);
            e.mem.set_u32(object + 0x24, 0x66);
            object.into_ret()
        });
        e.mem.set_u32(scene.entities[1] + 0xc, 0);
        assert!(e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 1u32])
            .bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xc8), vec![vec![scene.this, 0x66]]);
        // Without that object the word is 0.
        clear_log(&mut e);
        e.register(WORLD_SLOT + 0x94, |_, _| 0u32.into_ret());
        assert!(e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 1u32])
            .bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xc8), vec![vec![scene.this, 0]]);
        // A null entity counts as missing as well.
        let object = e.mem.u32(scene.this + 8);
        let data = e.mem.u32(object + 0x38);
        e.mem.set_u32(data, 0);
        e.mem.set_u32(scene.entities[1] + 0xc, scene.wrappers[1]);
        clear_log(&mut e);
        assert!(e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 1u32])
            .bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xc8).len(), 1);
        // No entities: false.
        e.mem.set_u32(object + 0x3c, 0);
        assert!(!e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 1u32])
            .bool());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_constraint_chain_process_entities_removes_the_entity_the_world_names() {
        let mut e = logged_engine();
        let scene = entities_scene(&mut e);
        // The world's +0x94 object has the second entity as its +0x24 word.
        e.map(0x7100_2000, 0x40);
        e.mem.set_u32(0x7100_2000 + 0x24, scene.entities[1]);
        e.register(WORLD_SLOT + 0x94, |_, _| 0x7100_2000u32.into_ret());
        assert!(!e
            .call(0x00d5_36a0, &args![scene.this, scene.world, 0u32])
            .bool());
        assert_eq!(calls_to(&e, THIS_SLOT + 0xc8), vec![vec![scene.this, 0]]);
        assert_eq!(calls_to(&e, WORLD_SLOT + 0x94).len(), 2);
        // No object: the +0x24 word is 0, no entity matches.
        clear_log(&mut e);
        e.register(WORLD_SLOT + 0x94, |_, _| 0u32.into_ret());
        e.call(0x00d5_36a0, &args![scene.this, scene.world, 0u32]);
        assert!(calls_to(&e, THIS_SLOT + 0xc8).is_empty());
        // Nothing of the adding branch ran.
        assert!(calls_to(&e, NO_EFFECT).is_empty());
    }

    #[test]
    fn bhk_constraint_chain_copy_cinfo_copies_the_entities_into_the_cinfo_array() {
        let mut e = logged_engine();
        install_router(&mut e);
        e.register(CONSTRAINT_COPY_CINFO, |_, _| Ret::default());
        let (this, _) = chain_with_entities(&mut e, &[0x10, 0x20, 0x30]);
        let cinfo = e.mem.alloc(0x30);
        e.call(0x00d5_3900, &args![this, cinfo]);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO), vec![vec![this, cinfo]]);
        // Capacity 0: reserved for the larger of twice 0 and 3.
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][1..], [cinfo + 0x14, 3, 4]);
        assert_eq!(e.mem.u32(cinfo + 0x18), 3);
        let data = e.mem.u32(cinfo + 0x14);
        let copied: Vec<u32> = (0..3).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(copied, vec![0x10, 0x20, 0x30]);
        // Enough capacity: no reserve; twice the capacity when larger than the count.
        clear_log(&mut e);
        e.mem.set_u32(cinfo + 0x1c, 0x4000_0008);
        e.call(0x00d5_3900, &args![this, cinfo]);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        e.mem.set_u32(cinfo + 0x1c, 2);
        e.call(0x00d5_3900, &args![this, cinfo]);
        assert_eq!(calls_to(&e, ARRAY_RESERVE)[0][1..], [cinfo + 0x14, 4, 4]);
        // Without a Havok constraint only the base copy runs.
        clear_log(&mut e);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00d5_3900, &args![this, cinfo]);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO).len(), 1);
        assert!(calls_to(&e, MEMORY_ROUTER).is_empty());
    }

    #[test]
    fn bhk_constraint_chain_link_object_fills_the_cinfo_from_the_linked_objects() {
        let mut e = logged_engine();
        install_router(&mut e);
        put_table(&mut e, THIS_VTABLE, THIS_SLOT, &[0xb4]);
        e.register(GET_NUMBER_OF_LINK_IDS, |_, _| 2u32.into_ret());
        e.register(GET_OBJECT_FROM_LINK_ID, |e, _| {
            let object = e.mem.alloc(0x10);
            let next = e.mem.u32(0x7100_3000);
            e.mem.set_u32(0x7100_3000, next + 1);
            e.mem.set_u32(object + 8, 0x100 + next);
            object.into_ret()
        });
        let cinfo = e.mem.alloc(0x30);
        e.map(0x7100_3000, 0x10);
        e.register(THIS_SLOT + 0xb4, |e, a| {
            // Answers the hkSerializableCinfo base (+4); keeps the cell's content.
            let cell = e.mem.u32(a[1]);
            e.mem.set_u32(0x7100_3004, cell);
            let cinfo = e.mem.u32(0x7100_3008);
            (cinfo + 4).into_ret()
        });
        e.mem.set_u32(0x7100_3008, cinfo);
        e.register(CONSTRAINT_LINK_OBJECT, |_, _| Ret::default());
        let (this, _) = chain_with_entities(&mut e, &[]);
        e.mem.set_u32(this, THIS_VTABLE);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00d5_3970, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(0x7100_3004), 0x9000);
        assert_eq!(e.mem.u32(cinfo + 0x18), 2);
        let data = e.mem.u32(cinfo + 0x14);
        assert_eq!(e.mem.u32(data), 0x100);
        assert_eq!(e.mem.u32(data + 4), 0x101);
        assert_eq!(calls_to(&e, GET_NUMBER_OF_LINK_IDS), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID), vec![vec![0x9000]; 2]);
        assert_eq!(
            calls_to(&e, CONSTRAINT_LINK_OBJECT),
            vec![vec![this, 0x9000]]
        );
        assert_eq!(calls_to(&e, ARRAY_RESERVE)[0][1..], [cinfo + 0x14, 2, 4]);
        let order = vec![
            GET_NUMBER_OF_LINK_IDS,
            THIS_SLOT + 0xb4,
            MEMORY_ROUTER,
            ARRAY_RESERVE,
            GET_OBJECT_FROM_LINK_ID,
            GET_OBJECT_FROM_LINK_ID,
            CONSTRAINT_LINK_OBJECT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    /// The cloning map of the process: `(bucket index, key equality)`; a hit returns the item
    /// whose key equals the wrapper.
    fn install_map(e: &mut Engine) -> u32 {
        put_table(e, MAP_VTABLE, MAP_SLOT, &[4, 8]);
        // The bucket index is the low bit of the key; equality compares the words.
        e.register(MAP_SLOT + 4, |_, a| (a[1] & 1).into_ret());
        e.register(MAP_SLOT + 8, |_, a| u32::from(a[1] == a[2]).into_ret());
        let map = e.mem.alloc(0x20);
        e.mem.set_u32(map, MAP_VTABLE);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, buckets);
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(process, map);
        process
    }

    #[test]
    fn bhk_constraint_chain_copy_members_replaces_the_entities_with_their_clones() {
        let mut e = logged_engine();
        let allocator = install_router(&mut e);
        put_table(&mut e, THIS_VTABLE, THIS_SLOT, &[0xb4]);
        put_table(&mut e, WRAPPER_VTABLE, WRAPPER_SLOT, &[0x48]);
        e.register(CONSTRAINT_COPY_MEMBERS, |_, _| Ret::default());
        let process = install_map(&mut e);
        let map = e.mem.u32(process);
        let buckets = e.mem.u32(map + 8);
        // The cinfo with two entities whose wrappers have the (odd) addresses below.
        let cinfo = e.mem.alloc(0x30);
        e.register(THIS_SLOT + 0xb4, |e, _| {
            let cinfo = e.mem.u32(0x7100_4000);
            (cinfo + 4).into_ret()
        });
        e.map(0x7100_4000, 0x10);
        e.mem.set_u32(0x7100_4000, cinfo);
        let mut wrappers = vec![];
        let mut entities = vec![];
        for _ in 0..2 {
            let wrapper = e.mem.alloc(0x20);
            e.mem.set_u32(wrapper, WRAPPER_VTABLE);
            let entity = e.mem.alloc(0x20);
            e.mem.set_u32(entity + 0xc, wrapper);
            wrappers.push(wrapper);
            entities.push(entity);
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, entities[0]);
        e.mem.set_u32(array + 4, entities[1]);
        e.mem.set_u32(cinfo + 0x14, array);
        e.mem.set_u32(cinfo + 0x18, 2);
        e.mem.set_u32(cinfo + 0x1c, 2);
        // The first wrapper has a clone in the map (bucket by the low bit of its address);
        // the second clones itself.
        let first_clone = e.mem.alloc(0x20);
        let first_hk = e.mem.alloc(0x20);
        e.mem.set_u32(first_clone + 8, first_hk);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 4, wrappers[0]);
        e.mem.set_u32(item + 8, first_clone);
        let bucket = wrappers[0] & 1;
        e.mem.set_u32(buckets + 4 * bucket, item);
        let second_clone = e.mem.alloc(0x20);
        let second_hk = e.mem.alloc(0x20);
        e.mem.set_u32(second_clone + 8, second_hk);
        e.mem.set_u32(0x7100_4004, second_clone);
        e.register(WRAPPER_SLOT + 0x48, |e, _| {
            e.mem.u32(0x7100_4004).into_ret()
        });
        let (this, _) = chain_with_entities(&mut e, &[]);
        e.mem.set_u32(this, THIS_VTABLE);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00d5_3a80, &args![this, 0x8000u32, process]);
        assert_eq!(calls_to(&e, THIS_SLOT + 0xb4).len(), 1);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, 0x8000, process]]
        );
        // The first clone came from the map; only the second wrapper cloned itself.
        assert_eq!(
            calls_to(&e, WRAPPER_SLOT + 0x48),
            vec![vec![wrappers[1], process]]
        );
        let data = e.mem.u32(cinfo + 0x14);
        assert_eq!(e.mem.u32(cinfo + 0x18), 2);
        assert_eq!(e.mem.u32(data), first_hk);
        assert_eq!(e.mem.u32(data + 4), second_hk);
        // The temporary array was reserved for 2 elements: freed with its capacity in bytes.
        let frees = calls_to(&e, ALLOCATOR_SLOT + 0x10);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][0], allocator);
        assert_eq!(frees[0][2], 8);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // The order: the copy of the members comes after the clones, the free last.
        let order = vec![
            THIS_SLOT + 0xb4,
            MAP_SLOT + 4,
            MAP_SLOT + 4,
            WRAPPER_SLOT + 0x48,
            CONSTRAINT_COPY_MEMBERS,
            ALLOCATOR_SLOT + 0x10,
        ];
        assert_eq!(order_of(&e, &order), order);
    }

    #[test]
    fn bhk_constraint_chain_copy_members_with_no_entities_frees_nothing() {
        let mut e = logged_engine();
        install_router(&mut e);
        put_table(&mut e, THIS_VTABLE, THIS_SLOT, &[0xb4]);
        e.register(CONSTRAINT_COPY_MEMBERS, |_, _| Ret::default());
        let process = install_map(&mut e);
        let cinfo = e.mem.alloc(0x30);
        e.register(THIS_SLOT + 0xb4, |e, _| {
            let cinfo = e.mem.u32(0x7100_5000);
            (cinfo + 4).into_ret()
        });
        e.map(0x7100_5000, 0x10);
        e.mem.set_u32(0x7100_5000, cinfo);
        let (this, _) = chain_with_entities(&mut e, &[]);
        e.mem.set_u32(this, THIS_VTABLE);
        e.call(0x00d5_3a80, &args![this, 0x8000u32, process]);
        // Nothing to reserve (capacity 0 against 0) and nothing to free: the temporary array
        // keeps its "do not deallocate" flag.
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        assert!(calls_to(&e, ALLOCATOR_SLOT + 0x10).is_empty());
        assert_eq!(e.mem.u32(cinfo + 0x18), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_MEMBERS).len(), 1);
    }
}
