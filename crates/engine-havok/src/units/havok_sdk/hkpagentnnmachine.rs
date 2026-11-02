//! `Havok SDK/hkpagentnnmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions work on agent entries of 0x80 bytes (`hkpAgentNnEntry` and its agent data),
//! kept in 0x200-byte sectors of a track (`hkpAgentNnTrack`, Xbox PDB: `m_bytesUsedInLastSector`
//! at +0, then the `hkInplaceArray` of sectors at +4: data, size, capacity word, inplace
//! storage). An entry links two collidables (pointers at +0x10 and +0x14); each collidable lists
//! its entries in `m_collisionEntries` (data +0x50, size +0x54, capacity word +0x58; 8 bytes per
//! element: entry, partner collidable), and the entry's `m_agentIndexOnCollidable` (16-bit words
//! at +4 and +6) say where. The collision dispatcher has one 0x50-byte record per agent type
//! (the entry's byte at +1) with function pointers at +0x16b4 (destroy), +0x16c8 (update shape
//! collection filter) and +0x16d4 (warp time).
//!
//! SSE code computes lane by lane in `f32`; floats copied through `FLD`/`FSTP` go through
//! [`x87_quiet`] and [`x87_quiet_bits`].

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpagent1nmachine::{MONITOR_SLOT, TLS_GET_VALUE};
use super::hkpagent1nmachineprocess::{
    entry_dot, swept_offset, MINIMUM_TIME_LABEL, SKIP_CREATE, SWEPT_LERP2,
    TRANSFORM_SET_MUL_INVERSE_MUL,
};

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// Size of an agent entry and of a sector (four entries).
const ENTRY_SIZE: u32 = 0x80;
const SECTOR_SIZE: u32 = 0x200;
/// `hkMemoryRouter` lookup (`00c85750`, no arguments); its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reduce` (Xbox PDB, `00c909d0`, cdecl `(allocator, array, element size,
/// inplace storage, new capacity)`).
const ARRAY_REDUCE: u32 = 0x00c9_09d0;
/// Array insertion (`00cd1e20`, `this` = the array, `(allocator, index, source, count)`; the
/// callee cleans the stack).
const ARRAY_INSERT_AT: u32 = 0x00cd_1e20;
/// `hkQuaternion::setMul` (Xbox PDB, `00c66320`, `this` = result, `(a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The `float` the warp-time function stores when the stored time does not match.
const WARP_MISMATCH_TIME: u32 = 0x0101_2054;
/// A `float` (-3.4028232e38) the process functions store as the remaining distance of an entry
/// they restart.
const UNSET_DISTANCE: u32 = 0x0104_e148;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Frees a block of `size` bytes through the router's allocator (vtable slot 8).
fn router_free(e: &mut Engine, block: u32, size: u32) {
    let allocator = router_allocator(e);
    e.vcall(allocator, 8, &args![block, size]);
}

/// The agent function the dispatcher keeps for the entry's agent type (byte at +1) at table
/// offset `table`.
fn dispatcher_function(e: &Engine, entry: u32, dispatcher: u32, table: u32) -> u32 {
    let record = u32::from(e.mem.u8(entry + 1)) * 0x50;
    e.mem
        .u32(dispatcher.wrapping_add(record).wrapping_add(table))
}

// Translated from 00cfdc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_WarpTimeInAgent` (Xbox PDB): for the entry's type (byte at +0), 2, 6, 10 and
/// 14 call the dispatcher's warp-time function (record +0x16d4; the dispatcher is the first word
/// of `input`) with the entry, its agent data at +0x20, `time_a`, `time_b` and `input`; types 4
/// and 12 first store `time_b` at +0x18 when `time_a` equals the stored time there (otherwise
/// the `float` at `01012054`) and call it with the agent data at +0x30. The other types do
/// nothing. Floats travel through `FLD`/`FSTP`, so signalling NaNs are quieted.
pub fn hk_agent_nn_machine_warp_time_in_agent(
    e: &mut Engine,
    entry: Ptr,
    time_a: f32,
    time_b: f32,
    input: Ptr,
) {
    let me = entry.addr();
    let agent_data = match e.mem.u8(me) {
        2 | 6 | 10 | 14 => me + 0x20,
        4 | 12 => {
            // An unordered (NaN) compare is not equal, as the `FUCOMP` flags say.
            let stored = e.mem.f32(me + 0x18);
            let new_time = if time_a == stored {
                x87_quiet(time_b)
            } else {
                x87_quiet(e.mem.f32(WARP_MISMATCH_TIME))
            };
            e.mem.set_f32(me + 0x18, new_time);
            me + 0x30
        }
        _ => return,
    };
    let dispatcher = e.mem.u32(input.addr());
    let function = dispatcher_function(e, me, dispatcher, 0x16d4);
    if function != 0 {
        e.call(
            function,
            &args![
                me,
                agent_data,
                x87_quiet_bits(time_a.to_bits()),
                x87_quiet_bits(time_b.to_bits()),
                input
            ],
        );
    }
}

// Translated from 00cfdd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_FindAgent` (Xbox PDB): the agent entry linking the two collidables, found
/// by searching the `m_collisionEntries` of the one with fewer entries (the first when equal)
/// for the other as partner; 0 when there is none.
pub fn hk_agent_nn_machine_find_agent(e: &mut Engine, first: Ptr, second: Ptr) -> Ptr {
    let (searched, wanted) = if e.mem.i32(first.addr() + 0x54) > e.mem.i32(second.addr() + 0x54) {
        (second.addr(), first.addr())
    } else {
        (first.addr(), second.addr())
    };
    let count = e.mem.i32(searched + 0x54);
    if count > 0 {
        let data = e.mem.u32(searched + 0x50);
        for index in 0..count as u32 {
            if e.mem.u32(data + index * 8 + 4) == wanted {
                return Ptr::new(e.mem.u32(data + index * 8));
            }
        }
    }
    Ptr::NULL
}

/// Copies the 0x80 bytes of an entry word by word.
fn copy_entry(e: &mut Engine, from: u32, to: u32) {
    for word in 0..ENTRY_SIZE / 4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Points the two collidables' `m_collisionEntries` elements for `entry` (at the indices the
/// entry stores at +4 and +6) at the entry's new address.
fn relink_entry(e: &mut Engine, entry: u32) {
    let first_index = u32::from(e.mem.u16(entry + 4));
    let second_index = u32::from(e.mem.u16(entry + 6));
    let first_collidable = e.mem.u32(entry + 0x10);
    let first_entries = e.mem.u32(first_collidable + 0x50);
    e.mem.set_u32(first_entries + first_index * 8, entry);
    let second_collidable = e.mem.u32(entry + 0x14);
    let second_entries = e.mem.u32(second_collidable + 0x50);
    e.mem.set_u32(second_entries + second_index * 8, entry);
}

/// The element count `hkArrayUtil` rounds up to: `((v >> 1) | v) + 1` with `v` the sizes's
/// `size + 3` smeared right by 16, 8, 4 and 2 bits (as a signed `int`, which can wrap
/// negative).
fn rounded_capacity(size: i32) -> i32 {
    let mut value = (size as u32).wrapping_add(3);
    value |= value >> 16;
    value |= value >> 8;
    value |= value >> 4;
    value |= value >> 2;
    ((value >> 1) | value).wrapping_add(1) as i32
}

/// The last sector of a track (`sectors[size - 1]`).
fn last_sector(e: &Engine, track: u32) -> u32 {
    let size = e.mem.u32(track + 8);
    let data = e.mem.u32(track + 4);
    e.mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4))
}

/// The tail of the track's last-entry removal that `00cfe020` and `00cff290` both contain: takes
/// one entry (0x80 bytes) off the last sector's used bytes; an emptied sector is freed and
/// removed from the sector array; then, unless the array's capacity word is negative (storage
/// not owned), the array shrinks to the rounded capacity (or back to the inplace storage when
/// the rounding is not positive).
fn drop_last_entry_slot(e: &mut Engine, track: u32) {
    let used = e.mem.u32(track).wrapping_sub(ENTRY_SIZE);
    e.mem.set_u32(track, used);
    let sectors = track + 4;
    if used == 0 {
        let last = last_sector(e, track);
        let size = e.mem.i32(sectors + 4);
        e.mem.set_i32(sectors + 4, size.wrapping_sub(1));
        router_free(e, last, SECTOR_SIZE);
        e.mem.set_u32(track, SECTOR_SIZE);
    }
    let capacity = e.mem.u32(sectors + 8);
    if (capacity as i32) < 0 {
        return;
    }
    let wanted = rounded_capacity(e.mem.i32(sectors + 4));
    if wanted < 1 {
        let allocator = router_allocator(e);
        e.call(
            ARRAY_REDUCE,
            &args![allocator, sectors, 4u32, sectors + 0xc, 1u32],
        );
    } else if wanted < (capacity & 0x3fff_ffff) as i32 {
        let allocator = router_allocator(e);
        e.call(
            ARRAY_REDUCE,
            &args![allocator, sectors, 4u32, 0u32, wanted as u32],
        );
    }
}

// Translated from 00cfe020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_InternalDeallocateEntry` (Xbox PDB): removes `entry` from the track by
/// moving the track's last entry into its place (all 0x80 bytes, then relinking the two
/// collidables' element pointers) when it is not the last itself, and releasing the freed slot
/// (`drop_last_entry_slot`).
pub fn hk_agent_nn_machine_internal_deallocate_entry(e: &mut Engine, track: Ptr, entry: Ptr) {
    let me = track.addr();
    let used = e.mem.u32(me);
    let last = last_sector(e, me)
        .wrapping_add(used)
        .wrapping_sub(ENTRY_SIZE);
    if entry.addr() != last {
        copy_entry(e, last, entry.addr());
        relink_entry(e, entry.addr());
    }
    drop_last_entry_slot(e, me);
}

// Translated from 00cfe820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_AllocateEntry` (Xbox PDB): the next free entry slot of the track. The
/// track arrives in EDI (the only argument): with room left in the last sector the slot is
/// there and the used bytes grow by 0x80; otherwise a new sector (0x200 bytes) is allocated
/// through the router's allocator, appended to the sector array (growing it when full), and its
/// first slot is returned.
pub fn hk_agent_nn_machine_allocate_entry(e: &mut Engine, track: Ptr) -> Ptr {
    let me = track.addr();
    let used = e.mem.u32(me);
    if used < SECTOR_SIZE {
        let sector = last_sector(e, me);
        e.mem.set_u32(me, used + ENTRY_SIZE);
        return Ptr::new(sector.wrapping_add(used));
    }
    let allocator = router_allocator(e);
    let sector = e.vcall(allocator, 4, &args![SECTOR_SIZE]).u32();
    let allocator = router_allocator(e);
    let sectors = me + 4;
    if e.mem.u32(sectors + 4) == e.mem.u32(sectors + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, sectors, 4u32]);
    }
    let size = e.mem.u32(sectors + 4);
    let data = e.mem.u32(sectors);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), sector);
    e.mem.set_u32(sectors + 4, size.wrapping_add(1));
    e.mem.set_u32(me, ENTRY_SIZE);
    Ptr::new(sector)
}

// Translated from 00cfe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentMachine_processTransformedShapes` (Xbox PDB): follows a chain of transform shapes
/// (shape type 15 at +0xc of the body's shape). `body` is a collision body (shape +0,
/// shape key +4, transform record +8, parent +0xc); `bodies` receives a body of 16 bytes per
/// level (the child shape at +0, 0 at +4, the transform buffer entry at +8, the parent body at
/// +0xc); `transforms` receives 0xb0 bytes per level. Per level it composes the record's
/// rotation (columns at +0, +0x10, +0x20) with the shape's own rotation (+0x30..+0x50) and
/// translation (+0x60), adds the record's translation (+0x30), copies +0x40 and +0x50,
/// multiplies the quaternions at +0x60 and +0x70 by the shape's at +0x20 (`00c66320`), writes
/// the offset `record +0x80 - shape +0x60` transformed by the transposed shape rotation at
/// +0x80, copies +0x90 and the `float` at +0xa0, and sets `*changed = 8`. Returns the last
/// body. The fourth stack word is not read. `bodies` must not be null.
pub fn hk_agent_machine_process_transformed_shapes(
    e: &mut Engine,
    body: Ptr,
    bodies: Ptr,
    transforms: Ptr,
    _unused_4: u32,
    changed: Ptr,
) -> Ptr {
    let mut body = body.addr();
    let mut bodies = bodies.addr();
    let mut transforms = transforms.addr();
    let mut shape = e.mem.u32(body);
    while e.mem.u32(shape + 0xc) == 0xf {
        let record = e.mem.u32(body + 8);
        let rotation = [
            load_vec(e, record),
            load_vec(e, record + 0x10),
            load_vec(e, record + 0x20),
        ];
        // The record's rotation applied to the shape's three columns and its translation.
        for column in 0..4u32 {
            let source = load_vec(e, shape + 0x30 + 0x10 * column);
            let product: Vec4 = std::array::from_fn(|lane| {
                (source[1] * rotation[1][lane] + source[0] * rotation[0][lane])
                    + source[2] * rotation[2][lane]
            });
            store_vec(e, transforms + 0x10 * column, product);
        }
        let record_translation = load_vec(e, record + 0x30);
        let moved = load_vec(e, transforms + 0x30);
        let translation: Vec4 = std::array::from_fn(|lane| record_translation[lane] + moved[lane]);
        store_vec(e, transforms + 0x30, translation);
        copy_vec(e, record + 0x40, transforms + 0x40);
        copy_vec(e, record + 0x50, transforms + 0x50);
        e.call(
            QUATERNION_SET_MUL,
            &args![transforms + 0x60, record + 0x60, shape + 0x20],
        );
        e.call(
            QUATERNION_SET_MUL,
            &args![transforms + 0x70, record + 0x70, shape + 0x20],
        );
        // The offset from the shape's translation, seen from inside the shape: the rows of
        // the shape's rotation are gathered with UNPCKLPS, UNPCKHPS, MOVLHPS, MOVHLPS and
        // SHUFPS (the fourth lanes take the odd values 'y' and 'w' of the last column).
        let offset_source = load_vec(e, record + 0x80);
        let shape_translation = load_vec(e, shape + 0x60);
        let offset: Vec4 =
            std::array::from_fn(|lane| offset_source[lane] - shape_translation[lane]);
        let first = load_vec(e, shape + 0x30);
        let second = load_vec(e, shape + 0x40);
        let third = load_vec(e, shape + 0x50);
        let x_rows = [first[0], second[0], third[0], third[1]];
        let y_rows = [first[1], second[1], third[1], third[3]];
        let z_rows = [first[2], second[2], third[2], third[3]];
        let local: Vec4 = std::array::from_fn(|lane| {
            (offset[1] * y_rows[lane] + offset[0] * x_rows[lane]) + offset[2] * z_rows[lane]
        });
        store_vec(e, transforms + 0x80, local);
        copy_vec(e, record + 0x90, transforms + 0x90);
        let extra = x87_quiet(e.mem.f32(record + 0xa0));
        e.mem.set_f32(transforms + 0xa0, extra);
        if bodies != 0 {
            e.mem.set_u32(bodies + 0xc, body);
            e.mem.set_u32(bodies + 8, transforms);
        }
        let child = e.mem.u32(shape + 0x14);
        transforms = transforms.wrapping_add(0xb0);
        e.mem.set_u32(bodies, child);
        e.mem.set_u32(bodies + 4, 0);
        body = bodies;
        shape = child;
        bodies = bodies.wrapping_add(0x10);
        e.mem.set_u8(changed.addr(), 8);
    }
    Ptr::new(body)
}

// Translated from 00cfeba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_UpdateShapeCollectionFilter` (Xbox PDB): for the entry's type (2, 6 → agent
/// data at +0x20; 4 → +0x30) calls the dispatcher's update function (record +0x16c8; the
/// dispatcher is the first word of `input`) with the entry, the agent data, the two collidables'
/// bodies (+0x10, +0x14), `input`, the word at entry +8 and `argument`. Types 10, 12 and 14
/// first resolve transform shapes on both bodies
/// (`hk_agent_machine_process_transformed_shapes`, into scratch buffers) and then continue as
/// type 2, 4 or 6 with the resolved bodies. Other types do nothing.
pub fn hk_agent_nn_machine_update_shape_collection_filter(
    e: &mut Engine,
    entry: Ptr,
    input: Ptr,
    argument: u32,
) {
    // The exe aligns ESP, then `SUB ESP,0x614` and three pushes: a 0x620-byte frame with the
    // changed flag at +0x1b, the body arrays at +0x20 and +0x60 and the transform arrays at +0xa0
    // and +0x360 (00cfebe1..00cfec13).
    with_aligned_frame(e, 0x620, |e, frame| {
        let me = entry.addr();
        let mut kind = u32::from(e.mem.u8(me));
        let mut first = e.mem.u32(me + 0x10);
        let mut second = e.mem.u32(me + 0x14);
        let agent_data = loop {
            match kind {
                2 | 6 => break me + 0x20,
                4 => break me + 0x30,
                10 | 12 | 14 => {
                    let first_bodies = frame + 0x20;
                    let first_transforms = frame + 0xa0;
                    let second_bodies = frame + 0x60;
                    let second_transforms = frame + 0x360;
                    let changed = frame + 0x1b;
                    first = hk_agent_machine_process_transformed_shapes(
                        e,
                        Ptr::new(first),
                        Ptr::new(first_bodies),
                        Ptr::new(first_transforms),
                        4,
                        Ptr::new(changed),
                    )
                    .addr();
                    second = hk_agent_machine_process_transformed_shapes(
                        e,
                        Ptr::new(second),
                        Ptr::new(second_bodies),
                        Ptr::new(second_transforms),
                        4,
                        Ptr::new(changed),
                    )
                    .addr();
                    kind &= 0xf7;
                }
                _ => {
                    return;
                }
            }
        };
        let dispatcher = e.mem.u32(input.addr());
        let function = dispatcher_function(e, me, dispatcher, 0x16c8);
        if function != 0 {
            let partner_data = e.mem.u32(me + 8);
            e.call(
                function,
                &args![me, agent_data, first, second, input, partner_data, argument],
            );
        }
    })
}

// Translated from 00cfea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_DestroyAgent` (Xbox PDB): for types 2, 6, 10 and 14 (agent data at +0x20)
/// and 4 and 12 (+0x30) calls the destroy function of the `dispatcher` (record +0x16b4) with the
/// entry, the agent data, the word at entry +8, `context` and `dispatcher`. Then, for each of
/// the entry's two collidables, removes its element from `m_collisionEntries` (the last element
/// moves into the freed index and its entry's stored index is updated, using the pointer at
/// +0x10 to tell which of the entry's two indices it is), shrinking the array to its rounded
/// capacity when that is smaller than the current one; finally releases the entry from the
/// track (`hk_agent_nn_machine_internal_deallocate_entry`).
pub fn hk_agent_nn_machine_destroy_agent(
    e: &mut Engine,
    track: Ptr,
    entry: Ptr,
    dispatcher: Ptr,
    context: u32,
) {
    let me = entry.addr();
    let agent_data = match e.mem.u8(me) {
        2 | 6 | 10 | 14 => Some(me + 0x20),
        4 | 12 => Some(me + 0x30),
        _ => None,
    };
    if let Some(agent_data) = agent_data {
        // The table entry is called without a null test, as the code does.
        let function = dispatcher_function(e, me, dispatcher.addr(), 0x16b4);
        let partner_data = e.mem.u32(me + 8);
        e.call(
            function,
            &args![me, agent_data, partner_data, context, dispatcher],
        );
    }
    for side in 0..2u32 {
        let collidable = e.mem.u32(me + 0x10 + 4 * side);
        let index = u32::from(e.mem.u16(me + 4 + 2 * side));
        let size = e.mem.i32(collidable + 0x54).wrapping_sub(1);
        e.mem.set_i32(collidable + 0x54, size);
        let array = collidable + 0x50;
        if size as u32 != index {
            let data = e.mem.u32(array);
            for word in 0..2 {
                let value = e.mem.u32(data + (size as u32) * 8 + 4 * word);
                e.mem.set_u32(data + index * 8 + 4 * word, value);
            }
        }
        if (index as i32) < size {
            let data = e.mem.u32(array);
            let moved = e.mem.u32(data + index * 8);
            let side_of_moved = u32::from(e.mem.u32(moved + 0x10) != collidable);
            e.mem.set_u16(moved + 4 + side_of_moved * 2, index as u16);
        }
        let allocator = router_allocator(e);
        let wanted = rounded_capacity(e.mem.i32(array + 4));
        if wanted < (e.mem.u32(array + 8) & 0x3fff_ffff) as i32 {
            e.call(
                ARRAY_REDUCE,
                &args![allocator, array, 8u32, 0u32, wanted as u32],
            );
        }
    }
    hk_agent_nn_machine_internal_deallocate_entry(e, track, entry);
}

// Translated from 00cfeca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_CopyAndRelinkAgentEntry` (Xbox PDB): allocates a slot in the track
/// (`hk_agent_nn_machine_allocate_entry`), copies the 0x80 bytes of `source` into it and
/// points the two collidables' element pointers at the copy. Leaves the new entry in EAX
/// (returned).
pub fn hk_agent_nn_machine_copy_and_relink_agent_entry(
    e: &mut Engine,
    track: Ptr,
    source: Ptr,
) -> Ptr {
    let entry = hk_agent_nn_machine_allocate_entry(e, track);
    copy_entry(e, source.addr(), entry.addr());
    relink_entry(e, entry.addr());
    entry
}

// Translated from 00cff290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_AppendTrack` (Xbox PDB): moves the entries of the track `source` onto the
/// end of `target`, last entry first (copy and relink into `target`, then release the slot in
/// `source`), while `target`'s last sector is not full (used bytes below 0x200) and `source`
/// still has sectors. What remains in `source` (whole sectors) is appended to `target`'s sector
/// array (`00cd1e20`), `target` takes `source`'s used bytes, and `source` is left empty with
/// 0x200 used.
pub fn hk_agent_nn_machine_append_track(e: &mut Engine, target: Ptr, source: Ptr) {
    let to = target.addr();
    let from = source.addr();
    while e.mem.u32(to) != SECTOR_SIZE && e.mem.u32(from + 8) != 0 {
        let used = e.mem.u32(from);
        let last = last_sector(e, from)
            .wrapping_add(used)
            .wrapping_sub(ENTRY_SIZE);
        hk_agent_nn_machine_copy_and_relink_agent_entry(e, target, Ptr::new(last));
        drop_last_entry_slot(e, from);
    }
    let remaining = e.mem.u32(from + 8);
    if remaining != 0 {
        let sectors = e.mem.u32(from + 4);
        let target_count = e.mem.u32(to + 8);
        let allocator = router_allocator(e);
        e.call(
            ARRAY_INSERT_AT,
            &args![to + 4, allocator, target_count, sectors, remaining],
        );
        let used = e.mem.u32(from);
        e.mem.set_u32(to, used);
        e.mem.set_u32(from + 8, 0);
        e.mem.set_u32(from, SECTOR_SIZE);
    }
}

// Translated from 00cfdbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_InvalidateTimInAgent` (named in the engine map, cdecl): asks the
/// dispatcher's function at record offset `+0x16d0` (a word of the first of `input`) to
/// invalidate the time stored in the agent. Types 2, 6, 10 and 14 call it with the entry, its
/// agent data at `+0x20` and `input`; types 4 and 12 first store the `float` at `01012054` at
/// `+0x18`, clear the vector at `+0x20` and call it with the agent data at `+0x30`. Other types
/// do nothing, and a missing function (0) is not called.
pub fn hk_agent_nn_machine_invalidate_tim_in_agent(e: &mut Engine, entry: Ptr, input: Ptr) {
    let me = entry.addr();
    let agent_data = match e.mem.u8(me) {
        2 | 6 | 10 | 14 => me + 0x20,
        4 | 12 => {
            let time = x87_quiet(e.mem.f32(WARP_MISMATCH_TIME));
            e.mem.set_f32(me + 0x18, time);
            for word in 0..4 {
                e.mem.set_u32(me + 0x20 + 4 * word, 0);
            }
            me + 0x30
        }
        _ => return,
    };
    let dispatcher = e.mem.u32(input.addr());
    let function = dispatcher_function(e, me, dispatcher, 0x16d0);
    if function != 0 {
        e.call(function, &args![me, agent_data, input]);
    }
}

// Translated from 00cfdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_GetAgentType` (named in the engine map, cdecl): the agent type for two
/// collision bodies (`first` and `second` point at bodies whose first word is a shape; the
/// shape's type is at `+0xc`). The dispatcher (the first word of `input`) has a table of agent
/// types by `(type of the first shape) * 0x20 + type of the second` at `+0xeb0` (at `+0x12b0`
/// when `input`'s word `+0x14` is not 0). `swapped` receives 1 when the agent wants the bodies
/// the other way round (its record's word `+0x16f0` is 2; for agent type 1 a byte of a table at
/// `+0x1a0` selects a record of `0x14` bytes at `+0x7c * 0x14` whose first byte decides), and
/// `agent_type` then the type for the swapped pair; otherwise `swapped` is 0 and `agent_type`
/// the type found.
pub fn hk_agent_nn_machine_get_agent_type(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    agent_type: Ptr,
    swapped: Ptr,
) {
    let dispatcher = e.mem.u32(input.addr());
    let type_a = e.mem.u32(e.mem.u32(first.addr()) + 0xc);
    let type_b = e.mem.u32(e.mem.u32(second.addr()) + 0xc);
    let index = (type_a << 5).wrapping_add(type_b);
    let table = if e.mem.u32(input.addr() + 0x14) == 0 {
        0xeb0
    } else {
        0x12b0
    };
    let found = u32::from(e.mem.u8(dispatcher.wrapping_add(index).wrapping_add(table)));
    let mut flag = e.mem.u32(
        dispatcher
            .wrapping_add(found.wrapping_mul(0x50))
            .wrapping_add(0x16f0),
    ) == 2;
    if found == 1 {
        let selector = u32::from(
            e.mem.u8(dispatcher
                .wrapping_add(type_a.wrapping_add(0xd) << 5)
                .wrapping_add(type_b)),
        );
        flag = e
            .mem
            .u8(dispatcher.wrapping_add(selector.wrapping_add(0x7c).wrapping_mul(0x14)))
            != 0;
    }
    e.mem.set_u32(swapped.addr(), u32::from(flag));
    if flag {
        let reversed = (type_b << 5).wrapping_add(type_a);
        let reversed_type = e
            .mem
            .u8(dispatcher.wrapping_add(reversed).wrapping_add(table));
        e.mem.set_u32(agent_type.addr(), u32::from(reversed_type));
    } else {
        e.mem.set_u32(agent_type.addr(), found);
    }
}

/// `FLD`/`FSTP` copy of a `float`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// Writes the five words of the agent input record the process functions hand to the agents
/// (two body records, 0, the collision input and the word the entry keeps at `+8`).
fn fill_agent_input(e: &mut Engine, base: u32, first: u32, second: u32, ctx: u32, entry: u32) {
    e.mem.set_u32(base, first);
    e.mem.set_u32(base + 4, second);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, ctx);
    let word = e.mem.u32(entry + 8);
    e.mem.set_u32(base + 0x10, word);
}

/// What `hkAgentNnMachine_ProcessAgent` does for one entry, in the exe's three copies (this
/// function's own code, the register-input `InlineProcessAgent` and the loop body of
/// `ProcessTrack`, which differ only in where the arguments live, in the counter the last two
/// keep and in the frame). `base` is the agent input record in the frame (the exe's `s + 0x20`):
/// the five words of [`fill_agent_input`], the relative transform at `+0x20`, the pending
/// distance at `+0x60` and the vector at `+0x70`; scratch records follow it (`+0x80`, `+0x90`,
/// results of `lerp2` at `+0xa0` and `+0xe0`, the second input at `+0x120` with its transform at
/// `+0x140`, buffers of the transform-shape resolution at `+0x180`, `+0x1c0`, `+0x200` and
/// `+0x4c0`, the changed flag at `-9`; the frame of the exe's copies swaps the two `0x40`-byte
/// buffers in `ProcessTrack`). `ctx` is the collision input (its first word the dispatcher),
/// `third` the record handed on to the agent functions (its word `+4` goes to function
/// `+0x16b8`), `counter` the address of a counter to increase for every entry passed on.
///
/// Types 2, 4, 6 and 10/12/14 (the same after the transform shapes of both bodies were resolved,
/// type 12 being 4) are handled; the others do nothing. The vector at `+0x70` moves the entry
/// by the bodies' sweep over the step `ctx` word `+0x58`. The slots the exe writes but never
/// reads (`0x7f7fffee` in a local) are not kept.
fn process_entry(
    e: &mut Engine,
    base: u32,
    entry: u32,
    ctx: u32,
    third: u32,
    counter: Option<u32>,
) {
    let mut kind = u32::from(e.mem.u8(entry));
    let mut first = e.mem.u32(entry + 0x10);
    let mut second = e.mem.u32(entry + 0x14);
    if kind.wrapping_sub(2) > 0xc {
        return;
    }
    loop {
        match kind {
            2 => {
                fill_agent_input(e, base, first, second, ctx, entry);
                let (motion_a, motion_b) = (e.mem.u32(first + 8), e.mem.u32(second + 8));
                let step = x87_quiet(e.mem.f32(ctx + 0x58));
                let vector = swept_offset(e, motion_a, motion_b, step);
                store_vec(e, base + 0x70, vector);
                e.call(
                    TRANSFORM_SET_MUL_INVERSE_MUL,
                    &args![base + 0x20, motion_a, motion_b],
                );
                let dispatcher = e.mem.u32(ctx);
                let function = dispatcher_function(e, entry, dispatcher, 0x16dc);
                e.call(function, &args![base, entry, entry + 0x20, 0u32, third]);
                return;
            }
            4 => {
                sweep_entry(e, base, entry, ctx, third, (first, second), counter);
                return;
            }
            6 => {
                let object = e.mem.u32(entry + 0x20);
                e.vcall(object, 0x18, &args![first, second, ctx, third]);
                return;
            }
            10 | 12 | 14 => {
                first = hk_agent_machine_process_transformed_shapes(
                    e,
                    Ptr::new(first),
                    Ptr::new(base + 0x180),
                    Ptr::new(base + 0x200),
                    4,
                    Ptr::new(base - 9),
                )
                .addr();
                second = hk_agent_machine_process_transformed_shapes(
                    e,
                    Ptr::new(second),
                    Ptr::new(base + 0x1c0),
                    Ptr::new(base + 0x4c0),
                    4,
                    Ptr::new(base - 9),
                )
                .addr();
                kind &= 0xf7;
                if kind.wrapping_sub(2) > 0xc {
                    return;
                }
            }
            _ => return,
        }
    }
}

/// Type 4 of [`process_entry`] (agent data at `+0x30`, the entry's vector at `+0x20`): the
/// entry's time (`+0x18`) is compared with the collision input's times (`+0x50`, `+0x54`), the
/// agent is brought to the input's time when it differs (restarted when the object at
/// `ctx + 0x60` has no state at `+0x10`, else prepared by the function at `+0x16d8` with the
/// bodies interpolated to the time) and the remaining distance (`+0x2c`) is reduced by the dot
/// product with the sweep vector at `base + 0x70`. When the collision input's limit (`+8`) is
/// not above what remains the entry is passed on, otherwise the agent function `+0x16dc` runs.
fn sweep_entry(
    e: &mut Engine,
    base: u32,
    entry: u32,
    ctx: u32,
    third: u32,
    bodies: (u32, u32),
    counter: Option<u32>,
) {
    let (first, second) = bodies;
    fill_agent_input(e, base, first, second, ctx, entry);
    let (motion_a, motion_b) = (e.mem.u32(first + 8), e.mem.u32(second + 8));
    let step = x87_quiet(e.mem.f32(ctx + 0x58));
    let vector = swept_offset(e, motion_a, motion_b, step);
    store_vec(e, base + 0x70, vector);
    let mut pending: Option<f32> = None;
    let mut update = false;
    if e.mem.f32(entry + 0x18) == e.mem.f32(ctx + 0x50) {
        update = true;
    } else {
        let object = e.mem.u32(ctx + 0x60);
        if e.mem.u32(object + 0x10) == 0 {
            copy_float(e, ctx + 0x54, entry + 0x18);
            for word in 0..4 {
                e.mem.set_u32(entry + 0x20 + 4 * word, 0);
            }
            let restarted = x87_quiet(e.mem.f32(UNSET_DISTANCE));
            e.mem.set_f32(entry + 0x2c, restarted);
            pending = Some(restarted);
        } else {
            prepare_at_time(e, base, entry, ctx);
            update = true;
        }
    }
    if update {
        copy_float(e, ctx + 0x54, entry + 0x18);
        let dot = entry_dot(e, entry + 0x20, base + 0x70);
        let remaining = (f64::from(e.mem.f32(entry + 0x2c)) - f64::from(dot)) as f32;
        if e.mem.f32(ctx + 8) <= remaining {
            e.mem.set_f32(entry + 0x2c, remaining);
            if e.mem.u8(entry + 2) != 0 {
                let dispatcher = e.mem.u32(ctx);
                let function = dispatcher_function(e, entry, dispatcher, 0x16b8);
                let word = e.mem.u32(base + 0x10);
                let passed = e.mem.u32(third + 4);
                e.call(function, &args![entry, entry + 0x30, word, passed]);
            }
            if let Some(counter) = counter {
                let count = e.mem.u32(counter).wrapping_add(1);
                e.mem.set_u32(counter, count);
            }
            return;
        }
        pending = Some(remaining);
    }
    let Some(distance) = pending else { return };
    e.mem.set_f32(base + 0x60, distance);
    e.call(
        TRANSFORM_SET_MUL_INVERSE_MUL,
        &args![base + 0x20, motion_a, motion_b],
    );
    let dispatcher = e.mem.u32(ctx);
    let function = dispatcher_function(e, entry, dispatcher, 0x16dc);
    e.call(
        function,
        &args![base, entry, entry + 0x30, entry + 0x20, third],
    );
}

/// The agent preparation of [`sweep_entry`]: both bodies' motion states are interpolated to the
/// collision input's time (`hkSweptTransformUtil::lerp2` into `base + 0xa0` and `base + 0xe0`),
/// two body records and a second agent input (`base + 0x120`) are built from them and the
/// function at `+0x16d8` is called with the second input, the entry, its agent data and its
/// vector.
fn prepare_at_time(e: &mut Engine, base: u32, entry: u32, ctx: u32) {
    let first = e.mem.u32(base);
    e.mem.set_u32(base + 0x128, 0);
    let motion_a = e.mem.u32(first + 8) + 0x40;
    let time = x87_quiet_bits(e.mem.u32(ctx + 0x50));
    e.call(SWEPT_LERP2, &args![motion_a, time, base + 0xa0]);
    let second = e.mem.u32(base + 4);
    let motion_b = e.mem.u32(second + 8) + 0x40;
    let time = x87_quiet_bits(e.mem.u32(e.mem.u32(base + 0xc) + 0x50));
    e.call(SWEPT_LERP2, &args![motion_b, time, base + 0xe0]);
    let word = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0x130, word);
    let (first_words, second_words) = (
        [e.mem.u32(first), e.mem.u32(first + 4)],
        [e.mem.u32(second), e.mem.u32(second + 4)],
    );
    e.mem.set_u32(base + 0x120, base + 0x80);
    let word = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0x12c, word);
    e.mem.set_u32(base + 0x124, base + 0x90);
    e.mem.set_u32(base + 0x84, first_words[1]);
    e.mem.set_u32(base + 0x80, first_words[0]);
    e.mem.set_u32(base + 0x9c, second);
    e.mem.set_u32(base + 0x94, second_words[1]);
    e.mem.set_u32(base + 0x98, base + 0xe0);
    e.mem.set_u32(base + 0x8c, first);
    e.mem.set_u32(base + 0x88, base + 0xa0);
    e.mem.set_u32(base + 0x90, second_words[0]);
    e.call(
        TRANSFORM_SET_MUL_INVERSE_MUL,
        &args![base + 0x140, base + 0xa0, base + 0xe0],
    );
    let dispatcher = e.mem.u32(ctx);
    let function = dispatcher_function(e, entry, dispatcher, 0x16d8);
    e.call(
        function,
        &args![base + 0x120, entry, entry + 0x30, entry + 0x20],
    );
}

// Translated from 00cfedd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_ProcessAgent` (named in the engine map, cdecl): brings one agent entry up
/// to the collision input's time and runs its agent function, see [`process_entry`]. `third` is
/// handed on to the agent functions (its word `+4` also to the pass-on function `+0x16b8`).
pub fn hk_agent_nn_machine_process_agent(e: &mut Engine, entry: Ptr, ctx: Ptr, third: Ptr) {
    e.with_stack(0x7c0, |e, frame| {
        process_entry(
            e,
            frame.addr() + 0x20,
            entry.addr(),
            ctx.addr(),
            third.addr(),
            None,
        );
    });
}

// Translated from 00cfe370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_InlineProcessAgent` (named in the engine map), register input: `entry` is
/// EAX and `ctx` (the collision input) is EDI; the stack holds `counter` (a counter increased
/// for every entry that is passed on, also when its function is skipped) and `third` (as in
/// [`hk_agent_nn_machine_process_agent`]; the caller removes both). No caller of this copy was
/// found in the exe.
pub fn hk_agent_nn_machine_inline_process_agent(
    e: &mut Engine,
    entry: Ptr,
    ctx: Ptr,
    counter: Ptr,
    third: Ptr,
) {
    e.with_stack(0x7c0, |e, frame| {
        process_entry(
            e,
            frame.addr() + 0x20,
            entry.addr(),
            ctx.addr(),
            third.addr(),
            Some(counter.addr()),
        );
    });
}

/// Makes room for one more element of 8 bytes in the collidable's entry list (data `+0x50`, size
/// `+0x54`, capacity word `+0x58`) and returns the address of the new element.
fn add_to_entry_list(e: &mut Engine, collidable: u32) -> u32 {
    let allocator = router_allocator(e);
    let array = collidable + 0x50;
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    slot
}

// Translated from 00cfe8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_CreateAgent` (named in the engine map, cdecl): allocates an entry in
/// `track` for the collidables `collidable_a` and `collidable_b` (with the bodies' records
/// `record_a` and `record_b`), links it into both collidables' entry lists, picks the agent
/// type's flags from the dispatcher (the first word of `ctx`, the collision input) and has the
/// agent create its data: a table at `+0x1bb0` (indexed by the collidables' signed bytes at
/// `+0x1a` as `a * 10 + b`) gives the byte stored at `+0xc` and the pointer stored in `ctx`'s
/// word `+0x60` (`+0x1c20` plus `0x40` per table value). `flags` (byte) is or-ed with 2 into the
/// entry's type, or with 4 when the agent type has a function at `+0x16d8` (its time is then the
/// mismatch value and its vector cleared; the agent data starts at `+0x30` instead of `+0x20`).
/// The agent's create function (`+0x16b0`) gets the agent input record, the entry and the data.
/// `word` is stored at `+8`. Returns the entry, whose size byte is 0x80.
#[allow(clippy::too_many_arguments)]
pub fn hk_agent_nn_machine_create_agent(
    e: &mut Engine,
    track: Ptr,
    collidable_a: Ptr,
    record_a: Ptr,
    collidable_b: Ptr,
    record_b: Ptr,
    flags: u32,
    agent_type: u32,
    ctx: Ptr,
    word: u32,
) -> Ptr {
    let (collidable_a, collidable_b) = (collidable_a.addr(), collidable_b.addr());
    let ctx = ctx.addr();
    let dispatcher = e.mem.u32(ctx);
    let entry = hk_agent_nn_machine_allocate_entry(e, track).addr();
    e.mem.set_u32(entry + 0x10, collidable_a);
    e.mem.set_u32(entry + 8, word);
    e.mem.set_u32(entry + 0x14, collidable_b);
    let count = e.mem.u16(collidable_a + 0x54);
    e.mem.set_u16(entry + 4, count);
    let count = e.mem.u16(collidable_b + 0x54);
    e.mem.set_u16(entry + 6, count);
    let slot_a = add_to_entry_list(e, collidable_a);
    let slot_b = add_to_entry_list(e, collidable_b);
    e.mem.set_u32(slot_a, entry);
    e.mem.set_u32(slot_b, entry);
    e.mem.set_u32(slot_a + 4, collidable_b);
    e.mem.set_u32(slot_b + 4, collidable_a);
    let (record_a, record_b) = (record_a.addr(), record_b.addr());
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, record_a);
        e.mem.set_u32(base + 4, record_b);
        e.mem.set_u32(base + 8, 0);
        e.mem.set_u32(base + 0xc, ctx);
        let entry_word = e.mem.u32(entry + 8);
        e.mem.set_u32(base + 0x10, entry_word);
        let (motion_a, motion_b) = (e.mem.u32(record_a + 8), e.mem.u32(record_b + 8));
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![base + 0x20, motion_a, motion_b],
        );
        // The signed bytes at +0x1a of the collidables pick the table cell.
        let cell = i32::from(e.mem.u8(collidable_b + 0x1a) as i8)
            + 10 * i32::from(e.mem.u8(collidable_a + 0x1a) as i8);
        let table = e.mem.u32(ctx);
        let value = e
            .mem
            .u8(table.wrapping_add(cell as u32).wrapping_add(0x1bb0));
        e.mem.set_u8(entry + 0xc, value);
        let table = e.mem.u32(ctx);
        let pointer = table
            .wrapping_add((i32::from(value as i8) << 6) as u32)
            .wrapping_add(0x1c20);
        e.mem.set_u32(ctx + 0x60, pointer);
        e.mem.set_u8(entry + 1, agent_type as u8);
        let record = dispatcher.wrapping_add(agent_type.wrapping_mul(0x50));
        let data = if e.mem.u32(record + 0x16d8) == 0 {
            e.mem.set_u8(entry, flags as u8 | 2);
            entry + 0x20
        } else {
            e.mem.set_u8(entry, flags as u8 | 4);
            copy_float(e, WARP_MISMATCH_TIME, entry + 0x18);
            for word in 0..4 {
                e.mem.set_u32(entry + 0x20 + 4 * word, 0);
            }
            entry + 0x30
        };
        let record = dispatcher.wrapping_add((agent_type & 0xff).wrapping_mul(0x50));
        let function = e.mem.u32(record + 0x16b0);
        e.call(function, &args![base, entry, data]);
    });
    e.mem.set_u8(entry + 3, 0x80);
    Ptr::new(entry)
}

// Translated from 00cff3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_ProcessTrack` (named in the engine map, cdecl): runs
/// [`hk_agent_nn_machine_process_agent`]'s work for every entry of `track` (its first word the
/// bytes used in the last sector, then the array of sector pointers: data at `+4`, size at `+8`;
/// the entries of a sector start at its first byte and each is as long as its byte `+3`
/// says), see [`process_entry`]. `passed_on` is the word the agent functions get at `+4` of the
/// record they are handed (`+0x16b8` takes it as its fourth argument). The record's first word
/// is the end of a list of sweep records the agents append to (a buffer in the frame, reset
/// for every entry): when an entry's agent added some, the object at the entry's `+8` is asked
/// (vtable slot `0x18`) with the two bodies' records as they were before the transform shapes
/// were resolved, the collision input and the record. When the global at `01269200` is 1 after
/// an entry the function stops at once. Finally a `MinumTim` record with the number of entries
/// passed on is appended to the thread's monitor stream.
pub fn hk_agent_nn_machine_process_track(e: &mut Engine, passed_on: u32, track: Ptr, ctx: Ptr) {
    let (track, ctx) = (track.addr(), ctx.addr());
    e.with_stack(0x3860, |e, frame| {
        let g = frame.addr();
        let (third, buffer) = (g + 0x7c0, g + 0x7d0);
        let counter = g + 0x24;
        e.mem.set_u32(third + 4, passed_on);
        e.mem.set_u32(counter, 0);
        let mut index = 0;
        while index < e.mem.i32(track + 8) {
            let sector = e.mem.u32(e.mem.u32(track + 4).wrapping_add(4 * index as u32));
            index += 1;
            let limit = if index == e.mem.i32(track + 8) {
                e.mem.u32(track).wrapping_add(sector)
            } else {
                sector.wrapping_add(0x200)
            };
            let mut entry = sector;
            while entry < limit {
                let first = e.mem.u32(entry + 0x10);
                let second = e.mem.u32(entry + 0x14);
                e.mem.set_u32(third, buffer);
                process_entry(e, g + 0x40, entry, ctx, third, Some(counter));
                if e.mem.u32(SKIP_CREATE) == 1 {
                    return;
                }
                if e.mem.u32(third) != buffer {
                    let object = e.mem.u32(entry + 8);
                    e.vcall(object, 0x18, &args![first, second, ctx, third]);
                }
                let size = u32::from(e.mem.u8(entry + 3));
                assert!(
                    size != 0,
                    "hkAgentNnMachine_ProcessTrack (00cff3c0): an entry of size 0 would loop forever"
                );
                entry = entry.wrapping_add(size);
            }
        }
        let count = e.mem.i32(counter);
        let index: u32 = e.global(MONITOR_SLOT);
        let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
        let cursor = e.mem.u32(stream + 4);
        if cursor < e.mem.u32(stream + 0xc) {
            e.mem.set_u32(cursor, MINIMUM_TIME_LABEL);
            e.mem.set_f32(cursor + 4, f64::from(count) as f32);
            e.mem.set_u32(stream + 4, cursor + 8);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_dc90,
            hk_agent_nn_machine_warp_time_in_agent(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00cf_dd70, hk_agent_nn_machine_find_agent(Ptr, Ptr) -> Ptr),
        entry!(
            0x00cf_e020,
            hk_agent_nn_machine_internal_deallocate_entry(Ptr, Ptr)
        ),
        entry!(0x00cf_e820, hk_agent_nn_machine_allocate_entry(Ptr) -> Ptr),
        entry!(
            0x00cf_e1d0,
            hk_agent_machine_process_transformed_shapes(Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00cf_eba0,
            hk_agent_nn_machine_update_shape_collection_filter(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_ea30,
            hk_agent_nn_machine_destroy_agent(Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_eca0,
            hk_agent_nn_machine_copy_and_relink_agent_entry(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cf_f290, hk_agent_nn_machine_append_track(Ptr, Ptr)),
        entry!(
            0x00cf_dbe0,
            hk_agent_nn_machine_invalidate_tim_in_agent(Ptr, Ptr)
        ),
        entry!(
            0x00cf_df60,
            hk_agent_nn_machine_get_agent_type(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cf_e370,
            hk_agent_nn_machine_inline_process_agent(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cf_e8a0,
            hk_agent_nn_machine_create_agent(Ptr, Ptr, Ptr, Ptr, Ptr, u32, u32, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cf_edd0,
            hk_agent_nn_machine_process_agent(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cf_f3c0,
            hk_agent_nn_machine_process_track(u32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const AGENT_FUNCTION: u32 = 0x0900_0040;
    const ALLOCATE_SLOT: u32 = 0x0900_0001;
    const FREE_SLOT: u32 = 0x0900_0002;
    /// A capacity word with the "storage not owned" bit: the array is never shrunk.
    const NOT_OWNED: u32 = 0x8000_0001;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// Registers a double at `address` that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &CallLog, address: u32, result: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A memory router whose allocator hands out heap blocks (slot 4) and records the blocks
    /// it is given back (slot 8); returns the allocator's address.
    fn install_router(e: &mut Engine, log: &CallLog) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        record(e, log, FREE_SLOT, 0);
        allocator
    }

    /// A dispatcher whose records for agent type index 1 hold `function` at all three tables.
    fn make_dispatcher(e: &mut Engine, function: u32) -> u32 {
        let dispatcher = e.mem.alloc(0x1800);
        for table in [0x16b4, 0x16c8, 0x16d4] {
            e.mem.set_u32(dispatcher + 0x50 + table, function);
        }
        dispatcher
    }

    fn make_entry(e: &mut Engine, kind: u8) -> u32 {
        let entry = e.mem.alloc(0x80);
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, 1);
        entry
    }

    #[test]
    fn warp_time_calls_the_agent_function_per_type() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 2);
        let (time_a, time_b) = (1.5f32, 2.5f32);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![
                entry,
                entry + 0x20,
                time_a.to_bits(),
                time_b.to_bits(),
                input
            ]]
        );
        // A type without a warp function does nothing.
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 3);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert!(log.borrow().is_empty());
        // A signalling NaN is quieted on its way to the function.
        e.mem.set_u8(entry, 14);
        e.call(
            0x00cf_dc90,
            &args![entry, f32::from_bits(0x7f80_0001), time_b, input],
        );
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][2], 0x7fc0_0001);
        // Without a function in the table nothing is called.
        e.mem.set_u32(dispatcher + 0x50 + 0x16d4, 0);
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 2);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn warp_time_of_types_four_and_twelve_keeps_a_matching_time() {
        let mut e = engine();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 4);
        e.mem.set_f32(entry + 0x18, 1.5);
        e.call(0x00cf_dc90, &args![entry, 1.5f32, 9.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), 9.0);
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][1], entry + 0x30);
        // A different stored time becomes the fallback constant.
        e.mem.set_u8(entry, 12);
        e.call(0x00cf_dc90, &args![entry, 4.0f32, 9.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        // A NaN never matches.
        e.mem.set_f32(entry + 0x18, f32::NAN);
        e.call(0x00cf_dc90, &args![entry, f32::NAN, 7.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        assert_eq!(calls_to(&log, AGENT_FUNCTION).len(), 3);
    }

    /// A collidable with `count` collision entries, each (entry, partner) pair given.
    fn make_collidable(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let collidable = e.mem.alloc(0x60);
        let data = e.mem.alloc(8 * pairs.len().max(1) as u32);
        for (index, (entry, partner)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *entry);
            e.mem.set_u32(data + 8 * index as u32 + 4, *partner);
        }
        e.mem.set_u32(collidable + 0x50, data);
        e.mem.set_i32(collidable + 0x54, pairs.len() as i32);
        collidable
    }

    #[test]
    fn find_agent_searches_the_shorter_list() {
        let mut e = engine();
        let a = make_collidable(&mut e, &[(0x1111, 0x9999), (0x2222, 0x8888)]);
        let b = make_collidable(&mut e, &[(0x3333, a)]);
        // b has fewer entries: its list is searched for a.
        assert_eq!(e.call(0x00cf_dd70, &args![a, b]).u32(), 0x3333);
        assert_eq!(e.call(0x00cf_dd70, &args![b, a]).u32(), 0x3333);
        // Equal lengths: the first one's list is searched for the second.
        let d = make_collidable(&mut e, &[(0x5555, 0x1234)]);
        let c = make_collidable(&mut e, &[(0x4444, d)]);
        assert_eq!(e.call(0x00cf_dd70, &args![c, d]).u32(), 0x4444);
        assert_eq!(e.call(0x00cf_dd70, &args![d, c]).u32(), 0);
        assert_eq!(e.call(0x00cf_dd70, &args![c, a]).u32(), 0);
        let empty = make_collidable(&mut e, &[]);
        assert_eq!(e.call(0x00cf_dd70, &args![empty, a]).u32(), 0);
    }

    /// A track with the given sectors (already allocated) and used bytes in the last one.
    fn make_track(e: &mut Engine, sectors: &[u32], used: u32, capacity_word: u32) -> u32 {
        let track = e.mem.alloc(0x20);
        let data = e.mem.alloc(4 * sectors.len().max(1) as u32);
        for (index, sector) in sectors.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *sector);
        }
        e.mem.set_u32(track, used);
        e.mem.set_u32(track + 4, data);
        e.mem.set_u32(track + 8, sectors.len() as u32);
        e.mem.set_u32(track + 0xc, capacity_word);
        track
    }

    /// Entry `index` of a sector linking collidables `a` and `b` at element `element`.
    fn init_entry(e: &mut Engine, entry: u32, a: u32, b: u32, element: u16, marker: u32) {
        e.mem.set_u8(entry, 2);
        e.mem.set_u8(entry + 1, 1);
        e.mem.set_u16(entry + 4, element);
        e.mem.set_u16(entry + 6, element);
        e.mem.set_u32(entry + 8, marker);
        e.mem.set_u32(entry + 0x10, a);
        e.mem.set_u32(entry + 0x14, b);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        e.mem.set_u32(a_data + 8 * u32::from(element), entry);
        e.mem.set_u32(a_data + 8 * u32::from(element) + 4, b);
        e.mem.set_u32(b_data + 8 * u32::from(element), entry);
        e.mem.set_u32(b_data + 8 * u32::from(element) + 4, a);
    }

    #[test]
    fn allocate_entry_uses_room_in_the_last_sector() {
        let mut e = engine();
        let sector = e.mem.alloc(0x200);
        let track = make_track(&mut e, &[sector], 0x100, 0x8001);
        assert_eq!(e.call(0x00cf_e820, &args![track]).u32(), sector + 0x100);
        assert_eq!(e.mem.u32(track), 0x180);
    }

    #[test]
    fn allocate_entry_adds_a_sector_when_the_last_is_full() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            reserve_log
                .borrow_mut()
                .push((ARRAY_RESERVE_MORE, a.to_vec()));
            // Room for another sector.
            let room = e.mem.alloc(16);
            let old = e.mem.u32(a[1]);
            e.mem.set_u32(room, e.mem.u32(old));
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let sector = e.mem.alloc(0x200);
        // Size 1, capacity 1: full, so the array grows.
        let track = make_track(&mut e, &[sector], 0x200, 1);
        let entry = e.call(0x00cf_e820, &args![track]).u32();
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(track + 8), 2);
        let data = e.mem.u32(track + 4);
        assert_eq!(e.mem.u32(data), sector);
        assert_eq!(e.mem.u32(data + 4), entry);
        assert_ne!(entry, sector);
        // With room in the array it is not grown.
        e.mem.set_u32(track, 0x200);
        e.call(0x00cf_e820, &args![track]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(track + 8), 3);
    }

    #[test]
    fn deallocate_entry_moves_the_last_entry_into_the_gap() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        let sector = e.mem.alloc(0x200);
        // Both entries sit between the same two collidables, at elements 0 and 1.
        let a = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        init_entry(&mut e, sector + 0x80, a, b, 1, 0xbb);
        // A negative capacity word (storage not owned) keeps the array.
        let track = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(sector + 8), 0xbb);
        assert_eq!(e.mem.u16(sector + 4), 1);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        assert_eq!(e.mem.u32(a_data + 8), sector);
        assert_eq!(e.mem.u32(b_data + 8), sector);
        assert!(calls_to(&log, ARRAY_REDUCE).is_empty());
        // Removing the last entry only releases it; the emptied sector goes back.
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track), SECTOR_SIZE);
        assert_eq!(e.mem.u32(track + 8), 0);
        assert_eq!(calls_to(&log, FREE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, FREE_SLOT)[0][1..], [sector, 0x200]);
    }

    #[test]
    fn deallocate_entry_shrinks_an_owned_sector_array() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        // One sector, capacity 16: after the release the rounded capacity 4 + 3 + ... is below.
        let track = make_track(&mut e, &[sector], 0x80, 16);
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track + 8), 0);
        let reduce = calls_to(&log, ARRAY_REDUCE);
        assert_eq!(reduce.len(), 1);
        assert_eq!(reduce[0], vec![allocator, track + 4, 4, 0, 4]);
    }

    #[test]
    fn rounded_capacity_follows_the_smearing_sequence() {
        assert_eq!(rounded_capacity(0), 4);
        assert_eq!(rounded_capacity(1), 8);
        assert_eq!(rounded_capacity(5), 16);
        assert!(rounded_capacity(0x7fff_fffd) < 1);
    }

    #[test]
    fn destroy_agent_unlinks_the_entry_from_both_collidables() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        e.mem.set_u32(a + 0x58, 16);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        init_entry(&mut e, sector + 0x80, a, b, 1, 0xbb);
        let track = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        e.call(0x00cf_ea30, &args![track, sector, dispatcher, 0x55u32]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![sector, sector + 0x20, 0xaa, 0x55, dispatcher]]
        );
        // The collidables lost an element; the surviving entry now sits at element 0 of both
        // and was moved into the freed slot.
        assert_eq!(e.mem.i32(a + 0x54), 1);
        assert_eq!(e.mem.i32(b + 0x54), 1);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(sector + 8), 0xbb);
        assert_eq!(e.mem.u16(sector + 4), 0);
        assert_eq!(e.mem.u16(sector + 6), 0);
        let a_data = e.mem.u32(a + 0x50);
        assert_eq!(e.mem.u32(a_data), sector);
        // Only `a` has a capacity larger than the rounded one (8 < 16).
        let reduce = calls_to(&log, ARRAY_REDUCE);
        assert_eq!(reduce, vec![vec![allocator, a + 0x50, 8, 0, 8]]);
    }

    #[test]
    fn destroy_agent_of_an_unknown_type_only_unlinks() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        e.mem.set_u8(sector, 7);
        let track = make_track(&mut e, &[sector], 0x80, NOT_OWNED);
        e.call(0x00cf_ea30, &args![track, sector, dispatcher, 0u32]);
        assert!(calls_to(&log, AGENT_FUNCTION).is_empty());
        assert_eq!(e.mem.i32(a + 0x54), 0);
        assert_eq!(e.mem.i32(b + 0x54), 0);
        assert_eq!(e.mem.u32(track), SECTOR_SIZE);
    }

    #[test]
    fn copy_and_relink_moves_an_entry_into_a_new_slot() {
        let mut e = engine();
        let sector = e.mem.alloc(0x200);
        let other = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, other, a, b, 0, 0xcc);
        let track = make_track(&mut e, &[sector], 0x80, 0x8001);
        let copy = e.call(0x00cf_eca0, &args![track, other]).u32();
        assert_eq!(copy, sector + 0x80);
        assert_eq!(e.mem.u32(track), 0x100);
        assert_eq!(e.mem.u32(copy + 8), 0xcc);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        assert_eq!(e.mem.u32(a_data), copy);
        assert_eq!(e.mem.u32(b_data), copy);
    }

    #[test]
    fn append_track_moves_entries_until_the_target_is_full() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_INSERT_AT, 0);
        let target_sector = e.mem.alloc(0x200);
        let source_sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
        init_entry(&mut e, target_sector, a, b, 0, 1);
        init_entry(&mut e, target_sector + 0x80, a, b, 1, 2);
        init_entry(&mut e, source_sector, a, b, 2, 3);
        init_entry(&mut e, source_sector + 0x80, a, b, 3, 4);
        init_entry(&mut e, source_sector + 0x100, a, b, 4, 5);
        let target = make_track(&mut e, &[target_sector], 0x100, NOT_OWNED);
        let source = make_track(&mut e, &[source_sector], 0x180, NOT_OWNED);
        e.call(0x00cf_f290, &args![target, source]);
        // Two entries moved (the last ones first); the target's sector is full.
        assert_eq!(e.mem.u32(target_sector + 0x100 + 8), 5);
        assert_eq!(e.mem.u32(target_sector + 0x180 + 8), 4);
        assert_eq!(e.mem.u32(target), 0x80);
        assert_eq!(e.mem.u32(source), SECTOR_SIZE);
        assert_eq!(e.mem.u32(source + 8), 0);
        let insert = calls_to(&log, ARRAY_INSERT_AT);
        assert_eq!(insert.len(), 1);
        assert_eq!(insert[0][0], target + 4);
        assert_eq!(insert[0][2..], [1, e.mem.u32(source + 4), 1]);
        let a_data = e.mem.u32(a + 0x50);
        assert_eq!(e.mem.u32(a_data + 8 * 4), target_sector + 0x100);
        assert_eq!(e.mem.u32(a_data + 8 * 3), target_sector + 0x180);
    }

    #[test]
    fn append_track_of_an_empty_source_changes_nothing() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_INSERT_AT, 0);
        let sector = e.mem.alloc(0x200);
        let target = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        let source = make_track(&mut e, &[], SECTOR_SIZE, NOT_OWNED);
        e.call(0x00cf_f290, &args![target, source]);
        assert_eq!(e.mem.u32(target), 0x100);
        assert!(calls_to(&log, ARRAY_INSERT_AT).is_empty());
    }

    struct TransformScene {
        body: u32,
        record: u32,
        shape: u32,
        child: u32,
    }

    /// A body whose shape is a transform shape (type 15) with a quarter turn about z and the
    /// translation (1, 2, 3), over a plain child shape; the body's record scales by (2, 3, 4)
    /// and translates by (10, 20, 30).
    fn transform_scene(e: &mut Engine) -> TransformScene {
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0xc, 5);
        let shape = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc, 0xf);
        e.mem.set_u32(shape + 0x14, child);
        store_vec(e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, shape + 0x40, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, shape + 0x50, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, shape + 0x60, [1.0, 2.0, 3.0, 0.0]);
        let record = e.mem.alloc(0xb0);
        store_vec(e, record, [2.0, 0.0, 0.0, 0.0]);
        store_vec(e, record + 0x10, [0.0, 3.0, 0.0, 0.0]);
        store_vec(e, record + 0x20, [0.0, 0.0, 4.0, 0.0]);
        store_vec(e, record + 0x30, [10.0, 20.0, 30.0, 0.0]);
        store_vec(e, record + 0x40, [7.0, 8.0, 9.0, 0.5]);
        store_vec(e, record + 0x50, [6.0, 5.0, 4.0, 0.25]);
        store_vec(e, record + 0x80, [5.0, 6.0, 7.0, 0.0]);
        store_vec(e, record + 0x90, [0.1, 0.2, 0.3, 0.4]);
        e.mem.set_f32(record + 0xa0, 0.75);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, record);
        TransformScene {
            body,
            record,
            shape,
            child,
        }
    }

    #[test]
    fn process_transformed_shapes_composes_one_level() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, QUATERNION_SET_MUL, 0);
        let scene = transform_scene(&mut e);
        let bodies = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x2c0);
        let changed = e.mem.alloc(8);
        let last = e
            .call(
                0x00cf_e1d0,
                &args![scene.body, bodies, transforms, 4u32, changed],
            )
            .u32();
        // The chain ends at the one body written, which describes the plain child.
        assert_eq!(last, bodies);
        assert_eq!(e.mem.u32(bodies), scene.child);
        assert_eq!(e.mem.u32(bodies + 4), 0);
        assert_eq!(e.mem.u32(bodies + 8), transforms);
        assert_eq!(e.mem.u32(bodies + 0xc), scene.body);
        assert_eq!(e.mem.u8(changed), 8);
        // The record's scale applied to the shape's columns and translation.
        assert_eq!(load_vec(&e, transforms), [0.0, 3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x10), [-2.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x20), [0.0, 0.0, 4.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x30), [12.0, 26.0, 42.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x40), [7.0, 8.0, 9.0, 0.5]);
        assert_eq!(load_vec(&e, transforms + 0x50), [6.0, 5.0, 4.0, 0.25]);
        // (5, 6, 7) - (1, 2, 3) = (4, 4, 4) seen through the transposed rotation.
        assert_eq!(load_vec(&e, transforms + 0x80), [4.0, -4.0, 4.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x90), [0.1, 0.2, 0.3, 0.4]);
        assert_eq!(e.mem.f32(transforms + 0xa0), 0.75);
        let multiplies = calls_to(&log, QUATERNION_SET_MUL);
        assert_eq!(
            multiplies,
            vec![
                vec![transforms + 0x60, scene.record + 0x60, scene.shape + 0x20],
                vec![transforms + 0x70, scene.record + 0x70, scene.shape + 0x20],
            ]
        );
    }

    #[test]
    fn process_transformed_shapes_returns_a_plain_body_untouched() {
        let mut e = engine();
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0xc, 5);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, child);
        let changed = e.mem.alloc(8);
        let last = e
            .call(0x00cf_e1d0, &args![body, 0u32, 0u32, 4u32, changed])
            .u32();
        assert_eq!(last, body);
        assert_eq!(e.mem.u8(changed), 0);
    }

    #[test]
    fn update_shape_collection_filter_calls_the_dispatcher_function() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 4);
        e.mem.set_u32(entry + 8, 0x77);
        e.mem.set_u32(entry + 0x10, 0x1000);
        e.mem.set_u32(entry + 0x14, 0x2000);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![entry, entry + 0x30, 0x1000, 0x2000, input, 0x77, 0x99]]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 6);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][1], entry + 0x20);
        e.mem.set_u8(entry, 5);
        log.borrow_mut().clear();
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn update_shape_collection_filter_resolves_transform_shapes_first() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        record(&mut e, &log, QUATERNION_SET_MUL, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let scene = transform_scene(&mut e);
        let plain_shape = e.mem.alloc(0x40);
        e.mem.set_u32(plain_shape + 0xc, 5);
        let plain = e.mem.alloc(0x10);
        e.mem.set_u32(plain, plain_shape);
        let entry = make_entry(&mut e, 12);
        e.mem.set_u32(entry + 0x10, scene.body);
        e.mem.set_u32(entry + 0x14, plain);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        let calls = calls_to(&log, AGENT_FUNCTION);
        assert_eq!(calls.len(), 1);
        // Type 12 continues as type 4 (agent data at +0x30), the first body is the resolved
        // one (a new body over the child shape), the second the untouched plain one.
        assert_eq!(calls[0][1], entry + 0x30);
        assert_ne!(calls[0][2], scene.body);
        assert_eq!(e.mem.u32(calls[0][2]), scene.child);
        assert_eq!(calls[0][3], plain);
        assert_eq!(calls_to(&log, QUATERNION_SET_MUL).len(), 2);
    }

    #[test]
    fn the_resolved_bodies_sit_in_the_aligned_exe_frame() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        record(&mut e, &log, QUATERNION_SET_MUL, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let scene = transform_scene(&mut e);
        let plain_shape = e.mem.alloc(0x40);
        e.mem.set_u32(plain_shape + 0xc, 5);
        let plain = e.mem.alloc(0x10);
        e.mem.set_u32(plain, plain_shape);
        let entry = make_entry(&mut e, 12);
        e.mem.set_u32(entry + 0x10, scene.body);
        e.mem.set_u32(entry + 0x14, plain);
        let raw = e.next_stack_local(0x630);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        let frame = aligned_frame_at(raw, 0x620);
        // The first shape resolves into the first body array at +0x20.
        let resolved = calls_to(&log, AGENT_FUNCTION)[0][2];
        assert!((frame + 0x20..frame + 0x60).contains(&resolved));
    }

    // ---- InvalidateTimInAgent, GetAgentType, CreateAgent, ProcessAgent, ProcessTrack ----

    const PASS_FUNCTION: u32 = 0x0900_0050;
    const PREPARE_FUNCTION: u32 = 0x0900_0051;
    const PROCESS_FUNCTION: u32 = 0x0900_0052;
    const CREATE_FUNCTION: u32 = 0x0900_0053;
    const OBJECT_SLOT: u32 = 0x0900_0054;

    #[test]
    fn invalidate_tim_calls_the_agent_function_per_type() {
        let mut e = engine();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = e.mem.alloc(0x1800);
        e.mem.set_u32(dispatcher + 0x50 + 0x16d0, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 2);
        e.call(0x00cf_dbe0, &args![entry, input]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![entry, entry + 0x20, input]]
        );
        // Types 4 and 12 clear their time and vector first and pass the data at +0x30.
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 12);
        e.mem.set_f32(entry + 0x18, 5.0);
        for word in 0..4 {
            e.mem.set_u32(entry + 0x20 + 4 * word, 0x1111);
        }
        e.call(0x00cf_dbe0, &args![entry, input]);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        for word in 0..4 {
            assert_eq!(e.mem.u32(entry + 0x20 + 4 * word), 0);
        }
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![entry, entry + 0x30, input]]
        );
        // Other types do nothing, and a missing function is not called.
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 3);
        e.call(0x00cf_dbe0, &args![entry, input]);
        e.mem.set_u8(entry, 14);
        e.mem.set_u32(dispatcher + 0x50 + 0x16d0, 0);
        e.call(0x00cf_dbe0, &args![entry, input]);
        assert!(log.borrow().is_empty());
    }

    /// A body (collidable) whose shape has type `kind`.
    fn typed_body(e: &mut Engine, kind: u32) -> u32 {
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, kind);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        body
    }

    #[test]
    fn get_agent_type_looks_up_the_shape_pair_and_the_swap_flag() {
        let mut e = engine();
        let dispatcher = e.mem.alloc(0x2000);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, dispatcher);
        let (a, b) = (typed_body(&mut e, 3), typed_body(&mut e, 5));
        let out = e.mem.alloc(8);
        // (3, 5) is cell 0x65 of the table at +0xeb0, (5, 3) is cell 0xa3.
        e.mem.set_u8(dispatcher + 0xeb0 + 0x65, 7);
        e.mem.set_u8(dispatcher + 0xeb0 + 0xa3, 9);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (7, 0));
        // The record of agent type 7 wants the pair swapped: the swapped pair's type.
        e.mem.set_u32(dispatcher + 7 * 0x50 + 0x16f0, 2);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (9, 1));
        // With the input's word +0x14 set the other table (at +0x12b0) is used.
        e.mem.set_u32(input + 0x14, 1);
        e.mem.set_u8(dispatcher + 0x12b0 + 0x65, 4);
        e.mem.set_u8(dispatcher + 0x12b0 + 0xa3, 6);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (4, 0));
        e.mem.set_u32(dispatcher + 4 * 0x50 + 0x16f0, 2);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (6, 1));
    }

    #[test]
    fn get_agent_type_of_agent_one_asks_a_per_type_byte() {
        let mut e = engine();
        let dispatcher = e.mem.alloc(0x2000);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, dispatcher);
        let (a, b) = (typed_body(&mut e, 3), typed_body(&mut e, 5));
        let out = e.mem.alloc(8);
        e.mem.set_u8(dispatcher + 0xeb0 + 0x65, 1);
        e.mem.set_u8(dispatcher + 0xeb0 + 0xa3, 9);
        // Cell (3 + 0xd) * 0x20 + 5 holds 2, which selects the byte at (2 + 0x7c) * 0x14.
        e.mem.set_u8(dispatcher + 0x205, 2);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (1, 0));
        e.mem.set_u8(dispatcher + 0x7e * 0x14, 1);
        e.call(0x00cf_df60, &args![a, b, input, out, out + 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (9, 1));
    }

    /// What an agent function double saw of the agent input record at its first argument.
    #[derive(Clone, Debug, PartialEq)]
    struct Snapshot {
        address: u32,
        args: Vec<u32>,
        /// The first five words of the record.
        words: Vec<u32>,
        /// The four words of the records its first two words point at.
        first: Vec<u32>,
        second: Vec<u32>,
        pending: f32,
        vector: Vec<f32>,
    }

    type Snapshots = Rc<RefCell<Vec<Snapshot>>>;

    /// Registers a double at `address` that records a [`Snapshot`] of the record at its first
    /// argument.
    fn snapshot(e: &mut Engine, log: &Snapshots, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let words: Vec<u32> = (0..5).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let first = (0..4).map(|i| e.mem.u32(words[0] + 4 * i)).collect();
            let second = (0..4).map(|i| e.mem.u32(words[1] + 4 * i)).collect();
            let vector = (0..4).map(|i| e.mem.f32(a[0] + 0x70 + 4 * i)).collect();
            log.borrow_mut().push(Snapshot {
                address,
                args: a.to_vec(),
                words,
                first,
                second,
                pending: e.mem.f32(a[0] + 0x60),
                vector,
            });
            Ret::default()
        });
    }

    /// What the process tests build their surroundings from.
    struct Process {
        dispatcher: u32,
        ctx: u32,
        third: u32,
        shapes: [u32; 2],
        bodies: [u32; 2],
        motions: [u32; 2],
        object: u32,
        log: CallLog,
        snapshots: Snapshots,
    }

    /// Two bodies with motions whose sweep vector over the step 0.25 is `(2, 1, -1, 8)`, a
    /// collision input (limit 1, times 1 and 0.5, step 0.25) and a dispatcher whose agent type 1
    /// has all functions and agent type 2 all but the one at `+0x16d8`.
    fn process_world(e: &mut Engine) -> Process {
        let log = new_log();
        let snapshots: Snapshots = Rc::default();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        e.mem.set_u32(SKIP_CREATE, 0);
        e.mem.set_u32(UNSET_DISTANCE, 0xff7f_ffee);
        record(e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        record(e, &log, SWEPT_LERP2, 0);
        record(e, &log, PASS_FUNCTION, 0);
        record(e, &log, OBJECT_SLOT, 0);
        for address in [PROCESS_FUNCTION, PREPARE_FUNCTION, CREATE_FUNCTION] {
            snapshot(e, &snapshots, address);
        }
        let dispatcher = e.mem.alloc(0x2000);
        for (table, function) in [
            (0x16b0, CREATE_FUNCTION),
            (0x16b8, PASS_FUNCTION),
            (0x16d8, PREPARE_FUNCTION),
            (0x16dc, PROCESS_FUNCTION),
        ] {
            e.mem.set_u32(dispatcher + 0x50 + table, function);
        }
        e.mem.set_u32(dispatcher + 0xa0 + 0x16b0, CREATE_FUNCTION);
        e.mem.set_u32(dispatcher + 0xa0 + 0x16dc, PROCESS_FUNCTION);
        // Motion states: step * (A.5c = 2) = 0.5 and step * (B.5c = 4) = 1.
        let motions = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        let (a, b) = (motions[0], motions[1]);
        e.mem.set_f32(a + 0x40, 4.0);
        e.mem.set_f32(a + 0x44, 2.0);
        e.mem.set_f32(a + 0x5c, 2.0);
        e.mem.set_f32(a + 0x9c, 2.0);
        e.mem.set_f32(a + 0xa0, 3.0);
        e.mem.set_f32(b + 0x48, 1.0);
        e.mem.set_f32(b + 0x5c, 4.0);
        e.mem.set_f32(b + 0x9c, 1.0);
        e.mem.set_f32(b + 0xa0, 5.0);
        let shapes = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        let bodies = [e.mem.alloc(0x60), e.mem.alloc(0x60)];
        for side in 0..2 {
            e.mem.set_u32(bodies[side], shapes[side]);
            e.mem.set_u32(bodies[side] + 4, 0x5b00 + side as u32);
            e.mem.set_u32(bodies[side] + 8, motions[side]);
        }
        let object = e.mem.alloc(0x40);
        let ctx = e.mem.alloc(0x100);
        e.mem.set_u32(ctx, dispatcher);
        e.mem.set_f32(ctx + 8, 1.0);
        e.mem.set_f32(ctx + 0x50, 1.0);
        e.mem.set_f32(ctx + 0x54, 0.5);
        e.mem.set_f32(ctx + 0x58, 0.25);
        e.mem.set_u32(ctx + 0x60, object);
        let third = e.mem.alloc(8);
        e.mem.set_u32(third, 0x7777);
        e.mem.set_u32(third + 4, 0x6666);
        Process {
            dispatcher,
            ctx,
            third,
            shapes,
            bodies,
            motions,
            object,
            log,
            snapshots,
        }
    }

    /// An agent entry of agent type 1 with pass-on flag `flag`, time `time`, vector
    /// `(1, 2, 3, 20)` at `+0x20` and the word `0x1234` at `+8`.
    fn swept_entry(e: &mut Engine, w: &Process, kind: u8, flag: u8, time: f32) -> u32 {
        let entry = make_entry(e, kind);
        e.mem.set_u8(entry + 2, flag);
        e.mem.set_u8(entry + 3, 0x40);
        e.mem.set_u32(entry + 8, 0x1234);
        e.mem.set_u32(entry + 0x10, w.bodies[0]);
        e.mem.set_u32(entry + 0x14, w.bodies[1]);
        e.mem.set_f32(entry + 0x18, time);
        for (lane, value) in [1.0f32, 2.0, 3.0, 20.0].into_iter().enumerate() {
            e.mem.set_f32(entry + 0x20 + 4 * lane as u32, value);
        }
        entry
    }

    #[test]
    fn process_agent_hands_a_type_two_entry_the_sweep_vector() {
        let mut e = engine();
        let w = process_world(&mut e);
        let entry = swept_entry(&mut e, &w, 2, 1, 1.0);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        let seen = w.snapshots.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].address, PROCESS_FUNCTION);
        assert_eq!(seen[0].args[1..], [entry, entry + 0x20, 0, w.third]);
        assert_eq!(
            seen[0].words,
            vec![w.bodies[0], w.bodies[1], 0, w.ctx, 0x1234]
        );
        // The offset of A over its half step plus B over its whole step; the fourth lane is
        // 2 * 0.5 * 3 + 1 * 1 * 5.
        assert_eq!(seen[0].vector, vec![2.0, 1.0, -1.0, 8.0]);
        let relative = calls_to(&w.log, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [w.motions[0], w.motions[1]]);
    }

    #[test]
    fn process_agent_passes_a_type_four_entry_of_equal_time_on() {
        let mut e = engine();
        let w = process_world(&mut e);
        let entry = swept_entry(&mut e, &w, 4, 1, 1.0);
        let silent = swept_entry(&mut e, &w, 4, 0, 1.0);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        e.call(0x00cf_edd0, &args![silent, w.ctx, w.third]);
        // The dot product is (3 * -1 + 8) + (2 * 1 + 1 * 2) = 9; 20 - 9 remain, which is more
        // than the limit of 1: the entry is passed on (when it asks to be) at the second time.
        for at in [entry, silent] {
            assert_eq!(e.mem.f32(at + 0x18), 0.5);
            assert_eq!(e.mem.f32(at + 0x2c), 11.0);
        }
        assert_eq!(
            calls_to(&w.log, PASS_FUNCTION),
            vec![vec![entry, entry + 0x30, 0x1234, 0x6666]]
        );
        assert!(w.snapshots.borrow().is_empty());
    }

    #[test]
    fn process_agent_runs_the_agent_for_a_type_four_entry_with_much_distance_left() {
        let mut e = engine();
        let w = process_world(&mut e);
        e.mem.set_f32(w.ctx + 8, 100.0);
        let entry = swept_entry(&mut e, &w, 4, 1, 1.0);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        let seen = w.snapshots.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].address, PROCESS_FUNCTION);
        assert_eq!(
            seen[0].args[1..],
            [entry, entry + 0x30, entry + 0x20, w.third]
        );
        assert_eq!(seen[0].pending, 11.0);
        // The distance in the entry is not updated for the agent.
        assert_eq!(e.mem.f32(entry + 0x2c), 20.0);
        assert_eq!(e.mem.f32(entry + 0x18), 0.5);
        assert!(calls_to(&w.log, PASS_FUNCTION).is_empty());
    }

    #[test]
    fn process_agent_restarts_a_type_four_entry_of_another_time_when_the_object_has_no_state() {
        let mut e = engine();
        let w = process_world(&mut e);
        let entry = swept_entry(&mut e, &w, 4, 1, 0.25);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        assert_eq!(e.mem.f32(entry + 0x18), 0.5);
        assert_eq!(e.mem.u32(entry + 0x2c), 0xff7f_ffee);
        for word in 0..3 {
            assert_eq!(e.mem.u32(entry + 0x20 + 4 * word), 0);
        }
        let seen = w.snapshots.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].address, PROCESS_FUNCTION);
        assert_eq!(seen[0].pending.to_bits(), 0xff7f_ffee);
        assert!(calls_to(&w.log, SWEPT_LERP2).is_empty());
    }

    #[test]
    fn process_agent_prepares_a_type_four_entry_of_another_time_at_the_input_time() {
        let mut e = engine();
        let w = process_world(&mut e);
        e.mem.set_u32(w.object + 0x10, 1);
        let entry = swept_entry(&mut e, &w, 4, 1, 0.25);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        let lerps = calls_to(&w.log, SWEPT_LERP2);
        assert_eq!(lerps.len(), 2);
        assert_eq!(lerps[0][..2], [w.motions[0] + 0x40, 1.0f32.to_bits()]);
        assert_eq!(lerps[1][..2], [w.motions[1] + 0x40, 1.0f32.to_bits()]);
        let relative = calls_to(&w.log, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [lerps[0][2], lerps[1][2]]);
        let seen = w.snapshots.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].address, PREPARE_FUNCTION);
        assert_eq!(seen[0].args[1..], [entry, entry + 0x30, entry + 0x20]);
        assert_eq!(seen[0].words[2..], [0, w.ctx, 0x1234]);
        // The two records: the bodies' shape and word +4, the interpolated transform and the body.
        assert_eq!(
            seen[0].first,
            vec![w.shapes[0], 0x5b00, lerps[0][2], w.bodies[0]]
        );
        assert_eq!(
            seen[0].second,
            vec![w.shapes[1], 0x5b01, lerps[1][2], w.bodies[1]]
        );
        // Then the entry is brought to the second time and passed on.
        assert_eq!(e.mem.f32(entry + 0x18), 0.5);
        assert_eq!(e.mem.f32(entry + 0x2c), 11.0);
        assert_eq!(calls_to(&w.log, PASS_FUNCTION).len(), 1);
    }

    #[test]
    fn process_agent_asks_the_object_of_a_type_six_entry() {
        let mut e = engine();
        let w = process_world(&mut e);
        let asked = make_slot_object(&mut e);
        let entry = swept_entry(&mut e, &w, 6, 0, 0.0);
        e.mem.set_u32(entry + 0x20, asked);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        assert_eq!(
            calls_to(&w.log, OBJECT_SLOT),
            vec![vec![asked, w.bodies[0], w.bodies[1], w.ctx, w.third]]
        );
    }

    /// An object whose vtable slot `0x18` is the recording double.
    fn make_slot_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, OBJECT_SLOT);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn process_agent_resolves_the_transform_shapes_of_a_type_twelve_entry() {
        let mut e = engine();
        let w = process_world(&mut e);
        // Plain shapes (type 0): nothing to resolve, then the entry is handled as type 4.
        let entry = swept_entry(&mut e, &w, 12, 1, 1.0);
        e.call(0x00cf_edd0, &args![entry, w.ctx, w.third]);
        assert_eq!(e.mem.u8(entry), 12);
        assert_eq!(e.mem.f32(entry + 0x2c), 11.0);
        assert_eq!(calls_to(&w.log, PASS_FUNCTION).len(), 1);
        // Types without a handler do nothing.
        let other = swept_entry(&mut e, &w, 3, 1, 1.0);
        e.call(0x00cf_edd0, &args![other, w.ctx, w.third]);
        assert_eq!(e.mem.f32(other + 0x18), 1.0);
    }

    #[test]
    fn inline_process_agent_counts_every_entry_it_passes_on() {
        let mut e = engine();
        let w = process_world(&mut e);
        let counter = e.mem.alloc(4);
        let entry = swept_entry(&mut e, &w, 4, 1, 1.0);
        let silent = swept_entry(&mut e, &w, 4, 0, 1.0);
        e.call(0x00cf_e370, &args![entry, w.ctx, counter, w.third]);
        e.call(0x00cf_e370, &args![silent, w.ctx, counter, w.third]);
        assert_eq!(e.mem.u32(counter), 2);
        assert_eq!(
            calls_to(&w.log, PASS_FUNCTION),
            vec![vec![entry, entry + 0x30, 0x1234, 0x6666]]
        );
        assert_eq!(e.mem.f32(entry + 0x2c), 11.0);
        // A type 2 entry is not counted.
        let plain = swept_entry(&mut e, &w, 2, 1, 1.0);
        e.call(0x00cf_e370, &args![plain, w.ctx, counter, w.third]);
        assert_eq!(e.mem.u32(counter), 2);
        assert_eq!(w.snapshots.borrow().len(), 1);
    }

    /// The monitor stream (cursor at `+4`, limit at `+0xc`) and the TLS slot.
    fn monitor_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// The `(label, float)` records written to the stream since it was made.
    fn minimum_times(e: &Engine, stream: u32) -> Vec<(u32, f32)> {
        let end = e.mem.u32(stream + 4);
        (stream + 0x40..end)
            .step_by(8)
            .map(|at| (e.mem.u32(at), e.mem.f32(at + 4)))
            .collect()
    }

    /// Writes the entry the way `swept_entry` does at `at` (inside a sector) with size `size`.
    fn write_entry(e: &mut Engine, w: &Process, at: u32, (kind, flag, size): (u8, u8, u8)) {
        e.mem.set_u8(at, kind);
        e.mem.set_u8(at + 1, 1);
        e.mem.set_u8(at + 2, flag);
        e.mem.set_u8(at + 3, size);
        e.mem.set_u32(at + 8, 0x1234);
        e.mem.set_u32(at + 0x10, w.bodies[0]);
        e.mem.set_u32(at + 0x14, w.bodies[1]);
        e.mem.set_f32(at + 0x18, 1.0);
        for (lane, value) in [1.0f32, 2.0, 3.0, 20.0].into_iter().enumerate() {
            e.mem.set_f32(at + 0x20 + 4 * lane as u32, value);
        }
    }

    #[test]
    fn process_track_processes_every_entry_of_every_sector() {
        let mut e = engine();
        let w = process_world(&mut e);
        let stream = monitor_stream(&mut e);
        let (first, last) = (e.mem.alloc(0x200), e.mem.alloc(0x200));
        // The first sector is full (two entries of 0x100 bytes), the last has 0x80 bytes used.
        write_entry(&mut e, &w, first, (4, 1, 0x80));
        write_entry(&mut e, &w, first + 0x80, (4, 0, 0x80));
        write_entry(&mut e, &w, first + 0x100, (4, 1, 0x80));
        write_entry(&mut e, &w, first + 0x180, (3, 1, 0x80));
        write_entry(&mut e, &w, last, (2, 1, 0x40));
        write_entry(&mut e, &w, last + 0x40, (6, 0, 0x40));
        let asked = make_slot_object(&mut e);
        e.mem.set_u32(last + 0x40 + 0x20, asked);
        // Beyond the used bytes there is an entry that is never looked at.
        write_entry(&mut e, &w, last + 0x80, (2, 1, 0x40));
        let track = make_track(&mut e, &[first, last], 0x80, NOT_OWNED);
        e.call(0x00cf_f3c0, &args![0x6666u32, track, w.ctx]);
        let passes = calls_to(&w.log, PASS_FUNCTION);
        assert_eq!(
            passes,
            vec![
                vec![first, first + 0x30, 0x1234, 0x6666],
                vec![first + 0x100, first + 0x130, 0x1234, 0x6666]
            ]
        );
        // Only the type 2 entry of the last sector went to the agent function; the type 6
        // entry asked its object.
        let seen = w.snapshots.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..3], [last, last + 0x20]);
        assert_eq!(calls_to(&w.log, OBJECT_SLOT).len(), 1);
        // Three entries were passed on (also the one that asked not to be called).
        assert_eq!(minimum_times(&e, stream), vec![(MINIMUM_TIME_LABEL, 3.0)]);
    }

    #[test]
    fn process_track_asks_the_entry_object_when_the_agent_added_sweep_records() {
        let mut e = engine();
        let w = process_world(&mut e);
        monitor_stream(&mut e);
        // The agent function appends a record: the list end (the record's first word) moves.
        e.register_double(PROCESS_FUNCTION, |e, a| {
            let end = e.mem.u32(a[4]);
            e.mem.set_u32(a[4], end + 0x30);
            Ret::default()
        });
        let sector = e.mem.alloc(0x200);
        write_entry(&mut e, &w, sector, (2, 1, 0x40));
        write_entry(&mut e, &w, sector + 0x40, (2, 1, 0x40));
        let asked = make_slot_object(&mut e);
        e.mem.set_u32(sector + 8, asked);
        e.mem.set_u32(sector + 0x40 + 8, asked);
        let track = make_track(&mut e, &[sector], 0x80, NOT_OWNED);
        e.call(0x00cf_f3c0, &args![0x6666u32, track, w.ctx]);
        let slot = calls_to(&w.log, OBJECT_SLOT);
        assert_eq!(slot.len(), 2);
        // The bodies as they were before any transform shapes were resolved, the collision
        // input and the record whose list end moved.
        assert_eq!(slot[0][0], asked);
        assert_eq!(slot[0][1..4], [w.bodies[0], w.bodies[1], w.ctx]);
    }

    #[test]
    fn process_track_stops_at_once_when_the_skip_global_is_one() {
        let mut e = engine();
        let w = process_world(&mut e);
        let stream = monitor_stream(&mut e);
        e.mem.set_u32(SKIP_CREATE, 1);
        let sector = e.mem.alloc(0x200);
        write_entry(&mut e, &w, sector, (4, 1, 0x40));
        write_entry(&mut e, &w, sector + 0x40, (4, 1, 0x40));
        let track = make_track(&mut e, &[sector], 0x80, NOT_OWNED);
        e.call(0x00cf_f3c0, &args![0x6666u32, track, w.ctx]);
        // Only the first entry was handled and no profiling record was written.
        assert_eq!(calls_to(&w.log, PASS_FUNCTION).len(), 1);
        assert!(minimum_times(&e, stream).is_empty());
    }

    #[test]
    fn create_agent_links_a_new_entry_into_both_collidables() {
        let mut e = engine();
        let w = process_world(&mut e);
        let log = Rc::clone(&w.log);
        install_router(&mut e, &log);
        // `_reserveMore` gives the array room for 4 elements, keeping its first one.
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            reserve_log
                .borrow_mut()
                .push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(32);
            let old = e.mem.u32(a[1]);
            e.mem.set_u32(room, e.mem.u32(old));
            e.mem.set_u32(room + 4, e.mem.u32(old + 4));
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let make_list = |e: &mut Engine, signed: u8, capacity: u32| {
            let collidable = e.mem.alloc(0x60);
            let data = e.mem.alloc(32);
            e.mem.set_u32(data, 0xaaaa);
            e.mem.set_u32(collidable + 0x50, data);
            e.mem.set_u32(collidable + 0x54, 1);
            e.mem.set_u32(collidable + 0x58, capacity);
            e.mem.set_u8(collidable + 0x1a, signed);
            collidable
        };
        // The first list has room, the second is full.
        let a = make_list(&mut e, 2, 4);
        let b = make_list(&mut e, 3, 1);
        // Cell 3 + 2 * 10 of the table at +0x1bb0 says 5.
        e.mem.set_u8(w.dispatcher + 0x1bb0 + 23, 5);
        let sector = e.mem.alloc(0x200);
        let track = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        let entry = e
            .call(
                0x00cf_e8a0,
                &args![
                    track,
                    a,
                    w.bodies[0],
                    b,
                    w.bodies[1],
                    0x10u32,
                    2u32,
                    w.ctx,
                    0x4242u32
                ],
            )
            .u32();
        assert_eq!(entry, sector + 0x100);
        assert_eq!(e.mem.u32(track), 0x180);
        assert_eq!(e.mem.u32(entry + 0x10), a);
        assert_eq!(e.mem.u32(entry + 0x14), b);
        assert_eq!(e.mem.u32(entry + 8), 0x4242);
        assert_eq!((e.mem.u16(entry + 4), e.mem.u16(entry + 6)), (1, 1));
        assert_eq!(e.mem.u8(entry + 1), 2);
        assert_eq!(e.mem.u8(entry + 3), 0x80);
        assert_eq!(e.mem.u8(entry + 0xc), 5);
        assert_eq!(e.mem.u32(w.ctx + 0x60), w.dispatcher + 0x1c20 + 5 * 0x40);
        // Agent type 2 has no function at +0x16d8: type 2 with the data at +0x20.
        assert_eq!(e.mem.u8(entry), 0x12);
        // Both lists got a second element (entry, partner).
        for (list, partner) in [(a, b), (b, a)] {
            assert_eq!(e.mem.u32(list + 0x54), 2);
            let data = e.mem.u32(list + 0x50);
            assert_eq!(e.mem.u32(data), 0xaaaa);
            assert_eq!(e.mem.u32(data + 8), entry);
            assert_eq!(e.mem.u32(data + 12), partner);
        }
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        let created = w.snapshots.borrow();
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].address, CREATE_FUNCTION);
        assert_eq!(created[0].args[1..], [entry, entry + 0x20]);
        assert_eq!(
            created[0].words,
            vec![w.bodies[0], w.bodies[1], 0, w.ctx, 0x4242]
        );
        let relative = calls_to(&log, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative[0][1..], [w.motions[0], w.motions[1]]);
    }

    #[test]
    fn create_agent_prepares_a_swept_entry_when_the_agent_has_a_prepare_function() {
        let mut e = engine();
        let w = process_world(&mut e);
        let make_list = |e: &mut Engine| {
            let collidable = e.mem.alloc(0x60);
            let data = e.mem.alloc(32);
            e.mem.set_u32(collidable + 0x50, data);
            e.mem.set_u32(collidable + 0x58, 4);
            collidable
        };
        let (a, b) = (make_list(&mut e), make_list(&mut e));
        install_router(&mut e, &w.log);
        let sector = e.mem.alloc(0x200);
        let track = make_track(&mut e, &[sector], 0, NOT_OWNED);
        let entry = e
            .call(
                0x00cf_e8a0,
                &args![
                    track,
                    a,
                    w.bodies[0],
                    b,
                    w.bodies[1],
                    0x21u32,
                    1u32,
                    w.ctx,
                    7u32
                ],
            )
            .u32();
        assert_eq!(entry, sector);
        // Type 4 or-ed with the flags, time -1, vector cleared, data at +0x30.
        assert_eq!(e.mem.u8(entry), 0x25);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        for word in 0..4 {
            assert_eq!(e.mem.u32(entry + 0x20 + 4 * word), 0);
        }
        let created = w.snapshots.borrow();
        assert_eq!(created[0].args[1..], [entry, entry + 0x30]);
        assert_eq!(e.mem.u32(a + 0x54), 1);
        assert_eq!(e.mem.u32(b + 0x54), 1);
    }
}
