//! A texture stage's binding: whether its texture changed, the stage's states handed to the render state
//! cache, the texture the texture manager finds for it, its transform and sampler commands; and the
//! texture manager's query with the getters of the texture data classes it reaches through vtables.
//!
//! The render state object (`0126f6c8`) is the renderer's state cache (`NiDX9RenderState`): its slots
//! 0xc0, 0xcc, 0xdc and 0x68 hold the shader family's setters, called directly through
//! [`state::cache_stage_state`] and the others. The texture manager's slot 0xc holds
//! [`texture_data`]; the texture data classes' slots hold the getters below and the leaf `00e7cd40`.

use super::layout::TextureStage;
use super::FAMILY;
use crate::shader::state;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::{args, typed_call, Engine};

/// The renderer (`0126f6c4`), the Direct3D device (`0126f6c0`) and the render state cache
/// (`0126f6c8`).
const RENDERER: u32 = 0x0126_f6c4;
const DEVICE: u32 = 0x0126_f6c0;
const RENDER_STATE: u32 = 0x0126_f6c8;
/// The texture each stage was last bound with (`0126f680`, a word a stage).
const SHADOWED_TEXTURES: u32 = 0x0126_f680;
/// The matrix `00e7df90` builds (`0126f6d0`).
const MATRIX_RESULT: u32 = 0x0126_f6d0;
/// How many texture stages there are (`011bf558`).
const STAGE_COUNT: u32 = 0x011b_f558;
/// The maximum anisotropy an anisotropic stage sets (`011a9608`).
const MAX_ANISOTROPY: u32 = 0x011a_9608;
/// `00e7df90` (unnamed; `thiscall`-less, two words: the mode and the matrix; a bool): builds the
/// texture matrix of a transform mode into [`MATRIX_RESULT`]. Called by address: the exe calls it from
/// [`send_transform`] only for a stage with a transform mode.
const BUILD_MATRIX: u32 = 0x00e7_df90;
/// The texture manager (renderer +0x8c4) and its lookup (slot 0xc, [`texture_data`]).
const SLOT_TEXTURE_LOOKUP: u32 = 0xc;
const TEXTURE_DATA: u32 = 0x00e9_0b10;
/// Device slots (`IDirect3DDevice9`): `SetTransform`, `SetSamplerState`.
const DEVICE_SET_TRANSFORM: u32 = 0xb0;
const DEVICE_SET_SAMPLER_STATE: u32 = 0x114;
/// The renderer's flags (slot 0xa0).
const SLOT_RENDERER_FLAGS: u32 = 0xa0;
/// The texture's renderer lock (`011f4748` + 0x180) with `EnterCriticalSection` and
/// `LeaveCriticalSection` (import slots), the texture's source (`00653290`, the dispatch family's RTTI cast to
/// the class `011f444c`) and the renderer data maker (`00e68ef0`, `cdecl`: the source and
/// the manager's renderer).
const TEXTURE_RENDERER: u32 = 0x011f_4748;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const SOURCE_KIND: u32 = 0x011f_444c;
const MAKE_RENDERER_DATA: u32 = 0x00e6_8ef0;
/// The texture data's slots: the levels (0x98), the Direct3D texture (0x9c), the render target or
/// other surface (0xa4, else 0xac), the source texture data (0xa8) and its reload (0xb4).
const SLOT_LEVELS: u32 = 0x98;
const SLOT_D3D_TEXTURE: u32 = 0x9c;
const SLOT_TARGET_A: u32 = 0xa4;
const SLOT_TARGET_B: u32 = 0xac;
const SLOT_SOURCE_DATA: u32 = 0xa8;
const SLOT_RELOAD: u32 = 0xb4;

/// The render state cache (`0126f6c8`), read afresh as the exe does before each call.
#[inline(always)]
fn render_state(e: &Engine) -> u32 {
    e.global::<u32>(RENDER_STATE)
}

/// The low byte of a word (a `bool` the exe pushes as a word).
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
fn low8(word: u32) -> u8 {
    word as u8
}

// Typed from 00e7eb00
/// `NiD3DTextureStage::ApplyTexture` (the census's `ApplyRenderStateChanges`), `thiscall(stage,
/// force)`: unless `force` is clear and the stage's texture is the one last bound to its stage
/// (`0126f680[stage]`), that texture is remembered there and the stage's state cache applies the
/// stage's states (`00e89410`); a non-zero answer returns false, else the stage is applied
/// ([`apply`]) and true is returned.
pub fn bind(e: &mut Engine, this: Ptr<TextureStage>, force: u8) -> bool {
    let stage = e.get(this, TextureStage::m_uiStage);
    let texture = e.get(this, TextureStage::m_pkTexture);
    let shadow = SHADOWED_TEXTURES.wrapping_add(stage.wrapping_mul(4));
    if force == 0 && texture == e.global::<u32>(shadow) {
        return true;
    }
    e.set_global(shadow, texture);
    let cache = e.get(this, TextureStage::pStateCache);
    if typed_call!(
        crate::shader::FAMILY,
        0x00e8_9410,
        state::apply_states(e, cache, stage)
    ) != 0
    {
        return false;
    }
    typed_call!(FAMILY, 0x00e7_ea00, apply(e, this));
    true
}

// Typed from 00e7ea00
/// `thiscall(stage)`: for a stage below the stage count the render state cache gets the stage's
/// texture coordinate index (state 0xb: the state cache's cached value, `00e89060`, else the stage
/// number); then the texture manager's lookup of the stage's texture (none without one) is set as the
/// stage's texture; with a texture, a clear second flag of the lookup sets the stage's transform flags
/// (sampler slot, `(stage, 7, 0, 0)`), the lookup's word goes to the sampler commands
/// ([`send_sampler_commands`]) and the transform is sent ([`send_transform`]). An anisotropic stage
/// (filter mode 6) also sets its maximum anisotropy on the device.
pub fn apply(e: &mut Engine, this: Ptr<TextureStage>) {
    let stage = e.get(this, TextureStage::m_uiStage);
    // The frame's words the callees fill: a dword, two flag bytes and a dword.
    e.with_stack(0x10, |e, frame| {
        let f = frame.addr();
        let value_slot = f;
        let flag_a = f.wrapping_add(8);
        let flag_b = f.wrapping_add(9);
        let table_slot = f.wrapping_add(0xc);
        if stage < e.global::<u32>(STAGE_COUNT) {
            e.mem.set_u32(value_slot, 0);
            e.mem.set_u8(flag_a, 0);
            let cache = e.get(this, TextureStage::pStateCache);
            let found = typed_call!(
                crate::shader::FAMILY,
                0x00e8_9060,
                state::cached_stage_state(e, cache, 0xb, value_slot, flag_a)
            );
            let value = if found {
                e.mem.u32(value_slot)
            } else {
                e.get(this, TextureStage::m_uiStage)
            };
            let target = render_state(e);
            let stage_now = e.get(this, TextureStage::m_uiStage);
            state::cache_stage_state(e, target, stage_now, 0xb, value, 0);
        }
        let mut found = 0u32;
        let texture = e.get(this, TextureStage::m_pkTexture);
        if texture != 0 {
            let renderer = e.global::<u32>(RENDERER);
            let manager = e.mem.u32(renderer.wrapping_add(0x8c4));
            found = if crate::slot_target(e, manager, SLOT_TEXTURE_LOOKUP) == TEXTURE_DATA {
                typed_call!(
                    FAMILY,
                    0x00e9_0b10,
                    texture_data(e, Ptr::new(manager), texture, flag_a, flag_b, table_slot)
                )
            } else {
                e.vcall(
                    manager,
                    SLOT_TEXTURE_LOOKUP,
                    &args![texture, flag_a, flag_b, table_slot],
                )
                .u32()
            };
        }
        let target = render_state(e);
        let stage_now = e.get(this, TextureStage::m_uiStage);
        state::cache_texture(e, target, stage_now, found);
        if found != 0 {
            if e.mem.u8(flag_b) == 0 {
                let target = render_state(e);
                let stage_now = e.get(this, TextureStage::m_uiStage);
                state::cache_sampler_state(e, target, stage_now, 7, 0, 0);
            }
            let word = e.mem.u32(table_slot);
            typed_call!(
                FAMILY,
                0x00e7_dc90,
                send_sampler_commands(e, this.cast(), word)
            );
            typed_call!(FAMILY, 0x00e7_e940, send_transform(e, this));
        }
        if e.get(this, TextureStage::m_eFilterMode) == 6 {
            let device = e.global::<u32>(DEVICE);
            let stage_now = e.get(this, TextureStage::m_uiStage);
            let value = e.global::<u32>(MAX_ANISOTROPY);
            e.vcall(
                device,
                DEVICE_SET_SAMPLER_STATE,
                &args![stage_now, 10u32, value],
            );
        }
    })
}

/// `SetTransform(stage + 0x10, matrix)` on the device (`D3DTS_TEXTURE0` + stage).
fn set_transform(e: &mut Engine, stage: u32, matrix: u32) {
    let device = e.global::<u32>(DEVICE);
    e.vcall(
        device,
        DEVICE_SET_TRANSFORM,
        &args![stage.wrapping_add(0x10), matrix],
    );
}

// Typed from 00e7e940
/// `thiscall(stage)`: the stage's texture transform to the device, for a stage below the stage count.
/// Without a transform the render state cache's stage slot gets `(stage, 0x18, 0, 0)`
/// (`D3DTSS_TEXTURETRANSFORMFLAGS` off). Otherwise the top nibble of the transform flags picks the
/// matrix: 0 the stage's own, set as it is; 1 the source's (+0x14 of +0x18) and 2 the stage's own,
/// set as they are with mode 0, else built by `00e7df90` with the mode and set when it succeeds; any
/// other nibble sets nothing.
pub fn send_transform(e: &mut Engine, this: Ptr<TextureStage>) {
    let stage = e.get(this, TextureStage::m_uiStage);
    if stage >= e.global::<u32>(STAGE_COUNT) {
        return;
    }
    if e.get(this, TextureStage::bHasTransform) == 0 {
        let target = render_state(e);
        state::cache_stage_state(e, target, stage, 0x18, 0, 0);
        return;
    }
    let flags = e.get(this, TextureStage::uiTransformFlags);
    let mode = flags & 0x0fff_ffff;
    let own = this.addr().wrapping_add(TextureStage::afTransform.off);
    let matrix = match flags & 0xf000_0000 {
        0 => {
            set_transform(e, stage, own);
            return;
        }
        0x1000_0000 => {
            let source = e.get(this, TextureStage::pTransformSource);
            let matrix = e.mem.u32(source.wrapping_add(0x14));
            if mode == 0 {
                set_transform(e, stage, matrix);
                return;
            }
            matrix
        }
        0x2000_0000 => {
            if mode == 0 {
                set_transform(e, stage, own);
                return;
            }
            own
        }
        _ => return,
    };
    if e.call(BUILD_MATRIX, &args![mode, matrix]).bool() {
        set_transform(e, stage, MATRIX_RESULT);
    }
}

// Typed from 00e7dc90
/// `thiscall(stage, enable)`, of which only the low byte of `enable` is read: with it set, and the
/// renderer's flags (slot 0xa0, asked twice) without bit 3 and with bit 2, the render state cache's
/// sampler slot gets `(stage word +4, 1, 3, 0)` and `(+4, 2, 3, 0)` (`D3DSAMP_ADDRESSU` and `V`
/// clamped), then for each of 8 steps its render state slot `(0x80 + step, 0, 0, 0)` and
/// `(0xc6 + step, 0, 0, 0)`.
pub fn send_sampler_commands(e: &mut Engine, this: Ptr, enable: u32) {
    if low8(enable) == 0 {
        return;
    }
    let renderer = e.global::<u32>(RENDERER);
    if e.vcall(renderer, SLOT_RENDERER_FLAGS, &[]).u32() & 8 != 0 {
        return;
    }
    let renderer = e.global::<u32>(RENDERER);
    if e.vcall(renderer, SLOT_RENDERER_FLAGS, &[]).u32() & 4 == 0 {
        return;
    }
    for which in [1u32, 2] {
        let target = render_state(e);
        let word = e.mem.u32(this.addr().wrapping_add(4));
        state::cache_sampler_state(e, target, word, which, 3, 0);
    }
    for step in 0..8u32 {
        let target = render_state(e);
        state::cache_render_state(e, target, 0x80 + step, 0, 0, 0);
        let target = render_state(e);
        state::cache_render_state(e, target, 0xc6 + step, 0, 0, 0);
    }
}

/// The texture's renderer data (+0x24) under the renderer's lock; when there is none it is made from
/// the texture's source (`00653290`, then `00e68ef0` with the manager's +0xc), and `made` is set; no
/// source answers null.
fn renderer_data(e: &mut Engine, this: Ptr, texture: u32, made: u32) -> u32 {
    let lock = e.global::<u32>(TEXTURE_RENDERER).wrapping_add(0x180);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let mut data = e.mem.u32(texture.wrapping_add(0x24));
    if data == 0 {
        let source = typed_call!(
            crate::dispatch::FAMILY,
            0x0065_3290,
            crate::dispatch::scene::cast(e, Ptr::new(texture), SOURCE_KIND)
        )
        .addr();
        if source == 0 {
            e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
            return 0;
        }
        let renderer = e.mem.u32(this.addr().wrapping_add(0xc));
        data = e.call(MAKE_RENDERER_DATA, &args![source, renderer]).u32();
        e.mem.set_u8(made, 1);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    data
}

/// A texture data's word-answering slot with no arguments: the getter the texture data classes hold
/// there ([`d3d_texture`], [`this_word`], [`levels`] or the leaf `00e7cd40`, 0) directly, another
/// target through the vtable.
#[inline(always)]
fn data_word(e: &mut Engine, data: u32, slot: u32) -> u32 {
    match crate::slot_target(e, data, slot) {
        0x00e7_cd40 => typed_call!(leaf::FAMILY, 0x00e7_cd40, leaf::zero(e, data)),
        0x00e6_8800 => typed_call!(FAMILY, 0x00e6_8800, d3d_texture(e, data)),
        0x00e6_8810 => typed_call!(FAMILY, 0x00e6_8810, this_word(e, data)),
        0x00e6_87f0 => typed_call!(FAMILY, 0x00e6_87f0, levels(e, data)),
        _ => e.vcall(data, slot, &[]).u32(),
    }
}

/// Whether `value` is a nonzero power of two.
#[inline(always)]
fn power_of_two(value: u32) -> bool {
    value != 0 && value & value.wrapping_sub(1) == 0
}

// Typed from 00e90b10
/// `NiDX9TextureManager`'s lookup (vtable slot 0xc; the census's `GetTextureData`), `thiscall(manager,
/// texture, made, mipmapped, not_power_of_two)`, the last three the addresses of bytes cleared first.
/// Null without a texture or without renderer data ([`renderer_data`], which sets `made` when it makes
/// it). `not_power_of_two` from the data's width (+0xc) and height (+0x10). When the data's slot 0xa4,
/// or else 0xac, answers an object, that object's slot 0x9c is the answer. Otherwise, with an object
/// from slot 0xa8, its slot 0xb4 reloads it when the data's slot 0x9c answers null (which sets `made`)
/// or the texture's byte +0x40 is clear; `mipmapped` is whether the data's slot 0x98 answers more than
/// 1, and the data's slot 0x9c is the answer.
pub fn texture_data(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    made: u32,
    mipmapped: u32,
    not_power_of_two: u32,
) -> u32 {
    e.mem.set_u8(made, 0);
    e.mem.set_u8(mipmapped, 0);
    e.mem.set_u8(not_power_of_two, 0);
    if texture == 0 {
        return 0;
    }
    let data = renderer_data(e, this, texture, made);
    if data == 0 {
        return 0;
    }
    let width = e.mem.u32(data.wrapping_add(0xc));
    let height = e.mem.u32(data.wrapping_add(0x10));
    let regular = power_of_two(width) && power_of_two(height);
    e.mem.set_u8(not_power_of_two, u8::from(!regular));
    let mut target = data_word(e, data, SLOT_TARGET_A);
    if target == 0 {
        target = data_word(e, data, SLOT_TARGET_B);
    }
    if target != 0 {
        return data_word(e, target, SLOT_D3D_TEXTURE);
    }
    let other = data_word(e, data, SLOT_SOURCE_DATA);
    if other != 0 {
        if data_word(e, data, SLOT_D3D_TEXTURE) == 0 {
            e.vcall(other, SLOT_RELOAD, &[]);
            e.mem.set_u8(made, 1);
        } else if e.mem.u8(texture.wrapping_add(0x40)) == 0 {
            e.vcall(other, SLOT_RELOAD, &[]);
        }
    }
    let levels = data_word(e, data, SLOT_LEVELS);
    e.mem.set_u8(mipmapped, u8::from(levels > 1));
    data_word(e, data, SLOT_D3D_TEXTURE)
}

// Typed from 00e68800
/// The word +0x64 of a texture data (`NiDX9SourceTextureData` and the other texture data classes'
/// slot 0x9c, folded): its Direct3D texture.
pub fn d3d_texture(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this.wrapping_add(0x64))
}

// Typed from 00e68810
/// `this` (a slot many classes share by identical code folding, among them a texture data's 0xa8).
pub fn this_word(_e: &mut Engine, this: u32) -> u32 {
    this
}

// Typed from 00e687f0
/// The word +0x68 of a texture data (slot 0x98, folded): its mipmap level count.
pub fn levels(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this.wrapping_add(0x68))
}
