//! `Havok SDK/hkpcachingshapephantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_ce60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4d310(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_ce60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// A halving factor stored as a double in the exe image (`01011588`).
const HALF: u32 = 0x0101_1588;
/// A small float constant stored in the exe image (`0104da4c`); the code stores it in a local
/// that is overwritten before anything reads it.
const SMALL_FLOAT: u32 = 0x0104_da4c;

/// The phantom's list of overlapping collidables: an array at `this + 0x160` of 8-byte
/// entries `{agent, collidable}` with its count at `this + 0x164`.
const LIST: u32 = 0x160;
/// Set to 1 whenever the list changes.
const LIST_CHANGED: u32 = 0x16c;

// Translated from 00d4cab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::setPositionAndLinearCast` (Xbox PDB). `this` is ECX; `from` and
/// `to` point to 16-byte vectors; `arg3` and `arg4` are handed unchanged to every agent.
/// Stores `from` at `this + 0xe0`; asks the object at `this + 0x10` (vtable offset `0x1c`,
/// arguments `this + 0xb0`, the float `0.5 * owner->0x78 record float +8 + to[5]`, a local
/// 32-byte bounding box) for the bounding box, grows it by the displacement `to - from` (the
/// lane-wise minimum and maximum against zero added to its corners) and passes it to
/// `00d07ec0(this, box)`. It then builds a local cast-input record from the owner's `0x78`
/// record (words 0 to 5, 16-byte blocks at `+0x20`, `+0x30` and `+0x40`, the word at `+0x70`),
/// the displacement and its length, and the floats `to[5]` and `to[4]`, and calls, for each
/// entry of the overlapping list from the last to the first, the agent's vtable offset `0x14`
/// with `(this + 0x10, collidable, record, arg3, arg4)`.
///
/// Uninitialised stack words: the exe never writes the record's words at +0x18 and +0x1c
/// (between the six copied words and the 16-byte blocks) nor the word at +0x6c (after the
/// `+0x68` tail word), and the dead store of 0 to +0x4 is overwritten by the word copy. They
/// are alignment padding of the cast-input struct: the callee that was checked
/// (`00daf540`, `hkpIterativeLinearCastAgent::linearCast`) copies the first 0x50 bytes whole
/// into a local `hkpCollisionInput` and reads only +0x08, +0x50, +0x60, +0x64 and +0x68 of the
/// input itself, never the padding words. The other agents behind the virtual call (slot
/// `0x14`) were not enumerated, so that no agent reads them is unconfirmed; they are zero here.
pub fn hkp_caching_shape_phantom_set_position_and_linear_cast(
    e: &mut Engine,
    this: Ptr,
    from: Ptr,
    to: Ptr,
    arg3: u32,
    arg4: u32,
) {
    let this = this.addr();
    let (from, to) = (from.addr(), to.addr());
    e.with_stack(0xb0, |e, local| {
        let b = local.addr();
        let lane = |e: &Engine, address: u32| -> [f32; 4] {
            [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
        };
        let put = |e: &mut Engine, address: u32, value: [f32; 4]| {
            for (i, v) in value.into_iter().enumerate() {
                e.mem.set_u32(address + 4 * i as u32, v.to_bits());
            }
        };
        let start = lane(e, from);
        put(e, this + 0xe0, start);
        let shape_owner = e.mem.u32(this + 8);
        let record = e.mem.u32(shape_owner + 0x78);
        let owner = e.mem.u32(this + 0x10);
        let half = (f64::from(e.mem.f32(record + 8)) * e.mem.f64(HALF)) as f32;
        let radius = x87_quiet(half + e.mem.f32(to + 0x14));
        e.vcall(owner, 0x1c, &args![this + 0xb0, radius.to_bits(), b + 0x10]);
        let end = lane(e, to);
        let mut delta = [0.0f32; 4];
        let mut low = [0.0f32; 4];
        let mut high = [0.0f32; 4];
        let bounds_min = lane(e, b + 0x10);
        let bounds_max = lane(e, b + 0x20);
        for i in 0..4 {
            delta[i] = end[i] - start[i];
            let zero = 0.0f32;
            let lo = if zero < delta[i] { zero } else { delta[i] };
            let hi = if zero > delta[i] { zero } else { delta[i] };
            low[i] = lo + bounds_min[i];
            high[i] = hi + bounds_max[i];
        }
        put(e, b + 0xa0, delta);
        put(e, b + 0x10, low);
        put(e, b + 0x20, high);
        e.call(0x00d0_7ec0, &args![this, b + 0x10]);
        let small = x87_quiet_bits(e.mem.u32(SMALL_FLOAT));
        e.mem.set_u32(b + 0x90, small);
        e.mem.set_u32(b + 0x34, 0);
        let record = e.mem.u32(e.mem.u32(this + 8) + 0x78);
        for k in 0..6u32 {
            let word = e.mem.u32(record + 4 * k);
            e.mem.set_u32(b + 0x30 + 4 * k, word);
        }
        for (to_offset, from_offset) in [(0x50, 0x20), (0x60, 0x30), (0x70, 0x40)] {
            let block = e.mem.bytes(record + from_offset, 16);
            e.mem.write(b + to_offset, &block);
        }
        let tail = e.mem.u32(record + 0x70);
        put(e, b + 0x80, delta);
        let squares = delta.map(|v| v * v);
        let sum = squares[2] + (squares[1] + squares[0]);
        e.mem.set_u32(b + 0x98, tail);
        let length = sum.sqrt();
        e.mem.set_u32(b + 0x94, x87_quiet_bits(length.to_bits()));
        let z = e.mem.u32(to + 0x14);
        e.mem.set_u32(b + 0x38, x87_quiet_bits(z));
        let w = e.mem.u32(to + 0x10);
        let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1) as i32;
        e.mem.set_u32(b + 0x90, x87_quiet_bits(w));
        while index >= 0 {
            let entry = e.mem.u32(this + LIST) + (index as u32) * 8;
            let agent = e.mem.u32(entry);
            let collidable = e.mem.u32(entry + 4);
            e.vcall(
                agent,
                0x14,
                &args![this + 0x10, collidable, b + 0x30, arg3, arg4],
            );
            index -= 1;
        }
    });
}

// Translated from 00d4cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::removeOverlappingCollidable` (Xbox PDB). `this` is ECX. Does
/// nothing when the collidable's first word is zero. Otherwise it searches the overlapping
/// list from the last entry down for the collidable; either way it runs
/// `00ca9520(this, collidable, flag)`, where `flag` is the collidable pointer with its low
/// byte replaced by 1 (found) or 0 (not found). When found, it also calls the entry's agent
/// (vtable offset `0x1c`) with a 24-byte local object whose vtable is `010ccebc` (RTTI name
/// `hkpConstraintOwner`) and whose 16-bit word at +6 is 1, removes the entry by moving the
/// last entry into its place and sets the list-changed byte.
pub fn hkp_caching_shape_phantom_remove_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: Ptr,
) {
    let this = this.addr();
    let c = collidable.addr();
    if e.mem.u32(c) == 0 {
        return;
    }
    let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1) as i32;
    let mut found = false;
    while index >= 0 {
        let entry = e.mem.u32(this + LIST) + (index as u32) * 8;
        if e.mem.u32(entry + 4) == c {
            found = true;
            break;
        }
        index -= 1;
    }
    if !found {
        e.call(0x00ca_9520, &args![this, c, c & 0xffff_ff00]);
        return;
    }
    e.call(0x00ca_9520, &args![this, c, (c & 0xffff_ff00) | 1]);
    let agent = e.mem.u32(e.mem.u32(this + LIST) + (index as u32) * 8);
    if agent != 0 {
        e.with_stack(0x18, |e, local| {
            e.mem.set_u32(local.addr(), 0x010c_cebc);
            e.mem.set_u16(local.addr() + 6, 1);
            e.vcall(agent, 0x1c, &args![local]);
        });
    }
    let count = e.mem.u32(this + LIST + 4).wrapping_sub(1);
    e.mem.set_u32(this + LIST + 4, count);
    if count != index as u32 {
        let data = e.mem.u32(this + LIST);
        let last = e.mem.u32(data + count * 8);
        let last_collidable = e.mem.u32(data + count * 8 + 4);
        e.mem.set_u32(data + (index as u32) * 8, last);
        e.mem
            .set_u32(data + (index as u32) * 8 + 4, last_collidable);
    }
    e.mem.set_u8(this + LIST_CHANGED, 1);
}

// Translated from 00d4cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::addOverlappingCollidable` (Xbox PDB). `this` is ECX. Does nothing
/// when the collidable's first word is zero or when `00ca94c0(this, collidable)` returns
/// non-zero. Otherwise it appends a slot to the overlapping list (`_reserveMore` with 8-byte
/// elements when the count equals the capacity), copies the owner's `0x78` record (80 bytes)
/// into a local, lets `00cfa7d0(local, record)` set it up, picks the agent-creation function
/// from the table the local's first word points to (index byte at
/// `table + 0x1a0 + (this->0x10 word +0xc << 5) + collidable->first word +0xc`, five words
/// per function at `table + 0x9a0`) and calls it as `(this + 0x10, collidable, local, 0)`,
/// storing its result and the collidable in the new slot. Sets the list-changed byte and frees
/// a 0x200-byte block (through the allocator's vtable offset 8) if the local's word at
/// `+0x84` ends up non-zero. The local frame is 0x94 bytes; the copied record is at its `+0x20`.
pub fn hkp_caching_shape_phantom_add_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: Ptr,
) {
    let this = this.addr();
    let c = collidable.addr();
    if e.mem.u32(c) == 0 {
        return;
    }
    if e.call(0x00ca_94c0, &args![this, c]).u32() != 0 {
        return;
    }
    let list = this + LIST;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
    if e.mem.u32(list + 4) == capacity {
        let heap = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![heap, list, 8u32]);
    }
    let count = e.mem.u32(list + 4);
    let slot = e.mem.u32(list) + count * 8;
    e.mem.set_u32(list + 4, count + 1);
    let record = e.mem.u32(e.mem.u32(this + 8) + 0x78);
    e.with_stack(0xa0, |e, local| {
        let b = local.addr();
        let words = e.mem.bytes(record, 0x50);
        e.mem.write(b + 0x20, &words);
        e.call(0x00cf_a7d0, &args![b + 0x20, record]);
        let table = e.mem.u32(b + 0x20);
        e.mem.set_u32(b + 0x34, 0);
        let first = e.mem.u32(c);
        let other = e.mem.u32(this + 0x10);
        e.mem.set_u32(b + 0x80, table.wrapping_add(0x1c60));
        let row = e.mem.u32(other + 0xc) << 5;
        let column = e.mem.u32(first + 0xc);
        let index = u32::from(
            e.mem.u8(table
                .wrapping_add(0x1a0)
                .wrapping_add(row)
                .wrapping_add(column)),
        );
        let function = e
            .mem
            .u32(table.wrapping_add(index * 20).wrapping_add(0x9a0));
        let agent = e
            .call(function, &args![this + 0x10, c, b + 0x20, 0u32])
            .u32();
        e.mem.set_u32(slot, agent);
        e.mem.set_u32(slot + 4, c);
        e.mem.set_u8(this + LIST_CHANGED, 1);
        let buffer = e.mem.u32(b + 0x84);
        if buffer != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![buffer, 0x200u32]);
        }
    });
}

/// The vtable `010d087c` of `hkpCachingShapePhantom` (by its RTTI), the vtable `010ccebc` of the
/// `hkpConstraintOwner` object the exe builds on its stack, and the three string labels the
/// statistics function passes (`0126902c` holds only zero bytes in the image, `"AgentPtr"` and
/// `"agents"`).
const PHANTOM_VTABLE: u32 = 0x010d_087c;
const CONSTRAINT_OWNER_VTABLE: u32 = 0x010c_cebc;
const BASE_STATISTICS_LABEL: u32 = 0x0126_902c;
const LABEL_AGENT_PTR: u32 = 0x010d_086c;
const LABEL_AGENTS: u32 = 0x010d_0864;
/// `hkpPhantom::calcContentStatistics` (`00d07cf0`, `this` = the phantom, then the collector
/// and a label), `hkpPhantom::~hkpPhantom` (`00d08130`), `hkpShapePhantom::
/// deallocateInternalArrays` (`00c9e820`), the `hkpShapePhantom` constructor (`00c9ea40`, `this`
/// then a shape word and a transform pointer), `hkStatisticsCollector::getAllocatedSize`
/// (`00d06f80`, cdecl `(block, bytes)`) and `hkpWorldObject::copyProperties` (`00ca92a0`, `this`
/// = the copy, then the original).
const PHANTOM_CALC_CONTENT_STATISTICS: u32 = 0x00d0_7cf0;
const PHANTOM_DESTRUCTOR: u32 = 0x00d0_8130;
const SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS: u32 = 0x00c9_e820;
const SHAPE_PHANTOM_CONSTRUCTOR: u32 = 0x00c9_ea40;
const STATISTICS_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
const COPY_PROPERTIES: u32 = 0x00ca_92a0;

// Translated from 00d4c910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::getType` (Xbox PDB): returns 2 (`this` is not used).
pub fn hkp_caching_shape_phantom_get_type(_e: &mut Engine, _unused_this: Ptr) -> u32 {
    2
}

// Translated from 00d4c920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::getClosestPoints` (Xbox PDB). `this` is ECX; `collector` and
/// `input` are the two stack words (the roles are described from their use). When `input` is
/// zero the record at `[[this + 8] + 0x78]` is used instead. For each entry of the overlapping
/// list (8 bytes: agent, collidable) from the last to the first it calls the agent's vtable
/// offset `0x10` with `(this + 0x10, collidable, input, collector)`.
pub fn hkp_caching_shape_phantom_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    input: Ptr,
) {
    let this = this.addr();
    let mut input = input.addr();
    if input == 0 {
        input = e.mem.u32(e.mem.u32(this + 8) + 0x78);
    }
    let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1);
    while (index as i32) >= 0 {
        let entry = e.mem.u32(this + LIST).wrapping_add(index.wrapping_mul(8));
        let agent = e.mem.u32(entry);
        let collidable = e.mem.u32(entry + 4);
        e.vcall(
            agent,
            0x10,
            &args![this + 0x10, collidable, input, collector],
        );
        index = index.wrapping_sub(1);
    }
}

// Translated from 00d4c980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::getPenetrations` (Xbox PDB). As `getClosestPoints` (`00d4c920`) with
/// the agent's vtable offset `0xc`; after each call the loop ends when the byte at `collector +
/// 4` is non-zero.
pub fn hkp_caching_shape_phantom_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    input: Ptr,
) {
    let this = this.addr();
    let mut input = input.addr();
    if input == 0 {
        input = e.mem.u32(e.mem.u32(this + 8) + 0x78);
    }
    let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1);
    while (index as i32) >= 0 {
        let entry = e.mem.u32(this + LIST).wrapping_add(index.wrapping_mul(8));
        let agent = e.mem.u32(entry);
        let collidable = e.mem.u32(entry + 4);
        e.vcall(
            agent,
            0xc,
            &args![this + 0x10, collidable, input, collector],
        );
        if e.mem.u8(collector.addr() + 4) != 0 {
            return;
        }
        index = index.wrapping_sub(1);
    }
}

// Translated from 00d4c9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::isOverlappingCollidableAdded` (Xbox PDB). `this` is ECX; `result`
/// is the byte to write: 1 when a list entry (at `this + 0x160`, 8 bytes each, count at `+0x164`)
/// holds `collidable` in its second word, 0 otherwise.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_caching_shape_phantom_is_overlapping_collidable_added(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    collidable: u32,
) -> Ptr {
    let this = this.addr();
    let count = e.mem.u32(this + LIST + 4) as i32;
    let data = e.mem.u32(this + LIST);
    let found =
        (0..count).any(|index| e.mem.u32(data.wrapping_add(4 + 8 * index as u32)) == collidable);
    e.mem.set_u8(result.addr(), u8::from(found));
    result
}

// Translated from 00d4ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::updateShapeCollectionFilter` (Xbox PDB; `this` is ECX, no stack
/// words). It copies the 0x50-byte record at `[[this + 8] + 0x78]` into a local and builds a
/// stack `hkpConstraintOwner` (vtable `010ccebc`, the half-word at +6 set to 1; the half-word at
/// +4 is never written by the exe and is zero here, the callee was not enumerated). Then, for
/// each list entry from the first to the last, it calls the agent's vtable offset `0x20` with
/// `(this + 0x10, collidable, record, owner)`.
pub fn hkp_caching_shape_phantom_update_shape_collection_filter(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(0x70, |e, frame| {
        let owner = frame.addr();
        let record = frame.addr() + 0x20;
        let source = e.mem.u32(e.mem.u32(this + 8) + 0x78);
        for word in 0..0x14 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(record + 4 * word, value);
        }
        e.mem.set_u16(owner + 6, 1);
        e.mem.set_u32(owner, CONSTRAINT_OWNER_VTABLE);
        let mut index = 0u32;
        if (e.mem.u32(this + LIST + 4) as i32) > 0 {
            loop {
                let entry = e.mem.u32(this + LIST).wrapping_add(index.wrapping_mul(8));
                let agent = e.mem.u32(entry);
                let collidable = e.mem.u32(entry + 4);
                e.vcall(agent, 0x20, &args![this + 0x10, collidable, record, owner]);
                index = index.wrapping_add(1);
                if (index as i32) >= (e.mem.u32(this + LIST + 4) as i32) {
                    break;
                }
            }
        }
    });
}

// Translated from 00d4cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::setTransformAndLinearCast` (Xbox PDB). `this` is ECX; `transform`
/// points to three 16-byte vectors followed by the position vector at `+0x30`. The three vectors
/// are copied (as raw 16-byte moves) to `this + 0xb0`, `+0xc0` and `+0xd0`, then
/// `setPositionAndLinearCast(this, transform + 0x30, to, arg3, arg4)` (`00d4cab0`) runs.
pub fn hkp_caching_shape_phantom_set_transform_and_linear_cast(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    to: Ptr,
    arg3: u32,
    arg4: u32,
) {
    for word in 0..12 {
        let value = e.mem.u32(transform.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0xb0 + 4 * word, value);
    }
    hkp_caching_shape_phantom_set_position_and_linear_cast(
        e,
        this,
        Ptr::new(transform.addr() + 0x30),
        to,
        arg3,
        arg4,
    );
}

// Translated from 00d4cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::calcContentStatistics` (Xbox PDB; `this` is ECX, `RET 8`: the
/// collector and a second stack word that is never read). The base statistics (`00d07cf0`,
/// `(this, collector, label 0126902c)`) run first. When the list's capacity word (`this +
/// 0x168`) is non-negative or the collector's byte at `+0x10` is set, the list is reported:
/// collector vtable offset `0x14` with `(3, "AgentPtr", data, count * 8, allocated size)`, the
/// size coming from `00d06f80(data, capacity * 8)`. Then, between the collector's offsets `0x18`
/// (`"agents"`) and `0x1c`, offset `0xc` is called with `(0, agent, 0)` for every entry.
pub fn hkp_caching_shape_phantom_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let this = this.addr();
    let collector = collector.addr();
    e.call(
        PHANTOM_CALC_CONTENT_STATISTICS,
        &args![this, collector, BASE_STATISTICS_LABEL],
    );
    let capacity = e.mem.u32(this + LIST + 8);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(this + LIST);
        let allocated = e
            .call(
                STATISTICS_ALLOCATED_SIZE,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(8)],
            )
            .u32();
        let data = e.mem.u32(this + LIST);
        let used = e.mem.u32(this + LIST + 4).wrapping_mul(8);
        e.vcall(
            collector,
            0x14,
            &args![3u32, LABEL_AGENT_PTR, data, used, allocated],
        );
    }
    e.vcall(collector, 0x18, &args![LABEL_AGENTS]);
    let mut index = 0u32;
    if (e.mem.u32(this + LIST + 4) as i32) > 0 {
        loop {
            let agent = e
                .mem
                .u32(e.mem.u32(this + LIST).wrapping_add(index.wrapping_mul(8)));
            e.vcall(collector, 0xc, &args![0u32, agent, 0u32]);
            index = index.wrapping_add(1);
            if (index as i32) >= (e.mem.u32(this + LIST + 4) as i32) {
                break;
            }
        }
    }
    e.vcall(collector, 0x1c, &args![]);
}

// Translated from 00d4ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `hkpCachingShapePhantom` (the map gives the function no name; `RET 0xC`).
/// It runs the `hkpShapePhantom` constructor (`00c9ea40`, `this` then `shape` and `transform`),
/// sets the vtable `010d087c`, empties the list (data 0, count 0, capacity word `0x80000000`),
/// clears the list-changed byte, stores `user_word` at `+0x2c` and returns `this`. The roles of
/// the three words are described from `clone` (`00d4d0d0`), which passes the word at `+0x10`,
/// `this + 0xb0` and the word at `+0x2c`.
pub fn fn_00d4ce10(e: &mut Engine, this: Ptr, shape: u32, transform: u32, user_word: u32) -> Ptr {
    let address = this.addr();
    e.call(SHAPE_PHANTOM_CONSTRUCTOR, &args![address, shape, transform]);
    e.mem.set_u32(address, PHANTOM_VTABLE);
    e.mem.set_u32(address + LIST, 0);
    e.mem.set_u32(address + LIST + 4, 0);
    e.mem.set_u32(address + LIST + 8, 0x8000_0000);
    e.mem.set_u8(address + LIST_CHANGED, 0);
    e.mem.set_u32(address + 0x2c, user_word);
    this
}

// Translated from 00d4ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::~hkpCachingShapePhantom` (Xbox PDB; `this` is ECX). It sets the
/// vtable `010d087c`, calls the vtable offset `0x1c` of every list entry's agent from the last
/// to the first with a stack `hkpConstraintOwner` (vtable `010ccebc`, half-word at +6 = 1), empties
/// the list, frees its heap storage through the memory router's allocator (offset `0x10`, `capacity
/// * 8` bytes, when the capacity word is non-negative), resets it to the inline empty state
/// (`0x80000000`) and finishes with `hkpPhantom::~hkpPhantom` (`00d08130`).
pub fn hkp_caching_shape_phantom_destructor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1);
    e.mem.set_u32(this, PHANTOM_VTABLE);
    e.with_stack(0x18, |e, owner| {
        let owner = owner.addr();
        e.mem.set_u16(owner + 6, 1);
        e.mem.set_u32(owner, CONSTRAINT_OWNER_VTABLE);
        while (index as i32) >= 0 {
            let agent = e
                .mem
                .u32(e.mem.u32(this + LIST).wrapping_add(index.wrapping_mul(8)));
            e.vcall(agent, 0x1c, &args![owner]);
            index = index.wrapping_sub(1);
        }
    });
    e.mem.set_u32(this + LIST + 4, 0);
    free_list_storage(e, this);
    e.mem.set_u32(this + LIST, 0);
    e.mem.set_u32(this + LIST + 8, 0x8000_0000);
    e.call(PHANTOM_DESTRUCTOR, &args![this]);
}

/// Frees the list's heap storage: the count is cleared and, when the capacity word is
/// non-negative, the data goes back to the memory router's allocator (offset `0x10`, `capacity
/// * 8` bytes).
fn free_list_storage(e: &mut Engine, this: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(this + LIST + 8);
    e.mem.set_u32(this + LIST + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(this + LIST);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(8)],
        );
    }
}

// Translated from 00d4d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::deallocateInternalArrays` (Xbox PDB; `this` is ECX). When the list
/// is empty its heap storage is freed and it is reset to the inline empty state; then
/// `hkpShapePhantom::deallocateInternalArrays` (`00c9e820`) runs.
pub fn hkp_caching_shape_phantom_deallocate_internal_arrays(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if e.mem.u32(this + LIST + 4) == 0 {
        free_list_storage(e, this);
        e.mem.set_u32(this + LIST, 0);
        e.mem.set_u32(this + LIST + 8, 0x8000_0000);
    }
    e.call(SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, &args![this]);
}

// Translated from 00d4d090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::ensureDeterministicOrder` (Xbox PDB; `this` is ECX). When the
/// list-changed byte is set, a list of more than one entry is sorted by the uid at `+0x14` of
/// each entry's collidable (`quickSortRecursive<hkpCollisionDetail, OrderByUid>`, `00d4d250`;
/// the fourth word is the empty comparison object, which the exe fills with `this` and its low
/// byte cleared), and the byte is cleared.
pub fn hkp_caching_shape_phantom_ensure_deterministic_order(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if e.mem.u8(this + LIST_CHANGED) != 0 {
        let count = e.mem.u32(this + LIST + 4) as i32;
        if count > 1 {
            let data = e.mem.u32(this + LIST);
            hk_algorithm_quick_sort_recursive_order_by_uid(
                e,
                data,
                0,
                count.wrapping_sub(1),
                this & 0xffff_ff00,
            );
        }
        e.mem.set_u8(this + LIST_CHANGED, 0);
    }
}

/// Makes the clone's word array at `offset` (data, count, capacity word) as large as the
/// original's and copies it: when the clone's capacity (low 30 bits of the capacity word) is
/// smaller than the original's count, the old block (if the capacity word is non-negative) is
/// freed through the memory router's allocator (offset `0x10`, `capacity * 4` bytes) and a new
/// one is requested through offset `0xc` with a pointer to the byte size (the allocator may
/// change it); the capacity becomes the granted size divided by 4 (rounding toward zero).
fn copy_word_array(e: &mut Engine, clone: u32, original: u32, offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(clone + offset + 8);
    let capacity = capacity_word & 0x3fff_ffff;
    if (capacity as i32) < (e.mem.u32(original + offset + 4) as i32) {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(clone + offset);
            e.vcall(allocator, 0x10, &args![data, capacity.wrapping_mul(4)]);
        }
        let bytes = e.mem.u32(original + offset + 4).wrapping_mul(4);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![slot]).u32();
            e.mem.set_u32(clone + offset, block);
            let granted = e.mem.u32(slot.addr()) as i32;
            let rounded = granted.wrapping_add((granted >> 31) & 3);
            e.mem.set_u32(clone + offset + 8, (rounded >> 2) as u32);
        });
    }
    let count = e.mem.u32(original + offset + 4);
    e.mem.set_u32(clone + offset + 4, count);
    let destination = e.mem.u32(clone + offset);
    let source = e.mem.u32(original + offset);
    for word in 0..count as i32 {
        let value = e.mem.u32(source.wrapping_add(4 * word as u32));
        e.mem
            .set_u32(destination.wrapping_add(4 * word as u32), value);
    }
}

// Translated from 00d4d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::clone` (Xbox PDB; `this` is ECX). It takes 0x170 bytes from the
/// memory router's allocator (offset 4), stores the size 0x170 in the half-word at +4, builds the
/// copy with the constructor `00d4ce10` from the word at `+0x10`, `this + 0xb0` and the word at
/// `+0x2c`, copies the two word arrays at `+0x8c` and `+0x98` (`copy_word_array`), lets
/// `hkpWorldObject::copyProperties` (`00ca92a0`, `(copy, this)`) run and returns the copy.
pub fn hkp_caching_shape_phantom_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x170u32]).u32();
    e.mem.set_u16(block + 4, 0x170);
    let shape = e.mem.u32(this + 0x10);
    let user_word = e.mem.u32(this + 0x2c);
    let copy = fn_00d4ce10(e, Ptr::new(block), shape, this + 0xb0, user_word).addr();
    copy_word_array(e, copy, this, 0x8c);
    copy_word_array(e, copy, this, 0x98);
    e.call(COPY_PROPERTIES, &args![copy, this]);
    Ptr::new(copy)
}

// Translated from 00d4d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpCachingShapePhantom::hkpCollisionDetail,
/// hkpCachingShapePhantom::OrderByUid>` (Xbox PDB; cdecl, four words). It sorts the 8-byte
/// entries (agent, collidable) at `array` from `low` to `high` (inclusive, signed indices) by
/// the unsigned uid at `+0x14` of the entry's collidable (the second word), by Hoare
/// partitioning around the middle entry (`(low + high) >> 1`, its collidable word kept and its
/// uid re-read on each pass), recursing on the lower part and looping on the upper one. The
/// fourth word is the comparison object and is only handed on to the recursion.
pub fn hk_algorithm_quick_sort_recursive_order_by_uid(
    e: &mut Engine,
    array: u32,
    low: i32,
    high: i32,
    comparator: u32,
) {
    let entry = |index: i32| array.wrapping_add((index as u32).wrapping_mul(8));
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = e.mem.u32(entry(middle) + 4);
        let mut i = low;
        let mut j = high;
        loop {
            let pivot_uid = e.mem.u32(pivot.wrapping_add(0x14));
            while e.mem.u32(e.mem.u32(entry(i) + 4).wrapping_add(0x14)) < pivot_uid {
                i = i.wrapping_add(1);
            }
            while pivot_uid < e.mem.u32(e.mem.u32(entry(j) + 4).wrapping_add(0x14)) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                for word in [0, 4] {
                    let from_i = e.mem.u32(entry(i) + word);
                    let from_j = e.mem.u32(entry(j) + word);
                    e.mem.set_u32(entry(j) + word, from_i);
                    e.mem.set_u32(entry(i) + word, from_j);
                }
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            hk_algorithm_quick_sort_recursive_order_by_uid(e, array, low, j, comparator);
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_d310, deleting_destructor_00d4d310(Ptr, u8) -> Ptr),
        entry!(
            0x00d4_cab0,
            hkp_caching_shape_phantom_set_position_and_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d4_cca0,
            hkp_caching_shape_phantom_remove_overlapping_collidable(Ptr, Ptr)
        ),
        entry!(
            0x00d4_cf10,
            hkp_caching_shape_phantom_add_overlapping_collidable(Ptr, Ptr)
        ),
        entry!(0x00d4_c910, hkp_caching_shape_phantom_get_type(Ptr) -> u32),
        entry!(
            0x00d4_c920,
            hkp_caching_shape_phantom_get_closest_points(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d4_c980,
            hkp_caching_shape_phantom_get_penetrations(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d4_c9e0,
            hkp_caching_shape_phantom_is_overlapping_collidable_added(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d4_ca30,
            hkp_caching_shape_phantom_update_shape_collection_filter(Ptr)
        ),
        entry!(
            0x00d4_cc50,
            hkp_caching_shape_phantom_set_transform_and_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d4_cd60,
            hkp_caching_shape_phantom_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d4_ce10, fn_00d4ce10(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d4_ce60, hkp_caching_shape_phantom_destructor(Ptr)),
        entry!(
            0x00d4_d030,
            hkp_caching_shape_phantom_deallocate_internal_arrays(Ptr)
        ),
        entry!(
            0x00d4_d090,
            hkp_caching_shape_phantom_ensure_deterministic_order(Ptr)
        ),
        entry!(0x00d4_d0d0, hkp_caching_shape_phantom_clone(Ptr) -> Ptr),
        entry!(
            0x00d4_d250,
            hk_algorithm_quick_sort_recursive_order_by_uid(u32, i32, i32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn deleting_destructor_00d4d310_frees_only_when_bit_zero_is_set() {
        check(0x00d4_d310, 0x00d4_ce60);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn vtable_with(e: &mut Engine, offset: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + offset, target);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn remove_moves_the_last_entry_into_the_gap() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_9520, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        let first = vtable_with(&mut e, 0x1c, 0x0900_001c);
        let last = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x180);
        let data = e.mem.alloc(0x20);
        let c1 = e.mem.alloc(0x10);
        let c2 = e.mem.alloc(0x10);
        e.mem.set_u32(c1, 1);
        e.mem.set_u32(c2, 1);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, c1);
        e.mem.set_u32(data + 8, last);
        e.mem.set_u32(data + 12, c2);
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x164, 2);
        e.call(0x00d4_cca0, &args![this, c1]);
        assert_eq!(e.mem.u32(this + 0x164), 1);
        assert_eq!(e.mem.u32(data), last);
        assert_eq!(e.mem.u32(data + 4), c2);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        let calls = log.borrow();
        assert_eq!(calls[0].0, 0x00ca_9520);
        assert_eq!(calls[0].1, vec![this, c1, (c1 & 0xffff_ff00) | 1]);
        assert_eq!(calls[1].0, 0x0900_001c);
        assert_eq!(calls[1].1[0], first);
    }

    #[test]
    fn remove_of_an_unknown_collidable_only_notifies() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_9520, 0);
        let this = e.mem.alloc(0x180);
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, 1);
        e.call(0x00d4_cca0, &args![this, c]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0].1[2], c & 0xffff_ff00);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        // A collidable whose first word is zero is ignored entirely.
        let zero = e.mem.alloc(0x10);
        e.call(0x00d4_cca0, &args![this, zero]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn add_appends_an_entry_made_by_the_table_function() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_94c0, 0);
        log_double(&mut e, &log, 0x00c9_0940, 0);
        log_double(&mut e, &log, 0x0900_00a0, 0x5555);
        let table = e.mem.alloc(0x2000);
        let row = 3u32;
        let column = 5u32;
        e.mem.set_u8(table + 0x1a0 + (row << 5) + column, 2);
        e.mem.set_u32(table + 2 * 20 + 0x9a0, 0x0900_00a0);
        let sink = Rc::clone(&log);
        e.register_double(0x00cf_a7d0, move |e, a| {
            sink.borrow_mut().push((0x00cf_a7d0, a.to_vec()));
            e.mem.set_u32(a[0], table);
            Ret::default()
        });
        let heap = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x180);
        let owner = e.mem.alloc(0x100);
        let record = e.mem.alloc(0x100);
        e.mem.set_u32(record + 4, 0x77);
        e.mem.set_u32(owner + 0x78, record);
        e.mem.set_u32(this + 8, owner);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, row);
        e.mem.set_u32(this + 0x10, other);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x164, 1);
        e.mem.set_u32(this + 0x168, 1);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, column);
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, shape);
        e.call(0x00d4_cf10, &args![this, c]);
        assert_eq!(e.mem.u32(this + 0x164), 2);
        assert_eq!(e.mem.u32(data + 8), 0x5555);
        assert_eq!(e.mem.u32(data + 12), c);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        let calls = log.borrow();
        let reserve = calls.iter().find(|c| c.0 == 0x00c9_0940).unwrap();
        assert_eq!(reserve.1, vec![heap, this + 0x160, 8]);
        let made = calls.iter().find(|c| c.0 == 0x0900_00a0).unwrap();
        assert_eq!(made.1[0], this + 0x10);
        assert_eq!(made.1[1], c);
        assert_eq!(made.1[3], 0);
    }

    #[test]
    fn add_skips_a_zero_or_rejected_collidable() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_94c0, 1);
        let this = e.mem.alloc(0x180);
        let zero = e.mem.alloc(0x10);
        e.call(0x00d4_cf10, &args![this, zero]);
        assert!(log.borrow().is_empty());
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, 1);
        e.call(0x00d4_cf10, &args![this, c]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(this + 0x164), 0);
    }

    #[test]
    fn linear_cast_grows_the_box_and_calls_each_agent() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        e.mem.set_f64(HALF, 0.5);
        let sink = Rc::clone(&log);
        e.register_double(0x00d0_7ec0, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|i| e.mem.u32(a[1] + 4 * i)));
            sink.borrow_mut().push((0x00d0_7ec0, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_001c, move |e, a| {
            sink.borrow_mut().push((0x0900_001c, a.to_vec()));
            let corners = [-1.0f32, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0];
            for (i, v) in corners.into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, v);
            }
            Ret::default()
        });
        log_double(&mut e, &log, 0x0900_0014, 0);
        let this = e.mem.alloc(0x180);
        let owner_object = vtable_with(&mut e, 0x1c, 0x0900_001c);
        e.mem.set_u32(this + 0x10, owner_object);
        let shape_owner = e.mem.alloc(0x100);
        let record = e.mem.alloc(0x100);
        e.mem.set_f32(record + 8, 4.0);
        e.mem.set_u32(record + 0x70, 0x99);
        e.mem.set_u32(shape_owner + 0x78, record);
        e.mem.set_u32(this + 8, shape_owner);
        let agent = vtable_with(&mut e, 0x14, 0x0900_0014);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, agent);
        e.mem.set_u32(list + 4, 0xc011);
        e.mem.set_u32(this + 0x160, list);
        e.mem.set_u32(this + 0x164, 1);
        let from = e.mem.alloc(0x10);
        let to = e.mem.alloc(0x20);
        for (i, v) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(from + 4 * i as u32, v);
        }
        for (i, v) in [4.0f32, 2.0, -1.0, 0.0, 0.25, 1.5].into_iter().enumerate() {
            e.mem.set_f32(to + 4 * i as u32, v);
        }
        e.call(0x00d4_cab0, &args![this, from, to, 7u32, 8u32]);
        assert_eq!(e.mem.f32(this + 0xe0), 1.0);
        assert_eq!(e.mem.f32(this + 0xe8), 3.0);
        let calls = log.borrow();
        let owner_call = calls.iter().find(|c| c.0 == 0x0900_001c).unwrap();
        assert_eq!(owner_call.1[1], this + 0xb0);
        // 0.5 * 4.0 + to[5]
        assert_eq!(f32::from_bits(owner_call.1[2]), 3.5);
        let boxed = calls.iter().find(|c| c.0 == 0x00d0_7ec0).unwrap();
        // displacement is (3, 0, -4, 0): the minimum adds -4 on z, the maximum 3 on x.
        let words: Vec<f32> = boxed.1[2..].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(words[0], -1.0);
        assert_eq!(words[2], -5.0);
        assert_eq!(words[4], 4.0);
        assert_eq!(words[6], 1.0);
        let agent_call = calls.iter().find(|c| c.0 == 0x0900_0014).unwrap();
        assert_eq!(agent_call.1[1..3], [this + 0x10, 0xc011]);
        assert_eq!(agent_call.1[4..], [7, 8]);
    }
}

#[cfg(test)]
mod havok_agent_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const AGENT_FIRST: u32 = 0x0900_3010;
    const AGENT_PENETRATIONS: u32 = 0x0900_300c;
    const AGENT_FILTER: u32 = 0x0900_3020;
    const AGENT_REMOVE: u32 = 0x0900_301c;
    const ALLOCATE: u32 = 0x0900_4004;
    const ALLOCATE_BUFFER: u32 = 0x0900_400c;
    const FREE: u32 = 0x0900_4010;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of the logged calls, in order.
    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The memory router; its allocator has the allocate (4), buffer-allocate (0xc) and free
    /// (0x10) slots, all logged.
    fn router(e: &mut Engine, log: &Log) {
        log_double(e, log, ALLOCATE, 0);
        log_double(e, log, ALLOCATE_BUFFER, 0);
        log_double(e, log, FREE, 0);
        let allocator = object(e, &[(4, ALLOCATE), (0xc, ALLOCATE_BUFFER), (0x10, FREE)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    /// An agent whose slots 0xc, 0x10, 0x1c and 0x20 are logging doubles tagged by the agent.
    fn agent(e: &mut Engine, log: &Log) -> u32 {
        let slots = [
            (0xc, AGENT_PENETRATIONS),
            (0x10, AGENT_FIRST),
            (0x1c, AGENT_REMOVE),
            (0x20, AGENT_FILTER),
        ];
        for (_, address) in slots {
            log_double(e, log, address, 0);
        }
        object(e, &slots)
    }

    /// A phantom with the given `(agent, collidable)` list entries in an array of `capacity`
    /// (the capacity word given), a shape owner whose `+0x78` points to a record of 20 words
    /// numbered 1 to 20.
    fn phantom(e: &mut Engine, entries: &[(u32, u32)], capacity_word: u32) -> u32 {
        let this = e.mem.alloc(0x1c0);
        let owner = e.mem.alloc(0x100);
        let record = e.mem.alloc(0x100);
        for word in 0..20 {
            e.mem.set_u32(record + 4 * word, word + 1);
        }
        e.mem.set_u32(owner + 0x78, record);
        e.mem.set_u32(this + 8, owner);
        let data = e.mem.alloc(0x100);
        for (index, (agent, collidable)) in entries.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *agent);
            e.mem.set_u32(data + 8 * index as u32 + 4, *collidable);
        }
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x164, entries.len() as u32);
        e.mem.set_u32(this + 0x168, capacity_word);
        this
    }

    fn record_of(e: &Engine, this: u32) -> u32 {
        e.mem.u32(e.mem.u32(this + 8) + 0x78)
    }

    #[test]
    fn the_type_is_two() {
        let (mut e, _log) = engine();
        assert_eq!(e.call(0x00d4_c910, &args![0u32]).u32(), 2);
    }

    #[test]
    fn closest_points_visit_the_agents_from_the_last() {
        let (mut e, log) = engine();
        let (a, b, c) = (
            agent(&mut e, &log),
            agent(&mut e, &log),
            agent(&mut e, &log),
        );
        let this = phantom(&mut e, &[(a, 0x11), (b, 0x22), (c, 0x33)], 4);
        e.call(0x00d4_c920, &args![this, 0x5150u32, 0x7777u32]);
        assert_eq!(
            calls(&log, AGENT_FIRST),
            vec![
                vec![c, this + 0x10, 0x33, 0x7777, 0x5150],
                vec![b, this + 0x10, 0x22, 0x7777, 0x5150],
                vec![a, this + 0x10, 0x11, 0x7777, 0x5150],
            ]
        );
        // A zero input is replaced by the owner's record.
        log.borrow_mut().clear();
        e.call(0x00d4_c920, &args![this, 0x5150u32, 0u32]);
        let record = record_of(&e, this);
        assert_eq!(calls(&log, AGENT_FIRST)[0][3], record);
        // An empty list calls nothing.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x164, 0);
        e.call(0x00d4_c920, &args![this, 0x5150u32, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn penetrations_stop_when_the_collector_byte_is_set() {
        let (mut e, log) = engine();
        let (a, b, c) = (
            agent(&mut e, &log),
            agent(&mut e, &log),
            agent(&mut e, &log),
        );
        let this = phantom(&mut e, &[(a, 0x11), (b, 0x22), (c, 0x33)], 4);
        let collector = e.mem.alloc(0x10);
        // The second call sets the byte.
        let sink = Rc::clone(&log);
        e.register_double(AGENT_PENETRATIONS, move |e, args| {
            sink.borrow_mut().push((AGENT_PENETRATIONS, args.to_vec()));
            if sink.borrow().len() == 2 {
                e.mem.set_u8(args[4] + 4, 1);
            }
            Ret::default()
        });
        e.call(0x00d4_c980, &args![this, collector, 0u32]);
        let record = record_of(&e, this);
        assert_eq!(
            calls(&log, AGENT_PENETRATIONS),
            vec![
                vec![c, this + 0x10, 0x33, record, collector],
                vec![b, this + 0x10, 0x22, record, collector],
            ]
        );
    }

    #[test]
    fn penetrations_visit_every_agent_when_the_byte_stays_clear() {
        let (mut e, log) = engine();
        let (a, b) = (agent(&mut e, &log), agent(&mut e, &log));
        let this = phantom(&mut e, &[(a, 0x11), (b, 0x22)], 4);
        let collector = e.mem.alloc(0x10);
        e.call(0x00d4_c980, &args![this, collector, 0x1234u32]);
        assert_eq!(calls(&log, AGENT_PENETRATIONS).len(), 2);
        assert_eq!(calls(&log, AGENT_PENETRATIONS)[1][3], 0x1234);
    }

    #[test]
    fn overlapping_collidable_lookup_writes_one_or_zero() {
        let (mut e, _log) = engine();
        let this = phantom(&mut e, &[(1, 0x11), (2, 0x22), (3, 0x33)], 4);
        let result = e.mem.alloc(0x10);
        e.call(0x00d4_c9e0, &args![this, result, 0x33u32]);
        assert_eq!(e.mem.u8(result), 1);
        e.call(0x00d4_c9e0, &args![this, result, 0x2u32]);
        assert_eq!(e.mem.u8(result), 0);
        // The agents' words are not compared, and an empty list never matches.
        e.mem.set_u8(result, 0xff);
        e.mem.set_u32(this + 0x164, 0);
        e.call(0x00d4_c9e0, &args![this, result, 0x11u32]);
        assert_eq!(e.mem.u8(result), 0);
    }

    #[test]
    fn fn_00d4c9e0_answers_its_result_pointer() {
        let (mut e, _log) = engine();
        let this = phantom(&mut e, &[(1, 0x11), (2, 0x22), (3, 0x33)], 4);
        let result = e.mem.alloc(0x10);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00d4_c9e0, &args![this, result, 0x33u32]).u32(),
            args![result][0]
        );
    }

    #[test]
    fn update_filter_hands_a_copy_of_the_record_and_a_constraint_owner_to_each_agent() {
        let (mut e, log) = engine();
        let seen = Rc::clone(&log);
        e.register_double(AGENT_FILTER, move |e, a| {
            // Capture the record words and the owner at call time.
            let mut words = a.to_vec();
            words.extend((0..20).map(|word| e.mem.u32(a[3] + 4 * word)));
            words.push(e.mem.u32(a[4]));
            words.push(u32::from(e.mem.u16(a[4] + 6)));
            seen.borrow_mut().push((AGENT_FILTER, words));
            Ret::default()
        });
        let a = object(&mut e, &[(0x20, AGENT_FILTER)]);
        let b = object(&mut e, &[(0x20, AGENT_FILTER)]);
        let this = phantom(&mut e, &[(a, 0x11), (b, 0x22)], 4);
        e.call(0x00d4_ca30, &args![this]);
        let made = calls(&log, AGENT_FILTER);
        assert_eq!(made.len(), 2);
        for (call, (agent, collidable)) in made.iter().zip([(a, 0x11), (b, 0x22)]) {
            assert_eq!(call[..3], [agent, this + 0x10, collidable]);
            // The record is a copy, not the original.
            assert_ne!(call[3], record_of(&e, this));
            assert_eq!(call[5..25], (1..=20).collect::<Vec<u32>>()[..]);
            assert_eq!(call[25], CONSTRAINT_OWNER_VTABLE);
            assert_eq!(call[26], 1);
        }
        // An empty list calls nothing.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x164, 0);
        e.call(0x00d4_ca30, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn set_transform_copies_the_rows_and_casts_from_the_position() {
        let (mut e, log) = engine();
        log_double(&mut e, &log, 0x00d0_7ec0, 0);
        log_double(&mut e, &log, 0x0900_501c, 0);
        let shape_owner = object(&mut e, &[(0x1c, 0x0900_501c)]);
        let this = phantom(&mut e, &[], 0x8000_0000);
        e.mem.set_u32(this + 0x10, shape_owner);
        let transform = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(transform + 4 * word, 0x100 + word);
        }
        let to = e.mem.alloc(0x20);
        e.call(
            0x00d4_cc50,
            &args![this, transform, to, 0xaaaau32, 0xbbbbu32],
        );
        for word in 0..12 {
            assert_eq!(e.mem.u32(this + 0xb0 + 4 * word), 0x100 + word);
        }
        // The position at +0x30 is the cast's start, kept at +0xe0.
        for word in 0..4 {
            assert_eq!(e.mem.u32(this + 0xe0 + 4 * word), 0x10c + word);
        }
        assert_eq!(calls(&log, 0x0900_501c).len(), 1);
        assert_eq!(calls(&log, 0x00d0_7ec0).len(), 1);
    }

    #[test]
    fn content_statistics_reports_the_list_and_every_agent() {
        let (mut e, log) = engine();
        log_double(&mut e, &log, PHANTOM_CALC_CONTENT_STATISTICS, 0);
        log_double(&mut e, &log, STATISTICS_ALLOCATED_SIZE, 0x66);
        for slot in [0xc, 0x14, 0x18, 0x1c] {
            log_double(&mut e, &log, 0x0900_6000 + slot, 0);
        }
        let collector = object(
            &mut e,
            &[
                (0xc, 0x0900_600c),
                (0x14, 0x0900_6014),
                (0x18, 0x0900_6018),
                (0x1c, 0x0900_601c),
            ],
        );
        let this = phantom(&mut e, &[(0xa1, 0x11), (0xa2, 0x22)], 4);
        let data = e.mem.u32(this + 0x160);
        e.call(0x00d4_cd60, &args![this, collector, 0u32]);
        assert_eq!(
            order(&log),
            vec![
                PHANTOM_CALC_CONTENT_STATISTICS,
                STATISTICS_ALLOCATED_SIZE,
                0x0900_6014,
                0x0900_6018,
                0x0900_600c,
                0x0900_600c,
                0x0900_601c,
            ]
        );
        assert_eq!(
            calls(&log, PHANTOM_CALC_CONTENT_STATISTICS),
            vec![vec![this, collector, BASE_STATISTICS_LABEL]]
        );
        assert_eq!(
            calls(&log, STATISTICS_ALLOCATED_SIZE),
            vec![vec![data, 4 * 8]]
        );
        assert_eq!(
            calls(&log, 0x0900_6014),
            vec![vec![collector, 3, LABEL_AGENT_PTR, data, 2 * 8, 0x66]]
        );
        assert_eq!(
            calls(&log, 0x0900_6018),
            vec![vec![collector, LABEL_AGENTS]]
        );
        assert_eq!(
            calls(&log, 0x0900_600c),
            vec![vec![collector, 0, 0xa1, 0], vec![collector, 0, 0xa2, 0]]
        );
    }

    #[test]
    fn content_statistics_skips_inline_storage_unless_the_collector_asks() {
        let (mut e, log) = engine();
        log_double(&mut e, &log, PHANTOM_CALC_CONTENT_STATISTICS, 0);
        log_double(&mut e, &log, STATISTICS_ALLOCATED_SIZE, 0x66);
        for slot in [0x14, 0x18, 0x1c] {
            log_double(&mut e, &log, 0x0900_6000 + slot, 0);
        }
        let collector = object(
            &mut e,
            &[
                (0x14, 0x0900_6014),
                (0x18, 0x0900_6018),
                (0x1c, 0x0900_601c),
            ],
        );
        let this = phantom(&mut e, &[], 0x8000_0004);
        e.call(0x00d4_cd60, &args![this, collector, 0u32]);
        assert!(calls(&log, 0x0900_6014).is_empty());
        assert!(calls(&log, STATISTICS_ALLOCATED_SIZE).is_empty());
        assert_eq!(calls(&log, 0x0900_601c).len(), 1);
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d4_cd60, &args![this, collector, 0u32]);
        let data = e.mem.u32(this + 0x160);
        assert_eq!(
            calls(&log, STATISTICS_ALLOCATED_SIZE),
            vec![vec![data, 4 * 8]]
        );
        assert_eq!(calls(&log, 0x0900_6014).len(), 1);
    }

    #[test]
    fn the_constructor_fills_in_the_fields() {
        let (mut e, log) = engine();
        log_double(&mut e, &log, SHAPE_PHANTOM_CONSTRUCTOR, 0);
        let this = e.mem.alloc(0x1c0);
        e.mem.set_u32(this + 0x160, 0xdead);
        e.mem.set_u8(this + 0x16c, 1);
        let got = e.call(0x00d4_ce10, &args![this, 0x11u32, 0x22u32, 0x33u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(
            calls(&log, SHAPE_PHANTOM_CONSTRUCTOR),
            vec![vec![this, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(this), PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x164), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x33);
    }

    #[test]
    fn the_destructor_releases_the_agents_from_the_last_and_frees_the_list() {
        let (mut e, log) = engine();
        router(&mut e, &log);
        log_double(&mut e, &log, PHANTOM_DESTRUCTOR, 0);
        let seen = Rc::clone(&log);
        e.register_double(AGENT_REMOVE, move |e, a| {
            seen.borrow_mut().push((
                AGENT_REMOVE,
                vec![a[0], a[1], e.mem.u32(a[1]), u32::from(e.mem.u16(a[1] + 6))],
            ));
            Ret::default()
        });
        let a = object(&mut e, &[(0x1c, AGENT_REMOVE)]);
        let b = object(&mut e, &[(0x1c, AGENT_REMOVE)]);
        let this = phantom(&mut e, &[(a, 0x11), (b, 0x22)], 6);
        let data = e.mem.u32(this + 0x160);
        e.mem.set_u8(this + 0x16c, 1);
        e.call(0x00d4_ce60, &args![this]);
        let removed = calls(&log, AGENT_REMOVE);
        assert_eq!(removed.len(), 2);
        assert_eq!(removed[0][0], b);
        assert_eq!(removed[1][0], a);
        for call in &removed {
            assert_eq!(call[2..], [CONSTRAINT_OWNER_VTABLE, 1]);
        }
        assert_eq!(e.mem.u32(this), PHANTOM_VTABLE);
        assert_eq!(calls(&log, FREE).len(), 1);
        assert_eq!(calls(&log, FREE)[0][1..], [data, 6 * 8]);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x164), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        assert_eq!(calls(&log, PHANTOM_DESTRUCTOR), vec![vec![this]]);
        // The base destructor is the last call.
        assert_eq!(*order(&log).last().unwrap(), PHANTOM_DESTRUCTOR);
    }

    #[test]
    fn the_destructor_does_not_free_inline_storage() {
        let (mut e, log) = engine();
        router(&mut e, &log);
        log_double(&mut e, &log, PHANTOM_DESTRUCTOR, 0);
        let this = phantom(&mut e, &[], 0x8000_0008);
        e.call(0x00d4_ce60, &args![this]);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        assert_eq!(calls(&log, PHANTOM_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn deallocate_frees_an_empty_list_and_always_runs_the_base() {
        let (mut e, log) = engine();
        router(&mut e, &log);
        log_double(&mut e, &log, SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, 0);
        let this = phantom(&mut e, &[], 5);
        let data = e.mem.u32(this + 0x160);
        e.call(0x00d4_d030, &args![this]);
        assert_eq!(calls(&log, FREE)[0][1..], [data, 5 * 8]);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        assert_eq!(
            calls(&log, SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS),
            vec![vec![this]]
        );
        // A list that still has entries is left alone.
        log.borrow_mut().clear();
        let busy = phantom(&mut e, &[(1, 2)], 5);
        e.call(0x00d4_d030, &args![busy]);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(e.mem.u32(busy + 0x168), 5);
        assert_eq!(
            calls(&log, SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS),
            vec![vec![busy]]
        );
    }

    #[test]
    fn deterministic_order_sorts_by_uid_once() {
        let (mut e, _log) = engine();
        let mut collidables = Vec::new();
        for uid in [30u32, 10, 20] {
            let collidable = e.mem.alloc(0x20);
            e.mem.set_u32(collidable + 0x14, uid);
            collidables.push(collidable);
        }
        let this = phantom(
            &mut e,
            &[
                (0xa0, collidables[0]),
                (0xa1, collidables[1]),
                (0xa2, collidables[2]),
            ],
            4,
        );
        // Not flagged: untouched.
        e.call(0x00d4_d090, &args![this]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x160)), 0xa0);
        e.mem.set_u8(this + 0x16c, 1);
        e.call(0x00d4_d090, &args![this]);
        let data = e.mem.u32(this + 0x160);
        let order: Vec<(u32, u32)> = (0..3)
            .map(|index| (e.mem.u32(data + 8 * index), e.mem.u32(data + 8 * index + 4)))
            .collect();
        assert_eq!(
            order,
            vec![
                (0xa1, collidables[1]),
                (0xa2, collidables[2]),
                (0xa0, collidables[0])
            ]
        );
        assert_eq!(e.mem.u8(this + 0x16c), 0);
    }

    #[test]
    fn deterministic_order_leaves_a_single_entry_and_clears_the_flag() {
        let (mut e, _log) = engine();
        let this = phantom(&mut e, &[(0xa0, 0x1234)], 4);
        e.mem.set_u8(this + 0x16c, 1);
        e.call(0x00d4_d090, &args![this]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x160) + 4), 0x1234);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
    }

    #[test]
    fn the_sort_orders_many_entries_with_duplicates_by_unsigned_uid() {
        let (mut e, _log) = engine();
        let uids = [
            7u32,
            3,
            0xffff_fff0,
            9,
            3,
            1,
            8,
            8,
            0,
            5,
            0x8000_0000,
            2,
            6,
            4,
            9,
            1,
        ];
        let array = e.mem.alloc(8 * uids.len() as u32);
        for (index, uid) in uids.iter().enumerate() {
            let collidable = e.mem.alloc(0x20);
            e.mem.set_u32(collidable + 0x14, *uid);
            e.mem
                .set_u32(array + 8 * index as u32, 0x1000 + index as u32);
            e.mem.set_u32(array + 8 * index as u32 + 4, collidable);
        }
        e.call(
            0x00d4_d250,
            &args![array, 0i32, uids.len() as i32 - 1, 0u32],
        );
        let sorted: Vec<(u32, u32)> = (0..uids.len() as u32)
            .map(|index| {
                let collidable = e.mem.u32(array + 8 * index + 4);
                (e.mem.u32(collidable + 0x14), e.mem.u32(array + 8 * index))
            })
            .collect();
        let mut expected: Vec<u32> = uids.to_vec();
        expected.sort_unstable();
        assert_eq!(
            sorted.iter().map(|entry| entry.0).collect::<Vec<_>>(),
            expected
        );
        // Every agent word is still there exactly once and still paired with its uid.
        let mut agents: Vec<u32> = sorted.iter().map(|entry| entry.1).collect();
        agents.sort_unstable();
        assert_eq!(
            agents,
            (0x1000..0x1000 + uids.len() as u32).collect::<Vec<_>>()
        );
        for (uid, agent) in sorted {
            assert_eq!(uids[(agent - 0x1000) as usize], uid);
        }
    }

    #[test]
    fn the_sort_handles_two_entries_and_a_sub_range() {
        let (mut e, _log) = engine();
        let array = e.mem.alloc(0x40);
        for (index, uid) in [5u32, 2, 9, 1].into_iter().enumerate() {
            let collidable = e.mem.alloc(0x20);
            e.mem.set_u32(collidable + 0x14, uid);
            e.mem.set_u32(array + 8 * index as u32, uid);
            e.mem.set_u32(array + 8 * index as u32 + 4, collidable);
        }
        // Only the entries 1 and 2 (uids 2 and 9) are in range: already in order.
        e.call(0x00d4_d250, &args![array, 1i32, 2i32, 0u32]);
        let agents: Vec<u32> = (0..4).map(|index| e.mem.u32(array + 8 * index)).collect();
        assert_eq!(agents, vec![5, 2, 9, 1]);
        // Entries 0 and 1 are swapped.
        e.call(0x00d4_d250, &args![array, 0i32, 1i32, 0u32]);
        let agents: Vec<u32> = (0..4).map(|index| e.mem.u32(array + 8 * index)).collect();
        assert_eq!(agents, vec![2, 5, 9, 1]);
    }

    #[test]
    fn clone_builds_the_copy_and_both_word_arrays() {
        let (mut e, log) = engine();
        router(&mut e, &log);
        log_double(&mut e, &log, SHAPE_PHANTOM_CONSTRUCTOR, 0);
        log_double(&mut e, &log, COPY_PROPERTIES, 0);
        let block = e.mem.alloc(0x170);
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE, move |_, a| {
            sink.borrow_mut().push((ALLOCATE, a.to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        // The buffer allocator hands out a block of 16 bytes more than asked.
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_BUFFER, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_BUFFER, a.to_vec()));
            let asked = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], asked + 16);
            Ret {
                eax: e.mem.alloc(asked + 16),
                ..Ret::default()
            }
        });
        // The base constructor leaves the copy with a small heap array and a small inline one.
        let old_block = e.mem.alloc(0x10);
        let sink = Rc::clone(&log);
        e.register_double(SHAPE_PHANTOM_CONSTRUCTOR, move |e, a| {
            sink.borrow_mut()
                .push((SHAPE_PHANTOM_CONSTRUCTOR, a.to_vec()));
            e.mem.set_u32(a[0] + 0x8c, old_block);
            e.mem.set_u32(a[0] + 0x94, 2);
            e.mem.set_u32(a[0] + 0x98, 0x1234);
            e.mem.set_u32(a[0] + 0xa0, 0x8000_0001);
            Ret::default()
        });
        let this = e.mem.alloc(0x1c0);
        e.mem.set_u32(this + 0x10, 0x5151);
        e.mem.set_u32(this + 0x2c, 0x2c2c);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        for word in 0..5 {
            e.mem.set_u32(first + 4 * word, 0x10 + word);
        }
        for word in 0..3 {
            e.mem.set_u32(second + 4 * word, 0x20 + word);
        }
        e.mem.set_u32(this + 0x8c, first);
        e.mem.set_u32(this + 0x90, 5);
        e.mem.set_u32(this + 0x98, second);
        e.mem.set_u32(this + 0x9c, 3);
        let copy = e.call(0x00d4_d0d0, &args![this]).u32();
        assert_eq!(copy, block);
        assert_eq!(e.mem.u16(copy + 4), 0x170);
        assert_eq!(e.mem.u32(copy), PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(copy + 0x2c), 0x2c2c);
        assert_eq!(calls(&log, ALLOCATE).len(), 1);
        assert_eq!(calls(&log, ALLOCATE)[0][1], 0x170);
        // The copy's first array (capacity 2 < 5, heap) is freed, then 20 bytes are asked for
        // and 36 granted: capacity 9.
        assert_eq!(calls(&log, FREE).len(), 1);
        assert_eq!(calls(&log, FREE)[0][1..], [old_block, 2 * 4]);
        assert_eq!(e.mem.u32(copy + 0x90), 5);
        assert_eq!(e.mem.u32(copy + 0x94), 9);
        let data = e.mem.u32(copy + 0x8c);
        assert_ne!(data, old_block);
        for word in 0..5 {
            assert_eq!(e.mem.u32(data + 4 * word), 0x10 + word);
        }
        // The second array's capacity word was inline (0x80000001, one element): not freed.
        assert_eq!(calls(&log, ALLOCATE_BUFFER).len(), 2);
        assert_eq!(e.mem.u32(copy + 0x9c), 3);
        assert_eq!(e.mem.u32(copy + 0xa0), (12 + 16) / 4);
        let data = e.mem.u32(copy + 0x98);
        for word in 0..3 {
            assert_eq!(e.mem.u32(data + 4 * word), 0x20 + word);
        }
        assert_eq!(calls(&log, COPY_PROPERTIES), vec![vec![copy, this]]);
        let made = calls(&log, SHAPE_PHANTOM_CONSTRUCTOR);
        assert_eq!(made, vec![vec![copy, 0x5151, this + 0xb0]]);
    }

    #[test]
    fn clone_keeps_arrays_that_are_already_large_enough() {
        let (mut e, log) = engine();
        router(&mut e, &log);
        log_double(&mut e, &log, SHAPE_PHANTOM_CONSTRUCTOR, 0);
        log_double(&mut e, &log, COPY_PROPERTIES, 0);
        let block = e.mem.alloc(0x170);
        e.register_double(ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        let own = e.mem.alloc(0x20);
        e.register_double(SHAPE_PHANTOM_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(a[0] + 0x8c, own);
            e.mem.set_u32(a[0] + 0x94, 4);
            Ret::default()
        });
        let this = e.mem.alloc(0x1c0);
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source, 0xabcd);
        e.mem.set_u32(this + 0x8c, source);
        e.mem.set_u32(this + 0x90, 1);
        let copy = e.call(0x00d4_d0d0, &args![this]).u32();
        assert!(calls(&log, ALLOCATE_BUFFER).is_empty());
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(e.mem.u32(copy + 0x8c), own);
        assert_eq!(e.mem.u32(own), 0xabcd);
        assert_eq!(e.mem.u32(copy + 0x90), 1);
        assert_eq!(e.mem.u32(copy + 0x9c), 0);
    }
}
