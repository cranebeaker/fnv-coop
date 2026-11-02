//! V.A.T.S.'s animated cameras (`meshes\vatscameras\*.nif`, one per `CAMS`
//! record): a node whose transform a controller keys, with an `NiCamera`
//! as its first child whose field of view a `BSFrustumFOVController` may
//! key. Read as FalloutNV.exe reads and plays them (addresses in
//! `%USERPROFILE%\nv-re\findings\vats_camera_menu.md`):
//!
//! - the controllers' time (`NiTimeController`, `00a6cd60`): frequency ×
//!   the time + phase, then by the cycle type in the flags (bits 1–2): 0
//!   loop, 1 back and forth, 2 clamp to the controller's start and stop
//!   times; flag 0x10 plays it backwards. A controller without flag 0x01
//!   counts from time 0 (the V.A.T.S. camera clock), so its time is simply
//!   that;
//! - keys (`00a26b40` for floats, `00a27490` for positions): the two keys
//!   around the time, `u` = how far between them (0–1), and the key type's
//!   function: linear (`00a2a560`, `00a2a7f0`), quadratic (`00a26fe0`,
//!   `00a27c80`) or TCB (`00a2c610`, `00a2af70`), the last two as one cubic
//!   (`00a42070`): `P0 + u·(out0 + u·(3(P1−P0) − 2·out0 − in1 + u·(out0 +
//!   in1 − 2(P1−P0))))`; past the last key its value, before the first the
//!   cubic run backwards. Quadratic keys store `in` then `out` after the
//!   value (`00a27250`, `00a281e0`); TCB keys tension and two more numbers
//!   (`00a2c9d0`), from which their `in` and `out` are worked out when they
//!   load (`00a2c7a0`, `00a2b410`, `00a2c6a0`, `00a2b1d0`; see
//!   [`tcb_tangents`]);
//! - the field of view (`BSFrustumFOVController`, `00a42dc0`): a keyed
//!   value v makes the camera's frustum left −v, right v, top 0.75 v,
//!   bottom −0.75 v (near and far kept).

use crate::error::{Error, Result};
use crate::file::Nif;
use crate::math::Vec3;
use crate::reader::Reader;

/// How a controller turns the clock into its own time (`NiTimeController`
/// `+0x08` flags, `+0x0c` frequency, `+0x10` phase, `+0x14`/`+0x18` start
/// and stop).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerTiming {
    pub flags: u16,
    pub frequency: f32,
    pub phase: f32,
    pub start: f32,
    pub stop: f32,
}

impl ControllerTiming {
    /// The controller's time at clock time `t` with `phase` (the file's,
    /// or as the shot set it), as `00a6cd60` works it out for a controller
    /// counting from 0 (flag 0x01 clear): `frequency × t + phase`, then
    /// looped (cycle 0), bounced (1) or clamped (2) into start..stop, and
    /// turned around with flag 0x10.
    pub fn scaled_time(&self, t: f32, phase: f32) -> f32 {
        let (low, high) = (self.start, self.stop);
        let mut x = self.frequency * t + phase;
        let cycle = (self.flags >> 1) & 3;
        let span = high - low;
        if span != 0.0 {
            match cycle {
                0 => {
                    // `fmod`, kept above the start.
                    let mut f = (x - low) % span + low;
                    if f < low {
                        f += span;
                    }
                    x = f;
                }
                1 => {
                    let mut f = (x - low) % (2.0 * span);
                    if f < 0.0 {
                        f += 2.0 * span;
                    }
                    if f > span {
                        f = 2.0 * span - f;
                    }
                    x = f + low;
                }
                _ => {}
            }
        }
        if x > high {
            x = high;
        } else if x < low {
            x = low;
        }
        if self.flags & 0x10 != 0 {
            x = high - (x - low);
        }
        x
    }
}

/// A value keys carry.
pub trait KeyValue: Copy {
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn scale(self, k: f32) -> Self;
}

impl KeyValue for f32 {
    fn add(self, o: Self) -> Self {
        self + o
    }
    fn sub(self, o: Self) -> Self {
        self - o
    }
    fn scale(self, k: f32) -> Self {
        self * k
    }
}

impl KeyValue for Vec3 {
    fn add(self, o: Self) -> Self {
        [self[0] + o[0], self[1] + o[1], self[2] + o[2]]
    }
    fn sub(self, o: Self) -> Self {
        [self[0] - o[0], self[1] - o[1], self[2] - o[2]]
    }
    fn scale(self, k: f32) -> Self {
        [self[0] * k, self[1] * k, self[2] * k]
    }
}

/// Keys of one kind, as stored.
#[derive(Debug, Clone, PartialEq)]
pub enum Keys<T> {
    /// Type 1.
    Linear(Vec<(f32, T)>),
    /// Type 2: time, value, in, out.
    Quadratic(Vec<(f32, T, T, T)>),
    /// Type 3: time, value, and the three numbers after it (tension, then
    /// the two the game reads at `+0x0c` and `+0x10`), with `in` and `out`
    /// as [`tcb_tangents`] works them out.
    Tcb(Vec<(f32, T, [f32; 3], T, T)>),
    /// Type 5: a value held until the next key.
    Step(Vec<(f32, T)>),
}

/// A TCB key's `in` (DS) and `out` (DD) from its neighbours
/// (`00a2c6a0` for floats, `00a2b1d0` for positions): with d0 = P −
/// before, d1 = after − P, tension T and the key's next two numbers a, b:
/// `in = (1−T)/2 · ((1−a)(1+b)·d0 + (1+a)(1−b)·d1) · 2·dt0/(dt0+dt1)`,
/// `out = (1−T)/2 · ((1+a)(1+b)·d0 + (1−a)(1−b)·d1) · 2·dt1/(dt0+dt1)`,
/// dt0 and dt1 the times to the key before and after.
pub fn tcb_tangents<T: KeyValue>(
    p: T,
    before: T,
    after: T,
    tcb: [f32; 3],
    dt0: f32,
    dt1: f32,
) -> (T, T) {
    let d0 = p.sub(before);
    let d1 = after.sub(p);
    let half = (1.0 - tcb[0]) * 0.5;
    let up = half * (tcb[1] + 1.0);
    let down = half * (1.0 - tcb[1]);
    let (b_up, b_down) = (tcb[2] + 1.0, 1.0 - tcb[2]);
    let k = 2.0 / (dt1 + dt0);
    let incoming = d0
        .scale(down * b_up)
        .add(d1.scale(b_down * up))
        .scale(k * dt0);
    let outgoing = d1
        .scale(b_down * down)
        .add(d0.scale(b_up * up))
        .scale(k * dt1);
    (incoming, outgoing)
}

/// Fills TCB keys' `in` and `out` as the game does when they load
/// (`00a2c7a0`, `00a2b410`): the first key against a point as far before
/// it as the second is after, the last likewise, both with unit intervals;
/// the others from their neighbours and the times between.
fn fill_tcb<T: KeyValue>(keys: &mut [(f32, T, [f32; 3], T, T)]) {
    let n = keys.len();
    if n < 2 {
        return;
    }
    let tangents: Vec<(T, T)> = (0..n)
        .map(|i| {
            let (t, p, tcb, _, _) = keys[i];
            if i == 0 {
                let after = keys[1].1;
                tcb_tangents(p, p.scale(2.0).sub(after), after, tcb, 1.0, 1.0)
            } else if i == n - 1 {
                let before = keys[n - 2].1;
                tcb_tangents(p, before, p.scale(2.0).sub(before), tcb, 1.0, 1.0)
            } else {
                let (t0, before) = (keys[i - 1].0, keys[i - 1].1);
                let (t1, after) = (keys[i + 1].0, keys[i + 1].1);
                tcb_tangents(p, before, after, tcb, t - t0, t1 - t)
            }
        })
        .collect();
    for (k, (i, o)) in keys.iter_mut().zip(tangents) {
        k.3 = i;
        k.4 = o;
    }
}

/// The cubic of quadratic and TCB keys (`00a42070`, `00a27c80`): from P0
/// with tangent `out0` to P1 with `in1` over u in 0..1.
fn hermite<T: KeyValue>(u: f32, p0: T, out0: T, p1: T, in1: T) -> T {
    let d = p1.sub(p0);
    let c2 = d.scale(3.0).sub(out0.scale(2.0).add(in1));
    let c3 = in1.add(out0).sub(d.scale(2.0));
    p0.add(out0.add(c2.add(c3.scale(u)).scale(u)).scale(u))
}

impl<T: KeyValue> Keys<T> {
    pub fn len(&self) -> usize {
        match self {
            Keys::Linear(k) | Keys::Step(k) => k.len(),
            Keys::Quadratic(k) => k.len(),
            Keys::Tcb(k) => k.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn time(&self, i: usize) -> f32 {
        match self {
            Keys::Linear(k) | Keys::Step(k) => k[i].0,
            Keys::Quadratic(k) => k[i].0,
            Keys::Tcb(k) => k[i].0,
        }
    }

    fn value(&self, i: usize) -> T {
        match self {
            Keys::Linear(k) | Keys::Step(k) => k[i].1,
            Keys::Quadratic(k) => k[i].1,
            Keys::Tcb(k) => k[i].1,
        }
    }

    /// The value at time `t` (`00a26b40`, `00a27490`).
    pub fn sample(&self, t: f32) -> Option<T> {
        let n = self.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            return Some(self.value(0));
        }
        // The first key after or at t (searching from the first).
        let hi = (1..n).find(|&i| t <= self.time(i));
        let Some(hi) = hi else {
            return Some(self.value(n - 1));
        };
        let lo = hi - 1;
        let (t0, t1) = (self.time(lo), self.time(hi));
        let u = (t - t0) / (t1 - t0);
        Some(match self {
            Keys::Linear(k) => k[lo].1.scale(1.0 - u).add(k[hi].1.scale(u)),
            // A step holds the earlier key until the later one is reached.
            Keys::Step(k) => {
                if u < 1.0 {
                    k[lo].1
                } else {
                    k[hi].1
                }
            }
            Keys::Quadratic(k) => hermite(u, k[lo].1, k[lo].3, k[hi].1, k[hi].2),
            Keys::Tcb(k) => hermite(u, k[lo].1, k[lo].4, k[hi].1, k[hi].3),
        })
    }
}

/// A camera's frustum (`NiCamera`): left, right, top, bottom, near, far.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frustum {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
    pub ortho: bool,
}

/// One V.A.T.S. camera model.
#[derive(Debug, Clone, PartialEq)]
pub struct CameraModel {
    /// The top node's name and its own translation (kept where no keys
    /// move it).
    pub root_name: String,
    pub root_translation: Vec3,
    /// The top node's keyed translation and its controller.
    pub translation: Option<(ControllerTiming, Keys<Vec3>)>,
    /// The top node's keyed rotation (same controller). With any rotation
    /// keys the game turns its camera by the node's angles on top of
    /// looking at the shot's target (`0058c5d0` sets the shot's `+0x74`,
    /// `0058d510` reads the node's world rotation as X·Y·Z angles,
    /// `0094ae40` multiplies them onto the look-at frame).
    pub rotation: Option<(ControllerTiming, Rotation)>,
    /// The camera's frustum as stored, and its keyed field of view.
    pub frustum: Frustum,
    pub fov: Option<(ControllerTiming, Keys<f32>)>,
}

/// A node's keyed rotation.
#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    /// Key type 4: three float key groups, the X, Y and Z angles (radians;
    /// 71 of the 73 V.A.T.S. cameras with rotation keys).
    Euler([Option<Keys<f32>>; 3]),
    /// Quaternion keys (types 1–3), counted only: their interpolation
    /// isn't read here (2 cameras: `GrenadeThrowActionCam`,
    /// `TargetHandyCamRT`).
    Quaternion { kind: u32, keys: usize },
}

/// One key group as the file stores it (count, then a type and the keys).
fn read_keys<T: KeyValue>(
    r: &mut Reader,
    value: &dyn Fn(&mut Reader) -> Result<T>,
    size: usize,
) -> Result<Option<Keys<T>>> {
    let n = r.u32("a key count")? as usize;
    if n == 0 {
        return Ok(None);
    }
    let kind = r.u32("a key type")?;
    let per_key = 4 + size * if kind == 2 { 3 } else { 1 } + if kind == 3 { 12 } else { 0 };
    if n.saturating_mul(per_key) > r.remaining() {
        return Err(Error::Malformed {
            offset: r.offset(),
            reason: format!("{n} keys, more than the data holds"),
        });
    }
    Ok(Some(match kind {
        2 => Keys::Quadratic(
            (0..n)
                .map(|_| {
                    let t = r.f32("a key time")?;
                    let v = value(r)?;
                    let i = value(r)?;
                    let o = value(r)?;
                    Ok((t, v, i, o))
                })
                .collect::<Result<_>>()?,
        ),
        3 => {
            let mut keys: Vec<(f32, T, [f32; 3], T, T)> = (0..n)
                .map(|_| {
                    let t = r.f32("a key time")?;
                    let v = value(r)?;
                    let tcb = [
                        r.f32("a tension")?,
                        r.f32("a TCB number")?,
                        r.f32("a TCB number")?,
                    ];
                    Ok((t, v, tcb, v, v))
                })
                .collect::<Result<_>>()?;
            fill_tcb(&mut keys);
            Keys::Tcb(keys)
        }
        5 => Keys::Step(
            (0..n)
                .map(|_| Ok((r.f32("a key time")?, value(r)?)))
                .collect::<Result<_>>()?,
        ),
        _ => Keys::Linear(
            (0..n)
                .map(|_| Ok((r.f32("a key time")?, value(r)?)))
                .collect::<Result<_>>()?,
        ),
    }))
}

/// `NiTimeController`'s fields (after the next-controller reference):
/// flags, frequency, phase, start, stop, target; then a single
/// interpolator controller's interpolator.
fn read_controller(r: &mut Reader) -> Result<(ControllerTiming, i32, i32)> {
    r.i32("the next controller")?;
    let flags = r.u16("the controller flags")?;
    let frequency = r.f32("the frequency")?;
    let phase = r.f32("the phase")?;
    let start = r.f32("the start time")?;
    let stop = r.f32("the stop time")?;
    let target = r.i32("the target")?;
    let interpolator = r.i32("the interpolator")?;
    Ok((
        ControllerTiming {
            flags,
            frequency,
            phase,
            start,
            stop,
        },
        target,
        interpolator,
    ))
}

impl Nif {
    /// The V.A.T.S. camera in this file: the first root node, its keyed
    /// translation, and its first child as the camera (`0058c5d0` takes
    /// the first child, and needs it to be an `NiCamera`).
    pub fn camera_model(&self) -> Result<Option<CameraModel>> {
        let Some(root) = self.roots().first().and_then(|&r| self.reference(r)) else {
            return Ok(None);
        };
        let crate::blocks::Block::Node(node) = self.block(root)? else {
            return Ok(None);
        };
        let Some(camera) = node.children.first().and_then(|&c| self.reference(c)) else {
            return Ok(None);
        };
        if self.block_type(camera) != "NiCamera" {
            return Ok(None);
        }
        let Some((frustum, camera_controller)) = self.camera_frustum(camera)? else {
            return Ok(None);
        };
        // The top node's controller: a transform controller with keys.
        let (translation, rotation) = self.keyed_transform(node.av.net.controller)?;
        let fov = self.keyed_fov(camera_controller)?;
        Ok(Some(CameraModel {
            root_name: node.av.net.name.clone(),
            root_translation: node.av.transform.translation,
            translation,
            rotation,
            frustum,
            fov,
        }))
    }

    /// An `NiCamera` block's frustum and its controller, as stored (its
    /// object fields read as a node's first).
    pub fn camera_frustum(&self, camera: usize) -> Result<Option<(Frustum, i32)>> {
        if self.block_type(camera) != "NiCamera" {
            return Ok(None);
        }
        let mut r = self.reader(camera);
        r.i32("the name")?;
        let extra = r.u32("the extra data count")? as usize;
        r.take(extra * 4, "extra data")?;
        let camera_controller = r.i32("the controller")?;
        if self.header().bs_version > 26 {
            r.u32("the flags")?;
        } else {
            r.u16("the flags")?;
        }
        r.take(12 + 36 + 4, "the transform")?;
        let properties = r.u32("the property count")? as usize;
        r.take(properties * 4, "properties")?;
        r.i32("the collision object")?;
        r.u16("the camera flags")?;
        let frustum = Frustum {
            left: r.f32("the frustum")?,
            right: r.f32("the frustum")?,
            top: r.f32("the frustum")?,
            bottom: r.f32("the frustum")?,
            near: r.f32("the frustum")?,
            far: r.f32("the frustum")?,
            ortho: r.bool("the projection")?,
        };
        Ok(Some((frustum, camera_controller)))
    }

    /// The top node's keyed translation and rotation.
    #[allow(clippy::type_complexity)]
    fn keyed_transform(
        &self,
        controller: i32,
    ) -> Result<(
        Option<(ControllerTiming, Keys<Vec3>)>,
        Option<(ControllerTiming, Rotation)>,
    )> {
        let Some(c) = self.reference(controller) else {
            return Ok((None, None));
        };
        if !matches!(
            self.block_type(c),
            "NiTransformController" | "NiKeyframeController"
        ) {
            return Ok((None, None));
        }
        let (timing, _, interpolator) = read_controller(&mut self.reader(c))?;
        let Some(i) = self
            .reference(interpolator)
            .filter(|&i| self.block_type(i) == "NiTransformInterpolator")
        else {
            return Ok((None, None));
        };
        let mut r = self.reader(i);
        r.take(12 + 16 + 4, "the interpolator's own transform")?;
        let data = r.i32("the transform data")?;
        let Some(d) = self
            .reference(data)
            .filter(|&d| matches!(self.block_type(d), "NiTransformData" | "NiKeyframeData"))
        else {
            return Ok((None, None));
        };
        let mut r = self.reader(d);
        // Rotations come first.
        let rotations = r.u32("the rotation key count")? as usize;
        let mut rotation = None;
        if rotations > 0 {
            let kind = r.u32("the rotation key type")?;
            if kind == 4 {
                let x = read_keys::<f32>(&mut r, &|r| r.f32("an angle"), 4)?;
                let y = read_keys::<f32>(&mut r, &|r| r.f32("an angle"), 4)?;
                let z = read_keys::<f32>(&mut r, &|r| r.f32("an angle"), 4)?;
                rotation = Some((timing, Rotation::Euler([x, y, z])));
            } else {
                let per = 4 + 16 + if kind == 3 { 12 } else { 0 };
                r.take(rotations * per, "rotation keys")?;
                rotation = Some((
                    timing,
                    Rotation::Quaternion {
                        kind,
                        keys: rotations,
                    },
                ));
            }
        }
        let keys = read_keys::<Vec3>(&mut r, &|r| r.vec3("a translation"), 12)?;
        Ok((keys.map(|k| (timing, k)), rotation))
    }

    fn keyed_fov(&self, controller: i32) -> Result<Option<(ControllerTiming, Keys<f32>)>> {
        let Some(c) = self.reference(controller) else {
            return Ok(None);
        };
        if self.block_type(c) != "BSFrustumFOVController" {
            return Ok(None);
        }
        let (timing, _, interpolator) = read_controller(&mut self.reader(c))?;
        let Some(i) = self
            .reference(interpolator)
            .filter(|&i| self.block_type(i) == "NiFloatInterpolator")
        else {
            return Ok(None);
        };
        let mut r = self.reader(i);
        r.f32("the interpolator's own value")?;
        let data = r.i32("the float data")?;
        let Some(d) = self
            .reference(data)
            .filter(|&d| self.block_type(d) == "NiFloatData")
        else {
            return Ok(None);
        };
        let keys = read_keys::<f32>(&mut self.reader(d), &|r| r.f32("a value"), 4)?;
        Ok(keys.map(|k| (timing, k)))
    }
}

/// A model's top node with its keyed rotation (the weapon wobbles,
/// `meshes\characters\weaponwobbles\*.nif`: a node named for the bone it
/// turns, keyed by X, Y, Z angles).
#[derive(Debug, Clone, PartialEq)]
pub struct KeyedNode {
    pub name: String,
    pub rotation: Option<(ControllerTiming, Rotation)>,
}

impl Nif {
    /// The first root node and its keyed rotation, when it's a node.
    pub fn keyed_root(&self) -> Result<Option<KeyedNode>> {
        let Some(root) = self.roots().first().and_then(|&r| self.reference(r)) else {
            return Ok(None);
        };
        let crate::blocks::Block::Node(node) = self.block(root)? else {
            return Ok(None);
        };
        let (_, rotation) = self.keyed_transform(node.av.net.controller)?;
        Ok(Some(KeyedNode {
            name: node.av.net.name.clone(),
            rotation,
        }))
    }
}

impl KeyedNode {
    /// The keyed X, Y, Z angles at clock `t` (the controller's own time
    /// worked out as [`ControllerTiming::scaled_time`] does); `None` without
    /// Euler keys.
    pub fn angles_at(&self, t: f32) -> Option<Vec3> {
        let (timing, Rotation::Euler(axes)) = self.rotation.as_ref()? else {
            return None;
        };
        let time = timing.scaled_time(t, timing.phase);
        Some([0, 1, 2].map(|i| axes[i].as_ref().and_then(|k| k.sample(time)).unwrap_or(0.0)))
    }

    /// The controller's start time (`NiTimeController` +0x14).
    pub fn start(&self) -> f32 {
        self.rotation.as_ref().map_or(0.0, |(t, _)| t.start)
    }
}

impl CameraModel {
    /// The top node's translation at camera clock `t`, the controllers'
    /// phases moved by `phase_shift` (a shot that starts at time zero
    /// shifts every controller's phase by minus the clock when it starts:
    /// `004b70f0`, `00621b20`).
    pub fn translation_at(&self, t: f32, phase_shift: Option<f32>) -> Vec3 {
        match &self.translation {
            Some((timing, keys)) => {
                let phase = phase_shift.unwrap_or(timing.phase);
                keys.sample(timing.scaled_time(t, phase))
                    .unwrap_or(self.root_translation)
            }
            None => self.root_translation,
        }
    }

    /// The top node's keyed X, Y, Z angles at clock `t` (each axis's float
    /// keys as `00a26b40` samples them; an axis without keys 0), for a
    /// camera with Euler rotation keys. `None` without rotation keys or
    /// with quaternion ones (see [`Rotation::Quaternion`]).
    pub fn angles_at(&self, t: f32, phase_shift: Option<f32>) -> Option<Vec3> {
        let (timing, Rotation::Euler(axes)) = self.rotation.as_ref()? else {
            return None;
        };
        let time = timing.scaled_time(t, phase_shift.unwrap_or(timing.phase));
        Some([0, 1, 2].map(|i| axes[i].as_ref().and_then(|k| k.sample(time)).unwrap_or(0.0)))
    }

    /// The frustum at clock `t` (the keyed field of view, when there is
    /// one, as `00a42dc0` sets it).
    pub fn frustum_at(&self, t: f32, phase_shift: Option<f32>) -> Frustum {
        let mut f = self.frustum;
        if let Some((timing, keys)) = &self.fov {
            let phase = phase_shift.unwrap_or(timing.phase);
            if let Some(v) = keys.sample(timing.scaled_time(t, phase)) {
                f.left = -v;
                f.right = v;
                f.top = 0.75 * v;
                f.bottom = -0.75 * v;
            }
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_controllers_hold_at_their_ends() {
        let c = ControllerTiming {
            flags: 0x4c,
            frequency: 1.0,
            phase: 0.0,
            start: 0.0,
            stop: 2.0,
        };
        assert_eq!(c.scaled_time(1.5, 0.0), 1.5);
        assert_eq!(c.scaled_time(3.0, 0.0), 2.0);
        // A shot starting at time zero: the phase is minus the clock then.
        assert_eq!(c.scaled_time(5.25, -5.0), 0.25);
        // Looping (cycle 0).
        let l = ControllerTiming { flags: 0x08, ..c };
        assert!((l.scaled_time(2.5, 0.0) - 0.5).abs() < 1e-6);
        // Back and forth (cycle 1).
        let b = ControllerTiming { flags: 0x0a, ..c };
        assert!((b.scaled_time(2.5, 0.0) - 1.5).abs() < 1e-6);
    }

    #[test]
    fn linear_and_step_keys() {
        let k = Keys::Linear(vec![(0.0, 0.0f32), (1.0, 10.0), (3.0, 30.0)]);
        assert_eq!(k.sample(0.5), Some(5.0));
        assert_eq!(k.sample(2.0), Some(20.0));
        // Past the last key: its value.
        assert_eq!(k.sample(9.0), Some(30.0));
        // Before the first: the first interval run backwards.
        assert_eq!(k.sample(-1.0), Some(-10.0));
        let s = Keys::Step(vec![(0.0, 1.0f32), (1.0, 2.0)]);
        assert_eq!(s.sample(0.99), Some(1.0));
        assert_eq!(s.sample(1.0), Some(2.0));
    }

    #[test]
    fn quadratic_keys_follow_their_tangents() {
        // in/out as stored: a key's own `out` leaves it, the next key's
        // `in` arrives at it.
        let k = Keys::Quadratic(vec![(0.0, 0.0f32, 0.0, 3.0), (1.0, 1.0, 0.0, 0.0)]);
        // P(u) = u·3 + u²(3 − 6 − 0) + u³(3 + 0 − 2) = 3u − 3u² + u³.
        let u = 0.5f32;
        let want = 3.0 * u - 3.0 * u * u + u * u * u;
        assert!((k.sample(0.5).unwrap() - want).abs() < 1e-6);
        assert_eq!(k.sample(1.0), Some(1.0));
    }

    #[test]
    fn tcb_keys_with_zero_numbers_pass_through_their_points() {
        let mut keys = vec![
            (0.0, [0.0f32, 0.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3]),
            (1.0, [10.0, 0.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3]),
            (3.0, [20.0, 0.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3]),
        ];
        fill_tcb(&mut keys);
        // The middle key's tangents: (1/2)(d0 + d1) scaled by its
        // intervals: in = 10 × 2·1/3, out = 10 × 2·2/3.
        assert!((keys[1].3[0] - 20.0 / 3.0).abs() < 1e-5);
        assert!((keys[1].4[0] - 40.0 / 3.0).abs() < 1e-5);
        // The first key against a point 10 before it: both tangents 10.
        assert!((keys[0].4[0] - 10.0).abs() < 1e-5);
        let k = Keys::Tcb(keys);
        assert_eq!(k.sample(1.0), Some([10.0, 0.0, 0.0]));
        assert_eq!(k.sample(3.0), Some([20.0, 0.0, 0.0]));
        let mid = k.sample(0.5).unwrap()[0];
        assert!(mid > 0.0 && mid < 10.0);
    }

    #[test]
    fn tcb_numbers_as_the_game_uses_them() {
        // Tension 1 flattens both tangents.
        let (i, o) = tcb_tangents(1.0f32, 0.0, 3.0, [1.0, 0.0, 0.0], 1.0, 1.0);
        assert_eq!((i, o), (0.0, 0.0));
        // The second number (+0x0c) at 1: `in` takes only the step after
        // the key (d1 = 2), `out` only the step before it (d0 = 1).
        let (i, o) = tcb_tangents(1.0f32, 0.0, 3.0, [0.0, 1.0, 0.0], 1.0, 1.0);
        assert!((i - 2.0).abs() < 1e-6 && (o - 1.0).abs() < 1e-6);
    }

    #[test]
    fn euler_rotation_keys_are_sampled_per_axis_on_the_controller_clock() {
        let timing = ControllerTiming {
            flags: 0x4c,
            frequency: 1.0,
            phase: 0.0,
            start: 0.0,
            stop: 2.0,
        };
        let cam = CameraModel {
            root_name: "c".into(),
            root_translation: [0.0; 3],
            translation: None,
            rotation: Some((
                timing,
                Rotation::Euler([
                    Some(Keys::Linear(vec![(0.0, 0.0), (2.0, 1.0)])),
                    None,
                    Some(Keys::Linear(vec![(0.0, -0.5)])),
                ]),
            )),
            frustum: Frustum {
                left: -1.0,
                right: 1.0,
                top: 0.75,
                bottom: -0.75,
                near: 1.0,
                far: 5000.0,
                ortho: false,
            },
            fov: None,
        };
        assert_eq!(cam.angles_at(1.0, None), Some([0.5, 0.0, -0.5]));
        // Clamped at the controller's stop.
        assert_eq!(cam.angles_at(9.0, None), Some([1.0, 0.0, -0.5]));
        // Started at time zero by a shot beginning at clock 4.
        assert_eq!(cam.angles_at(5.0, Some(-4.0)), Some([0.5, 0.0, -0.5]));
        let quats = CameraModel {
            rotation: Some((timing, Rotation::Quaternion { kind: 3, keys: 3 })),
            ..cam
        };
        assert_eq!(quats.angles_at(1.0, None), None);
    }
}
