//! `NiXenonRenderer/nidynamicgeometrygroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryBufferData`'s table (`010f017c`).
const GEOMETRY_BUFFER_DATA_VTABLE: u32 = 0x010f_017c;

// Translated from 00e947c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryBufferData::NiGeometryBufferData` (engine map), `thiscall`, no stack words: the
/// table `010f017c`, the words +4 to +0x10 cleared, the byte +0x14 cleared, +0x18 to +0x38
/// cleared, +0x3c set to 4 and +0x40 to +0x50 cleared. Answers `this`.
pub fn ni_geometry_buffer_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, GEOMETRY_BUFFER_DATA_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base.wrapping_add(0x14), 0);
    for offset in (0x18u32..=0x38).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base.wrapping_add(0x3c), 4);
    for offset in (0x40u32..=0x50).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    this
}

/// The first node of a pointer map (`NiTMapBase`: vtable +0, bucket count +4, buckets +8): the
/// first non-null bucket, or 0.
fn map_first(e: &mut Engine, map: u32) -> u32 {
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    for index in 0..count {
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
    }
    0
}

/// The node after `node` (next +0, key +4): its next, or the first non-null bucket after the
/// key's bucket (the map's virtual slot 4, `thiscall` with the key), or 0.
fn map_next(e: &mut Engine, map: u32, node: u32) -> u32 {
    let next = e.mem.u32(node);
    if next != 0 {
        return next;
    }
    let key = e.mem.u32(node.wrapping_add(4));
    let mut index = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
    while index < e.mem.u32(map.wrapping_add(4)) {
        let buckets = e.mem.u32(map.wrapping_add(8));
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
        index += 1;
    }
    0
}

/// The map removal `00e91fd0` (unit `nid3dshaderfactory.cpp`; `thiscall` on the map, the key)
/// and the Ni free `00aa1460` (`cdecl`: block, size).
const MAP_REMOVE_00E91FD0: u32 = 0x00e9_1fd0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

/// A 0x18-byte buffer record freed: the objects at +0x10 and then +0xc released (their slot
/// 8, `stdcall` with the object) when set, then the record freed.
fn free_buffer_record(e: &mut Engine, record: u32) {
    for offset in [0x10u32, 0xc] {
        let object = e.mem.u32(record.wrapping_add(offset));
        if object != 0 {
            e.vcall(object, 8, &args![]);
        }
    }
    e.call(NI_FREE_00AA1460, &args![record, 0x18u32]);
}

/// Every node of the buffer map at `map` (+0xc of the group): the next node found first, the
/// key removed (`00e91fd0`), and a value (a buffer record) freed.
fn purge_buffer_map(e: &mut Engine, map: u32) {
    let mut node = map_first(e, map);
    while node != 0 {
        let key = e.mem.u32(node.wrapping_add(4));
        let value = e.mem.u32(node.wrapping_add(8));
        node = map_next(e, map, node);
        e.call(MAP_REMOVE_00E91FD0, &args![map, key]);
        if value != 0 {
            free_buffer_record(e, value);
        }
    }
}

/// The record at `index` of the group's buffer array (data +0x30, size +0x36, count +0x38,
/// all 16-bit but the data) taken out as `NiTPrimitiveArray::RemoveAt` inlines it: the slot
/// cleared, the count down for a record, the size down when it was the last slot. Answers the
/// record (0 for an index at or past the size, nothing changed then).
fn take_from_array(e: &mut Engine, data: u32, size: u32, count: u32, index: u32) -> Option<u32> {
    if index >= u32::from(e.mem.u16(size)) {
        return None;
    }
    let slot = e.mem.u32(data).wrapping_add(index.wrapping_mul(4));
    let record = e.mem.u32(slot);
    e.mem.set_u32(slot, 0);
    if record != 0 {
        let left = e.mem.u16(count).wrapping_add(0xffff);
        e.mem.set_u16(count, left);
    }
    let last = e.mem.u16(size).wrapping_sub(1);
    if index == u32::from(last) {
        e.mem.set_u16(size, last);
    }
    Some(record)
}

// Translated from 00e94f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::Purge` (engine map), `thiscall`, no stack words. Every node of the
/// buffer map (+0xc) is removed (`00e91fd0` with its key) and its buffer record freed (the
/// objects at +0x10 and +0xc released, 0x18 bytes); every node of the chip map (+0x1c) gets its
/// chip's +8 cleared. Then for each index below the 16-bit word +0x34: the buffer array's
/// record at that index (data +0x30, size +0x36, count +0x38) is taken out and freed the same
/// way, and the chip at that index of the array at +0x40 gets +8 cleared.
pub fn ni_dynamic_geometry_group_purge(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    purge_buffer_map(e, base.wrapping_add(0xc));
    let chips = base.wrapping_add(0x1c);
    let mut node = map_first(e, chips);
    while node != 0 {
        let value = e.mem.u32(node.wrapping_add(8));
        node = map_next(e, chips, node);
        e.mem.set_u32(value.wrapping_add(8), 0);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base.wrapping_add(0x34))) {
        let taken = take_from_array(
            e,
            base.wrapping_add(0x30),
            base.wrapping_add(0x36),
            base.wrapping_add(0x38),
            index,
        );
        if let Some(record) = taken {
            if record != 0 {
                free_buffer_record(e, record);
            }
        }
        let chip_array = e.mem.u32(base.wrapping_add(0x40));
        let chip = e.mem.u32(chip_array.wrapping_add(index.wrapping_mul(4)));
        e.mem.set_u32(chip.wrapping_add(8), 0);
        index += 1;
    }
}

/// `NiGeometryGroup::RemoveObject` (`00e98750`, engine map; `thiscall`, the buffer data) and
/// `memset` (`00ec61c0`, `cdecl`).
const GROUP_REMOVE_OBJECT_00E98750: u32 = 0x00e9_8750;
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00e950f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::RemoveObject` (engine map), `thiscall` with the geometry data.
/// Nothing when the data has no buffer data (+0x34). Otherwise `NiGeometryGroup::RemoveObject`
/// (`00e98750`, same `this`) with it, its chip array (+0x28; count +0x20) zeroed when set, the
/// buffer data deleted (its slot 0 with 1) and the data's word cleared.
pub fn ni_dynamic_geometry_group_remove_object(e: &mut Engine, this: Ptr, data: u32) {
    let buffer_data = e.mem.u32(data.wrapping_add(0x34));
    if buffer_data == 0 {
        return;
    }
    e.call(GROUP_REMOVE_OBJECT_00E98750, &args![this, buffer_data]);
    let chips = e.mem.u32(buffer_data.wrapping_add(0x28));
    if chips != 0 {
        let count = e.mem.u32(buffer_data.wrapping_add(0x20));
        e.call(MEMSET, &args![chips, 0u32, count.wrapping_mul(4)]);
    }
    e.vcall(buffer_data, 0, &args![1u32]);
    e.mem.set_u32(data.wrapping_add(0x34), 0);
}

/// The chip's release `00a89af0` (unnamed; `thiscall`, one word; its body is a bare `RET 4`).
const CHIP_RELEASE_00A89AF0: u32 = 0x00a8_9af0;

// Translated from 00e951e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::ReleaseChip` (engine map), `thiscall` with two words (`this` not
/// read): the buffer data and the stream index. For an index below the data's count (+0x20)
/// whose chip (array +0x28) is set: a chip holding +8 gets `00a89af0` on its +4 with that word
/// and +8 cleared; then the slot is cleared (the index checked again).
pub fn ni_dynamic_geometry_group_release_chip(
    e: &mut Engine,
    _this: Ptr,
    buffer_data: u32,
    index: u32,
) {
    if index >= e.mem.u32(buffer_data.wrapping_add(0x20)) {
        return;
    }
    let chips = e.mem.u32(buffer_data.wrapping_add(0x28));
    let chip = e.mem.u32(chips.wrapping_add(index.wrapping_mul(4)));
    if chip == 0 {
        return;
    }
    let held = e.mem.u32(chip.wrapping_add(8));
    if held != 0 {
        let owner = e.mem.u32(chip.wrapping_add(4));
        e.call(CHIP_RELEASE_00A89AF0, &args![owner, held]);
        e.mem.set_u32(chip.wrapping_add(8), 0);
    }
    if index < e.mem.u32(buffer_data.wrapping_add(0x20)) {
        let chips = e.mem.u32(buffer_data.wrapping_add(0x28));
        e.mem.set_u32(chips.wrapping_add(index.wrapping_mul(4)), 0);
    }
}

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;

// Translated from 00e95230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidynamicgeometrygroup.cpp`): a new 0x18-byte buffer record,
/// `cdecl` with four words: the size, the device, and the words for +4 and +0. Null when the
/// allocation (`00aa13e0`) fails. Otherwise +0 to +0x10 cleared, the size rounded (a size with
/// any of its low four bits set becomes its multiple of 16 plus 0x20) stored at +8, the old +0xc
/// released when set (it was just cleared), the device kept at +0xc and counted (its slot 4,
/// `stdcall`) when not null, +4 and +0 set from the third and fourth words.
pub fn fn_00e95230(e: &mut Engine, size: u32, device: u32, third: u32, fourth: u32) -> u32 {
    let record = e.call(NI_ALLOCATE_00AA13E0, &args![0x18u32]).u32();
    if record == 0 {
        return 0;
    }
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        e.mem.set_u32(record + offset, 0);
    }
    let rounded = if size & 0xf != 0 {
        (size & 0xffff_fff0).wrapping_add(0x20)
    } else {
        size
    };
    e.mem.set_u32(record + 8, rounded);
    let old = e.mem.u32(record + 0xc);
    if old != 0 {
        e.vcall(old, 8, &args![]);
    }
    if device != 0 {
        e.mem.set_u32(record + 0xc, device);
        e.vcall(device, 4, &args![]);
    } else {
        e.mem.set_u32(record + 0xc, 0);
    }
    e.mem.set_u32(record + 4, third);
    e.mem.set_u32(record, fourth);
    record
}

/// The memory manager's allocate (`00aa3e40`, `thiscall` on `011f6238`, the size), the
/// signature check `0043b300` (unit `modelloader.cpp`; `cdecl`: a record at `011f4a70` and the
/// data), the RTTI record it compares the data's class chain with (`011f4a88`), the global word
/// the group's flag is capped by (`011bf558`), `NiGeometryGroup::AddObject` (`00e98730`,
/// engine map; `thiscall`, the buffer data) and `NiGeometryData::MarkAsChanged` (`00a67090`,
/// engine map; `thiscall`, the flags).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const SIGNATURE_CHECK_0043B300: u32 = 0x0043_b300;
const SIGNATURE_011F4A70: u32 = 0x011f_4a70;
const RTTI_011F4A88: u32 = 0x011f_4a88;
const STREAM_LIMIT_011BF558: u32 = 0x011b_f558;
const GROUP_ADD_OBJECT_00E98730: u32 = 0x00e9_8730;
const MARK_AS_CHANGED_00A67090: u32 = 0x00a6_7090;

/// A new `NiGeometryBufferData` (0x54 bytes from the memory manager, 0 when that fails).
fn new_buffer_data(e: &mut Engine) -> u32 {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x54u32],
        )
        .u32();
    if block == 0 {
        return 0;
    }
    ni_geometry_buffer_data_construct(e, Ptr::new(block)).addr()
}

// Translated from 00e953c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::AddObject` (engine map), `thiscall` with three words: the geometry
/// data, a word never read and the skin partition. With a partition: nothing when it already
/// has buffer data (+0x28); otherwise new buffer data (+0x3c 5 when the partition's 16-bit
/// +0x22 is set, else 4) goes to its +0x28. Without one: nothing when the data has buffer data
/// (+0x34); otherwise new buffer data whose +0x3c becomes 4 when the data's class chain (its
/// virtual slot 8, then each record's +4) reaches `011f4a88`, else 5 when `0043b300` answers
/// set for the data, and which goes to the data's +0x34. Then the buffer data's +4 is built
/// from the data: bit 0 of its 16-bit +0xc (capped by the word at `011bf558`) shifted to the
/// top byte, 0x400000 with +0x28 set, 0x800000 with +0x24 set and 0x200000 with +0x24 set and
/// any of the bits 0xf000 of +0xc; it is added to the group (`00e98730`) and the data marked
/// changed (`00a67090` with 0xffff). A failed allocation leaves 0, which the code writes
/// through.
pub fn ni_dynamic_geometry_group_add_object(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    _unused_2: u32,
    partition: u32,
) {
    let buffer_data;
    if partition != 0 {
        if e.mem.u32(partition.wrapping_add(0x28)) != 0 {
            return;
        }
        buffer_data = new_buffer_data(e);
        let kind = if e.mem.u16(partition.wrapping_add(0x22)) != 0 {
            5
        } else {
            4
        };
        e.mem.set_u32(buffer_data.wrapping_add(0x3c), kind);
        e.mem.set_u32(partition.wrapping_add(0x28), buffer_data);
    } else {
        if e.mem.u32(data.wrapping_add(0x34)) != 0 {
            return;
        }
        buffer_data = new_buffer_data(e);
        let mut rtti = e.vcall(data, 8, &args![]).u32();
        let mut found = false;
        while rtti != 0 {
            if rtti == RTTI_011F4A88 {
                found = true;
                break;
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
        if found {
            e.mem.set_u32(buffer_data.wrapping_add(0x3c), 4);
        } else if e
            .call(SIGNATURE_CHECK_0043B300, &args![SIGNATURE_011F4A70, data])
            .bool()
        {
            e.mem.set_u32(buffer_data.wrapping_add(0x3c), 5);
        }
        e.mem.set_u32(data.wrapping_add(0x34), buffer_data);
    }
    let has_28 = e.mem.u32(data.wrapping_add(0x28)) != 0;
    let has_24 = e.mem.u32(data.wrapping_add(0x24)) != 0;
    let flags = e.mem.u16(data.wrapping_add(0xc));
    let marked = has_24 && flags & 0xf000 != 0;
    let mut streams = u32::from(flags) & 1;
    let limit = e.mem.u32(STREAM_LIMIT_011BF558);
    if streams > limit {
        streams = limit;
    }
    let mut bits = if has_28 { 0x40_0000 } else { 0 };
    if has_24 {
        bits |= 0x80_0000;
    }
    if marked {
        bits |= 0x20_0000;
    }
    e.mem
        .set_u32(buffer_data.wrapping_add(4), (streams << 24) | bits);
    e.call(GROUP_ADD_OBJECT_00E98730, &args![this, buffer_data]);
    e.call(MARK_AS_CHANGED_00A67090, &args![data, 0xffffu32]);
}

/// What `CreateChip` calls: the arrays' `SetSize` (`00a90180`, `thiscall`, the size) and
/// `SetAt` (`00ba3aa0`, `thiscall`: index and the address of the element), the maps' `GetAt`
/// (`00a53c30`, `thiscall`: key and the address of the value; answers a byte) and `SetAt`
/// (`00b57ea0`, `thiscall`: key and value), the buffer lock `00e987c0` (unnamed; `stdcall`:
/// size, the addresses of the offset and of a word for the chip's +0x10, a byte flag and the
/// data's +0xc with its low byte replaced by its byte +0x14) and the chip maker `00537d60`
/// (unnamed; `cdecl`, five words).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const MAP_GET_AT_00A53C30: u32 = 0x00a5_3c30;
const MAP_SET_AT_00B57EA0: u32 = 0x00b5_7ea0;
const LOCK_00E987C0: u32 = 0x00e9_87c0;
const NEW_CHIP_00537D60: u32 = 0x0053_7d60;

// Translated from 00e95550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::CreateChip` (engine map), `thiscall` with two words: the buffer
/// data and the stream index. The stream's stride is the data's array +0x24 at the index (0
/// past the count +0x20), the size the data's +0x1c times it. With more than one stream the
/// two arrays (+0x2c, +0x3c) are grown to the stream count when their 16-bit +0x34 is below it,
/// the cursor +0x4c wraps to 0 at that size, the buffer record at the cursor (a new one,
/// `00e95230` with 0x800 and the device +8, stored with `SetAt`, when the slot is empty) is
/// locked (`00e987c0` with flag 1) and the chip at the cursor in +0x3c gets the locked buffer,
/// the offset and the size (a new one, `00537d60`, stored with `SetAt`, when the slot is
/// empty); the chip's +4 takes the record and +0x10 the lock's second word, the data's +0x38 is
/// cleared and the cursor moves on. With one stream the key is the data's +0xc (when 0, the
/// stride with the top bit set): the record comes from the map at +0xc (made and stored when
/// absent or null, with the data's +0xc as its third word), the lock runs with flag 0 and the
/// chip comes from the map at +0x1c (made and stored when absent); the data's +0x38 becomes the
/// offset divided by the stride. Answers the chip.
pub fn ni_dynamic_geometry_group_create_chip(
    e: &mut Engine,
    this: Ptr,
    buffer_data: u32,
    index: u32,
) -> u32 {
    let base = this.addr();
    let streams = e.mem.u32(buffer_data.wrapping_add(0x20));
    let key_word = e.mem.u32(buffer_data.wrapping_add(0xc));
    let stride = if index < streams {
        let strides = e.mem.u32(buffer_data.wrapping_add(0x24));
        e.mem.u32(strides.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    };
    let size = e
        .mem
        .u32(buffer_data.wrapping_add(0x1c))
        .wrapping_mul(stride);
    // The locals the callees write through: the record (+0), the chip (+4), the lock's
    // second word (+8) and the offset (+0xc).
    e.with_stack(0x10, |e, locals| {
        let locals = locals.addr();
        let record_slot = locals;
        let chip_slot = locals + 4;
        let second_slot = locals + 8;
        let offset_slot = locals + 0xc;
        let flag_word =
            (key_word & 0xffff_ff00) | u32::from(e.mem.u8(buffer_data.wrapping_add(0x14)));
        if streams > 1 {
            if u32::from(e.mem.u16(base.wrapping_add(0x34))) < streams {
                e.call(
                    ARRAY_SET_SIZE_00A90180,
                    &args![base.wrapping_add(0x2c), streams],
                );
                e.call(
                    ARRAY_SET_SIZE_00A90180,
                    &args![base.wrapping_add(0x3c), streams],
                );
            }
            if e.mem.u32(base.wrapping_add(0x4c)) >= u32::from(e.mem.u16(base.wrapping_add(0x34))) {
                e.mem.set_u32(base.wrapping_add(0x4c), 0);
            }
            let cursor = e.mem.u32(base.wrapping_add(0x4c));
            let records = e.mem.u32(base.wrapping_add(0x30));
            let record = e.mem.u32(records.wrapping_add(cursor.wrapping_mul(4)));
            e.mem.set_u32(record_slot, record);
            if record == 0 {
                let device = e.mem.u32(base.wrapping_add(8));
                let made = fn_00e95230(e, 0x800, device, 0, 0);
                let cursor = e.mem.u32(base.wrapping_add(0x4c));
                e.mem.set_u32(record_slot, made);
                e.call(
                    ARRAY_SET_AT_00BA3AA0,
                    &args![base.wrapping_add(0x2c), cursor, record_slot],
                );
            }
            // 00e9561c: `00e987c0` is a thiscall (RET 0x14) on the record (ECX from
            // the record slot), then the five pushed words.
            let record = e.mem.u32(record_slot);
            let locked = e
                .call(
                    LOCK_00E987C0,
                    &args![record, size, offset_slot, second_slot, 1u32, flag_word],
                )
                .u32();
            let cursor = e.mem.u32(base.wrapping_add(0x4c));
            let chips = e.mem.u32(base.wrapping_add(0x40));
            let mut chip = e.mem.u32(chips.wrapping_add(cursor.wrapping_mul(4)));
            if chip == 0 {
                let offset = e.mem.u32(offset_slot);
                chip = e
                    .call(NEW_CHIP_00537D60, &args![0u32, locked, offset, size, 0u32])
                    .u32();
                let cursor = e.mem.u32(base.wrapping_add(0x4c));
                e.mem.set_u32(chip_slot, chip);
                e.call(
                    ARRAY_SET_AT_00BA3AA0,
                    &args![base.wrapping_add(0x3c), cursor, chip_slot],
                );
            } else {
                e.mem.set_u32(chip.wrapping_add(8), locked);
                let offset = e.mem.u32(offset_slot);
                e.mem.set_u32(chip.wrapping_add(0xc), offset);
                e.mem.set_u32(chip.wrapping_add(0x14), size);
            }
            let record = e.mem.u32(record_slot);
            e.mem.set_u32(chip.wrapping_add(4), record);
            let second = e.mem.u32(second_slot);
            e.mem.set_u32(chip.wrapping_add(0x10), second);
            e.mem.set_u32(buffer_data.wrapping_add(0x38), 0);
            let cursor = e.mem.u32(base.wrapping_add(0x4c)).wrapping_add(1);
            e.mem.set_u32(base.wrapping_add(0x4c), cursor);
            return chip;
        }
        let key = if key_word != 0 {
            key_word
        } else {
            stride | 0x8000_0000
        };
        let found = e
            .call(
                MAP_GET_AT_00A53C30,
                &args![base.wrapping_add(0xc), key, record_slot],
            )
            .bool();
        if !found || e.mem.u32(record_slot) == 0 {
            let device = e.mem.u32(base.wrapping_add(8));
            let made = fn_00e95230(e, 0x800, device, key_word, 0);
            e.mem.set_u32(record_slot, made);
            e.call(
                MAP_SET_AT_00B57EA0,
                &args![base.wrapping_add(0xc), key, made],
            );
        }
        // 00e956f9: as above, `00e987c0` is a thiscall on the record in the slot.
        let record = e.mem.u32(record_slot);
        let locked = e
            .call(
                LOCK_00E987C0,
                &args![record, size, offset_slot, second_slot, 0u32, flag_word],
            )
            .u32();
        e.call(
            MAP_GET_AT_00A53C30,
            &args![base.wrapping_add(0x1c), key, chip_slot],
        );
        let mut chip = e.mem.u32(chip_slot);
        if chip == 0 {
            let offset = e.mem.u32(offset_slot);
            chip = e
                .call(NEW_CHIP_00537D60, &args![0u32, locked, offset, size, 0u32])
                .u32();
            e.call(
                MAP_SET_AT_00B57EA0,
                &args![base.wrapping_add(0x1c), key, chip],
            );
        } else {
            e.mem.set_u32(chip.wrapping_add(8), locked);
            let offset = e.mem.u32(offset_slot);
            e.mem.set_u32(chip.wrapping_add(0xc), offset);
            e.mem.set_u32(chip.wrapping_add(0x14), size);
        }
        let record = e.mem.u32(record_slot);
        e.mem.set_u32(chip.wrapping_add(4), record);
        let second = e.mem.u32(second_slot);
        e.mem.set_u32(chip.wrapping_add(0x10), second);
        // `DIV`: a stride of 0 faults in the game as it panics here.
        let offset = e.mem.u32(offset_slot);
        e.mem
            .set_u32(buffer_data.wrapping_add(0x38), offset / stride);
        chip
    })
}

/// What the constructor `00e957b0` stores and calls: the group's base constructor `00e986f0`
/// (unnamed; `thiscall`, no stack words), the tables of the group (`010f10c4`), of the buffer
/// map (`010f103c`, then `010f107c`), of the chip map (`010f105c`, then `010f109c`) and of the
/// two arrays (`010f10bc` at +0x2c, `010f0f8c` at +0x3c), the bucket allocation `00aa1070`
/// (`cdecl`, the byte count).
const BASE_CONSTRUCT_00E986F0: u32 = 0x00e9_86f0;
const GROUP_VTABLE_010F10C4: u32 = 0x010f_10c4;
const BUFFER_MAP_BASE_VTABLE_010F103C: u32 = 0x010f_103c;
const BUFFER_MAP_VTABLE_010F107C: u32 = 0x010f_107c;
const CHIP_MAP_BASE_VTABLE_010F105C: u32 = 0x010f_105c;
const CHIP_MAP_VTABLE_010F109C: u32 = 0x010f_109c;
const RECORD_ARRAY_VTABLE_010F10BC: u32 = 0x010f_10bc;
const CHIP_ARRAY_VTABLE_010F0F8C: u32 = 0x010f_0f8c;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;

/// A map set up as the constructor inlines it: the base table, 0x25 buckets, count 0, the
/// buckets (0x94 bytes) allocated and zeroed (`memset` with the bucket count read back), then
/// the map's own table.
fn construct_map(e: &mut Engine, map: u32, base_table: u32, table: u32) {
    e.mem.set_u32(map, base_table);
    e.mem.set_u32(map.wrapping_add(4), 0x25);
    e.mem.set_u32(map.wrapping_add(0xc), 0);
    let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
    e.mem.set_u32(map.wrapping_add(8), buckets);
    let count = e.mem.u32(map.wrapping_add(4));
    e.call(MEMSET, &args![buckets, 0u32, count.wrapping_mul(4)]);
    e.mem.set_u32(map, table);
}

// Translated from 00e957b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidynamicgeometrygroup.cpp`): the `NiDynamicGeometryGroup`
/// constructor, `thiscall`, no stack words. The base constructor `00e986f0`, the group's
/// table, the buffer map (+0xc) and the chip map (+0x1c) set up (0x25 buckets each), the
/// record array at +0x2c (data +0x30 null, 16-bit size +0x34, +0x36 and count +0x38 cleared,
/// growth +0x3a 1) and the chip array at +0x3c (the same at +0x40 to +0x4a), and the cursor
/// +0x4c cleared. Answers `this`.
pub fn fn_00e957b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00E986F0, &args![base]);
    e.mem.set_u32(base, GROUP_VTABLE_010F10C4);
    construct_map(
        e,
        base.wrapping_add(0xc),
        BUFFER_MAP_BASE_VTABLE_010F103C,
        BUFFER_MAP_VTABLE_010F107C,
    );
    construct_map(
        e,
        base.wrapping_add(0x1c),
        CHIP_MAP_BASE_VTABLE_010F105C,
        CHIP_MAP_VTABLE_010F109C,
    );
    e.mem.set_u32(base.wrapping_add(0x30), 0);
    e.mem
        .set_u32(base.wrapping_add(0x2c), RECORD_ARRAY_VTABLE_010F10BC);
    e.mem.set_u16(base.wrapping_add(0x34), 0);
    e.mem.set_u16(base.wrapping_add(0x38), 0);
    e.mem.set_u16(base.wrapping_add(0x3a), 1);
    e.mem.set_u16(base.wrapping_add(0x36), 0);
    e.mem.set_u16(base.wrapping_add(0x46), 0);
    e.mem.set_u32(base.wrapping_add(0x40), 0);
    e.mem.set_u16(base.wrapping_add(0x44), 0);
    e.mem.set_u16(base.wrapping_add(0x4a), 1);
    e.mem.set_u16(base.wrapping_add(0x48), 0);
    e.mem
        .set_u32(base.wrapping_add(0x3c), CHIP_ARRAY_VTABLE_010F0F8C);
    e.mem.set_u32(base.wrapping_add(0x4c), 0);
    this
}

/// What the destructor `00e95880` adds: the group singleton (`0126fa38`), the arrays' base
/// tables (`010f0f7c` at +0x3c, `010f1034` at +0x2c), the map clean-up `00b601a0` (unnamed;
/// `thiscall`, no stack words), the free `00aa10f0` (`cdecl`, one word) and the base destructor
/// `00e98710` (unnamed; `thiscall`, no stack words).
const GROUP_0126FA38: u32 = 0x0126_fa38;
const CHIP_ARRAY_BASE_VTABLE_010F0F7C: u32 = 0x010f_0f7c;
const RECORD_ARRAY_BASE_VTABLE_010F1034: u32 = 0x010f_1034;
const MAP_CLEAN_UP_00B601A0: u32 = 0x00b6_01a0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const BASE_DESTRUCT_00E98710: u32 = 0x00e9_8710;

/// A map undone as the destructor inlines it: its table, `00b601a0`, its base table,
/// `00b601a0`, and its buckets freed.
fn destruct_map(e: &mut Engine, map: u32, table: u32, base_table: u32) {
    e.mem.set_u32(map, table);
    e.call(MAP_CLEAN_UP_00B601A0, &args![map]);
    e.mem.set_u32(map, base_table);
    e.call(MAP_CLEAN_UP_00B601A0, &args![map]);
    let buckets = e.mem.u32(map.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
}

// Translated from 00e95880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidynamicgeometrygroup.cpp`): the `NiDynamicGeometryGroup`
/// destructor, `thiscall`, no stack words. Stores the group's table; every node of the buffer
/// map is removed (`00e91fd0`) and its record freed (see `Purge`); every node of the chip map
/// is removed and its chip's +8 cleared and the chip freed (0x20 bytes). For each index below
/// the 16-bit +0x34 the record array's element is taken out and freed and the chip array's
/// (data +0x40, size +0x46, count +0x48; 0 past the size, written through) gets +8 cleared and
/// is freed. The singleton `0126fa38` is cleared when it is this group; the arrays' base tables
/// stored and their data freed (`00aa10f0`), the maps undone (chip map first) and the base
/// destructor `00e98710` run (a tail jump).
pub fn fn_00e95880(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, GROUP_VTABLE_010F10C4);
    purge_buffer_map(e, base.wrapping_add(0xc));
    let chips = base.wrapping_add(0x1c);
    let mut node = map_first(e, chips);
    while node != 0 {
        let key = e.mem.u32(node.wrapping_add(4));
        let chip = e.mem.u32(node.wrapping_add(8));
        node = map_next(e, chips, node);
        e.call(MAP_REMOVE_00E91FD0, &args![chips, key]);
        e.mem.set_u32(chip.wrapping_add(8), 0);
        e.call(NI_FREE_00AA1460, &args![chip, 0x20u32]);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base.wrapping_add(0x34))) {
        let taken = take_from_array(
            e,
            base.wrapping_add(0x30),
            base.wrapping_add(0x36),
            base.wrapping_add(0x38),
            index,
        );
        if let Some(record) = taken {
            if record != 0 {
                free_buffer_record(e, record);
            }
        }
        let chip = take_from_array(
            e,
            base.wrapping_add(0x40),
            base.wrapping_add(0x46),
            base.wrapping_add(0x48),
            index,
        )
        .unwrap_or(0);
        e.mem.set_u32(chip.wrapping_add(8), 0);
        e.call(NI_FREE_00AA1460, &args![chip, 0x20u32]);
        index += 1;
    }
    if base == e.mem.u32(GROUP_0126FA38) {
        e.mem.set_u32(GROUP_0126FA38, 0);
    }
    let chip_data = e.mem.u32(base.wrapping_add(0x40));
    e.mem
        .set_u32(base.wrapping_add(0x3c), CHIP_ARRAY_BASE_VTABLE_010F0F7C);
    e.call(FREE_00AA10F0, &args![chip_data]);
    let record_data = e.mem.u32(base.wrapping_add(0x30));
    e.mem
        .set_u32(base.wrapping_add(0x2c), RECORD_ARRAY_BASE_VTABLE_010F1034);
    e.call(FREE_00AA10F0, &args![record_data]);
    destruct_map(
        e,
        base.wrapping_add(0x1c),
        CHIP_MAP_VTABLE_010F109C,
        CHIP_MAP_BASE_VTABLE_010F105C,
    );
    destruct_map(
        e,
        base.wrapping_add(0xc),
        BUFFER_MAP_VTABLE_010F107C,
        BUFFER_MAP_BASE_VTABLE_010F103C,
    );
    e.call(BASE_DESTRUCT_00E98710, &args![base]);
}

// Translated from 00e95b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidynamicgeometrygroup.cpp`): the dynamic group `CreateGroup`
/// makes for type 2, no arguments. Answers the singleton at `0126fa38`; when there is none,
/// makes it (0x50 bytes, `00e957b0`), stores and answers it (a failed allocation stores and
/// answers 0).
pub fn fn_00e95b10(e: &mut Engine) -> u32 {
    let group = e.mem.u32(GROUP_0126FA38);
    if group != 0 {
        return group;
    }
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x50u32]).u32();
    if block == 0 {
        e.mem.set_u32(GROUP_0126FA38, 0);
        return 0;
    }
    let group = fn_00e957b0(e, Ptr::new(block)).addr();
    e.mem.set_u32(GROUP_0126FA38, group);
    group
}

// Translated from 00e95b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup`'s scalar deleting destructor (engine map), `thiscall` with the
/// flags: the destructor `00e95880`, then with bit 0 set the 0x50 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_dynamic_geometry_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e95880(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x50u32]);
    }
    this
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
const MEMORY_MANAGER_FREE_00AA4060: u32 = 0x00aa_4060;

// Translated from 00a93c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiVBDynamicSet*,NiTMallocInterface<NiVBDynamicSet*>>::RemoveAt` (Xbox PDB, engine
/// map; other arrays share the body), `thiscall`, one word (the index): for an index below
/// the 16-bit size (+0x0A), takes the element out of the data (+0x04), counts one element
/// less in the 16-bit +0x0C when it was not null, and shrinks the size by one when the index
/// was the last slot. Answers the element (0 for an index past the size).
pub fn ni_t_array_ni_vb_dynamic_set_remove_at(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = this.addr();
    if index >= u32::from(e.mem.u16(array + 0xa)) {
        return 0;
    }
    let slot = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4));
    let element = e.mem.u32(slot);
    e.mem.set_u32(slot, 0);
    if element != 0 {
        let count = e.mem.u16(array + 0xc).wrapping_add(0xffff);
        e.mem.set_u16(array + 0xc, count);
    }
    let last = e.mem.u16(array + 0xa).wrapping_sub(1);
    if index == u32::from(last) {
        e.mem.set_u16(array + 0xa, last);
    }
    element
}

/// The removal of the two `RemoveObject` overloads below, inlined there on the object's word
/// `slot`: a held buffer data is taken out of the group (`NiGeometryGroup::RemoveObject`,
/// `00e98750`), its chip array (+0x28, count +0x20) zeroed when set, the data deleted (its
/// slot 0 with 1) and the word cleared.
fn remove_buffer_data_at(e: &mut Engine, this: Ptr, object: u32, slot: u32) {
    let buffer_data = e.mem.u32(object.wrapping_add(slot));
    if buffer_data == 0 {
        return;
    }
    e.call(GROUP_REMOVE_OBJECT_00E98750, &args![this, buffer_data]);
    let chips = e.mem.u32(buffer_data.wrapping_add(0x28));
    if chips != 0 {
        let count = e.mem.u32(buffer_data.wrapping_add(0x20));
        e.call(MEMSET, &args![chips, 0u32, count.wrapping_mul(4)]);
    }
    e.vcall(buffer_data, 0, &args![1u32]);
    e.mem.set_u32(object.wrapping_add(slot), 0);
}

// Translated from 00e95140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::RemoveObject` (Xbox PDB, the engine map's `_ov2`), `thiscall`, one
/// word: as `00e950f0` for the buffer data at +0x28 of its argument.
pub fn ni_dynamic_geometry_group_remove_object_ov2(e: &mut Engine, this: Ptr, object: u32) {
    remove_buffer_data_at(e, this, object, 0x28);
}

// Translated from 00e95190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::RemoveObject` (Xbox PDB, the engine map's `_ov3`), `thiscall`, one
/// word: as `00e950f0` for the buffer data at +0x1C of its argument.
pub fn ni_dynamic_geometry_group_remove_object_ov3(e: &mut Engine, this: Ptr, object: u32) {
    remove_buffer_data_at(e, this, object, 0x1c);
}

/// The map scalar deleting destructors below: each vtable in `tables` installed in turn with
/// the map emptied under it (`00b601a0`), then the bucket table (+0x08) freed (`00aa10f0`)
/// and, with bit 0 of `flags`, the map freed through the memory manager. Answers `this`.
fn delete_map(e: &mut Engine, this: Ptr, flags: u32, tables: &[u32]) -> Ptr {
    let map = this.addr();
    for table in tables {
        e.mem.set_u32(map, *table);
        e.call(MAP_CLEAN_UP_00B601A0, &args![map]);
    }
    let buckets = e.mem.u32(map + 8);
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_FREE_00AA4060,
            &args![MEMORY_MANAGER_OBJECT, map],
        );
    }
    this
}

// Translated from 00e952c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,NiVBDynamicSet*>` scalar
/// deleting destructor (its vtable `010f103c`'s; unnamed in the engine map), `thiscall` with
/// one stack word (the flags): empties the map under its vtable, frees the buckets and, with
/// bit 0, the map.
pub fn fn_00e952c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map(e, this, flags, &[BUFFER_MAP_BASE_VTABLE_010F103C])
}

// Translated from 00e95300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<unsigned int,NiVBDynamicSet*>` scalar deleting destructor (vtable
/// `010f107c`; unnamed in the engine map), `thiscall` with one stack word (the flags): empties
/// the map under its own vtable and again under its base's (`010f103c`), frees the buckets
/// and, with bit 0, the map.
pub fn fn_00e95300(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map(
        e,
        this,
        flags,
        &[BUFFER_MAP_VTABLE_010F107C, BUFFER_MAP_BASE_VTABLE_010F103C],
    )
}

// Translated from 00e95340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,NiVBChip*>` scalar deleting
/// destructor (vtable `010f105c`; unnamed in the engine map): as `00e952c0`.
pub fn fn_00e95340(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map(e, this, flags, &[CHIP_MAP_BASE_VTABLE_010F105C])
}

// Translated from 00e95380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<unsigned int,NiVBChip*>` scalar deleting destructor (vtable `010f109c`;
/// unnamed in the engine map): as `00e95300` with the chip map's vtables (`010f109c`, then
/// `010f105c`).
pub fn fn_00e95380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map(
        e,
        this,
        flags,
        &[CHIP_MAP_VTABLE_010F109C, CHIP_MAP_BASE_VTABLE_010F105C],
    )
}

// Translated from 00e95500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicGeometryGroup::AddObject` (Xbox PDB, the engine map's `_ov2`), `thiscall`, one
/// word: when the object's +0x1C is 0, makes a buffer data (0x54 bytes from the memory
/// manager, constructed), sets its +0x3C to 4, stores it at the object's +0x1C, sets its
/// `m_uiFlags` (+0x04) to `0x01400000`, adds it to the group (`00e98730`) and sets the
/// object's 16-bit +0x18 to 0xffff. A failed allocation leaves the data null and the exe
/// writes through it anyway.
pub fn ni_dynamic_geometry_group_add_object_ov2(e: &mut Engine, this: Ptr, object: u32) {
    if e.mem.u32(object.wrapping_add(0x1c)) != 0 {
        return;
    }
    let data = new_buffer_data(e);
    e.mem.set_u32(data.wrapping_add(0x3c), 4);
    e.mem.set_u32(object.wrapping_add(0x1c), data);
    e.mem.set_u32(data.wrapping_add(4), 0x0140_0000);
    e.call(GROUP_ADD_OBJECT_00E98730, &args![this, data]);
    e.mem.set_u16(object.wrapping_add(0x18), 0xffff);
}

// Translated from 00e95780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<NiVBDynamicSet*,NiTMallocInterface<NiVBDynamicSet*>>` scalar deleting
/// destructor (vtable `010f1034`; unnamed in the engine map), `thiscall` with one stack word
/// (the flags): installs the vtable, frees the element block (+0x04, `00aa10f0`) and, with
/// bit 0 of the flags, the array through the memory manager. Answers `this`.
pub fn fn_00e95780(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let array = this.addr();
    let elements = e.mem.u32(array + 4);
    e.mem.set_u32(array, RECORD_ARRAY_BASE_VTABLE_010F1034);
    e.call(FREE_00AA10F0, &args![elements]);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_FREE_00AA4060,
            &args![MEMORY_MANAGER_OBJECT, array],
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_3c50,
            ni_t_array_ni_vb_dynamic_set_remove_at(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_5140,
            ni_dynamic_geometry_group_remove_object_ov2(Ptr, u32)
        ),
        entry!(
            0x00e9_5190,
            ni_dynamic_geometry_group_remove_object_ov3(Ptr, u32)
        ),
        entry!(0x00e9_52c0, fn_00e952c0(Ptr, u32) -> Ptr),
        entry!(0x00e9_5300, fn_00e95300(Ptr, u32) -> Ptr),
        entry!(0x00e9_5340, fn_00e95340(Ptr, u32) -> Ptr),
        entry!(0x00e9_5380, fn_00e95380(Ptr, u32) -> Ptr),
        entry!(
            0x00e9_5500,
            ni_dynamic_geometry_group_add_object_ov2(Ptr, u32)
        ),
        entry!(0x00e9_5780, fn_00e95780(Ptr, u32) -> Ptr),
        entry!(0x00e9_47c0, ni_geometry_buffer_data_construct(Ptr) -> Ptr),
        entry!(0x00e9_4f30, ni_dynamic_geometry_group_purge(Ptr)),
        entry!(
            0x00e9_50f0,
            ni_dynamic_geometry_group_remove_object(Ptr, u32)
        ),
        entry!(
            0x00e9_51e0,
            ni_dynamic_geometry_group_release_chip(Ptr, u32, u32)
        ),
        entry!(0x00e9_5230, fn_00e95230(u32, u32, u32, u32) -> u32),
        entry!(
            0x00e9_53c0,
            ni_dynamic_geometry_group_add_object(Ptr, u32, u32, u32)
        ),
        entry!(0x00e9_5550, ni_dynamic_geometry_group_create_chip(Ptr, u32, u32) -> u32),
        entry!(0x00e9_57b0, fn_00e957b0(Ptr) -> Ptr),
        entry!(0x00e9_5880, fn_00e95880(Ptr)),
        entry!(0x00e9_5b10, fn_00e95b10() -> u32),
        entry!(
            0x00e9_5b40,
            ni_dynamic_geometry_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const KEY_TO_INDEX: u32 = 0x0e60_0004;
    const MAP_VTABLE: u32 = 0x0e61_0000;
    const OBJECT_VTABLE: u32 = 0x0e62_0000;
    const DELETE: u32 = 0x0e63_0000;
    const ADD_REF: u32 = 0x0e63_0004;
    const RELEASE: u32 = 0x0e63_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for no_op in [
            DELETE,
            ADD_REF,
            RELEASE,
            MAP_REMOVE_00E91FD0,
            NI_FREE_00AA1460,
            MEMSET,
            CHIP_RELEASE_00A89AF0,
            GROUP_REMOVE_OBJECT_00E98750,
            GROUP_ADD_OBJECT_00E98730,
            MARK_AS_CHANGED_00A67090,
            MAP_CLEAN_UP_00B601A0,
            FREE_00AA10F0,
            BASE_DESTRUCT_00E98710,
            BASE_CONSTRUCT_00E986F0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(KEY_TO_INDEX, |_, a| a[1].into_ret());
        e.put_vtable(MAP_VTABLE, &[0, KEY_TO_INDEX]);
        e.put_vtable(OBJECT_VTABLE, &[DELETE, ADD_REF, RELEASE]);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        object
    }

    /// A 0x18-byte buffer record holding counted objects at +0xc and +0x10.
    fn buffer_record(e: &mut Engine) -> (u32, u32, u32) {
        let record = e.mem.alloc(0x18);
        let (first, second) = (object(e), object(e));
        e.mem.set_u32(record + 0xc, first);
        e.mem.set_u32(record + 0x10, second);
        (record, first, second)
    }

    /// A group built by the constructor (the maps' buckets allocated), with 4 slots in each
    /// array (size 2) and the maps' tables replaced by the test map table.
    fn group(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x50);
        e.call(0x00e9_57b0, &args![this]);
        e.mem.set_u32(this + 0xc, MAP_VTABLE);
        e.mem.set_u32(this + 0x1c, MAP_VTABLE);
        let records = e.mem.alloc(0x10);
        let chips = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x30, records);
        e.mem.set_u32(this + 0x40, chips);
        for at in [0x34, 0x36, 0x44, 0x46] {
            e.mem.set_u16(this + at, 2);
        }
        this
    }

    #[test]
    fn geometry_buffer_data_constructor_sets_its_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.mem.write(this, &[0xcc; 0x54]);
        assert_eq!(e.call(0x00e9_47c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), GEOMETRY_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 0x11), vec![0; 0x11]);
        assert_eq!(e.mem.bytes(this + 0x15, 3), vec![0xcc; 3]);
        assert_eq!(e.mem.bytes(this + 0x18, 0x24), vec![0; 0x24]);
        assert_eq!(e.mem.u32(this + 0x3c), 4);
        assert_eq!(e.mem.bytes(this + 0x40, 0x14), vec![0; 0x14]);
    }

    #[test]
    fn constructor_and_singleton() {
        let mut e = engine();
        let group = e.call(0x00e9_5b10, &args![]).u32();
        assert_ne!(group, 0);
        assert_eq!(e.mem.u32(GROUP_0126FA38), group);
        assert_eq!(e.mem.u32(group), GROUP_VTABLE_010F10C4);
        assert_eq!(e.mem.u32(group + 0xc), BUFFER_MAP_VTABLE_010F107C);
        assert_eq!(e.mem.u32(group + 0x10), 0x25);
        assert_eq!(e.mem.u32(group + 0x1c), CHIP_MAP_VTABLE_010F109C);
        assert_eq!(e.mem.u32(group + 0x2c), RECORD_ARRAY_VTABLE_010F10BC);
        assert_eq!(e.mem.u32(group + 0x3c), CHIP_ARRAY_VTABLE_010F0F8C);
        assert_eq!(e.mem.u16(group + 0x3a), 1);
        assert_eq!(e.mem.u16(group + 0x4a), 1);
        assert_eq!(e.mem.u32(group + 0x4c), 0);
        // Already made: answered as is.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5b10, &args![]).u32(), group);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(GROUP_0126FA38, 0);
        e.register(NI_ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e9_5b10, &args![]).u32(), 0);
    }

    #[test]
    fn purge_frees_records_and_clears_chips() {
        let mut e = engine();
        let this = group(&mut e);
        // A buffer map node (key 1) with a record, a chip map node (key 2) with a chip.
        let (record, first, second) = buffer_record(&mut e);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 4, 1);
        e.mem.set_u32(node + 8, record);
        let buckets = e.mem.u32(this + 0x14);
        e.mem.set_u32(buckets + 4, node);
        let chip = e.mem.alloc(0x20);
        e.mem.set_u32(chip + 8, 9);
        let chip_node = e.mem.alloc(0xc);
        e.mem.set_u32(chip_node + 4, 2);
        e.mem.set_u32(chip_node + 8, chip);
        let buckets = e.mem.u32(this + 0x24);
        e.mem.set_u32(buckets + 8, chip_node);
        // The arrays: a record at index 1 (the last), chips at both indices.
        let (array_record, _, _) = buffer_record(&mut e);
        let records = e.mem.u32(this + 0x30);
        e.mem.set_u32(records + 4, array_record);
        e.mem.set_u16(this + 0x38, 1);
        let chips = e.mem.u32(this + 0x40);
        let (chip_a, chip_b) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(chip_a + 8, 5);
        e.mem.set_u32(chip_b + 8, 6);
        e.mem.set_u32(chips, chip_a);
        e.mem.set_u32(chips + 4, chip_b);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4f30, &args![this]);
        assert_eq!(calls(&e, MAP_REMOVE_00E91FD0), vec![vec![this + 0xc, 1]]);
        assert_eq!(
            calls(&e, RELEASE)[..2].to_vec(),
            vec![vec![second], vec![first]]
        );
        assert_eq!(
            calls(&e, NI_FREE_00AA1460),
            vec![vec![record, 0x18], vec![array_record, 0x18]]
        );
        assert_eq!(e.mem.u32(chip + 8), 0);
        assert_eq!(e.mem.u32(records + 4), 0);
        assert_eq!(e.mem.u16(this + 0x38), 0);
        assert_eq!(e.mem.u16(this + 0x36), 1);
        assert_eq!(e.mem.u32(chip_a + 8), 0);
        assert_eq!(e.mem.u32(chip_b + 8), 0);
    }

    #[test]
    fn remove_object_and_release_chip() {
        let mut e = engine();
        let buffer_data_block = e.mem.alloc(0x54);
        e.mem.set_u32(buffer_data_block, OBJECT_VTABLE);
        let chips = e.mem.alloc(8);
        e.mem.set_u32(buffer_data_block + 0x28, chips);
        e.mem.set_u32(buffer_data_block + 0x20, 2);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x34, buffer_data_block);
        e.call_log = Some(vec![]);
        e.call(0x00e9_50f0, &args![0x1234u32, data]);
        assert_eq!(
            calls(&e, GROUP_REMOVE_OBJECT_00E98750),
            vec![vec![0x1234, buffer_data_block]]
        );
        assert_eq!(calls(&e, MEMSET), vec![vec![chips, 0, 8]]);
        assert_eq!(calls(&e, DELETE), vec![vec![buffer_data_block, 1]]);
        assert_eq!(e.mem.u32(data + 0x34), 0);
        // Nothing to remove.
        e.call_log = Some(vec![]);
        e.call(0x00e9_50f0, &args![0x1234u32, data]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // ReleaseChip: the chip's +8 goes through 00a89af0 and the slot is cleared.
        let chip = e.mem.alloc(0xc);
        e.mem.set_u32(chip + 4, 0x4444);
        e.mem.set_u32(chip + 8, 0x8888);
        e.mem.set_u32(chips + 4, chip);
        e.call_log = Some(vec![]);
        e.call(0x00e9_51e0, &args![0u32, buffer_data_block, 1u32]);
        assert_eq!(calls(&e, CHIP_RELEASE_00A89AF0), vec![vec![0x4444, 0x8888]]);
        assert_eq!(e.mem.u32(chip + 8), 0);
        assert_eq!(e.mem.u32(chips + 4), 0);
        // Past the count, or an empty slot: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e9_51e0, &args![0u32, buffer_data_block, 2u32]);
        e.call(0x00e9_51e0, &args![0u32, buffer_data_block, 1u32]);
        assert!(calls(&e, CHIP_RELEASE_00A89AF0).is_empty());
    }

    #[test]
    fn fn_00e95230_rounds_and_counts_the_device() {
        let mut e = engine();
        let device = object(&mut e);
        e.call_log = Some(vec![]);
        let record = e
            .call(0x00e9_5230, &args![0x801u32, device, 3u32, 4u32])
            .u32();
        assert_eq!(e.mem.u32(record + 8), 0x820);
        assert_eq!(e.mem.u32(record + 0xc), device);
        assert_eq!(e.mem.u32(record + 4), 3);
        assert_eq!(e.mem.u32(record), 4);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        let record = e
            .call(0x00e9_5230, &args![0x800u32, 0u32, 5u32, 6u32])
            .u32();
        assert_eq!(e.mem.u32(record + 8), 0x800);
        assert_eq!(e.mem.u32(record + 0xc), 0);
        assert_eq!(e.mem.u32(record + 4), 5);
        assert_eq!(e.mem.u32(record), 6);
        e.register(NI_ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00e9_5230, &args![1u32, device, 0u32, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn add_object_builds_buffer_data() {
        let mut e = engine();
        e.register(SIGNATURE_CHECK_0043B300, |_, _| 1u32.into_ret());
        e.mem.set_u32(STREAM_LIMIT_011BF558, 4);
        // A data whose class chain does not reach 011f4a88.
        let rtti = e.mem.alloc(8);
        e.register_double(0x0e64_0008, move |_, _| rtti.into_ret());
        e.put_vtable(0x0e65_0000, &[0, 0, 0x0e64_0008]);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data, 0x0e65_0000);
        e.mem.set_u16(data + 0xc, 0x1001);
        e.mem.set_u32(data + 0x24, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_53c0, &args![0x5000u32, data, 0u32, 0u32]);
        let buffer_data = e.mem.u32(data + 0x34);
        assert_eq!(e.mem.u32(buffer_data), GEOMETRY_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.u32(buffer_data + 0x3c), 5);
        assert_eq!(e.mem.u32(buffer_data + 4), 0x01a0_0000);
        assert_eq!(
            calls(&e, SIGNATURE_CHECK_0043B300),
            vec![vec![SIGNATURE_011F4A70, data]]
        );
        assert_eq!(
            calls(&e, GROUP_ADD_OBJECT_00E98730),
            vec![vec![0x5000, buffer_data]]
        );
        assert_eq!(
            calls(&e, MARK_AS_CHANGED_00A67090),
            vec![vec![data, 0xffff]]
        );
        // Already there: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e9_53c0, &args![0x5000u32, data, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // The chain reaches 011f4a88: 4, and the cap of 0 clears the stream bit.
        e.mem.set_u32(rtti + 4, RTTI_011F4A88);
        e.mem.set_u32(data + 0x34, 0);
        e.mem.set_u32(STREAM_LIMIT_011BF558, 0);
        e.mem.set_u32(data + 0x28, 1);
        e.call(0x00e9_53c0, &args![0x5000u32, data, 0u32, 0u32]);
        let buffer_data = e.mem.u32(data + 0x34);
        assert_eq!(e.mem.u32(buffer_data + 0x3c), 4);
        assert_eq!(e.mem.u32(buffer_data + 4), 0x00e0_0000);
        // A skin partition: its own buffer data, 5 with +0x22 set.
        let partition = e.mem.alloc(0x2c);
        e.mem.set_u16(partition + 0x22, 1);
        e.call(0x00e9_53c0, &args![0x5000u32, data, 0u32, partition]);
        let partition_data = e.mem.u32(partition + 0x28);
        assert_eq!(e.mem.u32(partition_data + 0x3c), 5);
        assert_eq!(e.mem.u32(data + 0x34), buffer_data);
    }

    #[test]
    fn create_chip_with_one_stream_uses_the_maps() {
        let mut e = engine();
        let this = group(&mut e);
        let device = object(&mut e);
        e.mem.set_u32(this + 8, device);
        // GetAt finds nothing; SetAt records; the lock writes offset 0x30 and word 0x77.
        e.register(MAP_GET_AT_00A53C30, |_, _| 0u32.into_ret());
        e.register(MAP_SET_AT_00B57EA0, |_, _| Ret::default());
        e.register(LOCK_00E987C0, |e, a| {
            e.mem.set_u32(a[2], 0x30);
            e.mem.set_u32(a[3], 0x77);
            0xb000u32.into_ret()
        });
        e.register(NEW_CHIP_00537D60, |e, _| e.mem.alloc(0x18).into_ret());
        let strides = e.mem.alloc(4);
        e.mem.set_u32(strides, 0x10);
        let data = e.mem.alloc(0x3c);
        e.mem.set_u32(data + 0x20, 1);
        e.mem.set_u32(data + 0x24, strides);
        e.mem.set_u32(data + 0x1c, 3);
        e.mem.set_u8(data + 0x14, 0x5a);
        e.call_log = Some(vec![]);
        let chip = e.call(0x00e9_5550, &args![this, data, 0u32]).u32();
        let key = 0x8000_0010;
        let sets = calls(&e, MAP_SET_AT_00B57EA0);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][..2].to_vec(), vec![this + 0xc, key]);
        let record = sets[0][2];
        assert_eq!(sets[1], vec![this + 0x1c, key, chip]);
        assert_eq!(e.mem.u32(record + 0xc), device);
        let lock = &calls(&e, LOCK_00E987C0)[0];
        assert_eq!(
            (lock[0], lock[1], lock[4], lock[5]),
            (record, 0x30, 0, 0x5a)
        );
        assert_eq!(
            calls(&e, NEW_CHIP_00537D60),
            vec![vec![0, 0xb000, 0x30, 0x30, 0]]
        );
        assert_eq!(e.mem.u32(chip + 4), record);
        assert_eq!(e.mem.u32(chip + 0x10), 0x77);
        assert_eq!(e.mem.u32(data + 0x38), 3);
    }

    #[test]
    fn create_chip_with_streams_uses_the_cursor() {
        let mut e = engine();
        let this = group(&mut e);
        e.register(ARRAY_SET_SIZE_00A90180, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let data = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(data + 4 * a[1], value);
            Ret::default()
        });
        e.register(LOCK_00E987C0, |e, a| {
            e.mem.set_u32(a[2], 0x40);
            e.mem.set_u32(a[3], 0x99);
            0xc000u32.into_ret()
        });
        let existing = e.mem.alloc(0x18);
        let chips = e.mem.u32(this + 0x40);
        e.mem.set_u32(chips + 4, existing);
        let strides = e.mem.alloc(0xc);
        e.mem.set_u32(strides + 4, 8);
        let data = e.mem.alloc(0x3c);
        e.mem.set_u32(data + 0x20, 3);
        e.mem.set_u32(data + 0x24, strides);
        e.mem.set_u32(data + 0x1c, 2);
        e.mem.set_u32(data + 0x38, 5);
        e.mem.set_u32(this + 0x4c, 1);
        e.call_log = Some(vec![]);
        let chip = e.call(0x00e9_5550, &args![this, data, 1u32]).u32();
        assert_eq!(chip, existing);
        // The arrays grow to 3; the cursor 1 stays; a new record goes into slot 1.
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x2c, 3], vec![this + 0x3c, 3]]
        );
        let records = e.mem.u32(this + 0x30);
        let record = e.mem.u32(records + 4);
        assert_ne!(record, 0);
        assert_eq!(e.mem.u32(record + 8), 0x800);
        assert_eq!(e.mem.u32(chip + 4), record);
        assert_eq!(e.mem.u32(chip + 8), 0xc000);
        assert_eq!(e.mem.u32(chip + 0xc), 0x40);
        assert_eq!(e.mem.u32(chip + 0x14), 0x10);
        assert_eq!(e.mem.u32(chip + 0x10), 0x99);
        assert_eq!(e.mem.u32(data + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 2);
        assert_eq!(calls(&e, LOCK_00E987C0)[0][0], record);
        assert_eq!(calls(&e, LOCK_00E987C0)[0][4], 1);
    }

    #[test]
    fn destructor_empties_everything() {
        let mut e = engine();
        let this = group(&mut e);
        e.mem.set_u32(GROUP_0126FA38, this);
        let chip = e.mem.alloc(0x20);
        let chip_node = e.mem.alloc(0xc);
        e.mem.set_u32(chip_node + 4, 0);
        e.mem.set_u32(chip_node + 8, chip);
        let buckets = e.mem.u32(this + 0x24);
        e.mem.set_u32(buckets, chip_node);
        let chips = e.mem.u32(this + 0x40);
        let array_chip = e.mem.alloc(0x20);
        e.mem.set_u32(array_chip + 8, 1);
        e.mem.set_u32(chips, array_chip);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(chips + 4, other);
        e.mem.set_u16(this + 0x48, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_5b40, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, MAP_REMOVE_00E91FD0), vec![vec![this + 0x1c, 0]]);
        let frees = calls(&e, NI_FREE_00AA1460);
        assert_eq!(
            frees,
            vec![
                vec![chip, 0x20],
                vec![array_chip, 0x20],
                vec![other, 0x20],
                vec![this, 0x50]
            ]
        );
        assert_eq!(e.mem.u32(array_chip + 8), 0);
        assert_eq!(e.mem.u16(this + 0x48), 0);
        assert_eq!(e.mem.u16(this + 0x46), 1);
        assert_eq!(e.mem.u32(GROUP_0126FA38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), CHIP_ARRAY_BASE_VTABLE_010F0F7C);
        assert_eq!(e.mem.u32(this + 0x2c), RECORD_ARRAY_BASE_VTABLE_010F1034);
        assert_eq!(e.mem.u32(this + 0xc), BUFFER_MAP_BASE_VTABLE_010F103C);
        assert_eq!(e.mem.u32(this + 0x1c), CHIP_MAP_BASE_VTABLE_010F105C);
        assert_eq!(calls(&e, MAP_CLEAN_UP_00B601A0).len(), 4);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 4);
        assert_eq!(calls(&e, BASE_DESTRUCT_00E98710), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n009_tests {
    use super::*;

    const DELETE: u32 = 0x0e63_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for no_op in [
            DELETE,
            MEMSET,
            GROUP_REMOVE_OBJECT_00E98750,
            GROUP_ADD_OBJECT_00E98730,
            MAP_CLEAN_UP_00B601A0,
            FREE_00AA10F0,
            MEMORY_MANAGER_FREE_00AA4060,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn remove_at_takes_the_element_out() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(12);
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(data, 0x111);
        e.mem.set_u32(data + 8, 0x333);
        e.mem.set_u16(array + 0xa, 3);
        e.mem.set_u16(array + 0xc, 2);
        // A middle slot: the size stays.
        assert_eq!(e.call(0x00a9_3c50, &[array, 0]).u32(), 0x111);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u16(array + 0xa), 3);
        // An empty slot: no count change.
        assert_eq!(e.call(0x00a9_3c50, &[array, 1]).u32(), 0);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        // The last slot shrinks the size by one.
        assert_eq!(e.call(0x00a9_3c50, &[array, 2]).u32(), 0x333);
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        // Past the size: 0.
        assert_eq!(e.call(0x00a9_3c50, &[array, 2]).u32(), 0);
    }

    /// A buffer data with a deleting slot 0 and two chips.
    fn buffer_data(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, DELETE);
        let data = e.mem.alloc(0x54);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 0x20, 2);
        e.mem.set_u32(data + 0x28, 0x7000);
        data
    }

    fn removes_at(address: u32, slot: u32) {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + slot, data);
        e.call(address, &[0x9000, object]);
        assert_eq!(
            calls(&e, GROUP_REMOVE_OBJECT_00E98750),
            vec![vec![0x9000, data]]
        );
        assert_eq!(calls(&e, MEMSET), vec![vec![0x7000, 0, 8]]);
        assert_eq!(calls(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(object + slot), 0);
        // Without chips nothing is zeroed; without data nothing happens.
        e.mem.set_u32(data + 0x28, 0);
        e.mem.set_u32(object + slot, data);
        e.call(address, &[0x9000, object]);
        assert_eq!(calls(&e, MEMSET).len(), 1);
        e.call(address, &[0x9000, object]);
        assert_eq!(calls(&e, DELETE).len(), 2);
    }

    #[test]
    fn remove_object_ov2_deletes_the_data_at_28() {
        removes_at(0x00e9_5140, 0x28);
    }

    #[test]
    fn remove_object_ov3_deletes_the_data_at_1c() {
        removes_at(0x00e9_5190, 0x1c);
    }

    fn deletes_map(address: u32, tables: &[u32]) {
        let mut e = engine();
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x7700);
        let log_tables = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let tables_seen = log_tables.clone();
        e.register_double(MAP_CLEAN_UP_00B601A0, move |e, a| {
            tables_seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        assert_eq!(e.call(address, &[map, 1]).u32(), map);
        assert_eq!(*log_tables.borrow(), tables);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x7700]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE_00AA4060),
            vec![vec![MEMORY_MANAGER_OBJECT, map]]
        );
        e.call(address, &[map, 0]);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE_00AA4060).len(), 1);
    }

    #[test]
    fn set_map_base_deleting_destructor() {
        deletes_map(0x00e9_52c0, &[BUFFER_MAP_BASE_VTABLE_010F103C]);
    }

    #[test]
    fn set_map_deleting_destructor() {
        deletes_map(
            0x00e9_5300,
            &[BUFFER_MAP_VTABLE_010F107C, BUFFER_MAP_BASE_VTABLE_010F103C],
        );
    }

    #[test]
    fn chip_map_base_deleting_destructor() {
        deletes_map(0x00e9_5340, &[CHIP_MAP_BASE_VTABLE_010F105C]);
    }

    #[test]
    fn chip_map_deleting_destructor() {
        deletes_map(
            0x00e9_5380,
            &[CHIP_MAP_VTABLE_010F109C, CHIP_MAP_BASE_VTABLE_010F105C],
        );
    }

    #[test]
    fn add_object_ov2_makes_and_stores_a_data() {
        let mut e = engine();
        let object = e.mem.alloc(0x20);
        e.call(0x00e9_5500, &[0x9000, object]);
        let data = e.mem.u32(object + 0x1c);
        assert_eq!(e.mem.u32(data), GEOMETRY_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
        assert_eq!(e.mem.u32(data + 4), 0x0140_0000);
        assert_eq!(e.mem.u16(object + 0x18), 0xffff);
        assert_eq!(
            calls(&e, GROUP_ADD_OBJECT_00E98730),
            vec![vec![0x9000, data]]
        );
        e.mem.set_u16(object + 0x18, 0);
        e.call(0x00e9_5500, &[0x9000, object]);
        assert_eq!(calls(&e, GROUP_ADD_OBJECT_00E98730).len(), 1);
        assert_eq!(e.mem.u16(object + 0x18), 0);
    }

    #[test]
    fn set_array_deleting_destructor_frees_its_elements() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x6600);
        assert_eq!(e.call(0x00e9_5780, &[array, 1]).u32(), array);
        assert_eq!(e.mem.u32(array), RECORD_ARRAY_BASE_VTABLE_010F1034);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x6600]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE_00AA4060),
            vec![vec![MEMORY_MANAGER_OBJECT, array]]
        );
        e.call(0x00e9_5780, &[array, 0]);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE_00AA4060).len(), 1);
    }
}
