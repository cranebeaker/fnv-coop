//! `BSMain/run_00c5ced0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTriShape` default constructor (`00a744a0`, `thiscall`).
const TRI_SHAPE_DEFAULT_CONSTRUCTOR: u32 = 0x00a7_44a0;
/// The `BSScissorTriShape` virtual table (`010c2e7c`, by RTTI).
const SCISSOR_TRI_SHAPE_VTABLE: u32 = 0x010c_2e7c;

// Translated from 00c5ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSScissorTriShape` constructor (by its virtual table; unnamed in the engine map;
/// `thiscall`, `RET 0x10`): the `NiTriShape` default constructor (`00a744a0`), the class's
/// virtual table and its four words at +0xC4, +0xC8, +0xCC and +0xD0. Answers `this`.
pub fn fn_00c5ced0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    e.call(TRI_SHAPE_DEFAULT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), SCISSOR_TRI_SHAPE_VTABLE);
    for (k, word) in [first, second, third, fourth].iter().enumerate() {
        e.mem.set_u32(this.addr() + 0xc4 + 4 * k as u32, *word);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c5_ced0, fn_00c5ced0(Ptr, u32, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn stores_the_vtable_and_the_four_words() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_DEFAULT_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0xd4);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00c5_ced0, &args![this, 1u32, 2u32, 3u32, 4u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert!(log.contains(&(TRI_SHAPE_DEFAULT_CONSTRUCTOR, vec![this])));
        assert_eq!(e.mem.u32(this), SCISSOR_TRI_SHAPE_VTABLE);
        let words: Vec<u32> = (0..4).map(|k| e.mem.u32(this + 0xc4 + 4 * k)).collect();
        assert_eq!(words, vec![1, 2, 3, 4]);
    }
}
