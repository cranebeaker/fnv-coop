//! `NiXenonRenderer/nid3dhlslpixelshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DPixelShader::DestroyRendererData` (`00e898c0`) and `RecreateRendererData` (`00e898d0`),
/// both `thiscall` (unit `nid3dpixelshader.cpp`), reached by jumps.
const PIXEL_DESTROY_RENDERER_DATA_00E898C0: u32 = 0x00e8_98c0;
const PIXEL_RECREATE_RENDERER_DATA_00E898D0: u32 = 0x00e8_98d0;

// Translated from 00e96cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::DestroyRendererData` (engine map), `thiscall`: the constant table at
/// +0x38, when set, is released (its slot 8, `stdcall`) and cleared; then
/// `NiD3DPixelShader::DestroyRendererData` (`00e898c0`, a jump).
pub fn ni_d3d_hlsl_pixel_shader_destroy_renderer_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let table = e.mem.u32(base.wrapping_add(0x38));
    if table != 0 {
        e.vcall(table, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x38), 0);
    }
    e.call(PIXEL_DESTROY_RENDERER_DATA_00E898C0, &args![this]);
}

// Translated from 00e96d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::RecreateRendererData` (engine map), `thiscall`: a jump to
/// `NiD3DPixelShader::RecreateRendererData` (`00e898d0`); answers what it answers.
pub fn ni_d3d_hlsl_pixel_shader_recreate_renderer_data(e: &mut Engine, this: Ptr) -> u32 {
    e.call(PIXEL_RECREATE_RENDERER_DATA_00E898D0, &args![this])
        .u32()
}

/// The variable count of the constant table `table` (shared by the pixel and vertex shaders):
/// the table's slot 0x14 (`stdcall`) fills a 12-byte description on the stack; its third word
/// when the call answers a non-negative result, else 0 (the code masks the word with `SETL`,
/// `DEC`, `AND`).
pub(crate) fn hlsl_variable_count(e: &mut Engine, table: u32) -> u32 {
    e.with_stack(0xc, |e, description| {
        let description = description.addr();
        let result = e.vcall(table, 0x14, &args![description]).i32();
        let mask = u32::from(result < 0).wrapping_sub(1);
        mask & e.mem.u32(description + 8)
    })
}

// Translated from 00e96d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::GetVariableCount` (engine map), `thiscall`: 0 without a constant
/// table (+0x38); else the count its description gives ([`hlsl_variable_count`]).
pub fn ni_d3d_hlsl_pixel_shader_get_variable_count(e: &mut Engine, this: Ptr) -> u32 {
    let table = e.mem.u32(this.addr().wrapping_add(0x38));
    if table == 0 {
        return 0;
    }
    hlsl_variable_count(e, table)
}

/// The name of variable `index` of the constant table `table` (shared by the pixel and vertex
/// shaders; the table is not checked): the table's slot 0x20 (`stdcall`: no parent, the index)
/// answers its handle (0: answers 0); slot 0x18 (`stdcall`: the handle, a 0x30-byte description
/// on the stack and the address of a count set to 1 first) describes it. 0 when that answers a
/// negative result or a count of 0; else the description's first word.
pub(crate) fn hlsl_variable_name(e: &mut Engine, table: u32, index: u32) -> u32 {
    let handle = e.vcall(table, 0x20, &args![0u32, index]).u32();
    if handle == 0 {
        return 0;
    }
    e.with_stack(0x34, |e, frame| {
        let description = frame.addr();
        let count = description + 0x30;
        e.mem.set_u32(count, 1);
        let result = e
            .vcall(table, 0x18, &args![handle, description, count])
            .i32();
        if result < 0 || e.mem.u32(count) == 0 {
            0
        } else {
            e.mem.u32(description)
        }
    })
}

// Translated from 00e96d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::GetVariableName` (engine map), `thiscall` with the index: the name
/// [`hlsl_variable_name`] gives from the constant table at +0x38.
pub fn ni_d3d_hlsl_pixel_shader_get_variable_name(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let table = e.mem.u32(this.addr().wrapping_add(0x38));
    hlsl_variable_name(e, table, index)
}

/// What the `SetShaderConstant` functions of the two HLSL shaders call: the empty
/// `NiFixedString` word compared with (`0109b220`), the imported `InterlockedIncrement` and
/// `InterlockedDecrement` (on the string's count, 8 bytes before its text), the `NiFixedString`
/// assignment from text `0043ba10` (`thiscall` on the string, the text), the constant map
/// entry's name setter `00e81c60` (unit `nid3dshaderconstantmap.cpp`; `thiscall` on the entry,
/// the address of a string), the report `00a29680` (`cdecl`: format and arguments),
/// `D3DXMatrixTranspose` (`00ee6dd4`, `stdcall`: out, in) and the kind table `011f5fc4` (a word
/// per type byte of the entry's +0x14).
const EMPTY_FIXED_STRING_0109B220: u32 = 0x0109_b220;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const FIXED_STRING_ASSIGN_0043BA10: u32 = 0x0043_ba10;
const ENTRY_SET_NAME_00E81C60: u32 = 0x00e8_1c60;
const REPORT_00A29680: u32 = 0x00a2_9680;
pub(crate) const MATRIX_TRANSPOSE_00EE6DD4: u32 = 0x00ee_6dd4;
const CONSTANT_KINDS_011F5FC4: u32 = 0x011f_5fc4;

/// The kind word of a shader constant map entry (its type byte at +0x14).
pub(crate) fn hlsl_constant_kind(e: &Engine, entry: u32) -> u32 {
    let kind = e.mem.u32(entry.wrapping_add(0x14)) & 0xff;
    e.mem
        .u32(CONSTANT_KINDS_011F5FC4.wrapping_add(kind.wrapping_mul(4)))
}

/// What differs between the pixel and the vertex shader's `SetShaderConstant`.
pub(crate) struct HlslSide {
    /// The constant table's offset in the shader.
    pub(crate) table: u32,
    /// The report's format when the entry's name is not in the table.
    pub(crate) not_found: u32,
    /// The render state helper for kind 1 (`thiscall` on the render state: register, data,
    /// count, 0).
    pub(crate) bool_setter: u32,
    /// The same for kind 3.
    pub(crate) int_setter: u32,
    /// The same for the kinds 4 to 7 and 10.
    pub(crate) float_setter: u32,
    /// The device's slot for float registers (`stdcall`: register, data, count).
    pub(crate) device_float_slot: u32,
}

/// The pixel shader's.
const PIXEL_SIDE: HlslSide = HlslSide {
    table: 0x38,
    not_found: 0x010f_13f0,
    bool_setter: 0x00e8_98e0,
    int_setter: 0x00e8_9940,
    float_setter: 0x00e8_9910,
    device_float_slot: 0x1b4,
};

/// Drops the extra reference the lookup took on a name (unless it is the empty string).
fn release_name(e: &mut Engine, name: u32) {
    if name != e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
}

/// The start of both HLSL shaders' `SetShaderConstant` and `SetShaderConstantArray`: unless
/// the entry is resolved already (its byte +0x35), its name (`NiFixedString` at +0x24, counted
/// up into the stack word at `frame + name_at` unless it is the empty string; an empty name is
/// first made from the entry's text at +0xc, `0043ba10`, and set on the entry, `00e81c60`) is
/// looked up in the constant table (the shader's word at `table_at`; slot 0x24, `stdcall`: no
/// parent, the name). Not found: the report (`00a29680` with the format `not_found`, the
/// shader's +0xc and the name), the name let go, false.
/// Found: the table's slot 0x18 (`stdcall`: the handle, the description at `frame + 0x20` and
/// the count at `frame + count_at`, set to 1 first; its result not checked) describes it; the
/// description's register (+8) and register count (+0xc) go to the entry's +0x1c and +0x20, the
/// entry is marked resolved and the name let go. Answers true.
pub(crate) fn hlsl_resolve_entry(
    e: &mut Engine,
    this: u32,
    entry: u32,
    frame: u32,
    name_at: u32,
    count_at: u32,
    table_at: u32,
    not_found: u32,
) -> bool {
    if e.mem.u8(entry.wrapping_add(0x35)) != 0 {
        return true;
    }
    let held = e.mem.u32(entry.wrapping_add(0x24));
    if held != e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![held.wrapping_sub(8)]);
    }
    let mut name = e.mem.u32(entry.wrapping_add(0x24));
    let slot = frame + name_at;
    e.mem.set_u32(slot, name);
    if name == e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(
            FIXED_STRING_ASSIGN_0043BA10,
            &args![slot, entry.wrapping_add(0xc)],
        );
        e.call(ENTRY_SET_NAME_00E81C60, &args![entry, slot]);
        name = e.mem.u32(slot);
    }
    let table = e.mem.u32(this.wrapping_add(table_at));
    let handle = e.vcall(table, 0x24, &args![0u32, name]).u32();
    if handle == 0 {
        let shader_name = e.mem.u32(this.wrapping_add(0xc));
        e.call(REPORT_00A29680, &args![not_found, shader_name, name]);
        release_name(e, name);
        return false;
    }
    let table = e.mem.u32(this.wrapping_add(table_at));
    let description = frame + 0x20;
    let count = frame + count_at;
    e.mem.set_u32(count, 1);
    e.vcall(table, 0x18, &args![handle, description, count]);
    let register = e.mem.u32(description + 8);
    let registers = e.mem.u32(description + 0xc);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    e.mem.set_u32(entry.wrapping_add(0x20), registers);
    e.mem.set_u8(entry.wrapping_add(0x35), 1);
    release_name(e, name);
    true
}

/// The device (+0x10f8) of the shader's render state (`m_pkD3DRenderState` +0x28, Xbox PDB).
pub(crate) fn hlsl_shader_device(e: &Engine, this: u32) -> u32 {
    let state = e.mem.u32(this.wrapping_add(0x28));
    e.mem.u32(state.wrapping_add(0x10f8))
}

/// The body both HLSL shaders' `SetShaderConstant` share (only `side` differs): false without
/// the constant table or the shader word at +0x20; a null data takes the entry's +0x30; the
/// entry resolved ([`hlsl_resolve_entry`], false when its name is unknown). By the entry's
/// kind: 8 and 9 transpose the 4x4 matrix at the data into the stack (`D3DXMatrixTranspose`)
/// and hand it to the device's float slot (register +0x1c, register count +0x20; true when it
/// answers a non-negative result); 4 to 7 and 10 go to the float setter, 3 to the int setter
/// and 1 to the bool setter (the render state at +0x28 with the register, the data, the
/// register count and 0; true when it answers true); any other kind: false.
pub(crate) fn hlsl_set_shader_constant(
    e: &mut Engine,
    this: u32,
    entry: u32,
    data: u32,
    side: &HlslSide,
) -> bool {
    if e.mem.u32(this.wrapping_add(side.table)) == 0 || e.mem.u32(this.wrapping_add(0x20)) == 0 {
        return false;
    }
    let data = if data == 0 {
        e.mem.u32(entry.wrapping_add(0x30))
    } else {
        data
    };
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        if !hlsl_resolve_entry(
            e,
            this,
            entry,
            frame,
            0x18,
            0x18,
            side.table,
            side.not_found,
        ) {
            return false;
        }
        let setter = match hlsl_constant_kind(e, entry) {
            8 | 9 => {
                let matrix = frame + 0x20;
                e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, data]);
                let registers = e.mem.u32(entry.wrapping_add(0x20));
                let device = hlsl_shader_device(e, this);
                let register = e.mem.u32(entry.wrapping_add(0x1c));
                let result = e.vcall(
                    device,
                    side.device_float_slot,
                    &args![register, matrix, registers],
                );
                return result.i32() >= 0;
            }
            4..=7 | 10 => side.float_setter,
            3 => side.int_setter,
            1 => side.bool_setter,
            _ => return false,
        };
        let registers = e.mem.u32(entry.wrapping_add(0x20));
        let register = e.mem.u32(entry.wrapping_add(0x1c));
        let state = e.mem.u32(this.wrapping_add(0x28));
        e.call(setter, &args![state, register, data, registers, 0u32])
            .bool()
    })
}

// Translated from 00e96da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::SetShaderConstant` (engine map), `thiscall` with three words (the
/// constant map entry, the data and a count it does not read): [`hlsl_set_shader_constant`]
/// with the pixel shader's table (+0x38), report and setters (`00e898e0`, `00e89940`,
/// `00e89910`) and the device's slot 0x1b4.
pub fn ni_d3d_hlsl_pixel_shader_set_shader_constant(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    _unused_3: u32,
) -> bool {
    hlsl_set_shader_constant(e, this.addr(), entry, data, &PIXEL_SIDE)
}

/// The report's format when `SetShaderConstantArray` does not find the name (`010f1438`).
const ARRAY_NOT_FOUND_010F1438: u32 = 0x010f_1438;

// Translated from 00e96fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::SetShaderConstantArray` (engine map), `thiscall` with five words: the
/// constant map entry, the data, the element count, the registers per element and the 16-bit
/// index array (or 0). False without the constant table (+0x38) or the shader word at +0x20, or
/// unless the entry's kind is 8 or 9. A null data takes the entry's +0x30; the entry is resolved
/// ([`hlsl_resolve_entry`]; false when its name is unknown). Then for each element its 4x4
/// matrix (with an index array the indexed one, index times registers per element times 16
/// bytes on from the data; else the next one, stepping by registers per element times 16) is
/// transposed into the stack (`D3DXMatrixTranspose`) and handed to the device (slot 0x1b4: the
/// register, from the entry's +0x1c stepping by the registers per element, the matrix and the
/// registers per element). A negative result ends it with false; answers true when every call
/// succeeded (false for no elements).
pub fn ni_d3d_hlsl_pixel_shader_set_shader_constant_array(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    count: u32,
    registers: u32,
    indices: u32,
) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x38)) == 0 || e.mem.u32(base.wrapping_add(0x20)) == 0 {
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
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        if !hlsl_resolve_entry(
            e,
            base,
            entry,
            frame,
            0x14,
            0x1c,
            0x38,
            ARRAY_NOT_FOUND_010F1438,
        ) {
            return false;
        }
        let matrix = frame + 0x20;
        let mut register = e.mem.u32(entry.wrapping_add(0x1c));
        let mut source = data;
        for element in 0..count {
            if indices != 0 {
                let index = u32::from(e.mem.u16(indices.wrapping_add(element.wrapping_mul(2))));
                source = (index.wrapping_mul(registers) << 4).wrapping_add(data);
            }
            e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, source]);
            let device = hlsl_shader_device(e, base);
            let result = e.vcall(device, 0x1b4, &args![register, matrix, registers]);
            if result.i32() < 0 {
                return false;
            }
            if indices == 0 {
                source = source.wrapping_add(registers << 4);
            }
            register = register.wrapping_add(registers);
        }
        count != 0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_6cd0,
            ni_d3d_hlsl_pixel_shader_destroy_renderer_data(Ptr)
        ),
        entry!(
            0x00e9_6d00,
            ni_d3d_hlsl_pixel_shader_recreate_renderer_data(Ptr) -> u32
        ),
        entry!(
            0x00e9_6d10,
            ni_d3d_hlsl_pixel_shader_get_variable_count(Ptr) -> u32
        ),
        entry!(
            0x00e9_6d40,
            ni_d3d_hlsl_pixel_shader_get_variable_name(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_6da0,
            ni_d3d_hlsl_pixel_shader_set_shader_constant(Ptr, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e9_6fb0,
            ni_d3d_hlsl_pixel_shader_set_shader_constant_array(Ptr, u32, u32, u32, u32, u32)
                -> bool
        ),
    ]
}

/// Doubles and objects the HLSL shader tests of both units share.
#[cfg(test)]
pub(crate) mod hlsl_test_support {
    use super::*;

    /// The constant table's vtable and its slots (description 0x14, constant by index 0x20,
    /// by name 0x24, its description 0x18, release 8).
    pub(crate) const TABLE_VTABLE: u32 = 0x0e20_0000;
    pub(crate) const TABLE_RELEASE: u32 = 0x0e21_0008;
    pub(crate) const TABLE_DESCRIBE: u32 = 0x0e21_0014;
    pub(crate) const TABLE_DESCRIBE_CONSTANT: u32 = 0x0e21_0018;
    pub(crate) const TABLE_BY_INDEX: u32 = 0x0e21_0020;
    pub(crate) const TABLE_BY_NAME: u32 = 0x0e21_0024;
    /// The device's vtable (float slots 0x178 and 0x1b4 at these addresses).
    pub(crate) const DEVICE_VTABLE: u32 = 0x0e30_0000;
    pub(crate) const DEVICE_VERTEX_FLOATS: u32 = 0x0e31_0178;
    pub(crate) const DEVICE_PIXEL_FLOATS: u32 = 0x0e31_01b4;

    /// The calls to `addr` in the log.
    pub(crate) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A shader whose constant table is at `table_at`, with a renderer and device; the table
    /// finds every name (handle 0x77) and describes it as register 6, 2 registers; the device
    /// answers `device_result`. Answers the engine, the shader and the device.
    pub(crate) fn rig(table_at: u32, device_result: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.map(0x011f_5000, 0x1000);
        let mut slots = vec![0u32; 10];
        slots[2] = TABLE_RELEASE;
        slots[5] = TABLE_DESCRIBE;
        slots[6] = TABLE_DESCRIBE_CONSTANT;
        slots[8] = TABLE_BY_INDEX;
        slots[9] = TABLE_BY_NAME;
        e.put_vtable(TABLE_VTABLE, &slots);
        let mut device_slots = vec![0u32; 0x6e];
        device_slots[0x178 / 4] = DEVICE_VERTEX_FLOATS;
        device_slots[0x1b4 / 4] = DEVICE_PIXEL_FLOATS;
        e.put_vtable(DEVICE_VTABLE, &device_slots);
        e.register(TABLE_RELEASE, |_, _| Ret::default());
        e.register(TABLE_BY_NAME, |_, _| 0x77u32.into_ret());
        e.register(TABLE_DESCRIBE_CONSTANT, |e, a| {
            e.mem.set_u32(a[2] + 8, 6);
            e.mem.set_u32(a[2] + 0xc, 2);
            Ret::default()
        });
        for slot in [DEVICE_VERTEX_FLOATS, DEVICE_PIXEL_FLOATS] {
            e.register_double(slot, move |_, _| device_result.into_ret());
        }
        for callee in [
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
            REPORT_00A29680,
            MATRIX_TRANSPOSE_00EE6DD4,
            FIXED_STRING_ASSIGN_0043BA10,
            ENTRY_SET_NAME_00E81C60,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, TABLE_VTABLE);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x1100);
        e.mem.set_u32(renderer + 0x10f8, device);
        let shader = e.mem.alloc(0x50);
        e.mem.set_u32(shader + table_at, table);
        e.mem.set_u32(shader + 0x20, 1);
        e.mem.set_u32(shader + 0x28, renderer);
        e.mem.set_u32(shader + 0xc, 0x0e40_0000);
        e.call_log = Some(vec![]);
        (e, shader, device)
    }

    /// A constant map entry named by the string at `name` (its count 8 bytes before) of kind
    /// `kind` (type byte 2).
    pub(crate) fn new_entry(e: &mut Engine, name: u32, kind: u32) -> u32 {
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x14, 2);
        e.mem.set_u32(entry + 0x24, name);
        e.mem.set_u32(entry + 0x30, 0x0e50_0000);
        e.mem.set_u32(CONSTANT_KINDS_011F5FC4 + 8, kind);
        entry
    }
}

#[cfg(test)]
mod n008_tests {
    use super::hlsl_test_support::*;
    use super::*;

    #[test]
    fn destroy_renderer_data_releases_the_table_then_runs_the_base() {
        for held in [true, false] {
            let (mut e, shader, _) = rig(0x38, 0);
            e.register(PIXEL_DESTROY_RENDERER_DATA_00E898C0, |_, _| Ret::default());
            let table = e.mem.u32(shader + 0x38);
            if !held {
                e.mem.set_u32(shader + 0x38, 0);
            }
            e.call(0x00e9_6cd0, &args![shader]);
            let released = if held { vec![vec![table]] } else { vec![] };
            assert_eq!(calls(&e, TABLE_RELEASE), released);
            assert_eq!(e.mem.u32(shader + 0x38), 0);
            assert_eq!(
                calls(&e, PIXEL_DESTROY_RENDERER_DATA_00E898C0),
                vec![vec![shader]]
            );
        }
    }

    #[test]
    fn recreate_renderer_data_jumps_to_the_base() {
        let mut e = Engine::new();
        e.register(PIXEL_RECREATE_RENDERER_DATA_00E898D0, |_, _| {
            0x101u32.into_ret()
        });
        assert_eq!(e.call(0x00e9_6d00, &args![0x40u32]).u32(), 0x101);
    }

    #[test]
    fn get_variable_count_masks_a_failed_description() {
        for (result, expected) in [(0u32, 5u32), (0x8000_0001, 0)] {
            let (mut e, shader, _) = rig(0x38, 0);
            e.register_double(TABLE_DESCRIBE, move |e, a| {
                e.mem.set_u32(a[1] + 8, 5);
                result.into_ret()
            });
            assert_eq!(e.call(0x00e9_6d10, &args![shader]).u32(), expected);
            e.mem.set_u32(shader + 0x38, 0);
            assert_eq!(e.call(0x00e9_6d10, &args![shader]).u32(), 0);
        }
    }

    #[test]
    fn get_variable_name_needs_a_handle_a_description_and_a_count() {
        // (handle, description result, count written, expected)
        for (handle, result, count, expected) in [
            (0x55u32, 0u32, 1u32, 0x0e60_0000u32),
            (0, 0, 1, 0),
            (0x55, 0x8000_0000, 1, 0),
            (0x55, 0, 0, 0),
        ] {
            let (mut e, shader, _) = rig(0x38, 0);
            e.register_double(TABLE_BY_INDEX, move |_, _| handle.into_ret());
            e.register_double(TABLE_DESCRIBE_CONSTANT, move |e, a| {
                assert_eq!(e.mem.u32(a[3]), 1);
                e.mem.set_u32(a[2], 0x0e60_0000);
                e.mem.set_u32(a[3], count);
                result.into_ret()
            });
            assert_eq!(e.call(0x00e9_6d40, &args![shader, 3u32]).u32(), expected);
            let table = e.mem.u32(shader + 0x38);
            assert_eq!(calls(&e, TABLE_BY_INDEX), vec![vec![table, 0, 3]]);
        }
    }

    #[test]
    fn set_shader_constant_resolves_the_name_and_picks_by_kind() {
        // A matrix kind: transposed and set through the device's float slot.
        let (mut e, shader, device) = rig(0x38, 0);
        let entry = new_entry(&mut e, 0x0e70_0008, 8);
        assert!(e
            .call(0x00e9_6da0, &args![shader, entry, 0u32, 0u32])
            .bool());
        assert_eq!(e.mem.u32(entry + 0x1c), 6);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
        assert_eq!(e.mem.u8(entry + 0x35), 1);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![0x0e70_0000]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e70_0000]]
        );
        let transpose = calls(&e, MATRIX_TRANSPOSE_00EE6DD4);
        assert_eq!(transpose.len(), 1);
        assert_eq!(transpose[0][1], 0x0e50_0000);
        assert_eq!(
            calls(&e, DEVICE_PIXEL_FLOATS),
            vec![vec![device, 6, transpose[0][0], 2]]
        );
        // Resolved now: no lookup; a failing device call answers false.
        let (mut e, shader, _) = rig(0x38, 0x8000_0000);
        let entry = new_entry(&mut e, 0x0e70_0008, 9);
        e.mem.set_u8(entry + 0x35, 1);
        assert!(!e
            .call(0x00e9_6da0, &args![shader, entry, 0x1234u32, 0u32])
            .bool());
        assert!(calls(&e, TABLE_BY_NAME).is_empty());
        // The renderer helpers by kind.
        for (kind, setter) in [
            (5u32, PIXEL_SIDE.float_setter),
            (3, PIXEL_SIDE.int_setter),
            (1, PIXEL_SIDE.bool_setter),
        ] {
            let (mut e, shader, _) = rig(0x38, 0);
            e.register(setter, |_, _| 1u32.into_ret());
            let entry = new_entry(&mut e, 0x0e70_0008, kind);
            assert!(e
                .call(0x00e9_6da0, &args![shader, entry, 0x1234u32, 0u32])
                .bool());
            let renderer = e.mem.u32(shader + 0x28);
            assert_eq!(calls(&e, setter), vec![vec![renderer, 6, 0x1234, 2, 0]]);
        }
        // An unknown kind: false.
        let (mut e, shader, _) = rig(0x38, 0);
        let entry = new_entry(&mut e, 0x0e70_0008, 2);
        assert!(!e
            .call(0x00e9_6da0, &args![shader, entry, 0x1234u32, 0u32])
            .bool());
    }

    #[test]
    fn set_shader_constant_reports_an_unknown_name_and_makes_an_empty_one() {
        let (mut e, shader, _) = rig(0x38, 0);
        e.register(TABLE_BY_NAME, |_, _| 0u32.into_ret());
        // The empty string word (0109b220) is 0, so a null name is made from the text.
        e.register(FIXED_STRING_ASSIGN_0043BA10, |e, a| {
            e.mem.set_u32(a[0], 0x0e80_0008);
            Ret::default()
        });
        let entry = new_entry(&mut e, 0, 8);
        assert!(!e
            .call(0x00e9_6da0, &args![shader, entry, 0u32, 0u32])
            .bool());
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        let assigned = calls(&e, FIXED_STRING_ASSIGN_0043BA10);
        assert_eq!(assigned[0][1], entry + 0xc);
        assert_eq!(
            calls(&e, ENTRY_SET_NAME_00E81C60),
            vec![vec![entry, assigned[0][0]]]
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![PIXEL_SIDE.not_found, 0x0e40_0000, 0x0e80_0008]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e80_0000]]
        );
        assert_eq!(e.mem.u8(entry + 0x35), 0);
        // Without the table or the shader word: false before anything.
        e.mem.set_u32(shader + 0x20, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_6da0, &args![shader, entry, 0u32, 0u32])
            .bool());
        assert!(calls(&e, TABLE_BY_NAME).is_empty());
    }

    #[test]
    fn set_shader_constant_array_walks_indexed_and_plain_matrices() {
        let (mut e, shader, device) = rig(0x38, 0);
        let entry = new_entry(&mut e, 0x0e70_0008, 9);
        let indices = e.mem.alloc(4);
        e.mem.set_u16(indices, 3);
        e.mem.set_u16(indices + 2, 1);
        let call = args![shader, entry, 0x1000u32, 2u32, 4u32, indices];
        assert!(e.call(0x00e9_6fb0, &call).bool());
        let sources: Vec<u32> = calls(&e, MATRIX_TRANSPOSE_00EE6DD4)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sources, vec![0x1000 + 3 * 4 * 16, 0x1000 + 4 * 16]);
        let registers: Vec<u32> = calls(&e, DEVICE_PIXEL_FLOATS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(registers, vec![6, 10]);
        assert_eq!(calls(&e, DEVICE_PIXEL_FLOATS)[0][0], device);
        // Without indices the matrices follow each other.
        e.call_log = Some(vec![]);
        let call = args![shader, entry, 0x1000u32, 2u32, 4u32, 0u32];
        assert!(e.call(0x00e9_6fb0, &call).bool());
        let sources: Vec<u32> = calls(&e, MATRIX_TRANSPOSE_00EE6DD4)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sources, vec![0x1000, 0x1000 + 4 * 16]);
        // No elements: false; a kind other than 8 or 9: false.
        let call = args![shader, entry, 0x1000u32, 0u32, 4u32, 0u32];
        assert!(!e.call(0x00e9_6fb0, &call).bool());
        e.mem.set_u32(CONSTANT_KINDS_011F5FC4 + 8, 5);
        let call = args![shader, entry, 0x1000u32, 2u32, 4u32, 0u32];
        assert!(!e.call(0x00e9_6fb0, &call).bool());
        // A failing device call stops the walk.
        let (mut e, shader, _) = rig(0x38, 0x8000_0000);
        let entry = new_entry(&mut e, 0x0e70_0008, 8);
        let call = args![shader, entry, 0x1000u32, 2u32, 4u32, 0u32];
        assert!(!e.call(0x00e9_6fb0, &call).bool());
        assert_eq!(calls(&e, DEVICE_PIXEL_FLOATS).len(), 1);
    }
}
