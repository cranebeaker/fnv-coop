//! `BSHavok/bhktransformshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkTransformShape` (vtable `01066d44`, RTTI-confirmed) is a `bhkShape` (0x14 bytes): its
//! Havok shape is at `+8`, its construction info at `+0xC` and its material at `+0x10`. The
//! Havok shape keeps the wrapped child at `+0x14` (a `bhk` wrapper whose Havok object is at
//! `+8`) and its 4x4 transform at `+0x30` (64 bytes; the translation row at `+0x60`). The
//! construction info this class builds (`CreateCinfo`) is a 16-byte aligned 0x60-byte block
//! made by `0066d550`: the child pointer at `+4` and the transform at `+0x10` (translation
//! row at `+0x40`).
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

/// `00c84dc0`: the jump target of `00c8bf40`.
const JUMP_TARGET_00C8BF40: u32 = 0x00c8_4dc0;
/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate method `(size)` (`00aa3e40`).
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill`, and the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkShape` methods: destructor body, `LoadBinary`, `SaveBinary`, `CopyMembers`
/// `(source, process)`, `CopyCinfo`, `Link`, `CanShare`, `LinkObject`, `GetViewerStrings`.
const SHAPE_DESTRUCT: u32 = 0x00c8_4c50;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_LINK: u32 = 0x00c8_4e50;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c8_51c0;
/// `NiObject::RegisterStreamables` reached through `00aa0ad0`, a jump to `00a81fe0`.
const REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `NiStream` methods (`this` is the stream): `00a645b0` (unnamed, no arguments, result
/// unused by `LoadBinary`) and `GetObjectFromLinkID` (`00a63430`).
const STREAM_UNNAMED_READ: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `bhkStream::Endian` (`00c8c020`, cdecl, one pointer): swaps the first three rows of the
/// transform at `cinfo + 0x10`.
const ENDIAN: u32 = 0x00c8_c020;
/// `bhkMatrix3::Renormalize` (`00c8bf50`, `ECX` the matrix), run by `LoadBinary` on streams
/// older than version 0x32.
const RENORMALIZE: u32 = 0x00c8_bf50;
/// The renormalizing helper `Renormalize` hands its stack copy of the three rows to
/// (`00cb2fc0`, `ECX` = the block), and the vector copy (`004b4db0`, `ECX` = destination,
/// one stack word: the source).
const RENORMALIZE_BLOCK: u32 = 0x00cb_2fc0;
const COPY_VECTOR: u32 = 0x004b_4db0;
/// Two unnamed cdecl helpers: `(out, matrix)` writes 36 bytes (nine floats) for the 4x4
/// matrix (`00624b10`), and `(out, source)` writes 12 bytes (three floats, `00458620`).
const MATRIX_TO_ROTATION: u32 = 0x0062_4b10;
const COPY_POINT: u32 = 0x0045_8620;
/// The Havok transform shape constructor (`00cf5990`, `ECX` = the block, two stack words:
/// the child, the address of the four transform rows) and `hkReferencedObject::
/// removeReference` (`00c905b0`).
const HK_TRANSFORM_CONSTRUCT: u32 = 0x00cf_5990;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The construction info constructor (`0066d550`, `ECX` = the 16-byte aligned block,
/// answers it).
const CINFO_CONSTRUCT: u32 = 0x0066_d550;
/// The display node: constructor `(0)` (`00a5ecb0`, `this` is the 0xAC-byte block), the fixed
/// string maker (`00a5b690`, cdecl) and `SetName` (`00a5b950`, takes the address of the
/// fixed string). The fixed string `0109b220` holds is the empty string, which is not
/// released.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const NODE_NAME: u32 = 0x010c_3f4c;
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
const VTABLE_TRANSFORM_SHAPE: u32 = 0x0106_6d44;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_TRANSFORM_SHAPE: u32 = 0x0126_81b0;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_81b4;
/// The identity transform used when there is no Havok shape (64 bytes).
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
/// The names the position and rotation viewer strings are made under.
const POSITION_NAME: u32 = 0x010c_6050;
const ROTATION_NAME: u32 = 0x010c_604c;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// Copies `len` bytes.
fn copy_bytes(e: &mut Engine, to: u32, from: u32, len: u32) {
    let data = e.mem.bytes(from, len);
    e.mem.write(to, &data);
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The wrapped child's Havok object: `this`, its Havok shape (`+8`), the child wrapper
/// (`+0x14`) and its Havok object (`+8`) in turn; 0 when any link is missing.
fn child_object(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        return 0;
    }
    let wrapper = e.mem.u32(shape + 0x14);
    if wrapper == 0 {
        return 0;
    }
    e.mem.u32(wrapper + 8)
}

/// The address of the 64-byte transform of `this`: the Havok shape's `+0x30`, or the
/// default at `01268370` when there is no `this` or no Havok shape.
fn transform_of(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return DEFAULT_TRANSFORM;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        DEFAULT_TRANSFORM
    } else {
        shape + 0x30
    }
}

/// `FLD`/`FMUL`/`FSTP` of two floats through the x87 stack: both operands are quieted when
/// loaded, a NaN operand wins (the larger significand, then the positive
/// one, when both are NaN), otherwise the product rounded to a float.
fn x87_mul_bits(first: u32, second: u32) -> u32 {
    // Both operands loaded (quiet), the x87 NaN choice (#199); the product of two floats is
    // exact in f64, so it is rounded once.
    x87_store32(x87_mul(x87_load32(first), x87_load32(second)))
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

/// The constructor `00c8c090` and `00c8c4d0` both have inline: the base constructor, `+0xC`
/// and `+0x10` cleared, then the `bhkShape` and `bhkTransformShape` vtables, each with its
/// live-object count raised.
fn construct(e: &mut Engine, block: u32) {
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, VTABLE_SHAPE);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    increment_count(e, COUNT_SHAPE);
    e.mem.set_u32(block, VTABLE_TRANSFORM_SHAPE);
    increment_count(e, COUNT_TRANSFORM_SHAPE);
}

// Translated from 00c8bed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8bed0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTransformShape::~bhkTransformShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `012681b0` and runs the
/// `bhkShape` destructor body (`00c84c50`).
pub fn bhk_transform_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TRANSFORM_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_TRANSFORM_SHAPE);
    e.set_global(COUNT_TRANSFORM_SHAPE, count.wrapping_sub(1));
    e.call(SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00c8bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::IsEqual` (Xbox PDB): a 5-byte jump to `00c84dc0` (engine map: `bhkShape::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@bhkTransformShape@@UAA_NPAVNiObject@@@Z`.
pub fn bhk_transform_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C8BF40, &args![this, other]).bool()
}

// Translated from 00c8bf50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8bf50 - the /GS cookie check is not modeled.
/// `bhkMatrix3::Renormalize` (Xbox PDB), `ECX` = `this` (three rows of four floats at `+0`,
/// `+0x10` and `+0x20`): copies the rows to a stack block, hands the block to the
/// renormalizing helper (`00cb2fc0`, `ECX` = the block) and copies the three rows back
/// (`004b4db0`).
pub fn bhk_matrix3_renormalize(e: &mut Engine, this: Ptr) {
    e.with_stack(0x30, |e, rows| {
        copy_bytes(e, rows.addr(), this.addr(), 0x30);
        e.call(RENORMALIZE_BLOCK, &args![rows]);
        for offset in [0, 0x10, 0x20] {
            e.call(
                COPY_VECTOR,
                &args![this.addr() + offset, rows.addr() + offset],
            );
        }
    });
}

// Translated from 00c8c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkStream::Endian` (Xbox PDB), cdecl: byte-swaps the twelve words at `transform` in
/// place (the first three rows of a 4x4 transform).
pub fn bhk_stream_endian(e: &mut Engine, transform: u32) {
    for index in 0..12 {
        let word = e.mem.u32(transform + 4 * index);
        e.mem.set_u32(transform + 4 * index, word.swap_bytes());
    }
}

// Translated from 00c8c090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c090 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTransformShape` allocate-and-construct (unnamed in the engine map): allocates a
/// 0x14-byte block (`operator new`) and constructs a `bhkTransformShape` in it: the base
/// constructor (`00c87c30`), `+0xC` and `+0x10` cleared, the `bhkShape` and then the
/// `bhkTransformShape` vtable, each with its live-object count raised. Answers the object,
/// or 0 when the allocation fails.
pub fn fn_00c8c090(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00c8c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::CopyMembers` (Xbox PDB), `(this, source, process)`: takes the cinfo
/// (virtual slot `0xB4`) and its child's Havok object (`+8` of the wrapper at `+4`). With the
/// float at `process + 0x10` (through the x87 stack) equal to 1.0 the child is cloned
/// (virtual slot `0x48` with `process`) only when it exists and its virtual slot `0xD0`
/// (with `process`) answers false; otherwise the four floats at the cinfo's `+0x40` are
/// multiplied by that float and the child is always cloned (a missing child reads through
/// address 0). A clone replaces the cinfo's `+4` with its Havok object (`+8`), 0 when there
/// is no clone. Then `bhkShape::CopyMembers`.
pub fn bhk_transform_shape_copy_members(e: &mut Engine, this: Ptr, source: u32, process: u32) {
    let cinfo = cinfo_of(e, this);
    let wrapper = e.mem.u32(cinfo + 4);
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
            e.mem
                .set_u32(cinfo + offset, x87_mul_bits(lane, scale_bits));
        }
    }
    if clone_child {
        let clone = e.vcall(child, 0x48, &args![process]).u32();
        let replacement = if clone != 0 { e.mem.u32(clone + 8) } else { 0 };
        e.mem.set_u32(cinfo + 4, replacement);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00c8c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::LoadBinary` (Xbox PDB): calls the stream's unnamed method `00a645b0`
/// (result unused), runs `bhkShape::LoadBinary`, takes the cinfo and clears its `+4`; when
/// the stream's byte at `+0x1E5` is not 1 swaps the transform's four words at `+0x40` and
/// runs `bhkStream::Endian` on `cinfo + 0x10`; when the stream's version (`+4`) is below
/// 0x32 runs `bhkMatrix3::Renormalize` on `cinfo + 0x10`.
pub fn bhk_transform_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(STREAM_UNNAMED_READ, &args![stream]);
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    let cinfo = cinfo_of(e, this);
    e.mem.set_u32(cinfo + 4, 0);
    if e.mem.u8(stream + 0x1e5) != 1 {
        swap_transform(e, cinfo);
    }
    if e.mem.u32(stream + 4) < 0x32 {
        e.call(RENORMALIZE, &args![cinfo + 0x10]);
    }
}

// Translated from 00c8c260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::SaveBinary` (Xbox PDB): hands the child's Havok object (0 without
/// one) to the stream's virtual slot `0x30`, takes the cinfo and clears its `+4`, swaps its
/// transform (see `LoadBinary`) when the stream's byte at `+0x1E4` is not 1, runs
/// `bhkShape::SaveBinary`, clears the cinfo's `+4` again and swaps the transform back under
/// the same condition.
pub fn bhk_transform_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let child = child_object(e, this);
    e.vcall(stream, 0x30, &args![child]);
    let cinfo = cinfo_of(e, this);
    e.mem.set_u32(cinfo + 4, 0);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_transform(e, cinfo);
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    e.mem.set_u32(cinfo + 4, 0);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_transform(e, cinfo);
    }
}

// Translated from 00c8c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::LinkObject` (Xbox PDB): takes the cinfo, the next linked object of
/// the stream (`00a63430`) and stores that object's Havok object (`+8`, 0 when there is
/// none) at the cinfo's `+4`; then `bhkShape::LinkObject`.
pub fn bhk_transform_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let object = if linked != 0 {
        e.mem.u32(linked + 8)
    } else {
        0
    };
    e.mem.set_u32(cinfo + 4, object);
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c8c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::RegisterStreamables` (Xbox PDB; a `bool` there): registers the
/// child's Havok object through its virtual slot `0x54` with the stream, when there is one;
/// then `NiObject::RegisterStreamables` (through `00aa0ad0`), whose answer it answers (the
/// exe ends on that call and leaves its `EAX`).
pub fn bhk_transform_shape_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    let child = child_object(e, this);
    if child != 0 {
        e.vcall(child, 0x54, &args![stream]);
    }
    e.call(REGISTER_STREAMABLES, &args![this, stream]).bool()
}

// Translated from 00c8c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::CanShare` (Xbox PDB): `bhkShape::CanShare`, and while that answers
/// true and the child's Havok object exists, that object's virtual slot `0xD0` with `other`
/// decides.
pub fn bhk_transform_shape_can_share(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(SHAPE_CAN_SHARE, &args![this, other]).u8() == 0 {
        return false;
    }
    let child = child_object(e, this);
    if child == 0 {
        return true;
    }
    e.vcall(child, 0xd0, &args![other]).bool()
}

// Translated from 00c8c410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c410 - the /GS cookie check is not modeled.
/// `bhkTransformShape::CopyCinfo` (Xbox PDB): `bhkShape::CopyCinfo`, then the Havok shape's
/// child wrapper pointer (`+0x14`, 0 without a Havok shape) into the cinfo's `+4` and the
/// 64-byte transform (the Havok shape's `+0x30`, or the default at `01268370`) into the
/// cinfo's `+0x10`, a row at a time through the vector copy (`004b4db0`).
pub fn bhk_transform_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(SHAPE_COPY_CINFO, &args![this, cinfo]);
    let shape = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let wrapper = if shape == 0 {
        0
    } else {
        e.mem.u32(shape + 0x14)
    };
    e.mem.set_u32(cinfo + 4, wrapper);
    let source = transform_of(e, this);
    e.with_stack(0x40, |e, rows| {
        copy_bytes(e, rows.addr(), source, 0x40);
        for offset in [0, 0x10, 0x20, 0x30] {
            e.call(
                COPY_VECTOR,
                &args![cinfo + 0x10 + offset, rows.addr() + offset],
            );
        }
    });
}

// Translated from 00c8c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c4d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTransformShape::CreateClone` (Xbox PDB): in the Havok allocation scope and between
/// the clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x14-byte shape
/// (`operator new` and the constructor, inline; null stays null) and calls this shape's
/// virtual slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_transform_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block != 0 {
        construct(e, block);
    }
    e.vcall(this.addr(), 0xc4, &args![block, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    Ptr::new(block)
}

// Translated from 00c8c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c5b0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkTransformShape::Init` (Xbox PDB): in the Havok allocation scope builds a 0x70-byte
/// Havok transform shape from the cinfo's child (`+4`) and a stack copy of its transform
/// rows (`+0x10` to `+0x4F`) with `00cf5990`, hands it to virtual slot `0x8C`, releases it,
/// then calls virtual slot `0xBC` with the cinfo. When the shape now has a Havok shape whose
/// child's Havok object exists, the shape's material (`+0x10`) becomes that object's
/// material (`+0x10`) in the low five bits.
pub fn bhk_transform_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let block = allocate_havok(e, 0x70);
    let child = e.mem.u32(cinfo + 4);
    let shape = e.with_stack(0x40, |e, rows| {
        copy_bytes(e, rows.addr(), cinfo + 0x10, 0x40);
        e.call(HK_TRANSFORM_CONSTRUCT, &args![block, child, rows])
            .u32()
    });
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

// Translated from 00c8c770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c770 - the FS:[0] exception-unwinding frame is not modeled.
/// Release of the child (unnamed in the engine map): in the Havok allocation scope lowers
/// the reference count (`+4`, `InterlockedDecrement`) of the child's Havok object and runs
/// that object's virtual slot 1 (`+4`) when it reaches zero.
pub fn fn_00c8c770(e: &mut Engine, this: Ptr) {
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

// Translated from 00c8c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::Link` (Xbox PDB): when the low byte of `flag` is set raises the
/// reference count (`+4`, `InterlockedIncrement`) of the child's Havok object (when it
/// exists), otherwise releases it (`00c8c770`); then `bhkShape::Link` with the same word.
pub fn bhk_transform_shape_link(e: &mut Engine, this: Ptr, flag: u32) {
    if flag as u8 != 0 {
        let child = child_object(e, this);
        if child != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![child + 4]);
        }
    } else {
        fn_00c8c770(e, this);
    }
    e.call(SHAPE_LINK, &args![this, flag]);
}

// Translated from 00c8c860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8c860 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkTransformShape::BuildDisplayGeometry` (Xbox PDB), `(this, parent, extra)`: in the
/// Havok allocation scope copies the 64-byte transform (the Havok shape's `+0x30`, or the
/// default at `01268370`), derives the nine-float rotation (`00624b10`) and the three-float
/// translation row (`00458620`, from the transform's `+0x30`), makes a 0xAC-byte node
/// (constructor `00a5ecb0` with 0; a failed allocation leaves the node null, which the
/// stores below then fault on) named with the text at `010c3f4c` (`00a5b690`, `SetName`;
/// the name is released unless it is the empty string at `0109b220`), attaches it to
/// `parent` (virtual slot `0xDC` with `(node, 1)`), stores the translation at the node's
/// `+0x58` and the rotation at its `+0x34`, and, when the child's Havok object exists, calls
/// its virtual slot `0xD4` with `(node, extra)`.
pub fn bhk_transform_shape_build_display_geometry(
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

// Translated from 00c8ca60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ca60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTransformShape::CreateCinfo` (Xbox PDB), `(this, created)`: in the Havok allocation
/// scope stores into the byte at `created` whether the shape had no cinfo (`+0xC` null). In
/// that case it takes 0x60 bytes from the pool at `011f6238` (no null check), moves forward
/// to the next 16-byte boundary (1 to 16 bytes, recorded in the byte before the block),
/// constructs the cinfo there (`0066d550`, `ECX` = the block, answers it), stores it at
/// `+0xC` and, when the shape has a Havok shape, fills it with `CopyCinfo` (`00c8c410`).
/// Answers the cinfo at `+0xC`.
pub fn bhk_transform_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let raw = e.call(POOL_ALLOCATE, &args![POOL, 0x60u32]).u32();
        let adjust = 0x10 - (raw as u8 & 0xf);
        let aligned = raw.wrapping_add(adjust as u32);
        e.mem.set_u8(aligned.wrapping_sub(1), adjust);
        let cinfo = e.call(CINFO_CONSTRUCT, &args![aligned]).u32();
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            bhk_transform_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00c8cb20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8cb20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTransformShape::GetViewerStrings` (Xbox PDB): in the Havok allocation scope runs
/// `bhkShape::GetViewerStrings`, appends the viewer string of the text in the global at
/// `012681b4`, derives the rotation (`00624b10`) and translation row (`00458620`) of the
/// transform as `BuildDisplayGeometry` does, appends the viewer string of the translation
/// under the name at `010c6050` and of the rotation under the name at `010c604c`, and
/// finally calls the child's Havok object's virtual slot `0x60` with the list, when it
/// exists.
pub fn bhk_transform_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8_bed0, bhk_transform_shape_destructor(Ptr)),
        entry!(0x00c8_bf40, bhk_transform_shape_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c8_bf50, bhk_matrix3_renormalize(Ptr)),
        entry!(0x00c8_c020, bhk_stream_endian(u32)),
        entry!(0x00c8_c090, fn_00c8c090() -> Ptr),
        entry!(0x00c8_c130, bhk_transform_shape_copy_members(Ptr, u32, u32)),
        entry!(0x00c8_c1e0, bhk_transform_shape_load_binary(Ptr, u32)),
        entry!(0x00c8_c260, bhk_transform_shape_save_binary(Ptr, u32)),
        entry!(0x00c8_c330, bhk_transform_shape_link_object(Ptr, u32)),
        entry!(
            0x00c8_c390,
            bhk_transform_shape_register_streamables(Ptr, u32) -> bool
        ),
        entry!(0x00c8_c3d0, bhk_transform_shape_can_share(Ptr, u32) -> bool),
        entry!(0x00c8_c410, bhk_transform_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00c8_c4d0, bhk_transform_shape_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c8_c5b0, bhk_transform_shape_init(Ptr, u32)),
        entry!(0x00c8_c770, fn_00c8c770(Ptr)),
        entry!(0x00c8_c810, bhk_transform_shape_link(Ptr, u32)),
        entry!(
            0x00c8_c860,
            bhk_transform_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00c8_ca60, bhk_transform_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(
            0x00c8_cb20,
            bhk_transform_shape_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c8bf40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_bf40;
        const TARGET: u32 = 0x00c8_4dc0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

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
    /// child wrapper (at `+0x14`) holds `child`. Returns `(this, havok shape, child wrapper)`.
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
        e.mem.set_u32(havok + 0x14, wrapper);
        e.mem.set_u32(this + 8, havok);
        (this, havok, wrapper)
    }

    /// A vector copy double: 16 bytes from the source (second word) to the destination.
    fn copy_vector_double(e: &mut Engine) {
        e.register(COPY_VECTOR, |e, a| {
            let row = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &row);
            Ret::default()
        });
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_TRANSFORM_SHAPE, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, SHAPE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00c8_bed0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_TRANSFORM_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_TRANSFORM_SHAPE), 8);
        assert_eq!(
            order(&e),
            vec![0x00c8_bed0, SERIALIZABLE_KILL, SHAPE_DESTRUCT]
        );
        assert_eq!(calls_to(&e, SHAPE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn renormalize_copies_three_rows_through_the_helper_and_back() {
        let mut e = Engine::new();
        copy_vector_double(&mut e);
        // The helper changes the first and the last word of the block.
        e.register(RENORMALIZE_BLOCK, |e, a| {
            let first = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], first + 1);
            e.mem.set_u32(a[0] + 0x2c, 0xfeed);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        for index in 0..13 {
            e.mem.set_u32(this + 4 * index, 0x100 + index);
        }
        e.call_log = Some(vec![]);
        e.call(0x00c8_bf50, &args![this]);
        let block = calls_to(&e, RENORMALIZE_BLOCK)[0][0];
        assert_ne!(block, this);
        assert_eq!(e.mem.u32(this), 0x101);
        assert_eq!(e.mem.u32(this + 4), 0x101);
        assert_eq!(e.mem.u32(this + 0x2c), 0xfeed);
        assert_eq!(e.mem.u32(this + 0x28), 0x10a);
        // The fourth row is not part of it.
        assert_eq!(e.mem.u32(this + 0x30), 0x10c);
        assert_eq!(
            calls_to(&e, COPY_VECTOR),
            vec![
                vec![this, block],
                vec![this + 0x10, block + 0x10],
                vec![this + 0x20, block + 0x20]
            ]
        );
    }

    #[test]
    fn endian_swaps_twelve_words() {
        let mut e = Engine::new();
        let words = e.mem.alloc(0x40);
        for index in 0..13 {
            e.mem.set_u32(words + 4 * index, 0x0102_0300 + index);
        }
        e.call(0x00c8_c020, &args![words]);
        for index in 0..12 {
            assert_eq!(
                e.mem.u32(words + 4 * index),
                (0x0102_0300 + index).swap_bytes()
            );
        }
        assert_eq!(e.mem.u32(words + 0x30), 0x0102_030c);
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
        let made = e.call(0x00c8_c090, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u32(made), VTABLE_TRANSFORM_SHAPE);
        assert_eq!((e.mem.u32(made + 0xc), e.mem.u32(made + 0x10)), (0, 0));
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_TRANSFORM_SHAPE), 1);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c8_c090, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
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
        e.mem.set_u32(cinfo + 4, wrapper);
        e.mem.set_u32(this + 0xc, cinfo);
        let clone = e.mem.alloc(0x20);
        e.mem.set_u32(clone + 8, 0xc10e);
        e.mem.set_u32(child + 0x38, clone);
        let process = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x10, 1.0);
        // Shared: the slot 0xD0 answer is true, nothing is cloned.
        e.mem.set_u32(child + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c130, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), wrapper);
        assert!(calls_to(&e, 0x0900_0141).is_empty());
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x9999, process]]
        );
        assert_eq!(calls_to(&e, 0x0900_0140), vec![vec![child, process]]);
        // Not shared: the clone's Havok object replaces the wrapper.
        e.mem.set_u32(child + 0x34, 0);
        e.call(0x00c8_c130, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), 0xc10e);
        // No clone: null.
        e.mem.set_u32(cinfo + 4, wrapper);
        e.mem.set_u32(child + 0x38, 0);
        e.call(0x00c8_c130, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        // No child: nothing is called on it.
        e.mem.set_u32(cinfo + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c130, &args![this, 0x9999u32, process]);
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
        e.mem.set_u32(cinfo + 4, wrapper);
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
        e.call(0x00c8_c130, &args![this, 0u32, process]);
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(cinfo + 0x40 + 4 * i)).collect();
        assert_eq!(lanes, vec![2.0, 4.0, 6.0, 8.0]);
        assert!(calls_to(&e, 0x0900_0140).is_empty());
        assert_eq!(e.mem.u32(cinfo + 4), 0xc10e);
        // A signalling NaN scale is quieted and wins every lane.
        e.mem.set_u32(process + 0x10, 0x7f80_0001);
        e.mem.set_u32(cinfo + 4, wrapper);
        e.call(0x00c8_c130, &args![this, 0u32, process]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(cinfo + 0x40 + 4 * i), 0x7fc0_0001);
        }
        // A NaN lane under an ordinary scale comes out quiet.
        e.mem.set_f32(process + 0x10, 0.5);
        e.mem.set_u32(cinfo + 0x40, 0x7f80_0002);
        e.mem.set_f32(cinfo + 0x44, 8.0);
        e.mem.set_u32(cinfo + 4, wrapper);
        e.call(0x00c8_c130, &args![this, 0u32, process]);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x7fc0_0002);
        assert_eq!(e.mem.f32(cinfo + 0x44), 4.0);
    }

    /// A stream whose slot `0x30` records the object handed to it; 0x300 bytes, so its swap flag
    /// at `+0x1E4` is inside it (a store there must not land in the next block, such as the
    /// thread stack the first `with_stack` local makes, which zeroes it).
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
    fn load_binary_clears_the_child_swaps_and_renormalizes_old_streams() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[STREAM_UNNAMED_READ, SHAPE_LOAD_BINARY, ENDIAN, RENORMALIZE],
        );
        let (this, _, _) = shape(&mut e, 0);
        let cinfo = cinfo_with_lanes(&mut e, this);
        e.mem.set_u32(cinfo + 4, 0x7777);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.mem.set_u32(stream + 4, 0x32);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c1e0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert!(calls_to(&e, ENDIAN).is_empty());
        assert!(calls_to(&e, RENORMALIZE).is_empty());
        assert_eq!(
            order(&e)[..3],
            [0x00c8_c1e0, STREAM_UNNAMED_READ, SHAPE_LOAD_BINARY]
        );
        // Swapped, and an older version is renormalized after the swap.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.mem.set_u32(stream + 4, 0x31);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c1e0, &args![this, stream]);
        for i in 0..4 {
            assert_eq!(
                e.mem.u32(cinfo + 0x40 + 4 * i),
                (0x0102_0304 + i).swap_bytes()
            );
        }
        assert_eq!(calls_to(&e, ENDIAN), vec![vec![cinfo + 0x10]]);
        assert_eq!(calls_to(&e, RENORMALIZE), vec![vec![cinfo + 0x10]]);
        assert_eq!(calls_to(&e, STREAM_UNNAMED_READ), vec![vec![stream]]);
        assert_eq!(calls_to(&e, SHAPE_LOAD_BINARY), vec![vec![this, stream]]);
        let calls = order(&e);
        let endian = calls.iter().position(|a| *a == ENDIAN).unwrap();
        let renormalize = calls.iter().position(|a| *a == RENORMALIZE).unwrap();
        assert!(endian < renormalize);
    }

    #[test]
    fn save_binary_hands_over_the_child_and_swaps_around_the_base_save() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_SAVE_BINARY, ENDIAN]);
        let (this, _, _) = shape(&mut e, 0xc0de);
        let cinfo = cinfo_with_lanes(&mut e, this);
        e.mem.set_u32(cinfo + 4, 0x7777);
        let stream = stream(&mut e);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c260, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x0900_0150), vec![vec![stream, 0xc0de]]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert!(calls_to(&e, ENDIAN).is_empty());
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c260, &args![this, stream]);
        // Swapped twice: the original value is back, with one Endian call on each side.
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x0102_0304);
        assert_eq!(
            order(&e),
            vec![
                0x00c8_c260,
                0x0900_0150,
                0x0900_0100,
                ENDIAN,
                SHAPE_SAVE_BINARY,
                ENDIAN
            ]
        );
        assert_eq!(calls_to(&e, ENDIAN), vec![vec![cinfo + 0x10]; 2]);
        // No Havok shape: the stream gets 0.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c260, &args![this, stream]);
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
        e.call(0x00c8_c330, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0xbee);
        e.call(0x00c8_c330, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
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
        e.call(0x00c8_c390, &args![this, 0x6000u32]);
        assert_eq!(
            order(&e),
            vec![0x00c8_c390, 0x0900_0160, REGISTER_STREAMABLES]
        );
        assert_eq!(calls_to(&e, 0x0900_0160), vec![vec![child, 0x6000]]);
        assert_eq!(calls_to(&e, REGISTER_STREAMABLES), vec![vec![this, 0x6000]]);
        // No child: only the base.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c390, &args![this, 0x6000u32]);
        assert_eq!(order(&e), vec![0x00c8_c390, REGISTER_STREAMABLES]);
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
        assert!(e.call(0x00c8_c3d0, &args![this, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_0170), vec![vec![child, 1]]);
        e.mem.set_u32(child + 0x34, 0);
        assert!(!e.call(0x00c8_c3d0, &args![this, 1u32]).bool());
        // The base says no: the child is not asked.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c8_c3d0, &args![this, 0u32]).bool());
        assert!(calls_to(&e, 0x0900_0170).is_empty());
        // No child: the base answer stands.
        e.mem.set_u32(this + 8, 0);
        assert!(e.call(0x00c8_c3d0, &args![this, 1u32]).bool());
    }

    #[test]
    fn copy_cinfo_copies_the_child_wrapper_and_the_transform() {
        let mut e = Engine::new();
        e.map(DEFAULT_TRANSFORM, 0x40);
        e.mem.set_u32(DEFAULT_TRANSFORM, 0xd0d0);
        quiet(&mut e, &[SHAPE_COPY_CINFO]);
        copy_vector_double(&mut e);
        let (this, havok, wrapper) = shape(&mut e, 0x5555);
        for i in 0..16u32 {
            e.mem.set_u32(havok + 0x30 + 4 * i, 0x100 + i);
        }
        let cinfo = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c410, &args![this, cinfo]);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(e.mem.u32(cinfo + 4), wrapper);
        for i in 0..16u32 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x100 + i);
        }
        let destinations: Vec<u32> = calls_to(&e, COPY_VECTOR).iter().map(|c| c[0]).collect();
        assert_eq!(
            destinations,
            vec![cinfo + 0x10, cinfo + 0x20, cinfo + 0x30, cinfo + 0x40]
        );
        // No Havok shape: no wrapper, the default transform.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_c410, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0xd0d0);
        // No shape at all: the same.
        e.mem.set_u32(cinfo + 4, 1);
        e.mem.set_u32(cinfo + 0x10, 0);
        e.call(0x00c8_c410, &args![0u32, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0xd0d0);
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
        let clone = e.call(0x00c8_c4d0, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_TRANSFORM_SHAPE);
        assert_eq!(
            order(&e),
            vec![
                0x00c8_c4d0,
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
        assert_eq!(e.call(0x00c8_c4d0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
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
        // The constructor keeps a copy of the rows it was handed in the block (+0x20).
        e.register(HK_TRANSFORM_CONSTRUCT, |e, a| {
            let rows = e.mem.bytes(a[2], 0x40);
            e.mem.write(a[0] + 0x20, &rows);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0x10, 0x23);
        let this = object(&mut e, &[(0x8c, 0x0900_0103), (0xbc, 0x0900_0104)]);
        let havok = e.mem.alloc(0x80);
        let wrapper = e.mem.alloc(0x20);
        e.mem.set_u32(wrapper + 8, child);
        e.mem.set_u32(havok + 0x14, wrapper);
        e.mem.set_u32(this + 8, havok);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 4, 0xc41d);
        for i in 0..16u32 {
            e.mem.set_u32(cinfo + 0x10 + 4 * i, 0x300 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00c8_c5b0, &args![this, cinfo]);
        let built = calls_to(&e, HK_TRANSFORM_CONSTRUCT);
        assert_eq!(built[0][1], 0xc41d);
        let block = built[0][0];
        assert_ne!(built[0][2], cinfo + 0x10);
        for i in 0..16u32 {
            assert_eq!(e.mem.u32(block + 0x20 + 4 * i), 0x300 + i);
        }
        assert_eq!(e.mem.u16(block + 4), 0x70);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(e.mem.u32(this + 0x10), 0x03);
        assert_eq!(scope(&mut e), 0x77);
        // Without a child the material stays.
        e.mem.set_u32(wrapper + 8, 0);
        e.mem.set_u32(this + 0x10, 0x11);
        e.call(0x00c8_c5b0, &args![this, cinfo]);
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
        e.call(0x00c8_c770, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![child + 4]]);
        assert!(calls_to(&e, 0x0900_0180).is_empty());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call(0x00c8_c770, &args![this]);
        assert_eq!(calls_to(&e, 0x0900_0180), vec![vec![child]]);
        assert_eq!(scope(&mut e), 0x77);
        // No child: nothing.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c770, &args![this]);
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
        e.call(0x00c8_c810, &args![this, 0x101u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT), vec![vec![child + 4]]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        e.call(0x00c8_c810, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![child + 4]]);
        assert_eq!(
            calls_to(&e, SHAPE_LINK),
            vec![vec![this, 0x101], vec![this, 0x100]]
        );
        // No Havok shape: the count is not touched when referencing.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c810, &args![this, 1u32]);
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
        e.mem.set_u32(havok + 0x30, 0xaaaa);
        e.call_log = Some(vec![]);
        e.call(0x00c8_c860, &args![this, parent, 0x3131u32]);
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
        e.call(0x00c8_c860, &args![this, parent, 0x3131u32]);
        let node = calls_to(&e, 0x0900_0190)[0][1];
        assert_eq!(e.mem.u32(node + 0x34), 0xbbbb);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert!(calls_to(&e, 0x0900_0191).is_empty());
    }

    #[test]
    fn create_cinfo_builds_an_aligned_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[SHAPE_COPY_CINFO]);
        copy_vector_double(&mut e);
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1] + 0x20) + 3,
            ..Ret::default()
        });
        e.register(CINFO_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let (this, _, _) = shape(&mut e, 0);
        e.mem.set_u32(this + 8, 0);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00c8_ca60, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x60]]);
        assert_eq!(cinfo % 16, 0);
        let adjust = e.mem.u8(cinfo - 1) as u32;
        assert!((1..=16).contains(&adjust));
        assert_eq!(calls_to(&e, CINFO_CONSTRUCT), vec![vec![cinfo]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert!(calls_to(&e, SHAPE_COPY_CINFO).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_ca60, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        // With a Havok shape, CopyCinfo (the shape's own) fills it in.
        let (other, _, _) = shape(&mut e, 0);
        e.call_log = Some(vec![]);
        let filled = e.call(0x00c8_ca60, &args![other, created]).u32();
        assert_eq!(e.mem.u32(other + 0xc), filled);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO), vec![vec![other, filled]]);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text_position_and_rotation() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        quiet(
            &mut e,
            &[
                SHAPE_GET_VIEWER_STRINGS,
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
        e.call(0x00c8_cb20, &args![this, list]);
        assert_eq!(
            [
                e.mem.u32(list + 0x100),
                e.mem.u32(list + 0x104),
                e.mem.u32(list + 0x108)
            ],
            [0x1112, POSITION_NAME, ROTATION_NAME]
        );
        assert_eq!(
            calls_to(&e, SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        let rotation = calls_to(&e, MATRIX_TO_ROTATION);
        assert_eq!(rotation[0][1], havok + 0x30);
        assert_eq!(calls_to(&e, COPY_POINT)[0][1], havok + 0x60);
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
        e.call(0x00c8_cb20, &args![this, list]);
        assert_eq!(calls_to(&e, MATRIX_TO_ROTATION)[0][1], DEFAULT_TRANSFORM);
        assert!(calls_to(&e, 0x0900_01a0).is_empty());
    }

    #[test]
    fn register_streamables_answers_the_base_registration() {
        let mut e = Engine::new();
        e.register(REGISTER_STREAMABLES, |_, _| true.into_ret());
        let this = e.mem.alloc(0x40);
        assert!(e.call(0x00c8c390, &args![this, 0x5000u32]).bool());
        e.register(REGISTER_STREAMABLES, |_, _| false.into_ret());
        assert!(!e.call(0x00c8c390, &args![this, 0x5000u32]).bool());
    }
}
