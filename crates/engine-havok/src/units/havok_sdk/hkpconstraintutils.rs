//! `Havok SDK/hkpconstraintutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Four SSE lanes as bits.
type Lanes = [u32; 4];

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` / `removeReference` (`00c90510`, `00c905b0`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpConstraintInstance` constructor (`00cf3ea0`, four stack words).
const CONSTRAINT_INSTANCE_CONSTRUCT: u32 = 0x00cf_3ea0;
/// `hkMatrix3::transpose` (`00cd4b60`, `this`) and `hkMatrix3::changeBasis` (`00cd5880`,
/// `this`, one pointer).
const MATRIX_TRANSPOSE: u32 = 0x00cd_4b60;
const MATRIX_CHANGE_BASIS: u32 = 0x00cd_5880;
/// `hkVector4::setTransformedPos` (`00c7f640`; `this` = result, then the transform and the
/// vector).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The rigid body's `setPosition` (`00ca8800`, `this`, one pointer), `setLinearVelocity`
/// (`005616d0`) and `setAngularVelocity` (`00561800`) (`this`, one pointer each).
const SET_POSITION: u32 = 0x00ca_8800;
const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
/// The hinge limits and ragdoll limits constructors (`00d52fc0`, `00d4f2a0`; `this`) and the
/// ragdoll limits' flag setter (`00d4f270`, `this`, one byte).
const HINGE_LIMITS_CONSTRUCT: u32 = 0x00d5_2fc0;
const RAGDOLL_LIMITS_CONSTRUCT: u32 = 0x00d4_f2a0;
const RAGDOLL_LIMITS_SET_FLAG: u32 = 0x00d4_f270;
/// The ragdoll constraint data's `getTwistMotor`, `getConeMotor` and `getPlaneMotor`
/// (`00cdea20`, `00cdea80`, `00cdeaa0`; `this`).
const GET_TWIST_MOTOR: u32 = 0x00cd_ea20;
const GET_CONE_MOTOR: u32 = 0x00cd_ea80;
const GET_PLANE_MOTOR: u32 = 0x00cd_eaa0;
/// The report builder: constructor (`00cb1600`, `this`, buffer, size), text append (`00cc01c0`,
/// `this`, text) and destructor (`00cc00e0`, `this`); the global at `01268360` holds the sink
/// object whose slot `0xc` takes `(1, id, text, file, line)`; `010eca10` is the file name
/// `.\Constraint\Bilateral\hkpConstraintUtils.cpp`.
const REPORT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const REPORT_BUILDER_TEXT: u32 = 0x00cc_01c0;
const REPORT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const REPORT_SINK: u32 = 0x0126_8360;
const REPORT_FILE: u32 = 0x010e_ca10;
/// `FLT_EPSILON` as the float at `0104da4c`, the float `1e-6` at `010959f4` and the vector of
/// ones at `010ec7f0`.
const FLOAT_EPSILON: u32 = 0x0104_da4c;
const PIVOT_TOLERANCE: u32 = 0x0109_59f4;
const VECTOR_ONES: u32 = 0x010e_c7f0;

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn lanes_op(dest: Lanes, source: Lanes, op: fn(f32, f32) -> f32) -> Lanes {
    [0, 1, 2, 3].map(|lane| sse(dest[lane], source[lane], op))
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// The lane sum `x2 + (x1 + x0)` the code builds with two `ADDSS` over shuffled copies.
fn lane_sum(lanes: Lanes) -> u32 {
    let low = sse(lanes[1], lanes[0], add);
    sse(lanes[2], low, add)
}

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.u32(addr + 4 * lane))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

/// Copies the four words at `from` to `to`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let lanes = load_lanes(e, from);
    store_lanes(e, to, lanes);
}

/// Copies `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// A new block from the memory router's allocator (slot 4) of `size` bytes with its size
/// word (`u16 +4`) set to `size`.
fn allocate_sized(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// The report the functions of this unit write: a builder over a 0x200-byte buffer holding
/// `message`, sent to the sink at `01268360` (slot `0xc`) with `(1, id, text, file, line)`,
/// then destroyed.
fn report(e: &mut Engine, message: u32, id: u32, line: u32) {
    e.with_stack(0x20c, |e, block| {
        let builder = block.addr();
        let buffer = builder + 0xc;
        e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
        e.call(REPORT_BUILDER_TEXT, &args![builder, message]);
        let sink = e.global::<u32>(REPORT_SINK);
        e.vcall(sink, 0xc, &args![1u32, id, buffer, REPORT_FILE, line]);
        e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
    });
}

/// The inertia part of `setHingePivotToOptimalPosition` for one body: its motion object
/// (`body + 0xe0`, virtual slot `0x24`) writes a 3x3 matrix to `matrices` (0x30 bytes), the
/// hinge's basis (three vectors at `source_basis`) is copied after it, transposed
/// (`00cd4b60`) and used to change the basis of the matrix (`00cd5880`). Returns the float
/// `m22 + m21 + (m12 + m11)` of the changed matrix (x87 sums, each stored as a float),
/// divided by the body's `+0x1ac` when that is above `FLT_EPSILON`.
fn body_inertia_sum(e: &mut Engine, body: u32, source_basis: u32, matrices: u32) -> u32 {
    let inertia = matrices;
    let basis = matrices + 0x30;
    e.vcall(body + 0xe0, 0x24, &args![inertia]);
    for row in 0..3 {
        copy_vector(e, source_basis + 0x10 * row, basis + 0x10 * row);
    }
    e.call(MATRIX_TRANSPOSE, &args![basis]);
    e.call(MATRIX_CHANGE_BASIS, &args![inertia, basis]);
    let element = |e: &Engine, offset: u32| x87_load32(e.mem.u32(inertia + offset));
    let lower = x87_store32(x87_add(element(e, 0x28), element(e, 0x24)));
    let upper = x87_store32(x87_add(element(e, 0x18), element(e, 0x14)));
    let mut sum = x87_store32(x87_add(x87_load32(lower), x87_load32(upper)));
    let epsilon = x87_load32(e.mem.u32(FLOAT_EPSILON));
    let mass_factor = e.mem.u32(body + 0x1ac);
    if epsilon < f64::from(f32::from_bits(mass_factor)) {
        let divisor = e.mem.u32(body + 0x1ac);
        sum = x87_store32(x87_div(x87_load32(sum), x87_load32(divisor)));
    }
    sum
}

// Translated from 00e5a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintUtils::setHingePivotToOptimalPosition` (map name), cdecl on the constraint
/// instance. Only for constraint data of type 1 or 2 (virtual slot `+0x20`; hinge and limited
/// hinge). With the data's two bases (vectors at `+0x20` and `+0x60`), pivots (`+0x50`, `+0x90`)
/// and each body's position (`+0x170`) and inertia the two pivots are moved along the hinge
/// axes so that their distance along the axes is shared in proportion to the bodies' inertia
/// sums (`body_inertia_sum`): `d_a`, `d_b` are the axis components of the pivots' offsets from
/// the bodies' positions, `diff = d_a - d_b`, `inverse = 1 / (sum_b + sum_a)`, the pivots move by
/// `sum_b * diff * inverse - d_a` along axis A and `-diff * sum_a * inverse - d_b` along axis B.
/// The arithmetic is x87 (the scalar parts) and SSE (the vector parts), kept in the code's
/// order.
pub fn set_hinge_pivot_to_optimal_position(e: &mut Engine, constraint: Ptr) {
    let data = e.mem.u32(constraint.addr() + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    if kind != 1 && kind != 2 {
        return;
    }
    let body_a = e.mem.u32(constraint.addr() + 0x14);
    let body_b = e.mem.u32(constraint.addr() + 0x18);
    let (sum_a, sum_b) = e.with_stack(0xc0, |e, frame| {
        let sum_a = body_inertia_sum(e, body_a, data + 0x20, frame.addr());
        let sum_b = body_inertia_sum(e, body_b, data + 0x60, frame.addr() + 0x60);
        (sum_a, sum_b)
    });
    let position_a = load_lanes(e, body_a + 0x170);
    let axis_a = load_lanes(e, data + 0x20);
    let pivot_a = load_lanes(e, data + 0x50);
    let offset_a = lanes_op(pivot_a, position_a, sub);
    let distance_a = lane_sum(lanes_op(axis_a, offset_a, mul));
    let position_b = load_lanes(e, body_b + 0x170);
    let pivot_b = load_lanes(e, data + 0x90);
    let axis_b = load_lanes(e, data + 0x60);
    let offset_b = lanes_op(pivot_b, position_b, sub);
    let distance_b = lane_sum(lanes_op(axis_b, offset_b, mul));

    let sum_a = x87_load32(sum_a);
    let sum_b = x87_load32(sum_b);
    let inverse = x87_load32(x87_store32(x87_div(1.0, x87_add(sum_b, sum_a))));
    let distance_a = x87_load32(distance_a);
    let distance_b = x87_load32(distance_b);
    let difference = x87_load32(x87_store32(x87_sub(distance_a, distance_b)));
    let shared = x87_store32(x87_mul(x87_mul(sum_b, difference), inverse));
    let move_a = x87_store32(x87_sub(x87_load32(shared), distance_a));
    let shared = x87_store32(x87_mul(inverse, x87_mul(sum_a, -difference)));
    let move_b = x87_store32(x87_sub(x87_load32(shared), distance_b));

    let moved = lanes_op([move_a; 4], axis_a, mul);
    let moved = lanes_op(moved, offset_a, add);
    let moved = lanes_op(moved, position_a, add);
    store_lanes(e, data + 0x50, moved);
    let moved = lanes_op([move_b; 4], axis_b, mul);
    let moved = lanes_op(moved, offset_b, add);
    let position_b = load_lanes(e, body_b + 0x170);
    let moved = lanes_op(moved, position_b, add);
    store_lanes(e, data + 0x90, moved);
}

// Translated from 00e5aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintUtils::convertToLimits` (map name), cdecl on the constraint instance. By the
/// type of its data (virtual slot `+0x20`): 2 (limited hinge) builds a 0xa0-byte hinge limits
/// data (`00d52fc0`) and 7 (ragdoll) a 0xc0-byte ragdoll limits data (`00d4f2a0`, with its
/// flag from the byte at `+0x116` through `00d4f270`), copying the atoms' vectors and words
/// (the ranges are in the code); a 0x38-byte instance for the same two bodies and priority is
/// then built around it (`00cf3ea0`), the new data's reference dropped and the instance
/// returned. Types 0x12 and 0x13 add a reference to the instance and return it; any other
/// type returns null.
pub fn convert_to_limits(e: &mut Engine, constraint: Ptr) -> Ptr {
    let at = constraint.addr();
    let data = e.mem.u32(at + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    let limits = match kind {
        7 => {
            let block = allocate_sized(e, 0xc0);
            let limits = e.call(RAGDOLL_LIMITS_CONSTRUCT, &args![block]).u32();
            copy_atom_vectors(e, data, limits);
            copy_words(e, data + 0xfc, limits + 0x80, 5);
            copy_words(e, data + 0x124, limits + 0xa8, 5);
            copy_words(e, data + 0x110, limits + 0x94, 5);
            let flag = u32::from(e.mem.u8(data + 0x116) != 0);
            e.call(RAGDOLL_LIMITS_SET_FLAG, &args![limits, flag]);
            limits
        }
        2 => {
            let block = allocate_sized(e, 0xa0);
            let limits = e.call(HINGE_LIMITS_CONSTRUCT, &args![block]).u32();
            copy_atom_vectors(e, data, limits);
            copy_words(e, data + 0xc0, limits + 0x80, 5);
            limits
        }
        0x12 | 0x13 => {
            e.call(ADD_REFERENCE, &args![constraint]);
            return constraint;
        }
        _ => return Ptr::NULL,
    };
    let block = allocate_sized(e, 0x38);
    let entity_a = e.mem.u32(at + 0x14);
    let entity_b = e.mem.u32(at + 0x18);
    let priority = u32::from(e.mem.u8(at + 0x1c));
    let created = e
        .call(
            CONSTRAINT_INSTANCE_CONSTRUCT,
            &args![block, entity_a, entity_b, limits, priority],
        )
        .u32();
    e.call(REMOVE_REFERENCE, &args![limits]);
    Ptr::new(created)
}

/// The six vectors both limits data take from the source data: `+0x20`, `+0x30`, `+0x40` to the
/// same offsets and `+0x60`, `+0x70`, `+0x80` to `+0x50`, `+0x60`, `+0x70`.
fn copy_atom_vectors(e: &mut Engine, data: u32, limits: u32) {
    for (from, to) in [
        (0x20, 0x20),
        (0x30, 0x30),
        (0x40, 0x40),
        (0x60, 0x50),
        (0x70, 0x60),
        (0x80, 0x70),
    ] {
        copy_vector(e, data + from, limits + to);
    }
}

// Translated from 00e5ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintUtils::getConstraintPivots` (map name), cdecl `(constraint, out_a, out_b)`:
/// writes the two pivot vectors of the constraint's data (type, virtual slot `+0x20`) to
/// `out_a` and `out_b`: for type 0 the vectors at `+0x20` and `+0x30`, for the types 1, 2 and 7
/// those at `+0x50` and `+0x90`; returns 0. Any other type is reported ("Unsupported type of
/// constraint in prepareSystemForRagdoll()", id `0xabbabf3b`, line `0xe2`) and answers 1.
pub fn get_constraint_pivots(e: &mut Engine, constraint: Ptr, out_a: Ptr, out_b: Ptr) -> u32 {
    let data = e.mem.u32(constraint.addr() + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    let (from_a, from_b) = match kind {
        0 => (0x20, 0x30),
        1 | 2 | 7 => (0x50, 0x90),
        _ => {
            report(e, 0x010e_ca40, 0xabba_bf3b, 0xe2);
            return 1;
        }
    };
    copy_vector(e, data + from_a, out_a.addr());
    copy_vector(e, data + from_b, out_b.addr());
    0
}

// Translated from 00e5ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintUtils::getConstraintMotors` (map name), cdecl `(data, out_twist, out_cone,
/// out_plane)`: for data of type 2 (limited hinge) the motor at `+0xb0` goes to `out_twist` and
/// the other two outputs are cleared; for type 7 (ragdoll) the three motors come from
/// `00cdea20`, `00cdea80` and `00cdeaa0`; both answer 0. Other types clear the three outputs
/// and are reported ("This type of constraint does not have motors", id `0xabbae233`, line
/// `0x104`), answering 1.
pub fn get_constraint_motors(
    e: &mut Engine,
    data: Ptr,
    out_twist: Ptr,
    out_cone: Ptr,
    out_plane: Ptr,
) -> u32 {
    let kind = e.vcall(data.addr(), 0x20, &args![]).u32();
    if kind == 2 {
        let motor = e.mem.u32(data.addr() + 0xb0);
        e.mem.set_u32(out_twist.addr(), motor);
        e.mem.set_u32(out_cone.addr(), 0);
        e.mem.set_u32(out_plane.addr(), 0);
        return 0;
    }
    if kind != 7 {
        e.mem.set_u32(out_plane.addr(), 0);
        e.mem.set_u32(out_cone.addr(), 0);
        e.mem.set_u32(out_twist.addr(), 0);
        report(e, 0x010e_ca7c, 0xabba_e233, 0x104);
        return 1;
    }
    let twist = e.call(GET_TWIST_MOTOR, &args![data]).u32();
    e.mem.set_u32(out_twist.addr(), twist);
    let cone = e.call(GET_CONE_MOTOR, &args![data]).u32();
    e.mem.set_u32(out_cone.addr(), cone);
    let plane = e.call(GET_PLANE_MOTOR, &args![data]).u32();
    e.mem.set_u32(out_plane.addr(), plane);
    0
}

// Translated from 00e5af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintUtils::checkAndFixConstraint` (map name), cdecl `(out, constraint,
/// max_distance, blend)` with the two floats as bits: gets the constraint's pivots
/// (`get_constraint_pivots`); on failure `*out` is 0. A pivot A whose squared length is above
/// `1e-6` (the float at `010959f4`) is reported ("Pivot of child rigid body (A) is expected to
/// be aligned with the constraint at setup time.", id `0xabba5dff`, line `0x11d`). Both pivots
/// are transformed by their bodies' transforms (`body + 0xf0`, `00c7f640`); when the squared
/// distance of the results is above `max_distance^2` (the product taken on the x87 stack) the
/// first body is moved to `(1 - blend) * world_b + blend * [body_b + 0x120]` (`00ca8800`), takes
/// the second body's linear and angular velocity (`+0x1b0`, `+0x1c0`; `005616d0`, `00561800`)
/// and `*out` is 1; otherwise `*out` is 0.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn check_and_fix_constraint(
    e: &mut Engine,
    out: Ptr,
    constraint: Ptr,
    max_distance: u32,
    blend: u32,
) -> Ptr {
    e.with_stack(0x60, |e, frame| {
        let pivot_a = frame.addr() + 0x20;
        let pivot_b = frame.addr() + 0x30;
        let world_b = frame.addr() + 0x40;
        let world_a = frame.addr() + 0x50;
        let new_position = frame.addr() + 0x10;
        if get_constraint_pivots(e, constraint, Ptr::new(pivot_a), Ptr::new(pivot_b)) != 0 {
            e.mem.set_u8(out.addr(), 0);
            return;
        }
        let tolerance = x87_load32(e.mem.u32(PIVOT_TOLERANCE));
        let squares = lanes_op(load_lanes(e, pivot_a), load_lanes(e, pivot_a), mul);
        let length = f32::from_bits(lane_sum(squares));
        if tolerance < f64::from(length) {
            report(e, 0x010e_cab0, 0xabba_5dff, 0x11d);
        }
        let body_b = e.mem.u32(constraint.addr() + 0x18);
        let body_a = e.mem.u32(constraint.addr() + 0x14);
        e.call(SET_TRANSFORMED_POS, &args![world_b, body_b + 0xf0, pivot_b]);
        e.call(SET_TRANSFORMED_POS, &args![world_a, body_a + 0xf0, pivot_a]);
        let limit = x87_load32(max_distance);
        let apart = lanes_op(load_lanes(e, world_b), load_lanes(e, world_a), sub);
        let squares = lanes_op(apart, apart, mul);
        let distance = f64::from(f32::from_bits(lane_sum(squares)));
        if x87_mul(limit, limit) < distance {
            let blend = [blend; 4];
            let ones = load_lanes(e, VECTOR_ONES);
            let weighted = lanes_op(lanes_op(ones, blend, sub), load_lanes(e, world_b), mul);
            let other = lanes_op(load_lanes(e, body_b + 0x120), blend, mul);
            store_lanes(e, new_position, lanes_op(weighted, other, add));
            e.call(SET_POSITION, &args![body_a, new_position]);
            e.call(SET_LINEAR_VELOCITY, &args![body_a, body_b + 0x1b0]);
            e.call(SET_ANGULAR_VELOCITY, &args![body_a, body_b + 0x1c0]);
            e.mem.set_u8(out.addr(), 1);
        } else {
            e.mem.set_u8(out.addr(), 0);
        }
    });
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5_a800, set_hinge_pivot_to_optimal_position(Ptr)),
        entry!(0x00e5_aac0, convert_to_limits(Ptr) -> Ptr),
        entry!(0x00e5_ad40, get_constraint_pivots(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e5_ae60, get_constraint_motors(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e5_af80, check_and_fix_constraint(Ptr, Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range mapped and the constants this unit reads.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(FLOAT_EPSILON, 1.192_092_9e-7);
        e.mem.set_f32(PIVOT_TOLERANCE, 1e-6);
        for lane in 0..4 {
            e.mem.set_f32(VECTOR_ONES + 4 * lane, 1.0);
        }
        e
    }

    /// Doubles at `addresses` that log their arguments and return 0.
    fn log_calls(e: &mut Engine, addresses: &[u32]) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    /// A data object of type `kind` (virtual slot `0x20`) of `size` bytes, word `n` = `0x1000 + n`.
    fn data_object(e: &mut Engine, kind: u32, slot: u32, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        for word in 0..size / 4 {
            e.mem.set_u32(block + 4 * word, 0x1000 + word);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, slot);
        e.mem.set_u32(block, vtable);
        e.register_double(slot, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        block
    }

    /// A constraint instance over `data` and two bodies of `0x1d0` bytes.
    fn constraint(e: &mut Engine, data: u32) -> (u32, u32, u32) {
        let body_a = e.mem.alloc(0x1d0);
        let body_b = e.mem.alloc(0x1d0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x14, body_a);
        e.mem.set_u32(this + 0x18, body_b);
        e.mem.set_u8(this + 0x1c, 5);
        (this, body_a, body_b)
    }

    /// A sink at `01268360` and doubles for the report builder; the sink's calls are logged.
    fn install_report(e: &mut Engine) -> Calls {
        let log = log_calls(
            e,
            &[
                REPORT_BUILDER_CONSTRUCT,
                REPORT_BUILDER_TEXT,
                REPORT_BUILDER_DESTRUCT,
            ],
        );
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_00c0);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        let seen = Rc::clone(&log);
        e.register_double(0x0900_00c0, move |_, a| {
            seen.borrow_mut().push((0x0900_00c0, a[1..].to_vec()));
            Ret::default()
        });
        log
    }

    /// A memory router whose allocator hands out `blocks` in turn from slot 4, logging each
    /// request's size into `log`.
    fn install_allocator(e: &mut Engine, blocks: Vec<u32>, log: &Calls) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let queue = Rc::new(RefCell::new(VecDeque::from(blocks)));
        let sink = Rc::clone(log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push((4, a[1..].to_vec()));
            Ret {
                eax: queue.borrow_mut().pop_front().unwrap(),
                ..Ret::default()
            }
        });
    }

    fn set_vector(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    fn vector(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|lane| e.mem.f32(at + 4 * lane))
    }

    /// Sets the 3x3 inertia matrix a body's motion object returns: `m11`, `m12`, `m21`, `m22`.
    fn install_inertia(e: &mut Engine, body: u32, slot: u32, elements: [f32; 4]) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x24, slot);
        e.mem.set_u32(body + 0xe0, vtable);
        e.register_double(slot, move |e, a| {
            assert_eq!(a[0], body + 0xe0);
            let matrix = a[1];
            for (offset, value) in [0x14, 0x18, 0x24, 0x28].into_iter().zip(elements) {
                e.mem.set_f32(matrix + offset, value);
            }
            Ret::default()
        });
    }

    #[test]
    fn set_hinge_pivot_moves_the_pivots_along_the_axes() {
        let mut e = rig();
        let data = data_object(&mut e, 1, 0x0900_0100, 0xa0);
        let (this, body_a, body_b) = constraint(&mut e, data);
        let log = log_calls(&mut e, &[MATRIX_TRANSPOSE, MATRIX_CHANGE_BASIS]);
        install_inertia(&mut e, body_a, 0x0900_0101, [1.0, 1.0, 1.0, 1.0]);
        install_inertia(&mut e, body_b, 0x0900_0102, [1.0, 1.0, 1.0, 1.0]);
        set_vector(&mut e, body_a + 0x170, [1.0, 1.0, 1.0, 0.0]);
        set_vector(&mut e, body_b + 0x170, [0.0; 4]);
        set_vector(&mut e, data + 0x20, [1.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, data + 0x50, [3.0, 5.0, 7.0, 0.0]);
        set_vector(&mut e, data + 0x90, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00e5_a800, &args![this]);
        // sums 4 and 4, inverse 1/8, d_a 2, d_b 1, difference 1:
        // move A = 4 * 1 / 8 - 2 = -1.5, move B = -(4 * 1 / 8) - 1 = -1.5.
        assert_eq!(vector(&e, data + 0x50), [1.5, 5.0, 7.0, 0.0]);
        assert_eq!(vector(&e, data + 0x90), [-0.5, 2.0, 3.0, 0.0]);
        // The basis copies were transposed and applied for both bodies.
        let addresses: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            addresses,
            [
                MATRIX_TRANSPOSE,
                MATRIX_CHANGE_BASIS,
                MATRIX_TRANSPOSE,
                MATRIX_CHANGE_BASIS
            ]
        );
    }

    #[test]
    fn set_hinge_pivot_divides_a_sum_by_the_mass_factor_above_epsilon() {
        let mut e = rig();
        let data = data_object(&mut e, 2, 0x0900_0100, 0xa0);
        let (this, body_a, body_b) = constraint(&mut e, data);
        log_calls(&mut e, &[MATRIX_TRANSPOSE, MATRIX_CHANGE_BASIS]);
        install_inertia(&mut e, body_a, 0x0900_0101, [1.0, 1.0, 1.0, 1.0]);
        install_inertia(&mut e, body_b, 0x0900_0102, [0.0, 0.0, 3.0, 3.0]);
        // Body A's factor 2 halves its sum (4 -> 2); body B's factor is below epsilon.
        e.mem.set_f32(body_a + 0x1ac, 2.0);
        e.mem.set_f32(body_b + 0x1ac, 1e-9);
        set_vector(&mut e, body_a + 0x170, [1.0, 1.0, 1.0, 0.0]);
        set_vector(&mut e, body_b + 0x170, [0.0; 4]);
        set_vector(&mut e, data + 0x20, [1.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, data + 0x60, [1.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, data + 0x50, [3.0, 5.0, 7.0, 0.0]);
        set_vector(&mut e, data + 0x90, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00e5_a800, &args![this]);
        // sums 2 and 6, inverse 1/8: move A = 6 / 8 - 2 = -1.25, move B = -(2 / 8) - 1 = -1.25.
        assert_eq!(vector(&e, data + 0x50), [1.75, 5.0, 7.0, 0.0]);
        assert_eq!(vector(&e, data + 0x90), [-0.25, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn set_hinge_pivot_ignores_other_data_types() {
        let mut e = rig();
        let data = data_object(&mut e, 7, 0x0900_0100, 0xa0);
        let (this, _, _) = constraint(&mut e, data);
        let log = log_calls(&mut e, &[MATRIX_TRANSPOSE]);
        e.mem.set_f32(data + 0x50, 9.0);
        e.call(0x00e5_a800, &args![this]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.f32(data + 0x50), 9.0);
    }

    fn limits_rig(kind: u32) -> (Engine, u32, u32, Calls, [u32; 2]) {
        let mut e = rig();
        let data = data_object(&mut e, kind, 0x0900_0100, 0x140);
        let (this, _, _) = constraint(&mut e, data);
        let blocks = [e.mem.alloc(0xc0), e.mem.alloc(0x38)];
        let log = log_calls(
            &mut e,
            &[RAGDOLL_LIMITS_SET_FLAG, REMOVE_REFERENCE, ADD_REFERENCE],
        );
        install_allocator(&mut e, blocks.to_vec(), &log);
        // The constructors answer the block they were given.
        for address in [
            HINGE_LIMITS_CONSTRUCT,
            RAGDOLL_LIMITS_CONSTRUCT,
            CONSTRAINT_INSTANCE_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        (e, this, data, log, blocks)
    }

    #[test]
    fn convert_to_limits_builds_hinge_limits_for_type_two() {
        let (mut e, this, data, log, blocks) = limits_rig(2);
        let got = e.call(0x00e5_aac0, &args![this]).u32();
        assert_eq!(got, blocks[1]);
        let [limits, instance] = blocks;
        // Vectors: +0x20/30/40 as they are, +0x60/70/80 moved to +0x50/60/70; five words from +0xc0.
        let word = |offset: u32| 0x1000 + offset / 4;
        for lane in 0..4 {
            assert_eq!(e.mem.u32(limits + 0x20 + 4 * lane), word(0x20) + lane);
            assert_eq!(e.mem.u32(limits + 0x40 + 4 * lane), word(0x40) + lane);
            assert_eq!(e.mem.u32(limits + 0x50 + 4 * lane), word(0x60) + lane);
            assert_eq!(e.mem.u32(limits + 0x70 + 4 * lane), word(0x80) + lane);
        }
        for index in 0..5 {
            assert_eq!(e.mem.u32(limits + 0x80 + 4 * index), word(0xc0) + index);
        }
        assert_eq!(e.mem.u16(limits + 4), 0xa0);
        assert_eq!(e.mem.u16(instance + 4), 0x38);
        let body_a = e.mem.u32(this + 0x14);
        let body_b = e.mem.u32(this + 0x18);
        assert_eq!(
            *log.borrow(),
            [
                (4, vec![0xa0]),
                (HINGE_LIMITS_CONSTRUCT, vec![limits]),
                (4, vec![0x38]),
                (
                    CONSTRAINT_INSTANCE_CONSTRUCT,
                    vec![instance, body_a, body_b, limits, 5]
                ),
                (REMOVE_REFERENCE, vec![limits]),
            ]
        );
        assert_eq!(e.mem.u32(data + 0xc0), word(0xc0));
    }

    #[test]
    fn convert_to_limits_builds_ragdoll_limits_for_type_seven() {
        let (mut e, this, data, log, blocks) = limits_rig(7);
        e.mem.set_u8(data + 0x116, 0);
        let got = e.call(0x00e5_aac0, &args![this]).u32();
        assert_eq!(got, blocks[1]);
        let limits = blocks[0];
        let word = |offset: u32| 0x1000 + offset / 4;
        for index in 0..5 {
            assert_eq!(e.mem.u32(limits + 0x80 + 4 * index), word(0xfc) + index);
            assert_eq!(e.mem.u32(limits + 0xa8 + 4 * index), word(0x124) + index);
            assert_eq!(e.mem.u32(limits + 0x94 + 4 * index), word(0x110) + index);
        }
        assert_eq!(e.mem.u16(limits + 4), 0xc0);
        let flags: Vec<Vec<u32>> = log
            .borrow()
            .iter()
            .filter(|(address, _)| *address == RAGDOLL_LIMITS_SET_FLAG)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(flags, [vec![limits, 0]]);
        // A set byte at +0x116 passes 1.
        let (mut e, this, data, log, blocks) = limits_rig(7);
        e.mem.set_u8(data + 0x116, 0x80);
        e.call(0x00e5_aac0, &args![this]);
        assert!(log
            .borrow()
            .contains(&(RAGDOLL_LIMITS_SET_FLAG, vec![blocks[0], 1])));
    }

    #[test]
    fn convert_to_limits_shares_types_0x12_and_0x13_and_refuses_the_rest() {
        for kind in [0x12, 0x13] {
            let (mut e, this, _, log, _) = limits_rig(kind);
            assert_eq!(e.call(0x00e5_aac0, &args![this]).u32(), this);
            assert_eq!(*log.borrow(), [(ADD_REFERENCE, vec![this])]);
        }
        for kind in [0, 3, 0x14] {
            let (mut e, this, _, log, _) = limits_rig(kind);
            assert_eq!(e.call(0x00e5_aac0, &args![this]).u32(), 0);
            assert!(log.borrow().is_empty());
        }
    }

    #[test]
    fn get_constraint_pivots_picks_the_vectors_by_type() {
        for (kind, from_a, from_b) in [
            (0, 0x20, 0x30),
            (1, 0x50, 0x90),
            (2, 0x50, 0x90),
            (7, 0x50, 0x90),
        ] {
            let mut e = rig();
            let data = data_object(&mut e, kind, 0x0900_0100, 0xa0);
            let (this, _, _) = constraint(&mut e, data);
            let out_a = e.mem.alloc(0x10);
            let out_b = e.mem.alloc(0x10);
            assert_eq!(e.call(0x00e5_ad40, &args![this, out_a, out_b]).u32(), 0);
            let word = |offset: u32| 0x1000 + offset / 4;
            assert_eq!(e.mem.u32(out_a), word(from_a));
            assert_eq!(e.mem.u32(out_a + 0xc), word(from_a) + 3);
            assert_eq!(e.mem.u32(out_b), word(from_b));
            assert_eq!(e.mem.u32(out_b + 0xc), word(from_b) + 3);
        }
    }

    #[test]
    fn get_constraint_pivots_reports_an_unsupported_type() {
        let mut e = rig();
        let log = install_report(&mut e);
        let data = data_object(&mut e, 3, 0x0900_0100, 0xa0);
        let (this, _, _) = constraint(&mut e, data);
        let out_a = e.mem.alloc(0x10);
        let out_b = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00e5_ad40, &args![this, out_a, out_b]).u32(), 1);
        assert_eq!(e.mem.u32(out_a), 0);
        let log = log.borrow();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(
            addresses,
            [
                REPORT_BUILDER_CONSTRUCT,
                REPORT_BUILDER_TEXT,
                0x0900_00c0,
                REPORT_BUILDER_DESTRUCT
            ]
        );
        let builder = log[0].1[0];
        assert_eq!(log[0].1, vec![builder, builder + 0xc, 0x200]);
        assert_eq!(log[1].1, vec![builder, 0x010e_ca40]);
        assert_eq!(
            log[2].1,
            vec![1, 0xabba_bf3b, builder + 0xc, REPORT_FILE, 0xe2]
        );
        assert_eq!(log[3].1, vec![builder]);
    }

    #[test]
    fn get_constraint_motors_answers_the_motors_by_type() {
        let mut e = rig();
        let outputs = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        let [twist, cone, plane] = outputs;
        let hinge = data_object(&mut e, 2, 0x0900_0100, 0xc0);
        e.mem.set_u32(hinge + 0xb0, 0x7071);
        e.mem.set_u32(cone, 5);
        assert_eq!(
            e.call(0x00e5_ae60, &args![hinge, twist, cone, plane]).u32(),
            0
        );
        assert_eq!(
            [e.mem.u32(twist), e.mem.u32(cone), e.mem.u32(plane)],
            [0x7071, 0, 0]
        );
        let ragdoll = data_object(&mut e, 7, 0x0900_0101, 0xc0);
        for (address, motor) in [
            (GET_TWIST_MOTOR, 1),
            (GET_CONE_MOTOR, 2),
            (GET_PLANE_MOTOR, 3),
        ] {
            e.register_double(address, move |_, a| Ret {
                eax: a[0] + motor,
                ..Ret::default()
            });
        }
        assert_eq!(
            e.call(0x00e5_ae60, &args![ragdoll, twist, cone, plane])
                .u32(),
            0
        );
        assert_eq!(
            [e.mem.u32(twist), e.mem.u32(cone), e.mem.u32(plane)],
            [ragdoll + 1, ragdoll + 2, ragdoll + 3]
        );
    }

    #[test]
    fn get_constraint_motors_clears_and_reports_other_types() {
        let mut e = rig();
        let log = install_report(&mut e);
        let data = data_object(&mut e, 1, 0x0900_0100, 0xc0);
        let outputs = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        for output in outputs {
            e.mem.set_u32(output, 0xdead);
        }
        let [twist, cone, plane] = outputs;
        assert_eq!(
            e.call(0x00e5_ae60, &args![data, twist, cone, plane]).u32(),
            1
        );
        assert_eq!(
            [e.mem.u32(twist), e.mem.u32(cone), e.mem.u32(plane)],
            [0, 0, 0]
        );
        let log = log.borrow();
        let builder = log[0].1[0];
        assert_eq!(log[1].1, vec![builder, 0x010e_ca7c]);
        assert_eq!(
            log[2].1,
            vec![1, 0xabba_e233, builder + 0xc, REPORT_FILE, 0x104]
        );
    }

    /// A rig for `checkAndFixConstraint`: data of type 1 with pivots at the origin, a
    /// transform double that adds the first three floats at the transform address, and doubles
    /// for the body setters.
    fn fix_rig() -> (Engine, u32, u32, u32, Calls) {
        let mut e = rig();
        let data = data_object(&mut e, 1, 0x0900_0100, 0xa0);
        let (this, body_a, body_b) = constraint(&mut e, data);
        for at in [data + 0x50, data + 0x90] {
            set_vector(&mut e, at, [0.0; 4]);
        }
        set_vector(&mut e, body_a + 0xf0, [0.0; 4]);
        set_vector(&mut e, body_b + 0xf0, [3.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, body_b + 0x120, [4.0, 8.0, 12.0, 0.0]);
        e.register_double(SET_TRANSFORMED_POS, |e, a| {
            for lane in 0..3 {
                let value = e.mem.f32(a[2] + 4 * lane) + e.mem.f32(a[1] + 4 * lane);
                e.mem.set_f32(a[0] + 4 * lane, value);
            }
            let w = e.mem.u32(a[2] + 12);
            e.mem.set_u32(a[0] + 12, w);
            Ret::default()
        });
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(SET_POSITION, move |e, a| {
            let lanes = [0, 1, 2, 3].map(|lane| e.mem.u32(a[1] + 4 * lane));
            sink.borrow_mut().push((
                SET_POSITION,
                vec![a[0], lanes[0], lanes[1], lanes[2], lanes[3]],
            ));
            Ret::default()
        });
        for address in [SET_LINEAR_VELOCITY, SET_ANGULAR_VELOCITY] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, this, body_a, body_b, log)
    }

    #[test]
    fn check_and_fix_moves_the_first_body_when_the_pivots_are_too_far_apart() {
        let (mut e, this, body_a, body_b, log) = fix_rig();
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(
            0x00e5_af80,
            &args![out, this, 2.0f32.to_bits(), 0.25f32.to_bits()],
        );
        assert_eq!(e.mem.u8(out), 1);
        // world B = (3, 0, 0), world A = 0: 9 > 4. New position 0.75 * world B + 0.25 * (4, 8, 12).
        let expected = [3.25f32, 2.0, 3.0, 0.0].map(f32::to_bits);
        assert_eq!(
            *log.borrow(),
            [
                (
                    SET_POSITION,
                    vec![body_a, expected[0], expected[1], expected[2], expected[3]]
                ),
                (SET_LINEAR_VELOCITY, vec![body_a, body_b + 0x1b0]),
                (SET_ANGULAR_VELOCITY, vec![body_a, body_b + 0x1c0]),
            ]
        );
    }

    #[test]
    fn check_and_fix_moves_the_first_body_when_the_pivots_are_too_far_apart_answers_its_result_pointer(
    ) {
        let (mut e, this, _body_a, _body_b, _log) = fix_rig();
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(
                0x00e5_af80,
                &args![out, this, 2.0f32.to_bits(), 0.25f32.to_bits()],
            )
            .u32(),
            args![out][0]
        );
    }

    #[test]
    fn check_and_fix_leaves_close_pivots_and_reports_a_misaligned_pivot() {
        let (mut e, this, _, _, log) = fix_rig();
        let sink = install_report(&mut e);
        let data = e.mem.u32(this + 0xc);
        set_vector(&mut e, data + 0x50, [0.5, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(
            0x00e5_af80,
            &args![out, this, 10.0f32.to_bits(), 0.25f32.to_bits()],
        );
        assert_eq!(e.mem.u8(out), 0);
        assert!(log.borrow().is_empty());
        let reports: Vec<Vec<u32>> = sink
            .borrow()
            .iter()
            .filter(|(address, _)| *address == 0x0900_00c0)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0][1], 0xabba_5dff);
        assert_eq!(reports[0][4], 0x11d);
    }

    #[test]
    fn check_and_fix_gives_up_on_an_unsupported_constraint() {
        let mut e = rig();
        install_report(&mut e);
        let data = data_object(&mut e, 5, 0x0900_0100, 0xa0);
        let (this, _, _) = constraint(&mut e, data);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(0x00e5_af80, &args![out, this, 0u32, 0u32]);
        assert_eq!(e.mem.u8(out), 0);
    }
}
