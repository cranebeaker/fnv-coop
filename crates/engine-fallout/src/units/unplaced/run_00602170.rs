//! `(unplaced)/run_00602170` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00602170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `this + 0x1DC` (the address of a member; the class is not
/// confirmed).
pub fn fn_00602170(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x1dc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0060_2170, fn_00602170(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn at_00602170_returns_the_member_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0060_2170, &[0x3000]).u32(), 0x31dc);
        assert_eq!(e.call(0x0060_2170, &[0xffff_ff00]).u32(), 0xdc);
    }
}
