//! `NiAnimation/run_00a574b0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_strncpy_s` (`cdecl`, (destination, size, source, count)) and `_strcat_s` (`cdecl`,
/// (destination, size, source)).
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRCAT_S: u32 = 0x00ec_6bd2;

// Translated from 00a574b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (destination, size, source, count)): `_strncpy_s` with
/// the four words. Answers `destination`.
pub fn fn_00a574b0(e: &mut Engine, dest: u32, size: u32, source: u32, count: u32) -> u32 {
    e.call(STRNCPY_S, &args![dest, size, source, count]);
    dest
}

// Translated from 00a574e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (destination, size, source)): `_strcat_s` with the
/// three words. Answers `destination`.
pub fn fn_00a574e0(e: &mut Engine, dest: u32, size: u32, source: u32) -> u32 {
    e.call(STRCAT_S, &args![dest, size, source]);
    dest
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_74b0, fn_00a574b0(u32, u32, u32, u32) -> u32),
        entry!(0x00a5_74e0, fn_00a574e0(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_helpers_pass_their_words_and_answer_the_destination() {
        let mut e = Engine::new();
        e.register(STRNCPY_S, |_, _| 22u32.into_ret());
        e.register(STRCAT_S, |_, _| 22u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_74b0, &args![1u32, 2u32, 3u32, 4u32]).u32(), 1);
        assert_eq!(e.call(0x00a5_74e0, &args![5u32, 6u32, 7u32]).u32(), 5);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|c| c.0 == STRNCPY_S || c.0 == STRCAT_S)
            .collect();
        assert_eq!(
            calls,
            vec![(STRNCPY_S, vec![1, 2, 3, 4]), (STRCAT_S, vec![5, 6, 7])]
        );
    }
}
