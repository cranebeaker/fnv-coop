//! `NiXenonRenderer/nid3dshaderprogramcreatorasm_xenon.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both loaders assemble shader code with D3DX and copy it into a block of their own. The
//! D3DX buffers they get back are COM objects: slot 8 releases one, 0xc answers its data and
//! 0x10 its size (all `stdcall`).

#[allow(unused_imports)]
use crate::prelude::*;

/// What both loaders call: `NiShaderFactory::ReportError` (`00aa0020`, `cdecl`: severity, error
/// code, format and its arguments), the shader creation flags every creator adds (`0126fa24`),
/// the allocation `00aa1070` and free `00aa10f0` (`cdecl`, one word), the copy `00404d80`
/// (`cdecl`: destination, size, source) and `memcpy` (`00ec44d0`).
const REPORT_ERROR_00AA0020: u32 = 0x00aa_0020;
const CREATION_FLAGS_0126FA24: u32 = 0x0126_fa24;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const COPY_00404D80: u32 = 0x0040_4d80;
const MEMCPY: u32 = 0x00ec_44d0;

/// The flags both loaders hand to D3DX: the creator's `m_uiShaderCreationFlags` (+4, Xbox PDB)
/// with the global ones, the low two bits.
fn assembly_flags(e: &Engine, creator: u32) -> u32 {
    (e.mem.u32(creator.wrapping_add(4)) | e.mem.u32(CREATION_FLAGS_0126FA24)) & 3
}

/// A D3DX buffer's data copied into a new block: its size (slot 0x10) into `size_out`, a block
/// of that size (`00aa1070`) into `code_out`, then the buffer's data (slot 0xc) copied in
/// (`memcpy` of the size read back from `size_out`).
fn copy_code(e: &mut Engine, buffer_slot: u32, code_out: u32, size_out: u32) {
    let buffer = e.mem.u32(buffer_slot);
    let size = e.vcall(buffer, 0x10, &args![]).u32();
    e.mem.set_u32(size_out, size);
    let block = e.call(ALLOCATE_00AA1070, &args![size]).u32();
    e.mem.set_u32(code_out, block);
    let buffer = e.mem.u32(buffer_slot);
    let size = e.mem.u32(size_out);
    let data = e.vcall(buffer, 0xc, &args![]).u32();
    let destination = e.mem.u32(code_out);
    e.call(MEMCPY, &args![destination, data, size]);
}

/// A D3DX error buffer's text copied into a new block (when the buffer and its data are set:
/// `00aa1070` of its size, `00404d80`) and reported with `with_text` (`00aa0020` with 1, the
/// error code, the format, `before` words and the copy); the buffer is released. Without a
/// buffer `without` is reported (`before` words only). The block (or 0) is freed (`00aa10f0`).
fn report_errors(
    e: &mut Engine,
    errors_slot: u32,
    error: u32,
    with_text: u32,
    without: u32,
    before: &[u32],
) {
    let errors = e.mem.u32(errors_slot);
    let mut text = 0;
    if errors != 0 {
        let data = e.vcall(errors, 0xc, &args![]).u32();
        if data != 0 {
            let errors = e.mem.u32(errors_slot);
            let size = e.vcall(errors, 0x10, &args![]).u32();
            text = e.call(ALLOCATE_00AA1070, &args![size]).u32();
            e.call(COPY_00404D80, &args![text, size, data]);
            let mut words = vec![1u32, error, with_text];
            words.extend_from_slice(before);
            words.push(text);
            e.call(REPORT_ERROR_00AA0020, &words);
        }
        let errors = e.mem.u32(errors_slot);
        e.vcall(errors, 8, &args![]);
    } else {
        let mut words = vec![1u32, error, without];
        words.extend_from_slice(before);
        e.call(REPORT_ERROR_00AA0020, &words);
    }
    e.call(FREE_00AA10F0, &args![text]);
}

/// `NiD3DShaderProgramFactory::ResolveShaderFileName` (`00e930a0`, engine map; `cdecl`: the
/// name, a buffer and its size; answers a byte), `D3DXAssembleShaderFromFileA` (`00ee6e4c`,
/// `stdcall`: the file, defines, include, flags, the shader and error buffer out words) and the
/// four formats `LoadShaderCodeFromFile` reports with.
const RESOLVE_FILE_NAME_00E930A0: u32 = 0x00e9_30a0;
const ASSEMBLE_FROM_FILE_00EE6E4C: u32 = 0x00ee_6e4c;
const INVALID_FILE_NAME_010F12A8: u32 = 0x010f_12a8;
const FILE_NOT_FOUND_010F1338: u32 = 0x010f_1338;
const FILE_FAILED_010F1590: u32 = 0x010f_1590;
const FILE_FAILED_NONE_010F155C: u32 = 0x010f_155c;

// Translated from 00e97ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::LoadShaderCodeFromFile` (engine map), `thiscall` with six
/// words: the file name, the addresses of the code, the code size and two more words (all four
/// cleared first) and the error code for the reports. Without a name (or an empty one) reports
/// `"Invalid shader file name"` (`00aa0020` with 1 and 0) and answers false. The name is
/// resolved into a 0x104-byte stack buffer (`00e930a0`; failing: reported with the error code
/// and the name, false) and assembled (`D3DXAssembleShaderFromFileA` with the path, no defines
/// or include, the flags [`assembly_flags`] and the addresses of two stack words for the code
/// and error buffers). A negative result reports the errors ([`report_errors`] with the name)
/// and releases the code buffer when set: false. Else the code is copied ([`copy_code`]), both
/// buffers released (the error buffer when set): true.
// Translation incomplete: 00e97ea0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_d3d_shader_program_creator_asm_load_shader_code_from_file(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    code_out: u32,
    size_out: u32,
    third_out: u32,
    fourth_out: u32,
    error: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    e.mem.set_u32(third_out, 0);
    e.mem.set_u32(fourth_out, 0);
    if file == 0 || e.mem.u8(file) == 0 {
        e.call(
            REPORT_ERROR_00AA0020,
            &args![1u32, 0u32, INVALID_FILE_NAME_010F12A8],
        );
        return false;
    }
    e.with_stack(0x10c, |e, frame| {
        let errors_slot = frame.addr();
        let code_slot = errors_slot + 4;
        let path = errors_slot + 8;
        let resolved = e
            .call(RESOLVE_FILE_NAME_00E930A0, &args![file, path, 0x104u32])
            .bool();
        if !resolved {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, error, FILE_NOT_FOUND_010F1338, file],
            );
            return false;
        }
        let flags = assembly_flags(e, this.addr());
        let result = e
            .call(
                ASSEMBLE_FROM_FILE_00EE6E4C,
                &args![path, 0u32, 0u32, flags, code_slot, errors_slot],
            )
            .i32();
        if result < 0 {
            report_errors(
                e,
                errors_slot,
                error,
                FILE_FAILED_010F1590,
                FILE_FAILED_NONE_010F155C,
                &[file],
            );
            let code = e.mem.u32(code_slot);
            if code != 0 {
                e.vcall(code, 8, &args![]);
            }
            return false;
        }
        copy_code(e, code_slot, code_out, size_out);
        let code = e.mem.u32(code_slot);
        e.vcall(code, 8, &args![]);
        let errors = e.mem.u32(errors_slot);
        if errors != 0 {
            e.vcall(errors, 8, &args![]);
        }
        true
    })
}

/// `D3DXAssembleShader` (`00ee6e52`, `stdcall`: the source, its length, defines, include, flags,
/// the shader and error buffer out words) and the three formats `LoadShaderCodeFromBuffer`
/// reports with.
const ASSEMBLE_00EE6E52: u32 = 0x00ee_6e52;
const INVALID_BUFFER_010F1360: u32 = 0x010f_1360;
const BUFFER_FAILED_010F15F4: u32 = 0x010f_15f4;
const BUFFER_FAILED_NONE_010F15B8: u32 = 0x010f_15b8;

// Translated from 00e98080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorAsm::LoadShaderCodeFromBuffer` (engine map), `thiscall` with seven
/// words: the source, its length, the addresses of the code, the code size and two more words
/// (all four cleared first) and the error code. Without a source or a length reports the
/// invalid buffer (`00aa0020` with 1 and the error code) and answers false. Else the source is
/// assembled (`D3DXAssembleShader` with no defines or include, the flags [`assembly_flags`] and,
/// as the shader and error buffer out words, the argument words that held the code and size
/// addresses). A negative result reports the errors ([`report_errors`]) and releases the code
/// buffer when set: false. Else the code is copied ([`copy_code`]) through the addresses as
/// passed, both buffers released (the error buffer when set): true.
pub fn ni_d3d_shader_program_creator_asm_load_shader_code_from_buffer(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    length: u32,
    code_out: u32,
    size_out: u32,
    third_out: u32,
    fourth_out: u32,
    error: u32,
) -> bool {
    e.mem.set_u32(code_out, 0);
    e.mem.set_u32(size_out, 0);
    e.mem.set_u32(third_out, 0);
    e.mem.set_u32(fourth_out, 0);
    if source == 0 || length == 0 {
        e.call(
            REPORT_ERROR_00AA0020,
            &args![1u32, error, INVALID_BUFFER_010F1360],
        );
        return false;
    }
    // The two argument words the assembler writes its buffers to.
    e.with_stack(8, |e, words| {
        let code_slot = words.addr();
        let errors_slot = code_slot + 4;
        e.mem.set_u32(code_slot, code_out);
        e.mem.set_u32(errors_slot, size_out);
        let flags = assembly_flags(e, this.addr());
        let result = e
            .call(
                ASSEMBLE_00EE6E52,
                &args![source, length, 0u32, 0u32, flags, code_slot, errors_slot],
            )
            .i32();
        if result < 0 {
            report_errors(
                e,
                errors_slot,
                error,
                BUFFER_FAILED_010F15F4,
                BUFFER_FAILED_NONE_010F15B8,
                &[],
            );
            let code = e.mem.u32(code_slot);
            if code != 0 {
                e.vcall(code, 8, &args![]);
            }
            return false;
        }
        copy_code(e, code_slot, code_out, size_out);
        let code = e.mem.u32(code_slot);
        e.vcall(code, 8, &args![]);
        let errors = e.mem.u32(errors_slot);
        if errors != 0 {
            e.vcall(errors, 8, &args![]);
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_7ea0,
            ni_d3d_shader_program_creator_asm_load_shader_code_from_file(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(
            0x00e9_8080,
            ni_d3d_shader_program_creator_asm_load_shader_code_from_buffer(
                Ptr, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    const BUFFER_VTABLE: u32 = 0x0e00_0000;
    const BUFFER_RELEASE: u32 = 0x0e01_0008;
    const BUFFER_DATA: u32 = 0x0e01_000c;
    const BUFFER_SIZE: u32 = 0x0e01_0010;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Two D3DX buffers (code, errors) answering data 0x0e10_0000 and size 0x40; the creator
    /// with flags 6 (the global 1). Answers the engine, the creator, the code and error
    /// buffers and four out words.
    fn rig() -> (Engine, u32, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(CREATION_FLAGS_0126FA24, 1);
        e.put_vtable(
            BUFFER_VTABLE,
            &[0, 0, BUFFER_RELEASE, BUFFER_DATA, BUFFER_SIZE],
        );
        e.register(BUFFER_RELEASE, |_, _| Ret::default());
        e.register(BUFFER_DATA, |_, _| 0x0e10_0000u32.into_ret());
        e.register(BUFFER_SIZE, |_, _| 0x40u32.into_ret());
        e.register(ALLOCATE_00AA1070, |_, _| 0x0e20_0000u32.into_ret());
        for callee in [FREE_00AA10F0, COPY_00404D80, MEMCPY, REPORT_ERROR_00AA0020] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(RESOLVE_FILE_NAME_00E930A0, |_, _| 1u32.into_ret());
        let creator = e.mem.alloc(0x10);
        e.mem.set_u32(creator + 4, 6);
        let code = e.mem.alloc(4);
        e.mem.set_u32(code, BUFFER_VTABLE);
        let errors = e.mem.alloc(4);
        e.mem.set_u32(errors, BUFFER_VTABLE);
        let outs = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(outs + 4 * i, 0x55);
        }
        e.call_log = Some(vec![]);
        (e, creator, code, errors, outs)
    }

    /// The assembler double: writes `code` and `errors` to its last two words, answers
    /// `result`.
    fn assembler(e: &mut Engine, addr: u32, code: u32, errors: u32, result: u32) {
        e.register_double(addr, move |e, a| {
            let n = a.len();
            e.mem.set_u32(a[n - 2], code);
            e.mem.set_u32(a[n - 1], errors);
            result.into_ret()
        });
    }

    #[test]
    fn load_from_file_copies_the_assembled_code() {
        let (mut e, creator, code, errors, outs) = rig();
        assembler(&mut e, ASSEMBLE_FROM_FILE_00EE6E4C, code, errors, 0);
        e.mem.set_u8(0x0126_f800, b'a');
        let call = args![
            creator,
            0x0126_f800u32,
            outs,
            outs + 4,
            outs + 8,
            outs + 12,
            7u32
        ];
        assert!(e.call(0x00e9_7ea0, &call).bool());
        assert_eq!(e.mem.u32(outs), 0x0e20_0000);
        assert_eq!(e.mem.u32(outs + 4), 0x40);
        assert_eq!(e.mem.u32(outs + 8), 0);
        assert_eq!(e.mem.u32(outs + 12), 0);
        let assembled = &calls(&e, ASSEMBLE_FROM_FILE_00EE6E4C)[0];
        let path = calls(&e, RESOLVE_FILE_NAME_00E930A0)[0][1];
        assert_eq!(assembled[..4], [path, 0, 0, 3]);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![vec![0x0e20_0000, 0x0e10_0000, 0x40]]
        );
        assert_eq!(calls(&e, BUFFER_RELEASE), vec![vec![code], vec![errors]]);
    }

    #[test]
    fn load_from_file_reports_bad_names_and_errors() {
        // No name, an empty name: reported, false.
        let (mut e, creator, code, errors, outs) = rig();
        let call = args![creator, 0u32, outs, outs + 4, outs + 8, outs + 12, 7u32];
        assert!(!e.call(0x00e9_7ea0, &call).bool());
        assert_eq!(e.mem.u32(outs + 8), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, INVALID_FILE_NAME_010F12A8]]
        );
        // Not found: reported with the error code and the name.
        e.register(RESOLVE_FILE_NAME_00E930A0, |_, _| 0u32.into_ret());
        e.mem.set_u8(0x0126_f800, b'a');
        e.call_log = Some(vec![]);
        let call = args![
            creator,
            0x0126_f800u32,
            outs,
            outs + 4,
            outs + 8,
            outs + 12,
            7u32
        ];
        assert!(!e.call(0x00e9_7ea0, &call).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 7, FILE_NOT_FOUND_010F1338, 0x0126_f800]]
        );
        // Failing with an error buffer: its text copied and reported, everything let go.
        e.register(RESOLVE_FILE_NAME_00E930A0, |_, _| 1u32.into_ret());
        assembler(
            &mut e,
            ASSEMBLE_FROM_FILE_00EE6E4C,
            code,
            errors,
            0x8000_0000,
        );
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_7ea0, &call).bool());
        assert_eq!(
            calls(&e, COPY_00404D80),
            vec![vec![0x0e20_0000, 0x40, 0x0e10_0000]]
        );
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 7, FILE_FAILED_010F1590, 0x0126_f800, 0x0e20_0000]]
        );
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e20_0000]]);
        assert_eq!(calls(&e, BUFFER_RELEASE), vec![vec![errors], vec![code]]);
        // Failing without buffers.
        assembler(&mut e, ASSEMBLE_FROM_FILE_00EE6E4C, 0, 0, 0x8000_0000);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_7ea0, &call).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 7, FILE_FAILED_NONE_010F155C, 0x0126_f800]]
        );
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0]]);
        assert!(calls(&e, BUFFER_RELEASE).is_empty());
    }

    #[test]
    fn load_from_buffer_copies_or_reports() {
        let (mut e, creator, code, errors, outs) = rig();
        assembler(&mut e, ASSEMBLE_00EE6E52, code, 0, 0);
        let call = args![
            creator,
            0x0e30_0000u32,
            0x80u32,
            outs,
            outs + 4,
            outs + 8,
            outs + 12,
            7u32
        ];
        assert!(e.call(0x00e9_8080, &call).bool());
        assert_eq!(e.mem.u32(outs), 0x0e20_0000);
        assert_eq!(e.mem.u32(outs + 4), 0x40);
        assert_eq!(
            calls(&e, ASSEMBLE_00EE6E52)[0][..5],
            [0x0e30_0000, 0x80, 0, 0, 3]
        );
        assert_eq!(calls(&e, BUFFER_RELEASE), vec![vec![code]]);
        // Failing: the error text reported without a name.
        assembler(&mut e, ASSEMBLE_00EE6E52, 0, errors, 0x8000_0000);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_8080, &call).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 7, BUFFER_FAILED_010F15F4, 0x0e20_0000]]
        );
        assert_eq!(calls(&e, BUFFER_RELEASE), vec![vec![errors]]);
        // No source: the invalid buffer report.
        e.call_log = Some(vec![]);
        let mut call = call;
        call[2] = 0;
        assert!(!e.call(0x00e9_8080, &call).bool());
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 7, INVALID_BUFFER_010F1360]]
        );
        assert!(calls(&e, ASSEMBLE_00EE6E52).is_empty());
    }
}
