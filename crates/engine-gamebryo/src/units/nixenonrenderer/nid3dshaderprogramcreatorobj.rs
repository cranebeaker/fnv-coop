//! `NiXenonRenderer/nid3dshaderprogramcreatorobj.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiShaderFactory::ReportError` (`00aa0020`, engine map; `cdecl`: 1, the recoverable flag,
/// the format, the format's words).
const REPORT_ERROR: u32 = 0x00aa_0020;
/// `NiD3DShaderProgramFactory::ResolveShaderFileName` (`00e930a0`, engine map; `cdecl`: file,
/// path buffer, size; answers in `AL`).
const RESOLVE_SHADER_FILE_NAME: u32 = 0x00e9_30a0;
/// `00aa1070` (`cdecl`, the size) and `00aa10f0` (`cdecl`, the block): allocate and free a
/// block; `memset` (`00ec61c0`) and `_memcpy` (`00ec44d0`), `cdecl`.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
/// Import slots (`stdcall`): `CreateFileA` (`00fdf088`), `GetFileSize` (`00fdf084`),
/// `ReadFile` (`00fdf080`) and `CloseHandle` (`00fdf07c`), by the import table's names.
const IMPORT_CREATE_FILE: u32 = 0x00fd_f088;
const IMPORT_GET_FILE_SIZE: u32 = 0x00fd_f084;
const IMPORT_READ_FILE: u32 = 0x00fd_f080;
const IMPORT_CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// Formats: `"Invalid shader file name"` (`010f12a8`), `"Failed to find shader program file
/// %s"` (`010f1338`), `"Invalid shader file %s"` (`010f1628`) and `"Invalid shader buffer"`
/// (`010f1360`).
const INVALID_SHADER_FILE_NAME: u32 = 0x010f_12a8;
const FAILED_FIND_FILE: u32 = 0x010f_1338;
const INVALID_SHADER_FILE: u32 = 0x010f_1628;
const INVALID_SHADER_BUFFER: u32 = 0x010f_1360;

// Translated from 00e981f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e981f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramCreatorObj::LoadShaderCodeFromFile` (Xbox PDB, engine map), `thiscall`
/// (`this` not read), six words: the file, the code and size outs, two more outs (typed
/// `void*&` and `unsigned int&` in the Xbox PDB) and the recoverable flag. Clears the four
/// outs; reports `"Invalid shader file name"` for a null or empty file and `"Failed to find
/// shader program file %s"` when the factory cannot resolve it (into a 0x104-byte local),
/// answering 0. Opens the path (`CreateFileA`: read access 0x80000000, share 1, no security,
/// open existing 3, no flags, no template); a failure reports `"Invalid shader file %s"` with
/// the path and answers 0. Otherwise reads the whole file (`GetFileSize` without a high word)
/// into a block of its size plus 4 (`00aa1070`, zeroed with `memset`; a null block answers 0
/// with the file left open), closes the file and, when the bytes read equal the size, gives
/// the size and the block to the outs and answers 1; a short read frees the block and answers
/// 0.
pub fn ni_d3d_shader_program_creator_obj_load_shader_code_from_file(
    e: &mut Engine,
    _this: Ptr,
    file: u32,
    code_out: u32,
    size_out: u32,
    other_pointer_out: u32,
    other_size_out: u32,
    recoverable: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    e.mem.set_u32(other_pointer_out, 0);
    e.mem.set_u32(other_size_out, 0);
    if file == 0 || e.mem.u8(file) == 0 {
        e.call(REPORT_ERROR, &[1, recoverable, INVALID_SHADER_FILE_NAME]);
        return false;
    }
    // The locals: the bytes read (4) and the path (0x104).
    e.with_stack(0x108, |e, locals| {
        let read = locals.addr();
        let path = locals.addr() + 4;
        if !e
            .call(RESOLVE_SHADER_FILE_NAME, &[file, path, 0x104])
            .bool()
        {
            e.call(REPORT_ERROR, &[1, recoverable, FAILED_FIND_FILE, file]);
            return false;
        }
        let handle = e
            .call(IMPORT_CREATE_FILE, &[path, 0x8000_0000, 1, 0, 3, 0, 0])
            .u32();
        if handle == 0xffff_ffff {
            e.call(REPORT_ERROR, &[1, recoverable, INVALID_SHADER_FILE, path]);
            return false;
        }
        let size = e.call(IMPORT_GET_FILE_SIZE, &[handle, 0]).u32();
        let bytes = size.wrapping_add(4);
        let block = e.call(ALLOCATE_BLOCK, &[bytes]).u32();
        if block == 0 {
            return false;
        }
        e.call(MEMSET, &[block, 0, bytes]);
        e.call(IMPORT_READ_FILE, &[handle, block, size, read, 0]);
        e.call(IMPORT_CLOSE_HANDLE, &[handle]);
        if e.mem.u32(read) != size {
            e.call(FREE_BLOCK, &[block]);
            return false;
        }
        e.mem.set_u32(size_out, size);
        e.mem.set_u32(code_out, block);
        true
    })
}

// Translated from 00e98370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorObj::LoadShaderCodeFromBuffer` (Xbox PDB, engine map), `thiscall`
/// (`this` not read), seven words: the source, its size, the code and size outs, two more outs
/// and the recoverable flag. Clears the four outs; a null source or a zero size reports
/// `"Invalid shader buffer"` and answers 0. Otherwise copies the source into a new block of
/// its size (`00aa1070`, `_memcpy`), gives the block and the size to the outs and answers 1.
pub fn ni_d3d_shader_program_creator_obj_load_shader_code_from_buffer(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
    size: u32,
    code_out: u32,
    size_out: u32,
    other_pointer_out: u32,
    other_size_out: u32,
    recoverable: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    e.mem.set_u32(other_pointer_out, 0);
    e.mem.set_u32(other_size_out, 0);
    if source == 0 || size == 0 {
        e.call(REPORT_ERROR, &[1, recoverable, INVALID_SHADER_BUFFER]);
        return false;
    }
    let block = e.call(ALLOCATE_BLOCK, &[size]).u32();
    e.mem.set_u32(code_out, block);
    e.call(MEMCPY, &[block, source, size]);
    e.mem.set_u32(size_out, size);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_81f0,
            ni_d3d_shader_program_creator_obj_load_shader_code_from_file(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(
            0x00e9_8370,
            ni_d3d_shader_program_creator_obj_load_shader_code_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HANDLE: u32 = 0x0000_0444;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles: the file resolves, opens, has 6 bytes and reads `read` of them.
    fn file_engine(read: u32) -> Engine {
        let mut e = Engine::new();
        e.register(REPORT_ERROR, |_, _| Ret::default());
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 1u32.into_ret());
        e.register(IMPORT_CREATE_FILE, |_, _| HANDLE.into_ret());
        e.register(IMPORT_GET_FILE_SIZE, |_, _| 6u32.into_ret());
        e.register_double(IMPORT_READ_FILE, move |e, a| {
            e.mem.write(a[1], b"OBJECT");
            e.mem.set_u32(a[3], read);
            1u32.into_ret()
        });
        e.register(IMPORT_CLOSE_HANDLE, |_, _| 1u32.into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn outs(e: &mut Engine) -> u32 {
        let outs = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(outs + 4 * i, 0x99);
        }
        outs
    }

    #[test]
    fn load_from_file_reads_the_whole_file() {
        let mut e = file_engine(6);
        let file = e.mem.alloc(8);
        e.mem.set_cstr(file, b"a.obj");
        let o = outs(&mut e);
        let words = [0, file, o, o + 4, o + 8, o + 12, 1];
        assert!(e.call(0x00e9_81f0, &words).bool());
        assert_eq!(e.mem.u32(o + 4), 6);
        assert_eq!(e.mem.bytes(e.mem.u32(o), 6), b"OBJECT");
        assert_eq!([e.mem.u32(o + 8), e.mem.u32(o + 12)], [0, 0]);
        let open = &calls(&e, IMPORT_CREATE_FILE)[0];
        assert_eq!(open[1..], [0x8000_0000, 1, 0, 3, 0, 0]);
        assert_eq!(calls(&e, ALLOCATE_BLOCK), vec![vec![10]]);
        assert_eq!(calls(&e, IMPORT_CLOSE_HANDLE), vec![vec![HANDLE]]);
    }

    #[test]
    fn load_from_file_failures() {
        let mut e = file_engine(5);
        let file = e.mem.alloc(8);
        e.mem.set_cstr(file, b"a.obj");
        let o = outs(&mut e);
        // A short read frees the block.
        assert!(!e
            .call(0x00e9_81f0, &[0, file, o, o + 4, o + 8, o + 12, 1])
            .bool());
        assert_eq!(calls(&e, FREE_BLOCK).len(), 1);
        assert_eq!(e.mem.u32(o), 0);
        // The file does not open.
        e.register(IMPORT_CREATE_FILE, |_, _| 0xffff_ffffu32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_81f0, &[0, file, o, o + 4, o + 8, o + 12, 1])
            .bool());
        let report = &calls(&e, REPORT_ERROR)[0];
        assert_eq!(report[..3], [1, 1, INVALID_SHADER_FILE]);
        assert_eq!(report[3], calls(&e, IMPORT_CREATE_FILE)[0][0]);
        // Not found, and no file name.
        e.register(RESOLVE_SHADER_FILE_NAME, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_81f0, &[0, file, o, o + 4, o + 8, o + 12, 0])
            .bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, FAILED_FIND_FILE, file]]
        );
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_81f0, &[0, 0, o, o + 4, o + 8, o + 12, 0])
            .bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 0, INVALID_SHADER_FILE_NAME]]
        );
        assert!(calls(&e, RESOLVE_SHADER_FILE_NAME).is_empty());
    }

    #[test]
    fn load_from_buffer_copies_the_source() {
        let mut e = file_engine(0);
        let source = e.mem.alloc(8);
        e.mem.write(source, b"BYTES");
        let o = outs(&mut e);
        assert!(e
            .call(0x00e9_8370, &[0, source, 5, o, o + 4, o + 8, o + 12, 1])
            .bool());
        assert_eq!(e.mem.bytes(e.mem.u32(o), 5), b"BYTES");
        assert_eq!(e.mem.u32(o + 4), 5);
        assert_eq!([e.mem.u32(o + 8), e.mem.u32(o + 12)], [0, 0]);
        // No size: reported, the outs cleared.
        e.mem.set_u32(o + 8, 0x99);
        assert!(!e
            .call(0x00e9_8370, &[0, source, 0, o, o + 4, o + 8, o + 12, 1])
            .bool());
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![vec![1, 1, INVALID_SHADER_BUFFER]]
        );
        assert_eq!([e.mem.u32(o), e.mem.u32(o + 8)], [0, 0]);
    }
}
