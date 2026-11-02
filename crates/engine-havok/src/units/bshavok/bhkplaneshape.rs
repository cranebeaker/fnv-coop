//! `BSHavok/bhkplaneshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkPlaneShape` (0x14 bytes) wraps a Havok plane shape: `+8` holds the Havok object (its plane
//! vector is at `+0x10`), `+0xC` the construction info (`pInfo`, a 0x50-byte block with twelve
//! floats at `+0x10..+0x40`) and `+0x10` `eMaterial`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{copy_words, enter_context, push_viewer_string};
use super::bhknitristripsshape::{allocate_aligned, havok_allocator};

/// The class's own vtable (`010ca4dc`) and the vtables of the base classes its inlined
/// constructors store on the way (`bhkSerializable` `01030bf4`, `bhkShape` `010ca5c4`).
const VTABLE: u32 = 0x010c_a4dc;
const BASE_VTABLE_SERIALIZABLE: u32 = 0x0103_0bf4;
const BASE_VTABLE_SHAPE: u32 = 0x010c_a5c4;
/// Static object counts: the base classes' (`012680f8`, `01268f8c`) and the class's own
/// (`0126866c`, which the destructor decrements).
const SERIALIZABLE_COUNT: u32 = 0x0126_80f8;
const SHAPE_COUNT: u32 = 0x0126_8f8c;
const OBJECT_COUNT: u32 = 0x0126_866c;
/// The `NiRTTI` pointer word (`01268670`) that `GetViewerStrings` shows as the class name.
const CLASS_RTTI: u32 = 0x0126_8670;
/// `"Normal"` (`0101c29c`) and `"Constant"` (`010ca6a4`), the names of the two viewer strings built
/// from the plane vector.
const NAME_NORMAL: u32 = 0x0101_c29c;
const NAME_CONSTANT: u32 = 0x010c_a6a4;

/// Helpers of the game heap.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
#[cfg(test)]
const MEMORY_MANAGER: u32 = 0x011f_6238;
#[cfg(test)]
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

/// Base class methods: the `bhkRefObject` constructor (`00c87c30`), the destructor body of the
/// base (`00d54e60`), the clone lock and unlock (`00c876f0`, `00c87700`), `bhkShape::CopyMembers`
/// (`00c84df0`, `(this, destination, process)`), `CopyCinfo` (`00c84e20`), `LoadBinary`
/// (`00c84d00`), `SaveBinary` (`00c84d50`) and the viewer strings base (`00d54ed0`).
const REF_OBJECT_CONSTRUCTOR: u32 = 0x00c8_7c30;
const BASE_DESTRUCTOR_BODY: u32 = 0x00d5_4e60;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00d5_4ed0;
/// The copy of a 16-byte vector into the one at `this` (`004b4db0(this, source)`), the Havok plane
/// shape constructor (`00d55240`, `(block; three vectors)`), the release of the reference `Init`
/// holds on it (`00c905b0`).
const VECTOR_ASSIGN: u32 = 0x004b_4db0;
const HAVOK_PLANE_CONSTRUCT: u32 = 0x00d5_5240;
const HAVOK_RELEASE: u32 = 0x00c9_05b0;
/// Viewer-string builders: the class name from an `NiRTTI` (`00a7d330`, cdecl), `NiPoint3::
/// GetViewerString` (`00a59af0`, `(this = &point, name)`) and the name with a float (`00a7d550`,
/// cdecl `(name, value)`); the vector conversion that scales three lanes into three floats
/// (`00458620`, cdecl `(out, vector)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const SCALE_VECTOR: u32 = 0x0045_8620;

layout! {
    /// `bhkPlaneShape` (Xbox PDB), 0x14 bytes.
    pub struct BhkPlaneShape: 0x14 {
        /// `phkObject` (Xbox PDB, `bhkRefObject`): the Havok plane shape.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB, `bhkSerializable`): the construction info.
        0x0c pInfo: Ptr,
        /// `eMaterial` (Xbox PDB, `bhkShape`).
        0x10 eMaterial: u32,
    }
}

/// Byte-swaps the twelve words at `+0x10..+0x40` of the construction info.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for index in 0..12 {
        let address = cinfo + 0x10 + 4 * index;
        let word = e.mem.u32(address);
        e.mem.set_u32(address, word.swap_bytes());
    }
}

/// What the inlined constructors of the three base classes do after the allocation: the base
/// constructor, the vtables and object counts, and the cleared `pInfo` and `eMaterial` (the
/// class's own vtable and count come last).
fn construct(e: &mut Engine, this: u32) {
    e.call(REF_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this, BASE_VTABLE_SERIALIZABLE);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this, BASE_VTABLE_SHAPE);
    let count = e.mem.u32(SHAPE_COUNT);
    e.mem.set_u32(SHAPE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this, VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_add(1));
}

// Translated from 00ccb180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::_scalar_deleting_destructor_` (Xbox PDB): sets the vtable, decrements the
/// class's object count, runs the base destructor body (`00d54e60`) and, when bit 0 of `flags` is
/// set, the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes. Answers `this`.
pub fn bhk_plane_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkPlaneShape>,
    flags: u32,
) -> Ptr<BhkPlaneShape> {
    e.mem.set_u32(this.addr(), VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccb1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::CopyCinfo` (engine map): runs `bhkShape::CopyCinfo` (`00c84e20`) and, when the
/// shape (`this`, null allowed) has a Havok object, copies its vectors at `+0x20`, `+0x30` and
/// `+0x10` to `cinfo + 0x30`, `cinfo + 0x20` and `cinfo + 0x10` through `004b4db0`.
pub fn bhk_plane_shape_copy_cinfo(e: &mut Engine, this: Ptr<BhkPlaneShape>, cinfo: Ptr) {
    e.call(SHAPE_COPY_CINFO, &args![this, cinfo]);
    if this.is_null() {
        return;
    }
    let havok = e.get(this, BhkPlaneShape::phkObject).addr();
    if havok == 0 {
        return;
    }
    let cinfo = cinfo.addr();
    e.call(VECTOR_ASSIGN, &args![cinfo + 0x30, havok + 0x20]);
    e.call(VECTOR_ASSIGN, &args![cinfo + 0x20, havok + 0x30]);
    e.call(VECTOR_ASSIGN, &args![cinfo + 0x10, havok + 0x10]);
}

// Translated from 00ccb200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb200 - C++ exception unwinding (SEH frame state) is not translated.
/// `fn_00ccb200` (a constructing allocator for the class): takes 0x14 bytes from `operator new`
/// (`00aa13e0`) and, when that succeeded, constructs a `bhkPlaneShape` in them (the base
/// constructor `00c87c30`, the base vtables and counts, `pInfo` and `eMaterial` cleared, the
/// class's own vtable and count). Returns the object, or null.
pub fn fn_00ccb200(e: &mut Engine) -> Ptr<BhkPlaneShape> {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00ccb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::CopyMembers` (engine map): `process` is the cloning process whose float at
/// `+0x10` is a scale (stored and reloaded through the stack, quieting a signalling NaN). Takes
/// the construction info of `this` (`CreateCinfo`, virtual slot `+0xB4`); unless the scale equals
/// 1.0 (a NaN does not), multiplies the eight floats at `+0x20..+0x40` of it by the scale. Then
/// runs `bhkShape::CopyMembers` (`00c84df0`, `(destination, process)`) on `this`.
pub fn bhk_plane_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BhkPlaneShape>,
    destination: Ptr,
    process: Ptr,
) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let scale = x87_quiet(e.mem.f32(process.addr() + 0x10));
    if scale != 1.0 {
        for index in 0..8 {
            let address = cinfo + 0x20 + 4 * index;
            let value = e.mem.f32(address);
            e.mem.set_f32(address, scale * value);
        }
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, destination, process]);
}

// Translated from 00ccb350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::LoadBinary` (engine map): runs `bhkShape::LoadBinary` (`00c84d00`); when the
/// stream's byte at `+0x1E5` is not 1, takes the construction info (virtual slot `+0xB4`) and
/// byte-swaps its twelve words at `+0x10..+0x40`.
pub fn bhk_plane_shape_load_binary(e: &mut Engine, this: Ptr<BhkPlaneShape>, stream: Ptr) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        let cinfo = e.with_stack(4, |e, created| {
            e.vcall(this.addr(), 0xb4, &args![created]).u32()
        });
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00ccb3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::SaveBinary` (engine map): takes the construction info (virtual slot `+0xB4`);
/// when the stream's byte at `+0x1E4` is not 1 byte-swaps its twelve words at `+0x10..+0x40`; runs
/// `bhkShape::SaveBinary` (`00c84d50`); and swaps the words back when the byte, read again, is
/// still not 1.
pub fn bhk_plane_shape_save_binary(e: &mut Engine, this: Ptr<BhkPlaneShape>, stream: Ptr) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00ccb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb4e0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPlaneShape::CreateClone` (engine map): with the thread block's `+0x2b4` word at 0x10 and
/// the clone lock held (`00c876f0` / `00c87700`), allocates and constructs a new shape as
/// `00ccb200` does, then runs the original's `CopyMembers` (virtual slot `+0xC4`) with the new
/// shape (null when the allocation failed) and `process`. Returns the new shape.
pub fn bhk_plane_shape_create_clone(
    e: &mut Engine,
    this: Ptr<BhkPlaneShape>,
    process: Ptr,
) -> Ptr<BhkPlaneShape> {
    let (word, saved) = enter_context(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ccb200(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ccb5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb5d0 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `bhkPlaneShape::Init` (engine map): with the thread block's `+0x2b4` word at 0x10, takes 0x40
/// bytes from the Havok allocator (slot `+4` of the object at the router's `+0x10`), stores 0x40
/// in the 16-bit word at `+4`, gathers the construction info's floats into three 16-byte vectors
/// (`+0x10..+0x20`, `+0x30..+0x40`, `+0x20..+0x30`) and constructs the Havok plane shape in the
/// block (`00d55240` with the three vector addresses, in that order). Hands it to the shape
/// (virtual slot `+0x8C`), releases the creation reference (`00c905b0`) and runs `PostInit` (slot
/// `+0xBC`) with `cinfo`.
pub fn bhk_plane_shape_init(e: &mut Engine, this: Ptr<BhkPlaneShape>, cinfo: Ptr) {
    let (word, saved) = enter_context(e);
    let allocator = havok_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let info = cinfo.addr();
    // The three vectors sit one after the other on the exe's frame, 16-byte aligned.
    let shape = e.with_stack(0x40, |e, frame| {
        let third = (frame.addr() + 0xf) & !0xf;
        let second = third + 0x10;
        let first = third + 0x20;
        copy_words(e, info + 0x20, third, 4);
        copy_words(e, info + 0x30, second, 4);
        copy_words(e, info + 0x10, first, 4);
        e.call(HAVOK_PLANE_CONSTRUCT, &args![block, first, second, third])
            .u32()
    });
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(HAVOK_RELEASE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ccb730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb730 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPlaneShape::CreateCinfo` (engine map): with the thread block's `+0x2b4` word at 0x10, sets
/// the byte at `created` to whether `pInfo` was empty. If it was, takes 0x50 bytes aligned to 16
/// from the memory manager (`00aa3e40`, the shift stored in the byte before the block) and makes
/// them the construction info: the word at `+0` and the twelve floats at `+0x10..+0x40` zero;
/// stores it in `pInfo` and, when there is a Havok object, fills it from that (`CopyCinfo`,
/// `00ccb1b0`). Returns `pInfo`.
pub fn bhk_plane_shape_create_cinfo(e: &mut Engine, this: Ptr<BhkPlaneShape>, created: Ptr) -> Ptr {
    let (word, saved) = enter_context(e);
    let empty = e.get(this, BhkPlaneShape::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = allocate_aligned(e, 0x50);
        e.mem.set_u32(block, 0);
        for index in 0..12 {
            e.mem.set_f32(block + 0x10 + 4 * index, 0.0);
        }
        let has_havok_object = !e.get(this, BhkPlaneShape::phkObject).is_null();
        e.set(this, BhkPlaneShape::pInfo, Ptr::new(block));
        if has_havok_object {
            bhk_plane_shape_copy_cinfo(e, this, Ptr::new(block));
        }
    }
    let info = e.get(this, BhkPlaneShape::pInfo);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00ccb810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb810 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated; when the shape or its Havok object is missing, the plane vector is read from an uninitialised stack slot (zeros here).
/// `bhkPlaneShape::GetViewerStrings` (engine map): with the thread block's `+0x2b4` word at 0x10,
/// appends to `strings` the base class strings (`00d54ed0`) and the class name (`00a7d330` on the
/// `NiRTTI` pointer at `01268670`); then, with the plane vector (the 16 bytes at `+0x10` of the
/// Havok object), `"Normal"` with its first three lanes scaled into three floats (`00458620`;
/// `NiPoint3::GetViewerString`, `00a59af0`) and `"Constant"` with its fourth lane
/// (`00a7d550`; the float goes through the stack, quieting a signalling NaN).
pub fn bhk_plane_shape_get_viewer_strings(e: &mut Engine, this: Ptr<BhkPlaneShape>, strings: Ptr) {
    let (word, saved) = enter_context(e);
    let array = strings.addr();
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, array, text);
    // The plane vector (16-byte aligned) and the three scaled floats on the frame.
    let constant = e.with_stack(0x40, |e, frame| {
        let vector = (frame.addr() + 0xf) & !0xf;
        let point = vector + 0x10;
        if !this.is_null() {
            let havok = e.get(this, BhkPlaneShape::phkObject).addr();
            if havok != 0 {
                assert_eq!(
                    (havok + 0x10) & 0xf,
                    0,
                    "MOVAPS source is not 16-byte aligned"
                );
                for lane in 0..4 {
                    let value = e.mem.u32(havok + 0x10 + 4 * lane);
                    e.mem.set_u32(vector + 4 * lane, value);
                }
            }
        }
        e.call(SCALE_VECTOR, &args![point, vector]);
        let text = e
            .call(NI_POINT3_GET_VIEWER_STRING, &args![point, NAME_NORMAL])
            .u32();
        push_viewer_string(e, array, text);
        x87_quiet_bits(e.mem.u32(vector + 0xc))
    });
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![NAME_CONSTANT, constant])
        .u32();
    push_viewer_string(e, array, text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ccb130, bhk_plane_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccb150, bhk_height_field_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccb160, bhk_height_field_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00ccb180,
            bhk_plane_shape_scalar_deleting_destructor(
                Ptr<BhkPlaneShape>,
                u32,
            ) -> Ptr<BhkPlaneShape>
        ),
        entry!(
            0x00ccb1b0,
            bhk_plane_shape_copy_cinfo(Ptr<BhkPlaneShape>, Ptr)
        ),
        entry!(0x00ccb200, fn_00ccb200() -> Ptr<BhkPlaneShape>),
        entry!(
            0x00ccb2b0,
            bhk_plane_shape_copy_members(Ptr<BhkPlaneShape>, Ptr, Ptr)
        ),
        entry!(
            0x00ccb350,
            bhk_plane_shape_load_binary(Ptr<BhkPlaneShape>, Ptr)
        ),
        entry!(
            0x00ccb3e0,
            bhk_plane_shape_save_binary(Ptr<BhkPlaneShape>, Ptr)
        ),
        entry!(
            0x00ccb4e0,
            bhk_plane_shape_create_clone(Ptr<BhkPlaneShape>, Ptr) -> Ptr<BhkPlaneShape>
        ),
        entry!(0x00ccb5d0, bhk_plane_shape_init(Ptr<BhkPlaneShape>, Ptr)),
        entry!(
            0x00ccb730,
            bhk_plane_shape_create_cinfo(Ptr<BhkPlaneShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00ccb810,
            bhk_plane_shape_get_viewer_strings(Ptr<BhkPlaneShape>, Ptr)
        ),
    ]
}

// Translated from 00ccb160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkHeightFieldShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00d54e60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_height_field_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00d5_4e60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccb130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPlaneShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8670.
pub fn bhk_plane_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8670
}

// Translated from 00ccb150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkHeightFieldShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8f90.
pub fn bhk_height_field_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8f90
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    const ALLOCATOR_ROUTER: u32 = 0x00c8_5750;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with this unit's globals mapped and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(SERIALIZABLE_COUNT, 4);
        e.map(SHAPE_COUNT, 4);
        e.map(OBJECT_COUNT, 0x10);
        e.map(CLASS_RTTI, 4);
        e.call_log = Some(vec![]);
        e
    }

    /// An object of `size` bytes whose vtable (a zeroed block) holds the `(slot offset, target)`
    /// pairs.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// A shape whose vtable has the given slots.
    fn shape_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<BhkPlaneShape> {
        Ptr::new(object_with_slots(e, 0x14, slots))
    }

    /// A 16-byte aligned block of `size` bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        let raw = e.mem.alloc(size + 0x10);
        (raw + 0xf) & !0xf
    }

    #[test]
    fn height_field_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(0x00d5_4e60, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00ccb160, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00ccb160, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00d5_4e60), vec![vec![this]; 2]);
    }

    #[test]
    fn get_r_t_t_i_functions_return_their_constants() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_b130, &args![]).u32(), 0x0126_8670);
        assert_eq!(e.call(0x00cc_b150, &args![]).u32(), 0x0126_8f90);
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_base_body_and_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(BASE_DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = e.new_object::<BhkPlaneShape>();
        assert_eq!(
            e.call(0x00ccb180, &args![this, 0xfffe_u32]).u32(),
            this.addr()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00ccb180, &args![this, 3u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(OBJECT_COUNT), 3);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x14]]);
        assert_eq!(
            calls_to(&e, BASE_DESTRUCTOR_BODY),
            vec![vec![this.addr()]; 2]
        );
    }

    #[test]
    fn copy_cinfo_copies_the_three_vectors_of_the_havok_object() {
        let mut e = engine();
        e.register(SHAPE_COPY_CINFO, |_, _| Ret::default());
        e.register(VECTOR_ASSIGN, |_, _| Ret::default());
        let this = shape_with_slots(&mut e, &[]);
        let cinfo = 0x6000u32;
        // No Havok object: only the base copy.
        e.call(0x00ccb1b0, &args![this, cinfo]);
        assert!(calls_to(&e, VECTOR_ASSIGN).is_empty());
        // A null shape: the base copy and nothing else.
        e.call(0x00ccb1b0, &args![0u32, cinfo]);
        assert!(calls_to(&e, VECTOR_ASSIGN).is_empty());
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO).len(), 2);
        let havok = 0x7000u32;
        e.set(this, BhkPlaneShape::phkObject, Ptr::new(havok));
        e.call(0x00ccb1b0, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, VECTOR_ASSIGN),
            vec![
                vec![cinfo + 0x30, havok + 0x20],
                vec![cinfo + 0x20, havok + 0x30],
                vec![cinfo + 0x10, havok + 0x10]
            ]
        );
    }

    #[test]
    fn fn_00ccb200_constructs_the_object_or_answers_null() {
        let mut e = engine();
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 0xc, 0x99);
        e.mem.set_u32(block + 0x10, 0x77);
        e.mem.set_u32(SERIALIZABLE_COUNT, 1);
        e.mem.set_u32(SHAPE_COUNT, 2);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        assert_eq!(e.call(0x00ccb200, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(e.mem.u32(SHAPE_COUNT), 3);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR), vec![vec![block]]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ccb200, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn copy_members_scales_eight_floats_unless_the_scale_is_exactly_one() {
        // (scale bits, expected multiplier applied; None = untouched)
        let cases = [
            (2.5f32.to_bits(), Some(2.5f32)),
            (1.0f32.to_bits(), None),
            (0.0f32.to_bits(), Some(0.0)),
            (0x7fc0_0000, Some(f32::NAN)),
            // A signalling NaN is quieted on its way through the stack.
            (0x7f80_0001, Some(f32::NAN)),
        ];
        for (bits, multiplier) in cases {
            let mut e = engine();
            let cinfo = e.mem.alloc(0x50);
            for index in 0..8u32 {
                e.mem.set_f32(cinfo + 0x20 + 4 * index, 1.0 + index as f32);
            }
            e.mem.set_f32(cinfo + 0x1c, 9.0);
            e.mem.set_f32(cinfo + 0x40, 9.0);
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let process = e.mem.alloc(0x20);
            e.mem.set_u32(process + 0x10, bits);
            e.call(0x00ccb2b0, &args![this, 0x5000u32, process]);
            for index in 0..8u32 {
                let value = e.mem.f32(cinfo + 0x20 + 4 * index);
                let original = 1.0 + index as f32;
                match multiplier {
                    None => assert_eq!(value, original),
                    Some(m) if m.is_nan() => assert!(value.is_nan()),
                    Some(m) => assert_eq!(value, m * original),
                }
            }
            // The words around the eight are not touched.
            assert_eq!(e.mem.f32(cinfo + 0x1c), 9.0);
            assert_eq!(e.mem.f32(cinfo + 0x40), 9.0);
            assert_eq!(
                calls_to(&e, SHAPE_COPY_MEMBERS),
                vec![vec![this.addr(), 0x5000, process]]
            );
        }
    }

    #[test]
    fn load_binary_swaps_the_twelve_words_unless_the_stream_byte_is_1() {
        for (flag, swapped) in [(0u8, true), (1, false), (2, true)] {
            let mut e = engine();
            e.register(SHAPE_LOAD_BINARY, |_, _| Ret::default());
            let cinfo = e.mem.alloc(0x50);
            for word in 0..12 {
                e.mem.set_u32(cinfo + 0x10 + 4 * word, 0x0102_0300 + word);
            }
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let stream = e.mem.alloc(0x260);
            e.mem.set_u8(stream + 0x1e5, flag);
            e.call(0x00ccb350, &args![this, stream]);
            for word in 0..12 {
                let expected = if swapped {
                    (0x0102_0300u32 + word).swap_bytes()
                } else {
                    0x0102_0300 + word
                };
                assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * word), expected);
            }
            // The words after the twelve stay.
            assert_eq!(e.mem.u32(cinfo + 0x40), 0);
            assert_eq!(
                calls_to(&e, SHAPE_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(calls_to(&e, 0x7000_00b4).len(), usize::from(swapped));
        }
    }

    #[test]
    fn save_binary_swaps_around_the_base_save() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let mut e = engine();
            let cinfo = e.mem.alloc(0x50);
            for word in 0..12 {
                e.mem.set_u32(cinfo + 0x10 + 4 * word, 0x0a0b_0c00 + word);
            }
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let seen = Rc::new(RefCell::new(Vec::new()));
            let during = seen.clone();
            e.register_double(SHAPE_SAVE_BINARY, move |e, _| {
                *during.borrow_mut() = (0..12)
                    .map(|word| e.mem.u32(cinfo + 0x10 + 4 * word))
                    .collect();
                Ret::default()
            });
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let stream = e.mem.alloc(0x260);
            e.mem.set_u8(stream + 0x1e4, flag);
            e.call(0x00ccb3e0, &args![this, stream]);
            let inside: Vec<u32> = (0..12)
                .map(|word| {
                    let value = 0x0a0b_0c00u32 + word;
                    if swapped {
                        value.swap_bytes()
                    } else {
                        value
                    }
                })
                .collect();
            assert_eq!(*seen.borrow(), inside);
            // Afterwards they are back in the original order.
            for word in 0..12 {
                assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * word), 0x0a0b_0c00 + word);
            }
            assert_eq!(e.mem.u32(cinfo + 0x40), 0);
        }
    }

    #[test]
    fn create_clone_copies_members_into_the_new_object_under_the_lock() {
        let mut e = engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(0x7000_00c4, |_, _| Ret::default());
        let block = e.mem.alloc(0x14);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let this = shape_with_slots(&mut e, &[(0xc4, 0x7000_00c4)]);
        let clone = e.call(0x00ccb4e0, &args![this, 0x7000u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(
            calls_to(&e, 0x7000_00c4),
            vec![vec![this.addr(), block, 0x7000]]
        );
        assert_eq!(e.mem.u32(word), 0x42);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [CLONE_LOCK, CLONE_UNLOCK, 0x7000_00c4].contains(a))
            .collect();
        assert_eq!(order, [CLONE_LOCK, 0x7000_00c4, CLONE_UNLOCK]);

        // A failed allocation still runs CopyMembers, with a null clone.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ccb4e0, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x7000_00c4)[1], vec![this.addr(), 0, 0x7000]);
    }

    #[test]
    fn init_gathers_the_floats_into_three_vectors_for_the_havok_shape() {
        let mut e = engine();
        // The router's allocator object at +0x10, with slot +4 allocating.
        let allocator = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ALLOCATOR_ROUTER, move |_, _| ret(router));
        let block = e.mem.alloc(0x40);
        e.register_double(0x7000_0004, move |_, _| ret(block));
        let made = e.mem.alloc(0x10);
        let vectors = Rc::new(RefCell::new(Vec::new()));
        let seen = vectors.clone();
        e.register_double(HAVOK_PLANE_CONSTRUCT, move |e, a| {
            assert_eq!(a[1] & 0xf, 0);
            *seen.borrow_mut() = [a[1], a[2], a[3]]
                .iter()
                .map(|vector| (0..4).map(|lane| e.mem.u32(vector + 4 * lane)).collect())
                .collect::<Vec<Vec<u32>>>();
            ret(made)
        });
        e.register(HAVOK_RELEASE, |_, _| Ret::default());
        e.register(0x7000_008c, |_, _| Ret::default());
        e.register(0x7000_00bc, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0x50);
        for word in 0..12 {
            e.mem.set_u32(cinfo + 0x10 + 4 * word, 0x100 + word);
        }
        let this = shape_with_slots(&mut e, &[(0x8c, 0x7000_008c), (0xbc, 0x7000_00bc)]);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00ccb5d0, &args![this, cinfo]);

        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![allocator, 0x40]]);
        assert_eq!(e.mem.u16(block + 4), 0x40);
        // First argument: cinfo +0x10..; second: +0x30..; third: +0x20..
        assert_eq!(
            *vectors.borrow(),
            vec![
                vec![0x100, 0x101, 0x102, 0x103],
                vec![0x108, 0x109, 0x10a, 0x10b],
                vec![0x104, 0x105, 0x106, 0x107],
            ]
        );
        assert_eq!(calls_to(&e, 0x7000_008c), vec![vec![this.addr(), made]]);
        assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![made]]);
        assert_eq!(calls_to(&e, 0x7000_00bc), vec![vec![this.addr(), cinfo]]);
        assert_eq!(e.mem.u32(word), 0x42);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it_from_the_havok_object() {
        let mut e = engine();
        e.register(SHAPE_COPY_CINFO, |_, _| Ret::default());
        e.register(VECTOR_ASSIGN, |_, _| Ret::default());
        // The memory manager hands out an address that needs a shift of 7.
        let raw = e.mem.alloc(0x80) + 0x9;
        let raw = raw - (raw & 0xf) + 9;
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |e, _| {
            // Dirty memory: the function sets the floats itself.
            for offset in 0..0x50 {
                e.mem.set_u8(raw + 7 + offset, 0xaa);
            }
            ret(raw)
        });
        let this = shape_with_slots(&mut e, &[]);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let created = e.mem.alloc(4);

        let info = e.call(0x00ccb730, &args![this, created]).u32();
        let block = raw + 7;
        assert_eq!(info, block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u8(block - 1), 7);
        assert_eq!(e.get(this, BhkPlaneShape::pInfo).addr(), block);
        assert_eq!(e.mem.u32(block), 0);
        for index in 0..12 {
            assert_eq!(e.mem.u32(block + 0x10 + 4 * index), 0);
        }
        // The words the function leaves alone are the allocator's.
        assert_eq!(e.mem.u32(block + 4), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(block + 0x40), 0xaaaa_aaaa);
        // No Havok object: nothing is copied.
        assert!(calls_to(&e, SHAPE_COPY_CINFO).is_empty());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x50]]
        );
        assert_eq!(e.mem.u32(word), 0x42);

        // A second call finds the construction info and reports that it was not made.
        let again = e.call(0x00ccb730, &args![this, created]).u32();
        assert_eq!(again, block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);

        // With a Havok object, the new block is filled by CopyCinfo.
        e.set(this, BhkPlaneShape::phkObject, Ptr::new(0x7000));
        e.set(this, BhkPlaneShape::pInfo, Ptr::NULL);
        let info = e.call(0x00ccb730, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_CINFO),
            vec![vec![this.addr(), info]]
        );
        assert_eq!(calls_to(&e, VECTOR_ASSIGN).len(), 3);
    }

    /// Doubles for everything `GetViewerStrings` builds its strings with.
    fn install_viewer_doubles(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, _| ret(0xb000));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| ret(0xc000));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[2]));
            Ret::default()
        });
        stored
    }

    #[test]
    fn get_viewer_strings_lists_the_class_name_the_normal_and_the_constant() {
        let mut e = engine();
        let stored = install_viewer_doubles(&mut e);
        let converted = Rc::new(RefCell::new(Vec::new()));
        let seen = converted.clone();
        e.register_double(SCALE_VECTOR, move |e, a| {
            assert_eq!(a[1] & 0xf, 0);
            *seen.borrow_mut() = (0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)).collect();
            for lane in 0..3 {
                e.mem.set_f32(a[0] + 4 * lane, 7.0 + lane as f32);
            }
            ret(a[0])
        });
        let havok = aligned_block(&mut e, 0x40);
        for (lane, bits) in [0x1111u32, 0x2222, 0x3333, 0x7f80_0001].iter().enumerate() {
            e.mem.set_u32(havok + 0x10 + 4 * lane as u32, *bits);
        }
        let this = shape_with_slots(&mut e, &[]);
        e.set(this, BhkPlaneShape::phkObject, Ptr::new(havok));
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 20);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00ccb810, &args![this, strings]);

        assert_eq!(*stored.borrow(), [0xa000, 0xb000, 0xc000]);
        assert_eq!(*converted.borrow(), [0x1111, 0x2222, 0x3333, 0x7f80_0001]);
        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        let point = &calls_to(&e, NI_POINT3_GET_VIEWER_STRING)[0];
        assert_eq!(point[1], NAME_NORMAL);
        // The fourth lane goes on as the float, a signalling NaN quieted.
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![NAME_CONSTANT, 0x7fc0_0001]]
        );
        assert_eq!(e.mem.u32(word), 0x42);
    }

    #[test]
    fn get_viewer_strings_without_a_havok_object_still_lists_three_strings() {
        let mut e = engine();
        let stored = install_viewer_doubles(&mut e);
        e.register(SCALE_VECTOR, |_, a| ret(a[0]));
        let this = shape_with_slots(&mut e, &[]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 20);
        e.call(0x00ccb810, &args![this, strings]);
        assert_eq!(*stored.borrow(), [0xa000, 0xb000, 0xc000]);
        // The uninitialised vector reads as zero here; the constant is 0.0.
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![NAME_CONSTANT, 0]]
        );
    }
}
