//! The game's Havok world (Havok 7.1.0-r1 as linked into FalloutNV.exe
//! 1.4.0.525), translated piece by piece (B1, docs/PHYSICS.md "Havok's
//! world step"). Names are the Xbox 360 prototype's (Xbox PDB, ADR-0002);
//! addresses are the PC executable's (ADR-0003). Everything here works in
//! Havok units (1 Havok unit = [`crate::HAVOK_UNIT`] game units) unless a
//! function takes a `unit` scale.
//!
//! This module: the world's constants and solver settings
//! (`hkpWorld::hkpWorld` `00c95a80`, `hkpWorldCinfo` defaults `00c90b80`,
//! Bethesda's `00c681c0`, `bhkWorld::ScaleSolverInfo` `00c66a00`).

use crate::Vec3;

/// `[HAVOK] fMaxTime` (setting `01267b38`; 0.016 in `Fallout_default.ini`,
/// the executable's default is 1/60): the length of one Havok step at
/// normal speed, and the world's `expectedMinPsiDeltaTime`.
pub const MAX_TIME: f32 = 0.016;
/// The world's gravity: set by the cells' `InitHavok` (`00554010`
/// exterior, `00552dc0` interior) from `011ca158` (written by `00f4b550`).
pub const GRAVITY: Vec3 = [0.0, 0.0, -98.1];
/// `hkpWorldCinfo` solver settings kept from Havok's defaults (`00c90b80`):
/// tau, damping, iterations (solver substeps before `00c66a00` rescales
/// them) and micro steps.
pub const SOLVER_TAU: f32 = 0.6;
pub const SOLVER_DAMPING: f32 = 1.0;
pub const SOLVER_ITERATIONS: i32 = 4;
pub const SOLVER_MICRO_STEPS: i32 = 1;
/// `fHavokTauRatio` (game setting at `011d1320`, copied to `011afe60` by
/// `0086f260`; default 0.5).
pub const TAU_RATIO: f32 = 0.5;
/// `hkpWorldCinfo::m_deactivationReferenceDistance` (`+0xa8`, Havok's
/// default `00c90b80`, 0x3ca3d70a).
pub const DEACTIVATION_REFERENCE_DISTANCE: f32 = 0.02;
/// `m_collisionTolerance` (`+0x54`, both cinfos).
pub const COLLISION_TOLERANCE: f32 = 0.1;
/// `m_contactRestingVelocity` (`+0x24`): Bethesda's FLT_MAX (`00c681c0`).
pub const CONTACT_RESTING_VELOCITY: f32 = f32::MAX;
/// `m_expectedMaxLinearVelocity` (`+0x60`, Havok's).
pub const EXPECTED_MAX_LINEAR_VELOCITY: f32 = 200.0;
/// `m_maxConstraintViolation` (`+0x88`, Havok's): FLT_MAX.
pub const MAX_CONSTRAINT_VIOLATION: f32 = f32::MAX;
/// `m_frameMarkerPsiSnap` (`+0xdc`).
pub const FRAME_MARKER_PSI_SNAP: f32 = 0.0001;
/// `m_enableDeactivation` (`+0xd0`) and `m_enableSimulationIslands`
/// (`+0xd2`): both on, so the world wants deactivation (`+0xd5`,
/// `00c95a80`).
pub const WANT_DEACTIVATION: bool = true;
/// `m_shouldActivateOnRigidBodyTransformChange` (`+0xa7`): Bethesda's 0.
pub const ACTIVATE_ON_TRANSFORM_CHANGE: bool = false;

/// hkUFloat8's values (the table at `010c7948`, 256 floats, read from the
/// executable): a motion's most linear and angular speed are kept as an
/// index into it (`hkpMotion` `+0xbc`, `+0xbd`).
pub const UFLOAT8: [f32; 256] = [
    0.0, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.075, 0.08, 0.085, 0.091, 0.098, 0.104, 0.111,
    0.119, 0.127, 0.136, 0.145, 0.155, 0.166, 0.177, 0.19, 0.203, 0.217, 0.231, 0.247, 0.264,
    0.282, 0.302, 0.323, 0.345, 0.368, 0.394, 0.421, 0.45, 0.481, 0.514, 0.549, 0.587, 0.627, 0.67,
    0.716, 0.765, 0.818, 0.874, 0.934, 0.998, 1.067, 1.14, 1.218, 1.302, 1.391, 1.487, 1.589,
    1.698, 1.815, 1.939, 2.072, 2.215, 2.367, 2.529, 2.703, 2.889, 3.087, 3.299, 3.526, 3.768,
    4.027, 4.304, 4.599, 4.915, 5.253, 5.613, 5.999, 6.411, 6.851, 7.322, 7.825, 8.362, 8.937,
    9.551, 10.21, 10.91, 11.66, 12.46, 13.31, 14.23, 15.2, 16.25, 17.37, 18.56, 19.83, 21.2, 22.65,
    24.21, 25.87, 27.65, 29.55, 31.57, 33.74, 36.06, 38.54, 41.18, 44.01, 47.04, 50.27, 53.72,
    57.41, 61.35, 65.57, 70.07, 74.88, 80.03, 85.52, 91.4, 97.68, 104.4, 111.6, 119.2, 127.4,
    136.2, 145.5, 155.5, 166.2, 177.6, 189.8, 202.8, 216.8, 231.7, 247.6, 264.6, 282.7, 302.2,
    322.9, 345.1, 368.8, 394.1, 421.2, 450.1, 481.1, 514.1, 549.4, 587.2, 627.5, 670.6, 716.6,
    765.9, 818.5, 874.7, 934.8, 999.0, 1068.0, 1141.0, 1219.0, 1303.0, 1393.0, 1488.0, 1590.0,
    1700.0, 1816.0, 1941.0, 2074.0, 2217.0, 2369.0, 2532.0, 2706.0, 2892.0, 3090.0, 3303.0, 3530.0,
    3772.0, 4031.0, 4308.0, 4604.0, 4920.0, 5258.0, 5619.0, 6005.0, 6418.0, 6858.0, 7329.0, 7833.0,
    8371.0, 8946.0, 9560.0, 10217.0, 10919.0, 11669.0, 12470.0, 13327.0, 14242.0, 15220.0, 16266.0,
    17383.0, 18577.0, 19853.0, 21217.0, 22674.0, 24231.0, 25896.0, 27674.0, 29575.0, 31607.0,
    33778.0, 36098.0, 38577.0, 41227.0, 44059.0, 47085.0, 50319.0, 53775.0, 57469.0, 61416.0,
    65635.0, 70143.0, 74961.0, 80110.0, 85612.0, 91493.0, 97777.0, 104493.0, 111670.0, 119340.0,
    127538.0, 136298.0, 145660.0, 155664.0, 166356.0, 177783.0, 189994.0, 203044.0, 216991.0,
    231895.0, 247823.0, 264846.0, 283037.0, 302478.0, 323254.0, 345457.0, 369186.0, 394544.0,
    421644.0, 450605.0, 481556.0, 514632.0, 549980.0, 587757.0, 628128.0, 671272.0, 717379.0,
    766654.0, 819313.0, 875589.0, 935730.0, 1000002.0,
];

/// A float as hkUFloat8 (`00ca9360`): 0 at or under 0.01's bits less one,
/// else a binary search of the table on the floats' bit patterns (as
/// integers), first halving at 216.8, then six halvings, then one up when
/// the value is still above and not at the last entry.
// Translated from 00ca9360 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn ufloat8(value: f32) -> u8 {
    let v = value.to_bits() as i32;
    if v <= 0x3c23_d709 {
        return 0;
    }
    let bits = |i: usize| UFLOAT8[i].to_bits() as i32;
    let (mut lo, mut hi) = if v < 0x4358_ccce {
        (0, 128)
    } else {
        (128, 256)
    };
    for _ in 0..6 {
        let mid = (lo + hi) / 2;
        if bits(mid) < v {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let mut i = (lo + hi) / 2;
    if bits(i) < v && i < 255 {
        i += 1;
    }
    i as u8
}

/// hkHalf: a float's upper 16 bits (stored by truncation, `00c95a80`; read
/// back as the bits shifted up, `00d28a30`).
pub fn half(value: f32) -> u16 {
    (value.to_bits() >> 16) as u16
}

pub fn from_half(h: u16) -> f32 {
    f32::from_bits(u32::from(h) << 16)
}

/// One deactivation class's thresholds (`hkpSolverInfo::DeactivationInfo`,
/// 0x1c bytes from solver info `+0x64`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeactivationInfo {
    pub linear_velocity_threshold_inv: f32,
    pub angular_velocity_threshold_inv: f32,
    pub slow_object_velocity_multiplier: f32,
    pub relative_sleep_velocity_threshold: f32,
    /// Squared distance a body may stray from its reference position, for
    /// the frequent check (0) and the 16-step check (1).
    pub max_dist_sqrd: [f32; 2],
    /// Squared distance of its rotation from the reference one (hkHalf).
    pub max_rot_sqrd: [u16; 2],
}

/// The world's solver settings (`hkpSolverInfo`, world `+0x1e0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverInfo {
    pub tau: f32,
    pub damping: f32,
    pub friction_tau: f32,
    pub damp_div_tau: f32,
    pub tau_div_damp: f32,
    pub damp_div_friction_tau: f32,
    pub friction_tau_div_damp: f32,
    pub integrate_velocity_factor: f32,
    pub contact_resting_velocity: f32,
    pub deactivation: [DeactivationInfo; 6],
    /// Solver substeps (`m_numSteps`, `+0x114`) and their inverse.
    pub num_steps: i32,
    pub inv_num_steps: f32,
    pub num_micro_steps: i32,
    pub inv_num_micro_steps: f32,
    pub max_constraint_violation_sqrd: f32,
    /// `m_deactivationNumInactiveFramesSelectFlag[2]` (`+0x125`) and
    /// `m_deactivationIntegrateCounter` (`+0x127`); all 0 at the start
    /// (cinfo `+0xa4`..`+0xa6`, `00c90b80`).
    pub select_flags: [u8; 2],
    pub integrate_counter: u8,
}

impl Default for SolverInfo {
    fn default() -> Self {
        SolverInfo::new()
    }
}

impl SolverInfo {
    /// As the world's constructor leaves it (`00c95a80`), from the cinfo in
    /// force: tau and damping through `setTauAndDamping` (`00c90e60`), the
    /// iterations and micro steps, and the six deactivation classes from
    /// |gravity| (9.81 when it's 0) and the reference distance.
    // Translated from 00c95a80 and 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn new() -> Self {
        let mut s = SolverInfo {
            tau: 0.0,
            damping: 0.0,
            friction_tau: 0.0,
            damp_div_tau: 0.0,
            tau_div_damp: 0.0,
            damp_div_friction_tau: 0.0,
            friction_tau_div_damp: 0.0,
            integrate_velocity_factor: 0.0,
            contact_resting_velocity: CONTACT_RESTING_VELOCITY,
            deactivation: [DeactivationInfo {
                linear_velocity_threshold_inv: 0.0,
                angular_velocity_threshold_inv: 0.0,
                slow_object_velocity_multiplier: 0.0,
                relative_sleep_velocity_threshold: 0.0,
                max_dist_sqrd: [0.0; 2],
                max_rot_sqrd: [0; 2],
            }; 6],
            num_steps: SOLVER_ITERATIONS,
            inv_num_steps: 1.0 / SOLVER_ITERATIONS as f32,
            num_micro_steps: SOLVER_MICRO_STEPS,
            inv_num_micro_steps: 1.0 / SOLVER_MICRO_STEPS as f32,
            max_constraint_violation_sqrd: MAX_CONSTRAINT_VIOLATION * MAX_CONSTRAINT_VIOLATION,
            select_flags: [0; 2],
            integrate_counter: 0,
        };
        s.set_tau_and_damping(SOLVER_TAU, SOLVER_DAMPING);
        let mut g = crate::vec::length(GRAVITY);
        if g == 0.0 {
            g = 9.81;
        }
        let d = DEACTIVATION_REFERENCE_DISTANCE;
        for (class, info) in s.deactivation.iter_mut().enumerate() {
            let (v, a) = match class {
                0 | 1 => (1.192_092_9e-7, 0.0),
                2 => (0.01, 0.08),
                3 => (0.017, 0.2),
                4 => (0.02, 0.3),
                _ => (0.025, 0.4),
            };
            let v = v * g;
            let inv = 1.0 / SOLVER_ITERATIONS as f32;
            info.slow_object_velocity_multiplier = 1.0 - inv * 0.016 * ((a * g) / v);
            info.linear_velocity_threshold_inv = 1.0 / v;
            info.angular_velocity_threshold_inv = 1.0 / (v * g * 0.1);
            info.relative_sleep_velocity_threshold = if a <= 0.0 {
                2.126_762_5e37
            } else {
                (inv * 0.016) / a
            };
            info.max_dist_sqrd = [d * d, d * 4.0 * d * 4.0];
            info.max_rot_sqrd = [half((d + d) * (d + d)), half(d * 8.0 * d * 8.0)];
        }
        s
    }

    /// `hkpSolverInfo::setTauAndDamping` (`00c90e60`): friction tau is half
    /// of tau; the four ratios; the velocity factor tau ÷ damping.
    // Translated from 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn set_tau_and_damping(&mut self, tau: f32, damping: f32) {
        self.tau = tau;
        self.damping = damping;
        self.friction_tau = 0.5 * tau;
        self.damp_div_tau = damping / tau;
        self.tau_div_damp = tau / damping;
        self.damp_div_friction_tau = damping / self.friction_tau;
        self.friction_tau_div_damp = self.friction_tau / damping;
        self.integrate_velocity_factor = tau / damping;
    }

    /// `bhkWorld::ScaleSolverInfo` (`00c66a00`, `[HAVOK] iUpdateType` 0),
    /// once a frame before the steps, with `step` the step length: with
    /// r = step ÷ `fMaxTime`, the substeps are max(2, trunc(4r)); tau is
    /// 0.6 (1 − k) + 0.6 k r with k = `fHavokTauRatio`; then damping ÷ tau
    /// and tau ÷ damping. Friction tau and the integrate velocity factor
    /// are left as the constructor set them.
    // Translated from 00c66a00 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn scale(&mut self, step: f32, tau_ratio: f32) {
        let r = step / MAX_TIME;
        let n = ((4.0 * r) as i32).max(2);
        self.num_steps = n;
        self.inv_num_steps = 1.0 / n as f32;
        // 00c66a97..00c66aaf: `(1 - k) * 0.6` goes to ST2 (`DC CA`, `FMUL ST(2),ST(0)`, with 0.6 left
        // on top), `FMULP ST3` makes `r * k` and `FMULP ST2` multiplies it by 0.6: the sum is
        // `(1 - k) * 0.6 + (r * k) * 0.6` in extended precision (0.6 is the double `010290b8`,
        // the float widened), rounded to a float once.
        let blend = f64::from(0.6f32);
        let tau = ((1.0 - f64::from(tau_ratio)) * blend
            + (f64::from(r) * f64::from(tau_ratio)) * blend) as f32;
        self.tau = tau;
        self.damp_div_tau = self.damping / tau;
        self.tau_div_damp = tau / self.damping;
    }

    /// `hkpSolverInfo::incrementDeactivationFlags` (Xbox PDB; inlined in
    /// `hkpSimulation::integrateInternal` `00cf8da0`), once a step before
    /// the islands are integrated.
    // Translated from 00cf8da0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn increment_deactivation_flags(&mut self) {
        self.integrate_counter = self.integrate_counter.wrapping_add(1);
        let c = self.integrate_counter;
        if c.wrapping_sub(4) & 7 == 0 {
            self.select_flags[0] ^= 1;
        }
        if c & 7 == 0 {
            self.select_flags[0] ^= 2;
        }
        if c & 15 == 0 {
            self.integrate_counter = 0;
            self.select_flags[1] = 1 - self.select_flags[1];
        }
    }
}

/// `hkpMotion::MotionType` values (Xbox PDB), the motion's `+0x08`.
pub mod motion_type {
    pub const DYNAMIC: u8 = 1;
    pub const SPHERE_INERTIA: u8 = 2;
    pub const BOX_INERTIA: u8 = 3;
    pub const KEYFRAMED: u8 = 4;
    pub const FIXED: u8 = 5;
    pub const THIN_BOX_INERTIA: u8 = 6;
    pub const CHARACTER: u8 = 7;
}

/// A rotation as a quaternion (x, y, z, w), as Havok keeps it.
pub type Quat = [f32; 4];

/// Any velocity component this big or bigger (or not a number) resets
/// both velocities to zero (`00d28a30`, the zero vector at `01268370`).
pub const VELOCITY_LIMIT: f32 = 1e6;
/// The most half-turn a step may make, in the integrator's measure
/// (|ω| dt ÷ π; `00d28a30`).
pub const MAX_STEP_TURN: f32 = 0.9;
/// 4 ÷ π² (as the executable has it).
const FOUR_OVER_PI_SQ: f32 = 0.405_284_7;

/// `q · r`, Havok's quaternion product (`00c66320`).
// Translated from 00c66320 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn quat_mul(q: Quat, r: Quat) -> Quat {
    [
        r[3] * q[0] + q[3] * r[0] + (q[1] * r[2] - q[2] * r[1]),
        r[3] * q[1] + q[3] * r[1] + (q[2] * r[0] - q[0] * r[2]),
        r[3] * q[2] + q[3] * r[2] + (q[0] * r[1] - q[1] * r[0]),
        r[3] * q[3] - (q[2] * r[2] + q[1] * r[1] + q[0] * r[0]),
    ]
}

/// The quaternion made unit length (`005611c0` → `005611e0`; the
/// executable takes the reciprocal square root with `rsqrtss` and one
/// Newton step, here exactly).
pub fn quat_normalize(q: Quat) -> Quat {
    let l = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];
    if l == 0.0 {
        return q;
    }
    let s = 1.0 / l.sqrt();
    [q[0] * s, q[1] * s, q[2] * s, q[3] * s]
}

/// A body's motion as Havok steps it (`hkpMotion`, 0x120 bytes at entity
/// `+0xe0`; fields named by the Xbox PDB). Positions and velocities are in
/// world units: Havok units scaled by the `unit` the step functions take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    /// `m_type` (`+0x08`, [`motion_type`]).
    pub kind: u8,
    /// The swept transform's `m_centerOfMass1` (`+0x60`): the centre of
    /// mass now; `m_centerOfMass0` (`+0x50`): at the step's start.
    pub center: Vec3,
    pub center0: Vec3,
    /// `m_rotation1` (`+0x80`) and `m_rotation0` (`+0x70`).
    pub rotation: Quat,
    pub rotation0: Quat,
    /// `m_linearVelocity` (`+0xd0`), `m_angularVelocity` (`+0xe0`, world).
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    /// `+0xb4`, `+0xb8`.
    pub linear_damping: f32,
    pub angular_damping: f32,
    /// `m_maxLinearVelocity`, `m_maxAngularVelocity` (hkUFloat8, `+0xbc`,
    /// `+0xbd`; Havok units a second and radians a second).
    pub max_linear_velocity: u8,
    pub max_angular_velocity: u8,
    /// `m_gravityFactor` (hkHalf, `+0x11e`).
    pub gravity_factor: u16,
    /// `m_objectRadius` (`+0xb0`, Havok units).
    pub object_radius: f32,
    /// `m_deactivationClass` (`+0xbe`).
    pub deactivation_class: u8,
    pub deactivation: Deactivation,
}

impl Motion {
    /// A still motion at `center`/`rotation`, with a gravity factor of 1.
    pub fn new(kind: u8, center: Vec3, rotation: Quat) -> Motion {
        Motion {
            kind,
            center,
            center0: center,
            rotation,
            rotation0: rotation,
            linear_velocity: [0.0; 3],
            angular_velocity: [0.0; 3],
            linear_damping: 0.0,
            angular_damping: 0.0,
            max_linear_velocity: 255,
            max_angular_velocity: 255,
            gravity_factor: half(1.0),
            object_radius: 0.0,
            deactivation_class: 2,
            deactivation: Deactivation::default(),
        }
    }

    /// The forces part of a step: gravity × the gravity factor (not for
    /// keyframed, fixed or character motions) and damping
    /// v × max(0, 1 − dt × damping) for both velocities (not for keyframed
    /// or fixed). The same rule builds the solver's velocity accumulators
    /// (`hkRigidMotionUtilApplyForcesAndBuildAccumulators` `00d29830`),
    /// whose gravity the solver adds per substep instead.
    // Translated from 00d28a30 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn apply_forces(&mut self, dt: f32, gravity_per_step: Vec3) {
        use motion_type::*;
        match self.kind {
            KEYFRAMED | FIXED => return,
            CHARACTER => {}
            _ => {
                let gf = from_half(self.gravity_factor);
                for (v, g) in self.linear_velocity.iter_mut().zip(gravity_per_step) {
                    *v += gf * g;
                }
            }
        }
        self.apply_damping(dt);
    }

    /// Damping alone (`00d28a30`, `00d29830`).
    pub fn apply_damping(&mut self, dt: f32) {
        let l = (1.0 - dt * self.linear_damping).max(0.0);
        self.linear_velocity = crate::vec::scale(self.linear_velocity, l);
        let a = (1.0 - self.angular_damping * dt).max(0.0);
        self.angular_velocity = crate::vec::scale(self.angular_velocity, a);
    }

    /// Both velocities reset when any component is at least
    /// [`VELOCITY_LIMIT`] Havok units (or not a number): to the vector at
    /// `01268370`, which the static initializer `00fbdde0` fills with the
    /// unit x (1, 0, 0, 0) — so 1 Havok unit a second along x and 1 rad a
    /// second about x, not zero (corrected in PR 7).
    pub fn reset_invalid_velocities(&mut self, unit: f32) {
        let limit = VELOCITY_LIMIT * unit;
        let ok = |v: Vec3, l: f32| v.iter().all(|c| c.abs() < l);
        if !(ok(self.linear_velocity, limit) && ok(self.angular_velocity, VELOCITY_LIMIT)) {
            self.linear_velocity = [unit, 0.0, 0.0];
            self.angular_velocity = [1.0, 0.0, 0.0];
        }
    }

    /// The linear velocity held to the most linear speed.
    pub fn clamp_linear_velocity(&mut self, unit: f32) {
        let max = UFLOAT8[self.max_linear_velocity as usize] * unit;
        let v = self.linear_velocity;
        let l2 = crate::vec::dot(v, v);
        if max * max < l2 {
            self.linear_velocity = crate::vec::scale(v, (1.0 / l2.sqrt()) * max);
        }
    }

    /// The step's half turn `ω dt / 2` and the quaternion's w, with the
    /// turn held to min(most angular speed × dt, [`MAX_STEP_TURN`]) in
    /// the measure |a|² × 4/π² (the angular velocity scaled down with it).
    fn half_turn(&mut self, dt: f32) -> Quat {
        let h = dt * 0.5;
        let mut a = crate::vec::scale(self.angular_velocity, h);
        let mut x = crate::vec::dot(a, a) * FOUR_OVER_PI_SQ;
        let lim = (UFLOAT8[self.max_angular_velocity as usize] * dt).min(MAX_STEP_TURN);
        if lim * lim < x {
            let s = (1.0 / x.sqrt()) * lim;
            self.angular_velocity = crate::vec::scale(self.angular_velocity, s);
            a = crate::vec::scale(a, s);
            x = lim * lim;
        }
        let w = ((1.0 - x * 0.822_948) - x * x * 0.130_529) - x * 0.044_408 * x * x;
        [a[0], a[1], a[2], w]
    }

    /// The angular velocity held as [`Motion::integrate`] holds it, without
    /// turning the body (for bodies this solver's contacts move).
    pub fn clamp_angular_velocity(&mut self, dt: f32) {
        self.half_turn(dt);
    }

    /// The integration part of a step: invalid velocities reset, the swept
    /// transform's start set to its end, the linear velocity clamped, the
    /// centre moved by dt × v, the rotation turned by the half turn
    /// (`q = (a, w) · rotation0`, normalized).
    // Translated from 00d28a30 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn integrate(&mut self, dt: f32, unit: f32) {
        self.reset_invalid_velocities(unit);
        self.center0 = self.center;
        self.clamp_linear_velocity(unit);
        for k in 0..3 {
            self.center[k] += dt * self.linear_velocity[k];
        }
        self.rotation0 = self.rotation;
        let q = self.half_turn(dt);
        self.rotation = quat_normalize(quat_mul(q, self.rotation0));
    }

    /// One step of a body with no constraints in its island
    /// (`hkRigidMotionUtilApplyForcesAndStep`, Xbox PDB, `00d28a30`): the
    /// forces, then the integration. `gravity_per_step` is gravity × dt in
    /// world units (`hkpSolverInfo::m_globalAccelerationPerStep`). A fixed
    /// motion isn't touched. (A thin box's extra inertia handling in that
    /// function isn't translated: no thin-box bodies are simulated here.)
    /// Then the deactivation bookkeeping; gives the body's count of
    /// passing checks ([`Motion::update_deactivation`]).
    // Translated from 00d28a30 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn step(
        &mut self,
        dt: f32,
        gravity_per_step: Vec3,
        unit: f32,
        solver: &SolverInfo,
    ) -> Option<u32> {
        if self.kind == motion_type::FIXED {
            return None;
        }
        self.apply_forces(dt, gravity_per_step);
        self.integrate(dt, unit);
        self.update_deactivation(solver, unit)
    }
}

impl Motion {
    /// `hkRigidMotionUtilApplyAccumulators` (Xbox PDB, `00d29bf0`) for a
    /// box motion after the solver: the motion's velocities are the
    /// solver's (`linear`, `angular` in the world); the body moves by the
    /// solver's integration velocities (`move_linear`, `move_angular`;
    /// reset to the unit x when any component is at or over 1e6 or not a
    /// number, `01268370`): the swept transform's start set to its end, the
    /// centre moved by dt × the linear one held to the most linear speed,
    /// the rotation turned by the angular one as [`Motion::integrate`]
    /// turns it (held to min(most angular speed × dt, 0.9), the motion's
    /// own angular velocity untouched). Then both velocities are held to
    /// the most speeds (|ω| itself to the most angular speed), and the
    /// deactivation bookkeeping. All in world units (`unit` per Havok
    /// unit), angular in radians.
    // Translated from 00d29bf0 (decompiled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    pub fn apply_accumulator(
        &mut self,
        linear: Vec3,
        angular: Vec3,
        move_linear: Vec3,
        move_angular: Vec3,
        dt: f32,
        unit: f32,
        solver: &SolverInfo,
    ) -> Option<u32> {
        self.linear_velocity = linear;
        self.angular_velocity = angular;
        let limit = VELOCITY_LIMIT * unit;
        let ok = |v: Vec3, l: f32| v.iter().all(|c| c.abs() < l);
        let (mut ml, mut ma) = (move_linear, move_angular);
        if !(ok(ml, limit) && ok(ma, VELOCITY_LIMIT)) {
            ml = [unit, 0.0, 0.0];
            ma = [1.0, 0.0, 0.0];
        }
        self.center0 = self.center;
        let max = UFLOAT8[self.max_linear_velocity as usize] * unit;
        let l2 = crate::vec::dot(ml, ml);
        if max * max < l2 {
            ml = crate::vec::scale(ml, (1.0 / l2.sqrt()) * max);
        }
        for (c, m) in self.center.iter_mut().zip(ml) {
            *c += dt * m;
        }
        self.rotation0 = self.rotation;
        let kept = self.angular_velocity;
        self.angular_velocity = ma;
        let q = self.half_turn(dt);
        self.angular_velocity = kept;
        self.rotation = quat_normalize(quat_mul(q, self.rotation0));
        self.clamp_linear_velocity(unit);
        let maxa = UFLOAT8[self.max_angular_velocity as usize];
        let w2 = crate::vec::dot(self.angular_velocity, self.angular_velocity);
        if maxa * maxa < w2 {
            self.angular_velocity =
                crate::vec::scale(self.angular_velocity, (1.0 / w2.sqrt()) * maxa);
        }
        self.update_deactivation(solver, unit)
    }
}

/// A motion's deactivation state (`hkpMotion`, Xbox PDB names). A new
/// motion's is taken as all zero (the constructor, Xbox `8295e5a8`, isn't
/// traced on the PC).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Deactivation {
    /// `m_deactivationIntegrateCounter` (`+0x09`): counts steps, 1 to 16
    /// then 0; 0xff means never checked.
    pub counter: u8,
    /// `m_deactivationNumInactiveFrames[2]` (`+0x0a`): bits 0–6 the
    /// passing checks in a row (at most 64), bits 7–13 the count before,
    /// bits 14–15 the solver's select flag.
    pub inactive: [u16; 2],
    /// `m_deactivationRefPosition[2]` (`+0xf0`): where the body was when
    /// the count began (world units), and in `w` the largest
    /// |v|² + min(radius, 1)² |ω|² (Havok units) seen since.
    pub ref_position: [[f32; 4]; 2],
    /// `m_deactivationRefOrientation[2]` (`+0x110`): its rotation then,
    /// compressed ([`compress_quat`]).
    pub ref_orientation: [u32; 2],
}

/// A rotation in four bytes (`00d975d0`): each component × 116.36363 +
/// 196736.5 as a float, its bits 6–13 (so round(c × 116.36) + 128).
// Translated from 00d975d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn compress_quat(q: Quat) -> u32 {
    let byte = |c: f32| ((c * 116.363_63 + 196_736.5).to_bits() >> 6) & 0xff;
    byte(q[0]) | byte(q[1]) << 8 | byte(q[2]) << 16 | byte(q[3]) << 24
}

/// The bytes back: (b − 128) × 0.00859375 (`00d28a30`).
pub fn decompress_quat(c: u32) -> Quat {
    let f = |k: u32| (((c >> (8 * k)) & 0xff) as i32 - 0x80) as f32 * 0.008_593_75;
    [f(0), f(1), f(2), f(3)]
}

/// An island is deactivated once the fewest checks in a row any of its
/// bodies has passed is more than this (`00cf8da0`).
pub const INACTIVE_FRAMES_TO_DEACTIVATE: u32 = 5;

impl Motion {
    /// |v|² + min(radius, 1)² |ω|², in Havok units.
    fn energy(&self, unit: f32) -> f32 {
        let r = self.object_radius.min(1.0);
        let v = crate::vec::scale(self.linear_velocity, 1.0 / unit);
        let w = self.angular_velocity;
        r * r * crate::vec::dot(w, w) + crate::vec::dot(v, v)
    }

    /// The more frequent of the two checks' counts of passing checks in a
    /// row.
    pub fn inactive_frames(&self) -> u32 {
        let d = &self.deactivation;
        u32::from(d.inactive[0] & 0x7f).max(u32::from(d.inactive[1] & 0x7f))
    }

    /// The deactivation bookkeeping at the end of a body's step
    /// (`hkRigidMotionUtilApplyForcesAndStep` `00d28a30` and
    /// `hkRigidMotionUtilApplyAccumulators` `00d29bf0`): the counter goes
    /// up; every 4th step a check, of slot 1 every 16th (counter back to
    /// 0), else slot 0. The slot's reference energy keeps its largest
    /// value. When the centre is within the class's distance of the slot's
    /// reference position and the rotation within its distance of the
    /// reference rotation, the slot's count goes up by one (held at 64);
    /// otherwise it goes to 0 and the references are set to where the body
    /// is now. Gives the larger count of the two slots (none for a fixed
    /// motion).
    // Translated from 00d28a30 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update_deactivation(&mut self, solver: &SolverInfo, unit: f32) -> Option<u32> {
        if self.kind == motion_type::FIXED {
            return None;
        }
        let c = u32::from(self.deactivation.counter) + 1;
        self.deactivation.counter = c as u8;
        if c & 3 == 0 {
            let j = if c & 15 == 0 {
                if c == 0x100 {
                    self.deactivation.counter = 0xff;
                    return Some(self.inactive_frames());
                }
                self.deactivation.counter = 0;
                1
            } else {
                0
            };
            let e = self.energy(unit);
            let info = &solver.deactivation[usize::from(self.deactivation_class).min(5)];
            let d = &mut self.deactivation;
            d.ref_position[j][3] = d.ref_position[j][3].max(e);
            let r = d.ref_position[j];
            let dist = crate::vec::dist2([r[0], r[1], r[2]], self.center);
            let flag = u16::from(solver.select_flags[j]);
            let old = d.inactive[j] & 0x7f;
            let mut passed = false;
            if dist <= info.max_dist_sqrd[j] * unit * unit {
                let q = decompress_quat(d.ref_orientation[j]);
                let rot: f32 = (0..4).map(|k| (q[k] - self.rotation[k]).powi(2)).sum();
                if rot <= from_half(info.max_rot_sqrd[j]) {
                    let count = old - (old >> 6) + 1;
                    d.inactive[j] = count | ((flag << 7 | old) << 7);
                    passed = true;
                }
            }
            if !passed {
                d.inactive[j] = (flag << 7 | old) << 7;
                d.ref_position[j] = [self.center[0], self.center[1], self.center[2], 0.0];
                d.ref_orientation[j] = compress_quat(self.rotation);
            }
        }
        Some(self.inactive_frames())
    }

    /// The last test before an island marked inactive goes to sleep
    /// (`00d28560`, from the dirty-island cleanup `00cb55d0` →
    /// `00cb5310`): it fails when a quarter of the body's energy now less
    /// 0.01 is more than the largest energy its counting slot (the one with
    /// the larger count, slot 1 on a tie) has seen.
    // Translated from 00d28560 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn can_deactivate(&self, unit: f32) -> bool {
        let d = &self.deactivation;
        let j = if (d.inactive[1] & 0x7f) < (d.inactive[0] & 0x7f) {
            0
        } else {
            1
        };
        let fails = d.ref_position[j][3] < self.energy(unit) * 0.25 - 0.010_000_001;
        !fails
    }

    /// Going to sleep with its island (`00cb5310`): both velocities set to
    /// zero (`hkpMotion::setLinearVelocity`/`setAngularVelocity`, vtable
    /// `+0x40`/`+0x44`, with the zero vector at `01267e30`).
    // Translated from 00cb5310 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn deactivate(&mut self) {
        self.linear_velocity = [0.0; 3];
        self.angular_velocity = [0.0; 3];
    }

    /// Woken with its island (`00cb5100`): both counts start again at 0,
    /// keeping the solver's select flags (each flipped when the solver's
    /// counter is behind the body's).
    // Translated from 00cb5100 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn activate(&mut self, solver: &SolverInfo) {
        let d = &mut self.deactivation;
        let mut f0 = u16::from(solver.select_flags[0]);
        let mut f1 = u16::from(solver.select_flags[1]);
        if (solver.integrate_counter & 3) < (d.counter & 3) {
            f0 = !f0;
        }
        if solver.integrate_counter < d.counter {
            f1 = !f1;
        }
        d.inactive = [f0 << 14, f1 << 14];
    }
}

/// The most steps one frame takes (`00c66760`: 3, or 1 when its second
/// argument is set; the caller passes `00525420(...)` tested on a game
/// mode of 4, not traced further: taken as unset).
pub const MAX_STEPS: u32 = 3;
/// The most time the clock holds (`00c66760`).
pub const MAX_ACCUMULATED: f32 = 166.666_67;

/// The game's Havok frame clock and step driver: `bhkWorld::SetDeltaTime`
/// (`00c66760`, Xbox `8293ce80`) with `[HAVOK] iUpdateType` 0, as
/// `bhkWorld::Update` (`00c6ae70`) uses it.
///
/// Each frame the frame time joins the time carried over (`012677b0`),
/// held to [`MAX_ACCUMULATED`]; n = that ÷ the step, rounded (the x87's
/// round to nearest, halves to even), at most [`Clock::max_steps`]; the
/// rest (negative after rounding up) is carried, held to at most one
/// step (`0040ebd0` with a setting read through `00403e20`, taken as the
/// step: not traced). Under half a step waits for the next frame.
///
/// `bhkWorld::Update` then sets the step to `fMaxTime` × the time
/// multiplier again, sets the frame marker (`hkpWorld::setFrameTimeMarker`
/// `00c91040`: current time + the accumulated time) and calls
/// `hkpWorld::stepDeltaTime` (`00c91b10`) until the simulation is at the
/// marker (`00c91060`, exact equality): exactly n whole steps. Bodies are
/// read where the last whole step left them: nothing is interpolated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    /// `fMaxTime` (`01267b38`).
    pub max_time: f32,
    /// The time multiplier (`011ac3a0`, eased toward `011ac3a4` by
    /// `00aa4e40`; 1 at normal speed).
    pub time_multiplier: f32,
    pub max_steps: u32,
    /// The third argument or `01267b4c`: run a step even under half a
    /// step's time (not set by anything nv-rs does).
    pub forced: bool,
    /// Time not yet stepped (`012677b0`).
    left: f32,
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new(MAX_TIME)
    }
}

impl Clock {
    pub fn new(max_time: f32) -> Self {
        Clock {
            max_time,
            time_multiplier: 1.0,
            max_steps: MAX_STEPS,
            forced: false,
            left: 0.0,
        }
    }

    /// The step length (`012677ac` = `01267b38` × `011ac3a0`).
    pub fn step(&self) -> f32 {
        self.max_time * self.time_multiplier
    }

    /// Time carried to the next frame.
    pub fn left(&self) -> f32 {
        self.left
    }

    /// The whole steps a frame of `frame` seconds runs, and their length.
    // Translated from 00c66760 and 00c6ae70 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn advance(&mut self, frame: f32) -> (u32, f32) {
        let step = self.step();
        let t = (self.left + frame).min(MAX_ACCUMULATED);
        if t <= 0.0 {
            // The carried time isn't touched on this path.
            return (0, 0.0);
        }
        let q = t / step;
        let mut r = q.round();
        if (q - q.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
            r -= q.signum();
        }
        let n = (r as u32).min(self.max_steps);
        if n != 0 {
            self.left = (t - n as f32 * step).min(step);
            return (n, step);
        }
        if !self.forced && t < step * 0.5 {
            self.left = t;
            return (0, 0.0);
        }
        // Exactly half a step rounded down to 0 (or forced): the whole
        // time is the marker and nothing is carried; `bhkWorld::Update`
        // still steps by the fixed step (one step here; Havok's snapping
        // of the last step to the marker isn't traced).
        self.left = 0.0;
        (1, step)
    }
}

/// The fewest whole milliseconds the game's frame timer reports (`00aa4ee0`
/// sleeps out a shorter frame and reports 10).
pub const FRAME_MIN_MS: u32 = 10;
/// The most whole milliseconds it reports (`00aa4ee0`: 0xa6; the time
/// beyond is lost, so the game slows down).
pub const FRAME_MAX_MS: u32 = 166;

/// The game's frame timer as the Havok clock is given it: the frame time
/// that `00aa4ee0` stores at `+0xc` of the timer at `011f6394`, which
/// `0086f260` passes to `bhkWorld::SetDeltaTime` ([`Clock::advance`]).
///
/// The game measures each frame in whole milliseconds of `GetTickCount`;
/// a frame under [`FRAME_MIN_MS`] is slept out (`Sleep(10 - ms)`) and
/// reported as 10, one over [`FRAME_MAX_MS`] as 166; the result × the time
/// multiplier (1 at normal speed, as [`Clock`] takes it). So the clock never
/// sees a frame under 0.01 s. The viewer's frames are shorter (it isn't
/// held to 100 a second): their time is gathered here until a game frame's
/// worth has passed, as if the game were sleeping meanwhile, and then given
/// as one frame of the whole milliseconds gathered (the fraction kept for
/// the next).
///
/// Without this the clock's carried time goes below zero (a frame of 0.6
/// steps runs a whole step and carries −0.4 of one), and a following frame
/// shorter than that leaves the total at or under zero: the path on which
/// `00c66760` returns without touching the carried time, so no step runs
/// again while frames stay that short (bodies hang in the air).
// Translated from 00aa4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FrameTimer {
    /// Milliseconds gathered since the last game frame.
    pending: f64,
}

impl FrameTimer {
    /// A viewer frame of `seconds`: the game frame's time in seconds when
    /// one ends with it, else 0 (no game frame yet).
    pub fn frame(&mut self, seconds: f32) -> f32 {
        if seconds > 0.0 {
            self.pending += f64::from(seconds) * 1000.0;
        }
        let ms = self.pending.floor();
        if ms < f64::from(FRAME_MIN_MS) {
            return 0.0;
        }
        self.pending -= ms;
        // The 0xa6 cap: the rest of a long frame is dropped.
        let ms = (ms as u32).min(FRAME_MAX_MS);
        (f64::from(ms) * 0.001) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_frame_timer_never_gives_the_clock_a_frame_under_10_ms() {
        // 210 viewer frames a second for 2 s: game frames of 10 to 14 ms
        // (one ends with every third viewer frame or so), and the whole
        // time reaches the clock but for the fraction of a ms left.
        let mut timer = FrameTimer::default();
        let mut total = 0.0f64;
        for _ in 0..420 {
            let f = timer.frame(1.0 / 210.0);
            assert!(f == 0.0 || (0.010..=0.0145).contains(&f), "{f}");
            total += f64::from(f);
        }
        assert!((total - 2.0).abs() < 0.0011, "{total}");
        // 60 a second: 16 or 17 ms, every viewer frame.
        let mut timer = FrameTimer::default();
        for _ in 0..60 {
            let f = timer.frame(1.0 / 60.0);
            assert!((0.016..=0.0171).contains(&f), "{f}");
        }
        // A hitch of half a second reaches the clock as 166 ms.
        let mut timer = FrameTimer::default();
        assert!((timer.frame(0.5) - 0.166).abs() < 1e-6);
        assert_eq!(timer.frame(0.0), 0.0);
    }

    #[test]
    fn short_frames_through_the_timer_keep_the_clock_stepping() {
        // Straight from 210 Hz frames the carried time goes below zero
        // after the first step and no step runs again.
        let mut c = Clock::default();
        let steps: u32 = (0..420).map(|_| c.advance(1.0 / 210.0).0).sum();
        assert_eq!(steps, 1);
        // Through the game's frame timer, 2 s are 125 steps of 0.016 s,
        // give or take the carry.
        let mut c = Clock::default();
        let mut timer = FrameTimer::default();
        let mut steps = 0;
        for _ in 0..420 {
            let f = timer.frame(1.0 / 210.0);
            if f > 0.0 {
                steps += c.advance(f).0;
            }
        }
        assert!((123..=126).contains(&steps), "{steps}");
    }

    fn body() -> Motion {
        let mut m = Motion::new(
            motion_type::BOX_INERTIA,
            [0.0, 0.0, 100.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        m.max_linear_velocity = ufloat8(1068.0);
        m.max_angular_velocity = ufloat8(31.57);
        m
    }

    fn g_step(dt: f32) -> Vec3 {
        crate::vec::scale(GRAVITY, dt)
    }

    #[test]
    fn a_falling_body_follows_the_integrator_step_by_step() {
        let mut m = body();
        let dt = MAX_TIME;
        let (mut v, mut z) = (0.0f32, 100.0f32);
        for _ in 0..62 {
            let before = z;
            m.step(dt, g_step(dt), 1.0, &SolverInfo::new());
            // By hand, in the same order and precision.
            v += 1.0 * (-98.1 * dt);
            v *= (1.0 - dt * 0.0f32).max(0.0);
            z += dt * v;
            assert_eq!(m.linear_velocity, [0.0, 0.0, v]);
            assert_eq!(m.center, [0.0, 0.0, z]);
            assert_eq!(m.center0[2], before);
        }
        // About a second: 98.1 × 0.992 s.
        assert!((m.linear_velocity[2] + 98.1 * 62.0 * dt).abs() < 1e-3);
        assert_eq!(m.rotation, [0.0, 0.0, 0.0, 1.0]);
        // A gravity factor of 0 floats; keyframed and fixed bodies get no
        // gravity.
        let mut m = body();
        m.gravity_factor = half(0.0);
        m.step(dt, g_step(dt), 1.0, &SolverInfo::new());
        assert_eq!(m.linear_velocity, [0.0; 3]);
        for kind in [motion_type::KEYFRAMED, motion_type::FIXED] {
            let mut m = body();
            m.kind = kind;
            m.step(dt, g_step(dt), 1.0, &SolverInfo::new());
            assert_eq!(m.linear_velocity, [0.0; 3]);
        }
    }

    #[test]
    fn damping_takes_off_a_share_each_step() {
        let mut m = body();
        m.linear_damping = 0.1;
        m.angular_damping = 0.05;
        m.linear_velocity = [10.0, 0.0, 0.0];
        m.angular_velocity = [0.0, 0.0, 2.0];
        let dt = MAX_TIME;
        let (mut v, mut w) = (10.0f32, 2.0f32);
        for _ in 0..100 {
            m.step(dt, [0.0; 3], 1.0, &SolverInfo::new());
            v *= (1.0 - dt * 0.1f32).max(0.0);
            w *= (1.0 - 0.05f32 * dt).max(0.0);
            assert_eq!(m.linear_velocity[0], v);
            assert_eq!(m.angular_velocity[2], w);
        }
        // Damping past 1/dt stops the body at once.
        let mut m = body();
        m.linear_damping = 100.0;
        m.linear_velocity = [10.0, 0.0, 0.0];
        m.step(dt, [0.0; 3], 1.0, &SolverInfo::new());
        assert_eq!(m.linear_velocity, [0.0; 3]);
    }

    #[test]
    fn speed_and_spin_are_capped() {
        let dt = MAX_TIME;
        // 2000 Havok units a second against the bottle's 1068.
        let mut m = body();
        m.linear_velocity = [2000.0, 0.0, 0.0];
        m.step(dt, [0.0; 3], 1.0, &SolverInfo::new());
        assert!((m.linear_velocity[0] - 1068.0).abs() < 1e-3);
        assert!((m.center[0] - 1068.0 * dt).abs() < 1e-3);
        // In game units, the same cap scaled.
        let mut m = body();
        m.linear_velocity = [2000.0 * 7.0, 0.0, 0.0];
        m.step(dt, [0.0; 3], 7.0, &SolverInfo::new());
        assert!((m.linear_velocity[0] - 1068.0 * 7.0).abs() < 1e-2);
        // Spin: |ω| dt / π held to min(31.57 × dt, 0.9) = 0.505.
        let mut m = body();
        m.angular_velocity = [0.0, 0.0, 200.0];
        m.step(dt, [0.0; 3], 1.0, &SolverInfo::new());
        let cap = 31.57 * dt * std::f32::consts::PI / dt;
        assert!(
            (m.angular_velocity[2] - cap).abs() < 0.01,
            "{:?}",
            m.angular_velocity
        );
        // Under the cap, untouched; a quarter turn about z in 0.2 rad steps.
        let mut m = body();
        m.angular_velocity = [0.0, 0.0, 10.0];
        m.step(dt, [0.0; 3], 1.0, &SolverInfo::new());
        assert_eq!(m.angular_velocity[2], 10.0);
        let a = 10.0 * dt * 0.5;
        let x = a * a * FOUR_OVER_PI_SQ;
        let w = ((1.0 - x * 0.822_948) - x * x * 0.130_529) - x * 0.044_408 * x * x;
        let want = quat_normalize([0.0, 0.0, a, w]);
        assert_eq!(m.rotation, want);
        // The real half angle's sine and cosine, closely.
        assert!((want[2] - a.sin()).abs() < 1e-4 && (want[3] - a.cos()).abs() < 1e-4);
        // A big spin is held to 0.9 when the body allows more.
        let mut m = body();
        m.max_angular_velocity = 255;
        m.angular_velocity = [500.0, 0.0, 0.0];
        m.step(0.016, [0.0; 3], 1.0, &SolverInfo::new());
        let cap = MAX_STEP_TURN * std::f32::consts::PI / 0.016;
        assert!(
            (m.angular_velocity[0] - cap).abs() < 0.05,
            "{:?}",
            m.angular_velocity
        );
    }

    #[test]
    fn invalid_velocities_are_reset() {
        let mut m = body();
        m.linear_velocity = [f32::NAN, 0.0, 0.0];
        m.angular_velocity = [1.0, 0.0, 0.0];
        m.step(MAX_TIME, [0.0; 3], 1.0, &SolverInfo::new());
        // Reset to the unit x (`01268370`), then integrated.
        assert_eq!(m.linear_velocity, [1.0, 0.0, 0.0]);
        assert_eq!(m.angular_velocity, [1.0, 0.0, 0.0]);
        let mut m = body();
        m.max_linear_velocity = 255;
        m.angular_velocity = [0.0, 2e6, 0.0];
        m.step(MAX_TIME, [0.0; 3], 1.0, &SolverInfo::new());
        assert_eq!(m.angular_velocity, [1.0, 0.0, 0.0]);
        assert_eq!(m.center, [MAX_TIME, 0.0, 100.0]);
    }

    /// Steps a motion with no gravity until its count passes 5; the step
    /// that did it (None within `limit`).
    fn steps_to_rest(m: &mut Motion, limit: u32) -> Option<u32> {
        let mut s = SolverInfo::new();
        for k in 1..=limit {
            s.increment_deactivation_flags();
            let frames = m.step(MAX_TIME, [0.0; 3], 1.0, &s).unwrap();
            if frames > INACTIVE_FRAMES_TO_DEACTIVATE {
                return Some(k);
            }
        }
        None
    }

    #[test]
    fn a_still_body_passes_on_its_sixth_check() {
        // From a new motion (references zero): the first check (step 4)
        // sets the references; slot 0 then passes at 8, 12, 20, 24, 28,
        // 36 (16 and 32 are slot 1's): the sixth pass is at step 36.
        let mut m = body();
        m.center = [10.0, 20.0, 30.0];
        assert_eq!(steps_to_rest(&mut m, 200), Some(36));
        assert!(m.can_deactivate(1.0));
        // Packed: count 6, the count before (5) in bits 7–13.
        assert_eq!(m.deactivation.inactive[0] & 0x3fff, 6 | 5 << 7);
        // Asleep: velocities zeroed. Woken: the counts start again, so it
        // takes 6 more passes of slot 0; its counter stood at 4, so they
        // come 8 to 36 on it, skipping 16 and 32: 32 steps.
        m.linear_velocity = [0.001, 0.0, 0.0];
        m.deactivate();
        assert_eq!(m.linear_velocity, [0.0; 3]);
        let s = SolverInfo::new();
        m.activate(&s);
        assert_eq!(m.inactive_frames(), 0);
        assert_eq!(steps_to_rest(&mut m, 200), Some(32));
    }

    #[test]
    fn a_creeping_body_never_passes() {
        // 0.004 Havok units a step: 0.016 between checks, but its
        // reference stays put, so every second check fails.
        let mut m = body();
        m.linear_velocity = [0.004 / MAX_TIME, 0.0, 0.0];
        assert_eq!(steps_to_rest(&mut m, 1000), None);
        // Turning slowly (0.01 rad a step) never passes either.
        let mut m = body();
        m.angular_velocity = [0.0, 0.0, 0.01 / MAX_TIME];
        assert_eq!(steps_to_rest(&mut m, 1000), None);
        // A body that wanders within 0.08 but past 0.02 between checks:
        // only slot 1 (every 16 steps) can count; it isn't reached here
        // as the drift adds up.
        let mut m = body();
        m.linear_velocity = [0.006 / MAX_TIME, 0.0, 0.0];
        assert_eq!(steps_to_rest(&mut m, 1000), None);
    }

    #[test]
    fn a_body_moving_faster_than_it_counted_stays_awake() {
        let mut m = body();
        m.center = [10.0, 20.0, 30.0];
        steps_to_rest(&mut m, 200);
        assert!(m.can_deactivate(1.0));
        // Suddenly moving at 1 unit a second: 1/4 − 0.01 > 0.
        m.linear_velocity = [1.0, 0.0, 0.0];
        assert!(!m.can_deactivate(1.0));
    }

    #[test]
    fn rotations_compress_to_a_byte_each() {
        let q = quat_normalize([0.1, -0.2, 0.3, 0.9]);
        let back = decompress_quat(compress_quat(q));
        for k in 0..4 {
            assert!((back[k] - q[k]).abs() <= 0.5 / 116.0, "{back:?} {q:?}");
        }
        assert_eq!(
            compress_quat([0.0, 0.0, 0.0, 1.0]),
            0x80 | 0x80 << 8 | 0x80 << 16 | 244 << 24
        );
    }

    #[test]
    fn frames_run_whole_steps_and_carry_the_rest() {
        let mut c = Clock::default();
        // 60 Hz: one step a frame, 0.000667 s carried each time, so every
        // 24th frame or so runs two.
        let mut steps = 0;
        let mut twos = 0;
        for _ in 0..240 {
            let (n, dt) = c.advance(1.0 / 60.0);
            assert_eq!(dt, 0.016);
            assert!(n == 1 || n == 2, "{n}");
            twos += (n == 2) as u32;
            steps += n;
        }
        // 4 s of frames: 250 steps of 0.016 s, give or take the carry.
        assert!((249..=250).contains(&steps), "{steps}");
        assert!(twos > 0);
        // Frame times from 5 to 100 ms.
        for (frame, want) in [
            (0.005, 0),
            (0.009, 1),
            (0.016, 1),
            (0.025, 2),
            (0.033, 2),
            (0.05, 3),
            (0.1, 3),
        ] {
            let mut c = Clock::default();
            assert_eq!(c.advance(frame).0, want, "{frame}");
        }
        // A long frame: three steps, one step's time carried.
        let mut c = Clock::default();
        assert_eq!(c.advance(0.2), (3, 0.016));
        assert!((c.left() - 0.016).abs() < 1e-7);
        // Half speed: steps of 0.008.
        let mut c = Clock {
            time_multiplier: 0.5,
            ..Clock::default()
        };
        assert_eq!(c.advance(0.016), (2, 0.008));
        // One step only.
        let mut c = Clock {
            max_steps: 1,
            ..Clock::default()
        };
        assert_eq!(c.advance(0.05).0, 1);
    }

    #[test]
    fn the_speed_table_holds_the_bodies_values() {
        // Ends, and the bottle's stored speeds (Havok units, rad/s).
        assert_eq!(UFLOAT8[0], 0.0);
        assert_eq!(UFLOAT8[1], 0.01);
        assert_eq!(UFLOAT8[255], 1_000_002.0);
        // The literals are the executable's floats bit for bit.
        assert_eq!(UFLOAT8[1].to_bits(), 0x3c23_d70a);
        assert_eq!(UFLOAT8[128].to_bits(), 0x4358_cccd);
        assert_eq!(UFLOAT8[255].to_bits(), 0x4974_2420);
        assert_eq!(UFLOAT8[ufloat8(1068.0) as usize], 1068.0);
        assert_eq!(UFLOAT8[ufloat8(31.57) as usize], 31.57);
        assert_eq!(UFLOAT8[ufloat8(216.8) as usize], 216.8);
        // Between two entries: the one above. Under 0.01: 0. Huge: 255.
        assert_eq!(UFLOAT8[ufloat8(1000.0) as usize], 1068.0);
        assert_eq!(ufloat8(0.005), 0);
        assert_eq!(ufloat8(1e9), 255);
        // Every entry encodes to itself.
        for (i, &v) in UFLOAT8.iter().enumerate().skip(1) {
            assert_eq!(ufloat8(v) as usize, i, "{v}");
        }
    }

    #[test]
    fn halves_keep_the_upper_bits() {
        assert_eq!(from_half(half(1.0)), 1.0);
        let h = from_half(half(0.0016));
        assert!(h <= 0.0016 && h > 0.0015, "{h}");
    }

    #[test]
    fn the_world_starts_with_havoks_solver_settings() {
        let s = SolverInfo::new();
        assert_eq!((s.tau, s.damping, s.friction_tau), (0.6, 1.0, 0.3));
        assert_eq!((s.num_steps, s.num_micro_steps), (4, 1));
        assert!((s.tau_div_damp - 0.6).abs() < 1e-7);
        // Deactivation: 0.02 and 0.08 Havok units for every class.
        for d in &s.deactivation {
            assert!((d.max_dist_sqrd[0] - 0.0004).abs() < 1e-9);
            assert!((d.max_dist_sqrd[1] - 0.0064).abs() < 1e-8);
            assert_eq!(d.max_rot_sqrd, [half(0.0016), half(0.0256)]);
        }
        // The medium class: velocity 0.017 g, angle 0.2.
        let m = &s.deactivation[3];
        assert!((m.linear_velocity_threshold_inv - 1.0 / (0.017 * 98.1)).abs() < 1e-6);
        assert!((m.relative_sleep_velocity_threshold - 0.004 / 0.2).abs() < 1e-7);
    }

    #[test]
    fn substeps_and_tau_follow_the_time_multiplier() {
        let mut s = SolverInfo::new();
        s.scale(MAX_TIME, TAU_RATIO);
        assert_eq!(s.num_steps, 4);
        assert!((s.tau - 0.6).abs() < 1e-6);
        // Half speed: 2 substeps, tau 0.45; quarter speed: still 2.
        s.scale(MAX_TIME * 0.5, TAU_RATIO);
        assert_eq!(s.num_steps, 2);
        assert!((s.tau - 0.45).abs() < 1e-6, "{}", s.tau);
        s.scale(MAX_TIME * 0.25, TAU_RATIO);
        assert_eq!(s.num_steps, 2);
        // Friction tau isn't refreshed.
        assert_eq!(s.friction_tau, 0.3);
    }

    /// 00c66aa7 is `DC CA`: the blend is `(1 - k) * 0.6 + (r * k) * 0.6` (0.6 the double
    /// `010290b8`) summed in extended precision and rounded to a float once. With k = 0.05f
    /// and r = 0.15f (the step is `0.15 * MAX_TIME`): `(1 - 0.05f) * 0.6f + (0.15f * 0.05f)
    /// * 0.6f` = 0.574500024..., the float 0x3f13126f; float arithmetic step by step with
    /// `0.6 * k * r` (the decompiler's reading) gives 0x3f13126e.
    #[test]
    fn tau_blend_follows_the_exe_operation_order() {
        let mut s = SolverInfo::new();
        s.scale(0.15f32 * MAX_TIME, 0.05);
        assert_eq!(s.num_steps, 2);
        assert_eq!(s.tau.to_bits(), 0x3f13_126f);
    }

    #[test]
    fn deactivation_flags_turn_over_every_sixteen_steps() {
        let mut s = SolverInfo::new();
        let mut seen = Vec::new();
        for _ in 0..16 {
            s.increment_deactivation_flags();
            seen.push((s.integrate_counter, s.select_flags));
        }
        assert_eq!(seen[3], (4, [1, 0]));
        assert_eq!(seen[7], (8, [3, 0]));
        assert_eq!(seen[11], (12, [2, 0]));
        assert_eq!(seen[15], (0, [0, 1]));
    }
}
