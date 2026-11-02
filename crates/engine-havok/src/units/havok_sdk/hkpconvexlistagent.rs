//! `Havok SDK/hkpconvexlistagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use std::cmp::Ordering;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// `hkpIterativeLinearCastAgent::staticLinearCast` (Xbox PDB, `00daf8e0`, cdecl `(p0, p1, p2,
/// collector, collector)`).
const ITERATIVE_LINEAR_CAST: u32 = 0x00da_f8e0;
/// `00d43b90` (cdecl, five words): the cast `00d34810` forwards to.
const CAST_FORWARD: u32 = 0x00d4_3b90;
/// Timer labels: the start pair, the second-stage label and the end label.
const LABEL_LINEAR_CAST: u32 = 0x010c_ec54;
const LABEL_LINEAR_CAST_DETAIL: u32 = 0x010c_ec34;
const LABEL_CONVEX_CAST: u32 = 0x010c_ec4c;
const LABEL_END: u32 = 0x0107_6f38;
/// The vtable of the stack collector the first stage builds (`010cebe0`) and the one the
/// forwarding stage builds (`010ce80c`).
const FIRST_COLLECTOR_VTABLE: u32 = 0x010c_ebe0;
const CAST_COLLECTOR_VTABLE: u32 = 0x010c_e80c;
/// The `float` both collectors start with (the bits `0x7f7fffee`).
const FAR_DISTANCE: u32 = 0x0104_da48;
/// The sign mask (`010c44d0`, four lanes) that `XORPS` negates a vector with.
const SIGN_MASK: u32 = 0x010c_44d0;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The thread's monitor stream (`TlsGetValue` of the index at `0126835c`).
fn monitor_stream(e: &mut Engine) -> u32 {
    let index: u32 = e.global(MONITOR_SLOT);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

/// Appends a 12-byte timer record `(label, time stamp)` while the stream's cursor (+4) is
/// below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

/// Appends a 16-byte start record: `first` at +0, the time stamp at +4 and `second` at +0xC.
fn monitor_start_record(e: &mut Engine, first: u32, second: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, first);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The `float` constant `FLD`/`FSTP` copied: a signalling NaN comes back quiet.
fn far_distance_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.mem.u32(FAR_DISTANCE))
}

// Translated from 00d34810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cast the convex-list agent forwards to `00d43b90`: copies the 0x70-byte cast input
/// (`input`) with the vector at +0x50 negated lane by lane, builds one collector on the
/// stack (the vtable `010ce80c`, the far distance, a copy of the input vector at +0x10 and
/// `cast_collector` at +0x20) and, when `start_collector` is non-zero, a second one of the same
/// shape holding `start_collector`; then calls `00d43b90(second, first, copy, collector,
/// second collector or 0)`. The parameter roles are described from their use.
pub fn fn_00d34810(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    // The exe's frame, 0xD8 bytes: the first collector at +8, the second at +0x38 and the
    // input copy at +0x68.
    e.with_stack(0xd8, |e, frame| {
        let frame = frame.addr();
        let far = far_distance_bits(e);
        e.mem.set_u32(frame + 0xc, far);
        let input_bytes = e.mem.bytes(input.addr(), 0x70);
        e.mem.write(frame + 0x68, &input_bytes);
        for lane in 0..4u32 {
            let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
            e.mem.set_u32(frame + 0x18 + 4 * lane, word);
        }
        e.mem.set_u32(frame + 0x28, cast_collector);
        for lane in 0..4u32 {
            let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
            let mask = e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(frame + 0xb8 + 4 * lane, word ^ mask);
        }
        e.mem.set_u32(frame + 8, CAST_COLLECTOR_VTABLE);
        let second_collector = if start_collector != 0 {
            e.mem.set_u32(frame + 0x38, CAST_COLLECTOR_VTABLE);
            e.mem.set_u32(frame + 0x3c, far);
            for lane in 0..4u32 {
                let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
                e.mem.set_u32(frame + 0x48 + 4 * lane, word);
            }
            e.mem.set_u32(frame + 0x58, start_collector);
            frame + 0x38
        } else {
            0
        };
        e.call(
            CAST_FORWARD,
            &args![second, first, frame + 0x68, frame + 8, second_collector],
        );
    });
}

// Translated from 00d33ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The timed linear cast of the convex-list agent (the exe has a `linearCast` virtual that
/// forwards here): starts a timer record, builds a collector on the stack (the vtable
/// `010cebe0`, the far distance at +4 and +0x2C, a flag byte at +8 cleared) and runs the
/// iterative cast `00daf8e0(first, second, input, collector, collector)` on it; when the
/// collector's flag was set, a second timer record is written and the cast is forwarded
/// with `00d34810`. A closing record ends it. The parameter roles are described from their
/// use.
pub fn fn_00d33ca0(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    monitor_start_record(e, LABEL_LINEAR_CAST, LABEL_LINEAR_CAST_DETAIL);
    // The exe's frame: the collector at +0x10 (its field at +0x2C is the frame's +0x3C).
    e.with_stack(0x40, |e, frame| {
        let collector = frame.addr() + 0x10;
        let far = far_distance_bits(e);
        e.mem.set_u32(collector + 0x2c, far);
        e.mem.set_u32(collector + 4, far);
        e.mem.set_u32(collector, FIRST_COLLECTOR_VTABLE);
        e.mem.set_u8(collector + 8, 0);
        e.call(
            ITERATIVE_LINEAR_CAST,
            &args![first, second, input, collector, collector],
        );
        if e.mem.u8(collector + 8) != 0 {
            monitor_record(e, LABEL_CONVEX_CAST);
            fn_00d34810(e, first, second, input, cast_collector, start_collector);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d33da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::linearCast` (Xbox PDB): forwards its five words to `00d33ca0`.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_convex_list_agent_linear_cast(
    e: &mut Engine,
    _this: u32,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    fn_00d33ca0(e, first, second, input, cast_collector, start_collector);
}

/// The memory router (`00c85750`, no arguments): the word at +0x10 of the object it returns is
/// the allocator whose vtable slot +4 allocates `(this, size)` and slot +8 frees `(this, block,
/// size)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00d440f0` (thiscall, four stack words): the constructor of the 0x38-byte agent the factories
/// build for a convex shape pair.
const SIMPLE_AGENT_INIT: u32 = 0x00d4_40f0;
/// `00dacbd0` (thiscall, three stack words `(body_a, body_b, input)`): the base constructor
/// behind the 0x80-byte agent and the convex-list agent.
const GSK_AGENT_INIT: u32 = 0x00da_cbd0;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall, two stack words): `this = a^-1 * b`.
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `00dafcd0` (thiscall, three stack words): initialises the cache at `this + 0xC`.
const CACHE_INIT: u32 = 0x00da_fcd0;
/// The 1-n machine (`this + 0x30`) routines, all cdecl with the machine first.
const MACHINE_CALC_STATISTICS: u32 = 0x00d9_7250;
const MACHINE_INVALIDATE_TIM: u32 = 0x00da_3510;
const MACHINE_WARP_TIME: u32 = 0x00da_35f0;
const MACHINE_CLEAR_AND_DESTROY: u32 = 0x00da_42b0;
const MACHINE_UPDATE_FILTER: u32 = 0x00da_48e0;
const MACHINE_PROCESS: u32 = 0x00da_4aa0;
const MACHINE_RESET: u32 = 0x00da_4430;
const MACHINE_CLEANUP: u32 = 0x00da_d640;
/// The GSK-mode routines the agent forwards to with `this` (thiscall, one stack word).
const GSK_INVALIDATE_TIM: u32 = 0x00da_beb0;
const GSK_WARP_TIME: u32 = 0x00da_bed0;
const GSK_REMOVE_POINT: u32 = 0x00da_cb20;
const GSK_COMMIT_POTENTIAL: u32 = 0x00da_cb60;
const GSK_CREATE_ZOMBIE: u32 = 0x00da_cba0;
/// `hkpGskBaseAgent::staticGetPenetrations` (`00dac2b0`) and its closest-point sibling
/// (`00dac450`), both cdecl `(body_a, body_b, input, collector)`.
const GSK_STATIC_GET_PENETRATIONS: u32 = 0x00da_c2b0;
const GSK_STATIC_GET_CLOSEST_POINTS: u32 = 0x00da_c450;
/// `hkpGsk::getClosestPoint` (`00dabca0`, cdecl `(record, cache, normal_out, point_out)`).
const GSK_GET_CLOSEST_POINT: u32 = 0x00da_bca0;
/// `00d33670` (thiscall, one stack word) builds the local object at frame `+0x114`; `00d33600`
/// (thiscall, four stack words) works on it.
const LOCAL_OBJECT_INIT: u32 = 0x00d3_3670;
const LOCAL_OBJECT_ADD: u32 = 0x00d3_3600;
/// `00d42020` (thiscall, four stack words): the predictive GSK step of the process.
const PREDICTIVE_STEP: u32 = 0x00d4_2020;
/// The child-shape walkers: `00d43960` (closest points) and `00d43dc0` (penetrations), both cdecl
/// `(body_b, body_a, input, collector)`.
const CHILDREN_CLOSEST_POINTS: u32 = 0x00d4_3960;
const CHILDREN_PENETRATIONS: u32 = 0x00d4_3dc0;
/// `00cfbbe0` (thiscall, three stack words): registers an agent creation function table.
const REGISTER_AGENT_TABLE: u32 = 0x00cf_bbe0;
/// Timer labels of this unit's stages.
const LABEL_STATIC_LIST: u32 = 0x010c_ec40;
const LABEL_CHECK_HULL: u32 = 0x010c_ec34;
const LABEL_CHILDREN: u32 = 0x010c_ec28;
const LABEL_PROCESS: u32 = 0x010c_eb84;
const LABEL_PROCESS_DETAIL: u32 = 0x010c_eb7c;
const LABEL_STREAM: u32 = 0x010c_eb70;
const LABEL_GSK_STEP: u32 = 0x010c_eca0;
/// Vtables of the collectors and agents built here.
const PENETRATION_FLAG_VTABLE: u32 = 0x010c_ebd0;
const CLOSEST_POINT_VTABLE: u32 = 0x010c_476c;
const WRAPPED_PENETRATION_VTABLE: u32 = 0x010c_e81c;
const WRAPPED_CLOSEST_VTABLE: u32 = 0x010c_e7fc;
const PREDICTIVE_FRAME_VTABLE: u32 = 0x010c_eb44;
const COLLECTOR_BASE_VTABLE: u32 = 0x0104_da3c;
const COLLECTOR_ARRAY_VTABLE: u32 = 0x0104_e328;
const CONVEX_LIST_AGENT_VTABLE: u32 = 0x010c_ebf0;
const CONVEX_LIST_FACTORY_VTABLE: u32 = 0x010c_ec68;
const GSKF_AGENT_VTABLE: u32 = 0x010c_eb94;
const SIMPLE_AGENT_VTABLE: u32 = 0x010c_e8b0;
/// The float `-1.0` the constructor starts the distances with.
const MINUS_ONE: u32 = 0x0101_2054;
/// The sign mask that negates lanes 0 to 2 only (`010c4c00`).
const NEGATE_XYZ: u32 = 0x010c_4c00;

/// The allocator object the memory router names, and `size` bytes from it with the size
/// stored as the word at +4 of the block (as every factory does).
fn allocate_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let heap = e.mem.u32(router + 0x10);
    let block = e.vcall(heap, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// Whether the agent runs in GSK mode (the flag byte at +0x84).
fn gsk_mode(e: &Engine, this: u32) -> bool {
    e.mem.u8(this + 0x84) != 0
}

// Translated from 00d2dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleClosestContactCollector` scalar deleting destructor (Xbox PDB): stores the base
/// vtable and, when bit 0 of `flags` is set, frees the 0x30-byte object through the allocator
/// (slot +8, `(this, 0x30)`). Returns `this`.
pub fn hkp_simple_closest_contact_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), COLLECTOR_BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let heap = e.mem.u32(router + 0x10);
        e.vcall(heap, 8, &args![this.addr(), 0x30u32]);
    }
    this
}

// Translated from 00d33740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::calcContentStatistics` (Xbox PDB): outside GSK mode asks the 1-n
/// machine at `this + 0x30` (`00d97250(machine, stats->+8, stats)`). The second stack word is
/// not read.
pub fn hkp_convex_list_agent_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    stats: Ptr,
    _unused_3: u32,
) {
    if !gsk_mode(e, this.addr()) {
        let nested = e.mem.u32(stats.addr() + 8);
        e.call(
            MACHINE_CALC_STATISTICS,
            &args![this.addr() + 0x30, nested, stats.addr()],
        );
    }
}

// Translated from 00d33770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::invalidateTim` (Xbox PDB): in GSK mode tail-jumps to `00dabeb0(this,
/// word)`, otherwise calls the machine's `00da3510(machine, word)`.
pub fn hkp_convex_list_agent_invalidate_tim(e: &mut Engine, this: Ptr, word: u32) {
    if gsk_mode(e, this.addr()) {
        e.call(GSK_INVALIDATE_TIM, &args![this.addr(), word]);
    } else {
        e.call(MACHINE_INVALIDATE_TIM, &args![this.addr() + 0x30, word]);
    }
}

// Translated from 00d337a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::warpTime` (Xbox PDB): forwards both floats (an `FLD`/`FSTP` copy, so a
/// signalling NaN comes out quiet; they are passed as bit patterns) and the word to `00dabed0`
/// in GSK mode (`this` first) or to the machine's `00da35f0` (`this + 0x30` first).
pub fn hkp_convex_list_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    first: f32,
    second: f32,
    word: u32,
) {
    let first = x87_quiet(first).to_bits();
    let second = x87_quiet(second).to_bits();
    if gsk_mode(e, this.addr()) {
        e.call(GSK_WARP_TIME, &args![this.addr(), first, second, word]);
    } else {
        e.call(
            MACHINE_WARP_TIME,
            &args![this.addr() + 0x30, first, second, word],
        );
    }
}

// Translated from 00d337f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::removePoint` (Xbox PDB): in GSK mode tail-jumps to `00dacb20(this,
/// word)`; otherwise does nothing.
pub fn hkp_convex_list_agent_remove_point(e: &mut Engine, this: Ptr, word: u32) {
    if gsk_mode(e, this.addr()) {
        e.call(GSK_REMOVE_POINT, &args![this.addr(), word]);
    }
}

// Translated from 00d33810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::commitPotential` (Xbox PDB): in GSK mode tail-jumps to `00dacb60(this,
/// word)`; otherwise does nothing.
pub fn hkp_convex_list_agent_commit_potential(e: &mut Engine, this: Ptr, word: u32) {
    if gsk_mode(e, this.addr()) {
        e.call(GSK_COMMIT_POTENTIAL, &args![this.addr(), word]);
    }
}

// Translated from 00d33830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::createZombie` (Xbox PDB): in GSK mode tail-jumps to `00dacba0(this,
/// word)`; otherwise does nothing.
pub fn hkp_convex_list_agent_create_zombie(e: &mut Engine, this: Ptr, word: u32) {
    if gsk_mode(e, this.addr()) {
        e.call(GSK_CREATE_ZOMBIE, &args![this.addr(), word]);
    }
}

// Translated from 00d33850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Leaves the multi-contact mode: `00da42b0(machine, [this+0x80], [this+8], word)` clears the
/// 1-n machine at `this + 0x30`, whose first word is then zeroed, and the GSK-mode flag byte
/// (+0x84) is set.
pub fn fn_00d33850(e: &mut Engine, this: Ptr, word: u32) {
    let this = this.addr();
    let owner = e.mem.u32(this + 8);
    let key = e.mem.u32(this + 0x80);
    e.call(
        MACHINE_CLEAR_AND_DESTROY,
        &args![this + 0x30, key, owner, word],
    );
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_u8(this + 0x84, 1);
}

// Translated from 00d33890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::cleanup` (Xbox PDB): in GSK mode `00dad640(machine, [this+8], word)`,
/// otherwise `00da42b0(machine, [this+0x80], [this+8], word)`; then the agent's destructor
/// (vtable slot 0) runs with 1 (delete).
pub fn hkp_convex_list_agent_cleanup(e: &mut Engine, this: Ptr, word: u32) {
    let this = this.addr();
    let owner = e.mem.u32(this + 8);
    if gsk_mode(e, this) {
        e.call(MACHINE_CLEANUP, &args![this + 0x30, owner, word]);
    } else {
        let key = e.mem.u32(this + 0x80);
        e.call(
            MACHINE_CLEAR_AND_DESTROY,
            &args![this + 0x30, key, owner, word],
        );
    }
    e.vcall(this, 0, &args![1u32]);
}

// Translated from 00d338e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::updateShapeCollectionFilter` (Xbox PDB): outside GSK mode builds a
/// seven-word record on the stack (`first`, `second`, an unwritten word, the result of virtual
/// +0x10 of the object `second` points at, `third`, `[this+8]`, `fourth`) and passes it to the
/// machine's `00da48e0(machine, record)`.
pub fn hkp_convex_list_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: u32,
    fourth: u32,
) {
    let this = this.addr();
    if gsk_mode(e, this) {
        return;
    }
    e.with_stack(0x20, |e, record| {
        let record = record.addr();
        let owner = e.mem.u32(this + 8);
        let shape = e.mem.u32(second.addr());
        e.mem.set_u32(record, first.addr());
        e.mem.set_u32(record + 4, second.addr());
        let filter = e.vcall(shape, 0x10, &[]).u32();
        e.mem.set_u32(record + 0xc, filter);
        e.mem.set_u32(record + 0x10, third);
        e.mem.set_u32(record + 0x14, owner);
        e.mem.set_u32(record + 0x18, fourth);
        e.call(MACHINE_UPDATE_FILTER, &args![this + 0x30, record]);
    });
}

// Translated from 00d33940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent` constructor (Xbox PDB): runs the base constructor `00dacbd0(this,
/// body_a, body_b, fourth)`, stores the agent vtable, copies the first word of `pair` to +0x80,
/// sets the GSK-mode flag (+0x84) and clears +0x85, builds `body_a`'s transform inverse times
/// `body_b`'s on the stack and hands it with the two collidables to the cache initialiser
/// `00dafcd0(this + 0xC, [body_a], [body_b], transform)`. The two distances (+0x18, +0x2C) start
/// at `-1.0`. Returns `this`.
pub fn hkp_convex_list_agent_ctor(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    pair: Ptr,
    fourth: u32,
) -> Ptr {
    let this_addr = this.addr();
    e.call(
        GSK_AGENT_INIT,
        &args![this_addr, body_a.addr(), body_b.addr(), fourth],
    );
    e.mem.set_u32(this_addr, CONVEX_LIST_AGENT_VTABLE);
    let key = e.mem.u32(pair.addr());
    e.mem.set_u32(this_addr + 0x80, key);
    e.mem.set_u8(this_addr + 0x84, 1);
    e.mem.set_u8(this_addr + 0x85, 0);
    let collidable_a = e.mem.u32(body_a.addr());
    let collidable_b = e.mem.u32(body_b.addr());
    let transform_a = e.mem.u32(body_a.addr() + 8);
    let transform_b = e.mem.u32(body_b.addr() + 8);
    e.with_stack(0x60, |e, frame| {
        let relative = frame.addr();
        e.call(
            SET_MUL_INVERSE_MUL,
            &args![relative, transform_a, transform_b],
        );
        e.call(
            CACHE_INIT,
            &args![this_addr + 0xc, collidable_a, collidable_b, relative],
        );
    });
    let distance = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    e.mem.set_u32(this_addr + 0x2c, distance);
    e.mem.set_u32(this_addr + 0x18, distance);
    this
}

// Translated from 00d339d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::staticGetClosestPoints` (Xbox PDB, cdecl): a timer start record, then
/// the GSK penetration test `00dac2b0` with a flag collector; when it flags a hit the children
/// are walked (`00d347d0`). Otherwise the GSK closest-point test `00dac450` runs on a stack
/// collector (far distance, hit count at +0x30); with hits, a collector record (two vectors
/// copied from the stack collector, then the two bodies) goes to `collector` (virtual +4) when
/// the first body's collidable value at +0x18 is below the found distance, else the children
/// are walked.
pub fn hkp_convex_list_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    let (a, b, c, collector) = (body_a.addr(), body_b.addr(), input.addr(), collector.addr());
    monitor_start_record(e, LABEL_STATIC_LIST, LABEL_CHECK_HULL);
    // The exe's frame: the flag collector at +0xC, the closest-point collector at +0x44 and the
    // record at +0x14.
    e.with_stack(0xd0, |e, frame| {
        let frame = frame.addr();
        let flagged = frame + 0xc;
        e.mem.set_u32(flagged, PENETRATION_FLAG_VTABLE);
        e.mem.set_u8(flagged + 4, 0);
        e.call(GSK_STATIC_GET_PENETRATIONS, &args![a, b, c, flagged]);
        if e.mem.u8(flagged + 4) != 0 {
            monitor_record(e, LABEL_CHILDREN);
            fn_00d347d0(e, body_a, body_b, input, collector);
            return;
        }
        let closest = frame + 0x44;
        let far = far_distance_bits(e);
        e.mem.set_u32(closest + 0x2c, far);
        e.mem.set_u32(closest + 4, far);
        e.mem.set_u32(closest, CLOSEST_POINT_VTABLE);
        e.mem.set_u32(closest + 0x30, 0);
        e.call(GSK_STATIC_GET_CLOSEST_POINTS, &args![a, b, c, closest]);
        if e.mem.u32(closest + 0x30) == 0 {
            return;
        }
        let collidable = e.mem.u32(b);
        let limit = e.mem.f32(collidable + 0x18);
        let distance = e.mem.f32(closest + 0x2c);
        if limit < distance {
            let record = frame + 0x14;
            let first = e.mem.bytes(closest + 0x10, 0x10);
            e.mem.write(record, &first);
            let second = e.mem.bytes(closest + 0x20, 0x10);
            e.mem.write(record + 0x10, &second);
            e.mem.set_u32(record + 0x20, a);
            e.mem.set_u32(record + 0x24, b);
            e.vcall(collector, 4, &args![record]);
        } else {
            monitor_record(e, LABEL_CHILDREN);
            fn_00d347d0(e, body_a, body_b, input, collector);
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d33b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::getClosestPoints` (Xbox PDB): forwards its four stack words to
/// `staticGetClosestPoints`. `this` is not read.
pub fn hkp_convex_list_agent_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    hkp_convex_list_agent_static_get_closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d33b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::staticGetPenetrations` (Xbox PDB, cdecl): a timer start record, the GSK
/// penetration test `00dac2b0` with a flag collector (vtable `010cebd0`, flag byte at +4), the
/// children walk `00d34790` when it flagged a hit, then the end record.
pub fn hkp_convex_list_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    monitor_start_record(e, LABEL_STATIC_LIST, LABEL_CHECK_HULL);
    e.with_stack(0xc, |e, frame| {
        let flagged = frame.addr();
        e.mem.set_u32(flagged, PENETRATION_FLAG_VTABLE);
        e.mem.set_u8(flagged + 4, 0);
        e.call(
            GSK_STATIC_GET_PENETRATIONS,
            &args![body_a.addr(), body_b.addr(), input.addr(), flagged],
        );
        if e.mem.u8(flagged + 4) != 0 {
            monitor_record(e, LABEL_CHILDREN);
            fn_00d34790(e, body_a, body_b, input, collector.addr());
        }
    });
    monitor_record(e, LABEL_END);
}

// Translated from 00d33c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::getPenetrations` (Xbox PDB): forwards its four stack words to
/// `staticGetPenetrations`. `this` is not read.
pub fn hkp_convex_list_agent_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    hkp_convex_list_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

// Translated from 00d33dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent factory (cdecl, four words `(first, second, third, fourth)`; the parameter roles are
/// described from their use). With `fourth == 0` it builds the 0x38-byte simple agent with
/// `00d440f0(block, first, second, third, 0)`. Otherwise the object at `[third + 0x10]` decides
/// by its virtual +0xC `(first, second, third)`: 0 builds the convex-list agent (0x90 bytes,
/// vtable `010cec68`) with swapped bodies, 1 the simple agent (0x38 bytes) and 2 the 0x80-byte
/// agent with vtable `010ceb94`; any other result yields null. Every block gets its size as
/// the word at +4.
pub fn fn_00d33dd0(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32) -> Ptr {
    let (a1, a2, a3) = (first.addr(), second.addr(), third.addr());
    if fourth == 0 {
        let block = allocate_block(e, 0x38);
        return e
            .call(SIMPLE_AGENT_INIT, &args![block, a1, a2, a3, 0u32])
            .ptr();
    }
    let owner = e.mem.u32(a3 + 0x10);
    match e.vcall(owner, 0xc, &args![a1, a2, a3]).u32() {
        0 => {
            let block = allocate_block(e, 0x90);
            hkp_convex_list_agent_ctor(e, Ptr::new(block), second, first, third, fourth);
            e.mem.set_u32(block, CONVEX_LIST_FACTORY_VTABLE);
            Ptr::new(block)
        }
        1 => {
            let block = allocate_block(e, 0x38);
            e.call(SIMPLE_AGENT_INIT, &args![block, a1, a2, a3, fourth])
                .ptr()
        }
        2 => {
            let block = allocate_block(e, 0x80);
            e.call(GSK_AGENT_INIT, &args![block, a1, a2, fourth]);
            e.mem.set_u32(block, GSKF_AGENT_VTABLE);
            Ptr::new(block)
        }
        _ => Ptr::new(0),
    }
}

// Translated from 00d33ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-enters the multi-contact mode: `00dad640(machine, [this+8], word)` cleans the 1-n
/// machine at `this + 0x30`, the GSK-mode flag (+0x84) is cleared, the machine's first three
/// words are reset to `0, 0, 0x80000000` (its address is never null, the exe tests it anyway)
/// and `00da4430(machine)` runs. The distance at +0x3C becomes 0.0 and the countdown at +0x86
/// restarts at 0x19.
pub fn fn_00d33ef0(e: &mut Engine, this: Ptr, word: u32) {
    let this = this.addr();
    let machine = this.wrapping_add(0x30);
    let owner = e.mem.u32(this + 8);
    e.call(MACHINE_CLEANUP, &args![machine, owner, word]);
    e.mem.set_u8(this + 0x84, 0);
    if machine != 0 {
        e.mem.set_u32(machine, 0);
        e.mem.set_u32(machine + 4, 0);
        e.mem.set_u32(machine + 8, 0x8000_0000);
    }
    e.call(MACHINE_RESET, &args![machine]);
    e.mem.set_u32(this + 0x3c, 0);
    e.mem.set_u16(this + 0x86, 0x19);
}

/// Where `hkp_convex_list_agent_process_collision` continues.
#[derive(Clone, Copy)]
enum ProcessStep {
    /// The countdown and the GSK closest-point query (`00d3403f`).
    Head,
    /// The distance test (`00d34124`).
    Distance,
    /// The predictive GSK step (`00d34180`).
    Gsk,
    /// The 1-n machine pass (`00d342df`).
    Machine,
    /// The closing flag and timer record (`00d343a4`).
    End,
}

// Translated from 00d33f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListAgent::processCollision` (Xbox PDB): the agent's per-step update of two
/// bodies. A timer start record is written, then the relative velocity vector is built from the
/// two bodies' transforms (lanes 0-2 in single-precision SSE arithmetic, lane 3 the x87 sum of
/// the two damping terms stored as a float) and the stack object at frame `+0x114` is
/// initialised with `00d33670(object, result)`.
///
/// In multi-contact mode (flag byte +0x84 clear) each pass writes a stream record, counts the
/// word at +0x86 down and, once it runs out, restarts it at 0x19, builds the relative transform
/// on the stack and asks the GSK closest-point query `00dabca0` for a contact; no contact sets
/// the distance +0x3C to `-[+0x2C]` and goes to the machine pass, a contact switches to GSK
/// mode (`fn_00d33850`) and records it with `00d33600(object, a, b, result, [this+8])`. The
/// distance test then subtracts the vector's projection on the contact normal from +0x2C; when
/// it is still above `info->+8` the agent has left the range (cleaning the machine if
/// +0x32 is set) and the process ends. Otherwise the predictive step `00d42020` runs with
/// `this + 8` temporarily pointing at a stack frame; with no prediction the loop leaves
/// unless +0x32 is set, +0x85 is clear and the distance is below minus twice the smaller of
/// the two chain roots' values at +0x4C; otherwise the contact is recorded again, the agent
/// returns to multi-contact mode (`fn_00d33ef0`) and the loop restarts.
///
/// The machine pass reduces +0x3C by the vector's length, builds the relative transform and
/// the record the machine processes together with an index list taken from the router
/// (`00c85750`, bump allocation of `(count * 4 + 0x83) & ~0x7F` bytes, indices `0..count` and
/// a `-1` terminator, `count` being the word at +0x48 of the second body's collidable). The
/// flag at +0x85 is set and the end record written last.
pub fn hkp_convex_list_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    info: Ptr,
    result: Ptr,
) {
    let (this, a, b, info, result) = (
        this.addr(),
        body_a.addr(),
        body_b.addr(),
        info.addr(),
        result.addr(),
    );
    monitor_start_record(e, LABEL_PROCESS, LABEL_PROCESS_DETAIL);
    // The exe's frame is 0x1194 bytes; `f` is its lowest address.
    e.with_stack(0x1200, |e, frame| {
        let f = frame.addr();
        let transform_a = e.mem.u32(a + 8);
        let transform_b = e.mem.u32(b + 8);
        let collidable_b = e.mem.u32(b);
        let scale = x87_quiet(e.mem.f32(info + 0x58));
        let scaled_a = (f64::from(e.mem.f32(transform_a + 0x5c)) * f64::from(scale)) as f32;
        let scaled_b = (f64::from(scale) * f64::from(e.mem.f32(transform_b + 0x5c))) as f32;
        let mut relative = [0.0f32; 4];
        for (lane, slot) in relative.iter_mut().take(3).enumerate() {
            let offset = 4 * lane as u32;
            let toward_b =
                e.mem.f32(transform_b + 0x50 + offset) - e.mem.f32(transform_b + 0x40 + offset);
            let toward_a =
                e.mem.f32(transform_a + 0x40 + offset) - e.mem.f32(transform_a + 0x50 + offset);
            *slot = toward_b * scaled_b + toward_a * scaled_a;
        }
        let damping_a = f64::from(e.mem.f32(transform_a + 0x9c))
            * f64::from(scaled_a)
            * f64::from(e.mem.f32(transform_a + 0xa0));
        let damping_b = f64::from(e.mem.f32(transform_b + 0x9c))
            * f64::from(scaled_b)
            * f64::from(e.mem.f32(transform_b + 0xa0));
        relative[3] = (damping_a + damping_b) as f32;
        e.call(LOCAL_OBJECT_INIT, &args![f + 0x114, result]);

        let mut step = if gsk_mode(e, this) {
            ProcessStep::Distance
        } else {
            ProcessStep::Head
        };
        loop {
            match step {
                ProcessStep::Head => {
                    monitor_record(e, LABEL_STREAM);
                    let countdown = e.mem.u16(this + 0x86);
                    e.mem.set_u16(this + 0x86, countdown.wrapping_sub(1));
                    if (countdown as i16) >= 0 {
                        step = ProcessStep::Machine;
                        continue;
                    }
                    e.mem.set_u16(this + 0x86, 0x19);
                    let transform_a = e.mem.u32(a + 8);
                    let transform_b = e.mem.u32(b + 8);
                    e.call(
                        SET_MUL_INVERSE_MUL,
                        &args![f + 0xd4, transform_a, transform_b],
                    );
                    let collidable_a = e.mem.u32(a);
                    e.mem.set_u32(f + 0x24, collidable_a);
                    e.mem.set_u32(f + 0x1c, f + 0xd4);
                    let collidable_now = e.mem.u32(b);
                    e.mem.set_u32(f + 0x28, collidable_now);
                    let reach = x87_quiet_bits(e.mem.u32(info + 8));
                    e.mem.set_u32(f + 0x2c, reach);
                    e.mem.set_u32(f + 0x20, transform_a);
                    let found = e
                        .call(
                            GSK_GET_CLOSEST_POINT,
                            &args![f + 0x1c, this + 0xc, this + 0x20, f + 0xc4],
                        )
                        .u32();
                    if found == 0 {
                        let distance = x87_quiet(e.mem.f32(this + 0x2c));
                        e.mem.set_f32(this + 0x3c, -distance);
                        step = ProcessStep::Machine;
                        continue;
                    }
                    let key = e.mem.u32(result + 4);
                    fn_00d33850(e, Ptr::new(this), key);
                    let owner = e.mem.u32(this + 8);
                    e.call(LOCAL_OBJECT_ADD, &args![f + 0x114, a, b, result, owner]);
                    step = ProcessStep::Distance;
                }
                ProcessStep::Distance => {
                    let reach = e.mem.f32(info + 8);
                    let distance = e.mem.f32(this + 0x2c);
                    if reach < distance {
                        let normal = [0u32, 1, 2, 3].map(|lane| e.mem.f32(this + 0x20 + 4 * lane));
                        let product = [0usize, 1, 2].map(|lane| normal[lane] * relative[lane]);
                        let projection = (product[2] + relative[3]) + (product[1] + product[0]);
                        let remaining = (f64::from(distance) - f64::from(projection)) as f32;
                        e.mem.set_f32(this + 0x2c, remaining);
                        if e.mem.f32(info + 8) < e.mem.f32(this + 0x2c) {
                            if e.mem.u8(this + 0x32) != 0 {
                                let owner = e.mem.u32(this + 8);
                                let key = e.mem.u32(result + 4);
                                e.call(MACHINE_CLEANUP, &args![this + 0x30, owner, key]);
                            }
                            step = ProcessStep::End;
                            continue;
                        }
                    }
                    step = ProcessStep::Gsk;
                }
                ProcessStep::Gsk => {
                    monitor_record(e, LABEL_GSK_STEP);
                    let saved_owner = e.mem.u32(this + 8);
                    e.mem.set_u16(f + 0x22, 1);
                    e.mem.set_u32(f + 0x28, saved_owner);
                    e.mem.set_u32(f + 0x24, 2);
                    e.mem.set_u32(f + 0x1c, PREDICTIVE_FRAME_VTABLE);
                    e.mem.set_u8(f + 0x2c, 0);
                    e.mem.set_u32(this + 8, f + 0x1c);
                    e.call(PREDICTIVE_STEP, &args![this, a, b, info, result]);
                    let restored = e.mem.u32(f + 0x28);
                    let predicted = e.mem.u8(f + 0x2c);
                    e.mem.set_u32(this + 8, restored);
                    if predicted == 0 {
                        if e.mem.u8(this + 0x32) == 0 || e.mem.u8(this + 0x85) != 0 {
                            step = ProcessStep::End;
                            continue;
                        }
                        let mut root_b = b;
                        while e.mem.u32(root_b + 0xc) != 0 {
                            root_b = e.mem.u32(root_b + 0xc);
                        }
                        let value_b = x87_quiet(e.mem.f32(root_b + 0x4c));
                        let mut root_a = a;
                        while e.mem.u32(root_a + 0xc) != 0 {
                            root_a = e.mem.u32(root_a + 0xc);
                        }
                        let value_a = x87_quiet(e.mem.f32(root_a + 0x4c));
                        let smaller = if value_b > value_a { value_a } else { value_b };
                        let twice = (f64::from(smaller) + f64::from(smaller)) as f32;
                        let distance = e.mem.f32(this + 0x2c);
                        if (-twice).partial_cmp(&distance) != Some(Ordering::Greater) {
                            step = ProcessStep::End;
                            continue;
                        }
                    }
                    e.call(LOCAL_OBJECT_ADD, &args![f + 0x114, a, b, result, restored]);
                    let key = e.mem.u32(result + 4);
                    fn_00d33ef0(e, Ptr::new(this), key);
                    step = ProcessStep::Head;
                }
                ProcessStep::Machine => {
                    let squares = [0usize, 1, 2].map(|lane| relative[lane] * relative[lane]);
                    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
                    let travelled = x87_quiet(e.mem.f32(this + 0x3c));
                    e.mem.set_u32(f + 0x4c, 0);
                    let reduced = (f64::from(travelled) - f64::from(length)) as f32;
                    e.mem.set_f32(this + 0x3c, reduced);
                    let owner = e.mem.u32(this + 8);
                    e.mem.set_u32(f + 0x54, owner);
                    e.mem.set_u32(f + 0x44, a);
                    e.mem.set_u32(f + 0x48, b);
                    e.mem.set_u32(f + 0x50, info);
                    for (lane, value) in relative.iter().enumerate() {
                        e.mem.set_f32(f + 0xb4 + 4 * lane as u32, *value);
                    }
                    let transform_a = e.mem.u32(a + 8);
                    let transform_b = e.mem.u32(b + 8);
                    e.call(
                        SET_MUL_INVERSE_MUL,
                        &args![f + 0x64, transform_a, transform_b],
                    );
                    let count = e.mem.i32(collidable_b + 0x48);
                    let router = e.call(MEMORY_ROUTER, &[]).u32();
                    let indices = e.mem.u32(router);
                    let reserved =
                        ((count as u32).wrapping_mul(4).wrapping_add(0x83)) & 0xffff_ff80;
                    e.mem.set_u32(router, reserved.wrapping_add(indices));
                    for index in 0..count.max(0) {
                        e.mem
                            .set_u32(indices.wrapping_add(4 * index as u32), index as u32);
                    }
                    e.mem.set_u32(
                        indices.wrapping_add((count as u32).wrapping_mul(4)),
                        0xffff_ffff,
                    );
                    e.call(
                        MACHINE_PROCESS,
                        &args![this + 0x30, f + 0x44, collidable_b + 0x14, indices, result],
                    );
                    let router = e.call(MEMORY_ROUTER, &[]).u32();
                    e.mem.set_u32(router, indices);
                    step = ProcessStep::End;
                }
                ProcessStep::End => break,
            }
        }
    });
    e.mem.set_u8(this + 0x85, 1);
    monitor_record(e, LABEL_END);
}

// Translated from 00d343f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent factory for the swapped body order (cdecl, four words `(first, second, third,
/// fourth)`): like `00d33dd0`, but the selector is asked `(second, first, third)`, kind 0 builds
/// the 0x90-byte convex-list agent with the constructor (`first`, `second`) and returns what it
/// returns (no factory vtable), kind 1 builds the simple agent with swapped bodies and vtable
/// `010ce8b0`, kind 2 the 0x80-byte agent as in `00d33dd0`. With `fourth == 0` the simple agent
/// is built with swapped bodies and vtable `010ce8b0`.
pub fn fn_00d343f0(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32) -> Ptr {
    let (a1, a2, a3) = (first.addr(), second.addr(), third.addr());
    if fourth == 0 {
        let block = allocate_block(e, 0x38);
        e.call(SIMPLE_AGENT_INIT, &args![block, a2, a1, a3, 0u32]);
        e.mem.set_u32(block, SIMPLE_AGENT_VTABLE);
        return Ptr::new(block);
    }
    let owner = e.mem.u32(a3 + 0x10);
    match e.vcall(owner, 0xc, &args![a2, a1, a3]).u32() {
        0 => {
            let block = allocate_block(e, 0x90);
            hkp_convex_list_agent_ctor(e, Ptr::new(block), first, second, third, fourth)
        }
        1 => {
            let block = allocate_block(e, 0x38);
            e.call(SIMPLE_AGENT_INIT, &args![block, a2, a1, a3, fourth]);
            e.mem.set_u32(block, SIMPLE_AGENT_VTABLE);
            Ptr::new(block)
        }
        2 => {
            let block = allocate_block(e, 0x80);
            e.call(GSK_AGENT_INIT, &args![block, a1, a2, fourth]);
            e.mem.set_u32(block, GSKF_AGENT_VTABLE);
            Ptr::new(block)
        }
        _ => Ptr::new(0),
    }
}

// Translated from 00d34510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent factory with a second selector (cdecl, four words `(first, second, third, fourth)`).
/// With `fourth == 0` the simple agent is built with `00d440f0(block, first, second, third,
/// 0)`. The selector `[third + 0x10]` virtual +0xC `(first, second, third)` gives 0: the
/// convex-list agent with swapped bodies and vtable `010cec68`; 1: the simple agent; 2: the
/// selector is asked again with `(second, first, third)` and gives 0: the convex-list agent
/// (constructor result returned), 1: the simple agent with swapped bodies and vtable
/// `010ce8b0`, 2: the 0x80-byte agent (vtable `010ceb94`); anything else yields null.
pub fn fn_00d34510(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32) -> Ptr {
    let (a1, a2, a3) = (first.addr(), second.addr(), third.addr());
    if fourth == 0 {
        let block = allocate_block(e, 0x38);
        return e
            .call(SIMPLE_AGENT_INIT, &args![block, a1, a2, a3, 0u32])
            .ptr();
    }
    let owner = e.mem.u32(a3 + 0x10);
    match e.vcall(owner, 0xc, &args![a1, a2, a3]).u32() {
        0 => {
            let block = allocate_block(e, 0x90);
            hkp_convex_list_agent_ctor(e, Ptr::new(block), second, first, third, fourth);
            e.mem.set_u32(block, CONVEX_LIST_FACTORY_VTABLE);
            Ptr::new(block)
        }
        1 => {
            let block = allocate_block(e, 0x38);
            e.call(SIMPLE_AGENT_INIT, &args![block, a1, a2, a3, fourth])
                .ptr()
        }
        2 => {
            let owner = e.mem.u32(a3 + 0x10);
            match e.vcall(owner, 0xc, &args![a2, a1, a3]).u32() {
                0 => {
                    let block = allocate_block(e, 0x90);
                    hkp_convex_list_agent_ctor(e, Ptr::new(block), first, second, third, fourth)
                }
                1 => {
                    let block = allocate_block(e, 0x38);
                    e.call(SIMPLE_AGENT_INIT, &args![block, a2, a1, a3, fourth]);
                    e.mem.set_u32(block, SIMPLE_AGENT_VTABLE);
                    Ptr::new(block)
                }
                2 => {
                    let block = allocate_block(e, 0x80);
                    e.call(GSK_AGENT_INIT, &args![block, a1, a2, fourth]);
                    e.mem.set_u32(block, GSKF_AGENT_VTABLE);
                    Ptr::new(block)
                }
                _ => Ptr::new(0),
            }
        }
        _ => Ptr::new(0),
    }
}

/// One agent-table registration: the four function addresses, the first flag byte and the two
/// type words.
type AgentRegistration = ([u32; 4], u8, u32, u32);

// Translated from 00d346b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the convex-list agent creation tables with the registry (`registry`, cdecl): each
/// table (four function addresses at +0..+0xC, flag bytes at +0x10 and +0x11, the second always
/// 1) goes to `00cfbbe0(registry, table, type, type)` for the shape type pairs `(0x16, 0x12)`
/// with the `00d33dd0` creators, `(0x12, 0x16)` with the `00d343f0` creators and `(0x16, 0x16)`
/// with the `00d34510` creators.
pub fn fn_00d346b0(e: &mut Engine, registry: Ptr) {
    let registrations: [AgentRegistration; 3] = [
        (
            [0x00d3_3dd0, 0x00d3_4930, 0x00d3_4970, 0x00d3_49b0],
            1,
            0x16,
            0x12,
        ),
        (
            [0x00d3_43f0, 0x00d3_3b90, 0x00d3_39d0, 0x00d3_3ca0],
            0,
            0x12,
            0x16,
        ),
        (
            [0x00d3_4510, 0x00d3_3b90, 0x00d3_39d0, 0x00d3_3ca0],
            0,
            0x16,
            0x16,
        ),
    ];
    // The exe's 0x14-byte frame holds the table at +0.
    e.with_stack(0x14, |e, frame| {
        let table = frame.addr();
        for (functions, flag, word_1, word_2) in registrations {
            for (slot, function) in functions.iter().enumerate() {
                e.mem.set_u32(table + 4 * slot as u32, *function);
            }
            e.mem.set_u8(table + 0x10, flag);
            e.mem.set_u8(table + 0x11, 1);
            e.call(
                REGISTER_AGENT_TABLE,
                &args![registry.addr(), table, word_1, word_2],
            );
        }
    });
}

// Translated from 00d34770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleClosestContactCollector::reset` (Xbox PDB, `this` in `ECX`): clears the hit flag
/// byte (+8) and restores the far distance (`FLD`/`FST`, so quiet) at +0x2C and +4.
pub fn hkp_simple_closest_contact_collector_reset(e: &mut Engine, this: Ptr) {
    let far = far_distance_bits(e);
    e.mem.set_u8(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x2c, far);
    e.mem.set_u32(this.addr() + 4, far);
}

// Translated from 00d34790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the children for penetrations (cdecl, four words): wraps `collector` in a stack
/// collector (vtable `010ce81c`, flag byte at +4 cleared, `collector` at +8) and calls
/// `00d43dc0(second, first, third, wrapper)`.
pub fn fn_00d34790(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, collector: u32) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u8(wrapper + 4, 0);
        e.mem.set_u32(wrapper, WRAPPED_PENETRATION_VTABLE);
        e.call(
            CHILDREN_PENETRATIONS,
            &args![second.addr(), first.addr(), third.addr(), wrapper],
        );
    });
}

// Translated from 00d347d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the children for closest points (cdecl, four words): wraps `collector` in a stack
/// collector (vtable `010ce7fc`, the far distance at +4, `collector` at +8) and calls
/// `00d43960(second, first, third, wrapper)`.
pub fn fn_00d347d0(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, collector: u32) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        let far = far_distance_bits(e);
        e.mem.set_u32(wrapper + 4, far);
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u32(wrapper, WRAPPED_CLOSEST_VTABLE);
        e.call(
            CHILDREN_CLOSEST_POINTS,
            &args![second.addr(), first.addr(), third.addr(), wrapper],
        );
    });
}

// Translated from 00d34900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the 8-byte array collector (`this` in `ECX`, one stack
/// word): stores the vtable `0104e328` and, when bit 0 of `flags` is set, frees the 8-byte
/// object through the allocator (slot +8, `(this, 8)`). Returns `this`.
pub fn fn_00d34900(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), COLLECTOR_ARRAY_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let heap = e.mem.u32(router + 0x10);
        e.vcall(heap, 8, &args![this.addr(), 8u32]);
    }
    this
}

// Translated from 00d34930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The penetration test the agent table registers (cdecl, four words): wraps `collector` in a
/// stack collector (vtable `010ce81c`, flag byte at +4 cleared, `collector` at +8) and runs
/// `staticGetPenetrations(second, first, third, wrapper)`.
pub fn fn_00d34930(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, collector: u32) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u8(wrapper + 4, 0);
        e.mem.set_u32(wrapper, WRAPPED_PENETRATION_VTABLE);
        hkp_convex_list_agent_static_get_penetrations(e, second, first, third, Ptr::new(wrapper));
    });
}

// Translated from 00d34970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The closest-point test the agent table registers (cdecl, four words): wraps `collector` in a
/// stack collector (vtable `010ce7fc`, the far distance at +4, `collector` at +8) and runs
/// `staticGetClosestPoints(second, first, third, wrapper)`.
pub fn fn_00d34970(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, collector: u32) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        let far = far_distance_bits(e);
        e.mem.set_u32(wrapper + 4, far);
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u32(wrapper, WRAPPED_CLOSEST_VTABLE);
        hkp_convex_list_agent_static_get_closest_points(e, second, first, third, Ptr::new(wrapper));
    });
}

/// The collectors the swapped-order cast stubs build on the exe's frame (`frame` is the address
/// of its first local): the cast collector at `+8` (vtable `010ce80c`, the far distance at +4,
/// the cast input's vector at +0x10 and `cast_collector` at +0x20), a second one of the same
/// shape at `+0x38` holding `start_collector` when that is non-zero, and the 0x70-byte copy of
/// the cast input at `+0x68` whose vector at +0x50 is negated lane by lane. Returns the second
/// collector's address or 0.
fn build_cast_collectors(
    e: &mut Engine,
    frame: u32,
    input: u32,
    cast_collector: u32,
    start_collector: u32,
) -> u32 {
    let far = far_distance_bits(e);
    e.mem.set_u32(frame + 0xc, far);
    let copy = e.mem.bytes(input, 0x70);
    e.mem.write(frame + 0x68, &copy);
    for lane in 0..4u32 {
        let word = e.mem.u32(input + 0x50 + 4 * lane);
        e.mem.set_u32(frame + 0x18 + 4 * lane, word);
    }
    e.mem.set_u32(frame + 0x28, cast_collector);
    for lane in 0..4u32 {
        let word = e.mem.u32(input + 0x50 + 4 * lane);
        let mask = e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(frame + 0xb8 + 4 * lane, word ^ mask);
    }
    e.mem.set_u32(frame + 8, CAST_COLLECTOR_VTABLE);
    if start_collector == 0 {
        return 0;
    }
    e.mem.set_u32(frame + 0x38, CAST_COLLECTOR_VTABLE);
    e.mem.set_u32(frame + 0x3c, far);
    for lane in 0..4u32 {
        let word = e.mem.u32(input + 0x50 + 4 * lane);
        e.mem.set_u32(frame + 0x48 + 4 * lane, word);
    }
    e.mem.set_u32(frame + 0x58, start_collector);
    frame + 0x38
}

// Translated from 00d349b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The linear cast the agent table registers (cdecl, five words `(first, second, input,
/// cast_collector, start_collector)`; the roles are described from their use): builds the cast
/// collectors and the negated input copy on its frame and runs `00d33ca0(second, first, copy,
/// collector, second collector or 0)`.
pub fn fn_00d349b0(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    e.with_stack(0xd8, |e, frame| {
        let frame = frame.addr();
        let start = build_cast_collectors(e, frame, input.addr(), cast_collector, start_collector);
        fn_00d33ca0(e, second, first, Ptr::new(frame + 0x68), frame + 8, start);
    });
}

// Translated from 00d34aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpConvexListAgent>::updateShapeCollectionFilter` (Xbox PDB):
/// forwards `this` and the four stack words with the first two swapped to
/// `hkpConvexListAgent::updateShapeCollectionFilter`.
pub fn hkp_symmetric_agent_linear_cast_convex_list_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: u32,
    fourth: u32,
) {
    hkp_convex_list_agent_update_shape_collection_filter(e, this, second, first, third, fourth);
}

// Translated from 00d34ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpConvexListAgent>::getPenetrations` (Xbox PDB): wraps
/// `collector` in a stack collector (vtable `010ce81c`, flag byte at +4 cleared, `collector`
/// at +8) and runs `hkpConvexListAgent::getPenetrations(second, first, third, wrapper)`.
/// `this` is not read.
pub fn hkp_symmetric_agent_linear_cast_convex_list_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    collector: u32,
) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u8(wrapper + 4, 0);
        e.mem.set_u32(wrapper, WRAPPED_PENETRATION_VTABLE);
        hkp_convex_list_agent_get_penetrations(
            e,
            _unused_this,
            second,
            first,
            third,
            Ptr::new(wrapper),
        );
    });
}

// Translated from 00d34b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpConvexListAgent>::getClosestPoints` (Xbox PDB): wraps
/// `collector` in a stack collector (vtable `010ce7fc`, the far distance at +4, `collector` at
/// +8) and runs `hkpConvexListAgent::getClosestPoints(second, first, third, wrapper)`. `this`
/// is not read.
pub fn hkp_symmetric_agent_linear_cast_convex_list_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    collector: u32,
) {
    e.with_stack(0xc, |e, frame| {
        let wrapper = frame.addr();
        let far = far_distance_bits(e);
        e.mem.set_u32(wrapper + 8, collector);
        e.mem.set_u32(wrapper + 4, far);
        e.mem.set_u32(wrapper, WRAPPED_CLOSEST_VTABLE);
        hkp_convex_list_agent_get_closest_points(
            e,
            _unused_this,
            second,
            first,
            third,
            Ptr::new(wrapper),
        );
    });
}

// Translated from 00d34b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpConvexListAgent>::linearCast` (Xbox PDB): like `00d349b0`, builds the
/// cast collectors and the negated input copy on its frame and calls
/// `hkpConvexListAgent::linearCast(second, first, copy, collector, second collector or 0)`.
/// `this` is only forwarded in `ECX` to that callee, which does not read it.
pub fn hkp_symmetric_agent_convex_list_linear_cast(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    e.with_stack(0xd8, |e, frame| {
        let frame = frame.addr();
        let start = build_cast_collectors(e, frame, input.addr(), cast_collector, start_collector);
        hkp_convex_list_agent_linear_cast(
            e,
            this.addr(),
            second,
            first,
            Ptr::new(frame + 0x68),
            frame + 8,
            start,
        );
    });
}

// Translated from 00d34c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpConvexListAgent>::processCollision` (Xbox PDB): runs
/// `hkpConvexListAgent::processCollision(this, second, first, third, result)` and then mirrors
/// the contact records it appended to `result` (the stream from the first word of `result`,
/// 0x30 bytes per record): each record's point moves by its normal's fourth lane times the
/// normal (single-precision SSE) and the normal's lanes 0-2 are negated. If the float at
/// `result + 0x3030` changed (a NaN counts as changed) the vector at +0x3020 is negated the
/// same way, the nibbles of the byte at +0x304A swap, the bytes at +0x3048 and +0x3049 swap,
/// the words at +0x3040 and `+0x3040 + 2 * (the sum of those bytes - 1)` swap and, when both
/// bytes now share exactly bit 1 and nothing else (`&` is 2), the words at +0x3042 and +0x3044
/// swap.
pub fn hkp_symmetric_agent_linear_cast_convex_list_process_collision(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    result: Ptr,
) {
    let result = result.addr();
    let before = x87_quiet(e.mem.f32(result + 0x3030));
    let mut cursor = e.mem.u32(result);
    hkp_convex_list_agent_process_collision(e, this, second, first, third, Ptr::new(result));
    let negate = [0u32, 1, 2, 3].map(|lane| e.mem.u32(NEGATE_XYZ + 4 * lane));
    while cursor < e.mem.u32(result) {
        let point = [0u32, 1, 2, 3].map(|lane| e.mem.f32(cursor + 4 * lane));
        let normal = [0u32, 1, 2, 3].map(|lane| e.mem.f32(cursor + 0x10 + 4 * lane));
        for lane in 0..4usize {
            e.mem.set_f32(
                cursor + 4 * lane as u32,
                normal[3] * normal[lane] + point[lane],
            );
            let bits = e.mem.u32(cursor + 0x10 + 4 * lane as u32);
            e.mem
                .set_u32(cursor + 0x10 + 4 * lane as u32, bits ^ negate[lane]);
        }
        cursor = cursor.wrapping_add(0x30);
    }
    let after = x87_quiet(e.mem.f32(result + 0x3030));
    if after != before {
        for (lane, mask) in negate.iter().enumerate() {
            let at = result + 0x3020 + 4 * lane as u32;
            let bits = e.mem.u32(at);
            e.mem.set_u32(at, bits ^ mask);
        }
        let nibbles = e.mem.u8(result + 0x304a);
        e.mem
            .set_u8(result + 0x304a, (nibbles >> 4) | ((nibbles & 0xf) << 4));
        let old_second = e.mem.u8(result + 0x3049);
        let old_first = e.mem.u8(result + 0x3048);
        e.mem.set_u8(result + 0x3049, old_first);
        e.mem.set_u8(result + 0x3048, old_second);
        let index = (u32::from(old_second) + u32::from(old_first)).wrapping_sub(1);
        let head = e.mem.u16(result + 0x3040);
        let other_at = result
            .wrapping_add(0x3040)
            .wrapping_add(index.wrapping_mul(2));
        let other = e.mem.u16(other_at);
        e.mem.set_u16(result + 0x3040, other);
        e.mem.set_u16(other_at, head);
        if e.mem.u8(result + 0x3048) & e.mem.u8(result + 0x3049) == 2 {
            let low = e.mem.u16(result + 0x3042);
            let high = e.mem.u16(result + 0x3044);
            e.mem.set_u16(result + 0x3042, high);
            e.mem.set_u16(result + 0x3044, low);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_3ca0, fn_00d33ca0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_3da0,
            hkp_convex_list_agent_linear_cast(u32, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d3_4810, fn_00d34810(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d2_dc10,
            hkp_simple_closest_contact_collector_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d3_3740,
            hkp_convex_list_agent_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d3_3770, hkp_convex_list_agent_invalidate_tim(Ptr, u32)),
        entry!(
            0x00d3_37a0,
            hkp_convex_list_agent_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(0x00d3_37f0, hkp_convex_list_agent_remove_point(Ptr, u32)),
        entry!(
            0x00d3_3810,
            hkp_convex_list_agent_commit_potential(Ptr, u32)
        ),
        entry!(0x00d3_3830, hkp_convex_list_agent_create_zombie(Ptr, u32)),
        entry!(0x00d3_3850, fn_00d33850(Ptr, u32)),
        entry!(0x00d3_3890, hkp_convex_list_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_38e0,
            hkp_convex_list_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d3_3940,
            hkp_convex_list_agent_ctor(Ptr, Ptr, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d3_39d0,
            hkp_convex_list_agent_static_get_closest_points(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_3b70,
            hkp_convex_list_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_3b90,
            hkp_convex_list_agent_static_get_penetrations(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_3c80,
            hkp_convex_list_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_3dd0, fn_00d33dd0(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d3_3ef0, fn_00d33ef0(Ptr, u32)),
        entry!(
            0x00d3_3f40,
            hkp_convex_list_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_43f0, fn_00d343f0(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d3_4510, fn_00d34510(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d3_46b0, fn_00d346b0(Ptr)),
        entry!(0x00d3_4770, hkp_simple_closest_contact_collector_reset(Ptr)),
        entry!(0x00d3_4790, fn_00d34790(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_47d0, fn_00d347d0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_4900, fn_00d34900(Ptr, u32) -> Ptr),
        entry!(0x00d3_4930, fn_00d34930(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_4970, fn_00d34970(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_49b0, fn_00d349b0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_4aa0,
            hkp_symmetric_agent_linear_cast_convex_list_update_shape_collection_filter(
                Ptr, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d3_4ac0,
            hkp_symmetric_agent_linear_cast_convex_list_get_penetrations(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d3_4b00,
            hkp_symmetric_agent_linear_cast_convex_list_get_closest_points(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d3_4b40,
            hkp_symmetric_agent_convex_list_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d3_4c10,
            hkp_symmetric_agent_linear_cast_convex_list_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// What the forwarding double saw: its arguments, the input copy, both collectors.
    #[derive(Debug, Default, Clone, PartialEq)]
    struct Forwarded {
        args: Vec<u32>,
        copy: Vec<u8>,
        first: Vec<u32>,
        second: Vec<u32>,
    }

    type Seen = Rc<RefCell<Vec<Forwarded>>>;

    fn rig() -> (Engine, Seen) {
        let mut e = test_engine();
        e.set_global(FAR_DISTANCE, 0x7f7f_ffeeu32);
        for lane in 0..4 {
            e.set_global(SIGN_MASK + 4 * lane, 0x8000_0000u32);
        }
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CAST_FORWARD, move |e, a| {
            let words = |address: u32, count: u32| -> Vec<u32> {
                (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
            };
            let second = if a[4] != 0 {
                words(a[4], 9)
            } else {
                Vec::new()
            };
            sink.borrow_mut().push(Forwarded {
                args: a.to_vec(),
                copy: e.mem.bytes(a[2], 0x70),
                first: words(a[3], 9),
                second,
            });
            Ret::default()
        });
        (e, seen)
    }

    fn input(e: &mut Engine) -> u32 {
        let input = e.mem.alloc(0x70);
        for i in 0..0x1c {
            e.mem.set_u32(input + 4 * i, 0x100 + i);
        }
        input
    }

    #[test]
    fn forwarding_cast_builds_one_or_two_collectors_and_negates_the_vector() {
        let (mut e, seen) = rig();
        let input = input(&mut e);
        e.call(
            0x00d3_4810,
            &args![0x11u32, 0x22u32, input, 0x33u32, 0x44u32],
        );
        e.call(0x00d3_4810, &args![0x11u32, 0x22u32, input, 0x33u32, 0u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let with_start = &seen[0];
        assert_eq!(with_start.args[0..2], [0x22, 0x11]);
        // The copy equals the input except for the negated vector at +0x50.
        let mut expected = e.mem.bytes(input, 0x70);
        for lane in 0..4 {
            let at = 0x50 + 4 * lane;
            let word = u32::from_le_bytes(expected[at..at + 4].try_into().unwrap());
            expected[at..at + 4].copy_from_slice(&(word ^ 0x8000_0000).to_le_bytes());
        }
        assert_eq!(with_start.copy, expected);
        // Collector: vtable, far distance, ..., the vector at +0x10, the collector at +0x20.
        let collector = [
            CAST_COLLECTOR_VTABLE,
            0x7f7f_ffee,
            0,
            0,
            0x114,
            0x115,
            0x116,
            0x117,
            0x33,
        ];
        assert_eq!(with_start.first, collector);
        let mut second = collector;
        second[8] = 0x44;
        assert_eq!(with_start.second, second);
        assert_eq!(seen[1].args[4], 0);
        assert_eq!(seen[1].first, collector);
    }

    #[test]
    fn convex_cast_forwards_only_when_the_collector_flag_is_set() {
        let (mut e, seen) = rig();
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let hit = Rc::new(RefCell::new(false));
        let flag = Rc::clone(&hit);
        let collector_seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&collector_seen);
        e.register_double(ITERATIVE_LINEAR_CAST, move |e, a| {
            assert_eq!(a[3], a[4]);
            sink.borrow_mut()
                .push((e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 0x2c)));
            e.mem.set_u8(a[3] + 8, u8::from(*flag.borrow()));
            Ret::default()
        });
        let input = input(&mut e);
        e.call(0x00d3_3ca0, &args![1u32, 2u32, input, 3u32, 4u32]);
        assert!(seen.borrow().is_empty());
        // Start record (16 bytes) and the end record (12 bytes).
        assert_eq!(e.mem.u32(stream + 4), records + 0x10 + 0xc);
        assert_eq!(e.mem.u32(records), LABEL_LINEAR_CAST);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_LINEAR_CAST_DETAIL);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_END);
        *hit.borrow_mut() = true;
        e.call(
            0x00d3_3da0,
            &args![VCALL_THIS, 1u32, 2u32, input, 3u32, 4u32],
        );
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0].args[0..2], [2, 1]);
        assert_eq!(
            collector_seen.borrow()[0],
            (FIRST_COLLECTOR_VTABLE, 0x7f7f_ffee, 0x7f7f_ffee)
        );
        // 16 + 12 + 16 + 12 + 12 bytes in all.
        assert_eq!(
            e.mem.u32(stream + 4),
            records + 0x10 + 0xc + 0x10 + 0xc + 0xc
        );
        assert_eq!(e.mem.u32(records + 0x1c + 0x10), LABEL_CONVEX_CAST);
    }
}

#[cfg(test)]
mod convex_list_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Targets of the allocator, destructor, selector, filter and collector slots the tests
    /// put in their vtables.
    const ALLOC_SLOT: u32 = 0x00e0_1000;
    const FREE_SLOT: u32 = 0x00e0_1004;
    const SELECTOR_SLOT: u32 = 0x00e0_1008;
    const DESTRUCTOR_SLOT: u32 = 0x00e0_100c;
    const FILTER_SLOT: u32 = 0x00e0_1010;
    const COLLECT_SLOT: u32 = 0x00e0_1014;
    /// A signalling NaN: `FLD`/`FSTP` copies come out quiet.
    const SIGNALLING: u32 = 0x7f80_0001;
    const QUIETED: u32 = 0x7fc0_0001;

    /// An engine with the exe's data range and the import page mapped, as the tests run without
    /// the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    fn new_words() -> Words {
        Rc::new(RefCell::new(Vec::new()))
    }

    fn returns(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Registers a double at `addr` that records its call in `log` and then behaves as given.
    fn double(
        e: &mut Engine,
        log: &CallLog,
        addr: u32,
        mut behave: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |e, a| {
            sink.borrow_mut().push((addr, a.to_vec()));
            behave(e, a)
        });
    }

    /// Doubles that only record their calls and return 0.
    fn record_calls(e: &mut Engine, log: &CallLog, addrs: &[u32]) {
        for &addr in addrs {
            double(e, log, addr, |_, _| Ret::default());
        }
    }

    /// The arguments of every call to `addr`, in order.
    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// `count` words from `address`.
    fn snapshot(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    /// The memory router and its allocator: the allocator hands out zeroed blocks (logging
    /// `[allocator, size, block]` under `ALLOC_SLOT`) and logs frees. Returns the router
    /// object and the allocator object.
    fn memory_rig(e: &mut Engine, log: &CallLog) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let heap = e.mem.alloc(8);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(router + 0x10, heap);
        e.mem.set_u32(heap, table);
        e.mem.set_u32(table + 4, ALLOC_SLOT);
        e.mem.set_u32(table + 8, FREE_SLOT);
        double(e, log, MEMORY_ROUTER, move |_, _| returns(router));
        let sink = Rc::clone(log);
        e.register_double(ALLOC_SLOT, move |e, a| {
            let block = e.mem.alloc(a[1]);
            sink.borrow_mut()
                .push((ALLOC_SLOT, vec![a[0], a[1], block]));
            returns(block)
        });
        double(e, log, FREE_SLOT, |_, _| Ret::default());
        (router, heap)
    }

    /// The monitor stream behind `TlsGetValue`: returns the stream and the start of its
    /// 0x200-byte record area.
    fn monitor_rig(e: &mut Engine) -> (u32, u32) {
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x200);
        e.register_double(TLS_GET_VALUE, move |_, _| returns(stream));
        (stream, records)
    }

    /// An agent object with the GSK-mode flag as given.
    fn agent_with_flag(e: &mut Engine, gsk: bool) -> u32 {
        let this = e.mem.alloc(0x100);
        e.mem.set_u8(this + 0x84, u8::from(gsk));
        this
    }

    #[test]
    fn deleting_destructors_store_the_vtable_and_free_on_bit_zero() {
        let mut e = test_engine();
        let log = new_log();
        let (_, heap) = memory_rig(&mut e, &log);
        let this = e.mem.alloc(0x40);
        let kept = e.call(0x00d2_dc10, &args![this, 2u32]);
        assert_eq!(kept.u32(), this);
        assert_eq!(e.mem.u32(this), COLLECTOR_BASE_VTABLE);
        assert!(calls_to(&log, FREE_SLOT).is_empty());
        e.call(0x00d2_dc10, &args![this, 3u32]);
        assert_eq!(calls_to(&log, FREE_SLOT), vec![vec![heap, this, 0x30]]);
        let array = e.call(0x00d3_4900, &args![this, 0u32]);
        assert_eq!(array.u32(), this);
        assert_eq!(e.mem.u32(this), COLLECTOR_ARRAY_VTABLE);
        assert_eq!(calls_to(&log, FREE_SLOT).len(), 1);
        e.call(0x00d3_4900, &args![this, 1u32]);
        assert_eq!(calls_to(&log, FREE_SLOT)[1], vec![heap, this, 8]);
    }

    #[test]
    fn statistics_and_hooks_depend_on_the_mode() {
        let mut e = test_engine();
        let log = new_log();
        record_calls(
            &mut e,
            &log,
            &[
                MACHINE_CALC_STATISTICS,
                MACHINE_INVALIDATE_TIM,
                MACHINE_WARP_TIME,
                GSK_INVALIDATE_TIM,
                GSK_WARP_TIME,
                GSK_REMOVE_POINT,
                GSK_COMMIT_POTENTIAL,
                GSK_CREATE_ZOMBIE,
            ],
        );
        let stats = e.mem.alloc(0x10);
        e.mem.set_u32(stats + 8, 0x1234);
        let one_and_half = 1.5f32.to_bits();
        let multi = agent_with_flag(&mut e, false);
        e.call(0x00d3_3740, &args![multi, stats, 0u32]);
        e.call(0x00d3_3770, &args![multi, 7u32]);
        e.call(0x00d3_37a0, &args![multi, one_and_half, SIGNALLING, 9u32]);
        for hook in [0x00d3_37f0u32, 0x00d3_3810, 0x00d3_3830] {
            e.call(hook, &args![multi, 5u32]);
        }
        assert_eq!(
            calls_to(&log, MACHINE_CALC_STATISTICS),
            vec![vec![multi + 0x30, 0x1234, stats]]
        );
        assert_eq!(
            calls_to(&log, MACHINE_INVALIDATE_TIM),
            vec![vec![multi + 0x30, 7]]
        );
        assert_eq!(
            calls_to(&log, MACHINE_WARP_TIME),
            vec![vec![multi + 0x30, one_and_half, QUIETED, 9]]
        );
        for hook in [
            GSK_INVALIDATE_TIM,
            GSK_WARP_TIME,
            GSK_REMOVE_POINT,
            GSK_COMMIT_POTENTIAL,
            GSK_CREATE_ZOMBIE,
        ] {
            assert!(calls_to(&log, hook).is_empty());
        }

        let gsk = agent_with_flag(&mut e, true);
        e.call(0x00d3_3740, &args![gsk, stats, 0u32]);
        e.call(0x00d3_3770, &args![gsk, 7u32]);
        e.call(0x00d3_37a0, &args![gsk, one_and_half, SIGNALLING, 9u32]);
        for hook in [0x00d3_37f0u32, 0x00d3_3810, 0x00d3_3830] {
            e.call(hook, &args![gsk, 5u32]);
        }
        assert_eq!(calls_to(&log, MACHINE_CALC_STATISTICS).len(), 1);
        assert_eq!(calls_to(&log, MACHINE_INVALIDATE_TIM).len(), 1);
        assert_eq!(calls_to(&log, MACHINE_WARP_TIME).len(), 1);
        assert_eq!(calls_to(&log, GSK_INVALIDATE_TIM), vec![vec![gsk, 7]]);
        assert_eq!(
            calls_to(&log, GSK_WARP_TIME),
            vec![vec![gsk, one_and_half, QUIETED, 9]]
        );
        assert_eq!(calls_to(&log, GSK_REMOVE_POINT), vec![vec![gsk, 5]]);
        assert_eq!(calls_to(&log, GSK_COMMIT_POTENTIAL), vec![vec![gsk, 5]]);
        assert_eq!(calls_to(&log, GSK_CREATE_ZOMBIE), vec![vec![gsk, 5]]);
    }

    #[test]
    fn leaving_multi_contact_mode_clears_the_machine() {
        let mut e = test_engine();
        let log = new_log();
        record_calls(&mut e, &log, &[MACHINE_CLEAR_AND_DESTROY]);
        let this = agent_with_flag(&mut e, false);
        e.mem.set_u32(this + 8, 0x11);
        e.mem.set_u32(this + 0x80, 0x22);
        e.mem.set_u32(this + 0x30, 0x99);
        e.call(0x00d3_3850, &args![this, 5u32]);
        assert_eq!(
            calls_to(&log, MACHINE_CLEAR_AND_DESTROY),
            vec![vec![this + 0x30, 0x22, 0x11, 5]]
        );
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u8(this + 0x84), 1);
    }

    #[test]
    fn re_entering_multi_contact_mode_resets_the_machine() {
        let mut e = test_engine();
        let log = new_log();
        record_calls(&mut e, &log, &[MACHINE_CLEANUP, MACHINE_RESET]);
        let this = agent_with_flag(&mut e, true);
        e.mem.set_u32(this + 8, 0x11);
        e.mem.set_u32(this + 0x3c, 0x4000_0000);
        e.call(0x00d3_3ef0, &args![this, 6u32]);
        assert_eq!(
            calls_to(&log, MACHINE_CLEANUP),
            vec![vec![this + 0x30, 0x11, 6]]
        );
        assert_eq!(calls_to(&log, MACHINE_RESET), vec![vec![this + 0x30]]);
        assert_eq!(e.mem.u8(this + 0x84), 0);
        assert_eq!(snapshot(&e, this + 0x30, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u16(this + 0x86), 0x19);
    }

    #[test]
    fn cleanup_destroys_through_the_vtable() {
        let mut e = test_engine();
        let log = new_log();
        record_calls(
            &mut e,
            &log,
            &[MACHINE_CLEAR_AND_DESTROY, MACHINE_CLEANUP, DESTRUCTOR_SLOT],
        );
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, DESTRUCTOR_SLOT);
        for gsk in [false, true] {
            let this = agent_with_flag(&mut e, gsk);
            e.mem.set_u32(this, vtable);
            e.mem.set_u32(this + 8, 0x11);
            e.mem.set_u32(this + 0x80, 0x22);
            log.borrow_mut().clear();
            e.call(0x00d3_3890, &args![this, 8u32]);
            if gsk {
                assert_eq!(
                    calls_to(&log, MACHINE_CLEANUP),
                    vec![vec![this + 0x30, 0x11, 8]]
                );
                assert!(calls_to(&log, MACHINE_CLEAR_AND_DESTROY).is_empty());
            } else {
                assert_eq!(
                    calls_to(&log, MACHINE_CLEAR_AND_DESTROY),
                    vec![vec![this + 0x30, 0x22, 0x11, 8]]
                );
                assert!(calls_to(&log, MACHINE_CLEANUP).is_empty());
            }
            assert_eq!(calls_to(&log, DESTRUCTOR_SLOT), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn shape_collection_filter_builds_the_record_outside_gsk_mode() {
        let mut e = test_engine();
        let log = new_log();
        let records = new_words();
        let sink = Rc::clone(&records);
        double(&mut e, &log, MACHINE_UPDATE_FILTER, move |e, a| {
            sink.borrow_mut().push(snapshot(e, a[1], 7));
            Ret::default()
        });
        double(&mut e, &log, FILTER_SLOT, |_, _| returns(0x55));
        let shape = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FILTER_SLOT);
        e.mem.set_u32(shape, vtable);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, shape);
        let first = 0x1000u32;
        let gsk = agent_with_flag(&mut e, true);
        e.call(0x00d3_38e0, &args![gsk, first, second, 3u32, 4u32]);
        assert!(records.borrow().is_empty());
        let multi = agent_with_flag(&mut e, false);
        e.mem.set_u32(multi + 8, 0x77);
        e.call(0x00d3_38e0, &args![multi, first, second, 3u32, 4u32]);
        assert_eq!(
            *records.borrow(),
            vec![vec![first, second, 0, 0x55, 3, 0x77, 4]]
        );
        assert_eq!(calls_to(&log, MACHINE_UPDATE_FILTER)[0][0], multi + 0x30);
        assert_eq!(calls_to(&log, FILTER_SLOT), vec![vec![shape]]);
        // The symmetric stub swaps the first two words.
        records.borrow_mut().clear();
        let swapped = e.mem.alloc(8);
        e.mem.set_u32(swapped, shape);
        e.call(0x00d3_4aa0, &args![multi, swapped, first, 3u32, 4u32]);
        assert_eq!(
            *records.borrow(),
            vec![vec![first, swapped, 0, 0x55, 3, 0x77, 4]]
        );
    }

    #[test]
    fn constructor_initialises_the_agent() {
        let mut e = test_engine();
        let log = new_log();
        record_calls(
            &mut e,
            &log,
            &[GSK_AGENT_INIT, SET_MUL_INVERSE_MUL, CACHE_INIT],
        );
        e.set_global(MINUS_ONE, SIGNALLING);
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a, 0xaaa0);
        e.mem.set_u32(body_a + 8, 0xaaa8);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b, 0xbbb0);
        e.mem.set_u32(body_b + 8, 0xbbb8);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x4242);
        let this = e.mem.alloc(0x100);
        e.mem.set_u8(this + 0x85, 7);
        let made = e.call(0x00d3_3940, &args![this, body_a, body_b, pair, 9u32]);
        assert_eq!(made.u32(), this);
        assert_eq!(e.mem.u32(this), CONVEX_LIST_AGENT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x80), 0x4242);
        assert_eq!(e.mem.u8(this + 0x84), 1);
        assert_eq!(e.mem.u8(this + 0x85), 0);
        assert_eq!(e.mem.u32(this + 0x2c), QUIETED);
        assert_eq!(e.mem.u32(this + 0x18), QUIETED);
        assert_eq!(
            calls_to(&log, GSK_AGENT_INIT),
            vec![vec![this, body_a, body_b, 9]]
        );
        let relative = calls_to(&log, SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [0xaaa8, 0xbbb8]);
        assert_eq!(
            calls_to(&log, CACHE_INIT),
            vec![vec![this + 0xc, 0xaaa0, 0xbbb0, relative[0][0]]]
        );
    }

    #[test]
    fn simple_collector_reset_restores_the_far_distance() {
        let mut e = test_engine();
        e.set_global(FAR_DISTANCE, SIGNALLING);
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 8, 1);
        e.call(0x00d3_4770, &args![this]);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x2c), QUIETED);
        assert_eq!(e.mem.u32(this + 4), QUIETED);
    }

    #[test]
    fn registration_hands_the_three_tables_to_the_registry() {
        let mut e = test_engine();
        let log = new_log();
        let tables = new_words();
        let sink = Rc::clone(&tables);
        double(&mut e, &log, REGISTER_AGENT_TABLE, move |e, a| {
            let mut words = snapshot(e, a[1], 4);
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d3_46b0, &args![0x5000u32]);
        let calls = calls_to(&log, REGISTER_AGENT_TABLE);
        assert_eq!(calls.len(), 3);
        assert_eq!(
            calls.iter().map(|c| (c[0], c[2], c[3])).collect::<Vec<_>>(),
            vec![
                (0x5000, 0x16, 0x12),
                (0x5000, 0x12, 0x16),
                (0x5000, 0x16, 0x16)
            ]
        );
        assert_eq!(
            *tables.borrow(),
            vec![
                vec![0x00d3_3dd0, 0x00d3_4930, 0x00d3_4970, 0x00d3_49b0, 1, 1],
                vec![0x00d3_43f0, 0x00d3_3b90, 0x00d3_39d0, 0x00d3_3ca0, 0, 1],
                vec![0x00d3_4510, 0x00d3_3b90, 0x00d3_39d0, 0x00d3_3ca0, 0, 1],
            ]
        );
    }

    /// What the collectors and walkers saw during a static query.
    struct QueryRun {
        /// Arguments of every `00dac450` call, then the header of its collector (vtable, +4,
        /// +0x2C, +0x30) before the double filled it in.
        closest: Vec<Vec<u32>>,
        header: Vec<u32>,
        /// The ten words of each record the collector received.
        collected: Vec<Vec<u32>>,
        /// Arguments and wrapper words of every children walk.
        children: Vec<Vec<u32>>,
        /// Bytes of timer records written.
        used: u32,
        /// The four words the query was called with.
        ids: [u32; 4],
    }

    /// Runs `staticGetClosestPoints` with the penetration test flagging `flagged`, the
    /// closest-point test finding `count` hits at `distance_bits` and the first body's
    /// collidable limit at `limit_bits`.
    fn run_closest(flagged: bool, count: u32, limit_bits: u32, distance_bits: u32) -> QueryRun {
        let mut e = test_engine();
        let log = new_log();
        let (stream, records) = monitor_rig(&mut e);
        e.set_global(FAR_DISTANCE, SIGNALLING);
        double(&mut e, &log, GSK_STATIC_GET_PENETRATIONS, move |e, a| {
            e.mem.set_u8(a[3] + 4, u8::from(flagged));
            Ret::default()
        });
        let header = new_words();
        let sink = Rc::clone(&header);
        double(&mut e, &log, GSK_STATIC_GET_CLOSEST_POINTS, move |e, a| {
            let collector = a[3];
            sink.borrow_mut().push(vec![
                e.mem.u32(collector),
                e.mem.u32(collector + 4),
                e.mem.u32(collector + 0x2c),
                e.mem.u32(collector + 0x30),
            ]);
            e.mem.set_u32(collector + 0x30, count);
            for i in 0..8 {
                e.mem.set_u32(collector + 0x10 + 4 * i, 0x100 + i);
            }
            // The distance is the last lane of the second vector.
            e.mem.set_u32(collector + 0x2c, distance_bits);
            Ret::default()
        });
        let collected = new_words();
        let sink = Rc::clone(&collected);
        double(&mut e, &log, COLLECT_SLOT, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(snapshot(e, a[1], 10));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let children = new_words();
        let sink = Rc::clone(&children);
        double(&mut e, &log, CHILDREN_CLOSEST_POINTS, move |e, a| {
            let mut words = a.to_vec();
            words.extend(snapshot(e, a[3], 3));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let collidable = e.mem.alloc(0x40);
        e.mem.set_u32(b, collidable);
        e.mem.set_u32(collidable + 0x18, limit_bits);
        let input = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, COLLECT_SLOT);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.call(0x00d3_39d0, &args![a, b, input, collector]);
        let used = e.mem.u32(stream + 4) - records;
        assert_eq!(e.mem.u32(records), LABEL_STATIC_LIST);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_CHECK_HULL);
        let closest = calls_to(&log, GSK_STATIC_GET_CLOSEST_POINTS);
        let seen_header = header.borrow().first().cloned().unwrap_or_default();
        let collected = collected.borrow().clone();
        let children = children.borrow().clone();
        QueryRun {
            closest,
            header: seen_header,
            collected,
            children,
            used,
            ids: [a, b, input, collector],
        }
    }

    #[test]
    fn closest_points_report_the_nearest_hit_to_the_collector() {
        let near = 2.0f32.to_bits();
        let run = run_closest(false, 2, 1.0f32.to_bits(), near);
        let [a, b, input, collector] = run.ids;
        assert_eq!(run.closest.len(), 1);
        assert_eq!(run.closest[0][..3], [a, b, input]);
        assert_eq!(run.header[0], CLOSEST_POINT_VTABLE);
        assert_eq!(run.header[1..], [QUIETED, QUIETED, 0]);
        let mut record = vec![collector];
        record.extend((0..7).map(|i| 0x100 + i));
        record.extend([near, a, b]);
        assert_eq!(run.collected, vec![record]);
        assert!(run.children.is_empty());
        // Start record (16 bytes) and the end record (12).
        assert_eq!(run.used, 0x1c);
    }

    #[test]
    fn closest_points_walk_the_children_when_the_limit_is_not_below_the_distance() {
        for (limit, distance) in [
            (3.0f32.to_bits(), 2.0f32.to_bits()),
            (2.0f32.to_bits(), 2.0f32.to_bits()),
            (1.0f32.to_bits(), f32::NAN.to_bits()),
            (f32::NAN.to_bits(), 2.0f32.to_bits()),
        ] {
            let run = run_closest(false, 1, limit, distance);
            let [a, b, input, collector] = run.ids;
            assert!(run.collected.is_empty());
            assert_eq!(run.children.len(), 1);
            let walk = &run.children[0];
            // The walk gets the bodies swapped and a wrapper holding the far distance and the
            // collector.
            assert_eq!(walk[..3], [b, a, input]);
            assert_eq!(walk[4..], [WRAPPED_CLOSEST_VTABLE, QUIETED, collector]);
            // Start, children and end records.
            assert_eq!(run.used, 0x10 + 0xc + 0xc);
        }
    }

    #[test]
    fn closest_points_end_without_hits_and_walk_children_when_flagged() {
        let none = run_closest(false, 0, 1.0f32.to_bits(), 2.0f32.to_bits());
        assert!(none.collected.is_empty());
        assert!(none.children.is_empty());
        assert_eq!(none.used, 0x10 + 0xc);
        let flagged = run_closest(true, 5, 1.0f32.to_bits(), 2.0f32.to_bits());
        assert!(flagged.closest.is_empty());
        assert!(flagged.header.is_empty());
        assert_eq!(flagged.children.len(), 1);
        assert_eq!(flagged.used, 0x10 + 0xc + 0xc);
    }

    #[test]
    fn penetration_queries_flag_children() {
        for flagged in [false, true] {
            let mut e = test_engine();
            let log = new_log();
            let (stream, records) = monitor_rig(&mut e);
            let flag_collectors = new_words();
            let sink = Rc::clone(&flag_collectors);
            double(&mut e, &log, GSK_STATIC_GET_PENETRATIONS, move |e, a| {
                sink.borrow_mut()
                    .push(vec![e.mem.u32(a[3]), u32::from(e.mem.u8(a[3] + 4))]);
                e.mem.set_u8(a[3] + 4, u8::from(flagged));
                Ret::default()
            });
            let walks = new_words();
            let sink = Rc::clone(&walks);
            double(&mut e, &log, CHILDREN_PENETRATIONS, move |e, a| {
                let mut words = a.to_vec();
                words.push(e.mem.u32(a[3]));
                words.push(u32::from(e.mem.u8(a[3] + 4)));
                words.push(e.mem.u32(a[3] + 8));
                sink.borrow_mut().push(words);
                Ret::default()
            });
            let (a, b, input, collector) = (0x1001u32, 0x1002u32, 0x1003u32, 0x1004u32);
            e.call(0x00d3_3b90, &args![a, b, input, collector]);
            e.call(0x00d3_3c80, &args![0u32, a, b, input, collector]);
            let gsk = calls_to(&log, GSK_STATIC_GET_PENETRATIONS);
            assert_eq!(gsk.len(), 2);
            for call in &gsk {
                assert_eq!(call[..3], [a, b, input]);
            }
            for seen in flag_collectors.borrow().iter() {
                assert_eq!(*seen, vec![PENETRATION_FLAG_VTABLE, 0]);
            }
            assert_eq!(e.mem.u32(records), LABEL_STATIC_LIST);
            assert_eq!(e.mem.u32(records + 0xc), LABEL_CHECK_HULL);
            let used = e.mem.u32(stream + 4) - records;
            if flagged {
                assert_eq!(walks.borrow().len(), 2);
                for walk in walks.borrow().iter() {
                    assert_eq!(walk[..3], [b, a, input]);
                    assert_eq!(walk[4..], [WRAPPED_PENETRATION_VTABLE, 0, collector]);
                }
                assert_eq!(used, 2 * (0x10 + 0xc + 0xc));
            } else {
                assert!(walks.borrow().is_empty());
                assert_eq!(used, 2 * (0x10 + 0xc));
            }
        }
    }

    #[test]
    fn child_walk_stubs_wrap_the_collector() {
        let mut e = test_engine();
        let log = new_log();
        let walks = new_words();
        for walker in [CHILDREN_PENETRATIONS, CHILDREN_CLOSEST_POINTS] {
            let sink = Rc::clone(&walks);
            double(&mut e, &log, walker, move |e, a| {
                let mut words = a.to_vec();
                words.extend(snapshot(e, a[3], 3));
                sink.borrow_mut().push(words);
                Ret::default()
            });
        }
        e.set_global(FAR_DISTANCE, SIGNALLING);
        e.call(0x00d3_4790, &args![1u32, 2u32, 3u32, 4u32]);
        e.call(0x00d3_47d0, &args![1u32, 2u32, 3u32, 4u32]);
        let walks = walks.borrow();
        assert_eq!(walks.len(), 2);
        assert_eq!(walks[0][..3], [2, 1, 3]);
        assert_eq!(walks[0][4..], [WRAPPED_PENETRATION_VTABLE, 0, 4]);
        assert_eq!(walks[1][..3], [2, 1, 3]);
        assert_eq!(walks[1][4..], [WRAPPED_CLOSEST_VTABLE, QUIETED, 4]);
    }

    #[test]
    fn table_stubs_wrap_the_collector_and_swap_the_bodies() {
        let mut e = test_engine();
        let log = new_log();
        monitor_rig(&mut e);
        e.set_global(FAR_DISTANCE, SIGNALLING);
        let walks = new_words();
        // The penetration stubs reach the walkers through a flagged first stage, the
        // closest-point stubs likewise; each wrapper is read through the walker's wrapper.
        double(&mut e, &log, GSK_STATIC_GET_PENETRATIONS, |e, a| {
            e.mem.set_u8(a[3] + 4, 1);
            Ret::default()
        });
        for walker in [CHILDREN_PENETRATIONS, CHILDREN_CLOSEST_POINTS] {
            let sink = Rc::clone(&walks);
            double(&mut e, &log, walker, move |e, a| {
                let mut words = a.to_vec();
                // The outer wrapper (this stub's) is the collector the walker's wrapper holds.
                let inner = e.mem.u32(a[3] + 8);
                words.extend(snapshot(e, inner, 3));
                sink.borrow_mut().push(words);
                Ret::default()
            });
        }
        let (a, b, input, collector) = (0x1001u32, 0x1002u32, 0x1003u32, 0x1004u32);
        e.call(0x00d3_4930, &args![a, b, input, collector]);
        e.call(0x00d3_4970, &args![a, b, input, collector]);
        let walks = walks.borrow();
        assert_eq!(walks.len(), 2);
        // 00d34930: staticGetPenetrations(b, a, input, wrapper) then the walk (a, b, input, ..).
        assert_eq!(walks[0][..3], [a, b, input]);
        assert_eq!(walks[0][4..], [WRAPPED_PENETRATION_VTABLE, 0, collector]);
        // 00d34970: staticGetClosestPoints(b, a, input, wrapper), flagged, walk (a, b, input, ..).
        assert_eq!(walks[1][..3], [a, b, input]);
        assert_eq!(walks[1][4..], [WRAPPED_CLOSEST_VTABLE, QUIETED, collector]);
    }

    #[test]
    fn symmetric_stubs_swap_the_bodies_and_wrap_the_collector() {
        let mut e = test_engine();
        let log = new_log();
        monitor_rig(&mut e);
        e.set_global(FAR_DISTANCE, SIGNALLING);
        let wrappers = new_words();
        let sink = Rc::clone(&wrappers);
        double(&mut e, &log, GSK_STATIC_GET_PENETRATIONS, move |e, a| {
            sink.borrow_mut().push(vec![a[0], a[1], a[2]]);
            e.mem.set_u8(a[3] + 4, 1);
            Ret::default()
        });
        let walks = new_words();
        for walker in [CHILDREN_PENETRATIONS, CHILDREN_CLOSEST_POINTS] {
            let sink = Rc::clone(&walks);
            double(&mut e, &log, walker, move |e, a| {
                let mut words = a.to_vec();
                let inner = e.mem.u32(a[3] + 8);
                words.extend(snapshot(e, inner, 3));
                sink.borrow_mut().push(words);
                Ret::default()
            });
        }
        let (a, b, input, collector) = (0x1001u32, 0x1002u32, 0x1003u32, 0x1004u32);
        e.call(0x00d3_4ac0, &args![0u32, a, b, input, collector]);
        e.call(0x00d3_4b00, &args![0u32, a, b, input, collector]);
        // Both reach the penetration test with the bodies swapped.
        assert_eq!(*wrappers.borrow(), vec![vec![b, a, input]; 2]);
        let walks = walks.borrow();
        assert_eq!(walks.len(), 2);
        // getPenetrations(b, a, input, wrapper): the walk swaps back to (a, b).
        assert_eq!(walks[0][..3], [a, b, input]);
        assert_eq!(walks[0][4..], [WRAPPED_PENETRATION_VTABLE, 0, collector]);
        assert_eq!(walks[1][..3], [a, b, input]);
        assert_eq!(walks[1][4..], [WRAPPED_CLOSEST_VTABLE, QUIETED, collector]);
    }

    /// Bodies and a selector object for the agent factories.
    struct FactoryRig {
        e: Engine,
        log: CallLog,
        first: u32,
        second: u32,
        third: u32,
        owner: u32,
        selectors: Rc<RefCell<Vec<u32>>>,
    }

    fn factory_rig() -> FactoryRig {
        let mut e = test_engine();
        let log = new_log();
        memory_rig(&mut e, &log);
        record_calls(
            &mut e,
            &log,
            &[GSK_AGENT_INIT, SET_MUL_INVERSE_MUL, CACHE_INIT],
        );
        double(&mut e, &log, SIMPLE_AGENT_INIT, |_, _| returns(0x7777));
        let selectors = Rc::new(RefCell::new(Vec::new()));
        let queue = Rc::clone(&selectors);
        double(&mut e, &log, SELECTOR_SLOT, move |_, _| {
            let mut queue = queue.borrow_mut();
            returns(if queue.is_empty() {
                99
            } else {
                queue.remove(0)
            })
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SELECTOR_SLOT);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, vtable);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third + 0x10, owner);
        FactoryRig {
            e,
            log,
            first,
            second,
            third,
            owner,
            selectors,
        }
    }

    impl FactoryRig {
        /// Calls the factory with the given selector answers and `fourth`.
        fn run(&mut self, factory: u32, fourth: u32, answers: &[u32]) -> u32 {
            *self.selectors.borrow_mut() = answers.to_vec();
            self.log.borrow_mut().clear();
            let (a, b, c) = (self.first, self.second, self.third);
            self.e.call(factory, &args![a, b, c, fourth]).u32()
        }

        /// The one block the last run allocated, checked for its size and size word; `None`
        /// when it allocated nothing.
        fn block(&self, size: u32) -> Option<u32> {
            let blocks = calls_to(&self.log, ALLOC_SLOT);
            if blocks.is_empty() {
                return None;
            }
            assert_eq!(blocks.len(), 1);
            assert_eq!(blocks[0][1], size);
            assert_eq!(self.e.mem.u16(blocks[0][2] + 4), size as u16);
            Some(blocks[0][2])
        }

        fn selector_calls(&self) -> Vec<Vec<u32>> {
            calls_to(&self.log, SELECTOR_SLOT)
        }
    }

    #[test]
    fn factory_00d33dd0_picks_the_agent_by_the_selector() {
        let mut rig = factory_rig();
        let (a1, a2, a3, owner) = (rig.first, rig.second, rig.third, rig.owner);
        let made = rig.run(0x00d3_3dd0, 0, &[]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, 0x7777);
        assert!(rig.selector_calls().is_empty());
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a1, a2, a3, 0]]
        );

        let made = rig.run(0x00d3_3dd0, 9, &[0]);
        let block = rig.block(0x90).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.selector_calls(), vec![vec![owner, a1, a2, a3]]);
        assert_eq!(rig.e.mem.u32(block), CONVEX_LIST_FACTORY_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a2, a1, 9]]
        );
        assert_eq!(rig.e.mem.u8(block + 0x84), 1);

        let made = rig.run(0x00d3_3dd0, 9, &[1]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, 0x7777);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a1, a2, a3, 9]]
        );

        let made = rig.run(0x00d3_3dd0, 9, &[2]);
        let block = rig.block(0x80).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), GSKF_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a1, a2, 9]]
        );

        assert_eq!(rig.run(0x00d3_3dd0, 9, &[3]), 0);
        assert!(rig.block(0).is_none());
        assert_eq!(rig.run(0x00d3_3dd0, 9, &[0xffff_ffff]), 0);
    }

    #[test]
    fn factory_00d343f0_swaps_the_bodies_of_the_selector_and_the_simple_agent() {
        let mut rig = factory_rig();
        let (a1, a2, a3, owner) = (rig.first, rig.second, rig.third, rig.owner);
        let made = rig.run(0x00d3_43f0, 0, &[]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), SIMPLE_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a2, a1, a3, 0]]
        );

        let made = rig.run(0x00d3_43f0, 9, &[0]);
        let block = rig.block(0x90).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.selector_calls(), vec![vec![owner, a2, a1, a3]]);
        // The constructor's own vtable stays.
        assert_eq!(rig.e.mem.u32(block), CONVEX_LIST_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a1, a2, 9]]
        );

        let made = rig.run(0x00d3_43f0, 9, &[1]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), SIMPLE_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a2, a1, a3, 9]]
        );

        let made = rig.run(0x00d3_43f0, 9, &[2]);
        let block = rig.block(0x80).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), GSKF_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a1, a2, 9]]
        );

        assert_eq!(rig.run(0x00d3_43f0, 9, &[7]), 0);
        assert!(rig.block(0).is_none());
    }

    #[test]
    fn factory_00d34510_asks_the_selector_twice_for_kind_two() {
        let mut rig = factory_rig();
        let (a1, a2, a3, owner) = (rig.first, rig.second, rig.third, rig.owner);
        let made = rig.run(0x00d3_4510, 0, &[]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, 0x7777);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a1, a2, a3, 0]]
        );

        let made = rig.run(0x00d3_4510, 9, &[0]);
        let block = rig.block(0x90).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), CONVEX_LIST_FACTORY_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a2, a1, 9]]
        );

        let made = rig.run(0x00d3_4510, 9, &[1]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, 0x7777);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a1, a2, a3, 9]]
        );

        let made = rig.run(0x00d3_4510, 9, &[2, 0]);
        let block = rig.block(0x90).unwrap();
        assert_eq!(made, block);
        assert_eq!(
            rig.selector_calls(),
            vec![vec![owner, a1, a2, a3], vec![owner, a2, a1, a3]]
        );
        assert_eq!(rig.e.mem.u32(block), CONVEX_LIST_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a1, a2, 9]]
        );

        let made = rig.run(0x00d3_4510, 9, &[2, 1]);
        let block = rig.block(0x38).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), SIMPLE_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, SIMPLE_AGENT_INIT),
            vec![vec![block, a2, a1, a3, 9]]
        );

        let made = rig.run(0x00d3_4510, 9, &[2, 2]);
        let block = rig.block(0x80).unwrap();
        assert_eq!(made, block);
        assert_eq!(rig.e.mem.u32(block), GSKF_AGENT_VTABLE);
        assert_eq!(
            calls_to(&rig.log, GSK_AGENT_INIT),
            vec![vec![block, a1, a2, 9]]
        );

        assert_eq!(rig.run(0x00d3_4510, 9, &[2, 3]), 0);
        assert!(rig.block(0).is_none());
        assert_eq!(rig.run(0x00d3_4510, 9, &[5]), 0);
    }

    #[test]
    fn cast_stubs_build_the_negated_copy_and_the_collectors() {
        for addr in [0x00d3_49b0u32, 0x00d3_4b40] {
            for forwarded in [false, true] {
                for start in [0u32, 0x4444] {
                    let mut e = test_engine();
                    let log = new_log();
                    monitor_rig(&mut e);
                    e.set_global(FAR_DISTANCE, SIGNALLING);
                    for lane in 0..4 {
                        e.set_global(SIGN_MASK + 4 * lane, 0x8000_0000u32);
                    }
                    let casts = new_words();
                    let sink = Rc::clone(&casts);
                    double(&mut e, &log, ITERATIVE_LINEAR_CAST, move |e, a| {
                        let mut words = a.to_vec();
                        words.extend(snapshot(e, a[2], 0x1c));
                        sink.borrow_mut().push(words);
                        e.mem.set_u8(a[3] + 8, u8::from(forwarded));
                        Ret::default()
                    });
                    let forwards = new_words();
                    let sink = Rc::clone(&forwards);
                    double(&mut e, &log, CAST_FORWARD, move |e, a| {
                        let mut words = a.to_vec();
                        words.extend(snapshot(e, a[2], 0x1c));
                        // The stub's collectors, reached through the forwarding collectors.
                        words.extend(snapshot(e, e.mem.u32(a[3] + 0x20), 9));
                        if a[4] != 0 {
                            words.extend(snapshot(e, e.mem.u32(a[4] + 0x20), 9));
                        }
                        sink.borrow_mut().push(words);
                        Ret::default()
                    });
                    let input = e.mem.alloc(0x70);
                    for i in 0..0x1c {
                        e.mem.set_u32(input + 4 * i, 0x100 + i);
                    }
                    let (a1, a2, cast_collector) = (0x1001u32, 0x1002u32, 0x3333u32);
                    if addr == 0x00d3_49b0 {
                        e.call(addr, &args![a1, a2, input, cast_collector, start]);
                    } else {
                        e.call(addr, &args![0u32, a1, a2, input, cast_collector, start]);
                    }

                    let casts = casts.borrow();
                    assert_eq!(casts.len(), 1);
                    let cast = &casts[0];
                    assert_eq!(cast[..2], [a2, a1]);
                    assert_eq!(cast[3], cast[4]);
                    // The copy has the vector at +0x50 negated lane by lane.
                    let mut copy: Vec<u32> = (0..0x1c).map(|i| 0x100 + i).collect();
                    for word in &mut copy[0x14..0x18] {
                        *word ^= 0x8000_0000;
                    }
                    assert_eq!(cast[5..], copy[..]);
                    let vector: Vec<u32> = (0x114..0x118).collect();
                    let mut first_collector = vec![CAST_COLLECTOR_VTABLE, QUIETED, 0, 0];
                    first_collector.extend(&vector);
                    first_collector.push(cast_collector);
                    let mut second_collector = vec![CAST_COLLECTOR_VTABLE, QUIETED, 0, 0];
                    second_collector.extend(&vector);
                    second_collector.push(start);

                    let forwards = forwards.borrow();
                    if !forwarded {
                        assert!(forwards.is_empty());
                        continue;
                    }
                    assert_eq!(forwards.len(), 1);
                    let forward = &forwards[0];
                    assert_eq!(forward[..2], [a1, a2]);
                    if start == 0 {
                        assert_eq!(forward[4], 0);
                    } else {
                        assert_ne!(forward[4], 0);
                    }
                    // Negated twice, the second copy equals the input again.
                    let original: Vec<u32> = (0..0x1c).map(|i| 0x100 + i).collect();
                    assert_eq!(forward[5..5 + 0x1c], original[..]);
                    assert_eq!(forward[5 + 0x1c..5 + 0x1c + 9], first_collector[..]);
                    if start != 0 {
                        assert_eq!(forward[5 + 0x1c + 9..], second_collector[..]);
                    } else {
                        assert_eq!(forward.len(), 5 + 0x1c + 9);
                    }
                }
            }
        }
    }

    /// The owner object word at `this + 8` and the key word at `result + 4` the process tests use.
    const OWNER: u32 = 0x5150;
    const KEY: u32 = 0x4444;

    /// Two bodies, an agent, the cast input and the result block `processCollision` works on,
    /// with doubles for everything it calls.
    struct ProcessRig {
        e: Engine,
        log: CallLog,
        stream: u32,
        records: u32,
        router: u32,
        arena: u32,
        this: u32,
        a: u32,
        b: u32,
        info: u32,
        result: u32,
        transform_a: u32,
        collidable_b: u32,
    }

    impl ProcessRig {
        fn new() -> Self {
            let mut e = test_engine();
            let log = new_log();
            let (stream, records) = monitor_rig(&mut e);
            let (router, _) = memory_rig(&mut e, &log);
            let arena = e.mem.alloc(0x400);
            e.mem.set_u32(router, arena);
            record_calls(
                &mut e,
                &log,
                &[
                    LOCAL_OBJECT_INIT,
                    LOCAL_OBJECT_ADD,
                    SET_MUL_INVERSE_MUL,
                    GSK_GET_CLOSEST_POINT,
                    PREDICTIVE_STEP,
                    MACHINE_CLEANUP,
                    MACHINE_CLEAR_AND_DESTROY,
                    MACHINE_PROCESS,
                    MACHINE_RESET,
                ],
            );
            let this = e.mem.alloc(0x100);
            e.mem.set_u32(this + 8, OWNER);
            e.mem.set_u32(this + 0x80, 0x8080);
            let transform_a = e.mem.alloc(0xb0);
            let transform_b = e.mem.alloc(0xb0);
            let collidable_a = e.mem.alloc(0x80);
            let collidable_b = e.mem.alloc(0x80);
            e.mem.set_u32(collidable_b + 0x48, 2);
            let a = e.mem.alloc(0x60);
            e.mem.set_u32(a, collidable_a);
            e.mem.set_u32(a + 8, transform_a);
            let b = e.mem.alloc(0x60);
            e.mem.set_u32(b, collidable_b);
            e.mem.set_u32(b + 8, transform_b);
            let info = e.mem.alloc(0x70);
            let result = e.mem.alloc(0x3100);
            e.mem.set_u32(result + 4, KEY);
            ProcessRig {
                e,
                log,
                stream,
                records,
                router,
                arena,
                this,
                a,
                b,
                info,
                result,
                transform_a,
                collidable_b,
            }
        }

        fn run(&mut self) {
            let (this, a, b, info, result) = (self.this, self.a, self.b, self.info, self.result);
            self.e.call(0x00d3_3f40, &args![this, a, b, info, result]);
        }

        /// The exe frame's lowest address, from the argument of the first local-object call.
        fn frame(&self) -> u32 {
            calls_to(&self.log, LOCAL_OBJECT_INIT)[0][0] - 0x114
        }

        /// Bytes of timer records written.
        fn used(&self) -> u32 {
            self.e.mem.u32(self.stream + 4) - self.records
        }

        /// GSK mode with the given distance (+0x2C) and cast reach (`info + 8`).
        fn enter_gsk_mode(&mut self, distance: f32, reach: f32) {
            self.e.mem.set_u8(self.this + 0x84, 1);
            self.e.mem.set_f32(self.this + 0x2c, distance);
            self.e.mem.set_f32(self.info + 8, reach);
        }

        /// Motion that makes the relative vector `(2, 0, 0, 1)` and the contact normal `(1, 0,
        /// 0, 0)`: a projection of 3.
        fn set_motion(&mut self) {
            let ta = self.transform_a;
            self.e.mem.set_f32(ta + 0x40, 1.0);
            self.e.mem.set_f32(ta + 0x5c, 2.0);
            self.e.mem.set_f32(ta + 0x9c, 1.0);
            self.e.mem.set_f32(ta + 0xa0, 0.5);
            self.e.mem.set_f32(self.info + 0x58, 1.0);
            self.e.mem.set_f32(self.this + 0x20, 1.0);
        }

        fn machine_arguments(&self) -> Vec<Vec<u32>> {
            calls_to(&self.log, MACHINE_PROCESS)
        }
    }

    /// Replaces the machine double with one that keeps the router word, the 32-word record and the
    /// first three index words it saw.
    fn capture_machine(rig: &mut ProcessRig) -> Words {
        let seen = new_words();
        let sink = Rc::clone(&seen);
        let router = rig.router;
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, MACHINE_PROCESS, move |e, a| {
            let mut words = vec![e.mem.u32(router)];
            words.extend(snapshot(e, a[1], 0x20));
            words.extend(snapshot(e, a[3], 3));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        seen
    }

    #[test]
    fn out_of_range_agent_ends_the_process() {
        for cleanup in [false, true] {
            let mut rig = ProcessRig::new();
            rig.enter_gsk_mode(10.0, 0.0);
            rig.e.mem.set_u8(rig.this + 0x32, u8::from(cleanup));
            rig.run();
            let frame = rig.frame();
            assert_eq!(
                calls_to(&rig.log, LOCAL_OBJECT_INIT),
                vec![vec![frame + 0x114, rig.result]]
            );
            assert_eq!(rig.e.mem.f32(rig.this + 0x2c), 10.0);
            assert_eq!(rig.e.mem.u8(rig.this + 0x85), 1);
            assert!(rig.machine_arguments().is_empty());
            assert_eq!(
                calls_to(&rig.log, MACHINE_CLEANUP),
                if cleanup {
                    vec![vec![rig.this + 0x30, OWNER, KEY]]
                } else {
                    Vec::new()
                }
            );
            assert!(calls_to(&rig.log, GSK_GET_CLOSEST_POINT).is_empty());
            // The start pair, then the end record.
            assert_eq!(rig.used(), 0x10 + 0xc);
            assert_eq!(rig.e.mem.u32(rig.records), LABEL_PROCESS);
            assert_eq!(rig.e.mem.u32(rig.records + 0xc), LABEL_PROCESS_DETAIL);
            assert_eq!(rig.e.mem.u32(rig.records + 0x10), LABEL_END);
        }
    }

    #[test]
    fn projection_on_the_normal_reduces_the_distance() {
        let mut rig = ProcessRig::new();
        rig.set_motion();
        rig.enter_gsk_mode(10.0, 0.0);
        rig.run();
        assert_eq!(rig.e.mem.f32(rig.this + 0x2c), 7.0);
        assert!(rig.machine_arguments().is_empty());
        assert_eq!(rig.e.mem.u8(rig.this + 0x85), 1);
    }

    #[test]
    fn predicted_contact_returns_to_multi_contact_mode_and_runs_the_machine() {
        let mut rig = ProcessRig::new();
        rig.set_motion();
        rig.enter_gsk_mode(1.0, 5.0);
        let machine = capture_machine(&mut rig);
        let frames = new_words();
        let sink = Rc::clone(&frames);
        let this = rig.this;
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, PREDICTIVE_STEP, move |e, _| {
            let frame = e.mem.u32(this + 8);
            let mut words = vec![frame];
            words.extend(snapshot(e, frame, 4));
            sink.borrow_mut().push(words);
            e.mem.set_u8(frame + 0x10, 1);
            Ret::default()
        });
        rig.run();
        let f = rig.frame();
        let (this, a, b, info, result) = (rig.this, rig.a, rig.b, rig.info, rig.result);
        // The predictive step ran on a stack frame standing in for `this + 8`.
        assert_eq!(
            *frames.borrow(),
            vec![vec![
                f + 0x1c,
                PREDICTIVE_FRAME_VTABLE,
                0x0001_0000,
                2,
                OWNER
            ]]
        );
        assert_eq!(
            calls_to(&rig.log, PREDICTIVE_STEP),
            vec![vec![this, a, b, info, result]]
        );
        assert_eq!(rig.e.mem.u32(this + 8), OWNER);
        assert_eq!(
            calls_to(&rig.log, LOCAL_OBJECT_ADD),
            vec![vec![f + 0x114, a, b, result, OWNER]]
        );
        // `fn_00d33ef0` cleaned the machine and cleared GSK mode; the countdown ran from 0x19.
        assert_eq!(
            calls_to(&rig.log, MACHINE_CLEANUP),
            vec![vec![this + 0x30, OWNER, KEY]]
        );
        assert_eq!(rig.e.mem.u8(this + 0x84), 0);
        assert_eq!(rig.e.mem.u16(this + 0x86), 0x18);
        assert_eq!(rig.e.mem.f32(this + 0x3c), -2.0);
        assert!(calls_to(&rig.log, GSK_GET_CLOSEST_POINT).is_empty());
        // The machine pass.
        assert_eq!(
            calls_to(&rig.log, SET_MUL_INVERSE_MUL),
            vec![vec![f + 0x64, rig.transform_a, rig.e.mem.u32(b + 8)]]
        );
        assert_eq!(
            rig.machine_arguments(),
            vec![vec![
                this + 0x30,
                f + 0x44,
                rig.collidable_b + 0x14,
                rig.arena,
                result
            ]]
        );
        let seen = machine.borrow();
        assert_eq!(seen.len(), 1);
        let seen = &seen[0];
        // The router was bumped by (2 * 4 + 0x83) & ~0x7F while the machine ran.
        assert_eq!(seen[0], rig.arena + 0x80);
        assert_eq!(seen[1..6], [a, b, 0, info, OWNER]);
        assert_eq!(
            seen[1 + 28..1 + 32],
            [2.0f32.to_bits(), 0, 0, 1.0f32.to_bits()]
        );
        assert_eq!(seen[33..], [0, 1, 0xffff_ffff]);
        assert_eq!(rig.e.mem.u32(rig.router), rig.arena);
        assert_eq!(rig.e.mem.u8(this + 0x85), 1);
        // Start pair, GSK step record, stream record, end record.
        assert_eq!(rig.used(), 0x10 + 0xc + 0xc + 0xc);
        assert_eq!(rig.e.mem.u32(rig.records + 0x10), LABEL_GSK_STEP);
        assert_eq!(rig.e.mem.u32(rig.records + 0x1c), LABEL_STREAM);
        assert_eq!(rig.e.mem.u32(rig.records + 0x28), LABEL_END);
    }

    #[test]
    fn unpredicted_contact_continues_only_inside_the_margin() {
        // (+0x32, +0x85, value of A's root, value of B's chain end, distance, continues)
        let cases: [(u8, u8, f32, f32, f32, bool); 8] = [
            (1, 0, 1.0, 0.25, -5.0, true),
            (0, 0, 1.0, 0.25, -5.0, false),
            (1, 1, 1.0, 0.25, -5.0, false),
            (1, 0, 1.0, 0.25, -0.25, false),
            (1, 0, 1.0, 0.25, -0.5, false),
            (1, 0, 1.0, 0.25, f32::NAN, false),
            (1, 0, 1.0, 0.25, -1.0, true),
            (1, 0, 0.25, 1.0, -1.0, true),
        ];
        for (flag_32, flag_85, value_a, value_b, distance, continues) in cases {
            let mut rig = ProcessRig::new();
            rig.enter_gsk_mode(distance, 0.0);
            let (this, a, b) = (rig.this, rig.a, rig.b);
            rig.e.mem.set_u8(this + 0x32, flag_32);
            rig.e.mem.set_u8(this + 0x85, flag_85);
            rig.e.mem.set_f32(a + 0x4c, value_a);
            let chain_end = rig.e.mem.alloc(0x60);
            rig.e.mem.set_u32(b + 0xc, chain_end);
            rig.e.mem.set_f32(chain_end + 0x4c, value_b);
            rig.run();
            let added = calls_to(&rig.log, LOCAL_OBJECT_ADD).len();
            assert_eq!(
                added,
                usize::from(continues),
                "{flag_32} {flag_85} {distance}"
            );
            assert_eq!(rig.machine_arguments().len(), usize::from(continues));
            assert_eq!(rig.e.mem.u8(this + 0x85), 1);
        }
    }

    #[test]
    fn closest_point_query_runs_when_the_countdown_runs_out() {
        let mut rig = ProcessRig::new();
        rig.e.mem.set_u16(rig.this + 0x86, 0xffff);
        rig.e.mem.set_f32(rig.this + 0x2c, 3.0);
        rig.e.mem.set_u32(rig.info + 8, SIGNALLING);
        let queries = new_words();
        let sink = Rc::clone(&queries);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, GSK_GET_CLOSEST_POINT, move |e, a| {
            sink.borrow_mut().push(snapshot(e, a[0], 5));
            Ret::default()
        });
        rig.run();
        let f = rig.frame();
        let (this, a, b) = (rig.this, rig.a, rig.b);
        let (transform_a, transform_b) = (rig.transform_a, rig.e.mem.u32(b + 8));
        let (collidable_a, collidable_b) = (rig.e.mem.u32(a), rig.collidable_b);
        // No contact: the countdown restarted, the distance is the negated range.
        assert_eq!(rig.e.mem.u16(this + 0x86), 0x19);
        assert_eq!(rig.e.mem.f32(this + 0x3c), -3.0);
        assert_eq!(
            calls_to(&rig.log, GSK_GET_CLOSEST_POINT),
            vec![vec![f + 0x1c, this + 0xc, this + 0x20, f + 0xc4]]
        );
        assert_eq!(
            *queries.borrow(),
            vec![vec![
                f + 0xd4,
                transform_a,
                collidable_a,
                collidable_b,
                QUIETED
            ]]
        );
        assert_eq!(
            calls_to(&rig.log, SET_MUL_INVERSE_MUL),
            vec![
                vec![f + 0xd4, transform_a, transform_b],
                vec![f + 0x64, transform_a, transform_b]
            ]
        );
        assert!(calls_to(&rig.log, LOCAL_OBJECT_ADD).is_empty());
        assert_eq!(rig.machine_arguments().len(), 1);
    }

    #[test]
    fn a_found_contact_switches_to_gsk_mode_and_is_recorded() {
        let mut rig = ProcessRig::new();
        rig.e.mem.set_u16(rig.this + 0x86, 0xffff);
        rig.e.mem.set_f32(rig.this + 0x2c, -1.0);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, GSK_GET_CLOSEST_POINT, |_, _| returns(1));
        let this = rig.this;
        double(&mut rig.e, &log, PREDICTIVE_STEP, move |e, _| {
            let frame = e.mem.u32(this + 8);
            e.mem.set_u8(frame + 0x10, 1);
            Ret::default()
        });
        rig.run();
        let f = rig.frame();
        let (a, b, result) = (rig.a, rig.b, rig.result);
        assert_eq!(
            calls_to(&rig.log, MACHINE_CLEAR_AND_DESTROY),
            vec![vec![this + 0x30, 0x8080, OWNER, KEY]]
        );
        // Once after the contact was found, once after the predictive step.
        assert_eq!(
            calls_to(&rig.log, LOCAL_OBJECT_ADD),
            vec![vec![f + 0x114, a, b, result, OWNER]; 2]
        );
        assert_eq!(calls_to(&rig.log, GSK_GET_CLOSEST_POINT).len(), 1);
        assert_eq!(rig.e.mem.u8(this + 0x84), 0);
        assert_eq!(rig.machine_arguments().len(), 1);
    }

    #[test]
    fn running_countdown_goes_straight_to_the_machine_pass() {
        let mut rig = ProcessRig::new();
        rig.e.mem.set_u16(rig.this + 0x86, 5);
        rig.e.mem.set_u32(rig.collidable_b + 0x48, 0);
        let machine = capture_machine(&mut rig);
        rig.run();
        assert_eq!(rig.e.mem.u16(rig.this + 0x86), 4);
        assert!(calls_to(&rig.log, GSK_GET_CLOSEST_POINT).is_empty());
        assert_eq!(rig.machine_arguments().len(), 1);
        let seen = machine.borrow();
        // An empty list still bumps the router by 0x80 and holds the terminator.
        assert_eq!(seen[0][0], rig.arena + 0x80);
        assert_eq!(seen[0][33], 0xffff_ffff);
        assert_eq!(rig.e.mem.u32(rig.router), rig.arena);
    }

    #[test]
    fn symmetric_process_swaps_the_bodies_and_mirrors_the_new_contacts() {
        // (float before, float after, whether the swap block runs)
        let cases = [
            (1.0f32.to_bits(), 1.0f32.to_bits(), false),
            (1.0f32.to_bits(), 2.0f32.to_bits(), true),
            (f32::NAN.to_bits(), f32::NAN.to_bits(), true),
        ];
        for (before, after, swapped) in cases {
            let mut rig = ProcessRig::new();
            rig.set_motion();
            rig.enter_gsk_mode(10.0, 0.0);
            let result = rig.result;
            for lane in 0..4 {
                let mask = if lane < 3 { 0x8000_0000u32 } else { 0 };
                rig.e.set_global(NEGATE_XYZ + 4 * lane, mask);
            }
            rig.e.mem.set_u32(result, result + 0x100);
            rig.e.mem.set_u32(result + 0x3030, before);
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                rig.e.mem.set_f32(result + 0x3020 + 4 * lane as u32, value);
            }
            rig.e.mem.set_u8(result + 0x3048, 2);
            rig.e.mem.set_u8(result + 0x3049, 2);
            rig.e.mem.set_u8(result + 0x304a, 0x12);
            for (index, word) in [0xa0u16, 0xa1, 0xa2, 0xa3].into_iter().enumerate() {
                rig.e.mem.set_u16(result + 0x3040 + 2 * index as u32, word);
            }
            // The process appends two contact records and moves the float.
            let log = Rc::clone(&rig.log);
            double(&mut rig.e, &log, LOCAL_OBJECT_INIT, move |e, a| {
                let stream = e.mem.u32(a[1]);
                let records = [
                    [1.0f32, 2.0, 3.0, 4.0, 0.5, 0.25, 2.0, 3.0],
                    [1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0, 2.0],
                ];
                for (n, record) in records.iter().enumerate() {
                    for (lane, value) in record.iter().enumerate() {
                        e.mem
                            .set_f32(stream + 0x30 * n as u32 + 4 * lane as u32, *value);
                    }
                }
                e.mem.set_u32(a[1], stream + 0x60);
                e.mem.set_u32(a[1] + 0x3030, after);
                Ret::default()
            });
            let (this, a, b, info) = (rig.this, rig.a, rig.b, rig.info);
            // Called with the bodies the other way round; the agent sees them swapped back.
            rig.e.call(0x00d3_4c10, &args![this, b, a, info, result]);
            assert_eq!(rig.e.mem.f32(this + 0x2c), 7.0);
            let read = |e: &Engine, address: u32, count: u32| -> Vec<f32> {
                (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
            };
            assert_eq!(
                read(&rig.e, result + 0x100, 8),
                vec![2.5, 2.75, 9.0, 13.0, -0.5, -0.25, -2.0, 3.0]
            );
            assert_eq!(
                read(&rig.e, result + 0x130, 8),
                vec![3.0, 5.0, 7.0, 5.0, -1.0, -2.0, -3.0, 2.0]
            );
            assert_eq!(rig.e.mem.u32(result), result + 0x160);
            if swapped {
                assert_eq!(
                    read(&rig.e, result + 0x3020, 4),
                    vec![-1.0, -2.0, -3.0, 4.0]
                );
                assert_eq!(rig.e.mem.u8(result + 0x304a), 0x21);
                assert_eq!(rig.e.mem.u8(result + 0x3048), 2);
                assert_eq!(rig.e.mem.u8(result + 0x3049), 2);
                let words: Vec<u16> = (0..4)
                    .map(|i| rig.e.mem.u16(result + 0x3040 + 2 * i))
                    .collect();
                // The head swaps with the word the byte sum points at, then (the bytes share
                // exactly bit 1) the two middle words swap.
                assert_eq!(words, vec![0xa3, 0xa2, 0xa1, 0xa0]);
            } else {
                assert_eq!(read(&rig.e, result + 0x3020, 4), vec![1.0, 2.0, 3.0, 4.0]);
                assert_eq!(rig.e.mem.u8(result + 0x304a), 0x12);
                let words: Vec<u16> = (0..4)
                    .map(|i| rig.e.mem.u16(result + 0x3040 + 2 * i))
                    .collect();
                assert_eq!(words, vec![0xa0, 0xa1, 0xa2, 0xa3]);
            }
        }
    }
}
