//! `BSHavok/bhknitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkNiTriStripsShape` (0x14 bytes) wraps a Havok mesh shape: `+8` holds the Havok object (its
//! sub-part array is at `+0x98` with the count at `+0xa0`, 8 bytes per part, and its radius at
//! `+0x80`), `+0xC` the construction info (`pInfo`) and `+0x10` `eMaterial`. The construction
//! info is a 0x40-byte block with an `NiTLargeArray`-style array of 8-byte entries at `+8`
//! (vtable `010c7600`, data `+0xC`, capacity `+0x10`, size `+0x14`, grow step `+0x1C`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{
    enter_context, push_viewer_string, release_fixed_string, release_reference,
};

/// `bhkNiTriStripsShape`'s vtable (`010c771c`) and the two base-class vtables its inlined
/// constructors store on the way (`bhkSerializable` `01030bf4`, `bhkShape` `010c44e4`).
const VTABLE: u32 = 0x010c_771c;
const BASE_VTABLE_SERIALIZABLE: u32 = 0x0103_0bf4;
const BASE_VTABLE_SHAPE: u32 = 0x010c_44e4;
/// Static object counts the inlined constructors increment: `0126 80f8` and `0126 86d0` (base
/// classes) and the class's own `iCount` (`012682e4`, which the destructor decrements).
const SERIALIZABLE_COUNT: u32 = 0x0126_80f8;
const SHAPE_COUNT: u32 = 0x0126_86d0;
const OBJECT_COUNT: u32 = 0x0126_82e4;
/// The word at `012682e8` that `GetViewerStrings` shows as the class name (an `NiRTTI` pointer).
const CLASS_RTTI: u32 = 0x0126_82e8;
/// `bhkNiTriStripsShape::EmptySubpart` (Xbox PDB static, 8 bytes): the part the code uses when the
/// shape or its Havok object is missing.
const EMPTY_SUBPART: u32 = 0x0126_82f0;
/// The vtable of the construction info's entry array, and of the freed construction info.
const ARRAY_VTABLE: u32 = 0x010c_7600;
const FREED_CINFO_VTABLE: u32 = 0x010c_759c;
/// The float `010c72c0` (0.1f) stored into `pInfo + 4` by the binary save and load.
const CINFO_FLOAT: u32 = 0x010c_72c0;
/// `NiRTTI` of `NiTriStrips` (`011f4a70`), which `BuildDisplayGeometry` looks for in the part's
/// class chain.
const NI_TRI_STRIPS_RTTI: u32 = 0x011f_4a70;
/// `"bhkNiTriStripsShape"`, the name given to the display node.
const DISPLAY_NAME: u32 = 0x010c_3fdc;
/// Viewer-string names: `"Subparts"`, `"Part"`, `"MATERIAL"`.
const NAME_SUBPARTS: u32 = 0x010c_7708;
const NAME_PART: u32 = 0x010c_7700;
const NAME_MATERIAL: u32 = 0x010c_59cc;
/// The logger object pointer (`01268360`) and the strings the "old shape" warning is built from.
const LOGGER: u32 = 0x0126_8360;
const WARNING_FILE: u32 = 0x010c_784c;
const WARNING_MIDDLE: u32 = 0x010c_7854;
const WARNING_END: u32 = 0x010c_7878;
const WARNING_SOURCE: u32 = 0x010c_7808;
/// The text used for the file name (`stream + 8`) when it is empty (`"Please"` at `010bdd3c`).
const WARNING_DEFAULT_NAME: u32 = 0x010b_dd3c;

/// `KERNEL32` imports (slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// Helpers of the Havok / game heap.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const OPERATOR_DELETE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

/// Base class methods: `bhkRefObject` constructor (`00c87c30`), the `bhkSerializable` kill
/// (`00c876a0`), the base destructor body (`00cd2de0`), the clone lock and unlock (`00c876f0`,
/// `00c87700`), `bhkShape::CopyMembers` (`00c84df0`), `CopyCinfo` (`00c84e20`), `LoadBinary`
/// (`00c84d00`), `SaveBinary` (`00c84d50`), `LinkObject` (`00c84cc0`), `IsEqual` (`00c84dc0`),
/// `CanShare` (`00c84ec0`) and the viewer strings base (`00cd2fd0`).
const REF_OBJECT_CONSTRUCTOR: u32 = 0x00c8_7c30;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00cd_2de0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
const SHAPE_IS_EQUAL: u32 = 0x00c8_4dc0;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00cd_2fd0;
/// `NiObject::RegisterStreamables` (`00aa0ad0`) and `NiObject::LoadBinary` (`00a5d610`).
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The mesh shape's own methods: the Havok `hkNiTriStripsShape` constructor (`00ca4bc0`,
/// `(block, argument)`), copying the scale into a construction info (`00ca4d20`) and the
/// construction-info array's `SetSize` (`00ca47f0`), `SetAt` (`00ca49a0`) and clear (`00ca4b20`).
const MESH_SHAPE_CONSTRUCT: u32 = 0x00ca_4bc0;
const MESH_SHAPE_COPY_SCALE: u32 = 0x00ca_4d20;
const ARRAY_SET_SIZE: u32 = 0x00ca_47f0;
const ARRAY_SET_AT: u32 = 0x00ca_49a0;
const ARRAY_CLEAR: u32 = 0x00ca_4b20;
/// `00c905b0(object)`: releases the reference `Init` holds on the Havok shape.
const HAVOK_RELEASE: u32 = 0x00c9_05b0;
/// Display geometry: `NiTriStrips` construction from the strips data (`00a71cc0`, `(block,
/// data)`), `NiObjectNET::SetName` (`00a5b950`), `NiFixedString` creation (`00a5b690`, cdecl)
/// and the debug-geometry preparation (`00c811f0`, cdecl `(display, node, material, flag)`).
const NI_TRI_STRIPS_CONSTRUCT: u32 = 0x00a7_1cc0;
const SET_NAME: u32 = 0x00a5_b950;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const PREPARE_DEBUG_GEOMETRY: u32 = 0x00c8_11f0;
/// Streams: `NiStream::GetNumberOfLinkIDs` (`00a63460`), `GetObjectFromLinkID` (`00a63430`),
/// `ReadMultipleLinkIDs` (`00a64670`) and the post-link step of the old format (`00a67000`).
const STREAM_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const OBJECT_LINKED: u32 = 0x00a6_7000;
/// The viewer-string builders: the class name from an `NiRTTI` (`00a7d330`), a name with a
/// number (`00a7d510`, `00a7d4d0`; cdecl `(name, value)`), and the cinfo-part strings (`00c849b0`,
/// `(this = &part_word, strings)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_NUMBER: u32 = 0x00a7_d510;
const NI_GET_VIEWER_STRING_MATERIAL: u32 = 0x00a7_d4d0;
const PART_ADD_VIEWER_STRINGS: u32 = 0x00c8_49b0;
const STRINGS_GROW: u32 = 0x0096_ad30;
/// `hkOstream` constructor (`00cc0500`, `(this, buffer, size, flag)`), `operator<<(const char*)`
/// (`00cc01c0`, returns the stream) and destructor (`00cc00e0`).
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0500;
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;

layout! {
    /// `bhkNiTriStripsShape` (Xbox PDB), 0x14 bytes.
    pub struct BhkNiTriStripsShape: 0x14 {
        /// `phkObject` (Xbox PDB, `bhkRefObject`): the Havok shape.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB, `bhkSerializable`): the construction info.
        0x0c pInfo: Ptr,
        /// `eMaterial` (Xbox PDB, `bhkShape`).
        0x10 eMaterial: u32,
    }
}

/// The start of the part record `index` of the shape's Havok object, or the empty part when the
/// shape or its Havok object is missing. Each record is two words: a pointer to the strips data
/// (`NiTriStripsData`-like object whose `+0xC` word holds the material bits) and a word that the
/// part's viewer strings use.
fn part_record(e: &Engine, this: u32, index: u32) -> u32 {
    if this != 0 {
        let havok = e.mem.u32(this + 8);
        if havok != 0 {
            return e.mem.u32(havok + 0x98).wrapping_add(index.wrapping_mul(8));
        }
    }
    EMPTY_SUBPART
}

/// The number of parts: `+0xA0` of the Havok object, 0 without one.
fn part_count(e: &Engine, this: u32) -> u32 {
    if this != 0 {
        let havok = e.mem.u32(this + 8);
        if havok != 0 {
            return e.mem.u32(havok + 0xa0);
        }
    }
    0
}

/// The material bits (`(word at +0xC) >> 6 & 0x1F`) of the part's strips data, 0 without one.
fn part_material(e: &Engine, record: u32) -> u32 {
    let data = e.mem.u32(record);
    if data == 0 {
        0
    } else {
        u32::from(e.mem.u16(data + 0xc) >> 6) & 0x1f
    }
}

/// What the inlined constructors of the three base classes do: the base constructor, the vtables
/// and object counts, and the cleared `pInfo` / `eMaterial` (the exe stores the vtable of each
/// class in turn; the last, the class's own, stays).
fn construct(e: &mut Engine, this: Ptr<BhkNiTriStripsShape>) {
    e.call(REF_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BASE_VTABLE_SERIALIZABLE);
    e.set(this, BhkNiTriStripsShape::pInfo, Ptr::NULL);
    e.set(this, BhkNiTriStripsShape::eMaterial, 0);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), BASE_VTABLE_SHAPE);
    let count = e.mem.u32(SHAPE_COUNT);
    e.mem.set_u32(SHAPE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_add(1));
}

/// Takes `size` bytes from the memory manager aligned to 16: the shift is stored in the byte
/// before the returned block. Used for the Havok shape and the construction info.
pub(crate) fn allocate_aligned(e: &mut Engine, size: u32) -> u32 {
    let raw = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let shift = 0x10 - (raw & 0xf) as u8;
    let block = raw.wrapping_add(u32::from(shift));
    e.mem.set_u8(block.wrapping_sub(1), shift);
    block
}

/// The Havok allocator (`router + 0x10`).
pub(crate) fn havok_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Empties the Havok array whose data pointer is at `array`, size at `+4` and capacity (with the
/// "not owned" bit 31) at `+8`: sets the size to 0, frees the data (`capacity & 0x3fffffff`
/// elements of `element_size` bytes) through the Havok allocator's slot `+0x10` unless it is not
/// owned, and leaves it empty and not owned.
fn empty_hk_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = havok_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff) * element_size],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Empties the twelve arrays of the mesh construction block at `base` (the `+0x24 + 12 * k`
/// arrays of the 0xB4-byte structure `00ca7310` initialises). Array 4 holds bytes, the others
/// words. The destructor goes last to first, the loader first to last.
fn empty_cinfo_arrays(e: &mut Engine, base: u32, last_first: bool) {
    let order: Vec<u32> = if last_first {
        (0..12).rev().collect()
    } else {
        (0..12).collect()
    };
    for k in order {
        let element_size = if k == 4 { 1 } else { 4 };
        empty_hk_array(e, base + 0x24 + 12 * k, element_size);
    }
}

/// The inlined `NiTArray::SetAt` that `GetViewerStrings` uses from its fourth string on: sets the
/// element at the old size (growing first when full), keeping the count of non-null elements.
pub(crate) fn set_viewer_string_inline(e: &mut Engine, strings: u32, text: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + index]);
    }
    let data = e.mem.u32(strings + 4);
    let size = u32::from(e.mem.u16(strings + 0xa));
    let present = e.mem.u16(strings + 0xc);
    if index >= size {
        e.mem.set_u16(strings + 0xa, (index + 1) as u16);
        if text != 0 {
            e.mem.set_u16(strings + 0xc, present.wrapping_add(1));
        }
    } else {
        let old = e.mem.u32(data + index * 4);
        if text != 0 {
            if old == 0 {
                e.mem.set_u16(strings + 0xc, present.wrapping_add(1));
            }
        } else if old != 0 {
            e.mem.set_u16(strings + 0xc, present.wrapping_add(0xffff));
        }
    }
    e.mem.set_u32(data + index * 4, text);
}

// Translated from 00ca6590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6590 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::~bhkNiTriStripsShape` (engine map): the destructor body. Sets the
/// vtable, kills the serializable base (`00c876a0`), decrements the class's object count and runs
/// the base destructor body (`00cd2de0`).
pub fn bhk_ni_tri_strips_shape_destructor(e: &mut Engine, this: Ptr<BhkNiTriStripsShape>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00ca6610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::CanShare` (the slot `+0xD0` of the vtable `010c771c` holds this
/// function): a 5-byte jump to `00c84ec0` (engine map: `bhkShape::CanShare`), which takes its
/// object on the stack; the stack words and `this` are passed on and its result returned.
pub fn bhk_ni_tri_strips_shape_can_share(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(SHAPE_CAN_SHARE, &args![this, other]).bool()
}

// Translated from 00ca6620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the radius of the shape (fastcall `this`, result in `ST0`): the
/// float at `+0x80` of the Havok object (`[this + 8]`), stored and reloaded as a 32-bit float so a
/// signalling NaN comes back quiet, or 1.0 when `this` or its Havok object is null.
pub fn fn_00ca6620(e: &mut Engine, this: Ptr) -> f32 {
    if !this.is_null() {
        let havok = e.mem.u32(this.addr() + 8);
        if havok != 0 {
            return x87_quiet(e.mem.f32(havok + 0x80));
        }
    }
    1.0
}

// Translated from 00ca6670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6670 - C++ exception unwinding (SEH frame state) is not translated.
/// Unnamed in the engine map: allocates 0x14 bytes (`00aa13e0`) and constructs a
/// `bhkNiTriStripsShape` in them (the three base constructors inlined: the `bhkRefObject`
/// constructor `00c87c30`, then the vtables, the cleared `pInfo` and `eMaterial` and the three
/// object counts). Null when the allocation failed.
pub fn fn_00ca6670(e: &mut Engine) -> Ptr<BhkNiTriStripsShape> {
    let block = e
        .call(OPERATOR_NEW, &args![0x14u32])
        .ptr::<BhkNiTriStripsShape>();
    if block.is_null() {
        return Ptr::NULL;
    }
    construct(e, block);
    block
}

// Translated from 00ca6720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::CopyMembers` (engine map): `process` is the cloning process whose float
/// at `+0x10` is a scale. Takes the construction info (`CreateCinfo`, virtual slot `+0xB4`),
/// multiplies the four floats at `+0x20..+0x2C` of it by that scale (the float is stored and
/// reloaded through the stack, quieting a signalling NaN) and runs `bhkShape::CopyMembers`
/// (`00c84df0`, `(destination, process)`) on `this`.
pub fn bhk_ni_tri_strips_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    destination: Ptr,
    process: Ptr,
) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let scale = x87_quiet(e.mem.f32(process.addr() + 0x10));
    for offset in [0x20u32, 0x24, 0x28, 0x2c] {
        let value = e.mem.f32(cinfo + offset);
        e.mem.set_f32(cinfo + offset, scale * value);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, destination, process]);
}

// Translated from 00ca6780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::LoadCInfo` (engine map): `stream` is the `NiStream`. Takes the
/// construction info (virtual slot `+0xB4`) and its size (`QCinfoSize`, slot `+0xA8`, less 0x10 for
/// stream versions below 4), reads that many bytes into it through the stream's input object
/// (`[stream + 0x24c]`, function at `+8`, `(io, buffer, size, 0, 0)`), adds the bytes read to the
/// object's counter (`+4`) and, for a stream whose byte at `+0x1E5` is not 1 and version at least
/// 4, byte-swaps the four words at `+0x20..+0x2C`. Returns the size.
pub fn bhk_ni_tri_strips_shape_load_c_info(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    stream: Ptr,
) -> u32 {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let mut size = e.vcall(this.addr(), 0xa8, &args![]).u32();
    let version = e.mem.u32(stream.addr() + 4);
    if version < 4 {
        size = size.wrapping_sub(0x10);
    }
    let io = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(io + 8);
    let count = e.call(read, &args![io, cinfo, size, 0u32, 0u32]).u32();
    let total = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, total.wrapping_add(count));
    if e.mem.u8(stream.addr() + 0x1e5) != 1 && version >= 4 {
        for offset in [0x20u32, 0x24, 0x28, 0x2c] {
            let word = e.mem.u32(cinfo + offset);
            e.mem.set_u32(cinfo + offset, word.swap_bytes());
        }
    }
    size
}

// Translated from 00ca6810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::RegisterStreamables` (engine map): registers the strips data of every
/// part (virtual slot `+0x54` of the object in each part record, when there is one) with the
/// stream, then `NiObject::RegisterStreamables` (`00aa0ad0`), whose result is returned.
pub fn bhk_ni_tri_strips_shape_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    stream: Ptr,
) -> bool {
    let count = part_count(e, this.addr()) as i32;
    for index in 0..count {
        let record = part_record(e, this.addr(), index as u32);
        let data = e.mem.u32(record);
        if data != 0 {
            e.vcall(data, 0x54, &args![stream]);
        }
    }
    e.call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00ca6880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::IsEqual` (engine map): the base comparison (`00c84dc0`) must pass (its
/// result is returned when it does not); then the shapes must have the same number of parts (0
/// for a missing Havok object).
pub fn bhk_ni_tri_strips_shape_is_equal(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    other: Ptr<BhkNiTriStripsShape>,
) -> bool {
    if !e.call(SHAPE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = part_count(e, this.addr());
    if !other.is_null() && e.mem.u32(other.addr() + 8) != 0 {
        count == part_count(e, other.addr())
    } else {
        count == 0
    }
}

// Translated from 00ca68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::QSubMaterial` (engine map): `-1` gives the shape's `eMaterial` & 0x1F;
/// any other value is a packed part index (bits 20 and up): the material bits of that part's
/// strips data (0 when there is none).
pub fn bhk_ni_tri_strips_shape_q_sub_material(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    index: u32,
) -> u32 {
    if index == 0xffff_ffff {
        return e.get(this, BhkNiTriStripsShape::eMaterial) & 0x1f;
    }
    let record = part_record(e, this.addr(), index >> 0x14);
    part_material(e, record)
}

// Translated from 00ca6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::GetSubMaterialVarType` (engine map): `-1` tests `eMaterial` against the
/// mask; any other value is a packed part index: for the mask 0x20 bit 5 of the strips data's
/// word at `+0xC`, for 0x40 its bit 11, otherwise false (and false without strips data).
pub fn bhk_ni_tri_strips_shape_get_sub_material_var_type(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    index: u32,
    mask: u32,
) -> bool {
    if index == 0xffff_ffff {
        return e.get(this, BhkNiTriStripsShape::eMaterial) & mask != 0;
    }
    let record = part_record(e, this.addr(), index >> 0x14);
    let data = e.mem.u32(record);
    if data == 0 {
        return false;
    }
    match mask {
        0x20 => (e.mem.u8(data + 0xc) >> 5) & 1 != 0,
        0x40 => (e.mem.u16(data + 0xc) >> 0xb) & 1 != 0,
        _ => false,
    }
}

// Translated from 00ca6990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6990 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::CreateClone` (engine map): with the thread block's `+0x2b4` word at 0x10
/// and the clone lock held (`00c876f0` / `00c87700`), allocates and constructs a new shape as
/// `00ca6670` does, then runs the original's `CopyMembers` (virtual slot `+0xC4`) with the new
/// shape (null when the allocation failed) and `process`. Returns the new shape.
pub fn bhk_ni_tri_strips_shape_create_clone(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    process: Ptr,
) -> Ptr<BhkNiTriStripsShape> {
    let (word, saved) = enter_context(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ca6670(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ca6a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6a80 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::Init` (engine map): with the thread block's `+0x2b4` word at 0x10, takes
/// 0xC0 bytes aligned to 16 from the memory manager, constructs the Havok shape in them
/// (`00ca4bc0(block, argument)`), hands it to the shape (virtual slot `+0x8C`, `SethkObject`),
/// releases the creation reference (`00c905b0`) and runs `PostInit` (slot `+0xBC`) with
/// `argument`.
pub fn bhk_ni_tri_strips_shape_init(e: &mut Engine, this: Ptr<BhkNiTriStripsShape>, argument: u32) {
    let (word, saved) = enter_context(e);
    let block = allocate_aligned(e, 0xc0);
    let shape = e.call(MESH_SHAPE_CONSTRUCT, &args![block, argument]).u32();
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(HAVOK_RELEASE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![argument]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ca6b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6b60 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::BuildDisplayGeometry` (engine map): with the thread block's `+0x2b4` word
/// at 0x10, for every part whose strips data is an `NiTriStrips` (its class chain, from virtual
/// slot `+8`, contains the `NiRTTI` at `011f4a70`): builds an `NiTriStrips` from it (0xC4 bytes,
/// `00a71cc0`), names it `"bhkNiTriStripsShape"`, gives it the absolute value of the shape's radius
/// (`00ca6620`) as the scale at `+0x64`, prepares it as debug geometry (`00c811f0(display, node,
/// material bits, flag)`) and attaches it to `node` (slot `+0xDC` with 1).
pub fn bhk_ni_tri_strips_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    node: Ptr,
    flag: u32,
) {
    let (word, saved) = enter_context(e);
    let count = part_count(e, this.addr()) as i32;
    for index in 0..count {
        let record = part_record(e, this.addr(), index as u32);
        let data = e.mem.u32(record);
        if data == 0 {
            continue;
        }
        let mut class = e.vcall(data, 8, &args![]).u32();
        let mut is_strips = false;
        while class != 0 {
            if class == NI_TRI_STRIPS_RTTI {
                is_strips = true;
                break;
            }
            class = e.mem.u32(class + 4);
        }
        if !is_strips {
            continue;
        }
        let memory = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
        let display = if memory != 0 {
            e.call(NI_TRI_STRIPS_CONSTRUCT, &args![memory, data]).u32()
        } else {
            0
        };
        let handle = e.call(FIXED_STRING_CREATE, &args![DISPLAY_NAME]).u32();
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), handle);
            e.call(SET_NAME, &args![display, slot]);
        });
        release_fixed_string(e, handle);
        let radius = e.call(0x00ca_6620, &args![this]).f32();
        e.mem.set_f32(display + 0x64, radius.abs());
        let material = part_material(e, record);
        e.call(
            PREPARE_DEBUG_GEOMETRY,
            &args![display, node, material, flag],
        );
        e.vcall(node.addr(), 0xdc, &args![display, 1u32]);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00ca6d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6d40 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::CreateCinfo` (engine map): with the thread block's `+0x2b4` word at 0x10,
/// sets the byte at `created` to whether `pInfo` was empty. If it was, takes 0x40 bytes aligned to
/// 16 from the memory manager and makes them the construction info: vtable `010c7600` for the
/// entry array at `+8` (empty, grow step 1), the float at `+4` and `+0x20..+0x2C` zero, the scale
/// at `+0x20..+0x28` 1.0; stores it in `pInfo`; with a Havok object copies the shape's data into it
/// (`00c84e20`) and then the scale from the Havok object (`00ca4d20`), or sets the scale to 1.0
/// when `CopyCinfo` left the shape without a Havok object. Returns `pInfo`.
pub fn bhk_ni_tri_strips_shape_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    created: Ptr,
) -> Ptr {
    let (word, saved) = enter_context(e);
    let empty = e.get(this, BhkNiTriStripsShape::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let cinfo = allocate_aligned(e, 0x40);
        e.mem.set_u32(cinfo, 0);
        e.mem.set_f32(cinfo + 4, 0.0);
        e.mem.set_u32(cinfo + 0x10, 0);
        e.mem.set_u32(cinfo + 0x1c, 1);
        e.mem.set_u32(cinfo + 0x14, 0);
        e.mem.set_u32(cinfo + 0x18, 0);
        e.mem.set_u32(cinfo + 0xc, 0);
        e.mem.set_u32(cinfo + 8, ARRAY_VTABLE);
        for offset in [0x20u32, 0x24, 0x28, 0x2c] {
            e.mem.set_f32(cinfo + offset, 0.0);
        }
        for offset in [0x20u32, 0x24, 0x28] {
            e.mem.set_f32(cinfo + offset, 1.0);
        }
        e.set(this, BhkNiTriStripsShape::pInfo, Ptr::new(cinfo));
        if !e.get(this, BhkNiTriStripsShape::phkObject).is_null() {
            e.call(SHAPE_COPY_CINFO, &args![this, cinfo]);
            let havok = e.get(this, BhkNiTriStripsShape::phkObject);
            if havok.is_null() {
                for offset in [0x20u32, 0x24, 0x28] {
                    e.mem.set_f32(cinfo + offset, 1.0);
                }
            } else {
                e.call(MESH_SHAPE_COPY_SCALE, &args![havok, cinfo]);
            }
        }
    }
    let info = e.get(this, BhkNiTriStripsShape::pInfo);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00ca6e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6e60 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::KillCinfo` (engine map): with the thread block's `+0x2b4` word at 0x10
/// and when the low byte of `destroy` is not zero: if there is a construction info, gives it its
/// freed vtable (`010c759c`), frees its entry data (`00aa10f0`) and its aligned block (the shift
/// is in the byte before it, `00aa4060` on the memory manager); `pInfo` is cleared.
pub fn bhk_ni_tri_strips_shape_kill_cinfo(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    destroy: u32,
) {
    let (word, saved) = enter_context(e);
    if destroy as u8 != 0 {
        let cinfo = e.get(this, BhkNiTriStripsShape::pInfo).addr();
        if cinfo != 0 {
            let data = e.mem.u32(cinfo + 0xc);
            e.mem.set_u32(cinfo + 8, FREED_CINFO_VTABLE);
            e.call(OPERATOR_DELETE, &args![data]);
            let shift = u32::from(e.mem.u8(cinfo - 1));
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, cinfo - shift]);
        }
        e.set(this, BhkNiTriStripsShape::pInfo, Ptr::NULL);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00ca6f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca6f10 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::GetViewerStrings` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, appends to `strings` the base class strings (`00cd2fd0`), the class name, `"Subparts"`
/// with the number of parts, and then for the first five parts, and for the last three of those
/// beyond the fifth (last one first), `"Part"` with the index, `"MATERIAL"` with the part's
/// material bits and the strings the part adds for itself (`00c849b0` with the address of a copy of
/// the record's second word). The first two strings go through the array's `SetAt`
/// (`0096ae90`); the rest use its inlined form.
pub fn bhk_ni_tri_strips_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    strings: Ptr,
) {
    let (word, saved) = enter_context(e);
    let array = strings.addr();
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, array, text);
    let total = part_count(e, this.addr());
    let text = e
        .call(NI_GET_VIEWER_STRING_NUMBER, &args![NAME_SUBPARTS, total])
        .u32();
    push_viewer_string(e, array, text);
    let mut first = total;
    let mut last = 0;
    if total > 5 {
        last = total - 5;
        first = 5;
        if last > 3 {
            last = 3;
        }
    }
    for index in 0..first {
        add_part_strings(e, this, array, index);
    }
    for back in 0..last {
        add_part_strings(e, this, array, total - back - 1);
    }
    e.mem.set_u32(word, saved);
}

/// The viewer strings of the part `index`: `"Part"` with the index, `"MATERIAL"` with the
/// material bits, then the part's own strings.
fn add_part_strings(e: &mut Engine, this: Ptr<BhkNiTriStripsShape>, strings: u32, index: u32) {
    let text = e
        .call(NI_GET_VIEWER_STRING_NUMBER, &args![NAME_PART, index])
        .u32();
    set_viewer_string_inline(e, strings, text);
    let record = part_record(e, this.addr(), index);
    let material = part_material(e, record);
    let text = e
        .call(
            NI_GET_VIEWER_STRING_MATERIAL,
            &args![NAME_MATERIAL, material],
        )
        .u32();
    set_viewer_string_inline(e, strings, text);
    let word = e.mem.u32(record + 4);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), word);
        e.call(PART_ADD_VIEWER_STRINGS, &args![local, strings]);
    });
}

// Translated from 00ca7310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall `this`): initialises the 0xB4-byte mesh construction
/// block that `00ca73c0` takes apart and `LoadBinary` reads into: the word at `+0`, the four floats
/// at `+0x10..+0x1C` zero, and twelve empty Havok arrays from `+0x24` in steps of 12 (data 0, size
/// 0, capacity `0x80000000` meaning "not owned"). The words at `+4`, `+8`, `+0xC` and `+0x20` are
/// not written.
pub fn fn_00ca7310(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    for k in 0..12 {
        let array = base + 0x24 + 12 * k;
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
    }
}

// Translated from 00ca73c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca73c0 - C++ exception unwinding (SEH frame state) is not translated.
/// Destructor of the mesh construction block `00ca7310` initialises (the engine map names it
/// `hkpMeshShapeCinfo::~hkpMeshShapeCinfo`; the identity is unconfirmed beyond the Ghidra label,
/// code folding may hide other users). Empties the twelve Havok arrays from the last (`+0xA8`) to
/// the first (`+0x24`): each gets the router's allocator (`00c85750`), a zero size, its data freed
/// (slot `+0x10`, `(data, bytes)`) unless the capacity word has bit 31, then data 0 and the
/// capacity word `0x80000000`. The fifth array (`+0x54`) holds bytes, the others words.
pub fn fn_00ca73c0(e: &mut Engine, this: Ptr) {
    empty_cinfo_arrays(e, this.addr(), true);
}

// Translated from 00ca76b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca76b0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiTriStripsShape::LinkObject` (engine map): takes the construction info (virtual slot
/// `+0xB4`) and the number of link ids (`00a63460`). For stream versions below 2 first grows the
/// construction info's entry array to that many zero entries (`00ca47f0`, `00ca49a0`). Then for
/// each link id takes the linked object (`00a63430`), runs `00a67000` on it and stores it as the
/// first word of the entry (an `NiPointer` assignment: the old object is released, the new one gets a
/// reference). Ends with `bhkShape::LinkObject` (`00c84cc0`).
pub fn bhk_ni_tri_strips_shape_link_object(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    stream: Ptr,
) {
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let count = e.call(STREAM_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let array = cinfo + 8;
    if e.mem.u32(stream.addr() + 4) < 2 {
        e.call(ARRAY_SET_SIZE, &args![array, count]);
        for _ in 0..count {
            let size = e.mem.u32(array + 0xc);
            if size >= e.mem.u32(array + 8) {
                let step = e.mem.u32(array + 0x14);
                e.call(ARRAY_SET_SIZE, &args![array, step + size]);
            }
            e.with_stack(8, |e, entry| {
                e.call(ARRAY_SET_AT, &args![array, size, entry]);
            });
        }
    }
    for index in 0..count {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        e.call(OBJECT_LINKED, &args![object]);
        let slot = e.mem.u32(cinfo + 0xc) + index * 8;
        let old = e.mem.u32(slot);
        if old != object {
            release_reference(e, old);
            e.mem.set_u32(slot, object);
            if object != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
            }
        }
    }
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
}

/// Byte-swaps the float at `+4` and the four words at `+0x20..+0x2C` of the construction info.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for offset in [4u32, 0x20, 0x24, 0x28, 0x2c] {
        let word = e.mem.u32(cinfo + offset);
        e.mem.set_u32(cinfo + offset, word.swap_bytes());
    }
}

/// Writes `value` (4 bytes) through the stream's output object: cdecl `(io, &value, 4, &4, 1)`
/// at the function in slot `+0xC`; the byte count it returns is added to the counter at `+4`.
fn write_stream_word(e: &mut Engine, stream: u32, value: u32) {
    let io = e.mem.u32(stream + 0x250);
    let function = e.mem.u32(io + 0xc);
    let written = e.with_stack(8, |e, cells| {
        e.mem.set_u32(cells.addr(), value);
        e.mem.set_u32(cells.addr() + 4, 4);
        e.call(function, &args![io, cells, 4u32, cells.addr() + 4, 1u32])
            .u32()
    });
    let total = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, total.wrapping_add(written));
}

// Translated from 00ca77f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca77f0 - C++ exception unwinding (SEH frame state) is not translated; the upper three bytes of the `KillCinfo` argument come from the uninitialised stack word the exe reuses.
/// `bhkNiTriStripsShape::SaveBinary` (engine map): with the thread block's `+0x2b4` word at 0x10:
/// takes the construction info (slot `+0xB4`), copies its entry array (`+8`, 8-byte entries) into a
/// local array, empties the original (`00ca4b20`, `SetSize(0)`), stores the float `010c72c0` at
/// `pInfo + 4` and, when the stream's byte at `+0x1E4` is not 1, byte-swaps `+4` and
/// `+0x20..+0x2C`; runs `bhkShape::SaveBinary` (`00c84d50`); writes the entry count (4 bytes), the
/// link of each entry's first word (the stream's slot `+0x30`) and the count again followed by
/// each entry's second word; swaps the words back; kills the construction info (slot `+0xA4`)
/// and frees the local array's data (`00aa10f0`).
pub fn bhk_ni_tri_strips_shape_save_binary(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    stream: Ptr,
) {
    let (word, saved) = enter_context(e);
    let stream_addr = stream.addr();
    let (cinfo, created) = e.with_stack(4, |e, created| {
        let cinfo = e.vcall(this.addr(), 0xb4, &args![created]).u32();
        (cinfo, e.mem.u8(created.addr()))
    });
    let local = e.mem.alloc(0x18);
    e.mem.set_u32(local, ARRAY_VTABLE);
    e.mem.set_u32(local + 0x14, 1);
    let count = e.mem.u32(cinfo + 0x14) as i32;
    for index in 0..count {
        let entry = e.mem.u32(cinfo + 0xc).wrapping_add(index as u32 * 8);
        let size = e.mem.u32(local + 0xc);
        if size >= e.mem.u32(local + 8) {
            let step = e.mem.u32(local + 0x14);
            e.call(ARRAY_SET_SIZE, &args![local, step + size]);
        }
        e.call(ARRAY_SET_AT, &args![local, size, entry]);
    }
    let array = cinfo + 8;
    e.call(ARRAY_CLEAR, &args![array]);
    e.call(ARRAY_SET_SIZE, &args![array, 0u32]);
    let constant = x87_quiet_bits(e.mem.u32(CINFO_FLOAT));
    e.mem.set_u32(cinfo + 4, constant);
    let swap = e.mem.u8(stream_addr + 0x1e4) != 1;
    if swap {
        swap_cinfo_words(e, cinfo);
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    write_stream_word(e, stream_addr, count as u32);
    for index in 0..count {
        let object = e.mem.u32(e.mem.u32(local + 4) + index as u32 * 8);
        e.vcall(stream_addr, 0x30, &args![object]);
    }
    write_stream_word(e, stream_addr, count as u32);
    for index in 0..count {
        let value = e.mem.u32(e.mem.u32(local + 4) + index as u32 * 8 + 4);
        write_stream_word(e, stream_addr, value);
    }
    if e.mem.u8(stream_addr + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.vcall(this.addr(), 0xa4, &args![u32::from(created)]);
    let data = e.mem.u32(local + 4);
    e.call(OPERATOR_DELETE, &args![data]);
    e.mem.free(local);
    e.mem.set_u32(word, saved);
}

// Translated from 00ca7a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca7a70 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated; the third argument of the `hkOstream` constructor is built from a stack slot whose upper three bytes are uninitialised.
/// `bhkNiTriStripsShape::LoadBinary` (engine map). Takes the construction info (slot `+0xB4`).
///
/// Stream versions below 2 (an old `bhkpMeshShape`): builds the text `"File " + stream+0xE0 + "
/// contains an old bhkpMeshShape! " + (stream+8, or "Please" when that is empty) + " re-export\n"`
/// in a 0x200-byte `hkOstream` buffer and reports it through the logger at `01268360` (slot
/// `+0xC`, `(1, 0x234f2250, text, source path, 0xAA)`); runs `NiObject::LoadBinary` (`00a5d610`);
/// initialises a local mesh construction block (`00ca7310`), reads 0xC0 bytes into it from the
/// stream's input object (`[stream + 0x24c]`, function at `+8`), frees its arrays (as the loader
/// order of `00ca73c0`), byte-swaps (stream byte `+0x1E5` not 1) the first two words, copies the
/// float at `+4` of the block to `pInfo + 4`, byte-swaps `+0x10..+0x1C`, sets `eMaterial` to the
/// first word (0 when it is 0x24 or more), clears `pInfo`'s first word, sets `pInfo + 4` to
/// `010c72c0`, reads the link ids (`00a64670`) and destroys the block (`00ca73c0`).
///
/// Newer versions: `bhkShape::LoadBinary` (`00c84d00`), `00a64670`, a 4-byte entry count (swapped
/// when `+0x1E5` is not 1: the exe swaps the float at `pInfo + 4`), and for each entry a 4-byte
/// word (below version 9 bit 5 moves to bit 15) appended to the construction info's entry array
/// (first word 0).
pub fn bhk_ni_tri_strips_shape_load_binary(
    e: &mut Engine,
    this: Ptr<BhkNiTriStripsShape>,
    stream: Ptr,
) {
    let stream_addr = stream.addr();
    let cinfo = e.with_stack(4, |e, created| {
        e.vcall(this.addr(), 0xb4, &args![created]).u32()
    });
    let version = e.mem.u32(stream_addr + 4);
    if version < 2 {
        load_old_mesh_shape(e, this, stream_addr, cinfo);
    } else {
        e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
        let io = e.mem.u32(stream_addr + 0x24c);
        let (count, read) = e.with_stack(8, |e, cells| {
            e.mem.set_u32(cells.addr() + 4, 4);
            let function = e.mem.u32(io + 8);
            let read = e
                .call(function, &args![io, cells, 4u32, cells.addr() + 4, 1u32])
                .u32();
            (e.mem.u32(cells.addr()), read)
        });
        let total = e.mem.u32(io + 4);
        e.mem.set_u32(io + 4, total.wrapping_add(read));
        let swap = e.mem.u8(stream_addr + 0x1e5) != 1;
        if swap {
            let word = e.mem.u32(cinfo + 4);
            e.mem.set_u32(cinfo + 4, word.swap_bytes());
        }
        let array = cinfo + 8;
        for _ in 0..count {
            let io = e.mem.u32(stream_addr + 0x24c);
            let (first, mut second, read) = e.with_stack(0xc, |e, cells| {
                e.mem.set_u32(cells.addr() + 8, 4);
                let function = e.mem.u32(io + 8);
                let read = e
                    .call(
                        function,
                        &args![io, cells.addr() + 4, 4u32, cells.addr() + 8, 1u32],
                    )
                    .u32();
                (e.mem.u32(cells.addr()), e.mem.u32(cells.addr() + 4), read)
            });
            let total = e.mem.u32(io + 4);
            e.mem.set_u32(io + 4, total.wrapping_add(read));
            if e.mem.u32(stream_addr + 4) < 9 && second & 0x20 != 0 {
                second = (second & 0xffff_ffdf) | 0x8000;
            }
            let size = e.mem.u32(array + 0xc);
            if size >= e.mem.u32(array + 8) {
                let step = e.mem.u32(array + 0x14);
                e.call(ARRAY_SET_SIZE, &args![array, step + size]);
            }
            let leftover = e.with_stack(8, |e, entry| {
                e.mem.set_u32(entry.addr(), first);
                e.mem.set_u32(entry.addr() + 4, second);
                e.call(ARRAY_SET_AT, &args![array, size, entry]);
                e.mem.u32(entry.addr())
            });
            release_reference(e, leftover);
        }
    }
}

/// The stream-version-below-2 half of `LoadBinary`.
fn load_old_mesh_shape(e: &mut Engine, this: Ptr<BhkNiTriStripsShape>, stream: u32, cinfo: u32) {
    let name = if e.mem.u8(stream + 8) == 0 {
        WARNING_DEFAULT_NAME
    } else {
        stream + 8
    };
    let stream_ptr = Ptr::<()>::new(stream);
    e.with_stack(0x200, |e, buffer| {
        e.with_stack(0xc, |e, ostream| {
            e.call(OSTREAM_CONSTRUCT, &args![ostream, buffer, 0x200u32, 1u32]);
            let mut chain = ostream.addr();
            for text in [
                WARNING_FILE,
                stream + 0xe0,
                WARNING_MIDDLE,
                name,
                WARNING_END,
            ] {
                chain = e.call(OSTREAM_WRITE_TEXT, &args![chain, text]).u32();
            }
            let logger = e.mem.u32(LOGGER);
            e.vcall(
                logger,
                0xc,
                &args![1u32, 0x234f_2250u32, buffer, WARNING_SOURCE, 0xaau32],
            );
            e.call(OSTREAM_DESTRUCT, &args![ostream]);
        });
    });
    e.call(OBJECT_LOAD_BINARY, &args![this, stream_ptr]);
    e.with_stack(0xc0, |e, block| {
        let base = block.addr();
        fn_00ca7310(e, block);
        let io = e.mem.u32(stream + 0x24c);
        let function = e.mem.u32(io + 8);
        let read = e
            .call(function, &args![io, base, 0xc0u32, 0u32, 0u32])
            .u32();
        let total = e.mem.u32(io + 4);
        e.mem.set_u32(io + 4, total.wrapping_add(read));
        empty_cinfo_arrays(e, base, false);
        let swap = e.mem.u8(stream + 0x1e5) != 1;
        let mut material = e.mem.u32(base);
        if swap {
            material = material.swap_bytes();
            let second = e.mem.u32(base + 4).swap_bytes();
            e.mem.set_u32(base, material);
            e.mem.set_u32(base + 4, second);
        }
        let radius = x87_quiet_bits(e.mem.u32(base + 4));
        e.mem.set_u32(cinfo + 4, radius);
        if swap {
            for offset in [0x10u32, 0x14, 0x18, 0x1c] {
                let word = e.mem.u32(base + offset).swap_bytes();
                e.mem.set_u32(base + offset, word);
            }
        }
        if (material as i32) >= 0x24 {
            material = 0;
            e.mem.set_u32(base, 0);
        }
        e.set(this, BhkNiTriStripsShape::eMaterial, material);
        e.mem.set_u32(cinfo, 0);
        let constant = x87_quiet_bits(e.mem.u32(CINFO_FLOAT));
        e.mem.set_u32(cinfo + 4, constant);
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream_ptr]);
        fn_00ca73c0(e, block);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ca6590,
            bhk_ni_tri_strips_shape_destructor(Ptr<BhkNiTriStripsShape>)
        ),
        entry!(0x00ca6600, bhk_ni_tri_strips_shape_get_r_t_t_i() -> u32),
        entry!(0x00ca6610, bhk_ni_tri_strips_shape_can_share(Ptr, Ptr) -> bool),
        entry!(0x00ca6620, fn_00ca6620(Ptr) -> f32),
        entry!(0x00ca6650, bhk_ni_tri_strips_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ca6670, fn_00ca6670() -> Ptr<BhkNiTriStripsShape>),
        entry!(
            0x00ca6720,
            bhk_ni_tri_strips_shape_copy_members(Ptr<BhkNiTriStripsShape>, Ptr, Ptr)
        ),
        entry!(
            0x00ca6780,
            bhk_ni_tri_strips_shape_load_c_info(Ptr<BhkNiTriStripsShape>, Ptr) -> u32
        ),
        entry!(
            0x00ca6810,
            bhk_ni_tri_strips_shape_register_streamables(Ptr<BhkNiTriStripsShape>, Ptr) -> bool
        ),
        entry!(
            0x00ca6880,
            bhk_ni_tri_strips_shape_is_equal(
                Ptr<BhkNiTriStripsShape>,
                Ptr<BhkNiTriStripsShape>,
            ) -> bool
        ),
        entry!(
            0x00ca68d0,
            bhk_ni_tri_strips_shape_q_sub_material(Ptr<BhkNiTriStripsShape>, u32) -> u32
        ),
        entry!(
            0x00ca6920,
            bhk_ni_tri_strips_shape_get_sub_material_var_type(
                Ptr<BhkNiTriStripsShape>,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x00ca6990,
            bhk_ni_tri_strips_shape_create_clone(
                Ptr<BhkNiTriStripsShape>,
                Ptr,
            ) -> Ptr<BhkNiTriStripsShape>
        ),
        entry!(
            0x00ca6a80,
            bhk_ni_tri_strips_shape_init(Ptr<BhkNiTriStripsShape>, u32)
        ),
        entry!(
            0x00ca6b60,
            bhk_ni_tri_strips_shape_build_display_geometry(Ptr<BhkNiTriStripsShape>, Ptr, u32)
        ),
        entry!(
            0x00ca6d40,
            bhk_ni_tri_strips_shape_create_cinfo(Ptr<BhkNiTriStripsShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00ca6e60,
            bhk_ni_tri_strips_shape_kill_cinfo(Ptr<BhkNiTriStripsShape>, u32)
        ),
        entry!(
            0x00ca6f10,
            bhk_ni_tri_strips_shape_get_viewer_strings(Ptr<BhkNiTriStripsShape>, Ptr)
        ),
        entry!(0x00ca7310, fn_00ca7310(Ptr)),
        entry!(0x00ca73c0, fn_00ca73c0(Ptr)),
        entry!(
            0x00ca76b0,
            bhk_ni_tri_strips_shape_link_object(Ptr<BhkNiTriStripsShape>, Ptr)
        ),
        entry!(
            0x00ca77f0,
            bhk_ni_tri_strips_shape_save_binary(Ptr<BhkNiTriStripsShape>, Ptr)
        ),
        entry!(
            0x00ca7a70,
            bhk_ni_tri_strips_shape_load_binary(Ptr<BhkNiTriStripsShape>, Ptr)
        ),
    ]
}

// Translated from 00ca6650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ca6590`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ni_tri_strips_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_6590;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ca6600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiTriStripsShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_82e8.
pub fn bhk_ni_tri_strips_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_82e8
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

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
        e.map(0x0126_82e0, 0x20);
        e.map(CINFO_FLOAT, 4);
        e.map(LOGGER, 4);
        e.map(0x0109_b220, 4);
        e.mem.set_u32(0x0109_b220, 0x9000);
        e.mem.set_u32(CINFO_FLOAT, 0x3dcc_cccd);
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

    fn shape(e: &mut Engine) -> Ptr<BhkNiTriStripsShape> {
        e.new_object::<BhkNiTriStripsShape>()
    }

    /// A shape whose vtable has the given slots.
    fn shape_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<BhkNiTriStripsShape> {
        Ptr::new(object_with_slots(e, 0x14, slots))
    }

    /// A Havok object (0xb0 bytes) with a part array of `(strips data, word)` records.
    fn havok_with_parts(e: &mut Engine, parts: &[(u32, u32)]) -> u32 {
        let havok = e.mem.alloc(0xb0);
        let array = e.mem.alloc(8 * parts.len() as u32 + 8);
        for (index, (data, word)) in parts.iter().enumerate() {
            e.mem.set_u32(array + 8 * index as u32, *data);
            e.mem.set_u32(array + 8 * index as u32 + 4, *word);
        }
        e.mem.set_u32(havok + 0x98, array);
        e.mem.set_u32(havok + 0xa0, parts.len() as u32);
        havok
    }

    /// A strips-data object whose word at +0xC holds `material` in bits 6..10.
    fn strips_data(e: &mut Engine, material: u16, extra: u16) -> u32 {
        let data = e.mem.alloc(0x20);
        e.mem.set_u16(data + 0xc, (material << 6) | extra);
        data
    }

    /// A stream (0x260 bytes) of `version` whose input and output objects are one block `io` with
    /// the byte counter at +4 set to 10 and the functions at +8 and +0xC.
    fn stream_fixture(e: &mut Engine, version: u32, read: u32, write: u32) -> (u32, u32) {
        let stream = object_with_slots(e, 0x260, &[(0x30, 0x7000_0030)]);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 4, version);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(io + 8, read);
        e.mem.set_u32(io + 0xc, write);
        (stream, io)
    }

    /// Doubles for the entry array methods the construction info uses: `SetSize(array, n)` makes
    /// room for `n` entries (the data pointer at +4 and capacity at +8) and `SetAt(array, index,
    /// entry)` copies the two words and sets the size (+0xC) to `index + 1`.
    fn install_entry_array_doubles(e: &mut Engine) {
        e.register(ARRAY_SET_SIZE, |e, a| {
            let data = e.mem.alloc(8 * a[1] + 8);
            let old = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 0xc);
            for word in 0..2 * size.min(a[1]) {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(data + 4 * word, value);
            }
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u32(a[0] + 8, a[1]);
            ret(0)
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let data = e.mem.u32(a[0] + 4);
            let first = e.mem.u32(a[2]);
            let second = e.mem.u32(a[2] + 4);
            e.mem.set_u32(data + 8 * a[1], first);
            e.mem.set_u32(data + 8 * a[1] + 4, second);
            e.mem.set_u32(a[0] + 0xc, a[1] + 1);
            ret(0)
        });
    }

    /// A construction info (0x40 bytes) whose entry array is empty with grow step 1.
    fn cinfo_fixture(e: &mut Engine) -> u32 {
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(cinfo + 0x1c, 1);
        cinfo
    }

    #[test]
    fn destructor_body_counts_the_object_out() {
        let mut e = engine();
        e.register(SERIALIZABLE_KILL, |_, _| ret(0));
        e.register(BASE_DESTRUCTOR_BODY, |_, _| ret(0));
        e.mem.set_u32(OBJECT_COUNT, 4);
        let this = shape(&mut e);
        e.call(0x00ca6590, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 3);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00ca6590, SERIALIZABLE_KILL, BASE_DESTRUCTOR_BODY]);
    }

    #[test]
    fn can_share_passes_this_and_the_stack_word_to_the_base() {
        let mut e = engine();
        e.register(SHAPE_CAN_SHARE, |_, a| ret(u32::from(a[1] == 0x2000)));
        assert!(e.call(0x00ca6610, &args![0x1000u32, 0x2000u32]).bool());
        assert!(!e.call(0x00ca6610, &args![0x1000u32, 0x3000u32]).bool());
        assert_eq!(calls_to(&e, SHAPE_CAN_SHARE)[0], [0x1000, 0x2000]);
    }

    #[test]
    fn radius_is_read_from_the_havok_object_or_one() {
        let mut e = engine();
        let this = shape(&mut e);
        assert_eq!(e.call(0x00ca6620, &args![this]).f32(), 1.0);
        assert_eq!(e.call(0x00ca6620, &args![0u32]).f32(), 1.0);
        let havok = e.mem.alloc(0xb0);
        e.mem.set_f32(havok + 0x80, -2.5);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        assert_eq!(e.call(0x00ca6620, &args![this]).f32(), -2.5);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(havok + 0x80, 0x7f80_0001);
        let radius = e.call(0x00ca6620, &args![this]).f32();
        assert_eq!(radius.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn constructor_function_builds_a_shape_or_returns_null() {
        let mut e = engine();
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| ret(0));
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 0xc, 0x77);
        e.mem.set_u32(block + 0x10, 0x88);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [0x14]);
            ret(block)
        });
        e.mem.set_u32(SERIALIZABLE_COUNT, 1);
        e.mem.set_u32(SHAPE_COUNT, 2);
        e.mem.set_u32(OBJECT_COUNT, 3);
        let made = e.call(0x00ca6670, &args![]).u32();
        assert_eq!(made, block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(e.mem.u32(SHAPE_COUNT), 3);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR), vec![vec![block]]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ca6670, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR).len(), 1);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
    }

    #[test]
    fn copy_members_scales_the_four_floats_of_the_cinfo() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0x40);
        for (offset, value) in [(0x20, 1.0f32), (0x24, 2.0), (0x28, 3.0), (0x2c, 4.0)] {
            e.mem.set_f32(cinfo + offset, value);
        }
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        e.register(SHAPE_COPY_MEMBERS, |_, _| ret(0));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 2.5);
        e.call(0x00ca6720, &args![this, 0x5000u32, process]);
        let scaled: Vec<f32> = [0x20, 0x24, 0x28, 0x2c]
            .iter()
            .map(|offset| e.mem.f32(cinfo + offset))
            .collect();
        assert_eq!(scaled, [2.5, 5.0, 7.5, 10.0]);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this.addr(), 0x5000, process]]
        );
    }

    #[test]
    fn load_c_info_reads_the_cinfo_and_swaps_new_streams() {
        // (version, byte at +0x1e5, expected size read, whether the words are swapped)
        for (version, native, size, swapped) in [
            (3, 0u8, 0x30u32, false),
            (4, 1, 0x40, false),
            (4, 0, 0x40, true),
            (7, 0, 0x40, true),
        ] {
            let mut e = engine();
            let cinfo = e.mem.alloc(0x40);
            for offset in [0x20, 0x24, 0x28, 0x2c] {
                e.mem.set_u32(cinfo + offset, 0x1122_3344);
            }
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            e.register(0x7000_00a8, |_, _| ret(0x40));
            e.register(0x7000_0008, |_, _| ret(0x2c));
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4), (0xa8, 0x7000_00a8)]);
            let (stream, io) = stream_fixture(&mut e, version, 0x7000_0008, 0);
            e.mem.set_u8(stream + 0x1e5, native);
            let result = e.call(0x00ca6780, &args![this, stream]).u32();
            assert_eq!(result, size);
            assert_eq!(calls_to(&e, 0x7000_0008), vec![vec![io, cinfo, size, 0, 0]]);
            assert_eq!(e.mem.u32(io + 4), 10 + 0x2c);
            let expected = if swapped { 0x4433_2211 } else { 0x1122_3344 };
            assert_eq!(e.mem.u32(cinfo + 0x20), expected);
            assert_eq!(e.mem.u32(cinfo + 0x2c), expected);
        }
    }

    #[test]
    fn register_streamables_visits_every_part_then_the_base() {
        let mut e = engine();
        e.register(0x7000_0054, |_, _| ret(0));
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| ret(1));
        let first = object_with_slots(&mut e, 0x20, &[(0x54, 0x7000_0054)]);
        let second = object_with_slots(&mut e, 0x20, &[(0x54, 0x7000_0054)]);
        let havok = havok_with_parts(&mut e, &[(first, 0), (0, 0), (second, 0)]);
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        assert!(e.call(0x00ca6810, &args![this, 0x1234u32]).bool());
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00ca6810,
                0x7000_0054,
                0x7000_0054,
                OBJECT_REGISTER_STREAMABLES
            ]
        );
        let visited: Vec<u32> = calls_to(&e, 0x7000_0054).iter().map(|c| c[0]).collect();
        assert_eq!(visited, [first, second]);

        // No Havok object: only the base runs.
        let bare = shape(&mut e);
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| ret(0));
        assert!(!e.call(0x00ca6810, &args![bare, 0x1234u32]).bool());
        assert_eq!(calls_to(&e, 0x7000_0054).len(), 2);
    }

    #[test]
    fn is_equal_compares_the_part_counts_after_the_base() {
        let mut e = engine();
        e.register(SHAPE_IS_EQUAL, |_, _| ret(0));
        let this = shape(&mut e);
        let other = shape(&mut e);
        assert!(!e.call(0x00ca6880, &args![this, other]).bool());

        e.register(SHAPE_IS_EQUAL, |_, _| ret(1));
        // Neither has a Havok object: zero parts each.
        assert!(e.call(0x00ca6880, &args![this, other]).bool());
        let three = havok_with_parts(&mut e, &[(0, 0), (0, 0), (0, 0)]);
        let two = havok_with_parts(&mut e, &[(0, 0), (0, 0)]);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(three));
        // The other has no Havok object: equal only when this has no parts.
        assert!(!e.call(0x00ca6880, &args![this, other]).bool());
        e.set(other, BhkNiTriStripsShape::phkObject, Ptr::new(two));
        assert!(!e.call(0x00ca6880, &args![this, other]).bool());
        e.set(other, BhkNiTriStripsShape::phkObject, Ptr::new(three));
        assert!(e.call(0x00ca6880, &args![this, other]).bool());
        // A null other counts as having no Havok object.
        assert!(!e.call(0x00ca6880, &args![this, 0u32]).bool());
        let bare = shape(&mut e);
        assert!(e.call(0x00ca6880, &args![bare, 0u32]).bool());
    }

    #[test]
    fn sub_material_comes_from_the_shape_or_from_a_part() {
        let mut e = engine();
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::eMaterial, 0x25);
        assert_eq!(e.call(0x00ca68d0, &args![this, 0xffff_ffffu32]).u32(), 5);
        let with_material = strips_data(&mut e, 7, 0x3);
        let havok = havok_with_parts(&mut e, &[(0, 0), (0, 0), (with_material, 0)]);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        assert_eq!(e.call(0x00ca68d0, &args![this, 2u32 << 20]).u32(), 7);
        // A part without strips data, and the empty part of a shape without a Havok object.
        assert_eq!(e.call(0x00ca68d0, &args![this, 1u32 << 20]).u32(), 0);
        let bare = shape(&mut e);
        assert_eq!(e.call(0x00ca68d0, &args![bare, 3u32 << 20]).u32(), 0);
    }

    #[test]
    fn sub_material_var_type_tests_bits_of_the_shape_or_of_a_part() {
        let mut e = engine();
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::eMaterial, 0x6);
        assert!(e
            .call(0x00ca6920, &args![this, 0xffff_ffffu32, 0x2u32])
            .bool());
        assert!(!e
            .call(0x00ca6920, &args![this, 0xffff_ffffu32, 0x1u32])
            .bool());
        // Bit 5 of the low byte, bit 11 of the word.
        let bit5 = strips_data(&mut e, 0, 0x20);
        let bit11 = e.mem.alloc(0x20);
        e.mem.set_u16(bit11 + 0xc, 0x800);
        let havok = havok_with_parts(&mut e, &[(bit5, 0), (bit11, 0), (0, 0)]);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        let ask = |e: &mut Engine, part: u32, mask: u32| {
            e.call(0x00ca6920, &args![this, part << 20, mask]).bool()
        };
        assert!(ask(&mut e, 0, 0x20));
        assert!(!ask(&mut e, 0, 0x40));
        assert!(ask(&mut e, 1, 0x40));
        assert!(!ask(&mut e, 1, 0x20));
        // Another mask, and a part without strips data.
        assert!(!ask(&mut e, 0, 0x10));
        assert!(!ask(&mut e, 2, 0x20));
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
        let clone = e.call(0x00ca6990, &args![this, 0x6000u32]).u32();
        assert_eq!(clone, block);
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
                0x00ca6990,
                CLONE_LOCK,
                OPERATOR_NEW,
                REF_OBJECT_CONSTRUCTOR,
                0x7000_00c4,
                CLONE_UNLOCK
            ]
        );

        // A failed allocation still runs CopyMembers, with a null shape.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00ca6990, &args![this, 0x6000u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x7000_00c4)[1], [this.addr(), 0, 0x6000]);
    }

    #[test]
    fn init_builds_the_havok_shape_in_an_aligned_block() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let raw = e.mem.alloc(0xe0) + 3;
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, [MEMORY_MANAGER, 0xc0]);
            ret(raw)
        });
        e.register(MESH_SHAPE_CONSTRUCT, |_, _| ret(0x7777));
        e.register(0x7000_008c, |_, _| ret(0));
        e.register(HAVOK_RELEASE, |_, _| ret(0));
        let seen = Rc::new(RefCell::new(0));
        let during = seen.clone();
        e.register_double(0x7000_00bc, move |e, _| {
            *during.borrow_mut() = e.mem.u32(word);
            ret(0)
        });
        let this = shape_with_slots(&mut e, &[(0x8c, 0x7000_008c), (0xbc, 0x7000_00bc)]);
        e.call(0x00ca6a80, &args![this, 0x4242u32]);
        let block = raw + (0x10 - (raw & 0xf));
        assert_eq!(block & 0xf, 0);
        assert_eq!(e.mem.u8(block - 1), (0x10 - (raw & 0xf)) as u8);
        assert_eq!(
            calls_to(&e, MESH_SHAPE_CONSTRUCT),
            vec![vec![block, 0x4242]]
        );
        assert_eq!(calls_to(&e, 0x7000_008c), vec![vec![this.addr(), 0x7777]]);
        assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![0x7777]]);
        assert_eq!(calls_to(&e, 0x7000_00bc), vec![vec![this.addr(), 0x4242]]);
        assert_eq!(*seen.borrow(), 0x10);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    /// A mapped fixed-string block: the reference count at +0, the handle at +8.
    const STRING_BLOCK: u32 = 0x7800_0000;
    const STRING_HANDLE: u32 = STRING_BLOCK + 8;

    #[test]
    fn build_display_geometry_attaches_a_node_for_each_strips_part() {
        let mut e = engine();
        e.map(STRING_BLOCK, 0x20);
        e.mem.set_u32(STRING_BLOCK, 5);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        // Class chains: the first part's second class is NiTriStrips, the other's is not.
        let strips_class = e.mem.alloc(8);
        let outer = e.mem.alloc(8);
        e.mem.set_u32(strips_class + 4, NI_TRI_STRIPS_RTTI);
        e.mem.set_u32(outer + 4, strips_class);
        let other_class = e.mem.alloc(8);
        e.register_double(0x7000_0008, move |e, a| {
            ret(if e.mem.u32(a[0] + 0x1c) == 1 {
                outer
            } else {
                other_class
            })
        });
        let strips = object_with_slots(&mut e, 0x40, &[(8, 0x7000_0008)]);
        e.mem.set_u32(strips + 0x1c, 1);
        e.mem.set_u16(strips + 0xc, 5 << 6);
        let plain = object_with_slots(&mut e, 0x40, &[(8, 0x7000_0008)]);
        let havok = havok_with_parts(&mut e, &[(strips, 0), (plain, 0), (0, 0)]);
        e.mem.set_f32(havok + 0x80, -2.0);
        let display = e.mem.alloc(0xd0);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(NI_TRI_STRIPS_CONSTRUCT, move |_, _| ret(display));
        e.register(FIXED_STRING_CREATE, |_, _| ret(STRING_HANDLE));
        e.register(SET_NAME, |_, _| ret(0));
        e.register(PREPARE_DEBUG_GEOMETRY, |_, _| ret(0));
        e.register(0x7000_00dc, |_, _| ret(0));
        let node = object_with_slots(&mut e, 0x40, &[(0xdc, 0x7000_00dc)]);
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));

        e.call(0x00ca6b60, &args![this, node, 0x99u32]);

        assert_eq!(calls_to(&e, NI_TRI_STRIPS_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, NI_TRI_STRIPS_CONSTRUCT)[0][1], strips);
        assert_eq!(calls_to(&e, FIXED_STRING_CREATE), vec![vec![DISPLAY_NAME]]);
        assert_eq!(calls_to(&e, SET_NAME)[0][0], display);
        // The scale is the absolute radius.
        assert_eq!(e.mem.f32(display + 0x64), 2.0);
        assert_eq!(
            calls_to(&e, PREPARE_DEBUG_GEOMETRY),
            vec![vec![display, node, 5, 0x99]]
        );
        assert_eq!(calls_to(&e, 0x7000_00dc), vec![vec![node, display, 1]]);
        // The name handle is released (handle - 8).
        assert_eq!(calls_to(&e, 0x00fd_f1c4), vec![vec![STRING_BLOCK]]);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn build_display_geometry_does_nothing_without_parts() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let this = shape(&mut e);
        e.call(0x00ca6b60, &args![this, 0x1000u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn create_cinfo_makes_the_info_once_and_fills_it_from_the_havok_object() {
        let mut e = engine();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let raw = e.mem.alloc(0x60) + 5;
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, [MEMORY_MANAGER, 0x40]);
            ret(raw)
        });
        e.register(SHAPE_COPY_CINFO, |_, _| ret(0));
        e.register(MESH_SHAPE_COPY_SCALE, |_, _| ret(0));
        let this = shape(&mut e);
        let havok = e.mem.alloc(0x20);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 0xee);

        let info = e.call(0x00ca6d40, &args![this, created]).u32();

        let shift = 0x10 - (raw & 0xf);
        let block = raw + shift;
        assert_eq!(info, block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u8(block - 1), shift as u8);
        assert_eq!(e.get(this, BhkNiTriStripsShape::pInfo).addr(), block);
        assert_eq!(e.mem.u32(block + 8), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(block + 0x1c), 1);
        assert_eq!(e.mem.f32(block + 4), 0.0);
        assert_eq!(e.mem.f32(block + 0x20), 1.0);
        assert_eq!(e.mem.f32(block + 0x28), 1.0);
        assert_eq!(e.mem.f32(block + 0x2c), 0.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_CINFO),
            vec![vec![this.addr(), block]]
        );
        assert_eq!(
            calls_to(&e, MESH_SHAPE_COPY_SCALE),
            vec![vec![havok, block]]
        );
        assert_eq!(e.mem.u32(word), 0x33);

        // The second call finds the info and allocates nothing.
        let again = e.call(0x00ca6d40, &args![this, created]).u32();
        assert_eq!(again, block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
    }

    #[test]
    fn create_cinfo_without_a_havok_object_only_builds_the_block() {
        let mut e = engine();
        let raw = e.mem.alloc(0x60);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(raw));
        let this = shape(&mut e);
        let created = e.mem.alloc(4);
        let info = e.call(0x00ca6d40, &args![this, created]).u32();
        assert_eq!(info, raw + 0x10 - (raw & 0xf));
        assert!(calls_to(&e, SHAPE_COPY_CINFO).is_empty());
    }

    #[test]
    fn create_cinfo_resets_the_scale_when_copying_removed_the_havok_object() {
        let mut e = engine();
        let raw = e.mem.alloc(0x60);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(raw));
        e.register(SHAPE_COPY_CINFO, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_f32(a[1] + 0x24, 5.0);
            ret(0)
        });
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(0x4444));
        let created = e.mem.alloc(4);
        let info = e.call(0x00ca6d40, &args![this, created]).u32();
        assert_eq!(e.mem.f32(info + 0x24), 1.0);
        assert!(calls_to(&e, MESH_SHAPE_COPY_SCALE).is_empty());
    }

    #[test]
    fn kill_cinfo_frees_the_info_only_when_asked() {
        let mut e = engine();
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        e.register(MEMORY_MANAGER_FREE, |_, _| ret(0));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        let block = e.mem.alloc(0x60);
        let cinfo = block + 9;
        e.mem.set_u8(cinfo - 1, 9);
        e.mem.set_u32(cinfo + 0xc, 0x5151);
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::pInfo, Ptr::new(cinfo));

        // The low byte decides.
        e.call(0x00ca6e60, &args![this, 0x100u32]);
        assert_eq!(e.get(this, BhkNiTriStripsShape::pInfo).addr(), cinfo);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());

        e.call(0x00ca6e60, &args![this, 1u32]);
        assert!(e.get(this, BhkNiTriStripsShape::pInfo).is_null());
        assert_eq!(e.mem.u32(cinfo + 8), FREED_CINFO_VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x5151]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(e.mem.u32(word), 0x33);

        // Nothing to free: only the pointer is cleared.
        e.call(0x00ca6e60, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
    }

    /// Doubles for the viewer-string array: `SetAt` stores the text at the index and bumps the
    /// size and count the way the real array does.
    fn install_string_array_doubles(e: &mut Engine) {
        e.register(STRINGS_GROW, |_, _| ret(0));
        e.register(0x0096_ae90, |e, a| {
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
    fn viewer_strings_list_the_first_five_and_the_last_three_parts() {
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
        let parts: Vec<(u32, u32)> = (0..9u16)
            .map(|index| (strips_data(&mut e, index, 0), 0xa0 + u32::from(index)))
            .collect();
        let havok = havok_with_parts(&mut e, &parts);
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        let strings = strings_fixture(&mut e, 40);

        e.call(0x00ca6f10, &args![this, strings]);

        let mut expected = vec![0xa000, NAME_SUBPARTS + 9];
        for index in [0u32, 1, 2, 3, 4, 8, 7, 6] {
            expected.push(NAME_PART + index);
            expected.push(NAME_MATERIAL + index + 0x1000);
        }
        assert_eq!(string_data(&e, strings), expected);
        assert_eq!(e.mem.u16(strings + 0xc), 18);
        // The part strings get the address of a copy of the record's second word and the array.
        assert_eq!(
            *part_words.borrow(),
            [0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa8, 0xa7, 0xa6]
        );
        let arrays: Vec<u32> = calls_to(&e, PART_ADD_VIEWER_STRINGS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(arrays, vec![strings; 8]);
        assert_eq!(e.mem.u32(word), 0x33);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
    }

    #[test]
    fn viewer_strings_for_a_small_shape_list_every_part() {
        let mut e = engine();
        install_string_array_doubles(&mut e);
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| ret(0));
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(NI_GET_VIEWER_STRING_NUMBER, |_, a| ret(a[0] + a[1]));
        e.register(NI_GET_VIEWER_STRING_MATERIAL, |_, a| {
            ret(a[0] + a[1] + 0x1000)
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let words = seen.clone();
        e.register_double(PART_ADD_VIEWER_STRINGS, move |e, a| {
            words.borrow_mut().push(e.mem.u32(a[0]));
            ret(0)
        });
        let data = strips_data(&mut e, 2, 0);
        let havok = havok_with_parts(&mut e, &[(data, 0xb1), (0, 0xb2)]);
        let this = shape(&mut e);
        e.set(this, BhkNiTriStripsShape::phkObject, Ptr::new(havok));
        // A capacity of 2 makes the later strings grow the array first; the grow double does
        // nothing, so the data block is big enough up front.
        let strings = strings_fixture(&mut e, 2);
        let room = e.mem.alloc(0x100);
        e.mem.set_u32(strings + 4, room);

        e.call(0x00ca6f10, &args![this, strings]);

        assert_eq!(
            string_data(&e, strings),
            [
                0xa000,
                NAME_SUBPARTS + 2,
                NAME_PART,
                NAME_MATERIAL + 2 + 0x1000,
                NAME_PART + 1,
                NAME_MATERIAL + 0x1000
            ]
        );
        assert_eq!(*seen.borrow(), [0xb1, 0xb2]);
        assert_eq!(calls_to(&e, STRINGS_GROW).len(), 4);
    }

    #[test]
    fn inline_set_at_appends_and_counts_non_null_text() {
        let mut e = engine();
        e.register(STRINGS_GROW, |_, _| ret(0));
        let strings = strings_fixture(&mut e, 8);
        set_viewer_string_inline(&mut e, strings, 0x11);
        set_viewer_string_inline(&mut e, strings, 0);
        set_viewer_string_inline(&mut e, strings, 0x33);
        assert_eq!(string_data(&e, strings), [0x11, 0, 0x33]);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
    }

    #[test]
    fn inline_set_at_replaces_when_growing_raised_the_size() {
        // (old text, new text, count before, count after): the grow call raises the size past the
        // index, so the element is replaced and the count follows the null-ness of both.
        for (old, new, before, after) in [
            (0u32, 0x44u32, 1u16, 2u16),
            (0x55, 0x44, 1, 1),
            (0x55, 0, 1, 0),
            (0, 0, 1, 1),
        ] {
            let mut e = engine();
            e.register(STRINGS_GROW, |e, a| {
                e.mem.set_u16(a[0] + 0xa, 3);
                ret(0)
            });
            let strings = strings_fixture(&mut e, 1);
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(strings + 4, data);
            e.mem.set_u16(strings + 0xa, 1);
            e.mem.set_u16(strings + 0xc, before);
            e.mem.set_u32(data + 4, old);
            set_viewer_string_inline(&mut e, strings, new);
            assert_eq!(e.mem.u32(data + 4), new);
            assert_eq!(e.mem.u16(strings + 0xc), after);
            assert_eq!(e.mem.u16(strings + 0xa), 3);
            // The grow step (4) plus the old size.
            assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        }
    }

    #[test]
    fn mesh_block_initialiser_and_destructor_work_on_twelve_arrays() {
        let mut e = engine();
        let block = e.mem.alloc(0xc0);
        for offset in (0..0xb4).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        e.call(0x00ca7310, &args![block]);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0xdead_beef);
        assert_eq!(e.mem.u32(block + 0x20), 0xdead_beef);
        for offset in [0x10, 0x14, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
        for k in 0..12 {
            let array = block + 0x24 + 12 * k;
            assert_eq!(
                (e.mem.u32(array), e.mem.u32(array + 4), e.mem.u32(array + 8)),
                (0, 0, 0x8000_0000)
            );
        }
    }

    #[test]
    fn mesh_block_destructor_frees_owned_arrays_last_first() {
        let mut e = engine();
        let allocator = object_with_slots(&mut e, 0x20, &[(0x10, 0x7000_0010)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register(0x7000_0010, |_, _| ret(0));
        let block = e.mem.alloc(0xc0);
        e.call(0x00ca7310, &args![block]);
        // Array 0 owned with capacity 3 (words), array 4 owned with capacity 5 (bytes), array 7
        // with the not-owned bit and a pointer that must not be freed, array 11 owned.
        for (k, data, size, capacity) in [
            (0u32, 0x1000u32, 2u32, 3u32),
            (4, 0x2000, 1, 5),
            (7, 0x3000, 1, 0x8000_0002),
            (11, 0x4000, 4, 0x4000_0006),
        ] {
            let array = block + 0x24 + 12 * k;
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 4, size);
            e.mem.set_u32(array + 8, capacity);
        }
        e.call(0x00ca73c0, &args![block]);
        let freed: Vec<Words> = calls_to(&e, 0x7000_0010);
        // Last to first; the high bits of the capacity are not part of the size, which is in
        // bytes (array 4 holds bytes, the others words).
        assert_eq!(
            freed,
            vec![
                vec![allocator, 0x4000, 6 * 4],
                vec![allocator, 0x2000, 5],
                vec![allocator, 0x1000, 12]
            ]
        );
        assert_eq!(calls_to(&e, MEMORY_ROUTER).len(), 12);
        for k in 0..12 {
            let array = block + 0x24 + 12 * k;
            assert_eq!(
                (e.mem.u32(array), e.mem.u32(array + 4), e.mem.u32(array + 8)),
                (0, 0, 0x8000_0000)
            );
        }
    }

    /// Hands out link objects in order for `GetObjectFromLinkID`.
    fn install_link_objects(e: &mut Engine, objects: &[u32]) {
        let objects = RefCell::new(Vec::from(objects).into_iter());
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(objects.borrow_mut().next().unwrap())
        });
        e.register(OBJECT_LINKED, |_, _| ret(0));
        e.register(SHAPE_LINK_OBJECT, |_, _| ret(0));
        e.register(STREAM_NUMBER_OF_LINK_IDS, |_, _| ret(2));
    }

    #[test]
    fn link_object_of_an_old_stream_grows_the_entry_array_first() {
        let mut e = engine();
        install_entry_array_doubles(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        install_link_objects(&mut e, &[first, second]);
        let cinfo = cinfo_fixture(&mut e);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        let (stream, _) = stream_fixture(&mut e, 1, 0, 0);

        e.call(0x00ca76b0, &args![this, stream]);

        // SetSize(array, 2) once, then the zero entries are appended: size 0 < capacity 2.
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![cinfo + 8, 2]]);
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 2);
        let data = e.mem.u32(cinfo + 0xc);
        assert_eq!(e.mem.u32(data), first);
        assert_eq!(e.mem.u32(data + 8), second);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(calls_to(&e, OBJECT_LINKED), vec![vec![first], vec![second]]);
        assert_eq!(
            calls_to(&e, SHAPE_LINK_OBJECT),
            vec![vec![this.addr(), stream]]
        );
    }

    #[test]
    fn link_object_of_a_new_stream_replaces_the_entries_as_pointers() {
        let mut e = engine();
        e.register(0x7000_0004, |_, _| ret(0));
        let old = object_with_slots(&mut e, 0x20, &[(4, 0x7000_0004)]);
        e.mem.set_u32(old + 4, 1);
        let keep = e.mem.alloc(0x20);
        e.mem.set_u32(keep + 4, 1);
        let fresh = e.mem.alloc(0x20);
        install_link_objects(&mut e, &[fresh, keep]);
        let cinfo = cinfo_fixture(&mut e);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 0xc, data);
        e.mem.set_u32(data, old);
        e.mem.set_u32(data + 8, keep);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        let (stream, _) = stream_fixture(&mut e, 2, 0, 0);

        e.call(0x00ca76b0, &args![this, stream]);

        // Version 2: the array is not touched. The old object loses its last reference; the
        // second entry already held the object, so nothing changes for it.
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(e.mem.u32(data), fresh);
        assert_eq!(e.mem.u32(fresh + 4), 1);
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(data + 8), keep);
        assert_eq!(e.mem.u32(keep + 4), 1);
    }

    #[test]
    fn save_binary_writes_the_links_then_the_words_and_clears_the_info() {
        for native in [1u8, 0] {
            let mut e = engine();
            install_entry_array_doubles(&mut e);
            e.register(ARRAY_CLEAR, |_, _| ret(0));
            e.register(OPERATOR_DELETE, |_, _| ret(0));
            e.register(0x7000_0030, |_, _| ret(0));
            e.register(0x7000_00a4, |_, _| ret(0));
            let word = e.tls() + 0x2b4;
            e.mem.set_u32(word, 0x33);
            let cinfo = cinfo_fixture(&mut e);
            let data = e.mem.alloc(0x20);
            e.mem.set_u32(cinfo + 0xc, data);
            e.mem.set_u32(cinfo + 0x14, 2);
            for (index, (object, value)) in
                [(0x1010u32, 0xa1u32), (0x2020, 0xb2)].iter().enumerate()
            {
                e.mem.set_u32(data + 8 * index as u32, *object);
                e.mem.set_u32(data + 8 * index as u32 + 4, *value);
            }
            e.mem.set_u32(cinfo + 0x20, 0x1122_3344);
            e.mem.set_u32(cinfo + 4, 0x5566_7788);
            e.register_double(0x7000_00b4, move |e, a| {
                e.mem.set_u8(a[1], 1);
                ret(cinfo)
            });
            // The base SaveBinary sees the words as they are written to the file.
            let seen = Rc::new(RefCell::new((0, 0)));
            let during = seen.clone();
            e.register_double(SHAPE_SAVE_BINARY, move |e, _| {
                *during.borrow_mut() = (e.mem.u32(cinfo + 4), e.mem.u32(cinfo + 0x20));
                ret(0)
            });
            let written = Rc::new(RefCell::new(Vec::new()));
            let log = written.clone();
            e.register_double(0x7000_000c, move |e, a| {
                log.borrow_mut().push(e.mem.u32(a[1]));
                ret(4)
            });
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4), (0xa4, 0x7000_00a4)]);
            let (stream, io) = stream_fixture(&mut e, 5, 0, 0x7000_000c);
            e.mem.set_u8(stream + 0x1e4, native);

            e.call(0x00ca77f0, &args![this, stream]);

            let swapped = if native == 1 {
                (0x3dcc_cccd, 0x1122_3344)
            } else {
                (0xcdcc_cc3d, 0x4433_2211)
            };
            assert_eq!(*seen.borrow(), swapped);
            // After the save the words are back and the float is the constant.
            assert_eq!(e.mem.u32(cinfo + 0x20), 0x1122_3344);
            assert_eq!(*written.borrow(), [2, 2, 0xa1, 0xb2]);
            assert_eq!(e.mem.u32(io + 4), 10 + 16);
            let links: Vec<Words> = calls_to(&e, 0x7000_0030);
            assert_eq!(links, vec![vec![stream, 0x1010], vec![stream, 0x2020]]);
            assert_eq!(calls_to(&e, ARRAY_CLEAR), vec![vec![cinfo + 8]]);
            assert_eq!(
                calls_to(&e, ARRAY_SET_SIZE).last().unwrap(),
                &vec![cinfo + 8, 0]
            );
            assert_eq!(calls_to(&e, 0x7000_00a4), vec![vec![this.addr(), 1]]);
            // The local array's data is freed once.
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
            assert_eq!(e.mem.u32(word), 0x33);
        }
    }

    #[test]
    fn save_binary_of_an_empty_info_writes_two_zero_counts() {
        let mut e = engine();
        install_entry_array_doubles(&mut e);
        e.register(ARRAY_CLEAR, |_, _| ret(0));
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        e.register(SHAPE_SAVE_BINARY, |_, _| ret(0));
        e.register(0x7000_00a4, |_, _| ret(0));
        let cinfo = cinfo_fixture(&mut e);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let written = Rc::new(RefCell::new(Vec::new()));
        let log = written.clone();
        e.register_double(0x7000_000c, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            ret(4)
        });
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4), (0xa4, 0x7000_00a4)]);
        let (stream, _) = stream_fixture(&mut e, 5, 0, 0x7000_000c);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00ca77f0, &args![this, stream]);
        assert_eq!(*written.borrow(), [0, 0]);
        assert!(calls_to(&e, 0x7000_0030).is_empty());
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0]]);
    }

    /// Doubles for the logging path of the old-format load: the stream writer concatenates the
    /// texts it is given into the buffer string kept in a shared cell.
    fn install_logging_doubles(e: &mut Engine) -> Rc<RefCell<Vec<String>>> {
        let texts = Rc::new(RefCell::new(Vec::new()));
        e.register(OSTREAM_CONSTRUCT, |_, _| ret(0));
        e.register(OSTREAM_DESTRUCT, |_, _| ret(0));
        let log = texts.clone();
        e.register_double(OSTREAM_WRITE_TEXT, move |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            log.borrow_mut().push(text);
            ret(a[0])
        });
        texts
    }

    #[test]
    fn load_binary_of_an_old_stream_warns_and_reads_the_old_mesh_block() {
        for native in [1u8, 0] {
            let mut e = engine();
            let texts = install_logging_doubles(&mut e);
            e.register(OBJECT_LOAD_BINARY, |_, _| ret(0));
            e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| ret(0));
            // The warning goes to the logger.
            let logger = object_with_slots(&mut e, 0x20, &[(0xc, 0x7000_000c)]);
            e.mem.set_u32(LOGGER, logger);
            e.register(0x7000_000c, |_, _| ret(0));
            // Memory for the texts the exe has at fixed addresses.
            e.map(0x010c_7800, 0x100);
            e.mem.set_cstr(WARNING_FILE, b"File ");
            e.mem
                .set_cstr(WARNING_MIDDLE, b" contains an old bhkpMeshShape! ");
            e.mem.set_cstr(WARNING_END, b" re-export\n");
            e.map(WARNING_DEFAULT_NAME & !0xf, 0x20);
            e.mem.set_cstr(WARNING_DEFAULT_NAME, b"Please");
            // The Havok allocator for the arrays the loader empties.
            let allocator = object_with_slots(&mut e, 0x20, &[(0x10, 0x7000_0010)]);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
            e.register(0x7000_0010, |_, _| ret(0));
            // The read function fills the 0xc0-byte block.
            e.register(0x7000_0008, |e, a| {
                assert_eq!((a[2], a[3], a[4]), (0xc0, 0, 0));
                e.mem.set_u32(a[1], 0x0500_0000);
                e.mem.set_u32(a[1] + 4, 0x0000_003f);
                for (index, offset) in [0x10u32, 0x14, 0x18, 0x1c].iter().enumerate() {
                    e.mem
                        .set_u32(a[1] + offset, 0x0100_0000 * (index as u32 + 1));
                }
                // Array 0 owned (capacity 2).
                e.mem.set_u32(a[1] + 0x24, 0x1000);
                e.mem.set_u32(a[1] + 0x28, 1);
                e.mem.set_u32(a[1] + 0x2c, 2);
                ret(0xc0)
            });
            let cinfo = cinfo_fixture(&mut e);
            e.mem.set_u32(cinfo, 0x99);
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            e.set(this, BhkNiTriStripsShape::eMaterial, 0x77);
            let (stream, io) = stream_fixture(&mut e, 1, 0x7000_0008, 0);
            e.mem.set_u8(stream + 0x1e5, native);
            e.mem.set_cstr(stream + 0xe0, b"meshes\\a.nif");

            e.call(0x00ca7a70, &args![this, stream]);

            assert_eq!(
                *texts.borrow(),
                [
                    "File ",
                    "meshes\\a.nif",
                    " contains an old bhkpMeshShape! ",
                    "Please",
                    " re-export\n"
                ]
            );
            let report = calls_to(&e, 0x7000_000c);
            assert_eq!(report.len(), 1);
            assert_eq!((report[0][1], report[0][2]), (1, 0x234f_2250));
            assert_eq!((report[0][4], report[0][5]), (WARNING_SOURCE, 0xaa));
            assert_eq!(
                calls_to(&e, OBJECT_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(e.mem.u32(io + 4), 10 + 0xc0);
            // The material word 5 (0x0500_0000 swapped when the stream is not native).
            let material = if native == 1 {
                0x0500_0000u32 as i32
            } else {
                5
            };
            let expected_material = if material >= 0x24 { 0 } else { material as u32 };
            assert_eq!(
                e.get(this, BhkNiTriStripsShape::eMaterial),
                expected_material
            );
            // The info: first word cleared and the float constant stored.
            assert_eq!(e.mem.u32(cinfo), 0);
            assert_eq!(e.mem.u32(cinfo + 4), 0x3dcc_cccd);
            assert_eq!(
                calls_to(&e, STREAM_READ_MULTIPLE_LINK_IDS),
                vec![vec![stream]]
            );
            // The arrays were emptied twice (after the read, then by the destructor); only the
            // first pass found an owned one.
            assert_eq!(calls_to(&e, 0x7000_0010), vec![vec![allocator, 0x1000, 8]]);
            assert_eq!(calls_to(&e, MEMORY_ROUTER).len(), 24);
        }
    }

    #[test]
    fn load_binary_of_an_old_stream_with_a_file_name_uses_it() {
        let mut e = engine();
        let texts = install_logging_doubles(&mut e);
        e.register(OBJECT_LOAD_BINARY, |_, _| ret(0));
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| ret(0));
        let logger = object_with_slots(&mut e, 0x20, &[(0xc, 0x7000_000c)]);
        e.mem.set_u32(LOGGER, logger);
        e.register(0x7000_000c, |_, _| ret(0));
        e.map(0x010c_7800, 0x100);
        e.mem.set_cstr(WARNING_FILE, b"File ");
        e.mem
            .set_cstr(WARNING_MIDDLE, b" contains an old bhkpMeshShape! ");
        e.mem.set_cstr(WARNING_END, b" re-export\n");
        let router = e.mem.alloc(0x20);
        let allocator = object_with_slots(&mut e, 0x20, &[]);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        // A read of the material word 0x30 (>= 0x24) gives material 0.
        e.register(0x7000_0008, |e, a| {
            e.mem.set_u32(a[1], 0x30);
            ret(0xc0)
        });
        let cinfo = cinfo_fixture(&mut e);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        e.set(this, BhkNiTriStripsShape::eMaterial, 0x77);
        let (stream, _) = stream_fixture(&mut e, 0, 0x7000_0008, 0);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.mem.set_cstr(stream + 8, b"scene");
        e.mem.set_cstr(stream + 0xe0, b"x.nif");

        e.call(0x00ca7a70, &args![this, stream]);

        assert_eq!(texts.borrow()[3], "scene");
        assert_eq!(e.get(this, BhkNiTriStripsShape::eMaterial), 0);
    }

    #[test]
    fn load_binary_of_a_new_stream_appends_the_entries() {
        // (stream version, flag word read, expected stored word)
        for (version, read_word, stored) in [(3u32, 0x31u32, 0x8011u32), (9, 0x31, 0x31)] {
            let mut e = engine();
            install_entry_array_doubles(&mut e);
            e.register(SHAPE_LOAD_BINARY, |_, _| ret(0));
            e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| ret(0));
            e.register(OBJECT_LOAD_BINARY, |_, _| ret(0));
            // The first read is the count (2); later reads give the words.
            let reads = Rc::new(RefCell::new(0));
            let calls = reads.clone();
            e.register_double(0x7000_0008, move |e, a| {
                let mut calls = calls.borrow_mut();
                *calls += 1;
                assert_eq!((a[2], a[4]), (4, 1));
                e.mem.set_u32(a[1], if *calls == 1 { 2 } else { read_word });
                ret(4)
            });
            let cinfo = cinfo_fixture(&mut e);
            e.mem.set_u32(cinfo + 4, 0x1122_3344);
            e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
            let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
            let (stream, io) = stream_fixture(&mut e, version, 0x7000_0008, 0);
            e.mem.set_u8(stream + 0x1e5, 0);

            e.call(0x00ca7a70, &args![this, stream]);

            assert_eq!(
                calls_to(&e, SHAPE_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(
                calls_to(&e, STREAM_READ_MULTIPLE_LINK_IDS),
                vec![vec![stream]]
            );
            assert_eq!(e.mem.u32(io + 4), 10 + 12);
            // The float at +4 of the info is byte-swapped for a stream that is not native.
            assert_eq!(e.mem.u32(cinfo + 4), 0x4433_2211);
            assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 2);
            let data = e.mem.u32(cinfo + 0xc);
            assert_eq!(e.mem.u32(data), 0);
            assert_eq!(e.mem.u32(data + 4), stored);
            assert_eq!(e.mem.u32(data + 12), stored);
        }
    }

    #[test]
    fn load_binary_of_a_new_stream_without_entries_reads_only_the_count() {
        let mut e = engine();
        install_entry_array_doubles(&mut e);
        e.register(SHAPE_LOAD_BINARY, |_, _| ret(0));
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| ret(0));
        e.register(0x7000_0008, |e, a| {
            e.mem.set_u32(a[1], 0);
            ret(4)
        });
        let cinfo = cinfo_fixture(&mut e);
        e.register_double(0x7000_00b4, move |_, _| ret(cinfo));
        let this = shape_with_slots(&mut e, &[(0xb4, 0x7000_00b4)]);
        let (stream, _) = stream_fixture(&mut e, 2, 0x7000_0008, 0);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00ca7a70, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x7000_0008).len(), 1);
        assert!(calls_to(&e, ARRAY_SET_AT).is_empty());
    }

    #[test]
    fn bhk_ni_tri_strips_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_6590, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00ca6650, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ca6650, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00ca_6590), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ni_tri_strips_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ca_6600, &args![]).u32(), 0x0126_82e8);
    }
}
