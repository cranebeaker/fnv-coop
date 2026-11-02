//! `fallout shared/run_00593370` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetChunkData_ov3` (engine map): `__thiscall` on the file, the
/// destination and the byte count on the stack.
const TES_FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
/// Reads the byte at `file + 0x299` (the engine map gives it no name).
const TES_FILE_BYTE_299: u32 = 0x0040_1680;
/// This unit's function at `00593370` (not translated yet; no name in the
/// engine map).
const FIXUP_00593370: u32 = 0x0059_3370;

// Translated from 00593450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the current chunk of `file` into the first 0x24 bytes of `this`
/// (`TESFile::GetChunkData_ov3`, engine map), then, when the byte at
/// `file + 0x299` (`00401680`) is non-zero, runs `00593370` on `this`.
/// `__thiscall`, one stack word (`RET 4`). The engine map gives it no name.
pub fn fn_00593450(e: &mut Engine, this: Ptr, file: Ptr) {
    e.call(TES_FILE_GET_CHUNK_DATA, &args![file, this, 0x24u32]);
    if e.call(TES_FILE_BYTE_299, &args![file]).bool() {
        e.call(FIXUP_00593370, &args![this]);
    }
}

/// `00401080` (`cdecl`: the address of a word and a byte; the loaders call
/// it on each word of a record they byte-swap).
const SWAP_WORD: u32 = 0x0040_1080;
/// The word offsets `00593370` swaps, in its order.
const DECAL_WORDS: [u32; 8] = [0x00, 0x04, 0x08, 0x0c, 0x18, 0x10, 0x14, 0x20];

// Translated from 00593370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`; the fix-up `00593450` runs on a byte-swapped
/// file): `00401080(address, 0)` on eight words of the 0x24-byte record,
/// in the order of `DECAL_WORDS`.
pub fn fn_00593370(e: &mut Engine, this: Ptr) {
    for offset in DECAL_WORDS {
        e.call(SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_3450, fn_00593450(Ptr, Ptr)),
        entry!(0x0059_3370, fn_00593370(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn reads_the_chunk_and_runs_the_fixup_only_when_the_file_byte_is_set() {
        for flag in [0u32, 1, 0x100] {
            let mut e = Engine::new();
            let this: Ptr = Ptr::new(e.mem.alloc(0x24));
            let file: Ptr = Ptr::new(e.mem.alloc(0x300));
            e.register(TES_FILE_GET_CHUNK_DATA, |_, _| Ret::default());
            e.register_double(TES_FILE_BYTE_299, move |_, _| Ret {
                eax: flag,
                ..Ret::default()
            });
            e.register(FIXUP_00593370, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0059_3450, &args![this, file]);
            assert_eq!(
                calls_to(&e, TES_FILE_GET_CHUNK_DATA),
                vec![vec![file.addr(), this.addr(), 0x24]]
            );
            assert_eq!(calls_to(&e, TES_FILE_BYTE_299), vec![vec![file.addr()]]);
            let expected = if flag as u8 != 0 {
                vec![vec![this.addr()]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&e, FIXUP_00593370), expected);
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00593370_swaps_eight_words_in_order() {
        let mut e = Engine::new();
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_3370, &[0x1000]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == SWAP_WORD)
            .collect();
        let expected: Vec<(u32, Vec<u32>)> = DECAL_WORDS
            .iter()
            .map(|o| (SWAP_WORD, vec![0x1000 + o, 0]))
            .collect();
        assert_eq!(log, expected);
    }
}
