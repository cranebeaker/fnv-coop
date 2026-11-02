//! `BSHavok/bhkconvextransformshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConvexTransformShape` is a `bhkShape` (0x14 bytes): its Havok shape is at `+8` and its
//! construction info at `+0xC`. The Havok shape keeps the wrapped child at `+0x18` (a `bhk`
//! wrapper whose Havok object is at `+8`) and its 4x4 transform at `+0x20` (64 bytes; the
//! translation row at `+0x50`). The construction info this class builds (`CreateCinfo`) is a
//! 16-byte aligned 0x50-byte block: radius at `+4`, the child pointer at `+8`, the transform
//! at `+0x10` (identity, zero translation row at `+0x40`).
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames
//! (`FS:[0]`) and `/GS` cookie checks of the functions marked `Translation incomplete` are
//! not modeled. The 16-byte aligned vector moves (`MOVAPS`) are done as plain byte copies;
//! their alignment faults are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope,
};

/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate method `(size)` (`00aa3e40`).
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConvexShape` methods: destructor body, `CopyCinfo`, `LoadBinary`, `SaveBinary`,
/// `GetViewerStrings`.
const CONVEX_SHAPE_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_SHAPE_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_SHAPE_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_SHAPE_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape` methods: `CopyMembers` `(source, process)`, `CanShare`, `Link`, `LinkObject`.
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_LINK: u32 = 0x00c8_4e50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
/// `NiObject::RegisterStreamables` reached through `00aa0ad0`, a jump to `00a81fe0`.
const REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `NiStream` methods (`this` is the stream): `00a645b0` (unnamed, no arguments, result
/// unused by `LoadBinary`) and `GetObjectFromLinkID` (`00a63430`).
const STREAM_UNNAMED_READ: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `bhkStream::Endian` (`00c8c020`, cdecl, one pointer): swaps the transform at `cinfo + 0x10`.
const ENDIAN: u32 = 0x00c8_c020;
/// Two unnamed cdecl helpers: `(out, matrix)` writes 36 bytes (nine floats) for the 4x4
/// matrix (`00624b10`), and `(out, source)` writes 12 bytes (three floats, `00458620`).
const MATRIX_TO_ROTATION: u32 = 0x0062_4b10;
const COPY_POINT: u32 = 0x0045_8620;
/// The Havok transform shape constructor `(child, transform, 1)` (`00d5d7a0`, `RET 0xC`) and
/// `hkReferencedObject::removeReference` (`00c905b0`).
const HK_TRANSFORM_CONSTRUCT: u32 = 0x00d5_d7a0;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The display node: constructor `(0)` (`00a5ecb0`, `this` is the 0xAC-byte block), the fixed
/// string maker (`00a5b690`, cdecl) and `SetName` (`00a5b950`, takes the address of the
/// fixed string). The fixed string `0109b220` holds is the empty string, which is not
/// released.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const NODE_NAME: u32 = 0x010c_4074;
/// Imports: `InterlockedIncrement` and `InterlockedDecrement` (stdcall, one argument).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`00a7d330`, cdecl), `NiPoint3::GetViewerString` (`00a59af0`) and
/// `NiMatrix3::GetViewerString` (`00a598f0`), the last two with `this` the value and the
/// name pushed.
const VIEWER_STRING: u32 = 0x00a7_d330;
const POINT3_VIEWER_STRING: u32 = 0x00a5_9af0;
const MATRIX3_VIEWER_STRING: u32 = 0x00a5_98f0;
/// The vtables of the classes the constructor passes through (RTTI-confirmed) and their
/// live-object counts.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const VTABLE_CONVEX_TRANSFORM_SHAPE: u32 = 0x010c_aa0c;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
const COUNT_CONVEX_TRANSFORM_SHAPE: u32 = 0x0126_86ac;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_86b0;
/// The identity transform used when there is no Havok shape (64 bytes), and the float
/// `1.0`-style default radius the construction info starts with.
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
/// The names the position and rotation viewer strings are made under.
const POSITION_NAME: u32 = 0x010c_6050;
const ROTATION_NAME: u32 = 0x010c_604c;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// Copies `len` bytes.
pub(crate) fn copy_bytes(e: &mut Engine, to: u32, from: u32, len: u32) {
    let data = e.mem.bytes(from, len);
    e.mem.write(to, &data);
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The wrapped child's Havok object: `this`, its Havok shape (`+8`), the child wrapper
/// (`+0x18`) and its Havok object (`+8`) in turn; 0 when any link is missing.
fn child_object(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        return 0;
    }
    let wrapper = e.mem.u32(shape + 0x18);
    if wrapper == 0 {
        return 0;
    }
    e.mem.u32(wrapper + 8)
}

/// The address of the 64-byte transform of `this`: the Havok shape's `+0x20`, or the
/// default at `01268370` when there is no `this` or no Havok shape.
fn transform_of(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return DEFAULT_TRANSFORM;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        DEFAULT_TRANSFORM
    } else {
        shape + 0x20
    }
}

/// One lane of `MULPS dest, source`: a NaN in the destination operand wins (quieted), then a
/// NaN in the source operand, otherwise the product.
pub(crate) fn mulps_lane(dest: u32, source: u32) -> u32 {
    let quiet_bit = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet_bit
    } else if f32::from_bits(source).is_nan() {
        source | quiet_bit
    } else {
        (f32::from_bits(dest) * f32::from_bits(source)).to_bits()
    }
}

/// Byte-swaps the four words at `cinfo + 0x40` in place, then hands `cinfo + 0x10` to
/// `bhkStream::Endian`.
fn swap_transform(e: &mut Engine, cinfo: u32) {
    for offset in [0x40, 0x44, 0x48, 0x4c] {
        let word = e.mem.u32(cinfo + offset);
        e.mem.set_u32(cinfo + offset, word.swap_bytes());
    }
    e.call(ENDIAN, &args![cinfo + 0x10]);
}

// Translated from 00cce900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce900 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexTransformShape::~bhkConvexTransformShape` (Ghidra name): sets the class vtable,
/// runs `bhkSerializable::Kill`, decrements the live-object count at `012686ac` and runs the
/// `bhkConvexShape` destructor body (`00c9fb00`).
pub fn bhk_convex_transform_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_TRANSFORM_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_CONVEX_TRANSFORM_SHAPE);
    e.set_global(COUNT_CONVEX_TRANSFORM_SHAPE, count.wrapping_sub(1));
    e.call(CONVEX_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00cce980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::QCinfoSize` (Ghidra name): takes no arguments and answers 0x50,
/// the size of the construction info this class builds.
pub fn bhk_convex_transform_shape_q_cinfo_size(_e: &mut Engine) -> u32 {
    0x50
}

// Translated from 00cce990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape` constructor (unnamed in the engine map): the base constructor
/// (`00c87c30`), `+0xC` and `+0x10` cleared, then the vtable of each class on the way down
/// (`bhkShape`, `bhkSphereRepShape`, `bhkConvexShape`, `bhkConvexTransformShape`), each with
/// its live-object count raised. Answers `this`.
pub fn fn_00cce990(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in [
        (VTABLE_SHAPE, COUNT_SHAPE),
        (VTABLE_SPHERE_REP_SHAPE, COUNT_SPHERE_REP_SHAPE),
        (VTABLE_CONVEX_SHAPE, COUNT_CONVEX_SHAPE),
        (VTABLE_CONVEX_TRANSFORM_SHAPE, COUNT_CONVEX_TRANSFORM_SHAPE),
    ] {
        e.mem.set_u32(this.addr(), vtable);
        increment_count(e, counter);
    }
    this
}

// Translated from 00ccea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::CopyCinfo` (Ghidra name): `bhkConvexShape::CopyCinfo`, then
/// the Havok shape's child wrapper pointer (`+0x18`, 0 without a Havok shape) into the
/// cinfo's `+8` and the 64-byte transform (the Havok shape's `+0x20`, or the default at
/// `01268370`) into the cinfo's `+0x10`.
pub fn bhk_convex_transform_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(CONVEX_SHAPE_COPY_CINFO, &args![this, cinfo]);
    let shape = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let wrapper = if shape == 0 {
        0
    } else {
        e.mem.u32(shape + 0x18)
    };
    e.mem.set_u32(cinfo + 8, wrapper);
    let source = transform_of(e, this);
    copy_bytes(e, cinfo + 0x10, source, 0x40);
}

// Translated from 00ccea70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccea70 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates a 0x14-byte block (`operator new`) and constructs a `bhkConvexTransformShape`
/// in it with `00cce990` (unnamed in the engine map). Answers the object, or 0 when the
/// allocation fails.
pub fn fn_00ccea70(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00cce990(e, Ptr::new(block))
}

// Translated from 00cceae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::CopyMembers` (Ghidra name), `(this, source, process)`: takes
/// the cinfo (virtual slot `0xB4`) and its child's Havok object (`+8` of the wrapper at
/// `+8`). With the float at `process + 0x10` (through the x87 stack) equal to 1.0 the child
/// is cloned (virtual slot `0x48` with `process`) only when it exists and its virtual slot
/// `0xD0` (with `process`) answers false; otherwise the four floats at the cinfo's `+0x40`
/// are multiplied by that float (`MULPS`) and the child is always cloned (a missing child
/// reads through address 0). A clone replaces the cinfo's `+8` with its Havok object
/// (`+8`), 0 when there is no clone. Then `bhkShape::CopyMembers`.
pub fn bhk_convex_transform_shape_copy_members(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    process: u32,
) {
    let cinfo = cinfo_of(e, this);
    let wrapper = e.mem.u32(cinfo + 8);
    let child = if wrapper != 0 {
        e.mem.u32(wrapper + 8)
    } else {
        0
    };
    let scale_bits = x87_quiet_bits(e.mem.u32(process + 0x10));
    let mut clone_child = true;
    if f32::from_bits(scale_bits) == 1.0 {
        clone_child = child != 0 && !e.vcall(child, 0xd0, &args![process]).bool();
    } else {
        for offset in [0x40, 0x44, 0x48, 0x4c] {
            let lane = e.mem.u32(cinfo + offset);
            e.mem.set_u32(cinfo + offset, mulps_lane(scale_bits, lane));
        }
    }
    if clone_child {
        let clone = e.vcall(child, 0x48, &args![process]).u32();
        let replacement = if clone != 0 { e.mem.u32(clone + 8) } else { 0 };
        e.mem.set_u32(cinfo + 8, replacement);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00cceb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::LoadBinary` (Ghidra name): calls the stream's unnamed method
/// `00a645b0` (result unused), runs `bhkConvexShape::LoadBinary`, takes the cinfo and
/// clears its `+8`; when the stream's byte at `+0x1E5` is not 1 swaps the transform's
/// four words at `+0x40` and runs `bhkStream::Endian` on `cinfo + 0x10`.
pub fn bhk_convex_transform_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(STREAM_UNNAMED_READ, &args![stream]);
    e.call(CONVEX_SHAPE_LOAD_BINARY, &args![this, stream]);
    let cinfo = cinfo_of(e, this);
    e.mem.set_u32(cinfo + 8, 0);
    if e.mem.u8(stream + 0x1e5) != 1 {
        swap_transform(e, cinfo);
    }
}

// Translated from 00ccec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::SaveBinary` (Ghidra name): hands the child's Havok object (0
/// without one) to the stream's virtual slot `0x30`, takes the cinfo, swaps its transform
/// (see `LoadBinary`) when the stream's byte at `+0x1E4` is not 1, runs
/// `bhkConvexShape::SaveBinary`, and swaps the transform back under the same condition.
pub fn bhk_convex_transform_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let child = child_object(e, this);
    e.vcall(stream, 0x30, &args![child]);
    let cinfo = cinfo_of(e, this);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_transform(e, cinfo);
    }
    e.call(CONVEX_SHAPE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_transform(e, cinfo);
    }
}

// Translated from 00ccecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::LinkObject` (Ghidra name): takes the cinfo, the next linked
/// object of the stream (`00a63430`) and stores that object's Havok object (`+8`, 0 when
/// there is none) at the cinfo's `+8`; then `bhkShape::LinkObject`.
pub fn bhk_convex_transform_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let object = if linked != 0 {
        e.mem.u32(linked + 8)
    } else {
        0
    };
    e.mem.set_u32(cinfo + 8, object);
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00cced20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::RegisterStreamables` (Ghidra name): registers the child's Havok
/// object through its virtual slot `0x54` with the stream, when there is one; then
/// `NiObject::RegisterStreamables` (through `00aa0ad0`), whose `AL` it answers: the exe ends on
/// that call, and `RegisterStreamables` is `bool` in the Xbox PDB.
pub fn bhk_convex_transform_shape_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
) -> bool {
    let child = child_object(e, this);
    if child != 0 {
        e.vcall(child, 0x54, &args![stream]);
    }
    e.call(REGISTER_STREAMABLES, &args![this, stream]).bool()
}

// Translated from 00cced60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::CanShare` (Ghidra name): `bhkShape::CanShare`, and while that
/// answers true and the child's Havok object exists, that object's virtual slot `0xD0`
/// with `other` decides.
pub fn bhk_convex_transform_shape_can_share(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(SHAPE_CAN_SHARE, &args![this, other]).u8() == 0 {
        return false;
    }
    let child = child_object(e, this);
    if child == 0 {
        return true;
    }
    e.vcall(child, 0xd0, &args![other]).bool()
}

// Translated from 00cceda0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cceda0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexTransformShape::CreateClone` (Ghidra name): in the Havok allocation scope and
/// between the clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x14-byte shape
/// (`operator new` and the constructor `00cce990`; null stays null) and calls this shape's
/// virtual slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_convex_transform_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ccea70(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    clone
}

// Translated from 00ccee60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccee60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexTransformShape::CreateCinfo` (Ghidra name), `(this, created)`: in the Havok
/// allocation scope stores into the byte at `created` whether the shape had no cinfo
/// (`+0xC` null). In that case it takes 0x60 bytes from the pool at `011f6238` (no null
/// check), moves forward to the next 16-byte boundary (1 to 16 bytes, recorded in the byte
/// before the block), writes the cinfo (`+0` and `+8` zero, `+4` the float at `010c72bc`
/// through the x87 stack, the transform at `+0x10` as the identity with a zero translation
/// row at `+0x40`; `+0xC` is left as the pool gave it), stores it at `+0xC` and, when the
/// shape has a Havok shape, fills it with `CopyCinfo` (`00ccea00`). Answers the cinfo at
/// `+0xC`.
pub fn bhk_convex_transform_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let raw = e.call(POOL_ALLOCATE, &args![POOL, 0x60u32]).u32();
        let adjust = 0x10 - (raw as u8 & 0xf);
        let cinfo = raw.wrapping_add(adjust as u32);
        e.mem.set_u8(cinfo.wrapping_sub(1), adjust);
        e.mem.set_u32(cinfo, 0);
        let radius = x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS));
        e.mem.set_u32(cinfo + 8, 0);
        e.mem.set_u32(cinfo + 4, radius);
        for offset in (0x10..0x50).step_by(4) {
            e.mem.set_u32(cinfo + offset, 0);
        }
        let one = 1.0f32.to_bits();
        e.mem.set_u32(cinfo + 0x10, one);
        e.mem.set_u32(cinfo + 0x24, one);
        e.mem.set_u32(cinfo + 0x38, one);
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            bhk_convex_transform_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00ccef50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccef50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexTransformShape::Init` (Ghidra name): in the Havok allocation scope builds a
/// 0x60-byte Havok transform shape from the cinfo's child (`+8`) and transform (`+0x10`)
/// with `00d5d7a0`, hands it to virtual slot `0x8C`, releases it, then calls virtual slot
/// `0xBC` with the cinfo. When the shape now has a Havok shape whose child's Havok object
/// exists, the shape's material (`+0x10`) becomes that object's material (`+0x10`) in the
/// low five bits.
pub fn bhk_convex_transform_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let block = allocate_havok(e, 0x60);
    let child = e.mem.u32(cinfo + 8);
    let shape = e
        .call(
            HK_TRANSFORM_CONSTRUCT,
            &args![block, child, cinfo + 0x10, 1u32],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    if e.mem.u32(this.addr() + 8) != 0 {
        let object = child_object(e, this);
        if object != 0 {
            let material = e.mem.u32(object + 0x10) & 0x1f;
            e.mem.set_u32(this.addr() + 0x10, material);
        }
    }
    leave_scope(e, old);
}

// Translated from 00ccf060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf060 - the FS:[0] exception-unwinding frame is not modeled.
/// In the Havok allocation scope lowers the reference count (`+4`, `InterlockedDecrement`)
/// of the child's Havok object and runs that object's virtual slot 1 (`+4`) when it reaches
/// zero (unnamed in the engine map).
pub fn fn_00ccf060(e: &mut Engine, this: Ptr) {
    let old = enter_scope(e);
    let child = child_object(e, this);
    if child != 0 {
        let left = e.call(INTERLOCKED_DECREMENT, &args![child + 4]).u32();
        if left == 0 {
            e.vcall(child, 4, &args![]);
        }
    }
    leave_scope(e, old);
}

// Translated from 00ccf100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::Link` (Ghidra name): when the low byte of `flag` is set raises
/// the reference count (`+4`, `InterlockedIncrement`) of the child's Havok object (when it
/// exists), otherwise releases it (`00ccf060`); then `bhkShape::Link` with the same word.
pub fn bhk_convex_transform_shape_link(e: &mut Engine, this: Ptr, flag: u32) {
    if flag as u8 != 0 {
        let child = child_object(e, this);
        if child != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![child + 4]);
        }
    } else {
        fn_00ccf060(e, this);
    }
    e.call(SHAPE_LINK, &args![this, flag]);
}

// Translated from 00ccf150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf150 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkConvexTransformShape::BuildDisplayGeometry` (Ghidra name), `(this, parent, extra)`:
/// in the Havok allocation scope copies the 64-byte transform (the Havok shape's `+0x20`, or
/// the default at `01268370`), derives the nine-float rotation (`00624b10`) and the
/// three-float translation row (`00458620`, from the transform's `+0x30`), makes a 0xAC-byte
/// node (constructor `00a5ecb0` with 0; a failed allocation leaves the node null, which the
/// stores below then fault on) named with the text at `010c4074` (`00a5b690`, `SetName`;
/// the name is released unless it is the empty string at `0109b220`), attaches it to
/// `parent` (virtual slot `0xDC` with `(node, 1)`), stores the translation at the node's
/// `+0x58` and the rotation at its `+0x34`, and, when the child's Havok object exists, calls
/// its virtual slot `0xD4` with `(node, extra)`.
pub fn bhk_convex_transform_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
    extra: u32,
) {
    let old = enter_scope(e);
    let source = transform_of(e, this);
    e.with_stack(0x70, |e, local| {
        let matrix = local.addr();
        let rotation = matrix + 0x40;
        let translation = matrix + 0x64;
        copy_bytes(e, matrix, source, 0x40);
        e.call(MATRIX_TO_ROTATION, &args![rotation, matrix]);
        e.call(COPY_POINT, &args![translation, matrix + 0x30]);
        let block = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
        let node = if block != 0 {
            e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        };
        let name = e.call(FIXED_STRING_CREATE, &args![NODE_NAME]).u32();
        e.with_stack(4, |e, holder| {
            e.mem.set_u32(holder.addr(), name);
            e.call(SET_NAME, &args![node, holder]);
        });
        if name != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
        e.vcall(parent, 0xdc, &args![node, 1u32]);
        copy_bytes(e, node + 0x58, translation, 0xc);
        copy_bytes(e, node + 0x34, rotation, 0x24);
        let child = child_object(e, this);
        if child != 0 {
            e.vcall(child, 0xd4, &args![node, extra]);
        }
    });
    leave_scope(e, old);
}

// Translated from 00ccf350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf350 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexTransformShape::GetViewerStrings` (Ghidra name): in the Havok allocation scope
/// runs `bhkConvexShape::GetViewerStrings`, appends the viewer string of the text in the
/// global at `012686b0`, derives the rotation (`00624b10`) and translation row (`00458620`)
/// of the transform as `BuildDisplayGeometry` does, appends the viewer string of the
/// translation under the name at `010c6050` and of the rotation under the name at
/// `010c604c`, and finally calls the child's Havok object's virtual slot `0x60` with the
/// list, when it exists.
pub fn bhk_convex_transform_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(CONVEX_SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    let source = transform_of(e, this);
    e.with_stack(0x30, |e, local| {
        let rotation = local.addr();
        let translation = rotation + 0x24;
        e.call(MATRIX_TO_ROTATION, &args![rotation, source]);
        e.call(COPY_POINT, &args![translation, source + 0x30]);
        let text = e
            .call(POINT3_VIEWER_STRING, &args![translation, POSITION_NAME])
            .u32();
        add_viewer_string(e, list, text);
        let text = e
            .call(MATRIX3_VIEWER_STRING, &args![rotation, ROTATION_NAME])
            .u32();
        add_viewer_string(e, list, text);
    });
    let child = child_object(e, this);
    if child != 0 {
        e.vcall(child, 0x60, &args![list]);
    }
    leave_scope(e, old);
}

// Translated from 00cce9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cce900`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_convex_transform_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_e900;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00cce970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexTransformShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_86b0.
pub fn bhk_convex_transform_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_86b0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cce900, bhk_convex_transform_shape_destructor(Ptr)),
        entry!(0x00cce970, bhk_convex_transform_shape_get_r_t_t_i() -> u32),
        entry!(0x00cce980, bhk_convex_transform_shape_q_cinfo_size() -> u32),
        entry!(0x00cce990, fn_00cce990(Ptr) -> Ptr),
        entry!(0x00cce9e0, bhk_convex_transform_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ccea00, bhk_convex_transform_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00ccea70, fn_00ccea70() -> Ptr),
        entry!(
            0x00cceae0,
            bhk_convex_transform_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00cceb90, bhk_convex_transform_shape_load_binary(Ptr, u32)),
        entry!(0x00ccec00, bhk_convex_transform_shape_save_binary(Ptr, u32)),
        entry!(0x00ccecc0, bhk_convex_transform_shape_link_object(Ptr, u32)),
        entry!(
            0x00cced20,
            bhk_convex_transform_shape_register_streamables(Ptr, u32) -> bool
        ),
        entry!(0x00cced60, bhk_convex_transform_shape_can_share(Ptr, u32) -> bool),
        entry!(0x00cceda0, bhk_convex_transform_shape_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00ccee60, bhk_convex_transform_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00ccef50, bhk_convex_transform_shape_init(Ptr, u32)),
        entry!(0x00ccf060, fn_00ccf060(Ptr)),
        entry!(0x00ccf100, bhk_convex_transform_shape_link(Ptr, u32)),
        entry!(
            0x00ccf150,
            bhk_convex_transform_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(
            0x00ccf350,
            bhk_convex_transform_shape_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_convex_transform_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_e900, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cce9e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cce9e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_e900), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_convex_transform_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_e970, &args![]).u32(), 0x0126_86b0);
    }

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
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

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the allocation scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// A shape (slot `0xB4` answers its `+0xC`, slot `0xC4` records) with a Havok shape whose
    /// child wrapper holds `child`. Returns `(this, havok shape, child wrapper)`.
    fn shape(e: &mut Engine, child: u32) -> (u32, u32, u32) {
        let this = object(e, &[(0xb4, 0x0900_0100), (0xc4, 0x0900_0101)]);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0101]);
        let havok = e.mem.alloc(0x80);
        let wrapper = e.mem.alloc(0x20);
        e.mem.set_u32(wrapper + 8, child);
        e.mem.set_u32(havok + 0x18, wrapper);
        e.mem.set_u32(this + 8, havok);
        (this, havok, wrapper)
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_CONVEX_TRANSFORM_SHAPE, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00cce900, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_TRANSFORM_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_CONVEX_TRANSFORM_SHAPE), 8);
        assert_eq!(
            order(&e),
            vec![0x00cce900, SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn q_cinfo_size_answers_0x50() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cce980, &args![]).u32(), 0x50);
    }

    #[test]
    fn constructor_counts_every_class_on_the_way_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x10, 6);
        assert_eq!(e.call(0x00cce990, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_TRANSFORM_SHAPE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        for counter in [
            COUNT_SHAPE,
            COUNT_SPHERE_REP_SHAPE,
            COUNT_CONVEX_SHAPE,
            COUNT_CONVEX_TRANSFORM_SHAPE,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    #[test]
    fn copy_cinfo_copies_the_child_wrapper_and_the_transform() {
        let mut e = Engine::new();
        e.map(DEFAULT_TRANSFORM, 0x40);
        e.mem.set_u32(DEFAULT_TRANSFORM, 0xd0d0);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO]);
        let (this, havok, wrapper) = shape(&mut e, 0x5555);
        for i in 0..16u32 {
            e.mem.set_u32(havok + 0x20 + 4 * i, 0x100 + i);
        }
        let cinfo = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00ccea00, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        assert_eq!(e.mem.u32(cinfo + 8), wrapper);
        for i in 0..16u32 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x100 + i);
        }
        // No Havok shape: no wrapper, the default transform.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00ccea00, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0xd0d0);
        // No shape at all: the same.
        e.mem.set_u32(cinfo + 8, 1);
        e.mem.set_u32(cinfo + 0x10, 0);
        e.call(0x00ccea00, &args![0u32, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0xd0d0);
    }

    #[test]
    fn allocate_and_construct_makes_a_0x14_byte_shape() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ccea70, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(e.mem.u32(made), VTABLE_CONVEX_TRANSFORM_SHAPE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ccea70, &args![]).u32(), 0);
    }

    /// Slots of a child: `0xD0` answers its `+0x34`, `0x48` answers its `+0x38`.
    fn cloning_child(e: &mut Engine) -> u32 {
        let child = object(e, &[(0xd0, 0x0900_0140), (0x48, 0x0900_0141)]);
        e.register(0x0900_0140, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        e.register(0x0900_0141, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x38),
            ..Ret::default()
        });
        child
    }

    #[test]
    fn copy_members_clones_the_child_unless_it_is_shared() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_COPY_MEMBERS]);
        let child = cloning_child(&mut e);
        let (this, _, wrapper) = shape(&mut e, child);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 8, wrapper);
        e.mem.set_u32(this + 0xc, cinfo);
        let clone = e.mem.alloc(0x20);
        e.mem.set_u32(clone + 8, 0xc10e);
        e.mem.set_u32(child + 0x38, clone);
        let process = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x10, 1.0);
        // Shared: the slot 0xD0 answer is true, nothing is cloned.
        e.mem.set_u32(child + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cceae0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 8), wrapper);
        assert!(calls_to(&e, 0x0900_0141).is_empty());
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x9999, process]]
        );
        assert_eq!(calls_to(&e, 0x0900_0140), vec![vec![child, process]]);
        // Not shared: the clone's Havok object replaces the wrapper.
        e.mem.set_u32(child + 0x34, 0);
        e.call(0x00cceae0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 8), 0xc10e);
        // No clone: null.
        e.mem.set_u32(cinfo + 8, wrapper);
        e.mem.set_u32(child + 0x38, 0);
        e.call(0x00cceae0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        // No child: nothing is called on it.
        e.mem.set_u32(cinfo + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cceae0, &args![this, 0x9999u32, process]);
        assert!(calls_to(&e, 0x0900_0140).is_empty());
        assert!(calls_to(&e, 0x0900_0141).is_empty());
    }

    #[test]
    fn copy_members_scales_the_translation_row_and_always_clones() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_COPY_MEMBERS]);
        let child = cloning_child(&mut e);
        let (this, _, wrapper) = shape(&mut e, child);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 8, wrapper);
        e.mem.set_u32(this + 0xc, cinfo);
        let process = e.mem.alloc(0x40);
        let clone = e.mem.alloc(0x20);
        e.mem.set_u32(clone + 8, 0xc10e);
        e.mem.set_u32(child + 0x38, clone);
        // Even a shared child is cloned when the scale is not 1.0.
        e.mem.set_u32(child + 0x34, 1);
        e.mem.set_f32(process + 0x10, 2.0);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(cinfo + 0x40 + 4 * i as u32, value);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cceae0, &args![this, 0u32, process]);
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(cinfo + 0x40 + 4 * i)).collect();
        assert_eq!(lanes, vec![2.0, 4.0, 6.0, 8.0]);
        assert!(calls_to(&e, 0x0900_0140).is_empty());
        assert_eq!(e.mem.u32(cinfo + 8), 0xc10e);
        // A signalling NaN scale is quieted and wins every lane.
        e.mem.set_u32(process + 0x10, 0x7f80_0001);
        e.mem.set_u32(cinfo + 8, wrapper);
        e.call(0x00cceae0, &args![this, 0u32, process]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(cinfo + 0x40 + 4 * i), 0x7fc0_0001);
        }
        // A NaN lane under an ordinary scale comes out quiet.
        e.mem.set_f32(process + 0x10, 0.5);
        e.mem.set_u32(cinfo + 0x40, 0x7f80_0002);
        e.mem.set_f32(cinfo + 0x44, 8.0);
        e.mem.set_u32(cinfo + 8, wrapper);
        e.call(0x00cceae0, &args![this, 0u32, process]);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x7fc0_0002);
        assert_eq!(e.mem.f32(cinfo + 0x44), 4.0);
    }

    /// A stream whose slot `0x30` records the object handed to it, its block big enough for
    /// the byte-order flag at `+0x1E4` (an `object` block is only 0x100 bytes: the flag would
    /// lie in whatever is allocated next, such as the game thread's stack for locals).
    fn stream(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x30, 0x0900_0150);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream, vtable);
        quiet(e, &[0x0900_0150]);
        stream
    }

    fn cinfo_with_lanes(e: &mut Engine, this: u32) -> u32 {
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, cinfo);
        for i in 0..4 {
            e.mem.set_u32(cinfo + 0x40 + 4 * i, 0x0102_0304 + i);
        }
        cinfo
    }

    #[test]
    fn load_binary_clears_the_child_and_swaps_unless_the_stream_byte_is_one() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[STREAM_UNNAMED_READ, CONVEX_SHAPE_LOAD_BINARY, ENDIAN],
        );
        let (this, _, _) = shape(&mut e, 0);
        let cinfo = cinfo_with_lanes(&mut e, this);
        e.mem.set_u32(cinfo + 8, 0x7777);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cceb90, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert!(calls_to(&e, ENDIAN).is_empty());
        assert_eq!(
            order(&e)[..3],
            [0x00cceb90, STREAM_UNNAMED_READ, CONVEX_SHAPE_LOAD_BINARY]
        );
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cceb90, &args![this, stream]);
        for i in 0..4 {
            assert_eq!(
                e.mem.u32(cinfo + 0x40 + 4 * i),
                (0x0102_0304 + i).swap_bytes()
            );
        }
        assert_eq!(calls_to(&e, ENDIAN), vec![vec![cinfo + 0x10]]);
        assert_eq!(calls_to(&e, STREAM_UNNAMED_READ), vec![vec![stream]; 2]);
    }

    #[test]
    fn save_binary_hands_over_the_child_and_swaps_around_the_base_save() {
        let mut e = Engine::new();
        quiet(&mut e, &[CONVEX_SHAPE_SAVE_BINARY, ENDIAN]);
        let (this, _, _) = shape(&mut e, 0xc0de);
        let cinfo = cinfo_with_lanes(&mut e, this);
        let stream = stream(&mut e);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ccec00, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x0900_0150), vec![vec![stream, 0xc0de]]);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert!(calls_to(&e, ENDIAN).is_empty());
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccec00, &args![this, stream]);
        // Swapped twice: the original value is back, with one Endian call on each side.
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert_eq!(
            order(&e),
            vec![
                0x00ccec00,
                0x0900_0150,
                0x0900_0100,
                ENDIAN,
                CONVEX_SHAPE_SAVE_BINARY,
                ENDIAN
            ]
        );
        assert_eq!(calls_to(&e, ENDIAN), vec![vec![cinfo + 0x10]; 2]);
        // No Havok shape: the stream gets 0.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccec00, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x0900_0150), vec![vec![stream, 0]]);
    }

    #[test]
    fn link_object_takes_the_havok_object_of_the_linked_object() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_LINK_OBJECT]);
        let (this, _, _) = shape(&mut e, 0);
        let cinfo = cinfo_with_lanes(&mut e, this);
        let linked = e.mem.alloc(0x20);
        e.mem.set_u32(linked + 8, 0xbee);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut answers = vec![0, linked];
            move |_, _| Ret {
                eax: answers.pop().unwrap(),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00ccecc0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 8), 0xbee);
        e.call(0x00ccecc0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(
            calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x5000]; 2]
        );
        assert_eq!(calls_to(&e, SHAPE_LINK_OBJECT), vec![vec![this, 0x5000]; 2]);
    }

    #[test]
    fn register_streamables_registers_the_child_then_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[REGISTER_STREAMABLES, 0x0900_0160]);
        let child = object(&mut e, &[(0x54, 0x0900_0160)]);
        let (this, _, _) = shape(&mut e, child);
        e.call_log = Some(vec![]);
        e.call(0x00cced20, &args![this, 0x6000u32]);
        assert_eq!(
            order(&e),
            vec![0x00cced20, 0x0900_0160, REGISTER_STREAMABLES]
        );
        assert_eq!(calls_to(&e, 0x0900_0160), vec![vec![child, 0x6000]]);
        assert_eq!(calls_to(&e, REGISTER_STREAMABLES), vec![vec![this, 0x6000]]);
        // No child: only the base.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cced20, &args![this, 0x6000u32]);
        assert_eq!(order(&e), vec![0x00cced20, REGISTER_STREAMABLES]);
    }

    #[test]
    fn register_streamables_answers_the_base_al() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0900_0160]);
        e.register(REGISTER_STREAMABLES, |_, _| 0x0000_0101u32.into_ret());
        let child = object(&mut e, &[(0x54, 0x0900_0160)]);
        let (this, _, _) = shape(&mut e, child);
        assert!(e.call(0x00cced20, &args![this, 0x6000u32]).bool());
        e.register(REGISTER_STREAMABLES, |_, _| 0x0000_0100u32.into_ret());
        assert!(!e.call(0x00cced20, &args![this, 0x6000u32]).bool());
    }

    #[test]
    fn can_share_asks_the_child_only_while_the_base_agrees() {
        let mut e = Engine::new();
        e.register(SHAPE_CAN_SHARE, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let child = object(&mut e, &[(0xd0, 0x0900_0170)]);
        e.register(0x0900_0170, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        let (this, _, _) = shape(&mut e, child);
        e.mem.set_u32(child + 0x34, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00cced60, &args![this, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_0170), vec![vec![child, 1]]);
        e.mem.set_u32(child + 0x34, 0);
        assert!(!e.call(0x00cced60, &args![this, 1u32]).bool());
        // The base says no: the child is not asked.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00cced60, &args![this, 0u32]).bool());
        assert!(calls_to(&e, 0x0900_0170).is_empty());
        // No child: the base answer stands.
        e.mem.set_u32(this + 8, 0);
        assert!(e.call(0x00cced60, &args![this, 1u32]).bool());
    }

    #[test]
    fn create_clone_brackets_the_new_shape_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let (this, _, _) = shape(&mut e, 0);
        quiet(&mut e, &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00cceda0, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_CONVEX_TRANSFORM_SHAPE);
        assert_eq!(
            order(&e),
            vec![
                0x00cceda0,
                CLONE_LOCK,
                OPERATOR_NEW,
                BASE_CONSTRUCT,
                0x0900_0101,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, clone, 0x4242]]);
        assert_eq!(scope(&mut e), 0x77);
        // A failed allocation clones into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cceda0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
    }

    #[test]
    fn create_cinfo_builds_an_aligned_identity_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO]);
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x7f80_0001);
        e.map(DEFAULT_TRANSFORM, 0x40);
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let (this, _, _) = shape(&mut e, 0);
        e.mem.set_u32(this + 8, 0);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00ccee60, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x60]]);
        assert_eq!(cinfo % 16, 0);
        let adjust = e.mem.u8(cinfo - 1) as u32;
        assert!((1..=16).contains(&adjust));
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        let one = 1.0f32.to_bits();
        for offset in (0x10..0x50).step_by(4) {
            let expected = if offset == 0x10 || offset == 0x24 || offset == 0x38 {
                one
            } else {
                0
            };
            assert_eq!(e.mem.u32(cinfo + offset), expected, "offset {offset:x}");
        }
        assert!(calls_to(&e, CONVEX_SHAPE_COPY_CINFO).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccee60, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        // With a Havok shape, CopyCinfo (the shape's own) fills it in.
        let (other, _, _) = shape(&mut e, 0);
        e.call_log = Some(vec![]);
        let filled = e.call(0x00ccee60, &args![other, created]).u32();
        assert_eq!(e.mem.u32(other + 0xc), filled);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![other, filled]]
        );
    }

    #[test]
    fn init_builds_the_havok_shape_from_the_cinfo_and_takes_the_child_material() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        // The memory router and its allocator.
        let allocator = object(&mut e, &[(4, 0x0900_0200)]);
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(0x00c8_5750, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        e.register(HK_TRANSFORM_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0x10, 0x23);
        let this = object(&mut e, &[(0x8c, 0x0900_0103), (0xbc, 0x0900_0104)]);
        let havok = e.mem.alloc(0x80);
        let wrapper = e.mem.alloc(0x20);
        e.mem.set_u32(wrapper + 8, child);
        e.mem.set_u32(havok + 0x18, wrapper);
        e.mem.set_u32(this + 8, havok);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 8, 0xc41d);
        e.call_log = Some(vec![]);
        e.call(0x00ccef50, &args![this, cinfo]);
        let built = calls_to(&e, HK_TRANSFORM_CONSTRUCT);
        assert_eq!(&built[0][1..], &[0xc41d, cinfo + 0x10, 1]);
        let block = built[0][0];
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(e.mem.u32(this + 0x10), 0x03);
        assert_eq!(scope(&mut e), 0x77);
        // Without a child the material stays.
        e.mem.set_u32(wrapper + 8, 0);
        e.mem.set_u32(this + 0x10, 0x11);
        e.call(0x00ccef50, &args![this, cinfo]);
        assert_eq!(e.mem.u32(this + 0x10), 0x11);
    }

    #[test]
    fn release_lowers_the_count_and_destroys_the_child_at_zero() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let child = object(&mut e, &[(4, 0x0900_0180)]);
        quiet(&mut e, &[0x0900_0180]);
        let (this, _, _) = shape(&mut e, child);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ccf060, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![child + 4]]);
        assert!(calls_to(&e, 0x0900_0180).is_empty());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call(0x00ccf060, &args![this]);
        assert_eq!(calls_to(&e, 0x0900_0180), vec![vec![child]]);
        assert_eq!(scope(&mut e), 0x77);
        // No child: nothing.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccf060, &args![this]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn link_references_or_releases_the_child_then_the_base_link() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let child = object(&mut e, &[(4, 0x0900_0180)]);
        quiet(&mut e, &[0x0900_0180, SHAPE_LINK, INTERLOCKED_INCREMENT]);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let (this, _, _) = shape(&mut e, child);
        e.call_log = Some(vec![]);
        e.call(0x00ccf100, &args![this, 0x101u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT), vec![vec![child + 4]]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        e.call(0x00ccf100, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![child + 4]]);
        assert_eq!(
            calls_to(&e, SHAPE_LINK),
            vec![vec![this, 0x101], vec![this, 0x100]]
        );
        // No Havok shape: the count is not touched when referencing.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccf100, &args![this, 1u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(calls_to(&e, SHAPE_LINK), vec![vec![this, 1]]);
    }

    #[test]
    fn build_display_geometry_attaches_a_named_node_with_the_transform() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(DEFAULT_TRANSFORM, 0x40);
        e.mem.set_u32(DEFAULT_TRANSFORM, 0xbbbb);
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x1234);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NODE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        // SetName stores the name it was given into the node, to be seen by the test.
        e.register(SET_NAME, |e, a| {
            let name = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 0x7c, name);
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        // The rotation helper copies the first word of the matrix and numbers the rest; the
        // point helper numbers its three words.
        e.register(MATRIX_TO_ROTATION, |e, a| {
            let first = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], first);
            for i in 1..9 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            Ret::default()
        });
        e.register(COPY_POINT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, 0x200 + i);
            }
            Ret::default()
        });
        let parent = object(&mut e, &[(0xdc, 0x0900_0190)]);
        quiet(&mut e, &[0x0900_0190, 0x0900_0191]);
        let child = object(&mut e, &[(0xd4, 0x0900_0191)]);
        let (this, havok, _) = shape(&mut e, child);
        e.mem.set_u32(havok + 0x20, 0xaaaa);
        e.call_log = Some(vec![]);
        e.call(0x00ccf150, &args![this, parent, 0x3131u32]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0xac]]);
        let attach = calls_to(&e, 0x0900_0190);
        assert_eq!(attach.len(), 1);
        let node = attach[0][1];
        assert_eq!(attach, vec![vec![parent, node, 1]]);
        assert_eq!(calls_to(&e, NODE_CONSTRUCT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, FIXED_STRING_CREATE), vec![vec![NODE_NAME]]);
        assert_eq!(e.mem.u32(node + 0x7c), 0x4000);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x4000 - 8]]);
        // The rotation of the Havok shape's transform, the translation row.
        assert_eq!(e.mem.u32(node + 0x34), 0xaaaa);
        assert_eq!(e.mem.u32(node + 0x34 + 4 * 8), 0x108);
        assert_eq!(
            [
                e.mem.u32(node + 0x58),
                e.mem.u32(node + 0x5c),
                e.mem.u32(node + 0x60)
            ],
            [0x200, 0x201, 0x202]
        );
        let rotation_calls = calls_to(&e, MATRIX_TO_ROTATION);
        let point_calls = calls_to(&e, COPY_POINT);
        assert_eq!(rotation_calls[0][0], rotation_calls[0][1] + 0x40);
        assert_eq!(point_calls[0][1], rotation_calls[0][1] + 0x30);
        assert_eq!(point_calls[0][0], rotation_calls[0][0] + 0x24);
        assert_eq!(calls_to(&e, 0x0900_0191), vec![vec![child, node, 0x3131]]);
        assert_eq!(scope(&mut e), 0x77);
        // No Havok shape: the default transform; the empty name is not released.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x00ccf150, &args![this, parent, 0x3131u32]);
        let node = calls_to(&e, 0x0900_0190)[0][1];
        assert_eq!(e.mem.u32(node + 0x34), 0xbbbb);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert!(calls_to(&e, 0x0900_0191).is_empty());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text_position_and_rotation() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        e.map(DEFAULT_TRANSFORM, 0x40);
        quiet(
            &mut e,
            &[
                CONVEX_SHAPE_GET_VIEWER_STRINGS,
                MATRIX_TO_ROTATION,
                COPY_POINT,
                0x0096_ad30,
                0x0900_01a0,
            ],
        );
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(POINT3_VIEWER_STRING, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(MATRIX3_VIEWER_STRING, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        // The list stores each added text at its index.
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let child = object(&mut e, &[(0x60, 0x0900_01a0)]);
        let (this, havok, _) = shape(&mut e, child);
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00ccf350, &args![this, list]);
        assert_eq!(
            [
                e.mem.u32(list + 0x100),
                e.mem.u32(list + 0x104),
                e.mem.u32(list + 0x108)
            ],
            [0x1112, POSITION_NAME, ROTATION_NAME]
        );
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        let rotation = calls_to(&e, MATRIX_TO_ROTATION);
        assert_eq!(rotation[0][1], havok + 0x20);
        assert_eq!(calls_to(&e, COPY_POINT)[0][1], havok + 0x50);
        assert_eq!(calls_to(&e, COPY_POINT)[0][0], rotation[0][0] + 0x24);
        assert_eq!(
            calls_to(&e, POINT3_VIEWER_STRING),
            vec![vec![rotation[0][0] + 0x24, POSITION_NAME]]
        );
        assert_eq!(
            calls_to(&e, MATRIX3_VIEWER_STRING),
            vec![vec![rotation[0][0], ROTATION_NAME]]
        );
        assert_eq!(calls_to(&e, 0x0900_01a0), vec![vec![child, list]]);
        assert_eq!(scope(&mut e), 0x77);
        // No Havok shape: the default transform and no child call.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u16(list + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccf350, &args![this, list]);
        assert_eq!(calls_to(&e, MATRIX_TO_ROTATION)[0][1], DEFAULT_TRANSFORM);
        assert!(calls_to(&e, 0x0900_01a0).is_empty());
    }
}
