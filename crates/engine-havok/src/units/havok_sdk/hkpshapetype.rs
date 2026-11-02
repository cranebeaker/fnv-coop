//! `Havok SDK/hkpshapetype.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d464f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGetShapeTypeName` (Xbox PDB): the address of the name string of a shape type (cdecl, one
/// argument). The compiler's switch on the type plus one (an unsigned number): `-1` is
/// `"HK_SHAPE_ALL"`, `0` to `32` are the named types (`"HK_SHAPE_INVALID"`,
/// `"HK_SHAPE_SPHERE"`, ...; each case loads the address of its string), and anything above
/// answers zero.
pub fn hk_get_shape_type_name(_e: &mut Engine, shape_type: u32) -> u32 {
    match shape_type {
        0xffff_ffff => 0x010c_f70c,
        0 => 0x010c_f71c,
        1 => 0x010c_f6a8,
        2 => 0x010c_f65c,
        3 => 0x010c_f694,
        4 => 0x010c_f684,
        5 => 0x010c_f670,
        6 => 0x010c_f640,
        7 => 0x010c_f6cc,
        8 => 0x010c_f6b8,
        9 => 0x010c_f600,
        10 => 0x010c_f53c,
        11 => 0x010c_f50c,
        12 => 0x010c_f4f0,
        13 => 0x010c_f57c,
        14 => 0x010c_f4d8,
        15 => 0x010c_f528,
        16 => 0x010c_f6e0,
        17 => 0x010c_f4ac,
        18 => 0x010c_f6fc,
        19 => 0x010c_f4c0,
        20 => 0x010c_f628,
        21 => 0x010c_f610,
        22 => 0x010c_f5e8,
        23 => 0x010c_f5c8,
        24 => 0x010c_f5b4,
        25 => 0x010c_f59c,
        26 => 0x010c_f568,
        27 => 0x010c_f55c,
        28 => 0x010c_f54c,
        29 => 0x010c_f490,
        30 => 0x010c_f480,
        31 => 0x010c_f470,
        32 => 0x010c_f460,
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_64f0, hk_get_shape_type_name(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_looked_up_by_type_plus_one() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d4_64f0, &args![0xffff_ffffu32]).u32(),
            0x010c_f70c
        );
        assert_eq!(e.call(0x00d4_64f0, &args![0u32]).u32(), 0x010c_f71c);
        assert_eq!(e.call(0x00d4_64f0, &args![1u32]).u32(), 0x010c_f6a8);
        assert_eq!(e.call(0x00d4_64f0, &args![18u32]).u32(), 0x010c_f6fc);
        assert_eq!(e.call(0x00d4_64f0, &args![32u32]).u32(), 0x010c_f460);
    }

    #[test]
    fn unknown_types_have_no_name() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_64f0, &args![33u32]).u32(), 0);
        assert_eq!(e.call(0x00d4_64f0, &args![0xffff_fffeu32]).u32(), 0);
        assert_eq!(e.call(0x00d4_64f0, &args![0x8000_0000u32]).u32(), 0);
    }
}
