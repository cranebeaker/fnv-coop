//! `NiXenonRenderer/nid3dhlslvertexshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constant-table helpers are shared with `nid3dhlslpixelshader.rs`, whose functions have
//! the same code with the pixel shader's offsets and callees.

#[allow(unused_imports)]
use crate::prelude::*;

use super::nid3dhlslpixelshader::{
    hlsl_constant_kind, hlsl_resolve_entry, hlsl_set_shader_constant, hlsl_shader_device,
    hlsl_variable_count, hlsl_variable_name, HlslSide, MATRIX_TRANSPOSE_00EE6DD4,
};

/// `NiD3DVertexShader::DestroyRendererData` (`00e89ac0`) and `RecreateRendererData`
/// (`00e89ad0`), both `thiscall` (unit `nid3dvertexshader.cpp`), reached by jumps.
const VERTEX_DESTROY_RENDERER_DATA_00E89AC0: u32 = 0x00e8_9ac0;
const VERTEX_RECREATE_RENDERER_DATA_00E89AD0: u32 = 0x00e8_9ad0;

// Translated from 00e97210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::DestroyRendererData` (engine map), `thiscall`: the constant table at
/// +0x44, when set, is released (its slot 8, `stdcall`) and cleared; then
/// `NiD3DVertexShader::DestroyRendererData` (`00e89ac0`, a jump).
pub fn ni_d3d_hlsl_vertex_shader_destroy_renderer_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let table = e.mem.u32(base.wrapping_add(0x44));
    if table != 0 {
        e.vcall(table, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x44), 0);
    }
    e.call(VERTEX_DESTROY_RENDERER_DATA_00E89AC0, &args![this]);
}

// Translated from 00e97240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::RecreateRendererData` (engine map), `thiscall`: a jump to
/// `NiD3DVertexShader::RecreateRendererData` (`00e89ad0`); answers what it answers.
pub fn ni_d3d_hlsl_vertex_shader_recreate_renderer_data(e: &mut Engine, this: Ptr) -> u32 {
    e.call(VERTEX_RECREATE_RENDERER_DATA_00E89AD0, &args![this])
        .u32()
}

// Translated from 00e97250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::GetVariableCount` (engine map), `thiscall`: 0 without a constant
/// table (+0x44); else the count its description gives ([`hlsl_variable_count`]).
pub fn ni_d3d_hlsl_vertex_shader_get_variable_count(e: &mut Engine, this: Ptr) -> u32 {
    let table = e.mem.u32(this.addr().wrapping_add(0x44));
    if table == 0 {
        return 0;
    }
    hlsl_variable_count(e, table)
}

// Translated from 00e97280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::GetVariableName` (engine map), `thiscall` with the index: the name
/// [`hlsl_variable_name`] gives from the constant table at +0x44.
pub fn ni_d3d_hlsl_vertex_shader_get_variable_name(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let table = e.mem.u32(this.addr().wrapping_add(0x44));
    hlsl_variable_name(e, table, index)
}

/// The vertex shader's offsets and callees for [`hlsl_set_shader_constant`]: its render state
/// helpers `00e89ae0`, `00e89b40` and `00e89b10` (unit `nid3dvertexshader.cpp`) and the
/// device's slot 0x178.
const VERTEX_SIDE: HlslSide = HlslSide {
    table: 0x44,
    not_found: 0x010f_14a0,
    bool_setter: 0x00e8_9ae0,
    int_setter: 0x00e8_9b40,
    float_setter: 0x00e8_9b10,
    device_float_slot: 0x178,
};

// Translated from 00e972e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::SetShaderConstant` (engine map), `thiscall` with three words (the
/// constant map entry, the data and a count it does not read): [`hlsl_set_shader_constant`]
/// with the vertex shader's table (+0x44), report and setters (`00e89ae0`, `00e89b40`,
/// `00e89b10`) and the device's slot 0x178.
pub fn ni_d3d_hlsl_vertex_shader_set_shader_constant(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    _unused_3: u32,
) -> bool {
    hlsl_set_shader_constant(e, this.addr(), entry, data, &VERTEX_SIDE)
}

/// The report's format when `SetShaderConstantArray` does not find the name (`010f14e8`).
const ARRAY_NOT_FOUND_010F14E8: u32 = 0x010f_14e8;

// Translated from 00e974f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::SetShaderConstantArray` (engine map), `thiscall` with five words: the
/// constant map entry, the data, the element count, the registers per element and the 16-bit
/// index array (or 0). False without the constant table (+0x44) or the shader word at +0x20, or
/// unless the entry's kind is 8 or 9. A null data takes the entry's +0x30. An entry whose word
/// +0x10 is 0x21 to 0x24 hands its data over as it is; the others transpose each 4x4 matrix
/// into the stack first (`D3DXMatrixTranspose`). The entry is resolved
/// ([`hlsl_resolve_entry`]; false when its name is unknown). With an index array each element
/// (index times registers per element times 16 bytes on from the data) goes to the device's
/// slot 0x178 (`stdcall`: the register, from the entry's +0x1c stepping by the registers per
/// element, the data and the registers per element). Without one, an untransposed entry goes in
/// one call to the render state helper `00e89b10` (the render state at +0x28 with the
/// register, the data, the entry's register count +0x20 and 0; its answer), and a transposed
/// one element by element
/// as above, stepping through the data. A negative device result ends it with false; answers
/// true when every call succeeded (false for no elements).
pub fn ni_d3d_hlsl_vertex_shader_set_shader_constant_array(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    count: u32,
    registers: u32,
    indices: u32,
) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x44)) == 0 || e.mem.u32(base.wrapping_add(0x20)) == 0 {
        return false;
    }
    let kind = hlsl_constant_kind(e, entry);
    if kind != 8 && kind != 9 {
        return false;
    }
    let data = if data == 0 {
        e.mem.u32(entry.wrapping_add(0x30))
    } else {
        data
    };
    let untransposed = matches!(e.mem.u32(entry.wrapping_add(0x10)), 0x21..=0x24);
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        if !hlsl_resolve_entry(
            e,
            base,
            entry,
            frame,
            0x14,
            0x14,
            0x44,
            ARRAY_NOT_FOUND_010F14E8,
        ) {
            return false;
        }
        let matrix = frame + 0x20;
        if indices == 0 && untransposed {
            let registers = e.mem.u32(entry.wrapping_add(0x20));
            let register = e.mem.u32(entry.wrapping_add(0x1c));
            let state = e.mem.u32(base.wrapping_add(0x28));
            return e
                .call(
                    VERTEX_SIDE.float_setter,
                    &args![state, register, data, registers, 0u32],
                )
                .bool();
        }
        let mut register = e.mem.u32(entry.wrapping_add(0x1c));
        let mut source = data;
        for element in 0..count {
            let mut handed = matrix;
            if indices != 0 {
                let index = u32::from(e.mem.u16(indices.wrapping_add(element.wrapping_mul(2))));
                let element_data = (index.wrapping_mul(registers) << 4).wrapping_add(data);
                if untransposed {
                    handed = element_data;
                } else {
                    e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, element_data]);
                }
            } else {
                e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, source]);
            }
            let device = hlsl_shader_device(e, base);
            let result = e.vcall(device, 0x178, &args![register, handed, registers]);
            if result.i32() < 0 {
                return false;
            }
            register = register.wrapping_add(registers);
            if indices == 0 {
                source = source.wrapping_add(registers << 4);
            }
        }
        count != 0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_7210,
            ni_d3d_hlsl_vertex_shader_destroy_renderer_data(Ptr)
        ),
        entry!(
            0x00e9_7240,
            ni_d3d_hlsl_vertex_shader_recreate_renderer_data(Ptr) -> u32
        ),
        entry!(
            0x00e9_7250,
            ni_d3d_hlsl_vertex_shader_get_variable_count(Ptr) -> u32
        ),
        entry!(
            0x00e9_7280,
            ni_d3d_hlsl_vertex_shader_get_variable_name(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_72e0,
            ni_d3d_hlsl_vertex_shader_set_shader_constant(Ptr, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e9_74f0,
            ni_d3d_hlsl_vertex_shader_set_shader_constant_array(Ptr, u32, u32, u32, u32, u32)
                -> bool
        ),
    ]
}

#[cfg(test)]
mod n008_tests {
    use super::super::nid3dhlslpixelshader::hlsl_test_support::*;
    use super::*;

    #[test]
    fn destroy_renderer_data_releases_the_table_then_runs_the_base() {
        let (mut e, shader, _) = rig(0x44, 0);
        e.register(VERTEX_DESTROY_RENDERER_DATA_00E89AC0, |_, _| Ret::default());
        let table = e.mem.u32(shader + 0x44);
        e.call(0x00e9_7210, &args![shader]);
        assert_eq!(calls(&e, TABLE_RELEASE), vec![vec![table]]);
        assert_eq!(e.mem.u32(shader + 0x44), 0);
        e.call(0x00e9_7210, &args![shader]);
        assert_eq!(calls(&e, TABLE_RELEASE).len(), 1);
        assert_eq!(calls(&e, VERTEX_DESTROY_RENDERER_DATA_00E89AC0).len(), 2);
    }

    #[test]
    fn recreate_renderer_data_jumps_to_the_base() {
        let mut e = Engine::new();
        e.register(VERTEX_RECREATE_RENDERER_DATA_00E89AD0, |_, _| {
            0x201u32.into_ret()
        });
        assert_eq!(e.call(0x00e9_7240, &args![0x40u32]).u32(), 0x201);
    }

    #[test]
    fn get_variable_count_reads_the_description() {
        let (mut e, shader, _) = rig(0x44, 0);
        e.register(TABLE_DESCRIBE, |e, a| {
            e.mem.set_u32(a[1] + 8, 9);
            Ret::default()
        });
        assert_eq!(e.call(0x00e9_7250, &args![shader]).u32(), 9);
        e.mem.set_u32(shader + 0x44, 0);
        assert_eq!(e.call(0x00e9_7250, &args![shader]).u32(), 0);
    }

    #[test]
    fn get_variable_name_answers_the_description_name() {
        let (mut e, shader, _) = rig(0x44, 0);
        e.register(TABLE_BY_INDEX, |_, _| 0x99u32.into_ret());
        e.register(TABLE_DESCRIBE_CONSTANT, |e, a| {
            e.mem.set_u32(a[2], 0x0e61_0000);
            Ret::default()
        });
        assert_eq!(e.call(0x00e9_7280, &args![shader, 1u32]).u32(), 0x0e61_0000);
        let table = e.mem.u32(shader + 0x44);
        assert_eq!(calls(&e, TABLE_DESCRIBE_CONSTANT)[0][..2], [table, 0x99]);
    }

    #[test]
    fn set_shader_constant_uses_the_vertex_slot_and_helpers() {
        let (mut e, shader, device) = rig(0x44, 0);
        let entry = new_entry(&mut e, 0x0e70_0008, 8);
        assert!(e
            .call(0x00e9_72e0, &args![shader, entry, 0x2000u32, 0u32])
            .bool());
        let transpose = calls(&e, MATRIX_TRANSPOSE_00EE6DD4);
        assert_eq!(transpose[0][1], 0x2000);
        assert_eq!(
            calls(&e, DEVICE_VERTEX_FLOATS),
            vec![vec![device, 6, transpose[0][0], 2]]
        );
        let (mut e, shader, _) = rig(0x44, 0);
        e.register(VERTEX_SIDE.int_setter, |_, _| 0u32.into_ret());
        let entry = new_entry(&mut e, 0x0e70_0008, 3);
        assert!(!e
            .call(0x00e9_72e0, &args![shader, entry, 0x2000u32, 0u32])
            .bool());
        let renderer = e.mem.u32(shader + 0x28);
        assert_eq!(
            calls(&e, VERTEX_SIDE.int_setter),
            vec![vec![renderer, 6, 0x2000, 2, 0]]
        );
    }

    #[test]
    fn set_shader_constant_array_skips_the_transpose_for_the_untransposed_types() {
        // Indexed, untransposed: the element's own data goes to the device.
        let (mut e, shader, device) = rig(0x44, 0);
        let entry = new_entry(&mut e, 0x0e70_0008, 8);
        e.mem.set_u32(entry + 0x10, 0x22);
        let indices = e.mem.alloc(4);
        e.mem.set_u16(indices, 1);
        e.mem.set_u16(indices + 2, 0);
        let call = args![shader, entry, 0x3000u32, 2u32, 4u32, indices];
        assert!(e.call(0x00e9_74f0, &call).bool());
        assert!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        assert_eq!(
            calls(&e, DEVICE_VERTEX_FLOATS),
            vec![
                vec![device, 6, 0x3000 + 4 * 16, 4],
                vec![device, 10, 0x3000, 4]
            ]
        );
        // Plain, untransposed: one call to the renderer helper.
        e.call_log = Some(vec![]);
        e.register(VERTEX_SIDE.float_setter, |_, _| 1u32.into_ret());
        let call = args![shader, entry, 0x3000u32, 2u32, 4u32, 0u32];
        assert!(e.call(0x00e9_74f0, &call).bool());
        let renderer = e.mem.u32(shader + 0x28);
        assert_eq!(
            calls(&e, VERTEX_SIDE.float_setter),
            vec![vec![renderer, 6, 0x3000, 2, 0]]
        );
        assert!(calls(&e, DEVICE_VERTEX_FLOATS).is_empty());
        // Plain, transposed: each matrix in turn.
        e.mem.set_u32(entry + 0x10, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_74f0, &call).bool());
        let sources: Vec<u32> = calls(&e, MATRIX_TRANSPOSE_00EE6DD4)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sources, vec![0x3000, 0x3000 + 4 * 16]);
        // Indexed, transposed: the indexed matrix transposed.
        e.call_log = Some(vec![]);
        let call = args![shader, entry, 0x3000u32, 1u32, 4u32, indices];
        assert!(e.call(0x00e9_74f0, &call).bool());
        assert_eq!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4)[0][1], 0x3000 + 4 * 16);
        // No elements: false.
        let call = args![shader, entry, 0x3000u32, 0u32, 4u32, indices];
        assert!(!e.call(0x00e9_74f0, &call).bool());
        // A failing device call: false at once.
        let (mut e, shader, _) = rig(0x44, 0x8000_0000);
        let entry = new_entry(&mut e, 0x0e70_0008, 9);
        let call = args![shader, entry, 0x3000u32, 2u32, 4u32, 0u32];
        assert!(!e.call(0x00e9_74f0, &call).bool());
        assert_eq!(calls(&e, DEVICE_VERTEX_FLOATS).len(), 1);
    }
}
