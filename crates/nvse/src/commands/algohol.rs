//! Algohol's vector and quaternion commands as xNVSE carries them (`commands_Algohol.cpp`,
//! `Algohol/algMath.cpp`, `Algohol/algTypes.h` (xNVSE); Algohol is entim's, MIT, notice in
//! `LICENSE-algohol`), batch X20 of ADR-0016's build order.
//!
//! All arithmetic is `float`, in the order the C++ writes it (MSVC's SSE2 code rounds each
//! operation to `float`, as Rust's `f32` does). The older commands take the names of the
//! script's variables as strings and write them by name (`setVarByName`, compared with
//! `strcmp`, so case matters); the `...Ex` commands take the variables themselves.

use super::Read;
use crate::args::event_list_variable;
use crate::table::{guest_text, Command, Param, Returns};
use crate::{handled, CommandArgs};
use engine_core::prelude::*;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}
const S: u32 = 0x00;
const F: u32 = 0x02;
const I: u32 = 0x01;
const V: u32 = 0x2d;

// The parameter lists of `Algohol/paramTypes.cpp` (xNVSE).
const VECTOR3_FLOATS: &[Param] = &[
    p("vector_x", F, false),
    p("vector_y", F, false),
    p("vector_z", F, false),
];
const VECTOR3_STRINGS3_FLOATS: &[Param] = &[
    p("vector_x_name", S, false),
    p("vector_y_name", S, false),
    p("vector_z_name", S, false),
    p("vector_x", F, false),
    p("vector_y", F, false),
    p("vector_z", F, false),
];
const VECTOR3_STRINGS6_FLOATS: &[Param] = &[
    p("vector_x_name", S, false),
    p("vector_y_name", S, false),
    p("vector_z_name", S, false),
    p("vectorA_x", F, false),
    p("vectorA_y", F, false),
    p("vectorA_z", F, false),
    p("vectorB_x", F, false),
    p("vectorB_y", F, false),
    p("vectorB_z", F, false),
];
const QUAT_EULER_4STRINGS_3FLOATS_1INT: &[Param] = &[
    p("quat_w_name", S, false),
    p("quat_x_name", S, false),
    p("quat_y_name", S, false),
    p("quat_z_name", S, false),
    p("elevation", F, false),
    p("bank", F, false),
    p("heading", F, false),
    p("actorFlag", I, true),
];
const QUAT_AXIS_ANGLE_4STRINGS_4FLOATS: &[Param] = &[
    p("quat_w_name", S, false),
    p("quat_x_name", S, false),
    p("quat_y_name", S, false),
    p("quat_z_name", S, false),
    p("axis_x", F, false),
    p("axis_y", F, false),
    p("axis_z", F, false),
    p("angle", F, false),
];
const QUAT_4STRINGS_4FLOATS: &[Param] = &[
    p("quat_w_name", S, false),
    p("quat_x_name", S, false),
    p("quat_y_name", S, false),
    p("quat_z_name", S, false),
    p("quat_w", F, false),
    p("quat_x", F, false),
    p("quat_y", F, false),
    p("quat_z", F, false),
];
const QUAT_4STRINGS_8FLOATS: &[Param] = &[
    p("quat_w_name", S, false),
    p("quat_x_name", S, false),
    p("quat_y_name", S, false),
    p("quat_z_name", S, false),
    p("quatA_w", F, false),
    p("quatA_x", F, false),
    p("quatA_y", F, false),
    p("quatA_z", F, false),
    p("quatB_w", F, false),
    p("quatB_x", F, false),
    p("quatB_y", F, false),
    p("quatB_z", F, false),
];
const VECTOR_QUAT_VECTOR3_STRINGS_7FLOATS: &[Param] = &[
    p("vector_x_name", S, false),
    p("vector_y_name", S, false),
    p("vector_z_name", S, false),
    p("quat_w", F, false),
    p("quat_x", F, false),
    p("quat_y", F, false),
    p("quat_z", F, false),
    p("vector_x", F, false),
    p("vector_y", F, false),
    p("vector_z", F, false),
];
const QUAT_4STRINGS_9FLOATS_1INT: &[Param] = &[
    p("quat_w_name", S, false),
    p("quat_x_name", S, false),
    p("quat_y_name", S, false),
    p("quat_z_name", S, false),
    p("quatA_w", F, false),
    p("quatA_x", F, false),
    p("quatA_y", F, false),
    p("quatA_z", F, false),
    p("quatB_w", F, false),
    p("quatB_x", F, false),
    p("quatB_y", F, false),
    p("quatB_z", F, false),
    p("ratio", F, false),
    p("flag", I, true),
];
const EULER_QUAT_3STRINGS_4FLOATS_1INT: &[Param] = &[
    p("elevation_name", S, false),
    p("bank_name", S, false),
    p("heading_name", S, false),
    p("quat_w", F, false),
    p("quat_x", F, false),
    p("quat_y", F, false),
    p("quat_z", F, false),
    p("actorFlag", I, true),
];
const SV: Param = p("Script Var", V, false);
const FL: Param = p("Float", F, false);
const FLAG: Param = p("flag", I, true);
const THREE_VARS_THREE_FLOATS: &[Param] = &[SV, SV, SV, FL, FL, FL];
const THREE_VARS_SIX_FLOATS: &[Param] = &[SV, SV, SV, FL, FL, FL, FL, FL, FL];
const FOUR_VARS_THREE_FLOATS_ONE_OPT_INT: &[Param] = &[SV, SV, SV, SV, FL, FL, FL, FLAG];
const FOUR_VARS_FOUR_FLOATS: &[Param] = &[SV, SV, SV, SV, FL, FL, FL, FL];
const FOUR_VARS_EIGHT_FLOATS: &[Param] = &[SV, SV, SV, SV, FL, FL, FL, FL, FL, FL, FL, FL];
const THREE_VARS_SEVEN_FLOATS: &[Param] = &[SV, SV, SV, FL, FL, FL, FL, FL, FL, FL];
const FOUR_VARS_NINE_FLOATS_ONE_OPT_INT: &[Param] =
    &[SV, SV, SV, SV, FL, FL, FL, FL, FL, FL, FL, FL, FL, FLAG];
const THREE_VARS_FOUR_FLOATS_ONE_OPT_INT: &[Param] = &[SV, SV, SV, FL, FL, FL, FL, FLAG];

const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

pub static COMMANDS: &[Command] = &[
    cmd(0x15a2, "V3Length", "v3len", "returns length of given vector3", VECTOR3_FLOATS, v3_length),
    cmd(0x15a3, "V3Normalize", "v3norm", "returns normalized vector3", VECTOR3_STRINGS3_FLOATS, v3_normalize),
    cmd(0x15a4, "V3Crossproduct", "v3xprod", "returns crossproduct of two vectors", VECTOR3_STRINGS6_FLOATS, v3_crossproduct),
    cmd(0x15a5, "QFromEuler", "qfrome", "converts euler angles to quaternion. optional flag indicates if the angles came from an actor", QUAT_EULER_4STRINGS_3FLOATS_1INT, q_from_euler),
    cmd(0x15a6, "QFromAxisAngle", "qfromaa", "converts axis-angle rotation to quaternion", QUAT_AXIS_ANGLE_4STRINGS_4FLOATS, q_from_axis_angle),
    cmd(0x15a7, "QNormalize", "qnorm", "returns normalized quaternion", QUAT_4STRINGS_4FLOATS, q_normalize),
    cmd(0x15a8, "QMultQuatQuat", "qmultq", "multiplies two quaternions", QUAT_4STRINGS_8FLOATS, q_mult_quat_quat),
    cmd(0x15a9, "QMultQuatVector3", "qmultv3", "multiplies vector3 by quaternion", VECTOR_QUAT_VECTOR3_STRINGS_7FLOATS, q_mult_quat_vector3),
    cmd(0x15aa, "QToEuler", "qtoe", "converts quaternion to euler angles. optional flag indicates the output will be used for rotating an actor", EULER_QUAT_3STRINGS_4FLOATS_1INT, q_to_euler),
    cmd(0x15ab, "QInterpolate", "qint", "interpolates between two quaternions; default method is normalized linear interpolation. optional flag indicates spherical linear interpolation", QUAT_4STRINGS_9FLOATS_1INT, q_interpolate),
    cmd(0x1682, "V3NormalizeEx", "V3NormEx", "Returns normalized vector3", THREE_VARS_THREE_FLOATS, v3_normalize_ex),
    cmd(0x1683, "V3CrossproductEx", "V3CrossEx", "Returns crossproduct of two vectors", THREE_VARS_SIX_FLOATS, v3_crossproduct_ex),
    cmd(0x1684, "QFromEulerEx", "QFromEEx", "Converts euler angles to quaternion", FOUR_VARS_THREE_FLOATS_ONE_OPT_INT, q_from_euler_ex),
    cmd(0x1685, "QFromAxisAngleEx", "QFromAAEx", "Converts axis-angle rotation to quaternion", FOUR_VARS_FOUR_FLOATS, q_from_axis_angle_ex),
    cmd(0x1686, "QNormalizeEx", "QNormEx", "Returns normalized quaternion", FOUR_VARS_FOUR_FLOATS, q_normalize_ex),
    cmd(0x1687, "QMultQuatQuatEx", "QMultQEx", "Multiplies two quaternions", FOUR_VARS_EIGHT_FLOATS, q_mult_quat_quat_ex),
    cmd(0x1688, "QMultQuatVector3Ex", "QMultV3Ex", "Multiplies vector3 by quaternion", THREE_VARS_SEVEN_FLOATS, q_mult_quat_vector3_ex),
    cmd(0x1689, "QInterpolateEx", "QIntEx", "Interpolates between two quaternions", FOUR_VARS_NINE_FLOATS_ONE_OPT_INT, q_interpolate_ex),
    cmd(0x168a, "QToEulerEx", "QToEEx", "Converts quaternion to euler angles", THREE_VARS_FOUR_FLOATS_ONE_OPT_INT, q_to_euler_ex),
];

/// `DEGTORAD`, `RADTODEG` (`algMath.cpp`), `float`s.
#[allow(clippy::excessive_precision)]
const DEG_TO_RAD: f32 = 0.017_453_292_52;
#[allow(clippy::excessive_precision)]
const RAD_TO_DEG: f32 = 57.295_779_513_1;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Euler {
    pub elevation: f32,
    pub bank: f32,
    pub heading: f32,
}

impl Vector3 {
    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), common/ITypes.h: Vector3::Magnitude.
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: V3Normalize.
pub fn v3_normalize_math(v: Vector3) -> Vector3 {
    let mut len = v.magnitude();
    len = if len > 0.0 { 1.0 / len } else { 0.0 };
    Vector3 {
        x: v.x * len,
        y: v.y * len,
        z: v.z * len,
    }
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: V3Crossproduct.
pub fn v3_cross(va: Vector3, vb: Vector3) -> Vector3 {
    Vector3 {
        x: vb.y * va.z - vb.z * va.y,
        y: vb.z * va.x - vb.x * va.z,
        z: vb.x * va.y - vb.y * va.x,
    }
}

impl Quat {
    // Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: Quat::normalize.
    pub fn normalize(self) -> Quat {
        let mut len =
            (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        len = if len > 0.0 { 1.0 / len } else { 0.0 };
        self.scale(len)
    }

    // Translated from Algohol (in xNVSE 6.4.9), algTypes.h: Quat::operator*= (a float).
    fn scale(self, s: f32) -> Quat {
        Quat {
            w: self.w * s,
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    fn add(self, q: Quat) -> Quat {
        Quat {
            w: self.w + q.w,
            x: self.x + q.x,
            y: self.y + q.y,
            z: self.z + q.z,
        }
    }

    fn sub(self, q: Quat) -> Quat {
        Quat {
            w: self.w - q.w,
            x: self.x - q.x,
            y: self.y - q.y,
            z: self.z - q.z,
        }
    }

    fn neg(self) -> Quat {
        Quat {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    // Translated from Algohol (in xNVSE 6.4.9), algTypes.h: Quat::operator*= (a quaternion).
    pub fn mul_quat(self, q: Quat) -> Quat {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        Quat {
            w: w * q.w - x * q.x - y * q.y - z * q.z,
            x: w * q.x + x * q.w - y * q.z + z * q.y,
            y: w * q.y + x * q.z + y * q.w - z * q.x,
            z: w * q.z - x * q.y + y * q.x + z * q.w,
        }
    }

    // Translated from Algohol (in xNVSE 6.4.9), algTypes.h: Quat::operator* (a Vector3).
    pub fn rotate(self, v: Vector3) -> Vector3 {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let t2 = w * x;
        let t3 = w * y;
        let t4 = w * z;
        let t5 = -x * x;
        let t6 = x * y;
        let t7 = x * z;
        let t8 = -y * y;
        let t9 = y * z;
        let t10 = -z * z;
        Vector3 {
            x: 2.0 * ((t8 + t10) * v.x + (t6 + t4) * v.y + (t7 - t3) * v.z) + v.x,
            y: 2.0 * ((t6 - t4) * v.x + (t5 + t10) * v.y + (t9 + t2) * v.z) + v.y,
            z: 2.0 * ((t7 + t3) * v.x + (t9 - t2) * v.y + (t5 + t8) * v.z) + v.z,
        }
    }
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: QDotproduct.
fn q_dot(q1: Quat, q2: Quat) -> f32 {
    q1.w * q2.w + q1.x * q2.x + q1.y * q2.y + q1.z * q2.z
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: fromAxisAngle.
pub fn from_axis_angle(axis: Vector3, angle: f32) -> Quat {
    let axis = v3_normalize_math(axis);
    let angle = angle / (2.0 * RAD_TO_DEG);
    let s = angle.sin();
    Quat {
        w: angle.cos(),
        x: axis.x * s,
        y: axis.y * s,
        z: axis.z * s,
    }
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: fromEuler.
pub fn from_euler(e: Euler, flag: i32) -> Quat {
    let c1 = (e.elevation * DEG_TO_RAD / 2.0).cos();
    let c3 = (e.heading * DEG_TO_RAD / 2.0).cos();
    let s1 = (e.elevation * DEG_TO_RAD / 2.0).sin();
    let s3 = (e.heading * DEG_TO_RAD / 2.0).sin();
    if flag == 0 {
        let c2 = (e.bank * DEG_TO_RAD / 2.0).cos();
        let s2 = (e.bank * DEG_TO_RAD / 2.0).sin();
        let c1c2 = c1 * c2;
        let s1s2 = s1 * s2;
        Quat {
            w: c1c2 * c3 + s1s2 * s3,
            x: s1 * c2 * c3 - c1 * s2 * s3,
            y: c1 * s2 * c3 + s1 * c2 * s3,
            z: c1c2 * s3 - s1s2 * c3,
        }
    } else {
        Quat {
            w: c1 * c3,
            x: s1 * c3,
            y: -s1 * s3,
            z: c1 * s3,
        }
    }
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: nlerp.
pub fn nlerp(mut q1: Quat, q2: Quat, t: f32) -> Quat {
    if q_dot(q1, q2) < 0.0 {
        q1 = q1.neg();
    }
    q1.add(q2.sub(q1).scale(t)).normalize()
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: slerp.
pub fn slerp(q1: Quat, q2: Quat, t: f32) -> Quat {
    let mut q1 = q1.normalize();
    let q2 = q2.normalize();
    let cos_half_theta = q_dot(q1, q2);
    if cos_half_theta.abs() >= 1.0 {
        return q1;
    }
    if cos_half_theta < 0.0 {
        q1 = q1.neg();
    }
    let half_theta = cos_half_theta.acos();
    let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();
    if sin_half_theta.abs() < 0.001 {
        return q1.scale(0.5).add(q2.scale(0.5));
    }
    let ratio_a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
    let ratio_b = (t * half_theta).sin() / sin_half_theta;
    q1.scale(ratio_a).add(q2.scale(ratio_b))
}

// Translated from Algohol (in xNVSE 6.4.9), algMath.cpp: fromQuat. `func_flag` 0 is QToEuler's
// (legacy) gimbal-lock handling, 1 QToEulerEx's.
pub fn from_quat(q: Quat, actor_flag: i32, func_flag: i32) -> Euler {
    let sqw = q.w * q.w;
    let sqx = q.x * q.x;
    let sqy = q.y * q.y;
    let sqz = q.z * q.z;
    let unit = sqw + sqx + sqy + sqz;
    if actor_flag == 0 {
        let test = q.x * q.z - q.w * q.y;
        if func_flag == 0 {
            let epsilon = 0.49992 * unit;
            if test > epsilon {
                return Euler {
                    elevation: -(2.0 * q.x * q.y - 2.0 * q.w * q.z).atan2(sqw - sqx + sqy - sqz)
                        * RAD_TO_DEG,
                    bank: -90.0,
                    heading: 0.0,
                };
            }
            if test < -epsilon {
                return Euler {
                    elevation: -(2.0 * q.w * q.z - 2.0 * q.x * q.y).atan2(sqw - sqx + sqy - sqz)
                        * RAD_TO_DEG,
                    bank: 90.0,
                    heading: 0.0,
                };
            }
        } else {
            let epsilon = 0.499_999_9 * unit;
            if test > epsilon {
                return Euler {
                    elevation: 0.0,
                    bank: -90.0,
                    heading: 2.0 * q.z.atan2(q.w) * RAD_TO_DEG,
                };
            }
            if test < -epsilon {
                return Euler {
                    elevation: 0.0,
                    bank: 90.0,
                    heading: 2.0 * q.z.atan2(q.w) * RAD_TO_DEG,
                };
            }
        }
        Euler {
            elevation: -(-2.0 * q.x * q.w - 2.0 * q.y * q.z).atan2(sqw - sqx - sqy + sqz)
                * RAD_TO_DEG,
            bank: -((2.0 * test) / unit).asin() * RAD_TO_DEG,
            heading: -(-2.0 * q.x * q.y - 2.0 * q.w * q.z).atan2(sqw + sqx - sqy - sqz)
                * RAD_TO_DEG,
        }
    } else {
        Euler {
            elevation: -(2.0 * q.y * q.z - 2.0 * q.w * q.x).asin() * RAD_TO_DEG,
            bank: 0.0,
            heading: -(2.0 * q.x * q.y - 2.0 * q.w * q.z).atan2(sqw + sqx - sqy - sqz) * RAD_TO_DEG,
        }
    }
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: setVarByName.
/// The first variable of the script's list whose name equals `name` (`strcmp`) gets `value`
/// in its local. A name no variable has changes nothing.
fn set_var_by_name(e: &mut Engine, script: u32, event_list: u32, name: &[u8], value: f32) {
    let mut node = script + 0x4c;
    while node != 0 {
        let info = e.mem.u32(node);
        if info != 0 && guest_text(e, e.mem.u32(info + 0x18)) == name {
            let local = event_list_variable(e, event_list, e.mem.u32(info));
            // The C++ writes through the local without a check; a variable the list does not
            // hold (no such local) is left alone here.
            if local != 0 {
                e.mem.set_f64(local + 8, f64::from(value));
            }
            return;
        }
        node = e.mem.u32(node + 4);
    }
}

/// The floats from argument `from` on.
fn floats<const K: usize>(read: &Read, from: usize) -> [f32; K] {
    std::array::from_fn(|i| read.float(from + i, 0.0))
}

/// Writes the values by the names in arguments `0..values.len()`.
fn set_named(e: &mut Engine, a: &CommandArgs, read: &Read, values: &[f32]) {
    for (i, v) in values.iter().enumerate() {
        let name = read.text(i);
        set_var_by_name(e, a.script, a.event_list, &name, *v);
    }
}

/// Writes the values into the locals in arguments `0..values.len()`.
fn set_locals(e: &mut Engine, read: &Read, values: &[f32]) {
    for (i, v) in values.iter().enumerate() {
        e.mem.set_f64(read.local(i) + 8, f64::from(*v));
    }
}

fn vec3(v: [f32; 3]) -> Vector3 {
    Vector3 {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

fn quat(v: [f32; 4]) -> Quat {
    Quat {
        w: v[0],
        x: v[1],
        y: v[2],
        z: v[3],
    }
}

/// A read that must succeed (the older commands return on a failed read and write nothing).
fn read_all(e: &mut Engine, w: &[u32]) -> (CommandArgs, Option<Read>) {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let ok = read.ok;
    (a, ok.then_some(read))
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_V3Length_Execute.
/// The result is written only on a successful read (the handler does not zero it).
fn v3_length(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let v = vec3(floats(&read, 0)).magnitude();
        a.set_result(e, f64::from(v));
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_V3Normalize_Execute.
fn v3_normalize(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let v = v3_normalize_math(vec3(floats(&read, 3)));
        set_named(e, &a, &read, &[v.x, v.y, v.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_V3Crossproduct_Execute.
fn v3_crossproduct(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let v = v3_cross(vec3(floats(&read, 3)), vec3(floats(&read, 6)));
        set_named(e, &a, &read, &[v.x, v.y, v.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QFromEuler_Execute.
fn q_from_euler(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let [elevation, bank, heading] = floats(&read, 4);
        let q = from_euler(
            Euler {
                elevation,
                bank,
                heading,
            },
            read.int(7, 0) as i32,
        );
        set_named(e, &a, &read, &[q.w, q.x, q.y, q.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QFromAxisAngle_Execute.
fn q_from_axis_angle(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let q = from_axis_angle(vec3(floats(&read, 4)), read.float(7, 0.0));
        set_named(e, &a, &read, &[q.w, q.x, q.y, q.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QNormalize_Execute.
fn q_normalize(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let q = quat(floats(&read, 4)).normalize();
        set_named(e, &a, &read, &[q.w, q.x, q.y, q.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QMultQuatQuat_Execute.
fn q_mult_quat_quat(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let q = quat(floats(&read, 4)).mul_quat(quat(floats(&read, 8)));
        set_named(e, &a, &read, &[q.w, q.x, q.y, q.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QMultQuatVector3_Execute.
fn q_mult_quat_vector3(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let v = quat(floats(&read, 3)).rotate(vec3(floats(&read, 7)));
        set_named(e, &a, &read, &[v.x, v.y, v.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QInterpolate_Execute.
fn q_interpolate(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let (q1, q2, t) = (
            quat(floats(&read, 4)),
            quat(floats(&read, 8)),
            read.float(12, 0.0),
        );
        let q = if read.int(13, 0) == 0 {
            nlerp(q1, q2, t)
        } else {
            slerp(q1, q2, t)
        };
        set_named(e, &a, &read, &[q.w, q.x, q.y, q.z]);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_QToEuler_Execute.
fn q_to_euler(e: &mut Engine, w: &[u32]) -> Ret {
    let (a, read) = read_all(e, w);
    if let Some(read) = read {
        let out = from_quat(quat(floats(&read, 3)), read.int(7, 0) as i32, 0);
        set_named(e, &a, &read, &[out.elevation, out.bank, out.heading]);
    }
    handled(true)
}

/// The `...Ex` commands: `*result = 0` first (except `QMultQuatVector3Ex` and `QToEulerEx`,
/// which do not touch it), then the locals on a successful read.
fn ex(e: &mut Engine, w: &[u32], zero: bool, f: fn(&Read) -> Vec<f32>) -> Ret {
    let a = CommandArgs::new(w);
    if zero {
        a.set_result(e, 0.0);
    }
    let read = Read::of(e, &a);
    if read.ok {
        let values = f(&read);
        set_locals(e, &read, &values);
    }
    handled(true)
}

// Translated from Algohol (in xNVSE 6.4.9), commands_Algohol.cpp: Cmd_V3NormalizeEx_Execute,
// Cmd_V3CrossproductEx_Execute, Cmd_QFromEulerEx_Execute, Cmd_QFromAxisAngleEx_Execute,
// Cmd_QNormalizeEx_Execute, Cmd_QMultQuatQuatEx_Execute, Cmd_QMultQuatVector3Ex_Execute,
// Cmd_QInterpolateEx_Execute, Cmd_QToEulerEx_Execute.
fn v3_normalize_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let v = v3_normalize_math(vec3(floats(r, 3)));
        vec![v.x, v.y, v.z]
    })
}

fn v3_crossproduct_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let v = v3_cross(vec3(floats(r, 3)), vec3(floats(r, 6)));
        vec![v.x, v.y, v.z]
    })
}

fn q_from_euler_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let [elevation, bank, heading] = floats(r, 4);
        let q = from_euler(
            Euler {
                elevation,
                bank,
                heading,
            },
            r.int(7, 0) as i32,
        );
        vec![q.w, q.x, q.y, q.z]
    })
}

fn q_from_axis_angle_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let q = from_axis_angle(vec3(floats(r, 4)), r.float(7, 0.0));
        vec![q.w, q.x, q.y, q.z]
    })
}

fn q_normalize_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let q = quat(floats(r, 4)).normalize();
        vec![q.w, q.x, q.y, q.z]
    })
}

fn q_mult_quat_quat_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let q = quat(floats(r, 4)).mul_quat(quat(floats(r, 8)));
        vec![q.w, q.x, q.y, q.z]
    })
}

fn q_mult_quat_vector3_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, false, |r| {
        let v = quat(floats(r, 3)).rotate(vec3(floats(r, 7)));
        vec![v.x, v.y, v.z]
    })
}

fn q_interpolate_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, true, |r| {
        let (q1, q2, t) = (quat(floats(r, 4)), quat(floats(r, 8)), r.float(12, 0.0));
        let q = if r.int(13, 0) == 0 {
            nlerp(q1, q2, t)
        } else {
            slerp(q1, q2, t)
        };
        vec![q.w, q.x, q.y, q.z]
    })
}

fn q_to_euler_ex(e: &mut Engine, w: &[u32]) -> Ret {
    ex(e, w, false, |r| {
        let out = from_quat(quat(floats(r, 3)), r.int(7, 0) as i32, 1);
        vec![out.elevation, out.bank, out.heading]
    })
}

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::super::test_support::*;
    use engine_core::prelude::*;

    /// A case: label, opcode, names written, numbers, flag, the real game's four values.
    type Case = (&'static str, u32, usize, Vec<f64>, Option<i32>, [f64; 4]);

    /// Runs `opcode` with string names (locals 1.. named `va` .. `vd`, as the real-game scripts) and the
    /// numbers, and answers the four locals scaled by 2^40, as the real-game script printed them.
    fn named(
        e: &mut Engine,
        opcode: u32,
        outs: usize,
        numbers: &[f64],
        flag: Option<i32>,
    ) -> [f64; 4] {
        let names = ["va", "vb", "vc", "vd"];
        let locals: Vec<u32> = (0..4)
            .map(|i| add_local(e, i as u16 + 1, names[i], 0.0))
            .collect();
        let mut args: Vec<Vec<u8>> = names[..outs].iter().map(|n| s(n)).collect();
        args.extend(numbers.iter().map(|v| z(*v)));
        if let Some(f) = flag {
            args.push(n(f));
        }
        let (ok, _) = call(e, opcode, 0, &args);
        assert_eq!(ok, 1);
        std::array::from_fn(|i| local_value(e, locals[i]) * 1_099_511_627_776.0)
    }

    /// The real game's answers (`real-results/batch1.txt`, the `.x2^40` lines of the `d_` scripts).
    #[test]
    fn the_results_match_the_real_game_bit_for_bit() {
        let mut e = engine_with_table();
        let (ok, len) = call(&mut e, 0x15a2, 0, &[z(3.0), z(4.0), z(12.0)]);
        assert_eq!((ok, len * 1_099_511_627_776.0), (1, 14293651161088.0));
        let (_, len) = call(&mut e, 0x15a2, 0, &[z(1.0), z(1.0), z(1.0)]);
        assert_eq!(len * 1_099_511_627_776.0, 1904409968640.0);
        let cases: &[Case] = &[
            (
                "3v3norm",
                0x15a3,
                3,
                vec![3.0, 4.0, 12.0],
                None,
                [253733470208.0, 338311282688.0, 1014933880832.0, 0.0],
            ),
            (
                "4qfe",
                0x15a5,
                4,
                vec![30.0, 45.0, 60.0],
                None,
                [
                    904197832704.0,
                    24475123712.0,
                    483433021440.0,
                    396289703936.0,
                ],
            ),
            (
                "4qfe_actor",
                0x15a5,
                4,
                vec![30.0, 45.0, 60.0],
                Some(1),
                [
                    919759355904.0,
                    246448783360.0,
                    -142287273984.0,
                    531023331328.0,
                ],
            ),
            (
                "5qte",
                0x15aa,
                3,
                vec![0.8, 0.2, 0.3, 0.4],
                None,
                [43859065176064.0, 22128780902400.0, 66519656562688.0, 0.0],
            ),
            (
                "5qte_actor",
                0x15aa,
                3,
                vec![0.8, 0.2, 0.3, 0.4],
                Some(1),
                [5045182005248.0, 0.0, 55428440391680.0, 0.0],
            ),
            (
                "6qint",
                0x15ab,
                4,
                vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.3],
                None,
                [1010610667520.0, 0.0, 0.0, 433118871552.0],
            ),
            (
                "6qint_slerp",
                0x15ab,
                4,
                vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.3],
                Some(1),
                [979672039424.0, 0.0, 0.0, 499167887360.0],
            ),
            (
                "7qmq",
                0x15a8,
                4,
                vec![0.5, 0.5, 0.5, 0.5, 0.7071, 0.0, 0.7071, 0.0],
                None,
                [0.0, 777464643584.0, 777464643584.0, 0.0],
            ),
            (
                "8cross",
                0x15a4,
                3,
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                None,
                [3298534883328.0, -6597069766656.0, 3298534883328.0, 0.0],
            ),
            (
                "9qaa",
                0x15a6,
                4,
                vec![0.0, 0.0, 1.0, 90.0],
                None,
                [777472180224.0, 0.0, 0.0, 777472114688.0],
            ),
            (
                "9qmv",
                0x15a9,
                3,
                vec![0.707_106_8, 0.0, 0.0, 0.707_106_8, 1.0, 2.0, 3.0],
                None,
                [2199023255552.0, -1099512152064.0, 3298534883328.0, 0.0],
            ),
            (
                "9qnorm",
                0x15a7,
                4,
                vec![1.0, 2.0, 3.0, 4.0],
                None,
                [
                    200742436864.0,
                    401484873728.0,
                    602227343360.0,
                    802969747456.0,
                ],
            ),
        ];
        let mut wrong = Vec::new();
        for (label, opcode, outs, numbers, flag, real) in cases {
            let mut e = engine_with_table();
            let got = named(&mut e, *opcode, *outs, numbers, *flag);
            if got != *real {
                wrong.push(format!("{label}: port {got:?}, real {real:?}"));
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
    }

    /// The `...Ex` forms write the locals given as variables; a name the script does not have
    /// is passed over by the older forms.
    #[test]
    fn ex_forms_write_their_locals_and_unknown_names_change_nothing() {
        let mut e = engine_with_table();
        let x = add_local(&mut e, 1, "x", 9.0);
        let y = add_local(&mut e, 2, "y", 9.0);
        let zz = add_local(&mut e, 3, "z", 9.0);
        let (ok, _) = call(
            &mut e,
            0x1683,
            0,
            &[
                local(1),
                local(2),
                local(3),
                z(1.0),
                z(2.0),
                z(3.0),
                z(4.0),
                z(5.0),
                z(6.0),
            ],
        );
        assert_eq!(ok, 1);
        assert_eq!(
            [local_value(&e, x), local_value(&e, y), local_value(&e, zz)],
            [3.0, -6.0, 3.0]
        );
        // Names compare with case: "X" is not "x".
        call(
            &mut e,
            0x15a3,
            0,
            &[s("X"), s("y"), s("q"), z(0.0), z(2.0), z(0.0)],
        );
        assert_eq!(
            [local_value(&e, x), local_value(&e, y), local_value(&e, zz)],
            [3.0, 1.0, 3.0]
        );
    }
}
