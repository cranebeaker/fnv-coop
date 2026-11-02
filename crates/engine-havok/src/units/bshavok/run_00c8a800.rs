//! `BSHavok/run_00c8a800` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The routine run first (`00c879e0`, `this` plus the output block).
const FILL_BASE: u32 = 0x00c8_79e0;

// Translated from 00c8a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c8a800` (unnamed in the Xbox PDB): takes an output block. Runs `00c879e0(this, out)`,
/// then stores at `out + 4` and `out + 8` the words at `+0x18` and `+0x1c` of the object at
/// `this + 8`, or 0 and 0 when `this` is 0 or that object is 0. The exe tests `this` for 0
/// only after the first call; the translation does the same.
pub fn fn_00c8a800(e: &mut Engine, this: u32, out: u32) {
    e.call(FILL_BASE, &args![this, out]);
    let object = if this == 0 { 0 } else { e.mem.u32(this + 8) };
    let (first, second) = if object == 0 {
        (0, 0)
    } else {
        (e.mem.u32(object + 0x18), e.mem.u32(object + 0x1c))
    };
    e.mem.set_u32(out + 4, first);
    e.mem.set_u32(out + 8, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c8a800, fn_00c8a800(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn copies_the_two_words_of_the_inner_object() {
        let mut e = logging_engine();
        e.register(FILL_BASE, nothing);
        let this = e.mem.alloc(0x20);
        let inner = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, inner);
        e.mem.set_u32(inner + 0x18, 0x1111);
        e.mem.set_u32(inner + 0x1c, 0x2222);
        e.call(0x00c8_a800, &args![this, out]);
        assert_eq!(calls_to(&e, FILL_BASE), [[this, out]]);
        assert_eq!(e.mem.u32(out + 4), 0x1111);
        assert_eq!(e.mem.u32(out + 8), 0x2222);
    }

    #[test]
    fn stores_zeros_without_an_inner_object() {
        let mut e = logging_engine();
        e.register(FILL_BASE, nothing);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 4, 5);
        e.mem.set_u32(out + 8, 6);
        e.call(0x00c8_a800, &args![this, out]);
        assert_eq!((e.mem.u32(out + 4), e.mem.u32(out + 8)), (0, 0));
    }

    #[test]
    fn stores_zeros_for_a_null_object() {
        let mut e = logging_engine();
        e.register(FILL_BASE, nothing);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 4, 5);
        e.call(0x00c8_a800, &args![0u32, out]);
        assert_eq!(calls_to(&e, FILL_BASE), [[0, out]]);
        assert_eq!((e.mem.u32(out + 4), e.mem.u32(out + 8)), (0, 0));
    }
}
