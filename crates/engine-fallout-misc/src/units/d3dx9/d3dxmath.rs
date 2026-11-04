//! `d3dx9/d3dxmath.cpp` (Xbox PDB source unit), subsystem `d3dx9`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot `00ee6dd4` jumps through.
const IMPORT_SLOT_00EE6DD4: u32 = 0x00fd_f42c;

// Translated from 00ee6dd4 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf42c]` (Ghidra labels the import
/// `D3DXMatrixTranspose`, unconfirmed beyond that label). stdcall (the callee
/// pops its words): the 2 stack words go unchanged to the import and its result
/// comes back. Argument count from the caller at 004ed04a (two address pushes).
pub fn fn_00ee6dd4(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DD4, &args![word_0, word_1]).ptr()
}

/// The import slot `00ee6dda` jumps through.
const IMPORT_SLOT_00EE6DDA: u32 = 0x00fd_f428;

// Translated from 00ee6dda (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf428]` (Ghidra labels the import
/// `D3DXMatrixInverse`, unconfirmed beyond that label). stdcall: the 3 stack
/// words go unchanged to the import and its result comes back. Argument count
/// from the caller at 004ed037 (an address, 0, an address).
pub fn fn_00ee6dda(e: &mut Engine, word_0: Ptr, word_1: u32, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DDA, &args![word_0, word_1, word_2])
        .ptr()
}

/// The import slot `00ee6de0` jumps through.
const IMPORT_SLOT_00EE6DE0: u32 = 0x00fd_f424;

// Translated from 00ee6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf424]` (Ghidra labels the import
/// `D3DXMatrixMultiply`, unconfirmed beyond that label). stdcall: the 3 stack
/// words go unchanged to the import and its result comes back. Argument count
/// from the caller at 004ed15f (three pushes).
pub fn fn_00ee6de0(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DE0, &args![word_0, word_1, word_2])
        .ptr()
}

/// The import slot `00ee6dec` jumps through.
const IMPORT_SLOT_00EE6DEC: u32 = 0x00fd_f41c;

// Translated from 00ee6dec (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf41c]` (Ghidra labels the import
/// `D3DXVec3Normalize`, unconfirmed beyond that label). stdcall: the 2 stack
/// words go unchanged to the import and its result comes back. Argument count
/// from the caller at 00b7c818 (two address pushes).
pub fn fn_00ee6dec(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DEC, &args![word_0, word_1]).ptr()
}

/// The import slot `00ee6df2` jumps through.
const IMPORT_SLOT_00EE6DF2: u32 = 0x00fd_f420;

// Translated from 00ee6df2 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf420]` (Ghidra labels the import
/// `D3DXVec3TransformNormal`, unconfirmed beyond that label). stdcall: the 3
/// stack words go unchanged to the import and its result comes back. Argument
/// count from the caller at 00b7c809 (three pushes).
pub fn fn_00ee6df2(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DF2, &args![word_0, word_1, word_2])
        .ptr()
}

/// The import slot `00ee6df8` jumps through.
const IMPORT_SLOT_00EE6DF8: u32 = 0x00fd_f414;

// Translated from 00ee6df8 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf414]` (Ghidra labels the import
/// `D3DXVec3TransformCoord`, unconfirmed beyond that label). stdcall: the 3
/// stack words go unchanged to the import and its result comes back. Argument
/// count from the caller at 00b7e270 (an address, `011fa2a0`, an address).
pub fn fn_00ee6df8(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DF8, &args![word_0, word_1, word_2])
        .ptr()
}

/// The import slot `00ee6de6` jumps through.
const IMPORT_SLOT_00EE6DE6: u32 = 0x00fd_f3d0;

// Translated from 00ee6de6 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3d0]` (Ghidra labels the import
/// `D3DXLoadSurfaceFromSurface`, unconfirmed beyond that label). stdcall: the 8
/// stack words go unchanged to the import and its result comes back. Argument
/// count from the caller at 00b6bd75 (eight pushes).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ee6de6(
    e: &mut Engine,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) -> u32 {
    e.call(
        IMPORT_SLOT_00EE6DE6,
        &args![word_0, word_1, word_2, word_3, word_4, word_5, word_6, word_7],
    )
    .u32()
}

/// The import slot `00ee6dfe` jumps through.
const IMPORT_SLOT_00EE6DFE: u32 = 0x00fd_f410;

// Translated from 00ee6dfe (disassembled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf410]` (the exe's import table names the slot
/// `d3dx9_38.dll` `D3DXMatrixMultiplyTranspose`). stdcall: the 3 stack words go unchanged to
/// the import and its result comes back. Argument count from the import (out and two
/// matrices).
pub fn fn_00ee6dfe(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DFE, &args![word_0, word_1, word_2])
        .ptr()
}

/// The import slot `00ee6e04` jumps through.
const IMPORT_SLOT_00EE6E04: u32 = 0x00fd_f40c;

// Translated from 00ee6e04 (disassembled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf40c]` (the exe's import table names the slot
/// `d3dx9_38.dll` `D3DXMatrixRotationYawPitchRoll`). stdcall: the 4 stack words (out, then the
/// yaw, pitch and roll floats) go unchanged to the import and its result comes back. Argument
/// count from the caller at 00bb3034 (three float stores and a push).
pub fn fn_00ee6e04(e: &mut Engine, word_0: Ptr, word_1: u32, word_2: u32, word_3: u32) -> Ptr {
    e.call(IMPORT_SLOT_00EE6E04, &args![word_0, word_1, word_2, word_3])
        .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_6e04, fn_00ee6e04(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ee_6dfe, fn_00ee6dfe(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6dd4, fn_00ee6dd4(Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6dda, fn_00ee6dda(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00ee_6de0, fn_00ee6de0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6dec, fn_00ee6dec(Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6df2, fn_00ee6df2(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6df8, fn_00ee6df8(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00ee_6de6,
            fn_00ee6de6(u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls the thunk at `thunk` with `words` and checks that the import at
    /// `slot` got the same words and that its result came back.
    fn import(thunk: u32, slot: u32, words: &[u32]) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == slot).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, words);
        assert_eq!(result, 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6e04() {
        import(
            0x00ee_6e04,
            IMPORT_SLOT_00EE6E04,
            &[0x1001, 0x3f80_0000, 0xbf80_0000, 0x7fc0_0000],
        );
    }

    #[test]
    fn thunk_00ee6dfe() {
        import(0x00ee_6dfe, IMPORT_SLOT_00EE6DFE, &[0x1001, 0x1002, 0x1003]);
    }

    #[test]
    fn thunk_00ee6dd4() {
        import(0x00ee_6dd4, IMPORT_SLOT_00EE6DD4, &[0x1001, 0x1002]);
    }

    #[test]
    fn thunk_00ee6dda() {
        import(0x00ee_6dda, IMPORT_SLOT_00EE6DDA, &[0x1001, 0, 0x1003]);
    }

    #[test]
    fn thunk_00ee6de0() {
        import(0x00ee_6de0, IMPORT_SLOT_00EE6DE0, &[0x1001, 0x1002, 0x1003]);
    }

    #[test]
    fn thunk_00ee6dec() {
        import(0x00ee_6dec, IMPORT_SLOT_00EE6DEC, &[0x1001, 0x1002]);
    }

    #[test]
    fn thunk_00ee6df2() {
        import(0x00ee_6df2, IMPORT_SLOT_00EE6DF2, &[0x1001, 0x1002, 0x1003]);
    }

    #[test]
    fn thunk_00ee6df8() {
        import(0x00ee_6df8, IMPORT_SLOT_00EE6DF8, &[0x1001, 0x1002, 0x1003]);
    }

    #[test]
    fn thunk_00ee6de6() {
        import(
            0x00ee_6de6,
            IMPORT_SLOT_00EE6DE6,
            &[0x1001, 0, 0, 0x1004, 0, 0, 0xffff_ffff, 0],
        );
    }
}
