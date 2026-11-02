//! `NiCollision/nicapsulebv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, float_operand, load_float, stream_transfer,
};
use crate::units::nicollision::nispherebv::negated_bits;

layout! {
    /// `NiCapsuleBV` (Xbox PDB), 0x3c bytes: the vtable, `m_kCenter`, `m_kAxis`, `m_fExtent` and
    /// `m_kCapsule` (an `NiCapsule`: the segment origin and direction, then the radius), declared
    /// as words because most of the code only moves them.
    pub struct NiCapsuleBV: 0x3c {
        /// `m_kCenter` (Xbox PDB), x.
        0x04 m_kCenter_x: u32,
        /// `m_kCenter` (Xbox PDB), y.
        0x08 m_kCenter_y: u32,
        /// `m_kCenter` (Xbox PDB), z.
        0x0C m_kCenter_z: u32,
        /// `m_kAxis` (Xbox PDB), x.
        0x10 m_kAxis_x: u32,
        /// `m_kAxis` (Xbox PDB), y.
        0x14 m_kAxis_y: u32,
        /// `m_kAxis` (Xbox PDB), z.
        0x18 m_kAxis_z: u32,
        /// `m_fExtent` (Xbox PDB).
        0x1C m_fExtent: u32,
        /// `m_kCapsule.m_kSegment.m_kOrigin` (Xbox PDB), x.
        0x20 m_kCapsule_origin_x: u32,
        /// `m_kCapsule.m_kSegment.m_kOrigin` (Xbox PDB), y.
        0x24 m_kCapsule_origin_y: u32,
        /// `m_kCapsule.m_kSegment.m_kOrigin` (Xbox PDB), z.
        0x28 m_kCapsule_origin_z: u32,
        /// `m_kCapsule.m_kSegment.m_kDirection` (Xbox PDB), x.
        0x2C m_kCapsule_direction_x: u32,
        /// `m_kCapsule.m_kSegment.m_kDirection` (Xbox PDB), y.
        0x30 m_kCapsule_direction_y: u32,
        /// `m_kCapsule.m_kSegment.m_kDirection` (Xbox PDB), z.
        0x34 m_kCapsule_direction_z: u32,
        /// `m_kCapsule.m_fRadius` (Xbox PDB).
        0x38 m_kCapsule_radius: u32,
    }
}

/// `NiCapsuleBV`'s vtable (`010f17d0`).
const NI_CAPSULE_BV_VTABLE: u32 = 0x010f_17d0;
/// The memory manager instance (`this` of its allocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator `00aa1070` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = CAPSULE_BV"`.
const CAPSULE_BV_FORMAT: u32 = 0x010f_1800;
/// The three words at `011f426c` (zero in the image): the default center and the "no motion"
/// velocity the tests compare with.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
/// The three words at `011a946c` (1, 0, 0 in the image): the default axis.
const DEFAULT_AXIS_011A946C: u32 = 0x011a_946c;
/// The double `2.0` (`01011590`) the segment direction is scaled by.
const DIRECTION_SCALE: u32 = 0x0101_1590;
/// `NiCapsule::operator==` (`00eae690`, thiscall on one capsule, the other capsule).
const NI_CAPSULE_EQUAL: u32 = 0x00ea_e690;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`, thiscall, the stream).
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
/// `NiBoundingVolume::SaveBinary` (`00e9cb10`, thiscall, the stream).
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// `NiColor::LoadBinary` / `NiColor::SaveBinary` (`00a59be0` / `00a69780`, thiscall on a
/// three-float block, the stream): here the center and the axis.
const LOAD_THREE_FLOATS: u32 = 0x00a5_9be0;
const SAVE_THREE_FLOATS: u32 = 0x00a6_9780;
/// `NiSqrDistance::Compute` (`00eaf1f0`, cdecl: point, segment, out float), in ST0.
const SQR_DISTANCE_POINT_SEGMENT: u32 = 0x00ea_f1f0;
/// `NiSqrDistance::Compute_ov2` (`00eaf310`, cdecl: segment, segment, two out floats), in ST0.
const SQR_DISTANCE_SEGMENT_SEGMENT: u32 = 0x00ea_f310;
/// Unnamed in the engine map (`00eaff20`, cdecl: a segment, a block of a segment and its
/// displacement, three out floats), in ST0.
const SQR_DISTANCE_SEGMENT_MOVING_SEGMENT: u32 = 0x00ea_ff20;
/// `NiSqrDistance::Compute_ov3` (`00eb1d20`, cdecl: segment, triangle, three out floats), in ST0.
const SQR_DISTANCE_SEGMENT_TRIANGLE: u32 = 0x00eb_1d20;
/// Unnamed in the engine map (`00eb3230`, cdecl: a triangle, a block of a segment and its
/// displacement, four out floats), in ST0.
const SQR_DISTANCE_TRIANGLE_MOVING_SEGMENT: u32 = 0x00eb_3230;
/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;
/// Unnamed in the engine map: `(out, scalar, vector)` (cdecl), writes the vector scaled by the
/// scalar to `out` and answers `out`.
const SCALE_POINT: u32 = 0x004a_3760;
/// Unnamed in the engine map (`00439ef0` and `00439e90`, thiscall on a point: an out point and
/// another point; each answers the out point): the two point combinations the triangle find
/// uses when the capsule approaches the plane from either side (which is which is not
/// confirmed here).
const POINT_COMBINE_00439EF0: u32 = 0x0043_9ef0;
const POINT_COMBINE_00439E90: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one point, the other point), in ST0.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// `NiTrigon::GetNormal` (`00eb3830`, thiscall on the triangle, the out point).
const NI_TRIGON_GET_NORMAL: u32 = 0x00eb_3830;
/// The `float` `01011d78` (0.0 in the executable) the approach speed is compared with.
const APPROACH_THRESHOLD: u32 = 0x0101_1d78;
/// The double `0.001` (`01016978`) added to the plane constant.
const PLANE_MARGIN: u32 = 0x0101_6978;
/// The double (about `1e-5`, `0101dae8`) the squared relative speed times the time must exceed.
const MINIMUM_MOTION: u32 = 0x0101_dae8;
/// The `float`s `0.0001` (`010141cc`) and `0.001` (`01017d00`) handed to the intersectors as the
/// pseudodistance threshold and the derivative time step.
const PSEUDODISTANCE_THRESHOLD: u32 = 0x0101_41cc;
const DERIVATIVE_TIME_STEP: u32 = 0x0101_7d00;
/// The intersector constructors: `00eae9e0` and `00eaedc0` (thiscall: two volumes, then the
/// maximum time, the derivative time step, the pseudodistance threshold and the iteration
/// limit) for the capsule finds of 0x48 and 0x4c stack bytes, `00eb3750` (thiscall: the capsule,
/// the three triangle vertices, then the same four words) for the triangle find (0x6c bytes).
const INTERSECTOR_CONSTRUCTOR_00EAE9E0: u32 = 0x00ea_e9e0;
const INTERSECTOR_CONSTRUCTOR_00EAEDC0: u32 = 0x00ea_edc0;
const CAPSULE_TRI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00eb_3750;
/// `NiIntersector::Find` (`00eae7a0`, thiscall, the two velocities).
const INTERSECTOR_FIND: u32 = 0x00ea_e7a0;
/// The intersector's contact time getter (`00eae6f0`, in ST0), contact point getter
/// (`00eae700`), contact normal getter (`00eae710`), intersection type getter (`00be0aa0`) and
/// destructor (`00eae6d0`), all thiscall on the intersector.
const INTERSECTOR_CONTACT_TIME: u32 = 0x00ea_e6f0;
const INTERSECTOR_CONTACT_POINT: u32 = 0x00ea_e700;
const INTERSECTOR_CONTACT_NORMAL: u32 = 0x00ea_e710;
const INTERSECTOR_TYPE: u32 = 0x00be_0aa0;
const INTERSECTOR_DESTRUCTOR: u32 = 0x00ea_e6d0;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

/// `a - b` for the three floats at each address, each rounded to a float (`FLD`, `FSUB dword`,
/// `FSTP`), stored at `out`.
fn point_difference(e: &mut Engine, out: u32, a: u32, b: u32) -> [u32; 3] {
    let mut result = [0u32; 3];
    for (axis, slot) in result.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        *slot = x87_store32(x87_sub(
            load_float(e, a.wrapping_add(offset)),
            float_operand(e, b.wrapping_add(offset)),
        ));
        e.mem.set_u32(out.wrapping_add(offset), *slot);
    }
    result
}

/// The `FCOMPP` of the radius against the extent the triangle tests make: the extent when the
/// radius is below it (ordered), the radius otherwise, as `FLD`/`FSTP` leave the word.
fn reach_bits(e: &Engine, capsule: u32) -> u32 {
    let extent = load_float(e, capsule.wrapping_add(0x1c));
    let radius = load_float(e, capsule.wrapping_add(0x38));
    let source = if radius < extent { 0x1c } else { 0x38 };
    x87_quiet_bits(e.mem.u32(capsule.wrapping_add(source)))
}

/// Whether each of the three relative velocity components equals the matching word at
/// `011f426c` (`FUCOMPP`, unordered counts as different).
fn is_still(e: &Engine, relative: [u32; 3]) -> bool {
    relative.iter().enumerate().all(|(axis, word)| {
        load_float(e, ZERO_POINT_011F426C + 4 * axis as u32) == x87_load32(*word)
    })
}

// Translated from 00e9f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::operator==` (Xbox PDB): the other volume's `Type` (virtual slot +0xc) must be 2
/// and its capsule (+0x20) must equal this one's (`NiCapsule::operator==`, `00eae690`, called on
/// the other volume's capsule with this one's as the argument).
pub fn ni_capsule_bv_operator_(
    e: &mut Engine,
    this: Ptr<NiCapsuleBV>,
    other: Ptr<NiCapsuleBV>,
) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 2 {
        return false;
    }
    e.call(
        NI_CAPSULE_EQUAL,
        &args![
            other.addr().wrapping_add(0x20),
            this.addr().wrapping_add(0x20)
        ],
    )
    .bool()
}

// Translated from 00e9f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: rebuilds the capsule from the center, the axis, the extent and the
/// radius. With `h = extent - radius` (rounded to a float) the segment origin is
/// `center - h * axis` and the direction `2 * h * axis` (the double `2.0` at `01011590`), each
/// product and difference rounded to a float.
pub fn fn_00e9f1b0(e: &mut Engine, this: Ptr<NiCapsuleBV>) {
    let base = this.addr();
    let half = x87_store32(x87_sub(
        load_float(e, base + 0x1c),
        float_operand(e, base + 0x38),
    ));
    let h = x87_load32(half);
    let along = [
        x87_store32(x87_mul(load_float(e, base + 0x10), h)),
        x87_store32(x87_mul(load_float(e, base + 0x14), h)),
        x87_store32(x87_mul(h, float_operand(e, base + 0x18))),
    ];
    let mut origin = [0u32; 3];
    for (axis, slot) in origin.iter_mut().enumerate() {
        *slot = x87_store32(x87_sub(
            load_float(e, base + 4 + 4 * axis as u32),
            x87_m32(along[axis]),
        ));
    }
    e.set(this, NiCapsuleBV::m_kCapsule_origin_x, origin[0]);
    e.set(this, NiCapsuleBV::m_kCapsule_origin_y, origin[1]);
    e.set(this, NiCapsuleBV::m_kCapsule_origin_z, origin[2]);
    let along = [
        x87_store32(x87_mul(load_float(e, base + 0x10), h)),
        x87_store32(x87_mul(load_float(e, base + 0x14), h)),
        x87_store32(x87_mul(h, float_operand(e, base + 0x18))),
    ];
    let two = e.mem.f64(DIRECTION_SCALE);
    let x = x87_store32(x87_mul(x87_load32(along[0]), two));
    e.set(this, NiCapsuleBV::m_kCapsule_direction_x, x);
    let y = x87_store32(x87_mul(x87_load32(along[1]), two));
    e.set(this, NiCapsuleBV::m_kCapsule_direction_y, y);
    let z = x87_store32(x87_mul(two, x87_m32(along[2])));
    e.set(this, NiCapsuleBV::m_kCapsule_direction_z, z);
}

// Translated from 00e9f280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::Copy` (Xbox PDB): copies the center and the axis words, the extent and the
/// radius (through `FLD`/`FSTP`, a signalling NaN quieted) and rebuilds the capsule
/// ([`fn_00e9f1b0`]).
pub fn ni_capsule_bv_copy(e: &mut Engine, this: Ptr<NiCapsuleBV>, other: Ptr<NiCapsuleBV>) {
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        let word = e.mem.u32(other.addr().wrapping_add(offset));
        e.mem.set_u32(this.addr().wrapping_add(offset), word);
    }
    let extent = x87_quiet_bits(e.get(other, NiCapsuleBV::m_fExtent));
    e.set(this, NiCapsuleBV::m_fExtent, extent);
    let radius = x87_quiet_bits(e.get(other, NiCapsuleBV::m_kCapsule_radius));
    e.set(this, NiCapsuleBV::m_kCapsule_radius, radius);
    fn_00e9f1b0(e, this);
}

// Translated from 00e9f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::UpdateWorldData` (Xbox PDB): this volume becomes the model volume moved by the
/// transform (a row-major 3x3 rotation at +0, the translation at +0x24, the scale at +0x30).
/// The center is rotated (each row's three products summed in the executable's order and rounded),
/// scaled (the scale itself through `FLD`/`FSTP`) and translated; the axis is rotated only; the
/// extent and the radius are multiplied by the scale. The capsule is then rebuilt
/// ([`fn_00e9f1b0`]).
pub fn ni_capsule_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiCapsuleBV>,
    model: Ptr<NiCapsuleBV>,
    transform: Ptr,
) {
    let m = model.addr();
    let t = transform.addr();
    let ld = load_float;
    let op = float_operand;
    let rotated = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 8), op(e, t + 4)),
                x87_mul(ld(e, t), op(e, m + 4)),
            ),
            x87_mul(ld(e, m + 0xc), op(e, t + 8)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 8), op(e, t + 0x10)),
                x87_mul(ld(e, t + 0xc), op(e, m + 4)),
            ),
            x87_mul(ld(e, m + 0xc), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0x1c), op(e, m + 8)),
                x87_mul(ld(e, m + 4), op(e, t + 0x18)),
            ),
            x87_mul(ld(e, m + 0xc), op(e, t + 0x20)),
        )),
    ];
    let scale = x87_load32(x87_store32(ld(e, t + 0x30)));
    let scaled = [
        x87_store32(x87_mul(x87_load32(rotated[0]), scale)),
        x87_store32(x87_mul(x87_load32(rotated[1]), scale)),
        x87_store32(x87_mul(scale, x87_m32(rotated[2]))),
    ];
    let x = x87_store32(x87_add(ld(e, t + 0x24), x87_m32(scaled[0])));
    let y = x87_store32(x87_add(ld(e, t + 0x28), x87_m32(scaled[1])));
    let z = x87_store32(x87_add(ld(e, t + 0x2c), x87_m32(scaled[2])));
    e.set(this, NiCapsuleBV::m_kCenter_x, x);
    e.set(this, NiCapsuleBV::m_kCenter_y, y);
    e.set(this, NiCapsuleBV::m_kCenter_z, z);
    let axis = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x10), op(e, t)),
                x87_mul(ld(e, m + 0x14), op(e, t + 4)),
            ),
            x87_mul(ld(e, t + 8), op(e, m + 0x18)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x14), op(e, t + 0x10)),
                x87_mul(ld(e, t + 0xc), op(e, m + 0x10)),
            ),
            x87_mul(ld(e, m + 0x18), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x10), op(e, t + 0x18)),
                x87_mul(ld(e, m + 0x14), op(e, t + 0x1c)),
            ),
            x87_mul(ld(e, t + 0x20), op(e, m + 0x18)),
        )),
    ];
    e.set(this, NiCapsuleBV::m_kAxis_x, axis[0]);
    e.set(this, NiCapsuleBV::m_kAxis_y, axis[1]);
    e.set(this, NiCapsuleBV::m_kAxis_z, axis[2]);
    let extent = x87_store32(x87_mul(ld(e, m + 0x1c), op(e, t + 0x30)));
    e.set(this, NiCapsuleBV::m_fExtent, extent);
    let radius = x87_store32(x87_mul(ld(e, m + 0x38), op(e, t + 0x30)));
    e.set(this, NiCapsuleBV::m_kCapsule_radius, radius);
    fn_00e9f1b0(e, this);
}

/// The intersector part shared by [`fn_00e9f400`] and [`fn_00e9f510`]: an intersector of `size`
/// bytes on the stack made by `constructor` from the two volumes, the time, `0.001`, `0.0001`
/// and 0x20 iterations, `Find` with the two velocities, the contact time to `out_time` and, for an
/// intersection type of 3 or 2, the contact point to `out_point` and (when `want_normals`) the
/// contact normal to `out_normal` and its negation to `out_normal_negated`. The intersector is
/// destroyed on both paths.
#[allow(clippy::too_many_arguments)]
fn intersector_find(
    e: &mut Engine,
    constructor: u32,
    size: u32,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
    let step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
    let maximum_time = x87_quiet_bits(time.to_bits());
    e.with_stack(size, |e, intersector| {
        e.call(
            constructor,
            &args![
                intersector,
                first,
                second,
                maximum_time,
                step,
                threshold,
                0x20u32
            ],
        );
        e.call(
            INTERSECTOR_FIND,
            &args![intersector, first_velocity, second_velocity],
        );
        let contact_time = e.call(INTERSECTOR_CONTACT_TIME, &args![intersector]).f64();
        e.mem.set_u32(out_time.addr(), x87_store32(contact_time));
        if e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 3
            && e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 2
        {
            e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
            return false;
        }
        let point = e.call(INTERSECTOR_CONTACT_POINT, &args![intersector]).u32();
        copy_point(e, out_point.addr(), point);
        if want_normals {
            let normal = e
                .call(INTERSECTOR_CONTACT_NORMAL, &args![intersector])
                .u32();
            copy_point(e, out_normal.addr(), normal);
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
}

// Translated from 00e9f400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): a capsule `Find` test (cdecl, ten
/// words: the time, a volume and its velocity, another volume and its velocity, `out_time`,
/// `out_point`, `want_normals` (a byte), `out_normal`, `out_normal_negated`). It runs the general
/// intersector test with the intersector constructor `00eae9e0` (0x48 bytes on the stack); see
/// [`intersector_find`].
#[allow(clippy::too_many_arguments)]
pub fn fn_00e9f400(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    intersector_find(
        e,
        INTERSECTOR_CONSTRUCTOR_00EAE9E0,
        0x48,
        time,
        first,
        first_velocity,
        second,
        second_velocity,
        out_time,
        out_point,
        want_normals,
        out_normal,
        out_normal_negated,
    )
}

// Translated from 00e9f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): [`fn_00e9f400`] with the
/// intersector constructor `00eaedc0` (0x4c bytes on the stack).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e9f510(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    intersector_find(
        e,
        INTERSECTOR_CONSTRUCTOR_00EAEDC0,
        0x4c,
        time,
        first,
        first_velocity,
        second,
        second_velocity,
        out_time,
        out_point,
        want_normals,
        out_normal,
        out_normal_negated,
    )
}

// Translated from 00e9f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::CapsuleSphereTestIntersect` (Xbox PDB; cdecl, five words: the time, the capsule,
/// its velocity, the sphere volume, its velocity). The relative velocity is the sphere's minus the
/// capsule's (each component rounded). When it equals the zero point at `011f426c` the squared
/// distance is that of the sphere's center to the capsule's segment (`NiSqrDistance::Compute`);
/// otherwise that of the segment from the center along `time * velocity` to it
/// (`Compute_ov2`). The answer is whether the distance, rounded, is at most the square of the
/// summed radii (rounded), ordered.
pub fn ni_capsule_bv_capsule_sphere_test_intersect(
    e: &mut Engine,
    time: f32,
    capsule: Ptr<NiCapsuleBV>,
    capsule_velocity: Ptr,
    sphere: Ptr,
    sphere_velocity: Ptr,
) -> bool {
    e.with_stack(0x24, |e, frame| {
        let relative_at = frame.addr();
        let relative = point_difference(
            e,
            relative_at,
            sphere_velocity.addr(),
            capsule_velocity.addr(),
        );
        let outs = relative_at + 0xc;
        let segment_capsule = capsule.addr().wrapping_add(0x20);
        let distance = if is_still(e, relative) {
            e.call(
                SQR_DISTANCE_POINT_SEGMENT,
                &args![sphere.addr().wrapping_add(4), segment_capsule, outs],
            )
            .f64()
        } else {
            e.with_stack(0x18, |e, segment| {
                let scale = x87_load32(time.to_bits());
                copy_point(e, segment.addr(), sphere.addr().wrapping_add(4));
                let moved = [
                    x87_store32(x87_mul(x87_load32(relative[0]), scale)),
                    x87_store32(x87_mul(x87_load32(relative[1]), scale)),
                    x87_store32(x87_mul(scale, x87_load32(relative[2]))),
                ];
                for (axis, word) in moved.iter().enumerate() {
                    e.mem.set_u32(segment.addr() + 0xc + 4 * axis as u32, *word);
                }
                e.call(
                    SQR_DISTANCE_SEGMENT_SEGMENT,
                    &args![segment, segment_capsule, outs, outs + 4],
                )
                .f64()
            })
        };
        let distance = x87_store32(distance);
        let reach = x87_store32(x87_add(
            load_float(e, capsule.addr().wrapping_add(0x38)),
            float_operand(e, sphere.addr().wrapping_add(0x10)),
        ));
        let reach = x87_load32(reach);
        let reach_squared = x87_load32(x87_store32(x87_mul(reach, reach)));
        x87_load32(distance) <= reach_squared
    })
}

// Translated from 00e9f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the capsule-capsule test (cdecl, five words: the time, the first
/// capsule, its velocity, the second capsule, its velocity). The relative velocity is the
/// second's minus the first's (each rounded). When it equals the zero point at `011f426c` the
/// squared distance is that between the two segments (`Compute_ov2`); otherwise that between the
/// first segment and the second segment moving by `time * velocity` (`00eaff20`, given the
/// second segment and the displacement as one block). The answer is whether the distance, rounded,
/// is at most the square of the summed radii (rounded), ordered.
pub fn fn_00e9f760(
    e: &mut Engine,
    time: f32,
    first: Ptr<NiCapsuleBV>,
    first_velocity: Ptr,
    second: Ptr<NiCapsuleBV>,
    second_velocity: Ptr,
) -> bool {
    e.with_stack(0x30, |e, frame| {
        let relative_at = frame.addr();
        let relative = point_difference(
            e,
            relative_at,
            second_velocity.addr(),
            first_velocity.addr(),
        );
        let outs = relative_at + 0xc;
        let first_segment = first.addr().wrapping_add(0x20);
        let second_segment = second.addr().wrapping_add(0x20);
        let distance = if is_still(e, relative) {
            e.call(
                SQR_DISTANCE_SEGMENT_SEGMENT,
                &args![first_segment, second_segment, outs, outs + 4],
            )
            .f64()
        } else {
            e.with_stack(0x24, |e, block| {
                let scale = x87_load32(time.to_bits());
                for offset in (0..0x18u32).step_by(4) {
                    let word = e.mem.u32(second_segment + offset);
                    e.mem.set_u32(block.addr() + offset, word);
                }
                let moved = [
                    x87_store32(x87_mul(x87_load32(relative[0]), scale)),
                    x87_store32(x87_mul(x87_load32(relative[1]), scale)),
                    x87_store32(x87_mul(scale, x87_load32(relative[2]))),
                ];
                for (axis, word) in moved.iter().enumerate() {
                    e.mem.set_u32(block.addr() + 0x18 + 4 * axis as u32, *word);
                }
                e.call(
                    SQR_DISTANCE_SEGMENT_MOVING_SEGMENT,
                    &args![first_segment, block, outs, outs + 4, outs + 8],
                )
                .f64()
            })
        };
        let distance = x87_store32(distance);
        let reach = x87_store32(x87_add(
            load_float(e, first.addr().wrapping_add(0x38)),
            float_operand(e, second.addr().wrapping_add(0x38)),
        ));
        let reach = x87_load32(reach);
        let reach_squared = x87_load32(x87_store32(x87_mul(reach, reach)));
        x87_load32(distance) <= reach_squared
    })
}

// Translated from 00e9f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the capsule-triangle test (cdecl,
/// seven words: the time, the capsule, its velocity, the three triangle vertices, the triangle's
/// velocity). The triangle is the first vertex and its two edges (rounded), the relative
/// velocity the capsule's minus the triangle's (rounded).
///
/// A quick rejection first: with the plane normal `u` (the negated cross product of the edges,
/// each component rounded), the signed distances `d1 = u . center` and `d2 = u . vertex0`
/// (rounded) and the reach `R` (the extent when the radius is below it, else the radius), the
/// capsule is clear of the plane when `|u|^2 R^2` (rounded) is below `(d1 - d2)^2`. Then `u` is
/// unitized, and when `d2 < d1` an approach speed `u . velocity` that is not below the float at
/// `01011d78` answers false (when not, one not above it answers false); the near point `w` is
/// `center - R u` (or `center + R u`), the plane constant `u . vertex0 + 0.001`, and if `u . w`
/// and `u . (w + time * velocity)` are both above it the answer is false.
///
/// Otherwise the squared distance of the capsule's segment to the triangle is computed
/// (`Compute_ov3` when the relative velocity equals the zero point at `011f426c`, else
/// `00eb3230` with the segment moving by `time * velocity`); the answer is whether it, rounded,
/// is at most the radius times the radius, ordered.
pub fn fn_00e9f8c0(
    e: &mut Engine,
    time: f32,
    capsule: Ptr<NiCapsuleBV>,
    capsule_velocity: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    e.with_stack(0x88, |e, frame| {
        // The executable's locals, by their distance below the return address.
        let sp = |offset: u32| frame.addr() + 0x88 - offset;
        let cap = capsule.addr();
        let vertex = first.addr();
        let triangle = sp(0x48);
        copy_point(e, triangle, vertex);
        let edge0 = point_difference(e, sp(0x3c), second.addr(), vertex);
        let edge1 = point_difference(e, sp(0x30), third.addr(), vertex);
        let relative = point_difference(
            e,
            sp(0x54),
            capsule_velocity.addr(),
            triangle_velocity.addr(),
        );
        let a = edge0.map(x87_load32);
        let b = edge1.map(x87_load32);
        let normal = [
            x87_store32(x87_sub(x87_mul(b[2], a[1]), x87_mul(b[1], a[2]))),
            x87_store32(x87_sub(x87_mul(a[2], b[0]), x87_mul(b[2], a[0]))),
            x87_store32(x87_sub(x87_mul(a[0], b[1]), x87_mul(b[0], a[1]))),
        ];
        let unit = sp(0x84);
        for (axis, word) in normal.iter().enumerate() {
            e.mem.set_u32(unit + 4 * axis as u32, negated_bits(*word));
        }
        let u = [
            load_float(e, unit),
            load_float(e, unit + 4),
            load_float(e, unit + 8),
        ];
        let center_distance = x87_store32(x87_add(
            x87_mul(load_float(e, cap + 0xc), u[2]),
            x87_add(
                x87_mul(load_float(e, cap + 4), u[0]),
                x87_mul(load_float(e, cap + 8), u[1]),
            ),
        ));
        let reach = x87_load32(reach_bits(e, cap));
        let vertex_distance = x87_store32(x87_add(
            x87_add(
                x87_mul(load_float(e, vertex), u[0]),
                x87_mul(u[1], float_operand(e, vertex + 4)),
            ),
            x87_mul(load_float(e, vertex + 8), u[2]),
        ));
        let plane_distance = x87_load32(x87_store32(x87_sub(
            x87_load32(center_distance),
            x87_m32(vertex_distance),
        )));
        let distance_squared = x87_mul(plane_distance, plane_distance);
        let normal_squared = x87_store32(x87_add(
            x87_add(x87_mul(u[1], u[1]), x87_mul(u[0], u[0])),
            x87_mul(u[2], u[2]),
        ));
        let spread = x87_store32(x87_mul(x87_mul(x87_load32(normal_squared), reach), reach));
        if x87_load32(spread) < distance_squared {
            e.call(UNITIZE, &args![unit]);
            let u = [
                load_float(e, unit),
                load_float(e, unit + 4),
                load_float(e, unit + 8),
            ];
            let against = x87_load32(vertex_distance) < x87_load32(center_distance);
            let speed = x87_store32(x87_add(
                x87_add(
                    x87_mul(u[1], x87_m32(relative[1])),
                    x87_mul(u[0], x87_m32(relative[0])),
                ),
                x87_mul(u[2], x87_load32(relative[2])),
            ));
            let speed = x87_load32(speed);
            let threshold = float_operand(e, APPROACH_THRESHOLD);
            // `d2 < d1`: a speed not below the threshold (ordered) rejects; otherwise one not
            // above it.
            if against {
                if speed >= threshold {
                    return false;
                }
            } else if speed <= threshold {
                return false;
            }
            let along = [
                x87_store32(x87_mul(u[0], reach)),
                x87_store32(x87_mul(u[1], reach)),
                x87_store32(x87_mul(reach, u[2])),
            ];
            let mut near = [0u32; 3];
            for (axis, slot) in near.iter_mut().enumerate() {
                let center = load_float(e, cap + 4 + 4 * axis as u32);
                let step = x87_m32(along[axis]);
                *slot = x87_store32(if against {
                    x87_sub(center, step)
                } else {
                    x87_add(center, step)
                });
                e.mem.set_u32(sp(0x6c) + 4 * axis as u32, *slot);
            }
            // The near point also overwrites the unit normal's memory (the normal stays in
            // registers).
            for (axis, word) in near.iter().enumerate() {
                e.mem.set_u32(unit + 4 * axis as u32, *word);
            }
            let plane_constant = x87_store32(x87_add(
                x87_add(
                    x87_mul(load_float(e, vertex), u[0]),
                    x87_mul(u[1], float_operand(e, vertex + 4)),
                ),
                x87_mul(load_float(e, vertex + 8), u[2]),
            ));
            let plane = x87_add(x87_load32(plane_constant), e.mem.f64(PLANE_MARGIN));
            let height = x87_store32(x87_add(
                x87_add(
                    x87_mul(x87_load32(near[1]), u[1]),
                    x87_mul(x87_load32(near[0]), u[0]),
                ),
                x87_mul(x87_load32(near[2]), u[2]),
            ));
            if x87_load32(height) > plane {
                let t = x87_load32(time.to_bits());
                let moved = [
                    x87_store32(x87_mul(x87_load32(relative[0]), t)),
                    x87_store32(x87_mul(x87_load32(relative[1]), t)),
                    x87_store32(x87_mul(x87_load32(relative[2]), t)),
                ];
                let mut later = [0u32; 3];
                for (axis, slot) in later.iter_mut().enumerate() {
                    *slot = x87_store32(x87_add(
                        x87_load32(moved[axis]),
                        float_operand(e, unit + 4 * axis as u32),
                    ));
                }
                let later_height = x87_store32(x87_add(
                    x87_mul(u[2], x87_load32(later[2])),
                    x87_add(
                        x87_mul(u[0], x87_load32(later[0])),
                        x87_mul(u[1], x87_load32(later[1])),
                    ),
                ));
                if x87_load32(later_height) > plane {
                    return false;
                }
            }
        }

        // The distance test.
        let segment = cap + 0x20;
        let distance = if is_still(e, relative) {
            e.call(
                SQR_DISTANCE_SEGMENT_TRIANGLE,
                &args![segment, triangle, sp(0x5c), sp(0x60), sp(0x88)],
            )
            .f64()
        } else {
            let t = x87_load32(time.to_bits());
            let block = sp(0x24);
            for offset in (0..0x18u32).step_by(4) {
                let word = e.mem.u32(segment + offset);
                e.mem.set_u32(block + offset, word);
            }
            let moved = [
                x87_store32(x87_mul(x87_load32(relative[0]), t)),
                x87_store32(x87_mul(x87_load32(relative[1]), t)),
                x87_store32(x87_mul(t, x87_load32(relative[2]))),
            ];
            for (axis, word) in moved.iter().enumerate() {
                e.mem.set_u32(block + 0x18 + 4 * axis as u32, *word);
            }
            e.call(
                SQR_DISTANCE_TRIANGLE_MOVING_SEGMENT,
                &args![triangle, block, sp(0x60), sp(0x88), sp(0x58), sp(0x5c)],
            )
            .f64()
        };
        let distance = x87_load32(x87_store32(distance));
        let radius = load_float(e, cap + 0x38);
        distance <= x87_mul(radius, radius)
    })
}

// Translated from 00e9fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the capsule-triangle `Find`
/// (cdecl, twelve words: the time, the capsule, its velocity, the three triangle vertices, the
/// triangle's velocity, `out_time`, `out_point`, `want_normals` (a byte), `out_normal`,
/// `out_normal_negated`).
///
/// The squared distance of the capsule's segment to the triangle (`Compute_ov3`, with the
/// segment parameter and the two barycentric parameters as out floats) minus the squared radius
/// (rounded) at most 0 means the capsule already touches it: the time is 0, the point is the
/// closest triangle point (`vertex0 + s edge0 + t edge1`, each term rounded) and, when
/// `want_normals`, the triangle's normal (`NiTrigon::GetNormal`) goes to `out_normal_negated` and
/// its negation to `out_normal`; the answer is true.
///
/// Otherwise a squared relative speed (the capsule's velocity minus the triangle's) times the
/// time that is not above the double at `0101dae8` answers false; then the same plane rejection
/// as [`fn_00e9f8c0`] (the near point formed by `00439ef0` or `00439e90` from the center and
/// `R u` scaled by `004a3760`, and the later height by `NiPoint3::Dot`); every other case runs
/// the intersector (`00eb3750`, 0x6c bytes on the stack, the time, `0.001`, `0.0001`, 0x20
/// iterations): its contact time to `out_time` and, for an intersection type of 3 or 2, its
/// contact point to `out_point` and (when `want_normals`) its normal to `out_normal` and the
/// negation to `out_normal_negated`, answering true; otherwise false. The intersector is
/// destroyed on both paths.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e9fdc0(
    e: &mut Engine,
    time: f32,
    capsule: Ptr<NiCapsuleBV>,
    capsule_velocity: Ptr,
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
    e.with_stack(0x10c, |e, frame| {
        let sp = |offset: u32| frame.addr() + 0x10c - offset;
        let cap = capsule.addr();
        let vertex = first.addr();
        let radius = load_float(e, cap + 0x38);
        let radius_squared = x87_store32(x87_mul(radius, radius));
        let segment = sp(0xd0);
        for offset in (0..0x18u32).step_by(4) {
            let word = e.mem.u32(cap + 0x20 + offset);
            e.mem.set_u32(segment + offset, word);
        }
        let triangle = sp(0xac);
        copy_point(e, triangle, vertex);
        let edge0 = point_difference(e, sp(0xa0), second.addr(), vertex);
        let edge1 = point_difference(e, sp(0x94), third.addr(), vertex);
        let distance = e
            .call(
                SQR_DISTANCE_SEGMENT_TRIANGLE,
                &args![segment, triangle, sp(0x84), sp(0x80), sp(0x7c)],
            )
            .f64();
        let difference = x87_store32(x87_sub(distance, x87_m32(radius_squared)));
        if x87_load32(difference) <= 0.0 {
            // Already touching.
            e.mem.set_u32(out_time.addr(), 0);
            let t = load_float(e, sp(0x7c));
            let along1 = [
                x87_store32(x87_mul(load_float(e, sp(0x94)), t)),
                x87_store32(x87_mul(load_float(e, sp(0x90)), t)),
                x87_store32(x87_mul(t, float_operand(e, sp(0x8c)))),
            ];
            let s = load_float(e, sp(0x80));
            let along0 = [
                x87_store32(x87_mul(load_float(e, sp(0xa0)), s)),
                x87_store32(x87_mul(load_float(e, sp(0x9c)), s)),
                x87_store32(x87_mul(s, float_operand(e, sp(0x98)))),
            ];
            let mut on_edge0 = [0u32; 3];
            for (axis, slot) in on_edge0.iter_mut().enumerate() {
                *slot = x87_store32(x87_add(
                    load_float(e, triangle + 4 * axis as u32),
                    x87_m32(along0[axis]),
                ));
            }
            for axis in 0..3usize {
                let point = x87_store32(x87_add(x87_load32(on_edge0[axis]), x87_m32(along1[axis])));
                e.mem
                    .set_u32(out_point.addr().wrapping_add(4 * axis as u32), point);
            }
            if !want_normals {
                return true;
            }
            let normal = e
                .call(NI_TRIGON_GET_NORMAL, &args![triangle, sp(0xdc)])
                .u32();
            copy_point(e, out_normal_negated.addr(), normal);
            for axis in 0..3u32 {
                let word = e.mem.u32(out_normal_negated.addr().wrapping_add(4 * axis));
                e.mem
                    .set_u32(out_normal.addr().wrapping_add(4 * axis), negated_bits(word));
            }
            return true;
        }

        // Not touching yet: how the capsule moves.
        let relative = point_difference(
            e,
            sp(0x10c),
            capsule_velocity.addr(),
            triangle_velocity.addr(),
        );
        for (axis, word) in relative.iter().enumerate() {
            e.mem.set_u32(sp(0xb8) + 4 * axis as u32, *word);
        }
        let r = relative.map(x87_load32);
        let speed_squared = x87_store32(x87_add(
            x87_add(x87_mul(r[0], r[0]), x87_mul(r[1], r[1])),
            x87_mul(r[2], r[2]),
        ));
        let motion = x87_mul(x87_load32(speed_squared), x87_m32(time.to_bits()));
        if motion <= e.mem.f64(MINIMUM_MOTION) {
            return false;
        }
        let a = edge0.map(x87_load32);
        let b = edge1.map(x87_load32);
        let normal = [
            x87_store32(x87_sub(x87_mul(a[1], b[2]), x87_mul(a[2], b[1]))),
            x87_store32(x87_sub(x87_mul(a[2], b[0]), x87_mul(b[2], a[0]))),
            x87_store32(x87_sub(x87_mul(b[1], a[0]), x87_mul(a[1], b[0]))),
        ];
        let unit = sp(0xfc);
        for (axis, word) in normal.iter().enumerate() {
            e.mem.set_u32(unit + 4 * axis as u32, negated_bits(*word));
        }
        let u = [
            load_float(e, unit),
            load_float(e, unit + 4),
            load_float(e, unit + 8),
        ];
        let center_distance = x87_store32(x87_add(
            x87_mul(load_float(e, cap + 0xc), u[2]),
            x87_add(
                x87_mul(load_float(e, cap + 8), u[1]),
                x87_mul(u[0], float_operand(e, cap + 4)),
            ),
        ));
        let reach_word = reach_bits(e, cap);
        let reach = x87_load32(reach_word);
        let vertex_distance = x87_store32(x87_add(
            x87_add(
                x87_mul(u[0], float_operand(e, vertex)),
                x87_mul(load_float(e, vertex + 4), u[1]),
            ),
            x87_mul(u[2], float_operand(e, vertex + 8)),
        ));
        let plane_distance = x87_load32(x87_store32(x87_sub(
            x87_load32(center_distance),
            x87_m32(vertex_distance),
        )));
        let distance_squared = x87_mul(plane_distance, plane_distance);
        let normal_squared = x87_store32(x87_add(
            x87_mul(u[2], u[2]),
            x87_add(x87_mul(u[1], u[1]), x87_mul(u[0], u[0])),
        ));
        let spread = x87_store32(x87_mul(x87_mul(x87_load32(normal_squared), reach), reach));
        if x87_load32(spread) < distance_squared {
            e.call(UNITIZE, &args![unit]);
            let against = x87_load32(vertex_distance) < x87_load32(center_distance);
            let speed = x87_store32(x87_add(
                x87_add(
                    x87_mul(load_float(e, unit), float_operand(e, sp(0x10c))),
                    x87_mul(load_float(e, unit + 4), float_operand(e, sp(0x108))),
                ),
                x87_mul(load_float(e, unit + 8), float_operand(e, sp(0x104))),
            ));
            let speed = x87_load32(speed);
            let threshold = float_operand(e, APPROACH_THRESHOLD);
            if against {
                if speed >= threshold {
                    return false;
                }
            } else if speed <= threshold {
                return false;
            }
            let scaled = e
                .call(SCALE_POINT, &args![sp(0xdc), reach_word, unit])
                .u32();
            let combine = if against {
                POINT_COMBINE_00439EF0
            } else {
                POINT_COMBINE_00439E90
            };
            let near = e.call(combine, &args![cap + 4, sp(0xf0), scaled]).u32();
            let near_point = sp(0x10c);
            copy_point(e, near_point, near);
            let u = [
                load_float(e, unit),
                load_float(e, unit + 4),
                load_float(e, unit + 8),
            ];
            let plane_constant = x87_store32(x87_add(
                x87_add(
                    x87_mul(u[0], float_operand(e, vertex)),
                    x87_mul(load_float(e, vertex + 4), u[1]),
                ),
                x87_mul(u[2], float_operand(e, vertex + 8)),
            ));
            let plane = x87_add(x87_load32(plane_constant), e.mem.f64(PLANE_MARGIN));
            let height = x87_store32(x87_add(
                x87_add(
                    x87_mul(u[0], load_float(e, near_point)),
                    x87_mul(u[1], load_float(e, near_point + 4)),
                ),
                x87_mul(u[2], float_operand(e, near_point + 8)),
            ));
            if x87_load32(height) > plane {
                let time_bits = x87_quiet_bits(time.to_bits());
                let travelled = e
                    .call(SCALE_POINT, &args![sp(0x78), time_bits, sp(0xb8)])
                    .u32();
                let later = sp(0xdc);
                let x = x87_store32(x87_add(
                    load_float(e, near_point),
                    float_operand(e, travelled),
                ));
                e.mem.set_u32(later, x);
                let y = x87_store32(x87_add(
                    load_float(e, travelled + 4),
                    float_operand(e, near_point + 4),
                ));
                e.mem.set_u32(later + 4, y);
                let z = x87_store32(x87_add(
                    load_float(e, travelled + 8),
                    float_operand(e, near_point + 8),
                ));
                e.mem.set_u32(later + 8, z);
                let later_height = e.call(NI_POINT3_DOT, &args![unit, later]).f64();
                if later_height > plane {
                    return false;
                }
            }
        }

        // The general test.
        let intersector = sp(0x6c);
        let maximum_time = x87_quiet_bits(time.to_bits());
        let step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
        let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
        e.call(
            CAPSULE_TRI_INTERSECTOR_CONSTRUCTOR,
            &args![
                intersector,
                capsule,
                first,
                second,
                third,
                maximum_time,
                step,
                threshold,
                0x20u32
            ],
        );
        e.call(
            INTERSECTOR_FIND,
            &args![intersector, capsule_velocity, triangle_velocity],
        );
        let contact_time = e.call(INTERSECTOR_CONTACT_TIME, &args![intersector]).f64();
        e.mem.set_u32(out_time.addr(), x87_store32(contact_time));
        if e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 3
            && e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 2
        {
            e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
            return false;
        }
        let point = e.call(INTERSECTOR_CONTACT_POINT, &args![intersector]).u32();
        copy_point(e, out_point.addr(), point);
        if want_normals {
            let normal = e
                .call(INTERSECTOR_CONTACT_NORMAL, &args![intersector])
                .u32();
            copy_point(e, out_normal.addr(), normal);
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
}

/// Fills a fresh capsule volume as `Create` does: the vtable, the center from `011f426c`, the
/// axis from `011a946c`, extent and radius 1.0, then the capsule ([`fn_00e9f1b0`]).
fn initialize_default(e: &mut Engine, block: u32) {
    let created: Ptr<NiCapsuleBV> = Ptr::new(block);
    e.set(created, NiCapsuleBV::m_fExtent, 1.0f32.to_bits());
    copy_point(e, block + 4, ZERO_POINT_011F426C);
    e.set(created, NiCapsuleBV::m_kCapsule_radius, 1.0f32.to_bits());
    copy_point(e, block + 0x10, DEFAULT_AXIS_011A946C);
    e.mem.set_u32(block, NI_CAPSULE_BV_VTABLE);
    fn_00e9f1b0(e, created);
}

// Translated from 00ea04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the capsule volume's stream
/// loader (cdecl, the stream). Makes a capsule volume as [`ni_capsule_bv_create`] does and runs
/// its `LoadBinary` (virtual slot 0) on the stream. When the allocation fails the code still
/// makes the virtual call through the null pointer (it reads the vtable at address 0). Answers
/// the volume.
pub fn fn_00ea04b0(e: &mut Engine, stream: Ptr) -> Ptr<NiCapsuleBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x3cu32])
        .u32();
    if block != 0 {
        initialize_default(e, block);
    }
    e.vcall(block, 0, &args![stream]);
    Ptr::new(block)
}

// Translated from 00ea0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::LoadBinary` (Xbox PDB): the base volume's part (`00a89af0`), the center and the
/// axis (`NiColor::LoadBinary`), the extent and the radius (4-byte items through the stream's
/// read function at +8 of its reader at +0x24c, straight into the fields), then the capsule
/// ([`fn_00e9f1b0`]).
pub fn ni_capsule_bv_load_binary(e: &mut Engine, this: Ptr<NiCapsuleBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.call(LOAD_THREE_FLOATS, &args![this.addr() + 4, stream]);
    e.call(LOAD_THREE_FLOATS, &args![this.addr() + 0x10, stream]);
    for field in [0x1cu32, 0x38] {
        let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
        stream_transfer(e, reader, 8, this.addr() + field, 4);
    }
    fn_00e9f1b0(e, this);
}

// Translated from 00ea05c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::SaveBinary` (Xbox PDB): `NiBoundingVolume::SaveBinary`, the center and the axis
/// (`NiColor::SaveBinary`), then the extent and the radius as 4-byte items (the stream's write
/// function at +0xc of its writer at +0x250).
pub fn ni_capsule_bv_save_binary(e: &mut Engine, this: Ptr<NiCapsuleBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.call(SAVE_THREE_FLOATS, &args![this.addr() + 4, stream]);
    e.call(SAVE_THREE_FLOATS, &args![this.addr() + 0x10, stream]);
    for field in [0x1cu32, 0x38] {
        let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
        stream_transfer(e, writer, 0xc, this.addr() + field, 4);
    }
}

// Translated from 00ea0640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiCapsuleBV` copy constructor. Sets the vtable `010f17d0`,
/// copies the center and the axis words, the extent and the radius (through `FLD`/`FSTP`) and
/// rebuilds the capsule ([`fn_00e9f1b0`]). Answers `this`.
pub fn fn_00ea0640(
    e: &mut Engine,
    this: Ptr<NiCapsuleBV>,
    other: Ptr<NiCapsuleBV>,
) -> Ptr<NiCapsuleBV> {
    e.mem.set_u32(this.addr(), NI_CAPSULE_BV_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        let word = e.mem.u32(other.addr().wrapping_add(offset));
        e.mem.set_u32(this.addr().wrapping_add(offset), word);
    }
    let extent = x87_quiet_bits(e.get(other, NiCapsuleBV::m_fExtent));
    e.set(this, NiCapsuleBV::m_fExtent, extent);
    let radius = x87_quiet_bits(e.get(other, NiCapsuleBV::m_kCapsule_radius));
    e.set(this, NiCapsuleBV::m_kCapsule_radius, radius);
    fn_00e9f1b0(e, this);
    this
}

// Translated from 00ea0690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::Create` (Xbox PDB): 0x3c bytes from the memory manager and, when that gives a
/// block, a capsule volume with the center from `011f426c`, the axis from `011a946c`, extent and
/// radius 1.0 and its capsule built ([`fn_00e9f1b0`]). Answers the volume, or null.
pub fn ni_capsule_bv_create(e: &mut Engine) -> Ptr<NiCapsuleBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x3cu32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    initialize_default(e, block);
    Ptr::new(block)
}

// Translated from 00ea0700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::Clone` (Xbox PDB): 0x3c bytes from the memory manager, built as a copy of this
/// volume ([`fn_00ea0640`]). Answers the copy, or null.
pub fn ni_capsule_bv_clone(e: &mut Engine, this: Ptr<NiCapsuleBV>) -> Ptr<NiCapsuleBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x3cu32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00ea0640(e, Ptr::new(block), this)
}

// Translated from 00ea0730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleBV::AddViewerStrings` (Xbox PDB): appends `"<name> = CAPSULE_BV"` (a block of
/// `strlen(name) + 0xe` bytes from `00aa1070`, formatted by `NiSprintf`) to the strings array.
pub fn ni_capsule_bv_add_viewer_strings(
    e: &mut Engine,
    _this: Ptr<NiCapsuleBV>,
    name: Ptr,
    strings: Ptr,
) {
    let size = c_string_length(e, name.addr()).wrapping_add(0xe);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, CAPSULE_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_f170,
            ni_capsule_bv_operator_(Ptr<NiCapsuleBV>, Ptr<NiCapsuleBV>) -> bool
        ),
        entry!(0x00e9_f1b0, fn_00e9f1b0(Ptr<NiCapsuleBV>)),
        entry!(
            0x00e9_f280,
            ni_capsule_bv_copy(Ptr<NiCapsuleBV>, Ptr<NiCapsuleBV>)
        ),
        entry!(
            0x00e9_f2c0,
            ni_capsule_bv_update_world_data(Ptr<NiCapsuleBV>, Ptr<NiCapsuleBV>, Ptr)
        ),
        entry!(0x00e9_f400, fn_00e9f400(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool),
        entry!(0x00e9_f510, fn_00e9f510(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool),
        entry!(
            0x00e9_f620,
            ni_capsule_bv_capsule_sphere_test_intersect(
                f32,
                Ptr<NiCapsuleBV>,
                Ptr,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00e9_f760,
            fn_00e9f760(f32, Ptr<NiCapsuleBV>, Ptr, Ptr<NiCapsuleBV>, Ptr) -> bool
        ),
        entry!(
            0x00e9_f8c0,
            fn_00e9f8c0(f32, Ptr<NiCapsuleBV>, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_fdc0,
            fn_00e9fdc0(
                f32,
                Ptr<NiCapsuleBV>,
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
        entry!(0x00ea_04b0, fn_00ea04b0(Ptr) -> Ptr<NiCapsuleBV>),
        entry!(
            0x00ea_0540,
            ni_capsule_bv_load_binary(Ptr<NiCapsuleBV>, Ptr)
        ),
        entry!(
            0x00ea_05c0,
            ni_capsule_bv_save_binary(Ptr<NiCapsuleBV>, Ptr)
        ),
        entry!(
            0x00ea_0640,
            fn_00ea0640(Ptr<NiCapsuleBV>, Ptr<NiCapsuleBV>) -> Ptr<NiCapsuleBV>
        ),
        entry!(0x00ea_0690, ni_capsule_bv_create() -> Ptr<NiCapsuleBV>),
        entry!(
            0x00ea_0700,
            ni_capsule_bv_clone(Ptr<NiCapsuleBV>) -> Ptr<NiCapsuleBV>
        ),
        entry!(
            0x00ea_0730,
            ni_capsule_bv_add_viewer_strings(Ptr<NiCapsuleBV>, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nicollision::nicollisiondata::{VIEWER_STRINGS_GROW, VIEWER_STRINGS_SET_AT};

    /// The fake vtable for other volumes: slot `s` calls `SLOT_BASE + s`.
    const OTHER_VTABLE: u32 = 0x0200_0000;
    const SLOT_BASE: u32 = 0x0300_0000;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

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

    /// A double answering the `f64` stored at `RESULT` in ST0.
    const RESULT: u32 = 0x0400_0000;
    /// Where the segment-triangle double of the find test takes its answer.
    const DISTANCE: u32 = RESULT + 0x20;
    fn answer_result(e: &mut Engine, _: &[u32]) -> Ret {
        Ret {
            st0: e.mem.f64(RESULT),
            ..Ret::default()
        }
    }

    /// An engine with the data pages the unit reads mapped, its constants as in the executable
    /// and the memory manager's allocation answering fresh blocks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.map(RESULT, 0x40);
        e.mem.set_f64(DIRECTION_SCALE, 2.0);
        e.mem.set_f32(APPROACH_THRESHOLD, 0.0);
        e.mem.set_f64(PLANE_MARGIN, 0.001f32 as f64);
        e.mem.set_f64(MINIMUM_MOTION, 1.0e-5);
        e.mem.set_f32(PSEUDODISTANCE_THRESHOLD, 0.0001);
        e.mem.set_f32(DERIVATIVE_TIME_STEP, 0.001);
        e.mem.set_f32(DEFAULT_AXIS_011A946C, 1.0);
        let slots: Vec<u32> = (0..12).map(|i| SLOT_BASE + 4 * i).collect();
        e.put_vtable(OTHER_VTABLE, &slots);
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

    /// A capsule volume with its capsule built.
    fn capsule(
        e: &mut Engine,
        center: [f32; 3],
        axis: [f32; 3],
        extent: f32,
        radius: f32,
    ) -> Ptr<NiCapsuleBV> {
        let this = e.new_object::<NiCapsuleBV>();
        e.mem.set_u32(this.addr(), NI_CAPSULE_BV_VTABLE);
        for k in 0..3u32 {
            e.mem.set_f32(this.addr() + 4 + 4 * k, center[k as usize]);
            e.mem.set_f32(this.addr() + 0x10 + 4 * k, axis[k as usize]);
        }
        e.mem.set_f32(this.addr() + 0x1c, extent);
        e.mem.set_f32(this.addr() + 0x38, radius);
        fn_00e9f1b0(e, this);
        this
    }

    #[test]
    fn operator_equal_needs_type_two_and_equal_capsules() {
        let mut e = engine();
        e.register(SLOT_BASE + 0xc, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(NI_CAPSULE_EQUAL, |e, a| Ret {
            eax: e.mem.u32(a[0] - 0x20 + 8),
            ..Ret::default()
        });
        let this = capsule(&mut e, [0.0; 3], [1.0, 0.0, 0.0], 1.0, 0.5);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other, OTHER_VTABLE);
        e.mem.set_u32(other + 4, 2);
        e.mem.set_u32(other + 8, 1);
        assert!(e.call(0x00e9_f170, &args![this, other]).bool());
        assert_eq!(
            logged(&e, NI_CAPSULE_EQUAL),
            vec![vec![other + 0x20, this.addr() + 0x20]]
        );
        e.mem.set_u32(other + 8, 0);
        assert!(!e.call(0x00e9_f170, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0);
        e.mem.set_u32(other + 8, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_f170, &args![this, other]).bool());
        assert!(logged(&e, NI_CAPSULE_EQUAL).is_empty());
    }

    #[test]
    fn fn_00e9f1b0_builds_the_segment_from_center_axis_extent_and_radius() {
        let mut e = engine();
        let this = capsule(&mut e, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 3.0, 1.0);
        assert_eq!(
            floats(&e, this.addr() + 0x20, 6),
            vec![1.0, 2.0, 1.0, 0.0, 0.0, 4.0]
        );
        e.call(0x00e9_f1b0, &args![this]);
        assert_eq!(floats(&e, this.addr() + 0x2c, 3), vec![0.0, 0.0, 4.0]);
    }

    #[test]
    fn copy_moves_the_words_quiets_the_floats_and_rebuilds() {
        let mut e = engine();
        let other = capsule(&mut e, [1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 2.0, 0.5);
        e.mem.set_u32(other.addr() + 0x38, 0x7f80_0001);
        let this = e.new_object::<NiCapsuleBV>();
        e.call(0x00e9_f280, &args![this, other]);
        assert_eq!(
            floats(&e, this.addr() + 4, 7),
            floats(&e, other.addr() + 4, 7)
        );
        assert_eq!(e.get(this, NiCapsuleBV::m_kCapsule_radius), 0x7fc0_0001);
        // h = 2 - NaN: the segment is NaN.
        assert!(e.mem.f32(this.addr() + 0x20).is_nan());
        let copy = e.new_object::<NiCapsuleBV>();
        assert_eq!(e.call(0x00ea_0640, &args![copy, other]).u32(), copy.addr());
        assert_eq!(e.mem.u32(copy.addr()), NI_CAPSULE_BV_VTABLE);
        assert_eq!(e.get(copy, NiCapsuleBV::m_kCapsule_radius), 0x7fc0_0001);
    }

    #[test]
    fn update_world_data_moves_center_turns_axis_and_scales() {
        let mut e = engine();
        let model = capsule(&mut e, [1.0, 2.0, 3.0], [1.0, 0.0, 0.0], 2.0, 0.5);
        // A quarter turn about z (rows (0,-1,0), (1,0,0), (0,0,1)), translation (10,0,0),
        // scale 2.
        let transform = e.mem.alloc(0x34);
        let rotation = [0.0f32, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (i, value) in rotation.iter().enumerate() {
            e.mem.set_f32(transform + 4 * i as u32, *value);
        }
        e.mem.set_f32(transform + 0x24, 10.0);
        e.mem.set_f32(transform + 0x30, 2.0);
        let this = e.new_object::<NiCapsuleBV>();
        e.call(0x00e9_f2c0, &args![this, model, transform]);
        assert_eq!(floats(&e, this.addr() + 4, 3), vec![6.0, 2.0, 6.0]);
        assert_eq!(floats(&e, this.addr() + 0x10, 3), vec![0.0, 1.0, 0.0]);
        assert_eq!(e.mem.f32(this.addr() + 0x1c), 4.0);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 1.0);
        // The segment: h = 3, origin center - 3 axis, direction 6 axis.
        assert_eq!(
            floats(&e, this.addr() + 0x20, 6),
            vec![6.0, -1.0, 6.0, 0.0, 6.0, 0.0]
        );
    }

    /// Doubles for an intersector run whose type is the word at `RESULT + 8`.
    fn intersector_doubles(e: &mut Engine, constructor: u32) -> (u32, u32) {
        let point = point3(e, [1.0, 2.0, 3.0]);
        let normal = point3(e, [0.0, -1.0, 0.0]);
        e.mem.set_f64(RESULT, 0.25);
        e.register(constructor, nothing);
        e.register(INTERSECTOR_FIND, nothing);
        e.register(INTERSECTOR_CONTACT_TIME, answer_result);
        e.register(INTERSECTOR_TYPE, |e, _| Ret {
            eax: e.mem.u32(RESULT + 8),
            ..Ret::default()
        });
        e.register(INTERSECTOR_DESTRUCTOR, nothing);
        e.register_double(INTERSECTOR_CONTACT_POINT, move |_, _| Ret {
            eax: point,
            ..Ret::default()
        });
        e.register_double(INTERSECTOR_CONTACT_NORMAL, move |_, _| Ret {
            eax: normal,
            ..Ret::default()
        });
        (point, normal)
    }

    #[test]
    fn fn_00e9f400_and_00e9f510_run_their_intersectors() {
        for (address, constructor) in [
            (0x00e9_f400u32, INTERSECTOR_CONSTRUCTOR_00EAE9E0),
            (0x00e9_f510, INTERSECTOR_CONSTRUCTOR_00EAEDC0),
        ] {
            let mut e = engine();
            intersector_doubles(&mut e, constructor);
            let outs = e.mem.alloc(0x40);
            e.mem.set_u32(RESULT + 8, 3);
            let words = args![
                1.5f32,
                0x10u32,
                0x11u32,
                0x12u32,
                0x13u32,
                outs,
                outs + 4,
                true,
                outs + 0x10,
                outs + 0x1c
            ];
            assert!(e.call(address, &words).bool());
            let made = logged(&e, constructor);
            assert_eq!(
                made[0][1..],
                [
                    0x10,
                    0x12,
                    1.5f32.to_bits(),
                    0.001f32.to_bits(),
                    0.0001f32.to_bits(),
                    0x20
                ]
            );
            assert_eq!(logged(&e, INTERSECTOR_FIND)[0][1..], [0x11, 0x13]);
            assert_eq!(e.mem.f32(outs), 0.25);
            assert_eq!(floats(&e, outs + 4, 3), vec![1.0, 2.0, 3.0]);
            assert_eq!(floats(&e, outs + 0x10, 3), vec![0.0, -1.0, 0.0]);
            assert_eq!(e.mem.u32(outs + 0x1c), 0x8000_0000);
            assert_eq!(e.mem.f32(outs + 0x20), 1.0);
            assert_eq!(logged(&e, INTERSECTOR_DESTRUCTOR).len(), 1);
            // Type 1: no contact, the point is untouched.
            e.mem.set_u32(RESULT + 8, 1);
            e.mem.set_u32(outs + 4, 0);
            assert!(!e.call(address, &words).bool());
            assert_eq!(e.mem.u32(outs + 4), 0);
            assert_eq!(logged(&e, INTERSECTOR_DESTRUCTOR).len(), 2);
        }
    }

    #[test]
    fn capsule_sphere_test_picks_the_distance_by_motion() {
        let mut e = engine();
        e.register(SQR_DISTANCE_POINT_SEGMENT, answer_result);
        e.register(SQR_DISTANCE_SEGMENT_SEGMENT, answer_result);
        let this = capsule(&mut e, [0.0; 3], [1.0, 0.0, 0.0], 2.0, 0.5);
        let sphere = e.mem.alloc(0x14);
        e.mem.set_f32(sphere + 4, 3.0);
        e.mem.set_f32(sphere + 0x10, 1.0);
        let still = point3(&mut e, [1.0, 1.0, 1.0]);
        // Equal velocities: the point distance; 2.25 is (0.5 + 1)^2.
        e.mem.set_f64(RESULT, 2.25);
        assert!(e
            .call(0x00e9_f620, &args![2.0f32, this, still, sphere, still])
            .bool());
        let calls = logged(&e, SQR_DISTANCE_POINT_SEGMENT);
        assert_eq!(calls[0][..2], [sphere + 4, this.addr() + 0x20]);
        e.mem.set_f64(RESULT, 2.5);
        assert!(!e
            .call(0x00e9_f620, &args![2.0f32, this, still, sphere, still])
            .bool());
        // A relative velocity (0, 0.5, 0) over time 2: the segment from the center along (0,1,0).
        let moving = point3(&mut e, [1.0, 1.5, 1.0]);
        e.register(SQR_DISTANCE_SEGMENT_SEGMENT, |e, a| {
            let segment = floats(e, a[0], 6);
            assert_eq!(segment, vec![3.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        assert!(e
            .call(0x00e9_f620, &args![2.0f32, this, still, sphere, moving])
            .bool());
        assert_eq!(
            logged(&e, SQR_DISTANCE_SEGMENT_SEGMENT)[0][1],
            this.addr() + 0x20
        );
    }

    #[test]
    fn fn_00e9f760_tests_two_capsules() {
        let mut e = engine();
        e.register(SQR_DISTANCE_SEGMENT_SEGMENT, answer_result);
        let first = capsule(&mut e, [0.0; 3], [1.0, 0.0, 0.0], 2.0, 0.5);
        let second = capsule(&mut e, [0.0, 3.0, 0.0], [1.0, 0.0, 0.0], 2.0, 0.25);
        let still = point3(&mut e, [0.0; 3]);
        e.mem.set_f64(RESULT, 0.5625);
        assert!(e
            .call(0x00e9_f760, &args![1.0f32, first, still, second, still])
            .bool());
        assert_eq!(
            logged(&e, SQR_DISTANCE_SEGMENT_SEGMENT)[0][..2],
            [first.addr() + 0x20, second.addr() + 0x20]
        );
        e.register(SQR_DISTANCE_SEGMENT_MOVING_SEGMENT, |e, a| {
            let block = floats(e, a[1], 9);
            assert_eq!(block[6..], [0.0, -2.0, 0.0]);
            Ret {
                st0: 0.6,
                ..Ret::default()
            }
        });
        let moving = point3(&mut e, [0.0, -1.0, 0.0]);
        assert!(!e
            .call(0x00e9_f760, &args![2.0f32, first, still, second, moving])
            .bool());
        let calls = logged(&e, SQR_DISTANCE_SEGMENT_MOVING_SEGMENT);
        assert_eq!(calls[0][0], first.addr() + 0x20);
    }

    /// The triangle (0,0,0), (1,0,0), (0,1,0) in the plane z = 0.
    fn triangle(e: &mut Engine) -> [u32; 3] {
        [
            point3(e, [0.0, 0.0, 0.0]),
            point3(e, [1.0, 0.0, 0.0]),
            point3(e, [0.0, 1.0, 0.0]),
        ]
    }

    #[test]
    fn fn_00e9f8c0_rejects_early_or_measures_the_segment() {
        let mut e = engine();
        e.register(UNITIZE, nothing);
        e.register(SQR_DISTANCE_SEGMENT_TRIANGLE, answer_result);
        e.register(SQR_DISTANCE_TRIANGLE_MOVING_SEGMENT, answer_result);
        let [v0, v1, v2] = triangle(&mut e);
        let still = point3(&mut e, [0.0; 3]);
        // Above the plane, moving away: rejected without a distance.
        let above = capsule(&mut e, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 1.0, 0.5);
        let up = point3(&mut e, [0.0, 0.0, 1.0]);
        assert!(!e
            .call(0x00e9_f8c0, &args![1.0f32, above, up, v0, v1, v2, still])
            .bool());
        assert_eq!(logged(&e, UNITIZE).len(), 1);
        // Below the plane moving up, but not far enough in the time: rejected.
        let below = capsule(&mut e, [0.0, 0.0, -5.0], [0.0, 0.0, 1.0], 1.0, 0.5);
        assert!(!e
            .call(0x00e9_f8c0, &args![1.0f32, below, up, v0, v1, v2, still])
            .bool());
        assert!(logged(&e, SQR_DISTANCE_TRIANGLE_MOVING_SEGMENT).is_empty());
        // Far enough: the moving segment's distance decides (0.1 <= 0.25).
        e.mem.set_f64(RESULT, 0.1);
        assert!(e
            .call(0x00e9_f8c0, &args![10.0f32, below, up, v0, v1, v2, still])
            .bool());
        let call = &logged(&e, SQR_DISTANCE_TRIANGLE_MOVING_SEGMENT)[0];
        assert_eq!(
            floats(&e, call[0], 9),
            vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        // Near the plane and still: the plain segment distance, 1.0 > 0.25.
        let near = capsule(&mut e, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0], 1.0, 0.5);
        e.mem.set_f64(RESULT, 1.0);
        assert!(!e
            .call(0x00e9_f8c0, &args![1.0f32, near, still, v0, v1, v2, still])
            .bool());
        let call = &logged(&e, SQR_DISTANCE_SEGMENT_TRIANGLE)[0];
        assert_eq!(call[0], near.addr() + 0x20);
    }

    #[test]
    fn fn_00e9fdc0_reports_a_touch_or_runs_the_intersector() {
        let mut e = engine();
        e.register(UNITIZE, nothing);
        // The segment-triangle distance: 0 with s = 0.25, t = 0.5.
        e.register(SQR_DISTANCE_SEGMENT_TRIANGLE, |e, a| {
            e.mem.set_f32(a[3], 0.25);
            e.mem.set_f32(a[4], 0.5);
            Ret {
                st0: e.mem.f64(DISTANCE),
                ..Ret::default()
            }
        });
        let normal = point3(&mut e, [0.0, 0.0, 1.0]);
        e.register_double(NI_TRIGON_GET_NORMAL, move |_, _| Ret {
            eax: normal,
            ..Ret::default()
        });
        let [v0, v1, v2] = triangle(&mut e);
        let still = point3(&mut e, [0.0; 3]);
        let this = capsule(&mut e, [0.0, 0.0, 0.2], [0.0, 0.0, 1.0], 1.0, 0.5);
        let outs = e.mem.alloc(0x40);
        e.mem.set_f32(outs, 9.0);
        let words = |time: f32, velocity: u32| {
            args![
                time,
                this,
                velocity,
                v0,
                v1,
                v2,
                still,
                outs,
                outs + 4,
                true,
                outs + 0x10,
                outs + 0x1c
            ]
        };
        assert!(e.call(0x00e9_fdc0, &words(1.0, still)).bool());
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(floats(&e, outs + 4, 3), vec![0.25, 0.5, 0.0]);
        assert_eq!(floats(&e, outs + 0x1c, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(floats(&e, outs + 0x10, 3), vec![-0.0, -0.0, -1.0]);
        // Clear of it and still: no motion, false.
        e.mem.set_f64(DISTANCE, 4.0);
        assert!(!e.call(0x00e9_fdc0, &words(1.0, still)).bool());
        // Moving sideways (no plane rejection): the intersector decides, type 2.
        intersector_doubles(&mut e, CAPSULE_TRI_INTERSECTOR_CONSTRUCTOR);
        e.mem.set_u32(RESULT + 8, 2);
        let sideways = point3(&mut e, [1.0, 0.0, 0.0]);
        assert!(e.call(0x00e9_fdc0, &words(1.0, sideways)).bool());
        let made = &logged(&e, CAPSULE_TRI_INTERSECTOR_CONSTRUCTOR)[0];
        assert_eq!(made[1..5], [this.addr(), v0, v1, v2]);
        assert_eq!(made[8], 0x20);
        assert_eq!(e.mem.f32(outs), 0.25);
        assert_eq!(floats(&e, outs + 4, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(floats(&e, outs + 0x1c, 3), vec![-0.0, 1.0, -0.0]);
    }

    #[test]
    fn the_loader_makes_a_default_capsule_and_loads_it() {
        let mut e = engine();
        e.mem.set_u32(NI_CAPSULE_BV_VTABLE, 0x0500_0000);
        e.register(0x0500_0000, nothing);
        let volume = e.call(0x00ea_04b0, &args![0x9000u32]).u32();
        assert_eq!(logged(&e, 0x0500_0000), vec![vec![volume, 0x9000]]);
        assert_eq!(floats(&e, volume + 0x10, 3), vec![1.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(volume + 0x1c), 1.0);
        assert_eq!(e.mem.f32(volume + 0x38), 1.0);
        // h = 0: the segment sits at the center with no length.
        assert_eq!(floats(&e, volume + 0x20, 6), vec![0.0; 6]);
    }

    #[test]
    fn load_and_save_binary_move_the_center_axis_extent_and_radius() {
        let mut e = engine();
        for address in [
            NI_BOUNDING_VOLUME_LOAD_BINARY,
            NI_BOUNDING_VOLUME_SAVE_BINARY,
            LOAD_THREE_FLOATS,
            SAVE_THREE_FLOATS,
        ] {
            e.register(address, nothing);
        }
        // The read writes 1.5; both answer their size.
        e.register(0x0500_0000, |e, a| {
            e.mem.set_f32(a[1], 1.5);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        e.register(0x0500_0010, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let io = e.mem.alloc(0x10);
        e.mem.set_u32(io + 8, 0x0500_0000);
        e.mem.set_u32(io + 0xc, 0x0500_0010);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        let this = e.new_object::<NiCapsuleBV>();
        e.call(0x00ea_0540, &args![this, stream]);
        assert_eq!(
            logged(&e, LOAD_THREE_FLOATS),
            vec![
                vec![this.addr() + 4, stream],
                vec![this.addr() + 0x10, stream]
            ]
        );
        let reads: Vec<u32> = logged(&e, 0x0500_0000).iter().map(|c| c[1]).collect();
        assert_eq!(reads, vec![this.addr() + 0x1c, this.addr() + 0x38]);
        assert_eq!(e.mem.u32(io + 4), 8);
        e.call(0x00ea_05c0, &args![this, stream]);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(logged(&e, SAVE_THREE_FLOATS).len(), 2);
        let writes: Vec<u32> = logged(&e, 0x0500_0010).iter().map(|c| c[1]).collect();
        assert_eq!(writes, vec![this.addr() + 0x1c, this.addr() + 0x38]);
    }

    #[test]
    fn create_and_clone_allocate_0x3c_bytes() {
        let mut e = engine();
        let created = e.call(0x00ea_0690, &args![]).u32();
        assert_eq!(e.mem.u32(created), NI_CAPSULE_BV_VTABLE);
        assert_eq!(e.mem.f32(created + 0x38), 1.0);
        let other = capsule(&mut e, [1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 2.0, 0.5);
        let copy = e.call(0x00ea_0700, &args![other]).u32();
        assert_eq!(floats(&e, copy + 4, 14), floats(&e, other.addr() + 4, 14));
        assert_eq!(
            logged(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x3c], vec![MEMORY_MANAGER, 0x3c]]
        );
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ea_0690, &args![]).u32(), 0);
        assert_eq!(e.call(0x00ea_0700, &args![other]).u32(), 0);
    }

    #[test]
    fn add_viewer_strings_appends_one_line() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_SPRINTF, nothing);
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, nothing);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"ab\0");
        let strings = e.mem.alloc(0x10);
        e.call(0x00ea_0730, &args![0u32, name, strings]);
        let lines = logged(&e, NI_SPRINTF);
        assert_eq!(lines[0][1..], [0x10, CAPSULE_BV_FORMAT, name]);
        assert_eq!(logged(&e, VIEWER_STRINGS_GROW), vec![vec![strings, 0]]);
        assert_eq!(logged(&e, VIEWER_STRINGS_SET_AT).len(), 1);
    }
}
