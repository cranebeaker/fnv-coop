//! `Havok SDK/hkpphysicssystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{load_vec, store_vec, zip_with};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d06c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_6640`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d06c00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_6640, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// The physics system (`hkpPhysicsSystem`, Xbox PDB names): vtable at +0, reference count word at
// +6, four `hkArray`s of pointers (data +0, size +4, capacity and flags +8 inside each) for the
// rigid bodies (+0x08), constraints (+0x14), actions (+0x20) and phantoms (+0x2c), an
// `hkStringPtr` at +0x38, a dword at +0x3c and a byte at +0x40; 0x44 bytes in all.
const BODIES: u32 = 0x08;
const CONSTRAINTS: u32 = 0x14;
const ACTIONS: u32 = 0x20;
const PHANTOMS: u32 = 0x2c;
const NAME: u32 = 0x38;
/// Capacity word of an `hkArray`: bit 31 means the storage is not the array's to free.
const DONT_DEALLOCATE: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The vtable the constructor installs, and the base vtable the destructor ends with.
const PHYSICS_SYSTEM_VTABLE: u32 = 0x010c_ce28;
const BASE_VTABLE: u32 = 0x0102_e388;

/// `hkReferencedObject` add and remove reference (`Havok SDK/hkreferencedobject.cpp`; the
/// reference count is the word at +6): `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArrayUtil::_reserve` (Xbox PDB), cdecl `(allocator, array, count, element size)`, and the
/// unit's `00c90940` (cdecl `(allocator, array, element size)`), which grows an array that is
/// full.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkStringPtr` constructor (`this`, one word), destructor (`this`) and assignment (`this`,
/// the other string).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0b80;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c30;
/// `hkTransform::setMulEq` (Xbox PDB, `this`, one transform), `hkpRigidBody::setTransform`,
/// `hkpShapePhantom::setTransform`, `hkpAabbPhantom::setAabb` (each `this` and one pointer) and
/// `hkpConstraintInstance::transform`.
const TRANSFORM_MUL_EQ: u32 = 0x00cb_23e0;
const RIGID_BODY_SET_TRANSFORM: u32 = 0x00ca_8890;
const SHAPE_PHANTOM_SET_TRANSFORM: u32 = 0x00c9_e910;
const AABB_PHANTOM_SET_AABB: u32 = 0x00cf_24a0;
const CONSTRAINT_TRANSFORM: u32 = 0x00cf_30f0;
/// `hkpConstraintInstance::isConstrainedToWorld` (`this`, an out byte; returns that pointer) and
/// `hkpConstraintInstance::clone` (`this`, two entities, one word).
const CONSTRAINT_IS_CONSTRAINED_TO_WORLD: u32 = 0x00cf_2fb0;
const CONSTRAINT_CLONE: u32 = 0x00cf_3fa0;

/// The memory router's allocator (`router + 0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Element `index` of the pointer array at `array`; an index of `-1` (not found) reads the
/// word before the data, as the code does.
fn element(e: &Engine, array: u32, index: i32) -> u32 {
    let data = e.mem.u32(array);
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

/// The index of `value` in the pointer array at `array`, or `-1`.
fn index_of(e: &Engine, array: u32, value: u32) -> i32 {
    let count = e.mem.i32(array + 4);
    if count > 0 {
        let data = e.mem.u32(array);
        for index in 0..count {
            if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == value {
                return index;
            }
        }
    }
    -1
}

/// The inlined `hkArray::setSize` of pointers: grows the storage through `hkArrayUtil::_reserve`
/// when `size` exceeds the capacity (to at least twice the capacity), then stores the size.
fn array_set_size(e: &mut Engine, array: u32, size: i32) {
    let allocator = allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if size < doubled { doubled } else { size };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    e.mem.set_i32(array + 4, size);
}

/// The inlined empty-and-free of an `hkArray`: size 0, the storage returned to the allocator
/// (slot `0x10`, data and byte size) unless it is not the array's, then data 0 and the
/// "don't deallocate" capacity.
fn array_release(e: &mut Engine, array: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & DONT_DEALLOCATE == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, DONT_DEALLOCATE);
}

/// The inlined `hkArray::operator=` of pointers: when the capacity is below the other array's
/// size the old storage is freed (slot `0x10`) unless it is not the array's, new storage comes
/// from allocator slot `0xC` (it takes a pointer to the byte size, which it rounds up) and the
/// capacity becomes the granted size over four; then the size and the elements are copied.
fn array_assign(e: &mut Engine, array: u32, other: u32) {
    let allocator = allocator(e);
    let flags = e.mem.u32(array + 8);
    let capacity = (flags & CAPACITY_MASK) as i32;
    let other_size = e.mem.i32(other + 4);
    if capacity < other_size {
        if flags as i32 >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        let bytes = (other_size as u32).wrapping_mul(4);
        let (data, granted) = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), bytes);
            let data = e.vcall(allocator, 0xc, &args![local]).u32();
            (data, e.mem.i32(local.addr()))
        });
        e.mem.set_u32(array, data);
        e.mem
            .set_i32(array + 8, (granted + ((granted >> 31) & 3)) >> 2);
    }
    let size = e.mem.i32(other + 4);
    let data = e.mem.u32(array);
    e.mem.set_i32(array + 4, size);
    let other_data = e.mem.u32(other);
    for index in 0..size.max(0) as u32 {
        let word = e.mem.u32(other_data.wrapping_add(4 * index));
        e.mem.set_u32(data.wrapping_add(4 * index), word);
    }
}

/// Appends `item` to the pointer array at `array`, growing it through `00c90940` when it is
/// full (size equals capacity).
fn array_push(e: &mut Engine, array: u32, item: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), item);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Removes element `index` of the pointer array at `array` (size at `+4`): releases the
/// element, shrinks the size and moves the last element into the hole.
fn array_remove(e: &mut Engine, array: u32, index: i32) {
    let item = element(e, array, index);
    e.call(REMOVE_REFERENCE, &args![item]);
    let last = e.mem.i32(array + 4).wrapping_sub(1);
    e.mem.set_i32(array + 4, last);
    if last != index {
        let data = e.mem.u32(array);
        let moved = e.mem.u32(data.wrapping_add((last as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
}

/// Releases every non-null element of the pointer array at `array` (`count` entries, the data
/// pointer read again for each).
fn release_elements(e: &mut Engine, array: u32, count: i32) {
    for index in 0..count {
        let slot = e
            .mem
            .u32(array)
            .wrapping_add((index as u32).wrapping_mul(4));
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(REMOVE_REFERENCE, &args![item]);
        }
    }
}

// Translated from 00d05df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `tranformPhantom` (Xbox PDB): cdecl `(phantom, transform)`. Calls the phantom's virtual
/// at +0x14 (its kind). Kind 0 (an AABB phantom): copies the 32-byte box at +0xB0, adds the
/// transform's translation vector (`transform + 0x30`) to both corners and calls
/// `hkpAabbPhantom::setAabb`. Kinds 1 and 2 (shape phantoms): copies the 64-byte transform at
/// +0xB0, multiplies it by `transform` (`hkTransform::setMulEq`) and calls
/// `hkpShapePhantom::setTransform` on the phantom. Other kinds do nothing.
pub fn tranform_phantom(e: &mut Engine, this: Ptr, transform: Ptr) {
    let kind = e.vcall(this.addr(), 0x14, &args![]).u32();
    if kind == 0 {
        let translation = load_vec(e, transform.addr() + 0x30);
        e.with_stack(0x20, |e, aabb| {
            for word in 0..8 {
                let value = e.mem.u32(this.addr() + 0xb0 + 4 * word);
                e.mem.set_u32(aabb.addr() + 4 * word, value);
            }
            let low = zip_with(translation, load_vec(e, aabb.addr()), |a, b| a + b);
            store_vec(e, aabb.addr(), low);
            let high = zip_with(translation, load_vec(e, aabb.addr() + 0x10), |a, b| a + b);
            store_vec(e, aabb.addr() + 0x10, high);
            e.call(AABB_PHANTOM_SET_AABB, &args![this, aabb]);
        });
    } else if kind.wrapping_sub(1) <= 1 {
        e.with_stack(0x40, |e, copy| {
            for word in 0..16 {
                let value = e.mem.u32(this.addr() + 0xb0 + 4 * word);
                e.mem.set_u32(copy.addr() + 4 * word, value);
            }
            e.call(TRANSFORM_MUL_EQ, &args![copy, transform]);
            e.call(SHAPE_PHANTOM_SET_TRANSFORM, &args![this, copy]);
        });
    }
}

// Translated from 00d05f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::transform` (Xbox PDB): applies `transform` to every rigid body (a copy of
/// `transform` multiplied by the body's transform at +0xF0, then `hkpRigidBody::setTransform`),
/// every constraint (`hkpConstraintInstance::transform`) and every phantom (`tranformPhantom`).
pub fn physics_system_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.addr() + 0xc) {
        e.with_stack(0x40, |e, copy| {
            for word in 0..16 {
                let value = e.mem.u32(transform.addr() + 4 * word);
                e.mem.set_u32(copy.addr() + 4 * word, value);
            }
            let body = element(e, this.addr() + BODIES, index as i32);
            e.call(TRANSFORM_MUL_EQ, &args![copy, body.wrapping_add(0xf0)]);
            let body = element(e, this.addr() + BODIES, index as i32);
            e.call(RIGID_BODY_SET_TRANSFORM, &args![body, copy]);
        });
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.addr() + 0x18) {
        let constraint = element(e, this.addr() + CONSTRAINTS, index as i32);
        e.call(CONSTRAINT_TRANSFORM, &args![constraint, transform]);
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.addr() + 0x30) {
        let phantom = element(e, this.addr() + PHANTOMS, index as i32);
        tranform_phantom(e, Ptr::new(phantom), transform);
        index += 1;
    }
}

// Translated from 00d06090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::removeAll` (Xbox PDB): releases every non-null rigid body, phantom,
/// constraint and action in that order (`00c905b0`), zeroing each array's size after its
/// loop (the next array's size is read first).
pub fn physics_system_remove_all(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let count = e.mem.i32(this + 0xc);
    release_elements(e, this + BODIES, count);
    let count = e.mem.i32(this + 0x30);
    e.mem.set_u32(this + 0xc, 0);
    release_elements(e, this + PHANTOMS, count);
    let count = e.mem.i32(this + 0x18);
    e.mem.set_u32(this + 0x30, 0);
    release_elements(e, this + CONSTRAINTS, count);
    let count = e.mem.i32(this + 0x24);
    e.mem.set_u32(this + 0x18, 0);
    release_elements(e, this + ACTIONS, count);
    e.mem.set_u32(this + 0x24, 0);
}

// Translated from 00d06130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::removeRigidBody` (Xbox PDB): releases rigid body `index`, shrinks the
/// array and moves the last body into its place.
pub fn physics_system_remove_rigid_body(e: &mut Engine, this: Ptr, index: i32) {
    array_remove(e, this.addr() + BODIES, index);
}

// Translated from 00d061b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::removeConstraint` (Xbox PDB): as `removeRigidBody` on the constraints.
pub fn physics_system_remove_constraint(e: &mut Engine, this: Ptr, index: i32) {
    array_remove(e, this.addr() + CONSTRAINTS, index);
}

// Translated from 00d061f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::removeAction` (Xbox PDB): as `removeRigidBody` on the actions.
pub fn physics_system_remove_action(e: &mut Engine, this: Ptr, index: i32) {
    array_remove(e, this.addr() + ACTIONS, index);
}

// Translated from 00d06290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the physics system (the body has the shape of `operator=`; the engine
/// map has no name for it, so the name is unconfirmed): copies the rigid body, phantom,
/// constraint and action arrays (the inlined `hkArray` assignment), then the name
/// (`hkStringPtr` assignment at +0x38), the dword at +0x3C and the byte at +0x40 of `other`.
pub fn fn_00d06290(e: &mut Engine, this: Ptr, other: Ptr) {
    let (this, other) = (this.addr(), other.addr());
    array_assign(e, this + BODIES, other + BODIES);
    array_assign(e, this + PHANTOMS, other + PHANTOMS);
    array_assign(e, this + CONSTRAINTS, other + CONSTRAINTS);
    array_assign(e, this + ACTIONS, other + ACTIONS);
    e.call(STRING_PTR_ASSIGN, &args![this + NAME, other + NAME]);
    let word = e.mem.u32(other + 0x3c);
    e.mem.set_u32(this + 0x3c, word);
    let flag = e.mem.u8(other + 0x40);
    e.mem.set_u8(this + 0x40, flag);
}

// Translated from 00d064a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::addRigidBody` (Xbox PDB): a non-null body gets a reference added
/// (`00c90510`) and is appended to the rigid body array.
pub fn physics_system_add_rigid_body(e: &mut Engine, this: Ptr, body: u32) {
    if body != 0 {
        e.call(ADD_REFERENCE, &args![body]);
        array_push(e, this.addr() + BODIES, body);
    }
}

// Translated from 00d064f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::addPhantom` (Xbox PDB): as `addRigidBody` on the phantom array (+0x2C).
pub fn physics_system_add_phantom(e: &mut Engine, this: Ptr, phantom: u32) {
    if phantom != 0 {
        e.call(ADD_REFERENCE, &args![phantom]);
        array_push(e, this.addr() + PHANTOMS, phantom);
    }
}

// Translated from 00d06540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::addConstraint` (Xbox PDB): as `addRigidBody` on the constraint array
/// (+0x14).
pub fn physics_system_add_constraint(e: &mut Engine, this: Ptr, constraint: u32) {
    if constraint != 0 {
        e.call(ADD_REFERENCE, &args![constraint]);
        array_push(e, this.addr() + CONSTRAINTS, constraint);
    }
}

// Translated from 00d06590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::addAction` (Xbox PDB): as `addRigidBody` on the action array (+0x20).
pub fn physics_system_add_action(e: &mut Engine, this: Ptr, action: u32) {
    if action != 0 {
        e.call(ADD_REFERENCE, &args![action]);
        array_push(e, this.addr() + ACTIONS, action);
    }
}

// Translated from 00d065e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::hkpPhysicsSystem` (Xbox PDB): installs the vtable, reference count 1 at
/// +6, four empty arrays with the "don't deallocate" capacity, constructs the name
/// (`hkStringPtr` constructor with 0), sets +0x3C to 0 and the byte at +0x40 to 1. Returns
/// `this`.
pub fn physics_system_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u32(this_addr, PHYSICS_SYSTEM_VTABLE);
    e.mem.set_u16(this_addr + 6, 1);
    e.mem.set_u32(this_addr + 0x10, DONT_DEALLOCATE);
    e.mem.set_u32(this_addr + 0x8, 0);
    e.mem.set_u32(this_addr + 0xc, 0);
    e.mem.set_u32(this_addr + 0x14, 0);
    e.mem.set_u32(this_addr + 0x18, 0);
    e.mem.set_u32(this_addr + 0x1c, DONT_DEALLOCATE);
    e.mem.set_u32(this_addr + 0x20, 0);
    e.mem.set_u32(this_addr + 0x24, 0);
    e.mem.set_u32(this_addr + 0x28, DONT_DEALLOCATE);
    e.mem.set_u32(this_addr + 0x2c, 0);
    e.mem.set_u32(this_addr + 0x30, 0);
    e.mem.set_u32(this_addr + 0x34, DONT_DEALLOCATE);
    e.call(STRING_PTR_CONSTRUCT, &args![this_addr + NAME, 0u32]);
    e.mem.set_u32(this_addr + 0x3c, 0);
    e.mem.set_u8(this_addr + 0x40, 1);
    this
}

// Translated from 00d06640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::~hkpPhysicsSystem` (Xbox PDB): installs the vtable, `removeAll`, destroys
/// the name, empties and frees the phantom, action, constraint and rigid body arrays, then
/// installs the base vtable.
pub fn physics_system_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PHYSICS_SYSTEM_VTABLE);
    physics_system_remove_all(e, this);
    e.call(STRING_PTR_DESTRUCT, &args![this.addr() + NAME]);
    array_release(e, this.addr() + PHANTOMS);
    array_release(e, this.addr() + ACTIONS);
    array_release(e, this.addr() + CONSTRAINTS);
    array_release(e, this.addr() + BODIES);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00d06720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsSystem::clone` (Xbox PDB): allocates 0x44 bytes (allocator slot 4; the size is
/// stored at +4), constructs a system in it, copies the name, the dword at +0x3C and the byte
/// at +0x40, sizes the four arrays like the source's, then fills them. Every rigid body is
/// replaced by its virtual at +0x18 (clone) and every phantom by its virtual at +0x2C. Each
/// constraint is cloned with `hkpConstraintInstance::clone(entity A, entity B, 0)` where the
/// entities are the copies at the source's indices (entity B is 0 when
/// `isConstrainedToWorld` says so). Each non-null action collects its entities (virtual
/// +0x10) and phantoms (virtual +0x14) into temporary arrays, mapped to the copies, and is
/// cloned by its virtual at +0x1C with the two arrays. Returns the new system.
pub fn physics_system_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    let allocator_object = allocator(e);
    let block = e.vcall(allocator_object, 4, &args![0x44u32]).u32();
    e.mem.set_u16(block + 4, 0x44);
    let copy = physics_system_construct(e, Ptr::new(block)).addr();
    e.call(STRING_PTR_ASSIGN, &args![copy + NAME, this + NAME]);
    let word = e.mem.u32(this + 0x3c);
    e.mem.set_u32(copy + 0x3c, word);
    let flag = e.mem.u8(this + 0x40);
    e.mem.set_u8(copy + 0x40, flag);
    let size = e.mem.i32(this + 0xc);
    array_set_size(e, copy + BODIES, size);
    let size = e.mem.i32(this + 0x30);
    array_set_size(e, copy + PHANTOMS, size);
    let size = e.mem.i32(this + 0x18);
    array_set_size(e, copy + CONSTRAINTS, size);
    let size = e.mem.i32(this + 0x24);
    array_set_size(e, copy + ACTIONS, size);

    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0xc) {
        let source_data = e.mem.u32(this + BODIES);
        let copy_data = e.mem.u32(copy + BODIES);
        let body = e.mem.u32(source_data.wrapping_add(4 * index));
        let cloned = e.vcall(body, 0x18, &args![]).u32();
        e.mem.set_u32(copy_data.wrapping_add(4 * index), cloned);
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0x30) {
        let source_data = e.mem.u32(this + PHANTOMS);
        let copy_data = e.mem.u32(copy + PHANTOMS);
        let phantom = e.mem.u32(source_data.wrapping_add(4 * index));
        let cloned = e.vcall(phantom, 0x2c, &args![]).u32();
        e.mem.set_u32(copy_data.wrapping_add(4 * index), cloned);
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0x18) {
        let constraint = element(e, this + CONSTRAINTS, index as i32);
        let entity_a = e.mem.u32(constraint + 0x14);
        let found = index_of(e, this + BODIES, entity_a);
        let copy_a = element(e, copy + BODIES, found);
        let to_world = e.with_stack(4, |e, out| {
            let flag_pointer = e
                .call(CONSTRAINT_IS_CONSTRAINED_TO_WORLD, &args![constraint, out])
                .u32();
            e.mem.u8(flag_pointer)
        });
        let copy_b = if to_world != 0 {
            0
        } else {
            let entity_b = e.mem.u32(constraint + 0x18);
            let found = index_of(e, this + BODIES, entity_b);
            element(e, copy + BODIES, found)
        };
        let destination = e.mem.u32(copy + CONSTRAINTS).wrapping_add(4 * index);
        let cloned = e
            .call(CONSTRAINT_CLONE, &args![constraint, copy_a, copy_b, 0u32])
            .u32();
        e.mem.set_u32(destination, cloned);
        index += 1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0x24) {
        let action = element(e, this + ACTIONS, index as i32);
        if action == 0 {
            let data = e.mem.u32(copy + ACTIONS);
            e.mem.set_u32(data.wrapping_add(4 * index), 0);
        } else {
            // Four temporary arrays (data, size, capacity): the action's entities, their
            // copies, the action's phantoms and their copies.
            e.with_stack(0x30, |e, arrays| {
                let entities = arrays.addr();
                let entity_copies = arrays.addr() + 0xc;
                let phantoms = arrays.addr() + 0x18;
                let phantom_copies = arrays.addr() + 0x24;
                e.mem.set_u32(entities, 0);
                e.mem.set_u32(entities + 4, 0);
                e.mem.set_u32(entities + 8, DONT_DEALLOCATE);
                e.vcall(action, 0x10, &args![entities]);
                let count = e.mem.i32(entities + 4);
                e.mem.set_u32(entity_copies, 0);
                e.mem.set_u32(entity_copies + 4, 0);
                e.mem.set_u32(entity_copies + 8, DONT_DEALLOCATE);
                array_set_size(e, entity_copies, count);
                for position in 0..count.max(0) as u32 {
                    let entity = element(e, entities, position as i32);
                    let found = index_of(e, this + BODIES, entity);
                    let mapped = element(e, copy + BODIES, found);
                    let data = e.mem.u32(entity_copies);
                    e.mem.set_u32(data.wrapping_add(4 * position), mapped);
                }
                e.mem.set_u32(phantoms, 0);
                e.mem.set_u32(phantoms + 4, 0);
                e.mem.set_u32(phantoms + 8, DONT_DEALLOCATE);
                e.vcall(action, 0x14, &args![phantoms]);
                let count = e.mem.i32(phantoms + 4);
                e.mem.set_u32(phantom_copies, 0);
                e.mem.set_u32(phantom_copies + 4, 0);
                e.mem.set_u32(phantom_copies + 8, DONT_DEALLOCATE);
                array_set_size(e, phantom_copies, count);
                for position in 0..count.max(0) as u32 {
                    let phantom = element(e, phantoms, position as i32);
                    let found = index_of(e, this + PHANTOMS, phantom);
                    let mapped = element(e, copy + PHANTOMS, found);
                    let data = e.mem.u32(phantom_copies);
                    e.mem.set_u32(data.wrapping_add(4 * position), mapped);
                }
                let destination = e.mem.u32(copy + ACTIONS).wrapping_add(4 * index);
                let cloned = e
                    .vcall(action, 0x1c, &args![entity_copies, phantom_copies])
                    .u32();
                e.mem.set_u32(destination, cloned);
                array_release(e, phantom_copies);
                array_release(e, phantoms);
                array_release(e, entity_copies);
                array_release(e, entities);
            });
        }
        index += 1;
    }
    Ptr::new(copy)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_6c00, deleting_destructor_00d06c00(Ptr, u8) -> Ptr),
        entry!(0x00d0_5df0, tranform_phantom(Ptr, Ptr)),
        entry!(0x00d0_5f60, physics_system_transform(Ptr, Ptr)),
        entry!(0x00d0_6090, physics_system_remove_all(Ptr)),
        entry!(0x00d0_6130, physics_system_remove_rigid_body(Ptr, i32)),
        entry!(0x00d0_61b0, physics_system_remove_constraint(Ptr, i32)),
        entry!(0x00d0_61f0, physics_system_remove_action(Ptr, i32)),
        entry!(0x00d0_6290, fn_00d06290(Ptr, Ptr)),
        entry!(0x00d0_64a0, physics_system_add_rigid_body(Ptr, u32)),
        entry!(0x00d0_64f0, physics_system_add_phantom(Ptr, u32)),
        entry!(0x00d0_6540, physics_system_add_constraint(Ptr, u32)),
        entry!(0x00d0_6590, physics_system_add_action(Ptr, u32)),
        entry!(0x00d0_65e0, physics_system_construct(Ptr) -> Ptr),
        entry!(0x00d0_6640, physics_system_destruct(Ptr)),
        entry!(0x00d0_6720, physics_system_clone(Ptr) -> Ptr),
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
    fn deleting_destructor_00d06c00_frees_only_when_bit_zero_is_set() {
        check(0x00d0_6c00, 0x00d0_6640);
    }
}

#[cfg(test)]
mod a05_physics_system_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE: u32 = 0x0900_0104;
    const ALLOCATE_GRANT: u32 = 0x0900_010c;
    const FREE: u32 = 0x0900_0110;

    /// An engine with a memory router whose allocator hands out heap blocks (slot 4), grants
    /// eight bytes more than asked (slot `0xC`) and frees (slot `0x10`, a logged double).
    fn world() -> Engine {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_GRANT);
        e.mem.set_u32(vtable + 0x10, FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(ALLOCATE_GRANT, |e, a| {
            let bytes = e.mem.u32(a[1]) + 8;
            e.mem.set_u32(a[1], bytes);
            Ret {
                eax: e.mem.alloc(bytes),
                ..Ret::default()
            }
        });
        e.register(FREE, |_, _| Ret::default());
        e.map(0x0200_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// The arguments of every logged call to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The `(data, byte size)` of every block freed through the allocator.
    fn frees(e: &Engine) -> Vec<(u32, u32)> {
        calls_to(e, FREE)
            .into_iter()
            .map(|args| (args[1], args[2]))
            .collect()
    }

    /// A pointer array at `array` holding `items` in heap storage, with the capacity word.
    fn fill_array(e: &mut Engine, array: u32, items: &[u32], capacity: u32) {
        let data = e.mem.alloc(4 * (items.len() as u32 + 4));
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, items.len() as u32);
        e.mem.set_u32(array + 8, capacity);
    }

    fn items(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    /// An object whose vtable has the given `(slot, function)` pairs.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, function) in slots {
            e.mem.set_u32(vtable + slot, *function);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A phantom object whose virtual at +0x14 returns `kind`, with the 64 bytes at +0xB0
    /// numbered from `base`.
    fn phantom(e: &mut Engine, kind: u32, base: f32) -> u32 {
        let object = object_with(e, &[(0x14, 0x0900_0200)]);
        e.register_double(0x0900_0200, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        for word in 0..16 {
            e.mem.set_f32(object + 0xb0 + 4 * word, base + word as f32);
        }
        object
    }

    #[test]
    fn tranform_phantom_translates_the_box_of_a_kind_zero_phantom() {
        let mut e = world();
        let object = phantom(&mut e, 0, 1.0);
        let transform = e.mem.alloc(0x40);
        for lane in 0..4 {
            e.mem
                .set_f32(transform + 0x30 + 4 * lane, 10.0 * (lane + 1) as f32);
        }
        e.register(AABB_PHANTOM_SET_AABB, |e, a| {
            for word in 0..8 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(0x0200_0000 + 4 * word, value);
            }
            Ret::default()
        });
        e.call(0x00d0_5df0, &args![object, transform]);
        let calls = calls_to(&e, AABB_PHANTOM_SET_AABB);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], object);
        let expected = [11.0, 22.0, 33.0, 44.0, 15.0, 26.0, 37.0, 48.0];
        for (word, value) in expected.into_iter().enumerate() {
            assert_eq!(e.mem.f32(0x0200_0000 + 4 * word as u32), value);
        }
        assert!(calls_to(&e, SHAPE_PHANTOM_SET_TRANSFORM).is_empty());
    }

    #[test]
    fn tranform_phantom_multiplies_the_transform_of_a_shape_phantom() {
        for kind in [1, 2] {
            let mut e = world();
            let object = phantom(&mut e, kind, 100.0);
            let transform = e.mem.alloc(0x40);
            e.register(TRANSFORM_MUL_EQ, |e, a| {
                for word in 0..16 {
                    let value = e.mem.u32(a[0] + 4 * word);
                    e.mem.set_u32(0x0200_0000 + 4 * word, value);
                }
                Ret::default()
            });
            e.call(0x00d0_5df0, &args![object, transform]);
            let multiply = calls_to(&e, TRANSFORM_MUL_EQ);
            assert_eq!(multiply.len(), 1);
            assert_eq!(multiply[0][1], transform);
            let set = calls_to(&e, SHAPE_PHANTOM_SET_TRANSFORM);
            assert_eq!(set, vec![vec![object, multiply[0][0]]]);
            assert_eq!(e.mem.f32(0x0200_0000 + 4 * 15), 115.0);
            assert!(calls_to(&e, AABB_PHANTOM_SET_AABB).is_empty());
        }
    }

    #[test]
    fn tranform_phantom_ignores_other_kinds() {
        let mut e = world();
        let object = phantom(&mut e, 3, 0.0);
        let transform = e.mem.alloc(0x40);
        e.call(0x00d0_5df0, &args![object, transform]);
        assert!(calls_to(&e, AABB_PHANTOM_SET_AABB).is_empty());
        assert!(calls_to(&e, TRANSFORM_MUL_EQ).is_empty());
        assert!(calls_to(&e, SHAPE_PHANTOM_SET_TRANSFORM).is_empty());
    }

    #[test]
    fn transform_moves_every_body_constraint_and_phantom() {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        let bodies = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        fill_array(&mut e, system + BODIES, &bodies, 2);
        let constraint = e.mem.alloc(0x20);
        fill_array(&mut e, system + CONSTRAINTS, &[constraint], 1);
        let object = phantom(&mut e, 5, 0.0);
        fill_array(&mut e, system + PHANTOMS, &[object], 1);
        let transform = e.mem.alloc(0x40);
        e.register(TRANSFORM_MUL_EQ, |_, _| Ret::default());
        e.register(RIGID_BODY_SET_TRANSFORM, |_, _| Ret::default());
        e.register(CONSTRAINT_TRANSFORM, |_, _| Ret::default());
        e.call(0x00d0_5f60, &args![system, transform]);
        let multiply = calls_to(&e, TRANSFORM_MUL_EQ);
        assert_eq!(multiply.len(), 2);
        assert_eq!(multiply[0][1], bodies[0] + 0xf0);
        assert_eq!(multiply[1][1], bodies[1] + 0xf0);
        let set = calls_to(&e, RIGID_BODY_SET_TRANSFORM);
        assert_eq!(set.len(), 2);
        assert_eq!((set[0][0], set[0][1]), (bodies[0], multiply[0][0]));
        assert_eq!((set[1][0], set[1][1]), (bodies[1], multiply[1][0]));
        assert_eq!(
            calls_to(&e, CONSTRAINT_TRANSFORM),
            vec![vec![constraint, transform]]
        );
        // The phantom is of kind 5: only its virtual runs.
        assert!(calls_to(&e, AABB_PHANTOM_SET_AABB).is_empty());
    }

    #[test]
    fn remove_all_releases_non_null_elements_and_zeroes_the_sizes() {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        fill_array(&mut e, system + BODIES, &[0x11, 0, 0x12], 4);
        fill_array(&mut e, system + PHANTOMS, &[0x21], 4);
        fill_array(&mut e, system + CONSTRAINTS, &[0, 0x31], 4);
        fill_array(&mut e, system + ACTIONS, &[0x41, 0x42], 4);
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.call(0x00d0_6090, &args![system]);
        let released: Vec<u32> = calls_to(&e, REMOVE_REFERENCE)
            .into_iter()
            .map(|args| args[0])
            .collect();
        assert_eq!(released, vec![0x11, 0x12, 0x21, 0x31, 0x41, 0x42]);
        for size in [0xc, 0x30, 0x18, 0x24] {
            assert_eq!(e.mem.u32(system + size), 0);
        }
    }

    /// Removing index 0 of three releases that element, shrinks the array and moves the last
    /// element into place; removing the last index only shrinks it.
    fn check_remove(address: u32, array: u32) {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        fill_array(&mut e, system + array, &[0xa1, 0xa2, 0xa3], 4);
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.call(address, &args![system, 0u32]);
        assert_eq!(items(&e, system + array), vec![0xa3, 0xa2]);
        e.call(address, &args![system, 1u32]);
        assert_eq!(items(&e, system + array), vec![0xa3]);
        let released: Vec<u32> = calls_to(&e, REMOVE_REFERENCE)
            .into_iter()
            .map(|args| args[0])
            .collect();
        assert_eq!(released, vec![0xa1, 0xa2]);
    }

    #[test]
    fn remove_rigid_body_swaps_in_the_last_one() {
        check_remove(0x00d0_6130, BODIES);
    }

    #[test]
    fn remove_constraint_swaps_in_the_last_one() {
        check_remove(0x00d0_61b0, CONSTRAINTS);
    }

    #[test]
    fn remove_action_swaps_in_the_last_one() {
        check_remove(0x00d0_61f0, ACTIONS);
    }

    /// A null item does nothing; an item gets a reference and is appended; a full array grows
    /// first through `00c90940` (the double sets a larger capacity).
    fn check_add(address: u32, array: u32) {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        fill_array(&mut e, system + array, &[0xb1], 1);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(address, &args![system, 0u32]);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
        assert_eq!(items(&e, system + array), vec![0xb1]);
        e.call(address, &args![system, 0xb2u32]);
        assert_eq!(items(&e, system + array), vec![0xb1, 0xb2]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0xb2]]);
        let reserve = calls_to(&e, ARRAY_RESERVE_MORE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][1..], [system + array, 4]);
        e.call(address, &args![system, 0xb3u32]);
        assert_eq!(items(&e, system + array), vec![0xb1, 0xb2, 0xb3]);
        assert_eq!(calls_to(&e, ARRAY_RESERVE_MORE).len(), 1);
    }

    #[test]
    fn add_rigid_body_appends_and_adds_a_reference() {
        check_add(0x00d0_64a0, BODIES);
    }

    #[test]
    fn add_phantom_appends_and_adds_a_reference() {
        check_add(0x00d0_64f0, PHANTOMS);
    }

    #[test]
    fn add_constraint_appends_and_adds_a_reference() {
        check_add(0x00d0_6540, CONSTRAINTS);
    }

    #[test]
    fn add_action_appends_and_adds_a_reference() {
        check_add(0x00d0_6590, ACTIONS);
    }

    #[test]
    fn construct_sets_up_empty_arrays_and_the_name() {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        e.register(STRING_PTR_CONSTRUCT, |_, _| Ret::default());
        let got = e.call(0x00d0_65e0, &args![system]).u32();
        assert_eq!(got, system);
        assert_eq!(e.mem.u32(system), PHYSICS_SYSTEM_VTABLE);
        assert_eq!(e.mem.u16(system + 6), 1);
        for array in [BODIES, CONSTRAINTS, ACTIONS, PHANTOMS] {
            assert_eq!(e.mem.u32(system + array), 0);
            assert_eq!(e.mem.u32(system + array + 4), 0);
            assert_eq!(e.mem.u32(system + array + 8), DONT_DEALLOCATE);
        }
        assert_eq!(e.mem.u32(system + 0x3c), 0);
        assert_eq!(e.mem.u8(system + 0x40), 1);
        assert_eq!(
            calls_to(&e, STRING_PTR_CONSTRUCT),
            vec![vec![system + NAME, 0]]
        );
    }

    #[test]
    fn destruct_releases_everything_and_frees_owned_storage() {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        fill_array(&mut e, system + BODIES, &[0x11], 4);
        fill_array(&mut e, system + PHANTOMS, &[], DONT_DEALLOCATE | 4);
        fill_array(&mut e, system + ACTIONS, &[0x41], 4);
        fill_array(&mut e, system + CONSTRAINTS, &[0x31], 2);
        let owned = [
            e.mem.u32(system + ACTIONS),
            e.mem.u32(system + CONSTRAINTS),
            e.mem.u32(system + BODIES),
        ];
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(STRING_PTR_DESTRUCT, |_, _| Ret::default());
        e.call(0x00d0_6640, &args![system]);
        assert_eq!(e.mem.u32(system), BASE_VTABLE);
        // Storage with the "don't deallocate" flag (phantoms) is not freed.
        assert_eq!(
            frees(&e),
            vec![(owned[0], 16), (owned[1], 8), (owned[2], 16)]
        );
        for array in [BODIES, CONSTRAINTS, ACTIONS, PHANTOMS] {
            assert_eq!(e.mem.u32(system + array), 0);
            assert_eq!(e.mem.u32(system + array + 4), 0);
            assert_eq!(e.mem.u32(system + array + 8), DONT_DEALLOCATE);
        }
        assert_eq!(calls_to(&e, STRING_PTR_DESTRUCT), vec![vec![system + NAME]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 3);
    }

    #[test]
    fn assignment_copies_the_arrays_the_name_and_the_trailing_fields() {
        let mut e = world();
        let system = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        // The bodies array has room and keeps its storage; the others get new storage.
        fill_array(&mut e, system + BODIES, &[], 8);
        let kept = e.mem.u32(system + BODIES);
        fill_array(&mut e, other + BODIES, &[1, 2], 2);
        fill_array(&mut e, other + PHANTOMS, &[3, 4, 5], 3);
        fill_array(&mut e, other + CONSTRAINTS, &[6], 1);
        fill_array(&mut e, other + ACTIONS, &[7, 8], 2);
        e.mem.set_u32(system + PHANTOMS + 8, DONT_DEALLOCATE);
        e.mem.set_u32(system + CONSTRAINTS + 8, DONT_DEALLOCATE);
        fill_array(&mut e, system + ACTIONS, &[9], 1);
        let old_actions = e.mem.u32(system + ACTIONS);
        e.mem.set_u32(other + 0x3c, 0x1234_5678);
        e.mem.set_u8(other + 0x40, 7);
        e.register(STRING_PTR_ASSIGN, |_, _| Ret::default());
        e.call(0x00d0_6290, &args![system, other]);
        assert_eq!(e.mem.u32(system + BODIES), kept);
        assert_eq!(items(&e, system + BODIES), vec![1, 2]);
        assert_eq!(items(&e, system + PHANTOMS), vec![3, 4, 5]);
        assert_eq!(items(&e, system + CONSTRAINTS), vec![6]);
        assert_eq!(items(&e, system + ACTIONS), vec![7, 8]);
        // Granted size = asked + 8, over four.
        assert_eq!(e.mem.u32(system + PHANTOMS + 8), (12 + 8) / 4);
        assert_eq!(e.mem.u32(system + CONSTRAINTS + 8), (4 + 8) / 4);
        // Only storage that was not flagged is freed: the actions' (capacity 1, 4 bytes).
        assert_eq!(frees(&e), vec![(old_actions, 4)]);
        assert_eq!(
            calls_to(&e, STRING_PTR_ASSIGN),
            vec![vec![system + NAME, other + NAME]]
        );
        assert_eq!(e.mem.u32(system + 0x3c), 0x1234_5678);
        assert_eq!(e.mem.u8(system + 0x40), 7);
    }

    const BODY_CLONE: u32 = 0x0900_0300;
    const PHANTOM_CLONE: u32 = 0x0900_0304;
    const ACTION_ENTITIES: u32 = 0x0900_0308;
    const ACTION_PHANTOMS: u32 = 0x0900_030c;
    const ACTION_CLONE: u32 = 0x0900_0310;

    #[test]
    fn clone_copies_the_system_and_maps_constraints_and_actions_to_the_copies() {
        let mut e = world();
        e.register(STRING_PTR_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_PTR_ASSIGN, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(4 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register(BODY_CLONE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(PHANTOM_CLONE, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        e.register(CONSTRAINT_IS_CONSTRAINED_TO_WORLD, |e, a| {
            let to_world = e.mem.u8(a[0] + 0x1c);
            e.mem.set_u8(a[1], to_world);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(CONSTRAINT_CLONE, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(ACTION_ENTITIES, |e, a| {
            let data = e.mem.alloc(8);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            let first = e.mem.u32(a[0] + 0x10);
            let second = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(data, first);
            e.mem.set_u32(data + 4, second);
            Ret::default()
        });
        e.register(ACTION_PHANTOMS, |e, a| {
            let data = e.mem.alloc(4);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 1);
            let phantom = e.mem.u32(a[0] + 0x18);
            e.mem.set_u32(data, phantom);
            Ret::default()
        });
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(ACTION_CLONE, move |e, a| {
            let read = |array: u32| -> Vec<u32> {
                let data = e.mem.u32(array);
                (0..e.mem.u32(array + 4))
                    .map(|index| e.mem.u32(data + 4 * index))
                    .collect()
            };
            log.borrow_mut().push(read(a[1]));
            log.borrow_mut().push(read(a[2]));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        let system = e.mem.alloc(0x44);
        let body_vtable = [(0x18, BODY_CLONE)];
        let first = object_with(&mut e, &body_vtable);
        let second = object_with(&mut e, &body_vtable);
        let object = object_with(&mut e, &[(0x2c, PHANTOM_CLONE)]);
        let constraint_a = e.mem.alloc(0x40);
        e.mem.set_u32(constraint_a + 0x14, second);
        e.mem.set_u32(constraint_a + 0x18, first);
        let constraint_b = e.mem.alloc(0x40);
        e.mem.set_u32(constraint_b + 0x14, first);
        e.mem.set_u32(constraint_b + 0x18, second);
        e.mem.set_u8(constraint_b + 0x1c, 1);
        let action = object_with(
            &mut e,
            &[
                (0x10, ACTION_ENTITIES),
                (0x14, ACTION_PHANTOMS),
                (0x1c, ACTION_CLONE),
            ],
        );
        e.mem.set_u32(action + 0x10, second);
        e.mem.set_u32(action + 0x14, first);
        e.mem.set_u32(action + 0x18, object);
        fill_array(&mut e, system + BODIES, &[first, second], 2);
        fill_array(&mut e, system + PHANTOMS, &[object], 1);
        fill_array(
            &mut e,
            system + CONSTRAINTS,
            &[constraint_a, constraint_b],
            2,
        );
        fill_array(&mut e, system + ACTIONS, &[0, action], 2);
        e.mem.set_u32(system + 0x3c, 0xabcd);
        e.mem.set_u8(system + 0x40, 9);
        let copy = e.call(0x00d0_6720, &args![system]).u32();
        assert_ne!(copy, system);
        assert_eq!(e.mem.u16(copy + 4), 0x44);
        assert_eq!(e.mem.u32(copy), PHYSICS_SYSTEM_VTABLE);
        assert_eq!(e.mem.u32(copy + 0x3c), 0xabcd);
        assert_eq!(e.mem.u8(copy + 0x40), 9);
        assert_eq!(items(&e, copy + BODIES), vec![first + 1, second + 1]);
        assert_eq!(items(&e, copy + PHANTOMS), vec![object + 2]);
        assert_eq!(
            items(&e, copy + CONSTRAINTS),
            vec![constraint_a + 0x100, constraint_b + 0x100]
        );
        assert_eq!(items(&e, copy + ACTIONS), vec![0, 0x7777]);
        // Entity B is the copy of the second entity unless the constraint is to the world.
        let clones = calls_to(&e, CONSTRAINT_CLONE);
        assert_eq!(
            clones,
            vec![
                vec![constraint_a, second + 1, first + 1, 0],
                vec![constraint_b, first + 1, 0, 0]
            ]
        );
        // The action got the entity copies (second, first) and the phantom copy.
        assert_eq!(
            *seen.borrow(),
            vec![vec![second + 1, first + 1], vec![object + 2]]
        );
        // The four temporary arrays are freed last-built first.
        let freed: Vec<u32> = frees(&e).into_iter().map(|(_, bytes)| bytes).collect();
        assert_eq!(freed, vec![4, 4, 8, 8]);
        // The new block came from the allocator (slot 4) with 0x44 bytes.
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
        assert_eq!(calls_to(&e, ALLOCATE)[0][1], 0x44);
    }
}
