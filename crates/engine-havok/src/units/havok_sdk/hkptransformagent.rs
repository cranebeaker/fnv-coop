//! `Havok SDK/hkptransformagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::{
    create_function, set_cd_body, static_function, symmetric_linear_cast,
    symmetric_process_collision, with_flip_body_collector, with_flip_collector,
};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// `hkTransform::setMul` (Xbox PDB): `this` is the 64-byte result, then the two operands.
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;

// Translated from 00d431b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMotionState::hkMotionState` (Xbox PDB), the copy constructor: copies the 0xAF bytes of
/// `source` into `this`. The first 0x40 bytes are read as a block before any is written (a
/// 16-byte vector copy through the stack); +0x40..+0x90 are copied word by word; the three
/// `float`s at +0xA0, +0xA4 and +0xA8 go through the x87 stack (signalling NaNs come back
/// quiet) and the bytes at +0xAC, +0xAD and +0xAE are copied last. Returns `this`.
pub fn hkp_motion_state_copy_constructor(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let from = source.addr();
    let to = this.addr();
    let head = e.mem.bytes(from, 0x40);
    e.mem.write(to, &head);
    let body = e.mem.bytes(from + 0x40, 0x50);
    e.mem.write(to + 0x40, &body);
    for offset in [0x90, 0x94, 0x98, 0x9c] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xa0, 0xa4, 0xa8] {
        let value = x87_quiet(e.mem.f32(from + offset));
        e.mem.set_f32(to + offset, value);
    }
    for offset in [0xac, 0xad, 0xae] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    this
}

// Translated from 00d42cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::linearCast` (Xbox PDB). `entry` is the collision entry (`bodyA`,
/// the body at +0, an `hkTransform` pointer at +8): the transform of the entry is multiplied
/// by the transform at `body + 0x30` into a 64-byte local; then the agent held at `this + 0xc`
/// gets its virtual method at slot 0x14 called with a 16-byte record on the stack
/// (`word at body + 0x14`, 0, the local transform, `entry`) followed by the four remaining
/// words.
pub fn hkp_transform_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    let body = e.mem.u32(entry.addr());
    e.with_stack(0x50, |e, frame| {
        let record = frame.addr();
        let transform = frame.addr() + 0x10;
        let entry_transform = e.mem.u32(entry.addr() + 8);
        e.call(
            TRANSFORM_SET_MUL,
            &args![transform, entry_transform, body + 0x30],
        );
        let first = e.mem.u32(body + 0x14);
        e.mem.set_u32(record, first);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, transform);
        e.mem.set_u32(record + 12, entry.addr());
        let agent = e.mem.u32(this.addr() + 0xc);
        e.vcall(agent, 0x14, &args![record, a, b, c, d]);
    });
}

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkQuaternion::setMul` (Xbox PDB, `00c66320`): `this` is the 16-byte result, then the two
/// operands.
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// `hkpCollisionDispatcher` agent registration (`00cfbbe0`, `this` = dispatcher, `RET 0xC`):
/// `(agent function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The vtable of the agent as the creation functions first write it, and the one the
/// symmetric creation function leaves in the agent once the child agent is made.
const AGENT_VTABLE: u32 = 0x010c_f280;
const SYMMETRIC_AGENT_VTABLE: u32 = 0x010c_f2c8;
/// The string `"ChildAgent"` that `calcContentStatistics` hands to the statistics collector.
const CHILD_AGENT_NAME: u32 = 0x010c_f270;
/// Profiling labels of `processCollision`: `"TtTransform"` and the shared end label.
const LABEL_TRANSFORM: u32 = 0x010c_f2b8;
const LABEL_END: u32 = 0x010c_41f8;

/// Builds on the stack the `hkpCdBody` of the child shape of the transform shape of
/// `body_a` (`m_shape` = the transform shape's child at `+0x14`, key 0, `m_motion` = the
/// product of the body's transform and the shape's own at `+0x30`, `m_parent` = `body_a`) and
/// runs `f` with it (the 16-byte body is followed by the 64-byte product).
fn with_child_body<R>(e: &mut Engine, body_a: Ptr, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0x50, |e, frame| {
        let body = frame.addr();
        let product = body + 0x10;
        e.call(TRANSFORM_SET_MUL, &args![product, motion, shape + 0x30]);
        let child = e.mem.u32(shape + 0x14);
        set_cd_body(e, body, child, 0, product, body_a.addr());
        f(e, body)
    })
}

// Translated from 00d429c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::cleanup` (Xbox PDB): cleans up the child agent held at `+0xC` (its slot
/// `0x1C`, passing the word on) and then deletes the agent itself through its vtable's first
/// slot with the flag 1.
pub fn hkp_transform_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(child, 0x1c, &args![param]);
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d429e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::calcContentStatistics` (Xbox PDB): asks the statistics `collector`
/// (slot `0xC`) to account the child agent held at `+0xC` under the name `"ChildAgent"`
/// (`010cf270`) with a last argument of 0. The function ends `RET 8`: the second stack word is
/// not read.
pub fn hkp_transform_agent_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_1: u32,
) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(collector.addr(), 0xc, &args![CHILD_AGENT_NAME, child, 0u32]);
}

// Translated from 00d42a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::invalidateTim` (Xbox PDB): jumps to slot `0x24` of the child agent held
/// at `+0xC`, passing its one stack word on (the slot ends in `RET 4`: the base `00d9fca0`).
pub fn hkp_transform_agent_invalidate_tim(e: &mut Engine, this: Ptr, word: u32) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(child, 0x24, &args![word]);
}

// Translated from 00d42a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::warpTime` (Xbox PDB): passes the two times (floats, through
/// `FLD`/`FSTP`, so signalling NaNs come back quiet) and the third word to slot `0x28` of the
/// child agent held at `+0xC`.
pub fn hkp_transform_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    old_time: f32,
    new_time: f32,
    third: u32,
) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(
        child,
        0x28,
        &args![x87_quiet(old_time), x87_quiet(new_time), third],
    );
}

// Translated from 00d42a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::removePoint` (Xbox PDB): jumps to slot `0x2C` of the child agent held
/// at `+0xC`, passing its one stack word on (the slot ends in `RET 4`: the base `00d9fcc0`).
pub fn hkp_transform_agent_remove_point(e: &mut Engine, this: Ptr, word: u32) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(child, 0x2c, &args![word]);
}

// Translated from 00d42a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::commitPotential` (Xbox PDB): jumps to slot `0x30` of the child agent
/// held at `+0xC`, passing its one stack word on (the slot ends in `RET 4`: the base `00d9fcd0`).
pub fn hkp_transform_agent_commit_potential(e: &mut Engine, this: Ptr, word: u32) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(child, 0x30, &args![word]);
}

// Translated from 00d42a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::createZombie` (Xbox PDB): jumps to slot `0x34` of the child agent held
/// at `+0xC`, passing its one stack word on (the slot ends in `RET 4`: the base `00d9fce0`).
pub fn hkp_transform_agent_create_zombie(e: &mut Engine, this: Ptr, word: u32) {
    let child = e.mem.u32(this.addr() + 0xc);
    e.vcall(child, 0x34, &args![word]);
}

/// The body of the two creation functions (`00d42a70`, `00d43030`): allocates the 16-byte
/// agent (through the memory router's allocator, slot 4), sets its size word (`+4` = 0x10),
/// reference count (`+6` = 1), contact manager (`+8`) and vtable `010cf280`; builds on the
/// stack the `hkpCdBody` of the child shape of the transform shape of `shape_body` (its
/// motion is a copy of the motion state whose transform is replaced by the product of the
/// motion's transform and the shape's own at `+0x30`); asks the dispatcher (`create_function`:
/// the child's type against the type of `other_body`'s shape) for the child agent and keeps it
/// at `+0xC`. `final_vtable` is written last when the function sets one.
fn create_transform_agent(
    e: &mut Engine,
    shape_body: Ptr,
    other_body: Ptr,
    input: Ptr,
    manager: u32,
    final_vtable: Option<u32>,
) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let agent = e.vcall(allocator, 4, &args![0x10u32]).u32();
    e.mem.set_u16(agent + 4, 0x10);
    e.mem.set_u16(agent + 6, 1);
    e.mem.set_u32(agent + 8, manager);
    e.mem.set_u32(agent, AGENT_VTABLE);
    let shape = e.mem.u32(shape_body.addr());
    let child = e.mem.u32(shape + 0x14);
    e.with_stack(0xc0, |e, frame| {
        let body = frame.addr();
        let state = body + 0x10;
        let motion = e.mem.u32(shape_body.addr() + 8);
        hkp_motion_state_copy_constructor(e, Ptr::new(state), Ptr::new(motion));
        let motion = e.mem.u32(shape_body.addr() + 8);
        e.call(TRANSFORM_SET_MUL, &args![state, motion, shape + 0x30]);
        set_cd_body(e, body, child, 0, state, shape_body.addr());
        let type_a = e.mem.u32(child + 0xc);
        let other_shape = e.mem.u32(other_body.addr());
        let type_b = e.mem.u32(other_shape + 0xc);
        let function = create_function(e, input.addr(), type_a, type_b);
        let created = e
            .call(function, &args![body, other_body, input, manager])
            .u32();
        e.mem.set_u32(agent + 0xc, created);
    });
    if let Some(vtable) = final_vtable {
        e.mem.set_u32(agent, vtable);
    }
    Ptr::new(agent)
}

// Translated from 00d42a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word of the agent entry `00d43120` registers for the pair (transform shape, any
/// shape) (the map gives it no name; that it is the entry's `createFunc` is unconfirmed):
/// creates the transform agent for `body_a` (the body with the transform shape) against
/// `body_b`, see `create_transform_agent`. Returns the new agent.
pub fn fn_00d42a70(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, manager: u32) -> Ptr {
    create_transform_agent(e, body_a, body_b, input, manager, None)
}

// Translated from 00d42b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::processCollision` (Xbox PDB): between two profiling records
/// (`"TtTransform"` and the end label) it builds on the stack the body of the child shape of
/// the transform shape of `body_a`. Its motion is a copy of `body_a`'s motion state in which
/// the transform (`+0x00`) is the product of the motion's transform and the shape's own
/// (`+0x30`, `hkTransform::setMul`), the 32 bytes at `+0x40` are copied, the two quaternions
/// at `+0x60` and `+0x70` are the products with the shape's quaternion (`+0x20`,
/// `hkQuaternion::setMul`), the vector at `+0x80` is `(position - shape translation)` against
/// the shape's rotation columns at `+0x30`, `+0x40` and `+0x50` (SSE: lane `l` is
/// `(vy*Y[l] + vx*X[l]) + vz*Z[l]` with `X = (c0.x, c1.x, c2.x, c2.y)`, `Y = (c0.y, c1.y,
/// c2.y, c2.w)`, `Z = (c0.z, c1.z, c2.z, c2.w)`, as the shuffles leave them), the 16 bytes at
/// `+0x90` are copied and the float at `+0xA0` goes through the x87 stack. The child agent
/// held at `+0xC` then gets its slot `0x18` with the body, `body_b`, `input` and `output`.
pub fn hkp_transform_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, LABEL_TRANSFORM);
    let shape = e.mem.u32(body_a.addr());
    let motion = e.mem.u32(body_a.addr() + 8);
    e.with_stack(0xe0, |e, frame| {
        let body = frame.addr() + 0x10;
        let state = frame.addr() + 0x24;
        e.call(TRANSFORM_SET_MUL, &args![state, motion, shape + 0x30]);
        let motion = e.mem.u32(body_a.addr() + 8);
        let low = e.mem.bytes(motion + 0x40, 0x10);
        e.mem.write(state + 0x40, &low);
        let high = e.mem.bytes(motion + 0x50, 0x10);
        e.mem.write(state + 0x50, &high);
        e.call(
            QUATERNION_SET_MUL,
            &args![state + 0x60, motion + 0x60, shape + 0x20],
        );
        e.call(
            QUATERNION_SET_MUL,
            &args![state + 0x70, motion + 0x70, shape + 0x20],
        );
        let lanes = [0u32, 4, 8, 12];
        let load = |e: &Engine, address: u32| lanes.map(|lane| e.mem.f32(address + lane));
        let position = load(&*e, motion + 0x80);
        let translation = load(&*e, shape + 0x60);
        let (c0, c1, c2) = (
            load(&*e, shape + 0x30),
            load(&*e, shape + 0x40),
            load(&*e, shape + 0x50),
        );
        let difference = [0, 1, 2, 3].map(|lane| position[lane] - translation[lane]);
        let x_terms = [c0[0], c1[0], c2[0], c2[1]];
        let y_terms = [c0[1], c1[1], c2[1], c2[3]];
        let z_terms = [c0[2], c1[2], c2[2], c2[3]];
        for lane in 0..4 {
            let sum = difference[1] * y_terms[lane] + difference[0] * x_terms[lane];
            let value = sum + difference[2] * z_terms[lane];
            e.mem.set_f32(state + 0x80 + 4 * lane as u32, value);
        }
        let last = e.mem.bytes(motion + 0x90, 0x10);
        e.mem.write(state + 0x90, &last);
        let end = x87_quiet(e.mem.f32(motion + 0xa0));
        e.mem.set_f32(state + 0xa0, end);
        let child_shape = e.mem.u32(shape + 0x14);
        set_cd_body(e, body, child_shape, 0, state, body_a.addr());
        let child = e.mem.u32(this.addr() + 0xc);
        e.vcall(child, 0x18, &args![body, body_b, input, output]);
    });
    profile_record(e, LABEL_END);
}

/// The static entry points: builds the body of the child shape of `body_a`'s transform shape
/// (`with_child_body`) and calls the function that the dispatcher's static table gives (the
/// word at `slot` in the entry the child's type and `body_b`'s shape type pick) with the body,
/// `body_b`, `input` and the `rest` words.
fn static_call(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, slot: u32, rest: &[u32]) {
    with_child_body(e, body_a, |e, body| {
        let child = e.mem.u32(body);
        let type_a = e.mem.u32(child + 0xc);
        let other_shape = e.mem.u32(body_b.addr());
        let type_b = e.mem.u32(other_shape + 0xc);
        let function = static_function(e, input.addr(), type_a, type_b, slot);
        let mut words = vec![body, body_b.addr(), input.addr()];
        words.extend_from_slice(rest);
        e.call(function, &words);
    });
}

// Translated from 00d42d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fourth word of the agent entry `00d43120` registers (the map gives it no name; that it
/// is the entry's `linearCastFunc` is unconfirmed): the static linear cast of the transform
/// agent, see `static_call` (table word `0x9AC`, the two collectors passed on).
pub fn fn_00d42d60(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    static_call(
        e,
        body_a,
        body_b,
        input,
        0x9ac,
        &[cast_collector, start_collector],
    );
}

// Translated from 00d42de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::getClosestPoints` (Xbox PDB): builds the body of the child shape of
/// `body_a`'s transform shape (`with_child_body`) and passes it, `body_b`, `input` and the
/// collector to slot `0x10` of the child agent held at `+0xC`.
pub fn hkp_transform_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: u32,
) {
    with_child_body(e, body_a, |e, body| {
        let child = e.mem.u32(this.addr() + 0xc);
        e.vcall(child, 0x10, &args![body, body_b, input, collector]);
    });
}

// Translated from 00d42e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The third word of the agent entry `00d43120` registers (the map gives it no name; that it
/// is the entry's `getClosestPointFunc` is unconfirmed): the static `getClosestPoints` of the
/// transform agent, see `static_call` (table word `0x9A8`).
pub fn fn_00d42e50(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: u32) {
    static_call(e, body_a, body_b, input, 0x9a8, &[collector]);
}

// Translated from 00d42ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::getPenetrations` (Xbox PDB): as `getClosestPoints` with slot `0xC` of
/// the child agent.
pub fn hkp_transform_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: u32,
) {
    with_child_body(e, body_a, |e, body| {
        let child = e.mem.u32(this.addr() + 0xc);
        e.vcall(child, 0xc, &args![body, body_b, input, collector]);
    });
}

// Translated from 00d42f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second word of the agent entry `00d43120` registers (the map gives it no name; that it
/// is the entry's `getPenetrationsFunc` is unconfirmed): the static `getPenetrations` of the
/// transform agent, see `static_call` (table word `0x9A4`).
pub fn fn_00d42f40(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: u32) {
    static_call(e, body_a, body_b, input, 0x9a4, &[collector]);
}

// Translated from 00d42fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::updateShapeCollectionFilter` (Xbox PDB): as `getClosestPoints` with
/// slot `0x20` of the child agent and the two remaining words passed on as they are.
pub fn hkp_transform_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    with_child_body(e, body_a, |e, body| {
        let child = e.mem.u32(this.addr() + 0xc);
        e.vcall(child, 0x20, &args![body, body_b, third, fourth]);
    });
}

// Translated from 00d43030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word of the symmetric agent entry `00d43120` registers (the map gives it no name;
/// that it is the entry's `createFunc` is unconfirmed): as `fn_00d42a70` with the bodies'
/// roles swapped (`body_b` has the transform shape) and the agent's vtable set to `010cf2c8`
/// once the child agent is made. Returns the new agent.
pub fn fn_00d43030(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, manager: u32) -> Ptr {
    create_transform_agent(
        e,
        body_b,
        body_a,
        input,
        manager,
        Some(SYMMETRIC_AGENT_VTABLE),
    )
}

// Translated from 00d43120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the transform agent with a dispatcher (the map gives it no name; it has the shape
/// of `hkpTransformAgent::registerAgent`, which is unconfirmed): two agent function entries
/// of 5 words, built on the stack and handed to `00cfbbe0` (`this` = the dispatcher, `(entry,
/// type a, type b)`). The symmetric entry (`00d43030`, `00d432f0`, `00d43330`, `00d43370`,
/// both flag bytes 1) is registered for the pair (all types = -1, 0xF); the direct entry
/// (`00d42a70`, `00d42f40`, `00d42e50`, `00d42d60`, flag bytes 0 and 1) for (0xF, all types =
/// -1). The two bytes after the flags are not set by the exe (the callee copies the word they
/// are in); here they are zero.
pub fn fn_00d43120(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (words, flag_a, flag_b, type_a, type_b) in [
            (
                [0x00d4_3030, 0x00d4_32f0, 0x00d4_3330, 0x00d4_3370],
                1u8,
                1u8,
                0xffff_ffffu32,
                0xfu32,
            ),
            (
                [0x00d4_2a70, 0x00d4_2f40, 0x00d4_2e50, 0x00d4_2d60],
                0,
                1,
                0xf,
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

// Translated from 00d432f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the transform agent (the map gives it no name):
/// runs `00d42f40` with the two bodies swapped and, in place of the caller's collector word, a
/// flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the caller's word at `+8`) built on
/// the stack.
pub fn fn_00d432f0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        fn_00d42f40(e, body_b, body_a, input, flip);
    });
}

// Translated from 00d43330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the transform agent (the map gives it no name):
/// runs `00d42e50` with the two bodies swapped and a flip collector (vtable `010ce7fc`, the
/// large float through `FLD`/`FSTP` at `+4`, the caller's word at `+8`).
pub fn fn_00d43330(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        fn_00d42e50(e, body_b, body_a, input, flip);
    });
}

// Translated from 00d43370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static linear cast of the transform agent (the map gives it no name): runs
/// `00d42d60` with the two bodies swapped, a copy of the input whose path vector
/// (`input + 0x50`) is negated, and flip-cast collectors (vtable `010ce80c`) wrapping the
/// caller's two collectors (`start_collector` null stays null).
pub fn fn_00d43370(
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
            fn_00d42d60(e, body_b, body_a, Ptr::new(copy), first, second);
        },
    );
}

// Translated from 00d43460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpTransformAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpTransformAgent::getPenetrations` with the bodies swapped and a flip-body collector
/// (vtable `010ce81c`, byte 0 at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_transform_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_transform_agent_get_penetrations(e, this, body_b, body_a, input, flip);
    });
}

// Translated from 00d434a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpTransformAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpTransformAgent::getClosestPoints` with the bodies swapped and a flip collector (vtable
/// `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_transform_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_transform_agent_get_closest_points(e, this, body_b, body_a, input, flip);
    });
}

// Translated from 00d434e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpTransformAgent>::processCollision` (Xbox PDB): runs
/// `hkpTransformAgent::processCollision` with the bodies swapped, then turns what it appended
/// to the output around (contact positions moved along their normals by the normal's fourth
/// lane, normals negated, and the time-of-impact data when the time changed), as
/// `hkp_symmetric_agent_linear_cast_bv_agent_process_collision` does.
pub fn hkp_symmetric_agent_linear_cast_transform_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        hkp_transform_agent_process_collision(e, this, body_b, body_a, input, output);
    });
}

// Translated from 00d43600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpTransformAgent>::updateShapeCollectionFilter` (Xbox PDB):
/// calls `hkpTransformAgent::updateShapeCollectionFilter` with the two bodies swapped.
pub fn hkp_symmetric_agent_linear_cast_transform_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
    fourth: u32,
) {
    hkp_transform_agent_update_shape_collection_filter(e, this, body_b, body_a, third, fourth);
}

// Translated from 00d43620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpTransformAgent>::linearCast` (Xbox PDB): runs
/// `hkpTransformAgent::linearCast` with the two bodies swapped, a copy of the input whose
/// path vector is negated and flip-cast collectors wrapping the caller's two (as `00d43370`
/// does).
pub fn hkp_symmetric_agent_transform_agent_linear_cast(
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
            hkp_transform_agent_linear_cast(e, this, body_b, body_a.addr(), copy, first, second);
        },
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d431b0,
            hkp_motion_state_copy_constructor(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d42cf0,
            hkp_transform_agent_linear_cast(Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d429c0, hkp_transform_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d429e0,
            hkp_transform_agent_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d42a00, hkp_transform_agent_invalidate_tim(Ptr, u32)),
        entry!(
            0x00d42a10,
            hkp_transform_agent_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(0x00d42a40, hkp_transform_agent_remove_point(Ptr, u32)),
        entry!(0x00d42a50, hkp_transform_agent_commit_potential(Ptr, u32)),
        entry!(0x00d42a60, hkp_transform_agent_create_zombie(Ptr, u32)),
        entry!(0x00d42a70, fn_00d42a70(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00d42b50,
            hkp_transform_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d42d60, fn_00d42d60(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d42de0,
            hkp_transform_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d42e50, fn_00d42e50(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d42ed0,
            hkp_transform_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d42f40, fn_00d42f40(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d42fc0,
            hkp_transform_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d43030, fn_00d43030(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d43120, fn_00d43120(Ptr)),
        entry!(0x00d432f0, fn_00d432f0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d43330, fn_00d43330(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d43370, fn_00d43370(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d43460,
            hkp_symmetric_agent_linear_cast_transform_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d434a0,
            hkp_symmetric_agent_linear_cast_transform_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d434e0,
            hkp_symmetric_agent_linear_cast_transform_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(
            0x00d43600,
            hkp_symmetric_agent_linear_cast_transform_agent_update_shape_collection_filter(
                Ptr, Ptr, Ptr, u32, u32
            )
        ),
        entry!(
            0x00d43620,
            hkp_symmetric_agent_transform_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const LINEAR_CAST_SLOT: u32 = 0x0900_0014;

    #[test]
    fn motion_state_copy_copies_every_field_and_returns_this() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xb0);
        for offset in 0..0xaf {
            e.mem.set_u8(
                source + offset,
                (offset as u8).wrapping_mul(3).wrapping_add(1),
            );
        }
        // A signalling NaN at +0xa4 comes back quiet; the other floats are unchanged.
        e.mem.set_u32(source + 0xa4, 0x7f80_0001);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u8(this + 0xaf, 0x5a);
        let result = e.call(0x00d431b0, &args![this, source]).u32();
        assert_eq!(result, this);
        for offset in (0..0xaf).filter(|o| !(0xa4..0xa8).contains(o)) {
            assert_eq!(
                e.mem.u8(this + offset),
                e.mem.u8(source + offset),
                "{offset:#x}"
            );
        }
        assert_eq!(e.mem.u32(this + 0xa4), 0x7fc0_0001);
        // The byte after the last copied one is not touched.
        assert_eq!(e.mem.u8(this + 0xaf), 0x5a);
    }

    #[test]
    fn linear_cast_hands_the_agent_the_combined_transform() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(TRANSFORM_SET_MUL, move |e, a| {
            // The product is recorded as the sum of the two operand markers.
            let sum = e.mem.u32(a[1]) + e.mem.u32(a[2]);
            e.mem.set_u32(a[0], sum);
            sink.borrow_mut().push(vec![a[0], a[1], a[2]]);
            Ret::default()
        });
        let record_log = Rc::new(RefCell::new(vec![]));
        let sink = record_log.clone();
        e.register_double(LINEAR_CAST_SLOT, move |e, a| {
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            let combined = e.mem.u32(record[2]);
            sink.borrow_mut().push((a.to_vec(), record, combined));
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x14, LINEAR_CAST_SLOT);
        let agent = e.mem.alloc(8);
        e.mem.set_u32(agent, table);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, agent);
        let body = e.mem.alloc(0x80);
        e.mem.set_u32(body + 0x14, 0x777);
        e.mem.set_u32(body + 0x30, 30);
        let entry_transform = e.mem.alloc(0x40);
        e.mem.set_u32(entry_transform, 12);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, body);
        e.mem.set_u32(entry + 8, entry_transform);
        e.call(0x00d42cf0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][1..], [entry_transform, body + 0x30]);
        let logs = record_log.borrow();
        assert_eq!(logs.len(), 1);
        let (words, record, combined) = &logs[0];
        assert_eq!(words[0], agent);
        assert_eq!(words[2..], [1, 2, 3, 4]);
        assert_eq!(record[0], 0x777);
        assert_eq!(record[1], 0);
        assert_eq!(record[2], seen[0][0]);
        assert_eq!(record[3], entry);
        assert_eq!(*combined, 42);
    }

    use crate::units::havok_sdk::hkpbvagent::test_support::{
        body, calls, engine, input, log_double, logging_object, monitor, new_log, profile_labels,
        set_entry, set_matrix, shape, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_CAST_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };

    /// Vtable doubles of the child agent, of `this` and of a collector (slots at `base + slot`).
    const CHILD: u32 = 0x0900_0000;
    const THIS: u32 = 0x0910_0000;
    const COLLECTOR: u32 = 0x0920_0000;
    /// Functions the dispatcher hands out: `base + slot` for the static ones.
    const CREATE_FUNCTION: u32 = 0x0930_0000;
    const STATIC_FUNCTION: u32 = 0x0940_0000;
    /// The allocator's slot 4 double.
    const ALLOCATE: u32 = 0x0950_0004;
    const CHILD_TYPE: u32 = 3;
    const OTHER_TYPE: u32 = 7;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `address` that logs its arguments, copies the `n` words at its
    /// argument `index` when it is called and answers `eax`.
    fn capture(e: &mut Engine, log: &Log, address: u32, index: usize, n: u32, eax: u32) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let words = (0..n).map(|word| e.mem.u32(a[index] + 4 * word)).collect();
            copies.borrow_mut().push(words);
            Ret {
                eax,
                ..Ret::default()
            }
        });
        seen
    }

    type Bytes = Rc<RefCell<Vec<Vec<u8>>>>;

    /// Registers a double at `address` that logs its arguments, copies the 0xC0 bytes at its
    /// argument `index` (a child body and the motion state after it, which are gone once the
    /// caller returns) and answers `eax`.
    fn snapshot(e: &mut Engine, log: &Log, address: u32, index: usize, eax: u32) -> Bytes {
        let seen: Bytes = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            copies.borrow_mut().push(e.mem.bytes(a[index], 0xc0));
            Ret {
                eax,
                ..Ret::default()
            }
        });
        seen
    }

    /// The word at byte `offset` of a snapshot.
    fn word_at(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ])
    }

    /// The `n` words at `at`.
    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        child: u32,
        transform_shape: u32,
        child_shape: u32,
        motion: u32,
        body_a: u32,
        body_b: u32,
        input: u32,
        dispatcher: u32,
    }

    /// An agent (`this`, its child agent at `+0xC`), a body A whose shape is a transform shape
    /// (child shape of type 3 at `+0x14`) with the motion at `motion`, a body B with a shape
    /// of type 7, and an input whose dispatcher is empty; `hkTransform::setMul` and
    /// `hkQuaternion::setMul` are logging doubles.
    fn rig() -> Rig {
        let mut e = engine();
        let log = new_log();
        e.mem.set_u32(LARGE_FLOAT, 0x7f7f_ffee);
        let child = logging_object(&mut e, &log, CHILD);
        let this = logging_object(&mut e, &log, THIS);
        e.mem.set_u32(this + 0xc, child);
        let child_shape = shape(&mut e, CHILD_TYPE, 0, 0);
        let transform_shape = e.mem.alloc(0x80);
        e.mem.set_u32(transform_shape + 0x14, child_shape);
        let motion = e.mem.alloc(0xb0);
        let other_shape = shape(&mut e, OTHER_TYPE, 0, 0);
        let body_a = body(&mut e, transform_shape, motion);
        let body_b = body(&mut e, other_shape, 0x5678);
        let (input, dispatcher) = input(&mut e);
        log_double(&mut e, &log, TRANSFORM_SET_MUL, 0);
        log_double(&mut e, &log, QUATERNION_SET_MUL, 0);
        Rig {
            e,
            log,
            this,
            child,
            transform_shape,
            child_shape,
            motion,
            body_a,
            body_b,
            input,
            dispatcher,
        }
    }

    #[test]
    fn cleanup_cleans_the_child_and_deletes_the_agent() {
        let mut r = rig();
        r.e.call(0x00d4_29c0, &args![r.this, 0x44u32]);
        assert_eq!(calls(&r.log, CHILD + 0x1c), vec![vec![r.child, 0x44]]);
        assert_eq!(calls(&r.log, THIS), vec![vec![r.this, 1]]);
    }

    #[test]
    fn calc_content_statistics_names_the_child_agent() {
        let mut r = rig();
        let collector = logging_object(&mut r.e, &r.log, COLLECTOR);
        r.e.call(0x00d4_29e0, &args![r.this, collector, 0x99u32]);
        assert_eq!(
            calls(&r.log, COLLECTOR + 0xc),
            vec![vec![collector, 0x010c_f270, r.child, 0]]
        );
    }

    #[test]
    fn forwarders_jump_to_the_child_slot_with_their_word() {
        // Each jump hands its one stack word on: the slots end in RET 4 (00d9fca0, 00d9fcc0,
        // 00d9fcd0, 00d9fce0).
        for (address, slot) in [
            (0x00d4_2a00, 0x24),
            (0x00d4_2a40, 0x2c),
            (0x00d4_2a50, 0x30),
            (0x00d4_2a60, 0x34),
        ] {
            let mut r = rig();
            r.e.call(address, &args![r.this, 0x77u32]);
            assert_eq!(calls(&r.log, CHILD + slot), vec![vec![r.child, 0x77]]);
        }
    }

    #[test]
    fn warp_time_passes_the_floats_quieted() {
        let mut r = rig();
        r.e.call(
            0x00d4_2a10,
            &args![r.this, f32::from_bits(0x7f80_0001), 2.5f32, 0x66u32],
        );
        assert_eq!(
            calls(&r.log, CHILD + 0x28),
            vec![vec![r.child, 0x7fc0_0001, 2.5f32.to_bits(), 0x66]]
        );
    }

    #[test]
    fn child_body_queries_pass_the_child_shape_and_the_product() {
        for (address, slot) in [(0x00d4_2de0, 0x10), (0x00d4_2ed0, 0xc)] {
            let mut r = rig();
            let seen = capture(&mut r.e, &r.log, CHILD + slot, 1, 4, 0);
            r.e.call(
                address,
                &args![r.this, r.body_a, r.body_b, r.input, 0x5151u32],
            );
            let call = &calls(&r.log, CHILD + slot)[0];
            let set_mul = &calls(&r.log, TRANSFORM_SET_MUL)[0];
            assert_eq!(call[2..], [r.body_b, r.input, 0x5151]);
            assert_eq!(set_mul[1..], [r.motion, r.transform_shape + 0x30]);
            assert_eq!(
                seen.borrow()[0],
                vec![r.child_shape, 0, set_mul[0], r.body_a]
            );
            assert_eq!(set_mul[0], call[1] + 0x10);
        }
    }

    #[test]
    fn update_shape_collection_filter_passes_two_words_on() {
        let mut r = rig();
        let seen = capture(&mut r.e, &r.log, CHILD + 0x20, 1, 4, 0);
        r.e.call(0x00d4_2fc0, &args![r.this, r.body_a, r.body_b, 3u32, 4u32]);
        let call = &calls(&r.log, CHILD + 0x20)[0];
        assert_eq!(call[2..], [r.body_b, 3, 4]);
        assert_eq!(seen.borrow()[0][..2], [r.child_shape, 0]);
        assert_eq!(seen.borrow()[0][3], r.body_a);
    }

    /// Makes the dispatcher give `function` as word `slot` of entry `index` for the types of
    /// the fixture (the create matrix at `row`; the static matrix is the one at `0x1a0`).
    fn offer(r: &mut Rig, row: u32, index: u8, slot: u32, function: u32) {
        set_matrix(&mut r.e, r.dispatcher, row, (CHILD_TYPE, OTHER_TYPE), index);
        set_entry(&mut r.e, r.dispatcher, index, slot, function);
    }

    #[test]
    fn static_queries_pick_the_table_word_by_the_types() {
        for (address, slot) in [(0x00d4_2f40, 4), (0x00d4_2e50, 8)] {
            let mut r = rig();
            offer(&mut r, 0x1a0, 2, slot, STATIC_FUNCTION + slot);
            let seen = capture(&mut r.e, &r.log, STATIC_FUNCTION + slot, 0, 4, 0);
            r.e.call(address, &args![r.body_a, r.body_b, r.input, 0x5151u32]);
            let call = &calls(&r.log, STATIC_FUNCTION + slot)[0];
            let set_mul = &calls(&r.log, TRANSFORM_SET_MUL)[0];
            assert_eq!(call[1..], [r.body_b, r.input, 0x5151]);
            assert_eq!(
                seen.borrow()[0],
                vec![r.child_shape, 0, set_mul[0], r.body_a]
            );
        }
    }

    #[test]
    fn static_linear_cast_passes_both_collectors() {
        let mut r = rig();
        offer(&mut r, 0x1a0, 5, 0xc, STATIC_FUNCTION + 0xc);
        log_double(&mut r.e, &r.log, STATIC_FUNCTION + 0xc, 0);
        r.e.call(
            0x00d4_2d60,
            &args![r.body_a, r.body_b, r.input, 0xc1u32, 0xc2u32],
        );
        let call = &calls(&r.log, STATIC_FUNCTION + 0xc)[0];
        assert_eq!(call[1..], [r.body_b, r.input, 0xc1, 0xc2]);
        // The body is followed by the product the cast sees as its motion.
        assert_eq!(call[0] + 0x10, calls(&r.log, TRANSFORM_SET_MUL)[0][0]);
    }

    /// A memory router whose allocator hands out `block` for the 16-byte request.
    fn install_allocator(r: &mut Rig, block: u32) {
        let vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(vtable + 4, ALLOCATE);
        let allocator = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(allocator, vtable);
        let router = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(router + 0x10, allocator);
        r.e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut r.e, &r.log, ALLOCATE, block);
    }

    #[test]
    fn creation_makes_the_child_agent_and_fills_the_header() {
        // The direct function takes the transform body first; the symmetric one second.
        for (address, vtable, swapped, row, flag) in [
            (0x00d4_2a70, 0x010c_f280, false, 0x1a0, 0),
            (0x00d4_3030, 0x010c_f2c8, true, 0x5a0, 1),
        ] {
            let mut r = rig();
            let block = r.e.mem.alloc(0x10);
            install_allocator(&mut r, block);
            r.e.mem.set_u32(r.input + 0x14, flag);
            for offset in 0..0xb0 {
                r.e.mem.set_u8(r.motion + offset, (offset as u8) ^ 0x55);
            }
            offer(&mut r, row, 1, 0, CREATE_FUNCTION);
            let seen = snapshot(&mut r.e, &r.log, CREATE_FUNCTION, 0, 0x4141);
            let (first, second) = if swapped {
                (r.body_b, r.body_a)
            } else {
                (r.body_a, r.body_b)
            };
            let agent =
                r.e.call(address, &args![first, second, r.input, 0x77u32])
                    .u32();
            assert_eq!(agent, block);
            assert_eq!(r.e.mem.u16(block + 4), 0x10);
            assert_eq!(r.e.mem.u16(block + 6), 1);
            assert_eq!(r.e.mem.u32(block + 8), 0x77);
            assert_eq!(r.e.mem.u32(block + 0xc), 0x4141);
            assert_eq!(r.e.mem.u32(block), vtable);
            assert_eq!(calls(&r.log, ALLOCATE)[0][1..], [0x10]);
            let created = &calls(&r.log, CREATE_FUNCTION)[0];
            let other = r.body_b;
            assert_eq!(created[1..], [other, r.input, 0x77]);
            let seen = seen.borrow()[0].clone();
            let state = created[0] + 0x10;
            assert_eq!(word_at(&seen, 0), r.child_shape);
            assert_eq!(word_at(&seen, 4), 0);
            assert_eq!(word_at(&seen, 8), state);
            assert_eq!(
                word_at(&seen, 12),
                r.body_a,
                "the parent is the body with the transform shape"
            );
            // The state is a copy of the motion state (the product is a double here).
            for offset in 0x40..0xaf {
                assert_eq!(seen[0x10 + offset], (offset as u8) ^ 0x55);
            }
            let set_mul = &calls(&r.log, TRANSFORM_SET_MUL)[0];
            assert_eq!(set_mul[..2], [state, r.motion]);
        }
    }

    #[test]
    fn registration_hands_the_dispatcher_both_entries() {
        let mut e = engine();
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut entry = words(e, a[1], 4);
            entry.push(u32::from(e.mem.u8(a[1] + 0x10)));
            entry.push(u32::from(e.mem.u8(a[1] + 0x11)));
            entry.extend_from_slice(a);
            copies.borrow_mut().push(entry);
            Ret::default()
        });
        e.call(0x00d4_3120, &args![0x0777_0000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen[0][..6],
            [0x00d4_3030, 0x00d4_32f0, 0x00d4_3330, 0x00d4_3370, 1, 1]
        );
        assert_eq!(seen[0][6..], [0x0777_0000, seen[0][7], 0xffff_ffff, 0xf]);
        assert_eq!(
            seen[1][..6],
            [0x00d4_2a70, 0x00d4_2f40, 0x00d4_2e50, 0x00d4_2d60, 0, 1]
        );
        assert_eq!(seen[1][6..], [0x0777_0000, seen[1][7], 0xf, 0xffff_ffff]);
    }

    /// Lays out the transform shape and motion of the rig for `processCollision`: columns
    /// (1, 2, 3, 4), (5, 6, 7, 8), (9, 10, 11, 12) at `+0x30`, translation (0.5, 1.5, 2.5, 3.5)
    /// at `+0x60`, position (10, 20, 30, 40) at motion `+0x80`, marker bytes in the copied
    /// ranges and a signalling NaN at motion `+0xA0`.
    fn lay_out_motion(r: &mut Rig) {
        let columns = [
            [1.0f32, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [0.5, 1.5, 2.5, 3.5],
        ];
        for (index, column) in columns.iter().enumerate() {
            for (lane, value) in column.iter().enumerate() {
                r.e.mem.set_f32(
                    r.transform_shape + 0x30 + 0x10 * index as u32 + 4 * lane as u32,
                    *value,
                );
            }
        }
        for (lane, value) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
            r.e.mem.set_f32(r.motion + 0x80 + 4 * lane as u32, value);
        }
        for offset in (0x40..0x60).chain(0x90..0xa0) {
            r.e.mem.set_u8(r.motion + offset, offset as u8);
        }
        r.e.mem.set_u32(r.motion + 0xa0, 0x7f80_0001);
    }

    #[test]
    fn process_collision_builds_the_child_motion_and_calls_the_child() {
        let mut r = rig();
        let (stream, records) = monitor(&mut r.e, 0x100);
        lay_out_motion(&mut r);
        // The child body and the state after it are copied while the child runs.
        let seen = snapshot(&mut r.e, &r.log, CHILD + 0x18, 1, 0);
        r.e.call(
            0x00d4_2b50,
            &args![r.this, r.body_a, r.body_b, r.input, 0x6161u32],
        );
        // Two profile records: the start label and the end label.
        assert_eq!(
            profile_labels(&r.e, stream, records),
            vec![0x010c_f2b8, 0x010c_41f8]
        );
        let set_mul = &calls(&r.log, TRANSFORM_SET_MUL)[0];
        let state = set_mul[0];
        assert_eq!(set_mul[1..], [r.motion, r.transform_shape + 0x30]);
        let quaternions = calls(&r.log, QUATERNION_SET_MUL);
        assert_eq!(
            quaternions,
            vec![
                vec![state + 0x60, r.motion + 0x60, r.transform_shape + 0x20],
                vec![state + 0x70, r.motion + 0x70, r.transform_shape + 0x20],
            ]
        );
        let seen = seen.borrow()[0].clone();
        // The child body: the child shape, key 0, the state and the parent.
        let child_call = &calls(&r.log, CHILD + 0x18)[0];
        assert_eq!(child_call[2..], [r.body_b, r.input, 0x6161]);
        assert_eq!(word_at(&seen, 0), r.child_shape);
        assert_eq!(word_at(&seen, 4), 0);
        assert_eq!(word_at(&seen, 8), state);
        assert_eq!(word_at(&seen, 12), r.body_a);
        // Bytes copied as they are (the state follows the body at +0x14, past a saved word).
        for offset in (0x40..0x60).chain(0x90..0xa0) {
            assert_eq!(seen[0x14 + offset], offset as u8);
        }
        // The float at +0xA0 comes back quiet.
        assert_eq!(word_at(&seen, 0x14 + 0xa0), 0x7fc0_0001);
        // The vector at +0x80: (position - translation) = (9.5, 18.5, 27.5, 36.5) against
        // the columns, the way the decompiled expression writes it.
        let (vx, vy, vz) = (9.5f32, 18.5f32, 27.5f32);
        let expected = [
            vy * 2.0 + vx * 1.0 + 3.0 * vz,
            vy * 6.0 + vx * 5.0 + 7.0 * vz,
            vy * 10.0 + vx * 9.0 + 11.0 * vz,
            vy * 12.0 + vx * 10.0 + 12.0 * vz,
        ];
        let lanes = [0, 4, 8, 12].map(|lane| f32::from_bits(word_at(&seen, 0x14 + 0x80 + lane)));
        assert_eq!(lanes, expected);
    }

    #[test]
    fn symmetric_queries_swap_the_bodies_and_flip_the_collector() {
        for (address, slot, vtable, second_word) in [
            (0x00d4_3460, 0xc, FLIP_BODY_COLLECTOR_VTABLE, 0),
            (0x00d4_34a0, 0x10, FLIP_COLLECTOR_VTABLE, 0x7fc0_0001),
        ] {
            let mut r = rig();
            r.e.mem.set_u32(LARGE_FLOAT, 0x7f80_0001);
            let flip = capture(&mut r.e, &r.log, CHILD + slot, 4, 3, 0);
            r.e.call(
                address,
                &args![r.this, r.body_b, r.body_a, r.input, 0x5151u32],
            );
            let call = &calls(&r.log, CHILD + slot)[0];
            assert_eq!(call[2..4], [r.body_b, r.input]);
            assert_eq!(*flip.borrow(), vec![vec![vtable, second_word, 0x5151]]);
        }
    }

    #[test]
    fn symmetric_update_shape_collection_filter_swaps_the_bodies() {
        let mut r = rig();
        r.e.call(0x00d4_3600, &args![r.this, r.body_b, r.body_a, 3u32, 4u32]);
        assert_eq!(
            calls(&r.log, CHILD + 0x20)[0][2..],
            [r.body_b, 3, 4],
            "the first body is the one with the transform shape"
        );
    }

    #[test]
    fn symmetric_static_queries_swap_the_bodies_and_flip_the_collector() {
        for (address, slot, vtable, second_word) in [
            (0x00d4_32f0, 4, FLIP_BODY_COLLECTOR_VTABLE, 0),
            (0x00d4_3330, 8, FLIP_COLLECTOR_VTABLE, 0x7fc0_0001),
        ] {
            let mut r = rig();
            r.e.mem.set_u32(LARGE_FLOAT, 0x7f80_0001);
            offer(&mut r, 0x1a0, 2, slot, STATIC_FUNCTION + slot);
            let flip = capture(&mut r.e, &r.log, STATIC_FUNCTION + slot, 3, 3, 0);
            r.e.call(address, &args![r.body_b, r.body_a, r.input, 0x5151u32]);
            let call = &calls(&r.log, STATIC_FUNCTION + slot)[0];
            assert_eq!(call[1..3], [r.body_b, r.input]);
            assert_eq!(*flip.borrow(), vec![vec![vtable, second_word, 0x5151]]);
        }
    }

    /// The path vector (1, 2, 3, 4) at `input + 0x50`, the sign masks in memory and a marker
    /// word at the end of the 0x70-byte input.
    fn prepare_path(r: &mut Rig) {
        for lane in 0..4u32 {
            r.e.mem
                .set_f32(r.input + 0x50 + 4 * lane, (lane + 1) as f32);
            r.e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
        }
        r.e.mem.set_u32(r.input + 0x6c, 0x6c6c);
    }

    /// What a flip-cast collector held: vtable, large float, the path vector and the wrapped
    /// collector.
    fn flip_cast(wrapped: u32) -> Vec<u32> {
        let mut expected = vec![FLIP_CAST_COLLECTOR_VTABLE, 0x7f7f_ffee];
        expected.extend([1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits));
        expected.push(wrapped);
        expected
    }

    /// The vtable and large float at `collector` followed by the path vector and the wrapped
    /// collector at `+0x10`.
    fn collector_words(e: &Engine, collector: u32) -> Vec<u32> {
        let mut out = words(e, collector, 2);
        out.extend(words(e, collector + 0x10, 5));
        out
    }

    /// What a cast function saw: the negated path of the input copy, the marker word after
    /// it and the words of the two collectors (empty for a null second).
    type CastSeen = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>)>>>;

    /// A double at `address` that logs and copies what the cast function receives: the input
    /// copy at argument `copy`, the collectors at `first` and `first + 1`.
    fn cast_double(e: &mut Engine, log: &Log, address: u32, copy: usize) -> CastSeen {
        let seen: CastSeen = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let second = if a[copy + 2] == 0 {
                Vec::new()
            } else {
                collector_words(e, a[copy + 2])
            };
            copies.borrow_mut().push((
                words(e, a[copy] + 0x50, 4),
                words(e, a[copy] + 0x6c, 1),
                collector_words(e, a[copy + 1]),
                second,
            ));
            Ret::default()
        });
        seen
    }

    #[test]
    fn symmetric_static_linear_cast_negates_the_path_and_wraps_both_collectors() {
        for start in [0xc2u32, 0] {
            let mut r = rig();
            prepare_path(&mut r);
            offer(&mut r, 0x1a0, 5, 0xc, STATIC_FUNCTION + 0xc);
            let seen = cast_double(&mut r.e, &r.log, STATIC_FUNCTION + 0xc, 2);
            r.e.call(
                0x00d4_3370,
                &args![r.body_b, r.body_a, r.input, 0xc1u32, start],
            );
            let call = &calls(&r.log, STATIC_FUNCTION + 0xc)[0];
            assert_eq!(call[1], r.body_b);
            assert_ne!(call[2], r.input, "the input is copied");
            let seen = seen.borrow();
            assert_eq!(
                seen[0].0,
                [-1.0f32, -2.0, -3.0, -4.0].map(f32::to_bits).to_vec()
            );
            assert_eq!(seen[0].1, vec![0x6c6c]);
            assert_eq!(seen[0].2, flip_cast(0xc1));
            if start == 0 {
                assert_eq!(call[4], 0);
            } else {
                assert_eq!(seen[0].3, flip_cast(start));
            }
        }
    }

    #[test]
    fn symmetric_linear_cast_runs_the_agent_with_the_swapped_bodies() {
        let mut r = rig();
        prepare_path(&mut r);
        let seen = cast_double(&mut r.e, &r.log, CHILD + 0x14, 3);
        r.e.call(
            0x00d4_3620,
            &args![r.this, r.body_b, r.body_a, r.input, 0xc1u32, 0xc2u32],
        );
        let call = &calls(&r.log, CHILD + 0x14)[0];
        assert_eq!(call[2], r.body_b);
        assert_ne!(call[3], r.input);
        let seen = seen.borrow();
        assert_eq!(
            seen[0].0,
            [-1.0f32, -2.0, -3.0, -4.0].map(f32::to_bits).to_vec()
        );
        assert_eq!(seen[0].1, vec![0x6c6c]);
        assert_eq!(seen[0].2, flip_cast(0xc1));
        assert_eq!(seen[0].3, flip_cast(0xc2));
        // The record built from the transform shape of the body A is the first argument.
        assert_eq!(calls(&r.log, TRANSFORM_SET_MUL).len(), 1);
    }

    /// Runs the symmetric `processCollision` with a child agent that appends two contacts to
    /// the output and sets the time of impact to `new_time`. Before the call the output holds
    /// time `old_time`, the TOI normal (1, 2, 3, 4) at `+0x3020`, the bytes `toi_bytes` at
    /// `+0x3048` and the four words 10 to 13 at `+0x3040`. Returns the rig and the output.
    fn symmetric_process(old_time: f32, new_time: f32, toi_bytes: [u8; 3]) -> (Rig, u32) {
        let mut r = rig();
        monitor(&mut r.e, 0x100);
        let output = r.e.mem.alloc(0x3100);
        let contacts = r.e.mem.alloc(0x100);
        r.e.mem.set_u32(output, contacts);
        r.e.mem.set_f32(output + 0x3030, old_time);
        for lane in 0..4u32 {
            r.e.mem
                .set_f32(output + 0x3020 + 4 * lane, (lane + 1) as f32);
        }
        for (index, byte) in toi_bytes.into_iter().enumerate() {
            r.e.mem.set_u8(output + 0x3048 + index as u32, byte);
        }
        for index in 0..4u32 {
            r.e.mem
                .set_u16(output + 0x3040 + 2 * index, 10 + index as u16);
        }
        for lane in 0..3u32 {
            r.e.mem.set_u32(0x010c_4c00 + 4 * lane, 0x8000_0000);
        }
        let log = Rc::clone(&r.log);
        r.e.register_double(CHILD + 0x18, move |e, a| {
            log.borrow_mut().push((CHILD + 0x18, a.to_vec()));
            let end = e.mem.u32(a[4]);
            for index in 0..2u32 {
                let point = end + 0x30 * index;
                for (lane, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                    e.mem.set_f32(point + 4 * lane as u32, v + index as f32);
                }
                for (lane, v) in [0.5f32, 0.0, 1.0, 2.0].into_iter().enumerate() {
                    e.mem.set_f32(point + 0x10 + 4 * lane as u32, v);
                }
            }
            e.mem.set_u32(a[4], end + 0x60);
            e.mem.set_f32(a[4] + 0x3030, new_time);
            Ret::default()
        });
        r.e.call(
            0x00d4_34e0,
            &args![r.this, r.body_b, r.body_a, r.input, output],
        );
        (r, output)
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contacts() {
        let (r, output) = symmetric_process(0.5, 0.5, [1, 2, 0x12]);
        let call = &calls(&r.log, CHILD + 0x18)[0];
        assert_eq!(call[2..], [r.body_b, r.input, output]);
        let contacts = r.e.mem.u32(output) - 0x60;
        for index in 0..2u32 {
            let point = contacts + 0x30 * index;
            // position += normal.w * normal in every lane; the normal is negated in lanes 0 to 2.
            let position: Vec<f32> = (0..4).map(|lane| r.e.mem.f32(point + 4 * lane)).collect();
            let base = 1.0 + index as f32;
            assert_eq!(
                position,
                vec![base + 1.0, base + 1.0, base + 2.0 + 2.0, base + 3.0 + 4.0]
            );
            assert_eq!(
                words(&r.e, point + 0x10, 4),
                vec![
                    (-0.5f32).to_bits(),
                    0x8000_0000,
                    (-1.0f32).to_bits(),
                    2.0f32.to_bits()
                ]
            );
        }
        // The time did not change: the time-of-impact data stays.
        assert_eq!(
            words(&r.e, output + 0x3020, 4),
            [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits)
        );
        assert_eq!(r.e.mem.u8(output + 0x304a), 0x12);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes() {
        let (r, output) = symmetric_process(0.5, 0.25, [1, 2, 0x12]);
        assert_eq!(
            words(&r.e, output + 0x3020, 4),
            [-1.0f32, -2.0, -3.0, 4.0].map(f32::to_bits)
        );
        assert_eq!(r.e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(r.e.mem.u8(output + 0x3049), 1);
        assert_eq!(r.e.mem.u8(output + 0x3048), 2);
        // Index 2 + 1 - 1 = 2: words 0 and 2 exchange; the bytes' AND is 0.
        let toi: Vec<u16> = (0..4)
            .map(|i| r.e.mem.u16(output + 0x3040 + 2 * i))
            .collect();
        assert_eq!(toi, vec![12, 11, 10, 13]);
    }

    #[test]
    fn symmetric_process_collision_swaps_the_middle_words_for_two_and_two() {
        let (r, output) = symmetric_process(0.5, 0.25, [2, 2, 0x34]);
        assert_eq!(r.e.mem.u8(output + 0x304a), 0x43);
        let toi: Vec<u16> = (0..4)
            .map(|i| r.e.mem.u16(output + 0x3040 + 2 * i))
            .collect();
        assert_eq!(toi, vec![13, 12, 11, 10]);
    }

    #[test]
    fn symmetric_process_collision_treats_a_nan_time_as_changed() {
        let (r, output) = symmetric_process(f32::NAN, f32::NAN, [1, 2, 0x12]);
        assert_eq!(
            words(&r.e, output + 0x3020, 4),
            [-1.0f32, -2.0, -3.0, 4.0].map(f32::to_bits)
        );
    }
}
