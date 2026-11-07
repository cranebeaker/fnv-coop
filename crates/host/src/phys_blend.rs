//! Smooth drawing of physics bodies at high frame rates (`NV_PHYS_BLEND`), a fix of the `fixes`
//! preset ([`crate::fixes`], docs/FIXES.md): **not 1:1 with FalloutNV.exe**, and in the `vanilla`
//! preset nothing here runs.
//!
//! **What the original draws.** Havok steps the world 16 ms at a time whatever the frame rate
//! (`bhkWorld::Update` `00c6ae70`, after the accumulator `00c66760`). Once a frame the scene
//! graph's update hands every free body's node the body's state as it is then
//! (`bhkCollisionObject::hktoNiTransform` `00c65a80`; ragdoll bones reach it through
//! `bhkBlendCollisionObject`), and the frame is drawn from the nodes. That update comes before
//! the frame's world step, so the node holds the end of the step before the last: measured, 11 ms
//! behind the game's clock on average, between 0 and 23 (144 frames a second). Nothing blends
//! it: on a
//! frame with no step a dropped item, a ragdoll or debris is drawn where it was, on a frame with
//! one it jumps 16 ms of motion. At 144 frames a second 56% of the frames show a moving body
//! frozen, at 60 one frame in 24 moves it double (measure/hf-motion, 2026-11-06). Havok keeps,
//! per body, the step's swept transform and has the formula to draw between its ends
//! (`hkSweptTransform::approxTransformAt` `00cf22d0`); in this exe only the visual debugger
//! calls it.
//!
//! **What this does.** The simulation is not touched: same 16 ms steps, same nodes for every
//! piece of game logic. Only around the frame's drawing (`008706b0`, which picks the frame's
//! draw routine and runs culling, shadows and the passes) each moving body's node gets the body
//! as it was at the game's time minus one step (T - 16 ms, with T the world's current time plus
//! the accumulator's carried remainder `012677b0`): always between two steps Havok has run, so
//! it is interpolation, never extrapolation. The blend is `approxTransformAt`'s: the centre of
//! mass moved linearly, the rotation quaternion moved linearly and normalised, the position
//! rebuilt from the centre of mass. The nodes below the body's node (those without a collision
//! object of their own) are refreshed with the game's own world update (`00a68bf0`) and their
//! bounds moved with them; after the drawing every world transform and bound written is put back
//! as it was, unless something else wrote it in between (then that write stays). Bodies are drawn
//! one step (16 ms) behind the game's clock, about 5 ms later on average than the original's.
//!
//! **Which bodies.** The ones `hktoNiTransform` set in this frame (`00c65a80` is registered as
//! itself plus a note of the node, the body and the state it gave the node), whose node still holds that
//! transform at the drawing and whose body ran the world's last step (a sleeping body is drawn
//! as it is). Per body a small table keeps the last steps' ends (time, centre, rotation); a step
//! Havok cut short at a time of impact ends there and holds to the step's end, as the body does
//! in the simulation. It starts again (the body drawn from its last step's start, never before it) when
//! a body has just woken, when the last step's start isn't the end kept (a teleport, a
//! `SetPosition`, a new cell or world), or when the kept end changed without a step.
//!
//! What it doesn't do: the bounds of the nodes above a body (a cell's merged bound) keep the
//! original's, at most one step of motion apart; a node below a body with a collision object
//! that isn't blended (a keyframed attachment), or whose class has its own world update (a
//! world-space particle system, a fixed-rotation `BSValueNode`), stays where the original puts
//! it with everything below it; keyframed bodies (motion type 4) are drawn as the original does.

use crate::fixes::Fix;
use engine::prelude::*;
use engine::units::bshavok::bhkcollisionobject::bhk_collision_object_hk_to_ni_transform;
use engine::units::fallout_misc::main_p2::fn_008706b0;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::Mutex;

/// `bhkCollisionObject::hktoNiTransform`: the free body's state to its node.
pub const HK_TO_NI_TRANSFORM: u32 = 0x00c6_5a80;
/// The frame's drawing: picks the draw routine (`00870a00`, `00870bd0` or `008707c0`).
pub const DRAW_FRAME: u32 = 0x0087_06b0;

/// The row in [`crate::fixes::FIXES`].
pub const FIX: Fix = Fix {
    name: "smooth physics drawing",
    env: "NV_PHYS_BLEND",
    addresses: &[HK_TO_NI_TRANSFORM, DRAW_FRAME],
    summary: "free bodies drawn between Havok's 16 ms steps, one step behind the clock",
    install,
};

/// The collision object: its node (`m_pkSceneObject`) and its body (`spWorldObject`).
const COLLISION_NODE: u32 = 0x08;
const COLLISION_BODY: u32 = 0x10;
/// The `bhkRigidBody`'s Havok body (`hkpRigidBody`).
const BODY_HAVOK: u32 = 0x08;
/// The `hkpRigidBody`: its world, its transform's translation, and its swept transform
/// (`hkpMotion` at `+0xe0`, the sweep at the motion's `+0x50`: `00c86490`): the centre of mass
/// at the step's start (`w`: the start time) and end (`w`: one over the step), the rotation at
/// the start and end, and the centre of mass in the body's own space.
const HAVOK_WORLD: u32 = 0x08;
const MOTION_TYPE: u32 = 0xe8;
const HAVOK_TRANSLATION: u32 = 0x120;
const SWEEP_CENTRE_START: u32 = 0x130;
const SWEEP_CENTRE_END: u32 = 0x140;
const SWEEP_ROTATION_START: u32 = 0x150;
const SWEEP_ROTATION_END: u32 = 0x160;
const SWEEP_CENTRE_LOCAL: u32 = 0x170;
/// The `hkpWorld`: its simulation, and the last step's end time (`hkpSimulation::integrate`
/// `00cf9340` writes the step's start and end at `+0x1d0`; the start also moves to the times of
/// impact the continuous simulation works through, so only the end is read, and the step taken
/// is the world update's own: `fMaxTime` times the time multiplier).
const WORLD_SIMULATION: u32 = 0x08;
const WORLD_STEP_END: u32 = 0x1d4;
/// The `hkpSimulation`'s current time (`hkpWorld::getCurrentTime` `00c91020`): the frame's
/// marker once the world's update has run.
const SIMULATION_TIME: u32 = 0x14;
/// The accumulator's carried remainder, `fMaxTime` and the global time multiplier: the step the
/// world update takes is their product (`00c6af33`).
const CARRIED_TIME: u32 = 0x0126_77b0;
const MAX_TIME: u32 = 0x0126_7b38;
const TIME_MULTIPLIER: u32 = 0x011a_c3a0;
/// Game units per Havok unit (`004587d0`, what `bhkRigidBody::GetPosition` multiplies by).
const HAVOK_TO_GAME: u32 = 0x011c_3d18;
/// `NiAVObject`: the parent, the collision object, the world bound (a 16-byte block: centre and
/// radius) and the world transform (rotation, translation, scale: 13 words).
const NODE_COLLISION: u32 = 0x1c;
const NODE_BOUND: u32 = 0x20;
const NODE_WORLD: u32 = 0x68;
/// `NiNode`'s children (`NiTArray`: the element block and the count) and the virtual slot that
/// answers the object for an `NiNode` and 0 otherwise.
const NODE_CHILDREN: u32 = 0xa0;
const NODE_CHILD_COUNT: u32 = 0xa6;
const AS_NODE_SLOT: u32 = 0x0c;
/// The object's own world update without a collision object (`00a68bf0`): the parent's world
/// transform times its local one.
const NODE_WORLD_UPDATE: u32 = 0x00a6_8bf0;
/// `NiAVObject::UpdateWorldData` (`00a68c60`: the collision object's, else `00a68bf0`) and its
/// vtable slot; a class with another there does more than the parent's transform times its own.
const AV_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const UPDATE_WORLD_DATA_SLOT: u32 = 0xb8;

/// Whether the object's world update is `NiAVObject`'s own.
fn plain_world_update(e: &Engine, object: u32) -> bool {
    let vtable = e.mem.u32(object);
    vtable != 0 && e.mem.u32(vtable + UPDATE_WORLD_DATA_SLOT) == AV_UPDATE_WORLD_DATA
}
/// Kept samples per body: enough to reach two steps back (where T - 16 ms always is) when each
/// step ended early and holds.
const KEPT_SAMPLES: usize = 6;
/// Drawings after which a body nothing refreshed is forgotten.
const FORGET_AFTER: u64 = 600;

/// Registers the note at `00c65a80` and the blended drawing at `008706b0`.
pub fn install(e: &mut Engine) {
    let log = std::env::var_os("NV_PHYS_BLEND_LOG").and_then(|path| {
        std::fs::File::create(path)
            .ok()
            .map(std::io::BufWriter::new)
    });
    states().insert(
        key(e),
        State {
            log,
            ..State::default()
        },
    );
    e.register(HK_TO_NI_TRANSFORM, hk_to_ni_entry);
    e.register(DRAW_FRAME, draw_frame_entry);
}

fn hk_to_ni_entry(e: &mut Engine, a: &[u32]) -> Ret {
    bhk_collision_object_hk_to_ni_transform(e, Ptr::new(a[0]));
    note(e, a[0]);
    Ret::default()
}

fn draw_frame_entry(e: &mut Engine, a: &[u32]) -> Ret {
    draw_blended(e, |e| {
        fn_008706b0(e, Ptr::new(a[0]), a[1], a[2] as u8, a[3] as u8)
    });
    Ret::default()
}

/// Plans the blend (reads only), then writes it and draws under one guard: whatever was written
/// goes back after the drawing, and also when the writing or the drawing unwinds (a stop, or a
/// C++ exception the game catches further out), which then goes on unwinding.
fn draw_blended(e: &mut Engine, draw: impl FnOnce(&mut Engine)) {
    let moves = plan_blend(e);
    let mut written = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        write_moves(e, &moves, &mut written);
        draw(e);
    }));
    put_back(e, &written);
    if let Err(unwinding) = result {
        std::panic::resume_unwind(unwinding);
    }
}

/// What `hktoNiTransform` gave a node.
#[derive(Clone, Debug)]
struct Note {
    node: u32,
    body: u32,
    havok: u32,
    /// The vtables of the collision object, the node and the body: a cheap check against memory
    /// used again by another class (not against the same class: a note counts only in the frame
    /// it was taken, [`plan_blend`]).
    vtables: [u32; 3],
    /// The node's world rotation and translation after it (12 words).
    world: [u32; 12],
    /// The Havok body's state it came from: rotation (x, y, z, w) and translation.
    rotation: [f32; 4],
    translation: [f32; 3],
    /// The drawing count when it was taken.
    taken: u64,
}

/// One step end of a body: Havok time, centre of mass, rotation, and one over the time since the
/// sample before it (the step's own `1 / dt` when the two are a step's ends).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Sample {
    time: f32,
    centre: [f32; 3],
    rotation: [f32; 4],
    inverse_from_previous: f32,
}

#[derive(Default, Debug)]
struct Body {
    samples: Vec<Sample>,
    used: u64,
}

#[derive(Default)]
struct State {
    notes: HashMap<u32, Note>,
    bodies: HashMap<u32, Body>,
    drawings: u64,
    log: Option<std::io::BufWriter<std::fs::File>>,
}

/// The run's state, by engine (a run has one; tests have one each).
fn states() -> std::sync::MutexGuard<'static, HashMap<usize, State>> {
    static STATES: std::sync::OnceLock<Mutex<HashMap<usize, State>>> = std::sync::OnceLock::new();
    STATES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn key(e: &Engine) -> usize {
    e as *const Engine as usize
}

fn words<const N: usize>(e: &Engine, at: u32) -> [u32; N] {
    std::array::from_fn(|i| e.mem.u32(at + 4 * i as u32))
}

fn floats<const N: usize>(e: &Engine, at: u32) -> [f32; N] {
    std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32))
}

fn set_words(e: &mut Engine, at: u32, values: &[u32]) {
    for (i, value) in values.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *value);
    }
}

fn vtables_of(e: &Engine, collision: u32, node: u32, body: u32) -> [u32; 3] {
    [collision, node, body].map(|object| e.mem.u32(object))
}

/// After `hktoNiTransform`: notes the node, the body and the state the node was given.
fn note(e: &mut Engine, collision: u32) {
    let node = e.mem.u32(collision + COLLISION_NODE);
    let body = e.mem.u32(collision + COLLISION_BODY);
    if node == 0 || body == 0 {
        return;
    }
    let havok = e.mem.u32(body + BODY_HAVOK);
    // `hktoNiTransform` itself gives the node nothing from a body of motion type 5 in a world;
    // a keyframed body (motion type 4: animation drives it, a ragdoll at full animation blend)
    // is left as the original draws it.
    let motion_type = if havok != 0 {
        e.mem.u8(havok + MOTION_TYPE)
    } else {
        0
    };
    if havok == 0 || motion_type == 4 || (motion_type == 5 && e.mem.u32(havok + HAVOK_WORLD) != 0) {
        return;
    }
    let note = Note {
        node,
        body,
        havok,
        vtables: vtables_of(e, collision, node, body),
        world: words(e, node + NODE_WORLD),
        rotation: floats(e, havok + SWEEP_ROTATION_END),
        translation: floats(e, havok + HAVOK_TRANSLATION),
        taken: 0,
    };
    let mut states = states();
    if let Some(state) = states.get_mut(&key(e)) {
        let taken = state.drawings;
        state.notes.insert(collision, Note { taken, ..note });
    }
}

/// The last step as the body holds it.
#[derive(Clone, Copy, Debug)]
struct Sweep {
    start: Sample,
    end: Sample,
    centre_local: [f32; 3],
}

/// The sweep as the body holds it: the end's time is the start's plus one over `inverse` (the
/// `w` of the end centre).
fn sweep(e: &Engine, havok: u32) -> Sweep {
    let start: [f32; 4] = floats(e, havok + SWEEP_CENTRE_START);
    let end: [f32; 4] = floats(e, havok + SWEEP_CENTRE_END);
    Sweep {
        start: Sample {
            time: start[3],
            centre: [start[0], start[1], start[2]],
            rotation: floats(e, havok + SWEEP_ROTATION_START),
            inverse_from_previous: 0.0,
        },
        end: Sample {
            time: start[3] + 1.0 / end[3],
            centre: [end[0], end[1], end[2]],
            rotation: floats(e, havok + SWEEP_ROTATION_END),
            inverse_from_previous: end[3],
        },
        centre_local: floats(e, havok + SWEEP_CENTRE_LOCAL),
    }
}

fn same_state(a: &Sample, b: &Sample) -> bool {
    a.centre.map(f32::to_bits) == b.centre.map(f32::to_bits)
        && a.rotation.map(f32::to_bits) == b.rotation.map(f32::to_bits)
}

/// The samples the world's last step (start and end time) gives a body: its sweep's start and
/// end, and, when the sweep ends before the step does, the end held to the step's end (Havok
/// steps a body that hit something back to the time of impact, and it stays there for the rest
/// of the step). `None` when the sweep isn't inside the step: the body didn't run it (asleep).
fn step_samples(sweep: &Sweep, world_step: (f32, f32)) -> Option<Vec<Sample>> {
    let (step_start, step_end) = world_step;
    let step = step_end - step_start;
    let margin = 0.01 * step;
    let inverse = sweep.end.inverse_from_previous;
    let ran = step > 0.0
        && inverse > 0.0
        && inverse.is_finite()
        && sweep.start.time >= step_start - margin
        && sweep.end.time > sweep.start.time
        && sweep.end.time <= step_end + margin;
    if !ran {
        return None;
    }
    let mut samples = vec![sweep.start];
    if sweep.end.time >= step_end - margin {
        // The step's own end time, so the next step's start matches it exactly.
        samples.push(Sample {
            time: step_end,
            ..sweep.end
        });
    } else {
        samples.push(sweep.end);
        samples.push(Sample {
            time: step_end,
            inverse_from_previous: 1.0 / (step_end - sweep.end.time),
            ..sweep.end
        });
    }
    Some(samples)
}

/// Distance (centre of mass, Havok units) and turn (radians) between two samples.
fn change(a: &Sample, b: &Sample) -> (f32, f32) {
    let distance = (0..3)
        .map(|k| (b.centre[k] - a.centre[k]).powi(2))
        .sum::<f32>()
        .sqrt();
    let dot: f32 = (0..4).map(|k| a.rotation[k] * b.rotation[k]).sum();
    (distance, 2.0 * dot.abs().min(1.0).acos())
}

/// Whether the steps not seen between the last sample kept and the new step's start can be the
/// body's own motion: the distance and the turn across them are at most twice what the body's
/// fastest neighbouring step would cover in that time, plus a little (0.05 Havok units, 0.1
/// radians).
fn bridge_is_motion(kept: &[Sample], last: &Sample, start: &Sample, new: &[Sample]) -> bool {
    let rate = |a: &Sample, b: &Sample| {
        let (distance, turn) = change(a, b);
        let time = (b.time - a.time).max(f32::MIN_POSITIVE);
        (distance / time, turn / time)
    };
    let mut rates = vec![rate(&new[0], &new[1])];
    if let [.., before, _] = kept {
        rates.push(rate(before, last));
    }
    let speed = rates.iter().map(|r| r.0).fold(0.0, f32::max);
    let spin = rates.iter().map(|r| r.1).fold(0.0, f32::max);
    let gap = start.time - last.time;
    let (distance, turn) = change(last, start);
    distance <= 2.0 * speed * gap + 0.05 && turn <= 2.0 * spin * gap + 0.1
}

/// Takes the body's last step into its table (`step_samples`). Answers whether the body ran the
/// world's last step; a body that didn't has no table.
fn take_step(body: &mut Body, sweep: &Sweep, world_step: (f32, f32)) -> bool {
    let Some(new) = step_samples(sweep, world_step) else {
        body.samples.clear();
        return false;
    };
    let step = world_step.1 - world_step.0;
    let (start, newest) = (new[0], new[new.len() - 1]);
    let restart = match body.samples.last().copied() {
        None => true,
        // The same step as before: unchanged, or moved by something else (`SetPosition`).
        Some(last) if last.time == newest.time => !same_state(&last, &newest),
        // The step after the last one kept: it starts where that one ended, or the body was
        // put somewhere else.
        Some(last) if last.time == start.time => {
            if same_state(&last, &start) {
                body.samples.extend_from_slice(&new[1..]);
                false
            } else {
                true
            }
        }
        // Steps not seen in between (a frame with two or three): their start joins the table,
        // unless the body got there faster than it moves on either side (a teleport or a
        // `SetPosition` in that frame), which starts again.
        Some(last)
            if last.time < start.time
                && start.time - last.time <= 3.5 * step
                && bridge_is_motion(&body.samples, &last, &start, &new) =>
        {
            let gap = start.time - last.time;
            body.samples.push(Sample {
                inverse_from_previous: 1.0 / gap,
                ..start
            });
            body.samples.extend_from_slice(&new[1..]);
            false
        }
        // Time went back or jumped (a new world, a long pause): start again.
        Some(_) => true,
    };
    if restart {
        body.samples = new;
    }
    let excess = body.samples.len().saturating_sub(KEPT_SAMPLES);
    body.samples.drain(..excess);
    true
}

/// How a body is drawn at the time `at`.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Drawn {
    /// Between two kept step ends: the blended rotation and position (Havok units).
    Between([f32; 4], [f32; 3]),
    /// Before the first kept: that one.
    First([f32; 4], [f32; 3]),
    /// At or after the last: as the original draws it.
    AsIs,
}

/// The body at the time `at`, from its table (`approxTransformAt`'s formula between two samples).
fn drawn_at(samples: &[Sample], at: f32, centre_local: [f32; 3]) -> Drawn {
    let Some(last) = samples.last() else {
        return Drawn::AsIs;
    };
    if at >= last.time {
        return Drawn::AsIs;
    }
    let first = samples[0];
    if at < first.time {
        let (rotation, position) = place(first.centre, first.rotation, centre_local);
        return Drawn::First(rotation, position);
    }
    let i = samples
        .windows(2)
        .position(|pair| pair[0].time <= at && at < pair[1].time)
        .unwrap_or(samples.len() - 2);
    let (a, b) = (samples[i], samples[i + 1]);
    let t = (at - a.time) * b.inverse_from_previous;
    let centre: [f32; 3] = std::array::from_fn(|k| (1.0 - t) * a.centre[k] + b.centre[k] * t);
    // Consecutive Havok rotations stay on one side; a pair across steps not seen is checked.
    let dot: f32 = (0..4).map(|k| a.rotation[k] * b.rotation[k]).sum();
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    let rotation: [f32; 4] =
        std::array::from_fn(|k| (1.0 - t) * a.rotation[k] + sign * b.rotation[k] * t);
    let (rotation, position) = place(centre, normalised(rotation), centre_local);
    Drawn::Between(rotation, position)
}

fn normalised(q: [f32; 4]) -> [f32; 4] {
    let length_squared: f32 = q.iter().map(|v| v * v).sum();
    if length_squared == 0.0 {
        return q;
    }
    let inverse = 1.0 / length_squared.sqrt();
    q.map(|v| v * inverse)
}

/// The body's position from its centre of mass: centre minus the rotated local centre.
fn place(centre: [f32; 3], rotation: [f32; 4], centre_local: [f32; 3]) -> ([f32; 4], [f32; 3]) {
    let r = rotation_matrix(rotation);
    let rotated = apply(&r, centre_local);
    (rotation, std::array::from_fn(|k| centre[k] - rotated[k]))
}

/// The rotation matrix (row-major, as `NiMatrix3` holds it) of a Havok quaternion (x, y, z, w).
fn rotation_matrix(q: [f32; 4]) -> [f32; 9] {
    let [x, y, z, w] = q;
    [
        1.0 - 2.0 * (y * y + z * z),
        2.0 * (x * y - w * z),
        2.0 * (x * z + w * y),
        2.0 * (x * y + w * z),
        1.0 - 2.0 * (x * x + z * z),
        2.0 * (y * z - w * x),
        2.0 * (x * z - w * y),
        2.0 * (y * z + w * x),
        1.0 - 2.0 * (x * x + y * y),
    ]
}

fn apply(m: &[f32; 9], v: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|r| m[3 * r] * v[0] + m[3 * r + 1] * v[1] + m[3 * r + 2] * v[2])
}

fn multiply(a: &[f32; 9], b: &[f32; 9]) -> [f32; 9] {
    std::array::from_fn(|i| {
        let (r, c) = (i / 3, i % 3);
        (0..3).map(|k| a[3 * r + k] * b[3 * k + c]).sum()
    })
}

fn transposed(m: &[f32; 9]) -> [f32; 9] {
    std::array::from_fn(|i| m[3 * (i % 3) + i / 3])
}

/// The node's world rotation and translation for the body drawn at `rotation`, `position` (Havok
/// units), from what the note says the node holds for the body's noted state: the same rigid
/// move, so a node set through a local offset (`bhkRigidBodyT`) moves with its body.
fn moved_world(note: &Note, rotation: [f32; 4], position: [f32; 3], scale: f32) -> [u32; 12] {
    let world: [f32; 12] = note.world.map(f32::from_bits);
    let node_rotation: [f32; 9] = std::array::from_fn(|i| world[i]);
    let node_translation = [world[9], world[10], world[11]];
    let relative = multiply(
        &rotation_matrix(rotation),
        &transposed(&rotation_matrix(note.rotation)),
    );
    let new_rotation = multiply(&relative, &node_rotation);
    let offset: [f32; 3] =
        std::array::from_fn(|k| node_translation[k] - scale * note.translation[k]);
    let turned = apply(&relative, offset);
    let new_translation: [f32; 3] = std::array::from_fn(|k| turned[k] + scale * position[k]);
    let mut out = [0u32; 12];
    for i in 0..9 {
        out[i] = new_rotation[i].to_bits();
    }
    for k in 0..3 {
        out[9 + k] = new_translation[k].to_bits();
    }
    out
}

/// A node written for the drawing: its world transform and bound before and after. `world_after`
/// is `None` while the game's world update runs on the node (if that unwinds, the node is put
/// back whatever it holds).
#[derive(Clone, Debug, PartialEq)]
pub struct Written {
    node: u32,
    world_before: [u32; 13],
    world_after: Option<[u32; 13]>,
    bound: u32,
    bound_before: [u32; 4],
    bound_after: [u32; 4],
}

/// [`plan_blend`] and [`write_moves`] without the guard (tests).
pub fn blend_for_drawing(e: &mut Engine) -> Vec<Written> {
    let moves = plan_blend(e);
    let mut written = Vec::new();
    write_moves(e, &moves, &mut written);
    written
}

/// Each moving body's node and the world rotation and translation to draw it with; reads only.
/// A note counts only for the drawing right after it: `hktoNiTransform` ran on that collision
/// object in this frame's scene update, so it was a live object of the scene in this frame. (An
/// older note could name memory freed and used again by an object of the same class, which the
/// pointer and vtable checks below can't tell.)
fn plan_blend(e: &mut Engine) -> Vec<(u32, [u32; 12])> {
    let mut states = states();
    let Some(state) = states.get_mut(&key(e)) else {
        return Vec::new();
    };
    state.drawings += 1;
    let drawings = state.drawings;
    let carried = e.mem.f32(CARRIED_TIME);
    let step = e.mem.f32(MAX_TIME) * e.mem.f32(TIME_MULTIPLIER);
    let scale = e.mem.f32(HAVOK_TO_GAME);
    let mut collisions: Vec<u32> = state.notes.keys().copied().collect();
    collisions.sort_unstable();
    let mut moves: Vec<(u32, [u32; 12])> = Vec::new();
    let mut lines = Vec::new();
    for collision in collisions {
        let note = state.notes[&collision].clone();
        let valid = e.mem.u32(collision + COLLISION_NODE) == note.node
            && e.mem.u32(collision + COLLISION_BODY) == note.body
            && e.mem.u32(note.body + BODY_HAVOK) == note.havok
            && vtables_of(e, collision, note.node, note.body) == note.vtables
            && words::<12>(e, note.node + NODE_WORLD) == note.world;
        if !valid || note.taken + 1 != drawings {
            state.notes.remove(&collision);
            continue;
        }
        let world = e.mem.u32(note.havok + HAVOK_WORLD);
        let simulation = if world != 0 {
            e.mem.u32(world + WORLD_SIMULATION)
        } else {
            0
        };
        if simulation == 0 {
            continue;
        }
        let now = e.mem.f32(simulation + SIMULATION_TIME);
        let at = ((f64::from(now) + f64::from(carried)) - f64::from(step)) as f32;
        let step_end = e.mem.f32(world + WORLD_STEP_END);
        let step_start = step_end - step;
        let sweep = sweep(e, note.havok);
        let body = state.bodies.entry(note.havok).or_default();
        body.used = drawings;
        let ran = take_step(body, &sweep, (step_start, step_end));
        let drawn = if ran {
            drawn_at(&body.samples, at, sweep.centre_local)
        } else {
            Drawn::AsIs
        };
        let (kind, after) = match drawn {
            Drawn::Between(rotation, position) => {
                ('b', Some(moved_world(&note, rotation, position, scale)))
            }
            Drawn::First(rotation, position) => {
                ('f', Some(moved_world(&note, rotation, position, scale)))
            }
            Drawn::AsIs => (if ran { 'n' } else { 's' }, None),
        };
        if state.log.is_some() {
            let shown = after.unwrap_or(note.world);
            lines.push(format!(
                "B\t{collision:08x}\t{kind}\t{at:.6}\t{step_end:.6}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}",
                f32::from_bits(note.world[9]),
                f32::from_bits(note.world[10]),
                f32::from_bits(note.world[11]),
                f32::from_bits(shown[9]),
                f32::from_bits(shown[10]),
                f32::from_bits(shown[11]),
            ));
            if let Some(newest) = body.samples.last().filter(|_| ran) {
                // The body's path for the log's reader: the newest step end, where the node
                // would be with the body there.
                let (rotation, position) =
                    place(newest.centre, newest.rotation, sweep.centre_local);
                let there = moved_world(&note, rotation, position, scale);
                lines.push(format!(
                    "K\t{collision:08x}\t{:.6}\t{:.4}\t{:.4}\t{:.4}",
                    newest.time,
                    f32::from_bits(there[9]),
                    f32::from_bits(there[10]),
                    f32::from_bits(there[11]),
                ));
            }
        }
        if let Some(after) = after {
            moves.push((note.node, after));
        }
    }
    state
        .bodies
        .retain(|_, body| drawings <= body.used + FORGET_AFTER);
    if let Some(log) = state.log.as_mut() {
        // The wall clock, for the log only (the game's own clock isn't read here).
        let wall = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_micros());
        let _ = writeln!(
            log,
            "R\t{drawings}\t{wall}\t{carried}\t{step}\t{}",
            moves.len()
        );
        for line in &lines {
            let _ = writeln!(log, "{line}");
        }
        let _ = log.flush();
    }
    moves
}

/// Writes the bodies' nodes, then refreshes the nodes below each, recording into `written` before
/// any of the game's code runs on a node, so [`put_back`] can undo all of it.
fn write_moves(e: &mut Engine, moves: &[(u32, [u32; 12])], written: &mut Vec<Written>) {
    let mut seen = HashSet::new();
    for (node, after) in moves {
        if !seen.insert(*node) {
            continue;
        }
        let before: [u32; 13] = words(e, node + NODE_WORLD);
        set_words(e, node + NODE_WORLD, after);
        written.push(moved(e, *node, before));
    }
    for (node, _) in moves {
        refresh_below(e, *node, written, &mut seen);
    }
}

/// The record of a node whose world transform was `before` and is now what it holds; its bound
/// (when it has one) moved with it.
fn moved(e: &mut Engine, node: u32, before: [u32; 13]) -> Written {
    let after: [u32; 13] = words(e, node + NODE_WORLD);
    let bound = e.mem.u32(node + NODE_BOUND);
    let (bound_before, bound_after) = if bound != 0 {
        let old: [u32; 4] = words(e, bound);
        let centre = [0, 1, 2].map(|k| f32::from_bits(old[k]));
        let from = before.map(f32::from_bits);
        let to = after.map(f32::from_bits);
        let from_rotation: [f32; 9] = std::array::from_fn(|i| from[i]);
        let to_rotation: [f32; 9] = std::array::from_fn(|i| to[i]);
        let relative = multiply(&to_rotation, &transposed(&from_rotation));
        let offset: [f32; 3] = std::array::from_fn(|k| centre[k] - from[9 + k]);
        let turned = apply(&relative, offset);
        let mut new = old;
        for k in 0..3 {
            new[k] = (turned[k] + to[9 + k]).to_bits();
        }
        set_words(e, bound, &new);
        (old, new)
    } else {
        ([0; 4], [0; 4])
    };
    Written {
        node,
        world_before: before,
        world_after: Some(after),
        bound,
        bound_before,
        bound_after,
    }
}

/// The game's world update (`00a68bf0`) on every node below `node` that has no collision object
/// of its own, depth first.
///
/// A node whose class has its own world update (its vtable's `UpdateWorldData` isn't
/// `NiAVObject`'s `00a68c60`: a particle system in world space, a fixed-rotation `BSValueNode`,
/// a billboard, ...) is left as the original sets it, with everything below it.
fn refresh_below(e: &mut Engine, node: u32, written: &mut Vec<Written>, seen: &mut HashSet<u32>) {
    if e.vcall(node, AS_NODE_SLOT, &args![]).u32() == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node + NODE_CHILD_COUNT));
    for index in 0..count {
        let block = e.mem.u32(node + NODE_CHILDREN);
        let child = e.mem.u32(block.wrapping_add(4 * index));
        if child == 0 || e.mem.u32(child + NODE_COLLISION) != 0 || !plain_world_update(e, child) {
            continue;
        }
        if !seen.insert(child) {
            continue;
        }
        let before: [u32; 13] = words(e, child + NODE_WORLD);
        written.push(Written {
            node: child,
            world_before: before,
            world_after: None,
            bound: 0,
            bound_before: [0; 4],
            bound_after: [0; 4],
        });
        let slot = written.len() - 1;
        e.call(NODE_WORLD_UPDATE, &args![child]);
        written[slot] = moved(e, child, before);
        refresh_below(e, child, written, seen);
    }
}

/// Puts back what [`write_moves`] wrote, where it still holds what was written (a node the game's
/// world update was running on when it unwound: whatever it holds).
pub fn put_back(e: &mut Engine, written: &[Written]) {
    for w in written.iter().rev() {
        let ours = match w.world_after {
            None => true,
            Some(after) => words::<13>(e, w.node + NODE_WORLD) == after,
        };
        if ours {
            set_words(e, w.node + NODE_WORLD, &w.world_before);
        }
        if w.bound != 0
            && e.mem.u32(w.node + NODE_BOUND) == w.bound
            && words::<4>(e, w.bound) == w.bound_after
        {
            set_words(e, w.bound, &w.bound_before);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A step of 1/64 s, so the test times add up exactly.
    const STEP: f32 = 0.015_625;

    fn sample(time: f32, x: f32) -> Sample {
        Sample {
            time,
            centre: [x, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            inverse_from_previous: 1.0 / STEP,
        }
    }

    /// A sweep from `t0` to `t0 + 16 ms`, moving along x from `x0` to `x1`.
    fn step_sweep(t0: f32, x0: f32, x1: f32) -> Sweep {
        Sweep {
            start: Sample {
                inverse_from_previous: 0.0,
                ..sample(t0, x0)
            },
            end: sample(t0 + STEP, x1),
            centre_local: [0.0; 3],
        }
    }

    /// The step as the world's last one.
    fn take(body: &mut Body, sweep: Sweep) -> bool {
        take_step(body, &sweep, (sweep.start.time, sweep.end.time))
    }

    /// Where T - 16 ms falls for frames of any length: by the accumulator's rule the simulated
    /// time P after a frame is within a step either side of T, so T - 16 ms lies inside the last
    /// two steps, and the table brackets it.
    #[test]
    fn the_drawing_time_lies_inside_the_last_two_steps() {
        let mut body = Body::default();
        let mut x = 0.0;
        for n in 0..4 {
            let t0 = n as f32 * STEP;
            assert!(take(&mut body, step_sweep(t0, x, x + 1.0)));
            x += 1.0;
        }
        assert_eq!(body.samples.len(), 5);
        let p = body.samples[4].time;
        for lead in [-1.0f32, -0.5, -0.06, 0.0, 0.25, 0.5, 0.99] {
            let t = p + lead * STEP;
            let at = t - STEP;
            match drawn_at(&body.samples, at, [0.0; 3]) {
                Drawn::Between(_, position) => {
                    // Linear in time: x = at / step, the path's own place.
                    let expected = at / STEP;
                    assert!(
                        (position[0] - expected).abs() < 1e-3,
                        "{lead}: {position:?}"
                    );
                }
                other => panic!("{lead} steps ahead: {other:?}"),
            }
        }
        // At or after the last step's end: drawn as the original draws it.
        assert_eq!(drawn_at(&body.samples, p, [0.0; 3]), Drawn::AsIs);
    }

    /// The blend between a step's ends is `approxTransformAt`'s: centre and rotation linear,
    /// rotation normalised, position = centre - rotated local centre.
    #[test]
    fn the_blend_is_havoks_swept_transform() {
        let half = std::f32::consts::FRAC_1_SQRT_2;
        let start = Sample {
            time: 1.0,
            centre: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            inverse_from_previous: 0.0,
        };
        let end = Sample {
            time: 1.0 + STEP,
            centre: [2.0, 0.0, 0.0],
            rotation: [0.0, 0.0, half, half],
            inverse_from_previous: 1.0 / STEP,
        };
        let Drawn::Between(q, p) = drawn_at(&[start, end], 1.0 + STEP / 2.0, [1.0, 0.0, 0.0])
        else {
            panic!()
        };
        // Halfway: a rotation of 45 degrees about z (normalised lerp of 0 and 90).
        let angle = 2.0 * q[2].atan2(q[3]);
        assert!((angle - std::f32::consts::FRAC_PI_4).abs() < 1e-5, "{q:?}");
        let rotated = apply(&rotation_matrix(q), [1.0, 0.0, 0.0]);
        assert!((p[0] - (1.0 - rotated[0])).abs() < 1e-6);
        assert!((p[1] + rotated[1]).abs() < 1e-6);
    }

    /// A body that just woke (no table) is drawn from its first step's start, never before it.
    #[test]
    fn a_body_that_just_woke_starts_at_its_first_step() {
        let mut body = Body::default();
        assert!(take(&mut body, step_sweep(5.0, 10.0, 11.0)));
        assert_eq!(body.samples.len(), 2);
        let Drawn::First(_, p) = drawn_at(&body.samples, 5.0 - 0.004, [0.0; 3]) else {
            panic!()
        };
        assert_eq!(p[0], 10.0);
        // A sleeping body (its sweep isn't the world's last step) has no table.
        assert!(!take_step(
            &mut body,
            &step_sweep(5.0, 11.0, 11.0),
            (5.0 + STEP, 5.0 + 2.0 * STEP)
        ));
        assert!(body.samples.is_empty());
    }

    /// A teleport or `SetPosition`: the step's start isn't the end kept, or the kept end
    /// changed without a step. The table starts again from the new place.
    #[test]
    fn a_teleport_starts_the_table_again() {
        let mut body = Body::default();
        take(&mut body, step_sweep(0.0, 0.0, 1.0));
        take(&mut body, step_sweep(STEP, 1.0, 2.0));
        assert_eq!(body.samples.len(), 3);
        // The next step starts at 500, not 2.
        take(&mut body, step_sweep(2.0 * STEP, 500.0, 501.0));
        assert_eq!(
            body.samples.iter().map(|s| s.centre[0]).collect::<Vec<_>>(),
            [500.0, 501.0]
        );
        // The same step seen again but its end moved (`SetPosition` between frames).
        take(&mut body, step_sweep(2.0 * STEP, 900.0, 900.0));
        assert_eq!(
            body.samples.iter().map(|s| s.centre[0]).collect::<Vec<_>>(),
            [900.0, 900.0]
        );
        // Nothing drawn before the restart, even with T - 16 ms earlier.
        let Drawn::First(_, p) = drawn_at(&body.samples, 0.0, [0.0; 3]) else {
            panic!()
        };
        assert_eq!(p[0], 900.0);
        // Time going back (a new world) starts again too.
        take(&mut body, step_sweep(0.5, 3.0, 4.0));
        assert_eq!(body.samples[0].time, 0.5);
        assert_eq!(body.samples.len(), 2);
    }

    /// A frame with two steps: the start of the last joins the table, the step before is
    /// bridged by the two ends seen.
    #[test]
    fn steps_not_seen_are_bridged() {
        let mut body = Body::default();
        take(&mut body, step_sweep(0.0, 0.0, 1.0));
        take(&mut body, step_sweep(2.0 * STEP, 2.0, 3.0));
        assert_eq!(
            body.samples.iter().map(|s| s.centre[0]).collect::<Vec<_>>(),
            [0.0, 1.0, 2.0, 3.0]
        );
        let Drawn::Between(_, p) = drawn_at(&body.samples, 1.5 * STEP, [0.0; 3]) else {
            panic!()
        };
        assert!((p[0] - 1.5).abs() < 1e-4, "{p:?}");
        // A teleport in a frame with two steps: no slide across it, the table starts again.
        take(&mut body, step_sweep(5.0 * STEP, 500.0, 501.0));
        assert_eq!(
            body.samples.iter().map(|s| s.centre[0]).collect::<Vec<_>>(),
            [500.0, 501.0]
        );
        // So does a turn the body can't make in that time.
        let mut turned = step_sweep(7.0 * STEP, 502.0, 503.0);
        turned.start.rotation = [0.0, 0.0, 1.0, 0.0];
        turned.end.rotation = [0.0, 0.0, 1.0, 0.0];
        take(&mut body, turned);
        assert_eq!(body.samples.len(), 2);
        assert_eq!(body.samples[0].time, 7.0 * STEP);
    }

    /// A body that hit something: Havok stepped it back to the time of impact, so its sweep
    /// ends half way through the world's step; it is held there to the step's end, and the next
    /// step goes on from it.
    #[test]
    fn a_step_cut_short_by_an_impact_holds_to_the_steps_end() {
        let mut body = Body::default();
        take(&mut body, step_sweep(0.0, 0.0, 1.0));
        let mut impact = step_sweep(STEP, 1.0, 1.5);
        impact.end.time = 1.5 * STEP;
        impact.end.inverse_from_previous = 2.0 / STEP;
        assert!(take_step(&mut body, &impact, (STEP, 2.0 * STEP)));
        let times: Vec<f32> = body.samples.iter().map(|s| s.time / STEP).collect();
        let xs: Vec<f32> = body.samples.iter().map(|s| s.centre[0]).collect();
        assert_eq!(times, [0.0, 1.0, 1.5, 2.0]);
        assert_eq!(xs, [0.0, 1.0, 1.5, 1.5]);
        assert!(take(&mut body, step_sweep(2.0 * STEP, 1.5, 1.0)));
        assert_eq!(body.samples.last().unwrap().centre[0], 1.0);
        assert_eq!(body.samples.len(), 5);
        let Drawn::Between(_, p) = drawn_at(&body.samples, 1.25 * STEP, [0.0; 3]) else {
            panic!()
        };
        assert!((p[0] - 1.25).abs() < 1e-4, "{p:?}");
        // A sweep that ends after the world's step isn't this step's.
        let mut late = step_sweep(3.0 * STEP, 1.0, 1.0);
        late.end.time = 5.0 * STEP;
        assert!(!take_step(&mut body, &late, (3.0 * STEP, 4.0 * STEP)));
    }

    // ---- The scene: nodes, collision objects and Havok bodies in guest memory. --------------

    const IS_NODE: u32 = 0x7000_0010;
    const NOT_NODE: u32 = 0x7000_0020;

    struct Scene {
        e: Box<Engine>,
        world: u32,
        simulation: u32,
        node_vtable: u32,
        leaf_vtable: u32,
    }

    /// The state goes with the engine, so a later test's engine at the same address starts clean.
    impl Drop for Scene {
        fn drop(&mut self) {
            states().remove(&key(&self.e));
        }
    }

    impl Scene {
        fn new() -> Scene {
            let mut e = Box::new(Engine::new());
            for (address, value) in [
                (CARRIED_TIME, 0.0f32),
                (MAX_TIME, STEP),
                (TIME_MULTIPLIER, 1.0),
                (HAVOK_TO_GAME, 7.0),
            ] {
                e.map(address, 4);
                e.mem.set_f32(address, value);
            }
            // The transform product `00a68bf0` calls (`this * other` into `out`), as plain math:
            // the test is of the blend, not of the product's x87 order.
            e.register(0x0062_c250, |e, a| {
                let (this, out, other) = (a[0], a[1], a[2]);
                let ta: [f32; 13] = floats(e, this);
                let tb: [f32; 13] = floats(e, other);
                let ra: [f32; 9] = std::array::from_fn(|i| ta[i]);
                let rb: [f32; 9] = std::array::from_fn(|i| tb[i]);
                let rotation = multiply(&ra, &rb);
                let turned = apply(&ra, [tb[9], tb[10], tb[11]]);
                for (i, value) in rotation.iter().enumerate() {
                    e.mem.set_f32(out + 4 * i as u32, *value);
                }
                for (k, value) in turned.iter().enumerate() {
                    e.mem
                        .set_f32(out + 36 + 4 * k as u32, ta[9 + k] + ta[12] * value);
                }
                e.mem.set_f32(out + 48, ta[12] * tb[12]);
                out.into_ret()
            });
            e.register(IS_NODE, |_, a| a[0].into_ret());
            e.register(NOT_NODE, |_, _| 0u32.into_ret());
            let node_vtable = e.mem.alloc(0x100);
            e.mem.set_u32(node_vtable + AS_NODE_SLOT, IS_NODE);
            e.mem
                .set_u32(node_vtable + UPDATE_WORLD_DATA_SLOT, AV_UPDATE_WORLD_DATA);
            let leaf_vtable = e.mem.alloc(0x100);
            e.mem.set_u32(leaf_vtable + AS_NODE_SLOT, NOT_NODE);
            e.mem
                .set_u32(leaf_vtable + UPDATE_WORLD_DATA_SLOT, AV_UPDATE_WORLD_DATA);
            // A run's state left at this address by an engine of an earlier test is not this one's.
            states().remove(&key(&e));
            let world = e.mem.alloc(0x200);
            let simulation = e.mem.alloc(0x40);
            e.mem.set_u32(world + WORLD_SIMULATION, simulation);
            install(&mut e);
            Scene {
                e,
                world,
                simulation,
                node_vtable,
                leaf_vtable,
            }
        }

        /// A node (an `NiNode` with room for 4 children, or a leaf) with a bound, under `parent`.
        fn node(&mut self, parent: u32, is_node: bool, offset_x: f32) -> u32 {
            let e = &mut self.e;
            let node = e.mem.alloc(0xb0);
            e.mem.set_u32(
                node,
                if is_node {
                    self.node_vtable
                } else {
                    self.leaf_vtable
                },
            );
            e.mem.set_u32(node + 0x18, parent);
            let bound = e.mem.alloc(0x10);
            e.mem.set_f32(bound + 0xc, 3.0);
            e.mem.set_u32(node + NODE_BOUND, bound);
            // Local and world: identity rotation, scale 1.
            for base in [0x34, NODE_WORLD] {
                for i in [0, 4, 8] {
                    e.mem.set_f32(node + base + 4 * i, 1.0);
                }
                e.mem.set_f32(node + base + 0x30, 1.0);
            }
            e.mem.set_f32(node + 0x34 + 0x24, offset_x);
            if is_node {
                let block = e.mem.alloc(0x10);
                e.mem.set_u32(node + NODE_CHILDREN, block);
            }
            if parent != 0 {
                let count = u32::from(e.mem.u16(parent + NODE_CHILD_COUNT));
                let block = e.mem.u32(parent + NODE_CHILDREN);
                e.mem.set_u32(block + 4 * count, node);
                e.mem.set_u16(parent + NODE_CHILD_COUNT, count as u16 + 1);
                e.call(NODE_WORLD_UPDATE, &args![node]);
            }
            node
        }

        /// A free body on `node`, its collision object noted as `hktoNiTransform` leaves it,
        /// at centre x `x` (Havok units).
        fn body(&mut self, node: u32, x: f32) -> (u32, u32) {
            let e = &mut self.e;
            let havok = e.mem.alloc(0x200);
            e.mem.set_u32(havok + HAVOK_WORLD, self.world);
            e.mem.set_f32(havok + SWEEP_ROTATION_START + 12, 1.0);
            e.mem.set_f32(havok + SWEEP_ROTATION_END + 12, 1.0);
            let body = e.mem.alloc(0x20);
            e.mem.set_u32(body + BODY_HAVOK, havok);
            let collision = e.mem.alloc(0x14);
            e.mem.set_u32(collision + COLLISION_NODE, node);
            e.mem.set_u32(collision + COLLISION_BODY, body);
            e.mem.set_u32(node + NODE_COLLISION, collision);
            self.place(havok, x, x);
            (collision, havok)
        }

        /// The body stepped from `x0` to `x1` in the world's step that ends at `end`, the
        /// node set to the end as `hktoNiTransform` does and noted.
        fn step(&mut self, collision: u32, havok: u32, end: f32, x0: f32, x1: f32) {
            let e = &mut self.e;
            e.mem.set_f32(self.world + WORLD_STEP_END, end);
            e.mem.set_f32(self.simulation + SIMULATION_TIME, end);
            self.place(havok, x0, x1);
            let e = &mut self.e;
            e.mem.set_f32(havok + SWEEP_CENTRE_START + 12, end - STEP);
            e.mem.set_f32(havok + SWEEP_CENTRE_END + 12, 1.0 / STEP);
            let node = e.mem.u32(collision + COLLISION_NODE);
            e.mem.set_f32(node + NODE_WORLD + 0x24, 7.0 * x1);
            note(e, collision);
        }

        fn place(&mut self, havok: u32, x0: f32, x1: f32) {
            let e = &mut self.e;
            e.mem.set_f32(havok + SWEEP_CENTRE_START, x0);
            e.mem.set_f32(havok + SWEEP_CENTRE_END, x1);
            e.mem.set_f32(havok + HAVOK_TRANSLATION, x1);
        }
    }

    fn world_of(e: &Engine, node: u32) -> [u32; 13] {
        words(e, node + NODE_WORLD)
    }

    /// The nodes are drawn blended, the node below the body moves with it, and after the drawing
    /// every world transform and bound is bit for bit what it was.
    #[test]
    fn the_drawing_sees_the_blend_and_every_node_is_put_back() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, true, 0.0);
        let mesh = s.node(item, false, 2.0);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        // Simulated time at the end of the second step, a quarter step carried: T - one step is
        // a quarter step after the first step's end, x = 1.25 Havok units.
        s.e.mem.set_f32(CARRIED_TIME, 0.25 * STEP);
        // The scene graph's update: the mesh follows the item, its bound centred on it.
        s.e.call(NODE_WORLD_UPDATE, &args![mesh]);
        let nodes = [root, item, mesh];
        let bounds: Vec<u32> = nodes.iter().map(|n| s.e.mem.u32(n + NODE_BOUND)).collect();
        let mesh_world_x = s.e.mem.f32(mesh + NODE_WORLD + 0x24);
        assert_eq!(mesh_world_x, 16.0);
        s.e.mem.set_f32(bounds[2], mesh_world_x);
        let before: Vec<[u32; 13]> = nodes.iter().map(|n| world_of(&s.e, *n)).collect();
        let bounds_before: Vec<[u32; 4]> = bounds.iter().map(|b| words(&s.e, *b)).collect();
        let written = blend_for_drawing(&mut s.e);
        let x = s.e.mem.f32(item + NODE_WORLD + 0x24);
        assert!((x - 7.0 * 1.25).abs() < 1e-4, "{x}");
        let mesh_x = s.e.mem.f32(mesh + NODE_WORLD + 0x24);
        assert!((mesh_x - (x + 2.0)).abs() < 1e-4, "{mesh_x}");
        let bound_x = s.e.mem.f32(bounds[2]);
        assert!((bound_x - mesh_x).abs() < 1e-4, "{bound_x}");
        assert_eq!(world_of(&s.e, root), before[0]);
        put_back(&mut s.e, &written);
        for (i, n) in nodes.iter().enumerate() {
            assert_eq!(world_of(&s.e, *n), before[i], "node {i}");
            assert_eq!(words::<4>(&s.e, bounds[i]), bounds_before[i], "bound {i}");
        }
    }

    /// A node with its own world update below a body (a world-space particle system, a
    /// fixed-rotation `BSValueNode`) is left as the original sets it, and a keyframed body isn't
    /// blended at all.
    #[test]
    fn own_world_updates_and_keyframed_bodies_are_left_alone() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, true, 0.0);
        let particles = s.node(item, false, 2.0);
        let own = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(own + AS_NODE_SLOT, NOT_NODE);
        // `NiParticleSystem::UpdateWorldData`.
        s.e.mem.set_u32(own + UPDATE_WORLD_DATA_SLOT, 0x00c1_add0);
        s.e.mem.set_u32(particles, own);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        let before = world_of(&s.e, particles);
        let written = blend_for_drawing(&mut s.e);
        assert_eq!(written.len(), 1);
        assert_eq!(world_of(&s.e, particles), before);
        put_back(&mut s.e, &written);
        // Keyframed: no note, nothing written.
        let other = s.node(root, false, 0.0);
        let (collision, havok) = s.body(other, 0.0);
        s.e.mem.set_u8(havok + MOTION_TYPE, 4);
        s.step(collision, havok, 1.0 + 2.0 * STEP, 2.0, 3.0);
        assert!(blend_for_drawing(&mut s.e).iter().all(|w| w.node != other));
    }

    /// A noted object freed and its memory used again (another vtable) isn't written.
    #[test]
    fn a_note_whose_object_changed_is_dropped() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, false, 0.0);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        s.e.mem.set_u32(collision, 0x0102_0304);
        assert!(blend_for_drawing(&mut s.e).is_empty());
    }

    /// A note counts only for the drawing right after it: a body `hktoNiTransform` didn't reach
    /// in this frame (its object may be gone, its memory used again by the same class) isn't
    /// written.
    #[test]
    fn a_note_from_an_earlier_frame_writes_nothing() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, false, 0.0);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        let written = blend_for_drawing(&mut s.e);
        assert_eq!(written.len(), 1);
        put_back(&mut s.e, &written);
        assert!(blend_for_drawing(&mut s.e).is_empty());
    }

    /// The nodes go back when the game's world update on a node below the body unwinds, and
    /// when the drawing does.
    #[test]
    fn an_unwind_while_writing_or_drawing_puts_everything_back() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, true, 0.0);
        let mesh = s.node(item, false, 2.0);
        let (collision, havok) = s.body(item, 0.0);
        let nodes = [root, item, mesh];
        let both = |s: &mut Scene| {
            s.step(collision, havok, 1.0, 0.0, 1.0);
            s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
            s.e.call(NODE_WORLD_UPDATE, &args![mesh]);
        };
        both(&mut s);
        let before: Vec<[u32; 13]> = nodes.iter().map(|n| world_of(&s.e, *n)).collect();
        // The drawing unwinds.
        let drew = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            draw_blended(&mut s.e, |e| {
                assert_ne!(world_of(e, item), before[1]);
                panic!("the drawing stops");
            })
        }));
        assert!(drew.is_err());
        for (i, n) in nodes.iter().enumerate() {
            assert_eq!(world_of(&s.e, *n), before[i], "node {i} after the drawing");
        }
        // The world update on the mesh writes half its transform, then unwinds.
        both(&mut s);
        s.e.register(NODE_WORLD_UPDATE, |e, a| {
            e.mem.set_f32(a[0] + NODE_WORLD + 0x24, 1234.0);
            panic!("the world update stops");
        });
        let wrote = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            draw_blended(&mut s.e, |_| unreachable!())
        }));
        assert!(wrote.is_err());
        for (i, n) in nodes.iter().enumerate() {
            assert_eq!(world_of(&s.e, *n), before[i], "node {i} after the writing");
        }
    }

    /// A node something else wrote during the drawing keeps that write.
    #[test]
    fn a_write_during_the_drawing_stays() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, false, 0.0);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        let written = blend_for_drawing(&mut s.e);
        assert_eq!(written.len(), 1);
        s.e.mem.set_f32(item + NODE_WORLD + 0x24, 99.0);
        put_back(&mut s.e, &written);
        assert_eq!(s.e.mem.f32(item + NODE_WORLD + 0x24), 99.0);
    }

    /// A node that changed after `hktoNiTransform` (game logic moved it) isn't blended, and a
    /// body that didn't run the world's last step (asleep) is drawn as it is.
    #[test]
    fn moved_or_sleeping_bodies_are_drawn_as_they_are() {
        let mut s = Scene::new();
        let root = s.node(0, true, 0.0);
        let item = s.node(root, false, 0.0);
        let (collision, havok) = s.body(item, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.step(collision, havok, 1.0 + STEP, 1.0, 2.0);
        s.e.mem.set_f32(item + NODE_WORLD + 0x28, 5.0);
        assert!(blend_for_drawing(&mut s.e).is_empty());
        let other = s.node(root, false, 0.0);
        let (collision, havok) = s.body(other, 0.0);
        s.step(collision, havok, 1.0, 0.0, 1.0);
        s.e.mem.set_f32(s.world + WORLD_STEP_END, 2.0);
        assert!(blend_for_drawing(&mut s.e).is_empty());
    }

    /// The vanilla preset registers neither address: the translations run as they are.
    #[test]
    fn vanilla_registers_nothing() {
        use crate::fixes::Choice;
        let vanilla = Choice::vanilla();
        assert!(!vanilla.is_on(&FIX));
        let fixes = Choice::from_parts(None, |_| None).unwrap();
        assert!(fixes.is_on(&FIX));
        let off = Choice::from_parts(None, |name| {
            (name == "NV_PRESET" || name == "NV_PHYS_BLEND")
                .then(|| if name == "NV_PRESET" { "fixes" } else { "0" }.to_string())
        })
        .unwrap();
        assert!(!off.is_on(&FIX));
        // Installed vanilla: `hktoNiTransform` notes nothing, there is no state for the engine.
        let mut e = Box::new(Engine::new());
        // Whatever an earlier test's engine at this address left.
        states().remove(&key(&e));
        crate::fixes::install(&mut e, &vanilla);
        assert!(!states().contains_key(&key(&e)));
        assert!(blend_for_drawing(&mut e).is_empty());
    }
}
