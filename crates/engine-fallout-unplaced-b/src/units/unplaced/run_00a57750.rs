//! `(unplaced)/run_00a57750` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00aa13e0`, `cdecl (size)`) and the
/// `NiBlendBoolInterpolator` constructor (`00a56560`, `thiscall (this,
/// a, float, b)`, `RET 0xC`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const BLEND_BOOL_CONSTRUCTOR: u32 = 0x00a5_6560;

// Translated from 00a57750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes an `NiBlendBoolInterpolator` (the engine map has no name;
/// `thiscall`, five stack words, `RET 0x14`): `operator new(0x34)` and, when it
/// succeeds, the constructor with the second word, the fourth (a `float`,
/// through the FPU) and the fifth, answering the constructor's `EAX`; null
/// when the allocation fails. The first and third words are never read.
/// `this` (`ECX`) is not read, but it sits in vtables (slot 55) and is called with `ECX` set.
pub fn fn_00a57750(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    first: u32,
    _unused_3: u32,
    weight: f32,
    second: u32,
) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    if object == 0 {
        return 0;
    }
    let weight = x87_quiet_bits(weight.to_bits());
    e.call(
        BLEND_BOOL_CONSTRUCTOR,
        &args![object, first, weight, second],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_7750, fn_00a57750(Ptr, u32, u32, u32, f32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_bool_factory_constructs_in_a_new_block() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BLEND_BOOL_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e
            .call(
                0x00a5_7750,
                &args![0x5555u32, 1u32, 2u32, 3u32, 0.5f32, 5u32],
            )
            .u32();
        assert_ne!(object, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (OPERATOR_NEW, vec![0x34]));
        assert_eq!(
            log[2],
            (BLEND_BOOL_CONSTRUCTOR, vec![object, 2, 0.5f32.to_bits(), 5])
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00a5_7750,
                &args![0x5555u32, 1u32, 2u32, 3u32, 0.5f32, 5u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
