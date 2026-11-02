//! `Havok SDK/hkpconstraintinstance.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cf4160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_40b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cf4160(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_40b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::addReference` / `removeReference` (`00c90510`, `00c905b0`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00cf3040 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two entity pointers at `this + 0x14` and `this + 0x18`: when it equals the
/// word at `+0x24` of `world` (the world's fixed body; the only caller is
/// `hkpWorld::removeConstraint`, `00c97ef9`) the reference is dropped (`00c905b0`) and the
/// slot cleared. The class is unconfirmed.
pub fn fn_00cf3040(e: &mut Engine, this: Ptr, world: Ptr) {
    for slot in 0..2 {
        let at = this.addr() + 0x14 + 4 * slot;
        let entity = e.mem.u32(at);
        if entity == e.mem.u32(world.addr() + 0x24) {
            e.call(REMOVE_REFERENCE, &args![entity]);
            e.mem.set_u32(at, 0);
        }
    }
}

// Translated from 00cf3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two entity pointers at `this + 0x14` and `this + 0x18` that is null: the
/// other one (`[+0x18] ^ [+0x14]`, which is the other entity while this slot is null) is
/// taken, and when it exists and has a world (its word +8) that world's word at +0x24 (the
/// fixed body) becomes this slot and gets a reference (`00c90510`). The only caller is
/// `hkpWorld::addConstraint` (`00c97db3`). The class is unconfirmed.
pub fn fn_00cf3e60(e: &mut Engine, this: Ptr) {
    for slot in 0..2 {
        let at = this.addr() + 0x14 + 4 * slot;
        if e.mem.u32(at) == 0 {
            let other = e.mem.u32(this.addr() + 0x18) ^ e.mem.u32(this.addr() + 0x14);
            if other != 0 {
                let world = e.mem.u32(other + 8);
                if world != 0 {
                    let fixed = e.mem.u32(world + 0x24);
                    e.mem.set_u32(at, fixed);
                    e.call(ADD_REFERENCE, &args![fixed]);
                }
            }
        }
    }
}

/// `hkpWorldConstraintUtil::findModifier` / `addModifier` (`00d277c0`, `00d27960`, cdecl).
const FIND_MODIFIER: u32 = 0x00d2_77c0;
const ADD_MODIFIER: u32 = 0x00d2_7960;
/// `hkpWorldOperationUtil::removeConstraintImmediately` (`00cb3dd0`, cdecl).
const REMOVE_CONSTRAINT_IMMEDIATELY: u32 = 0x00cb_3dd0;
/// `hkVector4::setTransformedPos` / `setRotatedDir` (`00c7f640`, `00c7f700`; `this` = result,
/// then the transform and the vector).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// Reads one of the two bodies' transform inputs (`00d84ca0`, `00d84d30`; one word, returns a
/// pointer to a vector).
const PIVOT_A: u32 = 0x00d8_4ca0;
const PIVOT_B: u32 = 0x00d8_4d30;
/// `hkStringPtr` constructor, destructor and assignment (`00cb0a60`, `00cb0c00`, `00cb0c10`).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// The two calls that bracket the constructor's reference counting (`00c90490`, `00c904e0`;
/// no arguments; their use of a thread-local counter at `0x012681d4` is all that is known).
const ENTER_REFERENCE_SECTION: u32 = 0x00c9_0490;
const LEAVE_REFERENCE_SECTION: u32 = 0x00c9_04e0;
/// `hkpConstraintCallbackUtil::fireConstraintDeleted` (`00d85ef0`, cdecl).
const FIRE_CONSTRAINT_DELETED: u32 = 0x00d8_5ef0;
/// `hkpConstraintDataCloningUtil` clone functions (`00d85700`, `00d858f0`, cdecl, one word).
const CLONE_DATA: u32 = 0x00d8_5700;
const CLONE_IF_CAN_HAVE_MOTORS: u32 = 0x00d8_58f0;
/// The wrapped data of a malleable constraint (`00cdf6c0`, `this`).
const GET_WRAPPED_CONSTRAINT_DATA: u32 = 0x00cd_f6c0;

/// The memory router's allocator (`[00c85750() + 0x10]`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00cf2f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::entityAddedCallback` (map name): does nothing (`RET 4`; the one
/// word is the entity).
pub fn entity_added_callback(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00cf2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::entityDeletedCallback` (map name): does nothing (`RET 4`; the one
/// word is the entity).
pub fn entity_deleted_callback(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00cf2fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::isConstrainedToWorld` (map name): writes 1 to the byte at `out`
/// when the second entity (`this + 0x18`) is missing, or the first entity's world
/// (`[[this + 0x14] + 8]`) exists and its fixed body (`+0x24`) is that second entity; else 0.
/// Returns `out`.
pub fn is_constrained_to_world(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let second = e.mem.u32(this.addr() + 0x18);
    let constrained = if second == 0 {
        true
    } else {
        let first = e.mem.u32(this.addr() + 0x14);
        let world = e.mem.u32(first + 8);
        world != 0 && second == e.mem.u32(world + 0x24)
    };
    e.mem.set_u8(out.addr(), u8::from(constrained));
    out
}

// Translated from 00cf3070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `this + 0x1c` (the priority) and, when `this + 0x30` points
/// at a record, in the byte at `+0x13` of that record. The class is unconfirmed.
pub fn set_priority(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1c, value);
    let record = e.mem.u32(this.addr() + 0x30);
    if record != 0 {
        e.mem.set_u8(record + 0x13, value);
    }
}

// Translated from 00cf3090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::entityRemovedCallback` (map name): when the constraint has an owner
/// (`this + 8`), `hkpWorldOperationUtil::removeConstraintImmediately` (`00cb3dd0`) runs with the
/// entity's world (`[entity + 8]`), `this` and 1.
pub fn entity_removed_callback(e: &mut Engine, this: Ptr, entity: Ptr) {
    if e.mem.u32(this.addr() + 8) != 0 {
        let world = e.mem.u32(entity.addr() + 8);
        e.call(REMOVE_CONSTRAINT_IMMEDIATELY, &args![world, this, 1u32]);
    }
}

/// Copies the four words at `from` to `to` (the `MOVAPS` copies; the bits are not looked at).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00cf3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the two vectors `00d84ca0` and `00d84d30` give for `[this + 0xc]` (the constraint
/// data) and transforms the first by the transform at `+0x18` of the first entity
/// (`[this + 0x14]`) into `first_out`, the second by that of the second entity (`[this + 0x18]`)
/// into `second_out` (`00c7f640`). The class is unconfirmed.
pub fn transform_pivots(e: &mut Engine, this: Ptr, first_out: Ptr, second_out: Ptr) {
    let data = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0x20, |e, vectors| {
        let first = e.call(PIVOT_A, &args![data]).u32();
        copy_vector(e, first, vectors.addr());
        let second = e.call(PIVOT_B, &args![data]).u32();
        copy_vector(e, second, vectors.addr() + 0x10);
        let first_entity = e.mem.u32(this.addr() + 0x14);
        let second_entity = e.mem.u32(this.addr() + 0x18);
        let second_transform = e.mem.u32(second_entity + 0x18);
        let first_transform = e.mem.u32(first_entity + 0x18);
        e.call(
            SET_TRANSFORMED_POS,
            &args![first_out, first_transform, vectors],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![second_out, second_transform, vectors.addr() + 0x10],
        );
    });
}

// Translated from 00cf3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vectors `first` and `second` at `+0x20` and `+0x30` of the modifier record of
/// type `0x18` that `00d277c0` finds for `this`. Without one a new 0x40-byte record is taken
/// from the allocator (slot 4; `u16 +0` = `0x18`, `u16 +0x10` = `0x40`), filled the same way
/// and added with `00d27960(this, [this + 8], record)`. The code does not check the
/// allocation for null before storing into it. The class is unconfirmed.
pub fn set_modifier_vectors(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let found = e.call(FIND_MODIFIER, &args![this, 0x18u32]).u32();
    if found != 0 {
        copy_vector(e, first.addr(), found + 0x20);
        copy_vector(e, second.addr(), found + 0x30);
        return;
    }
    let allocator = memory_allocator(e);
    let record = e.vcall(allocator, 4, &args![0x40u32]).u32();
    if record != 0 {
        e.mem.set_u16(record, 0x18);
        e.mem.set_u16(record + 0x10, 0x40);
    }
    copy_vector(e, first.addr(), record + 0x20);
    copy_vector(e, second.addr(), record + 0x30);
    let owner = e.mem.u32(this.addr() + 8);
    e.call(ADD_MODIFIER, &args![this, owner, record]);
}

// Translated from 00cf3dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xcc` of the first entity (`[this + 0x14]`), or of the second one when the
/// byte at `+0xe8` of the first is 5. The class is unconfirmed.
pub fn get_entity_value(e: &mut Engine, this: Ptr) -> u32 {
    let mut entity = e.mem.u32(this.addr() + 0x14);
    if e.mem.u8(entity + 0xe8) == 5 {
        entity = e.mem.u32(this.addr() + 0x18);
    }
    e.mem.u32(entity + 0xcc)
}

// Translated from 00cf3ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::hkpConstraintInstance(entityA, entityB, data, priority)`
/// (Xbox PDB): sets the vtable (`0x010cc040`), clears the fields, initialises the name
/// (`hkStringPtr` constructor `00cb0a60` at `+0x28`) and takes a reference (`00c90510`) on
/// each entity (the second only when it is not null) and on the data, between a pair of calls
/// to `00c90490` and `00c904e0`. Returns `this`.
pub fn construct_instance(
    e: &mut Engine,
    this: Ptr,
    entity_a: u32,
    entity_b: u32,
    data: u32,
    priority: u8,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at + 0xc, data);
    e.mem.set_u32(at, 0x010c_c040);
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u8(at + 0x1c, priority);
    e.mem.set_u8(at + 0x1d, 1);
    e.mem.set_u8(at + 0x1e, 0);
    e.mem.set_u16(at + 0x26, 0x8000);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u16(at + 0x24, 0);
    e.call(STRING_PTR_CONSTRUCT, &args![at + 0x28]);
    e.mem.set_u32(at + 0x2c, 0);
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x34, 0xffff_fff0);
    e.mem.set_u32(at + 0x14, entity_a);
    e.mem.set_u32(at + 0x18, entity_b);
    e.mem.set_u32(at + 0x10, 0);
    e.call(ENTER_REFERENCE_SECTION, &args![]);
    e.call(ADD_REFERENCE, &args![entity_a]);
    if entity_b != 0 {
        e.call(ADD_REFERENCE, &args![entity_b]);
    }
    e.call(ADD_REFERENCE, &args![data]);
    e.call(LEAVE_REFERENCE_SECTION, &args![]);
    this
}

// Translated from 00cf3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor without entities or data (Xbox PDB `hkpConstraintInstance(priority)`):
/// the same fields as `00cf3ea0` but `+0xc`, `+0x14`, `+0x18` are left alone and nothing is
/// referenced. Returns `this`.
pub fn construct_empty_instance(e: &mut Engine, this: Ptr, priority: u8) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, 0x010c_c040);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0x10, 0);
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u8(at + 0x1c, priority);
    e.mem.set_u8(at + 0x1d, 1);
    e.mem.set_u8(at + 0x1e, 0);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u16(at + 0x24, 0);
    e.mem.set_u16(at + 0x26, 0x8000);
    e.call(STRING_PTR_CONSTRUCT, &args![at + 0x28]);
    e.mem.set_u32(at + 0x2c, 0);
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x34, 0xffff_fff0);
    this
}

// Translated from 00cf3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::clone(entityA, entityB, mode)` (map name): a new 0x38-byte instance
/// (allocator slot 4, `u16 +4` = `0x38`) built with `00cf3ea0` from the given entities, and its
/// name copied from `this + 0x28` (with bit 0 cleared) by `00cb0c10`. The data depends on
/// `mode`: 0 clones it through `00d85700` when the constraint is
/// tied to the world (the new instance then has no second entity) and shares it otherwise;
/// 1 clones it through `00d858f0` or, when that gives null, shares it with a new reference; any
/// other mode shares it. A cloned data loses the reference its creation gave it once the
/// instance holds one. Returns the new instance.
pub fn clone_instance(e: &mut Engine, this: Ptr, entity_a: u32, entity_b: u32, mode: u32) -> Ptr {
    let data = e.mem.u32(this.addr() + 0xc);
    let priority = e.mem.u8(this.addr() + 0x1c);
    let created = if mode == 0 {
        let constrained = e.with_stack(4, |e, out| {
            is_constrained_to_world(e, this, out);
            e.mem.u8(out.addr()) != 0
        });
        if constrained {
            let cloned = e.call(CLONE_DATA, &args![data]).u32();
            let block = allocate_instance(e);
            let created = construct_instance(e, block, entity_a, 0, cloned, priority);
            e.call(REMOVE_REFERENCE, &args![cloned]);
            created
        } else {
            let block = allocate_instance(e);
            construct_instance(e, block, entity_a, entity_b, data, priority)
        }
    } else if mode == 1 {
        let mut cloned = e.call(CLONE_IF_CAN_HAVE_MOTORS, &args![data]).u32();
        if cloned == 0 {
            cloned = data;
            e.call(ADD_REFERENCE, &args![cloned]);
        }
        let block = allocate_instance(e);
        let created = construct_instance(e, block, entity_a, entity_b, cloned, priority);
        e.call(REMOVE_REFERENCE, &args![cloned]);
        created
    } else {
        let block = allocate_instance(e);
        construct_instance(e, block, entity_a, entity_b, data, priority)
    };
    let name = e.mem.u32(this.addr() + 0x28) & 0xffff_fffe;
    e.call(STRING_PTR_ASSIGN, &args![created.addr() + 0x28, name]);
    created
}

/// A new 0x38-byte block from the allocator (slot 4) with its size word (`u16 +4`) set.
fn allocate_instance(e: &mut Engine) -> Ptr {
    let allocator = memory_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x38u32]).u32();
    e.mem.set_u16(block + 4, 0x38);
    Ptr::new(block)
}

// Translated from 00cf40b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::~hkpConstraintInstance` (map name). Sets the vtable
/// (`0x010cc040`), fires the constraint-deleted callbacks (`00d85ef0`), drops the references
/// (`00c905b0`) to the two entities (when set), frees the chain of runtime blocks starting at
/// `this + 0x10` through the allocator (slot 8, each block's size is its `u16` at `+0x10`, the
/// next one is its word at `+0x14`; the walk goes on while the next block's type word is
/// at least 0x17), drops the data reference (`+0xc`), runs the name destructor (`00cb0c00`
/// at `+0x28`), frees the `u16 +0x26 & 0x3fff`-word array at `+0x20` unless bit 15 of
/// that field marks it as not owned, and finally sets the base vtable (`0x0102e388`).
pub fn destruct_instance(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, 0x010c_c040);
    e.call(FIRE_CONSTRAINT_DELETED, &args![this]);
    for slot in [0x14, 0x18] {
        let entity = e.mem.u32(at + slot);
        if entity != 0 {
            e.call(REMOVE_REFERENCE, &args![entity]);
        }
    }
    let mut block = e.mem.u32(at + 0x10);
    if block != 0 && e.mem.u16(block) >= 0x17 {
        loop {
            let current = block;
            block = e.mem.u32(current + 0x14);
            let size = u32::from(e.mem.u16(current + 0x10));
            let allocator = memory_allocator(e);
            e.vcall(allocator, 8, &args![current, size]);
            if e.mem.u16(block) < 0x17 {
                break;
            }
        }
    }
    let data = e.mem.u32(at + 0xc);
    if data != 0 {
        e.call(REMOVE_REFERENCE, &args![data]);
    }
    e.call(STRING_PTR_DESTRUCT, &args![at + 0x28]);
    let capacity = u32::from(e.mem.u16(at + 0x26));
    if capacity & 0x8000 == 0 {
        let array = e.mem.u32(at + 0x20);
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![array, (capacity & 0x3fff) * 4]);
    }
    e.mem.set_u32(at, 0x0102_e388);
}

// Translated from 00cf30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintInstance::transform` (map name): moves the constraint's data into a new
/// body space, `transform` being the vector transform. Only for a constraint that
/// `isConstrainedToWorld` accepts. The data's type is its virtual slot `+0x20`; types above 14
/// do nothing. Type 12 continues with the data wrapped at `+0x18`, type 13 with
/// `00cdf6c0`'s wrapped data, and the walk repeats with the type of the new data. The others
/// copy the data's vectors out, transform the position (`00c7f640`) and axes
/// (`00c7f700`) in the copies and call the data type's `setInBodySpace` with them:
/// 0 `00d541b0` (in place: the pivot at `+0x30` is transformed and both vectors passed as
/// they are), 1 `00cbe400`, 2 `00cb8da0`, 6 `00d4fba0`, 7 `00cded30` (its two axes at `+0x60`
/// and `+0x70` are rotated in the data itself), 9 `00d4e960`, 14 `00d849a0` (the third vector
/// comes from `+(([+0xa2] + 6) * 0x10)`); type 8 transforms the vector at `+0x30` and
/// writes it back with the vector at `+0x20` and the float at `+0x44` (copied through the x87
/// stack, which quiets a signalling NaN). The names of the types are Ghidra's labels of
/// the `setInBodySpace` callees and are unconfirmed beyond that.
pub fn transform_instance(e: &mut Engine, this: Ptr, transform: Ptr) {
    let constrained = e.with_stack(4, |e, out| {
        is_constrained_to_world(e, this, out);
        e.mem.u8(out.addr()) != 0
    });
    if !constrained {
        return;
    }
    let mut data = e.mem.u32(this.addr() + 0xc);
    let mut kind = e.vcall(data, 0x20, &args![]).u32();
    while kind <= 0xe {
        match kind {
            0 => {
                e.call(
                    SET_TRANSFORMED_POS,
                    &args![data + 0x30, transform, data + 0x30],
                );
                e.call(0x00d5_41b0, &args![data, data + 0x20, data + 0x30]);
                return;
            }
            1 => {
                set_body_space_hinge(e, data, transform);
                return;
            }
            2 => {
                set_body_space_axes(e, data, transform, 0x00cb_8da0);
                return;
            }
            6 => {
                set_body_space_axes(e, data, transform, 0x00d4_fba0);
                return;
            }
            7 => {
                set_body_space_ragdoll(e, data, transform);
                return;
            }
            8 => {
                set_body_space_spring(e, data, transform);
                return;
            }
            9 => {
                set_body_space_wheel(e, data, transform);
                return;
            }
            12 => data = e.mem.u32(data + 0x18),
            13 => data = e.call(GET_WRAPPED_CONSTRAINT_DATA, &args![data]).u32(),
            14 => {
                set_body_space_plane(e, data, transform);
                return;
            }
            _ => return,
        }
        kind = e.vcall(data, 0x20, &args![]).u32();
    }
}

/// Type 1: copies the vectors at `+0x50`, `+0x90`, `+0x20` and `+0x60`, transforms the
/// position (`+0x90`) and rotates the axis (`+0x60`), and calls `00cbe400` with the four
/// copies in that order.
fn set_body_space_hinge(e: &mut Engine, data: u32, transform: Ptr) {
    e.with_stack(0x40, |e, copies| {
        let [pivot, position, axis_a, axis_b] =
            [0, 0x10, 0x20, 0x30].map(|slot| copies.addr() + slot);
        copy_vector(e, data + 0x20, axis_a);
        copy_vector(e, data + 0x50, pivot);
        copy_vector(e, data + 0x60, axis_b);
        copy_vector(e, data + 0x90, position);
        e.call(SET_TRANSFORMED_POS, &args![position, transform, position]);
        e.call(SET_ROTATED_DIR, &args![axis_b, transform, axis_b]);
        e.call(0x00cb_e400, &args![data, pivot, position, axis_a, axis_b]);
    });
}

/// Types 2 and 6: copies the vectors at `+0x50`, `+0x90`, `+0x20`, `+0x60`, `+0x30` and
/// `+0x70`, transforms the position (`+0x90`), rotates the vectors from `+0x60` and `+0x70`
/// (in that order) and calls `setter` with the six copies in that order.
fn set_body_space_axes(e: &mut Engine, data: u32, transform: Ptr, setter: u32) {
    e.with_stack(0x60, |e, copies| {
        let slots: [u32; 6] = [0, 0x10, 0x20, 0x30, 0x40, 0x50].map(|slot| copies.addr() + slot);
        for (slot, offset) in slots.iter().zip([0x50, 0x90, 0x20, 0x60, 0x30, 0x70]) {
            copy_vector(e, data + offset, *slot);
        }
        e.call(SET_TRANSFORMED_POS, &args![slots[1], transform, slots[1]]);
        e.call(SET_ROTATED_DIR, &args![slots[3], transform, slots[3]]);
        e.call(SET_ROTATED_DIR, &args![slots[5], transform, slots[5]]);
        e.call(
            setter,
            &args![data, slots[0], slots[1], slots[2], slots[3], slots[4], slots[5]],
        );
    });
}

/// Type 7: copies the vectors at `+0x50` and `+0x90`, transforms the second, rotates the
/// vectors at `+0x70` and `+0x60` in the data itself and calls `00cded30` with
/// `(copy, copy, data + 0x30, data + 0x70, data + 0x20, data + 0x60)`.
fn set_body_space_ragdoll(e: &mut Engine, data: u32, transform: Ptr) {
    e.with_stack(0x20, |e, copies| {
        let first = copies.addr();
        let position = copies.addr() + 0x10;
        copy_vector(e, data + 0x50, first);
        copy_vector(e, data + 0x90, position);
        e.call(SET_TRANSFORMED_POS, &args![position, transform, position]);
        e.call(SET_ROTATED_DIR, &args![data + 0x70, transform, data + 0x70]);
        e.call(SET_ROTATED_DIR, &args![data + 0x60, transform, data + 0x60]);
        e.call(
            0x00cd_ed30,
            &args![
                data,
                first,
                position,
                data + 0x30,
                data + 0x70,
                data + 0x20,
                data + 0x60
            ],
        );
    });
}

/// Type 8: transforms a copy of the vector at `+0x30` and writes the copy of the vector at
/// `+0x20`, the transformed one and the float at `+0x44` (through the x87 stack) back.
fn set_body_space_spring(e: &mut Engine, data: u32, transform: Ptr) {
    e.with_stack(0x20, |e, copies| {
        let first = copies.addr();
        let second = copies.addr() + 0x10;
        copy_vector(e, data + 0x20, first);
        let float = x87_quiet_bits(e.mem.u32(data + 0x44));
        copy_vector(e, data + 0x30, second);
        e.call(SET_TRANSFORMED_POS, &args![second, transform, second]);
        e.mem.set_u32(data + 0x44, x87_quiet_bits(float));
        copy_vector(e, first, data + 0x20);
        copy_vector(e, second, data + 0x30);
    });
}

/// Type 9: copies the vectors at `+0x50`, `+0x90`, `+0xd0`, `+0x100`, `+0x60` and `+0x110`,
/// transforms the position, rotates those from `+0x60`, `+0x100` and `+0x110` (in that
/// order) and calls `00d4e960` with the six copies in that order.
fn set_body_space_wheel(e: &mut Engine, data: u32, transform: Ptr) {
    e.with_stack(0x60, |e, copies| {
        let slots: [u32; 6] = [0, 0x10, 0x20, 0x30, 0x40, 0x50].map(|slot| copies.addr() + slot);
        for (slot, offset) in slots.iter().zip([0x50, 0x90, 0xd0, 0x100, 0x60, 0x110]) {
            copy_vector(e, data + offset, *slot);
        }
        e.call(SET_TRANSFORMED_POS, &args![slots[1], transform, slots[1]]);
        e.call(SET_ROTATED_DIR, &args![slots[4], transform, slots[4]]);
        e.call(SET_ROTATED_DIR, &args![slots[3], transform, slots[3]]);
        e.call(SET_ROTATED_DIR, &args![slots[5], transform, slots[5]]);
        e.call(
            0x00d4_e960,
            &args![data, slots[0], slots[1], slots[2], slots[3], slots[4], slots[5]],
        );
    });
}

/// Type 14: copies the vectors at `+0x50`, `+0x90` and the one at
/// `data + (byte [data + 0xa2] + 6) * 0x10`, transforms the second, rotates the third and
/// calls `00d849a0` with the three copies.
fn set_body_space_plane(e: &mut Engine, data: u32, transform: Ptr) {
    e.with_stack(0x30, |e, copies| {
        let [first, position, normal] = [0, 0x10, 0x20].map(|slot| copies.addr() + slot);
        let normal_at = data + (u32::from(e.mem.u8(data + 0xa2)) + 6) * 0x10;
        copy_vector(e, data + 0x50, first);
        copy_vector(e, data + 0x90, position);
        copy_vector(e, normal_at, normal);
        e.call(SET_TRANSFORMED_POS, &args![position, transform, position]);
        e.call(SET_ROTATED_DIR, &args![normal, transform, normal]);
        e.call(0x00d8_49a0, &args![data, first, position, normal]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_2f90, entity_added_callback(Ptr, u32)),
        entry!(0x00cf_2fa0, entity_deleted_callback(Ptr, u32)),
        entry!(0x00cf_2fb0, is_constrained_to_world(Ptr, Ptr) -> Ptr),
        entry!(0x00cf_3040, fn_00cf3040(Ptr, Ptr)),
        entry!(0x00cf_3070, set_priority(Ptr, u8)),
        entry!(0x00cf_3090, entity_removed_callback(Ptr, Ptr)),
        entry!(0x00cf_30f0, transform_instance(Ptr, Ptr)),
        entry!(0x00cf_3c90, transform_pivots(Ptr, Ptr, Ptr)),
        entry!(0x00cf_3d00, set_modifier_vectors(Ptr, Ptr, Ptr)),
        entry!(0x00cf_3dd0, get_entity_value(Ptr) -> u32),
        entry!(0x00cf_3e60, fn_00cf3e60(Ptr)),
        entry!(0x00cf_3ea0, construct_instance(Ptr, u32, u32, u32, u8) -> Ptr),
        entry!(0x00cf_3f40, construct_empty_instance(Ptr, u8) -> Ptr),
        entry!(0x00cf_3fa0, clone_instance(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00cf_40b0, destruct_instance(Ptr)),
        entry!(0x00cf_4160, deleting_destructor_00cf4160(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00cf4160_frees_only_when_bit_zero_is_set() {
        check(0x00cf_4160, 0x00cf_40b0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn rig() -> (Engine, Rc<RefCell<Vec<(u32, u32)>>>) {
        let mut e = test_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        (e, log)
    }

    #[test]
    fn detaching_clears_the_slots_that_hold_the_fixed_body() {
        let (mut e, log) = rig();
        let world = e.mem.alloc(0x40);
        let fixed = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x24, fixed);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, fixed);
        e.mem.set_u32(this + 0x18, other);
        e.call(0x00cf_3040, &args![this, world]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), other);
        assert_eq!(*log.borrow(), vec![(REMOVE_REFERENCE, fixed)]);
        // Both slots can hold it.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x14, fixed);
        e.mem.set_u32(this + 0x18, fixed);
        e.call(0x00cf_3040, &args![this, world]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn attaching_fills_a_missing_entity_with_the_fixed_body_of_the_other_world() {
        let (mut e, log) = rig();
        let world = e.mem.alloc(0x40);
        let fixed = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x24, fixed);
        let entity = e.mem.alloc(0x10);
        e.mem.set_u32(entity + 8, world);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, entity);
        e.call(0x00cf_3e60, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), fixed);
        assert_eq!(e.mem.u32(this + 0x18), entity);
        assert_eq!(*log.borrow(), vec![(ADD_REFERENCE, fixed)]);
        // Both entities missing, or the other one without a world: nothing happens.
        log.borrow_mut().clear();
        let empty = e.mem.alloc(0x40);
        e.call(0x00cf_3e60, &args![empty]);
        let no_world = e.mem.alloc(0x40);
        let orphan = e.mem.alloc(0x10);
        e.mem.set_u32(no_world + 0x14, orphan);
        e.call(0x00cf_3e60, &args![no_world]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(no_world + 0x18), 0);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One logged call: the address and its argument words.
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles at `addresses` that log their arguments and return 0.
    fn log_calls(e: &mut Engine, addresses: &[u32]) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    /// A memory router whose allocator hands out `block` from slot 4 and takes frees in slot 8;
    /// both log their arguments under `4` and `8`.
    fn install_allocator(e: &mut Engine, block: u32) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push((4, a[1..].to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0008, move |_, a| {
            sink.borrow_mut().push((8, a[1..].to_vec()));
            Ret::default()
        });
        log
    }

    #[test]
    fn the_added_and_deleted_callbacks_do_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_2f90, &args![0x10u32, 0x20u32]).eax, 0);
        assert_eq!(e.call(0x00cf_2fa0, &args![0x10u32, 0x20u32]).eax, 0);
    }

    #[test]
    fn is_constrained_to_world_compares_with_the_fixed_body() {
        let mut e = Engine::new();
        let world = e.mem.alloc(0x40);
        let fixed = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x24, fixed);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, world);
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        // No second entity.
        assert_eq!(e.call(0x00cf_2fb0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        // A second entity that is the world's fixed body.
        e.mem.set_u32(this + 0x14, first);
        e.mem.set_u32(this + 0x18, fixed);
        e.call(0x00cf_2fb0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        // Another entity.
        e.mem.set_u32(this + 0x18, first);
        e.call(0x00cf_2fb0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        // The first entity has no world.
        e.mem.set_u32(first + 8, 0);
        e.mem.set_u32(this + 0x18, fixed);
        e.call(0x00cf_2fb0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn set_priority_follows_the_record_when_there_is_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.call(0x00cf_3070, &args![this, 3u32]);
        assert_eq!(e.mem.u8(this + 0x1c), 3);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x30, record);
        e.call(0x00cf_3070, &args![this, 0x1ffu32]);
        assert_eq!(e.mem.u8(this + 0x1c), 0xff);
        assert_eq!(e.mem.u8(record + 0x13), 0xff);
    }

    #[test]
    fn entity_removed_callback_removes_only_an_owned_constraint() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[REMOVE_CONSTRAINT_IMMEDIATELY]);
        let this = e.mem.alloc(0x40);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, 0x7777);
        e.call(0x00cf_3090, &args![this, entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + 8, 0x1234);
        e.call(0x00cf_3090, &args![this, entity]);
        assert_eq!(
            *log.borrow(),
            [(REMOVE_CONSTRAINT_IMMEDIATELY, vec![0x7777, this, 1])]
        );
    }

    #[test]
    fn transform_pivots_moves_each_vector_by_its_entity() {
        let mut e = Engine::new();
        let first_vector = e.mem.alloc(0x10);
        let second_vector = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(first_vector + 4 * word, 1 + word);
            e.mem.set_u32(second_vector + 4 * word, 5 + word);
        }
        e.register_double(PIVOT_A, move |_, _| Ret {
            eax: first_vector,
            ..Ret::default()
        });
        e.register_double(PIVOT_B, move |_, _| Ret {
            eax: second_vector,
            ..Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            sink.borrow_mut()
                .push((a[0], a[1], e.mem.u32(a[2]), e.mem.u32(a[2] + 0xc)));
            Ret::default()
        });
        let first_entity = e.mem.alloc(0x40);
        let second_entity = e.mem.alloc(0x40);
        e.mem.set_u32(first_entity + 0x18, 0x1111);
        e.mem.set_u32(second_entity + 0x18, 0x2222);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x4000);
        e.mem.set_u32(this + 0x14, first_entity);
        e.mem.set_u32(this + 0x18, second_entity);
        e.call(0x00cf_3c90, &args![this, 0xa000u32, 0xb000u32]);
        assert_eq!(
            *seen.borrow(),
            [(0xa000, 0x1111, 1, 4), (0xb000, 0x2222, 5, 8)]
        );
    }

    #[test]
    fn set_modifier_vectors_updates_a_found_record() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[ADD_MODIFIER]);
        let record = e.mem.alloc(0x60);
        let sink = Rc::clone(&log);
        e.register_double(FIND_MODIFIER, move |_, a| {
            sink.borrow_mut().push((FIND_MODIFIER, a.to_vec()));
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(first + 4 * word, 0x10 + word);
            e.mem.set_u32(second + 4 * word, 0x20 + word);
        }
        let this = e.mem.alloc(0x20);
        e.call(0x00cf_3d00, &args![this, first, second]);
        assert_eq!(*log.borrow(), [(FIND_MODIFIER, vec![this, 0x18])]);
        for word in 0..4 {
            assert_eq!(e.mem.u32(record + 0x20 + 4 * word), 0x10 + word);
            assert_eq!(e.mem.u32(record + 0x30 + 4 * word), 0x20 + word);
        }
    }

    #[test]
    fn set_modifier_vectors_adds_a_new_record_when_none_is_found() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[FIND_MODIFIER, ADD_MODIFIER]);
        let block = e.mem.alloc(0x40);
        let allocations = install_allocator(&mut e, block);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first, 0xaaaa);
        e.mem.set_u32(second + 0xc, 0xbbbb);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x5555);
        e.call(0x00cf_3d00, &args![this, first, second]);
        assert_eq!(*allocations.borrow(), [(4, vec![0x40])]);
        assert_eq!(e.mem.u16(block), 0x18);
        assert_eq!(e.mem.u16(block + 0x10), 0x40);
        assert_eq!(e.mem.u32(block + 0x20), 0xaaaa);
        assert_eq!(e.mem.u32(block + 0x3c), 0xbbbb);
        assert_eq!(
            *log.borrow(),
            [
                (FIND_MODIFIER, vec![this, 0x18]),
                (ADD_MODIFIER, vec![this, 0x5555, block])
            ]
        );
    }

    #[test]
    fn get_entity_value_picks_the_second_entity_for_type_five() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(first + 0xcc, 0x11);
        e.mem.set_u32(second + 0xcc, 0x22);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, first);
        e.mem.set_u32(this + 0x18, second);
        assert_eq!(e.call(0x00cf_3dd0, &args![this]).u32(), 0x11);
        e.mem.set_u8(first + 0xe8, 5);
        assert_eq!(e.call(0x00cf_3dd0, &args![this]).u32(), 0x22);
    }

    #[test]
    fn the_constructor_references_the_entities_and_the_data() {
        let mut e = Engine::new();
        let log = log_calls(
            &mut e,
            &[
                STRING_PTR_CONSTRUCT,
                ENTER_REFERENCE_SECTION,
                ADD_REFERENCE,
                LEAVE_REFERENCE_SECTION,
            ],
        );
        let this = e.mem.alloc(0x40);
        let got = e
            .call(
                0x00cf_3ea0,
                &args![this, 0xa0u32, 0xb0u32, 0xd0u32, 0x107u32],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_c040);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0xd0);
        assert_eq!(e.mem.u32(this + 0x14), 0xa0);
        assert_eq!(e.mem.u32(this + 0x18), 0xb0);
        assert_eq!(e.mem.u8(this + 0x1c), 7);
        assert_eq!(e.mem.u8(this + 0x1d), 1);
        assert_eq!(e.mem.u16(this + 0x26), 0x8000);
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_fff0);
        let expected: Vec<(u32, Vec<u32>)> = vec![
            (STRING_PTR_CONSTRUCT, vec![this + 0x28]),
            (ENTER_REFERENCE_SECTION, vec![]),
            (ADD_REFERENCE, vec![0xa0]),
            (ADD_REFERENCE, vec![0xb0]),
            (ADD_REFERENCE, vec![0xd0]),
            (LEAVE_REFERENCE_SECTION, vec![]),
        ];
        assert_eq!(*log.borrow(), expected);
        // Without a second entity only two references are taken.
        log.borrow_mut().clear();
        e.call(0x00cf_3ea0, &args![this, 0xa0u32, 0u32, 0xd0u32, 1u32]);
        let references = log
            .borrow()
            .iter()
            .filter(|(address, _)| *address == ADD_REFERENCE)
            .count();
        assert_eq!(references, 2);
    }

    #[test]
    fn the_empty_constructor_only_sets_fields() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[STRING_PTR_CONSTRUCT]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x99);
        e.mem.set_u32(this + 0x14, 0x98);
        let got = e.call(0x00cf_3f40, &args![this, 4u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_c040);
        assert_eq!(e.mem.u8(this + 0x1c), 4);
        assert_eq!(e.mem.u16(this + 0x26), 0x8000);
        assert_eq!(e.mem.u32(this + 0xc), 0x99);
        assert_eq!(e.mem.u32(this + 0x14), 0x98);
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_fff0);
        assert_eq!(*log.borrow(), [(STRING_PTR_CONSTRUCT, vec![this + 0x28])]);
    }

    /// An engine for `clone` whose doubles all log into one list (allocator calls as 4/8).
    fn clone_rig() -> (Engine, Calls, u32) {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        let log = install_allocator(&mut e, block);
        for address in [
            STRING_PTR_CONSTRUCT,
            ENTER_REFERENCE_SECTION,
            LEAVE_REFERENCE_SECTION,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            STRING_PTR_ASSIGN,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        for address in [CLONE_DATA, CLONE_IF_CAN_HAVE_MOTORS] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: 0x0cc0_0000 + address,
                    ..Ret::default()
                }
            });
        }
        (e, log, block)
    }

    /// The addresses that were called, in order, without the constructor's bookkeeping.
    fn steps(log: &Calls) -> Vec<(u32, Vec<u32>)> {
        log.borrow()
            .iter()
            .filter(|(address, _)| {
                ![
                    STRING_PTR_CONSTRUCT,
                    ENTER_REFERENCE_SECTION,
                    LEAVE_REFERENCE_SECTION,
                ]
                .contains(address)
            })
            .cloned()
            .collect()
    }

    fn instance_with_data(e: &mut Engine, second_entity: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        let first_entity = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x14, first_entity);
        e.mem.set_u32(this + 0x18, second_entity);
        e.mem.set_u32(this + 0xc, 0xda7a);
        e.mem.set_u8(this + 0x1c, 6);
        e.mem.set_u32(this + 0x28, 0x7001);
        this
    }

    #[test]
    fn clone_mode_zero_shares_the_data_of_a_free_constraint() {
        let (mut e, log, block) = clone_rig();
        // The first entity has a world whose fixed body is not the second entity.
        let world = e.mem.alloc(0x40);
        let this = instance_with_data(&mut e, 0x5ec0);
        let first_entity = e.mem.u32(this + 0x14);
        e.mem.set_u32(first_entity + 8, world);
        let created = e
            .call(0x00cf_3fa0, &args![this, 0xa1u32, 0xb1u32, 0u32])
            .u32();
        assert_eq!(created, block);
        assert_eq!(e.mem.u16(block + 4), 0x38);
        assert_eq!(e.mem.u32(block + 0xc), 0xda7a);
        assert_eq!(e.mem.u32(block + 0x14), 0xa1);
        assert_eq!(e.mem.u32(block + 0x18), 0xb1);
        assert_eq!(e.mem.u8(block + 0x1c), 6);
        assert_eq!(
            steps(&log),
            [
                (4, vec![0x38]),
                (ADD_REFERENCE, vec![0xa1]),
                (ADD_REFERENCE, vec![0xb1]),
                (ADD_REFERENCE, vec![0xda7a]),
                (STRING_PTR_ASSIGN, vec![block + 0x28, 0x7000]),
            ]
        );
    }

    #[test]
    fn clone_mode_zero_clones_the_data_of_a_world_constraint() {
        let (mut e, log, block) = clone_rig();
        let this = instance_with_data(&mut e, 0);
        let created = e
            .call(0x00cf_3fa0, &args![this, 0xa1u32, 0xb1u32, 0u32])
            .u32();
        assert_eq!(created, block);
        let cloned = 0x0cc0_0000 + CLONE_DATA;
        assert_eq!(e.mem.u32(block + 0x14), 0xa1);
        assert_eq!(e.mem.u32(block + 0x18), 0);
        assert_eq!(e.mem.u32(block + 0xc), cloned);
        assert_eq!(
            steps(&log),
            [
                (CLONE_DATA, vec![0xda7a]),
                (4, vec![0x38]),
                (ADD_REFERENCE, vec![0xa1]),
                (ADD_REFERENCE, vec![cloned]),
                (REMOVE_REFERENCE, vec![cloned]),
                (STRING_PTR_ASSIGN, vec![block + 0x28, 0x7000]),
            ]
        );
    }

    #[test]
    fn clone_mode_one_clones_or_shares_the_data() {
        let (mut e, log, block) = clone_rig();
        let this = instance_with_data(&mut e, 0x5ec0);
        e.call(0x00cf_3fa0, &args![this, 0xa1u32, 0xb1u32, 1u32]);
        let cloned = 0x0cc0_0000 + CLONE_IF_CAN_HAVE_MOTORS;
        assert_eq!(e.mem.u32(block + 0xc), cloned);
        assert_eq!(
            steps(&log),
            [
                (CLONE_IF_CAN_HAVE_MOTORS, vec![0xda7a]),
                (4, vec![0x38]),
                (ADD_REFERENCE, vec![0xa1]),
                (ADD_REFERENCE, vec![0xb1]),
                (ADD_REFERENCE, vec![cloned]),
                (REMOVE_REFERENCE, vec![cloned]),
                (STRING_PTR_ASSIGN, vec![block + 0x28, 0x7000]),
            ]
        );
        // The clone function answers null: the data is shared and gets a reference first.
        let (mut e, log, block) = clone_rig();
        e.register_double(CLONE_IF_CAN_HAVE_MOTORS, |_, _| Ret::default());
        let this = instance_with_data(&mut e, 0x5ec0);
        e.call(0x00cf_3fa0, &args![this, 0xa1u32, 0xb1u32, 1u32]);
        assert_eq!(e.mem.u32(block + 0xc), 0xda7a);
        assert_eq!(steps(&log).first(), Some(&(ADD_REFERENCE, vec![0xda7a])));
        assert_eq!(
            steps(&log)
                .iter()
                .filter(|(a, _)| *a == REMOVE_REFERENCE)
                .count(),
            1
        );
    }

    #[test]
    fn clone_with_another_mode_shares_the_data() {
        let (mut e, log, block) = clone_rig();
        let this = instance_with_data(&mut e, 0x5ec0);
        e.call(0x00cf_3fa0, &args![this, 0xa1u32, 0xb1u32, 2u32]);
        assert_eq!(e.mem.u32(block + 0xc), 0xda7a);
        assert_eq!(
            steps(&log),
            [
                (4, vec![0x38]),
                (ADD_REFERENCE, vec![0xa1]),
                (ADD_REFERENCE, vec![0xb1]),
                (ADD_REFERENCE, vec![0xda7a]),
                (STRING_PTR_ASSIGN, vec![block + 0x28, 0x7000]),
            ]
        );
    }

    #[test]
    fn the_destructor_releases_everything_the_instance_holds() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        let log = install_allocator(&mut e, block);
        for address in [
            FIRE_CONSTRAINT_DELETED,
            REMOVE_REFERENCE,
            STRING_PTR_DESTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // Two runtime blocks (type words 0x17 and 0x20) followed by one of type 0x10.
        let end = e.mem.alloc(0x20);
        e.mem.set_u16(end, 0x10);
        let second = e.mem.alloc(0x20);
        e.mem.set_u16(second, 0x20);
        e.mem.set_u16(second + 0x10, 0x30);
        e.mem.set_u32(second + 0x14, end);
        let first = e.mem.alloc(0x20);
        e.mem.set_u16(first, 0x17);
        e.mem.set_u16(first + 0x10, 0x28);
        e.mem.set_u32(first + 0x14, second);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0xe1);
        e.mem.set_u32(this + 0x18, 0xe2);
        e.mem.set_u32(this + 0x10, first);
        e.mem.set_u32(this + 0xc, 0xda7a);
        e.mem.set_u32(this + 0x20, 0xa77a);
        e.mem.set_u16(this + 0x26, 0x0005);
        e.call(0x00cf_40b0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(
            *log.borrow(),
            [
                (FIRE_CONSTRAINT_DELETED, vec![this]),
                (REMOVE_REFERENCE, vec![0xe1]),
                (REMOVE_REFERENCE, vec![0xe2]),
                (8, vec![first, 0x28]),
                (8, vec![second, 0x30]),
                (REMOVE_REFERENCE, vec![0xda7a]),
                (STRING_PTR_DESTRUCT, vec![this + 0x28]),
                (8, vec![0xa77a, 20]),
            ]
        );
    }

    #[test]
    fn the_destructor_skips_what_is_absent_or_not_owned() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        let log = install_allocator(&mut e, block);
        for address in [
            FIRE_CONSTRAINT_DELETED,
            REMOVE_REFERENCE,
            STRING_PTR_DESTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 0x26, 0x8005);
        e.call(0x00cf_40b0, &args![this]);
        assert_eq!(
            *log.borrow(),
            [
                (FIRE_CONSTRAINT_DELETED, vec![this]),
                (STRING_PTR_DESTRUCT, vec![this + 0x28]),
            ]
        );
        // A first runtime block whose type word is below 0x17 is left alone.
        let low = e.mem.alloc(0x20);
        e.mem.set_u16(low, 0x16);
        e.mem.set_u32(this + 0x10, low);
        log.borrow_mut().clear();
        e.call(0x00cf_40b0, &args![this]);
        assert_eq!(log.borrow().len(), 2);
    }

    const TRANSFORM: u32 = 0x0ace_0000;
    const MARK_POSITION: u32 = 0x8000_0000;
    const MARK_AXIS: u32 = 0x4000_0000;
    const NOT_IN_DATA: u32 = u32::MAX;
    const BALL_AND_SOCKET_SETTER: u32 = 0x00d5_41b0;
    const HINGE_SETTER: u32 = 0x00cb_e400;
    const LIMITED_HINGE_SETTER: u32 = 0x00cb_8da0;
    const PRISMATIC_SETTER: u32 = 0x00d4_fba0;
    const RAGDOLL_SETTER: u32 = 0x00cd_ed30;
    const WHEEL_SETTER: u32 = 0x00d4_e960;
    const PLANE_SETTER: u32 = 0x00d8_49a0;

    /// A data object of the given type whose word `n` holds `0x1000 + n`.
    fn data_object(e: &mut Engine, kind: u32, slot: u32) -> u32 {
        let block = e.mem.alloc(0x130);
        for word in 0..0x4c {
            e.mem.set_u32(block + 4 * word, 0x1000 + word);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, slot);
        e.mem.set_u32(block, vtable);
        e.register_double(slot, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        block
    }

    fn instance_over(e: &mut Engine, data: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, data);
        this
    }

    /// Runs `transform` on `this` with doubles for every callee; returns the calls as
    /// `[address, ...]`: the vector calls with the result's offset in the data, the transform
    /// and the input's first word; the setters with an (offset, first word) pair per vector
    /// argument. Offsets are `NOT_IN_DATA` for the copies.
    fn run_transform(e: &mut Engine, this: u32, base: u32) -> Vec<Vec<u32>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let offset = move |p: u32| {
            if p.wrapping_sub(base) < 0x130 {
                p - base
            } else {
                NOT_IN_DATA
            }
        };
        for (address, mark) in [
            (SET_TRANSFORMED_POS, MARK_POSITION),
            (SET_ROTATED_DIR, MARK_AXIS),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                assert_eq!(a[0], a[2]);
                let word = e.mem.u32(a[2]);
                sink.borrow_mut()
                    .push(vec![address, offset(a[0]), a[1], word]);
                e.mem.set_u32(a[0], word | mark);
                Ret::default()
            });
        }
        for address in [
            BALL_AND_SOCKET_SETTER,
            HINGE_SETTER,
            LIMITED_HINGE_SETTER,
            PRISMATIC_SETTER,
            RAGDOLL_SETTER,
            WHEEL_SETTER,
            PLANE_SETTER,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                assert_eq!(a[0], base);
                let mut entry = vec![address];
                for &vector in &a[1..] {
                    entry.extend([offset(vector), e.mem.u32(vector)]);
                }
                sink.borrow_mut().push(entry);
                Ret::default()
            });
        }
        e.call(0x00cf_30f0, &args![this, TRANSFORM]);
        let calls = log.borrow().clone();
        calls
    }

    const COPY: u32 = NOT_IN_DATA;

    #[test]
    fn transform_ball_and_socket_moves_the_pivot_in_the_data() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 0, 0x0900_0100);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(
            calls,
            [
                vec![SET_TRANSFORMED_POS, 0x30, TRANSFORM, 0x100c],
                vec![
                    BALL_AND_SOCKET_SETTER,
                    0x20,
                    0x1008,
                    0x30,
                    0x100c | MARK_POSITION
                ],
            ]
        );
        assert_eq!(e.mem.u32(data + 0x30), 0x100c | MARK_POSITION);
    }

    #[test]
    fn transform_hinge_passes_four_copies() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 1, 0x0900_0100);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(
            calls,
            [
                vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x1024],
                vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1018],
                vec![
                    HINGE_SETTER,
                    COPY,
                    0x1014,
                    COPY,
                    0x1024 | MARK_POSITION,
                    COPY,
                    0x1008,
                    COPY,
                    0x1018 | MARK_AXIS
                ],
            ]
        );
        // The data itself is not touched.
        assert_eq!(e.mem.u32(data + 0x90), 0x1024);
    }

    #[test]
    fn transform_limited_hinge_and_prismatic_pass_six_copies() {
        for (kind, setter) in [(2, LIMITED_HINGE_SETTER), (6, PRISMATIC_SETTER)] {
            let mut e = Engine::new();
            let data = data_object(&mut e, kind, 0x0900_0100);
            let this = instance_over(&mut e, data);
            let calls = run_transform(&mut e, this, data);
            assert_eq!(
                calls,
                [
                    vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x1024],
                    vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1018],
                    vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x101c],
                    vec![
                        setter,
                        COPY,
                        0x1014,
                        COPY,
                        0x1024 | MARK_POSITION,
                        COPY,
                        0x1008,
                        COPY,
                        0x1018 | MARK_AXIS,
                        COPY,
                        0x100c,
                        COPY,
                        0x101c | MARK_AXIS
                    ],
                ]
            );
        }
    }

    #[test]
    fn transform_ragdoll_rotates_its_axes_in_the_data() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 7, 0x0900_0100);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(
            calls,
            [
                vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x1024],
                vec![SET_ROTATED_DIR, 0x70, TRANSFORM, 0x101c],
                vec![SET_ROTATED_DIR, 0x60, TRANSFORM, 0x1018],
                vec![
                    RAGDOLL_SETTER,
                    COPY,
                    0x1014,
                    COPY,
                    0x1024 | MARK_POSITION,
                    0x30,
                    0x100c,
                    0x70,
                    0x101c | MARK_AXIS,
                    0x20,
                    0x1008,
                    0x60,
                    0x1018 | MARK_AXIS
                ],
            ]
        );
        assert_eq!(e.mem.u32(data + 0x70), 0x101c | MARK_AXIS);
    }

    #[test]
    fn transform_type_eight_writes_the_vectors_and_the_float_back() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 8, 0x0900_0100);
        e.mem.set_u32(data + 0x44, 0x7f80_0001);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(calls, [vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x100c]]);
        assert_eq!(e.mem.u32(data + 0x20), 0x1008);
        assert_eq!(e.mem.u32(data + 0x2c), 0x100b);
        assert_eq!(e.mem.u32(data + 0x30), 0x100c | MARK_POSITION);
        assert_eq!(e.mem.u32(data + 0x3c), 0x100f);
        assert_eq!(e.mem.u32(data + 0x44), 0x7fc0_0001);
        assert_eq!(e.mem.u32(data + 0x48), 0x1012);
    }

    #[test]
    fn transform_wheel_passes_six_copies() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 9, 0x0900_0100);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(
            calls,
            [
                vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x1024],
                vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1018],
                vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1040],
                vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1044],
                vec![
                    WHEEL_SETTER,
                    COPY,
                    0x1014,
                    COPY,
                    0x1024 | MARK_POSITION,
                    COPY,
                    0x1034,
                    COPY,
                    0x1040 | MARK_AXIS,
                    COPY,
                    0x1018 | MARK_AXIS,
                    COPY,
                    0x1044 | MARK_AXIS
                ],
            ]
        );
    }

    #[test]
    fn transform_point_to_plane_picks_its_normal_by_the_count_byte() {
        let mut e = Engine::new();
        let data = data_object(&mut e, 14, 0x0900_0100);
        e.mem.set_u8(data + 0xa2, 2);
        let this = instance_over(&mut e, data);
        let calls = run_transform(&mut e, this, data);
        assert_eq!(
            calls,
            [
                vec![SET_TRANSFORMED_POS, COPY, TRANSFORM, 0x1024],
                vec![SET_ROTATED_DIR, COPY, TRANSFORM, 0x1020],
                vec![
                    PLANE_SETTER,
                    COPY,
                    0x1014,
                    COPY,
                    0x1024 | MARK_POSITION,
                    COPY,
                    0x1020 | MARK_AXIS
                ],
            ]
        );
    }

    #[test]
    fn transform_follows_wrapped_data() {
        let expected = [
            vec![SET_TRANSFORMED_POS, 0x30, TRANSFORM, 0x100c],
            vec![
                BALL_AND_SOCKET_SETTER,
                0x20,
                0x1008,
                0x30,
                0x100c | MARK_POSITION,
            ],
        ];
        // Type 12: the wrapped data is at +0x18.
        let mut e = Engine::new();
        let outer = data_object(&mut e, 12, 0x0900_0100);
        let inner = data_object(&mut e, 0, 0x0900_0101);
        e.mem.set_u32(outer + 0x18, inner);
        let this = instance_over(&mut e, outer);
        assert_eq!(run_transform(&mut e, this, inner), expected);
        // Type 13: the wrapped data comes from 00cdf6c0.
        let mut e = Engine::new();
        let outer = data_object(&mut e, 13, 0x0900_0100);
        let inner = data_object(&mut e, 12, 0x0900_0101);
        let innermost = data_object(&mut e, 0, 0x0900_0102);
        e.mem.set_u32(inner + 0x18, innermost);
        e.register_double(GET_WRAPPED_CONSTRAINT_DATA, move |_, a| {
            assert_eq!(a, [outer]);
            Ret {
                eax: inner,
                ..Ret::default()
            }
        });
        let this = instance_over(&mut e, outer);
        assert_eq!(run_transform(&mut e, this, innermost), expected);
    }

    #[test]
    fn transform_ignores_other_types_and_free_constraints() {
        for kind in [3, 4, 5, 10, 11, 15, 0x100] {
            let mut e = Engine::new();
            let data = data_object(&mut e, kind, 0x0900_0100);
            let this = instance_over(&mut e, data);
            assert!(run_transform(&mut e, this, data).is_empty());
        }
        // The first entity's world is missing, so the constraint is not tied to the world.
        let mut e = Engine::new();
        let data = data_object(&mut e, 0, 0x0900_0100);
        let this = instance_over(&mut e, data);
        let first_entity = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x14, first_entity);
        e.mem.set_u32(this + 0x18, 0x5ec0);
        assert!(run_transform(&mut e, this, data).is_empty());
    }
}
