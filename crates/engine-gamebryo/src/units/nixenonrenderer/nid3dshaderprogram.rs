//! `NiXenonRenderer/nid3dshaderprogram.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRefObject`'s vtable (`0101dce4`), its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`, raised through the imported `InterlockedIncrement`), `NiD3DShaderProgram`'s
/// vtable (`010ef73c`, RTTI `.?AVNiD3DShaderProgram@@`) and
/// `NiD3DShaderProgram::SetD3DRenderer` (`00e89760`, `thiscall`, one word).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const SHADER_PROGRAM_VTABLE_010EF73C: u32 = 0x010e_f73c;
const SET_D3D_RENDERER_00E89760: u32 = 0x00e8_9760;

// Translated from 00e897d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::NiD3DShaderProgram` (engine map), `thiscall` with two stack words: the
/// renderer and the program type. The `NiRefObject` part (vtable `0101dce4`, count +4 cleared,
/// `ms_uiObjects` at `011f4400` raised), the program type at +8, the vtable `010ef73c`, the words
/// +0xc to +0x28 cleared, then `SetD3DRenderer` with the renderer. Answers `this`.
pub fn ni_d3d_shader_program_ni_d3d_shader_program(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    program_type: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    e.mem.set_u32(base + 8, program_type);
    e.mem.set_u32(base, SHADER_PROGRAM_VTABLE_010EF73C);
    for offset in (0xcu32..=0x28).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_D3D_RENDERER_00E89760, &args![base, renderer]);
    this
}

// Translated from 00e89760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::SetD3DRenderer` (engine map), `thiscall` with one stack word (the
/// renderer), stored at +0x24. With a renderer: the old device at +0x20 is released (its
/// vtable slot +8, `stdcall`), the renderer's device (+0x288) is kept there and referenced
/// (slot +4) when not null, and +0x28 takes the renderer's +0x8b8. Without one: +0x28 is
/// cleared, the old device released and +0x20 cleared.
pub fn ni_d3d_shader_program_set_d3d_renderer(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 0x24, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer + 0x288);
        let old = e.mem.u32(base + 0x20);
        if old != 0 {
            e.vcall(old, 8, &args![]);
        }
        e.mem.set_u32(base + 0x20, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(base + 0x24);
        let state = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(base + 0x28, state);
        return;
    }
    let old = e.mem.u32(base + 0x20);
    e.mem.set_u32(base + 0x28, 0);
    if old != 0 {
        e.vcall(old, 8, &args![]);
    }
    e.mem.set_u32(base + 0x20, 0);
}

/// Imported `InterlockedDecrement` (import slot) and the Ni free `00aa10f0` (unnamed, `cdecl`,
/// one word).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00e89840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderprogram.cpp`): `NiD3DShaderProgram`'s destructor
/// body, `thiscall`, no stack words. The vtable `010ef73c`; the blocks at +0xc, +0x10 and +0x18
/// freed (`00aa10f0`); +0x24 and +0x28 cleared; the device at +0x20 released (its vtable slot
/// +8) and cleared; then the `NiRefObject` vtable `0101dce4` and `ms_uiObjects` (`011f4400`)
/// lowered.
pub fn fn_00e89840(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let first = e.mem.u32(base + 0xc);
    e.mem.set_u32(base, SHADER_PROGRAM_VTABLE_010EF73C);
    e.call(NI_FREE_00AA10F0, &args![first]);
    for offset in [0x10u32, 0x18] {
        let block = e.mem.u32(base + offset);
        e.call(NI_FREE_00AA10F0, &args![block]);
    }
    let device = e.mem.u32(base + 0x20);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0);
    if device != 0 {
        e.vcall(device, 8, &args![]);
    }
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.call(
        IMPORT_INTERLOCKED_DECREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
}

/// The RTTI record `NiD3DShaderProgram::GetRTTI` answers (`0126f9a4`).
const SHADER_PROGRAM_RTTI_0126F9A4: u32 = 0x0126_f9a4;
/// The sized free `00aa1460` (`cdecl`: block, size).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00a8f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::SetShaderConstantArray` (engine map; `thiscall`,
/// `RET 0x14`, none of the five words read): answers false.
pub fn ni_d3d_shader_program_set_shader_constant_array(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    false
}

// Translated from 00e89830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram::GetRTTI` (engine map; `ECX` not read): the RTTI
/// record at `0126f9a4`.
pub fn ni_d3d_shader_program_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    SHADER_PROGRAM_RTTI_0126F9A4
}

// Translated from 00e898a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgram` scalar deleting destructor (engine map; `RET 4`):
/// the destructor body (`00e89840`), then with flag bit 0 the 0x2c bytes
/// freed (`00aa1460`). Answers `this`.
pub fn ni_d3d_shader_program_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e89840(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x2cu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_f430,
            ni_d3d_shader_program_set_shader_constant_array(Ptr, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00e8_9830, ni_d3d_shader_program_get_rtti(Ptr) -> u32),
        entry!(
            0x00e8_98a0,
            ni_d3d_shader_program_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_97d0,
            ni_d3d_shader_program_ni_d3d_shader_program(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_9760,
            ni_d3d_shader_program_set_d3d_renderer(Ptr, Ptr)
        ),
        entry!(0x00e8_9840, fn_00e89840(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_program_constructor_sets_up_the_program() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(SET_D3D_RENDERER_00E89760, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        let answer = e
            .call(0x00e8_97d0, &args![this, 0x0e00_0000u32, 1u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), SHADER_PROGRAM_VTABLE_010EF73C);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.bytes(this + 0xc, 0x20), vec![0; 0x20]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (
                    IMPORT_INTERLOCKED_INCREMENT,
                    vec![NI_REF_OBJECT_COUNT_011F4400]
                ),
                (SET_D3D_RENDERER_00E89760, vec![this, 0x0e00_0000]),
            ]
        );
    }
}

#[cfg(test)]
mod set_renderer_tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e00_0000;
    const ADD_REFERENCE: u32 = 0x0e01_0004;
    const RELEASE: u32 = 0x0e01_0008;

    #[test]
    fn set_d3d_renderer_swaps_the_device() {
        let mut e = Engine::new();
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE, RELEASE]);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let old = e.mem.alloc(4);
        let device = e.mem.alloc(4);
        e.mem.set_u32(old, DEVICE_VTABLE);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x0e02_0000);
        let program = e.mem.alloc(0x2c);
        e.mem.set_u32(program + 0x20, old);
        e.call(0x00e8_9760, &args![program, renderer]);
        assert_eq!(e.mem.u32(program + 0x24), renderer);
        assert_eq!(e.mem.u32(program + 0x20), device);
        assert_eq!(e.mem.u32(program + 0x28), 0x0e02_0000);
        e.call(0x00e8_9760, &args![program, 0u32]);
        assert_eq!(e.mem.u32(program + 0x20), 0);
        assert_eq!(e.mem.u32(program + 0x28), 0);
        let log = e.call_log.take().unwrap();
        let released: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == RELEASE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(released, vec![old, device]);
        let referenced: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == ADD_REFERENCE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(referenced, vec![device]);
    }
}

#[cfg(test)]
mod b054_tests {
    use super::*;

    #[test]
    fn fn_00e89840_frees_the_blocks_and_releases_the_device() {
        let mut e = Engine::new();
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(0x0e01_0008, |_, _| Ret::default());
        e.put_vtable(0x0e00_0000, &[0, 0, 0x0e01_0008]);
        e.call_log = Some(vec![]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x0e00_0000);
        let this = e.mem.alloc(0x2c);
        for (offset, word) in [(0xcu32, 1u32), (0x10, 2), (0x18, 3), (0x24, 4), (0x28, 5)] {
            e.mem.set_u32(this + offset, word);
        }
        e.mem.set_u32(this + 0x20, device);
        e.call(0x00e8_9840, &args![this]);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        for offset in [0x20u32, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (NI_FREE_00AA10F0, vec![1]),
                (NI_FREE_00AA10F0, vec![2]),
                (NI_FREE_00AA10F0, vec![3]),
                (0x0e01_0008, vec![device]),
                (
                    IMPORT_INTERLOCKED_DECREMENT,
                    vec![NI_REF_OBJECT_COUNT_011F4400]
                ),
            ]
        );
        // Without a device nothing is released.
        e.call_log = Some(vec![]);
        e.call(0x00e8_9840, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != 0x0e01_0008));
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn set_shader_constant_array_answers_false() {
        let mut e = Engine::new();
        let r = e.call(0x00a8_f430, &args![0x10u32, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(r.u32() & 0xff, 0);
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e8_9830, &args![0x10u32]).u32(), 0x0126_f9a4);
    }

    #[test]
    fn scalar_deleting_destructor_frees_0x2c_bytes_on_bit_0() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
            e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| Ret::default());
            e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
            let this = e.mem.alloc(0x2c);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00e8_98a0, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
            let log = e.call_log.take().unwrap();
            let freed: Vec<_> = log
                .iter()
                .filter(|c| c.0 == NI_FREE_SIZED_00AA1460)
                .map(|c| c.1.clone())
                .collect();
            if flags == 1 {
                assert_eq!(freed, vec![vec![this, 0x2c]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }
}
