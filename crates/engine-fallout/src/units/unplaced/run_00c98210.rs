//! `(unplaced)/run_00c98210` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c98210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. cdecl `max(first, second)` on two `float`s, result in ST0:
/// `first` when `second < first` (an ordered compare, `FCOM` with
/// `TEST AH,5`), otherwise `second`, so a NaN in either gives `second`.
/// Both pass through `FLD`/`FSTP`, which quiets a signalling NaN.
pub fn fn_00c98210(_e: &mut Engine, first: f32, second: f32) -> f32 {
    let (first, second) = (x87_quiet(first), x87_quiet(second));
    if second < first {
        first
    } else {
        second
    }
}

// Translated from 00c98240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00c98370`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00c98240(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_8370, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c98210, fn_00c98210(f32, f32) -> f32),
        entry!(0x00c9_8240, fn_00c98240(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c98210_is_the_larger_and_prefers_the_second_on_ties_and_nans() {
        let mut e = Engine::new();
        let mut max = |a: f32, b: f32| e.call(0x00c9_8210, &args![a, b]).f32();
        assert_eq!(max(3.0, 2.0), 3.0);
        assert_eq!(max(2.0, 3.0), 3.0);
        assert_eq!(max(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(max(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(max(f32::NAN, 1.0), 1.0);
        assert!(max(1.0, f32::NAN).is_nan());
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00c98240() {
        assert_eq!(check(0x00c9_8240, 0x00c9_8370, 0x8), 0xdead_beef);
    }
}
