//! `Havok SDK/hkpcompressedsampledheightfieldshapeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpCompressedSampledHeightFieldShape` (Xbox PDB) keeps its samples as 16-bit heights:
//! `m_xRes` at `+0x10`, the triangle flip byte at `+0x6C`, the height array (data `+0x60`,
//! size `+0x64`, capacity and flags `+0x68`) and the height scale `+0x74` and offset `+0x70`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The base class destructor body (`this` in `ECX`, no arguments).
const BASE_DESTRUCTOR: u32 = 0x00d2_4660;

// Translated from 00e01140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCompressedSampledHeightFieldShape::getTriangleFlipImpl` (Xbox PDB): stores the flip
/// byte at `this + 0x6C` to `*out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_compressed_sampled_height_field_shape_get_triangle_flip_impl(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) -> Ptr {
    let flip = e.mem.u8(this.addr() + 0x6c);
    e.mem.set_u8(out.addr(), flip);
    out
}

// Translated from 00e01150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCompressedSampledHeightFieldShape::getHeightAtImpl` (Xbox PDB): the height of sample
/// `(x, z)`, returned in `ST0`: the `u16` at index `m_xRes * z + x` (signed 32-bit
/// arithmetic) of the height array, times the scale `+0x74` plus the offset `+0x70` (x87
/// arithmetic, stored as a single precision float).
pub fn hkp_compressed_sampled_height_field_shape_get_height_at_impl(
    e: &mut Engine,
    this: Ptr,
    x: i32,
    z: i32,
) -> f32 {
    let base = this.addr();
    let width = e.mem.u32(base + 0x10) as i32;
    let index = width.wrapping_mul(z).wrapping_add(x) as u32;
    let data = e.mem.u32(base + 0x60);
    let sample = e.mem.u16(data.wrapping_add(index.wrapping_mul(2)));
    let scale = e.mem.f32(base + 0x74);
    let offset = e.mem.f32(base + 0x70);
    ((f64::from(sample) * f64::from(scale)) + f64::from(offset)) as f32
}

// Translated from 00e01180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCompressedSampledHeightFieldShape` scalar deleting destructor (Xbox PDB name
/// `_scalar_deleting_destructor_`): clears the height array's size `+0x64`; when its
/// capacity word `+0x68` has no "not owned" top bit, frees the data (`+0x60`) through the
/// memory router's allocator (slot `0x10`, with the byte count `(capacity & 0x3fffffff) *
/// 2`); resets the array (data null, capacity word `0x80000000`); runs the base destructor
/// body (`00d24660`); and when bit 0 of `flags` is set frees the object (allocator slot 8,
/// with the 16-bit size at `+4`). Returns `this`.
pub fn hkp_compressed_sampled_height_field_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x68);
    e.mem.set_u32(base + 0x64, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x60);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x68, 0x8000_0000);
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e0_1140,
            hkp_compressed_sampled_height_field_shape_get_triangle_flip_impl(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00e0_1150,
            hkp_compressed_sampled_height_field_shape_get_height_at_impl(Ptr, i32, i32) -> f32
        ),
        entry!(
            0x00e0_1180,
            hkp_compressed_sampled_height_field_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    #[test]
    fn triangle_flip_copies_the_byte_00e01140() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let out = e.mem.alloc(4);
        for flip in [0u8, 1, 0xfe] {
            e.mem.set_u8(this + 0x6c, flip);
            e.call(0x00e0_1140, &[this, out]);
            assert_eq!(e.mem.u8(out), flip);
        }
    }

    #[test]
    fn triangle_flip_answers_its_out_pointer_00e01140() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let out = e.mem.alloc(4);
        // EAX is `out` at the RET (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00e0_1140, &[this, out]).u32(), out);
    }

    /// A shape with a 4 wide height array holding `100 * index`, scale 0.5 and offset 10.
    fn height_field(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x40);
        for index in 0..16u32 {
            e.mem.set_u16(data + 2 * index, (100 * index) as u16);
        }
        e.mem.set_u32(this + 0x10, 4);
        e.mem.set_u32(this + 0x60, data);
        e.mem.set_f32(this + 0x70, 10.0);
        e.mem.set_f32(this + 0x74, 0.5);
        this
    }

    #[test]
    fn height_at_indexes_row_major_and_scales_00e01150() {
        let mut e = Engine::new();
        let this = height_field(&mut e);
        // Index 4 * 2 + 1 = 9.
        assert_eq!(e.call(0x00e0_1150, &args![this, 1i32, 2i32]).f32(), 460.0);
        assert_eq!(e.call(0x00e0_1150, &args![this, 0i32, 0i32]).f32(), 10.0);
        assert_eq!(e.call(0x00e0_1150, &args![this, 3i32, 3i32]).f32(), 760.0);
    }

    #[test]
    fn height_at_rounds_once_to_single_precision_00e01150() {
        let mut e = Engine::new();
        let this = height_field(&mut e);
        // 100 * 1 = sample 100; 100 * 0.1 + 0.01 is not a float: the x87 sum is rounded
        // once, at the store.
        e.mem.set_f32(this + 0x74, 0.1);
        e.mem.set_f32(this + 0x70, 0.01);
        let expected = ((100.0f64 * f64::from(0.1f32)) + f64::from(0.01f32)) as f32;
        assert_eq!(
            e.call(0x00e0_1150, &args![this, 1i32, 0i32])
                .f32()
                .to_bits(),
            expected.to_bits()
        );
    }

    /// A router whose allocator frees through logging doubles at slots 8 and 0x10, a double
    /// of the base destructor body, and a shape with a height array of capacity word
    /// `capacity`. Returns `(this, data, log)`.
    fn destructible(e: &mut Engine, capacity: u32) -> (u32, u32, Log) {
        let log = new_log();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        log_double(e, &log, 0x0900_0008, 0);
        log_double(e, &log, 0x0900_0010, 0);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, &log, MEMORY_ROUTER, router);
        log_double(e, &log, BASE_DESTRUCTOR, 0);
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x70);
        e.mem.set_u32(this + 0x60, data);
        e.mem.set_u32(this + 0x64, 9);
        e.mem.set_u32(this + 0x68, capacity);
        (this, data, log)
    }

    #[test]
    fn destructor_frees_owned_data_and_the_object_00e01180() {
        let mut e = engine();
        let (this, data, log) = destructible(&mut e, 0x0000_0010);
        let result = e.call(0x00e0_1180, &[this, 1]);
        assert_eq!(result.u32(), this);
        let freed = calls(&log, 0x0900_0010);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [data, 0x20]);
        assert_eq!(calls(&log, BASE_DESTRUCTOR), vec![vec![this]]);
        let released = calls(&log, 0x0900_0008);
        assert_eq!(released.len(), 1);
        assert_eq!(released[0][1..], [this, 0x70]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0x8000_0000);
    }

    #[test]
    fn destructor_keeps_data_it_does_not_own_and_the_object_without_bit_zero_00e01180() {
        let mut e = engine();
        let (this, _, log) = destructible(&mut e, 0x8000_0010);
        let result = e.call(0x00e0_1180, &[this, 0xfe]);
        assert_eq!(result.u32(), this);
        assert!(calls(&log, 0x0900_0010).is_empty());
        assert!(calls(&log, 0x0900_0008).is_empty());
        assert_eq!(calls(&log, BASE_DESTRUCTOR).len(), 1);
        assert_eq!(e.mem.u32(this + 0x68), 0x8000_0000);
    }
}
