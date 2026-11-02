//! `fallout/ai/run_00984230` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00984230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `+0xB0` of `this` (`this + 0xB0`); the
/// engine map has no name and the class is unconfirmed.
pub fn fn_00984230(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xb0)
}

// Translated from 00984250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `+0xB8` of `this` (`this + 0xB8`); the
/// engine map has no name and the class is unconfirmed.
pub fn fn_00984250(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xb8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_4230, fn_00984230(Ptr) -> Ptr),
        entry!(0x0098_4250, fn_00984250(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_member_addresses() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0098_4230, &args![0x1000u32]).u32(), 0x10b0);
        assert_eq!(e.call(0x0098_4250, &args![0x1000u32]).u32(), 0x10b8);
        assert_eq!(e.call(0x0098_4250, &args![0xffff_ff50u32]).u32(), 0x8);
    }
}
