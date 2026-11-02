//! `BSShader/run_00c04030` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010bca64`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BCA64: u32 = 0x010b_ca64;

// Translated from 00c04600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, the vtable `010bca64`, +0x58
/// cleared. Answers `this`.
pub fn fn_00c04600(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BCA64);
    e.mem.set_u32(base + 0x58, 0);
    this
}

/// `BSShader::BSShader` (`00be2200`, `thiscall`), the image-space effect base constructor
/// `00ba4080` (`thiscall`, on the effect part at +0x6c), and the two set-ups the constructor
/// ends with: `00be0870` (unnamed; `thiscall`, no stack words) and `00c04030` (this unit,
/// unnamed; `thiscall`, no stack words).
const BS_SHADER_CONSTRUCT_00BE2200: u32 = 0x00be_2200;
const SET_UP_00BE0870: u32 = 0x00be_0870;
const SET_UP_00C04030: u32 = 0x00c0_4030;
/// The image-space shader's vtable (`010bc8f8`) and its effect part's (`010b0318`) (no RTTI
/// locators before them).
const SHADER_VTABLE_010BC8F8: u32 = 0x010b_c8f8;
const EFFECT_PART_VTABLE_010B0318: u32 = 0x010b_0318;

// Translated from 00c04570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): the image-space shader base constructor,
/// `thiscall`, no stack words. Runs `BSShader::BSShader` (`00be2200`) and the image-space
/// effect base constructor `00ba4080` on +0x6c, stores the vtables `010bc8f8` at +0 and
/// `010b0318` at +0x6c, clears +0xc4 and +0xc8, sets +0x68 to 0x18 and runs `00be0870` and
/// `00c04030` on the shader. Answers `this`.
// Translation incomplete: 00c04570 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the two bases) is not translated.
pub fn fn_00c04570(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT_00BE2200, &args![base]);
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base + 0x6c]);
    e.mem.set_u32(base, SHADER_VTABLE_010BC8F8);
    e.mem.set_u32(base + 0x6c, EFFECT_PART_VTABLE_010B0318);
    e.mem.set_u32(base + 0xc4, 0);
    e.mem.set_u32(base + 0xc8, 0);
    e.mem.set_u32(base + 0x68, 0x18);
    e.call(SET_UP_00BE0870, &args![base]);
    e.call(SET_UP_00C04030, &args![base]);
    this
}

/// The renderer (`011f9508`) and `NiXenonShaderDeclaration::Create` (engine map, `00e76700`,
/// `cdecl`: renderer, entry count, stream count).
const RENDERER_011F9508: u32 = 0x011f_9508;
const DECLARATION_CREATE_00E76700: u32 = 0x00e7_6700;

// Translated from 00c04030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): the image-space shader's declaration set-up,
/// `thiscall`, no stack words. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer, 2 entries, 1 stream), sets its entries through its vtable slot +0x8c ((0, 0, 0, 2,
/// 0) and (1, 7, 5, 1, 0)), calls its slot +0xb4 and hands it to the shader's slot +0xc0. The
/// declaration is used without a null check.
pub fn fn_00c04030(e: &mut Engine, this: Ptr) {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE_00E76700, &args![renderer, 2u32, 1u32])
        .u32();
    e.vcall(declaration, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(declaration, 0x8c, &args![1u32, 7u32, 5u32, 1u32, 0u32]);
    e.vcall(declaration, 0xb4, &args![]);
    e.vcall(this.addr(), 0xc0, &args![declaration]);
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, pass_render_state, release_reference, stage_release, CREATE_NEW_STAGE,
    IMPORT_INTERLOCKED_INCREMENT, MEMORY_MANAGER, NEW_PASS_00E804D0, PASS_SET_STAGE,
    SETUP_DEFAULT_STAGE,
};

/// `NiPointer<T>::operator=` from the pointer at `source`, as these functions expand it:
/// nothing when the slot already holds what `source` holds; else the slot's object is
/// released, `source` read again, stored and its object counted when not null.
fn assign_from(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    let object = e.mem.u32(source);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00c04090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`; Ghidra made no function of it): the image-space
/// shader's parameter set, `thiscall`, one word (an `ImageSpaceShaderParam`): the shader's
/// `NiPointer`s +0x34 and +0x30 take the parameter's `spVertexConstantMap` (+0x4c) and
/// `spPixelConstantMap` (+0x50) (Xbox PDB), and `pParam` (+0xcc) the parameter.
pub fn fn_00c04090(e: &mut Engine, this: Ptr, param: u32) {
    let base = this.addr();
    assign_from(e, base + 0x34, param + 0x4c);
    assign_from(e, base + 0x30, param + 0x50);
    e.mem.set_u32(base + 0xcc, param);
}

/// The pass the image-space shaders set up last (`0126f74c`).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;

// Translated from 00c04120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c04120 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
/// Unnamed in the engine map (`run_00c04030`): the image-space shader's pass set-up,
/// `thiscall`, no stack words. Gives the pass pointer at +0x5c a new pass (`00e804d0`); adds one
/// texture stage per texture (the count, a `u16` at +0x9c, re-read each pass): the stage pointer
/// (one local for all of them, set to null once) filled by `CreateNewStage`,
/// `SetupDefaultStage(stage, index, 3, 2, 1)` and `NiD3DPass::SetStage` with the pass's +0x14;
/// sets the render states 7, 0x0e, 0x1b, 0x0f to 0 and 0xa8 to 0x0f; calls the shader's slot
/// +0x150; gives the pass's `NiPointer`s +0x5c and +0x44 the shader's `spVertexShader` and
/// `spPixelShader`; keeps the pass in `0126f74c`; drops the stage local's reference. Answers
/// true.
pub fn fn_00c04120(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let pass_slot = base + 0x5c;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(NEW_PASS_00E804D0, &args![pass_slot]);
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(base + 0x9c)) {
            e.call(CREATE_NEW_STAGE, &args![local]);
            let stage = e.mem.u32(local);
            e.call(SETUP_DEFAULT_STAGE, &args![stage, index, 3u32, 2u32, 1u32]);
            let pass = e.mem.u32(pass_slot);
            let stage = e.mem.u32(local);
            let position = e.mem.u32(pass + 0x14);
            e.call(PASS_SET_STAGE, &args![pass, position, stage]);
            index += 1;
        }
        for (state, value) in [(7u32, 0u32), (0xe, 0), (0x1b, 0), (0xf, 0), (0xa8, 0xf)] {
            let pass = e.mem.u32(pass_slot);
            pass_render_state(e, pass, state, value);
        }
        e.vcall(base, 0x150, &args![]);
        let pass = e.mem.u32(pass_slot);
        let vertex = e.mem.u32(base + 0xc4);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(pass_slot);
        let pixel = e.mem.u32(base + 0xc8);
        ni_pointer_assign(e, pass + 0x44, pixel);
        let pass = e.mem.u32(pass_slot);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    true
}

/// A texture stage held in a local: when `next` differs from `held`, `held`'s reference is
/// dropped (count at +0x60, deleted through `00e7ecd0` at 0) and `next` counted (plain `INC`)
/// when not null. Answers the stage now held.
fn hold_stage(e: &mut Engine, held: u32, next: u32) -> u32 {
    if held == next {
        return held;
    }
    stage_release(e, held);
    if next != 0 {
        let count = e.mem.u32(next + 0x60).wrapping_add(1);
        e.mem.set_u32(next + 0x60, count);
    }
    next
}

/// `00ba38f0` (unit `imagespacetexture.cpp`, `thiscall` on the texture: the stage) and the
/// stage's `00e7eb00` (unnamed; `thiscall`, one word).
const TEXTURE_TO_STAGE_00BA38F0: u32 = 0x00ba_38f0;
const STAGE_UPDATE_00E7EB00: u32 = 0x00e7_eb00;

// Translated from 00c04310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c04310 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held stage) is not translated.
/// Unnamed in the engine map (`run_00c04030`), `thiscall`, one word it does not read: for each
/// texture index from 1 below the count (`u16` +0x9e, re-read each pass) holds stage
/// `index - 1` of the current pass (`0126f74c`, stage array at +0x24); with a stage, the
/// shader's texture `index` (array at +0x98) sets the stage up (`00ba38f0`), or the stage's +8
/// is cleared when that entry is null, then the stage's `00e7eb00(0)` runs. The last stage
/// held is dropped at the end.
pub fn fn_00c04310(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    let mut held = 0u32;
    let mut index = 1u32;
    while index < u32::from(e.mem.u16(base + 0x9e)) {
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let stages = e.mem.u32(pass + 0x24);
        let next = e
            .mem
            .u32(stages.wrapping_add(index.wrapping_mul(4)).wrapping_sub(4));
        held = hold_stage(e, held, next);
        if held != 0 {
            let textures = e.mem.u32(base + 0x98);
            let texture = e.mem.u32(textures.wrapping_add(index.wrapping_mul(4)));
            if texture != 0 {
                e.call(TEXTURE_TO_STAGE_00BA38F0, &args![texture, held]);
            } else {
                e.mem.set_u32(held + 8, 0);
            }
            e.call(STAGE_UPDATE_00E7EB00, &args![held, 0u32]);
        }
        index += 1;
    }
    stage_release(e, held);
}

/// The current geometry holder (`011f91e0`: its first word) and the default word `011f4288`
/// used when the geometry's +0x20 is null.
const CURRENT_GEOMETRY_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_011F4288: u32 = 0x011f_4288;

// Translated from 00c043f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`), `thiscall`, one word. Takes the current geometry
/// (the first word of `011f91e0`), its +0x20 (`011f4288` when null) and the +0x34 of its
/// +0xb8; then for the shader's objects at +0x30 and +0x34, when set and the current pass
/// (`0126f74c`) has a shader at +0x44 and +0x5c respectively, calls the object's slot +0x78
/// with that pass shader, the geometry, 0, 0, the +0x34 word, the argument, 0, the geometry's
/// +0x68, the +0x20 word, 0 and 1.
pub fn fn_00c043f0(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let holder = e.global::<u32>(CURRENT_GEOMETRY_011F91E0);
    let geometry = e.mem.u32(holder);
    let mut word = e.mem.u32(geometry + 0x20);
    if word == 0 {
        word = DEFAULT_011F4288;
    }
    let inner = e.mem.u32(e.mem.u32(geometry + 0xb8) + 0x34);
    for (map, shader) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let object = e.mem.u32(base + map);
        if object == 0 {
            continue;
        }
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let pass_shader = e.mem.u32(pass + shader);
        if pass_shader == 0 {
            continue;
        }
        e.vcall(
            object,
            0x78,
            &args![
                pass_shader,
                geometry,
                0u32,
                0u32,
                inner,
                value,
                0u32,
                geometry + 0x68,
                word,
                0u32,
                1u32
            ],
        );
    }
}

/// The pass's device getter `00e7f7c0` (unnamed; `thiscall` on the pass, no stack words).
const PASS_DEVICE_00E7F7C0: u32 = 0x00e7_f7c0;

// Translated from 00c04490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c04490 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held stage) is not translated.
/// Unnamed in the engine map (`run_00c04030`), `thiscall`, one word it does not read: for each
/// texture index from 1 below the count (`u16` +0x9e, re-read each pass) holds stage
/// `index - 1` of the current pass (`0126f74c`, stage array at +0x24); with a stage, its +8 is
/// cleared and the object `00e7f7c0` answers for the pass gets its slot +0xdc called with
/// `index - 1` and 0. The last stage held is dropped at the end.
pub fn fn_00c04490(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    let mut held = 0u32;
    let mut index = 1u32;
    while index < u32::from(e.mem.u16(base + 0x9e)) {
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let stages = e.mem.u32(pass + 0x24);
        let next = e
            .mem
            .u32(stages.wrapping_add(index.wrapping_mul(4)).wrapping_sub(4));
        held = hold_stage(e, held, next);
        if held != 0 {
            e.mem.set_u32(held + 8, 0);
            let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
            let device = e.call(PASS_DEVICE_00E7F7C0, &args![pass]).u32();
            e.vcall(device, 0xdc, &args![index.wrapping_sub(1), 0u32]);
        }
        index += 1;
    }
    stage_release(e, held);
}

/// `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`), `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on the manager, one word), the array setter `00ba3aa0` (`thiscall`
/// on an `NiTPrimitiveArray`: index, the address of the word) and
/// `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`) (engine map).
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

// Translated from 00c04620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c04620 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (`run_00c04030`): the destructor of the effect `00c04600` builds,
/// `thiscall`, no stack words. Stores the vtable `010bca64`; a texture at +0x58 is destroyed
/// (`~ImageSpaceTexture`), freed (`MemoryManager::Deallocate`) and texture 2 of the effect
/// (`pTextures`, +0x28) set to null; a first effect parameter (`pEffectParamList` +0x18, its
/// element 0) is deleted through its slot +0 with 1 and that element set to null; then
/// `ImageSpaceEffect::~ImageSpaceEffect`. (+0x58 is not cleared.)
pub fn fn_00c04620(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BCA64);
    let texture = e.mem.u32(base + 0x58);
    if texture != 0 {
        e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, texture]);
        set_to_null(e, base + 0x28, 2);
    }
    let parameters = e.mem.u32(base + 0x1c);
    if e.mem.u32(parameters) != 0 {
        let parameter = e.mem.u32(parameters);
        // The compiler tests the element again after the comparison above.
        if parameter != 0 {
            e.vcall(parameter, 0, &args![1u32]);
        }
        set_to_null(e, base + 0x18, 0);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// Element `index` of the `NiTPrimitiveArray` at `array` set to null through `00ba3aa0` (the
/// null word in a local).
fn set_to_null(e: &mut Engine, array: u32, index: u32) {
    e.with_stack(4, |e, local| {
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00c04710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): the scalar deleting destructor of the effect
/// `00c04600` builds, `thiscall`, one word: the destructor `00c04620`, then the memory is freed
/// (`MemoryManager::Deallocate`) when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00c04710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c04620(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The shader parameter's `SetPixelConstant` (`00b8a790`, unnamed in the engine map;
/// `thiscall`, five words).
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;

// Translated from 00c046e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bca80`), `thiscall`
/// with one unread word; answers true. The first shader parameter of the array at +0x18
/// (elements at +0x1c) gets pixel constant 0 (the `float` +0x5c through `FLD`/`FSTP`, then
/// 0.0, 0.0, 0.0).
pub fn fn_00c046e0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let parameters = e.mem.u32(base + 0x1c);
    let value = x87_quiet_bits(e.mem.u32(base + 0x5c));
    let parameter = e.mem.u32(parameters);
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![parameter, 0u32, value, 0u32, 0u32, 0u32],
    );
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_46e0, fn_00c046e0(Ptr, u32) -> bool),
        entry!(0x00c0_4600, fn_00c04600(Ptr) -> Ptr),
        entry!(0x00c0_4570, fn_00c04570(Ptr) -> Ptr),
        entry!(0x00c0_4030, fn_00c04030(Ptr)),
        entry!(0x00c0_4090, fn_00c04090(Ptr, u32)),
        entry!(0x00c0_4120, fn_00c04120(Ptr) -> bool),
        entry!(0x00c0_4310, fn_00c04310(Ptr, u32)),
        entry!(0x00c0_43f0, fn_00c043f0(Ptr, u32)),
        entry!(0x00c0_4490, fn_00c04490(Ptr, u32)),
        entry!(0x00c0_4620, fn_00c04620(Ptr)),
        entry!(0x00c0_4710, fn_00c04710(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c04600_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x5c);
        e.mem.write(this, &[0xcc; 0x5c]);
        assert_eq!(e.call(0x00c0_4600, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BCA64);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}

#[cfg(test)]
mod shader_base_tests {
    use super::*;

    #[test]
    fn fn_00c04570_builds_both_bases_and_runs_the_set_ups() {
        let mut e = Engine::new();
        for target in [
            BS_SHADER_CONSTRUCT_00BE2200,
            EFFECT_CONSTRUCT_00BA4080,
            SET_UP_00BE0870,
            SET_UP_00C04030,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xcc);
        e.mem.write(this, &[0xcc; 0xcc]);
        assert_eq!(e.call(0x00c0_4570, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BC8F8);
        assert_eq!(e.mem.u32(this + 0x6c), EFFECT_PART_VTABLE_010B0318);
        assert_eq!(e.mem.u32(this + 0x68), 0x18);
        assert_eq!(e.mem.u32(this + 0xc4), 0);
        assert_eq!(e.mem.u32(this + 0xc8), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BS_SHADER_CONSTRUCT_00BE2200, vec![this]),
                (EFFECT_CONSTRUCT_00BA4080, vec![this + 0x6c]),
                (SET_UP_00BE0870, vec![this]),
                (SET_UP_00C04030, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod declaration_tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    /// The address the rig's constructor double sits at (it gives the shader its vtable).
    const RIG_CONSTRUCTOR: u32 = 0x0f9f_0000;

    #[test]
    fn fn_00c04030_declares_two_entries_and_hands_them_over() {
        let mut e = rig(RIG_CONSTRUCTOR, &[]);
        e.call(RIG_CONSTRUCTOR, &args![SHADER]);
        e.call_log = Some(vec![]);
        e.call(0x00c0_4030, &args![SHADER]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        let d = DECLARATION_0;
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE_00E76700, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod pass_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        IMPORT_INTERLOCKED_DECREMENT, NEW_RENDER_STATES_00E7F5D0, SET_RENDER_STATE_00E7F430,
        STAGE_DELETE_00E7ECD0,
    };
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// An object whose vtable has `base + offset` at each given slot offset (each a double
    /// answering 0); vtables at `vtable` must be 0x1000 apart.
    fn with_slots(e: &mut Engine, vtable: u32, base: u32, offsets: &[u32]) -> u32 {
        let top = offsets.iter().max().copied().unwrap_or(0);
        let mut slots = vec![0u32; (top / 4 + 1) as usize];
        for offset in offsets {
            slots[(offset / 4) as usize] = base + offset;
            e.register(base + offset, |_, _| Ret::default());
        }
        e.put_vtable(vtable, &slots);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00c04090_takes_the_parameters_constant_maps() {
        let mut e = rig();
        let this = e.mem.alloc(0xd0);
        let param = e.mem.alloc(0x90);
        let vertex_map = object(&mut e, 0);
        let old = object(&mut e, 1);
        e.mem.set_u32(param + 0x4c, vertex_map);
        e.mem.set_u32(this + 0x30, old);
        e.call(0x00c0_4090, &args![this, param]);
        assert_eq!(e.mem.u32(this + 0x34), vertex_map);
        assert_eq!(e.mem.u32(vertex_map + 4), 1);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0xcc), param);
        // The same maps again: nothing counted or released.
        e.mem.set_u32(this + 0xcc, 0);
        e.call(0x00c0_4090, &args![this, param]);
        assert_eq!(e.mem.u32(vertex_map + 4), 1);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(e.mem.u32(this + 0xcc), param);
    }

    const SHADER_SLOTS: u32 = 0x0e20_0000;

    #[test]
    fn fn_00c04120_builds_the_pass() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let this = with_slots(&mut e, 0x0e21_0000, SHADER_SLOTS, &[0x150]);
        let vertex = object(&mut e, 0);
        let pixel = object(&mut e, 0);
        let old_vertex = object(&mut e, 1);
        e.mem.set_u32(this + 0xc4, vertex);
        e.mem.set_u32(this + 0xc8, pixel);
        e.mem.set_u16(this + 0x9c, 2);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x14, 5);
        e.mem.set_u32(pass + 0x5c, old_vertex);
        e.register_double(NEW_PASS_00E804D0, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        let stage = e.mem.alloc(0x70);
        e.mem.set_u32(stage + 0x60, 1);
        e.register_double(CREATE_NEW_STAGE, move |e, a| {
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        for callee in [
            SETUP_DEFAULT_STAGE,
            PASS_SET_STAGE,
            SET_RENDER_STATE_00E7F430,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(NEW_RENDER_STATES_00E7F5D0, |_, _| 0x0e22_0000u32.into_ret());
        assert!(e.call(0x00c0_4120, &args![this]).bool());
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x5c]]);
        let local = calls(&e, CREATE_NEW_STAGE)[0][0];
        assert_eq!(calls(&e, CREATE_NEW_STAGE), vec![vec![local], vec![local]]);
        assert_eq!(
            calls(&e, SETUP_DEFAULT_STAGE),
            vec![vec![stage, 0, 3, 2, 1], vec![stage, 1, 3, 2, 1]]
        );
        assert_eq!(
            calls(&e, PASS_SET_STAGE),
            vec![vec![pass, 5, stage], vec![pass, 5, stage]]
        );
        assert_eq!(calls(&e, NEW_RENDER_STATES_00E7F5D0).len(), 1);
        let block = 0x0e22_0000;
        assert_eq!(
            calls(&e, SET_RENDER_STATE_00E7F430),
            vec![
                vec![block, 7, 0, 1],
                vec![block, 0xe, 0, 1],
                vec![block, 0x1b, 0, 1],
                vec![block, 0xf, 0, 1],
                vec![block, 0xa8, 0xf, 1],
            ]
        );
        assert_eq!(calls(&e, SHADER_SLOTS + 0x150), vec![vec![this]]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old_vertex]]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), pass);
        // The stage's reference dropped once: 1 to 0, deleted.
        assert_eq!(e.mem.u32(stage + 0x60), 0);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
    }

    #[test]
    fn fn_00c04120_without_textures_keeps_a_null_stage() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let this = with_slots(&mut e, 0x0e21_0000, SHADER_SLOTS, &[0x150]);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x30, 0x0e22_0000);
        e.register_double(NEW_PASS_00E804D0, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        e.call(0x00c0_4120, &args![this]);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert!(calls(&e, NEW_RENDER_STATES_00E7F5D0).is_empty());
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).len(), 5);
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    /// A shader with `count` textures (entry 1 set, the others null) and a current pass whose
    /// stage array holds `stages`.
    fn stage_rig(count: u16, stages: &[u32]) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0xd0);
        let textures = e.mem.alloc(4 * u32::from(count));
        e.mem.set_u32(textures + 4, 0x4000);
        e.mem.set_u32(this + 0x98, textures);
        e.mem.set_u16(this + 0x9e, count);
        let pass = e.mem.alloc(0x70);
        let array = e.mem.alloc(4 * (stages.len() as u32).max(1));
        for (index, stage) in stages.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *stage);
        }
        e.mem.set_u32(pass + 0x24, array);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        for callee in [TEXTURE_TO_STAGE_00BA38F0, STAGE_UPDATE_00E7EB00] {
            e.register(callee, |_, _| Ret::default());
        }
        (e, this)
    }

    fn stage(e: &mut Engine, count: u32) -> u32 {
        let stage = e.mem.alloc(0x70);
        e.mem.set_u32(stage + 8, 0x77);
        e.mem.set_u32(stage + 0x60, count);
        stage
    }

    #[test]
    fn fn_00c04310_sets_up_each_stage_from_its_texture() {
        let (mut e, this) = stage_rig(4, &[0, 0, 0]);
        let first = stage(&mut e, 1);
        let array = e.mem.u32(e.global::<u32>(CURRENT_PASS_0126F74C) + 0x24);
        // Stage 0 (texture 1, set), stage 1 the same object (texture 2, null), stage 2 null.
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, first);
        e.mem.set_u32(array + 8, 0);
        e.call(0x00c0_4310, &args![this, 0u32]);
        assert_eq!(
            calls(&e, TEXTURE_TO_STAGE_00BA38F0),
            vec![vec![0x4000, first]]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(
            calls(&e, STAGE_UPDATE_00E7EB00),
            vec![vec![first, 0], vec![first, 0]]
        );
        // Counted once when held, dropped when the null stage came: back to 1.
        assert_eq!(e.mem.u32(first + 0x60), 1);
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn fn_00c04310_drops_the_last_stage_at_the_end() {
        let (mut e, this) = stage_rig(2, &[0]);
        let only = stage(&mut e, 0);
        let array = e.mem.u32(e.global::<u32>(CURRENT_PASS_0126F74C) + 0x24);
        e.mem.set_u32(array, only);
        e.call(0x00c0_4310, &args![this, 0u32]);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![only]]);
        // One texture only: no stage at all.
        let (mut e, this) = stage_rig(1, &[]);
        e.call(0x00c0_4310, &args![this, 0u32]);
        assert!(calls(&e, STAGE_UPDATE_00E7EB00).is_empty());
    }

    const MAP_SLOTS: u32 = 0x0e23_0000;

    #[test]
    fn fn_00c043f0_hands_both_constant_maps_the_pass_shaders() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        let geometry = e.mem.alloc(0xc0);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, geometry);
        e.set_global(CURRENT_GEOMETRY_011F91E0, holder);
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x34, 0x3434);
        e.mem.set_u32(geometry + 0xb8, inner);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x44, 0x4444);
        e.mem.set_u32(pass + 0x5c, 0x5c5c);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        let pixel_map = with_slots(&mut e, 0x0e24_0000, MAP_SLOTS, &[0x78]);
        let vertex_map = e.mem.alloc(8);
        e.mem.set_u32(vertex_map, e.mem.u32(pixel_map));
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0x30, pixel_map);
        e.mem.set_u32(this + 0x34, vertex_map);
        e.call(0x00c0_43f0, &args![this, 9u32]);
        let word = DEFAULT_011F4288;
        assert_eq!(
            calls(&e, MAP_SLOTS + 0x78),
            vec![
                vec![
                    pixel_map,
                    0x4444,
                    geometry,
                    0,
                    0,
                    0x3434,
                    9,
                    0,
                    geometry + 0x68,
                    word,
                    0,
                    1
                ],
                vec![
                    vertex_map,
                    0x5c5c,
                    geometry,
                    0,
                    0,
                    0x3434,
                    9,
                    0,
                    geometry + 0x68,
                    word,
                    0,
                    1
                ],
            ]
        );
        // A pass without a vertex shader and a geometry with its own +0x20 word.
        e.mem.set_u32(pass + 0x5c, 0);
        e.mem.set_u32(geometry + 0x20, 0x2020);
        e.mem.set_u32(this + 0x30, 0);
        e.call(0x00c0_43f0, &args![this, 9u32]);
        assert_eq!(calls(&e, MAP_SLOTS + 0x78).len(), 2);
        e.mem.set_u32(this + 0x30, pixel_map);
        e.call(0x00c0_43f0, &args![this, 9u32]);
        assert_eq!(calls(&e, MAP_SLOTS + 0x78)[2][9], 0x2020);
    }

    const DEVICE_SLOTS: u32 = 0x0e25_0000;

    #[test]
    fn fn_00c04490_clears_each_stage_and_its_device_texture() {
        let (mut e, this) = stage_rig(3, &[0, 0]);
        let first = stage(&mut e, 1);
        let second = stage(&mut e, 1);
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let array = e.mem.u32(pass + 0x24);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, second);
        let device = with_slots(&mut e, 0x0e26_0000, DEVICE_SLOTS, &[0xdc]);
        e.register_double(PASS_DEVICE_00E7F7C0, move |_, _| device.into_ret());
        e.call(0x00c0_4490, &args![this, 0u32]);
        assert_eq!(
            calls(&e, PASS_DEVICE_00E7F7C0),
            vec![vec![pass], vec![pass]]
        );
        assert_eq!(
            calls(&e, DEVICE_SLOTS + 0xdc),
            vec![vec![device, 0, 0], vec![device, 1, 0]]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(e.mem.u32(first + 0x60), 1);
        assert_eq!(e.mem.u32(second + 0x60), 1);
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn fn_00c04620_frees_the_texture_and_the_first_parameter() {
        let mut e = rig();
        for callee in [
            TEXTURE_DESTRUCT_00BA3A00,
            MEMORY_DEALLOCATE_00AA4060,
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x5c);
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x58, texture);
        let parameter = with_slots(&mut e, 0x0e27_0000, 0x0e28_0000, &[0]);
        let parameters = e.mem.alloc(4);
        e.mem.set_u32(parameters, parameter);
        e.mem.set_u32(this + 0x1c, parameters);
        e.call(0x00c0_4620, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BCA64);
        assert_eq!(calls(&e, TEXTURE_DESTRUCT_00BA3A00), vec![vec![texture]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, texture]]
        );
        let sets = calls(&e, ARRAY_SET_AT_00BA3AA0);
        assert_eq!(sets.len(), 2);
        assert_eq!((sets[0][0], sets[0][1]), (this + 0x28, 2));
        assert_eq!((sets[1][0], sets[1][1]), (this + 0x18, 0));
        assert_eq!(calls(&e, 0x0e28_0000), vec![vec![parameter, 1]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x58), texture);
    }

    #[test]
    fn fn_00c04620_and_00c04710_without_texture_or_parameter() {
        let mut e = rig();
        for callee in [
            MEMORY_DEALLOCATE_00AA4060,
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x5c);
        let parameters = e.mem.alloc(4);
        e.mem.set_u32(this + 0x1c, parameters);
        assert_eq!(e.call(0x00c0_4710, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, ARRAY_SET_AT_00BA3AA0).is_empty());
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(e.call(0x00c0_4710, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, engine};

    #[test]
    fn fn_00c046e0_sets_constant_0() {
        let mut e = engine(&[PARAM_SET_PIXEL_CONSTANT_00B8A790]);
        let this = e.mem.alloc(0x60);
        let parameters = e.mem.alloc(8);
        e.mem.set_u32(parameters, 0x88);
        e.mem.set_u32(this + 0x1c, parameters);
        e.mem.set_u32(this + 0x5c, 0x7f80_0001);
        assert!(e.call(0x00c0_46e0, &args![this, 0u32]).bool());
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![0x88, 0, 0x7fc0_0001, 0, 0, 0]]
        );
    }
}
