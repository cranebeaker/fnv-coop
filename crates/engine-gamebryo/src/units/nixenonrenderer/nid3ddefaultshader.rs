//! `NiXenonRenderer/nid3ddefaultshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00e818a0` (unnamed; `thiscall`, no stack words), the fixed string
/// maker `00a5b690` (unnamed; `cdecl`, a C string; answers the handle), the empty fixed string
/// (`0109b220`), `InterlockedDecrement` (import slot), the shader's name
/// (`"Default Shader From Gamebryo 2.1"` at `010eee94`, read in Ghidra) and
/// `NiD3DDefaultShader`'s vtable (`010eecac`, RTTI `.?AVNiD3DDefaultShader@@`).
const BASE_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const FIXED_STRING_MAKE_00A5B690: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const DEFAULT_NAME_010EEE94: u32 = 0x010e_ee94;
const DEFAULT_SHADER_VTABLE_010EECAC: u32 = 0x010e_ecac;

// Translated from 00e7c6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DDefaultShader::NiD3DDefaultShader` (engine map), `thiscall`, no stack words. Runs the
/// base constructor `00e818a0`, clears the bytes +0x24 and +0x25 and the words +0x28 to +0x38,
/// stores the vtable `010eecac`, clears +0x50 to +0x58, and names the shader: when the name
/// handle at +8 is not the address `010eee94` itself, +8 gets a new fixed string of that text
/// (`00a5b690`) and the old handle, unless it is the empty fixed string, is released
/// (`InterlockedDecrement` on the handle - 8). Sets the byte +0x20 and answers `this`.
pub fn ni_d3d_default_shader_ni_d3d_default_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base + 0x24, 0);
    e.mem.set_u8(base + 0x25, 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base, DEFAULT_SHADER_VTABLE_010EECAC);
    for offset in [0x50u32, 0x54, 0x58] {
        e.mem.set_u32(base + offset, 0);
    }
    let old = e.mem.u32(base + 8);
    if old != DEFAULT_NAME_010EEE94 {
        let name = e
            .call(FIXED_STRING_MAKE_00A5B690, &args![DEFAULT_NAME_010EEE94])
            .u32();
        e.mem.set_u32(base + 8, name);
        if old != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
        }
    }
    e.mem.set_u8(base + 0x20, 1);
    this
}

/// The calls of the shader functions below (all `thiscall`): `NiD3DShaderInterface::Initialize`
/// (`00e80570`, engine map), `00a29680` (no stack words), the base shader's `00e805f0` and
/// `00e81170` (seven words each) and destructor `00e806c0`, `00e8bb70` (three words), and of
/// the passes and stages: `00e804d0` (`cdecl`, the address of the pass pointer; engine map
/// `GeometryDecalShader::LoadStagesAndPasses`, folded code), `00e7f5d0` (no words), `00e7f430`
/// (three words), `00e80190` (the pass's release at count 0), `NiD3DPass::CheckFreeStages`
/// (`00e7f7f0`), `NiD3DTextureStage::CreateNewStage` (`00e7edf0`, `cdecl`, the address of the
/// stage pointer), the stage state store `00e88fc0` (`thiscall` on the stage's +0x10, (state,
/// value, 0)), `NiD3DTextureStage::SetFilterMode` (`00e7def0`), `00e7de90` (one word),
/// `NiTextureTransform::UpdateMatrix` (`00bc5c20`, no words), `NiD3DPass::SetStage`
/// (`00e802b0`, (index, stage)), the stage's release at count 0 (`00e7ecd0`) and `00e7cac0`
/// (seven words).
const SHADER_INTERFACE_INITIALIZE: u32 = 0x00e8_0570;
const AFTER_INITIALIZE: u32 = 0x00a2_9680;
const BASE_SET_UP: u32 = 0x00e8_05f0;
const BASE_RENDER: u32 = 0x00e8_1170;
const BASE_DESTRUCT: u32 = 0x00e8_06c0;
const RENDER_STATE_SET_UP: u32 = 0x00e8_bb70;
const PASS_LOAD: u32 = 0x00e8_04d0;
const PASS_MAKE: u32 = 0x00e7_f5d0;
const PASS_SET_UP: u32 = 0x00e7_f430;
const PASS_RELEASE: u32 = 0x00e8_0190;
const PASS_CHECK_FREE_STAGES: u32 = 0x00e7_f7f0;
const STAGE_CREATE: u32 = 0x00e7_edf0;
const STAGE_STATE_SET: u32 = 0x00e8_8fc0;
const STAGE_SET_FILTER_MODE: u32 = 0x00e7_def0;
const STAGE_SET_CLAMP: u32 = 0x00e7_de90;
const TEXTURE_TRANSFORM_UPDATE: u32 = 0x00bc_5c20;
const PASS_SET_STAGE: u32 = 0x00e8_02b0;
const STAGE_RELEASE: u32 = 0x00e7_ecd0;
const SHADER_STEP_00E7CAC0: u32 = 0x00e7_cac0;
/// The string storage free (`00aa10f0`, `cdecl`) and the sized free (`00aa1460`, `cdecl`,
/// (block, size)).
const STORAGE_FREE: u32 = 0x00aa_10f0;
const SIZED_FREE: u32 = 0x00aa_1460;
/// The pass the stages are added to (`0126f74c`) and the two counts the shader starts from
/// (`011bf558`, `011bf55c`).
const CURRENT_PASS: u32 = 0x0126_f74c;
const FIRST_COUNT: u32 = 0x011b_f558;
const SECOND_COUNT: u32 = 0x011b_f55c;
/// The render state the shader saves and restores (0x89; by its value `D3DRS_LIGHTING`).
const RENDER_STATE_0X89: u32 = 0x89;

/// A texture stage state store on `stage` (`00e88fc0` on its +0x10, (state, value, 0)).
fn stage_state(e: &mut Engine, stage: u32, state: u32, value: u32) {
    let states = e.mem.u32(stage + 0x10);
    e.call(STAGE_STATE_SET, &args![states, state, value, 0u32]);
}

// Translated from 00e7c560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): with the byte +0x10 clear,
/// `NiD3DShaderInterface::Initialize` (false stops: false) and then `00a29680`; answers true.
pub fn fn_00e7c560(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(this.addr() + 0x10) == 0 {
        if !e.call(SHADER_INTERFACE_INITIALIZE, &args![this]).bool() {
            return false;
        }
        e.call(AFTER_INITIALIZE, &args![this]);
    }
    true
}

// Translated from 00e7c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven words, `RET 0x1c`): sets the render state
/// 0x89 to the value kept at +0x4c (the render state object +0x1c, vtable +0x68, (0x89,
/// value, 0, 0)), then the base shader's `00e805f0` with the seven words, whose `EAX` it
/// answers (the exe ends on that call). The slot is `NiShader`'s `+0x74`, which the Xbox PDB
/// names `PostProcessPipeline` (`unsigned int`, seven words).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7c580(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    let t = this.addr();
    let states = e.mem.u32(t + 0x1c);
    let value = e.mem.u32(t + 0x4c);
    e.vcall(states, 0x68, &args![RENDER_STATE_0X89, value, 0u32, 0u32]);
    e.call(BASE_SET_UP, &args![this, a, b, c, d, f, g, h]).u32()
}

// Translated from 00e7c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the flags byte, `RET 4`; `NiD3DDefaultShader`'s
/// deleting destructor, by the vtable `010eecac` it stores): the storage at +0x50 freed
/// (`00aa10f0`), the base destructor (`00e806c0`) and, with flag 1, the 0x5c bytes freed
/// (`00aa1460`). Answers `this`.
pub fn fn_00e7c5d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    let storage = e.mem.u32(t + 0x50);
    e.mem.set_u32(t, DEFAULT_SHADER_VTABLE_010EECAC);
    e.call(STORAGE_FREE, &args![storage]);
    e.call(BASE_DESTRUCT, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x5cu32]);
    }
    this
}

// Translated from 00e7c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`stdcall`, (six floats, stage), `RET 8`): sets the stage's byte
/// +0x5e and its stage state 0x18 to 2 (`00e88fc0` on its +0x10), then stores the six floats
/// through the x87 stack at +0x1c, +0x2c, +0x3c, +0x20, +0x30 and +0x40, and 0.0 at +0x4c,
/// +0x50, +0x54, +0x44, +0x34, +0x24, +0x58, +0x48, +0x38 and +0x28.
pub fn fn_00e7c610(e: &mut Engine, values: Ptr, stage: Ptr) {
    let s = stage.addr();
    let states = e.mem.u32(s + 0x10);
    e.mem.set_u8(s + 0x5e, 1);
    e.call(STAGE_STATE_SET, &args![states, 0x18u32, 2u32, 0u32]);
    let v = values.addr();
    let value = |e: &Engine, i: u32| x87_quiet_bits(e.mem.u32(v + 4 * i));
    for (i, offset) in [0x1cu32, 0x2c, 0x3c].iter().enumerate() {
        let bits = value(e, i as u32);
        e.mem.set_u32(s + offset, bits);
    }
    let zero = x87_store32(0.0);
    e.mem.set_u32(s + 0x4c, zero);
    for (i, offset) in [0x20u32, 0x30, 0x40].iter().enumerate() {
        let bits = value(e, 3 + i as u32);
        e.mem.set_u32(s + offset, bits);
    }
    for offset in [0x50u32, 0x54, 0x44, 0x34, 0x24, 0x58, 0x48, 0x38, 0x28] {
        e.mem.set_u32(s + offset, zero);
    }
}

// Translated from 00e7c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven words, `RET 0x1c`): the render state object
/// (+0x1c) runs vtable +8 with the fourth word, `00e8bb70` on the renderer's (+0x18) +0x8c8
/// with (the fifth word, the fourth word's +0x14, 0), the base shader's `00e81170` with the
/// seven words, and the render state 0x89's value (vtable +0x6c) is kept at +0x4c. Answers
/// what `00e81170` answers.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7c720(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    let t = this.addr();
    let states = e.mem.u32(t + 0x1c);
    e.vcall(states, 8, &args![d]);
    let second = e.mem.u32(d + 0x14);
    let renderer = e.mem.u32(t + 0x18);
    let target = e.mem.u32(renderer + 0x8c8);
    e.call(RENDER_STATE_SET_UP, &args![target, f, second, 0u32]);
    let result = e.call(BASE_RENDER, &args![this, a, b, c, d, f, g, h]).u32();
    let states = e.mem.u32(t + 0x1c);
    let value = e.vcall(states, 0x6c, &args![RENDER_STATE_0X89]).u32();
    e.mem.set_u32(t + 0x4c, value);
    result
}

// Translated from 00e7c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words: two it does not read and two bytes,
/// `RET 0x10`): makes the current pass (`0126f74c`, cleared first; `00e804d0` fills a local
/// pass pointer), gives it a pass object at +0x30 when it has none (`00e7f5d0`) and sets that
/// up (`00e7f430` with 0xa8, 7, 1); stores the two bytes at +0x3c and +0x3d and the counts
/// `011bf558` and `011bf55c` at +0x44 and +0x48, both one less with the first byte set; then
/// the local pass's count (+0x64) goes down, releasing it (`00e80190`) at zero.
pub fn fn_00e7c790(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    first: u32,
    second: u32,
) {
    let t = this.addr();
    e.with_stack(4, |e, local| {
        let pass_at = local.addr();
        e.mem.set_u32(pass_at, 0);
        e.mem.set_u32(CURRENT_PASS, 0);
        e.call(PASS_LOAD, &args![pass_at]);
        let pass = e.mem.u32(pass_at);
        let slot = pass.wrapping_add(0x30);
        e.mem.set_u32(CURRENT_PASS, pass);
        if e.mem.u32(slot) == 0 {
            let made = e.call(PASS_MAKE, &[]).u32();
            e.mem.set_u32(slot, made);
        }
        let object = e.mem.u32(slot);
        e.call(PASS_SET_UP, &args![object, 0xa8u32, 7u32, 1u32]);
        let (first, second) = (first as u8, second as u8);
        e.mem.set_u8(t + 0x3c, first);
        e.mem.set_u8(t + 0x3d, second);
        let count = e.mem.u32(FIRST_COUNT);
        e.mem.set_u32(t + 0x44, count);
        let count = e.mem.u32(SECOND_COUNT);
        e.mem.set_u32(t + 0x48, count);
        if first != 0 {
            let count = e.mem.u32(t + 0x44).wrapping_sub(1);
            e.mem.set_u32(t + 0x44, count);
            let count = e.mem.u32(t + 0x48).wrapping_sub(1);
            e.mem.set_u32(t + 0x48, count);
        }
        let pass = e.mem.u32(pass_at);
        if pass != 0 {
            let count = e.mem.u32(pass + 0x64).wrapping_sub(1);
            e.mem.set_u32(pass + 0x64, count);
            if count == 0 {
                e.call(PASS_RELEASE, &args![pass]);
            }
        }
    });
}

// Translated from 00e7c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DDefaultShader::AddTextureStage` (engine map; `thiscall`, (map, texture coordinate
/// index), `RET 8`): with a current pass (`0126f74c`) that has a free stage
/// (`NiD3DPass::CheckFreeStages(1, map != null)`), a new stage (`CreateNewStage` into the map
/// argument's own stack word) gets its stage states: 0x1c to 1, and with a map holding a
/// texture (+8; stored at the stage's +8) 0xb to the index, then 1 = 2, 2 = 2, 3 = 1, 4 = 3,
/// 5 = 2 and 6 = 1 (0 once while the byte +0x3d is set, which is then cleared), the map's
/// filter mode (+5, low 4 bits) and clamp mode (+4, bits 12-13), and its texture transform
/// (+0xc): none clears the stage's byte +0x5e and sets state 0x18 to 0, else the matrix
/// (`UpdateMatrix`) goes in by `00e7c610`; either way +0x48 goes down. Without a texture:
/// 1 = 2, 2 = 0, 3 = 1, 4 = 3 (2 once while +0x3d is set, then cleared), 5 = 0, 6 = 1 and
/// +0x48 down unless 0. Then +0x44 goes down, the pass takes the stage at its index (+0x14;
/// `SetStage`) and the local stage pointer's count (+0x60) goes down (`00e7ecd0` at zero).
/// Answers the stage, or null.
pub fn ni_d3d_default_shader_add_texture_stage(
    e: &mut Engine,
    this: Ptr,
    map: u32,
    index: u32,
) -> u32 {
    let t = this.addr();
    let pass = e.mem.u32(CURRENT_PASS);
    if pass == 0 {
        return 0;
    }
    if !e
        .call(
            PASS_CHECK_FREE_STAGES,
            &args![pass, 1u32, u32::from(map != 0)],
        )
        .bool()
    {
        return 0;
    }
    e.with_stack(4, |e, local| {
        let stage_at = local.addr();
        e.mem.set_u32(stage_at, 0);
        e.call(STAGE_CREATE, &args![stage_at]);
        let stage = e.mem.u32(stage_at);
        stage_state(e, stage, 0x1c, 1);
        let texture = if map != 0 { e.mem.u32(map + 8) } else { 0 };
        if texture != 0 {
            let states = e.mem.u32(stage + 0x10);
            e.mem.set_u32(stage + 8, texture);
            e.call(STAGE_STATE_SET, &args![states, 0xbu32, index, 0u32]);
            for (state, value) in [(1u32, 2u32), (2, 2), (3, 1), (4, 3), (5, 2)] {
                stage_state(e, stage, state, value);
            }
            if e.mem.u8(t + 0x3d) != 0 {
                stage_state(e, stage, 6, 0);
                e.mem.set_u8(t + 0x3d, 0);
            } else {
                stage_state(e, stage, 6, 1);
            }
            let filter = u32::from(e.mem.u8(map + 5)) & 0xf;
            e.call(STAGE_SET_FILTER_MODE, &args![stage, filter]);
            let clamp = (u32::from(e.mem.u16(map + 4)) >> 12) & 3;
            e.call(STAGE_SET_CLAMP, &args![stage, clamp]);
            let transform = e.mem.u32(map + 0xc);
            if transform == 0 {
                let states = e.mem.u32(stage + 0x10);
                e.mem.set_u8(stage + 0x5e, 0);
                e.call(STAGE_STATE_SET, &args![states, 0x18u32, 0u32, 0u32]);
            } else {
                let matrix = e.call(TEXTURE_TRANSFORM_UPDATE, &args![transform]).u32();
                fn_00e7c610(e, Ptr::new(matrix), Ptr::new(stage));
            }
            let count = e.mem.u32(t + 0x48).wrapping_sub(1);
            e.mem.set_u32(t + 0x48, count);
        } else {
            for (state, value) in [(1u32, 2u32), (2, 0), (3, 1)] {
                stage_state(e, stage, state, value);
            }
            if e.mem.u8(t + 0x3d) != 0 {
                stage_state(e, stage, 4, 2);
                e.mem.set_u8(t + 0x3d, 0);
            } else {
                stage_state(e, stage, 4, 3);
            }
            stage_state(e, stage, 5, 0);
            stage_state(e, stage, 6, 1);
            let count = e.mem.u32(t + 0x48);
            if count != 0 {
                e.mem.set_u32(t + 0x48, count - 1);
            }
        }
        let count = e.mem.u32(t + 0x44).wrapping_sub(1);
        e.mem.set_u32(t + 0x44, count);
        let pass = e.mem.u32(CURRENT_PASS);
        let pass_index = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, pass_index, stage]);
        let held = e.mem.u32(stage_at);
        if held != 0 {
            let count = e.mem.u32(held + 0x60).wrapping_sub(1);
            e.mem.set_u32(held + 0x60, count);
            if count == 0 {
                e.call(STAGE_RELEASE, &args![held]);
            }
        }
        stage
    })
}

// Translated from 00e7cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven words, `RET 0x1c`): the shader's vtable
/// +0xec, then `00e7cac0` with the seven words, whose `EAX` it answers (the exe ends on that
/// call). The slot is `NiShader`'s `+0x58`, which the Xbox PDB names `UpdatePipeline`
/// (`unsigned int`, seven words).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7cc10(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    e.vcall(this.addr(), 0xec, &[]);
    e.call(SHADER_STEP_00E7CAC0, &args![this, a, b, c, d, f, g, h])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_c6b0, ni_d3d_default_shader_ni_d3d_default_shader(Ptr) -> Ptr),
        entry!(0x00e7_c560, fn_00e7c560(Ptr) -> bool),
        entry!(
            0x00e7_c580,
            fn_00e7c580(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e7_c5d0, fn_00e7c5d0(Ptr, u32) -> Ptr),
        entry!(0x00e7_c610, fn_00e7c610(Ptr, Ptr)),
        entry!(0x00e7_c720, fn_00e7c720(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e7_c790, fn_00e7c790(Ptr, u32, u32, u32, u32)),
        entry!(
            0x00e7_c830,
            ni_d3d_default_shader_add_texture_stage(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00e7_cc10,
            fn_00e7cc10(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_EMPTY_0109B220, 0x0e00_0008u32);
        e.register(BASE_CONSTRUCT_00E818A0, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_MAKE_00A5B690, |_, _| 0x0e10_0008u32.into_ret());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn default_shader_constructor_names_the_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.mem.write(this, &[0xcc; 0x5c]);
        e.mem.set_u32(this + 8, 0x0e20_0008);
        assert_eq!(e.call(0x00e7_c6b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), DEFAULT_SHADER_VTABLE_010EECAC);
        assert_eq!(e.mem.u32(this + 8), 0x0e10_0008);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.bytes(this + 0x24, 2), vec![0; 2]);
        assert_eq!(e.mem.bytes(this + 0x28, 0x14), vec![0; 0x14]);
        assert_eq!(e.mem.bytes(this + 0x50, 0xc), vec![0; 0xc]);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE_00A5B690),
            vec![vec![DEFAULT_NAME_010EEE94]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e20_0000]]
        );
    }

    #[test]
    fn default_shader_constructor_keeps_the_empty_string_and_the_name_itself() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 8, 0x0e00_0008);
        e.call(0x00e7_c6b0, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        e.mem.set_u32(this + 8, DEFAULT_NAME_010EEE94);
        e.call(0x00e7_c6b0, &args![this]);
        assert_eq!(e.mem.u32(this + 8), DEFAULT_NAME_010EEE94);
        assert_eq!(calls(&e, FIXED_STRING_MAKE_00A5B690).len(), 1);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn initialize_runs_once_the_interface_succeeds() {
        let mut e = engine();
        e.register(SHADER_INTERFACE_INITIALIZE, |_, _| 0x100u32.into_ret());
        e.register(AFTER_INITIALIZE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        assert!(!e.call(0x00e7_c560, &args![this]).bool());
        assert!(calls(&e, AFTER_INITIALIZE).is_empty());
        e.register(SHADER_INTERFACE_INITIALIZE, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e7_c560, &args![this]).bool());
        assert_eq!(calls(&e, AFTER_INITIALIZE), vec![vec![this]]);
        e.mem.set_u8(this + 0x10, 1);
        assert!(e.call(0x00e7_c560, &args![this]).bool());
        assert_eq!(calls(&e, SHADER_INTERFACE_INITIALIZE).len(), 2);
    }

    #[test]
    fn render_steps_save_and_restore_the_render_state() {
        let mut e = engine();
        let states = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x70);
        e.mem.set_u32(states, vtable);
        for (slot, target) in [
            (8u32, 0x034d_0008u32),
            (0x68, 0x034d_0068),
            (0x6c, 0x034d_006c),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.register(0x034d_0008, |_, _| Ret::default());
        e.register(0x034d_0068, |_, _| Ret::default());
        e.register(0x034d_006c, |_, _| 0x77u32.into_ret());
        e.register(BASE_SET_UP, |_, _| Ret::default());
        e.register(BASE_RENDER, |_, _| 0x55u32.into_ret());
        e.register(RENDER_STATE_SET_UP, |_, _| Ret::default());
        let this = e.mem.alloc(0x5c);
        let renderer = e.mem.alloc(0x8d0);
        e.mem.set_u32(renderer + 0x8c8, 0x4400);
        e.mem.set_u32(this + 0x18, renderer);
        e.mem.set_u32(this + 0x1c, states);
        let geometry = e.mem.alloc(0x18);
        e.mem.set_u32(geometry + 0x14, 0x4500);
        let words = args![1u32, 2u32, 3u32, geometry, 5u32, 6u32, 7u32];
        let mut with_this = vec![this];
        with_this.extend(&words);
        assert_eq!(e.call(0x00e7_c720, &with_this).u32(), 0x55);
        assert_eq!(e.mem.u32(this + 0x4c), 0x77);
        assert_eq!(calls(&e, 0x034d_0008), vec![vec![states, geometry]]);
        assert_eq!(
            calls(&e, RENDER_STATE_SET_UP),
            vec![vec![0x4400, 5, 0x4500, 0]]
        );
        assert_eq!(calls(&e, BASE_RENDER), vec![with_this.clone()]);
        assert_eq!(calls(&e, 0x034d_006c), vec![vec![states, 0x89]]);
        e.call(0x00e7_c580, &with_this);
        assert_eq!(calls(&e, 0x034d_0068), vec![vec![states, 0x89, 0x77, 0, 0]]);
        assert_eq!(calls(&e, BASE_SET_UP), vec![with_this.clone()]);
        // 00e7cc10: vtable +0xec, then 00e7cac0.
        let own = e.mem.alloc(0x100);
        e.mem.set_u32(own + 0xec, 0x034d_00ec);
        e.mem.set_u32(this, own);
        e.register(0x034d_00ec, |_, _| Ret::default());
        e.register(SHADER_STEP_00E7CAC0, |_, _| Ret::default());
        e.call(0x00e7_cc10, &with_this);
        assert_eq!(calls(&e, 0x034d_00ec), vec![vec![this]]);
        assert_eq!(calls(&e, SHADER_STEP_00E7CAC0), vec![with_this]);
    }

    #[test]
    fn pipeline_steps_answer_the_base_eax() {
        let mut e = engine();
        let states = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x70);
        e.mem.set_u32(states, vtable);
        e.mem.set_u32(vtable + 0x68, 0x034d_0068);
        e.register(0x034d_0068, |_, _| Ret::default());
        e.register(BASE_SET_UP, |_, _| 3u32.into_ret());
        e.register(SHADER_STEP_00E7CAC0, |_, _| 4u32.into_ret());
        let own = e.mem.alloc(0x100);
        e.mem.set_u32(own + 0xec, 0x034d_00ec);
        e.register(0x034d_00ec, |_, _| 9u32.into_ret());
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this, own);
        e.mem.set_u32(this + 0x1c, states);
        let with_this = vec![this, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(e.call(0x00e7_c580, &with_this).u32(), 3);
        assert_eq!(e.call(0x00e7_cc10, &with_this).u32(), 4);
    }

    #[test]
    fn deleting_destructor_frees_the_storage() {
        let mut e = engine();
        for address in [STORAGE_FREE, BASE_DESTRUCT, SIZED_FREE] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x50, 0x6600);
        assert_eq!(e.call(0x00e7_c5d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), DEFAULT_SHADER_VTABLE_010EECAC);
        assert!(calls(&e, SIZED_FREE).is_empty());
        e.call(0x00e7_c5d0, &args![this, 1u32]);
        assert_eq!(calls(&e, STORAGE_FREE), vec![vec![0x6600]; 2]);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]; 2]);
        assert_eq!(calls(&e, SIZED_FREE), vec![vec![this, 0x5c]]);
    }

    #[test]
    fn texture_matrix_goes_into_the_stage() {
        let mut e = engine();
        e.register(STAGE_STATE_SET, |_, _| Ret::default());
        let stage = e.mem.alloc(0x60);
        e.mem.write(stage + 0x1c, &[0xff; 0x40]);
        e.mem.set_u32(stage + 0x10, 0x4700);
        let values = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_f32(values + 4 * i, 1.0 + i as f32);
        }
        e.mem.set_u32(values, 0x7f80_0001);
        e.call(0x00e7_c610, &args![values, stage]);
        assert_eq!(e.mem.u8(stage + 0x5e), 1);
        assert_eq!(calls(&e, STAGE_STATE_SET), vec![vec![0x4700, 0x18, 2, 0]]);
        assert_eq!(e.mem.u32(stage + 0x1c), 0x7fc0_0001);
        let at = |e: &Engine, offset: u32| e.mem.f32(stage + offset);
        assert_eq!(
            [
                at(&e, 0x2c),
                at(&e, 0x3c),
                at(&e, 0x20),
                at(&e, 0x30),
                at(&e, 0x40)
            ],
            [2.0, 3.0, 4.0, 5.0, 6.0]
        );
        for offset in [
            0x24u32, 0x28, 0x34, 0x38, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58,
        ] {
            assert_eq!(e.mem.u32(stage + offset), 0, "{offset:#x}");
        }
    }

    #[test]
    fn pass_set_up_stores_bytes_and_counts() {
        let mut e = engine();
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, 2);
        e.register_double(PASS_LOAD, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(PASS_MAKE, |_, _| 0x4800u32.into_ret());
        e.register(PASS_SET_UP, |_, _| Ret::default());
        e.register(PASS_RELEASE, |_, _| Ret::default());
        e.mem.set_u32(FIRST_COUNT, 8);
        e.mem.set_u32(SECOND_COUNT, 4);
        let this = e.mem.alloc(0x5c);
        e.call(0x00e7_c790, &args![this, 0u32, 0u32, 0x101u32, 0x200u32]);
        assert_eq!(e.mem.u32(CURRENT_PASS), pass);
        assert_eq!(e.mem.u32(pass + 0x30), 0x4800);
        assert_eq!(calls(&e, PASS_SET_UP), vec![vec![0x4800, 0xa8, 7, 1]]);
        assert_eq!((e.mem.u8(this + 0x3c), e.mem.u8(this + 0x3d)), (1, 0));
        assert_eq!((e.mem.u32(this + 0x44), e.mem.u32(this + 0x48)), (7, 3));
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        // Second byte clear first: the counts as they are; the pass's last count releases it.
        e.call(0x00e7_c790, &args![this, 0u32, 0u32, 0u32, 1u32]);
        assert_eq!((e.mem.u32(this + 0x44), e.mem.u32(this + 0x48)), (8, 4));
        assert_eq!(calls(&e, PASS_MAKE).len(), 1);
        assert_eq!(calls(&e, PASS_RELEASE), vec![vec![pass]]);
    }

    /// An engine set up for `AddTextureStage`: a current pass with free stages, a new stage
    /// with a count of 1 from `CreateNewStage`.
    fn stage_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let pass = e.mem.alloc(0x18);
        e.mem.set_u32(pass + 0x14, 3);
        e.mem.set_u32(CURRENT_PASS, pass);
        e.register(PASS_CHECK_FREE_STAGES, |_, _| 1u32.into_ret());
        let stage = e.mem.alloc(0x64);
        e.mem.set_u32(stage + 0x10, 0x4900);
        e.register_double(STAGE_CREATE, move |e, a| {
            e.mem.set_u32(a[0], stage);
            e.mem.set_u32(stage + 0x60, 2);
            Ret::default()
        });
        for address in [
            STAGE_STATE_SET,
            STAGE_SET_FILTER_MODE,
            STAGE_SET_CLAMP,
            PASS_SET_STAGE,
            STAGE_RELEASE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        (e, pass, stage)
    }

    fn states(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, STAGE_STATE_SET)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect()
    }

    #[test]
    fn add_texture_stage_without_a_texture() {
        let (mut e, pass, stage) = stage_engine();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u8(this + 0x3d, 1);
        e.mem.set_u32(this + 0x44, 5);
        e.mem.set_u32(this + 0x48, 0);
        let r = e.call(0x00e7_c830, &args![this, 0u32, 9u32]).u32();
        assert_eq!(r, stage);
        assert_eq!(calls(&e, PASS_CHECK_FREE_STAGES), vec![vec![pass, 1, 0]]);
        assert_eq!(
            states(&e),
            vec![(0x1c, 1), (1, 2), (2, 0), (3, 1), (4, 2), (5, 0), (6, 1)]
        );
        assert_eq!(e.mem.u8(this + 0x3d), 0);
        assert_eq!((e.mem.u32(this + 0x44), e.mem.u32(this + 0x48)), (4, 0));
        assert_eq!(calls(&e, PASS_SET_STAGE), vec![vec![pass, 3, stage]]);
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        // No current pass, or no free stage: null.
        e.mem.set_u32(CURRENT_PASS, 0);
        assert_eq!(e.call(0x00e7_c830, &args![this, 0u32, 9u32]).u32(), 0);
        e.mem.set_u32(CURRENT_PASS, pass);
        e.register(PASS_CHECK_FREE_STAGES, |_, _| 0x100u32.into_ret());
        assert_eq!(e.call(0x00e7_c830, &args![this, 0u32, 9u32]).u32(), 0);
    }

    #[test]
    fn add_texture_stage_with_a_texture_and_transform() {
        let (mut e, _, stage) = stage_engine();
        e.register(TEXTURE_TRANSFORM_UPDATE, |_, _| 0x4a00u32.into_ret());
        e.map(0x4a00, 0x100);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x48, 2);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4b00);
        e.mem.set_u16(map + 4, 0x2135);
        e.mem.set_u32(map + 0xc, 0x4c00);
        e.call(0x00e7_c830, &args![this, map, 9u32]);
        assert_eq!(e.mem.u32(stage + 8), 0x4b00);
        assert_eq!(
            states(&e),
            vec![
                (0x1c, 1),
                (0xb, 9),
                (1, 2),
                (2, 2),
                (3, 1),
                (4, 3),
                (5, 2),
                (6, 1),
                (0x18, 2)
            ]
        );
        assert_eq!(calls(&e, STAGE_SET_FILTER_MODE), vec![vec![stage, 1]]);
        assert_eq!(calls(&e, STAGE_SET_CLAMP), vec![vec![stage, 2]]);
        assert_eq!(calls(&e, TEXTURE_TRANSFORM_UPDATE), vec![vec![0x4c00]]);
        assert_eq!(e.mem.u8(stage + 0x5e), 1);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        // No transform: state 0x18 to 0 and the byte cleared; the last count releases.
        e.mem.set_u32(map + 0xc, 0);
        e.register_double(STAGE_CREATE, move |e, a| {
            e.mem.set_u32(a[0], stage);
            e.mem.set_u32(stage + 0x60, 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_c830, &args![this, map, 9u32]);
        assert_eq!(states(&e).last(), Some(&(0x18, 0)));
        assert_eq!(e.mem.u8(stage + 0x5e), 0);
        assert_eq!(calls(&e, STAGE_RELEASE), vec![vec![stage]]);
    }
}
