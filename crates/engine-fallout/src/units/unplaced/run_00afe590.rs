//! `(unplaced)/run_00afe590` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `FileFinder::BuildFileList` (engine map), `00afe420` (cdecl, four words).
const BUILD_FILE_LIST: u32 = 0x00af_e420;

// Translated from 00afe590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FileFinder::BuildFileList(first, second, 0xFFFF, third)` (cdecl; no
/// caller or vtable in the exe), whose answer it passes on.
pub fn fn_00afe590(e: &mut Engine, first: u32, second: u32, third: u32) -> u32 {
    e.call(BUILD_FILE_LIST, &[first, second, 0xffff, third])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_e590, fn_00afe590(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_with_all_flags() {
        let mut e = Engine::new();
        e.register(BUILD_FILE_LIST, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_e590, &[1, 2, 3]).u32(), 7);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BUILD_FILE_LIST, vec![1, 2, 0xffff, 3])
        );
    }
}
