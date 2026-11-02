//! `Havok SDK/run_00de4de0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four accessors of the vectors and the angle block of `hkDisplaySemiCircle` (the offsets are
//! the ones `hkdisplaysemicircle.rs` and `run_00de48a0.rs` use): each answers the address of a
//! member in `EAX` (`LEA EAX, [ECX + offset]`).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00de4de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the vector at `this + 0x60` (the axis).
pub fn fn_00de4de0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x60))
}

// Translated from 00de4df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the vector at `this + 0x70` (the start direction).
pub fn fn_00de4df0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x70))
}

// Translated from 00de4e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the vector at `this + 0x80` (the centre).
pub fn fn_00de4e00(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x80))
}

// Translated from 00de4e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the block at `this + 0x90` (the start angle, end angle, radius and segment
/// count).
pub fn fn_00de4e10(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x90))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_4de0, fn_00de4de0(Ptr) -> Ptr),
        entry!(0x00de_4df0, fn_00de4df0(Ptr) -> Ptr),
        entry!(0x00de_4e00, fn_00de4e00(Ptr) -> Ptr),
        entry!(0x00de_4e10, fn_00de4e10(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answer(address: u32) -> u32 {
        let mut e = Engine::new();
        e.call(address, &args![0x1000u32]).u32()
    }

    #[test]
    fn the_axis_is_at_0x60() {
        assert_eq!(answer(0x00de_4de0), 0x1060);
    }

    #[test]
    fn the_start_direction_is_at_0x70() {
        assert_eq!(answer(0x00de_4df0), 0x1070);
    }

    #[test]
    fn the_centre_is_at_0x80() {
        assert_eq!(answer(0x00de_4e00), 0x1080);
    }

    #[test]
    fn the_angle_block_is_at_0x90() {
        assert_eq!(answer(0x00de_4e10), 0x1090);
    }
}
