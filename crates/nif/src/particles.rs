//! Particle systems: `NiParticleSystem` (and Bethesda's strip variant) with
//! its data block, the modifiers that move its particles, the controllers
//! that emit them and switch modifiers on and off, and the objects they
//! name (emitter volumes, gravity fields, colliders).
//!
//! Every layout here was read from the game's own loaders
//! (`LoadBinary`, vtable slot 19 of each class in `FalloutNV.exe`) and
//! checked byte for byte on the game's files: `nvinspect <Data> particles`
//! decodes every particle block of every model and reports any whose bytes
//! aren't consumed exactly. Addresses are the loaders':
//!
//! * `NiParticleSystem` `00c1b3a0`: `NiGeometry` (`00a80bf0`: scene object,
//!   data, skin, material count + names/extra data, active material, a
//!   "needs update" byte), then "world space" u8 and the modifier list.
//! * `NiPSysData` `00c25340` on `NiParticlesData` `00a96cf0` on
//!   `NiGeometryData` `00a67ea0`: with Bethesda version 34 none of the
//!   per-particle arrays are stored, only which exist; the vertex count is
//!   the most particles the system holds.
//! * Modifiers `00c318b0` (name, order, target, active) and each kind's own
//!   fields (see [`ModifierKind`]).
//! * Controllers `00a6d5e0` (`NiTimeController`), `00a571a0`
//!   (`NiPSysModifierCtlr`: interpolator + modifier name), `00c1bdf0`
//!   (`NiPSysEmitterCtlr`: + the "emitter active" interpolator).

use std::collections::BTreeMap;

use crate::anim::FloatKey;
use crate::blocks::{
    self, AlphaProperty, Block, Ctx, MaterialProperty, ShaderProperty, StencilProperty,
    ZBufferProperty,
};
use crate::error::{Error, Result};
use crate::file::Nif;
use crate::math::{Transform, Vec3};
use crate::reader::Reader;

/// A controller's clock (`NiTimeController`): flags, frequency, phase and
/// the key range it cycles through.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeControl {
    /// Bit 0 "app init" (time counts from the first update) else app time,
    /// bits 1–2 the cycle (0 loop, 1 reverse, 2 clamp), bit 3 active, bit 4
    /// play backwards, bit 5 driven by a controller manager, bit 6 "compute
    /// scaled time" (the loader sets it on every controller, `00a6d5e0`),
    /// bit 7 forced update (cleared by the loader).
    pub flags: u16,
    pub frequency: f32,
    pub phase: f32,
    /// The key range (`m_fLoKeyTime`, `m_fHiKeyTime`).
    pub start: f32,
    pub stop: f32,
}

impl TimeControl {
    pub fn active(&self) -> bool {
        self.flags & 0x8 != 0
    }

    /// 0 loop, 1 reverse, 2 clamp.
    pub fn cycle(&self) -> u16 {
        (self.flags >> 1) & 3
    }

    pub fn manager_controlled(&self) -> bool {
        self.flags & 0x20 != 0
    }
}

/// Keys of numbers (`NiFloatData`): their kind (1 linear, 2 quadratic, 3
/// TBC, 5 constant) and the keys, tangents kept for quadratic ones.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloatKeys {
    pub kind: u32,
    pub keys: Vec<FloatKey>,
}

/// `NiFloatInterpolator`: a value, and keys that override it when present.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloatInterp {
    pub value: f32,
    pub keys: FloatKeys,
}

/// `NiBoolInterpolator` / `NiBoolTimelineInterpolator`: a value (0, 1, or
/// 2 "not set") and on/off keys (`NiBoolData`, kind 1 linear or 5 step).
/// The timeline kind (same bytes) reports a switch it stepped over.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoolInterp {
    pub value: u8,
    pub kind: u32,
    pub keys: Vec<(f32, bool)>,
    pub timeline: bool,
}

/// A colour key (`NiColorData`): time and RGBA.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorKey {
    pub time: f32,
    pub color: [f32; 4],
}

/// The particle data block: how many particles fit and which values each
/// particle carries (the game allocates only those).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParticleData {
    pub type_name: String,
    /// The most particles alive at once (the stored vertex count).
    pub max_particles: u16,
    pub has_colors: bool,
    pub has_radii: bool,
    pub has_sizes: bool,
    pub has_rotations: bool,
    pub has_rotation_angles: bool,
    pub has_rotation_axes: bool,
    pub has_texture_indices: bool,
    pub has_rotation_speeds: bool,
    /// Pieces of an atlas texture a particle can show: (u, width, v,
    /// height), as the vertex shader's `SubTexOffsets` use them.
    pub subtexture_offsets: Vec<[f32; 4]>,
    /// `BSStripPSysData`: points per strip, cap sizes, depth pre-pass.
    pub strip: Option<StripData>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripData {
    pub max_points: i16,
    pub start_cap: f32,
    pub end_cap: f32,
    pub z_prepass: bool,
}

/// A scene object a modifier or controller names (emitter volume, gravity
/// or bomb object, collider): its block, name and the chain of nodes from
/// the top of the file down to it (each with its own transform, the top
/// node's left out as for placed models), so its place can be worked out
/// with or without animation.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectRef {
    pub block: usize,
    pub name: String,
    pub nodes: Vec<(String, Transform)>,
}

impl ObjectRef {
    /// Its transform in the model's space, from the stored transforms.
    pub fn transform(&self) -> Transform {
        self.nodes
            .iter()
            .fold(Transform::IDENTITY, |w, (_, t)| w.then_child(t))
    }
}

/// What every modifier has (`NiPSysModifier`, loader `00c318b0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub block: usize,
    pub name: String,
    /// Its place in the update (`NiPSysModifierOrder`: 0 kill old
    /// particles, 1000 emitters, 2000 spawn, 3000 general, 4000 forces, 5000
    /// colliders, 6000 position update, 7000 bound update, ...). The game
    /// keeps the file's order of the list; this is stored but not sorted on.
    pub order: u32,
    pub active: bool,
    pub kind: ModifierKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModifierKind {
    /// `NiPSysAgeDeathModifier` (`00c2f200`): spawn on death, the spawn
    /// modifier.
    AgeDeath {
        spawn_on_death: bool,
        spawn: Option<usize>,
    },
    /// `NiPSysBoundUpdateModifier` (`00c2e370`).
    BoundUpdate {
        update_skip: u16,
    },
    /// `NiPSysPositionModifier`: no fields.
    Position,
    Emitter(Emitter),
    Spawn(Spawn),
    Gravity(Gravity),
    Drag(Drag),
    Bomb(Bomb),
    GrowFade(GrowFade),
    Rotation(Rotation),
    /// `NiPSysColorModifier`: colour keys over the particle's life.
    Color {
        kind: u32,
        keys: Vec<ColorKey>,
    },
    SimpleColor(SimpleColor),
    /// `NiPSysColliderManager`: the colliders, first one first.
    Colliders(Vec<Collider>),
    /// `BSWindModifier` (`00c467c0`): strength.
    Wind {
        strength: f32,
    },
    /// `BSParentVelocityModifier` (`00c63900`): damping.
    ParentVelocity {
        damping: f32,
    },
    /// `BSPSysStripUpdateModifier` (`00c5ff60`): update delta time.
    StripUpdate {
        update_delta: f32,
    },
    /// A modifier type not read here.
    Other(String),
}

/// `NiPSysEmitter` (`00c223b0`): how particles start.
#[derive(Debug, Clone, PartialEq)]
pub struct Emitter {
    pub speed: f32,
    pub speed_variation: f32,
    /// Radians from the emitter's +Z axis.
    pub declination: f32,
    pub declination_variation: f32,
    /// Radians about +Z.
    pub planar_angle: f32,
    pub planar_angle_variation: f32,
    pub color: [f32; 4],
    pub radius: f32,
    pub radius_variation: f32,
    pub life_span: f32,
    pub life_span_variation: f32,
    pub shape: EmitterShape,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmitterShape {
    /// `NiPSysBoxEmitter` (`00c203b0`): width (x), height (y), depth (z).
    Box {
        object: Option<ObjectRef>,
        width: f32,
        height: f32,
        depth: f32,
    },
    /// `NiPSysCylinderEmitter` (`00c1fc40`): radius, height (along z).
    Cylinder {
        object: Option<ObjectRef>,
        radius: f32,
        height: f32,
    },
    /// `NiPSysSphereEmitter` (`00c20040`).
    Sphere {
        object: Option<ObjectRef>,
        radius: f32,
    },
    /// `BSPSysArrayEmitter` (`00c61ef0`): a volume emitter with no fields
    /// of its own.
    Array { object: Option<ObjectRef> },
    /// `NiPSysMeshEmitter` (`00c1e850`): meshes, initial velocity type
    /// (0 along the surface's normal, 1 random, 2 the axis), emission type
    /// (0 vertices, 1 triangle centres, 2 edge middles, 3 on triangles, 4
    /// on edges), emission axis. Each mesh's own vertices with it (none for
    /// a mesh that can't be read).
    Mesh {
        meshes: Vec<ObjectRef>,
        geometry: Vec<Option<MeshGeometry>>,
        velocity_type: u32,
        emission_type: u32,
        axis: Vec3,
    },
}

/// A mesh emitter's mesh: its vertices, normals and triangles in its own
/// space (strips turned into triangles), and whether it's skinned.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshGeometry {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub triangles: Vec<[u16; 3]>,
    pub skinned: bool,
}

/// `NiPSysSpawnModifier` (`00c247f0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Spawn {
    pub generations: u16,
    pub percentage: f32,
    pub min: u16,
    pub max: u16,
    pub speed_variation: f32,
    pub direction_variation: f32,
    pub life_span: f32,
    pub life_span_variation: f32,
}

/// `NiPSysGravityModifier` (`00c236b0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Gravity {
    pub object: Option<ObjectRef>,
    pub axis: Vec3,
    pub decay: f32,
    pub strength: f32,
    /// 0 planar (along the axis), 1 spherical (toward the object).
    pub force_type: u32,
    pub turbulence: f32,
    pub turbulence_scale: f32,
    /// Read for Bethesda versions above 20 (`+0x3c`).
    pub world_aligned: bool,
}

/// `NiPSysDragModifier` (`00c2c450`).
#[derive(Debug, Clone, PartialEq)]
pub struct Drag {
    pub object: Option<ObjectRef>,
    pub axis: Vec3,
    pub percentage: f32,
    pub range: f32,
    pub range_falloff: f32,
}

/// `NiPSysBombModifier` (`00c2eb80`).
#[derive(Debug, Clone, PartialEq)]
pub struct Bomb {
    pub object: Option<ObjectRef>,
    pub axis: Vec3,
    pub decay: f32,
    pub delta_v: f32,
    pub decay_type: u32,
    pub symmetry_type: u32,
}

/// `NiPSysGrowFadeModifier` (`00c2a530`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrowFade {
    pub grow_time: f32,
    pub grow_generation: u16,
    pub fade_time: f32,
    pub fade_generation: u16,
    /// Read for Bethesda versions above 16.
    pub base_scale: f32,
}

/// `NiPSysRotationModifier` (`00c20ae0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Rotation {
    pub speed: f32,
    pub speed_variation: f32,
    pub angle: f32,
    pub angle_variation: f32,
    pub random_speed_sign: bool,
    pub random_axis: bool,
    pub axis: Vec3,
}

/// `BSPSysSimpleColorModifier` (`00c60850`): three colours over the
/// particle's life and an alpha fade in and out. Fractions of the life:
/// `color1_end`, `color2_start`, `color2_end`, `color3_start` (the file's
/// order; the update `00c602e0` blends colour 1 to 2 between the first
/// two and 2 to 3 between the last two).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleColor {
    pub fade_in: f32,
    pub fade_out: f32,
    pub color1_end: f32,
    pub color2_start: f32,
    pub color2_end: f32,
    pub color3_start: f32,
    pub colors: [[f32; 4]; 3],
}

/// A collider (`NiPSysCollider` `00c32840` and its kinds).
#[derive(Debug, Clone, PartialEq)]
pub struct Collider {
    pub bounce: f32,
    pub spawn_on_collide: bool,
    pub die_on_collide: bool,
    pub spawn: Option<usize>,
    pub object: Option<ObjectRef>,
    pub shape: ColliderShape,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColliderShape {
    /// `NiPSysPlanarCollider` (`00c29770`): a rectangle.
    Plane {
        width: f32,
        height: f32,
        x_axis: Vec3,
        y_axis: Vec3,
    },
    /// `NiPSysSphericalCollider` (`00c28070`).
    Sphere {
        radius: f32,
    },
    Other(String),
}

/// A controller on the particle system.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub block: usize,
    pub type_name: String,
    pub time: TimeControl,
    pub kind: ControllerKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControllerKind {
    /// `NiPSysEmitterCtlr`: births per second and whether the emitter is
    /// on, for the emitter named `modifier`. `BSPSysMultiTargetEmitterCtlr`
    /// adds the most emitters and its master particle system.
    Emitter {
        modifier: String,
        birth_rate: Option<FloatInterp>,
        active: Option<BoolInterp>,
        multi_target: Option<(u16, i32)>,
    },
    /// `NiPSysUpdateCtlr`: runs the system's update.
    Update,
    /// `NiPSysModifierActiveCtlr`: switches a modifier on and off.
    ModifierActive {
        modifier: String,
        value: Option<BoolInterp>,
    },
    /// One of the `NiPSys…Ctlr` that set a modifier's number (emitter
    /// speed, gravity strength...): the type says which.
    ModifierFloat {
        modifier: String,
        value: Option<FloatInterp>,
    },
    /// `NiPSysResetOnLoopCtlr`: clears the system when its time wraps.
    ResetOnLoop,
    Other,
}

/// A particle system as placed in its model.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleSystem {
    pub block: usize,
    pub type_name: String,
    pub name: String,
    /// The nodes from the top of the file down to the system itself, each
    /// with its local transform (the top node's left out for placed
    /// models), as [`crate::Mesh::nodes`].
    pub nodes: Vec<(String, Transform)>,
    /// Simulated in world space (the system's own world transform is then
    /// reduced to its scale, `00c1add0`) or in its own.
    pub world_space: bool,
    pub data: ParticleData,
    /// In the file's (the update's) order.
    pub modifiers: Vec<Modifier>,
    /// Its controller chain, first first.
    pub controllers: Vec<Controller>,
    /// Drawing: the diffuse texture (shader's or texturing property's),
    /// the shader, material, alpha, depth and stencil properties.
    pub texture: Option<String>,
    pub shader: Option<ShaderProperty>,
    pub material: Option<MaterialProperty>,
    pub alpha: Option<AlphaProperty>,
    pub zbuffer: Option<ZBufferProperty>,
    pub stencil: Option<StencilProperty>,
    pub property_types: Vec<String>,
}

impl ParticleSystem {
    /// From the system's own space to the model's (stored transforms).
    pub fn transform(&self) -> Transform {
        self.nodes
            .iter()
            .fold(Transform::IDENTITY, |w, (_, t)| w.then_child(t))
    }

    /// The modifier at this place in [`Self::modifiers`] by block number.
    pub fn modifier_index(&self, block: usize) -> Option<usize> {
        self.modifiers.iter().position(|m| m.block == block)
    }
}

/// A model's animation sequence (`NiControllerSequence`) as far as it
/// drives particle controllers: its time range and cycle, and for each
/// controlled particle controller the interpolator it hands it.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleSequence {
    pub name: String,
    pub start: f32,
    pub stop: f32,
    /// 0 loop, 1 reverse, 2 clamp.
    pub cycle: u32,
    pub frequency: f32,
    pub tracks: Vec<ParticleTrack>,
    /// Nodes the sequence moves with interpolators `nif::anim` doesn't
    /// read (a `NiTransformController` driven by a `NiPathInterpolator`:
    /// the dust whirlwind's mover), by name.
    pub unread_movers: Vec<String>,
}

/// One controlled block of a sequence that names a particle controller:
/// the particle system (`node`), the controller's type and ID (the
/// modifier's name), the interpolator's ID and the interpolator.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleTrack {
    pub node: String,
    pub controller_type: String,
    pub controller_id: String,
    pub interpolator_id: String,
    pub value: TrackValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrackValue {
    Float(FloatInterp),
    Bool(BoolInterp),
}

/// Any decoded particle block, for checking layouts.
#[derive(Debug, Clone, PartialEq)]
pub enum ParticleBlock {
    System {
        world_space: bool,
        modifiers: Vec<i32>,
    },
    Data(ParticleData),
    Modifier(Modifier),
    Collider(Collider),
    Controller(Controller),
    FloatInterp(FloatInterp),
    BoolInterp(BoolInterp),
    MasterSystem {
        max_emitters: u16,
        systems: Vec<i32>,
    },
}

/// The block types this module reads.
pub fn is_particle_type(type_name: &str) -> bool {
    type_name.starts_with("NiPSys")
        || type_name.starts_with("BSPSys")
        || matches!(
            type_name,
            "NiParticleSystem"
                | "BSStripParticleSystem"
                | "BSStripPSysData"
                | "BSMasterParticleSystem"
                | "BSWindModifier"
                | "BSParentVelocityModifier"
        )
}

const MODIFIER_FLOAT_CONTROLLERS: &[&str] = &[
    "NiPSysEmitterSpeedCtlr",
    "NiPSysEmitterInitialRadiusCtlr",
    "NiPSysEmitterLifeSpanCtlr",
    "NiPSysEmitterDeclinationCtlr",
    "NiPSysEmitterDeclinationVarCtlr",
    "NiPSysEmitterPlanarAngleCtlr",
    "NiPSysEmitterPlanarAngleVarCtlr",
    "NiPSysGravityStrengthCtlr",
    "NiPSysInitialRotSpeedCtlr",
    "NiPSysInitialRotSpeedVarCtlr",
    "NiPSysInitialRotAngleCtlr",
    "NiPSysInitialRotAngleVarCtlr",
    "NiPSysFieldMagnitudeCtlr",
    "NiPSysFieldAttenuationCtlr",
    "NiPSysFieldMaxDistanceCtlr",
    "NiPSysAirFieldAirFrictionCtlr",
    "NiPSysAirFieldInheritVelocityCtlr",
    "NiPSysAirFieldSpreadCtlr",
];

fn string_of(ctx: &Ctx, r: &mut Reader, what: &str) -> Result<String> {
    let index = r.i32(what)?;
    Ok(usize::try_from(index)
        .ok()
        .and_then(|i| ctx.strings.get(i))
        .cloned()
        .unwrap_or_default())
}

fn vec4(r: &mut Reader, what: &str) -> Result<[f32; 4]> {
    Ok([r.f32(what)?, r.f32(what)?, r.f32(what)?, r.f32(what)?])
}

/// The bytes left over after a block was decoded: the layout is wrong.
fn exact(r: &Reader, type_name: &str) -> Result<()> {
    if r.remaining() == 0 {
        Ok(())
    } else {
        Err(Error::Malformed {
            offset: r.offset(),
            reason: format!(
                "{type_name} has {} bytes more than its fields",
                r.remaining()
            ),
        })
    }
}

/// `NiTimeController` (`00a6d5e0`): next, flags, frequency, phase, start,
/// stop, target. Returns the next controller and the clock.
fn read_time_control(r: &mut Reader) -> Result<(i32, TimeControl)> {
    let next = r.i32("the next controller")?;
    let flags = r.u16("controller flags")?;
    let frequency = r.f32("the frequency")?;
    let phase = r.f32("the phase")?;
    let start = r.f32("the start time")?;
    let stop = r.f32("the stop time")?;
    r.i32("the controller's target")?;
    // The loader clears "forced update" and sets "compute scaled time".
    let flags = (flags & 0xff7f) | 0x40;
    Ok((
        next,
        TimeControl {
            flags,
            frequency,
            phase,
            start,
            stop,
        },
    ))
}

impl Nif {
    /// Every particle system reached from the top node, as the game places
    /// the model (the top node's own transform left out) or, with
    /// `apply_root`, as model viewers show the file. Hidden branches and
    /// editor markers are skipped as for meshes; `BSMasterParticleSystem`
    /// nodes are walked like ordinary nodes.
    pub fn particle_systems(&self, apply_root: bool) -> Result<Vec<ParticleSystem>> {
        let parents = self.node_parents();
        let mut out = Vec::new();
        let mut visited = vec![false; self.blocks().len()];
        for &root in self.roots() {
            self.particle_walk(
                root,
                &[],
                &[],
                0,
                apply_root,
                &parents,
                &mut visited,
                &mut out,
            )?;
        }
        Ok(out)
    }

    /// The file's animation sequences, with the interpolators they hand
    /// particle controllers (`NiControllerSequence`, 20.2.0.7: name, count,
    /// grow-by, then per controlled block interpolator, controller,
    /// priority u8, node name, property type, controller type, controller
    /// ID, interpolator ID; then weight, text keys, cycle, frequency, start,
    /// stop, manager, accumulation root).
    pub fn particle_sequences(&self) -> Result<Vec<ParticleSequence>> {
        let ctx = self.ctx();
        let mut out = Vec::new();
        for index in 0..self.blocks().len() {
            if self.block_type(index) != "NiControllerSequence" {
                continue;
            }
            let mut r = self.reader(index);
            let name = string_of(&ctx, &mut r, "the sequence name")?;
            let count = r.u32("the controlled block count")? as usize;
            r.u32("the array grow-by")?;
            let mut blocks = Vec::with_capacity(count.min(1024));
            for _ in 0..count {
                let interpolator = r.i32("an interpolator")?;
                r.i32("a controller")?;
                r.u8("a priority")?;
                let node = string_of(&ctx, &mut r, "a node name")?;
                string_of(&ctx, &mut r, "a property type")?;
                let controller_type = string_of(&ctx, &mut r, "a controller type")?;
                let controller_id = string_of(&ctx, &mut r, "a controller ID")?;
                let interpolator_id = string_of(&ctx, &mut r, "an interpolator ID")?;
                blocks.push((
                    interpolator,
                    node,
                    controller_type,
                    controller_id,
                    interpolator_id,
                ));
            }
            r.f32("the weight")?;
            r.i32("the text keys")?;
            let cycle = r.u32("the cycle type")?;
            let frequency = r.f32("the frequency")?;
            let start = r.f32("the start time")?;
            let stop = r.f32("the stop time")?;
            let mut tracks = Vec::new();
            let mut unread_movers = Vec::new();
            for (interpolator, node, controller_type, controller_id, interpolator_id) in blocks {
                let Some(i) = self.reference(interpolator) else {
                    continue;
                };
                if controller_type == "NiTransformController"
                    && !matches!(
                        self.block_type(i),
                        "NiTransformInterpolator" | "NiBSplineCompTransformInterpolator"
                    )
                {
                    unread_movers.push(node);
                    continue;
                }
                if !is_particle_type(&controller_type) {
                    continue;
                }
                let value = match self.block_type(i) {
                    "NiFloatInterpolator" => TrackValue::Float(self.float_interp(i)?),
                    "NiBoolInterpolator" | "NiBoolTimelineInterpolator" => {
                        TrackValue::Bool(self.bool_interp(i)?)
                    }
                    _ => continue,
                };
                tracks.push(ParticleTrack {
                    node,
                    controller_type,
                    controller_id,
                    interpolator_id,
                    value,
                });
            }
            out.push(ParticleSequence {
                name,
                start,
                stop,
                cycle,
                frequency,
                tracks,
                unread_movers,
            });
        }
        Ok(out)
    }

    /// Each node-like block's parent, from the nodes' child lists.
    fn node_parents(&self) -> BTreeMap<usize, usize> {
        let mut parents = BTreeMap::new();
        for i in 0..self.blocks().len() {
            if let Ok(Some(node)) = self.node_like(i) {
                for &child in &node.children {
                    if let Some(c) = self.reference(child) {
                        parents.entry(c).or_insert(i);
                    }
                }
            }
        }
        parents
    }

    /// A node, or a `BSMasterParticleSystem` read as the node it is.
    fn node_like(&self, index: usize) -> Result<Option<blocks::Node>> {
        let type_name = self.block_type(index);
        if type_name == "BSMasterParticleSystem" {
            let ctx = self.ctx();
            let mut r = self.reader(index);
            return blocks::read_node(&mut r, &ctx, "NiNode").map(Some);
        }
        Ok(match self.block(index)? {
            Block::Node(n) => Some(n),
            _ => None,
        })
    }

    /// The chain of nodes from the top of the file down to `block` (with
    /// the block's own transform last), each with its local transform.
    fn chain_to(
        &self,
        block: usize,
        parents: &BTreeMap<usize, usize>,
        apply_root: bool,
    ) -> Vec<(String, Transform)> {
        let mut chain = Vec::new();
        let mut at = Some(block);
        let mut guard = 0;
        while let Some(i) = at {
            guard += 1;
            if guard > 128 {
                break;
            }
            let (name, transform) = self.av_of(i).unwrap_or_default();
            chain.push((i, name, transform));
            at = parents.get(&i).copied();
        }
        chain.reverse();
        let top = chain.first().map(|c| c.0);
        let is_root = |i: usize| self.roots().iter().any(|&r| r >= 0 && r as usize == i);
        chain
            .into_iter()
            .map(|(i, name, t)| {
                if Some(i) == top && is_root(i) && !apply_root {
                    (name, Transform::IDENTITY)
                } else {
                    (name, t)
                }
            })
            .collect()
    }

    /// A scene object's name and local transform.
    fn av_of(&self, index: usize) -> Option<(String, Transform)> {
        if let Ok(Some(node)) = self.node_like(index) {
            return Some((node.av.net.name, node.av.transform));
        }
        let ctx = self.ctx();
        let mut r = self.reader(index);
        let av = blocks::read_av_object(&mut r, &ctx).ok()?;
        Some((av.net.name, av.transform))
    }

    fn object_ref(
        &self,
        reference: i32,
        parents: &BTreeMap<usize, usize>,
        apply_root: bool,
    ) -> Option<ObjectRef> {
        let block = self.reference(reference)?;
        let (name, _) = self.av_of(block)?;
        Some(ObjectRef {
            block,
            name,
            nodes: self.chain_to(block, parents, apply_root),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn particle_walk(
        &self,
        reference: i32,
        path: &[(String, Transform)],
        inherited: &[i32],
        depth: usize,
        apply_root: bool,
        parents: &BTreeMap<usize, usize>,
        visited: &mut [bool],
        out: &mut Vec<ParticleSystem>,
    ) -> Result<()> {
        let Some(index) = self.reference(reference) else {
            return Ok(());
        };
        if visited[index] || depth > 64 {
            return Ok(());
        }
        visited[index] = true;
        let type_name = self.block_type(index);
        if type_name == "RootCollisionNode" {
            return Ok(());
        }
        let invisible = |av: &blocks::AvObject| {
            av.is_hidden() || av.net.name.to_ascii_lowercase().starts_with("editormarker")
        };
        if let Some(node) = self.node_like(index)? {
            if invisible(&node.av) {
                return Ok(());
            }
            let local = if depth == 0 && !apply_root {
                Transform::IDENTITY
            } else {
                node.av.transform
            };
            let mut path = path.to_vec();
            path.push((node.av.net.name.clone(), local));
            let properties = [inherited, &node.av.properties].concat();
            let children: Vec<i32> = match node.active_child {
                Some(k) => node.children.get(k).copied().into_iter().collect(),
                None => node.children,
            };
            for child in children {
                self.particle_walk(
                    child,
                    &path,
                    &properties,
                    depth + 1,
                    apply_root,
                    parents,
                    visited,
                    out,
                )?;
            }
            return Ok(());
        }
        if !matches!(type_name, "NiParticleSystem" | "BSStripParticleSystem") {
            return Ok(());
        }
        let ctx = self.ctx();
        let mut r = self.reader(index);
        let av = blocks::read_av_object(&mut r, &ctx)?;
        if invisible(&av) {
            return Ok(());
        }
        let data_ref = r.i32("the particle data")?;
        let (world_space, modifier_refs) = read_system_rest(&mut r, &ctx)?;
        let local = if depth == 0 && !apply_root {
            Transform::IDENTITY
        } else {
            av.transform
        };
        let mut nodes = path.to_vec();
        nodes.push((av.net.name.clone(), local));
        let properties = [inherited, &av.properties].concat();

        let data = match self.reference(data_ref) {
            Some(d) => self.particle_data(d)?,
            None => ParticleData::default(),
        };
        let mut modifiers = Vec::new();
        for &m in &modifier_refs {
            if let Some(m) = self.reference(m) {
                modifiers.push(self.modifier(m, parents, apply_root)?);
            }
        }
        let mut controllers = Vec::new();
        let mut next = av.net.controller;
        let mut seen = 0;
        while let Some(c) = self.reference(next) {
            seen += 1;
            if seen > 64 {
                break;
            }
            let (controller, after) = self.controller(c)?;
            controllers.push(controller);
            next = after;
        }
        let mut system = ParticleSystem {
            block: index,
            type_name: type_name.to_string(),
            name: av.net.name,
            nodes,
            world_space,
            data,
            modifiers,
            controllers,
            texture: None,
            shader: None,
            material: None,
            alpha: None,
            zbuffer: None,
            stencil: None,
            property_types: Vec::new(),
        };
        self.particle_properties(&properties, &mut system);
        out.push(system);
        Ok(())
    }

    /// Properties on the way down apply; the system's own come last.
    fn particle_properties(&self, properties: &[i32], system: &mut ParticleSystem) {
        let mut texturing_base = None;
        for &p in properties {
            let Some(p) = self.reference(p) else { continue };
            system.property_types.push(self.block_type(p).to_string());
            let Ok(block) = self.block(p) else { continue };
            match block {
                Block::Shader(shader) => {
                    if let Some(f) = shader.file_name.clone().filter(|f| !f.is_empty()) {
                        system.texture = Some(f);
                    } else if shader.lit {
                        if let Some(Ok(Block::TextureSet(set))) =
                            self.reference(shader.texture_set).map(|t| self.block(t))
                        {
                            system.texture =
                                set.textures.first().cloned().filter(|t| !t.is_empty());
                        }
                    }
                    system.shader = Some(shader);
                }
                Block::Texturing(t) => {
                    if let Some(Ok(Block::SourceTexture(s))) =
                        self.reference(t.base_texture).map(|i| self.block(i))
                    {
                        texturing_base = Some(s.file_name).filter(|f| !f.is_empty());
                    }
                }
                Block::Material(m) => system.material = Some(m),
                Block::Alpha(a) => system.alpha = Some(a),
                Block::ZBuffer(z) => system.zbuffer = Some(z),
                Block::Stencil(s) => system.stencil = Some(s),
                _ => {}
            }
        }
        if system.texture.is_none() {
            system.texture = texturing_base;
        }
    }

    /// Decodes any particle block, checking that its bytes are used up
    /// exactly (for surveys of the game's files).
    pub fn particle_block(&self, index: usize) -> Result<ParticleBlock> {
        let type_name = self.block_type(index).to_string();
        let ctx = self.ctx();
        let wrap = |e: Error| Error::InBlock {
            index,
            type_name: type_name.clone(),
            source: Box::new(e),
        };
        let parents = BTreeMap::new();
        match type_name.as_str() {
            "NiParticleSystem" | "BSStripParticleSystem" => {
                let mut r = self.reader(index);
                blocks::read_av_object(&mut r, &ctx).map_err(wrap)?;
                r.i32("the particle data").map_err(wrap)?;
                let (world_space, modifiers) = read_system_rest(&mut r, &ctx).map_err(wrap)?;
                exact(&r, &type_name).map_err(wrap)?;
                Ok(ParticleBlock::System {
                    world_space,
                    modifiers,
                })
            }
            "NiPSysData" | "BSStripPSysData" => self.particle_data(index).map(ParticleBlock::Data),
            "BSMasterParticleSystem" => {
                let mut r = self.reader(index);
                blocks::read_node(&mut r, &ctx, "NiNode").map_err(wrap)?;
                let max_emitters = r.u16("the most emitters").map_err(wrap)?;
                let systems = r.ref_list("particle systems").map_err(wrap)?;
                exact(&r, &type_name).map_err(wrap)?;
                Ok(ParticleBlock::MasterSystem {
                    max_emitters,
                    systems,
                })
            }
            "NiPSysPlanarCollider" | "NiPSysSphericalCollider" => self
                .collider(index, &parents, false)
                .map(|(c, _)| ParticleBlock::Collider(c)),
            "NiFloatInterpolator" => self.float_interp(index).map(ParticleBlock::FloatInterp),
            "NiBoolInterpolator" | "NiBoolTimelineInterpolator" => {
                self.bool_interp(index).map(ParticleBlock::BoolInterp)
            }
            t if t.ends_with("Ctlr") => self
                .controller(index)
                .map(|(c, _)| ParticleBlock::Controller(c)),
            _ => self
                .modifier(index, &parents, false)
                .map(ParticleBlock::Modifier),
        }
    }

    /// `NiPSysData` / `BSStripPSysData`.
    pub fn particle_data(&self, index: usize) -> Result<ParticleData> {
        let type_name = self.block_type(index).to_string();
        let ctx = self.ctx();
        let mut r = self.reader(index);
        let decoded = (|| -> Result<ParticleData> {
            // NiGeometryData (00a67ea0): with Bethesda version 34 the vertex
            // count is the most particles and no arrays follow.
            r.i32("a group ID")?;
            let max_particles = r.u16("the most particles")?;
            r.u8("keep flags")?;
            r.u8("compress flags")?;
            r.u8("the has-vertices flag")?;
            let data_flags = r.u16("geometry flags")?;
            if ctx.layout.material_crc {
                r.u32("a material CRC")?;
            }
            r.u8("the has-normals flag")?;
            r.take(16, "the bounding sphere")?;
            let has_colors = r.bool("the has-colors flag")?;
            let _uv_sets = data_flags & 1;
            r.u16("consistency flags")?;
            r.i32("an additional data reference")?;
            // NiParticlesData (00a96cf0).
            let has_radii = r.bool("the has-radii flag")?;
            r.u16("the active particle count")?;
            let has_sizes = r.bool("the has-sizes flag")?;
            let has_rotations = r.bool("the has-rotations flag")?;
            let has_rotation_angles = r.bool("the has-rotation-angles flag")?;
            let has_rotation_axes = r.bool("the has-rotation-axes flag")?;
            let has_texture_indices = r.bool("the has-texture-indices flag")?;
            let count = usize::from(r.u8("the sub-texture count")?);
            let subtexture_offsets = r.counted(count, 16, "sub-texture offsets", |r| {
                vec4(r, "a sub-texture")
            })?;
            // NiPSysData (00c25340).
            let has_rotation_speeds = r.bool("the has-rotation-speeds flag")?;
            let strip = if type_name == "BSStripPSysData" {
                Some(StripData {
                    max_points: r.u16("the most strip points")? as i16,
                    start_cap: r.f32("the start cap size")?,
                    end_cap: r.f32("the end cap size")?,
                    z_prepass: r.bool("the depth pre-pass flag")?,
                })
            } else {
                None
            };
            exact(&r, &type_name)?;
            Ok(ParticleData {
                type_name: type_name.clone(),
                max_particles,
                has_colors,
                has_radii,
                has_sizes,
                has_rotations,
                has_rotation_angles,
                has_rotation_axes,
                has_texture_indices,
                has_rotation_speeds,
                subtexture_offsets,
                strip,
            })
        })();
        decoded.map_err(|e| Error::InBlock {
            index,
            type_name,
            source: Box::new(e),
        })
    }

    fn modifier(
        &self,
        index: usize,
        parents: &BTreeMap<usize, usize>,
        apply_root: bool,
    ) -> Result<Modifier> {
        let type_name = self.block_type(index).to_string();
        let ctx = self.ctx();
        let mut r = self.reader(index);
        let object = |reference: i32| self.object_ref(reference, parents, apply_root);
        let decoded = (|| -> Result<Modifier> {
            let name = string_of(&ctx, &mut r, "the modifier's name")?;
            let order = r.u32("the modifier's order")?;
            r.i32("the modifier's target")?;
            let active = r.bool("the active flag")?;
            let kind = match type_name.as_str() {
                "NiPSysAgeDeathModifier" => {
                    let spawn_on_death = r.bool("spawn on death")?;
                    let spawn = self.reference(r.i32("the spawn modifier")?);
                    ModifierKind::AgeDeath {
                        spawn_on_death,
                        spawn,
                    }
                }
                "NiPSysBoundUpdateModifier" => ModifierKind::BoundUpdate {
                    update_skip: r.u16("the update skip")?,
                },
                "NiPSysPositionModifier" => ModifierKind::Position,
                "NiPSysBoxEmitter"
                | "NiPSysCylinderEmitter"
                | "NiPSysSphereEmitter"
                | "BSPSysArrayEmitter"
                | "NiPSysMeshEmitter" => {
                    let mut e = Emitter {
                        speed: r.f32("the speed")?,
                        speed_variation: r.f32("the speed variation")?,
                        declination: r.f32("the declination")?,
                        declination_variation: r.f32("the declination variation")?,
                        planar_angle: r.f32("the planar angle")?,
                        planar_angle_variation: r.f32("the planar angle variation")?,
                        color: vec4(&mut r, "the initial colour")?,
                        radius: r.f32("the initial radius")?,
                        radius_variation: r.f32("the radius variation")?,
                        life_span: r.f32("the life span")?,
                        life_span_variation: r.f32("the life span variation")?,
                        shape: EmitterShape::Array { object: None },
                    };
                    e.shape = if type_name == "NiPSysMeshEmitter" {
                        let meshes = r.ref_list("emitter meshes")?;
                        let velocity_type = r.u32("the initial velocity type")?;
                        let emission_type = r.u32("the emission type")?;
                        let axis = r.vec3("the emission axis")?;
                        let meshes: Vec<ObjectRef> =
                            meshes.iter().filter_map(|&m| object(m)).collect();
                        EmitterShape::Mesh {
                            geometry: meshes.iter().map(|m| self.mesh_geometry(m.block)).collect(),
                            meshes,
                            velocity_type,
                            emission_type,
                            axis,
                        }
                    } else {
                        let o = object(r.i32("the emitter object")?);
                        match type_name.as_str() {
                            "NiPSysBoxEmitter" => EmitterShape::Box {
                                object: o,
                                width: r.f32("the width")?,
                                height: r.f32("the height")?,
                                depth: r.f32("the depth")?,
                            },
                            "NiPSysCylinderEmitter" => EmitterShape::Cylinder {
                                object: o,
                                radius: r.f32("the radius")?,
                                height: r.f32("the height")?,
                            },
                            "NiPSysSphereEmitter" => EmitterShape::Sphere {
                                object: o,
                                radius: r.f32("the radius")?,
                            },
                            _ => EmitterShape::Array { object: o },
                        }
                    };
                    ModifierKind::Emitter(e)
                }
                "NiPSysSpawnModifier" => ModifierKind::Spawn(Spawn {
                    generations: r.u16("the spawn generations")?,
                    percentage: r.f32("the spawn percentage")?,
                    min: r.u16("the least to spawn")?,
                    max: r.u16("the most to spawn")?,
                    speed_variation: r.f32("the spawn speed variation")?,
                    direction_variation: r.f32("the spawn direction variation")?,
                    life_span: r.f32("the spawn life span")?,
                    life_span_variation: r.f32("the spawn life span variation")?,
                }),
                "NiPSysGravityModifier" => {
                    let o = object(r.i32("the gravity object")?);
                    let axis = r.vec3("the gravity axis")?;
                    let decay = r.f32("the decay")?;
                    let strength = r.f32("the strength")?;
                    let force_type = r.u32("the force type")?;
                    let turbulence = r.f32("the turbulence")?;
                    let turbulence_scale = r.f32("the turbulence scale")?;
                    let world_aligned = if ctx.bs_version > 20 {
                        r.bool("the world-aligned flag")?
                    } else {
                        false
                    };
                    ModifierKind::Gravity(Gravity {
                        object: o,
                        axis,
                        decay,
                        strength,
                        force_type,
                        turbulence,
                        turbulence_scale,
                        world_aligned,
                    })
                }
                "NiPSysDragModifier" => ModifierKind::Drag(Drag {
                    object: object(r.i32("the drag object")?),
                    axis: r.vec3("the drag axis")?,
                    percentage: r.f32("the percentage")?,
                    range: r.f32("the range")?,
                    range_falloff: r.f32("the range falloff")?,
                }),
                "NiPSysBombModifier" => ModifierKind::Bomb(Bomb {
                    object: object(r.i32("the bomb object")?),
                    axis: r.vec3("the bomb axis")?,
                    decay: r.f32("the decay")?,
                    delta_v: r.f32("the change of speed")?,
                    decay_type: r.u32("the decay type")?,
                    symmetry_type: r.u32("the symmetry type")?,
                }),
                "NiPSysGrowFadeModifier" => ModifierKind::GrowFade(GrowFade {
                    grow_time: r.f32("the grow time")?,
                    grow_generation: r.u16("the grow generation")?,
                    fade_time: r.f32("the fade time")?,
                    fade_generation: r.u16("the fade generation")?,
                    base_scale: if ctx.bs_version > 16 {
                        r.f32("the base scale")?
                    } else {
                        0.0
                    },
                }),
                "NiPSysRotationModifier" => {
                    let speed = r.f32("the rotation speed")?;
                    let speed_variation = r.f32("the rotation speed variation")?;
                    let angle = r.f32("the rotation angle")?;
                    let angle_variation = r.f32("the rotation angle variation")?;
                    let random_speed_sign = r.bool("the random speed sign flag")?;
                    let random_axis = r.bool("the random axis flag")?;
                    let axis = r.vec3("the rotation axis")?;
                    ModifierKind::Rotation(Rotation {
                        speed,
                        speed_variation,
                        angle,
                        angle_variation,
                        random_speed_sign,
                        random_axis,
                        axis,
                    })
                }
                "NiPSysColorModifier" => {
                    let data = r.i32("the colour data")?;
                    let (kind, keys) = match self.reference(data) {
                        Some(d) if self.block_type(d) == "NiColorData" => self.color_keys(d)?,
                        _ => (0, Vec::new()),
                    };
                    ModifierKind::Color { kind, keys }
                }
                "BSPSysSimpleColorModifier" => {
                    let fade_in = r.f32("the fade-in fraction")?;
                    let fade_out = r.f32("the fade-out fraction")?;
                    let color1_end = r.f32("colour 1's end")?;
                    let color2_start = r.f32("colour 2's start")?;
                    let color2_end = r.f32("colour 2's end")?;
                    let color3_start = r.f32("colour 3's start")?;
                    let colors = [
                        vec4(&mut r, "colour 1")?,
                        vec4(&mut r, "colour 2")?,
                        vec4(&mut r, "colour 3")?,
                    ];
                    ModifierKind::SimpleColor(SimpleColor {
                        fade_in,
                        fade_out,
                        color1_end,
                        color2_start,
                        color2_end,
                        color3_start,
                        colors,
                    })
                }
                "NiPSysColliderManager" => {
                    let mut colliders = Vec::new();
                    let mut next = r.i32("the first collider")?;
                    let mut guard = 0;
                    while let Some(c) = self.reference(next) {
                        guard += 1;
                        if guard > 32 {
                            break;
                        }
                        let (collider, after) = self.collider(c, parents, apply_root)?;
                        colliders.push(collider);
                        next = after;
                    }
                    ModifierKind::Colliders(colliders)
                }
                "BSWindModifier" => ModifierKind::Wind {
                    strength: r.f32("the strength")?,
                },
                "BSParentVelocityModifier" => ModifierKind::ParentVelocity {
                    damping: r.f32("the damping")?,
                },
                "BSPSysStripUpdateModifier" => ModifierKind::StripUpdate {
                    update_delta: r.f32("the update time")?,
                },
                other => {
                    r.take(r.remaining(), "an unread modifier")?;
                    ModifierKind::Other(other.to_string())
                }
            };
            exact(&r, &type_name)?;
            Ok(Modifier {
                block: index,
                name,
                order,
                active,
                kind,
            })
        })();
        decoded.map_err(|e| Error::InBlock {
            index,
            type_name,
            source: Box::new(e),
        })
    }

    /// A shape's vertices for a mesh emitter.
    fn mesh_geometry(&self, index: usize) -> Option<MeshGeometry> {
        let Ok(Block::Geometry(g)) = self.block(index) else {
            return None;
        };
        let data = self.reference(g.data)?;
        let Ok(Block::GeometryData(d)) = self.block(data) else {
            return None;
        };
        Some(MeshGeometry {
            positions: d.positions,
            normals: d.normals,
            triangles: d.triangles,
            skinned: self.reference(g.skin).is_some(),
        })
    }

    /// A collider and the next one in its manager's chain.
    fn collider(
        &self,
        index: usize,
        parents: &BTreeMap<usize, usize>,
        apply_root: bool,
    ) -> Result<(Collider, i32)> {
        let type_name = self.block_type(index).to_string();
        let mut r = self.reader(index);
        let decoded = (|| -> Result<(Collider, i32)> {
            let bounce = r.f32("the bounce")?;
            let spawn_on_collide = r.bool("spawn on collide")?;
            let die_on_collide = r.bool("die on collide")?;
            let spawn = self.reference(r.i32("the spawn modifier")?);
            r.i32("the collider's manager")?;
            let next = r.i32("the next collider")?;
            let object = self.object_ref(r.i32("the collider object")?, parents, apply_root);
            let shape = match type_name.as_str() {
                "NiPSysPlanarCollider" => ColliderShape::Plane {
                    width: r.f32("the width")?,
                    height: r.f32("the height")?,
                    x_axis: r.vec3("the x axis")?,
                    y_axis: r.vec3("the y axis")?,
                },
                "NiPSysSphericalCollider" => ColliderShape::Sphere {
                    radius: r.f32("the radius")?,
                },
                other => {
                    r.take(r.remaining(), "an unread collider")?;
                    ColliderShape::Other(other.to_string())
                }
            };
            exact(&r, &type_name)?;
            Ok((
                Collider {
                    bounce,
                    spawn_on_collide,
                    die_on_collide,
                    spawn,
                    object,
                    shape,
                },
                next,
            ))
        })();
        decoded.map_err(|e| Error::InBlock {
            index,
            type_name,
            source: Box::new(e),
        })
    }

    /// A controller and the next one in its chain.
    fn controller(&self, index: usize) -> Result<(Controller, i32)> {
        let type_name = self.block_type(index).to_string();
        let ctx = self.ctx();
        let mut r = self.reader(index);
        let decoded = (|| -> Result<(Controller, i32)> {
            let (next, time) = read_time_control(&mut r)?;
            let float_of = |reference: i32| -> Result<Option<FloatInterp>> {
                match self.reference(reference) {
                    Some(i) if self.block_type(i) == "NiFloatInterpolator" => {
                        self.float_interp(i).map(Some)
                    }
                    _ => Ok(None),
                }
            };
            let bool_of = |reference: i32| -> Result<Option<BoolInterp>> {
                match self.reference(reference) {
                    Some(i)
                        if matches!(
                            self.block_type(i),
                            "NiBoolInterpolator" | "NiBoolTimelineInterpolator"
                        ) =>
                    {
                        self.bool_interp(i).map(Some)
                    }
                    _ => Ok(None),
                }
            };
            let kind = match type_name.as_str() {
                "NiPSysEmitterCtlr" | "BSPSysMultiTargetEmitterCtlr" => {
                    let birth_rate = float_of(r.i32("the birth rate interpolator")?)?;
                    let modifier = string_of(&ctx, &mut r, "the emitter's name")?;
                    let active = bool_of(r.i32("the emitter-active interpolator")?)?;
                    let multi_target = if type_name == "BSPSysMultiTargetEmitterCtlr" {
                        Some((r.u16("the most emitters")?, r.i32("the master system")?))
                    } else {
                        None
                    };
                    ControllerKind::Emitter {
                        modifier,
                        birth_rate,
                        active,
                        multi_target,
                    }
                }
                "NiPSysUpdateCtlr" => ControllerKind::Update,
                "NiPSysResetOnLoopCtlr" => ControllerKind::ResetOnLoop,
                "NiPSysModifierActiveCtlr" => {
                    let value = bool_of(r.i32("the interpolator")?)?;
                    let modifier = string_of(&ctx, &mut r, "the modifier's name")?;
                    ControllerKind::ModifierActive { modifier, value }
                }
                t if MODIFIER_FLOAT_CONTROLLERS.contains(&t) => {
                    let value = float_of(r.i32("the interpolator")?)?;
                    let modifier = string_of(&ctx, &mut r, "the modifier's name")?;
                    ControllerKind::ModifierFloat { modifier, value }
                }
                _ => {
                    r.take(r.remaining(), "an unread controller")?;
                    ControllerKind::Other
                }
            };
            exact(&r, &type_name)?;
            Ok((
                Controller {
                    block: index,
                    type_name: type_name.clone(),
                    time,
                    kind,
                },
                next,
            ))
        })();
        decoded.map_err(|e| Error::InBlock {
            index,
            type_name: type_name.clone(),
            source: Box::new(e),
        })
    }

    /// `NiFloatInterpolator` (`00a3c800`): value, `NiFloatData`.
    fn float_interp(&self, index: usize) -> Result<FloatInterp> {
        let mut r = self.reader(index);
        let value = r.f32("a value")?;
        let data = r.i32("the float data")?;
        exact(&r, "NiFloatInterpolator")?;
        let keys = match self.reference(data) {
            Some(d) if self.block_type(d) == "NiFloatData" => {
                let mut r = self.reader(d);
                let keys = read_float_keys(&mut r)?;
                exact(&r, "NiFloatData")?;
                keys
            }
            _ => FloatKeys::default(),
        };
        Ok(FloatInterp { value, keys })
    }

    /// `NiBoolInterpolator` (`00a53600`): value u8, `NiBoolData`
    /// (`00a53a50`: key count, kind, then time + u8 per key).
    fn bool_interp(&self, index: usize) -> Result<BoolInterp> {
        let mut r = self.reader(index);
        let value = r.u8("a value")?;
        let data = r.i32("the bool data")?;
        let (kind, keys) = match self.reference(data) {
            Some(d) if self.block_type(d) == "NiBoolData" => {
                let mut d = self.reader(d);
                let n = d.u32("the key count")? as usize;
                let kind = if n > 0 { d.u32("the key type")? } else { 0 };
                let mut keys = Vec::with_capacity(n.min(4096));
                for _ in 0..n {
                    let t = d.f32("a key time")?;
                    let v = d.u8("a key value")? != 0;
                    keys.push((t, v));
                }
                exact(&d, "NiBoolData")?;
                (kind, keys)
            }
            _ => (0, Vec::new()),
        };
        Ok(BoolInterp {
            value,
            kind,
            keys,
            timeline: self.block_type(index) == "NiBoolTimelineInterpolator",
        })
    }

    /// `NiColorData` (`00a3d370`): key count, kind, time + RGBA (quadratic
    /// keys' tangents and TBC parameters skipped).
    fn color_keys(&self, index: usize) -> Result<(u32, Vec<ColorKey>)> {
        let mut r = self.reader(index);
        let n = r.u32("the key count")? as usize;
        let kind = if n > 0 { r.u32("the key type")? } else { 0 };
        let mut keys = Vec::with_capacity(n.min(4096));
        for _ in 0..n {
            let time = r.f32("a key time")?;
            let color = vec4(&mut r, "a key colour")?;
            match kind {
                2 => {
                    r.take(32, "key tangents")?;
                }
                3 => {
                    r.take(12, "TBC parameters")?;
                }
                _ => {}
            }
            keys.push(ColorKey { time, color });
        }
        exact(&r, "NiColorData")?;
        Ok((kind, keys))
    }
}

/// The part of `NiParticleSystem` after its data reference: skin, the
/// material list (count, then name + extra data each), the active
/// material, a "needs update" byte, world space, the modifiers.
fn read_system_rest(r: &mut Reader, _ctx: &Ctx) -> Result<(bool, Vec<i32>)> {
    r.i32("the skin instance")?;
    let materials = r.u32("the material count")? as usize;
    r.counted(materials, 8, "materials", |r| {
        r.i32("a material name")?;
        r.i32("a material's extra data")
    })?;
    r.i32("the active material")?;
    r.u8("the material needs-update flag")?;
    let world_space = r.bool("the world-space flag")?;
    let modifiers = r.ref_list("modifiers")?;
    Ok((world_space, modifiers))
}

/// `NiFloatData` keys: count, kind, then time + value (+ two tangents for
/// quadratic keys, + three TBC parameters).
fn read_float_keys(r: &mut Reader) -> Result<FloatKeys> {
    let n = r.u32("a key count")? as usize;
    if n == 0 {
        return Ok(FloatKeys::default());
    }
    let kind = r.u32("a key type")?;
    let mut keys = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        let time = r.f32("a key time")?;
        let value = r.f32("a key value")?;
        let tangents = match kind {
            2 => Some((r.f32("a tangent")?, r.f32("a tangent")?)),
            3 => {
                r.take(12, "TBC parameters")?;
                None
            }
            _ => None,
        };
        keys.push(FloatKey {
            time,
            value,
            tangents,
            hold: kind == 5,
        });
    }
    Ok(FloatKeys { kind, keys })
}
