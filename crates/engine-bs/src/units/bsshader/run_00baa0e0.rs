//! `BSShader/run_00baa0e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// This effect's vtable (`010b895c`; no RTTI locator before it).
const VTABLE_010B895C: u32 = 0x010b_895c;

// Translated from 00baa160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, then the vtable `010b895c`.
/// Answers `this`.
pub fn fn_00baa160(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B895C);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The vtable `00baa0e0` stores (`010b8488`), `BSShaderProperty::FreeRenderPasses` (engine
/// map, `00baa000`, `thiscall`) and the base destructor `00a5c240` (unit `niobjectnet.cpp`,
/// unnamed, `thiscall`).
const VTABLE_010B8488: u32 = 0x010b_8488;
const FREE_RENDER_PASSES_00BAA000: u32 = 0x00ba_a000;
const BASE_DESTRUCT_00A5C240: u32 = 0x00a5_c240;

// Translated from 00baa0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00baa0e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (`run_00baa0e0`): a shader property's destructor, `thiscall`, no
/// stack words: stores the vtable `010b8488`, clears +0x20 and +0x24, frees the render passes
/// (`BSShaderProperty::FreeRenderPasses`) and runs the base destructor `00a5c240`.
pub fn fn_00baa0e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B8488);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.call(FREE_RENDER_PASSES_00BAA000, &args![base]);
    e.call(BASE_DESTRUCT_00A5C240, &args![base]);
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`) and
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one word).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00baa180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00baa180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (`run_00baa0e0`): the destructor of the effect `00baa160` builds,
/// `thiscall`, no stack words: the vtable `010b895c`; a first effect parameter (+0x1c elements)
/// is deleted through its slot +0 with 1 (the element tested twice) and its element set to
/// null; then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00baa180(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B895C);
    let element = e.mem.u32(base + 0x1c);
    if e.mem.u32(element) != 0 {
        let param = e.mem.u32(element);
        if param != 0 {
            e.vcall(param, 0, &args![1u32]);
        }
        set_at(e, base + 0x18, 0, 0);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00baa220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`): the scalar deleting destructor of the effect
/// `00baa160` builds, `thiscall`, one word: `00baa180`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00baa220(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00baa180(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90`, the `NiTArray` set-size `00a90180`,
/// `ImageSpaceShaderParam`'s constructor (`00b8a9e0`), `InitConstantMap` (`00b8ab10`, two
/// words), `SetPixelConstant` (`00b8a790`, five words) and its render-state setter `00b8a8d0`
/// (unnamed; `thiscall`, two words: state, value), and the input and effect setters
/// `00ba3b10` and `00ba3c70` (unit `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const PARAM_SET_RENDER_STATE_00B8A8D0: u32 = 0x00b8_a8d0;
const SET_INPUT_00BA3B10: u32 = 0x00ba_3b10;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;

// Translated from 00baa240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00baa240 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter allocation) is not translated.
/// Unnamed in the engine map (`run_00baa0e0`): the `Setup` slot of the effect `00baa160`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 1
/// (`00ba3f90`) and `pTextures` to 2; one parameter (0x88 bytes; the constructor only on a
/// block) with `InitConstantMap(0, 1)`, pixel constant 0 = (1, 1, 1, 1) and render state 0xa8
/// = 0x0f; stored as parameter 0; input 0 is texture 1 (filter 0) and effect 0 the manager's
/// +0xc0.
pub fn fn_00baa240(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 1u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let param = if block != 0 {
        e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
    } else {
        0
    };
    e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 0u32, 1u32]);
    let one = 0x3f80_0000u32;
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, one, one, one, one],
    );
    e.call(
        PARAM_SET_RENDER_STATE_00B8A8D0,
        &args![param, 0xa8u32, 0xfu32],
    );
    set_at(e, base + 0x18, 0, param);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 0xc0);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, param, 0u32]);
}

/// The pass the image-space shaders set up last (`0126f74c`).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;

// Translated from 00baa3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`; Ghidra made no function of it), `thiscall`, one
/// word (a pass number): for passes 0x16 to 0x3d the word at `this + 4 * pass + 0x14`, for
/// passes 0x25c to 0x287 the word at `this + 4 * pass - 0x904` (both a table from +0x6c)
/// becomes the current pass (`0126f74c`); other passes change nothing.
pub fn fn_00baa3f0(e: &mut Engine, this: Ptr, pass: u32) {
    let slot = this.addr().wrapping_add(pass.wrapping_mul(4));
    if pass.wrapping_sub(0x16) <= 0x27 {
        let value = e.mem.u32(slot.wrapping_add(0x14));
        e.set_global(CURRENT_PASS_0126F74C, value);
    } else if pass.wrapping_sub(0x25c) <= 0x2b {
        let value = e.mem.u32(slot.wrapping_sub(0x904));
        e.set_global(CURRENT_PASS_0126F74C, value);
    }
}

/// The fog byte (`011f91c4`), `BSShaderManager::GetFogProperty` (engine map, `00b55520`,
/// `cdecl`, one word), the default `float` `0108cb00`, the default colour (`011a9bd0`) and
/// the eight `float`s set (`011fa280`).
const FOG_KIND_011F91C4: u32 = 0x011f_91c4;
const GET_FOG_PROPERTY_00B55520: u32 = 0x00b5_5520;
const DEFAULT_0108CB00: u32 = 0x0108_cb00;
const DEFAULT_COLOR_011A9BD0: u32 = 0x011a_9bd0;
const FOG_011FA280: u32 = 0x011f_a280;

// Translated from 00baa430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`; Ghidra made no function of it), `thiscall`, one
/// word it does not read: the fog constants at `011fa280`. With a fog property
/// (`GetFogProperty` of the byte `011f91c4`): its `float`s +0x30 and +0x30 - +0x2c (`FSUB`),
/// +0x60, 0.0, then its colour +0x20, +0x24, +0x28 and 0.0 (each through `FLD`/`FSTP`, a
/// signalling NaN quieted). Without one: the `float` at `0108cb00`, three 0.0 and the words of
/// the default colour `011a9bd0`.
pub fn fn_00baa430(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let kind = u32::from(e.global::<u8>(FOG_KIND_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY_00B55520, &args![kind]).u32();
    let words: [u32; 8] = if fog != 0 {
        let start = x87_quiet_bits(e.mem.u32(fog + 0x30));
        let range = x87_store32(x87_sub(x87_load32(start), x87_m32(e.mem.u32(fog + 0x2c))));
        let quiet = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(fog + offset));
        [
            start,
            range,
            quiet(e, 0x60),
            0,
            quiet(e, 0x20),
            quiet(e, 0x24),
            quiet(e, 0x28),
            0,
        ]
    } else {
        let default = x87_quiet_bits(e.global::<u32>(DEFAULT_0108CB00));
        let color = |e: &Engine, index: u32| e.global::<u32>(DEFAULT_COLOR_011A9BD0 + 4 * index);
        [
            default,
            0,
            0,
            0,
            color(e, 0),
            color(e, 1),
            color(e, 2),
            color(e, 3),
        ]
    };
    for (index, word) in words.iter().enumerate() {
        e.set_global(FOG_011FA280 + 4 * index as u32, *word);
    }
}

/// The sized free `00aa1460` (`cdecl`: block, size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00baa140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`): the shader property's scalar deleting
/// destructor, `thiscall`, one word: `00baa0e0`, then the 0x60-byte free (`00aa1460`) when bit
/// 0 of `flags` is set. Answers `this`.
pub fn fn_00baa140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00baa0e0(e, this);
    if flags as u8 & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x60u32]);
    }
    this
}

// Translated from 00baa210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 6 of the vtable `010b895c`, `thiscall`, no
/// stack words: a tail jump to slot 6 (+0x18) of effect 0 of the array at +0xc (what it
/// answers in EAX, EDX and ST0 is this function's result).
pub fn fn_00baa210(e: &mut Engine, this: Ptr) -> Ret {
    let effect = e.mem.u32(e.mem.u32(this.addr() + 0xc));
    e.vcall(effect, 0x18, &args![])
}

/// The uniform-form entry of [`fn_00baa210`]: `this`; the tail call's registers pass through.
fn fn_00baa210_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00baa210(e, Ptr::new(a[0]))
}

/// `ImageSpaceTexture::GetTexture` (engine map, `00ba3780`; `thiscall`, no stack words) and
/// 2^32 as a `float` (`01099378`).
const GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// `1 / size`, the size read unsigned (`FILD`, plus 2^32 when negative), stored as a `float`.
fn reciprocal_size(e: &Engine, size: u32) -> u32 {
    let mut value = f64::from(size as i32);
    if (size as i32) < 0 {
        value = x87_add(value, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_quiet_bits(x87_store32(x87_div(1.0, value)))
}

// Translated from 00baa330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 7 of the vtable `010b895c` (its
/// `UpdateParams`), `thiscall`, one word it does not read. With T the texture of `pTextures[1]`
/// (+0x2c; `GetTexture`): effect parameter 0 takes pixel constant 0 = (1 / T's slot +0x94,
/// 1 / T's slot +0x98, 0, 0), the slots asked +0x98 first and their answers read unsigned. The
/// two zeros and the first reciprocal are pushed before the calls they outlive. Answers true.
pub fn fn_00baa330(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let param = e.mem.u32(e.mem.u32(base + 0x1c));
    let holder = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
    let texture = e.call(GET_TEXTURE_00BA3780, &args![holder]).u32();
    let second = e.vcall(texture, 0x98, &args![]).u32();
    let second = reciprocal_size(e, second);
    let first = e.vcall(texture, 0x94, &args![]).u32();
    let first = reciprocal_size(e, first);
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, first, second, 0u32, 0u32],
    );
    true
}

// Translated from 00baa3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00baa0e0`), `thiscall`, no stack words: gives each of the
/// 0x2c passes at +0x6c to +0x118 to `this`'s vtable slot +0x128.
pub fn fn_00baa3c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..0x2cu32 {
        let pass = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![pass]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_a140, fn_00baa140(Ptr, u32) -> Ptr),
        (0x00ba_a210, fn_00baa210_entry as AbiFn),
        entry!(0x00ba_a330, fn_00baa330(Ptr, u32) -> bool),
        entry!(0x00ba_a3c0, fn_00baa3c0(Ptr)),
        entry!(0x00ba_a160, fn_00baa160(Ptr) -> Ptr),
        entry!(0x00ba_a0e0, fn_00baa0e0(Ptr)),
        entry!(0x00ba_a180, fn_00baa180(Ptr)),
        entry!(0x00ba_a220, fn_00baa220(Ptr, u32) -> Ptr),
        entry!(0x00ba_a240, fn_00baa240(Ptr, u32, u32)),
        entry!(0x00ba_a3f0, fn_00baa3f0(Ptr, u32)),
        entry!(0x00ba_a430, fn_00baa430(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00baa160_runs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |e, a| {
            e.mem.set_u32(a[0], 1);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x58);
        assert_eq!(e.call(0x00ba_a160, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B895C);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}

#[cfg(test)]
mod effect_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00baa0e0_frees_the_passes_and_runs_the_base() {
        let mut e = rig();
        e.register(FREE_RENDER_PASSES_00BAA000, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00A5C240, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0xcc; 0x60]);
        e.call(0x00ba_a0e0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B8488);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0xcccc_cccc);
        assert_eq!(calls(&e, FREE_RENDER_PASSES_00BAA000), vec![vec![this]]);
        assert_eq!(calls(&e, BASE_DESTRUCT_00A5C240), vec![vec![this]]);
    }

    #[test]
    fn fn_00baa180_and_00baa220_delete_the_parameter() {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            0x0e68_0000,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e69_0000, &[0x0e68_0000]);
        let this = e.mem.alloc(0x58);
        let list = e.mem.alloc(4);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e69_0000);
        e.mem.set_u32(list, param);
        e.mem.set_u32(this + 0x1c, list);
        e.call(0x00ba_a180, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B895C);
        assert_eq!(calls(&e, 0x0e68_0000), vec![vec![param, 1]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 0]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        e.mem.set_u32(list, 0);
        assert_eq!(e.call(0x00ba_a220, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, 0x0e68_0000).len(), 1);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00baa240_sets_up_the_parameter() {
        let mut e = rig();
        e.register(PARAM_CONSTRUCT_00B8A9E0, |_, _| 0x0e6a_0000u32.into_ret());
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            PARAM_SET_RENDER_STATE_00B8A8D0,
            ARRAY_SET_AT_00BA3AA0,
            SET_INPUT_00BA3B10,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x58);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0xc4);
        e.mem.set_u32(effects + 0xc0, 0x0e6b_0000);
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00ba_a240, &args![this, manager, 0u32]);
        let param = 0x0e6a_0000;
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 2]]
        );
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![param, 0, 1]]
        );
        let one = 1.0f32.to_bits();
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![param, 0, one, one, one, one]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_RENDER_STATE_00B8A8D0),
            vec![vec![param, 0xa8, 0xf]]
        );
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 0]);
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 0]]);
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![vec![this, 0, 0x0e6b_0000, param, 0]]
        );
    }

    #[test]
    fn fn_00baa3f0_picks_the_pass_from_two_ranges() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0x120);
        for index in 0..0x2cu32 {
            e.mem.set_u32(this + 0x6c + 4 * index, 0x100 + index);
        }
        e.call(0x00ba_a3f0, &args![this, 0x16u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x100);
        e.call(0x00ba_a3f0, &args![this, 0x3du32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x127);
        e.call(0x00ba_a3f0, &args![this, 0x287u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x12b);
        e.call(0x00ba_a3f0, &args![this, 0x25cu32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x100);
        for pass in [0x15u32, 0x3e, 0x25b, 0x288] {
            e.call(0x00ba_a3f0, &args![this, pass]);
            assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x100);
        }
    }

    fn fog(e: &Engine) -> Vec<u32> {
        (0..8)
            .map(|i| e.global::<u32>(FOG_011FA280 + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00baa430_copies_the_fog_property() {
        let mut e = rig();
        e.map(0x011f_9000, 0x2000);
        e.set_global(FOG_KIND_011F91C4, 3u8);
        let property = e.mem.alloc(0x64);
        e.register_double(GET_FOG_PROPERTY_00B55520, move |_, _| property.into_ret());
        e.mem.set_f32(property + 0x20, 0.25);
        e.mem.set_u32(property + 0x24, 0x7f80_0007);
        e.mem.set_f32(property + 0x28, 0.75);
        e.mem.set_f32(property + 0x2c, 100.0);
        e.mem.set_f32(property + 0x30, 1000.0);
        e.mem.set_f32(property + 0x60, 0.5);
        e.call(0x00ba_a430, &args![0u32, 0u32]);
        assert_eq!(calls(&e, GET_FOG_PROPERTY_00B55520), vec![vec![3]]);
        let bits = |v: f32| v.to_bits();
        assert_eq!(
            fog(&e),
            vec![
                bits(1000.0),
                bits(900.0),
                bits(0.5),
                0,
                bits(0.25),
                0x7fc0_0007,
                bits(0.75),
                0
            ]
        );
    }

    #[test]
    fn fn_00baa430_without_a_fog_property_takes_the_defaults() {
        let mut e = rig();
        for page in [0x011f_9000u32, 0x011f_a000, 0x0108_c000, 0x011a_9000] {
            e.map(page, 0x1000);
        }
        e.register(GET_FOG_PROPERTY_00B55520, |_, _| Ret::default());
        e.set_global(DEFAULT_0108CB00, 0x7f80_0008u32);
        for index in 0..4u32 {
            e.set_global(DEFAULT_COLOR_011A9BD0 + 4 * index, 0x7f80_0010 + index);
        }
        e.call(0x00ba_a430, &args![0u32, 0u32]);
        assert_eq!(
            fog(&e),
            vec![
                0x7fc0_0008,
                0,
                0,
                0,
                0x7f80_0010,
                0x7f80_0011,
                0x7f80_0012,
                0x7f80_0013
            ]
        );
    }
}

#[cfg(test)]
mod n070_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Slot doubles: `SLOTS + offset`.
    const SLOTS: u32 = 0x0ea0_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x130);
        for offset in (0..0x130u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for offset in (0..0x130u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00baa140_destroys_and_maybe_frees() {
        let mut e = rig();
        e.register(FREE_RENDER_PASSES_00BAA000, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00A5C240, |_, _| Ret::default());
        e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00ba_a140, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8488);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00ba_a140, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![this, 0x60]]);
    }

    #[test]
    fn fn_00baa210_forwards_to_effect_0() {
        let mut e = rig();
        e.register(SLOTS + 0x18, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let effect = object(&mut e, 8);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, effect);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, list);
        assert_eq!(e.call(0x00ba_a210, &args![this]).u32(), 1);
        assert_eq!(calls(&e, SLOTS + 0x18), vec![vec![effect]]);
    }

    #[test]
    fn fn_00baa330_sets_the_reciprocal_sizes() {
        let mut e = rig();
        e.map(0x0109_9000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.register(SLOTS + 0x94, |_, _| 4u32.into_ret());
        e.register(SLOTS + 0x98, |_, _| 0x8000_0000u32.into_ret());
        e.register(PARAM_SET_PIXEL_CONSTANT_00B8A790, |_, _| Ret::default());
        let texture = object(&mut e, 8);
        e.register(GET_TEXTURE_00BA3780, |e, a| e.mem.u32(a[0]).into_ret());
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, texture);
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures + 4, holder);
        let params = e.mem.alloc(4);
        e.mem.set_u32(params, 0x700);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x1c, params);
        e.mem.set_u32(this + 0x2c, textures);
        assert!(e.call(0x00ba_a330, &args![this, 0u32]).bool());
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a == SLOTS + 0x94 || *a == SLOTS + 0x98)
            .collect();
        assert_eq!(order, vec![SLOTS + 0x98, SLOTS + 0x94]);
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![
                0x700,
                0,
                0.25f32.to_bits(),
                (1.0f32 / 2_147_483_648.0).to_bits(),
                0,
                0
            ]]
        );
    }

    #[test]
    fn fn_00baa3c0_gives_every_pass_to_slot_0x128() {
        let mut e = rig();
        let this = object(&mut e, 0x11c);
        for index in 0..0x2cu32 {
            e.mem.set_u32(this + 0x6c + 4 * index, 0x100 + index);
        }
        e.call(0x00ba_a3c0, &args![this]);
        let given: Vec<u32> = calls(&e, SLOTS + 0x128).iter().map(|a| a[1]).collect();
        assert_eq!(given, (0x100..0x12c).collect::<Vec<u32>>());
    }
}
