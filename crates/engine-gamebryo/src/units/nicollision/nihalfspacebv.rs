//! `NiCollision/nihalfspacebv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, float_operand, load_float,
};
use crate::units::nicollision::nispherebv::negated_bits;

// Translated from 00e9e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nihalfspacebv.cpp`): a virtual slot (referenced only
/// from the vtables at `0109c5a0`, `010cb3d0` and `010f1734`, so `this` arrives in `ECX`
/// and is not read) that answers 5. Shared by identical-code folding; which method of
/// which class each slot is was not confirmed.
pub fn fn_00e9e670(_e: &mut Engine, _this: Ptr) -> u32 {
    5
}

layout! {
    /// `NiHalfSpaceBV` (Xbox PDB), 0x20 bytes: the vtable, `m_kPlane` (an `NiPlane`: the normal,
    /// then the constant) and `m_kCenter`, declared as words because most of the code only moves
    /// them. The half space is the side of the plane the normal points to (`n . x >= c`, by the
    /// tests below).
    pub struct NiHalfSpaceBV: 0x20 {
        /// `m_kPlane.m_kNormal` (Xbox PDB), x.
        0x04 m_kPlane_normal_x: u32,
        /// `m_kPlane.m_kNormal` (Xbox PDB), y.
        0x08 m_kPlane_normal_y: u32,
        /// `m_kPlane.m_kNormal` (Xbox PDB), z.
        0x0C m_kPlane_normal_z: u32,
        /// `m_kPlane.m_fConstant` (Xbox PDB).
        0x10 m_kPlane_constant: u32,
        /// `m_kCenter` (Xbox PDB), x.
        0x14 m_kCenter_x: u32,
        /// `m_kCenter` (Xbox PDB), y.
        0x18 m_kCenter_y: u32,
        /// `m_kCenter` (Xbox PDB), z.
        0x1C m_kCenter_z: u32,
    }
}

/// `NiHalfSpaceBV`'s vtable (`010f1728`).
const NI_HALF_SPACE_BV_VTABLE: u32 = 0x010f_1728;
/// The memory manager instance and its `Allocate` (`00aa3e40`, thiscall: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator `00aa1070` (cdecl, size).
const ALLOCATE: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The format of the viewer line (`010f1758`, `"%s = HALFSPACE_BV"` by its use).
const HALF_SPACE_BV_FORMAT: u32 = 0x010f_1758;
/// `NiPlane::NiPlane` (`00a69940`, thiscall, unit `NiMain/niplane.cpp`).
const NI_PLANE_CONSTRUCTOR: u32 = 0x00a6_9940;
/// `NiPoint3::UnitizeGetLength` (`00457910`, thiscall on the point): normalizes it in place and
/// answers its length in ST0.
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `00a92660` (thiscall on four floats, the other four floats; the engine map has no name):
/// the four-float equality `NiSphereBV::operator==` uses for its sphere, here on the planes.
const FOUR_FLOATS_EQUAL: u32 = 0x00a9_2660;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`) and `NiBoundingVolume::SaveBinary`
/// (`00e9cb10`), thiscall, the stream.
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// `00a69ac0` / `00eb5410` (the engine map's `NiSphere::LoadBinary` / `SaveBinary`: four floats,
/// shared by identical-code folding), here on the plane.
const LOAD_FOUR_FLOATS: u32 = 0x00a6_9ac0;
const SAVE_FOUR_FLOATS: u32 = 0x00eb_5410;
/// `NiColor::LoadBinary` / `NiColor::SaveBinary` (`00a59be0` / `00a69780`, three floats), here
/// on the center.
const LOAD_THREE_FLOATS: u32 = 0x00a5_9be0;
const SAVE_THREE_FLOATS: u32 = 0x00a6_9780;
/// The three words at `011f426c` (zero in the image) and `011a9478` that `Create` and the
/// stream loader give the constructor as the point and the normal.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
const DEFAULT_NORMAL_011A9478: u32 = 0x011a_9478;
/// The double at `010cb650` (about -0.999999) the two normals' dot product is compared with.
const OPPOSITE_NORMALS_010CB650: u32 = 0x010c_b650;
/// The `float` `0.001` at `01017d00`: the box find's threshold for an axis to count.
const AXIS_THRESHOLD_01017D00: u32 = 0x0101_7d00;

/// `FLD dword [a]` followed by `FMUL dword [b]`.
fn product(e: &Engine, a: u32, b: u32) -> f64 {
    x87_mul(load_float(e, a), float_operand(e, b))
}

/// Three products summed in order (`FADDP` after the second and the third), stored as a float.
fn sum3(products: [f64; 3]) -> u32 {
    x87_store32(x87_add(x87_add(products[0], products[1]), products[2]))
}

/// `FLD`, `FSUB dword`, `FSTP` of each component: `a - b` as three floats.
fn difference(e: &Engine, a: u32, b: u32) -> [u32; 3] {
    let mut out = [0u32; 3];
    for (axis, slot) in out.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        *slot = x87_store32(x87_sub(
            load_float(e, a.wrapping_add(offset)),
            float_operand(e, b.wrapping_add(offset)),
        ));
    }
    out
}

/// The float `dot - constant`: the signed distance of a point from the plane, from the stored
/// dot product (`FLD`, `FSUB dword [plane + 0x10]`, `FSTP`).
fn plane_distance(e: &Engine, plane_volume: u32, dot: u32) -> u32 {
    x87_store32(x87_sub(
        x87_load32(dot),
        float_operand(e, plane_volume.wrapping_add(0x10)),
    ))
}

/// The bits `FLD`, `FABS` and `FSTP` leave for a float.
fn abs_bits(bits: u32) -> u32 {
    x87_quiet_bits(bits) & 0x7fff_ffff
}

/// Writes three words at `out`.
fn store_point(e: &mut Engine, out: u32, words: [u32; 3]) {
    for (axis, word) in words.iter().enumerate() {
        e.mem.set_u32(out.wrapping_add(4 * axis as u32), *word);
    }
}

/// The normals the finds hand back: the plane's normal into `second_normal`, and its negation
/// (each word read back from `second_normal`, `FLD`/`FCHS`/`FSTP`) into `first_normal`.
fn plane_normals(e: &mut Engine, half_space: u32, first_normal: u32, second_normal: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(half_space.wrapping_add(4 + offset));
        e.mem.set_u32(second_normal.wrapping_add(offset), word);
    }
    let negated =
        [0u32, 4, 8].map(|offset| negated_bits(e.mem.u32(second_normal.wrapping_add(offset))));
    store_point(e, first_normal, negated);
}

/// The common end of the moving tests: with `speed` the stored normal component of the relative
/// velocity, false unless it is positive (unordered goes on), then whether `reach` is at most
/// `distance + speed * time` (the time a `float` memory operand).
fn reaches_in_time(time: u32, distance: f64, reach: f64, speed: u32) -> bool {
    let speed = x87_load32(speed);
    if speed <= 0.0 {
        return false;
    }
    reach <= x87_add(distance, x87_mul(speed, x87_m32(time)))
}

// Translated from 00e9cb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::LoadBinary` (Xbox PDB): the base part (`00a89af0`), the plane's four floats
/// (`00a69ac0` on +4) and the center's three (`00a59be0` on +0x14).
pub fn ni_half_space_bv_load_binary(e: &mut Engine, this: Ptr<NiHalfSpaceBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.call(
        LOAD_FOUR_FLOATS,
        &args![this.addr().wrapping_add(4), stream],
    );
    e.call(
        LOAD_THREE_FLOATS,
        &args![this.addr().wrapping_add(0x14), stream],
    );
}

// Translated from 00e9cb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::SaveBinary` (Xbox PDB): the base part (`00e9cb10`), the plane's four floats
/// (`00eb5410` on +4) and the center's three (`00a69780` on +0x14).
pub fn ni_half_space_bv_save_binary(e: &mut Engine, this: Ptr<NiHalfSpaceBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.call(
        SAVE_FOUR_FLOATS,
        &args![this.addr().wrapping_add(4), stream],
    );
    e.call(
        SAVE_THREE_FLOATS,
        &args![this.addr().wrapping_add(0x14), stream],
    );
}

// Translated from 00e9cbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::SetPlane` (Xbox PDB): copies the plane's normal to a local, normalizes it
/// (`NiPoint3::UnitizeGetLength`, `00457910`) and stores it with the constant divided by the
/// length (the constant kept as a double, `FDIVR`, the quotient stored as a float).
pub fn ni_half_space_bv_set_plane(e: &mut Engine, this: Ptr<NiHalfSpaceBV>, plane: Ptr) {
    let p = plane.addr();
    let constant = load_float(e, p.wrapping_add(0xc));
    e.with_stack(12, |e, normal| {
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(p.wrapping_add(offset));
            e.mem.set_u32(normal.addr() + offset, word);
        }
        let length = e.call(UNITIZE_GET_LENGTH, &args![normal]).f64();
        let quotient = x87_store32(x87_div(constant, length));
        let x = e.mem.u32(normal.addr());
        e.set(this, NiHalfSpaceBV::m_kPlane_normal_x, x);
        let y = e.mem.u32(normal.addr() + 4);
        e.set(this, NiHalfSpaceBV::m_kPlane_normal_y, y);
        let z = e.mem.u32(normal.addr() + 8);
        e.set(this, NiHalfSpaceBV::m_kPlane_normal_z, z);
        e.set(
            this,
            NiHalfSpaceBV::m_kPlane_constant,
            x87_quiet_bits(quotient),
        );
    });
}

// Translated from 00e9cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::Copy` (Xbox PDB): `SetPlane` with the other volume's plane (+4); the center
/// is not copied.
pub fn ni_half_space_bv_copy(e: &mut Engine, this: Ptr<NiHalfSpaceBV>, other: Ptr<NiHalfSpaceBV>) {
    ni_half_space_bv_set_plane(e, this, Ptr::new(other.addr().wrapping_add(4)));
}

// Translated from 00e9cc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::operator==` (Xbox PDB): the other volume's `Type` (virtual slot +0xc) must be 5
/// and its plane (+4) must equal this one's (`00a92660`, called on the other volume's plane with
/// this one's as the argument).
pub fn ni_half_space_bv_operator_(
    e: &mut Engine,
    this: Ptr<NiHalfSpaceBV>,
    other: Ptr<NiHalfSpaceBV>,
) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 5 {
        return false;
    }
    e.call(
        FOUR_FLOATS_EQUAL,
        &args![other.addr().wrapping_add(4), this.addr().wrapping_add(4)],
    )
    .bool()
}

// Translated from 00e9cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::UpdateWorldData` (Xbox PDB): sets this plane to the model volume's plane moved
/// by `transform` (a row-major 3x3 rotation at +0, the translation at +0x24, the scale at +0x30).
/// Each normal component is a rotation row times the model normal (the row's second element
/// first), stored as a float; the constant is the translation dotted with the new normal
/// (stored) plus the scale times the model constant, stored as a float (through `FLD`/`FSTP`
/// once more). The center is not updated.
pub fn ni_half_space_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiHalfSpaceBV>,
    model: Ptr<NiHalfSpaceBV>,
    transform: Ptr,
) {
    let t = transform.addr();
    let m = model.addr();
    let mut normal = [0u32; 3];
    for (row, slot) in normal.iter_mut().enumerate() {
        let r = t.wrapping_add(12 * row as u32);
        *slot = sum3([
            product(e, r.wrapping_add(4), m.wrapping_add(8)),
            product(e, r, m.wrapping_add(4)),
            product(e, r.wrapping_add(8), m.wrapping_add(0xc)),
        ]);
    }
    let along = sum3([
        x87_mul(load_float(e, t.wrapping_add(0x28)), x87_m32(normal[1])),
        x87_mul(load_float(e, t.wrapping_add(0x24)), x87_m32(normal[0])),
        x87_mul(load_float(e, t.wrapping_add(0x2c)), x87_m32(normal[2])),
    ]);
    let scaled = product(e, t.wrapping_add(0x30), m.wrapping_add(0x10));
    let constant = x87_store32(x87_add(x87_load32(along), scaled));
    e.set(this, NiHalfSpaceBV::m_kPlane_normal_x, normal[0]);
    e.set(this, NiHalfSpaceBV::m_kPlane_normal_y, normal[1]);
    e.set(this, NiHalfSpaceBV::m_kPlane_normal_z, normal[2]);
    e.set(
        this,
        NiHalfSpaceBV::m_kPlane_constant,
        x87_quiet_bits(constant),
    );
}

// Translated from 00e9cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (test table entry half space x sphere, `cdecl`): whether the moving
/// sphere `sphere` (center +4, radius +0x10) touches the moving half space during `time`. With
/// `d` the center's signed distance from the plane (stored float) and `-r` the negated radius
/// (stored): true when `-r <= d`; otherwise, with `s` the normal component of the relative
/// velocity `sphere_velocity - velocity` (each component and the sum stored as floats), false
/// unless `s > 0` (unordered goes on), then whether `-r <= d + s * time`.
pub fn fn_00e9cd10(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    sphere: Ptr,
    sphere_velocity: Ptr,
) -> bool {
    let n = half_space.addr();
    let c = sphere.addr();
    let dot = sum3([
        product(e, c.wrapping_add(8), n.wrapping_add(8)),
        product(e, c.wrapping_add(4), n.wrapping_add(4)),
        product(e, c.wrapping_add(0xc), n.wrapping_add(0xc)),
    ]);
    let distance = x87_load32(plane_distance(e, n, dot));
    let reach = x87_load32(negated_bits(e.mem.u32(c.wrapping_add(0x10))));
    if reach <= distance {
        return true;
    }
    let relative = difference(e, sphere_velocity.addr(), velocity.addr());
    let speed = sum3([
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(x87_load32(relative[0]), float_operand(e, n.wrapping_add(4))),
        x87_mul(load_float(e, n.wrapping_add(0xc)), x87_m32(relative[2])),
    ]);
    reaches_in_time(time.to_bits(), distance, reach, speed)
}

// Translated from 00e9cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (test table entry half space x box, `cdecl`): as the sphere test,
/// with the box (center +4, axes +0x10/+0x1c/+0x28, extents +0x34..) reaching
/// `R = |e1 (a1 . n)| + |e0 (a0 . n)| + |e2 (a2 . n)|` toward the plane (each axis product and
/// absolute value stored as a float, summed in that order): true when `-R <= d`, otherwise
/// whether the relative motion along the normal brings `d` to `-R` within `time`.
pub fn fn_00e9cde0(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    box_volume: Ptr,
    box_velocity: Ptr,
) -> bool {
    let n = half_space.addr();
    let b = box_volume.addr();
    let mut projected = [0u32; 3];
    for (axis, slot) in projected.iter_mut().enumerate() {
        let a = b.wrapping_add(0x10 + 12 * axis as u32);
        let along = sum3([
            product(e, a.wrapping_add(4), n.wrapping_add(8)),
            product(e, a, n.wrapping_add(4)),
            product(e, a.wrapping_add(8), n.wrapping_add(0xc)),
        ]);
        *slot = x87_store32(x87_mul(
            x87_load32(along),
            float_operand(e, b.wrapping_add(0x34 + 4 * axis as u32)),
        ));
    }
    let first = abs_bits(projected[1]);
    let second = abs_bits(projected[0]);
    let third = abs_bits(projected[2]);
    let extent = x87_store32(x87_add(
        x87_add(x87_load32(first), x87_m32(second)),
        x87_m32(third),
    ));
    let dot = sum3([
        product(e, b.wrapping_add(8), n.wrapping_add(8)),
        product(e, b.wrapping_add(4), n.wrapping_add(4)),
        product(e, b.wrapping_add(0xc), n.wrapping_add(0xc)),
    ]);
    let distance = x87_load32(plane_distance(e, n, dot));
    let reach = x87_load32(negated_bits(extent));
    if reach <= distance {
        return true;
    }
    let relative = difference(e, box_velocity.addr(), velocity.addr());
    let speed = sum3([
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(x87_load32(relative[0]), float_operand(e, n.wrapping_add(4))),
        x87_mul(
            x87_load32(relative[2]),
            float_operand(e, n.wrapping_add(0xc)),
        ),
    ]);
    reaches_in_time(time.to_bits(), distance, reach, speed)
}

// Translated from 00e9cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (test table entry half space x capsule, `cdecl`): with `d0` the
/// capsule segment origin's signed distance (+0x20; stored), `d1 = (direction . n) + d0` that of
/// its other end (+0x2c; stored) and `-r` the negated radius (+0x38): true when `-r <= d0` or
/// `d1 >= -r`; otherwise, with `s` the normal component of the relative velocity, false unless
/// `s > 0`, then with `q = -(s * time + r)` (stored) whether `q <= d0` or `d1 >= q`.
pub fn fn_00e9cf50(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    capsule: Ptr,
    capsule_velocity: Ptr,
) -> bool {
    let n = half_space.addr();
    let c = capsule.addr();
    let dot = sum3([
        product(e, c.wrapping_add(0x24), n.wrapping_add(8)),
        product(e, c.wrapping_add(0x20), n.wrapping_add(4)),
        product(e, c.wrapping_add(0x28), n.wrapping_add(0xc)),
    ]);
    let start = x87_load32(plane_distance(e, n, dot));
    let reach = x87_load32(negated_bits(e.mem.u32(c.wrapping_add(0x38))));
    if reach <= start {
        return true;
    }
    let along = sum3([
        product(e, c.wrapping_add(0x30), n.wrapping_add(8)),
        product(e, c.wrapping_add(0x2c), n.wrapping_add(4)),
        product(e, c.wrapping_add(0x34), n.wrapping_add(0xc)),
    ]);
    let end = x87_load32(x87_store32(x87_add(x87_load32(along), start)));
    if end >= reach {
        return true;
    }
    let relative = difference(e, capsule_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(x87_load32(relative[0]), float_operand(e, n.wrapping_add(4))),
        x87_mul(load_float(e, n.wrapping_add(0xc)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let moved = x87_add(
        x87_mul(speed, x87_m32(time.to_bits())),
        float_operand(e, c.wrapping_add(0x38)),
    );
    let limit = x87_load32(x87_store32(-moved));
    limit <= start || end >= limit
}

// Translated from 00e9d070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (test table entry half space x half space, `cdecl`): true when the
/// normals' dot product (stored) is above the double at `010cb650` (about -0.999999: not
/// opposite), or when `-c2 >= c1`; otherwise, with `s` this normal's component of the relative
/// velocity (stored), false unless `s > 0`, then whether `c1 <= s * time - c2` (time loaded by
/// `FLD`).
pub fn fn_00e9d070(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    other: Ptr<NiHalfSpaceBV>,
    other_velocity: Ptr,
) -> bool {
    let a = half_space.addr();
    let b = other.addr();
    let dot = sum3([
        product(e, b.wrapping_add(8), a.wrapping_add(8)),
        product(e, a.wrapping_add(4), b.wrapping_add(4)),
        product(e, b.wrapping_add(0xc), a.wrapping_add(0xc)),
    ]);
    let limit = e.mem.f64(OPPOSITE_NORMALS_010CB650);
    if x87_load32(dot) > limit {
        return true;
    }
    let own_constant = load_float(e, a.wrapping_add(0x10));
    if -load_float(e, b.wrapping_add(0x10)) >= own_constant {
        return true;
    }
    let relative = difference(e, other_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, a.wrapping_add(4)), x87_m32(relative[0])),
        x87_mul(load_float(e, a.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(load_float(e, a.wrapping_add(0xc)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let constant = load_float(e, a.wrapping_add(0x10));
    let moved = x87_mul(speed, x87_load32(time.to_bits()));
    let other_constant = load_float(e, b.wrapping_add(0x10));
    constant <= x87_sub(moved, other_constant)
}

// Translated from 00e9d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (triangle test table entry of the half space, `cdecl`): true when
/// one of the vertices' signed distances `d0`, `d1`, `d2` (stored floats) is at least 0, tested
/// in order; otherwise, with `s` the normal component of `triangle_velocity - velocity`
/// (stored), false unless `s > 0`, then with `m = s * -time` (stored) whether `m <= d0`,
/// `d1 >= m` or `m <= d2`.
pub fn fn_00e9d150(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    let n = half_space.addr();
    let distance = |e: &Engine, p: u32| {
        let dot = sum3([
            product(e, p.wrapping_add(4), n.wrapping_add(8)),
            product(e, p, n.wrapping_add(4)),
            product(e, p.wrapping_add(8), n.wrapping_add(0xc)),
        ]);
        x87_load32(plane_distance(e, n, dot))
    };
    let first = distance(e, vertex0.addr());
    if first >= 0.0 {
        return true;
    }
    let second = distance(e, vertex1.addr());
    if second >= 0.0 {
        return true;
    }
    let third = distance(e, vertex2.addr());
    if third >= 0.0 {
        return true;
    }
    let relative = difference(e, triangle_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(load_float(e, n.wrapping_add(4)), x87_m32(relative[0])),
        x87_mul(load_float(e, n.wrapping_add(0xc)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let moved = x87_mul(speed, -x87_load32(time.to_bits()));
    let moved = x87_load32(x87_store32(moved));
    moved <= first || second >= moved || moved <= third
}

// Translated from 00e9d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (find table entry half space x sphere, `cdecl`, ten words): when
/// and where the moving sphere meets the half space. With `d` the center's signed distance and
/// `-r` the negated radius (stored floats):
/// - `-r <= d` (already touching): the time is 0, the point is the center minus `d` times the
///   normal (each product and difference stored), and with `calculate_normals` the normals are
///   the plane's (`second_normal`) and its negation (`first_normal`). True.
/// - otherwise, with `s` the normal component of `sphere_velocity - velocity` (stored): false
///   unless `s > 0` and `s * time + d >= -r` (unordered fails); then the time is
///   `-((d + r) / s)`, the normals as above when asked, and the point is the center plus the
///   time times `sphere_velocity`, plus `r` times the words in `second_normal` (read also when
///   the normals were not written). True.
pub fn fn_00e9d2c0(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    sphere: Ptr,
    sphere_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: bool,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let n = half_space.addr();
    let c = sphere.addr();
    let dot = sum3([
        product(e, c.wrapping_add(8), n.wrapping_add(8)),
        product(e, n.wrapping_add(4), c.wrapping_add(4)),
        product(e, c.wrapping_add(0xc), n.wrapping_add(0xc)),
    ]);
    let distance = x87_load32(plane_distance(e, n, dot));
    let reach = x87_load32(negated_bits(e.mem.u32(c.wrapping_add(0x10))));
    if reach <= distance {
        e.mem.set_u32(out_time.addr(), 0);
        let offset = [
            x87_store32(x87_mul(load_float(e, n.wrapping_add(4)), distance)),
            x87_store32(x87_mul(load_float(e, n.wrapping_add(8)), distance)),
            x87_store32(x87_mul(distance, float_operand(e, n.wrapping_add(0xc)))),
        ];
        let mut point = [0u32; 3];
        for (axis, slot) in point.iter_mut().enumerate() {
            *slot = x87_store32(x87_sub(
                load_float(e, c.wrapping_add(4 + 4 * axis as u32)),
                x87_m32(offset[axis]),
            ));
        }
        store_point(e, out_point.addr(), point);
        if calculate_normals {
            plane_normals(e, n, first_normal.addr(), second_normal.addr());
        }
        return true;
    }
    let relative = difference(e, sphere_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, n.wrapping_add(4)), x87_m32(relative[0])),
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(load_float(e, n.wrapping_add(0xc)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let moved = x87_add(x87_mul(speed, x87_m32(time.to_bits())), distance);
    if !(moved >= reach) {
        return false;
    }
    let total = x87_add(distance, load_float(e, c.wrapping_add(0x10)));
    let contact = x87_store32(-x87_div(total, speed));
    e.mem.set_u32(out_time.addr(), contact);
    if calculate_normals {
        plane_normals(e, n, first_normal.addr(), second_normal.addr());
    }
    finish_contact_point(
        e,
        c,
        sphere_velocity.addr(),
        out_time.addr(),
        out_point.addr(),
        second_normal.addr(),
    );
    true
}

/// The sphere find's contact point after a moving contact: `radius` times the words at
/// `second_normal`, the contact time (read back from `out_time`) times the velocity, and the
/// center plus that motion, then the sum (each product and sum stored as a float, in the exe's
/// operand order).
fn finish_contact_point(
    e: &mut Engine,
    sphere: u32,
    sphere_velocity: u32,
    out_time: u32,
    out_point: u32,
    second_normal: u32,
) {
    let radius = load_float(e, sphere.wrapping_add(0x10));
    let reach = [
        x87_store32(x87_mul(radius, float_operand(e, second_normal))),
        x87_store32(x87_mul(
            load_float(e, second_normal.wrapping_add(4)),
            radius,
        )),
        x87_store32(x87_mul(
            radius,
            float_operand(e, second_normal.wrapping_add(8)),
        )),
    ];
    let contact = load_float(e, out_time);
    let motion = [
        x87_store32(x87_mul(load_float(e, sphere_velocity), contact)),
        x87_store32(x87_mul(
            contact,
            float_operand(e, sphere_velocity.wrapping_add(4)),
        )),
        x87_store32(x87_mul(
            contact,
            float_operand(e, sphere_velocity.wrapping_add(8)),
        )),
    ];
    let moved = [
        x87_store32(x87_add(
            x87_load32(motion[0]),
            float_operand(e, sphere.wrapping_add(4)),
        )),
        x87_store32(x87_add(
            load_float(e, sphere.wrapping_add(8)),
            x87_m32(motion[1]),
        )),
        x87_store32(x87_add(
            load_float(e, sphere.wrapping_add(0xc)),
            x87_m32(motion[2]),
        )),
    ];
    let mut point = [0u32; 3];
    for axis in 0..3 {
        point[axis] = x87_store32(x87_add(x87_load32(moved[axis]), x87_m32(reach[axis])));
        e.mem
            .set_u32(out_point.wrapping_add(4 * axis as u32), point[axis]);
    }
}

// Translated from 00e9d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (find table entry half space x box, `cdecl`, ten words). With
/// `p_i = e_i (a_i . n)` the box's axis projections (stored), `R = |p0| + |p1| + |p2|` and `d`
/// the center's signed distance:
/// - `-R <= d`: time 0, the point is the center minus `d` times the normal, the normals (when
///   asked) the plane's into `second_normal` and its negation into `first_normal`. True.
/// - otherwise false unless the normal component `s` of the relative velocity is positive and
///   `s * time + d >= -R`; then the time is `-((R + d) / s)`, the point starts as the center
///   moved by the time times `box_velocity`, and for each axis whose `|p_i|` is above `0.001`
///   (`01017d00`) the extent times the axis is added when `p_i > 0` and subtracted otherwise
///   (unordered subtracts). With `calculate_normals` the first normal is the negated plane normal
///   and the second the point minus the center. True.
pub fn fn_00e9d540(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    box_volume: Ptr,
    box_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: bool,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let n = half_space.addr();
    let b = box_volume.addr();
    let axis_dot = |e: &Engine, axis: u32, swapped: bool| {
        let a = b.wrapping_add(0x10 + 12 * axis);
        let middle = if swapped {
            product(e, a, n.wrapping_add(4))
        } else {
            product(e, n.wrapping_add(4), a)
        };
        let along = sum3([
            product(e, a.wrapping_add(4), n.wrapping_add(8)),
            middle,
            product(e, a.wrapping_add(8), n.wrapping_add(0xc)),
        ]);
        x87_store32(x87_mul(
            x87_load32(along),
            float_operand(e, b.wrapping_add(0x34 + 4 * axis)),
        ))
    };
    let projected = [
        axis_dot(e, 0, false),
        axis_dot(e, 1, false),
        axis_dot(e, 2, true),
    ];
    let magnitude1 = abs_bits(projected[1]);
    let first = x87_load32(projected[0]);
    let magnitude0 = x87_store32(first.abs());
    let magnitude2 = abs_bits(projected[2]);
    let magnitude0 = x87_load32(magnitude0);
    let extent = x87_store32(x87_add(
        x87_add(magnitude0, x87_m32(magnitude1)),
        x87_m32(magnitude2),
    ));
    let dot = sum3([
        product(e, b.wrapping_add(8), n.wrapping_add(8)),
        product(e, n.wrapping_add(4), b.wrapping_add(4)),
        product(e, b.wrapping_add(0xc), n.wrapping_add(0xc)),
    ]);
    let distance = x87_load32(plane_distance(e, n, dot));
    let extent = x87_load32(extent);
    let reach = x87_load32(x87_store32(-extent));
    if reach <= distance {
        e.mem.set_u32(out_time.addr(), 0);
        let offset = [
            x87_store32(x87_mul(load_float(e, n.wrapping_add(4)), distance)),
            x87_store32(x87_mul(load_float(e, n.wrapping_add(8)), distance)),
            x87_store32(x87_mul(distance, float_operand(e, n.wrapping_add(0xc)))),
        ];
        let mut point = [0u32; 3];
        for (axis, slot) in point.iter_mut().enumerate() {
            *slot = x87_store32(x87_sub(
                load_float(e, b.wrapping_add(4 + 4 * axis as u32)),
                x87_m32(offset[axis]),
            ));
        }
        store_point(e, out_point.addr(), point);
        if calculate_normals {
            plane_normals(e, n, first_normal.addr(), second_normal.addr());
        }
        return true;
    }
    let relative = difference(e, box_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, n.wrapping_add(4)), x87_m32(relative[0])),
        x87_mul(load_float(e, n.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(
            x87_load32(relative[2]),
            float_operand(e, n.wrapping_add(0xc)),
        ),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let moved = x87_add(x87_mul(speed, x87_m32(time.to_bits())), distance);
    if !(moved >= reach) {
        return false;
    }
    let contact = x87_store32(-x87_div(x87_add(extent, distance), speed));
    e.mem.set_u32(out_time.addr(), contact);
    let contact = x87_load32(contact);
    let v = box_velocity.addr();
    let motion = [
        x87_store32(x87_mul(load_float(e, v), contact)),
        x87_store32(x87_mul(load_float(e, v.wrapping_add(4)), contact)),
        x87_store32(x87_mul(contact, float_operand(e, v.wrapping_add(8)))),
    ];
    let out = out_point.addr();
    let point = [
        x87_store32(x87_add(
            load_float(e, b.wrapping_add(4)),
            x87_m32(motion[0]),
        )),
        x87_store32(x87_add(
            x87_load32(motion[1]),
            float_operand(e, b.wrapping_add(8)),
        )),
        x87_store32(x87_add(
            x87_load32(motion[2]),
            float_operand(e, b.wrapping_add(0xc)),
        )),
    ];
    store_point(e, out, point);
    let threshold = load_float(e, AXIS_THRESHOLD_01017D00);
    if threshold < magnitude0 {
        push_along_axis(e, b, out, 0, 0.0 < first);
    }
    if threshold < x87_m32(magnitude1) {
        push_along_axis(e, b, out, 1, 0.0 < x87_m32(projected[1]));
    }
    if threshold < x87_m32(magnitude2) {
        push_along_axis(e, b, out, 2, 0.0 < x87_m32(projected[2]));
    }
    if calculate_normals {
        let negated = [4u32, 8, 0xc].map(|offset| negated_bits(e.mem.u32(n.wrapping_add(offset))));
        store_point(e, first_normal.addr(), negated);
        let normal = difference(e, out, b.wrapping_add(4));
        store_point(e, second_normal.addr(), normal);
    }
    true
}

/// The box find's step along axis `axis`: the extent (`FLD`/`FSTP` of it first) times the axis
/// added to the point at `out` when `add`, subtracted otherwise, each product and sum stored as a
/// float in the exe's operand order.
fn push_along_axis(e: &mut Engine, box_volume: u32, out: u32, axis: u32, add: bool) {
    let a = box_volume.wrapping_add(0x10 + 12 * axis);
    let extent = x87_load32(x87_quiet_bits(
        e.mem.u32(box_volume.wrapping_add(0x34 + 4 * axis)),
    ));
    if add {
        let offset = [
            x87_store32(x87_mul(load_float(e, a), extent)),
            x87_store32(x87_mul(load_float(e, a.wrapping_add(4)), extent)),
            x87_store32(x87_mul(extent, float_operand(e, a.wrapping_add(8)))),
        ];
        for component in 0..2u32 {
            let slot = out.wrapping_add(4 * component);
            let sum = x87_store32(x87_add(
                load_float(e, slot),
                x87_m32(offset[component as usize]),
            ));
            e.mem.set_u32(slot, sum);
        }
        let slot = out.wrapping_add(8);
        let sum = x87_store32(x87_add(x87_load32(offset[2]), float_operand(e, slot)));
        e.mem.set_u32(slot, sum);
    } else {
        let offset = [
            x87_store32(x87_mul(extent, float_operand(e, a))),
            x87_store32(x87_mul(load_float(e, a.wrapping_add(4)), extent)),
            x87_store32(x87_mul(extent, float_operand(e, a.wrapping_add(8)))),
        ];
        for component in 0..3u32 {
            let slot = out.wrapping_add(4 * component);
            let rest = x87_store32(x87_sub(
                load_float(e, slot),
                x87_m32(offset[component as usize]),
            ));
            e.mem.set_u32(slot, rest);
        }
    }
}

/// The sphere volume constructor from a radius and a center (`00eab8a0`, thiscall).
const SPHERE_BV_CONSTRUCTOR: u32 = 0x00ea_b8a0;
/// Unnamed in the engine map (`0045bb20`, thiscall on a point: the out point and a float): the
/// point times the float into the out point; answers it.
const POINT_TIMES_SCALAR: u32 = 0x0045_bb20;
/// Unnamed in the engine map (`004a3760`, cdecl: the out point, a float, a point): the point
/// times the float into the out point; answers it.
const SCALAR_TIMES_POINT: u32 = 0x004a_3760;
/// Unnamed in the engine map (`00439e90`, thiscall on a point: the out point and another
/// point): the sum into the out point (each component `FLD`, `FADD`, stored); answers it.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
/// The double at `01017cf8` (about `1e-6`) the capsule find compares its two end distances with.
const END_MARGIN_01017CF8: u32 = 0x0101_7cf8;
/// The double `0.5` at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;

/// A `double` loaded by `FLD qword` from `address`.
fn load_double(e: &Engine, address: u32) -> f64 {
    x87_load64(e.mem.u64(address))
}

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00e9da30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (find table entry half space x capsule, `cdecl`, ten words). With
/// `d0` and `d1` the signed distances of the capsule segment's origin (+0x20) and end (origin
/// plus direction, +0x2c; each sum stored) and `-r` the negated radius (+0x38):
/// - the segment crosses the plane (`d0 < 0 < d1` or `d1 < 0 < d0`): time 0, the point is the
///   origin plus `d0 / (d0 - d1)` times the direction; normals as below. True.
/// - `-r <= d0`: time 0, then a sphere volume of the capsule's radius on the stack (`00eab8a0`
///   with radius 1.0 at `011f426c`, then the radius stored) centered at the capsule's center (+4)
///   when also `-r <= d1`, at the origin otherwise, handed to the sphere find (`fn_00e9d2c0`)
///   with the other nine words; its answer.
/// - `-r <= d1`: the same with the sphere centered at the end point.
/// - otherwise, with `s` the normal component of the relative velocity (stored), false unless
///   `s > 0` and `q = -(s * time + r)` (stored) is at most `d0` or `d1`. Then the end nearer the
///   plane gives the time `-((d + r) / s)` (the origin when `d1 + 1e-6 < d0`, the end when
///   `d0 + 1e-6 < d1`, else the segment's middle with `d1`) and the point that end (origin, origin
///   plus direction, or origin plus half the direction) plus the time times `capsule_velocity`
///   plus `r` times the normal (`0045bb20`, `004a3760`, `00439e90`).
///
/// With `calculate_normals` (not in the sphere cases) `second_normal` gets the plane's normal and
/// `first_normal` its negation. True.
pub fn fn_00e9da30(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    capsule: Ptr,
    capsule_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: bool,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let n = half_space.addr();
    let normal = n.wrapping_add(4);
    let c = capsule.addr();
    let origin = c.wrapping_add(0x20);
    let direction = c.wrapping_add(0x2c);
    let radius = c.wrapping_add(0x38);
    let dot = sum3([
        product(e, origin.wrapping_add(4), n.wrapping_add(8)),
        product(e, origin, normal),
        product(e, origin.wrapping_add(8), n.wrapping_add(0xc)),
    ]);
    let start = x87_load32(plane_distance(e, n, dot));
    let end = [
        x87_store32(x87_add(load_float(e, origin), float_operand(e, direction))),
        x87_store32(x87_add(
            load_float(e, direction.wrapping_add(4)),
            float_operand(e, origin.wrapping_add(4)),
        )),
        x87_store32(x87_add(
            load_float(e, direction.wrapping_add(8)),
            float_operand(e, origin.wrapping_add(8)),
        )),
    ];
    let dot = sum3([
        x87_mul(load_float(e, normal), x87_m32(end[0])),
        x87_mul(load_float(e, normal.wrapping_add(4)), x87_m32(end[1])),
        x87_mul(load_float(e, normal.wrapping_add(8)), x87_m32(end[2])),
    ]);
    let finish = x87_load32(plane_distance(e, n, dot));
    let time_bits = x87_quiet_bits(time.to_bits());
    if (start < 0.0 && finish > 0.0) || (start > 0.0 && finish < 0.0) {
        e.mem.set_u32(out_time.addr(), 0);
        let along = x87_load32(x87_store32(x87_div(start, x87_sub(start, finish))));
        let offset = [
            x87_store32(x87_mul(load_float(e, direction), along)),
            x87_store32(x87_mul(load_float(e, direction.wrapping_add(4)), along)),
            x87_store32(x87_mul(along, float_operand(e, direction.wrapping_add(8)))),
        ];
        let mut point = [0u32; 3];
        for (axis, slot) in point.iter_mut().enumerate() {
            *slot = x87_store32(x87_add(
                load_float(e, origin.wrapping_add(4 * axis as u32)),
                x87_m32(offset[axis]),
            ));
        }
        store_point(e, out_point.addr(), point);
        if calculate_normals {
            plane_normals(e, n, first_normal.addr(), second_normal.addr());
        }
        return true;
    }
    let reach = x87_load32(negated_bits(e.mem.u32(radius)));
    let touching_start = reach <= start;
    if touching_start || reach <= finish {
        e.mem.set_u32(out_time.addr(), 0);
        return e.with_stack(0x14, |e, sphere| {
            e.call(
                SPHERE_BV_CONSTRUCTOR,
                &args![sphere, 1.0f32, ZERO_POINT_011F426C],
            );
            let s = sphere.addr();
            e.mem.set_u32(s + 0x10, x87_quiet_bits(e.mem.u32(radius)));
            if touching_start {
                let end_reaches = -load_float(e, radius) <= finish;
                let center = if end_reaches {
                    c.wrapping_add(4)
                } else {
                    origin
                };
                copy_point(e, s + 4, center);
            } else {
                let center = [0u32, 4, 8].map(|offset| {
                    x87_store32(x87_add(
                        load_float(e, origin.wrapping_add(offset)),
                        float_operand(e, direction.wrapping_add(offset)),
                    ))
                });
                store_point(e, s + 4, center);
            }
            fn_00e9d2c0(
                e,
                f32::from_bits(time_bits),
                half_space,
                velocity,
                sphere,
                capsule_velocity,
                out_time,
                out_point,
                calculate_normals,
                first_normal,
                second_normal,
            )
        });
    }
    let relative = difference(e, capsule_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, normal.wrapping_add(4)), x87_m32(relative[1])),
        x87_mul(load_float(e, normal), x87_m32(relative[0])),
        x87_mul(load_float(e, normal.wrapping_add(8)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let limit = x87_add(x87_mul(speed, x87_m32(time_bits)), float_operand(e, radius));
    let limit = x87_load32(x87_store32(-limit));
    if !(limit <= start) && !(limit <= finish) {
        return false;
    }
    let margin = load_double(e, END_MARGIN_01017CF8);
    let v = capsule_velocity.addr();
    let radius_bits = x87_quiet_bits(e.mem.u32(radius));
    e.with_stack(0x40, |e, scratch| {
        let scratch = scratch.addr();
        let (scaled, motion, sum, result) =
            (scratch, scratch + 0xc, scratch + 0x18, scratch + 0x24);
        let first_end = scratch + 0x30;
        if x87_add(finish, margin) < start {
            let contact = x87_store32(-x87_div(x87_add(start, float_operand(e, radius)), speed));
            e.mem.set_u32(out_time.addr(), x87_quiet_bits(contact));
            e.call(POINT_TIMES_SCALAR, &args![normal, scaled, radius_bits]);
            e.call(SCALAR_TIMES_POINT, &args![motion, contact, v]);
            e.call(POINT_PLUS_POINT, &args![origin, sum, motion]);
            e.call(POINT_PLUS_POINT, &args![sum, result, scaled]);
            copy_point(e, out_point.addr(), result);
        } else {
            let start_margin = x87_add(start, margin);
            let contact = x87_store32(-x87_div(x87_add(finish, float_operand(e, radius)), speed));
            if start_margin < finish {
                e.mem.set_u32(out_time.addr(), x87_quiet_bits(contact));
                e.call(POINT_TIMES_SCALAR, &args![normal, scaled, radius_bits]);
                e.call(SCALAR_TIMES_POINT, &args![motion, contact, v]);
                e.call(POINT_PLUS_POINT, &args![origin, first_end, direction]);
                e.call(POINT_PLUS_POINT, &args![first_end, sum, motion]);
                e.call(POINT_PLUS_POINT, &args![sum, result, scaled]);
                copy_point(e, out_point.addr(), result);
            } else {
                let contact = x87_quiet_bits(contact);
                e.mem.set_u32(out_time.addr(), contact);
                let half = load_double(e, HALF_01011588);
                let offset = [
                    x87_store32(x87_mul(load_float(e, direction), half)),
                    x87_store32(x87_mul(load_float(e, direction.wrapping_add(4)), half)),
                    x87_store32(x87_mul(half, float_operand(e, direction.wrapping_add(8)))),
                ];
                let middle = [0u32, 1, 2].map(|axis| {
                    x87_store32(x87_add(
                        load_float(e, origin.wrapping_add(4 * axis)),
                        x87_m32(offset[axis as usize]),
                    ))
                });
                e.call(SCALAR_TIMES_POINT, &args![motion, contact, v]);
                let moved = [0u32, 1, 2].map(|axis| {
                    x87_store32(x87_add(
                        load_float(e, motion + 4 * axis),
                        x87_m32(middle[axis as usize]),
                    ))
                });
                e.call(POINT_TIMES_SCALAR, &args![normal, scaled, radius_bits]);
                let point = [0u32, 1, 2].map(|axis| {
                    x87_store32(x87_add(
                        load_float(e, scaled + 4 * axis),
                        x87_m32(moved[axis as usize]),
                    ))
                });
                store_point(e, out_point.addr(), point);
            }
        }
    });
    if calculate_normals {
        plane_normals(e, n, first_normal.addr(), second_normal.addr());
    }
    true
}

// Translated from 00e9dfa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; find table entry half space x half space, `cdecl`, ten words).
/// With `k` the normals' dot product (stored):
/// - `k` above the double at `010cb650` (the planes are not opposite): time 0, and with
///   `w = 1 / (1 - k k)` (stored), `a = (c1 - k c2) w`, `b = (c2 - k c1) w` (stored) the point is
///   `a n1 + b n2`; the normals (when asked) are `-n1` into `first_normal` and `-n2` into
///   `second_normal`. True.
/// - `-c2 >= c1` (they overlap): time 0, the point is `(c1 - c2) n1`; the normals (when asked)
///   `n1` into `second_normal` and `n2` into `first_normal`. True.
/// - otherwise, with `s` this normal's component of the relative velocity: false unless `s > 0`
///   and `s * time - c2 >= c1`; then the time is `(c2 + c1) / s` and the point `c1 n1` plus the
///   time times `velocity` (this volume's own velocity); normals as in the overlap case. True.
pub fn fn_00e9dfa0(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    other: Ptr<NiHalfSpaceBV>,
    other_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: bool,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let a = half_space.addr();
    let b = other.addr();
    let dot = x87_load32(sum3([
        product(e, b.wrapping_add(4), a.wrapping_add(4)),
        product(e, b.wrapping_add(8), a.wrapping_add(8)),
        product(e, b.wrapping_add(0xc), a.wrapping_add(0xc)),
    ]));
    if dot > e.mem.f64(OPPOSITE_NORMALS_010CB650) {
        let squared = x87_mul(dot, dot);
        let inverse = x87_load32(x87_store32(x87_div(1.0, x87_sub(1.0, squared))));
        let own = load_float(e, a.wrapping_add(0x10));
        let towards = x87_sub(own, x87_mul(load_float(e, b.wrapping_add(0x10)), dot));
        let own_scale = x87_load32(x87_store32(x87_mul(towards, inverse)));
        let other_constant = load_float(e, b.wrapping_add(0x10));
        let product_own = x87_mul(dot, load_float(e, a.wrapping_add(0x10)));
        let other_scale = x87_load32(x87_store32(x87_mul(
            x87_sub(other_constant, product_own),
            inverse,
        )));
        e.mem.set_u32(out_time.addr(), 0);
        let along_other = [
            x87_store32(x87_mul(load_float(e, b.wrapping_add(4)), other_scale)),
            x87_store32(x87_mul(load_float(e, b.wrapping_add(8)), other_scale)),
            x87_store32(x87_mul(other_scale, float_operand(e, b.wrapping_add(0xc)))),
        ];
        let along_own = [
            x87_store32(x87_mul(load_float(e, a.wrapping_add(4)), own_scale)),
            x87_store32(x87_mul(load_float(e, a.wrapping_add(8)), own_scale)),
            x87_store32(x87_mul(own_scale, float_operand(e, a.wrapping_add(0xc)))),
        ];
        let point = [
            x87_store32(x87_add(x87_load32(along_own[0]), x87_m32(along_other[0]))),
            x87_store32(x87_add(x87_load32(along_other[1]), x87_m32(along_own[1]))),
            x87_store32(x87_add(x87_load32(along_own[2]), x87_m32(along_other[2]))),
        ];
        store_point(e, out_point.addr(), point);
        if calculate_normals {
            let negated =
                [4u32, 8, 0xc].map(|offset| negated_bits(e.mem.u32(a.wrapping_add(offset))));
            store_point(e, first_normal.addr(), negated);
            let negated =
                [4u32, 8, 0xc].map(|offset| negated_bits(e.mem.u32(b.wrapping_add(offset))));
            store_point(e, second_normal.addr(), negated);
        }
        return true;
    }
    let own_constant = load_float(e, a.wrapping_add(0x10));
    if -load_float(e, b.wrapping_add(0x10)) >= own_constant {
        e.mem.set_u32(out_time.addr(), 0);
        let gap = x87_load32(x87_store32(x87_sub(
            load_float(e, a.wrapping_add(0x10)),
            float_operand(e, b.wrapping_add(0x10)),
        )));
        let point = [
            x87_store32(x87_mul(load_float(e, a.wrapping_add(4)), gap)),
            x87_store32(x87_mul(load_float(e, a.wrapping_add(8)), gap)),
            x87_store32(x87_mul(gap, float_operand(e, a.wrapping_add(0xc)))),
        ];
        store_point(e, out_point.addr(), point);
        if calculate_normals {
            copy_point(e, second_normal.addr(), a.wrapping_add(4));
            copy_point(e, first_normal.addr(), b.wrapping_add(4));
        }
        return true;
    }
    let relative = difference(e, other_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, a.wrapping_add(8)), x87_m32(relative[1])),
        x87_mul(load_float(e, a.wrapping_add(4)), x87_m32(relative[0])),
        x87_mul(load_float(e, a.wrapping_add(0xc)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let own_constant = load_float(e, a.wrapping_add(0x10));
    let moved = x87_sub(
        x87_mul(speed, x87_m32(time.to_bits())),
        float_operand(e, b.wrapping_add(0x10)),
    );
    if !(moved >= own_constant) {
        return false;
    }
    let total = x87_add(
        load_float(e, b.wrapping_add(0x10)),
        float_operand(e, a.wrapping_add(0x10)),
    );
    let contact = x87_store32(x87_div(total, speed));
    e.mem.set_u32(out_time.addr(), contact);
    let contact = x87_load32(contact);
    let v = velocity.addr();
    let motion = [
        x87_store32(x87_mul(contact, float_operand(e, v))),
        x87_store32(x87_mul(load_float(e, v.wrapping_add(4)), contact)),
        x87_store32(x87_mul(contact, float_operand(e, v.wrapping_add(8)))),
    ];
    let constant = x87_load32(x87_quiet_bits(e.mem.u32(a.wrapping_add(0x10))));
    let on_plane = [
        x87_store32(x87_mul(load_float(e, a.wrapping_add(4)), constant)),
        x87_store32(x87_mul(load_float(e, a.wrapping_add(8)), constant)),
        x87_store32(x87_mul(constant, float_operand(e, a.wrapping_add(0xc)))),
    ];
    let point = [0usize, 1, 2]
        .map(|axis| x87_store32(x87_add(x87_load32(on_plane[axis]), x87_m32(motion[axis]))));
    store_point(e, out_point.addr(), point);
    if calculate_normals {
        copy_point(e, second_normal.addr(), a.wrapping_add(4));
        copy_point(e, first_normal.addr(), b.wrapping_add(4));
    }
    true
}

// Translated from 00e9e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (triangle find table entry of the half space, `cdecl`, twelve words).
/// The first vertex whose signed distance (stored) is at least 0 is the contact at time 0: the
/// point is that vertex, and with `calculate_normals` `second_normal` gets the plane's normal and
/// `first_normal` its negation. True. Otherwise, with `s` the normal component of
/// `triangle_velocity - velocity` (stored): false unless `s > 0` and `m = -time * s` (stored) is at
/// most one of the distances. Then the vertex of the largest distance `d` (the first of equal ones:
/// `v0` when `d0 >= d1` and `d2 <= d0`, `v1` when `d0 < d1` and `d2 <= d1`, else `v2`; unordered
/// falls to the later ones) gives the time `-d / s`, and the point is that vertex plus `time` (the
/// whole time, as the exe passes it) times `triangle_velocity` (`004a3760`, `00439e90`); normals as
/// above. True.
pub fn fn_00e9e2c0(
    e: &mut Engine,
    time: f32,
    half_space: Ptr<NiHalfSpaceBV>,
    velocity: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    triangle_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: bool,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let n = half_space.addr();
    let normal = n.wrapping_add(4);
    let vertices = [vertex0.addr(), vertex1.addr(), vertex2.addr()];
    let mut distances = [0f64; 3];
    for (index, vertex) in vertices.iter().enumerate() {
        let p = *vertex;
        let middle = if index == 2 {
            product(e, p, normal)
        } else {
            product(e, normal, p)
        };
        let dot = sum3([
            product(e, p.wrapping_add(4), n.wrapping_add(8)),
            middle,
            product(e, p.wrapping_add(8), n.wrapping_add(0xc)),
        ]);
        distances[index] = x87_load32(plane_distance(e, n, dot));
        if distances[index] >= 0.0 {
            e.mem.set_u32(out_time.addr(), 0);
            copy_point(e, out_point.addr(), p);
            if calculate_normals {
                plane_normals(e, n, first_normal.addr(), second_normal.addr());
            }
            return true;
        }
    }
    let [first, second, third] = distances;
    let relative = difference(e, triangle_velocity.addr(), velocity.addr());
    let speed = x87_load32(sum3([
        x87_mul(load_float(e, normal.wrapping_add(4)), x87_m32(relative[1])),
        x87_mul(load_float(e, normal), x87_m32(relative[0])),
        x87_mul(load_float(e, normal.wrapping_add(8)), x87_m32(relative[2])),
    ]));
    if speed <= 0.0 {
        return false;
    }
    let time = x87_load32(time.to_bits());
    let moved = x87_load32(x87_store32(x87_mul(-time, speed)));
    if !(moved <= first) && !(moved <= second) && !(moved <= third) {
        return false;
    }
    let (vertex, distance) = if first >= second {
        if third <= first {
            (vertices[0], first)
        } else {
            (vertices[2], third)
        }
    } else if third <= second {
        (vertices[1], second)
    } else {
        (vertices[2], third)
    };
    let contact = x87_store32(x87_div(-distance, speed));
    e.mem.set_u32(out_time.addr(), contact);
    e.with_stack(0x18, |e, scratch| {
        let motion = scratch.addr();
        let point = motion + 0xc;
        e.call(
            SCALAR_TIMES_POINT,
            &args![motion, x87_store32(time), triangle_velocity],
        );
        e.call(POINT_PLUS_POINT, &args![vertex, point, motion]);
        copy_point(e, out_point.addr(), point);
    });
    if calculate_normals {
        plane_normals(e, n, first_normal.addr(), second_normal.addr());
    }
    true
}

// Translated from 00e9e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::NiHalfSpaceBV` (Xbox PDB), from a point on the plane and the normal: sets the
/// vtable, constructs the plane (`NiPlane::NiPlane`, `00a69940`), copies `point` to the center,
/// then builds a local plane (constructed the same way) of `normal` and the constant
/// `point . normal` (stored as a float) and hands it to `SetPlane`. Answers `this`.
pub fn ni_half_space_bv_ni_half_space_bv(
    e: &mut Engine,
    this: Ptr<NiHalfSpaceBV>,
    point: Ptr,
    normal: Ptr,
) -> Ptr<NiHalfSpaceBV> {
    let base = this.addr();
    e.mem.set_u32(base, NI_HALF_SPACE_BV_VTABLE);
    e.call(NI_PLANE_CONSTRUCTOR, &args![base.wrapping_add(4)]);
    let p = point.addr();
    let x = e.mem.u32(p);
    e.set(this, NiHalfSpaceBV::m_kCenter_x, x);
    let y = e.mem.u32(p.wrapping_add(4));
    e.set(this, NiHalfSpaceBV::m_kCenter_y, y);
    let z = e.mem.u32(p.wrapping_add(8));
    e.set(this, NiHalfSpaceBV::m_kCenter_z, z);
    e.with_stack(0x10, |e, plane| {
        e.call(NI_PLANE_CONSTRUCTOR, &args![plane]);
        let n = normal.addr();
        let constant = sum3([
            product(e, p.wrapping_add(4), n.wrapping_add(4)),
            product(e, p, n),
            product(e, p.wrapping_add(8), n.wrapping_add(8)),
        ]);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(n.wrapping_add(offset));
            e.mem.set_u32(plane.addr() + offset, word);
        }
        e.mem.set_u32(plane.addr() + 0xc, constant);
        ni_half_space_bv_set_plane(e, this, plane);
    });
    this
}

// Translated from 00e9e700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::Create` (Xbox PDB): allocates 0x20 bytes from the memory manager and
/// constructs a half space through the point `011f426c` with the normal `011a9478`. Answers it,
/// or null.
pub fn ni_half_space_bv_create(e: &mut Engine) -> Ptr<NiHalfSpaceBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x20u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    ni_half_space_bv_ni_half_space_bv(
        e,
        Ptr::new(block),
        Ptr::new(ZERO_POINT_011F426C),
        Ptr::new(DEFAULT_NORMAL_011A9478),
    )
}

// Translated from 00e9e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::Clone` (Xbox PDB): allocates 0x20 bytes from the memory manager; a block gets
/// the vtable, a constructed plane (`00a69940`), this plane through `SetPlane` and this center's
/// three words. Answers the copy, or null.
pub fn ni_half_space_bv_clone(e: &mut Engine, this: Ptr<NiHalfSpaceBV>) -> Ptr<NiHalfSpaceBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x20u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let copy: Ptr<NiHalfSpaceBV> = Ptr::new(block);
    e.mem.set_u32(block, NI_HALF_SPACE_BV_VTABLE);
    e.call(NI_PLANE_CONSTRUCTOR, &args![block + 4]);
    ni_half_space_bv_set_plane(e, copy, Ptr::new(this.addr().wrapping_add(4)));
    let x = e.get(this, NiHalfSpaceBV::m_kCenter_x);
    e.set(copy, NiHalfSpaceBV::m_kCenter_x, x);
    let y = e.get(this, NiHalfSpaceBV::m_kCenter_y);
    e.set(copy, NiHalfSpaceBV::m_kCenter_y, y);
    let z = e.get(this, NiHalfSpaceBV::m_kCenter_z);
    e.set(copy, NiHalfSpaceBV::m_kCenter_z, z);
    copy
}

// Translated from 00e9e780 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the half space's stream loader stored at `0126fa90` by the
/// NiCollision start-up, `cdecl`): allocates 0x20 bytes from the memory manager, constructs a half
/// space through `011f426c` with the normal `011a9478` and calls its `LoadBinary` (virtual slot
/// +0) with the stream. A failed allocation calls through null (the exe faults there). Answers
/// the volume.
pub fn fn_00e9e780(e: &mut Engine, stream: Ptr) -> Ptr<NiHalfSpaceBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x20u32])
        .u32();
    let volume = if block != 0 {
        ni_half_space_bv_ni_half_space_bv(
            e,
            Ptr::new(block),
            Ptr::new(ZERO_POINT_011F426C),
            Ptr::new(DEFAULT_NORMAL_011A9478),
        )
    } else {
        Ptr::NULL
    };
    e.vcall(volume.addr(), 0, &args![stream]);
    volume
}

// Translated from 00e9e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiHalfSpaceBV::AddViewerStrings` (Xbox PDB): appends the line `"<name> = HALFSPACE_BV"` (a
/// block of `strlen(name) + 0x10` bytes from `00aa1070`, formatted by `NiSprintf` with the format
/// at `010f1758`) to the strings array. `this` is not read.
pub fn ni_half_space_bv_add_viewer_strings(
    e: &mut Engine,
    _this: Ptr<NiHalfSpaceBV>,
    name: Ptr,
    strings: Ptr,
) {
    let size = c_string_length(e, name.addr()).wrapping_add(0x10);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, HALF_SPACE_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_e670, fn_00e9e670(Ptr) -> u32),
        entry!(
            0x00e9_cb50,
            ni_half_space_bv_load_binary(Ptr<NiHalfSpaceBV>, Ptr)
        ),
        entry!(
            0x00e9_cb80,
            ni_half_space_bv_save_binary(Ptr<NiHalfSpaceBV>, Ptr)
        ),
        entry!(
            0x00e9_cbb0,
            ni_half_space_bv_set_plane(Ptr<NiHalfSpaceBV>, Ptr)
        ),
        entry!(
            0x00e9_cc10,
            ni_half_space_bv_copy(Ptr<NiHalfSpaceBV>, Ptr<NiHalfSpaceBV>)
        ),
        entry!(
            0x00e9_cc20,
            ni_half_space_bv_operator_(Ptr<NiHalfSpaceBV>, Ptr<NiHalfSpaceBV>) -> bool
        ),
        entry!(
            0x00e9_cc60,
            ni_half_space_bv_update_world_data(Ptr<NiHalfSpaceBV>, Ptr<NiHalfSpaceBV>, Ptr)
        ),
        entry!(
            0x00e9_cd10,
            fn_00e9cd10(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_cde0,
            fn_00e9cde0(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_cf50,
            fn_00e9cf50(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_d070,
            fn_00e9d070(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr<NiHalfSpaceBV>, Ptr) -> bool
        ),
        entry!(
            0x00e9_d150,
            fn_00e9d150(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_d2c0,
            fn_00e9d2c0(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_d540,
            fn_00e9d540(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_da30,
            fn_00e9da30(f32, Ptr<NiHalfSpaceBV>, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_dfa0,
            fn_00e9dfa0(
                f32,
                Ptr<NiHalfSpaceBV>,
                Ptr,
                Ptr<NiHalfSpaceBV>,
                Ptr,
                Ptr,
                Ptr,
                bool,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00e9_e2c0,
            fn_00e9e2c0(
                f32,
                Ptr<NiHalfSpaceBV>,
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
            0x00e9_e680,
            ni_half_space_bv_ni_half_space_bv(Ptr<NiHalfSpaceBV>, Ptr, Ptr) -> Ptr<NiHalfSpaceBV>
        ),
        entry!(0x00e9_e700, ni_half_space_bv_create() -> Ptr<NiHalfSpaceBV>),
        entry!(
            0x00e9_e730,
            ni_half_space_bv_clone(Ptr<NiHalfSpaceBV>) -> Ptr<NiHalfSpaceBV>
        ),
        entry!(0x00e9_e780, fn_00e9e780(Ptr) -> Ptr<NiHalfSpaceBV>),
        entry!(
            0x00e9_e7d0,
            ni_half_space_bv_add_viewer_strings(Ptr<NiHalfSpaceBV>, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_five() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_e670, &args![0x5000u32]).u32(), 5);
    }
}

#[cfg(test)]
mod n034_tests {
    use super::*;
    use crate::units::nicollision::nicollisiondata::VIEWER_STRINGS_SET_AT;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// A half space volume (vtable word 0) with that normal and constant.
    fn half_space(e: &mut Engine, normal: [f32; 3], constant: f32) -> u32 {
        floats(
            e,
            &[
                0.0, normal[0], normal[1], normal[2], constant, 0.0, 0.0, 0.0,
            ],
        )
    }

    /// A box volume: center, the identity axes, extents.
    fn box_volume(e: &mut Engine, center: [f32; 3], extents: [f32; 3]) -> u32 {
        let mut words = vec![0.0, center[0], center[1], center[2]];
        words.extend([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        words.extend(extents);
        floats(e, &words)
    }

    #[test]
    fn load_and_save_go_through_the_parts() {
        let mut e = Engine::new();
        for f in [
            NI_BOUNDING_VOLUME_LOAD_BINARY,
            LOAD_FOUR_FLOATS,
            LOAD_THREE_FLOATS,
            NI_BOUNDING_VOLUME_SAVE_BINARY,
            SAVE_FOUR_FLOATS,
            SAVE_THREE_FLOATS,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00e9_cb50, &args![0x5000u32, 0x77u32]);
        e.call(0x00e9_cb80, &args![0x5000u32, 0x77u32]);
        assert_eq!(
            e.call_log.clone().unwrap(),
            vec![
                (0x00e9_cb50, vec![0x5000, 0x77]),
                (NI_BOUNDING_VOLUME_LOAD_BINARY, vec![0x5000, 0x77]),
                (LOAD_FOUR_FLOATS, vec![0x5004, 0x77]),
                (LOAD_THREE_FLOATS, vec![0x5014, 0x77]),
                (0x00e9_cb80, vec![0x5000, 0x77]),
                (NI_BOUNDING_VOLUME_SAVE_BINARY, vec![0x5000, 0x77]),
                (SAVE_FOUR_FLOATS, vec![0x5004, 0x77]),
                (SAVE_THREE_FLOATS, vec![0x5014, 0x77]),
            ]
        );
    }

    /// `UnitizeGetLength` for the tests: divides by the length (exact for axis vectors).
    fn unitize(e: &mut Engine, a: &[u32]) -> Ret {
        let p = a[0];
        let v = read(e, p, 3);
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, c / length);
        }
        Ret {
            st0: f64::from(length),
            ..Ret::default()
        }
    }

    #[test]
    fn set_plane_normalizes_and_scales_the_constant() {
        let mut e = Engine::new();
        e.register(UNITIZE_GET_LENGTH, unitize);
        let this = half_space(&mut e, [9.0, 9.0, 9.0], 9.0);
        let plane = floats(&mut e, &[0.0, 0.0, 2.0, 6.0]);
        e.call(0x00e9_cbb0, &args![this, plane]);
        assert_eq!(read(&e, this + 4, 4), vec![0.0, 0.0, 1.0, 3.0]);
        // The caller's plane is not changed.
        assert_eq!(read(&e, plane, 4), vec![0.0, 0.0, 2.0, 6.0]);
        // Copy takes the other volume's plane.
        let other = half_space(&mut e, [4.0, 0.0, 0.0], -8.0);
        e.call(0x00e9_cc10, &args![this, other]);
        assert_eq!(read(&e, this + 4, 4), vec![1.0, 0.0, 0.0, -2.0]);
    }

    #[test]
    fn equality_needs_type_five_and_equal_planes() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0xc, 0x00ab_0005);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, vtable);
        e.register(FOUR_FLOATS_EQUAL, |_, _| 1u32.into_ret());
        e.register(0x00ab_0005, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_cc20, &args![0x6000u32, other]).bool());
        assert_eq!(
            calls_to(&e, FOUR_FLOATS_EQUAL),
            vec![vec![other + 4, 0x6004]]
        );
        e.register(0x00ab_0005, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_cc20, &args![0x6000u32, other]).bool());
        assert!(calls_to(&e, FOUR_FLOATS_EQUAL).is_empty());
    }

    #[test]
    fn update_world_data_rotates_and_moves_the_plane() {
        let mut e = Engine::new();
        let model = half_space(&mut e, [0.0, 1.0, 0.0], 2.0);
        let this = half_space(&mut e, [5.0, 5.0, 5.0], 5.0);
        e.mem.set_f32(this + 0x14, 7.5);
        let transform = floats(
            &mut e,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 2.0,
            ],
        );
        e.call(0x00e9_cc60, &args![this, model, transform]);
        assert_eq!(read(&e, this + 4, 4), vec![0.0, 0.0, 1.0, 7.0]);
        assert_eq!(e.mem.f32(this + 0x14), 7.5);
    }

    #[test]
    fn sphere_test_touching_or_reaching_in_time() {
        let mut e = Engine::new();
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 2.0]);
        let down = floats(&mut e, &[0.0, 0.0, -2.0]);
        let touching = floats(&mut e, &[0.0, 0.0, 0.0, -0.5, 1.0]);
        let below = floats(&mut e, &[0.0, 0.0, 0.0, -5.0, 1.0]);
        let mut test =
            |t: f32, hv: u32, s: u32, v: u32| e.call(0x00e9_cd10, &args![t, hs, hv, s, v]).bool();
        assert!(test(0.0, still, touching, still));
        assert!(!test(10.0, still, below, still));
        // d + s t = -5 + 2 t reaches -1 at t = 2.
        assert!(test(2.0, still, below, up));
        assert!(!test(1.5, still, below, up));
        // Moving away.
        assert!(!test(100.0, still, below, down));
        // The half space moving toward the sphere counts the same.
        assert!(test(2.0, down, below, still));
    }

    #[test]
    fn box_test_uses_the_projected_extent() {
        let mut e = Engine::new();
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let below = box_volume(&mut e, [0.0, 0.0, -5.0], [1.0, 2.0, 3.0]);
        let near = box_volume(&mut e, [0.0, 0.0, -3.0], [1.0, 2.0, 3.0]);
        let mut test =
            |t: f32, b: u32, v: u32| e.call(0x00e9_cde0, &args![t, hs, still, b, v]).bool();
        assert!(test(0.0, near, still));
        assert!(!test(1.0, below, up));
        assert!(test(2.0, below, up));
        assert!(!test(2.0, below, still));
    }

    /// A capsule volume whose segment runs along z from `origin` by `direction`, radius 1.
    fn capsule(e: &mut Engine, origin: f32, direction: f32) -> u32 {
        let mut words = vec![0.0f32; 15];
        words[10] = origin;
        words[13] = direction;
        words[14] = 1.0;
        floats(e, &words)
    }

    #[test]
    fn capsule_test_checks_both_ends() {
        let mut e = Engine::new();
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let long = capsule(&mut e, -5.0, 5.0);
        let short = capsule(&mut e, -5.0, 2.0);
        let touching = capsule(&mut e, -0.5, 0.0);
        let mut test =
            |t: f32, c: u32, v: u32| e.call(0x00e9_cf50, &args![t, hs, still, c, v]).bool();
        // The far end reaches past -r.
        assert!(test(0.0, long, still));
        // d0 = -5, d1 = -3: q = -(t + 1) reaches d1 at t = 2.
        assert!(!test(1.0, short, up));
        assert!(test(2.0, short, up));
        assert!(!test(9.0, short, still));
        assert!(test(0.0, touching, still));
    }

    #[test]
    fn half_space_pair_test() {
        let mut e = Engine::new();
        e.map(0x010c_b000, 0x1000);
        e.mem.set_f64(OPPOSITE_NORMALS_010CB650, -0.999_999);
        let a = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let b = half_space(&mut e, [0.0, 0.0, -1.0], 5.0);
        let tilted = half_space(&mut e, [0.0, 1.0, 0.0], 5.0);
        let overlapping = half_space(&mut e, [0.0, 0.0, -1.0], -1.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let mut test =
            |t: f32, o: u32, v: u32| e.call(0x00e9_d070, &args![t, a, still, o, v]).bool();
        assert!(test(0.0, tilted, still));
        assert!(!test(9.0, b, still));
        assert!(!test(4.0, b, up));
        assert!(test(5.0, b, up));
        assert!(test(0.0, overlapping, still));
    }

    #[test]
    fn triangle_test_checks_each_vertex_then_the_motion() {
        let mut e = Engine::new();
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let low = floats(&mut e, &[0.0, 0.0, -3.0]);
        let lower = floats(&mut e, &[1.0, 0.0, -4.0]);
        let touching = floats(&mut e, &[0.0, 1.0, 0.0]);
        let mut test = |t: f32, third: u32, v: u32| {
            e.call(0x00e9_d150, &args![t, hs, still, low, lower, third, v])
                .bool()
        };
        assert!(test(0.0, touching, still));
        // m = -t reaches d0 = -3 at t = 3.
        assert!(!test(2.0, lower, up));
        assert!(test(3.0, lower, up));
        assert!(!test(9.0, lower, still));
    }

    #[test]
    fn sphere_find_touching_and_moving() {
        let mut e = Engine::new();
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 2.0]);
        let outs = floats(&mut e, &[9.0; 10]);
        let (time, point, first, second) = (outs, outs + 4, outs + 0x10, outs + 0x1c);
        let near = floats(&mut e, &[0.0, 1.0, 2.0, 0.5, 1.0]);
        let below = floats(&mut e, &[0.0, 0.0, 0.0, -5.0, 1.0]);
        let mut find = |t: f32, hv: u32, s: u32, v: u32, normals: bool| {
            e.call(
                0x00e9_d2c0,
                &args![t, hs, hv, s, v, time, point, normals, first, second],
            )
            .bool()
        };
        // Touching: center at z = 0.5, the point projected on the plane.
        assert!(find(0.0, still, near, still, true));
        // Moving: center z = -5, r = 1, speed 2: contact at t = 2, point on the plane.
        assert!(find(3.0, still, below, up, true));
        // Too short a time, or moving away: no contact.
        assert!(!find(1.5, still, below, up, true));
        assert!(!find(9.0, up, below, still, true));
        assert!(e
            .call(
                0x00e9_d2c0,
                &args![3.0f32, hs, still, below, up, time, point, true, first, second]
            )
            .bool());
        assert_eq!(read(&e, time, 4), vec![2.0, 0.0, 0.0, 0.0]);
        assert_eq!(read(&e, second, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(e.mem.u32(first), 0x8000_0000);
        assert_eq!(e.mem.f32(first + 8), -1.0);
        assert!(e
            .call(
                0x00e9_d2c0,
                &args![0.0f32, hs, still, near, still, time, point, true, first, second]
            )
            .bool());
        assert_eq!(read(&e, time, 4), vec![0.0, 1.0, 2.0, 0.0]);
        // Without normals the point still uses the words in the second normal.
        e.mem.set_f32(second + 8, 3.0);
        assert!(e
            .call(
                0x00e9_d2c0,
                &args![3.0f32, hs, still, below, up, time, point, false, first, second]
            )
            .bool());
        assert_eq!(e.mem.f32(point + 8), 2.0);
    }

    #[test]
    fn box_find_moves_to_the_contact_and_pushes_along_the_axes() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.mem.set_f32(AXIS_THRESHOLD_01017D00, 0.001);
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let tilted = half_space(&mut e, [0.0, -0.6, 0.8], 10.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let outs = floats(&mut e, &[9.0; 10]);
        let (time, point, first, second) = (outs, outs + 4, outs + 0x10, outs + 0x1c);
        let below = box_volume(&mut e, [0.0, 0.0, -5.0], [1.0, 2.0, 3.0]);
        let near = box_volume(&mut e, [1.0, 2.0, -2.0], [1.0, 2.0, 3.0]);
        let r = e.call(
            0x00e9_d540,
            &args![5.0f32, hs, still, below, up, time, point, true, first, second],
        );
        assert!(r.bool());
        assert_eq!(read(&e, time, 4), vec![2.0, 0.0, 0.0, 0.0]);
        assert_eq!(read(&e, first, 3), vec![-0.0, -0.0, -1.0]);
        assert_eq!(read(&e, second, 3), vec![0.0, 0.0, 5.0]);
        // A tilted plane uses two axes: y pushed back (p1 < 0), z forward (p2 > 0).
        let r = e.call(
            0x00e9_d540,
            &args![100.0f32, tilted, still, below, up, time, point, false, first, second],
        );
        assert!(r.bool());
        let p = read(&e, point, 3);
        assert_eq!(p[0], 0.0);
        assert!(p[1] < 0.0 && p[2] > -5.0, "{p:?}");
        // Touching: the center projected on the plane.
        let r = e.call(
            0x00e9_d540,
            &args![0.0f32, hs, still, near, still, time, point, false, first, second],
        );
        assert!(r.bool());
        assert_eq!(read(&e, time, 4), vec![0.0, 1.0, 2.0, 0.0]);
        let r = e.call(
            0x00e9_d540,
            &args![1.0f32, hs, still, below, up, time, point, false, first, second],
        );
        assert!(!r.bool());
    }

    #[test]
    fn constructor_create_clone_and_loader() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(DEFAULT_NORMAL_011A9478 + 4, 1.0);
        e.register(UNITIZE_GET_LENGTH, unitize);
        e.register(NI_PLANE_CONSTRUCTOR, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            a[0].into_ret()
        });
        let point = floats(&mut e, &[1.0, 2.0, 3.0]);
        let normal = floats(&mut e, &[0.0, 0.0, 2.0]);
        let this = e.mem.alloc(0x20);
        let r = e.call(0x00e9_e680, &args![this, point, normal]).u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), NI_HALF_SPACE_BV_VTABLE);
        // Constant 6 over the length 2.
        assert_eq!(
            read(&e, this + 4, 7),
            vec![0.0, 0.0, 1.0, 3.0, 1.0, 2.0, 3.0]
        );

        let block = e.mem.alloc(0x20);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00e9_e700, &args![]).u32(), block);
        assert_eq!(
            read(&e, block + 4, 7),
            vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );

        assert_eq!(e.call(0x00e9_e730, &args![this]).u32(), block);
        assert_eq!(
            read(&e, block + 4, 7),
            vec![0.0, 0.0, 1.0, 3.0, 1.0, 2.0, 3.0]
        );

        e.put_vtable(NI_HALF_SPACE_BV_VTABLE, &[0x00ab_0040]);
        e.register(0x00ab_0040, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_e780, &args![0x9999u32]).u32(), block);
        assert_eq!(calls_to(&e, 0x00ab_0040), vec![vec![block, 0x9999]]);

        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_e700, &args![]).u32(), 0);
        assert_eq!(e.call(0x00e9_e730, &args![this]).u32(), 0);
    }

    #[test]
    fn viewer_strings_add_one_line() {
        let mut e = Engine::new();
        let name = e.mem.alloc(8);
        for (i, b) in "plane".bytes().enumerate() {
            e.mem.set_u8(name + i as u32, b);
        }
        e.register(ALLOCATE, |_, a| (0x7000 + a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(VIEWER_STRINGS_SET_AT, |_, _| Ret::default());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00e9_e7d0, &args![0u32, name, strings]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![5 + 0x10]]);
        assert_eq!(
            calls_to(&e, NI_SPRINTF),
            vec![vec![0x7015, 0x15, HALF_SPACE_BV_FORMAT, name]]
        );
        let set = calls_to(&e, VIEWER_STRINGS_SET_AT);
        assert_eq!(&set[0][..2], &[strings, 0]);
    }
}

#[cfg(test)]
mod n034_find_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn half_space(e: &mut Engine, normal: [f32; 3], constant: f32) -> u32 {
        floats(
            e,
            &[
                0.0, normal[0], normal[1], normal[2], constant, 0.0, 0.0, 0.0,
            ],
        )
    }

    /// The point helpers the finds call, as plain float arithmetic (exact in these tests).
    fn point_doubles(e: &mut Engine) {
        e.register(POINT_TIMES_SCALAR, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * s;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(SCALAR_TIMES_POINT, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = s * e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_PLUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(SPHERE_BV_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            a[0].into_ret()
        });
    }

    /// A capsule volume (center +4) whose segment starts at `origin` and runs by `direction`,
    /// radius 1.
    fn capsule(e: &mut Engine, center: [f32; 3], origin: [f32; 3], direction: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32; 15];
        words[1..4].copy_from_slice(&center);
        words[8..11].copy_from_slice(&origin);
        words[11..14].copy_from_slice(&direction);
        words[14] = 1.0;
        floats(e, &words)
    }

    struct Outs {
        time: u32,
        point: u32,
        first: u32,
        second: u32,
    }

    fn outs(e: &mut Engine) -> Outs {
        let block = floats(e, &[9.0; 10]);
        Outs {
            time: block,
            point: block + 4,
            first: block + 0x10,
            second: block + 0x1c,
        }
    }

    fn capsule_find(e: &mut Engine, t: f32, hs: u32, capsule: u32, v: u32, o: &Outs) -> bool {
        let still = floats(e, &[0.0, 0.0, 0.0]);
        e.call(
            0x00e9_da30,
            &args![t, hs, still, capsule, v, o.time, o.point, true, o.first, o.second],
        )
        .bool()
    }

    #[test]
    fn capsule_find_crossing_and_touching() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        point_doubles(&mut e);
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let o = outs(&mut e);
        // Crossing: a quarter along the segment.
        let crossing = capsule(&mut e, [7.0; 3], [0.0, 0.0, -1.0], [0.0, 0.0, 4.0]);
        e.call_log = Some(vec![]);
        assert!(capsule_find(&mut e, 0.0, hs, crossing, still, &o));
        assert_eq!(read(&e, o.time, 4), vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(read(&e, o.second, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(e.mem.f32(o.first + 8), -1.0);
        assert!(calls_to(&e, SPHERE_BV_CONSTRUCTOR).is_empty());
        // The origin touches, the end does not: a sphere at the origin.
        let origin_side = capsule(&mut e, [7.0; 3], [0.0, 2.0, -0.5], [0.0, 0.0, -4.0]);
        e.call_log = Some(vec![]);
        assert!(capsule_find(&mut e, 0.0, hs, origin_side, still, &o));
        assert_eq!(read(&e, o.time, 4), vec![0.0, 0.0, 2.0, 0.0]);
        let built = calls_to(&e, SPHERE_BV_CONSTRUCTOR);
        assert_eq!(built.len(), 1);
        assert_eq!(&built[0][1..], &[1.0f32.to_bits(), ZERO_POINT_011F426C]);
        // Both ends touch: the sphere at the capsule's center.
        let inside = capsule(&mut e, [3.0, 4.0, 0.25], [0.0, 0.0, -0.5], [0.0, 0.0, 0.25]);
        assert!(capsule_find(&mut e, 0.0, hs, inside, still, &o));
        assert_eq!(read(&e, o.point, 3), vec![3.0, 4.0, 0.0]);
        // Only the end touches: the sphere at the end point.
        let end_side = capsule(&mut e, [7.0; 3], [1.0, 0.0, -5.0], [0.0, 0.0, 4.5]);
        assert!(capsule_find(&mut e, 0.0, hs, end_side, still, &o));
        assert_eq!(read(&e, o.point, 3), vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn capsule_find_moving_picks_the_nearer_end() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x7000);
        e.mem.set_f64(END_MARGIN_01017CF8, 1e-6);
        e.mem.set_f64(HALF_01011588, 0.5);
        point_doubles(&mut e);
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let o = outs(&mut e);
        // The end is nearer: contact at t = 2 at the end point plus the radius.
        let end_first = capsule(&mut e, [7.0; 3], [0.0, 0.0, -5.0], [0.0, 0.0, 2.0]);
        assert!(capsule_find(&mut e, 10.0, hs, end_first, up, &o));
        assert_eq!(read(&e, o.time, 4), vec![2.0, 0.0, 0.0, 0.0]);
        // The origin is nearer.
        let origin_first = capsule(&mut e, [7.0; 3], [0.0, 1.0, -3.0], [0.0, 0.0, -2.0]);
        assert!(capsule_find(&mut e, 10.0, hs, origin_first, up, &o));
        assert_eq!(read(&e, o.time, 4), vec![2.0, 0.0, 1.0, 0.0]);
        // Level: the middle of the segment.
        let level = capsule(&mut e, [7.0; 3], [0.0, 0.0, -5.0], [2.0, 0.0, 0.0]);
        assert!(capsule_find(&mut e, 10.0, hs, level, up, &o));
        assert_eq!(read(&e, o.time, 4), vec![4.0, 1.0, 0.0, 0.0]);
        // Too short a time, or moving away.
        assert!(!capsule_find(&mut e, 1.0, hs, level, up, &o));
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        assert!(!capsule_find(&mut e, 10.0, hs, level, down, &o));
    }

    fn pair_find(e: &mut Engine, t: f32, a: u32, va: u32, b: u32, vb: u32, o: &Outs) -> bool {
        e.call(
            0x00e9_dfa0,
            &args![t, a, va, b, vb, o.time, o.point, true, o.first, o.second],
        )
        .bool()
    }

    #[test]
    fn half_space_pair_find() {
        let mut e = Engine::new();
        e.map(0x010c_b000, 0x1000);
        e.mem.set_f64(OPPOSITE_NORMALS_010CB650, -0.999_999);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let o = outs(&mut e);
        // Crossing planes: the point on both.
        let a = half_space(&mut e, [0.0, 0.0, 1.0], 1.0);
        let b = half_space(&mut e, [0.0, 1.0, 0.0], 2.0);
        assert!(pair_find(&mut e, 0.0, a, still, b, still, &o));
        assert_eq!(read(&e, o.time, 4), vec![0.0, 0.0, 2.0, 1.0]);
        assert_eq!(read(&e, o.first, 3), vec![-0.0, -0.0, -1.0]);
        assert_eq!(read(&e, o.second, 3), vec![-0.0, -1.0, -0.0]);
        // Opposite and overlapping.
        let a = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let b = half_space(&mut e, [0.0, 0.0, -1.0], -1.0);
        assert!(pair_find(&mut e, 0.0, a, still, b, still, &o));
        assert_eq!(read(&e, o.time, 4), vec![0.0, 0.0, 0.0, 1.0]);
        assert_eq!(read(&e, o.second, 3), vec![0.0, 0.0, 1.0]);
        assert_eq!(read(&e, o.first, 3), vec![0.0, 0.0, -1.0]);
        // Opposite and apart: they meet at t = 5; the point moves with this volume.
        let b = half_space(&mut e, [0.0, 0.0, -1.0], 5.0);
        let sideways = floats(&mut e, &[1.0, 0.0, 0.0]);
        let closing = floats(&mut e, &[1.0, 0.0, 1.0]);
        assert!(pair_find(&mut e, 6.0, a, sideways, b, closing, &o));
        assert_eq!(read(&e, o.time, 4), vec![5.0, 5.0, 0.0, 0.0]);
        assert!(!pair_find(&mut e, 4.0, a, sideways, b, closing, &o));
        assert!(!pair_find(&mut e, 9.0, a, still, b, still, &o));
    }

    fn triangle_find(e: &mut Engine, t: f32, hs: u32, v: [u32; 3], vt: u32, o: &Outs) -> bool {
        let still = floats(e, &[0.0, 0.0, 0.0]);
        e.call(
            0x00e9_e2c0,
            &args![t, hs, still, v[0], v[1], v[2], vt, o.time, o.point, true, o.first, o.second],
        )
        .bool()
    }

    #[test]
    fn triangle_find_touching_vertex_or_the_nearest_one() {
        let mut e = Engine::new();
        point_doubles(&mut e);
        let hs = half_space(&mut e, [0.0, 0.0, 1.0], 0.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let o = outs(&mut e);
        let low = floats(&mut e, &[0.0, 0.0, -3.0]);
        let high = floats(&mut e, &[1.0, 0.0, -1.0]);
        let middle = floats(&mut e, &[2.0, 0.0, -2.0]);
        let touching = floats(&mut e, &[5.0, 6.0, 0.0]);
        assert!(triangle_find(
            &mut e,
            0.0,
            hs,
            [low, touching, high],
            still,
            &o
        ));
        assert_eq!(read(&e, o.time, 4), vec![0.0, 5.0, 6.0, 0.0]);
        assert_eq!(read(&e, o.second, 3), vec![0.0, 0.0, 1.0]);
        // Moving: the highest vertex leads; the point moves by the whole time.
        assert!(triangle_find(&mut e, 5.0, hs, [low, high, middle], up, &o));
        assert_eq!(read(&e, o.time, 4), vec![1.0, 1.0, 0.0, 4.0]);
        assert!(triangle_find(&mut e, 5.0, hs, [high, low, middle], up, &o));
        assert_eq!(read(&e, o.point, 3), vec![1.0, 0.0, 4.0]);
        assert!(triangle_find(&mut e, 5.0, hs, [low, middle, high], up, &o));
        assert_eq!(e.mem.f32(o.point), 1.0);
        // Not reaching, or moving away.
        assert!(!triangle_find(&mut e, 0.5, hs, [low, middle, high], up, &o));
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        assert!(!triangle_find(
            &mut e,
            9.0,
            hs,
            [low, middle, high],
            down,
            &o
        ));
    }
}
