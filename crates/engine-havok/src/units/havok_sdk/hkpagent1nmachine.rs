//! `Havok SDK/hkpagent1nmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A track (`hkpAgent1nTrack` in the Xbox PDB) is an `hkArray` of sector pointers: the data
//! pointer at `+0`, the size at `+4` and the capacity word at `+8` (the low 30 bits are the
//! capacity; bit 31 means the storage is not owned). A sector is `0x200` bytes: the number of
//! bytes used at `+0`, the entries from `+0x10`. An entry starts with its type (byte `+0`), the
//! agent type (byte `+1`) and its size in bytes (byte `+3`, a multiple of 16); the agent data
//! follows at `+0x10` (types 2, 3, 6, 10, 11, 14) or `+0x20` (types 4, 5, 12, 13). Type 0 is
//! padding (skipped by its size) and type 1 ends the track. For the other types (7, 8, 9 and
//! above 14) the exe's loops never advance and run forever when such an entry is met before the
//! end of its sector; the translations stop there with a panic instead. The collision
//! dispatcher keeps one `0x50`-byte record per agent type with function pointers at `+0x16b4`
//! (destroy), `+0x16bc`, `+0x16c0`, `+0x16c4`, `+0x16c8` (update shape collection filter),
//! `+0x16d0` (invalidate time) and `+0x16d4` (warp time).
//!
//! SSE code computes lane by lane in `f32`; x87 code computes in `f64` and rounds to `f32` at
//! each `float` store. Floats copied through `FLD`/`FSTP` go through [`x87_quiet`] and
//! [`x87_quiet_bits`]. The profiling records (`RDTSC` time stamps appended to the thread's
//! monitor stream) use the host clock for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;
use std::cmp::Ordering;

/// Size of a sector.
pub(super) const SECTOR_SIZE: u32 = 0x200;
/// `hkMemoryRouter` lookup (`00c85750`, no arguments); its field `+0x10` is the allocator.
pub(super) const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
pub(super) const ARRAY_RESERVE: u32 = 0x00c9_08c0;
pub(super) const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkMemUtil::memMove` (`00d2cc70`, cdecl `(destination, source, bytes)`).
pub(super) const MEM_MOVE: u32 = 0x00d2_cc70;
/// `hkAgentMachine_processTransformedShapes` (`00cfe1d0`, cdecl `(body record, bodies,
/// transforms, 4, changed byte)`, returns the last body).
pub(super) const PROCESS_TRANSFORMED_SHAPES: u32 = 0x00cf_e1d0;
/// `hkCalcMultiPenetrationDepth` (`00dfaba0`, cdecl `(argument, world word, bodies, count,
/// array, contacts)`).
const CALC_MULTI_PENETRATION_DEPTH: u32 = 0x00df_aba0;
/// The callback of [`hk_agent1n_machine_update_shape_collection_filter`].
const UPDATE_SHAPE_CALLBACK: u32 = 0x00da_3f90;
/// The `float` the warp-time and invalidate-time functions store (-1.0).
pub(super) const WARP_MISMATCH_TIME: u32 = 0x0101_2054;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
pub(super) const TLS_GET_VALUE: u32 = 0x00fd_f044;
pub(super) const MONITOR_SLOT: u32 = 0x0126_835c;
/// The labels of the profiling records of the weld function ("TtConflicts" and the end label).
const LABEL_CONFLICTS: u32 = 0x010d_509c;
const LABEL_END: u32 = 0x010c_41f8;
/// `double` constants of the contact comparison (1.0, the largest dot product that still
/// counts as parallel, 0.8, 0.1 and 0.05) and the addend of the weld distance (0.0).
const ONE: u32 = 0x0101_2070;
const PARALLEL_DOT: u32 = 0x010c_c5d0;
const LOW_DOT: u32 = 0x0102_17d8;
const SMALL_DEPTH: u32 = 0x0101_ffa0;
const TOLERANCE_SCALE: u32 = 0x0103_c7c8;
const WELD_ADDEND: u32 = 0x0101_2060;
/// The `float` the weld function starts its search for the smallest dot product with (2.0).
const INITIAL_DOT: u32 = 0x0101_62c0;

/// Offsets in the collision dispatcher's per-agent-type records.
const INVALIDATE_TIME_FUNCTION: u32 = 0x16d0;
const WARP_TIME_FUNCTION: u32 = 0x16d4;
const DESTROY_FUNCTION: u32 = 0x16b4;
const UPDATE_FILTER_FUNCTION: u32 = 0x16c8;

/// The memory router's allocator (`router + 0x10`).
pub(super) fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Frees a block of `size` bytes through the router's allocator (vtable slot 8).
pub(super) fn router_free(e: &mut Engine, block: u32, size: u32) {
    let allocator = router_allocator(e);
    e.vcall(allocator, 8, &args![block, size]);
}

/// Takes a `0x200`-byte block from the router's allocator (vtable slot 4); the first word of a
/// block it returned is cleared. Returns 0 when there is no block.
pub(super) fn take_sector(e: &mut Engine) -> u32 {
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![SECTOR_SIZE]).u32();
    if block != 0 {
        e.mem.set_u32(block, 0);
    }
    block
}

/// The agent function the dispatcher keeps for the entry's agent type (byte at +1) at table
/// offset `table`.
pub(super) fn dispatcher_function(e: &Engine, entry: u32, dispatcher: u32, table: u32) -> u32 {
    let record = u32::from(e.mem.u8(entry + 1)) * 0x50;
    e.mem
        .u32(dispatcher.wrapping_add(record).wrapping_add(table))
}

/// The size of an entry (byte at +3).
pub(super) fn entry_size(e: &Engine, entry: u32) -> u32 {
    u32::from(e.mem.u8(entry + 3))
}

/// The stand-in for `RDTSC`'s low word: the host clock.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
pub(super) fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Where the next entry of a walk over the track's sectors is: the entry, the end of its sector's
/// used bytes and the byte offset of the next sector pointer in the track's data.
struct Walk {
    entry: u32,
    limit: u32,
    next_cell: u32,
}

impl Walk {
    /// Starts at the first entry of the first sector.
    fn start(e: &Engine, track: u32) -> Walk {
        let sector = e.mem.u32(e.mem.u32(track));
        Walk {
            entry: sector + 0x10,
            limit: sector.wrapping_add(e.mem.u32(sector)).wrapping_add(0x10),
            next_cell: 4,
        }
    }

    /// When `entry` has reached the end of the sector's used bytes (unsigned compare), moves to
    /// the first entry of the next sector.
    fn advance(&mut self, e: &Engine, track: u32) {
        if self.entry >= self.limit {
            let sector = e.mem.u32(e.mem.u32(track).wrapping_add(self.next_cell));
            self.next_cell += 4;
            self.limit = sector.wrapping_add(e.mem.u32(sector)).wrapping_add(0x10);
            self.entry = sector + 0x10;
        }
    }

    /// The exe's loops do not move on at an entry of a type they do not know: they would run
    /// forever while the entry is before the end of its sector.
    fn stuck(&self, address: &str) {
        if self.entry < self.limit {
            panic!("endless loop at an entry of an unknown type in {address}");
        }
    }
}

// Translated from 00da3510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_InvalidateTim` (named in the engine map): walks the entries of `track`. For
/// types 4, 5, 12 and 13 the `float` at the entry's +0xc is set to the `float` at `01012054`
/// (-1.0) and the 16 bytes at +0x10 are cleared. For types 2, 3, 6, 10, 11, 14 (agent data at
/// +0x10) and those (agent data at +0x20) the dispatcher's function at +0x16d0 of the agent
/// type's record (the dispatcher is the first word of `input`), when it is not null, is called
/// with the entry, the agent data and `input`. Type 0 is skipped, type 1 ends the walk.
pub fn hk_agent1n_machine_invalidate_tim(e: &mut Engine, track: Ptr, input: Ptr) {
    let track = track.addr();
    let input = input.addr();
    let mut walk = Walk::start(e, track);
    loop {
        let entry = walk.entry;
        match e.mem.u8(entry) {
            0 => walk.entry += entry_size(e, entry),
            1 => return,
            kind @ (2 | 3 | 6 | 10 | 11 | 14 | 4 | 5 | 12 | 13) => {
                let agent_data = if matches!(kind, 4 | 5 | 12 | 13) {
                    let time = x87_quiet(e.mem.f32(WARP_MISMATCH_TIME));
                    e.mem.set_f32(entry + 0xc, time);
                    for word in 0..4 {
                        e.mem.set_u32(entry + 0x10 + 4 * word, 0);
                    }
                    entry + 0x20
                } else {
                    entry + 0x10
                };
                walk.entry += entry_size(e, entry);
                let dispatcher = e.mem.u32(input);
                let function = dispatcher_function(e, entry, dispatcher, INVALIDATE_TIME_FUNCTION);
                if function != 0 {
                    e.call(function, &args![entry, agent_data, input]);
                }
            }
            _ => walk.stuck("00da3510"),
        }
        walk.advance(e, track);
    }
}

// Translated from 00da35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_WarpTime` (named in the engine map): walks the entries of `track` like
/// [`hk_agent1n_machine_invalidate_tim`]. Types 4, 5, 12 and 13 first store `time_b` at the
/// entry's +0xc when `time_a` equals the stored time there (otherwise the `float` at `01012054`).
/// The dispatcher's function at +0x16d4 of the agent type's record (the dispatcher is the first
/// word of `input`), when it is not null, is called with the entry, its agent data, `time_a`,
/// `time_b` and `input`. Floats travel through `FLD`/`FSTP`, so signalling NaNs are quieted.
pub fn hk_agent1n_machine_warp_time(
    e: &mut Engine,
    track: Ptr,
    time_a: f32,
    time_b: f32,
    input: Ptr,
) {
    let track = track.addr();
    let input = input.addr();
    let mut walk = Walk::start(e, track);
    loop {
        let entry = walk.entry;
        match e.mem.u8(entry) {
            0 => walk.entry += entry_size(e, entry),
            1 => return,
            kind @ (2 | 3 | 6 | 10 | 11 | 14 | 4 | 5 | 12 | 13) => {
                let agent_data = if matches!(kind, 4 | 5 | 12 | 13) {
                    // An unordered (NaN) compare is not equal, as the `FUCOMPP` flags say.
                    let stored = e.mem.f32(entry + 0xc);
                    let new_time = if time_a == stored {
                        x87_quiet(time_b)
                    } else {
                        x87_quiet(e.mem.f32(WARP_MISMATCH_TIME))
                    };
                    e.mem.set_f32(entry + 0xc, new_time);
                    entry + 0x20
                } else {
                    entry + 0x10
                };
                walk.entry += entry_size(e, entry);
                let dispatcher = e.mem.u32(input);
                let function = dispatcher_function(e, entry, dispatcher, WARP_TIME_FUNCTION);
                if function != 0 {
                    e.call(
                        function,
                        &args![
                            entry,
                            agent_data,
                            x87_quiet_bits(time_a.to_bits()),
                            x87_quiet_bits(time_b.to_bits()),
                            input
                        ],
                    );
                }
            }
            _ => walk.stuck("00da35f0"),
        }
        walk.advance(e, track);
    }
}

/// The relation of two contact records (`this` and `other`, each: the point at +0, the normal at
/// +0x10, a 16-bit-or-`-1` word at +0x20 and the distance at +0x1c) that
/// [`fn_00da37d0`] and the weld function share. `a` is the tolerance, `sum` the `float` it is
/// compared against (the tolerance plus a second term, rounded to `float`) and `limit` the
/// address of the `float` that scales the final tolerance. Returns 0 (not related), 1 (the
/// caller stops scanning the current contact) or 2 (a conflict).
fn contact_relation(e: &Engine, this: u32, other: u32, a: f32, sum: f32, limit: u32) -> u32 {
    let load = |base: u32, lane: u32| e.mem.f32(base + 4 * lane);
    let difference: [f32; 3] =
        std::array::from_fn(|i| load(this, i as u32) - load(other, i as u32));
    let other_normal: [f32; 3] = std::array::from_fn(|i| load(other, 4 + i as u32));
    let this_normal: [f32; 3] = std::array::from_fn(|i| load(this, 4 + i as u32));
    // Sums of three products, added as `(second + first) + third` the way the lanes are.
    let sum3 = |p: [f32; 3]| p[2] + (p[1] + p[0]);
    let along_other = sum3(std::array::from_fn(|i| other_normal[i] * difference[i]));
    let along_this = sum3(std::array::from_fn(|i| this_normal[i] * difference[i]));
    let normals = sum3(std::array::from_fn(|i| this_normal[i] * other_normal[i]));
    let one = e.mem.f64(ONE);
    let scaled = (f64::from(normals) - one) * f64::from(a);
    let behind = (-f64::from(along_this) + scaled) as f32;
    let ahead = (scaled + f64::from(along_other)) as f32;
    // `FCOM`, then the flag test: only a strict "less" ends the comparison.
    if sum < behind {
        return 0;
    }
    let word = e.mem.u32(this + 0x20);
    if f64::from(normals) > e.mem.f64(PARALLEL_DOT) && word == 0xffff {
        return 0;
    }
    let distance_this = f64::from(e.mem.f32(this + 0x1c));
    let distance_other = f64::from(e.mem.f32(other + 0x1c));
    if word == 0xffff
        && f64::from(normals) < e.mem.f64(LOW_DOT)
        && f64::from(behind) + f64::from(ahead) < e.mem.f64(SMALL_DEPTH)
    {
        let bound = (-f64::from(sum) * 2.0) as f32;
        if distance_other + distance_this < f64::from(bound) {
            return 2;
        }
    }
    let tolerance = if word == 0xffff {
        0.0f32
    } else {
        (f64::from(e.mem.f32(limit)) * e.mem.f64(TOLERANCE_SCALE)) as f32
    };
    // `FCOMPP` with the "below" bit tested: below or unordered counts.
    let remaining = (distance_this - f64::from(tolerance)).partial_cmp(&distance_other);
    u32::from(matches!(remaining, Some(Ordering::Less) | None))
}

// Translated from 00da37d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed; register-input function (cdecl for the stack words): `this_contact` is ECX, `other`
/// is ESI, then the stack words `tolerance` (a `float`), `limit` (a pointer to a `float`) and
/// `margin` (a `float`). No caller is known. Compares the contact `this_contact` with `other`
/// (see [`contact_relation`], the second term of the comparison value is `margin`): returns 0
/// when they are not related, 1 when `other` is in front of the other by more than the
/// tolerance and 2 when they conflict.
pub fn fn_00da37d0(
    e: &mut Engine,
    this_contact: Ptr,
    other: Ptr,
    tolerance: f32,
    limit: Ptr,
    margin: f32,
) -> u32 {
    let sum = (f64::from(tolerance) + f64::from(margin)) as f32;
    contact_relation(
        e,
        this_contact.addr(),
        other.addr(),
        tolerance,
        sum,
        limit.addr(),
    )
}

/// Reads 12-byte records of the list at `list + 0x408` as `(contact, body entry, word)`.
fn list_record(list: u32, index: u32) -> u32 {
    list + 0x408 + 12 * index
}

/// The dot product of the normals (+0x10) of two contacts, added the way `ADDSS` does: `z + (y +
/// x)`.
fn normal_dot(e: &Engine, first: u32, second: u32) -> f32 {
    let product: [f32; 3] = std::array::from_fn(|i| {
        e.mem.f32(second + 0x10 + 4 * i as u32) * e.mem.f32(first + 0x10 + 4 * i as u32)
    });
    product[2] + (product[1] + product[0])
}

// Translated from 00da3930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_Weld` (named in the engine map): `context` is a record of pointers (`+0` a
/// pointer to the world object whose first word points to a structure with a `float` tolerance at
/// `+0x10`, a type word at `+0xc` and a word at `+8`; `+4` the object with a word at `+8`; `+8`
/// is word 2 of the exe's first argument; `+0xc` the pointer to the dispatcher; `+0x10` the object with the
/// callbacks at vtable slots `0xc`, `0x10` and `0x14`; `+0x20` an array handed on),
/// `objects` the object whose slot `0x14` returns a body for a body entry's word `+8` and
/// `state` the structure with the pool of contacts (pointer at `+0`, 0x30 bytes per contact)
/// and the list at `+0x3080` (end of the 12-byte records `(contact, entry, word)` from `+0x408`
/// at `+0`; end of the contact pointer array from `+8` at `+4`).
///
/// For every record the contacts of the pointer array are compared with its contact (the
/// relation of [`fn_00da37d0`], tolerance from the world structure, the addend `double` at
/// `01012060`, the limit pointer `[[context+0xc]+0x60]`): a record that is in conflict with a
/// contact is queued; a record that is in front of a contact is not handled further; every other
/// record is handed to slot `0xc` of the context's object (with a record of the body, the
/// context, `state`, 0 and the contact), which returns a 16-bit handle (`0xffff` for none; a
/// handle is followed by slot `0x10` with -1). The dispatcher's function at `+0x16c4` is called
/// for the queued records. While at least two are queued the pair with the smallest dot product
/// of the contact normals is resolved by `hkCalcMultiPenetrationDepth` (when the bodies pass the
/// `0x40000` flag test; a failed test ends the loop) and both leave the queue; a profiling
/// record is written at each pass. A single queued record with a handle has it released (slots
/// `0x14` and `0x10`). Last, from the last record back, a record with a handle gets it stored at
/// its contact's +0x20 (function at `+0x16c0`); one without gets the dispatcher's function at
/// `+0x16bc` and takes over the newest contact of the pool (`state[0] -= 0x30`, 0x24 bytes
/// copied) after slot `0x10` is called with -1.
pub fn hk_agent1n_machine_weld(e: &mut Engine, context: Ptr, objects: Ptr, state: Ptr) {
    let (ctx, bodies, state) = (context.addr(), objects.addr(), state.addr());
    // The exe keeps its scratch buffers (the bodies at +0x260 and +0x660, the body record at
    // +0x30 and the contact pair at +0x44) in one frame.
    e.with_stack(0x860, |e, frame| weld(e, frame.addr(), ctx, bodies, state));
}

/// The body of [`hk_agent1n_machine_weld`], with the address of its scratch frame.
fn weld(e: &mut Engine, frame: u32, ctx: u32, bodies: u32, state: u32) {
    let list = e.mem.u32(state + 0x3080);
    let world = e.mem.u32(e.mem.u32(ctx));
    let mut handles = [0u16; 256];
    let mut queue = [0u16; 256];
    let mut queued = 0usize;
    let mut record_index = 0usize;
    let mut record = list + 0x408;
    if record < e.mem.u32(list) {
        loop {
            let contact = e.mem.u32(record);
            handles[record_index] = 0xffff;
            let mut conflict = false;
            let mut in_front = false;
            let mut other = list + 8;
            if other < e.mem.u32(list + 4) {
                loop {
                    let candidate = e.mem.u32(other);
                    if contact != candidate {
                        let limit = e.mem.u32(e.mem.u32(ctx + 0xc) + 0x60);
                        let tolerance = x87_quiet(e.mem.f32(world + 0x10));
                        let sum = (f64::from(tolerance) + e.mem.f64(WELD_ADDEND)) as f32;
                        match contact_relation(e, candidate, contact, tolerance, sum, limit) {
                            0 => {}
                            1 => {
                                in_front = true;
                                break;
                            }
                            _ => conflict = true,
                        }
                    }
                    other += 4;
                    if other >= e.mem.u32(list + 4) {
                        break;
                    }
                }
                if !in_front && conflict {
                    queue[queued] = record_index as u16;
                    queued += 1;
                }
            }
            if !in_front {
                let entry = e.mem.u32(record + 4);
                let key = e.mem.u32(entry + 8);
                let body = e.vcall(bodies, 0x14, &args![key, frame + 0x260]).u32();
                let first = e.mem.u32(ctx + 4);
                let first_word = e.mem.u32(first + 8);
                let entry_word = e.mem.u32(entry + 8);
                let shape = frame + 0x30;
                e.mem.set_u32(shape, body);
                e.mem.set_u32(shape + 4, entry_word);
                e.mem.set_u32(shape + 8, first_word);
                e.mem.set_u32(shape + 0xc, first);
                let contact = e.mem.u32(record);
                let callbacks = e.mem.u32(ctx + 0x10);
                let ctx0 = e.mem.u32(ctx);
                let dispatcher_holder = e.mem.u32(ctx + 0xc);
                let handle = e
                    .vcall(
                        callbacks,
                        0xc,
                        &args![ctx0, shape, dispatcher_holder, state, 0u32, contact],
                    )
                    .u32() as u16;
                handles[record_index] = handle;
                if handle != 0xffff {
                    let callbacks = e.mem.u32(ctx + 0x10);
                    e.vcall(callbacks, 0x10, &args![0xffff_ffffu32]);
                }
            }
            record_index += 1;
            record += 0xc;
            if record >= e.mem.u32(list) {
                break;
            }
        }
    }
    let dispatcher = e.mem.u32(e.mem.u32(ctx + 0xc));
    for &index in &queue[..queued] {
        let at = list_record(list, u32::from(index));
        let entry = e.mem.u32(at + 4);
        let word = e.mem.u32(at + 8);
        let function = dispatcher_function(e, entry, dispatcher, 0x16c4);
        e.call(function, &args![entry, word, 0xffffu32]);
    }
    while queued >= 2 {
        profile_record(e, LABEL_CONFLICTS);
        let mut smallest = x87_quiet(e.mem.f32(INITIAL_DOT));
        let mut pair = (0usize, 1usize);
        for i in 0..queued {
            let first = e.mem.u32(list_record(list, u32::from(queue[i])));
            for j in i + 1..queued {
                let second = e.mem.u32(list_record(list, u32::from(queue[j])));
                let dot = normal_dot(e, first, second);
                if smallest > dot {
                    smallest = dot;
                    pair = (i, j);
                }
            }
        }
        let (i, j) = pair;
        let first_record = list_record(list, u32::from(queue[i]));
        let second_record = list_record(list, u32::from(queue[j]));
        let first_entry = e.mem.u32(first_record + 4);
        let second_entry = e.mem.u32(second_record + 4);
        let world_object = e.mem.u32(e.mem.u32(ctx));
        let flagged = |e: &Engine, object: u32| {
            let dispatcher = e.mem.u32(e.mem.u32(ctx + 0xc));
            let kind = e.mem.u32(object + 0xc);
            e.mem.u32(dispatcher + 0x110 + 4 * kind) & 0x40000 != 0
        };
        if !flagged(e, world_object) {
            break;
        }
        let key = e.mem.u32(first_entry + 8);
        let first_body = e.vcall(bodies, 0x14, &args![key, frame + 0x260]).u32();
        if !flagged(e, first_body) {
            break;
        }
        let key = e.mem.u32(second_entry + 8);
        let second_body = e.vcall(bodies, 0x14, &args![key, frame + 0x660]).u32();
        if !flagged(e, second_body) {
            break;
        }
        let first_contact = e.mem.u32(first_record);
        let second_contact = e.mem.u32(second_record);
        let shapes = frame + 0x30;
        let contacts = frame + 0x44;
        e.mem.set_u32(shapes, first_body);
        e.mem.set_u32(shapes + 4, second_body);
        e.mem.set_u32(contacts, first_contact);
        e.mem.set_u32(contacts + 4, second_contact);
        let ctx0 = e.mem.u32(ctx);
        let argument = e.mem.u32(ctx0 + 8);
        e.call(
            CALC_MULTI_PENETRATION_DEPTH,
            &args![argument, world_object, shapes, 2u32, ctx + 0x20, contacts],
        );
        queue[j] = queue[queued - 1];
        queued -= 2;
        queue[i] = queue[queued];
        profile_record(e, LABEL_END);
    }
    if queued != 0 {
        let index = usize::from(queue[0]);
        if handles[index] != 0xffff {
            let pool = e.mem.u32(state + 4);
            let callbacks = e.mem.u32(ctx + 0x10);
            e.vcall(callbacks, 0x14, &args![u32::from(handles[index]), pool]);
            handles[index] = 0xffff;
            let callbacks = e.mem.u32(ctx + 0x10);
            e.vcall(callbacks, 0x10, &args![1u32]);
        }
    }
    let dispatcher = e.mem.u32(e.mem.u32(ctx + 0xc));
    let mut end = e.mem.u32(list);
    let bytes = end.wrapping_sub(list).wrapping_sub(0x408) as i32;
    let mut index = bytes / 12 - 1;
    while index >= 0 {
        let handle = handles[index as usize];
        end -= 12;
        if handle != 0xffff {
            let entry = e.mem.u32(end + 4);
            let word = e.mem.u32(end + 8);
            let function = dispatcher_function(e, entry, dispatcher, 0x16c0);
            e.call(function, &args![entry, word, u32::from(handle)]);
            let contact = e.mem.u32(end);
            e.mem.set_u32(contact + 0x20, u32::from(handle));
        } else {
            let callbacks = e.mem.u32(ctx + 0x10);
            e.vcall(callbacks, 0x10, &args![0xffff_ffffu32]);
            let entry = e.mem.u32(end + 4);
            let word = e.mem.u32(end + 8);
            let function = dispatcher_function(e, entry, dispatcher, 0x16bc);
            e.call(function, &args![entry, word, 0xffffu32]);
            let top = e.mem.u32(state).wrapping_sub(0x30);
            e.mem.set_u32(state, top);
            let contact = e.mem.u32(end);
            for word in 0..9 {
                let value = e.mem.u32(top + 4 * word);
                e.mem.set_u32(contact + 4 * word, value);
            }
        }
        index -= 1;
    }
}

// Translated from 00da3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the callback of [`hk_agent1n_machine_update_shape_collection_filter`]): `context`
/// is a record of words (`+0`, `+4`: a collidable whose word `+8` is also used, `+0xc`: an
/// object with slot `0x14`, `+0x10`: an object whose first word is the dispatcher and whose word
/// `+0xc` leads to the object that decides, `+0x14`, `+0x18`), `entry` the entry and `third` an
/// argument passed on. Slot 4 of the object at `[[context+0x10]+0xc]+0xc` is asked (with a flag
/// byte pointer, the context's words `+0x10`, `+0`, `+4`, `+0xc` and the entry's word `+8`) for a
/// pointer to a flag byte. When the flag is set and the dispatcher has an update function
/// (`+0x16c8`) for the entry's agent type, slot `0x14` of `context+0xc` gives the body for the
/// entry's word `+8` (into a 0x200-byte scratch buffer), a body record `(body, word, collidable
/// word +8, collidable)` is built, transform shapes are resolved when the entry's flag `8` is
/// set (`hkAgentMachine_processTransformedShapes`, into scratch buffers) and the update function
/// is called with the entry, `third`, the context's first word and the record (the record
/// first for entry types 3 and 5) and the context's words `+0x10`, `+0x14`, `+0x18`; the result
/// is the entry plus its size. With the flag clear the dispatcher's destroy function (`+0x16b4`)
/// is called with the entry, `third`, the context's words `+0x14` and `+0x18` and the
/// dispatcher, and the result is the entry itself.
pub fn fn_00da3f90(e: &mut Engine, context: Ptr, entry: Ptr, third: u32) -> Ptr {
    let (ctx, entry) = (context.addr(), entry.addr());
    // The exe keeps the flag byte (+0x17), the body record (+0x20), the bodies (+0x30), the
    // body buffer (+0x70) and the transforms (+0x270) in one frame.
    e.with_stack(0x530, |e, frame| {
        let frame = frame.addr();
        let entry_word = e.mem.u32(entry + 8);
        let first = e.mem.u32(ctx + 0x10);
        let holder = e.mem.u32(first + 0xc) + 0xc;
        let ctx0 = e.mem.u32(ctx);
        let ctx1 = e.mem.u32(ctx + 4);
        let ctx3 = e.mem.u32(ctx + 0xc);
        let flag = frame + 0x17;
        let result = e
            .vcall(holder, 4, &args![flag, first, ctx0, ctx1, ctx3, entry_word])
            .u32();
        let dispatcher = e.mem.u32(e.mem.u32(ctx + 0x10));
        if e.mem.u8(result) == 0 {
            let ctx5 = e.mem.u32(ctx + 0x14);
            let ctx6 = e.mem.u32(ctx + 0x18);
            let function = dispatcher_function(e, entry, dispatcher, DESTROY_FUNCTION);
            e.call(function, &args![entry, third, ctx5, ctx6, dispatcher]);
            return Ptr::new(entry);
        }
        let function = dispatcher_function(e, entry, dispatcher, UPDATE_FILTER_FUNCTION);
        if function != 0 {
            let ctx3 = e.mem.u32(ctx + 0xc);
            let body = e.vcall(ctx3, 0x14, &args![entry_word, frame + 0x70]).u32();
            let flags = e.mem.u8(entry);
            let ctx1 = e.mem.u32(ctx + 4);
            let ctx1_word = e.mem.u32(ctx1 + 8);
            let ctx0 = e.mem.u32(ctx);
            let record = frame + 0x20;
            e.mem.set_u32(record, body);
            e.mem.set_u32(record + 4, entry_word);
            e.mem.set_u32(record + 8, ctx1_word);
            e.mem.set_u32(record + 0xc, ctx1);
            let mut transformed = record;
            if flags & 8 != 0 {
                e.mem.set_u8(flag, 0);
                transformed = e
                    .call(
                        PROCESS_TRANSFORMED_SHAPES,
                        &args![record, frame + 0x30, frame + 0x270, 4u32, flag],
                    )
                    .u32();
            }
            let ctx4 = e.mem.u32(ctx + 0x10);
            let ctx5 = e.mem.u32(ctx + 0x14);
            let ctx6 = e.mem.u32(ctx + 0x18);
            let (left, right) = if matches!(e.mem.u8(entry) & 7, 3 | 5) {
                (transformed, ctx0)
            } else {
                (ctx0, transformed)
            };
            e.call(
                function,
                &args![entry, third, left, right, ctx4, ctx5, ctx6],
            );
        }
        Ptr::new(entry + entry_size(e, entry))
    })
}

// Translated from 00da40e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: like [`fn_00da3f90`], for a context with two body-providing objects (`+8` and
/// `+0xc`). Slot 0 of the object at `[[context+0x10]+0xc]+0xc` is asked with a flag byte
/// pointer, the context's words `+0x10`, `+0`, `+4`, `+8`, `+0xc` and the entry's words `+4`
/// and `+8` for a pointer to a flag byte. With the flag clear the dispatcher's destroy function
/// (`+0x16b4`) is called with the entry, `third`, the context's words `+0x14`, `+0x18` and the
/// dispatcher, and the entry itself is returned. With it set and an update function (`+0x16c8`)
/// for the entry's agent type, slot `0x14` of `context+0xc` gives the body for the entry's word
/// `+8` and slot `0x14` of `context+8` the one for its word `+4` (scratch buffers of 0x200
/// bytes); a record `(body, key, collidable word +8, collidable)` is built for each (the
/// collidables are the context's words `+0` and `+4`), transform shapes are resolved when the
/// entry's flag `8` is set and the update function is called with the entry, `third`, the two
/// records (swapped for entry types 3 and 5) and the context's words `+0x10`, `+0x14`,
/// `+0x18`. The entry plus its size is returned.
pub fn fn_00da40e0(e: &mut Engine, context: Ptr, entry: Ptr, third: u32) -> Ptr {
    let (ctx, entry) = (context.addr(), entry.addr());
    // The exe keeps the flag bytes (+0xb, +0xf), the body records (+0x18, +0x28), the bodies
    // (+0x40, +0x80), the body buffers (+0xc0, +0x580) and the transforms (+0x2c0, +0x780) in
    // one frame.
    e.with_stack(0xa40, |e, frame| {
        let frame = frame.addr();
        let first = e.mem.u32(ctx + 0x10);
        let holder = e.mem.u32(first + 0xc) + 0xc;
        let word_eight = e.mem.u32(entry + 8);
        let word_four = e.mem.u32(entry + 4);
        let ctx0 = e.mem.u32(ctx);
        let ctx1 = e.mem.u32(ctx + 4);
        let ctx2 = e.mem.u32(ctx + 8);
        let ctx3 = e.mem.u32(ctx + 0xc);
        let result = e
            .vcall(
                holder,
                0,
                &args![
                    frame + 0xb,
                    first,
                    ctx0,
                    ctx1,
                    ctx2,
                    ctx3,
                    word_four,
                    word_eight
                ],
            )
            .u32();
        let dispatcher = e.mem.u32(e.mem.u32(ctx + 0x10));
        if e.mem.u8(result) == 0 {
            let ctx5 = e.mem.u32(ctx + 0x14);
            let ctx6 = e.mem.u32(ctx + 0x18);
            let function = dispatcher_function(e, entry, dispatcher, DESTROY_FUNCTION);
            e.call(function, &args![entry, third, ctx5, ctx6, dispatcher]);
            return Ptr::new(entry);
        }
        let function = dispatcher_function(e, entry, dispatcher, UPDATE_FILTER_FUNCTION);
        if function != 0 {
            let ctx3 = e.mem.u32(ctx + 0xc);
            let body_a = e.vcall(ctx3, 0x14, &args![word_eight, frame + 0x580]).u32();
            let ctx2 = e.mem.u32(ctx + 8);
            let body_b = e.vcall(ctx2, 0x14, &args![word_four, frame + 0xc0]).u32();
            let flags = e.mem.u8(entry);
            let ctx1 = e.mem.u32(ctx + 4);
            let ctx1_word = e.mem.u32(ctx1 + 8);
            let ctx0 = e.mem.u32(ctx);
            let ctx0_word = e.mem.u32(ctx0 + 8);
            // The record of the first collidable (the context's word +0, with the body for the
            // entry's word +4) and that of the second.
            let first_record = frame + 0x18;
            let second_record = frame + 0x28;
            e.mem.set_u32(first_record, body_b);
            e.mem.set_u32(first_record + 4, word_four);
            e.mem.set_u32(first_record + 8, ctx0_word);
            e.mem.set_u32(first_record + 0xc, ctx0);
            e.mem.set_u32(second_record, body_a);
            e.mem.set_u32(second_record + 4, word_eight);
            e.mem.set_u32(second_record + 8, ctx1_word);
            e.mem.set_u32(second_record + 0xc, ctx1);
            let (mut left, mut right) = (first_record, second_record);
            if flags & 8 != 0 {
                e.mem.set_u8(frame + 0xf, 0);
                right = e
                    .call(
                        PROCESS_TRANSFORMED_SHAPES,
                        &args![
                            second_record,
                            frame + 0x80,
                            frame + 0x2c0,
                            4u32,
                            frame + 0xf
                        ],
                    )
                    .u32();
                left = e
                    .call(
                        PROCESS_TRANSFORMED_SHAPES,
                        &args![first_record, frame + 0x40, frame + 0x780, 4u32, frame + 0xf],
                    )
                    .u32();
            }
            let ctx4 = e.mem.u32(ctx + 0x10);
            let ctx5 = e.mem.u32(ctx + 0x14);
            let ctx6 = e.mem.u32(ctx + 0x18);
            let (one, two) = if matches!(e.mem.u8(entry) & 7, 3 | 5) {
                (right, left)
            } else {
                (left, right)
            };
            e.call(function, &args![entry, third, one, two, ctx4, ctx5, ctx6]);
        }
        Ptr::new(entry + entry_size(e, entry))
    })
}

// Translated from 00da42b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_Destroy` (named in the engine map, cdecl): walks the entries of `track`
/// and calls the dispatcher's destroy function (`+0x16b4` of the agent type's record) for every
/// entry of types 2, 3, 6, 10, 11 and 14 (agent data at +0x10) and 4, 5, 12 and 13 (agent
/// data at +0x20) with the entry, the agent data, `argument_a`, `argument_b` and `dispatcher`.
/// Every sector is returned to the memory router's allocator (`0x200` bytes) when the walk
/// leaves it, and at the end entry (type 1) the last one and the track's storage (when owned,
/// slot `0x10`) too; the track is left empty (size 0, data 0, capacity word `0x80000000`).
pub fn hk_agent1n_machine_destroy(
    e: &mut Engine,
    track: Ptr,
    dispatcher: Ptr,
    argument_a: u32,
    argument_b: u32,
) {
    let track = track.addr();
    let dispatcher = dispatcher.addr();
    let sectors = e.mem.u32(track);
    let mut walk = Walk::start(e, track);
    let mut index = 0u32;
    loop {
        let entry = walk.entry;
        match e.mem.u8(entry) {
            0 => walk.entry += entry_size(e, entry),
            1 => {
                let sector = e.mem.u32(sectors + 4 * index);
                if sector != 0 {
                    router_free(e, sector, SECTOR_SIZE);
                }
                let allocator = router_allocator(e);
                let capacity = e.mem.u32(track + 8);
                e.mem.set_u32(track + 4, 0);
                if capacity & 0x8000_0000 == 0 {
                    let data = e.mem.u32(track);
                    e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
                }
                e.mem.set_u32(track + 8, 0x8000_0000);
                e.mem.set_u32(track, 0);
                return;
            }
            kind @ (2 | 3 | 6 | 10 | 11 | 14 | 4 | 5 | 12 | 13) => {
                let agent_data = if matches!(kind, 4 | 5 | 12 | 13) {
                    entry + 0x20
                } else {
                    entry + 0x10
                };
                walk.entry += entry_size(e, entry);
                let function = dispatcher_function(e, entry, dispatcher, DESTROY_FUNCTION);
                e.call(
                    function,
                    &args![entry, agent_data, argument_a, argument_b, dispatcher],
                );
            }
            _ => walk.stuck("00da42b0"),
        }
        if walk.entry >= walk.limit {
            let done = e.mem.u32(sectors + 4 * index);
            index += 1;
            if done != 0 {
                router_free(e, done, SECTOR_SIZE);
            }
            let next = e.mem.u32(e.mem.u32(track) + 4 * index);
            walk.entry = next + 0x10;
            walk.limit = next.wrapping_add(e.mem.u32(next)).wrapping_add(0x10);
        }
    }
}

// Translated from 00da4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: gives `track` its first sector. The size is set to 0, the capacity grown to at
/// least 4 sector pointers when it is below 4 (`_reserve`), made room for one more
/// (`_reserveMore` when full) and the size set to 1; a `0x200`-byte sector is taken from the
/// memory router's allocator and stored as the first sector. Its used bytes are 0x10, it holds a
/// single end entry (type 1 at +0x10, size 0x10 at +0x13, words -1 at +0x14 and +0x18).
pub fn fn_00da4430(e: &mut Engine, track: Ptr) {
    let track = track.addr();
    e.mem.set_u32(track + 4, 0);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(track + 8) & 0x3fff_ffff;
    if (capacity as i32) < 4 {
        let mut wanted = capacity * 2;
        if (wanted as i32) <= 4 {
            wanted = 4;
        }
        e.call(ARRAY_RESERVE, &args![allocator, track, wanted, 4u32]);
    }
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(track + 8) & 0x3fff_ffff;
    if e.mem.u32(track + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, track, 4u32]);
    }
    let size = e.mem.u32(track + 4);
    e.mem.set_u32(track + 4, size.wrapping_add(1));
    let sector = take_sector(e);
    let data = e.mem.u32(track);
    e.mem.set_u32(data, sector);
    e.mem.set_u8(sector + 0x13, 0x10);
    e.mem.set_u8(sector + 0x10, 1);
    e.mem.set_u32(sector + 0x18, 0xffff_ffff);
    e.mem.set_u32(sector + 0x14, 0xffff_ffff);
    e.mem.set_u32(sector, 0x10);
}

/// Copies `units` 16-byte units (at least one: the exe's loops test after the copy) from `from`
/// to `to`.
pub(super) fn copy_units(e: &mut Engine, from: u32, to: u32, units: i32) {
    let mut done = 0u32;
    let mut left = units;
    loop {
        for word in 0..4 {
            let value = e.mem.u32(from + 16 * done + 4 * word);
            e.mem.set_u32(to + 16 * done + 4 * word, value);
        }
        done += 1;
        left -= 1;
        if left <= 0 {
            break;
        }
    }
}

// Translated from 00da44d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_VisitAllAgents` (named in the engine map, cdecl): rebuilds `track` entry by
/// entry. Every entry is copied into a new sector (a 0x80-byte scratch area collects entries when
/// the sector is nearly full); the eight words at `input` are copied into a scratch block for every
/// entry and `callback(scratch, entry copy, agent data)` (cdecl) is called for the types
/// 2, 3, 6, 10, 11 and 14 (agent data at +0x10) and 4, 5, 12 and 13 (+0x20), whose result is where
/// the copy goes on (the callback may resize the entry); an entry of type 1 or 9 is kept as is
/// (0x10 bytes), the other types are dropped. A finished sector is stored in the track (inserted
/// before the input sectors still to come, `memMove`), an emptied input sector is freed (with a
/// delay of one sector so one can be reused) and the track ends as the list of the new sectors.
pub fn hk_agent1n_machine_visit_all_agents(e: &mut Engine, track: Ptr, input: Ptr, callback: u32) {
    let track = track.addr();
    let input = input.addr();
    e.with_stack(0xa0, |e, scratch| {
        let temp = scratch.addr();
        let temp_end = temp + 0x80;
        let copy = temp_end;
        let mut previous = e.mem.u32(e.mem.u32(track));
        let mut spare = 0u32;
        let mut output_index = 0u32;
        let mut output = take_sector(e);
        let mut read = previous + 0x10;
        let mut write = output.wrapping_add(0x10);
        let mut read_limit = previous
            .wrapping_add(e.mem.u32(previous))
            .wrapping_add(0x10);
        let mut write_limit = output.wrapping_add(0x200);
        let mut input_index = 1i32;
        let mut used = 0u32;
        let mut in_temp = false;
        loop {
            // Copy the entry, then the input words, and let the callback handle it.
            let units = i32::from(e.mem.u8(read + 3) >> 4);
            copy_units(e, read, write, units);
            read += entry_size(e, read);
            copy_units_words(e, input, copy);
            match e.mem.u8(write) & 0xf7 {
                1 => write += 0x10,
                2 | 3 | 6 => {
                    write = e.call(callback, &args![copy, write, write + 0x10]).u32();
                }
                4 | 5 => {
                    write = e.call(callback, &args![copy, write, write + 0x20]).u32();
                }
                _ => {}
            }
            if read == read_limit {
                if spare != 0 {
                    router_free(e, spare, SECTOR_SIZE);
                }
                spare = previous;
                if input_index >= e.mem.i32(track + 4) {
                    // The input is used up: store the last output sector.
                    if in_temp {
                        let bytes = write.wrapping_sub(temp);
                        copy_units(e, temp, output + used + 0x10, (bytes as i32) >> 4);
                        used = used.wrapping_add(bytes);
                    } else {
                        used = write.wrapping_sub(output).wrapping_sub(0x10);
                    }
                    e.mem.set_u32(output, used);
                    let size = output_index.wrapping_add(1);
                    let allocator = router_allocator(e);
                    let capacity = e.mem.u32(track + 8) & 0x3fff_ffff;
                    if (capacity as i32) < size as i32 {
                        let mut wanted = capacity.wrapping_mul(2);
                        if (size as i32) >= wanted as i32 {
                            wanted = size;
                        }
                        e.call(ARRAY_RESERVE, &args![allocator, track, wanted, 4u32]);
                    }
                    let data = e.mem.u32(track);
                    e.mem.set_u32(track + 4, size);
                    e.mem.set_u32(data + 4 * output_index, output);
                    if previous != 0 {
                        router_free(e, previous, SECTOR_SIZE);
                    }
                    return;
                }
                let sector = e.mem.u32(e.mem.u32(track) + 4 * input_index as u32);
                read = sector + 0x10;
                previous = sector;
                read_limit = sector.wrapping_add(e.mem.u32(sector)).wrapping_add(0x10);
                input_index += 1;
            }
            if write.wrapping_add(0x80) <= write_limit {
                continue;
            }
            if !in_temp {
                used = write.wrapping_sub(output).wrapping_sub(0x10);
                in_temp = true;
                write = temp;
                write_limit = temp_end;
                continue;
            }
            let bytes = write.wrapping_sub(temp);
            if (bytes as i32) <= 0x1f0 - used as i32 {
                copy_units(e, temp, output + used + 0x10, (bytes as i32) >> 4);
                used = used.wrapping_add(bytes);
                if (used as i32) < 0x1f0 {
                    write = temp;
                    continue;
                }
                in_temp = false;
            }
            // Store the full output sector and start a new one.
            e.mem.set_u32(output, used);
            if (output_index as i32) >= input_index {
                let allocator = router_allocator(e);
                let size = e.mem.u32(track + 4);
                let new_size = size.wrapping_add(1);
                let shift = size.wrapping_sub(input_index as u32);
                let capacity = e.mem.u32(track + 8) & 0x3fff_ffff;
                if (capacity as i32) < new_size as i32 {
                    let mut wanted = capacity.wrapping_mul(2);
                    if (new_size as i32) >= wanted as i32 {
                        wanted = new_size;
                    }
                    e.call(ARRAY_RESERVE, &args![allocator, track, wanted, 4u32]);
                }
                let source = e.mem.u32(track) + 4 * input_index as u32;
                e.call(MEM_MOVE, &args![source + 4, source, shift * 4]);
                input_index += 1;
                e.mem.set_u32(track + 4, new_size);
            }
            let data = e.mem.u32(track);
            e.mem.set_u32(data + 4 * output_index, output);
            output_index += 1;
            if spare != 0 {
                output = spare;
                spare = 0;
            } else {
                output = take_sector(e);
            }
            write = output.wrapping_add(0x10);
            write_limit = output.wrapping_add(0x200);
            if in_temp {
                copy_units(e, temp, write, (bytes as i32) >> 4);
                write = write.wrapping_add(bytes);
                in_temp = false;
            }
        }
    });
}

/// Copies the eight words at `from` to `to`.
fn copy_units_words(e: &mut Engine, from: u32, to: u32) {
    for word in 0..8 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00da48e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_UpdateShapeCollectionFilter` (named in the engine map, cdecl): visits all
/// agents of `track` with `input` and the callback `00da3f90` ([`fn_00da3f90`]).
pub fn hk_agent1n_machine_update_shape_collection_filter(e: &mut Engine, track: Ptr, input: Ptr) {
    hk_agent1n_machine_visit_all_agents(e, track, input, UPDATE_SHAPE_CALLBACK);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_3510, hk_agent1n_machine_invalidate_tim(Ptr, Ptr)),
        entry!(
            0x00da_35f0,
            hk_agent1n_machine_warp_time(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00da_37d0, fn_00da37d0(Ptr, Ptr, f32, Ptr, f32) -> u32),
        entry!(0x00da_3930, hk_agent1n_machine_weld(Ptr, Ptr, Ptr)),
        entry!(0x00da_3f90, fn_00da3f90(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00da_40e0, fn_00da40e0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00da_42b0, hk_agent1n_machine_destroy(Ptr, Ptr, u32, u32)),
        entry!(0x00da_4430, fn_00da4430(Ptr)),
        entry!(
            0x00da_44d0,
            hk_agent1n_machine_visit_all_agents(Ptr, Ptr, u32)
        ),
        entry!(
            0x00da_48e0,
            hk_agent1n_machine_update_shape_collection_filter(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// Doubles standing in for dispatcher functions and objects.
    const FUNCTION_A: u32 = 0x0900_0101;
    const FUNCTION_B: u32 = 0x0900_0102;
    const FUNCTION_C: u32 = 0x0900_0103;
    const SLOT_FIRST: u32 = 0x0900_0110;
    const BODY_SLOT: u32 = 0x0900_0113;
    const SECOND_BODY_SLOT: u32 = 0x0900_0114;
    const TAKE_DOUBLE: u32 = 0x0900_0120;
    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0121;
    const FREE_DATA_DOUBLE: u32 = 0x0900_0122;
    const CALLBACK_DOUBLE: u32 = 0x0900_0130;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        e
    }

    /// An engine that logs its calls.
    fn logged() -> Engine {
        let mut e = engine();
        e.call_log = Some(Vec::new());
        e
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

    /// The addresses of the logged calls that are in `wanted`, in order.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| wanted.contains(a))
            .collect()
    }

    /// A sector with entries `(type, agent type, size)` one after the other from `+0x10`; the
    /// used bytes are their sizes. Returns the sector and the entries' addresses.
    fn sector(e: &mut Engine, entries: &[(u8, u8, u8)]) -> (u32, Vec<u32>) {
        let at = e.mem.alloc(0x200);
        let mut offset = 0x10;
        let mut addresses = Vec::new();
        for &(kind, agent, size) in entries {
            let entry = at + offset;
            e.mem.set_u8(entry, kind);
            e.mem.set_u8(entry + 1, agent);
            e.mem.set_u8(entry + 3, size);
            addresses.push(entry);
            offset += u32::from(size);
        }
        e.mem.set_u32(at, offset - 0x10);
        (at, addresses)
    }

    /// A track (data, size, capacity word) holding `sectors`, with room for four more.
    fn track(e: &mut Engine, sectors: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * sectors.len() as u32 + 32);
        for (i, sector) in sectors.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *sector);
        }
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, sectors.len() as u32);
        e.mem.set_u32(at + 8, sectors.len() as u32 + 8);
        at
    }

    /// A dispatcher with the function `function` at `table` of the record of each agent type in
    /// `agents`, and an input structure whose first word it is. Returns both.
    fn dispatcher(e: &mut Engine, table: u32, agents: &[(u8, u32)]) -> (u32, u32) {
        let dispatcher = e.mem.alloc(0x2000);
        for &(agent, function) in agents {
            e.mem
                .set_u32(dispatcher + u32::from(agent) * 0x50 + table, function);
        }
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, dispatcher);
        (dispatcher, input)
    }

    /// An object whose vtable has `slots` `(byte offset, function)`.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for &(offset, function) in slots {
            e.mem.set_u32(vtable + offset, function);
        }
        let at = e.mem.alloc(0x30);
        e.mem.set_u32(at, vtable);
        at
    }

    /// Doubles for the memory router: its allocator hands out zeroed blocks (slot 4), takes
    /// blocks back (slot 8) and frees data (slot `0x10`). Returns the allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.register_double(TAKE_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(FREE_BLOCK_DOUBLE, |_, _| Ret::default());
        e.register_double(FREE_DATA_DOUBLE, |_, _| Ret::default());
        let allocator = object(
            e,
            &[
                (4, TAKE_DOUBLE),
                (8, FREE_BLOCK_DOUBLE),
                (0x10, FREE_DATA_DOUBLE),
            ],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// Doubles for `_reserve` and `_reserveMore`: the array moves to a block of the new
    /// capacity, keeping its elements.
    fn install_reserve(e: &mut Engine) {
        fn grow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for offset in 0..size * element {
                let byte = e.mem.u8(old + offset);
                e.mem.set_u8(block + offset, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
        }
        e.register_double(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff) * 2 + 4;
            grow(e, a[1], capacity, a[2]);
            Ret::default()
        });
    }

    /// Writes the constants the contact comparison reads.
    fn comparison_constants(e: &mut Engine) {
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f64(PARALLEL_DOT, 0.9999);
        e.mem.set_f64(LOW_DOT, 0.8);
        e.mem.set_f64(SMALL_DEPTH, 0.1);
        e.mem.set_f64(TOLERANCE_SCALE, 0.05);
        e.mem.set_f64(WELD_ADDEND, 0.0);
        e.mem.set_f32(INITIAL_DOT, 2.0);
    }

    /// A contact: point, normal at `+0x10`, distance at `+0x1c`, word at `+0x20`.
    fn contact(e: &mut Engine, normal: [f32; 3], distance: f32, word: u32) -> u32 {
        let at = e.mem.alloc(0x30);
        for (i, value) in normal.iter().enumerate() {
            e.mem.set_f32(at + 0x10 + 4 * i as u32, *value);
        }
        e.mem.set_f32(at + 0x1c, distance);
        e.mem.set_u32(at + 0x20, word);
        at
    }

    fn set_point(e: &mut Engine, contact: u32, point: [f32; 3]) {
        for (i, value) in point.iter().enumerate() {
            e.mem.set_f32(contact + 4 * i as u32, *value);
        }
    }

    // ---- invalidate time and warp time ----

    #[test]
    fn invalidate_tim_resets_the_times_and_calls_the_functions() {
        let mut e = logged();
        e.mem
            .set_f32(WARP_MISMATCH_TIME, f32::from_bits(0x7f80_0001));
        let (first, a) = sector(&mut e, &[(4, 1, 0x30), (0, 0, 0x10), (2, 2, 0x20)]);
        let (second, b) = sector(&mut e, &[(10, 1, 0x20), (13, 1, 0x30), (1, 0, 0x10)]);
        for word in 0..8 {
            e.mem.set_u32(a[0] + 0xc + 4 * word, 0x1111_1111);
        }
        e.mem.set_u32(a[2] + 0xc, 0x2222_2222);
        let walked = track(&mut e, &[first, second]);
        let (_, input) = dispatcher(&mut e, INVALIDATE_TIME_FUNCTION, &[(1, FUNCTION_A)]);
        e.register_double(FUNCTION_A, |_, _| Ret::default());
        e.call(0x00da_3510, &args![walked, input]);
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![
                vec![a[0], a[0] + 0x20, input],
                vec![b[0], b[0] + 0x10, input],
                vec![b[1], b[1] + 0x20, input],
            ]
        );
        // The time is the signalling NaN, quieted by the load and store; the 16 bytes at +0x10
        // are cleared.
        assert_eq!(e.mem.u32(a[0] + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(b[1] + 0xc), 0x7fc0_0001);
        for word in 0..4 {
            assert_eq!(e.mem.u32(a[0] + 0x10 + 4 * word), 0);
            assert_eq!(e.mem.u32(b[1] + 0x10 + 4 * word), 0);
        }
        assert_eq!(e.mem.u32(a[0] + 0x20), 0x1111_1111);
        // An entry of type 2 whose agent type has no function is left alone.
        assert_eq!(e.mem.u32(a[2] + 0xc), 0x2222_2222);
    }

    #[test]
    #[should_panic(expected = "endless loop")]
    fn invalidate_tim_stops_where_the_exe_would_loop_forever() {
        let mut e = engine();
        let (first, _) = sector(&mut e, &[(7, 1, 0x10), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        let (_, input) = dispatcher(&mut e, INVALIDATE_TIME_FUNCTION, &[]);
        e.call(0x00da_3510, &args![walked, input]);
    }

    #[test]
    fn warp_time_stores_the_new_time_when_the_old_one_matches() {
        let mut e = logged();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        let (first, a) = sector(
            &mut e,
            &[(4, 1, 0x30), (12, 1, 0x30), (2, 1, 0x20), (3, 2, 0x20)],
        );
        let (second, b) = sector(&mut e, &[(0, 0, 0x10), (14, 1, 0x20), (1, 0, 0x10)]);
        e.mem.set_f32(a[0] + 0xc, 2.0);
        e.mem.set_f32(a[1] + 0xc, 3.0);
        let walked = track(&mut e, &[first, second]);
        let (_, input) = dispatcher(&mut e, WARP_TIME_FUNCTION, &[(1, FUNCTION_A)]);
        e.register_double(FUNCTION_A, |_, _| Ret::default());
        e.call(0x00da_35f0, &args![walked, 2.0f32, 5.0f32, input]);
        let two = 2.0f32.to_bits();
        let five = 5.0f32.to_bits();
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![
                vec![a[0], a[0] + 0x20, two, five, input],
                vec![a[1], a[1] + 0x20, two, five, input],
                vec![a[2], a[2] + 0x10, two, five, input],
                vec![b[1], b[1] + 0x10, two, five, input],
            ]
        );
        assert_eq!(e.mem.f32(a[0] + 0xc), 5.0);
        assert_eq!(e.mem.f32(a[1] + 0xc), -1.0);
    }

    #[test]
    fn warp_time_treats_a_nan_time_as_a_mismatch_and_quiets_the_arguments() {
        let mut e = logged();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        let (first, a) = sector(&mut e, &[(5, 1, 0x30), (1, 0, 0x10)]);
        e.mem.set_u32(a[0] + 0xc, 0x7fc0_0001);
        let walked = track(&mut e, &[first]);
        let (_, input) = dispatcher(&mut e, WARP_TIME_FUNCTION, &[(1, FUNCTION_A)]);
        e.register_double(FUNCTION_A, |_, _| Ret::default());
        let signalling = f32::from_bits(0x7f80_0002);
        let quiet_nan = f32::from_bits(0x7fc0_0001);
        e.call(0x00da_35f0, &args![walked, quiet_nan, signalling, input]);
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![vec![a[0], a[0] + 0x20, 0x7fc0_0001, 0x7fc0_0002, input]]
        );
        assert_eq!(e.mem.f32(a[0] + 0xc), -1.0);
    }

    #[test]
    #[should_panic(expected = "endless loop")]
    fn warp_time_stops_where_the_exe_would_loop_forever() {
        let mut e = engine();
        let (first, _) = sector(&mut e, &[(15, 1, 0x10), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        let (_, input) = dispatcher(&mut e, WARP_TIME_FUNCTION, &[]);
        e.call(0x00da_35f0, &args![walked, 0.0f32, 0.0f32, input]);
    }

    // ---- contact comparison ----

    /// Calls `00da37d0` with the contacts, a tolerance of 1.0, a limit of 1.0 and a margin of 0.
    fn relation(e: &mut Engine, this: u32, other: u32) -> u32 {
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 1.0);
        e.call(0x00da_37d0, &args![this, other, 1.0f32, limit, 0.0f32])
            .u32()
    }

    #[test]
    fn contact_comparison_returns_zero_when_the_other_contact_is_far_behind() {
        let mut e = engine();
        comparison_constants(&mut e);
        let this = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 3);
        let other = contact(&mut e, [0.0, 0.0, 1.0], 2.0, 3);
        set_point(&mut e, other, [0.0, 0.0, 5.0]);
        assert_eq!(relation(&mut e, this, other), 0);
    }

    #[test]
    fn contact_comparison_returns_one_when_the_contact_is_in_front() {
        let mut e = engine();
        comparison_constants(&mut e);
        // The distance 1.0 less 0.05 is below 2.0.
        let this = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 3);
        let other = contact(&mut e, [0.0, 0.0, 1.0], 2.0, 3);
        assert_eq!(relation(&mut e, this, other), 1);
        // A distance of 5.0 reaches 2.0.
        e.mem.set_f32(this + 0x1c, 5.0);
        assert_eq!(relation(&mut e, this, other), 0);
        // An unordered distance counts as below.
        e.mem.set_f32(this + 0x1c, f32::NAN);
        assert_eq!(relation(&mut e, this, other), 1);
    }

    #[test]
    fn contact_comparison_treats_parallel_contacts_of_a_new_pair_as_unrelated() {
        let mut e = engine();
        comparison_constants(&mut e);
        let this = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 0xffff);
        let other = contact(&mut e, [0.0, 0.0, 1.0], 2.0, 3);
        assert_eq!(relation(&mut e, this, other), 0);
    }

    #[test]
    fn contact_comparison_returns_two_for_a_conflict() {
        let mut e = engine();
        comparison_constants(&mut e);
        let this = contact(&mut e, [0.0, 0.0, 1.0], -5.0, 0xffff);
        let other = contact(&mut e, [1.0, 0.0, 0.0], -5.0, 3);
        assert_eq!(relation(&mut e, this, other), 2);
        // Distances that sum to more than -2.0 are no conflict: the contact is not below.
        e.mem.set_f32(this + 0x1c, -0.5);
        e.mem.set_f32(other + 0x1c, -0.5);
        assert_eq!(relation(&mut e, this, other), 0);
        // A tolerance and a margin that sum below the depth end the comparison early.
        let limit = e.mem.alloc(4);
        let result = e
            .call(0x00da_37d0, &args![this, other, -4.0f32, limit, 1.0f32])
            .u32();
        assert_eq!(result, 0);
    }

    // ---- weld ----

    const CALLBACK_C: u32 = 0x0900_0201;
    const CALLBACK_D: u32 = 0x0900_0202;
    const CALLBACK_E: u32 = 0x0900_0203;
    const SLOT_C: u32 = 0x0900_0210;
    const SLOT_10: u32 = 0x0900_0211;
    const SLOT_14: u32 = 0x0900_0212;
    const WELD_BODY_SLOT: u32 = 0x0900_0213;

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

    /// The labels of the profiling records written since the stream was made.
    fn record_labels(e: &Engine, stream: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        (stream + 0x40..end)
            .step_by(12)
            .map(|at| e.mem.u32(at))
            .collect()
    }

    /// The records `[body, key, word, collidable]` the weld callback was handed.
    type Shapes = Rc<RefCell<Vec<[u32; 4]>>>;
    /// The calls of the penetration depth function: its arguments, the bodies and the contacts.
    type Penetrations = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, Vec<u32>)>>>;

    /// The things a weld test looks at.
    struct Weld {
        ctx: u32,
        state: u32,
        list: u32,
        bodies: u32,
        shapes: Shapes,
        penetration: Penetrations,
        pool: u32,
    }

    /// A weld setup with records `(contact, entry word)`, the peer contacts, the handles the
    /// callback returns in order (as 16-bit values below junk) and the flag words of the kinds
    /// 0 to 3 of the dispatcher.
    fn weld_setup(
        e: &mut Engine,
        records: &[(u32, u32)],
        peers: &[u32],
        handles: &[u32],
        flags: [u32; 4],
    ) -> Weld {
        comparison_constants(e);
        let (dispatcher, _) = dispatcher(e, 0x16bc, &[(1, CALLBACK_C)]);
        e.mem.set_u32(dispatcher + 0x50 + 0x16c0, CALLBACK_D);
        e.mem.set_u32(dispatcher + 0x50 + 0x16c4, CALLBACK_E);
        for (kind, flag) in flags.iter().enumerate() {
            e.mem.set_u32(dispatcher + 0x110 + 4 * kind as u32, *flag);
        }
        for function in [CALLBACK_C, CALLBACK_D, CALLBACK_E, SLOT_10, SLOT_14] {
            e.register_double(function, |_, _| Ret::default());
        }
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 1.0);
        let holder = e.mem.alloc(0x70);
        e.mem.set_u32(holder, dispatcher);
        e.mem.set_u32(holder + 0x60, limit);
        let world = e.mem.alloc(0x20);
        e.mem.set_f32(world + 0x10, 1.0);
        e.mem.set_u32(world + 0xc, 1);
        let ctx0 = e.mem.alloc(0x10);
        e.mem.set_u32(ctx0, world);
        e.mem.set_u32(ctx0 + 8, 0x7a7a);
        let ctx1 = e.mem.alloc(0x10);
        e.mem.set_u32(ctx1 + 8, 0x6b6b);
        let shapes: Shapes = Rc::default();
        let shape_log = Rc::clone(&shapes);
        let queue = Rc::new(RefCell::new(handles.to_vec()));
        e.register_double(SLOT_C, move |e, a| {
            let at = a[2];
            shape_log
                .borrow_mut()
                .push(std::array::from_fn(|i| e.mem.u32(at + 4 * i as u32)));
            let handle = queue.borrow_mut().remove(0);
            Ret {
                eax: 0xbeef_0000 | handle,
                ..Ret::default()
            }
        });
        let callbacks = object(e, &[(0xc, SLOT_C), (0x10, SLOT_10), (0x14, SLOT_14)]);
        let ctx = e.mem.alloc(0x40);
        e.mem.set_u32(ctx, ctx0);
        e.mem.set_u32(ctx + 4, ctx1);
        e.mem.set_u32(ctx + 0xc, holder);
        e.mem.set_u32(ctx + 0x10, callbacks);
        // The bodies: one per record, kind 2.
        let mut keyed = HashMap::new();
        let list = e.mem.alloc(0x408 + 12 * records.len() as u32 + 16);
        for (i, (contact, key)) in records.iter().enumerate() {
            let entry = e.mem.alloc(0x20);
            e.mem.set_u8(entry + 1, 1);
            e.mem.set_u32(entry + 8, *key);
            let at = list + 0x408 + 12 * i as u32;
            e.mem.set_u32(at, *contact);
            e.mem.set_u32(at + 4, entry);
            e.mem.set_u32(at + 8, 0x300 + i as u32);
            let body = e.mem.alloc(0x20);
            e.mem.set_u32(body + 0xc, 2);
            keyed.insert(*key, body);
        }
        e.register_double(WELD_BODY_SLOT, move |_, a| Ret {
            eax: keyed[&a[1]],
            ..Ret::default()
        });
        let bodies = object(e, &[(0x14, WELD_BODY_SLOT)]);
        e.mem
            .set_u32(list, list + 0x408 + 12 * records.len() as u32);
        for (i, peer) in peers.iter().enumerate() {
            e.mem.set_u32(list + 8 + 4 * i as u32, *peer);
        }
        e.mem.set_u32(list + 4, list + 8 + 4 * peers.len() as u32);
        let pool = e.mem.alloc(0x30 * 4);
        let state = e.mem.alloc(0x3100);
        e.mem.set_u32(state, pool + 0x30 * 3);
        e.mem.set_u32(state + 4, 0x5151);
        e.mem.set_u32(state + 0x3080, list);
        for word in 0..9 {
            e.mem.set_u32(pool + 0x60 + 4 * word, 0xa0a0_0000 + word);
        }
        let penetration: Penetrations = Rc::default();
        let penetration_log = Rc::clone(&penetration);
        e.register_double(CALC_MULTI_PENETRATION_DEPTH, move |e, a| {
            let words = |at: u32| vec![e.mem.u32(at), e.mem.u32(at + 4)];
            penetration_log
                .borrow_mut()
                .push((a.to_vec(), words(a[2]), words(a[5])));
            Ret::default()
        });
        Weld {
            ctx,
            state,
            list,
            bodies,
            shapes,
            penetration,
            pool,
        }
    }

    /// The record `i` of the weld list: contact, entry, word.
    fn weld_record(e: &Engine, weld: &Weld, i: u32) -> [u32; 3] {
        let at = weld.list + 0x408 + 12 * i;
        [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
    }

    fn weld_run(e: &mut Engine, weld: &Weld) {
        e.call(0x00da_3930, &args![weld.ctx, weld.bodies, weld.state]);
    }

    #[test]
    fn weld_hands_records_to_the_callback_and_stores_the_handles() {
        let mut e = logged();
        let first = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 3);
        let second = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 3);
        let weld = weld_setup(
            &mut e,
            &[(first, 0x901), (second, 0x902)],
            &[],
            &[7, 0xffff],
            [0x40000; 4],
        );
        weld_run(&mut e, &weld);
        let record_0 = weld_record(&e, &weld, 0);
        let record_1 = weld_record(&e, &weld, 1);
        let ctx0 = e.mem.u32(weld.ctx);
        let ctx1 = e.mem.u32(weld.ctx + 4);
        let holder = e.mem.u32(weld.ctx + 0xc);
        let callbacks = e.mem.u32(weld.ctx + 0x10);
        // The callback got the context's first object, a body record, the holder, the state,
        // 0 and the contact.
        let slot_calls = calls(&e, SLOT_C);
        assert_eq!(slot_calls.len(), 2);
        assert_eq!(slot_calls[0][0..2], [callbacks, ctx0]);
        assert_eq!(slot_calls[0][3..], [holder, weld.state, 0, first]);
        assert_eq!(slot_calls[1][6], second);
        let shapes = weld.shapes.borrow();
        assert_eq!(calls(&e, WELD_BODY_SLOT).len(), 2);
        assert_eq!(shapes[0][1..], [0x901, 0x6b6b, ctx1]);
        assert_eq!(shapes[1][1], 0x902);
        // The first record keeps its handle (slot 0x10 with -1 follows it), the second has none.
        assert_eq!(calls(&e, SLOT_14), Vec::<Vec<u32>>::new());
        assert_eq!(
            calls(&e, SLOT_10),
            vec![vec![callbacks, 0xffff_ffff], vec![callbacks, 0xffff_ffff],]
        );
        // Last to first: the record without a handle, then the one with handle 7.
        assert_eq!(
            order(&e, &[SLOT_C, SLOT_10, CALLBACK_C, CALLBACK_D]),
            vec![SLOT_C, SLOT_10, SLOT_C, SLOT_10, CALLBACK_C, CALLBACK_D]
        );
        assert_eq!(
            calls(&e, CALLBACK_C),
            vec![vec![record_1[1], record_1[2], 0xffff]]
        );
        assert_eq!(
            calls(&e, CALLBACK_D),
            vec![vec![record_0[1], record_0[2], 7]]
        );
        assert_eq!(e.mem.u32(first + 0x20), 7);
        // The newest contact of the pool replaced the second one.
        assert_eq!(e.mem.u32(weld.state), weld.pool + 0x60);
        for word in 0..9 {
            assert_eq!(e.mem.u32(second + 4 * word), 0xa0a0_0000 + word);
        }
    }

    #[test]
    fn weld_leaves_a_record_that_is_in_front_of_a_peer_alone() {
        let mut e = logged();
        let ahead = contact(&mut e, [0.0, 0.0, 1.0], 2.0, 3);
        let peer = contact(&mut e, [0.0, 0.0, 1.0], 1.0, 3);
        let weld = weld_setup(&mut e, &[(ahead, 0x901)], &[peer], &[], [0x40000; 4]);
        weld_run(&mut e, &weld);
        // No callback, no handle: the record is torn down like one without a handle.
        assert!(calls(&e, SLOT_C).is_empty());
        assert!(calls(&e, WELD_BODY_SLOT).is_empty());
        let record = weld_record(&e, &weld, 0);
        assert_eq!(
            calls(&e, CALLBACK_C),
            vec![vec![record[1], record[2], 0xffff]]
        );
    }

    /// Three contacts with perpendicular normals at the same point, all new, in conflict.
    fn conflicting_contacts(e: &mut Engine) -> [u32; 3] {
        let normals = [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        std::array::from_fn(|i| contact(e, normals[i], -5.0, 0xffff))
    }

    #[test]
    fn weld_resolves_the_queued_pair_with_the_smallest_dot_product() {
        let mut e = logged();
        let stream = monitor_stream(&mut e);
        let contacts = conflicting_contacts(&mut e);
        let weld = weld_setup(
            &mut e,
            &[
                (contacts[0], 0x901),
                (contacts[1], 0x902),
                (contacts[2], 0x903),
            ],
            &contacts,
            &[5, 6, 7],
            [0x40000; 4],
        );
        weld_run(&mut e, &weld);
        let callbacks = e.mem.u32(weld.ctx + 0x10);
        let world = e.mem.u32(e.mem.u32(weld.ctx));
        let records: Vec<[u32; 3]> = (0..3).map(|i| weld_record(&e, &weld, i)).collect();
        // The queued records are announced to the dispatcher in order.
        assert_eq!(
            calls(&e, CALLBACK_E),
            records
                .iter()
                .map(|r| vec![r[1], r[2], 0xffff])
                .collect::<Vec<_>>()
        );
        // The first pair (the dot products are all 0) is resolved.
        let penetration = weld.penetration.borrow();
        assert_eq!(penetration.len(), 1);
        let (args_seen, bodies, contact_pair) = &penetration[0];
        assert_eq!(args_seen[0], 0x7a7a);
        assert_eq!(args_seen[1], world);
        assert_eq!(args_seen[3], 2);
        assert_eq!(args_seen[4], weld.ctx + 0x20);
        assert_eq!(*contact_pair, vec![contacts[0], contacts[1]]);
        assert_eq!(bodies.len(), 2);
        assert_ne!(bodies[0], bodies[1]);
        assert_eq!(record_labels(&e, stream), vec![LABEL_CONFLICTS, LABEL_END]);
        // The third record stays queued alone: its handle is released.
        assert_eq!(calls(&e, SLOT_14), vec![vec![callbacks, 7, 0x5151]]);
        assert_eq!(
            order(&e, &[SLOT_14, CALLBACK_C, CALLBACK_D]),
            vec![SLOT_14, CALLBACK_C, CALLBACK_D, CALLBACK_D]
        );
        // Records 1 and 0 keep their handles 6 and 5.
        assert_eq!(e.mem.u32(contacts[1] + 0x20), 6);
        assert_eq!(e.mem.u32(contacts[0] + 0x20), 5);
    }

    #[test]
    fn weld_stops_resolving_when_a_body_is_not_flagged() {
        let mut e = logged();
        let stream = monitor_stream(&mut e);
        let contacts = conflicting_contacts(&mut e);
        // Kind 1 (the world) passes, kind 2 (the bodies) does not.
        let weld = weld_setup(
            &mut e,
            &[(contacts[0], 0x901), (contacts[1], 0x902)],
            &contacts[..2],
            &[5, 6],
            [0, 0x40000, 0, 0],
        );
        weld_run(&mut e, &weld);
        assert!(weld.penetration.borrow().is_empty());
        assert_eq!(record_labels(&e, stream), vec![LABEL_CONFLICTS]);
        // Phase 1 asked for both bodies, the resolving loop for the first one more.
        assert_eq!(calls(&e, WELD_BODY_SLOT).len(), 3);
        // Both are still queued: the first handle is released.
        let callbacks = e.mem.u32(weld.ctx + 0x10);
        assert_eq!(calls(&e, SLOT_14), vec![vec![callbacks, 5, 0x5151]]);
        assert_eq!(e.mem.u32(contacts[1] + 0x20), 6);
    }

    #[test]
    fn weld_stops_resolving_when_the_world_is_not_flagged() {
        let mut e = logged();
        let stream = monitor_stream(&mut e);
        let contacts = conflicting_contacts(&mut e);
        let weld = weld_setup(
            &mut e,
            &[(contacts[0], 0x901), (contacts[1], 0x902)],
            &contacts[..2],
            &[5, 6],
            [0x40000, 0, 0x40000, 0x40000],
        );
        weld_run(&mut e, &weld);
        assert!(weld.penetration.borrow().is_empty());
        assert_eq!(record_labels(&e, stream), vec![LABEL_CONFLICTS]);
        assert_eq!(calls(&e, WELD_BODY_SLOT).len(), 2);
    }

    // ---- update shape collection filter and its callbacks ----

    /// A callback context: words `+0` to `+0x18`; the object of `+0x10` holds the dispatcher and
    /// leads to an object whose slot `slot` returns a pointer to a flag byte.
    struct Context {
        ctx: u32,
        entry: u32,
        dispatcher: u32,
        ctx0: u32,
        ctx1: u32,
        flag: u32,
    }

    fn context(e: &mut Engine, slot: u32, flagged: bool, kind: u8, with_update: bool) -> Context {
        for function in [
            FUNCTION_A,
            FUNCTION_B,
            FUNCTION_C,
            BODY_SLOT,
            SECOND_BODY_SLOT,
        ] {
            e.register_double(function, |_, _| Ret::default());
        }
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, u8::from(flagged));
        e.register_double(SLOT_FIRST, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.register_double(BODY_SLOT, |_, _| Ret {
            eax: 0xb0d1,
            ..Ret::default()
        });
        e.register_double(SECOND_BODY_SLOT, |_, _| Ret {
            eax: 0xb0d2,
            ..Ret::default()
        });
        let dispatcher = e.mem.alloc(0x2000);
        e.mem
            .set_u32(dispatcher + 0x50 + DESTROY_FUNCTION, FUNCTION_A);
        if with_update {
            e.mem
                .set_u32(dispatcher + 0x50 + UPDATE_FILTER_FUNCTION, FUNCTION_B);
        }
        let decider = object(e, &[(slot, SLOT_FIRST)]);
        let ctx4 = e.mem.alloc(0x20);
        e.mem.set_u32(ctx4, dispatcher);
        e.mem.set_u32(ctx4 + 0xc, decider - 0xc);
        let ctx0 = e.mem.alloc(0x10);
        e.mem.set_u32(ctx0 + 8, 0x7a7a);
        let ctx1 = e.mem.alloc(0x10);
        e.mem.set_u32(ctx1 + 8, 0x6b6b);
        let ctx2 = object(e, &[(0x14, SECOND_BODY_SLOT)]);
        let ctx3 = object(e, &[(0x14, BODY_SLOT)]);
        let ctx = e.mem.alloc(0x20);
        for (offset, value) in [
            (0, ctx0),
            (4, ctx1),
            (8, ctx2),
            (0xc, ctx3),
            (0x10, ctx4),
            (0x14, 5),
            (0x18, 6),
        ] {
            e.mem.set_u32(ctx + offset, value);
        }
        let entry = e.mem.alloc(0x40);
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, 1);
        e.mem.set_u8(entry + 3, 0x30);
        e.mem.set_u32(entry + 4, 0x4444);
        e.mem.set_u32(entry + 8, 0x8888);
        Context {
            ctx,
            entry,
            dispatcher,
            ctx0,
            ctx1,
            flag,
        }
    }

    /// Records the words at the third and fourth arguments of every call.
    fn capture_pair(e: &mut Engine, address: u32) -> (Shapes, Shapes) {
        let first: Shapes = Rc::default();
        let second: Shapes = Rc::default();
        let (log_first, log_second) = (Rc::clone(&first), Rc::clone(&second));
        e.register_double(address, move |e, a| {
            for (log, at) in [(&log_first, a[2]), (&log_second, a[3])] {
                log.borrow_mut()
                    .push(std::array::from_fn(|i| e.mem.u32(at + 4 * i as u32)));
            }
            Ret::default()
        });
        (first, second)
    }

    /// Registers a double at `address` that records the four words at argument `index`.
    fn capture(e: &mut Engine, address: u32, index: usize) -> Shapes {
        let seen: Shapes = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            let at = a[index];
            log.borrow_mut()
                .push(std::array::from_fn(|i| e.mem.u32(at + 4 * i as u32)));
            Ret::default()
        });
        seen
    }

    #[test]
    fn fn_00da3f90_destroys_the_agent_when_the_flag_is_clear() {
        let mut e = logged();
        let c = context(&mut e, 4, false, 2, true);
        let got = e.call(0x00da_3f90, &args![c.ctx, c.entry, 0x1234u32]).u32();
        assert_eq!(got, c.entry);
        let ctx3 = e.mem.u32(c.ctx + 0xc);
        let ctx4 = e.mem.u32(c.ctx + 0x10);
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![vec![c.entry, 0x1234, 5, 6, c.dispatcher]]
        );
        let slot = calls(&e, SLOT_FIRST);
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0][0], e.mem.u32(ctx4 + 0xc) + 0xc);
        assert_eq!(slot[0][2..], [ctx4, c.ctx0, c.ctx1, ctx3, 0x8888]);
        assert!(calls(&e, BODY_SLOT).is_empty());
        assert_eq!(e.mem.u8(c.flag), 0);
    }

    #[test]
    fn fn_00da3f90_returns_the_next_entry_without_an_update_function() {
        let mut e = logged();
        let c = context(&mut e, 4, true, 2, false);
        let got = e.call(0x00da_3f90, &args![c.ctx, c.entry, 0x1234u32]).u32();
        assert_eq!(got, c.entry + 0x30);
        assert!(calls(&e, FUNCTION_A).is_empty());
        assert!(calls(&e, BODY_SLOT).is_empty());
    }

    #[test]
    fn fn_00da3f90_updates_with_the_context_object_and_the_body_record() {
        let mut e = logged();
        let c = context(&mut e, 4, true, 2, true);
        let record = capture(&mut e, FUNCTION_B, 3);
        let got = e.call(0x00da_3f90, &args![c.ctx, c.entry, 0x1234u32]).u32();
        assert_eq!(got, c.entry + 0x30);
        let ctx3 = e.mem.u32(c.ctx + 0xc);
        let ctx4 = e.mem.u32(c.ctx + 0x10);
        let body_calls = calls(&e, BODY_SLOT);
        assert_eq!(body_calls.len(), 1);
        assert_eq!(body_calls[0][..2], [ctx3, 0x8888]);
        let update = calls(&e, FUNCTION_B);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][..4], [c.entry, 0x1234, c.ctx0, update[0][3]]);
        assert_eq!(update[0][4..], [ctx4, 5, 6]);
        assert_eq!(record.borrow()[0], [0xb0d1, 0x8888, 0x6b6b, c.ctx1]);
        // The other way round for entry type 3.
        let mut e = logged();
        let c = context(&mut e, 4, true, 3, true);
        e.call(0x00da_3f90, &args![c.ctx, c.entry, 0x1234u32]);
        let update = calls(&e, FUNCTION_B);
        assert_eq!(update[0][3], c.ctx0);
    }

    #[test]
    fn fn_00da3f90_resolves_transform_shapes_for_entries_with_flag_eight() {
        for (kind, swapped) in [(10u8, false), (11, true)] {
            let mut e = logged();
            let c = context(&mut e, 4, true, kind, true);
            let changed_before = Rc::new(RefCell::new(0xffu8));
            let seen = Rc::clone(&changed_before);
            e.register_double(PROCESS_TRANSFORMED_SHAPES, move |e, a| {
                *seen.borrow_mut() = e.mem.u8(a[4]);
                Ret {
                    eax: 0x7777,
                    ..Ret::default()
                }
            });
            e.call(0x00da_3f90, &args![c.ctx, c.entry, 0x1234u32]);
            let process = calls(&e, PROCESS_TRANSFORMED_SHAPES);
            assert_eq!(process.len(), 1);
            assert_eq!(process[0][3], 4);
            assert_eq!(*changed_before.borrow(), 0);
            let update = calls(&e, FUNCTION_B);
            let expected = if swapped {
                [0x7777, c.ctx0]
            } else {
                [c.ctx0, 0x7777]
            };
            assert_eq!(update[0][2..4], expected);
        }
    }

    #[test]
    fn fn_00da40e0_destroys_the_agent_when_the_flag_is_clear() {
        let mut e = logged();
        let c = context(&mut e, 0, false, 2, true);
        let got = e.call(0x00da_40e0, &args![c.ctx, c.entry, 0x1234u32]).u32();
        assert_eq!(got, c.entry);
        let ctx2 = e.mem.u32(c.ctx + 8);
        let ctx3 = e.mem.u32(c.ctx + 0xc);
        let ctx4 = e.mem.u32(c.ctx + 0x10);
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![vec![c.entry, 0x1234, 5, 6, c.dispatcher]]
        );
        let slot = calls(&e, SLOT_FIRST);
        assert_eq!(
            slot[0][2..],
            [ctx4, c.ctx0, c.ctx1, ctx2, ctx3, 0x4444, 0x8888]
        );
    }

    #[test]
    fn fn_00da40e0_updates_with_the_two_body_records() {
        let mut e = logged();
        let c = context(&mut e, 0, true, 2, true);
        let (first, second) = capture_pair(&mut e, FUNCTION_B);
        let got = e.call(0x00da_40e0, &args![c.ctx, c.entry, 0x1234u32]).u32();
        assert_eq!(got, c.entry + 0x30);
        let ctx2 = e.mem.u32(c.ctx + 8);
        let ctx3 = e.mem.u32(c.ctx + 0xc);
        let ctx4 = e.mem.u32(c.ctx + 0x10);
        let first_body = calls(&e, BODY_SLOT);
        let second_body = calls(&e, SECOND_BODY_SLOT);
        assert_eq!(first_body[0][..2], [ctx3, 0x8888]);
        assert_eq!(second_body[0][..2], [ctx2, 0x4444]);
        let update = calls(&e, FUNCTION_B);
        assert_eq!(update[0][..2], [c.entry, 0x1234]);
        assert_eq!(update[0][4..], [ctx4, 5, 6]);
        // The first record belongs to the context's first collidable.
        assert_eq!(first.borrow()[0], [0xb0d2, 0x4444, 0x7a7a, c.ctx0]);
        assert_eq!(second.borrow()[0], [0xb0d1, 0x8888, 0x6b6b, c.ctx1]);
        // The other way round for entry type 5.
        let mut e = logged();
        let c = context(&mut e, 0, true, 5, true);
        let (one, two) = capture_pair(&mut e, FUNCTION_B);
        e.call(0x00da_40e0, &args![c.ctx, c.entry, 0x1234u32]);
        assert_eq!(one.borrow()[0], [0xb0d1, 0x8888, 0x6b6b, c.ctx1]);
        assert_eq!(two.borrow()[0], [0xb0d2, 0x4444, 0x7a7a, c.ctx0]);
    }

    #[test]
    fn fn_00da40e0_resolves_transform_shapes_of_both_records() {
        for (kind, swapped) in [(10u8, false), (11, true)] {
            let mut e = logged();
            let c = context(&mut e, 0, true, kind, true);
            let count = Rc::new(RefCell::new(0u32));
            let counter = Rc::clone(&count);
            e.register_double(PROCESS_TRANSFORMED_SHAPES, move |_, _| {
                *counter.borrow_mut() += 1;
                Ret {
                    eax: 0x7000 + *counter.borrow(),
                    ..Ret::default()
                }
            });
            e.call(0x00da_40e0, &args![c.ctx, c.entry, 0x1234u32]);
            let process = calls(&e, PROCESS_TRANSFORMED_SHAPES);
            assert_eq!(process.len(), 2);
            // The second record is resolved first (result 0x7001), then the first (0x7002).
            let update = calls(&e, FUNCTION_B);
            let expected = if swapped {
                [0x7001, 0x7002]
            } else {
                [0x7002, 0x7001]
            };
            assert_eq!(update[0][2..4], expected);
        }
    }

    // ---- destroy ----

    #[test]
    fn destroy_calls_the_functions_and_gives_the_sectors_and_storage_back() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        let (first, a) = sector(&mut e, &[(4, 1, 0x30), (0, 0, 0x10), (2, 1, 0x20)]);
        let (second, b) = sector(&mut e, &[(13, 1, 0x30), (0, 0, 0x10), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first, second]);
        let data = e.mem.u32(walked);
        let (dispatcher, _) = dispatcher(&mut e, DESTROY_FUNCTION, &[(1, FUNCTION_A)]);
        e.register_double(FUNCTION_A, |_, _| Ret::default());
        e.call(0x00da_42b0, &args![walked, dispatcher, 11u32, 22u32]);
        assert_eq!(
            calls(&e, FUNCTION_A),
            vec![
                vec![a[0], a[0] + 0x20, 11, 22, dispatcher],
                vec![a[2], a[2] + 0x10, 11, 22, dispatcher],
                vec![b[0], b[0] + 0x20, 11, 22, dispatcher],
            ]
        );
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![
                vec![allocator, first, 0x200],
                vec![allocator, second, 0x200]
            ]
        );
        assert_eq!(calls(&e, FREE_DATA_DOUBLE), vec![vec![allocator, data, 40]]);
        assert_eq!(
            [
                e.mem.u32(walked),
                e.mem.u32(walked + 4),
                e.mem.u32(walked + 8)
            ],
            [0, 0, 0x8000_0000]
        );
        // The first sector is given back before the function of the second sector runs.
        assert_eq!(
            order(&e, &[FUNCTION_A, FREE_BLOCK_DOUBLE, FREE_DATA_DOUBLE]),
            vec![
                FUNCTION_A,
                FUNCTION_A,
                FREE_BLOCK_DOUBLE,
                FUNCTION_A,
                FREE_BLOCK_DOUBLE,
                FREE_DATA_DOUBLE
            ]
        );
    }

    #[test]
    fn destroy_keeps_storage_it_does_not_own() {
        let mut e = logged();
        install_allocator(&mut e);
        let (first, _) = sector(&mut e, &[(1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        e.mem.set_u32(walked + 8, 0x8000_0001);
        let (dispatcher, _) = dispatcher(&mut e, DESTROY_FUNCTION, &[]);
        e.call(0x00da_42b0, &args![walked, dispatcher, 0u32, 0u32]);
        assert!(calls(&e, FREE_DATA_DOUBLE).is_empty());
        assert_eq!(calls(&e, FREE_BLOCK_DOUBLE).len(), 1);
        assert_eq!(e.mem.u32(walked + 8), 0x8000_0000);
    }

    #[test]
    #[should_panic(expected = "endless loop")]
    fn destroy_stops_where_the_exe_would_loop_forever() {
        let mut e = engine();
        install_allocator(&mut e);
        let (first, _) = sector(&mut e, &[(9, 0, 0x10), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        let (dispatcher, _) = dispatcher(&mut e, DESTROY_FUNCTION, &[]);
        e.call(0x00da_42b0, &args![walked, dispatcher, 0u32, 0u32]);
    }

    // ---- first sector ----

    #[test]
    fn fn_00da4430_makes_a_track_with_one_end_sector() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_reserve(&mut e);
        let walked = e.mem.alloc(0x10);
        e.mem.set_u32(walked + 4, 9);
        e.mem.set_u32(walked + 8, 0x8000_0000);
        e.call(0x00da_4430, &args![walked]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![allocator, walked, 4, 4]]
        );
        assert!(calls(&e, ARRAY_RESERVE_MORE).is_empty());
        assert_eq!(e.mem.u32(walked + 4), 1);
        let sector = e.mem.u32(e.mem.u32(walked));
        assert_eq!(e.mem.u32(sector), 0x10);
        assert_eq!(e.mem.u8(sector + 0x10), 1);
        assert_eq!(e.mem.u8(sector + 0x13), 0x10);
        assert_eq!(e.mem.u32(sector + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(sector + 0x18), 0xffff_ffff);
        assert_eq!(calls(&e, TAKE_DOUBLE), vec![vec![allocator, 0x200]]);
        // Three router lookups: reserve, room check, sector.
        assert_eq!(calls(&e, MEMORY_ROUTER).len(), 3);
    }

    #[test]
    fn fn_00da4430_grows_a_small_array_by_doubling_and_leaves_a_big_one() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_reserve(&mut e);
        let walked = e.mem.alloc(0x10);
        e.mem.set_u32(walked + 8, 3);
        e.call(0x00da_4430, &args![walked]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![allocator, walked, 6, 4]]
        );
        let mut e = logged();
        install_allocator(&mut e);
        install_reserve(&mut e);
        let walked = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x40);
        e.mem.set_u32(walked, storage);
        e.mem.set_u32(walked + 8, 0x8000_0005);
        e.call(0x00da_4430, &args![walked]);
        assert!(calls(&e, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.u32(walked + 4), 1);
    }

    #[test]
    fn fn_00da4430_asks_for_more_room_when_the_array_is_still_full() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        // A reserve that changes nothing leaves size and capacity both 0.
        e.register_double(ARRAY_RESERVE, |_, _| Ret::default());
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let block = e.mem.alloc(16);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let walked = e.mem.alloc(0x10);
        e.call(0x00da_4430, &args![walked]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, walked, 4]]
        );
        assert_eq!(e.mem.u32(walked + 4), 1);
    }

    // ---- visiting all agents ----

    /// The calls of a visiting callback: its arguments and the eight words of its first argument.
    type Visits = Rc<RefCell<Vec<(Vec<u32>, [u32; 8])>>>;

    /// Registers the visiting callback: records `(copy, entry, data, the eight words of the
    /// copy)` and sets the entry's size byte to `size` (when given) before returning the end of
    /// the entry.
    fn install_visitor(e: &mut Engine, size: Option<u8>) -> Visits {
        let seen: Visits = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(CALLBACK_DOUBLE, move |e, a| {
            let words = std::array::from_fn(|i| e.mem.u32(a[0] + 4 * i as u32));
            log.borrow_mut().push((a.to_vec(), words));
            if let Some(size) = size {
                e.mem.set_u8(a[1] + 3, size);
            }
            Ret {
                eax: a[1] + u32::from(e.mem.u8(a[1] + 3)),
                ..Ret::default()
            }
        });
        seen
    }

    /// Eight distinct words to visit with.
    fn visit_input(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(at + 4 * word, 0x1000 + word);
        }
        at
    }

    /// The entries `(type, size)` of the sector stored at `index` in the track, and its used
    /// bytes.
    fn stored_entries(e: &Engine, walked: u32, index: u32) -> (Vec<(u8, u8)>, u32) {
        let sector = e.mem.u32(e.mem.u32(walked) + 4 * index);
        let used = e.mem.u32(sector);
        let mut entries = Vec::new();
        let mut at = sector + 0x10;
        while at < sector + 0x10 + used {
            let size = e.mem.u8(at + 3);
            entries.push((e.mem.u8(at), size));
            at += u32::from(size);
        }
        (entries, used)
    }

    #[test]
    fn visit_all_agents_copies_the_kept_entries_into_a_new_sector() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        let seen = install_visitor(&mut e, None);
        let input = visit_input(&mut e);
        let (first, _) = sector(
            &mut e,
            &[
                (2, 1, 0x20),
                (7, 0, 0x10),
                (0, 0, 0x10),
                (5, 1, 0x30),
                (1, 0, 0x10),
            ],
        );
        let walked = track(&mut e, &[first]);
        e.call(0x00da_44d0, &args![walked, input, CALLBACK_DOUBLE]);
        let (entries, used) = stored_entries(&e, walked, 0);
        // The unknown types 7 and 0 are dropped; the end entry (type 1) is kept.
        assert_eq!(entries, vec![(2, 0x20), (5, 0x30), (1, 0x10)]);
        assert_eq!(used, 0x60);
        assert_eq!(e.mem.u32(walked + 4), 1);
        let out = e.mem.u32(e.mem.u32(walked));
        assert_ne!(out, first);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0[1..], [out + 0x10, out + 0x20]);
        assert_eq!(seen[1].0[1..], [out + 0x30, out + 0x50]);
        let words: [u32; 8] = std::array::from_fn(|i| 0x1000 + i as u32);
        assert_eq!(seen[0].1, words);
        assert_eq!(seen[1].1, words);
        // The entry bytes were copied.
        assert_eq!(e.mem.u8(out + 0x11), 1);
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![vec![allocator, first, 0x200]]
        );
    }

    #[test]
    fn visit_all_agents_moves_on_through_the_input_sectors() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_visitor(&mut e, None);
        let input = visit_input(&mut e);
        let (first, _) = sector(&mut e, &[(2, 1, 0x20), (2, 1, 0x20)]);
        let (second, _) = sector(&mut e, &[(5, 1, 0x30), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first, second]);
        e.call(0x00da_44d0, &args![walked, input, CALLBACK_DOUBLE]);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x20), (2, 0x20), (5, 0x30), (1, 0x10)]);
        assert_eq!(used, 0x80);
        assert_eq!(e.mem.u32(walked + 4), 1);
        // Each input sector is given back, the first one a sector late.
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![
                vec![allocator, first, 0x200],
                vec![allocator, second, 0x200]
            ]
        );
    }

    #[test]
    fn visit_all_agents_splits_a_sector_that_grew_through_the_scratch_area() {
        let mut e = logged();
        install_allocator(&mut e);
        let seen = install_visitor(&mut e, Some(0x40));
        let input = visit_input(&mut e);
        let entries = [(2u8, 1u8, 0x30u8); 9];
        let (first, _) = sector(&mut e, &entries);
        let walked = track(&mut e, &[first]);
        e.call(0x00da_44d0, &args![walked, input, CALLBACK_DOUBLE]);
        assert_eq!(e.mem.u32(walked + 4), 2);
        let (first_entries, first_used) = stored_entries(&e, walked, 0);
        let (second_entries, second_used) = stored_entries(&e, walked, 1);
        assert_eq!(first_entries, vec![(2, 0x40); 7]);
        assert_eq!(first_used, 0x1c0);
        assert_eq!(second_entries, vec![(2, 0x40); 2]);
        assert_eq!(second_used, 0x80);
        assert_eq!(seen.borrow().len(), 9);
        // The eighth entry was visited in the scratch area, not in a sector.
        let out = e.mem.u32(e.mem.u32(walked));
        let calls_seen = seen.borrow();
        assert!(calls_seen[7].0[1] < out || calls_seen[7].0[1] > out + 0x200);
    }

    #[test]
    fn visit_all_agents_makes_room_in_the_track_for_the_sectors_it_adds() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_visitor(&mut e, Some(0x70));
        let input = visit_input(&mut e);
        let (first, _) = sector(&mut e, &[(2u8, 1u8, 0x30u8); 10]);
        let (second, _) = sector(&mut e, &[(1, 0, 0x10)]);
        let walked = track(&mut e, &[first, second]);
        e.call(0x00da_44d0, &args![walked, input, CALLBACK_DOUBLE]);
        assert_eq!(e.mem.u32(walked + 4), 3);
        let (one, one_used) = stored_entries(&e, walked, 0);
        let (two, two_used) = stored_entries(&e, walked, 1);
        let (three, three_used) = stored_entries(&e, walked, 2);
        assert_eq!(one, vec![(2, 0x70); 4]);
        assert_eq!(one_used, 0x1c0);
        assert_eq!(two, vec![(2, 0x70); 4]);
        assert_eq!(two_used, 0x1c0);
        assert_eq!(three, vec![(2, 0x70), (2, 0x70), (1, 0x10)]);
        assert_eq!(three_used, 0xf0);
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![
                vec![allocator, first, 0x200],
                vec![allocator, second, 0x200]
            ]
        );
    }

    #[test]
    fn visit_all_agents_grows_the_track_array_when_it_is_full() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reserve(&mut e);
        install_visitor(&mut e, Some(0x70));
        let input = visit_input(&mut e);
        let (first, _) = sector(&mut e, &[(2u8, 1u8, 0x30u8); 10]);
        let (second, _) = sector(&mut e, &[(1, 0, 0x10)]);
        let walked = track(&mut e, &[first, second]);
        // No spare capacity: the capacity word equals the size.
        e.mem.set_u32(walked + 8, 2);
        e.call(0x00da_44d0, &args![walked, input, CALLBACK_DOUBLE]);
        assert_eq!(e.mem.u32(walked + 4), 3);
        let reserves = calls(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][2..], [4, 4]);
        assert_eq!(stored_entries(&e, walked, 2).1, 0xf0);
    }

    #[test]
    fn update_shape_collection_filter_visits_with_the_update_callback() {
        let mut e = logged();
        install_allocator(&mut e);
        let seen: Visits = Rc::default();
        // The callback of the exe address is replaced by the recording double.
        e.register_double(UPDATE_SHAPE_CALLBACK, {
            let log = Rc::clone(&seen);
            move |e, a| {
                log.borrow_mut().push((a.to_vec(), [0; 8]));
                Ret {
                    eax: a[1] + u32::from(e.mem.u8(a[1] + 3)),
                    ..Ret::default()
                }
            }
        });
        let input = visit_input(&mut e);
        let (first, _) = sector(&mut e, &[(2, 1, 0x20), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        e.call(0x00da_48e0, &args![walked, input]);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x20), (1, 0x10)]);
        assert_eq!(used, 0x30);
        let out = e.mem.u32(e.mem.u32(walked));
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1..], [out + 0x10, out + 0x20]);
    }
}
