//! `NiMain/run_00a89650` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `32767.0` (`double`, `01029790`): the scale of a compacted 16-bit value.
const SHORT_SCALE: u32 = 0x0102_9790;

// Translated from 00a89650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Expands `count` compacted 16-bit values (the engine map has no name;
/// `cdecl`, its one caller `NiBSplineData::GetCompactedValueDegree3`):
/// each signed value `v` becomes the `float` `v / 32767.0 * range +
/// offset` (x87 `FDIV`, `FMUL`, `FADD` in that order). The second argument
/// is never read. The exe unrolls the loop by four while at least four
/// values remain (signed count); the order of the values and the
/// arithmetic are the same.
pub fn fn_00a89650(
    e: &mut Engine,
    source: Ptr,
    _unused_2: u32,
    offset: f32,
    range: f32,
    dest: Ptr,
    count: u32,
) {
    let range = x87_load32(range.to_bits());
    let offset = x87_load32(offset.to_bits());
    let scale = x87_load64(e.global::<u64>(SHORT_SCALE));
    let mut index = 0u32;
    while index < count {
        let value = e.mem.i16(source.addr().wrapping_add(2 * index));
        let quotient = x87_div(f64::from(value), scale);
        let expanded = x87_add(x87_mul(quotient, range), offset);
        e.mem
            .set_u32(dest.addr().wrapping_add(4 * index), x87_store32(expanded));
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8_9650,
        fn_00a89650(Ptr, u32, f32, f32, Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compacted_values_expand_to_offset_plus_scaled_range() {
        let mut e = Engine::new();
        e.map(SHORT_SCALE, 8);
        e.set_global(SHORT_SCALE, 32767.0f64);
        // Six values: one unrolled group of four, then two more.
        let values: [i16; 6] = [32767, -32767, 0, 16384, -1, 1];
        let source = e.mem.alloc(12);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_i16(source + 2 * i as u32, *value);
        }
        let dest = e.mem.alloc(28);
        e.mem.set_f32(dest + 24, 99.0);
        e.call(
            0x00a8_9650,
            &args![source, 0u32, 1.0f32, 2.0f32, dest, 6u32],
        );
        for (i, value) in values.iter().enumerate() {
            let expected = (f64::from(*value) / 32767.0 * 2.0 + 1.0) as f32;
            assert_eq!(e.mem.f32(dest + 4 * i as u32), expected);
        }
        assert_eq!(e.mem.f32(dest + 24), 99.0);
        // A count of zero writes nothing.
        e.call(
            0x00a8_9650,
            &args![source, 0u32, 5.0f32, 2.0f32, dest, 0u32],
        );
        assert_eq!(e.mem.f32(dest), 3.0);
    }
}
