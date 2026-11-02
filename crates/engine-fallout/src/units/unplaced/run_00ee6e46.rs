//! `(unplaced)/run_00ee6e46` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The slot `00ee6e46` jumps through.
const IMPORT_SLOT_00EE6E46: u32 = 0x00fd_f3e0;

// Translated from 00ee6e46 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3e0]` (the import slot; Ghidra labels the import `D3DXCompileShaderFromFileA`, unconfirmed beyond that label). stdcall (the callee pops its words): the 9 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e9636b (nine pushes); the caller tests the result for a negative value.
pub fn fn_00ee6e46(
    e: &mut Engine,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6E46,
        &args![word_0, word_1, word_2, word_3, word_4, word_5, word_6, word_7, word_8],
    )
    .i32()
}

/// The slot `00ee6e4c` jumps through.
const IMPORT_SLOT_00EE6E4C: u32 = 0x00fd_f3dc;

// Translated from 00ee6e4c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3dc]` (the import slot; Ghidra labels the import `D3DXAssembleShaderFromFileA`, unconfirmed beyond that label). stdcall (the callee pops its words): the 6 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e97f5a (six pushes); the caller tests the result for a negative value.
pub fn fn_00ee6e4c(
    e: &mut Engine,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6E4C,
        &args![word_0, word_1, word_2, word_3, word_4, word_5],
    )
    .i32()
}

/// The slot `00ee6e52` jumps through.
const IMPORT_SLOT_00EE6E52: u32 = 0x00fd_f3d8;

// Translated from 00ee6e52 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3d8]` (the import slot; Ghidra labels the import `D3DXAssembleShader`, unconfirmed beyond that label). stdcall (the callee pops its words): the 7 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00e980e0 (seven pushes); the caller tests the result for a negative value.
pub fn fn_00ee6e52(
    e: &mut Engine,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6E52,
        &args![word_0, word_1, word_2, word_3, word_4, word_5, word_6],
    )
    .i32()
}

/// The slot `00ee7f10` jumps through.
const IMPORT_SLOT_00EE7F10: u32 = 0x00fd_f02c;

// Translated from 00ee7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf02c]` (the import slot; Ghidra labels the import `DirectInput8Create`, unconfirmed beyond that label). stdcall (the callee pops its words): the 5 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00a226c5 (five pushes).
pub fn fn_00ee7f10(
    e: &mut Engine,
    word_0: u32,
    word_1: u32,
    word_2: u32,
    word_3: Ptr,
    word_4: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE7F10,
        &args![word_0, word_1, word_2, word_3, word_4],
    )
    .i32()
}

/// The slot `00ee81e0` jumps through.
const IMPORT_SLOT_00EE81E0: u32 = 0x00fd_f034;

// Translated from 00ee81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf034]` (the import slot; Ghidra labels the import `DirectSoundCreate8`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00aea2be (three pushes).
pub fn fn_00ee81e0(e: &mut Engine, word_0: u32, word_1: Ptr, word_2: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE81E0, &args![word_0, word_1, word_2])
        .i32()
}

/// The slot `00ee81e6` jumps through.
const IMPORT_SLOT_00EE81E6: u32 = 0x00fd_f43c;

// Translated from 00ee81e6 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf43c]` (the import slot; Ghidra labels the import `ov_clear`, unconfirmed beyond that label). cdecl (the caller pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af1d9b (`ADD ESP,0x4`).
pub fn fn_00ee81e6(e: &mut Engine, word_0: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE81E6, &args![word_0]).i32()
}

/// The slot `00ee81ec` jumps through.
const IMPORT_SLOT_00EE81EC: u32 = 0x00fd_f440;

// Translated from 00ee81ec (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf440]` (the import slot; Ghidra labels the import `ov_pcm_total`, unconfirmed beyond that label). cdecl (the caller pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af20d3 (`ADD ESP,0x8`); that caller keeps only `EAX` of the result (a 64-bit value would also use `EDX`), so only `EAX` is returned here.
pub fn fn_00ee81ec(e: &mut Engine, word_0: Ptr, word_1: i32) -> u32 {
    e.call(IMPORT_SLOT_00EE81EC, &args![word_0, word_1]).u32()
}

/// The slot `00ee81f2` jumps through.
const IMPORT_SLOT_00EE81F2: u32 = 0x00fd_f444;

// Translated from 00ee81f2 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf444]` (the import slot; Ghidra labels the import `ov_seekable`, unconfirmed beyond that label). cdecl (the caller pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af20be (`ADD ESP,0x4`); the result is tested against zero.
pub fn fn_00ee81f2(e: &mut Engine, word_0: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE81F2, &args![word_0]).u32()
}

/// The slot `00ee81f8` jumps through.
const IMPORT_SLOT_00EE81F8: u32 = 0x00fd_f44c;

// Translated from 00ee81f8 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf44c]` (the import slot; Ghidra labels the import `ov_info`, unconfirmed beyond that label). cdecl (the caller pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af20a3 (`ADD ESP,0x8`; the second word is -1).
pub fn fn_00ee81f8(e: &mut Engine, word_0: Ptr, word_1: i32) -> u32 {
    e.call(IMPORT_SLOT_00EE81F8, &args![word_0, word_1]).u32()
}

/// The slot `00ee81fe` jumps through.
const IMPORT_SLOT_00EE81FE: u32 = 0x00fd_f438;

// Translated from 00ee81fe (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf438]` (the import slot; Ghidra labels the import `ov_open_callbacks`, unconfirmed beyond that label). cdecl (the caller pops its words): the 8 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af2004 (`ADD ESP,0x20`: eight words).
pub fn fn_00ee81fe(
    e: &mut Engine,
    word_0: u32,
    word_1: Ptr,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE81FE,
        &args![word_0, word_1, word_2, word_3, word_4, word_5, word_6, word_7],
    )
    .i32()
}

/// The slot `00ee8204` jumps through.
const IMPORT_SLOT_00EE8204: u32 = 0x00fd_f448;

// Translated from 00ee8204 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf448]` (the import slot; Ghidra labels the import `ov_read`, unconfirmed beyond that label). cdecl (the caller pops its words): the 7 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00af2145 (`ADD ESP,0x1c`: seven words).
pub fn fn_00ee8204(
    e: &mut Engine,
    word_0: Ptr,
    word_1: Ptr,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: Ptr,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE8204,
        &args![word_0, word_1, word_2, word_3, word_4, word_5, word_6],
    )
    .i32()
}

/// The slot `00ee820a` jumps through.
const IMPORT_SLOT_00EE820A: u32 = 0x00fd_f398;

// Translated from 00ee820a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf398]` (the import slot; Ghidra labels the import `XInputSetState`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 00a25859 (two pushes, no stack cleanup).
pub fn fn_00ee820a(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE820A, &args![word_0, word_1]).u32()
}

/// The slot `00ee8210` jumps through.
const IMPORT_SLOT_00EE8210: u32 = 0x00fd_f174;

// Translated from 00ee8210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf174]` (the import slot; Ghidra labels the import `RtlUnwind`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee8210(
    e: &mut Engine,
    target_frame: Ptr,
    target_ip: Ptr,
    exception_record: Ptr,
    return_value: Ptr,
) {
    e.call(
        IMPORT_SLOT_00EE8210,
        &args![target_frame, target_ip, exception_record, return_value],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_6e46, fn_00ee6e46(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ee_6e4c, fn_00ee6e4c(u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ee_6e52, fn_00ee6e52(u32, u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ee_7f10, fn_00ee7f10(u32, u32, u32, Ptr, u32) -> i32),
        entry!(0x00ee_81e0, fn_00ee81e0(u32, Ptr, u32) -> i32),
        entry!(0x00ee_81e6, fn_00ee81e6(Ptr) -> i32),
        entry!(0x00ee_81ec, fn_00ee81ec(Ptr, i32) -> u32),
        entry!(0x00ee_81f2, fn_00ee81f2(Ptr) -> u32),
        entry!(0x00ee_81f8, fn_00ee81f8(Ptr, i32) -> u32),
        entry!(0x00ee_81fe, fn_00ee81fe(u32, Ptr, u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ee_8204, fn_00ee8204(Ptr, Ptr, u32, u32, u32, u32, Ptr) -> i32),
        entry!(0x00ee_820a, fn_00ee820a(u32, Ptr) -> u32),
        entry!(0x00ee_8210, fn_00ee8210(Ptr, Ptr, Ptr, Ptr)),
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
    fn thunk_00ee6e46() {
        import(
            0x00ee_6e46,
            IMPORT_SLOT_00EE6E46,
            &[
                0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x1007, 0x1008, 0x1009,
            ],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e4c() {
        import(
            0x00ee_6e4c,
            IMPORT_SLOT_00EE6E4C,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6e52() {
        import(
            0x00ee_6e52,
            IMPORT_SLOT_00EE6E52,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x1007],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee7f10() {
        import(
            0x00ee_7f10,
            IMPORT_SLOT_00EE7F10,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee81e0() {
        import(
            0x00ee_81e0,
            IMPORT_SLOT_00EE81E0,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee81e6() {
        import(0x00ee_81e6, IMPORT_SLOT_00EE81E6, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee81ec() {
        import(
            0x00ee_81ec,
            IMPORT_SLOT_00EE81EC,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee81f2() {
        import(0x00ee_81f2, IMPORT_SLOT_00EE81F2, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee81f8() {
        import(
            0x00ee_81f8,
            IMPORT_SLOT_00EE81F8,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee81fe() {
        import(
            0x00ee_81fe,
            IMPORT_SLOT_00EE81FE,
            &[
                0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x1007, 0x1008,
            ],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee8204() {
        import(
            0x00ee_8204,
            IMPORT_SLOT_00EE8204,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x1007],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee820a() {
        import(
            0x00ee_820a,
            IMPORT_SLOT_00EE820A,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee8210() {
        import(
            0x00ee_8210,
            IMPORT_SLOT_00EE8210,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0,
        );
    }
}
