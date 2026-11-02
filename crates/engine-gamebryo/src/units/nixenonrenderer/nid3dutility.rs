//! `NiXenonRenderer/nid3dutility.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFilename::Splitpath` (`00ec7f1e`, engine map; `cdecl`, nine words: path, then drive,
/// directory, name and extension buffers each with its size).
const SPLIT_PATH: u32 = 0x00ec_7f1e;
/// `_strncpy_s` (`00ec8d5f`, engine map; `cdecl`: destination, size, source, count).
const STRNCPY_S: u32 = 0x00ec_8d5f;

// Translated from 00e98430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e98430 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DUtility::GetFileExtension` (Xbox PDB, engine map), `cdecl`, three words (path,
/// buffer, buffer size): splits the path (drive 3 bytes, directory 0x104, name 0x100,
/// extension 0x100, all locals) and copies the extension into the buffer (`_strncpy_s` with
/// the extension's length). Answers 1.
pub fn ni_d3d_utility_get_file_extension(
    e: &mut Engine,
    path: u32,
    buffer: u32,
    size: u32,
) -> bool {
    // The locals: drive (4 bytes), extension (0x100), name (0x100), directory (0x104).
    e.with_stack(0x308, |e, locals| {
        let drive = locals.addr();
        let extension = drive + 4;
        let name = drive + 0x104;
        let directory = drive + 0x204;
        e.call(
            SPLIT_PATH,
            &[
                path, drive, 3, directory, 0x104, name, 0x100, extension, 0x100,
            ],
        );
        let length = e.mem.cstr(extension).len() as u32;
        e.call(STRNCPY_S, &[buffer, size, extension, length]);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e9_8430,
        ni_d3d_utility_get_file_extension(u32, u32, u32) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_extension_copies_the_split_extension() {
        let mut e = Engine::new();
        e.register(SPLIT_PATH, |e, a| {
            e.mem.set_cstr(a[7], b".hlsl");
            Ret::default()
        });
        e.register(STRNCPY_S, |e, a| {
            let text = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &text);
            e.mem.set_u8(a[0] + a[3], 0);
            Ret::default()
        });
        let path = e.mem.alloc(16);
        let buffer = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_8430, &[path, buffer, 0x100]).bool());
        assert_eq!(e.mem.cstr(buffer), b".hlsl");
        let log = e.call_log.take().unwrap();
        let split = &log.iter().find(|c| c.0 == SPLIT_PATH).unwrap().1;
        assert_eq!(split[0], path);
        assert_eq!(
            [split[2], split[4], split[6], split[8]],
            [3, 0x104, 0x100, 0x100]
        );
        let copy = &log.iter().find(|c| c.0 == STRNCPY_S).unwrap().1;
        assert_eq!(
            [copy[0], copy[1], copy[2], copy[3]],
            [buffer, 0x100, split[7], 5]
        );
    }
}
