//! `NiCollision/nispherebv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, float_operand, load_float, stream_transfer,
};

layout! {
    /// `NiSphereBV` (Xbox PDB), 0x14 bytes: the vtable and an `NiSphere` (center, then radius),
    /// declared as words because most of the code only moves them.
    pub struct NiSphereBV: 0x14 {
        /// `m_kSphere.m_kCenter` (Xbox PDB), x.
        0x04 m_kSphere_center_x: u32,
        /// `m_kSphere.m_kCenter` (Xbox PDB), y.
        0x08 m_kSphere_center_y: u32,
        /// `m_kSphere.m_kCenter` (Xbox PDB), z.
        0x0C m_kSphere_center_z: u32,
        /// `m_kSphere.m_fRadius` (Xbox PDB).
        0x10 m_kSphere_radius: u32,
    }
}

/// The vtable stored at +0 (`010f1854`).
const NI_SPHERE_BV_VTABLE: u32 = 0x010f_1854;
/// The memory manager instance (`this` of its allocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator `00aa1070` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The three words at `011f426c` that `Create` uses as the default center.
const DEFAULT_CENTER_011F426C: u32 = 0x011f_426c;
/// `NiSphere::operator==` (`00a92660`, thiscall on one sphere, the other sphere).
const NI_SPHERE_EQUAL: u32 = 0x00a9_2660;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`, thiscall, the stream; it does nothing).
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
/// `NiBoundingVolume::SaveBinary` (`00e9cb10`, thiscall, the stream).
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// `NiSphere::LoadBinary` (`00a69ac0`, thiscall on the sphere, the stream).
const NI_SPHERE_LOAD_BINARY: u32 = 0x00a6_9ac0;
/// `NiSphere::SaveBinary` (`00eb5410`, thiscall on the sphere, the stream).
const NI_SPHERE_SAVE_BINARY: u32 = 0x00eb_5410;
/// `NiColor::LoadBinary` (`00a59be0`, thiscall on a three-float block, the stream): the three
/// floats are read as three 4-byte items.
const LOAD_THREE_FLOATS: u32 = 0x00a5_9be0;
/// `NiPoint3::GetViewerString` (`00a59af0`, thiscall on the point, the name).
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `"%s = SPHERE_BV"` and the name of the center line and the radius format.
const SPHERE_BV_FORMAT: u32 = 0x010f_18a4;
const CENTER_NAME: u32 = 0x010f_1898;
const RADIUS_FORMAT: u32 = 0x010f_1884;
/// The double `100.0` (`01017a40`) the radius is multiplied by before it is truncated.
const RADIUS_SCALE: u32 = 0x0101_7a40;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (here a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `NiSqrDistance::Compute_ov4` (`00eb5450`, cdecl: point, triangle, two out floats): the squared
/// distance from a point to a triangle, in ST0.
const SQR_DISTANCE_POINT_TRIANGLE: u32 = 0x00eb_5450;
/// `NiSqrDistance::Compute_ov3` (`00eb1d20`, cdecl: segment, triangle, three out floats): the
/// squared distance from a segment to a triangle, in ST0.
const SQR_DISTANCE_SEGMENT_TRIANGLE: u32 = 0x00eb_1d20;
/// Unnamed in the engine map: normalizes the three floats at ECX in place (a vector whose
/// length is at most `1e-6` becomes zero). `00457990` gives the length.
const UNITIZE: u32 = 0x004a_0c10;
/// Unnamed in the engine map: `(out, scalar, vector)` (cdecl), writes the vector scaled by the
/// scalar to `out` and answers `out`.
const SCALE_POINT: u32 = 0x004a_3760;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one point, the other point), in ST0.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// `NiTrigon::GetNormal` (`00eb3830`, thiscall on the triangle, the out point): answers the
/// out point.
const NI_TRIGON_GET_NORMAL: u32 = 0x00eb_3830;
/// The `float` `01011d78` (0.0 in the executable) the approach speed is compared with.
const APPROACH_THRESHOLD: u32 = 0x0101_1d78;
/// The double `0.001` (`01016978`) added to the plane constant.
const PLANE_MARGIN: u32 = 0x0101_6978;
/// The double (about `1e-5`, `0101dae8`) the squared relative speed times the time must exceed.
const MINIMUM_MOTION: u32 = 0x0101_dae8;
/// The `float`s (`0.0001`, `0.001`) handed to the intersector as its pseudodistance threshold and
/// derivative time step.
const PSEUDODISTANCE_THRESHOLD: u32 = 0x0101_41cc;
const DERIVATIVE_TIME_STEP: u32 = 0x0101_7d00;
/// The intersector's constructor (`00eb5e30`, thiscall on the object, the sphere volume, the three
/// triangle vertices, then the maximum time, the derivative time step, the pseudodistance
/// threshold and the iteration limit), `NiSphereTriIntersector` of 0x6c bytes.
const SPHERE_TRI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00eb_5e30;
/// `NiIntersector::Find` (`00eae7a0`, thiscall, the two velocities).
const INTERSECTOR_FIND: u32 = 0x00ea_e7a0;
/// The intersector's contact time getter (`00eae6f0`, in ST0), contact point getter (`00eae700`),
/// contact normal getter (`00eae710`), intersection type getter (`00be0aa0`) and destructor
/// (`00eae6d0`), all thiscall on the intersector.
const INTERSECTOR_CONTACT_TIME: u32 = 0x00ea_e6f0;
const INTERSECTOR_CONTACT_POINT: u32 = 0x00ea_e700;
const INTERSECTOR_CONTACT_NORMAL: u32 = 0x00ea_e710;
const INTERSECTOR_TYPE: u32 = 0x00be_0aa0;
const INTERSECTOR_DESTRUCTOR: u32 = 0x00ea_e6d0;
/// The size of `NiSphereTriIntersector` (Xbox PDB).
const SPHERE_TRI_INTERSECTOR_SIZE: u32 = 0x6c;

/// The bits `FLD`, `FCHS` and `FSTP` leave for a float: the sign flipped, a signalling NaN
/// quieted.
pub(crate) fn negated_bits(bits: u32) -> u32 {
    x87_quiet_bits(bits) ^ 0x8000_0000
}

// Translated from 00eab820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::operator==` (Xbox PDB): the other volume's virtual `Type` (slot +0xc) must be 0
/// and its sphere (+4) must equal this one's (`NiSphere::operator==`, `00a92660`, called on the
/// other volume's sphere with this one's as the argument).
pub fn ni_sphere_bv_operator_(
    e: &mut Engine,
    this: Ptr<NiSphereBV>,
    other: Ptr<NiSphereBV>,
) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 0 {
        return false;
    }
    e.call(
        NI_SPHERE_EQUAL,
        &args![other.addr().wrapping_add(4), this.addr().wrapping_add(4)],
    )
    .bool()
}

// Translated from 00eab860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::LoadBinary` (Xbox PDB): the base volume's part (`00a89af0`, which does nothing),
/// then `NiSphere::LoadBinary` on the sphere at +4.
pub fn ni_sphere_bv_load_binary(e: &mut Engine, this: Ptr<NiSphereBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.call(
        NI_SPHERE_LOAD_BINARY,
        &args![this.addr().wrapping_add(4), stream],
    );
}

// Translated from 00eab880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::SaveBinary` (Xbox PDB): `NiBoundingVolume::SaveBinary`, then
/// `NiSphere::SaveBinary` on the sphere at +4.
pub fn ni_sphere_bv_save_binary(e: &mut Engine, this: Ptr<NiSphereBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.call(
        NI_SPHERE_SAVE_BINARY,
        &args![this.addr().wrapping_add(4), stream],
    );
}

// Translated from 00eab8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiSphereBV` constructor from a radius and a center. Sets the
/// vtable `010f1854`, copies the center point (three words at `center`) to +4 and stores the
/// radius at +0x10 as `FLD`/`FSTP` leave it (a signalling NaN quieted). Answers `this`.
pub fn fn_00eab8a0(
    e: &mut Engine,
    this: Ptr<NiSphereBV>,
    radius: f32,
    center: Ptr,
) -> Ptr<NiSphereBV> {
    e.mem.set_u32(this.addr(), NI_SPHERE_BV_VTABLE);
    let x = e.mem.u32(center.addr());
    e.set(this, NiSphereBV::m_kSphere_center_x, x);
    let y = e.mem.u32(center.addr().wrapping_add(4));
    e.set(this, NiSphereBV::m_kSphere_center_y, y);
    let z = e.mem.u32(center.addr().wrapping_add(8));
    e.set(
        this,
        NiSphereBV::m_kSphere_radius,
        x87_quiet_bits(radius.to_bits()),
    );
    e.set(this, NiSphereBV::m_kSphere_center_z, z);
    this
}

// Translated from 00eab8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::Copy` (Xbox PDB): copies the four words of the other volume's sphere (+4..+0x13).
pub fn ni_sphere_bv_copy(e: &mut Engine, this: Ptr<NiSphereBV>, other: Ptr<NiSphereBV>) {
    for offset in [4u32, 8, 0xc, 0x10] {
        let word = e.mem.u32(other.addr().wrapping_add(offset));
        e.mem.set_u32(this.addr().wrapping_add(offset), word);
    }
}

// Translated from 00eab900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::UpdateWorldData` (Xbox PDB): sets this volume to the model sphere `model`
/// moved by the transform `transform` (a row-major 3x3 rotation at +0, the translation at +0x24,
/// the scale at +0x30). Each component of the rotated center is the sum of the products of
/// a rotation row with the model center (the row's second element first), rounded to a float; it
/// is then multiplied by the scale (the scale itself passes through `FLD`/`FSTP`), rounded, and the
/// translation (`FLD` plus the stored product) added and rounded. The radius is the model radius
/// times the scale.
pub fn ni_sphere_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiSphereBV>,
    model: Ptr<NiSphereBV>,
    transform: Ptr,
) {
    let t = transform.addr();
    let m = model.addr();
    let mut rotated = [0u32; 3];
    for (row, slot) in rotated.iter_mut().enumerate() {
        let r = t.wrapping_add(12 * row as u32);
        let second = x87_mul(
            load_float(e, r.wrapping_add(4)),
            float_operand(e, m.wrapping_add(8)),
        );
        let first = x87_mul(load_float(e, r), float_operand(e, m.wrapping_add(4)));
        let third = x87_mul(
            load_float(e, r.wrapping_add(8)),
            float_operand(e, m.wrapping_add(0xc)),
        );
        *slot = x87_store32(x87_add(x87_add(second, first), third));
    }
    let scale_bits = x87_store32(load_float(e, t.wrapping_add(0x30)));
    let scale = x87_load32(scale_bits);
    let scaled = [
        x87_store32(x87_mul(x87_load32(rotated[0]), scale)),
        x87_store32(x87_mul(x87_load32(rotated[1]), scale)),
        x87_store32(x87_mul(scale, x87_m32(rotated[2]))),
    ];
    let mut world = [0u32; 3];
    for (axis, slot) in world.iter_mut().enumerate() {
        let translation = load_float(e, t.wrapping_add(0x24 + 4 * axis as u32));
        *slot = x87_store32(x87_add(translation, x87_m32(scaled[axis])));
    }
    e.set(this, NiSphereBV::m_kSphere_center_x, world[0]);
    e.set(this, NiSphereBV::m_kSphere_center_y, world[1]);
    e.set(this, NiSphereBV::m_kSphere_center_z, world[2]);
    let radius = x87_mul(
        load_float(e, m.wrapping_add(0x10)),
        float_operand(e, t.wrapping_add(0x30)),
    );
    e.set(this, NiSphereBV::m_kSphere_radius, x87_store32(radius));
}

// Translated from 00eac200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::Create` (Xbox PDB): allocates 0x14 bytes from the memory manager and, when that
/// gives a block, sets the vtable, the center from the three words at `011f426c` and the radius
/// to 1.0. Answers the new volume, or null.
pub fn ni_sphere_bv_create(e: &mut Engine) -> Ptr<NiSphereBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let created: Ptr<NiSphereBV> = Ptr::new(block);
    e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
    let x = e.mem.u32(DEFAULT_CENTER_011F426C);
    e.set(created, NiSphereBV::m_kSphere_center_x, x);
    let y = e.mem.u32(DEFAULT_CENTER_011F426C + 4);
    e.set(created, NiSphereBV::m_kSphere_center_y, y);
    let z = e.mem.u32(DEFAULT_CENTER_011F426C + 8);
    e.set(created, NiSphereBV::m_kSphere_radius, 1.0f32.to_bits());
    e.set(created, NiSphereBV::m_kSphere_center_z, z);
    created
}

// Translated from 00eac240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::SphereTriTestIntersect` (Xbox PDB), a static taking seven words: the time
/// (`time`), the sphere volume, the sphere's velocity, the three triangle vertices and the
/// triangle's velocity. Answers whether the sphere touches the triangle during the time.
///
/// The triangle is the first vertex and the two edges from it (floats, each difference rounded).
/// The relative velocity is the sphere's minus the triangle's, formed component by component and
/// compared (`FUCOMPP`) with the three words at `011f426c`: when all three are equal the
/// squared distance is that of the sphere's center to the triangle (`Compute_ov4`, `00eb5450`);
/// otherwise it is that of the segment from the center along `time * velocity` (each product
/// rounded to a float) to the triangle (`Compute_ov3`, `00eb1d20`). The answer is whether that
/// distance, rounded to a float, is at most the radius times the radius (ordered).
pub fn ni_sphere_bv_sphere_tri_test_intersect(
    e: &mut Engine,
    time: f32,
    sphere: Ptr<NiSphereBV>,
    sphere_velocity: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    e.with_stack(0x24 + 12 + 24 + 12, |e, frame| {
        let triangle = frame.addr();
        let segment = triangle + 0x24;
        let scratch = segment + 24;
        for axis in 0..3u32 {
            let word = e.mem.u32(first.addr().wrapping_add(4 * axis));
            e.mem.set_u32(triangle + 4 * axis, word);
        }
        for axis in 0..3u32 {
            let edge = x87_sub(
                load_float(e, second.addr().wrapping_add(4 * axis)),
                float_operand(e, first.addr().wrapping_add(4 * axis)),
            );
            e.mem.set_u32(triangle + 0xc + 4 * axis, x87_store32(edge));
        }
        for axis in 0..3u32 {
            let edge = x87_sub(
                load_float(e, third.addr().wrapping_add(4 * axis)),
                float_operand(e, first.addr().wrapping_add(4 * axis)),
            );
            e.mem.set_u32(triangle + 0x18 + 4 * axis, x87_store32(edge));
        }
        let mut relative = [0u32; 3];
        for (axis, slot) in relative.iter_mut().enumerate() {
            let offset = 4 * axis as u32;
            let difference = x87_sub(
                load_float(e, sphere_velocity.addr().wrapping_add(offset)),
                float_operand(e, triangle_velocity.addr().wrapping_add(offset)),
            );
            *slot = x87_store32(difference);
        }
        // `FUCOMPP` against the three words at 011f426c: equal only when ordered and equal.
        let mut still = true;
        for (axis, slot) in relative.iter().enumerate() {
            let zero = load_float(e, DEFAULT_CENTER_011F426C + 4 * axis as u32);
            if !(x87_load32(*slot) == zero) {
                still = false;
                break;
            }
        }
        let center = sphere.addr().wrapping_add(4);
        let squared_distance = if still {
            e.call(
                SQR_DISTANCE_POINT_TRIANGLE,
                &args![center, triangle, scratch, scratch + 4],
            )
            .f64()
        } else {
            let scale = x87_load32(time.to_bits());
            for axis in 0..3u32 {
                let word = e.mem.u32(center.wrapping_add(4 * axis));
                e.mem.set_u32(segment + 4 * axis, word);
            }
            let x = x87_store32(x87_mul(x87_load32(relative[0]), scale));
            let y = x87_store32(x87_mul(x87_load32(relative[1]), scale));
            let z = x87_store32(x87_mul(scale, x87_load32(relative[2])));
            e.mem.set_u32(segment + 12, x);
            e.mem.set_u32(segment + 16, y);
            e.mem.set_u32(segment + 20, z);
            e.call(
                SQR_DISTANCE_SEGMENT_TRIANGLE,
                &args![segment, triangle, scratch, scratch + 4, scratch + 8],
            )
            .f64()
        };
        let distance = x87_load32(x87_store32(squared_distance));
        let radius = load_float(e, sphere.addr().wrapping_add(0x10));
        distance <= x87_mul(radius, radius)
    })
}

// Translated from 00eac3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the sphere-triangle `Find`
/// test, a static taking twelve words: the time, the sphere volume, the sphere's velocity, the
/// three triangle vertices, the triangle's velocity, `out_time` (a float), `out_point` (a point),
/// `want_normals` (a byte), `out_normal` and `out_normal_negated` (points). Answers whether the
/// sphere touches the triangle during the time and, if so, when and where.
///
/// The squared distance from the sphere's center to the triangle is computed
/// (`Compute_ov4`, `00eb5450`, with the barycentric parameters of the closest point as out
/// floats) and `distance - radius * radius` rounded to a float. When that is at most 0 the sphere
/// already touches the triangle: the time is 0, the contact point is the closest point
/// (`origin + s * edge0 + t * edge1`, each term and sum rounded to a float) and, when
/// `want_normals` is set, the triangle's normal (`NiTrigon::GetNormal`) goes to
/// `out_normal_negated` and its negation to `out_normal` (the roles are swapped against the
/// general case below); the answer is true.
///
/// Otherwise the relative velocity (the sphere's minus the triangle's, each component rounded) is
/// squared (`vx*vx + vy*vy + vz*vz`, rounded) and multiplied by the time; a product that is not
/// above the double at `0101dae8` answers false. The triangle's normal is the cross product of
/// its edges (each component rounded); its negation `u` gives the signed distances `d1 = u .
/// center` and `d2 = u . vertex0` (rounded) and `dist = d1 - d2` (rounded). When
/// `radius^2 * |u|^2` (both rounded) is below `dist^2`, the sphere is clear of the plane: `u` is
/// unitized, and when `d2 < d1` an approach speed `u . velocity` that is not below the float
/// `01011d78` answers false (when `d2 >= d1` one at or below it answers false); the sphere's
/// contact point `w` is `center - radius * u` (or `center + radius * u` in the second case),
/// the plane constant is `u . vertex0 + 0.001` (double at `01016978`) and, if `u . w` is above
/// it and `u . (w + time * velocity)` is also above it, the answer is false. Every other case ends
/// in the general test: an `NiSphereTriIntersector` (0x6c bytes on the stack, constructed by
/// `00eb5e30` with the time, `0.001`, `0.0001` and 0x20 iterations) runs `Find` with the two
/// velocities; its contact time goes to `out_time`; with an intersection type of 2 or 3 its
/// contact point goes to `out_point`, its contact normal to `out_normal` and the negated normal
/// to `out_normal_negated` (when `want_normals`) and the answer is true, otherwise false. The
/// intersector is destroyed on both paths.
pub fn fn_00eac3f0(
    e: &mut Engine,
    time: f32,
    sphere: Ptr<NiSphereBV>,
    sphere_velocity: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    triangle_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    let sphere_base = sphere.addr();
    let radius = load_float(e, sphere_base.wrapping_add(0x10));
    let radius_squared = x87_store32(x87_mul(radius, radius));
    e.with_stack(0x68, |e, frame| {
        let triangle = frame.addr();
        let s_out = triangle + 0x24;
        let t_out = triangle + 0x28;
        let center = triangle + 0x2c;
        let unit = triangle + 0x38;
        let result_a = triangle + 0x44;
        let result_b = triangle + 0x50;
        let velocity = triangle + 0x5c;
        for axis in 0..3u32 {
            let word = e.mem.u32(sphere_base.wrapping_add(4 + 4 * axis));
            e.mem.set_u32(center + 4 * axis, word);
            let word = e.mem.u32(first.addr().wrapping_add(4 * axis));
            e.mem.set_u32(triangle + 4 * axis, word);
        }
        for axis in 0..3u32 {
            let edge = x87_sub(
                load_float(e, second.addr().wrapping_add(4 * axis)),
                float_operand(e, first.addr().wrapping_add(4 * axis)),
            );
            e.mem.set_u32(triangle + 0xc + 4 * axis, x87_store32(edge));
        }
        for axis in 0..3u32 {
            let edge = x87_sub(
                load_float(e, third.addr().wrapping_add(4 * axis)),
                float_operand(e, first.addr().wrapping_add(4 * axis)),
            );
            e.mem.set_u32(triangle + 0x18 + 4 * axis, x87_store32(edge));
        }
        let distance = e
            .call(
                SQR_DISTANCE_POINT_TRIANGLE,
                &args![center, triangle, s_out, t_out],
            )
            .f64();
        let difference = x87_store32(x87_sub(distance, x87_load32(radius_squared)));
        if x87_load32(difference) <= 0.0 {
            // The sphere already touches the triangle.
            e.mem.set_u32(out_time.addr(), 0);
            let t = load_float(e, t_out);
            let s = load_float(e, s_out);
            let along_second = [
                x87_store32(x87_mul(load_float(e, triangle + 0x18), t)),
                x87_store32(x87_mul(load_float(e, triangle + 0x1c), t)),
                x87_store32(x87_mul(t, float_operand(e, triangle + 0x20))),
            ];
            let along_first = [
                x87_store32(x87_mul(load_float(e, triangle + 0xc), s)),
                x87_store32(x87_mul(load_float(e, triangle + 0x10), s)),
                x87_store32(x87_mul(s, float_operand(e, triangle + 0x14))),
            ];
            let mut on_plane = [0u32; 3];
            for axis in 0..3usize {
                on_plane[axis] = x87_store32(x87_add(
                    load_float(e, triangle + 4 * axis as u32),
                    x87_m32(along_first[axis]),
                ));
            }
            for axis in 0..3usize {
                let point = x87_store32(x87_add(
                    x87_load32(on_plane[axis]),
                    x87_m32(along_second[axis]),
                ));
                e.mem
                    .set_u32(out_point.addr().wrapping_add(4 * axis as u32), point);
            }
            if !want_normals {
                return true;
            }
            let normal = e
                .call(NI_TRIGON_GET_NORMAL, &args![triangle, result_a])
                .u32();
            for axis in 0..3u32 {
                let word = e.mem.u32(normal.wrapping_add(4 * axis));
                e.mem
                    .set_u32(out_normal_negated.addr().wrapping_add(4 * axis), word);
            }
            for axis in 0..3u32 {
                let word = e.mem.u32(out_normal_negated.addr().wrapping_add(4 * axis));
                e.mem
                    .set_u32(out_normal.addr().wrapping_add(4 * axis), negated_bits(word));
            }
            return true;
        }

        // The sphere starts clear of the triangle: look at how it moves.
        let mut moving = [0u32; 3];
        for (axis, slot) in moving.iter_mut().enumerate() {
            let offset = 4 * axis as u32;
            let difference = x87_sub(
                load_float(e, sphere_velocity.addr().wrapping_add(offset)),
                float_operand(e, triangle_velocity.addr().wrapping_add(offset)),
            );
            *slot = x87_store32(difference);
            e.mem.set_u32(velocity + offset, *slot);
        }
        let (vx, vy, vz) = (
            x87_load32(moving[0]),
            x87_load32(moving[1]),
            x87_load32(moving[2]),
        );
        let speed_squared = x87_store32(x87_add(
            x87_add(x87_mul(vx, vx), x87_mul(vy, vy)),
            x87_mul(vz, vz),
        ));
        let motion = x87_mul(x87_load32(speed_squared), x87_m32(time.to_bits()));
        if motion <= e.mem.f64(MINIMUM_MOTION) {
            return false;
        }
        let edge0 = [
            load_float(e, triangle + 0xc),
            load_float(e, triangle + 0x10),
            load_float(e, triangle + 0x14),
        ];
        let edge1 = [
            load_float(e, triangle + 0x18),
            load_float(e, triangle + 0x1c),
            load_float(e, triangle + 0x20),
        ];
        let normal = [
            x87_store32(x87_sub(
                x87_mul(edge0[1], edge1[2]),
                x87_mul(edge0[2], edge1[1]),
            )),
            x87_store32(x87_sub(
                x87_mul(edge0[2], edge1[0]),
                x87_mul(edge0[0], edge1[2]),
            )),
            x87_store32(x87_sub(
                x87_mul(edge0[0], edge1[1]),
                x87_mul(edge0[1], edge1[0]),
            )),
        ];
        for axis in 0..3usize {
            e.mem
                .set_u32(unit + 4 * axis as u32, negated_bits(normal[axis]));
        }
        let (a, b, c) = (
            load_float(e, unit),
            load_float(e, unit + 4),
            load_float(e, unit + 8),
        );
        let (cx, cy, cz) = (
            float_operand(e, center),
            load_float(e, center + 4),
            load_float(e, center + 8),
        );
        let center_distance = x87_store32(x87_add(
            x87_mul(cz, c),
            x87_add(x87_mul(cy, b), x87_mul(a, cx)),
        ));
        let vertex_distance = x87_store32(x87_add(
            x87_add(
                x87_mul(a, float_operand(e, first.addr())),
                x87_mul(load_float(e, first.addr().wrapping_add(4)), b),
            ),
            x87_mul(c, float_operand(e, first.addr().wrapping_add(8))),
        ));
        let plane_distance = x87_store32(x87_sub(
            x87_load32(center_distance),
            x87_m32(vertex_distance),
        ));
        let plane_distance = x87_load32(plane_distance);
        let distance_squared = x87_mul(plane_distance, plane_distance);
        let normal_squared = x87_store32(x87_add(
            x87_add(x87_mul(a, a), x87_mul(b, b)),
            x87_mul(c, c),
        ));
        let reach = x87_store32(x87_mul(
            x87_load32(radius_squared),
            x87_load32(normal_squared),
        ));
        let clear_of_plane = x87_load32(reach) < distance_squared;
        if clear_of_plane {
            e.call(UNITIZE, &args![unit]);
            let u = [
                load_float(e, unit),
                load_float(e, unit + 4),
                load_float(e, unit + 8),
            ];
            let speed = x87_add(
                x87_add(
                    x87_mul(u[0], float_operand(e, velocity)),
                    x87_mul(u[1], float_operand(e, velocity + 4)),
                ),
                x87_mul(u[2], float_operand(e, velocity + 8)),
            );
            let speed = x87_load32(x87_store32(speed));
            let threshold = float_operand(e, APPROACH_THRESHOLD);
            // The vertex distance against the center distance (`FCOMPP`) picks the sign of the
            // test and of the contact point.
            let against = x87_load32(vertex_distance) < x87_load32(center_distance);
            if against {
                if speed >= threshold {
                    return false;
                }
            } else if speed <= threshold {
                return false;
            }
            let radius_bits = x87_quiet_bits(e.mem.u32(sphere_base.wrapping_add(0x10)));
            let scaled = e
                .call(SCALE_POINT, &args![result_a, radius_bits, unit])
                .u32();
            let mut contact = [0u32; 3];
            for (axis, slot) in contact.iter_mut().enumerate() {
                let offset = 4 * axis as u32;
                *slot = if against {
                    x87_store32(x87_sub(
                        load_float(e, center + offset),
                        float_operand(e, scaled.wrapping_add(offset)),
                    ))
                } else {
                    x87_store32(x87_add(
                        load_float(e, scaled.wrapping_add(offset)),
                        float_operand(e, center + offset),
                    ))
                };
            }
            let unit_x = load_float(e, unit);
            let unit_y = load_float(e, unit + 4);
            let unit_z = load_float(e, unit + 8);
            let plane_constant = x87_store32(x87_add(
                x87_add(
                    x87_mul(unit_x, float_operand(e, first.addr())),
                    x87_mul(load_float(e, first.addr().wrapping_add(4)), unit_y),
                ),
                x87_mul(unit_z, float_operand(e, first.addr().wrapping_add(8))),
            ));
            let margin = x87_add(x87_load32(plane_constant), e.mem.f64(PLANE_MARGIN));
            let contact_height = x87_load32(x87_store32(x87_add(
                x87_add(
                    x87_mul(unit_y, x87_load32(contact[1])),
                    x87_mul(unit_x, x87_load32(contact[0])),
                ),
                x87_mul(unit_z, x87_m32(contact[2])),
            )));
            // Not above the plane (or unordered): the general test decides.
            if contact_height > margin {
                let time_bits = x87_quiet_bits(time.to_bits());
                let travelled = e
                    .call(SCALE_POINT, &args![result_b, time_bits, velocity])
                    .u32();
                for (axis, word) in contact.iter().enumerate() {
                    let offset = 4 * axis as u32;
                    let moved = x87_store32(x87_add(
                        load_float(e, travelled.wrapping_add(offset)),
                        x87_m32(*word),
                    ));
                    e.mem.set_u32(result_a + offset, moved);
                }
                let height = e.call(NI_POINT3_DOT, &args![unit, result_a]).f64();
                if height > margin {
                    return false;
                }
            }
        }

        // The general test.
        e.with_stack(SPHERE_TRI_INTERSECTOR_SIZE, |e, intersector| {
            let maximum_time = x87_quiet_bits(time.to_bits());
            let derivative_step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
            let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
            e.call(
                SPHERE_TRI_INTERSECTOR_CONSTRUCTOR,
                &args![
                    intersector,
                    sphere,
                    first,
                    second,
                    third,
                    maximum_time,
                    derivative_step,
                    threshold,
                    0x20u32
                ],
            );
            e.call(
                INTERSECTOR_FIND,
                &args![intersector, sphere_velocity, triangle_velocity],
            );
            let contact_time = e.call(INTERSECTOR_CONTACT_TIME, &args![intersector]).f64();
            e.mem.set_u32(out_time.addr(), x87_store32(contact_time));
            let kind = e.call(INTERSECTOR_TYPE, &args![intersector]).u32();
            if kind != 3 && kind != 2 {
                e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
                return false;
            }
            let point = e.call(INTERSECTOR_CONTACT_POINT, &args![intersector]).u32();
            for axis in 0..3u32 {
                let word = e.mem.u32(point.wrapping_add(4 * axis));
                e.mem.set_u32(out_point.addr().wrapping_add(4 * axis), word);
            }
            if want_normals {
                let normal = e
                    .call(INTERSECTOR_CONTACT_NORMAL, &args![intersector])
                    .u32();
                for axis in 0..3u32 {
                    let word = e.mem.u32(normal.wrapping_add(4 * axis));
                    e.mem
                        .set_u32(out_normal.addr().wrapping_add(4 * axis), word);
                }
                for axis in 0..3u32 {
                    let word = e.mem.u32(out_normal.addr().wrapping_add(4 * axis));
                    e.mem.set_u32(
                        out_normal_negated.addr().wrapping_add(4 * axis),
                        negated_bits(word),
                    );
                }
            }
            e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
            true
        })
    })
}

// Translated from 00eacac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::Clone` (Xbox PDB): allocates 0x14 bytes from the memory manager and, when that
/// gives a block, sets the vtable and copies the four words of this volume's sphere. Answers
/// the copy, or null.
pub fn ni_sphere_bv_clone(e: &mut Engine, this: Ptr<NiSphereBV>) -> Ptr<NiSphereBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10] {
        let word = e.mem.u32(this.addr().wrapping_add(offset));
        e.mem.set_u32(block.wrapping_add(offset), word);
    }
    Ptr::new(block)
}

// Translated from 00eacb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the stream loader for the older sphere volume record. Reads three
/// points (`00a59be0`, three floats each, into three local blocks) and three 4-byte floats (the
/// stream's read function at +8, as in [`ni_collision_data_load_binary`](super::nicollisiondata)),
/// allocates 0x14 bytes from the memory manager and, when that gives a block, makes a sphere volume
/// of the FIRST point as center and the first float as radius (stored as `FLD`/`FSTP` leave it).
/// The two other points and floats are read and dropped. The reads happen even when the
/// allocation fails. Answers the volume, or null.
pub fn fn_00eacb00(e: &mut Engine, stream: Ptr) -> Ptr<NiSphereBV> {
    e.with_stack(12 * 3 + 12, |e, blocks| {
        let center = blocks.addr();
        e.call(LOAD_THREE_FLOATS, &args![center, stream]);
        e.call(LOAD_THREE_FLOATS, &args![center + 12, stream]);
        e.call(LOAD_THREE_FLOATS, &args![center + 24, stream]);
        let floats = center + 36;
        for index in 0..3u32 {
            let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
            stream_transfer(e, reader, 8, floats + 4 * index, 4);
        }
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        if block == 0 {
            return Ptr::NULL;
        }
        let created: Ptr<NiSphereBV> = Ptr::new(block);
        e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
        let radius = x87_quiet_bits(e.mem.u32(floats));
        let x = e.mem.u32(center);
        e.set(created, NiSphereBV::m_kSphere_center_x, x);
        let y = e.mem.u32(center + 4);
        e.set(created, NiSphereBV::m_kSphere_center_y, y);
        let z = e.mem.u32(center + 8);
        e.set(created, NiSphereBV::m_kSphere_radius, radius);
        e.set(created, NiSphereBV::m_kSphere_center_z, z);
        created
    })
}

// Translated from 00eacbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereBV::AddViewerStrings` (Xbox PDB): to the strings array `strings` it appends the line
/// `"<name> = SPHERE_BV"` (a block of `strlen(name) + 0xd` bytes from `00aa1070`, formatted by
/// `NiSprintf`), the center's strings (`NiPoint3::GetViewerString` on the center at +4, named
/// `"center"`), and the line `"     radius = %g"` for the radius: the radius times 100.0 (double
/// at `01017a40`, x87 product) truncated by `_ftol2_sse` (`00ec62c0`) and divided back into 100.0,
/// as a double (block of `strlen(name) + 0x1e` bytes).
pub fn ni_sphere_bv_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiSphereBV>,
    name: Ptr,
    strings: Ptr,
) {
    let length = c_string_length(e, name.addr());
    let size = length.wrapping_add(0xd);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, SPHERE_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
    let center = e
        .call(
            NI_POINT3_GET_VIEWER_STRING,
            &args![this.addr().wrapping_add(4), CENTER_NAME],
        )
        .u32();
    add_viewer_string(e, strings, center);
    let size = c_string_length(e, name.addr()).wrapping_add(0x1e);
    let line = e.call(ALLOCATE, &args![size]).u32();
    let scale = e.mem.f64(RADIUS_SCALE);
    let radius = load_float(e, this.addr().wrapping_add(0x10));
    let scaled = x87_mul(radius, scale);
    let truncated = e.call(FTOL, &args![scaled]).i32();
    let value = x87_div(truncated as f64, scale);
    e.call(NI_SPRINTF, &args![line, size, RADIUS_FORMAT, value]);
    add_viewer_string(e, strings, line);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eab820,
            ni_sphere_bv_operator_(Ptr<NiSphereBV>, Ptr<NiSphereBV>) -> bool
        ),
        entry!(0x00eab860, ni_sphere_bv_load_binary(Ptr<NiSphereBV>, Ptr)),
        entry!(0x00eab880, ni_sphere_bv_save_binary(Ptr<NiSphereBV>, Ptr)),
        entry!(
            0x00eab8a0,
            fn_00eab8a0(Ptr<NiSphereBV>, f32, Ptr) -> Ptr<NiSphereBV>
        ),
        entry!(
            0x00eab8d0,
            ni_sphere_bv_copy(Ptr<NiSphereBV>, Ptr<NiSphereBV>)
        ),
        entry!(
            0x00eab900,
            ni_sphere_bv_update_world_data(Ptr<NiSphereBV>, Ptr<NiSphereBV>, Ptr)
        ),
        entry!(0x00eac200, ni_sphere_bv_create() -> Ptr<NiSphereBV>),
        entry!(
            0x00eac240,
            ni_sphere_bv_sphere_tri_test_intersect(
                f32,
                Ptr<NiSphereBV>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00eac3f0,
            fn_00eac3f0(
                f32,
                Ptr<NiSphereBV>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                bool,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00eacac0,
            ni_sphere_bv_clone(Ptr<NiSphereBV>) -> Ptr<NiSphereBV>
        ),
        entry!(0x00eacb00, fn_00eacb00(Ptr) -> Ptr<NiSphereBV>),
        entry!(
            0x00eacbe0,
            ni_sphere_bv_add_viewer_strings(Ptr<NiSphereBV>, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nicollision::nicollisiondata::{VIEWER_STRINGS_GROW, VIEWER_STRINGS_SET_AT};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// The argument words of every logged call to `addr`.
    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the data pages the unit reads mapped (`01000000..01200000`), the constants
    /// set as in the executable and the memory manager's allocation answering fresh blocks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.mem.set_f32(APPROACH_THRESHOLD, 0.0);
        e.mem.set_f64(PLANE_MARGIN, 0.001f32 as f64);
        e.mem.set_f64(MINIMUM_MOTION, 1.0e-5);
        e.mem.set_f32(PSEUDODISTANCE_THRESHOLD, 0.0001);
        e.mem.set_f32(DERIVATIVE_TIME_STEP, 0.001);
        e.mem.set_f64(RADIUS_SCALE, 100.0);
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn point3(e: &mut Engine, values: [f32; 3]) -> u32 {
        let address = e.mem.alloc(12);
        for (axis, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * axis as u32, *value);
        }
        address
    }

    fn floats(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    /// A sphere volume at a fresh block.
    fn sphere(e: &mut Engine, center: [f32; 3], radius: f32) -> Ptr<NiSphereBV> {
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
        for (axis, value) in center.iter().enumerate() {
            e.mem.set_f32(block + 4 + 4 * axis as u32, *value);
        }
        e.mem.set_f32(block + 0x10, radius);
        Ptr::new(block)
    }

    #[test]
    fn operator_equal_asks_the_other_volume_for_its_type_and_compares_spheres() {
        let mut e = engine();
        e.register(0x0300_0001, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        // Called on the other volume's sphere (+4): the answer word sits at volume + 0x44.
        e.register(NI_SPHERE_EQUAL, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.map(0x0200_0000, 0x200);
        e.mem.set_u32(0x0200_0000 + 0xc, 0x0300_0001);
        let this = sphere(&mut e, [0.0; 3], 1.0);
        let other = e.mem.alloc(0x100);
        e.mem.set_u32(other, 0x0200_0000);
        // Type 0 and equal spheres.
        e.mem.set_u32(other + 0x44, 1);
        assert!(e.call(0x00ea_b820, &args![this, other]).bool());
        assert_eq!(
            logged(&e, NI_SPHERE_EQUAL),
            vec![vec![other + 4, this.addr() + 4]]
        );
        // Type 0, different spheres.
        e.mem.set_u32(other + 0x44, 0);
        assert!(!e.call(0x00ea_b820, &args![this, other]).bool());
        // Another type: the spheres are not compared.
        e.mem.set_u32(other + 0x40, 2);
        e.mem.set_u32(other + 0x44, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ea_b820, &args![this, other]).bool());
        assert!(logged(&e, NI_SPHERE_EQUAL).is_empty());
    }

    #[test]
    fn load_and_save_binary_pass_the_stream_to_the_base_and_the_sphere() {
        let mut e = engine();
        for address in [
            NI_BOUNDING_VOLUME_LOAD_BINARY,
            NI_SPHERE_LOAD_BINARY,
            NI_BOUNDING_VOLUME_SAVE_BINARY,
            NI_SPHERE_SAVE_BINARY,
        ] {
            e.register(address, nothing);
        }
        e.call(0x00ea_b860, &args![0x5000u32, 0x6000u32]);
        e.call(0x00ea_b880, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_LOAD_BINARY),
            vec![vec![0x5000, 0x6000]]
        );
        assert_eq!(
            logged(&e, NI_SPHERE_LOAD_BINARY),
            vec![vec![0x5004, 0x6000]]
        );
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_SAVE_BINARY),
            vec![vec![0x5000, 0x6000]]
        );
        assert_eq!(
            logged(&e, NI_SPHERE_SAVE_BINARY),
            vec![vec![0x5004, 0x6000]]
        );
    }

    #[test]
    fn fn_00eab8a0_builds_the_volume_from_a_radius_and_a_center() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let center = point3(&mut e, [1.0, 2.0, 3.0]);
        assert_eq!(
            e.call(0x00ea_b8a0, &args![this, 2.5f32, center]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x010f_1854);
        assert_eq!(floats(&e, this + 4, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(e.mem.f32(this + 0x10), 2.5);
        // A signalling NaN radius is quieted by the FLD/FSTP pair.
        e.call(
            0x00ea_b8a0,
            &args![this, f32::from_bits(0x7f80_0001), center],
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
    }

    #[test]
    fn copy_copies_the_four_sphere_words() {
        let mut e = engine();
        let this = sphere(&mut e, [0.0; 3], 0.0);
        let other = sphere(&mut e, [1.0, 2.0, 3.0], 4.0);
        e.call(0x00ea_b8d0, &args![this, other]);
        assert_eq!(floats(&e, this.addr() + 4, 4), vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(this.addr()), 0x010f_1854);
    }

    /// A transform: the rotation rows, translation and scale.
    fn transform(e: &mut Engine, rotation: [f32; 9], translation: [f32; 3], scale: f32) -> u32 {
        let address = e.mem.alloc(0x34);
        for (index, value) in rotation.iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, *value);
        }
        for (index, value) in translation.iter().enumerate() {
            e.mem.set_f32(address + 0x24 + 4 * index as u32, *value);
        }
        e.mem.set_f32(address + 0x30, scale);
        address
    }

    #[test]
    fn update_world_data_rotates_scales_and_translates_the_model_sphere() {
        let mut e = engine();
        let this = sphere(&mut e, [0.0; 3], 0.0);
        let model = sphere(&mut e, [1.0, 2.0, 3.0], 0.5);
        let t = transform(
            &mut e,
            [0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            [10.0, 20.0, 30.0],
            2.0,
        );
        e.call(0x00ea_b900, &args![this, model, t]);
        assert_eq!(floats(&e, this.addr() + 4, 4), vec![14.0, 18.0, 36.0, 1.0]);
    }

    #[test]
    fn update_world_data_keeps_a_signalling_scale_quiet() {
        let mut e = engine();
        let this = sphere(&mut e, [0.0; 3], 0.0);
        let model = sphere(&mut e, [1.0, 2.0, 3.0], 0.5);
        let t = transform(&mut e, [1.0; 9], [0.0; 3], 1.0);
        e.mem.set_u32(t + 0x30, 0x7f80_0001);
        e.call(0x00ea_b900, &args![this, model, t]);
        for offset in [4, 8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0x7fc0_0001, "+{offset:#x}");
        }
    }

    #[test]
    fn create_makes_a_unit_sphere_at_the_default_center() {
        let mut e = engine();
        for (axis, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem
                .set_f32(DEFAULT_CENTER_011F426C + 4 * axis as u32, *value);
        }
        let made = e.call(0x00ea_c200, &args![]).u32();
        assert_eq!(
            logged(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(e.mem.u32(made), 0x010f_1854);
        assert_eq!(floats(&e, made + 4, 4), vec![1.0, 2.0, 3.0, 1.0]);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ea_c200, &args![]).u32(), 0);
    }

    #[test]
    fn clone_copies_the_sphere_into_a_new_block() {
        let mut e = engine();
        let this = sphere(&mut e, [1.0, 2.0, 3.0], 4.0);
        let made = e.call(0x00ea_cac0, &args![this]).u32();
        assert_ne!(made, this.addr());
        assert_eq!(e.mem.u32(made), 0x010f_1854);
        assert_eq!(floats(&e, made + 4, 4), vec![1.0, 2.0, 3.0, 4.0]);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ea_cac0, &args![this]).u32(), 0);
    }

    /// The doubles of the point-triangle and segment-triangle distances: they answer the shared
    /// `distance` and note the triangle and the segment they were given.
    struct Distances {
        distance: Rc<Cell<f64>>,
        triangle: Rc<RefCell<Vec<f32>>>,
        segment: Rc<RefCell<Vec<f32>>>,
    }

    fn distances(e: &mut Engine, distance: f64) -> Distances {
        let d = Distances {
            distance: Rc::new(Cell::new(distance)),
            triangle: Rc::new(RefCell::new(vec![])),
            segment: Rc::new(RefCell::new(vec![])),
        };
        let (shared, triangle) = (d.distance.clone(), d.triangle.clone());
        e.register_double(SQR_DISTANCE_POINT_TRIANGLE, move |e, a| {
            *triangle.borrow_mut() = (0..9).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            e.mem.set_f32(a[2], 0.25);
            e.mem.set_f32(a[3], 0.5);
            Ret {
                st0: shared.get(),
                ..Ret::default()
            }
        });
        let (shared, triangle, segment) =
            (d.distance.clone(), d.triangle.clone(), d.segment.clone());
        e.register_double(SQR_DISTANCE_SEGMENT_TRIANGLE, move |e, a| {
            *segment.borrow_mut() = (0..6).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            *triangle.borrow_mut() = (0..9).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            Ret {
                st0: shared.get(),
                ..Ret::default()
            }
        });
        d
    }

    /// A triangle (1, 2, 3), (2, 2, 3), (1, 4, 3) and its words, the sphere at the origin.
    fn triangle_scene(e: &mut Engine, radius: f32) -> (Ptr<NiSphereBV>, u32, u32, u32) {
        let sphere = sphere(e, [7.0, 8.0, 9.0], radius);
        let first = point3(e, [1.0, 2.0, 3.0]);
        let second = point3(e, [2.0, 2.0, 3.0]);
        let third = point3(e, [1.0, 4.0, 3.0]);
        (sphere, first, second, third)
    }

    fn test_intersect(
        e: &mut Engine,
        time: f32,
        scene: (Ptr<NiSphereBV>, u32, u32, u32),
        sphere_velocity: [f32; 3],
        triangle_velocity: [f32; 3],
    ) -> bool {
        let va = point3(e, sphere_velocity);
        let vb = point3(e, triangle_velocity);
        e.call(
            0x00ea_c240,
            &args![time, scene.0, va, scene.1, scene.2, scene.3, vb],
        )
        .bool()
    }

    #[test]
    fn sphere_tri_test_uses_the_point_distance_when_nothing_moves_relative_to_the_triangle() {
        let mut e = engine();
        let d = distances(&mut e, 4.0);
        let scene = triangle_scene(&mut e, 2.0);
        assert!(test_intersect(
            &mut e,
            5.0,
            scene,
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0]
        ));
        let calls = logged(&e, SQR_DISTANCE_POINT_TRIANGLE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], scene.0.addr() + 4);
        assert!(logged(&e, SQR_DISTANCE_SEGMENT_TRIANGLE).is_empty());
        // The triangle is the first vertex and the two edges.
        assert_eq!(
            *d.triangle.borrow(),
            vec![1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0]
        );
    }

    #[test]
    fn sphere_tri_test_compares_the_rounded_distance_with_the_squared_radius() {
        let mut e = engine();
        let d = distances(&mut e, 4.0);
        let scene = triangle_scene(&mut e, 2.0);
        assert!(test_intersect(&mut e, 1.0, scene, [0.0; 3], [0.0; 3]));
        d.distance.set(4.000_000_1);
        // 4.0000001 rounds to 4.0 as a float: still at most the squared radius.
        assert!(test_intersect(&mut e, 1.0, scene, [0.0; 3], [0.0; 3]));
        d.distance.set(4.001);
        assert!(!test_intersect(&mut e, 1.0, scene, [0.0; 3], [0.0; 3]));
        d.distance.set(f64::NAN);
        assert!(!test_intersect(&mut e, 1.0, scene, [0.0; 3], [0.0; 3]));
    }

    #[test]
    fn sphere_tri_test_uses_the_segment_distance_when_the_relative_velocity_is_not_zero() {
        let mut e = engine();
        let d = distances(&mut e, 1.0);
        let scene = triangle_scene(&mut e, 2.0);
        assert!(test_intersect(
            &mut e,
            2.0,
            scene,
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0]
        ));
        assert!(logged(&e, SQR_DISTANCE_POINT_TRIANGLE).is_empty());
        // The segment starts at the center and goes along time * (relative velocity).
        assert_eq!(*d.segment.borrow(), vec![7.0, 8.0, 9.0, 2.0, 4.0, 4.0]);
        d.distance.set(5.0);
        assert!(!test_intersect(
            &mut e,
            2.0,
            scene,
            [0.0, 1.0, 0.0],
            [0.0; 3]
        ));
        // Each of the three components decides alone.
        for velocity in [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]] {
            e.call_log = Some(vec![]);
            test_intersect(&mut e, 1.0, scene, velocity, [0.0; 3]);
            assert_eq!(logged(&e, SQR_DISTANCE_SEGMENT_TRIANGLE).len(), 1);
        }
    }

    #[test]
    fn sphere_tri_test_treats_a_nan_velocity_difference_as_motion() {
        let mut e = engine();
        let _ = distances(&mut e, 1.0);
        let scene = triangle_scene(&mut e, 2.0);
        test_intersect(&mut e, 1.0, scene, [f32::NAN, 0.0, 0.0], [0.0; 3]);
        assert_eq!(logged(&e, SQR_DISTANCE_SEGMENT_TRIANGLE).len(), 1);
    }

    /// The scene of the `Find` test: a sphere, the triangle (0, 0, 0), (10, 0, 0), (0, 10, 0),
    /// the two velocities, the four out blocks and the doubles of everything outside the
    /// function (point distances, unitize, scale, dot, triangle normal and the intersector).
    struct Find {
        e: Engine,
        sphere: Ptr<NiSphereBV>,
        triangle: [u32; 3],
        velocity_a: u32,
        velocity_b: u32,
        out_time: u32,
        out_point: u32,
        out_normal: u32,
        out_negated: u32,
        distance: Rc<Cell<f64>>,
        kind: Rc<Cell<u32>>,
    }

    impl Find {
        fn new(center: [f32; 3], radius: f32, distance: f64) -> Find {
            let mut e = engine();
            let d = distances(&mut e, distance);
            e.register(UNITIZE, |e, a| {
                let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
                let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                for (axis, value) in v.iter().enumerate() {
                    let scaled = if length <= 1.0e-6 {
                        0.0
                    } else {
                        value / length
                    };
                    e.mem.set_f32(a[0] + 4 * axis as u32, scaled);
                }
                Ret::default()
            });
            e.register(SCALE_POINT, |e, a| {
                for axis in 0..3 {
                    let value = f32::from_bits(a[1]) * e.mem.f32(a[2] + 4 * axis);
                    e.mem.set_f32(a[0] + 4 * axis, value);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            e.register(NI_POINT3_DOT, |e, a| {
                let sum: f32 = (0..3)
                    .map(|i| e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i))
                    .sum();
                Ret {
                    st0: sum as f64,
                    ..Ret::default()
                }
            });
            e.register(NI_TRIGON_GET_NORMAL, |e, a| {
                e.mem.set_f32(a[1], 0.0);
                e.mem.set_f32(a[1] + 4, 0.0);
                e.mem.set_f32(a[1] + 8, 1.0);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            let contact_point = point3(&mut e, [1.0, 2.0, 3.0]);
            let contact_normal = point3(&mut e, [0.0, 1.0, 0.0]);
            let kind = Rc::new(Cell::new(3));
            let shared_kind = kind.clone();
            e.register(SPHERE_TRI_INTERSECTOR_CONSTRUCTOR, nothing);
            e.register(INTERSECTOR_FIND, nothing);
            e.register(INTERSECTOR_CONTACT_TIME, |_, _| Ret {
                st0: 0.25,
                ..Ret::default()
            });
            e.register_double(INTERSECTOR_TYPE, move |_, _| Ret {
                eax: shared_kind.get(),
                ..Ret::default()
            });
            e.register_double(INTERSECTOR_CONTACT_POINT, move |_, _| Ret {
                eax: contact_point,
                ..Ret::default()
            });
            e.register_double(INTERSECTOR_CONTACT_NORMAL, move |_, _| Ret {
                eax: contact_normal,
                ..Ret::default()
            });
            e.register(INTERSECTOR_DESTRUCTOR, nothing);
            let sphere = sphere(&mut e, center, radius);
            let triangle = [
                point3(&mut e, [0.0, 0.0, 0.0]),
                point3(&mut e, [10.0, 0.0, 0.0]),
                point3(&mut e, [0.0, 10.0, 0.0]),
            ];
            let (velocity_a, velocity_b) = (point3(&mut e, [0.0; 3]), point3(&mut e, [0.0; 3]));
            let out_time = e.mem.alloc(4);
            e.mem.set_f32(out_time, 99.0);
            let out_point = e.mem.alloc(12);
            let out_normal = e.mem.alloc(12);
            let out_negated = e.mem.alloc(12);
            Find {
                e,
                sphere,
                triangle,
                velocity_a,
                velocity_b,
                out_time,
                out_point,
                out_normal,
                out_negated,
                distance: d.distance,
                kind,
            }
        }

        fn set_velocity(&mut self, a: [f32; 3], b: [f32; 3]) {
            for axis in 0..3u32 {
                self.e
                    .mem
                    .set_f32(self.velocity_a + 4 * axis, a[axis as usize]);
                self.e
                    .mem
                    .set_f32(self.velocity_b + 4 * axis, b[axis as usize]);
            }
        }

        fn run(&mut self, time: f32, want_normals: bool) -> bool {
            self.e
                .call(
                    0x00ea_c3f0,
                    &args![
                        time,
                        self.sphere,
                        self.velocity_a,
                        self.triangle[0],
                        self.triangle[1],
                        self.triangle[2],
                        self.velocity_b,
                        self.out_time,
                        self.out_point,
                        want_normals,
                        self.out_normal,
                        self.out_negated
                    ],
                )
                .bool()
        }

        fn intersector_built(&self) -> bool {
            !logged(&self.e, SPHERE_TRI_INTERSECTOR_CONSTRUCTOR).is_empty()
        }
    }

    #[test]
    fn find_reports_a_sphere_that_already_touches_the_triangle() {
        let mut f = Find::new([2.5, 5.0, 1.0], 2.0, 1.0);
        assert!(f.run(1.0, true));
        let e = &f.e;
        assert_eq!(e.mem.f32(f.out_time), 0.0);
        // The closest point: the first vertex plus 0.25 of the first edge and 0.5 of the second.
        assert_eq!(floats(e, f.out_point, 3), vec![2.5, 5.0, 0.0]);
        // The triangle's normal goes to the last word, its negation to the one before.
        assert_eq!(floats(e, f.out_negated, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(floats(e, f.out_normal, 3), vec![-0.0, -0.0, -1.0]);
        assert_eq!(e.mem.u32(f.out_normal), 0x8000_0000);
        assert!(!f.intersector_built());
        let calls = logged(e, SQR_DISTANCE_POINT_TRIANGLE);
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0][0], f.sphere.addr() + 4);
    }

    #[test]
    fn find_leaves_the_normals_alone_when_they_are_not_wanted() {
        let mut f = Find::new([2.5, 5.0, 1.0], 2.0, 4.0);
        assert!(f.run(1.0, false));
        assert_eq!(f.e.mem.f32(f.out_time), 0.0);
        assert!(logged(&f.e, NI_TRIGON_GET_NORMAL).is_empty());
        assert_eq!(f.e.mem.u32(f.out_normal), 0);
    }

    #[test]
    fn find_with_a_nan_distance_goes_on_to_the_motion_test() {
        let mut f = Find::new([2.5, 5.0, 1.0], 2.0, f64::NAN);
        assert!(!f.run(1.0, true));
        assert_eq!(f.e.mem.f32(f.out_time), 99.0);
        assert!(!f.intersector_built());
    }

    #[test]
    fn find_gives_up_when_the_relative_motion_is_negligible() {
        let mut f = Find::new([5.0, 5.0, 3.0], 1.0, 9.0);
        f.set_velocity([0.5, 0.5, 0.5], [0.5, 0.5, 0.5]);
        assert!(!f.run(10.0, true));
        f.set_velocity([0.0, 0.0, 1.0], [0.0; 3]);
        // 1 * 0.000001 is below the minimum.
        assert!(!f.run(0.000_001, true));
        assert!(!f.intersector_built());
    }

    #[test]
    fn find_gives_up_when_a_sphere_below_the_plane_cannot_reach_it() {
        // Below the plane (the side of the unit normal `u = (0, 0, -1)`), moving away from it.
        let mut f = Find::new([5.0, 5.0, -3.0], 1.0, 9.0);
        f.set_velocity([0.0, 0.0, 1.0], [0.0; 3]);
        assert!(!f.run(0.5, true));
        assert!(!f.intersector_built());
        assert_eq!(logged(&f.e, UNITIZE).len(), 1);
        // The contact point `(5, 5, -2)` moved by `0.5 * velocity` is checked with `Dot`.
        assert_eq!(logged(&f.e, NI_POINT3_DOT).len(), 1);
    }

    #[test]
    fn find_runs_the_general_test_when_the_sphere_can_reach_the_plane() {
        // Below the plane and moving toward it: in time the moved contact point is not clear.
        let mut f = Find::new([5.0, 5.0, -3.0], 1.0, 9.0);
        f.set_velocity([0.0, 0.0, 1.0], [0.0; 3]);
        assert!(f.run(10.0, true));
        let built = logged(&f.e, SPHERE_TRI_INTERSECTOR_CONSTRUCTOR);
        assert_eq!(built.len(), 1);
        let tail = &built[0][1..];
        assert_eq!(tail[0], f.sphere.addr());
        assert_eq!(tail[1..4], f.triangle);
        assert_eq!(tail[4], 10.0f32.to_bits());
        assert_eq!(tail[5], 0.001f32.to_bits());
        assert_eq!(tail[6], 0.0001f32.to_bits());
        assert_eq!(tail[7], 0x20);
        assert_eq!(
            logged(&f.e, INTERSECTOR_FIND),
            vec![vec![built[0][0], f.velocity_a, f.velocity_b]]
        );
        let e = &f.e;
        assert_eq!(e.mem.f32(f.out_time), 0.25);
        assert_eq!(floats(e, f.out_point, 3), vec![1.0, 2.0, 3.0]);
        // The contact normal goes to the first normal block, its negation to the second.
        assert_eq!(floats(e, f.out_normal, 3), vec![0.0, 1.0, 0.0]);
        assert_eq!(floats(e, f.out_negated, 3), vec![-0.0, -1.0, -0.0]);
        assert_eq!(logged(e, INTERSECTOR_DESTRUCTOR), vec![vec![built[0][0]]]);
    }

    #[test]
    fn find_approaching_from_above_the_plane_runs_the_general_test() {
        let mut f = Find::new([5.0, 5.0, 3.0], 1.0, 9.0);
        f.set_velocity([0.0, 0.0, -1.0], [0.0; 3]);
        assert!(f.run(10.0, false));
        assert!(f.intersector_built());
        assert!(logged(&f.e, INTERSECTOR_CONTACT_NORMAL).is_empty());
        assert_eq!(f.e.mem.u32(f.out_normal), 0);
    }

    #[test]
    fn find_gives_up_when_the_sphere_moves_away_from_the_plane() {
        // Above the plane and moving away (`u . velocity` is negative).
        let mut f = Find::new([5.0, 5.0, 3.0], 1.0, 9.0);
        f.set_velocity([0.0, 0.0, 1.0], [0.0; 3]);
        assert!(!f.run(10.0, true));
        // Below the plane and moving away (`u . velocity` is positive).
        let mut f = Find::new([5.0, 5.0, -3.0], 1.0, 9.0);
        f.set_velocity([0.0, 0.0, -1.0], [0.0; 3]);
        assert!(!f.run(10.0, true));
        assert!(!f.intersector_built());
    }

    #[test]
    fn find_runs_the_general_test_when_the_sphere_touches_the_plane_but_not_the_triangle() {
        let mut f = Find::new([-5.0, -5.0, 0.5], 1.0, 50.25);
        f.set_velocity([1.0, 1.0, 0.0], [0.0; 3]);
        assert!(f.run(10.0, true));
        assert!(logged(&f.e, UNITIZE).is_empty());
        assert!(f.intersector_built());
    }

    #[test]
    fn find_accepts_intersection_types_2_and_3_only() {
        for (kind, expected) in [(0u32, false), (1, false), (2, true), (3, true), (4, false)] {
            let mut f = Find::new([-5.0, -5.0, 0.5], 1.0, 50.25);
            f.set_velocity([1.0, 1.0, 0.0], [0.0; 3]);
            f.kind.set(kind);
            assert_eq!(f.run(10.0, true), expected, "type {kind}");
            // The contact time is stored whatever the type, the intersector destroyed.
            assert_eq!(f.e.mem.f32(f.out_time), 0.25);
            assert_eq!(logged(&f.e, INTERSECTOR_DESTRUCTOR).len(), 1);
            assert_eq!(f.e.mem.u32(f.out_point) != 0, expected);
        }
    }

    #[test]
    fn find_sets_the_threshold_comparisons_from_the_distance() {
        // A distance just above the squared radius is not "touching".
        let mut f = Find::new([-5.0, -5.0, 0.5], 1.0, 1.0001);
        f.set_velocity([1.0, 1.0, 0.0], [0.0; 3]);
        f.run(10.0, true);
        assert!(f.intersector_built());
        f.distance.set(1.0);
        let mut g = Find::new([-5.0, -5.0, 0.5], 1.0, 1.0);
        assert!(g.run(10.0, true));
        assert!(!g.intersector_built());
    }

    /// A stream whose read function (+8 of the input object at +0x24c) is the double at
    /// `03000010`, handing out `values`, each written as `size` bytes.
    fn stream_with_reads(e: &mut Engine, values: Vec<u32>) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(io + 8, 0x0300_0010);
        let mut queue = std::collections::VecDeque::from(values);
        e.register_double(0x0300_0010, move |e, a| {
            let value = queue.pop_front().unwrap();
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        (stream, io)
    }

    /// `LOAD_THREE_FLOATS` answers (1.5, 2.5, 3.5) for the first block and (9, 9, 9) for the others.
    fn three_floats(e: &mut Engine) {
        let calls = Rc::new(Cell::new(0));
        e.register_double(LOAD_THREE_FLOATS, move |e, a| {
            let value = if calls.get() == 0 {
                [1.5f32, 2.5, 3.5]
            } else {
                [9.0; 3]
            };
            calls.set(calls.get() + 1);
            for (axis, word) in value.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * axis as u32, *word);
            }
            Ret::default()
        });
    }

    #[test]
    fn fn_00eacb00_builds_a_sphere_from_the_first_point_and_the_first_float() {
        let mut e = engine();
        three_floats(&mut e);
        let (stream, io) = stream_with_reads(
            &mut e,
            vec![4.0f32.to_bits(), 5.0f32.to_bits(), 6.0f32.to_bits()],
        );
        let made = e.call(0x00ea_cb00, &args![stream]).u32();
        assert_eq!(logged(&e, LOAD_THREE_FLOATS).len(), 3);
        assert!(logged(&e, LOAD_THREE_FLOATS)
            .iter()
            .all(|call| call[1] == stream));
        assert_eq!(e.mem.u32(made), 0x010f_1854);
        assert_eq!(floats(&e, made + 4, 4), vec![1.5, 2.5, 3.5, 4.0]);
        // Three 4-byte reads moved the counter by 12.
        assert_eq!(e.mem.u32(io + 4), 12);
        assert_eq!(logged(&e, 0x0300_0010).len(), 3);
        assert_eq!(
            logged(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
    }

    #[test]
    fn fn_00eacb00_quiets_a_signalling_radius_and_reads_on_after_a_failed_allocation() {
        let mut e = engine();
        three_floats(&mut e);
        let (stream, _) = stream_with_reads(&mut e, vec![0x7f80_0001, 0, 0]);
        let made = e.call(0x00ea_cb00, &args![stream]).u32();
        assert_eq!(e.mem.u32(made + 0x10), 0x7fc0_0001);
        let mut e = engine();
        three_floats(&mut e);
        let (stream, io) = stream_with_reads(&mut e, vec![0, 0, 0]);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ea_cb00, &args![stream]).u32(), 0);
        assert_eq!(e.mem.u32(io + 4), 12);
    }

    #[test]
    fn add_viewer_strings_appends_the_class_line_the_center_and_the_radius() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_SPRINTF, nothing);
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x6001,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x0600_0000 + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.map(0x0600_0000, 0x1000);
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, 8);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"m_bv");
        let this = sphere(&mut e, [1.0, 2.0, 3.0], 0.123_456);
        e.call(0x00ea_cbe0, &args![this, name, strings]);
        let lines = logged(&e, NI_SPRINTF);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0][1..], [4 + 0xd, SPHERE_BV_FORMAT, name]);
        // The radius times 100 is truncated (12), then divided by 100 as a double.
        assert_eq!(lines[1][1..3], [4 + 0x1e, RADIUS_FORMAT]);
        let value = f64::from_bits(lines[1][3] as u64 | (lines[1][4] as u64) << 32);
        assert_eq!(value, 0.12);
        assert_eq!(
            logged(&e, NI_POINT3_GET_VIEWER_STRING),
            vec![vec![this.addr() + 4, CENTER_NAME]]
        );
        assert_eq!(e.mem.u32(0x0600_0000), lines[0][0]);
        assert_eq!(e.mem.u32(0x0600_0004), 0x6001);
        assert_eq!(e.mem.u32(0x0600_0008), lines[1][0]);
        // FTOL was given the x87 product: 0.123456 (as a float) times 100.
        let scaled = logged(&e, FTOL);
        assert_eq!(scaled.len(), 1);
        let product = f64::from_bits(scaled[0][0] as u64 | (scaled[0][1] as u64) << 32);
        assert_eq!(product, 0.123_456f32 as f64 * 100.0);
    }
}
