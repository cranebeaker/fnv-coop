//! `BSAudio/run_00ad8790` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ad87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that answers 0 (no PDB name; `this` is not read, no stack
/// words).
pub fn fn_00ad87c0(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// `BSAudio`'s destructor body (`00ad7000`, `thiscall`, no stack words: its virtual table
/// `010a323c`, then the member at +0x14), the heap the deleting destructor frees into
/// (`011f6238`) and its free (`00aa4060`, `thiscall`, one word).
const BS_AUDIO_DESTRUCTOR: u32 = 0x00ad_7000;
const HEAP: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;

// Translated from 00ad8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot 0 of the `BSAudio` virtual table (`010a323c`, RTTI
/// `.?AVBSAudio@@`), the scalar deleting destructor (`thiscall`, one word of flags): the
/// destructor body `00ad7000`, then with flag bit 0 the block freed into the heap at
/// `011f6238` (`00aa4060`). Answers `this`.
pub fn fn_00ad8790(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BS_AUDIO_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ad_8790, fn_00ad8790(Ptr, u32) -> Ptr),
        entry!(0x00ad_87c0, fn_00ad87c0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ad87c0_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ad_87c0, &args![0x5000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00ad8790_destroys_then_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(BS_AUDIO_DESTRUCTOR, |_, _| Ret::default());
        e.register(HEAP_FREE, |_, _| Ret::default());
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_8790, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert_eq!(e.call(0x00ad_8790, &args![0x5000u32, 3u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![HEAP, 0x5000]]);
        assert_eq!(calls_to(&e, BS_AUDIO_DESTRUCTOR), vec![vec![0x5000]; 2]);
    }
}
