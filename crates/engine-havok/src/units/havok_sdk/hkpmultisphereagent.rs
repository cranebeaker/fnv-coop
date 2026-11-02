//! `Havok SDK/hkpmultisphereagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMultiSphereAgent` (Xbox PDB) holds, from `+0xc`, an `hkArray` of 8-byte entries
//! (`{sphere index, agent}`: data `+0xc`, size `+0x10`, capacity and flags `+0x14`, four
//! entries of inline storage at `+0x18`), one agent per sphere of the multi sphere shape
//! (`m_numSpheres` at `+0x10` of the shape, the spheres from `+0x20`, 16 bytes each with the
//! radius in the fourth lane). `+8` is the contact manager the constructor is given.
//!
//! Every agent function builds the same locals on the stack: an `hkpCdBody` (`m_shape` +0,
//! `m_shapeKey` +4, `m_motion` +8, `m_parent` +0xc), an `hkpSphereShape` built with radius 0
//! (`00cd8160`: radius at `+0x10`, shape type at `+0xc`) and an `hkMotionState` copied from the
//! body's (`00d431b0`). For each sphere it sets the motion state's translation (`+0x30`) to
//! the body's plus the sphere centre rotated by the three columns of that copy (SSE, lane by
//! lane: `(y * column1 + x * column0) + z * column2`), the sphere shape's radius, and calls
//! the agent of that sphere with the address of the cd body. Here they are laid out in one
//! block of [`FRAME_SIZE`] bytes in that order (cd body, sphere, motion state), as the exe's
//! frames have them, so the callee sees the same three pointers.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMotionState::hkMotionState` copy constructor (`00d431b0`, `this`, source).
const MOTION_STATE_COPY: u32 = 0x00d4_31b0;
/// `hkpSphereShape::hkpSphereShape` (`00cd8160`, `this`, radius as a `float`).
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// Registers one group of agent functions with the dispatcher (`00cfbbe0`, `this` = the
/// dispatcher, `(functions, type a, type b)`).
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The vtable of the agent (`010cf354`) and the one every destructor leaves (`0102e388`).
const AGENT_VTABLE: u32 = 0x010c_f354;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The `float` `0x7f7fffee` (`0104da48`) the flipped records start from.
pub(crate) const MAX_FLOAT: u32 = 0x0104_da48;
/// Four sign masks (`010c4c00`) and the mask the input path is negated with (`010c44d0`),
/// both applied with `XORPS`.
pub(crate) const SIGN_MASK: u32 = 0x010c_4c00;
const PATH_SIGN_MASK: u32 = 0x010c_44d0;
/// Vtables of the flipping records the symmetric functions build on the stack: for
/// `getPenetrations` (`010ce81c`), `getClosestPoints` (`010ce7fc`) and `linearCast`
/// (`010ce80c`). Their classes are unconfirmed.
pub(crate) const FLIPPED_PENETRATIONS_VTABLE: u32 = 0x010c_e81c;
pub(crate) const FLIPPED_CLOSEST_VTABLE: u32 = 0x010c_e7fc;
const FLIPPED_CAST_VTABLE: u32 = 0x010c_e80c;

/// `m_agents` (unconfirmed name): data `+0xc`, size `+0x10`, capacity and flags `+0x14`.
const AGENTS: u32 = 0x0c;
/// The four entries of inline storage.
const AGENTS_INLINE: u32 = 0x18;
/// Bit 31 of a capacity word: the storage is not owned.
const DONT_DEALLOCATE_FLAG: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The dispatcher's tables: the agent type is the byte at
/// `table + (type of A << 5) + type of B`, whose 0x14-byte function record starts at
/// `0x9a0 + type * 0x14`. `0x5a0` is the table the constructor uses for predictive agents.
const TABLE_AGENT: u32 = 0x1a0;
const TABLE_PREDICTIVE: u32 = 0x5a0;
const FUNCTIONS: u32 = 0x9a0;
const SLOT_CREATE: u32 = 0x00;
const SLOT_GET_PENETRATIONS: u32 = 0x04;
const SLOT_GET_CLOSEST_POINTS: u32 = 0x08;
const SLOT_LINEAR_CAST: u32 = 0x0c;

/// The local frame: `hkpCdBody` at 0, the sphere shape at `0x10` (radius `+0x10`, type
/// `+0xc`), the motion state at `0x30` (columns at `+0`, `+0x10`, `+0x20`, translation
/// `+0x30`, the swept centres `+0x40` and `+0x50`).
const FRAME_SIZE: u32 = 0x100;
const CD_SHAPE: u32 = 0x00;
const CD_SHAPE_KEY: u32 = 0x04;
const CD_MOTION: u32 = 0x08;
const CD_PARENT: u32 = 0x0c;
const SPHERE: u32 = 0x10;
const SPHERE_TYPE: u32 = 0x0c;
const SPHERE_RADIUS: u32 = 0x10;
const MOTION: u32 = 0x30;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// The sphere centre `v` rotated by the three columns at `motion`:
/// `(y * column1 + x * column0) + z * column2` in every lane.
fn rotate(e: &Engine, motion: u32, v: Vec4) -> Vec4 {
    let column0 = load_vec(e, motion);
    let column1 = load_vec(e, motion + 0x10);
    let column2 = load_vec(e, motion + 0x20);
    std::array::from_fn(|lane| (v[1] * column1[lane] + v[0] * column0[lane]) + v[2] * column2[lane])
}

/// Builds the locals of an agent function the way the exe does: the motion state copied from
/// the body's (`body_a + 8`), the sphere built with radius 0, then the cd body's motion and
/// parent pointers.
fn build_locals(e: &mut Engine, frame: u32, body_a: u32) {
    let motion = e.mem.u32(body_a + 8);
    e.call(MOTION_STATE_COPY, &args![frame + MOTION, motion]);
    e.call(SPHERE_CONSTRUCT, &args![frame + SPHERE, 0.0f32]);
    e.mem.set_u32(frame + CD_MOTION, frame + MOTION);
    e.mem.set_u32(frame + CD_PARENT, body_a);
}

/// Puts one sphere (at `sphere`: centre and radius) into the locals: each of the `outputs`
/// (offsets in the motion state: `0x30`, and the swept centres `0x40` and `0x50`) becomes the
/// body's vector at that offset plus the rotated centre; the radius goes through the x87
/// stack (`FLD` / `FSTP`) into the local sphere shape, which the cd body then names.
fn place_sphere(e: &mut Engine, frame: u32, body_a: u32, sphere: u32, outputs: &[u32]) {
    let rotated = rotate(e, frame + MOTION, load_vec(e, sphere));
    let motion = e.mem.u32(body_a + 8);
    for &offset in outputs {
        let base = load_vec(e, motion + offset);
        store_vec(
            e,
            frame + MOTION + offset,
            std::array::from_fn(|lane| base[lane] + rotated[lane]),
        );
    }
    let radius = x87_quiet_bits(e.mem.u32(sphere + 0xc));
    e.mem.set_u32(frame + SPHERE + SPHERE_RADIUS, radius);
    e.mem.set_u32(frame + CD_SHAPE, frame + SPHERE);
}

/// The agent function the dispatcher names for the local sphere (`own_type`) and the other
/// body's shape type: the agent type is the byte at `table + (own_type << 5) + other_type`,
/// and the function is the word at `slot` of that type's record.
fn agent_function(
    e: &Engine,
    dispatcher: u32,
    table: u32,
    own_type: u32,
    other_type: u32,
    slot: u32,
) -> u32 {
    let kind = e.mem.u8(dispatcher
        .wrapping_add(table)
        .wrapping_add(own_type << 5)
        .wrapping_add(other_type));
    e.mem.u32(
        dispatcher
            .wrapping_add(u32::from(kind) * 0x14)
            .wrapping_add(FUNCTIONS + slot),
    )
}

/// The shape type of body `b`: `m_type` (`+0xc`) of the shape at `+0`.
fn shape_type(e: &Engine, body: u32) -> u32 {
    let shape = e.mem.u32(body);
    e.mem.u32(shape + 0xc)
}

// Translated from 00d443a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::cleanup` (Xbox PDB): `ECX` = agent, one stack word, `owner`. Calls
/// every agent of the array in order through its vtable slot `0x1c` with `owner` (the size
/// and the data pointer are read again on each pass), then the agent's own destructor
/// (slot 0) with the flag 1.
pub fn hkp_multi_sphere_agent_cleanup(e: &mut Engine, this: Ptr, owner: u32) {
    let me = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(me + AGENTS + 4) {
        let data = e.mem.u32(me + AGENTS);
        let agent = e.mem.u32(data + index * 8 + 4);
        e.vcall(agent, 0x1c, &args![owner]);
        index += 1;
    }
    e.vcall(me, 0, &args![1u32]);
}

// Translated from 00d443e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::processCollision` (Xbox PDB): `ECX` = agent, four stack words
/// (`body_a`, `body_b`, `input`, `output`; names from the Xbox PDB signature). For each entry
/// of the array in order (the shape key counts down from `size - 1`) the sphere named by the
/// entry's index is placed, with all three motion state vectors (`+0x30`, `+0x40`, `+0x50`)
/// offset by it, and the entry's agent is called through slot `0x18` with
/// `(cd body, body_b, input, output)`.
pub fn hkp_multi_sphere_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    let me = this.addr();
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let mut entry = e.mem.u32(me + AGENTS);
        let mut key = e.mem.i32(me + AGENTS + 4) - 1;
        while key >= 0 {
            let sphere = shape.wrapping_add(e.mem.u32(entry).wrapping_add(2) << 4);
            place_sphere(e, frame, body_a, sphere, &[0x30, 0x40, 0x50]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, key as u32);
            let agent = e.mem.u32(entry + 4);
            e.vcall(agent, 0x18, &args![frame, body_b, input, output]);
            entry += 8;
            key -= 1;
        }
    });
}

// Translated from 00d444f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::getClosestPoints` (Xbox PDB): as [`hkp_multi_sphere_agent_process_collision`]
/// but only the translation (`+0x30`) is set, and the agent is called through slot `0x10`.
pub fn hkp_multi_sphere_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    let me = this.addr();
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let mut entry = e.mem.u32(me + AGENTS);
        let mut key = e.mem.i32(me + AGENTS + 4) - 1;
        while key >= 0 {
            let sphere = shape.wrapping_add(e.mem.u32(entry).wrapping_add(2) << 4);
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, key as u32);
            let agent = e.mem.u32(entry + 4);
            e.vcall(agent, 0x10, &args![frame, body_b, input, output]);
            entry += 8;
            key -= 1;
        }
    });
}

// Translated from 00d445e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cdecl agent function `getClosestPoints` registered for the pair (no map name; four stack
/// words, `body_a`, `body_b`, `input`, `output`). Walks the shape's spheres in order; the shape
/// key is the sphere's index. For each, the function the dispatcher (`[input]`) names in table
/// `0x1a0` for the local sphere's type and `body_b`'s shape type (record word `0x9a8`) is called
/// with `(cd body, body_b, input, output)`.
pub fn fn_00d445e0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, output: u32) {
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let other_type = shape_type(e, body_b);
        let mut index = 0i32;
        while index < e.mem.i32(shape + 0x10) {
            let sphere = shape + 0x20 + 0x10 * index as u32;
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, index as u32);
            let dispatcher = e.mem.u32(input);
            let own_type = e.mem.u32(frame + SPHERE + SPHERE_TYPE);
            let function = agent_function(
                e,
                dispatcher,
                TABLE_AGENT,
                own_type,
                other_type,
                SLOT_GET_CLOSEST_POINTS,
            );
            e.call(function, &args![frame, body_b, input, output]);
            index += 1;
        }
    });
}

// Translated from 00d446f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::linearCast` (Xbox PDB): `ECX` = agent, five stack words (`body_a`,
/// `body_b`, `input`, `output`, `early_out`). Counts the sphere index down from `size - 1`; the
/// sphere is the one at that index of the shape and the shape key is the index. The agent is
/// always the one of the FIRST entry of the array (the exe reads `data[0].agent` on every pass,
/// not the entry of the sphere): it is called through slot `0x14` with
/// `(cd body, body_b, input, output, early_out)`.
pub fn hkp_multi_sphere_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
    early_out: u32,
) {
    let me = this.addr();
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let data = e.mem.u32(me + AGENTS);
        let mut index = e.mem.i32(me + AGENTS + 4) - 1;
        while index >= 0 {
            let sphere = shape.wrapping_add((index as u32).wrapping_add(2) << 4);
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, index as u32);
            let agent = e.mem.u32(data + 4);
            e.vcall(agent, 0x14, &args![frame, body_b, input, output, early_out]);
            index -= 1;
        }
    });
}

// Translated from 00d447f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cdecl agent function `linearCast` registered for the pair (no map name; five stack words,
/// `body_a`, `body_b`, `input`, `output`, `early_out`). As [`fn_00d445e0`] with the record word
/// `0x9ac`; the function gets `(cd body, body_b, input, output, early_out)`.
pub fn fn_00d447f0(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
    early_out: u32,
) {
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let other_type = shape_type(e, body_b);
        let mut index = 0i32;
        while index < e.mem.i32(shape + 0x10) {
            let sphere = shape + 0x20 + 0x10 * index as u32;
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, index as u32);
            let dispatcher = e.mem.u32(input);
            let own_type = e.mem.u32(frame + SPHERE + SPHERE_TYPE);
            let function = agent_function(
                e,
                dispatcher,
                TABLE_AGENT,
                own_type,
                other_type,
                SLOT_LINEAR_CAST,
            );
            e.call(function, &args![frame, body_b, input, output, early_out]);
            index += 1;
        }
    });
}

// Translated from 00d44900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::getPenetrations` (Xbox PDB): `ECX` = agent, four stack words
/// (`body_a`, `body_b`, `input`, `collector`). As [`hkp_multi_sphere_agent_get_closest_points`]
/// with slot `0xc` and the agent called with `(cd body, body_b, input, collector)`; after each
/// call the loop stops when the byte at `collector + 4` is not zero.
pub fn hkp_multi_sphere_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    let me = this.addr();
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let mut entry = e.mem.u32(me + AGENTS);
        let mut key = e.mem.i32(me + AGENTS + 4) - 1;
        while key >= 0 {
            let sphere = shape.wrapping_add(e.mem.u32(entry).wrapping_add(2) << 4);
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, key as u32);
            let agent = e.mem.u32(entry + 4);
            e.vcall(agent, 0xc, &args![frame, body_b, input, collector]);
            if e.mem.u8(collector + 4) != 0 {
                break;
            }
            entry += 8;
            key -= 1;
        }
    });
}

// Translated from 00d449f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cdecl agent function `getPenetrations` registered for the pair (no map name; four stack
/// words, `body_a`, `body_b`, `input`, `collector`). As [`fn_00d445e0`] with the record word
/// `0x9a4`; after each call the loop stops when the byte at `collector + 4` is not zero.
pub fn fn_00d449f0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, collector: u32) {
    let shape = e.mem.u32(body_a);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        build_locals(e, frame, body_a);
        let other_type = shape_type(e, body_b);
        let mut index = 0i32;
        while index < e.mem.i32(shape + 0x10) {
            let sphere = shape + 0x20 + 0x10 * index as u32;
            place_sphere(e, frame, body_a, sphere, &[0x30]);
            e.mem.set_u32(frame + CD_SHAPE_KEY, index as u32);
            let dispatcher = e.mem.u32(input);
            let own_type = e.mem.u32(frame + SPHERE + SPHERE_TYPE);
            let function = agent_function(
                e,
                dispatcher,
                TABLE_AGENT,
                own_type,
                other_type,
                SLOT_GET_PENETRATIONS,
            );
            e.call(function, &args![frame, body_b, input, collector]);
            if e.mem.u8(collector + 4) != 0 {
                break;
            }
            index += 1;
        }
    });
}

// Translated from 00d44b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::hkpMultiSphereAgent` (Xbox PDB): `ECX` = agent, four stack words
/// (`body_a`, `body_b`, `input`, `contact_manager`). Sets `+8` to the contact manager, the vtable
/// `010cf354`, the 16-bit field at `+6` to 1 and the array to empty over its four inline entries
/// (capacity word `0x80000004`). When the shape's sphere count exceeds the capacity, reserves
/// `max(2 * capacity, count)` entries of 8 bytes (`00c908c0` with the allocator of the memory
/// router). Then for each sphere in order a new entry `{index, agent}` is appended (the size is
/// raised before the call, the entry is written after it): the agent comes from the record word
/// `0x9a0` the dispatcher (`[input]`) names for the local sphere's type and `body_b`'s shape type,
/// in table `0x5a0` when the word at `input + 0x14` is not zero and `0x1a0` otherwise, called with
/// `(cd body, body_b, input, contact_manager)`. Returns `this`. The sphere count is read once.
pub fn hkp_multi_sphere_agent_construct(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    contact_manager: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, contact_manager);
    e.mem.set_u32(me, AGENT_VTABLE);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + AGENTS, me + AGENTS_INLINE);
    e.mem.set_u32(me + AGENTS + 4, 0);
    e.mem.set_u32(me + AGENTS + 8, 0x8000_0004);
    let shape = e.mem.u32(body_a);
    let count = e.mem.i32(shape + 0x10);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(me + AGENTS + 8) & CAPACITY_MASK) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, me + AGENTS, wanted as u32, 8u32],
        );
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        e.call(SPHERE_CONSTRUCT, &args![frame + SPHERE, 0.0f32]);
        let motion = e.mem.u32(body_a + 8);
        e.call(MOTION_STATE_COPY, &args![frame + MOTION, motion]);
        e.mem.set_u32(frame + CD_PARENT, body_a);
        e.mem.set_u32(frame + CD_MOTION, frame + MOTION);
        let other_type = shape_type(e, body_b);
        let mut index = 0i32;
        while index < count {
            let sphere = shape + 0x20 + 0x10 * index as u32;
            place_sphere(e, frame, body_a, sphere, &[0x30, 0x40, 0x50]);
            let size = e.mem.u32(me + AGENTS + 4);
            let data = e.mem.u32(me + AGENTS);
            let entry = data.wrapping_add(size.wrapping_mul(8));
            e.mem.set_u32(me + AGENTS + 4, size.wrapping_add(1));
            let dispatcher = e.mem.u32(input);
            let table = if e.mem.u32(input + 0x14) != 0 {
                TABLE_PREDICTIVE
            } else {
                TABLE_AGENT
            };
            e.mem.set_u32(frame + CD_SHAPE_KEY, index as u32);
            let own_type = e.mem.u32(frame + SPHERE + SPHERE_TYPE);
            let function = agent_function(e, dispatcher, table, own_type, other_type, SLOT_CREATE);
            let agent = e
                .call(function, &args![frame, body_b, input, contact_manager])
                .u32();
            e.mem.set_u32(entry, index as u32);
            e.mem.set_u32(entry + 4, agent);
            index += 1;
        }
    });
    this
}

// Translated from 00d44d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the multi sphere agent with `dispatcher` (cdecl, one stack word; no map name): two
/// calls of `00cfbbe0` (`this` = the dispatcher) with a 0x14-byte record on the stack
/// `{create, getPenetrations, getClosestPoints, linearCast, flipped byte, predictive byte}`:
/// first `(00d44d00, 00d44de0, 00d44e20, 00d44e60, 1, 1)` for the types `(-1, 0x15)`, then
/// `(00d44cc0, 00d449f0, 00d445e0, 00d447f0, 0, 1)` for `(0x15, -1)`. The two bytes after
/// the record's flags are not written.
pub fn fn_00d44d50(e: &mut Engine, dispatcher: u32) {
    for (functions, flipped, first, second) in [
        (
            [0x00d4_4d00u32, 0x00d4_4de0, 0x00d4_4e20, 0x00d4_4e60],
            1u8,
            u32::MAX,
            0x15u32,
        ),
        (
            [0x00d4_4cc0u32, 0x00d4_49f0, 0x00d4_45e0, 0x00d4_47f0],
            0u8,
            0x15u32,
            u32::MAX,
        ),
    ] {
        e.with_stack(0x14, |e, record| {
            let record = record.addr();
            for (slot, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(record + 4 * slot as u32, function);
            }
            e.mem.set_u8(record + 0x10, flipped);
            e.mem.set_u8(record + 0x11, 1);
            e.call(REGISTER_AGENT, &args![dispatcher, record, first, second]);
        });
    }
}

/// A flipping record the symmetric functions build on the stack: the vtable, then (for
/// `getClosestPoints`) a `float` copy of `MAX_FLOAT` (through the x87 stack) or (for
/// `getPenetrations`) a byte 0, and at `+8` the caller's collector.
pub(crate) fn with_flipped_record(
    e: &mut Engine,
    vtable: u32,
    closest: bool,
    collector: u32,
    f: impl FnOnce(&mut Engine, u32),
) {
    e.with_stack(0xc, |e, record| {
        let record = record.addr();
        e.mem.set_u32(record + 8, collector);
        if closest {
            let max = x87_quiet_bits(e.mem.u32(MAX_FLOAT));
            e.mem.set_u32(record + 4, max);
        } else {
            e.mem.set_u8(record + 4, 0);
        }
        e.mem.set_u32(record, vtable);
        f(e, record)
    })
}

// Translated from 00d44de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `getPenetrations` function registered for the flipped pair (cdecl, four stack words; no
/// map name): calls [`fn_00d449f0`] with the two bodies swapped, `input`, and a record on the
/// stack `{vtable 010ce81c, byte 0, +8 = collector}` in place of the collector.
pub fn fn_00d44de0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, collector: u32) {
    with_flipped_record(
        e,
        FLIPPED_PENETRATIONS_VTABLE,
        false,
        collector,
        |e, record| fn_00d449f0(e, body_b, body_a, input, record),
    );
}

// Translated from 00d44e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `getClosestPoints` function registered for the flipped pair (cdecl, four stack words; no
/// map name): calls [`fn_00d445e0`] with the two bodies swapped, `input`, and a record
/// `{vtable 010ce7fc, float 0x7f7fffee, +8 = output}` in place of the output.
pub fn fn_00d44e20(e: &mut Engine, body_a: u32, body_b: u32, input: u32, output: u32) {
    with_flipped_record(e, FLIPPED_CLOSEST_VTABLE, true, output, |e, record| {
        fn_00d445e0(e, body_b, body_a, input, record)
    });
}

/// The records `linearCast` builds for the flipped pair: a copy of the 0x70-byte input with the
/// path vector (`+0x50`) negated (`XORPS` with `010c44d0`), a record `{vtable 010ce80c,
/// float 0x7f7fffee, +0x10 = the input's path, +0x20 = output}`, and when `early_out` is not 0 a
/// second one with `early_out` at `+0x20`. Returns the addresses of the copy, the first record
/// and the second (0 when there is none) inside `frame` (0xd0 bytes).
fn flip_cast_records(
    e: &mut Engine,
    frame: u32,
    input: u32,
    output: u32,
    early_out: u32,
) -> (u32, u32, u32) {
    let copy = frame;
    let first = frame + 0x70;
    let second = frame + 0xa0;
    let max = x87_quiet_bits(e.mem.u32(MAX_FLOAT));
    let bytes = e.mem.bytes(input, 0x70);
    e.mem.write(copy, &bytes);
    for lane in 0..4 {
        let path = e.mem.u32(input + 0x50 + 4 * lane);
        let mask = e.mem.u32(PATH_SIGN_MASK + 4 * lane);
        e.mem.set_u32(copy + 0x50 + 4 * lane, path ^ mask);
        e.mem.set_u32(first + 0x10 + 4 * lane, path);
    }
    e.mem.set_u32(first, FLIPPED_CAST_VTABLE);
    e.mem.set_u32(first + 4, max);
    e.mem.set_u32(first + 0x20, output);
    if early_out == 0 {
        return (copy, first, 0);
    }
    e.mem.set_u32(second, FLIPPED_CAST_VTABLE);
    e.mem.set_u32(second + 4, max);
    for lane in 0..4 {
        let path = e.mem.u32(input + 0x50 + 4 * lane);
        e.mem.set_u32(second + 0x10 + 4 * lane, path);
    }
    e.mem.set_u32(second + 0x20, early_out);
    (copy, first, second)
}

// Translated from 00d44e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `linearCast` function registered for the flipped pair (cdecl, five stack words: `body_a`,
/// `body_b`, `input`, `output`, `early_out`; no map name). Calls [`fn_00d447f0`] with the bodies
/// swapped, a copy of `input` whose path is negated, a record standing for `output` and, when
/// `early_out` is not 0, one standing for it (see `flip_cast_records`), else 0.
pub fn fn_00d44e60(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
    early_out: u32,
) {
    e.with_stack(0xd0, |e, frame| {
        let (copy, first, second) = flip_cast_records(e, frame.addr(), input, output, early_out);
        fn_00d447f0(e, body_b, body_a, copy, first, second);
    });
}

// Translated from 00d44f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereAgent>::getPenetrations` (engine map): `ECX` =
/// agent, four stack words. Calls [`hkp_multi_sphere_agent_get_penetrations`] on the same agent
/// with the two bodies swapped, `input`, and a record `{vtable 010ce81c, byte 0,
/// +8 = collector}` in place of the collector.
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    with_flipped_record(
        e,
        FLIPPED_PENETRATIONS_VTABLE,
        false,
        collector,
        |e, record| hkp_multi_sphere_agent_get_penetrations(e, this, body_b, body_a, input, record),
    );
}

// Translated from 00d44f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereAgent>::getClosestPoints` (engine map): `ECX` =
/// agent, four stack words. Calls [`hkp_multi_sphere_agent_get_closest_points`] with the two
/// bodies swapped, `input`, and a record `{vtable 010ce7fc, float 0x7f7fffee, +8 = output}`.
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    with_flipped_record(e, FLIPPED_CLOSEST_VTABLE, true, output, |e, record| {
        hkp_multi_sphere_agent_get_closest_points(e, this, body_b, body_a, input, record)
    });
}

// Translated from 00d44fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereAgent>::processCollision` (engine map): `ECX` =
/// agent, four stack words (`body_a`, `body_b`, `input`, `result`). Remembers `result`'s write
/// pointer (`[result]`) and the `float` at `result + 0x3030`, calls
/// [`hkp_multi_sphere_agent_process_collision`] with the bodies swapped, then flips every contact
/// point it appended (0x30 bytes each, from the old write pointer while below the new one): the
/// position (lane by lane) becomes `position + w * normal` with `normal` at `+0x10` and `w` its
/// fourth lane, and the normal's sign mask `010c4c00` is applied (`XORPS`). If the `float` at
/// `+0x3030` is not equal to the remembered one (unordered counts as different, the exe's
/// `FUCOMPP`), the vector at `+0x3020` is negated with the same mask, the bytes at
/// `+0x3048`/`+0x3049` are exchanged and the one at `+0x304a` has its nibbles swapped, the
/// halfword at `+0x3040` is exchanged with the one `byte[0x3048] + byte[0x3049] - 1` halfwords
/// further (after the exchange), and when both bytes are 2 the halfwords at `+0x3042` and
/// `+0x3044` are exchanged.
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    result: u32,
) {
    let saved = e.mem.f32(result + 0x3030);
    let first_point = e.mem.u32(result);
    hkp_multi_sphere_agent_process_collision(e, this, body_b, body_a, input, result);
    flip_contacts(e, result, first_point, saved);
}

/// What the symmetric `processCollision` functions do after the inner call (shared with the
/// multi sphere triangle agent): flips every contact point appended since `first_point` and
/// the output state when the `float` at `result + 0x3030` is not equal to `saved`.
pub(crate) fn flip_contacts(e: &mut Engine, result: u32, first_point: u32, saved: f32) {
    let mut point = first_point;
    while point < e.mem.u32(result) {
        let normal = load_vec(e, point + 0x10);
        let position = load_vec(e, point);
        let scale = normal[3];
        store_vec(
            e,
            point,
            std::array::from_fn(|lane| scale * normal[lane] + position[lane]),
        );
        for lane in 0..4 {
            let bits = e.mem.u32(point + 0x10 + 4 * lane);
            let mask = e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(point + 0x10 + 4 * lane, bits ^ mask);
        }
        point += 0x30;
    }
    if e.mem.f32(result + 0x3030) != saved {
        for lane in 0..4 {
            let bits = e.mem.u32(result + 0x3020 + 4 * lane);
            let mask = e.mem.u32(SIGN_MASK + 4 * lane);
            e.mem.set_u32(result + 0x3020 + 4 * lane, bits ^ mask);
        }
        let third = e.mem.u8(result + 0x304a);
        let rotated = (third >> 4) | ((third & 0xf) << 4);
        let second = e.mem.u8(result + 0x3049);
        e.mem.set_u8(result + 0x304a, rotated);
        let first = e.mem.u8(result + 0x3048);
        e.mem.set_u8(result + 0x3049, first);
        e.mem.set_u8(result + 0x3048, second);
        let head = e.mem.u16(result + 0x3040);
        let other = u32::from(second)
            .wrapping_add(u32::from(first))
            .wrapping_sub(1);
        let other_at = (result + 0x3040).wrapping_add(other.wrapping_mul(2));
        let moved = e.mem.u16(other_at);
        e.mem.set_u16(result + 0x3040, moved);
        e.mem.set_u16(other_at, head);
        if e.mem.u8(result + 0x3048) & e.mem.u8(result + 0x3049) == 2 {
            let low = e.mem.u16(result + 0x3042);
            let high = e.mem.u16(result + 0x3044);
            e.mem.set_u16(result + 0x3042, high);
            e.mem.set_u16(result + 0x3044, low);
        }
    }
}

// Translated from 00d450f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereAgent::_scalar_deleting_destructor_` (Xbox PDB): `ECX` = agent, one stack
/// word, `flags`. Empties the array (size 0; storage returned through the allocator's slot
/// `0x10` as `(data, capacity * 8)` unless bit 31 of the capacity word says it is not owned),
/// leaves data null and the capacity word `0x80000000`, sets the vtable `0102e388` and, with
/// bit 0 of `flags`, frees the object through the allocator's slot 8 with the size at `+4`.
/// Returns `this`.
pub fn hkp_multi_sphere_agent_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + AGENTS + 8);
    e.mem.set_u32(me + AGENTS + 4, 0);
    if capacity & DONT_DEALLOCATE_FLAG == 0 {
        let data = e.mem.u32(me + AGENTS);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 3],
        );
    }
    e.mem.set_u32(me + AGENTS, 0);
    e.mem.set_u32(me + AGENTS + 8, DONT_DEALLOCATE_FLAG);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(allocator, 8, &args![me, size]);
    }
    this
}

// Translated from 00d45160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpMultiSphereAgent>::linearCast` (engine map): `ECX` = agent, five stack
/// words (`body_a`, `body_b`, `input`, `output`, `early_out`). As [`fn_00d44e60`] but calls
/// [`hkp_multi_sphere_agent_linear_cast`] on the same agent.
pub fn hkp_symmetric_agent_multi_sphere_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
    early_out: u32,
) {
    e.with_stack(0xd0, |e, frame| {
        let (copy, first, second) = flip_cast_records(e, frame.addr(), input, output, early_out);
        hkp_multi_sphere_agent_linear_cast(e, this, body_b, body_a, copy, first, second);
    });
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

// Translated from 00d44cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `first`, `second`, `third`, `fourth`): a create
/// function of a collision agent (code at `00d44d9d` stores its address in a collision-agent record).
/// Allocates 0x38 bytes (`allocate_agent_block`) and runs the constructor `00d44b00` (`thiscall`,
/// `RET 0x10`) on the block with the four words; answers what the constructor answers.
pub fn fn_00d44cc0(e: &mut Engine, first: u32, second: u32, third: u32, fourth: u32) -> u32 {
    let block = allocate_agent_block(e, 0x38);
    e.call(0x00d4_4b00, &args![block, first, second, third, fourth])
        .u32()
}

// Translated from 00d44d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `first`, `second`, `third`, `fourth`): the
/// create function of the flipped agent (code at `00d44d63` stores its address in a
/// collision-agent record). Allocates 0x38 bytes (`allocate_agent_block`), runs the constructor `00d44b00` on the
/// block with the first two words swapped (`second`, `first`, `third`, `fourth`), then stores the
/// vtable word 0x010c_f390 in the block's first word. Answers the block.
pub fn fn_00d44d00(e: &mut Engine, first: u32, second: u32, third: u32, fourth: u32) -> u32 {
    let block = allocate_agent_block(e, 0x38);
    e.call(0x00d4_4b00, &args![block, second, first, third, fourth]);
    e.mem.set_u32(block, 0x010c_f390);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d44cc0, fn_00d44cc0(u32, u32, u32, u32) -> u32),
        entry!(0x00d44d00, fn_00d44d00(u32, u32, u32, u32) -> u32),
        entry!(0x00d443a0, hkp_multi_sphere_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d443e0,
            hkp_multi_sphere_agent_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00d444f0,
            hkp_multi_sphere_agent_get_closest_points(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d445e0, fn_00d445e0(u32, u32, u32, u32)),
        entry!(
            0x00d446f0,
            hkp_multi_sphere_agent_linear_cast(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x00d447f0, fn_00d447f0(u32, u32, u32, u32, u32)),
        entry!(
            0x00d44900,
            hkp_multi_sphere_agent_get_penetrations(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d449f0, fn_00d449f0(u32, u32, u32, u32)),
        entry!(
            0x00d44b00,
            hkp_multi_sphere_agent_construct(Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00d44d50, fn_00d44d50(u32)),
        entry!(0x00d44de0, fn_00d44de0(u32, u32, u32, u32)),
        entry!(0x00d44e20, fn_00d44e20(u32, u32, u32, u32)),
        entry!(0x00d44e60, fn_00d44e60(u32, u32, u32, u32, u32)),
        entry!(
            0x00d44f50,
            hkp_symmetric_agent_linear_cast_multi_sphere_get_penetrations(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00d44f90,
            hkp_symmetric_agent_linear_cast_multi_sphere_get_closest_points(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d44fd0,
            hkp_symmetric_agent_linear_cast_multi_sphere_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00d450f0,
            hkp_multi_sphere_agent_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00d45160,
            hkp_symmetric_agent_multi_sphere_linear_cast(Ptr, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn engine_with_allocator(block: u32) -> (Engine, std::rc::Rc<std::cell::RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
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
        (e, sizes)
    }

    /// The argument lists of the logged calls to `address`.
    fn seen_calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00d44cc0_allocates_the_block_and_runs_the_constructor() {
        let (mut e, sizes) = engine_with_allocator(0x0800_0000);
        e.map(0x0800_0000, 0x100);
        e.register(0x00d4_4b00, |_, a| Ret {
            eax: a[0].wrapping_add(1),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d4_4cc0, &args![1u32, 2u32, 3u32, 4u32]).u32();
        assert_eq!(answer, 0x0800_0001);
        assert_eq!(*sizes.borrow(), vec![0x38]);
        assert_eq!(e.mem.u16(0x0800_0004), 0x38);
        assert_eq!(
            seen_calls(&e, 0x00d4_4b00),
            vec![vec![0x0800_0000, 1, 2, 3, 4]]
        );
    }

    #[test]
    fn fn_00d44d00_swaps_the_first_two_words_and_sets_the_vtable() {
        let (mut e, sizes) = engine_with_allocator(0x0800_0000);
        e.map(0x0800_0000, 0x100);
        e.register(0x00d4_4b00, |_, a| Ret {
            eax: a[0].wrapping_add(1),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d4_4d00, &args![1u32, 2u32, 3u32, 4u32]).u32();
        assert_eq!(answer, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x38]);
        assert_eq!(e.mem.u16(0x0800_0004), 0x38);
        assert_eq!(e.mem.u32(0x0800_0000), 0x010c_f390);
        assert_eq!(
            seen_calls(&e, 0x00d4_4b00),
            vec![vec![0x0800_0000, 2, 1, 3, 4]]
        );
    }

    type Log = Vec<(u32, Vec<u32>)>;

    const ROUTER: u32 = 0x0030_1000;
    const ALLOCATOR: u32 = 0x0030_2000;
    const ALLOCATOR_VTABLE: u32 = 0x0030_3000;
    const DISPATCHER: u32 = 0x0031_0000;
    const AGENT_TABLE: u32 = 0x0032_0000;
    const FAKE_DELETE: u32 = 0x7000_0008;
    const FAKE_FREE: u32 = 0x7000_0010;
    const FAKE_DESTRUCTOR: u32 = 0x7000_0100;
    const FAKE_PENETRATIONS: u32 = 0x7000_0103;
    const FAKE_CLOSEST: u32 = 0x7000_0104;
    const FAKE_CAST: u32 = 0x7000_0105;
    const FAKE_PROCESS: u32 = 0x7000_0106;
    const FAKE_CLEANUP: u32 = 0x7000_0107;
    const FAKE_CREATE: u32 = 0x7000_0200;
    const FAKE_STATIC_PENETRATIONS: u32 = 0x7000_0204;
    const FAKE_STATIC_CLOSEST: u32 = 0x7000_0208;
    const FAKE_STATIC_CAST: u32 = 0x7000_020c;
    const FAKE_RESERVE_DONE: u32 = 0x0033_0000;

    /// What a double saw of the locals when an agent function was called through it.
    #[derive(Debug, Clone, PartialEq)]
    struct Seen {
        words: Vec<u32>,
        radius: f32,
        key: u32,
        parent: u32,
        motion: u32,
        points: [Vec4; 3],
    }

    type Seens = Rc<RefCell<Vec<Seen>>>;

    /// A double that records the cd body (at word `cd_index`) it is given and, when
    /// `stop_at` names a word, sets the byte at +4 of the record that word points to.
    fn recorder(
        log: &Seens,
        cd_index: usize,
        stop_at: Option<usize>,
    ) -> impl FnMut(&mut Engine, &[u32]) -> Ret + 'static {
        let log = Rc::clone(log);
        move |e, a| {
            let cd = a[cd_index];
            let shape = e.mem.u32(cd);
            let motion = e.mem.u32(cd + CD_MOTION);
            log.borrow_mut().push(Seen {
                words: a.to_vec(),
                radius: e.mem.f32(shape + 0x10),
                key: e.mem.u32(cd + CD_SHAPE_KEY),
                parent: e.mem.u32(cd + CD_PARENT),
                motion,
                points: [
                    load_vec(e, motion + 0x30),
                    load_vec(e, motion + 0x40),
                    load_vec(e, motion + 0x50),
                ],
            });
            if let Some(word) = stop_at {
                e.mem.set_u8(a[word] + 4, 1);
            }
            Ret::default()
        }
    }

    struct World {
        e: Engine,
        body_a: u32,
        body_b: u32,
        input: u32,
        output: u32,
        agent: u32,
        agent_x: u32,
        agent_y: u32,
    }

    /// Body A is a multi sphere shape of two spheres, `(1, 2, 3)` radius 0.5 and `(4, 5, 6)`
    /// radius 0.75, whose motion state rotates `(x, y, z)` to `(-y, x, z)` and translates by
    /// `(10, 20, 30, 1)`, `(11, 21, 31, 1)` and `(12, 22, 32, 1)`. Body B's shape has type 3.
    /// The agent has two entries, `(index 1, agent X)` and `(index 0, agent Y)`.
    fn world() -> World {
        let mut e = Engine::new();
        e.map(ROUTER, 0x20);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.map(ALLOCATOR, 8);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, FAKE_DELETE, 0, FAKE_FREE]);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        e.register(FAKE_DELETE, |_, _| Ret::default());
        e.register(FAKE_FREE, |_, _| Ret::default());
        e.register(MOTION_STATE_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x60);
            e.mem.write(a[0], &bytes);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(SPHERE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + SPHERE_RADIUS, a[1]);
            e.mem.set_u32(a[0] + SPHERE_TYPE, 1);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.map(DISPATCHER, 0x1000);
        // Agent type 2 for (sphere type 1, type 3) in both tables.
        e.mem.set_u8(DISPATCHER + TABLE_AGENT + (1 << 5) + 3, 2);
        e.mem
            .set_u8(DISPATCHER + TABLE_PREDICTIVE + (1 << 5) + 3, 2);
        let record = DISPATCHER + FUNCTIONS + 2 * 0x14;
        e.mem.set_u32(record, FAKE_CREATE);
        e.mem.set_u32(record + 4, FAKE_STATIC_PENETRATIONS);
        e.mem.set_u32(record + 8, FAKE_STATIC_CLOSEST);
        e.mem.set_u32(record + 12, FAKE_STATIC_CAST);
        e.put_vtable(
            AGENT_TABLE,
            &[
                FAKE_DESTRUCTOR,
                0,
                0,
                FAKE_PENETRATIONS,
                FAKE_CLOSEST,
                FAKE_CAST,
                FAKE_PROCESS,
                FAKE_CLEANUP,
            ],
        );

        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(shape + 0x10, 2);
        store_vec(&mut e, shape + 0x20, [1.0, 2.0, 3.0, 0.5]);
        store_vec(&mut e, shape + 0x30, [4.0, 5.0, 6.0, 0.75]);
        let motion = e.mem.alloc(0xb0);
        store_vec(&mut e, motion, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, motion + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, motion + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, motion + 0x30, [10.0, 20.0, 30.0, 1.0]);
        store_vec(&mut e, motion + 0x40, [11.0, 21.0, 31.0, 1.0]);
        store_vec(&mut e, motion + 0x50, [12.0, 22.0, 32.0, 1.0]);
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a, shape);
        e.mem.set_u32(body_a + 8, motion);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_u32(shape_b + 0xc, 3);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b, shape_b);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, DISPATCHER);
        let output = e.mem.alloc(0x3100);
        let agent_x = e.mem.alloc(8);
        e.mem.set_u32(agent_x, AGENT_TABLE);
        let agent_y = e.mem.alloc(8);
        e.mem.set_u32(agent_y, AGENT_TABLE);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent, AGENT_TABLE);
        e.mem.set_u32(agent + AGENTS, agent + AGENTS_INLINE);
        e.mem.set_u32(agent + AGENTS + 4, 2);
        e.mem.set_u32(agent + AGENTS + 8, 0x8000_0004);
        e.mem.set_u32(agent + AGENTS_INLINE, 1);
        e.mem.set_u32(agent + AGENTS_INLINE + 4, agent_x);
        e.mem.set_u32(agent + AGENTS_INLINE + 8, 0);
        e.mem.set_u32(agent + AGENTS_INLINE + 12, agent_y);
        World {
            e,
            body_a,
            body_b,
            input,
            output,
            agent,
            agent_x,
            agent_y,
        }
    }

    /// The motion state vectors the world's sphere 1 `(4, 5, 6)` and sphere 0 `(1, 2, 3)`
    /// produce: the centre rotated is `(-5, 4, 6, 0)` and `(-2, 1, 3, 0)`.
    const SPHERE_1: [Vec4; 3] = [
        [5.0, 24.0, 36.0, 1.0],
        [6.0, 25.0, 37.0, 1.0],
        [7.0, 26.0, 38.0, 1.0],
    ];
    const SPHERE_0: [Vec4; 3] = [
        [8.0, 21.0, 33.0, 1.0],
        [9.0, 22.0, 34.0, 1.0],
        [10.0, 23.0, 35.0, 1.0],
    ];

    #[test]
    fn cleanup_cleans_each_agent_then_destroys_itself() {
        let mut w = world();
        w.e.call_log = Some(vec![]);
        for fake in [FAKE_CLEANUP, FAKE_DESTRUCTOR] {
            w.e.register(fake, |_, _| Ret::default());
        }
        w.e.call(0x00d4_43a0, &args![w.agent, 0x1234u32]);
        let log: Log = w.e.call_log.take().unwrap();
        let cleanups: Vec<_> = log.iter().filter(|c| c.0 == FAKE_CLEANUP).collect();
        assert_eq!(cleanups.len(), 2);
        assert_eq!(cleanups[0].1, args![w.agent_x, 0x1234u32].to_vec());
        assert_eq!(cleanups[1].1, args![w.agent_y, 0x1234u32].to_vec());
        assert_eq!(
            log.last().unwrap(),
            &(FAKE_DESTRUCTOR, args![w.agent, 1u32].to_vec())
        );
    }

    #[test]
    fn process_collision_places_each_sphere_and_calls_its_agent() {
        let mut w = world();
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_PROCESS, recorder(&seen, 1, None));
        w.e.call(
            0x00d4_43e0,
            &args![w.agent, w.body_a, w.body_b, w.input, w.output],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // Entry 0 is sphere index 1 with key 1; entry 1 is sphere 0 with key 0.
        assert_eq!(seen[0].words[0], w.agent_x);
        assert_eq!(seen[0].words[2..], [w.body_b, w.input, w.output]);
        assert_eq!(seen[0].points, SPHERE_1);
        assert_eq!((seen[0].radius, seen[0].key), (0.75, 1));
        assert_eq!(seen[0].parent, w.body_a);
        assert_eq!(seen[0].motion, seen[0].words[1] + MOTION);
        assert_eq!(seen[1].words[0], w.agent_y);
        assert_eq!(seen[1].points, SPHERE_0);
        assert_eq!((seen[1].radius, seen[1].key), (0.5, 0));
    }

    #[test]
    fn get_closest_points_sets_only_the_translation() {
        let mut w = world();
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_CLOSEST, recorder(&seen, 1, None));
        w.e.call(
            0x00d4_44f0,
            &args![w.agent, w.body_a, w.body_b, w.input, w.output],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].points[0], SPHERE_1[0]);
        assert_eq!(seen[0].points[1], [11.0, 21.0, 31.0, 1.0]);
        assert_eq!(seen[0].points[2], [12.0, 22.0, 32.0, 1.0]);
        assert_eq!(seen[1].points[0], SPHERE_0[0]);
        assert_eq!((seen[0].key, seen[1].key), (1, 0));
        assert_eq!(seen[1].words[0], w.agent_y);
    }

    #[test]
    fn static_closest_points_goes_through_the_dispatcher_table() {
        let mut w = world();
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_STATIC_CLOSEST, recorder(&seen, 0, None));
        w.e.call(0x00d4_45e0, &args![w.body_a, w.body_b, w.input, w.output]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // Spheres in order; the key is the sphere's index.
        assert_eq!(seen[0].points[0], SPHERE_0[0]);
        assert_eq!((seen[0].radius, seen[0].key), (0.5, 0));
        assert_eq!(seen[1].points[0], SPHERE_1[0]);
        assert_eq!((seen[1].radius, seen[1].key), (0.75, 1));
        assert_eq!(seen[0].words[1..], [w.body_b, w.input, w.output]);
        assert_eq!(seen[0].parent, w.body_a);
    }

    #[test]
    fn linear_cast_uses_the_first_entrys_agent_for_every_sphere() {
        let mut w = world();
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_CAST, recorder(&seen, 1, None));
        w.e.call(
            0x00d4_46f0,
            &args![w.agent, w.body_a, w.body_b, w.input, w.output, 0x55u32],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].words[0], w.agent_x);
        assert_eq!(seen[1].words[0], w.agent_x);
        // Sphere index and key count down from the entry count minus one.
        assert_eq!((seen[0].radius, seen[0].key), (0.75, 1));
        assert_eq!((seen[1].radius, seen[1].key), (0.5, 0));
        assert_eq!(seen[0].points[0], SPHERE_1[0]);
        assert_eq!(seen[1].points[0], SPHERE_0[0]);
        assert_eq!(seen[0].words[2..], [w.body_b, w.input, w.output, 0x55]);
    }

    #[test]
    fn static_linear_cast_passes_the_early_out_word() {
        let mut w = world();
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_STATIC_CAST, recorder(&seen, 0, None));
        w.e.call(
            0x00d4_47f0,
            &args![w.body_a, w.body_b, w.input, w.output, 0x66u32],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[1].words[1..], [w.body_b, w.input, w.output, 0x66]);
        assert_eq!(seen[1].points[0], SPHERE_1[0]);
    }

    #[test]
    fn get_penetrations_stops_when_the_collector_says_so() {
        let mut w = world();
        let collector = w.e.mem.alloc(8);
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_PENETRATIONS, recorder(&seen, 1, None));
        w.e.call(
            0x00d4_4900,
            &args![w.agent, w.body_a, w.body_b, w.input, collector],
        );
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(seen.borrow()[0].points[0], SPHERE_1[0]);
        assert_eq!(seen.borrow()[0].words[4], collector);
        let stopped: Seens = Rc::default();
        w.e.register_double(FAKE_PENETRATIONS, recorder(&stopped, 1, Some(4)));
        w.e.call(
            0x00d4_4900,
            &args![w.agent, w.body_a, w.body_b, w.input, collector],
        );
        assert_eq!(stopped.borrow().len(), 1);
    }

    #[test]
    fn static_get_penetrations_stops_when_the_collector_says_so() {
        let mut w = world();
        let collector = w.e.mem.alloc(8);
        let seen: Seens = Rc::default();
        w.e.register_double(FAKE_STATIC_PENETRATIONS, recorder(&seen, 0, Some(3)));
        w.e.call(0x00d4_49f0, &args![w.body_a, w.body_b, w.input, collector]);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0].words[1..], [w.body_b, w.input, collector]);
        assert_eq!(seen.borrow()[0].points[0], SPHERE_0[0]);
    }

    /// An uninitialised agent object over a fresh block of 0x40 bytes.
    fn construct(w: &mut World, input: u32, mgr: u32) -> (u32, Log) {
        let created = Rc::new(RefCell::new(0u32));
        let counter = Rc::clone(&created);
        w.e.register_double(FAKE_CREATE, move |_, _| {
            *counter.borrow_mut() += 1;
            Ret {
                eax: 0x7100_0000 + *counter.borrow(),
                ..Ret::default()
            }
        });
        let me = w.e.mem.alloc(0x40);
        w.e.call_log = Some(vec![]);
        let result =
            w.e.call(0x00d4_4b00, &args![me, w.body_a, w.body_b, input, mgr]);
        assert_eq!(result.u32(), me);
        let log = w.e.call_log.take().unwrap();
        (me, log)
    }

    #[test]
    fn construct_fills_the_inline_array_with_one_agent_per_sphere() {
        let mut w = world();
        let input = w.input;
        let (me, log) = construct(&mut w, input, 0x4321);
        let e = &w.e;
        assert_eq!(e.mem.u32(me), AGENT_VTABLE);
        assert_eq!(e.mem.u32(me + 8), 0x4321);
        assert_eq!(e.mem.u16(me + 6), 1);
        assert_eq!(e.mem.u32(me + AGENTS), me + AGENTS_INLINE);
        assert_eq!(e.mem.u32(me + AGENTS + 4), 2);
        assert_eq!(e.mem.u32(me + AGENTS + 8), 0x8000_0004);
        assert_eq!(e.mem.u32(me + AGENTS_INLINE), 0);
        assert_eq!(e.mem.u32(me + AGENTS_INLINE + 4), 0x7100_0001);
        assert_eq!(e.mem.u32(me + AGENTS_INLINE + 8), 1);
        assert_eq!(e.mem.u32(me + AGENTS_INLINE + 12), 0x7100_0002);
        // Two spheres fit the inline storage: nothing is reserved.
        assert!(log.iter().all(|c| c.0 != ARRAY_RESERVE));
        let creates: Vec<_> = log.iter().filter(|c| c.0 == FAKE_CREATE).collect();
        assert_eq!(creates.len(), 2);
        assert_eq!(creates[0].1[1..], [w.body_b, w.input, 0x4321]);
        assert_eq!(log.iter().filter(|c| c.0 == MOTION_STATE_COPY).count(), 1);
        assert_eq!(log.iter().filter(|c| c.0 == SPHERE_CONSTRUCT).count(), 1);
    }

    #[test]
    fn construct_reserves_room_for_more_than_four_spheres() {
        let mut w = world();
        let big = w.e.mem.alloc(0x20 + 6 * 16);
        for i in 0..6u32 {
            store_vec(&mut w.e, big + 0x20 + 16 * i, [i as f32, 0.0, 0.0, 1.0]);
        }
        w.e.mem.set_u32(big + 0x10, 6);
        w.e.mem.set_u32(w.body_a, big);
        w.e.map(FAKE_RESERVE_DONE, 0x100);
        w.e.register(ARRAY_RESERVE, |e, a| {
            // Stands in for the reserve: 8 entries of storage, as asked.
            e.mem.set_u32(a[1], FAKE_RESERVE_DONE);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let input = w.input;
        let (me, log) = construct(&mut w, input, 0);
        let reserves: Vec<_> = log.iter().filter(|c| c.0 == ARRAY_RESERVE).collect();
        assert_eq!(reserves.len(), 1);
        assert_eq!(
            reserves[0].1,
            args![ALLOCATOR, me + AGENTS, 8u32, 8u32].to_vec()
        );
        assert_eq!(w.e.mem.u32(me + AGENTS + 4), 6);
        assert_eq!(w.e.mem.u32(FAKE_RESERVE_DONE + 5 * 8), 5);
        assert_eq!(w.e.mem.u32(FAKE_RESERVE_DONE + 5 * 8 + 4), 0x7100_0006);
    }

    #[test]
    fn construct_uses_the_predictive_table_when_asked() {
        let mut w = world();
        // Agent type 3 only in the predictive table.
        w.e.mem
            .set_u8(DISPATCHER + TABLE_PREDICTIVE + (1 << 5) + 3, 3);
        w.e.mem
            .set_u32(DISPATCHER + FUNCTIONS + 3 * 0x14, 0x7000_0300);
        w.e.register(0x7000_0300, |_, _| Ret::default());
        w.e.mem.set_u32(w.input + 0x14, 1);
        let input = w.input;
        let (_, log) = construct(&mut w, input, 7);

        assert!(log.iter().any(|c| c.0 == 0x7000_0300));
    }

    #[test]
    fn registration_makes_two_records() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let log = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words: Vec<u32> = a.to_vec();
            for slot in 0..4 {
                words.push(e.mem.u32(a[1] + 4 * slot));
            }
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d4_4d50, &args![0x1234u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let first = &seen[0];
        assert_eq!((first[0], first[2], first[3]), (0x1234, u32::MAX, 0x15));
        assert_eq!(
            first[4..],
            [0x00d4_4d00, 0x00d4_4de0, 0x00d4_4e20, 0x00d4_4e60, 1, 1]
        );
        let second = &seen[1];
        assert_eq!((second[0], second[2], second[3]), (0x1234, 0x15, u32::MAX));
        assert_eq!(
            second[4..],
            [0x00d4_4cc0, 0x00d4_49f0, 0x00d4_45e0, 0x00d4_47f0, 0, 1]
        );
    }

    /// Records the three words of the record at word `word` and the call's words.
    fn record_flipped(
        log: &Rc<RefCell<Vec<(Vec<u32>, [u32; 3])>>>,
        word: usize,
    ) -> impl FnMut(&mut Engine, &[u32]) -> Ret + 'static {
        let log = Rc::clone(log);
        move |e, a| {
            let at = a[word];
            let record = [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)];
            log.borrow_mut().push((a.to_vec(), record));
            Ret::default()
        }
    }

    fn set_flip_constants(w: &mut World) {
        w.e.map(MAX_FLOAT & !0xfff, 0x1000);
        w.e.mem.set_f32(MAX_FLOAT, f32::from_bits(0x7f7f_ffee));
    }

    #[test]
    fn flipped_get_penetrations_swaps_the_bodies_and_wraps_the_collector() {
        let mut w = world();
        let collector = w.e.mem.alloc(8);
        let seen = Rc::default();
        // Body B goes first: it is the multi sphere body here.
        w.e.register_double(FAKE_STATIC_PENETRATIONS, record_flipped(&seen, 3));
        // With the bodies swapped, body A must be a plain body of shape type 3 and body B the
        // multi sphere body.
        let (plain, multi) = (w.body_b, w.body_a);
        w.e.call(0x00d4_4de0, &args![plain, multi, w.input, collector]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0[1..3], [plain, w.input]);
        assert_eq!(seen[0].1, [FLIPPED_PENETRATIONS_VTABLE, 0, collector]);
    }

    #[test]
    fn flipped_closest_points_wraps_the_output_with_the_maximum_float() {
        let mut w = world();
        set_flip_constants(&mut w);
        let seen = Rc::default();
        w.e.register_double(FAKE_STATIC_CLOSEST, record_flipped(&seen, 3));
        let (plain, multi) = (w.body_b, w.body_a);
        w.e.call(0x00d4_4e20, &args![plain, multi, w.input, w.output]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[1].1, [FLIPPED_CLOSEST_VTABLE, 0x7f7f_ffee, w.output]);
    }

    #[test]
    fn member_flipped_get_penetrations_and_closest_points() {
        let mut w = world();
        set_flip_constants(&mut w);
        let collector = w.e.mem.alloc(8);
        let (plain, multi) = (w.body_b, w.body_a);
        let seen = Rc::default();
        w.e.register_double(FAKE_PENETRATIONS, record_flipped(&seen, 4));
        w.e.call(
            0x00d4_4f50,
            &args![w.agent, plain, multi, w.input, collector],
        );
        {
            let seen = seen.borrow();
            assert_eq!(seen.len(), 2);
            assert_eq!(seen[0].0[2..4], [plain, w.input]);
            assert_eq!(seen[0].1, [FLIPPED_PENETRATIONS_VTABLE, 0, collector]);
        }
        let closest = Rc::default();
        w.e.register_double(FAKE_CLOSEST, record_flipped(&closest, 4));
        w.e.call(
            0x00d4_4f90,
            &args![w.agent, plain, multi, w.input, w.output],
        );
        let closest = closest.borrow();
        assert_eq!(closest.len(), 2);
        assert_eq!(
            closest[0].1,
            [FLIPPED_CLOSEST_VTABLE, 0x7f7f_ffee, w.output]
        );
    }

    /// A 0x70-byte linear cast input whose words are their own index and whose path (+0x50)
    /// is `(1, 2, 3, 4)`.
    fn cast_input(w: &mut World) -> u32 {
        let input = w.e.mem.alloc(0x70);
        for word in 0..0x1c {
            w.e.mem.set_u32(input + 4 * word, 0x100 + word);
        }
        store_vec(&mut w.e, input + 0x50, [1.0, 2.0, 3.0, 4.0]);
        w.e.mem.set_u32(input, DISPATCHER);
        input
    }

    type Cast = (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>);

    /// Records the call's words, the copied input and the two records (`None` when 0).
    fn record_cast(
        log: &Rc<RefCell<Vec<Cast>>>,
        first: usize,
    ) -> impl FnMut(&mut Engine, &[u32]) -> Ret + 'static {
        let log = Rc::clone(log);
        move |e, a| {
            let words = |at: u32, count: u32| -> Vec<u32> {
                if at == 0 {
                    return vec![];
                }
                (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
            };
            let copy = words(a[first], 0x1c);
            let one = words(a[first + 1], 9);
            let two = words(a[first + 2], 9);
            log.borrow_mut().push((a.to_vec(), copy, one, two));
            Ret::default()
        }
    }

    fn check_flipped_cast(seen: &[Cast], plain: (usize, u32), early_out: u32, output: u32) {
        assert_eq!(seen.len(), 2);
        let (words, copy, one, two) = &seen[0];
        assert_eq!(words[plain.0], plain.1);
        // The input is copied word for word except the path, which is negated.
        assert_eq!(copy[0], DISPATCHER);
        assert_eq!(copy[1], 0x101);
        assert_eq!(copy[0x1b], 0x100 + 0x1b);
        assert_eq!(f32::from_bits(copy[0x14]), -1.0);
        assert_eq!(f32::from_bits(copy[0x15]), -2.0);
        assert_eq!(f32::from_bits(copy[0x16]), -3.0);
        assert_eq!(f32::from_bits(copy[0x17]), 4.0);
        assert_eq!(one[0], FLIPPED_CAST_VTABLE);
        assert_eq!(one[1], 0x7f7f_ffee);
        assert_eq!(
            [
                f32::from_bits(one[4]),
                f32::from_bits(one[5]),
                f32::from_bits(one[6])
            ],
            [1.0, 2.0, 3.0]
        );
        assert_eq!(one[8], output);
        if early_out == 0 {
            assert!(two.is_empty());
            assert_eq!(words[words.len() - 1], 0);
        } else {
            assert_eq!(two[0], FLIPPED_CAST_VTABLE);
            assert_eq!(two[1], 0x7f7f_ffee);
            assert_eq!(f32::from_bits(two[7]), 4.0);
            assert_eq!(two[8], early_out);
        }
    }

    #[test]
    fn flipped_linear_cast_copies_the_input_with_a_negated_path() {
        let mut w = world();
        set_flip_constants(&mut w);
        set_mask(&mut w.e, PATH_SIGN_MASK);
        let input = cast_input(&mut w);
        let (plain, multi) = (w.body_b, w.body_a);
        let seen = Rc::default();
        w.e.register_double(FAKE_STATIC_CAST, record_cast(&seen, 2));
        w.e.call(0x00d4_4e60, &args![plain, multi, input, w.output, 0u32]);
        check_flipped_cast(&seen.borrow(), (1, plain), 0, w.output);
        let seen2 = Rc::default();
        w.e.register_double(FAKE_STATIC_CAST, record_cast(&seen2, 2));
        w.e.call(0x00d4_4e60, &args![plain, multi, input, w.output, 0x99u32]);
        check_flipped_cast(&seen2.borrow(), (1, plain), 0x99, w.output);
    }

    #[test]
    fn member_flipped_linear_cast_does_the_same_on_the_agent() {
        let mut w = world();
        set_flip_constants(&mut w);
        set_mask(&mut w.e, PATH_SIGN_MASK);
        let input = cast_input(&mut w);
        let (plain, multi) = (w.body_b, w.body_a);
        let seen = Rc::default();
        w.e.register_double(FAKE_CAST, record_cast(&seen, 3));
        w.e.call(
            0x00d4_5160,
            &args![w.agent, plain, multi, input, w.output, 0x99u32],
        );
        check_flipped_cast(&seen.borrow(), (2, plain), 0x99, w.output);
        assert_eq!(seen.borrow()[0].0[0], w.agent_x);
    }

    /// Maps the page of a sign mask and sets it to flip the sign of lanes 0 to 2.
    fn set_mask(e: &mut Engine, addr: u32) {
        e.map(addr & !0xfff, 0x1000);
        for (lane, bits) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(addr + 4 * lane as u32, bits);
        }
    }

    /// An agent double that appends one contact point `{position (1, 2, 3, 4), normal
    /// (0.5, 0, 0, 2)}` to the output and sets the `float` at +0x3030 to 9.
    fn appender(e: &mut Engine) {
        e.register(FAKE_PROCESS, |e, a| {
            let output = a[4];
            let at = e.mem.u32(output);
            store_vec(e, at, [1.0, 2.0, 3.0, 4.0]);
            store_vec(e, at + 0x10, [0.5, 0.0, 0.0, 2.0]);
            e.mem.set_u32(output, at + 0x30);
            e.mem.set_f32(output + 0x3030, 9.0);
            Ret::default()
        });
    }

    #[test]
    fn flipped_process_collision_flips_the_new_points_and_the_toi_state() {
        let mut w = world();
        set_mask(&mut w.e, SIGN_MASK);
        // Two entries, so two appended points; the output's write pointer starts at +0x30.
        let out = w.output;
        w.e.mem.set_u32(out, out + 0x30);
        w.e.mem.set_f32(out + 0x3030, 1.0);
        store_vec(&mut w.e, out + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        w.e.mem.set_u8(out + 0x3048, 1);
        w.e.mem.set_u8(out + 0x3049, 2);
        w.e.mem.set_u8(out + 0x304a, 0x12);
        for (i, half) in [0x1111u16, 0x2222, 0x3333, 0x4444].into_iter().enumerate() {
            w.e.mem.set_u16(out + 0x3040 + 2 * i as u32, half);
        }
        appender(&mut w.e);
        // Run with the bodies swapped: body B of the call is the multi sphere body.
        let (plain, multi) = (w.body_b, w.body_a);
        w.e.call(0x00d4_4fd0, &args![w.agent, plain, multi, w.input, out]);
        // The old write pointer was out + 0x30; two points were appended after it.
        for point in [out + 0x30, out + 0x60] {
            assert_eq!(load_vec(&w.e, point), [2.0, 2.0, 3.0, 8.0]);
            let normal = load_vec(&w.e, point + 0x10);
            assert_eq!(normal[0], -0.5);
            assert!(normal[1].is_sign_negative() && normal[1] == 0.0);
            assert!(normal[2].is_sign_negative() && normal[2] == 0.0);
            assert_eq!(normal[3], 2.0);
        }
        // The state at +0x3020 and the bytes and halfwords were flipped (9 != 1).
        assert_eq!(load_vec(&w.e, out + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(w.e.mem.u8(out + 0x304a), 0x21);
        assert_eq!((w.e.mem.u8(out + 0x3048), w.e.mem.u8(out + 0x3049)), (2, 1));
        // 2 + 1 - 1 = 2: halfwords 0 and 2 exchanged; the bytes AND to 0, so no second swap.
        assert_eq!(w.e.mem.u16(out + 0x3040), 0x3333);
        assert_eq!(w.e.mem.u16(out + 0x3044), 0x1111);
        assert_eq!(w.e.mem.u16(out + 0x3042), 0x2222);
    }

    #[test]
    fn flipped_process_collision_exchanges_two_more_halfwords_for_twos() {
        let mut w = world();
        set_mask(&mut w.e, SIGN_MASK);
        let out = w.output;
        w.e.mem.set_u32(out, out + 0x30);
        w.e.mem.set_f32(out + 0x3030, 1.0);
        w.e.mem.set_u8(out + 0x3048, 2);
        w.e.mem.set_u8(out + 0x3049, 2);
        for (i, half) in [0x1111u16, 0x2222, 0x3333, 0x4444].into_iter().enumerate() {
            w.e.mem.set_u16(out + 0x3040 + 2 * i as u32, half);
        }
        appender(&mut w.e);
        let (plain, multi) = (w.body_b, w.body_a);
        w.e.call(0x00d4_4fd0, &args![w.agent, plain, multi, w.input, out]);
        // 2 + 2 - 1 = 3: halfwords 0 and 3 exchanged, then 1 and 2.
        assert_eq!(w.e.mem.u16(out + 0x3040), 0x4444);
        assert_eq!(w.e.mem.u16(out + 0x3046), 0x1111);
        assert_eq!(w.e.mem.u16(out + 0x3042), 0x3333);
        assert_eq!(w.e.mem.u16(out + 0x3044), 0x2222);
    }

    #[test]
    fn flipped_process_collision_leaves_the_state_alone_when_the_float_is_unchanged() {
        let mut w = world();
        set_mask(&mut w.e, SIGN_MASK);
        let out = w.output;
        w.e.mem.set_u32(out, out + 0x30);
        w.e.mem.set_f32(out + 0x3030, 9.0);
        store_vec(&mut w.e, out + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        appender(&mut w.e);
        let (plain, multi) = (w.body_b, w.body_a);
        w.e.call(0x00d4_4fd0, &args![w.agent, plain, multi, w.input, out]);
        // The points are flipped, the state is not.
        assert_eq!(load_vec(&w.e, out + 0x30), [2.0, 2.0, 3.0, 8.0]);
        assert_eq!(load_vec(&w.e, out + 0x3020), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_owned_agents_and_optionally_itself() {
        let mut w = world();
        let me = w.e.mem.alloc(0x40);
        w.e.mem.set_u16(me + 4, 0x40);
        w.e.mem.set_u32(me + AGENTS, 0x00c0_0000);
        w.e.mem.set_u32(me + AGENTS + 4, 2);
        w.e.mem.set_u32(me + AGENTS + 8, 6);
        w.e.call_log = Some(vec![]);
        let result = w.e.call(0x00d4_50f0, &args![me, 1u32]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(result.u32(), me);
        let frees: Vec<_> = log.iter().filter(|c| c.0 == FAKE_FREE).collect();
        assert_eq!(
            frees[0].1,
            args![ALLOCATOR, 0x00c0_0000u32, 6u32 * 8].to_vec()
        );
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == FAKE_DELETE).collect();
        assert_eq!(deletes[0].1, args![ALLOCATOR, me, 0x40u32].to_vec());
        assert_eq!(w.e.mem.u32(me), BASE_VTABLE);
        assert_eq!(w.e.mem.u32(me + AGENTS), 0);
        assert_eq!(w.e.mem.u32(me + AGENTS + 4), 0);
        assert_eq!(w.e.mem.u32(me + AGENTS + 8), 0x8000_0000);
        // Inline storage is not owned: nothing is freed, and without the flag nothing is deleted.
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d4_50f0, &args![w.agent, 0u32]);
        let log: Log = w.e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FAKE_FREE && c.0 != FAKE_DELETE));
    }
}
