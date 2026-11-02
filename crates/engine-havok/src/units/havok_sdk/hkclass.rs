//! `Havok SDK/hkclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d472a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A Havok class record's constructor (`hkClass`, Havok SDK; `thiscall`,
/// `RET 0x34`: thirteen words). Stores the first three words at +0, +4 and
/// +8, skips the fourth, and stores the other nine at +0xC to +0x2C, in
/// order. Returns nothing (`EAX` is `this`, unused by the callers).
pub fn fn_00d472a0(e: &mut Engine, this: Ptr, words: &[u32]) {
    let t = this.addr();
    for (i, &w) in words[..3].iter().enumerate() {
        e.mem.set_u32(t + 4 * i as u32, w);
    }
    for (i, &w) in words[4..13].iter().enumerate() {
        e.mem.set_u32(t + 0xc + 4 * i as u32, w);
    }
}

fn fn_00d472a0_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00d472a0(e, Ptr::new(a[0]), &a[1..14]);
    a[0].into_ret()
}

/// `hkString::strCmp` (`00cb0fa0`, cdecl `(first, second)`; zero when equal).
const STR_CMP: u32 = 0x00cb_0fa0;
/// `hkString::strLen` (`00cb10d0`, cdecl `(string)`).
const STR_LEN: u32 = 0x00cb_10d0;
/// `hkOArchive::hkOArchive` (`00d60830`, `this` = the archive, `(stream, byte flag)`).
const OARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
/// `hkOArchive::write32` (`00d60940`, `this` = the archive, `(value)`).
const OARCHIVE_WRITE32: u32 = 0x00d6_0940;
/// `hkOArchive::write16` (`00d60900`, `this` = the archive, `(value)`: writes the low 16 bits).
const OARCHIVE_WRITE16: u32 = 0x00d6_0900;
/// `hkOArchive::writeRaw` (`00d607b0`, `this` = the archive, `(data, length)`).
const OARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
/// `hkOArchive::~hkOArchive` (`00d60590`, `this` = the archive).
const OARCHIVE_DESTRUCT: u32 = 0x00d6_0590;
/// `hkClassEnum::writeSignature` (`00db7600`, `this` = the enum record, `(stream)`).
const CLASS_ENUM_WRITE_SIGNATURE: u32 = 0x00db_7600;
/// `hkClassMember::getClassMemberTypeProperties` (`00d868a0`, cdecl `(type)`).
const MEMBER_TYPE_PROPERTIES: u32 = 0x00d8_68a0;
/// `hkClassMember::getClass` (`00d86920`, `this` = the member).
const MEMBER_GET_CLASS: u32 = 0x00d8_6920;
/// The member's enum accessor (`00d86930`, `this` = the member).
const MEMBER_GET_ENUM: u32 = 0x00d8_6930;
/// `hkClassMember::getSizeInBytes` (`00d86980`, `this` = the member).
const MEMBER_GET_SIZE_IN_BYTES: u32 = 0x00d8_6980;
/// The `hkCrc32StreamWriter` constructor (`00db7870`, `this` = the writer, `(initial value)`).
const CRC_WRITER_CONSTRUCT: u32 = 0x00db_7870;
/// The `hkCrc32StreamWriter` checksum accessor (`00db7860`, `this` = the writer).
const CRC_WRITER_GET_CRC: u32 = 0x00db_7860;

// Translated from 00d46cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCustomAttributes::getAttribute` (Xbox PDB; `this` = the attribute list: data pointer
/// `+0`, count `+4`; `RET 4`): the first twelve-byte entry whose name (its first word) equals
/// `name` under `hkString::strCmp`, answered as the address of the entry's second word, or 0.
pub fn hk_custom_attributes_get_attribute(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let t = this.addr();
    if e.mem.i32(t + 4) <= 0 {
        return 0;
    }
    let mut index = 0u32;
    loop {
        let data = e.mem.u32(t);
        let entry_name = e.mem.u32(data.wrapping_add(index.wrapping_mul(12)));
        if e.call(STR_CMP, &args![entry_name, name]).i32() == 0 {
            let data = e.mem.u32(t);
            return data.wrapping_add(index.wrapping_mul(12)).wrapping_add(4);
        }
        index = index.wrapping_add(1);
        if index as i32 >= e.mem.i32(t + 4) {
            return 0;
        }
    }
}

// Translated from 00d46d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getName` (Xbox PDB): the class record's name pointer (`+0`).
pub fn hk_class_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr())
}

// Translated from 00d46d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getParent` (Xbox PDB): the parent class record (`+4`).
pub fn hk_class_get_parent(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4)
}

// Translated from 00d46d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getParent` (the second overload, Xbox PDB): the same word as
/// [`hk_class_get_parent`], the parent class record (`+4`).
pub fn hk_class_get_parent_ov2(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4)
}

// Translated from 00d46d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A class-chain name test (no map name; `this` = a class record, then `(out, other)`,
/// `RET 8`): walks `other` and its parents and stores 1 at the byte `out` as soon as a class
/// has the name of `this` (`hkString::strCmp` of the two names, `other`'s first), or 0 when
/// the chain ends. Answers `out`.
pub fn fn_00d46d80(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> u32 {
    let mut candidate = other;
    while !candidate.is_null() {
        let own_name = hk_class_get_name(e, this);
        let candidate_name = hk_class_get_name(e, candidate);
        if e.call(STR_CMP, &args![candidate_name, own_name]).i32() == 0 {
            e.mem.set_u8(out.addr(), 1);
            return out.addr();
        }
        candidate = Ptr::new(hk_class_get_parent(e, candidate));
    }
    e.mem.set_u8(out.addr(), 0);
    out.addr()
}

/// The sum of the word at `field` of the class and of every parent class (the loop the
/// count accessors of `00d46de0`, `00d46e30` and `00d46f00` share).
fn chain_total(e: &mut Engine, this: Ptr, field: u32) -> i32 {
    let t = this.addr();
    let mut total = e.mem.i32(t + field);
    let mut parent = e.mem.u32(t + 4);
    while parent != 0 {
        total = total.wrapping_add(e.mem.i32(parent + field));
        parent = e.mem.u32(parent + 4);
    }
    total
}

// Translated from 00d46de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getNumInterfaces` (Xbox PDB): the class's word at `+0xC` plus those of every
/// parent class.
pub fn hk_class_get_num_interfaces(e: &mut Engine, this: Ptr) -> i32 {
    chain_total(e, this, 0xc)
}

// Translated from 00d46e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the class's own word at `+0xC` (the term [`hk_class_get_num_interfaces`]
/// starts its sum with).
pub fn fn_00d46e20(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0xc)
}

// Translated from 00d46e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the sum of the class's word at `+0x14` and those of every parent class (the
/// count of the enum records, `+0x10` pointing at twenty-byte entries).
pub fn fn_00d46e30(e: &mut Engine, this: Ptr) -> i32 {
    chain_total(e, this, 0x14)
}

/// Finds the record `index` of an array that is spread over a class and its parents (the
/// parents' records first): `count` is the field of each class's own number of records,
/// `array` the field of its record pointer, `stride` a record's size. `total` is the sum of
/// the counts over the chain. An index past the chain falls back to the class's own array.
fn chain_record(e: &mut Engine, this: Ptr, index: i32, total: i32, layout: (u32, u32, u32)) -> u32 {
    let (count, array, stride) = layout;
    let t = this.addr();
    let mut rest = index.wrapping_sub(total);
    if t != 0 {
        let mut class = t;
        loop {
            rest = rest.wrapping_add(e.mem.i32(class + count));
            if rest >= 0 {
                return e
                    .mem
                    .u32(class + array)
                    .wrapping_add((rest as u32).wrapping_mul(stride));
            }
            class = e.mem.u32(class + 4);
            if class == 0 {
                break;
            }
        }
    }
    e.mem.u32(t + array)
}

// Translated from 00d46e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `index`, `RET 4`): the enum record number
/// `index` counted over the whole class chain, parents first (twenty-byte records); an index
/// out of range answers the class's own enum array.
pub fn fn_00d46e50(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let total = fn_00d46e30(e, this);
    chain_record(e, this, index, total, (0x14, 0x10, 0x14))
}

// Translated from 00d46e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `name`, `RET 4`): the first enum record of the
/// chain (see [`fn_00d46e50`]) whose name (its first word) equals `name` under
/// `hkString::strCmp`, or 0.
pub fn fn_00d46e90(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let mut index = 0i32;
    if fn_00d46e30(e, this) <= 0 {
        return 0;
    }
    loop {
        let record = fn_00d46e50(e, this, index);
        let record_name = e.mem.u32(record);
        if e.call(STR_CMP, &args![record_name, name]).i32() == 0 {
            return record;
        }
        index = index.wrapping_add(1);
        if index >= fn_00d46e30(e, this) {
            return 0;
        }
    }
}

// Translated from 00d46ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `index`, `RET 4`): the class's own enum record
/// number `index` (twenty-byte records at `+0x10`).
pub fn fn_00d46ee0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    e.mem
        .u32(this.addr() + 0x10)
        .wrapping_add((index as u32).wrapping_mul(0x14))
}

// Translated from 00d46ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the class's own number of enum records (`+0x14`).
pub fn fn_00d46ef0(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0x14)
}

// Translated from 00d46f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getNumMembers` (Xbox PDB): the class's number of declared members (`+0x1C`) plus
/// those of every parent class.
pub fn hk_class_get_num_members(e: &mut Engine, this: Ptr) -> i32 {
    chain_total(e, this, 0x1c)
}

// Translated from 00d46f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getMember` (Xbox PDB; `this` = a class record, then `index`, `RET 4`): the
/// member record number `index` counted over the whole class chain, parents first
/// (twenty-four-byte records); an index out of range answers the class's own member array.
pub fn hk_class_get_member(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let total = hk_class_get_num_members(e, this);
    chain_record(e, this, index, total, (0x1c, 0x18, 0x18))
}

// Translated from 00d46f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getMember` (the second overload, Xbox PDB): a jump to [`hk_class_get_member`].
pub fn hk_class_get_member_ov2(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    hk_class_get_member(e, this, index)
}

// Translated from 00d46f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getNumDeclaredMembers` (Xbox PDB): the class's own number of members (`+0x1C`).
pub fn hk_class_get_num_declared_members(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0x1c)
}

// Translated from 00d46f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getDeclaredMember` (Xbox PDB; `this` = a class record, then `index`, `RET 4`):
/// the class's own member record number `index` (twenty-four-byte records at `+0x18`).
pub fn hk_class_get_declared_member(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    e.mem
        .u32(this.addr() + 0x18)
        .wrapping_add((index as u32).wrapping_mul(0x18))
}

// Translated from 00d46f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `name`, `RET 4`): the first of the class's own
/// member records whose name (its first word) equals `name` under `hkString::strCmp`, or 0.
pub fn fn_00d46f90(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let mut index = 0i32;
    if hk_class_get_num_declared_members(e, this) <= 0 {
        return 0;
    }
    loop {
        let member = hk_class_get_declared_member(e, this, index);
        let member_name = e.mem.u32(member);
        if e.call(STR_CMP, &args![member_name, name]).i32() == 0 {
            return member;
        }
        index = index.wrapping_add(1);
        if index >= hk_class_get_num_declared_members(e, this) {
            return 0;
        }
    }
}

// Translated from 00d46fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getMemberByName` (Xbox PDB; `this` = a class record, then `name`, `RET 4`): the
/// first member record of the whole chain (parents first) whose name equals `name` under
/// `hkString::strCmp`, or 0.
pub fn hk_class_get_member_by_name(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let mut index = 0i32;
    if hk_class_get_num_members(e, this) <= 0 {
        return 0;
    }
    loop {
        let member = hk_class_get_member(e, this, index);
        let member_name = e.mem.u32(member);
        if e.call(STR_CMP, &args![member_name, name]).i32() == 0 {
            return member;
        }
        index = index.wrapping_add(1);
        if index >= hk_class_get_num_members(e, this) {
            return 0;
        }
    }
}

// Translated from 00d47030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getMemberIndexByName` (Xbox PDB; `this` = a class record, then `name`, `RET 4`):
/// the index over the whole chain of the first member whose name equals `name`, or -1.
pub fn hk_class_get_member_index_by_name(e: &mut Engine, this: Ptr, name: u32) -> i32 {
    let mut index = 0i32;
    if hk_class_get_num_members(e, this) <= 0 {
        return -1;
    }
    loop {
        let member = hk_class_get_member(e, this, index);
        let member_name = e.mem.u32(member);
        if e.call(STR_CMP, &args![member_name, name]).i32() == 0 {
            return index;
        }
        index = index.wrapping_add(1);
        if index >= hk_class_get_num_members(e, this) {
            return -1;
        }
    }
}

// Translated from 00d47080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getDeclaredMemberIndexByName` (Xbox PDB; `this` = a class record, then `name`,
/// `RET 4`): the index among the class's own members of the first one whose name equals
/// `name`, or -1.
pub fn hk_class_get_declared_member_index_by_name(e: &mut Engine, this: Ptr, name: u32) -> i32 {
    let mut index = 0i32;
    if hk_class_get_num_declared_members(e, this) <= 0 {
        return -1;
    }
    loop {
        let member = hk_class_get_declared_member(e, this, index);
        let member_name = e.mem.u32(member);
        if e.call(STR_CMP, &args![member_name, name]).i32() == 0 {
            return index;
        }
        index = index.wrapping_add(1);
        if index >= hk_class_get_num_declared_members(e, this) {
            return -1;
        }
    }
}

// Translated from 00d470d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getObjectSize` (Xbox PDB): the class's object size (`+8`).
pub fn hk_class_get_object_size(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 8)
}

// Translated from 00d470e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `size`, `RET 4`): stores `size` as the class's
/// object size (`+8`), the setter of [`hk_class_get_object_size`].
pub fn fn_00d470e0(e: &mut Engine, this: Ptr, size: i32) {
    e.mem.set_i32(this.addr() + 8, size);
}

// Translated from 00d470f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::hasVtable` (Xbox PDB; `this` = a class record, then `out`, `RET 4`): stores at the
/// byte `out` whether the topmost class of the parent chain has a non-zero word at `+0xC`.
/// Answers `out`.
pub fn hk_class_has_vtable(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let mut top = this.addr();
    let mut parent = hk_class_get_parent(e, this);
    while parent != 0 {
        top = parent;
        parent = hk_class_get_parent(e, Ptr::new(top));
    }
    let flag = e.mem.u32(top + 0xc) != 0;
    e.mem.set_u8(out.addr(), u8::from(flag));
    out.addr()
}

// Translated from 00d47120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getDescribedVersion` (Xbox PDB): the class's described version (`+0x2C`).
pub fn hk_class_get_described_version(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2c)
}

// Translated from 00d47130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::retrieveMember` (Xbox PDB; `this` = a class record, then `(index, out_default,
/// out_member)`, `RET 0xC`): finds the class of the chain that holds member `index` (parents
/// first) and, when it has a default-offset table (`+0x20`) whose entry for the member is not
/// negative, stores the default's address (table start plus the entry) at `out_default` and
/// the member record's address at `out_member` and answers 0. Answers 1 otherwise.
pub fn hk_class_retrieve_member(
    e: &mut Engine,
    this: Ptr,
    index: i32,
    out_default: Ptr,
    out_member: Ptr,
) -> u32 {
    let total = hk_class_get_num_members(e, this);
    let mut rest = index.wrapping_sub(total);
    let mut class = this.addr();
    if class == 0 {
        return 1;
    }
    loop {
        rest = rest.wrapping_add(e.mem.i32(class + 0x1c));
        if rest >= 0 {
            break;
        }
        class = e.mem.u32(class + 4);
        if class == 0 {
            return 1;
        }
    }
    let defaults = e.mem.u32(class + 0x20);
    if defaults == 0 {
        return 1;
    }
    let offset = e
        .mem
        .i32(defaults.wrapping_add((rest as u32).wrapping_mul(4)));
    if offset < 0 {
        return 1;
    }
    e.mem
        .set_u32(out_default.addr(), defaults.wrapping_add(offset as u32));
    let members = e.mem.u32(class + 0x18);
    e.mem.set_u32(
        out_member.addr(),
        members.wrapping_add((rest as u32).wrapping_mul(0x18)),
    );
    0
}

// Translated from 00d471c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getDefault` (Xbox PDB; `this` = a class record, then `(index, writer)`, `RET 8`):
/// looks the member up with [`hk_class_retrieve_member`]; when that fails answers 1. Otherwise
/// asks `hkClassMember::getSizeInBytes` (`00d86980`) for the member and calls the writer's
/// virtual method at slot `0x10` with the default's address and that size, and answers 0.
pub fn hk_class_get_default(e: &mut Engine, this: Ptr, index: i32, writer: Ptr) -> u32 {
    // The two out words live on the frame (the exe keeps the second in its argument slot).
    e.with_stack(8, |e, frame| {
        let default_slot = frame;
        let member_slot = frame.byte_add(4);
        let failed = hk_class_retrieve_member(e, this, index, default_slot, member_slot);
        if failed != 0 {
            return failed;
        }
        let member = e.mem.u32(member_slot.addr());
        let default_address = e.mem.u32(default_slot.addr());
        let size = e.call(MEMBER_GET_SIZE_IN_BYTES, &args![member]).u32();
        e.vcall(writer.addr(), 0x10, &args![default_address, size]);
        0
    })
}

// Translated from 00d47260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getAttribute` (Xbox PDB; `this` = a class record, then `name`, `RET 4`): the
/// class's attribute list (`+0x24`) searched with [`hk_custom_attributes_get_attribute`]; 0
/// when the class has none.
pub fn hk_class_get_attribute(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let attributes = e.mem.u32(this.addr() + 0x24);
    if attributes == 0 {
        return 0;
    }
    hk_custom_attributes_get_attribute(e, Ptr::new(attributes), name)
}

// Translated from 00d47280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getFlags` (Xbox PDB): the address of the flags word inside the record
/// (`this + 0x28`).
pub fn hk_class_get_flags(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr() + 0x28
}

// Translated from 00d47300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = a class record, then `name`, `RET 4`): the first of the class's own
/// enum records ([`fn_00d46ee0`], counted by [`fn_00d46ef0`]) whose name (its first word)
/// equals `name` under `hkString::strCmp`, or 0.
pub fn fn_00d47300(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let mut index = 0i32;
    if fn_00d46ef0(e, this) <= 0 {
        return 0;
    }
    loop {
        let record = fn_00d46ee0(e, this, index);
        let record_name = e.mem.u32(record);
        if e.call(STR_CMP, &args![record_name, name]).i32() == 0 {
            return record;
        }
        index = index.wrapping_add(1);
        if index >= fn_00d46ef0(e, this) {
            return 0;
        }
    }
}

// Translated from 00d47490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::writeSignature` (Xbox PDB; `this` = a class record, then `stream`, `RET 4`):
/// feeds the class's description to `stream` through a local `hkOArchive` over it (the byte
/// flag handed to the archive constructor is the low byte of the word `this & 0xFFFFFF00`,
/// so 0): the word at `+0xC`; every enum record (`+0x10`, `+0x14` of them) through
/// `hkClassEnum::writeSignature`; the enum count; for each declared member (`+0x18`, `+0x1C`
/// of them): the signatures of the member's class and of its parents (unless the class
/// pointer is 0 or the member's type or subtype is 0x14), the member's enum, the member's
/// name (`writeRaw` of `strLen` bytes), the type, a second byte, an optional third byte, and
/// two words made of the flags, the c-array size and the offset; then the member count. A
/// member of type 0x18 or 0x1F gets its flags xor-ed with eight times the size of its
/// subtype's type properties and a second byte of 0; a flags bit 0x400 is cleared, the type
/// byte becomes 0x13, the old second byte moves to the optional third byte and the old type
/// becomes the second byte.
pub fn hk_class_write_signature(e: &mut Engine, this: Ptr, stream: u32) {
    let t = this.addr();
    e.with_stack(0x10, |e, archive| {
        e.call(OARCHIVE_CONSTRUCT, &args![archive, stream, t & 0xffff_ff00]);
        let word = e.mem.u32(t + 0xc);
        e.call(OARCHIVE_WRITE32, &args![archive, word]);
        let mut index = 0i32;
        if e.mem.i32(t + 0x14) > 0 {
            loop {
                let enums = e.mem.u32(t + 0x10);
                let record = enums.wrapping_add((index as u32).wrapping_mul(0x14));
                e.call(CLASS_ENUM_WRITE_SIGNATURE, &args![record, stream]);
                index += 1;
                if index >= e.mem.i32(t + 0x14) {
                    break;
                }
            }
        }
        let enum_count = e.mem.u32(t + 0x14);
        e.call(OARCHIVE_WRITE32, &args![archive, enum_count]);
        let mut member_index = 0i32;
        if e.mem.i32(t + 0x1c) > 0 {
            let mut offset = 0u32;
            loop {
                let member = e.mem.u32(t + 0x18).wrapping_add(offset);
                let name = e.mem.u32(member);
                let member_class = e.mem.u32(member + 4);
                let enum_record = e.mem.u32(member + 8);
                let type_word = e.mem.u32(member + 0xc);
                let flag_word = e.mem.u32(member + 0x10);
                let mut flags = flag_word as u16;
                let offset_field = (flag_word >> 16) as u16;
                let array_size = (type_word >> 16) as u16;
                let mut type_byte = type_word as u8;
                let mut second = if type_byte == 0x18 || type_byte == 0x1f {
                    let subtype = (type_word >> 8) & 0xff;
                    let properties = e.call(MEMBER_TYPE_PROPERTIES, &args![subtype]).u32();
                    let size = e.mem.u16(properties + 8);
                    flags ^= size.wrapping_mul(8);
                    0
                } else {
                    (type_word >> 8) as u8
                };
                let mut third = 0u32;
                if flags & 0x400 != 0 {
                    flags ^= 0x400;
                    if second != 0 {
                        third = u32::from(second);
                    }
                    second = type_byte;
                    type_byte = 0x13;
                }
                if member_class != 0
                    && e.mem.u8(member + 0xc) != 0x14
                    && e.mem.u8(member + 0xd) != 0x14
                {
                    let mut class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                    while class != 0 {
                        hk_class_write_signature(e, Ptr::new(class), stream);
                        class = hk_class_get_parent(e, Ptr::new(class));
                    }
                }
                if enum_record != 0 {
                    let enum_record = e.call(MEMBER_GET_ENUM, &args![member]).u32();
                    e.call(CLASS_ENUM_WRITE_SIGNATURE, &args![enum_record, stream]);
                }
                let length = e.call(STR_LEN, &args![name]).u32();
                e.call(OARCHIVE_WRITE_RAW, &args![archive, name, length]);
                e.call(OARCHIVE_WRITE16, &args![archive, u32::from(type_byte)]);
                e.call(OARCHIVE_WRITE16, &args![archive, u32::from(second)]);
                if third as u16 != 0 {
                    e.call(OARCHIVE_WRITE16, &args![archive, third]);
                }
                let size_and_flags = u32::from(array_size) | (u32::from(flags) << 16);
                e.call(OARCHIVE_WRITE16, &args![archive, size_and_flags]);
                let flags_and_offset = u32::from(flags) | (u32::from(offset_field) << 16);
                e.call(OARCHIVE_WRITE16, &args![archive, flags_and_offset]);
                offset = offset.wrapping_add(0x18);
                member_index += 1;
                if member_index >= e.mem.i32(t + 0x1c) {
                    break;
                }
            }
        }
        let member_count = e.mem.u32(t + 0x1c);
        e.call(OARCHIVE_WRITE32, &args![archive, member_count]);
        e.call(OARCHIVE_DESTRUCT, &args![archive]);
    });
}

// Translated from 00d47690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::getSignature` (Xbox PDB; `this` = a class record, then `flag`, `RET 4`): feeds
/// the class's description ([`hk_class_write_signature`]) into a local `hkCrc32StreamWriter`
/// started at 0, then, while bit 0 of `flag` is clear, the descriptions of its parents one
/// after the other; answers the checksum.
pub fn hk_class_get_signature(e: &mut Engine, this: Ptr, flag: u32) -> u32 {
    e.with_stack(0xc, |e, writer| {
        e.call(CRC_WRITER_CONSTRUCT, &args![writer, 0u32]);
        let continue_to_parents = flag & 1 == 0;
        let mut class = this;
        while !class.is_null() {
            hk_class_write_signature(e, class, writer.addr());
            if !continue_to_parents {
                break;
            }
            class = Ptr::new(hk_class_get_parent(e, class));
        }
        e.call(CRC_WRITER_GET_CRC, &args![writer]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00d472a0, fn_00d472a0_entry as AbiFn),
        entry!(0x00d46cf0, hk_custom_attributes_get_attribute(Ptr, u32) -> u32),
        entry!(0x00d46d40, hk_class_get_name(Ptr) -> u32),
        entry!(0x00d46d50, hk_class_get_parent(Ptr) -> u32),
        entry!(0x00d46d60, hk_class_get_parent_ov2(Ptr) -> u32),
        entry!(0x00d46d80, fn_00d46d80(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d46de0, hk_class_get_num_interfaces(Ptr) -> i32),
        entry!(0x00d46e20, fn_00d46e20(Ptr) -> i32),
        entry!(0x00d46e30, fn_00d46e30(Ptr) -> i32),
        entry!(0x00d46e50, fn_00d46e50(Ptr, i32) -> u32),
        entry!(0x00d46e90, fn_00d46e90(Ptr, u32) -> u32),
        entry!(0x00d46ee0, fn_00d46ee0(Ptr, i32) -> u32),
        entry!(0x00d46ef0, fn_00d46ef0(Ptr) -> i32),
        entry!(0x00d46f00, hk_class_get_num_members(Ptr) -> i32),
        entry!(0x00d46f20, hk_class_get_member(Ptr, i32) -> u32),
        entry!(0x00d46f60, hk_class_get_member_ov2(Ptr, i32) -> u32),
        entry!(0x00d46f70, hk_class_get_num_declared_members(Ptr) -> i32),
        entry!(0x00d46f80, hk_class_get_declared_member(Ptr, i32) -> u32),
        entry!(0x00d46f90, fn_00d46f90(Ptr, u32) -> u32),
        entry!(0x00d46fe0, hk_class_get_member_by_name(Ptr, u32) -> u32),
        entry!(0x00d47030, hk_class_get_member_index_by_name(Ptr, u32) -> i32),
        entry!(0x00d47080, hk_class_get_declared_member_index_by_name(Ptr, u32) -> i32),
        entry!(0x00d470d0, hk_class_get_object_size(Ptr) -> i32),
        entry!(0x00d470e0, fn_00d470e0(Ptr, i32)),
        entry!(0x00d470f0, hk_class_has_vtable(Ptr, Ptr) -> u32),
        entry!(0x00d47120, hk_class_get_described_version(Ptr) -> u32),
        entry!(0x00d47130, hk_class_retrieve_member(Ptr, i32, Ptr, Ptr) -> u32),
        entry!(0x00d471c0, hk_class_get_default(Ptr, i32, Ptr) -> u32),
        entry!(0x00d47260, hk_class_get_attribute(Ptr, u32) -> u32),
        entry!(0x00d47280, hk_class_get_flags(Ptr) -> u32),
        entry!(0x00d47300, fn_00d47300(Ptr, u32) -> u32),
        entry!(0x00d47490, hk_class_write_signature(Ptr, u32)),
        entry!(0x00d47690, hk_class_get_signature(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_twelve_of_the_thirteen_words() {
        let mut e = Engine::new();
        let c = e.mem.alloc(0x30);
        let mut words = vec![c];
        words.extend(1..=13u32);
        e.call(0x00d472a0, &words);
        let got: Vec<u32> = (0..12).map(|i| e.mem.u32(c + 4 * i)).collect();
        assert_eq!(got, [1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    const WRITER_DOUBLE: u32 = 0x0900_0011;

    fn str_cmp_double(e: &mut Engine, a: &[u32]) -> Ret {
        let first = e.mem.cstr(a[0]);
        let second = e.mem.cstr(a[1]);
        Ret {
            eax: first.cmp(&second) as i32 as u32,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STR_CMP, str_cmp_double);
        e
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let a = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(a, s.as_bytes());
        a
    }

    /// A class record with a name and a parent.
    fn class(e: &mut Engine, name: &str, parent: u32) -> u32 {
        let c = e.mem.alloc(0x30);
        let n = text(e, name);
        e.mem.set_u32(c, n);
        e.mem.set_u32(c + 4, parent);
        c
    }

    /// Gives the class one array of `stride`-byte records (the pointer at `array_field`, the
    /// count right after it), each record starting with its name pointer.
    fn records(e: &mut Engine, class: u32, array_field: u32, stride: u32, names: &[&str]) -> u32 {
        let base = e.mem.alloc(stride * names.len().max(1) as u32);
        for (i, name) in names.iter().enumerate() {
            let n = text(e, name);
            e.mem.set_u32(base + stride * i as u32, n);
        }
        e.mem.set_u32(class + array_field, base);
        e.mem.set_u32(class + array_field + 4, names.len() as u32);
        base
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn simple_accessors_read_and_write_their_fields() {
        let mut e = engine();
        let parent = class(&mut e, "Base", 0);
        let c = class(&mut e, "Derived", parent);
        let name = e.mem.u32(c);
        e.mem.set_u32(c + 0xc, 4);
        e.mem.set_u32(c + 0x2c, 0x77);
        let members = records(&mut e, c, 0x18, 0x18, &["a", "b", "c"]);
        let enums = records(&mut e, c, 0x10, 0x14, &["x", "y"]);
        assert_eq!(e.call(0x00d46d40, &args![c]).u32(), name);
        assert_eq!(e.call(0x00d46d50, &args![c]).u32(), parent);
        assert_eq!(e.call(0x00d46d60, &args![c]).u32(), parent);
        assert_eq!(e.call(0x00d46e20, &args![c]).i32(), 4);
        assert_eq!(e.call(0x00d46ef0, &args![c]).i32(), 2);
        assert_eq!(e.call(0x00d46ee0, &args![c, 1i32]).u32(), enums + 0x14);
        assert_eq!(e.call(0x00d46f70, &args![c]).i32(), 3);
        assert_eq!(e.call(0x00d46f80, &args![c, 2i32]).u32(), members + 0x30);
        assert_eq!(e.call(0x00d47120, &args![c]).u32(), 0x77);
        assert_eq!(e.call(0x00d47280, &args![c]).u32(), c + 0x28);
        e.call(0x00d470e0, &args![c, 0x40i32]);
        assert_eq!(e.mem.u32(c + 8), 0x40);
        assert_eq!(e.call(0x00d470d0, &args![c]).i32(), 0x40);
    }

    #[test]
    fn counts_add_up_over_the_parent_chain() {
        let mut e = engine();
        let grandparent = class(&mut e, "G", 0);
        let parent = class(&mut e, "P", grandparent);
        let c = class(&mut e, "C", parent);
        for (record, interfaces, enums, members) in [
            (grandparent, 1u32, 2u32, 3u32),
            (parent, 10, 20, 30),
            (c, 100, 200, 300),
        ] {
            e.mem.set_u32(record + 0xc, interfaces);
            e.mem.set_u32(record + 0x14, enums);
            e.mem.set_u32(record + 0x1c, members);
        }
        assert_eq!(e.call(0x00d46de0, &args![c]).i32(), 111);
        assert_eq!(e.call(0x00d46e30, &args![c]).i32(), 222);
        assert_eq!(e.call(0x00d46f00, &args![c]).i32(), 333);
        assert_eq!(e.call(0x00d46f00, &args![grandparent]).i32(), 3);
    }

    #[test]
    fn members_and_enums_are_indexed_parents_first() {
        let mut e = engine();
        let parent = class(&mut e, "P", 0);
        let c = class(&mut e, "C", parent);
        let parent_members = records(&mut e, parent, 0x18, 0x18, &["a", "b"]);
        let own_members = records(&mut e, c, 0x18, 0x18, &["c"]);
        let parent_enums = records(&mut e, parent, 0x10, 0x14, &["x"]);
        let own_enums = records(&mut e, c, 0x10, 0x14, &["y", "z"]);
        let member = |e: &mut Engine, addr: u32, index: i32| e.call(addr, &args![c, index]).u32();
        for addr in [0x00d46f20, 0x00d46f60] {
            assert_eq!(member(&mut e, addr, 0), parent_members);
            assert_eq!(member(&mut e, addr, 1), parent_members + 0x18);
            assert_eq!(member(&mut e, addr, 2), own_members);
            // Past the end: the own array's record, as far as the index reaches.
            assert_eq!(member(&mut e, addr, 5), own_members + 0x18 * 3);
            // Before the start: the own array.
            assert_eq!(member(&mut e, addr, -10), own_members);
        }
        assert_eq!(member(&mut e, 0x00d46e50, 0), parent_enums);
        assert_eq!(member(&mut e, 0x00d46e50, 1), own_enums);
        assert_eq!(member(&mut e, 0x00d46e50, 2), own_enums + 0x14);
        assert_eq!(member(&mut e, 0x00d46e50, -10), own_enums);
    }

    #[test]
    fn member_lookups_by_name() {
        let mut e = engine();
        let parent = class(&mut e, "P", 0);
        let c = class(&mut e, "C", parent);
        let parent_members = records(&mut e, parent, 0x18, 0x18, &["a", "b"]);
        let own_members = records(&mut e, c, 0x18, 0x18, &["c", "b"]);
        let empty = class(&mut e, "E", 0);
        let b = text(&mut e, "b");
        let c_name = text(&mut e, "c");
        let missing = text(&mut e, "zz");
        e.call_log = Some(Vec::new());
        assert_eq!(
            e.call(0x00d46fe0, &args![c, b]).u32(),
            parent_members + 0x18
        );
        assert_eq!(e.call(0x00d46fe0, &args![c, c_name]).u32(), own_members);
        assert_eq!(e.call(0x00d46fe0, &args![c, missing]).u32(), 0);
        assert_eq!(e.call(0x00d47030, &args![c, b]).i32(), 1);
        assert_eq!(e.call(0x00d47030, &args![c, c_name]).i32(), 2);
        assert_eq!(e.call(0x00d47030, &args![c, missing]).i32(), -1);
        assert_eq!(e.call(0x00d46f90, &args![c, b]).u32(), own_members + 0x18);
        assert_eq!(e.call(0x00d46f90, &args![c, missing]).u32(), 0);
        assert_eq!(e.call(0x00d47080, &args![c, b]).i32(), 1);
        assert_eq!(e.call(0x00d47080, &args![c, missing]).i32(), -1);
        // A class without members answers without comparing anything.
        let before = calls(&e, STR_CMP).len();
        assert_eq!(e.call(0x00d46fe0, &args![empty, b]).u32(), 0);
        assert_eq!(e.call(0x00d47030, &args![empty, b]).i32(), -1);
        assert_eq!(e.call(0x00d46f90, &args![empty, b]).u32(), 0);
        assert_eq!(e.call(0x00d47080, &args![empty, b]).i32(), -1);
        assert_eq!(calls(&e, STR_CMP).len(), before);
        // The comparison gets the member's name first.
        let first = calls(&e, STR_CMP)[0].clone();
        assert_eq!(first[1], b);
    }

    #[test]
    fn enum_lookups_by_name() {
        let mut e = engine();
        let parent = class(&mut e, "P", 0);
        let c = class(&mut e, "C", parent);
        let parent_enums = records(&mut e, parent, 0x10, 0x14, &["x"]);
        let own_enums = records(&mut e, c, 0x10, 0x14, &["y", "z"]);
        let empty = class(&mut e, "E", 0);
        let x = text(&mut e, "x");
        let z = text(&mut e, "z");
        let missing = text(&mut e, "nope");
        assert_eq!(e.call(0x00d46e90, &args![c, x]).u32(), parent_enums);
        assert_eq!(e.call(0x00d46e90, &args![c, z]).u32(), own_enums + 0x14);
        assert_eq!(e.call(0x00d46e90, &args![c, missing]).u32(), 0);
        assert_eq!(e.call(0x00d46e90, &args![empty, x]).u32(), 0);
        assert_eq!(e.call(0x00d47300, &args![c, z]).u32(), own_enums + 0x14);
        assert_eq!(e.call(0x00d47300, &args![c, x]).u32(), 0);
        assert_eq!(e.call(0x00d47300, &args![empty, x]).u32(), 0);
    }

    #[test]
    fn class_chain_name_test_stores_whether_a_class_of_the_chain_matches() {
        let mut e = engine();
        let root = class(&mut e, "Root", 0);
        let middle = class(&mut e, "Middle", root);
        let leaf = class(&mut e, "Leaf", middle);
        let same_as_root = class(&mut e, "Root", 0);
        let other = class(&mut e, "Other", 0);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0x55);
        e.call_log = Some(Vec::new());
        assert_eq!(
            e.call(0x00d46d80, &args![same_as_root, out, leaf]).u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        // Names are compared as (candidate, own) until a match.
        let own = e.mem.u32(same_as_root);
        let compared = calls(&e, STR_CMP);
        assert_eq!(compared.len(), 3);
        assert_eq!(compared[2], vec![e.mem.u32(root), own]);
        assert_eq!(e.call(0x00d46d80, &args![other, out, leaf]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.set_u8(out, 0x55);
        assert_eq!(e.call(0x00d46d80, &args![other, out, 0u32]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn has_vtable_looks_at_the_topmost_class() {
        let mut e = engine();
        let root = class(&mut e, "Root", 0);
        let leaf = class(&mut e, "Leaf", root);
        let out = e.mem.alloc(8);
        e.mem.set_u32(root + 0xc, 0);
        e.mem.set_u32(leaf + 0xc, 3);
        assert_eq!(e.call(0x00d470f0, &args![leaf, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.set_u32(root + 0xc, 2);
        e.mem.set_u32(leaf + 0xc, 0);
        e.call(0x00d470f0, &args![leaf, out]);
        assert_eq!(e.mem.u8(out), 1);
        // A class without a parent answers for itself.
        e.call(0x00d470f0, &args![root, out]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(root + 0xc, 0);
        e.call(0x00d470f0, &args![root, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// A parent with two members and a child with one member and a default table.
    fn default_fixture(e: &mut Engine) -> (u32, u32, u32) {
        let parent = class(e, "P", 0);
        let c = class(e, "C", parent);
        records(e, parent, 0x18, 0x18, &["a", "b"]);
        let own_members = records(e, c, 0x18, 0x18, &["c"]);
        let table = e.mem.alloc(0x20);
        e.mem.set_i32(table, 0x10);
        e.mem.set_u32(c + 0x20, table);
        (c, own_members, table)
    }

    #[test]
    fn retrieve_member_finds_the_default_of_a_member() {
        let mut e = engine();
        let (c, own_members, table) = default_fixture(&mut e);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00d47130, &args![c, 2i32, out, out + 4]).u32(), 0);
        assert_eq!(e.mem.u32(out), table + 0x10);
        assert_eq!(e.mem.u32(out + 4), own_members);
        // A negative table entry is a failure.
        e.mem.set_i32(table, -1);
        e.mem.set_u32(out, 0);
        assert_eq!(e.call(0x00d47130, &args![c, 2i32, out, out + 4]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0);
        // No table.
        e.mem.set_u32(c + 0x20, 0);
        assert_eq!(e.call(0x00d47130, &args![c, 2i32, out, out + 4]).u32(), 1);
        // An index before the first member never reaches a class.
        assert_eq!(e.call(0x00d47130, &args![c, -5i32, out, out + 4]).u32(), 1);
    }

    #[test]
    fn get_default_hands_the_default_and_the_member_size_to_the_writer() {
        let mut e = engine();
        let (c, own_members, table) = default_fixture(&mut e);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, WRITER_DOUBLE);
        let writer = e.mem.alloc(8);
        e.mem.set_u32(writer, vtable);
        e.register(MEMBER_GET_SIZE_IN_BYTES, |_, _| Ret {
            eax: 12,
            ..Ret::default()
        });
        e.register(WRITER_DOUBLE, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        assert_eq!(e.call(0x00d471c0, &args![c, 2i32, writer]).u32(), 0);
        assert_eq!(calls(&e, MEMBER_GET_SIZE_IN_BYTES), vec![vec![own_members]]);
        assert_eq!(
            calls(&e, WRITER_DOUBLE),
            vec![vec![writer, table + 0x10, 12]]
        );
        // The member lookup failing answers 1 and writes nothing.
        e.mem.set_u32(c + 0x20, 0);
        assert_eq!(e.call(0x00d471c0, &args![c, 2i32, writer]).u32(), 1);
        assert_eq!(calls(&e, WRITER_DOUBLE).len(), 1);
    }

    #[test]
    fn attribute_lists_are_searched_by_name() {
        let mut e = engine();
        let c = class(&mut e, "C", 0);
        let list = e.mem.alloc(8);
        let data = e.mem.alloc(24);
        let first = text(&mut e, "first");
        let second = text(&mut e, "second");
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 12, second);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, 2);
        assert_eq!(e.call(0x00d46cf0, &args![list, second]).u32(), data + 16);
        assert_eq!(e.call(0x00d46cf0, &args![list, first]).u32(), data + 4);
        let missing = text(&mut e, "third");
        assert_eq!(e.call(0x00d46cf0, &args![list, missing]).u32(), 0);
        e.mem.set_u32(list + 4, 0);
        assert_eq!(e.call(0x00d46cf0, &args![list, first]).u32(), 0);
        // The class's own method: no list, no search; otherwise the list's.
        e.call_log = Some(Vec::new());
        assert_eq!(e.call(0x00d47260, &args![c, first]).u32(), 0);
        assert!(calls(&e, STR_CMP).is_empty());
        e.mem.set_u32(list + 4, 2);
        e.mem.set_u32(c + 0x24, list);
        assert_eq!(e.call(0x00d47260, &args![c, second]).u32(), data + 16);
    }

    /// Doubles for everything `writeSignature` calls outside this file.
    fn archive_doubles(e: &mut Engine) {
        let properties = e.mem.alloc(0x10);
        e.mem.set_u16(properties + 8, 4);
        e.register_double(MEMBER_TYPE_PROPERTIES, move |_, _| Ret {
            eax: properties,
            ..Ret::default()
        });
        e.register(MEMBER_GET_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(MEMBER_GET_ENUM, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        for addr in [
            OARCHIVE_CONSTRUCT,
            OARCHIVE_WRITE32,
            OARCHIVE_WRITE16,
            OARCHIVE_WRITE_RAW,
            OARCHIVE_DESTRUCT,
            CLASS_ENUM_WRITE_SIGNATURE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// The logged calls other than the string comparisons and the top-level call.
    fn signature_log(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .filter(|(a, _)| *a != STR_CMP)
            .cloned()
            .collect()
    }

    #[test]
    fn write_signature_describes_enums_and_members() {
        let mut e = engine();
        archive_doubles(&mut e);
        let c = class(&mut e, "C", 0);
        e.mem.set_u32(c + 0xc, 0x1234);
        let enums = records(&mut e, c, 0x10, 0x14, &["E"]);
        let members = records(&mut e, c, 0x18, 0x18, &["alpha", "beta", "gamma"]);
        // alpha: a plain member (type 3, subtype 5, c-array size 7, flags 1, offset 0x20).
        e.mem.set_u32(members + 0xc, 3 | (5 << 8) | (7 << 16));
        e.mem.set_u32(members + 0x10, 1 | (0x20 << 16));
        // beta: an array type 0x18 of subtype 2, flags 2, offset 0x30.
        let beta = members + 0x18;
        e.mem.set_u32(beta + 0xc, 0x18 | (2 << 8));
        e.mem.set_u32(beta + 0x10, 2 | (0x30 << 16));
        // gamma: type 7, subtype 9, flags 0x410, a class with a parent, and an enum.
        let gamma = members + 0x30;
        let struct_parent = class(&mut e, "SP", 0);
        e.mem.set_u32(struct_parent + 0xc, 0x66);
        let struct_class = class(&mut e, "S", struct_parent);
        e.mem.set_u32(struct_class + 0xc, 0x55);
        e.mem.set_u32(gamma + 4, struct_class);
        e.mem.set_u32(gamma + 8, enums);
        e.mem.set_u32(gamma + 0xc, 7 | (9 << 8));
        e.mem.set_u32(gamma + 0x10, 0x410 | (0x40 << 16));
        let stream = e.mem.alloc(0x10);
        e.call_log = Some(Vec::new());
        e.call(0x00d47490, &args![c, stream]);
        let log = signature_log(&e);
        let archive: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == OARCHIVE_CONSTRUCT)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(archive.len(), 3);
        let name = |member: u32| e.mem.u32(member);
        let with_high = |low: u32, high: u32| low | (high << 16);
        let mut expected: Vec<(u32, Vec<u32>)> = vec![
            (
                OARCHIVE_CONSTRUCT,
                vec![archive[0], stream, c & 0xffff_ff00],
            ),
            (OARCHIVE_WRITE32, vec![archive[0], 0x1234]),
            (CLASS_ENUM_WRITE_SIGNATURE, vec![enums, stream]),
            (OARCHIVE_WRITE32, vec![archive[0], 1]),
            // alpha
            (STR_LEN, vec![name(members)]),
            (OARCHIVE_WRITE_RAW, vec![archive[0], name(members), 5]),
            (OARCHIVE_WRITE16, vec![archive[0], 3]),
            (OARCHIVE_WRITE16, vec![archive[0], 5]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(7, 1)]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(1, 0x20)]),
            // beta: the flags are xor-ed with 8 * the subtype's size (4)
            (MEMBER_TYPE_PROPERTIES, vec![2]),
            (STR_LEN, vec![name(beta)]),
            (OARCHIVE_WRITE_RAW, vec![archive[0], name(beta), 4]),
            (OARCHIVE_WRITE16, vec![archive[0], 0x18]),
            (OARCHIVE_WRITE16, vec![archive[0], 0]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(0, 0x22)]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(0x22, 0x30)]),
            // gamma: the class and its parent, then the enum
            (MEMBER_GET_CLASS, vec![gamma]),
        ];
        for (class_record, index, word) in [(struct_class, 1, 0x55), (struct_parent, 2, 0x66)] {
            expected.extend([
                (
                    OARCHIVE_CONSTRUCT,
                    vec![archive[index], stream, class_record & 0xffff_ff00],
                ),
                (OARCHIVE_WRITE32, vec![archive[index], word]),
                (OARCHIVE_WRITE32, vec![archive[index], 0]),
                (OARCHIVE_WRITE32, vec![archive[index], 0]),
                (OARCHIVE_DESTRUCT, vec![archive[index]]),
            ]);
        }
        expected.extend([
            (MEMBER_GET_ENUM, vec![gamma]),
            (CLASS_ENUM_WRITE_SIGNATURE, vec![enums, stream]),
            (STR_LEN, vec![name(gamma)]),
            (OARCHIVE_WRITE_RAW, vec![archive[0], name(gamma), 5]),
            // type becomes 0x13, the old type the second byte, the old second the third
            (OARCHIVE_WRITE16, vec![archive[0], 0x13]),
            (OARCHIVE_WRITE16, vec![archive[0], 7]),
            (OARCHIVE_WRITE16, vec![archive[0], 9]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(0, 0x10)]),
            (OARCHIVE_WRITE16, vec![archive[0], with_high(0x10, 0x40)]),
            (OARCHIVE_WRITE32, vec![archive[0], 3]),
            (OARCHIVE_DESTRUCT, vec![archive[0]]),
        ]);
        assert_eq!(log, expected);
    }

    #[test]
    fn write_signature_skips_the_class_of_members_of_type_0x14() {
        let mut e = engine();
        archive_doubles(&mut e);
        let c = class(&mut e, "C", 0);
        let members = records(&mut e, c, 0x18, 0x18, &["m"]);
        let other = class(&mut e, "O", 0);
        e.mem.set_u32(members + 4, other);
        e.mem.set_u32(members + 0xc, 0x14);
        let stream = e.mem.alloc(0x10);
        e.call_log = Some(Vec::new());
        e.call(0x00d47490, &args![c, stream]);
        assert!(calls(&e, MEMBER_GET_CLASS).is_empty());
        assert_eq!(calls(&e, OARCHIVE_CONSTRUCT).len(), 1);
        // The same through the subtype byte.
        e.mem.set_u32(members + 0xc, 1 | (0x14 << 8));
        e.call_log = Some(Vec::new());
        e.call(0x00d47490, &args![c, stream]);
        assert!(calls(&e, MEMBER_GET_CLASS).is_empty());
        // Any other type reaches the class.
        e.mem.set_u32(members + 0xc, 1 | (2 << 8));
        e.call_log = Some(Vec::new());
        e.call(0x00d47490, &args![c, stream]);
        assert_eq!(calls(&e, MEMBER_GET_CLASS).len(), 1);
        assert_eq!(calls(&e, OARCHIVE_CONSTRUCT).len(), 2);
    }

    #[test]
    fn get_signature_checksums_the_class_and_optionally_its_parents() {
        let mut e = engine();
        archive_doubles(&mut e);
        e.register(CRC_WRITER_CONSTRUCT, |_, _| Ret::default());
        e.register(CRC_WRITER_GET_CRC, |_, _| Ret {
            eax: 0xabcd_1234,
            ..Ret::default()
        });
        let parent = class(&mut e, "P", 0);
        let c = class(&mut e, "C", parent);
        e.call_log = Some(Vec::new());
        assert_eq!(e.call(0x00d47690, &args![c, 0u32]).u32(), 0xabcd_1234);
        let writer = calls(&e, CRC_WRITER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, CRC_WRITER_CONSTRUCT), vec![vec![writer, 0]]);
        assert_eq!(calls(&e, CRC_WRITER_GET_CRC), vec![vec![writer]]);
        let streams: Vec<u32> = calls(&e, OARCHIVE_CONSTRUCT).iter().map(|w| w[1]).collect();
        assert_eq!(streams, vec![writer, writer]);
        // With bit 0 of the flag set only the class itself is written.
        e.call_log = Some(Vec::new());
        e.call(0x00d47690, &args![c, 1u32]);
        assert_eq!(calls(&e, OARCHIVE_CONSTRUCT).len(), 1);
        // A null class writes nothing.
        e.call_log = Some(Vec::new());
        e.call(0x00d47690, &args![0u32, 0u32]);
        assert!(calls(&e, OARCHIVE_CONSTRUCT).is_empty());
    }
}
