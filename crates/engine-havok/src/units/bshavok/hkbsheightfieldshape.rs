//! `BSHavok/hkbsheightfieldshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cafec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBSHeightFieldShape::getHeightAtImpl` (Xbox PDB): the height at grid cell `(x, z)`.
/// The cell index is `(rows - z) * stride + x` (32-bit wrapping, `rows` at `+0x64`,
/// `stride` at `+0x10`); the sample is the float at `heights + 12 * index + 8`
/// (`heights` at `+0x60`) plus the float at `+0x68`, added in the x87 (53-bit mantissa) and stored back through
/// a float, so a signalling NaN comes out quiet.
pub fn hk_b_s_height_field_shape_get_height_at_impl(
    e: &mut Engine,
    this: Ptr,
    x: i32,
    z: i32,
) -> f32 {
    let base = this.addr();
    let rows = e.mem.i32(base + 0x64);
    let stride = e.mem.i32(base + 0x10);
    let heights = e.mem.u32(base + 0x60);
    let index = rows.wrapping_sub(z).wrapping_mul(stride).wrapping_add(x);
    let offset = index.wrapping_add(index.wrapping_mul(2)) as u32;
    let at = heights.wrapping_add(offset.wrapping_mul(4)).wrapping_add(8);
    let sample = x87_quiet(e.mem.f32(at));
    let bias = x87_quiet(e.mem.f32(base + 0x68));
    x87_quiet((f64::from(sample) + f64::from(bias)) as f32)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ca_fec0,
        hk_b_s_height_field_shape_get_height_at_impl(Ptr, i32, i32) -> f32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn height_is_the_sample_plus_the_bias_at_the_computed_cell() {
        let mut e = Engine::new();
        let shape = e.mem.alloc(0x70);
        let samples = e.mem.alloc(12 * 20);
        e.mem.set_u32(shape + 0x60, samples);
        e.mem.set_i32(shape + 0x64, 4);
        e.mem.set_i32(shape + 0x10, 3);
        e.mem.set_f32(shape + 0x68, 0.5);
        // x = 2, z = 1: index (4 - 1) * 3 + 2 = 11.
        e.mem.set_f32(samples + 12 * 11 + 8, 10.25);
        let height = e.call(0x00ca_fec0, &args![shape, 2i32, 1i32]).f32();
        assert_eq!(height, 10.75);
        // Negative coordinates index backwards from the first sample.
        e.mem.set_f32(samples + 12 * 3 + 8, -2.0);
        e.mem.set_i32(shape + 0x64, 0);
        let height = e.call(0x00ca_fec0, &args![shape, 0i32, -1i32]).f32();
        assert_eq!(height, -1.5);
    }

    #[test]
    fn a_signalling_nan_sample_comes_out_quiet() {
        let mut e = Engine::new();
        let shape = e.mem.alloc(0x70);
        let samples = e.mem.alloc(24);
        e.mem.set_u32(shape + 0x60, samples);
        e.mem.set_u32(samples + 8, 0x7f80_0001);
        let height = e.call(0x00ca_fec0, &args![shape, 0i32, 0i32]).f32();
        assert!(height.is_nan());
        assert_eq!(height.to_bits(), 0x7fc0_0001);
    }
}
