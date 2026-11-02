//! `fallout/misc/run_0084bb50` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSSaveGameBuffer::BGSSaveGameBuffer` (`00865bd0`).
const SAVE_GAME_BUFFER_CONSTRUCT: u32 = 0x0086_5bd0;
/// `BGSSaveGameBuffer::~BGSSaveGameBuffer` (`00865c10`).
const SAVE_GAME_BUFFER_DESTRUCT: u32 = 0x0086_5c10;
/// `BGSSaveGameBuffer::Save` (`00865c40`, thiscall: the file).
const SAVE_GAME_BUFFER_SAVE: u32 = 0x0086_5c40;
/// Clears the buffer's position and overhead (`00847db0`).
const SAVE_GAME_BUFFER_RESET: u32 = 0x0084_7db0;
/// `BGSSaveLoadGlobalData::SaveGlobalData` (`0084bcd0`, cdecl: the index
/// and the buffer; a `bool` in AL).
const SAVE_GLOBAL_DATA: u32 = 0x0084_bcd0;
/// `SaveGameDataToFileFunction` (`00845d00`, cdecl: file, source, size).
const SAVE_DATA_TO_FILE: u32 = 0x0084_5d00;
/// Size of a `BGSSaveGameBuffer` (Xbox PDB and the layout: 0x14).
const SAVE_GAME_BUFFER_SIZE: u32 = 0x14;

// Translated from 0084bb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084bb50 - the function's C++ exception frame (FS:[0], unwind state for the stack BGSSaveGameBuffer) is not translated
/// Saves the global data blocks `first..end` to `file` (cdecl; unnamed in
/// the engine map): a `BGSSaveGameBuffer` on the stack, then for each index
/// whose `BGSSaveLoadGlobalData::SaveGlobalData` fills the buffer, the
/// index (4 bytes, `SaveGameDataToFileFunction`), the buffer
/// (`BGSSaveGameBuffer::Save`) and the buffer's position reset
/// (`00847db0`). The buffer is destroyed at the end. Returns how many
/// blocks were saved.
pub fn fn_0084bb50(e: &mut Engine, file: Ptr, first: u32, end: u32) -> u32 {
    // The index lives on the stack (its address is written to the file),
    // the buffer 8 bytes above it.
    e.with_stack(8 + SAVE_GAME_BUFFER_SIZE, |e, frame| {
        let index = frame.addr();
        let buffer = frame.addr() + 8;
        let mut saved = 0u32;
        e.call(SAVE_GAME_BUFFER_CONSTRUCT, &args![buffer]);
        e.mem.set_u32(index, first);
        while e.mem.u32(index) < end {
            let current = e.mem.u32(index);
            if e.call(SAVE_GLOBAL_DATA, &args![current, buffer]).u32() & 0xff != 0 {
                e.call(SAVE_DATA_TO_FILE, &args![file, index, 4u32]);
                e.call(SAVE_GAME_BUFFER_SAVE, &args![buffer, file]);
                e.call(SAVE_GAME_BUFFER_RESET, &args![buffer]);
                saved = saved.wrapping_add(1);
            }
            let next = e.mem.u32(index).wrapping_add(1);
            e.mem.set_u32(index, next);
        }
        e.call(SAVE_GAME_BUFFER_DESTRUCT, &args![buffer]);
        saved
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_bb50, fn_0084bb50(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn saves_each_filled_block_with_its_index_and_counts_them() {
        let mut e = Engine::new();
        for callee in [
            SAVE_GAME_BUFFER_CONSTRUCT,
            SAVE_GAME_BUFFER_DESTRUCT,
            SAVE_GAME_BUFFER_SAVE,
            SAVE_GAME_BUFFER_RESET,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // Indexes 3 and 5 have data.
        e.register(SAVE_GLOBAL_DATA, |_, a| Ret {
            eax: if a[0] == 3 || a[0] == 5 {
                0x0000_0101
            } else {
                0x0000_0100
            },
            ..Ret::default()
        });
        // The file double records the index word it is given.
        e.map(0x0320_0000, 0x10);
        e.register(SAVE_DATA_TO_FILE, |e, a| {
            assert_eq!(a[2], 4);
            let used = e.mem.u32(0x0320_0000);
            let index = e.mem.u32(a[1]);
            e.mem.set_u32(0x0320_0004 + used * 4, index);
            e.mem.set_u32(0x0320_0000, used + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let saved = e
            .call(0x0084_bb50, &args![0x0777_0000u32, 2u32, 6u32])
            .u32();
        assert_eq!(saved, 2);
        assert_eq!(e.mem.u32(0x0320_0000), 2);
        assert_eq!(e.mem.u32(0x0320_0004), 3);
        assert_eq!(e.mem.u32(0x0320_0008), 5);
        let buffer = calls_to(&e, SAVE_GAME_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, SAVE_GLOBAL_DATA),
            vec![
                vec![2, buffer],
                vec![3, buffer],
                vec![4, buffer],
                vec![5, buffer]
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_GAME_BUFFER_SAVE),
            vec![vec![buffer, 0x0777_0000]; 2]
        );
        assert_eq!(calls_to(&e, SAVE_GAME_BUFFER_RESET), vec![vec![buffer]; 2]);
        assert_eq!(calls_to(&e, SAVE_GAME_BUFFER_DESTRUCT), vec![vec![buffer]]);

        // An empty range constructs and destroys the buffer only.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_bb50, &args![0u32, 7u32, 7u32]).u32(), 0);
        assert!(calls_to(&e, SAVE_GLOBAL_DATA).is_empty());
        assert_eq!(calls_to(&e, SAVE_GAME_BUFFER_DESTRUCT).len(), 1);
    }
}
