//! `Havok SDK/hkpsavecontactpointsutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element_size)`).
const RESERVE_ARRAY: u32 = 0x00c9_08c0;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element_size)`).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// `hkReferencedObject::addReference` (`00c90510`, thiscall).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The CRT `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// `hkpSaveContactPointsUtil::getSerializedAgentType` (`00dcd4d0`, cdecl, one word).
const GET_SERIALIZED_AGENT_TYPE: u32 = 0x00dc_d4d0;
/// `hkpSaveContactPointsUtil::serializeEntry` (`00dcd550`).
const SERIALIZE_ENTRY: u32 = 0x00dc_d550;
/// `hkpSaveContactPointsUtil::serialize1nTrack` (`00dcdf70`, cdecl `(track, context, record)`).
const SERIALIZE_1N_TRACK: u32 = 0x00dc_df70;
/// `hkpSaveContactPointsUtil::deserialize1nTrack` (`00dcd950`, cdecl `(source, context, track)`).
const DESERIALIZE_1N_TRACK: u32 = 0x00dc_d950;
/// `hkpSaveContactPointsUtil::serializeCollisionEntry` (`00dcd720`).
const SERIALIZE_COLLISION_ENTRY: u32 = 0x00dc_d720;
/// The constructor of a saved collision record (`00dce930`, `ECX` is `this`, answers `this`).
const CONSTRUCT_RECORD: u32 = 0x00dc_e930;
/// What the lists do with a freshly serialized record (`00dcec70`, thiscall with one word).
const STORE_RECORD: u32 = 0x00dc_ec70;
/// The destructor of a 1n-track record (`00e05c20`, thiscall).
const DESTRUCT_TRACK_RECORD: u32 = 0x00e0_5c20;
/// `hkConvexListAgent3::getAgent1nTrack_ov2` (`00d32ad0`, cdecl `(entry, payload)`).
const GET_AGENT_1N_TRACK_SERIALIZE: u32 = 0x00d3_2ad0;
/// `hkConvexListAgent3::getAgent1nTrack` (`00d32ac0`, cdecl `(entry, payload)`).
const GET_AGENT_1N_TRACK: u32 = 0x00d3_2ac0;
/// `hkpSaveContactPointsEndianUtil::swapEndianTypeInCollisionEntry` (`00e062a0`).
const SWAP_ENDIAN_COLLISION_ENTRY: u32 = 0x00e0_62a0;
/// `hkpSaveContactPointsUtil::deserializeEntry` (`00dcde10`).
const DESERIALIZE_ENTRY: u32 = 0x00dc_de10;
/// `hkpSaveContactPointsUtil::destroyOldEntry` (`00dcd6a0`).
const DESTROY_OLD_ENTRY: u32 = 0x00dc_d6a0;
/// `hkpSaveContactPointsUtil::deserializeCollisionEntry` (`00dce180`).
const DESERIALIZE_COLLISION_ENTRY: u32 = 0x00dc_e180;
/// The vtable of the entity selector base (`scalar deleting destructor` is `00dcd4b0`).
const ENTITY_SELECTOR_VTABLE: u32 = 0x010d_77b4;
/// The vtable of the stack selector `loadContactPoints` hands to the internal loader.
const LOAD_SELECTOR_VTABLE: u32 = 0x010d_77c0;
/// The vtable of a saved collision record.
const RECORD_VTABLE: u32 = 0x010d_77d8;

/// The memory router's allocator (the field at `+0x10` of what `00c85750` answers).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The growth the exe has inlined in front of every array fill: when the capacity (the low 30 bits
/// of the word at `array + 8`) is below `wanted`, the array is reserved to the larger of `wanted`
/// and twice the capacity.
fn grow_array(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let allocator = allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            RESERVE_ARRAY,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// Allocates through slot 4 of the allocator (`0x4` is its first function after the destructor
/// slots: thiscall, one word, the byte count).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let allocator = allocator(e);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// Frees a block through slot 8 of the allocator (thiscall `(block, size)`).
fn free_block(e: &mut Engine, block: u32, size: u32) {
    let allocator = allocator(e);
    e.vcall(allocator, 8, &args![block, size]);
}

/// The body a collidable pointer stands for: when the byte at `+0x18` is 1 the pointer plus the
/// signed byte at `+0x10`, otherwise null.
fn adjusted_pointer(e: &Engine, pointer: u32) -> u32 {
    if e.mem.u8(pointer + 0x18) == 1 {
        pointer.wrapping_add(e.mem.i8(pointer + 0x10) as u32)
    } else {
        0
    }
}

/// Zeroes an empty array header the way the exe's inlined initialiser does: data 0, size 0 and
/// the capacity word `0x80000000` (not owned).
fn init_empty_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00dcd4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::EntitySelector::scalar deleting destructor` (Xbox PDB), thiscall
/// with one argument (`RET 4`): `ECX` is `this`. Sets the vtable and, when bit 0 of the flag byte
/// is set, frees the block. Returns `this`.
pub fn entity_selector_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), ENTITY_SELECTOR_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00dcd4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::getSerializedAgentType` (Xbox PDB), cdecl with one word: the
/// number of the agent's process function. `0x00d3fe90` is 1, `0x00d39390` 2, `0x00d41a30` 3,
/// `0x00d40d80` 4, `0x00d32cd0` 5, `0x00da6eb0` 6, `0x00d2d890` 7, `0x00d35770` 8,
/// `0x00d365d0` 9, and everything else 0.
pub fn hkp_save_contact_points_util_get_serialized_agent_type(
    _e: &mut Engine,
    process: u32,
) -> u32 {
    match process {
        0x00d3_fe90 => 1,
        0x00d3_9390 => 2,
        0x00d4_1a30 => 3,
        0x00d4_0d80 => 4,
        0x00d3_2cd0 => 5,
        0x00da_6eb0 => 6,
        0x00d2_d890 => 7,
        0x00d3_5770 => 8,
        0x00d3_65d0 => 9,
        _ => 0,
    }
}

// Translated from 00dcd550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::serializeEntry` (Xbox PDB), cdecl with seven words
/// `(entry, larger_layout, context, size, kind, track, record)`: writes 0 to `*track` and
/// dispatches on the entry's type byte.
///
/// * Types 0 and 1: `*size = 0x10`, answers 0.
/// * Types 6 and 14: answers 1 at once. Types 7 to 9 and everything above 14: answers 0.
/// * Types 2, 3, 10, 11: the payload is at `entry + 0x10` (`+0x20` when the low byte of
///   `larger_layout` is nonzero). Types 4, 5, 12, 13: `entry + 0x20` (`+0x30`).
///
/// For those the agent type is `getSerializedAgentType` of the word at
/// `[context] + 0x16dc + 0x50 * byte [entry + 1]`, stored at `*kind`. Types 1 to 4 store
/// `*size = byte [entry + 3]` and answer 0. Type 5 does the same, and when bit 5 of
/// `byte [payload + 0xB]` is clear also takes the 1n-track from `getAgent1nTrack_ov2(entry,
/// payload)`, stores it at `*track` and answers `serialize1nTrack(track, context, record)`.
/// Types 6 to 9 store the size, `*track = payload` and answer `serialize1nTrack(payload,
/// context, record)`. Other agent types answer 1.
pub fn hkp_save_contact_points_util_serialize_entry(
    e: &mut Engine,
    entry: Ptr,
    larger_layout: u8,
    context: Ptr,
    size: Ptr,
    kind: Ptr,
    track: Ptr,
    record: Ptr,
) -> u32 {
    let entry = entry.addr();
    e.mem.set_u32(track.addr(), 0);
    let extra = if larger_layout != 0 { 0x10 } else { 0 };
    let payload = match e.mem.u8(entry) {
        0 | 1 => {
            e.mem.set_u32(size.addr(), 0x10);
            return 0;
        }
        2 | 3 | 10 | 11 => entry.wrapping_add(0x10 + extra),
        4 | 5 | 12 | 13 => entry.wrapping_add(0x20 + extra),
        6 | 14 => return 1,
        _ => return 0,
    };
    let agent_at = e
        .mem
        .u32(context.addr())
        .wrapping_add(u32::from(e.mem.u8(entry + 1)) * 0x50)
        .wrapping_add(0x16dc);
    let agent_process = e.mem.u32(agent_at);
    let agent = e
        .call(GET_SERIALIZED_AGENT_TYPE, &args![agent_process])
        .u32();
    e.mem.set_u32(kind.addr(), agent);
    match agent.wrapping_sub(1) {
        0..=3 => {
            e.mem.set_u32(kind.addr(), agent);
            e.mem.set_u32(size.addr(), u32::from(e.mem.u8(entry + 3)));
            0
        }
        4 => {
            e.mem.set_u32(kind.addr(), agent);
            e.mem.set_u32(size.addr(), u32::from(e.mem.u8(entry + 3)));
            if e.mem.u8(payload + 0xb) & 0x20 != 0 {
                return 0;
            }
            let found = e
                .call(GET_AGENT_1N_TRACK_SERIALIZE, &args![entry, payload])
                .u32();
            e.mem.set_u32(track.addr(), found);
            e.call(SERIALIZE_1N_TRACK, &args![found, context, record])
                .u32()
        }
        5..=8 => {
            e.mem.set_u32(size.addr(), u32::from(e.mem.u8(entry + 3)));
            e.mem.set_u32(kind.addr(), agent);
            e.mem.set_u32(track.addr(), payload);
            e.call(SERIALIZE_1N_TRACK, &args![payload, context, record])
                .u32()
        }
        _ => 1,
    }
}

// Translated from 00dcd6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::destroyOldEntry` (Xbox PDB), cdecl with five words
/// `(wanted_kind, context, first, second, entry)`: for an entry of type 2 (payload at
/// `+0x20`) or 4 (`+0x30`), looks up its agent type like `serializeEntry` does and, when it is
/// the nonzero `wanted_kind`, calls the function stored at `+0x16b4` of the agent's record with
/// `(entry, payload, first, second, [context])` and answers 0. Everything else answers 1.
pub fn hkp_save_contact_points_util_destroy_old_entry(
    e: &mut Engine,
    wanted_kind: u32,
    context: Ptr,
    first: u32,
    second: u32,
    entry: Ptr,
) -> u32 {
    let entry = entry.addr();
    let payload = match e.mem.u8(entry) {
        2 => entry.wrapping_add(0x20),
        4 => entry.wrapping_add(0x30),
        _ => return 1,
    };
    let base = e.mem.u32(context.addr());
    let agent_record = base.wrapping_add(u32::from(e.mem.u8(entry + 1)) * 0x50);
    let agent_process = e.mem.u32(agent_record.wrapping_add(0x16dc));
    let agent = e
        .call(GET_SERIALIZED_AGENT_TYPE, &args![agent_process])
        .u32();
    if agent == wanted_kind && wanted_kind != 0 {
        let callback = e.mem.u32(agent_record.wrapping_add(0x16b4));
        e.call(callback, &args![entry, payload, first, second, base]);
        return 0;
    }
    1
}

// Translated from 00dcd720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::serializeCollisionEntry` (Xbox PDB), cdecl with four words
/// `(converter, entry, context, record)`. The entry's object (the word at `+8`) must answer 0
/// from its virtual function at `0x34` and its header (`[object + 0x3c]`) must have a nonzero
/// count (the word at `+4`); otherwise 1 is answered.
///
/// `serializeEntry(entry, 1, context, &size, &kind, &track, record + 0x114)` fills the record:
/// a failure answers 1, otherwise the kind byte goes to `record + 0x19`, `size` bytes of the
/// entry are copied to `record + 0x74` (when `size` is not 0; `serializeEntry` leaves it at
/// `0xffffffff` for entry types it does not handle, which the exe then tries to copy), the
/// `[object + 0x24]` bytes at `[object + 0x20]` become the array at `record + 0x68`, the
/// header's byte count times its word count bytes the array at `record + 0x50`, its
/// `count * 32` bytes the array at `record + 0x5c`, and its first twelve words `record + 0x20`.
/// The two bodies at `[entry + 0x10]` and `[entry + 0x14]` (adjusted by their signed byte at
/// `+0x10` when the byte at `+0x18` is 1, else null) are then either mapped through the function
/// at `converter + 4` into `record + 0x10` and `+0x14` (when `byte [converter]` is nonzero, which
/// is also stored at `record + 0x18`) or stored at `record + 8` and `+0xC` with a reference added
/// to each. Answers 0.
pub fn hkp_save_contact_points_util_serialize_collision_entry(
    e: &mut Engine,
    converter: Ptr,
    entry: Ptr,
    context: Ptr,
    record: Ptr,
) -> u32 {
    let converter = converter.addr();
    let entry = entry.addr();
    let record = record.addr();
    let object = e.mem.u32(entry + 8);
    if e.vcall(object, 0x34, &args![]).u32() != 0 {
        return 1;
    }
    let header = e.mem.u32(e.mem.u32(entry + 8) + 0x3c);
    if e.mem.u16(header + 4) == 0 {
        return 1;
    }
    // The exe's frame (`SUB ESP,0xc`, two pushes): 0x14 bytes, the kind at +0x8, the size at
    // +0xc and at +0x10 the byte 1 that is both the flag argument and the track out word
    // (00dcd757..00dcd77e).
    let (failed, kind, size) = e.with_stack(0x14, |e, frame| {
        let scratch = frame.addr() + 8;
        let (kind_at, size_at, track_at) = (scratch, scratch + 4, scratch + 8);
        e.mem.set_u8(track_at, 1);
        e.mem.set_u32(size_at, u32::MAX);
        e.mem.set_u32(kind_at, 0);
        let failed = e
            .call(
                SERIALIZE_ENTRY,
                &args![
                    entry,
                    1u32,
                    context,
                    size_at,
                    kind_at,
                    track_at,
                    record + 0x114
                ],
            )
            .u32();
        (failed, e.mem.u8(kind_at), e.mem.u32(size_at))
    });
    if failed != 0 {
        return 1;
    }
    e.mem.set_u8(record + 0x19, kind);
    if size != 0 {
        e.call(MEM_COPY, &args![record + 0x74, entry, size]);
    }
    let object = e.mem.u32(entry + 8);
    let count = e.mem.i32(object + 0x24);
    grow_array(e, record + 0x68, count, 1);
    e.mem.set_u32(record + 0x6c, count as u32);
    if e.mem.i32(record + 0x6c) > 0 {
        let mut index: u32 = 0;
        loop {
            let source = e.mem.u32(object + 0x20);
            let byte = e.mem.u8(source.wrapping_add(index));
            let destination = e.mem.u32(record + 0x68);
            e.mem.set_u8(destination.wrapping_add(index), byte);
            index = index.wrapping_add(1);
            if index as i32 >= e.mem.i32(record + 0x6c) {
                break;
            }
        }
    }
    let header = e.mem.u32(object + 0x3c);
    let contact_bytes =
        (u32::from(e.mem.u8(header + 0xa)) * u32::from(e.mem.u16(header + 4))) as i32;
    grow_array(e, record + 0x50, contact_bytes, 1);
    e.mem.set_u32(record + 0x54, contact_bytes as u32);
    let contact_bytes = u32::from(e.mem.u8(header + 0xa)) * u32::from(e.mem.u16(header + 4));
    let source = header
        .wrapping_add(u32::from(e.mem.u16(header + 6)) << 5)
        .wrapping_add(0x30);
    let destination = e.mem.u32(record + 0x50);
    e.call(MEM_COPY, &args![destination, source, contact_bytes]);
    let points = i32::from(e.mem.u16(header + 4));
    grow_array(e, record + 0x5c, points, 0x20);
    e.mem.set_u32(record + 0x60, points as u32);
    let destination = e.mem.u32(record + 0x5c);
    let bytes = u32::from(e.mem.u16(header + 4)) << 5;
    e.call(MEM_COPY, &args![destination, header + 0x30, bytes]);
    let header = e.mem.u32(object + 0x3c);
    for word in 0..12 {
        let value = e.mem.u32(header + 4 * word);
        e.mem.set_u32(record + 0x20 + 4 * word, value);
    }
    let first = adjusted_pointer(e, e.mem.u32(entry + 0x10));
    let second = adjusted_pointer(e, e.mem.u32(entry + 0x14));
    let convert = e.mem.u8(converter);
    e.mem.set_u8(record + 0x18, convert);
    if convert != 0 {
        let function = e.mem.u32(converter + 4);
        let mapped = e.call(function, &args![first]).u32();
        e.mem.set_u32(record + 0x10, mapped);
        let function = e.mem.u32(converter + 4);
        let mapped = e.call(function, &args![second]).u32();
        e.mem.set_u32(record + 0x14, mapped);
        return 0;
    }
    e.mem.set_u32(record + 8, first);
    e.mem.set_u32(record + 0xc, second);
    e.call(ADD_REFERENCE, &args![first]);
    let second = e.mem.u32(record + 0xc);
    e.call(ADD_REFERENCE, &args![second]);
    0
}

// Translated from 00dcd950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::deserialize1nTrack` (Xbox PDB), cdecl with three words
/// `(source, context, track)`. The source track keeps `+0` an array of 0x200-byte items, `+4`
/// their count, `+0xC` an array of block pointers and `+0x10` their count. The destination
/// track's item array is grown to the count (`element size 4`) and set to that size, a fresh
/// zeroed-first-word 0x200-byte block is allocated for every item and filled with the item's
/// 0x200 bytes (a failed allocation is a write to address 0, which panics here). Then each block
/// of the source has the nested track at `[destination item[block + 0x18]] + 0x10 +
/// [block + 0x1c]` initialised (data 0, size 0, capacity `0x80000000`) and is deserialized into
/// it by a recursive call; a recursive answer of exactly 1 is answered at once. Answers 0
/// otherwise.
pub fn hkp_save_contact_points_util_deserialize_1n_track(
    e: &mut Engine,
    source: Ptr,
    context: u32,
    track: Ptr,
) -> u32 {
    let source = source.addr();
    let track = track.addr();
    let count = e.mem.i32(source + 4);
    grow_array(e, track, count, 4);
    e.mem.set_u32(track + 4, count as u32);
    if e.mem.i32(source + 4) > 0 {
        let mut index: u32 = 0;
        loop {
            let block = allocate(e, 0x200);
            if block != 0 {
                e.mem.set_u32(block, 0);
            }
            let destination_list = e.mem.u32(track);
            e.mem
                .set_u32(destination_list.wrapping_add(index.wrapping_mul(4)), block);
            let from = e
                .mem
                .u32(e.mem.u32(source).wrapping_add(index.wrapping_mul(4)));
            let to = e
                .mem
                .u32(e.mem.u32(track).wrapping_add(index.wrapping_mul(4)));
            assert!(to != 0, "00dcd950: the allocation of an item block failed");
            for word in 0..0x80 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(to + 4 * word, value);
            }
            index += 1;
            if index as i32 >= e.mem.i32(source + 4) {
                break;
            }
        }
    }
    if e.mem.i32(source + 0x10) > 0 {
        let mut index: u32 = 0;
        loop {
            let child = e.mem.u32(e.mem.u32(source + 0xc).wrapping_add(index * 4));
            let items = e.mem.u32(track);
            let item = e
                .mem
                .u32(items.wrapping_add(e.mem.u32(child + 0x18).wrapping_mul(4)));
            let nested = item
                .wrapping_add(e.mem.u32(child + 0x1c))
                .wrapping_add(0x10);
            if nested != 0 {
                init_empty_array(e, nested);
            }
            let result = e
                .call(DESERIALIZE_1N_TRACK, &args![child, context, nested])
                .u32();
            if result == 1 {
                return 1;
            }
            index += 1;
            if index as i32 >= e.mem.i32(source + 0x10) {
                break;
            }
        }
    }
    0
}

/// The sort over an array (`00cd1e20`, thiscall `(array, allocator, count, first, second)`).
const SORT_ARRAY: u32 = 0x00cd_1e20;

// Translated from 00dcda40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, three words `(converter, group, container)`): saves the contact points of a
/// group. The group keeps `+0x28` an array of items, `+0x2C` their count, `+0x34` and `+0x38` the
/// two words `00cd1e20` sorts them with, and `+0x78` the context; every item keeps `+0x5C` an
/// array of block pointers, `+0x60` their count and `+0x58` the size of the last block.
///
/// A local array of the item pointers is taken from the allocator (slot `0xC`, which grants a
/// byte count; the array is sized for what it granted, and nothing is taken for an empty
/// group), copied from the group's array and sorted with `00cd1e20`. For each item and each of
/// its blocks (a block is `0x200` bytes, the last one the item's `+0x58` bytes) every entry
/// (its size is the byte at `+3`) is serialized into a new `0x140`-byte record (`00dce930`, then
/// `serializeCollisionEntry(converter, entry, context, record)`); a record that serialized
/// (answer 0) is handed to `00dcec70` on `container`; the record loses its reference either way.
/// The local array is freed through slot `0x10` unless it was never taken.
pub fn fn_00dcda40(e: &mut Engine, converter: Ptr, group: Ptr, container: Ptr) {
    // The exe's frame (`SUB ESP,0x10`, four pushes) taken with the arguments: the sorted array
    // at +0x14 (00dcda4d, 00dcdaf3), the return address at +0x20; the byte count goes to the
    // allocator through the `group` argument's own slot, +0x28 (00dcda9c).
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x24, converter.addr());
        e.mem.set_u32(frame + 0x28, group.addr());
        e.mem.set_u32(frame + 0x2c, container.addr());
        fn_00dcda40_in_frame(e, frame, converter, group, container)
    });
}

/// [`fn_00dcda40`] in its frame.
fn fn_00dcda40_in_frame(e: &mut Engine, frame: u32, converter: Ptr, group: Ptr, container: Ptr) {
    let converter = converter.addr();
    let group = group.addr();
    let container = container.addr();
    let mut data: u32 = 0;
    let mut capacity: u32 = 0x8000_0000;
    let allocator_object = allocator(e);
    if 0 < e.mem.i32(group + 0x2c) {
        // The code that would free an earlier array is unreachable: the capacity word starts
        // with its "not owned" bit set.
        let wanted = e.mem.i32(group + 0x2c).wrapping_mul(4);
        let bytes = frame + 0x28;
        e.mem.set_i32(bytes, wanted);
        data = e.vcall(allocator_object, 0xc, &args![bytes]).u32();
        let granted = e.mem.i32(bytes);
        capacity = (granted.wrapping_add((granted >> 31) & 3) >> 2) as u32;
    }
    let count = e.mem.i32(group + 0x2c);
    let source = e.mem.u32(group + 0x28);
    if count > 0 {
        for index in 0..count as u32 {
            let item = e.mem.u32(source.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(data.wrapping_add(index.wrapping_mul(4)), item);
        }
    }
    let sort_second = e.mem.u32(group + 0x38);
    let sort_first = e.mem.u32(group + 0x34);
    let allocator_object = allocator(e);
    let array = frame + 0x14;
    e.mem.set_u32(array, data);
    e.mem.set_u32(array + 4, count as u32);
    e.mem.set_u32(array + 8, capacity);
    e.call(
        SORT_ARRAY,
        &args![array, allocator_object, count, sort_first, sort_second],
    );
    if count > 0 {
        let mut item_index: u32 = 0;
        loop {
            let mut item = e.mem.u32(data.wrapping_add(item_index.wrapping_mul(4)));
            let mut block_index: i32 = 0;
            if e.mem.i32(item + 0x60) > 0 {
                loop {
                    let current = e.mem.u32(data.wrapping_add(item_index.wrapping_mul(4)));
                    let blocks = e.mem.u32(current + 0x5c);
                    let block = e
                        .mem
                        .u32(blocks.wrapping_add((block_index as u32).wrapping_mul(4)));
                    block_index += 1;
                    let end = if block_index == e.mem.i32(current + 0x60) {
                        e.mem.u32(item + 0x58).wrapping_add(block)
                    } else {
                        block.wrapping_add(0x200)
                    };
                    let mut entry = block;
                    while entry < end {
                        let fresh = allocate(e, 0x140);
                        e.mem.set_u16(fresh + 4, 0x140);
                        let record = e.call(CONSTRUCT_RECORD, &args![fresh]).u32();
                        let context = e.mem.u32(group + 0x78);
                        let failed = e
                            .call(
                                SERIALIZE_COLLISION_ENTRY,
                                &args![converter, entry, context, record],
                            )
                            .u32();
                        if failed == 0 {
                            e.call(STORE_RECORD, &args![container, record]);
                        }
                        e.call(REMOVE_REFERENCE, &args![record]);
                        entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
                    }
                    item = e.mem.u32(data.wrapping_add(item_index.wrapping_mul(4)));
                    if block_index >= e.mem.i32(item + 0x60) {
                        break;
                    }
                }
            }
            item_index += 1;
            if item_index as i32 >= count {
                break;
            }
        }
    }
    let allocator_object = allocator(e);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator_object, 0x10, &args![data, bytes]);
    }
}

/// `hkMapBase<unsigned long, unsigned long>::reserve` (`00cb1fe0`, thiscall `(allocator, count)`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkMapBase::insert` (`00cb1870`, thiscall `(allocator, key, value)`).
const MAP_INSERT: u32 = 0x00cb_1870;
/// `hkMapBase::findKey` (`00cb18f0`, thiscall `(key)`, answers an iterator index).
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// `hkMapBase::clearAndDeallocate` (`00cb1a90`, thiscall `(allocator)`).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// `hkMapBase::getWithDefault` (`00cb1930`, thiscall `(key, default)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// The destructor of the `hkMap` (`00a29680`, thiscall).
const MAP_DESTRUCT: u32 = 0x00a2_9680;
/// `hkpLinkedCollidable::getCollisionEntriesSorted` (`00d08260`, thiscall `(list)`).
const GET_COLLISION_ENTRIES_SORTED: u32 = 0x00d0_8260;

// Translated from 00dcdc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::saveContactPoints` (Xbox PDB), cdecl with four words
/// `(converter, bodies, count, container)`. The world is `[[bodies[0] + 8]`. An `hkMap` of the
/// bodies seen is kept on the stack (reserved for `count` entries). For each body: the body's
/// collidable (`+0x10`) goes into the map with value 0, and `getCollisionEntriesSorted` fills a
/// local list of `(entry, other collidable)` pairs; each pair whose other collidable is not in
/// the map yet (the map answers an index above its third word) is serialized like in
/// `00dcda40` into a new `0x140`-byte record handed to `00dcec70` on `container` when it
/// serialized, with the reference dropped either way, using the world's context (`+0x78`). The
/// list is freed through slot `0x10` unless it is not owned. The map is cleared and destroyed.
pub fn hkp_save_contact_points_util_save_contact_points(
    e: &mut Engine,
    converter: Ptr,
    bodies: Ptr,
    count: i32,
    container: Ptr,
) {
    let converter = converter.addr();
    let bodies = bodies.addr();
    let container = container.addr();
    let world = e.mem.u32(e.mem.u32(bodies) + 8);
    // The exe's frame (`SUB ESP,0x1c`, four pushes): 0x2c bytes, the entries list at +0x14
    // (00dcdc6a) and the body map at +0x20 (00dcdc16).
    e.with_stack(0x2c, |e, frame| {
        let map = frame.addr() + 0x20;
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, u32::MAX);
        let allocator_object = allocator(e);
        e.call(MAP_RESERVE, &args![map, allocator_object, count]);
        let list = frame.addr() + 0x14;
        if count > 0 {
            let mut body_index: u32 = 0;
            loop {
                let collidable = e.mem.u32(bodies.wrapping_add(body_index.wrapping_mul(4))) + 0x10;
                let allocator_object = allocator(e);
                e.call(MAP_INSERT, &args![map, allocator_object, collidable, 0u32]);
                init_empty_array(e, list);
                let collidable = e.mem.u32(bodies.wrapping_add(body_index.wrapping_mul(4))) + 0x10;
                e.call(GET_COLLISION_ENTRIES_SORTED, &args![collidable, list]);
                let mut pair_index: u32 = 0;
                if e.mem.i32(list + 4) > 0 {
                    loop {
                        let pairs = e.mem.u32(list);
                        let pair = pairs.wrapping_add(pair_index.wrapping_mul(8));
                        let other = e.mem.u32(pair.wrapping_add(4));
                        let found = e.call(MAP_FIND_KEY, &args![map, other]).i32();
                        if found > e.mem.i32(map + 8) {
                            let fresh = allocate(e, 0x140);
                            e.mem.set_u16(fresh + 4, 0x140);
                            let record = e.call(CONSTRUCT_RECORD, &args![fresh]).u32();
                            let context = e.mem.u32(world + 0x78);
                            let entry = e.mem.u32(pair);
                            let failed = e
                                .call(
                                    SERIALIZE_COLLISION_ENTRY,
                                    &args![converter, entry, context, record],
                                )
                                .u32();
                            if failed == 0 {
                                e.call(STORE_RECORD, &args![container, record]);
                            }
                            e.call(REMOVE_REFERENCE, &args![record]);
                        }
                        pair_index += 1;
                        if pair_index as i32 >= e.mem.i32(list + 4) {
                            break;
                        }
                    }
                }
                let allocator_object = allocator(e);
                let capacity = e.mem.u32(list + 8);
                e.mem.set_u32(list + 4, 0);
                if (capacity as i32) >= 0 {
                    let data = e.mem.u32(list);
                    let bytes = (capacity & 0x3fff_ffff).wrapping_mul(8);
                    e.vcall(allocator_object, 0x10, &args![data, bytes]);
                }
                body_index += 1;
                init_empty_array(e, list);
                if body_index as i32 >= count {
                    break;
                }
            }
        }
        let allocator_object = allocator(e);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator_object]);
        e.call(MAP_DESTRUCT, &args![map]);
    });
}

// Translated from 00dcde10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::deserializeEntry` (Xbox PDB), cdecl with five words
/// `(record, kind, track, context, entry)`. For an entry of type 2 (payload at `+0x20`) or 4
/// (`+0x30`) whose agent type (looked up like `serializeEntry` does) equals `kind`: kinds 1 to
/// 4 copy `0x68` bytes from `record + 0x8c` to `entry + 0x18` and the bytes at `record + 0x76`
/// and `+0x77` to `entry + 2` and `+3`, answering 0. Kind 5 does the same and, when bit 5 of
/// `byte [payload + 0xB]` is clear, takes the 1n-track from `getAgent1nTrack(entry, payload)`,
/// initialises it (data 0, size 0, capacity `0x80000000`) when it is not null and answers
/// `deserialize1nTrack(track, context, found)`. Kinds 6 to 9 copy the `0x68` bytes,
/// initialise the payload as an empty array when it is not null and answer
/// `deserialize1nTrack(track, context, payload)`. Everything else answers 1.
pub fn hkp_save_contact_points_util_deserialize_entry(
    e: &mut Engine,
    record: Ptr,
    kind: u32,
    track: Ptr,
    context: Ptr,
    entry: Ptr,
) -> u32 {
    let record = record.addr();
    let entry = entry.addr();
    let payload = match e.mem.u8(entry) {
        2 => entry.wrapping_add(0x20),
        4 => entry.wrapping_add(0x30),
        _ => return 1,
    };
    let agent_at = e
        .mem
        .u32(context.addr())
        .wrapping_add(u32::from(e.mem.u8(entry + 1)) * 0x50)
        .wrapping_add(0x16dc);
    let agent_process = e.mem.u32(agent_at);
    let agent = e
        .call(GET_SERIALIZED_AGENT_TYPE, &args![agent_process])
        .u32();
    if agent != kind || kind.wrapping_sub(1) > 8 {
        return 1;
    }
    match kind {
        1..=5 => {
            e.call(MEM_COPY, &args![entry + 0x18, record + 0x8c, 0x68u32]);
            let first = e.mem.u8(record + 0x76);
            e.mem.set_u8(entry + 2, first);
            let second = e.mem.u8(record + 0x77);
            e.mem.set_u8(entry + 3, second);
            if kind != 5 || e.mem.u8(payload + 0xb) & 0x20 != 0 {
                return 0;
            }
            let found = e.call(GET_AGENT_1N_TRACK, &args![entry, payload]).u32();
            if found != 0 {
                init_empty_array(e, found);
            }
            e.call(DESERIALIZE_1N_TRACK, &args![track, context, found])
                .u32()
        }
        _ => {
            e.call(MEM_COPY, &args![entry + 0x18, record + 0x8c, 0x68u32]);
            if payload != 0 {
                init_empty_array(e, payload);
            }
            e.call(DESERIALIZE_1N_TRACK, &args![track, context, payload])
                .u32()
        }
    }
}

// Translated from 00dcdf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::serialize1nTrack` (Xbox PDB), cdecl with three words
/// `(source, context, track)`. The source keeps `+0` an array of 0x200-byte items and `+4` their
/// count; every item starts with its size in its first word, and `0x10` bytes later its entries
/// follow. The destination's item array (at `+0`/`+4`/`+8`, elements of 4 bytes) is grown to the
/// source's count first (its size is raised one item at a time).
///
/// For every entry of every item a record of `0x20` bytes (words `+0` 0, `+4` 0, `+8`
/// `0x80000000`, `+0xC` 0, `+0x10` 0, `+0x14` `0x80000000`, `+0x18` -1) is allocated and
/// `serializeEntry(entry, 0, context, &size, &kind, &track, record)` runs. A failure destroys
/// and frees the record and answers 1. A record with a nonzero second word gets the item index
/// at `+0x18` and the offset of the track the entry wrote (relative to the item's entries) at
/// `+0x1C`, and is appended to the list at `track + 0xC` (size at `+0x10`, capacity at `+0x14`,
/// grown by one when full); one with a zero second word is destroyed and freed. The cursor moves
/// on by the size the entry wrote. Finally the item array grows by one and gets a copy of the
/// item's `0x200` bytes in a fresh block (first word set to 0 before the copy).
///
/// A null record or block allocation is a write through null in the exe and panics here. An
/// entry that writes a size of 0 keeps the loop at the same entry (the exe loops).
pub fn hkp_save_contact_points_util_serialize_1n_track(
    e: &mut Engine,
    source: Ptr,
    context: Ptr,
    track: Ptr,
) -> u32 {
    let source = source.addr();
    let track = track.addr();
    let count = e.mem.i32(source + 4);
    grow_array(e, track, count, 4);
    if e.mem.i32(source + 4) <= 0 {
        return 0;
    }
    // The exe's frame (`SUB ESP,0x18`, four pushes): 0x28 bytes, the track word at +0x14, the
    // size at +0x18 and the kind at +0x24 (00dce037..00dce049).
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        serialize_1n_track_in_frame(e, frame, source, context, track)
    })
}

/// [`hkp_save_contact_points_util_serialize_1n_track`] in its frame.
fn serialize_1n_track_in_frame(
    e: &mut Engine,
    frame: u32,
    source: u32,
    context: Ptr,
    track: u32,
) -> u32 {
    let (track_at, size_at, kind_at) = (frame + 0x14, frame + 0x18, frame + 0x24);
    let mut item_index: i32 = 0;
    loop {
        let item = e.mem.u32(
            e.mem
                .u32(source)
                .wrapping_add((item_index as u32).wrapping_mul(4)),
        );
        let end = e.mem.u32(item).wrapping_add(item).wrapping_add(0x10);
        let mut cursor = item.wrapping_add(0x10);
        if cursor < end {
            loop {
                e.mem.set_u32(track_at, 0);
                e.mem.set_u32(size_at, 0);
                let record = allocate(e, 0x20);
                if record != 0 {
                    e.mem.set_u32(record, 0);
                    e.mem.set_u32(record + 4, 0);
                    e.mem.set_u32(record + 8, 0x8000_0000);
                    e.mem.set_u32(record + 0xc, 0);
                    e.mem.set_u32(record + 0x10, 0);
                    e.mem.set_u32(record + 0x14, 0x8000_0000);
                    e.mem.set_u32(record + 0x18, u32::MAX);
                }
                let failed = e
                    .call(
                        SERIALIZE_ENTRY,
                        &args![cursor, 0u32, context, size_at, kind_at, track_at, record],
                    )
                    .u32();
                if failed != 0 {
                    if record != 0 {
                        e.call(DESTRUCT_TRACK_RECORD, &args![record]);
                        free_block(e, record, 0x20);
                    }
                    return 1;
                }
                assert!(record != 0, "00dcdf70: the allocation of a record failed");
                if e.mem.u32(record + 4) != 0 {
                    let written = e.mem.u32(track_at);
                    e.mem.set_u32(record + 0x18, item_index as u32);
                    e.mem
                        .set_u32(record + 0x1c, written.wrapping_sub(item).wrapping_sub(0x10));
                    let list = track + 0xc;
                    let allocator_object = allocator(e);
                    if e.mem.u32(list + 4) == e.mem.u32(list + 8) & 0x3fff_ffff {
                        e.call(RESERVE_MORE, &args![allocator_object, list, 4u32]);
                    }
                    let at = e
                        .mem
                        .u32(list)
                        .wrapping_add(e.mem.u32(list + 4).wrapping_mul(4));
                    e.mem.set_u32(at, record);
                    let size = e.mem.u32(list + 4).wrapping_add(1);
                    e.mem.set_u32(list + 4, size);
                } else {
                    e.call(DESTRUCT_TRACK_RECORD, &args![record]);
                    free_block(e, record, 0x20);
                }
                cursor = cursor.wrapping_add(e.mem.u32(size_at));
                if cursor >= end {
                    break;
                }
            }
        }
        let allocator_object = allocator(e);
        if e.mem.u32(track + 4) == e.mem.u32(track + 8) & 0x3fff_ffff {
            e.call(RESERVE_MORE, &args![allocator_object, track, 4u32]);
        }
        let size = e.mem.u32(track + 4).wrapping_add(1);
        e.mem.set_u32(track + 4, size);
        let block = allocate(e, 0x200);
        if block != 0 {
            e.mem.set_u32(block, 0);
        }
        let at = e
            .mem
            .u32(track)
            .wrapping_add((item_index as u32).wrapping_mul(4));
        e.mem.set_u32(at, block);
        let destination = e.mem.u32(
            e.mem
                .u32(track)
                .wrapping_add((item_index as u32).wrapping_mul(4)),
        );
        assert!(
            destination != 0,
            "00dcdf70: the allocation of a block failed"
        );
        for word in 0..0x80 {
            let value = e.mem.u32(item + 4 * word);
            e.mem.set_u32(destination + 4 * word, value);
        }
        item_index += 1;
        if item_index >= e.mem.i32(source + 4) {
            break;
        }
    }
    0
}
/// `hkpWorldAgentUtil::getIslandFromAgentEntry` (`00cc0e20`, cdecl `(agent, island_a, island_b)`).
const GET_ISLAND_FROM_AGENT_ENTRY: u32 = 0x00cc_0e20;
/// `hkpWorldOperationUtil::addConstraintImmediately` (`00cb3ca0`, cdecl `(world, owner, flag)`).
const ADD_CONSTRAINT_IMMEDIATELY: u32 = 0x00cb_3ca0;
/// `atomSizeRoundUp` (`00d92440`, cdecl, one word).
const ATOM_SIZE_ROUND_UP: u32 = 0x00d9_2440;
/// `hkpSimpleContactConstraintAtomUtil::allocateAtom` (`00d92510`, cdecl `(count, a, b, c)`).
const ALLOCATE_ATOM: u32 = 0x00d9_2510;
/// `hkpConstraintOwner::addConstraintInfo` (`00cb74d0`, thiscall `(owner, info)`).
const ADD_CONSTRAINT_INFO: u32 = 0x00cb_74d0;
/// `hkAgentNnMachine_InvalidateTimInAgent` (`00cfdbe0`, cdecl `(agent, context)`).
const INVALIDATE_TIM_IN_AGENT: u32 = 0x00cf_dbe0;
/// `hkAgentNnMachine_FindAgent` (`00cfdd70`, cdecl `(collidable_a, collidable_b)`).
const FIND_AGENT: u32 = 0x00cf_dd70;
/// `hkpWorldCallbackUtil::fireContactPointAdded` (`00d01850`, cdecl `(world, event)`).
const FIRE_CONTACT_POINT_ADDED: u32 = 0x00d0_1850;
/// `hkpEntityCallbackUtil::fireContactPointAddedInternal` (`00d00110`, cdecl `(entity, event)`).
const FIRE_CONTACT_POINT_ADDED_INTERNAL: u32 = 0x00d0_0110;
/// `hkpWorldCallbackUtil::fireContactPointCallback` (`00d00ca0`, cdecl `(world, event)`).
const FIRE_CONTACT_POINT_CALLBACK: u32 = 0x00d0_0ca0;
/// `hkpEntityCallbackUtil::fireContactPointCallbackInternal` (`00cfff00`, cdecl `(entity, event)`).
const FIRE_CONTACT_POINT_CALLBACK_INTERNAL: u32 = 0x00cf_ff00;
/// The `float` stored in the frame of `deserializeCollisionEntry` before each added event.
const ADDED_EVENT_FLOAT: u32 = 0x0104_da48;
/// The size of the frame `00dce180` allocates on the stack (`0x30f4` bytes).
const COLLISION_FRAME: u32 = 0x3100;

// Translated from 00dce180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::deserializeCollisionEntry` (Xbox PDB), cdecl with six words
/// `(options, saved, body_a, body_b, context, agent)`; the function reserves `0x30f4` bytes of
/// stack (`__alloca_probe`, `00ec8660` is only the stack probe) and has no exception frame.
///
/// 1. The island is the one of `body_a` (`[+0xCC]`); when the islands differ, it is the other
///    one if `body_a`'s byte at `+0xE8` is 5, else `body_a`'s when `body_b`'s byte is 5, else
///    `getIslandFromAgentEntry(agent, island_a, island_b)`.
/// 2. `destroyOldEntry(saved kind (signed byte at +0x19), context, [agent + 8], island, agent)`;
///    a nonzero answer answers 1.
/// 3. When `body_a` is not the body of `[agent + 0x10]` (see `serializeCollisionEntry`), the two
///    body pointers of the agent (`+0x10`, `+0x14`), its two words at `+4` and `+6`, the two
///    words at `+0x58` and `+0x5C` of its object and the words at `+4` and `+8` of the object's
///    `+0x74` are swapped, and the byte at `+0x1A` of the latter flipped.
/// 4. `deserializeEntry(saved, kind, saved + 0x114, context, agent)`; nonzero answers 1.
/// 5. The `saved[+0x6C]` bytes at `saved[+0x68]` become the object's array at `+0x20`.
/// 6. With no atoms (`word [[object + 0x3c] + 4] == 0`) the constraint is added to the world of
///    the agent's first body (with the world's counter and flag adjusted around the call).
/// 7. The old atoms (`[object + 0x3c]`) are subtracted from the island's counters (and the
///    owner's) and released (`atomSizeRoundUp(word [atoms + 2])` bytes), new atoms are allocated
///    from the saved words (`+0x24`, bytes `+0x28`, `+0x29`, word `+0x2C`), filled from
///    `saved + 0x30` (eight words) and the contact point bytes/properties of the saved record, the
///    constraint info is added, and (when `byte [options + 1]` is set) the third word of every
///    contact point record is cleared.
/// 8. The ids of the contact points (a local array filled by the object's slot `0x30`) get
///    `contactPointAdded` events when `byte [options + 8]` is set, and `contactPointCallback`
///    events (for the points whose byte at `+0xF` of their record is even) when
///    `byte [options + 9]` is set; the events are built in the same stack frame the exe uses
///    (`0x30f4` bytes, event at `+0x3C`, the unwritten words are zero here and garbage there).
/// 9. `InvalidateTimInAgent(agent, context)`; the local array is freed through slot `0x10`
///    unless not owned. Answers 0.
pub fn hkp_save_contact_points_util_deserialize_collision_entry(
    e: &mut Engine,
    options: Ptr,
    saved: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    context: Ptr,
    agent: Ptr,
) -> u32 {
    let options = options.addr();
    let saved = saved.addr();
    let body_a = body_a.addr();
    let body_b = body_b.addr();
    let context = context.addr();
    let agent = agent.addr();
    let island_b = e.mem.u32(body_b + 0xcc);
    let mut island = e.mem.u32(body_a + 0xcc);
    let object = e.mem.u32(agent + 8);
    if island != island_b {
        if e.mem.u8(body_a + 0xe8) == 5 {
            island = island_b;
        } else if e.mem.u8(body_b + 0xe8) != 5 {
            island = e
                .call(GET_ISLAND_FROM_AGENT_ENTRY, &args![agent, island, island_b])
                .u32();
        }
    }
    let saved_kind = e.mem.i8(saved + 0x19) as u32;
    let failed = e
        .call(
            DESTROY_OLD_ENTRY,
            &args![saved_kind, context, object, island, agent],
        )
        .u32();
    if failed != 0 {
        return 1;
    }
    let first_body = e.mem.u32(agent + 0x10);
    if body_a != adjusted_pointer(e, first_body) {
        let second_body = e.mem.u32(agent + 0x14);
        e.mem.set_u32(agent + 0x10, second_body);
        e.mem.set_u32(agent + 0x14, first_body);
        let low = e.mem.u16(agent + 4);
        let high = e.mem.u16(agent + 6);
        e.mem.set_u16(agent + 4, high);
        e.mem.set_u16(agent + 6, low);
        let first = e.mem.u32(object + 0x58);
        let second = e.mem.u32(object + 0x5c);
        e.mem.set_u32(object + 0x58, second);
        e.mem.set_u32(object + 0x5c, first);
        if e.mem.u32(object + 0x74) != 0 {
            let owner = e.mem.u32(object + 0x74);
            let first = e.mem.u32(owner + 4);
            let second = e.mem.u32(owner + 8);
            e.mem.set_u32(owner + 4, second);
            e.mem.set_u32(owner + 8, first);
            let owner = e.mem.u32(object + 0x74);
            let flipped = u8::from(e.mem.u8(owner + 0x1a) == 0);
            e.mem.set_u8(owner + 0x1a, flipped);
        }
    }
    let saved_kind = e.mem.i8(saved + 0x19) as u32;
    let failed = e
        .call(
            DESERIALIZE_ENTRY,
            &args![saved, saved_kind, saved + 0x114, context, agent],
        )
        .u32();
    if failed != 0 {
        return 1;
    }
    let size = e.mem.i32(saved + 0x6c);
    grow_array(e, object + 0x20, size, 1);
    e.mem.set_u32(object + 0x24, size as u32);
    if size > 0 {
        let mut index: u32 = 0;
        loop {
            let byte = e.mem.u8(e.mem.u32(saved + 0x68).wrapping_add(index));
            let destination = e.mem.u32(object + 0x20);
            e.mem.set_u8(destination.wrapping_add(index), byte);
            index += 1;
            if index as i32 >= e.mem.i32(object + 0x24) {
                break;
            }
        }
    }
    let atoms = e.mem.u32(object + 0x3c);
    if e.mem.u16(atoms + 4) == 0 {
        let first_body = e.mem.u32(agent + 0x10);
        let body = adjusted_pointer(e, first_body);
        let world = e.mem.u32(body + 8);
        let counter = e.mem.u32(world + 0x94).wrapping_sub(1);
        e.mem.set_u32(world + 0x94, counter);
        e.mem.set_u8(world + 0x9c, 1);
        e.call(
            ADD_CONSTRAINT_IMMEDIATELY,
            &args![world, object + 0x44, 1u32],
        );
        let counter = e.mem.u32(world + 0x94).wrapping_add(1);
        e.mem.set_u32(world + 0x94, counter);
        e.mem.set_u8(world + 0x9c, 0);
    }
    let atoms = e.mem.u32(object + 0x3c);
    let count = u32::from(e.mem.u16(atoms + 4));
    let mut size_bytes = (count >> 1) * 0x70 + (count & 1) * 0x30 + 0x80;
    let mut elements = count + 2;
    let mut temps = count + 3;
    if count >= 2 {
        size_bytes += 0x20;
        elements += 1;
        temps += 1;
    }
    if e.mem.u32(object + 0x74) != 0 {
        let owner = e.mem.u32(object + 0x74);
        let island_at = e.mem.u32(owner + u32::from(e.mem.u8(owner + 0x1a)) * 4 + 4);
        let counters = e.mem.u32(island_at + 0xcc);
        let value = e.mem.u32(counters + 0xc).wrapping_sub(size_bytes);
        e.mem.set_u32(counters + 0xc, value);
        let value = e.mem.u32(counters + 0x10).wrapping_sub(elements);
        e.mem.set_u32(counters + 0x10, value);
        let value = e.mem.u32(counters + 0x14).wrapping_sub(temps);
        e.mem.set_u32(counters + 0x14, value);
        let owner = e.mem.u32(object + 0x74);
        let value = e.mem.u16(owner + 0x16).wrapping_sub(elements as u16);
        e.mem.set_u16(owner + 0x16, value);
        let value = e.mem.u16(owner + 0x18).wrapping_sub(temps as u16);
        e.mem.set_u16(owner + 0x18, value);
        let value = e.mem.u16(owner + 0x14).wrapping_sub(size_bytes as u16);
        e.mem.set_u16(owner + 0x14, value);
    }
    let atoms = e.mem.u32(object + 0x3c);
    let rounded = e
        .call(ATOM_SIZE_ROUND_UP, &args![u32::from(e.mem.u16(atoms + 2))])
        .u32();
    free_block(e, atoms, rounded);
    let atom_count = u32::from(e.mem.u16(saved + 0x24));
    let new_atoms = e
        .call(
            ALLOCATE_ATOM,
            &args![
                atom_count,
                u32::from(e.mem.u8(saved + 0x28)),
                u32::from(e.mem.u8(saved + 0x29)),
                u32::from(e.mem.u16(saved + 0x2c))
            ],
        )
        .u32();
    e.mem.set_u32(object + 0x3c, new_atoms);
    e.mem.set_u16(new_atoms + 4, atom_count as u16);
    let atoms = e.mem.u32(object + 0x3c);
    for word in 0..8 {
        let value = e.mem.u32(saved + 0x30 + 4 * word);
        e.mem.set_u32(atoms + 0x10 + 4 * word, value);
    }
    let atoms = e.mem.u32(object + 0x3c);
    let owner = e.mem.u32(object + 0x74);
    e.mem.set_u32(owner + 0xc, atoms);
    let atoms = e.mem.u32(object + 0x3c);
    let owner = e.mem.u32(object + 0x74);
    let word = e.mem.u16(atoms + 2);
    e.mem.set_u16(owner + 0x10, word);
    let atoms = e.mem.u32(object + 0x3c);
    let count = u32::from(e.mem.u16(atoms + 4));
    let size_bytes = (count >> 1) * 0x70 + (count & 1) * 0x30 + 0x80;
    let mut info = [size_bytes + 0x20, size_bytes, count + 2, count + 3];
    if count >= 2 {
        info[2] += 1;
        info[3] += 1;
        info[1] = size_bytes + 0x20;
    }
    let owner = e.mem.u32(object + 0x74);
    let island_at = e.mem.u32(owner + u32::from(e.mem.u8(owner + 0x1a)) * 4 + 4);
    let counters = e.mem.u32(island_at + 0xcc);
    // The stack frame of the exe: the ids array at +0x1C, the constraint info at +0x28
    // (00dce44e), an event at +0x3C and the words the exe stores there (some far away from
    // the event, at +0x30A0 to +0x30C4). The frame starts on a 16-byte boundary
    // (`AND ESP,0xfffffff0`, `0x30f4` bytes through `_chkstk`, three pushes).
    e.with_stack(COLLISION_FRAME + 0x10, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        let info_at = frame + 0x28;
        for (index, value) in info.iter().enumerate() {
            e.mem.set_u32(info_at + 4 * index as u32, *value);
        }
        e.call(
            ADD_CONSTRAINT_INFO,
            &args![counters, object + 0x44, info_at],
        );
        let atoms = e.mem.u32(object + 0x3c);
        let contact_bytes = u32::from(e.mem.u8(atoms + 0xa)).wrapping_mul(atom_count);
        let destination = atoms
            .wrapping_add(u32::from(e.mem.u16(atoms + 6)) << 5)
            .wrapping_add(0x30);
        let source = e.mem.u32(saved + 0x50);
        e.call(MEM_COPY, &args![destination, source, contact_bytes]);
        let source = e.mem.u32(saved + 0x5c);
        let atoms = e.mem.u32(object + 0x3c);
        e.call(MEM_COPY, &args![atoms + 0x30, source, atom_count << 5]);
        if e.mem.u8(options + 1) != 0 {
            let atoms = e.mem.u32(object + 0x3c);
            let stride = u32::from(e.mem.u8(atoms + 0xa));
            let mut cursor = atoms
                .wrapping_add(u32::from(e.mem.u16(atoms + 6)) << 5)
                .wrapping_add(0x30);
            if e.mem.u16(atoms + 4) != 0 {
                let mut index: i32 = 0;
                loop {
                    e.mem.set_u32(cursor + 8, 0);
                    let total = i32::from(e.mem.u16(e.mem.u32(object + 0x3c) + 4));
                    index += 1;
                    cursor = cursor.wrapping_add(stride);
                    if index >= total {
                        break;
                    }
                }
            }
        }
        let ids = frame + 0x1c;
        init_empty_array(e, ids);
        e.vcall(object, 0x30, &args![ids]);
        let event = frame + 0x3c;
        let mut count = e.mem.i32(ids + 4);
        if e.mem.u8(options + 8) != 0 && count > 0 {
            let mut index: u32 = 0;
            loop {
                let id = u32::from(
                    e.mem
                        .u16(e.mem.u32(ids).wrapping_add(index.wrapping_mul(2))),
                );
                let quiet = x87_quiet_bits(e.global::<u32>(ADDED_EVENT_FLOAT));
                e.mem.set_u32(frame + 0x30a0, quiet);
                e.mem.set_u32(frame + 0x30c0, 0);
                e.mem.set_u32(frame + 0x74, 0);
                e.mem.set_u32(frame + 0x30c4, 0);
                e.mem.set_u32(frame + 0x38, 0);
                e.mem.set_u32(frame + 0x10, 0);
                if e.mem.u8(options + 0xa) != 0 {
                    e.mem.set_u32(frame + 0x38, body_a + 0x10);
                    e.mem.set_u32(frame + 0x10, body_b + 0x10);
                }
                let point = e.vcall(object, 0x28, &args![id]).u32();
                let properties = e.vcall(object, 0x2c, &args![id]).u32();
                e.mem.set_u32(frame + 0x58, 0);
                let first = e.mem.u32(frame + 0x38);
                e.mem.set_u32(event, first);
                let second = e.mem.u32(frame + 0x10);
                e.mem.set_u32(event + 4, second);
                e.mem.set_u32(event + 0x10, properties);
                e.mem.set_u32(event + 8, 1);
                e.mem.set_u32(event + 0x14, 0);
                e.mem.set_u32(event + 0x18, point);
                e.mem.set_u32(event + 0x24, object);
                e.mem.set_u32(event + 0x28, context);
                e.mem.set_u32(event + 0x2c, frame + 0x70);
                e.mem.set_u32(event + 0x20, 0);
                e.mem.set_u16(event + 0x30, id as u16);
                if e.mem.u32(object + 8) == 0 {
                    let word = e.mem.u16(object + 0x12);
                    e.mem.set_u16(event + 0x32, word);
                } else {
                    e.mem.set_u16(event + 0x32, 0);
                }
                let world = e.mem.u32(body_a + 8);
                e.call(FIRE_CONTACT_POINT_ADDED, &args![world, event]);
                if e.mem.u16(body_a + 0x204) != 0 {
                    e.call(FIRE_CONTACT_POINT_ADDED_INTERNAL, &args![body_a, event]);
                }
                if e.mem.u16(body_b + 0x204) != 0 {
                    e.call(FIRE_CONTACT_POINT_ADDED_INTERNAL, &args![body_b, event]);
                }
                index += 1;
                count = e.mem.i32(ids + 4);
                if index as i32 >= count {
                    break;
                }
            }
        }
        if e.mem.u8(options + 9) != 0 {
            let total = count;
            if total > 0 {
                let mut index: u32 = 0;
                loop {
                    let id = u32::from(
                        e.mem
                            .u16(e.mem.u32(ids).wrapping_add(index.wrapping_mul(2))),
                    );
                    let point = e.vcall(object, 0x28, &args![id]).u32();
                    if e.mem.u8(point + 0xf) & 1 == 0 {
                        let point = e.vcall(object, 0x28, &args![id]).u32();
                        let properties = e.vcall(object, 0x2c, &args![id]).u32();
                        e.mem.set_u32(frame + 0x50, properties);
                        e.mem.set_u32(frame + 0x54, point);
                        e.mem.set_u32(frame + 0x64, point.wrapping_add(0x14));
                        let world = e.mem.u32(body_a + 8);
                        e.mem.set_u32(event, 2);
                        e.mem.set_u32(event + 0xc, object);
                        e.mem.set_u32(event + 4, body_a);
                        e.mem.set_u32(event + 8, body_b);
                        e.mem.set_u32(event + 0x10, 4);
                        e.mem.set_u8(event + 0x1c, 0);
                        e.mem.set_u8(event + 0x1d, 0);
                        e.mem.set_u8(event + 0x1e, 0);
                        e.mem.set_u32(event + 0x20, 0);
                        e.mem.set_u32(event + 0x24, 0);
                        e.mem.set_u32(event + 0x2c, 0);
                        e.mem.set_u32(event + 0x30, 0);
                        e.call(FIRE_CONTACT_POINT_CALLBACK, &args![world, event]);
                        e.mem.set_u32(event, 0);
                        if e.mem.u16(body_a + 0x204) != 0 {
                            e.call(FIRE_CONTACT_POINT_CALLBACK_INTERNAL, &args![body_a, event]);
                        }
                        e.mem.set_u32(event, 1);
                        if e.mem.u16(body_b + 0x204) != 0 {
                            e.call(FIRE_CONTACT_POINT_CALLBACK_INTERNAL, &args![body_b, event]);
                        }
                    }
                    index += 1;
                    if index as i32 >= total {
                        break;
                    }
                }
            }
        }
        e.call(INVALIDATE_TIM_IN_AGENT, &args![agent, context]);
        let allocator_object = allocator(e);
        let capacity = e.mem.u32(ids + 8);
        e.mem.set_u32(ids + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(ids);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(2);
            e.vcall(allocator_object, 0x10, &args![data, bytes]);
        }
    });
    0
}

// Translated from 00dce7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::loadContactPoints` (Xbox PDB), cdecl with three words
/// `(converter, group, world)`: runs `loadContactPointsInternal` with a selector object on the
/// stack whose only word is the vtable `010d77c0`.
pub fn hkp_save_contact_points_util_load_contact_points(
    e: &mut Engine,
    converter: Ptr,
    group: Ptr,
    world: Ptr,
) {
    // The exe's frame (`PUSH ECX`): the selector is all of it (00dce7b9).
    e.with_stack(4, |e, selector| {
        e.mem.set_u32(selector.addr(), LOAD_SELECTOR_VTABLE);
        e.call(
            LOAD_CONTACT_POINTS_INTERNAL,
            &args![converter, group, world, selector],
        );
    });
}

/// `hkpSaveContactPointsUtil::loadContactPointsInternal` (`00dcea10`).
const LOAD_CONTACT_POINTS_INTERNAL: u32 = 0x00dc_ea10;
/// `hkpWorld` functions the loader ends with: `00c91120` and `00c91140` (thiscall, no words).
const WORLD_FLUSH_A: u32 = 0x00c9_1120;
const WORLD_FLUSH_B: u32 = 0x00c9_1140;

// Translated from 00dce8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkEntitySelectorListed::isEntityOk` (Xbox PDB, anonymous namespace), thiscall with one
/// argument (`RET 4`): the map at `this + 4` answers its value for the entity (default 0).
pub fn entity_selector_listed_is_entity_ok(e: &mut Engine, this: Ptr, entity: u32) -> u32 {
    e.call(
        MAP_GET_WITH_DEFAULT,
        &args![this.addr().wrapping_add(4), entity, 0u32],
    )
    .u32()
}

// Translated from 00dce8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkEntitySelectorListed::scalar deleting destructor` (Xbox PDB, anonymous namespace),
/// thiscall with one argument (`RET 4`): clears and deallocates the map at `this + 4` with the
/// allocator, destroys it, sets the base vtable and, when bit 0 of the flag byte is set, frees the
/// block. Returns `this`.
pub fn entity_selector_listed_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let map = this.addr().wrapping_add(4);
    let allocator_object = allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator_object]);
    e.call(MAP_DESTRUCT, &args![map]);
    e.mem.set_u32(this.addr(), ENTITY_SELECTOR_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00dce930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed constructor of a saved collision record (`ECX` is `this`, the other registers are
/// not read): sets the vtable `010d77d8`, `-1` at `+0x10` and `+0x14`, the word `1` at `+6`, zero
/// at `+8`, `+0xC` and the byte at `+0x18`, the word `0x16` at `+0x20`, zero at `+0x30`, the word
/// `3` at `+0x32`, zero at `+0x34` to `+0x54`, empty arrays (data 0, size 0, capacity
/// `0x80000000`) at `+0x50`, `+0x5C`, `+0x68`, `+0x114` and `+0x120`, then `0x103` at `+0x12C`
/// and `1` at `+0x130`. Answers `this`.
pub fn fn_00dce930(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, RECORD_VTABLE);
    e.mem.set_u32(at + 0x10, u32::MAX);
    e.mem.set_u32(at + 0x14, u32::MAX);
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u8(at + 0x18, 0);
    e.mem.set_u16(at + 0x20, 0x16);
    e.mem.set_u16(at + 0x30, 0);
    e.mem.set_u16(at + 0x32, 3);
    for offset in (0x34..=0x54).step_by(4) {
        e.mem.set_u32(at + offset, 0);
    }
    for array in [0x58, 0x64, 0x70] {
        e.mem.set_u32(at + array, 0x8000_0000);
    }
    for offset in [0x5c, 0x60, 0x68, 0x6c] {
        e.mem.set_u32(at + offset, 0);
    }
    for array in [0x114, 0x120] {
        e.mem.set_u32(at + array, 0);
        e.mem.set_u32(at + array + 4, 0);
        e.mem.set_u32(at + array + 8, 0x8000_0000);
    }
    e.mem.set_u32(at + 0x12c, 0x103);
    e.mem.set_u32(at + 0x130, 1);
    this
}

// Translated from 00dcea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsUtil::loadContactPointsInternal` (Xbox PDB), cdecl with four words
/// `(converter, group, world, selector)`. The world's counter at `+0x94` is raised for the
/// duration. The group keeps `+0x44` an array of saved records and `+0x48` their count; they are
/// visited from the last to the first. A record's bodies are `[+8]`/`[+0xC]` (or, when the byte at
/// `+0x18` is nonzero, `converter + 4`'s function applied to `[+0x10]` and `[+0x14]`); when both
/// are not null and the selector's virtual function at `+4` accepts one of them,
/// `FindAgent(a + 0x10, b + 0x10)` finds the agent; for one, the record is byte-swapped when its
/// word at `+0x12C` is not `0x103` (`swapEndianTypeInCollisionEntry(world[+0x78], record)`) and
/// `deserializeCollisionEntry(converter, record, a, b, world[+0x78], agent)` runs. When the
/// converter's first byte is set and it answered nonzero, the record loses its reference and
/// is removed from the group by moving the last record into its place. The counter is lowered
/// at the end; when it reaches zero, the world's byte at `+0x9C` is clear, `00c91120` runs if
/// `[world + 0x8C]` is set, and `00c91140` if `[world + 0xA4] == 1` and `[world + 0x90]` is set.
pub fn hkp_save_contact_points_util_load_contact_points_internal(
    e: &mut Engine,
    converter: Ptr,
    group: Ptr,
    world: Ptr,
    selector: Ptr,
) {
    let converter = converter.addr();
    let group = group.addr();
    let world = world.addr();
    let selector = selector.addr();
    let counter = e.mem.u32(world + 0x94).wrapping_add(1);
    e.mem.set_u32(world + 0x94, counter);
    let mut index = e.mem.i32(group + 0x48).wrapping_sub(1);
    if index >= 0 {
        loop {
            let list = e.mem.u32(group + 0x44);
            let entry = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
            let (body_a, body_b) = if e.mem.u8(entry + 0x18) != 0 {
                let function = e.mem.u32(converter + 4);
                let first = e.mem.u32(entry + 0x10);
                let body_a = e.call(function, &args![first]).u32();
                let function = e.mem.u32(converter + 4);
                let second = e.mem.u32(entry + 0x14);
                let body_b = e.call(function, &args![second]).u32();
                (body_a, body_b)
            } else {
                (e.mem.u32(entry + 8), e.mem.u32(entry + 0xc))
            };
            if body_a != 0 && body_b != 0 {
                let mut accepted = e.vcall(selector, 4, &args![body_a]).u32() != 0;
                if !accepted {
                    accepted = e.vcall(selector, 4, &args![body_b]).u32() != 0;
                }
                if accepted {
                    let agent = e
                        .call(FIND_AGENT, &args![body_a + 0x10, body_b + 0x10])
                        .u32();
                    if agent != 0 {
                        if e.mem.u32(entry + 0x12c) != 0x103 {
                            let context = e.mem.u32(world + 0x78);
                            e.call(SWAP_ENDIAN_COLLISION_ENTRY, &args![context, entry]);
                        }
                        let context = e.mem.u32(world + 0x78);
                        let failed = e
                            .call(
                                DESERIALIZE_COLLISION_ENTRY,
                                &args![converter, entry, body_a, body_b, context, agent],
                            )
                            .u32();
                        if e.mem.u8(converter) != 0 && failed != 0 {
                            let list = e.mem.u32(group + 0x44);
                            let doomed =
                                e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
                            e.call(REMOVE_REFERENCE, &args![doomed]);
                            let remaining = e.mem.i32(group + 0x48).wrapping_sub(1);
                            e.mem.set_u32(group + 0x48, remaining as u32);
                            let last = e.mem.i32(group + 0x48);
                            if last != index {
                                let list = e.mem.u32(group + 0x44);
                                let moved =
                                    e.mem.u32(list.wrapping_add((last as u32).wrapping_mul(4)));
                                e.mem.set_u32(
                                    list.wrapping_add((index as u32).wrapping_mul(4)),
                                    moved,
                                );
                            }
                        }
                    }
                }
            }
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
        }
    }
    let counter = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, counter);
    if counter == 0 && e.mem.u8(world + 0x9c) == 0 {
        if e.mem.u32(world + 0x8c) != 0 {
            e.call(WORLD_FLUSH_A, &args![world]);
        }
        if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
            e.call(WORLD_FLUSH_B, &args![world]);
        }
    }
}
// Translated from 00dce9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_5d10`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dce9e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_5d10, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_d4b0, entity_selector_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00dc_d4d0, hkp_save_contact_points_util_get_serialized_agent_type(u32) -> u32),
        entry!(0x00dc_d550, hkp_save_contact_points_util_serialize_entry(Ptr, u8, Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc_d6a0, hkp_save_contact_points_util_destroy_old_entry(u32, Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00dc_d720, hkp_save_contact_points_util_serialize_collision_entry(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc_d950, hkp_save_contact_points_util_deserialize_1n_track(Ptr, u32, Ptr) -> u32),
        entry!(0x00dc_da40, fn_00dcda40(Ptr, Ptr, Ptr)),
        entry!(
            0x00dc_dc00,
            hkp_save_contact_points_util_save_contact_points(Ptr, Ptr, i32, Ptr)
        ),
        entry!(0x00dc_de10, hkp_save_contact_points_util_deserialize_entry(Ptr, u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc_df70, hkp_save_contact_points_util_serialize_1n_track(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dc_e180, hkp_save_contact_points_util_deserialize_collision_entry(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00dc_e7b0,
            hkp_save_contact_points_util_load_contact_points(Ptr, Ptr, Ptr)
        ),
        entry!(0x00dc_e8d0, entity_selector_listed_is_entity_ok(Ptr, u32) -> u32),
        entry!(0x00dc_e8f0, entity_selector_listed_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00dc_e930, fn_00dce930(Ptr) -> Ptr),
        entry!(0x00dc_e9e0, deleting_destructor_00dce9e0(Ptr, u8) -> Ptr),
        entry!(
            0x00dc_ea10,
            hkp_save_contact_points_util_load_contact_points_internal(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log, Log};
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dce9e0_frees_only_when_bit_zero_is_set() {
        check(0x00dc_e9e0, 0x00e0_5d10);
    }

    const ALLOCATE_DOUBLE: u32 = 0x0900_0012;
    const ARRAY_ALLOCATE_DOUBLE: u32 = 0x0900_0013;
    const ARRAY_FREE_DOUBLE: u32 = 0x0900_0014;
    const SELECT_DOUBLE: u32 = 0x0900_0020;
    const MAP_FUNCTION_DOUBLE: u32 = 0x0900_0030;
    const CALLBACK_DOUBLE: u32 = 0x0900_0040;
    const PROCESSES: [u32; 9] = [
        0x00d3_fe90,
        0x00d3_9390,
        0x00d4_1a30,
        0x00d4_0d80,
        0x00d3_2cd0,
        0x00da_6eb0,
        0x00d2_d890,
        0x00d3_5770,
        0x00d3_65d0,
    ];

    /// A memory router whose allocator hands out blocks and logs its calls: slot 4 allocates
    /// (zeroed), slot 8 frees a block, slot `0xC` grants the asked byte count plus 8, slot `0x10`
    /// frees an array. Returns the allocator.
    fn router(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0xc, ARRAY_ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, ARRAY_FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router_block = e.mem.alloc(0x20);
        e.mem.set_u32(router_block + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router_block,
            ..Ret::default()
        });
        let allocate_log = Rc::clone(log);
        e.register_double(ALLOCATE_DOUBLE, move |e, a| {
            allocate_log
                .borrow_mut()
                .push((ALLOCATE_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_double(e, log, FREE_SLOT_DOUBLE, 0);
        let array_log = Rc::clone(log);
        e.register_double(ARRAY_ALLOCATE_DOUBLE, move |e, a| {
            array_log
                .borrow_mut()
                .push((ARRAY_ALLOCATE_DOUBLE, a.to_vec()));
            let bytes = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], bytes + 8);
            Ret {
                eax: e.mem.alloc(bytes + 8),
                ..Ret::default()
            }
        });
        log_double(e, log, ARRAY_FREE_DOUBLE, 0);
        allocator
    }

    /// `hkArrayBase::_reserveMore`-style reserve (`00c908c0`): a fresh block for `capacity`
    /// elements, stored with the capacity in the array header.
    fn reserve_double(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(RESERVE_ARRAY, move |e, a| {
            log.borrow_mut().push((RESERVE_ARRAY, a.to_vec()));
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    /// `hkString::memCpy` (`00cb1260`): copies `(destination, source, count)`.
    fn copy_double(e: &mut Engine) {
        e.register_double(MEM_COPY, |e, a| {
            for index in 0..a[2] {
                let byte = e.mem.u8(a[1] + index);
                e.mem.set_u8(a[0] + index, byte);
            }
            Ret::default()
        });
    }

    /// A context whose agent table (the word at the context) names `process` for the agent
    /// index `slot`.
    fn context_with_agent(e: &mut Engine, slot: u8, process: u32) -> u32 {
        let table = e.mem.alloc(0x2000);
        e.mem
            .set_u32(table + 0x16dc + 0x50 * u32::from(slot), process);
        let context = e.mem.alloc(8);
        e.mem.set_u32(context, table);
        context
    }

    /// An entry (0x80 bytes) of the given type, agent index and size byte.
    fn entry_of(e: &mut Engine, kind: u8, slot: u8, size: u8) -> u32 {
        let entry = e.mem.alloc(0x80);
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, slot);
        e.mem.set_u8(entry + 3, size);
        entry
    }

    /// Fills `len` bytes at `at` with `first`, `first + 1`, ...
    fn fill(e: &mut Engine, at: u32, len: u32, first: u8) {
        for index in 0..len {
            e.mem.set_u8(at + index, first.wrapping_add(index as u8));
        }
    }

    fn bytes(e: &Engine, at: u32, len: u32) -> Vec<u8> {
        (0..len).map(|index| e.mem.u8(at + index)).collect()
    }

    #[test]
    fn entity_selector_destructor_00dcd4b0_sets_the_vtable_and_frees_on_bit_zero() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, FREE, 0);
        let this = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00dc_d4b0, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ENTITY_SELECTOR_VTABLE);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(e.call(0x00dc_d4b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&log, FREE), vec![vec![this]]);
    }

    #[test]
    fn serialized_agent_type_00dcd4d0_numbers_the_nine_process_functions() {
        let mut e = engine();
        for (index, process) in PROCESSES.into_iter().enumerate() {
            assert_eq!(e.call(0x00dc_d4d0, &args![process]).u32(), index as u32 + 1);
        }
        for other in [0u32, 0x00d3_fe91, 0xffff_ffff] {
            assert_eq!(e.call(0x00dc_d4d0, &args![other]).u32(), 0);
        }
    }

    /// Calls `serializeEntry` with the three output words at `outs` (size, kind, track).
    fn serialize(
        e: &mut Engine,
        entry: u32,
        larger: u32,
        context: u32,
        outs: u32,
        record: u32,
    ) -> u32 {
        e.call(
            0x00dc_d550,
            &args![entry, larger, context, outs, outs + 4, outs + 8, record],
        )
        .u32()
    }

    #[test]
    fn serialize_entry_00dcd550_answers_by_entry_type() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SERIALIZE_1N_TRACK, 5);
        let context = context_with_agent(&mut e, 0, 0);
        let outs = e.mem.alloc(0x10);
        for kind in [0u8, 1] {
            for index in 0..3 {
                e.mem.set_u32(outs + 4 * index, 0x5555_5555);
            }
            let entry = entry_of(&mut e, kind, 0, 9);
            assert_eq!(serialize(&mut e, entry, 1, context, outs, 0), 0);
            assert_eq!(e.mem.u32(outs), 0x10);
            assert_eq!(e.mem.u32(outs + 4), 0x5555_5555);
            assert_eq!(e.mem.u32(outs + 8), 0);
        }
        for (kind, answer) in [
            (6u8, 1u32),
            (14, 1),
            (7, 0),
            (8, 0),
            (9, 0),
            (15, 0),
            (200, 0),
        ] {
            for index in 0..3 {
                e.mem.set_u32(outs + 4 * index, 0x5555_5555);
            }
            let entry = entry_of(&mut e, kind, 0, 9);
            assert_eq!(serialize(&mut e, entry, 0, context, outs, 0), answer);
            assert_eq!(e.mem.u32(outs), 0x5555_5555);
            assert_eq!(e.mem.u32(outs + 4), 0x5555_5555);
            assert_eq!(e.mem.u32(outs + 8), 0);
        }
        assert!(calls(&log, SERIALIZE_1N_TRACK).is_empty());
    }

    #[test]
    fn serialize_entry_00dcd550_stores_the_size_of_the_simple_agent_types() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SERIALIZE_1N_TRACK, 5);
        let outs = e.mem.alloc(0x10);
        for (agent, process) in PROCESSES[..4].iter().enumerate() {
            let context = context_with_agent(&mut e, 3, *process);
            for kind in [2u8, 3, 4, 10, 11, 12, 13] {
                e.mem.set_u32(outs + 8, 0x5555_5555);
                let entry = entry_of(&mut e, kind, 3, 0x28);
                assert_eq!(serialize(&mut e, entry, 1, context, outs, 0), 0);
                assert_eq!(e.mem.u32(outs), 0x28);
                assert_eq!(e.mem.u32(outs + 4), agent as u32 + 1);
                assert_eq!(e.mem.u32(outs + 8), 0);
            }
        }
        assert!(calls(&log, SERIALIZE_1N_TRACK).is_empty());
    }

    #[test]
    fn serialize_entry_00dcd550_takes_the_track_of_agent_type_five() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, GET_AGENT_1N_TRACK_SERIALIZE, 0x7000);
        log_double(&mut e, &log, SERIALIZE_1N_TRACK, 9);
        let context = context_with_agent(&mut e, 2, PROCESSES[4]);
        let outs = e.mem.alloc(0x10);
        let entry = entry_of(&mut e, 4, 2, 0x30);
        let record = e.mem.alloc(0x10);
        // Bit 5 of the byte at +0xB of the payload set: the size is stored and nothing else.
        e.mem.set_u8(entry + 0x20 + 0xb, 0x20);
        assert_eq!(serialize(&mut e, entry, 0, context, outs, record), 0);
        assert_eq!(e.mem.u32(outs), 0x30);
        assert_eq!(e.mem.u32(outs + 4), 5);
        assert_eq!(e.mem.u32(outs + 8), 0);
        assert!(calls(&log, SERIALIZE_1N_TRACK).is_empty());
        // Clear: the track comes from the agent and is serialized (payload at +0x30 for the
        // larger layout).
        e.mem.set_u8(entry + 0x30 + 0xb, 0);
        assert_eq!(serialize(&mut e, entry, 1, context, outs, record), 9);
        assert_eq!(e.mem.u32(outs + 8), 0x7000);
        assert_eq!(
            calls(&log, GET_AGENT_1N_TRACK_SERIALIZE),
            vec![vec![entry, entry + 0x30]]
        );
        assert_eq!(
            calls(&log, SERIALIZE_1N_TRACK),
            vec![vec![0x7000, context, record]]
        );
    }

    #[test]
    fn serialize_entry_00dcd550_serializes_the_payload_for_agent_types_six_to_nine() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SERIALIZE_1N_TRACK, 11);
        let outs = e.mem.alloc(0x10);
        let record = e.mem.alloc(0x10);
        for (agent, process) in PROCESSES.iter().enumerate().skip(5) {
            let context = context_with_agent(&mut e, 1, *process);
            for (kind, offset) in [
                (2u8, 0x20u32),
                (3, 0x20),
                (10, 0x20),
                (11, 0x20),
                (5, 0x30),
                (13, 0x30),
            ] {
                log.borrow_mut().clear();
                let entry = entry_of(&mut e, kind, 1, 0x44);
                assert_eq!(serialize(&mut e, entry, 1, context, outs, record), 11);
                assert_eq!(e.mem.u32(outs), 0x44);
                assert_eq!(e.mem.u32(outs + 4), agent as u32 + 1);
                assert_eq!(e.mem.u32(outs + 8), entry + offset);
                assert_eq!(
                    calls(&log, SERIALIZE_1N_TRACK),
                    vec![vec![entry + offset, context, record]]
                );
            }
        }
    }

    #[test]
    fn serialize_entry_00dcd550_fails_for_an_unknown_agent_function() {
        let mut e = engine();
        let context = context_with_agent(&mut e, 0, 0x00d3_fe91);
        let outs = e.mem.alloc(0x10);
        let entry = entry_of(&mut e, 2, 0, 0x30);
        assert_eq!(serialize(&mut e, entry, 0, context, outs, 0), 1);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(e.mem.u32(outs), 0);
    }

    #[test]
    fn destroy_old_entry_00dcd6a0_calls_the_agent_function_for_the_wanted_kind() {
        for (kind, offset) in [(2u8, 0x20u32), (4, 0x30)] {
            let mut e = engine();
            let log = new_log();
            log_double(&mut e, &log, CALLBACK_DOUBLE, 0);
            let context = context_with_agent(&mut e, 2, PROCESSES[3]);
            let table = e.mem.u32(context);
            e.mem.set_u32(table + 0x16b4 + 2 * 0x50, CALLBACK_DOUBLE);
            let entry = entry_of(&mut e, kind, 2, 0);
            let got = e.call(0x00dc_d6a0, &args![4u32, context, 0x11u32, 0x22u32, entry]);
            assert_eq!(got.u32(), 0);
            assert_eq!(
                calls(&log, CALLBACK_DOUBLE),
                vec![vec![entry, entry + offset, 0x11, 0x22, table]]
            );
            // Another wanted kind, or none: nothing is called.
            for wanted in [3u32, 0] {
                let got = e.call(
                    0x00dc_d6a0,
                    &args![wanted, context, 0x11u32, 0x22u32, entry],
                );
                assert_eq!(got.u32(), 1);
            }
            assert_eq!(calls(&log, CALLBACK_DOUBLE).len(), 1);
        }
    }

    #[test]
    fn destroy_old_entry_00dcd6a0_fails_for_other_entry_types() {
        let mut e = engine();
        let context = context_with_agent(&mut e, 2, PROCESSES[3]);
        for kind in [0u8, 1, 3, 5, 6, 12] {
            let entry = entry_of(&mut e, kind, 2, 0);
            let got = e.call(0x00dc_d6a0, &args![4u32, context, 0u32, 0u32, entry]);
            assert_eq!(got.u32(), 1);
        }
    }

    /// What `serializeCollisionEntry` works on.
    struct Collision {
        converter: u32,
        entry: u32,
        object: u32,
        header: u32,
        record: u32,
        context: u32,
        first: u32,
    }

    /// An entry whose object has `count` contact points (stride 3, offset 1 in 32-byte units),
    /// five array bytes and the vtable function at `0x34` answering `busy`; `serializeEntry`
    /// is a double that stores the size `0x10` and the kind 7 and answers `failed`.
    fn collision(e: &mut Engine, log: &Log, busy: u32, count: u16, failed: u32) -> Collision {
        router(e, log);
        reserve_double(e, log);
        copy_double(e);
        log_double(e, log, ADD_REFERENCE, 0);
        log_double(e, log, SELECT_DOUBLE, busy);
        let serialize_log = Rc::clone(log);
        e.register_double(SERIALIZE_ENTRY, move |e, a| {
            serialize_log
                .borrow_mut()
                .push((SERIALIZE_ENTRY, a.to_vec()));
            e.mem.set_u32(a[3], 0x10);
            e.mem.set_u32(a[4], 7);
            Ret {
                eax: failed,
                ..Ret::default()
            }
        });
        e.register_double(MAP_FUNCTION_DOUBLE, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x34, SELECT_DOUBLE);
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        let header = e.mem.alloc(0x200);
        fill(e, header, 0x200, 1);
        e.mem.set_u16(header + 4, count);
        e.mem.set_u8(header + 0xa, 3);
        e.mem.set_u16(header + 6, 1);
        e.mem.set_u32(object + 0x3c, header);
        let array = e.mem.alloc(8);
        fill(e, array, 8, 0x90);
        e.mem.set_u32(object + 0x20, array);
        e.mem.set_u32(object + 0x24, 5);
        let first = e.mem.alloc(0x40);
        e.mem.set_u8(first + 0x10, 0xfc);
        e.mem.set_u8(first + 0x18, 1);
        let second = e.mem.alloc(0x40);
        let entry = e.mem.alloc(0x40);
        fill(e, entry, 0x40, 0x40);
        e.mem.set_u32(entry + 8, object);
        e.mem.set_u32(entry + 0x10, first);
        e.mem.set_u32(entry + 0x14, second);
        let converter = e.mem.alloc(8);
        e.mem.set_u32(converter + 4, MAP_FUNCTION_DOUBLE);
        Collision {
            converter,
            entry,
            object,
            header,
            record: e.mem.alloc(0x200),
            context: 0x7007,
            first,
        }
    }

    fn serialize_collision(e: &mut Engine, c: &Collision) -> u32 {
        e.call(
            0x00dc_d720,
            &args![c.converter, c.entry, c.context, c.record],
        )
        .u32()
    }

    #[test]
    fn serialize_collision_entry_00dcd720_fills_the_record_and_references_the_bodies() {
        let mut e = engine();
        let log = new_log();
        let c = collision(&mut e, &log, 0, 2, 0);
        // The exe's 0x14-byte frame: the kind at +0x8, the size at +0xc, the track at +0x10.
        let frame = e.next_stack_local(0x14);
        assert_eq!(serialize_collision(&mut e, &c), 0);
        let serialize = calls(&log, SERIALIZE_ENTRY);
        assert_eq!(serialize.len(), 1);
        assert_eq!(serialize[0][..3], [c.entry, 1, c.context]);
        assert_eq!(serialize[0][3..6], [frame + 0xc, frame + 8, frame + 0x10]);
        assert_eq!(serialize[0][6], c.record + 0x114);
        assert_eq!(e.mem.u8(c.record + 0x19), 7);
        assert_eq!(bytes(&e, c.record + 0x74, 0x10), bytes(&e, c.entry, 0x10));
        // The array of the object: reserved for five bytes and copied.
        assert_eq!(e.mem.u32(c.record + 0x6c), 5);
        let array = e.mem.u32(c.record + 0x68);
        assert_eq!(
            bytes(&e, array, 5),
            bytes(&e, e.mem.u32(c.object + 0x20), 5)
        );
        // The contact bytes: stride 3 times 2 points, from header + (1 << 5) + 0x30.
        assert_eq!(e.mem.u32(c.record + 0x54), 6);
        assert_eq!(
            bytes(&e, e.mem.u32(c.record + 0x50), 6),
            bytes(&e, c.header + 0x50, 6)
        );
        // The points: two of 32 bytes from header + 0x30.
        assert_eq!(e.mem.u32(c.record + 0x60), 2);
        assert_eq!(
            bytes(&e, e.mem.u32(c.record + 0x5c), 0x40),
            bytes(&e, c.header + 0x30, 0x40)
        );
        assert_eq!(bytes(&e, c.record + 0x20, 0x30), bytes(&e, c.header, 0x30));
        assert_eq!(
            calls(&log, RESERVE_ARRAY)
                .iter()
                .map(|call| call[2..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![5, 1], vec![6, 1], vec![2, 0x20]]
        );
        // No converter: the bodies are kept (the second collidable is not a body) and referenced.
        assert_eq!(e.mem.u8(c.record + 0x18), 0);
        assert_eq!(e.mem.u32(c.record + 8), c.first - 4);
        assert_eq!(e.mem.u32(c.record + 0xc), 0);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![c.first - 4], vec![0]]);
    }

    #[test]
    fn serialize_collision_entry_00dcd720_maps_the_bodies_through_the_converter() {
        let mut e = engine();
        let log = new_log();
        let c = collision(&mut e, &log, 0, 2, 0);
        e.mem.set_u8(c.converter, 1);
        assert_eq!(serialize_collision(&mut e, &c), 0);
        assert_eq!(e.mem.u8(c.record + 0x18), 1);
        assert_eq!(e.mem.u32(c.record + 0x10), c.first - 4 + 0x1000);
        assert_eq!(e.mem.u32(c.record + 0x14), 0x1000);
        assert_eq!(e.mem.u32(c.record + 8), 0);
        assert!(calls(&log, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn serialize_collision_entry_00dcd720_gives_up_without_changing_the_record() {
        // The object answers nonzero from its virtual function at 0x34.
        let mut e = engine();
        let log = new_log();
        let c = collision(&mut e, &log, 1, 2, 0);
        assert_eq!(serialize_collision(&mut e, &c), 1);
        assert!(calls(&log, SERIALIZE_ENTRY).is_empty());
        // The header has no contact points.
        let mut e = engine();
        let log = new_log();
        let c = collision(&mut e, &log, 0, 0, 0);
        assert_eq!(serialize_collision(&mut e, &c), 1);
        assert!(calls(&log, SERIALIZE_ENTRY).is_empty());
        // serializeEntry fails.
        let mut e = engine();
        let log = new_log();
        let c = collision(&mut e, &log, 0, 2, 1);
        assert_eq!(serialize_collision(&mut e, &c), 1);
        assert_eq!(calls(&log, SERIALIZE_ENTRY).len(), 1);
        assert_eq!(e.mem.u8(c.record + 0x19), 0);
        assert!(calls(&log, RESERVE_ARRAY).is_empty());
    }

    #[test]
    fn deserialize_1n_track_00dcd950_copies_the_items_and_clears_the_nested_tracks() {
        let mut e = engine();
        let log = new_log();
        router(&mut e, &log);
        reserve_double(&mut e, &log);
        let items = e.mem.alloc(8);
        let first = e.mem.alloc(0x200);
        fill(&mut e, first, 0x200, 0x10);
        let second = e.mem.alloc(0x200);
        fill(&mut e, second, 0x200, 0x70);
        e.mem.set_u32(items, first);
        e.mem.set_u32(items + 4, second);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child + 0x18, 1);
        e.mem.set_u32(child + 0x1c, 0x40);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, child);
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source, items);
        e.mem.set_u32(source + 4, 2);
        e.mem.set_u32(source + 0xc, children);
        e.mem.set_u32(source + 0x10, 1);
        let track = e.mem.alloc(0xc);
        let got = e.call(0x00dc_d950, &args![source, 0x1234u32, track]).u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(track + 4), 2);
        let blocks = e.mem.u32(track);
        let (copy_one, copy_two) = (e.mem.u32(blocks), e.mem.u32(blocks + 4));
        assert_eq!(bytes(&e, copy_one, 0x200), bytes(&e, first, 0x200));
        assert_ne!(copy_one, first);
        // The nested track of the child: in the second copy at 0x10 + 0x40.
        assert_eq!(bytes(&e, copy_two, 0x50), bytes(&e, second, 0x50));
        let nested = copy_two + 0x50;
        assert_eq!(
            [
                e.mem.u32(nested),
                e.mem.u32(nested + 4),
                e.mem.u32(nested + 8)
            ],
            [0, 0, 0x8000_0000]
        );
        assert_eq!(
            bytes(&e, copy_two + 0x5c, 0x40),
            bytes(&e, second + 0x5c, 0x40)
        );
        assert_eq!(
            calls(&log, RESERVE_ARRAY)
                .iter()
                .map(|call| call[2..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![2, 4]]
        );
        assert_eq!(calls(&log, ALLOCATE_DOUBLE).len(), 2);
    }

    /// The group of `00dcda40`: two items of one block each; item 0 has a last block of 0x40
    /// bytes (two entries of 0x20), item 1 of 0x20 (one entry). Returns the group and the
    /// three entries in order.
    fn group(e: &mut Engine, count: u32) -> (u32, [u32; 3]) {
        let group = e.mem.alloc(0x100);
        let list = e.mem.alloc(8);
        let mut entries = [0; 3];
        for (index, last) in [0x40u32, 0x20].into_iter().enumerate() {
            let block = e.mem.alloc(0x200);
            for entry in [0u32, 0x20] {
                e.mem.set_u8(block + entry + 3, 0x20);
            }
            if index == 0 {
                entries[0] = block;
                entries[1] = block + 0x20;
            } else {
                entries[2] = block;
            }
            let blocks = e.mem.alloc(4);
            e.mem.set_u32(blocks, block);
            let item = e.mem.alloc(0x80);
            e.mem.set_u32(item + 0x5c, blocks);
            e.mem.set_u32(item + 0x60, 1);
            e.mem.set_u32(item + 0x58, last);
            e.mem.set_u32(list + 4 * index as u32, item);
        }
        e.mem.set_u32(group + 0x28, list);
        e.mem.set_u32(group + 0x2c, count);
        e.mem.set_u32(group + 0x34, 0x11);
        e.mem.set_u32(group + 0x38, 0x22);
        e.mem.set_u32(group + 0x78, 0xc0de);
        (group, entries)
    }

    /// Doubles for the record handling shared by `00dcda40` and `00dcdc00`: the constructor
    /// answers its `this`, the serializer fails for the entry `bad` and succeeds for the others.
    fn record_doubles(e: &mut Engine, log: &Log, bad: u32) {
        let construct_log = Rc::clone(log);
        e.register_double(CONSTRUCT_RECORD, move |_, a| {
            construct_log
                .borrow_mut()
                .push((CONSTRUCT_RECORD, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let serialize_log = Rc::clone(log);
        e.register_double(SERIALIZE_COLLISION_ENTRY, move |_, a| {
            serialize_log
                .borrow_mut()
                .push((SERIALIZE_COLLISION_ENTRY, a.to_vec()));
            Ret {
                eax: u32::from(a[1] == bad),
                ..Ret::default()
            }
        });
        log_double(e, log, STORE_RECORD, 0);
        log_double(e, log, REMOVE_REFERENCE, 0);
    }

    /// The sort `00cd1e20` of `00dcda40`: logs its words and, behind them, the array header
    /// and the first `count` elements.
    fn sort_double(e: &mut Engine, log: &Log) {
        let sort_log = Rc::clone(log);
        e.register_double(SORT_ARRAY, move |e, a| {
            let mut words = a.to_vec();
            let size = e.mem.i32(a[0] + 4);
            words.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)]);
            for index in 0..size.max(0) as u32 {
                words.push(e.mem.u32(e.mem.u32(a[0]) + 4 * index));
            }
            sort_log.borrow_mut().push((SORT_ARRAY, words));
            Ret::default()
        });
    }

    #[test]
    fn fn_00dcda40_serializes_every_entry_of_every_item() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        sort_double(&mut e, &log);
        let (group, entries) = group(&mut e, 2);
        record_doubles(&mut e, &log, entries[1]);
        let converter = e.mem.alloc(8);
        let container = e.mem.alloc(8);
        // The sorted array sits at +0x14 of the exe's 0x30-byte frame (with the arguments).
        let frame = e.next_stack_local(0x30);
        e.call(0x00dc_da40, &args![converter, group, container]);
        let list = e.mem.u32(group + 0x28);
        // The sort sees: this, allocator, count, the two words, the array header and the items.
        let sort = calls(&log, SORT_ARRAY);
        assert_eq!(sort.len(), 1);
        assert_eq!(sort[0][0], frame + 0x14);
        assert_eq!(sort[0][1..5], [allocator, 2, 0x11, 0x22]);
        let data = sort[0][5];
        assert_eq!(
            sort[0][5..],
            [data, 2, 4, e.mem.u32(list), e.mem.u32(list + 4)]
        );
        // 8 bytes asked for, 16 granted: the array holds 4 elements and is freed as 16 bytes.
        assert_eq!(calls(&log, ARRAY_ALLOCATE_DOUBLE).len(), 1);
        assert_eq!(
            calls(&log, ARRAY_FREE_DOUBLE),
            vec![vec![allocator, data, 16]]
        );
        let serialized = calls(&log, SERIALIZE_COLLISION_ENTRY);
        assert_eq!(
            serialized.iter().map(|call| call[1]).collect::<Vec<_>>(),
            entries.to_vec()
        );
        for call in &serialized {
            assert_eq!(call[0], converter);
            assert_eq!(call[2], 0xc0de);
        }
        let records: Vec<u32> = serialized.iter().map(|call| call[3]).collect();
        assert_eq!(
            calls(&log, CONSTRUCT_RECORD),
            records
                .iter()
                .map(|record| vec![*record])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls(&log, ALLOCATE_DOUBLE),
            records
                .iter()
                .map(|_| vec![allocator, 0x140])
                .collect::<Vec<_>>()
        );
        for record in &records {
            assert_eq!(e.mem.u16(record + 4), 0x140);
        }
        // The record of the entry that failed is not stored; every record loses a reference.
        assert_eq!(
            calls(&log, STORE_RECORD),
            vec![vec![container, records[0]], vec![container, records[2]]]
        );
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            records
                .iter()
                .map(|record| vec![*record])
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fn_00dcda40_sorts_an_empty_group_without_taking_memory() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        sort_double(&mut e, &log);
        record_doubles(&mut e, &log, 0);
        let (group, _) = group(&mut e, 0);
        e.call(0x00dc_da40, &args![0u32, group, 0u32]);
        let sort = calls(&log, SORT_ARRAY);
        assert_eq!(sort.len(), 1);
        assert_eq!(sort[0][1..], [allocator, 0, 0x11, 0x22, 0, 0, 0x8000_0000]);
        assert!(calls(&log, ARRAY_ALLOCATE_DOUBLE).is_empty());
        assert!(calls(&log, ARRAY_FREE_DOUBLE).is_empty());
        assert!(calls(&log, SERIALIZE_COLLISION_ENTRY).is_empty());
    }

    /// A body whose world (`+8`) is `world`.
    fn body_in(e: &mut Engine, world: u32) -> u32 {
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 8, world);
        body
    }

    #[test]
    fn save_contact_points_00dcdc00_serializes_the_pairs_that_were_not_seen() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        // The pair (0x4201, 0x4202) fails to serialize.
        record_doubles(&mut e, &log, 0x4201);
        for address in [
            MAP_RESERVE,
            MAP_INSERT,
            MAP_CLEAR_AND_DEALLOCATE,
            MAP_DESTRUCT,
        ] {
            log_double(&mut e, &log, address, 0);
        }
        // The map has the other collidable 0x4101 already (an index not above -1).
        let find_log = Rc::clone(&log);
        e.register_double(MAP_FIND_KEY, move |_, a| {
            find_log.borrow_mut().push((MAP_FIND_KEY, a.to_vec()));
            Ret {
                eax: if a[1] == 0x4101 { 0xffff_fffe } else { 3 },
                ..Ret::default()
            }
        });
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, 0xc0de);
        let first = body_in(&mut e, world);
        let second = body_in(&mut e, world);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, first);
        e.mem.set_u32(bodies + 4, second);
        // The first collidable has two pairs in a list it does not own; the second has one in an
        // owned list of four elements.
        let list_log = Rc::clone(&log);
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            list_log
                .borrow_mut()
                .push((GET_COLLISION_ENTRIES_SORTED, a.to_vec()));
            let first_collidable = a[0] == first + 0x10;
            let pairs: &[u32] = if first_collidable {
                &[0x4001, 0x4101, 0x4002, 0x4102]
            } else {
                &[0x4201, 0x4202]
            };
            let (count, capacity) = if first_collidable {
                (2, 0x8000_0002u32)
            } else {
                (1, 4)
            };
            let data = e.mem.alloc(0x20);
            for (index, word) in pairs.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *word);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        let container = e.mem.alloc(8);
        let converter = e.mem.alloc(8);
        // The body map sits at +0x20 of the exe's 0x2c-byte frame.
        let frame = e.next_stack_local(0x2c);
        e.call(0x00dc_dc00, &args![converter, bodies, 2i32, container]);
        let reserved = calls(&log, MAP_RESERVE);
        assert_eq!(reserved.len(), 1);
        assert_eq!(reserved[0][0], frame + 0x20);
        assert_eq!(reserved[0][2], 2);
        assert_eq!(
            calls(&log, MAP_INSERT)
                .iter()
                .map(|call| call[2..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![first + 0x10, 0], vec![second + 0x10, 0]]
        );
        assert_eq!(
            calls(&log, MAP_FIND_KEY)
                .iter()
                .map(|call| call[1])
                .collect::<Vec<_>>(),
            vec![0x4101, 0x4102, 0x4202]
        );
        let serialized = calls(&log, SERIALIZE_COLLISION_ENTRY);
        assert_eq!(
            serialized
                .iter()
                .map(|call| (call[1], call[2]))
                .collect::<Vec<_>>(),
            vec![(0x4002, 0xc0de), (0x4201, 0xc0de)]
        );
        let records: Vec<u32> = serialized.iter().map(|call| call[3]).collect();
        assert_eq!(calls(&log, STORE_RECORD), vec![vec![container, records[0]]]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            records
                .iter()
                .map(|record| vec![*record])
                .collect::<Vec<_>>()
        );
        assert_eq!(calls(&log, ARRAY_FREE_DOUBLE).len(), 1);
        assert_eq!(calls(&log, ARRAY_FREE_DOUBLE)[0][0], allocator);
        assert_eq!(calls(&log, ARRAY_FREE_DOUBLE)[0][2], 32);
        assert_eq!(calls(&log, MAP_CLEAR_AND_DEALLOCATE).len(), 1);
        assert_eq!(calls(&log, MAP_DESTRUCT).len(), 1);
    }

    #[test]
    fn deserialize_entry_00dcde10_copies_the_saved_entry_for_the_simple_agent_types() {
        let mut e = engine();
        let log = new_log();
        copy_double(&mut e);
        log_double(&mut e, &log, DESERIALIZE_1N_TRACK, 5);
        let record = e.mem.alloc(0x100);
        fill(&mut e, record, 0x100, 0x30);
        let track = e.mem.alloc(0xc);
        for (agent, process) in PROCESSES[..4].iter().enumerate() {
            let context = context_with_agent(&mut e, 1, *process);
            for kind in [2u8, 4] {
                let entry = entry_of(&mut e, kind, 1, 0);
                let got = e.call(
                    0x00dc_de10,
                    &args![record, agent as u32 + 1, track, context, entry],
                );
                assert_eq!(got.u32(), 0);
                assert_eq!(
                    bytes(&e, entry + 0x18, 0x68),
                    bytes(&e, record + 0x8c, 0x68)
                );
                assert_eq!(e.mem.u8(entry + 2), e.mem.u8(record + 0x76));
                assert_eq!(e.mem.u8(entry + 3), e.mem.u8(record + 0x77));
            }
        }
        assert!(calls(&log, DESERIALIZE_1N_TRACK).is_empty());
    }

    #[test]
    fn deserialize_entry_00dcde10_refuses_entries_of_another_kind() {
        let mut e = engine();
        copy_double(&mut e);
        let record = e.mem.alloc(0x100);
        let context = context_with_agent(&mut e, 1, PROCESSES[0]);
        let unknown = context_with_agent(&mut e, 1, 0x00d3_fe91);
        // The entry type must be 2 or 4.
        for kind in [0u8, 1, 3, 5, 6] {
            let entry = entry_of(&mut e, kind, 1, 0);
            let got = e.call(0x00dc_de10, &args![record, 1u32, 0u32, context, entry]);
            assert_eq!(got.u32(), 1);
        }
        let entry = entry_of(&mut e, 2, 1, 0);
        // The agent type is another one, or the wanted kind is not one of the nine.
        assert_eq!(
            e.call(0x00dc_de10, &args![record, 2u32, 0u32, context, entry])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00dc_de10, &args![record, 0u32, 0u32, unknown, entry])
                .u32(),
            1
        );
        assert_eq!(bytes(&e, entry + 0x18, 4), vec![0; 4]);
    }

    #[test]
    fn deserialize_entry_00dcde10_takes_the_track_of_agent_type_five() {
        let mut e = engine();
        let log = new_log();
        copy_double(&mut e);
        log_double(&mut e, &log, DESERIALIZE_1N_TRACK, 0x66);
        let context = context_with_agent(&mut e, 1, PROCESSES[4]);
        let record = e.mem.alloc(0x100);
        let track = e.mem.alloc(0xc);
        let found = e.mem.alloc(0xc);
        fill(&mut e, found, 0xc, 0x77);
        let with_track = entry_of(&mut e, 4, 1, 0);
        let without_track = entry_of(&mut e, 2, 1, 0);
        let skipped = entry_of(&mut e, 4, 1, 0);
        // The record's payload copy lands at the entry's +0x18, so its byte at +0x23 becomes the
        // payload byte at +0xB.
        e.mem.set_u8(record + 0x8c + 0x23, 0x20);
        let lookup_log = Rc::clone(&log);
        e.register_double(GET_AGENT_1N_TRACK, move |_, a| {
            lookup_log
                .borrow_mut()
                .push((GET_AGENT_1N_TRACK, a.to_vec()));
            Ret {
                eax: if a[0] == with_track { found } else { 0 },
                ..Ret::default()
            }
        });
        // Bit 5 of the payload's byte at +0xB: no lookup.
        let got = e.call(0x00dc_de10, &args![record, 5u32, track, context, skipped]);
        assert_eq!(got.u32(), 0);
        assert!(calls(&log, GET_AGENT_1N_TRACK).is_empty());
        e.mem.set_u8(record + 0x8c + 0x23, 0);
        // The found track is initialised as an empty array and deserialized into.
        let got = e.call(
            0x00dc_de10,
            &args![record, 5u32, track, context, with_track],
        );
        assert_eq!(got.u32(), 0x66);
        assert_eq!(
            [e.mem.u32(found), e.mem.u32(found + 4), e.mem.u32(found + 8)],
            [0, 0, 0x8000_0000]
        );
        // Nothing found: the null track is passed on.
        let got = e.call(
            0x00dc_de10,
            &args![record, 5u32, track, context, without_track],
        );
        assert_eq!(got.u32(), 0x66);
        assert_eq!(
            calls(&log, GET_AGENT_1N_TRACK),
            vec![
                vec![with_track, with_track + 0x30],
                vec![without_track, without_track + 0x20]
            ]
        );
        assert_eq!(
            calls(&log, DESERIALIZE_1N_TRACK),
            vec![vec![track, context, found], vec![track, context, 0]]
        );
    }

    #[test]
    fn deserialize_entry_00dcde10_clears_the_payload_for_agent_types_six_to_nine() {
        let mut e = engine();
        let log = new_log();
        copy_double(&mut e);
        log_double(&mut e, &log, DESERIALIZE_1N_TRACK, 0x66);
        let record = e.mem.alloc(0x100);
        fill(&mut e, record, 0x100, 0x30);
        let track = e.mem.alloc(0xc);
        for (agent, process) in PROCESSES.iter().enumerate().skip(5) {
            let context = context_with_agent(&mut e, 1, *process);
            log.borrow_mut().clear();
            let entry = entry_of(&mut e, 2, 1, 0);
            fill(&mut e, entry + 0x20, 0xc, 0x77);
            let got = e.call(
                0x00dc_de10,
                &args![record, agent as u32 + 1, track, context, entry],
            );
            assert_eq!(got.u32(), 0x66);
            // The copy is overwritten by the empty array at the payload (`+0x20`, 12 bytes).
            assert_eq!(bytes(&e, entry + 0x18, 8), bytes(&e, record + 0x8c, 8));
            assert_eq!(
                bytes(&e, entry + 0x2c, 0x54),
                bytes(&e, record + 0x8c + 0x14, 0x54)
            );
            assert_eq!(e.mem.u8(entry + 2), 0);
            assert_eq!(
                [
                    e.mem.u32(entry + 0x20),
                    e.mem.u32(entry + 0x24),
                    e.mem.u32(entry + 0x28)
                ],
                [0, 0, 0x8000_0000]
            );
            assert_eq!(
                calls(&log, DESERIALIZE_1N_TRACK),
                vec![vec![track, context, entry + 0x20]]
            );
        }
    }

    /// A source with one item of 0x200 bytes holding two entries of 0x20 bytes after its
    /// header (the first word of the item is the byte count 0x40).
    fn one_item_source(e: &mut Engine) -> (u32, u32) {
        let item = e.mem.alloc(0x200);
        fill(e, item, 0x200, 0x20);
        e.mem.set_u32(item, 0x40);
        let items = e.mem.alloc(4);
        e.mem.set_u32(items, item);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, items);
        e.mem.set_u32(source + 4, 1);
        (source, item)
    }

    /// `serializeEntry` double of `00dcdf70`: stores the size `0x20` and, as the track it wrote,
    /// the entry plus 8; the first entry `kept` gets a nonzero second record word; the entry
    /// `bad` fails.
    fn entry_double(e: &mut Engine, log: &Log, kept: u32, bad: u32) {
        let log = Rc::clone(log);
        e.register_double(SERIALIZE_ENTRY, move |e, a| {
            log.borrow_mut().push((SERIALIZE_ENTRY, a.to_vec()));
            e.mem.set_u32(a[3], 0x20);
            e.mem.set_u32(a[5], a[0] + 8);
            if a[0] == kept {
                e.mem.set_u32(a[6] + 4, 1);
            }
            Ret {
                eax: u32::from(a[0] == bad),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn serialize_1n_track_00dcdf70_keeps_the_records_that_wrote_a_track() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        copy_double(&mut e);
        log_double(&mut e, &log, DESTRUCT_TRACK_RECORD, 0);
        let more_log = Rc::clone(&log);
        e.register_double(RESERVE_MORE, move |e, a| {
            more_log.borrow_mut().push((RESERVE_MORE, a.to_vec()));
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let (source, item) = one_item_source(&mut e);
        entry_double(&mut e, &log, item + 0x10, 0);
        let track = e.mem.alloc(0x20);
        // The exe's 0x28-byte frame: the track word +0x14, the size +0x18, the kind +0x24.
        let frame = e.next_stack_local(0x28);
        let got = e.call(0x00dc_df70, &args![source, 0x99u32, track]).u32();
        assert_eq!(got, 0);
        let serialized = calls(&log, SERIALIZE_ENTRY);
        assert_eq!(
            serialized[0][3..6],
            [frame + 0x18, frame + 0x24, frame + 0x14]
        );
        assert_eq!(
            serialized
                .iter()
                .map(|call| (call[0], call[1], call[2]))
                .collect::<Vec<_>>(),
            vec![(item + 0x10, 0, 0x99), (item + 0x30, 0, 0x99)]
        );
        let records: Vec<u32> = serialized.iter().map(|call| call[6]).collect();
        assert_eq!(
            calls(&log, ALLOCATE_DOUBLE),
            vec![
                vec![allocator, 0x20],
                vec![allocator, 0x20],
                vec![allocator, 0x200]
            ]
        );
        // The first record is kept: item index 0 and the offset 8 of its written track.
        assert_eq!(e.mem.u32(records[0] + 0x18), 0);
        assert_eq!(e.mem.u32(records[0] + 0x1c), 8);
        assert_eq!(e.mem.u32(track + 0x10), 1);
        assert_eq!(e.mem.u32(e.mem.u32(track + 0xc)), records[0]);
        assert_eq!(
            calls(&log, RESERVE_MORE),
            vec![vec![allocator, track + 0xc, 4]]
        );
        // The second one is destroyed and freed again, with its initial words.
        assert_eq!(calls(&log, DESTRUCT_TRACK_RECORD), vec![vec![records[1]]]);
        assert_eq!(
            calls(&log, FREE_SLOT_DOUBLE),
            vec![vec![allocator, records[1], 0x20]]
        );
        assert_eq!(
            (0..7)
                .map(|word| e.mem.u32(records[1] + 4 * word))
                .collect::<Vec<_>>(),
            vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000, 0xffff_ffff]
        );
        // The item is kept as a block copy.
        assert_eq!(e.mem.u32(track + 4), 1);
        let block = e.mem.u32(e.mem.u32(track));
        assert_eq!(bytes(&e, block, 0x200), bytes(&e, item, 0x200));
        assert_eq!(calls(&log, RESERVE_ARRAY).len(), 1);
    }

    #[test]
    fn serialize_1n_track_00dcdf70_stops_at_the_first_entry_that_fails() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        log_double(&mut e, &log, DESTRUCT_TRACK_RECORD, 0);
        let (source, item) = one_item_source(&mut e);
        entry_double(&mut e, &log, 0, item + 0x10);
        let track = e.mem.alloc(0x20);
        let got = e.call(0x00dc_df70, &args![source, 0x99u32, track]).u32();
        assert_eq!(got, 1);
        assert_eq!(calls(&log, SERIALIZE_ENTRY).len(), 1);
        let record = calls(&log, SERIALIZE_ENTRY)[0][6];
        assert_eq!(calls(&log, DESTRUCT_TRACK_RECORD), vec![vec![record]]);
        assert_eq!(
            calls(&log, FREE_SLOT_DOUBLE),
            vec![vec![allocator, record, 0x20]]
        );
        assert_eq!(e.mem.u32(track + 4), 0);
    }

    #[test]
    fn serialize_1n_track_00dcdf70_has_nothing_to_do_for_an_empty_source() {
        let mut e = engine();
        let log = new_log();
        router(&mut e, &log);
        reserve_double(&mut e, &log);
        let source = e.mem.alloc(8);
        let track = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_df70, &args![source, 0u32, track]).u32(), 0);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(track + 4), 0);
    }

    const POINT_DOUBLE: u32 = 0x0900_0050;
    const PROPERTIES_DOUBLE: u32 = 0x0900_0051;
    const IDS_DOUBLE: u32 = 0x0900_0052;

    /// A double that logs its words and, behind them, the words `extra` reads at the time of
    /// the call.
    fn snapshot_double(
        e: &mut Engine,
        log: &Log,
        address: u32,
        extra: fn(&Engine, &[u32]) -> Vec<u32>,
    ) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            words.extend(extra(e, a));
            log.borrow_mut().push((address, words));
            Ret::default()
        });
    }

    /// The first thirteen words of the event at the second argument.
    fn event_words(e: &Engine, a: &[u32]) -> Vec<u32> {
        (0..13).map(|word| e.mem.u32(a[1] + 4 * word)).collect()
    }

    /// The counter and the flag byte of the world at the first argument.
    fn world_state(e: &Engine, a: &[u32]) -> Vec<u32> {
        vec![e.mem.u32(a[0] + 0x94), u32::from(e.mem.u8(a[0] + 0x9c))]
    }

    /// The four words of the constraint info at the third argument.
    fn info_words(e: &Engine, a: &[u32]) -> Vec<u32> {
        (0..4).map(|word| e.mem.u32(a[2] + 4 * word)).collect()
    }

    /// What `deserializeCollisionEntry` works on: both bodies are in the same island `0x1a1a`
    /// and the world `world`; the agent's first collidable is the one of `body_a`.
    struct Scene {
        allocator: u32,
        options: u32,
        saved: u32,
        body_a: u32,
        body_b: u32,
        agent: u32,
        object: u32,
        owner: u32,
        old_atoms: u32,
        new_atoms: u32,
        counters: u32,
        other_counters: u32,
        world: u32,
        points: u32,
        properties: u32,
    }

    /// The scene with the doubles of every callee. `destroy` and `entry_answer` are what
    /// `destroyOldEntry` and `deserializeEntry` answer and `ids_capacity` is the capacity word of
    /// the contact point id array the object fills.
    fn scene(
        e: &mut Engine,
        log: &Log,
        destroy: u32,
        entry_answer: u32,
        ids_capacity: u32,
    ) -> Scene {
        let allocator = router(e, log);
        reserve_double(e, log);
        copy_double(e);
        log_double(e, log, DESTROY_OLD_ENTRY, destroy);
        log_double(e, log, DESERIALIZE_ENTRY, entry_answer);
        log_double(e, log, GET_ISLAND_FROM_AGENT_ENTRY, 0x1517);
        log_double(e, log, ATOM_SIZE_ROUND_UP, 0x40);
        log_double(e, log, INVALIDATE_TIM_IN_AGENT, 0);
        snapshot_double(e, log, ADD_CONSTRAINT_IMMEDIATELY, world_state);
        snapshot_double(e, log, ADD_CONSTRAINT_INFO, info_words);
        for address in [
            FIRE_CONTACT_POINT_ADDED,
            FIRE_CONTACT_POINT_ADDED_INTERNAL,
            FIRE_CONTACT_POINT_CALLBACK,
            FIRE_CONTACT_POINT_CALLBACK_INTERNAL,
        ] {
            snapshot_double(e, log, address, event_words);
        }
        let new_atoms = e.mem.alloc(0x400);
        e.mem.set_u16(new_atoms + 2, 0x50);
        e.mem.set_u16(new_atoms + 6, 1);
        e.mem.set_u8(new_atoms + 0xa, 3);
        log_double(e, log, ALLOCATE_ATOM, new_atoms);
        let points = e.mem.alloc(0x200);
        e.mem.set_u8(points + 5 * 0x20 + 0xf, 1);
        let properties = e.mem.alloc(0x100);
        e.register_double(POINT_DOUBLE, move |_, a| Ret {
            eax: points + a[1] * 0x20,
            ..Ret::default()
        });
        e.register_double(PROPERTIES_DOUBLE, move |_, a| Ret {
            eax: properties + a[1] * 0x10,
            ..Ret::default()
        });
        let ids_log = Rc::clone(log);
        e.register_double(IDS_DOUBLE, move |e, a| {
            ids_log.borrow_mut().push((IDS_DOUBLE, a.to_vec()));
            let data = e.mem.alloc(8);
            e.mem.set_u16(data, 3);
            e.mem.set_u16(data + 2, 5);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, ids_capacity);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x28, POINT_DOUBLE);
        e.mem.set_u32(vtable + 0x2c, PROPERTIES_DOUBLE);
        e.mem.set_u32(vtable + 0x30, IDS_DOUBLE);
        let counters = e.mem.alloc(0x20);
        let other_counters = e.mem.alloc(0x20);
        for block in [counters, other_counters] {
            e.mem.set_u32(block + 0xc, 0x1000);
            e.mem.set_u32(block + 0x10, 50);
            e.mem.set_u32(block + 0x14, 60);
        }
        let owner = e.mem.alloc(0x40);
        for (slot, block) in [(4, counters), (8, other_counters)] {
            let island = e.mem.alloc(0xd4);
            e.mem.set_u32(island + 0xcc, block);
            e.mem.set_u32(owner + slot, island);
        }
        e.mem.set_u16(owner + 0x14, 0x400);
        e.mem.set_u16(owner + 0x16, 100);
        e.mem.set_u16(owner + 0x18, 100);
        let old_atoms = e.mem.alloc(0x100);
        e.mem.set_u16(old_atoms + 2, 0x30);
        e.mem.set_u16(old_atoms + 4, 3);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x3c, old_atoms);
        e.mem.set_u32(object + 0x74, owner);
        e.mem.set_u32(object + 0x58, 0x58a);
        e.mem.set_u32(object + 0x5c, 0x5ca);
        e.mem.set_u16(object + 0x12, 0x77);
        let world = e.mem.alloc(0x100);
        let mut bodies = [0; 2];
        for body in &mut bodies {
            *body = e.mem.alloc(0x300);
            e.mem.set_u32(*body + 8, world);
            e.mem.set_u32(*body + 0xcc, 0x1a1a);
            e.mem.set_u8(*body + 0x20, 0xf0);
            e.mem.set_u8(*body + 0x28, 1);
        }
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent + 8, object);
        e.mem.set_u32(agent + 0x10, bodies[0] + 0x10);
        e.mem.set_u32(agent + 0x14, bodies[1] + 0x10);
        e.mem.set_u16(agent + 4, 0x44);
        e.mem.set_u16(agent + 6, 0x66);
        let saved = e.mem.alloc(0x200);
        e.mem.set_u8(saved + 0x19, 0xfe);
        e.mem.set_u16(saved + 0x24, 2);
        e.mem.set_u8(saved + 0x28, 4);
        e.mem.set_u8(saved + 0x29, 5);
        e.mem.set_u16(saved + 0x2c, 6);
        fill(e, saved + 0x30, 0x20, 0x11);
        for (offset, length, first) in [(0x50, 8, 0xa0), (0x5c, 0x40, 0xb0), (0x68, 8, 0xc0)] {
            let block = e.mem.alloc(length);
            fill(e, block, length, first);
            e.mem.set_u32(saved + offset, block);
        }
        e.mem.set_u32(saved + 0x6c, 3);
        Scene {
            allocator,
            options: e.mem.alloc(0x10),
            saved,
            body_a: bodies[0],
            body_b: bodies[1],
            agent,
            object,
            owner,
            old_atoms,
            new_atoms,
            counters,
            other_counters,
            world,
            points,
            properties,
        }
    }

    /// Sets the option bytes at `+1`, `+8`, `+9` and `+0xA`.
    fn set_options(e: &mut Engine, s: &Scene, flags: [u8; 4]) {
        for (offset, flag) in [1, 8, 9, 0xa].into_iter().zip(flags) {
            e.mem.set_u8(s.options + offset, flag);
        }
    }

    fn deserialize_collision(e: &mut Engine, s: &Scene) -> u32 {
        e.call(
            0x00dc_e180,
            &args![s.options, s.saved, s.body_a, s.body_b, 0xc0deu32, s.agent],
        )
        .u32()
    }

    fn counter_words(e: &Engine, counters: u32) -> [u32; 3] {
        [
            e.mem.u32(counters + 0xc),
            e.mem.u32(counters + 0x10),
            e.mem.u32(counters + 0x14),
        ]
    }

    #[test]
    fn deserialize_collision_entry_00dce180_rebuilds_the_atoms_and_fires_the_events() {
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 0, 0, 4);
        set_options(&mut e, &s, [1, 1, 1, 1]);
        e.mem.set_u16(s.body_a + 0x204, 1);
        // The constraint info at +0x28 of the exe's aligned frame.
        let block = e.next_stack_local(COLLISION_FRAME + 0x10);
        let frame = (block + 0xf) & !0xf;
        assert_eq!(deserialize_collision(&mut e, &s), 0);
        assert_eq!(calls(&log, ADD_CONSTRAINT_INFO)[0][2], frame + 0x28);
        assert_eq!(
            calls(&log, DESTROY_OLD_ENTRY),
            vec![vec![0xffff_fffe, 0xc0de, s.object, 0x1a1a, s.agent]]
        );
        assert!(calls(&log, GET_ISLAND_FROM_AGENT_ENTRY).is_empty());
        assert_eq!(
            calls(&log, DESERIALIZE_ENTRY),
            vec![vec![s.saved, 0xffff_fffe, s.saved + 0x114, 0xc0de, s.agent]]
        );
        // The saved array becomes the object's array.
        assert_eq!(e.mem.u32(s.object + 0x24), 3);
        assert_eq!(
            bytes(&e, e.mem.u32(s.object + 0x20), 3),
            bytes(&e, e.mem.u32(s.saved + 0x68), 3)
        );
        assert_eq!(
            calls(&log, RESERVE_ARRAY),
            vec![vec![s.allocator, s.object + 0x20, 3, 1]]
        );
        // Three old atoms: 0x140 bytes, 6 elements and 7 temporaries come off the counters.
        assert!(calls(&log, ADD_CONSTRAINT_IMMEDIATELY).is_empty());
        assert_eq!(counter_words(&e, s.counters), [0xec0, 44, 53]);
        assert_eq!(counter_words(&e, s.other_counters), [0x1000, 50, 60]);
        assert_eq!(e.mem.u16(s.owner + 0x14), 0x2c0);
        assert_eq!(e.mem.u16(s.owner + 0x16), 94);
        assert_eq!(e.mem.u16(s.owner + 0x18), 93);
        assert_eq!(calls(&log, ATOM_SIZE_ROUND_UP), vec![vec![0x30]]);
        assert_eq!(
            calls(&log, FREE_SLOT_DOUBLE),
            vec![vec![s.allocator, s.old_atoms, 0x40]]
        );
        // The new atoms.
        assert_eq!(calls(&log, ALLOCATE_ATOM), vec![vec![2, 4, 5, 6]]);
        assert_eq!(e.mem.u32(s.object + 0x3c), s.new_atoms);
        assert_eq!(e.mem.u16(s.new_atoms + 4), 2);
        assert_eq!(
            bytes(&e, s.new_atoms + 0x10, 0x20),
            bytes(&e, s.saved + 0x30, 0x20)
        );
        assert_eq!(e.mem.u32(s.owner + 0xc), s.new_atoms);
        assert_eq!(e.mem.u16(s.owner + 0x10), 0x50);
        let info = calls(&log, ADD_CONSTRAINT_INFO);
        assert_eq!(info.len(), 1);
        assert_eq!(info[0][..2], [s.counters, s.object + 0x44]);
        assert_eq!(info[0][3..], [0x110, 0x110, 5, 6]);
        // The contact bytes and points are copied (the points over the end of the bytes) and the
        // third word of every contact is cleared.
        let points = bytes(&e, e.mem.u32(s.saved + 0x5c), 0x40);
        let expected: Vec<u8> = (0..16)
            .map(|index| {
                if (8..15).contains(&index) {
                    0
                } else {
                    points[0x20 + index]
                }
            })
            .collect();
        assert_eq!(bytes(&e, s.new_atoms + 0x50, 16), expected);
        assert_eq!(bytes(&e, s.new_atoms + 0x30, 0x20), points[..0x20].to_vec());
        // The events of the two contact points (3 and 5).
        let added = calls(&log, FIRE_CONTACT_POINT_ADDED);
        assert_eq!(added.len(), 2);
        for (call, id) in added.iter().zip([3u32, 5]) {
            assert_eq!(call[0], s.world);
            let expected = [
                s.body_a + 0x10,
                s.body_b + 0x10,
                1,
                0,
                s.properties + id * 0x10,
                0,
                s.points + id * 0x20,
                0,
                0,
                s.object,
                0xc0de,
            ];
            assert_eq!(call[2..13], expected);
            assert_eq!(call[14], id | (0x77 << 16));
        }
        let internal = calls(&log, FIRE_CONTACT_POINT_ADDED_INTERNAL);
        assert_eq!(internal.len(), 2);
        assert!(internal.iter().all(|call| call[0] == s.body_a));
        // Only contact point 3 has an even flag byte and gets the callbacks.
        let callbacks = calls(&log, FIRE_CONTACT_POINT_CALLBACK);
        assert_eq!(callbacks.len(), 1);
        let call = &callbacks[0];
        assert_eq!(call[0], s.world);
        assert_eq!(call[2..7], [2, s.body_a, s.body_b, s.object, 4]);
        assert_eq!(call[7..9], [s.properties + 0x30, s.points + 0x60]);
        assert_eq!(call[9] & 0x00ff_ffff, 0);
        assert_eq!(call[10..12], [0, 0]);
        assert_eq!(call[12], s.points + 0x60 + 0x14);
        assert_eq!(call[13..15], [0, 0]);
        let callbacks = calls(&log, FIRE_CONTACT_POINT_CALLBACK_INTERNAL);
        assert_eq!(callbacks.len(), 1);
        assert_eq!(callbacks[0][0], s.body_a);
        assert_eq!(callbacks[0][2], 0);
        assert_eq!(
            calls(&log, INVALIDATE_TIM_IN_AGENT),
            vec![vec![s.agent, 0xc0de]]
        );
        // The id array was owned (capacity 4 of 2-byte ids).
        let freed = calls(&log, ARRAY_FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][0], s.allocator);
        assert_eq!(freed[0][2], 8);
    }

    #[test]
    fn deserialize_collision_entry_00dce180_leaves_out_what_the_options_switch_off() {
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 0, 0, 0x8000_0004);
        e.mem.set_u16(s.body_b + 0x204, 1);
        // Options 8 and 9 on, the body words (option 0xA) and the clearing (option 1) off.
        set_options(&mut e, &s, [0, 1, 1, 0]);
        assert_eq!(deserialize_collision(&mut e, &s), 0);
        let added = calls(&log, FIRE_CONTACT_POINT_ADDED);
        assert_eq!(added.len(), 2);
        assert_eq!(added[0][2..4], [0, 0]);
        assert_eq!(
            calls(&log, FIRE_CONTACT_POINT_ADDED_INTERNAL)
                .iter()
                .map(|call| call[0])
                .collect::<Vec<_>>(),
            vec![s.body_b, s.body_b]
        );
        let callbacks = calls(&log, FIRE_CONTACT_POINT_CALLBACK_INTERNAL);
        assert_eq!(callbacks.len(), 1);
        assert_eq!((callbacks[0][0], callbacks[0][2]), (s.body_b, 1));
        // Nothing was cleared: the bytes after the copied contacts are the saved points.
        let points = bytes(&e, e.mem.u32(s.saved + 0x5c), 0x40);
        assert_eq!(
            bytes(&e, s.new_atoms + 0x50, 16),
            points[0x20..0x30].to_vec()
        );
        // The id array was not owned: it is not freed.
        assert!(calls(&log, ARRAY_FREE_DOUBLE).is_empty());
        // Everything off: no event at all.
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 0, 0, 4);
        set_options(&mut e, &s, [0, 0, 0, 0]);
        assert_eq!(deserialize_collision(&mut e, &s), 0);
        for address in [
            FIRE_CONTACT_POINT_ADDED,
            FIRE_CONTACT_POINT_ADDED_INTERNAL,
            FIRE_CONTACT_POINT_CALLBACK,
            FIRE_CONTACT_POINT_CALLBACK_INTERNAL,
        ] {
            assert!(calls(&log, address).is_empty());
        }
        assert_eq!(calls(&log, ARRAY_FREE_DOUBLE).len(), 1);
    }

    #[test]
    fn deserialize_collision_entry_00dce180_swaps_the_sides_when_the_agent_has_them_the_other_way()
    {
        for owner_byte in [0u8, 1] {
            let mut e = engine();
            let log = new_log();
            let s = scene(&mut e, &log, 0, 0, 4);
            set_options(&mut e, &s, [0, 0, 0, 0]);
            // The agent holds the bodies the other way round.
            e.mem.set_u32(s.agent + 0x10, s.body_b + 0x10);
            e.mem.set_u32(s.agent + 0x14, s.body_a + 0x10);
            e.mem.set_u8(s.owner + 0x1a, owner_byte);
            if owner_byte == 1 {
                let (first, second) = (e.mem.u32(s.owner + 4), e.mem.u32(s.owner + 8));
                e.mem.set_u32(s.owner + 4, second);
                e.mem.set_u32(s.owner + 8, first);
            }
            let (owner_a, owner_b) = (e.mem.u32(s.owner + 4), e.mem.u32(s.owner + 8));
            assert_eq!(deserialize_collision(&mut e, &s), 0);
            assert_eq!(e.mem.u32(s.agent + 0x10), s.body_a + 0x10);
            assert_eq!(e.mem.u32(s.agent + 0x14), s.body_b + 0x10);
            assert_eq!(
                (e.mem.u16(s.agent + 4), e.mem.u16(s.agent + 6)),
                (0x66, 0x44)
            );
            assert_eq!(
                (e.mem.u32(s.object + 0x58), e.mem.u32(s.object + 0x5c)),
                (0x5ca, 0x58a)
            );
            assert_eq!(
                (e.mem.u32(s.owner + 4), e.mem.u32(s.owner + 8)),
                (owner_b, owner_a)
            );
            assert_eq!(e.mem.u8(s.owner + 0x1a), 1 - owner_byte);
            // The same island's counters are used as without the swap.
            assert_eq!(counter_words(&e, s.counters), [0xec0, 44, 53]);
            assert_eq!(counter_words(&e, s.other_counters), [0x1000, 50, 60]);
            assert_eq!(calls(&log, ADD_CONSTRAINT_INFO)[0][0], s.counters);
        }
    }

    #[test]
    fn deserialize_collision_entry_00dce180_picks_the_island_of_the_agent() {
        // (island of body a, of body b, kind byte of a, of b, expected island)
        let cases = [
            (0x1a1a, 0x1a1a, 0, 0, 0x1a1a),
            (0x1a1a, 0x2b2b, 5, 0, 0x2b2b),
            (0x1a1a, 0x2b2b, 0, 5, 0x1a1a),
            (0x1a1a, 0x2b2b, 5, 5, 0x2b2b),
            (0x1a1a, 0x2b2b, 0, 0, 0x1517),
        ];
        for (island_a, island_b, kind_a, kind_b, expected) in cases {
            let mut e = engine();
            let log = new_log();
            let s = scene(&mut e, &log, 0, 0, 4);
            set_options(&mut e, &s, [0, 0, 0, 0]);
            e.mem.set_u32(s.body_a + 0xcc, island_a);
            e.mem.set_u32(s.body_b + 0xcc, island_b);
            e.mem.set_u8(s.body_a + 0xe8, kind_a);
            e.mem.set_u8(s.body_b + 0xe8, kind_b);
            assert_eq!(deserialize_collision(&mut e, &s), 0);
            assert_eq!(calls(&log, DESTROY_OLD_ENTRY)[0][3], expected);
            let asked = calls(&log, GET_ISLAND_FROM_AGENT_ENTRY);
            if expected == 0x1517 {
                assert_eq!(asked, vec![vec![s.agent, island_a, island_b]]);
            } else {
                assert!(asked.is_empty());
            }
        }
    }

    #[test]
    fn deserialize_collision_entry_00dce180_adds_the_constraint_when_there_are_no_atoms() {
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 0, 0, 4);
        set_options(&mut e, &s, [0, 0, 0, 0]);
        e.mem.set_u16(s.old_atoms + 4, 0);
        e.mem.set_u32(s.world + 0x94, 5);
        assert_eq!(deserialize_collision(&mut e, &s), 0);
        // The world's counter was lowered and its flag set during the call.
        assert_eq!(
            calls(&log, ADD_CONSTRAINT_IMMEDIATELY),
            vec![vec![s.world, s.object + 0x44, 1, 4, 1]]
        );
        assert_eq!(e.mem.u32(s.world + 0x94), 5);
        assert_eq!(e.mem.u8(s.world + 0x9c), 0);
        // No atoms: 0x80 bytes, 2 elements and 3 temporaries come off the counters.
        assert_eq!(counter_words(&e, s.counters), [0xf80, 48, 57]);
        assert_eq!(e.mem.u16(s.owner + 0x14), 0x380);
        assert_eq!(e.mem.u16(s.owner + 0x16), 98);
        assert_eq!(e.mem.u16(s.owner + 0x18), 97);
    }

    #[test]
    fn deserialize_collision_entry_00dce180_stops_when_an_entry_function_fails() {
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 1, 0, 4);
        assert_eq!(deserialize_collision(&mut e, &s), 1);
        assert!(calls(&log, DESERIALIZE_ENTRY).is_empty());
        assert!(calls(&log, ALLOCATE_ATOM).is_empty());
        let mut e = engine();
        let log = new_log();
        let s = scene(&mut e, &log, 0, 1, 4);
        assert_eq!(deserialize_collision(&mut e, &s), 1);
        assert_eq!(calls(&log, DESERIALIZE_ENTRY).len(), 1);
        assert!(calls(&log, ALLOCATE_ATOM).is_empty());
        assert!(calls(&log, ARRAY_FREE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(s.object + 0x3c), s.old_atoms);
    }

    #[test]
    fn load_contact_points_00dce7b0_hands_a_selector_to_the_internal_loader() {
        let mut e = engine();
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(LOAD_CONTACT_POINTS_INTERNAL, move |e, a| {
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[3]));
            seen.borrow_mut()
                .push((LOAD_CONTACT_POINTS_INTERNAL, words));
            Ret::default()
        });
        // The selector is the exe's whole 4-byte frame.
        let frame = e.next_stack_local(4);
        e.call(0x00dc_e7b0, &args![0x11u32, 0x22u32, 0x33u32]);
        let loaded = calls(&log, LOAD_CONTACT_POINTS_INTERNAL);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0][3], frame);
        assert_eq!(loaded[0][..3], [0x11, 0x22, 0x33]);
        assert_eq!(loaded[0][4], LOAD_SELECTOR_VTABLE);
    }

    #[test]
    fn entity_selector_is_entity_ok_00dce8d0_looks_the_entity_up_in_its_map() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, MAP_GET_WITH_DEFAULT, 7);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dc_e8d0, &args![this, 0x1234u32]).u32(), 7);
        assert_eq!(
            calls(&log, MAP_GET_WITH_DEFAULT),
            vec![vec![this + 4, 0x1234, 0]]
        );
    }

    #[test]
    fn entity_selector_listed_destructor_00dce8f0_clears_the_map_and_frees_on_bit_zero() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        for address in [MAP_CLEAR_AND_DEALLOCATE, MAP_DESTRUCT, FREE] {
            log_double(&mut e, &log, address, 0);
        }
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00dc_e8f0, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ENTITY_SELECTOR_VTABLE);
        assert_eq!(
            calls(&log, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![this + 4, allocator]]
        );
        assert_eq!(calls(&log, MAP_DESTRUCT), vec![vec![this + 4]]);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(e.call(0x00dc_e8f0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&log, FREE), vec![vec![this]]);
        assert_eq!(calls(&log, MAP_DESTRUCT).len(), 2);
    }

    #[test]
    fn record_constructor_00dce930_sets_the_vtable_and_empty_arrays() {
        let mut e = engine();
        let this = e.mem.alloc(0x140);
        for offset in 0..0x140 {
            e.mem.set_u8(this + offset, 0xff);
        }
        assert_eq!(e.call(0x00dc_e930, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), RECORD_VTABLE);
        assert_eq!(
            (e.mem.u32(this + 0x10), e.mem.u32(this + 0x14)),
            (u32::MAX, u32::MAX)
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (0, 0));
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u16(this + 0x20), 0x16);
        assert_eq!(e.mem.u16(this + 0x30), 0);
        assert_eq!(e.mem.u16(this + 0x32), 3);
        for offset in (0x34..0x58).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for array in [0x50, 0x5c, 0x68, 0x114, 0x120] {
            assert_eq!(
                [
                    e.mem.u32(this + array),
                    e.mem.u32(this + array + 4),
                    e.mem.u32(this + array + 8)
                ],
                [0, 0, 0x8000_0000]
            );
        }
        assert_eq!(
            (e.mem.u32(this + 0x12c), e.mem.u32(this + 0x130)),
            (0x103, 1)
        );
        // The words it does not write keep what they had.
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 0x1c), u32::MAX);
        assert_eq!(e.mem.u32(this + 0x134), u32::MAX);
    }

    /// The world of the loader: context `0xc0de` at `+0x78` and a group of the given entries.
    fn loader(e: &mut Engine, entries: &[u32]) -> (u32, u32) {
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, 0xc0de);
        let list = e.mem.alloc(4 * entries.len().max(1) as u32);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *entry);
        }
        let group = e.mem.alloc(0x50);
        e.mem.set_u32(group + 0x44, list);
        e.mem.set_u32(group + 0x48, entries.len() as u32);
        (world, group)
    }

    /// An entry (0x140 bytes) holding the two bodies directly (or, with `mapped`, as ids).
    fn saved_entry(e: &mut Engine, first: u32, second: u32, mapped: bool, swapped: bool) -> u32 {
        let entry = e.mem.alloc(0x140);
        if mapped {
            e.mem.set_u8(entry + 0x18, 1);
            e.mem.set_u32(entry + 0x10, first);
            e.mem.set_u32(entry + 0x14, second);
        } else {
            e.mem.set_u32(entry + 8, first);
            e.mem.set_u32(entry + 0xc, second);
        }
        e.mem
            .set_u32(entry + 0x12c, if swapped { 0 } else { 0x103 });
        entry
    }

    /// The doubles of the loader: the selector accepts the bodies `accepted`, `FindAgent` finds
    /// an agent (the first collidable plus 1) for the first collidables in `agent_for`, and the entry loader answers 1 (failure)
    /// for the entries `failing`. Returns the selector.
    fn loader_doubles(
        e: &mut Engine,
        log: &Log,
        accepted: &'static [u32],
        agent_for: &'static [u32],
        failing: Vec<u32>,
    ) -> u32 {
        log_double(e, log, REMOVE_REFERENCE, 0);
        log_double(e, log, SWAP_ENDIAN_COLLISION_ENTRY, 0);
        log_double(e, log, WORLD_FLUSH_A, 0);
        log_double(e, log, WORLD_FLUSH_B, 0);
        e.register_double(MAP_FUNCTION_DOUBLE, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let select_log = Rc::clone(log);
        e.register_double(SELECT_DOUBLE, move |_, a| {
            select_log.borrow_mut().push((SELECT_DOUBLE, a.to_vec()));
            Ret {
                eax: u32::from(accepted.contains(&a[1])),
                ..Ret::default()
            }
        });
        let find_log = Rc::clone(log);
        e.register_double(FIND_AGENT, move |_, a| {
            find_log.borrow_mut().push((FIND_AGENT, a.to_vec()));
            Ret {
                eax: if agent_for.contains(&a[0]) {
                    a[0] + 1
                } else {
                    0
                },
                ..Ret::default()
            }
        });
        let load_log = Rc::clone(log);
        e.register_double(DESERIALIZE_COLLISION_ENTRY, move |_, a| {
            load_log
                .borrow_mut()
                .push((DESERIALIZE_COLLISION_ENTRY, a.to_vec()));
            Ret {
                eax: u32::from(failing.contains(&a[1])),
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, SELECT_DOUBLE);
        let selector = e.mem.alloc(4);
        e.mem.set_u32(selector, vtable);
        selector
    }

    fn load_internal(e: &mut Engine, converter: u32, group: u32, world: u32, selector: u32) {
        e.call(0x00dc_ea10, &args![converter, group, world, selector]);
    }

    #[test]
    fn load_contact_points_internal_00dcea10_removes_the_entries_that_fail_from_the_last() {
        let mut e = engine();
        let log = new_log();
        let first = saved_entry(&mut e, 0x100, 0x200, false, true);
        let second = saved_entry(&mut e, 0x300, 0x400, true, false);
        let third = saved_entry(&mut e, 0x500, 0x600, false, false);
        let (world, group) = loader(&mut e, &[first, second, third]);
        // Every body is accepted, every pair has an agent, the first two entries fail.
        let accepted: &'static [u32] = &[0x100, 0x1300, 0x500];
        let selector = loader_doubles(
            &mut e,
            &log,
            accepted,
            &[0x110, 0x1310, 0x510],
            vec![first, second],
        );
        let converter = e.mem.alloc(8);
        e.mem.set_u8(converter, 1);
        e.mem.set_u32(converter + 4, MAP_FUNCTION_DOUBLE);
        e.mem.set_u8(world + 0x9c, 0);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        load_internal(&mut e, converter, group, world, selector);
        // Entries are visited from the last: the order of the deserializer's calls.
        let loaded = calls(&log, DESERIALIZE_COLLISION_ENTRY);
        assert_eq!(
            loaded,
            vec![
                vec![converter, third, 0x500, 0x600, 0xc0de, 0x511],
                vec![converter, second, 0x1300, 0x1400, 0xc0de, 0x1311],
                vec![converter, first, 0x100, 0x200, 0xc0de, 0x111],
            ]
        );
        // The endian swap only for the entry whose version word is not 0x103.
        assert_eq!(
            calls(&log, SWAP_ENDIAN_COLLISION_ENTRY),
            vec![vec![0xc0de, first]]
        );
        // The failed ones lose a reference and the last one moves into their place.
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![second], vec![first]]
        );
        assert_eq!(e.mem.u32(group + 0x48), 1);
        let list = e.mem.u32(group + 0x44);
        assert_eq!(
            [e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)],
            [third, third, third]
        );
        assert_eq!(e.mem.u32(world + 0x94), 0);
        assert_eq!(calls(&log, WORLD_FLUSH_A), vec![vec![world]]);
        assert_eq!(calls(&log, WORLD_FLUSH_B), vec![vec![world]]);
    }

    #[test]
    fn load_contact_points_internal_00dcea10_skips_entries_without_bodies_selection_or_agent() {
        let mut e = engine();
        let log = new_log();
        let nobody = saved_entry(&mut e, 0, 0x200, false, false);
        let rejected = saved_entry(&mut e, 0x300, 0x400, false, false);
        let unknown = saved_entry(&mut e, 0x500, 0x600, false, false);
        let found = saved_entry(&mut e, 0x700, 0x800, false, true);
        let (world, group) = loader(&mut e, &[nobody, rejected, unknown, found]);
        // Only the second body of the last entry and the first of the third are accepted; an
        // agent exists for the last entry alone. The counter starts at 2: no flush at the end.
        let accepted: &'static [u32] = &[0x500, 0x800];
        let selector = loader_doubles(&mut e, &log, accepted, &[0x710], vec![found]);
        let converter = e.mem.alloc(8);
        e.mem.set_u32(world + 0x94, 2);
        e.mem.set_u32(world + 0x8c, 1);
        load_internal(&mut e, converter, group, world, selector);
        assert_eq!(
            calls(&log, SELECT_DOUBLE)
                .iter()
                .map(|call| call[1])
                .collect::<Vec<_>>(),
            vec![0x700, 0x800, 0x500, 0x300, 0x400]
        );
        assert_eq!(
            calls(&log, FIND_AGENT),
            vec![vec![0x710, 0x810], vec![0x510, 0x610]]
        );
        assert_eq!(
            calls(&log, SWAP_ENDIAN_COLLISION_ENTRY),
            vec![vec![0xc0de, found]]
        );
        assert_eq!(
            calls(&log, DESERIALIZE_COLLISION_ENTRY),
            vec![vec![converter, found, 0x700, 0x800, 0xc0de, 0x711]]
        );
        // The converter does not drop failed entries.
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(group + 0x48), 4);
        assert_eq!(e.mem.u32(world + 0x94), 2);
        assert!(calls(&log, WORLD_FLUSH_A).is_empty());
        assert!(calls(&log, WORLD_FLUSH_B).is_empty());
    }

    #[test]
    fn load_contact_points_internal_00dcea10_flushes_the_world_when_the_counter_returns_to_zero() {
        let mut e = engine();
        let log = new_log();
        let (world, group) = loader(&mut e, &[]);
        let selector = loader_doubles(&mut e, &log, &[], &[], Vec::new());
        let converter = e.mem.alloc(8);
        // (word +0x8C, +0xA4, +0x90, byte +0x9C) and the flushes expected (A, B).
        let cases = [
            (1, 1, 1, 0, true, true),
            (0, 1, 1, 0, false, true),
            (1, 0, 1, 0, true, false),
            (1, 2, 1, 0, true, false),
            (1, 1, 0, 0, true, false),
            (1, 1, 1, 1, false, false),
        ];
        for (flush_a, mode, flush_b, busy, expect_a, expect_b) in cases {
            log.borrow_mut().clear();
            e.mem.set_u32(world + 0x8c, flush_a);
            e.mem.set_u32(world + 0xa4, mode);
            e.mem.set_u32(world + 0x90, flush_b);
            e.mem.set_u8(world + 0x9c, busy);
            load_internal(&mut e, converter, group, world, selector);
            assert_eq!(calls(&log, WORLD_FLUSH_A).len(), usize::from(expect_a));
            assert_eq!(calls(&log, WORLD_FLUSH_B).len(), usize::from(expect_b));
            assert_eq!(e.mem.u32(world + 0x94), 0);
        }
    }
}
