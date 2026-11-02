//! `NiXenonRenderer/nid3dshaderprogramcreator_xenon.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The program creators' Direct3D device (`0126fa28`, kept by `00e93f50`), the report
/// `NiShaderFactory::ReportError` (`00aa0020`, `cdecl`: severity, error code, format and its
/// arguments) and its two formats.
const DEVICE_0126FA28: u32 = 0x0126_fa28;
const REPORT_ERROR_00AA0020: u32 = 0x00aa_0020;
const VERTEX_FAILED_010F166C: u32 = 0x010f_166c;
const PIXEL_FAILED_010F169C: u32 = 0x010f_169c;

/// The device's create call at `slot` (`stdcall`: the code and the address of the word the
/// shader is written to). The code passes the address of its own first argument word, which
/// holds the code until the device overwrites it; the word is answered when the call answers a
/// non-negative result. A negative result is reported (`00aa0020` with 1, 0, the format and the
/// result) and 0 answered.
fn create_shader(e: &mut Engine, slot: u32, code: u32, failed: u32) -> u32 {
    let device = e.mem.u32(DEVICE_0126FA28);
    e.with_stack(4, |e, word| {
        let word = word.addr();
        e.mem.set_u32(word, code);
        let result = e.vcall(device, slot, &args![code, word]).i32();
        if result < 0 {
            e.call(
                REPORT_ERROR_00AA0020,
                &args![1u32, 0u32, failed, result as u32],
            );
            return 0;
        }
        e.mem.u32(word)
    })
}

// Translated from 00e98890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreator::CreateVertexShader` (engine map), `thiscall` (`this` not read)
/// with six words, only the first (the shader code) read: the vertex shader the device's slot
/// 0x16c makes from it, or 0 with the failure reported ([`create_shader`]).
pub fn ni_d3d_shader_program_creator_create_vertex_shader(
    e: &mut Engine,
    _this: Ptr,
    code: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    create_shader(e, 0x16c, code, VERTEX_FAILED_010F166C)
}

// Translated from 00e988d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreator::CreatePixelShader` (engine map), `thiscall` (`this` not read)
/// with the shader code: the pixel shader the device's slot 0x1a8 makes from it, or 0 with the
/// failure reported ([`create_shader`]).
pub fn ni_d3d_shader_program_creator_create_pixel_shader(
    e: &mut Engine,
    _this: Ptr,
    code: u32,
) -> u32 {
    create_shader(e, 0x1a8, code, PIXEL_FAILED_010F169C)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_8890,
            ni_d3d_shader_program_creator_create_vertex_shader(
                Ptr, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_88d0,
            ni_d3d_shader_program_creator_create_pixel_shader(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e00_0000;
    const CREATE_VERTEX: u32 = 0x0e01_016c;
    const CREATE_PIXEL: u32 = 0x0e01_01a8;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A device whose create calls write `shader` and answer `result`.
    fn rig(shader: u32, result: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let mut slots = vec![0u32; 0x6b];
        slots[0x16c / 4] = CREATE_VERTEX;
        slots[0x1a8 / 4] = CREATE_PIXEL;
        e.put_vtable(DEVICE_VTABLE, &slots);
        for slot in [CREATE_VERTEX, CREATE_PIXEL] {
            e.register_double(slot, move |e, a| {
                // The out word holds the code until the device writes it.
                assert_eq!(e.mem.u32(a[2]), a[1]);
                e.mem.set_u32(a[2], shader);
                result.into_ret()
            });
        }
        e.register(REPORT_ERROR_00AA0020, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(DEVICE_0126FA28, device);
        e.call_log = Some(vec![]);
        (e, device)
    }

    #[test]
    fn create_vertex_shader_answers_the_shader_or_reports() {
        let (mut e, device) = rig(0x0e50_0000, 0);
        let call = args![0u32, 0x1000u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        assert_eq!(e.call(0x00e9_8890, &call).u32(), 0x0e50_0000);
        assert_eq!(calls(&e, CREATE_VERTEX)[0][..2], [device, 0x1000]);
        let (mut e, _) = rig(0x0e50_0000, 0x8000_4005);
        assert_eq!(e.call(0x00e9_8890, &call).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, VERTEX_FAILED_010F166C, 0x8000_4005]]
        );
    }

    #[test]
    fn create_pixel_shader_answers_the_shader_or_reports() {
        let (mut e, device) = rig(0x0e51_0000, 1);
        assert_eq!(
            e.call(0x00e9_88d0, &args![0u32, 0x2000u32]).u32(),
            0x0e51_0000
        );
        assert_eq!(calls(&e, CREATE_PIXEL)[0][..2], [device, 0x2000]);
        let (mut e, _) = rig(0x0e51_0000, 0x8876_086c);
        assert_eq!(e.call(0x00e9_88d0, &args![0u32, 0x2000u32]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, PIXEL_FAILED_010F169C, 0x8876_086c]]
        );
    }
}
