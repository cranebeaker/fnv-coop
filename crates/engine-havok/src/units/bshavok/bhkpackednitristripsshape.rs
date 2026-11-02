//! `BSHavok/bhkpackednitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkPackedNiTriStripsShape` (0x14 bytes) wraps a Havok packed mesh shape: `+8` holds the Havok
//! object (its shape data is at `+0x84`: triangle count `+8`, sub-part count as a 16-bit word
//! `+0x10`, triangles `+0x14` (8 bytes each: three 16-bit vertex indices), vertices `+0x18` (12
//! bytes each) and the sub-part table `+0x24` (12 bytes each: a word, the 16-bit vertex count at
//! `+4` and the material at `+8`); the scale is the float at `+0x90`), `+0xC` the construction info
//! (`pInfo`, a 0x30-byte block aligned to 16 whose vector at `+0x10` holds the scale) and `+0x10`
//! `eMaterial`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{
    enter_context, push_viewer_string, release_fixed_string, release_reference,
};
use super::bhknitristripsshape::{allocate_aligned, havok_allocator, set_viewer_string_inline};

/// The class's vtable (`010c761c`) and the two base-class vtables its inlined constructor stores
/// on the way (`bhkSerializable` `01030bf4`, `bhkShape` `010c44e4`).
const VTABLE: u32 = 0x010c_761c;
const BASE_VTABLE_SERIALIZABLE: u32 = 0x0103_0bf4;
const BASE_VTABLE_SHAPE: u32 = 0x010c_44e4;
/// The static object counts of `bhkSerializable`, `bhkShape` and this class, and the `NiRTTI`
/// pointer word (`012682dc`) that `GetViewerStrings` shows as the class name.
const SERIALIZABLE_COUNT: u32 = 0x0126_80f8;
const SHAPE_COUNT: u32 = 0x0126_86d0;
const OBJECT_COUNT: u32 = 0x0126_82d8;
const CLASS_RTTI: u32 = 0x0126_82dc;
/// The float 1.0 (`01012068`) that the construction info's scale vector starts as, and the
/// Havok-to-game scale (`011b1864`) that the display vertices are multiplied by.
const UNIT_SCALE: u32 = 0x0101_2068;
const DISPLAY_SCALE: u32 = 0x011b_1864;
/// The name the display geometry gets (`"bhkPackedNiTriStripsShape"`, `010c3fc0`).
const DISPLAY_NAME: u32 = 0x010c_3fc0;
/// The vtable of the local `NiTArray<unsigned short>` that `BuildDisplayGeometry` collects the
/// triangle indices in (`0109b2d4`).
const INDEX_ARRAY_VTABLE: u32 = 0x0109_b2d4;

/// Names `GetViewerStrings` builds its strings with (`"Subparts"`, `"Part"`, `"MATERIAL"`).
const NAME_SUBPARTS: u32 = 0x010c_7708;
const NAME_PART: u32 = 0x010c_7700;
const NAME_MATERIAL: u32 = 0x010c_59cc;

const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `operator new` (`00aa13e0`), `operator delete` (`00aa10f0`, cdecl), the array allocator
/// (`00aa1070`, cdecl `(bytes)`), the memory manager (`011f6238`) and its allocate / free
/// (`00aa3e40` / `00aa4060`, `(manager, bytes)` / `(manager, block)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const OPERATOR_DELETE: u32 = 0x00aa_10f0;
const ARRAY_ALLOCATE: u32 = 0x00aa_1070;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The CRT `memcpy` (`00ec44d0`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;

/// Base class code this class's functions chain to: the `bhkRefObject` constructor, the
/// `bhkSerializable` kill, the base destructor body, the clone lock, `bhkShape::CopyMembers` /
/// `CopyCinfo` / `SaveBinary` / `LinkObject` / `IsEqual`, and the `GetViewerStrings` of the shape
/// collection base.
const REF_OBJECT_CONSTRUCTOR: u32 = 0x00c8_7c30;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00cd_2de0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
const SHAPE_IS_EQUAL: u32 = 0x00c8_4dc0;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00cd_2fd0;
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;

/// The Havok packed mesh shape: the constructor (`00ca39f0`, `(block, argument)`), the copy of
/// its data into a construction info (`00ca3b20`, `(havok, destination)`) and the release of the
/// creation reference (`00c905b0`).
const HAVOK_CONSTRUCT: u32 = 0x00ca_39f0;
const HAVOK_COPY_CINFO: u32 = 0x00ca_3b20;
const HAVOK_RELEASE: u32 = 0x00c9_05b0;

/// Display geometry: `NiTriShapeData` constructor (`00a7b690`), `NiGeometryData::SetData`
/// (`00a66b30`, seven arguments), the triangle setter (`00a7b700`, `(count, indices)`), the
/// `NiTriShape` constructor (`00a74480`), the fixed string helpers (`00a5b690`, `00a5b950`), the
/// `NiMaterialProperty` constructor (`00a75650`) and `NiAVObject::AttachProperty` (`00439410`).
const TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b690;
const SET_DATA: u32 = 0x00a6_6b30;
const SET_TRIANGLES: u32 = 0x00a7_b700;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const ATTACH_PROPERTY: u32 = 0x0043_9410;

/// The stream's object lookup (`00a63430`), the viewer string builders (`00a7d330`, `00a7d510`,
/// `00a7d4d0`, cdecl) and the strings a sub-part adds for itself (`00c849b0`).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_NUMBER: u32 = 0x00a7_d510;
const NI_GET_VIEWER_STRING_MATERIAL: u32 = 0x00a7_d4d0;
const PART_ADD_VIEWER_STRINGS: u32 = 0x00c8_49b0;

layout! {
    /// `bhkPackedNiTriStripsShape` (Xbox PDB), 0x14 bytes.
    pub struct BhkPackedNiTriStripsShape: 0x14 {
        /// `phkObject` (Xbox PDB, `bhkRefObject`): the Havok shape.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB, `bhkSerializable`): the construction info.
        0x0c pInfo: Ptr,
        /// `eMaterial` (Xbox PDB, `bhkShape`).
        0x10 eMaterial: u32,
    }
}

/// The Havok object (`[this + 8]`), 0 for a null shape.
fn havok_of(e: &Engine, this: u32) -> u32 {
    if this == 0 {
        0
    } else {
        e.mem.u32(this + 8)
    }
}

/// The exe moves 16-byte vectors with `MOVAPS`, which faults on an address that is not 16-byte
/// aligned.
fn assert_aligned(address: u32) {
    assert_eq!(
        address & 0xf,
        0,
        "MOVAPS address {address:#010x} is not 16-byte aligned"
    );
}

/// `SHUFPS` of one float into all four lanes and a `MOVAPS` store of the vector.
fn store_splat(e: &mut Engine, address: u32, bits: u32) {
    assert_aligned(address);
    for word in 0..4 {
        e.mem.set_u32(address + 4 * word, bits);
    }
}

/// What the inlined constructors of the three base classes do: the base constructor, the vtables
/// and object counts, and the cleared `pInfo` / `eMaterial`.
fn construct(e: &mut Engine, this: Ptr<BhkPackedNiTriStripsShape>) {
    e.call(REF_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BASE_VTABLE_SERIALIZABLE);
    e.set(this, BhkPackedNiTriStripsShape::pInfo, Ptr::NULL);
    e.set(this, BhkPackedNiTriStripsShape::eMaterial, 0);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), BASE_VTABLE_SHAPE);
    let count = e.mem.u32(SHAPE_COUNT);
    e.mem.set_u32(SHAPE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_add(1));
}

// Translated from 00a5b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<unsigned short>::SetSize` (Ghidra labels the address with the constructor name of
/// the class; the body is a resize): `this` has the element data at `+4`, the 16-bit capacity at
/// `+8`, the size at `+0xA` and the count of non-zero elements at `+0xC`. Does nothing when
/// `new_size` is the capacity. Shrinking below the size first zeroes the elements from `new_size`
/// on (counting the non-zero ones out) and sets the size. Then the capacity becomes `new_size`:
/// with 0 the data is dropped, otherwise a new block of `new_size` 16-bit elements
/// (`00aa1070`) takes the old elements and zeroes the rest. The old block is freed
/// (`00aa10f0`) in both cases.
pub fn fn_00a5b790(e: &mut Engine, this: Ptr, new_size: u32) {
    let array = this.addr();
    if new_size == u32::from(e.mem.u16(array + 8)) {
        return;
    }
    let size = e.mem.u16(array + 0xa);
    if new_size < u32::from(size) {
        let data = e.mem.u32(array + 4);
        let mut index = new_size as u16;
        if index < size {
            loop {
                let slot = data + u32::from(index) * 2;
                if e.mem.u16(slot) != 0 {
                    e.mem.set_u16(slot, 0);
                    let count = e.mem.u16(array + 0xc);
                    e.mem.set_u16(array + 0xc, count.wrapping_add(0xffff));
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(array + 0xa, new_size as u16);
    }
    let old = e.mem.u32(array + 4);
    e.mem.set_u16(array + 8, new_size as u16);
    if new_size == 0 {
        e.mem.set_u32(array + 4, 0);
    } else {
        let bytes = u32::from(new_size as u16) * 2;
        let data = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
        e.mem.set_u32(array + 4, data);
        let size = e.mem.u16(array + 0xa);
        for index in 0..u32::from(size) {
            let element = e.mem.u16(old + index * 2);
            e.mem.set_u16(data + index * 2, element);
        }
        for index in u32::from(size)..u32::from(e.mem.u16(array + 8)) {
            e.mem.set_u16(data + index * 2, 0);
        }
    }
    e.call(OPERATOR_DELETE, &args![old]);
}

// Translated from 00ca5290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5290 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::~bhkPackedNiTriStripsShape` (engine map): the destructor body. Sets
/// the vtable, kills the serializable base (`00c876a0`), decrements the class's object count and
/// runs the base destructor body (`00cd2de0`).
pub fn bhk_packed_ni_tri_strips_shape_destructor(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00ca5310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall `this`, result in `ST0`): the scale of the shape, the float
/// at `+0x90` of the Havok object (`[this + 8]`), stored and reloaded as a 32-bit float so a
/// signalling NaN comes back quiet, or 1.0 when `this` or its Havok object is null.
pub fn fn_00ca5310(e: &mut Engine, this: Ptr) -> f32 {
    if !this.is_null() {
        let havok = e.mem.u32(this.addr() + 8);
        if havok != 0 {
            return x87_quiet(e.mem.f32(havok + 0x90));
        }
    }
    1.0
}

// Translated from 00ca5360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::CopyCinfo` (engine map): runs `bhkShape::CopyCinfo` (`00c84e20`,
/// `(this, destination)`). With a Havok object copies its data into `destination` (`00ca3b20`);
/// without one (or without a shape) sets the float at `destination + 8` to 0.0 and the 16-byte
/// vector at `destination + 0x10` to four copies of the float at `01012068` (1.0). The vector
/// is stored with `MOVAPS`, so `destination + 0x10` must be 16-byte aligned.
pub fn bhk_packed_ni_tri_strips_shape_copy_cinfo(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    destination: Ptr,
) {
    e.call(SHAPE_COPY_CINFO, &args![this, destination]);
    if !this.is_null() {
        let havok = e.get(this, BhkPackedNiTriStripsShape::phkObject);
        if !havok.is_null() {
            e.call(HAVOK_COPY_CINFO, &args![havok, destination]);
            return;
        }
    }
    e.mem.set_f32(destination.addr() + 8, 0.0);
    let one = e.mem.u32(UNIT_SCALE);
    store_splat(e, destination.addr() + 0x10, one);
}

// Translated from 00ca53b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca53b0 - C++ exception unwinding (SEH frame state) is not translated.
/// Unnamed in the engine map: allocates 0x14 bytes (`00aa13e0`) and constructs a
/// `bhkPackedNiTriStripsShape` in them (the three base constructors inlined: the `bhkRefObject`
/// constructor `00c87c30`, then the vtables, the cleared `pInfo` and `eMaterial` and the three
/// object counts). Null when the allocation failed.
pub fn fn_00ca53b0(e: &mut Engine) -> Ptr<BhkPackedNiTriStripsShape> {
    let block = e
        .call(OPERATOR_NEW, &args![0x14u32])
        .ptr::<BhkPackedNiTriStripsShape>();
    if block.is_null() {
        return Ptr::NULL;
    }
    construct(e, block);
    block
}

// Translated from 00ca5460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::CopyMembers` (engine map): `process` is the cloning process whose
/// float at `+0x10` is a scale. Takes the construction info (`CreateCinfo`, virtual slot `+0xB4`),
/// multiplies the four lanes of the vector at `+0x10` of it by that scale (the float is stored
/// and reloaded through the stack, quieting a signalling NaN; the vector is loaded and stored
/// with `MOVAPS`, so it must be 16-byte aligned) and runs `bhkShape::CopyMembers` (`00c84df0`,
/// `(this, destination, process)`).
pub fn bhk_packed_ni_tri_strips_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    destination: Ptr,
    process: Ptr,
) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let scale = x87_quiet(e.mem.f32(process.addr() + 0x10));
    assert_aligned(cinfo + 0x10);
    for lane in 0..4 {
        let address = cinfo + 0x10 + 4 * lane;
        let value = e.mem.f32(address);
        e.mem.set_f32(address, scale * value);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, destination, process]);
}

// Translated from 00ca54c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::RegisterStreamables` (engine map): registers the shape data of the
/// Havok object (the object at `+0x84` of it) with the stream through its virtual slot `+0x54`,
/// then `NiObject::RegisterStreamables` (`00aa0ad0`), whose result is returned.
pub fn bhk_packed_ni_tri_strips_shape_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    stream: Ptr,
) -> bool {
    let havok = havok_of(e, this.addr());
    let data = e.mem.u32(havok + 0x84);
    e.vcall(data, 0x54, &args![stream]);
    e.call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00ca54f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::IsEqual` (engine map): the base comparison (`00c84dc0`) must pass
/// (false is returned when it does not); then the two shapes must use the same shape data (the
/// word at `+0x84` of their Havok objects).
pub fn bhk_packed_ni_tri_strips_shape_is_equal(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    other: Ptr<BhkPackedNiTriStripsShape>,
) -> bool {
    if !e.call(SHAPE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let theirs = havok_of(e, other.addr());
    let ours = havok_of(e, this.addr());
    e.mem.u32(ours + 0x84) == e.mem.u32(theirs + 0x84)
}

// Translated from 00ca5540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::QSubMaterial` (engine map): a packed sub-part index (bits 24 and
/// up) below the sub-part count of the shape data gives that sub-part's material (the word at
/// `+8` of its 12-byte record); `-1` and any index past the count give the shape's `eMaterial`
/// & 0x1F.
pub fn bhk_packed_ni_tri_strips_shape_q_sub_material(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    index: u32,
) -> u32 {
    let havok = havok_of(e, this.addr());
    let part = index >> 24;
    if index != 0xffff_ffff {
        let data = e.mem.u32(havok + 0x84);
        if part < u32::from(e.mem.u16(data + 0x10)) {
            let table = e.mem.u32(data + 0x24);
            return e.mem.u32(table + part * 12 + 8);
        }
    }
    e.get(this, BhkPackedNiTriStripsShape::eMaterial) & 0x1f
}

// Translated from 00ca5580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5580 - C++ exception unwinding (SEH frame state) is not translated.
/// Unnamed in the engine map (fastcall `this`): initialises the 0x30-byte construction info:
/// the words at `+0` and `+4` zero, the float at `+8` 0.0, the `NiPointer` at `+4` released if it
/// is set (the exe re-reads the word it just cleared, so it never is) and the vector at `+0x10`
/// four copies of the float at `01012068` (stored with `MOVAPS`: `this + 0x10` must be 16-byte
/// aligned). Answers `this`.
pub fn fn_00ca5580(e: &mut Engine, this: Ptr) -> Ptr {
    let block = this.addr();
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_f32(block + 8, 0.0);
    let object = e.mem.u32(block + 4);
    if object != 0 {
        release_reference(e, object);
        e.mem.set_u32(block + 4, 0);
    }
    let one = e.mem.u32(UNIT_SCALE);
    store_splat(e, block + 0x10, one);
    this
}

// Translated from 00ca5620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5620 - C++ exception unwinding (SEH frame state) is not translated.
/// Unnamed in the engine map (fastcall `this`): the destructor of the construction info. Releases
/// the `NiPointer` at `+4` (clearing it), then releases whatever the word holds again, as the
/// inlined smart pointer destructor does.
pub fn fn_00ca5620(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr() + 4);
    if object != 0 {
        release_reference(e, object);
        e.mem.set_u32(this.addr() + 4, 0);
    }
    let object = e.mem.u32(this.addr() + 4);
    release_reference(e, object);
}

// Translated from 00ca56b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca56b0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::CreateClone` (engine map): with the thread block's `+0x2b4` word
/// at 0x10 and the clone lock held (`00c876f0` / `00c87700`), allocates and constructs a new
/// shape as `00ca53b0` does, then runs the original's `CopyMembers` (virtual slot `+0xC4`) with
/// the new shape (null when the allocation failed) and `process`. Returns the new shape.
pub fn bhk_packed_ni_tri_strips_shape_create_clone(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    process: Ptr,
) -> Ptr<BhkPackedNiTriStripsShape> {
    let (word, saved) = enter_context(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ca53b0(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ca57a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca57a0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::CreateCinfo` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, sets the byte at `created` to whether `pInfo` was empty. If it was, takes 0x30 bytes
/// aligned to 16 from the memory manager, initialises them as the construction info (`00ca5580`),
/// stores it in `pInfo` and, when the shape has a Havok object, fills it from the shape
/// (`CopyCinfo`, `00ca5360`). Returns `pInfo`.
pub fn bhk_packed_ni_tri_strips_shape_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    created: Ptr,
) -> Ptr {
    let (word, saved) = enter_context(e);
    let empty = e.get(this, BhkPackedNiTriStripsShape::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = allocate_aligned(e, 0x30);
        let cinfo = fn_00ca5580(e, Ptr::new(block));
        e.set(this, BhkPackedNiTriStripsShape::pInfo, cinfo);
        if !e.get(this, BhkPackedNiTriStripsShape::phkObject).is_null() {
            bhk_packed_ni_tri_strips_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let info = e.get(this, BhkPackedNiTriStripsShape::pInfo);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00ca5870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5870 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::Init` (engine map): with the thread block's `+0x2b4` word at 0x10,
/// takes 0xB0 bytes from the Havok allocator (`router + 0x10`, virtual slot `+4`), stores 0xB0 as
/// the 16-bit word at `+4` of the block, constructs the Havok shape in it (`00ca39f0(block,
/// argument)`), hands it to the shape (virtual slot `+0x8C`, `SethkObject`), releases the creation
/// reference (`00c905b0`) and runs `PostInit` (slot `+0xBC`) with `argument`.
pub fn bhk_packed_ni_tri_strips_shape_init(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    argument: u32,
) {
    let (word, saved) = enter_context(e);
    let allocator = havok_allocator(e);
    let block = e.vcall(allocator, 4, &args![0xb0u32]).u32();
    e.mem.set_u16(block + 4, 0xb0);
    let shape = e.call(HAVOK_CONSTRUCT, &args![block, argument]).u32();
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(HAVOK_RELEASE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![argument]);
    e.mem.set_u32(word, saved);
}

/// Writes `size` bytes at `data` through the stream's output object (`[stream + 0x250]`): cdecl
/// `(io, data, size, &size, 1)` at the function in slot `+0xC`; the byte count it returns is
/// added to the counter at `+4`.
fn write_stream_bytes(e: &mut Engine, stream: u32, data: u32, size: u32) {
    let io = e.mem.u32(stream + 0x250);
    let function = e.mem.u32(io + 0xc);
    let written = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), size);
        e.call(function, &args![io, data, size, cell, 1u32]).u32()
    });
    let total = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, total.wrapping_add(written));
}

// Translated from 00ca59d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca59d0 - the argument of the `KillCinfo` call (slot `+0xA4`) is a stack dword of which only the byte `CreateCinfo` wrote is initialised; the upper three bytes are not.
/// `bhkPackedNiTriStripsShape::SaveBinary` (engine map): takes the construction info (slot
/// `+0xB4`, which also gives the "created" byte), runs `bhkShape::SaveBinary` (`00c84d50`), writes
/// the Havok object's scale float (`+0xA0`, through a copy that quiets a signalling NaN; 4 bytes)
/// and the 16 bytes at `+0x90` through the stream's output object, links the shape data (the
/// stream's slot `+0x30` with the word at `+0x84`) and kills the construction info (slot
/// `+0xA4` with the created byte).
pub fn bhk_packed_ni_tri_strips_shape_save_binary(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    stream: Ptr,
) {
    let created = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]);
        e.mem.u8(created.addr())
    });
    let havok = e.mem.u32(this.addr() + 8);
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    let scale = x87_quiet_bits(e.mem.u32(havok + 0xa0));
    e.with_stack(4, |e, copy| {
        e.mem.set_u32(copy.addr(), scale);
        write_stream_bytes(e, stream.addr(), copy.addr(), 4);
    });
    write_stream_bytes(e, stream.addr(), havok + 0x90, 0x10);
    let data = e.mem.u32(havok + 0x84);
    e.vcall(stream.addr(), 0x30, &args![data]);
    e.vcall(this.addr(), 0xa4, &args![u32::from(created)]);
}

// Translated from 00ca5a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5a80 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::KillCinfo` (unnamed in the engine map): with the thread block's
/// `+0x2b4` word at 0x10 and when the low byte of `destroy` is not zero: destroys the construction
/// info if there is one (`00ca5620`) and frees its aligned block (the shift is in the byte before
/// it, `00aa4060` on the memory manager); `pInfo` is cleared.
pub fn fn_00ca5a80(e: &mut Engine, this: Ptr<BhkPackedNiTriStripsShape>, destroy: u32) {
    let (word, saved) = enter_context(e);
    if destroy as u8 != 0 {
        let cinfo = e.get(this, BhkPackedNiTriStripsShape::pInfo).addr();
        if cinfo != 0 {
            fn_00ca5620(e, Ptr::new(cinfo));
            let shift = u32::from(e.mem.u8(cinfo - 1));
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, cinfo - shift]);
        }
        e.set(this, BhkPackedNiTriStripsShape::pInfo, Ptr::NULL);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00ca5b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5b20 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPackedNiTriStripsShape::GetViewerStrings` (engine map): with the thread block's `+0x2b4`
/// word at 0x10, appends to `strings` the base class strings (`00cd2fd0`), the class name and
/// `"Subparts"` with the number of sub-parts (these two through the array's `SetAt`, `0096ae90`),
/// then for the first five sub-parts `"Part"` with the index, the strings the sub-part adds for
/// itself (`00c849b0` with the address of a copy of the first word of its record) and
/// `"MATERIAL"` with its material (the inlined form of `SetAt` for the `"Part"` and
/// `"MATERIAL"` strings).
pub fn bhk_packed_ni_tri_strips_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    strings: Ptr,
) {
    let (word, saved) = enter_context(e);
    let array = strings.addr();
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, array, text);
    let havok = havok_of(e, this.addr());
    let data = e.mem.u32(havok + 0x84);
    let total = u32::from(e.mem.u16(data + 0x10));
    let text = e
        .call(NI_GET_VIEWER_STRING_NUMBER, &args![NAME_SUBPARTS, total])
        .u32();
    push_viewer_string(e, array, text);
    for index in 0..total.min(5) {
        let text = e
            .call(NI_GET_VIEWER_STRING_NUMBER, &args![NAME_PART, index])
            .u32();
        set_viewer_string_inline(e, array, text);
        let data = e.mem.u32(havok + 0x84);
        let record = e.mem.u32(data + 0x24) + index * 12;
        let first = e.mem.u32(record);
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), first);
            e.call(PART_ADD_VIEWER_STRINGS, &args![local, strings]);
        });
        let data = e.mem.u32(havok + 0x84);
        let record = e.mem.u32(data + 0x24) + index * 12;
        let material = e.mem.u32(record + 8);
        let text = e
            .call(
                NI_GET_VIEWER_STRING_MATERIAL,
                &args![NAME_MATERIAL, material],
            )
            .u32();
        set_viewer_string_inline(e, array, text);
    }
    e.mem.set_u32(word, saved);
}

/// What `BuildDisplayGeometry` inlines to add one index to its local `NiTArray<unsigned short>`
/// (`array`: data `+4`, 16-bit capacity `+8`, size `+0xA`, count of non-zero elements `+0xC`, grow
/// step `+0xE`): sets the element at the old size, growing the array first (`00a5b790`) when it is
/// full, and keeps the count of non-zero elements. Only the low 16 bits of `value` are kept.
fn append_index(e: &mut Engine, array: u32, value: u32) {
    let index = u32::from(e.mem.u16(array + 0xa));
    if index >= u32::from(e.mem.u16(array + 8)) {
        let step = u32::from(e.mem.u16(array + 0xe));
        fn_00a5b790(e, Ptr::new(array), step + index);
    }
    let data = e.mem.u32(array + 4);
    let size = u32::from(e.mem.u16(array + 0xa));
    let count = e.mem.u16(array + 0xc);
    let value = value as u16;
    if index >= size {
        e.mem.set_u16(array + 0xa, (index + 1) as u16);
        if value != 0 {
            e.mem.set_u16(array + 0xc, count.wrapping_add(1));
        }
    } else {
        let old = e.mem.u16(data + index * 2);
        if value != 0 {
            if old == 0 {
                e.mem.set_u16(array + 0xc, count.wrapping_add(1));
            }
        } else if old != 0 {
            e.mem.set_u16(array + 0xc, count.wrapping_add(0xffff));
        }
    }
    e.mem.set_u16(data + index * 2, value);
}

/// The exclusive end of the vertices of the sub-part whose table record is `record`: the first
/// vertex index (16 bits) plus the dword at `+4` of the record.
fn part_vertex_limit(e: &Engine, record: u32, vertex_base: u32) -> u32 {
    (vertex_base & 0xffff).wrapping_add(e.mem.u32(record + 4))
}

/// The body of `BuildDisplayGeometry` between the setup and the release of the index array.
fn build_subparts(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    node: Ptr,
    colour: Ptr,
    shapes: u32,
    array: u32,
) {
    let subpart_count = u32::from(e.mem.u16(shapes + 0x10));
    let mut vertex_base = 0u32;
    let mut previous_triangles = 0u32;
    let mut triangle = 0u32;
    for subpart in 0..subpart_count {
        let record = e.mem.u32(shapes + 0x24) + subpart * 12;
        let mut run = 0u32;
        let mut first = 0u32;
        let mut second = 0u32;
        let mut third = 0u32;
        loop {
            let triangles = e.mem.u32(shapes + 0x14);
            let entry = triangles.wrapping_add((triangle & 0xffff) * 8);
            if run as u16 == 0 {
                first = u32::from(e.mem.u16(entry));
                second = u32::from(e.mem.u16(entry + 2));
                third = u32::from(e.mem.u16(entry + 4));
                if first >= part_vertex_limit(e, record, vertex_base) {
                    break;
                }
                for index in [first, second, third] {
                    append_index(e, array, index.wrapping_sub(vertex_base));
                }
                triangle = triangle.wrapping_add(1);
                run = 1;
            } else if second as u16 == e.mem.u16(entry) && third as u16 == e.mem.u16(entry + 2) {
                let next = u32::from(e.mem.u16(entry + 4));
                for index in [second, third, next] {
                    append_index(e, array, index.wrapping_sub(vertex_base));
                }
                first = second;
                second = third;
                third = next;
                run = run.wrapping_add(1);
                triangle = triangle.wrapping_add(1);
            } else {
                run = 0;
            }
            if (first & 0xffff) >= part_vertex_limit(e, record, vertex_base)
                || (triangle & 0xffff) >= e.mem.u32(shapes + 8)
            {
                break;
            }
        }
        let part = PartSource {
            shapes,
            record,
            array,
            vertex_base,
            triangles: triangle.wrapping_sub(previous_triangles),
        };
        build_subpart_geometry(e, this, node, colour, &part);
        previous_triangles = triangle & 0xffff;
        vertex_base = vertex_base.wrapping_add(u32::from(e.mem.u16(record + 4)));
    }
}

// Translated from 00ca5da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5da0 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `bhkPackedNiTriStripsShape::BuildDisplayGeometry` (engine map): with the thread block's
/// `+0x2b4` word at 0x10, walks the triangles of the shape data sub-part by sub-part with a local
/// `NiTArray<unsigned short>` of indices (vtable `0109b2d4`, grow step 1). Within a sub-part the
/// first triangle contributes its three indices (relative to the sub-part's first vertex, a
/// running 16-bit total of the vertex counts); a following triangle whose first two indices are
/// the previous triangle's last two continues the strip and contributes three indices again
/// (the previous second and third and its own third); any other triangle starts anew. A sub-part
/// ends at a triangle whose first index is past the sub-part's vertices or at the last triangle.
/// For each sub-part an `NiTriShapeData` (0x58 bytes, `00a7b690`) takes the vertices, multiplied
/// by the float at `011b1864` (`SetData`, `00a66b30`) and the indices (`00a7b700`), an
/// `NiTriShape` (0xC4 bytes, `00a74480`) is named `"bhkPackedNiTriStripsShape"`, gets the
/// absolute scale of the shape (`00ca5310`) at `+0x64`, an `NiMaterialProperty` (0x4C bytes,
/// `00a75650`) with the three words at `colour` at `+0x28` and a reference at `+0x44` is attached
/// (`00439410`) and the shape is attached to `node` (slot `+0xDC` with 1). The index array is
/// cleared after each sub-part and its data freed at the end (`00aa10f0`).
pub fn bhk_packed_ni_tri_strips_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    node: Ptr,
    colour: Ptr,
) {
    let (word, saved) = enter_context(e);
    let havok = havok_of(e, this.addr());
    let shapes = e.mem.u32(havok + 0x84);
    e.with_stack(0x10, |e, array| {
        e.mem.set_u32(array.addr(), INDEX_ARRAY_VTABLE);
        e.mem.set_u16(array.addr() + 0xe, 1);
        build_subparts(e, this, node, colour, shapes, array.addr());
        let data = e.mem.u32(array.addr() + 4);
        e.call(OPERATOR_DELETE, &args![data]);
    });
    e.mem.set_u32(word, saved);
}

/// What the display objects of one sub-part are built from: the shape data, the sub-part's table
/// record, the array of collected indices, the sub-part's first vertex and the number of
/// triangles taken for it (16 bits are used).
struct PartSource {
    shapes: u32,
    record: u32,
    array: u32,
    vertex_base: u32,
    triangles: u32,
}

/// Builds the display objects of one sub-part of `BuildDisplayGeometry` (see there).
fn build_subpart_geometry(
    e: &mut Engine,
    this: Ptr<BhkPackedNiTriStripsShape>,
    node: Ptr,
    colour: Ptr,
    part: &PartSource,
) {
    let PartSource {
        shapes,
        record,
        array,
        vertex_base,
        triangles,
    } = *part;
    let memory = e.call(OPERATOR_NEW, &args![0x58u32]).u32();
    let data = if memory == 0 {
        0
    } else {
        e.call(TRI_SHAPE_DATA_CONSTRUCT, &args![memory]).u32()
    };
    let vertex_count = u32::from(e.mem.u16(record + 4));
    let bytes = vertex_count.saturating_mul(12);
    let vertices = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    let scale = e.mem.f32(DISPLAY_SCALE);
    let first_vertex = vertex_base & 0xffff;
    let end = vertex_count + first_vertex;
    let source = e.mem.u32(shapes + 0x18);
    let mut index = first_vertex;
    while index < end {
        let from = source.wrapping_add(index * 12);
        let to = vertices.wrapping_add(index.wrapping_sub(first_vertex).wrapping_mul(12));
        for lane in 0..3 {
            let value = x87_quiet(e.mem.f32(from + 4 * lane));
            e.mem.set_f32(to + 4 * lane, value * scale);
        }
        index = (index + 1) & 0xffff;
    }
    e.call(
        SET_DATA,
        &args![data, vertex_count, vertices, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let triangle_count = triangles & 0xffff;
    let bytes = (triangle_count * 3).saturating_mul(2);
    let indices = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    let used = u32::from(e.mem.u16(array + 0xa));
    let collected = e.mem.u32(array + 4);
    e.call(MEMCPY, &args![indices, collected, used * 2]);
    e.call(SET_TRIANGLES, &args![data, triangle_count, indices]);
    let memory = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
    let display = if memory == 0 {
        0
    } else {
        e.call(TRI_SHAPE_CONSTRUCT, &args![memory, data]).u32()
    };
    let handle = e.call(FIXED_STRING_CREATE, &args![DISPLAY_NAME]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(SET_NAME, &args![display, slot]);
    });
    release_fixed_string(e, handle);
    let radius = fn_00ca5310(e, Ptr::new(this.addr()));
    e.mem.set_f32(display + 0x64, radius.abs());
    for slot in 0..used {
        e.mem.set_u16(collected + slot * 2, 0);
    }
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    let memory = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let material = if memory == 0 {
        0
    } else {
        e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![memory]).u32()
    };
    let (red, green, blue) = (
        e.mem.u32(colour.addr()),
        e.mem.u32(colour.addr() + 4),
        e.mem.u32(colour.addr() + 8),
    );
    e.mem.set_u32(material + 0x28, red);
    e.mem.set_u32(material + 0x2c, green);
    let references = e.mem.u32(material + 0x44);
    e.mem.set_u32(material + 0x44, references.wrapping_add(1));
    e.mem.set_u32(material + 0x30, blue);
    e.call(ATTACH_PROPERTY, &args![display, material]);
    e.vcall(node.addr(), 0xdc, &args![display, 1u32]);
}

// Translated from 00ccc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed here (the engine map labels the address `bhkExtendedMeshShape::LinkObject (check)`, and
/// Ghidra gives it that class; unconfirmed which class owns the code, it belongs to this unit's
/// source file): takes the construction info (virtual slot `+0xB4`), looks the linked object up
/// in the stream (`00a63430`) and stores it as the `NiPointer` at `+4` of the construction info
/// (the old object is released, the new one gets a reference), then runs `bhkShape::LinkObject`
/// (`00c84cc0`, `(this, stream)`).
pub fn fn_00ccc3b0(e: &mut Engine, this: Ptr<BhkPackedNiTriStripsShape>, stream: Ptr) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let old = e.mem.u32(cinfo + 4);
    if old != object {
        release_reference(e, old);
        e.mem.set_u32(cinfo + 4, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
    }
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00ca5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ca5290`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_packed_ni_tri_strips_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_5290;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ca5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPackedNiTriStripsShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_82dc.
pub fn bhk_packed_ni_tri_strips_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_82dc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5b790, fn_00a5b790(Ptr, u32)),
        entry!(
            0x00ca5290,
            bhk_packed_ni_tri_strips_shape_destructor(Ptr<BhkPackedNiTriStripsShape>)
        ),
        entry!(0x00ca5300, bhk_packed_ni_tri_strips_shape_get_r_t_t_i() -> u32),
        entry!(0x00ca5310, fn_00ca5310(Ptr) -> f32),
        entry!(
            0x00ca5340,
            bhk_packed_ni_tri_strips_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ca5360,
            bhk_packed_ni_tri_strips_shape_copy_cinfo(Ptr<BhkPackedNiTriStripsShape>, Ptr)
        ),
        entry!(0x00ca53b0, fn_00ca53b0() -> Ptr<BhkPackedNiTriStripsShape>),
        entry!(
            0x00ca5460,
            bhk_packed_ni_tri_strips_shape_copy_members(Ptr<BhkPackedNiTriStripsShape>, Ptr, Ptr)
        ),
        entry!(
            0x00ca54c0,
            bhk_packed_ni_tri_strips_shape_register_streamables(
                Ptr<BhkPackedNiTriStripsShape>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00ca54f0,
            bhk_packed_ni_tri_strips_shape_is_equal(
                Ptr<BhkPackedNiTriStripsShape>,
                Ptr<BhkPackedNiTriStripsShape>,
            ) -> bool
        ),
        entry!(
            0x00ca5540,
            bhk_packed_ni_tri_strips_shape_q_sub_material(
                Ptr<BhkPackedNiTriStripsShape>,
                u32,
            ) -> u32
        ),
        entry!(0x00ca5580, fn_00ca5580(Ptr) -> Ptr),
        entry!(0x00ca5620, fn_00ca5620(Ptr)),
        entry!(
            0x00ca56b0,
            bhk_packed_ni_tri_strips_shape_create_clone(
                Ptr<BhkPackedNiTriStripsShape>,
                Ptr,
            )
                -> Ptr<BhkPackedNiTriStripsShape>
        ),
        entry!(
            0x00ca57a0,
            bhk_packed_ni_tri_strips_shape_create_cinfo(Ptr<BhkPackedNiTriStripsShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00ca5870,
            bhk_packed_ni_tri_strips_shape_init(Ptr<BhkPackedNiTriStripsShape>, u32)
        ),
        entry!(
            0x00ca59d0,
            bhk_packed_ni_tri_strips_shape_save_binary(Ptr<BhkPackedNiTriStripsShape>, Ptr)
        ),
        entry!(0x00ca5a80, fn_00ca5a80(Ptr<BhkPackedNiTriStripsShape>, u32)),
        entry!(
            0x00ca5b20,
            bhk_packed_ni_tri_strips_shape_get_viewer_strings(Ptr<BhkPackedNiTriStripsShape>, Ptr)
        ),
        entry!(
            0x00ca5da0,
            bhk_packed_ni_tri_strips_shape_build_display_geometry(
                Ptr<BhkPackedNiTriStripsShape>,
                Ptr,
                Ptr,
            )
        ),
        entry!(0x00ccc3b0, fn_00ccc3b0(Ptr<BhkPackedNiTriStripsShape>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    /// The router (`00c85750`) whose `+0x10` word is the Havok allocator.
    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

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

    /// An engine with this unit's globals mapped, the interlocked imports working on memory and
    /// the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(SERIALIZABLE_COUNT, 4);
        e.map(SHAPE_COUNT, 4);
        e.map(OBJECT_COUNT, 0x10);
        e.map(UNIT_SCALE, 4);
        e.map(DISPLAY_SCALE, 4);
        e.map(0x0109_b220, 4);
        e.mem.set_u32(0x0109_b220, 0x9000);
        e.mem.set_f32(UNIT_SCALE, 1.0);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.register(0x00fd_f1c4, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
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

    fn shape(e: &mut Engine) -> Ptr<BhkPackedNiTriStripsShape> {
        e.new_object::<BhkPackedNiTriStripsShape>()
    }

    /// A shape whose vtable has the given slots.
    fn shape_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<BhkPackedNiTriStripsShape> {
        Ptr::new(object_with_slots(e, 0x14, slots))
    }

    /// `size` zeroed bytes starting at a 16-byte aligned address.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// A Havok object (0xb0 bytes) whose shape data (`+0x84`) is a block of 0x40 bytes with the
    /// given sub-part count; its table (`+0x24`) holds `parts` as `(word, vertices, material)`.
    fn havok_with_parts(e: &mut Engine, parts: &[(u32, u32, u32)]) -> (u32, u32) {
        let havok = e.mem.alloc(0xb0);
        let data = e.mem.alloc(0x40);
        let table = e.mem.alloc(12 * parts.len() as u32 + 12);
        for (index, (word, vertices, material)) in parts.iter().enumerate() {
            let record = table + 12 * index as u32;
            e.mem.set_u32(record, *word);
            e.mem.set_u32(record + 4, *vertices);
            e.mem.set_u32(record + 8, *material);
        }
        e.mem.set_u16(data + 0x10, parts.len() as u16);
        e.mem.set_u32(data + 0x24, table);
        e.mem.set_u32(havok + 0x84, data);
        (havok, data)
    }

    /// A 16-bit element array with a capacity and the given elements (grow step 1).
    fn ushort_array(e: &mut Engine, capacity: u16, elements: &[u16]) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(2 * u32::from(capacity) + 2);
        for (index, element) in elements.iter().enumerate() {
            e.mem.set_u16(data + 2 * index as u32, *element);
        }
        e.mem.set_u32(array + 4, data);
        e.mem.set_u16(array + 8, capacity);
        e.mem.set_u16(array + 0xa, elements.len() as u16);
        let count = elements.iter().filter(|element| **element != 0).count();
        e.mem.set_u16(array + 0xc, count as u16);
        e.mem.set_u16(array + 0xe, 1);
        array
    }

    /// The first `count` 16-bit elements at `data`.
    fn ushort_elements_at(e: &Engine, data: u32, count: u32) -> Vec<u16> {
        (0..count)
            .map(|index| e.mem.u16(data + 2 * index))
            .collect()
    }

    /// The first `count` elements of the data of a 16-bit element array.
    fn ushort_elements(e: &Engine, array: u32, count: u32) -> Vec<u16> {
        ushort_elements_at(e, e.mem.u32(array + 4), count)
    }

    /// Doubles for what the resize calls: the allocator fills its block with 0xffff words and
    /// `operator delete` does nothing.
    fn install_resize_doubles(e: &mut Engine) {
        e.register(ARRAY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(2) {
                e.mem.set_u16(block + offset, 0xffff);
            }
            ret(block)
        });
        e.register(OPERATOR_DELETE, |_, _| ret(0));
    }

    #[test]
    fn resizing_up_copies_the_elements_and_zeroes_the_rest() {
        let mut e = engine();
        install_resize_doubles(&mut e);
        let array = ushort_array(&mut e, 2, &[7, 0]);
        let old = e.mem.u32(array + 4);

        e.call(0x00a5b790, &args![array, 5u32]);

        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![10]]);
        assert_ne!(e.mem.u32(array + 4), old);
        assert_eq!(ushort_elements(&e, array, 5), [7, 0, 0, 0, 0]);
        assert_eq!(e.mem.u16(array + 8), 5);
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![old]]);
    }

    #[test]
    fn resizing_down_zeroes_and_counts_out_the_dropped_elements() {
        let mut e = engine();
        install_resize_doubles(&mut e);
        let array = ushort_array(&mut e, 6, &[1, 0, 3, 4]);
        let old = e.mem.u32(array + 4);

        e.call(0x00a5b790, &args![array, 2u32]);

        // The old block lost its dropped elements, the new one has the first two.
        assert_eq!(ushort_elements_at(&e, old, 4), [1, 0, 0, 0]);
        assert_eq!(ushort_elements(&e, array, 2), [1, 0]);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![4]]);
        assert_eq!(e.mem.u16(array + 8), 2);
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![old]]);
    }

    #[test]
    fn resizing_to_the_capacity_does_nothing() {
        let mut e = engine();
        install_resize_doubles(&mut e);
        let array = ushort_array(&mut e, 4, &[1, 2]);
        e.call(0x00a5b790, &args![array, 4u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u16(array + 0xa), 2);
    }

    #[test]
    fn resizing_to_zero_drops_the_data() {
        let mut e = engine();
        install_resize_doubles(&mut e);
        let array = ushort_array(&mut e, 4, &[5, 6]);
        let old = e.mem.u32(array + 4);

        e.call(0x00a5b790, &args![array, 0u32]);

        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 8), 0);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        assert!(calls_to(&e, ARRAY_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![old]]);
    }

    #[test]
    fn destructor_body_counts_the_object_out() {
        let mut e = engine();
        e.register(SERIALIZABLE_KILL, |_, _| ret(0));
        e.register(BASE_DESTRUCTOR_BODY, |_, _| ret(0));
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = shape(&mut e);
        e.call(0x00ca5290, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY), vec![vec![this.addr()]]);
    }

    #[test]
    fn scale_is_read_from_the_havok_object_or_one() {
        let mut e = engine();
        let this = shape(&mut e);
        assert_eq!(e.call(0x00ca5310, &args![this]).f32(), 1.0);
        assert_eq!(e.call(0x00ca5310, &args![0u32]).f32(), 1.0);
        let havok = e.mem.alloc(0xb0);
        e.mem.set_f32(havok + 0x90, 2.5);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        assert_eq!(e.call(0x00ca5310, &args![this]).f32(), 2.5);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(havok + 0x90, 0x7f80_0001);
        assert_eq!(
            e.call(0x00ca5310, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn copy_cinfo_asks_the_havok_object_or_sets_the_default_scale() {
        let mut e = engine();
        e.register(SHAPE_COPY_CINFO, |_, _| ret(0));
        e.register(HAVOK_COPY_CINFO, |_, _| ret(0));
        e.mem.set_f32(UNIT_SCALE, 1.5);
        let destination = aligned_block(&mut e, 0x40);
        let this = shape(&mut e);
        let havok = e.mem.alloc(0xb0);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));

        e.call(0x00ca5360, &args![this, destination]);

        assert_eq!(
            calls_to(&e, SHAPE_COPY_CINFO),
            vec![vec![this.addr(), destination]]
        );
        assert_eq!(
            calls_to(&e, HAVOK_COPY_CINFO),
            vec![vec![havok, destination]]
        );
        assert_eq!(e.mem.f32(destination + 0x10), 0.0);

        // Without a Havok object, and without a shape, the defaults are stored.
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::NULL);
        e.mem.set_f32(destination + 8, 9.0);
        e.call(0x00ca5360, &args![this, destination]);
        assert_eq!(calls_to(&e, HAVOK_COPY_CINFO).len(), 1);
        assert_eq!(e.mem.f32(destination + 8), 0.0);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(destination + 0x10 + 4 * lane), 1.5);
        }
        e.mem.set_f32(destination + 0x1c, 7.0);
        e.call(0x00ca5360, &args![0u32, destination]);
        assert_eq!(calls_to(&e, HAVOK_COPY_CINFO).len(), 1);
        assert_eq!(e.mem.f32(destination + 0x1c), 1.5);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO).len(), 3);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn copy_cinfo_needs_an_aligned_vector() {
        let mut e = engine();
        e.register(SHAPE_COPY_CINFO, |_, _| ret(0));
        let destination = aligned_block(&mut e, 0x40) + 8;
        let this = shape(&mut e);
        e.call(0x00ca5360, &args![this, destination]);
    }

    #[test]
    fn constructor_function_builds_a_shape_or_returns_null() {
        let mut e = engine();
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| ret(0));
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 0xc, 0x1111);
        e.mem.set_u32(block + 0x10, 0x2222);
        e.mem.set_u32(SERIALIZABLE_COUNT, 1);
        e.mem.set_u32(SHAPE_COUNT, 2);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [0x14]);
            ret(block)
        });

        assert_eq!(e.call(0x00ca53b0, &args![]).u32(), block);

        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(e.mem.u32(SHAPE_COUNT), 3);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR), vec![vec![block]]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ca53b0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn copy_members_scales_the_four_lanes_of_the_cinfo_vector() {
        let mut e = engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| ret(0));
        let cinfo = aligned_block(&mut e, 0x30);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(cinfo + 0x10 + 4 * lane as u32, value);
        }
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 0.5);

        e.call(0x00ca5460, &args![this, 0x4000u32, process]);

        for (lane, expected) in [0.5f32, 1.0, 1.5, 2.0].into_iter().enumerate() {
            assert_eq!(e.mem.f32(cinfo + 0x10 + 4 * lane as u32), expected);
        }
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this.addr(), 0x4000, process]]
        );
    }

    #[test]
    fn register_streamables_registers_the_shape_data_then_the_base() {
        let mut e = engine();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| ret(1));
        e.register(0x7000_0054, |_, _| ret(0));
        let data = object_with_slots(&mut e, 0x40, &[(0x54, 0x7000_0054)]);
        let havok = e.mem.alloc(0xb0);
        e.mem.set_u32(havok + 0x84, data);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));

        assert!(e.call(0x00ca54c0, &args![this, 0x6000u32]).bool());

        assert_eq!(calls_to(&e, 0x7000_0054), vec![vec![data, 0x6000]]);
        assert_eq!(
            calls_to(&e, OBJECT_REGISTER_STREAMABLES),
            vec![vec![this.addr(), 0x6000]]
        );
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| ret(0));
        assert!(!e.call(0x00ca54c0, &args![this, 0x6000u32]).bool());
    }

    #[test]
    fn is_equal_compares_the_shape_data_after_the_base() {
        let mut e = engine();
        e.register(SHAPE_IS_EQUAL, |_, _| ret(1));
        let (first_havok, _) = havok_with_parts(&mut e, &[]);
        let (second_havok, _) = havok_with_parts(&mut e, &[]);
        let this = shape(&mut e);
        let other = shape(&mut e);
        e.set(
            this,
            BhkPackedNiTriStripsShape::phkObject,
            Ptr::new(first_havok),
        );
        e.set(
            other,
            BhkPackedNiTriStripsShape::phkObject,
            Ptr::new(second_havok),
        );
        assert!(!e.call(0x00ca54f0, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, SHAPE_IS_EQUAL),
            vec![vec![this.addr(), other.addr()]]
        );

        let shared = e.mem.u32(first_havok + 0x84);
        e.mem.set_u32(second_havok + 0x84, shared);
        assert!(e.call(0x00ca54f0, &args![this, other]).bool());

        // A failing base comparison decides.
        e.register(SHAPE_IS_EQUAL, |_, _| ret(0));
        assert!(!e.call(0x00ca54f0, &args![this, other]).bool());
    }

    #[test]
    fn sub_material_comes_from_the_part_table_or_from_the_shape() {
        let mut e = engine();
        let (havok, _) = havok_with_parts(&mut e, &[(0, 4, 0x21), (0, 4, 0x22)]);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        e.set(this, BhkPackedNiTriStripsShape::eMaterial, 0x1234_5645);
        let material = |e: &mut Engine, index: u32| e.call(0x00ca5540, &args![this, index]).u32();
        assert_eq!(material(&mut e, 0), 0x21);
        assert_eq!(material(&mut e, 0x0100_0abc), 0x22);
        // Past the count and -1 use the shape's own material.
        assert_eq!(material(&mut e, 0x0200_0000), 5);
        assert_eq!(material(&mut e, 0xffff_ffff), 5);
    }

    #[test]
    fn cinfo_initialiser_clears_the_words_and_sets_the_scale_vector() {
        let mut e = engine();
        let block = aligned_block(&mut e, 0x30);
        for offset in [0u32, 4, 8, 0x10, 0x14, 0x18, 0x1c] {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        let result = e.call(0x00ca5580, &args![block]).u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.f32(block + 8), 0.0);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(block + 0x10 + 4 * lane), 1.0);
        }
    }

    #[test]
    fn cinfo_destructor_releases_the_pointer_once_and_clears_it() {
        let mut e = engine();
        e.register(0x7000_0004, |_, _| ret(0));
        let object = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let block = e.mem.alloc(0x30);
        // The last reference: the object's delete runs.
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(block + 4, object);
        e.call(0x00ca5620, &args![block]);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![object]]);
        // Another holder keeps it alive.
        e.mem.set_u32(object + 4, 2);
        e.mem.set_u32(block + 4, object);
        e.call(0x00ca5620, &args![block]);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(calls_to(&e, 0x7000_0004).len(), 1);
        // Nothing to release.
        e.call(0x00ca5620, &args![block]);
        assert_eq!(calls_to(&e, 0x00fd_f1c4).len(), 2);
    }

    #[test]
    fn create_clone_copies_members_into_a_new_shape_under_the_clone_lock() {
        let mut e = engine();
        e.register(CLONE_LOCK, |_, _| ret(0));
        e.register(CLONE_UNLOCK, |_, _| ret(0));
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| ret(0));
        let block = e.mem.alloc(0x14);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let seen = Rc::new(RefCell::new(0));
        let during = seen.clone();
        e.register_double(0x7000_00c4, move |e, _| {
            *during.borrow_mut() = e.mem.u32(word);
            ret(0)
        });
        let this = shape_with_slots(&mut e, &[(0xc4, 0x7000_00c4)]);

        let clone = e.call(0x00ca56b0, &args![this, 0x6000u32]).u32();

        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(*seen.borrow(), 0x10);
        assert_eq!(e.mem.u32(word), 0x33);
        assert_eq!(
            calls_to(&e, 0x7000_00c4),
            vec![vec![this.addr(), block, 0x6000]]
        );
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00ca56b0,
                CLONE_LOCK,
                OPERATOR_NEW,
                REF_OBJECT_CONSTRUCTOR,
                0x7000_00c4,
                CLONE_UNLOCK
            ]
        );

        // A failed allocation still runs CopyMembers, with a null shape.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ca56b0, &args![this, 0x6000u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x7000_00c4)[1], [this.addr(), 0, 0x6000]);
    }

    #[test]
    fn create_cinfo_makes_the_info_once_and_fills_it_from_the_shape() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let raw = e.mem.alloc(0x60) + 5;
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, [MEMORY_MANAGER, 0x30]);
            ret(raw)
        });
        e.register(SHAPE_COPY_CINFO, |_, _| ret(0));
        e.register(HAVOK_COPY_CINFO, |_, _| ret(0));
        let this = shape(&mut e);
        let havok = e.mem.alloc(0xb0);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 0xee);

        let info = e.call(0x00ca57a0, &args![this, created]).u32();

        let shift = 0x10 - (raw & 0xf);
        let block = raw + shift;
        assert_eq!(info, block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u8(block - 1), shift as u8);
        assert_eq!(e.get(this, BhkPackedNiTriStripsShape::pInfo).addr(), block);
        assert_eq!(e.mem.f32(block + 0x10), 1.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_CINFO),
            vec![vec![this.addr(), block]]
        );
        assert_eq!(calls_to(&e, HAVOK_COPY_CINFO), vec![vec![havok, block]]);
        assert_eq!(e.mem.u32(word), 0x33);

        // The second call finds the info and allocates nothing.
        let again = e.call(0x00ca57a0, &args![this, created]).u32();
        assert_eq!(again, block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
    }

    #[test]
    fn create_cinfo_without_a_havok_object_only_builds_the_block() {
        let mut e = engine();
        let raw = e.mem.alloc(0x60);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(raw));
        e.register(SHAPE_COPY_CINFO, |_, _| ret(0));
        let this = shape(&mut e);
        let created = e.mem.alloc(4);
        let info = e.call(0x00ca57a0, &args![this, created]).u32();
        assert_ne!(info, 0);
        assert_eq!(e.mem.u8(created), 1);
        assert!(calls_to(&e, SHAPE_COPY_CINFO).is_empty());
        assert_eq!(e.get(this, BhkPackedNiTriStripsShape::pInfo).addr(), info);
    }

    #[test]
    fn init_builds_the_havok_shape_in_a_block_of_the_havok_allocator() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let block = e.mem.alloc(0xb0);
        e.register_double(0x7000_0004, move |_, a| {
            assert_eq!(a[1..], [0xb0]);
            ret(block)
        });
        let allocator = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register(HAVOK_CONSTRUCT, |_, _| ret(0x7777));
        e.register(0x7000_008c, |_, _| ret(0));
        e.register(HAVOK_RELEASE, |_, _| ret(0));
        let seen = Rc::new(RefCell::new(0));
        let during = seen.clone();
        e.register_double(0x7000_00bc, move |e, _| {
            *during.borrow_mut() = e.mem.u32(word);
            ret(0)
        });
        let this = shape_with_slots(&mut e, &[(0x8c, 0x7000_008c), (0xbc, 0x7000_00bc)]);

        e.call(0x00ca5870, &args![this, 0x4242u32]);

        assert_eq!(e.mem.u16(block + 4), 0xb0);
        assert_eq!(calls_to(&e, HAVOK_CONSTRUCT), vec![vec![block, 0x4242]]);
        assert_eq!(calls_to(&e, 0x7000_008c), vec![vec![this.addr(), 0x7777]]);
        assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![0x7777]]);
        assert_eq!(calls_to(&e, 0x7000_00bc), vec![vec![this.addr(), 0x4242]]);
        assert_eq!(*seen.borrow(), 0x10);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn save_binary_writes_the_scale_and_the_vector_then_links_the_shape_data() {
        let mut e = engine();
        e.register(SHAPE_SAVE_BINARY, |_, _| ret(0));
        e.register(0x7000_00b4, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(0)
        });
        e.register(0x7000_00a4, |_, _| ret(0));
        e.register(0x7000_0030, |_, _| ret(0));
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(0x7000_000c, move |e, a| {
            sink.borrow_mut()
                .push((a[2], e.mem.u32(a[1]), e.mem.u32(a[3])));
            ret(a[2])
        });
        let stream = object_with_slots(&mut e, 0x260, &[(0x30, 0x7000_0030)]);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(io + 0xc, 0x7000_000c);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(stream + 0x250, io);
        let (havok, data) = havok_with_parts(&mut e, &[]);
        e.mem.set_u32(havok + 0xa0, 0x7f80_0001);
        e.mem.set_u32(havok + 0x90, 0x1234);
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4), (0xa4, 0x7000_00a4)]);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));

        e.call(0x00ca59d0, &args![this, stream]);

        // (size, first word, the size cell) of each write; the scale is stored quiet.
        assert_eq!(
            *written.borrow(),
            [(4, 0x7fc0_0001, 4), (0x10, 0x1234, 0x10)]
        );
        assert_eq!(e.mem.u32(io + 4), 10 + 4 + 0x10);
        assert_eq!(
            calls_to(&e, SHAPE_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(calls_to(&e, 0x7000_0030), vec![vec![stream, data]]);
        assert_eq!(calls_to(&e, 0x7000_00a4), vec![vec![this.addr(), 1]]);
    }

    #[test]
    fn kill_cinfo_frees_the_info_only_when_asked() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_FREE, |_, _| ret(0));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let block = e.mem.alloc(0x60);
        let cinfo = block + 9;
        e.mem.set_u8(cinfo - 1, 9);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::pInfo, Ptr::new(cinfo));

        // The low byte decides.
        e.call(0x00ca5a80, &args![this, 0x100u32]);
        assert_eq!(e.get(this, BhkPackedNiTriStripsShape::pInfo).addr(), cinfo);
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());

        e.call(0x00ca5a80, &args![this, 1u32]);
        assert!(e.get(this, BhkPackedNiTriStripsShape::pInfo).is_null());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(e.mem.u32(word), 0x33);

        // Nothing to free: only the pointer is cleared.
        e.call(0x00ca5a80, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_FREE).len(), 1);
    }

    /// Doubles for the viewer-string array: `SetAt` stores the text at the index and bumps the
    /// size and count the way the real array does.
    fn install_string_array_doubles(e: &mut Engine) {
        e.register(STRINGS_GROW, |_, _| ret(0));
        e.register(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            let data = e.mem.u32(a[0] + 4);
            e.mem.set_u32(data + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            let count = e.mem.u16(a[0] + 0xc);
            e.mem.set_u16(a[0] + 0xc, count + u16::from(text != 0));
            ret(0)
        });
    }

    fn strings_fixture(e: &mut Engine, capacity: u16) -> u32 {
        let strings = e.mem.alloc(0x10);
        let data = e.mem.alloc(4 * u32::from(capacity) + 4);
        e.mem.set_u32(strings + 4, data);
        e.mem.set_u16(strings + 8, capacity);
        e.mem.set_u16(strings + 0xe, 4);
        strings
    }

    fn string_data(e: &Engine, strings: u32) -> Vec<u32> {
        let data = e.mem.u32(strings + 4);
        (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn viewer_strings_list_the_class_the_count_and_at_most_five_parts() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        install_string_array_doubles(&mut e);
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| ret(0));
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(NI_GET_VIEWER_STRING_NUMBER, |_, a| ret(a[0] + a[1]));
        e.register(NI_GET_VIEWER_STRING_MATERIAL, |_, a| {
            ret(a[0] + a[1] + 0x1000)
        });
        let part_words = Rc::new(RefCell::new(Vec::new()));
        let seen_words = part_words.clone();
        e.register_double(PART_ADD_VIEWER_STRINGS, move |e, a| {
            seen_words.borrow_mut().push(e.mem.u32(a[0]));
            ret(0)
        });
        let parts: Vec<(u32, u32, u32)> = (0..7).map(|index| (0xa0 + index, 3, index)).collect();
        let (havok, _) = havok_with_parts(&mut e, &parts);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        let strings = strings_fixture(&mut e, 40);

        e.call(0x00ca5b20, &args![this, strings]);

        let mut expected = vec![0xa000, NAME_SUBPARTS + 7];
        for index in 0..5u32 {
            expected.push(NAME_PART + index);
            expected.push(NAME_MATERIAL + index + 0x1000);
        }
        assert_eq!(string_data(&e, strings), expected);
        assert_eq!(e.mem.u16(strings + 0xc), 12);
        // The part strings get the address of a copy of the record's first word and the array.
        assert_eq!(*part_words.borrow(), [0xa0, 0xa1, 0xa2, 0xa3, 0xa4]);
        let arrays: Vec<u32> = calls_to(&e, PART_ADD_VIEWER_STRINGS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(arrays, vec![strings; 5]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        assert_eq!(
            calls_to(&e, SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn viewer_strings_of_a_shape_without_parts_stop_after_the_count() {
        let mut e = engine();
        install_string_array_doubles(&mut e);
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| ret(0));
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(NI_GET_VIEWER_STRING_NUMBER, |_, a| ret(a[0] + a[1]));
        let (havok, _) = havok_with_parts(&mut e, &[]);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        let strings = strings_fixture(&mut e, 8);
        e.call(0x00ca5b20, &args![this, strings]);
        assert_eq!(string_data(&e, strings), [0xa000, NAME_SUBPARTS]);
        assert!(calls_to(&e, PART_ADD_VIEWER_STRINGS).is_empty());
    }

    /// A mapped fixed-string block: the reference count at +0, the handle at +8.
    const STRING_BLOCK: u32 = 0x7800_0000;
    const STRING_HANDLE: u32 = STRING_BLOCK + 8;

    /// The three vertex indices of each of the scene's triangles; the vertices are
    /// `(n, 10 + n, 20)`.
    const TRIANGLES: [(u16, u16, u16); 3] = [(0, 1, 2), (1, 2, 3), (4, 5, 6)];

    /// A shape whose Havok object has two parts (4 and 3 vertices, scale -3.0) and the triangles
    /// of `TRIANGLES`: the second triangle continues the first, the third belongs to the second
    /// part. Returns the shape and a node with a slot `+0xDC`.
    fn display_scene(e: &mut Engine) -> (Ptr<BhkPackedNiTriStripsShape>, u32) {
        e.map(STRING_BLOCK, 0x20);
        e.mem.set_u32(STRING_BLOCK, 5);
        e.mem.set_f32(DISPLAY_SCALE, 0.5);
        let (havok, data) = havok_with_parts(e, &[(0, 4, 0), (0, 3, 0)]);
        e.mem.set_f32(havok + 0x90, -3.0);
        let triangle_block = e.mem.alloc(8 * TRIANGLES.len() as u32);
        for (index, (a, b, c)) in TRIANGLES.iter().enumerate() {
            let record = triangle_block + 8 * index as u32;
            e.mem.set_u16(record, *a);
            e.mem.set_u16(record + 2, *b);
            e.mem.set_u16(record + 4, *c);
        }
        let vertex_block = e.mem.alloc(12 * 7);
        for n in 0..7u32 {
            e.mem.set_f32(vertex_block + 12 * n, n as f32);
            e.mem.set_f32(vertex_block + 12 * n + 4, 10.0 + n as f32);
            e.mem.set_f32(vertex_block + 12 * n + 8, 20.0);
        }
        e.mem.set_u32(data + 8, TRIANGLES.len() as u32);
        e.mem.set_u32(data + 0x14, triangle_block);
        e.mem.set_u32(data + 0x18, vertex_block);
        let this = shape(e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        let node = object_with_slots(e, 0x40, &[(0xdc, 0x7000_00dc)]);
        (this, node)
    }

    /// Doubles for the objects `BuildDisplayGeometry` makes: plain memory blocks for the
    /// allocations, constructors that answer their block.
    fn install_display_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| ret(e.mem.alloc(a[1])));
        e.register(TRI_SHAPE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TRI_SHAPE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SET_DATA, |_, _| ret(0));
        e.register(SET_TRIANGLES, |_, _| ret(0));
        e.register(FIXED_STRING_CREATE, |_, _| ret(STRING_HANDLE));
        e.register(SET_NAME, |_, _| ret(0));
        e.register(ATTACH_PROPERTY, |_, _| ret(0));
        e.register(0x7000_00dc, |_, _| ret(0));
    }

    /// The indices `SET_TRIANGLES` was given: `(data, count, indices)`.
    fn triangle_indices(e: &Engine, call: &[u32]) -> Vec<u16> {
        (0..call[1] * 3)
            .map(|n| e.mem.u16(call[2] + 2 * n))
            .collect()
    }

    /// The vertices `SET_DATA` was given: `(data, count, vertices, ...)`.
    fn data_vertices(e: &Engine, call: &[u32]) -> Vec<[f32; 3]> {
        (0..call[1])
            .map(|n| [0, 4, 8].map(|offset| e.mem.f32(call[2] + 12 * n + offset)))
            .collect()
    }

    #[test]
    fn display_geometry_builds_a_shape_for_each_part_from_runs_of_triangles() {
        let mut e = engine();
        install_display_doubles(&mut e);
        let allocated = Rc::new(RefCell::new(Vec::new()));
        let sink = allocated.clone();
        e.register_double(ARRAY_ALLOCATE, move |e, a| {
            let block = e.mem.alloc(a[0]);
            sink.borrow_mut().push(block);
            ret(block)
        });
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let (this, node) = display_scene(&mut e);
        let colour = e.mem.alloc(0xc);
        for (index, value) in [0x3f00_0000u32, 0x3f80_0000, 0x4000_0000]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(colour + 4 * index as u32, value);
        }

        e.call(0x00ca5da0, &args![this, node, colour]);

        // The first part has the vertices 0..4 and the two triangles that continue each other
        // (the second one repeats the last two indices and adds its own); the second part
        // starts at vertex 4, so its indices are relative to it.
        let set_data = calls_to(&e, SET_DATA);
        assert_eq!(set_data.len(), 2);
        assert_eq!(set_data[0][1], 4);
        assert_eq!(set_data[0][3..], [0u32; 5]);
        assert_eq!(
            data_vertices(&e, &set_data[0]),
            [
                [0.0, 5.0, 10.0],
                [0.5, 5.5, 10.0],
                [1.0, 6.0, 10.0],
                [1.5, 6.5, 10.0]
            ]
        );
        assert_eq!(
            data_vertices(&e, &set_data[1]),
            [[2.0, 7.0, 10.0], [2.5, 7.5, 10.0], [3.0, 8.0, 10.0]]
        );
        let set_triangles = calls_to(&e, SET_TRIANGLES);
        assert_eq!(set_triangles.len(), 2);
        assert_eq!(set_triangles[0][..2], [set_data[0][0], 2]);
        assert_eq!(triangle_indices(&e, &set_triangles[0]), [0, 1, 2, 1, 2, 3]);
        assert_eq!(set_triangles[1][..2], [set_data[1][0], 1]);
        assert_eq!(triangle_indices(&e, &set_triangles[1]), [0, 1, 2]);

        // Each part gets a named shape with the absolute scale and a coloured material,
        // attached to the node.
        let shapes: Vec<u32> = calls_to(&e, TRI_SHAPE_CONSTRUCT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(shapes.len(), 2);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_CONSTRUCT)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            [set_data[0][0], set_data[1][0]]
        );
        assert_eq!(
            calls_to(&e, FIXED_STRING_CREATE),
            vec![vec![DISPLAY_NAME]; 2]
        );
        assert_eq!(
            calls_to(&e, SET_NAME)
                .iter()
                .map(|c| c[0])
                .collect::<Vec<_>>(),
            shapes
        );
        assert_eq!(calls_to(&e, 0x00fd_f1c4), vec![vec![STRING_BLOCK]; 2]);
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 2);
        for (index, shape_address) in shapes.iter().enumerate() {
            assert_eq!(e.mem.f32(shape_address + 0x64), 3.0);
            assert_eq!(attached[index][0], *shape_address);
            let material = attached[index][1];
            assert_eq!(e.mem.u32(material + 0x28), 0x3f00_0000);
            assert_eq!(e.mem.u32(material + 0x2c), 0x3f80_0000);
            assert_eq!(e.mem.u32(material + 0x30), 0x4000_0000);
            assert_eq!(e.mem.u32(material + 0x44), 1);
        }
        assert_eq!(
            calls_to(&e, 0x7000_00dc),
            vec![vec![node, shapes[0], 1], vec![node, shapes[1], 1]]
        );
        assert_eq!(e.mem.u32(word), 0x33);

        // The index array grew by one element per index (capacity 1, 2, .. 6; the second part
        // reuses it after it was cleared); every old block was freed, and the last at the end.
        let blocks = allocated.borrow().clone();
        assert_eq!(blocks.len(), 6);
        assert_eq!(
            calls_to(&e, ARRAY_ALLOCATE),
            (1..=6).map(|n| vec![2 * n]).collect::<Vec<_>>()
        );
        let mut freed = vec![vec![0]];
        freed.extend(blocks.iter().map(|b| vec![*b]));
        assert_eq!(calls_to(&e, OPERATOR_DELETE), freed);
        // The array was zeroed before the second part used it again.
        assert_eq!(ushort_elements_at(&e, blocks[5], 6), [0; 6]);
    }

    #[test]
    fn display_geometry_of_a_shape_without_parts_only_frees_the_empty_array() {
        let mut e = engine();
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let (havok, _) = havok_with_parts(&mut e, &[]);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        e.call(0x00ca5da0, &args![this, 0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0]]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn display_geometry_starts_a_new_run_when_a_triangle_does_not_continue() {
        let mut e = engine();
        install_display_doubles(&mut e);
        e.register(ARRAY_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        e.map(STRING_BLOCK, 0x20);
        e.mem.set_f32(DISPLAY_SCALE, 1.0);
        // One part of 5 vertices: (0 1 2), then (0 1 3) does not continue it and is taken whole,
        // then (1 3 4) continues that one.
        let (havok, data) = havok_with_parts(&mut e, &[(0, 5, 0)]);
        let triangle_block = e.mem.alloc(8 * 3);
        for (index, (a, b, c)) in [(0u16, 1u16, 2u16), (0, 1, 3), (1, 3, 4)]
            .iter()
            .enumerate()
        {
            let record = triangle_block + 8 * index as u32;
            e.mem.set_u16(record, *a);
            e.mem.set_u16(record + 2, *b);
            e.mem.set_u16(record + 4, *c);
        }
        let vertex_block = e.mem.alloc(12 * 5);
        e.mem.set_u32(data + 8, 3);
        e.mem.set_u32(data + 0x14, triangle_block);
        e.mem.set_u32(data + 0x18, vertex_block);
        let this = shape(&mut e);
        e.set(this, BhkPackedNiTriStripsShape::phkObject, Ptr::new(havok));
        let node = object_with_slots(&mut e, 0x40, &[(0xdc, 0x7000_00dc)]);
        let colour = e.mem.alloc(0xc);

        e.call(0x00ca5da0, &args![this, node, colour]);

        let set_triangles = calls_to(&e, SET_TRIANGLES);
        assert_eq!(set_triangles.len(), 1);
        assert_eq!(set_triangles[0][1], 3);
        assert_eq!(
            triangle_indices(&e, &set_triangles[0]),
            [0, 1, 2, 0, 1, 3, 1, 3, 4]
        );
    }

    #[test]
    fn appending_indices_grows_the_array_and_counts_the_non_zero_ones() {
        let mut e = engine();
        install_resize_doubles(&mut e);
        let array = ushort_array(&mut e, 1, &[]);
        for value in [5u32, 0, 7, 0x1_0009] {
            append_index(&mut e, array, value);
        }
        assert_eq!(ushort_elements(&e, array, 4), [5, 0, 7, 9]);
        assert_eq!(e.mem.u16(array + 0xa), 4);
        assert_eq!(e.mem.u16(array + 0xc), 3);
        // Capacity 1 grew by the step 1 each time the array was full.
        assert_eq!(e.mem.u16(array + 8), 4);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE).len(), 3);
    }

    #[test]
    fn link_object_stores_the_linked_object_as_a_counted_pointer() {
        let mut e = engine();
        e.register(0x7000_0004, |_, _| ret(0));
        e.register(SHAPE_LINK_OBJECT, |_, _| ret(0));
        let old = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        e.mem.set_u32(old + 4, 1);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, 1);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| ret(object));
        let cinfo = e.mem.alloc(0x30);
        e.mem.set_u32(cinfo + 4, old);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);

        e.call(0x00ccc3b0, &args![this, 0x6000u32]);

        // The old object lost its last reference and was deleted; the new one has two.
        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(cinfo + 4), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(
            calls_to(&e, SHAPE_LINK_OBJECT),
            vec![vec![this.addr(), 0x6000]]
        );

        // The same object again changes nothing.
        e.call(0x00ccc3b0, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(calls_to(&e, SHAPE_LINK_OBJECT).len(), 2);

        // No object: the pointer is released and cleared.
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| ret(0));
        e.call(0x00ccc3b0, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 1);
    }

    #[test]
    fn bhk_packed_ni_tri_strips_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_5290, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00ca5340, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ca5340, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00ca_5290), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_packed_ni_tri_strips_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ca_5300, &args![]).u32(), 0x0126_82dc);
    }
}
