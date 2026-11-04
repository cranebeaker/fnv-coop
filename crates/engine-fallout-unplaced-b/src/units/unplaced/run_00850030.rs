//! `(unplaced)/run_00850030` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `XContentClose` (engine map; `0084ff90`), `thiscall`, three words: the
/// name, the path buffer, the flag. It writes the file's path.
const MAKE_PATH: u32 = 0x0084_ff90;
/// `_access` (engine map), `cdecl`: path, mode; -1 when the file is not
/// there.
const ACCESS: u32 = 0x00ec_bf05;
/// `DeleteFileA` import slot (`kernel32`, stdcall: the path).
const DELETE_FILE_A: u32 = 0x00fd_f0d4;
/// `00846520`: opens the file (`cdecl`: path, flag, buffer size); the
/// file object.
const OPEN_FILE: u32 = 0x0084_6520;
/// `008464f0`: `thiscall` on the file, one word (the name).
const FILE_SET_NAME: u32 = 0x0084_64f0;
/// `008463c0`: `thiscall` on the file.
const FILE_008463C0: u32 = 0x0084_63c0;
/// The path buffer's size (`EBP-0x118` to `EBP-0xc`).
const PATH_BUFFER_SIZE: u32 = 0x10c;

// Translated from 00850030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00850030 - the /GS cookie check (00ec408c) is not translated
/// No map name: opens the save file `name`: builds its path (`0084ff90`),
/// deletes an existing file first when `write` is set, opens it
/// (`00846520(path, write, size)`, the size 0x200000 for `kind` 1 and
/// 0x10000 otherwise), gives it the name (`008464f0`), runs `008463c0` on
/// it, and returns it. The fourth word is passed but not read.
pub fn fn_00850030(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    write: bool,
    kind: u32,
    _unused_4: u32,
) -> u32 {
    e.with_stack(PATH_BUFFER_SIZE, |e, path| {
        let flag = u32::from(write);
        e.call(MAKE_PATH, &args![this, name, path, flag]);
        let size = if kind == 0 {
            0x10000u32
        } else if kind == 1 {
            0x20_0000
        } else {
            0x10000
        };
        if write && e.call(ACCESS, &args![path, 0u32]).i32() != -1 {
            e.call(DELETE_FILE_A, &args![path]);
        }
        let file = e.call(OPEN_FILE, &args![path, flag, size]).u32();
        e.call(FILE_SET_NAME, &args![file, name]);
        e.call(FILE_008463C0, &args![file]);
        file
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0085_0030, fn_00850030(Ptr, u32, bool, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn opens_with_the_size_for_its_kind() {
        let mut e = Engine::new();
        e.register(MAKE_PATH, |_, _| Ret::default());
        e.register(ACCESS, |_, _| 0u32.into_ret());
        e.register(DELETE_FILE_A, |_, _| 1u32.into_ret());
        e.register(OPEN_FILE, |_, _| 0x6000u32.into_ret());
        e.register(FILE_SET_NAME, |_, _| Ret::default());
        e.register(FILE_008463C0, |_, _| Ret::default());
        for (write, kind, size, deletes) in [
            (true, 1u32, 0x20_0000u32, 1usize),
            (false, 0, 0x10000, 0),
            (true, 7, 0x10000, 1),
        ] {
            e.call_log = Some(vec![]);
            let file = e.call(0x0085_0030, &args![0x100u32, 0x200u32, write, kind, 9u32]);
            assert_eq!(file.u32(), 0x6000);
            let path = calls_to(&e, MAKE_PATH)[0][2];
            assert_eq!(
                calls_to(&e, MAKE_PATH)[0],
                vec![0x100, 0x200, path, u32::from(write)]
            );
            assert_eq!(calls_to(&e, DELETE_FILE_A).len(), deletes);
            assert_eq!(
                calls_to(&e, OPEN_FILE),
                vec![vec![path, u32::from(write), size]]
            );
            assert_eq!(calls_to(&e, FILE_SET_NAME), vec![vec![0x6000, 0x200]]);
            assert_eq!(calls_to(&e, FILE_008463C0), vec![vec![0x6000]]);
        }
        // No file there: nothing deleted.
        e.register(ACCESS, |_, _| 0xffff_ffffu32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0085_0030, &args![0x100u32, 0x200u32, true, 0u32, 0u32]);
        assert!(calls_to(&e, DELETE_FILE_A).is_empty());
    }
}
