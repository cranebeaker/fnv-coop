//! `BSHavok/bhkragdolltemplate.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

//!
//! `bhkRagdollTemplate` (0x1c bytes, vtable `010c8dcc`) is an `NiExtraData` (0xc bytes) followed by
//! a `BSSimpleArray` (vtable `010c8d28`, at `+0xc`: buffer `+0x10`, size `+0x14`, reserved
//! `+0x18`) of `NiPointer<bhkRagdollTemplateData>`. `bhkRagdollTemplateData` (0x40 bytes,
//! vtable `010c8d3c`) holds its name (an `NiFixedString`, `+8`), a `BSSimpleArray` of
//! constraint cinfo pointers (vtable `010c8d00`, `+0xc`: buffer `+0x10`, size `+0x14`), a
//! `BSSimpleArray` of constraint type words (vtable `010c8d14`, `+0x1c`: buffer `+0x20`) and the
//! rigid-body values `fRBMass` (`+0x2c`), `fRBRestitution` (`+0x30`), `fRBFriction` (`+0x34`),
//! `fRBRadius` (`+0x38`) and `uiRBMaterial` (`+0x3c`) (Xbox PDB names, offsets checked against
//! the PC code). The array object's own virtual slots `+4` and `+8` allocate and free its buffer.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00aa13e0`, cdecl `(size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`) and its deallocation (`00aa4060`, `(manager, block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memmove of the C runtime (`00ec7230`, cdecl `(destination, source, size)`).
const MEMMOVE: u32 = 0x00ec_7230;
/// The array growth helper (`00cb9ce0`, thiscall on the array: new capacity and old size).
const ARRAY_REALLOCATE: u32 = 0x00cb_9ce0;
/// `bhkRefObject` constructor (`00c87c30`) and `bhkConstraint::Init` (`00c89dd0`, thiscall on
/// the constraint with the cinfo holder's address plus 4, or 0).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_INIT: u32 = 0x00c8_9dd0;
/// Base `NiExtraData` functions: constructor (`00a7b2e0`), destructor body (`00a7b300`),
/// `RegisterStreamables` (`00a7b290`), `SaveBinary` (`00a7b2c0`), `LinkObject` (`00a7b3b0`),
/// `LoadBinary` (`00a7b4a0`), `IsEqual` (`00a7b3d0`) and the function `CopyMembers` jumps to
/// (`00a7b330`).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_LINK_OBJECT: u32 = 0x00a7_b3b0;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_IS_EQUAL: u32 = 0x00a7_b3d0;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// `NiObject` functions: constructor (`00a5d3a0`), destructor body (`00a5d3d0`) and
/// `LoadBinary` (`00a5d610`); the `SaveBinary` that only pops its argument (`00a89af0`).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `NiStream` functions: `GetObjectFromLinkID` (`00a63430`), `SaveFixedString` (`00a641f0`),
/// `LoadFixedString` (`00a64df0`), the link-id reader (`00a645b0`) and the link-id writer
/// (virtual slot `0x30`).
const STREAM_GET_LINK_OBJECT: u32 = 0x00a6_3430;
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// The `NiFixedString` copy (`00a5b690`, cdecl `(text)`) and the global default handle
/// (`0109b220`) a new string starts with.
const FIXED_STRING_ASSIGN: u32 = 0x00a5_b690;
const FIXED_STRING_DEFAULT: u32 = 0x0109_b220;
/// `NiExtraData::SetName` (`00a5b950`, thiscall with the address of a global name handle).
const EXTRA_DATA_SET_NAME: u32 = 0x00a5_b950;
const RAGDOLL_TEMPLATE_NAME: u32 = 0x011f_49e0;
/// Vtables and object counters of the classes the constructors below build.
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const VTABLE_MALLEABLE: u32 = 0x010c_81ac;
const COUNT_MALLEABLE: u32 = 0x0126_830c;
const VTABLE_BALL_AND_SOCKET: u32 = 0x010c_8b0c;
const COUNT_BALL_AND_SOCKET: u32 = 0x0126_8648;
const VTABLE_RAGDOLL_CONSTRAINT: u32 = 0x010c_8bec;
const COUNT_RAGDOLL_CONSTRAINT: u32 = 0x0126_860c;
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
const VTABLE_CAPSULE_SHAPE: u32 = 0x0106_6e2c;
const COUNT_CAPSULE_SHAPE: u32 = 0x0126_8324;
/// `bhkCapsuleShape::Init` (`00cab0e0`, thiscall: the two end points and the radius).
const CAPSULE_SHAPE_INIT: u32 = 0x00ca_b0e0;
/// Vtables of the cinfo holders the constructors and `LoadBinary` build.
const VTABLE_BALL_AND_SOCKET_CINFO: u32 = 0x010c_8cc8;
const VTABLE_RAGDOLL_CINFO: u32 = 0x010c_8ce4;
const VTABLE_HINGE_CINFO: u32 = 0x0108_ff98;
const VTABLE_LIMITED_HINGE_CINFO: u32 = 0x010c_5dc0;
const VTABLE_MALLEABLE_CINFO: u32 = 0x010c_8288;
/// Vtables of the template classes and of their three array types.
const VTABLE_TEMPLATE: u32 = 0x010c_8dcc;
const VTABLE_TEMPLATE_DATA: u32 = 0x010c_8d3c;
const VTABLE_TEMPLATE_ARRAY: u32 = 0x010c_8d28;
const VTABLE_CINFO_ARRAY: u32 = 0x010c_8d00;
const VTABLE_TYPE_ARRAY: u32 = 0x010c_8d14;
/// The buffer-freeing function of each array vtable's slot `+8`, read from the vtable.
const CINFO_ARRAY_FREE_SLOT: u32 = 0x010c_8d08;
const TYPE_ARRAY_FREE_SLOT: u32 = 0x010c_8d1c;

/// `BSSimpleArray` members (Xbox PDB): vtable `+0`, `pBuffer`, `iSize`, `iReservedSize`.
const ARRAY_BUFFER: u32 = 4;
const ARRAY_SIZE: u32 = 8;
const ARRAY_RESERVED: u32 = 0xc;

fn count_up(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// `NiPointer` release: lowers the reference count at `object + 4` and, when it reaches 0,
/// runs the object's virtual slot 4 (the delete).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The array destructor sequence the exe repeats: `buffer` is released through the array's
/// virtual slot `+8` (the array object is at `array`) and the three members are cleared.
fn free_array_buffer(e: &mut Engine, array: u32, buffer: u32) {
    e.vcall(array, 8, &args![buffer]);
    e.mem.set_u32(array + ARRAY_BUFFER, 0);
    e.mem.set_u32(array + 0xc, 0);
    e.mem.set_u32(array + ARRAY_SIZE, 0);
}

/// Releases the buffer of an array whose free routine is the vtable word at `slot`, when it
/// has one, and clears the members (`CALL dword ptr [slot]`, thiscall on the array with the
/// buffer as the one stack word).
fn free_static_array_buffer(e: &mut Engine, array: u32, slot: u32) {
    let buffer = e.mem.u32(array + ARRAY_BUFFER);
    if buffer != 0 {
        let routine = e.mem.u32(slot);
        e.call(routine, &args![array, buffer]);
        e.mem.set_u32(array + ARRAY_BUFFER, 0);
        e.mem.set_u32(array + 0xc, 0);
        e.mem.set_u32(array + ARRAY_SIZE, 0);
    }
}

/// Reads or writes `size` bytes at `buffer` through the stream's file: the function at
/// `function` of the file object found at `stream + file_slot`, called as
/// `(file, buffer, size, &size, 1)`; the count it answers moves the file position (`+4`).
fn transfer(e: &mut Engine, stream: u32, file_slot: u32, function: u32, buffer: u32, size: u32) {
    let file = e.mem.u32(stream + file_slot);
    let routine = e.mem.u32(file + function);
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        e.call(routine, &args![file, buffer, size, slot, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

/// The inline array resize `bhkRagdollTemplateData::LoadBinary` repeats for its two arrays
/// (the elements are neither released nor cleared): `count` of 0 frees an existing buffer;
/// a larger count allocates (virtual slot 4, when the reserved size is 0) or reallocates
/// (`00cb9ce0`) and reserves `count`; otherwise the size becomes `count` and, when that is
/// below the size and at most a quarter of the reserved size, the buffer is trimmed.
fn load_resize(e: &mut Engine, array: u32, count: u32) {
    if count == 0 {
        let buffer = e.mem.u32(array + ARRAY_BUFFER);
        if buffer != 0 {
            free_array_buffer(e, array, buffer);
        }
        return;
    }
    let reserved = e.mem.u32(array + ARRAY_RESERVED);
    if count > reserved {
        if reserved == 0 {
            let buffer = e.vcall(array, 4, &args![count]).u32();
            e.mem.set_u32(array + ARRAY_BUFFER, buffer);
        } else {
            let size = e.mem.u32(array + ARRAY_SIZE);
            e.call(ARRAY_REALLOCATE, &args![array, count, size]);
        }
        e.mem.set_u32(array + ARRAY_SIZE, count);
        e.mem.set_u32(array + ARRAY_RESERVED, count);
    } else {
        let size = e.mem.u32(array + ARRAY_SIZE);
        e.mem.set_u32(array + ARRAY_SIZE, count);
        if count < size && count <= reserved >> 2 {
            e.call(ARRAY_REALLOCATE, &args![array, count, count]);
            e.mem.set_u32(array + ARRAY_RESERVED, count);
        }
    }
}

/// A 0x14-byte cinfo holder from the memory manager (`00aa3e40`) with the vtable `vtable`,
/// `+4` and `+0xc`, `+0x10` cleared and `+8` set to 1; 0 when the manager answers 0.
fn new_cinfo_holder(e: &mut Engine, vtable: u32) -> u32 {
    const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    if block != 0 {
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 1);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x10, 0);
        e.mem.set_u32(block, vtable);
    }
    block
}

// Translated from 00cb9910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::CopyMembers` (engine map name): a jump to `00a7b330` (the
/// `NiExtraData` copy), passing on `this` and its two words.
pub fn bhk_ragdoll_template_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: u32,
    cloning: u32,
) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, destination, cloning]);
}

// Translated from 00cb9920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map calls it `bhkExtraData::IsEqual` (identical code folded with another
/// class's): a jump to `NiExtraData::IsEqual` (`00a7b3d0`); `(this, other)` and the byte answer
/// pass through.
pub fn fn_00cb9920(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(EXTRA_DATA_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00cb9930 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkragdolltemplate.obj`). Thiscall on a `BSSimpleArray`
/// (`this` is not read), `RET 0xc`: moves `count` 4-byte items from `source` to `destination`
/// one item at a time with `memmove` (`00ec7230`, 4 bytes): forwards from the first item when
/// `destination` is below `source`, backwards from the last item when it is above, and not at
/// all when they are equal or `count` is 0.
pub fn fn_00cb9930(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut target = destination;
        let mut left = count;
        while left != 0 {
            let from = source.wrapping_add(target.wrapping_sub(destination));
            e.call(MEMMOVE, &args![target, from, 4u32]);
            target = target.wrapping_add(4);
            left -= 1;
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        if index < 0 {
            return;
        }
        let mut target = destination.wrapping_add((index as u32).wrapping_mul(4));
        let delta = source.wrapping_sub(destination);
        while index >= 0 {
            let from = target.wrapping_add(delta);
            e.call(MEMMOVE, &args![target, from, 4u32]);
            index -= 1;
            target = target.wrapping_sub(4);
        }
    }
}

// Translated from 00cb99a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb99a0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkConstraint` constructor that takes a cinfo holder (the vtable it
/// stores is `bhkConstraint`'s, RTTI-confirmed): the `bhkRefObject` constructor (`00c87c30`),
/// `+0xC` cleared, the vtable, then `bhkConstraint::Init` (`00c89dd0`) with the holder plus 4
/// (0 for no holder) and the counter at `01268188` up by one. Answers `this`.
pub fn fn_00cb99a0(e: &mut Engine, this: Ptr, holder: u32) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr(), VTABLE_CONSTRAINT);
    let adjusted = if holder == 0 { 0 } else { holder + 4 };
    e.call(CONSTRAINT_INIT, &args![this, adjusted]);
    count_up(e, COUNT_CONSTRAINT);
    this
}

// Translated from 00cb9a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9a10 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkMalleableConstraint` constructor that takes a cinfo holder (vtable
/// `010c81ac`, RTTI-confirmed): `bhkRefObject` constructor (`00c87c30`), `+0xC` cleared, the
/// `bhkConstraint` vtable and its counter (`01268188`), the class vtable, `bhkConstraint::Init`
/// (`00c89dd0`) with the holder plus 4 (0 for no holder) and the class counter (`0126830c`) up
/// by one. Answers `this`.
pub fn fn_00cb9a10(e: &mut Engine, this: Ptr, holder: u32) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr(), VTABLE_CONSTRAINT);
    count_up(e, COUNT_CONSTRAINT);
    e.mem.set_u32(this.addr(), VTABLE_MALLEABLE);
    let adjusted = if holder == 0 { 0 } else { holder + 4 };
    e.call(CONSTRAINT_INIT, &args![this, adjusted]);
    count_up(e, COUNT_MALLEABLE);
    this
}

// Translated from 00cb9a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9a90 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkCapsuleShape` constructor that takes the two end points and the
/// radius (vtable `01066e2c`, RTTI-confirmed): `bhkRefObject` constructor (`00c87c30`), the
/// vtables and counters of the chain of base classes (`bhkShape`, `bhkSphereRepShape`,
/// `bhkConvexShape`) and of the class, `+0xC` and `+0x10` cleared, then `bhkCapsuleShape::Init`
/// (`00cab0e0`) on `(vertex_a, vertex_b, radius)`, the radius copied by `FLD`/`FSTP`. Answers
/// `this`.
pub fn fn_00cb9a90(e: &mut Engine, this: Ptr, vertex_a: Ptr, vertex_b: Ptr, radius: f32) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_SHAPE);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    count_up(e, COUNT_SHAPE);
    e.mem.set_u32(this.addr(), VTABLE_SPHERE_REP_SHAPE);
    count_up(e, COUNT_SPHERE_REP_SHAPE);
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_SHAPE);
    count_up(e, COUNT_CONVEX_SHAPE);
    let radius = x87_quiet_bits(radius.to_bits());
    e.mem.set_u32(this.addr(), VTABLE_CAPSULE_SHAPE);
    count_up(e, COUNT_CAPSULE_SHAPE);
    e.call(CAPSULE_SHAPE_INIT, &args![this, vertex_a, vertex_b, radius]);
    this
}

// Translated from 00cb9b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The `bhkBallAndSocketConstraint` constructor that takes a cinfo holder: the
/// `bhkConstraint` one (`00cb99a0`), then the class vtable (`010c8b0c`, RTTI-confirmed) and
/// its counter (`01268648`) up by one. Answers `this`.
pub fn fn_00cb9b30(e: &mut Engine, this: Ptr, holder: u32) -> Ptr {
    fn_00cb99a0(e, this, holder);
    e.mem.set_u32(this.addr(), VTABLE_BALL_AND_SOCKET);
    count_up(e, COUNT_BALL_AND_SOCKET);
    this
}

// Translated from 00cb9b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9b80 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkRagdollConstraint` constructor that takes a cinfo holder (vtable
/// `010c8bec`, RTTI-confirmed): `bhkRefObject` constructor (`00c87c30`), `+0xC` cleared, the
/// `bhkConstraint` vtable and its counter (`01268188`), the class vtable, `bhkConstraint::Init`
/// (`00c89dd0`) with the holder plus 4 (0 for no holder) and the class counter (`0126860c`) up
/// by one. Answers `this`.
pub fn fn_00cb9b80(e: &mut Engine, this: Ptr, holder: u32) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr(), VTABLE_CONSTRAINT);
    count_up(e, COUNT_CONSTRAINT);
    e.mem.set_u32(this.addr(), VTABLE_RAGDOLL_CONSTRAINT);
    let adjusted = if holder == 0 { 0 } else { holder + 4 };
    e.call(CONSTRAINT_INIT, &args![this, adjusted]);
    count_up(e, COUNT_RAGDOLL_CONSTRAINT);
    this
}

// Translated from 00cb9c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Constructs a `hkBallAndSocketConstraintCinfo` (vtable `010c8cc8`,
/// RTTI-confirmed) in `this`: `+4` cleared, `+8` (priority) 1, `+0xC` and `+0x10` cleared.
/// Answers `this`.
pub fn fn_00cb9c30(e: &mut Engine, this: Ptr) -> Ptr {
    construct_cinfo_holder(e, this, VTABLE_BALL_AND_SOCKET_CINFO)
}

// Translated from 00cb9c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Constructs a `hkRagdollConstraintCinfo` (vtable `010c8ce4`, RTTI-confirmed) in
/// `this`: `+4` cleared, `+8` (priority) 1, `+0xC` and `+0x10` cleared. Answers `this`.
pub fn fn_00cb9c50(e: &mut Engine, this: Ptr) -> Ptr {
    construct_cinfo_holder(e, this, VTABLE_RAGDOLL_CINFO)
}

/// The body of the two cinfo-holder constructors.
fn construct_cinfo_holder(e: &mut Engine, this: Ptr, vtable: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, 1);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me, vtable);
    this
}

// Translated from 00cb9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::RegisterStreamables` (Xbox PDB), `(stream)`: the `NiExtraData`
/// registration (`00a7b290`); when it answers true, runs virtual slot `0x54` (the same
/// registration) with the stream on every item of the array (the size is read once, the
/// buffer each time). Answers the base's answer.
pub fn bhk_ragdoll_template_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    let registered = e
        .call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u32();
    if registered as u8 != 0 {
        let count = e.mem.u32(this.addr() + 0x14);
        let mut index = 0;
        while index < count {
            let item = e.mem.u32(e.mem.u32(this.addr() + 0x10) + 4 * index);
            e.vcall(item, 0x54, &args![stream]);
            index += 1;
        }
    }
    registered as u8 != 0
}

// Translated from 00cb9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall on a `BSSimpleArray` (`this` is not read), `RET 8`: clears `count`
/// pointers from `pointers` on, skipping a null slot address (the `NiPointer` constructor
/// loop).
pub fn fn_00cb9d50(e: &mut Engine, _this: Ptr, pointers: u32, count: u32) {
    let mut slot = pointers;
    let mut left = count;
    while left != 0 {
        if slot != 0 {
            e.mem.set_u32(slot, 0);
        }
        slot = slot.wrapping_add(4);
        left -= 1;
    }
}

// Translated from 00cb9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::SaveBinary` (Xbox PDB), `(stream)`: the base `SaveBinary`
/// (`00a89af0`), the name through `NiStream::SaveFixedString` (`00a641f0`, the address of
/// `+8`), then through the write function of the stream's file (`[stream + 0x250]`, function
/// at `+0xC`, `(file, source, 4, &4, 1)`, each count added to the file position at `+4`) the
/// five words at `+0x2c` .. `+0x3c`, the item count (the size at `+0x14`, read once) and that
/// many words of the type array (buffer at `+0x20`); then it runs the virtual slot `0xc` (the
/// cinfo's `SaveBinary`) with the stream on each cinfo of the buffer at `+0x10`.
pub fn bhk_ragdoll_template_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let me = this.addr();
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    let count = e.mem.u32(me + 0x14);
    e.call(STREAM_SAVE_FIXED_STRING, &args![stream, me + 8]);
    for offset in [0x2cu32, 0x30, 0x34, 0x38, 0x3c] {
        transfer(e, stream.addr(), 0x250, 0xc, me + offset, 4);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), count);
        transfer(e, stream.addr(), 0x250, 0xc, local.addr(), 4);
    });
    let mut index = 0;
    while index < count {
        let value = e.mem.u32(e.mem.u32(me + 0x20) + 4 * index);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), value);
            transfer(e, stream.addr(), 0x250, 0xc, local.addr(), 4);
        });
        index += 1;
    }
    let mut index = 0;
    while index < count {
        let cinfo = e.mem.u32(e.mem.u32(me + 0x10) + 4 * index);
        e.vcall(cinfo, 0xc, &args![stream]);
        index += 1;
    }
}

// Translated from 00cb9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::LinkObject` (Xbox PDB), `(stream)`: the `NiExtraData` link (`00a7b3b0`),
/// then for each item index below the size at `+0x14` (read again each time) replaces the
/// pointer in the buffer at `+0x10` by the object the stream answers (`00a63430`, on the
/// stream, no arguments): when it differs, releases the old one (`InterlockedDecrement`;
/// virtual slot 4 at 0), stores the new and takes a reference on it (`InterlockedIncrement`).
pub fn bhk_ragdoll_template_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let me = this.addr();
    e.call(EXTRA_DATA_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(me + 0x14) == 0 {
        return;
    }
    let mut index = 0;
    loop {
        let slot = e.mem.u32(me + 0x10).wrapping_add(4 * index);
        let new = e.call(STREAM_GET_LINK_OBJECT, &args![stream]).u32();
        let old = e.mem.u32(slot);
        if old != new {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(slot, new);
            if new != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
            }
        }
        index += 1;
        if index >= e.mem.u32(me + 0x14) {
            break;
        }
    }
}

// Translated from 00cb9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::SaveBinary` (Xbox PDB), `(stream)`: the `NiExtraData` save
/// (`00a7b2c0`), then the item count (the size at `+0x14`) through the write function of the
/// stream's file (`[stream + 0x250]`, function at `+0xC`, `(file, &count, 4, &4, 1)`, the
/// count added to the file position at `+4`) and the stream's virtual slot `0x30` (the link id
/// writer) on each item of the buffer at `+0x10`.
pub fn bhk_ragdoll_template_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let me = this.addr();
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let count = e.mem.u32(me + 0x14);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), count);
        transfer(e, stream.addr(), 0x250, 0xc, local.addr(), 4);
    });
    let mut index = 0;
    while index < count {
        let item = e.mem.u32(e.mem.u32(me + 0x10) + 4 * index);
        e.vcall(stream.addr(), 0x30, &args![item]);
        index += 1;
    }
}

// Translated from 00cb9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The destructor body of the `BSSimpleArray<hkConstraintCinfo *>` (vtable
/// `010c8d00`, RTTI-confirmed): stores the vtable and, when there is a buffer, passes it to
/// the function at the vtable's slot `+8` (`010c8d08`) and clears the buffer, reserved size
/// and size.
pub fn fn_00cb9fd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CINFO_ARRAY);
    free_static_array_buffer(e, this.addr(), CINFO_ARRAY_FREE_SLOT);
}

// Translated from 00cba000 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The destructor body of the `BSSimpleArray<ConstraintType>` (vtable
/// `010c8d14`, RTTI-confirmed): stores the vtable and, when there is a buffer, passes it to
/// the function at the vtable's slot `+8` (`010c8d1c`) and clears the buffer, reserved size
/// and size.
pub fn fn_00cba000(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TYPE_ARRAY);
    free_static_array_buffer(e, this.addr(), TYPE_ARRAY_FREE_SLOT);
}

// Translated from 00cba030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<hkConstraintCinfo *,1024>::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body (`00cb9fd0`), then the memory manager's deallocation (`00aa4060` on
/// `011f6238`) of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn bhk_cinfo_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00cb9fd0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00cba080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<hkpConstraintData::ConstraintType,1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor body (`00cba000`), then the memory manager's deallocation (`00aa4060`
/// on `011f6238`) of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn bhk_type_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00cba000(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00cba0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall on a `BSSimpleArray` (`this` is not read), `RET 8`: for each of
/// `count` pointers from `pointers` on, a non-null one is released (`InterlockedDecrement` on
/// `+4`; virtual slot 4 at 0).
pub fn fn_00cba0d0(e: &mut Engine, _this: Ptr, pointers: u32, count: u32) {
    let mut index = 0;
    while index < count {
        let object = e.mem.u32(pointers.wrapping_add(4 * index));
        if object != 0 {
            release(e, object);
        }
        index += 1;
    }
}

// Translated from 00cba110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cba110 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkRagdollTemplateData` constructor (vtable `010c8d3c`, RTTI-confirmed):
/// the `NiObject` constructor (`00a5d3a0`), the vtable, the name handle `+8` from the global
/// default (`0109b220`), the two arrays (vtables `010c8d00` at `+0xc` and `010c8d14` at `+0x1c`,
/// their buffer, size and reserved members cleared); when the name handle is not 0, replaces it
/// by `00a5b690(0)` and releases the old one (`InterlockedDecrement` on the handle minus 8)
/// unless it is the global default. Answers `this`.
pub fn fn_00cba110(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE_TEMPLATE_DATA);
    let default_handle: u32 = e.global(FIXED_STRING_DEFAULT);
    e.mem.set_u32(me + 8, default_handle);
    e.mem.set_u32(me + 0xc, VTABLE_CINFO_ARRAY);
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_u32(me + offset, 0);
    }
    e.mem.set_u32(me + 0x1c, VTABLE_TYPE_ARRAY);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(me + offset, 0);
    }
    let old = e.mem.u32(me + 8);
    if old != 0 {
        let fresh = e.call(FIXED_STRING_ASSIGN, &args![0u32]).u32();
        e.mem.set_u32(me + 8, fresh);
        if old != e.global::<u32>(FIXED_STRING_DEFAULT) {
            e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
        }
    }
    this
}

// Translated from 00cba1d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cba1d0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The destructor body of `bhkRagdollTemplateData`: stores the vtable; deletes
/// (virtual slot 0 with 1) each non-null cinfo of the buffer at `+0x10` (the size at `+0x14` is
/// read again each time); frees that buffer through the array's virtual slot `+8` and clears
/// the array; the type array (`+0x1c`) gets its vtable and frees its buffer through
/// the function at `010c8d1c`; the cinfo array gets its vtable again and frees a remaining
/// buffer through the function at `010c8d08`; the name handle is released
/// (`InterlockedDecrement` on the handle minus 8) unless it is the global default (`0109b220`);
/// last the `NiObject` destructor body (`00a5d3d0`).
pub fn fn_00cba1d0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_TEMPLATE_DATA);
    let size = e.mem.u32(me + 0x14);
    if size != 0 {
        let mut index = 0;
        loop {
            let cinfo = e.mem.u32(e.mem.u32(me + 0x10) + 4 * index);
            if cinfo != 0 {
                e.vcall(cinfo, 0, &args![1u32]);
            }
            index += 1;
            if index >= e.mem.u32(me + 0x14) {
                break;
            }
        }
    }
    let cinfos = me + 0xc;
    let buffer = e.mem.u32(me + 0x10);
    if buffer != 0 {
        free_array_buffer(e, cinfos, buffer);
    }
    let types = me + 0x1c;
    e.mem.set_u32(types, VTABLE_TYPE_ARRAY);
    free_static_array_buffer(e, types, TYPE_ARRAY_FREE_SLOT);
    e.mem.set_u32(cinfos, VTABLE_CINFO_ARRAY);
    free_static_array_buffer(e, cinfos, CINFO_ARRAY_FREE_SLOT);
    let name = e.mem.u32(me + 8);
    if name != e.global::<u32>(FIXED_STRING_DEFAULT) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00cba2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cba2d0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. Allocates a `bhkRagdollTemplateData` (0x40 bytes, `operator new`) and, when
/// that succeeded, constructs it (`00cba110`). Answers the object, or 0.
pub fn fn_00cba2d0(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00cba110(e, Ptr::new(block))
}

// Translated from 00cba340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::LoadBinary` (Xbox PDB), `(stream)`: the `NiObject` load
/// (`00a5d610`), the name through `NiStream::LoadFixedString` (`00a64df0`, the address of `+8`),
/// then through the read function of the stream's file (`[stream + 0x24C]`, function at `+8`,
/// `(file, destination, 4, &4, 1)`, each count added to the file position at `+4`) the five
/// words at `+0x2c` .. `+0x3c` and the item count. Both arrays (`+0xc`, `+0x1c`) are resized to
/// the count without releasing items; then the count words of the type array (buffer at
/// `+0x20`) are read and, for each item, the cinfo of its type is built (the type words
/// 0, 1, 2, 7 and 13 give a cinfo holder of the ball-and-socket, hinge, limited hinge, ragdoll
/// and malleable cinfo classes, RTTI-confirmed: 0x14 bytes from the memory manager; a failed
/// allocation stores 0; other types build nothing) and stored in the buffer at `+0x10`, and
/// the item's `LoadBinary` (virtual slot 8) runs with the stream. An item that was not built is
/// whatever the buffer held.
pub fn bhk_ragdoll_template_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let me = this.addr();
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_LOAD_FIXED_STRING, &args![stream, me + 8]);
    for offset in [0x2cu32, 0x30, 0x34, 0x38, 0x3c] {
        transfer(e, stream.addr(), 0x24c, 8, me + offset, 4);
    }
    let count = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        transfer(e, stream.addr(), 0x24c, 8, local.addr(), 4);
        e.mem.u32(local.addr())
    });
    load_resize(e, me + 0xc, count);
    load_resize(e, me + 0x1c, count);
    let mut index = 0;
    while index < count {
        let value = e.with_stack(4, |e, local| {
            transfer(e, stream.addr(), 0x24c, 8, local.addr(), 4);
            e.mem.u32(local.addr())
        });
        let types = e.mem.u32(me + 0x20);
        e.mem.set_u32(types + 4 * index, value);
        index += 1;
    }
    let mut index = 0;
    while index < count {
        let kind = e.mem.u32(e.mem.u32(me + 0x20) + 4 * index);
        let vtable = match kind {
            0 => Some(VTABLE_BALL_AND_SOCKET_CINFO),
            1 => Some(VTABLE_HINGE_CINFO),
            2 => Some(VTABLE_LIMITED_HINGE_CINFO),
            7 => Some(VTABLE_RAGDOLL_CINFO),
            13 => Some(VTABLE_MALLEABLE_CINFO),
            _ => None,
        };
        if let Some(vtable) = vtable {
            let cinfo = new_cinfo_holder(e, vtable);
            let buffer = e.mem.u32(me + 0x10);
            e.mem.set_u32(buffer + 4 * index, cinfo);
        }
        let cinfo = e.mem.u32(e.mem.u32(me + 0x10) + 4 * index);
        e.vcall(cinfo, 8, &args![stream]);
        index += 1;
    }
}

/// The constraint classes `00cba6f0` builds, by the type word of the template's cinfo.
#[derive(Clone, Copy)]
enum ConstraintKind {
    BallAndSocket,
    Hinge,
    LimitedHinge,
    Ragdoll,
    Malleable,
}

/// `hkBallAndSocketConstraintCinfo::ApplyScale`, `hkHingeConstraintCinfo::ApplyScale` and
/// `hkMalleableConstraintCinfo::ApplyScale` (cdecl `(scale, data)`).
const APPLY_SCALE_BALL_AND_SOCKET: u32 = 0x00cc_96b0;
const APPLY_SCALE_HINGE: u32 = 0x00cb_d9e0;
const APPLY_SCALE_MALLEABLE: u32 = 0x00ca_9940;
/// The constructors of the hinge (`009ba410`) and limited hinge (`00c88380`) constraints that
/// take a cinfo holder (thiscall on the new block; the engine map does not name them).
const HINGE_CONSTRUCT: u32 = 0x009b_a410;
const LIMITED_HINGE_CONSTRUCT: u32 = 0x00c8_8380;

impl ConstraintKind {
    fn apply_scale(self) -> u32 {
        match self {
            ConstraintKind::BallAndSocket => APPLY_SCALE_BALL_AND_SOCKET,
            ConstraintKind::Hinge | ConstraintKind::LimitedHinge | ConstraintKind::Ragdoll => {
                APPLY_SCALE_HINGE
            }
            ConstraintKind::Malleable => APPLY_SCALE_MALLEABLE,
        }
    }

    /// Runs the constructor of the class on the new block with the cinfo holder.
    fn construct(self, e: &mut Engine, block: u32, holder: u32) -> u32 {
        match self {
            ConstraintKind::BallAndSocket => fn_00cb9b30(e, Ptr::new(block), holder).addr(),
            ConstraintKind::Hinge => e.call(HINGE_CONSTRUCT, &args![block, holder]).u32(),
            ConstraintKind::LimitedHinge => {
                e.call(LIMITED_HINGE_CONSTRUCT, &args![block, holder]).u32()
            }
            ConstraintKind::Ragdoll => fn_00cb9b80(e, Ptr::new(block), holder).addr(),
            ConstraintKind::Malleable => fn_00cb9a10(e, Ptr::new(block), holder).addr(),
        }
    }
}

/// The step of `00cba6f0` for one cinfo `element` of the template (`offset` = 4 times its
/// index): scales its data by `scale` (`ApplyScale`), builds the constraint of its class from
/// it, and, unless the scale is 0 (a NaN is not), clones the data with the clone process
/// (`bhkConstraintManager::Clone`, `00cf4a30`), hands the clone to the element's `SetData`
/// (`00c89ac0`) and drops the clone's reference (`00c905b0`); finally scales the data by the
/// inverse (`1.0 / scale` in x87 precision, stored as a float). Answers the new constraint.
fn rescale_and_construct(
    e: &mut Engine,
    this: u32,
    offset: u32,
    kind: ConstraintKind,
    scale: f32,
    process: u32,
) -> u32 {
    const CLONE: u32 = 0x00cf_4a30;
    const SET_DATA: u32 = 0x00c8_9ac0;
    const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
    let element_at = |e: &Engine| e.mem.u32(e.mem.u32(this + 0x10).wrapping_add(offset));
    let element = element_at(e);
    let data = e.mem.u32(element + 4);
    let scale_bits = x87_quiet_bits(scale.to_bits());
    e.call(kind.apply_scale(), &args![scale_bits, data]);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let constraint = if block == 0 {
        0
    } else {
        let element = element_at(e);
        kind.construct(e, block, element)
    };
    if scale != 0.0 {
        let element = element_at(e);
        let data = e.mem.u32(element + 4);
        let clone = e.call(CLONE, &args![data, process]).u32();
        if clone != 0 {
            let element = element_at(e);
            e.call(SET_DATA, &args![element, clone]);
            e.call(REMOVE_REFERENCE, &args![clone]);
        }
    }
    let inverse = (1.0f64 / f64::from(x87_quiet(scale))) as f32;
    let element = element_at(e);
    let data = e.mem.u32(element + 4);
    e.call(
        kind.apply_scale(),
        &args![x87_quiet_bits(inverse.to_bits()), data],
    );
    constraint
}

// Translated from 00cba6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cba6f0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. A `bhkRagdollTemplateData` method, `(source, scale)` (`RET 8`; the scale is a
/// float): rebuilds the Havok body of `source` from the template. Nothing happens unless
/// `source + 0x1c` casts (`00653290`, to the type at `012043f8`) to an object whose `+0x10`
/// (the body holder) has a body at `+8`. The template's mass (`+0x2c`) goes to the body
/// (virtual slot `0xc` of what `00ca86c0` answers, else `00ca88c0`); its friction (`+0x34`) and
/// restitution (`+0x30`) to the body's `+0x90` and `+0x94`; its material (`+0x3c`) to the shape
/// owner; a `bhkCapsuleShape` (`00cb9a90`) is built from the shape's end points (`+0x20`,
/// `+0x30`; the shared zero vector `01267e30` for none) and the radius (`+0x38`) times the
/// scale, takes the material and is set as the body's shape (virtual slot `0xc`); then for
/// each cinfo of the template, when the holder has a constraint (`+0x14`): that constraint's
/// cinfo (`CreateCinfo`, virtual `+0xb4`) gives its `+0xc` and `+0x10` words to the template's
/// cinfo; a constraint of the cinfo's type (0, 1, 2, 7 and 13) is built from it
/// ([`rescale_and_construct`]); it replaces the holder's constraint (reference counted) and
/// gets the world word (the holder's virtual `+0x94` answer's `+0x350`) through its virtual
/// slot `0x9c`. For any other type nothing was built and that virtual call reads through a
/// null object. The local clone process (`004ad0c0`, scale 1.0 in all three components) is
/// deleted with its two maps' virtual slot 0 after each item.
pub fn fn_00cba6f0(e: &mut Engine, this: Ptr, source: Ptr, scale: f32) {
    const BODY_CAST_TYPE: u32 = 0x0120_43f8;
    const CAST: u32 = 0x0065_3290;
    const BODY_TARGET: u32 = 0x00ca_86c0;
    const SET_MASS: u32 = 0x00ca_88c0;
    const NOTHING: u32 = 0x00a2_9680;
    const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d0c0;
    const ZERO_VECTOR: u32 = 0x0126_7e30;
    let me = this.addr();
    let owner = e.mem.u32(source.addr() + 0x1c);
    if owner == 0 {
        return;
    }
    let cast = e.call(CAST, &args![owner, BODY_CAST_TYPE]).u32();
    if cast == 0 {
        return;
    }
    let holder = e.mem.u32(cast + 0x10);
    if holder == 0 {
        return;
    }
    let body = e.mem.u32(holder + 8);
    if body == 0 {
        return;
    }
    let mass = x87_quiet_bits(e.mem.u32(me + 0x2c));
    let target = e.call(BODY_TARGET, &args![body]).u32();
    if target != 0 {
        e.vcall(target, 0xc, &args![mass]);
    } else {
        let again = e.mem.u32(holder + 8);
        if again != 0 {
            e.call(SET_MASS, &args![again, mass]);
        }
    }
    let friction = x87_quiet_bits(e.mem.u32(me + 0x34));
    let body_now = e.mem.u32(holder + 8);
    e.mem.set_u32(body_now + 0x90, friction);
    let restitution = x87_quiet_bits(e.mem.u32(me + 0x30));
    e.mem.set_u32(body_now + 0x94, restitution);
    let mut shape_owner = 0;
    if body_now != 0 {
        e.call(NOTHING, &args![holder]);
        e.call(NOTHING, &args![holder]);
        let node = e.mem.u32(body_now + 0x10);
        if node != 0 {
            shape_owner = e.mem.u32(node + 8);
        }
    }
    let material = e.mem.u32(me + 0x3c);
    e.mem.set_u32(shape_owner + 0x10, material);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let mut capsule = 0;
    if block != 0 {
        let shape = e.mem.u32(shape_owner + 8);
        let (vertex_a, vertex_b) = if shape == 0 {
            (ZERO_VECTOR, ZERO_VECTOR)
        } else {
            (shape + 0x20, shape + 0x30)
        };
        let radius = x87_quiet(f32::from_bits(e.mem.u32(me + 0x38)));
        let radius = (f64::from(radius) * f64::from(x87_quiet(scale))) as f32;
        capsule = fn_00cb9a90(
            e,
            Ptr::new(block),
            Ptr::new(vertex_a),
            Ptr::new(vertex_b),
            radius,
        )
        .addr();
    }
    e.call(INTERLOCKED_INCREMENT, &args![capsule + 4]);
    let material = e.mem.u32(me + 0x3c);
    e.mem.set_u32(capsule + 0x10, material);
    let shape = e.mem.u32(capsule + 8);
    e.vcall(body, 0xc, &args![shape]);
    release(e, shape_owner);
    let world = e.vcall(holder, 0x94, &args![]).u32();
    let world_word = if world == 0 {
        0
    } else {
        e.mem.u32(world + 0x350)
    };
    if e.mem.u32(me + 0x14) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let constraint = e.mem.u32(holder + 0x14);
        if constraint != 0 {
            let offset = index.wrapping_mul(4);
            let cinfo = e
                .with_stack(4, |e, created| e.vcall(constraint, 0xb4, &args![created]))
                .u32();
            let block = if cinfo == 0 { 0 } else { cinfo.wrapping_sub(4) };
            let element = e.mem.u32(e.mem.u32(me + 0x10).wrapping_add(offset));
            let word = e.mem.u32(block + 0xc);
            e.mem.set_u32(element + 0xc, word);
            let element = e.mem.u32(e.mem.u32(me + 0x10).wrapping_add(offset));
            let word = e.mem.u32(block + 0x10);
            e.mem.set_u32(element + 0x10, word);
            e.with_stack(0x1c, |e, process| {
                e.call(CLONING_PROCESS_CONSTRUCT, &args![process, 0x101u32]);
                let one = 1.0f32.to_bits();
                e.mem.set_u32(process.addr() + 0x18, one);
                e.mem.set_u32(process.addr() + 0x14, one);
                e.mem.set_u32(process.addr() + 0x10, one);
                let kind = e.mem.u32(e.mem.u32(me + 0x20).wrapping_add(offset));
                let kind = match kind {
                    0 => Some(ConstraintKind::BallAndSocket),
                    1 => Some(ConstraintKind::Hinge),
                    2 => Some(ConstraintKind::LimitedHinge),
                    7 => Some(ConstraintKind::Ragdoll),
                    13 => Some(ConstraintKind::Malleable),
                    _ => None,
                };
                let new = match kind {
                    Some(kind) => rescale_and_construct(e, me, offset, kind, scale, process.addr()),
                    None => 0,
                };
                if new != 0 {
                    e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
                    let old = e.mem.u32(holder + 0x14);
                    if old != new {
                        if old != 0 {
                            release(e, old);
                        }
                        e.mem.set_u32(holder + 0x14, new);
                        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
                    }
                    release(e, new);
                }
                // The exe calls through the object also when nothing was built (the vtable
                // read then faults on address 0).
                e.vcall(new, 0x9c, &args![world_word]);
                for slot in [0u32, 4] {
                    let map = e.mem.u32(process.addr() + slot);
                    if map != 0 {
                        e.vcall(map, 0, &args![1u32]);
                    }
                }
            });
        }
        index += 1;
        if index >= e.mem.u32(me + 0x14) {
            break;
        }
    }
}

// Translated from 00cbadb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The destructor body of the `BSSimpleArray<NiPointer<bhkRagdollTemplateData>>`
/// (vtable `010c8d28`, RTTI-confirmed): stores the vtable and, when there is a buffer,
/// releases its items (`00cba0d0` on the buffer and the size), frees the buffer through the
/// array's virtual slot `+8` and clears the buffer, reserved size and size.
pub fn fn_00cbadb0(e: &mut Engine, this: Ptr) {
    destruct_template_array(e, this.addr());
}

/// The body of the destructor of the template array.
fn destruct_template_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, VTABLE_TEMPLATE_ARRAY);
    let buffer = e.mem.u32(array + ARRAY_BUFFER);
    if buffer != 0 {
        let size = e.mem.u32(array + ARRAY_SIZE);
        fn_00cba0d0(e, Ptr::new(array), buffer, size);
        let buffer = e.mem.u32(array + ARRAY_BUFFER);
        free_array_buffer(e, array, buffer);
    }
}

// Translated from 00cbadf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `BSSimpleArray<NiPointer<bhkRagdollTemplateData>>` resize, `(count, shrink)`
/// (`RET 8`; `shrink` is a byte): with a count of 0 the items are released (`00cba0d0`) and,
/// when `shrink` is not 0, the buffer is freed (virtual slot `+8`) and the reserved size
/// cleared; the size becomes 0 (nothing happens without a buffer). A larger count than the
/// reserved size allocates (virtual slot 4, when it was 0) or reallocates (`00cb9ce0`) and
/// reserves the count; then the new items are cleared (`00cb9d50`) and the size becomes the
/// count. A smaller one than the size releases the items beyond it, sets the size and, with
/// `shrink` and a count of at most a quarter of the reserved size, reallocates down.
pub fn fn_00cbadf0(e: &mut Engine, this: Ptr, count: u32, shrink: u32) {
    let me = this.addr();
    if count == 0 {
        let buffer = e.mem.u32(me + ARRAY_BUFFER);
        if buffer == 0 {
            return;
        }
        let size = e.mem.u32(me + ARRAY_SIZE);
        fn_00cba0d0(e, this, buffer, size);
        if shrink as u8 != 0 {
            let buffer = e.mem.u32(me + ARRAY_BUFFER);
            e.vcall(me, 8, &args![buffer]);
            e.mem.set_u32(me + ARRAY_BUFFER, 0);
            e.mem.set_u32(me + ARRAY_RESERVED, 0);
        }
        e.mem.set_u32(me + ARRAY_SIZE, 0);
        return;
    }
    let reserved = e.mem.u32(me + ARRAY_RESERVED);
    if count > reserved {
        if reserved == 0 {
            let buffer = e.vcall(me, 4, &args![count]).u32();
            e.mem.set_u32(me + ARRAY_BUFFER, buffer);
        } else {
            let size = e.mem.u32(me + ARRAY_SIZE);
            e.call(ARRAY_REALLOCATE, &args![this, count, size]);
        }
        e.mem.set_u32(me + ARRAY_RESERVED, count);
    } else {
        let size = e.mem.u32(me + ARRAY_SIZE);
        if count < size {
            let buffer = e.mem.u32(me + ARRAY_BUFFER);
            fn_00cba0d0(e, this, buffer.wrapping_add(4 * count), size - count);
            e.mem.set_u32(me + ARRAY_SIZE, count);
            if shrink as u8 != 0 && count <= e.mem.u32(me + ARRAY_RESERVED) >> 2 {
                e.call(ARRAY_REALLOCATE, &args![this, count, count]);
                e.mem.set_u32(me + ARRAY_RESERVED, count);
            }
            return;
        }
    }
    let size = e.mem.u32(me + ARRAY_SIZE);
    let buffer = e.mem.u32(me + ARRAY_BUFFER);
    fn_00cb9d50(
        e,
        this,
        buffer.wrapping_add(size.wrapping_mul(4)),
        count.wrapping_sub(size),
    );
    e.mem.set_u32(me + ARRAY_SIZE, count);
}

// Translated from 00cbaec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<bhkRagdollTemplateData>,1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor body (as `00cbadb0`), then the memory manager's deallocation
/// (`00aa4060` on `011f6238`) of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn bhk_template_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    destruct_template_array(e, this.addr());
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00cbaf20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbaf20 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The `bhkRagdollTemplate` constructor (vtable `010c8dcc`, RTTI-confirmed): the
/// `NiExtraData` constructor (`00a7b2e0`), the vtable, the array at `+0xc` (vtable `010c8d28`)
/// with its buffer, size and reserved members cleared. Answers `this`.
pub fn fn_00cbaf20(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(EXTRA_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE_TEMPLATE);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0xc, VTABLE_TEMPLATE_ARRAY);
    this
}

// Translated from 00cbaf90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbaf90 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. The destructor body of `bhkRagdollTemplate`: stores the vtable; when the array
/// at `+0xc` has a buffer, releases its items (`00cba0d0`), frees the buffer through the
/// array's virtual slot `+8` and clears the array; stores the array vtable and does the same
/// for a remaining buffer; then the `NiExtraData` destructor body (`00a7b300`).
pub fn fn_00cbaf90(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_TEMPLATE);
    let array = me + 0xc;
    let buffer = e.mem.u32(me + 0x10);
    if buffer != 0 {
        let size = e.mem.u32(array + ARRAY_SIZE);
        fn_00cba0d0(e, Ptr::new(array), buffer, size);
        let buffer = e.mem.u32(array + ARRAY_BUFFER);
        free_array_buffer(e, array, buffer);
    }
    destruct_template_array(e, array);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00cbb050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::LoadBinary` (Xbox PDB), `(stream)`: the `NiExtraData` load (`00a7b4a0`),
/// then the item count through the read function of the stream's file (`[stream + 0x24C]`,
/// function at `+8`, `(file, &count, 4, &4, 1)`, the count added to the file position at
/// `+4`); resizes the array at `+0xc` to the count with `shrink` 1 (`00cbadf0`) and reads one
/// link id per item (`00a645b0`, on the stream).
pub fn bhk_ragdoll_template_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let count = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        transfer(e, stream.addr(), 0x24c, 8, local.addr(), 4);
        e.mem.u32(local.addr())
    });
    fn_00cbadf0(e, Ptr::new(this.addr() + 0xc), count, 1);
    let mut index = 0;
    while index < count {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        index += 1;
    }
}

// Translated from 00cbb0c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbb0c0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. Allocates a `bhkRagdollTemplate` (0x1c bytes, `operator new`), constructs it
/// when that succeeded (`00cbaf20`) and names it with `NiExtraData::SetName` (`00a5b950`) on
/// the address `011f49e0` (also when the allocation failed, with a null `this`). Answers the
/// object, or 0.
pub fn fn_00cbb0c0(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    let mut object = Ptr::NULL;
    if block != 0 {
        object = fn_00cbaf20(e, Ptr::new(block));
    }
    e.call(EXTRA_DATA_SET_NAME, &args![object, RAGDOLL_TEMPLATE_NAME]);
    object
}

// Translated from 00cb9b60(decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallAndSocketConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc9640`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ball_and_socket_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_9640;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cb9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc66b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_66b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cba6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cba1d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_template_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cb_a1d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

// Translated from 00cbb130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cbaf90`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x1c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_template_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cb_af90;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00cb9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallAndSocketConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_864c.
pub fn bhk_ball_and_socket_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_864c
}

// Translated from 00cb9c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8610.
pub fn bhk_ragdoll_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8610
}

// Translated from 00cba1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8400.
pub fn bhk_ragdoll_template_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8400
}

// Translated from 00cbaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8408.
pub fn bhk_ragdoll_template_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8408
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb9910, bhk_ragdoll_template_copy_members(Ptr, u32, u32)),
        entry!(0x00cb9920, fn_00cb9920(Ptr, Ptr) -> bool),
        entry!(0x00cb9930, fn_00cb9930(Ptr, u32, u32, u32)),
        entry!(0x00cb99a0, fn_00cb99a0(Ptr, u32) -> Ptr),
        entry!(0x00cb9a10, fn_00cb9a10(Ptr, u32) -> Ptr),
        entry!(0x00cb9a90, fn_00cb9a90(Ptr, Ptr, Ptr, f32) -> Ptr),
        entry!(0x00cb9b30, fn_00cb9b30(Ptr, u32) -> Ptr),
        entry!(0x00cb9b80, fn_00cb9b80(Ptr, u32) -> Ptr),
        entry!(0x00cb9c30, fn_00cb9c30(Ptr) -> Ptr),
        entry!(0x00cb9c50, fn_00cb9c50(Ptr) -> Ptr),
        entry!(0x00cb9c70, bhk_ragdoll_template_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00cb9d50, fn_00cb9d50(Ptr, u32, u32)),
        entry!(0x00cb9d80, bhk_ragdoll_template_data_save_binary(Ptr, Ptr)),
        entry!(0x00cb9ee0, bhk_ragdoll_template_link_object(Ptr, Ptr)),
        entry!(0x00cb9f60, bhk_ragdoll_template_save_binary(Ptr, Ptr)),
        entry!(0x00cb9fd0, fn_00cb9fd0(Ptr)),
        entry!(0x00cba000, fn_00cba000(Ptr)),
        entry!(0x00cba030, bhk_cinfo_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cba080, bhk_type_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cba0d0, fn_00cba0d0(Ptr, u32, u32)),
        entry!(0x00cba110, fn_00cba110(Ptr) -> Ptr),
        entry!(0x00cba1d0, fn_00cba1d0(Ptr)),
        entry!(0x00cba2d0, fn_00cba2d0() -> Ptr),
        entry!(0x00cba340, bhk_ragdoll_template_data_load_binary(Ptr, Ptr)),
        entry!(0x00cba6f0, fn_00cba6f0(Ptr, Ptr, f32)),
        entry!(0x00cbadb0, fn_00cbadb0(Ptr)),
        entry!(0x00cbadf0, fn_00cbadf0(Ptr, u32, u32)),
        entry!(0x00cbaec0, bhk_template_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cbaf20, fn_00cbaf20(Ptr) -> Ptr),
        entry!(0x00cbaf90, fn_00cbaf90(Ptr)),
        entry!(0x00cbb050, bhk_ragdoll_template_load_binary(Ptr, Ptr)),
        entry!(0x00cbb0c0, fn_00cbb0c0() -> Ptr),
        entry!(0x00cb9b50, bhk_ball_and_socket_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cb9c00, bhk_ragdoll_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cba1c0, bhk_ragdoll_template_data_get_r_t_t_i() -> u32),
        entry!(0x00cbaf80, bhk_ragdoll_template_get_r_t_t_i() -> u32),
        entry!(0x00cb9b60, bhk_ball_and_socket_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cb9c10, bhk_ragdoll_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cba6d0, bhk_ragdoll_template_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cbb130, bhk_ragdoll_template_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x7fff_0004;
    const FREE: u32 = 0x7fff_0008;
    const ALLOC: u32 = 0x7fff_000c;
    const CINFO_SAVE: u32 = 0x7fff_0010;
    const CINFO_LOAD: u32 = 0x7fff_0014;
    const LINK_ID_WRITE: u32 = 0x7fff_0030;
    const ITEM_REGISTER: u32 = 0x7fff_0054;
    const FILE_READ: u32 = 0x7fff_0100;
    const FILE_WRITE: u32 = 0x7fff_0104;
    /// Where the doubles of the stream's file keep the bytes (`file + FILE_DATA`).
    const FILE_DATA: u32 = 0x40;
    const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

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

    /// `Engine::new` does not register the Interlocked imports.
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
    }

    /// A vtable in heap memory whose slots (byte offsets) hold the targets.
    fn vtable_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        table
    }

    fn object_with(e: &mut Engine, table: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// An object whose virtual slot 4 is [`DELETE`] and whose reference count is `count`.
    fn counted_object(e: &mut Engine, count: u32) -> u32 {
        let table = vtable_with(e, &[(4, DELETE)]);
        let object = object_with(e, table);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A `BSSimpleArray` block holding `items` in a buffer with room for `reserved` (none when
    /// both are empty).
    fn array_of(e: &mut Engine, vtable: u32, items: &[u32], reserved: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, vtable);
        if reserved != 0 || !items.is_empty() {
            let buffer = e.mem.alloc(4 * reserved.max(items.len() as u32));
            for (index, item) in items.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * index as u32, *item);
            }
            e.mem.set_u32(array + 4, buffer);
        }
        e.mem.set_u32(array + 8, items.len() as u32);
        e.mem.set_u32(array + 0xc, reserved);
        array
    }

    fn template_array(e: &mut Engine, items: &[u32], reserved: u32) -> u32 {
        array_of(e, VTABLE_TEMPLATE_ARRAY, items, reserved)
    }

    /// The allocate (`ALLOC`, fresh memory filled with `0xdeadbeef`), free and reallocate
    /// routines of the template array as doubles, and the doubles the release path needs.
    fn template_array_doubles(e: &mut Engine) {
        interlocked(e);
        e.register(DELETE, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(ALLOC, |e, a| {
            let buffer = e.mem.alloc(4 * a[1]);
            for index in 0..a[1] {
                e.mem.set_u32(buffer + 4 * index, 0xdead_beef);
            }
            buffer.into_ret()
        });
        e.register(ARRAY_REALLOCATE, |e, a| {
            let old = e.mem.u32(a[0] + 4);
            let buffer = e.mem.alloc(4 * a[1]);
            for index in 0..a[1] {
                let word = if index < a[2] {
                    e.mem.u32(old + 4 * index)
                } else {
                    0xdead_beef
                };
                e.mem.set_u32(buffer + 4 * index, word);
            }
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.mem.set_u32(VTABLE_TEMPLATE_ARRAY + 4, ALLOC);
        e.mem.set_u32(VTABLE_TEMPLATE_ARRAY + 8, FREE);
    }

    struct Stream {
        stream: u32,
        file: u32,
    }

    /// The file doubles keep the bytes at `file + FILE_DATA + position`.
    fn file_write(e: &mut Engine, a: &[u32]) -> Ret {
        let position = e.mem.u32(a[0] + 4);
        let data = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0] + FILE_DATA + position, &data);
        a[2].into_ret()
    }

    fn file_read(e: &mut Engine, a: &[u32]) -> Ret {
        let position = e.mem.u32(a[0] + 4);
        let data = e.mem.bytes(a[0] + FILE_DATA + position, a[2]);
        e.mem.write(a[1], &data);
        a[2].into_ret()
    }

    /// A stream whose read (`+0x24c`) and write (`+0x250`) file has the double functions at
    /// `+8` and `+0xc` and whose virtual slot `0x30` is [`LINK_ID_WRITE`].
    fn stream(e: &mut Engine) -> Stream {
        let file = e.mem.alloc(0x200);
        e.mem.set_u32(file + 8, FILE_READ);
        e.mem.set_u32(file + 0xc, FILE_WRITE);
        let table = vtable_with(e, &[(0x30, LINK_ID_WRITE)]);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, table);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        e.register(FILE_READ, file_read);
        e.register(FILE_WRITE, file_write);
        e.register(LINK_ID_WRITE, |_, _| Ret::default());
        Stream { stream, file }
    }

    /// The words written to the stream's file so far.
    fn written(e: &Engine, s: &Stream) -> Vec<u32> {
        (0..e.mem.u32(s.file + 4) / 4)
            .map(|index| e.mem.u32(s.file + FILE_DATA + 4 * index))
            .collect()
    }

    /// Puts the words the stream's file reads from its start.
    fn put_words(e: &mut Engine, s: &Stream, words: &[u32]) {
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(s.file + FILE_DATA + 4 * index as u32, *word);
        }
    }

    fn constructor_doubles(e: &mut Engine) {
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(CONSTRAINT_INIT, |_, _| Ret::default());
    }

    #[test]
    fn bhk_ball_and_socket_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_9640, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cb9b60, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cb9b60, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_9640), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_66b0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cb9c10, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cb9c10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_66b0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_template_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cb_a1d0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cba6d0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cba6d0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00cb_a1d0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_template_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cb_af90, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cbb130, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cbb130, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x1c]]);
        assert_eq!(calls_to(&e, 0x00cb_af90), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ball_and_socket_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_9b50, &args![]).u32(), 0x0126_864c);
    }

    #[test]
    fn bhk_ragdoll_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_9c00, &args![]).u32(), 0x0126_8610);
    }

    #[test]
    fn bhk_ragdoll_template_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_a1c0, &args![]).u32(), 0x0126_8400);
    }

    #[test]
    fn bhk_ragdoll_template_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_af80, &args![]).u32(), 0x0126_8408);
    }

    #[test]
    fn bhk_ragdoll_template_copy_members_passes_the_words_to_the_extra_data_copy() {
        let mut e = logged_engine();
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.call(0x00cb9910, &args![0x5000u32, 0x6000u32, 1u32]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![0x5000, 0x6000, 1]]
        );
    }

    #[test]
    fn fn_00cb9920_passes_the_extra_data_comparison_through() {
        let mut e = logged_engine();
        e.register(EXTRA_DATA_IS_EQUAL, |_, a| a[1].into_ret());
        assert!(e.call(0x00cb9920, &args![0x5000u32, 1u32]).bool());
        assert!(!e.call(0x00cb9920, &args![0x5000u32, 0x100u32]).bool());
        assert_eq!(
            calls_to(&e, EXTRA_DATA_IS_EQUAL),
            vec![vec![0x5000, 1], vec![0x5000, 0x100]]
        );
    }

    #[test]
    fn fn_00cb9930_moves_items_forwards_or_backwards_one_by_one() {
        let mut e = logged_engine();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call(0x00cb9930, &args![0u32, 0x1000u32, 0x2000u32, 3u32]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            vec![
                vec![0x1000, 0x2000, 4],
                vec![0x1004, 0x2004, 4],
                vec![0x1008, 0x2008, 4]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00cb9930, &args![0u32, 0x2000u32, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            vec![
                vec![0x2008, 0x1008, 4],
                vec![0x2004, 0x1004, 4],
                vec![0x2000, 0x1000, 4]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00cb9930, &args![0u32, 0x1000u32, 0x1000u32, 3u32]);
        e.call(0x00cb9930, &args![0u32, 0x1000u32, 0x2000u32, 0u32]);
        e.call(0x00cb9930, &args![0u32, 0x2000u32, 0x1000u32, 0u32]);
        assert!(calls_to(&e, MEMMOVE).is_empty());
    }

    #[test]
    fn fn_00cb99a0_builds_a_constraint_from_the_holder() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xdead);
        e.set_global(COUNT_CONSTRAINT, 5u32);
        assert_eq!(e.call(0x00cb99a0, &args![this, 0x2000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CONSTRAINT);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 6);
        assert_eq!(e.call(0x00cb99a0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![this]; 2]);
        assert_eq!(
            calls_to(&e, CONSTRAINT_INIT),
            vec![vec![this, 0x2004], vec![this, 0]]
        );
    }

    #[test]
    fn fn_00cb9a10_builds_a_malleable_constraint_from_the_holder() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xdead);
        e.set_global(COUNT_CONSTRAINT, 5u32);
        e.set_global(COUNT_MALLEABLE, 9u32);
        assert_eq!(e.call(0x00cb9a10, &args![this, 0x2000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_MALLEABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 6);
        assert_eq!(e.global::<u32>(COUNT_MALLEABLE), 10);
        assert_eq!(e.call(0x00cb9a10, &args![this, 0u32]).u32(), this);
        assert_eq!(
            calls_to(&e, CONSTRAINT_INIT),
            vec![vec![this, 0x2004], vec![this, 0]]
        );
    }

    #[test]
    fn fn_00cb9a90_builds_a_capsule_shape_from_two_points_and_a_radius() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(CAPSULE_SHAPE_INIT, |_, _| Ret::default());
        let counters = [
            COUNT_SHAPE,
            COUNT_SPHERE_REP_SHAPE,
            COUNT_CONVEX_SHAPE,
            COUNT_CAPSULE_SHAPE,
        ];
        for counter in counters {
            e.set_global(counter, 7u32);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xdead);
        e.mem.set_u32(this + 0x10, 0xbeef);
        let call = e.call(0x00cb9a90, &args![this, 0x1000u32, 0x2000u32, 0.5f32]);
        assert_eq!(call.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CAPSULE_SHAPE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        // The radius is copied through the x87 stack: a signalling NaN is quieted.
        let signalling = f32::from_bits(0x7fa0_0000);
        e.call(0x00cb9a90, &args![this, 0x1000u32, 0x2000u32, signalling]);
        for counter in counters {
            assert_eq!(e.global::<u32>(counter), 9);
        }
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![this]; 2]);
        assert_eq!(
            calls_to(&e, CAPSULE_SHAPE_INIT),
            vec![
                vec![this, 0x1000, 0x2000, 0.5f32.to_bits()],
                vec![this, 0x1000, 0x2000, 0x7fe0_0000]
            ]
        );
    }

    #[test]
    fn fn_00cb9b30_builds_a_ball_and_socket_constraint_from_the_holder() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.set_global(COUNT_CONSTRAINT, 1u32);
        e.set_global(COUNT_BALL_AND_SOCKET, 20u32);
        assert_eq!(e.call(0x00cb9b30, &args![this, 0x2000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_BALL_AND_SOCKET);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 2);
        assert_eq!(e.global::<u32>(COUNT_BALL_AND_SOCKET), 21);
        assert_eq!(calls_to(&e, CONSTRAINT_INIT), vec![vec![this, 0x2004]]);
    }

    #[test]
    fn fn_00cb9b80_builds_a_ragdoll_constraint_from_the_holder() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xdead);
        e.set_global(COUNT_CONSTRAINT, 5u32);
        e.set_global(COUNT_RAGDOLL_CONSTRAINT, 9u32);
        assert_eq!(e.call(0x00cb9b80, &args![this, 0x2000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_RAGDOLL_CONSTRAINT);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 6);
        assert_eq!(e.global::<u32>(COUNT_RAGDOLL_CONSTRAINT), 10);
        assert_eq!(e.call(0x00cb9b80, &args![this, 0u32]).u32(), this);
        assert_eq!(
            calls_to(&e, CONSTRAINT_INIT),
            vec![vec![this, 0x2004], vec![this, 0]]
        );
    }

    #[test]
    fn fn_00cb9c30_constructs_a_ball_and_socket_cinfo_holder() {
        let mut e = logged_engine();
        let this = e.mem.alloc(0x20);
        for offset in [4, 0xc, 0x10] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        assert_eq!(e.call(0x00cb9c30, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_BALL_AND_SOCKET_CINFO);
        assert_eq!(e.mem.u32(this + 8), 1);
        for offset in [4, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn fn_00cb9c50_constructs_a_ragdoll_cinfo_holder() {
        let mut e = logged_engine();
        let this = e.mem.alloc(0x20);
        for offset in [4, 0xc, 0x10] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        assert_eq!(e.call(0x00cb9c50, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_RAGDOLL_CINFO);
        assert_eq!(e.mem.u32(this + 8), 1);
        for offset in [4, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn bhk_ragdoll_template_register_streamables_registers_each_item_when_the_base_does() {
        let mut e = logged_engine();
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, a| a[1].into_ret());
        e.register(ITEM_REGISTER, |_, _| Ret::default());
        let table = vtable_with(&mut e, &[(0x54, ITEM_REGISTER)]);
        let first = object_with(&mut e, table);
        let second = object_with(&mut e, table);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, first);
        e.mem.set_u32(buffer + 4, second);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, buffer);
        e.mem.set_u32(this + 0x14, 2);
        assert!(e.call(0x00cb9c70, &args![this, 1u32]).bool());
        assert_eq!(
            calls_to(&e, ITEM_REGISTER),
            vec![vec![first, 1], vec![second, 1]]
        );
        // Only the low byte of the answer of the base counts.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00cb9c70, &args![this, 0x100u32]).bool());
        assert!(calls_to(&e, ITEM_REGISTER).is_empty());
    }

    #[test]
    fn fn_00cb9d50_clears_the_pointer_slots() {
        let mut e = logged_engine();
        let slots = e.mem.alloc(16);
        for index in 0..4 {
            e.mem.set_u32(slots + 4 * index, 7);
        }
        e.call(0x00cb9d50, &args![0u32, slots, 3u32]);
        let words: Vec<u32> = (0..4).map(|index| e.mem.u32(slots + 4 * index)).collect();
        assert_eq!(words, vec![0, 0, 0, 7]);
        e.call(0x00cb9d50, &args![0u32, slots + 12, 0u32]);
        assert_eq!(e.mem.u32(slots + 12), 7);
    }

    #[test]
    fn bhk_ragdoll_template_data_save_binary_writes_the_words_and_saves_each_cinfo() {
        let mut e = logged_engine();
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_FIXED_STRING, |_, _| Ret::default());
        e.register(CINFO_SAVE, |_, _| Ret::default());
        let s = stream(&mut e);
        let table = vtable_with(&mut e, &[(0xc, CINFO_SAVE)]);
        let first = object_with(&mut e, table);
        let second = object_with(&mut e, table);
        let cinfos = e.mem.alloc(8);
        e.mem.set_u32(cinfos, first);
        e.mem.set_u32(cinfos + 4, second);
        let types = e.mem.alloc(8);
        e.mem.set_u32(types, 7);
        e.mem.set_u32(types + 4, 13);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x10, cinfos);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x20, types);
        for (index, word) in [10u32, 11, 12, 13, 14].into_iter().enumerate() {
            e.mem.set_u32(this + 0x2c + 4 * index as u32, word);
        }
        e.call(0x00cb9d80, &args![this, s.stream]);
        assert_eq!(written(&e, &s), vec![10, 11, 12, 13, 14, 2, 7, 13]);
        assert_eq!(e.mem.u32(s.file + 4), 32);
        assert_eq!(calls_to(&e, OBJECT_SAVE_BINARY), vec![vec![this, s.stream]]);
        assert_eq!(
            calls_to(&e, STREAM_SAVE_FIXED_STRING),
            vec![vec![s.stream, this + 8]]
        );
        assert_eq!(
            calls_to(&e, CINFO_SAVE),
            vec![vec![first, s.stream], vec![second, s.stream]]
        );
    }

    #[test]
    fn bhk_ragdoll_template_link_object_replaces_each_item_with_the_stream_answer() {
        let mut e = logged_engine();
        interlocked(&mut e);
        e.register(EXTRA_DATA_LINK_OBJECT, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        let replaced = counted_object(&mut e, 1);
        let kept = counted_object(&mut e, 1);
        let fresh = counted_object(&mut e, 0);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, replaced);
        e.mem.set_u32(buffer + 4, kept);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, buffer);
        e.mem.set_u32(this + 0x14, 2);
        let mut answers = vec![fresh, kept].into_iter();
        e.register_double(STREAM_GET_LINK_OBJECT, move |_, _| {
            answers.next().unwrap().into_ret()
        });
        e.call(0x00cb9ee0, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(buffer), fresh);
        assert_eq!(e.mem.u32(buffer + 4), kept);
        assert_eq!(e.mem.u32(replaced + 4), 0);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(e.mem.u32(fresh + 4), 1);
        assert_eq!(calls_to(&e, DELETE), vec![vec![replaced]]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LINK_OBJECT),
            vec![vec![this, 0x9000]]
        );
        // A null answer drops the old object and stores 0.
        let mut none = vec![0u32].into_iter();
        e.register_double(STREAM_GET_LINK_OBJECT, move |_, _| {
            none.next().unwrap().into_ret()
        });
        e.mem.set_u32(this + 0x14, 1);
        e.call(0x00cb9ee0, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(buffer), 0);
        assert_eq!(calls_to(&e, DELETE), vec![vec![replaced], vec![fresh]]);
        // Without items the stream is not asked.
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x00cb9ee0, &args![this, 0x9000u32]);
        assert_eq!(calls_to(&e, STREAM_GET_LINK_OBJECT).len(), 3);
    }

    #[test]
    fn bhk_ragdoll_template_save_binary_writes_the_count_and_a_link_id_per_item() {
        let mut e = logged_engine();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        let s = stream(&mut e);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, 0x1111);
        e.mem.set_u32(buffer + 4, 0x2222);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, buffer);
        e.mem.set_u32(this + 0x14, 2);
        e.call(0x00cb9f60, &args![this, s.stream]);
        assert_eq!(written(&e, &s), vec![2]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SAVE_BINARY),
            vec![vec![this, s.stream]]
        );
        assert_eq!(
            calls_to(&e, LINK_ID_WRITE),
            vec![vec![s.stream, 0x1111], vec![s.stream, 0x2222]]
        );
    }

    #[test]
    fn fn_00cb9fd0_frees_the_buffer_of_a_cinfo_array() {
        let mut e = logged_engine();
        e.register(FREE, |_, _| Ret::default());
        e.mem.set_u32(CINFO_ARRAY_FREE_SLOT, FREE);
        let array = array_of(&mut e, 0, &[1, 2, 3], 4);
        let buffer = e.mem.u32(array + 4);
        e.call(0x00cb9fd0, &args![array]);
        assert_eq!(e.mem.u32(array), VTABLE_CINFO_ARRAY);
        let members = [
            e.mem.u32(array + 4),
            e.mem.u32(array + 8),
            e.mem.u32(array + 0xc),
        ];
        assert_eq!(members, [0; 3]);
        assert_eq!(calls_to(&e, FREE), vec![vec![array, buffer]]);
        e.call(0x00cb9fd0, &args![array]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn fn_00cba000_frees_the_buffer_of_a_type_array() {
        let mut e = logged_engine();
        e.register(FREE, |_, _| Ret::default());
        e.mem.set_u32(TYPE_ARRAY_FREE_SLOT, FREE);
        let array = array_of(&mut e, 0, &[1, 2, 3], 4);
        let buffer = e.mem.u32(array + 4);
        e.call(0x00cba000, &args![array]);
        assert_eq!(e.mem.u32(array), VTABLE_TYPE_ARRAY);
        let members = [
            e.mem.u32(array + 4),
            e.mem.u32(array + 8),
            e.mem.u32(array + 0xc),
        ];
        assert_eq!(members, [0; 3]);
        assert_eq!(calls_to(&e, FREE), vec![vec![array, buffer]]);
        e.call(0x00cba000, &args![array]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn bhk_cinfo_array_scalar_deleting_destructor_deallocates_only_on_bit_0() {
        let mut e = logged_engine();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let array = array_of(&mut e, 0, &[], 0);
        assert_eq!(e.call(0x00cba030, &args![array, 0xfffe_u32]).u32(), array);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(array), VTABLE_CINFO_ARRAY);
        assert_eq!(e.call(0x00cba030, &args![array, 1u32]).u32(), array);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn bhk_type_array_scalar_deleting_destructor_deallocates_only_on_bit_0() {
        let mut e = logged_engine();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let array = array_of(&mut e, 0, &[], 0);
        assert_eq!(e.call(0x00cba080, &args![array, 0xfffe_u32]).u32(), array);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(array), VTABLE_TYPE_ARRAY);
        assert_eq!(e.call(0x00cba080, &args![array, 1u32]).u32(), array);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn fn_00cba0d0_releases_the_non_null_pointers() {
        let mut e = logged_engine();
        interlocked(&mut e);
        e.register(DELETE, |_, _| Ret::default());
        let shared = counted_object(&mut e, 2);
        let last = counted_object(&mut e, 1);
        let pointers = e.mem.alloc(12);
        e.mem.set_u32(pointers, shared);
        e.mem.set_u32(pointers + 8, last);
        e.call(0x00cba0d0, &args![0u32, pointers, 3u32]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(e.mem.u32(last + 4), 0);
        assert_eq!(calls_to(&e, DELETE), vec![vec![last]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![shared + 4], vec![last + 4]]
        );
    }

    #[test]
    fn fn_00cba110_constructs_a_template_data() {
        let mut e = logged_engine();
        interlocked(&mut e);
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(FIXED_STRING_ASSIGN, |_, _| 0x7777u32.into_ret());
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 5);
        let this = e.mem.alloc(0x40);
        for offset in [0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        // The default name handle is replaced by a fresh one and not released.
        e.set_global(FIXED_STRING_DEFAULT, block + 8);
        assert_eq!(e.call(0x00cba110, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE_DATA);
        assert_eq!(e.mem.u32(this + 8), 0x7777);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_CINFO_ARRAY);
        assert_eq!(e.mem.u32(this + 0x1c), VTABLE_TYPE_ARRAY);
        for offset in [0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls_to(&e, FIXED_STRING_ASSIGN), vec![vec![0]]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        // A zero default handle stays.
        e.set_global(FIXED_STRING_DEFAULT, 0u32);
        e.call(0x00cba110, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls_to(&e, FIXED_STRING_ASSIGN).len(), 1);
        // The global changing while the fresh handle is made releases the old handle.
        e.register(FIXED_STRING_ASSIGN, |e, _| {
            e.set_global(FIXED_STRING_DEFAULT, 0x5100u32);
            0x7777u32.into_ret()
        });
        e.set_global(FIXED_STRING_DEFAULT, block + 8);
        e.call(0x00cba110, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), 4);
    }

    #[test]
    fn fn_00cba1d0_destroys_a_template_data() {
        let mut e = logged_engine();
        interlocked(&mut e);
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.mem.set_u32(CINFO_ARRAY_FREE_SLOT, FREE);
        e.mem.set_u32(TYPE_ARRAY_FREE_SLOT, FREE);
        let table = vtable_with(&mut e, &[(0, DELETE)]);
        let first = object_with(&mut e, table);
        let third = object_with(&mut e, table);
        let this = e.mem.alloc(0x50);
        let cinfos = array_of(&mut e, VTABLE_CINFO_ARRAY, &[first, 0, third], 3);
        let types = array_of(&mut e, VTABLE_TYPE_ARRAY, &[0, 1, 2], 3);
        let cinfo_buffer = e.mem.u32(cinfos + 4);
        let type_buffer = e.mem.u32(types + 4);
        for (array, offset) in [(cinfos, 0xc), (types, 0x1c)] {
            for member in [0, 4, 8, 0xc] {
                let word = e.mem.u32(array + member);
                e.mem.set_u32(this + offset + member, word);
            }
        }
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 5);
        e.mem.set_u32(this + 8, block + 8);
        e.set_global(FIXED_STRING_DEFAULT, 0u32);
        e.call(0x00cba1d0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE_DATA);
        assert_eq!(calls_to(&e, DELETE), vec![vec![first, 1], vec![third, 1]]);
        assert_eq!(
            calls_to(&e, FREE),
            vec![
                vec![this + 0xc, cinfo_buffer],
                vec![this + 0x1c, type_buffer]
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_CINFO_ARRAY);
        assert_eq!(e.mem.u32(this + 0x1c), VTABLE_TYPE_ARRAY);
        for offset in [0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(block), 4);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this]]);
        // The default name handle is not released.
        e.set_global(FIXED_STRING_DEFAULT, block + 8);
        e.call(0x00cba1d0, &args![this]);
        assert_eq!(e.mem.u32(block), 4);
    }

    #[test]
    fn fn_00cba2d0_allocates_and_constructs_a_template_data() {
        let mut e = logged_engine();
        interlocked(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(FIXED_STRING_ASSIGN, |_, _| Ret::default());
        let this = e.call(0x00cba2d0, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE_DATA);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cba2d0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT).len(), 1);
    }

    /// The doubles `bhkRagdollTemplateData::LoadBinary` calls outside the file; the cinfo
    /// classes get [`CINFO_LOAD`] as their virtual slot 8.
    fn load_doubles(e: &mut Engine) {
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(CINFO_LOAD, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ARRAY_REALLOCATE, |e, a| {
            let buffer = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.register(ALLOC, |e, a| e.mem.alloc(4 * a[1]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        for vtable in [
            VTABLE_BALL_AND_SOCKET_CINFO,
            VTABLE_HINGE_CINFO,
            VTABLE_LIMITED_HINGE_CINFO,
            VTABLE_RAGDOLL_CINFO,
            VTABLE_MALLEABLE_CINFO,
        ] {
            e.mem.set_u32(vtable + 8, CINFO_LOAD);
        }
        for vtable in [VTABLE_CINFO_ARRAY, VTABLE_TYPE_ARRAY] {
            e.mem.set_u32(vtable + 4, ALLOC);
            e.mem.set_u32(vtable + 8, FREE);
        }
    }

    /// A template data block whose two arrays are empty and have the exe's vtables.
    fn empty_template_data(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0xc, VTABLE_CINFO_ARRAY);
        e.mem.set_u32(this + 0x1c, VTABLE_TYPE_ARRAY);
        this
    }

    #[test]
    fn bhk_ragdoll_template_data_load_binary_builds_a_cinfo_for_each_type() {
        let mut e = logged_engine();
        load_doubles(&mut e);
        let this = empty_template_data(&mut e);
        let s = stream(&mut e);
        put_words(&mut e, &s, &[1, 2, 3, 4, 5, 5, 0, 1, 2, 7, 13]);
        e.call(0x00cba340, &args![this, s.stream]);
        for (index, word) in [1u32, 2, 3, 4, 5].into_iter().enumerate() {
            assert_eq!(e.mem.u32(this + 0x2c + 4 * index as u32), word);
        }
        for offset in [0x14, 0x18, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 5);
        }
        let cinfos = e.mem.u32(this + 0x10);
        let types = e.mem.u32(this + 0x20);
        let expected = [
            (0, VTABLE_BALL_AND_SOCKET_CINFO),
            (1, VTABLE_HINGE_CINFO),
            (2, VTABLE_LIMITED_HINGE_CINFO),
            (7, VTABLE_RAGDOLL_CINFO),
            (13, VTABLE_MALLEABLE_CINFO),
        ];
        let mut loaded = vec![];
        for (index, (kind, vtable)) in expected.into_iter().enumerate() {
            let index = index as u32;
            assert_eq!(e.mem.u32(types + 4 * index), kind);
            let cinfo = e.mem.u32(cinfos + 4 * index);
            assert_eq!(e.mem.u32(cinfo), vtable);
            assert_eq!(e.mem.u32(cinfo + 8), 1);
            for offset in [4, 0xc, 0x10] {
                assert_eq!(e.mem.u32(cinfo + offset), 0);
            }
            loaded.push(vec![cinfo, s.stream]);
        }
        assert_eq!(calls_to(&e, CINFO_LOAD), loaded);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]; 5]
        );
        assert_eq!(calls_to(&e, OBJECT_LOAD_BINARY), vec![vec![this, s.stream]]);
        assert_eq!(
            calls_to(&e, STREAM_LOAD_FIXED_STRING),
            vec![vec![s.stream, this + 8]]
        );
        assert_eq!(
            calls_to(&e, ALLOC),
            vec![vec![this + 0xc, 5], vec![this + 0x1c, 5]]
        );
        assert_eq!(e.mem.u32(s.file + 4), 44);
    }

    #[test]
    fn bhk_ragdoll_template_data_load_binary_keeps_the_cinfo_of_an_unknown_type() {
        let mut e = logged_engine();
        load_doubles(&mut e);
        let this = empty_template_data(&mut e);
        let table = vtable_with(&mut e, &[(8, CINFO_LOAD)]);
        let existing = object_with(&mut e, table);
        let cinfo_buffer = e.mem.alloc(8);
        e.mem.set_u32(cinfo_buffer, existing);
        e.mem.set_u32(this + 0x10, cinfo_buffer);
        e.mem.set_u32(this + 0x18, 2);
        let type_buffer = e.mem.alloc(8);
        e.mem.set_u32(this + 0x20, type_buffer);
        e.mem.set_u32(this + 0x28, 2);
        let s = stream(&mut e);
        put_words(&mut e, &s, &[0, 0, 0, 0, 0, 1, 5]);
        e.call(0x00cba340, &args![this, s.stream]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, CINFO_LOAD), vec![vec![existing, s.stream]]);
        assert_eq!(e.mem.u32(type_buffer), 5);
        assert_eq!(e.mem.u32(cinfo_buffer), existing);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x28), 2);
    }

    #[test]
    fn bhk_ragdoll_template_data_load_binary_frees_the_arrays_for_a_count_of_zero() {
        let mut e = logged_engine();
        load_doubles(&mut e);
        let this = empty_template_data(&mut e);
        let cinfo_buffer = e.mem.alloc(8);
        let type_buffer = e.mem.alloc(8);
        for (offset, value) in [
            (0x10, cinfo_buffer),
            (0x14, 2),
            (0x18, 2),
            (0x20, type_buffer),
            (0x24, 2),
            (0x28, 2),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        let s = stream(&mut e);
        put_words(&mut e, &s, &[0, 0, 0, 0, 0, 0]);
        e.call(0x00cba340, &args![this, s.stream]);
        assert_eq!(
            calls_to(&e, FREE),
            vec![
                vec![this + 0xc, cinfo_buffer],
                vec![this + 0x1c, type_buffer]
            ]
        );
        for offset in [0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn bhk_ragdoll_template_data_load_binary_reallocates_and_trims_the_arrays() {
        let mut e = logged_engine();
        load_doubles(&mut e);
        let this = empty_template_data(&mut e);
        // Growing: the cinfo array has room for one (reallocated), the type array none.
        let cinfo_buffer = e.mem.alloc(4);
        e.mem.set_u32(this + 0x10, cinfo_buffer);
        e.mem.set_u32(this + 0x14, 1);
        e.mem.set_u32(this + 0x18, 1);
        let first = stream(&mut e);
        put_words(&mut e, &first, &[0, 0, 0, 0, 0, 3, 0, 0, 0]);
        e.call(0x00cba340, &args![this, first.stream]);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE), vec![vec![this + 0xc, 3, 1]]);
        assert_eq!(calls_to(&e, ALLOC), vec![vec![this + 0x1c, 3]]);
        for offset in [0x14, 0x18, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 3);
        }
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 3);
        // Shrinking to a quarter of the reserved size or less trims both.
        e.call_log = Some(vec![]);
        for offset in [0x18, 0x28] {
            e.mem.set_u32(this + offset, 8);
        }
        let second = stream(&mut e);
        put_words(&mut e, &second, &[0, 0, 0, 0, 0, 1, 0]);
        e.call(0x00cba340, &args![this, second.stream]);
        assert_eq!(
            calls_to(&e, ARRAY_REALLOCATE),
            vec![vec![this + 0xc, 1, 1], vec![this + 0x1c, 1, 1]]
        );
        for offset in [0x14, 0x18, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 1);
        }
    }

    #[test]
    fn bhk_template_array_scalar_deleting_destructor_releases_the_items_and_deallocates() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let first = counted_object(&mut e, 1);
        let second = counted_object(&mut e, 2);
        let array = template_array(&mut e, &[first, second], 2);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(e.call(0x00cbaec0, &args![array, 0xfffe_u32]).u32(), array);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(array), VTABLE_TEMPLATE_ARRAY);
        assert_eq!(calls_to(&e, DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(calls_to(&e, FREE), vec![vec![array, buffer]]);
        let members = [
            e.mem.u32(array + 4),
            e.mem.u32(array + 8),
            e.mem.u32(array + 0xc),
        ];
        assert_eq!(members, [0; 3]);
        assert_eq!(e.call(0x00cbaec0, &args![array, 1u32]).u32(), array);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn fn_00cbadb0_releases_the_items_and_frees_the_buffer() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        let first = counted_object(&mut e, 1);
        let second = counted_object(&mut e, 2);
        let array = template_array(&mut e, &[first, second], 2);
        let buffer = e.mem.u32(array + 4);
        e.call(0x00cbadb0, &args![array]);
        assert_eq!(e.mem.u32(array), VTABLE_TEMPLATE_ARRAY);
        assert_eq!(calls_to(&e, DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(calls_to(&e, FREE), vec![vec![array, buffer]]);
        let members = [
            e.mem.u32(array + 4),
            e.mem.u32(array + 8),
            e.mem.u32(array + 0xc),
        ];
        assert_eq!(members, [0; 3]);
        // An array without a buffer is left alone.
        e.call(0x00cbadb0, &args![array]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn fn_00cbadf0_with_a_count_of_zero_releases_the_items_and_maybe_frees_the_buffer() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        let empty = template_array(&mut e, &[], 0);
        e.call(0x00cbadf0, &args![empty, 0u32, 1u32]);
        assert!(calls_to(&e, FREE).is_empty());
        let first = counted_object(&mut e, 1);
        let second = counted_object(&mut e, 2);
        let array = template_array(&mut e, &[first, second], 4);
        let buffer = e.mem.u32(array + 4);
        // Only the low byte of the shrink flag counts.
        e.call(0x00cbadf0, &args![array, 0u32, 0x100u32]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(e.mem.u32(array + 4), buffer);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x00cbadf0, &args![array, 0u32, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![array, buffer]]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 0xc), 0);
    }

    #[test]
    fn fn_00cbadf0_grows_by_allocating_or_reallocating_and_clears_the_new_items() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        let empty = template_array(&mut e, &[], 0);
        e.call(0x00cbadf0, &args![empty, 3u32, 1u32]);
        assert_eq!(calls_to(&e, ALLOC), vec![vec![empty, 3]]);
        let buffer = e.mem.u32(empty + 4);
        let words: Vec<u32> = (0..3).map(|index| e.mem.u32(buffer + 4 * index)).collect();
        assert_eq!(words, vec![0, 0, 0]);
        assert_eq!(e.mem.u32(empty + 8), 3);
        assert_eq!(e.mem.u32(empty + 0xc), 3);
        let kept = counted_object(&mut e, 1);
        let array = template_array(&mut e, &[kept], 1);
        e.call(0x00cbadf0, &args![array, 3u32, 0u32]);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE), vec![vec![array, 3, 1]]);
        let buffer = e.mem.u32(array + 4);
        let words: Vec<u32> = (0..3).map(|index| e.mem.u32(buffer + 4 * index)).collect();
        assert_eq!(words, vec![kept, 0, 0]);
        assert_eq!(e.mem.u32(array + 8), 3);
        assert_eq!(e.mem.u32(array + 0xc), 3);
    }

    #[test]
    fn fn_00cbadf0_shrinks_by_releasing_the_items_beyond_the_count() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        let items: Vec<u32> = (0..3).map(|_| counted_object(&mut e, 1)).collect();
        let array = template_array(&mut e, &items, 8);
        e.call(0x00cbadf0, &args![array, 1u32, 0u32]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![items[1]], vec![items[2]]]);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls_to(&e, ARRAY_REALLOCATE).is_empty());
        // With the shrink flag and a count of at most a quarter of the reserved size: trimmed.
        let more: Vec<u32> = (0..3).map(|_| counted_object(&mut e, 1)).collect();
        let trimmed = template_array(&mut e, &more, 8);
        e.call(0x00cbadf0, &args![trimmed, 1u32, 1u32]);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE), vec![vec![trimmed, 1, 1]]);
        assert_eq!(e.mem.u32(trimmed + 0xc), 1);
        // A count above a quarter of the reserved size is not.
        let rest: Vec<u32> = (0..3).map(|_| counted_object(&mut e, 1)).collect();
        let kept = template_array(&mut e, &rest, 7);
        e.call(0x00cbadf0, &args![kept, 2u32, 1u32]);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(kept + 8), 2);
        assert_eq!(e.mem.u32(kept + 0xc), 7);
    }

    #[test]
    fn fn_00cbadf0_with_the_current_size_changes_nothing() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        let first = counted_object(&mut e, 1);
        let second = counted_object(&mut e, 1);
        let array = template_array(&mut e, &[first, second], 4);
        let buffer = e.mem.u32(array + 4);
        e.call(0x00cbadf0, &args![array, 2u32, 1u32]);
        assert!(calls_to(&e, DELETE).is_empty());
        assert!(calls_to(&e, ARRAY_REALLOCATE).is_empty());
        assert_eq!(e.mem.u32(array + 4), buffer);
        assert_eq!(e.mem.u32(buffer), first);
        assert_eq!(e.mem.u32(buffer + 4), second);
        assert_eq!(e.mem.u32(array + 8), 2);
        assert_eq!(e.mem.u32(array + 0xc), 4);
    }

    #[test]
    fn fn_00cbaf20_constructs_a_template() {
        let mut e = logged_engine();
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        for offset in [0x10, 0x14, 0x18] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        assert_eq!(e.call(0x00cbaf20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_TEMPLATE_ARRAY);
        for offset in [0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00cbaf90_destroys_a_template() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        e.register(EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        let first = counted_object(&mut e, 1);
        let this = e.mem.alloc(0x20);
        let items = template_array(&mut e, &[first], 1);
        let buffer = e.mem.u32(items + 4);
        for (member, offset) in [(0, 0xc), (4, 0x10), (8, 0x14), (0xc, 0x18)] {
            let word = e.mem.u32(items + member);
            e.mem.set_u32(this + offset, word);
        }
        e.call(0x00cbaf90, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_TEMPLATE_ARRAY);
        assert_eq!(calls_to(&e, DELETE), vec![vec![first]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this + 0xc, buffer]]);
        for offset in [0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn bhk_ragdoll_template_load_binary_reads_the_count_and_one_link_id_per_item() {
        let mut e = logged_engine();
        template_array_doubles(&mut e);
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        let s = stream(&mut e);
        put_words(&mut e, &s, &[2]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, VTABLE_TEMPLATE_ARRAY);
        e.call(0x00cbb050, &args![this, s.stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, s.stream]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID), vec![vec![s.stream]; 2]);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(s.file + 4), 4);
    }

    #[test]
    fn fn_00cbb0c0_allocates_constructs_and_names_a_template() {
        let mut e = logged_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(EXTRA_DATA_SET_NAME, |_, _| Ret::default());
        let this = e.call(0x00cbb0c0, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(e.mem.u32(this), VTABLE_TEMPLATE);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SET_NAME),
            vec![vec![this, RAGDOLL_TEMPLATE_NAME]]
        );
        // A failed allocation still names the null object.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cbb0c0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SET_NAME)[1],
            vec![0, RAGDOLL_TEMPLATE_NAME]
        );
    }

    const CAST: u32 = 0x0065_3290;
    const BODY_CAST_TYPE: u32 = 0x0120_43f8;
    const BODY_TARGET: u32 = 0x00ca_86c0;
    const SET_MASS: u32 = 0x00ca_88c0;
    const NOTHING: u32 = 0x00a2_9680;
    const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d0c0;
    const CLONE: u32 = 0x00cf_4a30;
    const SET_DATA: u32 = 0x00c8_9ac0;
    const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
    const TARGET_MASS: u32 = 0x7fff_0200;
    const SET_SHAPE: u32 = 0x7fff_0204;
    const GET_WORLD: u32 = 0x7fff_0208;
    const SET_WORLD: u32 = 0x7fff_020c;
    const CREATE_CINFO: u32 = 0x7fff_0210;
    const MAP_DELETE: u32 = 0x7fff_0214;
    const SHAPE_ID: u32 = 0x4444;
    const WORLD_WORD: u32 = 0x1234_5678;
    /// Virtual slots of the constraints the rebuild creates: delete (4), set world (0x9c) and
    /// `CreateCinfo` (0xb4).
    const CONSTRAINT_SLOTS: [(u32, u32); 3] =
        [(4, DELETE), (0x9c, SET_WORLD), (0xb4, CREATE_CINFO)];

    /// The objects `00cba6f0` walks, with doubles for everything it calls outside the file.
    struct World {
        this: u32,
        source: u32,
        cast: u32,
        holder: u32,
        body: u32,
        shape_owner: u32,
        shape: u32,
        target: u32,
        old_constraint: u32,
        elements: Vec<u32>,
        map: u32,
    }

    /// A world whose template has one cinfo element per type word.
    fn world(e: &mut Engine, types: &[u32]) -> World {
        interlocked(e);
        constructor_doubles(e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            SET_MASS,
            NOTHING,
            TARGET_MASS,
            SET_SHAPE,
            SET_WORLD,
            MAP_DELETE,
            DELETE,
            SET_DATA,
            REMOVE_REFERENCE,
            APPLY_SCALE_BALL_AND_SOCKET,
            APPLY_SCALE_HINGE,
            APPLY_SCALE_MALLEABLE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CAPSULE_SHAPE_INIT, |e, a| {
            e.mem.set_u32(a[0] + 8, SHAPE_ID);
            Ret::default()
        });
        e.register(CREATE_CINFO, |e, _| {
            let block = e.mem.alloc(0x20);
            e.mem.set_u32(block + 0xc, 0xaaa);
            e.mem.set_u32(block + 0x10, 0xbbb);
            (block + 4).into_ret()
        });
        e.register(CLONE, |e, a| {
            for offset in [0x10, 0x14, 0x18] {
                assert_eq!(e.mem.u32(a[1] + offset), 1.0f32.to_bits());
            }
            e.mem.alloc(0x10).into_ret()
        });
        let owner = e.mem.alloc(8);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x1c, owner);
        let cast = e.mem.alloc(0x20);
        let holder_table = vtable_with(e, &[(0x94, GET_WORLD)]);
        let holder = object_with(e, holder_table);
        e.mem.set_u32(cast + 0x10, holder);
        let world_object = e.mem.alloc(0x360);
        e.mem.set_u32(world_object + 0x350, WORLD_WORD);
        e.register_double(GET_WORLD, move |_, _| world_object.into_ret());
        e.register_double(CAST, move |_, _| cast.into_ret());
        let body_table = vtable_with(e, &[(0xc, SET_SHAPE)]);
        let body = e.mem.alloc(0xa0);
        e.mem.set_u32(body, body_table);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(body + 0x10, node);
        let shape = e.mem.alloc(0x40);
        let shape_owner = e.mem.alloc(0x20);
        e.mem.set_u32(shape_owner + 4, 2);
        e.mem.set_u32(shape_owner + 8, shape);
        e.mem.set_u32(node + 8, shape_owner);
        e.mem.set_u32(holder + 8, body);
        let target_table = vtable_with(e, &[(0xc, TARGET_MASS)]);
        let target = object_with(e, target_table);
        e.register_double(BODY_TARGET, move |_, _| target.into_ret());
        let constraint_table = vtable_with(e, &CONSTRAINT_SLOTS);
        let old_constraint = object_with(e, constraint_table);
        e.mem.set_u32(old_constraint + 4, 1);
        e.mem.set_u32(holder + 0x14, old_constraint);
        for vtable in [
            VTABLE_BALL_AND_SOCKET,
            VTABLE_MALLEABLE,
            VTABLE_RAGDOLL_CONSTRAINT,
        ] {
            for (offset, function) in CONSTRAINT_SLOTS {
                e.mem.set_u32(vtable + offset, function);
            }
        }
        for construct in [HINGE_CONSTRUCT, LIMITED_HINGE_CONSTRUCT] {
            e.register_double(construct, move |e, a| {
                e.mem.set_u32(a[0], constraint_table);
                a[0].into_ret()
            });
        }
        let map_table = vtable_with(e, &[(0, MAP_DELETE)]);
        let map = object_with(e, map_table);
        e.register_double(CLONING_PROCESS_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], map);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let element_buffer = e.mem.alloc(4 * types.len().max(1) as u32);
        let type_buffer = e.mem.alloc(4 * types.len().max(1) as u32);
        let mut elements = vec![];
        for (index, kind) in types.iter().enumerate() {
            let element = e.mem.alloc(0x20);
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(element + 4, data);
            e.mem.set_u32(element_buffer + 4 * index as u32, element);
            e.mem.set_u32(type_buffer + 4 * index as u32, *kind);
            elements.push(element);
        }
        e.mem.set_u32(this + 0x10, element_buffer);
        e.mem.set_u32(this + 0x14, types.len() as u32);
        e.mem.set_u32(this + 0x20, type_buffer);
        e.mem.set_f32(this + 0x2c, 3.5);
        e.mem.set_f32(this + 0x30, 0.25);
        e.mem.set_u32(this + 0x34, 0x7fa0_0001);
        e.mem.set_f32(this + 0x38, 0.25);
        e.mem.set_u32(this + 0x3c, 0x77);
        World {
            this,
            source,
            cast,
            holder,
            body,
            shape_owner,
            shape,
            target,
            old_constraint,
            elements,
            map,
        }
    }

    #[test]
    fn fn_00cba6f0_rebuilds_the_body_and_the_constraints_from_the_template() {
        let mut e = logged_engine();
        let w = world(&mut e, &[0, 1, 2, 7, 13]);
        e.call(0x00cba6f0, &args![w.this, w.source, 2.0f32]);
        let owner = e.mem.u32(w.source + 0x1c);
        assert_eq!(calls_to(&e, CAST), vec![vec![owner, BODY_CAST_TYPE]]);
        assert_eq!(calls_to(&e, BODY_TARGET), vec![vec![w.body]]);
        assert_eq!(
            calls_to(&e, TARGET_MASS),
            vec![vec![w.target, 3.5f32.to_bits()]]
        );
        assert!(calls_to(&e, SET_MASS).is_empty());
        // The friction goes through the x87 stack: the signalling NaN is quieted.
        assert_eq!(e.mem.u32(w.body + 0x90), 0x7fe0_0001);
        assert_eq!(e.mem.u32(w.body + 0x94), 0.25f32.to_bits());
        assert_eq!(calls_to(&e, NOTHING), vec![vec![w.holder]; 2]);
        assert_eq!(e.mem.u32(w.shape_owner + 0x10), 0x77);
        let mut sizes = vec![vec![0x14u32]];
        sizes.extend(vec![vec![0x10u32]; 5]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), sizes);
        let capsules = calls_to(&e, CAPSULE_SHAPE_INIT);
        assert_eq!(capsules.len(), 1);
        let capsule = capsules[0][0];
        assert_eq!(
            capsules[0][1..],
            [w.shape + 0x20, w.shape + 0x30, 0.5f32.to_bits()]
        );
        assert_eq!(e.mem.u32(capsule + 4), 1);
        assert_eq!(e.mem.u32(capsule + 0x10), 0x77);
        assert_eq!(calls_to(&e, SET_SHAPE), vec![vec![w.body, SHAPE_ID]]);
        assert_eq!(e.mem.u32(w.shape_owner + 4), 1);
        assert_eq!(calls_to(&e, GET_WORLD), vec![vec![w.holder]]);

        let data: Vec<u32> = w
            .elements
            .iter()
            .map(|element| e.mem.u32(element + 4))
            .collect();
        let scale_pair =
            |data: u32| vec![vec![2.0f32.to_bits(), data], vec![0.5f32.to_bits(), data]];
        assert_eq!(
            calls_to(&e, APPLY_SCALE_BALL_AND_SOCKET),
            scale_pair(data[0])
        );
        let mut hinge = scale_pair(data[1]);
        hinge.extend(scale_pair(data[2]));
        hinge.extend(scale_pair(data[3]));
        assert_eq!(calls_to(&e, APPLY_SCALE_HINGE), hinge);
        assert_eq!(calls_to(&e, APPLY_SCALE_MALLEABLE), scale_pair(data[4]));
        let cloned: Vec<u32> = calls_to(&e, CLONE).iter().map(|c| c[0]).collect();
        assert_eq!(cloned, data);
        let set_data = calls_to(&e, SET_DATA);
        let set_on: Vec<u32> = set_data.iter().map(|c| c[0]).collect();
        assert_eq!(set_on, w.elements);
        let clones: Vec<Vec<u32>> = set_data.iter().map(|c| vec![c[1]]).collect();
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), clones);

        let world_calls = calls_to(&e, SET_WORLD);
        assert!(world_calls.iter().all(|c| c[1] == WORLD_WORD));
        let new: Vec<u32> = world_calls.iter().map(|c| c[0]).collect();
        assert_eq!(new.len(), 5);
        let mut replaced = vec![w.old_constraint];
        replaced.extend(&new[..4]);
        let created: Vec<u32> = calls_to(&e, CREATE_CINFO).iter().map(|c| c[0]).collect();
        assert_eq!(created, replaced);
        let deleted: Vec<Vec<u32>> = replaced.iter().map(|object| vec![*object]).collect();
        assert_eq!(calls_to(&e, DELETE), deleted);
        assert_eq!(e.mem.u32(w.holder + 0x14), new[4]);
        assert_eq!(e.mem.u32(new[4] + 4), 1);
        let hinge_table = e.mem.u32(w.old_constraint);
        assert_eq!(e.mem.u32(new[0]), VTABLE_BALL_AND_SOCKET);
        assert_eq!(e.mem.u32(new[1]), hinge_table);
        assert_eq!(e.mem.u32(new[2]), hinge_table);
        assert_eq!(e.mem.u32(new[3]), VTABLE_RAGDOLL_CONSTRAINT);
        assert_eq!(e.mem.u32(new[4]), VTABLE_MALLEABLE);
        assert_eq!(
            calls_to(&e, HINGE_CONSTRUCT),
            vec![vec![new[1], w.elements[1]]]
        );
        assert_eq!(
            calls_to(&e, LIMITED_HINGE_CONSTRUCT),
            vec![vec![new[2], w.elements[2]]]
        );
        for element in &w.elements {
            assert_eq!(e.mem.u32(element + 0xc), 0xaaa);
            assert_eq!(e.mem.u32(element + 0x10), 0xbbb);
        }
        let processes: Vec<u32> = calls_to(&e, CLONING_PROCESS_CONSTRUCT)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(processes, vec![0x101; 5]);
        assert_eq!(calls_to(&e, MAP_DELETE), vec![vec![w.map, 1]; 5]);
    }

    #[test]
    fn fn_00cba6f0_clones_for_a_nan_scale_but_not_for_zero() {
        let mut e = logged_engine();
        let w = world(&mut e, &[0]);
        let data = e.mem.u32(w.elements[0] + 4);
        e.call(0x00cba6f0, &args![w.this, w.source, 0.0f32]);
        assert_eq!(
            calls_to(&e, APPLY_SCALE_BALL_AND_SOCKET),
            vec![vec![0, data], vec![0x7f80_0000, data]]
        );
        assert!(calls_to(&e, CLONE).is_empty());
        assert!(calls_to(&e, SET_DATA).is_empty());
        let mut e = logged_engine();
        let w = world(&mut e, &[0]);
        e.call(0x00cba6f0, &args![w.this, w.source, f32::NAN]);
        assert_eq!(calls_to(&e, CLONE).len(), 1);
        let scales = calls_to(&e, APPLY_SCALE_BALL_AND_SOCKET);
        assert!(f32::from_bits(scales[0][0]).is_nan());
        assert!(f32::from_bits(scales[1][0]).is_nan());
    }

    #[test]
    fn fn_00cba6f0_sets_the_mass_directly_when_the_body_has_no_target() {
        let mut e = logged_engine();
        let w = world(&mut e, &[]);
        e.register(BODY_TARGET, |_, _| Ret::default());
        e.call(0x00cba6f0, &args![w.this, w.source, 1.0f32]);
        assert_eq!(calls_to(&e, SET_MASS), vec![vec![w.body, 3.5f32.to_bits()]]);
        assert!(calls_to(&e, TARGET_MASS).is_empty());
        assert!(calls_to(&e, CREATE_CINFO).is_empty());
    }

    #[test]
    fn fn_00cba6f0_does_nothing_without_a_body() {
        for variant in 0..4 {
            let mut e = logged_engine();
            let w = world(&mut e, &[0]);
            match variant {
                0 => e.mem.set_u32(w.source + 0x1c, 0),
                1 => e.register(CAST, |_, _| Ret::default()),
                2 => e.mem.set_u32(w.cast + 0x10, 0),
                _ => e.mem.set_u32(w.holder + 8, 0),
            }
            e.call(0x00cba6f0, &args![w.this, w.source, 2.0f32]);
            assert!(calls_to(&e, OPERATOR_NEW).is_empty(), "variant {variant}");
            assert!(calls_to(&e, BODY_TARGET).is_empty(), "variant {variant}");
        }
    }

    #[test]
    #[should_panic(expected = "read of unmapped memory at 00000000")]
    fn fn_00cba6f0_calls_through_a_null_constraint_for_an_unknown_type() {
        let mut e = logged_engine();
        let w = world(&mut e, &[99]);
        e.call(0x00cba6f0, &args![w.this, w.source, 1.0f32]);
    }
}
