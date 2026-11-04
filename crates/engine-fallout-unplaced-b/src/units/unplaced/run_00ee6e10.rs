//! `(unplaced)/run_00ee6e10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The slot `00ee6e10` jumps through.
const IMPORT_SLOT_00EE6E10: u32 = 0x00fd_f404;

// Translated from 00ee6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf404]` (the import slot; Ghidra labels the import `D3DXCreateVolumeTextureFromFileInMemory`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e68e18 (four pushes).
pub fn fn_00ee6e10(e: &mut Engine, word_0: u32, word_1: u32, word_2: u32, word_3: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6E10, &args![word_0, word_1, word_2, word_3])
        .u32()
}

/// The slot `00ee6e16` jumps through.
const IMPORT_SLOT_00EE6E16: u32 = 0x00fd_f400;

// Translated from 00ee6e16 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf400]` (the import slot; Ghidra labels the import `D3DXCreateCubeTextureFromFileInMemory`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e68df3 (four pushes).
pub fn fn_00ee6e16(e: &mut Engine, word_0: u32, word_1: u32, word_2: u32, word_3: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6E16, &args![word_0, word_1, word_2, word_3])
        .u32()
}

/// The slot `00ee6e1c` jumps through.
const IMPORT_SLOT_00EE6E1C: u32 = 0x00fd_f3fc;

// Translated from 00ee6e1c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3fc]` (the import slot; Ghidra labels the import `D3DXCreateTextureFromFileInMemory`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e68dcd (four pushes).
pub fn fn_00ee6e1c(e: &mut Engine, word_0: u32, word_1: u32, word_2: u32, word_3: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6E1C, &args![word_0, word_1, word_2, word_3])
        .u32()
}

/// The slot `00ee6e22` jumps through.
const IMPORT_SLOT_00EE6E22: u32 = 0x00fd_f3f8;

// Translated from 00ee6e22 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3f8]` (the import slot; Ghidra labels the import `D3DXGetImageInfoFromFileInMemory`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e68bcd (three pushes); the caller tests the result for a negative value.
pub fn fn_00ee6e22(e: &mut Engine, word_0: u32, word_1: u32, word_2: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6E22, &args![word_0, word_1, word_2])
        .i32()
}

/// The slot `00ee6e28` jumps through.
const IMPORT_SLOT_00EE6E28: u32 = 0x00fd_f3f4;

// Translated from 00ee6e28 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3f4]` (the import slot; Ghidra labels the import `D3DXGetPixelShaderProfile`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e6bea0 (one push).
pub fn fn_00ee6e28(e: &mut Engine, word_0: u32) -> Ptr {
    e.call(IMPORT_SLOT_00EE6E28, &args![word_0]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_6e10, fn_00ee6e10(u32, u32, u32, u32) -> u32),
        entry!(0x00ee_6e16, fn_00ee6e16(u32, u32, u32, u32) -> u32),
        entry!(0x00ee_6e1c, fn_00ee6e1c(u32, u32, u32, u32) -> u32),
        entry!(0x00ee_6e22, fn_00ee6e22(u32, u32, Ptr) -> i32),
        entry!(0x00ee_6e28, fn_00ee6e28(u32) -> Ptr),
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
    fn thunk_00ee6e10() {
        import(
            0x00ee_6e10,
            IMPORT_SLOT_00EE6E10,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e16() {
        import(
            0x00ee_6e16,
            IMPORT_SLOT_00EE6E16,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e1c() {
        import(
            0x00ee_6e1c,
            IMPORT_SLOT_00EE6E1C,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e22() {
        import(
            0x00ee_6e22,
            IMPORT_SLOT_00EE6E22,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e28() {
        import(0x00ee_6e28, IMPORT_SLOT_00EE6E28, &[0x1001], 0xdead_beef);
    }
}
