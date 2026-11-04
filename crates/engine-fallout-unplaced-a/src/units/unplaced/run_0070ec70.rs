//! `(unplaced)/run_0070ec70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_ftol2` (`00ec62c0`): truncates ST0 to an integer (a leading `f64`
/// argument in the uniform form).
const FTOL: u32 = 0x00ec_62c0;

// Translated from 0070ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the float at `this + 0x44` truncated to an integer
/// (`_ftol2`).
pub fn fn_0070ec70(e: &mut Engine, this: Ptr) -> i32 {
    let value = x87_load32(e.mem.u32(this.addr() + 0x44));
    e.call(FTOL, &args![value]).i32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_ec70, fn_0070ec70(Ptr) -> i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0070ec70_truncates_the_float_at_44() {
        let mut e = Engine::new();
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        let this = e.mem.alloc(0x48);
        e.mem.set_f32(this + 0x44, -7.75);
        assert_eq!(e.call(0x0070_ec70, &args![this]).i32(), -7);
    }
}
