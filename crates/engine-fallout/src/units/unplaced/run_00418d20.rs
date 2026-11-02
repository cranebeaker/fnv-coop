//! `(unplaced)/run_00418d20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body it runs (`0048fb50`, `animation.cpp`).
const DESTRUCT_0048FB50: u32 = 0x0048_fb50;
/// The game's `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;

// Translated from 00418d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (unnamed in the engine map): the
/// destructor body `0048fb50`, then `00401030(this)` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00418d20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_0048FB50, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0041_8d20, fn_00418d20(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_then_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTRUCT_0048FB50, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0041_8d20, &args![0x5000u32, flags]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.into_iter().filter(|(a, _)| *a != 0x0041_8d20).collect();
            let mut expected = vec![(DESTRUCT_0048FB50, vec![0x5000])];
            if frees {
                expected.push((FREE, vec![0x5000]));
            }
            assert_eq!(calls, expected);
        }
    }
}
