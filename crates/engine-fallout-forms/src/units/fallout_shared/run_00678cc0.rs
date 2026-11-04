//! `fallout shared/run_00678cc0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00678cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the signed byte at `this + 1`
/// (sign-extended into `EAX`).
pub fn fn_00678cc0(e: &mut Engine, this: Ptr) -> i8 {
    e.mem.i8(this.addr().wrapping_add(1))
}

// Translated from 00678ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at `this + 2`.
pub fn fn_00678ce0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(2))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00678cc0, fn_00678cc0(Ptr) -> i8),
        entry!(0x00678ce0, fn_00678ce0(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_two_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.write(this, &[0x11, 0xfe, 0x7f, 0x22]);
        assert_eq!(e.call(0x00678cc0, &args![this]).u32(), 0xffff_fffe);
        assert_eq!(e.call(0x00678ce0, &args![this]).u8(), 0x7f);
    }
}
