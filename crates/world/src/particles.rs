//! Particle systems as the game runs them: Gamebryo's particle update in
//! `FalloutNV.exe`, rule by rule, on the blocks `nif::particles` reads.
//! The findings with every address are in `nv-re\findings\particles.md`.
//!
//! Each frame the system's controllers run in their chain's order:
//!
//! * an emitter controller (`NiPSysEmitterCtlr::Update` `00c1c630`) works out
//!   its time (its own clock, or the playing sequence's when a controller
//!   manager drives it), walks the "emitter active" keys between the last
//!   time and now, and for each stretch the emitter was on emits the
//!   particles due in it (`00c1c3c0`): particle n of a stretch is due once
//!   `trunc((t − start) × birth rate)` reaches n, at most 15 a call, each
//!   aged by how long ago it was due;
//! * the emitter (`NiPSysEmitter::EmitParticles` `00c220c0`) gives each its
//!   life span, speed and direction, its place in the emitter's volume, its
//!   colour, radius and sub-texture, and lets the modifiers set it up;
//! * the update controller (`00c271c0`) runs the system (`00c1b260`): new
//!   particles join, then every active modifier updates them in the list's
//!   order: age and death, colour, size, rotation, forces, colliders, then
//!   the position modifier moves each by its velocity over the time since
//!   it was last moved.
//!
//! Random numbers follow the game's rules (which value is drawn, in which
//! order, and how) with the Microsoft C runtime's `rand()` sequence; the
//! game shares one generator with everything else, so the numbers
//! themselves can't match.

use std::sync::Arc;

use nif::particles::{
    BoolInterp, Collider, ColliderShape, ControllerKind, Emitter, EmitterShape, FloatInterp,
    MeshGeometry, ModifierKind, ObjectRef, ParticleSequence, ParticleSystem, Spawn, TimeControl,
    TrackValue,
};
use nif::Transform;

pub type Vec3 = [f32; 3];

/// The game's "no time yet" (`-FLT_MAX`: the double at `010241b0` read as
/// a float).
pub const INVALID_TIME: f32 = -f32::MAX;

/// Most particles one emission step makes (`[011ae75c]`, 15, read as a
/// u16 by `00c1c3c0`).
pub const MAX_EMITTED_AT_ONCE: u16 = 15;

/// The gravity modifier's factor on its strength (the double at
/// `0106efa0`, 1.6, in `00c22b70`).
pub const GRAVITY_FACTOR: f32 = 1.6;

/// The drag modifier's frame (`010bfee8`, 1/30 s): its percentage is per
/// thirtieth of a second (`00c2c170`).
pub const DRAG_FRAME: f32 = 0.033_333_3;

/// The collision tests' tolerance (`010beaf8`, `010be824`: 0.001).
pub const COLLISION_EPSILON: f32 = 0.001;

/// The Microsoft C runtime's `rand()` (`00ecadb8`): the linear
/// congruential generator `s = s × 214013 + 2531011`, giving bits 16–30.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rand(pub u32);

impl Rand {
    pub fn draw(&mut self) -> i32 {
        self.0 = self.0.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.0 >> 16) & 0x7fff) as i32
    }

    /// `rand() / 32767.0`, divided in double and kept as a float, as every
    /// particle routine does (`FILD`, `FDIV [01029790]`, `FSTP float`).
    pub fn unit(&mut self) -> f32 {
        (f64::from(self.draw()) / 32767.0) as f32
    }

    /// `(2 × rand()) / 32767.0 − 1` (declination, planar angle, radius,
    /// rotation): from −1 to 1.
    pub fn signed(&mut self) -> f32 {
        let r = f64::from(self.draw());
        ((r + r) / 32767.0 - 1.0) as f32
    }

    /// `006397c0`: `rand() × 2 / 32767 − 1` in floats (gravity's
    /// turbulence).
    pub fn turbulence(&mut self) -> f32 {
        (self.draw() as f32 * 2.0) / 32767.0 - 1.0
    }
}

/// Gamebryo's sine and cosine tables (`011f52e0`, `011f4ae0`, filled by
/// `00a813c0`): 512 entries, the angle stepped in floats by `2π (float) ×
/// 1/512` from 0. Emitters and shapes look angles up here
/// (`trunc(angle × 512 / 2π) & 511`).
pub struct SinCos {
    pub sin: [f32; 512],
    pub cos: [f32; 512],
}

impl SinCos {
    pub fn new() -> SinCos {
        let mut sin = [0.0; 512];
        let mut cos = [0.0; 512];
        let step = f64::from(std::f32::consts::TAU) * 0.001_953_125;
        let mut x = 0.0f32;
        for i in 0..512 {
            sin[i] = f64::from(x).sin() as f32;
            cos[i] = f64::from(x).cos() as f32;
            x = (f64::from(x) + step) as f32;
        }
        SinCos { sin, cos }
    }

    /// The table index for an angle (radians): `ftol(angle × 512/2π)`
    /// (truncated toward zero) masked to 9 bits. Worked out in floats, as
    /// the x87 unit does in the single precision Direct3D 9 sets it to
    /// (a device made without `D3DCREATE_FPU_PRESERVE`; an assumption):
    /// π/2 gives 128.
    pub fn index(angle: f32) -> usize {
        let k = 512.0f32 / std::f32::consts::TAU;
        ((angle * k) as i64 as i32 & 0x1ff) as usize
    }
}

impl Default for SinCos {
    fn default() -> Self {
        Self::new()
    }
}

fn table() -> &'static SinCos {
    static TABLE: std::sync::OnceLock<SinCos> = std::sync::OnceLock::new();
    TABLE.get_or_init(SinCos::new)
}

/// x87 `FISTP` with the default rounding (to nearest, halves to even),
/// which `ROUND` in the game's code is.
pub fn round_even(x: f32) -> i32 {
    let r = x.round();
    if (x - x.trunc()).abs() == 0.5 && (r as i64) % 2 != 0 {
        (r - x.signum()) as i32
    } else {
        r as i32
    }
}

/// `00c1ced0`: `x × y × (1.5 − 0.5 × x × y²)` with `y` the bit trick
/// `0x5f3759df − (bits(x) >> 1)`: the square root of `x` (one Newton step).
pub fn fast_sqrt(x: f32) -> f32 {
    let y = f32::from_bits(0x5f37_59df_u32.wrapping_sub(x.to_bits() >> 1));
    x * y * (1.5 - y * y * 0.5 * x)
}

fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale(a: Vec3, s: f32) -> Vec3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `004a0c10`: unit length, or zero when shorter than 1e-6.
fn unitize(v: Vec3) -> Vec3 {
    let l = dot(v, v).sqrt();
    if l <= 1e-6 {
        [0.0; 3]
    } else {
        scale(v, 1.0 / l)
    }
}

/// A controller's clock (`NiTimeController` fields `+0x1c`..`+0x28`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    pub start: f32,
    pub last: f32,
    pub weighted: f32,
    pub scaled: f32,
    pub forced: bool,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            start: INVALID_TIME,
            last: INVALID_TIME,
            weighted: 0.0,
            scaled: INVALID_TIME,
            forced: false,
        }
    }
}

/// The key range a controller cycles through.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub lo: f32,
    pub hi: f32,
}

/// `NiTimeController::ComputeScaledTime` (`00a6cd60`): the first update
/// counts from the time itself (from 0 with "app init"), later ones add
/// `frequency × (t − last)`; plus the phase; then cycled through the key
/// range (loop: `lo + fmod(s − lo, hi − lo)`; reverse: back and forth;
/// clamp), clamped to it, and turned around for "play backwards".
pub fn compute_scaled_time(clock: &mut Clock, tc: &TimeControl, range: Range, t: f32) -> f32 {
    if clock.start == INVALID_TIME {
        clock.start = t;
    }
    let delta = if clock.last == INVALID_TIME {
        clock.weighted = 0.0;
        if tc.flags & 1 != 0 {
            0.0
        } else {
            t
        }
    } else {
        t - clock.last
    };
    clock.weighted += tc.frequency * delta;
    let scaled = clock.weighted + tc.phase;
    clock.last = t;
    cycle(tc, range, scaled)
}

fn cycle(tc: &TimeControl, Range { lo, hi }: Range, scaled: f32) -> f32 {
    let mut s = scaled;
    if hi != INVALID_TIME && lo != f32::MAX {
        let span = hi - lo;
        match tc.cycle() {
            0 => {
                if span == 0.0 {
                    s = lo;
                } else {
                    s = (s - lo) % span + lo;
                    if s < lo {
                        s += span;
                    }
                }
            }
            1 => {
                if span == 0.0 {
                    s = lo;
                } else {
                    let mut f = s % (span + span);
                    if f < 0.0 {
                        f += span + span;
                    }
                    if span < f {
                        f = span + span - f;
                    }
                    s = f + lo;
                }
            }
            _ => {}
        }
    }
    if s > hi {
        s = hi;
    } else if s < lo {
        s = lo;
    }
    if tc.flags & 0x10 != 0 {
        s = hi - (s - lo);
    }
    s
}

/// `NiTimeController::DontDoUpdate` (`00a36250`): true when nothing should
/// happen this frame (switched off, the same time again, or the scaled time
/// unchanged). Updates the clock's scaled time when it runs.
pub fn dont_do_update(clock: &mut Clock, tc: &TimeControl, range: Range, t: f32) -> bool {
    if !tc.active() {
        return true;
    }
    if t == clock.last && !clock.forced {
        return true;
    }
    if tc.flags & 0x40 == 0 {
        clock.forced = false;
        return false;
    }
    let scaled = compute_scaled_time(clock, tc, range, t);
    if scaled != clock.scaled || clock.forced {
        clock.scaled = scaled;
        clock.forced = false;
        return false;
    }
    true
}

/// A float interpolator's value at a time: its keys when it has any
/// (quadratic keys as Hermite curves, constant keys held), else its value.
pub fn float_at(interp: &FloatInterp, t: f32) -> f32 {
    nif::sample_curve(&interp.keys.keys, t).unwrap_or(interp.value)
}

/// A bool interpolator's value at a time (`00a29250` finds the keys around
/// it; the step keys' function `00a2cc40` gives the earlier key's value
/// until the later key's time is reached). Without keys, its value (1 on).
pub fn bool_at(interp: &BoolInterp, t: f32) -> bool {
    if interp.keys.is_empty() {
        return interp.value == 1;
    }
    bool_interval(&interp.keys, t).0
}

/// The value at `t` and the index of the key starting the interval that
/// holds it (`00a29250`: the first key at or after `t` closes it; before
/// the first key, the first interval).
fn bool_interval(keys: &[(f32, bool)], t: f32) -> (bool, usize) {
    if keys.len() == 1 || t < keys[0].0 {
        return (keys[0].1, 0);
    }
    for (i, pair) in keys.windows(2).enumerate() {
        let (a, b) = (pair[0], pair[1]);
        if t <= b.0 {
            let span = b.0 - a.0;
            let ratio = if span > 0.0 { (t - a.0) / span } else { 1.0 };
            return (if ratio < 1.0 { a.1 } else { b.1 }, i);
        }
    }
    (keys[keys.len() - 1].1, keys.len() - 1)
}

/// `NiBoolTimelineInterpolator`'s state: the interval last found and the
/// value read there.
#[derive(Debug, Clone, Copy, Default)]
struct Timeline {
    index: usize,
    raw: bool,
}

/// `NiBoolTimelineInterpolator::Update` (`00a52d80`): the keys' value at
/// `t`, except that when it equals the last value read and a key of the
/// other value lies strictly between the last interval and this one
/// (round past the end when time wrapped), that other value: a short
/// switch between two frames isn't missed.
fn timeline_at(state: &mut Timeline, interp: &BoolInterp, t: f32) -> bool {
    let keys = &interp.keys;
    if keys.is_empty() {
        return interp.value == 1;
    }
    let previous = state.index;
    let (v, index) = bool_interval(keys, t);
    let mut out = v;
    if v == state.raw {
        let crossed = |range: std::ops::Range<usize>| range.into_iter().any(|i| keys[i].1 != v);
        let n = keys.len();
        let found = if previous <= index {
            crossed(previous + 1..index.min(n))
        } else {
            crossed((previous + 1).min(n)..n) || crossed(0..index)
        };
        if found {
            out = !v;
        }
    }
    state.index = index;
    state.raw = v;
    out
}

/// One particle (`NiParticleInfo`, 28 bytes, and its entries in the data
/// arrays).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub age: f32,
    pub life_span: f32,
    /// When it was last moved (`m_fLastUpdate`).
    pub last_update: f32,
    pub generation: u16,
    pub color: [f32; 4],
    pub radius: f32,
    pub size: f32,
    pub rotation_angle: f32,
    pub rotation_speed: f32,
    pub texture_index: u8,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            position: [0.0; 3],
            velocity: [0.0; 3],
            age: 0.0,
            life_span: 0.0,
            last_update: 0.0,
            generation: 0,
            color: [1.0; 4],
            radius: 1.0,
            size: 1.0,
            rotation_angle: 0.0,
            rotation_speed: 0.0,
            texture_index: 0,
        }
    }
}

/// The particles of a system (`NiPSysData`): the active ones first, then
/// those added since the last update, which join at the next one
/// (`ResolveAddedParticles` `00c24d20`). Removing one moves the last active
/// particle into its place (`00a964c0`).
#[derive(Debug, Clone, Default)]
pub struct Particles {
    pub list: Vec<Particle>,
    pub active: usize,
    added: usize,
    added_base: usize,
    pub max: usize,
}

impl Particles {
    pub fn new(max: usize) -> Particles {
        Particles {
            list: Vec::with_capacity(max),
            active: 0,
            added: 0,
            added_base: 0,
            max,
        }
    }

    /// `NiPSysData::AddParticle` (`00c24ea0`): a slot after the active
    /// particles, or none when the system is full.
    fn add(&mut self) -> Option<usize> {
        let slot = if self.added == 0 {
            self.added_base = self.active;
            if self.added_base >= self.max {
                return None;
            }
            self.added = 1;
            self.added_base
        } else if self.added_base + self.added < self.max {
            self.added += 1;
            self.added_base + self.added - 1
        } else {
            return None;
        };
        if self.list.len() <= slot {
            self.list.resize(slot + 1, Particle::default());
        }
        self.list[slot] = Particle::default();
        Some(slot)
    }

    /// `RemoveParticle` (`00a964c0`): the last active particle takes its
    /// place.
    fn remove(&mut self, i: usize) {
        if self.active == 0 || i >= self.active {
            return;
        }
        let last = self.active - 1;
        if i != last {
            self.list[i] = self.list[last];
        }
        self.active -= 1;
    }

    /// `ResolveAddedParticles` (`00c24d20`): the particles added since the
    /// last update become active; when some were removed in between, the
    /// gap is closed by moving the newest down.
    fn resolve_added(&mut self) {
        let active = self.active;
        let base = self.added_base;
        if active < base {
            self.active = self.added + base;
            let mut i = active;
            while i < base && self.active > base {
                self.remove(i);
                i += 1;
            }
            if i < base {
                self.active = i;
            }
        } else {
            self.active = self.added + active;
        }
        self.added = 0;
        self.added_base = self.active;
    }

    pub fn active(&self) -> &[Particle] {
        &self.list[..self.active.min(self.list.len())]
    }
}

/// The state of one controller of a system.
#[derive(Debug, Clone, Default)]
struct ControllerState {
    clock: Clock,
    /// Emitter controllers: the time the keys were last walked to
    /// (`+0x48`) and whether the emitter was on then (`+0x4c`).
    last_scaled: Option<f32>,
    last_active: bool,
    /// `NiPSysResetOnLoopCtlr`'s last scaled time (`+0x34`).
    reset_last: Option<f32>,
    /// The sequence whose interpolators drove it last (`+0x44`).
    driven_by: Option<String>,
    /// A timeline interpolator's state (modifier active controllers).
    timeline: Timeline,
}

/// What a playing sequence hands a controller under a controller manager:
/// which sequence, its time now, and its interpolators for the controller.
#[derive(Debug, Clone, Default)]
struct Driven {
    sequence: String,
    time: f32,
    float: Option<FloatInterp>,
    bool: Option<BoolInterp>,
}

/// What a frame hands the simulation.
pub struct Frame<'a> {
    /// The scene's clock (seconds) the controllers are updated with.
    pub time: f32,
    /// The system's world transform now (its node chain under the placed
    /// object), before world-space systems drop all but its scale.
    pub system_world: Transform,
    /// The world transform now of another scene object of the model
    /// (emitter volumes, gravity and bomb objects, colliders), by block.
    pub object_world: &'a dyn Fn(usize) -> Option<Transform>,
    /// The model's sequences playing now, each with its time: controllers
    /// a controller manager drives take their interpolators and time from
    /// the first that has them.
    pub playing: &'a [(&'a ParticleSequence, f32)],
    /// `BSWindModifier`'s wind vector (`01202e68`).
    pub wind: Vec3,
}

/// The playing sequence's interpolators for a controller: tracks naming
/// the system, the controller's type and its modifier.
fn driven(system: &str, type_name: &str, modifier: &str, frame: &Frame) -> Option<Driven> {
    for (sequence, time) in frame.playing {
        let mut d = Driven {
            sequence: sequence.name.clone(),
            time: *time,
            ..Driven::default()
        };
        for t in &sequence.tracks {
            if t.node == system && t.controller_type == type_name && t.controller_id == modifier {
                match &t.value {
                    TrackValue::Float(f) => d.float = Some(f.clone()),
                    TrackValue::Bool(b) => d.bool = Some(b.clone()),
                }
            }
        }
        if d.float.is_some() || d.bool.is_some() {
            return Some(d);
        }
    }
    None
}

/// Runtime copies of the modifier values controllers change.
#[derive(Debug, Clone)]
struct ModifierState {
    active: bool,
    emitter: Option<Emitter>,
    gravity_strength: f32,
    rotation: Option<nif::particles::Rotation>,
}

/// A collider set up for this frame in the system's space.
#[derive(Debug, Clone, Copy)]
enum ColliderNow {
    /// `00c294b0`: centre, the turned x and y axes, the plane (normal,
    /// distance), the squared half width and height.
    Plane {
        center: Vec3,
        x_axis: Vec3,
        y_axis: Vec3,
        normal: Vec3,
        distance: f32,
        half_width2: f32,
        half_height2: f32,
    },
    /// `00c27f60`: centre, radius, radius².
    Sphere {
        center: Vec3,
        radius: f32,
        radius2: f32,
    },
    None,
}

/// A particle system running.
pub struct System {
    pub def: Arc<ParticleSystem>,
    pub particles: Particles,
    /// `+0xd4`: the time of the last update.
    pub last_update: f32,
    /// `+0xd8`: clear everything at the next update.
    reset: bool,
    controllers: Vec<ControllerState>,
    modifiers: Vec<ModifierState>,
    pub rand: Rand,
    /// The transform the particles live in this frame (`+0x68` after
    /// `UpdateWorldData` `00c1add0`: for world-space systems only the
    /// scale is left).
    pub sim_world: Transform,
}

impl System {
    pub fn new(def: Arc<ParticleSystem>, seed: u32) -> System {
        let max = usize::from(def.data.max_particles);
        let modifiers = def
            .modifiers
            .iter()
            .map(|m| ModifierState {
                active: m.active,
                emitter: match &m.kind {
                    ModifierKind::Emitter(e) => Some(e.clone()),
                    _ => None,
                },
                gravity_strength: match &m.kind {
                    ModifierKind::Gravity(g) => g.strength,
                    _ => 0.0,
                },
                rotation: match &m.kind {
                    ModifierKind::Rotation(r) => Some(r.clone()),
                    _ => None,
                },
            })
            .collect();
        System {
            controllers: vec![ControllerState::default(); def.controllers.len()],
            modifiers,
            particles: Particles::new(max),
            last_update: INVALID_TIME,
            reset: false,
            rand: Rand(seed),
            sim_world: Transform::IDENTITY,
            def,
        }
    }

    /// The particles alive now.
    pub fn active(&self) -> &[Particle] {
        self.particles.active()
    }

    fn modifier_named(&self, name: &str) -> Option<usize> {
        self.def.modifiers.iter().position(|m| m.name == name)
    }

    /// One frame: every controller in its chain's order.
    pub fn update(&mut self, frame: &Frame) {
        // UpdateWorldData (00c1add0): world-space systems keep only their
        // scale.
        self.sim_world = if self.def.world_space {
            Transform {
                scale: frame.system_world.scale,
                ..Transform::IDENTITY
            }
        } else {
            frame.system_world
        };
        let def = Arc::clone(&self.def);
        for (k, c) in def.controllers.iter().enumerate() {
            match &c.kind {
                ControllerKind::Emitter {
                    modifier,
                    birth_rate,
                    active,
                    multi_target: None,
                } => self.emitter_controller(
                    k,
                    &c.type_name,
                    &c.time,
                    modifier,
                    (birth_rate, active),
                    frame,
                ),
                ControllerKind::Update => self.update_controller(k, &c.time, frame),
                ControllerKind::ModifierActive { modifier, value } => self
                    .modifier_active_controller(k, &c.type_name, &c.time, modifier, value, frame),
                ControllerKind::ModifierFloat { modifier, value } => {
                    self.modifier_float_controller(k, &c.type_name, &c.time, modifier, value, frame)
                }
                ControllerKind::ResetOnLoop => self.reset_on_loop(k, &c.time, frame),
                _ => {}
            }
        }
    }

    /// `NiPSysUpdateCtlr::Update` (`00c271c0`): the scaled time with the key
    /// range opened to everything and the cycle forced to clamp, then the
    /// system's update. When the first emitter controller loops and the
    /// gap since its last update is longer than its emitter's particles
    /// live, the clock jumps to now and this frame is skipped.
    fn update_controller(&mut self, k: usize, tc: &TimeControl, frame: &Frame) {
        let mut forced = *tc;
        forced.flags = (forced.flags & 0xfffd) | 4;
        let range = Range {
            lo: -f32::MAX,
            hi: f32::MAX,
        };
        let t = frame.time;
        let clock_last = self.controllers[k].clock.last;
        if clock_last != INVALID_TIME {
            if let Some(life) = self.looping_emitter_life() {
                if life < t - clock_last {
                    self.controllers[k].clock.last = t;
                }
            }
        }
        let clock = &mut self.controllers[k].clock;
        if dont_do_update(clock, &forced, range, t) {
            return;
        }
        let scaled = clock.scaled;
        self.update_system(scaled, frame);
    }

    /// The life span of the emitter the first emitter controller drives,
    /// when that controller loops (`00c271c0` looks up the first
    /// `NiPSysEmitterCtlr` on the system and its emitter's `+0x48`).
    fn looping_emitter_life(&self) -> Option<f32> {
        let c = self
            .def
            .controllers
            .iter()
            .find(|c| matches!(c.kind, ControllerKind::Emitter { .. }))?;
        if c.time.flags & 6 != 0 {
            return None;
        }
        let ControllerKind::Emitter { modifier, .. } = &c.kind else {
            return None;
        };
        let m = self.modifier_named(modifier)?;
        self.modifiers[m].emitter.as_ref().map(|e| e.life_span)
    }

    /// `NiPSysModifierBoolCtlr::Update` (`00c32cb0`) for the modifier
    /// active controller (`00c29d60`): the modifier on or off by the
    /// interpolator at the controller's scaled time, or under a controller
    /// manager the playing sequence's interpolator at the sequence's time.
    /// An interpolator without keys whose value isn't set (2) does nothing.
    fn modifier_active_controller(
        &mut self,
        k: usize,
        type_name: &str,
        tc: &TimeControl,
        modifier: &str,
        value: &Option<BoolInterp>,
        frame: &Frame,
    ) {
        let Some(m) = self.modifier_named(modifier) else {
            return;
        };
        let range = Range {
            lo: tc.start,
            hi: tc.stop,
        };
        let manager = tc.manager_controlled();
        let (interp, time) = if manager {
            self.controllers[k].clock.scaled = INVALID_TIME;
            match driven(&self.def.name, type_name, modifier, frame) {
                Some(Driven {
                    bool: Some(b),
                    time,
                    ..
                }) => (b, time),
                _ => return,
            }
        } else {
            let clock = &mut self.controllers[k].clock;
            if dont_do_update(clock, tc, range, frame.time) {
                return;
            }
            match value {
                Some(v) => (v.clone(), clock.scaled),
                None => return,
            }
        };
        if interp.keys.is_empty() && interp.value > 1 {
            return;
        }
        let on = if interp.timeline {
            timeline_at(&mut self.controllers[k].timeline, &interp, time)
        } else {
            bool_at(&interp, time)
        };
        self.modifiers[m].active = on;
    }

    /// `NiPSysModifierFloatCtlr::Update` (`00c32ee0`) and the setters at
    /// vtable slot 61: the emitter's speed, radius, life span, angles, the
    /// gravity's strength, the rotation's starting values.
    fn modifier_float_controller(
        &mut self,
        k: usize,
        type_name: &str,
        tc: &TimeControl,
        modifier: &str,
        value: &Option<FloatInterp>,
        frame: &Frame,
    ) {
        let Some(m) = self.modifier_named(modifier) else {
            return;
        };
        let range = Range {
            lo: tc.start,
            hi: tc.stop,
        };
        let manager = tc.manager_controlled();
        let v = if manager {
            self.controllers[k].clock.scaled = INVALID_TIME;
            match driven(&self.def.name, type_name, modifier, frame) {
                Some(Driven {
                    float: Some(f),
                    time,
                    ..
                }) => float_at(&f, time),
                _ => return,
            }
        } else {
            let clock = &mut self.controllers[k].clock;
            if dont_do_update(clock, tc, range, frame.time) {
                return;
            }
            match value {
                Some(f) => float_at(f, clock.scaled),
                None => return,
            }
        };
        // A value without keys left "not set" gives nothing.
        if v == -f32::MAX {
            return;
        }
        let state = &mut self.modifiers[m];
        if let Some(e) = state.emitter.as_mut() {
            match type_name {
                "NiPSysEmitterSpeedCtlr" => e.speed = v,
                "NiPSysEmitterInitialRadiusCtlr" => e.radius = v,
                "NiPSysEmitterLifeSpanCtlr" => e.life_span = v,
                "NiPSysEmitterDeclinationCtlr" => e.declination = v,
                "NiPSysEmitterDeclinationVarCtlr" => e.declination_variation = v,
                "NiPSysEmitterPlanarAngleCtlr" => e.planar_angle = v,
                "NiPSysEmitterPlanarAngleVarCtlr" => e.planar_angle_variation = v,
                _ => {}
            }
        }
        if type_name == "NiPSysGravityStrengthCtlr" {
            state.gravity_strength = v;
        }
        if let Some(r) = state.rotation.as_mut() {
            match type_name {
                "NiPSysInitialRotSpeedCtlr" => r.speed = v,
                "NiPSysInitialRotSpeedVarCtlr" => r.speed_variation = v,
                "NiPSysInitialRotAngleCtlr" => r.angle = v,
                "NiPSysInitialRotAngleVarCtlr" => r.angle_variation = v,
                _ => {}
            }
        }
    }

    /// `NiPSysResetOnLoopCtlr::Update` (`00c28490`): with its cycle forced
    /// to loop, the system is cleared at the next update once its scaled
    /// time goes back.
    fn reset_on_loop(&mut self, k: usize, tc: &TimeControl, frame: &Frame) {
        let mut looped = *tc;
        looped.flags &= 0xfff9;
        let range = Range {
            lo: tc.start,
            hi: tc.stop,
        };
        let state = &mut self.controllers[k];
        if dont_do_update(&mut state.clock, &looped, range, frame.time) {
            return;
        }
        let now = state.clock.scaled;
        let last = *state.reset_last.get_or_insert(now);
        if now < last {
            self.reset = true;
        }
        self.controllers[k].reset_last = Some(now);
    }

    /// `NiPSysEmitterCtlr::Update` (`00c1c630`): see the module's notes.
    fn emitter_controller(
        &mut self,
        k: usize,
        type_name: &str,
        tc: &TimeControl,
        modifier: &str,
        (own_rate, own_active): (&Option<FloatInterp>, &Option<BoolInterp>),
        frame: &Frame,
    ) {
        let t = frame.time;
        let Some(m) = self.modifier_named(modifier) else {
            return;
        };
        let range = Range {
            lo: tc.start,
            hi: tc.stop,
        };
        let manager = tc.manager_controlled();
        let life = self.modifiers[m]
            .emitter
            .as_ref()
            .map_or(0.0, |e| e.life_span);
        let state = &mut self.controllers[k];
        if t < state.clock.last {
            state.last_scaled = None;
        }
        if manager {
            state.clock.scaled = INVALID_TIME;
        }
        if state.clock.last != INVALID_TIME && tc.flags & 6 == 0 && life < t - state.clock.last {
            state.clock.last = t;
        }
        if dont_do_update(&mut state.clock, tc, range, t) {
            return;
        }
        // The interpolators: the controller's own at its scaled time, or
        // (under a controller manager) the playing sequence's at the
        // sequence's time; none playing, nothing happens.
        let (rate, active, now) = if manager {
            let Some(d) = driven(&self.def.name, type_name, modifier, frame) else {
                return;
            };
            // A new sequence: its time is noted and nothing is emitted
            // this frame (`+0x44` changed).
            if state.driven_by.as_deref() != Some(d.sequence.as_str()) {
                state.driven_by = Some(d.sequence.clone());
                state.last_scaled = Some(d.time);
                return;
            }
            (d.float, d.bool, d.time)
        } else {
            (own_rate.clone(), own_active.clone(), state.clock.scaled)
        };
        let (Some(rate), Some(active)) = (rate, active) else {
            return;
        };
        if active.keys.is_empty() {
            self.constant_emitter(k, m, &rate, active.value, now, frame);
            return;
        }
        let keys = active.keys.clone();
        let on_now = bool_at(&active, now);
        let state = &mut self.controllers[k];
        let Some(mut last) = state.last_scaled else {
            state.last_scaled = Some(now);
            state.last_active = on_now;
            return;
        };
        let mut was_on = state.last_active;
        state.last_active = on_now;
        state.last_scaled = Some(now);
        let n = keys.len();
        let span = range.hi - range.lo;
        for _ in 0..20 {
            let wrapped = now < last;
            let start_k = if !was_on {
                let found = (0..n)
                    .find(|&i| last < keys[i].0 && (wrapped || keys[i].0 <= now) && keys[i].1);
                match found {
                    Some(i) => i,
                    None => {
                        if !wrapped {
                            return;
                        }
                        match (0..n).find(|&i| keys[i].0 < now && keys[i].1) {
                            Some(i) => i,
                            None => return,
                        }
                    }
                }
            } else {
                (0..n)
                    .rev()
                    .find(|&i| keys[i].0 <= last && !keys[i].1)
                    .map_or(0, |i| i + 1)
            };
            let end_k = (start_k..n).find(|&i| !keys[i].1).unwrap_or(n - 1);
            let start = keys[start_k.min(n - 1)].0;
            let end_t = keys[end_k].0;
            // (current, last for the count, stop, when the birth rate is read)
            let (current, emit_last, stop, at) = if wrapped {
                if end_t <= last {
                    let stop = if end_t < now { end_t } else { now };
                    (now, keys[0].0, stop, start + (stop - start) * 0.5)
                } else {
                    (keys[n - 1].0, last, end_t, start + (end_t - start) * 0.5)
                }
            } else if now <= end_t {
                (now, last, end_t, now)
            } else {
                (end_t, last, end_t, start + (end_t - start) * 0.5)
            };
            let r = float_at(&rate, at);
            // The particles' clock: the system's last update (as it
            // stands, even unset), a whole loop on when time wrapped, plus
            // the time since the keys were last walked.
            let mut time = self.last_update;
            if wrapped {
                time += span;
            }
            let time = time + now - last;
            self.emit_stretch(m, time, current, emit_last, start, stop, r, frame);
            if now <= end_t {
                return;
            }
            last = current;
            was_on = bool_at(&active, current);
        }
    }

    /// `00c1c630` from `00c1c865`: "emitter active" without keys. Off (0)
    /// or not set (2), nothing (and the last time isn't moved on). On: the
    /// particles due between the last time and now, counted from 0 (from
    /// 0 again once time wrapped), on the system's clock moved on by the
    /// time between; with birth-rate keys, counted from the first key and
    /// the rate read halfway, a wrapped stretch first run to the last key.
    fn constant_emitter(
        &mut self,
        k: usize,
        m: usize,
        rate: &FloatInterp,
        value: u8,
        now: f32,
        frame: &Frame,
    ) {
        if value != 1 {
            return;
        }
        let state = &mut self.controllers[k];
        let Some(last) = state.last_scaled.replace(now) else {
            // (The game would count from −FLT_MAX here; it never starts
            // without a time when a sequence drives it.)
            return;
        };
        let system_last = self.last_update;
        if rate.keys.keys.is_empty() {
            if rate.value == -f32::MAX {
                return;
            }
            let (last, delta) = if last > now {
                (0.0, now)
            } else {
                (last, now - last)
            };
            self.emit_stretch(
                m,
                system_last + delta,
                now,
                last,
                0.0,
                now,
                rate.value,
                frame,
            );
            return;
        }
        let keys = &rate.keys.keys;
        let (first, final_key) = (keys[0].time, keys[keys.len() - 1].time);
        let mut last = last;
        let mut extra = 0.0;
        if now < last {
            let d = final_key - last;
            let r = float_at(rate, d * 0.5 + last);
            self.emit_stretch(
                m,
                system_last + d,
                final_key,
                last,
                first,
                final_key,
                r,
                frame,
            );
            extra = d;
            last = first;
        }
        let d = now - last;
        let r = float_at(rate, d * 0.5 + last);
        self.emit_stretch(m, system_last + d + extra, now, last, first, now, r, frame);
    }

    /// `00c1c3c0`: the particles due between `last` and `current` in the
    /// stretch the emitter is on from `start` to `stop`.
    #[allow(clippy::too_many_arguments)]
    fn emit_stretch(
        &mut self,
        m: usize,
        time: f32,
        current: f32,
        last: f32,
        start: f32,
        stop: f32,
        rate: f32,
        frame: &Frame,
    ) {
        let ages = emission_ages(current, last, start, stop, rate);
        if !ages.is_empty() {
            self.emit(m, time, &ages, frame);
        }
    }

    /// `NiPSysEmitter::EmitParticles` (`00c220c0`).
    fn emit(&mut self, m: usize, time: f32, ages: &[f32], frame: &Frame) {
        let Some(e) = self.modifiers[m].emitter.clone() else {
            return;
        };
        let def = Arc::clone(&self.def);
        let data = &def.data;
        let sub_textures = data.subtexture_offsets.len();
        let tab = table();
        for &age in ages {
            let life = ((f64::from(self.rand.unit()) - 0.5) * f64::from(e.life_span_variation)
                + f64::from(e.life_span)) as f32;
            if age > life {
                continue;
            }
            let Some(slot) = self.particles.add() else {
                return;
            };
            let speed = ((f64::from(self.rand.unit()) - 0.5) * f64::from(e.speed_variation)
                + f64::from(e.speed)) as f32;
            let declination = self.rand.signed() * e.declination_variation + e.declination;
            let planar = self.rand.signed() * e.planar_angle_variation + e.planar_angle;
            let (i1, i2) = (SinCos::index(declination), SinCos::index(planar));
            let (sd, cd) = (tab.sin[i1], tab.cos[i1]);
            let (sp, cp) = (tab.sin[i2], tab.cos[i2]);
            let mut velocity = [sd * cp * speed, sd * sp * speed, cd * speed];
            let mut position = [0.0; 3];
            self.initial_position(&e, &mut position, &mut velocity, frame);
            // The radius array is always there (`00a96cf0` allocates it).
            // `+0x50` is 1 (the constructor's, `00c21f00`).
            let radius = self.rand.signed() * e.radius_variation + e.radius;
            let texture_index = if data.has_texture_indices && sub_textures > 0 {
                (self.rand.draw() % sub_textures as i32) as u8
            } else {
                0
            };
            {
                let p = &mut self.particles.list[slot];
                p.position = position;
                p.velocity = velocity;
                p.age = age;
                p.life_span = life;
                p.generation = 0;
                if data.has_colors {
                    p.color = e.color;
                }
                p.radius = radius;
                p.size = 1.0;
                p.texture_index = texture_index;
                p.last_update = time - age;
            }
            self.initialize_particle(slot);
        }
    }

    /// From an object's space into the system's: `inverse(system) ×
    /// object` (`004b4880`, `0062c250`).
    fn system_from(&self, object: Option<&ObjectRef>, frame: &Frame) -> Transform {
        let world = object
            .and_then(|o| (frame.object_world)(o.block))
            .unwrap_or(self.sim_world);
        self.sim_world.inverse().then_child(&world)
    }

    /// The emitter's `ComputeInitialPositionAndVelocity`: for volume
    /// emitters (`00c31cf0`) a point in the shape (`00c201d0` box,
    /// `00c1fa50` cylinder, `00c1fe20` sphere), carried from the emitter
    /// object's space into the system's with the velocity turned the same
    /// way; for mesh emitters (`00c1f5f0`) a point on a mesh.
    fn initial_position(
        &mut self,
        e: &Emitter,
        position: &mut Vec3,
        velocity: &mut Vec3,
        frame: &Frame,
    ) {
        let tab = table();
        let (point, object) = match &e.shape {
            EmitterShape::Box {
                width,
                height,
                depth,
                object,
            } => {
                // The calls run right to left: z, then y, then x.
                let z = (f64::from(self.rand.unit()) - 0.5) as f32 * depth;
                let y = (f64::from(self.rand.unit()) - 0.5) as f32 * height;
                let x = width * (f64::from(self.rand.unit()) - 0.5) as f32;
                ([x, y, z], object.as_ref())
            }
            EmitterShape::Cylinder {
                radius,
                height,
                object,
            } => {
                let r = radius * self.rand.unit();
                let angle = std::f32::consts::TAU * self.rand.unit();
                let i = SinCos::index(angle);
                let z = (f64::from(self.rand.unit()) - 0.5) as f32 * height;
                ([r * tab.cos[i], r * tab.sin[i], z], object.as_ref())
            }
            EmitterShape::Sphere { radius, object } => {
                let r = radius * self.rand.unit();
                let a = std::f32::consts::TAU * self.rand.unit();
                let b = std::f32::consts::TAU * self.rand.unit();
                let (i1, i2) = (SinCos::index(a), SinCos::index(b));
                let rs = r * tab.sin[i2];
                (
                    [tab.cos[i1] * rs, tab.sin[i1] * rs, r * tab.cos[i2]],
                    object.as_ref(),
                )
            }
            EmitterShape::Array { object } => ([0.0; 3], object.as_ref()),
            EmitterShape::Mesh {
                meshes,
                geometry,
                velocity_type,
                emission_type,
                axis,
            } => {
                self.mesh_point(
                    e,
                    meshes,
                    geometry,
                    *velocity_type,
                    *emission_type,
                    *axis,
                    position,
                    velocity,
                    frame,
                );
                return;
            }
        };
        let into = self.system_from(object, frame);
        *position = into.apply_point(point);
        *velocity = into.apply_direction(*velocity);
    }

    /// `NiPSysMeshEmitter` (`00c1f5f0`), unskinned meshes: one picked at
    /// `trunc(n × U)` (n − 1 at most), then a vertex (`00c1d9e0`, emission
    /// type 0), a triangle's centre or a point on it (`00c1eb20`, types 1
    /// and 3: `a = sqrt(U)`, `b = U`, the point `v0 + e1 + a(b e2 − e1)`),
    /// or an edge's middle or a point on it (`00c1dae0`, types 2 and 4);
    /// triangles with a repeated corner are passed over (6 tries). With
    /// velocity type 0 the speed goes along the normal there; then into
    /// the system's space (`00c1cf50`), where type 1 takes a random
    /// direction and type 2 the emission axis.
    #[allow(clippy::too_many_arguments)]
    fn mesh_point(
        &mut self,
        e: &Emitter,
        meshes: &[ObjectRef],
        geometry: &[Option<MeshGeometry>],
        velocity_type: u32,
        emission_type: u32,
        axis: Vec3,
        position: &mut Vec3,
        velocity: &mut Vec3,
        frame: &Frame,
    ) {
        let count = meshes.len();
        if count == 0 {
            return;
        }
        // Chopped (`FISTP` with the control word set to truncate).
        let pick = ((count as f32 * self.rand.unit()) as usize).min(count - 1);
        let Some(Some(g)) = geometry.get(pick) else {
            return;
        };
        if g.positions.is_empty() {
            return;
        }
        let speed = dot(*velocity, *velocity).sqrt();
        let normals = g.normals.len() == g.positions.len();
        let along_normal = |n: Vec3| scale(unitize(n), speed);
        let triangle = |rand: &mut Rand| -> Option<[usize; 3]> {
            let n = g.triangles.len();
            if n == 0 {
                return None;
            }
            let mut i = (rand.draw() as usize) % n;
            for _ in 0..6 {
                let t = g.triangles[i];
                if t[0] != t[1] && t[0] != t[2] && t[1] != t[2] {
                    break;
                }
                i = (i + 1) % n;
            }
            let t = g.triangles[i];
            Some([usize::from(t[0]), usize::from(t[1]), usize::from(t[2])])
        };
        match emission_type {
            1 | 3 => {
                let Some([a, b, c]) = triangle(&mut self.rand) else {
                    return;
                };
                let (p0, p1, p2) = (g.positions[a], g.positions[b], g.positions[c]);
                *position = scale(add(add(p0, p1), p2), 0.333_333_34);
                if velocity_type == 0 && normals {
                    let n = add(add(g.normals[a], g.normals[b]), g.normals[c]);
                    *velocity = along_normal(scale(n, 0.333_333_34));
                }
                if emission_type == 3 {
                    let e1 = sub(p1, p0);
                    let e2 = sub(p2, p0);
                    let s = fast_sqrt(self.rand.unit());
                    let t = self.rand.unit();
                    let q = add(scale(sub(scale(e2, t), e1), s), p0);
                    *position = add(e1, q);
                }
            }
            2 | 4 => {
                let Some([a, b, c]) = triangle(&mut self.rand) else {
                    return;
                };
                let (x, y) = match self.rand.draw() % 3 {
                    1 => (b, c),
                    2 => (a, c),
                    _ => (a, b),
                };
                let (p0, p1) = (g.positions[x], g.positions[y]);
                *position = scale(add(p0, p1), 0.5);
                if velocity_type == 0 && normals {
                    *velocity = along_normal(scale(add(g.normals[x], g.normals[y]), 0.5));
                }
                if emission_type == 4 {
                    let t = self.rand.unit();
                    *position = add(scale(sub(p1, p0), t), p0);
                }
            }
            _ => {
                let i = (self.rand.draw() as usize) % g.positions.len();
                *position = g.positions[i];
                if velocity_type == 0 && normals {
                    *velocity = scale(g.normals[i], speed);
                }
            }
        }
        let into = self.system_from(meshes.get(pick), frame);
        *position = into.apply_point(*position);
        match velocity_type {
            1 => {
                let d = unitize([self.rand.signed(), self.rand.signed(), self.rand.signed()]);
                let s = (f64::from(self.rand.unit()) - 0.5) as f32 * e.speed_variation + e.speed;
                *velocity = scale(d, s);
            }
            2 => {
                let d = unitize(into.apply_direction(axis));
                let s = (f64::from(self.rand.unit()) - 0.5) as f32 * e.speed_variation + e.speed;
                *velocity = scale(d, s);
            }
            _ => *velocity = into.apply_direction(*velocity),
        }
    }

    /// `NiParticleSystem::InitializeNewParticle` (`00c1aee0`): every
    /// modifier's set-up for a new particle (only the rotation modifier
    /// has one among those read here: `00c207a0`).
    fn initialize_particle(&mut self, slot: usize) {
        let def = Arc::clone(&self.def);
        for (m, _) in def.modifiers.iter().enumerate() {
            let Some(r) = self.modifiers[m].rotation.clone() else {
                continue;
            };
            // (Rotation axes: no file of the game's has them.)
            if def.data.has_rotation_angles {
                let angle = r.angle_variation * self.rand.signed() + r.angle;
                let mut speed = r.speed_variation * self.rand.signed() + r.speed;
                if r.random_speed_sign && self.rand.unit() <= 0.5 {
                    speed = -speed;
                }
                let p = &mut self.particles.list[slot];
                p.rotation_angle = angle;
                if def.data.has_rotation_speeds {
                    p.rotation_speed = speed;
                }
            }
        }
    }

    /// `NiParticleSystem::Do_UpdateSystem` (`00c1b260`).
    fn update_system(&mut self, time: f32, frame: &Frame) {
        self.particles.resolve_added();
        if self.reset || time < self.last_update {
            self.reset_system();
            self.reset = false;
        }
        if self.last_update == INVALID_TIME {
            self.last_update = time;
        }
        let def = Arc::clone(&self.def);
        let mut last: Option<usize> = None;
        for (m, modifier) in def.modifiers.iter().enumerate() {
            // `BSPSysStripUpdateModifier` runs after the others (vtable
            // slot 40 true), the last such one.
            if matches!(modifier.kind, ModifierKind::StripUpdate { .. }) {
                last = Some(m);
                continue;
            }
            if self.modifiers[m].active {
                self.run_modifier(m, time, frame);
            }
        }
        if let Some(m) = last {
            if self.modifiers[m].active {
                self.run_modifier(m, time, frame);
            }
        }
        self.last_update = time;
    }

    /// `ResetParticleSystem` (`00c1b120`): no particles, no last time, and
    /// every modifier controller's state cleared.
    fn reset_system(&mut self) {
        self.particles.active = 0;
        self.particles.added = 0;
        self.particles.added_base = 0;
        self.last_update = INVALID_TIME;
        for c in &mut self.controllers {
            c.last_scaled = None;
        }
    }

    fn run_modifier(&mut self, m: usize, time: f32, frame: &Frame) {
        let def = Arc::clone(&self.def);
        match &def.modifiers[m].kind {
            ModifierKind::AgeDeath { .. } => self.age_death(time),
            ModifierKind::Position => self.position(time),
            ModifierKind::Rotation(_) => self.rotate(time),
            ModifierKind::GrowFade(g) => self.grow_fade(g),
            ModifierKind::SimpleColor(c) => self.simple_color(c),
            ModifierKind::Color { keys, .. } => self.color_keys(keys),
            ModifierKind::Gravity(g) => {
                let strength = self.modifiers[m].gravity_strength;
                self.gravity(g, strength, time, frame)
            }
            ModifierKind::Drag(d) => self.drag(d, time, frame),
            ModifierKind::Bomb(b) => self.bomb(b, time, frame),
            ModifierKind::Wind { strength } => {
                for p in &mut self.particles.list[..self.particles.active] {
                    let d = (time - p.last_update) * strength;
                    p.velocity = add(p.velocity, scale(frame.wind, d));
                }
            }
            ModifierKind::Colliders(colliders) => self.colliders(colliders, time, frame),
            _ => {}
        }
    }

    /// `NiPSysAgeDeathModifier::Update` (`00c2f010`): from the last particle
    /// back, each ages by the time since it was last moved and goes once
    /// its age passes its life span. (Spawning on death: no file of the
    /// game's asks for it.)
    fn age_death(&mut self, time: f32) {
        let mut i = self.particles.active;
        while i > 0 {
            i -= 1;
            let p = &mut self.particles.list[i];
            p.age += time - p.last_update;
            if p.life_span < p.age {
                self.particles.remove(i);
            }
        }
    }

    /// `NiPSysPositionModifier::Update` (`00c28bc0`): particles added this
    /// update join, then each moves by its velocity over the time since it
    /// was last moved.
    fn position(&mut self, time: f32) {
        self.particles.resolve_added();
        for p in &mut self.particles.list[..self.particles.active] {
            let dt = time - p.last_update;
            p.position = add(p.position, scale(p.velocity, dt));
            p.last_update = time;
        }
    }

    /// `NiPSysRotationModifier::Update` (`00c206b0`): the angle turns by
    /// the speed over the time since the particle was last moved; past
    /// 10π it goes back to 0, past 2π it wraps.
    fn rotate(&mut self, time: f32) {
        if !self.def.data.has_rotation_angles {
            return;
        }
        let two_pi = 6.283_185_5f32;
        let limit = 31.415_928f32;
        for p in &mut self.particles.list[..self.particles.active] {
            let mut a = (time - p.last_update) * p.rotation_speed + p.rotation_angle;
            if a <= limit {
                while two_pi < a {
                    a -= two_pi;
                }
            } else {
                a = 0.0;
            }
            p.rotation_angle = a;
        }
    }

    /// `NiPSysGrowFadeModifier::Update` (`00c2a390`): the size grows from 0
    /// over the grow time and shrinks over the fade time before death (for
    /// the generations named), `min(grow, fade) × (1 − base) + base`, at
    /// least 0.0001.
    fn grow_fade(&mut self, g: &nif::particles::GrowFade) {
        for p in &mut self.particles.list[..self.particles.active] {
            let mut grow = 1.0;
            if p.generation == g.grow_generation && p.age < g.grow_time && g.grow_time != 0.0 {
                grow = p.age / g.grow_time;
            }
            let mut fade = 1.0;
            let left = p.life_span - p.age;
            if p.generation == g.fade_generation && left < g.fade_time && g.fade_time != 0.0 {
                fade = left / g.fade_time;
            }
            let s = if grow < fade { grow } else { fade };
            let s = s * (1.0 - g.base_scale) + g.base_scale;
            p.size = if s < 1e-4 { 1e-4 } else { s };
        }
    }

    /// `BSPSysSimpleColorModifier::Update` (`00c602e0`): by the share of its
    /// life a particle has lived, colour 1 → 2 between `color1_end` and
    /// `color2_start`, 2 → 3 between `color2_end` and `color3_start`; alpha
    /// from colour 1's to 2's over the fade-in and from 2's to 3's after
    /// the fade-out mark.
    fn simple_color(&mut self, c: &nif::particles::SimpleColor) {
        if !self.def.data.has_colors {
            return;
        }
        let [c1, c2, c3] = c.colors;
        let span12 = c.color2_start - c.color1_end;
        let span23 = c.color3_start - c.color2_end;
        let mix = |a: [f32; 4], b: [f32; 4], f: f32| {
            [
                (b[0] - a[0]) * f + a[0],
                (b[1] - a[1]) * f + a[1],
                (b[2] - a[2]) * f + a[2],
            ]
        };
        for p in &mut self.particles.list[..self.particles.active] {
            let t = p.age / p.life_span;
            let rgb = if span12 == 0.0 || c.color2_start <= t {
                if span23 == 0.0 || t <= c.color2_end {
                    [c2[0], c2[1], c2[2]]
                } else if t <= c.color3_start {
                    mix(c2, c3, (t - c.color2_end) / span23)
                } else {
                    [c3[0], c3[1], c3[2]]
                }
            } else if c.color1_end <= t {
                mix(c1, c2, (t - c.color1_end) / span12)
            } else {
                [c1[0], c1[1], c1[2]]
            };
            let alpha = if c.fade_out == 0.0 || t <= c.fade_out {
                if c.fade_in == 0.0 || c.fade_in <= t {
                    c2[3]
                } else {
                    (c2[3] - c1[3]) * (t / c.fade_in) + c1[3]
                }
            } else {
                (c3[3] - c2[3]) * ((t - c.fade_out) / (1.0 - c.fade_out)) + c2[3]
            };
            p.color = [rgb[0], rgb[1], rgb[2], alpha];
        }
    }

    /// `NiPSysColorModifier::Update` (`00c2cfb0`): the colour keys at the
    /// share of life lived, clamped to the keys' times, straight lines
    /// between keys (every file's are linear, kind 1).
    fn color_keys(&mut self, keys: &[nif::particles::ColorKey]) {
        if !self.def.data.has_colors || keys.is_empty() {
            return;
        }
        let (first, last) = (keys[0].time, keys[keys.len() - 1].time);
        for p in &mut self.particles.list[..self.particles.active] {
            let mut t = p.age / p.life_span;
            if t <= first {
                t = first;
            }
            if last <= t {
                t = last;
            }
            let mut c = keys[keys.len() - 1].color;
            for pair in keys.windows(2) {
                if t <= pair[1].time {
                    let span = pair[1].time - pair[0].time;
                    let f = if span > 0.0 {
                        (t - pair[0].time) / span
                    } else {
                        0.0
                    };
                    c = [0, 1, 2, 3]
                        .map(|k| pair[0].color[k] + (pair[1].color[k] - pair[0].color[k]) * f);
                    break;
                }
            }
            p.color = c;
        }
    }

    /// `NiPSysGravityModifier::Update` (`00c22b70`): with no gravity object
    /// nothing happens. The axis (turned into the system's space, unit
    /// length) or the direction to the object (spherical, unit length),
    /// times `strength × 1.6`, times `exp(−decay × distance)` with decay
    /// (along the axis for planar gravity), plus turbulence (each axis a
    /// random −1..1 × `turbulence × scale × 500`), over the time since
    /// each particle was last moved.
    fn gravity(&mut self, g: &nif::particles::Gravity, strength: f32, time: f32, frame: &Frame) {
        let Some(object) = g.object.as_ref() else {
            return;
        };
        if self.particles.active == 0 {
            return;
        }
        let into = self.system_from(Some(object), frame);
        let axis = unitize(into.apply_direction(g.axis));
        let center = into.translation;
        let force = strength * GRAVITY_FACTOR;
        let turbulence = g.turbulence_scale * g.turbulence * 500.0;
        let spherical = g.force_type == 1;
        if g.force_type > 1 {
            return;
        }
        for i in 0..self.particles.active {
            let mut noise = [0.0; 3];
            if g.turbulence != 0.0 {
                noise = [
                    self.rand.turbulence() * turbulence,
                    self.rand.turbulence() * turbulence,
                    self.rand.turbulence() * turbulence,
                ];
            }
            let p = &mut self.particles.list[i];
            let (direction, falloff) = if spherical {
                let to = sub(center, p.position);
                let length = dot(to, to).sqrt();
                let d = unitize(to);
                let f = if g.decay != 0.0 {
                    (-g.decay * length).exp()
                } else {
                    1.0
                };
                (d, f)
            } else {
                let f = if g.decay != 0.0 {
                    let along = dot(sub(center, p.position), axis);
                    (-g.decay * along.abs()).exp()
                } else {
                    1.0
                };
                (axis, f)
            };
            let dt = time - p.last_update;
            let a = add(scale(direction, falloff * force), noise);
            p.velocity = add(p.velocity, scale(a, dt));
        }
    }

    /// `NiPSysDragModifier::Update` (`00c2c170`): a share of each particle's
    /// speed along the axis is taken off, `percentage × (time since last
    /// moved) / (1/30 s)` (all of it past 1), full within the range and
    /// tapering to nothing at the falloff distance from the object.
    fn drag(&mut self, d: &nif::particles::Drag, time: f32, frame: &Frame) {
        if d.percentage.is_nan() || d.percentage <= 0.0 || self.particles.active == 0 {
            return;
        }
        let Some(object) = d.object.as_ref() else {
            return;
        };
        let into = self.system_from(Some(object), frame);
        let axis = unitize(into.apply_direction(d.axis));
        let length2 = dot(axis, axis);
        let center = into.translation;
        let taper = d.range_falloff - d.range;
        for p in &mut self.particles.list[..self.particles.active] {
            let to = sub(p.position, center);
            let distance = dot(to, to).sqrt();
            let frames = (time - p.last_update) / DRAG_FRAME;
            let share = if distance <= d.range {
                d.percentage
            } else if distance < d.range_falloff {
                (1.0 - (distance - d.range) / taper) * d.percentage
            } else {
                continue;
            };
            let along = dot(axis, p.velocity);
            if share * frames <= 1.0 {
                let k = (along / length2) * -share * frames;
                p.velocity = add(p.velocity, scale(axis, k));
            } else {
                p.velocity = add(p.velocity, scale(axis, -along / length2));
            }
        }
    }

    /// `NiPSysBombModifier::Update` (`00c2e670`): a push of `delta v` a
    /// second away from the bomb object (symmetry 0 spherical, 1
    /// cylindrical about the axis, 2 planar along the axis), within the
    /// decay distance (decay type 1 linear `(decay − d)/decay`, 2
    /// `exp(−d/decay)`; type 0 everywhere at full strength).
    fn bomb(&mut self, b: &nif::particles::Bomb, time: f32, frame: &Frame) {
        let (center, axis) = match b.object.as_ref() {
            Some(o) if self.particles.active > 0 => {
                let into = self.system_from(Some(o), frame);
                (into.translation, unitize(into.apply_direction(b.axis)))
            }
            _ => ([0.0; 3], b.axis),
        };
        for p in &mut self.particles.list[..self.particles.active] {
            if p.last_update >= time || p.last_update.is_nan() {
                continue;
            }
            let to = sub(p.position, center);
            let mut distance = dot(to, to).sqrt();
            if b.decay_type != 0 && distance > b.decay {
                continue;
            }
            let direction = match b.symmetry_type {
                0 => scale(to, 1.0 / distance),
                1 => {
                    let along = dot(to, axis);
                    let off = sub(to, scale(axis, along));
                    let l = dot(off, off).sqrt();
                    if l != 0.0 {
                        scale(off, 1.0 / l)
                    } else {
                        off
                    }
                }
                2 => {
                    let along = dot(axis, to);
                    distance = along;
                    if along < 0.0 {
                        distance = -along;
                        scale(axis, -1.0)
                    } else {
                        axis
                    }
                }
                _ => continue,
            };
            let falloff = match b.decay_type {
                1 => (b.decay - distance) / b.decay,
                2 => (-distance / b.decay).exp(),
                _ => 1.0,
            };
            let push = b.delta_v * falloff * (time - p.last_update);
            p.velocity = add(p.velocity, scale(direction, push));
        }
    }

    /// The colliders as they stand this frame (`00c294b0` planes,
    /// `00c27f60` spheres), in the system's space.
    fn colliders_now(&self, colliders: &[Collider], frame: &Frame) -> Vec<ColliderNow> {
        colliders
            .iter()
            .map(|c| {
                // Without an object the collider stays in the system's own
                // space.
                let into = match c.object.as_ref() {
                    Some(object) => self.system_from(Some(object), frame),
                    None => Transform::IDENTITY,
                };
                match &c.shape {
                    ColliderShape::Plane {
                        width,
                        height,
                        x_axis,
                        y_axis,
                    } => {
                        let normal =
                            unitize(into.apply_direction(unitize(cross(*x_axis, *y_axis))));
                        let hw = width * into.scale * 0.5;
                        let hh = height * into.scale * 0.5;
                        ColliderNow::Plane {
                            center: into.translation,
                            x_axis: into.apply_direction(*x_axis),
                            y_axis: into.apply_direction(*y_axis),
                            normal,
                            distance: dot(normal, into.translation),
                            half_width2: hw * hw,
                            half_height2: hh * hh,
                        }
                    }
                    ColliderShape::Sphere { radius } => {
                        let r = radius * into.scale;
                        ColliderNow::Sphere {
                            center: into.translation,
                            radius: r,
                            radius2: r * r,
                        }
                    }
                    ColliderShape::Other(_) => ColliderNow::None,
                }
            })
            .collect()
    }

    /// `NiPSysColliderManager::Update` (`00c2d300`): for each particle the
    /// earliest collision with any collider between the system's last
    /// update and now (each collider's test, `00c28d10` planes, `00c27b40`
    /// spheres); a hit puts the particle at the point of impact, bounces
    /// its velocity (`00c29320`, `00c27e20`), spawns or kills it
    /// (`00c32490`), and its last-moved time becomes the time of impact.
    /// Every other particle's last-moved time becomes the system's last
    /// update.
    fn colliders(&mut self, colliders: &[Collider], time: f32, frame: &Frame) {
        if colliders.is_empty() {
            return;
        }
        let now_shapes = self.colliders_now(colliders, frame);
        let start = self.last_update;
        let mut i = 0;
        while i < self.particles.active {
            let mut end = time;
            let mut hit: Option<(usize, Vec3)> = None;
            let p = self.particles.list[i];
            for (k, shape) in now_shapes.iter().enumerate() {
                if let Some((t, at)) = collide(shape, &p, start, end) {
                    end = t;
                    hit = Some((k, at));
                }
            }
            let mut moved_at = start;
            if let Some((k, at)) = hit {
                moved_at = end;
                let c = &colliders[k];
                {
                    let q = &mut self.particles.list[i];
                    q.velocity = bounce(&now_shapes[k], q.velocity, at, c.bounce, time != end);
                    q.position = at;
                }
                if c.spawn_on_collide {
                    if let Some(spawn) = c.spawn.and_then(|b| self.spawn_modifier(b)) {
                        self.spawn(&spawn, time, end, i);
                    }
                }
                if c.die_on_collide {
                    self.particles.remove(i);
                }
            }
            // The slot's particle (the one moved in after a removal too)
            // was last moved at the time of impact, or the system's last
            // update; the loop goes on to the next slot.
            if i < self.particles.active {
                self.particles.list[i].last_update = moved_at;
            }
            i += 1;
        }
    }

    fn spawn_modifier(&self, block: usize) -> Option<Spawn> {
        self.def.modifiers.iter().find_map(|m| match &m.kind {
            ModifierKind::Spawn(s) if m.block == block => Some(s.clone()),
            _ => None,
        })
    }

    /// `NiPSysSpawnModifier` (`00c23db0`, `00c23ee0`): for a particle of a
    /// generation below the limit, with chance `percentage`, `min + (max −
    /// min) × U` (rounded half down; at least 1) new ones at its place,
    /// each with the parent's speed × (1 + variation × U), turned off its
    /// direction by `direction variation × U × π` about a random azimuth,
    /// a life span of `life ± variation/2`, the parent's colour, size and
    /// turn, one generation on, aged from the moment of spawning.
    fn spawn(&mut self, s: &Spawn, time: f32, at: f32, parent: usize) {
        let p = self.particles.list[parent];
        if p.generation >= s.generations || {
            let u = self.rand.unit();
            u > s.percentage || u.is_nan() || s.percentage.is_nan()
        } {
            return;
        }
        let x = f32::from(s.max.wrapping_sub(s.min)) * self.rand.unit();
        let mut n = x.trunc() as u16;
        if x % 1.0 > 0.5 {
            n += 1;
        }
        let mut count = s.min.wrapping_add(n);
        if count == 0 {
            count = 1;
        }
        let tab = table();
        for _ in 0..count {
            let Some(slot) = self.particles.add() else {
                return;
            };
            let speed_now = dot(p.velocity, p.velocity).sqrt();
            let speed = (s.speed_variation * self.rand.unit() + 1.0) * speed_now;
            let cone = s.direction_variation * self.rand.unit() * std::f32::consts::PI;
            let azimuth = std::f32::consts::TAU * self.rand.unit();
            let (i1, i2) = (SinCos::index(cone), SinCos::index(azimuth));
            let local = [
                tab.sin[i1] * tab.cos[i2],
                tab.sin[i1] * tab.sin[i2],
                tab.cos[i1],
            ];
            let direction = turn_z_to(local, p.velocity);
            let life = (self.rand.unit() - 0.5) * s.life_span_variation + s.life_span;
            let q = &mut self.particles.list[slot];
            *q = p;
            q.velocity = scale(direction, speed);
            q.age = time - at;
            q.life_span = life;
            q.generation = p.generation + 1;
            q.last_update = time - q.age;
            self.initialize_particle(slot);
        }
    }
}

/// `v` turned the way that takes +Z onto the direction of `to` (`00c23ee0`:
/// the shortest turn; along +Z nothing, along −Z the matrix −1). Which
/// twist about `to` the game's matrix has doesn't change where spawned
/// particles go, as their azimuth is uniformly random.
fn turn_z_to(v: Vec3, to: Vec3) -> Vec3 {
    let length = dot(to, to).sqrt();
    if length == 0.0 {
        return v;
    }
    let to = scale(to, 1.0 / length);
    let z = [0.0, 0.0, 1.0];
    let c = dot(z, to);
    if to[0] * to[0] + to[1] * to[1] <= 1e-8 {
        return if c >= 0.0 { v } else { scale(v, -1.0) };
    }
    let k = unitize(cross(z, to));
    let s = (1.0 - c * c).max(0.0).sqrt();
    // Rodrigues: v cos + (k × v) sin + k (k·v)(1 − cos).
    add(
        add(scale(v, c), scale(cross(k, v), s)),
        scale(k, dot(k, v) * (1.0 - c)),
    )
}

/// One collider's test between `start` and `end` (earlier than any other
/// hit found): the time and point of impact.
fn collide(shape: &ColliderNow, p: &Particle, start: f32, end: f32) -> Option<(f32, Vec3)> {
    match *shape {
        ColliderNow::Plane {
            center,
            x_axis,
            y_axis,
            normal,
            distance,
            half_width2,
            half_height2,
        } => {
            // Inside the rectangle: the offset's parts along the turned x
            // and y axes shorter than half the width and height.
            let within = |q: Vec3| {
                let rel = sub(q, center);
                let a = scale(x_axis, dot(x_axis, rel));
                let b = scale(y_axis, dot(y_axis, rel));
                dot(a, a) < half_width2 && dot(b, b) < half_height2
            };
            let s0 = dot(normal, p.position) - distance;
            if s0.abs() < COLLISION_EPSILON {
                // On the plane already: a hit at the start.
                return within(p.position).then_some((start, p.position));
            }
            let span = end - start;
            let along = dot(normal, p.velocity) * span;
            let mut s1 = along + s0;
            if s1.abs() < COLLISION_EPSILON {
                s1 = 0.0;
            }
            if s1 * s0 >= 0.0 {
                return None;
            }
            let t = (-s0 * span) / along;
            let mut q = add(p.position, scale(p.velocity, t));
            if (dot(normal, q) - distance) * s0 < 0.0 {
                // Went through by rounding: moved back out by `pow(2, −21)`
                // (worked out once, `00ec9b00` with 2.0 and −21.0) × the
                // largest coordinate.
                let m = q[0].abs().max(q[1].abs()).max(q[2].abs());
                let nudge = 4.768_371_6e-7 * m;
                q = if s0 >= 0.0 {
                    add(q, scale(normal, nudge))
                } else {
                    sub(q, scale(normal, nudge))
                };
            }
            within(q).then_some((t + start, q))
        }
        ColliderNow::Sphere {
            center, radius2, ..
        } => {
            let rel = sub(p.position, center);
            let s = dot(rel, rel) - radius2;
            let inside = if -s > COLLISION_EPSILON {
                true
            } else if s <= COLLISION_EPSILON {
                return None;
            } else {
                false
            };
            let toward = -dot(p.velocity, rel);
            if !(inside || toward > 0.0) {
                return None;
            }
            let v2 = dot(p.velocity, p.velocity);
            let t0 = toward / v2;
            let closest = sub(add(p.position, scale(p.velocity, t0)), center);
            let c2 = dot(closest, closest);
            if !(inside || c2 < radius2) {
                return None;
            }
            let root = ((radius2 - c2) / v2).sqrt();
            let t = if inside { root + t0 } else { t0 - root };
            if t < end - start {
                Some((t + start, add(p.position, scale(p.velocity, t))))
            } else {
                None
            }
        }
        ColliderNow::None => None,
    }
}

/// A hit's new velocity: mirrored in the surface and × bounce, or (a plane
/// met at exactly this frame's time while moving along it) just the
/// part along the surface.
fn bounce(shape: &ColliderNow, v: Vec3, at: Vec3, factor: f32, reflect: bool) -> Vec3 {
    match *shape {
        ColliderNow::Plane { normal, .. } => {
            let along = dot(normal, v);
            if reflect || along >= COLLISION_EPSILON {
                scale(sub(v, scale(normal, along + along)), factor)
            } else {
                sub(v, scale(normal, along))
            }
        }
        ColliderNow::Sphere { center, radius, .. } => {
            let n = scale(sub(at, center), 1.0 / radius);
            let d = dot(n, v);
            scale(sub(v, scale(n, d + d)), factor)
        }
        ColliderNow::None => v,
    }
}

/// `00c1c3c0`'s ages: particle n of a stretch (counting from its start) is
/// due once `trunc((t − start) × rate)` reaches n (`FISTP` with the control
/// word set to chop); those due between `last` and `current` (clipped to
/// `stop`), at most [`MAX_EMITTED_AT_ONCE`], each aged `(end − start) − n
/// / rate` (never below 0).
pub fn emission_ages(current: f32, last: f32, start: f32, stop: f32, rate: f32) -> Vec<f32> {
    if rate.is_nan() || rate <= 0.0 || stop <= start || current <= start || last >= stop {
        return Vec::new();
    }
    let end = if current <= stop { current } else { stop };
    let before = if start <= last { last - start } else { 0.0 };
    let done = (before * rate) as i32 as i16;
    let now = ((end - start) * rate) as i32 as i16;
    let mut count = now.wrapping_sub(done) as u16;
    if count > MAX_EMITTED_AT_ONCE {
        count = MAX_EMITTED_AT_ONCE;
    }
    let step = 1.0 / rate;
    (1..=count)
        .map(|k| {
            let n = (i32::from(done as u16) + i32::from(k)) & 0xffff;
            (end - start) - n as f32 * step
        })
        .collect()
}

/// One corner of a drawn particle, in the game's world (`00e6ab40`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Corner {
    pub position: Vec3,
    /// The packed colour as the game sends it (`D3DCOLOR`: bytes B, G, R,
    /// A from `round(c × 255)` each, not clamped).
    pub color: [u8; 4],
    pub uv: [f32; 2],
    /// Which piece of the atlas (`SubTexOffsets`).
    pub sub_texture: f32,
}

/// The camera as the renderer hands it to the particle quads
/// (`SetCameraData` `00e6c780`: `+0x634` right, `+0x640` up), world axes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraBasis {
    pub right: Vec3,
    pub up: Vec3,
    /// The view direction (the culler's camera, `00a96910`'s sort key).
    pub direction: Vec3,
}

impl System {
    /// The quads to draw now (`NiDX9Renderer`'s particle path `00e70140`,
    /// corners `00e6ab40`): per particle, in the order drawn, four corners
    /// at `centre ∓ D`, `centre ∓ E` with `D = s((c + n) R + (c − n) U)`,
    /// `E = s(−(c − n) R + (c + n) U)` (`s = radius × size`, `c`, `n` the
    /// cosine and sine of its angle, `R`, `U` the camera's right and up in
    /// the system's space: `Rᵀ × world` without its scale), uv (0,1),
    /// (1,1), (1,0), (0,0); triangles 0-1-2 and 0-2-3. Blended (not added)
    /// systems are drawn back to front: sorted by `position · view
    /// direction`, farthest first (`NiParticles::OnVisible` `00a9b2a0` →
    /// `00a96910`, the quicksort `00a966c0`).
    pub fn quads(&self, camera: &CameraBasis, sort: bool) -> Vec<Corner> {
        let w = &self.sim_world;
        let r = &w.rotation;
        let to_system = |v: Vec3| {
            [
                r[0][0] * v[0] + r[1][0] * v[1] + r[2][0] * v[2],
                r[0][1] * v[0] + r[1][1] * v[1] + r[2][1] * v[2],
                r[0][2] * v[0] + r[1][2] * v[1] + r[2][2] * v[2],
            ]
        };
        let right = to_system(camera.right);
        let up = to_system(camera.up);
        let active = self.active();
        let mut order: Vec<usize> = (0..active.len()).collect();
        if sort && active.len() > 1 {
            let depth: Vec<f32> = active
                .iter()
                .map(|p| dot(p.position, camera.direction))
                .collect();
            order.sort_by(|&a, &b| depth[b].total_cmp(&depth[a]));
        }
        let angles = self.def.data.has_rotation_angles;
        let colors = self.def.data.has_colors;
        let mut out = Vec::with_capacity(active.len() * 4);
        for &i in &order {
            let p = &active[i];
            let s = p.radius * p.size;
            let (d, e) = if angles {
                let (sn, cs) = (
                    (f64::from(p.rotation_angle)).sin() as f32,
                    (f64::from(p.rotation_angle)).cos() as f32,
                );
                let k2 = s * (cs + sn);
                let k1 = s * (cs - sn);
                (
                    add(scale(right, k2), scale(up, k1)),
                    add(scale(right, -k1), scale(up, k2)),
                )
            } else {
                (scale(add(up, right), s), scale(sub(up, right), s))
            };
            let color = if colors {
                pack_color(p.color)
            } else {
                [255; 4]
            };
            let corner = |position: Vec3, uv: [f32; 2]| Corner {
                position: w.apply_point(position),
                color,
                uv,
                sub_texture: f32::from(p.texture_index),
            };
            out.push(corner(sub(p.position, d), [0.0, 1.0]));
            out.push(corner(sub(p.position, e), [1.0, 1.0]));
            out.push(corner(add(p.position, d), [1.0, 0.0]));
            out.push(corner(add(p.position, e), [0.0, 0.0]));
        }
        out
    }
}

/// The game's colour packing (`00e6ab40`): `((round(a × 255) << 8 |
/// round(r × 255)) << 8 | round(g × 255)) << 8 | round(b × 255)` in 32-bit
/// integers (a value past 1 spills into the next channel), as bytes B, G,
/// R, A.
pub fn pack_color(c: [f32; 4]) -> [u8; 4] {
    let q = |v: f32| round_even((f64::from(v) * 255.0) as f32);
    let packed = ((((q(c[3]) << 8) | q(c[0])) << 8 | q(c[1])) << 8) | q(c[2]);
    (packed as u32).to_le_bytes()
}

/// A sequence's own time `seconds` after it started (`frequency` applied):
/// looping ones wrap through their range, clamped ones play once and
/// hold, reversing ones go back and forth; one that doesn't run holds its
/// start (as `preview::cell`'s placed sequences).
pub fn sequence_time(sequence: &ParticleSequence, seconds: f32, runs: bool) -> f32 {
    let length = sequence.stop - sequence.start;
    if !runs || length <= 0.0 {
        return sequence.start;
    }
    let t = (seconds * sequence.frequency).max(0.0);
    match sequence.cycle {
        0 => sequence.start + t % length,
        1 => {
            let f = t % (length + length);
            sequence.start + if f > length { length + length - f } else { f }
        }
        _ => sequence.start + t.min(length),
    }
}

/// Every scene object a system names (emitter volumes and meshes, gravity,
/// drag and bomb objects, colliders), by block.
fn objects_of(def: &ParticleSystem) -> Vec<ObjectRef> {
    let mut out: Vec<ObjectRef> = Vec::new();
    let mut add = |o: &Option<ObjectRef>| {
        if let Some(o) = o {
            if !out.iter().any(|k| k.block == o.block) {
                out.push(o.clone());
            }
        }
    };
    for m in &def.modifiers {
        match &m.kind {
            ModifierKind::Emitter(e) => match &e.shape {
                EmitterShape::Box { object, .. }
                | EmitterShape::Cylinder { object, .. }
                | EmitterShape::Sphere { object, .. }
                | EmitterShape::Array { object } => add(object),
                EmitterShape::Mesh { meshes, .. } => {
                    for mesh in meshes {
                        add(&Some(mesh.clone()));
                    }
                }
            },
            ModifierKind::Gravity(g) => add(&g.object),
            ModifierKind::Drag(d) => add(&d.object),
            ModifierKind::Bomb(b) => add(&b.object),
            ModifierKind::Colliders(list) => {
                for c in list {
                    add(&c.object);
                }
            }
            _ => {}
        }
    }
    out
}

/// A placed model's particle systems running, with the sequences the
/// placed object plays (which drive controllers under a controller
/// manager) and, given with [`PlacedParticles::set_motion`], those moving
/// the model's nodes (the systems and the objects they name follow them).
pub struct PlacedParticles {
    pub systems: Vec<System>,
    objects: Vec<Vec<ObjectRef>>,
    sequences: Vec<ParticleSequence>,
    /// Index into `sequences`, and whether it runs (else it holds its
    /// start).
    playing: Vec<(usize, bool)>,
    /// Sequences moving nodes, and whether each runs.
    motion: Vec<(Arc<nif::Sequence>, bool)>,
}

/// A node-moving sequence's time `seconds` after it started: looping ones
/// wrap, others play once and hold, held ones stay at their start (as
/// `preview::cell`'s placed pieces).
fn motion_time(s: &nif::Sequence, seconds: f32, runs: bool) -> f32 {
    let length = s.stop - s.start;
    if !runs || length <= 0.0 {
        return s.start;
    }
    if s.looping {
        s.start + seconds.max(0.0) % length
    } else {
        s.start + seconds.clamp(0.0, length)
    }
}

impl PlacedParticles {
    /// `seed` starts the systems' random numbers (each system its own).
    pub fn new(
        systems: &[Arc<ParticleSystem>],
        sequences: Vec<ParticleSequence>,
        playing: Vec<(usize, bool)>,
        seed: u32,
    ) -> PlacedParticles {
        PlacedParticles {
            objects: systems.iter().map(|s| objects_of(s)).collect(),
            systems: systems
                .iter()
                .enumerate()
                .map(|(k, s)| System::new(Arc::clone(s), seed.wrapping_add(k as u32 * 7919)))
                .collect(),
            sequences,
            playing,
            motion: Vec::new(),
        }
    }

    /// The sequences moving the model's nodes as the placed object plays
    /// them (each with whether it runs).
    pub fn set_motion(&mut self, motion: Vec<(Arc<nif::Sequence>, bool)>) {
        self.motion = motion;
    }

    /// One frame, `seconds` after the object appeared, the model placed by
    /// `placed` (model to world, game units).
    pub fn update(&mut self, seconds: f32, placed: &Transform, wind: Vec3) {
        let playing: Vec<(&ParticleSequence, f32)> = self
            .playing
            .iter()
            .filter_map(|&(i, runs)| {
                let s = self.sequences.get(i)?;
                Some((s, sequence_time(s, seconds, runs)))
            })
            .collect();
        let layers: Vec<(&nif::Sequence, f32)> = self
            .motion
            .iter()
            .map(|(s, runs)| (&**s, motion_time(s, seconds, *runs)))
            .collect();
        let posed =
            |nodes: &[(String, Transform)]| placed.then_child(&nif::posed_chain(nodes, &layers));
        for (system, objects) in self.systems.iter_mut().zip(&self.objects) {
            let system_world = posed(&system.def.nodes);
            let object_world = |block: usize| {
                objects
                    .iter()
                    .find(|o| o.block == block)
                    .map(|o| posed(&o.nodes))
            };
            let frame = Frame {
                time: seconds,
                system_world,
                object_world: &object_world,
                playing: &playing,
                wind,
            };
            system.update(&frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn particle(position: Vec3, velocity: Vec3) -> Particle {
        Particle {
            position,
            velocity,
            ..Particle::default()
        }
    }

    fn floor() -> ColliderNow {
        ColliderNow::Plane {
            center: [0.0; 3],
            x_axis: [1.0, 0.0, 0.0],
            y_axis: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            distance: 0.0,
            half_width2: 50.0 * 50.0,
            half_height2: 50.0 * 50.0,
        }
    }

    #[test]
    fn a_falling_particle_meets_the_floor_and_bounces() {
        let p = particle([0.0, 0.0, 10.0], [0.0, 0.0, -100.0]);
        let (t, at) = collide(&floor(), &p, 0.0, 1.0).unwrap();
        assert!((t - 0.1).abs() < 1e-5, "{t}");
        assert!(at[2].abs() < 1e-3);
        // Mirrored and halved.
        let v = bounce(&floor(), p.velocity, at, 0.5, true);
        assert_eq!(v, [0.0, 0.0, 50.0]);
        // Too slow to get there in the time: no hit.
        let slow = particle([0.0, 0.0, 10.0], [0.0, 0.0, -5.0]);
        assert!(collide(&floor(), &slow, 0.0, 1.0).is_none());
        // Outside the rectangle (half sizes 50): no hit.
        let wide = particle([60.0, 0.0, 10.0], [0.0, 0.0, -100.0]);
        assert!(collide(&floor(), &wide, 0.0, 1.0).is_none());
    }

    #[test]
    fn a_particle_meets_a_sphere_from_outside() {
        let ball = ColliderNow::Sphere {
            center: [0.0; 3],
            radius: 10.0,
            radius2: 100.0,
        };
        let p = particle([0.0, 0.0, 30.0], [0.0, 0.0, -40.0]);
        let (t, at) = collide(&ball, &p, 2.0, 3.0).unwrap();
        assert!((t - 2.5).abs() < 1e-5, "{t}");
        assert!((at[2] - 10.0).abs() < 1e-4);
        assert_eq!(bounce(&ball, p.velocity, at, 1.0, true), [0.0, 0.0, 40.0]);
    }

    #[test]
    fn a_timeline_reports_a_switch_stepped_over() {
        // On only between 1.0 and 1.1; frames at 0.9 and 1.2 step over it.
        let interp = BoolInterp {
            value: 2,
            kind: 5,
            keys: vec![(0.0, false), (1.0, true), (1.1, false), (5.0, false)],
            timeline: true,
        };
        let mut state = Timeline::default();
        assert!(!timeline_at(&mut state, &interp, 0.9));
        assert!(timeline_at(&mut state, &interp, 1.2));
        assert!(!timeline_at(&mut state, &interp, 1.3));
        // A plain read misses it.
        assert!(!bool_at(&interp, 1.2));
    }

    #[test]
    fn spawned_directions_turn_from_z_onto_the_parent() {
        let to = [0.0, 3.0, 0.0];
        let v = turn_z_to([0.0, 0.0, 1.0], to);
        assert!((v[1] - 1.0).abs() < 1e-6 && v[0].abs() < 1e-6 && v[2].abs() < 1e-6);
        // Straight down: the matrix −1.
        assert_eq!(
            turn_z_to([0.0, 0.0, 1.0], [0.0, 0.0, -2.0]),
            [0.0, 0.0, -1.0]
        );
        assert_eq!(turn_z_to([0.5, 0.0, 1.0], [0.0, 0.0, 2.0]), [0.5, 0.0, 1.0]);
    }

    #[test]
    fn the_fast_square_root_is_close() {
        for x in [0.01f32, 0.25, 0.5, 1.0] {
            assert!((fast_sqrt(x) - x.sqrt()).abs() < 2e-3 * x.sqrt(), "{x}");
        }
    }

    #[test]
    fn removing_moves_the_last_particle_in() {
        let mut ps = Particles::new(4);
        for i in 0..3 {
            let slot = ps.add().unwrap();
            ps.list[slot].age = i as f32;
        }
        ps.resolve_added();
        assert_eq!(ps.active().len(), 3);
        ps.remove(0);
        assert_eq!(
            ps.active().iter().map(|p| p.age).collect::<Vec<_>>(),
            [2.0, 1.0]
        );
        // Full: no slot.
        for _ in 0..2 {
            assert!(ps.add().is_some());
        }
        assert!(ps.add().is_none());
    }
}
