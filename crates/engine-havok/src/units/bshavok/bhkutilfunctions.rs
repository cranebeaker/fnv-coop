//! `BSHavok/bhkutilfunctions.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The scene-graph walkers of this unit all look at a node the same way: the collision
//! object is at `node + 0x1c`, the dynamic cast `00653290` (`ECX` = object, one type word)
//! with the type at `012043f8` turns it into the Havok wrapper holder whose `+0x10` is the
//! wrapper (its Havok body is at `+8` and its constraint list at `+0x14`); the children come
//! from virtual slot `+0xC` of the node, as an object with the child array at `+0xA0` and
//! the child count (16 bits) at `+0xA6`.
//!
//! The arrays of this unit (`BSSimpleArray`, vtable `010c53b4`, layout: data `+4`, count `+8`,
//! capacity `+0xC`) allocate through virtual slot `+4` `(count)`, free through slot `+8`
//! `(block)` and reallocate through slot `+0xC` `(block, count)`.
//!
//! The functions that allocate Havok objects raise the per-thread allocation word at
//! `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
//! exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

use super::bhkconstraint::{enter_scope, leave_scope};
use super::bhkconvextransformshape::{copy_bytes, mulps_lane};
use super::bhklimitedhingeconstraint::{ALLOCATE, MEMORY_ROUTER, POOL_ALLOCATE, POOL_OBJECT};
#[allow(unused_imports)]
use crate::prelude::*;

/// Dynamic cast (`00653290`, `ECX` = the object, one type word; answers the object or 0), its
/// null-safe cdecl form `(type, object)` (`00653270`) and the cdecl test `(type, object)`
/// (`0043b300`, answers in `AL`).
const CAST: u32 = 0x0065_3290;
const CAST_CDECL: u32 = 0x0065_3270;
const CAST_TEST: u32 = 0x0043_b300;
/// The type words: collision object holder, constraint, ray-testable shape, and the
/// types `00c7fcc0` and `00c80090` test.
const TYPE_HOLDER: u32 = 0x0120_43f8;
const TYPE_CONSTRAINT: u32 = 0x0126_818c;
const TYPE_RAY_SHAPE: u32 = 0x0126_832c;
const TYPE_SCALED_HOLDER: u32 = 0x0126_7e64;
const TYPE_TRANSFORMED: u32 = 0x0126_83c8;
const TYPE_ENTITY_HOLDER: u32 = 0x0126_81c0;
/// The element copy of the array (`00c7f900`, `stdcall` `(destination, source, count)`; the
/// array the callers put in `ECX` is not read; 8-byte elements).
const COPY_ELEMENTS: u32 = 0x00c7_f900;
/// The stiff spring constraint: base constructor (`00c87c30`), `bhkConstraint::Init`
/// (`00c89dd0`), destructor (`00cc58c0`), sized free (`00aa1460`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_INIT: u32 = 0x00c8_9dd0;
const STIFF_SPRING_DESTRUCT: u32 = 0x00cc_58c0;
const SIZED_FREE: u32 = 0x00aa_1460;
/// The vtables of `bhkConstraint` and `bhkStiffSpringConstraint`, with their live counts.
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const VTABLE_STIFF_SPRING: u32 = 0x010c_52bc;
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const COUNT_STIFF_SPRING: u32 = 0x0126_85f4;
/// `bhkWorldObject::GetProperty` (`00c85ef0`, cdecl `(out, object, name)`, answers `out`),
/// the property name used by `GetNiAVObject`, and `bhkWorldObject::GetAVObject`
/// (`00c85c80`, `ECX` = the object, one word).
const GET_PROPERTY: u32 = 0x00c8_5ef0;
const PROPERTY_AV_OBJECT: u32 = 0x0126_7b70;
const GET_AV_OBJECT: u32 = 0x00c8_5c80;
/// The type of the `hkpEntity` activity query (`00c9bff0`, `ECX` = the entity, one out
/// pointer, answers a pointer to a byte).
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// The collision tracker query `00c43490` (cdecl, one word) used by `00c80090`.
const TRACKER_QUERY: u32 = 0x00c4_3490;
/// Windows imports: `EnterCriticalSection`, `LeaveCriticalSection`, `InterlockedIncrement`,
/// `InterlockedDecrement` and `GetCurrentThreadId` (slots in the import table).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// The critical section `DoThreadSetup` and `DoThreadShutdown` hold.
const THREAD_SECTION: u32 = 0x0126_7e48;
/// `hkMemoryRouter` constructor (`00c85770`, `ECX` = the 0x20-byte block, answers it),
/// `hkMemorySystem::getInstance` (`00cbbfc0`), `hkBaseSystem::initThread` (`00cbb930`, cdecl,
/// one word) and `hkBaseSystem::quitThread` (`00cbb990`).
const ROUTER_CONSTRUCT: u32 = 0x00c8_5770;
const MEMORY_SYSTEM: u32 = 0x00cb_bfc0;
const INIT_THREAD: u32 = 0x00cb_b930;
const QUIT_THREAD: u32 = 0x00cb_b990;
/// The text `DoThreadSetup` registers its router under.
const ROUTER_NAME: u32 = 0x010c_53c4;
/// The vtable of the arrays of this unit, and their pool object.
const VTABLE_ARRAY: u32 = 0x010c_53b4;
/// `hkpRigidBody::setLinearVelocity` (`005616d0`) and `setAngularVelocity` (`00561800`),
/// `ECX` = the body, one pointer to a 16-byte vector; and the no-argument `00a29680`.
const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
const NO_OP: u32 = 0x00a2_9680;
/// The zero vector the velocity damping uses when there is no body.
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// `00626cb0` (`ECX` = the wrapper, one word: the new collision filter word).
const SET_FILTER_INFO: u32 = 0x0062_6cb0;
/// The list assignment (`006e5cc0`, `ECX` = the list node, one pointer to a node whose first
/// word is the item) and the node destructor (`004ee840`, `ECX` = the node).
const LIST_ASSIGN: u32 = 0x006e_5cc0;
const LIST_NODE_DESTRUCT: u32 = 0x004e_e840;
/// `hkpResponseModifier::enableConstraint` (`00ce0020`) and `disableConstraint` (`00cdff60`),
/// both cdecl `(constraint, constraint data)`; and `005380d0` (`ECX` = a body id, one
/// pointer: the constraint wrapper).
const ENABLE_CONSTRAINT: u32 = 0x00ce_0020;
const DISABLE_CONSTRAINT: u32 = 0x00cd_ff60;
const ATTACH_BODY: u32 = 0x0053_80d0;
/// Array growth `00c90940` (cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `00c7f640` (`hkVector4::setTransformedPos`, `ECX` = destination, `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `NI2HK` (`0056df80`, cdecl `(destination, source)`), the hkTransform copy `00cb2d90`
/// (`ECX` = destination, one pointer) and `hkTransform::setMul` (`00cb2240`, `ECX` = the
/// product, `(first, second)`); the ray cast of the shape `00cab3c0` (`ECX` = the shape,
/// `(ray, transform, result)`); `008cffa0` (`ECX` = the wrapper, answers in `AL`).
const NI_TO_HAVOK: u32 = 0x0056_df80;
const TRANSFORM_COPY: u32 = 0x00cb_2d90;
const TRANSFORM_MUL: u32 = 0x00cb_2240;
const RAY_CAST: u32 = 0x00ca_b3c0;
const IS_SCALED: u32 = 0x008c_ffa0;
const QUERY_SHAPE: u32 = 0x004a_e6a0;
/// `hkpMalleableConstraintData::getWrappedConstraintData` (`00cdf6c0`, `ECX` = the data),
/// `hkStiffSpringConstraintCinfo::CreateData` (`00cc5ae0`, `ECX` = the cinfo),
/// `hkConstraintCinfo::SetData` (`00c89ac0`, `ECX` = the cinfo, one word), the smart pointer
/// release (`0045cec0`, `ECX` = the slot).
const WRAPPED_CONSTRAINT_DATA: u32 = 0x00cd_f6c0;
const CINFO_CREATE_DATA: u32 = 0x00cc_5ae0;
const CINFO_SET_DATA: u32 = 0x00c8_9ac0;
const RELEASE_SLOT: u32 = 0x0045_cec0;
/// The vtables of `hkStiffSpringConstraintCinfo` and `hkConstraintCinfo`.
const VTABLE_STIFF_SPRING_CINFO: u32 = 0x010c_5398;
const VTABLE_CINFO: u32 = 0x0108_ffb4;
/// The stiff spring rest length used when two pivots coincide (a float).
const MINIMUM_LENGTH: u32 = 0x0101_7d00;
/// Capsule and rigid body creation of `CreateTree`: `bhkCapsuleShape` constructor
/// (`0066d6a0`), `bhkRigidBodyCinfo` constructor (`00c8f510`, `ECX` = the block),
/// `bhkRigidBody` constructor (`0056d380`), the float constants.
const CAPSULE_CONSTRUCT: u32 = 0x0066_d6a0;
const RIGID_BODY_CINFO_CONSTRUCT: u32 = 0x00c8_f510;
const RIGID_BODY_CONSTRUCT: u32 = 0x0056_d380;
const HALF: u32 = 0x0126_7e44;
const TREE_FLOAT: u32 = 0x0101_8180;
const TREE_DOUBLE: u32 = 0x0101_2070;
/// `PrepareDebugGeom`: the colour hook (`00c84cd0`, cdecl `(word, colour)`), the flag byte
/// that enables it, `NiMaterialProperty` constructor (`00a75650`, `ECX` = the block),
/// `NiAVObject::AttachProperty` (`00439410`, `ECX` = the object, one word) and the
/// optional callback pointer at `01267b64` (cdecl `(object, colour)`).
const COLOUR_HOOK: u32 = 0x00c8_4cd0;
const COLOUR_HOOK_FLAG: u32 = 0x0126_80fc;
const MATERIAL_CONSTRUCT: u32 = 0x00a7_5650;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const COLOUR_CALLBACK: u32 = 0x0126_7b64;

// ---- float helpers -------------------------------------------------------------------------

/// One lane of `ADDSS` / `SUBPS`: a NaN in the destination operand wins (quieted), then a NaN
/// in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: impl Fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

/// `SQRTSS` on the low lane: a NaN comes out quiet, a negative number (not -0) gives the
/// default NaN.
fn sqrtss(bits: u32) -> u32 {
    let value = f32::from_bits(bits);
    if value.is_nan() {
        bits | 0x0040_0000
    } else if value < 0.0 {
        0xffc0_0000
    } else {
        value.sqrt().to_bits()
    }
}

/// The wrapper of `node`'s collision object (the holder's `+0x10`), 0 when any link is missing.
fn wrapper_of(e: &mut Engine, node: u32) -> u32 {
    let tracker = e.mem.u32(node + 0x1c);
    if tracker == 0 {
        return 0;
    }
    let holder = e.call(CAST, &args![tracker, TYPE_HOLDER]).u32();
    if holder == 0 {
        return 0;
    }
    e.mem.u32(holder + 0x10)
}

/// The child count of a children object.
fn child_count(e: &Engine, children: u32) -> u32 {
    u32::from(e.mem.u16(children + 0xa6))
}

/// The child at `index`.
fn child_at(e: &Engine, children: u32, index: u32) -> u32 {
    let array = e.mem.u32(children + 0xa0);
    e.mem.u32(array + index * 4)
}

// Translated from 00c7f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkutilfunctions.obj`). `ECX` = an array object (data `+4`,
/// capacity `+0xC`), stack `(new_capacity, count)`. Without data: allocates `new_capacity`
/// elements and takes it as the capacity. With the capacity equal to `count`: reallocates the
/// block. Otherwise allocates a new block of `new_capacity`, copies `count` elements over
/// (`00c7f900`), frees the old block and keeps the old capacity word.
pub fn fn_00c7f970(e: &mut Engine, this: Ptr, new_capacity: u32, count: u32) {
    let me = this.addr();
    let data = e.mem.u32(me + 4);
    if data == 0 {
        let block = e.vcall(me, 4, &args![new_capacity]).u32();
        e.mem.set_u32(me + 0xc, new_capacity);
        e.mem.set_u32(me + 4, block);
        return;
    }
    if count == e.mem.u32(me + 0xc) {
        let block = e.vcall(me, 0xc, &args![data, new_capacity]).u32();
        e.mem.set_u32(me + 4, block);
        return;
    }
    let block = e.vcall(me, 4, &args![new_capacity]).u32();
    // 00c7f9ae: `00c7f900` is stdcall (RET 0xc) and does not read ECX.
    e.call(COPY_ELEMENTS, &args![block, data, count]);
    let old = e.mem.u32(me + 4);
    e.vcall(me, 8, &args![old]);
    e.mem.set_u32(me + 4, block);
}

// Translated from 00c7f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7f9e0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. `ECX` = the block, stack `(cinfo)`: the `bhkStiffSpringConstraint` constructor.
/// Runs the `bhkRefObject` constructor (`00c87c30`), clears `+0xC`, sets the `bhkConstraint`
/// vtable and counts it (`01268188`), sets the stiff spring vtable, then `bhkConstraint::Init`
/// (`00c89dd0`) with the cinfo's `+4` field (0 for no cinfo), and counts it (`012685f4`).
/// Answers the block.
pub fn fn_00c7f9e0(e: &mut Engine, this: Ptr, cinfo: u32) -> u32 {
    let me = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me, VTABLE_CONSTRAINT);
    let count = e.global::<u32>(COUNT_CONSTRAINT);
    e.set_global(COUNT_CONSTRAINT, count.wrapping_add(1));
    e.mem.set_u32(me, VTABLE_STIFF_SPRING);
    let field = if cinfo == 0 { 0 } else { cinfo + 4 };
    e.call(CONSTRAINT_INIT, &args![this, field]);
    let count = e.global::<u32>(COUNT_STIFF_SPRING);
    e.set_global(COUNT_STIFF_SPRING, count.wrapping_add(1));
    me
}

// Translated from 00c7fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkStiffSpringConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85f8.
pub fn bhk_stiff_spring_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85f8
}

// Translated from 00c7fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkStiffSpringConstraint::scalar deleting destructor` (Xbox PDB), `ECX` = the object,
/// stack `(flags)`: runs the destructor (`00cc58c0`), and with bit 0 of the flags frees the
/// 0x10 bytes (`00aa1460`, cdecl `(block, size)`). Answers the object.
pub fn bhk_stiff_spring_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    e.call(STIFF_SPRING_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x10u32]);
    }
    this.addr()
}

// Translated from 00c7fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUtilFunctions::GetNiAVObject` (Xbox PDB), stack `(object)`: by the kind byte at
/// `+0x18`. Kind 1: the world object at `object + (signed byte at +0x10)` (when not 0), its
/// property `01267b70` (`00c85ef0`) and, when set, that property's `+8`. Kind 2: the same, and
/// without the property the world object's `+0xC` object's `GetAVObject(0)` (`00c85c80`).
/// Otherwise 0.
pub fn bhk_util_functions_get_ni_a_v_object(e: &mut Engine, object: Ptr) -> u32 {
    let me = object.addr();
    let kind = e.mem.u8(me + 0x18);
    let world = me.wrapping_add(e.mem.u8(me + 0x10) as i8 as u32);
    if kind == 1 {
        if world == 0 {
            return 0;
        }
        let property = property_of(e, world);
        if property == 0 {
            return 0;
        }
        return e.mem.u32(property + 8);
    }
    if kind == 2 {
        if world == 0 {
            return 0;
        }
        let property = property_of(e, world);
        if property != 0 {
            return e.mem.u32(property + 8);
        }
        let other = e.mem.u32(world + 0xc);
        if other != 0 {
            return e.call(GET_AV_OBJECT, &args![other, 0u32]).u32();
        }
    }
    0
}

/// The value `GetProperty` (`00c85ef0`) leaves in the slot it is given: the first word of the
/// pointer it answers.
fn property_of(e: &mut Engine, world: u32) -> u32 {
    e.with_stack(8, |e, out| {
        let slot = e
            .call(GET_PROPERTY, &args![out, world, PROPERTY_AV_OBJECT])
            .u32();
        e.mem.u32(slot)
    })
}

// Translated from 00c7fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = the block: sets the vtable `010c5398` (the stiff spring cinfo's) with
/// `+4` = 0, `+8` = 1, `+0xC` = 0 and `+0x10` = 0. Answers the block.
pub fn fn_00c7fb20(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, 1);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me, VTABLE_STIFF_SPRING_CINFO);
    me
}

// Translated from 00c7fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, stack `(object)`: follows a chain of objects with an array of 0x30-byte
/// entries (array at `+0xAC`, 16-bit count at `+0xB0`). Each round: when every entry's
/// object (the entry's `+0xC`, virtual slot `+0x20`) answers 0xB, the current object is the
/// result; otherwise it finds the first entry that is neither kind 0xB nor a kind 0xC whose
/// flag byte (the 16-bit index at the object's `+0x1C`, in the table at `[entry + 0x30] +
/// 0x1C`) is set, and moves on to that entry's `+0x14` object (its `+0x18` object when that
/// is the current one). Answers 0 when the chain ends, the current object when no such
/// entry is found.
pub fn fn_00c7fb50(e: &mut Engine, object: Ptr) -> u32 {
    let mut current = object.addr();
    if current == 0 {
        return 0;
    }
    loop {
        let count = i32::from(e.mem.u16(current + 0xb0));
        if count == 0 {
            return current;
        }
        let mut all_eleven = true;
        let mut index = 0;
        let mut offset = 0u32;
        while all_eleven {
            let entries = e.mem.u32(current + 0xac);
            let entry = e.mem.u32(entries + offset);
            let inner = e.mem.u32(entry + 0xc);
            let kind = e.vcall(inner, 0x20, &args![]).u32();
            let total = i32::from(e.mem.u16(current + 0xb0));
            all_eleven = kind == 0xb;
            index += 1;
            offset += 0x30;
            if index >= total {
                break;
            }
        }
        if all_eleven {
            return current;
        }
        let mut index = 0;
        let mut offset = 0u32;
        let entry = loop {
            let total = i32::from(e.mem.u16(current + 0xb0));
            if index >= total {
                return current;
            }
            let entries = e.mem.u32(current + 0xac);
            let entry = e.mem.u32(entries + offset);
            let inner = e.mem.u32(entry + 0xc);
            if e.vcall(inner, 0x20, &args![]).u32() != 0xb {
                let kind = e.vcall(inner, 0x20, &args![]).u32();
                if kind != 0xc {
                    break entry;
                }
                let flag_index = u32::from(e.mem.u16(inner + 0x1c));
                let table = e.mem.u32(e.mem.u32(entry + 0x30) + 0x1c);
                if e.mem.u8(flag_index + table) == 0 {
                    break entry;
                }
            }
            index += 1;
            offset += 0x30;
        };
        let first = e.mem.u32(entry + 0x14);
        let next = if first == current {
            e.mem.u32(entry + 0x18)
        } else {
            first
        };
        current = next;
        if current == 0 {
            return 0;
        }
    }
}

// Translated from 00c7fc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<struct bhkUtilFunctions::AVLevel>::_Allocate` (Xbox PDB), stack `(count)`:
/// the pool allocator (`00aa3e40` on the pool object `011f6238`) for `count` 8-byte elements.
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn bs_simple_array_av_level_allocate(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(POOL_ALLOCATE, &args![POOL_OBJECT, count.wrapping_mul(8)])
        .u32()
}

// Translated from 00c7fc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<struct bhkUtilFunctions::AVLevel>::_Reallocate` (Xbox PDB), stack `(block,
/// count)`: the pool reallocation (`00aa4150` on the pool object, `(block, bytes)`) for `count`
/// 8-byte elements.
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn bs_simple_array_av_level_reallocate(
    e: &mut Engine,
    _this: Ptr,
    block: u32,
    count: u32,
) -> u32 {
    e.call(
        POOL_REALLOCATE,
        &args![POOL_OBJECT, block, count.wrapping_mul(8)],
    )
    .u32()
}

/// The pool reallocation `00aa4150` (`ECX` = the pool object, `(block, bytes)`).
const POOL_REALLOCATE: u32 = 0x00aa_4150;

// Translated from 00c7fc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = an array object: makes room for one more element and answers its
/// index (the old count). A full array gets 4 elements first (without data), or twice its
/// capacity (`+ 0x400` once above 0x400) through `00c7f970`.
pub fn fn_00c7fc60(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    let count = e.mem.u32(me + 8);
    let capacity = e.mem.u32(me + 0xc);
    if count == capacity {
        if capacity == 0 {
            let block = e.vcall(me, 4, &args![4u32]).u32();
            let used = e.mem.u32(me + 8);
            e.mem.set_u32(me + 8, used.wrapping_add(1));
            e.mem.set_u32(me + 4, block);
            e.mem.set_u32(me + 0xc, 4);
            return e.mem.u32(me + 8).wrapping_sub(1);
        }
        let bigger = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_mul(2)
        };
        fn_00c7f970(e, this, bigger, count);
        e.mem.set_u32(me + 0xc, bigger);
    }
    let used = e.mem.u32(me + 8);
    e.mem.set_u32(me + 8, used.wrapping_add(1));
    e.mem.u32(me + 8).wrapping_sub(1)
}

// Translated from 00c7fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7fcc0 - the /GS cookie check is not modeled, and when the scaled-holder test passes while 008cffa0 answers true and the scale is not 1.0 the 64-byte transform at stack +0x20 is never written in the exe (stack garbage, taken as zeros).
/// No map name, cdecl stack `(node, ray, best, flag)`; answers in `AL`. Ray test of a node and
/// its subtree against the shapes of the nodes' wrappers. For a node with a wrapper whose
/// Havok body's collision filter word (`[body + 0x10 + 0x1c]`) has kind (low 7 bits) 8 or
/// 0x1d and layer (bits 8..12) 1 (only without `flag`'s low byte) or 2..=16: with the shape
/// of the wrapper (`004ae6a0`, cast to the type `0126832c` by `00653270`) it takes the
/// wrapper's transform (virtual slot `+0xF4`, into a 64-byte local), replaces it by the node's
/// world transform (`NI2HK` of `node + 0x68`) combined with the wrapper's own when the holder
/// passes the tests at `01267e64` / `012683c8` (and is not scaled other than 1.0), and casts
/// the ray (`00cab3c0` on the shape with `(ray, transform, result)`); a hit replaces `best`
/// (pointer at `+0`, 32 result bytes at `+0x10`, distance float at `+0x2c`) when it has none
/// or the new distance (result `+0x1C`) is smaller. The children are visited the same way.
/// The `AL` the exe answers is the OR of the children's, and nothing ever sets it: always
/// false.
pub fn fn_00c7fcc0(e: &mut Engine, node: Ptr, ray: u32, best: Ptr, flag: u32) -> bool {
    let me = node.addr();
    if me == 0 {
        return false;
    }
    let tracker = e.mem.u32(me + 0x1c);
    if tracker != 0 {
        let holder = e.call(CAST, &args![tracker, TYPE_HOLDER]).u32();
        if holder != 0 {
            let wrapper = e.mem.u32(holder + 0x10);
            if wrapper != 0 {
                e.with_stack(0x100, |e, frame| {
                    ray_test(e, me, ray, best.addr(), flag, holder, wrapper, frame.addr());
                });
            }
        }
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    let mut result = false;
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            result |= fn_00c7fcc0(e, Ptr::new(child), ray, best, flag);
            index += 1;
        }
    }
    result
}

/// The ray test of one wrapper for [`fn_00c7fcc0`]; `frame` is the 0x100-byte local area
/// (transform at `+0x20`, result at `+0x60`, copies at `+0x80` and `+0xC0`).
#[allow(clippy::too_many_arguments)]
fn ray_test(
    e: &mut Engine,
    node: u32,
    ray: u32,
    best: u32,
    flag: u32,
    holder: u32,
    wrapper: u32,
    frame: u32,
) {
    let body = e.mem.u32(wrapper + 8);
    let filter = if body != 0 && body.wrapping_add(0x10) != 0 {
        e.mem.u32(body.wrapping_add(0x10) + 0x1c)
    } else {
        0
    };
    let kind = filter & 0x7f;
    if kind != 8 && kind != 0x1d {
        return;
    }
    let layer = (filter >> 8) & 0x1f;
    if layer == 1 {
        if flag as u8 != 0 {
            return;
        }
    } else if layer.wrapping_sub(2) > 0xe {
        return;
    }
    let query = e.call(QUERY_SHAPE, &args![wrapper]).u32();
    if query == 0 {
        return;
    }
    let shape = e.call(CAST_CDECL, &args![TYPE_RAY_SHAPE, query]).u32();
    if shape == 0 {
        return;
    }
    e.vcall(wrapper, 0xf4, &args![frame + 0x20]);
    if e.call(CAST_TEST, &args![TYPE_SCALED_HOLDER, holder]).bool() {
        let scaled = e.call(IS_SCALED, &args![wrapper]).bool();
        let scale = e.mem.u32(holder + 0x18);
        if !scaled || scale == 1.0f32.to_bits() {
            e.call(NI_TO_HAVOK, &args![frame + 0x20, node + 0x68]);
            if e.call(CAST_TEST, &args![TYPE_TRANSFORMED, wrapper]).bool() {
                copy_bytes(e, frame + 0x80, frame + 0x20, 0x40);
                e.call(TRANSFORM_COPY, &args![frame + 0xc0, wrapper + 0x20]);
                copy_bytes(e, frame + 0xf0, wrapper + 0x30, 0x10);
                e.call(
                    TRANSFORM_MUL,
                    &args![frame + 0x20, frame + 0x80, frame + 0xc0],
                );
            }
        }
    } else {
        e.vcall(wrapper, 0xf4, &args![frame + 0x20]);
    }
    if !e
        .call(RAY_CAST, &args![shape, ray, frame + 0x20, frame + 0x60])
        .bool()
    {
        return;
    }
    if e.mem.u32(best) != 0 {
        let current = f32::from_bits(x87_quiet_bits(e.mem.u32(best + 0x2c)));
        let new = f32::from_bits(x87_quiet_bits(e.mem.u32(frame + 0x7c)));
        // FCOMPP leaves the update to "new < current"; unordered and equal skip it.
        if new.partial_cmp(&current) != Some(std::cmp::Ordering::Less) {
            return;
        }
    }
    e.mem.set_u32(best, wrapper);
    copy_bytes(e, best + 0x10, frame + 0x60, 0x10);
    copy_bytes(e, best + 0x20, frame + 0x70, 0x10);
}

// Translated from 00c7ff40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, stack `(object, layer)`: finds the first wrapper in the subtree whose Havok
/// body's collision filter word `(word >> 8) & 0x1f` equals `layer`. Checks the node's own
/// wrapper holder first (`[node + 0x1c]` cast to the type `012043f8`, its `+0x10` object,
/// that object's body `+8`, the word at `body + 0x10 + 0x1c`), then the children in turn.
/// Answers the wrapper or 0.
pub fn fn_00c7ff40(e: &mut Engine, object: Ptr, layer: u32) -> u32 {
    let me = object.addr();
    if me == 0 {
        return 0;
    }
    if e.mem.u32(me + 0x1c) != 0 {
        let holder = e
            .call(CAST, &args![e.mem.u32(me + 0x1c), TYPE_HOLDER])
            .u32();
        if holder != 0 {
            let wrapper = e.mem.u32(holder + 0x10);
            if wrapper != 0 {
                let body = e.mem.u32(wrapper + 8);
                let filter = if body == 0 || body.wrapping_add(0x10) == 0 {
                    0
                } else {
                    e.mem.u32(body.wrapping_add(0x10) + 0x1c)
                };
                if (filter >> 8) & 0x1f == layer {
                    return wrapper;
                }
            }
        }
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    let mut found = 0;
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            found = fn_00c7ff40(e, Ptr::new(child), layer);
            index += 1;
            if found != 0 {
                break;
            }
        }
    }
    found
}

// Translated from 00c7ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkutilfunctions.obj`). cdecl, one
/// word. Takes the object `[object + 0xc]` (or `object` itself when that is
/// null), then the object `[holder]`. When the word at `+0xc` of that
/// object is 8 or 10, its virtual slot `+0x10` is called and, unless the
/// result is 0 or 0x10, the word at `result - 8` is returned; when it is 9
/// the word at `+8` is returned. Otherwise 0.
pub fn fn_00c7ffd0(e: &mut Engine, object: Ptr) -> u32 {
    let mut holder = e.mem.u32(object.addr() + 0xc);
    if holder == 0 {
        holder = object.addr();
    }
    if holder == 0 {
        return 0;
    }
    let inner = e.mem.u32(holder);
    let kind = e.mem.u32(inner + 0xc);
    if kind == 8 || kind == 10 {
        let result = e.vcall(inner, 0x10, &args![]).u32();
        if result == 0 || result == 0x10 {
            return 0;
        }
        return e.mem.u32(result.wrapping_sub(8));
    }
    if inner != 0 && kind == 9 {
        return e.mem.u32(inner + 8);
    }
    0
}

// Translated from 00c80020 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, stack `(object)`: takes the owner the way [`fn_00c7ffd0`] does (the exe has
/// that code inline) and, when there is one, answers its virtual slot `+0xE0` called with the
/// object's `+4`; 0x24 without an owner.
pub fn fn_00c80020(e: &mut Engine, object: Ptr) -> u32 {
    let owner = fn_00c7ffd0(e, object);
    if owner == 0 {
        return 0x24;
    }
    let argument = e.mem.u32(object.addr() + 4);
    e.vcall(owner, 0xe0, &args![argument]).u32()
}

// Translated from 00c80090 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, stack `(object)`; answers in `AL`: whether the object's entity is active. The
/// tracker is the object's `+0x1C` (virtual slot `+0x38`); without a result, when the tracker
/// query `00c43490` answers a record with bit 1 of its `+0xC`, the first child's tracker is
/// tried the same way. The result's `+0x10` is cast to the type `012681c0`; its `+8` body is
/// asked `isActive` (`00c9bff0`, an out pointer holding the object) and the byte it points
/// at decides.
pub fn fn_00c80090(e: &mut Engine, object: Ptr) -> bool {
    let me = object.addr();
    let tracker = e.mem.u32(me + 0x1c);
    let mut found = 0;
    if tracker != 0 {
        found = e.vcall(tracker, 0x38, &args![]).u32();
    }
    if found == 0 {
        let record = e.call(TRACKER_QUERY, &args![me]).u32();
        if record == 0 || e.mem.u32(record + 0xc) & 2 == 0 {
            return false;
        }
        let first = if e.mem.u16(me + 0xa6) == 0 {
            0
        } else {
            e.mem.u32(e.mem.u32(me + 0xa0))
        };
        let inner = e.mem.u32(first + 0x1c);
        if inner == 0 {
            return false;
        }
        found = e.vcall(inner, 0x38, &args![]).u32();
        if found == 0 {
            return false;
        }
    }
    let cast_from = e.mem.u32(found + 0x10);
    if cast_from == 0 {
        return false;
    }
    let holder = e.call(CAST, &args![cast_from, TYPE_ENTITY_HOLDER]).u32();
    if holder == 0 {
        return false;
    }
    let body = e.mem.u32(holder + 8);
    if body == 0 {
        return false;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), me);
        let flag = e.call(ENTITY_IS_ACTIVE, &args![body, out]).u32();
        e.mem.u8(flag) != 0
    })
}

// Translated from 00c80130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c80130 - the FS:[0] exception-unwinding frame is not modeled.
/// `DoThreadSetup` (Xbox PDB), stack `(slot)`: in the allocation scope, when `[slot]` is 0,
/// under the critical section `01267e48`: builds a 0x20-byte `hkMemoryRouter` (`00c85770`;
/// the exe also builds one in a 0x20-byte local that is never used again) in a 0x24-byte
/// pool block, registers it with the memory system (`00cbbfc0` object, virtual slot `+0xC`
/// `(router, 010c53c4, 1)`) and initialises the thread (`00cbb930`). The router's `+0xC` and
/// `+0x18` are set to its allocator (the word at its `+0x10`), 0x5000 bytes are allocated
/// through that allocator's virtual slot `+4`, and the block goes to `[slot + 4]`, to the
/// router's `+4` and `+0`, and (plus 0x5000) to its `+8`; the router becomes `[slot]`.
pub fn do_thread_setup(e: &mut Engine, slot: Ptr) {
    let old = enter_scope(e);
    let me = slot.addr();
    if e.mem.u32(me) == 0 {
        e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![THREAD_SECTION]);
        e.with_stack(0x20, |e, temporary| {
            e.call(ROUTER_CONSTRUCT, &args![temporary]);
        });
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 0x24u32]).u32();
        let mut router = 0;
        if block != 0 {
            router = e.call(ROUTER_CONSTRUCT, &args![block]).u32();
        }
        let system = e.call(MEMORY_SYSTEM, &args![]).u32();
        e.vcall(system, 0xc, &args![router, ROUTER_NAME, 1u32]);
        e.call(INIT_THREAD, &args![router]);
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(router + 0xc, allocator);
        e.mem.set_u32(router + 0x18, allocator);
        let memory = e.vcall(allocator, 4, &args![0x5000u32]).u32();
        e.mem.set_u32(me + 4, memory);
        e.mem.set_u32(router + 4, memory);
        e.mem.set_u32(router, memory);
        e.mem.set_u32(router + 8, memory.wrapping_add(0x5000));
        e.mem.set_u32(me, router);
        e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![THREAD_SECTION]);
    }
    leave_scope(e, old);
}

// Translated from 00c80240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoThreadShutdown` (Xbox PDB), stack `(slot)`: when `[slot]` is not 0, under the critical
/// section `01267e48`: frees the 0x5000 bytes at `[slot + 4]` through the allocator at the
/// router's `+0x10` (virtual slot `+8`, `(block, 0x5000)`), clears `[slot + 4]`, quits the
/// thread (`00cbb990`) when `GetCurrentThreadId` equals `[slot + 8]`, tells the memory system
/// (virtual slot `+0x10` `(router, 1)`), clears the router's `+4`, `+0` and `+8` and `[slot]`.
pub fn do_thread_shutdown(e: &mut Engine, slot: Ptr) {
    let me = slot.addr();
    let router = e.mem.u32(me);
    if router == 0 {
        return;
    }
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![THREAD_SECTION]);
    let allocator = e.mem.u32(router + 0x10);
    let block = e.mem.u32(me + 4);
    e.vcall(allocator, 8, &args![block, 0x5000u32]);
    e.mem.set_u32(me + 4, 0);
    if e.call(IMPORT_GET_CURRENT_THREAD_ID, &args![]).u32() == e.mem.u32(me + 8) {
        e.call(QUIT_THREAD, &args![]);
    }
    let system = e.call(MEMORY_SYSTEM, &args![]).u32();
    e.vcall(system, 0x10, &args![router, 1u32]);
    e.mem.set_u32(router + 4, 0);
    e.mem.set_u32(router, 0);
    e.mem.set_u32(router + 8, 0);
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![THREAD_SECTION]);
    e.mem.set_u32(me, 0);
}

// Translated from 00c802d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUtilFunctions::FindNextCollisionObject` (Xbox PDB), stack `(object)`: the object's own
/// `+0x1C` collision object when it has one; otherwise the first non-zero answer for its
/// children (checked while the answer is still 0), or 0.
pub fn bhk_util_functions_find_next_collision_object(e: &mut Engine, object: Ptr) -> u32 {
    let me = object.addr();
    let mut found = e.mem.u32(me + 0x1c);
    if found != 0 {
        return found;
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return found;
    }
    let mut index = 0;
    while index < child_count(e, children) && found == 0 {
        let child = child_at(e, children, index);
        if child != 0 {
            found = bhk_util_functions_find_next_collision_object(e, Ptr::new(child));
        }
        index += 1;
    }
    found
}

// Translated from 00c80330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUtilFunctions::SetCOUseVel` (Xbox PDB), stack `(flag, object)`: sets (low byte of the
/// flag non-zero) or clears bit 5 of the 16-bit word at the collision object's `+0xC` for
/// the object and, recursively with the whole flag word, for every non-null child.
pub fn bhk_util_functions_set_c_o_use_vel(e: &mut Engine, flag: u32, object: Ptr) {
    let me = object.addr();
    let tracker = e.mem.u32(me + 0x1c);
    if tracker != 0 {
        let word = e.mem.u16(tracker + 0xc);
        e.mem.set_u16(
            tracker + 0xc,
            if flag as u8 != 0 {
                word | 0x20
            } else {
                word & 0xffdf
            },
        );
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0;
    while index < child_count(e, children) {
        let child = child_at(e, children, index);
        if child != 0 {
            bhk_util_functions_set_c_o_use_vel(e, flag, Ptr::new(child));
        }
        index += 1;
    }
}

// Translated from 00c803a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c803a0 - the /GS cookie check is not modeled.
/// `bhkUtilFunctions::DampBodiesVelocity` (Xbox PDB), stack `(object, scale)`: for the object's
/// wrapper body (`[holder + 0x10] + 8`; the zero vector `01267e30` stands in for a missing
/// body) multiplies the 16-byte vectors at `+0x1B0` and `+0x1C0` by the scale in every lane
/// (`MULPS`), and, when there is a body, hands them to `setLinearVelocity` (`005616d0`) and
/// `setAngularVelocity` (`00561800`) between calls of `00a29680` on the wrapper. Then the
/// same for every child (no null check), with the scale passed through `FLD`/`FSTP`.
pub fn bhk_util_functions_damp_bodies_velocity(e: &mut Engine, object: Ptr, scale: f32) {
    let me = object.addr();
    if me == 0 {
        return;
    }
    let scale_bits = scale.to_bits();
    let wrapper = wrapper_of(e, me);
    if wrapper != 0 {
        let body = e.mem.u32(wrapper + 8);
        let linear = if body != 0 { body + 0x1b0 } else { ZERO_VECTOR };
        let angular = if body != 0 { body + 0x1c0 } else { ZERO_VECTOR };
        e.with_stack(0x30, |e, frame| {
            let frame = frame.addr();
            for lane in 0..4 {
                let first = mulps_lane(scale_bits, e.mem.u32(linear + lane * 4));
                e.mem.set_u32(frame + 0x10 + lane * 4, first);
                let second = mulps_lane(scale_bits, e.mem.u32(angular + lane * 4));
                e.mem.set_u32(frame + 0x20 + lane * 4, second);
            }
            if body != 0 {
                e.call(NO_OP, &args![wrapper]);
                e.call(SET_LINEAR_VELOCITY, &args![body, frame + 0x10]);
                e.call(NO_OP, &args![wrapper]);
            }
            let body = e.mem.u32(wrapper + 8);
            if body != 0 {
                e.call(NO_OP, &args![wrapper]);
                e.call(SET_ANGULAR_VELOCITY, &args![body, frame + 0x20]);
                e.call(NO_OP, &args![wrapper]);
            }
        });
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0;
    while index < child_count(e, children) {
        let child = child_at(e, children, index);
        let passed = f32::from_bits(x87_quiet_bits(scale_bits));
        bhk_util_functions_damp_bodies_velocity(e, Ptr::new(child), passed);
        index += 1;
    }
}

// Translated from 00c804d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, stack `(object, value)`: takes the collision filter word of the object's
/// wrapper body (the word at `body + 0x10 + 0x1c`, 0 without a body), replaces its low 7 bits
/// with those of `value` and hands the result to `00626cb0` on the wrapper; then does the
/// same for every child (no null check).
pub fn fn_00c804d0(e: &mut Engine, object: Ptr, value: u32) {
    let me = object.addr();
    if me == 0 {
        return;
    }
    let tracker = e.mem.u32(me + 0x1c);
    if tracker != 0 {
        let holder = e.call(CAST, &args![tracker, TYPE_HOLDER]).u32();
        if holder != 0 {
            let wrapper = e.mem.u32(holder + 0x10);
            if wrapper != 0 {
                let body = e.mem.u32(wrapper + 8);
                let current = if body != 0 && body.wrapping_add(0x10) != 0 {
                    e.mem.u32(body.wrapping_add(0x10) + 0x1c)
                } else {
                    0
                };
                let word = ((current ^ value) & 0x7f) ^ current;
                e.call(SET_FILTER_INFO, &args![wrapper, word]);
            }
        }
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0;
    while index < child_count(e, children) {
        let child = child_at(e, children, index);
        fn_00c804d0(e, Ptr::new(child), value);
        index += 1;
    }
}

// Translated from 00c80570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = a list node (item `+0`, next `+4`), stack `(slot)` (the item to remove
/// is `[slot]`): does nothing for a null item or an empty list. Finds the first node holding
/// the item. The list's own first node gives its place up by taking over the second node's
/// item (`006e5cc0`), unlinking and destroying that node (`004ee840`, then freed to the pool
/// with `00aa4060`), or, when it is the only node, releases the item (`InterlockedDecrement` of
/// its count at `+4`, the virtual destructor slot `+4` at zero) and clears the node's item. Any
/// later node is unlinked from the one before it, destroyed and freed.
pub fn fn_00c80570(e: &mut Engine, list: Ptr, slot: Ptr) {
    let me = list.addr();
    let item = e.mem.u32(slot.addr());
    if item == 0 {
        return;
    }
    let next = e.mem.u32(me + 4);
    if next == 0 && e.mem.u32(me) == 0 {
        return;
    }
    let mut node = me;
    let mut previous = me;
    while e.mem.u32(node) != item {
        previous = node;
        node = e.mem.u32(node + 4);
        if node == 0 {
            return;
        }
    }
    if node == me {
        if next != 0 {
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(me + 4, after);
            e.call(LIST_ASSIGN, &args![me, next]);
            e.mem.set_u32(next + 4, 0);
            e.call(LIST_NODE_DESTRUCT, &args![next]);
            e.call(POOL_FREE, &args![POOL_OBJECT, next]);
            return;
        }
        let held = e.mem.u32(me);
        if held == 0 {
            return;
        }
        if e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held + 4]).u32() == 0 {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(me, 0);
        return;
    }
    let after = e.mem.u32(node + 4);
    e.mem.set_u32(previous + 4, after);
    e.mem.set_u32(node + 4, 0);
    e.call(LIST_NODE_DESTRUCT, &args![node]);
    e.call(POOL_FREE, &args![POOL_OBJECT, node]);
}

/// The pool free (`00aa4060`, `ECX` = the pool object, one pointer).
const POOL_FREE: u32 = 0x00aa_4060;

// Translated from 00c80630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = an array: sets the array vtable `010c53b4`; with data, frees it through
/// the table's slot `+8` (`010c53bc`) and clears data, capacity and count.
pub fn fn_00c80630(e: &mut Engine, this: Ptr) {
    release_array(e, this.addr());
}

/// The body of the array destructors: the vtable, then the data freed and the fields cleared.
fn release_array(e: &mut Engine, me: u32) {
    let data = e.mem.u32(me + 4);
    e.mem.set_u32(me, VTABLE_ARRAY);
    if data != 0 {
        e.vcall(me, 8, &args![data]);
        e.mem.set_u32(me + 4, 0);
        e.mem.set_u32(me + 0xc, 0);
        e.mem.set_u32(me + 8, 0);
    }
}

// Translated from 00c80660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<struct bhkUtilFunctions::AVLevel>::scalar deleting destructor` (Xbox PDB),
/// `ECX` = the array, stack `(flags)`: the array release of `00c80630`, and with bit 0 of the
/// flags the block is freed to the pool (`00aa4060`). Answers the array.
pub fn bs_simple_array_av_level_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    release_array(e, this.addr());
    if flags & 1 != 0 {
        e.call(POOL_FREE, &args![POOL_OBJECT, this]);
    }
    this.addr()
}

// Translated from 00c806b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c806b0 - the /GS cookie check is not modeled, and the 0x3c-byte result record is left as stack garbage except for its first word.
/// No map name, cdecl stack `(node, ray, flag)`: runs the ray test `00c7fcc0` with a local best
/// record whose first word is 0, and answers that word (the wrapper of the nearest hit).
pub fn fn_00c806b0(e: &mut Engine, node: Ptr, ray: u32, flag: u32) -> u32 {
    e.with_stack(0x3c, |e, best| {
        e.mem.set_u32(best.addr(), 0);
        fn_00c7fcc0(e, node, ray, best, flag);
        e.mem.u32(best.addr())
    })
}

// Translated from 00c80700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c80700 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name, stack `(node)`: walks the wrapper's constraint list (`+0x14` of the wrapper;
/// nodes item `+0`, next `+4`). For every item that is a constraint (the RTTI chain from
/// virtual slot `+8` reaches `0126818c`) by the kind of its Havok data object (virtual slot
/// `+0x20`): kinds 2, 7 and 13 take the body ids off the first body (virtual slot `+0xCC`,
/// that object's `+0xC`, virtual slot `+0x94`, `+0x350`) when the Havok constraint has no
/// owner (`[[+8] + 8]`) and give them to virtual slot `+0x9C`, then enable the constraint
/// (`00ce0020`); kind 8 hands the constraint to `005380d0` on that body id and removes it
/// from the list (`00c80570`, with the item counted up and down around it, and the virtual
/// destructor slot `+4` at zero). Then the same for the children.
pub fn fn_00c80700(e: &mut Engine, node: Ptr) {
    let me = node.addr();
    if me == 0 {
        return;
    }
    let wrapper = wrapper_of(e, me);
    if wrapper != 0 {
        let mut cell = wrapper + 0x14;
        while cell != 0 {
            let item = e.mem.u32(cell);
            if item == 0 {
                break;
            }
            constraint_step(e, cell, item);
            cell = e.mem.u32(cell + 4);
        }
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0;
    while index < child_count(e, children) {
        let child = child_at(e, children, index);
        fn_00c80700(e, Ptr::new(child));
        index += 1;
    }
}

/// The Havok data object of a constraint wrapper: the Havok constraint's `+0xC` object when
/// there is one (`+8`), else the first word of the object at `+0xC`, else 0.
fn constraint_data(e: &Engine, constraint: u32) -> u32 {
    let havok = e.mem.u32(constraint + 8);
    if havok != 0 {
        return e.mem.u32(havok + 0xc);
    }
    let other = e.mem.u32(constraint + 0xc);
    if other != 0 {
        e.mem.u32(other)
    } else {
        0
    }
}

/// The body id of a constraint's first body: virtual slot `+0xCC`, that object's `+0xC`,
/// virtual slot `+0x94`, the answer's `+0x350` (0 at any missing link, where the exe goes on
/// with a null `ECX`).
fn first_body_id(e: &mut Engine, constraint: u32) -> u32 {
    let first = e.vcall(constraint, 0xcc, &args![]).u32();
    let object = if first != 0 {
        e.mem.u32(first + 0xc)
    } else {
        0
    };
    let id_holder = e.vcall(object, 0x94, &args![]).u32();
    if id_holder != 0 {
        e.mem.u32(id_holder + 0x350)
    } else {
        0
    }
}

/// One item of the list walk of [`fn_00c80700`].
fn constraint_step(e: &mut Engine, cell: u32, item: u32) {
    let mut rtti = e.vcall(item, 8, &args![]).u32();
    let mut is_constraint = false;
    while rtti != 0 {
        if rtti == TYPE_CONSTRAINT {
            is_constraint = true;
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    if !is_constraint {
        return;
    }
    let data = constraint_data(e, item);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    match kind.wrapping_sub(2) {
        0 | 5 | 11 => {
            let havok = e.mem.u32(item + 8);
            if e.mem.u32(havok + 8) == 0 {
                let id = first_body_id(e, item);
                e.vcall(item, 0x9c, &args![id]);
            }
            let havok = e.mem.u32(item + 8);
            if e.mem.u32(havok + 8) != 0 {
                let inner = e.mem.u32(havok + 8);
                e.call(ENABLE_CONSTRAINT, &args![havok, inner]);
            }
        }
        6 => {
            let first = e.vcall(item, 0xcc, &args![]).u32();
            if first != 0 {
                let object = e.mem.u32(first + 0xc);
                if object != 0 {
                    let id_holder = e.vcall(object, 0x94, &args![]).u32();
                    let id = if id_holder != 0 {
                        e.mem.u32(id_holder + 0x350)
                    } else {
                        0
                    };
                    e.call(ATTACH_BODY, &args![id, item]);
                }
            }
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![item + 4]);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                fn_00c80570(e, Ptr::new(cell), slot);
            });
            if e.call(IMPORT_INTERLOCKED_DECREMENT, &args![item + 4]).u32() == 0 {
                e.vcall(item, 4, &args![]);
            }
        }
        _ => {}
    }
}

// Translated from 00c80930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkUtilFunctions::GatherRigidBodies` (Xbox PDB), stack `(node, array)`: appends the Havok
/// body (`+8` of the node's wrapper) to the pointer array (data `+0`, count `+4`, capacity
/// and flags `+8`), growing it by 4 elements through `00c90940` on the memory router's
/// allocator when full, and does the same for every child.
pub fn bhk_util_functions_gather_rigid_bodies(e: &mut Engine, node: Ptr, array: Ptr) {
    let me = node.addr();
    if me == 0 {
        return;
    }
    let wrapper = wrapper_of(e, me);
    if wrapper != 0 {
        let body = e.mem.u32(wrapper + 8);
        push_word(e, array.addr(), body);
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0;
    while index < child_count(e, children) {
        let child = child_at(e, children, index);
        bhk_util_functions_gather_rigid_bodies(e, Ptr::new(child), array);
        index += 1;
    }
}

/// Appends `value` to a pointer array (data `+0`, count `+4`, capacity and flags `+8`): the
/// memory router is looked up first, and a full array is grown by one element size 4 through
/// `00c90940` `(allocator, array, 4)`.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let count = e.mem.u32(array + 4);
    if count == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let count = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(count.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, count.wrapping_add(1));
}

// Translated from 00c809e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, cdecl stack `(node, array)`: appends the answer of `00c7fb50` for the node to
/// the pointer array the way [`bhk_util_functions_gather_rigid_bodies`] does.
pub fn fn_00c809e0(e: &mut Engine, node: Ptr, array: Ptr) {
    let found = fn_00c7fb50(e, node);
    push_word(e, array.addr(), found);
}

// Translated from 00c80a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<class NiPointer<class bhkSerializable> >::AddHead` (Xbox PDB), `ECX` = the
/// head node (item `+0`, next `+4`), stack `(slot)` (the item is `[slot]`): does nothing for a
/// null item. A head without an item takes it over (`006e5cc0`). Otherwise a new 8-byte node
/// (`00aa3e40`) takes the head's item (counted up with `InterlockedIncrement` of its `+4`)
/// and the head's next link, becomes the head's next, and the head takes the new item. A
/// failed allocation writes through address 4 in the exe.
pub fn bs_simple_list_ni_pointer_bhk_serializable_add_head(e: &mut Engine, list: Ptr, slot: Ptr) {
    let me = list.addr();
    if e.mem.u32(slot.addr()) == 0 {
        return;
    }
    if e.mem.u32(me) == 0 {
        e.call(LIST_ASSIGN, &args![me, slot]);
        return;
    }
    let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 8u32]).u32();
    let mut node = 0;
    if block != 0 {
        let head = e.mem.u32(me);
        e.mem.set_u32(block, head);
        if head != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![head + 4]);
        }
        e.mem.set_u32(block + 4, 0);
        node = block;
    }
    let next = e.mem.u32(me + 4);
    e.mem.set_u32(node + 4, next);
    e.mem.set_u32(me + 4, node);
    e.call(LIST_ASSIGN, &args![me, slot]);
}

// Translated from 00c80aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<class NiPointer<class bhkSerializable> >::AddTail` (Xbox PDB), `ECX` = the
/// head node, stack `(slot)`: does nothing for a null item. Walks to the last node; a last
/// node without an item takes it over (`006e5cc0`); otherwise a new 8-byte node (`00aa3e40`)
/// with the item (counted up with `InterlockedIncrement` of its `+4`) and no next link is
/// attached (a failed allocation attaches null).
pub fn bs_simple_list_ni_pointer_bhk_serializable_add_tail(e: &mut Engine, list: Ptr, slot: Ptr) {
    let me = list.addr();
    let item = e.mem.u32(slot.addr());
    if item == 0 {
        return;
    }
    let mut last = me;
    while e.mem.u32(last + 4) != 0 {
        last = e.mem.u32(last + 4);
    }
    if e.mem.u32(last) == 0 {
        e.call(LIST_ASSIGN, &args![last, slot]);
        return;
    }
    let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 8u32]).u32();
    if block == 0 {
        e.mem.set_u32(last + 4, 0);
        return;
    }
    let item = e.mem.u32(slot.addr());
    e.mem.set_u32(block, item);
    if item != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![item + 4]);
    }
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(last + 4, block);
}

// Translated from 00c80b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = an array of 8-byte elements (data `+4`, count `+8`, capacity `+0xC`),
/// stack `(index, element)`: inserts the element (a pointer to 8 bytes) at the index. At the
/// end it makes room with `00c7fc60`. A full array is regrown (`4` slot of the table, capacity
/// doubled, `+ 0x400` above 0x400) with the elements before the index copied to the new block
/// and those from the index on copied one place further (`00c7f900`), the old block freed
/// (slot `+8`); otherwise the elements from the index on move up one place in place.
pub fn fn_00c80b20(e: &mut Engine, this: Ptr, index: u32, element: Ptr) {
    let me = this.addr();
    let source = element.addr();
    let count = e.mem.u32(me + 8);
    if index == count {
        let at = fn_00c7fc60(e, this);
        let data = e.mem.u32(me + 4);
        let low = e.mem.u32(source);
        let high = e.mem.u32(source + 4);
        e.mem.set_u32(data.wrapping_add(at.wrapping_mul(8)), low);
        e.mem
            .set_u32(data.wrapping_add(at.wrapping_mul(8)) + 4, high);
        return;
    }
    let capacity = e.mem.u32(me + 0xc);
    let offset = index.wrapping_mul(8);
    if count == capacity {
        let bigger = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_add(capacity)
        };
        let block = e.vcall(me, 4, &args![bigger]).u32();
        let data = e.mem.u32(me + 4);
        // 00c80b88 / 00c80ba7 / 00c80bdc: `00c7f900` is stdcall (RET 0xc) and does not
        // read ECX.
        e.call(COPY_ELEMENTS, &args![block, data, index]);
        let count = e.mem.u32(me + 8);
        let data = e.mem.u32(me + 4);
        e.call(
            COPY_ELEMENTS,
            &args![
                offset.wrapping_add(block).wrapping_add(8),
                data.wrapping_add(offset),
                count.wrapping_sub(index)
            ],
        );
        let old = e.mem.u32(me + 4);
        e.vcall(me, 8, &args![old]);
        e.mem.set_u32(me + 4, block);
        e.mem.set_u32(me + 0xc, bigger);
    } else {
        let data = e.mem.u32(me + 4).wrapping_add(offset);
        e.call(
            COPY_ELEMENTS,
            &args![data.wrapping_add(8), data, count.wrapping_sub(index)],
        );
    }
    let used = e.mem.u32(me + 8);
    e.mem.set_u32(me + 8, used.wrapping_add(1));
    let data = e.mem.u32(me + 4);
    let low = e.mem.u32(source);
    let high = e.mem.u32(source + 4);
    e.mem.set_u32(data.wrapping_add(offset), low);
    e.mem.set_u32(data.wrapping_add(offset) + 4, high);
}

// Translated from 00c80c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, cdecl stack `(node, array, max_depth, depth)`: breadth-first search for the
/// first node with a collision holder (`[node + 0x1c]` cast to the type `012043f8`). A node
/// without one pushes its non-null children, each with depth + 1, at the front of the array
/// of 8-byte `{node, depth}` entries (`00c80b20` at index 0) while `depth < max_depth`. Then,
/// unless a holder was found, the last entry is taken off the array and searched the same
/// way. Answers the holder or 0.
pub fn fn_00c80c00(e: &mut Engine, node: Ptr, array: Ptr, max_depth: u32, depth: u32) -> u32 {
    let me = node.addr();
    let tracker = e.mem.u32(me + 0x1c);
    let found = if tracker != 0 {
        e.call(CAST, &args![tracker, TYPE_HOLDER]).u32()
    } else {
        0
    };
    if found == 0 {
        let children = e.vcall(me, 0xc, &args![]).u32();
        if children != 0 && depth < max_depth {
            let mut index = 0;
            while index < child_count(e, children) {
                let child = child_at(e, children, index);
                if child != 0 {
                    e.with_stack(8, |e, pair| {
                        e.mem.set_u32(pair.addr(), child);
                        e.mem.set_u32(pair.addr() + 4, depth + 1);
                        fn_00c80b20(e, array, 0, pair);
                    });
                }
                index += 1;
            }
        }
    }
    let count = e.mem.u32(array.addr() + 8);
    if count != 0 && found == 0 {
        let data = e.mem.u32(array.addr() + 4);
        let last = data.wrapping_add(count.wrapping_mul(8));
        let next_node = e.mem.u32(last.wrapping_sub(8));
        let next_depth = e.mem.u32(last.wrapping_sub(4));
        e.mem.set_u32(array.addr() + 8, count - 1);
        return fn_00c80c00(e, Ptr::new(next_node), array, max_depth, next_depth);
    }
    found
}

// Translated from 00c80ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c80ce0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkUtilFunctions::ReplaceConstraints` (Xbox PDB), cdecl stack `(node, mode)`: in the
/// allocation scope, only for mode 8 and a non-null node. For every constraint in the node's
/// wrapper constraint list (items cast to the type `0126818c`) whose Havok data object has kind
/// 2 or 7 (or kind 13, a malleable wrapper, whose wrapped data has kind 7 or 2) and that has
/// two bodies (virtual slots `+0xCC` and `+0xD0`, each object's `+0xC`): builds a
/// `hkStiffSpringConstraintCinfo` on the stack (`00cc5ae0` creates its data), puts the two
/// pivots (16 bytes at `+0x50` and `+0x90` of the data) into the data's `+0x20` and `+0x30`,
/// transforms both pivots by their bodies' transforms (virtual slot `+0xF4`, `00c7f640`),
/// takes the distance between them (`SUBPS`, `MULPS`, `ADDSS`, `SQRTSS`; 0 becomes the float
/// at `01017d00`) as the data's `+0x44`, creates the new `bhkStiffSpringConstraint`
/// (`00c7f9e0` in a 0x10-byte block), disables the old Havok constraint (`00cdff60`), adds the
/// new constraint to the list (`00c80aa0` after the last node, or `00c80a30` after the
/// current), lets the new constraint know the first body's id (virtual slot `+0x9C`) and
/// clears the cinfo (`00c89ac0`). Then the same for every child with mode 8.
pub fn bhk_util_functions_replace_constraints(e: &mut Engine, node: Ptr, mode: u32) {
    let old = enter_scope(e);
    if node.is_null() || mode != 8 {
        leave_scope(e, old);
        return;
    }
    let me = node.addr();
    let wrapper = wrapper_of(e, me);
    if wrapper != 0 {
        let mut cell = wrapper + 0x14;
        while cell != 0 {
            let item = e.mem.u32(cell);
            if item == 0 {
                break;
            }
            e.with_stack(0x150, |e, frame| {
                replace_one(e, cell, item, frame.addr());
            });
            cell = e.mem.u32(cell + 4);
        }
    }
    let children = e.vcall(me, 0xc, &args![]).u32();
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            bhk_util_functions_replace_constraints(e, Ptr::new(child), 8);
            index += 1;
        }
    }
    leave_scope(e, old);
}

/// One list item of [`bhk_util_functions_replace_constraints`]; `frame` is the local area
/// (pivots at `+0x44` and `+0x54`, cinfo at `+0x24`, transformed pivots at `+0x64` and
/// `+0x74`, transforms at `+0x84` and `+0xC4`).
fn replace_one(e: &mut Engine, cell: u32, item: u32, frame: u32) {
    let constraint = e.call(CAST, &args![item, TYPE_CONSTRAINT]).u32();
    if constraint == 0 {
        return;
    }
    let data = constraint_data(e, constraint);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    let (first, second) = match kind {
        2 | 7 => {
            copy_bytes(e, frame + 0x44, data + 0x50, 0x10);
            copy_bytes(e, frame + 0x54, data + 0x90, 0x10);
            (body_of(e, constraint, 0xcc), body_of(e, constraint, 0xd0))
        }
        13 => {
            let wrapped = e.call(WRAPPED_CONSTRAINT_DATA, &args![data]).u32();
            let mut kept = 0;
            let mut second = 0;
            if e.vcall(wrapped, 0x20, &args![]).u32() == 7 {
                copy_bytes(e, frame + 0x44, wrapped + 0x50, 0x10);
                copy_bytes(e, frame + 0x54, wrapped + 0x90, 0x10);
                kept = body_of(e, constraint, 0xcc);
                second = body_of(e, constraint, 0xd0);
            }
            if e.vcall(wrapped, 0x20, &args![]).u32() == 2 {
                copy_bytes(e, frame + 0x44, wrapped + 0x50, 0x10);
                copy_bytes(e, frame + 0x54, wrapped + 0x90, 0x10);
                (body_of(e, constraint, 0xcc), body_of(e, constraint, 0xd0))
            } else {
                (kept, second)
            }
        }
        _ => return,
    };
    if first == 0 || second == 0 {
        return;
    }
    let cinfo = frame + 0x24;
    e.mem.set_u32(cinfo + 4, 0);
    e.mem.set_u32(cinfo + 8, 1);
    e.mem.set_u32(cinfo + 0xc, 0);
    e.mem.set_u32(cinfo + 0x10, 0);
    e.mem.set_u32(cinfo, VTABLE_STIFF_SPRING_CINFO);
    e.call(CINFO_CREATE_DATA, &args![cinfo]);
    let spring = e.mem.u32(cinfo + 4);
    let body_a = e.mem.u32(first + 8);
    let body_b = e.mem.u32(second + 8);
    e.mem.set_u32(cinfo + 0xc, body_a);
    e.mem.set_u32(cinfo + 0x10, body_b);
    if spring != 0 {
        copy_bytes(e, spring + 0x20, frame + 0x44, 0x10);
        let spring = e.mem.u32(cinfo + 4);
        copy_bytes(e, spring + 0x30, frame + 0x54, 0x10);
    }
    e.vcall(first, 0xf4, &args![frame + 0x84]);
    e.vcall(second, 0xf4, &args![frame + 0xc4]);
    e.call(
        SET_TRANSFORMED_POS,
        &args![frame + 0x64, frame + 0x84, frame + 0x44],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![frame + 0x74, frame + 0xc4, frame + 0x54],
    );
    let mut difference = [0u32; 4];
    for (lane, slot) in difference.iter_mut().enumerate() {
        let lane = lane as u32;
        let a = e.mem.u32(frame + 0x64 + lane * 4);
        let b = e.mem.u32(frame + 0x74 + lane * 4);
        *slot = sse(a, b, |x, y| x - y);
        e.mem.set_u32(frame + 0x64 + lane * 4, *slot);
    }
    let squares = difference.map(|lane| mulps_lane(lane, lane));
    let pair = sse(squares[1], squares[0], |x, y| x + y);
    let sum = sse(squares[2], pair, |x, y| x + y);
    let length = sqrtss(sum);
    e.mem.set_u32(frame + 0x38, length);
    let mut rest = x87_quiet_bits(length);
    // FUCOMPP of 0.0 against the length: equal replaces it.
    if f32::from_bits(length) == 0.0 {
        rest = x87_quiet_bits(e.mem.u32(MINIMUM_LENGTH));
    }
    e.mem.set_u32(frame + 0x18, rest);
    let spring = e.mem.u32(cinfo + 4);
    if spring != 0 {
        let stored = x87_quiet_bits(e.mem.u32(frame + 0x18));
        e.mem.set_u32(spring + 0x44, stored);
    }
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let created = if block != 0 {
        fn_00c7f9e0(e, Ptr::new(block), cinfo)
    } else {
        0
    };
    let holder = e.vcall(first, 0x94, &args![]).u32();
    let id = if holder != 0 {
        e.mem.u32(holder + 0x350)
    } else {
        0
    };
    let havok = e.mem.u32(constraint + 8);
    if e.mem.u32(havok + 8) != 0 {
        let inner = e.mem.u32(havok + 8);
        e.call(DISABLE_CONSTRAINT, &args![havok, inner]);
    }
    e.mem.set_u32(frame + 0x18, created);
    let next = e.mem.u32(cell + 4);
    if next == 0 {
        if created != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![created + 4]);
        }
        bs_simple_list_ni_pointer_bhk_serializable_add_tail(
            e,
            Ptr::new(cell),
            Ptr::new(frame + 0x18),
        );
    } else {
        if created != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![created + 4]);
        }
        let next = e.mem.u32(cell + 4);
        bs_simple_list_ni_pointer_bhk_serializable_add_head(
            e,
            Ptr::new(next),
            Ptr::new(frame + 0x18),
        );
    }
    e.call(RELEASE_SLOT, &args![frame + 0x18]);
    if created != 0 {
        e.vcall(created, 0x9c, &args![id]);
    }
    e.mem.set_u32(cinfo, VTABLE_CINFO);
    e.call(CINFO_SET_DATA, &args![cinfo, 0u32]);
}

/// The body at virtual slot `slot` (`+0xCC` or `+0xD0`) of a constraint wrapper: that object's
/// `+0xC`, or 0.
fn body_of(e: &mut Engine, constraint: u32, slot: u32) -> u32 {
    let object = e.vcall(constraint, slot, &args![]).u32();
    if object != 0 {
        e.mem.u32(object + 0xc)
    } else {
        0
    }
}

// Translated from 00c811f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c811f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkUtilFunctions::PrepareDebugGeom` (Xbox PDB), cdecl stack `(geometry, owner, key, colour)`:
/// copies the three colour words, runs the colour hook (`00c84cd0` on `(key, copy)`) when the
/// flag byte `012680fc` is set, builds a 0x4c-byte `NiMaterialProperty` (`00a75650`) with the
/// colour at `+0x28`, counts it up (`+0x44`) and attaches it to the geometry (`00439410`);
/// calls the callback pointer at `01267b64` with `(geometry, copy)` when set; and finally
/// calls virtual slot `+0xDC` of the owner with `(geometry, 1)`. A failed allocation writes
/// through the null in the exe.
pub fn bhk_util_functions_prepare_debug_geom(
    e: &mut Engine,
    geometry: u32,
    owner: u32,
    key: u32,
    colour: Ptr,
) {
    e.with_stack(0xc, |e, copy| {
        let source = colour.addr();
        for word in 0..3 {
            let value = e.mem.u32(source + word * 4);
            e.mem.set_u32(copy.addr() + word * 4, value);
        }
        if e.mem.u8(COLOUR_HOOK_FLAG) != 0 {
            e.call(COLOUR_HOOK, &args![key, copy]);
        }
        let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
        let property = if block != 0 {
            e.call(MATERIAL_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        for word in 0..3 {
            let value = e.mem.u32(copy.addr() + word * 4);
            e.mem.set_u32(property + 0x28 + word * 4, value);
        }
        let references = e.mem.u32(property + 0x44);
        e.mem.set_u32(property + 0x44, references.wrapping_add(1));
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
        let callback = e.mem.u32(COLOUR_CALLBACK);
        if callback != 0 {
            e.call(callback, &args![geometry, copy]);
        }
        e.vcall(owner, 0xdc, &args![geometry, 1u32]);
    });
}

// Translated from 00c812d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c812d0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name, cdecl stack `(node, max_depth)`: runs the search `00c80c00` with a local array
/// object (vtable `010c53b4`, allocated for 0x20 elements through the table's slot `+4`, empty)
/// and depth 0, then gives the array's block back through the table's slot `+8`. Answers the
/// holder found.
pub fn fn_00c812d0(e: &mut Engine, node: Ptr, max_depth: u32) -> u32 {
    e.with_stack(0x10, |e, array| {
        let me = array.addr();
        e.mem.set_u32(me, VTABLE_ARRAY);
        e.mem.set_u32(me + 4, 0);
        e.mem.set_u32(me + 8, 0);
        e.mem.set_u32(me + 0xc, 0);
        let block = e.vcall(me, 4, &args![0x20u32]).u32();
        e.mem.set_u32(me + 4, block);
        e.mem.set_u32(me + 0xc, 0x20);
        let found = fn_00c80c00(e, node, array, max_depth, 0);
        e.mem.set_u32(me, VTABLE_ARRAY);
        let data = e.mem.u32(me + 4);
        if data != 0 {
            e.vcall(me, 8, &args![data]);
        }
        found
    })
}

// Translated from 00c81380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c81380 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled, and the two words at +8 and +0xC of the capsule description (stack +0x1c, +0x20) are never written in the exe (stack garbage, taken as zeros).
/// `bhkUtilFunctions::CreateTree` (Xbox PDB), cdecl stack `(height, radius)` (floats): builds a
/// capsule `bhkCapsuleShape` and a rigid body around it. With `r` the radius and `h` the
/// height: when `h <= r + r` (ordered), `h` becomes `r + r + double(01012070)` (stored as a
/// float). The capsule description (on the stack, 0x30 bytes from `+0x14`) holds the radius
/// `half * r`, the points `(0, 0, half * (h - r), 0)` and `(0, 0, half * r, 0)` (`half` is the
/// float at `01267e44`); a 0x14-byte capsule is built from it (`0066d6a0`) and its `+0x10` set
/// to 9. A `bhkRigidBodyCinfo` (`00c8f510`, on the stack) gets the filter word 0x40009 at `+0`
/// and `+0x20`, the capsule's Havok shape at `+4` and `+0x24`, the float at `01018180` at
/// `+0xC0`, zero at `+0xB0` and the byte 5 at `+0xD4`; a 0x1c-byte `bhkRigidBody` is built
/// from it (`0056d380`); the cinfo's array at `+0xC` is freed through the router's allocator
/// (slot `+0x10`, `(block, bytes)`) unless its capacity word at `+0x14` is negative. Answers
/// the rigid body, or 0 when the capsule allocation failed.
pub fn bhk_util_functions_create_tree(e: &mut Engine, height: f32, radius: f32) -> u32 {
    let radius_value = x87_load32(radius.to_bits());
    let double_radius = x87_add(radius_value, radius_value);
    let mut height_bits = height.to_bits();
    let height_value = x87_load32(height_bits);
    if !height_value.is_nan() && !double_radius.is_nan() && height_value <= double_radius {
        let constant = e.mem.f64(TREE_DOUBLE);
        height_bits = x87_store32(x87_add(double_radius, constant));
    }
    let half = x87_load32(e.mem.u32(HALF));
    let cap_bits = x87_store32(x87_mul(half, radius_value));
    let difference = x87_sub(x87_load32(height_bits), radius_value);
    let difference_bits = x87_store32(difference);
    let body_bits = x87_store32(x87_mul(half, x87_load32(difference_bits)));
    e.with_stack(0x150, |e, frame| {
        let q = frame.addr();
        e.mem.set_u32(q + 0x14, 0);
        e.mem.set_u32(q + 0x18, cap_bits);
        e.mem.set_u32(q + 0x24, 0);
        e.mem.set_u32(q + 0x28, 0);
        e.mem.set_u32(q + 0x2c, body_bits);
        e.mem.set_u32(q + 0x30, 0);
        e.mem.set_u32(q + 0x34, 0);
        e.mem.set_u32(q + 0x38, 0);
        e.mem.set_u32(q + 0x3c, x87_quiet_bits(cap_bits));
        e.mem.set_u32(q + 0x40, 0);
        let block = e.call(ALLOCATE, &args![0x14u32]).u32();
        let shape = if block != 0 {
            e.call(CAPSULE_CONSTRUCT, &args![block, q + 0x14]).u32()
        } else {
            0
        };
        if shape == 0 {
            return 0;
        }
        e.mem.set_u32(shape + 0x10, 9);
        let cinfo = q + 0x44;
        e.call(RIGID_BODY_CINFO_CONSTRUCT, &args![cinfo]);
        let havok = e.mem.u32(shape + 8);
        e.mem.set_u32(q + 0xf4, 0);
        let constant = x87_quiet_bits(e.mem.u32(TREE_FLOAT));
        e.mem.set_u32(q + 0x104, constant);
        e.mem.set_u32(cinfo, 0x0004_0009);
        e.mem.set_u32(cinfo + 0x20, 0x0004_0009);
        e.mem.set_u32(cinfo + 4, havok);
        e.mem.set_u32(cinfo + 0x24, havok);
        e.mem.set_u8(q + 0x118, 5);
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let body = if block != 0 {
            e.call(RIGID_BODY_CONSTRUCT, &args![block, cinfo]).u32()
        } else {
            0
        };
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(cinfo + 0x14);
        e.mem.set_u32(cinfo + 0x10, 0);
        if capacity as i32 >= 0 {
            let data = e.mem.u32(cinfo + 0xc);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 3]);
        }
        body
    })
}

// Translated from 00cb9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<struct bhkRagdollController::BoneTransf>::_Deallocate` (Xbox PDB), stack
/// `(block)`: the pool free (`00aa4060` on the pool object `011f6238`).
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn bs_simple_array_bone_transf_deallocate(e: &mut Engine, _this: Ptr, block: u32) {
    e.call(POOL_FREE, &args![POOL_OBJECT, block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c7f970, fn_00c7f970(Ptr, u32, u32)),
        entry!(0x00c7f9e0, fn_00c7f9e0(Ptr, u32) -> u32),
        entry!(0x00c7fa60, bhk_stiff_spring_constraint_get_r_t_t_i() -> u32),
        entry!(0x00c7fa70, bhk_stiff_spring_constraint_scalar_deleting_destructor(Ptr, u32) -> u32),
        entry!(0x00c7fa90, bhk_util_functions_get_ni_a_v_object(Ptr) -> u32),
        entry!(0x00c7fb20, fn_00c7fb20(Ptr) -> u32),
        entry!(0x00c7fb50, fn_00c7fb50(Ptr) -> u32),
        entry!(0x00c7fc20, bs_simple_array_av_level_allocate(Ptr, u32) -> u32),
        entry!(0x00c7fc40, bs_simple_array_av_level_reallocate(Ptr, u32, u32) -> u32),
        entry!(0x00c7fc60, fn_00c7fc60(Ptr) -> u32),
        entry!(0x00c7fcc0, fn_00c7fcc0(Ptr, u32, Ptr, u32) -> bool),
        entry!(0x00c7ff40, fn_00c7ff40(Ptr, u32) -> u32),
        entry!(0x00c7ffd0, fn_00c7ffd0(Ptr) -> u32),
        entry!(0x00c80020, fn_00c80020(Ptr) -> u32),
        entry!(0x00c80090, fn_00c80090(Ptr) -> bool),
        entry!(0x00c80130, do_thread_setup(Ptr)),
        entry!(0x00c80240, do_thread_shutdown(Ptr)),
        entry!(0x00c802d0, bhk_util_functions_find_next_collision_object(Ptr) -> u32),
        entry!(0x00c80330, bhk_util_functions_set_c_o_use_vel(u32, Ptr)),
        entry!(
            0x00c803a0,
            bhk_util_functions_damp_bodies_velocity(Ptr, f32)
        ),
        entry!(0x00c804d0, fn_00c804d0(Ptr, u32)),
        entry!(0x00c80570, fn_00c80570(Ptr, Ptr)),
        entry!(0x00c80630, fn_00c80630(Ptr)),
        entry!(0x00c80660, bs_simple_array_av_level_scalar_deleting_destructor(Ptr, u32) -> u32),
        entry!(0x00c806b0, fn_00c806b0(Ptr, u32, u32) -> u32),
        entry!(0x00c80700, fn_00c80700(Ptr)),
        entry!(0x00c80930, bhk_util_functions_gather_rigid_bodies(Ptr, Ptr)),
        entry!(0x00c809e0, fn_00c809e0(Ptr, Ptr)),
        entry!(
            0x00c80a30,
            bs_simple_list_ni_pointer_bhk_serializable_add_head(Ptr, Ptr)
        ),
        entry!(
            0x00c80aa0,
            bs_simple_list_ni_pointer_bhk_serializable_add_tail(Ptr, Ptr)
        ),
        entry!(0x00c80b20, fn_00c80b20(Ptr, u32, Ptr)),
        entry!(0x00c80c00, fn_00c80c00(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00c80ce0, bhk_util_functions_replace_constraints(Ptr, u32)),
        entry!(
            0x00c811f0,
            bhk_util_functions_prepare_debug_geom(u32, u32, u32, Ptr)
        ),
        entry!(0x00c812d0, fn_00c812d0(Ptr, u32) -> u32),
        entry!(0x00c81380, bhk_util_functions_create_tree(f32, f32) -> u32),
        entry!(0x00cb9cb0, bs_simple_array_bone_transf_deallocate(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object whose `+0xc` word is null, holding `inner` at `+0`.
    fn object_00c7ffd0(e: &mut Engine, kind: u32, vtable: u32) -> (Ptr, u32) {
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner, vtable);
        e.mem.set_u32(inner + 0xc, kind);
        e.mem.set_u32(inner + 8, 0x1111);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, inner);
        (Ptr::<()>::new(object), inner)
    }

    #[test]
    fn fn_00c7ffd0_reads_the_word_at_8_for_kind_9() {
        let mut e = Engine::new();
        let (object, _) = object_00c7ffd0(&mut e, 9, 0);
        assert_eq!(e.call(0x00c7_ffd0, &args![object]).u32(), 0x1111);
        let (other, _) = object_00c7ffd0(&mut e, 7, 0);
        assert_eq!(e.call(0x00c7_ffd0, &args![other]).u32(), 0);
    }

    #[test]
    fn fn_00c7ffd0_follows_the_holder_word_and_the_virtual_result() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_2000, &[0, 0, 0, 0, 0x0090_3000]);
        e.register(0x0090_3000, |e, _| Ret {
            eax: e.global::<u32>(0x0090_4000),
            ..Ret::default()
        });
        e.map(0x0090_4000, 0x10);
        let (outer, inner) = object_00c7ffd0(&mut e, 8, 0x0090_2000);
        // The holder is a separate object whose first word is `inner`.
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder, inner);
        e.mem.set_u32(outer.addr() + 0xc, holder);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0x8, 0x3333);
        // A virtual result r makes the function read the word at r - 8.
        e.set_global(0x0090_4000, record + 0x10);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0x3333);
        e.set_global(0x0090_4000, 0u32);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0);
        e.set_global(0x0090_4000, 0x10u32);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0);
    }

    #[test]
    fn bhk_stiff_spring_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c7_fa60, &args![]).u32(), 0x0126_85f8);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    type Log = Vec<Vec<u32>>;

    /// Doubles for virtual slots and callbacks.
    const ALLOC_D: u32 = 0x0900_0100;
    const FREE_D: u32 = 0x0900_0104;
    const REALLOC_D: u32 = 0x0900_0108;
    const KIND_D: u32 = 0x0900_010c;
    const CHILDREN_D: u32 = 0x0900_0200;
    const FILL_D: u32 = 0x0900_0300;
    const RECORD_D: u32 = 0x0900_0304;
    /// Scratch words in the mapped exe data page a double can leave results in.
    const SAVED: u32 = 0x0126_7f00;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the exe data pages the unit reads mapped and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x2000);
        e.map(0x0126_7000, 0x2000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The array object of the unit: data `+4`, count `+8`, capacity `+0xC`; the allocation
    /// slot answers fresh blocks, the free slot is a double and the reallocation slot answers
    /// `0xc0ffee`.
    fn array(e: &mut Engine) -> u32 {
        e.register(ALLOC_D, |e, _| ret(e.mem.alloc(0x400)));
        quiet(e, &[FREE_D]);
        answer(e, REALLOC_D, 0x00c0_ffee);
        object(e, &[(4, ALLOC_D), (8, FREE_D), (0xc, REALLOC_D)])
    }

    /// The scope word starts at a recognisable value so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// The dynamic cast double: a holder type answers the word at `+8` of the object, the
    /// constraint type the object itself.
    fn install_cast(e: &mut Engine) {
        e.register(CAST, |e, a| match a[1] {
            TYPE_HOLDER | TYPE_ENTITY_HOLDER => ret(e.mem.u32(a[0] + 8)),
            TYPE_CONSTRAINT => ret(a[0]),
            _ => ret(0),
        });
    }

    /// A scene node whose children (virtual slot `+0xC`) are the object at `+0x60`.
    fn node(e: &mut Engine) -> u32 {
        e.register(CHILDREN_D, |e, a| ret(e.mem.u32(a[0] + 0x60)));
        object(e, &[(0xc, CHILDREN_D)])
    }

    fn adopt(e: &mut Engine, parent: u32, children: &[u32]) {
        let block = e.mem.alloc(0x100);
        let list = e.mem.alloc(0x100);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *child);
        }
        e.mem.set_u32(block + 0xa0, list);
        e.mem.set_u16(block + 0xa6, children.len() as u16);
        e.mem.set_u32(parent + 0x60, block);
    }

    /// Gives `node` a collision object chain whose body has the filter word; answers `(holder,
    /// wrapper, body)`. The wrapper gets a vtable with the given slots.
    fn collision(e: &mut Engine, node: u32, filter: u32, slots: &[(u32, u32)]) -> (u32, u32, u32) {
        let tracker = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x40);
        let wrapper = object(e, slots);
        let body = e.mem.alloc(0x400);
        e.mem.set_u32(node + 0x1c, tracker);
        e.mem.set_u32(tracker + 8, holder);
        e.mem.set_u32(holder + 0x10, wrapper);
        e.mem.set_u32(wrapper + 8, body);
        e.mem.set_u32(body + 0x2c, filter);
        (holder, wrapper, body)
    }

    #[test]
    fn array_reallocation_follows_the_three_cases() {
        let mut e = engine();
        e.register(COPY_ELEMENTS, |_, _| Ret::default());
        let this = array(&mut e);
        // Without data: allocate and take the capacity.
        e.call(0x00c7_f970, &args![this, 8u32, 0u32]);
        let first = e.mem.u32(this + 4);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        // Count equal to the capacity: reallocate the block.
        e.call(0x00c7_f970, &args![this, 16u32, 8u32]);
        assert_eq!(calls_to(&e, REALLOC_D), vec![vec![this, first, 16]]);
        assert_eq!(e.mem.u32(this + 4), 0x00c0_ffee);
        // Otherwise copy into a new block, free the old one, keep the capacity word.
        e.mem.set_u32(this + 4, first);
        e.call(0x00c7_f970, &args![this, 16u32, 3u32]);
        let fresh = e.mem.u32(this + 4);
        assert_ne!(fresh, first);
        assert_eq!(calls_to(&e, COPY_ELEMENTS), vec![vec![fresh, first, 3]]);
        assert_eq!(calls_to(&e, FREE_D), vec![vec![this, first]]);
        assert_eq!(e.mem.u32(this + 0xc), 8);
    }

    #[test]
    fn stiff_spring_constructor_chains_the_vtables_and_counts() {
        let mut e = engine();
        quiet(&mut e, &[REF_OBJECT_CONSTRUCT, CONSTRAINT_INIT]);
        e.set_global(COUNT_CONSTRAINT, 4u32);
        e.set_global(COUNT_STIFF_SPRING, 9u32);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x55);
        let cinfo = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c7_f9e0, &args![this, cinfo]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_STIFF_SPRING);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 5);
        assert_eq!(e.global::<u32>(COUNT_STIFF_SPRING), 10);
        assert_eq!(
            order(&e),
            vec![0x00c7_f9e0, REF_OBJECT_CONSTRUCT, CONSTRAINT_INIT]
        );
        assert_eq!(calls_to(&e, CONSTRAINT_INIT), vec![vec![this, cinfo + 4]]);
        e.call(0x00c7_f9e0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, CONSTRAINT_INIT)[1], vec![this, 0]);
    }

    #[test]
    fn stiff_spring_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[STIFF_SPRING_DESTRUCT, SIZED_FREE]);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c7_fa70, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, SIZED_FREE).is_empty());
        assert_eq!(e.call(0x00c7_fa70, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_FREE), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, STIFF_SPRING_DESTRUCT).len(), 2);
    }

    #[test]
    fn get_ni_a_v_object_reads_the_property_or_asks_the_world_object() {
        let mut e = engine();
        let object = e.mem.alloc(0x200);
        // The world object is at object + (signed byte at +0x10).
        let world = object + 0x40;
        let property = e.mem.alloc(0x20);
        e.mem.set_u32(property + 8, 0x1234);
        // GetProperty leaves the property in the out slot for the world object only.
        e.register_double(GET_PROPERTY, move |e, a| {
            if a[1] == world {
                e.mem.set_u32(a[0], property);
            }
            ret(a[0])
        });
        answer(&mut e, GET_AV_OBJECT, 0x4321);
        e.mem.set_u8(object + 0x10, 0x40);
        e.mem.set_u8(object + 0x18, 1);
        assert_eq!(e.call(0x00c7_fa90, &args![object]).u32(), 0x1234);
        assert_eq!(
            calls_to(&e, GET_PROPERTY)[0][1..],
            [world, PROPERTY_AV_OBJECT]
        );
        e.mem.set_u8(object + 0x18, 2);
        assert_eq!(e.call(0x00c7_fa90, &args![object]).u32(), 0x1234);
        // Another world object without the property: kind 1 gives 0, kind 2 asks +0xC.
        let bare = e.mem.alloc(0x400) + 0x200;
        e.mem.set_u8(bare + 0x10, 0xf0);
        e.mem.set_u8(bare + 0x18, 1);
        assert_eq!(e.call(0x00c7_fa90, &args![bare]).u32(), 0);
        assert_eq!(calls_to(&e, GET_PROPERTY)[2][1], bare - 0x10);
        e.mem.set_u8(bare + 0x18, 2);
        assert_eq!(e.call(0x00c7_fa90, &args![bare]).u32(), 0);
        e.mem.set_u32(bare - 0x10 + 0xc, 0x7000);
        assert_eq!(e.call(0x00c7_fa90, &args![bare]).u32(), 0x4321);
        assert_eq!(calls_to(&e, GET_AV_OBJECT), vec![vec![0x7000, 0]]);
        // Other kinds, and no world object at all.
        e.mem.set_u8(bare + 0x18, 3);
        assert_eq!(e.call(0x00c7_fa90, &args![bare]).u32(), 0);
        let none = e.mem.alloc(0x200);
        e.mem.set_u8(none + 0x10, 0);
        e.mem.set_u8(none + 0x18, 2);
        // Offset 0 puts the world object at the object itself, which exists.
        assert_eq!(e.call(0x00c7_fa90, &args![none]).u32(), 0);
    }

    #[test]
    fn cinfo_constructor_sets_the_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        for offset in [4, 8, 0xc, 0x10] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        assert_eq!(e.call(0x00c7_fb20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_STIFF_SPRING_CINFO);
        assert_eq!(
            [4, 8, 0xc, 0x10].map(|offset| e.mem.u32(this + offset)),
            [0, 1, 0, 0]
        );
    }

    /// A chain entry (0x30 bytes): the object at `+0xC` answers `kind` from virtual slot
    /// `0x20`; `+0x14` and `+0x18` are the two ends.
    fn chain_entry(e: &mut Engine, kind: u32) -> u32 {
        e.register(KIND_D, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        let inner = object(e, &[(0x20, KIND_D)]);
        e.mem.set_u32(inner + 0x40, kind);
        let entry = e.mem.alloc(0x40);
        e.mem.set_u32(entry + 0xc, inner);
        entry
    }

    /// A chain object with the given entries (array at `+0xAC`, count at `+0xB0`).
    fn chain(e: &mut Engine, entries: &[u32]) -> u32 {
        let block = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x100 * 0x30);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(table + 0x30 * index as u32, *entry);
        }
        e.mem.set_u32(block + 0xac, table);
        e.mem.set_u16(block + 0xb0, entries.len() as u16);
        block
    }

    #[test]
    fn chain_walk_stops_at_an_all_eleven_object_and_follows_the_ends() {
        let mut e = engine();
        // An object whose entries are all kind 0xb is the answer.
        let eleven = chain_entry(&mut e, 0xb);
        let last = chain(&mut e, &[eleven]);
        assert_eq!(e.call(0x00c7_fb50, &args![last]).u32(), last);
        assert_eq!(e.call(0x00c7_fb50, &args![0u32]).u32(), 0);
        let empty = chain(&mut e, &[]);
        assert_eq!(e.call(0x00c7_fb50, &args![empty]).u32(), empty);
        // A kind-5 entry sends the walk to the entry's other end.
        let five = chain_entry(&mut e, 5);
        let first = chain(&mut e, &[eleven, five]);
        e.mem.set_u32(five + 0x14, first);
        e.mem.set_u32(five + 0x18, last);
        assert_eq!(e.call(0x00c7_fb50, &args![first]).u32(), last);
        // The entry's first end is used when it is not the current object.
        e.mem.set_u32(five + 0x14, last);
        assert_eq!(e.call(0x00c7_fb50, &args![first]).u32(), last);
        // A walk that runs out of ends answers 0.
        e.mem.set_u32(five + 0x14, first);
        e.mem.set_u32(five + 0x18, 0);
        assert_eq!(e.call(0x00c7_fb50, &args![first]).u32(), 0);
    }

    #[test]
    fn chain_walk_skips_kind_twelve_entries_whose_flag_is_set() {
        let mut e = engine();
        let twelve = chain_entry(&mut e, 0xc);
        let five = chain_entry(&mut e, 5);
        let end = chain(&mut e, &[]);
        // The kind-12 entry: flag index at inner + 0x1c, flag table at [entry + 0x30] + 0x1c.
        let inner = e.mem.u32(twelve + 0xc);
        e.mem.set_u16(inner + 0x1c, 3);
        let owner = e.mem.alloc(0x40);
        let flags = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x1c, flags);
        e.mem.set_u32(twelve + 0x30, owner);
        let first = chain(&mut e, &[twelve, five]);
        e.mem.set_u32(five + 0x14, end);
        e.mem.set_u32(twelve + 0x14, first);
        e.mem.set_u32(twelve + 0x18, 0);
        // Flag clear: the kind-12 entry itself is chosen (its +0x18 end is 0).
        assert_eq!(e.call(0x00c7_fb50, &args![first]).u32(), 0);
        // Flag set: it is skipped and the kind-5 entry is chosen.
        e.mem.set_u8(flags + 3, 1);
        assert_eq!(e.call(0x00c7_fb50, &args![first]).u32(), end);
    }

    #[test]
    fn pool_array_allocation_scales_by_eight() {
        let mut e = engine();
        answer(&mut e, POOL_ALLOCATE, 0x1111);
        answer(&mut e, POOL_REALLOCATE, 0x2222);
        assert_eq!(e.call(0x00c7_fc20, &args![0x5555u32, 5u32]).u32(), 0x1111);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL_OBJECT, 40]]);
        assert_eq!(
            e.call(0x00c7_fc40, &args![0x5555u32, 0x9000u32, 3u32])
                .u32(),
            0x2222
        );
        assert_eq!(
            calls_to(&e, POOL_REALLOCATE),
            vec![vec![POOL_OBJECT, 0x9000, 24]]
        );
    }

    #[test]
    fn array_room_grows_an_empty_full_and_nearly_full_array() {
        let mut e = engine();
        let this = array(&mut e);
        // Empty array: 4 elements, index 0.
        assert_eq!(e.call(0x00c7_fc60, &args![this]).u32(), 0);
        assert_ne!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(calls_to(&e, ALLOC_D), vec![vec![this, 4]]);
        // Room left: only the count moves.
        assert_eq!(e.call(0x00c7_fc60, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(this + 8), 2);
        // Full: the capacity doubles through 00c7f970 (here: count 4 of 4).
        e.mem.set_u32(this + 8, 4);
        assert_eq!(e.call(0x00c7_fc60, &args![this]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u32(this + 8), 5);
        // Above 0x400 elements the capacity grows by 0x400.
        e.mem.set_u32(this + 8, 0x500);
        e.mem.set_u32(this + 0xc, 0x500);
        e.call(0x00c7_fc60, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 0x900);
    }

    const OWNER_D: u32 = 0x0900_0400;
    const TRACKER_D: u32 = 0x0900_0404;

    /// A ray: the distance the cast reports and a tag it leaves in the result's first word.
    fn ray(e: &mut Engine, distance: f32, tag: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, distance.to_bits());
        e.mem.set_u32(block + 4, tag);
        block
    }

    /// A scene node whose wrapper passes the ray test filter checks, with doubles for the
    /// shape lookups and the cast (result word 0 is the ray's tag, the distance at `+0x1c` is
    /// the ray's first word). Answers `(node, holder, wrapper, body)`.
    fn ray_scene(e: &mut Engine, filter: u32) -> (u32, u32, u32, u32) {
        install_cast(e);
        e.register(FILL_D, |e, a| {
            e.mem.write(a[1], &[0xab; 64]);
            Ret::default()
        });
        answer(e, QUERY_SHAPE, 0x5000);
        e.register(CAST_CDECL, |_, a| {
            ret(if a[0] == TYPE_RAY_SHAPE { 0x6000 } else { 0 })
        });
        e.register(RAY_CAST, |e, a| {
            let distance = e.mem.u32(a[1]);
            let tag = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[3] + 0x1c, distance);
            e.mem.set_u32(a[3], tag);
            ret(1)
        });
        let node = node(e);
        let (holder, wrapper, body) = collision(e, node, filter, &[(0xf4, FILL_D)]);
        (node, holder, wrapper, body)
    }

    #[test]
    fn ray_test_keeps_the_nearest_hit() {
        let mut e = engine();
        let (node, _, wrapper, _) = ray_scene(&mut e, 0x208);
        answer(&mut e, CAST_TEST, 0);
        let best = e.mem.alloc(0x40);
        let far = ray(&mut e, 5.0, 0x1111);
        assert!(!e.call(0x00c7_fcc0, &args![node, far, best, 0u32]).bool());
        assert_eq!(e.mem.u32(best), wrapper);
        assert_eq!(e.mem.u32(best + 0x10), 0x1111);
        assert_eq!(e.mem.u32(best + 0x2c), 5.0f32.to_bits());
        // The transform is fetched twice (before and after the holder test).
        assert_eq!(calls_to(&e, FILL_D).len(), 2);
        let cast = calls_to(&e, RAY_CAST);
        assert_eq!(cast[0][..2], [0x6000, far]);
        assert_eq!(cast[0][3], cast[0][2] + 0x40);
        // Farther and equal hits keep the first, a nearer one replaces it.
        let farther = ray(&mut e, 7.0, 0x2222);
        e.call(0x00c7_fcc0, &args![node, farther, best, 0u32]);
        assert_eq!(e.mem.u32(best + 0x10), 0x1111);
        let equal = ray(&mut e, 5.0, 0x3333);
        e.call(0x00c7_fcc0, &args![node, equal, best, 0u32]);
        assert_eq!(e.mem.u32(best + 0x10), 0x1111);
        let near = ray(&mut e, 2.0, 0x4444);
        e.call(0x00c7_fcc0, &args![node, near, best, 0u32]);
        assert_eq!(e.mem.u32(best + 0x10), 0x4444);
        assert_eq!(e.mem.u32(best + 0x2c), 2.0f32.to_bits());
        // A NaN distance never replaces it.
        let unordered = ray(&mut e, f32::NAN, 0x5555);
        e.call(0x00c7_fcc0, &args![node, unordered, best, 0u32]);
        assert_eq!(e.mem.u32(best + 0x10), 0x4444);
    }

    #[test]
    fn ray_test_filters_by_kind_and_layer() {
        let mut e = engine();
        let (node, _, _, body) = ray_scene(&mut e, 0x208);
        answer(&mut e, CAST_TEST, 0);
        let best = e.mem.alloc(0x40);
        let probe = ray(&mut e, 1.0, 1);
        // Kind 8 or 0x1d with layers 2..=16 are tested; layer 1 only without the flag.
        for (filter, flag, tested) in [
            (0x0208u32, 0u32, true),
            (0x021d, 0, true),
            (0x1008, 0, true),
            (0x1108, 0, false),
            (0x0108, 0, true),
            (0x0108, 1, false),
            (0x0008, 0, false),
            (0x0203, 0, false),
        ] {
            e.mem.set_u32(body + 0x2c, filter);
            let before = calls_to(&e, RAY_CAST).len();
            e.call(0x00c7_fcc0, &args![node, probe, best, flag]);
            assert_eq!(
                calls_to(&e, RAY_CAST).len() - before,
                usize::from(tested),
                "filter {filter:#x} flag {flag}"
            );
            e.mem.set_u32(best, 0);
        }
    }

    #[test]
    fn ray_test_combines_the_transforms_of_transformed_holders() {
        let mut e = engine();
        let (node, holder, wrapper, _) = ray_scene(&mut e, 0x208);
        answer(&mut e, CAST_TEST, 1);
        answer(&mut e, IS_SCALED, 0);
        quiet(&mut e, &[NI_TO_HAVOK, TRANSFORM_COPY, TRANSFORM_MUL]);
        let best = e.mem.alloc(0x40);
        let probe = ray(&mut e, 3.0, 9);
        e.call(0x00c7_fcc0, &args![node, probe, best, 0u32]);
        assert_eq!(
            order(&e),
            vec![
                0x00c7_fcc0,
                CAST,
                QUERY_SHAPE,
                CAST_CDECL,
                FILL_D,
                CAST_TEST,
                IS_SCALED,
                NI_TO_HAVOK,
                CAST_TEST,
                TRANSFORM_COPY,
                TRANSFORM_MUL,
                RAY_CAST,
                CHILDREN_D
            ]
        );
        assert_eq!(
            calls_to(&e, CAST_TEST),
            vec![
                vec![TYPE_SCALED_HOLDER, holder],
                vec![TYPE_TRANSFORMED, wrapper]
            ]
        );
        let transform = calls_to(&e, NI_TO_HAVOK)[0].clone();
        assert_eq!(transform[1], node + 0x68);
        assert_eq!(
            calls_to(&e, TRANSFORM_COPY),
            vec![vec![transform[0] + 0xa0, wrapper + 0x20]]
        );
        assert_eq!(
            calls_to(&e, TRANSFORM_MUL),
            vec![vec![transform[0], transform[0] + 0x60, transform[0] + 0xa0]]
        );
        // A scaled holder with a scale other than 1.0 keeps the transform as fetched.
        answer(&mut e, IS_SCALED, 1);
        e.mem.set_u32(holder + 0x18, 2.0f32.to_bits());
        e.call(0x00c7_fcc0, &args![node, probe, best, 0u32]);
        assert_eq!(calls_to(&e, NI_TO_HAVOK).len(), 1);
        e.mem.set_u32(holder + 0x18, 1.0f32.to_bits());
        e.call(0x00c7_fcc0, &args![node, probe, best, 0u32]);
        assert_eq!(calls_to(&e, NI_TO_HAVOK).len(), 2);
    }

    #[test]
    fn ray_test_visits_the_children() {
        let mut e = engine();
        let (child, _, wrapper, _) = ray_scene(&mut e, 0x208);
        answer(&mut e, CAST_TEST, 0);
        let parent = node(&mut e);
        adopt(&mut e, parent, &[child]);
        let best = e.mem.alloc(0x40);
        let probe = ray(&mut e, 3.0, 9);
        assert!(!e
            .call(0x00c7_fcc0, &args![parent, probe, best, 0u32])
            .bool());
        assert_eq!(e.mem.u32(best), wrapper);
        assert!(!e.call(0x00c7_fcc0, &args![0u32, probe, best, 0u32]).bool());
    }

    #[test]
    fn nearest_hit_wrapper_is_answered_by_the_wrapper_function() {
        let mut e = engine();
        let (node, _, wrapper, _) = ray_scene(&mut e, 0x208);
        answer(&mut e, CAST_TEST, 0);
        let probe = ray(&mut e, 3.0, 9);
        assert_eq!(
            e.call(0x00c8_06b0, &args![node, probe, 0u32]).u32(),
            wrapper
        );
        e.register(RAY_CAST, |_, _| ret(0));
        assert_eq!(e.call(0x00c8_06b0, &args![node, probe, 0u32]).u32(), 0);
    }

    #[test]
    fn layer_search_finds_the_first_wrapper_with_the_layer() {
        let mut e = engine();
        install_cast(&mut e);
        let root = node(&mut e);
        let first = node(&mut e);
        let second = node(&mut e);
        let plain = node(&mut e);
        let (_, wrapper_a, _) = collision(&mut e, first, 3 << 8, &[]);
        let (_, wrapper_b, _) = collision(&mut e, second, 5 << 8 | 0x2008, &[]);
        adopt(&mut e, root, &[plain, first, second]);
        assert_eq!(e.call(0x00c7_ff40, &args![root, 5u32]).u32(), wrapper_b);
        assert_eq!(e.call(0x00c7_ff40, &args![root, 3u32]).u32(), wrapper_a);
        assert_eq!(e.call(0x00c7_ff40, &args![root, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00c7_ff40, &args![0u32, 9u32]).u32(), 0);
        // A node whose own wrapper matches answers without looking at its children.
        let before = calls_to(&e, CHILDREN_D).len();
        assert_eq!(e.call(0x00c7_ff40, &args![first, 3u32]).u32(), wrapper_a);
        assert_eq!(calls_to(&e, CHILDREN_D).len(), before);
    }

    #[test]
    fn owner_slot_gets_the_objects_fourth_word() {
        let mut e = engine();
        e.register(OWNER_D, |_, a| ret(a[1] + 0x1000));
        let owner = object(&mut e, &[(0xe0, OWNER_D)]);
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0xc, 9);
        e.mem.set_u32(inner + 8, owner);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target, inner);
        e.mem.set_u32(target + 4, 0x33);
        assert_eq!(e.call(0x00c8_0020, &args![target]).u32(), 0x1033);
        assert_eq!(calls_to(&e, OWNER_D), vec![vec![owner, 0x33]]);
        // Without an owner (an unknown kind) the answer is 0x24.
        e.mem.set_u32(inner + 0xc, 7);
        assert_eq!(e.call(0x00c8_0020, &args![target]).u32(), 0x24);
    }

    #[test]
    fn activity_follows_the_tracker_or_the_first_child() {
        let mut e = engine();
        install_cast(&mut e);
        e.register(TRACKER_D, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        // The activity byte: the entity query answers the pointer kept at body + 0x40.
        e.register(ENTITY_IS_ACTIVE, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        let flag = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(body + 0x40, flag);
        let entity = e.mem.alloc(0x40);
        e.mem.set_u32(entity + 8, body);
        let cast_from = e.mem.alloc(0x40);
        e.mem.set_u32(cast_from + 8, entity);
        let found = e.mem.alloc(0x40);
        e.mem.set_u32(found + 0x10, cast_from);
        let tracker = object(&mut e, &[(0x38, TRACKER_D)]);
        e.mem.set_u32(tracker + 0x40, found);
        let target = e.mem.alloc(0x200);
        e.mem.set_u32(target + 0x1c, tracker);
        assert!(!e.call(0x00c8_0090, &args![target]).bool());
        e.mem.set_u8(flag, 1);
        assert!(e.call(0x00c8_0090, &args![target]).bool());
        assert_eq!(calls_to(&e, ENTITY_IS_ACTIVE)[1][0], body);
        // The tracker answers nothing: the first child's tracker is used when the record's
        // bit 1 is set.
        e.mem.set_u32(tracker + 0x40, 0);
        let record = e.mem.alloc(0x20);
        answer(&mut e, TRACKER_QUERY, record);
        assert!(!e.call(0x00c8_0090, &args![target]).bool());
        e.mem.set_u32(record + 0xc, 2);
        let kid = e.mem.alloc(0x200);
        let tracker_two = object(&mut e, &[(0x38, TRACKER_D)]);
        e.mem.set_u32(tracker_two + 0x40, found);
        e.mem.set_u32(kid + 0x1c, tracker_two);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, kid);
        e.mem.set_u32(target + 0xa0, list);
        e.mem.set_u16(target + 0xa6, 1);
        assert!(e.call(0x00c8_0090, &args![target]).bool());
        // No tracker at all and an unmarked record: not active.
        e.mem.set_u32(target + 0x1c, 0);
        e.mem.set_u32(record + 0xc, 1);
        assert!(!e.call(0x00c8_0090, &args![target]).bool());
        answer(&mut e, TRACKER_QUERY, 0);
        assert!(!e.call(0x00c8_0090, &args![target]).bool());
    }

    #[test]
    fn thread_setup_builds_the_router_once() {
        let mut e = engine();
        mark_scope(&mut e);
        e.register(ALLOC_D, |e, _| ret(e.mem.alloc(0x6000)));
        let allocator = object(&mut e, &[(4, ALLOC_D)]);
        e.register_double(ROUTER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x10, allocator);
            ret(a[0])
        });
        e.register(POOL_ALLOCATE, |e, _| ret(e.mem.alloc(0x40)));
        let system = object(&mut e, &[(0xc, RECORD_D)]);
        answer(&mut e, MEMORY_SYSTEM, system);
        quiet(
            &mut e,
            &[
                RECORD_D,
                IMPORT_ENTER_CRITICAL_SECTION,
                IMPORT_LEAVE_CRITICAL_SECTION,
                INIT_THREAD,
            ],
        );
        let slot = e.mem.alloc(0x20);
        e.call(0x00c8_0130, &args![slot]);
        let router = e.mem.u32(slot);
        let block = e.mem.u32(slot + 4);
        assert_ne!(router, 0);
        assert_ne!(block, 0);
        assert_eq!(
            [0, 4, 8, 0xc, 0x18].map(|offset| e.mem.u32(router + offset)),
            [block, block, block + 0x5000, allocator, allocator]
        );
        assert_eq!(
            order(&e),
            vec![
                0x00c8_0130,
                IMPORT_ENTER_CRITICAL_SECTION,
                ROUTER_CONSTRUCT,
                POOL_ALLOCATE,
                ROUTER_CONSTRUCT,
                MEMORY_SYSTEM,
                RECORD_D,
                INIT_THREAD,
                ALLOC_D,
                IMPORT_LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(
            calls_to(&e, RECORD_D),
            vec![vec![system, router, ROUTER_NAME, 1]]
        );
        assert_eq!(calls_to(&e, ALLOC_D), vec![vec![allocator, 0x5000]]);
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![THREAD_SECTION]]
        );
        assert_eq!(scope(&mut e), 0x77);
        // Already set up: nothing but the scope word.
        let before = order(&e).len();
        e.call(0x00c8_0130, &args![slot]);
        assert_eq!(order(&e).len(), before + 1);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn thread_shutdown_releases_the_router() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FREE_D,
                RECORD_D,
                IMPORT_ENTER_CRITICAL_SECTION,
                IMPORT_LEAVE_CRITICAL_SECTION,
                QUIT_THREAD,
            ],
        );
        answer(&mut e, IMPORT_GET_CURRENT_THREAD_ID, 0x1234);
        let allocator = object(&mut e, &[(8, FREE_D)]);
        let system = object(&mut e, &[(0x10, RECORD_D)]);
        answer(&mut e, MEMORY_SYSTEM, system);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        let slot = e.mem.alloc(0x20);
        e.mem.set_u32(slot, router);
        e.mem.set_u32(slot + 4, 0xb10c);
        e.mem.set_u32(slot + 8, 0x1234);
        e.call(0x00c8_0240, &args![slot]);
        assert_eq!(
            order(&e),
            vec![
                0x00c8_0240,
                IMPORT_ENTER_CRITICAL_SECTION,
                FREE_D,
                IMPORT_GET_CURRENT_THREAD_ID,
                QUIT_THREAD,
                MEMORY_SYSTEM,
                RECORD_D,
                IMPORT_LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(calls_to(&e, FREE_D), vec![vec![allocator, 0xb10c, 0x5000]]);
        assert_eq!(calls_to(&e, RECORD_D), vec![vec![system, router, 1]]);
        assert_eq!(
            [slot, slot + 4, router + 4, router, router + 8].map(|word| e.mem.u32(word)),
            [0; 5]
        );
        // Nothing to do without a router.
        let before = order(&e).len();
        e.call(0x00c8_0240, &args![slot]);
        assert_eq!(order(&e).len(), before + 1);
        // Another thread does not quit the thread.
        e.mem.set_u32(slot, router);
        e.mem.set_u32(slot + 8, 0x9999);
        e.call(0x00c8_0240, &args![slot]);
        assert_eq!(calls_to(&e, QUIT_THREAD).len(), 1);
    }

    #[test]
    fn next_collision_object_is_found_depth_first() {
        let mut e = engine();
        let root = node(&mut e);
        let empty = node(&mut e);
        let deep = node(&mut e);
        let hit = node(&mut e);
        let later = node(&mut e);
        e.mem.set_u32(hit + 0x1c, 0x5555);
        e.mem.set_u32(later + 0x1c, 0x6666);
        adopt(&mut e, deep, &[hit]);
        adopt(&mut e, root, &[0, empty, deep, later]);
        assert_eq!(e.call(0x00c8_02d0, &args![root]).u32(), 0x5555);
        // A node with its own collision object answers it without looking further.
        let before = calls_to(&e, CHILDREN_D).len();
        assert_eq!(e.call(0x00c8_02d0, &args![later]).u32(), 0x6666);
        assert_eq!(calls_to(&e, CHILDREN_D).len(), before);
        assert_eq!(e.call(0x00c8_02d0, &args![empty]).u32(), 0);
    }

    #[test]
    fn collision_flag_bit_is_set_or_cleared_down_the_tree() {
        let mut e = engine();
        let root = node(&mut e);
        let child = node(&mut e);
        let bare = node(&mut e);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(root + 0x1c, first);
        e.mem.set_u32(child + 0x1c, second);
        e.mem.set_u16(first + 0xc, 0x0100);
        e.mem.set_u16(second + 0xc, 0x0001);
        adopt(&mut e, root, &[child, 0, bare]);
        e.call(0x00c8_0330, &args![0x101u32, root]);
        assert_eq!(e.mem.u16(first + 0xc), 0x0120);
        assert_eq!(e.mem.u16(second + 0xc), 0x0021);
        // Only the low byte of the flag matters.
        e.call(0x00c8_0330, &args![0x100u32, root]);
        assert_eq!(e.mem.u16(first + 0xc), 0x0100);
        assert_eq!(e.mem.u16(second + 0xc), 0x0001);
    }

    /// Lets the velocity setters leave the vector they were given at `SAVED` (linear) and
    /// `SAVED + 0x10` (angular).
    fn record_velocities(e: &mut Engine) {
        e.register(SET_LINEAR_VELOCITY, |e, a| {
            let vector = e.mem.bytes(a[1], 16);
            e.mem.write(SAVED, &vector);
            Ret::default()
        });
        e.register(SET_ANGULAR_VELOCITY, |e, a| {
            let vector = e.mem.bytes(a[1], 16);
            e.mem.write(SAVED + 0x10, &vector);
            Ret::default()
        });
        quiet(e, &[NO_OP]);
    }

    fn put_vector(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (index, lane) in lanes.iter().enumerate() {
            e.mem.set_u32(at + 4 * index as u32, lane.to_bits());
        }
    }

    fn saved_vector(e: &Engine, at: u32) -> [u32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
    }

    #[test]
    fn damping_scales_the_velocities_of_every_body() {
        let mut e = engine();
        install_cast(&mut e);
        record_velocities(&mut e);
        let root = node(&mut e);
        let child = node(&mut e);
        let (_, wrapper, body) = collision(&mut e, root, 0, &[]);
        let (_, other, other_body) = collision(&mut e, child, 0, &[]);
        adopt(&mut e, root, &[child]);
        put_vector(&mut e, body + 0x1b0, [1.0, 2.0, 3.0, 4.0]);
        put_vector(&mut e, body + 0x1c0, [5.0, 6.0, 7.0, 8.0]);
        put_vector(&mut e, other_body + 0x1b0, [1.0; 4]);
        put_vector(&mut e, other_body + 0x1c0, [2.0; 4]);
        e.call(0x00c8_03a0, &args![root, 0.5f32.to_bits()]);
        // The child's call came last: its vectors are what is left.
        assert_eq!(
            calls_to(&e, SET_LINEAR_VELOCITY)
                .iter()
                .map(|call| call[0])
                .collect::<Vec<_>>(),
            vec![body, other_body]
        );
        assert_eq!(saved_vector(&e, SAVED), [0.5f32.to_bits(); 4]);
        assert_eq!(saved_vector(&e, SAVED + 0x10), [1.0f32.to_bits(); 4]);
        assert_eq!(
            calls_to(&e, NO_OP),
            vec![
                vec![wrapper],
                vec![wrapper],
                vec![wrapper],
                vec![wrapper],
                vec![other],
                vec![other],
                vec![other],
                vec![other]
            ]
        );
    }

    #[test]
    fn damping_keeps_nan_payloads_and_handles_a_missing_body() {
        let mut e = engine();
        install_cast(&mut e);
        record_velocities(&mut e);
        let root = node(&mut e);
        let (_, wrapper, body) = collision(&mut e, root, 0, &[]);
        put_vector(&mut e, body + 0x1b0, [1.0, 2.0, 3.0, 4.0]);
        // A signalling NaN scale comes out quiet in every lane.
        e.call(0x00c8_03a0, &args![root, 0x7f80_0001u32]);
        assert_eq!(saved_vector(&e, SAVED), [0x7fc0_0001; 4]);
        // A vector lane NaN against a scale: the scale operand (destination) is a number, so
        // the vector's NaN is quieted and kept.
        e.mem.set_u32(body + 0x1b0, 0x7f80_0003);
        e.call(0x00c8_03a0, &args![root, 2.0f32.to_bits()]);
        assert_eq!(e.mem.u32(SAVED), 0x7fc0_0003);
        // Without a body the zero vector stands in and nothing is set.
        e.mem.set_u32(wrapper + 8, 0);
        let before = calls_to(&e, SET_LINEAR_VELOCITY).len();
        e.call(0x00c8_03a0, &args![root, 2.0f32.to_bits()]);
        assert_eq!(calls_to(&e, SET_LINEAR_VELOCITY).len(), before);
        e.call(0x00c8_03a0, &args![0u32, 2.0f32.to_bits()]);
    }

    #[test]
    fn filter_word_low_bits_are_replaced() {
        let mut e = engine();
        install_cast(&mut e);
        quiet(&mut e, &[SET_FILTER_INFO]);
        let root = node(&mut e);
        let child = node(&mut e);
        let (_, wrapper, _) = collision(&mut e, root, 0x8d, &[]);
        let (_, other, _) = collision(&mut e, child, 0x1f00, &[]);
        adopt(&mut e, root, &[child]);
        e.call(0x00c8_04d0, &args![root, 0x72u32]);
        assert_eq!(
            calls_to(&e, SET_FILTER_INFO),
            vec![vec![wrapper, 0xf2], vec![other, 0x1f72]]
        );
        // Without a body the word reads as 0.
        e.mem.set_u32(wrapper + 8, 0);
        e.call(0x00c8_04d0, &args![root, 0x7fu32]);
        assert_eq!(calls_to(&e, SET_FILTER_INFO)[2], vec![wrapper, 0x7f]);
    }

    /// Doubles for the list calls: the assignment copies the item of the node the slot
    /// points to, destruction and free are recorded, the interlocked decrement answers 0.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_ASSIGN, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        quiet(e, &[LIST_NODE_DESTRUCT, POOL_FREE, RECORD_D]);
        answer(e, IMPORT_INTERLOCKED_DECREMENT, 0);
    }

    /// A list item: an object with a deleting slot `+4` and a reference count at `+4`...
    fn list_item(e: &mut Engine) -> u32 {
        object(e, &[(4, RECORD_D)])
    }

    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    #[test]
    fn list_removal_handles_middle_head_and_sole_nodes() {
        let mut e = engine();
        list_doubles(&mut e);
        let (a, b, c) = (list_item(&mut e), list_item(&mut e), list_item(&mut e));
        let last = list_node(&mut e, c, 0);
        let middle = list_node(&mut e, b, last);
        let head = list_node(&mut e, a, middle);
        let slot = e.mem.alloc(8);
        // The middle node is unlinked, destroyed and freed.
        e.mem.set_u32(slot, b);
        e.call(0x00c8_0570, &args![head, slot]);
        assert_eq!(e.mem.u32(head + 4), last);
        assert_eq!(e.mem.u32(middle + 4), 0);
        assert_eq!(calls_to(&e, LIST_NODE_DESTRUCT), vec![vec![middle]]);
        assert_eq!(calls_to(&e, POOL_FREE), vec![vec![POOL_OBJECT, middle]]);
        // The head gives its place up to the second node's item.
        e.mem.set_u32(slot, a);
        e.call(0x00c8_0570, &args![head, slot]);
        assert_eq!(e.mem.u32(head), c);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(calls_to(&e, LIST_NODE_DESTRUCT)[1], vec![last]);
        assert_eq!(calls_to(&e, POOL_FREE)[1], vec![POOL_OBJECT, last]);
        // The sole node releases its item: a count that reaches 0 runs the deleter.
        e.mem.set_u32(slot, c);
        e.call(0x00c8_0570, &args![head, slot]);
        assert_eq!(e.mem.u32(head), 0);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![c + 4]]
        );
        assert_eq!(calls_to(&e, RECORD_D), vec![vec![c]]);
    }

    #[test]
    fn list_removal_leaves_the_list_alone_when_there_is_nothing_to_remove() {
        let mut e = engine();
        list_doubles(&mut e);
        let (a, b) = (list_item(&mut e), list_item(&mut e));
        let head = list_node(&mut e, a, 0);
        let slot = e.mem.alloc(8);
        // Not in the list, null item, and an empty list.
        e.mem.set_u32(slot, b);
        e.call(0x00c8_0570, &args![head, slot]);
        e.mem.set_u32(slot, 0);
        e.call(0x00c8_0570, &args![head, slot]);
        let empty = list_node(&mut e, 0, 0);
        e.mem.set_u32(slot, a);
        e.call(0x00c8_0570, &args![empty, slot]);
        assert_eq!(e.mem.u32(head), a);
        assert_eq!(order(&e).len(), 3);
        // A count that does not reach 0 keeps the item alive.
        answer(&mut e, IMPORT_INTERLOCKED_DECREMENT, 2);
        e.call(0x00c8_0570, &args![head, slot]);
        assert_eq!(e.mem.u32(head), 0);
        assert!(calls_to(&e, RECORD_D).is_empty());
    }

    const RTTI_D: u32 = 0x0900_0500;
    const BODY_D: u32 = 0x0900_0504;
    const BODY2_D: u32 = 0x0900_0508;
    const SET_ID_D: u32 = 0x0900_050c;
    const HOLDER_D: u32 = 0x0900_0510;
    const TRANSFORM_D: u32 = 0x0900_0514;
    const DELETE_D: u32 = 0x0900_0518;
    const CALLBACK_D: u32 = 0x0900_051c;
    /// The vtable of the replacement constraint: slot `+0x9C` is word 0x27.
    const SPRING_SLOTS: usize = 0x28;

    fn array_table(e: &mut Engine) {
        e.put_vtable(VTABLE_ARRAY, &[0, ALLOC_D, FREE_D, 0]);
    }

    #[test]
    fn array_release_frees_the_data_and_clears_the_fields() {
        let mut e = engine();
        quiet(&mut e, &[FREE_D]);
        array_table(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xd47a);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 4);
        e.call(0x00c8_0630, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_ARRAY);
        assert_eq!(calls_to(&e, FREE_D), vec![vec![this, 0xd47a]]);
        assert_eq!([4, 8, 0xc].map(|offset| e.mem.u32(this + offset)), [0; 3]);
        // Without data only the vtable is set.
        e.mem.set_u32(this, 0);
        e.mem.set_u32(this + 8, 3);
        e.call(0x00c8_0630, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_ARRAY);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(calls_to(&e, FREE_D).len(), 1);
    }

    #[test]
    fn array_scalar_deleting_destructor_frees_the_block_on_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[FREE_D, POOL_FREE]);
        array_table(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xd47a);
        assert_eq!(e.call(0x00c8_0660, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, POOL_FREE).is_empty());
        assert_eq!(calls_to(&e, FREE_D), vec![vec![this, 0xd47a]]);
        assert_eq!(e.call(0x00c8_0660, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, POOL_FREE), vec![vec![POOL_OBJECT, this]]);
    }

    /// A body object: `+8` is its Havok body, virtual slot `+0x94` answers its id holder
    /// (the id at `+0x350`) and slot `+0xF4` fills a transform.
    fn body_object(e: &mut Engine, id: u32) -> u32 {
        let holder = e.mem.alloc(0x400);
        e.mem.set_u32(holder + 0x350, id);
        let body = object(e, &[(0x94, HOLDER_D), (0xf4, TRANSFORM_D)]);
        e.mem.set_u32(body + 0x50, holder);
        e.mem.set_u32(body + 8, id + 0x1000);
        body
    }

    /// The link a constraint's body slots answer: the body object is at its `+0xC`.
    fn link_to(e: &mut Engine, body: u32) -> u32 {
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link + 0xc, body);
        link
    }

    /// Doubles for the objects built by the constraint helpers.
    fn constraint_doubles(e: &mut Engine) {
        e.register(RTTI_D, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(BODY_D, |e, a| ret(e.mem.u32(a[0] + 0x48)));
        e.register(BODY2_D, |e, a| ret(e.mem.u32(a[0] + 0x4c)));
        e.register(HOLDER_D, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(KIND_D, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        quiet(e, &[TRANSFORM_D, DELETE_D]);
    }

    /// A data object of the given kind with the pivots at `+0x50` and `+0x90`.
    fn data_object(e: &mut Engine, kind: u32, pivot_a: [f32; 4], pivot_b: [f32; 4]) -> u32 {
        let data = object(e, &[(0x20, KIND_D)]);
        e.mem.set_u32(data + 0x40, kind);
        put_vector(e, data + 0x50, pivot_a);
        put_vector(e, data + 0x90, pivot_b);
        data
    }

    /// A constraint wrapper (RTTI chain reaching the constraint type) whose Havok constraint
    /// (`+8`) has `data` and an owner word `owner`, with the two body links.
    fn constraint_object(e: &mut Engine, data: u32, owner: u32, first: u32, second: u32) -> u32 {
        let chain = e.mem.alloc(8);
        e.mem.set_u32(chain + 4, TYPE_CONSTRAINT);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(havok + 0xc, data);
        e.mem.set_u32(havok + 8, owner);
        let item = object(
            e,
            &[
                (4, DELETE_D),
                (8, RTTI_D),
                (0x9c, SET_ID_D),
                (0xcc, BODY_D),
                (0xd0, BODY2_D),
            ],
        );
        e.mem.set_u32(item + 8, havok);
        e.mem.set_u32(item + 0x44, chain);
        e.mem.set_u32(item + 0x48, first);
        e.mem.set_u32(item + 0x4c, second);
        item
    }

    /// Puts `items` into the constraint list of the node's wrapper (the list head is the
    /// wrapper's `+0x14`, the following cells are blocks) and answers the head cell.
    fn constraint_list(e: &mut Engine, wrapper: u32, items: &[u32]) -> u32 {
        let mut cell = wrapper + 0x14;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(cell, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(cell + 4, next);
                cell = next;
            }
        }
        wrapper + 0x14
    }

    /// A node with a constraint list; answers `(node, wrapper)`.
    fn constrained_node(e: &mut Engine, items: &[u32]) -> (u32, u32) {
        install_cast(e);
        let root = node(e);
        let (_, wrapper, _) = collision(e, root, 0, &[]);
        constraint_list(e, wrapper, items);
        (root, wrapper)
    }

    #[test]
    fn constraint_enabling_sets_the_body_id_and_enables_the_havok_constraint() {
        for kind in [2, 7, 13] {
            let mut e = engine();
            constraint_doubles(&mut e);
            e.register(SET_ID_D, |e, a| {
                let havok = e.mem.u32(a[0] + 8);
                e.mem.set_u32(havok + 8, 0x7777);
                Ret::default()
            });
            quiet(&mut e, &[ENABLE_CONSTRAINT]);
            let body = body_object(&mut e, 0x55);
            let link = link_to(&mut e, body);
            let data = data_object(&mut e, kind, [0.0; 4], [0.0; 4]);
            let item = constraint_object(&mut e, data, 0, link, 0);
            let (root, _) = constrained_node(&mut e, &[item]);
            e.call(0x00c8_0700, &args![root]);
            assert_eq!(
                calls_to(&e, SET_ID_D),
                vec![vec![item, 0x55]],
                "kind {kind}"
            );
            let havok = e.mem.u32(item + 8);
            assert_eq!(
                calls_to(&e, ENABLE_CONSTRAINT),
                vec![vec![havok, 0x7777]],
                "kind {kind}"
            );
        }
    }

    #[test]
    fn constraint_enabling_leaves_owned_and_other_constraints_alone() {
        let mut e = engine();
        constraint_doubles(&mut e);
        quiet(&mut e, &[SET_ID_D, ENABLE_CONSTRAINT]);
        let body = body_object(&mut e, 0x55);
        let link = link_to(&mut e, body);
        // Already owned: no id, but the constraint is enabled again.
        let data = data_object(&mut e, 2, [0.0; 4], [0.0; 4]);
        let owned = constraint_object(&mut e, data, 0x5555, link, 0);
        // Another kind: nothing.
        let other_data = data_object(&mut e, 3, [0.0; 4], [0.0; 4]);
        let other = constraint_object(&mut e, other_data, 0, link, 0);
        // Not a constraint at all: no data lookup.
        let plain = object(&mut e, &[(8, RTTI_D)]);
        let (root, _) = constrained_node(&mut e, &[plain, owned, other]);
        e.call(0x00c8_0700, &args![root]);
        assert!(calls_to(&e, SET_ID_D).is_empty());
        let havok = e.mem.u32(owned + 8);
        assert_eq!(calls_to(&e, ENABLE_CONSTRAINT), vec![vec![havok, 0x5555]]);
        e.call(0x00c8_0700, &args![0u32]);
    }

    #[test]
    fn constraint_enabling_hands_kind_eight_constraints_to_the_body_and_drops_them() {
        let mut e = engine();
        constraint_doubles(&mut e);
        list_doubles(&mut e);
        quiet(&mut e, &[ATTACH_BODY, IMPORT_INTERLOCKED_INCREMENT]);
        let body = body_object(&mut e, 0x66);
        let link = link_to(&mut e, body);
        let data = data_object(&mut e, 8, [0.0; 4], [0.0; 4]);
        let item = constraint_object(&mut e, data, 0, link, 0);
        let (root, wrapper) = constrained_node(&mut e, &[item]);
        e.call(0x00c8_0700, &args![root]);
        assert_eq!(calls_to(&e, ATTACH_BODY), vec![vec![0x66, item]]);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![item + 4]]
        );
        // The list releases it and the caller's own count then reaches 0 too.
        assert_eq!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 2);
        assert_eq!(calls_to(&e, DELETE_D).len(), 2);
        assert_eq!(e.mem.u32(wrapper + 0x14), 0);
        // With a count that stays above 0 the item is not deleted.
        let again = constraint_object(&mut e, data, 0, link, 0);
        constraint_list(&mut e, wrapper, &[again]);
        answer(&mut e, IMPORT_INTERLOCKED_DECREMENT, 1);
        e.call(0x00c8_0700, &args![root]);
        assert_eq!(calls_to(&e, DELETE_D).len(), 2);
    }

    #[test]
    fn constraint_enabling_walks_the_children() {
        let mut e = engine();
        constraint_doubles(&mut e);
        quiet(&mut e, &[SET_ID_D, ENABLE_CONSTRAINT]);
        let body = body_object(&mut e, 0x55);
        let link = link_to(&mut e, body);
        let data = data_object(&mut e, 2, [0.0; 4], [0.0; 4]);
        let item = constraint_object(&mut e, data, 0, link, 0);
        let (child, _) = constrained_node(&mut e, &[item]);
        let parent = node(&mut e);
        adopt(&mut e, parent, &[child]);
        e.call(0x00c8_0700, &args![parent]);
        assert_eq!(calls_to(&e, SET_ID_D), vec![vec![item, 0x55]]);
    }

    /// Doubles for the growth of a pointer array: a fresh 0x100-byte block that keeps the
    /// elements, the capacity word raised by the requested element size.
    fn growing_array(e: &mut Engine) {
        e.register(ARRAY_GROW, |e, a| {
            let array = a[1];
            let old = e.mem.u32(array);
            let count = e.mem.u32(array + 4);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let fresh = e.mem.alloc(0x100);
            if count != 0 {
                let kept = e.mem.bytes(old, count * 4);
                e.mem.write(fresh, &kept);
            }
            e.mem.set_u32(array, fresh);
            e.mem.set_u32(array + 8, capacity + a[2]);
            Ret::default()
        });
    }

    #[test]
    fn rigid_body_gathering_appends_the_bodies_in_tree_order() {
        let mut e = engine();
        install_cast(&mut e);
        growing_array(&mut e);
        let allocator = 0x00a1_10c0;
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        answer(&mut e, MEMORY_ROUTER, router);
        let root = node(&mut e);
        let plain = node(&mut e);
        let child = node(&mut e);
        let (_, _, root_body) = collision(&mut e, root, 0, &[]);
        let (_, _, child_body) = collision(&mut e, child, 0, &[]);
        adopt(&mut e, root, &[plain, child]);
        let array = e.mem.alloc(0x10);
        e.call(0x00c8_0930, &args![root, array]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(
            [0, 4].map(|offset| e.mem.u32(data + offset)),
            [root_body, child_body]
        );
        // Grown once, by four elements.
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![allocator, array, 4]]);
        assert_eq!(e.mem.u32(array + 8) & 0x3fff_ffff, 4);
        e.call(0x00c8_0930, &args![0u32, array]);
        assert_eq!(e.mem.u32(array + 4), 2);
    }

    #[test]
    fn chain_end_gathering_appends_the_answer_for_the_node() {
        let mut e = engine();
        growing_array(&mut e);
        let router = e.mem.alloc(0x40);
        answer(&mut e, MEMORY_ROUTER, router);
        let last = chain(&mut e, &[]);
        let array = e.mem.alloc(0x10);
        e.call(0x00c8_09e0, &args![last, array]);
        assert_eq!(e.mem.u32(array + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(array)), last);
        // A full array is not grown again while it has room.
        e.call(0x00c8_09e0, &args![last, array]);
        assert_eq!(calls_to(&e, ARRAY_GROW).len(), 1);
        assert_eq!(e.mem.u32(array + 4), 2);
    }

    /// Doubles for the list calls of the add functions: the pool hands out 8-byte nodes.
    fn adding_doubles(e: &mut Engine) {
        e.register(POOL_ALLOCATE, |e, _| ret(e.mem.alloc(8)));
        quiet(e, &[IMPORT_INTERLOCKED_INCREMENT]);
        e.register(LIST_ASSIGN, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
    }

    #[test]
    fn adding_at_the_head_links_a_new_node_behind_the_head() {
        let mut e = engine();
        adding_doubles(&mut e);
        let (a, x) = (list_item(&mut e), list_item(&mut e));
        let tail = list_node(&mut e, 0x77, 0);
        let head = list_node(&mut e, a, tail);
        let slot = e.mem.alloc(4);
        // A null item adds nothing.
        e.call(0x00c8_0a30, &args![head, slot]);
        assert_eq!(order(&e), vec![0x00c8_0a30]);
        e.mem.set_u32(slot, x);
        e.call(0x00c8_0a30, &args![head, slot]);
        let node = e.mem.u32(head + 4);
        assert_ne!(node, tail);
        assert_eq!(e.mem.u32(head), x);
        assert_eq!([e.mem.u32(node), e.mem.u32(node + 4)], [a, tail]);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![a + 4]]
        );
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL_OBJECT, 8]]);
        // A head without an item takes the item over.
        let empty = list_node(&mut e, 0, 0);
        e.call(0x00c8_0a30, &args![empty, slot]);
        assert_eq!(e.mem.u32(empty), x);
        assert_eq!(e.mem.u32(empty + 4), 0);
        assert_eq!(calls_to(&e, POOL_ALLOCATE).len(), 1);
    }

    #[test]
    fn adding_at_the_tail_appends_a_node_after_the_last() {
        let mut e = engine();
        adding_doubles(&mut e);
        let (a, b, x) = (list_item(&mut e), list_item(&mut e), list_item(&mut e));
        let last = list_node(&mut e, b, 0);
        let head = list_node(&mut e, a, last);
        let slot = e.mem.alloc(4);
        e.call(0x00c8_0aa0, &args![head, slot]);
        assert_eq!(order(&e), vec![0x00c8_0aa0]);
        e.mem.set_u32(slot, x);
        e.call(0x00c8_0aa0, &args![head, slot]);
        let node = e.mem.u32(last + 4);
        assert_ne!(node, 0);
        assert_eq!([e.mem.u32(node), e.mem.u32(node + 4)], [x, 0]);
        assert_eq!(e.mem.u32(head), a);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![x + 4]]
        );
        // A last node without an item takes the item over.
        let empty = list_node(&mut e, 0, 0);
        e.call(0x00c8_0aa0, &args![empty, slot]);
        assert_eq!(e.mem.u32(empty), x);
        // A failed allocation attaches nothing.
        e.register(POOL_ALLOCATE, |_, _| ret(0));
        e.call(0x00c8_0aa0, &args![head, slot]);
        assert_eq!(e.mem.u32(node + 4), 0);
    }

    type Pairs = Vec<(u32, u32)>;

    fn pair(e: &mut Engine, low: u32, high: u32) -> u32 {
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, low);
        e.mem.set_u32(block + 4, high);
        block
    }

    fn pairs(e: &Engine, array: u32) -> Pairs {
        let data = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|index| (e.mem.u32(data + 8 * index), e.mem.u32(data + 8 * index + 4)))
            .collect()
    }

    /// The element copy of the array (`00c7f900`) as a memory move of 8-byte elements.
    fn moving_copy(e: &mut Engine) {
        e.register(COPY_ELEMENTS, |e, a| {
            if a[2] != 0 {
                let moved = e.mem.bytes(a[1], a[2] * 8);
                e.mem.write(a[0], &moved);
            }
            Ret::default()
        });
    }

    #[test]
    fn pair_insertion_appends_inserts_and_regrows() {
        let mut e = engine();
        moving_copy(&mut e);
        let this = array(&mut e);
        for value in 1..=4 {
            let element = pair(&mut e, value, value * 10);
            let end = e.mem.u32(this + 8);
            e.call(0x00c8_0b20, &args![this, end, element]);
        }
        assert_eq!(pairs(&e, this), vec![(1, 10), (2, 20), (3, 30), (4, 40)]);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // Full: inserting in the middle regrows to twice the capacity.
        let old = e.mem.u32(this + 4);
        let middle = pair(&mut e, 9, 90);
        e.call(0x00c8_0b20, &args![this, 1u32, middle]);
        assert_eq!(
            pairs(&e, this),
            vec![(1, 10), (9, 90), (2, 20), (3, 30), (4, 40)]
        );
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(calls_to(&e, ALLOC_D).last().unwrap(), &vec![this, 8]);
        assert_eq!(calls_to(&e, FREE_D), vec![vec![this, old]]);
        // Room left: the tail moves up in place.
        let front = pair(&mut e, 7, 70);
        let data = e.mem.u32(this + 4);
        e.call(0x00c8_0b20, &args![this, 0u32, front]);
        assert_eq!(e.mem.u32(this + 4), data);
        assert_eq!(
            pairs(&e, this),
            vec![(7, 70), (1, 10), (9, 90), (2, 20), (3, 30), (4, 40)]
        );
        assert_eq!(
            calls_to(&e, COPY_ELEMENTS).last().unwrap(),
            &vec![data + 8, data, 5]
        );
    }

    #[test]
    fn pair_insertion_grows_by_0x400_above_0x400_elements() {
        let mut e = engine();
        moving_copy(&mut e);
        let this = array(&mut e);
        e.register(ALLOC_D, |e, a| ret(e.mem.alloc(a[1] * 8 + 16)));
        let data = e.mem.alloc(0x401 * 8 + 16);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, 0x401);
        e.mem.set_u32(this + 0xc, 0x401);
        let element = pair(&mut e, 5, 6);
        e.call(0x00c8_0b20, &args![this, 0x400u32, element]);
        assert_eq!(e.mem.u32(this + 0xc), 0x801);
        assert_eq!(e.mem.u32(this + 8), 0x402);
        assert_eq!(pairs(&e, this)[0x400], (5, 6));
    }

    /// A node tree for the holder search: `root` has the children `a` (no holder, no
    /// children) and `b` whose child `c` has a holder.
    fn search_scene(e: &mut Engine) -> (u32, u32, u32, u32) {
        install_cast(e);
        moving_copy(e);
        let root = node(e);
        let a = node(e);
        let b = node(e);
        let c = node(e);
        let (holder_c, _, _) = collision(e, c, 0, &[]);
        adopt(e, b, &[c]);
        adopt(e, root, &[a, 0, b]);
        (root, a, b, holder_c)
    }

    #[test]
    fn holder_search_goes_breadth_first_within_the_depth_limit() {
        let mut e = engine();
        let (root, a, _, holder_c) = search_scene(&mut e);
        let this = array(&mut e);
        assert_eq!(
            e.call(0x00c8_0c00, &args![root, this, 5u32, 0u32]).u32(),
            holder_c
        );
        assert_eq!(e.mem.u32(this + 8), 0);
        // The depth limit stops the descent: c is two levels down.
        assert_eq!(e.call(0x00c8_0c00, &args![root, this, 1u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00c8_0c00, &args![root, this, 0u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x00c8_0c00, &args![root, this, 2u32, 0u32]).u32(),
            holder_c
        );
        // A start depth counts against the limit.
        assert_eq!(e.call(0x00c8_0c00, &args![root, this, 2u32, 1u32]).u32(), 0);
        // A node with a holder answers it before anything is queued.
        let (own, _, _) = collision(&mut e, a, 0, &[]);
        let before = e.mem.u32(this + 8);
        assert_eq!(e.call(0x00c8_0c00, &args![a, this, 5u32, 0u32]).u32(), own);
        assert_eq!(e.mem.u32(this + 8), before);
    }

    #[test]
    fn holder_search_leaves_the_unvisited_entries_in_the_array() {
        let mut e = engine();
        install_cast(&mut e);
        moving_copy(&mut e);
        let root = node(&mut e);
        let a = node(&mut e);
        let b = node(&mut e);
        let (holder_a, _, _) = collision(&mut e, a, 0, &[]);
        adopt(&mut e, root, &[a, b]);
        let this = array(&mut e);
        assert_eq!(
            e.call(0x00c8_0c00, &args![root, this, 5u32, 0u32]).u32(),
            holder_a
        );
        // b (depth 1) is still queued.
        assert_eq!(pairs(&e, this), vec![(b, 1)]);
    }

    #[test]
    fn holder_search_with_its_own_array_frees_the_block() {
        let mut e = engine();
        let (root, _, _, holder_c) = search_scene(&mut e);
        array_table(&mut e);
        let block = e.mem.alloc(0x400);
        answer(&mut e, ALLOC_D, block);
        quiet(&mut e, &[FREE_D]);
        assert_eq!(e.call(0x00c8_12d0, &args![root, 5u32]).u32(), holder_c);
        let allocation = calls_to(&e, ALLOC_D);
        assert_eq!(allocation[0][1], 0x20);
        assert_eq!(calls_to(&e, FREE_D), vec![vec![allocation[0][0], block]]);
        assert_eq!(e.call(0x00c8_12d0, &args![root, 0u32]).u32(), 0);
    }

    /// The doubles of the replacement of one stiff spring: the cinfo data block is saved at
    /// `SAVED`, the transform call copies the pivot, the base object constructor and the
    /// init leave the new constraint with a kind-3 data object.
    fn replacement_doubles(e: &mut Engine) {
        constraint_doubles(e);
        quiet(
            e,
            &[
                REF_OBJECT_CONSTRUCT,
                RELEASE_SLOT,
                DISABLE_CONSTRAINT,
                SET_ID_D,
                IMPORT_INTERLOCKED_INCREMENT,
            ],
        );
        e.register(CINFO_SET_DATA, |e, a| {
            let cinfo = e.mem.bytes(a[0], 0x14);
            e.mem.write(SAVED + 0x10, &cinfo);
            Ret::default()
        });
        e.register(CINFO_CREATE_DATA, |e, a| {
            let spring = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 4, spring);
            e.mem.set_u32(SAVED, spring);
            Ret::default()
        });
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let pivot = e.mem.bytes(a[2], 16);
            e.mem.write(a[0], &pivot);
            Ret::default()
        });
        e.register(CONSTRAINT_INIT, |e, a| {
            let vtable = e.mem.alloc(0x100);
            e.mem.set_u32(vtable + 0x20, KIND_D);
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(data, vtable);
            e.mem.set_u32(data + 0x40, 3);
            let havok = e.mem.alloc(0x40);
            e.mem.set_u32(havok + 0xc, data);
            e.mem.set_u32(a[0] + 8, havok);
            Ret::default()
        });
        adding_doubles(e);
        let mut slots = vec![0; SPRING_SLOTS];
        slots[0x9c / 4] = SET_ID_D;
        e.put_vtable(VTABLE_STIFF_SPRING, &slots);
    }

    /// A constraint with two bodies (ids 0x11 and 0x22) in a node of its own; answers
    /// `(node, item, first body, second body, created block)`.
    fn replacement_scene(
        e: &mut Engine,
        kind: u32,
        pivots: ([f32; 4], [f32; 4]),
    ) -> (u32, u32, u32, u32, u32) {
        replacement_doubles(e);
        let first = body_object(e, 0x11);
        let second = body_object(e, 0x22);
        let links = (link_to(e, first), link_to(e, second));
        let data = data_object(e, kind, pivots.0, pivots.1);
        let item = constraint_object(e, data, 0, links.0, links.1);
        let (root, _) = constrained_node(e, &[item]);
        let created = e.mem.alloc(0x40);
        answer(e, ALLOCATE, created);
        (root, item, first, second, created)
    }

    #[test]
    fn replacing_a_constraint_builds_a_stiff_spring_between_the_bodies() {
        let mut e = engine();
        mark_scope(&mut e);
        let pivots = ([4.0, 0.0, 0.0, 1.0], [1.0, 0.0, 4.0, 1.0]);
        let (root, item, first, second, created) = replacement_scene(&mut e, 2, pivots);
        e.call(0x00c8_0ce0, &args![root, 8u32]);
        let spring = e.mem.u32(SAVED);
        assert_eq!(saved_vector(&e, spring + 0x20), pivots.0.map(f32::to_bits));
        assert_eq!(saved_vector(&e, spring + 0x30), pivots.1.map(f32::to_bits));
        // The distance between the pivots, 5.0, is the spring's length.
        assert_eq!(e.mem.u32(spring + 0x44), 5.0f32.to_bits());
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, CONSTRAINT_INIT).len(), 1);
        assert_eq!(e.mem.u32(created), VTABLE_STIFF_SPRING);
        // The new constraint learns the first body's id; the old Havok constraint is not
        // disabled without an owner.
        assert_eq!(calls_to(&e, SET_ID_D), vec![vec![created, 0x11]]);
        assert!(calls_to(&e, DISABLE_CONSTRAINT).is_empty());
        // The transforms are asked of both bodies and applied to the pivots.
        let transforms = calls_to(&e, TRANSFORM_D);
        assert_eq!(transforms.len(), 2);
        assert_eq!(transforms[0][0], first);
        assert_eq!(transforms[1][0], second);
        let positions = calls_to(&e, SET_TRANSFORMED_POS);
        assert_eq!(positions[0][1], transforms[0][1]);
        assert_eq!(positions[1][1], transforms[1][1]);
        assert_eq!(positions[1][0], positions[0][0] + 0x10);
        // The cinfo is cleared at the end and the new constraint is the list's new tail.
        assert_eq!(calls_to(&e, CINFO_SET_DATA).len(), 1);
        assert_eq!(calls_to(&e, CINFO_SET_DATA)[0][1], 0);
        assert_eq!(scope(&mut e), 0x77);
        let wrapper = {
            let tracker = e.mem.u32(root + 0x1c);
            let holder = e.mem.u32(tracker + 8);
            e.mem.u32(holder + 0x10)
        };
        let tail = e.mem.u32(wrapper + 0x18);
        assert_ne!(tail, 0);
        assert_eq!([e.mem.u32(tail), e.mem.u32(tail + 4)], [created, 0]);
        assert_eq!(e.mem.u32(wrapper + 0x14), item);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![created + 4], vec![created + 4]]
        );
        // The cinfo carried the bodies' Havok bodies when it was cleared.
        assert_eq!(e.mem.u32(SAVED + 0x10), VTABLE_CINFO);
        assert_eq!(e.mem.u32(SAVED + 0x1c), 0x1011);
        assert_eq!(e.mem.u32(SAVED + 0x20), 0x1022);
    }

    #[test]
    fn replacing_a_constraint_uses_the_minimum_length_for_coincident_pivots() {
        let mut e = engine();
        e.mem.set_u32(MINIMUM_LENGTH, 0.001f32.to_bits());
        let pivots = ([1.0, 2.0, 3.0, 1.0], [1.0, 2.0, 3.0, 1.0]);
        let (root, _, _, _, _) = replacement_scene(&mut e, 7, pivots);
        e.call(0x00c8_0ce0, &args![root, 8u32]);
        let spring = e.mem.u32(SAVED);
        assert_eq!(e.mem.u32(spring + 0x44), 0.001f32.to_bits());
    }

    #[test]
    fn replacing_a_constraint_disables_an_owned_havok_constraint_and_inserts_after_it() {
        let mut e = engine();
        let pivots = ([4.0, 0.0, 0.0, 1.0], [1.0, 0.0, 4.0, 1.0]);
        let (root, item, _, _, created) = replacement_scene(&mut e, 2, pivots);
        // A second, skipped constraint behind it: the new one goes after the first.
        let other_data = data_object(&mut e, 3, [0.0; 4], [0.0; 4]);
        let other = constraint_object(&mut e, other_data, 0, 0, 0);
        let wrapper = {
            let tracker = e.mem.u32(root + 0x1c);
            let holder = e.mem.u32(tracker + 8);
            e.mem.u32(holder + 0x10)
        };
        constraint_list(&mut e, wrapper, &[item, other]);
        let havok = e.mem.u32(item + 8);
        e.mem.set_u32(havok + 8, 0x4242);
        e.call(0x00c8_0ce0, &args![root, 8u32]);
        assert_eq!(calls_to(&e, DISABLE_CONSTRAINT), vec![vec![havok, 0x4242]]);
        // The head cell's item stays, the second cell holds the new constraint and the
        // old second item follows it.
        let second = e.mem.u32(wrapper + 0x18);
        let third = e.mem.u32(second + 4);
        assert_eq!(e.mem.u32(wrapper + 0x14), item);
        assert_eq!(e.mem.u32(second), created);
        assert_eq!(e.mem.u32(third), other);
    }

    #[test]
    fn replacing_a_malleable_constraint_looks_at_the_wrapped_data() {
        for (wrapped_kind, expected) in [(7, 1), (2, 1), (5, 0)] {
            let mut e = engine();
            let pivots = ([4.0, 0.0, 0.0, 1.0], [1.0, 0.0, 4.0, 1.0]);
            let (root, _, _, _, _) = replacement_scene(&mut e, 13, pivots);
            let wrapped = data_object(&mut e, wrapped_kind, pivots.0, pivots.1);
            e.mem.set_u32(SAVED + 0x40, wrapped);
            e.register(WRAPPED_CONSTRAINT_DATA, |e, _| ret(e.mem.u32(SAVED + 0x40)));
            e.call(0x00c8_0ce0, &args![root, 8u32]);
            assert_eq!(
                calls_to(&e, CINFO_CREATE_DATA).len(),
                expected,
                "wrapped kind {wrapped_kind}"
            );
        }
    }

    #[test]
    fn replacing_constraints_ignores_other_modes_and_incomplete_pairs() {
        let mut e = engine();
        mark_scope(&mut e);
        let pivots = ([4.0, 0.0, 0.0, 1.0], [1.0, 0.0, 4.0, 1.0]);
        let (root, _, _, _, _) = replacement_scene(&mut e, 2, pivots);
        e.call(0x00c8_0ce0, &args![root, 7u32]);
        e.call(0x00c8_0ce0, &args![0u32, 8u32]);
        assert_eq!(order(&e), vec![0x00c8_0ce0, 0x00c8_0ce0]);
        assert_eq!(scope(&mut e), 0x77);
        // A constraint without a second body is left alone.
        let first = body_object(&mut e, 0x11);
        let link = link_to(&mut e, first);
        let data = data_object(&mut e, 2, pivots.0, pivots.1);
        let single = constraint_object(&mut e, data, 0, link, 0);
        let (lonely, _) = constrained_node(&mut e, &[single]);
        e.call(0x00c8_0ce0, &args![lonely, 8u32]);
        assert!(calls_to(&e, CINFO_CREATE_DATA).is_empty());
        // The children are searched with mode 8.
        let parent = node(&mut e);
        adopt(&mut e, parent, &[root]);
        e.call(0x00c8_0ce0, &args![parent, 8u32]);
        assert_eq!(calls_to(&e, CINFO_CREATE_DATA).len(), 1);
    }

    #[test]
    fn debug_geometry_gets_a_coloured_material() {
        let mut e = engine();
        quiet(&mut e, &[ATTACH_PROPERTY]);
        e.register(ALLOCATE, |e, _| ret(e.mem.alloc(0x60)));
        e.register(MATERIAL_CONSTRUCT, |_, a| ret(a[0]));
        let owner = object(&mut e, &[(0xdc, RECORD_D)]);
        quiet(&mut e, &[RECORD_D]);
        let colour = e.mem.alloc(0xc);
        for (index, word) in [0x3f80_0000u32, 0x4000_0000, 0x4040_0000]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(colour + 4 * index as u32, *word);
        }
        e.call(0x00c8_11f0, &args![0x1000u32, owner, 0x2000u32, colour]);
        let property = calls_to(&e, MATERIAL_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(
            [0x28, 0x2c, 0x30].map(|offset| e.mem.u32(property + offset)),
            [0x3f80_0000, 0x4000_0000, 0x4040_0000]
        );
        assert_eq!(e.mem.u32(property + 0x44), 1);
        assert_eq!(calls_to(&e, ATTACH_PROPERTY), vec![vec![0x1000, property]]);
        assert_eq!(calls_to(&e, RECORD_D), vec![vec![owner, 0x1000, 1]]);
        assert!(calls_to(&e, COLOUR_HOOK).is_empty());
    }

    #[test]
    fn debug_geometry_colour_goes_through_the_hook_and_the_callback() {
        let mut e = engine();
        quiet(&mut e, &[ATTACH_PROPERTY, RECORD_D]);
        e.register(ALLOCATE, |e, _| ret(e.mem.alloc(0x60)));
        e.register(MATERIAL_CONSTRUCT, |_, a| ret(a[0]));
        e.register(COLOUR_HOOK, |e, a| {
            e.mem.set_u32(a[1], 0x4120_0000);
            Ret::default()
        });
        e.register(CALLBACK_D, |e, a| {
            let seen = e.mem.bytes(a[1], 12);
            e.mem.write(SAVED, &seen);
            Ret::default()
        });
        e.mem.set_u8(COLOUR_HOOK_FLAG, 1);
        e.mem.set_u32(COLOUR_CALLBACK, CALLBACK_D);
        let owner = object(&mut e, &[(0xdc, RECORD_D)]);
        let colour = e.mem.alloc(0xc);
        e.mem.set_u32(colour, 0x3f80_0000);
        e.mem.set_u32(colour + 4, 0x4000_0000);
        e.call(0x00c8_11f0, &args![0x1000u32, owner, 0x2000u32, colour]);
        let property = calls_to(&e, MATERIAL_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, COLOUR_HOOK)[0][0], 0x2000);
        assert_eq!(
            [0x28, 0x2c, 0x30].map(|offset| e.mem.u32(property + offset)),
            [0x4120_0000, 0x4000_0000, 0]
        );
        // The callback saw the same colour; the caller's own colour is untouched.
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.u32(SAVED + offset)),
            [0x4120_0000, 0x4000_0000, 0]
        );
        assert_eq!(e.mem.u32(colour), 0x3f80_0000);
        assert_eq!(
            order(&e),
            vec![
                0x00c8_11f0,
                COLOUR_HOOK,
                ALLOCATE,
                MATERIAL_CONSTRUCT,
                ATTACH_PROPERTY,
                CALLBACK_D,
                RECORD_D
            ]
        );
    }

    /// The doubles of the tree builder: the capsule description and the rigid body cinfo are
    /// copied out at `SAVED` and `SAVED + 0x100`; the cinfo's array is block `0xd000`
    /// of capacity word 3.
    fn tree_doubles(e: &mut Engine) {
        e.mem.set_u32(HALF, 0.5f32.to_bits());
        e.mem.set_u32(TREE_FLOAT, 0.75f32.to_bits());
        e.mem.set_f64(TREE_DOUBLE, 0.25);
        e.register(ALLOCATE, |e, a| {
            if a[0] == 0x14 && e.mem.u32(SAVED + 0x200) != 0 {
                return ret(0);
            }
            ret(e.mem.alloc(0x40))
        });
        e.register(CAPSULE_CONSTRUCT, |e, a| {
            let description = e.mem.bytes(a[1], 0x30);
            e.mem.write(SAVED, &description);
            e.mem.set_u32(a[0] + 8, 0x1234);
            ret(a[0])
        });
        e.register(RIGID_BODY_CINFO_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0xd000);
            e.mem.set_u32(a[0] + 0x14, 3);
            Ret::default()
        });
        e.register(RIGID_BODY_CONSTRUCT, |e, a| {
            let cinfo = e.mem.bytes(a[1], 0xe0);
            e.mem.write(SAVED + 0x100, &cinfo);
            ret(a[0])
        });
        let allocator = object(e, &[(0x10, FREE_D)]);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        quiet(e, &[FREE_D]);
    }

    #[test]
    fn tree_building_describes_a_capsule_and_wraps_it_in_a_rigid_body() {
        let mut e = engine();
        tree_doubles(&mut e);
        let body = e.call(0x00c8_1380, &args![4.0f32, 1.0f32]).u32();
        assert_eq!(body, calls_to(&e, RIGID_BODY_CONSTRUCT)[0][0]);
        // The capsule description: radius 0.5 (half of 1.0), points at half of the shaft.
        assert_eq!(e.mem.u32(SAVED + 4), 0.5f32.to_bits());
        assert_eq!(e.mem.u32(SAVED + 0x18), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(SAVED + 0x28), 0.5f32.to_bits());
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x14], vec![0x1c]]);
        let capsule = calls_to(&e, CAPSULE_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(capsule + 0x10), 9);
        // The rigid body description.
        let cinfo = SAVED + 0x100;
        assert_eq!(e.mem.u32(cinfo), 0x0004_0009);
        assert_eq!(e.mem.u32(cinfo + 0x20), 0x0004_0009);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0xb0), 0);
        assert_eq!(e.mem.u32(cinfo + 0xc0), 0.75f32.to_bits());
        assert_eq!(e.mem.u8(cinfo + 0xd4), 5);
        // The cinfo's array goes back through the router's allocator: 3 elements of 8 bytes.
        let free = calls_to(&e, FREE_D);
        assert_eq!(free.len(), 1);
        assert_eq!(free[0][1..], [0xd000, 24]);
    }

    #[test]
    fn tree_building_lengthens_a_short_shaft_and_keeps_nan_heights() {
        let mut e = engine();
        tree_doubles(&mut e);
        // Height 1.0 is not above twice the radius 1.0: it becomes 2.0 + 0.25.
        e.call(0x00c8_1380, &args![1.0f32, 1.0f32]);
        assert_eq!(e.mem.u32(SAVED + 0x18), 0.625f32.to_bits());
        // A NaN height is not replaced and stays a (quiet) NaN.
        e.call(0x00c8_1380, &args![f32::NAN, 1.0f32]);
        assert!(f32::from_bits(e.mem.u32(SAVED + 0x18)).is_nan());
    }

    #[test]
    fn tree_building_gives_up_without_a_capsule_block() {
        let mut e = engine();
        tree_doubles(&mut e);
        e.mem.set_u32(SAVED + 0x200, 1);
        assert_eq!(e.call(0x00c8_1380, &args![4.0f32, 1.0f32]).u32(), 0);
        assert!(calls_to(&e, RIGID_BODY_CONSTRUCT).is_empty());
    }

    #[test]
    fn tree_building_keeps_an_array_with_a_negative_capacity_word() {
        let mut e = engine();
        tree_doubles(&mut e);
        e.register(RIGID_BODY_CINFO_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0xd000);
            e.mem.set_u32(a[0] + 0x14, 0x8000_0003);
            Ret::default()
        });
        e.call(0x00c8_1380, &args![4.0f32, 1.0f32]);
        assert!(calls_to(&e, FREE_D).is_empty());
    }

    #[test]
    fn bone_transform_arrays_free_their_block_to_the_pool() {
        let mut e = engine();
        quiet(&mut e, &[POOL_FREE]);
        e.call(0x00cb_9cb0, &args![0x5555u32, 0x1234u32]);
        assert_eq!(calls_to(&e, POOL_FREE), vec![vec![POOL_OBJECT, 0x1234]]);
    }
}
