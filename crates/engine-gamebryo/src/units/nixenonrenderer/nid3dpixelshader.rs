//! `NiXenonRenderer/nid3dpixelshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DPixelShader`'s vtable (`010ef7d4`, RTTI `.?AVNiD3DPixelShader@@`).
const VTABLE: u32 = 0x010e_f7d4;
/// The shader factory's pixel-shader name removal `00e937c0` (`cdecl`, one word: the shader).
const FACTORY_REMOVE_PIXEL_SHADER: u32 = 0x00e9_37c0;
/// `NiXenonRenderer::PurgePixelShader` (engine map, `00e699c0`, `thiscall` on the renderer:
/// the shader).
const PURGE_PIXEL_SHADER: u32 = 0x00e6_99c0;
/// The base destructor body `00e89840` (unit `nid3dshaderprogram.cpp`, unnamed in the engine
/// map; `thiscall`, no stack words), reached by a jump.
const SHADER_PROGRAM_DESTRUCT: u32 = 0x00e8_9840;

// Translated from 00e89970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `NiD3DPixelShader`'s destructor
/// body. Stores the class's vtable, removes the shader from the factory (`00e937c0`); when
/// the shader handle at +0x2c is set, the renderer at +0x24 purges it
/// (`NiXenonRenderer::PurgePixelShader`); clears +0x2c and ends in the base destructor body
/// `00e89840` (a jump).
pub fn fn_00e89970(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(FACTORY_REMOVE_PIXEL_SHADER, &args![this]);
    if e.mem.u32(base + 0x2c) != 0 {
        let renderer = e.mem.u32(base + 0x24);
        e.call(PURGE_PIXEL_SHADER, &args![renderer, this]);
    }
    e.mem.set_u32(base + 0x2c, 0);
    e.call(SHADER_PROGRAM_DESTRUCT, &args![this]);
}

// Translated from 00e898c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::DestroyRendererData` (engine map), `thiscall`: when the shader handle at
/// +0x2c is set, the renderer at +0x24 purges the shader (`NiXenonRenderer::PurgePixelShader`).
pub fn ni_d3d_pixel_shader_destroy_renderer_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x2c)) != 0 {
        let renderer = e.mem.u32(base.wrapping_add(0x24));
        e.call(PURGE_PIXEL_SHADER, &args![renderer, this]);
    }
}

/// `NiD3DShaderProgramFactory::RecreatePixelShader` (`00e92f30`, engine map; `cdecl`, the
/// shader).
const RECREATE_PIXEL_SHADER_00E92F30: u32 = 0x00e9_2f30;

// Translated from 00e898d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::RecreateRendererData` (engine map), `thiscall`: answers what
/// `NiD3DShaderProgramFactory::RecreatePixelShader` (`00e92f30`) answers for `this` (EAX as it
/// leaves it).
pub fn ni_d3d_pixel_shader_recreate_renderer_data(e: &mut Engine, this: Ptr) -> u32 {
    e.call(RECREATE_PIXEL_SHADER_00E92F30, &args![this]).u32()
}

/// The Direct3D device the render state object holds (+0x10f8).
fn device(e: &Engine, state: u32) -> u32 {
    e.mem.u32(state.wrapping_add(0x10f8))
}

/// A device call answering an `HRESULT`: true when it is not negative (`SETGE`).
fn succeeded(result: Ret) -> bool {
    result.i32() >= 0
}

// Translated from 00e898e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpixelshader.cpp`), `thiscall` on the render state
/// object (the shaders' `m_pkD3DRenderState` +0x28, Xbox PDB) with four stack words (the
/// fourth not read): the device's (+0x10f8) virtual slot 0x1c4 with the
/// first three (`stdcall`; `SetPixelShaderConstantB` in Direct3D 9's interface order); true
/// when it answers a non-negative result.
pub fn fn_00e898e0(
    e: &mut Engine,
    this: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    let device = device(e, this.addr());
    succeeded(e.vcall(device, 0x1c4, &args![register, data, count]))
}

// Translated from 00e89910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpixelshader.cpp`): as [`fn_00e898e0`] with the
/// device's slot 0x1b4 (`SetPixelShaderConstantF` in Direct3D 9's interface order).
pub fn fn_00e89910(
    e: &mut Engine,
    this: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    let device = device(e, this.addr());
    succeeded(e.vcall(device, 0x1b4, &args![register, data, count]))
}

// Translated from 00e89940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpixelshader.cpp`): as [`fn_00e898e0`] with the
/// device's slot 0x1bc (`SetPixelShaderConstantI` in Direct3D 9's interface order).
pub fn fn_00e89940(
    e: &mut Engine,
    this: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    let device = device(e, this.addr());
    succeeded(e.vcall(device, 0x1bc, &args![register, data, count]))
}

/// The kind of each shader constant type (`011f5fc4`, a word per type byte: the entry's +0x14
/// low byte indexes it).
const CONSTANT_KINDS_011F5FC4: u32 = 0x011f_5fc4;

/// The kind word of a shader constant map entry (its type byte at +0x14).
fn constant_kind(e: &Engine, entry: u32) -> u32 {
    let kind = e.mem.u32(entry.wrapping_add(0x14)) & 0xff;
    e.mem
        .u32(CONSTANT_KINDS_011F5FC4.wrapping_add(kind.wrapping_mul(4)))
}

// Translated from 00e899a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::SetShaderConstant` (engine map), `thiscall` with the constant map entry,
/// the data and the count. A null data takes the entry's +0x30, a zero count the entry's +0x20
/// (1 when that is 0 too). The render state's (+0x28) device's (+0x10f8) slot is picked by the
/// entry's kind (`011f5fc4`): 1 slot 0x1c4, 3 slot 0x1bc, any other 0x1b4; it gets the entry's
/// register (+0x1c), the data and the count. True when it answers a non-negative result.
pub fn ni_d3d_pixel_shader_set_shader_constant(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    count: u32,
) -> bool {
    let data = if data == 0 {
        e.mem.u32(entry.wrapping_add(0x30))
    } else {
        data
    };
    let mut count = count;
    if count == 0 {
        count = e.mem.u32(entry.wrapping_add(0x20));
        if count == 0 {
            count = 1;
        }
    }
    let register = e.mem.u32(entry.wrapping_add(0x1c));
    let state = e.mem.u32(this.addr().wrapping_add(0x28));
    let device = device(e, state);
    let slot = match constant_kind(e, entry) {
        1 => 0x1c4,
        3 => 0x1bc,
        _ => 0x1b4,
    };
    succeeded(e.vcall(device, slot, &args![register, data, count]))
}

// Translated from 00e89a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::SetShaderConstantArray` (engine map), `thiscall` with five words: the
/// constant map entry, the data, the element count, the registers per element and the 16-bit
/// index array. False unless the entry's kind (`011f5fc4`) is 8 or 9. Else for each element
/// the render state's (+0x28, read each time) device's slot 0x1b4 (`stdcall`) gets the
/// register (the entry's +0x1c, stepped by the registers per element), the data of the indexed
/// element (index
/// times registers per element times 16 bytes on from the data) and the registers per element;
/// its results are not checked. Answers true.
pub fn ni_d3d_pixel_shader_set_shader_constant_array(
    e: &mut Engine,
    this: Ptr,
    entry: u32,
    data: u32,
    count: u32,
    registers: u32,
    indices: u32,
) -> bool {
    let kind = constant_kind(e, entry);
    if kind != 8 && kind != 9 {
        return false;
    }
    let mut register = e.mem.u32(entry.wrapping_add(0x1c));
    for element in 0..count {
        let state = e.mem.u32(this.addr().wrapping_add(0x28));
        let index = u32::from(e.mem.u16(indices.wrapping_add(element.wrapping_mul(2))));
        let device = device(e, state);
        let source = (index.wrapping_mul(registers) << 4).wrapping_add(data);
        e.vcall(device, 0x1b4, &args![register, source, registers]);
        register = register.wrapping_add(registers);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_9970, fn_00e89970(Ptr)),
        entry!(0x00e8_98c0, ni_d3d_pixel_shader_destroy_renderer_data(Ptr)),
        entry!(
            0x00e8_98d0,
            ni_d3d_pixel_shader_recreate_renderer_data(Ptr) -> u32
        ),
        entry!(0x00e8_98e0, fn_00e898e0(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00e8_9910, fn_00e89910(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00e8_9940, fn_00e89940(Ptr, u32, u32, u32, u32) -> bool),
        entry!(
            0x00e8_99a0,
            ni_d3d_pixel_shader_set_shader_constant(Ptr, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e8_9a40,
            ni_d3d_pixel_shader_set_shader_constant_array(Ptr, u32, u32, u32, u32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e89970_purges_a_held_shader_then_runs_the_base() {
        for (handle, purges) in [(0x0e00_7000u32, true), (0, false)] {
            let mut e = Engine::new();
            for callee in [PURGE_PIXEL_SHADER, SHADER_PROGRAM_DESTRUCT] {
                e.register(callee, |_, _| Ret::default());
            }
            // The factory removal sees the vtable stored already.
            e.register(FACTORY_REMOVE_PIXEL_SHADER, |e, a| {
                assert_eq!(e.mem.u32(a[0]), VTABLE);
                Ret::default()
            });
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x24, 0x0e00_6000);
            e.mem.set_u32(this + 0x2c, handle);
            e.call_log = Some(vec![]);
            e.call(0x00e8_9970, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(FACTORY_REMOVE_PIXEL_SHADER, vec![this])];
            if purges {
                expected.push((PURGE_PIXEL_SHADER, vec![0x0e00_6000, this]));
            }
            expected.push((SHADER_PROGRAM_DESTRUCT, vec![this]));
            assert_eq!(log[1..].to_vec(), expected);
            assert_eq!(e.mem.u32(this + 0x2c), 0);
        }
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e10_0000;
    const SLOT_B: u32 = 0x0e11_01c4;
    const SLOT_F: u32 = 0x0e11_01b4;
    const SLOT_I: u32 = 0x0e11_01bc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with a renderer whose device answers `result` from the three constant slots;
    /// answers the renderer and the device.
    fn rig(result: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x72];
        slots[0x1b4 / 4] = SLOT_F;
        slots[0x1bc / 4] = SLOT_I;
        slots[0x1c4 / 4] = SLOT_B;
        e.put_vtable(DEVICE_VTABLE, &slots);
        for slot in [SLOT_B, SLOT_F, SLOT_I] {
            e.register_double(slot, move |_, _| result.into_ret());
        }
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x1100);
        e.mem.set_u32(renderer + 0x10f8, device);
        e.map(0x011f_5000, 0x1000);
        e.call_log = Some(vec![]);
        (e, renderer, device)
    }

    #[test]
    fn destroy_renderer_data_purges_only_a_held_shader() {
        for (handle, purges) in [(7u32, true), (0, false)] {
            let mut e = Engine::new();
            e.register(PURGE_PIXEL_SHADER, |_, _| Ret::default());
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x24, 0x0e00_6000);
            e.mem.set_u32(this + 0x2c, handle);
            e.call_log = Some(vec![]);
            e.call(0x00e8_98c0, &args![this]);
            let expected = if purges {
                vec![vec![0x0e00_6000, this]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, PURGE_PIXEL_SHADER), expected);
        }
    }

    #[test]
    fn recreate_renderer_data_answers_the_factory_result() {
        let mut e = Engine::new();
        e.register(RECREATE_PIXEL_SHADER_00E92F30, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x00e8_98d0, &args![0x40u32]).u32(), 0x41);
    }

    #[test]
    fn constant_setters_call_their_slot_and_test_the_sign() {
        for (addr, slot) in [
            (0x00e8_98e0u32, SLOT_B),
            (0x00e8_9910, SLOT_F),
            (0x00e8_9940, SLOT_I),
        ] {
            let (mut e, renderer, device) = rig(0);
            let ok = e.call(addr, &args![renderer, 3u32, 0x5000u32, 2u32, 0xdeadu32]);
            assert!(ok.bool());
            assert_eq!(calls(&e, slot), vec![vec![device, 3, 0x5000, 2]]);
            let (mut e, renderer, _) = rig(0x8000_0005);
            assert!(!e
                .call(addr, &args![renderer, 3u32, 0u32, 2u32, 0u32])
                .bool());
        }
    }

    #[test]
    fn set_shader_constant_picks_the_slot_by_kind_and_fills_defaults() {
        for (kind, slot) in [(1u32, SLOT_B), (3, SLOT_I), (8, SLOT_F)] {
            let (mut e, renderer, device) = rig(1);
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x28, renderer);
            let entry = e.mem.alloc(0x34);
            e.mem.set_u32(entry + 0x14, 0x0000_0102);
            e.mem.set_u32(CONSTANT_KINDS_011F5FC4 + 8, kind);
            e.mem.set_u32(entry + 0x1c, 9);
            e.mem.set_u32(entry + 0x20, 0);
            e.mem.set_u32(entry + 0x30, 0x7000);
            assert!(e.call(0x00e8_99a0, &args![this, entry, 0u32, 0u32]).bool());
            // No data and no counts: the entry's data and a count of 1.
            assert_eq!(calls(&e, slot), vec![vec![device, 9, 0x7000, 1]]);
            e.mem.set_u32(entry + 0x20, 4);
            e.call(0x00e8_99a0, &args![this, entry, 0u32, 0u32]);
            e.call(0x00e8_99a0, &args![this, entry, 0x6000u32, 2u32]);
            let seen = calls(&e, slot);
            assert_eq!(seen[1], vec![device, 9, 0x7000, 4]);
            assert_eq!(seen[2], vec![device, 9, 0x6000, 2]);
        }
    }

    #[test]
    fn set_shader_constant_array_needs_a_matrix_kind() {
        let (mut e, renderer, device) = rig(0x8000_0000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x28, renderer);
        let entry = e.mem.alloc(0x34);
        e.mem.set_u32(entry + 0x14, 5);
        e.mem.set_u32(entry + 0x1c, 10);
        let indices = e.mem.alloc(4);
        e.mem.set_u16(indices, 2);
        e.mem.set_u16(indices + 2, 0);
        e.mem.set_u32(CONSTANT_KINDS_011F5FC4 + 20, 3);
        let call = args![this, entry, 0x8000u32, 2u32, 4u32, indices];
        assert!(!e.call(0x00e8_9a40, &call).bool());
        assert!(calls(&e, SLOT_F).is_empty());
        e.mem.set_u32(CONSTANT_KINDS_011F5FC4 + 20, 9);
        // Failing device calls are not checked.
        assert!(e.call(0x00e8_9a40, &call).bool());
        assert_eq!(
            calls(&e, SLOT_F),
            vec![
                vec![device, 10, 0x8000 + 2 * 4 * 16, 4],
                vec![device, 14, 0x8000, 4]
            ]
        );
    }
}
