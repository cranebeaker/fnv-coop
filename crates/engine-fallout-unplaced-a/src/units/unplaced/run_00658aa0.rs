//! `(unplaced)/run_00658aa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00659230`; `thiscall`: a word and a byte).
const BASE_CONSTRUCTOR: u32 = 0x0065_9230;
/// The virtual table the constructor stores (`01065870`).
const CONSTRUCTED_VTABLE: u32 = 0x0106_5870;

// Translated from 00658aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the engine map; `thiscall`, `RET 8`): the base constructor
/// (`00659230`) with the first word and the low byte of the second (zero-extended), then
/// the virtual table `01065870`. Answers `this`.
pub fn fn_00658aa0(e: &mut Engine, this: Ptr, first: u32, flag: u8) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, first, u32::from(flag)]);
    e.mem.set_u32(this.addr(), CONSTRUCTED_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_8aa0, fn_00658aa0(Ptr, u32, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_byte_zero_extended() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_8aa0, &args![this, 5u32, 0x1ffu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), CONSTRUCTED_VTABLE);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|c| c.0 == BASE_CONSTRUCTOR).collect();
        assert_eq!(base[0].1, vec![this, 5, 0xff]);
    }
}
