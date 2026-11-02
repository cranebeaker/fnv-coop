//! `NiCollision/niboxbv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, float_operand, load_float,
};
use crate::units::nicollision::nispherebv::negated_bits;

layout! {
    /// `NiBoxBV` (Xbox PDB), 0x40 bytes (the PC code allocates 0x40): the
    /// vtable, then `m_kBox`, an `NiBox` (Xbox PDB, 0x3c bytes):
    /// `m_kCenter` at +4, the three unit axes `m_akAxis` at +0x10, +0x1c
    /// and +0x28, the three half extents `m_afExtent` at +0x34.
    pub struct NiBoxBV: 0x40 {
        /// `m_kBox.m_kCenter` (Xbox PDB), x.
        0x04 m_kBox_center_x: u32,
        /// `m_kBox.m_akAxis[0]` (Xbox PDB), x.
        0x10 m_kBox_axis_x: u32,
        /// `m_kBox.m_afExtent[0]` (Xbox PDB).
        0x34 m_kBox_extent: u32,
    }
}

/// The offsets inside the volume of `m_kBox`'s centre, first axis and
/// first extent (see [`NiBoxBV`]).
const CENTER: u32 = 0x04;
const AXES: u32 = 0x10;
const EXTENTS: u32 = 0x34;
/// `NiBoxBV`'s vtable (`010f1814`) and `NiBoundingVolume`'s (`010f16f4`),
/// from the RTTI before them.
const NI_BOX_BV_VTABLE: u32 = 0x010f_1814;
const NI_BOUNDING_VOLUME_VTABLE: u32 = 0x010f_16f4;
/// The memory manager instance (`this` of its allocate and free).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on [`MEMORY_MANAGER`],
/// the size) and its deallocation (`00aa4060`, the block).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The defaults `Create` and the stream loader build a box from: the
/// extents (`011a9490`, 1, 1, 1 in the image), the centre (`011f426c`, zero),
/// and the axes (`011a946c`, `011a9478`, `011a9484`: the unit x, y and z
/// in the image).
const DEFAULT_EXTENTS: u32 = 0x011a_9490;
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
const DEFAULT_AXIS_X: u32 = 0x011a_946c;
const DEFAULT_AXIS_Y: u32 = 0x011a_9478;
const DEFAULT_AXIS_Z: u32 = 0x011a_9484;
/// `NiBox::operator==`, `NiBox::LoadBinary`, `NiBox::SaveBinary` (Xbox
/// PDB; `00eae520`, `00eae5d0`, `00eae630`; `thiscall` on the box, one
/// word).
const NI_BOX_EQUAL: u32 = 0x00ea_e520;
const NI_BOX_LOAD_BINARY: u32 = 0x00ea_e5d0;
const NI_BOX_SAVE_BINARY: u32 = 0x00ea_e630;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`, `thiscall`, the
/// stream) and `NiBoundingVolume::SaveBinary` (`00e9cb10`).
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// The allocator `00aa1070` (cdecl, size) and `NiSprintf` (`00a305b0`,
/// cdecl: buffer, size, format, then the format's arguments).
const ALLOCATE: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = BOX_BV"`.
const BOX_BV_FORMAT: u32 = 0x010f_1844;
/// The `float`s `0.0001` (`010141cc`) and `0.001` (`01017d00`) handed to
/// the intersectors as the pseudodistance threshold and the derivative
/// time step.
const PSEUDODISTANCE_THRESHOLD: u32 = 0x0101_41cc;
const DERIVATIVE_TIME_STEP: u32 = 0x0101_7d00;
/// The intersector constructors the box tests use (`thiscall`: two
/// volumes, then the maximum time, the derivative time step, the
/// pseudodistance threshold and the iteration limit; `RET 0x18`):
/// `00eb3c40` with a sphere volume, `00eb3890` with a capsule volume.
const INTERSECTOR_CONSTRUCTOR_00EB3C40: u32 = 0x00eb_3c40;
const INTERSECTOR_CONSTRUCTOR_00EB3890: u32 = 0x00eb_3890;
/// `NiIntersector::Test` (`00eae6e0`) and `::Find` (`00eae7a0`), each
/// `thiscall` with the two velocities; the contact time (`00eae6f0`, ST0),
/// point (`00eae700`) and normal (`00eae710`); the intersection type
/// (`00be0aa0`); the destructor (`00eae6d0`).
const INTERSECTOR_TEST: u32 = 0x00ea_e6e0;
const INTERSECTOR_FIND: u32 = 0x00ea_e7a0;
const INTERSECTOR_CONTACT_TIME: u32 = 0x00ea_e6f0;
const INTERSECTOR_CONTACT_POINT: u32 = 0x00ea_e700;
const INTERSECTOR_CONTACT_NORMAL: u32 = 0x00ea_e710;
const INTERSECTOR_TYPE: u32 = 0x00be_0aa0;
const INTERSECTOR_DESTRUCTOR: u32 = 0x00ea_e6d0;
/// The float `GetNormal` compares the projections with (`0104da4c`;
/// `FLT_EPSILON` in the image) and the unitize it ends with (`004a0c10`,
/// `thiscall` on the point).
const FLT_EPSILON: u32 = 0x0104_da4c;
const UNITIZE_004A0C10: u32 = 0x004a_0c10;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00e9e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `NiBoxBV::operator_` (`thiscall`; the other volume;
/// `RET 4`): by its body `operator!=`, the negation of this volume's
/// virtual slot +0x24 (`operator==` in the Xbox PDB's vtable) on the other
/// volume.
pub fn ni_box_bv_operator_(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr) -> bool {
    !e.vcall(this.addr(), 0x24, &args![other]).bool()
}

// Translated from 00ea07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::operator==` (engine map `operator__ov2`; `thiscall`; the other
/// volume; `RET 4`): the other volume's `Type` (virtual slot +0xc) must be
/// 1 and its box must equal this one's (`NiBox::operator==` called on the
/// other volume's box with this one's as the argument).
pub fn ni_box_bv_operator_ov2(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 1 {
        return false;
    }
    e.call(
        NI_BOX_EQUAL,
        &args![other.addr() + CENTER, this.addr() + CENTER],
    )
    .bool()
}

// Translated from 00ea07e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::LoadBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`): the
/// base volume's part (`00a89af0`), then the box (`NiBox::LoadBinary`).
pub fn ni_box_bv_load_binary(e: &mut Engine, this: Ptr<NiBoxBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.call(NI_BOX_LOAD_BINARY, &args![this.addr() + CENTER, stream]);
}

// Translated from 00ea0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::SaveBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`):
/// `NiBoundingVolume::SaveBinary`, then the box (`NiBox::SaveBinary`).
pub fn ni_box_bv_save_binary(e: &mut Engine, this: Ptr<NiBoxBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.call(NI_BOX_SAVE_BINARY, &args![this.addr() + CENTER, stream]);
}

// Translated from 00ea0820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoxBV` constructor from parts (no name in the engine map;
/// `thiscall`; the extents, the centre and the three axes, each the
/// address of three floats; `RET 0x14`): the vtable `010f1814`, the
/// centre's words, the extents through `FLD`/`FSTP` (a signalling NaN
/// quieted), then the axes' words. Returns `this`.
pub fn fn_00ea0820(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    extents: Ptr,
    center: Ptr,
    axis_x: Ptr,
    axis_y: Ptr,
    axis_z: Ptr,
) -> Ptr<NiBoxBV> {
    let base = this.addr();
    e.mem.set_u32(base, NI_BOX_BV_VTABLE);
    copy_point(e, base + CENTER, center.addr());
    for axis in 0..3u32 {
        let word = x87_quiet_bits(e.mem.u32(extents.addr() + 4 * axis));
        e.mem.set_u32(base + EXTENTS + 4 * axis, word);
    }
    copy_point(e, base + AXES, axis_x.addr());
    copy_point(e, base + AXES + 0xc, axis_y.addr());
    copy_point(e, base + AXES + 0x18, axis_z.addr());
    this
}

// Translated from 00ea08a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Copy` (Xbox PDB; `thiscall`; the other volume; `RET 4`): the
/// 15 words of the other volume's box (`REP MOVSD`).
pub fn ni_box_bv_copy(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr<NiBoxBV>) {
    for word in 0..15u32 {
        let value = e.mem.u32(other.addr() + CENTER + 4 * word);
        e.mem.set_u32(this.addr() + CENTER + 4 * word, value);
    }
}

// Translated from 00ea08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::UpdateWorldData` (Xbox PDB; `thiscall`; the model volume and
/// the transform; `RET 8`): this volume becomes the model volume moved by
/// the transform (a row-major 3x3 rotation at +0, the translation at
/// +0x24, the scale at +0x30). The centre is rotated (each row's three
/// products summed in the executable's order and rounded), scaled (the
/// scale itself through `FLD`/`FSTP`) and translated; each axis is rotated
/// only and each extent multiplied by the scale. Every component is read
/// before it is written, as in the executable, so the model may be this
/// volume.
pub fn ni_box_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    model: Ptr<NiBoxBV>,
    transform: Ptr,
) {
    let (out, m, t) = (this.addr(), model.addr(), transform.addr());
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    // The rotated centre: (y r1 + x r0) + z r2 per row, in the code's
    // operand order.
    let c0 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 8), op(e, t + 4)),
            x87_mul(ld(e, m + 4), op(e, t)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 8)),
    ));
    let c1 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 8), op(e, t + 0x10)),
            x87_mul(ld(e, t + 0xc), op(e, m + 4)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 0x14)),
    ));
    let c2 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 4), op(e, t + 0x18)),
            x87_mul(ld(e, t + 0x1c), op(e, m + 8)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 0x20)),
    ));
    // `FLD scale; FSTP` (into the model argument's stack slot), then the
    // products with the scale on the stack.
    let scale = x87_load32(x87_quiet_bits(e.mem.u32(t + 0x30)));
    let s0 = x87_store32(x87_mul(x87_load32(c0), scale));
    let s1 = x87_store32(x87_mul(x87_load32(c1), scale));
    let s2 = x87_store32(x87_mul(scale, x87_m32(c2)));
    let c0 = x87_store32(x87_add(ld(e, t + 0x24), x87_m32(s0)));
    let c1 = x87_store32(x87_add(ld(e, t + 0x28), x87_m32(s1)));
    let c2 = x87_store32(x87_add(ld(e, t + 0x2c), x87_m32(s2)));
    e.mem.set_u32(out + 4, c0);
    e.mem.set_u32(out + 8, c1);
    e.mem.set_u32(out + 0xc, c2);

    // The first axis (+0x10).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x14), op(e, t + 4)),
                x87_mul(ld(e, m + 0x10), op(e, t)),
            ),
            x87_mul(ld(e, t + 8), op(e, m + 0x18)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x10), op(e, t + 0xc)),
                x87_mul(ld(e, m + 0x14), op(e, t + 0x10)),
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
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x10 + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x34), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x34, extent);

    // The second axis (+0x1c).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t), op(e, m + 0x1c)),
                x87_mul(ld(e, m + 0x20), op(e, t + 4)),
            ),
            x87_mul(ld(e, t + 8), op(e, m + 0x24)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x20), op(e, t + 0x10)),
                x87_mul(ld(e, t + 0xc), op(e, m + 0x1c)),
            ),
            x87_mul(ld(e, m + 0x24), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0x1c), op(e, m + 0x20)),
                x87_mul(ld(e, m + 0x1c), op(e, t + 0x18)),
            ),
            x87_mul(ld(e, t + 0x20), op(e, m + 0x24)),
        )),
    ];
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x1c + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x38), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x38, extent);

    // The third axis (+0x28).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t), op(e, m + 0x28)),
                x87_mul(ld(e, t + 4), op(e, m + 0x2c)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 8)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0xc), op(e, m + 0x28)),
                x87_mul(ld(e, t + 0x10), op(e, m + 0x2c)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0x1c), op(e, m + 0x2c)),
                x87_mul(ld(e, m + 0x28), op(e, t + 0x18)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 0x20)),
        )),
    ];
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x28 + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x3c), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x3c, extent);
}

/// The intersector `Find` the two box finds run: an intersector on the
/// stack made by `constructor` from the two volumes, the time, `0.001`,
/// `0.0001` (both read from the exe's data through `FLD`/`FSTP`) and 0x20
/// iterations; `Find` with the two velocities; the contact time to
/// `out_time` and, for an intersection type of 3 or 2, the contact point to
/// `out_point` and (when `want_normals`) the contact normal to
/// `out_normal` and its negation to `out_normal_negated`. The intersector
/// is destroyed on both paths.
#[allow(clippy::too_many_arguments)]
fn intersector_find(
    e: &mut Engine,
    constructor: u32,
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
    // The intersector at `[ESP+0xc]` of the 0x5c-byte frame.
    e.with_stack(0x50, |e, intersector| {
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

// Translated from 00ea3950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's sphere column of the find table set up at `00e99129`): the box
/// `Find` test against a sphere volume (cdecl, ten words: the time, the
/// box and its velocity, the other volume and its velocity, `out_time`,
/// `out_point`, `want_normals` (a byte), `out_normal`,
/// `out_normal_negated`), through the intersector constructor `00eb3c40`;
/// see [`intersector_find`].
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea3950(
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
        INTERSECTOR_CONSTRUCTOR_00EB3C40,
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

// Translated from 00ea3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's capsule column of the find table, `00e99137`): [`fn_00ea3950`]
/// with the intersector constructor `00eb3890`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea3c50(
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
        INTERSECTOR_CONSTRUCTOR_00EB3890,
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

/// The absolute value the box tests take of a projection: `FSTP` (the
/// sum rounded to a float), `FLD`, `FABS`, `FSTP`, `FLD`.
fn abs_rounded(value: f64) -> f64 {
    x87_load32(x87_quiet_bits(x87_store32(value)) & 0x7fff_ffff)
}

/// The three projections of `d` (the other volume's centre minus the
/// box's) on the box's axes, in the box tests' operand order, each
/// compared with its extent: whether all three are within (`FCOMPP`
/// against the extent; a NaN is outside).
fn center_inside(e: &Engine, b: u32, d: [f64; 3]) -> bool {
    let op = |a: u32| float_operand(e, a);
    let ld = |a: u32| load_float(e, a);
    let p0 = x87_add(
        x87_add(x87_mul(ld(b + 0x14), d[1]), x87_mul(d[0], op(b + 0x10))),
        x87_mul(ld(b + 0x18), d[2]),
    );
    if !(ld(b + 0x34) >= abs_rounded(p0)) {
        return false;
    }
    let p1 = x87_add(
        x87_add(x87_mul(ld(b + 0x20), d[1]), x87_mul(ld(b + 0x1c), d[0])),
        x87_mul(ld(b + 0x24), d[2]),
    );
    if !(ld(b + 0x38) >= abs_rounded(p1)) {
        return false;
    }
    let p2 = x87_add(
        x87_add(x87_mul(d[1], ld(b + 0x2c)), x87_mul(d[0], op(b + 0x28))),
        x87_mul(ld(b + 0x30), d[2]),
    );
    ld(b + 0x3c) >= abs_rounded(p2)
}

/// The other volume's centre (+4) minus the box's, each component rounded
/// to a float (`FLD`, `FSUB dword`, `FSTP`), loaded back.
fn center_offset(e: &Engine, b: u32, other: u32) -> [f64; 3] {
    [0u32, 4, 8].map(|offset| {
        x87_load32(x87_store32(x87_sub(
            load_float(e, other + 4 + offset),
            float_operand(e, b + 4 + offset),
        )))
    })
}

// Translated from 00ea2030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box row's capsule column of the test
/// table set up at `00e9908d`): the box test against a capsule volume
/// (cdecl, five words: the time, the box, its velocity, the other volume,
/// its velocity). True when the other volume's centre (+4) lies within the
/// box (its offset's projection on each axis, rounded and made absolute,
/// not above the extent); otherwise the intersector made by `00eb3890`
/// (the time, `0.001`, `0.0001`, 0x20 iterations) is run
/// (`NiIntersector::Test` with the two velocities) and the answer is
/// whether its intersection type is 3 or 2. The intersector is destroyed
/// on both paths.
pub fn fn_00ea2030(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
) -> bool {
    let d = center_offset(e, first.addr(), second.addr());
    if center_inside(e, first.addr(), d) {
        return true;
    }
    let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
    let step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
    let maximum_time = x87_quiet_bits(time.to_bits());
    e.with_stack(0x50, |e, intersector| {
        e.call(
            INTERSECTOR_CONSTRUCTOR_00EB3890,
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
            INTERSECTOR_TEST,
            &args![intersector, first_velocity, second_velocity],
        );
        let hit = e.call(INTERSECTOR_TYPE, &args![intersector]).u32() == 3
            || e.call(INTERSECTOR_TYPE, &args![intersector]).u32() == 2;
        e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
        hit
    })
}

// Translated from 00ea3a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::GetNormal` (Xbox PDB; `thiscall`; the out point, the three
/// face signs and the velocity's three floats; `RET 0x1c`): `out` starts as
/// the zero point at `011f426c`; each axis whose sign is 1 is added to it
/// and each whose sign is -1 subtracted (component by component, rounded),
/// but only when the velocity is the zero point or the velocity's
/// projection on that axis is, rounded and made absolute, above the float
/// at `0104da4c` (`FLT_EPSILON` in the image). Unless exactly one axis was
/// applied, `out` is unitized (`004a0c10`). Returns `out`.
#[allow(clippy::too_many_arguments)]
pub fn ni_box_bv_get_normal(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    out: Ptr,
    first: i32,
    second: i32,
    third: i32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
) -> Ptr {
    let (b, out) = (this.addr(), out.addr());
    copy_point(e, out, ZERO_POINT_011F426C);
    let vx = x87_load32(velocity_x.to_bits());
    let vy = x87_load32(velocity_y.to_bits());
    let vz = x87_load32(velocity_z.to_bits());
    // `FUCOMPP` with the zero point's floats: equal, NaN unequal.
    let still = load_float(e, ZERO_POINT_011F426C) == vx
        && load_float(e, ZERO_POINT_011F426C + 4) == vy
        && load_float(e, ZERO_POINT_011F426C + 8) == vz;
    let epsilon = load_float(e, FLT_EPSILON);
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    let mut applied = 0u32;
    let signs = [first, second, third];
    for (axis, sign) in signs.iter().enumerate() {
        let a = b + AXES + 0xc * axis as u32;
        let projection = match axis {
            0 => x87_add(
                x87_add(x87_mul(ld(e, a + 4), vy), x87_mul(ld(e, a), vx)),
                x87_mul(ld(e, a + 8), vz),
            ),
            1 => x87_add(
                x87_add(x87_mul(ld(e, a + 4), vy), x87_mul(vx, op(e, a))),
                x87_mul(ld(e, a + 8), vz),
            ),
            _ => x87_add(
                x87_add(x87_mul(vy, ld(e, a + 4)), x87_mul(vx, ld(e, a))),
                x87_mul(vz, ld(e, a + 8)),
            ),
        };
        // Without a still velocity the projection is tested (`TEST
        // AH,0x41` or `0x5`: on when it is above the epsilon, ordered).
        if !still && !(abs_rounded(projection) > epsilon) {
            continue;
        }
        if *sign == 1 {
            let x = x87_store32(x87_add(ld(e, out), op(e, a)));
            e.mem.set_u32(out, x);
            let y = x87_store32(x87_add(ld(e, a + 4), op(e, out + 4)));
            e.mem.set_u32(out + 4, y);
            let z = x87_store32(x87_add(ld(e, a + 8), op(e, out + 8)));
            e.mem.set_u32(out + 8, z);
            applied += 1;
        } else if *sign == -1 {
            for offset in [0u32, 4, 8] {
                let value = x87_store32(x87_sub(ld(e, out + offset), op(e, a + offset)));
                e.mem.set_u32(out + offset, value);
            }
            applied += 1;
        }
    }
    if applied != 1 {
        e.call(UNITIZE_004A0C10, &args![out]);
    }
    Ptr::new(out)
}

// Translated from 00ea7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// loader of the table at `0126fa80`, set at `00e9901d`): the box volume's
/// stream loader (cdecl, the stream). Makes a box volume as
/// [`ni_box_bv_create`] does and runs its `LoadBinary` (virtual slot 0) on
/// the stream. When the allocation fails the code still makes the virtual
/// call through the null pointer (it reads the vtable at address 0).
/// Answers the volume.
pub fn fn_00ea7490(e: &mut Engine, stream: Ptr) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    let volume = if block != 0 {
        construct_default(e, block)
    } else {
        Ptr::NULL
    };
    e.vcall(volume.addr(), 0, &args![stream]);
    volume
}

/// [`fn_00ea0820`] on `block` with the default extents, centre and axes.
fn construct_default(e: &mut Engine, block: u32) -> Ptr<NiBoxBV> {
    fn_00ea0820(
        e,
        Ptr::new(block),
        Ptr::new(DEFAULT_EXTENTS),
        Ptr::new(ZERO_POINT_011F426C),
        Ptr::new(DEFAULT_AXIS_X),
        Ptr::new(DEFAULT_AXIS_Y),
        Ptr::new(DEFAULT_AXIS_Z),
    )
}

// Translated from 00ea74f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Create` (Xbox PDB): 0x40 bytes from the memory manager and,
/// when that gives a block, a box volume with the default extents,
/// centre and axes ([`fn_00ea0820`]). Answers the volume, or null.
pub fn ni_box_bv_create(e: &mut Engine) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_default(e, block)
}

// Translated from 00ea7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Clone` (Xbox PDB; `thiscall`): 0x40 bytes from the memory
/// manager, the vtable `010f1814` and this volume's 15 box words copied
/// (`REP MOVSD`). Answers the copy, or null.
pub fn ni_box_bv_clone(e: &mut Engine, this: Ptr<NiBoxBV>) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, NI_BOX_BV_VTABLE);
    for word in 0..15u32 {
        let value = e.mem.u32(this.addr() + CENTER + 4 * word);
        e.mem.set_u32(block + CENTER + 4 * word, value);
    }
    Ptr::new(block)
}

// Translated from 00eab7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::AddViewerStrings` (Xbox PDB; `thiscall`, `this` not read; the
/// name and the strings array; `RET 8`): appends `"<name> = BOX_BV"` (a
/// block of `strlen(name) + 0xa` bytes from `00aa1070`, formatted by
/// `NiSprintf`) to the strings array.
pub fn ni_box_bv_add_viewer_strings(e: &mut Engine, _this: Ptr<NiBoxBV>, name: Ptr, strings: Ptr) {
    let size = c_string_length(e, name.addr()).wrapping_add(0xa);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, BOX_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
}

// Translated from 00eac1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoxBV` scalar deleting destructor (engine map; `thiscall`;
/// flags; `RET 4`): the `NiBoundingVolume` vtable (`010f16f4`, the base
/// destructor inlined), then the block goes back to the memory manager
/// (`00aa4060`) when bit 0 of `flags` is set. Returns `this`.
pub fn ni_box_bv_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    flags: u32,
) -> Ptr<NiBoxBV> {
    e.mem.set_u32(this.addr(), NI_BOUNDING_VOLUME_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}
/// `00eb42e0` (cdecl: a point, a box, three out floats; ST0) and `00eb50f0`
/// (cdecl: a segment, a box, four out floats; ST0): the distances the
/// box-sphere test compares with the squared radius.
const POINT_BOX_DISTANCE_00EB42E0: u32 = 0x00eb_42e0;
const SEGMENT_BOX_DISTANCE_00EB50F0: u32 = 0x00eb_50f0;

/// [`center_inside`] with the operand order of [`fn_00ea7530`] (two of the
/// products take the axis component as a loaded value or a memory operand
/// the other way round).
fn center_inside_7530(e: &Engine, b: u32, d: [f64; 3]) -> bool {
    let op = |a: u32| float_operand(e, a);
    let ld = |a: u32| load_float(e, a);
    let p0 = x87_add(
        x87_add(x87_mul(ld(b + 0x14), d[1]), x87_mul(ld(b + 0x10), d[0])),
        x87_mul(ld(b + 0x18), d[2]),
    );
    if !(ld(b + 0x34) >= abs_rounded(p0)) {
        return false;
    }
    let p1 = x87_add(
        x87_add(x87_mul(ld(b + 0x20), d[1]), x87_mul(ld(b + 0x1c), d[0])),
        x87_mul(ld(b + 0x24), d[2]),
    );
    if !(ld(b + 0x38) >= abs_rounded(p1)) {
        return false;
    }
    let p2 = x87_add(
        x87_add(x87_mul(d[1], ld(b + 0x2c)), x87_mul(d[0], ld(b + 0x28))),
        x87_mul(d[2], op(b + 0x30)),
    );
    ld(b + 0x3c) >= abs_rounded(p2)
}

// Translated from 00ea7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box row's sphere column of the test
/// table set up at `00e9907f`): the box test against a sphere volume
/// (cdecl, five words: the time, the box, its velocity, the sphere volume,
/// its velocity; the sphere's centre at +4, radius at +0x10). True when the
/// sphere's centre lies within the box (as in [`fn_00ea2030`]). Otherwise
/// the relative velocity (the sphere's minus the box's, each component
/// rounded) decides: when it equals the zero point at `011f426c`
/// (`FUCOMPP`) the distance is `00eb42e0` of the centre and the box,
/// otherwise `00eb50f0` of the segment from the centre along `time *
/// velocity` (each component rounded) and the box. The answer is whether
/// that distance, rounded, is at most the radius squared (`FCOMPP`,
/// ordered).
pub fn fn_00ea7530(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
) -> bool {
    let (b, s) = (first.addr(), second.addr());
    let d = center_offset(e, b, s);
    if center_inside_7530(e, b, d) {
        return true;
    }
    // The frame: the relative velocity at `[ESP+0x10]` (then the scaled
    // components), the segment (centre, direction) at `[ESP+0x1c]`, two
    // out floats at `[ESP+0xc]` and `[ESP+0x10]`, and two more in the
    // argument slots of the box and the sphere.
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let relative = [0u32, 4, 8].map(|offset| {
            x87_store32(x87_sub(
                load_float(e, second_velocity.addr() + offset),
                float_operand(e, first_velocity.addr() + offset),
            ))
        });
        let [rx, ry, rz] = relative.map(x87_load32);
        let still = load_float(e, ZERO_POINT_011F426C) == rx
            && load_float(e, ZERO_POINT_011F426C + 4) == ry
            && load_float(e, ZERO_POINT_011F426C + 8) == rz;
        let (out_a, out_b, out_c, out_d) = (f, f + 4, f + 8, f + 0xc);
        let distance = if still {
            e.call(
                POINT_BOX_DISTANCE_00EB42E0,
                &args![s + 4, b + 4, out_a, out_c, out_d],
            )
            .f64()
        } else {
            let t = x87_load32(time.to_bits());
            let segment = f + 0x10;
            copy_point(e, segment, s + 4);
            let direction = [
                x87_store32(x87_mul(rx, t)),
                x87_store32(x87_mul(ry, t)),
                x87_store32(x87_mul(t, rz)),
            ];
            for (axis, word) in direction.iter().enumerate() {
                e.mem.set_u32(segment + 0xc + 4 * axis as u32, *word);
            }
            e.call(
                SEGMENT_BOX_DISTANCE_00EB50F0,
                &args![segment, b + 4, out_b, out_a, out_c, out_d],
            )
            .f64()
        };
        let distance = x87_load32(x87_store32(distance));
        let radius = load_float(e, s + 0x10);
        // `FCOMPP; TEST AH,0x41; JP`: true when below or equal.
        distance <= x87_mul(radius, radius)
    })
}

/// `00eab8a0` (`thiscall` on the sphere volume; the radius, the centre's
/// address; `RET 8`): builds an `NiSphereBV` (its vtable `010f1854` by the
/// RTTI) on the stack for [`fn_00ea7730`].
const SPHERE_BV_CONSTRUCT_00EAB8A0: u32 = 0x00ea_b8a0;
/// The radius of that sphere (`01013ea4`, a `float`; 0.01 in the image).
const PROBE_RADIUS: u32 = 0x0101_3ea4;
/// The `double` (0.99 in the image, `0106d508`) the extents are scaled by
/// before a projection is clamped to them.
const CLAMP_FACTOR: u32 = 0x0106_d508;

/// The clamp of [`fn_00ea7730`] for one axis: the projection `p` (a loaded
/// float) above `factor` times the extent (`FCOMP`, ordered) becomes the
/// extent; below `factor` times the negated extent (the negation stored
/// and reloaded) the negated extent; otherwise (a NaN included) it stays.
fn clamp_to_extent(e: &Engine, p: f64, extent: u32, factor: f64) -> f64 {
    let full = load_float(e, extent);
    // `TEST AH,0x5; JP`: on unless the scaled extent is below `p`.
    if x87_mul(full, factor) < p {
        return x87_load32(x87_store32(full));
    }
    let negated = x87_load32(x87_store32(-full));
    // `TEST AH,0x41; JNZ`: kept unless the scaled negation is above `p`.
    if x87_mul(negated, factor) > p {
        return x87_load32(x87_store32(negated));
    }
    p
}

/// One half of [`fn_00ea7730`]: moves the point at `point` into the box
/// `b` moved by its velocity `v` times the contact time at `time_at`: the
/// offset of the point from the moved centre is projected on the three
/// axes (rounded), each projection clamped ([`clamp_to_extent`]), and the
/// point becomes the moved centre plus the axes times the clamped
/// projections (each product and sum rounded, the three axes added one
/// after the other). `second` selects the operand forms of the second
/// half (a few products take a memory operand where the first loads it).
fn clamp_point_into_box(e: &mut Engine, b: u32, v: u32, time_at: u32, point: u32, second: bool) {
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    let st = |x: f64| x87_load32(x87_store32(x));
    let tt = x87_load32(x87_quiet_bits(e.mem.u32(time_at)));
    let moved = [
        x87_store32(x87_mul(ld(e, v), tt)),
        x87_store32(x87_mul(ld(e, v + 4), tt)),
        x87_store32(x87_mul(tt, op(e, v + 8))),
    ];
    let c = [
        st(x87_add(x87_load32(moved[0]), op(e, b + 4))),
        st(x87_add(ld(e, b + 8), x87_m32(moved[1]))),
        st(x87_add(ld(e, b + 0xc), x87_m32(moved[2]))),
    ];
    // The moved centre's y and z come back as memory operands: the same
    // values (stored floats are never signalling).
    let d = [
        st(x87_sub(ld(e, point), c[0])),
        st(x87_sub(ld(e, point + 4), c[1])),
        st(x87_sub(ld(e, point + 8), c[2])),
    ];
    let a = |i: u32| b + 0x10 + 4 * i;
    let (p0, p1, p2) = if second {
        (
            x87_add(
                x87_mul(ld(e, a(2)), d[2]),
                x87_add(x87_mul(ld(e, a(1)), d[1]), x87_mul(d[0], op(e, a(0)))),
            ),
            x87_add(
                x87_add(x87_mul(ld(e, a(4)), d[1]), x87_mul(ld(e, a(3)), d[0])),
                x87_mul(ld(e, a(5)), d[2]),
            ),
            x87_add(
                x87_mul(d[2], ld(e, a(8))),
                x87_add(x87_mul(d[1], ld(e, a(7))), x87_mul(d[0], op(e, a(6)))),
            ),
        )
    } else {
        (
            x87_add(
                x87_mul(ld(e, a(2)), d[2]),
                x87_add(x87_mul(ld(e, a(0)), d[0]), x87_mul(ld(e, a(1)), d[1])),
            ),
            x87_add(
                x87_add(x87_mul(ld(e, a(4)), d[1]), x87_mul(d[0], op(e, a(3)))),
                x87_mul(ld(e, a(5)), d[2]),
            ),
            x87_add(
                x87_add(x87_mul(d[0], ld(e, a(6))), x87_mul(d[1], ld(e, a(7)))),
                x87_mul(d[2], op(e, a(8))),
            ),
        )
    };
    let factor = x87_load64(e.mem.u64(CLAMP_FACTOR));
    let p0 = clamp_to_extent(e, st(p0), b + 0x34, factor);
    let p1 = clamp_to_extent(e, st(p1), b + 0x38, factor);
    let p2 = clamp_to_extent(e, st(p2), b + 0x3c, factor);
    let q2 = if second {
        [
            st(x87_mul(p2, op(e, a(6)))),
            st(x87_mul(ld(e, a(7)), p2)),
            st(x87_mul(p2, op(e, a(8)))),
        ]
    } else {
        [
            st(x87_mul(ld(e, a(6)), p2)),
            st(x87_mul(ld(e, a(7)), p2)),
            st(x87_mul(p2, op(e, a(8)))),
        ]
    };
    let q1 = [
        st(x87_mul(ld(e, a(3)), p1)),
        st(x87_mul(ld(e, a(4)), p1)),
        st(x87_mul(p1, ld(e, a(5)))),
    ];
    let q0 = [
        st(x87_mul(ld(e, a(0)), p0)),
        st(x87_mul(ld(e, a(1)), p0)),
        st(x87_mul(p0, if second { op(e, a(2)) } else { ld(e, a(2)) })),
    ];
    let r = [
        st(x87_add(c[0], q0[0])),
        st(x87_add(q0[1], c[1])),
        st(x87_add(q0[2], c[2])),
    ];
    for axis in 0..3 {
        let s = st(x87_add(r[axis], q1[axis]));
        let word = x87_store32(x87_add(s, q2[axis]));
        e.mem.set_u32(point + 4 * axis as u32, word);
    }
}

// Translated from 00ea7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, seven words: the time, box `a`, its
/// velocity, box `b`, its velocity, the address of the contact time, the
/// contact point): moves the contact point into both boxes. A small sphere
/// volume is built on the stack (`00eab8a0` with radius 1.0 and the zero
/// point, then the radius from `01013ea4` and the point as its centre);
/// when the box-sphere test ([`fn_00ea7530`], the sphere not moving) says
/// it does not touch box `a`, the point is clamped into box `a` moved to
/// the contact time ([`clamp_point_into_box`]); then the same for box `b`
/// (the sphere still at the original point). The function answers nothing
/// its callers read (`EAX` holds the last test's byte or a point word).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea7730(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
    contact_time: Ptr,
    point: Ptr,
) {
    let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
    // The sphere volume `[ESP+0x4c]` (vtable, centre, radius).
    e.with_stack(0x14, |e, sphere| {
        let s = sphere.addr();
        e.call(
            SPHERE_BV_CONSTRUCT_00EAB8A0,
            &args![s, 1.0f32, ZERO_POINT_011F426C],
        );
        let radius = x87_quiet_bits(e.mem.u32(PROBE_RADIUS));
        e.mem.set_u32(s + 0x10, radius);
        copy_point(e, s + 4, point.addr());
        let zero = Ptr::new(ZERO_POINT_011F426C);
        if !fn_00ea7530(e, time, a, a_velocity, sphere, zero) {
            clamp_point_into_box(
                e,
                a.addr(),
                a_velocity.addr(),
                contact_time.addr(),
                point.addr(),
                false,
            );
        }
        if !fn_00ea7530(e, time, b, b_velocity, sphere, zero) {
            clamp_point_into_box(
                e,
                b.addr(),
                b_velocity.addr(),
                contact_time.addr(),
                point.addr(),
                true,
            );
        }
    });
}

/// `NiPoint3::Dot` (engine map, `004b6190`; `thiscall` on one point, the
/// other point's address; `RET 4`; ST0).
const POINT_DOT: u32 = 0x004b_6190;
/// `004b3800` (`thiscall` on a point `p`; out, `q`; `RET 8`): `p x q` into
/// `out`.
const POINT_CROSS_004B3800: u32 = 0x004b_3800;

/// `FSTP float` then `FLD float`: the value rounded to a float, loaded back.
fn rounded(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The separating-axis decision the box-box test makes for each axis: `r`
/// the summed radii (a stored float), `d0` the centre offset's projection
/// and `d1` the projection at the end of the motion. Separated when both
/// are above `r`, or, when `d0` is not above `r` (a NaN included), when
/// `d0` and then `d1` (asked only then) are below `-r` (stored and
/// reloaded). `d1_above` gives `d1` for the first case, `d1_below` for the
/// second (they differ only in how some axes compute it).
fn axis_separated(
    e: &mut Engine,
    r: f64,
    d0: f64,
    d1_above: impl FnOnce(&mut Engine) -> f64,
    d1_below: impl FnOnce(&mut Engine) -> f64,
) -> bool {
    // `TEST AH,0x5; JP`: the second case unless `r` is below `d0`.
    if r < d0 {
        let d1 = d1_above(e);
        return d1 > r;
    }
    let negated = rounded(-r);
    // `TEST AH,0x41; JNZ`: on to the next axis unless `-r` is above `d0`.
    if !(negated > d0) {
        return false;
    }
    let d1 = d1_below(e);
    negated > d1
}

// Translated from 00ea0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's box column of the test table set up at `00e99086`): the moving
/// box-box test (cdecl, five words: the time, box `a`, its velocity, box
/// `b`, its velocity). With `T` the centre offset (`b - a`), `V` the
/// relative velocity (`b`'s minus `a`'s) and `T1 = V * time + T` (every
/// component rounded), the boxes are apart when one of these axes
/// separates them ([`axis_separated`], all values rounded where the code
/// stores them): `a`'s three axes, `b`'s three axes, the nine axis cross
/// products (with `T1` projected in extended precision), and the six
/// products of `V` with the axes (with `|L . (V x T)|` against the summed
/// radii, `NiPoint3::Dot` and `004b3800`). Answers false as soon as one
/// separates, true otherwise.
pub fn fn_00ea0ad0(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
) -> bool {
    let (a, av, b, bv) = (a.addr(), a_velocity.addr(), b.addr(), b_velocity.addr());
    // The frame: `[ESP+0x34]` the three floats of `T1` (later `V x T`),
    // `[ESP+0x9c]` a copy of `V`, `[ESP+0xa8]` of `T1`, `[ESP+0xb4]` of `T`.
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let (block, v_copy, t1_copy, t_copy) = (f, f + 0xc, f + 0x18, f + 0x24);
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let op = |e: &Engine, x: u32| float_operand(e, x);
        let mul = x87_mul;
        let add = x87_add;
        let sub = x87_sub;
        let v = [0u32, 4, 8].map(|k| rounded(sub(ld(e, bv + k), op(e, av + k))));
        let t = [0u32, 4, 8].map(|k| rounded(sub(ld(e, b + 4 + k), op(e, a + 4 + k))));
        for k in 0..3 {
            e.mem.set_u32(v_copy + 4 * k, x87_store32(v[k as usize]));
            e.mem.set_u32(t_copy + 4 * k, x87_store32(t[k as usize]));
        }
        let time = x87_load32(time.to_bits());
        let moved = [
            rounded(mul(v[0], time)),
            rounded(mul(v[1], time)),
            rounded(mul(time, v[2])),
        ];
        let t1 = [0usize, 1, 2].map(|k| rounded(add(moved[k], t[k])));
        for k in 0..3 {
            let bits = x87_store32(t1[k as usize]);
            e.mem.set_u32(block + 4 * k, bits);
            e.mem.set_u32(t1_copy + 4 * k, bits);
        }
        let axis_a = |i: u32| a + 0x10 + 0xc * i;
        let axis_b = |j: u32| b + 0x10 + 0xc * j;
        let extent_a = |i: u32| a + 0x34 + 4 * i;
        let extent_b = |j: u32| b + 0x34 + 4 * j;
        // `(x + y) + z` of three products, each `FLD` of the first address
        // times the second as a memory operand.
        let dot = |e: &Engine, terms: [(u32, u32); 3]| {
            add(
                add(
                    mul(ld(e, terms[0].0), op(e, terms[0].1)),
                    mul(ld(e, terms[1].0), op(e, terms[1].1)),
                ),
                mul(ld(e, terms[2].0), op(e, terms[2].1)),
            )
        };
        let (a0, a1, a2) = (axis_a(0), axis_a(1), axis_a(2));
        let (b0, b1, b2) = (axis_b(0), axis_b(1), axis_b(2));
        // The axis dot products `c[i][j] = Ai . Bj`, in the code's operand
        // forms.
        let c = [
            [
                rounded(dot(e, [(b0, a0), (b0 + 4, a0 + 4), (b0 + 8, a0 + 8)])),
                rounded(dot(e, [(b1 + 4, a0 + 4), (b1, a0), (b1 + 8, a0 + 8)])),
                rounded(dot(e, [(b2 + 4, a0 + 4), (b2, a0), (b2 + 8, a0 + 8)])),
            ],
            [
                rounded(dot(e, [(a1 + 4, b0 + 4), (b0, a1), (a1 + 8, b0 + 8)])),
                rounded(dot(e, [(b1 + 4, a1 + 4), (a1, b1), (b1 + 8, a1 + 8)])),
                rounded(dot(e, [(b2 + 4, a1 + 4), (b2, a1), (b2 + 8, a1 + 8)])),
            ],
            [
                rounded(dot(e, [(a2 + 4, b0 + 4), (a2, b0), (a2 + 8, b0 + 8)])),
                rounded(dot(e, [(b1 + 4, a2 + 4), (b1, a2), (b1 + 8, a2 + 8)])),
                rounded(dot(e, [(b2 + 4, a2 + 4), (b2, a2), (b2 + 8, a2 + 8)])),
            ],
        ];
        let abs_c = c.map(|row| row.map(|value| rounded(value.abs())));
        // `T . Ai` and `T1 . Ai`: the first axis takes x and y as memory
        // operands and loads z; the others load y and z.
        let project_a = |e: &Engine, i: u32, p: [f64; 3]| {
            let axis = axis_a(i);
            if i == 0 {
                add(
                    add(mul(p[0], op(e, axis)), mul(p[1], op(e, axis + 4))),
                    mul(ld(e, axis + 8), p[2]),
                )
            } else {
                add(
                    add(mul(ld(e, axis + 4), p[1]), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                )
            }
        };
        let d_a = [0, 1, 2].map(|i| rounded(project_a(e, i, t)));
        let d1_a = [0, 1, 2].map(|i| rounded(project_a(e, i, t1)));
        for i in 0..3usize {
            let radius_b = rounded(add(
                add(
                    mul(ld(e, extent_b(1)), abs_c[i][1]),
                    mul(ld(e, extent_b(0)), abs_c[i][0]),
                ),
                mul(abs_c[i][2], op(e, extent_b(2))),
            ));
            let r = rounded(add(radius_b, op(e, extent_a(i as u32))));
            let d1 = d1_a[i];
            if axis_separated(e, r, d_a[i], |_| d1, |_| d1) {
                return false;
            }
        }
        // `b`'s axes: `T . Bj` and, when needed, `T1 . Bj` (on the second
        // side of `b`'s second and third axes through `NiPoint3::Dot` on the
        // copy of `T1`, unrounded).
        let project_b = |e: &Engine, j: u32, p: [f64; 3]| {
            let axis = axis_b(j);
            match j {
                0 => add(
                    add(mul(p[1], op(e, axis + 4)), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                ),
                1 => add(
                    add(mul(ld(e, axis + 4), p[1]), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                ),
                _ => add(
                    add(mul(p[0], ld(e, axis)), mul(p[1], ld(e, axis + 4))),
                    mul(ld(e, axis + 8), p[2]),
                ),
            }
        };
        for j in 0..3u32 {
            let ju = j as usize;
            let d0 = rounded(project_b(e, j, t));
            let radius_a = rounded(add(
                add(
                    mul(ld(e, extent_a(1)), abs_c[1][ju]),
                    mul(ld(e, extent_a(0)), abs_c[0][ju]),
                ),
                mul(ld(e, extent_a(2)), abs_c[2][ju]),
            ));
            let r = rounded(add(radius_a, op(e, extent_b(j))));
            let above = |e: &mut Engine| rounded(project_b(e, j, t1));
            let separated = if j == 0 {
                axis_separated(e, r, d0, above, |e| rounded(project_b(e, 0, t1)))
            } else {
                axis_separated(e, r, d0, above, |e| {
                    e.call(POINT_DOT, &args![axis_b(j), t1_copy]).f64()
                })
            };
            if separated {
                return false;
            }
        }
        // The nine cross products `Ai x Bj`: `d0` rounded, `d1` in extended
        // precision; the radii as the two rounded sums of the code.
        let (da, d1a) = (d_a, d1_a);
        let e_a = [0u32, 1, 2].map(extent_a);
        let e_b = [0u32, 1, 2].map(extent_b);
        // (d0, d1, the first radius term, the second) per axis.
        type Cross = (f64, f64, f64, f64);
        let crosses: [Cross; 9] = [
            (
                rounded(sub(mul(da[2], c[1][0]), mul(c[2][0], da[1]))),
                sub(mul(d1a[2], c[1][0]), mul(c[2][0], d1a[1])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[0][2]),
                    mul(abs_c[0][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][0]),
                    mul(ld(e, e_a[1]), abs_c[2][0]),
                )),
            ),
            (
                rounded(sub(mul(da[2], c[1][1]), mul(c[2][1], da[1]))),
                sub(mul(d1a[2], c[1][1]), mul(c[2][1], d1a[1])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[0][2]),
                    mul(abs_c[0][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][1]),
                    mul(ld(e, e_a[1]), abs_c[2][1]),
                )),
            ),
            (
                rounded(sub(mul(da[2], c[1][2]), mul(c[2][2], da[1]))),
                sub(mul(d1a[2], c[1][2]), mul(c[2][2], d1a[1])),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][2]),
                    mul(ld(e, e_a[1]), abs_c[2][2]),
                )),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[0][0]),
                    mul(ld(e, e_b[0]), abs_c[0][1]),
                )),
            ),
            (
                rounded(sub(mul(c[2][0], da[0]), mul(da[2], c[0][0]))),
                sub(mul(c[2][0], d1a[0]), mul(d1a[2], c[0][0])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[1][2]),
                    mul(abs_c[1][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][0]),
                    mul(ld(e, e_a[0]), abs_c[2][0]),
                )),
            ),
            (
                rounded(sub(mul(c[2][1], da[0]), mul(da[2], c[0][1]))),
                sub(mul(c[2][1], d1a[0]), mul(d1a[2], c[0][1])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[1][2]),
                    mul(abs_c[1][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][1]),
                    mul(ld(e, e_a[0]), abs_c[2][1]),
                )),
            ),
            (
                rounded(sub(mul(c[2][2], da[0]), mul(da[2], c[0][2]))),
                sub(mul(c[2][2], d1a[0]), mul(d1a[2], c[0][2])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[1][0]),
                    mul(ld(e, e_b[0]), abs_c[1][1]),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][2]),
                    mul(ld(e, e_a[0]), abs_c[2][2]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][0]), mul(c[1][0], da[0]))),
                sub(mul(c[0][0], d1a[1]), mul(c[1][0], d1a[0])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[2][2]),
                    mul(abs_c[2][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][0]),
                    mul(ld(e, e_a[0]), abs_c[1][0]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][1]), mul(c[1][1], da[0]))),
                sub(mul(c[0][1], d1a[1]), mul(c[1][1], d1a[0])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[2][2]),
                    mul(abs_c[2][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][1]),
                    mul(ld(e, e_a[0]), abs_c[1][1]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][2]), mul(da[0], c[1][2]))),
                sub(mul(c[0][2], d1a[1]), mul(d1a[0], c[1][2])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[2][0]),
                    mul(ld(e, e_b[0]), abs_c[2][1]),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][2]),
                    mul(ld(e, e_a[0]), abs_c[1][2]),
                )),
            ),
        ];
        for (d0, d1, first, second) in crosses {
            let r = rounded(add(first, second));
            if axis_separated(e, r, d0, |_| d1, |_| d1) {
                return false;
            }
        }
        // The motion axes: `N = V x T` (into the first block), the axes'
        // products with `V`, and for each axis `L` the test of
        // `|L . N|` against the radii.
        e.call(POINT_CROSS_004B3800, &args![v_copy, block, t_copy]);
        let dot_call =
            |e: &mut Engine, this: u32, other: u32| e.call(POINT_DOT, &args![this, other]).f64();
        let abs_rounded_value = |value: f64| rounded(value.abs());
        // `a`'s axes.
        let n1 = rounded(dot_call(e, v_copy, a1));
        let n2 = rounded(dot_call(e, v_copy, a2));
        let abs_n2 = abs_rounded_value(n2);
        let abs_n1 = abs_rounded_value(n1);
        let reach = abs_rounded_value(dot_call(e, a0, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n2, c[1][1]), mul(n1, c[2][1])))),
            abs_rounded_value(rounded(sub(mul(n2, c[1][0]), mul(n1, c[2][0])))),
            abs_rounded_value(rounded(sub(mul(n2, c[1][2]), mul(n1, c[2][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(ld(e, e_a[2]), abs_n1), mul(ld(e, e_a[1]), abs_n2))),
        ));
        // `TEST AH,0x5; JNP`: separated when the radius is below.
        if radius < reach {
            return false;
        }
        let n0 = rounded(dot_call(e, v_copy, a0));
        let abs_n0 = abs_rounded_value(n0);
        let reach = abs_rounded_value(dot_call(e, a1, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n0, c[2][1]), mul(n2, c[0][1])))),
            abs_rounded_value(rounded(sub(mul(n0, c[2][0]), mul(n2, c[0][0])))),
            abs_rounded_value(rounded(sub(mul(n0, c[2][2]), mul(n2, c[0][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(abs_n2, op(e, e_a[0])), mul(ld(e, e_a[2]), abs_n0))),
        ));
        if radius < reach {
            return false;
        }
        let reach = abs_rounded_value(dot_call(e, a2, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n1, c[0][1]), mul(n0, c[1][1])))),
            abs_rounded_value(rounded(sub(mul(n1, c[0][0]), mul(n0, c[1][0])))),
            abs_rounded_value(rounded(sub(mul(n1, c[0][2]), mul(n0, c[1][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(abs_n1, op(e, e_a[0])), mul(ld(e, e_a[1]), abs_n0))),
        ));
        if radius < reach {
            return false;
        }
        // `b`'s axes.
        let m1 = rounded(dot_call(e, v_copy, b1));
        let m2 = rounded(dot_call(e, v_copy, b2));
        let abs_m2 = abs_rounded_value(m2);
        let abs_m1 = abs_rounded_value(m1);
        let reach = abs_rounded_value(dot_call(e, b0, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m2, c[1][1]), mul(m1, c[1][2])))),
            abs_rounded_value(rounded(sub(mul(m2, c[0][1]), mul(m1, c[0][2])))),
            abs_rounded_value(rounded(sub(mul(m2, c[2][1]), mul(m1, c[2][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m1, op(e, e_b[2])), mul(abs_m2, op(e, e_b[1])))),
        ));
        if radius < reach {
            return false;
        }
        let m0 = rounded(dot_call(e, v_copy, b0));
        let abs_m0 = abs_rounded_value(m0);
        let reach = abs_rounded_value(dot_call(e, b1, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m0, c[1][2]), mul(m2, c[1][0])))),
            abs_rounded_value(rounded(sub(mul(m0, c[0][2]), mul(m2, c[0][0])))),
            abs_rounded_value(rounded(sub(mul(m0, c[2][2]), mul(m2, c[2][0])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m0, op(e, e_b[2])), mul(abs_m2, op(e, e_b[0])))),
        ));
        if radius < reach {
            return false;
        }
        let reach = abs_rounded_value(dot_call(e, b2, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m1, c[1][0]), mul(m0, c[1][1])))),
            abs_rounded_value(rounded(sub(mul(m1, c[0][0]), mul(m0, c[0][1])))),
            abs_rounded_value(rounded(sub(mul(m1, c[2][0]), mul(m0, c[2][1])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m0, op(e, e_b[1])), mul(abs_m1, op(e, e_b[0])))),
        ));
        !(radius < reach)
    })
}

/// One of the box's own axes in [`fn_00ea2190`]: `p` the projection of
/// the first vertex's offset from the centre, `q1` and `q2` those of the
/// two edges from that vertex, `w` the relative velocity's (all stored
/// floats) and `extent` the box's half extent on the axis (loaded). Above
/// the extent: the lowest vertex (`p` plus `0`, `q1` or `q2`, the sum
/// rounded) must be above it, then that vertex moved by `w` times the time
/// (extended precision); below the negated extent (stored and reloaded)
/// the same with the highest vertex. The comparisons take a NaN the way
/// the code's flag tests do: a NaN `q1` or `q2` picks the other sum.
fn triangle_box_axis_separated(p: f64, q1: f64, q2: f64, w: f64, extent: f64, time: f64) -> bool {
    let moved = |x: f64| x87_add(x, x87_mul(w, time));
    // `TEST AH,0x5; JP`: the negative side unless the extent is below `p`.
    if extent < p {
        let lowest = if q1 > 0.0 {
            if q2 >= 0.0 {
                // The first vertex is the lowest: `p` itself, already above.
                return extent < moved(p);
            }
            rounded(x87_add(p, q2))
        } else if q2 <= q1 {
            rounded(x87_add(p, q2))
        } else {
            rounded(x87_add(p, q1))
        };
        return extent < lowest && extent < moved(lowest);
    }
    let negated = rounded(-extent);
    // `TEST AH,0x41; JNZ`: on to the next axis unless the negated extent
    // is above `p`.
    if !(negated > p) {
        return false;
    }
    let highest = if q1 <= 0.0 {
        if q2 <= 0.0 {
            return moved(p) < negated;
        }
        rounded(x87_add(p, q2))
    } else if q2 >= q1 {
        rounded(x87_add(p, q2))
    } else {
        rounded(x87_add(p, q1))
    };
    highest < negated && moved(highest) < negated
}

/// The other axes of [`fn_00ea2190`] (the triangle's normal and the nine
/// products of a box axis with an edge): `d` the first vertex's offset
/// projected, `k` how far the one vertex off the first edge's line lies
/// from it on the axis (the other vertex projects to `d`; zero for the
/// normal), `radius` the box's projected radius and `w` the relative
/// velocity's projection (stored floats). Above the radius the lower end
/// (`d`, or `d + k` rounded when `k` is negative or a NaN) must be above
/// it, then that end moved by `w` times the time; below the negated radius
/// (stored and reloaded) the same with the higher end.
fn triangle_edge_axis_separated(d: f64, k: f64, radius: f64, w: f64, time: f64) -> bool {
    let moved = |x: f64| x87_add(x, x87_mul(w, time));
    // `TEST AH,0x5; JP`: the negative side unless the radius is below `d`.
    if radius < d {
        if k >= 0.0 {
            return moved(d) > radius;
        }
        let lowest = rounded(x87_add(d, k));
        return lowest > radius && moved(lowest) > radius;
    }
    let negated = rounded(-radius);
    // `TEST AH,0x41; JNZ`: on to the next axis unless the negated radius
    // is above `d`.
    if !(negated > d) {
        return false;
    }
    if k <= 0.0 {
        return moved(d) < negated;
    }
    let highest = rounded(x87_add(d, k));
    highest < negated && moved(highest) < negated
}

// Translated from 00ea2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the moving box-triangle test (cdecl, seven
/// words: the time, the box volume, its velocity, the triangle's three
/// vertices, the triangle's velocity). With `W` the relative velocity
/// (the triangle's minus the box's), `E1` and `E2` the edges from the
/// first vertex, `N = E1 x E2` and `D` the first vertex's offset from the
/// box centre (every component rounded), the two are apart when one of
/// these axes separates them: the normal ([`triangle_edge_axis_separated`]
/// with the radius `|e1 (N . A1)| + |e0 (N . A0)| + |e2 (N . A2)|`), the
/// box's three axes ([`triangle_box_axis_separated`]), then for each box
/// axis its products with `E1`, `E2` and `E2 - E1` (the first axis's
/// computed in place, the others through `004b3800`, the second axis's
/// last products' projections through `NiPoint3::Dot`; the third edge's
/// projections the differences of the first two's). Every dot product is
/// `(x + y) + z` and rounded where the code stores it. Answers false as
/// soon as one axis separates, true otherwise.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea2190(
    e: &mut Engine,
    time: f32,
    bx: Ptr,
    box_velocity: Ptr,
    v0: Ptr,
    v1: Ptr,
    v2: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    let (b, bv, tv) = (bx.addr(), box_velocity.addr(), triangle_velocity.addr());
    let (v0, v1, v2) = (v0.addr(), v1.addr(), v2.addr());
    // The frame: `[ESP+0x28]` a copy of the current product axis (`L`),
    // `[ESP+0x34]` `W`, `[ESP+0x74]` the product `004b3800` writes (first
    // `E1`, unused by then), `[ESP+0x84]` a copy of `E1`, `[ESP+0x90]` of
    // `E2`, `[ESP+0x9c]` of `D`.
    e.with_stack(0x48, |e, frame| {
        let f = frame.addr();
        let (l_copy, w_at, out, e1_copy, e2_copy, d_copy) =
            (f, f + 0xc, f + 0x18, f + 0x24, f + 0x30, f + 0x3c);
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let (add, sub, mul) = (x87_add, x87_sub, x87_mul);
        let dot = |p: [f64; 3], q: [f64; 3]| {
            rounded(add(add(mul(p[0], q[0]), mul(p[1], q[1])), mul(p[2], q[2])))
        };
        let cross = |p: [f64; 3], q: [f64; 3]| {
            [
                rounded(sub(mul(p[1], q[2]), mul(p[2], q[1]))),
                rounded(sub(mul(p[2], q[0]), mul(p[0], q[2]))),
                rounded(sub(mul(p[0], q[1]), mul(p[1], q[0]))),
            ]
        };
        let difference = |e: &Engine, p: u32, q: u32| {
            [0u32, 4, 8].map(|k| rounded(sub(ld(e, p + k), ld(e, q + k))))
        };
        let store = |e: &mut Engine, at: u32, v: [f64; 3]| {
            for (k, x) in v.iter().enumerate() {
                e.mem.set_u32(at + 4 * k as u32, x87_store32(*x));
            }
        };
        let w = difference(e, tv, bv);
        let e1 = difference(e, v1, v0);
        let e2 = difference(e, v2, v0);
        store(e, w_at, w);
        store(e, e1_copy, e1);
        store(e, e2_copy, e2);
        let n = cross(e1, e2);
        let d = difference(e, v0, b + CENTER);
        store(e, d_copy, d);
        let axis = |e: &Engine, i: u32| {
            let a = b + AXES + 0xc * i;
            [ld(e, a), ld(e, a + 4), ld(e, a + 8)]
        };
        let extent = |e: &Engine, i: u32| ld(e, b + EXTENTS + 4 * i);
        let t = x87_load32(time.to_bits());
        let axes = [axis(e, 0), axis(e, 1), axis(e, 2)];
        let ext = [extent(e, 0), extent(e, 1), extent(e, 2)];
        let na = axes.map(|a| dot(a, n));
        // The normal: every vertex projects to `D . N`.
        let term = |x: f64, y: f64| rounded(rounded(mul(x, y)).abs());
        let radius = rounded(add(
            add(term(ext[1], na[1]), term(ext[0], na[0])),
            term(na[2], ext[2]),
        ));
        if triangle_edge_axis_separated(dot(d, n), 0.0, radius, dot(w, n), t) {
            return false;
        }
        // The box's axes.
        let mut q1 = [0.0; 3];
        let mut q2 = [0.0; 3];
        for i in 0..3 {
            let a = axes[i];
            let p = dot(a, d);
            q1[i] = dot(a, e1);
            q2[i] = dot(a, e2);
            if triangle_box_axis_separated(p, q1[i], q2[i], dot(a, w), ext[i], t) {
                return false;
            }
        }
        let q3 = [0usize, 1, 2].map(|i| rounded(sub(q2[i], q1[i])));
        // The box axis `i` times an edge: the radius from the edge's
        // projections on the other two axes.
        let edge_radius = |i: usize, q: [f64; 3]| {
            let (j, k) = match i {
                0 => (1, 2),
                1 => (0, 2),
                _ => (0, 1),
            };
            rounded(add(term(ext[j], q[k]), term(ext[k], q[j])))
        };
        // The product axis as `004b3800` returns it, copied to `L`.
        let product = |e: &mut Engine, i: u32, edge: u32| {
            let ret = e.call(POINT_CROSS_004B3800, &args![b + AXES + 0xc * i, out, edge]);
            let at = ret.eax;
            for k in 0..3 {
                let word = e.mem.u32(at + 4 * k);
                e.mem.set_u32(l_copy + 4 * k, word);
            }
            [ld(e, l_copy), ld(e, l_copy + 4), ld(e, l_copy + 8)]
        };
        for i in 0..3usize {
            // `k` for `E1` is `N . Ai`, for `E2` and `E2 - E1` its negation.
            let k = na[i];
            let (d1, w1) = if i == 0 {
                let l = cross(axes[0], e1);
                (dot(l, d), dot(l, w))
            } else {
                let l = product(e, i as u32, e1_copy);
                (dot(l, d), dot(l, w))
            };
            if triangle_edge_axis_separated(d1, k, edge_radius(i, q1), w1, t) {
                return false;
            }
            let (d2, w2) = match i {
                0 => {
                    let l = cross(axes[0], e2);
                    (dot(l, d), dot(l, w))
                }
                1 => {
                    let l = product(e, 1, e2_copy);
                    (dot(l, d), dot(l, w))
                }
                _ => {
                    product(e, 2, e2_copy);
                    let d2 = e.call(POINT_DOT, &args![l_copy, d_copy]).f64();
                    let w2 = e.call(POINT_DOT, &args![l_copy, w_at]).f64();
                    (rounded(d2), rounded(w2))
                }
            };
            if triangle_edge_axis_separated(d2, -k, edge_radius(i, q2), w2, t) {
                return false;
            }
            let d3 = rounded(sub(d2, d1));
            let w3 = rounded(sub(w2, w1));
            if triangle_edge_axis_separated(d3, -k, edge_radius(i, q3), w3, t) {
                return false;
            }
        }
        true
    })
}

/// The `double` (`010c9300`; 2^-23 in the image) a new contact time must
/// exceed the current one by before [`fn_00ea7d40`] takes its axis.
const TIME_SLACK_010C9300: u32 = 0x010c_9300;
/// The `float` 0.5 (`01016248` in the image) the overlap case halves the
/// sum of the two centres with.
const HALF_01016248: u32 = 0x0101_6248;
/// The `double` 1.0 (`01012070` in the image) the normal's squared length
/// is compared with.
const ONE_01012070: u32 = 0x0101_2070;
/// The `float` `1e-6` (`010959f4` in the image): an edge case whose
/// `|1 - c^2|` is not above it takes its own axis's offset as zero.
const PARALLEL_LIMIT_010959F4: u32 = 0x0109_59f4;
/// `00439e90` and `00439ef0` (`thiscall` on a point `p`; out, `q`; `RET 8`;
/// no Xbox name): by their bodies `p + q` and `p - q` into `out`,
/// returning `out`.
const POINT_SUM_00439E90: u32 = 0x0043_9e90;
const POINT_DIFFERENCE_00439EF0: u32 = 0x0043_9ef0;
/// `004a3760` (cdecl: out, a `float`, a point; no Xbox name): by its body
/// the point times the float into `out`, returning `out`.
const POINT_SCALE_004A3760: u32 = 0x004a_3760;
/// `004a7290` (`thiscall` on a point; ST0; no Xbox name): by its body the
/// point's squared length.
const POINT_LENGTH_SQUARED_004A7290: u32 = 0x004a_7290;

/// The bytes of [`fn_00ea7d40`]'s frame (`SUB ESP,0x1e8` under the four
/// saved registers; the locals sit at `[ESP+0x10]` to `[ESP+0x1f7]`) and the
/// locals whose address it hands to its callees: the copy of the relative
/// velocity `V` (`[ESP+0xd8]`), the point `D` the edge cases project
/// (`[ESP+0x54]`), and the overlap case's sum of the centres (`[ESP+0xe4]`)
/// and half of it (`[ESP+0xa8]`).
const FIND_FRAME: u32 = 0x1f8;
const FIND_V_COPY: u32 = 0xd8;
const FIND_D_POINT: u32 = 0x54;
const FIND_CENTRE_SUM: u32 = 0xe4;
const FIND_HALF_SUM: u32 = 0xa8;

/// What [`fn_00ea7d40`] has found so far and what its contact cases read:
/// the arguments, the frame, the axis dot products `c[i][j] = Ai . Bj`, the
/// centre offset `T`, the contact axis (0 when none was taken) and its side
/// (1 or -1, 0 when none).
struct BoxFind {
    time: u32,
    a: u32,
    va: u32,
    b: u32,
    vb: u32,
    out_time: u32,
    point: u32,
    want_normals: bool,
    normal: u32,
    negated: u32,
    frame: u32,
    c: [[f64; 3]; 3],
    t: [f64; 3],
    axis: u32,
    side: i32,
}

/// One separating axis of [`fn_00ea7d40`]: `r` the summed projected radii,
/// `d0` the centre offset's projection now and `d1` (asked only when
/// needed; the face axes have it computed before, the same value) its
/// projection at the end of the motion. Above `r` (`TEST AH,0x5; JP` on
/// `r` against `d0`): apart when `d1` is above `r` too, else the boxes meet
/// at `(r - d0) * time_above / (d1 - d0)` (rounded); below `-r` (stored and
/// reloaded) the same with `(r + d0) * -time / (d1 - d0)`. A NaN takes
/// neither side. A meeting time above the current contact time plus
/// [`TIME_SLACK_010C9300`] (the current one is the `0.0` of `FLDZ` for the
/// first axis, `*out_time` after) becomes the contact time, with this axis
/// and side. Returns true when the axis separates the boxes.
#[allow(clippy::too_many_arguments)]
fn find_axis(
    e: &mut Engine,
    s: &mut BoxFind,
    axis: u32,
    r: f64,
    d0: f64,
    d1: impl FnOnce() -> f64,
    time_above: f64,
    time: f64,
) -> bool {
    let (t, side) = if r < d0 {
        let d1 = d1();
        // `TEST AH,0x41; JNZ`: on unless `d1` is above `r`.
        if d1 > r {
            return true;
        }
        let t = rounded(x87_div(
            x87_mul(x87_sub(r, d0), time_above),
            x87_sub(d1, d0),
        ));
        (t, 1)
    } else {
        let negated = rounded(-r);
        // `TEST AH,0x41; JNZ`: on to the next axis unless `-r` is above `d0`.
        if !(negated > d0) {
            return false;
        }
        let d1 = d1();
        if negated > d1 {
            return true;
        }
        let t = rounded(x87_div(x87_mul(x87_add(r, d0), -time), x87_sub(d1, d0)));
        (t, -1)
    };
    let current = if axis == 1 {
        0.0
    } else {
        load_float(e, s.out_time)
    };
    let slack = f64::from_bits(e.mem.u64(TIME_SLACK_010C9300));
    // `FCOMPP; TEST AH,0x5; JP`: taken only when the slack sum is below.
    if x87_add(current, slack) < t {
        e.mem.set_u32(s.out_time, x87_store32(t));
        s.axis = axis;
        s.side = side;
    }
    false
}

/// The offset along one box axis of a contact point: `c` above zero gives
/// `positive` (`FILD` of the side or its negation) times the half extent at
/// `extent` (a memory operand), below zero the other sign, zero or a NaN
/// `0.0` (the `FLDZ` stored), each product rounded.
fn signed_extent(e: &Engine, c: f64, positive: i32, extent: u32) -> f64 {
    if c > 0.0 {
        rounded(x87_mul(f64::from(positive), float_operand(e, extent)))
    } else if c < 0.0 {
        rounded(x87_mul(
            f64::from(positive.wrapping_neg()),
            float_operand(e, extent),
        ))
    } else {
        0.0
    }
}

/// The contact point of the face and edge cases: box `bx`'s centre moved by
/// `velocity` times the contact time, plus its three axes times `k`, added
/// in that order (`(((ot v + centre) + k0 A0) + k1 A1) + k2 A2`) and
/// stored component by component into `point`. The x of the first axis
/// and the z of the axes `z_memory` names are memory operands, the other
/// axis components loaded; `velocity_x_loaded` is the code that loads the
/// velocity's x and takes the contact time as the operand (the faces of
/// `a`), the others load the time.
fn store_contact_point(
    e: &mut Engine,
    s: &BoxFind,
    bx: u32,
    velocity: u32,
    k: [f64; 3],
    velocity_x_loaded: bool,
    z_memory: [bool; 3],
) {
    for component in 0..3u32 {
        let moved = if component == 0 && velocity_x_loaded {
            x87_mul(load_float(e, velocity), float_operand(e, s.out_time))
        } else {
            x87_mul(
                load_float(e, s.out_time),
                float_operand(e, velocity + 4 * component),
            )
        };
        let mut sum = x87_add(moved, float_operand(e, bx + CENTER + 4 * component));
        for (axis, &offset) in k.iter().enumerate() {
            let field = bx + AXES + 0xc * axis as u32 + 4 * component;
            let memory = match component {
                0 => axis == 0,
                1 => false,
                _ => z_memory[axis],
            };
            let value = if memory {
                float_operand(e, field)
            } else {
                load_float(e, field)
            };
            sum = x87_add(sum, x87_mul(offset, value));
        }
        e.mem.set_u32(s.point + 4 * component, x87_store32(sum));
    }
}

/// `FLD; FCHS; FSTP` of the three floats at `from` into `to` (all three
/// read before the first is written).
fn store_negated_point(e: &mut Engine, to: u32, from: u32) {
    let words = [0u32, 4, 8].map(|k| negated_bits(e.mem.u32(from + k)));
    for (k, word) in words.iter().enumerate() {
        e.mem.set_u32(to + 4 * k as u32, *word);
    }
}

/// The time argument of [`fn_00ea7730`] as [`fn_00ea7d40`] passes it
/// (`FLD; FSTP` onto the stack: a signalling NaN quieted).
fn passed_time(s: &BoxFind) -> f32 {
    f32::from_bits(x87_quiet_bits(s.time))
}

/// The end of [`fn_00ea7d40`] for the overlap and edge cases (`00ea96d5`):
/// without `want_normals` nothing. Otherwise, when the normal's squared
/// length (`004a7290`, rounded) differs from 1.0 by more than `FLT_EPSILON`
/// (`|len2 - 1|` rounded; ordered compares), the normal is unitized
/// (`004a0c10`) when the squared length is above `FLT_EPSILON`, else
/// replaced by the default x axis (`011a946c`, three words). Then the
/// negated normal is stored. Answers true.
fn finish_normal(e: &mut Engine, s: &BoxFind) -> bool {
    if !s.want_normals {
        return true;
    }
    let length = rounded(
        e.call(POINT_LENGTH_SQUARED_004A7290, &args![s.normal])
            .f64(),
    );
    let one = x87_load64(e.mem.u64(ONE_01012070));
    let off = rounded(rounded(x87_sub(length, one)).abs());
    let epsilon = load_float(e, FLT_EPSILON);
    if epsilon < off {
        if epsilon < length {
            e.call(UNITIZE_004A0C10, &args![s.normal]);
        } else {
            copy_point(e, s.normal, DEFAULT_AXIS_X);
        }
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// Case 0 of [`fn_00ea7d40`] (no axis was taken: the boxes overlap at the
/// start; the contact time stays 0): the contact point is half the sum of
/// the centres (`00439e90` into `[ESP+0xe4]`, then `004a3760` with the
/// float at `01016248` into `[ESP+0xa8]`, three words copied), the normal
/// the point minus `a`'s centre (`00439ef0` into the copy of `V`), finished
/// by [`finish_normal`].
fn find_overlap(e: &mut Engine, s: &BoxFind) -> bool {
    let f = s.frame;
    let sum = e
        .call(
            POINT_SUM_00439E90,
            &args![s.a + CENTER, f + FIND_CENTRE_SUM, s.b + CENTER],
        )
        .u32();
    let half = x87_quiet_bits(e.mem.u32(HALF_01016248));
    let point = e
        .call(POINT_SCALE_004A3760, &args![f + FIND_HALF_SUM, half, sum])
        .u32();
    copy_point(e, s.point, point);
    if !s.want_normals {
        return true;
    }
    let normal = e
        .call(
            POINT_DIFFERENCE_00439EF0,
            &args![s.point, f + FIND_V_COPY, s.a + CENTER],
        )
        .u32();
    copy_point(e, s.normal, normal);
    finish_normal(e, s)
}

/// Cases 1 to 3 of [`fn_00ea7d40`] (`a`'s axis `i` separated longest): the
/// contact point is on `b`: its offsets along its axes `j` are
/// [`signed_extent`] of `c[i][j]` with the negated side, `b` moved to the
/// contact time ([`store_contact_point`]), then [`fn_00ea7730`] (`b`
/// first). The normal is `a`'s axis `i` (the three words, or negated when
/// the side is negative), the other output its negation; the normal is not
/// unitized. Answers true.
fn find_face_of_a(e: &mut Engine, s: &BoxFind, i: usize) -> bool {
    let k = [0usize, 1, 2].map(|j| {
        signed_extent(
            e,
            s.c[i][j],
            s.side.wrapping_neg(),
            s.b + EXTENTS + 4 * j as u32,
        )
    });
    store_contact_point(e, s, s.b, s.vb, k, true, [false, false, true]);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.b),
        p(s.vb),
        p(s.a),
        p(s.va),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    let axis = s.a + AXES + 0xc * i as u32;
    if s.side < 0 {
        store_negated_point(e, s.normal, axis);
    } else {
        copy_point(e, s.normal, axis);
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// Cases 4 to 6 of [`fn_00ea7d40`] (`b`'s axis `j`): as
/// [`find_face_of_a`] with the boxes' roles exchanged: the point on `a`
/// (offsets [`signed_extent`] of `c[i][j]` with the side itself), the
/// normal `b`'s axis `j`.
fn find_face_of_b(e: &mut Engine, s: &BoxFind, j: usize) -> bool {
    let k =
        [0usize, 1, 2].map(|i| signed_extent(e, s.c[i][j], s.side, s.a + EXTENTS + 4 * i as u32));
    store_contact_point(e, s, s.a, s.va, k, false, [false, false, true]);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.a),
        p(s.va),
        p(s.b),
        p(s.vb),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    let axis = s.b + AXES + 0xc * j as u32;
    if s.side < 0 {
        store_negated_point(e, s.normal, axis);
    } else {
        copy_point(e, s.normal, axis);
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// One edge case of [`fn_00ea7d40`] (`a`'s axis `i` times `b`'s axis `j`):
/// `a_terms` and `b_terms` are the two other axes of each box, each with
/// the `c` entry ([row, column]) whose sign gives its offset and whether a
/// positive entry takes the negated side, in the code's order; `scale` the
/// local `004a3760` writes `V` times the contact time into, `negative` and
/// `positive` the locals `004b3800` writes the normal into for each side,
/// `z_memory` the contact point's operand forms
/// ([`store_contact_point`]).
struct EdgeCase {
    i: usize,
    j: usize,
    a_terms: [(usize, [usize; 2], bool); 2],
    b_terms: [(usize, [usize; 2], bool); 2],
    scale: u32,
    negative: u32,
    positive: u32,
    z_memory: [bool; 3],
}

/// Cases 7 to 15 of [`fn_00ea7d40`], in order (`a`'s axis 0 with `b`'s
/// axes 0, 1, 2, then `a`'s axes 1 and 2).
const EDGE_CASES: [EdgeCase; 9] = [
    EdgeCase {
        i: 0,
        j: 0,
        a_terms: [(1, [2, 0], true), (2, [1, 0], false)],
        b_terms: [(1, [0, 2], true), (2, [0, 1], false)],
        scale: 0x1d4,
        negative: 0x9c,
        positive: 0x54,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 0,
        j: 1,
        a_terms: [(1, [2, 1], true), (2, [1, 1], false)],
        b_terms: [(0, [0, 2], false), (2, [0, 0], true)],
        scale: 0xf0,
        negative: 0x44,
        positive: 0x1c,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 0,
        j: 2,
        a_terms: [(1, [2, 2], true), (2, [1, 2], false)],
        b_terms: [(0, [0, 1], true), (1, [0, 0], false)],
        scale: 0x1ec,
        negative: 0x28,
        positive: 0x108,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 1,
        j: 0,
        a_terms: [(0, [2, 0], false), (2, [0, 0], true)],
        b_terms: [(1, [1, 2], true), (2, [1, 1], false)],
        scale: 0x180,
        negative: 0x120,
        positive: 0x1e0,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 1,
        j: 1,
        a_terms: [(0, [2, 1], false), (2, [0, 1], true)],
        b_terms: [(0, [1, 2], false), (2, [1, 0], true)],
        scale: 0x138,
        negative: 0x198,
        positive: 0x150,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 1,
        j: 2,
        a_terms: [(0, [2, 2], false), (2, [0, 2], true)],
        b_terms: [(0, [1, 1], true), (1, [1, 0], false)],
        scale: 0x1c8,
        negative: 0x168,
        positive: 0x1b0,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 2,
        j: 0,
        a_terms: [(0, [1, 0], true), (1, [0, 0], false)],
        b_terms: [(1, [2, 2], true), (2, [2, 1], false)],
        scale: 0xfc,
        negative: 0x114,
        positive: 0x12c,
        z_memory: [false, false, false],
    },
    EdgeCase {
        i: 2,
        j: 1,
        a_terms: [(0, [1, 1], true), (1, [0, 1], false)],
        b_terms: [(0, [2, 2], false), (2, [2, 0], true)],
        scale: 0x144,
        negative: 0x15c,
        positive: 0x174,
        z_memory: [false, false, false],
    },
    EdgeCase {
        i: 2,
        j: 2,
        a_terms: [(0, [1, 2], true), (1, [0, 2], false)],
        b_terms: [(0, [2, 1], true), (1, [2, 0], false)],
        scale: 0x18c,
        negative: 0x1a4,
        positive: 0x1bc,
        z_memory: [false, false, false],
    },
];

/// An edge case of [`fn_00ea7d40`] (see [`EdgeCase`]): the offsets of the
/// four other axes are [`signed_extent`]s; `a`'s own offset along axis `i`
/// solves for the closest point of the two edges when `|1 - c[i][j]^2|`
/// (rounded) is above the float at `010959f4`: with `D = T + V * time`
/// (`004a3760` on the copy of `V` with the contact time, each sum rounded,
/// stored at `[ESP+0x54]`), `q` the sum of `a`'s other offsets times their
/// `c[.][j]` (a `double`), `r1 = q - Bj . D` (rounded) and the
/// `NiPoint3::Dot`s with `D`, it is
/// `((Ai . D + r1 c[i][j]) + kb c[i][.] + kb' c[i][.]) / (1 - c[i][j]^2)`;
/// otherwise 0. The contact point is on `a` moved to the contact time, then
/// [`fn_00ea7730`]. The normal (with `want_normals`) is `Ai x Bj`
/// (`004b3800`), negated for a negative side, finished by
/// [`finish_normal`].
fn find_edge(e: &mut Engine, s: &BoxFind, edge: &EdgeCase) -> bool {
    let (i, j, c, f) = (edge.i, edge.j, s.c, s.frame);
    let a_axis = |n: usize| s.a + AXES + 0xc * n as u32;
    let b_axis = |n: usize| s.b + AXES + 0xc * n as u32;
    let mut ka = [0.0f64; 3];
    let mut kb = [0.0f64; 3];
    for (n, [row, column], negated) in edge.a_terms {
        let positive = if negated {
            s.side.wrapping_neg()
        } else {
            s.side
        };
        ka[n] = signed_extent(e, c[row][column], positive, s.a + EXTENTS + 4 * n as u32);
    }
    for (n, [row, column], negated) in edge.b_terms {
        let positive = if negated {
            s.side.wrapping_neg()
        } else {
            s.side
        };
        kb[n] = signed_extent(e, c[row][column], positive, s.b + EXTENTS + 4 * n as u32);
    }
    let denominator = rounded(x87_sub(1.0, x87_mul(c[i][j], c[i][j])));
    // `FCOMP float [010959f4]; TEST AH,0x41; JNZ`: solved only when above.
    ka[i] = if rounded(denominator.abs()) > float_operand(e, PARALLEL_LIMIT_010959F4) {
        let contact = x87_quiet_bits(e.mem.u32(s.out_time));
        let moved = e
            .call(
                POINT_SCALE_004A3760,
                &args![f + edge.scale, contact, f + FIND_V_COPY],
            )
            .u32();
        let d = [
            rounded(x87_add(s.t[0], float_operand(e, moved))),
            rounded(x87_add(load_float(e, moved + 4), s.t[1])),
            rounded(x87_add(load_float(e, moved + 8), s.t[2])),
        ];
        for (k, x) in d.iter().enumerate() {
            e.mem
                .set_u32(f + FIND_D_POINT + 4 * k as u32, x87_store32(*x));
        }
        let [(p0, _, _), (p1, _, _)] = edge.a_terms;
        let q = x87_add(x87_mul(ka[p0], c[p0][j]), x87_mul(ka[p1], c[p1][j]));
        let dot_b = e.call(POINT_DOT, &args![b_axis(j), f + FIND_D_POINT]).f64();
        let r1 = rounded(x87_sub(q, dot_b));
        let r1c = x87_mul(r1, c[i][j]);
        let dot_a = e.call(POINT_DOT, &args![a_axis(i), f + FIND_D_POINT]).f64();
        let [(s0, _, _), (s1, _, _)] = edge.b_terms;
        let sum = x87_add(
            x87_add(x87_add(dot_a, r1c), x87_mul(kb[s0], c[i][s0])),
            x87_mul(kb[s1], c[i][s1]),
        );
        rounded(x87_div(sum, denominator))
    } else {
        0.0
    };
    store_contact_point(e, s, s.a, s.va, ka, false, edge.z_memory);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.a),
        p(s.va),
        p(s.b),
        p(s.vb),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    if s.side < 0 {
        let normal = e
            .call(
                POINT_CROSS_004B3800,
                &args![a_axis(i), f + edge.negative, b_axis(j)],
            )
            .u32();
        store_negated_point(e, s.normal, normal);
    } else {
        let normal = e
            .call(
                POINT_CROSS_004B3800,
                &args![a_axis(i), f + edge.positive, b_axis(j)],
            )
            .u32();
        copy_point(e, s.normal, normal);
    }
    finish_normal(e, s)
}

// Translated from 00ea7d40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (a function Ghidra never created; the box row's box column of
/// the find table set up at `00e99130`): the moving box-box `Find` (cdecl,
/// ten words as [`fn_00ea3950`]: the time, box `a` and its velocity, box
/// `b` and its velocity, `out_time`, `out_point`, `want_normals` (a byte),
/// `out_normal`, `out_normal_negated`). With `V` the relative velocity
/// (`b`'s minus `a`'s), `T` the centre offset (`b - a`), `T1 = T + V time`
/// and `c[i][j] = Ai . Bj` (every value rounded where the code stores it),
/// the fifteen separating axes are tried in turn ([`find_axis`]): `a`'s
/// three axes, `b`'s three, then the nine products `Ai x Bj` (`a`'s axis
/// major); the first that keeps the boxes apart for the whole motion
/// answers false. Otherwise the axis whose meeting time is the latest (by
/// more than the slack) is the contact axis, its time `*out_time` (0 when
/// none was taken), and a `switch` on it (a jump table after the body)
/// stores the contact point and, with `want_normals`, the normal and its
/// negation: [`find_overlap`] (0), [`find_face_of_a`] (1 to 3),
/// [`find_face_of_b`] (4 to 6), [`find_edge`] (7 to 15). Answers true.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea7d40(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    e.with_stack(FIND_FRAME, |e, frame| {
        let f = frame.addr();
        let (a, va, b, vb) = (a.addr(), a_velocity.addr(), b.addr(), b_velocity.addr());
        let ot = out_time.addr();
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let op = |e: &Engine, x: u32| float_operand(e, x);
        let time_l = x87_load32(time.to_bits());
        let time_m = x87_m32(time.to_bits());
        let v = [
            rounded(x87_sub(ld(e, vb), op(e, va))),
            rounded(x87_sub(ld(e, vb + 4), op(e, va + 4))),
            rounded(x87_sub(ld(e, vb + 8), op(e, va + 8))),
        ];
        for (k, x) in v.iter().enumerate() {
            e.mem
                .set_u32(f + FIND_V_COPY + 4 * k as u32, x87_store32(*x));
        }
        let t = [
            rounded(x87_sub(ld(e, b + 4), op(e, a + 4))),
            rounded(x87_sub(ld(e, b + 8), op(e, a + 8))),
            rounded(x87_sub(ld(e, b + 0xc), op(e, a + 0xc))),
        ];
        let m = [
            rounded(x87_mul(v[0], time_l)),
            rounded(x87_mul(v[1], time_l)),
            rounded(x87_mul(time_l, v[2])),
        ];
        let t1 = [
            rounded(x87_add(t[0], m[0])),
            rounded(x87_add(m[1], t[1])),
            rounded(x87_add(m[2], t[2])),
        ];
        // `FLDZ; FSTP [EDI]`.
        e.mem.set_u32(ot, 0);
        let mut s = BoxFind {
            time: time.to_bits(),
            a,
            va,
            b,
            vb,
            out_time: ot,
            point: out_point.addr(),
            want_normals,
            normal: out_normal.addr(),
            negated: out_normal_negated.addr(),
            frame: f,
            c: [[0.0; 3]; 3],
            t,
            axis: 0,
            side: 0,
        };
        let mut c = [[0.0f64; 3]; 3];
        let mut ac = [[0.0f64; 3]; 3];
        let mut da = [0.0f64; 3];
        let mut d1a = [0.0f64; 3];
        // `a`'s first axis.
        c[0] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x14), op(e, a + 0x14)),
                    x87_mul(ld(e, b + 0x10), op(e, a + 0x10)),
                ),
                x87_mul(ld(e, b + 0x18), op(e, a + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x14)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x10)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x14)),
                    x87_mul(ld(e, a + 0x10), op(e, b + 0x28)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x18)),
            )),
        ];
        da[0] = rounded(x87_add(
            x87_add(
                x87_mul(t[0], op(e, a + 0x10)),
                x87_mul(t[1], op(e, a + 0x14)),
            ),
            x87_mul(t[2], op(e, a + 0x18)),
        ));
        d1a[0] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x10), t1[0]),
                x87_mul(t1[1], op(e, a + 0x14)),
            ),
            x87_mul(t1[2], op(e, a + 0x18)),
        ));
        // `FABS` of a stored float, stored and reloaded twice: exact.
        ac[0] = c[0].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[0][1]),
                x87_mul(ld(e, b + 0x34), ac[0][0]),
            ),
            x87_mul(ac[0][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x34)));
        let d1 = d1a[0];
        if find_axis(e, &mut s, 1, r, da[0], || d1, time_l, time_l) {
            return false;
        }
        // `a`'s second axis.
        c[1] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, a + 0x20), op(e, b + 0x14)),
                    x87_mul(ld(e, b + 0x10), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, a + 0x24), op(e, b + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x20)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x24)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x20)),
                    x87_mul(ld(e, b + 0x28), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x24)),
            )),
        ];
        da[1] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x20), t[1]),
                x87_mul(t[0], op(e, a + 0x1c)),
            ),
            x87_mul(ld(e, a + 0x24), t[2]),
        ));
        d1a[1] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x20), t1[1]),
                x87_mul(ld(e, a + 0x1c), t1[0]),
            ),
            x87_mul(ld(e, a + 0x24), t1[2]),
        ));
        ac[1] = c[1].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[1][1]),
                x87_mul(ld(e, b + 0x34), ac[1][0]),
            ),
            x87_mul(ac[1][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x38)));
        let d1 = d1a[1];
        if find_axis(e, &mut s, 2, r, da[1], || d1, time_l, time_l) {
            return false;
        }
        // `a`'s third axis.
        c[2] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, a + 0x2c), op(e, b + 0x14)),
                    x87_mul(ld(e, a + 0x28), op(e, b + 0x10)),
                ),
                x87_mul(ld(e, a + 0x30), op(e, b + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x2c)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x28)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x30)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x2c)),
                    x87_mul(ld(e, b + 0x28), op(e, a + 0x28)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x30)),
            )),
        ];
        da[2] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x2c), t[1]),
                x87_mul(t[0], op(e, a + 0x28)),
            ),
            x87_mul(ld(e, a + 0x30), t[2]),
        ));
        d1a[2] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x2c), t1[1]),
                x87_mul(ld(e, a + 0x28), t1[0]),
            ),
            x87_mul(ld(e, a + 0x30), t1[2]),
        ));
        ac[2] = c[2].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[2][1]),
                x87_mul(ld(e, b + 0x34), ac[2][0]),
            ),
            x87_mul(ac[2][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x3c)));
        let d1 = d1a[2];
        if find_axis(e, &mut s, 3, r, da[2], || d1, time_l, time_l) {
            return false;
        }
        s.c = c;
        // `b`'s axes (the meeting time of the first two takes the time as a
        // memory operand).
        let db0 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x10), t[0]),
                x87_mul(t[1], op(e, b + 0x14)),
            ),
            x87_mul(t[2], op(e, b + 0x18)),
        ));
        let d1b0 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x10), t1[0]),
                x87_mul(t1[1], op(e, b + 0x14)),
            ),
            x87_mul(t1[2], op(e, b + 0x18)),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][0]),
                x87_mul(ac[0][0], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][0]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x34)));
        if find_axis(e, &mut s, 4, r, db0, || d1b0, time_m, time_l) {
            return false;
        }
        let db1 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x20), t[1]),
                x87_mul(t[0], op(e, b + 0x1c)),
            ),
            x87_mul(ld(e, b + 0x24), t[2]),
        ));
        let d1b1 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x20), t1[1]),
                x87_mul(ld(e, b + 0x1c), t1[0]),
            ),
            x87_mul(ld(e, b + 0x24), t1[2]),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][1]),
                x87_mul(ac[0][1], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][1]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x38)));
        if find_axis(e, &mut s, 5, r, db1, || d1b1, time_m, time_l) {
            return false;
        }
        let db2 = rounded(x87_add(
            x87_mul(t[2], ld(e, b + 0x30)),
            x87_add(
                x87_mul(t[0], ld(e, b + 0x28)),
                x87_mul(t[1], ld(e, b + 0x2c)),
            ),
        ));
        let d1b2 = rounded(x87_add(
            x87_add(
                x87_mul(t1[1], ld(e, b + 0x2c)),
                x87_mul(t1[0], op(e, b + 0x28)),
            ),
            x87_mul(ld(e, b + 0x30), t1[2]),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][2]),
                x87_mul(ac[0][2], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][2]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x3c)));
        if find_axis(e, &mut s, 6, r, db2, || d1b2, time_l, time_l) {
            return false;
        }
        // The nine products `Ai x Bj`: `d0` and `d1` from `a`'s projections,
        // the radii as two rounded sums (each pair in the code's operand
        // forms).
        let cross = |p: f64, x: f64, q: f64, y: f64| rounded(x87_sub(x87_mul(p, x), x87_mul(q, y)));
        let (eb, ea) = (|k: u32| b + EXTENTS + 4 * k, |k: u32| a + EXTENTS + 4 * k);
        let pair = |x: f64, y: f64| rounded(x87_add(x, y));
        // A0 x B0.
        let d0 = cross(da[2], c[1][0], c[2][0], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[0][2]),
                x87_mul(ac[0][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[2][0]),
                x87_mul(ld(e, ea(2)), ac[1][0]),
            ),
        );
        let d1 = || cross(c[1][0], d1a[2], c[2][0], d1a[1]);
        if find_axis(e, &mut s, 7, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A0 x B1.
        let d0 = cross(da[2], c[1][1], c[2][1], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(1)), ac[2][1]),
                x87_mul(ld(e, ea(2)), ac[1][1]),
            ),
            pair(
                x87_mul(ac[0][2], ld(e, eb(0))),
                x87_mul(ac[0][0], op(e, eb(2))),
            ),
        );
        let d1 = || cross(c[1][1], d1a[2], c[2][1], d1a[1]);
        if find_axis(e, &mut s, 8, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A0 x B2.
        let d0 = cross(da[2], c[1][2], c[2][2], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(1)), ac[2][2]),
                x87_mul(ld(e, ea(2)), ac[1][2]),
            ),
            pair(
                x87_mul(ld(e, eb(1)), ac[0][0]),
                x87_mul(ld(e, eb(0)), ac[0][1]),
            ),
        );
        let d1 = || cross(c[1][2], d1a[2], c[2][2], d1a[1]);
        if find_axis(e, &mut s, 9, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B0.
        let d0 = cross(c[2][0], da[0], da[2], c[0][0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[1][2]),
                x87_mul(ac[1][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(2)), ac[0][0]),
                x87_mul(ac[2][0], op(e, ea(0))),
            ),
        );
        let d1 = || cross(c[2][0], d1a[0], c[0][0], d1a[2]);
        if find_axis(e, &mut s, 10, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B1.
        let d0 = cross(c[2][1], da[0], da[2], c[0][1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(2)), ac[0][1]),
                x87_mul(ac[2][1], op(e, ea(0))),
            ),
            pair(
                x87_mul(ld(e, eb(0)), ac[1][2]),
                x87_mul(ac[1][0], op(e, eb(2))),
            ),
        );
        let d1 = || cross(c[2][1], d1a[0], c[0][1], d1a[2]);
        if find_axis(e, &mut s, 11, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B2.
        let d0 = cross(c[2][2], da[0], da[2], c[0][2]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(2)), ac[0][2]),
                x87_mul(ac[2][2], op(e, ea(0))),
            ),
            pair(
                x87_mul(ld(e, eb(1)), ac[1][0]),
                x87_mul(ld(e, eb(0)), ac[1][1]),
            ),
        );
        let d1 = || cross(c[2][2], d1a[0], c[0][2], d1a[2]);
        if find_axis(e, &mut s, 12, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B0.
        let d0 = cross(da[1], c[0][0], c[1][0], da[0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[2][2]),
                x87_mul(ac[2][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][0]),
                x87_mul(ac[1][0], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][0], c[1][0], d1a[0]);
        if find_axis(e, &mut s, 13, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B1.
        let d0 = cross(da[1], c[0][1], c[1][1], da[0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(0)), ac[2][2]),
                x87_mul(ac[2][0], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][1]),
                x87_mul(ac[1][1], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][1], c[1][1], d1a[0]);
        if find_axis(e, &mut s, 14, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B2.
        let d0 = cross(da[1], c[0][2], da[0], c[1][2]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[2][0]),
                x87_mul(ld(e, eb(0)), ac[2][1]),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][2]),
                x87_mul(ac[1][2], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][2], d1a[0], c[1][2]);
        if find_axis(e, &mut s, 15, r, d0, d1, time_l, time_l) {
            return false;
        }
        // The jump table at `00eab768` (the compiler's bound check `JBE` on
        // 15 lets a larger value fall through to the normal's end, which
        // cannot happen: the axis is one of the fifteen or 0).
        match s.axis {
            0 => find_overlap(e, &s),
            1..=3 => find_face_of_a(e, &s, s.axis as usize - 1),
            4..=6 => find_face_of_b(e, &s, s.axis as usize - 4),
            7..=15 => find_edge(e, &s, &EDGE_CASES[s.axis as usize - 7]),
            _ => finish_normal(e, &s),
        }
    })
}

/// The integer flags the lifted [`fn_00ea3d60`] reads (`ZF`, `SF`, `CF`, `OF`, `PF`), set by
/// each instruction that sets them as the CPU does, on 8- or 32-bit operands.
#[derive(Default)]
struct LiftFlags {
    zf: bool,
    sf: bool,
    cf: bool,
    of: bool,
    pf: bool,
}

impl LiftFlags {
    fn mask(bits: u32) -> u32 {
        if bits == 8 {
            0xff
        } else {
            u32::MAX
        }
    }

    /// `ZF`, `SF` and `PF` of a result (`PF` from its low byte).
    fn result(&mut self, r: u32, bits: u32) -> u32 {
        let r = r & Self::mask(bits);
        self.zf = r == 0;
        self.sf = r >> (bits - 1) & 1 != 0;
        self.pf = (r & 0xff).count_ones() & 1 == 0;
        r
    }

    /// `ADD`.
    fn add(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let (a, b) = (a & m, b & m);
        let r = self.result(a.wrapping_add(b), bits);
        self.cf = u64::from(a) + u64::from(b) > u64::from(m);
        let sign = 1 << (bits - 1);
        self.of = (a & sign) == (b & sign) && (r & sign) != (a & sign);
        r
    }

    /// `SUB` and `CMP`.
    fn sub(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let (a, b) = (a & m, b & m);
        let r = self.result(a.wrapping_sub(b), bits);
        self.cf = a < b;
        let sign = 1 << (bits - 1);
        self.of = (a & sign) != (b & sign) && (r & sign) != (a & sign);
        r
    }

    /// `AND` and `TEST`.
    fn and(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.cf = false;
        self.of = false;
        self.result(a & b, bits)
    }

    /// `OR`.
    fn or(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.cf = false;
        self.of = false;
        self.result(a | b, bits)
    }

    /// `XOR`.
    fn xor(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.cf = false;
        self.of = false;
        self.result(a ^ b, bits)
    }

    /// `INC` (32-bit; `CF` kept).
    fn inc(&mut self, a: u32) -> u32 {
        let r = self.result(a.wrapping_add(1), 32);
        self.of = a == 0x7fff_ffff;
        r
    }

    /// `NEG` (32-bit).
    fn neg(&mut self, a: u32) -> u32 {
        let r = self.result(0u32.wrapping_sub(a), 32);
        self.cf = a != 0;
        self.of = a == 0x8000_0000;
        r
    }
}

/// The x87 register stack of the lifted [`fn_00ea3d60`]: eight registers, `ST0` the top, each
/// value as the `f64` the exe's double precision control keeps.
struct LiftStack {
    regs: [f64; 8],
    depth: usize,
}

impl LiftStack {
    fn st(&self, i: usize) -> f64 {
        self.regs[self.depth - 1 - i]
    }

    fn set(&mut self, i: usize, value: f64) {
        self.regs[self.depth - 1 - i] = value;
    }

    fn push(&mut self, value: f64) {
        assert!(self.depth < 8, "x87 stack overflow");
        self.regs[self.depth] = value;
        self.depth += 1;
    }

    fn pop(&mut self) {
        assert!(self.depth > 0, "x87 stack underflow");
        self.depth -= 1;
    }

    /// `FXCH ST(i)`.
    fn xch(&mut self, i: usize) {
        let top = self.st(0);
        let other = self.st(i);
        self.set(0, other);
        self.set(i, top);
    }
}

/// The `AH` an `FCOM` of `ST0 = a` with `b` leaves for `FNSTSW AX`: `C0` 0x01, `C2` 0x04, `C3`
/// 0x40 (all three when unordered).
fn lift_fcom(a: f64, b: f64) -> u8 {
    if a.is_nan() || b.is_nan() {
        0x45
    } else if a < b {
        0x01
    } else if a == b {
        0x40
    } else {
        0
    }
}

/// The emulated stack bytes of [`fn_00ea3d60`]: its frame (`SUB ESP, 0x1cc`, four saved
/// registers, the callees' argument words) below the entry `ESP`, the return slot and twelve
/// argument words above it.
const LIFT_STACK_BYTES: u32 = 0x300;
const LIFT_ENTRY_ESP: u32 = 0x280;

// Translated from 00ea3d60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (a function Ghidra never created, 00ea3d60 to the `RET` at 00ea7452 with its jump
/// table at 00ea7454; the box row's triangle column of the find table set up at `00e99130`):
/// the moving box-triangle `Find`, `cdecl` with twelve words as [`fn_00ea7d40`]'s shape: the
/// time, the box, its velocity, the triangle's three vertices, the triangle's velocity,
/// `out_time`, `out_point`, `want_normals` (a byte), `out_normal`, `out_normal_negated`.
/// Answers whether the two meet within the time (`AL`).
///
/// Lifted instruction by instruction from the exe (4380 instructions in 423 blocks; each
/// statement carries its address): the integer registers, the flags the jumps read, the x87
/// register stack (each operation through `crate::x87`: `FLD dword` quiets a signalling NaN,
/// arithmetic memory operands do not, stores round to `float`) and the frame as words of the
/// emulated stack, with the exe's own compare-and-jump sequences (`FNSTSW AX; TEST AH, n;
/// JP/JNP/JNZ/JZ`). Its callees: `00439e90` and `004b3800` (`thiscall`, two words, `RET 8`),
/// `004a3760` (`cdecl`, three words), `0063c8a0` and `NiPoint3::Dot` `004b6190` (`thiscall`, one
/// word, `RET 4`; the dot product returned in `ST0`) and `004a0c10` (`thiscall`, none). The
/// x87 stack is empty at every call (checked over every path).
#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop,
    clippy::needless_continue,
    clippy::too_many_lines,
    clippy::identity_op,
    clippy::erasing_op
)]
pub fn fn_00ea3d60(
    e: &mut Engine,
    time: f32,
    bx: Ptr,
    box_velocity: Ptr,
    v0: Ptr,
    v1: Ptr,
    v2: Ptr,
    triangle_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: u32,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    e.with_stack(LIFT_STACK_BYTES, |e, frame| {
        let mut esp = frame.addr().wrapping_add(LIFT_ENTRY_ESP);
        let words = [
            0,
            time.to_bits(),
            bx.addr(),
            box_velocity.addr(),
            v0.addr(),
            v1.addr(),
            v2.addr(),
            triangle_velocity.addr(),
            out_time.addr(),
            out_point.addr(),
            want_normals,
            out_normal.addr(),
            out_normal_negated.addr(),
        ];
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(esp.wrapping_add(4 * i as u32), *word);
        }
        let (mut eax, mut ebx, mut ecx, mut edx) = (0u32, 0u32, 0u32, 0u32);
        let (mut esi, mut edi, mut ebp) = (0u32, 0u32, 0u32);
        let mut fl = LiftFlags::default();
        let mut x = LiftStack {
            regs: [0.0; 8],
            depth: 0,
        };
        let (mut sw, mut ah) = (0u8, 0u8);
        let fcom = lift_fcom;
        let mut pc = 0x00ea_3d60u32;
        loop {
            match pc {
                0x00ea_3d60 => {
                    let r = fl.sub(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea3d60
                    eax = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea3d66
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea3d6d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00ea3d6f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00ea3d70
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00ea3d71
                    esi = e.mem.u32(esp.wrapping_add(0x000001e4)); // 00ea3d72
                    x.set(0, x87_sub(x.st(0), x87_m32(e.mem.u32(esi)))); // 00ea3d79
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e0)); // 00ea3d7b
                    edx = ecx.wrapping_add(0x00000034); // 00ea3d82
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00ea3d85
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000080), x87_store32(x.st(0)));
                    x.pop(); // 00ea3d86
                    ebx = ecx.wrapping_add(0x00000010); // 00ea3d8d
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000004)))); // 00ea3d90
                    e.mem.set_u32(esp.wrapping_add(0x0000001c), edx); // 00ea3d93
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esi.wrapping_add(0x00000004)))),
                    ); // 00ea3d97
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000084), x87_store32(x.st(0)));
                    x.pop(); // 00ea3d9a
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000008)))); // 00ea3da1
                    eax = e.mem.u32(esp.wrapping_add(0x000001ec)); // 00ea3da4
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esi.wrapping_add(0x00000008)))),
                    ); // 00ea3dab
                    esi = e.mem.u32(esp.wrapping_add(0x000001f0)); // 00ea3dae
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000088), x87_store32(x.st(0)));
                    x.pop(); // 00ea3db5
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea3dbc
                    x.set(0, x87_sub(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea3dbe
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea3dc0
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000004)))); // 00ea3dc4
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000004)))),
                    ); // 00ea3dc7
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea3dca
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000008)))); // 00ea3dce
                    esi = e.mem.u32(esp.wrapping_add(0x00000020)); // 00ea3dd1
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000008)))),
                    ); // 00ea3dd5
                    e.mem.set_u32(esp.wrapping_add(0x000000c8), esi); // 00ea3dd8
                    esi = e.mem.u32(esp.wrapping_add(0x00000024)); // 00ea3ddf
                    e.mem.set_u32(esp.wrapping_add(0x000000cc), esi); // 00ea3de3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea3dea
                    esi = e.mem.u32(esp.wrapping_add(0x00000028)); // 00ea3dee
                    e.mem.set_u32(esp.wrapping_add(0x000000d0), esi); // 00ea3df2
                    esi = e.mem.u32(esp.wrapping_add(0x000001f4)); // 00ea3df9
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea3e00
                    x.set(0, x87_sub(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea3e02
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000050), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e04
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000004)))); // 00ea3e08
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000004)))),
                    ); // 00ea3e0b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000054), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e0e
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000008)))); // 00ea3e12
                    esi = e.mem.u32(esp.wrapping_add(0x00000050)); // 00ea3e15
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000008)))),
                    ); // 00ea3e19
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), esi); // 00ea3e1c
                    esi = e.mem.u32(esp.wrapping_add(0x00000054)); // 00ea3e23
                    e.mem.set_u32(esp.wrapping_add(0x000000d8), esi); // 00ea3e27
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000058), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e2e
                    esi = e.mem.u32(esp.wrapping_add(0x00000058)); // 00ea3e32
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000058)))); // 00ea3e36
                    e.mem.set_u32(esp.wrapping_add(0x000000dc), esi); // 00ea3e3a
                    x.push(x.st(0)); // 00ea3e41
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea3e43
                    x.push(x.st(0)); // 00ea3e47
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea3e49
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea3e4b
                    x.push(x.st(0)); // 00ea3e4f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea3e51
                    x.push(x.st(0)); // 00ea3e55
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea3e57
                    x.xch(4); // 00ea3e59
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea3e5b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000104), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e5d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea3e64
                    x.push(x.st(0)); // 00ea3e68
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea3e6a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea3e6c
                    x.push(x.st(0)); // 00ea3e70
                    x.set(6, x87_mul(x.st(6), x.st(0)));
                    x.pop(); // 00ea3e72
                    x.xch(4); // 00ea3e74
                    x.set(5, x87_sub(x.st(0), x.st(5)));
                    x.pop(); // 00ea3e76
                    x.xch(4); // 00ea3e78
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000108), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e7a
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea3e81
                    x.push(x.st(0)); // 00ea3e83
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea3e85
                    x.xch(1); // 00ea3e87
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea3e89
                    x.xch(1); // 00ea3e8b
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000010c), x87_store32(x.st(0)));
                    x.pop(); // 00ea3e8d
                    esi = e.mem.u32(esp.wrapping_add(0x00000104)); // 00ea3e94
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea3e9b
                    let r = fl.add(ecx, 0x0000_0004, 32);
                    ecx = r; // 00ea3e9d
                    x.set(0, x87_sub(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea3ea0
                    e.mem.set_u32(esp.wrapping_add(0x000000f8), esi); // 00ea3ea2
                    esi = e.mem.u32(esp.wrapping_add(0x00000108)); // 00ea3ea9
                    e.mem.set_u32(esp.wrapping_add(0x000000fc), esi); // 00ea3eb0
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000005c), x87_store32(x.st(0)));
                    x.pop(); // 00ea3eb7
                    esi = e.mem.u32(esp.wrapping_add(0x0000010c)); // 00ea3ebb
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000004)))); // 00ea3ec2
                    e.mem.set_u32(esp.wrapping_add(0x00000100), esi); // 00ea3ec5
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea3ecc
                    esi = e.mem.u32(esp.wrapping_add(0x000001fc)); // 00ea3ecf
                    e.mem.set_u32(esp.wrapping_add(0x00000090), ecx); // 00ea3ed6
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea3edd
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000060), x87_store32(x.st(0)));
                    x.pop(); // 00ea3edf
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000008)))); // 00ea3ee3
                    eax = e.mem.u32(esp.wrapping_add(0x0000005c)); // 00ea3ee6
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea3eea
                    ecx = e.mem.u32(esp.wrapping_add(0x00000060)); // 00ea3eed
                    e.mem.set_u32(esp.wrapping_add(0x000000ec), eax); // 00ea3ef1
                    e.mem.set_u32(esp.wrapping_add(0x000000f0), ecx); // 00ea3ef8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000064), x87_store32(x.st(0)));
                    x.pop(); // 00ea3eff
                    eax = e.mem.u32(esp.wrapping_add(0x00000064)); // 00ea3f03
                    x.push(0.0); // 00ea3f07
                    let r = fl.xor(ecx, ecx, 32);
                    ecx = r; // 00ea3f09
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea3f0b
                    e.mem.set_u32(esp.wrapping_add(0x000000f4), eax); // 00ea3f0d
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea3f14
                    let r = fl.xor(edi, edi, 32);
                    edi = r; // 00ea3f17
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000108)))); // 00ea3f19
                    x.push(x.st(0)); // 00ea3f20
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea3f22
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000104)))); // 00ea3f24
                    x.push(x.st(0)); // 00ea3f2b
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ebx)))); // 00ea3f2d
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea3f2f
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea3f31
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000010c)))); // 00ea3f34
                    x.push(x.st(0)); // 00ea3f3b
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea3f3d
                    x.xch(4); // 00ea3f3f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f41
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000068), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f43
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea3f47
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea3f4a
                    x.push(x.st(1)); // 00ea3f4c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x0000000c)))),
                    ); // 00ea3f4e
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f51
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea3f53
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea3f56
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f58
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000006c), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f5a
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea3f5e
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea3f61
                    x.push(x.st(1)); // 00ea3f63
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000018)))),
                    ); // 00ea3f65
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f68
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000020)))); // 00ea3f6a
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea3f6d
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f6f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000070), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f71
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea3f75
                    x.set(0, x.st(0).abs()); // 00ea3f79
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f7b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea3f7f
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea3f83
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea3f85
                    x.set(0, x.st(0).abs()); // 00ea3f89
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f8b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea3f8f
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea3f93
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3f96
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea3f98
                    x.set(0, x.st(0).abs()); // 00ea3f9c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea3f9e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea3fa2
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea3fa6
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3fa9
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea3fab
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000060)))); // 00ea3faf
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea3fb3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea3fb5
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea3fb9
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3fbb
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000064)))); // 00ea3fbd
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea3fc1
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea3fc3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea3fc5
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea3fc9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea3fcd
                    sw = fcom(x.st(0), x.st(1)); // 00ea3fd1
                    ah = sw; // 00ea3fd3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea3fd5
                    fl.and(u32::from(ah), 5, 8); // 00ea3fdc
                    if fl.pf {
                        pc = 0x00ea_4067;
                        continue;
                    } // 00ea3fdf
                    pc = 0x00ea_3fe5;
                }
                0x00ea_3fe5 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea3fe5
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea3fec
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea3fee
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea3ff5
                    x.xch(4); // 00ea3ff7
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea3ff9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea3ffb
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea4002
                    x.xch(2); // 00ea4004
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea4006
                    x.xch(3); // 00ea4008
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea400a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea400e
                    x.push(x.st(0)); // 00ea4012
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea4014
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea4016
                    sw = fcom(x.st(0), x.st(2));
                    x.pop(); // 00ea4018
                    ah = sw; // 00ea401a
                    fl.and(u32::from(ah), 65, 8); // 00ea401c
                    if !fl.zf {
                        pc = 0x00ea_4038;
                        continue;
                    } // 00ea401f
                    pc = 0x00ea_4021;
                }
                0x00ea_4021 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4021
                    pc = 0x00ea_4023;
                }
                0x00ea_4023 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4023
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4025
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4026
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4028
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4029
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea402b
                    x.set(0, x.st(0));
                    x.pop(); // 00ea402c
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea402e
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4030
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4031
                    return (eax & 0xff) != 0; // 00ea4037
                }
                0x00ea_4038 => {
                    x.xch(1); // 00ea4038
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea403a
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea403c
                    x.xch(1); // 00ea403e
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4040
                    x.push(0.0); // 00ea4044
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4046
                    sw = fcom(x.st(0), x.st(1)); // 00ea404a
                    ah = sw; // 00ea404c
                    x.set(1, x.st(0));
                    x.pop(); // 00ea404e
                    fl.and(u32::from(ah), 65, 8); // 00ea4050
                    if !fl.zf {
                        pc = 0x00ea_414d;
                        continue;
                    } // 00ea4053
                    pc = 0x00ea_4059;
                }
                0x00ea_4059 => {
                    ecx = 0x0000_0001; // 00ea4059
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea405e
                    edi = ecx; // 00ea4060
                    pc = 0x00ea_4102;
                    continue; // 00ea4062
                }
                0x00ea_4067 => {
                    x.set(6, x.st(0));
                    x.pop(); // 00ea4067
                    x.set(5, x.st(0));
                    x.pop(); // 00ea4069
                    x.push(x.st(4)); // 00ea406b
                    x.set(0, -x.st(0)); // 00ea406d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea406f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4073
                    sw = fcom(x.st(0), x.st(1)); // 00ea4077
                    ah = sw; // 00ea4079
                    fl.and(u32::from(ah), 65, 8); // 00ea407b
                    if !fl.zf {
                        pc = 0x00ea_4134;
                        continue;
                    } // 00ea407e
                    pc = 0x00ea_4084;
                }
                0x00ea_4084 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea4084
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea408b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea408d
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea4094
                    x.xch(3); // 00ea4096
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4098
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea409a
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea40a1
                    x.xch(1); // 00ea40a3
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea40a5
                    x.xch(2); // 00ea40a7
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea40a9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea40ad
                    x.push(x.st(0)); // 00ea40b1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea40b3
                    x.push(x.st(0)); // 00ea40ba
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea40bc
                    x.push(x.st(4)); // 00ea40be
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea40c0
                    x.xch(1); // 00ea40c2
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea40c4
                    ah = sw; // 00ea40c6
                    x.set(2, x.st(0));
                    x.pop(); // 00ea40c8
                    fl.and(u32::from(ah), 5, 8); // 00ea40ca
                    if !fl.pf {
                        pc = 0x00ea_4023;
                        continue;
                    } // 00ea40cd
                    pc = 0x00ea_40d3;
                }
                0x00ea_40d3 => {
                    x.xch(2); // 00ea40d3
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea40d5
                    x.xch(2); // 00ea40d7
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea40d9
                    x.set(0, -x.st(0)); // 00ea40db
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea40dd
                    x.push(0.0); // 00ea40e1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea40e3
                    sw = fcom(x.st(0), x.st(1)); // 00ea40e7
                    ah = sw; // 00ea40e9
                    x.set(1, x.st(0));
                    x.pop(); // 00ea40eb
                    fl.and(u32::from(ah), 65, 8); // 00ea40ed
                    if !fl.zf {
                        pc = 0x00ea_4130;
                        continue;
                    } // 00ea40f0
                    pc = 0x00ea_40f2;
                }
                0x00ea_40f2 => {
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea40f2
                    ecx = 0x0000_0001; // 00ea40f4
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea40f9
                    pc = 0x00ea_40fc;
                }
                0x00ea_40fc => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea40fc
                    x.xch(1); // 00ea4100
                    pc = 0x00ea_4102;
                }
                0x00ea_4102 => {
                    eax = esp.wrapping_add(0x00000098); // 00ea4102
                    e.mem.set_u32(esp.wrapping_add(0x00000048), eax); // 00ea4109
                    eax = ebx.wrapping_add(0x00000008); // 00ea410d
                    e.mem.set_u32(esp.wrapping_add(0x0000002c), eax); // 00ea4110
                    eax = esp.wrapping_add(0x00000074); // 00ea4114
                    let r = fl.sub(eax, edx, 32);
                    eax = r; // 00ea4118
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edx); // 00ea411a
                    edx = e.mem.u32(esp.wrapping_add(0x0000002c)); // 00ea411e
                    e.mem.set_u32(esp.wrapping_add(0x00000010), 0x0000_0002); // 00ea4122
                    e.mem.set_u32(esp.wrapping_add(0x00000030), eax); // 00ea412a
                    pc = 0x00ea_4155;
                    continue; // 00ea412e
                }
                0x00ea_4130 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4130
                    pc = 0x00ea_40fc;
                    continue; // 00ea4132
                }
                0x00ea_4134 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4134
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4136
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4138
                    x.set(1, x.st(0));
                    x.pop(); // 00ea413a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea413c
                    x.set(0, x.st(0));
                    x.pop(); // 00ea413e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea4140
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea4144
                    pc = 0x00ea_4102;
                    continue; // 00ea414b
                }
                0x00ea_414d => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea414d
                    pc = 0x00ea_4102;
                    continue; // 00ea414f
                }
                0x00ea_4151 => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000030)); // 00ea4151
                    pc = 0x00ea_4155;
                }
                0x00ea_4155 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000060)))); // 00ea4155
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000004)))),
                    ); // 00ea4159
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea415c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000008)))),
                    ); // 00ea4160
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4163
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea4165
                    edx = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4167
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea416b
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea416f
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4171
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea4175
                    e.mem
                        .set_u32(eax.wrapping_add(edx.wrapping_mul(1)), x87_store32(x.st(0))); // 00ea4179
                    edx = e.mem.u32(esp.wrapping_add(0x0000002c)); // 00ea417c
                    eax = e.mem.u32(esp.wrapping_add(0x00000048)); // 00ea4180
                    x.push(x.st(2)); // 00ea4184
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000004)))),
                    ); // 00ea4186
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea4189
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000008)))),
                    ); // 00ea418d
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4190
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea4192
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000028)))),
                    ); // 00ea4194
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4198
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea419a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea419e
                    e.mem
                        .set_u32(eax.wrapping_sub(0x00000004), x87_store32(x.st(0))); // 00ea41a2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea41a5
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000004)))),
                    ); // 00ea41a9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea41ac
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000008)))),
                    ); // 00ea41b0
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea41b3
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea41b5
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000058)))),
                    ); // 00ea41b7
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea41bb
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea41bd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea41c1
                    e.mem.set_u32(eax, x87_store32(x.st(0))); // 00ea41c5
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea41c7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea41cb
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000004)))),
                    ); // 00ea41d2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea41d5
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_sub(0x00000008)))),
                    ); // 00ea41dc
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea41df
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea41e1
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000088)))),
                    ); // 00ea41e3
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea41ea
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea41ec
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea41f0
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea41f2
                    ah = sw; // 00ea41f4
                    fl.and(u32::from(ah), 5, 8); // 00ea41f6
                    if fl.pf {
                        pc = 0x00ea_434d;
                        continue;
                    } // 00ea41f9
                    pc = 0x00ea_41ff;
                }
                0x00ea_41ff => {
                    x.push(0.0); // 00ea41ff
                    sw = fcom(x.st(0), x.st(2)); // 00ea4201
                    ah = sw; // 00ea4203
                    fl.and(u32::from(ah), 5, 8); // 00ea4205
                    if fl.pf {
                        pc = 0x00ea_4267;
                        continue;
                    } // 00ea4208
                    pc = 0x00ea_420a;
                }
                0x00ea_420a => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea420a
                    sw = fcom(x.st(0), x.st(1)); // 00ea420c
                    ah = sw; // 00ea420e
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4210
                    fl.and(u32::from(ah), 1, 8); // 00ea4212
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4215
                    if !fl.zf {
                        pc = 0x00ea_4278;
                        continue;
                    } // 00ea4219
                    pc = 0x00ea_421b;
                }
                0x00ea_421b => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea421b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea421d
                    x.push(x.st(0)); // 00ea4221
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea4223
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4225
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea4227
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea4229
                    ah = sw; // 00ea422b
                    fl.and(u32::from(ah), 5, 8); // 00ea422d
                    if !fl.pf {
                        pc = 0x00ea_4c0d;
                        continue;
                    } // 00ea4230
                    pc = 0x00ea_4236;
                }
                0x00ea_4236 => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4236
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea423a
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea423c
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea423e
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4240
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4244
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4248
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea424a
                    ah = sw; // 00ea424c
                    fl.and(u32::from(ah), 5, 8); // 00ea424e
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea4251
                    pc = 0x00ea_4257;
                }
                0x00ea_4257 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea4257
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea425b
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea425d
                    edi = ebp.wrapping_add(0x00000001); // 00ea425f
                    pc = 0x00ea_44a7;
                    continue; // 00ea4262
                }
                0x00ea_4267 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4267
                    sw = fcom(x.st(0), x.st(1)); // 00ea4269
                    ah = sw; // 00ea426b
                    fl.and(u32::from(ah), 65, 8); // 00ea426d
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4270
                    if fl.pf {
                        pc = 0x00ea_42e2;
                        continue;
                    } // 00ea4274
                    pc = 0x00ea_4276;
                }
                0x00ea_4276 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4276
                    pc = 0x00ea_4278;
                }
                0x00ea_4278 => {
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4278
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea427a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea427e
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea4282
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4284
                    ah = sw; // 00ea4286
                    fl.and(u32::from(ah), 5, 8); // 00ea4288
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea428b
                    pc = 0x00ea_4291;
                }
                0x00ea_4291 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4291
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4295
                    x.push(x.st(0)); // 00ea4299
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea429b
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea429d
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea429f
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea42a1
                    ah = sw; // 00ea42a3
                    fl.and(u32::from(ah), 5, 8); // 00ea42a5
                    if !fl.pf {
                        pc = 0x00ea_4c0d;
                        continue;
                    } // 00ea42a8
                    pc = 0x00ea_42ae;
                }
                0x00ea_42ae => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea42ae
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea42b2
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea42b4
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea42b6
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea42b8
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea42bc
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea42c0
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea42c2
                    ah = sw; // 00ea42c4
                    fl.and(u32::from(ah), 5, 8); // 00ea42c6
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea42c9
                    pc = 0x00ea_42cf;
                }
                0x00ea_42cf => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea42cf
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea42d3
                    ebp = 0x0000_0002; // 00ea42d5
                    edi = ebp.wrapping_sub(0x00000001); // 00ea42da
                    pc = 0x00ea_44a7;
                    continue; // 00ea42dd
                }
                0x00ea_42e2 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea42e2
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea42e4
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea42e6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea42ea
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea42ee
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea42f0
                    ah = sw; // 00ea42f2
                    fl.and(u32::from(ah), 5, 8); // 00ea42f4
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea42f7
                    pc = 0x00ea_42fd;
                }
                0x00ea_42fd => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea42fd
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4301
                    x.push(x.st(0)); // 00ea4305
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea4307
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4309
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea430b
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea430d
                    ah = sw; // 00ea430f
                    fl.and(u32::from(ah), 5, 8); // 00ea4311
                    if !fl.pf {
                        pc = 0x00ea_4c0d;
                        continue;
                    } // 00ea4314
                    pc = 0x00ea_431a;
                }
                0x00ea_431a => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea431a
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea431e
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea4320
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea4322
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4324
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4328
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea432c
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea432e
                    ah = sw; // 00ea4330
                    fl.and(u32::from(ah), 5, 8); // 00ea4332
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea4335
                    pc = 0x00ea_433b;
                }
                0x00ea_433b => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea433b
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea433f
                    ebp = 0x0000_0001; // 00ea4341
                    edi = ebp; // 00ea4346
                    pc = 0x00ea_44a7;
                    continue; // 00ea4348
                }
                0x00ea_434d => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea434d
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea4351
                    x.set(0, -x.st(0)); // 00ea4353
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4355
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea4359
                    sw = fcom(x.st(0), x.st(3)); // 00ea435d
                    ah = sw; // 00ea435f
                    fl.and(u32::from(ah), 65, 8); // 00ea4361
                    if !fl.zf {
                        pc = 0x00ea_449f;
                        continue;
                    } // 00ea4364
                    pc = 0x00ea_436a;
                }
                0x00ea_436a => {
                    x.push(0.0); // 00ea436a
                    sw = fcom(x.st(0), x.st(3)); // 00ea436c
                    ah = sw; // 00ea436e
                    fl.and(u32::from(ah), 1, 8); // 00ea4370
                    if !fl.zf {
                        pc = 0x00ea_43d0;
                        continue;
                    } // 00ea4373
                    pc = 0x00ea_4375;
                }
                0x00ea_4375 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4375
                    x.xch(2); // 00ea4377
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4379
                    ah = sw; // 00ea437b
                    fl.and(u32::from(ah), 1, 8); // 00ea437d
                    if !fl.zf {
                        pc = 0x00ea_43e1;
                        continue;
                    } // 00ea4380
                    pc = 0x00ea_4382;
                }
                0x00ea_4382 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4382
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4384
                    x.push(x.st(0)); // 00ea4388
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea438a
                    x.set(0, x87_add(x.st(0), x.st(3))); // 00ea438c
                    sw = fcom(x.st(0), x.st(2));
                    x.pop(); // 00ea438e
                    ah = sw; // 00ea4390
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4392
                    fl.and(u32::from(ah), 5, 8); // 00ea4394
                    if !fl.pf {
                        pc = 0x00ea_4c0d;
                        continue;
                    } // 00ea4397
                    pc = 0x00ea_439d;
                }
                0x00ea_439d => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea439d
                    x.push(x87_load32(e.mem.u32(eax))); // 00ea43a1
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea43a3
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea43a5
                    x.set(0, -x.st(0)); // 00ea43a7
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea43a9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea43ad
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea43b1
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea43b3
                    ah = sw; // 00ea43b5
                    fl.and(u32::from(ah), 5, 8); // 00ea43b7
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea43ba
                    pc = 0x00ea_43c0;
                }
                0x00ea_43c0 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea43c0
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea43c4
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea43c6
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea43c8
                    pc = 0x00ea_44a7;
                    continue; // 00ea43cb
                }
                0x00ea_43d0 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea43d0
                    x.xch(1); // 00ea43d2
                    sw = fcom(x.st(0), x.st(2)); // 00ea43d4
                    ah = sw; // 00ea43d6
                    fl.and(u32::from(ah), 1, 8); // 00ea43d8
                    if !fl.zf {
                        pc = 0x00ea_4440;
                        continue;
                    } // 00ea43db
                    pc = 0x00ea_43dd;
                }
                0x00ea_43dd => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea43dd
                    x.xch(1); // 00ea43df
                    pc = 0x00ea_43e1;
                }
                0x00ea_43e1 => {
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea43e1
                    x.xch(1); // 00ea43e3
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea43e5
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea43e9
                    sw = fcom(x.st(0), x.st(1)); // 00ea43ed
                    ah = sw; // 00ea43ef
                    fl.and(u32::from(ah), 5, 8); // 00ea43f1
                    if fl.pf {
                        pc = 0x00ea_44a3;
                        continue;
                    } // 00ea43f4
                    pc = 0x00ea_43fa;
                }
                0x00ea_43fa => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea43fa
                    x.push(x.st(0)); // 00ea43fe
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea4400
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4402
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4404
                    ah = sw; // 00ea4406
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4408
                    fl.and(u32::from(ah), 5, 8); // 00ea440a
                    if !fl.pf {
                        pc = 0x00ea_4a49;
                        continue;
                    } // 00ea440d
                    pc = 0x00ea_4413;
                }
                0x00ea_4413 => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4413
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea4417
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea4419
                    x.set(0, -x.st(0)); // 00ea441b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea441d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4421
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4425
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4427
                    ah = sw; // 00ea4429
                    fl.and(u32::from(ah), 5, 8); // 00ea442b
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea442e
                    pc = 0x00ea_4430;
                }
                0x00ea_4430 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea4430
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4434
                    ebp = 0x0000_0002; // 00ea4436
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea443b
                    pc = 0x00ea_44a7;
                    continue; // 00ea443e
                }
                0x00ea_4440 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4440
                    x.xch(1); // 00ea4442
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4444
                    x.xch(1); // 00ea4446
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4448
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea444c
                    sw = fcom(x.st(0), x.st(1)); // 00ea4450
                    ah = sw; // 00ea4452
                    fl.and(u32::from(ah), 5, 8); // 00ea4454
                    if fl.pf {
                        pc = 0x00ea_44a3;
                        continue;
                    } // 00ea4457
                    pc = 0x00ea_4459;
                }
                0x00ea_4459 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4459
                    x.push(x.st(0)); // 00ea445d
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea445f
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4461
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4463
                    ah = sw; // 00ea4465
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4467
                    fl.and(u32::from(ah), 5, 8); // 00ea4469
                    if !fl.pf {
                        pc = 0x00ea_4a49;
                        continue;
                    } // 00ea446c
                    pc = 0x00ea_4472;
                }
                0x00ea_4472 => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000014)); // 00ea4472
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea4476
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea4478
                    x.set(0, -x.st(0)); // 00ea447a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea447c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4480
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4484
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4486
                    ah = sw; // 00ea4488
                    fl.and(u32::from(ah), 5, 8); // 00ea448a
                    if fl.pf {
                        pc = 0x00ea_44a5;
                        continue;
                    } // 00ea448d
                    pc = 0x00ea_448f;
                }
                0x00ea_448f => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea448f
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4493
                    ebp = 0x0000_0001; // 00ea4495
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea449a
                    pc = 0x00ea_44a7;
                    continue; // 00ea449d
                }
                0x00ea_449f => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea449f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea44a1
                    pc = 0x00ea_44a3;
                }
                0x00ea_44a3 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea44a3
                    pc = 0x00ea_44a5;
                }
                0x00ea_44a5 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea44a5
                    pc = 0x00ea_44a7;
                }
                0x00ea_44a7 => {
                    eax = e.mem.u32(esp.wrapping_add(0x00000010)); // 00ea44a7
                    let r = fl.add(e.mem.u32(esp.wrapping_add(0x00000048)), 0x0000_000c, 32);
                    e.mem.set_u32(esp.wrapping_add(0x00000048), r); // 00ea44ab
                    let r = fl.add(e.mem.u32(esp.wrapping_add(0x00000014)), 0x0000_0004, 32);
                    e.mem.set_u32(esp.wrapping_add(0x00000014), r); // 00ea44b0
                    eax = fl.inc(eax); // 00ea44b5
                    e.mem.set_u32(esp.wrapping_add(0x00000010), eax); // 00ea44b6
                    let r = fl.add(edx, 0x0000_000c, 32);
                    edx = r; // 00ea44ba
                    let r = fl.add(eax, 0xffff_fffe, 32);
                    eax = r; // 00ea44bd
                    let r = fl.sub(eax, 0x0000_0003, 32); // 00ea44c0
                    e.mem.set_u32(esp.wrapping_add(0x0000002c), edx); // 00ea44c3
                    if fl.sf != fl.of {
                        pc = 0x00ea_4151;
                        continue;
                    } // 00ea44c7
                    pc = 0x00ea_44cd;
                }
                0x00ea_44cd => {
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea44cd
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea44d0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea44d4
                    x.push(x.st(0)); // 00ea44d8
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea44da
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea44dc
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea44df
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea44e1
                    x.xch(1); // 00ea44e3
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000003c), x87_store32(x.st(0)));
                    x.pop(); // 00ea44e5
                    edx = e.mem.u32(esp.wrapping_add(0x0000003c)); // 00ea44e9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea44ed
                    e.mem.set_u32(esp.wrapping_add(0x00000134), edx); // 00ea44f1
                    x.push(x.st(0)); // 00ea44f8
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000008)))),
                    ); // 00ea44fa
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea44fd
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea44ff
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea4501
                    x.xch(1); // 00ea4503
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000040), x87_store32(x.st(0)));
                    x.pop(); // 00ea4505
                    eax = e.mem.u32(esp.wrapping_add(0x00000040)); // 00ea4509
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea450d
                    e.mem.set_u32(esp.wrapping_add(0x00000138), eax); // 00ea450f
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea4516
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000004)))),
                    ); // 00ea4518
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea451b
                    x.xch(1); // 00ea451d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000044), x87_store32(x.st(0)));
                    x.pop(); // 00ea451f
                    edx = e.mem.u32(esp.wrapping_add(0x00000044)); // 00ea4523
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000040)))); // 00ea4527
                    e.mem.set_u32(esp.wrapping_add(0x0000013c), edx); // 00ea452b
                    x.push(x.st(0)); // 00ea4532
                    edx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea4534
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea4538
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000003c)))); // 00ea453c
                    x.push(x.st(0)); // 00ea4540
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000005c)))),
                    ); // 00ea4542
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4546
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000044)))); // 00ea4548
                    x.push(x.st(0)); // 00ea454c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea454e
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea4552
                    x.xch(2); // 00ea4554
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea4556
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea455a
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea4561
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000080)))),
                    ); // 00ea4563
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea456a
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000088)))),
                    ); // 00ea456c
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4573
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000048), x87_store32(x.st(0)));
                    x.pop(); // 00ea4575
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a0)))); // 00ea4579
                    x.set(0, x.st(0).abs()); // 00ea4580
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c0), x87_store32(x.st(0)));
                    x.pop(); // 00ea4582
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000ac)))); // 00ea4589
                    x.set(0, x.st(0).abs()); // 00ea4590
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000bc), x87_store32(x.st(0)));
                    x.pop(); // 00ea4592
                    x.push(x87_load32(e.mem.u32(edx.wrapping_add(0x00000004)))); // 00ea4599
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000bc)))),
                    ); // 00ea459c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c0)))); // 00ea45a3
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea45aa
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea45ad
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea45af
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea45b3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea45b7
                    sw = fcom(x.st(0), x.st(1)); // 00ea45bb
                    ah = sw; // 00ea45bd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea45bf
                    fl.and(u32::from(ah), 5, 8); // 00ea45c3
                    if fl.pf {
                        pc = 0x00ea_469f;
                        continue;
                    } // 00ea45c6
                    pc = 0x00ea_45cc;
                }
                0x00ea_45cc => {
                    x.push(0.0); // 00ea45cc
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea45ce
                    sw = fcom(x.st(0), x.st(1)); // 00ea45d2
                    ah = sw; // 00ea45d4
                    x.set(1, x.st(0));
                    x.pop(); // 00ea45d6
                    fl.and(u32::from(ah), 1, 8); // 00ea45d8
                    if !fl.zf {
                        pc = 0x00ea_4628;
                        continue;
                    } // 00ea45db
                    pc = 0x00ea_45dd;
                }
                0x00ea_45dd => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea45dd
                    x.push(x.st(0)); // 00ea45df
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea45e1
                    x.push(x.st(2)); // 00ea45e3
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea45e5
                    x.xch(3); // 00ea45e7
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea45e9
                    ah = sw; // 00ea45eb
                    fl.and(u32::from(ah), 65, 8); // 00ea45ed
                    if fl.zf {
                        pc = 0x00ea_48bd;
                        continue;
                    } // 00ea45f0
                    pc = 0x00ea_45f6;
                }
                0x00ea_45f6 => {
                    x.set(0, x87_sub(x.st(0), x.st(1))); // 00ea45f6
                    x.set(0, x87_div(x.st(0), x.st(2))); // 00ea45f8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea45fa
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea45fe
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4602
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4604
                    ah = sw; // 00ea4606
                    fl.and(u32::from(ah), 5, 8); // 00ea4608
                    if fl.pf {
                        pc = 0x00ea_4777;
                        continue;
                    } // 00ea460b
                    pc = 0x00ea_4611;
                }
                0x00ea_4611 => {
                    edi = 0x0000_0001; // 00ea4611
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4616
                    ecx = 0x0000_0005; // 00ea4618
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea461d
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea461f
                    pc = 0x00ea_4779;
                    continue; // 00ea4623
                }
                0x00ea_4628 => {
                    x.set(0, x87_add(x.st(0), x.st(3))); // 00ea4628
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea462a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea462e
                    sw = fcom(x.st(0), x.st(2)); // 00ea4632
                    ah = sw; // 00ea4634
                    fl.and(u32::from(ah), 65, 8); // 00ea4636
                    if !fl.zf {
                        pc = 0x00ea_4765;
                        continue;
                    } // 00ea4639
                    pc = 0x00ea_463f;
                }
                0x00ea_463f => {
                    x.push(x.st(1)); // 00ea463f
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea4641
                    x.push(x.st(0)); // 00ea4643
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea4645
                    x.xch(4); // 00ea4647
                    sw = fcom(x.st(0), x.st(2));
                    x.pop(); // 00ea4649
                    ah = sw; // 00ea464b
                    fl.and(u32::from(ah), 65, 8); // 00ea464d
                    if !fl.zf {
                        pc = 0x00ea_4667;
                        continue;
                    } // 00ea4650
                    pc = 0x00ea_4652;
                }
                0x00ea_4652 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4652
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4654
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4655
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4657
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4658
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea465a
                    x.set(0, x.st(0));
                    x.pop(); // 00ea465b
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea465d
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea465f
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4660
                    return (eax & 0xff) != 0; // 00ea4666
                }
                0x00ea_4667 => {
                    x.xch(1); // 00ea4667
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea4669
                    x.push(x.st(0)); // 00ea466b
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea466d
                    x.xch(2); // 00ea466f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4671
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4675
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4679
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea467b
                    ah = sw; // 00ea467d
                    fl.and(u32::from(ah), 5, 8); // 00ea467f
                    if fl.pf {
                        pc = 0x00ea_4777;
                        continue;
                    } // 00ea4682
                    pc = 0x00ea_4688;
                }
                0x00ea_4688 => {
                    ebp = 0x0000_0001; // 00ea4688
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea468d
                    edi = ebp; // 00ea468f
                    ecx = 0x0000_0005; // 00ea4691
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4696
                    pc = 0x00ea_4779;
                    continue; // 00ea469a
                }
                0x00ea_469f => {
                    x.push(x.st(1)); // 00ea469f
                    x.set(0, -x.st(0)); // 00ea46a1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea46a3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea46a7
                    sw = fcom(x.st(0), x.st(3)); // 00ea46ab
                    ah = sw; // 00ea46ad
                    fl.and(u32::from(ah), 65, 8); // 00ea46af
                    if !fl.zf {
                        pc = 0x00ea_4765;
                        continue;
                    } // 00ea46b2
                    pc = 0x00ea_46b8;
                }
                0x00ea_46b8 => {
                    x.push(0.0); // 00ea46b8
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea46ba
                    sw = fcom(x.st(0), x.st(1)); // 00ea46be
                    ah = sw; // 00ea46c0
                    x.set(1, x.st(0));
                    x.pop(); // 00ea46c2
                    fl.and(u32::from(ah), 65, 8); // 00ea46c4
                    if fl.pf {
                        pc = 0x00ea_470d;
                        continue;
                    } // 00ea46c7
                    pc = 0x00ea_46c9;
                }
                0x00ea_46c9 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea46c9
                    x.push(x.st(1)); // 00ea46cb
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea46cd
                    x.push(x.st(3)); // 00ea46cf
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea46d1
                    sw = fcom(x.st(0), x.st(4));
                    x.pop(); // 00ea46d3
                    ah = sw; // 00ea46d5
                    x.set(3, x.st(0));
                    x.pop(); // 00ea46d7
                    fl.and(u32::from(ah), 65, 8); // 00ea46d9
                    if fl.zf {
                        pc = 0x00ea_48bd;
                        continue;
                    } // 00ea46dc
                    pc = 0x00ea_46e2;
                }
                0x00ea_46e2 => {
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea46e2
                    x.set(0, x87_div(x.st(0), x.st(2))); // 00ea46e4
                    x.set(0, -x.st(0)); // 00ea46e6
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea46e8
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea46ec
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea46f0
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea46f2
                    ah = sw; // 00ea46f4
                    fl.and(u32::from(ah), 5, 8); // 00ea46f6
                    if fl.pf {
                        pc = 0x00ea_4777;
                        continue;
                    } // 00ea46f9
                    pc = 0x00ea_46fb;
                }
                0x00ea_46fb => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea46fb
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea46fd
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea46ff
                    ecx = 0x0000_0005; // 00ea4702
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4707
                    pc = 0x00ea_4779;
                    continue; // 00ea470b
                }
                0x00ea_470d => {
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea470d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea470f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea4713
                    sw = fcom(x.st(0), x.st(1)); // 00ea4717
                    ah = sw; // 00ea4719
                    fl.and(u32::from(ah), 5, 8); // 00ea471b
                    if fl.pf {
                        pc = 0x00ea_476d;
                        continue;
                    } // 00ea471e
                    pc = 0x00ea_4720;
                }
                0x00ea_4720 => {
                    x.push(x.st(2)); // 00ea4720
                    x.set(6, x87_mul(x.st(6), x.st(0)));
                    x.pop(); // 00ea4722
                    x.push(x.st(0)); // 00ea4724
                    x.set(6, x87_add(x.st(6), x.st(0)));
                    x.pop(); // 00ea4726
                    x.xch(5); // 00ea4728
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea472a
                    ah = sw; // 00ea472c
                    fl.and(u32::from(ah), 5, 8); // 00ea472e
                    if !fl.pf {
                        pc = 0x00ea_4652;
                        continue;
                    } // 00ea4731
                    pc = 0x00ea_4737;
                }
                0x00ea_4737 => {
                    x.xch(3); // 00ea4737
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4739
                    x.set(0, x87_div(x.st(0), x.st(2))); // 00ea473b
                    x.set(0, -x.st(0)); // 00ea473d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea473f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4743
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4747
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4749
                    ah = sw; // 00ea474b
                    fl.and(u32::from(ah), 5, 8); // 00ea474d
                    if fl.pf {
                        pc = 0x00ea_4777;
                        continue;
                    } // 00ea4750
                    pc = 0x00ea_4752;
                }
                0x00ea_4752 => {
                    ecx = 0x0000_0005; // 00ea4752
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4757
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4759
                    ebp = ecx.wrapping_sub(0x00000004); // 00ea475c
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea475f
                    pc = 0x00ea_4779;
                    continue; // 00ea4763
                }
                0x00ea_4765 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4765
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4767
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4769
                    pc = 0x00ea_4779;
                    continue; // 00ea476b
                }
                0x00ea_476d => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea476d
                    x.set(0, x.st(0));
                    x.pop(); // 00ea476f
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4771
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4773
                    pc = 0x00ea_4779;
                    continue; // 00ea4775
                }
                0x00ea_4777 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4777
                    pc = 0x00ea_4779;
                }
                0x00ea_4779 => {
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea4779
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000058)))); // 00ea477c
                    x.push(x.st(0)); // 00ea4780
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4782
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea4784
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea4787
                    x.push(x.st(0)); // 00ea478b
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea478d
                    x.xch(3); // 00ea478f
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea4791
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000003c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4793
                    eax = e.mem.u32(esp.wrapping_add(0x0000003c)); // 00ea4797
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea479b
                    e.mem.set_u32(esp.wrapping_add(0x0000011c), eax); // 00ea479f
                    x.push(x.st(0)); // 00ea47a6
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000008)))),
                    ); // 00ea47a8
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea47ab
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea47ad
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea47af
                    x.xch(1); // 00ea47b1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000040), x87_store32(x.st(0)));
                    x.pop(); // 00ea47b3
                    eax = e.mem.u32(esp.wrapping_add(0x00000040)); // 00ea47b7
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea47bb
                    e.mem.set_u32(esp.wrapping_add(0x00000120), eax); // 00ea47bd
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea47c4
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000004)))),
                    ); // 00ea47c6
                    x.set(1, x87_sub(x.st(1), x.st(0)));
                    x.pop(); // 00ea47c9
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000044), x87_store32(x.st(0)));
                    x.pop(); // 00ea47cb
                    eax = e.mem.u32(esp.wrapping_add(0x00000044)); // 00ea47cf
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000040)))); // 00ea47d3
                    e.mem.set_u32(esp.wrapping_add(0x00000124), eax); // 00ea47d7
                    x.push(x.st(0)); // 00ea47de
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea47e0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea47e4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000003c)))); // 00ea47e8
                    x.push(x.st(0)); // 00ea47ec
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea47ee
                    x.xch(2); // 00ea47f0
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea47f2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000044)))); // 00ea47f4
                    x.push(x.st(0)); // 00ea47f8
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea47fa
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea47fe
                    x.xch(1); // 00ea4800
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea4802
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea4806
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea480d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea480f
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4816
                    x.xch(2); // 00ea4818
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea481a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea481c
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4823
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4825
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4827
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a4)))); // 00ea482e
                    x.set(0, x.st(0).abs()); // 00ea4835
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4837
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b0)))); // 00ea483b
                    x.set(0, x.st(0).abs()); // 00ea4842
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b8), x87_store32(x.st(0)));
                    x.pop(); // 00ea4844
                    x.push(x87_load32(e.mem.u32(edx.wrapping_add(0x00000004)))); // 00ea484b
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b8)))),
                    ); // 00ea484e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea4855
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea4859
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea485c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea485e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea4862
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4866
                    sw = fcom(x.st(0), x.st(1)); // 00ea486a
                    ah = sw; // 00ea486c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea486e
                    fl.and(u32::from(ah), 5, 8); // 00ea4875
                    if fl.pf {
                        pc = 0x00ea_4972;
                        continue;
                    } // 00ea4878
                    pc = 0x00ea_487e;
                }
                0x00ea_487e => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea487e
                    x.set(0, -x.st(0)); // 00ea4882
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4884
                    x.push(0.0); // 00ea4888
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea488a
                    sw = fcom(x.st(0), x.st(1)); // 00ea488e
                    ah = sw; // 00ea4890
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4892
                    fl.and(u32::from(ah), 1, 8); // 00ea4894
                    if !fl.zf {
                        pc = 0x00ea_4908;
                        continue;
                    } // 00ea4897
                    pc = 0x00ea_4899;
                }
                0x00ea_4899 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4899
                    x.push(x.st(0)); // 00ea489b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea489d
                    x.push(x.st(0)); // 00ea48a4
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea48a6
                    x.push(x.st(4)); // 00ea48a8
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea48aa
                    x.xch(1); // 00ea48ac
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea48ae
                    ah = sw; // 00ea48b0
                    fl.and(u32::from(ah), 65, 8); // 00ea48b2
                    if !fl.zf {
                        pc = 0x00ea_48d0;
                        continue;
                    } // 00ea48b5
                    pc = 0x00ea_48b7;
                }
                0x00ea_48b7 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea48b7
                    x.set(1, x.st(0));
                    x.pop(); // 00ea48b9
                    x.set(0, x.st(0));
                    x.pop(); // 00ea48bb
                    pc = 0x00ea_48bd;
                }
                0x00ea_48bd => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea48bd
                    x.set(0, x.st(0));
                    x.pop(); // 00ea48be
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea48c0
                    x.set(0, x.st(0));
                    x.pop(); // 00ea48c1
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea48c3
                    x.set(0, x.st(0));
                    x.pop(); // 00ea48c4
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea48c6
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea48c8
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea48c9
                    return (eax & 0xff) != 0; // 00ea48cf
                }
                0x00ea_48d0 => {
                    x.push(x.st(3)); // 00ea48d0
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea48d2
                    x.push(x.st(1)); // 00ea48d4
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea48d6
                    x.xch(2); // 00ea48d8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea48da
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea48de
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea48e2
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea48e4
                    ah = sw; // 00ea48e6
                    fl.and(u32::from(ah), 5, 8); // 00ea48e8
                    if fl.pf {
                        pc = 0x00ea_4b8c;
                        continue;
                    } // 00ea48eb
                    pc = 0x00ea_48f1;
                }
                0x00ea_48f1 => {
                    edi = 0x0000_0001; // 00ea48f1
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea48f6
                    ecx = 0x0000_0006; // 00ea48f8
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea48fd
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea48ff
                    pc = 0x00ea_4a9b;
                    continue; // 00ea4903
                }
                0x00ea_4908 => {
                    x.set(0, x87_add(x.st(0), x.st(3))); // 00ea4908
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea490a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea490e
                    sw = fcom(x.st(0), x.st(2)); // 00ea4912
                    ah = sw; // 00ea4914
                    fl.and(u32::from(ah), 65, 8); // 00ea4916
                    if !fl.zf {
                        pc = 0x00ea_4b7a;
                        continue;
                    } // 00ea4919
                    pc = 0x00ea_491f;
                }
                0x00ea_491f => {
                    x.push(x.st(1)); // 00ea491f
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea4921
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea4928
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea492a
                    ah = sw; // 00ea492c
                    fl.and(u32::from(ah), 65, 8); // 00ea492e
                    if fl.zf {
                        pc = 0x00ea_48b7;
                        continue;
                    } // 00ea4931
                    pc = 0x00ea_4933;
                }
                0x00ea_4933 => {
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea4933
                    x.push(x.st(0)); // 00ea4935
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea4937
                    x.xch(1); // 00ea4939
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea493b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea493f
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4943
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4945
                    ah = sw; // 00ea4947
                    fl.and(u32::from(ah), 5, 8); // 00ea4949
                    if fl.pf {
                        pc = 0x00ea_4b93;
                        continue;
                    } // 00ea494c
                    pc = 0x00ea_4952;
                }
                0x00ea_4952 => {
                    ebp = 0x0000_0001; // 00ea4952
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4957
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea4959
                    edi = ebp; // 00ea4960
                    ecx = 0x0000_0006; // 00ea4962
                    x.xch(1); // 00ea4967
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4969
                    pc = 0x00ea_4a9b;
                    continue; // 00ea496d
                }
                0x00ea_4972 => {
                    x.push(x.st(1)); // 00ea4972
                    x.set(0, -x.st(0)); // 00ea4974
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4976
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea497a
                    sw = fcom(x.st(0), x.st(3)); // 00ea497e
                    ah = sw; // 00ea4980
                    fl.and(u32::from(ah), 65, 8); // 00ea4982
                    if !fl.zf {
                        pc = 0x00ea_4b7a;
                        continue;
                    } // 00ea4985
                    pc = 0x00ea_498b;
                }
                0x00ea_498b => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea498b
                    x.set(0, -x.st(0)); // 00ea498f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4991
                    x.push(0.0); // 00ea4995
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4997
                    sw = fcom(x.st(0), x.st(1)); // 00ea499b
                    ah = sw; // 00ea499d
                    x.set(1, x.st(0));
                    x.pop(); // 00ea499f
                    fl.and(u32::from(ah), 65, 8); // 00ea49a1
                    if fl.pf {
                        pc = 0x00ea_4a14;
                        continue;
                    } // 00ea49a4
                    pc = 0x00ea_49a6;
                }
                0x00ea_49a6 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea49a6
                    x.push(x.st(1)); // 00ea49a8
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea49aa
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea49b1
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea49b3
                    ah = sw; // 00ea49b5
                    fl.and(u32::from(ah), 5, 8); // 00ea49b7
                    if fl.pf {
                        pc = 0x00ea_49d3;
                        continue;
                    } // 00ea49ba
                    pc = 0x00ea_49bc;
                }
                0x00ea_49bc => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea49bc
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea49be
                    x.set(0, x.st(0));
                    x.pop(); // 00ea49bf
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea49c1
                    x.set(0, x.st(0));
                    x.pop(); // 00ea49c2
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea49c4
                    x.set(0, x.st(0));
                    x.pop(); // 00ea49c5
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea49c7
                    x.set(0, x.st(0));
                    x.pop(); // 00ea49c9
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea49cb
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea49cc
                    return (eax & 0xff) != 0; // 00ea49d2
                }
                0x00ea_49d3 => {
                    x.push(x.st(2)); // 00ea49d3
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea49d5
                    x.push(x.st(0)); // 00ea49d7
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea49d9
                    x.xch(1); // 00ea49db
                    x.set(0, -x.st(0)); // 00ea49dd
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea49df
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea49e3
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea49e7
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea49e9
                    ah = sw; // 00ea49eb
                    fl.and(u32::from(ah), 5, 8); // 00ea49ed
                    if fl.pf {
                        pc = 0x00ea_4b93;
                        continue;
                    } // 00ea49f0
                    pc = 0x00ea_49f6;
                }
                0x00ea_49f6 => {
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea49f6
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea49f8
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea49fa
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4a01
                    ecx = 0x0000_0006; // 00ea4a04
                    x.xch(1); // 00ea4a09
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4a0b
                    pc = 0x00ea_4a9b;
                    continue; // 00ea4a0f
                }
                0x00ea_4a14 => {
                    x.set(6, x.st(0));
                    x.pop(); // 00ea4a14
                    x.push(x.st(3)); // 00ea4a16
                    x.set(6, x87_add(x.st(6), x.st(0)));
                    x.pop(); // 00ea4a18
                    x.xch(5); // 00ea4a1a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea4a1c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea4a20
                    sw = fcom(x.st(0), x.st(5)); // 00ea4a24
                    ah = sw; // 00ea4a26
                    fl.and(u32::from(ah), 5, 8); // 00ea4a28
                    if fl.pf {
                        pc = 0x00ea_4b6f;
                        continue;
                    } // 00ea4a2b
                    pc = 0x00ea_4a31;
                }
                0x00ea_4a31 => {
                    x.push(x.st(1)); // 00ea4a31
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea4a33
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea4a3a
                    sw = fcom(x.st(0), x.st(6));
                    x.pop(); // 00ea4a3c
                    ah = sw; // 00ea4a3e
                    x.set(5, x.st(0));
                    x.pop(); // 00ea4a40
                    fl.and(u32::from(ah), 5, 8); // 00ea4a42
                    if fl.pf {
                        pc = 0x00ea_4a5e;
                        continue;
                    } // 00ea4a45
                    pc = 0x00ea_4a47;
                }
                0x00ea_4a47 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4a47
                    pc = 0x00ea_4a49;
                }
                0x00ea_4a49 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4a49
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4a4b
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4a4c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4a4e
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4a4f
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4a51
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4a52
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4a54
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4a56
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4a57
                    return (eax & 0xff) != 0; // 00ea4a5d
                }
                0x00ea_4a5e => {
                    x.xch(4); // 00ea4a5e
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4a60
                    x.set(0, x87_div(x.st(0), x.st(3))); // 00ea4a62
                    x.set(0, -x.st(0)); // 00ea4a64
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4a66
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4a6a
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4a6e
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4a70
                    ah = sw; // 00ea4a72
                    fl.and(u32::from(ah), 5, 8); // 00ea4a74
                    if fl.pf {
                        pc = 0x00ea_4b73;
                        continue;
                    } // 00ea4a77
                    pc = 0x00ea_4a7d;
                }
                0x00ea_4a7d => {
                    ecx = 0x0000_0006; // 00ea4a7d
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4a82
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4a84
                    ebp = ecx.wrapping_sub(0x00000005); // 00ea4a87
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4a8a
                    pc = 0x00ea_4a8e;
                }
                0x00ea_4a8e => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea4a8e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea4a95
                    x.xch(4); // 00ea4a99
                    pc = 0x00ea_4a9b;
                }
                0x00ea_4a9b => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a4)))); // 00ea4a9b
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a0)))),
                    ); // 00ea4aa2
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a8), x87_store32(x.st(0)));
                    x.pop(); // 00ea4aa9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b0)))); // 00ea4ab0
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000ac)))),
                    ); // 00ea4ab7
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b4), x87_store32(x.st(0)));
                    x.pop(); // 00ea4abe
                    x.xch(2); // 00ea4ac5
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea4ac7
                    x.xch(2); // 00ea4ac9
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4acb
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea4ad2
                    x.xch(1); // 00ea4ad4
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4ad6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b4)))); // 00ea4ada
                    x.set(0, x.st(0).abs()); // 00ea4ae1
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c4), x87_store32(x.st(0)));
                    x.pop(); // 00ea4ae3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a8)))); // 00ea4aea
                    x.set(0, x.st(0).abs()); // 00ea4af1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea4af3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea4af7
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea4afb
                    x.push(x87_load32(e.mem.u32(edx.wrapping_add(0x00000004)))); // 00ea4afe
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000c4)))),
                    ); // 00ea4b01
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4b08
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4b0a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea4b0e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4b15
                    sw = fcom(x.st(0), x.st(1)); // 00ea4b19
                    ah = sw; // 00ea4b1b
                    fl.and(u32::from(ah), 5, 8); // 00ea4b1d
                    if fl.pf {
                        pc = 0x00ea_4c56;
                        continue;
                    } // 00ea4b20
                    pc = 0x00ea_4b26;
                }
                0x00ea_4b26 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea4b26
                    x.set(0, -x.st(0)); // 00ea4b2a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4b2c
                    x.push(0.0); // 00ea4b30
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4b32
                    sw = fcom(x.st(0), x.st(1)); // 00ea4b36
                    ah = sw; // 00ea4b38
                    fl.and(u32::from(ah), 1, 8); // 00ea4b3a
                    if !fl.zf {
                        pc = 0x00ea_4bd9;
                        continue;
                    } // 00ea4b3d
                    pc = 0x00ea_4b43;
                }
                0x00ea_4b43 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4b43
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4b45
                    x.push(x.st(0)); // 00ea4b49
                    x.set(0, x87_mul(x.st(0), x.st(5))); // 00ea4b4b
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea4b4d
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4b4f
                    ah = sw; // 00ea4b51
                    fl.and(u32::from(ah), 65, 8); // 00ea4b53
                    if !fl.zf {
                        pc = 0x00ea_4ba3;
                        continue;
                    } // 00ea4b56
                    pc = 0x00ea_4b58;
                }
                0x00ea_4b58 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4b58
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4b5a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4b5b
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4b5d
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4b5e
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4b60
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4b61
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4b63
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4b65
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4b67
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4b68
                    return (eax & 0xff) != 0; // 00ea4b6e
                }
                0x00ea_4b6f => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4b6f
                    x.set(4, x.st(0));
                    x.pop(); // 00ea4b71
                    pc = 0x00ea_4b73;
                }
                0x00ea_4b73 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4b73
                    pc = 0x00ea_4a8e;
                    continue; // 00ea4b75
                }
                0x00ea_4b7a => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4b7a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4b7c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea4b7e
                    x.xch(1); // 00ea4b85
                    pc = 0x00ea_4a9b;
                    continue; // 00ea4b87
                }
                0x00ea_4b8c => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4b8c
                    pc = 0x00ea_4a9b;
                    continue; // 00ea4b8e
                }
                0x00ea_4b93 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4b93
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea4b95
                    x.xch(1); // 00ea4b9c
                    pc = 0x00ea_4a9b;
                    continue; // 00ea4b9e
                }
                0x00ea_4ba3 => {
                    x.xch(2); // 00ea4ba3
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea4ba5
                    x.xch(2); // 00ea4ba7
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea4ba9
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4bab
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4baf
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4bb3
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4bb5
                    ah = sw; // 00ea4bb7
                    fl.and(u32::from(ah), 5, 8); // 00ea4bb9
                    if fl.pf {
                        pc = 0x00ea_4d6f;
                        continue;
                    } // 00ea4bbc
                    pc = 0x00ea_4bc2;
                }
                0x00ea_4bc2 => {
                    edi = 0x0000_0001; // 00ea4bc2
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4bc7
                    ecx = 0x0000_0007; // 00ea4bc9
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea4bce
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4bd0
                    pc = 0x00ea_4d71;
                    continue; // 00ea4bd4
                }
                0x00ea_4bd9 => {
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea4bd9
                    x.xch(2); // 00ea4bdb
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4bdd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea4be4
                    sw = fcom(x.st(0), x.st(1)); // 00ea4beb
                    ah = sw; // 00ea4bed
                    fl.and(u32::from(ah), 65, 8); // 00ea4bef
                    if !fl.zf {
                        pc = 0x00ea_4d67;
                        continue;
                    } // 00ea4bf2
                    pc = 0x00ea_4bf8;
                }
                0x00ea_4bf8 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4bf8
                    x.push(x.st(0)); // 00ea4bfc
                    x.set(0, x87_mul(x.st(0), x.st(5))); // 00ea4bfe
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4c00
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4c02
                    ah = sw; // 00ea4c04
                    fl.and(u32::from(ah), 65, 8); // 00ea4c06
                    if !fl.zf {
                        pc = 0x00ea_4c22;
                        continue;
                    } // 00ea4c09
                    pc = 0x00ea_4c0b;
                }
                0x00ea_4c0b => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4c0b
                    pc = 0x00ea_4c0d;
                }
                0x00ea_4c0d => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4c0d
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4c0f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4c10
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4c12
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4c13
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4c15
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4c16
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4c18
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4c1a
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4c1b
                    return (eax & 0xff) != 0; // 00ea4c21
                }
                0x00ea_4c22 => {
                    x.xch(2); // 00ea4c22
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea4c24
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea4c26
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4c28
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4c2c
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4c30
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4c32
                    ah = sw; // 00ea4c34
                    fl.and(u32::from(ah), 5, 8); // 00ea4c36
                    if fl.pf {
                        pc = 0x00ea_4d6f;
                        continue;
                    } // 00ea4c39
                    pc = 0x00ea_4c3f;
                }
                0x00ea_4c3f => {
                    ebp = 0x0000_0001; // 00ea4c3f
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4c44
                    edi = ebp; // 00ea4c46
                    ecx = 0x0000_0007; // 00ea4c48
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4c4d
                    pc = 0x00ea_4d71;
                    continue; // 00ea4c51
                }
                0x00ea_4c56 => {
                    x.push(x.st(0)); // 00ea4c56
                    x.set(0, -x.st(0)); // 00ea4c58
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4c5a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4c5e
                    sw = fcom(x.st(0), x.st(2)); // 00ea4c62
                    ah = sw; // 00ea4c64
                    fl.and(u32::from(ah), 65, 8); // 00ea4c66
                    if !fl.zf {
                        pc = 0x00ea_4d5d;
                        continue;
                    } // 00ea4c69
                    pc = 0x00ea_4c6f;
                }
                0x00ea_4c6f => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea4c6f
                    x.set(0, -x.st(0)); // 00ea4c73
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4c75
                    x.push(0.0); // 00ea4c79
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4c7b
                    sw = fcom(x.st(0), x.st(1)); // 00ea4c7f
                    ah = sw; // 00ea4c81
                    fl.and(u32::from(ah), 65, 8); // 00ea4c83
                    if fl.pf {
                        pc = 0x00ea_4cea;
                        continue;
                    } // 00ea4c86
                    pc = 0x00ea_4c88;
                }
                0x00ea_4c88 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4c88
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4c8a
                    x.push(x.st(0)); // 00ea4c8e
                    x.set(0, x87_mul(x.st(0), x.st(6))); // 00ea4c90
                    x.set(0, x87_add(x.st(0), x.st(5))); // 00ea4c92
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4c94
                    ah = sw; // 00ea4c96
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4c98
                    fl.and(u32::from(ah), 5, 8); // 00ea4c9a
                    if fl.pf {
                        pc = 0x00ea_4cb6;
                        continue;
                    } // 00ea4c9d
                    pc = 0x00ea_4c9f;
                }
                0x00ea_4c9f => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4c9f
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4ca1
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4ca2
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4ca4
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4ca5
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4ca7
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4ca8
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4caa
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4cac
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4cae
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4caf
                    return (eax & 0xff) != 0; // 00ea4cb5
                }
                0x00ea_4cb6 => {
                    x.xch(3); // 00ea4cb6
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4cb8
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea4cba
                    x.set(0, -x.st(0)); // 00ea4cbc
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4cbe
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4cc2
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4cc6
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4cc8
                    ah = sw; // 00ea4cca
                    fl.and(u32::from(ah), 5, 8); // 00ea4ccc
                    if fl.pf {
                        pc = 0x00ea_4d6f;
                        continue;
                    } // 00ea4ccf
                    pc = 0x00ea_4cd5;
                }
                0x00ea_4cd5 => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea4cd5
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4cd7
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4cd9
                    ecx = 0x0000_0007; // 00ea4cdc
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4ce1
                    pc = 0x00ea_4d71;
                    continue; // 00ea4ce5
                }
                0x00ea_4cea => {
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea4cea
                    x.xch(3); // 00ea4cec
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4cee
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea4cf5
                    sw = fcom(x.st(0), x.st(1)); // 00ea4cfc
                    ah = sw; // 00ea4cfe
                    fl.and(u32::from(ah), 5, 8); // 00ea4d00
                    if fl.pf {
                        pc = 0x00ea_4d6b;
                        continue;
                    } // 00ea4d03
                    pc = 0x00ea_4d05;
                }
                0x00ea_4d05 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4d05
                    x.push(x.st(0)); // 00ea4d09
                    x.set(0, x87_mul(x.st(0), x.st(6))); // 00ea4d0b
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea4d0d
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4d0f
                    ah = sw; // 00ea4d11
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4d13
                    fl.and(u32::from(ah), 5, 8); // 00ea4d15
                    if fl.pf {
                        pc = 0x00ea_4d31;
                        continue;
                    } // 00ea4d18
                    pc = 0x00ea_4d1a;
                }
                0x00ea_4d1a => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d1a
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4d1c
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4d1d
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4d1f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d20
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4d22
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4d23
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4d25
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d27
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4d29
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4d2a
                    return (eax & 0xff) != 0; // 00ea4d30
                }
                0x00ea_4d31 => {
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4d31
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea4d33
                    x.set(0, -x.st(0)); // 00ea4d35
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4d37
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4d3b
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4d3f
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4d41
                    ah = sw; // 00ea4d43
                    fl.and(u32::from(ah), 5, 8); // 00ea4d45
                    if fl.pf {
                        pc = 0x00ea_4d6f;
                        continue;
                    } // 00ea4d48
                    pc = 0x00ea_4d4a;
                }
                0x00ea_4d4a => {
                    ecx = 0x0000_0007; // 00ea4d4a
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4d4f
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4d51
                    ebp = ecx.wrapping_sub(0x00000006); // 00ea4d54
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4d57
                    pc = 0x00ea_4d71;
                    continue; // 00ea4d5b
                }
                0x00ea_4d5d => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4d5d
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d5f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d61
                    x.push(0.0); // 00ea4d63
                    pc = 0x00ea_4d71;
                    continue; // 00ea4d65
                }
                0x00ea_4d67 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d67
                    pc = 0x00ea_4d6f;
                    continue; // 00ea4d69
                }
                0x00ea_4d6b => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d6b
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4d6d
                    pc = 0x00ea_4d6f;
                }
                0x00ea_4d6f => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4d6f
                    pc = 0x00ea_4d71;
                }
                0x00ea_4d71 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea4d71
                    x.push(x.st(0)); // 00ea4d75
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000010)))),
                    ); // 00ea4d77
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea4d7a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea4d7d
                    x.push(x.st(0)); // 00ea4d81
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4d83
                    x.xch(2); // 00ea4d85
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea4d87
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000003c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4d89
                    eax = e.mem.u32(esp.wrapping_add(0x0000003c)); // 00ea4d8d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea4d91
                    e.mem.set_u32(esp.wrapping_add(0x00000110), eax); // 00ea4d95
                    x.push(x.st(0)); // 00ea4d9c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000014)))),
                    ); // 00ea4d9e
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea4da1
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea4da4
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea4da6
                    x.xch(2); // 00ea4da8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000040), x87_store32(x.st(0)));
                    x.pop(); // 00ea4daa
                    eax = e.mem.u32(esp.wrapping_add(0x00000040)); // 00ea4dae
                    e.mem.set_u32(esp.wrapping_add(0x00000114), eax); // 00ea4db2
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x0000000c)))),
                    ); // 00ea4db9
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea4dbc
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4dbf
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea4dc1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000044), x87_store32(x.st(0)));
                    x.pop(); // 00ea4dc3
                    eax = e.mem.u32(esp.wrapping_add(0x00000044)); // 00ea4dc7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000040)))); // 00ea4dcb
                    e.mem.set_u32(esp.wrapping_add(0x00000118), eax); // 00ea4dcf
                    x.push(x.st(0)); // 00ea4dd6
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea4dd8
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea4ddc
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000003c)))); // 00ea4de0
                    x.push(x.st(0)); // 00ea4de4
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4de6
                    x.xch(2); // 00ea4de8
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4dea
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000044)))); // 00ea4dec
                    x.push(x.st(0)); // 00ea4df0
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea4df2
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4df6
                    x.xch(1); // 00ea4df8
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4dfa
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea4dfe
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea4e05
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea4e07
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4e0e
                    x.xch(2); // 00ea4e10
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4e12
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea4e14
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea4e1b
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4e1d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000048), x87_store32(x.st(0)));
                    x.pop(); // 00ea4e1f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000094)))); // 00ea4e23
                    x.set(0, x.st(0).abs()); // 00ea4e2a
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea4e2c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea4e33
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea4e3a
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea4e3d
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000bc)))),
                    ); // 00ea4e3f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea4e46
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4e48
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea4e4c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4e50
                    sw = fcom(x.st(0), x.st(1)); // 00ea4e54
                    ah = sw; // 00ea4e56
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea4e58
                    fl.and(u32::from(ah), 5, 8); // 00ea4e5c
                    if fl.pf {
                        pc = 0x00ea_4f40;
                        continue;
                    } // 00ea4e5f
                    pc = 0x00ea_4e65;
                }
                0x00ea_4e65 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea4e65
                    sw = fcom(x.st(0), x.st(4)); // 00ea4e69
                    ah = sw; // 00ea4e6b
                    x.set(4, x.st(0));
                    x.pop(); // 00ea4e6d
                    fl.and(u32::from(ah), 1, 8); // 00ea4e6f
                    if !fl.zf {
                        pc = 0x00ea_4ec1;
                        continue;
                    } // 00ea4e72
                    pc = 0x00ea_4e74;
                }
                0x00ea_4e74 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4e74
                    x.push(x.st(2)); // 00ea4e76
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea4e78
                    x.push(x.st(1)); // 00ea4e7a
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea4e7c
                    sw = fcom(x.st(0), x.st(3)); // 00ea4e7e
                    ah = sw; // 00ea4e80
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4e82
                    fl.and(u32::from(ah), 5, 8); // 00ea4e84
                    if !fl.pf {
                        pc = 0x00ea_4ef7;
                        continue;
                    } // 00ea4e87
                    pc = 0x00ea_4e89;
                }
                0x00ea_4e89 => {
                    x.push(x.st(0)); // 00ea4e89
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea4e8b
                    x.push(x.st(1)); // 00ea4e8d
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea4e8f
                    x.xch(2); // 00ea4e91
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4e93
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4e97
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4e9b
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4e9d
                    ah = sw; // 00ea4e9f
                    fl.and(u32::from(ah), 5, 8); // 00ea4ea1
                    if fl.pf {
                        pc = 0x00ea_5056;
                        continue;
                    } // 00ea4ea4
                    pc = 0x00ea_4eaa;
                }
                0x00ea_4eaa => {
                    edi = 0x0000_0001; // 00ea4eaa
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4eaf
                    ecx = 0x0000_0008; // 00ea4eb1
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea4eb6
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4eb8
                    pc = 0x00ea_5058;
                    continue; // 00ea4ebc
                }
                0x00ea_4ec1 => {
                    x.push(x.st(2)); // 00ea4ec1
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea4ec3
                    x.xch(3); // 00ea4ec5
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000bc), x87_store32(x.st(0)));
                    x.pop(); // 00ea4ec7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000bc)))); // 00ea4ece
                    sw = fcom(x.st(0), x.st(1)); // 00ea4ed5
                    ah = sw; // 00ea4ed7
                    fl.and(u32::from(ah), 65, 8); // 00ea4ed9
                    if !fl.zf {
                        pc = 0x00ea_5032;
                        continue;
                    } // 00ea4edc
                    pc = 0x00ea_4ee2;
                }
                0x00ea_4ee2 => {
                    x.push(x.st(3)); // 00ea4ee2
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea4ee4
                    x.push(x.st(0)); // 00ea4ee6
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea4ee8
                    x.xch(4); // 00ea4eea
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4eec
                    ah = sw; // 00ea4eee
                    fl.and(u32::from(ah), 65, 8); // 00ea4ef0
                    if !fl.zf {
                        pc = 0x00ea_4f0a;
                        continue;
                    } // 00ea4ef3
                    pc = 0x00ea_4ef5;
                }
                0x00ea_4ef5 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4ef5
                    pc = 0x00ea_4ef7;
                }
                0x00ea_4ef7 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4ef7
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4ef8
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4efa
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4efb
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4efd
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4efe
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4f00
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4f02
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4f03
                    return (eax & 0xff) != 0; // 00ea4f09
                }
                0x00ea_4f0a => {
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea4f0a
                    x.push(x.st(1)); // 00ea4f0c
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea4f0e
                    x.xch(2); // 00ea4f10
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4f12
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4f16
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4f1a
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4f1c
                    ah = sw; // 00ea4f1e
                    fl.and(u32::from(ah), 5, 8); // 00ea4f20
                    if fl.pf {
                        pc = 0x00ea_5056;
                        continue;
                    } // 00ea4f23
                    pc = 0x00ea_4f29;
                }
                0x00ea_4f29 => {
                    ebp = 0x0000_0001; // 00ea4f29
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4f2e
                    edi = ebp; // 00ea4f30
                    ecx = 0x0000_0008; // 00ea4f32
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4f37
                    pc = 0x00ea_5058;
                    continue; // 00ea4f3b
                }
                0x00ea_4f40 => {
                    x.push(x.st(1)); // 00ea4f40
                    x.set(0, -x.st(0)); // 00ea4f42
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea4f44
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea4f48
                    sw = fcom(x.st(0), x.st(3)); // 00ea4f4c
                    ah = sw; // 00ea4f4e
                    fl.and(u32::from(ah), 65, 8); // 00ea4f50
                    if !fl.zf {
                        pc = 0x00ea_503a;
                        continue;
                    } // 00ea4f53
                    pc = 0x00ea_4f59;
                }
                0x00ea_4f59 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea4f59
                    sw = fcom(x.st(0), x.st(5)); // 00ea4f5d
                    ah = sw; // 00ea4f5f
                    x.set(5, x.st(0));
                    x.pop(); // 00ea4f61
                    fl.and(u32::from(ah), 65, 8); // 00ea4f63
                    if fl.pf {
                        pc = 0x00ea_4fcc;
                        continue;
                    } // 00ea4f66
                    pc = 0x00ea_4f68;
                }
                0x00ea_4f68 => {
                    x.set(4, x.st(0));
                    x.pop(); // 00ea4f68
                    x.push(x.st(0)); // 00ea4f6a
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea4f6c
                    x.push(x.st(2)); // 00ea4f6e
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea4f70
                    x.xch(4); // 00ea4f72
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea4f74
                    ah = sw; // 00ea4f76
                    x.set(2, x.st(0));
                    x.pop(); // 00ea4f78
                    fl.and(u32::from(ah), 5, 8); // 00ea4f7a
                    if fl.pf {
                        pc = 0x00ea_4f92;
                        continue;
                    } // 00ea4f7d
                    pc = 0x00ea_4f7f;
                }
                0x00ea_4f7f => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4f7f
                    x.set(1, x.st(0));
                    x.pop(); // 00ea4f80
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4f82
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4f83
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4f85
                    x.set(0, x.st(0));
                    x.pop(); // 00ea4f86
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea4f88
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea4f8a
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea4f8b
                    return (eax & 0xff) != 0; // 00ea4f91
                }
                0x00ea_4f92 => {
                    x.push(x.st(0)); // 00ea4f92
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea4f94
                    x.push(x.st(2)); // 00ea4f96
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea4f98
                    x.xch(1); // 00ea4f9a
                    x.set(0, -x.st(0)); // 00ea4f9c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea4f9e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea4fa2
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea4fa6
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea4fa8
                    ah = sw; // 00ea4faa
                    fl.and(u32::from(ah), 5, 8); // 00ea4fac
                    if fl.pf {
                        pc = 0x00ea_5046;
                        continue;
                    } // 00ea4faf
                    pc = 0x00ea_4fb5;
                }
                0x00ea_4fb5 => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea4fb5
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea4fb7
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea4fb9
                    x.xch(1); // 00ea4fbc
                    ecx = 0x0000_0008; // 00ea4fbe
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea4fc3
                    pc = 0x00ea_5058;
                    continue; // 00ea4fc7
                }
                0x00ea_4fcc => {
                    x.push(x.st(3)); // 00ea4fcc
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea4fce
                    x.xch(4); // 00ea4fd0
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000bc), x87_store32(x.st(0)));
                    x.pop(); // 00ea4fd2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000bc)))); // 00ea4fd9
                    sw = fcom(x.st(0), x.st(4)); // 00ea4fe0
                    ah = sw; // 00ea4fe2
                    fl.and(u32::from(ah), 5, 8); // 00ea4fe4
                    if fl.pf {
                        pc = 0x00ea_504c;
                        continue;
                    } // 00ea4fe7
                    pc = 0x00ea_4fe9;
                }
                0x00ea_4fe9 => {
                    x.push(x.st(1)); // 00ea4fe9
                    x.set(6, x87_mul(x.st(6), x.st(0)));
                    x.pop(); // 00ea4feb
                    x.push(x.st(0)); // 00ea4fed
                    x.set(6, x87_add(x.st(6), x.st(0)));
                    x.pop(); // 00ea4fef
                    x.xch(5); // 00ea4ff1
                    sw = fcom(x.st(0), x.st(4));
                    x.pop(); // 00ea4ff3
                    ah = sw; // 00ea4ff5
                    x.set(3, x.st(0));
                    x.pop(); // 00ea4ff7
                    fl.and(u32::from(ah), 5, 8); // 00ea4ff9
                    if !fl.pf {
                        pc = 0x00ea_4ef5;
                        continue;
                    } // 00ea4ffc
                    pc = 0x00ea_5002;
                }
                0x00ea_5002 => {
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea5002
                    x.push(x.st(1)); // 00ea5004
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea5006
                    x.xch(2); // 00ea5008
                    x.set(0, -x.st(0)); // 00ea500a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea500c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5010
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5014
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5016
                    ah = sw; // 00ea5018
                    fl.and(u32::from(ah), 5, 8); // 00ea501a
                    if fl.pf {
                        pc = 0x00ea_5056;
                        continue;
                    } // 00ea501d
                    pc = 0x00ea_501f;
                }
                0x00ea_501f => {
                    ecx = 0x0000_0008; // 00ea501f
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5024
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5026
                    ebp = ecx.wrapping_sub(0x00000007); // 00ea5029
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea502c
                    pc = 0x00ea_5058;
                    continue; // 00ea5030
                }
                0x00ea_5032 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5032
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5034
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5036
                    pc = 0x00ea_5058;
                    continue; // 00ea5038
                }
                0x00ea_503a => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea503a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea503c
                    x.set(3, x.st(0));
                    x.pop(); // 00ea503e
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5040
                    x.xch(1); // 00ea5042
                    pc = 0x00ea_5058;
                    continue; // 00ea5044
                }
                0x00ea_5046 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5046
                    x.xch(1); // 00ea5048
                    pc = 0x00ea_5058;
                    continue; // 00ea504a
                }
                0x00ea_504c => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea504c
                    x.set(3, x.st(0));
                    x.pop(); // 00ea504e
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5050
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5052
                    pc = 0x00ea_5058;
                    continue; // 00ea5054
                }
                0x00ea_5056 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5056
                    pc = 0x00ea_5058;
                }
                0x00ea_5058 => {
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea5058
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000058)))); // 00ea505b
                    x.push(x.st(0)); // 00ea505f
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5061
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea5063
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea5066
                    x.push(x.st(0)); // 00ea506a
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea506c
                    x.xch(3); // 00ea506e
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea5070
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000003c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5072
                    eax = e.mem.u32(esp.wrapping_add(0x0000003c)); // 00ea5076
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea507a
                    e.mem.set_u32(esp.wrapping_add(0x00000128), eax); // 00ea507e
                    x.push(x.st(0)); // 00ea5085
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000014)))),
                    ); // 00ea5087
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea508a
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea508d
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea508f
                    x.xch(1); // 00ea5091
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000040), x87_store32(x.st(0)));
                    x.pop(); // 00ea5093
                    eax = e.mem.u32(esp.wrapping_add(0x00000040)); // 00ea5097
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea509b
                    e.mem.set_u32(esp.wrapping_add(0x0000012c), eax); // 00ea509e
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea50a5
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000010)))),
                    ); // 00ea50a7
                    x.set(1, x87_sub(x.st(1), x.st(0)));
                    x.pop(); // 00ea50aa
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000044), x87_store32(x.st(0)));
                    x.pop(); // 00ea50ac
                    eax = e.mem.u32(esp.wrapping_add(0x00000044)); // 00ea50b0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000040)))); // 00ea50b4
                    e.mem.set_u32(esp.wrapping_add(0x00000130), eax); // 00ea50b8
                    x.push(x.st(0)); // 00ea50bf
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea50c1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea50c5
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000003c)))); // 00ea50c9
                    x.push(x.st(0)); // 00ea50cd
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea50cf
                    x.xch(2); // 00ea50d1
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea50d3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000044)))); // 00ea50d5
                    x.push(x.st(0)); // 00ea50d9
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea50db
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea50df
                    x.xch(1); // 00ea50e1
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea50e3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea50e7
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea50ee
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea50f0
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea50f7
                    x.xch(2); // 00ea50f9
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea50fb
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea50fd
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5104
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5106
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5108
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000098)))); // 00ea510c
                    x.set(0, x.st(0).abs()); // 00ea5113
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000bc), x87_store32(x.st(0)));
                    x.pop(); // 00ea5115
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000bc)))); // 00ea511c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea5123
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea5126
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b8)))),
                    ); // 00ea5128
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea512f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5131
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5135
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5139
                    sw = fcom(x.st(0), x.st(1)); // 00ea513d
                    ah = sw; // 00ea513f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5141
                    fl.and(u32::from(ah), 5, 8); // 00ea5145
                    if fl.pf {
                        pc = 0x00ea_524c;
                        continue;
                    } // 00ea5148
                    pc = 0x00ea_514e;
                }
                0x00ea_514e => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea514e
                    x.set(0, -x.st(0)); // 00ea5152
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5154
                    x.push(0.0); // 00ea5158
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea515a
                    sw = fcom(x.st(0), x.st(1)); // 00ea515e
                    ah = sw; // 00ea5160
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5162
                    fl.and(u32::from(ah), 1, 8); // 00ea5164
                    if !fl.zf {
                        pc = 0x00ea_51d8;
                        continue;
                    } // 00ea5167
                    pc = 0x00ea_5169;
                }
                0x00ea_5169 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5169
                    x.push(x.st(0)); // 00ea516b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea516d
                    x.push(x.st(0)); // 00ea5174
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5176
                    x.push(x.st(4)); // 00ea5178
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea517a
                    x.xch(1); // 00ea517c
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea517e
                    ah = sw; // 00ea5180
                    fl.and(u32::from(ah), 65, 8); // 00ea5182
                    if !fl.zf {
                        pc = 0x00ea_51a0;
                        continue;
                    } // 00ea5185
                    pc = 0x00ea_5187;
                }
                0x00ea_5187 => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea5187
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea5189
                    x.set(1, x.st(0));
                    x.pop(); // 00ea518a
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea518c
                    x.set(0, x.st(0));
                    x.pop(); // 00ea518d
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea518f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5190
                    let r = fl.xor((eax & 0xff), (eax & 0xff), 8);
                    eax = (eax & 0xffff_ff00) | r; // 00ea5192
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5194
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea5196
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5197
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea5199
                    return (eax & 0xff) != 0; // 00ea519f
                }
                0x00ea_51a0 => {
                    x.push(x.st(3)); // 00ea51a0
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea51a2
                    x.push(x.st(1)); // 00ea51a4
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea51a6
                    x.xch(2); // 00ea51a8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea51aa
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea51ae
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea51b2
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea51b4
                    ah = sw; // 00ea51b6
                    fl.and(u32::from(ah), 5, 8); // 00ea51b8
                    if fl.pf {
                        pc = 0x00ea_543e;
                        continue;
                    } // 00ea51bb
                    pc = 0x00ea_51c1;
                }
                0x00ea_51c1 => {
                    edi = 0x0000_0001; // 00ea51c1
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea51c6
                    ecx = 0x0000_0009; // 00ea51c8
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea51cd
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea51cf
                    pc = 0x00ea_5352;
                    continue; // 00ea51d3
                }
                0x00ea_51d8 => {
                    x.set(0, x87_add(x.st(0), x.st(3))); // 00ea51d8
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b8), x87_store32(x.st(0)));
                    x.pop(); // 00ea51da
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b8)))); // 00ea51e1
                    sw = fcom(x.st(0), x.st(2)); // 00ea51e8
                    ah = sw; // 00ea51ea
                    fl.and(u32::from(ah), 65, 8); // 00ea51ec
                    if !fl.zf {
                        pc = 0x00ea_542c;
                        continue;
                    } // 00ea51ef
                    pc = 0x00ea_51f5;
                }
                0x00ea_51f5 => {
                    x.push(x.st(1)); // 00ea51f5
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea51f7
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea51fe
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5200
                    ah = sw; // 00ea5202
                    fl.and(u32::from(ah), 65, 8); // 00ea5204
                    if fl.zf {
                        pc = 0x00ea_5187;
                        continue;
                    } // 00ea5207
                    pc = 0x00ea_520d;
                }
                0x00ea_520d => {
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea520d
                    x.push(x.st(0)); // 00ea520f
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea5211
                    x.xch(1); // 00ea5213
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5215
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5219
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea521d
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea521f
                    ah = sw; // 00ea5221
                    fl.and(u32::from(ah), 5, 8); // 00ea5223
                    if fl.pf {
                        pc = 0x00ea_5445;
                        continue;
                    } // 00ea5226
                    pc = 0x00ea_522c;
                }
                0x00ea_522c => {
                    ebp = 0x0000_0001; // 00ea522c
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5231
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea5233
                    edi = ebp; // 00ea523a
                    ecx = 0x0000_0009; // 00ea523c
                    x.xch(1); // 00ea5241
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5243
                    pc = 0x00ea_5352;
                    continue; // 00ea5247
                }
                0x00ea_524c => {
                    x.push(x.st(1)); // 00ea524c
                    x.set(0, -x.st(0)); // 00ea524e
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5250
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5254
                    sw = fcom(x.st(0), x.st(3)); // 00ea5258
                    ah = sw; // 00ea525a
                    fl.and(u32::from(ah), 65, 8); // 00ea525c
                    if !fl.zf {
                        pc = 0x00ea_542c;
                        continue;
                    } // 00ea525f
                    pc = 0x00ea_5265;
                }
                0x00ea_5265 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea5265
                    x.set(0, -x.st(0)); // 00ea5269
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea526b
                    x.push(0.0); // 00ea526f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5271
                    sw = fcom(x.st(0), x.st(1)); // 00ea5275
                    ah = sw; // 00ea5277
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5279
                    fl.and(u32::from(ah), 65, 8); // 00ea527b
                    if fl.pf {
                        pc = 0x00ea_52d8;
                        continue;
                    } // 00ea527e
                    pc = 0x00ea_5280;
                }
                0x00ea_5280 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5280
                    x.push(x.st(1)); // 00ea5282
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5284
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea528b
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea528d
                    ah = sw; // 00ea528f
                    fl.and(u32::from(ah), 5, 8); // 00ea5291
                    if !fl.pf {
                        pc = 0x00ea_4a47;
                        continue;
                    } // 00ea5294
                    pc = 0x00ea_529a;
                }
                0x00ea_529a => {
                    x.push(x.st(2)); // 00ea529a
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea529c
                    x.push(x.st(0)); // 00ea529e
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea52a0
                    x.xch(1); // 00ea52a2
                    x.set(0, -x.st(0)); // 00ea52a4
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea52a6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea52aa
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea52ae
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea52b0
                    ah = sw; // 00ea52b2
                    fl.and(u32::from(ah), 5, 8); // 00ea52b4
                    if fl.pf {
                        pc = 0x00ea_5445;
                        continue;
                    } // 00ea52b7
                    pc = 0x00ea_52bd;
                }
                0x00ea_52bd => {
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea52bd
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea52bf
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea52c1
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea52c8
                    ecx = 0x0000_0009; // 00ea52cb
                    x.xch(1); // 00ea52d0
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea52d2
                    pc = 0x00ea_5352;
                    continue; // 00ea52d6
                }
                0x00ea_52d8 => {
                    x.set(5, x.st(0));
                    x.pop(); // 00ea52d8
                    x.push(x.st(3)); // 00ea52da
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea52dc
                    x.xch(4); // 00ea52de
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b8), x87_store32(x.st(0)));
                    x.pop(); // 00ea52e0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b8)))); // 00ea52e7
                    sw = fcom(x.st(0), x.st(4)); // 00ea52ee
                    ah = sw; // 00ea52f0
                    fl.and(u32::from(ah), 5, 8); // 00ea52f2
                    if fl.pf {
                        pc = 0x00ea_5421;
                        continue;
                    } // 00ea52f5
                    pc = 0x00ea_52fb;
                }
                0x00ea_52fb => {
                    x.push(x.st(1)); // 00ea52fb
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea52fd
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea5304
                    sw = fcom(x.st(0), x.st(5));
                    x.pop(); // 00ea5306
                    ah = sw; // 00ea5308
                    x.set(4, x.st(0));
                    x.pop(); // 00ea530a
                    fl.and(u32::from(ah), 5, 8); // 00ea530c
                    if !fl.pf {
                        pc = 0x00ea_49bc;
                        continue;
                    } // 00ea530f
                    pc = 0x00ea_5315;
                }
                0x00ea_5315 => {
                    x.xch(3); // 00ea5315
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5317
                    x.set(0, x87_div(x.st(0), x.st(2))); // 00ea5319
                    x.set(0, -x.st(0)); // 00ea531b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea531d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5321
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5325
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5327
                    ah = sw; // 00ea5329
                    fl.and(u32::from(ah), 5, 8); // 00ea532b
                    if fl.pf {
                        pc = 0x00ea_5425;
                        continue;
                    } // 00ea532e
                    pc = 0x00ea_5334;
                }
                0x00ea_5334 => {
                    ecx = 0x0000_0009; // 00ea5334
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5339
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea533b
                    ebp = ecx.wrapping_sub(0x00000008); // 00ea533e
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5341
                    pc = 0x00ea_5345;
                }
                0x00ea_5345 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea5345
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea534c
                    x.xch(3); // 00ea5350
                    pc = 0x00ea_5352;
                }
                0x00ea_5352 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000098)))); // 00ea5352
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000094)))),
                    ); // 00ea5359
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000009c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5360
                    x.xch(2); // 00ea5367
                    x.set(4, x87_sub(x.st(0), x.st(4)));
                    x.pop(); // 00ea5369
                    x.xch(3); // 00ea536b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000048), x87_store32(x.st(0)));
                    x.pop(); // 00ea536d
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea5371
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5373
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000009c)))); // 00ea5377
                    x.set(0, x.st(0).abs()); // 00ea537e
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b8), x87_store32(x.st(0)));
                    x.pop(); // 00ea5380
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b8)))); // 00ea5387
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea538e
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea5391
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000c4)))),
                    ); // 00ea5393
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea539a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea539c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea53a0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea53a4
                    sw = fcom(x.st(0), x.st(1)); // 00ea53a8
                    ah = sw; // 00ea53aa
                    fl.and(u32::from(ah), 5, 8); // 00ea53ac
                    if fl.pf {
                        pc = 0x00ea_54bf;
                        continue;
                    } // 00ea53af
                    pc = 0x00ea_53b5;
                }
                0x00ea_53b5 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea53b5
                    x.set(0, -x.st(0)); // 00ea53b9
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea53bb
                    x.push(0.0); // 00ea53bf
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea53c1
                    sw = fcom(x.st(0), x.st(1)); // 00ea53c5
                    ah = sw; // 00ea53c7
                    fl.and(u32::from(ah), 1, 8); // 00ea53c9
                    if !fl.zf {
                        pc = 0x00ea_5455;
                        continue;
                    } // 00ea53cc
                    pc = 0x00ea_53d2;
                }
                0x00ea_53d2 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea53d2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea53d4
                    x.push(x.st(0)); // 00ea53d8
                    x.set(0, x87_mul(x.st(0), x.st(5))); // 00ea53da
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea53dc
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea53de
                    ah = sw; // 00ea53e0
                    fl.and(u32::from(ah), 65, 8); // 00ea53e2
                    if fl.zf {
                        pc = 0x00ea_4b58;
                        continue;
                    } // 00ea53e5
                    pc = 0x00ea_53eb;
                }
                0x00ea_53eb => {
                    x.xch(2); // 00ea53eb
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea53ed
                    x.xch(2); // 00ea53ef
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea53f1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea53f3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea53f7
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea53fb
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea53fd
                    ah = sw; // 00ea53ff
                    fl.and(u32::from(ah), 5, 8); // 00ea5401
                    if fl.pf {
                        pc = 0x00ea_55af;
                        continue;
                    } // 00ea5404
                    pc = 0x00ea_540a;
                }
                0x00ea_540a => {
                    edi = 0x0000_0001; // 00ea540a
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea540f
                    ecx = 0x0000_000a; // 00ea5411
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea5416
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5418
                    pc = 0x00ea_55b1;
                    continue; // 00ea541c
                }
                0x00ea_5421 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5421
                    x.set(3, x.st(0));
                    x.pop(); // 00ea5423
                    pc = 0x00ea_5425;
                }
                0x00ea_5425 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5425
                    pc = 0x00ea_5345;
                    continue; // 00ea5427
                }
                0x00ea_542c => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea542c
                    x.set(1, x.st(0));
                    x.pop(); // 00ea542e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea5430
                    x.xch(1); // 00ea5437
                    pc = 0x00ea_5352;
                    continue; // 00ea5439
                }
                0x00ea_543e => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea543e
                    pc = 0x00ea_5352;
                    continue; // 00ea5440
                }
                0x00ea_5445 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5445
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000001e0)))); // 00ea5447
                    x.xch(1); // 00ea544e
                    pc = 0x00ea_5352;
                    continue; // 00ea5450
                }
                0x00ea_5455 => {
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea5455
                    x.xch(2); // 00ea5457
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c4), x87_store32(x.st(0)));
                    x.pop(); // 00ea5459
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c4)))); // 00ea5460
                    sw = fcom(x.st(0), x.st(1)); // 00ea5467
                    ah = sw; // 00ea5469
                    fl.and(u32::from(ah), 65, 8); // 00ea546b
                    if !fl.zf {
                        pc = 0x00ea_55a7;
                        continue;
                    } // 00ea546e
                    pc = 0x00ea_5474;
                }
                0x00ea_5474 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5474
                    x.push(x.st(0)); // 00ea5478
                    x.set(0, x87_mul(x.st(0), x.st(5))); // 00ea547a
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea547c
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea547e
                    ah = sw; // 00ea5480
                    fl.and(u32::from(ah), 65, 8); // 00ea5482
                    if fl.zf {
                        pc = 0x00ea_4c0b;
                        continue;
                    } // 00ea5485
                    pc = 0x00ea_548b;
                }
                0x00ea_548b => {
                    x.xch(2); // 00ea548b
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea548d
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea548f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5491
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5495
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5499
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea549b
                    ah = sw; // 00ea549d
                    fl.and(u32::from(ah), 5, 8); // 00ea549f
                    if fl.pf {
                        pc = 0x00ea_55af;
                        continue;
                    } // 00ea54a2
                    pc = 0x00ea_54a8;
                }
                0x00ea_54a8 => {
                    ebp = 0x0000_0001; // 00ea54a8
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea54ad
                    edi = ebp; // 00ea54af
                    ecx = 0x0000_000a; // 00ea54b1
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea54b6
                    pc = 0x00ea_55b1;
                    continue; // 00ea54ba
                }
                0x00ea_54bf => {
                    x.push(x.st(0)); // 00ea54bf
                    x.set(0, -x.st(0)); // 00ea54c1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea54c3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea54c7
                    sw = fcom(x.st(0), x.st(2)); // 00ea54cb
                    ah = sw; // 00ea54cd
                    fl.and(u32::from(ah), 65, 8); // 00ea54cf
                    if !fl.zf {
                        pc = 0x00ea_559d;
                        continue;
                    } // 00ea54d2
                    pc = 0x00ea_54d8;
                }
                0x00ea_54d8 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea54d8
                    x.set(0, -x.st(0)); // 00ea54dc
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea54de
                    x.push(0.0); // 00ea54e2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea54e4
                    sw = fcom(x.st(0), x.st(1)); // 00ea54e8
                    ah = sw; // 00ea54ea
                    fl.and(u32::from(ah), 65, 8); // 00ea54ec
                    if fl.pf {
                        pc = 0x00ea_553d;
                        continue;
                    } // 00ea54ef
                    pc = 0x00ea_54f1;
                }
                0x00ea_54f1 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea54f1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea54f3
                    x.push(x.st(0)); // 00ea54f7
                    x.set(0, x87_mul(x.st(0), x.st(6))); // 00ea54f9
                    x.set(0, x87_add(x.st(0), x.st(5))); // 00ea54fb
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea54fd
                    ah = sw; // 00ea54ff
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5501
                    fl.and(u32::from(ah), 5, 8); // 00ea5503
                    if !fl.pf {
                        pc = 0x00ea_4c9f;
                        continue;
                    } // 00ea5506
                    pc = 0x00ea_550c;
                }
                0x00ea_550c => {
                    x.xch(3); // 00ea550c
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea550e
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea5510
                    x.set(0, -x.st(0)); // 00ea5512
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5514
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5518
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea551c
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea551e
                    ah = sw; // 00ea5520
                    fl.and(u32::from(ah), 5, 8); // 00ea5522
                    if fl.pf {
                        pc = 0x00ea_55af;
                        continue;
                    } // 00ea5525
                    pc = 0x00ea_552b;
                }
                0x00ea_552b => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea552b
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea552d
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea552f
                    ecx = 0x0000_000a; // 00ea5532
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5537
                    pc = 0x00ea_55b1;
                    continue; // 00ea553b
                }
                0x00ea_553d => {
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea553d
                    x.xch(3); // 00ea553f
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c4), x87_store32(x.st(0)));
                    x.pop(); // 00ea5541
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c4)))); // 00ea5548
                    sw = fcom(x.st(0), x.st(1)); // 00ea554f
                    ah = sw; // 00ea5551
                    fl.and(u32::from(ah), 5, 8); // 00ea5553
                    if fl.pf {
                        pc = 0x00ea_55ab;
                        continue;
                    } // 00ea5556
                    pc = 0x00ea_5558;
                }
                0x00ea_5558 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5558
                    x.push(x.st(0)); // 00ea555c
                    x.set(0, x87_mul(x.st(0), x.st(6))); // 00ea555e
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea5560
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5562
                    ah = sw; // 00ea5564
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5566
                    fl.and(u32::from(ah), 5, 8); // 00ea5568
                    if !fl.pf {
                        pc = 0x00ea_4d1a;
                        continue;
                    } // 00ea556b
                    pc = 0x00ea_5571;
                }
                0x00ea_5571 => {
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5571
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea5573
                    x.set(0, -x.st(0)); // 00ea5575
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5577
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea557b
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea557f
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5581
                    ah = sw; // 00ea5583
                    fl.and(u32::from(ah), 5, 8); // 00ea5585
                    if fl.pf {
                        pc = 0x00ea_55af;
                        continue;
                    } // 00ea5588
                    pc = 0x00ea_558a;
                }
                0x00ea_558a => {
                    ecx = 0x0000_000a; // 00ea558a
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea558f
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5591
                    ebp = ecx.wrapping_sub(0x00000009); // 00ea5594
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5597
                    pc = 0x00ea_55b1;
                    continue; // 00ea559b
                }
                0x00ea_559d => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea559d
                    x.set(0, x.st(0));
                    x.pop(); // 00ea559f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea55a1
                    x.push(0.0); // 00ea55a3
                    pc = 0x00ea_55b1;
                    continue; // 00ea55a5
                }
                0x00ea_55a7 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea55a7
                    pc = 0x00ea_55af;
                    continue; // 00ea55a9
                }
                0x00ea_55ab => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea55ab
                    x.set(1, x.st(0));
                    x.pop(); // 00ea55ad
                    pc = 0x00ea_55af;
                }
                0x00ea_55af => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea55af
                    pc = 0x00ea_55b1;
                }
                0x00ea_55b1 => {
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea55b1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea55b4
                    x.push(x.st(0)); // 00ea55b8
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea55ba
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000020)))); // 00ea55bc
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea55bf
                    x.push(x.st(0)); // 00ea55c3
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea55c5
                    x.xch(3); // 00ea55c7
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea55c9
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000003c), x87_store32(x.st(0)));
                    x.pop(); // 00ea55cb
                    eax = e.mem.u32(esp.wrapping_add(0x0000003c)); // 00ea55cf
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea55d3
                    e.mem.set_u32(esp.wrapping_add(0x00000140), eax); // 00ea55d7
                    x.push(x.st(0)); // 00ea55de
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea55e0
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea55e3
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea55e6
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea55e8
                    x.xch(1); // 00ea55ea
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000040), x87_store32(x.st(0)));
                    x.pop(); // 00ea55ec
                    eax = e.mem.u32(esp.wrapping_add(0x00000040)); // 00ea55f0
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea55f4
                    e.mem.set_u32(esp.wrapping_add(0x00000144), eax); // 00ea55f7
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea55fe
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x0000001c)))),
                    ); // 00ea5600
                    x.set(1, x87_sub(x.st(1), x.st(0)));
                    x.pop(); // 00ea5603
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000044), x87_store32(x.st(0)));
                    x.pop(); // 00ea5605
                    eax = e.mem.u32(esp.wrapping_add(0x00000044)); // 00ea5609
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000040)))); // 00ea560d
                    e.mem.set_u32(esp.wrapping_add(0x00000148), eax); // 00ea5611
                    x.push(x.st(0)); // 00ea5618
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea561a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea561e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000003c)))); // 00ea5622
                    x.push(x.st(0)); // 00ea5626
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5628
                    x.xch(2); // 00ea562a
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea562c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000044)))); // 00ea562e
                    x.push(x.st(0)); // 00ea5632
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea5634
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5638
                    x.xch(1); // 00ea563a
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c4), x87_store32(x.st(0)));
                    x.pop(); // 00ea563c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea5643
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea564a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea564c
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5653
                    x.xch(2); // 00ea5655
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5657
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea5659
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5660
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5662
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000048), x87_store32(x.st(0)));
                    x.pop(); // 00ea5664
                    x.push(x87_load32(e.mem.u32(edx.wrapping_add(0x00000004)))); // 00ea5668
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000008c)))),
                    ); // 00ea566b
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea5672
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000c0)))),
                    ); // 00ea5674
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea567b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea567d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c4)))); // 00ea5681
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5688
                    sw = fcom(x.st(0), x.st(1)); // 00ea568c
                    ah = sw; // 00ea568e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea5690
                    fl.and(u32::from(ah), 5, 8); // 00ea5694
                    if fl.pf {
                        pc = 0x00ea_576b;
                        continue;
                    } // 00ea5697
                    pc = 0x00ea_569d;
                }
                0x00ea_569d => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea569d
                    sw = fcom(x.st(0), x.st(4)); // 00ea56a1
                    ah = sw; // 00ea56a3
                    x.set(4, x.st(0));
                    x.pop(); // 00ea56a5
                    fl.and(u32::from(ah), 1, 8); // 00ea56a7
                    if !fl.zf {
                        pc = 0x00ea_56fd;
                        continue;
                    } // 00ea56aa
                    pc = 0x00ea_56ac;
                }
                0x00ea_56ac => {
                    x.set(3, x.st(0));
                    x.pop(); // 00ea56ac
                    x.push(x.st(2)); // 00ea56ae
                    x.set(4, x87_mul(x.st(4), x.st(0)));
                    x.pop(); // 00ea56b0
                    x.push(x.st(1)); // 00ea56b2
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea56b4
                    sw = fcom(x.st(0), x.st(3)); // 00ea56b6
                    ah = sw; // 00ea56b8
                    x.set(3, x.st(0));
                    x.pop(); // 00ea56ba
                    fl.and(u32::from(ah), 5, 8); // 00ea56bc
                    if !fl.pf {
                        pc = 0x00ea_4ef7;
                        continue;
                    } // 00ea56bf
                    pc = 0x00ea_56c5;
                }
                0x00ea_56c5 => {
                    x.push(x.st(0)); // 00ea56c5
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea56c7
                    x.push(x.st(1)); // 00ea56c9
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea56cb
                    x.xch(2); // 00ea56cd
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea56cf
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea56d3
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea56d7
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea56d9
                    ah = sw; // 00ea56db
                    fl.and(u32::from(ah), 5, 8); // 00ea56dd
                    if fl.pf {
                        pc = 0x00ea_5872;
                        continue;
                    } // 00ea56e0
                    pc = 0x00ea_56e6;
                }
                0x00ea_56e6 => {
                    edi = 0x0000_0001; // 00ea56e6
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea56eb
                    ecx = 0x0000_000b; // 00ea56ed
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea56f2
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea56f4
                    pc = 0x00ea_5874;
                    continue; // 00ea56f8
                }
                0x00ea_56fd => {
                    x.push(x.st(2)); // 00ea56fd
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea56ff
                    x.xch(3); // 00ea5701
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c0), x87_store32(x.st(0)));
                    x.pop(); // 00ea5703
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c0)))); // 00ea570a
                    sw = fcom(x.st(0), x.st(1)); // 00ea5711
                    ah = sw; // 00ea5713
                    fl.and(u32::from(ah), 65, 8); // 00ea5715
                    if !fl.zf {
                        pc = 0x00ea_584e;
                        continue;
                    } // 00ea5718
                    pc = 0x00ea_571e;
                }
                0x00ea_571e => {
                    x.push(x.st(3)); // 00ea571e
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea5720
                    x.push(x.st(0)); // 00ea5722
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea5724
                    x.xch(4); // 00ea5726
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5728
                    ah = sw; // 00ea572a
                    fl.and(u32::from(ah), 65, 8); // 00ea572c
                    if fl.zf {
                        pc = 0x00ea_4ef5;
                        continue;
                    } // 00ea572f
                    pc = 0x00ea_5735;
                }
                0x00ea_5735 => {
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea5735
                    x.push(x.st(1)); // 00ea5737
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea5739
                    x.xch(2); // 00ea573b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea573d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5741
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5745
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5747
                    ah = sw; // 00ea5749
                    fl.and(u32::from(ah), 5, 8); // 00ea574b
                    if fl.pf {
                        pc = 0x00ea_5872;
                        continue;
                    } // 00ea574e
                    pc = 0x00ea_5754;
                }
                0x00ea_5754 => {
                    ebp = 0x0000_0001; // 00ea5754
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5759
                    edi = ebp; // 00ea575b
                    ecx = 0x0000_000b; // 00ea575d
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5762
                    pc = 0x00ea_5874;
                    continue; // 00ea5766
                }
                0x00ea_576b => {
                    x.push(x.st(1)); // 00ea576b
                    x.set(0, -x.st(0)); // 00ea576d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea576f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5773
                    sw = fcom(x.st(0), x.st(3)); // 00ea5777
                    ah = sw; // 00ea5779
                    fl.and(u32::from(ah), 65, 8); // 00ea577b
                    if !fl.zf {
                        pc = 0x00ea_5856;
                        continue;
                    } // 00ea577e
                    pc = 0x00ea_5784;
                }
                0x00ea_5784 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5784
                    sw = fcom(x.st(0), x.st(5)); // 00ea5788
                    ah = sw; // 00ea578a
                    x.set(5, x.st(0));
                    x.pop(); // 00ea578c
                    fl.and(u32::from(ah), 65, 8); // 00ea578e
                    if fl.pf {
                        pc = 0x00ea_57e8;
                        continue;
                    } // 00ea5791
                    pc = 0x00ea_5793;
                }
                0x00ea_5793 => {
                    x.set(4, x.st(0));
                    x.pop(); // 00ea5793
                    x.push(x.st(0)); // 00ea5795
                    x.set(5, x87_mul(x.st(5), x.st(0)));
                    x.pop(); // 00ea5797
                    x.push(x.st(2)); // 00ea5799
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea579b
                    x.xch(4); // 00ea579d
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea579f
                    ah = sw; // 00ea57a1
                    x.set(2, x.st(0));
                    x.pop(); // 00ea57a3
                    fl.and(u32::from(ah), 5, 8); // 00ea57a5
                    if !fl.pf {
                        pc = 0x00ea_4f7f;
                        continue;
                    } // 00ea57a8
                    pc = 0x00ea_57ae;
                }
                0x00ea_57ae => {
                    x.push(x.st(0)); // 00ea57ae
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea57b0
                    x.push(x.st(2)); // 00ea57b2
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea57b4
                    x.xch(1); // 00ea57b6
                    x.set(0, -x.st(0)); // 00ea57b8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea57ba
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea57be
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea57c2
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea57c4
                    ah = sw; // 00ea57c6
                    fl.and(u32::from(ah), 5, 8); // 00ea57c8
                    if fl.pf {
                        pc = 0x00ea_5862;
                        continue;
                    } // 00ea57cb
                    pc = 0x00ea_57d1;
                }
                0x00ea_57d1 => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea57d1
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea57d3
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea57d5
                    x.xch(1); // 00ea57d8
                    ecx = 0x0000_000b; // 00ea57da
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea57df
                    pc = 0x00ea_5874;
                    continue; // 00ea57e3
                }
                0x00ea_57e8 => {
                    x.push(x.st(3)); // 00ea57e8
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea57ea
                    x.xch(4); // 00ea57ec
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c0), x87_store32(x.st(0)));
                    x.pop(); // 00ea57ee
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c0)))); // 00ea57f5
                    sw = fcom(x.st(0), x.st(4)); // 00ea57fc
                    ah = sw; // 00ea57fe
                    fl.and(u32::from(ah), 5, 8); // 00ea5800
                    if fl.pf {
                        pc = 0x00ea_5868;
                        continue;
                    } // 00ea5803
                    pc = 0x00ea_5805;
                }
                0x00ea_5805 => {
                    x.push(x.st(1)); // 00ea5805
                    x.set(6, x87_mul(x.st(6), x.st(0)));
                    x.pop(); // 00ea5807
                    x.push(x.st(0)); // 00ea5809
                    x.set(6, x87_add(x.st(6), x.st(0)));
                    x.pop(); // 00ea580b
                    x.xch(5); // 00ea580d
                    sw = fcom(x.st(0), x.st(4));
                    x.pop(); // 00ea580f
                    ah = sw; // 00ea5811
                    x.set(3, x.st(0));
                    x.pop(); // 00ea5813
                    fl.and(u32::from(ah), 5, 8); // 00ea5815
                    if !fl.pf {
                        pc = 0x00ea_4ef5;
                        continue;
                    } // 00ea5818
                    pc = 0x00ea_581e;
                }
                0x00ea_581e => {
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea581e
                    x.push(x.st(1)); // 00ea5820
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea5822
                    x.xch(2); // 00ea5824
                    x.set(0, -x.st(0)); // 00ea5826
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5828
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea582c
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5830
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5832
                    ah = sw; // 00ea5834
                    fl.and(u32::from(ah), 5, 8); // 00ea5836
                    if fl.pf {
                        pc = 0x00ea_5872;
                        continue;
                    } // 00ea5839
                    pc = 0x00ea_583b;
                }
                0x00ea_583b => {
                    ecx = 0x0000_000b; // 00ea583b
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5840
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5842
                    ebp = ecx.wrapping_sub(0x0000000a); // 00ea5845
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5848
                    pc = 0x00ea_5874;
                    continue; // 00ea584c
                }
                0x00ea_584e => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea584e
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5850
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5852
                    pc = 0x00ea_5874;
                    continue; // 00ea5854
                }
                0x00ea_5856 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5856
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5858
                    x.set(3, x.st(0));
                    x.pop(); // 00ea585a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea585c
                    x.xch(1); // 00ea585e
                    pc = 0x00ea_5874;
                    continue; // 00ea5860
                }
                0x00ea_5862 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5862
                    x.xch(1); // 00ea5864
                    pc = 0x00ea_5874;
                    continue; // 00ea5866
                }
                0x00ea_5868 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5868
                    x.set(3, x.st(0));
                    x.pop(); // 00ea586a
                    x.set(0, x.st(0));
                    x.pop(); // 00ea586c
                    x.set(2, x.st(0));
                    x.pop(); // 00ea586e
                    pc = 0x00ea_5874;
                    continue; // 00ea5870
                }
                0x00ea_5872 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5872
                    pc = 0x00ea_5874;
                }
                0x00ea_5874 => {
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea5874
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000058)))); // 00ea5877
                    x.push(x.st(0)); // 00ea587b
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea587d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea587f
                    x.push(x.st(0)); // 00ea5883
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea5885
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea5888
                    x.xch(2); // 00ea588a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea588c
                    eax = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea5890
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea5894
                    e.mem.set_u32(esp.wrapping_add(0x0000003c), eax); // 00ea5898
                    x.push(x.st(0)); // 00ea589c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea589e
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea58a1
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea58a4
                    x.set(2, x87_sub(x.st(0), x.st(2)));
                    x.pop(); // 00ea58a6
                    x.xch(1); // 00ea58a8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000078), x87_store32(x.st(0)));
                    x.pop(); // 00ea58aa
                    eax = e.mem.u32(esp.wrapping_add(0x00000078)); // 00ea58ae
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea58b2
                    e.mem.set_u32(esp.wrapping_add(0x00000040), eax); // 00ea58b5
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea58b9
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x0000001c)))),
                    ); // 00ea58bb
                    x.set(1, x87_sub(x.st(1), x.st(0)));
                    x.pop(); // 00ea58be
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea58c0
                    eax = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea58c4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000078)))); // 00ea58c8
                    e.mem.set_u32(esp.wrapping_add(0x00000044), eax); // 00ea58cc
                    x.push(x.st(0)); // 00ea58d0
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000060)))),
                    ); // 00ea58d2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea58d6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000074)))); // 00ea58da
                    x.push(x.st(0)); // 00ea58de
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea58e0
                    x.xch(2); // 00ea58e2
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea58e4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000007c)))); // 00ea58e6
                    x.push(x.st(0)); // 00ea58ea
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000064)))),
                    ); // 00ea58ec
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea58f0
                    x.xch(1); // 00ea58f2
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000c0), x87_store32(x.st(0)));
                    x.pop(); // 00ea58f4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000084)))); // 00ea58fb
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea5902
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000080)))); // 00ea5904
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea590b
                    x.xch(2); // 00ea590d
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea590f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000088)))); // 00ea5911
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5918
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea591a
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000008c), x87_store32(x.st(0)));
                    x.pop(); // 00ea591c
                    x.push(x87_load32(e.mem.u32(edx.wrapping_add(0x00000004)))); // 00ea5923
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000bc)))),
                    ); // 00ea5926
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea592d
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000004c)))),
                    ); // 00ea592f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5933
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5935
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000c0)))); // 00ea5939
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5940
                    sw = fcom(x.st(0), x.st(1)); // 00ea5944
                    ah = sw; // 00ea5946
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000008c)))); // 00ea5948
                    fl.and(u32::from(ah), 5, 8); // 00ea594f
                    if fl.pf {
                        pc = 0x00ea_5a2e;
                        continue;
                    } // 00ea5952
                    pc = 0x00ea_5958;
                }
                0x00ea_5958 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5958
                    x.set(0, -x.st(0)); // 00ea595c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea595e
                    x.push(0.0); // 00ea5962
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5964
                    sw = fcom(x.st(0), x.st(1)); // 00ea5968
                    ah = sw; // 00ea596a
                    fl.and(u32::from(ah), 1, 8); // 00ea596c
                    if !fl.zf {
                        pc = 0x00ea_59c3;
                        continue;
                    } // 00ea596f
                    pc = 0x00ea_5971;
                }
                0x00ea_5971 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5971
                    x.push(x.st(1)); // 00ea5973
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5975
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea597c
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea597e
                    ah = sw; // 00ea5980
                    fl.and(u32::from(ah), 65, 8); // 00ea5982
                    if fl.zf {
                        pc = 0x00ea_5187;
                        continue;
                    } // 00ea5985
                    pc = 0x00ea_598b;
                }
                0x00ea_598b => {
                    x.push(x.st(3)); // 00ea598b
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea598d
                    x.push(x.st(1)); // 00ea598f
                    x.set(3, x87_div(x.st(3), x.st(0)));
                    x.pop(); // 00ea5991
                    x.xch(2); // 00ea5993
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5995
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5999
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea599d
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea599f
                    ah = sw; // 00ea59a1
                    fl.and(u32::from(ah), 5, 8); // 00ea59a3
                    if fl.pf {
                        pc = 0x00ea_5be4;
                        continue;
                    } // 00ea59a6
                    pc = 0x00ea_59ac;
                }
                0x00ea_59ac => {
                    edi = 0x0000_0001; // 00ea59ac
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea59b1
                    ecx = 0x0000_000c; // 00ea59b3
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea59b8
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea59ba
                    pc = 0x00ea_5b24;
                    continue; // 00ea59be
                }
                0x00ea_59c3 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea59c3
                    x.set(0, x87_add(x.st(0), x.st(3))); // 00ea59c5
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea59c7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea59cb
                    sw = fcom(x.st(0), x.st(2)); // 00ea59cf
                    ah = sw; // 00ea59d1
                    fl.and(u32::from(ah), 65, 8); // 00ea59d3
                    if !fl.zf {
                        pc = 0x00ea_5bd7;
                        continue;
                    } // 00ea59d6
                    pc = 0x00ea_59dc;
                }
                0x00ea_59dc => {
                    x.push(x.st(1)); // 00ea59dc
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea59de
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea59e5
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea59e7
                    ah = sw; // 00ea59e9
                    fl.and(u32::from(ah), 65, 8); // 00ea59eb
                    if fl.zf {
                        pc = 0x00ea_5187;
                        continue;
                    } // 00ea59ee
                    pc = 0x00ea_59f4;
                }
                0x00ea_59f4 => {
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea59f4
                    x.push(x.st(0)); // 00ea59f6
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea59f8
                    x.xch(1); // 00ea59fa
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea59fc
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5a00
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5a04
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5a06
                    ah = sw; // 00ea5a08
                    fl.and(u32::from(ah), 5, 8); // 00ea5a0a
                    if fl.pf {
                        pc = 0x00ea_5beb;
                        continue;
                    } // 00ea5a0d
                    pc = 0x00ea_5a13;
                }
                0x00ea_5a13 => {
                    ebp = 0x0000_0001; // 00ea5a13
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5a18
                    x.push(0.0); // 00ea5a1a
                    edi = ebp; // 00ea5a1c
                    ecx = 0x0000_000c; // 00ea5a1e
                    x.xch(1); // 00ea5a23
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5a25
                    pc = 0x00ea_5b24;
                    continue; // 00ea5a29
                }
                0x00ea_5a2e => {
                    x.push(x.st(1)); // 00ea5a2e
                    x.set(0, -x.st(0)); // 00ea5a30
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5a32
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5a36
                    sw = fcom(x.st(0), x.st(3)); // 00ea5a3a
                    ah = sw; // 00ea5a3c
                    fl.and(u32::from(ah), 65, 8); // 00ea5a3e
                    if !fl.zf {
                        pc = 0x00ea_5bd7;
                        continue;
                    } // 00ea5a41
                    pc = 0x00ea_5a47;
                }
                0x00ea_5a47 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5a47
                    x.set(0, -x.st(0)); // 00ea5a4b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5a4d
                    x.push(0.0); // 00ea5a51
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5a53
                    sw = fcom(x.st(0), x.st(1)); // 00ea5a57
                    ah = sw; // 00ea5a59
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5a5b
                    fl.and(u32::from(ah), 65, 8); // 00ea5a5d
                    if fl.pf {
                        pc = 0x00ea_5ab5;
                        continue;
                    } // 00ea5a60
                    pc = 0x00ea_5a62;
                }
                0x00ea_5a62 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5a62
                    x.push(x.st(1)); // 00ea5a64
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5a66
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea5a6d
                    sw = fcom(x.st(0), x.st(1));
                    x.pop();
                    x.pop(); // 00ea5a6f
                    ah = sw; // 00ea5a71
                    fl.and(u32::from(ah), 5, 8); // 00ea5a73
                    if !fl.pf {
                        pc = 0x00ea_4a47;
                        continue;
                    } // 00ea5a76
                    pc = 0x00ea_5a7c;
                }
                0x00ea_5a7c => {
                    x.push(x.st(2)); // 00ea5a7c
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5a7e
                    x.push(x.st(0)); // 00ea5a80
                    x.set(2, x87_div(x.st(2), x.st(0)));
                    x.pop(); // 00ea5a82
                    x.xch(1); // 00ea5a84
                    x.set(0, -x.st(0)); // 00ea5a86
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5a88
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5a8c
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5a90
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5a92
                    ah = sw; // 00ea5a94
                    fl.and(u32::from(ah), 5, 8); // 00ea5a96
                    if fl.pf {
                        pc = 0x00ea_5beb;
                        continue;
                    } // 00ea5a99
                    pc = 0x00ea_5a9f;
                }
                0x00ea_5a9f => {
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5a9f
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea5aa1
                    x.push(0.0); // 00ea5aa3
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5aa5
                    ecx = 0x0000_000c; // 00ea5aa8
                    x.xch(1); // 00ea5aad
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5aaf
                    pc = 0x00ea_5b24;
                    continue; // 00ea5ab3
                }
                0x00ea_5ab5 => {
                    x.set(5, x.st(0));
                    x.pop(); // 00ea5ab5
                    x.push(x.st(3)); // 00ea5ab7
                    x.set(5, x87_add(x.st(5), x.st(0)));
                    x.pop(); // 00ea5ab9
                    x.xch(4); // 00ea5abb
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5abd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea5ac1
                    sw = fcom(x.st(0), x.st(4)); // 00ea5ac5
                    ah = sw; // 00ea5ac7
                    fl.and(u32::from(ah), 5, 8); // 00ea5ac9
                    if fl.pf {
                        pc = 0x00ea_5bcc;
                        continue;
                    } // 00ea5acc
                    pc = 0x00ea_5ad2;
                }
                0x00ea_5ad2 => {
                    x.push(x.st(1)); // 00ea5ad2
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5ad4
                    x.set(0, x87_add(x.st(0), x.st(1))); // 00ea5adb
                    sw = fcom(x.st(0), x.st(5));
                    x.pop(); // 00ea5add
                    ah = sw; // 00ea5adf
                    x.set(4, x.st(0));
                    x.pop(); // 00ea5ae1
                    fl.and(u32::from(ah), 5, 8); // 00ea5ae3
                    if !fl.pf {
                        pc = 0x00ea_49bc;
                        continue;
                    } // 00ea5ae6
                    pc = 0x00ea_5aec;
                }
                0x00ea_5aec => {
                    x.xch(3); // 00ea5aec
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5aee
                    x.set(0, x87_div(x.st(0), x.st(2))); // 00ea5af0
                    x.set(0, -x.st(0)); // 00ea5af2
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5af4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5af8
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5afc
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5afe
                    ah = sw; // 00ea5b00
                    fl.and(u32::from(ah), 5, 8); // 00ea5b02
                    if fl.pf {
                        pc = 0x00ea_5bd0;
                        continue;
                    } // 00ea5b05
                    pc = 0x00ea_5b0b;
                }
                0x00ea_5b0b => {
                    ecx = 0x0000_000c; // 00ea5b0b
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5b10
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5b12
                    ebp = ecx.wrapping_sub(0x0000000b); // 00ea5b15
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5b18
                    pc = 0x00ea_5b1c;
                }
                0x00ea_5b1c => {
                    x.push(0.0); // 00ea5b1c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000048)))); // 00ea5b1e
                    x.xch(3); // 00ea5b22
                    pc = 0x00ea_5b24;
                }
                0x00ea_5b24 => {
                    x.xch(2); // 00ea5b24
                    x.set(4, x87_sub(x.st(0), x.st(4)));
                    x.pop(); // 00ea5b26
                    x.xch(3); // 00ea5b28
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000004c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5b2a
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea5b2e
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000002c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5b30
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b8)))); // 00ea5b34
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea5b3b
                    x.push(x87_load32(e.mem.u32(edx))); // 00ea5b3e
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000030)))),
                    ); // 00ea5b40
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5b44
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5b46
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000004c)))); // 00ea5b4a
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5b4e
                    sw = fcom(x.st(0), x.st(1)); // 00ea5b52
                    ah = sw; // 00ea5b54
                    fl.and(u32::from(ah), 5, 8); // 00ea5b56
                    if fl.pf {
                        pc = 0x00ea_5c5f;
                        continue;
                    } // 00ea5b59
                    pc = 0x00ea_5b5f;
                }
                0x00ea_5b5f => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5b5f
                    x.push(x.st(0)); // 00ea5b63
                    x.set(0, -x.st(0)); // 00ea5b65
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5b67
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5b6b
                    sw = fcom(x.st(0), x.st(4)); // 00ea5b6f
                    ah = sw; // 00ea5b71
                    fl.and(u32::from(ah), 1, 8); // 00ea5b73
                    if !fl.zf {
                        pc = 0x00ea_5bf6;
                        continue;
                    } // 00ea5b76
                    pc = 0x00ea_5b78;
                }
                0x00ea_5b78 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5b78
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5b7a
                    x.push(x.st(0)); // 00ea5b7e
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5b80
                    x.set(0, x87_add(x.st(0), x.st(4))); // 00ea5b87
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5b89
                    ah = sw; // 00ea5b8b
                    fl.and(u32::from(ah), 65, 8); // 00ea5b8d
                    if fl.zf {
                        pc = 0x00ea_4b58;
                        continue;
                    } // 00ea5b90
                    pc = 0x00ea_5b96;
                }
                0x00ea_5b96 => {
                    x.xch(2); // 00ea5b96
                    x.set(3, x87_sub(x.st(0), x.st(3)));
                    x.pop(); // 00ea5b98
                    x.xch(2); // 00ea5b9a
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea5b9c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5b9e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5ba2
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5ba6
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5ba8
                    ah = sw; // 00ea5baa
                    fl.and(u32::from(ah), 5, 8); // 00ea5bac
                    if fl.pf {
                        pc = 0x00ea_5d55;
                        continue;
                    } // 00ea5baf
                    pc = 0x00ea_5bb5;
                }
                0x00ea_5bb5 => {
                    edi = 0x0000_0001; // 00ea5bb5
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5bba
                    ecx = 0x0000_000d; // 00ea5bbc
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea5bc1
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5bc3
                    pc = 0x00ea_5d57;
                    continue; // 00ea5bc7
                }
                0x00ea_5bcc => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5bcc
                    x.set(3, x.st(0));
                    x.pop(); // 00ea5bce
                    pc = 0x00ea_5bd0;
                }
                0x00ea_5bd0 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5bd0
                    pc = 0x00ea_5b1c;
                    continue; // 00ea5bd2
                }
                0x00ea_5bd7 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5bd7
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5bd9
                    x.push(0.0); // 00ea5bdb
                    x.xch(1); // 00ea5bdd
                    pc = 0x00ea_5b24;
                    continue; // 00ea5bdf
                }
                0x00ea_5be4 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5be4
                    pc = 0x00ea_5b24;
                    continue; // 00ea5be6
                }
                0x00ea_5beb => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5beb
                    x.push(0.0); // 00ea5bed
                    x.xch(1); // 00ea5bef
                    pc = 0x00ea_5b24;
                    continue; // 00ea5bf1
                }
                0x00ea_5bf6 => {
                    x.set(3, x87_add(x.st(3), x.st(0)));
                    x.pop(); // 00ea5bf6
                    x.xch(2); // 00ea5bf8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea5bfa
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea5bfe
                    sw = fcom(x.st(0), x.st(1)); // 00ea5c02
                    ah = sw; // 00ea5c04
                    fl.and(u32::from(ah), 65, 8); // 00ea5c06
                    if !fl.zf {
                        pc = 0x00ea_5d4d;
                        continue;
                    } // 00ea5c09
                    pc = 0x00ea_5c0f;
                }
                0x00ea_5c0f => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5c0f
                    x.push(x.st(0)); // 00ea5c13
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5c15
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea5c1c
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5c1e
                    ah = sw; // 00ea5c20
                    fl.and(u32::from(ah), 65, 8); // 00ea5c22
                    if fl.zf {
                        pc = 0x00ea_4c0b;
                        continue;
                    } // 00ea5c25
                    pc = 0x00ea_5c2b;
                }
                0x00ea_5c2b => {
                    x.xch(2); // 00ea5c2b
                    x.set(1, x87_sub(x.st(0), x.st(1)));
                    x.pop(); // 00ea5c2d
                    x.set(1, x87_div(x.st(0), x.st(1)));
                    x.pop(); // 00ea5c2f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5c31
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5c35
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5c39
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5c3b
                    ah = sw; // 00ea5c3d
                    fl.and(u32::from(ah), 5, 8); // 00ea5c3f
                    if fl.pf {
                        pc = 0x00ea_5d55;
                        continue;
                    } // 00ea5c42
                    pc = 0x00ea_5c48;
                }
                0x00ea_5c48 => {
                    ebp = 0x0000_0001; // 00ea5c48
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5c4d
                    edi = ebp; // 00ea5c4f
                    ecx = 0x0000_000d; // 00ea5c51
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5c56
                    pc = 0x00ea_5d57;
                    continue; // 00ea5c5a
                }
                0x00ea_5c5f => {
                    x.push(x.st(0)); // 00ea5c5f
                    x.set(0, -x.st(0)); // 00ea5c61
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000018), x87_store32(x.st(0)));
                    x.pop(); // 00ea5c63
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000018)))); // 00ea5c67
                    sw = fcom(x.st(0), x.st(2)); // 00ea5c6b
                    ah = sw; // 00ea5c6d
                    fl.and(u32::from(ah), 65, 8); // 00ea5c6f
                    if !fl.zf {
                        pc = 0x00ea_5d41;
                        continue;
                    } // 00ea5c72
                    pc = 0x00ea_5c78;
                }
                0x00ea_5c78 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5c78
                    x.push(x.st(0)); // 00ea5c7c
                    x.set(0, -x.st(0)); // 00ea5c7e
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5c80
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5c84
                    sw = fcom(x.st(0), x.st(5)); // 00ea5c88
                    ah = sw; // 00ea5c8a
                    fl.and(u32::from(ah), 65, 8); // 00ea5c8c
                    if fl.pf {
                        pc = 0x00ea_5ce2;
                        continue;
                    } // 00ea5c8f
                    pc = 0x00ea_5c91;
                }
                0x00ea_5c91 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5c91
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5c93
                    x.push(x.st(0)); // 00ea5c97
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5c99
                    x.set(0, x87_add(x.st(0), x.st(5))); // 00ea5ca0
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5ca2
                    ah = sw; // 00ea5ca4
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5ca6
                    fl.and(u32::from(ah), 5, 8); // 00ea5ca8
                    if !fl.pf {
                        pc = 0x00ea_4c9f;
                        continue;
                    } // 00ea5cab
                    pc = 0x00ea_5cb1;
                }
                0x00ea_5cb1 => {
                    x.xch(3); // 00ea5cb1
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5cb3
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea5cb5
                    x.set(0, -x.st(0)); // 00ea5cb7
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5cb9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5cbd
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5cc1
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5cc3
                    ah = sw; // 00ea5cc5
                    fl.and(u32::from(ah), 5, 8); // 00ea5cc7
                    if fl.pf {
                        pc = 0x00ea_5d55;
                        continue;
                    } // 00ea5cca
                    pc = 0x00ea_5cd0;
                }
                0x00ea_5cd0 => {
                    let r = fl.xor(ebp, ebp, 32);
                    ebp = r; // 00ea5cd0
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5cd2
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5cd4
                    ecx = 0x0000_000d; // 00ea5cd7
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5cdc
                    pc = 0x00ea_5d57;
                    continue; // 00ea5ce0
                }
                0x00ea_5ce2 => {
                    x.set(4, x87_add(x.st(4), x.st(0)));
                    x.pop(); // 00ea5ce2
                    x.xch(3); // 00ea5ce4
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000030), x87_store32(x.st(0)));
                    x.pop(); // 00ea5ce6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000030)))); // 00ea5cea
                    sw = fcom(x.st(0), x.st(1)); // 00ea5cee
                    ah = sw; // 00ea5cf0
                    fl.and(u32::from(ah), 5, 8); // 00ea5cf2
                    if fl.pf {
                        pc = 0x00ea_5d51;
                        continue;
                    } // 00ea5cf5
                    pc = 0x00ea_5cf7;
                }
                0x00ea_5cf7 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000002c)))); // 00ea5cf7
                    x.push(x.st(0)); // 00ea5cfb
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000001e0)))),
                    ); // 00ea5cfd
                    x.set(0, x87_add(x.st(0), x.st(2))); // 00ea5d04
                    sw = fcom(x.st(0), x.st(3));
                    x.pop(); // 00ea5d06
                    ah = sw; // 00ea5d08
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5d0a
                    fl.and(u32::from(ah), 5, 8); // 00ea5d0c
                    if !fl.pf {
                        pc = 0x00ea_4d1a;
                        continue;
                    } // 00ea5d0f
                    pc = 0x00ea_5d15;
                }
                0x00ea_5d15 => {
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5d15
                    x.set(1, x87_div(x.st(1), x.st(0)));
                    x.pop(); // 00ea5d17
                    x.set(0, -x.st(0)); // 00ea5d19
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000010), x87_store32(x.st(0)));
                    x.pop(); // 00ea5d1b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000010)))); // 00ea5d1f
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5d23
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5d25
                    ah = sw; // 00ea5d27
                    fl.and(u32::from(ah), 5, 8); // 00ea5d29
                    if fl.pf {
                        pc = 0x00ea_5d55;
                        continue;
                    } // 00ea5d2c
                    pc = 0x00ea_5d2e;
                }
                0x00ea_5d2e => {
                    ecx = 0x0000_000d; // 00ea5d2e
                    e.mem.set_u32(esi, x87_store32(x.st(0)));
                    x.pop(); // 00ea5d33
                    let r = fl.or(edi, 0xffff_ffff, 32);
                    edi = r; // 00ea5d35
                    ebp = ecx.wrapping_sub(0x0000000c); // 00ea5d38
                    e.mem.set_u32(esp.wrapping_add(0x00000014), edi); // 00ea5d3b
                    pc = 0x00ea_5d57;
                    continue; // 00ea5d3f
                }
                0x00ea_5d41 => {
                    x.set(2, x.st(0));
                    x.pop(); // 00ea5d41
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d43
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d45
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000070)))); // 00ea5d47
                    pc = 0x00ea_5d57;
                    continue; // 00ea5d4b
                }
                0x00ea_5d4d => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d4d
                    pc = 0x00ea_5d55;
                    continue; // 00ea5d4f
                }
                0x00ea_5d51 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d51
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5d53
                    pc = 0x00ea_5d55;
                }
                0x00ea_5d55 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d55
                    pc = 0x00ea_5d57;
                }
                0x00ea_5d57 => {
                    let r = fl.sub(ecx, 0x0000_000d, 32); // 00ea5d57
                    if !fl.cf && !fl.zf {
                        pc = 0x00ea_73d8;
                        continue;
                    } // 00ea5d5a
                    pc = 0x00ea_5d60;
                }
                0x00ea_5d60 => {
                    pc = [
                        0x00ea_5d67,
                        0x00ea_5e01,
                        0x00ea_5f42,
                        0x00ea_5f42,
                        0x00ea_5f42,
                        0x00ea_6035,
                        0x00ea_625d,
                        0x00ea_63fd,
                        0x00ea_666c,
                        0x00ea_6893,
                        0x00ea_6ab7,
                        0x00ea_6d26,
                        0x00ea_6f51,
                        0x00ea_7169,
                    ][ecx as usize]; // 00ea5d60 (jump table 00ea7454)
                    continue; // 00ea5d60
                }
                0x00ea_5d67 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea5d67
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5d6e
                    eax = e.mem.u32(esp.wrapping_add(0x000001f4)); // 00ea5d70
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5d77
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5d79
                    edx = esp.wrapping_add(0x00000078); // 00ea5d7a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5d7e
                    edx = e.mem.u32(esp.wrapping_add(0x000001f8)); // 00ea5d7f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5d86
                    ecx = esp.wrapping_add(0x00000048); // 00ea5d87
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5d8b
                    ecx = e.mem.u32(esp.wrapping_add(0x000001fc)); // 00ea5d8c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5d93
                    eax = esp.wrapping_add(0x0000013c); // 00ea5d94
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5d9b
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea5d9c
                    ecx = eax; // 00ea5da1
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea5da3
                    x.push(x87_load32(e.mem.u32(0x011c_1340))); // 00ea5da8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5dae
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5daf
                    ecx = esp.wrapping_add(0x0000012c); // 00ea5db0
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea5db7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5dba
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea5dbb
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea5dc0
                    ecx = eax; // 00ea5dc3
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea5dc5
                    x.push(x87_load32(e.mem.u32(0x0101_6248))); // 00ea5dca
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5dd0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5dd1
                    edx = esp.wrapping_add(0x00000148); // 00ea5dd2
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea5dd9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5ddc
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea5ddd
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea5de2
                    pc = 0x00ea_5de5;
                }
                0x00ea_5de5 => {
                    edx = e.mem.u32(eax); // 00ea5de5
                    ecx = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea5de7
                    e.mem.set_u32(ecx, edx); // 00ea5dee
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea5df0
                    e.mem.set_u32(ecx.wrapping_add(0x00000004), edx); // 00ea5df3
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea5df6
                    e.mem.set_u32(ecx.wrapping_add(0x00000008), eax); // 00ea5df9
                    pc = 0x00ea_73dc;
                    continue; // 00ea5dfc
                }
                0x00ea_5e01 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000068)))); // 00ea5e01
                    sw = fcom(x.st(0), x.st(2)); // 00ea5e05
                    ah = sw; // 00ea5e07
                    fl.and(u32::from(ah), 65, 8); // 00ea5e09
                    if !fl.zf {
                        pc = 0x00ea_5e1e;
                        continue;
                    } // 00ea5e0c
                    pc = 0x00ea_5e0e;
                }
                0x00ea_5e0e => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5e0e
                    ecx = edx; // 00ea5e10
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea5e12
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea5e16
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea5e18
                    pc = 0x00ea_5e47;
                    continue; // 00ea5e1c
                }
                0x00ea_5e1e => {
                    sw = fcom(x.st(0), x.st(2));
                    x.pop(); // 00ea5e1e
                    ah = sw; // 00ea5e20
                    fl.and(u32::from(ah), 5, 8); // 00ea5e22
                    if fl.pf {
                        pc = 0x00ea_5e3d;
                        continue;
                    } // 00ea5e25
                    pc = 0x00ea_5e27;
                }
                0x00ea_5e27 => {
                    ecx = edi; // 00ea5e27
                    ecx = fl.neg(ecx); // 00ea5e29
                    e.mem.set_u32(esp.wrapping_add(0x00000030), ecx); // 00ea5e2b
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea5e2f
                    ecx = edx; // 00ea5e33
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea5e35
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea5e37
                    pc = 0x00ea_5e47;
                    continue; // 00ea5e3b
                }
                0x00ea_5e3d => {
                    x.xch(1); // 00ea5e3d
                    ecx = edx; // 00ea5e3f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea5e41
                    x.xch(1); // 00ea5e45
                    pc = 0x00ea_5e47;
                }
                0x00ea_5e47 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000006c)))); // 00ea5e47
                    sw = fcom(x.st(0), x.st(2)); // 00ea5e4b
                    ah = sw; // 00ea5e4d
                    fl.and(u32::from(ah), 65, 8); // 00ea5e4f
                    if !fl.zf {
                        pc = 0x00ea_5e63;
                        continue;
                    } // 00ea5e52
                    pc = 0x00ea_5e54;
                }
                0x00ea_5e54 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5e54
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea5e56
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea5e5a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea5e5d
                    pc = 0x00ea_5e89;
                    continue; // 00ea5e61
                }
                0x00ea_5e63 => {
                    sw = fcom(x.st(0), x.st(2));
                    x.pop(); // 00ea5e63
                    ah = sw; // 00ea5e65
                    fl.and(u32::from(ah), 5, 8); // 00ea5e67
                    if fl.pf {
                        pc = 0x00ea_5e81;
                        continue;
                    } // 00ea5e6a
                    pc = 0x00ea_5e6c;
                }
                0x00ea_5e6c => {
                    edx = edi; // 00ea5e6c
                    edx = fl.neg(edx); // 00ea5e6e
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edx); // 00ea5e70
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea5e74
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea5e78
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea5e7b
                    pc = 0x00ea_5e89;
                    continue; // 00ea5e7f
                }
                0x00ea_5e81 => {
                    x.xch(1); // 00ea5e81
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0))); // 00ea5e83
                    x.xch(1); // 00ea5e87
                    pc = 0x00ea_5e89;
                }
                0x00ea_5e89 => {
                    sw = fcom(x.st(0), x.st(1)); // 00ea5e89
                    ah = sw; // 00ea5e8b
                    fl.and(u32::from(ah), 65, 8); // 00ea5e8d
                    if !fl.zf {
                        pc = 0x00ea_5e9c;
                        continue;
                    } // 00ea5e90
                    pc = 0x00ea_5e92;
                }
                0x00ea_5e92 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5e92
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5e94
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea5e96
                    pc = 0x00ea_5eb1;
                    continue; // 00ea5e9a
                }
                0x00ea_5e9c => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea5e9c
                    ah = sw; // 00ea5e9e
                    fl.and(u32::from(ah), 5, 8); // 00ea5ea0
                    if fl.pf {
                        pc = 0x00ea_5eb4;
                        continue;
                    } // 00ea5ea3
                    pc = 0x00ea_5ea5;
                }
                0x00ea_5ea5 => {
                    edi = fl.neg(edi); // 00ea5ea5
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5ea7
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea5ea9
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea5ead
                    pc = 0x00ea_5eb1;
                }
                0x00ea_5eb1 => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea5eb1
                    pc = 0x00ea_5eb4;
                }
                0x00ea_5eb4 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea5eb4
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea5eb8
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5ebf
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea5ec1
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea5ec8
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea5eca
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea5ed1
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea5ed3
                    x.push(x.st(0)); // 00ea5ed7
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ebx)))); // 00ea5ed9
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5edb
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea5edd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea5ee0
                    x.push(x.st(0)); // 00ea5ee4
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5ee6
                    x.xch(3); // 00ea5ee8
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5eea
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea5eec
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea5eef
                    x.push(x.st(0)); // 00ea5ef3
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea5ef5
                    x.xch(2); // 00ea5ef7
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5ef9
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea5efb
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5efd
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea5eff
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea5f02
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea5f05
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea5f08
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5f0a
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea5f0c
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea5f0f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5f11
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea5f13
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea5f16
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5f18
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea5f1a
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5f1d
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea5f1f
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea5f22
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea5f25
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea5f28
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5f2a
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea5f2c
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea5f2f
                    x.xch(1); // 00ea5f31
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea5f33
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea5f35
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea5f38
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea5f3a
                    pc = 0x00ea_73dc;
                    continue; // 00ea5f3d
                }
                0x00ea_5f42 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea5f42
                    x.set(0, x.st(0));
                    x.pop(); // 00ea5f44
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea5f46
                    fl.and(ebp, ebp, 32); // 00ea5f48
                    if !fl.zf {
                        pc = 0x00ea_5f82;
                        continue;
                    } // 00ea5f4a
                    pc = 0x00ea_5f4c;
                }
                0x00ea_5f4c => {
                    eax = e.mem.u32(esp.wrapping_add(0x000001f8)); // 00ea5f4c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5f53
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5f54
                    ecx = esp.wrapping_add(0x00000118); // 00ea5f55
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea5f5c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5f5f
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea5f60
                    ecx = e.mem.u32(esp.wrapping_add(0x000001f8)); // 00ea5f65
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea5f6c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5f6f
                    edx = esp.wrapping_add(0x00000138); // 00ea5f70
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5f77
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea5f78
                    pc = 0x00ea_5de5;
                    continue; // 00ea5f7d
                }
                0x00ea_5f82 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x000001f8)); // 00ea5f82
                    let r = fl.sub(ebp, 0x0000_0001, 32); // 00ea5f89
                    if !fl.zf {
                        pc = 0x00ea_5ff5;
                        continue;
                    } // 00ea5f8c
                    pc = 0x00ea_5f8e;
                }
                0x00ea_5f8e => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5f8e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea5f8f
                    edx = esp.wrapping_add(0x00000100); // 00ea5f90
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea5f97
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5f9a
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea5f9b
                    ecx = e.mem.u32(esp.wrapping_add(0x000001f8)); // 00ea5fa0
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea5fa7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5faa
                    eax = esp.wrapping_add(0x0000006c); // 00ea5fab
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea5faf
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea5fb0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea5fb5
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea5fb9
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea5fbb
                    ecx = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea5fbf
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000004)))); // 00ea5fc3
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000024)))),
                    ); // 00ea5fc6
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000078), x87_store32(x.st(0)));
                    x.pop(); // 00ea5fca
                    edx = e.mem.u32(esp.wrapping_add(0x00000078)); // 00ea5fce
                    x.push(x87_load32(e.mem.u32(eax.wrapping_add(0x00000008)))); // 00ea5fd2
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea5fd5
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000028)))),
                    ); // 00ea5fdc
                    e.mem.set_u32(eax, ecx); // 00ea5fe0
                    e.mem.set_u32(eax.wrapping_add(0x00000004), edx); // 00ea5fe2
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea5fe5
                    ecx = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea5fe9
                    e.mem.set_u32(eax.wrapping_add(0x00000008), ecx); // 00ea5fed
                    pc = 0x00ea_73dc;
                    continue; // 00ea5ff0
                }
                0x00ea_5ff5 => {
                    edx = esp.wrapping_add(0x000000d4); // 00ea5ff5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea5ffc
                    eax = esp.wrapping_add(0x00000054); // 00ea5ffd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6001
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6002
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6003
                    edx = esp.wrapping_add(0x0000006c); // 00ea6004
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea6008
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea600b
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea600c
                    ecx = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea6011
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6018
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea601b
                    eax = esp.wrapping_add(0x0000008c); // 00ea601c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6023
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6024
                    ecx = eax; // 00ea6029
                    let r = e.call(
                        0x0043_9e90,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea602b
                    pc = 0x00ea_5de5;
                    continue; // 00ea6030
                }
                0x00ea_6035 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6035
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000ac)))); // 00ea6037
                    sw = fcom(x.st(0), x.st(1)); // 00ea603e
                    ah = sw; // 00ea6040
                    fl.and(u32::from(ah), 65, 8); // 00ea6042
                    if !fl.zf {
                        pc = 0x00ea_6060;
                        continue;
                    } // 00ea6045
                    pc = 0x00ea_6047;
                }
                0x00ea_6047 => {
                    ecx = edi; // 00ea6047
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6049
                    ecx = fl.neg(ecx); // 00ea604b
                    e.mem.set_u32(esp.wrapping_add(0x00000030), ecx); // 00ea604d
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6051
                    ecx = edx; // 00ea6055
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6057
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea605a
                    pc = 0x00ea_607c;
                    continue; // 00ea605e
                }
                0x00ea_6060 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6060
                    ecx = edx; // 00ea6062
                    ah = sw; // 00ea6064
                    fl.and(u32::from(ah), 5, 8); // 00ea6066
                    if fl.pf {
                        pc = 0x00ea_6078;
                        continue;
                    } // 00ea6069
                    pc = 0x00ea_606b;
                }
                0x00ea_606b => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea606b
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea606f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6072
                    pc = 0x00ea_607c;
                    continue; // 00ea6076
                }
                0x00ea_6078 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0))); // 00ea6078
                    pc = 0x00ea_607c;
                }
                0x00ea_607c => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a0)))); // 00ea607c
                    sw = fcom(x.st(0), x.st(1)); // 00ea6083
                    ah = sw; // 00ea6085
                    fl.and(u32::from(ah), 65, 8); // 00ea6087
                    if !fl.zf {
                        pc = 0x00ea_6096;
                        continue;
                    } // 00ea608a
                    pc = 0x00ea_608c;
                }
                0x00ea_608c => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea608c
                    x.set(0, x.st(0));
                    x.pop(); // 00ea608e
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6090
                    pc = 0x00ea_60ab;
                    continue; // 00ea6094
                }
                0x00ea_6096 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6096
                    ah = sw; // 00ea6098
                    fl.and(u32::from(ah), 5, 8); // 00ea609a
                    if fl.pf {
                        pc = 0x00ea_60ae;
                        continue;
                    } // 00ea609d
                    pc = 0x00ea_609f;
                }
                0x00ea_609f => {
                    edi = fl.neg(edi); // 00ea609f
                    x.set(0, x.st(0));
                    x.pop(); // 00ea60a1
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea60a3
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea60a7
                    pc = 0x00ea_60ab;
                }
                0x00ea_60ab => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea60ab
                    pc = 0x00ea_60ae;
                }
                0x00ea_60ae => {
                    edx = esp.wrapping_add(0x00000080); // 00ea60ae
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea60b5
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea60b9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea60bb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea60bc
                    eax = esp.wrapping_add(0x00000038); // 00ea60bd
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea60c1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea60c4
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea60c5
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea60ca
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea60cd
                    ecx = esp.wrapping_add(0x000000f0); // 00ea60ce
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea60d5
                    ecx = esp.wrapping_add(0x000000c8); // 00ea60da
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea60e1
                    edx = esp.wrapping_add(0x0000015c); // 00ea60e2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea60e9
                    ecx = esp.wrapping_add(0x000000f4); // 00ea60ea
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea60f1
                    ecx = e.mem.u32(eax); // 00ea60f6
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea60f8
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea60fb
                    e.mem.set_u32(esp.wrapping_add(0x000000c8), ecx); // 00ea60fe
                    ecx = esp.wrapping_add(0x00000134); // 00ea6105
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea610c
                    e.mem.set_u32(esp.wrapping_add(0x000000d0), edx); // 00ea610d
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), eax); // 00ea6114
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea611b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000094), x87_store32(x.st(0)));
                    x.pop(); // 00ea6120
                    edx = esp.wrapping_add(0x00000110); // 00ea6127
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea612e
                    ecx = esp.wrapping_add(0x00000138); // 00ea612f
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6136
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a0), x87_store32(x.st(0)));
                    x.pop(); // 00ea613b
                    eax = esp.wrapping_add(0x00000140); // 00ea6142
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6149
                    ecx = esp.wrapping_add(0x00000138); // 00ea614a
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6151
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000ac), x87_store32(x.st(0)));
                    x.pop(); // 00ea6156
                    ecx = esp.wrapping_add(0x000000c8); // 00ea615d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6164
                    ecx = esp.wrapping_add(0x00000138); // 00ea6165
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea616c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea6171
                    x.push(x.st(0)); // 00ea6175
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a0)))),
                    ); // 00ea6177
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea617e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea6180
                    x.push(x.st(0)); // 00ea6184
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000ac)))),
                    ); // 00ea6186
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea618d
                    x.xch(2); // 00ea618f
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6191
                    fl.and(ebp, ebp, 32); // 00ea6195
                    if !fl.zf {
                        pc = 0x00ea_61aa;
                        continue;
                    } // 00ea6197
                    pc = 0x00ea_6199;
                }
                0x00ea_6199 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea6199
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000094)))),
                    ); // 00ea619d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea61a4
                    pc = 0x00ea_61e1;
                    continue; // 00ea61a8
                }
                0x00ea_61aa => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea61aa
                    edx = esp.wrapping_add(0x000000f8); // 00ea61ac
                    x.set(0, x.st(0));
                    x.pop(); // 00ea61b3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea61b5
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea61b6
                    ecx = esp.wrapping_add(0x00000138); // 00ea61ba
                    e.mem
                        .set_u64(esp.wrapping_add(0x00000034), x.st(0).to_bits());
                    x.pop(); // 00ea61c1
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea61c5
                    x.set(
                        0,
                        x87_sub(
                            f64::from_bits(e.mem.u64(esp.wrapping_add(0x00000030))),
                            x.st(0),
                        ),
                    ); // 00ea61ca
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000094)))),
                    ); // 00ea61ce
                    pc = 0x00ea_61d5;
                }
                0x00ea_61d5 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea61d5
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea61d9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea61dd
                    pc = 0x00ea_61e1;
                }
                0x00ea_61e1 => {
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea61e1
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea61e3
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea61ea
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea61ec
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea61f3
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea61fa
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea61fc
                    x.push(x.st(0)); // 00ea6200
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ebx)))); // 00ea6202
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea6204
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea6206
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea6209
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea620b
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea620d
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6210
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea6212
                    x.xch(1); // 00ea6214
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea6216
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6218
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea621a
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea621d
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea6220
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea6223
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6225
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea6227
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea622a
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea622c
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea622e
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6231
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6233
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea6235
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6238
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea623a
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea623d
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea6240
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6243
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6245
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea6247
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea624a
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea624c
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000020)))); // 00ea624e
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6251
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6253
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea6255
                    pc = 0x00ea_73dc;
                    continue; // 00ea6258
                }
                0x00ea_625d => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea625d
                    ecx = edx; // 00ea625f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b0)))); // 00ea6261
                    sw = fcom(x.st(0), x.st(1)); // 00ea6268
                    ah = sw; // 00ea626a
                    fl.and(u32::from(ah), 65, 8); // 00ea626c
                    if !fl.zf {
                        pc = 0x00ea_6288;
                        continue;
                    } // 00ea626f
                    pc = 0x00ea_6271;
                }
                0x00ea_6271 => {
                    eax = edi; // 00ea6271
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6273
                    eax = fl.neg(eax); // 00ea6275
                    e.mem.set_u32(esp.wrapping_add(0x00000030), eax); // 00ea6277
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea627b
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea627f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6282
                    pc = 0x00ea_62a2;
                    continue; // 00ea6286
                }
                0x00ea_6288 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6288
                    ah = sw; // 00ea628a
                    fl.and(u32::from(ah), 5, 8); // 00ea628c
                    if fl.pf {
                        pc = 0x00ea_629e;
                        continue;
                    } // 00ea628f
                    pc = 0x00ea_6291;
                }
                0x00ea_6291 => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6291
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6295
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6298
                    pc = 0x00ea_62a2;
                    continue; // 00ea629c
                }
                0x00ea_629e => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0))); // 00ea629e
                    pc = 0x00ea_62a2;
                }
                0x00ea_62a2 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a4)))); // 00ea62a2
                    sw = fcom(x.st(0), x.st(1)); // 00ea62a9
                    ah = sw; // 00ea62ab
                    fl.and(u32::from(ah), 65, 8); // 00ea62ad
                    if !fl.zf {
                        pc = 0x00ea_62bc;
                        continue;
                    } // 00ea62b0
                    pc = 0x00ea_62b2;
                }
                0x00ea_62b2 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea62b2
                    x.set(0, x.st(0));
                    x.pop(); // 00ea62b4
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea62b6
                    pc = 0x00ea_62d1;
                    continue; // 00ea62ba
                }
                0x00ea_62bc => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea62bc
                    ah = sw; // 00ea62be
                    fl.and(u32::from(ah), 5, 8); // 00ea62c0
                    if fl.pf {
                        pc = 0x00ea_62d4;
                        continue;
                    } // 00ea62c3
                    pc = 0x00ea_62c5;
                }
                0x00ea_62c5 => {
                    edi = fl.neg(edi); // 00ea62c5
                    x.set(0, x.st(0));
                    x.pop(); // 00ea62c7
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea62c9
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea62cd
                    pc = 0x00ea_62d1;
                }
                0x00ea_62d1 => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea62d1
                    pc = 0x00ea_62d4;
                }
                0x00ea_62d4 => {
                    ecx = esp.wrapping_add(0x00000080); // 00ea62d4
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea62db
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea62df
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea62e1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea62e2
                    edx = esp.wrapping_add(0x000001c0); // 00ea62e3
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea62ea
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea62ed
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea62ee
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea62f3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea62f6
                    ecx = esp.wrapping_add(0x000000f0); // 00ea62f7
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea62fe
                    eax = esp.wrapping_add(0x000000d4); // 00ea6303
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea630a
                    ecx = esp.wrapping_add(0x0000015c); // 00ea630b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6312
                    ecx = esp.wrapping_add(0x000000f4); // 00ea6313
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea631a
                    edx = e.mem.u32(eax); // 00ea631f
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6321
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), edx); // 00ea6324
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea632b
                    eax = esp.wrapping_add(0x0000011c); // 00ea632e
                    e.mem.set_u32(esp.wrapping_add(0x000000d8), ecx); // 00ea6335
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea633c
                    ecx = eax; // 00ea633d
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), edx); // 00ea633f
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6346
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000098), x87_store32(x.st(0)));
                    x.pop(); // 00ea634b
                    ecx = esp.wrapping_add(0x00000128); // 00ea6352
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6359
                    ecx = esp.wrapping_add(0x00000120); // 00ea635a
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6361
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a4), x87_store32(x.st(0)));
                    x.pop(); // 00ea6366
                    edx = esp.wrapping_add(0x0000003c); // 00ea636d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6371
                    ecx = esp.wrapping_add(0x00000120); // 00ea6372
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6379
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b0), x87_store32(x.st(0)));
                    x.pop(); // 00ea637e
                    eax = esp.wrapping_add(0x000000d4); // 00ea6385
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea638c
                    ecx = esp.wrapping_add(0x00000120); // 00ea638d
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6394
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea6399
                    x.push(x.st(0)); // 00ea639d
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a4)))),
                    ); // 00ea639f
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea63a6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea63a8
                    x.push(x.st(0)); // 00ea63ac
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b0)))),
                    ); // 00ea63ae
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea63b5
                    x.xch(2); // 00ea63b7
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea63b9
                    fl.and(ebp, ebp, 32); // 00ea63bd
                    if !fl.zf {
                        pc = 0x00ea_63d5;
                        continue;
                    } // 00ea63bf
                    pc = 0x00ea_63c1;
                }
                0x00ea_63c1 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea63c1
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000098)))),
                    ); // 00ea63c5
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea63cc
                    pc = 0x00ea_61e1;
                    continue; // 00ea63d0
                }
                0x00ea_63d5 => {
                    ecx = esp.wrapping_add(0x000000f8); // 00ea63d5
                    x.set(0, x.st(0));
                    x.pop(); // 00ea63dc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea63de
                    x.set(0, x.st(0));
                    x.pop(); // 00ea63df
                    ecx = esp.wrapping_add(0x00000120); // 00ea63e1
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea63e8
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea63ed
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000098)))),
                    ); // 00ea63f1
                    pc = 0x00ea_61d5;
                    continue; // 00ea63f8
                }
                0x00ea_63fd => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea63fd
                    x.set(0, x.st(0));
                    x.pop(); // 00ea63ff
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea6401
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000020)))),
                    ); // 00ea6405
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea6409
                    edx = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea640d
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea6411
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), edx); // 00ea6415
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000024)))),
                    ); // 00ea641c
                    edx = esp.wrapping_add(0x000000e0); // 00ea6420
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6427
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6428
                    eax = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea642c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea6430
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), eax); // 00ea6434
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000002c)))),
                    ); // 00ea643b
                    eax = esp.wrapping_add(0x0000015c); // 00ea643f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6446
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000084), x87_store32(x.st(0)));
                    x.pop(); // 00ea6447
                    ecx = e.mem.u32(esp.wrapping_add(0x00000084)); // 00ea644e
                    e.mem.set_u32(esp.wrapping_add(0x000000f0), ecx); // 00ea6455
                    ecx = ebx; // 00ea645c
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea645e
                    ecx = e.mem.u32(eax); // 00ea6463
                    e.mem.set_u32(esp.wrapping_add(0x00000068), ecx); // 00ea6465
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6469
                    e.mem.set_u32(esp.wrapping_add(0x0000006c), edx); // 00ea646c
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6470
                    ecx = esp.wrapping_add(0x000000e0); // 00ea6473
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea647a
                    edx = esp.wrapping_add(0x00000168); // 00ea647b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6482
                    ecx = ebx.wrapping_add(0x0000000c); // 00ea6483
                    e.mem.set_u32(esp.wrapping_add(0x00000078), eax); // 00ea6486
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea648a
                    ecx = e.mem.u32(eax); // 00ea648f
                    e.mem.set_u32(esp.wrapping_add(0x00000050), ecx); // 00ea6491
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6495
                    e.mem.set_u32(esp.wrapping_add(0x00000054), edx); // 00ea6498
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea649c
                    ecx = esp.wrapping_add(0x000000e0); // 00ea649f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea64a6
                    edx = esp.wrapping_add(0x00000174); // 00ea64a7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea64ae
                    ecx = ebx.wrapping_add(0x00000018); // 00ea64af
                    e.mem.set_u32(esp.wrapping_add(0x00000060), eax); // 00ea64b2
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea64b6
                    x.push(0.0); // 00ea64bb
                    ecx = e.mem.u32(eax); // 00ea64bd
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b4)))); // 00ea64bf
                    e.mem.set_u32(esp.wrapping_add(0x0000005c), ecx); // 00ea64c6
                    sw = fcom(x.st(0), x.st(1)); // 00ea64ca
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea64cc
                    e.mem.set_u32(esp.wrapping_add(0x00000060), edx); // 00ea64cf
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea64d3
                    e.mem.set_u32(esp.wrapping_add(0x00000064), eax); // 00ea64d6
                    ah = sw; // 00ea64da
                    fl.and(u32::from(ah), 65, 8); // 00ea64dc
                    if !fl.zf {
                        pc = 0x00ea_64fc;
                        continue;
                    } // 00ea64df
                    pc = 0x00ea_64e1;
                }
                0x00ea_64e1 => {
                    edx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea64e1
                    x.set(0, x.st(0));
                    x.pop(); // 00ea64e5
                    ecx = edi; // 00ea64e7
                    ecx = fl.neg(ecx); // 00ea64e9
                    e.mem.set_u32(esp.wrapping_add(0x00000030), ecx); // 00ea64eb
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea64ef
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea64f3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea64f6
                    pc = 0x00ea_651a;
                    continue; // 00ea64fa
                }
                0x00ea_64fc => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea64fc
                    ah = sw; // 00ea64fe
                    fl.and(u32::from(ah), 5, 8); // 00ea6500
                    if fl.pf {
                        pc = 0x00ea_6516;
                        continue;
                    } // 00ea6503
                    pc = 0x00ea_6505;
                }
                0x00ea_6505 => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6505
                    eax = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6509
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000004)))),
                    ); // 00ea650d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6510
                    pc = 0x00ea_651a;
                    continue; // 00ea6514
                }
                0x00ea_6516 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0))); // 00ea6516
                    pc = 0x00ea_651a;
                }
                0x00ea_651a => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a8)))); // 00ea651a
                    sw = fcom(x.st(0), x.st(1)); // 00ea6521
                    ah = sw; // 00ea6523
                    fl.and(u32::from(ah), 65, 8); // 00ea6525
                    if !fl.zf {
                        pc = 0x00ea_653b;
                        continue;
                    } // 00ea6528
                    pc = 0x00ea_652a;
                }
                0x00ea_652a => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea652a
                    x.set(1, x.st(0));
                    x.pop(); // 00ea652e
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6530
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6532
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea6536
                    pc = 0x00ea_6557;
                    continue; // 00ea6539
                }
                0x00ea_653b => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea653b
                    ah = sw; // 00ea653d
                    fl.and(u32::from(ah), 5, 8); // 00ea653f
                    if fl.pf {
                        pc = 0x00ea_6557;
                        continue;
                    } // 00ea6542
                    pc = 0x00ea_6544;
                }
                0x00ea_6544 => {
                    edx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6544
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6548
                    edi = fl.neg(edi); // 00ea654a
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea654c
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6550
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea6554
                    pc = 0x00ea_6557;
                }
                0x00ea_6557 => {
                    eax = esp.wrapping_add(0x00000080); // 00ea6557
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea655e
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6562
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6564
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6565
                    ecx = esp.wrapping_add(0x000001cc); // 00ea6566
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea656d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6570
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea6571
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6576
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6579
                    ecx = esp.wrapping_add(0x000000f0); // 00ea657a
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea6581
                    edx = esp.wrapping_add(0x000000e0); // 00ea6586
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea658d
                    eax = esp.wrapping_add(0x00000150); // 00ea658e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6595
                    ecx = esp.wrapping_add(0x000000f4); // 00ea6596
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea659d
                    ecx = e.mem.u32(eax); // 00ea65a2
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea65a4
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea65a7
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), ecx); // 00ea65aa
                    ecx = esp.wrapping_add(0x00000068); // 00ea65b1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea65b5
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), edx); // 00ea65b6
                    e.mem.set_u32(esp.wrapping_add(0x000000ec), eax); // 00ea65bd
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea65c4
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000009c), x87_store32(x.st(0)));
                    x.pop(); // 00ea65c9
                    edx = esp.wrapping_add(0x00000050); // 00ea65d0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea65d4
                    ecx = esp.wrapping_add(0x0000006c); // 00ea65d5
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea65d9
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a8), x87_store32(x.st(0)));
                    x.pop(); // 00ea65de
                    eax = esp.wrapping_add(0x0000005c); // 00ea65e5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea65e9
                    ecx = esp.wrapping_add(0x0000006c); // 00ea65ea
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea65ee
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b4), x87_store32(x.st(0)));
                    x.pop(); // 00ea65f3
                    ecx = esp.wrapping_add(0x000000e0); // 00ea65fa
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6601
                    ecx = esp.wrapping_add(0x0000006c); // 00ea6602
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6606
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea660b
                    x.push(x.st(0)); // 00ea660f
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a8)))),
                    ); // 00ea6611
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea6618
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea661a
                    x.push(x.st(0)); // 00ea661e
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b4)))),
                    ); // 00ea6620
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea6627
                    x.xch(2); // 00ea6629
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea662b
                    fl.and(ebp, ebp, 32); // 00ea662f
                    if !fl.zf {
                        pc = 0x00ea_6647;
                        continue;
                    } // 00ea6631
                    pc = 0x00ea_6633;
                }
                0x00ea_6633 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea6633
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000009c)))),
                    ); // 00ea6637
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea663e
                    pc = 0x00ea_61e1;
                    continue; // 00ea6642
                }
                0x00ea_6647 => {
                    edx = esp.wrapping_add(0x000000f8); // 00ea6647
                    x.set(0, x.st(0));
                    x.pop(); // 00ea664e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6650
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6651
                    ecx = esp.wrapping_add(0x0000006c); // 00ea6653
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6657
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea665c
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000009c)))),
                    ); // 00ea6660
                    pc = 0x00ea_61d5;
                    continue; // 00ea6667
                }
                0x00ea_666c => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea666c
                    ecx = edx; // 00ea666e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000ac)))); // 00ea6670
                    sw = fcom(x.st(0), x.st(1)); // 00ea6677
                    ah = sw; // 00ea6679
                    fl.and(u32::from(ah), 65, 8); // 00ea667b
                    if !fl.zf {
                        pc = 0x00ea_668e;
                        continue;
                    } // 00ea667e
                    pc = 0x00ea_6680;
                }
                0x00ea_6680 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6680
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6682
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6686
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6688
                    pc = 0x00ea_66af;
                    continue; // 00ea668c
                }
                0x00ea_668e => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea668e
                    ah = sw; // 00ea6690
                    fl.and(u32::from(ah), 5, 8); // 00ea6692
                    if fl.pf {
                        pc = 0x00ea_66ab;
                        continue;
                    } // 00ea6695
                    pc = 0x00ea_6697;
                }
                0x00ea_6697 => {
                    eax = edi; // 00ea6697
                    eax = fl.neg(eax); // 00ea6699
                    e.mem.set_u32(esp.wrapping_add(0x00000030), eax); // 00ea669b
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea669f
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea66a3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea66a5
                    pc = 0x00ea_66af;
                    continue; // 00ea66a9
                }
                0x00ea_66ab => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea66ab
                    pc = 0x00ea_66af;
                }
                0x00ea_66af => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000094)))); // 00ea66af
                    sw = fcom(x.st(0), x.st(1)); // 00ea66b6
                    ah = sw; // 00ea66b8
                    fl.and(u32::from(ah), 65, 8); // 00ea66ba
                    if !fl.zf {
                        pc = 0x00ea_66cf;
                        continue;
                    } // 00ea66bd
                    pc = 0x00ea_66bf;
                }
                0x00ea_66bf => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea66bf
                    edi = fl.neg(edi); // 00ea66c1
                    x.set(0, x.st(0));
                    x.pop(); // 00ea66c3
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea66c5
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea66c9
                    pc = 0x00ea_66de;
                    continue; // 00ea66cd
                }
                0x00ea_66cf => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea66cf
                    ah = sw; // 00ea66d1
                    fl.and(u32::from(ah), 5, 8); // 00ea66d3
                    if fl.pf {
                        pc = 0x00ea_66e1;
                        continue;
                    } // 00ea66d6
                    pc = 0x00ea_66d8;
                }
                0x00ea_66d8 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea66d8
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea66da
                    pc = 0x00ea_66de;
                }
                0x00ea_66de => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea66de
                    pc = 0x00ea_66e1;
                }
                0x00ea_66e1 => {
                    ecx = esp.wrapping_add(0x00000080); // 00ea66e1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea66e8
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea66ec
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea66ee
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea66ef
                    edx = esp.wrapping_add(0x000001d8); // 00ea66f0
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea66f7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea66fa
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea66fb
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6700
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6703
                    ecx = esp.wrapping_add(0x000000f0); // 00ea6704
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea670b
                    eax = esp.wrapping_add(0x000000c8); // 00ea6710
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6717
                    ecx = esp.wrapping_add(0x00000150); // 00ea6718
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea671f
                    ecx = esp.wrapping_add(0x000000f4); // 00ea6720
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6727
                    edx = e.mem.u32(eax); // 00ea672c
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea672e
                    e.mem.set_u32(esp.wrapping_add(0x000000c8), edx); // 00ea6731
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6738
                    eax = esp.wrapping_add(0x00000134); // 00ea673b
                    e.mem.set_u32(esp.wrapping_add(0x000000cc), ecx); // 00ea6742
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6749
                    ecx = esp.wrapping_add(0x00000114); // 00ea674a
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), edx); // 00ea6751
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6758
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000094), x87_store32(x.st(0)));
                    x.pop(); // 00ea675d
                    ecx = esp.wrapping_add(0x00000110); // 00ea6764
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea676b
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea676c
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a0), x87_store32(x.st(0)));
                    x.pop(); // 00ea6771
                    edx = esp.wrapping_add(0x00000140); // 00ea6778
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea677f
                    ecx = esp.wrapping_add(0x00000114); // 00ea6780
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6787
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000ac), x87_store32(x.st(0)));
                    x.pop(); // 00ea678c
                    eax = esp.wrapping_add(0x000000c8); // 00ea6793
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea679a
                    ecx = esp.wrapping_add(0x00000114); // 00ea679b
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea67a2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea67a7
                    x.push(x.st(0)); // 00ea67ab
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000094)))),
                    ); // 00ea67ad
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea67b4
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea67b6
                    x.push(x.st(0)); // 00ea67ba
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000ac)))),
                    ); // 00ea67bc
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea67c3
                    x.xch(2); // 00ea67c5
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea67c7
                    fl.and(ebp, ebp, 32); // 00ea67cb
                    if !fl.zf {
                        pc = 0x00ea_67e0;
                        continue;
                    } // 00ea67cd
                    pc = 0x00ea_67cf;
                }
                0x00ea_67cf => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea67cf
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a0)))),
                    ); // 00ea67d3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea67da
                    pc = 0x00ea_6817;
                    continue; // 00ea67de
                }
                0x00ea_67e0 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea67e0
                    ecx = esp.wrapping_add(0x000000f8); // 00ea67e2
                    x.set(0, x.st(0));
                    x.pop(); // 00ea67e9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea67eb
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea67ec
                    ecx = esp.wrapping_add(0x00000114); // 00ea67f0
                    e.mem
                        .set_u64(esp.wrapping_add(0x00000034), x.st(0).to_bits());
                    x.pop(); // 00ea67f7
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea67fb
                    x.set(
                        0,
                        x87_sub(
                            f64::from_bits(e.mem.u64(esp.wrapping_add(0x00000030))),
                            x.st(0),
                        ),
                    ); // 00ea6800
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a0)))),
                    ); // 00ea6804
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea680b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea680f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6813
                    pc = 0x00ea_6817;
                }
                0x00ea_6817 => {
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6817
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea6819
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6820
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea6822
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea6829
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea6830
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea6832
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea6834
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6836
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea6838
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea683b
                    x.push(x.st(0)); // 00ea683f
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6841
                    x.xch(2); // 00ea6843
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6845
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea6847
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea684a
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea684c
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea684e
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6850
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6852
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea6855
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea6858
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea685b
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea685d
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea685f
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea6862
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6864
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea6866
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6869
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea686b
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea686d
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6870
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea6872
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea6875
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea6878
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea687b
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea687d
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000014)))),
                    ); // 00ea687f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6882
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000020)))); // 00ea6884
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6887
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6889
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea688b
                    pc = 0x00ea_73dc;
                    continue; // 00ea688e
                }
                0x00ea_6893 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6893
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b0)))); // 00ea6895
                    sw = fcom(x.st(0), x.st(1)); // 00ea689c
                    ah = sw; // 00ea689e
                    fl.and(u32::from(ah), 65, 8); // 00ea68a0
                    if !fl.zf {
                        pc = 0x00ea_68b5;
                        continue;
                    } // 00ea68a3
                    pc = 0x00ea_68a5;
                }
                0x00ea_68a5 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea68a5
                    ecx = edx; // 00ea68a7
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea68a9
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea68ad
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea68af
                    pc = 0x00ea_68dc;
                    continue; // 00ea68b3
                }
                0x00ea_68b5 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea68b5
                    ah = sw; // 00ea68b7
                    fl.and(u32::from(ah), 5, 8); // 00ea68b9
                    if fl.pf {
                        pc = 0x00ea_68d6;
                        continue;
                    } // 00ea68bc
                    pc = 0x00ea_68be;
                }
                0x00ea_68be => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea68be
                    edx = edi; // 00ea68c2
                    edx = fl.neg(edx); // 00ea68c4
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edx); // 00ea68c6
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea68ca
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea68ce
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea68d0
                    pc = 0x00ea_68dc;
                    continue; // 00ea68d4
                }
                0x00ea_68d6 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea68d6
                    ecx = edx; // 00ea68da
                    pc = 0x00ea_68dc;
                }
                0x00ea_68dc => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000098)))); // 00ea68dc
                    sw = fcom(x.st(0), x.st(1)); // 00ea68e3
                    ah = sw; // 00ea68e5
                    fl.and(u32::from(ah), 65, 8); // 00ea68e7
                    if !fl.zf {
                        pc = 0x00ea_68fc;
                        continue;
                    } // 00ea68ea
                    pc = 0x00ea_68ec;
                }
                0x00ea_68ec => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea68ec
                    edi = fl.neg(edi); // 00ea68ee
                    x.set(0, x.st(0));
                    x.pop(); // 00ea68f0
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea68f2
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea68f6
                    pc = 0x00ea_690b;
                    continue; // 00ea68fa
                }
                0x00ea_68fc => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea68fc
                    ah = sw; // 00ea68fe
                    fl.and(u32::from(ah), 5, 8); // 00ea6900
                    if fl.pf {
                        pc = 0x00ea_690e;
                        continue;
                    } // 00ea6903
                    pc = 0x00ea_6905;
                }
                0x00ea_6905 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6905
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6907
                    pc = 0x00ea_690b;
                }
                0x00ea_690b => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea690b
                    pc = 0x00ea_690e;
                }
                0x00ea_690e => {
                    eax = esp.wrapping_add(0x00000080); // 00ea690e
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea6915
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6919
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea691b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea691c
                    ecx = esp.wrapping_add(0x0000019c); // 00ea691d
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea6924
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6927
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea6928
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea692d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6930
                    ecx = esp.wrapping_add(0x000000f0); // 00ea6931
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea6938
                    edx = esp.wrapping_add(0x000000d4); // 00ea693d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6944
                    eax = esp.wrapping_add(0x00000150); // 00ea6945
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea694c
                    ecx = esp.wrapping_add(0x000000f4); // 00ea694d
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6954
                    ecx = e.mem.u32(eax); // 00ea6959
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea695b
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea695e
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), ecx); // 00ea6961
                    ecx = esp.wrapping_add(0x0000011c); // 00ea6968
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea696f
                    ecx = esp.wrapping_add(0x0000012c); // 00ea6970
                    e.mem.set_u32(esp.wrapping_add(0x000000dc), edx); // 00ea6977
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), eax); // 00ea697e
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6985
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000098), x87_store32(x.st(0)));
                    x.pop(); // 00ea698a
                    edx = esp.wrapping_add(0x00000128); // 00ea6991
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6998
                    ecx = edx; // 00ea6999
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea699b
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a4), x87_store32(x.st(0)));
                    x.pop(); // 00ea69a0
                    eax = esp.wrapping_add(0x0000003c); // 00ea69a7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea69ab
                    ecx = esp.wrapping_add(0x0000012c); // 00ea69ac
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea69b3
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b0), x87_store32(x.st(0)));
                    x.pop(); // 00ea69b8
                    ecx = esp.wrapping_add(0x000000d4); // 00ea69bf
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea69c6
                    ecx = esp.wrapping_add(0x0000012c); // 00ea69c7
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea69ce
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea69d3
                    x.push(x.st(0)); // 00ea69d7
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000098)))),
                    ); // 00ea69d9
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea69e0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea69e2
                    x.push(x.st(0)); // 00ea69e6
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b0)))),
                    ); // 00ea69e8
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea69ef
                    x.xch(2); // 00ea69f1
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea69f3
                    fl.and(ebp, ebp, 32); // 00ea69f7
                    if !fl.zf {
                        pc = 0x00ea_6a0c;
                        continue;
                    } // 00ea69f9
                    pc = 0x00ea_69fb;
                }
                0x00ea_69fb => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea69fb
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a4)))),
                    ); // 00ea69ff
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6a06
                    pc = 0x00ea_6a3b;
                    continue; // 00ea6a0a
                }
                0x00ea_6a0c => {
                    edx = esp.wrapping_add(0x000000f8); // 00ea6a0c
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6a13
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6a15
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6a16
                    ecx = esp.wrapping_add(0x0000012c); // 00ea6a18
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6a1f
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea6a24
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a4)))),
                    ); // 00ea6a28
                    pc = 0x00ea_6a2f;
                }
                0x00ea_6a2f => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6a2f
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea6a33
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6a37
                    pc = 0x00ea_6a3b;
                }
                0x00ea_6a3b => {
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6a3b
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea6a3d
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6a44
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea6a46
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea6a4d
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea6a54
                    x.push(x.st(1)); // 00ea6a56
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ebx)))); // 00ea6a58
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a5a
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea6a5c
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea6a5f
                    x.push(x.st(0)); // 00ea6a63
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6a65
                    x.xch(2); // 00ea6a67
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a69
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea6a6b
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6a6e
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a70
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea6a72
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6a74
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6a76
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea6a79
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea6a7c
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea6a7f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a81
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea6a83
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea6a86
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a88
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea6a8a
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6a8d
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6a8f
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea6a91
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6a94
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea6a96
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea6a99
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea6a9c
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea6a9f
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea6aa1
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000014)))),
                    ); // 00ea6aa3
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6aa6
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000020)))); // 00ea6aa8
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6aab
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6aad
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea6aaf
                    pc = 0x00ea_73dc;
                    continue; // 00ea6ab2
                }
                0x00ea_6ab7 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea6ab7
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6ab9
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea6abb
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000020)))),
                    ); // 00ea6abf
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea6ac3
                    eax = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea6ac7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea6acb
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), eax); // 00ea6acf
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000024)))),
                    ); // 00ea6ad6
                    eax = esp.wrapping_add(0x000000e0); // 00ea6ada
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6ae1
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6ae2
                    ecx = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea6ae6
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea6aea
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), ecx); // 00ea6aee
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000002c)))),
                    ); // 00ea6af5
                    ecx = esp.wrapping_add(0x00000150); // 00ea6af9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6b00
                    ecx = ebx; // 00ea6b01
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000084), x87_store32(x.st(0)));
                    x.pop(); // 00ea6b03
                    edx = e.mem.u32(esp.wrapping_add(0x00000084)); // 00ea6b0a
                    e.mem.set_u32(esp.wrapping_add(0x000000f0), edx); // 00ea6b11
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6b18
                    edx = e.mem.u32(eax); // 00ea6b1d
                    e.mem.set_u32(esp.wrapping_add(0x00000068), edx); // 00ea6b1f
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6b23
                    e.mem.set_u32(esp.wrapping_add(0x0000006c), ecx); // 00ea6b26
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6b2a
                    eax = esp.wrapping_add(0x000000e0); // 00ea6b2d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6b34
                    ecx = esp.wrapping_add(0x00000174); // 00ea6b35
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6b3c
                    ecx = ebx.wrapping_add(0x0000000c); // 00ea6b3d
                    e.mem.set_u32(esp.wrapping_add(0x00000078), edx); // 00ea6b40
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6b44
                    edx = e.mem.u32(eax); // 00ea6b49
                    e.mem.set_u32(esp.wrapping_add(0x00000050), edx); // 00ea6b4b
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6b4f
                    e.mem.set_u32(esp.wrapping_add(0x00000054), ecx); // 00ea6b52
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6b56
                    eax = esp.wrapping_add(0x000000e0); // 00ea6b59
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6b60
                    ecx = esp.wrapping_add(0x00000168); // 00ea6b61
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6b68
                    ecx = ebx.wrapping_add(0x00000018); // 00ea6b69
                    e.mem.set_u32(esp.wrapping_add(0x00000060), edx); // 00ea6b6c
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6b70
                    x.push(0.0); // 00ea6b75
                    edx = e.mem.u32(eax); // 00ea6b77
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000b4)))); // 00ea6b79
                    e.mem.set_u32(esp.wrapping_add(0x0000005c), edx); // 00ea6b80
                    sw = fcom(x.st(0), x.st(1)); // 00ea6b84
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6b86
                    e.mem.set_u32(esp.wrapping_add(0x00000060), ecx); // 00ea6b89
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6b8d
                    e.mem.set_u32(esp.wrapping_add(0x00000064), edx); // 00ea6b90
                    ah = sw; // 00ea6b94
                    fl.and(u32::from(ah), 65, 8); // 00ea6b96
                    if !fl.zf {
                        pc = 0x00ea_6bad;
                        continue;
                    } // 00ea6b99
                    pc = 0x00ea_6b9b;
                }
                0x00ea_6b9b => {
                    eax = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6b9b
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6b9f
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6ba1
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(eax)))); // 00ea6ba5
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6ba7
                    pc = 0x00ea_6bd2;
                    continue; // 00ea6bab
                }
                0x00ea_6bad => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6bad
                    ah = sw; // 00ea6baf
                    fl.and(u32::from(ah), 5, 8); // 00ea6bb1
                    if fl.pf {
                        pc = 0x00ea_6bce;
                        continue;
                    } // 00ea6bb4
                    pc = 0x00ea_6bb6;
                }
                0x00ea_6bb6 => {
                    edx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6bb6
                    ecx = edi; // 00ea6bba
                    ecx = fl.neg(ecx); // 00ea6bbc
                    e.mem.set_u32(esp.wrapping_add(0x00000030), ecx); // 00ea6bbe
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6bc2
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea6bc6
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6bc8
                    pc = 0x00ea_6bd2;
                    continue; // 00ea6bcc
                }
                0x00ea_6bce => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea6bce
                    pc = 0x00ea_6bd2;
                }
                0x00ea_6bd2 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000009c)))); // 00ea6bd2
                    sw = fcom(x.st(0), x.st(1)); // 00ea6bd9
                    ah = sw; // 00ea6bdb
                    fl.and(u32::from(ah), 65, 8); // 00ea6bdd
                    if !fl.zf {
                        pc = 0x00ea_6bf9;
                        continue;
                    } // 00ea6be0
                    pc = 0x00ea_6be2;
                }
                0x00ea_6be2 => {
                    eax = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6be2
                    x.set(1, x.st(0));
                    x.pop(); // 00ea6be6
                    edi = fl.neg(edi); // 00ea6be8
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6bea
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea6bec
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6bf0
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000008)))),
                    ); // 00ea6bf4
                    pc = 0x00ea_6c0f;
                    continue; // 00ea6bf7
                }
                0x00ea_6bf9 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6bf9
                    ah = sw; // 00ea6bfb
                    fl.and(u32::from(ah), 5, 8); // 00ea6bfd
                    if fl.pf {
                        pc = 0x00ea_6c0f;
                        continue;
                    } // 00ea6c00
                    pc = 0x00ea_6c02;
                }
                0x00ea_6c02 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6c02
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6c06
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6c08
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea6c0c
                    pc = 0x00ea_6c0f;
                }
                0x00ea_6c0f => {
                    edx = esp.wrapping_add(0x00000080); // 00ea6c0f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea6c16
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6c1a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6c1c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6c1d
                    eax = esp.wrapping_add(0x00000184); // 00ea6c1e
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea6c25
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6c28
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea6c29
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6c2e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6c31
                    ecx = esp.wrapping_add(0x000000f0); // 00ea6c32
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea6c39
                    ecx = esp.wrapping_add(0x000000e0); // 00ea6c3e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6c45
                    edx = esp.wrapping_add(0x0000015c); // 00ea6c46
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6c4d
                    ecx = esp.wrapping_add(0x000000f4); // 00ea6c4e
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6c55
                    ecx = e.mem.u32(eax); // 00ea6c5a
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6c5c
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6c5f
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), ecx); // 00ea6c62
                    ecx = esp.wrapping_add(0x00000068); // 00ea6c69
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6c6d
                    ecx = esp.wrapping_add(0x00000054); // 00ea6c6e
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), edx); // 00ea6c72
                    e.mem.set_u32(esp.wrapping_add(0x000000ec), eax); // 00ea6c79
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6c80
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000009c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6c85
                    edx = esp.wrapping_add(0x00000050); // 00ea6c8c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6c90
                    ecx = edx; // 00ea6c91
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6c93
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a8), x87_store32(x.st(0)));
                    x.pop(); // 00ea6c98
                    eax = esp.wrapping_add(0x0000005c); // 00ea6c9f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6ca3
                    ecx = esp.wrapping_add(0x00000054); // 00ea6ca4
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6ca8
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b4), x87_store32(x.st(0)));
                    x.pop(); // 00ea6cad
                    ecx = esp.wrapping_add(0x000000e0); // 00ea6cb4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6cbb
                    ecx = esp.wrapping_add(0x00000054); // 00ea6cbc
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6cc0
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6cc5
                    x.push(x.st(0)); // 00ea6cc9
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000009c)))),
                    ); // 00ea6ccb
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea6cd2
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea6cd4
                    x.push(x.st(0)); // 00ea6cd8
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b4)))),
                    ); // 00ea6cda
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea6ce1
                    x.xch(2); // 00ea6ce3
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6ce5
                    fl.and(ebp, ebp, 32); // 00ea6ce9
                    if !fl.zf {
                        pc = 0x00ea_6d01;
                        continue;
                    } // 00ea6ceb
                    pc = 0x00ea_6ced;
                }
                0x00ea_6ced => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea6ced
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a8)))),
                    ); // 00ea6cf1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6cf8
                    pc = 0x00ea_6a3b;
                    continue; // 00ea6cfc
                }
                0x00ea_6d01 => {
                    edx = esp.wrapping_add(0x000000f8); // 00ea6d01
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d08
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6d0a
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d0b
                    ecx = esp.wrapping_add(0x00000054); // 00ea6d0d
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6d11
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea6d16
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a8)))),
                    ); // 00ea6d1a
                    pc = 0x00ea_6a2f;
                    continue; // 00ea6d21
                }
                0x00ea_6d26 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d26
                    ecx = edx; // 00ea6d28
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a0)))); // 00ea6d2a
                    sw = fcom(x.st(0), x.st(1)); // 00ea6d31
                    ah = sw; // 00ea6d33
                    fl.and(u32::from(ah), 65, 8); // 00ea6d35
                    if !fl.zf {
                        pc = 0x00ea_6d50;
                        continue;
                    } // 00ea6d38
                    pc = 0x00ea_6d3a;
                }
                0x00ea_6d3a => {
                    eax = edi; // 00ea6d3a
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d3c
                    eax = fl.neg(eax); // 00ea6d3e
                    e.mem.set_u32(esp.wrapping_add(0x00000030), eax); // 00ea6d40
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6d44
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6d48
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6d4a
                    pc = 0x00ea_6d69;
                    continue; // 00ea6d4e
                }
                0x00ea_6d50 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6d50
                    ah = sw; // 00ea6d52
                    fl.and(u32::from(ah), 5, 8); // 00ea6d54
                    if fl.pf {
                        pc = 0x00ea_6d65;
                        continue;
                    } // 00ea6d57
                    pc = 0x00ea_6d59;
                }
                0x00ea_6d59 => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6d59
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6d5d
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6d5f
                    pc = 0x00ea_6d69;
                    continue; // 00ea6d63
                }
                0x00ea_6d65 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea6d65
                    pc = 0x00ea_6d69;
                }
                0x00ea_6d69 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000094)))); // 00ea6d69
                    sw = fcom(x.st(0), x.st(1)); // 00ea6d70
                    ah = sw; // 00ea6d72
                    fl.and(u32::from(ah), 65, 8); // 00ea6d74
                    if !fl.zf {
                        pc = 0x00ea_6d83;
                        continue;
                    } // 00ea6d77
                    pc = 0x00ea_6d79;
                }
                0x00ea_6d79 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d79
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d7b
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6d7d
                    pc = 0x00ea_6d98;
                    continue; // 00ea6d81
                }
                0x00ea_6d83 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6d83
                    ah = sw; // 00ea6d85
                    fl.and(u32::from(ah), 5, 8); // 00ea6d87
                    if fl.pf {
                        pc = 0x00ea_6d9b;
                        continue;
                    } // 00ea6d8a
                    pc = 0x00ea_6d8c;
                }
                0x00ea_6d8c => {
                    edi = fl.neg(edi); // 00ea6d8c
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6d8e
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea6d90
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6d94
                    pc = 0x00ea_6d98;
                }
                0x00ea_6d98 => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6d98
                    pc = 0x00ea_6d9b;
                }
                0x00ea_6d9b => {
                    ecx = esp.wrapping_add(0x00000080); // 00ea6d9b
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6da2
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6da6
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6da8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6da9
                    edx = esp.wrapping_add(0x000001b4); // 00ea6daa
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea6db1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6db4
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea6db5
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6dba
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6dbd
                    ecx = esp.wrapping_add(0x000000f0); // 00ea6dbe
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea6dc5
                    eax = esp.wrapping_add(0x000000c8); // 00ea6dca
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6dd1
                    ecx = esp.wrapping_add(0x00000150); // 00ea6dd2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6dd9
                    ecx = esp.wrapping_add(0x000000f4); // 00ea6dda
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea6de1
                    edx = e.mem.u32(eax); // 00ea6de6
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea6de8
                    e.mem.set_u32(esp.wrapping_add(0x000000c8), edx); // 00ea6deb
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea6df2
                    eax = esp.wrapping_add(0x00000134); // 00ea6df5
                    e.mem.set_u32(esp.wrapping_add(0x000000cc), ecx); // 00ea6dfc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6e03
                    ecx = esp.wrapping_add(0x00000144); // 00ea6e04
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), edx); // 00ea6e0b
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6e12
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000094), x87_store32(x.st(0)));
                    x.pop(); // 00ea6e17
                    ecx = esp.wrapping_add(0x00000110); // 00ea6e1e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6e25
                    ecx = esp.wrapping_add(0x00000144); // 00ea6e26
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6e2d
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a0), x87_store32(x.st(0)));
                    x.pop(); // 00ea6e32
                    edx = esp.wrapping_add(0x00000140); // 00ea6e39
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea6e40
                    ecx = edx; // 00ea6e41
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6e43
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000ac), x87_store32(x.st(0)));
                    x.pop(); // 00ea6e48
                    eax = esp.wrapping_add(0x000000c8); // 00ea6e4f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6e56
                    ecx = esp.wrapping_add(0x00000144); // 00ea6e57
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6e5e
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6e63
                    x.push(x.st(0)); // 00ea6e67
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000094)))),
                    ); // 00ea6e69
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea6e70
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea6e72
                    x.push(x.st(0)); // 00ea6e76
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a0)))),
                    ); // 00ea6e78
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea6e7f
                    x.xch(2); // 00ea6e81
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea6e83
                    fl.and(ebp, ebp, 32); // 00ea6e87
                    if !fl.zf {
                        pc = 0x00ea_6e9c;
                        continue;
                    } // 00ea6e89
                    pc = 0x00ea_6e8b;
                }
                0x00ea_6e8b => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea6e8b
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000ac)))),
                    ); // 00ea6e8f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea6e96
                    pc = 0x00ea_6ed3;
                    continue; // 00ea6e9a
                }
                0x00ea_6e9c => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6e9c
                    ecx = esp.wrapping_add(0x000000f8); // 00ea6e9e
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6ea5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6ea7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6ea8
                    ecx = esp.wrapping_add(0x00000144); // 00ea6eac
                    e.mem
                        .set_u64(esp.wrapping_add(0x00000034), x.st(0).to_bits());
                    x.pop(); // 00ea6eb3
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea6eb7
                    x.set(
                        0,
                        x87_sub(
                            f64::from_bits(e.mem.u64(esp.wrapping_add(0x00000030))),
                            x.st(0),
                        ),
                    ); // 00ea6ebc
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000ac)))),
                    ); // 00ea6ec0
                    pc = 0x00ea_6ec7;
                }
                0x00ea_6ec7 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea6ec7
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea6ecb
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea6ecf
                    pc = 0x00ea_6ed3;
                }
                0x00ea_6ed3 => {
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6ed3
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea6ed5
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6edc
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea6ede
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea6ee5
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea6eec
                    x.push(x.st(1)); // 00ea6eee
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ebx)))); // 00ea6ef0
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6ef2
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea6ef4
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea6ef7
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6ef9
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea6efb
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea6efe
                    x.push(x.st(0)); // 00ea6f02
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea6f04
                    x.xch(2); // 00ea6f06
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6f08
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea6f0a
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6f0c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6f0e
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea6f11
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea6f14
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea6f17
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6f19
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea6f1b
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea6f1e
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6f20
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea6f22
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea6f25
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6f27
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea6f29
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6f2c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea6f2e
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea6f31
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea6f34
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea6f37
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea6f39
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea6f3b
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea6f3e
                    x.xch(1); // 00ea6f40
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea6f42
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea6f44
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea6f47
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea6f49
                    pc = 0x00ea_73dc;
                    continue; // 00ea6f4c
                }
                0x00ea_6f51 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6f51
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a4)))); // 00ea6f53
                    sw = fcom(x.st(0), x.st(1)); // 00ea6f5a
                    ah = sw; // 00ea6f5c
                    fl.and(u32::from(ah), 65, 8); // 00ea6f5e
                    if !fl.zf {
                        pc = 0x00ea_6f7d;
                        continue;
                    } // 00ea6f61
                    pc = 0x00ea_6f63;
                }
                0x00ea_6f63 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea6f63
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6f67
                    edx = edi; // 00ea6f69
                    edx = fl.neg(edx); // 00ea6f6b
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edx); // 00ea6f6d
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6f71
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6f75
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6f77
                    pc = 0x00ea_6f98;
                    continue; // 00ea6f7b
                }
                0x00ea_6f7d => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6f7d
                    ecx = edx; // 00ea6f7f
                    ah = sw; // 00ea6f81
                    fl.and(u32::from(ah), 5, 8); // 00ea6f83
                    if fl.pf {
                        pc = 0x00ea_6f94;
                        continue;
                    } // 00ea6f86
                    pc = 0x00ea_6f88;
                }
                0x00ea_6f88 => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6f88
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea6f8c
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea6f8e
                    pc = 0x00ea_6f98;
                    continue; // 00ea6f92
                }
                0x00ea_6f94 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea6f94
                    pc = 0x00ea_6f98;
                }
                0x00ea_6f98 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000098)))); // 00ea6f98
                    sw = fcom(x.st(0), x.st(1)); // 00ea6f9f
                    ah = sw; // 00ea6fa1
                    fl.and(u32::from(ah), 65, 8); // 00ea6fa3
                    if !fl.zf {
                        pc = 0x00ea_6fb2;
                        continue;
                    } // 00ea6fa6
                    pc = 0x00ea_6fa8;
                }
                0x00ea_6fa8 => {
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6fa8
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6faa
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea6fac
                    pc = 0x00ea_6fc7;
                    continue; // 00ea6fb0
                }
                0x00ea_6fb2 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea6fb2
                    ah = sw; // 00ea6fb4
                    fl.and(u32::from(ah), 5, 8); // 00ea6fb6
                    if fl.pf {
                        pc = 0x00ea_6fca;
                        continue;
                    } // 00ea6fb9
                    pc = 0x00ea_6fbb;
                }
                0x00ea_6fbb => {
                    edi = fl.neg(edi); // 00ea6fbb
                    x.set(0, x.st(0));
                    x.pop(); // 00ea6fbd
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea6fbf
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea6fc3
                    pc = 0x00ea_6fc7;
                }
                0x00ea_6fc7 => {
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea6fc7
                    pc = 0x00ea_6fca;
                }
                0x00ea_6fca => {
                    eax = esp.wrapping_add(0x00000080); // 00ea6fca
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea6fd1
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea6fd5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6fd7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6fd8
                    ecx = esp.wrapping_add(0x00000190); // 00ea6fd9
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea6fe0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea6fe3
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea6fe4
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea6fe9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea6fec
                    ecx = esp.wrapping_add(0x000000f0); // 00ea6fed
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea6ff4
                    edx = esp.wrapping_add(0x000000d4); // 00ea6ff9
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea7000
                    eax = esp.wrapping_add(0x00000150); // 00ea7001
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea7008
                    ecx = esp.wrapping_add(0x000000f4); // 00ea7009
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea7010
                    ecx = e.mem.u32(eax); // 00ea7015
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea7017
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea701a
                    e.mem.set_u32(esp.wrapping_add(0x000000d4), ecx); // 00ea701d
                    ecx = esp.wrapping_add(0x0000011c); // 00ea7024
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea702b
                    ecx = esp.wrapping_add(0x00000040); // 00ea702c
                    e.mem.set_u32(esp.wrapping_add(0x000000dc), edx); // 00ea7030
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), eax); // 00ea7037
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea703e
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000098), x87_store32(x.st(0)));
                    x.pop(); // 00ea7043
                    edx = esp.wrapping_add(0x00000128); // 00ea704a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea7051
                    ecx = esp.wrapping_add(0x00000040); // 00ea7052
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7056
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a4), x87_store32(x.st(0)));
                    x.pop(); // 00ea705b
                    eax = esp.wrapping_add(0x0000003c); // 00ea7062
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea7066
                    ecx = eax; // 00ea7067
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7069
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b0), x87_store32(x.st(0)));
                    x.pop(); // 00ea706e
                    ecx = esp.wrapping_add(0x000000d4); // 00ea7075
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea707c
                    ecx = esp.wrapping_add(0x00000040); // 00ea707d
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7081
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea7086
                    x.push(x.st(0)); // 00ea708a
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000098)))),
                    ); // 00ea708c
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea7093
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea7095
                    x.push(x.st(0)); // 00ea7099
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a4)))),
                    ); // 00ea709b
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea70a2
                    x.xch(2); // 00ea70a4
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea70a6
                    fl.and(ebp, ebp, 32); // 00ea70aa
                    if !fl.zf {
                        pc = 0x00ea_70bf;
                        continue;
                    } // 00ea70ac
                    pc = 0x00ea_70ae;
                }
                0x00ea_70ae => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea70ae
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b0)))),
                    ); // 00ea70b2
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea70b9
                    pc = 0x00ea_70eb;
                    continue; // 00ea70bd
                }
                0x00ea_70bf => {
                    edx = esp.wrapping_add(0x000000f8); // 00ea70bf
                    x.set(0, x.st(0));
                    x.pop(); // 00ea70c6
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea70c8
                    x.set(0, x.st(0));
                    x.pop(); // 00ea70c9
                    ecx = esp.wrapping_add(0x00000040); // 00ea70cb
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea70cf
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea70d4
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b0)))),
                    ); // 00ea70d8
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea70df
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea70e3
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea70e7
                    pc = 0x00ea_70eb;
                }
                0x00ea_70eb => {
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea70eb
                    ecx = e.mem.u32(esp.wrapping_add(0x000001e8)); // 00ea70ed
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea70f4
                    edx = e.mem.u32(esp.wrapping_add(0x00000090)); // 00ea70f6
                    eax = e.mem.u32(esp.wrapping_add(0x00000200)); // 00ea70fd
                    x.set(0, x87_add(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea7104
                    x.push(x87_load32(e.mem.u32(ebx))); // 00ea7106
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea7108
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea710a
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000000c)))); // 00ea710c
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea710f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea7111
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000018)))); // 00ea7113
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000028)))); // 00ea7116
                    x.push(x.st(0)); // 00ea711a
                    x.set(2, x87_mul(x.st(2), x.st(0)));
                    x.pop(); // 00ea711c
                    x.xch(2); // 00ea711e
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea7120
                    e.mem.set_u32(eax, x87_store32(x.st(0)));
                    x.pop(); // 00ea7122
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea7124
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea7126
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000004)))),
                    ); // 00ea7129
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000004)))); // 00ea712c
                    x.set(0, x87_mul(x.st(0), x.st(3))); // 00ea712f
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea7131
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000010)))); // 00ea7133
                    x.set(0, x87_mul(x.st(0), x.st(4))); // 00ea7136
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea7138
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x0000001c)))); // 00ea713a
                    x.set(0, x87_mul(x.st(0), x.st(2))); // 00ea713d
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea713f
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000004), x87_store32(x.st(0)));
                    x.pop(); // 00ea7141
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea7144
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000008)))),
                    ); // 00ea7146
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(edx.wrapping_add(0x00000008)))),
                    ); // 00ea7149
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000008)))); // 00ea714c
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea714f
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea7151
                    x.push(x87_load32(e.mem.u32(ebx.wrapping_add(0x00000014)))); // 00ea7153
                    x.set(3, x87_mul(x.st(3), x.st(0)));
                    x.pop(); // 00ea7156
                    x.xch(1); // 00ea7158
                    x.set(2, x87_add(x.st(2), x.st(0)));
                    x.pop(); // 00ea715a
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ebx.wrapping_add(0x00000020)))),
                    ); // 00ea715c
                    x.set(1, x87_add(x.st(1), x.st(0)));
                    x.pop(); // 00ea715f
                    e.mem
                        .set_u32(eax.wrapping_add(0x00000008), x87_store32(x.st(0)));
                    x.pop(); // 00ea7161
                    pc = 0x00ea_73dc;
                    continue; // 00ea7164
                }
                0x00ea_7169 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea7169
                    x.set(0, x.st(0));
                    x.pop(); // 00ea716b
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000050)))); // 00ea716d
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000020)))),
                    ); // 00ea7171
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea7175
                    eax = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea7179
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000054)))); // 00ea717d
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), eax); // 00ea7181
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x00000024)))),
                    ); // 00ea7188
                    eax = esp.wrapping_add(0x000000e0); // 00ea718c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea7193
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea7194
                    ecx = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea7198
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000005c)))); // 00ea719c
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), ecx); // 00ea71a0
                    x.set(
                        0,
                        x87_sub(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000002c)))),
                    ); // 00ea71a7
                    ecx = esp.wrapping_add(0x00000150); // 00ea71ab
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea71b2
                    ecx = ebx; // 00ea71b3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000084), x87_store32(x.st(0)));
                    x.pop(); // 00ea71b5
                    edx = e.mem.u32(esp.wrapping_add(0x00000084)); // 00ea71bc
                    e.mem.set_u32(esp.wrapping_add(0x000000f0), edx); // 00ea71c3
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea71ca
                    edx = e.mem.u32(eax); // 00ea71cf
                    e.mem.set_u32(esp.wrapping_add(0x00000068), edx); // 00ea71d1
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea71d5
                    e.mem.set_u32(esp.wrapping_add(0x0000006c), ecx); // 00ea71d8
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea71dc
                    eax = esp.wrapping_add(0x000000e0); // 00ea71df
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea71e6
                    ecx = esp.wrapping_add(0x00000174); // 00ea71e7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea71ee
                    ecx = ebx.wrapping_add(0x0000000c); // 00ea71ef
                    e.mem.set_u32(esp.wrapping_add(0x00000078), edx); // 00ea71f2
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea71f6
                    edx = e.mem.u32(eax); // 00ea71fb
                    e.mem.set_u32(esp.wrapping_add(0x00000050), edx); // 00ea71fd
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea7201
                    e.mem.set_u32(esp.wrapping_add(0x00000054), ecx); // 00ea7204
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea7208
                    eax = esp.wrapping_add(0x000000e0); // 00ea720b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea7212
                    ecx = esp.wrapping_add(0x00000168); // 00ea7213
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea721a
                    ecx = ebx.wrapping_add(0x00000018); // 00ea721b
                    e.mem.set_u32(esp.wrapping_add(0x00000060), edx); // 00ea721e
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea7222
                    x.push(0.0); // 00ea7227
                    edx = e.mem.u32(eax); // 00ea7229
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x000000a8)))); // 00ea722b
                    e.mem.set_u32(esp.wrapping_add(0x0000005c), edx); // 00ea7232
                    sw = fcom(x.st(0), x.st(1)); // 00ea7236
                    ecx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea7238
                    e.mem.set_u32(esp.wrapping_add(0x00000060), ecx); // 00ea723b
                    edx = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea723f
                    e.mem.set_u32(esp.wrapping_add(0x00000064), edx); // 00ea7242
                    ah = sw; // 00ea7246
                    fl.and(u32::from(ah), 65, 8); // 00ea7248
                    if !fl.zf {
                        pc = 0x00ea_7267;
                        continue;
                    } // 00ea724b
                    pc = 0x00ea_724d;
                }
                0x00ea_724d => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea724d
                    x.set(0, x.st(0));
                    x.pop(); // 00ea7251
                    eax = edi; // 00ea7253
                    eax = fl.neg(eax); // 00ea7255
                    e.mem.set_u32(esp.wrapping_add(0x00000030), eax); // 00ea7257
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea725b
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(ecx)))); // 00ea725f
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea7261
                    pc = 0x00ea_7284;
                    continue; // 00ea7265
                }
                0x00ea_7267 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea7267
                    ah = sw; // 00ea7269
                    fl.and(u32::from(ah), 5, 8); // 00ea726b
                    if fl.pf {
                        pc = 0x00ea_7280;
                        continue;
                    } // 00ea726e
                    pc = 0x00ea_7270;
                }
                0x00ea_7270 => {
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea7270
                    edx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea7274
                    x.set(0, x87_mul(x.st(0), x87_m32(e.mem.u32(edx)))); // 00ea7278
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0)));
                    x.pop(); // 00ea727a
                    pc = 0x00ea_7284;
                    continue; // 00ea727e
                }
                0x00ea_7280 => {
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000020), x87_store32(x.st(0))); // 00ea7280
                    pc = 0x00ea_7284;
                }
                0x00ea_7284 => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000009c)))); // 00ea7284
                    sw = fcom(x.st(0), x.st(1)); // 00ea728b
                    ah = sw; // 00ea728d
                    fl.and(u32::from(ah), 65, 8); // 00ea728f
                    if !fl.zf {
                        pc = 0x00ea_72a5;
                        continue;
                    } // 00ea7292
                    pc = 0x00ea_7294;
                }
                0x00ea_7294 => {
                    eax = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea7294
                    x.set(1, x.st(0));
                    x.pop(); // 00ea7298
                    x.set(0, x.st(0));
                    x.pop(); // 00ea729a
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000014)) as i32)); // 00ea729c
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(eax.wrapping_add(0x00000004)))),
                    ); // 00ea72a0
                    pc = 0x00ea_72c1;
                    continue; // 00ea72a3
                }
                0x00ea_72a5 => {
                    sw = fcom(x.st(0), x.st(1));
                    x.pop(); // 00ea72a5
                    ah = sw; // 00ea72a7
                    fl.and(u32::from(ah), 5, 8); // 00ea72a9
                    if fl.pf {
                        pc = 0x00ea_72c1;
                        continue;
                    } // 00ea72ac
                    pc = 0x00ea_72ae;
                }
                0x00ea_72ae => {
                    ecx = e.mem.u32(esp.wrapping_add(0x0000001c)); // 00ea72ae
                    x.set(0, x.st(0));
                    x.pop(); // 00ea72b2
                    edi = fl.neg(edi); // 00ea72b4
                    e.mem.set_u32(esp.wrapping_add(0x00000030), edi); // 00ea72b6
                    x.push(f64::from(e.mem.u32(esp.wrapping_add(0x00000030)) as i32)); // 00ea72ba
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(ecx.wrapping_add(0x00000004)))),
                    ); // 00ea72be
                    pc = 0x00ea_72c1;
                }
                0x00ea_72c1 => {
                    edx = esp.wrapping_add(0x00000080); // 00ea72c1
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000024), x87_store32(x.st(0)));
                    x.pop(); // 00ea72c8
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea72cc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea72ce
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea72cf
                    eax = esp.wrapping_add(0x000001a8); // 00ea72d0
                    e.mem.set_u32(esp, x87_store32(x.st(0)));
                    x.pop(); // 00ea72d7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea72da
                    let r = e.call(
                        0x004a_3760,
                        &[
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx; // 00ea72db
                    let r = fl.add(esp, 0x0000_000c, 32);
                    esp = r; // 00ea72e0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea72e3
                    ecx = esp.wrapping_add(0x000000f0); // 00ea72e4
                    let r = e.call(0x0063_c8a0, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4); // 00ea72eb
                    ecx = esp.wrapping_add(0x000000e0); // 00ea72f0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea72f7
                    edx = esp.wrapping_add(0x0000015c); // 00ea72f8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea72ff
                    ecx = esp.wrapping_add(0x000000f4); // 00ea7300
                    let r = e.call(
                        0x004b_3800,
                        &[
                            ecx,
                            e.mem.u32(esp.wrapping_add(0)),
                            e.mem.u32(esp.wrapping_add(4)),
                        ],
                    );
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(8); // 00ea7307
                    ecx = e.mem.u32(eax); // 00ea730c
                    edx = e.mem.u32(eax.wrapping_add(0x00000004)); // 00ea730e
                    eax = e.mem.u32(eax.wrapping_add(0x00000008)); // 00ea7311
                    e.mem.set_u32(esp.wrapping_add(0x000000e0), ecx); // 00ea7314
                    ecx = esp.wrapping_add(0x00000068); // 00ea731b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea731f
                    ecx = esp.wrapping_add(0x00000060); // 00ea7320
                    e.mem.set_u32(esp.wrapping_add(0x000000e8), edx); // 00ea7324
                    e.mem.set_u32(esp.wrapping_add(0x000000ec), eax); // 00ea732b
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7332
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000009c), x87_store32(x.st(0)));
                    x.pop(); // 00ea7337
                    edx = esp.wrapping_add(0x00000050); // 00ea733e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea7342
                    ecx = esp.wrapping_add(0x00000060); // 00ea7343
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7347
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000a8), x87_store32(x.st(0)));
                    x.pop(); // 00ea734c
                    eax = esp.wrapping_add(0x0000005c); // 00ea7353
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00ea7357
                    ecx = eax; // 00ea7358
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea735a
                    e.mem
                        .set_u32(esp.wrapping_add(0x000000b4), x87_store32(x.st(0)));
                    x.pop(); // 00ea735f
                    ecx = esp.wrapping_add(0x000000e0); // 00ea7366
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00ea736d
                    ecx = esp.wrapping_add(0x00000060); // 00ea736e
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea7372
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000020)))); // 00ea7377
                    x.push(x.st(0)); // 00ea737b
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000009c)))),
                    ); // 00ea737d
                    x.set(2, x87_sub(x.st(2), x.st(0)));
                    x.pop(); // 00ea7384
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x00000024)))); // 00ea7386
                    x.push(x.st(0)); // 00ea738a
                    x.set(
                        0,
                        x87_mul(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000a8)))),
                    ); // 00ea738c
                    x.set(3, x87_sub(x.st(3), x.st(0)));
                    x.pop(); // 00ea7393
                    x.xch(2); // 00ea7395
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000001c), x87_store32(x.st(0)));
                    x.pop(); // 00ea7397
                    fl.and(ebp, ebp, 32); // 00ea739b
                    if !fl.zf {
                        pc = 0x00ea_73b3;
                        continue;
                    } // 00ea739d
                    pc = 0x00ea_739f;
                }
                0x00ea_739f => {
                    x.push(x87_load32(e.mem.u32(esp.wrapping_add(0x0000001c)))); // 00ea739f
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b4)))),
                    ); // 00ea73a3
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000028), x87_store32(x.st(0)));
                    x.pop(); // 00ea73aa
                    pc = 0x00ea_6ed3;
                    continue; // 00ea73ae
                }
                0x00ea_73b3 => {
                    edx = esp.wrapping_add(0x000000f8); // 00ea73b3
                    x.set(0, x.st(0));
                    x.pop(); // 00ea73ba
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00ea73bc
                    x.set(0, x.st(0));
                    x.pop(); // 00ea73bd
                    ecx = esp.wrapping_add(0x00000060); // 00ea73bf
                    let r = e.call(0x004b_6190, &[ecx, e.mem.u32(esp.wrapping_add(0))]);
                    eax = r.eax;
                    edx = r.edx;
                    esp = esp.wrapping_add(4);
                    x.push(r.st0); // 00ea73c3
                    x.set(
                        0,
                        x87_add(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x0000001c)))),
                    ); // 00ea73c8
                    x.set(
                        0,
                        x87_div(x.st(0), x87_m32(e.mem.u32(esp.wrapping_add(0x000000b4)))),
                    ); // 00ea73cc
                    pc = 0x00ea_6ec7;
                    continue; // 00ea73d3
                }
                0x00ea_73d8 => {
                    x.set(1, x.st(0));
                    x.pop(); // 00ea73d8
                    x.set(0, x.st(0));
                    x.pop(); // 00ea73da
                    pc = 0x00ea_73dc;
                }
                0x00ea_73dc => {
                    let r = fl.sub(
                        u32::from(e.mem.u8(esp.wrapping_add(0x00000204))),
                        0x0000_0000,
                        8,
                    ); // 00ea73dc
                    if fl.zf {
                        pc = 0x00ea_7446;
                        continue;
                    } // 00ea73e4
                    pc = 0x00ea_73e6;
                }
                0x00ea_73e6 => {
                    esi = e.mem.u32(esp.wrapping_add(0x0000020c)); // 00ea73e6
                    eax = e.mem.u32(esp.wrapping_add(0x00000104)); // 00ea73ed
                    ecx = e.mem.u32(esp.wrapping_add(0x00000108)); // 00ea73f4
                    edx = e.mem.u32(esp.wrapping_add(0x0000010c)); // 00ea73fb
                    e.mem.set_u32(esi, eax); // 00ea7402
                    e.mem.set_u32(esi.wrapping_add(0x00000004), ecx); // 00ea7404
                    ecx = esi; // 00ea7407
                    e.mem.set_u32(esi.wrapping_add(0x00000008), edx); // 00ea7409
                    let r = e.call(0x004a_0c10, &[ecx]);
                    eax = r.eax;
                    edx = r.edx; // 00ea740c
                    x.push(x87_load32(e.mem.u32(esi))); // 00ea7411
                    eax = e.mem.u32(esp.wrapping_add(0x00000208)); // 00ea7413
                    x.set(0, -x.st(0)); // 00ea741a
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000074), x87_store32(x.st(0)));
                    x.pop(); // 00ea741c
                    ecx = e.mem.u32(esp.wrapping_add(0x00000074)); // 00ea7420
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000004)))); // 00ea7424
                    x.set(0, -x.st(0)); // 00ea7427
                    e.mem
                        .set_u32(esp.wrapping_add(0x00000078), x87_store32(x.st(0)));
                    x.pop(); // 00ea7429
                    edx = e.mem.u32(esp.wrapping_add(0x00000078)); // 00ea742d
                    x.push(x87_load32(e.mem.u32(esi.wrapping_add(0x00000008)))); // 00ea7431
                    e.mem.set_u32(eax, ecx); // 00ea7434
                    x.set(0, -x.st(0)); // 00ea7436
                    e.mem.set_u32(eax.wrapping_add(0x00000004), edx); // 00ea7438
                    e.mem
                        .set_u32(esp.wrapping_add(0x0000007c), x87_store32(x.st(0)));
                    x.pop(); // 00ea743b
                    ecx = e.mem.u32(esp.wrapping_add(0x0000007c)); // 00ea743f
                    e.mem.set_u32(eax.wrapping_add(0x00000008), ecx); // 00ea7443
                    pc = 0x00ea_7446;
                }
                0x00ea_7446 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea7446
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea7447
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea7448
                    eax = (eax & 0xffff_ff00) | 1; // 00ea7449
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00ea744b
                    let r = fl.add(esp, 0x0000_01cc, 32);
                    esp = r; // 00ea744c
                    return (eax & 0xff) != 0; // 00ea7452
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_e840, ni_box_bv_operator_(Ptr<NiBoxBV>, Ptr) -> bool),
        entry!(
            0x00ea_07a0,
            ni_box_bv_operator_ov2(Ptr<NiBoxBV>, Ptr) -> bool
        ),
        entry!(0x00ea_07e0, ni_box_bv_load_binary(Ptr<NiBoxBV>, Ptr)),
        entry!(0x00ea_0800, ni_box_bv_save_binary(Ptr<NiBoxBV>, Ptr)),
        entry!(
            0x00ea_0820,
            fn_00ea0820(Ptr<NiBoxBV>, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr<NiBoxBV>
        ),
        entry!(0x00ea_08a0, ni_box_bv_copy(Ptr<NiBoxBV>, Ptr<NiBoxBV>)),
        entry!(
            0x00ea_08c0,
            ni_box_bv_update_world_data(Ptr<NiBoxBV>, Ptr<NiBoxBV>, Ptr)
        ),
        entry!(0x00ea_2030, fn_00ea2030(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00ea_3950,
            fn_00ea3950(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00ea_3a60,
            ni_box_bv_get_normal(Ptr<NiBoxBV>, Ptr, i32, i32, i32, f32, f32, f32) -> Ptr
        ),
        entry!(
            0x00ea_3c50,
            fn_00ea3c50(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(0x00ea_7490, fn_00ea7490(Ptr) -> Ptr<NiBoxBV>),
        entry!(0x00ea_74f0, ni_box_bv_create() -> Ptr<NiBoxBV>),
        entry!(0x00ea_7530, fn_00ea7530(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00ea_7d10, ni_box_bv_clone(Ptr<NiBoxBV>) -> Ptr<NiBoxBV>),
        entry!(
            0x00ea_b7b0,
            ni_box_bv_add_viewer_strings(Ptr<NiBoxBV>, Ptr, Ptr)
        ),
        entry!(
            0x00ea_c1d0,
            ni_box_bv_scalar_deleting_destructor(Ptr<NiBoxBV>, u32) -> Ptr<NiBoxBV>
        ),
        entry!(0x00ea_7730, fn_00ea7730(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ea_0ad0, fn_00ea0ad0(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00ea_2190,
            fn_00ea2190(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00ea_7d40,
            fn_00ea7d40(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00ea_3d60,
            fn_00ea3d60(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::units::nicollision::nicollisiondata::{VIEWER_STRINGS_GROW, VIEWER_STRINGS_SET_AT};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn read(e: &Engine, p: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(p + 4 * i)).collect()
    }

    /// A box volume: centre, three axes, three extents.
    fn box_volume(e: &mut Engine, values: [f32; 15]) -> u32 {
        let b = e.mem.alloc(0x40);
        e.mem.set_u32(b, NI_BOX_BV_VTABLE);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(b + 4 + 4 * i as u32, *v);
        }
        b
    }

    /// An axis-aligned box at `center` with half extents `extent`.
    fn aligned(e: &mut Engine, center: [f32; 3], extent: [f32; 3]) -> u32 {
        box_volume(
            e,
            [
                center[0], center[1], center[2], 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                extent[0], extent[1], extent[2],
            ],
        )
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Data pages the box functions read: the zero point and default
    /// axes, the extents, the epsilon and the intersector floats.
    fn data(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        for (i, v) in [
            1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_f32(DEFAULT_AXIS_X + 4 * i as u32, *v);
        }
        e.map(0x0104_d000, 0x1000);
        e.mem.set_u32(FLT_EPSILON, 0x3400_0000);
        e.map(0x0101_4000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_f32(PSEUDODISTANCE_THRESHOLD, 0.0001);
        e.mem.set_f32(DERIVATIVE_TIME_STEP, 0.001);
    }

    #[test]
    fn operators_compare_types_and_boxes() {
        let mut e = Engine::new();
        // The other volume's `Type` (slot +0xc of a test vtable) answers its
        // word at +0x40.
        e.register(0x0f00_000c, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0, 0, 0x0f00_000c]);
        e.register(NI_BOX_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        e.mem.set_u32(other, 0x0f00_1000);
        e.mem.set_u32(other + 0x40, 1);
        e.mem.set_u32(this + 4, 7);
        e.mem.set_u32(other + 4, 7);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ea_07a0, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_BOX_EQUAL), vec![vec![other + 4, this + 4]]);
        e.mem.set_u32(other + 4, 8);
        assert!(!e.call(0x00ea_07a0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 7);
        e.mem.set_u32(other + 0x40, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ea_07a0, &args![this, other]).bool());
        assert!(calls(&e.call_log.take().unwrap(), NI_BOX_EQUAL).is_empty());
        // `operator!=`: the negation of slot +0x24 on this volume.
        e.register(0x0f00_0024, |_, a| u32::from(a[1] == 0x1234).into_ret());
        e.put_vtable(0x0f00_2000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f00_0024]);
        e.mem.set_u32(this, 0x0f00_2000);
        assert!(!e.call(0x00e9_e840, &args![this, 0x1234u32]).bool());
        assert!(e.call(0x00e9_e840, &args![this, 0x1235u32]).bool());
    }

    #[test]
    fn binary_io_runs_the_base_then_the_box() {
        for (addr, base, part) in [
            (
                0x00ea_07e0u32,
                NI_BOUNDING_VOLUME_LOAD_BINARY,
                NI_BOX_LOAD_BINARY,
            ),
            (
                0x00ea_0800,
                NI_BOUNDING_VOLUME_SAVE_BINARY,
                NI_BOX_SAVE_BINARY,
            ),
        ] {
            let mut e = Engine::new();
            e.register(base, nothing);
            e.register(part, nothing);
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x5000u32, 0x6000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [(base, vec![0x5000, 0x6000]), (part, vec![0x5004, 0x6000])]
            );
        }
    }

    #[test]
    fn constructor_copy_and_clone() {
        let mut e = Engine::new();
        let extents = floats(&mut e, &[1.0, 2.0, f32::from_bits(0x7f80_0001)]);
        let center = floats(&mut e, &[4.0, 5.0, 6.0]);
        let x = floats(&mut e, &[1.0, 0.0, 0.0]);
        let y = floats(&mut e, &[0.0, 1.0, 0.0]);
        let z = floats(&mut e, &[0.0, 0.0, 1.0]);
        let this = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00ea_0820, &args![this, extents, center, x, y, z])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), NI_BOX_BV_VTABLE);
        assert_eq!(read(&e, this + 4, 3), vec![4.0, 5.0, 6.0]);
        assert_eq!(
            read(&e, this + 0x10, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x34, 2), vec![1.0, 2.0]);
        // The extents go through FLD/FSTP: the signalling NaN is quieted.
        assert_eq!(e.mem.u32(this + 0x3c), 0x7fc0_0001);
        // Copy: the 15 words, the vtable kept.
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other, 0xdead);
        e.call(0x00ea_08a0, &args![other, this]);
        assert_eq!(e.mem.u32(other), 0xdead);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(other + 4 * word), e.mem.u32(this + 4 * word));
        }
        // Clone: a block from the memory manager with the vtable and words.
        e.register(MEMORY_ALLOCATE, |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 0x40]);
            e.mem.alloc(0x40).into_ret()
        });
        let copy = e.call(0x00ea_7d10, &args![other]).u32();
        assert_eq!(e.mem.u32(copy), NI_BOX_BV_VTABLE);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(copy + 4 * word), e.mem.u32(this + 4 * word));
        }
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00ea_7d10, &args![other]).u32(), 0);
    }

    #[test]
    fn create_and_load_build_the_default_box() {
        let mut e = Engine::new();
        data(&mut e);
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(0x40).into_ret());
        let made = e.call(0x00ea_74f0, &[]).u32();
        assert_eq!(e.mem.u32(made), NI_BOX_BV_VTABLE);
        assert_eq!(read(&e, made + 4, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(
            read(&e, made + 0x10, 12),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]
        );
        // The loader makes the same box and runs its `LoadBinary` (slot 0).
        e.register(0x0f00_0000, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.put_vtable(NI_BOX_BV_VTABLE, &[0x0f00_0000]);
        let loaded = e.call(0x00ea_7490, &args![0x6000u32]).u32();
        assert_eq!(e.mem.u32(loaded + 4), 0x6000);
        assert_eq!(e.mem.f32(loaded + 0x34), 1.0);
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00ea_74f0, &[]).u32(), 0);
    }

    #[test]
    fn update_world_data_moves_the_box() {
        let mut e = Engine::new();
        // A rotation by 90 degrees about z, translation (10, 20, 30),
        // scale 2.
        let transform = floats(
            &mut e,
            &[
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
            ],
        );
        let model = box_volume(
            &mut e,
            [
                1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, 1.5, 2.5,
            ],
        );
        let this = e.mem.alloc(0x40);
        e.call(0x00ea_08c0, &args![this, model, transform]);
        // Centre: R (1,2,3) = (-2, 1, 3), times 2, plus the translation.
        assert_eq!(read(&e, this + 4, 3), vec![6.0, 22.0, 36.0]);
        assert_eq!(
            read(&e, this + 0x10, 9),
            vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x34, 3), vec![1.0, 3.0, 5.0]);
        // In place (the model is the volume itself) gives the same.
        e.call(0x00ea_08c0, &args![model, model, transform]);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(model + 4 * word), e.mem.u32(this + 4 * word));
        }
    }

    #[test]
    fn get_normal_adds_the_signed_axes() {
        let mut e = Engine::new();
        data(&mut e);
        e.register(UNITIZE_004A0C10, nothing);
        let this = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let out = e.mem.alloc(0xc);
        // A still velocity: every signed axis counts; two of them: unitized.
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x00ea_3a60,
                &args![this, out, 1i32, -1i32, 0i32, 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        assert_eq!(back, out);
        assert_eq!(read(&e, out, 3), vec![1.0, -1.0, 0.0]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10),
            vec![vec![out]]
        );
        // Moving along x: only the x axis is above the epsilon; one axis,
        // not unitized.
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_3a60,
            &args![this, out, -1i32, 1i32, 1i32, 2.0f32, 0.0f32, 0.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![-1.0, 0.0, 0.0]);
        assert!(calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10).is_empty());
        // No sign of 1 or -1: nothing applied, unitized (of zero).
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_3a60,
            &args![this, out, 2i32, 0i32, 5i32, 0.0f32, 0.0f32, 1.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10).len(),
            1
        );
        // A NaN velocity is not still and no projection is above.
        e.call(
            0x00ea_3a60,
            &args![this, out, 1i32, 1i32, 1i32, f32::NAN, 0.0f32, 0.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![0.0, 0.0, 0.0]);
    }

    /// Doubles for an intersector at `[ESP+0xc]`: the constructor records
    /// nothing, `Test`/`Find` nothing, the contact time 0.5, the type the
    /// first word of `kind`, the point and normal two fixed triples.
    fn intersector(e: &mut Engine, kind: u32) -> (u32, u32) {
        let point = floats(e, &[1.0, 2.0, 3.0]);
        let normal = floats(e, &[0.0, 1.0, f32::from_bits(0x7f80_0001)]);
        for addr in [
            INTERSECTOR_CONSTRUCTOR_00EB3C40,
            INTERSECTOR_CONSTRUCTOR_00EB3890,
            INTERSECTOR_TEST,
            INTERSECTOR_FIND,
            INTERSECTOR_DESTRUCTOR,
        ] {
            e.register(addr, nothing);
        }
        e.register(INTERSECTOR_CONTACT_TIME, |_, _| st0(0.5));
        e.register_double(INTERSECTOR_TYPE, move |_, _| kind.into_ret());
        e.register_double(INTERSECTOR_CONTACT_POINT, move |_, _| point.into_ret());
        e.register_double(INTERSECTOR_CONTACT_NORMAL, move |_, _| normal.into_ret());
        (point, normal)
    }

    #[test]
    fn finds_run_their_intersector() {
        for (addr, constructor) in [
            (0x00ea_3950u32, INTERSECTOR_CONSTRUCTOR_00EB3C40),
            (0x00ea_3c50, INTERSECTOR_CONSTRUCTOR_00EB3890),
        ] {
            for (kind, hit) in [(3u32, true), (2, true), (1, false)] {
                let mut e = Engine::new();
                data(&mut e);
                intersector(&mut e, kind);
                let outs = e.mem.alloc(0x28);
                let (time, point, normal, negated) = (outs, outs + 4, outs + 0x10, outs + 0x1c);
                e.call_log = Some(vec![]);
                let result = e
                    .call(
                        addr,
                        &args![
                            2.0f32, 0x5000u32, 0x5100u32, 0x5200u32, 0x5300u32, time, point, true,
                            normal, negated
                        ],
                    )
                    .bool();
                assert_eq!(result, hit);
                let log = e.call_log.take().unwrap();
                let made = calls(&log, constructor);
                assert_eq!(
                    &made[0][1..],
                    &[
                        0x5000,
                        0x5200,
                        2.0f32.to_bits(),
                        0.001f32.to_bits(),
                        0.0001f32.to_bits(),
                        0x20
                    ]
                );
                assert_eq!(
                    calls(&log, INTERSECTOR_FIND),
                    vec![vec![made[0][0], 0x5100, 0x5300]]
                );
                assert_eq!(calls(&log, INTERSECTOR_DESTRUCTOR).len(), 1);
                assert_eq!(e.mem.f32(time), 0.5);
                if hit {
                    assert_eq!(read(&e, point, 3), vec![1.0, 2.0, 3.0]);
                    assert_eq!(read(&e, normal, 2), vec![0.0, 1.0]);
                    assert_eq!(e.mem.u32(negated), 0x8000_0000);
                    assert_eq!(e.mem.f32(negated + 4), -1.0);
                    // FLD; FCHS; FSTP: the signalling NaN quieted, negated.
                    assert_eq!(e.mem.u32(negated + 8), 0xffc0_0001);
                }
            }
        }
    }

    #[test]
    fn find_without_normals_leaves_them() {
        let mut e = Engine::new();
        data(&mut e);
        intersector(&mut e, 3);
        let outs = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00ea_3950,
                &args![
                    1.0f32,
                    0x5000u32,
                    0x5100u32,
                    0x5200u32,
                    0x5300u32,
                    outs,
                    outs + 4,
                    0x100u32,
                    outs + 0x10,
                    outs + 0x1c
                ],
            )
            .bool());
        assert!(calls(&e.call_log.take().unwrap(), INTERSECTOR_CONTACT_NORMAL).is_empty());
        assert_eq!(e.mem.u32(outs + 0x10), 0);
    }

    #[test]
    fn capsule_test_accepts_an_inside_centre_else_asks_the_intersector() {
        let mut e = Engine::new();
        data(&mut e);
        intersector(&mut e, 1);
        let b = aligned(&mut e, [0.0; 3], [1.0, 2.0, 3.0]);
        let other = floats(&mut e, &[0.0, 0.5, -1.5, 2.5]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        assert!(calls(&e.call_log.take().unwrap(), INTERSECTOR_TEST).is_empty());
        // On the face (|offset| equal to the extent) still counts.
        e.mem.set_f32(other + 0xc, 3.0);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        // Outside on z: the intersector decides (type 1: no).
        e.mem.set_f32(other + 0xc, 3.5);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let made = calls(&log, INTERSECTOR_CONSTRUCTOR_00EB3890);
        assert_eq!(made[0][1..3], [b, other]);
        assert_eq!(
            calls(&log, INTERSECTOR_TEST),
            vec![vec![made[0][0], 0x5100, 0x5300]]
        );
        assert_eq!(calls(&log, INTERSECTOR_DESTRUCTOR).len(), 1);
        intersector(&mut e, 2);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        // A NaN offset is outside.
        e.mem.set_u32(other + 4, 0x7fc0_0000);
        intersector(&mut e, 0);
        assert!(!e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
    }

    #[test]
    fn sphere_test_measures_the_distance_when_outside() {
        let mut e = Engine::new();
        data(&mut e);
        let b = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // A sphere volume: vtable, centre at +4, radius at +0x10.
        let sphere = floats(&mut e, &[0.0, 0.5, 0.5, 0.5, 2.0]);
        let still = floats(&mut e, &[1.0, 1.0, 1.0]);
        // Inside: true without a distance.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
        // Outside and still (equal velocities): the point distance, 4.0 is
        // the radius squared exactly.
        e.mem.set_f32(sphere + 4, 5.0);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(4.0));
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        let log = e.call_log.take().unwrap();
        let measured = calls(&log, POINT_BOX_DISTANCE_00EB42E0);
        assert_eq!(measured[0][0..2], [sphere + 4, b + 4]);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(4.000_001));
        assert!(!e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        // Moving: the segment from the centre along time * (sphere - box).
        let moving = floats(&mut e, &[1.0, 3.0, 1.0]);
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |e, a| {
            let segment: Vec<f32> = (0..6).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            assert_eq!(segment, vec![5.0, 0.5, 0.5, 0.0, 4.0, 0.0]);
            st0(1.0)
        });
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![2.0f32, b, still, sphere, moving])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SEGMENT_BOX_DISTANCE_00EB50F0)[0][1], b + 4);
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
    }

    #[test]
    fn viewer_strings_and_destructor() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, nothing);
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, nothing);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"bound");
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ea_b7b0, &args![0u32, name, strings]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ALLOCATE), vec![vec![15]]);
        let printed = calls(&log, NI_SPRINTF);
        assert_eq!(printed[0][1..], [15, BOX_BV_FORMAT, name]);
        assert!(calls(&log, VIEWER_STRINGS_GROW).is_empty());
        assert_eq!(calls(&log, VIEWER_STRINGS_SET_AT)[0][1], 2);
        // The scalar deleting destructor.
        e.register(MEMORY_DEALLOCATE, nothing);
        let volume = e.mem.alloc(0x40);
        e.mem.set_u32(volume, NI_BOX_BV_VTABLE);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ea_c1d0, &args![volume, flags]).u32(), volume);
            assert_eq!(e.mem.u32(volume), NI_BOUNDING_VOLUME_VTABLE);
            let log = e.call_log.take().unwrap();
            let freed = calls(&log, MEMORY_DEALLOCATE);
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0], vec![MEMORY_MANAGER, volume]);
            }
        }
    }

    #[test]
    fn contact_point_is_clamped_into_both_boxes() {
        let mut e = Engine::new();
        data(&mut e);
        e.map(PROBE_RADIUS & !0xfff, 0x1000);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.map(CLAMP_FACTOR & !0xfff, 0x1000);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        // Far from the probe sphere: never touching.
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(100.0));
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let time = floats(&mut e, &[0.5]);
        let point = floats(&mut e, &[5.0, 0.25, -3.0]);
        e.call(0x00ea_7730, &args![1.0f32, a, still, b, still, time, point]);
        // Into a: x clamped to 1 (above 0.99), z to -1; then into b: x
        // clamped to 3 - 1 (below -0.99 of b's extent).
        assert_eq!(read(&e, point, 3), vec![2.0, 0.25, -1.0]);
    }

    #[test]
    fn contact_point_moves_with_the_box_and_stays_when_touching() {
        let mut e = Engine::new();
        data(&mut e);
        e.map(PROBE_RADIUS & !0xfff, 0x1000);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.map(CLAMP_FACTOR & !0xfff, 0x1000);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, nothing);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(100.0));
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |_, _| st0(100.0));
        // Box a moves by (2, 0, 0) per unit; at contact time 0.5 its centre
        // is at x 1: the point (5, 0, 0) is clamped to x 2.
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let moving = floats(&mut e, &[2.0, 0.0, 0.0]);
        // Box b contains the original point: left alone.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let time = floats(&mut e, &[0.5]);
        let point = floats(&mut e, &[5.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_7730,
            &args![1.0f32, a, moving, b, still, time, point],
        );
        assert_eq!(read(&e, point, 3), vec![2.0, 0.0, 0.0]);
        // Box a moves relative to the probe: the segment distance; the
        // original point is inside box b: no distance for it.
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SEGMENT_BOX_DISTANCE_00EB50F0).len(), 1);
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
    }

    /// `NiPoint3::Dot` and the cross product `004b3800` over plain floats.
    fn vector_doubles(e: &mut Engine) {
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[1], 3));
            st0(f64::from(p[0] * q[0] + p[1] * q[1] + p[2] * q[2]))
        });
        e.register(POINT_CROSS_004B3800, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[2], 3));
            let out = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            for (i, v) in out.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            a[1].into_ret()
        });
    }

    fn box_box(e: &mut Engine, b: u32, t: f32, velocity: [f32; 3]) -> (bool, Vec<(u32, Vec<u32>)>) {
        let a = aligned(e, [0.0; 3], [1.0; 3]);
        let still = floats(e, &[0.0, 0.0, 0.0]);
        let moving = floats(e, &velocity);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ea_0ad0, &args![t, a, still, b, moving]).bool();
        let log = e.call_log.take().unwrap();
        (result, log)
    }

    #[test]
    fn box_box_test_separates_on_the_first_axis() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_DOT).is_empty());
        // Also on the negative side.
        let b = aligned(&mut e, [-3.0, 0.0, 0.0], [1.0; 3]);
        assert!(!box_box(&mut e, b, 1.0, [0.0; 3]).0);
    }

    #[test]
    fn box_box_test_overlap_and_motion() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // Overlapping: every axis passes, the motion axes included (with a
        // zero velocity the cross product is zero).
        let b = aligned(&mut e, [1.0, 0.5, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [0.0; 3]);
        assert!(hit);
        assert_eq!(calls(&log, POINT_CROSS_004B3800).len(), 1);
        assert_eq!(calls(&log, POINT_DOT).len(), 12);
        // Moving through the box along x: apart at the start (3 > 2) and on
        // the other side at the end: not separated.
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        assert!(box_box(&mut e, b, 1.0, [-6.0, 0.0, 0.0]).0);
        // Moving along x but 3 apart in y: separated on the second axis.
        let b = aligned(&mut e, [0.0, 3.0, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [5.0, 0.0, 0.0]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        // Moving towards it but not far enough in the time: still apart.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        assert!(!box_box(&mut e, b, 1.0, [-2.0, 0.0, 0.0]).0);
        assert!(box_box(&mut e, b, 2.0, [-2.0, 0.0, 0.0]).0);
    }

    #[test]
    fn box_box_test_separates_on_the_other_boxs_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let c = std::f32::consts::FRAC_1_SQRT_2;
        // A thin strip along the anti-diagonal: only its own first axis
        // (the diagonal) separates it.
        let strip = box_volume(
            &mut e,
            [
                2.2, 2.2, 0.0, c, c, 0.0, -c, c, 0.0, 0.0, 0.0, 1.0, 0.1, 3.0, 1.0,
            ],
        );
        let (hit, log) = box_box(&mut e, strip, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_DOT).is_empty());
        // The same strip with its axes swapped, on the negative side: the
        // second axis separates, its end projection through `Dot`.
        let strip = box_volume(
            &mut e,
            [
                -2.2, -2.2, 0.0, -c, c, 0.0, c, c, 0.0, 0.0, 0.0, 1.0, 3.0, 0.1, 1.0,
            ],
        );
        let (hit, log) = box_box(&mut e, strip, 1.0, [0.0; 3]);
        assert!(!hit);
        let dots = calls(&log, POINT_DOT);
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0][0], strip + 0x1c);
    }

    fn box_triangle(
        e: &mut Engine,
        vertices: [[f32; 3]; 3],
        t: f32,
        velocity: [f32; 3],
    ) -> (bool, Vec<(u32, Vec<u32>)>) {
        let b = aligned(e, [0.0; 3], [1.0; 3]);
        let still = floats(e, &[0.0, 0.0, 0.0]);
        let moving = floats(e, &velocity);
        let [p0, p1, p2] = vertices.map(|v| floats(e, &v));
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ea_2190, &args![t, b, still, p0, p1, p2, moving])
            .bool();
        let log = e.call_log.take().unwrap();
        (result, log)
    }

    #[test]
    fn box_triangle_test_separates_on_the_normal_and_the_box_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // A triangle in the plane x = 3 facing the box: its normal separates.
        let facing = [[3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, facing, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        // Edge-on (normal along y): the box's first axis separates, on the
        // positive side and on the negative side.
        let edge_on = [[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, edge_on, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        let behind = [[-3.0, 0.0, 0.0], [-4.0, 0.0, 0.0], [-3.0, 0.0, 1.0]];
        assert!(!box_triangle(&mut e, behind, 1.0, [0.0; 3]).0);
    }

    #[test]
    fn box_triangle_test_follows_the_motion() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // Moving through the box: no axis separates; every product axis is
        // asked, the second axis's last ones through `Dot`.
        let facing = [[3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, facing, 1.0, [-5.0, 0.0, 0.0]);
        assert!(hit);
        let crosses = calls(&log, POINT_CROSS_004B3800);
        assert_eq!(crosses.len(), 4);
        assert_eq!(calls(&log, POINT_DOT).len(), 2);
        // `004b3800` on the box's second axis with `E1`, a copy of
        // `v1 - v0`.
        let box_at = crosses[0][0] - 0x1c;
        assert_eq!(read(&e, crosses[0][2], 3), vec![0.0, 1.0, 0.0]);
        assert_eq!(crosses[2][0], box_at + 0x28);
        let edge_on = [[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 1.0]];
        assert!(box_triangle(&mut e, edge_on, 1.0, [-5.0, 0.0, 0.0]).0);
        // Too slow to reach the box in the time.
        assert!(!box_triangle(&mut e, edge_on, 1.0, [-1.0, 0.0, 0.0]).0);
        assert!(box_triangle(&mut e, edge_on, 2.0, [-1.0, 0.0, 0.0]).0);
    }

    #[test]
    fn box_triangle_test_separates_on_an_edge_product() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // A flat triangle past the box's corner edge at x = y = 1: only the
        // box's third axis times the first edge, `(10, 10, 0)`, separates.
        let corner = [[-3.75, 6.25, 0.0], [6.25, -3.75, 0.0], [3.0, 3.0, 0.0]];
        let (hit, log) = box_triangle(&mut e, corner, 1.0, [0.0; 3]);
        assert!(!hit);
        assert_eq!(calls(&log, POINT_CROSS_004B3800).len(), 3);
        assert!(calls(&log, POINT_DOT).is_empty());
        // Moving towards the box it is no longer apart at the end.
        let (hit, log) = box_triangle(&mut e, corner, 1.0, [-5.0, -5.0, 0.0]);
        assert!(hit);
        assert_eq!(calls(&log, POINT_DOT).len(), 2);
    }

    fn read3(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn write3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (k, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * k as u32, *x);
        }
    }

    /// Everything `fn_00ea7d40` and the contact point clamp read: the data
    /// floats, the vector helpers over plain floats, and the probe tests
    /// answering "touching" (distance 0), so the point is never clamped.
    fn find_world(e: &mut Engine) {
        data(e);
        vector_doubles(e);
        for page in [
            0x010c_9000u32,
            0x0101_6000,
            0x0101_2000,
            0x0109_5000,
            0x0101_3000,
            0x0106_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(TIME_SLACK_010C9300, 0x3e80_0000_0000_0000);
        e.mem.set_f32(HALF_01016248, 0.5);
        e.mem.set_u64(ONE_01012070, 1.0f64.to_bits());
        e.mem.set_f32(PARALLEL_LIMIT_010959F4, 1e-6);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, nothing);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(0.0));
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |_, _| st0(0.0));
        e.register(POINT_SUM_00439E90, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_DIFFERENCE_00439EF0, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_SCALE_004A3760, |e, a| {
            let (s, p) = (f32::from_bits(a[1]), read3(e, a[2]));
            write3(e, a[0], [s * p[0], s * p[1], s * p[2]]);
            a[0].into_ret()
        });
        e.register(POINT_LENGTH_SQUARED_004A7290, |e, a| {
            let p = read3(e, a[0]);
            st0(f64::from(p[2] * p[2] + p[1] * p[1] + p[0] * p[0]))
        });
        e.register(UNITIZE_004A0C10, |e, a| {
            let p = read3(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write3(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
    }

    /// What one find leaves: the answer, the contact time, point, normal
    /// and negated normal (words; `0x7777_7777` where nothing was stored)
    /// and the call log.
    struct Found {
        hit: bool,
        time: f32,
        point: [f32; 3],
        normal: [u32; 3],
        negated: [u32; 3],
        log: Vec<(u32, Vec<u32>)>,
    }

    fn find(
        e: &mut Engine,
        time: f32,
        a: u32,
        va: [f32; 3],
        b: u32,
        vb: [f32; 3],
        want: bool,
    ) -> Found {
        let (va, vb) = (floats(e, &va), floats(e, &vb));
        let outs = e.mem.alloc(0x28);
        for word in 0..10 {
            e.mem.set_u32(outs + 4 * word, 0x7777_7777);
        }
        e.call_log = Some(vec![]);
        let hit = e
            .call(
                0x00ea_7d40,
                &args![
                    time,
                    a,
                    va,
                    b,
                    vb,
                    outs,
                    outs + 4,
                    want,
                    outs + 0x10,
                    outs + 0x1c
                ],
            )
            .bool();
        let words = |at: u32| [0u32, 4, 8].map(|k| e.mem.u32(at + k));
        Found {
            hit,
            time: e.mem.f32(outs),
            point: read3(e, outs + 4),
            normal: words(outs + 0x10),
            negated: words(outs + 0x1c),
            log: e.call_log.take().unwrap(),
        }
    }

    fn bits(v: [f32; 3]) -> [u32; 3] {
        v.map(f32::to_bits)
    }

    fn close(found: [f32; 3], want: [f32; 3]) -> bool {
        found.iter().zip(want).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    const STILL: [f32; 3] = [0.0; 3];

    #[test]
    fn box_box_find_separated_answers_false() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // Apart on `a`'s first axis, on both sides, and still apart after
        // a motion too short to close the gap.
        for (centre, velocity) in [
            ([5.0, 0.0, 0.0], STILL),
            ([-5.0, 0.0, 0.0], STILL),
            ([5.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            ([-5.0, 0.0, 0.0], [2.5, 0.0, 0.0]),
            ([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ] {
            let b = aligned(&mut e, centre, [1.0; 3]);
            let found = find(&mut e, 1.0, a, STILL, b, velocity, true);
            assert!(!found.hit);
            // The contact time is cleared first; nothing else is stored.
            assert_eq!(found.time, 0.0);
            assert_eq!(found.normal, [0x7777_7777; 3]);
            assert!(calls(&found.log, POINT_DOT).is_empty());
        }
    }

    #[test]
    fn box_box_find_overlap_takes_the_midpoint() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // Overlapping at the start: case 0, the contact time stays 0.
        let b = aligned(&mut e, [1.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert!(found.hit);
        assert_eq!(found.time, 0.0);
        assert_eq!(found.point, [0.5, 0.0, 0.0]);
        // The normal is the point minus `a`'s centre, unitized (its squared
        // length 0.25 is off 1 and above the epsilon).
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        let log = &found.log;
        let added = calls(log, POINT_SUM_00439E90);
        assert_eq!([added[0][0], added[0][2]], [a + 4, b + 4]);
        let scaled = calls(log, POINT_SCALE_004A3760);
        assert_eq!(scaled[0][1], 0.5f32.to_bits());
        assert_eq!(scaled[0][2], added[0][1]);
        assert_eq!(calls(log, POINT_DIFFERENCE_00439EF0)[0][2], a + 4);
        assert_eq!(calls(log, UNITIZE_004A0C10).len(), 1);
        // A normal of length 1 is left alone.
        let b = aligned(&mut e, [2.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert_eq!(found.point, [1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert!(calls(&found.log, UNITIZE_004A0C10).is_empty());
        // Coinciding centres: a zero normal becomes the default x axis.
        let b = aligned(&mut e, [0.0; 3], [0.5; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert_eq!(found.point, [0.0; 3]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, UNITIZE_004A0C10).is_empty());
        // Without normals: the point only.
        let b = aligned(&mut e, [1.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, false);
        assert_eq!(found.point, [0.5, 0.0, 0.0]);
        assert_eq!(found.normal, [0x7777_7777; 3]);
        assert!(calls(&found.log, POINT_DIFFERENCE_00439EF0).is_empty());
    }

    #[test]
    fn box_box_find_face_of_the_first_box() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // `b` comes along x and meets `a`'s face x = 1 at 3/4 of the time
        // (`b`'s own x axis meets at the same time: not later by the slack).
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.75);
        // On `b`'s face: its centre at the contact time minus its extent.
        assert_eq!(found.point, [1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, POINT_CROSS_004B3800).is_empty());
        assert!(calls(&found.log, POINT_LENGTH_SQUARED_004A7290).is_empty());
        // From the other side: the negative side, the negated axis.
        let b = aligned(&mut e, [-5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.75);
        assert_eq!(found.point, [-1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([-1.0, -0.0, -0.0]));
        assert_eq!(found.negated, bits([1.0, 0.0, 0.0]));
        // `a` moving instead, on y, for twice the time: the point on the
        // still `b`'s face.
        let b = aligned(&mut e, [0.0, 5.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 2.0, a, [0.0, 2.0, 0.0], b, STILL, false);
        assert!(found.hit);
        assert_eq!(found.time, 1.5);
        assert_eq!(found.point, [0.0, 4.0, 0.0]);
        assert_eq!(found.normal, [0x7777_7777; 3]);
    }

    /// `a` turned 45 degrees about z and then about x (no axis of it is
    /// perpendicular to x), extents 1, at the origin.
    fn tilted(e: &mut Engine) -> u32 {
        let h = std::f32::consts::FRAC_1_SQRT_2;
        box_volume(
            e,
            [
                0.0, 0.0, 0.0, h, h, 0.0, -0.5, 0.5, h, 0.5, -0.5, h, 1.0, 1.0, 1.0,
            ],
        )
    }

    #[test]
    fn box_box_find_face_of_the_second_box() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = tilted(&mut e);
        // `b` (aligned) comes along x: `a`'s corner hits `b`'s face x = -1.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        let reach = std::f32::consts::FRAC_1_SQRT_2 + 1.0;
        assert!((found.time - (4.0 - reach) / 4.0).abs() < 1e-6);
        // The corner of `a` reaching furthest along x: A0 - A1 + A2.
        let h = std::f32::consts::FRAC_1_SQRT_2;
        assert!(close(found.point, [h + 1.0, h - 1.0, 0.0]));
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, POINT_CROSS_004B3800).is_empty());
        // From the other side: the corner A1 - A0 - A2, the negated axis.
        let b = aligned(&mut e, [-5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!(close(found.point, [-h - 1.0, 1.0 - h, 0.0]));
        assert_eq!(found.normal, bits([-1.0, -0.0, -0.0]));
    }

    /// `a` turned 45 degrees about z (an edge along z leading in x), `b`
    /// 45 degrees about y (an edge along y leading in -x), `b` at `centre`.
    fn crossed_boxes(e: &mut Engine, centre: [f32; 3]) -> (u32, u32) {
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let a = box_volume(
            e,
            [
                0.0, 0.0, 0.0, h, h, 0.0, -h, h, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
            ],
        );
        let b = box_volume(
            e,
            [
                centre[0], centre[1], centre[2], h, 0.0, -h, 0.0, 1.0, 0.0, h, 0.0, h, 1.0, 1.0,
                1.0,
            ],
        );
        (a, b)
    }

    #[test]
    fn box_box_find_edge_against_edge() {
        let mut e = Engine::new();
        find_world(&mut e);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        // The edges meet when `b` has moved 5 - 2 sqrt 2: on the axis
        // A2 x B1 = -x, the negative side (case 14).
        let (a, b) = crossed_boxes(&mut e, [5.0, 0.0, 0.5]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!((found.time - (5.0 - 4.0 * h) / 4.0).abs() < 1e-6);
        // On `a`'s edge, at `b`'s edge's height: `a`'s own offset along z
        // solved from the two edges (0.5).
        assert!(close(found.point, [2.0 * h, 0.0, 0.5]));
        let log = &found.log;
        let scaled = calls(log, POINT_SCALE_004A3760);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][1], found.time.to_bits());
        let dots = calls(log, POINT_DOT);
        assert_eq!([dots[0][0], dots[1][0]], [b + 0x1c, a + 0x28]);
        assert_eq!(dots[0][1], scaled[0][0] - 0x144 + 0x54);
        // The normal: -(A2 x B1) through `004b3800`, already of length 1.
        let crossed = calls(log, POINT_CROSS_004B3800);
        assert_eq!([crossed[0][0], crossed[0][2]], [a + 0x28, b + 0x1c]);
        assert_eq!(crossed[0][1], scaled[0][0] - 0x144 + 0x15c);
        assert_eq!(found.normal, bits([1.0, -0.0, -0.0]));
        assert_eq!(found.negated, bits([-1.0, 0.0, 0.0]));
        assert!(calls(log, UNITIZE_004A0C10).is_empty());
        // From the other side: the positive side, the product itself.
        let (a, b) = crossed_boxes(&mut e, [-5.0, 0.0, -0.5]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!(close(found.point, [-2.0 * h, 0.0, -0.5]));
        assert_eq!(found.normal, bits([-1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([1.0, -0.0, -0.0]));
        let crossed = calls(&found.log, POINT_CROSS_004B3800);
        let scaled = calls(&found.log, POINT_SCALE_004A3760);
        assert_eq!(crossed[0][1], scaled[0][0] - 0x144 + 0x174);
    }

    #[test]
    fn box_box_find_parallel_edges_skip_the_solve() {
        let mut e = Engine::new();
        find_world(&mut e);
        // Case 7 with `a`'s and `b`'s first axes parallel (`1 - c00^2` is
        // 0): `a`'s offset along it is 0, nothing is projected, and the zero
        // product of the axes becomes the default normal.
        let a = aligned(&mut e, [1.0, 2.0, 3.0], [1.0; 3]);
        let b = aligned(&mut e, [4.0, 0.0, 0.0], [1.0; 3]);
        let (va, vb, ot, point) = (
            floats(&mut e, &[2.0, 0.0, 0.0]),
            floats(&mut e, &[0.0; 3]),
            floats(&mut e, &[0.5]),
            e.mem.alloc(0x24),
        );
        let s = BoxFind {
            time: 1.0f32.to_bits(),
            a,
            va,
            b,
            vb,
            out_time: ot,
            point,
            want_normals: true,
            normal: point + 0xc,
            negated: point + 0x18,
            frame: e.mem.alloc(FIND_FRAME),
            c: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            t: [3.0, -2.0, -3.0],
            axis: 7,
            side: 1,
        };
        e.call_log = Some(vec![]);
        assert!(find_edge(&mut e, &s, &EDGE_CASES[0]));
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, POINT_SCALE_004A3760).is_empty());
        assert!(calls(&log, POINT_DOT).is_empty());
        let crossed = calls(&log, POINT_CROSS_004B3800);
        assert_eq!(crossed, vec![vec![a + 0x10, s.frame + 0x54, b + 0x10]]);
        // `a`'s centre moved by half its velocity; every offset is 0.
        assert_eq!(read3(&e, point), [2.0, 2.0, 3.0]);
        assert_eq!(read3(&e, point + 0xc), [1.0, 0.0, 0.0]);
        assert_eq!(read3(&e, point + 0x18), [-1.0, -0.0, -0.0]);
    }

    #[test]
    fn box_box_find_signed_extents_and_the_slack() {
        let mut e = Engine::new();
        let x = e.mem.alloc(8);
        e.mem.set_f32(x, 2.0);
        e.mem.set_u32(x + 4, 0x7f80_0001);
        assert_eq!(signed_extent(&e, 0.5, 1, x), 2.0);
        assert_eq!(signed_extent(&e, -0.5, 1, x), -2.0);
        assert_eq!(signed_extent(&e, 0.5, -1, x), -2.0);
        assert_eq!(signed_extent(&e, 0.0, 1, x), 0.0);
        assert_eq!(signed_extent(&e, f64::NAN, 1, x), 0.0);
        // A signalling NaN extent comes back quiet.
        assert_eq!(x87_store32(signed_extent(&e, 1.0, 1, x + 4)), 0x7fc0_0001);
        // `find_axis`: a later meeting not later by the slack keeps the axis.
        find_world(&mut e);
        let ot = floats(&mut e, &[0.5]);
        let mut s = BoxFind {
            time: 0,
            a: 0,
            va: 0,
            b: 0,
            vb: 0,
            out_time: ot,
            point: 0,
            want_normals: false,
            normal: 0,
            negated: 0,
            frame: 0,
            c: [[0.0; 3]; 3],
            t: [0.0; 3],
            axis: 3,
            side: 1,
        };
        // (2 - 4) / (0 - 4) = 0.5: equal, not taken.
        assert!(!find_axis(&mut e, &mut s, 5, 2.0, 4.0, || 0.0, 1.0, 1.0));
        assert_eq!((s.axis, s.side), (3, 1));
        // From below: (2 - 5) * -1 / (-1 + 5) = 0.75: taken, side -1.
        assert!(!find_axis(&mut e, &mut s, 6, 2.0, -5.0, || -1.0, 1.0, 1.0));
        assert_eq!((s.axis, s.side), (6, -1));
        assert_eq!(e.mem.f32(ot), 0.75);
        // Apart from below for the whole motion: separated.
        assert!(find_axis(&mut e, &mut s, 8, 2.0, -5.0, || -3.0, 1.0, 1.0));
        // A NaN offset takes neither side.
        assert!(!find_axis(
            &mut e,
            &mut s,
            9,
            2.0,
            f64::NAN,
            || 0.0,
            1.0,
            1.0
        ));
        assert_eq!(s.axis, 6);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    const SENTINEL: u32 = 0x7777_7777;

    fn read3(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn write3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (k, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * k as u32, *x);
        }
    }

    fn point(e: &mut Engine, v: [f32; 3]) -> u32 {
        let p = e.mem.alloc(0xc);
        write3(e, p, v);
        p
    }

    /// The point helpers over plain floats, and the two data floats the find reads (0.5 at
    /// `01016248`, 1/3 at `011c1340`).
    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x011c_1000, 0x1000);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_u32(0x011c_1340, 0x3eaa_aaab);
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[1]));
            Ret {
                st0: f64::from(p[0] * q[0] + p[1] * q[1] + p[2] * q[2]),
                ..Ret::default()
            }
        });
        e.register(POINT_CROSS_004B3800, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            let out = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            write3(e, a[1], out);
            a[1].into_ret()
        });
        e.register(POINT_SUM_00439E90, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_SCALE_004A3760, |e, a| {
            let (s, p) = (f32::from_bits(a[1]), read3(e, a[2]));
            write3(e, a[0], [s * p[0], s * p[1], s * p[2]]);
            a[0].into_ret()
        });
        e.register(0x0063_c8a0, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[1]));
            write3(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[0].into_ret()
        });
        e.register(UNITIZE_004A0C10, |e, a| {
            let p = read3(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write3(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
        e
    }

    /// An axis-aligned box volume (vtable word, centre, three axes, three extents).
    fn aligned(e: &mut Engine, center: [f32; 3], extent: [f32; 3]) -> u32 {
        let b = e.mem.alloc(0x40);
        let values = [
            center[0], center[1], center[2], 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            extent[0], extent[1], extent[2],
        ];
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(b + 4 + 4 * i as u32, *v);
        }
        b
    }

    struct Found {
        hit: bool,
        time: f32,
        point: [u32; 3],
        normal: [u32; 3],
        negated: [u32; 3],
    }

    type Triangle = [[f32; 3]; 3];

    fn find(e: &mut Engine, b: u32, tri: Triangle, velocity: [f32; 3], normals: bool) -> Found {
        let still = point(e, [0.0; 3]);
        let moving = point(e, velocity);
        let v: Vec<u32> = tri.iter().map(|p| point(e, *p)).collect();
        let outs: Vec<u32> = (0..4)
            .map(|_| {
                let p = e.mem.alloc(0xc);
                for k in 0..3 {
                    e.mem.set_u32(p + 4 * k, SENTINEL);
                }
                p
            })
            .collect();
        let hit = e
            .call(
                0x00ea_3d60,
                &args![
                    1.0f32,
                    b,
                    still,
                    v[0],
                    v[1],
                    v[2],
                    moving,
                    outs[0],
                    outs[1],
                    u32::from(normals),
                    outs[2],
                    outs[3]
                ],
            )
            .bool();
        let words = |e: &Engine, p: u32| [e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8)];
        Found {
            hit,
            time: e.mem.f32(outs[0]),
            point: words(e, outs[1]),
            normal: words(e, outs[2]),
            negated: words(e, outs[3]),
        }
    }

    fn test(e: &mut Engine, b: u32, tri: Triangle, velocity: [f32; 3]) -> bool {
        let still = point(e, [0.0; 3]);
        let moving = point(e, velocity);
        let v: Vec<u32> = tri.iter().map(|p| point(e, *p)).collect();
        e.call(
            0x00ea_2190,
            &args![1.0f32, b, still, v[0], v[1], v[2], moving],
        )
        .bool()
    }

    #[test]
    fn fn_00ea3d60_apart_touching_and_moving() {
        let mut e = world();
        let b = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let far = [[5.0, -1.0, -1.0], [5.0, 1.0, -1.0], [5.0, 0.0, 1.0]];
        assert!(!find(&mut e, b, far, [0.0; 3], true).hit);
        // Through the box, standing still: met at time 0, a point stored.
        let through = [[0.5, -2.0, -2.0], [0.5, 2.0, -2.0], [0.5, 0.0, 2.0]];
        let found = find(&mut e, b, through, [0.0; 3], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.0);
        assert_ne!(found.point, [SENTINEL; 3]);
        // Moving onto the box along -x from x = 3 at speed 4: met at time 0.5.
        let ahead = [[3.0, -2.0, -2.0], [3.0, 2.0, -2.0], [3.0, 0.0, 2.0]];
        let found = find(&mut e, b, ahead, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.5);
        // Without normals they stay untouched.
        let found = find(&mut e, b, ahead, [-4.0, 0.0, 0.0], false);
        assert!(found.hit);
        assert_eq!(
            (found.normal, found.negated),
            ([SENTINEL; 3], [SENTINEL; 3])
        );
        // Too slow to arrive: apart.
        assert!(!find(&mut e, b, ahead, [-1.0, 0.0, 0.0], true).hit);
    }

    /// The find agrees with the moving box-triangle test `00ea2190` on whether the two meet,
    /// over a spread of triangles and velocities.
    #[test]
    fn fn_00ea3d60_agrees_with_the_test() {
        let mut e = world();
        let b = aligned(&mut e, [0.25, -0.5, 0.0], [1.0, 0.75, 1.5]);
        let mut seed = 0x1234_5678u32;
        let mut next = || {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            ((seed >> 8) % 801) as f32 / 100.0 - 4.0
        };
        let (mut hits, mut misses) = (0, 0);
        for _ in 0..400 {
            let tri = [
                [next(), next(), next()],
                [next(), next(), next()],
                [next(), next(), next()],
            ];
            let velocity = [next(), next(), next()];
            let found = find(&mut e, b, tri, velocity, true);
            let tested = test(&mut e, b, tri, velocity);
            assert_eq!(found.hit, tested, "{tri:?} {velocity:?}");
            if found.hit {
                hits += 1;
                assert!((0.0..=1.0).contains(&found.time), "{}", found.time);
            } else {
                misses += 1;
            }
        }
        assert!(hits > 20 && misses > 20, "{hits} {misses}");
    }
}
