//! `(unplaced)/run_00456a20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `FileFinder` instance pointer (`011f81dc`; 0 before it is built).
const FILE_FINDER_INSTANCE: u32 = 0x011f_81dc;
/// The instance method `00afe0d0` (`thiscall`, four stack words, `RET 0x10`).
const FILE_FINDER_EXIST_METHOD: u32 = 0x00af_e0d0;

// Translated from 00456a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FileFinder::Exist` (engine map name; `cdecl`, four stack words): answers 0
/// when the `FileFinder` instance (`011f81dc`) is null; otherwise forwards the
/// four words, in order, to `00afe0d0` on the instance and answers its result.
pub fn file_finder_exist(e: &mut Engine, path: u32, second: u32, third: u32, fourth: u32) -> u32 {
    let instance = e.global::<u32>(FILE_FINDER_INSTANCE);
    if instance == 0 {
        return 0;
    }
    e.call(
        FILE_FINDER_EXIST_METHOD,
        &args![instance, path, second, third, fourth],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00456a20, file_finder_exist(u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_an_instance_the_answer_is_zero() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.register(FILE_FINDER_EXIST_METHOD, |_, _| panic!("no instance"));
        assert_eq!(e.call(0x00456a20, &args![1u32, 2u32, 3u32, 4u32]).u32(), 0);
    }

    #[test]
    fn with_an_instance_the_words_are_forwarded() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.set_global(FILE_FINDER_INSTANCE, 0x7000u32);
        e.register(FILE_FINDER_EXIST_METHOD, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x00456a20, &args![1u32, 2u32, 3u32, u32::MAX]).u32();
        assert_eq!(result, 101);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(FILE_FINDER_EXIST_METHOD, vec![0x7000, 1, 2, 3, u32::MAX])]
        );
    }
}
