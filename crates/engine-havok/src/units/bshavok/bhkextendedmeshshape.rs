//! `BSHavok/bhkextendedmeshshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkExtendedMeshShape` is a 0x18-byte shape (vtable, the Havok shape at `+8`, its cinfo at
//! `+0xC`, the `bhkExtendedMeshShapeData` at `+0x14`). The data object (0x10 bytes) keeps a
//! count at `+8` and an array of 0x14-byte sub-parts at `+0xC` (the count is stored in the word
//! before the array), each sub-part being `{vertex count, vertices (12 bytes each), triangle
//! count, indices (6 bytes per triangle), 0x18-byte block}`.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    aligned_allocate, enter_scope, havok_allocate, push_viewer_string, release_ni_object,
    stream_transfer, CLONE_LOCK, CLONE_UNLOCK, HEAP_ALLOCATE, INTERLOCKED_INCREMENT,
    MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, NI_GET_VIEWER_STRING, REMOVE_REFERENCE,
    SERIALIZABLE_KILL,
};
use super::bhkhingeconstraint::bump;

/// Base class routines: constructor (`00c87c30`), `LoadBinary` (`00c84d00`), `SaveBinary`
/// (`00c84d50`), `IsEqual` (`00c84dc0`), `CopyMembers` (`00c84df0`, `(this, clone, context)`),
/// `CopyCinfo` (`00c84e20`), the destructor body (`00cd2de0`) and `GetViewerStrings`
/// (`00cd2fd0`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
const BASE_LOAD_BINARY: u32 = 0x00c8_4d00;
const BASE_SAVE_BINARY: u32 = 0x00c8_4d50;
const BASE_IS_EQUAL: u32 = 0x00c8_4dc0;
const BASE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const BASE_COPY_CINFO: u32 = 0x00c8_4e20;
const BASE_DESTRUCT: u32 = 0x00cd_2de0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00cd_2fd0;
/// `RegisterStreamables` of the base (`00aa0ad0`, a jump to `00a81fe0`) and the stream call
/// `LoadBinary` ends with (`00a645b0`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const STREAM_AFTER_LOAD: u32 = 0x00a6_45b0;
/// The Havok shape routine `QSubMaterial` asks (`00d55530`, thiscall on the shape).
const SUB_MATERIAL_LOOKUP: u32 = 0x00d5_5530;
/// The data object's base constructor (`00a5d3a0`) and destructor (`00a5d3d0`).
const DATA_BASE_CONSTRUCT: u32 = 0x00a5_d3a0;
const DATA_BASE_DESTRUCT: u32 = 0x00a5_d3d0;
/// Vtables: shape (`010ca744`), its two base classes (`01030bf4`, `010c44e4`) and the data
/// object (`010ca6b4`); the live counters of the three shape classes.
const SHAPE_VTABLE: u32 = 0x010c_a744;
const SHAPE_BASE_VTABLE_A: u32 = 0x0103_0bf4;
const SHAPE_BASE_VTABLE_B: u32 = 0x010c_44e4;
const DATA_VTABLE: u32 = 0x010c_a6b4;
const COUNT_BASE_A: u32 = 0x0126_80f8;
const COUNT_BASE_B: u32 = 0x0126_86d0;
const COUNT_SHAPE: u32 = 0x0126_8678;
/// `NiRTTI` global of the shape class.
const SHAPE_RTTI: u32 = 0x0126_867c;
/// Havok-side routines `Init` uses: the shape constructor (`00d569c0`, thiscall on a 0xF0-byte
/// block with `(float, 0xC)`), the vector setter (`00d554c0`, one argument) and the float
/// (`011b153c`) and 0xF0 size it is built with; the cinfo block's vector constant is
/// `01012068`.
const HK_SHAPE_CONSTRUCT: u32 = 0x00d5_69c0;
const HK_SHAPE_SET_VECTOR: u32 = 0x00d5_54c0;
const HK_SHAPE_FLOAT: u32 = 0x011b_153c;
const CINFO_VECTOR_CONSTANT: u32 = 0x0101_2068;
/// The sub-part array's per-element constructor (`00e46e80`) and destructor (`00ccbb50`) given
/// to the CRT iterators `00ec782f` (`(array, size, count, constructor, destructor)`) and
/// `00ec5fce` (`(array, size, count, destructor)`).
const SUBPART_CONSTRUCTOR: u32 = 0x00e4_6e80;
const SUBPART_DESTRUCTOR: u32 = 0x00cc_bb50;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The memory manager's `Free` (`00aa4060`, thiscall), the heap free (`00aa10f0`, cdecl) and
/// `memcpy` (`00ec44d0`, cdecl `(destination, source, size)`).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const HEAP_FREE: u32 = 0x00aa_10f0;
const MEMCPY: u32 = 0x00ec_44d0;
/// This unit's own routines the others call: the pointer-wrapper constructor (`00ccbaa0`),
/// `CopyCinfo` (`00ccbce0`), the sub-part `LoadBinary` (`00ccc440`) / `SaveBinary` (`00ccc620`)
/// and `Init` (virtual slot `0xB0`).
const POINTER_WRAPPER_CONSTRUCT: u32 = 0x00cc_baa0;
const COPY_CINFO: u32 = 0x00cc_bce0;
const SUBPART_LOAD: u32 = 0x00cc_c440;
const SUBPART_SAVE: u32 = 0x00cc_c620;
/// The destructor of the pointer wrapper (`00ca5620`, thiscall) and the stack array of 16-bit
/// indices `fn_00ccc980` builds: its vtable (`0109b2d4`) and `SetSize` (`00a5b790`).
const POINTER_WRAPPER_DESTRUCT: u32 = 0x00ca_5620;
const INDEX_ARRAY_VTABLE: u32 = 0x0109_b2d4;
const INDEX_ARRAY_SET_SIZE: u32 = 0x00a5_b790;

/// `a * b` as `MUL` followed by `SETO` / `NEG` / `OR` computes it: 0xFFFFFFFF on overflow.
fn saturating_times(a: u32, b: u32) -> u32 {
    a.saturating_mul(b)
}

/// The `NiPointer` assignment the exe inlines: when the word at `slot` differs from the word
/// at `source`, the old object is released, the new one is stored and gets a reference.
fn assign_pointer(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old != e.mem.u32(source) {
        if old != 0 {
            release_ni_object(e, old);
        }
        let new = e.mem.u32(source);
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

/// `MULPS` on one lane with `first` as the destination operand: a NaN in the first operand
/// wins, then one in the second (a signalling NaN comes out quiet), else the product.
fn mulps_lane(first: u32, second: u32) -> u32 {
    if f32::from_bits(first).is_nan() {
        x87_quiet_bits(first)
    } else if f32::from_bits(second).is_nan() {
        x87_quiet_bits(second)
    } else {
        (f32::from_bits(first) * f32::from_bits(second)).to_bits()
    }
}

/// Constructs a shape object in `block` (when there is one): the base constructor, the three
/// vtables with their live counters (`+0xC` and `+0x10` zeroed after the first) and a zero at
/// `+0x14`. Answers `block`, or 0 when it is 0.
fn construct_shape(e: &mut Engine, block: u32) -> u32 {
    if block == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, SHAPE_BASE_VTABLE_A);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    bump(e, COUNT_BASE_A);
    e.mem.set_u32(block, SHAPE_BASE_VTABLE_B);
    bump(e, COUNT_BASE_B);
    e.mem.set_u32(block, SHAPE_VTABLE);
    e.mem.set_u32(block + 0x14, 0);
    bump(e, COUNT_SHAPE);
    block
}

// Translated from 00ca5940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::LoadBinary` (Xbox PDB): asks virtual slot `0xB4` for the cinfo (with
/// the address of an out byte), runs the base load, then reads through the stream's reader
/// (`stream + 0x24C`, function at `+8`, flag 1, the length cell starting at 4 both times) 4
/// bytes into the cinfo at `+8` and 0x10 bytes at `+0x10`, and ends with `00a645b0` on the
/// stream.
pub fn bhk_extended_mesh_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.with_stack(8, |e, locals| {
        let cinfo = e.vcall(this.addr(), 0xb4, &args![locals.addr() + 3]).u32();
        e.call(BASE_LOAD_BINARY, &args![this, stream]);
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, cinfo + 8, 4, Some(4), 1);
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, cinfo + 0x10, 0x10, Some(4), 1);
    });
    e.call(STREAM_AFTER_LOAD, &args![stream]);
}

// Translated from 00ccb9a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb9a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShapeData` factory (no PDB name): allocates 0x10 bytes from the game heap
/// and, when that worked, runs the base constructor (`00a5d3a0`), installs the data vtable and
/// zeroes `+8` and `+0xC`. Answers the object, or 0 (takes no arguments).
pub fn fn_00ccb9a0(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(DATA_BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, DATA_VTABLE);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    block
}

// Translated from 00ccba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::IsEqual` (Xbox PDB): whether `other` is `this` itself.
pub fn bhk_extended_mesh_shape_data_is_equal(_e: &mut Engine, this: Ptr, other: u32) -> bool {
    other == this.addr()
}

// Translated from 00ccba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::RegisterStreamables` (Xbox PDB): registers the data object (`+0x14`,
/// virtual slot `0x54` with the stream; no null check), then runs the base registration
/// (`00aa0ad0`) and answers its `EAX`.
pub fn bhk_extended_mesh_shape_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    let data = e.mem.u32(this.addr() + 0x14);
    e.vcall(data, 0x54, &args![stream]);
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00ccba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::IsEqual` (Xbox PDB): the base comparison (`00c84dc0`), and when it
/// says equal also whether both shapes hold the same data object (`+0x14`).
pub fn bhk_extended_mesh_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.u32(this.addr() + 0x14) == e.mem.u32(other.addr() + 0x14)
}

// Translated from 00ccba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::QSubMaterial` (Xbox PDB): 0x24 for the index 0xFFFFFFFF; otherwise
/// the word at `+4` of what `00d55530` answers for the index, asked of the Havok shape (`+8`;
/// a null `this` gives a null shape).
pub fn bhk_extended_mesh_shape_q_sub_material(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let shape = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    if index == 0xffff_ffff {
        return 0x24;
    }
    let record = e.call(SUB_MATERIAL_LOOKUP, &args![shape, index]).u32();
    e.mem.u32(record + 4)
}

// Translated from 00ccbaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbaa0 - the FS:[0] exception-unwinding frame is not modeled.
/// Constructor of a small pointer wrapper (no PDB name): zeroes `+0` and `+4`, then assigns
/// `object` to the pointer at `+4` with a reference (the release of the old value, always
/// null here, is kept as the exe has it). Answers `this`.
pub fn fn_00ccbaa0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let wrapper = this.addr();
    e.mem.set_u32(wrapper, 0);
    e.mem.set_u32(wrapper + 4, 0);
    let old = e.mem.u32(wrapper + 4);
    if old != object {
        if old != 0 {
            release_ni_object(e, old);
        }
        e.mem.set_u32(wrapper + 4, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    this
}

// Translated from 00ccbc00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbc00 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShape::~bhkExtendedMeshShape` (Xbox PDB): installs the shape vtable, clears
/// the word at `+0x14` of each of the Havok shape's (`+8`) `+0xB8` entries (0x60 bytes each,
/// from the array at `+0xB4`), runs `bhkSerializable::Kill`, decrements the live counter,
/// releases the data object (`+0x14`, cleared) and runs the base destructor body (`00cd2de0`).
/// The exe's second release of `+0x14` (after it was cleared) is kept.
pub fn bhk_extended_mesh_shape_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, SHAPE_VTABLE);
    let shape = e.mem.u32(me + 8);
    let entries = e.mem.i32(shape + 0xb8);
    if entries > 0 {
        let mut offset = 0u32;
        let mut remaining = entries;
        loop {
            let array = e.mem.u32(shape + 0xb4);
            e.mem
                .set_u32(array.wrapping_add(offset).wrapping_add(0x14), 0);
            offset = offset.wrapping_add(0x60);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_SHAPE);
    e.mem.set_u32(COUNT_SHAPE, count.wrapping_sub(1));
    let held = e.mem.u32(me + 0x14);
    if held != 0 {
        release_ni_object(e, held);
        e.mem.set_u32(me + 0x14, 0);
    }
    let held = e.mem.u32(me + 0x14);
    if held != 0 {
        release_ni_object(e, held);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00ccbce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::CopyCinfo` (Xbox PDB): the base copy (`00c84e20`), then the data
/// object (`+0x14`) assigned to the cinfo's pointer at `+4` (with the release and reference
/// counting), the Havok shape's (`+8`) float at `+0xE4` copied to cinfo `+8` through the x87
/// and its 16 bytes at `+0x80` to cinfo `+0x10`.
pub fn bhk_extended_mesh_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(BASE_COPY_CINFO, &args![this, cinfo]);
    let me = this.addr();
    let shape = if me != 0 { e.mem.u32(me + 8) } else { 0 };
    assign_pointer(e, cinfo + 4, me + 0x14);
    let scale = x87_quiet_bits(e.mem.u32(shape + 0xe4));
    e.mem.set_u32(cinfo + 8, scale);
    for lane in 0..4 {
        let word = e.mem.u32(shape + 0x80 + 4 * lane);
        e.mem.set_u32(cinfo + 0x10 + 4 * lane, word);
    }
}

// Translated from 00ccbd70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbd70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShape` factory (no PDB name): allocates 0x18 bytes from the game heap and
/// constructs a shape in them (`construct_shape`). Answers the object, or 0 (takes no
/// arguments).
pub fn fn_00ccbd70(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    construct_shape(e, block)
}

// Translated from 00ccbe10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbe10 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShape::CreateClone` (Xbox PDB): with the allocation-scope word at 0x10 and
/// the clone lock held, builds a new shape (`construct_shape` on 0x18 heap bytes) and has
/// `this` copy its members into it through virtual slot `0xC4` (`(clone, context)`). Answers
/// the clone (0 when the allocation failed).
pub fn bhk_extended_mesh_shape_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    let clone = construct_shape(e, block);
    e.vcall(this.addr(), 0xc4, &args![clone, context]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ccbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::CopyMembers` (Xbox PDB): asks virtual slot `0xB4` for the cinfo,
/// multiplies the four floats at its `+0x10` by the float at `context + 0x10` (read through
/// the x87; `MULPS` with the scale as destination operand), assigns this shape's data object
/// (`+0x14`) to the clone's, then runs the base copy (`00c84df0`, `(this, clone, context)`).
pub fn bhk_extended_mesh_shape_copy_members(e: &mut Engine, this: Ptr, clone: u32, context: u32) {
    let me = this.addr();
    let cinfo = e.with_stack(4, |e, flag| e.vcall(me, 0xb4, &args![flag]).u32());
    let scale = x87_quiet_bits(e.mem.u32(context + 0x10));
    let mut lanes = [0u32; 4];
    for (lane, word) in lanes.iter_mut().enumerate() {
        *word = e.mem.u32(cinfo + 0x10 + 4 * lane as u32);
    }
    for (lane, word) in lanes.iter().enumerate() {
        e.mem
            .set_u32(cinfo + 0x10 + 4 * lane as u32, mulps_lane(scale, *word));
    }
    assign_pointer(e, clone + 0x14, me + 0x14);
    e.call(BASE_COPY_MEMBERS, &args![this, clone, context]);
}

// Translated from 00ccbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccbfa0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShape::CreateCinfo` (Xbox PDB): with the allocation-scope word at 0x10,
/// stores whether the cinfo (`+0xC`) is still missing in the byte `created` points to and,
/// when it is, builds one (0x30 bytes from the memory manager, 16-byte aligned: zeros at `+0`,
/// `+4` and `+8` and the float at `01012068` in the four lanes at `+0x10`), stores it and
/// fills it from the shape through `CopyCinfo` (`00ccbce0`) when the Havok shape (`+8`) exists.
/// Answers the cinfo.
pub fn bhk_extended_mesh_shape_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let missing = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created, u8::from(missing));
    if missing {
        let block = aligned_allocate(e, 0x30);
        let constant = e.global::<u32>(CINFO_VECTOR_CONSTANT);
        e.mem.set_u32(block, 0);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0);
        for lane in 0..4 {
            e.mem.set_u32(block + 0x10 + 4 * lane, constant);
        }
        e.mem.set_u32(me + 0xc, block);
        if e.mem.u32(me + 8) != 0 {
            e.call(COPY_CINFO, &args![this, block]);
        }
    }
    let cinfo = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00ccc080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc080 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled; the padding of the stack sub-part record the loop hands to slot 0x34 (bytes 0x24..0x2F and 0x4A..0x4F) is never written by the exe and is zero here.
/// `bhkExtendedMeshShape::Init` (Xbox PDB): with the allocation-scope word at 0x10, builds a
/// Havok shape (0xF0 bytes from the current Havok allocator, `00d569c0` with the float at
/// `011b153c` and 0xC), sets its `+0xB0` to 0xF, its `+0xE4` to the float at `cinfo + 8`
/// (x87 copy) and its vector (`00d554c0` with `cinfo + 0x10`). Then, for each of the
/// sub-parts of the data at `cinfo + 4` (count at `+8`, array at `+0xC`, 0x14 bytes each), fills
/// a 0x54-byte record on the stack from it (vertex count, vertices, triangle count, indices
/// and block) and hands it to the shape's virtual slot `0x34`. Afterwards the data is assigned
/// to this shape's pointer at `+0x14`, slot `0x8C` is called with the new shape, the
/// reference the allocation held is dropped (`00c905b0`) and slot `0xBC` is called with the
/// cinfo.
pub fn bhk_extended_mesh_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let block = havok_allocate(e, 0xf0);
    let float = x87_quiet_bits(e.global::<u32>(HK_SHAPE_FLOAT));
    let shape = e
        .call(HK_SHAPE_CONSTRUCT, &args![block, float, 0xcu32])
        .u32();
    e.mem.set_u32(shape + 0xb0, 0xf);
    let scale = x87_quiet_bits(e.mem.u32(cinfo + 8));
    e.mem.set_u32(shape + 0xe4, scale);
    e.call(HK_SHAPE_SET_VECTOR, &args![shape, cinfo + 0x10]);
    let data = e.mem.u32(cinfo + 4);
    if e.mem.u32(data + 8) != 0 {
        e.with_stack(0x54, |e, record| {
            let record = record.addr();
            let mut index = 0u32;
            let mut offset = 0u32;
            loop {
                let element = e.mem.u32(data + 0xc).wrapping_add(offset);
                e.mem.set_u8(record, 0);
                e.mem.set_u8(record + 1, 1);
                e.mem.set_u16(record + 2, 8);
                e.mem.set_u32(record + 4, 0);
                e.mem.set_u16(record + 8, 1);
                e.mem.set_u16(record + 0xa, 1);
                let block = e.mem.u32(element + 0x10);
                e.mem.set_u32(record + 0xc, block);
                e.mem.set_u32(record + 0x10, 0);
                let triangles = e.mem.u32(element + 8);
                e.mem.set_u32(record + 0x14, triangles);
                let vertices = e.mem.u32(element + 4);
                e.mem.set_u32(record + 0x18, vertices);
                e.mem.set_u32(record + 0x1c, 0xc);
                let vertex_count = e.mem.u32(element);
                e.mem.set_u32(record + 0x20, vertex_count);
                for lane in 0..4 {
                    e.mem.set_u32(record + 0x30 + 4 * lane, 0);
                }
                let indices = e.mem.u32(element + 0xc);
                e.mem.set_u32(record + 0x40, indices);
                e.mem.set_u32(record + 0x44, 6);
                e.mem.set_u8(record + 0x48, 2);
                e.mem.set_u8(record + 0x49, 0);
                e.mem.set_u32(record + 0x50, 0);
                e.vcall(shape, 0x34, &args![record]);
                offset = offset.wrapping_add(0x14);
                index += 1;
                if index >= e.mem.u32(data + 8) {
                    break;
                }
            }
        });
    }
    assign_pointer(e, me + 0x14, cinfo + 4);
    e.vcall(me, 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(me, 0xbc, &args![cinfo]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ccc440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc440 - the FS:[0] exception-unwinding frame is not modeled.
/// Sub-part `LoadBinary` (no PDB name): with the allocation-scope word at 0x10, reads through
/// the stream's reader (`stream + 0x24C`, function at `+8`, flag 1, the length cell starting
/// at the size except for the index buffer, which starts at 2): the vertex count into `+0`;
/// the vertices into a block allocated for it (12 bytes each, the allocation size saturating at
/// 0xFFFFFFFF, the read size wrapping) at `+4`; the triangle count into `+8`; the indices into
/// a block (6 bytes per triangle) at `+0xC`; two more words into locals, which are stored in
/// a new 16-byte aligned 0x18-byte block at `+0x10` (its other bytes are left as
/// allocated).
pub fn fn_00ccc440(e: &mut Engine, this: Ptr, stream: u32) {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, me, 4, Some(4), 1);
    let vertex_count = e.mem.u32(me);
    let size = saturating_times(vertex_count.wrapping_mul(3), 4);
    let vertices = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let vertex_bytes = e.mem.u32(me).wrapping_mul(12);
    e.mem.set_u32(me + 4, vertices);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, vertices, vertex_bytes, Some(4), 1);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, me + 8, 4, Some(4), 1);
    let triangle_count = e.mem.u32(me + 8);
    let size = saturating_times(triangle_count.wrapping_mul(3), 2);
    let indices = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let index_bytes = e.mem.u32(me + 8).wrapping_mul(6);
    e.mem.set_u32(me + 0xc, indices);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, indices, index_bytes, Some(2), 1);
    let (first, second) = e.with_stack(8, |e, locals| {
        let locals = locals.addr();
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, locals, 4, Some(4), 1);
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, locals + 4, 4, Some(4), 1);
        (e.mem.u32(locals), e.mem.u32(locals + 4))
    });
    let block = aligned_allocate(e, 0x18);
    e.mem.set_u32(block + 4, second);
    e.mem.set_u32(block, first);
    e.mem.set_u32(me + 0x10, block);
    e.mem.set_u32(word, saved);
}

// Translated from 00ccc620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub-part `SaveBinary` (no PDB name): the mirror of `00ccc440` through the stream's writer
/// (`stream + 0x250`, function at `+0xC`, flag 1, the length cell starting at the size except
/// for the indices, which start at 2): the vertex count, the vertices (12 bytes each), the
/// triangle count, the indices (6 bytes per triangle) and the two words of the block at
/// `+0x10`.
pub fn fn_00ccc620(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, me, 4, Some(4), 1);
    let vertex_bytes = e.mem.u32(me).wrapping_mul(12);
    let vertices = e.mem.u32(me + 4);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, vertices, vertex_bytes, Some(4), 1);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, me + 8, 4, Some(4), 1);
    let index_bytes = e.mem.u32(me + 8).wrapping_mul(6);
    let indices = e.mem.u32(me + 0xc);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, indices, index_bytes, Some(2), 1);
    let block = e.mem.u32(me + 0x10);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, block, 4, Some(4), 1);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, block + 4, 4, Some(4), 1);
}

// Translated from 00ccc720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc720 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShapeData::~bhkExtendedMeshShapeData` (no PDB name for the body): installs
/// the data vtable and, with the allocation-scope word at 0x10, destroys the sub-part array
/// (`+0xC`, count in the word before it) with the CRT iterator (`00ec5fce`, 0x14-byte
/// elements, destructor `00ccbb50`) and frees its block (`00aa4060`) when there is one; then
/// restores the scope word and runs the base destructor (`00a5d3d0`).
pub fn fn_00ccc720(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, DATA_VTABLE);
    let (word, saved) = enter_scope(e);
    let array = e.mem.u32(me + 0xc);
    if array != 0 {
        let count = e.mem.u32(array - 4);
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![array, 0x14u32, count, SUBPART_DESTRUCTOR],
        );
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, array - 4]);
    }
    e.mem.set_u32(word, saved);
    e.call(DATA_BASE_DESTRUCT, &args![this]);
}

// Translated from 00ccc7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc7e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShapeData::LoadBinary` (Xbox PDB): with the allocation-scope word at 0x10,
/// reads the sub-part count into `+8` through the stream's reader (`stream + 0x24C`, function at
/// `+8`, length cell 4, flag 1), allocates the array block (`count * 0x14 + 4` bytes, both
/// steps saturating at 0xFFFFFFFF), stores the count in its first word and constructs the
/// elements with the CRT iterator (`00ec782f`, constructor `00e46e80`, destructor `00ccbb50`),
/// keeps the elements' start at `+0xC` (0 when the allocation failed) and loads each element
/// with `00ccc440`.
pub fn bhk_extended_mesh_shape_data_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, me + 8, 4, Some(4), 1);
    let count = e.mem.u32(me + 8);
    let size = saturating_times(count, 0x14).saturating_add(4);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let array = if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                block + 4,
                0x14u32,
                count,
                SUBPART_CONSTRUCTOR,
                SUBPART_DESTRUCTOR
            ],
        );
        block + 4
    } else {
        0
    };
    e.mem.set_u32(me + 0xc, array);
    if e.mem.u32(me + 8) != 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let element = e.mem.u32(me + 0xc).wrapping_add(offset);
            e.call(SUBPART_LOAD, &args![element, stream]);
            offset = offset.wrapping_add(0x14);
            index += 1;
            if index >= e.mem.u32(me + 8) {
                break;
            }
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00ccc920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::SaveBinary` (Xbox PDB): writes the sub-part count (`+8`) through
/// the stream's writer (`stream + 0x250`, function at `+0xC`, length cell 4, flag 1), then
/// saves each of the sub-parts (array at `+0xC`, 0x14 bytes each) with `00ccc620`.
pub fn bhk_extended_mesh_shape_data_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, me + 8, 4, Some(4), 1);
    if e.mem.u32(me + 8) != 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let element = e.mem.u32(me + 0xc).wrapping_add(offset);
            e.call(SUBPART_SAVE, &args![element, stream]);
            index += 1;
            offset = offset.wrapping_add(0x14);
            if index >= e.mem.u32(me + 8) {
                break;
            }
        }
    }
}

/// `NiTArray<ushort>::SetAt(size, value)` as the exe inlines it for `fn_00ccc980`: the array
/// is `{vtable, data +4, capacity +8, size +0xA, non-zero count +0xC, growth +0xE}` (16-bit
/// words). When the size has reached the capacity it grows by the growth word plus the size
/// (`00a5b790`); a value stored past the size extends it; the count follows the changes
/// between zero and non-zero values.
fn index_array_push(e: &mut Engine, array: u32, value: u16) {
    let index = u32::from(e.mem.u16(array + 0xa));
    if index >= u32::from(e.mem.u16(array + 8)) {
        let growth = u32::from(e.mem.u16(array + 0xe));
        e.call(
            INDEX_ARRAY_SET_SIZE,
            &args![array, growth.wrapping_add(index)],
        );
    }
    let count = e.mem.u16(array + 0xc);
    if index >= u32::from(e.mem.u16(array + 0xa)) {
        e.mem.set_u16(array + 0xa, (index + 1) as u16);
        if value != 0 {
            e.mem.set_u16(array + 0xc, count.wrapping_add(1));
        }
    } else {
        let data = e.mem.u32(array + 4);
        let old = e.mem.u16(data + 2 * index);
        if value != 0 {
            if old == 0 {
                e.mem.set_u16(array + 0xc, count.wrapping_add(1));
            }
        } else if old != 0 {
            e.mem.set_u16(array + 0xc, count.wrapping_add(0xffff));
        }
    }
    let data = e.mem.u32(array + 4);
    e.mem.set_u16(data + 2 * index, value);
}

// Translated from 00ccc980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccc980 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled; the pointer wrapper's word at +0xC, which the exe never writes, is zero here.
/// Builds the `bhkExtendedMeshShapeData` from a source description and initialises this shape
/// from it (no PDB name): with the allocation-scope word at 0x10, makes a data object
/// (as `00ccb9a0` does) and an array of one sub-part per word at `mesh + 0x10` (`mesh` is the
/// word at `source + 4`; the array is built as `00ccc7e0` does), then walks the mesh's
/// triangle table (`mesh + 0x14`, 8 bytes per entry: three 16-bit vertex indices, entry count
/// word at `+8`) and records (`mesh + 0x24`, 0xC bytes each, the vertex count at `+4`),
/// vertices at `mesh + 0x18` (12 bytes each).
///
/// For each sub-part the triangles whose first index is below the running vertex base plus
/// its vertex count are taken in order and their three indices, minus the base, are pushed on a
/// stack array of 16-bit indices. A sub-part without triangles is dropped (the data's count
/// goes down by one and the base is not advanced); otherwise its vertices are copied from the
/// base on through the x87 into a block of `12 * vertices + 12` bytes, the indices are copied
/// to a block of `2 * size` bytes (`memcpy`), a 0x18-byte aligned block gets the record's words
/// at `+0` and `+8`, and the sub-part is filled in (vertex count, vertices, triangle count,
/// indices, block) and the base advances by the vertex count. The index array is zeroed and
/// emptied after each sub-part.
///
/// Last, a pointer wrapper (`00ccbaa0`) holding the data, with the float at `source + 8`
/// at `+8` and the 16 bytes at `source + 0x10` at `+0x10`, goes to this shape's virtual slot
/// `0xB0` (`Init`), is destroyed (`00ca5620`) and the index array's data is freed.
#[allow(clippy::needless_late_init)]
pub fn fn_00ccc980(e: &mut Engine, this: Ptr, source: u32) {
    let (word, saved) = enter_scope(e);
    let mesh = e.mem.u32(source + 4);
    let data = fn_00ccb9a0_inline(e);
    let subparts = u32::from(e.mem.u16(mesh + 0x10));
    let size = saturating_times(subparts, 0x14).saturating_add(4);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let array = if block != 0 {
        e.mem.set_u32(block, subparts);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                block + 4,
                0x14u32,
                subparts,
                SUBPART_CONSTRUCTOR,
                SUBPART_DESTRUCTOR
            ],
        );
        block + 4
    } else {
        0
    };
    e.mem.set_u32(data + 0xc, array);
    let subparts = u32::from(e.mem.u16(mesh + 0x10));
    e.mem.set_u32(data + 8, subparts);
    e.with_stack(0x10, |e, indices| {
        let indices = indices.addr();
        e.mem.set_u16(indices + 8, 0);
        e.mem.set_u16(indices + 0xe, 1);
        e.mem.set_u16(indices + 0xa, 0);
        e.mem.set_u16(indices + 0xc, 0);
        e.mem.set_u32(indices + 4, 0);
        e.mem.set_u32(indices, INDEX_ARRAY_VTABLE);
        if e.mem.u16(mesh + 0x10) != 0 {
            let mut base = 0u32;
            let mut entry = 0u32;
            let mut record_offset = 0u32;
            let mut element4 = array.wrapping_add(4);
            let mut subpart = 0u32;
            loop {
                let record = e.mem.u32(mesh + 0x24).wrapping_add(record_offset);
                let mut triangles = 0u32;
                // The triangles of this sub-part.
                loop {
                    let entry_count = e.mem.u16(mesh + 8);
                    let table = e.mem.u32(mesh + 0x14);
                    let at = table.wrapping_add((entry & 0xffff) * 8);
                    if (entry & 0xffff) as u16 >= entry_count {
                        break;
                    }
                    let first = u32::from(e.mem.u16(at));
                    let limit = (base & 0xffff).wrapping_add(e.mem.u32(record + 4));
                    if first >= limit {
                        break;
                    }
                    let second = u32::from(e.mem.u16(at + 2));
                    let third = u32::from(e.mem.u16(at + 4));
                    for value in [first, second, third] {
                        index_array_push(e, indices, (value.wrapping_sub(base) & 0xffff) as u16);
                    }
                    entry += 1;
                    triangles += 1;
                }
                let advance;
                if triangles == 0 {
                    let count = e.mem.u32(data + 8);
                    e.mem.set_u32(data + 8, count.wrapping_sub(1));
                    advance = 0;
                } else {
                    let vertex_count = u32::from(e.mem.u16(record + 4));
                    let size = saturating_times(vertex_count * 3 + 3, 4);
                    let vertices = e
                        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
                        .u32();
                    let base16 = base & 0xffff;
                    let limit = vertex_count + base16;
                    if (base16 as i32) < (limit as i32) {
                        let mut counter = base16;
                        let mut vertex = base16;
                        loop {
                            let source_offset = vertex.wrapping_mul(12);
                            let target = vertices.wrapping_add((vertex - base16).wrapping_mul(12));
                            for lane in 0..3 {
                                let table = e.mem.u32(mesh + 0x18);
                                let word = e.mem.u32(table.wrapping_add(source_offset) + 4 * lane);
                                e.mem.set_u32(target + 4 * lane, x87_quiet_bits(word));
                            }
                            counter += 1;
                            vertex = counter & 0xffff;
                            if (vertex as i32) >= (vertex_count + base16) as i32 {
                                break;
                            }
                        }
                    }
                    let index_words = u32::from(e.mem.u16(indices + 0xa));
                    let index_block = e
                        .call(
                            MEMORY_MANAGER_ALLOCATE,
                            &args![MEMORY_MANAGER, saturating_times(index_words, 2)],
                        )
                        .u32();
                    let index_data = e.mem.u32(indices + 4);
                    e.call(
                        MEMCPY,
                        &args![index_block, index_data, index_words + index_words],
                    );
                    let small = aligned_allocate(e, 0x18);
                    let record_again = e.mem.u32(mesh + 0x24).wrapping_add(record_offset);
                    let first_word = e.mem.u32(record_again);
                    let second_word = e.mem.u32(record_again + 8);
                    e.mem.set_u32(small, first_word);
                    e.mem.set_u32(small + 4, second_word);
                    e.mem.set_u32(element4 + 0xc, small);
                    e.mem.set_u32(element4 - 4, vertex_count);
                    e.mem.set_u32(element4, vertices);
                    e.mem.set_u32(element4 + 8, index_block);
                    e.mem.set_u32(element4 + 4, triangles);
                    advance = vertex_count;
                }
                // Empty the index array.
                let size = e.mem.u16(indices + 0xa);
                if size > 0 {
                    let index_data = e.mem.u32(indices + 4);
                    for word in 0..u32::from(size) {
                        e.mem.set_u16(index_data + 2 * word, 0);
                    }
                }
                let subparts = u32::from(e.mem.u16(mesh + 0x10));
                base = base.wrapping_add(advance);
                record_offset = record_offset.wrapping_add(0xc);
                element4 = element4.wrapping_add(0x14);
                e.mem.set_u16(indices + 0xa, 0);
                subpart += 1;
                e.mem.set_u16(indices + 0xc, 0);
                if (subpart as i32) >= (subparts as i32) {
                    break;
                }
            }
        }
        // The initialisation record for slot 0xB0.
        e.with_stack(0x20, |e, wrapper| {
            let wrapper = wrapper.addr();
            e.call(POINTER_WRAPPER_CONSTRUCT, &args![wrapper, data]);
            let scale = x87_quiet_bits(e.mem.u32(source + 8));
            e.mem.set_u32(wrapper + 8, scale);
            for lane in 0..4 {
                let word = e.mem.u32(source + 0x10 + 4 * lane);
                e.mem.set_u32(wrapper + 0x10 + 4 * lane, word);
            }
            e.vcall(this.addr(), 0xb0, &args![wrapper]);
            e.call(POINTER_WRAPPER_DESTRUCT, &args![wrapper]);
        });
        let index_data = e.mem.u32(indices + 4);
        e.call(HEAP_FREE, &args![index_data]);
    });
    e.mem.set_u32(word, saved);
}

/// The data object construction `fn_00ccc980` inlines from `00ccb9a0`: 0x10 heap bytes, the
/// base constructor, the data vtable and zeros at `+8` and `+0xC`; 0 when the allocation failed.
fn fn_00ccb9a0_inline(e: &mut Engine) -> u32 {
    fn_00ccb9a0(e)
}

// Translated from 00cccf00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cccf00 - the argument of virtual slot 0xA4 is a dword read from stack of which only the first byte (the out byte of slot 0xB4) is written by the exe; the other three bytes are zero here.
/// `bhkExtendedMeshShape::SaveBinary` (Xbox PDB): asks virtual slot `0xB4` for the cinfo
/// (writing an out byte on the stack), runs the base save (`00c84d50`), writes through the
/// stream's writer (`stream + 0x250`, function at `+0xC`, flag 1) the Havok shape's (`+8`)
/// float at `+0xE4` (x87 copy, length cell 4) and its 0x10 bytes at `+0x80` (length cell
/// 0x10), saves the data object (`+0x14`) through stream virtual slot `0x30` and calls this
/// shape's slot `0xA4` with the dword that holds the out byte.
pub fn bhk_extended_mesh_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.with_stack(8, |e, locals| {
        let locals = locals.addr();
        e.vcall(me, 0xb4, &args![locals + 4]);
        let shape = e.mem.u32(me + 8);
        e.call(BASE_SAVE_BINARY, &args![this, stream]);
        let scale = x87_quiet_bits(e.mem.u32(shape + 0xe4));
        e.mem.set_u32(locals, scale);
        let writer = e.mem.u32(stream + 0x250);
        stream_transfer(e, writer, 0xc, locals, 4, Some(4), 1);
        let writer = e.mem.u32(stream + 0x250);
        stream_transfer(e, writer, 0xc, shape + 0x80, 0x10, Some(0x10), 1);
        let data = e.mem.u32(me + 0x14);
        e.vcall(stream, 0x30, &args![data]);
        let flag = e.mem.u32(locals + 4);
        e.vcall(me, 0xa4, &args![flag]);
    });
}

// Translated from 00cccfd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cccfd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtendedMeshShape::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10,
/// runs the base strings (`00cd2fd0`) and appends the class name.
pub fn bhk_extended_mesh_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(SHAPE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}

// Translated from 00ccc420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccbc00`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_extended_mesh_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_bc00;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00cccfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccc720`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_extended_mesh_shape_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_c720;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00ccb990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShapeData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8684.
pub fn bhk_extended_mesh_shape_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8684
}

// Translated from 00ccbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtendedMeshShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_867c.
pub fn bhk_extended_mesh_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_867c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca5940, bhk_extended_mesh_shape_load_binary(Ptr, u32)),
        entry!(0x00ccb990, bhk_extended_mesh_shape_data_get_r_t_t_i() -> u32),
        entry!(0x00ccb9a0, fn_00ccb9a0() -> u32),
        entry!(0x00ccba20, bhk_extended_mesh_shape_data_is_equal(Ptr, u32) -> bool),
        entry!(0x00ccba30, bhk_extended_mesh_shape_register_streamables(Ptr, u32) -> u32),
        entry!(0x00ccba50, bhk_extended_mesh_shape_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00ccba70, bhk_extended_mesh_shape_q_sub_material(Ptr, u32) -> u32),
        entry!(0x00ccbaa0, fn_00ccbaa0(Ptr, u32) -> Ptr),
        entry!(0x00ccbb30, bhk_extended_mesh_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccbc00, bhk_extended_mesh_shape_destruct(Ptr)),
        entry!(0x00ccbce0, bhk_extended_mesh_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00ccbd70, fn_00ccbd70() -> u32),
        entry!(0x00ccbe10, bhk_extended_mesh_shape_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00ccbf00,
            bhk_extended_mesh_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00ccbfa0, bhk_extended_mesh_shape_create_cinfo(Ptr, u32) -> u32),
        entry!(0x00ccc080, bhk_extended_mesh_shape_init(Ptr, u32)),
        entry!(0x00ccc420, bhk_extended_mesh_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ccc440, fn_00ccc440(Ptr, u32)),
        entry!(0x00ccc620, fn_00ccc620(Ptr, u32)),
        entry!(0x00ccc720, fn_00ccc720(Ptr)),
        entry!(
            0x00ccc7e0,
            bhk_extended_mesh_shape_data_load_binary(Ptr, u32)
        ),
        entry!(
            0x00ccc920,
            bhk_extended_mesh_shape_data_save_binary(Ptr, u32)
        ),
        entry!(0x00ccc980, fn_00ccc980(Ptr, u32)),
        entry!(0x00cccf00, bhk_extended_mesh_shape_save_binary(Ptr, u32)),
        entry!(0x00cccfb0, bhk_extended_mesh_shape_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00cccfd0,
            bhk_extended_mesh_shape_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::SCOPE_WORD;
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Chunks = Rc<RefCell<Vec<Vec<u8>>>>;
    type Transfers = Rc<RefCell<Vec<(u32, u32, u32)>>>;
    type SlotLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const FAKE_READER: u32 = 0x0900_0e08;
    const FAKE_WRITER: u32 = 0x0900_0e0c;
    const SLOT_A: u32 = 0x0900_1000;
    const SLOT_B: u32 = 0x0900_1004;
    const SLOT_C: u32 = 0x0900_1008;
    const DELETE_SLOT: u32 = 0x0900_1100;

    /// The doubles every test of this unit needs, with the unit's counters mapped.
    fn mesh_engine() -> Engine {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        allocator_doubles(&mut e);
        viewer_doubles(&mut e);
        for counter in [COUNT_BASE_A, COUNT_BASE_B, COUNT_SHAPE] {
            e.map(counter, 4);
        }
        e
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// An object whose vtable has the given `(slot offset, function)` pairs.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for &(slot, function) in slots {
            e.mem.set_u32(vtable + slot, function);
        }
        object
    }

    /// Registers doubles at `fakes` that log `(address, argument words)` in call order.
    fn log_slots(e: &mut Engine, fakes: &[u32]) -> SlotLog {
        let log = SlotLog::default();
        for &fake in fakes {
            let log = log.clone();
            e.register_double(fake, move |_, a| {
                log.borrow_mut().push((fake, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn scope_word(e: &mut Engine) -> u32 {
        e.tls() + SCOPE_WORD
    }

    /// A stream whose reader (`+0x24C`, function at `+8`) hands out `input` chunk by chunk and
    /// whose writer (`+0x250`, function at `+0xC`) keeps what it is given.
    struct TestStream {
        stream: u32,
        reader: u32,
        transfers: Transfers,
        written: Chunks,
    }

    fn test_stream(e: &mut Engine, input: Vec<Vec<u8>>) -> TestStream {
        let stream = e.mem.alloc(0x260);
        let reader = e.mem.alloc(0x20);
        let writer = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 0x250, writer);
        e.mem.set_u32(reader + 8, FAKE_READER);
        e.mem.set_u32(writer + 0xc, FAKE_WRITER);
        let input: Chunks = Rc::new(RefCell::new(input));
        let written = Chunks::default();
        let transfers = Transfers::default();
        let (reader_log, reader_input) = (transfers.clone(), input);
        e.register_double(FAKE_READER, move |e, a| {
            let cell = if a[3] != 0 { e.mem.u32(a[3]) } else { 0 };
            reader_log.borrow_mut().push((a[1], a[2], cell));
            let chunk = reader_input.borrow_mut().remove(0);
            e.mem.write(a[1], &chunk[..chunk.len().min(a[2] as usize)]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let (writer_log, writer_out) = (transfers.clone(), written.clone());
        e.register_double(FAKE_WRITER, move |e, a| {
            let cell = if a[3] != 0 { e.mem.u32(a[3]) } else { 0 };
            writer_log.borrow_mut().push((a[1], a[2], cell));
            let bytes = e.mem.bytes(a[1], a[2]);
            writer_out.borrow_mut().push(bytes);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        TestStream {
            stream,
            reader,
            transfers,
            written,
        }
    }

    #[test]
    fn load_binary_reads_the_scale_and_the_vector_into_the_cinfo() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_LOAD_BINARY, STREAM_AFTER_LOAD]);
        let cinfo = e.mem.alloc(0x40);
        e.register_double(SLOT_A, move |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret {
                eax: cinfo,
                ..Ret::default()
            }
        });
        let this = object_with_slots(&mut e, &[(0xb4, SLOT_A)]);
        let s = test_stream(&mut e, vec![words(&[0x1122_3344]), words(&[1, 2, 3, 4])]);
        e.call(0x00ca_5940, &args![this, s.stream]);
        assert_eq!(e.mem.u32(cinfo + 8), 0x1122_3344);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(cinfo + 0x10 + 4 * i))
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            *s.transfers.borrow(),
            vec![(cinfo + 8, 4, 4), (cinfo + 0x10, 0x10, 4)]
        );
        assert_eq!(e.mem.u32(s.reader + 4), 0x14);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY), vec![vec![this, s.stream]]);
        assert_eq!(calls_to(&e, STREAM_AFTER_LOAD), vec![vec![s.stream]]);
    }

    #[test]
    fn data_factory_builds_a_data_object_or_nothing() {
        let mut e = mesh_engine();
        noop(&mut e, &[DATA_BASE_CONSTRUCT]);
        e.register(HEAP_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u32(block + 8, 0xdead_beef);
            e.mem.set_u32(block + 0xc, 0xdead_beef);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let data = e.call(0x00cc_b9a0, &args![]).u32();
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data), DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(calls_to(&e, DATA_BASE_CONSTRUCT), vec![vec![data]]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x10]]);
        // An allocation that fails gives 0 and constructs nothing.
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_b9a0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, DATA_BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn data_is_equal_only_to_itself() {
        let mut e = Engine::new();
        assert!(e.call(0x00cc_ba20, &args![0x5000u32, 0x5000u32]).bool());
        assert!(!e.call(0x00cc_ba20, &args![0x5000u32, 0x5004u32]).bool());
    }

    #[test]
    fn register_streamables_registers_the_data_then_the_base() {
        let mut e = mesh_engine();
        let log = log_slots(&mut e, &[SLOT_A]);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let data = object_with_slots(&mut e, &[(0x54, SLOT_A)]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, data);
        let result = e.call(0x00cc_ba30, &args![this, 0x6000u32]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(*log.borrow(), vec![(SLOT_A, vec![data, 0x6000])]);
        assert_eq!(
            calls_to(&e, BASE_REGISTER_STREAMABLES),
            vec![vec![this, 0x6000]]
        );
    }

    #[test]
    fn is_equal_needs_the_base_and_the_same_data() {
        let mut e = mesh_engine();
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x1234);
        e.mem.set_u32(other + 0x14, 0x1234);
        e.register(BASE_IS_EQUAL, |_, _| Ret {
            eax: 0xffff_ff00,
            ..Ret::default()
        });
        assert!(!e.call(0x00cc_ba50, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x00cc_ba50, &args![this, other]).bool());
        e.mem.set_u32(other + 0x14, 0x1238);
        assert!(!e.call(0x00cc_ba50, &args![this, other]).bool());
        assert_eq!(calls_to(&e, BASE_IS_EQUAL).len(), 3);
    }

    #[test]
    fn q_sub_material_answers_the_record_word_or_0x24() {
        let mut e = mesh_engine();
        e.register(SUB_MATERIAL_LOOKUP, |e, a| {
            let record = e.mem.alloc(8);
            e.mem.set_u32(record + 4, 0x55 + a[1]);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        let shape = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, shape);
        assert_eq!(
            e.call(0x00cc_ba70, &args![this, 0xffff_ffffu32]).u32(),
            0x24
        );
        assert!(calls_to(&e, SUB_MATERIAL_LOOKUP).is_empty());
        assert_eq!(e.call(0x00cc_ba70, &args![this, 3u32]).u32(), 0x58);
        assert_eq!(calls_to(&e, SUB_MATERIAL_LOOKUP), vec![vec![shape, 3]]);
        // A null this has no shape.
        assert_eq!(e.call(0x00cc_ba70, &args![0u32, 1u32]).u32(), 0x56);
        assert_eq!(calls_to(&e, SUB_MATERIAL_LOOKUP)[1], vec![0, 1]);
    }

    #[test]
    fn pointer_wrapper_holds_a_counted_reference() {
        let mut e = mesh_engine();
        let object = ni_object(&mut e, 1, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x99);
        e.mem.set_u32(this + 4, 0x99);
        assert_eq!(e.call(0x00cc_baa0, &args![this, object]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        // No object: nothing is counted.
        let empty = e.mem.alloc(0x20);
        e.mem.set_u32(empty + 4, 0x99);
        assert_eq!(e.call(0x00cc_baa0, &args![empty, 0u32]).u32(), empty);
        assert_eq!(e.mem.u32(empty + 4), 0);
    }

    #[test]
    fn destructor_clears_the_shape_entries_and_releases_the_data() {
        let mut e = mesh_engine();
        noop(&mut e, &[SERIALIZABLE_KILL, BASE_DESTRUCT]);
        let deletes = log_slots(&mut e, &[DELETE_SLOT]);
        let shape = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0xc0);
        for offset in [0x14, 0x74] {
            e.mem.set_u32(entries + offset, 0x99);
        }
        e.mem.set_u32(shape + 0xb4, entries);
        e.mem.set_u32(shape + 0xb8, 2);
        let data = ni_object(&mut e, 1, DELETE_SLOT);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, shape);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(COUNT_SHAPE, 5);
        e.call(0x00cc_bc00, &args![this]);
        assert_eq!(e.mem.u32(this), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(entries + 0x14), 0);
        assert_eq!(e.mem.u32(entries + 0x74), 0);
        assert_eq!(e.mem.u32(COUNT_SHAPE), 4);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(*deletes.borrow(), vec![(DELETE_SLOT, vec![data])]);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn destructor_leaves_the_entries_alone_without_any_and_keeps_a_shared_data() {
        let mut e = mesh_engine();
        noop(&mut e, &[SERIALIZABLE_KILL, BASE_DESTRUCT]);
        let deletes = log_slots(&mut e, &[DELETE_SLOT]);
        let shape = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0xc0);
        e.mem.set_u32(entries + 0x14, 0x99);
        e.mem.set_u32(shape + 0xb4, entries);
        let data = ni_object(&mut e, 2, DELETE_SLOT);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, shape);
        e.mem.set_u32(this + 0x14, data);
        e.call(0x00cc_bc00, &args![this]);
        assert_eq!(e.mem.u32(entries + 0x14), 0x99);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(deletes.borrow().is_empty());
    }

    #[test]
    fn copy_cinfo_copies_the_scale_the_vector_and_the_data() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_COPY_CINFO]);
        let deletes = log_slots(&mut e, &[DELETE_SLOT]);
        let shape = e.mem.alloc(0x100);
        e.mem.set_u32(shape + 0xe4, 0x7fa0_0000);
        for lane in 0..4 {
            e.mem.set_u32(shape + 0x80 + 4 * lane, lane + 1);
        }
        let data = ni_object(&mut e, 1, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, shape);
        e.mem.set_u32(this + 0x14, data);
        let old = ni_object(&mut e, 1, DELETE_SLOT);
        let cinfo = e.mem.alloc(0x30);
        e.mem.set_u32(cinfo + 4, old);
        e.call(0x00cc_bce0, &args![this, cinfo]);
        assert_eq!(calls_to(&e, BASE_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(e.mem.u32(cinfo + 4), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(*deletes.borrow(), vec![(DELETE_SLOT, vec![old])]);
        assert_eq!(e.mem.u32(cinfo + 8), 0x7fe0_0000);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(cinfo + 0x10 + 4 * i))
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
        // The same data again: no change in the count.
        e.call(0x00cc_bce0, &args![this, cinfo]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn factory_builds_a_shape_or_nothing() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_CONSTRUCT]);
        e.register(HEAP_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in [0xc, 0x10, 0x14] {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let shape = e.call(0x00cc_bd70, &args![]).u32();
        assert_ne!(shape, 0);
        assert_eq!(e.mem.u32(shape), SHAPE_VTABLE);
        assert_eq!(
            [0xc, 0x10, 0x14].map(|offset| e.mem.u32(shape + offset)),
            [0, 0, 0]
        );
        assert_eq!(
            [COUNT_BASE_A, COUNT_BASE_B, COUNT_SHAPE].map(|c| e.mem.u32(c)),
            [1, 1, 1]
        );
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![shape]]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x18]]);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_bd70, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_a_shape_and_has_this_copy_into_it() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_CONSTRUCT]);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let seen = Rc::new(Cell::new(0u32));
        let log = SlotLog::default();
        let (scope_seen, slot_log) = (seen.clone(), log.clone());
        e.register_double(SLOT_A, move |e, a| {
            let word = e.tls() + SCOPE_WORD;
            scope_seen.set(e.mem.u32(word));
            slot_log.borrow_mut().push((SLOT_A, a.to_vec()));
            Ret::default()
        });
        let this = object_with_slots(&mut e, &[(0xc4, SLOT_A)]);
        let clone = e.call(0x00cc_be10, &args![this, 0x7000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), SHAPE_VTABLE);
        assert_eq!(*log.borrow(), vec![(SLOT_A, vec![this, clone, 0x7000])]);
        assert_eq!(seen.get(), 0x10);
        assert_eq!(e.mem.u32(word), 0x33);
        assert_eq!(calls_to(&e, CLONE_LOCK).len(), 1);
        assert_eq!(calls_to(&e, CLONE_UNLOCK).len(), 1);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![clone]]);
    }

    #[test]
    fn copy_members_scales_the_vector_and_shares_the_data() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_COPY_MEMBERS]);
        let cinfo = e.mem.alloc(0x30);
        let lanes = [
            2.0f32.to_bits(),
            0x7fa0_0001,
            3.0f32.to_bits(),
            0.5f32.to_bits(),
        ];
        for (lane, word) in lanes.iter().enumerate() {
            e.mem.set_u32(cinfo + 0x10 + 4 * lane as u32, *word);
        }
        e.register_double(SLOT_A, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: cinfo,
                ..Ret::default()
            }
        });
        let data = ni_object(&mut e, 1, 0);
        let this = object_with_slots(&mut e, &[(0xb4, SLOT_A)]);
        e.mem.set_u32(this + 0x14, data);
        let clone = e.mem.alloc(0x20);
        let context = e.mem.alloc(0x20);
        e.mem.set_u32(context + 0x10, 4.0f32.to_bits());
        e.call(0x00cc_bf00, &args![this, clone, context]);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(cinfo + 0x10 + 4 * i))
                .collect::<Vec<_>>(),
            vec![
                8.0f32.to_bits(),
                0x7fe0_0001,
                12.0f32.to_bits(),
                2.0f32.to_bits()
            ]
        );
        assert_eq!(e.mem.u32(clone + 0x14), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, context]]
        );
        // A signalling NaN scale wins every lane, quieted.
        e.mem.set_u32(context + 0x10, 0x7fa0_0000);
        e.call(0x00cc_bf00, &args![this, clone, context]);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(cinfo + 0x10 + 4 * i))
                .collect::<Vec<_>>(),
            vec![0x7fe0_0000; 4]
        );
    }

    #[test]
    fn create_cinfo_builds_the_block_once() {
        let mut e = mesh_engine();
        noop(&mut e, &[COPY_CINFO]);
        e.map(CINFO_VECTOR_CONSTANT, 4);
        e.mem.set_u32(CINFO_VECTOR_CONSTANT, 0x3f80_0000);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x4000);
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 9);
        let block = e.call(0x00cc_bfa0, &args![this, created]).u32();
        assert_ne!(block, 0);
        assert_eq!(block & 0xf, 0);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), block);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(block + 0x10 + 4 * i))
                .collect::<Vec<_>>(),
            vec![0x3f80_0000; 4]
        );
        assert_eq!(calls_to(&e, COPY_CINFO), vec![vec![this, block]]);
        assert_eq!(e.mem.u32(word), 0x33);
        // The second call finds the block.
        assert_eq!(e.call(0x00cc_bfa0, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, COPY_CINFO).len(), 1);
    }

    #[test]
    fn create_cinfo_without_a_havok_shape_is_not_filled() {
        let mut e = mesh_engine();
        noop(&mut e, &[COPY_CINFO]);
        e.map(CINFO_VECTOR_CONSTANT, 4);
        let this = e.mem.alloc(0x20);
        let created = e.mem.alloc(4);
        let block = e.call(0x00cc_bfa0, &args![this, created]).u32();
        assert_ne!(block, 0);
        assert!(calls_to(&e, COPY_CINFO).is_empty());
    }

    /// A cinfo with `parts` sub-parts of the data at `+4`, each `(vertex count, vertices,
    /// triangle count, indices, block)`.
    fn init_cinfo(e: &mut Engine, parts: &[[u32; 5]]) -> (u32, u32) {
        let cinfo = e.mem.alloc(0x40);
        let data = ni_object(e, 1, 0);
        let array = e.mem.alloc(0x14 * parts.len() as u32 + 0x14);
        for (index, part) in parts.iter().enumerate() {
            for (word, value) in part.iter().enumerate() {
                e.mem
                    .set_u32(array + 0x14 * index as u32 + 4 * word as u32, *value);
            }
        }
        e.mem.set_u32(data + 8, parts.len() as u32);
        e.mem.set_u32(data + 0xc, array);
        e.mem.set_u32(cinfo + 4, data);
        e.mem.set_u32(cinfo + 8, 0x4000_0000);
        for lane in 0..4 {
            e.mem.set_u32(cinfo + 0x10 + 4 * lane, 0x10 + lane);
        }
        (cinfo, data)
    }

    /// Havok shape doubles for `Init`: the constructor answers its block with a vtable whose
    /// slot `0x34` keeps the records it is given.
    fn init_doubles(e: &mut Engine, records: &Chunks, log: &SlotLog) {
        e.map(HK_SHAPE_FLOAT, 4);
        e.mem.set_u32(HK_SHAPE_FLOAT, 0x3f00_0000);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x34, SLOT_C);
        e.register_double(HK_SHAPE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        noop(e, &[HK_SHAPE_SET_VECTOR, REMOVE_REFERENCE]);
        let (records, log) = (records.clone(), log.clone());
        e.register_double(SLOT_C, move |e, a| {
            records.borrow_mut().push(e.mem.bytes(a[1], 0x54));
            log.borrow_mut().push((SLOT_C, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn init_builds_the_havok_shape_from_the_sub_parts() {
        let mut e = mesh_engine();
        let records = Chunks::default();
        let log = log_slots(&mut e, &[SLOT_A, SLOT_B]);
        init_doubles(&mut e, &records, &log);
        let (cinfo, data) = init_cinfo(
            &mut e,
            &[
                [3, 0xaaa0, 1, 0xbbb0, 0xccc0],
                [4, 0xaaa1, 2, 0xbbb1, 0xccc1],
            ],
        );
        let this = object_with_slots(&mut e, &[(0x8c, SLOT_A), (0xbc, SLOT_B)]);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        e.call(0x00cc_c080, &args![this, cinfo]);
        let calls = calls_to(&e, HK_SHAPE_CONSTRUCT);
        assert_eq!(calls.len(), 1);
        let shape = calls[0][0];
        assert_eq!(calls[0][1..], [0x3f00_0000, 0xc]);
        assert_eq!(e.mem.u32(shape + 0xb0), 0xf);
        assert_eq!(e.mem.u32(shape + 0xe4), 0x4000_0000);
        assert_eq!(
            calls_to(&e, HK_SHAPE_SET_VECTOR),
            vec![vec![shape, cinfo + 0x10]]
        );
        let records = records.borrow();
        assert_eq!(records.len(), 2);
        for (record, part) in records.iter().zip([
            [3u32, 0xaaa0, 1, 0xbbb0, 0xccc0],
            [4, 0xaaa1, 2, 0xbbb1, 0xccc1],
        ]) {
            let word = |at: usize| u32::from_le_bytes(record[at..at + 4].try_into().unwrap());
            assert_eq!(record[0], 0);
            assert_eq!(record[1], 1);
            assert_eq!(u16::from_le_bytes([record[2], record[3]]), 8);
            assert_eq!(word(4), 0);
            assert_eq!(u16::from_le_bytes([record[8], record[9]]), 1);
            assert_eq!(u16::from_le_bytes([record[10], record[11]]), 1);
            assert_eq!(word(0xc), part[4]);
            assert_eq!(word(0x10), 0);
            assert_eq!(word(0x14), part[2]);
            assert_eq!(word(0x18), part[1]);
            assert_eq!(word(0x1c), 0xc);
            assert_eq!(word(0x20), part[0]);
            assert_eq!(&record[0x30..0x40], &[0u8; 16]);
            assert_eq!(word(0x40), part[3]);
            assert_eq!(word(0x44), 6);
            assert_eq!(record[0x48], 2);
            assert_eq!(record[0x49], 0);
            assert_eq!(word(0x50), 0);
        }
        let slots: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(slots, vec![SLOT_C, SLOT_C, SLOT_A, SLOT_B]);
        assert_eq!(log.borrow()[2].1, vec![this, shape]);
        assert_eq!(log.borrow()[3].1, vec![this, cinfo]);
        assert_eq!(e.mem.u32(this + 0x14), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![shape]]);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn init_without_sub_parts_makes_no_records() {
        let mut e = mesh_engine();
        let records = Chunks::default();
        let log = log_slots(&mut e, &[SLOT_A, SLOT_B]);
        init_doubles(&mut e, &records, &log);
        let (cinfo, _) = init_cinfo(&mut e, &[]);
        let this = object_with_slots(&mut e, &[(0x8c, SLOT_A), (0xbc, SLOT_B)]);
        e.call(0x00cc_c080, &args![this, cinfo]);
        assert!(records.borrow().is_empty());
        let slots: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(slots, vec![SLOT_A, SLOT_B]);
    }

    #[test]
    fn sub_part_load_reads_the_arrays_and_the_block() {
        let mut e = mesh_engine();
        let vertices: Vec<u32> = (1..=6).collect();
        let s = test_stream(
            &mut e,
            vec![
                words(&[2]),
                words(&vertices),
                words(&[1]),
                vec![1, 0, 2, 0, 3, 0],
                words(&[0xa1]),
                words(&[0xb2]),
            ],
        );
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let this = e.mem.alloc(0x20);
        e.call(0x00cc_c440, &args![this, s.stream]);
        assert_eq!(e.mem.u32(this), 2);
        let verts = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(verts, 24), words(&vertices));
        assert_eq!(e.mem.u32(this + 8), 1);
        let indices = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.bytes(indices, 6), vec![1, 0, 2, 0, 3, 0]);
        let block = e.mem.u32(this + 0x10);
        assert_eq!(block & 0xf, 0);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0xa1, 0xb2));
        let transfers = s.transfers.borrow().clone();
        assert_eq!(transfers.len(), 6);
        assert_eq!(transfers[0], (this, 4, 4));
        assert_eq!(transfers[1], (verts, 24, 4));
        assert_eq!(transfers[2], (this + 8, 4, 4));
        assert_eq!(transfers[3], (indices, 6, 2));
        assert_eq!((transfers[4].1, transfers[4].2), (4, 4));
        assert_eq!((transfers[5].1, transfers[5].2), (4, 4));
        assert_eq!(transfers[5].0, transfers[4].0 + 4);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sizes, vec![24, 6, 0x18]);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn sub_part_save_writes_what_load_reads() {
        let mut e = mesh_engine();
        let s = test_stream(&mut e, vec![]);
        let this = e.mem.alloc(0x20);
        let vertices = e.mem.alloc(0x20);
        let indices = e.mem.alloc(0x10);
        let block = e.mem.alloc(0x20);
        e.mem.write(vertices, &words(&[1, 2, 3, 4, 5, 6]));
        e.mem.write(indices, &[1, 0, 2, 0, 3, 0]);
        e.mem.set_u32(block, 0xa1);
        e.mem.set_u32(block + 4, 0xb2);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, vertices);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_u32(this + 0xc, indices);
        e.mem.set_u32(this + 0x10, block);
        e.call(0x00cc_c620, &args![this, s.stream]);
        assert_eq!(
            *s.written.borrow(),
            vec![
                words(&[2]),
                words(&[1, 2, 3, 4, 5, 6]),
                words(&[1]),
                vec![1, 0, 2, 0, 3, 0],
                words(&[0xa1]),
                words(&[0xb2]),
            ]
        );
        assert_eq!(
            *s.transfers.borrow(),
            vec![
                (this, 4, 4),
                (vertices, 24, 4),
                (this + 8, 4, 4),
                (indices, 6, 2),
                (block, 4, 4),
                (block + 4, 4, 4),
            ]
        );
        assert_eq!(e.mem.u32(s.reader + 4), 0);
    }

    #[test]
    fn data_destructor_destroys_the_sub_part_array() {
        let mut e = mesh_engine();
        noop(
            &mut e,
            &[
                VECTOR_DESTRUCTOR_ITERATOR,
                MEMORY_MANAGER_FREE,
                DATA_BASE_DESTRUCT,
            ],
        );
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, 3);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, block + 4);
        e.call(0x00cc_c720, &args![this]);
        assert_eq!(e.mem.u32(this), DATA_VTABLE);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![block + 4, 0x14, 3, SUBPART_DESTRUCTOR]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to(&e, DATA_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(word), 0x33);
        // No array: only the base destructor.
        let empty = e.mem.alloc(0x10);
        e.call(0x00cc_c720, &args![empty]);
        assert_eq!(calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR).len(), 1);
        assert_eq!(calls_to(&e, DATA_BASE_DESTRUCT).len(), 2);
    }

    #[test]
    fn data_load_builds_the_array_and_loads_each_sub_part() {
        let mut e = mesh_engine();
        noop(&mut e, &[VECTOR_CONSTRUCTOR_ITERATOR, SUBPART_LOAD]);
        let s = test_stream(&mut e, vec![words(&[2])]);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_c7e0, &args![this, s.stream]);
        assert_eq!(e.mem.u32(this + 8), 2);
        let array = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(array - 4), 2);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x2c]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                array,
                0x14,
                2,
                SUBPART_CONSTRUCTOR,
                SUBPART_DESTRUCTOR
            ]]
        );
        assert_eq!(
            calls_to(&e, SUBPART_LOAD),
            vec![vec![array, s.stream], vec![array + 0x14, s.stream]]
        );
        assert_eq!(*s.transfers.borrow(), vec![(this + 8, 4, 4)]);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn data_load_with_no_sub_parts_loads_none() {
        let mut e = mesh_engine();
        noop(&mut e, &[VECTOR_CONSTRUCTOR_ITERATOR, SUBPART_LOAD]);
        let s = test_stream(&mut e, vec![words(&[0])]);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_c7e0, &args![this, s.stream]);
        assert!(calls_to(&e, SUBPART_LOAD).is_empty());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4]]
        );
    }

    #[test]
    fn data_load_with_a_failed_allocation_keeps_no_array() {
        let mut e = mesh_engine();
        noop(&mut e, &[VECTOR_CONSTRUCTOR_ITERATOR, SUBPART_LOAD]);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        let s = test_stream(&mut e, vec![words(&[2])]);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_c7e0, &args![this, s.stream]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
        assert_eq!(
            calls_to(&e, SUBPART_LOAD),
            vec![vec![0, s.stream], vec![0x14, s.stream]]
        );
    }

    #[test]
    fn data_save_writes_the_count_and_each_sub_part() {
        let mut e = mesh_engine();
        noop(&mut e, &[SUBPART_SAVE]);
        let s = test_stream(&mut e, vec![]);
        let this = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, array);
        e.call(0x00cc_c920, &args![this, s.stream]);
        assert_eq!(*s.written.borrow(), vec![words(&[2])]);
        assert_eq!(*s.transfers.borrow(), vec![(this + 8, 4, 4)]);
        assert_eq!(
            calls_to(&e, SUBPART_SAVE),
            vec![vec![array, s.stream], vec![array + 0x14, s.stream]]
        );
        let empty = e.mem.alloc(0x10);
        e.call(0x00cc_c920, &args![empty, s.stream]);
        assert_eq!(calls_to(&e, SUBPART_SAVE).len(), 2);
    }

    /// A mesh description for `fn_00ccc980`: the triangle table, the vertex table and the
    /// sub-part records `(word, vertex count, word)`.
    fn build_mesh(
        e: &mut Engine,
        triangles: &[[u16; 3]],
        vertices: &[[u32; 3]],
        records: &[(u32, u32, u32)],
    ) -> u32 {
        let mesh = e.mem.alloc(0x40);
        let table = e.mem.alloc(8 * triangles.len() as u32 + 8);
        for (index, triangle) in triangles.iter().enumerate() {
            for (word, value) in triangle.iter().enumerate() {
                e.mem
                    .set_u16(table + 8 * index as u32 + 2 * word as u32, *value);
            }
        }
        let vertex_table = e.mem.alloc(12 * vertices.len() as u32 + 12);
        for (index, vertex) in vertices.iter().enumerate() {
            for (lane, value) in vertex.iter().enumerate() {
                e.mem
                    .set_u32(vertex_table + 12 * index as u32 + 4 * lane as u32, *value);
            }
        }
        let record_table = e.mem.alloc(0xc * records.len() as u32 + 0xc);
        for (index, record) in records.iter().enumerate() {
            let at = record_table + 0xc * index as u32;
            e.mem.set_u32(at, record.0);
            e.mem.set_u32(at + 4, record.1);
            e.mem.set_u32(at + 8, record.2);
        }
        e.mem.set_u16(mesh + 8, triangles.len() as u16);
        e.mem.set_u16(mesh + 0x10, records.len() as u16);
        e.mem.set_u32(mesh + 0x14, table);
        e.mem.set_u32(mesh + 0x18, vertex_table);
        e.mem.set_u32(mesh + 0x24, record_table);
        mesh
    }

    /// What the pointer wrapper handed to slot `0xB0` held: the data, the scale and the vector.
    type WrapperSeen = Rc<RefCell<Vec<u32>>>;

    fn conversion_doubles(e: &mut Engine) -> (WrapperSeen, Rc<RefCell<Vec<u32>>>) {
        noop(
            e,
            &[
                DATA_BASE_CONSTRUCT,
                VECTOR_CONSTRUCTOR_ITERATOR,
                POINTER_WRAPPER_DESTRUCT,
            ],
        );
        e.register(INDEX_ARRAY_SET_SIZE, |e, a| {
            let old = e.mem.u32(a[0] + 4);
            let size = u32::from(e.mem.u16(a[0] + 0xa));
            let block = e.mem.alloc(a[1] * 2 + 2);
            if old != 0 {
                let kept = e.mem.bytes(old, size * 2);
                e.mem.write(block, &kept);
            }
            e.mem.set_u32(a[0] + 4, block);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let freed_log = freed.clone();
        e.register_double(HEAP_FREE, move |_, a| {
            freed_log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let seen = WrapperSeen::default();
        let seen_log = seen.clone();
        e.register_double(SLOT_A, move |e, a| {
            let wrapper = a[1];
            let mut kept = vec![e.mem.u32(wrapper + 4), e.mem.u32(wrapper + 8)];
            kept.extend((0..4).map(|i| e.mem.u32(wrapper + 0x10 + 4 * i)));
            *seen_log.borrow_mut() = kept;
            Ret::default()
        });
        (seen, freed)
    }

    fn conversion_source(e: &mut Engine, mesh: u32) -> u32 {
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source + 4, mesh);
        e.mem.set_u32(source + 8, 0x7fa0_0000);
        for lane in 0..4 {
            e.mem.set_u32(source + 0x10 + 4 * lane, 0x20 + lane);
        }
        source
    }

    #[test]
    fn conversion_splits_the_mesh_into_sub_parts() {
        let mut e = mesh_engine();
        let (seen, freed) = conversion_doubles(&mut e);
        let vertices: Vec<[u32; 3]> = (0..5u32)
            .map(|k| [3 * k + 1, 3 * k + 2, 3 * k + 3])
            .collect();
        let mut vertices_in = vertices.clone();
        vertices_in[0][0] = 0x7fa0_0000;
        let mesh = build_mesh(
            &mut e,
            &[[0, 1, 2], [3, 4, 4]],
            &vertices_in,
            &[(0xa0, 3, 0xb0), (0xa1, 2, 0xb1)],
        );
        let source = conversion_source(&mut e, mesh);
        let this = object_with_slots(&mut e, &[(0xb0, SLOT_A)]);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        e.call(0x00cc_c980, &args![this, source]);
        let seen = seen.borrow().clone();
        assert_eq!(seen.len(), 6);
        let data = seen[0];
        assert_ne!(data, 0);
        assert_eq!(&seen[1..], [0x7fe0_0000, 0x20, 0x21, 0x22, 0x23]);
        assert_eq!(e.mem.u32(data), DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 8), 2);
        assert_eq!(e.mem.u32(data + 4), 1);
        let array = e.mem.u32(data + 0xc);
        assert_eq!(e.mem.u32(array - 4), 2);
        // First sub-part: vertices 0..3 (the signalling NaN comes out quiet), indices 0 1 2.
        assert_eq!(e.mem.u32(array), 3);
        let verts = e.mem.u32(array + 4);
        let mut expected: Vec<u32> = vertices[..3].iter().flatten().copied().collect();
        expected[0] = 0x7fe0_0000;
        assert_eq!(e.mem.bytes(verts, 36), words(&expected));
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(
            e.mem.bytes(e.mem.u32(array + 0xc), 6),
            vec![0, 0, 1, 0, 2, 0]
        );
        let block = e.mem.u32(array + 0x10);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0xa0, 0xb0));
        // Second sub-part: vertices 3..5, indices relative to its base.
        let second = array + 0x14;
        assert_eq!(e.mem.u32(second), 2);
        let verts = e.mem.u32(second + 4);
        let expected: Vec<u32> = vertices[3..].iter().flatten().copied().collect();
        assert_eq!(e.mem.bytes(verts, 24), words(&expected));
        assert_eq!(e.mem.u32(second + 8), 1);
        assert_eq!(
            e.mem.bytes(e.mem.u32(second + 0xc), 6),
            vec![0, 0, 1, 0, 1, 0]
        );
        let block = e.mem.u32(second + 0x10);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0xa1, 0xb1));
        // Allocation sizes: array, then vertices / indices / block for each sub-part.
        let sizes: Vec<u32> = calls_to(&e, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sizes, vec![0x2c, 48, 6, 0x18, 36, 6, 0x18]);
        assert_eq!(calls_to(&e, POINTER_WRAPPER_DESTRUCT).len(), 1);
        assert_eq!(freed.borrow().len(), 1);
        assert_ne!(freed.borrow()[0], 0);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn conversion_drops_a_sub_part_without_triangles() {
        let mut e = mesh_engine();
        let (seen, _) = conversion_doubles(&mut e);
        let mesh = build_mesh(&mut e, &[], &[[1, 2, 3]], &[(0xa0, 3, 0xb0)]);
        let source = conversion_source(&mut e, mesh);
        let this = object_with_slots(&mut e, &[(0xb0, SLOT_A)]);
        e.call(0x00cc_c980, &args![this, source]);
        let data = seen.borrow()[0];
        assert_eq!(e.mem.u32(data + 8), 0);
        let array = e.mem.u32(data + 0xc);
        assert_eq!(e.mem.bytes(array, 0x14), vec![0u8; 0x14]);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sizes, vec![0x18]);
    }

    #[test]
    fn save_binary_writes_the_scale_the_vector_and_the_data() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_SAVE_BINARY]);
        let log = log_slots(&mut e, &[SLOT_B, SLOT_C]);
        e.register_double(SLOT_A, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let s = test_stream(&mut e, vec![]);
        let stream_vtable = e.mem.u32(s.stream);
        e.mem.set_u32(stream_vtable + 0x30, SLOT_C);
        let shape = e.mem.alloc(0x100);
        e.mem.set_u32(shape + 0xe4, 0x7fa0_0000);
        for lane in 0..4 {
            e.mem.set_u32(shape + 0x80 + 4 * lane, lane + 1);
        }
        let data = e.mem.alloc(0x10);
        let this = object_with_slots(&mut e, &[(0xb4, SLOT_A), (0xa4, SLOT_B)]);
        e.mem.set_u32(this + 8, shape);
        e.mem.set_u32(this + 0x14, data);
        e.call(0x00cc_cf00, &args![this, s.stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, s.stream]]);
        assert_eq!(
            *s.written.borrow(),
            vec![words(&[0x7fe0_0000]), words(&[1, 2, 3, 4])]
        );
        let transfers = s.transfers.borrow().clone();
        assert_eq!((transfers[0].1, transfers[0].2), (4, 4));
        assert_eq!(transfers[1], (shape + 0x80, 0x10, 0x10));
        assert_eq!(
            *log.borrow(),
            vec![(SLOT_C, vec![s.stream, data]), (SLOT_B, vec![this, 1])]
        );
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = mesh_engine();
        noop(&mut e, &[BASE_GET_VIEWER_STRINGS]);
        e.map(SHAPE_RTTI, 4);
        e.mem.set_u32(SHAPE_RTTI, 0x1234);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 0x33);
        let strings = strings_header(&mut e, 0, 4, 2);
        let this = e.mem.alloc(0x20);
        e.call(0x00cc_cfd0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(pushed_texts(&e, strings, 0, 1), vec![0x7000 + 0x1234]);
        assert_eq!(e.mem.u16(strings + 0xa), 1);
        assert_eq!(e.mem.u32(word), 0x33);
    }

    #[test]
    fn bhk_extended_mesh_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_bc00, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00ccc420, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ccc420, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00cc_bc00), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_extended_mesh_shape_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_c720, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cccfb0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cccfb0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_c720), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_extended_mesh_shape_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_b990, &args![]).u32(), 0x0126_8684);
    }

    #[test]
    fn bhk_extended_mesh_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_bb30, &args![]).u32(), 0x0126_867c);
    }
}
