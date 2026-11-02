//! `NiAnimation/run_00a4eaa0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`cdecl`, `(size)`) and the `NiBlendPoint3Interpolator` constructor
/// (`00a55a10`, `thiscall`, three words, the second a float).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const BLEND_POINT3_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_5a10;

// Translated from 00a4eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, five stack words, `RET 0x14`; the first and third are never
/// read): a 0x40-byte block (`operator new`) constructed as an `NiBlendPoint3Interpolator`
/// (`00a55a10`, the vtable's class confirmed by its RTTI) from the second, fourth (a float)
/// and fifth words; null when the allocation fails.
/// `this` (`ECX`) is not read, but it sits in vtables (slot 55) and is called with `ECX` set.
pub fn fn_00a4eaa0(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    second: u32,
    _unused_3: u32,
    float_argument: f32,
    fifth: u32,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(
        BLEND_POINT3_INTERPOLATOR_CONSTRUCTOR,
        &args![
            block,
            second,
            x87_quiet_bits(float_argument.to_bits()),
            fifth
        ],
    )
    .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4eaa0, fn_00a4eaa0(Ptr, u32, u32, u32, f32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a4eaa0_constructs_a_blend_point3_interpolator() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register(BLEND_POINT3_INTERPOLATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let made = e
            .call(
                0x00a4eaa0,
                &args![0x5555u32, 1u32, 2u32, 3u32, 0.5f32, 5u32],
            )
            .u32();
        assert_eq!(made, 0x6000);
        let constructed: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BLEND_POINT3_INTERPOLATOR_CONSTRUCTOR)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(constructed, vec![vec![0x6000, 2, 0.5f32.to_bits(), 5]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(
            e.call(
                0x00a4eaa0,
                &args![0x5555u32, 1u32, 2u32, 3u32, 0.5f32, 5u32]
            )
            .u32(),
            0
        );
    }
}
