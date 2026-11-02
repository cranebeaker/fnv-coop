//! `Havok SDK/hkpbvagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpBvAgent` (vtable `010cf3cc`) is the collision agent of a body whose shape is a
//! bounding-volume shape: it keeps a first agent at `+0xC` (the pair of the bounding volume
//! and the other body, made when the agent is) and a second agent at `+0x10` (the pair of the
//! wrapped child shape and the other body, made on demand when the bounding volume overlaps).
//! The agent base is the Xbox PDB's `hkpCollisionAgent`: `m_memSizeAndFlags` `+4` (u16),
//! `m_referenceCount` `+6` (i16), `m_contactMgr` `+8`; its vtable slots are `0xC`
//! `getPenetrations`, `0x10` `getClosestPoints`, `0x14` `linearCast`, `0x18`
//! `processCollision`, `0x1C` `cleanup`, `0x20` `updateShapeCollectionFilter`, `0x24`
//! `invalidateTim`, `0x28` `warpTime`, `0x2C` `removePoint`, `0x30` `commitPotential` and
//! `0x34` `createZombie`.
//!
//! A body passed to the agent functions is an `hkpCdBody` (Xbox PDB, 0x10 bytes: `m_shape`
//! `+0`, `m_shapeKey` `+4`, `m_motion` `+8`, `m_parent` `+0xC`); the agents build the one for
//! the wrapped shape on the stack. The collision dispatcher (the first word of the
//! `hkpProcessCollisionInput`) holds a table of 5-word agent entries at `+0x9A0` (`createFunc`
//! `+0`, `getPenetrationsFunc` `+4`, `getClosestPointsFunc` `+8`, `linearCastFunc` `+0xC`),
//! indexed by a byte matrix of shape-type pairs.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::{
    profile_record, time_stamp_low, PROFILE_SLOT, TLS_GET_VALUE,
};

/// The float at `0104da48` (bits `7f7fffee`), the "no hit yet" distance of the collectors.
pub(crate) const LARGE_FLOAT: u32 = 0x0104_da48;
/// Labels of the profiling records: `"LthkpBvAgent"`, `"StcheckBvShape"`, `"Stchild"` and the
/// shared end label.
pub(crate) const LABEL_BV_AGENT: u32 = 0x010c_f414;
pub(crate) const LABEL_CHECK_BV_SHAPE: u32 = 0x010c_f404;
pub(crate) const LABEL_CHILD: u32 = 0x010c_ec4c;
pub(crate) const LABEL_END: u32 = 0x0107_6f38;
/// Vtable of the `hkpBvAgent` (`.?AVhkpBvAgent@@`).
const BV_AGENT_VTABLE: u32 = 0x010c_f3cc;
/// Vtable of `hkpFlagCdBodyPairCollector`: an 8-byte object (vtable, hit flag byte at `+4`).
pub(crate) const FLAG_COLLECTOR_VTABLE: u32 = 0x010c_ebd0;
/// Vtable of `hkpSimpleClosestContactCollector`, the collector the linear cast checks with.
pub(crate) const CLOSEST_CONTACT_COLLECTOR_VTABLE: u32 = 0x010c_ebe0;
/// Vtables of `hkpSymmetricAgentFlipCollector`, `hkpSymmetricAgentFlipCastCollector` and
/// `hkpSymmetricAgentFlipBodyCollector`.
pub(crate) const FLIP_COLLECTOR_VTABLE: u32 = 0x010c_e7fc;
pub(crate) const FLIP_CAST_COLLECTOR_VTABLE: u32 = 0x010c_e80c;
pub(crate) const FLIP_BODY_COLLECTOR_VTABLE: u32 = 0x010c_e81c;
/// `hkpCollisionDispatcher` agent registration (`00cfbbe0`, `this` = dispatcher, `RET 0xC`):
/// `(agent function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The sign masks the code XORs a vector with to negate it: all four lanes (`010c44d0`) and
/// lanes 0 to 2 (`010c4c00`).
const NEGATE_ALL_MASK: u32 = 0x010c_44d0;
const NEGATE_XYZ_MASK: u32 = 0x010c_4c00;

/// Appends a 16-byte profiling record to the thread's record stream while its cursor (`+4`)
/// is below its limit (`+0xC`): `first` at `+0`, the time stamp at `+4`, `second` at `+0xC`.
pub(crate) fn profile_start_record(e: &mut Engine, first: u32, second: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, first);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// Writes an `hkpCdBody` (`m_shape`, `m_shapeKey`, `m_motion`, `m_parent`) at `body`.
pub(crate) fn set_cd_body(
    e: &mut Engine,
    body: u32,
    shape: u32,
    key: u32,
    motion: u32,
    parent: u32,
) {
    e.mem.set_u32(body, shape);
    e.mem.set_u32(body + 4, key);
    e.mem.set_u32(body + 8, motion);
    e.mem.set_u32(body + 0xc, parent);
}

/// The function the dispatcher's `createFunc` matrix gives for a pair of shape types: the
/// byte at `dispatcher + row + (type_a << 5) + type_b` (row `0x1A0`, or `0x5A0` when the
/// input's `m_createPredictiveAgents` (`+0x14`) is set) indexes the 5-word entries at
/// `+0x9A0`.
pub(crate) fn create_function(e: &Engine, input: u32, type_a: u32, type_b: u32) -> u32 {
    let dispatcher = e.mem.u32(input);
    let row = if e.mem.u32(input + 0x14) == 0 {
        0x1a0
    } else {
        0x5a0
    };
    let index = u32::from(
        e.mem.u8((type_a << 5)
            .wrapping_add(dispatcher.wrapping_add(row))
            .wrapping_add(type_b)),
    );
    e.mem.u32(
        dispatcher
            .wrapping_add(index.wrapping_mul(0x14))
            .wrapping_add(0x9a0),
    )
}

/// The function of a static entry point: the byte at `dispatcher + ((type_a + 0xD) << 5) +
/// type_b` indexes the 5-word entries, and `slot` (`0x9A4` `getPenetrations`, `0x9A8`
/// `getClosestPoints`, `0x9AC` `linearCast`) picks the word.
pub(crate) fn static_function(e: &Engine, input: u32, type_a: u32, type_b: u32, slot: u32) -> u32 {
    let dispatcher = e.mem.u32(input);
    let index = u32::from(
        e.mem.u8((type_a.wrapping_add(0xd) << 5)
            .wrapping_add(dispatcher)
            .wrapping_add(type_b)),
    );
    e.mem.u32(
        dispatcher
            .wrapping_add(index.wrapping_mul(0x14))
            .wrapping_add(slot),
    )
}

/// The type word (`hkpShape::m_type`, `+0xC`) of the shape `shape` points to.
pub(crate) fn shape_type(e: &Engine, shape: u32) -> u32 {
    e.mem.u32(shape + 0xc)
}

/// Whether a flag collector (vtable `010cebd0`, hit byte at `+4`) was hit.
fn flag_hit(e: &Engine, collector: u32) -> bool {
    e.mem.u8(collector + 4) != 0
}

/// Writes the hit-flag collector at `collector`: its vtable and a cleared hit byte.
pub(crate) fn init_flag_collector(e: &mut Engine, collector: u32) {
    e.mem.set_u32(collector, FLAG_COLLECTOR_VTABLE);
    e.mem.set_u8(collector + 4, 0);
}

/// Writes the closest-contact collector the linear casts check with at `collector`: its
/// vtable, the large float (through `FLD`/`FSTP`) at `+4` and `+0x2C`, and a cleared hit byte
/// at `+8`.
pub(crate) fn init_closest_contact_collector(e: &mut Engine, collector: u32) {
    let large = x87_quiet_bits(e.global::<u32>(LARGE_FLOAT));
    e.mem.set_u32(collector, CLOSEST_CONTACT_COLLECTOR_VTABLE);
    e.mem.set_u32(collector + 4, large);
    e.mem.set_u8(collector + 8, 0);
    e.mem.set_u32(collector + 0x2c, large);
}

// Translated from 00d45230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::cleanup` (Xbox PDB): cleans up the first agent (slot `0x1C`) and the second
/// one when there is one (clearing its slot), then deletes the agent itself through its
/// vtable's first slot with the flag 1. `param` is the word passed on.
pub fn hkp_bv_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let first = e.mem.u32(this.addr() + 0xc);
    e.vcall(first, 0x1c, &args![param]);
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x1c, &args![param]);
        e.mem.set_u32(this.addr() + 0x10, 0);
    }
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d45270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::invalidateTim` (Xbox PDB): passes the word to both agents' slot `0x24`.
pub fn hkp_bv_agent_invalidate_tim(e: &mut Engine, this: Ptr, param: u32) {
    let first = e.mem.u32(this.addr() + 0xc);
    e.vcall(first, 0x24, &args![param]);
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x24, &args![param]);
    }
}

// Translated from 00d452a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::warpTime` (Xbox PDB): passes the two times (floats, through `FLD`/`FSTP`)
/// and the third word to both agents' slot `0x28`.
pub fn hkp_bv_agent_warp_time(e: &mut Engine, this: Ptr, old_time: f32, new_time: f32, third: u32) {
    let old_time = x87_quiet(old_time);
    let new_time = x87_quiet(new_time);
    let first = e.mem.u32(this.addr() + 0xc);
    e.vcall(first, 0x28, &args![old_time, new_time, third]);
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x28, &args![old_time, new_time, third]);
    }
}

// Translated from 00d452f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::removePoint` (Xbox PDB): when there is a second agent, passes the word to its
/// slot `0x2C` (a tail jump).
pub fn hkp_bv_agent_remove_point(e: &mut Engine, this: Ptr, param: u32) {
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x2c, &args![param]);
    }
}

// Translated from 00d45310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::commitPotential` (Xbox PDB): when there is a second agent, passes the word to
/// its slot `0x30` (a tail jump).
pub fn hkp_bv_agent_commit_potential(e: &mut Engine, this: Ptr, param: u32) {
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x30, &args![param]);
    }
}

// Translated from 00d45330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::createZombie` (Xbox PDB): when there is a second agent, passes the word to its
/// slot `0x34` (a tail jump).
pub fn hkp_bv_agent_create_zombie(e: &mut Engine, this: Ptr, param: u32) {
    let second = e.mem.u32(this.addr() + 0x10);
    if second != 0 {
        e.vcall(second, 0x34, &args![param]);
    }
}

// Translated from 00d45350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::updateShapeCollectionFilter` (Xbox PDB): calls slot `0x20` of the first agent
/// with a body for the bounding volume (the shape's `+0x10`, key -1) and, when there is a
/// second agent, of that one with a body for the child shape (the shape's `+0x18`, key 0);
/// both bodies carry the first body's motion and the first body as parent, and both calls
/// pass the other body and the two remaining words on.
pub fn hkp_bv_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0x10, |e, body| {
        let body = body.addr();
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        let first = e.mem.u32(this.addr() + 0xc);
        e.vcall(first, 0x20, &args![body, body_b, third, fourth]);
        let second = e.mem.u32(this.addr() + 0x10);
        if second != 0 {
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            e.vcall(second, 0x20, &args![body, body_b, third, fourth]);
        }
    });
}

// Translated from 00d453d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent` constructor (the map gives this body no name): sets the vtable, a reference
/// count of 1 (`+6`) and the contact manager (`+8`), builds the body of the bounding volume
/// (the first body's shape `+0x10`, key -1, its motion, and the body as parent), makes the
/// first agent with the dispatcher's `createFunc` for the pair (bounding-volume shape type,
/// other body's shape type) (`input` `+0x14` set picks the predictive row) and stores it at
/// `+0xC`; the second agent (`+0x10`) starts empty. Returns `this`.
pub fn fn_00d453d0(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    contact_mgr: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), BV_AGENT_VTABLE);
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 8, contact_mgr);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    let first = e.with_stack(0x10, |e, body| {
        let body = body.addr();
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        let type_a = shape_type(e, bv_shape);
        let shape_b = e.mem.u32(body_b.addr());
        let type_b = shape_type(e, shape_b);
        let function = create_function(e, input.addr(), type_a, type_b);
        e.call(function, &args![body, body_b, input, contact_mgr])
            .u32()
    });
    e.mem.set_u32(this.addr() + 0xc, first);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 00d454c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::processCollision` (Xbox PDB): profiled (`"LthkpBvAgent"` / `"StcheckBvShape"`).
/// Asks the first agent (slot `0xC`) with the bounding-volume body and a hit-flag collector
/// whether the volume overlaps the other body. On a hit (record `"Stchild"`) it makes the
/// second agent, when there is none, with `createFunc` for (child shape type, other shape
/// type) and the contact manager, and runs its `processCollision` (slot `0x18`) on a body for
/// the child shape (shape `+0x18`, key 0). Without a hit it cleans a second agent up (slot
/// `0x1C`, with the output's word at `+4`) and clears it. Ends with the shared end record.
pub fn hkp_bv_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_start_record(e, LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0x18, |e, frame| {
        let body = frame.addr();
        let collector = body + 0x10;
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        init_flag_collector(e, collector);
        let first = e.mem.u32(this.addr() + 0xc);
        e.vcall(first, 0xc, &args![body, body_b, input, collector]);
        if flag_hit(e, collector) {
            profile_record(e, LABEL_CHILD);
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            if e.mem.u32(this.addr() + 0x10) == 0 {
                let type_a = shape_type(e, child_shape);
                let shape_b = e.mem.u32(body_b.addr());
                let type_b = shape_type(e, shape_b);
                let function = create_function(e, input.addr(), type_a, type_b);
                let contact_mgr = e.mem.u32(this.addr() + 8);
                let second = e
                    .call(function, &args![body, body_b, input, contact_mgr])
                    .u32();
                e.mem.set_u32(this.addr() + 0x10, second);
            }
            let second = e.mem.u32(this.addr() + 0x10);
            e.vcall(second, 0x18, &args![body, body_b, input, output]);
        } else {
            let second = e.mem.u32(this.addr() + 0x10);
            if second != 0 {
                let owner = e.mem.u32(output.addr() + 4);
                e.vcall(second, 0x1c, &args![owner]);
                e.mem.set_u32(this.addr() + 0x10, 0);
            }
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d45660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::linearCast` (Xbox PDB): profiled like `processCollision`. Casts against the
/// bounding volume through the first agent (slot `0x14`) with a closest-contact collector
/// (large float at `+4` and `+0x2C`, hit byte at `+8`) passed as both collectors. On a hit
/// it makes the second agent, when there is none, as `processCollision` does, and runs its
/// `linearCast` (slot `0x14`) for the child-shape body with the caller's two collectors.
pub fn hkp_bv_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    profile_start_record(e, LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0x40, |e, frame| {
        let body = frame.addr();
        let collector = body + 0x10;
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        init_closest_contact_collector(e, collector);
        let first = e.mem.u32(this.addr() + 0xc);
        e.vcall(
            first,
            0x14,
            &args![body, body_b, input, collector, collector],
        );
        if e.mem.u8(collector + 8) != 0 {
            profile_record(e, LABEL_CHILD);
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            if e.mem.u32(this.addr() + 0x10) == 0 {
                let shape_b = e.mem.u32(body_b.addr());
                let type_b = shape_type(e, shape_b);
                let type_a = shape_type(e, child_shape);
                let function = create_function(e, input.addr(), type_a, type_b);
                let contact_mgr = e.mem.u32(this.addr() + 8);
                let second = e
                    .call(function, &args![body, body_b, input, contact_mgr])
                    .u32();
                e.mem.set_u32(this.addr() + 0x10, second);
            }
            let second = e.mem.u32(this.addr() + 0x10);
            e.vcall(
                second,
                0x14,
                &args![body, body_b, input, cast_collector, start_collector],
            );
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d45800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static linear cast of the bounding-volume pair (the map gives it no name): as
/// `hkpBvAgent::linearCast` but without an agent. The bounding-volume check and the cast of
/// the child shape both go through the dispatcher's `linearCastFunc` (`+0x9AC`) for the
/// static matrix (types of the bounding volume and the other shape; types of the child shape
/// and the other shape).
pub fn fn_00d45800(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    profile_start_record(e, LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    let shape_b = e.mem.u32(body_b.addr());
    e.with_stack(0x40, |e, frame| {
        let body = frame.addr();
        let collector = body + 0x10;
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        let type_a = shape_type(e, bv_shape);
        let type_b = shape_type(e, shape_b);
        let function = static_function(e, input.addr(), type_a, type_b, 0x9ac);
        init_closest_contact_collector(e, collector);
        e.call(function, &args![body, body_b, input, collector, collector]);
        if e.mem.u8(collector + 8) != 0 {
            profile_record(e, LABEL_CHILD);
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            let type_a = shape_type(e, child_shape);
            let function = static_function(e, input.addr(), type_a, type_b, 0x9ac);
            e.call(
                function,
                &args![body, body_b, input, cast_collector, start_collector],
            );
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d45970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::getClosestPoints` (Xbox PDB): as `processCollision`, but the first agent is
/// asked through slot `0xC` (`getPenetrations`) and the second agent, made the same way when
/// the volume overlaps, runs `getClosestPoints` (slot `0x10`) with the caller's `output`. A
/// miss leaves the agents alone.
pub fn hkp_bv_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    bv_agent_get_points(e, this, body_a, body_b, input, output, 0x10);
}

// Translated from 00d45c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvAgent::getPenetrations` (Xbox PDB): as `getClosestPoints` but the second agent runs
/// `getPenetrations` (slot `0xC`).
pub fn hkp_bv_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    bv_agent_get_points(e, this, body_a, body_b, input, output, 0xc);
}

/// The body of `getClosestPoints` and `getPenetrations` (they differ in the slot of the
/// second call).
fn bv_agent_get_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
    second_slot: u32,
) {
    profile_start_record(e, LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0x18, |e, frame| {
        let body = frame.addr();
        let collector = body + 0x10;
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        init_flag_collector(e, collector);
        let first = e.mem.u32(this.addr() + 0xc);
        e.vcall(first, 0xc, &args![body, body_b, input, collector]);
        if flag_hit(e, collector) {
            profile_record(e, LABEL_CHILD);
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            if e.mem.u32(this.addr() + 0x10) == 0 {
                let type_a = shape_type(e, child_shape);
                let shape_b = e.mem.u32(body_b.addr());
                let type_b = shape_type(e, shape_b);
                let function = create_function(e, input.addr(), type_a, type_b);
                let contact_mgr = e.mem.u32(this.addr() + 8);
                let second = e
                    .call(function, &args![body, body_b, input, contact_mgr])
                    .u32();
                e.mem.set_u32(this.addr() + 0x10, second);
            }
            let second = e.mem.u32(this.addr() + 0x10);
            e.vcall(second, second_slot, &args![body, body_b, input, output]);
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d45af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getClosestPoints` of the bounding-volume pair (the map gives it no name): the
/// bounding-volume check calls the dispatcher's `getPenetrationsFunc` (`+0x9A4`) with a
/// hit-flag collector; on a hit (record `"Stchild"`) the child shape's body goes to
/// `getClosestPointsFunc` (`+0x9A8`) with the caller's `output`.
pub fn fn_00d45af0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    static_get_points(e, body_a, body_b, input, output, 0x9a8);
}

// Translated from 00d45dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getPenetrations` of the bounding-volume pair (the map gives it no name): as
/// `00d45af0`, but the child shape's body also goes to `getPenetrationsFunc` (`+0x9A4`).
pub fn fn_00d45dd0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    static_get_points(e, body_a, body_b, input, output, 0x9a4);
}

/// The body of the two static point queries (they differ in the slot of the second call).
fn static_get_points(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
    second_slot: u32,
) {
    profile_start_record(e, LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    let shape_b = e.mem.u32(body_b.addr());
    e.with_stack(0x18, |e, frame| {
        let body = frame.addr();
        let collector = body + 0x10;
        let bv_shape = e.mem.u32(shape + 0x10);
        set_cd_body(e, body, bv_shape, 0xffff_ffff, motion, body_a.addr());
        let type_a = shape_type(e, bv_shape);
        let type_b = shape_type(e, shape_b);
        let function = static_function(e, input.addr(), type_a, type_b, 0x9a4);
        init_flag_collector(e, collector);
        e.call(function, &args![body, body_b, input, collector]);
        if flag_hit(e, collector) {
            profile_record(e, LABEL_CHILD);
            let child_shape = e.mem.u32(shape + 0x18);
            e.mem.set_u32(body, child_shape);
            e.mem.set_u32(body + 4, 0);
            let type_a = shape_type(e, child_shape);
            let function = static_function(e, input.addr(), type_a, type_b, second_slot);
            e.call(function, &args![body, body_b, input, output]);
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d45f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the bounding-volume agent with a dispatcher (the map gives it no name; it has
/// the shape of `hkpBvAgent::registerAgent`, which is unconfirmed): two agent function
/// entries of 5 words, built on the stack and handed to `00cfbbe0` (`this` = the dispatcher,
/// `(entry, type a, type b)`). The symmetric entry (`createFunc` `00d45f30`, `00d46010`,
/// `00d46050`, `00d46090`, both flag bytes 1) is registered for the pair (all types = -1,
/// 0x1B); the direct entry (`00d45480`, `00d45dd0`, `00d45af0`, `00d45800`, flag bytes 0 and
/// 1) for (0x1B, all types = -1). The two bytes after the flags are not set by the exe (the
/// callee copies the word they are in); here they are zero.
pub fn fn_00d45f80(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (words, flag_a, flag_b, type_a, type_b) in [
            (
                [0x00d4_5f30, 0x00d4_6010, 0x00d4_6050, 0x00d4_6090],
                1u8,
                1u8,
                0xffff_ffffu32,
                0x1bu32,
            ),
            (
                [0x00d4_5480, 0x00d4_5dd0, 0x00d4_5af0, 0x00d4_5800],
                0,
                1,
                0x1b,
                0xffff_ffff,
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

/// Runs `f` with a flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's word
/// `output` at `+8`) built on the stack.
pub(crate) fn with_flip_body_collector(
    e: &mut Engine,
    output: u32,
    f: impl FnOnce(&mut Engine, u32),
) {
    e.with_stack(0xc, |e, flip| {
        let flip = flip.addr();
        e.mem.set_u32(flip, FLIP_BODY_COLLECTOR_VTABLE);
        e.mem.set_u8(flip + 4, 0);
        e.mem.set_u32(flip + 8, output);
        f(e, flip);
    });
}

/// Runs `f` with a flip collector (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at
/// `+4`, the caller's word `output` at `+8`) built on the stack.
pub(crate) fn with_flip_collector(e: &mut Engine, output: u32, f: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(0xc, |e, flip| {
        let flip = flip.addr();
        e.mem.set_u32(flip, FLIP_COLLECTOR_VTABLE);
        let large = x87_quiet_bits(e.global::<u32>(LARGE_FLOAT));
        e.mem.set_u32(flip + 4, large);
        e.mem.set_u32(flip + 8, output);
        f(e, flip);
    });
}

// Translated from 00d46010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the bounding-volume pair (the map gives it no
/// name): runs `00d45dd0` with the two bodies swapped and, in place of the caller's
/// collector word, a flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's
/// word at `+8`) built on the stack.
pub fn fn_00d46010(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        fn_00d45dd0(e, body_b, body_a, input, flip);
    });
}

// Translated from 00d46050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the bounding-volume pair (the map gives it no
/// name): runs `00d45af0` with the two bodies swapped and a flip collector (vtable
/// `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word at `+8`).
pub fn fn_00d46050(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        fn_00d45af0(e, body_b, body_a, input, flip);
    });
}

/// Builds the pair of flip-cast collectors the symmetric linear casts hand on (vtable
/// `010ce80c`, the large float through `FLD`/`FSTP` at `+4`, the input's path vector
/// (`input + 0x50`) at `+0x10` and the caller's collector at `+0x20`) and calls `cast` with
/// the swapped bodies, a copy of the input with its path vector (`+0x50`, four lanes)
/// negated, the first collector and the second (null when the caller's is). `frame` holds
/// the copy (0x70 bytes at `+0`), the first collector (0x24 at `+0x70`) and the second (0x24
/// at `+0x98`).
pub(crate) fn symmetric_linear_cast(
    e: &mut Engine,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
    cast: impl FnOnce(&mut Engine, u32, u32, u32),
) {
    e.with_stack(0xc0, |e, frame| {
        let frame = frame.addr();
        let copy = frame;
        let first = frame + 0x70;
        let second = frame + 0x98;
        let large = x87_quiet_bits(e.global::<u32>(LARGE_FLOAT));
        let path = input.addr() + 0x50;
        // The copy of the input, with the path vector negated (XORPS with `010c44d0`).
        let bytes = e.mem.bytes(input.addr(), 0x70);
        e.mem.write(copy, &bytes);
        for lane in 0..4u32 {
            let mask = e.mem.u32(NEGATE_ALL_MASK + 4 * lane);
            let value = e.mem.u32(path + 4 * lane) ^ mask;
            e.mem.set_u32(copy + 0x50 + 4 * lane, value);
        }
        // The first collector wraps the caller's cast collector.
        e.mem.set_u32(first, FLIP_CAST_COLLECTOR_VTABLE);
        e.mem.set_u32(first + 4, large);
        for lane in 0..4u32 {
            let value = e.mem.u32(path + 4 * lane);
            e.mem.set_u32(first + 0x10 + 4 * lane, value);
        }
        e.mem.set_u32(first + 0x20, cast_collector);
        let second_argument = if start_collector != 0 {
            // The second collector wraps the caller's start collector.
            e.mem.set_u32(second, FLIP_CAST_COLLECTOR_VTABLE);
            e.mem.set_u32(second + 4, large);
            for lane in 0..4u32 {
                let value = e.mem.u32(path + 4 * lane);
                e.mem.set_u32(second + 0x10 + 4 * lane, value);
            }
            e.mem.set_u32(second + 0x20, start_collector);
            second
        } else {
            0
        };
        cast(e, copy, first, second_argument);
    });
}

// Translated from 00d46090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static linear cast of the bounding-volume pair (the map gives it no name):
/// runs `00d45800` with the two bodies swapped, a copy of the input whose path vector
/// (`input + 0x50`) is negated, and flip-cast collectors (vtable `010ce80c`) wrapping the
/// caller's two collectors (`start_collector` null stays null).
pub fn fn_00d46090(
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
            fn_00d45800(e, body_b, body_a, Ptr::new(copy), first, second);
        },
    );
}

// Translated from 00d46180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvAgent>::updateShapeCollectionFilter` (Xbox PDB): calls
/// `hkpBvAgent::updateShapeCollectionFilter` with the two bodies swapped.
pub fn hkp_symmetric_agent_linear_cast_bv_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    hkp_bv_agent_update_shape_collection_filter(e, this, body_b, body_a, third, fourth);
}

// Translated from 00d461a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpBvAgent::getPenetrations` with the bodies swapped and a flip-body collector (vtable
/// `010ce81c`, byte 0 at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_bv_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_bv_agent_get_penetrations(e, this, body_b, body_a, input, flip);
    });
}

// Translated from 00d461e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpBvAgent::getClosestPoints` with the bodies swapped and a flip collector (vtable
/// `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_bv_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_bv_agent_get_closest_points(e, this, body_b, body_a, input, flip);
    });
}

// Translated from 00d46220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpBvAgent>::processCollision` (Xbox PDB): runs
/// `hkpBvAgent::processCollision` with the bodies swapped, then turns what it appended to the
/// output around. Every contact point added since (`m_firstFreeContactPoint` was `first`,
/// now `*output`; 0x30-byte `hkpProcessCdPoint`s) moves its position along its normal by the
/// normal's fourth lane (`position += normal.w * normal`) and has its normal negated (sign
/// mask `010c4c00`, three lanes). When the time of impact (`+0x3030`) differs from what it
/// was (also when it is NaN), the TOI contact normal (`+0x3020`) is negated too, the two
/// nibbles of the byte at `+0x304A` are swapped, the bytes at `+0x3048` and `+0x3049`
/// exchange, and the `u16` at `+0x3040` is exchanged with the one at index `(old +0x3049) +
/// (old +0x3048) - 1` of the array that starts there; when the two bytes' AND is then 2, the
/// words at `+0x3042` and `+0x3044` exchange.
pub fn hkp_symmetric_agent_linear_cast_bv_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let output = output.addr();
    symmetric_process_collision(e, output, |e| {
        hkp_bv_agent_process_collision(e, this, body_b, body_a, input, Ptr::new(output));
    });
}

/// The symmetric `processCollision` of the agents (`00d46220`, `00d3b260`, ...): notes the
/// output's first free contact point and its time of impact (`+0x3030`), runs `process` (the
/// agent's own `processCollision` with the bodies swapped) and turns what it appended around;
/// see `hkp_symmetric_agent_linear_cast_bv_agent_process_collision`.
pub(crate) fn symmetric_process_collision(
    e: &mut Engine,
    output: u32,
    process: impl FnOnce(&mut Engine),
) {
    let mut point = e.mem.u32(output);
    let old_time = e.mem.f32(output + 0x3030);
    process(e);
    while point < e.mem.u32(output) {
        let normal: [f32; 4] = [0, 4, 8, 12].map(|offset| e.mem.f32(point + 0x10 + offset));
        let position: [f32; 4] = [0, 4, 8, 12].map(|offset| e.mem.f32(point + offset));
        for lane in 0..4usize {
            e.mem.set_f32(
                point + 4 * lane as u32,
                normal[3] * normal[lane] + position[lane],
            );
        }
        negate_xyz(e, point + 0x10);
        point += 0x30;
    }
    let new_time = e.mem.f32(output + 0x3030);
    // FUCOMPP then `TEST AH,0x44; JNP`: the flip is skipped only when the two times are equal.
    if old_time != new_time {
        negate_xyz(e, output + 0x3020);
        flip_toi_words(e, output + 0x3040);
    }
}

/// Turns the time-of-impact words around that start at `base` (`output + 0x3040` for an
/// output): the two nibbles of the byte at `base + 0xA` swap, the bytes at `base + 8` and
/// `base + 9` exchange, the `u16` at `base` is exchanged with the one at index `(old
/// base + 9) + (old base + 8) - 1` of the array that starts there, and when the AND of the
/// two bytes is then 2 the words at `base + 2` and `base + 4` exchange.
pub(crate) fn flip_toi_words(e: &mut Engine, base: u32) {
    let packed = e.mem.u8(base + 0xa);
    let swapped = (packed >> 4) | ((packed & 0xf) << 4);
    let old_9 = e.mem.u8(base + 9);
    e.mem.set_u8(base + 0xa, swapped);
    let old_8 = e.mem.u8(base + 8);
    e.mem.set_u8(base + 9, old_8);
    e.mem.set_u8(base + 8, old_9);
    let index = u32::from(old_9)
        .wrapping_add(u32::from(old_8))
        .wrapping_sub(1);
    let other_at = base.wrapping_add(index.wrapping_mul(2));
    let first = e.mem.u16(base);
    let other = e.mem.u16(other_at);
    e.mem.set_u16(base, other);
    e.mem.set_u16(other_at, first);
    if e.mem.u8(base + 8) & e.mem.u8(base + 9) == 2 {
        let at_2 = e.mem.u16(base + 2);
        let at_4 = e.mem.u16(base + 4);
        e.mem.set_u16(base + 2, at_4);
        e.mem.set_u16(base + 4, at_2);
    }
}

/// XORPS of the vector at `at` with the mask at `010c4c00` (the sign of lanes 0 to 2).
fn negate_xyz(e: &mut Engine, at: u32) {
    for lane in 0..4u32 {
        let mask = e.mem.u32(NEGATE_XYZ_MASK + 4 * lane);
        let value = e.mem.u32(at + 4 * lane) ^ mask;
        e.mem.set_u32(at + 4 * lane, value);
    }
}

// Translated from 00d46340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpBvAgent>::linearCast` (Xbox PDB): runs `hkpBvAgent::linearCast` with
/// the bodies swapped, a copy of the input whose path vector is negated and flip-cast
/// collectors wrapping the caller's two (as `00d46090` does).
pub fn hkp_symmetric_agent_bv_agent_linear_cast(
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
            hkp_bv_agent_linear_cast(e, this, body_b, body_a, Ptr::new(copy), first, second);
        },
    );
}

/// Allocates `size` bytes the way the agent create functions do: the thread's memory router
/// (`00c85750`) has the allocator at `+0x10`; its virtual slot `4` hands out the block, whose
/// size goes into the 16-bit word at `+4`.
fn allocate_agent_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00d45480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): the
/// create function of the bounding-volume pair (code at `00d45fcd` stores its address in an
/// agent record). Allocates the 0x14-byte agent (`allocate_agent_block`) and runs the
/// `hkpBvAgent` constructor `00d453d0` on it with the four words in this order. Answers what
/// the constructor answers, the agent.
pub fn fn_00d45480(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x14);
    fn_00d453d0(e, Ptr::new(agent), body_a, body_b, input, contact_mgr).addr()
}

// Translated from 00d45f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): the
/// create function of the flipped bounding-volume agent (code at `00d45f93` stores its address
/// in an agent record). Allocates the 0x14-byte agent (`allocate_agent_block`), runs the
/// constructor `00d453d0` with the two bodies exchanged (`body_b`, `body_a`, `input`,
/// `contact_mgr`) and stores the vtable `010cf428`. Answers the agent.
pub fn fn_00d45f30(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x14);
    fn_00d453d0(e, Ptr::new(agent), body_b, body_a, input, contact_mgr);
    e.mem.set_u32(agent, 0x010c_f428);
    agent
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d45480, fn_00d45480(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00d45f30, fn_00d45f30(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00d4_5230, hkp_bv_agent_cleanup(Ptr, u32)),
        entry!(0x00d4_5270, hkp_bv_agent_invalidate_tim(Ptr, u32)),
        entry!(0x00d4_52a0, hkp_bv_agent_warp_time(Ptr, f32, f32, u32)),
        entry!(0x00d4_52f0, hkp_bv_agent_remove_point(Ptr, u32)),
        entry!(0x00d4_5310, hkp_bv_agent_commit_potential(Ptr, u32)),
        entry!(0x00d4_5330, hkp_bv_agent_create_zombie(Ptr, u32)),
        entry!(
            0x00d4_5350,
            hkp_bv_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d4_53d0, fn_00d453d0(Ptr, Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00d4_54c0,
            hkp_bv_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d4_5660,
            hkp_bv_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d4_5800, fn_00d45800(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d4_5970,
            hkp_bv_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d4_5af0, fn_00d45af0(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d4_5c50,
            hkp_bv_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d4_5dd0, fn_00d45dd0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d4_5f80, fn_00d45f80(Ptr)),
        entry!(0x00d4_6010, fn_00d46010(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d4_6050, fn_00d46050(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d4_6090, fn_00d46090(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d4_6180,
            hkp_symmetric_agent_linear_cast_bv_agent_update_shape_collection_filter(
                Ptr, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d4_61a0,
            hkp_symmetric_agent_linear_cast_bv_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d4_61e0,
            hkp_symmetric_agent_linear_cast_bv_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d4_6220,
            hkp_symmetric_agent_linear_cast_bv_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d4_6340,
            hkp_symmetric_agent_bv_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
    ]
}

/// Helpers the tests of the agent units share.
#[cfg(test)]
pub(crate) mod test_support {
    use super::{LABEL_BV_AGENT, LABEL_CHECK_BV_SHAPE};
    use crate::prelude::*;
    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The calls the doubles saw: (address, argument words).
    pub(crate) type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range mapped (the tests run without the exe).
    pub(crate) fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    pub(crate) fn new_log() -> Log {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs its arguments and answers `eax`.
    pub(crate) fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// The argument words of every logged call to `address`.
    pub(crate) fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object (0x40 bytes) whose vtable slots `0` to `0x3C` are logging doubles at
    /// `base + slot`.
    pub(crate) fn logging_object(e: &mut Engine, log: &Log, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for slot in (0..0x40).step_by(4) {
            log_double(e, log, base + slot, 0);
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The thread's record stream: TLS index 3, a cursor at the returned `records` and room
    /// for `room` bytes. Returns `(stream, records)`.
    pub(crate) fn monitor(e: &mut Engine, room: u32) -> (u32, u32) {
        e.set_global(PROFILE_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(room);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + room);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (stream, records)
    }

    /// The labels of the 12-byte profile records written since `records`.
    pub(crate) fn profile_labels(e: &Engine, stream: u32, records: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        (records..end)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    /// Checks the 16-byte start record of an agent function at `records` and returns the
    /// labels of the 12-byte records after it.
    pub(crate) fn labels_after_start(e: &Engine, stream: u32, records: u32) -> Vec<u32> {
        assert_eq!(e.mem.u32(records), LABEL_BV_AGENT);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_CHECK_BV_SHAPE);
        let end = e.mem.u32(stream + 4);
        (records + 0x10..end)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    /// A shape with `m_type` (`+0xC`), the bounding-volume child (`+0x10`) and the wrapped
    /// child (`+0x18`).
    pub(crate) fn shape(e: &mut Engine, shape_type: u32, bv: u32, child: u32) -> u32 {
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, shape_type);
        e.mem.set_u32(shape + 0x10, bv);
        e.mem.set_u32(shape + 0x18, child);
        shape
    }

    /// A body (`hkpCdBody`) with the given shape and motion.
    pub(crate) fn body(e: &mut Engine, shape: u32, motion: u32) -> u32 {
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        body
    }

    /// A dispatcher (0x1000 bytes) and an input (0x100 bytes) pointing to it: `(input,
    /// dispatcher)`.
    pub(crate) fn input(e: &mut Engine) -> (u32, u32) {
        let dispatcher = e.mem.alloc(0x1000);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        (input, dispatcher)
    }

    /// Makes the dispatcher's byte matrix give entry `index` for the shape types `types`:
    /// the byte at `dispatcher + offset + (types.0 << 5) + types.1`.
    pub(crate) fn set_matrix(
        e: &mut Engine,
        dispatcher: u32,
        offset: u32,
        types: (u32, u32),
        index: u8,
    ) {
        e.mem
            .set_u8(dispatcher + offset + (types.0 << 5) + types.1, index);
    }

    /// Sets word `slot` (0 `createFunc`, 4, 8, 0xC) of the dispatcher's entry `index`.
    pub(crate) fn set_entry(e: &mut Engine, dispatcher: u32, index: u8, slot: u32, function: u32) {
        e.mem.set_u32(
            dispatcher + 0x9a0 + u32::from(index) * 0x14 + slot,
            function,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn install_allocator(e: &mut Engine, block: u32) -> Rc<RefCell<Vec<u32>>> {
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0950_0000, &[0, 0x0950_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0950_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0950_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        sizes
    }

    #[test]
    fn fn_00d45480_allocates_the_agent_and_runs_the_constructor() {
        let mut f = fixture();
        let sizes = install_allocator(&mut f.e, 0x0800_0000);
        let agent =
            f.e.call(0x00d4_5480, &args![f.body_a, f.body_b, f.input, 0x77u32])
                .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(f.e.mem.u16(agent + 4), 0x14);
        assert_eq!(f.e.mem.u32(agent), BV_AGENT_VTABLE);
        assert_eq!(f.e.mem.u32(agent + 8), 0x77);
        let created = calls(&f.log, CREATE_BV);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [f.body_b, f.input, 0x77]);
    }

    #[test]
    fn fn_00d45f30_exchanges_the_bodies_and_stores_the_flipped_vtable() {
        let mut f = fixture();
        let sizes = install_allocator(&mut f.e, 0x0800_0000);
        // Called with the bodies the other way round, the constructor sees them in order.
        let agent =
            f.e.call(0x00d4_5f30, &args![f.body_b, f.body_a, f.input, 0x77u32])
                .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(f.e.mem.u32(agent), 0x010c_f428);
        assert_eq!(f.e.mem.u32(agent + 8), 0x77);
        let created = calls(&f.log, CREATE_BV);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [f.body_b, f.input, 0x77]);
    }

    /// The vtable doubles of the first and second agent and of `this`.
    const FIRST: u32 = 0x0900_0000;
    const SECOND: u32 = 0x0910_0000;
    const THIS: u32 = 0x0930_0000;
    /// Agent functions the dispatcher hands out (the static ones at `base + slot`).
    const CREATE_CHILD: u32 = 0x0920_0000;
    const CREATE_BV: u32 = 0x0920_0100;
    const STATIC_BV: u32 = 0x0920_0200;
    const STATIC_CHILD: u32 = 0x0920_0300;
    /// Shape types of the fixture.
    const TYPE_BV: u32 = 2;
    const TYPE_CHILD: u32 = 3;
    const TYPE_OTHER: u32 = 7;

    type Bodies = Rc<RefCell<Vec<Vec<u32>>>>;

    struct Fixture {
        e: Engine,
        log: Log,
        stream: u32,
        records: u32,
        this: u32,
        first: u32,
        second: u32,
        child_shape: u32,
        bv_shape: u32,
        body_a: u32,
        body_b: u32,
        input: u32,
        output: u32,
    }

    /// An agent (`this`, first agent at `+0xC`, contact manager 0x77 at `+8`), a body A with
    /// a bounding-volume shape (motion 0x1234), a body B, and an input whose dispatcher makes
    /// the second agent for (child, other), the first for (bounding volume, other) and
    /// answers the static queries too (the createFunc and static matrices are one table, so the
    /// same entries carry both: entry 3 for the bounding volume, entry 1 for the child).
    fn fixture() -> Fixture {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        e.mem.set_u32(LARGE_FLOAT, 0x7f7f_ffee);
        let first = logging_object(&mut e, &log, FIRST);
        let second = logging_object(&mut e, &log, SECOND);
        let this = logging_object(&mut e, &log, THIS);
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0xc, first);
        let bv_shape = shape(&mut e, TYPE_BV, 0, 0);
        let child_shape = shape(&mut e, TYPE_CHILD, 0, 0);
        let shape_a = shape(&mut e, 5, bv_shape, child_shape);
        let shape_b = shape(&mut e, TYPE_OTHER, 0, 0);
        let body_a = body(&mut e, shape_a, 0x1234);
        let body_b = body(&mut e, shape_b, 0x5678);
        let (input, dispatcher) = input(&mut e);
        log_double(&mut e, &log, CREATE_CHILD, second);
        log_double(&mut e, &log, CREATE_BV, first);
        set_matrix(&mut e, dispatcher, 0x1a0, (TYPE_CHILD, TYPE_OTHER), 1);
        set_entry(&mut e, dispatcher, 1, 0, CREATE_CHILD);
        set_matrix(&mut e, dispatcher, 0x1a0, (TYPE_BV, TYPE_OTHER), 3);
        set_entry(&mut e, dispatcher, 3, 0, CREATE_BV);
        for slot in [4, 8, 0xc] {
            log_double(&mut e, &log, STATIC_BV + slot, 0);
            log_double(&mut e, &log, STATIC_CHILD + slot, 0);
            set_entry(&mut e, dispatcher, 3, slot, STATIC_BV + slot);
            set_entry(&mut e, dispatcher, 1, slot, STATIC_CHILD + slot);
        }
        let output = e.mem.alloc(0x3100);
        e.mem.set_u32(output + 4, 0xabc0);
        Fixture {
            e,
            log,
            stream,
            records,
            this,
            first,
            second,
            child_shape,
            bv_shape,
            body_a,
            body_b,
            input,
            output,
        }
    }

    /// Registers a double at `address` that logs its arguments and, at the time of the call,
    /// copies the four words at its argument `body_index`; with `flag` = `(index, offset)`
    /// it also sets the byte at that argument plus the offset to 1 (a hit). Returns the
    /// copies.
    fn probe(
        e: &mut Engine,
        log: &Log,
        address: u32,
        body_index: usize,
        flag: Option<(usize, u32)>,
    ) -> Bodies {
        let bodies: Bodies = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&bodies);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let words = (0..4)
                .map(|word| e.mem.u32(a[body_index] + 4 * word))
                .collect::<Vec<u32>>();
            seen.borrow_mut().push(words);
            if let Some((index, offset)) = flag {
                e.mem.set_u8(a[index] + offset, 1);
            }
            Ret::default()
        });
        bodies
    }

    /// The `n` words at `at`.
    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    /// Registers a double at `address` that logs its arguments and, at the time of the
    /// call, copies the `n` words at its argument `index`.
    fn capture(e: &mut Engine, log: &Log, address: u32, index: usize, n: u32) -> Bodies {
        let seen: Bodies = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            copies.borrow_mut().push(words(e, a[index], n));
            Ret::default()
        });
        seen
    }

    #[test]
    fn cleanup_cleans_both_agents_and_deletes_the_agent() {
        let mut f = fixture();
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(0x00d4_5230, &args![f.this, 0x44u32]);
        assert_eq!(calls(&f.log, FIRST + 0x1c), vec![vec![f.first, 0x44]]);
        assert_eq!(calls(&f.log, SECOND + 0x1c), vec![vec![f.second, 0x44]]);
        assert_eq!(f.e.mem.u32(f.this + 0x10), 0);
        assert_eq!(calls(&f.log, THIS), vec![vec![f.this, 1]]);
    }

    #[test]
    fn cleanup_without_a_second_agent() {
        let mut f = fixture();
        f.e.call(0x00d4_5230, &args![f.this, 0x44u32]);
        assert_eq!(calls(&f.log, FIRST + 0x1c).len(), 1);
        assert!(calls(&f.log, SECOND + 0x1c).is_empty());
        assert_eq!(calls(&f.log, THIS).len(), 1);
    }

    #[test]
    fn invalidate_tim_reaches_both_agents() {
        let mut f = fixture();
        f.e.call(0x00d4_5270, &args![f.this, 9u32]);
        assert_eq!(calls(&f.log, FIRST + 0x24), vec![vec![f.first, 9]]);
        assert!(calls(&f.log, SECOND + 0x24).is_empty());
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(0x00d4_5270, &args![f.this, 9u32]);
        assert_eq!(calls(&f.log, SECOND + 0x24), vec![vec![f.second, 9]]);
    }

    #[test]
    fn warp_time_passes_the_floats_quieted() {
        let mut f = fixture();
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(
            0x00d4_52a0,
            &args![f.this, f32::from_bits(0x7f80_0001), 2.5f32, 0x66u32],
        );
        for (base, agent) in [(FIRST, f.first), (SECOND, f.second)] {
            assert_eq!(
                calls(&f.log, base + 0x28),
                vec![vec![agent, 0x7fc0_0001, 2.5f32.to_bits(), 0x66]]
            );
        }
    }

    #[test]
    fn point_forwarders_need_a_second_agent() {
        for (address, slot) in [
            (0x00d4_52f0, 0x2c),
            (0x00d4_5310, 0x30),
            (0x00d4_5330, 0x34),
        ] {
            let mut f = fixture();
            f.e.call(address, &args![f.this, 5u32]);
            assert!(calls(&f.log, SECOND + slot).is_empty());
            assert!(calls(&f.log, FIRST + slot).is_empty());
            f.e.mem.set_u32(f.this + 0x10, f.second);
            f.e.call(address, &args![f.this, 5u32]);
            assert_eq!(calls(&f.log, SECOND + slot), vec![vec![f.second, 5]]);
            assert!(calls(&f.log, FIRST + slot).is_empty());
        }
    }

    #[test]
    fn update_shape_collection_filter_builds_both_bodies() {
        let mut f = fixture();
        let first_body = probe(&mut f.e, &f.log, FIRST + 0x20, 1, None);
        let second_body = probe(&mut f.e, &f.log, SECOND + 0x20, 1, None);
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(0x00d4_5350, &args![f.this, f.body_a, f.body_b, 3u32, 4u32]);
        assert_eq!(
            *first_body.borrow(),
            vec![vec![f.bv_shape, 0xffff_ffff, 0x1234, f.body_a]]
        );
        assert_eq!(
            *second_body.borrow(),
            vec![vec![f.child_shape, 0, 0x1234, f.body_a]]
        );
        assert_eq!(calls(&f.log, FIRST + 0x20)[0][2..], [f.body_b, 3, 4]);
        assert_eq!(calls(&f.log, SECOND + 0x20)[0][2..], [f.body_b, 3, 4]);
    }

    #[test]
    fn update_shape_collection_filter_without_a_second_agent() {
        let mut f = fixture();
        f.e.call(0x00d4_5350, &args![f.this, f.body_a, f.body_b, 3u32, 4u32]);
        assert_eq!(calls(&f.log, FIRST + 0x20).len(), 1);
        assert!(calls(&f.log, SECOND + 0x20).is_empty());
    }

    #[test]
    fn constructor_makes_the_first_agent_for_the_bounding_volume() {
        let mut f = fixture();
        let bodies = probe(&mut f.e, &f.log, CREATE_BV, 0, None);
        let agent = f.e.mem.alloc(0x14);
        f.e.mem.set_u32(agent + 0x10, 0x99);
        let result = f.e.call(
            0x00d4_53d0,
            &args![agent, f.body_a, f.body_b, f.input, 0x77u32],
        );
        assert_eq!(result.u32(), agent);
        assert_eq!(f.e.mem.u32(agent), BV_AGENT_VTABLE);
        assert_eq!(f.e.mem.u16(agent + 6), 1);
        assert_eq!(f.e.mem.u32(agent + 8), 0x77);
        // The double answered 0 (the probe does not return the agent), so the slot is 0.
        assert_eq!(f.e.mem.u32(agent + 0xc), 0);
        assert_eq!(f.e.mem.u32(agent + 0x10), 0);
        assert_eq!(
            *bodies.borrow(),
            vec![vec![f.bv_shape, 0xffff_ffff, 0x1234, f.body_a]]
        );
        let created = calls(&f.log, CREATE_BV);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [f.body_b, f.input, 0x77]);
    }

    #[test]
    fn constructor_stores_the_created_agent_and_uses_the_predictive_row() {
        let mut f = fixture();
        let dispatcher = f.e.mem.u32(f.input);
        set_matrix(&mut f.e, dispatcher, 0x5a0, (TYPE_BV, TYPE_OTHER), 6);
        set_entry(&mut f.e, dispatcher, 6, 0, 0x0940_0000);
        log_double(&mut f.e, &f.log, 0x0940_0000, 0x5150);
        f.e.mem.set_u32(f.input + 0x14, 1);
        let agent = f.e.mem.alloc(0x14);
        f.e.call(
            0x00d4_53d0,
            &args![agent, f.body_a, f.body_b, f.input, 0x77u32],
        );
        assert_eq!(f.e.mem.u32(agent + 0xc), 0x5150);
        assert_eq!(calls(&f.log, 0x0940_0000).len(), 1);
        assert!(calls(&f.log, CREATE_BV).is_empty());
    }

    #[test]
    fn process_collision_creates_the_second_agent_on_a_hit() {
        let mut f = fixture();
        let check = probe(&mut f.e, &f.log, FIRST + 0xc, 1, Some((4, 4)));
        let bodies = probe(&mut f.e, &f.log, SECOND + 0x18, 1, None);
        f.e.call(
            0x00d4_54c0,
            &args![f.this, f.body_a, f.body_b, f.input, f.output],
        );
        assert_eq!(
            *check.borrow(),
            vec![vec![f.bv_shape, 0xffff_ffff, 0x1234, f.body_a]]
        );
        let flag_check = calls(&f.log, FIRST + 0xc);
        assert_eq!(flag_check[0][2..4], [f.body_b, f.input]);
        assert_eq!(f.e.mem.u32(f.this + 0x10), f.second);
        let created = calls(&f.log, CREATE_CHILD);
        assert_eq!(created[0][1..], [f.body_b, f.input, 0x77]);
        assert_eq!(
            *bodies.borrow(),
            vec![vec![f.child_shape, 0, 0x1234, f.body_a]]
        );
        assert_eq!(
            calls(&f.log, SECOND + 0x18)[0][0..],
            [f.second, created[0][0], f.body_b, f.input, f.output]
        );
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_CHILD, LABEL_END]
        );
    }

    #[test]
    fn process_collision_reuses_the_second_agent() {
        let mut f = fixture();
        probe(&mut f.e, &f.log, FIRST + 0xc, 1, Some((4, 4)));
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(
            0x00d4_54c0,
            &args![f.this, f.body_a, f.body_b, f.input, f.output],
        );
        assert!(calls(&f.log, CREATE_CHILD).is_empty());
        let process = calls(&f.log, SECOND + 0x18);
        assert_eq!(process.len(), 1);
        assert_eq!(process[0][0], f.second);
        assert_eq!(process[0][2..], [f.body_b, f.input, f.output]);
    }

    #[test]
    fn process_collision_without_a_hit_cleans_up_the_second_agent() {
        let mut f = fixture();
        f.e.mem.set_u32(f.this + 0x10, f.second);
        f.e.call(
            0x00d4_54c0,
            &args![f.this, f.body_a, f.body_b, f.input, f.output],
        );
        assert_eq!(calls(&f.log, SECOND + 0x1c), vec![vec![f.second, 0xabc0]]);
        assert_eq!(f.e.mem.u32(f.this + 0x10), 0);
        assert!(calls(&f.log, SECOND + 0x18).is_empty());
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_END]
        );
        // Without a second agent there is nothing to clean up.
        f.e.call(
            0x00d4_54c0,
            &args![f.this, f.body_a, f.body_b, f.input, f.output],
        );
        assert_eq!(calls(&f.log, SECOND + 0x1c).len(), 1);
    }

    #[test]
    fn closest_points_and_penetrations_run_the_second_agent_on_a_hit() {
        for (address, second_slot) in [(0x00d4_5970, 0x10), (0x00d4_5c50, 0xc)] {
            let mut f = fixture();
            probe(&mut f.e, &f.log, FIRST + 0xc, 1, Some((4, 4)));
            let bodies = probe(&mut f.e, &f.log, SECOND + second_slot, 1, None);
            f.e.call(
                address,
                &args![f.this, f.body_a, f.body_b, f.input, 0x4321u32],
            );
            assert_eq!(f.e.mem.u32(f.this + 0x10), f.second);
            assert_eq!(
                *bodies.borrow(),
                vec![vec![f.child_shape, 0, 0x1234, f.body_a]]
            );
            let seen = calls(&f.log, SECOND + second_slot);
            assert_eq!(seen[0][2..], [f.body_b, f.input, 0x4321]);
            assert_eq!(
                labels_after_start(&f.e, f.stream, f.records),
                vec![LABEL_CHILD, LABEL_END]
            );
        }
    }

    #[test]
    fn closest_points_and_penetrations_do_nothing_more_on_a_miss() {
        for address in [0x00d4_5970, 0x00d4_5c50] {
            let mut f = fixture();
            f.e.mem.set_u32(f.this + 0x10, f.second);
            f.e.call(
                address,
                &args![f.this, f.body_a, f.body_b, f.input, 0x4321u32],
            );
            for slot in [0xc, 0x10, 0x1c] {
                assert!(calls(&f.log, SECOND + slot).is_empty());
            }
            assert_eq!(f.e.mem.u32(f.this + 0x10), f.second);
            assert_eq!(
                labels_after_start(&f.e, f.stream, f.records),
                vec![LABEL_END]
            );
        }
    }

    /// Runs the agent linear cast (`address`, `this` first) with a first agent that raises
    /// the hit byte (`+8`) of the collector when `hit`. Returns the fixture and what the
    /// first agent saw: its argument words and the collector's words at `+0`, `+4`, `+8` and
    /// `+0x2C`.
    fn linear_cast(hit: bool) -> (Fixture, Vec<Vec<u32>>) {
        let mut f = fixture();
        let seen: Bodies = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        let log = Rc::clone(&f.log);
        f.e.register_double(FIRST + 0x14, move |e, a| {
            log.borrow_mut().push((FIRST + 0x14, a.to_vec()));
            copies.borrow_mut().push(words(e, a[4], 12));
            if hit {
                e.mem.set_u8(a[4] + 8, 1);
            }
            Ret::default()
        });
        f.e.call(
            0x00d4_5660,
            &args![f.this, f.body_a, f.body_b, f.input, 0xc1u32, 0xc2u32],
        );
        let copies = seen.borrow().clone();
        (f, copies)
    }

    #[test]
    fn linear_cast_checks_the_bounding_volume_with_a_closest_contact_collector() {
        let (f, _) = linear_cast(false);
        let check = calls(&f.log, FIRST + 0x14);
        assert_eq!(check.len(), 1);
        assert_eq!(check[0][4], check[0][5]);
        assert_eq!(check[0][2..4], [f.body_b, f.input]);
        // The miss ends there.
        assert!(calls(&f.log, CREATE_CHILD).is_empty());
        assert_eq!(f.e.mem.u32(f.this + 0x10), 0);
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_END]
        );
    }

    #[test]
    fn linear_cast_collector_is_built_with_the_large_float() {
        let (_, copies) = linear_cast(false);
        // The words at +0 .. +0x30 of the collector when the first agent ran.
        assert_eq!(copies[0][0], CLOSEST_CONTACT_COLLECTOR_VTABLE);
        assert_eq!(copies[0][1], 0x7f7f_ffee);
        assert_eq!(copies[0][2] & 0xff, 0);
        assert_eq!(copies[0][11], 0x7f7f_ffee);
    }

    #[test]
    fn linear_cast_hit_makes_the_second_agent_and_casts_with_the_callers_collectors() {
        let (f, _) = linear_cast(true);
        assert_eq!(f.e.mem.u32(f.this + 0x10), f.second);
        let created = calls(&f.log, CREATE_CHILD);
        assert_eq!(created[0][1..], [f.body_b, f.input, 0x77]);
        let cast = calls(&f.log, SECOND + 0x14);
        assert_eq!(cast.len(), 1);
        assert_eq!(
            cast[0][0..],
            [f.second, created[0][0], f.body_b, f.input, 0xc1, 0xc2]
        );
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_CHILD, LABEL_END]
        );
    }

    #[test]
    fn static_linear_cast_goes_through_the_static_table() {
        let mut f = fixture();
        let bv = probe(&mut f.e, &f.log, STATIC_BV + 0xc, 0, Some((3, 8)));
        let child = probe(&mut f.e, &f.log, STATIC_CHILD + 0xc, 0, None);
        f.e.call(
            0x00d4_5800,
            &args![f.body_a, f.body_b, f.input, 0xc1u32, 0xc2u32],
        );
        assert_eq!(
            *bv.borrow(),
            vec![vec![f.bv_shape, 0xffff_ffff, 0x1234, f.body_a]]
        );
        let check = calls(&f.log, STATIC_BV + 0xc);
        assert_eq!(check[0][1..3], [f.body_b, f.input]);
        assert_eq!(check[0][3], check[0][4]);
        assert_eq!(
            *child.borrow(),
            vec![vec![f.child_shape, 0, 0x1234, f.body_a]]
        );
        assert_eq!(
            calls(&f.log, STATIC_CHILD + 0xc)[0][1..],
            [f.body_b, f.input, 0xc1, 0xc2]
        );
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_CHILD, LABEL_END]
        );
    }

    #[test]
    fn static_linear_cast_without_a_hit_stops_after_the_check() {
        let mut f = fixture();
        f.e.call(
            0x00d4_5800,
            &args![f.body_a, f.body_b, f.input, 0xc1u32, 0xc2u32],
        );
        assert!(calls(&f.log, STATIC_CHILD + 0xc).is_empty());
        assert_eq!(calls(&f.log, STATIC_BV + 0xc).len(), 1);
        assert_eq!(
            labels_after_start(&f.e, f.stream, f.records),
            vec![LABEL_END]
        );
    }

    #[test]
    fn static_point_queries_use_their_table_slots() {
        for (address, second_slot) in [(0x00d4_5af0, 8), (0x00d4_5dd0, 4)] {
            let mut f = fixture();
            probe(&mut f.e, &f.log, STATIC_BV + 4, 0, Some((3, 4)));
            let child = probe(&mut f.e, &f.log, STATIC_CHILD + second_slot, 0, None);
            f.e.call(address, &args![f.body_a, f.body_b, f.input, 0x4321u32]);
            assert_eq!(
                *child.borrow(),
                vec![vec![f.child_shape, 0, 0x1234, f.body_a]]
            );
            assert_eq!(
                calls(&f.log, STATIC_CHILD + second_slot)[0][1..],
                [f.body_b, f.input, 0x4321]
            );
            assert_eq!(
                labels_after_start(&f.e, f.stream, f.records),
                vec![LABEL_CHILD, LABEL_END]
            );
        }
    }

    #[test]
    fn static_point_queries_without_a_hit_stop_after_the_check() {
        for address in [0x00d4_5af0, 0x00d4_5dd0] {
            let mut f = fixture();
            f.e.call(address, &args![f.body_a, f.body_b, f.input, 0x4321u32]);
            assert_eq!(calls(&f.log, STATIC_BV + 4).len(), 1);
            assert!(calls(&f.log, STATIC_CHILD + 4).is_empty());
            assert!(calls(&f.log, STATIC_CHILD + 8).is_empty());
        }
    }

    #[test]
    fn registration_hands_the_dispatcher_both_entries() {
        let mut e = engine();
        let seen: Bodies = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut entry = words(e, a[1], 4);
            entry.push(u32::from(e.mem.u8(a[1] + 0x10)));
            entry.push(u32::from(e.mem.u8(a[1] + 0x11)));
            entry.extend_from_slice(a);
            copies.borrow_mut().push(entry);
            Ret::default()
        });
        e.call(0x00d4_5f80, &args![0x0777_0000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen[0][..6],
            [0x00d4_5f30, 0x00d4_6010, 0x00d4_6050, 0x00d4_6090, 1, 1]
        );
        assert_eq!(seen[0][6..], [0x0777_0000, seen[0][7], 0xffff_ffff, 0x1b]);
        assert_eq!(
            seen[1][..6],
            [0x00d4_5480, 0x00d4_5dd0, 0x00d4_5af0, 0x00d4_5800, 0, 1]
        );
        assert_eq!(seen[1][6..], [0x0777_0000, seen[1][7], 0x1b, 0xffff_ffff]);
    }

    #[test]
    fn symmetric_get_penetrations_swaps_the_bodies_and_flips_the_collector() {
        let mut f = fixture();
        probe(&mut f.e, &f.log, STATIC_BV + 4, 0, Some((3, 4)));
        let flip = capture(&mut f.e, &f.log, STATIC_CHILD + 4, 3, 3);
        f.e.call(0x00d4_6010, &args![f.body_b, f.body_a, f.input, 0x5151u32]);
        assert_eq!(calls(&f.log, STATIC_BV + 4)[0][1], f.body_b);
        assert_eq!(
            *flip.borrow(),
            vec![vec![FLIP_BODY_COLLECTOR_VTABLE, 0, 0x5151]]
        );
    }

    #[test]
    fn symmetric_get_closest_points_swaps_the_bodies_and_flips_the_collector() {
        let mut f = fixture();
        f.e.mem.set_u32(LARGE_FLOAT, 0x7f80_0001);
        probe(&mut f.e, &f.log, STATIC_BV + 4, 0, Some((3, 4)));
        let flip = capture(&mut f.e, &f.log, STATIC_CHILD + 8, 3, 3);
        f.e.call(0x00d4_6050, &args![f.body_b, f.body_a, f.input, 0x5151u32]);
        assert_eq!(calls(&f.log, STATIC_BV + 4)[0][1], f.body_b);
        assert_eq!(
            *flip.borrow(),
            vec![vec![FLIP_COLLECTOR_VTABLE, 0x7fc0_0001, 0x5151]]
        );
    }

    /// A path vector (1, 2, 3, 4) at `input + 0x50`, the sign masks in memory and a marker
    /// word at the end of the 0x70-byte input.
    fn prepare_path(f: &mut Fixture) {
        for lane in 0..4u32 {
            f.e.mem
                .set_f32(f.input + 0x50 + 4 * lane, (lane + 1) as f32);
            f.e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
        }
        f.e.mem.set_u32(f.input + 0x6c, 0x6c6c);
    }

    /// What the flip-cast collector at a call's argument `index` held: vtable, large float,
    /// the path vector (4 words) and the wrapped collector.
    fn flip_cast(vtable: u32, wrapped: u32) -> Vec<u32> {
        let mut expected = vec![vtable, 0x7f7f_ffee];
        expected.extend([1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits));
        expected.push(wrapped);
        expected
    }

    /// The words of the flip-cast collector the cast function received.
    fn collector_words(e: &Engine, collector: u32) -> Vec<u32> {
        let mut out = words(e, collector, 2);
        out.extend(words(e, collector + 0x10, 5));
        out
    }

    #[test]
    fn symmetric_static_linear_cast_negates_the_path_and_wraps_both_collectors() {
        for start in [0xc2u32, 0] {
            let mut f = fixture();
            prepare_path(&mut f);
            probe(&mut f.e, &f.log, STATIC_BV + 0xc, 0, Some((3, 8)));
            let seen = Rc::new(RefCell::new(Vec::new()));
            let copies = Rc::clone(&seen);
            let log = Rc::clone(&f.log);
            f.e.register_double(STATIC_CHILD + 0xc, move |e, a| {
                log.borrow_mut().push((STATIC_CHILD + 0xc, a.to_vec()));
                let second = if a[4] == 0 {
                    Vec::new()
                } else {
                    collector_words(e, a[4])
                };
                copies.borrow_mut().push((
                    words(e, a[2] + 0x50, 4),
                    words(e, a[2] + 0x6c, 1),
                    collector_words(e, a[3]),
                    second,
                ));
                Ret::default()
            });
            f.e.call(
                0x00d4_6090,
                &args![f.body_b, f.body_a, f.input, 0xc1u32, start],
            );
            let call = &calls(&f.log, STATIC_CHILD + 0xc)[0];
            assert_eq!(call[1..3], [f.body_b, call[2]]);
            assert_ne!(call[2], f.input, "the input is copied");
            let seen: Vec<(Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>)> = seen.borrow().clone();
            let negated = [-1.0f32, -2.0, -3.0, -4.0].map(f32::to_bits).to_vec();
            assert_eq!(seen[0].0, negated);
            assert_eq!(seen[0].1, vec![0x6c6c]);
            assert_eq!(seen[0].2, flip_cast(FLIP_CAST_COLLECTOR_VTABLE, 0xc1));
            if start == 0 {
                assert_eq!(call[4], 0);
            } else {
                assert_eq!(seen[0].3, flip_cast(FLIP_CAST_COLLECTOR_VTABLE, start));
            }
        }
    }

    #[test]
    fn symmetric_agent_linear_cast_runs_the_agent_with_the_swapped_bodies() {
        let mut f = fixture();
        prepare_path(&mut f);
        let seen = capture(&mut f.e, &f.log, FIRST + 0x14, 3, 0x1c);
        f.e.call(
            0x00d4_6340,
            &args![f.this, f.body_b, f.body_a, f.input, 0xc1u32, 0xc2u32],
        );
        let check = &calls(&f.log, FIRST + 0x14)[0];
        assert_eq!(check[0..3], [f.first, check[1], f.body_b]);
        assert_ne!(check[3], f.input);
        let seen = seen.borrow();
        assert_eq!(
            seen[0][0x14..0x18],
            [-1.0f32, -2.0, -3.0, -4.0].map(f32::to_bits)
        );
        assert_eq!(seen[0][0x1b], 0x6c6c);
        // A miss: the agent function was asked once, with the first collector.
        assert_eq!(check[4], check[5]);
    }

    #[test]
    fn symmetric_update_shape_collection_filter_swaps_the_bodies() {
        let mut f = fixture();
        f.e.call(0x00d4_6180, &args![f.this, f.body_b, f.body_a, 3u32, 4u32]);
        assert_eq!(calls(&f.log, FIRST + 0x20)[0][2..], [f.body_b, 3, 4]);
    }

    #[test]
    fn symmetric_get_penetrations_and_closest_points_flip_the_collector() {
        for (address, vtable, second_slot) in [
            (0x00d4_61a0, FLIP_BODY_COLLECTOR_VTABLE, 0xc),
            (0x00d4_61e0, FLIP_COLLECTOR_VTABLE, 0x10),
        ] {
            let mut f = fixture();
            probe(&mut f.e, &f.log, FIRST + 0xc, 1, Some((4, 4)));
            let flip = capture(&mut f.e, &f.log, SECOND + second_slot, 4, 3);
            f.e.call(
                address,
                &args![f.this, f.body_b, f.body_a, f.input, 0x5151u32],
            );
            let large = 0x7f7f_ffee;
            let word = if vtable == FLIP_COLLECTOR_VTABLE {
                large
            } else {
                0
            };
            assert_eq!(*flip.borrow(), vec![vec![vtable, word, 0x5151]]);
        }
    }

    /// Runs the symmetric process collision with the second agent present and a
    /// `processCollision` that appends two contacts to the output and sets the time of
    /// impact to `new_time`. Before the call the output holds time `old_time`, the TOI
    /// normal (1, 2, 3, 4) at `+0x3020`, the bytes `toi_bytes` at `+0x3048` and the four words
    /// 10 to 13 at `+0x3040`.
    fn symmetric_process(old_time: f32, new_time: f32, toi_bytes: [u8; 3]) -> Fixture {
        let mut f = fixture();
        let contacts = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(f.output, contacts);
        f.e.mem.set_f32(f.output + 0x3030, old_time);
        for lane in 0..4u32 {
            f.e.mem
                .set_f32(f.output + 0x3020 + 4 * lane, (lane + 1) as f32);
        }
        for (index, byte) in toi_bytes.into_iter().enumerate() {
            f.e.mem.set_u8(f.output + 0x3048 + index as u32, byte);
        }
        for index in 0..4u32 {
            f.e.mem
                .set_u16(f.output + 0x3040 + 2 * index, 10 + index as u16);
        }
        for lane in 0..3u32 {
            f.e.mem.set_u32(0x010c_4c00 + 4 * lane, 0x8000_0000);
        }
        f.e.mem.set_u32(f.this + 0x10, f.second);
        probe(&mut f.e, &f.log, FIRST + 0xc, 1, Some((4, 4)));
        let log = Rc::clone(&f.log);
        f.e.register_double(SECOND + 0x18, move |e, a| {
            log.borrow_mut().push((SECOND + 0x18, a.to_vec()));
            let output = a[4];
            let end = e.mem.u32(output);
            for index in 0..2u32 {
                let point = end + 0x30 * index;
                for (lane, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                    e.mem.set_f32(point + 4 * lane as u32, v + index as f32);
                }
                for (lane, v) in [0.5f32, 0.0, 1.0, 2.0].into_iter().enumerate() {
                    e.mem.set_f32(point + 0x10 + 4 * lane as u32, v);
                }
            }
            e.mem.set_u32(output, end + 0x60);
            e.mem.set_f32(output + 0x3030, new_time);
            Ret::default()
        });
        f.e.call(
            0x00d4_6220,
            &args![f.this, f.body_b, f.body_a, f.input, f.output],
        );
        f
    }

    /// The words (`u16`) at `+0x3040` of the output.
    fn toi_words(f: &Fixture) -> Vec<u16> {
        (0..4)
            .map(|index| f.e.mem.u16(f.output + 0x3040 + 2 * index))
            .collect()
    }

    /// The TOI normal at `+0x3020` of the output, as bits.
    fn toi_normal(f: &Fixture) -> Vec<u32> {
        words(&f.e, f.output + 0x3020, 4)
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contacts() {
        let f = symmetric_process(0.5, 0.5, [1, 2, 0x12]);
        let contacts = f.e.mem.u32(f.output) - 0x60;
        for index in 0..2u32 {
            let point = contacts + 0x30 * index;
            // position += normal.w * normal (every lane); the normal is negated in lanes 0 to 2.
            let position: Vec<f32> = (0..4).map(|lane| f.e.mem.f32(point + 4 * lane)).collect();
            let base = 1.0 + index as f32;
            assert_eq!(
                position,
                vec![
                    base + 1.0,
                    base + 1.0 + 0.0,
                    base + 2.0 + 2.0,
                    base + 3.0 + 4.0
                ]
            );
            assert_eq!(
                words(&f.e, point + 0x10, 4),
                vec![
                    (-0.5f32).to_bits(),
                    0x8000_0000,
                    (-1.0f32).to_bits(),
                    2.0f32.to_bits()
                ]
            );
        }
        // The bytes of the first contact set do not count here: the time did not change.
        assert_eq!(toi_normal(&f), [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits));
        assert_eq!(f.e.mem.u8(f.output + 0x304a), 0x12);
        assert_eq!(toi_words(&f), vec![10, 11, 12, 13]);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes() {
        let f = symmetric_process(0.5, 0.25, [1, 2, 0x12]);
        assert_eq!(toi_normal(&f), [-1.0f32, -2.0, -3.0, 4.0].map(f32::to_bits));
        assert_eq!(f.e.mem.u8(f.output + 0x304a), 0x21);
        assert_eq!(f.e.mem.u8(f.output + 0x3049), 1);
        assert_eq!(f.e.mem.u8(f.output + 0x3048), 2);
        // Index 2 + 1 - 1 = 2; the bytes' AND is 0, so words 1 and 2 stay.
        assert_eq!(toi_words(&f), vec![12, 11, 10, 13]);
    }

    #[test]
    fn symmetric_process_collision_swaps_the_middle_words_when_both_bytes_are_two() {
        let f = symmetric_process(0.5, 0.25, [2, 2, 0x34]);
        assert_eq!(f.e.mem.u8(f.output + 0x304a), 0x43);
        // Index 2 + 2 - 1 = 3, then words 1 and 2 exchange.
        assert_eq!(toi_words(&f), vec![13, 12, 11, 10]);
    }

    #[test]
    fn symmetric_process_collision_treats_a_nan_time_as_changed() {
        let f = symmetric_process(f32::NAN, f32::NAN, [1, 2, 0x12]);
        assert_eq!(toi_normal(&f), [-1.0f32, -2.0, -3.0, 4.0].map(f32::to_bits));
    }
}
