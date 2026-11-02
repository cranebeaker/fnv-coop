//! Native head and eye tracking ("LookIK") of `bhkRagdollController`
//! (Xbox PDB), translated from FalloutNV.exe 1.4.0.525.
//!
//! The controller works on Havok poses (`hkaPose`, Xbox PDB): one built
//! from the actor's scene graph (`pSGPose`, controller +0x88) and, for
//! actors that are not creatures, a two-bone "head pose" (`pHeadPose`,
//! +0x54) whose second bone is the eye point. Each update the head bone
//! is turned toward the target (`BoneTrack`, 00c78610), capped per update
//! (`LimitBoneRot`, 00c755e0), and the eye bone is then solved inside the
//! head's frame; its direction gives the eye heading and pitch.
//!
//! Field names come from the Xbox 360 prototype's PDB; the PC offsets in
//! the comments were matched against the PC constructor (00c7f060) and
//! the functions below. Positions are in Havok units: game units times
//! [`havok_scale`].
//!
//! Precision: the native code uses SSE `rsqrtss`/`rcpps` estimates with
//! one Newton step (CPU tier C) and x87 transcendental calls (tier D);
//! these are reproduced with exact reciprocals, so results agree to a few
//! ULP, not bit for bit. See docs/OPENING_LOOK_IK.md.

pub type V4 = [f32; 4];

/// `hkQsTransform`: translation, rotation (x, y, z, w) and scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Qs {
    pub t: V4,
    pub q: V4,
    pub s: V4,
}

impl Qs {
    pub const IDENTITY: Qs = Qs {
        t: [0.0; 4],
        q: QUAT_IDENTITY,
        s: [1.0; 4],
    };
}

/// 010c71b0: the identity quaternion constant.
pub const QUAT_IDENTITY: V4 = [0.0, 0.0, 0.0, 1.0];

/// 011b02ec (6.9991255): game units per Havok unit; 00fbcd50 stores its
/// reciprocal at 01267c20 and 00f3d3d0 the same value at 011c582c.
pub const HAVOK_UNITS: f32 = 6.999_125_5;

pub fn havok_scale() -> f32 {
    (1.0f64 / HAVOK_UNITS as f64) as f32
}

/// 01023128: degrees to radians, the double holding f32(pi / 180).
const DEG_TO_RAD: f64 = 0.017_453_292_384_743_69;
/// 01030f38: f32(pi / 2) as a double.
const HALF_PI: f64 = 1.570_796_370_506_286_6;
/// 0102b3c8: f32(pi).
const PI_F32: f32 = std::f32::consts::PI;
/// 01017d00: the "unchanged" tolerance of quaternion comparisons.
const SAME: f32 = 0.001;
/// 011b05a8: largest distance of the stored target from the head
/// (Havok units), 00c78160.
const TARGET_REACH: f32 = 5.0;
/// 0101712c / 0102caf8: sideways offset used for targets behind the
/// actor, 00c78160.
const BEHIND_SIDE: f32 = 5.0;
/// 01017718: distance ahead of the head at which an easing-out target is
/// put, 00c78160.
const EASE_AHEAD: f32 = 3.0;
/// 01050c98 / 01050c48: eye point in the head's frame, game units
/// (00c7de60, before Havok scaling).
const EYE_OFFSET: [f32; 2] = [9.0, 6.0];
/// 010c4df8: the constructor's `fLookAtMaxAngle` (60 degrees), 00c7f060.
#[allow(clippy::approx_constant)] // the stored f32, 0x3f860a92
const DEFAULT_MAX_ANGLE: f32 = 1.047_197_6;

/// The LookIK settings (`[LookIK]`/`[RagdollAnim]` INI settings).
/// Defaults are the executable's constructor values; the shipped INI
/// files set none of them (`fLookAtGain`/`fLookAtTargetGain` there are
/// not registered by FalloutNV.exe).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `fAngleMax:LookIK` (01267d24), degrees per update.
    pub angle_max: f32,
    /// `fAngleMaxEase:LookIK` (01267d30), degrees per update while
    /// easing out.
    pub angle_max_ease: f32,
    /// `fEaseAngleShutOff:LookIK` (01267d3c), degrees.
    pub ease_angle_shut_off: f32,
    /// `fMaxTrackingDist:LookIK` (01267d0c), game units.
    pub max_tracking_dist: f32,
    /// `fMinTrackingDist:LookIK` (01267d18), game units.
    pub min_tracking_dist: f32,
    /// `bLookIK:RagdollAnim` (01267c4c).
    pub look_ik: bool,
    /// `bRagdollAnim:RagdollAnim` (01267c28).
    pub ragdoll_anim: bool,
}

impl Default for Settings {
    /// Constructors 00fbd0a0..00fbd160 and 00fbcd70..00fbcde0.
    fn default() -> Self {
        Settings {
            angle_max: 3.5,
            angle_max_ease: 1.0,
            ease_angle_shut_off: 0.5,
            max_tracking_dist: 1500.0,
            min_tracking_dist: 12.0,
            look_ik: true,
            ragdoll_anim: true,
        }
    }
}

impl Settings {
    /// The defaults with any INI value (`float(section, key)`; booleans
    /// as numbers). Ported from the contributor branch
    /// `playcon/claude/lookik-head-tracking` (da055f5).
    pub fn read(float: impl Fn(&str, &str) -> Option<f32>) -> Settings {
        let d = Settings::default();
        let f = |key: &str, v: f32| float("LookIK", key).unwrap_or(v);
        let b = |key: &str, v: bool| float("RagdollAnim", key).map_or(v, |x| x != 0.0);
        Settings {
            angle_max: f("fAngleMax", d.angle_max),
            angle_max_ease: f("fAngleMaxEase", d.angle_max_ease),
            ease_angle_shut_off: f("fEaseAngleShutOff", d.ease_angle_shut_off),
            max_tracking_dist: f("fMaxTrackingDist", d.max_tracking_dist),
            min_tracking_dist: f("fMinTrackingDist", d.min_tracking_dist),
            look_ik: b("bLookIK", d.look_ik),
            ragdoll_anim: b("bRagdollAnim", d.ragdoll_anim),
        }
    }
}

/// The FaceGen eye-tracking game settings the look controller takes when
/// a head is attached (00607420), degrees. Registered by 00f80fc0,
/// 00f80ff0, 00f81020 and 00f81050; `GMST` records override them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrackSettings {
    /// `fTrackEyeXY` (28).
    pub eye_xy: f32,
    /// `fTrackEyeZ` (20).
    pub eye_z: f32,
    /// `fTrackDeadZoneXY` (20).
    pub dead_zone_xy: f32,
    /// `fTrackDeadZoneZ` (10).
    pub dead_zone_z: f32,
}

impl Default for TrackSettings {
    fn default() -> Self {
        TrackSettings {
            eye_xy: 28.0,
            eye_z: 20.0,
            dead_zone_xy: 20.0,
            dead_zone_z: 10.0,
        }
    }
}

/// 01064f50 / 0101ff38: the 90-degree cap of 00649f00 and its relatives.
const RIGHT_ANGLE: f32 = std::f32::consts::FRAC_PI_2;

/// Degrees to radians kept within 0..90 degrees, as 00649f00/00649f70 do
/// (`x * 0.0174533`, below 0 is 0, above pi/2 is pi/2).
fn range_radians(degrees: f32) -> f32 {
    ((degrees as f64 * DEG_TO_RAD) as f32).clamp(0.0, RIGHT_ANGLE)
}

impl TrackSettings {
    /// The defaults with any `GMST` value (`gmst(name)`).
    pub fn read(gmst: impl Fn(&str) -> Option<f32>) -> TrackSettings {
        let d = TrackSettings::default();
        TrackSettings {
            eye_xy: gmst("fTrackEyeXY").unwrap_or(d.eye_xy),
            eye_z: gmst("fTrackEyeZ").unwrap_or(d.eye_z),
            dead_zone_xy: gmst("fTrackDeadZoneXY").unwrap_or(d.dead_zone_xy),
            dead_zone_z: gmst("fTrackDeadZoneZ").unwrap_or(d.dead_zone_z),
        }
    }

    /// 00649f00: the eye's heading range, radians.
    pub fn eye_xy(&self) -> f32 {
        range_radians(self.eye_xy)
    }

    /// 00649f70: the eye's pitch range, radians.
    pub fn eye_z(&self) -> f32 {
        range_radians(self.eye_z)
    }

    /// 00649fe0: the heading dead zone, radians, at most the range.
    pub fn dead_zone_xy(&self) -> f32 {
        range_radians(self.dead_zone_xy).min(self.eye_xy())
    }

    /// 0064a070: the pitch dead zone, radians, at most the range.
    pub fn dead_zone_z(&self) -> f32 {
        range_radians(self.dead_zone_z).min(self.eye_z())
    }
}

/// Whom an actor looks at this update, game units: the target's look
/// anchor (its virtual +0x194, see [`actor_anchor`]) and its position
/// (virtual +0x1f4), from which the tracking distance is measured.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    pub anchor: [f32; 3],
    pub position: [f32; 3],
}

/// The bone an actor's look anchor is (`Actor` virtual +0x194,
/// 008a2fa0): a character's bone cache (`Character` +0x1b4, filled by
/// 004aad00 from the names at 01188b74) has `Bip01 Head` first; a
/// creature's is found by that name (011c61ac) through its
/// `NiControllerManager`'s object palette. Traced on the contributor
/// branch `playcon/claude/lookik-actor-anchor` (5dfb5d9), re-checked
/// 2026-10-06.
pub const ANCHOR_BONE: &str = "Bip01 Head";

/// 0106b9e8: how far up an actor without that bone is looked at, as a
/// share of its height (008a30a9).
pub const ANCHOR_HEIGHT_SHARE: f64 = 0.9;

/// Where an actor other than the player in first person is looked at
/// (008a2fa0), game units: its head node's world position (`head`), with
/// z replaced by its own look controller's eye point (`eye_z`, 00c757b0,
/// only when that controller has a head pose, +0x190); without the node,
/// `position` raised by 0.9 x `height` (008853a0, see [`actor_height`]).
pub fn actor_anchor(
    head: Option<[f32; 3]>,
    eye_z: Option<f32>,
    position: [f32; 3],
    height: f32,
) -> [f32; 3] {
    match head {
        Some([x, y, z]) => [x, y, eye_z.unwrap_or(z)],
        None => [
            position[0],
            position[1],
            (height as f64 * ANCHOR_HEIGHT_SHARE + position[2] as f64) as f32,
        ],
    }
}

/// An actor's height for [`actor_anchor`] (008853a0): the z extent of its
/// bounds (virtual +0x1dc max less +0x1d8 min) times its scale (00567400).
pub fn actor_height(bound_min_z: f32, bound_max_z: f32, scale: f32) -> f32 {
    (bound_max_z - bound_min_z) * scale
}

// ---------------------------------------------------------------------
// Vector and quaternion helpers, in the native operation order.

/// The SSE reciprocal square root with one Newton step and its zero guard
/// (`0.5 * r * (3 - x * r * r)`, 0 when `x == 0`), as inlined throughout.
/// Tier C: `r` starts from the exact reciprocal root, not `rsqrtss`.
pub fn rsqrt_nr(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }
    let r = (1.0 / (x as f64).sqrt()) as f32;
    0.5 * r * (3.0 - x * r * r)
}

/// `rcpps` with one Newton step: `2r - x r r`. Tier C (exact start).
fn rcp_nr(x: f32) -> f32 {
    let r = 1.0 / x;
    (r + r) - r * x * r
}

fn dot3(a: V4, b: V4) -> f32 {
    (a[1] * b[1] + a[0] * b[0]) + a[2] * b[2]
}

fn sub(a: V4, b: V4) -> V4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

fn add(a: V4, b: V4) -> V4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn scale(a: V4, k: f32) -> V4 {
    [a[0] * k, a[1] * k, a[2] * k, a[3] * k]
}

fn mul(a: V4, b: V4) -> V4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// `a x b` on xyz; the fourth lane is `a.w*b.w - a.w*b.w` as the
/// shuffled SIMD code leaves it.
#[allow(clippy::eq_op)] // NaN/inf propagate as natively
fn cross(a: V4, b: V4) -> V4 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        a[3] * b[3] - a[3] * b[3],
    ]
}

/// Scales all four lanes by the reciprocal length of xyz.
pub fn normalize3(v: V4) -> V4 {
    let k = rsqrt_nr(v[2] * v[2] + v[1] * v[1] + v[0] * v[0]);
    scale(v, k)
}

/// Rotates `v` by `q` (q v q*), all four lanes.
/// Translated from 00c7f7a0 (decompiled, FalloutNV.exe 1.4.0.525).
#[allow(clippy::eq_op)] // the w lane's zero cross term, as natively
pub fn rotate(q: V4, v: V4) -> V4 {
    let w = q[3];
    let s = dot3(q, v);
    let c = cross(q, v);
    let k = w * w + -0.5;
    let r = [
        c[0] * w + (k * v[0] + s * q[0]),
        c[1] * w + (k * v[1] + s * q[1]),
        c[2] * w + (k * v[2] + s * q[2]),
        (w * v[3] - w * v[3]) * w + (k * v[3] + s * q[3]),
    ];
    add(r, r)
}

/// Rotates `v` by the inverse of `q` (q* v q), all four lanes.
/// Translated from 00c7f840 (decompiled, FalloutNV.exe 1.4.0.525).
#[allow(clippy::eq_op)] // the w lane's zero cross term, as natively
pub fn rotate_inverse(q: V4, v: V4) -> V4 {
    let w = q[3];
    let s = dot3(q, v);
    let c = cross(v, q);
    let k = w * w + -0.5;
    let r = [
        c[0] * w + (k * v[0] + s * q[0]),
        c[1] * w + (k * v[1] + s * q[1]),
        c[2] * w + (k * v[2] + s * q[2]),
        (w * v[3] - w * v[3]) * w + (k * v[3] + s * q[3]),
    ];
    add(r, r)
}

/// `a * b` (Hamilton product).
/// Translated from 00c66320 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn qmul(a: V4, b: V4) -> V4 {
    [
        b[3] * a[0] + a[3] * b[0] + (b[2] * a[1] - b[1] * a[2]),
        b[3] * a[1] + a[3] * b[1] + (b[0] * a[2] - b[2] * a[0]),
        b[3] * a[2] + a[3] * b[2] + (b[1] * a[0] - b[0] * a[1]),
        b[3] * a[3] - (b[2] * a[2] + b[1] * a[1] + b[0] * a[0]),
    ]
}

/// `a * conjugate(b)`.
/// Translated from 00c74ac0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn qmul_conj(a: V4, b: V4) -> V4 {
    [
        b[3] * a[0] + ((b[1] * a[2] - b[2] * a[1]) - a[3] * b[0]),
        b[3] * a[1] + ((b[2] * a[0] - b[0] * a[2]) - a[3] * b[1]),
        b[3] * a[2] + ((b[0] * a[1] - b[1] * a[0]) - a[3] * b[2]),
        b[3] * a[3] + b[1] * a[1] + b[2] * a[2] + b[0] * a[0],
    ]
}

/// In-place `a = a * b`.
/// Translated from 00ce0ca0 (decompiled, FalloutNV.exe 1.4.0.525).
fn qmul_in_place(a: V4, b: V4) -> V4 {
    [
        b[3] * a[0] + a[3] * b[0] + (a[1] * b[2] - a[2] * b[1]),
        b[3] * a[1] + a[3] * b[1] + (a[2] * b[0] - a[0] * b[2]),
        b[3] * a[2] + a[3] * b[2] + (a[0] * b[1] - a[1] * b[0]),
        a[3] * b[3] - (a[2] * b[2] + a[1] * b[1] + a[0] * b[0]),
    ]
}

fn conj(q: V4) -> V4 {
    [-q[0], -q[1], -q[2], q[3]]
}

/// Quaternion normalization over all four components.
/// Translated from 005611e0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn normalize_quat(q: V4) -> V4 {
    let n = (q[1] * q[1] + q[0] * q[0]) + q[2] * q[2] + q[3] * q[3];
    scale(q, rsqrt_nr(n))
}

/// A rotation of `angle` radians about the unit `axis` (all four lanes of
/// `axis` scaled by the sine, then w replaced by the cosine).
/// Translated from 00cb2450 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn axis_angle(axis: V4, angle: f32) -> V4 {
    let half = (angle as f64 * 0.5) as f32;
    let s = (half as f64).sin() as f32;
    let mut q = scale(axis, s);
    q[3] = (half as f64).cos() as f32;
    q
}

/// The rotation angle of a quaternion: `2 acos(|w|)`, 0 when |w| >= 1.
/// Translated from 00c74c20 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn quat_angle(q: V4) -> f32 {
    let x = q[3].abs();
    if x < 1.0 {
        let a = (x as f64).acos() as f32;
        return a + a;
    }
    // 00c74c49: 0 compared with |w|; unordered (NaN) gives pi.
    let a = if x.is_nan() { PI_F32 } else { 0.0 };
    a + a
}

/// `acos` clamped at the ends (|x| >= 1: 0 for x > 0, pi otherwise).
/// Translated from 00c74a70 (decompiled, FalloutNV.exe 1.4.0.525).
fn clamped_acos(x: f32) -> f32 {
    if 1.0 <= x.abs() {
        return if x <= 0.0 { PI_F32 } else { 0.0 };
    }
    (x as f64).acos() as f32
}

/// The unit axis of a rotation, flipped so that w is not negative.
/// Translated from 00c74c90 (decompiled, FalloutNV.exe 1.4.0.525).
fn quat_axis(q: V4) -> V4 {
    let k = rsqrt_nr(q[2] * q[2] + q[1] * q[1] + q[0] * q[0]);
    let a = [k * q[0], k * q[1], k * q[2], k * q[3]];
    if q[3] < 0.0 {
        [-a[0], -a[1], -a[2], -a[3]]
    } else {
        a
    }
}

/// `transform` applied to a point: `t + rotate(q, s * v)`.
/// Translated from 00c7f7a0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn transform_point(x: &Qs, v: V4) -> V4 {
    add(x.t, rotate(x.q, mul(x.s, v)))
}

/// The inverse of `transform` applied to a point:
/// `rotate_inverse(q, v - t) / s`.
/// Translated from 00c7f840 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn inverse_transform_point(x: &Qs, v: V4) -> V4 {
    let r = rotate_inverse(x.q, sub(v, x.t));
    [
        rcp_nr(x.s[0]) * r[0],
        rcp_nr(x.s[1]) * r[1],
        rcp_nr(x.s[2]) * r[2],
        rcp_nr(x.s[3]) * r[3],
    ]
}

/// `a` then `b`: the model transform of a child from its parent's model
/// transform and its own local one (hkQsTransform::setMul, Havok).
fn compose(a: &Qs, b: &Qs) -> Qs {
    Qs {
        t: transform_point(a, b.t),
        q: qmul(a.q, b.q),
        s: mul(a.s, b.s),
    }
}

/// The local transform of a bone from its parent's model transform and
/// its own; inlined in 00c78610/00c7aa60 (translation is not divided by
/// the parent's scale, and the scale's w lane is masked by 010c4c10).
fn relative(parent: &Qs, model: &Qs) -> Qs {
    let pc = conj(parent.q);
    let base = rotate_inverse(parent.q, parent.t);
    let base = [-base[0], -base[1], -base[2], -base[3]];
    let inv_s = [
        rcp_nr(parent.s[0]),
        rcp_nr(parent.s[1]),
        rcp_nr(parent.s[2]),
        0.0,
    ];
    Qs {
        t: add(rotate(pc, model.t), base),
        q: qmul(pc, model.q),
        s: mul(model.s, [inv_s[0], inv_s[1], inv_s[2], 0.0]),
    }
}

// ---------------------------------------------------------------------
// hkaPose (Havok): local and model transforms with dirty flags.

const LOCAL_DIRTY: u8 = 1;
const MODEL_DIRTY: u8 = 2;

/// A pose of a skeleton, kept in both spaces like Havok's `hkaPose`:
/// reading one space recomputes it from the other when it is stale.
#[derive(Debug, Clone)]
pub struct Pose {
    pub parents: Vec<i16>,
    local: Vec<Qs>,
    model: Vec<Qs>,
    flags: Vec<u8>,
}

impl Pose {
    /// A pose given in local space (parents before children).
    pub fn from_locals(parents: Vec<i16>, local: Vec<Qs>) -> Pose {
        let n = local.len();
        Pose {
            parents,
            model: vec![Qs::IDENTITY; n],
            flags: vec![MODEL_DIRTY; n],
            local,
        }
    }

    pub fn len(&self) -> usize {
        self.local.len()
    }

    pub fn is_empty(&self) -> bool {
        self.local.is_empty()
    }

    /// `hkaPose::getBoneModelSpace` (calculateBoneModelSpace, 00cda180).
    pub fn model(&mut self, i: usize) -> Qs {
        if self.flags[i] & MODEL_DIRTY != 0 {
            let local = self.local(i);
            let m = match self.parents[i] {
                -1 => local,
                p => {
                    let pm = self.model(p as usize);
                    compose(&pm, &local)
                }
            };
            self.model[i] = m;
            self.flags[i] &= !MODEL_DIRTY;
        }
        self.model[i]
    }

    /// `hkaPose::getBoneLocalSpace`.
    pub fn local(&mut self, i: usize) -> Qs {
        if self.flags[i] & LOCAL_DIRTY != 0 {
            let m = self.model[i];
            let l = match self.parents[i] {
                -1 => m,
                p => {
                    let pm = self.model(p as usize);
                    relative(&pm, &m)
                }
            };
            self.local[i] = l;
            self.flags[i] &= !LOCAL_DIRTY;
        }
        self.local[i]
    }

    /// Makes every descendant of `i` valid in local space and marks it
    /// stale in model space, as the inlined loops of 00c78610/00c7aa60
    /// do before bone `i` changes.
    fn detach_descendants(&mut self, i: usize) {
        let n = self.len();
        let mut below = vec![false; n];
        below[i] = true;
        for j in i + 1..n {
            let p = self.parents[j];
            if p >= 0 && below[p as usize] {
                self.local(j);
                below[j] = true;
            }
        }
        for (j, b) in below.iter().enumerate().skip(i + 1) {
            if *b {
                self.flags[j] |= MODEL_DIRTY;
            }
        }
    }

    /// `hkaPose::accessBoneModelSpace(i, PROPAGATE)` (00cdad80).
    pub fn set_model(&mut self, i: usize, m: Qs) {
        self.model(i);
        self.detach_descendants(i);
        self.model[i] = m;
        self.flags[i] = LOCAL_DIRTY;
    }

    /// `hkaPose::accessBoneLocalSpace(i)` (00cda8e0).
    pub fn set_local(&mut self, i: usize, l: Qs) {
        self.local(i);
        self.detach_descendants(i);
        self.local[i] = l;
        self.flags[i] = MODEL_DIRTY;
    }
}

// ---------------------------------------------------------------------
// The controller.

/// `bhkRagdollController::LookIKBone` (Xbox PDB).
pub const EYE_BONE: usize = 0;
pub const HEAD_BONE: usize = 1;

/// `bhkRagdollController::LookIKBoneParams` (Xbox PDB), 0x50 bytes each
/// from controller +0xf0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoneParams {
    /// +0x04 `ssBoneIdx`.
    pub bone: i16,
    /// +0x06 `ssParentIdx`.
    pub parent: i16,
    /// +0x08 `fLookAtMaxAngle`, radians.
    pub max_angle: f32,
    /// +0x10 `FwdLS`.
    pub fwd_ls: V4,
    /// +0x20 `FwdParentMS`.
    pub fwd_parent_ms: V4,
    /// +0x30 `PrevRotLS`.
    pub prev_rot_ls: V4,
    /// +0x40 `bLimitBoneRot`.
    pub limit_bone_rot: bool,
    /// +0x41 `bActive`.
    pub active: bool,
    /// +0x44 `fDeadHeading`.
    pub dead_heading: f32,
    /// +0x48 `fDeadPitch`.
    pub dead_pitch: f32,
}

impl Default for BoneParams {
    /// 00c7f060's per-bone initialization.
    fn default() -> Self {
        BoneParams {
            bone: -1,
            parent: -1,
            max_angle: DEFAULT_MAX_ANGLE,
            fwd_ls: [0.0; 4],
            fwd_parent_ms: [0.0; 4],
            prev_rot_ls: QUAT_IDENTITY,
            limit_bone_rot: true,
            active: false,
            dead_heading: 0.0,
            dead_pitch: 0.0,
        }
    }
}

/// The LookIK part of `bhkRagdollController` (Xbox PDB).
#[derive(Debug, Clone)]
pub struct LookIk {
    /// +0xb0 `bInitLookIK`.
    pub init: bool,
    /// +0xb1 `bEnableLookIK`.
    pub enable: bool,
    /// +0xb2 `bEaseOutLookIK`.
    pub ease_out: bool,
    /// +0xb3 `bActiveLookIK`.
    pub active: bool,
    /// +0x43 (Xbox PDB `bEaseOutRagdollAnim`).
    pub ease_out_ragdoll_anim: bool,
    /// +0xc0 `CurrentHeadRotOffset`.
    pub head_rot_offset: V4,
    /// +0xd0 `CurrentTargetWS`, Havok units.
    pub target_ws: V4,
    /// +0xf0 `BoneParamA`.
    pub bones: [BoneParams; 2],
    /// +0x190 `bUseHeadPose`.
    pub use_head_pose: bool,
    /// +0x194 `eCurrentBone`.
    pub current_bone: usize,
    /// +0x19c `fEyeHeading`.
    pub eye_heading: f32,
    /// +0x1a0 `fEyePitch`.
    pub eye_pitch: f32,
    /// +0x1a4 `bTargetBehind`.
    pub target_behind: bool,
    /// +0x1a8: `fTrackEyeZ` in radians (00607830); no reader found.
    pub eye_pitch_range: f32,
    /// +0x54 `pHeadPose`: the head and the eye point.
    pub head_pose: Option<Pose>,
}

impl Default for LookIk {
    fn default() -> Self {
        LookIk {
            init: false,
            enable: false,
            ease_out: false,
            active: false,
            ease_out_ragdoll_anim: false,
            head_rot_offset: QUAT_IDENTITY,
            target_ws: [0.0; 4],
            bones: [BoneParams::default(); 2],
            use_head_pose: false,
            current_bone: 0,
            eye_heading: 0.0,
            eye_pitch: 0.0,
            target_behind: false,
            eye_pitch_range: 0.0,
            head_pose: None,
        }
    }
}

/// The seed direction of `InitBoneParams` and the eye setup: model +Y.
const FORWARD: V4 = [0.0, 1.0, 0.0, 0.0];

impl LookIk {
    /// Sets up head tracking for an actor, as 0087e130 does for the body
    /// part flagged for head tracking (BPND flag 0x20).
    ///
    /// `head` is the skeleton index of the part's IK start node (BPNI,
    /// `Bip01 Head` for the standard human), `pose` the scene-graph pose,
    /// `is_creature` the actor's `IsCreature` (vtable +0x21c), and
    /// `tracking_max_angle` the part's BPND tracking max angle (degrees).
    pub fn init(
        &mut self,
        pose: &mut Pose,
        head: Option<usize>,
        is_creature: bool,
        tracking_max_angle: f32,
    ) -> bool {
        // 0087e543: actors that are not creatures use the head pose.
        if !is_creature {
            self.use_head_pose = true;
        }
        self.init_look_ik(pose, head);
        // 0087e5a9 (00607810): the head's maximum angle from the body part.
        self.bones[HEAD_BONE].max_angle = (tracking_max_angle as f64 * DEG_TO_RAD) as f32;
        // 0087e5bc (005ba4f0).
        self.enable = self.init;
        self.enable
    }

    /// `InitLookIK` (Xbox PDB).
    /// Translated from 00c7de60 (decompiled, FalloutNV.exe 1.4.0.525).
    fn init_look_ik(&mut self, pose: &mut Pose, head: Option<usize>) {
        let ok = self.init_bone_params(pose, head, HEAD_BONE);
        if ok {
            self.current_bone = HEAD_BONE;
            if self.use_head_pose {
                let s = havok_scale();
                let eye = Qs {
                    t: [EYE_OFFSET[0] * s, EYE_OFFSET[1] * s, 0.0, 0.0],
                    ..Qs::IDENTITY
                };
                // The head bone's local transform: 00a86bf0 constructs an
                // identity NiTransform (identity rotation, zero point,
                // scale 1), converted by 00c74dd0 (00c7e103..00c7e118).
                self.head_pose = Some(Pose::from_locals(vec![-1, 0], vec![Qs::IDENTITY, eye]));
                let e = &mut self.bones[EYE_BONE];
                e.bone = 1;
                e.fwd_ls = FORWARD;
                e.fwd_parent_ms = FORWARD;
                e.parent = 0;
                self.current_bone = EYE_BONE;
            }
        }
        self.init = ok;
    }

    /// `InitBoneParams` (Xbox PDB): the bone's and its parent's
    /// expression of model +Y, from the pose at set-up time.
    /// Translated from 00c79340 (decompiled, FalloutNV.exe 1.4.0.525).
    fn init_bone_params(&mut self, pose: &mut Pose, bone: Option<usize>, which: usize) -> bool {
        self.ease_out = false;
        let Some(bone) = bone else {
            self.bones[which].bone = -1;
            return false;
        };
        let parent = pose.parents[bone];
        let b = &mut self.bones[which];
        b.bone = bone as i16;
        b.parent = parent;
        if parent == -1 {
            return false;
        }
        let bm = pose.model(bone);
        let pm = pose.model(parent as usize);
        let b = &mut self.bones[which];
        b.fwd_parent_ms = rotate_inverse(pm.q, FORWARD);
        b.fwd_ls = normalize3(rotate_inverse(bm.q, FORWARD));
        true
    }

    /// `SetLookIKActive` (Xbox PDB).
    /// Translated from 00c75580 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn set_active(&mut self, on: bool) {
        if !self.ease_out_ragdoll_anim {
            self.active = self.enable && on;
        }
        if !on {
            for b in &mut self.bones {
                b.prev_rot_ls = QUAT_IDENTITY;
                b.limit_bone_rot = true;
            }
        }
    }

    /// `SetLookAtIKTarget` (Xbox PDB); `target` in game units.
    /// Translated from 008a3b70 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn set_target(&mut self, target: [f32; 3], activate: bool) {
        let s = havok_scale();
        self.target_ws = [target[0] * s, target[1] * s, target[2] * s, 0.0];
        if activate {
            self.set_active(true);
            self.ease_out = false;
            self.ease_out_ragdoll_anim = false;
        }
    }

    /// The head-tracking part of the actor update 008a3100: with a target
    /// whose position lies within the tracking distances of the actor's
    /// own, the controller is (re)activated toward the target's look
    /// anchor; a target out of range changes nothing (008a3a72/008a3a83
    /// jump to the end, so the look stays on its stored point); without a
    /// target an active controller starts easing out (008a3bf0(1)).
    ///
    /// The distance is between the two actors' positions (both from
    /// virtual +0x1f4, 00457990), not from the head (settled 2026-10-06;
    /// first found on the contributor branch `playcon/claude/lookik-
    /// head-tracking`, da055f5). Whom to look at comes from
    /// [`crate::head_track`] (008a3ed0 and the target slots).
    pub fn update_target(
        &mut self,
        settings: &Settings,
        own_position: [f32; 3],
        target: Option<Target>,
    ) {
        if let Some(t) = target {
            let d = [
                t.position[0] - own_position[0],
                t.position[1] - own_position[1],
                t.position[2] - own_position[2],
            ];
            let dist = (d[2] * d[2] + d[1] * d[1] + d[0] * d[0]).sqrt();
            // 008a3a72/008a3a83: distance <= fMaxTrackingDist and >
            // fMinTrackingDist.
            if dist <= settings.max_tracking_dist && settings.min_tracking_dist < dist {
                self.ease_out = false;
                self.set_active(true);
                self.set_target(t.anchor, true);
            }
            return;
        }
        if self.active && !self.ease_out {
            self.ease_out = true;
        }
    }

    /// The eye limits a FaceGen head sets when it is attached (00607420,
    /// for an actor with a look controller): the eye bone's cone is
    /// `fTrackEyeXY` (00607810(0, ..)), `fTrackEyeZ` is stored at +0x1a8
    /// (00607830; no reader found in the controller), and the eye's dead
    /// zones are `fTrackDeadZoneXY`/`Z` (00c748d0(0, ..)). Beyond a dead
    /// zone the head turns too (00c7b04f..00c7b082).
    pub fn head_attached(&mut self, track: &TrackSettings) {
        let (xy, z) = (track.eye_xy(), track.eye_z());
        self.eye_pitch_range = z;
        self.bones[EYE_BONE].max_angle = xy;
        self.bones[EYE_BONE].dead_heading = track.dead_zone_xy();
        self.bones[EYE_BONE].dead_pitch = track.dead_zone_z();
    }

    /// What the actor update 00888970 hands the face after the look
    /// update (00888a20 into `fDesiredEyeHeading`/`Pitch`, +0x150/+0x154,
    /// Xbox PDB): the eye heading and pitch while the look is on
    /// (+0xb3, 00888a50), else zero.
    pub fn desired_eyes(&self) -> (f32, f32) {
        if self.active {
            (self.eye_heading, self.eye_pitch)
        } else {
            (0.0, 0.0)
        }
    }

    /// The eye point in the head bone's frame, game units, when the
    /// controller has a head pose (00c757b0 reads the head pose's eye
    /// bone): 00c7de60's 9, 6, 0.
    pub fn eye_offset(&self) -> Option<[f32; 3]> {
        (self.use_head_pose && self.init).then_some([EYE_OFFSET[0], EYE_OFFSET[1], 0.0])
    }

    /// `AdjustCurrentTarget` (Xbox PDB): keeps the stored target within
    /// reach of the head and handles targets behind the actor.
    /// Translated from 00c78160 (decompiled, FalloutNV.exe 1.4.0.525).
    fn adjust_current_target(&mut self, sg: &mut Pose, world_from_model: &Qs) {
        let local = inverse_transform_point(world_from_model, self.target_ws);
        let n = normalize3(local);
        let ahead = 0.0 * n[2] + 1.0 * n[1] + 0.0 * n[0];
        self.target_behind = ahead < 0.0;
        let head = self.bones[HEAD_BONE].bone as usize;
        let eye_point = |s: &mut LookIk| -> V4 {
            match (&mut s.head_pose, s.use_head_pose) {
                (Some(p), true) => {
                    let l = p.local(1).t;
                    [l[0], l[1], l[2], l[3]]
                }
                _ => [0.0; 4],
            }
        };
        if !self.ease_out {
            if ahead < 0.0 {
                let raw = inverse_transform_point(world_from_model, self.target_ws);
                let side = if raw[0] < 0.0 {
                    -BEHIND_SIDE
                } else {
                    BEHIND_SIDE
                };
                let e = eye_point(self);
                let hm = sg.model(head);
                let mut p = transform_point(&hm, e);
                p[0] += side;
                self.target_ws = transform_point(world_from_model, p);
            }
        } else {
            let fwd = rotate(sg.model(head).q, self.bones[HEAD_BONE].fwd_ls);
            let fwd = rotate(world_from_model.q, fwd);
            let fwd = scale(fwd, EASE_AHEAD);
            let e = eye_point(self);
            let hm = sg.model(head);
            let p = transform_point(&hm, e);
            let p = transform_point(world_from_model, p);
            self.target_ws = add(p, fwd);
        }
        // 00c784db: clamp the target's distance from the eye point.
        let e = eye_point(self);
        let hm = sg.model(head);
        let p = transform_point(world_from_model, transform_point(&hm, e));
        let d = sub(self.target_ws, p);
        let len2 = d[2] * d[2] + d[1] * d[1] + d[0] * d[0];
        let len = len2.sqrt();
        let k = rsqrt_nr(len2);
        if TARGET_REACH < len {
            self.target_ws = [
                k * d[0] * TARGET_REACH + p[0],
                k * d[1] * TARGET_REACH + p[1],
                k * d[2] * TARGET_REACH + p[2],
                k * d[3] * TARGET_REACH + p[3],
            ];
        }
    }

    /// `LimitBoneRot` (Xbox PDB): caps the change of the bone's local
    /// rotation since the previous update; returns the angle applied.
    /// Translated from 00c755e0 (decompiled, FalloutNV.exe 1.4.0.525).
    fn limit_bone_rot(
        &mut self,
        settings: &Settings,
        pose: &mut Pose,
        which: usize,
        wanted: V4,
    ) -> f32 {
        let prev = self.bones[which].prev_rot_ls;
        let same = (0..3).all(|k| (wanted[k] - prev[k]).abs() <= SAME);
        if same {
            return 0.0;
        }
        let mut rel = qmul_conj(wanted, prev);
        let angle = quat_angle(rel);
        let cap_deg = if self.ease_out {
            settings.angle_max_ease
        } else {
            settings.angle_max
        };
        let cap = cap_deg as f64 * DEG_TO_RAD;
        let mut out = angle;
        // Not `angle <= cap`: a NaN angle takes the capped branch.
        if !matches!(
            (angle as f64).partial_cmp(&cap),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ) {
            let cap = cap as f32;
            let axis = normalize3(quat_axis(rel));
            rel = axis_angle(axis, cap);
            out = cap;
        }
        let q = normalize_quat(qmul(rel, prev));
        let bone = self.bones[which].bone as usize;
        let mut l = pose.local(bone);
        l.q = q;
        pose.set_local(bone, l);
        self.bones[which].prev_rot_ls = q;
        out
    }

    /// `BoneTrack` (Xbox PDB): turns one bone toward the target; returns
    /// true when the target lies outside the bone's cone.
    /// Translated from 00c78610 (decompiled, FalloutNV.exe 1.4.0.525).
    fn bone_track(
        &mut self,
        settings: &Settings,
        pose: &mut Pose,
        which: usize,
        world_from_model: &Qs,
        eye: Option<V4>,
    ) -> bool {
        let p = self.bones[which];
        let parent_ms = pose.model(p.parent as usize);
        let cone_axis = normalize3(rotate(parent_ms.q, p.fwd_parent_ms));
        let mut target = inverse_transform_point(world_from_model, self.target_ws);
        let mut offset = [0.0; 4];
        if let (Some(e), true) = (eye, which == HEAD_BONE) {
            // 00c787e0: keep the target beyond the eye's sideways offset.
            let f = p.fwd_ls;
            let n = normalize3(cross(f, e));
            let m = normalize3(cross(n, f));
            let m2 = mul(m, m);
            let lateral = dot3(m, e) / (m2[2] + m2[1] + m2[0]).sqrt();
            let hm = pose.model(p.bone as usize);
            let d = sub(target, hm.t);
            let len2 = d[2] * d[2] + d[1] * d[1] + d[0] * d[0];
            let k = rsqrt_nr(len2);
            if len2.sqrt() < lateral {
                target = [
                    k * d[0] * lateral + target[0],
                    k * d[1] * lateral + target[1],
                    k * d[2] * lateral + target[2],
                    k * d[3] * lateral + target[3],
                ];
            }
            offset = e;
        }
        let bone = p.bone as usize;
        let original = pose.local(bone);
        if (0..3).all(|k| (p.prev_rot_ls[k] - 0.0).abs() <= SAME) {
            self.bones[which].prev_rot_ls = original.q;
        }
        let mut ms = pose.model(bone);
        let block = SolveInput {
            fwd: p.fwd_ls,
            offset,
            cone_axis,
            max_angle: p.max_angle,
        };
        let inside = solve_direction(&block, target, 1.0, &mut ms);
        pose.set_model(bone, ms);
        let local = pose.local(bone);
        let angle = if self.bones[which].limit_bone_rot {
            self.limit_bone_rot(settings, pose, bone_slot(which), local.q)
        } else {
            0.0
        };
        let check = pose.model(bone).q;
        if !quat_ok(check) {
            pose.set_local(bone, original);
        }
        self.bones[which].limit_bone_rot = true;
        if self.ease_out && (angle as f64) < settings.ease_angle_shut_off as f64 * DEG_TO_RAD {
            self.bones[which].prev_rot_ls = QUAT_IDENTITY;
            if which == EYE_BONE || !self.use_head_pose {
                self.ease_out = false;
                self.active = false;
            } else {
                self.bones[self.current_bone].active = false;
                self.current_bone = self.current_bone.wrapping_sub(1);
            }
        }
        !inside
    }

    /// `DoLookAtIK` (Xbox PDB), run each update while look IK is active.
    /// `world_from_model` is the scene-graph pose's model-to-world
    /// transform in Havok units.
    /// Translated from 00c7aa60 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn do_look_at_ik(&mut self, settings: &Settings, sg: &mut Pose, world_from_model: &Qs) {
        if !self.init {
            return;
        }
        self.adjust_current_target(sg, world_from_model);
        let eye = match (&mut self.head_pose, self.use_head_pose) {
            (Some(p), true) => Some(p.local(1).t),
            _ => None,
        };
        if self.bones[HEAD_BONE].active || !self.use_head_pose {
            self.current_bone = HEAD_BONE;
            self.bone_track(settings, sg, HEAD_BONE, world_from_model, eye);
        }
        let head = self.bones[HEAD_BONE].bone as usize;
        let mut hm = sg.model(head);
        hm.q = qmul(hm.q, self.head_rot_offset);
        sg.set_model(head, hm);
        if !self.use_head_pose {
            return;
        }
        let hm = sg.model(head);
        // 00c7ac87: the head's world transform (translation rotated but
        // not scaled by the parent's scale, as the native code does).
        let world_head = Qs {
            t: add(rotate(world_from_model.q, hm.t), world_from_model.t),
            q: qmul(world_from_model.q, hm.q),
            s: mul(world_from_model.s, hm.s),
        };
        let Some(mut hp) = self.head_pose.take() else {
            return;
        };
        let outside = self.bone_track(settings, &mut hp, EYE_BONE, &world_head, None);
        self.bones[HEAD_BONE].active = outside || self.current_bone != 0;
        let eq = hp.model(1).q;
        self.head_pose = Some(hp);
        let v = rotate(eq, FORWARD);
        let heading = -v[2] + v[1] * 0.0 + v[0] * 0.0;
        let pitch = v[2] * 0.0 + v[1] * 0.0 + v[0] * 1.0;
        self.eye_heading = (HALF_PI - (heading as f64).acos()) as f32;
        self.eye_pitch = (HALF_PI - (pitch as f64).acos()) as f32;
        let e = &self.bones[EYE_BONE];
        if e.dead_pitch < self.eye_pitch.abs() || e.dead_heading < self.eye_heading.abs() {
            self.bones[HEAD_BONE].active = true;
        }
    }
}

/// `LimitBoneRot` indexes the bone records by `LookIKBone`.
fn bone_slot(which: usize) -> usize {
    which
}

/// `hkQuaternion::isOk`: finite and of unit length within 0.001.
/// Translated from 00cb24c0 (decompiled, FalloutNV.exe 1.4.0.525).
fn quat_ok(q: V4) -> bool {
    q.iter().all(|c| c.is_finite())
        && ((q[3] * q[3] + q[1] * q[1] + q[2] * q[2] + q[0] * q[0]) - 1.0).abs() < SAME
}

/// The 13-float block 00c78610 hands to the direction solve.
#[derive(Debug, Clone, Copy)]
struct SolveInput {
    /// Lanes 0..3: the bone's own forward (`FwdLS`).
    fwd: V4,
    /// Lanes 4..7: the eye point in the bone's frame (zero for the eye).
    offset: V4,
    /// Lanes 8..11: the cone axis (`FwdParentMS` turned by the parent).
    cone_axis: V4,
    /// Lane 12: the cone half-angle.
    max_angle: f32,
}

/// Turns the bone's model transform `ms` so that its forward points at
/// `target` (model space), limited to a cone, with the eye-point
/// correction when `offset` is not zero. Returns true when the target was
/// inside the cone.
/// Translated from 00ce0290 (decompiled, FalloutNV.exe 1.4.0.525);
/// the constraint-box argument is always null at the LookIK call site.
fn solve_direction(input: &SolveInput, target: V4, gain: f32, ms: &mut Qs) -> bool {
    let p = ms.t;
    let q = ms.q;
    let d = sub(target, p);
    let len2 = d[2] * d[2] + d[1] * d[1] + d[0] * d[0];
    let dist = len2.sqrt();
    let mut dir = scale(d, rsqrt_nr(len2));
    let b = input.cone_axis;
    let mut inside = true;
    let cos_max = (input.max_angle as f64).cos() as f32;
    if (dot3(b, dir) as f64) < cos_max as f64 {
        let axis = normalize3(cross(b, dir));
        let limit = axis_angle(axis, input.max_angle);
        dir = rotate(limit, b);
        inside = false;
    }
    let r = rotate(q, input.fwd);
    let c = dot3(r, dir);
    let angle = if c.abs() < 1.0 {
        (c as f64).acos() as f32
    } else if c > 0.0 {
        0.0
    } else {
        PI_F32
    };
    let axis = normalize3(cross(r, dir));
    let mut delta = axis_angle(axis, angle * gain);
    let e = input.offset;
    let e_len = (e[1] * e[1] + e[0] * e[0] + e[2] * e[2]).sqrt();
    if 0.0 < e_len {
        let a = input.fwd;
        let n = normalize3(cross(a, e));
        let side = normalize3(cross(n, a));
        let s2 = mul(side, side);
        let along = dot3(side, e) / (s2[1] + s2[0] + s2[2]).sqrt();
        let twist = eye_twist(dist, along);
        let er = rotate(q, e);
        let axis = normalize3(cross(r, er));
        let t = axis_angle(axis, -twist);
        delta = qmul_in_place(delta, t);
    }
    ms.q = qmul(delta, q);
    inside
}

/// The eye-point angle of 00ce09e2..00ce0ad6: with the target `dist`
/// away and the eye `side` off the forward axis, the angle by which the
/// head turns back so the eye, not the head centre, faces the target.
/// The x87 sequence is transliterated with f32 stores where the native
/// code stores to memory.
fn eye_twist(dist: f32, side: f32) -> f32 {
    let l2 = (dist as f64 * dist as f64) as f32;
    let d2 = (side as f64 * side as f64) as f32;
    let s = ((l2 as f64 - d2 as f64).abs() as f32 as f64).sqrt() as f32;
    let s2 = (s as f64 * s as f64) as f32;
    // 00ce0a3d..00ce0a7d, instruction by instruction.
    let mut f = Fpu(Vec::new());
    f.ld(l2 as f64); // FLD [esp+0x18]
    f.ld_st(0); // FLD ST0
    f.ld(2.0); // FLD 2.0
    f.mul_into(1); // FMUL ST1 (DC C9: ST1 = ST1 * ST0)
    f.ld(s2 as f64); // FLD [esp+0x20]
    f.ld_st(0);
    f.ld(d2 as f64); // FLD [esp+0x3c]
    f.ld_st(0);
    f.ld_st(3);
    f.mulp(5);
    f.ld_st(1);
    f.mulp(5);
    f.xch(3);
    f.mul_st(5);
    f.subp(4);
    f.ld_st(5);
    f.mulp(6);
    f.xch(3);
    f.addp(5);
    f.xch(2);
    f.mulp(3);
    f.xch(3);
    f.addp(2);
    f.mul_st(0);
    f.addp(1);
    f.ld_st(1);
    f.mulp(2);
    f.addp(1);
    let area = (f.0[0] as f32).abs();
    let r = (((area as f64).sqrt() as f32) as f64 * 0.5 / s as f64) as f32;
    if r == 0.0 {
        return 0.0;
    }
    clamped_acos((side as f64 / r as f64) as f32)
}

/// A minimal x87 register stack (top first) for transliterated sequences;
/// extended precision is approximated by f64.
struct Fpu(Vec<f64>);

impl Fpu {
    fn ld(&mut self, v: f64) {
        self.0.insert(0, v);
    }
    fn ld_st(&mut self, i: usize) {
        let v = self.0[i];
        self.ld(v);
    }
    fn mul_st(&mut self, i: usize) {
        self.0[0] *= self.0[i];
    }
    fn mul_into(&mut self, i: usize) {
        self.0[i] *= self.0[0];
    }
    fn xch(&mut self, i: usize) {
        self.0.swap(0, i);
    }
    fn mulp(&mut self, i: usize) {
        self.0[i] *= self.0[0];
        self.0.remove(0);
    }
    fn addp(&mut self, i: usize) {
        self.0[i] += self.0[0];
        self.0.remove(0);
    }
    fn subp(&mut self, i: usize) {
        self.0[i] -= self.0[0];
        self.0.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() <= tol * (1.0 + b.abs())
    }

    #[test]
    fn rotations_match_their_inverse() {
        let q = normalize_quat([0.1, 0.7, -0.2, 0.6]);
        let v = [3.0, -1.0, 2.0, 0.0];
        let back = rotate_inverse(q, rotate(q, v));
        for k in 0..3 {
            assert!(close(back[k], v[k], 1e-5), "{back:?}");
        }
    }

    #[test]
    fn rotate_turns_y_like_the_native_basis_formula() {
        // SETUP_ASSEMBLY: the inverse rotation of +Y has
        // x = 2(xy + zw), y = 2(yy + ww - 0.5), z = 2(yz - xw).
        let q = normalize_quat([0.3, -0.1, 0.4, 0.8]);
        let r = rotate_inverse(q, FORWARD);
        let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
        assert!(close(r[0], 2.0 * (x * y + z * w), 1e-6));
        assert!(close(r[1], 2.0 * (y * y + w * w - 0.5), 1e-6));
        assert!(close(r[2], 2.0 * (y * z - x * w), 1e-6));
    }

    /// nv-call, 005611c0 on FalloutNV.exe 1.4.0.525 (AMD Ryzen AI 9 HX
    /// 370, 2026-10-05): input and output bit patterns. Tier C.
    #[test]
    fn normalize_matches_the_native_vectors() {
        let cases: [([f32; 4], [u32; 4]); 4] = [
            ([3.0, 4.0, 0.0, 0.0], [0x3f19_999a, 0x3f4c_cccd, 0, 0]),
            (
                [1.0, 1.0, 1.0, 0.0],
                [0x3f13_cd3a, 0x3f13_cd3a, 0x3f13_cd3a, 0],
            ),
            (
                [1.0, 2.0, 3.0, 4.0],
                [0x3e3a_f4b9, 0x3eba_f4b9, 0x3f0c_378b, 0x3f3a_f4b9],
            ),
            (
                [-0.3, 0.7, -2.5, 0.0],
                [0xbdeb_17f9, 0x3e89_2351, 0xbf74_e3a3, 0],
            ),
        ];
        for (input, want) in cases {
            let got = normalize_quat(input);
            for k in 0..4 {
                let w = f32::from_bits(want[k]);
                assert!(close(got[k], w, 2e-7), "{input:?}: {got:?} vs {w}");
            }
        }
        assert_eq!(normalize_quat([0.0; 4]), [0.0; 4]);
    }

    fn ulps(a: f32, b: f32) -> u32 {
        (a.to_bits() as i64 - b.to_bits() as i64).unsigned_abs() as u32
    }

    /// nv-call on FalloutNV.exe 1.4.0.525 (AMD Ryzen AI 9 HX 370,
    /// 2026-10-06): 00c66320, 00c74ac0 (tier B, exact), 00c74c20 and
    /// 00cb2450 (tier D, x87 transcendentals, within 2 ULP).
    #[test]
    fn quaternion_helpers_match_the_native_vectors() {
        let a = [0.1, 0.7, -0.2, 0.6];
        let b = [-0.3, 0.2, 0.5, 0.75];
        let want = [0x3e91_eb84, 0x3f27_ae14, 0x3ec2_8f5d, 0x3ee1_47af];
        let got = qmul(a, b);
        for k in 0..4 {
            assert!(ulps(got[k], f32::from_bits(want[k])) <= 1, "qmul {got:?}");
        }
        let want = [0xbe0a_3d6f, 0x3eca_3d70, 0xbf2e_147c, 0x3eeb_8520];
        let got = qmul_conj(a, b);
        for k in 0..4 {
            assert!(
                ulps(got[k], f32::from_bits(want[k])) <= 1,
                "qmul_conj {got:?}"
            );
        }
        assert!(ulps(quat_angle(b), f32::from_bits(0x3fb9_051d)) <= 2);
        let axis = [0.267_261, 0.534_522, 0.801_784, 0.0];
        let want = [0x3dbb_af6f, 0x3e3b_af6f, 0x3e8c_c39f, 0x3f70_7abb];
        let got = axis_angle(axis, 0.7);
        for k in 0..4 {
            assert!(
                ulps(got[k], f32::from_bits(want[k])) <= 2,
                "axis_angle {got:?}"
            );
        }
    }

    #[test]
    fn limit_caps_the_turn_per_update() {
        let s = Settings::default();
        let mut ik = LookIk::default();
        ik.bones[HEAD_BONE].bone = 0;
        let mut pose = Pose::from_locals(vec![-1], vec![Qs::IDENTITY]);
        let wanted = axis_angle([0.0, 0.0, 1.0, 0.0], 0.5);
        let applied = ik.limit_bone_rot(&s, &mut pose, HEAD_BONE, wanted);
        let cap = (3.5f64 * DEG_TO_RAD) as f32;
        assert!(close(applied, cap, 1e-6));
        assert!(close(quat_angle(pose.local(0).q), cap, 1e-4));
        // A small change is taken whole.
        let prev = ik.bones[HEAD_BONE].prev_rot_ls;
        let small = normalize_quat(qmul(axis_angle([0.0, 0.0, 1.0, 0.0], 0.01), prev));
        let applied = ik.limit_bone_rot(&s, &mut pose, HEAD_BONE, small);
        assert!(close(applied, 0.01, 1e-3));
    }

    /// nv-call, 00ce0290 on FalloutNV.exe 1.4.0.525 (AMD Ryzen AI 9 HX
    /// 370, 2026-10-06): forward +Y, cone +Y at 60 degrees, bone at the
    /// origin; resulting rotation bit patterns and the in-cone byte.
    /// Tiers C/D (rsqrt estimate, x87 acos/sin/cos).
    #[test]
    #[allow(clippy::type_complexity)]
    fn solve_matches_the_native_vectors() {
        let e = [1.2859, 0.857, 0.0, 0.0];
        let z = [0.0; 4];
        let id = QUAT_IDENTITY;
        let q2 = [0.1, 0.2, -0.05, 0.9721];
        let cases: [(&str, V4, V4, V4, bool, [u32; 4]); 6] = [
            (
                "base",
                z,
                [1.0, 2.0, 0.5, 0.0],
                id,
                true,
                [0x3de6_ea1a, 0, 0xbe66_ea1a, 0x3f77_baef],
            ),
            (
                "eye1",
                e,
                [5.0, 5.0, 0.0, 0.0],
                id,
                true,
                [0, 0, 0xbe97_ec9a, 0x3f74_786a],
            ),
            (
                "eye2",
                e,
                [1.0, 3.0, 0.2, 0.0],
                id,
                true,
                [0x3d00_2c70, 0xbbd9_6b59, 0x3d45_5d68, 0x3f7f_924c],
            ),
            (
                "eye3",
                e,
                [-2.0, 4.0, 1.0, 0.0],
                q2,
                true,
                [0x3dc1_3884, 0x3e1e_8f72, 0x3ec1_26cb, 0x3f68_23f3],
            ),
            (
                "cone",
                z,
                [5.0, 0.0, 0.0, 0.0],
                id,
                false,
                [0, 0, 0xbeff_ffff, 0x3f5d_b3d7],
            ),
            (
                "eyefar",
                e,
                [40.0, 40.0, 0.0, 0.0],
                id,
                true,
                [0, 0, 0xbebe_8b70, 0x3f6d_9c86],
            ),
        ];
        for (name, offset, target, q, want_inside, want) in cases {
            let input = SolveInput {
                fwd: FORWARD,
                offset,
                cone_axis: FORWARD,
                max_angle: DEFAULT_MAX_ANGLE,
            };
            let mut ms = Qs { q, ..Qs::IDENTITY };
            let inside = solve_direction(&input, target, 1.0, &mut ms);
            assert_eq!(inside, want_inside, "{name}");
            for k in 0..4 {
                let w = f32::from_bits(want[k]);
                assert!(
                    (ms.q[k] - w).abs() <= 2e-6,
                    "{name}: {:?} vs {:?}",
                    ms.q,
                    want.map(f32::from_bits)
                );
            }
        }
    }

    #[test]
    fn solve_points_the_forward_at_a_target_inside_the_cone() {
        let input = SolveInput {
            fwd: FORWARD,
            offset: [0.0; 4],
            cone_axis: FORWARD,
            max_angle: DEFAULT_MAX_ANGLE,
        };
        let mut ms = Qs::IDENTITY;
        let target = [1.0, 2.0, 0.5, 0.0];
        assert!(solve_direction(&input, target, 1.0, &mut ms));
        let f = rotate(ms.q, FORWARD);
        let t = normalize3(target);
        for k in 0..3 {
            assert!(close(f[k], t[k], 1e-4), "{f:?} vs {t:?}");
        }
    }

    #[test]
    fn solve_stops_at_the_cone_edge() {
        let input = SolveInput {
            fwd: FORWARD,
            offset: [0.0; 4],
            cone_axis: FORWARD,
            max_angle: DEFAULT_MAX_ANGLE,
        };
        let mut ms = Qs::IDENTITY;
        // Straight to the side: 90 degrees, beyond the 60-degree cone.
        assert!(!solve_direction(&input, [5.0, 0.0, 0.0, 0.0], 1.0, &mut ms));
        let f = rotate(ms.q, FORWARD);
        assert!(close(dot3(f, FORWARD), DEFAULT_MAX_ANGLE.cos(), 1e-4));
    }

    #[test]
    fn eye_twist_is_zero_without_an_offset_and_small_for_far_targets() {
        assert_eq!(
            eye_twist(10.0, 0.0),
            clamped_acos(0.0).min(eye_twist(10.0, 0.0))
        );
        let near = eye_twist(2.0, 0.5);
        let far = eye_twist(200.0, 0.5);
        assert!(near.is_finite() && far.is_finite());
    }

    #[test]
    fn set_active_off_resets_the_previous_rotations() {
        let mut ik = LookIk {
            enable: true,
            ..LookIk::default()
        };
        ik.bones[0].prev_rot_ls = [0.1, 0.0, 0.0, 0.99];
        ik.bones[1].limit_bone_rot = false;
        ik.set_active(true);
        assert!(ik.active);
        ik.set_active(false);
        assert!(!ik.active);
        assert_eq!(ik.bones[0].prev_rot_ls, QUAT_IDENTITY);
        assert!(ik.bones[1].limit_bone_rot);
    }

    fn at(anchor: [f32; 3], position: [f32; 3]) -> Option<Target> {
        Some(Target { anchor, position })
    }

    #[test]
    fn targets_outside_the_tracking_distances_change_nothing() {
        let s = Settings::default();
        let mut ik = LookIk {
            enable: true,
            ..LookIk::default()
        };
        // The anchor is looked at; the distance is between positions.
        ik.update_target(&s, [0.0; 3], at([100.0, 0.0, 120.0], [100.0, 0.0, 0.0]));
        assert!(ik.active && !ik.ease_out);
        let h = havok_scale();
        assert!(close(ik.target_ws[0], 100.0 * h, 1e-6));
        assert!(close(ik.target_ws[2], 120.0 * h, 1e-6));
        // Too close (12 is excluded) or too far (1500 included): nothing
        // changes, the stored point stays.
        ik.update_target(&s, [0.0; 3], at([1.0, 0.0, 120.0], [12.0, 0.0, 0.0]));
        assert!(ik.active && !ik.ease_out);
        assert!(close(ik.target_ws[0], 100.0 * h, 1e-6));
        ik.update_target(&s, [0.0; 3], at([1500.0, 0.0, 0.0], [1500.5, 0.0, 0.0]));
        assert!(close(ik.target_ws[0], 100.0 * h, 1e-6));
        ik.update_target(&s, [0.0; 3], at([1500.0, 0.0, 0.0], [1500.0, 0.0, 0.0]));
        assert!(close(ik.target_ws[0], 1500.0 * h, 1e-6));
        // No target: easing out.
        ik.update_target(&s, [0.0; 3], None);
        assert!(ik.ease_out);
    }

    #[test]
    fn settings_read_the_ini() {
        assert_eq!(Settings::read(|_, _| None), Settings::default());
        let s = Settings::read(|section, key| match (section, key) {
            ("LookIK", "fAngleMax") => Some(10.0),
            ("RagdollAnim", "bLookIK") => Some(0.0),
            _ => None,
        });
        assert_eq!(s.angle_max, 10.0);
        assert_eq!(s.angle_max_ease, 1.0);
        assert!(!s.look_ik && s.ragdoll_anim);
    }

    #[test]
    fn a_head_sets_the_eye_cone_and_dead_zones() {
        let mut ik = LookIk::default();
        ik.head_attached(&TrackSettings::default());
        let r = |d: f64| (d * DEG_TO_RAD) as f32;
        assert_eq!(ik.bones[EYE_BONE].max_angle, r(28.0));
        assert_eq!(ik.bones[EYE_BONE].dead_heading, r(20.0));
        assert_eq!(ik.bones[EYE_BONE].dead_pitch, r(10.0));
        assert_eq!(ik.eye_pitch_range, r(20.0));
        // nv-call, 00649f00, 00649f70, 00649fe0 and 0064a070 on
        // FalloutNV.exe 1.4.0.525 (AMD Ryzen AI 9 HX 370, 2026-10-06) with
        // the settings at their defaults: bit for bit.
        let t = TrackSettings::default();
        assert_eq!(t.eye_xy().to_bits(), 0x3efa_35dd);
        assert_eq!(t.eye_z().to_bits(), 0x3eb2_b8c2);
        assert_eq!(t.dead_zone_xy().to_bits(), 0x3eb2_b8c2);
        assert_eq!(t.dead_zone_z().to_bits(), 0x3e32_b8c2);
        // 0..90 degrees, a dead zone no wider than its range.
        let wide = TrackSettings {
            eye_xy: 120.0,
            eye_z: -5.0,
            dead_zone_xy: 200.0,
            dead_zone_z: 10.0,
        };
        ik.head_attached(&wide);
        assert_eq!(ik.bones[EYE_BONE].max_angle, RIGHT_ANGLE);
        assert_eq!(ik.bones[EYE_BONE].dead_heading, RIGHT_ANGLE);
        assert_eq!(ik.bones[EYE_BONE].dead_pitch, 0.0);
        let read = TrackSettings::read(|n| (n == "fTrackEyeXY").then_some(30.0));
        assert_eq!(read.eye_xy, 30.0);
        assert_eq!(read.dead_zone_z, 10.0);
    }

    #[test]
    fn the_anchor_is_the_head_with_the_eye_height_else_nine_tenths_up() {
        let feet = [10.0, 20.0, 30.0];
        assert_eq!(
            actor_anchor(Some([1.0, 2.0, 3.0]), None, feet, 128.0),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(
            actor_anchor(Some([1.0, 2.0, 3.0]), Some(7.0), feet, 128.0),
            [1.0, 2.0, 7.0]
        );
        let height = actor_height(-4.0, 124.0, 1.0);
        let up = actor_anchor(None, None, feet, height);
        assert_eq!(&up[..2], &feet[..2]);
        assert!((up[2] - (30.0 + 0.9 * 128.0)).abs() < 1e-4);
    }

    #[test]
    fn head_tracking_turns_a_head_toward_a_target() {
        let s = Settings::default();
        // Root, neck, head 10 units above.
        let up = |z: f32| Qs {
            t: [0.0, 0.0, z, 0.0],
            ..Qs::IDENTITY
        };
        let mut pose = Pose::from_locals(vec![-1, 0, 1], vec![Qs::IDENTITY, up(10.0), up(2.0)]);
        let mut ik = LookIk::default();
        assert!(ik.init(&mut pose, Some(2), false, 60.0));
        ik.update_target(&s, [0.0; 3], at([300.0, 300.0, 84.0], [300.0, 300.0, 0.0]));
        let world = Qs::IDENTITY;
        let mut last = 0.0;
        for _ in 0..40 {
            let mut p =
                Pose::from_locals(vec![-1, 0, 1], vec![Qs::IDENTITY, up(10.0), pose.local(2)]);
            ik.do_look_at_ik(&s, &mut p, &world);
            pose = p;
            let f = rotate(pose.model(2).q, FORWARD);
            last = f[0];
        }
        // The head has turned toward +x, by no more than the cone.
        assert!(last > 0.3, "{last}");
        assert!(ik.eye_heading.is_finite() && ik.eye_pitch.is_finite());
        assert_eq!(ik.desired_eyes(), (ik.eye_heading, ik.eye_pitch));
        ik.active = false;
        assert_eq!(ik.desired_eyes(), (0.0, 0.0));
    }

    /// Root, neck, and a head 12 units up turned like a biped's
    /// `Bip01 Head`: its +X up, +Y the model's forward, +Z the model's
    /// left (so the eye point 9, 6, 0 is up and ahead).
    fn person() -> Pose {
        let up = |z: f32| Qs {
            t: [0.0, 0.0, z * havok_scale(), 0.0],
            ..Qs::IDENTITY
        };
        let head = Qs {
            q: axis_angle([0.0, 1.0, 0.0, 0.0], -std::f32::consts::FRAC_PI_2),
            ..up(12.0)
        };
        Pose::from_locals(vec![-1, 0, 1], vec![Qs::IDENTITY, up(100.0), head])
    }

    fn run_frames(ik: &mut LookIk, s: &Settings, head: &mut Qs, n: usize) {
        for _ in 0..n {
            let mut p = person();
            p.set_local(2, *head);
            ik.do_look_at_ik(s, &mut p, &Qs::IDENTITY);
            *head = p.local(2);
        }
    }

    #[test]
    fn within_the_eyes_dead_zones_only_the_eyes_move() {
        // 00c7aa60: the head turns only once the eye leaves its cone or
        // its heading/pitch passes the dead zone (00607420's limits).
        let s = Settings::default();
        let mut pose = person();
        let mut ik = LookIk::default();
        assert!(ik.init(&mut pose, Some(2), false, 60.0));
        ik.head_attached(&TrackSettings::default());
        let rest = pose.local(2);
        // A target 10 degrees to the side, far ahead: inside the dead zone.
        let a = 10f32.to_radians();
        let far = 1000.0;
        let t = [far * a.sin(), far * a.cos(), 112.0];
        ik.update_target(&s, [0.0; 3], at(t, [t[0], t[1], 0.0]));
        let mut head = rest;
        run_frames(&mut ik, &s, &mut head, 20);
        assert!(!ik.bones[HEAD_BONE].active);
        assert_eq!(head.q, rest.q);
        // To the right (+x): a positive heading, the LookRight side of
        // 0064c410.
        assert!(ik.eye_heading > 0.05, "{}", ik.eye_heading);
        assert!(ik.eye_pitch.abs() < 0.05, "{}", ik.eye_pitch);
        // 40 degrees: beyond the dead zone, the head follows.
        let a = 40f32.to_radians();
        let t = [far * a.sin(), far * a.cos(), 112.0];
        ik.update_target(&s, [0.0; 3], at(t, [t[0], t[1], 0.0]));
        run_frames(&mut ik, &s, &mut head, 40);
        assert!(ik.bones[HEAD_BONE].active);
        let f = rotate(head.q, FORWARD);
        assert!(f[0].abs() > 0.3, "{f:?}");
    }
}
