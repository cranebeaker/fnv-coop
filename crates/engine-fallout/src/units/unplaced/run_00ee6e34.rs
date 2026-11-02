//! `(unplaced)/run_00ee6e34` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The slot `00ee6e34` jumps through.
const IMPORT_SLOT_00EE6E34: u32 = 0x00fd_f3ec;

// Translated from 00ee6e34 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3ec]` (the import slot; Ghidra labels the import `D3DXVec4Transform`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e82d62 (three address pushes).
pub fn fn_00ee6e34(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6E34, &args![word_0, word_1, word_2])
        .ptr()
}

/// The slot `00ee6e3a` jumps through.
const IMPORT_SLOT_00EE6E3A: u32 = 0x00fd_f3e8;

// Translated from 00ee6e3a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3e8]` (the import slot; Ghidra labels the import `D3DXCreateBuffer`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e8e0d7 (two pushes); the caller tests the result for a negative value.
pub fn fn_00ee6e3a(e: &mut Engine, word_0: u32, word_1: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6E3A, &args![word_0, word_1]).i32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_6e34, fn_00ee6e34(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6e3a, fn_00ee6e3a(u32, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls `thunk` with `words`, with a double over the import `slot` that
    /// returns `0xdeadbeef`; checks that the double ran once with exactly
    /// those words and that the thunk returned `expected`.
    fn import(thunk: u32, slot: u32, words: &[u32], expected: u32) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == slot).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_00ee6e34() {
        import(
            0x00ee_6e34,
            IMPORT_SLOT_00EE6E34,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e3a() {
        import(
            0x00ee_6e3a,
            IMPORT_SLOT_00EE6E3A,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }
}
