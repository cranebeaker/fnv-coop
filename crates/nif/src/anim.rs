//! Skeletons and animations: the bones of a skeleton file
//! (`skeleton.nif`), and the sequences of an animation file (`.kf`): for
//! each bone, its translation, rotation and scale over time, as keyframes
//! (`NiTransformInterpolator` + `NiTransformData`) or as compressed
//! B-splines (`NiBSplineCompTransformInterpolator`, which most of the
//! game's character animations use). Layouts follow the community NIF
//! format description for 20.2.0.7 with Bethesda version 34.

use std::collections::HashMap;

use crate::blocks::Block;
use crate::error::Result;
use crate::file::Nif;
use crate::math::{Mat3, Transform, Vec3};
use crate::reader::Reader;

/// A bone of a skeleton.
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    pub name: String,
    /// Index of the parent bone in the same list.
    pub parent: Option<usize>,
    /// Relative to the parent (the file's own pose).
    pub local: Transform,
}

/// A rotation as a unit quaternion (w, x, y, z), as the files store it.
pub type Quat = [f32; 4];

/// One animation sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    pub name: String,
    pub start: f32,
    pub stop: f32,
    /// Looping (`cycle type` 0) or not.
    pub looping: bool,
    pub tracks: Vec<Track>,
    /// The node that carries the actor's own movement (`Bip01` for
    /// people); see [`posed`].
    pub accum_root: Option<String>,
    /// Changes to shapes' materials (opacity, glow), by shape name.
    pub materials: Vec<MaterialTrack>,
    /// The sequence's text keys (`NiTextKeyExtraData`): a time and a
    /// label (`start`, `end`, `a:L`, `Blend: 10`, `SoundPath: …`), in file
    /// order. The game reads its animation groups' timing from them.
    pub text_keys: Vec<(f32, String)>,
}

/// How a sequence changes a shape's material: the controlled block's
/// shape (`node`), which value, and its keys.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialTrack {
    pub node: String,
    pub target: MaterialTarget,
    pub keys: MaterialKeys,
}

/// Which material value a [`MaterialTrack`] changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialTarget {
    /// `NiAlphaController`: the material's alpha.
    Alpha,
    /// `BSMaterialEmittanceMultController`: the emissive multiplier.
    EmissiveMult,
    /// `NiMaterialColorController` with controller ID `SELF_ILLUM`: the
    /// emissive colour.
    Emissive,
}

/// A material track's keys: numbers (curves as for angles) or colours.
#[derive(Debug, Clone, PartialEq)]
pub enum MaterialKeys {
    Float(Vec<FloatKey>),
    /// Colours, straight lines between keys (tangents of quadratic keys
    /// aren't kept).
    Color(Vec<(f32, Vec3)>),
}

impl MaterialTrack {
    /// The number at `time` (a float track), if it has keys.
    pub fn float_at(&self, time: f32) -> Option<f32> {
        match &self.keys {
            MaterialKeys::Float(keys) => sample_curve(keys, time),
            MaterialKeys::Color(_) => None,
        }
    }

    /// The colour at `time` (a colour track), if it has keys.
    pub fn color_at(&self, time: f32) -> Option<Vec3> {
        match &self.keys {
            MaterialKeys::Color(keys) => sample_linear(keys, time, |a, b, f| {
                [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * f)
            }),
            MaterialKeys::Float(_) => None,
        }
    }
}

impl Sequence {
    /// How far the accumulation root moves over the sequence, in the
    /// skeleton's axes (x, y on the ground, z up), if it moves at all.
    pub fn root_travel(&self) -> Option<Vec3> {
        let root = self.accum_root.as_deref()?;
        let track = self.tracks.iter().find(|t| t.node == root)?;
        let a = track.sample(self.start).translation?;
        let b = track.sample(self.stop).translation?;
        Some([b[0] - a[0], b[1] - a[1], b[2] - a[2]])
    }

    /// How fast the actor moves while it plays, in units a second: the
    /// accumulation root's travel along the ground over the sequence's
    /// length. In the game it's this movement that carries a walking actor
    /// (the root's motion is accumulated into the actor's position).
    pub fn root_speed(&self) -> Option<f32> {
        let d = self.root_travel()?;
        let length = self.stop - self.start;
        (length > 0.0).then(|| (d[0] * d[0] + d[1] * d[1]).sqrt() / length)
    }

    /// How far the accumulation root has moved `time` seconds into the
    /// sequence (from its start, held at the ends), in the skeleton's
    /// axes: the actor's own movement so far, as [`Self::root_travel`] is
    /// over the whole sequence. `None` when the root doesn't move.
    pub fn root_offset(&self, time: f32) -> Option<Vec3> {
        let root = self.accum_root.as_deref()?;
        let track = self.tracks.iter().find(|t| t.node == root)?;
        let a = track.sample(self.start).translation?;
        let t = (self.start + time).clamp(self.start, self.stop);
        let b = track.sample(t).translation?;
        Some([b[0] - a[0], b[1] - a[1], b[2] - a[2]])
    }

    /// Which way the accumulation root faces `time` seconds into the
    /// sequence: the angle of its +y axis on the ground, radians clockwise
    /// from the skeleton's +y (as headings count). `None` when the
    /// sequence doesn't turn it.
    pub fn root_heading(&self, time: f32) -> Option<f32> {
        self.bone_heading(self.accum_root.as_deref()?, time)
    }

    /// Like [`Self::root_heading`] for any bone the sequence moves, in its
    /// parent's axes (`Bip01 NonAccum`: which way the body faces on the
    /// actor's spot).
    pub fn bone_heading(&self, bone: &str, time: f32) -> Option<f32> {
        let track = self
            .tracks
            .iter()
            .find(|t| t.node.eq_ignore_ascii_case(bone))?;
        let t = (self.start + time).clamp(self.start, self.stop);
        let m = quat_matrix(track.sample(t).rotation?);
        // The rotated +y axis is the matrix's second column.
        Some(m[0][1].atan2(m[1][1]))
    }
}

/// How one bone moves.
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub node: String,
    pub motion: Motion,
    /// The controlled block's priority: when several sequences move the
    /// same bone, the highest priority's pose wins (Gamebryo's blend
    /// interpolators), lower ones only filling in while it fades in or
    /// out. The game's walks move the body at a higher priority than its
    /// idles.
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Motion {
    Keys {
        translation: Vec<(f32, Vec3)>,
        rotation: Vec<(f32, Quat)>,
        scale: Vec<(f32, f32)>,
        /// The interpolator's own values, for channels without keys.
        default: (Option<Vec3>, Option<Quat>, Option<f32>),
        /// Rotation stored as three angle curves (radians about X, Y and
        /// Z; rotation key type 4) instead of `rotation`'s quaternions.
        /// They're sampled at each moment and only then turned into a
        /// rotation: a ceiling fan's Z curve runs from 0 to −20π over its
        /// 10 s, which keys turned into rotations at their own times would
        /// lose (every whole turn is the same rotation).
        euler: Option<Box<[Vec<FloatKey>; 3]>>,
    },
    Spline(Spline),
}

/// One key of a curve of numbers (an angle, a scale).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatKey {
    pub time: f32,
    pub value: f32,
    /// Quadratic keys (type 2): the tangents into and out of the key, as
    /// the files store them (in first), per interval between keys.
    pub tangents: Option<(f32, f32)>,
    /// Constant keys (type 5): the value holds until the next key.
    pub hold: bool,
}

/// A curve's value at `t` (held at the ends): between two quadratic keys
/// the cubic Hermite curve through their values with the first's outgoing
/// and the second's incoming tangent, else a straight line. The tangents'
/// order (in, then out) is read off the game's files: the ceiling fan's
/// spin, from 0 to −62.83 over 10 s, has (0, −62.83) on its first key and
/// (−62.83, 0) on its last, which read so turns at an even speed (read the
/// other way round it would speed up and slow down every 10 s).
pub fn sample_curve(keys: &[FloatKey], t: f32) -> Option<f32> {
    let first = keys.first()?;
    if t <= first.time {
        return Some(first.value);
    }
    for pair in keys.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if t <= b.time {
            if a.hold {
                return Some(if t < b.time { a.value } else { b.value });
            }
            let span = b.time - a.time;
            let u = if span > 0.0 { (t - a.time) / span } else { 1.0 };
            return Some(match (a.tangents, b.tangents) {
                (Some((_, out)), Some((into, _))) => {
                    let (u2, u3) = (u * u, u * u * u);
                    a.value * (2.0 * u3 - 3.0 * u2 + 1.0)
                        + b.value * (-2.0 * u3 + 3.0 * u2)
                        + out * (u3 - 2.0 * u2 + u)
                        + into * (u3 - u2)
                }
                _ => a.value + (b.value - a.value) * u,
            });
        }
    }
    keys.last().map(|k| k.value)
}

/// A compressed B-spline transform (`NiBSplineCompTransformInterpolator`).
#[derive(Debug, Clone, PartialEq)]
pub struct Spline {
    pub start: f32,
    pub stop: f32,
    /// Control points per channel, decompressed; empty uses the default.
    pub translation: Vec<Vec3>,
    pub rotation: Vec<Quat>,
    pub scale: Vec<f32>,
    pub default: (Vec3, Quat, f32),
}

/// A bone's pose at one moment; `None` where the animation leaves it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pose {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<f32>,
}

impl Nif {
    /// Every node of the file as a bone list (parents before children),
    /// with the file's own transforms.
    pub fn skeleton(&self) -> Result<Vec<Bone>> {
        let mut bones = Vec::new();
        let mut visited = vec![false; self.blocks().len()];
        for &root in self.roots() {
            self.skeleton_visit(root, None, 0, &mut visited, &mut bones)?;
        }
        Ok(bones)
    }

    fn skeleton_visit(
        &self,
        reference: i32,
        parent: Option<usize>,
        depth: usize,
        visited: &mut [bool],
        bones: &mut Vec<Bone>,
    ) -> Result<()> {
        let Some(index) = self.reference(reference) else {
            return Ok(());
        };
        if visited[index] || depth > 128 {
            return Ok(());
        }
        visited[index] = true;
        if let Block::Node(node) = self.block(index)? {
            bones.push(Bone {
                name: node.av.net.name.clone(),
                parent,
                local: node.av.transform,
            });
            let me = bones.len() - 1;
            for child in node.children {
                self.skeleton_visit(child, Some(me), depth + 1, visited, bones)?;
            }
        }
        Ok(())
    }

    /// The text keys of the file's sequences (`NiTextKeyExtraData`: a name,
    /// a count, then per key a time f32 and a string): moments an animation
    /// names (`start`, `Hit`, `end`), in file order. The first-person
    /// `Pipboy.kf` holds the arm up at its `Hit` (0.33 s of 0.73).
    pub fn text_keys(&self) -> Result<Vec<(f32, String)>> {
        let mut out = Vec::new();
        for index in 0..self.blocks().len() {
            if self.block_type(index) != "NiTextKeyExtraData" {
                continue;
            }
            let mut r = self.reader(index);
            self.string_at(&mut r, "the extra data's name")?;
            let count = r.u32("the text key count")? as usize;
            for _ in 0..count.min(4096) {
                let time = r.f32("a text key's time")?;
                let text = self.string_at(&mut r, "a text key")?;
                out.push((time, text));
            }
        }
        Ok(out)
    }

    /// The animation sequences in the file (a `.kf`).
    pub fn sequences(&self) -> Result<Vec<Sequence>> {
        let mut out = Vec::new();
        for index in 0..self.blocks().len() {
            if self.block_type(index) == "NiControllerSequence" {
                out.push(self.sequence(index)?);
            }
        }
        Ok(out)
    }

    /// The controllers a model carries itself, outside any
    /// `NiControllerSequence`, as one sequence (named `""`): each node's
    /// `NiTransformController` (its interpolator's keys move that node) and
    /// each shape's material's `NiAlphaController` (its keys set that
    /// shape's opacity). The game's impact effects animate this way
    /// (`Effects\ImpactBallistic*01.NIF`: the billboard node and the
    /// material's alpha). Start and stop are the controllers' earliest start
    /// and latest stop; it loops when the first controller's cycle type
    /// (flags bits 1–2) is 0. `None` without such controllers.
    ///
    /// A controller (`NiTimeController`, 20.2.0.7): next controller, flags
    /// (u16), frequency, phase, start, stop, target, then (single
    /// interpolator controllers) the interpolator.
    pub fn own_controllers(&self) -> Result<Option<Sequence>> {
        let mut tracks = Vec::new();
        let mut materials = Vec::new();
        let mut span: Option<(f32, f32)> = None;
        let mut looping = None;
        // The controllers hanging off one object, in chain order.
        let chain = |first: i32| -> Result<Vec<ChainLink>> {
            let mut out: Vec<ChainLink> = Vec::new();
            let mut next = first;
            while let Some(c) = self.reference(next) {
                if out.len() > 64 || out.iter().any(|&(i, ..)| i == c) {
                    break;
                }
                let mut r = self.reader(c);
                next = r.i32("the next controller")?;
                let flags = r.u16("the controller flags")?;
                r.f32("the frequency")?;
                r.f32("the phase")?;
                let start = r.f32("the start time")?;
                let stop = r.f32("the stop time")?;
                r.i32("the target")?;
                let interpolator = r.i32("the interpolator").unwrap_or(-1);
                out.push((c, start, stop, flags, interpolator));
            }
            Ok(out)
        };
        let mut note = |start: f32, stop: f32, flags: u16| {
            span = Some(span.map_or((start, stop), |(a, b)| (a.min(start), b.max(stop))));
            looping.get_or_insert((flags >> 1) & 3 == 0);
        };
        for index in 0..self.blocks().len() {
            let (name, controller, properties) = match self.block(index) {
                Ok(Block::Node(n)) => (n.av.net.name, n.av.net.controller, Vec::new()),
                Ok(Block::Geometry(g)) => (g.av.net.name, g.av.net.controller, g.av.properties),
                _ => continue,
            };
            for (c, start, stop, flags, interpolator) in chain(controller)? {
                if self.block_type(c) != "NiTransformController" {
                    continue;
                }
                let Some(i) = self
                    .reference(interpolator)
                    .filter(|&i| self.block_type(i) == "NiTransformInterpolator")
                else {
                    continue;
                };
                tracks.push(Track {
                    node: name.clone(),
                    motion: self.transform_interpolator(i)?,
                    priority: 0,
                });
                note(start, stop, flags);
            }
            // The shape's material property: an `NiObjectNET` (name, extra
            // data list, controller).
            for p in properties.iter().filter_map(|&p| self.reference(p)) {
                if self.block_type(p) != "NiMaterialProperty" {
                    continue;
                }
                let mut r = self.reader(p);
                r.i32("the property name")?;
                let extras = r.u32("the extra data count")? as usize;
                for _ in 0..extras.min(1024) {
                    r.i32("an extra data")?;
                }
                let first = r.i32("the property's controller")?;
                for (c, start, stop, flags, interpolator) in chain(first)? {
                    if self.block_type(c) != "NiAlphaController" {
                        continue;
                    }
                    let Some(i) = self.reference(interpolator) else {
                        continue;
                    };
                    if let Some(keys) = self.material_keys(i)? {
                        materials.push(MaterialTrack {
                            node: name.clone(),
                            target: MaterialTarget::Alpha,
                            keys,
                        });
                        note(start, stop, flags);
                    }
                }
            }
        }
        let Some((start, stop)) = span else {
            return Ok(None);
        };
        Ok(Some(Sequence {
            name: String::new(),
            start,
            stop,
            looping: looping.unwrap_or(false),
            tracks,
            accum_root: None,
            materials,
            text_keys: Vec::new(),
        }))
    }

    fn string_at(&self, r: &mut Reader, what: &str) -> Result<String> {
        let i = r.i32(what)?;
        Ok(usize::try_from(i)
            .ok()
            .and_then(|i| self.header().strings.get(i))
            .cloned()
            .unwrap_or_default())
    }

    fn sequence(&self, index: usize) -> Result<Sequence> {
        let mut r = self.reader(index);
        let name = self.string_at(&mut r, "the sequence name")?;
        let count = r.u32("the controlled block count")? as usize;
        r.u32("the array grow-by")?;
        let mut blocks = Vec::with_capacity(count.min(1024));
        for _ in 0..count {
            let interpolator = r.i32("an interpolator")?;
            r.i32("a controller")?;
            let priority = r.u8("a priority")?;
            let node = self.string_at(&mut r, "a node name")?;
            r.i32("a property type")?;
            let controller = self.string_at(&mut r, "a controller type")?;
            let controller_id = self.string_at(&mut r, "a controller ID")?;
            r.i32("an interpolator ID")?;
            blocks.push((interpolator, node, controller, controller_id, priority));
        }
        r.f32("the weight")?;
        let text_keys = r.i32("the text keys")?;
        let cycle = r.u32("the cycle type")?;
        r.f32("the frequency")?;
        let start = r.f32("the start time")?;
        let stop = r.f32("the stop time")?;
        r.i32("the manager")?;
        let accum_root =
            Some(self.string_at(&mut r, "the accumulation root")?).filter(|s| !s.is_empty());
        let text_keys = match self
            .reference(text_keys)
            .filter(|&i| self.block_type(i) == "NiTextKeyExtraData")
        {
            Some(i) => self.text_keys_of_block(i)?,
            None => Vec::new(),
        };
        let mut tracks = Vec::new();
        let mut materials = Vec::new();
        for (interpolator, node, controller, controller_id, priority) in blocks {
            let Some(i) = self.reference(interpolator) else {
                continue;
            };
            let target = match (controller.as_str(), controller_id.as_str()) {
                ("NiAlphaController", _) => Some(MaterialTarget::Alpha),
                ("BSMaterialEmittanceMultController", _) => Some(MaterialTarget::EmissiveMult),
                ("NiMaterialColorController", "SELF_ILLUM") => Some(MaterialTarget::Emissive),
                _ => None,
            };
            if let Some(target) = target {
                if let Some(keys) = self.material_keys(i)? {
                    materials.push(MaterialTrack { node, target, keys });
                }
                continue;
            }
            let motion = match self.block_type(i) {
                "NiTransformInterpolator" => self.transform_interpolator(i)?,
                "NiBSplineCompTransformInterpolator" => self.spline_interpolator(i)?,
                _ => continue,
            };
            tracks.push(Track {
                node,
                motion,
                priority,
            });
        }
        Ok(Sequence {
            name,
            start,
            stop,
            looping: cycle == 0,
            tracks,
            accum_root,
            materials,
            text_keys,
        })
    }

    /// `NiTextKeyExtraData`: the extra data's name, then a key count and
    /// (time, string) keys.
    fn text_keys_of_block(&self, index: usize) -> Result<Vec<(f32, String)>> {
        let mut r = self.reader(index);
        self.string_at(&mut r, "the extra data name")?;
        let n = r.u32("the text key count")? as usize;
        let mut keys = Vec::with_capacity(n.min(1024));
        for _ in 0..n {
            let time = r.f32("a text key time")?;
            let text = self.string_at(&mut r, "a text key")?;
            keys.push((time, text));
        }
        Ok(keys)
    }

    /// A material controller's keys: `NiFloatInterpolator` (its value, then
    /// `NiFloatData`) or `NiPoint3Interpolator` (its value, then
    /// `NiPosData`). Without data, the interpolator's own value as a single
    /// key, unless it's the "not set" marker. `None` for other kinds.
    fn material_keys(&self, index: usize) -> Result<Option<MaterialKeys>> {
        let mut r = self.reader(index);
        let data_of =
            |data: i32, kind: &str| self.reference(data).filter(|&d| self.block_type(d) == kind);
        Ok(match self.block_type(index) {
            "NiFloatInterpolator" => {
                let value = r.f32("a value")?;
                let data = r.i32("the float data")?;
                let keys = match data_of(data, "NiFloatData") {
                    Some(d) => read_float_curve(&mut self.reader(d))?,
                    None => Vec::new(),
                };
                if keys.is_empty() && !is_set(value) {
                    return Ok(None);
                }
                Some(MaterialKeys::Float(if keys.is_empty() {
                    vec![FloatKey {
                        time: 0.0,
                        value,
                        tangents: None,
                        hold: false,
                    }]
                } else {
                    keys
                }))
            }
            "NiPoint3Interpolator" => {
                let value = r.vec3("a value")?;
                let data = r.i32("the position data")?;
                let keys = match data_of(data, "NiPosData") {
                    Some(d) => read_key_group(&mut self.reader(d), |r| r.vec3("a colour"), 12)?,
                    None => Vec::new(),
                };
                if keys.is_empty() && !value.iter().all(|&v| is_set(v)) {
                    return Ok(None);
                }
                Some(MaterialKeys::Color(if keys.is_empty() {
                    vec![(0.0, value)]
                } else {
                    keys
                }))
            }
            _ => None,
        })
    }

    fn transform_interpolator(&self, index: usize) -> Result<Motion> {
        let mut r = self.reader(index);
        let translation = r.vec3("a translation")?;
        let rotation = read_quat(&mut r)?;
        let scale = r.f32("a scale")?;
        let data = r.i32("the transform data")?;
        let set = is_set;
        let default = (
            translation.iter().all(|&v| set(v)).then_some(translation),
            rotation.iter().all(|&v| set(v)).then_some(rotation),
            set(scale).then_some(scale),
        );
        let (translation, rotation, scale, euler) = match self
            .reference(data)
            .filter(|&i| matches!(self.block_type(i), "NiTransformData" | "NiKeyframeData"))
        {
            Some(i) => read_keyframes(&mut self.reader(i))?,
            None => (Vec::new(), Vec::new(), Vec::new(), None),
        };
        Ok(Motion::Keys {
            translation,
            rotation,
            scale,
            default,
            euler,
        })
    }

    fn spline_interpolator(&self, index: usize) -> Result<Motion> {
        let mut r = self.reader(index);
        let start = r.f32("the start time")?;
        let stop = r.f32("the stop time")?;
        let data = r.i32("the spline data")?;
        r.i32("the basis data")?;
        let translation = r.vec3("a translation")?;
        let rotation = read_quat(&mut r)?;
        let scale = r.f32("a scale")?;
        let handles = [
            r.u32("the translation handle")?,
            r.u32("the rotation handle")?,
            r.u32("the scale handle")?,
        ];
        let mut ranges = [(0.0f32, 0.0f32); 3];
        for range in &mut ranges {
            *range = (r.f32("an offset")?, r.f32("a half range")?);
        }
        let shorts = match self
            .reference(data)
            .filter(|&i| self.block_type(i) == "NiBSplineData")
        {
            Some(i) => {
                let mut d = self.reader(i);
                let floats = d.u32("the float control point count")? as usize;
                d.take(floats * 4, "float control points")?;
                let n = d.u32("the short control point count")? as usize;
                d.counted(n, 2, "short control points", |d| {
                    Ok(d.u16("a control point")? as i16)
                })?
            }
            None => Vec::new(),
        };
        let points = self.spline_points(index)?;
        let channel = |handle: u32, width: usize, range: (f32, f32)| {
            spline_channel(&shorts, handle, width, points, range)
        };
        Ok(Motion::Spline(Spline {
            start,
            stop,
            translation: channel(handles[0], 3, ranges[0])
                .into_iter()
                .map(|v| [v[0], v[1], v[2]])
                .collect(),
            rotation: channel(handles[1], 4, ranges[1])
                .into_iter()
                .map(|v| normalize_quat([v[0], v[1], v[2], v[3]]))
                .collect(),
            scale: channel(handles[2], 1, ranges[2])
                .into_iter()
                .map(|v| v[0])
                .collect(),
            default: (translation, rotation, scale),
        }))
    }

    /// The control point count of a spline interpolator's basis.
    fn spline_points(&self, interpolator: usize) -> Result<usize> {
        let mut r = self.reader(interpolator);
        r.take(12, "the spline header")?;
        let basis = r.i32("the basis data")?;
        Ok(
            match self
                .reference(basis)
                .filter(|&i| self.block_type(i) == "NiBSplineBasisData")
            {
                Some(i) => self.reader(i).u32("the control point count")? as usize,
                None => 0,
            },
        )
    }
}

/// One channel's control points of a compressed B-spline: from the handle,
/// `width` shorts each, as many as the spline has (the basis's count), each
/// `offset + half range × short / 32767`. A handle of 0xFFFF (USHRT_MAX,
/// as the files write it: the gecko's idle, whose data has more shorts than
/// that, has its unused channels so, with -FLT_MAX ranges) or 0xFFFFFFFF
/// means the channel isn't there: none.
fn spline_channel(
    shorts: &[i16],
    handle: u32,
    width: usize,
    points: usize,
    (offset, half): (f32, f32),
) -> Vec<Vec<f32>> {
    if handle == u32::MAX || handle == u32::from(u16::MAX) {
        return Vec::new();
    }
    (0..points)
        .filter_map(|p| {
            let at = handle as usize + p * width;
            let values = shorts.get(at..at + width)?;
            Some(
                values
                    .iter()
                    .map(|&s| offset + half * (f32::from(s) / 32767.0))
                    .collect(),
            )
        })
        .collect()
}

fn read_quat(r: &mut Reader) -> Result<Quat> {
    Ok([
        r.f32("a rotation")?,
        r.f32("a rotation")?,
        r.f32("a rotation")?,
        r.f32("a rotation")?,
    ])
}

/// A controller in a chain: its block, start, stop, flags and interpolator.
type ChainLink = (usize, f32, f32, u16, i32);

type Keyframes = (
    Vec<(f32, Vec3)>,
    Vec<(f32, Quat)>,
    Vec<(f32, f32)>,
    Option<Box<[Vec<FloatKey>; 3]>>,
);

/// `NiTransformData`: rotation keys, then translation and scale key
/// groups. Interpolation types: 1 linear, 2 quadratic (tangents follow
/// each value), 3 TBC (three parameters follow), 4 separate X, Y, Z angle
/// curves, 5 constant.
fn read_keyframes(r: &mut Reader) -> Result<Keyframes> {
    let n = r.u32("the rotation key count")? as usize;
    let mut rotation = Vec::new();
    let mut euler = None;
    if n > 0 {
        let kind = r.u32("the rotation key type")?;
        if kind == 4 {
            // Euler angle curves (radians) about X, Y and Z, kept as curves
            // (see `Motion::Keys::euler`).
            let mut curves: [Vec<FloatKey>; 3] = Default::default();
            for curve in &mut curves {
                *curve = read_float_curve(r)?;
            }
            euler = Some(Box::new(curves));
        } else {
            for _ in 0..n {
                let t = r.f32("a key time")?;
                let q = read_quat(r)?;
                if kind == 3 {
                    r.take(12, "TBC parameters")?;
                }
                rotation.push((t, q));
            }
        }
    }
    let translation = read_key_group(r, |r| r.vec3("a translation"), 12)?;
    let scale = read_key_group(r, |r| r.f32("a scale"), 4)?;
    Ok((translation, rotation, scale, euler))
}

/// A group of float keys with their tangents (quadratic keys) kept. TBC
/// keys' parameters are skipped: they're drawn as straight lines (a
/// guess; none were found in angle curves).
fn read_float_curve(r: &mut Reader) -> Result<Vec<FloatKey>> {
    let n = r.u32("a key count")? as usize;
    if n == 0 {
        return Ok(Vec::new());
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
    Ok(keys)
}

fn read_key_group<T>(
    r: &mut Reader,
    mut value: impl FnMut(&mut Reader) -> Result<T>,
    size: usize,
) -> Result<Vec<(f32, T)>> {
    let n = r.u32("a key count")? as usize;
    if n == 0 {
        return Ok(Vec::new());
    }
    let kind = r.u32("a key type")?;
    let mut keys = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        let t = r.f32("a key time")?;
        let v = value(r)?;
        match kind {
            2 => {
                r.take(2 * size, "key tangents")?;
            }
            3 => {
                r.take(12, "TBC parameters")?;
            }
            _ => {}
        }
        keys.push((t, v));
    }
    Ok(keys)
}

fn normalize_quat(q: Quat) -> Quat {
    let len = q.iter().map(|c| c * c).sum::<f32>().sqrt();
    if len > 0.0 {
        q.map(|c| c / len)
    } else {
        [1.0, 0.0, 0.0, 0.0]
    }
}

/// A rotation from angles about X, then Y, then Z (applied in that order).
fn euler_quat(a: [f32; 3]) -> Quat {
    let q = |axis: usize, angle: f32| {
        let mut q = [(angle / 2.0).cos(), 0.0, 0.0, 0.0];
        q[1 + axis] = (angle / 2.0).sin();
        q
    };
    quat_mul(q(2, a[2]), quat_mul(q(1, a[1]), q(0, a[0])))
}

fn quat_mul(a: Quat, b: Quat) -> Quat {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

/// A unit quaternion as a row-major rotation matrix (column vectors).
pub fn quat_matrix(q: Quat) -> Mat3 {
    let [w, x, y, z] = normalize_quat(q);
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ]
}

fn sample_scalar(keys: &[(f32, f32)], t: f32) -> Option<f32> {
    sample_linear(keys, t, |a, b, f| a + (b - a) * f)
}

/// Linear interpolation between the keys around `t` (held at the ends).
fn sample_linear<T: Copy>(keys: &[(f32, T)], t: f32, lerp: impl Fn(T, T, f32) -> T) -> Option<T> {
    let first = keys.first()?;
    if t <= first.0 {
        return Some(first.1);
    }
    for pair in keys.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if t <= b.0 {
            let span = b.0 - a.0;
            let f = if span > 0.0 { (t - a.0) / span } else { 1.0 };
            return Some(lerp(a.1, b.1, f));
        }
    }
    keys.last().map(|k| k.1)
}

fn slerp(a: Quat, b: Quat, f: f32) -> Quat {
    let mut d: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let b = if d < 0.0 {
        d = -d;
        b.map(|c| -c)
    } else {
        b
    };
    if d > 0.9995 {
        return normalize_quat([0, 1, 2, 3].map(|k| a[k] + (b[k] - a[k]) * f));
    }
    let theta = d.clamp(-1.0, 1.0).acos();
    let s = theta.sin();
    let wa = ((1.0 - f) * theta).sin() / s;
    let wb = (f * theta).sin() / s;
    [0, 1, 2, 3].map(|k| wa * a[k] + wb * b[k])
}

/// The value of an open uniform cubic B-spline over `points` at `u` in
/// 0..1 (the curve starts at the first point and ends at the last).
fn bspline<const N: usize>(points: &[[f32; N]], u: f32) -> Option<[f32; N]> {
    let n = points.len();
    if n == 0 {
        return None;
    }
    let degree = 3.min(n - 1);
    if degree == 0 {
        return Some(points[0]);
    }
    // Knots: degree + 1 zeros, uniform inside, degree + 1 at the end.
    let spans = (n - degree) as f32;
    let knot = |i: usize| -> f32 {
        if i <= degree {
            0.0
        } else if i >= n {
            spans
        } else {
            (i - degree) as f32
        }
    };
    let t = (u.clamp(0.0, 1.0) * spans).min(spans);
    // The span holding t.
    let mut k = degree;
    while k + 1 < n && t >= knot(k + 1) {
        k += 1;
    }
    // De Boor.
    let mut d: Vec<[f32; N]> = (0..=degree).map(|j| points[j + k - degree]).collect();
    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = j + k - degree;
            let denom = knot(i + degree + 1 - r) - knot(i);
            let alpha = if denom > 0.0 {
                (t - knot(i)) / denom
            } else {
                0.0
            };
            let previous = d[j - 1];
            for (value, before) in d[j].iter_mut().zip(previous) {
                *value = (1.0 - alpha) * before + alpha * *value;
            }
        }
    }
    Some(d[degree])
}

impl Track {
    /// The bone's pose at `time` (seconds, the sequence's own clock).
    pub fn sample(&self, time: f32) -> Pose {
        match &self.motion {
            Motion::Keys {
                translation,
                rotation,
                scale,
                default,
                euler,
            } => Pose {
                translation: sample_linear(translation, time, |a, b, f| {
                    [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * f)
                })
                .or(default.0),
                rotation: match euler {
                    // A curve without keys leaves its angle at 0.
                    Some(curves) => Some(euler_quat(
                        [0, 1, 2].map(|k| sample_curve(&curves[k], time).unwrap_or(0.0)),
                    )),
                    None => sample_linear(rotation, time, slerp),
                }
                .or(default.1),
                scale: sample_scalar(scale, time).or(default.2),
            },
            Motion::Spline(s) => {
                let span = s.stop - s.start;
                let u = if span > 0.0 {
                    (time - s.start) / span
                } else {
                    0.0
                };
                // Channels without control points use the interpolator's
                // own value, unless it's the "not set" marker (-FLT_MAX),
                // as it is for the idle's scales.
                Pose {
                    translation: bspline(&s.translation, u)
                        .or(Some(s.default.0).filter(|v| v.iter().all(|&c| is_set(c)))),
                    rotation: bspline(&s.rotation, u)
                        .map(normalize_quat)
                        .or(Some(s.default.1).filter(|v| v.iter().all(|&c| is_set(c)))),
                    scale: bspline(&s.scale.iter().map(|&v| [v]).collect::<Vec<_>>(), u)
                        .map(|v| v[0])
                        .or(Some(s.default.2).filter(|&v| is_set(v))),
                }
            }
        }
    }
}

impl Pose {
    /// A bone's local transform with this pose over its own.
    pub fn apply(&self, own: &Transform) -> Transform {
        Transform {
            rotation: self.rotation.map_or(own.rotation, quat_matrix),
            translation: self.translation.unwrap_or(own.translation),
            scale: self.scale.unwrap_or(own.scale),
        }
    }
}

/// Values stored as -FLT_MAX mean "not set".
fn is_set(v: f32) -> bool {
    v > -3.0e38 && v.is_finite()
}

/// Every bone's transform in the skeleton's space, with a sequence's pose
/// at `time` laid over the skeleton's own (bones the sequence doesn't move
/// keep theirs).
///
/// The sequence's accumulation root (`Bip01` for people) carries the
/// actor's own movement, which starts where the actor stands: when the
/// sequence doesn't move it, it sits at the skeleton's origin, and the
/// node under it (`Bip01 NonAccum`) takes the body to its height. (The
/// skeleton file has `Bip01` at the pelvis, 68 up, and the idle has
/// `Bip01 NonAccum` 66 above its parent; left as they are, the body
/// floats a body-height up.)
///
/// When the sequence does move the accumulation root (walking), that
/// movement is the actor's, applied to where the actor stands (see
/// [`Sequence::root_speed`]): the root keeps its turn but not its
/// position, so the body walks on the spot.
pub fn posed(skeleton: &[Bone], sequence: Option<&Sequence>, time: f32) -> Vec<Transform> {
    let tracks: HashMap<&str, &Track> = sequence
        .map(|s| s.tracks.iter().map(|t| (t.node.as_str(), t)).collect())
        .unwrap_or_default();
    let accum_root = sequence.and_then(|s| s.accum_root.as_deref());
    let mut world: Vec<Transform> = Vec::with_capacity(skeleton.len());
    for bone in skeleton {
        let is_root = accum_root == Some(bone.name.as_str());
        let local = match tracks.get(bone.name.as_str()) {
            Some(track) if is_root => {
                let mut t = track.sample(time).apply(&bone.local);
                t.translation = [0.0; 3];
                t
            }
            Some(track) => track.sample(time).apply(&bone.local),
            None if is_root => Transform::IDENTITY,
            None => bone.local,
        };
        let w = match bone.parent.and_then(|p| world.get(p)) {
            Some(parent) => parent.then_child(&local),
            None => local,
        };
        world.push(w);
    }
    world
}

/// Like [`posed`] with sequences laid one over another, each at its own
/// time: a bone takes the last layer that moves it (a first-person attack
/// over the hold pose), else its own transform.
pub fn posed_layers(skeleton: &[Bone], layers: &[(&Sequence, f32)]) -> Vec<Transform> {
    let tracks: Vec<HashMap<&str, &Track>> = layers
        .iter()
        .map(|(s, _)| s.tracks.iter().map(|t| (t.node.as_str(), t)).collect())
        .collect();
    let mut world: Vec<Transform> = Vec::with_capacity(skeleton.len());
    for bone in skeleton {
        let mut local = bone.local;
        for ((sequence, time), tracks) in layers.iter().zip(&tracks) {
            if let Some(track) = tracks.get(bone.name.as_str()) {
                local = track.sample(*time).apply(&local);
                if sequence.accum_root.as_deref() == Some(bone.name.as_str()) {
                    local.translation = [0.0; 3];
                }
            }
        }
        let w = match bone.parent.and_then(|p| world.get(p)) {
            Some(parent) => parent.then_child(&local),
            None => local,
        };
        world.push(w);
    }
    world
}

/// Like [`posed`] for the bottom sequence (its accumulation root at the
/// actor's origin unless the sequence moves it, and never moved away from
/// it), with more sequences laid over it as in [`posed_layers`]: an attack
/// over the hold pose.
pub fn posed_over(
    skeleton: &[Bone],
    base: Option<(&Sequence, f32)>,
    layers: &[(&Sequence, f32)],
) -> Vec<Transform> {
    let base_tracks: HashMap<&str, &Track> = base
        .map(|(s, _)| s.tracks.iter().map(|t| (t.node.as_str(), t)).collect())
        .unwrap_or_default();
    let accum_root = base.and_then(|(s, _)| s.accum_root.as_deref());
    let tracks: Vec<HashMap<&str, &Track>> = layers
        .iter()
        .map(|(s, _)| s.tracks.iter().map(|t| (t.node.as_str(), t)).collect())
        .collect();
    let mut world: Vec<Transform> = Vec::with_capacity(skeleton.len());
    for bone in skeleton {
        let name = bone.name.as_str();
        let is_root = accum_root == Some(name);
        let mut local = match (base_tracks.get(name), base) {
            (Some(track), Some((_, time))) => track.sample(time).apply(&bone.local),
            _ if is_root => Transform::IDENTITY,
            _ => bone.local,
        };
        for ((sequence, time), tracks) in layers.iter().zip(&tracks) {
            if let Some(track) = tracks.get(name) {
                local = track.sample(*time).apply(&local);
                if sequence.accum_root.as_deref() == Some(name) {
                    local.translation = [0.0; 3];
                }
            }
        }
        if is_root {
            local.translation = [0.0; 3];
        }
        let w = match bone.parent.and_then(|p| world.get(p)) {
            Some(parent) => parent.then_child(&local),
            None => local,
        };
        world.push(w);
    }
    world
}

/// The node a group's `prn:` text key names (the group's parent name,
/// `TESAnimGroup` +0x30 `pParentName`, Xbox PDB): the text after `prn:`
/// (any case) and one space, as the text-key parser keeps it
/// (`005f3a20`). The weapon bone is moved under it when the weapon is
/// drawn or put away (`00923960`, `MiddleHighProcess::ReparentWeapon`,
/// Xbox PDB): `1hpequip.kf` names `Bip01 R Hand`, `1hpholster.kf` `Bip01
/// Pelvis`, `2hrholster.kf` `Bip01 Spine2`, `2hhholster.kf` `Bip01 R Hand`
/// (heavy weapons stay in hand).
// Translated from 005f3a20 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn weapon_parent(sequence: &Sequence) -> Option<&str> {
    sequence.text_keys.iter().rev().find_map(|(_, text)| {
        let head = text.get(..4)?;
        if !head.eq_ignore_ascii_case("prn:") {
            return None;
        }
        let rest = &text[4..];
        Some(rest.strip_prefix(' ').unwrap_or(rest))
    })
}

/// The skeleton's `Weapon` bone moved under `parent` (by name, any case):
/// its own transform kept relative to its new parent, as `NiNode::
/// AttachChild` keeps a child's local transform (`00923960` attaches the
/// weapon bone, vfunc +0xdc), and the bones under it carried along. A
/// parent the skeleton lacks leaves it where it is (the game logs the
/// missing `prn:` node and doesn't reparent).
pub fn reparent_weapon(skeleton: &[Bone], pose: &mut [Transform], parent: &str) {
    let index = |name: &str| {
        skeleton
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case(name))
    };
    let (Some(weapon), Some(to)) = (index("Weapon"), index(parent)) else {
        return;
    };
    let Some(from) = skeleton[weapon].parent else {
        return;
    };
    if from == to || weapon >= pose.len() || to >= pose.len() {
        return;
    }
    let local = pose[from].inverse().then_child(&pose[weapon]);
    move_weapon(skeleton, pose, weapon, pose[to].then_child(&local));
}

/// The `Weapon` bone placed at `at` (model space), the bones under it
/// following.
fn move_weapon(skeleton: &[Bone], pose: &mut [Transform], weapon: usize, at: Transform) {
    let was = pose[weapon].inverse();
    pose[weapon] = at;
    let under = |mut i: usize| {
        while let Some(p) = skeleton[i].parent {
            if p == weapon {
                return true;
            }
            i = p;
        }
        false
    };
    for i in 0..skeleton.len().min(pose.len()) {
        if i != weapon && under(i) {
            pose[i] = at.then_child(&was.then_child(&pose[i]));
        }
    }
}

/// A weapon put away: the holster pose (`<kind>holster.kf`, the `Holster`
/// group the game plays when it puts the weapon away at once, `00923960`)
/// gives the `Weapon` bone's transform, under the node its `prn:` key
/// names ([`weapon_parent`]): the spine for rifles, on the back; the
/// pelvis for pistols and melee weapons, at the hip; the right hand for
/// heavy weapons. A `prn:` naming no bone of the skeleton keeps the bone's
/// own parent. The rest of the pose is left as it is.
pub fn hang_weapon(skeleton: &[Bone], pose: &mut [Transform], holster: &Sequence) {
    let index = |name: &str| {
        skeleton
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case(name))
    };
    let Some(weapon) = index("Weapon") else {
        return;
    };
    let Some(track) = holster
        .tracks
        .iter()
        .find(|t| t.node.eq_ignore_ascii_case("Weapon"))
    else {
        return;
    };
    let parent = weapon_parent(holster)
        .and_then(index)
        .or(skeleton[weapon].parent);
    let Some(parent) = parent.filter(|&p| p < pose.len() && weapon < pose.len()) else {
        return;
    };
    let local = track.sample(holster.start).apply(&skeleton[weapon].local);
    move_weapon(skeleton, pose, weapon, pose[parent].then_child(&local));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Gamebryo 20.2.0.7 file (Bethesda 34) holding the given blocks and
    /// string table.
    fn file(blocks: &[(&str, Vec<u8>)], strings: &[&str]) -> Vec<u8> {
        let sized = |s: &str| {
            let mut v = (s.len() as u32).to_le_bytes().to_vec();
            v.extend(s.as_bytes());
            v
        };
        let mut types: Vec<&str> = Vec::new();
        for (t, _) in blocks {
            if !types.contains(t) {
                types.push(t);
            }
        }
        let mut out = b"Gamebryo File Format, Version 20.2.0.7\n".to_vec();
        out.extend(0x1402_0007u32.to_le_bytes());
        out.push(1);
        out.extend(11u32.to_le_bytes());
        out.extend((blocks.len() as u32).to_le_bytes());
        out.extend(34u32.to_le_bytes());
        for s in ["test", "", ""] {
            out.push(s.len() as u8 + 1);
            out.extend(s.as_bytes());
            out.push(0);
        }
        out.extend((types.len() as u16).to_le_bytes());
        for t in &types {
            out.extend(sized(t));
        }
        for (t, _) in blocks {
            out.extend((types.iter().position(|x| x == t).unwrap() as u16).to_le_bytes());
        }
        for (_, data) in blocks {
            out.extend((data.len() as u32).to_le_bytes());
        }
        out.extend((strings.len() as u32).to_le_bytes());
        out.extend((strings.iter().map(|s| s.len()).max().unwrap_or(0) as u32).to_le_bytes());
        for s in strings {
            out.extend(sized(s));
        }
        out.extend(0u32.to_le_bytes());
        for (_, data) in blocks {
            out.extend(data);
        }
        out.extend(1u32.to_le_bytes());
        out.extend(0i32.to_le_bytes());
        out
    }

    #[test]
    fn angle_curves_follow_their_tangents() {
        // The ceiling fan's spin: −62.83 rad over 10 s, tangents stored
        // (in, out) = (0, −62.83) and (−62.83, 0): an even turn.
        let end = -62.831_852;
        let key = |time, value, tangents| FloatKey {
            time,
            value,
            tangents: Some(tangents),
            hold: false,
        };
        let spin = [key(0.0, 0.0, (0.0, end)), key(10.0, end, (end, 0.0))];
        let at = |t| sample_curve(&spin, t).unwrap();
        assert!((at(2.5) - end / 4.0).abs() < 1e-4, "{}", at(2.5));
        assert!((at(5.0) - end / 2.0).abs() < 1e-4);
        assert_eq!(at(-1.0), 0.0);
        assert_eq!(at(11.0), end);
        // Tangents that ease in and out bend the curve.
        let eased = [key(0.0, 0.0, (0.0, 0.0)), key(10.0, 1.0, (0.0, 0.0))];
        assert!((sample_curve(&eased, 2.5).unwrap() - 0.15625).abs() < 1e-6);
        // Constant keys hold.
        let step = [
            FloatKey {
                time: 0.0,
                value: 1.0,
                tangents: None,
                hold: true,
            },
            FloatKey {
                time: 1.0,
                value: 2.0,
                tangents: None,
                hold: true,
            },
        ];
        assert_eq!(sample_curve(&step, 0.9), Some(1.0));
        // Turned into a rotation only after sampling: 2.5 s in the fan has
        // turned a quarter, not the nothing whole turns at the keys give.
        let track = Track {
            node: "FanBlades".into(),
            priority: 0,
            motion: Motion::Keys {
                translation: Vec::new(),
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: Some(Box::new([Vec::new(), Vec::new(), spin.to_vec()])),
            },
        };
        let m = quat_matrix(track.sample(0.25).rotation.unwrap());
        // −90° about Z takes x to −y.
        assert!(
            (m[1][0] + 1.0).abs() < 1e-4 && m[0][0].abs() < 1e-4,
            "{m:?}"
        );
    }

    #[test]
    fn text_keys_are_read_with_their_times() {
        // As the first-person `Pipboy.kf` names its moments.
        let strings = ["", "start", "Hit", "end"];
        let mut block = 0i32.to_le_bytes().to_vec();
        block.extend(3u32.to_le_bytes());
        for (time, key) in [(0.0f32, 1i32), (0.33, 2), (0.73, 3)] {
            block.extend(time.to_le_bytes());
            block.extend(key.to_le_bytes());
        }
        let nif = Nif::parse(file(&[("NiTextKeyExtraData", block)], &strings)).unwrap();
        assert_eq!(
            nif.text_keys().unwrap(),
            [
                (0.0, "start".to_string()),
                (0.33, "Hit".to_string()),
                (0.73, "end".to_string())
            ]
        );
    }

    #[test]
    fn a_put_away_weapon_hangs_from_the_holsters_prn_bone() {
        let bone = |name: &str, parent: Option<usize>, z: f32| Bone {
            name: name.into(),
            parent,
            local: Transform {
                translation: [0.0, 0.0, z],
                ..Transform::IDENTITY
            },
        };
        // Root, spine (10 up), hand (40 up the spine), the weapon in it.
        let bones = vec![
            bone("Bip01", None, 0.0),
            bone("Bip01 Spine2", Some(0), 10.0),
            bone("Bip01 R Hand", Some(1), 40.0),
            bone("Weapon", Some(2), 5.0),
        ];
        let at = |node: &str, p: [f32; 3]| Track {
            node: node.into(),
            priority: 0,
            motion: Motion::Keys {
                translation: vec![(0.0, p)],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
        };
        let holster = Sequence {
            name: "Holster".into(),
            start: 0.0,
            stop: 0.1,
            looping: false,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            // As `2hrholster.kf` has them.
            text_keys: vec![
                (0.0, "start".into()),
                (0.033, "Blend: 1".into()),
                (0.067, "prn: Bip01 Spine2".into()),
                (0.1, "end".into()),
            ],
            tracks: vec![
                at("Bip01", [0.0; 3]),
                at("Weapon", [0.0, -7.0, 0.0]),
                at("##ModelPart", [1.0; 3]),
            ],
        };
        assert_eq!(weapon_parent(&holster), Some("Bip01 Spine2"));
        let mut pose = posed(&bones, None, 0.0);
        assert_eq!(pose[3].translation, [0.0, 0.0, 55.0]);
        hang_weapon(&bones, &mut pose, &holster);
        // On the spine's back, not in the hand; the rest as it was.
        assert_eq!(pose[3].translation, [0.0, -7.0, 10.0]);
        assert_eq!(pose[2].translation, [0.0, 0.0, 50.0]);
        // Heavy weapons' holster names the hand (`2hhholster.kf`): it stays
        // there, at the file's transform; a name the skeleton lacks keeps
        // the bone's own parent too.
        for prn in ["Prn:Bip01 R Hand", "prn: Bip01 R Hand  -at none"] {
            let in_hand = Sequence {
                text_keys: vec![(0.0, prn.into())],
                ..holster.clone()
            };
            let mut pose = posed(&bones, None, 0.0);
            hang_weapon(&bones, &mut pose, &in_hand);
            assert_eq!(pose[3].translation, [0.0, -7.0, 50.0]);
        }
    }

    #[test]
    fn a_drawn_weapon_moves_under_the_equips_prn_bone_keeping_its_own_transform() {
        let bone = |name: &str, parent: Option<usize>, z: f32| Bone {
            name: name.into(),
            parent,
            local: Transform {
                translation: [0.0, 0.0, z],
                ..Transform::IDENTITY
            },
        };
        let bones = vec![
            bone("Bip01", None, 0.0),
            bone("Bip01 R ForeTwist", Some(0), 30.0),
            bone("Bip01 R Hand", Some(1), 18.0),
            bone("Weapon", Some(2), 6.0),
            bone("Under", Some(3), 1.0),
        ];
        let mut pose = posed(&bones, None, 0.0);
        reparent_weapon(&bones, &mut pose, "bip01 r foretwist");
        assert_eq!(pose[3].translation, [0.0, 0.0, 36.0]);
        assert_eq!(pose[4].translation, [0.0, 0.0, 37.0]);
        // Its own parent, or a bone the skeleton lacks: nothing moves.
        let before = posed(&bones, None, 0.0);
        let mut same = before.clone();
        reparent_weapon(&bones, &mut same, "Bip01 R Hand");
        reparent_weapon(&bones, &mut same, "Bip01 Nowhere");
        assert_eq!(same, before);
    }

    #[test]
    fn layers_over_a_base_keep_its_root_at_the_origin() {
        let bones = vec![
            Bone {
                name: "Bip01".into(),
                parent: None,
                local: Transform {
                    translation: [0.0, 0.0, 68.0],
                    ..Transform::IDENTITY
                },
            },
            Bone {
                name: "Weapon".into(),
                parent: Some(0),
                local: Transform::IDENTITY,
            },
        ];
        let still = |node: &str, at: [f32; 3]| Track {
            node: node.into(),
            priority: 0,
            motion: Motion::Keys {
                translation: vec![(0.0, at)],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
        };
        let idle = Sequence {
            name: "Idle".into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: Vec::new(),
            tracks: Vec::new(),
        };
        let holster = Sequence {
            name: "Holster".into(),
            tracks: vec![still("Weapon", [5.0, 0.0, 0.0])],
            looping: false,
            ..idle.clone()
        };
        let pose = posed_over(&bones, Some((&idle, 0.0)), &[(&holster, 0.0)]);
        // The root sits at the origin, as `posed` puts it.
        assert_eq!(pose[0].translation, [0.0; 3]);
        assert_eq!(pose[1].translation, [5.0, 0.0, 0.0]);
        // Without layers it's `posed`.
        assert_eq!(
            posed_over(&bones, Some((&idle, 0.0)), &[]),
            posed(&bones, Some(&idle), 0.0)
        );
    }

    #[test]
    fn later_layers_move_only_their_own_bones() {
        let bone = |name: &str, parent: Option<usize>| Bone {
            name: name.into(),
            parent,
            local: Transform::IDENTITY,
        };
        let skeleton = [
            bone("Root", None),
            bone("Arm", Some(0)),
            bone("Hand", Some(1)),
        ];
        let moves = |node: &str, x: f32| Track {
            node: node.into(),
            priority: 0,
            motion: Motion::Keys {
                translation: vec![(0.0, [x, 0.0, 0.0])],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
        };
        let sequence = |tracks: Vec<Track>| Sequence {
            name: String::new(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            tracks,
            accum_root: None,
            materials: Vec::new(),
            text_keys: Vec::new(),
        };
        let hold = sequence(vec![moves("Arm", 1.0), moves("Hand", 2.0)]);
        let attack = sequence(vec![moves("Hand", 5.0)]);
        let world = posed_layers(&skeleton, &[(&hold, 0.0), (&attack, 0.0)]);
        // The arm keeps the hold pose; the hand takes the attack's.
        assert_eq!(world[1].translation, [1.0, 0.0, 0.0]);
        assert_eq!(world[2].translation, [6.0, 0.0, 0.0]);
        assert_eq!(
            posed_layers(&skeleton, &[(&hold, 0.0)]),
            posed(&skeleton, Some(&hold), 0.0)
        );
    }

    #[test]
    fn spline_channels_with_the_unused_handle_have_no_points() {
        // More shorts than 0xFFFF, as in the gecko's idle: a channel whose
        // handle is 0xFFFF (with -FLT_MAX ranges) mustn't read them.
        let shorts = vec![16384i16; 70_000];
        let unused = spline_channel(&shorts, 0xFFFF, 1, 4, (-f32::MAX, f32::MAX));
        assert!(unused.is_empty());
        assert!(spline_channel(&shorts, u32::MAX, 1, 4, (0.0, 1.0)).is_empty());
        // A real one: offset + half × short / 32767.
        let used = spline_channel(&shorts, 10, 3, 2, (1.0, 2.0));
        assert_eq!(used.len(), 2);
        assert!((used[0][0] - (1.0 + 2.0 * 16384.0 / 32767.0)).abs() < 1e-6);
    }

    #[test]
    fn b_splines_start_and_end_on_their_end_points() {
        let points = [[0.0f32], [1.0], [3.0], [6.0], [10.0]];
        assert_eq!(bspline(&points, 0.0), Some([0.0]));
        assert!((bspline(&points, 1.0).unwrap()[0] - 10.0).abs() < 1e-5);
        let mid = bspline(&points, 0.5).unwrap()[0];
        assert!(mid > 1.0 && mid < 6.0, "{mid}");
        // A constant curve stays constant.
        let flat = [[2.0f32]; 6];
        assert!((bspline(&flat, 0.37).unwrap()[0] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn keys_interpolate_and_hold_at_the_ends() {
        let track = Track {
            node: "Bip01".into(),
            priority: 0,
            motion: Motion::Keys {
                translation: vec![(0.0, [0.0, 0.0, 0.0]), (1.0, [10.0, 0.0, 0.0])],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, Some([1.0, 0.0, 0.0, 0.0]), None),
                euler: None,
            },
        };
        assert_eq!(track.sample(0.5).translation, Some([5.0, 0.0, 0.0]));
        assert_eq!(track.sample(2.0).translation, Some([10.0, 0.0, 0.0]));
        assert_eq!(track.sample(0.5).rotation, Some([1.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn quaternions_turn_like_the_files_matrices() {
        // 90° about Z: x goes to y.
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = quat_matrix([s, 0.0, 0.0, s]);
        let v = crate::math::mat_vec(&m, [1.0, 0.0, 0.0]);
        assert!((v[1] - 1.0).abs() < 1e-6 && v[0].abs() < 1e-6, "{v:?}");
        let e = euler_quat([0.0, 0.0, std::f32::consts::FRAC_PI_2]);
        assert!((e[0] - s).abs() < 1e-6 && (e[3] - s).abs() < 1e-6, "{e:?}");
    }

    #[test]
    fn a_pose_moves_children_with_their_parents() {
        let skeleton = vec![
            Bone {
                name: "Root".into(),
                parent: None,
                local: Transform::IDENTITY,
            },
            Bone {
                name: "Arm".into(),
                parent: Some(0),
                local: Transform {
                    translation: [10.0, 0.0, 0.0],
                    ..Transform::IDENTITY
                },
            },
        ];
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let sequence = Sequence {
            name: "Turn".into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: None,
            materials: Vec::new(),
            text_keys: Vec::new(),
            tracks: vec![Track {
                node: "Root".into(),
                priority: 0,
                motion: Motion::Keys {
                    translation: Vec::new(),
                    rotation: vec![(0.0, [s, 0.0, 0.0, s])],
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        };
        let world = posed(&skeleton, Some(&sequence), 0.0);
        let arm = world[1].translation;
        assert!(
            (arm[1] - 10.0).abs() < 1e-5 && arm[0].abs() < 1e-5,
            "{arm:?}"
        );
    }
}
