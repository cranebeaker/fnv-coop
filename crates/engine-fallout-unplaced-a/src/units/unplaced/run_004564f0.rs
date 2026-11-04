//! `(unplaced)/run_004564f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_strnicmp` (`00ec7ec0`, `cdecl(first, second, count)`).
const STRNICMP: u32 = 0x00ec_7ec0;

// Translated from 004564f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `cdecl` wrapper: `_strnicmp(first, second, count)`. The function
/// leaves the callee's result in `EAX`, so it is returned.
pub fn fn_004564f0(e: &mut Engine, first: Ptr, second: Ptr, count: u32) -> i32 {
    e.call(STRNICMP, &args![first, second, count]).i32()
}

/// The word `00456510` answers (`011f4500`; read by the map's `TestAllCells`,
/// `DumpTextures`, `OutputMemStats` and `DumpTexturePalette` through it).
const VALUE_011F4500: u32 = 0x011f_4500;

// Translated from 00456510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011f4500`.
pub fn fn_00456510(e: &mut Engine) -> u32 {
    e.global(VALUE_011F4500)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045_64f0, fn_004564f0(Ptr, Ptr, u32) -> i32),
        entry!(0x0045_6510, fn_00456510() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_three_words_and_returns_the_result() {
        let mut e = Engine::new();
        e.register(STRNICMP, |_, _| (-1i32 as u32).into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0045_64f0, &args![0x100u32, 0x200u32, 5u32]).i32();
        assert_eq!(result, -1);
        let calls: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == STRNICMP)
            .collect();
        assert_eq!(calls, vec![(STRNICMP, vec![0x100, 0x200, 5])]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00456510_answers_the_global() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.set_global(VALUE_011F4500, 0x0bad_f00du32);
        assert_eq!(e.call(0x0045_6510, &args![]).u32(), 0x0bad_f00d);
    }
}
