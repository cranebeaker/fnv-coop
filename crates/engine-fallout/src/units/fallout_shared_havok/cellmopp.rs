//! `fallout shared/havok/cellmopp.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CellMopp` (class name confirmed by the RTTI of its vtable `0104dab4`) is
//! the object that builds the cell's collision shapes: `Create` turns a list
//! of scene-graph objects into rigid bodies that share a heightfield BV-tree
//! shape, and keeps the bodies in an array at `+8`.
//!
//! The compiler's exception-unwinding frames (`FS:[0]` registrations and the
//! `/GS` cookie check) are not modeled; the functions that have them carry a
//! `Translation incomplete` line.
//!
//! x87 note: a `float` loaded and stored again (`FLD`/`FSTP`) turns a
//! signalling NaN into a quiet one; `x87_quiet_bits` does that on the bit pattern.

#[allow(unused_imports)]
use crate::prelude::*;

/// `CellMopp`'s vtable (RTTI name `.?AVCellMopp@@`).
const CELL_MOPP_VTABLE: u32 = 0x0104_dab4;
/// `bhkTriSampledHeightFieldBvTreeShape`'s vtable (RTTI name confirmed).
const TRI_SAMPLED_SHAPE_VTABLE: u32 = 0x0104_dac4;
/// `bhkBvTreeShape`'s vtable (RTTI name confirmed).
const BV_TREE_SHAPE_VTABLE: u32 = 0x0104_dbac;
/// Instance counters: `CellMopp`'s (its destructor decrements it), and those
/// the `bhkBvTreeShape` and `bhkTriSampledHeightFieldBvTreeShape`
/// constructors increment.
const CELL_MOPP_COUNT: u32 = 0x011c_c1c8;
const BV_TREE_SHAPE_COUNT: u32 = 0x0126_82cc;
const TRI_SAMPLED_SHAPE_COUNT: u32 = 0x0126_834c;
/// The `NiRTTI` records returned by the two `GetRTTI` functions.
const BV_TREE_SHAPE_RTTI: u32 = 0x0126_82d0;
const TRI_SAMPLED_SHAPE_RTTI: u32 = 0x0126_8350;
/// A setting object whose word at `+4` `Create` reads as a `float`; the
/// getter `00403e20` returns the address of that word (of a static zero for
/// a null object).
const SETTING_OBJECT: u32 = 0x011c_c1d8;
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// The `float` at `0101e704`, which `Create` scales (`004a3e90`) to build the
/// extents vector.
const EXTENT_BASE: u32 = 0x0101_e704;
/// Rigid-body cinfo (`bhkRigidBodyCinfo`, 0xE0 bytes in the Xbox PDB):
/// constructor (engine map), destructor, and the setters `Create` calls
/// (`00507610` takes a `float`, `0056d360` a small integer, `005076c0` a
/// `float`, `0056f0f0` a packed word; their Xbox names are not in the map).
const CINFO_CONSTRUCT: u32 = 0x00c8_f510;
const CINFO_DESTRUCT: u32 = 0x0056_d730;
const CINFO_SET_FIRST_FLOAT: u32 = 0x0050_7610;
const CINFO_SET_SMALL_INTEGER: u32 = 0x0056_d360;
const CINFO_SET_SECOND_FLOAT: u32 = 0x0050_76c0;
const CINFO_SET_PACKED_WORD: u32 = 0x0056_f0f0;
/// `bhkRigidBody::bhkRigidBody` (engine map), takes the cinfo.
const RIGID_BODY_CONSTRUCT: u32 = 0x0056_d380;
/// `bhkWorldObject::SetAVObject` (engine map): `(this, avObject, word)`.
const SET_AV_OBJECT: u32 = 0x00c8_5c10;
/// The word `Create` passes as the second argument of `SetAVObject`.
const SET_AV_OBJECT_ARGUMENT: u32 = 0x011f_4a20;
/// `bhkTriSampledHeightFieldBvTreeShape::Init` (engine map).
const TRI_SAMPLED_SHAPE_INIT: u32 = 0x00cb_0820;
/// `bhkRefObject::bhkRefObject` (engine map).
const REF_OBJECT_CONSTRUCT: u32 = 0x0056_e6f0;
/// `bhkBvTreeShape::~bhkBvTreeShape` and the derived destructor (engine map).
const BV_TREE_SHAPE_DESTRUCT: u32 = 0x00ca_4da0;
const TRI_SAMPLED_SHAPE_DESTRUCT: u32 = 0x00ca_ff80;
/// The allocator: `operator new(size)` and `operator delete(block, size)`.
const ALLOCATE: u32 = 0x00aa_13e0;
const FREE: u32 = 0x00aa_1460;
/// Smart-pointer helpers: store a pointer (`00633c90`; it adds a reference
/// when the pointer is not null) and release it (`0045cec0`).
const SMART_POINTER_SET: u32 = 0x0063_3c90;
const SMART_POINTER_RELEASE: u32 = 0x0045_cec0;
/// `*this` (`MOV EAX,[ECX]`).
const POINTER_GET: u32 = 0x0055_9450;
/// The `float` at `011c582c` times a `float` argument, result in `ST0`.
const SCALE_FLOAT: u32 = 0x004a_3e90;
/// Stores four `float`s at `this[0..4]`; the arguments are in memory order.
const SET_VECTOR4: u32 = 0x004b_4d50;
/// Heightfield cinfo helpers: the constructor of the 0x10-byte object
/// `Init` takes, and the routine that works out the minimum and maximum
/// height of the extents object.
const SHAPE_CINFO_CONSTRUCT: u32 = 0x0062_2490;
const EXTENTS_UPDATE_RANGE: u32 = 0x00ca_fef0;
/// `00559450(this + 0xb8)` then `007af430`: the form the object refers to.
const GET_OBJECT_FORM: u32 = 0x0049_ec60;
/// Reads the shape's word that goes into the cinfo (`0044ddc0` behind it).
const GET_SHAPE_WORD: u32 = 0x0062_0b80;
/// `00622470`: nonzero when the array at `this + 8` has entries (its count,
/// the `u16` at `+0xc`, is read by `009938b0`).
const HAS_ENTRIES: u32 = 0x0062_2470;
/// Array helpers: element count (`009938b0`, the `u16` at `+0xc`), element at
/// an index (`00877a30`), the owner's body remover (`005380d0`) and the
/// array's `RemoveAll` (engine map `NiTArray<...>::RemoveAll`).
const LIST_COUNT: u32 = 0x0099_38b0;
const LIST_AT: u32 = 0x0087_7a30;
const REMOVE_BODY: u32 = 0x0053_80d0;
const LIST_REMOVE_ALL: u32 = 0x004d_ffa0;
/// Base destructor called last by the `CellMopp` destructor.
const BASE_DESTRUCT: u32 = 0x0049_6910;
/// Destructor of the array object at `+8` (the engine map's library match
/// `~basic_streambuf<>` is wrong: it sets vtable `0104dc94`, an `NiTArray`
/// of `NiPointer`s per its RTTI name, and frees the buffer at `+4`).
const LIST_DESTRUCT: u32 = 0x0062_2780;
/// Array insert used by `fn_006227b0`.
const LIST_SET_AT: u32 = 0x0062_28e0;
/// `00621480`: what the owner lookup returns for the first body's `+0x94`
/// value (the engine map names it `HighProcess::GetCurrentProcessIdle`; the
/// name is unconfirmed).
const OWNER_LOOKUP: u32 = 0x0062_1480;
/// `bhkBvTreeShape::FindShapeCollection` target.
const FIND_SHAPE_COLLECTION: u32 = 0x0057_7190;

// Translated from 00621d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CellMopp::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// body and, when bit 0 of `flags` is set, frees the 0x18-byte object.
/// Returns `this`.
pub fn cell_mopp_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00621dc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00621dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00621dc0 - the FS:[0] exception-unwinding frame is not modeled.
/// The `CellMopp` destructor body (the class is confirmed by the vtable's RTTI):
/// restores its vtable, decrements the global instance counter, destroys the
/// rigid bodies and the list at `+8`, then runs the base destructor.
pub fn fn_00621dc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_MOPP_VTABLE);
    let count = e.mem.u32(CELL_MOPP_COUNT);
    e.mem.set_u32(CELL_MOPP_COUNT, count.wrapping_sub(1));
    fn_00621e60(e, this);
    fn_00621e40(e, this.byte_add(8));
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00621e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the destructor of the array object at `+8` (`00622780`: it sets
/// vtable `0104dc94` and frees the buffer at `+4`; the engine map's
/// `~basic_streambuf<>` is a wrong library match).
pub fn fn_00621e40(e: &mut Engine, this: Ptr) {
    e.call(LIST_DESTRUCT, &args![this]);
}

// Translated from 00621e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the bodies from the owner (`fn_00621e80`), then empties the array
/// at `+8` (`RemoveAll`, engine map `NiTArray<NiPointer<BSTempEffect>...>::RemoveAll`:
/// the name is a folded-code match, the array element type is unconfirmed).
pub fn fn_00621e60(e: &mut Engine, this: Ptr) {
    fn_00621e80(e, this);
    e.call(LIST_REMOVE_ALL, &args![this.byte_add(8)]);
}

// Translated from 00621e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each entry of the array at `+8`, takes the pointer the entry holds
/// and, when it is not null, hands it to the owner found by `fn_00621ef0`
/// (`005380d0`). Does nothing when there is no owner.
pub fn fn_00621e80(e: &mut Engine, this: Ptr) {
    let owner = fn_00621ef0(e, this);
    if owner == 0 {
        return;
    }
    let list = this.byte_add(8);
    let mut index = 0u32;
    loop {
        let count = e.call(LIST_COUNT, &args![list]).u32();
        if index >= count {
            break;
        }
        let entry = e.call(LIST_AT, &args![list, index]).u32();
        let body = e.call(POINTER_GET, &args![entry]).u32();
        if body != 0 {
            e.call(REMOVE_BODY, &args![owner, body]);
        }
        index += 1;
    }
}

// Translated from 00621ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The owner of the bodies: 0 when the array at `+8` is empty or its first
/// entry holds nothing; otherwise `00621480` applied to what the first
/// body's virtual function at `+0x94` returns.
pub fn fn_00621ef0(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.byte_add(8);
    if e.call(LIST_COUNT, &args![list]).u32() == 0 {
        return 0;
    }
    let first = e.call(LIST_AT, &args![list, 0u32]).u32();
    if e.call(POINTER_GET, &args![first]).u32() == 0 {
        return 0;
    }
    let first = e.call(LIST_AT, &args![list, 0u32]).u32();
    let body = e.call(POINTER_GET, &args![first]).u32();
    let value = e.vcall(body, 0x94, &args![]).u32();
    e.call(OWNER_LOOKUP, &args![value]).u32()
}

// Translated from 00621f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00621f60 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `CellMopp::Create` (Xbox PDB): returns false when the array at `+8`
/// already has entries (`00622470`). Otherwise, when `av_objects` is not
/// null, constructs one rigid-body cinfo and, for each non-null entry of the
/// `count`-element array `av_objects`, builds a `bhkTriSampledHeightField
/// BvTreeShape` from a 0x11 by 0x11 sample grid (extents from `004a3e90`,
/// scale `scale`), makes a `bhkRigidBody` from the cinfo carrying that shape,
/// attaches the entry to the body (`bhkWorldObject::SetAVObject`) and stores
/// the body in the array at `+8` (`fn_006227b0`); the cinfo is destroyed
/// after the loop. Returns true.
pub fn cell_mopp_create(
    e: &mut Engine,
    this: Ptr,
    av_objects: u32,
    count: u32,
    scale: f32,
) -> bool {
    if e.call(HAS_ENTRIES, &args![this]).bool() {
        return false;
    }
    if av_objects == 0 {
        return true;
    }
    // The game's stack frame: `EBP` is `frame + 0x190`; the objects sit at
    // their `EBP`-relative offsets.
    e.with_stack(0x190, |e, frame| {
        let ebp = frame.addr() + 0x190;
        let cinfo = ebp - 0x100;
        let color_word = ebp - 0x104;
        let shape_cinfo = ebp - 0x160;
        let extents = ebp - 0x150;
        let shape_pointer = ebp - 0x170;
        let body_pointer = ebp - 0x184;

        e.call(CINFO_CONSTRUCT, &args![cinfo]);
        e.call(CINFO_SET_FIRST_FLOAT, &args![cinfo, 0.0f32]);
        e.call(CINFO_SET_SMALL_INTEGER, &args![cinfo, 5u32]);
        let lookup = e.call(SETTING_VALUE_ADDRESS, &args![SETTING_OBJECT]).u32();
        let lookup_value = x87_quiet_bits(e.mem.u32(lookup));
        e.call(CINFO_SET_SECOND_FLOAT, &args![cinfo, lookup_value]);
        fn_00622290(e, Ptr::new(color_word), 0x11, 1, 0);
        let color = e.call(POINTER_GET, &args![color_word]).u32();
        e.call(CINFO_SET_PACKED_WORD, &args![cinfo, color]);

        for index in 0..count {
            let object = e.mem.u32(av_objects + index * 4);
            if object == 0 {
                continue;
            }
            e.call(SHAPE_CINFO_CONSTRUCT, &args![shape_cinfo]);
            e.mem.set_u32(ebp - 0x13c, 0x11);
            e.mem.set_u32(ebp - 0x140, 0x11);
            let base = x87_quiet_bits(e.mem.u32(EXTENT_BASE));
            let first = e.call(SCALE_FLOAT, &args![base]).f32();
            let second = e.call(SCALE_FLOAT, &args![1.0f32]).f32();
            let third = e.call(SCALE_FLOAT, &args![base]).f32();
            e.call(SET_VECTOR4, &args![extents, third, second, first, 0.0f32]);
            let form = e.call(GET_OBJECT_FORM, &args![object]).u32();
            e.mem.set_u32(ebp - 0x120, form);
            e.mem.set_u32(ebp - 0x11c, x87_quiet_bits(scale.to_bits()));
            e.call(EXTENTS_UPDATE_RANGE, &args![extents]);

            let shape_memory = e.call(ALLOCATE, &args![0x14u32]).u32();
            let shape = if shape_memory == 0 {
                0
            } else {
                bhk_tri_sampled_height_field_bv_tree_shape_bhk_tri_sampled_height_field_bv_tree_shape(
                    e,
                    Ptr::new(shape_memory),
                    shape_cinfo,
                )
                .addr()
            };
            e.call(SMART_POINTER_SET, &args![shape_pointer, shape]);
            let held_shape = e.call(POINTER_GET, &args![shape_pointer]).u32();
            let shape_id = e.call(GET_SHAPE_WORD, &args![held_shape]).u32();
            fn_006222f0(e, Ptr::new(cinfo), shape_id);

            let body_memory = e.call(ALLOCATE, &args![0x1cu32]).u32();
            let body = if body_memory == 0 {
                0
            } else {
                e.call(RIGID_BODY_CONSTRUCT, &args![body_memory, cinfo]).u32()
            };
            e.call(SET_AV_OBJECT, &args![body, object, SET_AV_OBJECT_ARGUMENT]);
            e.call(SMART_POINTER_SET, &args![body_pointer, body]);
            fn_006227b0(e, this.byte_add(8), body_pointer);
            e.call(SMART_POINTER_RELEASE, &args![body_pointer]);
            e.call(SMART_POINTER_RELEASE, &args![shape_pointer]);
        }
        e.call(CINFO_DESTRUCT, &args![cinfo]);
    });
    true
}

// Translated from 00622290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs three bytes' worth of values into the word at `this` (see
/// `fn_006222c0`) and returns `this`. The engine map's library name
/// (`CStringT<>`) is a wrong match; the body is the three-argument packer.
pub fn fn_00622290(e: &mut Engine, this: Ptr, low: u32, middle: u32, high: u32) -> Ptr {
    fn_006222c0(e, this, low, middle, high);
    this
}

// Translated from 006222c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `(middle << 16) | (high << 8) | low` at `this`, the argument order
/// being the stack order of the call.
pub fn fn_006222c0(e: &mut Engine, this: Ptr, low: u32, middle: u32, high: u32) {
    e.mem
        .set_u32(this.addr(), (middle << 16) | (high << 8) | low);
}

// Translated from 006222f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+4` and at `+0x24` of a rigid-body cinfo (the Xbox
/// PDB's `bhkRigidBodyCinfo` has `m_shape` at `+4` and the Havok cinfo's
/// shape at `+0x24`, the shape pointer written to both).
pub fn fn_006222f0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 4, value);
    e.mem.set_u32(this.addr() + 0x24, value);
}

// Translated from 00622310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622310 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::bhkTriSampledHeightFieldBvTreeShape`
/// (Xbox PDB): the `bhkBvTreeShape` base constructor, the derived vtable,
/// `Init(cinfo)` and the instance counter. Returns `this`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_bhk_tri_sampled_height_field_bv_tree_shape(
    e: &mut Engine,
    this: Ptr,
    cinfo: u32,
) -> Ptr {
    fn_00622390(e, this);
    e.mem.set_u32(this.addr(), TRI_SAMPLED_SHAPE_VTABLE);
    e.call(TRI_SAMPLED_SHAPE_INIT, &args![this, cinfo]);
    let count = e.mem.u32(TRI_SAMPLED_SHAPE_COUNT);
    e.mem
        .set_u32(TRI_SAMPLED_SHAPE_COUNT, count.wrapping_add(1));
    this
}

// Translated from 00622390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkBvTreeShape` constructor (vtable `0104dbac`, RTTI confirmed):
/// the `bhkRefObject` constructor, the vtable and the instance counter.
/// Returns `this`.
pub fn fn_00622390(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BV_TREE_SHAPE_VTABLE);
    let count = e.mem.u32(BV_TREE_SHAPE_COUNT);
    e.mem.set_u32(BV_TREE_SHAPE_COUNT, count.wrapping_add(1));
    this
}

// Translated from 006223c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::GetRTTI` (Xbox PDB): the class's `NiRTTI` record.
pub fn bhk_bv_tree_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BV_TREE_SHAPE_RTTI
}

// Translated from 006223d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::FindShapeCollection` (Xbox PDB): forwards to `00577190`
/// and returns its result.
pub fn bhk_bv_tree_shape_find_shape_collection(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIND_SHAPE_COLLECTION, &args![this]).u32()
}

// Translated from 006223f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the 0x14-byte object.
/// Returns `this`.
pub fn bhk_bv_tree_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BV_TREE_SHAPE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00622420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::GetRTTI` (Xbox PDB): the class's
/// `NiRTTI` record.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    TRI_SAMPLED_SHAPE_RTTI
}

// Translated from 00622440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor and, when bit 0 of `flags` is set, frees the
/// 0x14-byte object. Returns `this`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(TRI_SAMPLED_SHAPE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 006227b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `006228e0(this, index, value)` with the `u16` at `this + 0xa` as the
/// index (`006228e0` makes room when the `u16` at `+8` is not above the index,
/// then stores `value` through `00622920`) and returns its result.
pub fn fn_006227b0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let index = u32::from(e.mem.u16(this.addr() + 0xa));
    e.call(LIST_SET_AT, &args![this, index, value]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00621d90,
            cell_mopp_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00621dc0, fn_00621dc0(Ptr)),
        entry!(0x00621e40, fn_00621e40(Ptr)),
        entry!(0x00621e60, fn_00621e60(Ptr)),
        entry!(0x00621e80, fn_00621e80(Ptr)),
        entry!(0x00621ef0, fn_00621ef0(Ptr) -> u32),
        entry!(0x00621f60, cell_mopp_create(Ptr, u32, u32, f32) -> bool),
        entry!(0x00622290, fn_00622290(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006222c0, fn_006222c0(Ptr, u32, u32, u32)),
        entry!(0x006222f0, fn_006222f0(Ptr, u32)),
        entry!(
            0x00622310,
            bhk_tri_sampled_height_field_bv_tree_shape_bhk_tri_sampled_height_field_bv_tree_shape(
                Ptr, u32
            ) -> Ptr
        ),
        entry!(0x00622390, fn_00622390(Ptr) -> Ptr),
        entry!(0x006223c0, bhk_bv_tree_shape_get_rtti(Ptr) -> u32),
        entry!(
            0x006223d0,
            bhk_bv_tree_shape_find_shape_collection(Ptr) -> u32
        ),
        entry!(
            0x006223f0,
            bhk_bv_tree_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00622420,
            bhk_tri_sampled_height_field_bv_tree_shape_get_rtti(Ptr) -> u32
        ),
        entry!(
            0x00622440,
            bhk_tri_sampled_height_field_bv_tree_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006227b0, fn_006227b0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the pages the unit's globals live on, and a call log.
    fn mopp_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_e000, 0x011c_c000, 0x0126_8000] {
            e.map(page, 0x1000);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn logged(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .collect()
    }

    /// The logged calls to `addr`, as argument lists.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_the_flag() {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = mopp_engine();
            let this: Ptr = Ptr::new(e.mem.alloc(0x18));
            e.register(LIST_COUNT, |_, _| ret(0));
            for addr in [LIST_REMOVE_ALL, LIST_DESTRUCT, BASE_DESTRUCT, FREE] {
                e.register(addr, |_, _| ret(0));
            }
            let result = e.call(0x0062_1d90, &args![this, flags]).ptr::<()>();
            assert_eq!(result.addr(), this.addr());
            assert_eq!(
                calls_to(&e, FREE),
                if frees {
                    vec![vec![this.addr(), 0x18]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn destructor_restores_the_vtable_counts_down_and_runs_the_parts_in_order() {
        let mut e = mopp_engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        e.set_global(CELL_MOPP_COUNT, 5u32);
        e.register(LIST_COUNT, |_, _| ret(0));
        for addr in [LIST_REMOVE_ALL, LIST_DESTRUCT, BASE_DESTRUCT] {
            e.register(addr, |_, _| ret(0));
        }
        e.call(0x0062_1dc0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0104_dab4);
        assert_eq!(e.global::<u32>(CELL_MOPP_COUNT), 4);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &vec![
                (0x0062_1dc0, vec![this.addr()]),
                (LIST_COUNT, vec![this.addr() + 8]),
                (LIST_REMOVE_ALL, vec![this.addr() + 8]),
                (LIST_DESTRUCT, vec![this.addr() + 8]),
                (BASE_DESTRUCT, vec![this.addr()]),
            ]
        );
    }

    #[test]
    fn array_destructor_wrapper_forwards_this() {
        let mut e = mopp_engine();
        e.register(LIST_DESTRUCT, |_, _| ret(0));
        e.call(0x0062_1e40, &args![0x5000u32]);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &vec![(0x0062_1e40, vec![0x5000]), (LIST_DESTRUCT, vec![0x5000])]
        );
    }

    #[test]
    fn remove_all_first_removes_bodies_then_empties_the_array() {
        let mut e = mopp_engine();
        e.register(LIST_COUNT, |_, _| ret(0));
        e.register(LIST_REMOVE_ALL, |_, _| ret(0));
        e.call(0x0062_1e60, &args![0x5000u32]);
        // An empty array has no owner, so only the emptying follows.
        assert_eq!(logged(&e), vec![0x0062_1e60, LIST_COUNT, LIST_REMOVE_ALL]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![0x5008]]);
    }

    /// An owner lookup for a list of three entries: entry `n` is the address
    /// `0x1000 + n`; `POINTER_GET` of it is the body `0x2000 + n`, nothing
    /// for entry 1; the first body's virtual function at `+0x94` returns 0x30.
    fn owner_with_three_entries(e: &mut Engine) {
        e.register(LIST_COUNT, |_, _| ret(3));
        e.register(LIST_AT, |_, a| ret(0x1000 + a[1]));
        e.register(POINTER_GET, |e, a| match a[0] {
            0x1001 => ret(0),
            0x1000 | 0x1002 => ret(0x2000 + a[0] - 0x1000),
            other => ret(e.mem.u32(other)),
        });
        e.map(0x2000, 0x1000);
        e.mem.set_u32(0x2000, 0x4000);
        e.put_vtable(0x4000, &[0; 0x30]);
        e.mem.set_u32(0x4000 + 0x94, 0x6000);
        e.register(0x6000, |_, _| ret(0x30));
        e.register(OWNER_LOOKUP, |_, a| ret(a[0] + 1));
    }

    #[test]
    fn bodies_are_handed_to_the_owner_except_empty_entries() {
        let mut e = mopp_engine();
        owner_with_three_entries(&mut e);
        e.register(REMOVE_BODY, |_, _| ret(0));
        e.call(0x0062_1e80, &args![0x5000u32]);
        // Owner 0x31; entries 0 and 2 hold bodies 0x2000 and 0x2002.
        assert_eq!(
            calls_to(&e, REMOVE_BODY),
            vec![vec![0x31, 0x2000], vec![0x31, 0x2002]]
        );
    }

    #[test]
    fn owner_is_zero_without_a_first_body() {
        let mut e = mopp_engine();
        e.register(LIST_COUNT, |_, _| ret(0));
        assert_eq!(e.call(0x0062_1ef0, &args![0x5000u32]).u32(), 0);

        // One entry, but it holds nothing.
        let mut e = mopp_engine();
        e.register(LIST_COUNT, |_, _| ret(1));
        e.register(LIST_AT, |_, _| ret(0x1001));
        e.register(POINTER_GET, |_, _| ret(0));
        assert_eq!(e.call(0x0062_1ef0, &args![0x5000u32]).u32(), 0);
    }

    #[test]
    fn owner_comes_from_the_first_bodys_virtual_function() {
        let mut e = mopp_engine();
        owner_with_three_entries(&mut e);
        e.register(OWNER_LOOKUP, |_, a| ret(a[0] * 2));
        assert_eq!(e.call(0x0062_1ef0, &args![0x5000u32]).u32(), 0x60);
        assert_eq!(
            calls_to(&e, LIST_AT),
            vec![vec![0x5008, 0], vec![0x5008, 0]]
        );
    }

    /// Doubles for everything `Create` calls.
    fn create_doubles(e: &mut Engine) {
        e.register(HAS_ENTRIES, |e, a| ret(e.mem.u32(a[0] + 8)));
        for addr in [
            CINFO_CONSTRUCT,
            CINFO_DESTRUCT,
            CINFO_SET_FIRST_FLOAT,
            CINFO_SET_SMALL_INTEGER,
            CINFO_SET_SECOND_FLOAT,
            CINFO_SET_PACKED_WORD,
            SET_VECTOR4,
            SHAPE_CINFO_CONSTRUCT,
            EXTENTS_UPDATE_RANGE,
            TRI_SAMPLED_SHAPE_INIT,
            REF_OBJECT_CONSTRUCT,
            SMART_POINTER_RELEASE,
            SET_AV_OBJECT,
            LIST_SET_AT,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(SETTING_VALUE_ADDRESS, |_, _| ret(0x011c_c1e0));
        e.register(SCALE_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]) * 2.0),
            ..Ret::default()
        });
        e.register(GET_OBJECT_FORM, |_, a| ret(a[0] + 0x100));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(SMART_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(GET_SHAPE_WORD, |_, a| ret(a[0] + 1));
        e.register(RIGID_BODY_CONSTRUCT, |_, a| ret(a[0]));
    }

    #[test]
    fn create_does_nothing_when_the_array_already_has_entries() {
        let mut e = mopp_engine();
        create_doubles(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        e.mem.set_u32(this.addr() + 8, 1);
        let done = e.call(0x0062_1f60, &args![this, 0x7000u32, 1u32, 1.0f32]);
        assert!(!done.bool());
        assert_eq!(logged(&e), vec![0x0062_1f60, HAS_ENTRIES]);
    }

    #[test]
    fn create_without_objects_returns_true_and_builds_nothing() {
        let mut e = mopp_engine();
        create_doubles(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        let done = e.call(0x0062_1f60, &args![this, 0u32, 3u32, 1.0f32]);
        assert!(done.bool());
        assert_eq!(logged(&e), vec![0x0062_1f60, HAS_ENTRIES]);
    }

    #[test]
    fn create_builds_a_body_for_each_object_and_skips_null_entries() {
        let mut e = mopp_engine();
        create_doubles(&mut e);
        e.set_global(EXTENT_BASE, 3.0f32);
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        let objects = e.mem.alloc(8);
        e.mem.set_u32(objects, 0);
        e.mem.set_u32(objects + 4, 0x9000);
        let done = e.call(0x0062_1f60, &args![this, objects, 2u32, 0.5f32]);
        assert!(done.bool());
        assert_eq!(
            logged(&e),
            vec![
                0x0062_1f60,
                HAS_ENTRIES,
                CINFO_CONSTRUCT,
                CINFO_SET_FIRST_FLOAT,
                CINFO_SET_SMALL_INTEGER,
                SETTING_VALUE_ADDRESS,
                CINFO_SET_SECOND_FLOAT,
                POINTER_GET,
                CINFO_SET_PACKED_WORD,
                // The only object, 0x9000.
                SHAPE_CINFO_CONSTRUCT,
                SCALE_FLOAT,
                SCALE_FLOAT,
                SCALE_FLOAT,
                SET_VECTOR4,
                GET_OBJECT_FORM,
                EXTENTS_UPDATE_RANGE,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                TRI_SAMPLED_SHAPE_INIT,
                SMART_POINTER_SET,
                POINTER_GET,
                GET_SHAPE_WORD,
                ALLOCATE,
                RIGID_BODY_CONSTRUCT,
                SET_AV_OBJECT,
                SMART_POINTER_SET,
                LIST_SET_AT,
                SMART_POINTER_RELEASE,
                SMART_POINTER_RELEASE,
                CINFO_DESTRUCT,
            ]
        );
        // The extents vector gets the scaled values in reverse order and a
        // zero: scale(3.0) = 6.0, scale(1.0) = 2.0, scale(3.0) = 6.0.
        let vector = &calls_to(&e, SET_VECTOR4)[0];
        assert_eq!(
            vector[1..],
            [6.0f32.to_bits(), 2.0f32.to_bits(), 6.0f32.to_bits(), 0]
        );
        assert_eq!(calls_to(&e, SCALE_FLOAT)[1], vec![1.0f32.to_bits()]);
        // The vector sits at EBP - 0x150 and the rigid-body cinfo at EBP - 0x100.
        let cinfo = calls_to(&e, CINFO_CONSTRUCT)[0][0];
        assert_eq!(vector[0] + 0x50, cinfo);
        // The packed word goes to the cinfo; the object, the form and the
        // scale land in the shape cinfo's frame (EBP - 0x120 and - 0x11c).
        assert_eq!(
            calls_to(&e, CINFO_SET_PACKED_WORD),
            vec![vec![cinfo, 0x0001_0011]]
        );
        assert_eq!(e.mem.u32(vector[0] + 0x30), 0x9100);
        assert_eq!(e.mem.f32(vector[0] + 0x34), 0.5);
        assert_eq!(e.mem.u32(vector[0] + 0x10), 0x11);
        assert_eq!(e.mem.u32(vector[0] + 0x14), 0x11);
        assert_eq!(calls_to(&e, GET_OBJECT_FORM), vec![vec![0x9000]]);
        let body = calls_to(&e, RIGID_BODY_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, SET_AV_OBJECT),
            vec![vec![body, 0x9000, SET_AV_OBJECT_ARGUMENT]]
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x14], vec![0x1c]]);
        // The body goes into the array at +8 through its pointer cell.
        let insert = &calls_to(&e, LIST_SET_AT)[0];
        assert_eq!(insert[0], this.addr() + 8);
        assert_eq!(insert[1], 0, "the u16 at +0xa");
        // The cell held the body when it was handed over.
        assert_eq!(e.mem.u32(insert[2]), body);
    }

    #[test]
    fn create_passes_the_shape_to_the_cinfo_at_both_offsets() {
        let mut e = mopp_engine();
        create_doubles(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        let objects = e.mem.alloc(4);
        e.mem.set_u32(objects, 0x9000);
        e.call(0x0062_1f60, &args![this, objects, 1u32, 1.0f32]);
        let cinfo = calls_to(&e, CINFO_CONSTRUCT)[0][0];
        // The double of the shape-word getter returns its argument plus one,
        // the argument being the shape the constructor built.
        let shape = calls_to(&e, TRI_SAMPLED_SHAPE_INIT)[0][0];
        assert_eq!(e.mem.u32(cinfo + 4), shape + 1);
        assert_eq!(e.mem.u32(cinfo + 0x24), shape + 1);
    }

    #[test]
    fn packer_builds_the_word_from_three_values() {
        let mut e = mopp_engine();
        let word = e.mem.alloc(4);
        let result = e.call(0x0062_2290, &args![word, 0x11u32, 1u32, 0u32]);
        assert_eq!(result.u32(), word);
        assert_eq!(e.mem.u32(word), 0x0001_0011);
        e.call(0x0062_22c0, &args![word, 0xaau32, 0xbbu32, 0xccu32]);
        assert_eq!(e.mem.u32(word), 0x00bb_ccaa);
    }

    #[test]
    fn cinfo_shape_setter_writes_both_words() {
        let mut e = mopp_engine();
        let cinfo = e.mem.alloc(0xe0);
        e.call(0x0062_22f0, &args![cinfo, 0x1234u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x1234);
    }

    #[test]
    fn height_field_shape_constructor_chains_base_vtable_init_and_counter() {
        let mut e = mopp_engine();
        e.set_global(TRI_SAMPLED_SHAPE_COUNT, 7u32);
        e.set_global(BV_TREE_SHAPE_COUNT, 1u32);
        e.register(REF_OBJECT_CONSTRUCT, |_, _| ret(0));
        e.register(TRI_SAMPLED_SHAPE_INIT, |e, a| ret(e.mem.u32(a[0])));
        let this = e.mem.alloc(0x14);
        let result = e.call(0x0062_2310, &args![this, 0xc1c1u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_dac4);
        assert_eq!(e.global::<u32>(TRI_SAMPLED_SHAPE_COUNT), 8);
        assert_eq!(e.global::<u32>(BV_TREE_SHAPE_COUNT), 2);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &vec![
                (0x0062_2310, vec![this, 0xc1c1]),
                (REF_OBJECT_CONSTRUCT, vec![this]),
                (TRI_SAMPLED_SHAPE_INIT, vec![this, 0xc1c1]),
            ]
        );
    }

    #[test]
    fn bv_tree_shape_constructor_sets_vtable_and_counts() {
        let mut e = mopp_engine();
        e.set_global(BV_TREE_SHAPE_COUNT, 0xffff_ffffu32);
        e.register(REF_OBJECT_CONSTRUCT, |_, _| ret(0));
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0062_2390, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_dbac);
        assert_eq!(e.global::<u32>(BV_TREE_SHAPE_COUNT), 0, "wraps like ADD");
    }

    #[test]
    fn rtti_getters_return_the_records() {
        let mut e = mopp_engine();
        assert_eq!(e.call(0x0062_23c0, &args![0u32]).u32(), 0x0126_82d0);
        assert_eq!(e.call(0x0062_2420, &args![0u32]).u32(), 0x0126_8350);
    }

    #[test]
    fn find_shape_collection_forwards_and_returns_the_result() {
        let mut e = mopp_engine();
        e.register(FIND_SHAPE_COLLECTION, |_, a| ret(a[0] + 9));
        assert_eq!(e.call(0x0062_23d0, &args![0x40u32]).u32(), 0x49);
    }

    #[test]
    fn shape_scalar_deleting_destructors_free_twenty_bytes_on_request() {
        for (addr, destruct) in [
            (0x0062_23f0u32, BV_TREE_SHAPE_DESTRUCT),
            (0x0062_2440, TRI_SAMPLED_SHAPE_DESTRUCT),
        ] {
            for flags in [0u32, 1] {
                let mut e = mopp_engine();
                e.register(destruct, |_, _| ret(0));
                e.register(FREE, |_, _| ret(0));
                let result = e.call(addr, &args![0x80u32, flags]);
                assert_eq!(result.u32(), 0x80);
                let mut expected = vec![(addr, vec![0x80, flags]), (destruct, vec![0x80])];
                if flags == 1 {
                    expected.push((FREE, vec![0x80, 0x14]));
                }
                assert_eq!(e.call_log.as_ref().unwrap(), &expected);
            }
        }
    }

    #[test]
    fn insert_passes_the_index_word_at_plus_ten() {
        let mut e = mopp_engine();
        e.register(LIST_SET_AT, |_, a| ret(a[1]));
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 0xa, 0x1234);
        let result = e.call(0x0062_27b0, &args![array, 0xabcu32]);
        assert_eq!(result.u32(), 0x1234);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (LIST_SET_AT, vec![array, 0x1234, 0xabc])
        );
    }
}
