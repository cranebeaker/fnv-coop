//! `fallout/magic/run_0081c530` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MagicHitEffect` constructor `0081b650` (`thiscall`, no stack words).
const MAGIC_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_b650;
/// The `MagicModelHitEffect` vtable (`0107b24c`, RTTI `.?AVMagicModelHitEffect@@`).
const MAGIC_MODEL_HIT_EFFECT_VTABLE: u32 = 0x0107_b24c;
/// `00633c90` (`thiscall`, one word): constructs a reference-counted pointer from the word.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, one word): assigns the word to a reference-counted pointer.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0044dee0` (`thiscall`, no stack words): constructs the member at +0x38
/// (`LoadedDataSubBuffer`, Xbox PDB).
const SUB_BUFFER_CONSTRUCT: u32 = 0x0044_dee0;

// Translated from 0081c530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081c530 - the FS:[0] exception frame (destroying the members built so far when a constructor throws) is not translated
/// The `MagicModelHitEffect` constructor (by the vtable it stores; unnamed in the engine
/// map; `thiscall`, plain `RET`): the `MagicHitEffect` constructor `0081b650`, the vtable,
/// null `spObject` (+0x30) and `spTarget` (+0x34) (`00633c90`), `LoadedDataSubBuffer`
/// (+0x38, `0044dee0`), both pointers assigned null again (`0066b0d0`), then `b3rdPerson`
/// (+0x29), `pFilename` (+0x2C) and `bOwnPath` (+0x28) cleared (Xbox PDB names). Returns
/// `this`.
pub fn fn_0081c530(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, MAGIC_MODEL_HIT_EFFECT_VTABLE);
    e.call(POINTER_CONSTRUCT, &args![base + 0x30, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base + 0x34, 0u32]);
    e.call(SUB_BUFFER_CONSTRUCT, &args![base + 0x38]);
    e.call(POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    e.mem.set_u8(base + 0x29, 0);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u8(base + 0x28, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_c530, fn_0081c530(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn constructs_the_model_hit_effect() {
        let mut e = Engine::new();
        for callee in [
            MAGIC_HIT_EFFECT_CONSTRUCTOR,
            POINTER_CONSTRUCT,
            POINTER_ASSIGN,
            SUB_BUFFER_CONSTRUCT,
        ] {
            e.register(callee, |_, a| a[0].into_ret());
        }
        let this = e.mem.alloc(0x3c);
        for i in 0..15 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_c530, &[this]).u32(), this);
        assert_eq!(calls(&e, MAGIC_HIT_EFFECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls(&e, POINTER_CONSTRUCT),
            vec![vec![this + 0x30, 0], vec![this + 0x34, 0]]
        );
        assert_eq!(calls(&e, SUB_BUFFER_CONSTRUCT), vec![vec![this + 0x38]]);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![this + 0x30, 0], vec![this + 0x34, 0]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_MODEL_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x28), 0xffff_0000);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
    }
}
