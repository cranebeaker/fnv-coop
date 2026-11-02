//! `NiXenonRenderer/run_00e983f0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_splitpath_s` (`00ec7f1e`; the engine map calls it `NiFilename::Splitpath`), `cdecl`,
/// nine words.
const SPLITPATH_S: u32 = 0x00ec_7f1e;

// Translated from 00e983f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: splits a path (`cdecl`, five words: the path and the drive,
/// directory, name and extension buffers) with `_splitpath_s` (buffer sizes 3, 0x104, 0x104 and
/// 0x100) and answers true.
pub fn fn_00e983f0(
    e: &mut Engine,
    path: u32,
    drive: u32,
    directory: u32,
    name: u32,
    extension: u32,
) -> bool {
    e.call(
        SPLITPATH_S,
        &args![path, drive, 3u32, directory, 0x104u32, name, 0x104u32, extension, 0x100u32],
    );
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e9_83f0,
        fn_00e983f0(u32, u32, u32, u32, u32) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e983f0_passes_the_buffer_sizes() {
        let mut e = Engine::new();
        e.register(SPLITPATH_S, |_, _| 0x16u32.into_ret());
        e.call_log = Some(vec![]);
        let got = e.call(0x00e9_83f0, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert!(got.bool());
        let log = e.call_log.take().unwrap();
        let split: Vec<_> = log.iter().filter(|c| c.0 == SPLITPATH_S).collect();
        assert_eq!(split[0].1, vec![1, 2, 3, 3, 0x104, 4, 0x104, 5, 0x100]);
    }
}
