//! `NiMain/run_00a88ac0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a88ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): a growth size: the first of 0x20, 0x40,
/// 0x80, 0xff, 0x200 and 0x400 that is above `size` (unsigned), else `size + 1`.
pub fn fn_00a88ac0(_e: &mut Engine, size: u32) -> u32 {
    for step in [0x20u32, 0x40, 0x80, 0xff, 0x200, 0x400] {
        if size < step {
            return step;
        }
    }
    size.wrapping_add(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_8ac0, fn_00a88ac0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_next_step() {
        let mut e = Engine::new();
        for (size, answer) in [
            (0u32, 0x20u32),
            (0x1f, 0x20),
            (0x20, 0x40),
            (0x7f, 0x80),
            (0x80, 0xff),
            (0xfe, 0xff),
            (0xff, 0x200),
            (0x3ff, 0x400),
            (0x400, 0x401),
            (0xffff_ffff, 0),
        ] {
            assert_eq!(e.call(0x00a8_8ac0, &args![size]).u32(), answer, "{size:#x}");
        }
    }
}
