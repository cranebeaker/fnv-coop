//! `Havok SDK/hkdataobject.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The classes of this unit are thin handles. `hkDataClass`, `hkDataObject` and `hkDataArray`
//! are one word, the pointer to their implementation object (`this + 0`); the implementation
//! is a reference-counted object (16-bit count at `+6`, 32-bit count at `+8`) and everything
//! the handles do is a call through its vtable. `hkDataObject::Value` and `hkDataArray::Value`
//! are two words, the implementation (`this + 0`) and the member or element selector
//! (`this + 4`) handed to it. Slot numbers below are byte offsets into the implementation's
//! vtable, exactly as the code indexes it; what a slot does is only named where the handle's
//! own name says it.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

/// Adds a reference to a handle's implementation (`00dba760`, `00dbab50`): the 16-bit count at
/// `+6` and the 32-bit count at `+8` both go up by one, when there is an implementation.
fn add_reference(e: &mut Engine, implementation: u32) {
    if implementation != 0 {
        let small = e.mem.u16(implementation + 6).wrapping_add(1);
        e.mem.set_u16(implementation + 6, small);
        let big = e.mem.u32(implementation + 8).wrapping_add(1);
        e.mem.set_u32(implementation + 8, big);
    }
}

/// Drops a reference of a handle's implementation (`00dba7f0`, `00dbab70`): both counts go
/// down by one and, when the 32-bit count reaches zero, the implementation's deleting
/// destructor (slot 0) runs with flag 1.
fn release_reference(e: &mut Engine, implementation: u32) {
    if implementation != 0 {
        let small = e.mem.u16(implementation + 6).wrapping_add(0xffff);
        e.mem.set_u16(implementation + 6, small);
        let big = e.mem.u32(implementation + 8).wrapping_add(0xffff_ffff);
        e.mem.set_u32(implementation + 8, big);
        if big == 0 {
            e.vcall(implementation, 0, &args![1u32]);
        }
    }
}

// Translated from 00dba650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::hkDataClass` (Xbox PDB): stores the implementation pointer at `this + 0`
/// (no reference is taken). Returns `this`.
pub fn hk_data_class_constructor(e: &mut Engine, this: Ptr, implementation: u32) -> Ptr {
    e.mem.set_u32(this.addr(), implementation);
    this
}

// Translated from 00dba660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::~hkDataClass` (Xbox PDB): does nothing (the exe's body is a bare `RET`).
pub fn hk_data_class_destructor(_e: &mut Engine, _this: Ptr) {}

// Translated from 00dba670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::isNull` (Xbox PDB): whether the implementation pointer at `this + 0` is null.
pub fn hk_data_class_is_null(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) == 0
}

// Translated from 00dba680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getName` (Xbox PDB): the implementation's slot `0x8` called with no
/// arguments; its result is returned.
pub fn hk_data_class_get_name(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x8, &args![]).u32()
}

// Translated from 00dba690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getVersion` (Xbox PDB): the implementation's slot `0xc` called with no
/// arguments; its result is returned.
pub fn hk_data_class_get_version(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0xc, &args![]).u32()
}

// Translated from 00dba6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getParent` (Xbox PDB): builds the parent class handle in `out` from the
/// implementation's slot `0x10` result (`00dba650`). Returns `out`.
pub fn hk_data_class_get_parent(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let parent = e.vcall(implementation, 0x10, &args![]).u32();
    hk_data_class_constructor(e, out, parent)
}

// Translated from 00dba6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getWorld` (Xbox PDB): the implementation's slot `0x4` called with no
/// arguments; its result is returned.
pub fn hk_data_class_get_world(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x4, &args![]).u32()
}

// Translated from 00dba6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getNumDeclaredMembers` (Xbox PDB): the implementation's slot `0x18` called
/// with no arguments; its count is returned.
pub fn hk_data_class_get_num_declared_members(e: &mut Engine, this: Ptr) -> i32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x18, &args![]).i32()
}

// Translated from 00dba720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x28` the two words
/// every caller pushes (an index and the address of a zeroed 0x14-byte record the callee
/// fills, see `00dbaed0`); its result is returned.
pub fn fn_00dba720(e: &mut Engine, this: Ptr, index: u32, record: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x28, &args![index, record]).u32()
}

// Translated from 00dba730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getNumMembers` (Xbox PDB): the implementation's slot `0x24` called with no
/// arguments; its count is returned.
pub fn hk_data_class_get_num_members(e: &mut Engine, this: Ptr) -> i32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x24, &args![]).i32()
}

// Translated from 00dba740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x2c` the one word
/// every caller pushes; its result is returned.
pub fn fn_00dba740(e: &mut Engine, this: Ptr, argument: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x2c, &args![argument]).u32()
}

// Translated from 00dba760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::hkDataObject` (Xbox PDB): stores the implementation pointer at `this + 0`
/// and, when it is not null, adds a reference (16-bit count at `+6`, 32-bit count at `+8`).
/// Returns `this`.
pub fn hk_data_object_constructor(e: &mut Engine, this: Ptr, implementation: u32) -> Ptr {
    e.mem.set_u32(this.addr(), implementation);
    add_reference(e, implementation);
    this
}

// Translated from 00dba780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::hkDataObject` copy constructor (Xbox PDB): takes the implementation pointer
/// of `other` (`other + 0`), stores it at `this + 0` and adds a reference when it is not
/// null. Returns `this`.
pub fn hk_data_object_constructor_ov2(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let implementation = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), implementation);
    add_reference(e, implementation);
    this
}

// Translated from 00dba7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::~hkDataObject` (Xbox PDB): drops the reference on the implementation at
/// `this + 0` when it is not null; at zero the implementation's slot `0` (deleting
/// destructor) runs with flag 1.
pub fn hk_data_object_destructor(e: &mut Engine, this: Ptr) {
    let implementation = e.mem.u32(this.addr());
    release_reference(e, implementation);
}

// Translated from 00dba810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::getHandle` (Xbox PDB): writes the object's two-word handle to `out`. With an
/// implementation the words are those of the record its slot `0x4` returns (the slot gets the
/// address of a scratch record); without one they are zero. Returns `out`.
pub fn hk_data_object_get_handle(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.with_stack(0x10, |e, scratch| {
        let handle = if implementation != 0 {
            e.vcall(implementation, 0x4, &args![scratch.addr() + 8])
                .u32()
        } else {
            scratch.addr()
        };
        let low = e.mem.u32(handle);
        let high = e.mem.u32(handle + 4);
        e.mem.set_u32(out.addr() + 4, high);
        e.mem.set_u32(out.addr(), low);
        out
    })
}

// Translated from 00dba850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::getClass` (Xbox PDB): builds the class handle of the object in `out` from
/// the implementation's slot `0x8` result (`00dba650`). Returns `out`.
pub fn hk_data_object_get_class(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let class = e.vcall(implementation, 0x8, &args![]).u32();
    hk_data_class_constructor(e, out, class)
}

// Translated from 00dba870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::operator_` (engine map name, the operator is unconfirmed): the
/// implementation's slot `0xc` is called with the result record `out` and `key`. Returns
/// `out`.
pub fn hk_data_object_operator(e: &mut Engine, this: Ptr, out: Ptr, key: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0xc, &args![out, key]);
    out
}

// Translated from 00dba890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::operator_` second overload (engine map name `operator__ov2`, the operator
/// is unconfirmed): the same body as `00dba870`, slot `0xc` with `out` and `key`. Returns `out`.
pub fn hk_data_object_operator_ov2(e: &mut Engine, this: Ptr, out: Ptr, key: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0xc, &args![out, key]);
    out
}

// Translated from 00dba8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. The implementation's slot `0x14` called with no arguments
/// (its caller passes the result on to `00dba8c0`, `00dba8d0`, `00dba8e0` and `00dba8f0`:
/// the start of a member iteration); its result is returned.
pub fn fn_00dba8b0(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x14, &args![]).u32()
}

// Translated from 00dba8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x18` the one word
/// every caller pushes (the value `00dba8b0` returned); its result is returned.
pub fn fn_00dba8c0(e: &mut Engine, this: Ptr, iterator: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x18, &args![iterator]).u32()
}

// Translated from 00dba8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x1c` the one word
/// every caller pushes; its result is returned.
pub fn fn_00dba8d0(e: &mut Engine, this: Ptr, iterator: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x1c, &args![iterator]).u32()
}

// Translated from 00dba8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x20` the one word
/// every caller pushes; its result is returned.
pub fn fn_00dba8e0(e: &mut Engine, this: Ptr, iterator: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x20, &args![iterator]).u32()
}

// Translated from 00dba8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. The implementation's slot `0x24` is called with the result
/// record `out` and `iterator` (the callers pass a value `00dba8b0` returned). Returns `out`.
pub fn fn_00dba8f0(e: &mut Engine, this: Ptr, out: Ptr, iterator: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x24, &args![out, iterator]);
    out
}

// Translated from 00dba910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::hasMember` (Xbox PDB): forwards `name` to the implementation's slot `0x10`;
/// its result (all of `EAX`, which callers test as a word) is returned.
pub fn hk_data_object_has_member(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x10, &args![name]).u32()
}

// Translated from 00dba920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::getType` (Xbox PDB): 0 without an implementation (`this + 0`).
/// Otherwise the class comes from the implementation's slot `0x8`, the member's index from the
/// class's slot `0x2c` given the member (`this + 4`), and the class's slot `0x28` fills a
/// zeroed 0x14-byte record from that index; the record's third word (`+8`) is returned.
pub fn hk_data_object_value_get_type(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    if implementation == 0 {
        return 0;
    }
    let class = e.vcall(implementation, 0x8, &args![]).u32();
    let member = e.mem.u32(this.addr() + 4);
    e.with_stack(0x14, |e, record| {
        let index = e.vcall(class, 0x2c, &args![member]).u32();
        e.vcall(class, 0x28, &args![index, record]);
        e.mem.u32(record.addr() + 8)
    })
}

// Translated from 00dba980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::isNull` (Xbox PDB): whether the implementation pointer at `this + 0` is
/// null.
pub fn hk_data_object_is_null(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) == 0
}

// Translated from 00dba990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asInt` (Xbox PDB): the implementation's slot `0x30` called with the
/// member (`this + 4`); its result is returned.
pub fn hk_data_object_value_as_int(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x30, &args![member]).u32()
}

// Translated from 00dba9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asVec` (Xbox PDB): the implementation's slot `0x38` called with the
/// member (`this + 4`) and `count`; its result is returned.
pub fn hk_data_object_value_as_vec(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x38, &args![member, count]).u32()
}

// Translated from 00dba9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asString` (Xbox PDB): the implementation's slot `0x2c` called with
/// the member (`this + 4`); its result is returned.
pub fn hk_data_object_value_as_string(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x2c, &args![member]).u32()
}

// Translated from 00dba9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asObject` (Xbox PDB): builds an object handle in `out` (`00dba760`,
/// which takes a reference) from the implementation's slot `0x34` result for the member
/// (`this + 4`). Returns `out`.
pub fn hk_data_object_value_as_object(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    let object = e.vcall(implementation, 0x34, &args![member]).u32();
    hk_data_object_constructor(e, out, object)
}

// Translated from 00dbaa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asReal` (Xbox PDB): the implementation's slot `0x3c` called with the
/// member (`this + 4`); the value it leaves in `ST0` is returned.
pub fn hk_data_object_value_as_real(e: &mut Engine, this: Ptr) -> f32 {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x3c, &args![member]).f32()
}

// Translated from 00dbaa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map (a `hkDataObject::Value` setter). The implementation's slot
/// `0x5c` is called with the member (`this + 4`) and `value`.
pub fn fn_00dbaa10(e: &mut Engine, this: Ptr, value: u32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x5c, &args![member, value]);
}

// Translated from 00dbaa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::operator_` (engine map name; an assignment of an `int`): the
/// implementation's slot `0x40` is called with the member (`this + 4`) and `value`
/// sign-extended to 64 bits (low word, then high word).
pub fn hk_data_object_value_operator(e: &mut Engine, this: Ptr, value: i32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(
        implementation,
        0x40,
        &args![member, value as u32, (value >> 31) as u32],
    );
}

// Translated from 00dbaa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::operator_` second overload (engine map name `operator__ov2`): reads
/// the first word of the object at `other` and calls the implementation's slot `0x58` with
/// the member (`this + 4`) and that word.
pub fn hk_data_object_value_operator_ov2(e: &mut Engine, this: Ptr, other: Ptr) {
    let referenced = e.mem.u32(other.addr());
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x58, &args![member, referenced]);
}

// Translated from 00dbaa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map (a `hkDataObject::Value` assignment of a 64-bit value). The
/// implementation's slot `0x40` is called with the member (`this + 4`) and the two words
/// `low`, `high`.
pub fn fn_00dbaa90(e: &mut Engine, this: Ptr, low: u32, high: u32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x40, &args![member, low, high]);
}

// Translated from 00dbaab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::operator_` third overload (engine map name `operator__ov3`; an
/// assignment of a `float`): the float goes through the x87 stack (`FLD`/`FSTP`, which quiets
/// a signalling NaN) and the implementation's slot `0x4c` is called with the member
/// (`this + 4`) and the float's bit pattern.
pub fn hk_data_object_value_operator_ov3(e: &mut Engine, this: Ptr, value: f32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(
        implementation,
        0x4c,
        &args![member, x87_quiet_bits(value.to_bits())],
    );
}

// Translated from 00dbab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::setVec` (Xbox PDB): the implementation's slot `0x44` is called with
/// the member (`this + 4`) and the two words `values`, `count`, in that order.
pub fn hk_data_object_value_set_vec(e: &mut Engine, this: Ptr, values: u32, count: u32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x44, &args![member, values, count]);
}

// Translated from 00dbab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::operator_` fourth overload (engine map name `operator__ov4`): the
/// implementation's slot `0x50` is called with the member (`this + 4`) and `value`.
pub fn hk_data_object_value_operator_ov4(e: &mut Engine, this: Ptr, value: u32) {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x50, &args![member, value]);
}

// Translated from 00dbab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::hkDataArray` (Xbox PDB): stores the implementation pointer at `this + 0` and,
/// when it is not null, adds a reference (16-bit count at `+6`, 32-bit count at `+8`).
pub fn hk_data_array_constructor(e: &mut Engine, this: Ptr, implementation: u32) -> Ptr {
    e.mem.set_u32(this.addr(), implementation);
    add_reference(e, implementation);
    this
}

// Translated from 00dbab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::~hkDataArray` (Xbox PDB): drops the reference on the implementation at
/// `this + 0` when it is not null; at zero the implementation's slot `0` (deleting
/// destructor) runs with flag 1.
pub fn hk_data_array_destructor(e: &mut Engine, this: Ptr) {
    let implementation = e.mem.u32(this.addr());
    release_reference(e, implementation);
}

// Translated from 00dbab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::operator_` (engine map name; the operator is unconfirmed): builds a
/// `hkDataArray::Value` in `out`: the implementation (`this + 0`) and `index`. Returns `out`.
pub fn hk_data_array_operator(e: &mut Engine, this: Ptr, out: Ptr, index: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.mem.set_u32(out.addr(), implementation);
    e.mem.set_u32(out.addr() + 4, index);
    out
}

// Translated from 00dbabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::operator_` second overload (engine map name `operator__ov2`): the same body
/// as `00dbab90`, the implementation (`this + 0`) and `index` stored into `out`. Returns `out`.
pub fn hk_data_array_operator_ov2(e: &mut Engine, this: Ptr, out: Ptr, index: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    e.mem.set_u32(out.addr(), implementation);
    e.mem.set_u32(out.addr() + 4, index);
    out
}

// Translated from 00dbabe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::setSize` (Xbox PDB): forwards `size` to the implementation's slot `0x10`;
/// its result is returned.
pub fn hk_data_array_set_size(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x10, &args![size]).u32()
}

// Translated from 00dbac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::getSize` (Xbox PDB): the implementation's slot `0x14` called with no
/// arguments; its result is returned.
pub fn hk_data_array_get_size(e: &mut Engine, this: Ptr) -> i32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x14, &args![]).i32()
}

// Translated from 00dbac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::getType` (Xbox PDB): the implementation's slot `0x4` called with no
/// arguments; its result is returned.
pub fn hk_data_array_get_type(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x4, &args![]).u32()
}

// Translated from 00dbac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::getClass` (Xbox PDB): builds a class handle in `out` from the
/// implementation's slot `0x18` result (`00dba650`). Returns `out`.
pub fn hk_data_array_get_class(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let class = e.vcall(implementation, 0x18, &args![]).u32();
    hk_data_class_constructor(e, out, class)
}

// Translated from 00dbac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::getType` (Xbox PDB): the implementation's slot `0x4` called with no
/// arguments (the element index at `this + 4` is not passed); its result is returned.
pub fn hk_data_array_value_get_type(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x4, &args![]).u32()
}

// Translated from 00dbac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asVec` (Xbox PDB): takes one word that the exe overwrites on its stack
/// (`_unused_1`) with the element at `this + 4` and jumps to the implementation's slot
/// `0x20`, so the callee gets the element only; its result is returned.
pub fn hk_data_array_value_as_vec(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x20, &args![element]).u32()
}

// Translated from 00dbac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asString` (Xbox PDB): the implementation's slot `0x28` called with
/// the element (`this + 4`); its result is returned.
pub fn hk_data_array_value_as_string(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x28, &args![element]).u32()
}

// Translated from 00dbac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asReal` (Xbox PDB): the implementation's slot `0x30` called with the
/// element (`this + 4`); the value it leaves in `ST0` is returned.
pub fn hk_data_array_value_as_real(e: &mut Engine, this: Ptr) -> f32 {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x30, &args![element]).f32()
}

// Translated from 00dbac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asInt` (Xbox PDB): the implementation's slot `0x40` called with the
/// element (`this + 4`); its result is returned.
pub fn hk_data_array_value_as_int(e: &mut Engine, this: Ptr) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x40, &args![element]).u32()
}

// Translated from 00dbaca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asObject` (Xbox PDB): builds an object handle in `out` (`00dba760`,
/// which takes a reference) from the implementation's slot `0x50` result for the element
/// (`this + 4`). Returns `out`.
pub fn hk_data_array_value_as_object(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    let object = e.vcall(implementation, 0x50, &args![element]).u32();
    hk_data_object_constructor(e, out, object)
}

// Translated from 00dbacd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::asArray` (Xbox PDB): builds an array handle in `out` (`00dbab50`,
/// which takes a reference) from the implementation's slot `0x58` result for the element
/// (`this + 4`). Returns `out`.
pub fn hk_data_array_value_as_array(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    let array = e.vcall(implementation, 0x58, &args![element]).u32();
    hk_data_array_constructor(e, out, array)
}

// Translated from 00dbad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::operator_` (engine map name; an assignment from an array handle):
/// reads the first word of the handle at `other` and calls the implementation's slot `0x54`
/// with the element (`this + 4`) and that word.
pub fn hk_data_array_value_operator(e: &mut Engine, this: Ptr, other: Ptr) {
    let referenced = e.mem.u32(other.addr());
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x54, &args![element, referenced]);
}

// Translated from 00dbad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::operator_` second overload (engine map name `operator__ov2`; an
/// assignment of a `float`): the float goes through the x87 stack (`FLD`/`FSTP`, which quiets
/// a signalling NaN) and the implementation's slot `0x34` is called with the element
/// (`this + 4`) and the float's bit pattern.
pub fn hk_data_array_value_operator_ov2(e: &mut Engine, this: Ptr, value: f32) {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(
        implementation,
        0x34,
        &args![element, x87_quiet_bits(value.to_bits())],
    );
}

// Translated from 00dbad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::setVec` (Xbox PDB): the implementation's slot `0x24` is called with
/// the element (`this + 4`) and `values`. The function takes a second word (`RET 8`) that it
/// never reads (`_unused_2`).
pub fn hk_data_array_value_set_vec(e: &mut Engine, this: Ptr, values: u32, _unused_2: u32) {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x24, &args![element, values]);
}

// Translated from 00dbada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::operator_` third overload (engine map name `operator__ov3`): the
/// implementation's slot `0x44` is called with the element (`this + 4`) and `value`.
pub fn hk_data_array_value_operator_ov3(e: &mut Engine, this: Ptr, value: u32) {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x44, &args![element, value]);
}

// Translated from 00dbadc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArray::Value::operator_` fourth overload (engine map name `operator__ov4`): the
/// implementation's slot `0x2c` is called with the element (`this + 4`) and `value`.
pub fn hk_data_array_value_operator_ov4(e: &mut Engine, this: Ptr, value: u32) {
    let implementation = e.mem.u32(this.addr());
    let element = e.mem.u32(this.addr() + 4);
    e.vcall(implementation, 0x2c, &args![element, value]);
}

// Translated from 00dbae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x80` the two words
/// every caller pushes (`first`, `second`); its result is returned.
pub fn fn_00dbae10(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x80, &args![first, second]).u32()
}

// Translated from 00dbae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Forwards to the implementation's slot `0x64` the two words
/// every caller pushes (`first`, `second`); its result is returned.
pub fn fn_00dbae80(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    e.vcall(implementation, 0x64, &args![first, second]).u32()
}

// Translated from 00dbaea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Engine map name `hkDataArray::swizzleObjectMember`, which does not fit the body: builds an
/// array handle in `out` (`00dbab50`, which takes a reference) from the implementation's slot
/// `0x1c` result for `argument`. Returns `out`.
pub fn fn_00dbaea0(e: &mut Engine, this: Ptr, out: Ptr, argument: u32) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let array = e.vcall(implementation, 0x1c, &args![argument]).u32();
    hk_data_array_constructor(e, out, array)
}

// Translated from 00dbaed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getAllMemberInfo` (Xbox PDB): for each of the class's members
/// (`getNumMembers`, `00dba730`) calls the implementation's slot `0x28` with the member index
/// and the address of that member's 0x14-byte record in the array whose first word is at
/// `*records`.
pub fn hk_data_class_get_all_member_info(e: &mut Engine, this: Ptr, records: Ptr) {
    let count = hk_data_class_get_num_members(e, this);
    let mut index = 0i32;
    while index < count {
        let base = e.mem.u32(records.addr());
        let implementation = e.mem.u32(this.addr());
        let record = base.wrapping_add((index as u32).wrapping_mul(0x14));
        e.vcall(implementation, 0x28, &args![index, record]);
        index += 1;
    }
}

// Translated from 00dbaf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getAllDeclaredMemberInfo` (Xbox PDB): for each of the class's declared
/// members (`getNumDeclaredMembers`, `00dba6f0`) calls the implementation's slot `0x1c` with
/// the member index and the address of that member's 0x14-byte record in the array whose
/// first word is at `*records`.
pub fn hk_data_class_get_all_declared_member_info(e: &mut Engine, this: Ptr, records: Ptr) {
    let count = hk_data_class_get_num_declared_members(e, this);
    let mut index = 0i32;
    while index < count {
        let base = e.mem.u32(records.addr());
        let implementation = e.mem.u32(this.addr());
        let record = base.wrapping_add((index as u32).wrapping_mul(0x14));
        e.vcall(implementation, 0x1c, &args![index, record]);
        index += 1;
    }
}

// Translated from 00dbaf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map (the equality of two `hkDataObject`s): whether the two-word
/// handles (`00dba810`) of `this` and of the object at `other` are equal.
pub fn fn_00dbaf50(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.with_stack(0x10, |e, scratch| {
        hk_data_object_get_handle(e, this, scratch);
        hk_data_object_get_handle(e, other, scratch.byte_add(8));
        e.mem.u32(scratch.addr()) == e.mem.u32(scratch.addr() + 8)
            && e.mem.u32(scratch.addr() + 4) == e.mem.u32(scratch.addr() + 12)
    })
}

// Translated from 00dbaf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map (the inequality of two `hkDataObject`s): the negation of
/// `00dbaf50` for `this` and the object at `other`.
pub fn fn_00dbaf90(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    !fn_00dbaf50(e, this, other)
}

// Translated from 00dbafb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObject::Value::asArray` (Xbox PDB): builds an array handle in `out` (`00dbab50`,
/// which takes a reference) from the implementation's slot `0x28` result for the member
/// (`this + 4`). Returns `out`.
pub fn hk_data_object_value_as_array(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let implementation = e.mem.u32(this.addr());
    let member = e.mem.u32(this.addr() + 4);
    let array = e.vcall(implementation, 0x28, &args![member]).u32();
    hk_data_array_constructor(e, out, array)
}

// Translated from 00dbafe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClass::getCinfo` (Xbox PDB): fills the class description at `out`: `+0` the name
/// (implementation slot `0x8`), `+4` the version (slot `0xc`), `+8` the parent's name (the
/// parent from slot `0x10`, then its own slot `0x8`; 0 without a parent) and, from `+0xc`,
/// a `hkArray` of 16-byte member records (data `+0xc`, size `+0x10`, capacity and flags
/// `+0x14`). The array is reserved (`hkArrayUtil::_reserve`, `00c908c0`, through the memory
/// router's allocator) to the larger of twice its capacity and the member count when it is
/// too small, its size set to `getNumDeclaredMembers` (slot `0x18`), and each record `i`
/// built from the 0x14-byte info slot `0x1c` writes: info words `0`, `2`, `4` go to record
/// words `0`, `1`, `2`, and record word `3` is the slot `0x8` result of the object at info
/// word `3` (0 when null).
pub fn hk_data_class_get_cinfo(e: &mut Engine, this: Ptr, out: Ptr) {
    let o = out.addr();
    let implementation = e.mem.u32(this.addr());
    let name = e.vcall(implementation, 0x8, &args![]).u32();
    e.mem.set_u32(o, name);
    let implementation = e.mem.u32(this.addr());
    let version = e.vcall(implementation, 0xc, &args![]).u32();
    e.mem.set_u32(o + 4, version);
    let implementation = e.mem.u32(this.addr());
    let parent = e.vcall(implementation, 0x10, &args![]).u32();
    let parent_name = if parent != 0 {
        let implementation = e.mem.u32(this.addr());
        let parent = e.vcall(implementation, 0x10, &args![]).u32();
        e.vcall(parent, 0x8, &args![]).u32()
    } else {
        0
    };
    e.mem.set_u32(o + 8, parent_name);
    let implementation = e.mem.u32(this.addr());
    let count = e.vcall(implementation, 0x18, &args![]).i32();
    let array = o + 0xc;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity + capacity;
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    e.mem.set_u32(array + 4, count as u32);
    let mut index = 0i32;
    let mut offset = 0u32;
    while index < count {
        let implementation = e.mem.u32(this.addr());
        e.with_stack(0x14, |e, info| {
            e.vcall(implementation, 0x1c, &args![index, info]);
            let record = e.mem.u32(array).wrapping_add(offset);
            let object = e.mem.u32(info.addr() + 0xc);
            let object_name = if object != 0 {
                e.vcall(object, 0x8, &args![]).u32()
            } else {
                0
            };
            let word0 = e.mem.u32(info.addr());
            let word1 = e.mem.u32(info.addr() + 8);
            let word2 = e.mem.u32(info.addr() + 0x10);
            e.mem.set_u32(record + 4, word1);
            e.mem.set_u32(record, word0);
            e.mem.set_u32(record + 0xc, object_name);
            e.mem.set_u32(record + 8, word2);
        });
        index += 1;
        offset = offset.wrapping_add(0x10);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db_a650, hk_data_class_constructor(Ptr, u32) -> Ptr),
        entry!(0x00db_a660, hk_data_class_destructor(Ptr)),
        entry!(0x00db_a670, hk_data_class_is_null(Ptr) -> bool),
        entry!(0x00db_a680, hk_data_class_get_name(Ptr) -> u32),
        entry!(0x00db_a690, hk_data_class_get_version(Ptr) -> u32),
        entry!(0x00db_a6a0, hk_data_class_get_parent(Ptr, Ptr) -> Ptr),
        entry!(0x00db_a6c0, hk_data_class_get_world(Ptr) -> u32),
        entry!(0x00db_a6f0, hk_data_class_get_num_declared_members(Ptr) -> i32),
        entry!(0x00db_a720, fn_00dba720(Ptr, u32, u32) -> u32),
        entry!(0x00db_a730, hk_data_class_get_num_members(Ptr) -> i32),
        entry!(0x00db_a740, fn_00dba740(Ptr, u32) -> u32),
        entry!(0x00db_a760, hk_data_object_constructor(Ptr, u32) -> Ptr),
        entry!(0x00db_a780, hk_data_object_constructor_ov2(Ptr, Ptr) -> Ptr),
        entry!(0x00db_a7f0, hk_data_object_destructor(Ptr)),
        entry!(0x00db_a810, hk_data_object_get_handle(Ptr, Ptr) -> Ptr),
        entry!(0x00db_a850, hk_data_object_get_class(Ptr, Ptr) -> Ptr),
        entry!(0x00db_a870, hk_data_object_operator(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_a890, hk_data_object_operator_ov2(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_a8b0, fn_00dba8b0(Ptr) -> u32),
        entry!(0x00db_a8c0, fn_00dba8c0(Ptr, u32) -> u32),
        entry!(0x00db_a8d0, fn_00dba8d0(Ptr, u32) -> u32),
        entry!(0x00db_a8e0, fn_00dba8e0(Ptr, u32) -> u32),
        entry!(0x00db_a8f0, fn_00dba8f0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_a910, hk_data_object_has_member(Ptr, u32) -> u32),
        entry!(0x00db_a920, hk_data_object_value_get_type(Ptr) -> u32),
        entry!(0x00db_a980, hk_data_object_is_null(Ptr) -> bool),
        entry!(0x00db_a990, hk_data_object_value_as_int(Ptr) -> u32),
        entry!(0x00db_a9a0, hk_data_object_value_as_vec(Ptr, u32) -> u32),
        entry!(0x00db_a9c0, hk_data_object_value_as_string(Ptr) -> u32),
        entry!(0x00db_a9d0, hk_data_object_value_as_object(Ptr, Ptr) -> Ptr),
        entry!(0x00db_aa00, hk_data_object_value_as_real(Ptr) -> f32),
        entry!(0x00db_aa10, fn_00dbaa10(Ptr, u32)),
        entry!(0x00db_aa30, hk_data_object_value_operator(Ptr, i32)),
        entry!(0x00db_aa50, hk_data_object_value_operator_ov2(Ptr, Ptr)),
        entry!(0x00db_aa90, fn_00dbaa90(Ptr, u32, u32)),
        entry!(0x00db_aab0, hk_data_object_value_operator_ov3(Ptr, f32)),
        entry!(0x00db_ab10, hk_data_object_value_set_vec(Ptr, u32, u32)),
        entry!(0x00db_ab30, hk_data_object_value_operator_ov4(Ptr, u32)),
        entry!(0x00db_ab50, hk_data_array_constructor(Ptr, u32) -> Ptr),
        entry!(0x00db_ab70, hk_data_array_destructor(Ptr)),
        entry!(0x00db_ab90, hk_data_array_operator(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_abb0, hk_data_array_operator_ov2(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_abe0, hk_data_array_set_size(Ptr, u32) -> u32),
        entry!(0x00db_ac00, hk_data_array_get_size(Ptr) -> i32),
        entry!(0x00db_ac10, hk_data_array_get_type(Ptr) -> u32),
        entry!(0x00db_ac20, hk_data_array_get_class(Ptr, Ptr) -> Ptr),
        entry!(0x00db_ac40, hk_data_array_value_get_type(Ptr) -> u32),
        entry!(0x00db_ac50, hk_data_array_value_as_vec(Ptr, u32) -> u32),
        entry!(0x00db_ac70, hk_data_array_value_as_string(Ptr) -> u32),
        entry!(0x00db_ac80, hk_data_array_value_as_real(Ptr) -> f32),
        entry!(0x00db_ac90, hk_data_array_value_as_int(Ptr) -> u32),
        entry!(0x00db_aca0, hk_data_array_value_as_object(Ptr, Ptr) -> Ptr),
        entry!(0x00db_acd0, hk_data_array_value_as_array(Ptr, Ptr) -> Ptr),
        entry!(0x00db_ad00, hk_data_array_value_operator(Ptr, Ptr)),
        entry!(0x00db_ad20, hk_data_array_value_operator_ov2(Ptr, f32)),
        entry!(0x00db_ad60, hk_data_array_value_set_vec(Ptr, u32, u32)),
        entry!(0x00db_ada0, hk_data_array_value_operator_ov3(Ptr, u32)),
        entry!(0x00db_adc0, hk_data_array_value_operator_ov4(Ptr, u32)),
        entry!(0x00db_ae10, fn_00dbae10(Ptr, u32, u32) -> u32),
        entry!(0x00db_ae80, fn_00dbae80(Ptr, u32, u32) -> u32),
        entry!(0x00db_aea0, fn_00dbaea0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_aed0, hk_data_class_get_all_member_info(Ptr, Ptr)),
        entry!(
            0x00db_af10,
            hk_data_class_get_all_declared_member_info(Ptr, Ptr)
        ),
        entry!(0x00db_af50, fn_00dbaf50(Ptr, Ptr) -> bool),
        entry!(0x00db_af90, fn_00dbaf90(Ptr, Ptr) -> bool),
        entry!(0x00db_afb0, hk_data_object_value_as_array(Ptr, Ptr) -> Ptr),
        entry!(0x00db_afe0, hk_data_class_get_cinfo(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The implementation's vtable.
    const VTABLE: u32 = 0x0b00_0000;
    /// The double of the implementation's slot `s` sits at `TARGET + s`.
    const TARGET: u32 = 0x0a00_0000;
    /// Every double of the implementation returns `RESULT_BASE + slot` in `EAX`.
    const RESULT_BASE: u32 = 0x1000;
    const MEMBER: u32 = 0x77;

    /// An engine with an implementation object whose 34 slots are doubles that log
    /// `[slot, this, arguments...]`, and a two-word handle (`implementation`, `MEMBER`).
    struct Rig {
        e: Engine,
        implementation: u32,
        holder: u32,
        log: Log,
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(RESULT_BASE, 0x1000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let slots: Vec<u32> = (0..0x22).map(|i| TARGET + i * 4).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x22u32 {
            let slot = i * 4;
            let calls = Rc::clone(&log);
            e.register_double(TARGET + slot, move |_, a| {
                let mut entry = vec![slot];
                entry.extend_from_slice(a);
                calls.borrow_mut().push(entry);
                Ret {
                    eax: RESULT_BASE + slot,
                    edx: 0,
                    st0: 1.5,
                }
            });
        }
        let implementation = e.mem.alloc(0x40);
        e.mem.set_u32(implementation, VTABLE);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, implementation);
        e.mem.set_u32(holder + 4, MEMBER);
        Rig {
            e,
            implementation,
            holder,
            log,
        }
    }

    /// Calls `address` on a handle with `arguments` after `this` and checks that exactly slot
    /// `slot` of the implementation ran, with the implementation then `expected`, and that
    /// its result came back (the assignments and setters return nothing, so `eax` is zero).
    fn forwards(address: u32, slot: u32, arguments: &[u32], expected: &[u32]) {
        let mut r = rig();
        let mut words = vec![r.holder];
        words.extend_from_slice(arguments);
        let got = r.e.call(address, &words).u32();
        let mut entry = vec![slot, r.implementation];
        entry.extend_from_slice(expected);
        assert_eq!(*r.log.borrow(), vec![entry]);
        let returns_nothing = matches!(
            address,
            0x00db_aa10
                | 0x00db_aa30
                | 0x00db_aa90
                | 0x00db_aab0
                | 0x00db_ab10
                | 0x00db_ab30
                | 0x00db_ad20
                | 0x00db_ad60
                | 0x00db_ada0
                | 0x00db_adc0
        );
        assert_eq!(
            got,
            if returns_nothing {
                0
            } else {
                RESULT_BASE + slot
            }
        );
    }

    /// Like [`forwards`] for a function that returns its first argument, a result record.
    fn forwards_into(address: u32, slot: u32, arguments: &[u32], expected: &[u32]) {
        let mut r = rig();
        let out = r.e.mem.alloc(0x10);
        let mut words = vec![r.holder, out];
        words.extend_from_slice(arguments);
        let got = r.e.call(address, &words).u32();
        let mut entry = vec![slot, r.implementation, out];
        entry.extend_from_slice(expected);
        assert_eq!(*r.log.borrow(), vec![entry]);
        assert_eq!(got, out);
    }

    /// Calls `address` with the handle and an output handle and checks that the output holds
    /// the slot's result as its implementation and was returned.
    fn builds_handle(address: u32, slot: u32) {
        let mut r = rig();
        let out = r.e.mem.alloc(8);
        let got = r.e.call(address, &[r.holder, out]).u32();
        assert_eq!(*r.log.borrow(), vec![vec![slot, r.implementation]]);
        assert_eq!(got, out);
        assert_eq!(r.e.mem.u32(out), RESULT_BASE + slot);
    }

    macro_rules! forward_tests {
        ($($name:ident: $addr:literal, $slot:literal, [$($arg:expr),*], [$($want:expr),*];)*) => {$(
            #[test]
            fn $name() {
                forwards($addr, $slot, &[$($arg),*], &[$($want),*]);
            }
        )*};
    }

    forward_tests! {
        class_get_name_calls_slot_8: 0x00db_a680, 0x8, [], [];
        class_get_version_calls_slot_c: 0x00db_a690, 0xc, [], [];
        class_get_world_calls_slot_4: 0x00db_a6c0, 0x4, [], [];
        class_get_num_declared_members_calls_slot_18: 0x00db_a6f0, 0x18, [], [];
        class_get_num_members_calls_slot_24: 0x00db_a730, 0x24, [], [];
        fn_00dba720_passes_index_and_record_to_slot_28: 0x00db_a720, 0x28, [3, 0x500], [3, 0x500];
        fn_00dba740_passes_its_word_to_slot_2c: 0x00db_a740, 0x2c, [9], [9];
        fn_00dba8b0_calls_slot_14: 0x00db_a8b0, 0x14, [], [];
        fn_00dba8c0_passes_its_word_to_slot_18: 0x00db_a8c0, 0x18, [4], [4];
        fn_00dba8d0_passes_its_word_to_slot_1c: 0x00db_a8d0, 0x1c, [5], [5];
        fn_00dba8e0_passes_its_word_to_slot_20: 0x00db_a8e0, 0x20, [6], [6];
        object_has_member_passes_the_name_to_slot_10: 0x00db_a910, 0x10, [0x600], [0x600];
        object_value_as_int_passes_the_member_to_slot_30: 0x00db_a990, 0x30, [], [MEMBER];
        object_value_as_vec_passes_member_and_count_to_slot_38: 0x00db_a9a0, 0x38, [3], [MEMBER, 3];
        object_value_as_string_passes_the_member_to_slot_2c: 0x00db_a9c0, 0x2c, [], [MEMBER];
        fn_00dbaa10_passes_member_and_value_to_slot_5c: 0x00db_aa10, 0x5c, [8], [MEMBER, 8];
        object_value_operator_extends_the_int_to_64_bits: 0x00db_aa30, 0x40, [0xffff_fffe], [MEMBER, 0xffff_fffe, 0xffff_ffff];
        fn_00dbaa90_passes_member_low_and_high_to_slot_40: 0x00db_aa90, 0x40, [1, 2], [MEMBER, 1, 2];
        object_value_set_vec_passes_member_values_and_count_to_slot_44: 0x00db_ab10, 0x44, [0x700, 4], [MEMBER, 0x700, 4];
        object_value_operator_ov4_passes_member_and_value_to_slot_50: 0x00db_ab30, 0x50, [0x42], [MEMBER, 0x42];
        array_set_size_passes_the_size_to_slot_10: 0x00db_abe0, 0x10, [12], [12];
        array_get_size_calls_slot_14: 0x00db_ac00, 0x14, [], [];
        array_get_type_calls_slot_4: 0x00db_ac10, 0x4, [], [];
        array_value_get_type_calls_slot_4_without_the_element: 0x00db_ac40, 0x4, [], [];
        array_value_as_vec_replaces_its_word_with_the_element: 0x00db_ac50, 0x20, [0xdead], [MEMBER];
        array_value_as_string_passes_the_element_to_slot_28: 0x00db_ac70, 0x28, [], [MEMBER];
        array_value_as_int_passes_the_element_to_slot_40: 0x00db_ac90, 0x40, [], [MEMBER];
        array_value_set_vec_passes_element_and_first_word_to_slot_24: 0x00db_ad60, 0x24, [0x800, 0x801], [MEMBER, 0x800];
        array_value_operator_ov3_passes_element_and_value_to_slot_44: 0x00db_ada0, 0x44, [0x43], [MEMBER, 0x43];
        array_value_operator_ov4_passes_element_and_value_to_slot_2c: 0x00db_adc0, 0x2c, [0x44], [MEMBER, 0x44];
        fn_00dbae10_passes_both_words_to_slot_80: 0x00db_ae10, 0x80, [1, 2], [1, 2];
        fn_00dbae80_passes_both_words_to_slot_64: 0x00db_ae80, 0x64, [3, 4], [3, 4];
    }

    #[test]
    fn object_value_as_real_returns_the_float_result() {
        let mut r = rig();
        let got = r.e.call(0x00db_aa00, &[r.holder]).f32();
        assert_eq!(*r.log.borrow(), vec![vec![0x3c, r.implementation, MEMBER]]);
        assert_eq!(got, 1.5);
    }

    #[test]
    fn array_value_as_real_returns_the_float_result() {
        let mut r = rig();
        let got = r.e.call(0x00db_ac80, &[r.holder]).f32();
        assert_eq!(*r.log.borrow(), vec![vec![0x30, r.implementation, MEMBER]]);
        assert_eq!(got, 1.5);
    }

    /// The float assignment passes the bit pattern after the x87 round trip: a signalling NaN
    /// comes out quiet.
    #[test]
    fn object_value_operator_ov3_passes_the_quieted_float_to_slot_4c() {
        let one = 1.0f32.to_bits();
        forwards(0x00db_aab0, 0x4c, &[one], &[MEMBER, one]);
        forwards(0x00db_aab0, 0x4c, &[0x7f80_0001], &[MEMBER, 0x7fc0_0001]);
    }

    #[test]
    fn array_value_operator_ov2_passes_the_quieted_float_to_slot_34() {
        let value = 2.5f32.to_bits();
        forwards(0x00db_ad20, 0x34, &[value], &[MEMBER, value]);
        forwards(0x00db_ad20, 0x34, &[0xff80_0002], &[MEMBER, 0xffc0_0002]);
    }

    #[test]
    fn object_value_operator_ov2_passes_the_first_word_of_the_other_object() {
        let mut r = rig();
        let other = r.e.mem.alloc(8);
        r.e.mem.set_u32(other, 0x9999);
        r.e.call(0x00db_aa50, &[r.holder, other]);
        assert_eq!(
            *r.log.borrow(),
            vec![vec![0x58, r.implementation, MEMBER, 0x9999]]
        );
    }

    #[test]
    fn array_value_operator_passes_the_first_word_of_the_other_handle() {
        let mut r = rig();
        let other = r.e.mem.alloc(8);
        r.e.mem.set_u32(other, 0x9998);
        r.e.call(0x00db_ad00, &[r.holder, other]);
        assert_eq!(
            *r.log.borrow(),
            vec![vec![0x54, r.implementation, MEMBER, 0x9998]]
        );
    }

    #[test]
    fn class_is_null_tests_the_implementation_pointer() {
        let mut r = rig();
        assert!(!r.e.call(0x00db_a670, &[r.holder]).bool());
        let empty = r.e.mem.alloc(4);
        assert!(r.e.call(0x00db_a670, &[empty]).bool());
    }

    #[test]
    fn object_is_null_tests_the_implementation_pointer() {
        let mut r = rig();
        assert!(!r.e.call(0x00db_a980, &[r.holder]).bool());
        let empty = r.e.mem.alloc(4);
        assert!(r.e.call(0x00db_a980, &[empty]).bool());
    }

    #[test]
    fn class_constructor_stores_the_implementation_without_a_reference() {
        let mut r = rig();
        let this = r.e.mem.alloc(4);
        let got = r.e.call(0x00db_a650, &[this, r.implementation]).u32();
        assert_eq!(got, this);
        assert_eq!(r.e.mem.u32(this), r.implementation);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 0);
    }

    #[test]
    fn class_destructor_does_nothing() {
        let mut r = rig();
        r.e.call(0x00db_a660, &[r.holder]);
        assert_eq!(r.e.mem.u32(r.holder), r.implementation);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 0);
        assert!(r.log.borrow().is_empty());
    }

    #[test]
    fn class_get_parent_builds_a_class_handle_from_slot_10() {
        builds_handle(0x00db_a6a0, 0x10);
    }

    #[test]
    fn object_get_class_builds_a_class_handle_from_slot_8() {
        builds_handle(0x00db_a850, 0x8);
    }

    #[test]
    fn array_get_class_builds_a_class_handle_from_slot_18() {
        builds_handle(0x00db_ac20, 0x18);
    }

    /// The object and array handle builders take a reference on the result: both counts go up
    /// by one.
    fn check_reference(address: u32, slot: u32) {
        let mut r = rig();
        let out = r.e.mem.alloc(8);
        let got = r.e.call(address, &[r.holder, out]).u32();
        let made = RESULT_BASE + slot;
        assert_eq!(got, out);
        assert_eq!(r.e.mem.u32(out), made);
        assert_eq!(r.e.mem.u16(made + 6), 1);
        assert_eq!(r.e.mem.u32(made + 8), 1);
    }

    #[test]
    fn object_value_as_object_takes_a_reference_on_the_slot_34_result() {
        check_reference(0x00db_a9d0, 0x34);
    }

    #[test]
    fn array_value_as_object_takes_a_reference_on_the_slot_50_result() {
        check_reference(0x00db_aca0, 0x50);
    }

    #[test]
    fn array_value_as_array_takes_a_reference_on_the_slot_58_result() {
        check_reference(0x00db_acd0, 0x58);
    }

    #[test]
    fn object_value_as_array_takes_a_reference_on_the_slot_28_result() {
        check_reference(0x00db_afb0, 0x28);
    }

    #[test]
    fn fn_00dbaea0_takes_a_reference_on_the_slot_1c_result() {
        let mut r = rig();
        let out = r.e.mem.alloc(8);
        let got = r.e.call(0x00db_aea0, &[r.holder, out, 0x55]).u32();
        let made = RESULT_BASE + 0x1c;
        assert_eq!(got, out);
        assert_eq!(*r.log.borrow(), vec![vec![0x1c, r.implementation, 0x55]]);
        assert_eq!(r.e.mem.u32(out), made);
        assert_eq!(r.e.mem.u16(made + 6), 1);
        assert_eq!(r.e.mem.u32(made + 8), 1);
    }

    #[test]
    fn object_constructor_adds_a_reference_unless_the_pointer_is_null() {
        let mut r = rig();
        let this = r.e.mem.alloc(4);
        let got = r.e.call(0x00db_a760, &[this, r.implementation]).u32();
        assert_eq!(got, this);
        assert_eq!(r.e.mem.u32(this), r.implementation);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 1);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
        r.e.call(0x00db_a760, &[this, 0]);
        assert_eq!(r.e.mem.u32(this), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
    }

    #[test]
    fn array_constructor_adds_a_reference_unless_the_pointer_is_null() {
        let mut r = rig();
        let this = r.e.mem.alloc(4);
        r.e.mem.set_u16(r.implementation + 6, 0xffff);
        let got = r.e.call(0x00db_ab50, &[this, r.implementation]).u32();
        assert_eq!(got, this);
        assert_eq!(r.e.mem.u32(this), r.implementation);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
        r.e.call(0x00db_ab50, &[this, 0]);
        assert_eq!(r.e.mem.u32(this), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
    }

    #[test]
    fn object_constructor_ov2_copies_the_implementation_and_adds_a_reference() {
        let mut r = rig();
        let this = r.e.mem.alloc(4);
        let got = r.e.call(0x00db_a780, &[this, r.holder]).u32();
        assert_eq!(got, this);
        assert_eq!(r.e.mem.u32(this), r.implementation);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 1);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
        let empty = r.e.mem.alloc(4);
        r.e.call(0x00db_a780, &[this, empty]);
        assert_eq!(r.e.mem.u32(this), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
    }

    /// The two destructors drop a reference and only the last one deletes the implementation
    /// (slot 0 with flag 1).
    fn check_release(address: u32) {
        let mut r = rig();
        r.e.mem.set_u16(r.implementation + 6, 2);
        r.e.mem.set_u32(r.implementation + 8, 2);
        r.e.call(address, &[r.holder]);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 1);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 1);
        assert!(r.log.borrow().is_empty());
        r.e.call(address, &[r.holder]);
        assert_eq!(r.e.mem.u16(r.implementation + 6), 0);
        assert_eq!(r.e.mem.u32(r.implementation + 8), 0);
        assert_eq!(*r.log.borrow(), vec![vec![0, r.implementation, 1]]);
        let empty = r.e.mem.alloc(4);
        r.e.call(address, &[empty]);
        assert_eq!(r.log.borrow().len(), 1);
    }

    #[test]
    fn object_destructor_deletes_the_implementation_at_the_last_reference() {
        check_release(0x00db_a7f0);
    }

    #[test]
    fn array_destructor_deletes_the_implementation_at_the_last_reference() {
        check_release(0x00db_ab70);
    }

    #[test]
    fn object_operator_calls_slot_c_with_the_result_record_and_the_key() {
        forwards_into(0x00db_a870, 0xc, &[0x321], &[0x321]);
    }

    #[test]
    fn object_operator_ov2_calls_slot_c_with_the_result_record_and_the_key() {
        forwards_into(0x00db_a890, 0xc, &[0x322], &[0x322]);
    }

    #[test]
    fn fn_00dba8f0_calls_slot_24_with_the_result_record_and_the_iterator() {
        forwards_into(0x00db_a8f0, 0x24, &[0x323], &[0x323]);
    }

    #[test]
    fn array_operator_builds_the_value_from_the_implementation_and_index() {
        for address in [0x00db_ab90, 0x00db_abb0] {
            let mut r = rig();
            let out = r.e.mem.alloc(8);
            let got = r.e.call(address, &[r.holder, out, 5]).u32();
            assert_eq!(got, out);
            assert_eq!(r.e.mem.u32(out), r.implementation);
            assert_eq!(r.e.mem.u32(out + 4), 5);
        }
    }

    #[test]
    fn object_get_handle_copies_the_record_slot_4_returns() {
        let mut r = rig();
        let record = r.e.mem.alloc(8);
        r.e.mem.set_u32(record, 0xa1);
        r.e.mem.set_u32(record + 4, 0xb2);
        let scratch: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let seen = Rc::clone(&scratch);
        r.e.register_double(TARGET + 4, move |_, a| {
            *seen.borrow_mut() = a[1];
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        let out = r.e.mem.alloc(8);
        let got = r.e.call(0x00db_a810, &[r.holder, out]).u32();
        assert_eq!(got, out);
        assert_eq!(r.e.mem.u32(out), 0xa1);
        assert_eq!(r.e.mem.u32(out + 4), 0xb2);
        assert_ne!(*scratch.borrow(), 0);
    }

    #[test]
    fn object_get_handle_without_an_implementation_is_zero() {
        let mut r = rig();
        let empty = r.e.mem.alloc(4);
        let out = r.e.mem.alloc(8);
        r.e.mem.set_u32(out, 0x1234);
        r.e.mem.set_u32(out + 4, 0x5678);
        let got = r.e.call(0x00db_a810, &[empty, out]).u32();
        assert_eq!(got, out);
        assert_eq!(r.e.mem.u32(out), 0);
        assert_eq!(r.e.mem.u32(out + 4), 0);
        assert!(r.log.borrow().is_empty());
    }

    /// Makes the handle at `handle` an object whose implementation answers slot 4 with the
    /// record (`first`, `second`).
    fn give_handle(r: &mut Rig, handle: u32, first: u32, second: u32, double: u32) {
        let record = r.e.mem.alloc(8);
        r.e.mem.set_u32(record, first);
        r.e.mem.set_u32(record + 4, second);
        let implementation = r.e.mem.alloc(0x10);
        let vtable = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(vtable + 4, double);
        r.e.mem.set_u32(implementation, vtable);
        r.e.mem.set_u32(handle, implementation);
        r.e.register_double(double, move |_, _| Ret {
            eax: record,
            ..Ret::default()
        });
    }

    #[test]
    fn handles_compare_equal_when_both_words_match() {
        let mut r = rig();
        let first = r.e.mem.alloc(4);
        give_handle(&mut r, first, 7, 8, 0x0a10_0004);
        let same = r.e.mem.alloc(4);
        give_handle(&mut r, same, 7, 8, 0x0a10_0008);
        assert!(r.e.call(0x00db_af50, &[first, same]).bool());
        let other_second = r.e.mem.alloc(4);
        give_handle(&mut r, other_second, 7, 9, 0x0a10_000c);
        assert!(!r.e.call(0x00db_af50, &[first, other_second]).bool());
        let other_first = r.e.mem.alloc(4);
        give_handle(&mut r, other_first, 6, 8, 0x0a10_0010);
        assert!(!r.e.call(0x00db_af50, &[first, other_first]).bool());
        let empty = r.e.mem.alloc(4);
        assert!(!r.e.call(0x00db_af50, &[first, empty]).bool());
    }

    #[test]
    fn handle_inequality_is_the_negation_of_equality() {
        let mut r = rig();
        let first = r.e.mem.alloc(4);
        let second = r.e.mem.alloc(4);
        assert!(!r.e.call(0x00db_af90, &[first, second]).bool());
        give_handle(&mut r, second, 0, 1, 0x0a10_0004);
        assert!(r.e.call(0x00db_af90, &[first, second]).bool());
    }

    /// A class object (own vtable) whose slot `0x2c` answers `index` for any member and whose
    /// slot `0x28` writes `kind` into word 2 of the record it is given. Both log
    /// `[slot, this, first argument]`.
    fn make_class(r: &mut Rig, index: u32, kind: u32) -> u32 {
        let vtable = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(vtable + 0x2c, 0x0a20_002c);
        r.e.mem.set_u32(vtable + 0x28, 0x0a20_0028);
        let class = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(class, vtable);
        let seen = Rc::clone(&r.log);
        r.e.register_double(0x0a20_002c, move |_, a| {
            seen.borrow_mut().push(vec![0x2c, a[0], a[1]]);
            Ret {
                eax: index,
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&r.log);
        r.e.register_double(0x0a20_0028, move |e, a| {
            seen.borrow_mut().push(vec![0x28, a[0], a[1]]);
            assert_eq!(e.mem.bytes(a[2], 0x14), vec![0; 0x14]);
            e.mem.set_u32(a[2] + 8, kind);
            Ret::default()
        });
        class
    }

    #[test]
    fn object_value_get_type_reads_word_2_of_the_member_record() {
        let mut r = rig();
        let class = make_class(&mut r, 4, 0xabcd);
        r.e.register_double(TARGET + 8, move |_, _| Ret {
            eax: class,
            ..Ret::default()
        });
        let got = r.e.call(0x00db_a920, &[r.holder]).u32();
        assert_eq!(got, 0xabcd);
        assert_eq!(
            *r.log.borrow(),
            vec![vec![0x2c, class, MEMBER], vec![0x28, class, 4]]
        );
    }

    #[test]
    fn object_value_get_type_without_an_implementation_is_zero() {
        let mut r = rig();
        let empty = r.e.mem.alloc(8);
        assert_eq!(r.e.call(0x00db_a920, &[empty]).u32(), 0);
        assert!(r.log.borrow().is_empty());
    }

    /// Makes slot `count_slot` answer `count` and slot `slot` log its arguments.
    fn count_rig(count_slot: u32, count: u32, slot: u32) -> Rig {
        let mut r = rig();
        r.e.register_double(TARGET + count_slot, move |_, _| Ret {
            eax: count,
            ..Ret::default()
        });
        let seen = Rc::clone(&r.log);
        r.e.register_double(TARGET + slot, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        r
    }

    #[test]
    fn class_get_all_member_info_asks_slot_28_for_every_member() {
        let mut r = count_rig(0x24, 3, 0x28);
        let records = r.e.mem.alloc(4);
        let data = r.e.mem.alloc(0x3c);
        r.e.mem.set_u32(records, data);
        r.e.call(0x00db_aed0, &[r.holder, records]);
        let im = r.implementation;
        assert_eq!(
            *r.log.borrow(),
            vec![
                vec![im, 0, data],
                vec![im, 1, data + 0x14],
                vec![im, 2, data + 0x28]
            ]
        );
        let mut r = count_rig(0x24, 0, 0x28);
        r.e.call(0x00db_aed0, &[r.holder, records]);
        assert!(r.log.borrow().is_empty());
    }

    #[test]
    fn class_get_all_declared_member_info_asks_slot_1c_for_every_declared_member() {
        let mut r = count_rig(0x18, 2, 0x1c);
        let records = r.e.mem.alloc(4);
        let data = r.e.mem.alloc(0x28);
        r.e.mem.set_u32(records, data);
        r.e.call(0x00db_af10, &[r.holder, records]);
        let im = r.implementation;
        assert_eq!(
            *r.log.borrow(),
            vec![vec![im, 0, data], vec![im, 1, data + 0x14]]
        );
        let mut r = count_rig(0x18, 0xffff_ffff, 0x1c);
        r.e.call(0x00db_af10, &[r.holder, records]);
        assert!(r.log.borrow().is_empty());
    }

    const ROUTER: u32 = 0x0c00_0000;
    const ALLOCATOR: u32 = 0x0c00_0100;

    /// A `getCinfo` rig: a class implementation with name 0x1111, version 0x2222, a parent
    /// named 0x4444 (optional) and `count` declared members; member `i` has info words
    /// (`0x100 + i`, junk, `0x200 + i`, object, `0x300 + i`) where the object, which answers
    /// slot 8 with 0x4444, is only present for member 0. The reserve call is recorded and
    /// gives the array room.
    fn cinfo_rig(parent: bool, count: u32) -> (Rig, u32, Log) {
        let mut r = rig();
        r.e.map(ROUTER, 0x1000);
        r.e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        r.e.register_double(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        let named = r.e.mem.alloc(0x10);
        let vtable = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(vtable + 8, 0x0a30_0008);
        r.e.mem.set_u32(named, vtable);
        r.e.register_double(0x0a30_0008, |_, _| Ret {
            eax: 0x4444,
            ..Ret::default()
        });
        r.e.register_double(TARGET + 8, |_, _| Ret {
            eax: 0x1111,
            ..Ret::default()
        });
        r.e.register_double(TARGET + 0xc, |_, _| Ret {
            eax: 0x2222,
            ..Ret::default()
        });
        r.e.register_double(TARGET + 0x10, move |_, _| Ret {
            eax: if parent { named } else { 0 },
            ..Ret::default()
        });
        r.e.register_double(TARGET + 0x18, move |_, _| Ret {
            eax: count,
            ..Ret::default()
        });
        r.e.register_double(TARGET + 0x1c, move |e, a| {
            let (index, info) = (a[1], a[2]);
            assert_eq!(e.mem.bytes(info, 0x14), vec![0; 0x14]);
            e.mem.set_u32(info, 0x100 + index);
            e.mem.set_u32(info + 4, 0xdead);
            e.mem.set_u32(info + 8, 0x200 + index);
            e.mem
                .set_u32(info + 0xc, if index == 0 { named } else { 0 });
            e.mem.set_u32(info + 0x10, 0x300 + index);
            Ret::default()
        });
        let reserves: Log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&reserves);
        r.e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let out = r.e.mem.alloc(0x20);
        (r, out, reserves)
    }

    #[test]
    fn class_get_cinfo_fills_the_description_and_the_member_records() {
        let (mut r, out, reserves) = cinfo_rig(true, 2);
        r.e.call(0x00db_afe0, &[r.holder, out]);
        assert_eq!(r.e.mem.u32(out), 0x1111);
        assert_eq!(r.e.mem.u32(out + 4), 0x2222);
        assert_eq!(r.e.mem.u32(out + 8), 0x4444);
        assert_eq!(r.e.mem.u32(out + 0x10), 2);
        assert_eq!(
            *reserves.borrow(),
            vec![vec![ALLOCATOR, out + 0xc, 2, 0x10]]
        );
        let data = r.e.mem.u32(out + 0xc);
        assert_eq!(r.e.mem.u32(data), 0x100);
        assert_eq!(r.e.mem.u32(data + 4), 0x200);
        assert_eq!(r.e.mem.u32(data + 8), 0x300);
        assert_eq!(r.e.mem.u32(data + 0xc), 0x4444);
        assert_eq!(r.e.mem.u32(data + 0x10), 0x101);
        assert_eq!(r.e.mem.u32(data + 0x14), 0x201);
        assert_eq!(r.e.mem.u32(data + 0x18), 0x301);
        assert_eq!(r.e.mem.u32(data + 0x1c), 0);
    }

    #[test]
    fn class_get_cinfo_without_a_parent_keeps_a_large_enough_array() {
        let (mut r, out, reserves) = cinfo_rig(false, 1);
        let data = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(out + 0xc, data);
        r.e.mem.set_u32(out + 0x14, 0x8000_0003);
        r.e.call(0x00db_afe0, &[r.holder, out]);
        assert_eq!(r.e.mem.u32(out + 8), 0);
        assert!(reserves.borrow().is_empty());
        assert_eq!(r.e.mem.u32(out + 0xc), data);
        assert_eq!(r.e.mem.u32(out + 0x10), 1);
        assert_eq!(r.e.mem.u32(data), 0x100);
    }

    #[test]
    fn class_get_cinfo_grows_to_twice_the_capacity_when_that_is_enough() {
        let (mut r, out, reserves) = cinfo_rig(false, 3);
        r.e.mem.set_u32(out + 0x14, 2);
        r.e.call(0x00db_afe0, &[r.holder, out]);
        assert_eq!(
            *reserves.borrow(),
            vec![vec![ALLOCATOR, out + 0xc, 4, 0x10]]
        );
    }

    #[test]
    fn class_get_cinfo_without_members_sets_the_size_to_zero() {
        let (mut r, out, reserves) = cinfo_rig(false, 0);
        r.e.mem.set_u32(out + 0x10, 9);
        r.e.call(0x00db_afe0, &[r.holder, out]);
        assert_eq!(r.e.mem.u32(out + 0x10), 0);
        assert!(reserves.borrow().is_empty());
    }
}
