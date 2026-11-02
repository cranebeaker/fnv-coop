//! `(unplaced)/run_00bc3040` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The receiver's pointer at +0x2C and the float at +0x40 of the object it points to.
const INNER: u32 = 0x2c;
const INNER_FLOAT: u32 = 0x40;

// Translated from 00bc3040 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, one word: the output float's address; `RET 4`): slot 59 of the
/// vtable `010ba944` (a class without RTTI). Stores at `out` the float at +0x40 of the object
/// the receiver's +0x2C points to (`FLD`/`FSTP`, so a signalling NaN comes out quiet).
/// Returns nothing.
pub fn fn_00bc3040(e: &mut Engine, this: Ptr, out: u32) {
    let inner = e.mem.u32(this.addr() + INNER);
    let bits = e.mem.u32(inner + INNER_FLOAT);
    e.mem.set_u32(out, x87_quiet_bits(bits));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_3040, fn_00bc3040(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_getter_copies_the_inner_float() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let inner = e.mem.alloc(0x44);
        let out = e.mem.alloc(4);
        e.mem.set_u32(this + 0x2c, inner);
        e.mem.set_f32(inner + 0x40, 2.5);
        e.call(0x00bc_3040, &args![this, out]);
        assert_eq!(e.mem.f32(out), 2.5);
        // A signalling NaN passes through the x87 stack quieted.
        e.mem.set_u32(inner + 0x40, 0x7f80_0001);
        e.call(0x00bc_3040, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
    }
}
