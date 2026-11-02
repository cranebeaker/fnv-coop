//! `fallout shared/multiboundmarkerdata.cpp` (Xbox PDB source unit), subsystem
//! `fallout shared`: functions from FalloutNV.exe 1.4.0.525, translated per
//! `docs/ENGINE_CRATE.md`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiPoint3, NiTPointerList};

const ALLOCATE: u32 = 0x00aa_13e0;
const DEALLOCATE: u32 = 0x00aa_1460;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const MULTI_BOUND_GET_FORM: u32 = 0x007a_f430;
const MULTI_BOUND_FORM_A: u32 = 0x011c_a230;
const MULTI_BOUND_FORM_B: u32 = 0x011c_a238;
const DEFAULT_HALF_EXTENTS: u32 = 0x011c_3ae4;
const MULTI_BOUND_DATA_READ: u32 = 0x0040_1080;
const MULTI_BOUND_SAVE_CHUNK: u32 = 0x0048_5710;
const MULTI_BOUND_GET_CHUNK: u32 = 0x0047_26b0;
const MULTI_BOUND_READ_CHUNK_DATA: u32 = 0x0047_2890;
const MULTI_BOUND_READ_MODE: u32 = 0x0040_1680;
const NI_PROPERTY_BASE_CONSTRUCT: u32 = 0x00a5_c200;
const NI_PROPERTY_BASE_DESTRUCT: u32 = 0x00a5_c240;
const NI_ALPHA_FLAGS_SET: u32 = 0x0063_3ba0;
const VECTOR_SET: u32 = 0x0041_6870;
const VECTOR_MULTIBOUND_INTERSECT: u32 = 0x004b_1e80;

layout! {
    /// `MultiBoundMarkerData` (Xbox PDB), 0x0C bytes on PC: three half extents.
    pub struct MultiBoundMarkerData: 0x0C {
        /// `HalfExtents` (Xbox PDB): `NiPoint3`.
        0x00 HalfExtents: Inline<NiPoint3>,
    }
}

layout! {
    /// `NiProperty` base size from the Xbox PDB; the member body reads only its vtable.
    pub struct NiProperty: 0x18 {}
}

layout! {
    /// `NiAlphaProperty` (Xbox PDB), 0x1C bytes; PC offsets match the code.
    pub struct NiAlphaProperty: 0x1C {
        0x18 m_uFlags: u16,
        0x1A m_ucAlphaTestRef: u8,
    }
}

fn read_vector(e: &Engine, p: Ptr<()>) -> [f32; 3] {
    [
        e.mem.f32(p.addr()),
        e.mem.f32(p.addr() + 4),
        e.mem.f32(p.addr() + 8),
    ]
}

fn write_vector(e: &mut Engine, p: Ptr<()>, v: [f32; 3]) {
    e.mem.set_f32(p.addr(), v[0]);
    e.mem.set_f32(p.addr() + 4, v[1]);
    e.mem.set_f32(p.addr() + 8, v[2]);
}

// Translated from 00438f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MultiBoundMarkerData`'s constructor (Xbox PDB): calls the base constructor,
/// then copies the three default half extents from the executable's data.
pub fn multi_bound_marker_data_construct(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
) -> Ptr<MultiBoundMarkerData> {
    e.call(0x0068_15c0, &args![this]);
    for offset in [0, 4, 8] {
        let value = e.mem.u32(DEFAULT_HALF_EXTENTS + offset);
        e.mem.set_u32(this.addr() + offset, value);
    }
    this
}

// Translated from 00438f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the `MultiBoundMarkerData` chunk (`XMBO`, 12 bytes) only when the
/// values differ from the default marker extents.
pub fn multi_bound_marker_data_save(e: &mut Engine, this: Ptr<MultiBoundMarkerData>) {
    if !multi_bound_marker_data_is_default(e, this) {
        e.call(MULTI_BOUND_SAVE_CHUNK, &args![0x4f42_4d58u32, this, 3u32]);
    }
}

// Translated from 00438fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a 12-byte `XMBO` chunk into this marker record and reports whether
/// the chunk was present. The load-mode check clears each float via the
/// executable's data reader when the reader says to zero it.
pub fn multi_bound_marker_data_load(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    file: Ptr<()>,
) -> bool {
    if file.is_null() || e.call(MULTI_BOUND_GET_CHUNK, &args![file]).u32() != 0x4f42_4d58 {
        return false;
    }
    e.call(MULTI_BOUND_READ_CHUNK_DATA, &args![file, this, 12u32]);
    if e.call(MULTI_BOUND_READ_MODE, &args![file]).bool() {
        for offset in [0, 4, 8] {
            e.call(MULTI_BOUND_DATA_READ, &args![this.addr() + offset, 0u32]);
        }
    }
    true
}

// Translated from 00439060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MultiBoundMarkerData::Compare` (Xbox PDB): true for a null other record
/// or when its three half extents differ from this record.
pub fn multi_bound_marker_data_compare(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    other: Ptr<MultiBoundMarkerData>,
) -> bool {
    other.is_null() || !multi_bound_marker_data_extents_equal(e, this, other)
}

// Translated from 00439090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether this record's three floats differ from the supplied record.
pub fn multi_bound_marker_data_differs(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    other: Ptr<MultiBoundMarkerData>,
) -> bool {
    !multi_bound_marker_data_extents_equal(e, this, other)
}

// Translated from 004390c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares all three half-extent floats using the game's x87 equality checks.
pub fn multi_bound_marker_data_extents_equal(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    other: Ptr<MultiBoundMarkerData>,
) -> bool {
    (0..3).all(|i| {
        let offset = i * 4;
        e.mem.f32(this.addr() + offset) == e.mem.f32(other.addr() + offset)
    })
}

// Translated from 00439120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three half extents when the source pointer is non-null.
pub fn multi_bound_marker_data_copy(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    other: Ptr<MultiBoundMarkerData>,
) {
    if !other.is_null() {
        for offset in [0, 4, 8] {
            let value = e.mem.u32(other.addr() + offset);
            e.mem.set_u32(this.addr() + offset, value);
        }
    }
}

// Translated from 00439150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns whether this record equals the default marker half extents.
pub fn multi_bound_marker_data_is_default(e: &mut Engine, this: Ptr<MultiBoundMarkerData>) -> bool {
    !multi_bound_marker_data_differs(e, this, Ptr::new(DEFAULT_HALF_EXTENTS))
}

// Translated from 00439180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales each half extent by the supplied float and returns this record.
pub fn multi_bound_marker_data_scale(
    e: &mut Engine,
    this: Ptr<MultiBoundMarkerData>,
    scale: f32,
) -> Ptr<MultiBoundMarkerData> {
    for offset in [0, 4, 8] {
        // `FLD` the half extent, `FMUL` the float argument in memory.
        let value = x87_load32(e.mem.u32(this.addr() + offset));
        let product = x87_mul(value, x87_m32(scale.to_bits()));
        e.mem.set_u32(this.addr() + offset, x87_store32(product));
    }
    this
}

// open: C++ exception unwinding and constructor cleanup are omitted.
// Translation incomplete: 004391c0 - C++ exception unwinding and cleanup on throw.
// Translated from 004391c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the engine's default `NiAlphaProperty` instance at this address.
/// The executable's SEH unwinding frame is omitted; normal constructor calls
/// and the object's initial values are preserved.
pub fn ni_alpha_property_construct(
    e: &mut Engine,
    this: Ptr<NiAlphaProperty>,
) -> Ptr<NiAlphaProperty> {
    ni_property_construct(e, this.cast());
    e.mem.set_u32(this.addr(), 0x0101_62dc);
    e.mem.set_u16(this.addr() + 0x18, 0);
    e.call(0x0049_ed90, &args![this, 0u32]);
    ni_alpha_property_set_field_1(e, this, 6);
    ni_alpha_property_set_field_5(e, this, 7);
    ni_alpha_property_set_flag_9(e, this, 0);
    ni_alpha_property_set_field_10(e, this, 0);
    e.call(0x0094_db80, &args![this, 0u32]);
    this
}

// Translated from 00439260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the `NiProperty` base and installs its vtable.
pub fn ni_property_construct(e: &mut Engine, this: Ptr<NiProperty>) -> Ptr<NiProperty> {
    e.call(NI_PROPERTY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_6374);
    this
}

// Translated from 00439280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the executable's `NiProperty` RTTI object.
pub fn ni_property_get_rtti(_e: &mut Engine, _this: Ptr<NiProperty>) -> Ptr<()> {
    Ptr::new(0x011f_4420)
}

// Translated from 00439290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiProperty` scalar deleting destructor: runs the destructor and frees
/// 0x18 bytes only when `flags & 1` is set.
pub fn ni_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiProperty>,
    flags: u32,
) -> Ptr<NiProperty> {
    ni_property_destruct(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 004392c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the `NiProperty` base destructor.
pub fn ni_property_destruct(e: &mut Engine, this: Ptr<NiProperty>) {
    e.call(NI_PROPERTY_BASE_DESTRUCT, &args![this]);
}

// Translated from 004392e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the executable's `NiAlphaProperty` RTTI object.
pub fn ni_alpha_property_get_rtti(_e: &mut Engine, _this: Ptr<NiAlphaProperty>) -> Ptr<()> {
    Ptr::new(0x011f_4408)
}

// Translated from 004392f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty` scalar deleting destructor: runs its destructor and
/// frees 0x1C bytes only when `flags & 1` is set.
pub fn ni_alpha_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiAlphaProperty>,
    flags: u32,
) -> Ptr<NiAlphaProperty> {
    ni_alpha_property_destruct(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00439320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the `NiAlphaProperty` base destructor chain.
pub fn ni_alpha_property_destruct(e: &mut Engine, this: Ptr<NiAlphaProperty>) {
    ni_property_destruct(e, this.cast());
}

// Translated from 00439340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the alpha property's four-bit field at bits 1–4.
pub fn ni_alpha_property_set_field_1(e: &mut Engine, this: Ptr<NiAlphaProperty>, value: u16) {
    ni_alpha_property_update_flags(e, this, value, 0x001e, 1);
}

// Translated from 00439360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the selected shifted bits in `NiAlphaProperty::m_uFlags`.
pub fn ni_alpha_property_update_flags(
    e: &mut Engine,
    this: Ptr<NiAlphaProperty>,
    value: u16,
    mask: u16,
    shift: u32,
) {
    let flags = e.mem.u16(this.addr() + 0x18);
    // The executable promotes the u16 argument to a 32-bit register, shifts
    // with x86's five-bit count mask, then truncates when storing the flags.
    let value = (u32::from(value) << (shift & 0x1f)) as u16;
    e.mem.set_u16(this.addr() + 0x18, (flags & !mask) | value);
}

// Translated from 00439390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the alpha property's four-bit field at bits 5–8.
pub fn ni_alpha_property_set_field_5(e: &mut Engine, this: Ptr<NiAlphaProperty>, value: u16) {
    ni_alpha_property_update_flags(e, this, value, 0x01e0, 5);
}

// Translated from 004393c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the flag at mask 0x0200 through the executable's shared bit setter.
pub fn ni_alpha_property_set_flag_9(e: &mut Engine, this: Ptr<NiAlphaProperty>, value: u8) {
    e.call(NI_ALPHA_FLAGS_SET, &args![this, value, 0x0200u32]);
}

// Translated from 004393e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the alpha property's three-bit field at bits 10–12.
pub fn ni_alpha_property_set_field_10(e: &mut Engine, this: Ptr<NiAlphaProperty>, value: u16) {
    ni_alpha_property_update_flags(e, this, value, 0x1c00, 10);
}

// open: C++ exception unwinding for the temporary NiPointer is omitted.
// Translation incomplete: 00439410 - C++ exception unwinding and temporary cleanup on throw.
// Translated from 00439410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the property to the object's property list when its virtual type is
/// below 7. The temporary `NiPointer` and its normal destructor are retained;
/// the executable's SEH unwinding frame is omitted.
pub fn ni_av_object_attach_property(e: &mut Engine, this: Ptr<()>, property: Ptr<()>) {
    if e.vcall(property.addr(), 0x8c, &[]).i32() < 7 {
        e.with_stack(4, |e, local_pointer| {
            e.call(NI_POINTER_CONSTRUCT, &args![local_pointer, property]);
            ni_property_pointer_list_add_head(
                e,
                Ptr::new(this.addr() + 0x24),
                local_pointer.cast(),
            );
            e.call(NI_POINTER_DESTRUCT, &args![local_pointer]);
        });
    }
}

// open: C++ exception unwinding and allocation cleanup are omitted.
// Translation incomplete: 00439490 - C++ exception unwinding and cleanup on throw.
// Translated from 00439490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a multibound from a reference: rotation components at or above the
/// game's 0.01 threshold use an oriented box; otherwise it uses an axis-aligned box.
/// PC helper `00430830` returns the reference's angle vector at `+0x24`; actual
/// half-extents are fetched separately by `00569880`. The result owns the
/// selected shape through `BSMultiBound`.
pub fn multi_bound_marker_data_create_for_ref(e: &mut Engine, reference: Ptr<()>) -> Ptr<()> {
    if reference.is_null() {
        return Ptr::NULL;
    }

    let mut oriented = false;
    for offset in [0, 4, 8] {
        let rotation_angles: Ptr<()> = e.call(0x0043_0830, &args![reference]).ptr();
        let extent_value =
            f32::from_bits(x87_quiet_bits(e.mem.u32(rotation_angles.addr() + offset)));
        let extent = e.call(0x0040_8840, &args![extent_value]).f64();
        let threshold = e.global::<f64>(0x0101_6408);
        if extent.partial_cmp(&threshold) != Some(core::cmp::Ordering::Less) {
            oriented = true;
            break;
        }
    }

    let (shape_allocation, shape_ctor) = if oriented {
        (e.call(ALLOCATE, &args![0x48u32]).ptr::<()>(), 0x00c3_6310)
    } else {
        (e.call(ALLOCATE, &args![0x24u32]).ptr::<()>(), 0x00c3_7ec0)
    };
    let shape = if shape_allocation.is_null() {
        shape_allocation
    } else {
        e.call(shape_ctor, &args![shape_allocation]).ptr::<()>()
    };

    if oriented {
        e.with_stack(0x24, |e, orientation| {
            let returned = e
                .call(0x0056_fa00, &args![reference, orientation])
                .ptr::<()>();
            e.call(0x0043_96b0, &args![shape, returned]);
        });
    }
    let shape_half_extents: Ptr<()> = e.call(0x0056_9880, &args![reference]).ptr();
    e.call(0x0043_9680, &args![shape, shape_half_extents]);

    let center = e.vcall(reference.addr(), 0x1f4, &[]).ptr::<()>();
    e.vcall(shape.addr(), 0xb8, &args![center]);

    let bound_allocation = e.call(ALLOCATE, &args![0x10u32]).ptr::<()>();
    let bound = if bound_allocation.is_null() {
        bound_allocation
    } else {
        e.call(0x00c3_5e00, &args![bound_allocation]).ptr::<()>()
    };
    e.call(0x004a_ddc0, &args![bound, shape]);
    bound
}

// Translated from 00439940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks the twelve selected edges of a reference's oriented eight-corner
/// box against the cell rectangle. The executable returns the intersection
/// result in `AL` despite Ghidra's `void` declaration.
pub fn multi_bound_marker_data_intersects_cell(
    e: &mut Engine,
    reference: Ptr<()>,
    cell_x: i32,
    cell_y: i32,
) -> bool {
    if reference.is_null() || !multi_bound_marker_data_is_marker_form(e, reference) {
        return false;
    }

    let min_x = cell_x.wrapping_shl(12);
    let max_x = min_x.wrapping_add(0x1000);
    let min_y = cell_y.wrapping_shl(12);
    let max_y = min_y.wrapping_add(0x1000);
    e.with_stack(0x10, |e, bounds| {
        // Native locals are [min_x, max_y, max_x, min_y] at EBP-40.
        e.mem.set_u32(bounds.addr(), min_x as u32);
        e.mem.set_u32(bounds.addr() + 4, max_y as u32);
        e.mem.set_u32(bounds.addr() + 8, max_x as u32);
        e.mem.set_u32(bounds.addr() + 12, min_y as u32);

        e.with_stack(0x60, |e, corner_vectors| {
            // The executable calls this non-no-op iterator and its
            // `006815c0` element constructor for all eight corner vectors.
            e.call(
                0x0040_1050,
                &args![corner_vectors, 0x0cu32, 8u32, 0x0068_15c0u32],
            );
            e.with_stack(0x24, |e, point_temporaries| {
                // These constructors return their input in EAX (`mov eax,ecx`);
                // retain the three native calls because they are not bare RETs.
                for offset in [0, 0x0c, 0x18] {
                    e.call(0x0068_15c0, &args![point_temporaries.addr() + offset]);
                }
                let half_extents: Ptr<()> = e.call(0x0056_9880, &args![reference]).ptr();
                let scales = read_vector(e, half_extents);
                let center_ptr: Ptr<()> = e.vcall(reference.addr(), 0x1f4, &[]).ptr();
                let center = read_vector(e, center_ptr);
                let signs: [[f32; 3]; 8] = [
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0, -1.0],
                    [1.0, -1.0, 1.0],
                    [1.0, -1.0, -1.0],
                    [-1.0, 1.0, 1.0],
                    [-1.0, 1.0, -1.0],
                    [-1.0, -1.0, 1.0],
                    [-1.0, -1.0, -1.0],
                ];
                e.with_stack(0x0c, |e, center_buffer| {
                    write_vector(e, center_buffer, center);
                    e.with_stack(0x24, |e, orientation| {
                        e.call(0x0056_fa00, &args![reference, orientation]);
                        e.with_stack(0x18, |e, scratch| {
                            let axes = [
                                point_temporaries,
                                Ptr::new(point_temporaries.addr() + 0x0c),
                                Ptr::new(point_temporaries.addr() + 0x18),
                            ];
                            for i in 0..3 {
                                multibound_matrix_get_axis(e, orientation, i as u32, axes[i]);
                                multi_bound_marker_data_scale(e, axes[i].cast(), scales[i]);
                            }
                            let buffers = [scratch, Ptr::new(scratch.addr() + 0x0c)];
                            for (index, sign) in signs.iter().enumerate() {
                                let mut current = center_buffer;
                                for axis in 0..3 {
                                    let output = buffers[axis & 1];
                                    current = if sign[axis] > 0.0 {
                                        multibound_vector_add(e, current, output, axes[axis])
                                    } else {
                                        multibound_vector_subtract(e, current, output, axes[axis])
                                    };
                                }
                                write_vector(
                                    e,
                                    Ptr::new(corner_vectors.addr() + index as u32 * 0x0c),
                                    read_vector(e, current),
                                );
                            }
                        });
                        const FACE_VERTICES: [[usize; 4]; 4] =
                            [[0, 1, 2, 4], [3, 1, 2, 7], [5, 1, 4, 7], [6, 2, 4, 7]];
                        e.with_stack(0x0c, |e, output| {
                            e.call(0x0068_15c0, &args![output]);
                            for face in FACE_VERTICES {
                                let start = read_vector(
                                    e,
                                    Ptr::new(corner_vectors.addr() + face[0] as u32 * 0x0c),
                                );
                                let intersects = e.with_stack(0x18, |e, points| {
                                    let first = points;
                                    let second = Ptr::new(points.addr() + 0x0c);
                                    write_vector(e, first, start);
                                    for &vertex in face.iter().skip(1) {
                                        let end = read_vector(
                                            e,
                                            Ptr::new(corner_vectors.addr() + vertex as u32 * 0x0c),
                                        );
                                        write_vector(e, second, end);
                                        if e.call(
                                            VECTOR_MULTIBOUND_INTERSECT,
                                            &args![bounds, first, second, output],
                                        )
                                        .bool()
                                        {
                                            return true;
                                        }
                                    }
                                    false
                                });
                                if intersects {
                                    return true;
                                }
                            }
                            false
                        })
                    })
                })
            })
        })
    })
}

// Translated from 00439e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds two three-float vectors into `result` and returns it.
pub fn multibound_vector_add(
    e: &mut Engine,
    this: Ptr<()>,
    result: Ptr<()>,
    other: Ptr<()>,
) -> Ptr<()> {
    let left = read_vector(e, this);
    let right = read_vector(e, other);
    let values: [f32; 3] = core::array::from_fn(|i| (left[i] as f64 + right[i] as f64) as f32);
    e.call(VECTOR_SET, &args![result, values[0], values[1], values[2]]);
    result
}

// Translated from 00439ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts the second three-float vector from the first into `result`.
pub fn multibound_vector_subtract(
    e: &mut Engine,
    this: Ptr<()>,
    result: Ptr<()>,
    other: Ptr<()>,
) -> Ptr<()> {
    let left = read_vector(e, this);
    let right = read_vector(e, other);
    let values: [f32; 3] = core::array::from_fn(|i| (left[i] as f64 - right[i] as f64) as f32);
    e.call(VECTOR_SET, &args![result, values[0], values[1], values[2]]);
    result
}

// Translated from 00439f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the indexed three-float column from a matrix into `result`.
pub fn multibound_matrix_get_axis(e: &mut Engine, matrix: Ptr<()>, index: u32, result: Ptr<()>) {
    for (component, offset) in [0u32, 0x0c, 0x18].into_iter().enumerate() {
        let component_address = matrix
            .addr()
            .wrapping_add(offset)
            .wrapping_add(index.wrapping_mul(4));
        let value = f32::from_bits(x87_quiet_bits(e.mem.u32(component_address)));
        e.mem.set_f32(result.addr() + (component as u32) * 4, value);
    }
}

// Translated from 00439f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns true when the reference's form pointer equals either global
/// marker form pointer used by the executable.
pub fn multi_bound_marker_data_is_marker_form(e: &mut Engine, reference: Ptr<()>) -> bool {
    let form = e.call(MULTI_BOUND_GET_FORM, &args![reference]).u32();
    form == e.global::<u32>(MULTI_BOUND_FORM_A)
        || e.call(MULTI_BOUND_GET_FORM, &args![reference]).u32()
            == e.global::<u32>(MULTI_BOUND_FORM_B)
}

// Translated from 00439fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the supplied `NiPointer<NiProperty>` at the head of a NiTPointerList.
pub fn ni_property_pointer_list_add_head(
    e: &mut Engine,
    this: Ptr<NiTPointerList>,
    property: Ptr<()>,
) {
    let node: Ptr<()> = e.call(0x0049_fa80, &args![this]).ptr();
    e.call(0x006e_5cc0, &args![node.addr() + 8, property]);
    e.call(0x0049_fa20, &args![this, node]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00438f50,
            multi_bound_marker_data_construct(
                Ptr<MultiBoundMarkerData>,
            ) -> Ptr<MultiBoundMarkerData>
        ),
        entry!(
            0x00438f90,
            multi_bound_marker_data_save(Ptr<MultiBoundMarkerData>)
        ),
        entry!(
            0x00438fc0,
            multi_bound_marker_data_load(Ptr<MultiBoundMarkerData>, Ptr<()>) -> bool
        ),
        entry!(
            0x00439060,
            multi_bound_marker_data_compare(
                Ptr<MultiBoundMarkerData>,
                Ptr<MultiBoundMarkerData>,
            ) -> bool
        ),
        entry!(
            0x00439090,
            multi_bound_marker_data_differs(
                Ptr<MultiBoundMarkerData>,
                Ptr<MultiBoundMarkerData>,
            ) -> bool
        ),
        entry!(
            0x004390c0,
            multi_bound_marker_data_extents_equal(
                Ptr<MultiBoundMarkerData>,
                Ptr<MultiBoundMarkerData>,
            ) -> bool
        ),
        entry!(
            0x00439120,
            multi_bound_marker_data_copy(Ptr<MultiBoundMarkerData>, Ptr<MultiBoundMarkerData>)
        ),
        entry!(
            0x00439150,
            multi_bound_marker_data_is_default(Ptr<MultiBoundMarkerData>) -> bool
        ),
        entry!(
            0x00439180,
            multi_bound_marker_data_scale(
                Ptr<MultiBoundMarkerData>,
                f32,
            ) -> Ptr<MultiBoundMarkerData>
        ),
        entry!(
            0x004391c0,
            ni_alpha_property_construct(Ptr<NiAlphaProperty>) -> Ptr<NiAlphaProperty>
        ),
        entry!(
            0x00439260,
            ni_property_construct(Ptr<NiProperty>) -> Ptr<NiProperty>
        ),
        entry!(0x00439280, ni_property_get_rtti(Ptr<NiProperty>) -> Ptr<()>),
        entry!(
            0x00439290,
            ni_property_scalar_deleting_destructor(Ptr<NiProperty>, u32) -> Ptr<NiProperty>
        ),
        entry!(0x004392c0, ni_property_destruct(Ptr<NiProperty>)),
        entry!(
            0x004392e0,
            ni_alpha_property_get_rtti(Ptr<NiAlphaProperty>) -> Ptr<()>
        ),
        entry!(
            0x004392f0,
            ni_alpha_property_scalar_deleting_destructor(
                Ptr<NiAlphaProperty>,
                u32,
            ) -> Ptr<NiAlphaProperty>
        ),
        entry!(0x00439320, ni_alpha_property_destruct(Ptr<NiAlphaProperty>)),
        entry!(
            0x00439340,
            ni_alpha_property_set_field_1(Ptr<NiAlphaProperty>, u16)
        ),
        entry!(
            0x00439360,
            ni_alpha_property_update_flags(Ptr<NiAlphaProperty>, u16, u16, u32)
        ),
        entry!(
            0x00439390,
            ni_alpha_property_set_field_5(Ptr<NiAlphaProperty>, u16)
        ),
        entry!(
            0x004393c0,
            ni_alpha_property_set_flag_9(Ptr<NiAlphaProperty>, u8)
        ),
        entry!(
            0x004393e0,
            ni_alpha_property_set_field_10(Ptr<NiAlphaProperty>, u16)
        ),
        entry!(0x00439410, ni_av_object_attach_property(Ptr<()>, Ptr<()>)),
        entry!(
            0x00439490,
            multi_bound_marker_data_create_for_ref(Ptr<()>) -> Ptr<()>
        ),
        entry!(
            0x00439940,
            multi_bound_marker_data_intersects_cell(Ptr<()>, i32, i32) -> bool
        ),
        entry!(
            0x00439e90,
            multibound_vector_add(Ptr<()>, Ptr<()>, Ptr<()>) -> Ptr<()>
        ),
        entry!(
            0x00439ef0,
            multibound_vector_subtract(Ptr<()>, Ptr<()>, Ptr<()>) -> Ptr<()>
        ),
        entry!(
            0x00439f50,
            multibound_matrix_get_axis(Ptr<()>, u32, Ptr<()>)
        ),
        entry!(
            0x00439f90,
            multi_bound_marker_data_is_marker_form(Ptr<()>) -> bool
        ),
        entry!(
            0x00439fd0,
            ni_property_pointer_list_add_head(Ptr<NiTPointerList>, Ptr<()>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    fn engine() -> Engine {
        Engine::new()
    }

    fn allocate<T>(e: &mut Engine, size: u32) -> Ptr<T> {
        Ptr::new(e.mem.alloc(size))
    }

    fn map_data(e: &mut Engine, address: u32) {
        let page = address & !0xfff;
        if !e.mem.is_mapped(page) {
            e.map(page, 0x1000);
        }
    }

    fn install_vtable(e: &mut Engine, object: u32, slots: &[u32]) {
        let table = e.mem.alloc(slots.len() as u32 * 4);
        e.put_vtable(table, slots);
        e.mem.set_u32(object, table);
    }

    fn register_value(e: &mut Engine, address: u32, value: Ret) {
        e.register_double(address, move |_, _| value);
    }

    fn register_vector_setter(e: &mut Engine) {
        e.register_double(VECTOR_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[0] + i as u32 * 4, f32::from_bits(a[i + 1]));
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn constructor_copies_the_three_executable_defaults() {
        let mut e = engine();
        let this = allocate::<MultiBoundMarkerData>(&mut e, 12);
        map_data(&mut e, DEFAULT_HALF_EXTENTS);
        for (i, value) in [2.0f32, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_HALF_EXTENTS + i as u32 * 4, value);
        }
        e.register(0x0068_15c0, |_, _| Ret::default());
        assert_eq!(multi_bound_marker_data_construct(&mut e, this), this);
        assert_eq!(read_vector(&e, this.cast()), [2.0, 3.0, 4.0]);
    }

    #[test]
    fn save_writes_only_when_extents_differ_from_the_default() {
        let mut e = engine();
        let this = allocate::<MultiBoundMarkerData>(&mut e, 12);
        map_data(&mut e, DEFAULT_HALF_EXTENTS);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_HALF_EXTENTS + i as u32 * 4, value);
            e.mem.set_f32(this.addr() + i as u32 * 4, value);
        }
        e.register(0x0048_5710, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        multi_bound_marker_data_save(&mut e, this);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == MULTI_BOUND_SAVE_CHUNK));
        e.mem.set_f32(this.addr() + 8, 9.0);
        multi_bound_marker_data_save(&mut e, this);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == MULTI_BOUND_SAVE_CHUNK));
    }

    #[test]
    fn load_requires_the_expected_chunk_and_runs_its_clear_mode_reads() {
        let mut e = engine();
        let this = allocate::<MultiBoundMarkerData>(&mut e, 12);
        map_data(&mut e, DEFAULT_HALF_EXTENTS);
        let file = allocate::<()>(&mut e, 4);
        register_value(
            &mut e,
            MULTI_BOUND_GET_CHUNK,
            Ret {
                eax: 0x4f42_4d58,
                ..Ret::default()
            },
        );
        register_value(
            &mut e,
            MULTI_BOUND_READ_MODE,
            Ret {
                eax: 1,
                ..Ret::default()
            },
        );
        e.register(MULTI_BOUND_READ_CHUNK_DATA, |_, _| Ret::default());
        e.register(MULTI_BOUND_DATA_READ, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(multi_bound_marker_data_load(&mut e, this, file));
        let read_count = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MULTI_BOUND_DATA_READ)
            .count();
        assert_eq!(read_count, 3);
        assert!(!multi_bound_marker_data_load(&mut e, this, Ptr::NULL));
    }

    #[test]
    fn compare_and_differs_keep_null_and_nan_semantics() {
        let mut e = engine();
        let left = allocate::<MultiBoundMarkerData>(&mut e, 12);
        let right = allocate::<MultiBoundMarkerData>(&mut e, 12);
        write_vector(&mut e, left.cast(), [1.0, 2.0, 3.0]);
        write_vector(&mut e, right.cast(), [1.0, 2.0, 3.0]);
        assert!(multi_bound_marker_data_compare(&mut e, left, Ptr::NULL));
        assert!(!multi_bound_marker_data_compare(&mut e, left, right));
        e.mem.set_f32(right.addr() + 4, 7.0);
        assert!(multi_bound_marker_data_differs(&mut e, left, right));
        e.mem.set_f32(right.addr(), f32::NAN);
        assert!(!multi_bound_marker_data_extents_equal(&mut e, left, right));
    }

    #[test]
    fn copy_ignores_null_and_copies_all_components() {
        let mut e = engine();
        let left = allocate::<MultiBoundMarkerData>(&mut e, 12);
        let right = allocate::<MultiBoundMarkerData>(&mut e, 12);
        write_vector(&mut e, left.cast(), [8.0, 9.0, 10.0]);
        multi_bound_marker_data_copy(&mut e, left, Ptr::NULL);
        assert_eq!(read_vector(&e, left.cast()), [8.0, 9.0, 10.0]);
        write_vector(&mut e, right.cast(), [1.0, 2.0, 3.0]);
        multi_bound_marker_data_copy(&mut e, left, right);
        assert_eq!(read_vector(&e, left.cast()), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn default_test_inverts_the_different_result_from_00439090() {
        let mut e = engine();
        let this = allocate::<MultiBoundMarkerData>(&mut e, 12);
        map_data(&mut e, DEFAULT_HALF_EXTENTS);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_HALF_EXTENTS + i as u32 * 4, value);
            e.mem.set_f32(this.addr() + i as u32 * 4, value);
        }
        assert!(multi_bound_marker_data_is_default(&mut e, this));
        e.mem.set_f32(this.addr(), 5.0);
        assert!(!multi_bound_marker_data_is_default(&mut e, this));
    }

    #[test]
    fn scale_rounds_each_stored_component_and_returns_this() {
        let mut e = engine();
        let this = allocate::<MultiBoundMarkerData>(&mut e, 12);
        write_vector(&mut e, this.cast(), [1.0, -2.0, 4.0]);
        assert_eq!(multi_bound_marker_data_scale(&mut e, this, 0.5), this);
        assert_eq!(read_vector(&e, this.cast()), [0.5, -1.0, 2.0]);
    }

    #[test]
    fn alpha_property_constructor_sets_the_executable_defaults() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        e.register(NI_PROPERTY_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(0x0049_ed90, |_, _| Ret::default());
        e.register(0x0094_db80, |_, _| Ret::default());
        e.register(NI_ALPHA_FLAGS_SET, |_, _| Ret::default());
        assert_eq!(ni_alpha_property_construct(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), 0x0101_62dc);
        assert_eq!(e.mem.u16(this.addr() + 0x18) & 0x1e, 0x0c);
        assert_eq!(e.mem.u16(this.addr() + 0x18) & 0x1e0, 0xe0);
    }

    #[test]
    fn property_constructor_calls_its_base_and_installs_its_vtable() {
        let mut e = engine();
        let this = allocate::<NiProperty>(&mut e, 0x18);
        e.register(NI_PROPERTY_BASE_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(ni_property_construct(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6374);
    }

    #[test]
    fn rtti_accessors_return_their_static_objects() {
        let mut e = engine();
        assert_eq!(
            ni_property_get_rtti(&mut e, Ptr::new(0)).addr(),
            0x011f_4420
        );
        assert_eq!(
            ni_alpha_property_get_rtti(&mut e, Ptr::new(0)).addr(),
            0x011f_4408
        );
    }

    #[test]
    fn property_deleting_destructor_frees_only_with_flag_bit_zero_set() {
        let mut e = engine();
        let this = allocate::<NiProperty>(&mut e, 0x18);
        e.register(NI_PROPERTY_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            ni_property_scalar_deleting_destructor(&mut e, this, 0),
            this
        );
        assert_eq!(
            ni_property_scalar_deleting_destructor(&mut e, this, 1),
            this
        );
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == DEALLOCATE)
                .count(),
            1
        );
    }

    #[test]
    fn property_base_destructor_calls_the_engine_base() {
        let mut e = engine();
        let this = allocate::<NiProperty>(&mut e, 0x18);
        e.register(NI_PROPERTY_BASE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_property_destruct(&mut e, this);
        assert_eq!(
            e.call_log.unwrap(),
            vec![(NI_PROPERTY_BASE_DESTRUCT, vec![this.addr()])]
        );
    }

    #[test]
    fn alpha_property_deleting_destructor_preserves_the_flag_gate() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        e.register(NI_PROPERTY_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_alpha_property_scalar_deleting_destructor(&mut e, this, 2);
        ni_alpha_property_scalar_deleting_destructor(&mut e, this, 3);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == DEALLOCATE)
                .count(),
            1
        );
    }

    #[test]
    fn alpha_property_destructor_runs_the_base_chain() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        e.register(NI_PROPERTY_BASE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_alpha_property_destruct(&mut e, this);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == NI_PROPERTY_BASE_DESTRUCT));
    }

    #[test]
    fn alpha_property_bit_setters_preserve_unselected_bits() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        e.mem.set_u16(this.addr() + 0x18, 0xe000);
        ni_alpha_property_set_field_1(&mut e, this, 3);
        assert_eq!(e.mem.u16(this.addr() + 0x18), 0xe006);
        ni_alpha_property_set_field_5(&mut e, this, 4);
        assert_eq!(e.mem.u16(this.addr() + 0x18) & 0x01e0, 0x0080);
        ni_alpha_property_set_field_10(&mut e, this, 5);
        assert_eq!(e.mem.u16(this.addr() + 0x18) & 0x1c00, 0x1400);
    }

    #[test]
    fn alpha_property_shift_uses_a_promoted_u32_then_truncates() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        // Golden native vectors: FalloutNV.exe 1.4.0.525, 00439360;
        // phase0/codex-audit/multibound-leaf-native-results.jsonl.
        for (shift, expected) in [
            (0, 0xabcd),
            (1, 0x579a),
            (15, 0x8000),
            (16, 0),
            (31, 0),
            (32, 0xabcd),
            (48, 0),
        ] {
            e.mem.set_u8(this.addr() + 0x17, 0xaa);
            e.mem.set_u8(this.addr() + 0x1a, 0xaa);
            e.mem.set_u16(this.addr() + 0x18, 0x1234);
            ni_alpha_property_update_flags(&mut e, this, 0xabcd, 0xffff, shift);
            assert_eq!(e.mem.u16(this.addr() + 0x18), expected, "shift {shift}");
            assert_eq!(e.mem.u8(this.addr() + 0x17), 0xaa);
            assert_eq!(e.mem.u8(this.addr() + 0x1a), 0xaa);
        }
    }

    #[test]
    fn alpha_property_single_flag_uses_the_shared_bit_setter() {
        let mut e = engine();
        let this = allocate::<NiAlphaProperty>(&mut e, 0x1c);
        e.register(NI_ALPHA_FLAGS_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_alpha_property_set_flag_9(&mut e, this, 1);
        assert_eq!(
            e.call_log.unwrap(),
            vec![(NI_ALPHA_FLAGS_SET, vec![this.addr(), 1, 0x200])]
        );
    }

    #[test]
    fn attach_property_adds_only_virtual_types_below_seven() {
        let mut e = engine();
        let object = allocate::<()>(&mut e, 0x28);
        let property = allocate::<()>(&mut e, 4);
        let mut slots = vec![0; 0x8c / 4 + 1];
        slots[0x8c / 4] = 0x3333;
        install_vtable(&mut e, property.addr(), &slots);
        register_value(
            &mut e,
            0x3333,
            Ret {
                eax: 6,
                ..Ret::default()
            },
        );
        e.register(NI_POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        register_value(
            &mut e,
            0x0049_fa80,
            Ret {
                eax: 0x9000,
                ..Ret::default()
            },
        );
        e.register(0x006e_5cc0, |_, _| Ret::default());
        e.register(0x0049_fa20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_av_object_attach_property(&mut e, object, property);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0049_fa80));
        register_value(
            &mut e,
            0x3333,
            Ret {
                eax: 7,
                ..Ret::default()
            },
        );
        e.call_log = Some(vec![]);
        ni_av_object_attach_property(&mut e, object, property);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0049_fa80));
    }

    #[test]
    fn create_multibound_returns_null_for_a_null_reference() {
        let mut e = engine();
        assert!(multi_bound_marker_data_create_for_ref(&mut e, Ptr::NULL).is_null());
    }

    #[test]
    fn create_multibound_uses_rotation_threshold_and_returns_constructor_pointers() {
        let mut e = engine();
        let reference = allocate::<()>(&mut e, 4);
        let center = allocate::<()>(&mut e, 12);
        let rotation_angles = allocate::<()>(&mut e, 12);
        let extents = allocate::<()>(&mut e, 12);
        let returned_obb = allocate::<()>(&mut e, 0x48);
        let returned_aabb = allocate::<()>(&mut e, 0x24);
        let returned_bound = allocate::<()>(&mut e, 0x10);
        let abs_calls = Rc::new(Cell::new(0));
        let abs_calls_in_callback = Rc::clone(&abs_calls);
        write_vector(&mut e, center, [10.0, 20.0, 30.0]);
        write_vector(
            &mut e,
            rotation_angles,
            [f32::from_bits(0x7f80_0001), 0.0, 0.0],
        );
        write_vector(&mut e, extents, [0.5, 0.25, 0.0]);
        map_data(&mut e, 0x0101_6408);
        e.set_global(0x0101_6408, 0.01f64);
        let mut ref_slots = vec![0; 0x1f4 / 4 + 1];
        ref_slots[0x1f4 / 4] = 0x5555;
        install_vtable(&mut e, reference.addr(), &ref_slots);
        register_value(
            &mut e,
            0x0056_9880,
            Ret {
                eax: extents.addr(),
                ..Ret::default()
            },
        );
        register_value(
            &mut e,
            0x5555,
            Ret {
                eax: center.addr(),
                ..Ret::default()
            },
        );
        e.register_double(0x0043_0830, move |_, _| Ret {
            eax: rotation_angles.addr(),
            ..Ret::default()
        });
        e.register_double(0x0040_8840, move |e, a| {
            // A callback can change the threshold. The caller must reload it
            // after ABS, so this first extent now selects the oriented path.
            if abs_calls_in_callback.get() == 0 {
                assert_eq!(a[0], 0x7fc0_0001);
                e.set_global(0x0101_6408, 0.004f64);
            }
            abs_calls_in_callback.set(abs_calls_in_callback.get() + 1);
            Ret {
                st0: if a[0] == 0x7fc0_0001 {
                    0.005
                } else {
                    f32::from_bits(a[0]) as f64
                },
                ..Ret::default()
            }
        });
        e.register_double(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let mut shape_slots = vec![0; 0xb8 / 4 + 1];
        shape_slots[0xb8 / 4] = 0x6666;
        e.register_double(0x00c3_6310, move |e, _a| {
            install_vtable(e, returned_obb.addr(), &shape_slots);
            Ret {
                eax: returned_obb.addr(),
                ..Ret::default()
            }
        });
        e.register_double(0x00c3_5e00, move |_, _| Ret {
            eax: returned_bound.addr(),
            ..Ret::default()
        });
        let mut aabb_slots = vec![0; 0xb8 / 4 + 1];
        aabb_slots[0xb8 / 4] = 0x7777;
        e.register_double(0x00c3_7ec0, move |e, _a| {
            install_vtable(e, returned_aabb.addr(), &aabb_slots);
            Ret {
                eax: returned_aabb.addr(),
                ..Ret::default()
            }
        });
        e.register(0x0043_96b0, |_, _| Ret::default());
        e.register(0x0043_9680, |_, _| Ret::default());
        e.register_double(0x004a_ddc0, move |_, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], returned_bound.addr());
            assert_eq!(a[1], returned_obb.addr());
            Ret::default()
        });
        e.register(0x0056_fa00, |e, a| {
            for i in 0..9 {
                e.mem
                    .set_f32(a[1] + i * 4, if i % 4 == 0 { 1.0 } else { 0.0 });
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(0x6666, |_, _| Ret::default());
        e.register(0x7777, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let bound = multi_bound_marker_data_create_for_ref(&mut e, reference);
        assert_eq!(bound, returned_bound);
        let first_log = e.call_log.as_ref().unwrap();
        let allocations: Vec<u32> = first_log
            .iter()
            .filter(|(address, _)| *address == ALLOCATE)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(allocations, [0x48, 0x10]);
        assert_eq!(
            first_log
                .iter()
                .filter(|(address, _)| *address == 0x0043_0830)
                .count(),
            1
        );

        write_vector(&mut e, rotation_angles, [0.0, 0.0, 0.0]);
        e.set_global(0x0101_6408, 0.01f64);
        e.register_double(0x004a_ddc0, move |_, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], returned_bound.addr());
            assert_eq!(a[1], returned_aabb.addr());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let axis_aligned = multi_bound_marker_data_create_for_ref(&mut e, reference);
        assert_eq!(axis_aligned, returned_bound);
        let second_log = e.call_log.as_ref().unwrap();
        let allocations: Vec<u32> = second_log
            .iter()
            .filter(|(address, _)| *address == ALLOCATE)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(allocations, [0x24, 0x10]);
        assert_eq!(
            second_log
                .iter()
                .filter(|(address, _)| *address == 0x0043_0830)
                .count(),
            3
        );
        assert_eq!(abs_calls.get(), 4);
    }

    #[test]
    fn intersects_cell_checks_the_normal_path_and_returns_the_first_hit() {
        let mut e = engine();
        let reference = allocate::<()>(&mut e, 4);
        map_data(&mut e, MULTI_BOUND_FORM_A);
        let center = allocate::<()>(&mut e, 12);
        let extents = allocate::<()>(&mut e, 12);
        map_data(&mut e, MULTI_BOUND_FORM_A);
        write_vector(&mut e, center, [2.0, 3.0, 4.0]);
        write_vector(&mut e, extents, [1.0, 1.0, 1.0]);
        let mut ref_slots = vec![0; 0x1f4 / 4 + 1];
        ref_slots[0x1f4 / 4] = 0x4444;
        install_vtable(&mut e, reference.addr(), &ref_slots);
        register_value(
            &mut e,
            MULTI_BOUND_GET_FORM,
            Ret {
                eax: 0x1234,
                ..Ret::default()
            },
        );
        e.mem.set_u32(MULTI_BOUND_FORM_A, 0x1234);
        register_value(
            &mut e,
            0x0056_9880,
            Ret {
                eax: extents.addr(),
                ..Ret::default()
            },
        );
        e.register_double(0x4444, move |e, _| {
            // The extents must already have been copied before this getter.
            write_vector(e, extents, [50.0, 50.0, 50.0]);
            Ret {
                eax: center.addr(),
                ..Ret::default()
            }
        });
        e.register_double(0x0056_fa00, move |e, a| {
            // The center must already have been copied before this callback.
            write_vector(e, center, [10.0, 20.0, 30.0]);
            for i in 0..9 {
                e.mem
                    .set_f32(a[1] + i * 4, if i % 4 == 0 { 1.0 } else { 0.0 });
            }
            Ret::default()
        });
        e.register_double(0x0040_1050, |e, a| {
            for index in 0..a[2] {
                e.call(a[3], &args![a[0].wrapping_add(a[1].wrapping_mul(index))]);
            }
            Ret::default()
        });
        e.register_double(0x0068_15c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        register_vector_setter(&mut e);
        e.register_double(VECTOR_MULTIBOUND_INTERSECT, |e, a| {
            let bounds: Ptr<()> = Ptr::new(a[0]);
            assert_eq!(e.mem.u32(bounds.addr()), 8192);
            assert_eq!(e.mem.u32(bounds.addr() + 4), 24576);
            assert_eq!(e.mem.u32(bounds.addr() + 8), 12288);
            assert_eq!(e.mem.u32(bounds.addr() + 12), 20480);
            let output_constructor = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .rev()
                .find(|(address, _)| *address == 0x0068_15c0)
                .unwrap()
                .1[0];
            assert_eq!(a[3], output_constructor);
            assert_eq!(read_vector(e, Ptr::new(a[1])), [3.0, 4.0, 5.0]);
            assert_eq!(read_vector(e, Ptr::new(a[2])), [3.0, 4.0, 3.0]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert!(multi_bound_marker_data_intersects_cell(
            &mut e, reference, 2, 5
        ));
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == VECTOR_MULTIBOUND_INTERSECT)
                .count(),
            1
        );
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(calls.iter().filter(|(a, _)| *a == 0x0040_1050).count(), 1);
        assert_eq!(calls.iter().filter(|(a, _)| *a == 0x0068_15c0).count(), 12);

        write_vector(&mut e, center, [2.0, 3.0, 4.0]);
        write_vector(&mut e, extents, [1.0, 1.0, 1.0]);
        let segment_calls = Rc::new(Cell::new(0));
        let segment_calls_in_callback = Rc::clone(&segment_calls);
        e.register_double(VECTOR_MULTIBOUND_INTERSECT, move |e, a| {
            let output_constructor = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .rev()
                .find(|(address, _)| *address == 0x0068_15c0)
                .unwrap()
                .1[0];
            assert_eq!(a[3], output_constructor);
            segment_calls_in_callback.set(segment_calls_in_callback.get() + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!multi_bound_marker_data_intersects_cell(
            &mut e, reference, 2, 5
        ));
        assert_eq!(segment_calls.get(), 12);
    }

    #[test]
    fn vector_add_stores_the_rounded_sum_and_returns_destination() {
        let mut e = engine();
        let left = allocate::<()>(&mut e, 12);
        let right = allocate::<()>(&mut e, 12);
        let result = allocate::<()>(&mut e, 12);
        write_vector(&mut e, left, [1.0, 2.0, 3.0]);
        write_vector(&mut e, right, [4.0, 5.0, 6.0]);
        register_vector_setter(&mut e);
        assert_eq!(multibound_vector_add(&mut e, left, result, right), result);
        assert_eq!(read_vector(&e, result), [5.0, 7.0, 9.0]);
    }

    #[test]
    fn vector_subtract_stores_the_rounded_difference_and_returns_destination() {
        let mut e = engine();
        let left = allocate::<()>(&mut e, 12);
        let right = allocate::<()>(&mut e, 12);
        let result = allocate::<()>(&mut e, 12);
        write_vector(&mut e, left, [5.0, 7.0, 9.0]);
        write_vector(&mut e, right, [4.0, 5.0, 6.0]);
        register_vector_setter(&mut e);
        assert_eq!(
            multibound_vector_subtract(&mut e, left, result, right),
            result
        );
        assert_eq!(read_vector(&e, result), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn matrix_axis_copy_uses_the_three_strided_components() {
        let mut e = engine();
        let matrix = allocate::<()>(&mut e, 0x24);
        let result = allocate::<()>(&mut e, 0x10);
        // Golden native vector: FalloutNV.exe 1.4.0.525, 00439f50;
        // phase0/codex-audit/multibound-leaf-native-results.jsonl. x87
        // FLD/FSTP quiets this sNaN while the adjacent guard stays untouched.
        e.mem.set_u32(result.addr() + 0x0c, 0xaaaa_aaaa);
        for i in 0..9 {
            e.mem.set_f32(matrix.addr() + i * 4, i as f32);
        }
        e.mem.set_u32(matrix.addr() + 4, 0x7f80_0001);
        multibound_matrix_get_axis(&mut e, matrix, 1, result);
        assert_eq!(e.mem.u32(result.addr()), 0x7fc0_0001);
        assert_eq!(e.mem.f32(result.addr() + 4), 4.0);
        assert_eq!(e.mem.f32(result.addr() + 8), 7.0);
        assert_eq!(e.mem.u32(result.addr() + 0x0c), 0xaaaa_aaaa);
    }

    #[test]
    fn x87_float_round_trip_quiets_generated_signaling_nan() {
        let signaling_nan = f32::from_bits(0xff80_0123);
        assert_eq!(x87_quiet_bits(signaling_nan.to_bits()), 0xffc0_0123);
        assert_eq!(x87_quiet_bits((-3.5f32).to_bits()), (-3.5f32).to_bits());
    }

    #[test]
    fn marker_form_test_reads_again_only_after_the_first_mismatch() {
        let mut e = engine();
        let reference = allocate::<()>(&mut e, 4);
        map_data(&mut e, MULTI_BOUND_FORM_A);
        e.mem.set_u32(MULTI_BOUND_FORM_A, 10);
        e.mem.set_u32(MULTI_BOUND_FORM_B, 20);
        e.call_log = Some(vec![]);
        register_value(
            &mut e,
            MULTI_BOUND_GET_FORM,
            Ret {
                eax: 10,
                ..Ret::default()
            },
        );
        assert!(multi_bound_marker_data_is_marker_form(&mut e, reference));
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == MULTI_BOUND_GET_FORM)
                .count(),
            1
        );
    }

    #[test]
    fn property_pointer_list_add_head_calls_its_three_steps_in_order() {
        let mut e = engine();
        let list = allocate::<NiTPointerList>(&mut e, 12);
        let node = allocate::<()>(&mut e, 12);
        let property_pointer = allocate::<()>(&mut e, 4);
        register_value(
            &mut e,
            0x0049_fa80,
            Ret {
                eax: node.addr(),
                ..Ret::default()
            },
        );
        e.register(0x006e_5cc0, |_, _| Ret::default());
        e.register(0x0049_fa20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        ni_property_pointer_list_add_head(&mut e, list, property_pointer);
        let addrs: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(addrs, [0x0049_fa80, 0x006e_5cc0, 0x0049_fa20]);
    }
}
