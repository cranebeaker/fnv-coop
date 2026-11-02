//! `NiXenonRenderer/nid3dvertexshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DVertexShader`'s vtable (its RTTI locator names
/// `.?AVNiD3DVertexShader@@`).
const VERTEX_SHADER_VTABLE: u32 = 0x010e_f87c;
/// Called first by the destructor (`00e93790`,
/// `nid3dshaderprogramfactory.cpp`; cdecl, one word: the shader).
const FACTORY_RELEASE_00E93790: u32 = 0x00e9_3790;
/// `NiXenonRenderer::PurgeVertexShader` (engine map; thiscall, one word).
const RENDERER_PURGE_VERTEX_SHADER: u32 = 0x00e6_9970;
/// The base destructor the destructor ends in (`00e89840`,
/// `nid3dshaderprogram.cpp`; thiscall).
const SHADER_PROGRAM_DESTRUCTOR: u32 = 0x00e8_9840;
/// `_memcpy` (CRT; cdecl: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// Table of constant kinds, a word per low byte of an entry's flags.
const CONSTANT_KINDS: u32 = 0x011f_5fc4;
/// The renderer's Direct3D device (renderer +0x10F8).
const RENDERER_DEVICE: u32 = 0x10f8;
/// The device's `SetVertexShaderConstantF`, `...I` and `...B` slots (the
/// order of the `IDirect3DDevice9` interface; the code calls them with the
/// device pushed first: device, register, data, count).
const DEVICE_SET_CONSTANT_F: u32 = 0x178;
const DEVICE_SET_CONSTANT_I: u32 = 0x180;
const DEVICE_SET_CONSTANT_B: u32 = 0x188;

/// The device of the shader's renderer (+0x28).
fn device(e: &Engine, this: Ptr) -> u32 {
    let renderer = e.mem.u32(this.addr() + 0x28);
    e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE))
}

/// The constant kind of an entry (the table entry for the low byte of its
/// flags, +0x14).
fn constant_kind(e: &Engine, entry: u32) -> u32 {
    let flags = e.mem.u32(entry + 0x14);
    e.mem.u32(CONSTANT_KINDS + (flags & 0xff) * 4)
}

/// Whether the entry's constant lives in its own buffer (flags' top nibble
/// 2 and a buffer at +0x30).
fn has_buffer(e: &Engine, entry: u32) -> bool {
    e.mem.u32(entry + 0x14) & 0xf000_0000 == 0x2000_0000 && e.mem.u32(entry + 0x30) != 0
}

/// Calls the device's interface method at `slot` (device, register, data,
/// count) and answers whether its result is not negative.
fn device_call(
    e: &mut Engine,
    device: u32,
    slot: u32,
    register: u32,
    data: u32,
    count: u32,
) -> bool {
    let vtable = e.mem.u32(device);
    let method = e.mem.u32(vtable.wrapping_add(slot));
    e.call(method, &[device, register, data, count]).i32() >= 0
}

// Translated from 00e89b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiD3DVertexShader` (unnamed in the engine map; its
/// vtable `010ef87c` is the class's): sets the vtable, runs `00e93790`
/// on the shader, has the renderer (+0x24) purge it
/// (`NiXenonRenderer::PurgeVertexShader`) when its +0x34 is set, and ends
/// in the base destructor `00e89840`.
pub fn fn_00e89b70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VERTEX_SHADER_VTABLE);
    e.call(FACTORY_RELEASE_00E93790, &[base]);
    if e.mem.u32(base + 0x34) != 0 {
        let renderer = e.mem.u32(base + 0x24);
        e.call(RENDERER_PURGE_VERTEX_SHADER, &[renderer, base]);
    }
    e.call(SHADER_PROGRAM_DESTRUCTOR, &[base]);
}

// Translated from 00e89ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetShaderConstant` (Xbox PDB; thiscall, three
/// words): `data` defaults to the entry's buffer (+0x30) and `count` to
/// its +0x20 (or 1 when that is 0). An entry with its own buffer gets the
/// data copied into it (`_memcpy`, size +0x28) and answers true; otherwise
/// the device's `SetVertexShaderConstantB` (kind 1), `...I` (kind 3) or
/// `...F` (any other kind) gets the entry's register (+0x1C), the data and
/// the count, and the answer is whether the result is not negative.
pub fn ni_d3d_vertex_shader_set_shader_constant(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    data: u32,
    count: u32,
) -> bool {
    let entry = entry.addr();
    let data = if data == 0 {
        e.mem.u32(entry + 0x30)
    } else {
        data
    };
    let mut count = count;
    if count == 0 {
        count = e.mem.u32(entry + 0x20);
        if count == 0 {
            count = 1;
        }
    }
    let kind = constant_kind(e, entry);
    if has_buffer(e, entry) {
        let buffer = e.mem.u32(entry + 0x30);
        let size = e.mem.u32(entry + 0x28);
        e.call(MEMCPY, &[buffer, data, size]);
        return true;
    }
    let register = e.mem.u32(entry + 0x1c);
    let device = device(e, this);
    let slot = match kind {
        1 => DEVICE_SET_CONSTANT_B,
        3 => DEVICE_SET_CONSTANT_I,
        _ => DEVICE_SET_CONSTANT_F,
    };
    device_call(e, device, slot, register, data, count)
}

// Translated from 00e89c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetShaderConstantArray` (Xbox PDB; thiscall, five
/// words): only for entries of kind 8 or 9 (otherwise answers false). For
/// each of the `count` 16-bit indices, the source is `data + index *
/// registers * 16`: an entry with its own buffer gets it copied to
/// `buffer + i * registers * 16` (`_memcpy`, size +0x28), otherwise the
/// device's `SetVertexShaderConstantF` gets the register (+0x1C, advanced
/// by `registers` each time), the source and `registers`. Answers true.
pub fn ni_d3d_vertex_shader_set_shader_constant_array(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    data: u32,
    count: u32,
    registers: u32,
    indices: u32,
) -> bool {
    let entry = entry.addr();
    let kind = constant_kind(e, entry);
    if kind != 8 && kind != 9 {
        return false;
    }
    let mut register = e.mem.u32(entry + 0x1c);
    let mut i = 0u32;
    if i < count {
        let stride = registers << 4;
        let mut offset = 0u32;
        loop {
            let index = u32::from(e.mem.u16(indices.wrapping_add(i * 2)));
            let source = (index.wrapping_mul(registers) << 4).wrapping_add(data);
            if has_buffer(e, entry) {
                let buffer = e.mem.u32(entry + 0x30);
                let size = e.mem.u32(entry + 0x28);
                e.call(MEMCPY, &[offset.wrapping_add(buffer), source, size]);
            } else {
                let device = device(e, this);
                device_call(
                    e,
                    device,
                    DEVICE_SET_CONSTANT_F,
                    register,
                    source,
                    registers,
                );
            }
            offset = offset.wrapping_add(stride);
            i += 1;
            register = register.wrapping_add(registers);
            if i >= count {
                break;
            }
        }
    }
    true
}

/// `NiD3DShaderProgramFactory::RecreateVertexShader` (`00e92ee0`, engine map; `cdecl`, one
/// word: the shader).
const FACTORY_RECREATE_VERTEX_SHADER: u32 = 0x00e9_2ee0;

// Translated from 00e89ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::DestroyRendererData` (Xbox PDB, engine map), `thiscall`, no stack
/// words: when the shader has a handle (+0x34), has its renderer (+0x24) purge it
/// (`NiXenonRenderer::PurgeVertexShader` with the shader).
pub fn ni_d3d_vertex_shader_destroy_renderer_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) != 0 {
        let renderer = e.mem.u32(base + 0x24);
        e.call(RENDERER_PURGE_VERTEX_SHADER, &[renderer, base]);
    }
}

// Translated from 00e89ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::RecreateRendererData` (Xbox PDB, engine map), `thiscall`, no stack
/// words: answers what `NiD3DShaderProgramFactory::RecreateVertexShader` (`00e92ee0`) answers
/// for the shader (`EAX` as it comes back).
pub fn ni_d3d_vertex_shader_recreate_renderer_data(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FACTORY_RECREATE_VERTEX_SHADER, &[this.addr()]).u32()
}

/// The renderer's device (+0x10f8) method at `slot` with (device, register, data, count):
/// whether its result is not negative.
fn renderer_device_call(
    e: &mut Engine,
    renderer: Ptr,
    slot: u32,
    register: u32,
    data: u32,
    count: u32,
) -> bool {
    let device = e.mem.u32(renderer.addr() + RENDERER_DEVICE);
    device_call(e, device, slot, register, data, count)
}

// Translated from 00e89ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, four words, the fourth unread):
/// the renderer's device (+0x10f8) `SetVertexShaderConstantB` (slot +0x188) with the
/// register, data and count; answers whether the result is not negative (`SETGE` into `AL`).
pub fn fn_00e89ae0(
    e: &mut Engine,
    renderer: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    renderer_device_call(e, renderer, DEVICE_SET_CONSTANT_B, register, data, count)
}

// Translated from 00e89b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, four words, the fourth unread): as
/// `00e89ae0` with `SetVertexShaderConstantF` (slot +0x178).
pub fn fn_00e89b10(
    e: &mut Engine,
    renderer: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    renderer_device_call(e, renderer, DEVICE_SET_CONSTANT_F, register, data, count)
}

// Translated from 00e89b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, four words, the fourth unread): as
/// `00e89ae0` with `SetVertexShaderConstantI` (slot +0x180).
pub fn fn_00e89b40(
    e: &mut Engine,
    renderer: Ptr,
    register: u32,
    data: u32,
    count: u32,
    _unused_4: u32,
) -> bool {
    renderer_device_call(e, renderer, DEVICE_SET_CONSTANT_I, register, data, count)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_9ac0, ni_d3d_vertex_shader_destroy_renderer_data(Ptr)),
        entry!(
            0x00e8_9ad0,
            ni_d3d_vertex_shader_recreate_renderer_data(Ptr) -> u32
        ),
        entry!(0x00e8_9ae0, fn_00e89ae0(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00e8_9b10, fn_00e89b10(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00e8_9b40, fn_00e89b40(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00e8_9b70, fn_00e89b70(Ptr)),
        entry!(
            0x00e8_9ba0,
            ni_d3d_vertex_shader_set_shader_constant(Ptr, Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e8_9c70,
            ni_d3d_vertex_shader_set_shader_constant_array(Ptr, Ptr, u32, u32, u32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_F: u32 = 0x0066_0178;
    const SET_I: u32 = 0x0066_0180;
    const SET_B: u32 = 0x0066_0188;

    /// An engine with a device whose three constant setters are doubles
    /// (`SET_I` fails), a shader using it, and an entry of kind `kind`.
    fn engine(kind: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.register(SET_F, |_, _| 0u32.into_ret());
        e.register(SET_I, |_, _| 0x8000_0005u32.into_ret());
        e.register(SET_B, |_, _| 1u32.into_ret());
        e.register(MEMCPY, |_, a| a[0].into_ret());
        let vtable = e.mem.alloc(0x190);
        e.mem.set_u32(vtable + DEVICE_SET_CONSTANT_F, SET_F);
        e.mem.set_u32(vtable + DEVICE_SET_CONSTANT_I, SET_I);
        e.mem.set_u32(vtable + DEVICE_SET_CONSTANT_B, SET_B);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let renderer = e.mem.alloc(0x1100);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let shader = e.mem.alloc(0x38);
        e.mem.set_u32(shader + 0x28, renderer);
        let entry = e.mem.alloc(0x34);
        e.mem.set_u32(entry + 0x14, 0x0000_0005);
        e.mem.set_u32(CONSTANT_KINDS + 5 * 4, kind);
        e.mem.set_u32(entry + 0x1c, 12);
        e.mem.set_u32(entry + 0x28, 0x40);
        e.call_log = Some(vec![]);
        (e, shader, entry)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn device_of(e: &Engine, shader: u32) -> u32 {
        device(e, Ptr::new(shader))
    }

    #[test]
    fn destructor_purges_only_a_live_shader() {
        let mut e = Engine::new();
        for addr in [
            FACTORY_RELEASE_00E93790,
            RENDERER_PURGE_VERTEX_SHADER,
            SHADER_PROGRAM_DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x24, 0x7000);
        e.mem.set_u32(this + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_9b70, &[this]);
        assert_eq!(e.mem.u32(this), VERTEX_SHADER_VTABLE);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (FACTORY_RELEASE_00E93790, vec![this]),
                (RENDERER_PURGE_VERTEX_SHADER, vec![0x7000, this]),
                (SHADER_PROGRAM_DESTRUCTOR, vec![this]),
            ]
        );
        e.mem.set_u32(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_9b70, &[this]);
        assert!(calls_to(&e, RENDERER_PURGE_VERTEX_SHADER).is_empty());
    }

    #[test]
    fn set_constant_picks_the_device_method_by_kind() {
        let (mut e, shader, entry) = engine(0);
        e.mem.set_u32(entry + 0x30, 0x5550);
        e.mem.set_u32(entry + 0x20, 0);
        // Defaults: the entry's buffer and a count of 1.
        assert!(e.call(0x00e8_9ba0, &[shader, entry, 0, 0]).bool());
        let device = device_of(&e, shader);
        assert_eq!(calls_to(&e, SET_F), vec![vec![device, 12, 0x5550, 1]]);
        e.mem.set_u32(CONSTANT_KINDS + 5 * 4, 3);
        e.mem.set_u32(entry + 0x20, 4);
        assert!(!e.call(0x00e8_9ba0, &[shader, entry, 0x6000, 0]).bool());
        assert_eq!(calls_to(&e, SET_I), vec![vec![device, 12, 0x6000, 4]]);
        e.mem.set_u32(CONSTANT_KINDS + 5 * 4, 1);
        assert!(e.call(0x00e8_9ba0, &[shader, entry, 0x6000, 2]).bool());
        assert_eq!(calls_to(&e, SET_B), vec![vec![device, 12, 0x6000, 2]]);
        assert!(calls_to(&e, MEMCPY).is_empty());
    }

    #[test]
    fn set_constant_copies_into_its_own_buffer() {
        let (mut e, shader, entry) = engine(1);
        e.mem.set_u32(entry + 0x14, 0x2000_0005);
        e.mem.set_u32(entry + 0x30, 0x5550);
        assert!(e.call(0x00e8_9ba0, &[shader, entry, 0x6000, 0]).bool());
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x5550, 0x6000, 0x40]]);
        assert!(calls_to(&e, SET_B).is_empty());
    }

    #[test]
    fn set_constant_array_walks_the_indices() {
        let (mut e, shader, entry) = engine(7);
        let indices = e.mem.alloc(6);
        e.mem.set_u16(indices, 2);
        e.mem.set_u16(indices + 2, 0);
        e.mem.set_u16(indices + 4, 1);
        // Kind 7: refused.
        assert!(!e
            .call(0x00e8_9c70, &[shader, entry, 0x8000, 3, 2, indices])
            .bool());
        assert!(calls_to(&e, SET_F).is_empty());
        e.mem.set_u32(CONSTANT_KINDS + 5 * 4, 8);
        assert!(e
            .call(0x00e8_9c70, &[shader, entry, 0x8000, 3, 2, indices])
            .bool());
        let device = device_of(&e, shader);
        assert_eq!(
            calls_to(&e, SET_F),
            vec![
                vec![device, 12, 0x8040, 2],
                vec![device, 14, 0x8000, 2],
                vec![device, 16, 0x8020, 2],
            ]
        );
        // No indices: nothing, still true.
        assert!(e
            .call(0x00e8_9c70, &[shader, entry, 0x8000, 0, 2, indices])
            .bool());
        assert_eq!(calls_to(&e, SET_F).len(), 3);
    }

    #[test]
    fn set_constant_array_copies_into_its_own_buffer() {
        let (mut e, shader, entry) = engine(9);
        e.mem.set_u32(entry + 0x14, 0x2000_0005);
        e.mem.set_u32(entry + 0x30, 0x5000);
        let indices = e.mem.alloc(4);
        e.mem.set_u16(indices, 3);
        e.mem.set_u16(indices + 2, 1);
        assert!(e
            .call(0x00e8_9c70, &[shader, entry, 0x8000, 2, 1, indices])
            .bool());
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x5000, 0x8030, 0x40], vec![0x5010, 0x8010, 0x40]]
        );
    }

    #[test]
    fn destroy_renderer_data_purges_a_live_shader() {
        let mut e = Engine::new();
        e.register(RENDERER_PURGE_VERTEX_SHADER, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x24, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x00e8_9ac0, &[this]);
        assert!(calls_to(&e, RENDERER_PURGE_VERTEX_SHADER).is_empty());
        e.mem.set_u32(this + 0x34, 9);
        e.call(0x00e8_9ac0, &[this]);
        assert_eq!(
            calls_to(&e, RENDERER_PURGE_VERTEX_SHADER),
            vec![vec![0x7000, this]]
        );
    }

    #[test]
    fn recreate_renderer_data_answers_the_factory() {
        let mut e = Engine::new();
        e.register(FACTORY_RECREATE_VERTEX_SHADER, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_9ad0, &[this]).u32(), this + 1);
        assert_eq!(
            calls_to(&e, FACTORY_RECREATE_VERTEX_SHADER),
            vec![vec![this]]
        );
    }

    /// Calls `address` on the renderer of `engine(0)` and checks the device method got
    /// (device, register, data, count) and the answer matches its sign.
    fn renderer_constant(address: u32, method: u32, answer: bool) {
        let (mut e, shader, _) = engine(0);
        let renderer = e.mem.u32(shader + 0x28);
        let device = device_of(&e, shader);
        let words = [renderer, 4, 0x6000, 2, 0xdead];
        assert_eq!(e.call(address, &words).bool(), answer);
        assert_eq!(calls_to(&e, method), vec![vec![device, 4, 0x6000, 2]]);
    }

    #[test]
    fn renderer_set_constant_b_calls_slot_0x188() {
        renderer_constant(0x00e8_9ae0, SET_B, true);
    }

    #[test]
    fn renderer_set_constant_f_calls_slot_0x178() {
        renderer_constant(0x00e8_9b10, SET_F, true);
    }

    #[test]
    fn renderer_set_constant_i_calls_slot_0x180() {
        renderer_constant(0x00e8_9b40, SET_I, false);
    }
}
