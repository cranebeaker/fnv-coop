//! `Havok SDK/hkpbvtreestreamagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpBvTreeStreamAgent` (Xbox PDB; vtable `010ce838`, 0x40 bytes) is the collision agent of
//! a body with a bounding-volume tree and another shape. Its base is the Xbox PDB's
//! `hkpCollisionAgent`: size `+4` (u16), reference count `+6`, contact manager `+8`. From
//! there it keeps the first word of the input it was made with (`+0xC`, the collision
//! dispatcher), a cached box (two 16-byte vectors, `+0x10` and `+0x20`) and, at `+0x30`, an
//! `hkArray` (data `+0x30`, size `+0x34`, capacity and flags `+0x38`) of the agent stream
//! the `hkAgent1nMachine` functions work on. The static functions and the `hkpBvTreeAgent`
//! functions it forwards to are other units'.
//!
//! The unit also holds the symmetric wrappers of the Xbox PDB's
//! `hkpSymmetricAgentLinearCast<>` templates for this agent and for the shape collection
//! agent (the pair with the roles of the two bodies exchanged).
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, cross, mul, rows_times, sub};
use crate::units::havok_sdk::hkpbvagent::{
    flip_toi_words, profile_start_record, symmetric_linear_cast, symmetric_process_collision,
    with_flip_body_collector, with_flip_collector, LABEL_END,
};
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, maxps, minps, profile_record, store_vec, zip_with, Vec4, SIGN_MASK,
};

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Vtables: the agent (`.?AVhkpBvTreeStreamAgent@@`), the base the destructor ends with and
/// the symmetric agent the contact-manager-less constructor turns it into.
const VTABLE: u32 = 0x010c_e838;
const BASE_VTABLE: u32 = 0x0102_e388;
const SYMMETRIC_VTABLE: u32 = 0x010c_e874;
/// The label `"SectorPtrs"` the content statistics report the array under.
const LABEL_SECTOR_PTRS: u32 = 0x010c_e828;
/// The capacity word of an empty array that owns nothing (the flag bit set, capacity 0).
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// The static (cdecl) queries of the bounding-volume tree pair: `getPenetrations` (4 words),
/// `getClosestPoints` (4) and `linearCast` (5).
const STATIC_GET_PENETRATIONS: u32 = 0x00d3_12f0;
const STATIC_GET_CLOSEST_POINTS: u32 = 0x00d3_1110;
const STATIC_LINEAR_CAST: u32 = 0x00d3_0f50;
/// The functions of the agent stream (`hkAgent1nMachine`) the agent's array is handed to
/// (cdecl): cleanup `(array, dispatcher, contact manager, param)`, invalidateTim `(array,
/// param)`, warpTime `(array, old time, new time, third)`, updateShapeCollectionFilter
/// `(array, parameter block)`, the allocated byte count `(data, bytes)`, calcStatistics
/// `(array, a word of the collector, collector)` and the array constructor `(array)`.
const STREAM_CLEANUP: u32 = 0x00da_42b0;
const STREAM_INVALIDATE_TIM: u32 = 0x00da_3510;
const STREAM_WARP_TIME: u32 = 0x00da_35f0;
const STREAM_UPDATE_FILTER: u32 = 0x00da_48e0;
const ALLOCATED_BYTES: u32 = 0x00d0_6f80;
const STREAM_CALC_STATISTICS: u32 = 0x00d9_7250;
const STREAM_INIT: u32 = 0x00da_4430;
/// The constructor of the shape collection pair's agent (`this` = the new block, the four
/// words of a create function).
const SHAPE_COLLECTION_CONSTRUCTOR: u32 = 0x00d4_40f0;
/// `hkpCollisionDispatcher` agent registration (`this` = dispatcher, `RET 0xC`): `(agent
/// function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The shape collection agent's functions the symmetric wrappers forward to.
const SHAPE_COLLECTION_UPDATE_FILTER: u32 = 0x00d4_3f10;
const SHAPE_COLLECTION_GET_PENETRATIONS: u32 = 0x00d4_3cd0;
const SHAPE_COLLECTION_GET_CLOSEST_POINTS: u32 = 0x00d4_3880;
const SHAPE_COLLECTION_PROCESS_COLLISION: u32 = 0x00d4_37a0;
const SHAPE_COLLECTION_LINEAR_CAST: u32 = 0x00d4_3aa0;
/// `hkpBvTreeStreamAgent::processCollision` (the function of this unit below).
const PROCESS_COLLISION: u32 = 0x00d2_df90;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, `this` = result, `(a, b)`) and
/// `hkTransform::setInverse` (`00cb21e0`, `this` = result, one word).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
/// `hkAgent1nMachine_Process` (`00da4aa0`, cdecl, five words), the unsigned word sort
/// (`00d2da70`, cdecl, four words) and the out-of-memory state setter (`00d88040`, cdecl, one
/// word).
const AGENT_1N_PROCESS: u32 = 0x00da_4aa0;
const QUICK_SORT: u32 = 0x00d2_da70;
const SET_OUT_OF_MEMORY_STATE: u32 = 0x00d8_8040;
/// The profiling labels of `processCollision`: `"LtBvTree3"`, `"StQueryTree"` and
/// `"StNarrow"`.
const LABEL_LT_BV_TREE3: u32 = 0x010c_e7ec;
const LABEL_ST_QUERY_TREE: u32 = 0x010c_e7e0;
const LABEL_ST_NARROW: u32 = 0x010c_e7d4;
/// The `double` 0.5 and the two `float` constants (0.4 and -2.0) that grow the cached box.
const HALF_F64: u32 = 0x0101_1588;
const GROWTH_FACTOR: u32 = 0x0102_31a8;
const SWEEP_FACTOR: u32 = 0x0104_a494;

/// The allocator of the memory router (`[router + 0x10]`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Allocates `size` bytes through the allocator's slot 4 and sets the block's size word
/// (`u16` at `+4`).
fn allocate_agent(e: &mut Engine, size: u32) -> u32 {
    let allocator = allocator(e);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

// Translated from 00d2dd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::getPenetrations` (Xbox PDB): hands the two bodies, the input and
/// the collector on to the static query `00d312f0` (cdecl); `this` is not used.
pub fn hkp_bv_tree_stream_agent_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    e.call(
        STATIC_GET_PENETRATIONS,
        &args![body_a, body_b, input, collector],
    );
}

// Translated from 00d2dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::getClosestPoints` (Xbox PDB): hands the two bodies, the input and
/// the collector on to the static query `00d31110` (cdecl); `this` is not used.
pub fn hkp_bv_tree_stream_agent_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    e.call(
        STATIC_GET_CLOSEST_POINTS,
        &args![body_a, body_b, input, collector],
    );
}

// Translated from 00d2dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::linearCast` (Xbox PDB): hands the two bodies, the input and the two
/// collectors on to the static cast `00d30f50` (cdecl); `this` is not used.
pub fn hkp_bv_tree_stream_agent_linear_cast(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    e.call(
        STATIC_LINEAR_CAST,
        &args![body_a, body_b, input, cast_collector, start_collector],
    );
}

// Translated from 00d2dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::cleanup` (Xbox PDB): cleans the agent stream up
/// (`00da42b0`: the array at `+0x30`, the dispatcher word `+0xC`, the contact manager `+8`
/// and `param`), then deletes the agent through its vtable's first slot with the flag 1.
pub fn hkp_bv_tree_stream_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let this = this.addr();
    let dispatcher = e.mem.u32(this + 0xc);
    let contact_mgr = e.mem.u32(this + 8);
    e.call(
        STREAM_CLEANUP,
        &args![this + 0x30, dispatcher, contact_mgr, param],
    );
    e.vcall(this, 0, &args![1u32]);
}

// Translated from 00d2ddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::invalidateTim` (Xbox PDB): passes the agent stream (the array at
/// `+0x30`) and the word on to `00da3510`.
pub fn hkp_bv_tree_stream_agent_invalidate_tim(e: &mut Engine, this: Ptr, param: u32) {
    e.call(STREAM_INVALIDATE_TIM, &args![this.addr() + 0x30, param]);
}

// Translated from 00d2ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::warpTime` (Xbox PDB): passes the agent stream, the two times
/// (floats, through `FLD`/`FSTP`) and the third word on to `00da35f0`.
pub fn hkp_bv_tree_stream_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    old_time: f32,
    new_time: f32,
    third: u32,
) {
    let old_time = x87_quiet(old_time);
    let new_time = x87_quiet(new_time);
    e.call(
        STREAM_WARP_TIME,
        &args![this.addr() + 0x30, old_time, new_time, third],
    );
}

// Translated from 00d2de20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::updateShapeCollectionFilter` (Xbox PDB): clears the cached box
/// (`+0x10` to `+0x2F`), asks the object the second word points to (`[body_b]`) for its slot
/// `0x10` and hands the agent stream to `00da48e0` together with a 7-word block: the first
/// body, the second, a word the exe leaves unwritten (zero here), the answer of that slot, the
/// third word, the contact manager (`+8`) and the fourth word.
pub fn hkp_bv_tree_stream_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    let this = this.addr();
    for word in 0..8 {
        e.mem.set_u32(this + 0x10 + 4 * word, 0);
    }
    let object = e.mem.u32(body_b.addr());
    let answer = e.vcall(object, 0x10, &args![]).u32();
    let contact_mgr = e.mem.u32(this + 8);
    e.with_stack(0x1c, |e, block| {
        let block = block.addr();
        for (index, word) in [
            body_a.addr(),
            body_b.addr(),
            0,
            answer,
            third,
            contact_mgr,
            fourth,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(block + 4 * index as u32, word);
        }
        e.call(STREAM_UPDATE_FILTER, &args![this + 0x30, block]);
    });
}

// Translated from 00d2de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::calcContentStatistics` (Xbox PDB): when the array's capacity word
/// (`+0x38`) is not negative, or the collector's byte `+0x10` is set, reports the array to the
/// collector (slot `0x14`: 3, `"SectorPtrs"`, the data, the used bytes (size * 4) and the
/// allocated bytes `00d06f80` answers for the capacity (low 30 bits) * 4). Then lets
/// `00d97250` add the agent stream's statistics (the array, the collector's word `+8`, the
/// collector). The second word is not used.
pub fn hkp_bv_tree_stream_agent_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let array = this.addr() + 0x30;
    let collector = collector.addr();
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        let allocated = e.call(ALLOCATED_BYTES, &args![data, bytes]).u32();
        let used = e.mem.u32(array + 4).wrapping_mul(4);
        e.vcall(
            collector,
            0x14,
            &args![3u32, LABEL_SECTOR_PTRS, data, used, allocated],
        );
    }
    let word = e.mem.u32(collector + 8);
    e.call(STREAM_CALC_STATISTICS, &args![array, word, collector]);
}

// Translated from 00d2def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent` constructor (the map gives this body no name): sets the vtable, a
/// reference count of 1 (`+6`), the contact manager (`+8`), the dispatcher word (`+0xC`, the
/// input's first word), an empty array (`+0x30`: no data, size 0, capacity word
/// `0x80000000`) and a cleared cached box (`+0x10` to `+0x2F`), then runs `00da4430` on the
/// array. The first two words are not used. Returns `this`.
pub fn fn_00d2def0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    input: Ptr,
    contact_mgr: u32,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 8, contact_mgr);
    e.mem.set_u32(at, VTABLE);
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x34, 0);
    e.mem.set_u32(at + 0x38, EMPTY_CAPACITY);
    let dispatcher = e.mem.u32(input.addr());
    e.mem.set_u32(at + 0xc, dispatcher);
    for word in 0..8 {
        e.mem.set_u32(at + 0x10 + 4 * word, 0);
    }
    e.call(STREAM_INIT, &args![at + 0x30]);
    this
}

// Translated from 00d2e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The create function of the tree-stream pairs (the map gives it no name; cdecl). With a
/// contact manager it asks the object at the input's `+0x10` (slot `0xC`: the two bodies,
/// the input) what to make: 1 makes a shape collection agent (0x38 bytes, from the memory
/// router's allocator, size word `+4`, constructor `00d440f0`), 0 or 2 an `hkpBvTreeStreamAgent`
/// (0x40 bytes, `00d2def0`), anything else nothing (0). Without a contact manager it makes the
/// 0x40-byte agent (with the bodies in the other order, which the constructor does not use)
/// and turns it into the symmetric agent (vtable `010ce874`). Returns the new agent.
pub fn fn_00d2e5a0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    if contact_mgr == 0 {
        let agent = allocate_agent(e, 0x40);
        fn_00d2def0(e, Ptr::new(agent), body_b.addr(), body_a.addr(), input, 0);
        e.mem.set_u32(agent, SYMMETRIC_VTABLE);
        return agent;
    }
    let mediator = e.mem.u32(input.addr() + 0x10);
    let kind = e.vcall(mediator, 0xc, &args![body_a, body_b, input]).u32();
    match kind {
        1 => {
            let agent = allocate_agent(e, 0x38);
            e.call(
                SHAPE_COLLECTION_CONSTRUCTOR,
                &args![agent, body_a, body_b, input, contact_mgr],
            )
            .u32()
        }
        0 | 2 => {
            let agent = allocate_agent(e, 0x40);
            fn_00d2def0(
                e,
                Ptr::new(agent),
                body_a.addr(),
                body_b.addr(),
                input,
                contact_mgr,
            )
            .addr()
        }
        _ => 0,
    }
}

/// The registration of a tree-stream pair with the dispatcher (the three `00d2e6d0`-like
/// functions): the symmetric entry (`create_symmetric`, the two flip functions `00d2e970` and
/// `00d2e9b0` and the symmetric cast `00d2e9f0`; both flag bytes 1) for the types (8,
/// `other_type`), then the direct entry (`create_direct` and the static queries `00d312f0`,
/// `00d31110`, `00d30f50`; flag bytes 0 and 1) for (`other_type`, 8), through `00cfbbe0` (`this`
/// = the dispatcher). Both entries are built in the same 20-byte block on the stack; its bytes
/// `+0x12` and `+0x13` are not set by the exe (the callee copies the word they are in); here
/// they are zero.
fn register_pair(
    e: &mut Engine,
    dispatcher: Ptr,
    other_type: u32,
    create_symmetric: u32,
    create_direct: u32,
) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (words, flag_a, flag_b, type_a, type_b) in [
            (
                [create_symmetric, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_e9f0],
                1u8,
                1u8,
                8u32,
                other_type,
            ),
            (
                [
                    create_direct,
                    STATIC_GET_PENETRATIONS,
                    STATIC_GET_CLOSEST_POINTS,
                    STATIC_LINEAR_CAST,
                ],
                0,
                1,
                other_type,
                8,
            ),
        ] {
            for (index, word) in words.into_iter().enumerate() {
                e.mem.set_u32(entry + 4 * index as u32, word);
            }
            e.mem.set_u8(entry + 0x10, flag_a);
            e.mem.set_u8(entry + 0x11, flag_b);
            e.call(REGISTER_AGENT, &args![dispatcher, entry, type_a, type_b]);
        }
    });
}

// Translated from 00d2e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the tree-stream agent with a dispatcher for the shape types 8 and `0x12` (the
/// map gives the function no name): the symmetric entry (`createFunc` `00d2e680`) and the
/// direct entry (`createFunc` `00d2df50`); see `register_pair`.
pub fn fn_00d2e6d0(e: &mut Engine, dispatcher: Ptr) {
    register_pair(e, dispatcher, 0x12, 0x00d2_e680, 0x00d2_df50);
}

// Translated from 00d2e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d2e6d0`, for the shape types 8 and `0x18` (the map gives the function no name).
pub fn fn_00d2e760(e: &mut Engine, dispatcher: Ptr) {
    register_pair(e, dispatcher, 0x18, 0x00d2_e680, 0x00d2_df50);
}

// Translated from 00d2e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d2e6d0`, for the shape types 8 and `0x16` (the map gives the function no name), with
/// the `createFunc`s `00d2e7f0` (symmetric) and `00d2e5a0` (direct).
pub fn fn_00d2e8e0(e: &mut Engine, dispatcher: Ptr) {
    register_pair(e, dispatcher, 0x16, 0x00d2_e7f0, 0x00d2_e5a0);
}

// Translated from 00d2e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the tree-stream pair (the map gives it no name;
/// cdecl): runs `00d312f0` with the two bodies swapped and, in place of the caller's collector
/// word `output`, a flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's word
/// at `+8`) built on the stack.
pub fn fn_00d2e970(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        e.call(STATIC_GET_PENETRATIONS, &args![body_b, body_a, input, flip]);
    });
}

// Translated from 00d2e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the tree-stream pair (the map gives it no name;
/// cdecl): runs `00d31110` with the two bodies swapped and a flip collector (vtable
/// `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word `output` at
/// `+8`).
pub fn fn_00d2e9b0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        e.call(
            STATIC_GET_CLOSEST_POINTS,
            &args![body_b, body_a, input, flip],
        );
    });
}

// Translated from 00d2e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static linear cast of the tree-stream pair (the map gives it no name; cdecl):
/// runs `00d30f50` with the two bodies swapped, a copy of the input whose path vector
/// (`input + 0x50`) is negated, and flip-cast collectors (vtable `010ce80c`) wrapping the
/// caller's two (`start_collector` null stays null).
pub fn fn_00d2e9f0(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    symmetric_linear_cast(
        e,
        input,
        cast_collector,
        start_collector,
        |e, copy, first, second| {
            e.call(
                STATIC_LINEAR_CAST,
                &args![body_b, body_a, copy, first, second],
            );
        },
    );
}

// Translated from 00d2eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent` scalar deleting destructor (Xbox PDB): sets the vtable, empties the
/// array at `+0x30` (size 0; when the capacity word is not negative the data goes back to the
/// memory router's allocator, slot `0x10`, with the capacity (low 30 bits) * 4 bytes; data
/// null, capacity word `0x80000000`), sets the base vtable (`0102e388`) and, when bit 0 of
/// `flags` is set, returns the agent's block (size word `+4`) to the allocator (slot 8).
/// Returns `this`.
pub fn hkp_bv_tree_stream_agent_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, VTABLE);
    let allocator_object = allocator(e);
    let capacity = e.mem.u32(at + 0x38);
    e.mem.set_u32(at + 0x34, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(at + 0x30);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator_object, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x38, EMPTY_CAPACITY);
    e.mem.set_u32(at, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator_object = allocator(e);
        let size = u32::from(e.mem.u16(at + 4));
        e.vcall(allocator_object, 8, &args![this, size]);
    }
    this
}

// Translated from 00d2eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns the time-of-impact words that start at `this` around (the map gives it no name;
/// `ECX` holds the address, as in the output's `+0x3040` block): see `flip_toi_words`.
pub fn fn_00d2eb50(e: &mut Engine, this: Ptr) {
    flip_toi_words(e, this.addr());
}

// Translated from 00d2ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric `getPenetrations` of the tree-stream pair (the map gives it no name): runs
/// `hkpBvTreeStreamAgent::getPenetrations` (`00d2dd30`) with the bodies swapped and a
/// flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's word at `+8`) built
/// on the stack. `this` is not used.
pub fn fn_00d2ebb0(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_bv_tree_stream_agent_get_penetrations(
            e,
            _unused_this,
            body_b,
            body_a,
            input,
            Ptr::new(flip),
        );
    });
}

// Translated from 00d2ebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric `getClosestPoints` of the tree-stream pair (the map gives it no name): runs
/// `hkpBvTreeStreamAgent::getClosestPoints` (`00d2dd50`) with the bodies swapped and a flip
/// collector (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's
/// word at `+8`). `this` is not used.
pub fn fn_00d2ebf0(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_bv_tree_stream_agent_get_closest_points(
            e,
            _unused_this,
            body_b,
            body_a,
            input,
            Ptr::new(flip),
        );
    });
}

// Translated from 00d2ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvTreeStreamAgent>::updateShapeCollectionFilter` (Xbox
/// PDB): calls `hkpBvTreeStreamAgent::updateShapeCollectionFilter` with the two bodies
/// swapped.
pub fn hkp_symmetric_agent_linear_cast_bv_tree_stream_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    hkp_bv_tree_stream_agent_update_shape_collection_filter(e, this, body_b, body_a, third, fourth);
}

// Translated from 00d2ec50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvTreeStreamAgent>::processCollision` (Xbox PDB): runs
/// `hkpBvTreeStreamAgent::processCollision` (`00d2df90`) with the bodies swapped, then turns
/// what it appended to the output around, as `00d46220` does for the bounding-volume agent.
pub fn hkp_symmetric_agent_linear_cast_bv_tree_stream_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        e.call(
            PROCESS_COLLISION,
            &args![this, body_b, body_a, input, output],
        );
    });
}

// Translated from 00d2ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpShapeCollectionAgent>::updateShapeCollectionFilter` (Xbox
/// PDB): calls `hkpShapeCollectionAgent::updateShapeCollectionFilter` (`00d43f10`) with the
/// two bodies swapped.
pub fn hkp_symmetric_agent_linear_cast_shape_collection_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    e.call(
        SHAPE_COLLECTION_UPDATE_FILTER,
        &args![this, body_b, body_a, third, fourth],
    );
}

// Translated from 00d2ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpShapeCollectionAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpShapeCollectionAgent::getPenetrations` (`00d43cd0`) with the bodies swapped and a
/// flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_shape_collection_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        e.call(
            SHAPE_COLLECTION_GET_PENETRATIONS,
            &args![this, body_b, body_a, input, flip],
        );
    });
}

// Translated from 00d2edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpShapeCollectionAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpShapeCollectionAgent::getClosestPoints` (`00d43880`) with the bodies swapped and a
/// flip collector (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the
/// caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_shape_collection_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        e.call(
            SHAPE_COLLECTION_GET_CLOSEST_POINTS,
            &args![this, body_b, body_a, input, flip],
        );
    });
}

// Translated from 00d2ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpShapeCollectionAgent>::processCollision` (Xbox PDB): runs
/// `hkpShapeCollectionAgent::processCollision` (`00d437a0`) with the bodies swapped, then
/// turns what it appended to the output around, as `00d46220` does for the bounding-volume
/// agent.
pub fn hkp_symmetric_agent_linear_cast_shape_collection_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        e.call(
            SHAPE_COLLECTION_PROCESS_COLLISION,
            &args![this, body_b, body_a, input, output],
        );
    });
}

// Translated from 00d2ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric linear cast of the tree-stream pair (the map gives it no name): runs
/// `hkpBvTreeStreamAgent::linearCast` (`00d2dd70`) with the bodies swapped, a copy of the
/// input whose path vector (`input + 0x50`) is negated, and flip-cast collectors (vtable
/// `010ce80c`) wrapping the caller's two (`start_collector` null stays null).
pub fn fn_00d2ef30(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    symmetric_linear_cast(
        e,
        input,
        cast_collector,
        start_collector,
        |e, copy, first, second| {
            hkp_bv_tree_stream_agent_linear_cast(
                e,
                this,
                body_b,
                body_a,
                Ptr::new(copy),
                first,
                second,
            );
        },
    );
}

// Translated from 00d2f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpShapeCollectionAgent>::linearCast` (Xbox PDB): runs
/// `hkpShapeCollectionAgent::linearCast` (`00d43aa0`) with the bodies swapped, a copy of the
/// input whose path vector is negated and flip-cast collectors wrapping the caller's two
/// (as `00d2ef30` does).
pub fn hkp_symmetric_agent_shape_collection_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    symmetric_linear_cast(
        e,
        input,
        cast_collector,
        start_collector,
        |e, copy, first, second| {
            e.call(
                SHAPE_COLLECTION_LINEAR_CAST,
                &args![this, body_b, body_a, copy, first, second],
            );
        },
    );
}

/// The step of `processCollision` (`00d2df90`) between the tree query and the 1:N machine:
/// asks the memory router's allocator (slot `0x24`) for `((size / 4) - agent.size + 1) << 9`
/// where `size` is the size of the stack `array`; on success sorts the array as unsigned
/// words (`00d2da70`: data, first and last index, and a fourth word that is stack residue the
/// sort never reads, passed as 0) when it holds more than one entry, and returns true.
fn hit_storage_ready(e: &mut Engine, agent: u32, array: u32) -> bool {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let hits = e.mem.i32(array + 4);
    let wanted = (hits / 4)
        .wrapping_sub(e.mem.i32(agent + 0x34))
        .wrapping_add(1)
        << 9;
    if !e.vcall(allocator, 0x24, &args![wanted]).bool() {
        return false;
    }
    let hits = e.mem.i32(array + 4);
    if hits > 1 {
        let data = e.mem.u32(array);
        e.call(QUICK_SORT, &args![data, 0i32, hits - 1, 0u32]);
    }
    true
}

// Translated from 00d2df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeStreamAgent::processCollision` (Xbox PDB): `this` is the agent, the bodies are
/// `hkpCdBody`s whose motion states (`+8`) carry the transforms, `input` the
/// `hkpProcessCollisionInput` and `output` the result block of the agent stream. Between
/// profiling records (`"LtBvTree3"` with `"StQueryTree"` in one 16-byte record, `"StNarrow"`
/// and the shared end label in 12-byte ones) it:
///
/// - builds the relative transform of the two motion states (`00cb22f0`) and its inverse
///   (`00cb21e0`), and the displacement `W` of body A relative to body B over the time step
///   `input[0x58]`, carried into B's frame by B's rotation rows (the lanes follow the SSE
///   code; `W[3]` is stored but never used);
/// - asks body A's shape (slot `0x1C`: the inverse transform, a tolerance, a 32-byte box) for
///   its box in B's frame; when `[[input + 0x60] + 0x10]` is not 0 the tolerance and a
///   widening of the box by the swept radius terms are added, and, when B's field `+0x9C` is
///   positive, a velocity term from B's `+0x80`/`+0x90` is added to the displacement;
/// - compares the box with the agent's cached box (`this + 0x10` and `+0x20`, three lanes):
///   when the cached box holds it the tree query is skipped; otherwise the box is grown (by
///   0.4 of its extents and -2 times the displacement, constants read from the exe's data) and
///   written to the cached box, and body B's shape (slot `0x30`: the box, a stack array that
///   starts with one entry, `0xFFFFFFFF`, and room for 0x80) fills the array;
/// - runs the 1:N machine (`00da4aa0`: the agent's array at `this + 0x30`, the query block
///   `{body A, body B, 0, input, this[8]}`, the value of B's shape slot `0x10`, then 0 when
///   the cached box held the box or the array data, and `output`). Before that, when the
///   array was queried, it asks the memory router's allocator (slot `0x24`) for
///   `((size / 4) - this.size + 1) << 9`; on failure it sets the out-of-memory state
///   (`00d88040`) and ends, otherwise it sorts the array as unsigned words (`00d2da70`) when
///   it holds more than one entry;
/// - finally empties the stack array and frees its storage (allocator slot `0x10`) when it
///   owns heap memory.
///
/// The exe's `JZ` after `ADD EAX, 0x10` on the agent address is taken only for an agent at
/// `0xFFFFFFF0`, which no object can be; it is not modelled. The fourth word of the sort call
/// is the stack residue of a byte store; the sort never reads it, so 0 is passed.
pub fn hkp_bv_tree_stream_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_start_record(e, LABEL_LT_BV_TREE3, LABEL_ST_QUERY_TREE);
    let agent = this.addr();
    let input_address = input.addr();
    let state_a = e.mem.u32(body_a.addr() + 8);
    let state_b = e.mem.u32(body_b.addr() + 8);
    let shape_a = e.mem.u32(body_a.addr());
    let shape_b = e.mem.u32(body_b.addr());
    let half: f64 = e.global(HALF_F64);
    // The exe's frame: the box at +0x20 (minimum) and +0x30 (maximum), the displacement in
    // B's frame at +0x40, the query block at +0x50, the relative transform at +0x70, the
    // displacement W at +0xC0, its inverse at +0xD0 and the stack array (data, size,
    // capacity) at +0x110 with its 0x80 entries at +0x11C.
    e.with_stack(0x320, |e, frame| {
        let frame = frame.addr();
        let bounds = frame + 0x20;
        let query = frame + 0x50;
        let transform = frame + 0x70;
        let inverse = frame + 0xd0;
        let array = frame + 0x110;
        e.mem.set_u32(query, body_a.addr());
        e.mem.set_u32(query + 4, body_b.addr());
        e.mem.set_u32(query + 8, 0);
        e.mem.set_u32(query + 0xc, input_address);
        e.mem.set_u32(query + 0x10, e.mem.u32(agent + 8));

        let time = x87_quiet(e.mem.f32(input_address + 0x58));
        let a_scale = (f64::from(x87_quiet(e.mem.f32(state_a + 0x5c))) * f64::from(time)) as f32;
        let b_scale = (f64::from(time) * f64::from(e.mem.f32(state_b + 0x5c))) as f32;
        let a_offset = sub(load_vec(e, state_a + 0x40), load_vec(e, state_a + 0x50));
        let b_offset = sub(load_vec(e, state_b + 0x50), load_vec(e, state_b + 0x40));
        let mut moved = add(mul(b_offset, [b_scale; 4]), mul(a_offset, [a_scale; 4]));
        moved[3] = (f64::from(x87_quiet(e.mem.f32(state_b + 0x9c)))
            * f64::from(b_scale)
            * f64::from(e.mem.f32(state_b + 0xa0))
            + f64::from(x87_quiet(e.mem.f32(state_a + 0x9c)))
                * f64::from(a_scale)
                * f64::from(e.mem.f32(state_a + 0xa0))) as f32;
        store_vec(e, frame + 0xc0, moved);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, state_a, state_b],
        );
        e.mem.set_u32(array, frame + 0x11c);
        e.mem.set_u32(array + 8, 0x8000_0080);
        e.mem.set_u32(array + 0xc, 0xffff_ffff);
        e.mem.set_u32(array + 4, 1);
        e.call(TRANSFORM_SET_INVERSE, &args![inverse, transform]);

        let rows = [
            load_vec(e, state_b),
            load_vec(e, state_b + 0x10),
            load_vec(e, state_b + 0x20),
        ];
        let carried = rows_times(moved, rows[0], rows[1], rows[2]);
        store_vec(e, frame + 0x40, carried);
        let sweeping = e.mem.u32(e.mem.u32(input_address + 0x60) + 0x10) != 0;
        let (lower, upper, extents, motion) = if sweeping {
            let radius = x87_quiet(e.mem.f32(state_b + 0x9c));
            let mass = e.mem.f32(state_b + 0xa0);
            let own_term = (f64::from(radius) * f64::from(radius) * f64::from(mass)) as f32;
            let other_radius = x87_quiet(e.mem.f32(state_a + 0x9c));
            let other_mass = e.mem.f32(state_a + 0xa0);
            let combined =
                ((f64::from(other_radius) + f64::from(radius)) * f64::from(other_mass)) as f32;
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let tolerance = ((f64::from(combined) + f64::from(own_term)) + margin) as f32;
            e.vcall(shape_a, 0x1c, &args![inverse, tolerance, bounds]);
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let growth =
                ((margin + f64::from(e.mem.f32(state_a + 0xa0))) + f64::from(own_term)) as f32;
            let difference = sub(load_vec(e, state_a + 0x50), load_vec(e, state_b + 0x30));
            let local = rows_times(difference, rows[0], rows[1], rows[2]);
            let box_lower = load_vec(e, bounds);
            let box_upper = load_vec(e, bounds + 0x10);
            let spread = [growth; 4];
            let widened_lower = zip_with(box_lower, sub(local, spread), maxps);
            let widened_upper = zip_with(box_upper, add(local, spread), minps);
            store_vec(e, bounds, widened_lower);
            store_vec(e, bounds + 0x10, widened_upper);
            let box_extents = sub(widened_upper, widened_lower);
            let radius_positive = 0.0 < f64::from(radius);
            let total = if radius_positive {
                let scale = (f64::from(x87_quiet(e.mem.f32(state_b + 0x5c)))
                    * f64::from(e.mem.f32(input_address + 0x58)))
                    as f32;
                let offset = sub(local, load_vec(e, state_b + 0x80));
                let spin = mul(cross(offset, load_vec(e, state_b + 0x90)), [scale; 4]);
                add(spin, carried)
            } else {
                carried
            };
            let zero = [0.0; 4];
            let below = zip_with(zero, total, minps);
            let above = zip_with(zero, total, maxps);
            let lower = add(below, widened_lower);
            let upper = add(above, widened_upper);
            store_vec(e, bounds, lower);
            store_vec(e, bounds + 0x10, upper);
            (lower, upper, box_extents, total)
        } else {
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            e.vcall(shape_a, 0x1c, &args![inverse, margin as f32, bounds]);
            let lower = load_vec(e, bounds);
            let upper = load_vec(e, bounds + 0x10);
            (lower, upper, sub(upper, lower), carried)
        };

        let cached = agent + 0x10;
        let cached_lower = load_vec(e, cached);
        let cached_upper = load_vec(e, cached + 0x10);
        let inside = (0..3)
            .all(|lane| cached_lower[lane] <= lower[lane] && upper[lane] <= cached_upper[lane]);
        if !inside {
            let margin = (f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half) as f32;
            let growth_factor = x87_quiet(e.global::<f32>(GROWTH_FACTOR));
            let sweep_factor = x87_quiet(e.global::<f32>(SWEEP_FACTOR));
            let margin_vector = [margin; 4];
            let lowered = sub(lower, margin_vector);
            let raised = add(margin_vector, upper);
            let zero = [0.0; 4];
            let forward = mul([sweep_factor; 4], zip_with(zero, motion, maxps));
            let grown = mul([growth_factor; 4], extents);
            let sign = load_bits(e, SIGN_MASK);
            let negated: Vec4 =
                std::array::from_fn(|lane| f32::from_bits(grown[lane].to_bits() ^ sign[lane]));
            let backward = mul([sweep_factor; 4], zip_with(zero, motion, minps));
            let new_lower = add(zip_with(forward, negated, maxps), lowered);
            let new_upper = add(zip_with(backward, grown, minps), raised);
            store_vec(e, bounds, new_lower);
            store_vec(e, bounds + 0x10, new_upper);
            store_vec(e, cached, new_lower);
            store_vec(e, cached + 0x10, new_upper);
            e.vcall(shape_b, 0x30, &args![bounds, array]);
        }
        profile_record(e, LABEL_ST_NARROW);
        if inside || hit_storage_ready(e, agent, array) {
            let source = e.vcall(shape_b, 0x10, &args![]).u32();
            let data = if inside { 0 } else { e.mem.u32(array) };
            e.call(
                AGENT_1N_PROCESS,
                &args![agent + 0x30, query, source, data, output],
            );
        } else {
            e.call(SET_OUT_OF_MEMORY_STATE, &args![1u32]);
        }
        profile_record(e, LABEL_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00d2df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): a
/// create function of the tree-stream pair (code at `00d2e71d` stores its address in an agent
/// record). Allocates the 0x40-byte agent through the memory router's allocator
/// (`allocate_agent`) and runs the `hkpBvTreeStreamAgent` constructor `00d2def0` on it with the
/// four words in this order. Answers the agent.
pub fn fn_00d2df50(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent(e, 0x40);
    fn_00d2def0(
        e,
        Ptr::new(agent),
        body_a.addr(),
        body_b.addr(),
        input,
        contact_mgr,
    )
    .addr()
}

// Translated from 00d2e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): the
/// create function of the symmetric tree-stream agent (code at `00d2e6e3` stores its address in
/// an agent record). Allocates the 0x40-byte agent (`allocate_agent`), runs the constructor
/// `00d2def0` with the two bodies in the other order (`body_b`, `body_a`, `input`,
/// `contact_mgr`) and turns the agent into the symmetric one by storing the vtable
/// `010ce874`. Answers the agent.
pub fn fn_00d2e680(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent(e, 0x40);
    fn_00d2def0(
        e,
        Ptr::new(agent),
        body_b.addr(),
        body_a.addr(),
        input,
        contact_mgr,
    );
    e.mem.set_u32(agent, SYMMETRIC_VTABLE);
    agent
}

// Translated from 00d2e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): the
/// create function of the symmetric pair (code at `00d2e8f3` stores its address in an agent
/// record), the counterpart of `fn_00d2e5a0` with the two bodies exchanged. Without a contact
/// manager it makes the symmetric tree-stream agent (0x40 bytes, constructor `00d2def0` with
/// `body_b`, `body_a`, `input`, 0, then the vtable `010ce874`). With one it asks the object at
/// `input + 0x10` (slot `0xC`, with `(body_b, body_a, input)`): 0 and 2 make the same symmetric
/// agent (with the contact manager); 1 makes the symmetric shape collection agent (0x38 bytes,
/// constructor `00d440f0` with `(body_b, body_a, input, contact_mgr)`, then the vtable
/// `010ce8b0`); any other answer makes nothing and answers 0. Answers the new agent.
pub fn fn_00d2e7f0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    if contact_mgr == 0 {
        let agent = allocate_agent(e, 0x40);
        fn_00d2def0(e, Ptr::new(agent), body_b.addr(), body_a.addr(), input, 0);
        e.mem.set_u32(agent, SYMMETRIC_VTABLE);
        return agent;
    }
    let mediator = e.mem.u32(input.addr() + 0x10);
    let kind = e.vcall(mediator, 0xc, &args![body_b, body_a, input]).u32();
    match kind {
        1 => {
            let agent = allocate_agent(e, 0x38);
            e.call(
                SHAPE_COLLECTION_CONSTRUCTOR,
                &args![agent, body_b, body_a, input, contact_mgr],
            );
            e.mem.set_u32(agent, 0x010c_e8b0);
            agent
        }
        0 | 2 => {
            let agent = allocate_agent(e, 0x40);
            fn_00d2def0(
                e,
                Ptr::new(agent),
                body_b.addr(),
                body_a.addr(),
                input,
                contact_mgr,
            );
            e.mem.set_u32(agent, SYMMETRIC_VTABLE);
            agent
        }
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2df50, fn_00d2df50(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00d2e680, fn_00d2e680(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00d2e7f0, fn_00d2e7f0(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(
            0x00d2_dd30,
            hkp_bv_tree_stream_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d2_dd50,
            hkp_bv_tree_stream_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d2_dd70,
            hkp_bv_tree_stream_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d2_dda0, hkp_bv_tree_stream_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d2_ddd0,
            hkp_bv_tree_stream_agent_invalidate_tim(Ptr, u32)
        ),
        entry!(
            0x00d2_ddf0,
            hkp_bv_tree_stream_agent_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(
            0x00d2_de20,
            hkp_bv_tree_stream_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d2_de90,
            hkp_bv_tree_stream_agent_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d2_def0, fn_00d2def0(Ptr, u32, u32, Ptr, u32) -> Ptr),
        entry!(
            0x00d2_df90,
            hkp_bv_tree_stream_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d2_e5a0, fn_00d2e5a0(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00d2_e6d0, fn_00d2e6d0(Ptr)),
        entry!(0x00d2_e760, fn_00d2e760(Ptr)),
        entry!(0x00d2_e8e0, fn_00d2e8e0(Ptr)),
        entry!(0x00d2_e970, fn_00d2e970(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d2_e9b0, fn_00d2e9b0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d2_e9f0, fn_00d2e9f0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d2_eae0,
            hkp_bv_tree_stream_agent_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00d2_eb50, fn_00d2eb50(Ptr)),
        entry!(0x00d2_ebb0, fn_00d2ebb0(Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(0x00d2_ebf0, fn_00d2ebf0(Ptr, Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d2_ec30,
            hkp_symmetric_agent_linear_cast_bv_tree_stream_agent_update_shape_collection_filter(
                Ptr, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d2_ec50,
            hkp_symmetric_agent_linear_cast_bv_tree_stream_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(
            0x00d2_ed70,
            hkp_symmetric_agent_linear_cast_shape_collection_agent_update_shape_collection_filter(
                Ptr, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d2_ed90,
            hkp_symmetric_agent_linear_cast_shape_collection_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d2_edd0,
            hkp_symmetric_agent_linear_cast_shape_collection_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d2_ee10,
            hkp_symmetric_agent_linear_cast_shape_collection_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(0x00d2_ef30, fn_00d2ef30(Ptr, Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d2_f000,
            hkp_symmetric_agent_shape_collection_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        body, calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_CAST_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn install_allocator(e: &mut Engine, block: u32) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        sizes
    }

    /// An input whose word at `+0` is 0x77 and whose `+0x10` object answers `kind` from slot
    /// `0xc`; the mediator's calls are logged.
    fn input_for_00d2e7f0(e: &mut Engine, kind: u32) -> u32 {
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x77);
        e.put_vtable(0x0900_0200, &[0, 0, 0, 0x0900_0300]);
        let mediator = e.mem.alloc(8);
        e.mem.set_u32(mediator, 0x0900_0200);
        e.mem.set_u32(input + 0x10, mediator);
        e.register_double(0x0900_0300, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        input
    }

    fn calls_of_00d2e7f0(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00d2df50_allocates_the_agent_and_runs_the_constructor() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        e.register(STREAM_INIT, |_, _| Ret::default());
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x77);
        let agent = e
            .call(0x00d2_df50, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        assert_eq!(e.mem.u16(agent + 4), 0x40);
        assert_eq!(e.mem.u32(agent), VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent + 0xc), 0x77);
    }

    #[test]
    fn fn_00d2e680_makes_the_symmetric_agent() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        e.register(STREAM_INIT, |_, _| Ret::default());
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x77);
        let agent = e
            .call(0x00d2_e680, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        assert_eq!(e.mem.u16(agent + 4), 0x40);
        assert_eq!(e.mem.u32(agent), SYMMETRIC_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent + 0xc), 0x77);
    }

    #[test]
    fn fn_00d2e7f0_without_a_contact_manager_makes_the_symmetric_agent() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        e.register(STREAM_INIT, |_, _| Ret::default());
        let input = input_for_00d2e7f0(&mut e, 1);
        e.call_log = Some(vec![]);
        let agent = e.call(0x00d2_e7f0, &args![1u32, 2u32, input, 0u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        assert_eq!(e.mem.u32(agent), SYMMETRIC_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0);
        // The mediator is not asked.
        assert!(calls_of_00d2e7f0(&e, 0x0900_0300).is_empty());
    }

    #[test]
    fn fn_00d2e7f0_makes_the_symmetric_agent_for_answers_zero_and_two() {
        for kind in [0, 2] {
            let mut e = Engine::new();
            let sizes = install_allocator(&mut e, 0x0800_0000);
            e.register(STREAM_INIT, |_, _| Ret::default());
            let input = input_for_00d2e7f0(&mut e, kind);
            e.call_log = Some(vec![]);
            let agent = e
                .call(0x00d2_e7f0, &args![1u32, 2u32, input, 0x88u32])
                .u32();
            assert_eq!(agent, 0x0800_0000);
            assert_eq!(*sizes.borrow(), vec![0x40]);
            assert_eq!(e.mem.u32(agent), SYMMETRIC_VTABLE);
            assert_eq!(e.mem.u32(agent + 8), 0x88);
            assert_eq!(
                calls_of_00d2e7f0(&e, 0x0900_0300),
                vec![vec![e.mem.u32(input + 0x10), 2, 1, input]]
            );
        }
    }

    #[test]
    fn fn_00d2e7f0_makes_the_symmetric_shape_collection_agent_for_answer_one() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        e.register(SHAPE_COLLECTION_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let input = input_for_00d2e7f0(&mut e, 1);
        e.call_log = Some(vec![]);
        let agent = e
            .call(0x00d2_e7f0, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x38]);
        assert_eq!(e.mem.u16(agent + 4), 0x38);
        assert_eq!(e.mem.u32(agent), 0x010c_e8b0);
        assert_eq!(
            calls_of_00d2e7f0(&e, SHAPE_COLLECTION_CONSTRUCTOR),
            vec![vec![agent, 2, 1, input, 0x88]]
        );
    }

    #[test]
    fn fn_00d2e7f0_makes_nothing_for_any_other_answer() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let input = input_for_00d2e7f0(&mut e, 3);
        let agent = e
            .call(0x00d2_e7f0, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0);
        assert!(sizes.borrow().is_empty());
    }

    /// Objects whose vtable slots log: the agent, the object a body word points to, a
    /// collector, the allocator and the dispatcher's mediator.
    const AGENT: u32 = 0x0900_0000;
    const POINTED: u32 = 0x0910_0000;
    const COLLECTOR: u32 = 0x0920_0000;
    const ALLOCATOR: u32 = 0x0930_0000;
    const MEDIATOR: u32 = 0x0940_0000;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;
    const QUIETED_NAN: u32 = 0x7fe0_0001;

    /// Words copied at the time of a call.
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    /// An agent object (logging vtable) with contact manager `0x88` and dispatcher word
    /// `0x99`.
    fn agent(e: &mut Engine, log: &Log) -> u32 {
        let this = logging_object(e, log, AGENT);
        e.mem.set_u32(this + 8, 0x88);
        e.mem.set_u32(this + 0xc, 0x99);
        this
    }

    /// A memory router whose allocator is a logging object; its slot 4 hands out zeroed
    /// 0x40-byte blocks and notes the size asked for.
    fn memory(e: &mut Engine, log: &Log) {
        let allocator = logging_object(e, log, ALLOCATOR);
        let notes = Rc::clone(log);
        e.register_double(ALLOCATOR + 4, move |e, a| {
            notes.borrow_mut().push((ALLOCATOR + 4, a.to_vec()));
            Ret {
                eax: e.mem.alloc(0x40),
                ..Ret::default()
            }
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, MEMORY_ROUTER, router);
    }

    /// Runs `address` (a forwarder of `count` words after `this`) and returns the words the
    /// double of `callee` saw.
    fn forwarded(address: u32, callee: u32, count: u32) -> Vec<Vec<u32>> {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, callee, 0);
        let mut all = vec![0x7777];
        all.extend(1..=count);
        e.call(address, &all);
        calls(&log, callee)
    }

    #[test]
    fn get_penetrations_passes_four_words_on_00d2dd30() {
        assert_eq!(
            forwarded(0x00d2_dd30, STATIC_GET_PENETRATIONS, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn get_closest_points_passes_four_words_on_00d2dd50() {
        assert_eq!(
            forwarded(0x00d2_dd50, STATIC_GET_CLOSEST_POINTS, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn linear_cast_passes_five_words_on_00d2dd70() {
        assert_eq!(
            forwarded(0x00d2_dd70, STATIC_LINEAR_CAST, 5),
            vec![vec![1, 2, 3, 4, 5]]
        );
    }

    #[test]
    fn cleanup_cleans_the_stream_and_deletes_the_agent_00d2dda0() {
        let mut e = engine();
        let log = new_log();
        let this = agent(&mut e, &log);
        log_double(&mut e, &log, STREAM_CLEANUP, 0);
        e.call(0x00d2_dda0, &[this, 0x55]);
        assert_eq!(
            log.borrow().iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            vec![STREAM_CLEANUP, AGENT]
        );
        assert_eq!(
            calls(&log, STREAM_CLEANUP),
            vec![vec![this + 0x30, 0x99, 0x88, 0x55]]
        );
        assert_eq!(calls(&log, AGENT), vec![vec![this, 1]]);
    }

    #[test]
    fn invalidate_tim_passes_the_stream_on_00d2ddd0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STREAM_INVALIDATE_TIM, 0);
        e.call(0x00d2_ddd0, &[0x1000, 0x55]);
        assert_eq!(calls(&log, STREAM_INVALIDATE_TIM), vec![vec![0x1030, 0x55]]);
    }

    #[test]
    fn warp_time_quiets_the_two_times_00d2ddf0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STREAM_WARP_TIME, 0);
        e.call(0x00d2_ddf0, &[0x1000, SIGNALLING_NAN, 0x3f80_0000, 0x77]);
        assert_eq!(
            calls(&log, STREAM_WARP_TIME),
            vec![vec![0x1030, QUIETED_NAN, 0x3f80_0000, 0x77]]
        );
    }

    /// The words of the parameter block `00da48e0` saw, with the array pointer in front.
    fn filter_block(address: u32, args_of_call: &[u32]) -> (Engine, u32, Vec<u32>) {
        let mut e = engine();
        let log = new_log();
        let this = agent(&mut e, &log);
        for word in 0..8 {
            e.mem.set_u32(this + 0x10 + 4 * word, 0xdead_0000 + word);
        }
        let pointed = logging_object(&mut e, &log, POINTED);
        log_double(&mut e, &log, POINTED + 0x10, 0x4242);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, pointed);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(STREAM_UPDATE_FILTER, move |e, a| {
            let mut all = a.to_vec();
            all.extend(words(e, a[1], 7));
            copy.borrow_mut().push(all);
            Ret::default()
        });
        let mut all = vec![this];
        all.extend_from_slice(args_of_call);
        // The second body word is the holder; the first stays as given.
        all[2] = holder;
        e.call(address, &all);
        let result = seen.borrow()[0].clone();
        assert_eq!(result[0], this + 0x30);
        (e, this, result[2..].to_vec())
    }

    #[test]
    fn update_shape_collection_filter_clears_the_box_and_builds_the_block_00d2de20() {
        let (e, this, block) = filter_block(0x00d2_de20, &[0x11, 0, 0x33, 0x44]);
        assert_eq!(words(&e, this + 0x10, 8), vec![0; 8]);
        // body A, body B (the holder), unwritten, the answer of slot 0x10, third word, the
        // contact manager and the fourth word.
        assert_eq!(block[0], 0x11);
        assert_ne!(block[1], 0);
        assert_eq!(block[2..], [0, 0x4242, 0x33, 0x88, 0x44]);
    }

    #[test]
    fn symmetric_update_shape_collection_filter_swaps_the_bodies_00d2ec30() {
        // After the swap the first word of the block is the second body and the second is the
        // first body, which is the one with the object.
        let mut e = engine();
        let log = new_log();
        let this = agent(&mut e, &log);
        let pointed = logging_object(&mut e, &log, POINTED);
        log_double(&mut e, &log, POINTED + 0x10, 0x4242);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, pointed);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(STREAM_UPDATE_FILTER, move |e, a| {
            copy.borrow_mut().push(words(e, a[1], 7));
            Ret::default()
        });
        e.call(0x00d2_ec30, &[this, holder, 0x22, 0x33, 0x44]);
        assert_eq!(
            *seen.borrow(),
            vec![vec![0x22, holder, 0, 0x4242, 0x33, 0x88, 0x44]]
        );
    }

    /// Runs `calcContentStatistics` on an array of 5 elements at `0x5000` with the given
    /// capacity word and a collector whose byte `+0x10` is `flag`; returns the log, the agent
    /// and the collector.
    fn statistics(capacity: u32, flag: u8) -> (Log, u32, u32) {
        let mut e = engine();
        let log = new_log();
        let this = agent(&mut e, &log);
        e.mem.set_u32(this + 0x30, 0x5000);
        e.mem.set_u32(this + 0x34, 5);
        e.mem.set_u32(this + 0x38, capacity);
        let collector = logging_object(&mut e, &log, COLLECTOR);
        e.mem.set_u32(collector + 8, 0x6000);
        e.mem.set_u8(collector + 0x10, flag);
        log_double(&mut e, &log, ALLOCATED_BYTES, 0x1234);
        log_double(&mut e, &log, STREAM_CALC_STATISTICS, 0);
        e.call(0x00d2_de90, &[this, collector, 0xdddd]);
        (log, this, collector)
    }

    #[test]
    fn calc_content_statistics_reports_owned_arrays_and_adds_the_stream_00d2de90() {
        // (capacity word, collector flag, bytes asked for when it reports).
        let cases: [(u32, u8, Option<u32>); 4] = [
            (0x8000_0000, 0, None),
            (0x8000_0000, 1, Some(0)),
            (0x0000_0010, 0, Some(0x40)),
            (0xc000_0004, 1, Some(0x10)),
        ];
        for (capacity, flag, bytes) in cases {
            let (log, this, collector) = statistics(capacity, flag);
            match bytes {
                None => {
                    assert!(calls(&log, ALLOCATED_BYTES).is_empty());
                    assert!(calls(&log, COLLECTOR + 0x14).is_empty());
                }
                Some(bytes) => {
                    assert_eq!(calls(&log, ALLOCATED_BYTES), vec![vec![0x5000, bytes]]);
                    assert_eq!(
                        calls(&log, COLLECTOR + 0x14),
                        vec![vec![collector, 3, LABEL_SECTOR_PTRS, 0x5000, 20, 0x1234]]
                    );
                }
            }
            assert_eq!(
                calls(&log, STREAM_CALC_STATISTICS),
                vec![vec![this + 0x30, 0x6000, collector]]
            );
        }
    }

    #[test]
    fn constructor_builds_an_empty_agent_00d2def0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, STREAM_INIT, 0);
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xff; 0x40]);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, 0x1234);
        let result = e.call(0x00d2_def0, &[this, 1, 2, input, 0x88]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x88);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(words(&e, this + 0x10, 8), vec![0; 8]);
        assert_eq!(words(&e, this + 0x30, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(this + 0x3c), 0xffff_ffff);
        assert_eq!(calls(&log, STREAM_INIT), vec![vec![this + 0x30]]);
    }

    /// The pieces of a `createFunc` test: the engine, the log, the input, and the mediator
    /// that answers `kind`.
    fn creation(kind: u32) -> (Engine, Log, u32) {
        let mut e = engine();
        let log = new_log();
        memory(&mut e, &log);
        let mediator = logging_object(&mut e, &log, MEDIATOR);
        log_double(&mut e, &log, MEDIATOR + 0xc, kind);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x4242);
        e.mem.set_u32(input + 0x10, mediator);
        log_double(&mut e, &log, STREAM_INIT, 0);
        log_double(&mut e, &log, SHAPE_COLLECTION_CONSTRUCTOR, 0xfeed);
        (e, log, input)
    }

    #[test]
    fn create_asks_the_mediator_and_makes_the_agent_it_names_00d2e5a0() {
        // Answer 1: a shape collection agent, 0x38 bytes.
        let (mut e, log, input) = creation(1);
        let made = e.call(0x00d2_e5a0, &[0xa, 0xb, input, 0x88]).u32();
        assert_eq!(made, 0xfeed);
        let mediator = e.mem.u32(input + 0x10);
        assert_eq!(
            calls(&log, MEDIATOR + 0xc),
            vec![vec![mediator, 0xa, 0xb, input]]
        );
        let block = calls(&log, ALLOCATOR + 4)[0].clone();
        assert_eq!(block[1], 0x38);
        let constructor = calls(&log, SHAPE_COLLECTION_CONSTRUCTOR);
        assert_eq!(constructor.len(), 1);
        assert_eq!(constructor[0][1..], [0xa, 0xb, input, 0x88]);
        assert_eq!(e.mem.u16(constructor[0][0] + 4), 0x38);
        assert!(calls(&log, STREAM_INIT).is_empty());
        // Answers 0 and 2: a tree-stream agent, 0x40 bytes.
        for kind in [0, 2] {
            let (mut e, log, input) = creation(kind);
            let made = e.call(0x00d2_e5a0, &[0xa, 0xb, input, 0x88]).u32();
            assert_eq!(calls(&log, ALLOCATOR + 4).len(), 1);
            assert_eq!(calls(&log, ALLOCATOR + 4)[0][1], 0x40);
            assert_eq!(e.mem.u16(made + 4), 0x40);
            assert_eq!(e.mem.u32(made), VTABLE);
            assert_eq!(e.mem.u32(made + 8), 0x88);
            assert_eq!(e.mem.u32(made + 0xc), 0x4242);
            assert!(calls(&log, SHAPE_COLLECTION_CONSTRUCTOR).is_empty());
        }
        // Any other answer: nothing is made.
        for kind in [3, 0xffff_ffff] {
            let (mut e, log, input) = creation(kind);
            assert_eq!(e.call(0x00d2_e5a0, &[0xa, 0xb, input, 0x88]).u32(), 0);
            assert!(calls(&log, ALLOCATOR + 4).is_empty());
        }
    }

    #[test]
    fn create_without_a_contact_manager_makes_the_symmetric_agent_00d2e5a0() {
        let (mut e, log, input) = creation(1);
        let made = e.call(0x00d2_e5a0, &[0xa, 0xb, input, 0]).u32();
        assert!(calls(&log, MEDIATOR + 0xc).is_empty());
        assert_eq!(calls(&log, ALLOCATOR + 4)[0][1], 0x40);
        assert_eq!(e.mem.u16(made + 4), 0x40);
        assert_eq!(e.mem.u32(made), SYMMETRIC_VTABLE);
        assert_eq!(e.mem.u32(made + 8), 0);
        assert_eq!(e.mem.u32(made + 0xc), 0x4242);
        assert_eq!(calls(&log, STREAM_INIT), vec![vec![made + 0x30]]);
    }

    /// Runs the registration function `address`; returns, for each call of the dispatcher's
    /// registration, the entry's four words and flag bytes, then the arguments.
    fn registrations(address: u32) -> Vec<Vec<u32>> {
        let mut e = engine();
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut entry = words(e, a[1], 4);
            entry.push(u32::from(e.mem.u8(a[1] + 0x10)));
            entry.push(u32::from(e.mem.u8(a[1] + 0x11)));
            entry.extend([e.mem.u8(a[1] + 0x12), e.mem.u8(a[1] + 0x13)].map(u32::from));
            entry.extend_from_slice(a);
            copy.borrow_mut().push(entry);
            Ret::default()
        });
        e.call(address, &[0x0777_0000]);
        let result = seen.borrow().clone();
        result
    }

    /// Checks the two registrations of a tree-stream pair of the other type `other`.
    fn check_registrations(seen: &[Vec<u32>], other: u32, symmetric: u32, direct: u32) {
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen[0][..8],
            [symmetric, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_e9f0, 1, 1, 0, 0]
        );
        assert_eq!(seen[0][8], 0x0777_0000);
        assert_eq!(seen[0][10..], [8, other]);
        assert_eq!(
            seen[1][..8],
            [direct, 0x00d3_12f0, 0x00d3_1110, 0x00d3_0f50, 0, 1, 0, 0]
        );
        assert_eq!(seen[1][8], 0x0777_0000);
        assert_eq!(seen[1][10..], [other, 8]);
        assert_eq!(seen[0][9], seen[1][9]);
    }

    #[test]
    fn registration_for_the_types_8_and_0x12_00d2e6d0() {
        check_registrations(&registrations(0x00d2_e6d0), 0x12, 0x00d2_e680, 0x00d2_df50);
    }

    #[test]
    fn registration_for_the_types_8_and_0x18_00d2e760() {
        check_registrations(&registrations(0x00d2_e760), 0x18, 0x00d2_e680, 0x00d2_df50);
    }

    #[test]
    fn registration_for_the_types_8_and_0x16_00d2e8e0() {
        check_registrations(&registrations(0x00d2_e8e0), 0x16, 0x00d2_e7f0, 0x00d2_e5a0);
    }

    /// Runs a flip wrapper: `address` with `all` words, the double of `callee` noting its
    /// arguments and the three words of the collector it was given (its last argument).
    fn flip_call(address: u32, callee: u32, all: &[u32]) -> Vec<u32> {
        let mut e = engine();
        e.mem.set_u32(LARGE_FLOAT, SIGNALLING_NAN);
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(callee, move |e, a| {
            let flip = *a.last().unwrap();
            let mut noted = a.to_vec();
            noted.extend(words(e, flip, 3));
            copy.borrow_mut().push(noted);
            Ret::default()
        });
        e.call(address, all);
        let result = seen.borrow()[0].clone();
        result
    }

    #[test]
    fn static_symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d2e970() {
        let seen = flip_call(
            0x00d2_e970,
            STATIC_GET_PENETRATIONS,
            &[0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..3], [0x33, 0x22, 0x44]);
        assert_eq!(seen[4..], [FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
    }

    #[test]
    fn static_symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d2e9b0() {
        let seen = flip_call(
            0x00d2_e9b0,
            STATIC_GET_CLOSEST_POINTS,
            &[0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..3], [0x33, 0x22, 0x44]);
        assert_eq!(seen[4..], [FLIP_COLLECTOR_VTABLE, QUIETED_NAN, 0x55]);
    }

    #[test]
    fn symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d2ebb0() {
        let seen = flip_call(
            0x00d2_ebb0,
            STATIC_GET_PENETRATIONS,
            &[0x11, 0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..3], [0x33, 0x22, 0x44]);
        assert_eq!(seen[4..], [FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
    }

    #[test]
    fn symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d2ebf0() {
        let seen = flip_call(
            0x00d2_ebf0,
            STATIC_GET_CLOSEST_POINTS,
            &[0x11, 0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..3], [0x33, 0x22, 0x44]);
        assert_eq!(seen[4..], [FLIP_COLLECTOR_VTABLE, QUIETED_NAN, 0x55]);
    }

    #[test]
    fn shape_collection_symmetric_get_penetrations_flips_the_collector_00d2ed90() {
        let seen = flip_call(
            0x00d2_ed90,
            SHAPE_COLLECTION_GET_PENETRATIONS,
            &[0x11, 0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..4], [0x11, 0x33, 0x22, 0x44]);
        assert_eq!(seen[5..], [FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
    }

    #[test]
    fn shape_collection_symmetric_get_closest_points_flips_the_collector_00d2edd0() {
        let seen = flip_call(
            0x00d2_edd0,
            SHAPE_COLLECTION_GET_CLOSEST_POINTS,
            &[0x11, 0x22, 0x33, 0x44, 0x55],
        );
        assert_eq!(seen[..4], [0x11, 0x33, 0x22, 0x44]);
        assert_eq!(seen[5..], [FLIP_COLLECTOR_VTABLE, QUIETED_NAN, 0x55]);
    }

    #[test]
    fn shape_collection_symmetric_update_shape_collection_filter_swaps_the_bodies_00d2ed70() {
        let seen = forwarded_swapped(0x00d2_ed70, SHAPE_COLLECTION_UPDATE_FILTER);
        assert_eq!(seen, vec![vec![0x7777, 2, 1, 3, 4]]);
    }

    /// Runs `address` with `this` 0x7777 and the words 1 to 4 and returns what the double of
    /// `callee` saw.
    fn forwarded_swapped(address: u32, callee: u32) -> Vec<Vec<u32>> {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, callee, 0);
        e.call(address, &[0x7777, 1, 2, 3, 4]);
        calls(&log, callee)
    }

    /// Everything a symmetric linear cast showed its callee: the argument words and, at the
    /// time of the call, the path vector of the input copy and the words of both collectors.
    struct Cast {
        args: Vec<u32>,
        path: Vec<u32>,
        first: Vec<u32>,
        second: Vec<u32>,
    }

    /// The vtable, large float, path vector and wrapped collector of a flip-cast collector.
    fn cast_collector(e: &Engine, at: u32) -> Vec<u32> {
        if at == 0 {
            return Vec::new();
        }
        let mut out = words(e, at, 2);
        out.extend(words(e, at + 0x10, 5));
        out
    }

    /// Runs the symmetric linear cast `address` with `prefix` words before the bodies (`this`)
    /// and the start collector `start`.
    fn cast(address: u32, callee: u32, prefix: &[u32], start: u32) -> Cast {
        let mut e = engine();
        e.mem.set_u32(LARGE_FLOAT, SIGNALLING_NAN);
        for lane in 0..4 {
            e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
        }
        let seen: Rc<RefCell<Option<Cast>>> = Rc::new(RefCell::new(None));
        let copy = Rc::clone(&seen);
        e.register_double(callee, move |e, a| {
            let n = a.len();
            *copy.borrow_mut() = Some(Cast {
                args: a.to_vec(),
                path: words(e, a[n - 3] + 0x50, 4),
                first: cast_collector(e, a[n - 2]),
                second: cast_collector(e, a[n - 1]),
            });
            Ret::default()
        });
        let input = e.mem.alloc(0x70);
        for (n, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(input + 0x50 + 4 * n as u32, value);
        }
        let mut all = prefix.to_vec();
        all.extend([0xa, 0xb, input, 0xc1, start]);
        e.call(address, &all);
        let result = seen.borrow_mut().take().unwrap();
        assert_ne!(result.args[result.args.len() - 3], input);
        result
    }

    /// Checks a symmetric cast's result: the bodies after `skip` words are swapped, the path
    /// is negated and the collectors wrap the caller's.
    fn check_cast(cast: &Cast, skip: usize, start: u32) {
        assert_eq!(cast.args[skip..skip + 2], [0xb, 0xa]);
        assert_eq!(cast.path, [-1.0f32, -2.0, -3.0, -4.0].map(f32::to_bits));
        let wrapped = |inner: u32| {
            let mut expected = vec![FLIP_CAST_COLLECTOR_VTABLE, QUIETED_NAN];
            expected.extend([1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits));
            expected.push(inner);
            expected
        };
        assert_eq!(cast.first, wrapped(0xc1));
        if start == 0 {
            assert_eq!(cast.args[cast.args.len() - 1], 0);
            assert!(cast.second.is_empty());
        } else {
            assert_eq!(cast.second, wrapped(start));
        }
    }

    #[test]
    fn static_symmetric_linear_cast_wraps_the_collectors_00d2e9f0() {
        for start in [0xd2, 0] {
            let result = cast(0x00d2_e9f0, STATIC_LINEAR_CAST, &[], start);
            check_cast(&result, 0, start);
        }
    }

    #[test]
    fn symmetric_linear_cast_wraps_the_collectors_00d2ef30() {
        for start in [0xd2, 0] {
            let result = cast(0x00d2_ef30, STATIC_LINEAR_CAST, &[0x7777], start);
            // The cast reaches the static cast through `hkpBvTreeStreamAgent::linearCast`.
            check_cast(&result, 0, start);
        }
    }

    #[test]
    fn shape_collection_symmetric_linear_cast_wraps_the_collectors_00d2f000() {
        for start in [0xd2, 0] {
            let result = cast(0x00d2_f000, SHAPE_COLLECTION_LINEAR_CAST, &[0x7777], start);
            assert_eq!(result.args[0], 0x7777);
            check_cast(&result, 1, start);
        }
    }

    /// Runs a symmetric `processCollision` (`address`) whose process function (`process`) is
    /// a double that appends one contact point and sets the time of impact to `new_time`;
    /// returns the engine, the output, the first free contact point and the words the process
    /// function was asked with.
    fn symmetric_process(
        address: u32,
        process: u32,
        new_time: f32,
    ) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = engine();
        for lane in 0..4u32 {
            let sign = if lane < 3 { 0x8000_0000 } else { 0 };
            e.mem.set_u32(0x010c_4c00 + 4 * lane, sign);
        }
        let output = e.mem.alloc(0x3100);
        let free = e.mem.alloc(0x100);
        e.mem.set_u32(output, free);
        e.mem.set_f32(output + 0x3030, 0.5);
        e.mem.set_u8(output + 0x3048, 1);
        e.mem.set_u8(output + 0x3049, 2);
        e.mem.set_u8(output + 0x304a, 0x12);
        for n in 0..4 {
            e.mem.set_u16(output + 0x3040 + 2 * n, 10 + n as u16);
        }
        store_vec(&mut e, output + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(process, move |e, a| {
            *copy.borrow_mut() = a.to_vec();
            let at = e.mem.u32(a[4]);
            store_vec(e, at, [1.0, 2.0, 3.0, 4.0]);
            store_vec(e, at + 0x10, [0.5, 0.0, 0.25, 2.0]);
            e.mem.set_u32(a[4], at + 0x30);
            e.mem.set_f32(a[4] + 0x3030, new_time);
            Ret::default()
        });
        e.call(address, &[0x11, 0x22, 0x33, 0x44, output]);
        let words = seen.borrow().clone();
        (e, output, free, words)
    }

    fn check_flipped_contacts(address: u32, process: u32) {
        let (e, output, free, words) = symmetric_process(address, process, 0.5);
        assert_eq!(words, vec![0x11, 0x33, 0x22, 0x44, output]);
        assert_eq!(e.mem.u32(output), free + 0x30);
        // Position += normal.w * normal, normal.xyz negated.
        assert_eq!(load_vec(&e, free), [2.0, 2.0, 3.5, 8.0]);
        assert_eq!(load_vec(&e, free + 0x10), [-0.5, -0.0, -0.25, 2.0]);
        // The time did not change: the TOI data stays.
        assert_eq!(load_vec(&e, output + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x12);
    }

    fn check_turned_toi(address: u32, process: u32) {
        let (e, output, _, _) = symmetric_process(address, process, 0.25);
        assert_eq!(load_vec(&e, output + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(e.mem.u8(output + 0x3048), 2);
        assert_eq!(e.mem.u8(output + 0x3049), 1);
        let turned: Vec<u16> = (0..4).map(|n| e.mem.u16(output + 0x3040 + 2 * n)).collect();
        assert_eq!(turned, vec![12, 11, 10, 13]);
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contacts_00d2ec50() {
        check_flipped_contacts(0x00d2_ec50, PROCESS_COLLISION);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes_00d2ec50() {
        check_turned_toi(0x00d2_ec50, PROCESS_COLLISION);
    }

    #[test]
    fn shape_collection_symmetric_process_collision_flips_the_new_contacts_00d2ee10() {
        check_flipped_contacts(0x00d2_ee10, SHAPE_COLLECTION_PROCESS_COLLISION);
    }

    #[test]
    fn shape_collection_symmetric_process_collision_turns_the_toi_data_00d2ee10() {
        check_turned_toi(0x00d2_ee10, SHAPE_COLLECTION_PROCESS_COLLISION);
    }

    #[test]
    fn flip_toi_words_turns_the_block_at_this_00d2eb50() {
        let mut e = engine();
        let at = e.mem.alloc(0x20);
        for n in 0..4 {
            e.mem.set_u16(at + 2 * n, 10 + n as u16);
        }
        e.mem.set_u8(at + 8, 1);
        e.mem.set_u8(at + 9, 2);
        e.mem.set_u8(at + 0xa, 0x12);
        e.call(0x00d2_eb50, &[at]);
        assert_eq!(e.mem.u8(at + 0xa), 0x21);
        assert_eq!((e.mem.u8(at + 8), e.mem.u8(at + 9)), (2, 1));
        let turned: Vec<u16> = (0..4).map(|n| e.mem.u16(at + 2 * n)).collect();
        assert_eq!(turned, vec![12, 11, 10, 13]);
        // Both counts 2: the middle words exchange too.
        e.mem.set_u8(at + 8, 2);
        e.mem.set_u8(at + 9, 2);
        e.call(0x00d2_eb50, &[at]);
        let turned: Vec<u16> = (0..4).map(|n| e.mem.u16(at + 2 * n)).collect();
        assert_eq!(turned, vec![13, 10, 11, 12]);
    }

    /// Runs the destructor on an agent with the given array and `flags`; returns
    /// the log and the agent.
    fn destroy(capacity: u32, flags: u8) -> (Log, u32) {
        let mut e = engine();
        let log = new_log();
        memory(&mut e, &log);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x40);
        e.mem.set_u32(this + 0x30, 0x5000);
        e.mem.set_u32(this + 0x34, 5);
        e.mem.set_u32(this + 0x38, capacity);
        let result = e.call(0x00d2_eae0, &[this, u32::from(flags)]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(words(&e, this + 0x30, 3), vec![0, 0, 0x8000_0000]);
        (log, this)
    }

    #[test]
    fn destructor_gives_the_data_and_the_block_back_00d2eae0() {
        let (log, this) = destroy(0x10, 1);
        let allocator = calls(&log, ALLOCATOR + 0x10);
        assert_eq!(allocator.len(), 1);
        assert_eq!(allocator[0][1..], [0x5000, 0x40]);
        assert_eq!(
            calls(&log, ALLOCATOR + 8),
            vec![vec![allocator[0][0], this, 0x40]]
        );
    }

    #[test]
    fn destructor_keeps_borrowed_data_and_the_block_without_the_flag_00d2eae0() {
        let (log, _) = destroy(0xc000_0004, 0);
        assert!(calls(&log, ALLOCATOR + 0x10).is_empty());
        assert!(calls(&log, ALLOCATOR + 8).is_empty());
    }

    const SHAPE_A: u32 = 0x0950_0000;
    const SHAPE_B: u32 = 0x0960_0000;
    const HIT_SOURCE: u32 = 0x1234_0000;
    const OUTPUT: u32 = 0x0abc_0000;

    /// A box: minimum and maximum vector.
    type Bounds = (Vec4, Vec4);
    /// The boxes body B's shape was asked about.
    type BoxesSeen = Rc<RefCell<Vec<Bounds>>>;

    /// Everything a run of `processCollision` needs: an agent (word `0x77` at `+8`, three
    /// entries in its array), two bodies with motion states, an input and the doubles of the
    /// callees.
    struct Collision {
        e: Engine,
        log: Log,
        agent: u32,
        body_a: u32,
        body_b: u32,
        input: u32,
        state_b: u32,
        stream: u32,
        records: u32,
        process: Words,
    }

    /// Body A's shape answers its box query with `shape_box`; `sweeping` is the flag word
    /// the input points to. The motion states are chosen so the displacement carried into B's
    /// frame is `(1, -0.5, 0)` (time step 2, B's rotation the identity), and the constants
    /// the code reads from the exe's data are set to values exact in binary (growth factor
    /// 0.5 instead of 0.4).
    fn collision(sweeping: bool, shape_box: Bounds) -> Collision {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        e.set_global(HALF_F64, 0.5f64);
        e.set_global(GROWTH_FACTOR, 0.5f32);
        e.set_global(SWEEP_FACTOR, -2.0f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        log_double(&mut e, &log, ALLOCATOR + 0x24, 1);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        for callee in [
            TRANSFORM_SET_MUL_INVERSE_MUL,
            TRANSFORM_SET_INVERSE,
            QUICK_SORT,
            SET_OUT_OF_MEMORY_STATE,
        ] {
            log_double(&mut e, &log, callee, 0);
        }
        let process: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&process);
        e.register_double(AGENT_1N_PROCESS, move |e, a| {
            let mut seen = a.to_vec();
            seen.extend(words(e, a[1], 5));
            sink.borrow_mut().push(seen);
            Ret::default()
        });

        let shape_a = logging_object(&mut e, &log, SHAPE_A);
        let notes = Rc::clone(&log);
        e.register_double(SHAPE_A + 0x1c, move |e, a| {
            notes.borrow_mut().push((SHAPE_A + 0x1c, a.to_vec()));
            store_vec(e, a[3], shape_box.0);
            store_vec(e, a[3] + 0x10, shape_box.1);
            Ret::default()
        });
        let shape_b = logging_object(&mut e, &log, SHAPE_B);
        log_double(&mut e, &log, SHAPE_B + 0x10, HIT_SOURCE);

        let state_a = e.mem.alloc(0xc0);
        store_vec(&mut e, state_a + 0x40, [2.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, state_a + 0x50, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(state_a + 0x5c, 0.5);
        e.mem.set_f32(state_a + 0x9c, 1.0);
        e.mem.set_f32(state_a + 0xa0, 0.25);
        let state_b = e.mem.alloc(0xc0);
        store_vec(&mut e, state_b, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, state_b + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, state_b + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, state_b + 0x30, [0.5, 0.0, 0.0, 0.0]);
        store_vec(&mut e, state_b + 0x50, [0.0, -0.5, 0.0, 0.0]);
        e.mem.set_f32(state_b + 0x5c, 0.5);
        store_vec(&mut e, state_b + 0x90, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_f32(state_b + 0x9c, 2.0);
        e.mem.set_f32(state_b + 0xa0, 0.5);
        let body_a = body(&mut e, shape_a, state_a);
        let body_b = body(&mut e, shape_b, state_b);

        let input = e.mem.alloc(0x70);
        e.mem.set_f32(input + 8, 0.5);
        e.mem.set_f32(input + 0x58, 2.0);
        let flags = e.mem.alloc(0x20);
        e.mem.set_u32(flags + 0x10, u32::from(sweeping));
        e.mem.set_u32(input + 0x60, flags);

        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent + 8, 0x77);
        e.mem.set_u32(agent + 0x34, 3);
        Collision {
            e,
            log,
            agent,
            body_a,
            body_b,
            input,
            state_b,
            stream,
            records,
            process,
        }
    }

    impl Collision {
        fn set_cached_box(&mut self, bounds: Bounds) {
            store_vec(&mut self.e, self.agent + 0x10, bounds.0);
            store_vec(&mut self.e, self.agent + 0x20, bounds.1);
        }

        fn cached_box(&self) -> Bounds {
            (
                load_vec(&self.e, self.agent + 0x10),
                load_vec(&self.e, self.agent + 0x20),
            )
        }

        /// Body B's shape answers its box query (slot `0x30`) by noting the box and, when
        /// `hits` is not empty, giving the stack array heap storage (room for 16 entries,
        /// owned) holding `hits`.
        fn answer_shape_query(&mut self, hits: Vec<u32>) -> BoxesSeen {
            let seen: BoxesSeen = Rc::new(RefCell::new(Vec::new()));
            let sink = Rc::clone(&seen);
            let notes = Rc::clone(&self.log);
            self.e.register_double(SHAPE_B + 0x30, move |e, a| {
                notes.borrow_mut().push((SHAPE_B + 0x30, a.to_vec()));
                sink.borrow_mut()
                    .push((load_vec(e, a[1]), load_vec(e, a[1] + 0x10)));
                if !hits.is_empty() {
                    let data = e.mem.alloc(0x40);
                    for (index, hit) in hits.iter().enumerate() {
                        e.mem.set_u32(data + 4 * index as u32, *hit);
                    }
                    e.mem.set_u32(a[2], data);
                    e.mem.set_u32(a[2] + 4, hits.len() as u32);
                    e.mem.set_u32(a[2] + 8, 0x10);
                }
                Ret::default()
            });
            seen
        }

        fn run(&mut self) {
            self.e.call(
                PROCESS_COLLISION,
                &[self.agent, self.body_a, self.body_b, self.input, OUTPUT],
            );
        }

        /// The labels of the profile records after the 16-byte start record.
        fn labels(&self) -> Vec<u32> {
            assert_eq!(self.e.mem.u32(self.records), LABEL_LT_BV_TREE3);
            assert_eq!(self.e.mem.u32(self.records + 0xc), LABEL_ST_QUERY_TREE);
            profile_labels(&self.e, self.stream, self.records + 0x10)
        }

        /// The addresses of the logged calls among `addresses`, in order.
        fn order(&self, addresses: &[u32]) -> Vec<u32> {
            self.log
                .borrow()
                .iter()
                .map(|(address, _)| *address)
                .filter(|address| addresses.contains(address))
                .collect()
        }

        /// The first word after `this` of every logged call to `address`.
        fn first_words(&self, address: u32) -> Vec<u32> {
            calls(&self.log, address).iter().map(|a| a[1]).collect()
        }
    }

    const SHAPE_BOX: Bounds = ([1.0, 2.0, 3.0, 0.0], [5.0, 6.0, 7.0, 0.0]);
    const SWEEP_BOX: Bounds = ([-3.0, -1.0, -3.0, -1.0], [3.0, 1.0, 3.0, 1.0]);
    const SMALL_CACHED_BOX: Bounds = ([0.0; 4], [1.0; 4]);

    #[test]
    fn process_collision_skips_the_shape_query_when_the_cached_box_holds_the_box_00d2df90() {
        let mut c = collision(false, SHAPE_BOX);
        // Lane 3 is outside the cached box but only lanes 0 to 2 are compared.
        let cached = ([0.0, 0.0, 0.0, 1000.0], [10.0, 10.0, 10.0, -1000.0]);
        c.set_cached_box(cached);
        c.run();
        let aabb = calls(&c.log, SHAPE_A + 0x1c);
        assert_eq!(aabb.len(), 1);
        let (inverse, bounds) = (aabb[0][1], aabb[0][3]);
        assert_eq!(f32::from_bits(aabb[0][2]), 0.25);
        assert_eq!(inverse, bounds + 0xb0);
        let relative = inverse - 0x60;
        let (state_a, state_b) = (c.e.mem.u32(c.body_a + 8), c.e.mem.u32(c.body_b + 8));
        assert_eq!(
            calls(&c.log, TRANSFORM_SET_MUL_INVERSE_MUL),
            vec![vec![relative, state_a, state_b]]
        );
        assert_eq!(
            calls(&c.log, TRANSFORM_SET_INVERSE),
            vec![vec![inverse, relative]]
        );
        assert!(calls(&c.log, SHAPE_B + 0x30).is_empty());
        assert!(calls(&c.log, ALLOCATOR + 0x24).is_empty());
        assert!(calls(&c.log, QUICK_SORT).is_empty());
        assert!(calls(&c.log, ALLOCATOR + 0x10).is_empty());
        assert_eq!(
            *c.process.borrow(),
            vec![vec![
                c.agent + 0x30,
                bounds + 0x30,
                HIT_SOURCE,
                0,
                OUTPUT,
                c.body_a,
                c.body_b,
                0,
                c.input,
                0x77
            ]]
        );
        assert_eq!(c.cached_box(), cached);
        assert_eq!(c.labels(), vec![LABEL_ST_NARROW, LABEL_END]);
    }

    #[test]
    fn process_collision_grows_the_cached_box_and_asks_the_shape_00d2df90() {
        let mut c = collision(false, SHAPE_BOX);
        c.set_cached_box(([2.0, 0.0, 0.0, 0.0], [10.0; 4]));
        let seen = c.answer_shape_query(Vec::new());
        c.run();
        let grown = ([-1.25, 1.75, 2.75, -0.25], [5.25, 7.25, 7.25, 0.25]);
        assert_eq!(c.cached_box(), grown);
        assert_eq!(*seen.borrow(), vec![grown]);
        let query = calls(&c.log, SHAPE_B + 0x30);
        assert_eq!(query.len(), 1);
        let (bounds, array) = (query[0][1], query[0][2]);
        assert_eq!(array, bounds + 0xf0);
        // (0 / 4 - 3 + 1) << 9
        assert_eq!(c.first_words(ALLOCATOR + 0x24), vec![0xffff_fc00]);
        assert_eq!(
            c.order(&[SHAPE_B + 0x30, ALLOCATOR + 0x24, SHAPE_B + 0x10]),
            vec![SHAPE_B + 0x30, ALLOCATOR + 0x24, SHAPE_B + 0x10]
        );
        assert!(calls(&c.log, QUICK_SORT).is_empty());
        assert!(calls(&c.log, ALLOCATOR + 0x10).is_empty());
        // The array still holds its first entry in the frame, 0xFFFFFFFF.
        assert_eq!(
            *c.process.borrow(),
            vec![vec![
                c.agent + 0x30,
                bounds + 0x30,
                HIT_SOURCE,
                array + 0xc,
                OUTPUT,
                c.body_a,
                c.body_b,
                0,
                c.input,
                0x77
            ]]
        );
        assert_eq!(c.labels(), vec![LABEL_ST_NARROW, LABEL_END]);
    }

    #[test]
    fn process_collision_sorts_the_hits_and_frees_their_storage_00d2df90() {
        let mut c = collision(false, SHAPE_BOX);
        c.set_cached_box(([2.0, 0.0, 0.0, 0.0], [10.0; 4]));
        c.e.mem.set_u32(c.agent + 0x34, 1);
        c.answer_shape_query((0..10).rev().collect());
        c.run();
        // (10 / 4 - 1 + 1) << 9
        assert_eq!(c.first_words(ALLOCATOR + 0x24), vec![0x400]);
        let sort = calls(&c.log, QUICK_SORT);
        assert_eq!(sort.len(), 1);
        let heap = sort[0][0];
        assert_eq!(sort[0][1..3], [0, 9]);
        assert_eq!(
            c.order(&[
                ALLOCATOR + 0x24,
                QUICK_SORT,
                SHAPE_B + 0x10,
                ALLOCATOR + 0x10
            ]),
            vec![
                ALLOCATOR + 0x24,
                QUICK_SORT,
                SHAPE_B + 0x10,
                ALLOCATOR + 0x10
            ]
        );
        let process = c.process.borrow();
        assert_eq!(process.len(), 1);
        assert_eq!(process[0][3], heap);
        // The storage holds 16 entries of 4 bytes.
        assert_eq!(
            calls(&c.log, ALLOCATOR + 0x10)
                .iter()
                .map(|a| a[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![heap, 0x40]]
        );
    }

    #[test]
    fn process_collision_gives_up_when_the_allocator_refuses_00d2df90() {
        let mut c = collision(false, SHAPE_BOX);
        c.set_cached_box(([2.0, 0.0, 0.0, 0.0], [10.0; 4]));
        c.answer_shape_query(vec![5, 4]);
        log_double(&mut c.e, &c.log, ALLOCATOR + 0x24, 0);
        c.run();
        assert_eq!(
            c.order(&[ALLOCATOR + 0x24, SET_OUT_OF_MEMORY_STATE]),
            vec![ALLOCATOR + 0x24, SET_OUT_OF_MEMORY_STATE]
        );
        assert_eq!(calls(&c.log, SET_OUT_OF_MEMORY_STATE), vec![vec![1]]);
        assert!(calls(&c.log, QUICK_SORT).is_empty());
        assert!(calls(&c.log, SHAPE_B + 0x10).is_empty());
        assert!(c.process.borrow().is_empty());
        let freed = calls(&c.log, ALLOCATOR + 0x10);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 0x40);
        assert_eq!(c.labels(), vec![LABEL_ST_NARROW, LABEL_END]);
    }

    #[test]
    fn process_collision_widens_a_sweeping_box_by_the_spin_when_the_radius_is_positive_00d2df90() {
        let mut c = collision(true, SWEEP_BOX);
        c.set_cached_box(SMALL_CACHED_BOX);
        c.run();
        let aabb = calls(&c.log, SHAPE_A + 0x1c);
        // ((1 + 2) * 0.25 + 2 * 2 * 0.5) + 0.5 * 0.5
        assert_eq!(f32::from_bits(aabb[0][2]), 3.0);
        assert_eq!(
            c.cached_box(),
            ([-4.25, -1.75, -3.75, -1.25], [4.25, 2.25, 3.25, 1.25])
        );
    }

    #[test]
    fn process_collision_leaves_a_sweeping_box_without_the_spin_for_no_radius_00d2df90() {
        let mut c = collision(true, SWEEP_BOX);
        c.set_cached_box(SMALL_CACHED_BOX);
        c.e.mem.set_f32(c.state_b + 0x9c, 0.0);
        c.run();
        let aabb = calls(&c.log, SHAPE_A + 0x1c);
        // (1 * 0.25 + 0) + 0.5 * 0.5
        assert_eq!(f32::from_bits(aabb[0][2]), 0.5);
        assert_eq!(
            c.cached_box(),
            ([-0.75, -1.25, -0.75, -0.75], [2.25, 1.25, 0.75, 0.75])
        );
    }

    #[test]
    fn process_collision_writes_no_records_when_the_stream_is_full_00d2df90() {
        let mut c = collision(false, SHAPE_BOX);
        c.set_cached_box(([0.0; 4], [10.0; 4]));
        c.e.mem.set_u32(c.stream + 0xc, c.records);
        c.run();
        assert_eq!(c.e.mem.u32(c.stream + 4), c.records);
        assert_eq!(c.process.borrow().len(), 1);
    }
}
