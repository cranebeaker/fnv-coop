//! `fallout shared/run_0057e9c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Where one field of the shader data's defaults comes from.
#[derive(Clone, Copy)]
enum Source {
    /// `FLD dword [addr]`: a `float` constant of the exe, copied.
    Constant(u32),
    /// `FLD1`.
    One,
    /// `FLDZ`.
    Zero,
    /// `MOV dword`: a colour word.
    Word(u32),
}

/// The stores of `0057e9c0`, in the exe's order: (offset, value).
const DEFAULTS: &[(u32, Source)] = &[
    (0x00, Source::Constant(0x0101_e2bc)),
    (0x04, Source::Constant(0x0102_49d8)),
    (0x08, Source::Constant(0x0101_6248)),
    (0x0c, Source::One),
    (0x10, Source::Constant(0x0101_b268)),
    (0x14, Source::Constant(0x0101_6248)),
    (0x18, Source::Constant(0x0103_13d4)),
    (0x1c, Source::Zero),
    (0x20, Source::Zero),
    (0x24, Source::Zero),
    (0x84, Source::One),
    (0x90, Source::Zero),
    (0x94, Source::Constant(0x0101_3974)),
    (0x8c, Source::One),
    (0x28, Source::Word(0x0080_8000)),
    (0x2c, Source::Word(0x0019_0000)),
    (0x30, Source::Word(0x00ff_ffff)),
    (0x38, Source::Constant(0x0101_e2bc)),
    (0x3c, Source::Constant(0x0101_8180)),
    (0x40, Source::Constant(0x0102_31a4)),
    (0x44, Source::Constant(0x0101_62c0)),
    (0x48, Source::Constant(0x0101_3ea4)),
    (0x4c, Source::Constant(0x0102_31a8)),
    (0x50, Source::Constant(0x0101_8180)),
    (0x54, Source::Constant(0x0102_31a4)),
    (0x58, Source::Constant(0x0101_7b78)),
    (0x5c, Source::Constant(0x0101_81bc)),
    (0x7c, Source::Zero),
    (0x80, Source::Zero),
    (0x60, Source::One),
    (0x88, Source::Constant(0x0101_3d84)),
    (0xac, Source::Constant(0x0101_6410)),
    (0xb0, Source::Constant(0x0101_6410)),
    (0xb4, Source::Constant(0x0101_6410)),
    (0x98, Source::Constant(0x0101_b2d0)),
    (0x9c, Source::Constant(0x0101_6410)),
    (0xa0, Source::One),
    (0xa4, Source::Constant(0x0102_2958)),
    (0xa8, Source::One),
];

// Translated from 0057e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The defaults of a `WaterShaderData` (the 0xC4-byte block at
/// `TESWaterForm` +0x84): wind, waves, sun, fog and noise floats copied
/// from the exe's `float` constants or 1.0 / 0.0, and the shallow, deep and
/// reflection colours (0x808000, 0x190000, 0xFFFFFF). The fields at +0x34
/// (`cBlend`, Xbox PDB), +0x64..+0x78 and +0xB8..+0xC0 are not written.
/// Each constant is an `FLD dword` / `FSTP dword` copy.
pub fn fn_0057e9c0(e: &mut Engine, this: Ptr) {
    for &(offset, source) in DEFAULTS {
        let bits = match source {
            Source::Constant(addr) => x87_quiet_bits(e.global::<u32>(addr)),
            Source::One => 1.0f32.to_bits(),
            Source::Zero => 0.0f32.to_bits(),
            Source::Word(word) => word,
        };
        e.mem.set_u32(this.addr() + offset, bits);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0057_e9c0, fn_0057e9c0(Ptr))]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_0057e9c0_writes_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0101_3000, 0x0002_1000);
        // Every constant holds its own address as bits, except one
        // signalling NaN, which the FLD/FSTP copy quiets.
        for &(_, source) in DEFAULTS {
            if let Source::Constant(addr) = source {
                e.set_global::<u32>(addr, addr);
            }
        }
        e.set_global::<u32>(0x0102_49d8, 0x7f80_0001);
        let this = e.mem.alloc(0xc4);
        for i in 0..0xc4 / 4 {
            e.mem.set_u32(this + 4 * i, 0xdead_beef);
        }
        e.call(0x0057_e9c0, &args![this]);
        let at = |o: u32| e.mem.u32(this + o);
        assert_eq!(at(0x00), 0x0101_e2bc);
        assert_eq!(at(0x04), 0x7fc0_0001);
        assert_eq!(at(0x0c), 0x3f80_0000);
        assert_eq!(at(0x18), 0x0103_13d4);
        assert_eq!(at(0x24), 0);
        assert_eq!(at(0x28), 0x0080_8000);
        assert_eq!(at(0x2c), 0x0019_0000);
        assert_eq!(at(0x30), 0x00ff_ffff);
        assert_eq!(at(0x34), 0xdead_beef);
        assert_eq!(at(0x5c), 0x0101_81bc);
        assert_eq!(at(0x60), 0x3f80_0000);
        assert_eq!(at(0x64), 0xdead_beef);
        assert_eq!(at(0x78), 0xdead_beef);
        assert_eq!(at(0x7c), 0);
        assert_eq!(at(0x88), 0x0101_3d84);
        assert_eq!(at(0x98), 0x0101_b2d0);
        assert_eq!(at(0xa4), 0x0102_2958);
        assert_eq!(at(0xa8), 0x3f80_0000);
        assert_eq!(at(0xb4), 0x0101_6410);
        assert_eq!(at(0xb8), 0xdead_beef);
        assert_eq!(at(0xc0), 0xdead_beef);
    }
}
