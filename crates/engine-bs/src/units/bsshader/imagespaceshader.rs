//! `BSShader/imagespaceshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ImageSpaceShader` (Xbox PDB, 0xd0 bytes; the PC code reads the same offsets) is a
//! `BSShader` with an `ImageSpaceEffect` base at +0x6c, `spVertexShader` +0xc4,
//! `spPixelShader` +0xc8 and `pParam` (an `ImageSpaceShaderParam`) +0xcc. The effect part's
//! `pTextures` array (`NiTPrimitiveArray<ImageSpaceTexture *>`, +0x28 in the effect) has its
//! element pointer at +0x98 and its count (`u16`) at +0x9e of the shader.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, ni_pointer_clear, release_reference, COMPILE_PIXEL_SHADER,
    COMPILE_VERTEX_SHADER, FORMAT_00406D00, IMPORT_INTERLOCKED_INCREMENT, MAKE_PATH,
};

/// The vtables the destructor stores: the shader's (`010bc8f8`) and its effect part's
/// (`010b0318`).
const SHADER_VTABLE_010BC8F8: u32 = 0x010b_c8f8;
const EFFECT_PART_VTABLE_010B0318: u32 = 0x010b_0318;
/// `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`, `thiscall`) and
/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`).
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const BS_SHADER_DESTRUCT_00BE0F10: u32 = 0x00be_0f10;

// Translated from 00c03a20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03a20 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the pointers and the bases) is not translated.
/// `ImageSpaceShader::~ImageSpaceShader` (Xbox PDB), `thiscall`, no stack words: stores the
/// vtables `010bc8f8` (+0) and `010b0318` (+0x6c), releases `spPixelShader` and
/// `spVertexShader` (the `NiPointer` destructors: released, not cleared), then runs
/// `ImageSpaceEffect::~ImageSpaceEffect` on +0x6c and `BSShader::~BSShader`.
pub fn image_space_shader_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_VTABLE_010BC8F8);
    e.mem.set_u32(base + 0x6c, EFFECT_PART_VTABLE_010B0318);
    for slot in [0xc8u32, 0xc4] {
        let shader = e.mem.u32(base + slot);
        if shader != 0 {
            release_reference(e, shader);
        }
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base + 0x6c]);
    e.call(BS_SHADER_DESTRUCT_00BE0F10, &args![base]);
}

/// The compiled-file name formats: `"IS%s.vso"` (`010bca58`) and `"IS%s.pso"` (`010bca4c`).
const VERTEX_NAME_FORMAT_010BCA58: u32 = 0x010b_ca58;
const PIXEL_NAME_FORMAT_010BCA4C: u32 = 0x010b_ca4c;

/// One compile of `00c03ae0`: `MakePath` of the source block's file name into the path
/// buffer, the compiled name (`format` with `name`) into the name buffer, then the compile on
/// the shader (path, the block's macros at +4, profile, name buffer). Answers the compiled
/// shader.
fn compile(
    e: &mut Engine,
    this: u32,
    buffers: u32,
    source: u32,
    format: u32,
    name: u32,
    profile: u32,
    pixel: bool,
) -> u32 {
    let name_buffer = buffers;
    let path_buffer = buffers + 0x104;
    let file = e.mem.u32(source);
    e.call(MAKE_PATH, &args![file, path_buffer]);
    e.call(FORMAT_00406D00, &args![name_buffer, 0x104u32, format, name]);
    let compiler = if pixel {
        COMPILE_PIXEL_SHADER
    } else {
        COMPILE_VERTEX_SHADER
    };
    e.call(
        compiler,
        &args![this, path_buffer, source + 4, profile, name_buffer],
    )
    .u32()
}

// Translated from 00c03ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03ae0 - the /GS stack cookie (011c16bc, checked through 00ec408c) is not modelled.
/// Unnamed in the engine map (unit `imagespaceshader.cpp`; Ghidra made no function of it): the
/// image-space shader's shader loader, `thiscall`, five words: the vertex source block, the
/// pixel source block, the name the compiled files are formatted with, the vertex profile and
/// the pixel profile. Compiles the vertex shader (`MakePath`, `"IS%s.vso"`,
/// `BSShader::CompileVertexShaderHLSL`) into `spVertexShader`; with a pixel profile, the pixel
/// shader (`"IS%s.pso"`, `BSShader::CompilePixelShaderHLSL`) into `spPixelShader`, else
/// `spPixelShader` is released and cleared. A source block is the file name followed by the
/// macros. The two 0x104-byte buffers are the exe's stack buffers.
pub fn fn_00c03ae0(
    e: &mut Engine,
    this: Ptr,
    vertex_source: u32,
    pixel_source: u32,
    name: u32,
    vertex_profile: u32,
    pixel_profile: u32,
) {
    let base = this.addr();
    e.with_stack(0x208, |e, buffers| {
        let buffers = buffers.addr();
        let vertex = compile(
            e,
            base,
            buffers,
            vertex_source,
            VERTEX_NAME_FORMAT_010BCA58,
            name,
            vertex_profile,
            false,
        );
        ni_pointer_assign(e, base + 0xc4, vertex);
        if pixel_profile != 0 {
            let pixel = compile(
                e,
                base,
                buffers,
                pixel_source,
                PIXEL_NAME_FORMAT_010BCA4C,
                name,
                pixel_profile,
                true,
            );
            ni_pointer_assign(e, base + 0xc8, pixel);
        } else {
            ni_pointer_clear(e, base + 0xc8);
        }
    });
}

/// `ImageSpaceTexture::GetTexture` (`00ba3780`, `thiscall`), the renderer (`011f4748`) and
/// `ImageSpaceShaderParam::SetTexOffset` (engine map, `00b8a6e0`, `thiscall` on the
/// parameter: index, width, height).
const TEXTURE_GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const RENDERER_011F4748: u32 = 0x011f_4748;
const SET_TEX_OFFSET_00B8A6E0: u32 = 0x00b8_a6e0;

/// The renderer's slot +0xc8 (it answers a render-target group), that group's slot +0xb0
/// (`NiRenderTargetGroup::GetBuffer`, Xbox PDB) with 0, and the `Ni2DBuffer` word at
/// `offset` (`m_uiWidth` +8, `m_uiHeight` +0xc, Xbox PDB); 0 when the group or the buffer is
/// null.
fn target_size(e: &mut Engine, offset: u32) -> u32 {
    let renderer = e.global::<u32>(RENDERER_011F4748);
    let group = e.vcall(renderer, 0xc8, &args![]).u32();
    if group == 0 {
        return 0;
    }
    let buffer = e.vcall(group, 0xb0, &args![0u32]).u32();
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + offset)
}

// Translated from 00c03c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShader::SetVSConstants` (Xbox PDB), `thiscall`, no stack words: for each texture
/// of the effect part (the count re-read each pass), and only when `pParam` is set, gives the
/// parameter the texture's size: `SetTexOffset(index, width, height)` with the texture's
/// `GetWidth` (+0x94) and `GetHeight` (+0x98, asked first) when the entry and its
/// `GetTexture` are not null, else with the current render target's width and height (the
/// height asked first).
pub fn image_space_shader_set_vs_constants(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base + 0x9e)) {
        let entry = e
            .mem
            .u32(e.mem.u32(base + 0x98).wrapping_add(index.wrapping_mul(4)));
        let texture = if entry != 0 {
            e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![entry]).u32()
        } else {
            0
        };
        let param = e.mem.u32(base + 0xcc);
        if param != 0 {
            let (width, height) = if texture != 0 {
                let height = e.vcall(texture, 0x98, &args![]).u32();
                let width = e.vcall(texture, 0x94, &args![]).u32();
                (width, height)
            } else {
                let height = target_size(e, 0xc);
                let width = target_size(e, 8);
                (width, height)
            };
            let param = e.mem.u32(base + 0xcc);
            e.call(SET_TEX_OFFSET_00B8A6E0, &args![param, index, width, height]);
        }
        index += 1;
    }
}

/// The word `00c03d70` sets (`0126f74c`).
const GLOBAL_0126F74C: u32 = 0x0126_f74c;

// Translated from 00c03d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceshader.cpp`; Ghidra made no function of it),
/// `thiscall`, one word it does not read: copies the word at +0x5c to `0126f74c`.
pub fn fn_00c03d70(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let value = e.mem.u32(this.addr() + 0x5c);
    e.set_global(GLOBAL_0126F74C, value);
}

/// `ImageSpaceTexture::GetRenderedTexture` (`00ba3770`, `thiscall`), the default parameter
/// (`01202410`), the byte `011f9426`, the Direct3D renderer (`011f9508`), the clear colour
/// (0, 0, 0, 1) at `011a9bd0`, the texture manager's word `011f91ac`, the address `011ad840`
/// and the pass's shader number `0x2f6`.
const TEXTURE_GET_RENDERED_00BA3770: u32 = 0x00ba_3770;
const DEFAULT_PARAM_01202410: u32 = 0x0120_2410;
const FLAG_011F9426: u32 = 0x011f_9426;
const RENDERER_011F9508: u32 = 0x011f_9508;
const CLEAR_COLOR_011A9BD0: u32 = 0x011a_9bd0;
const GLOBAL_011F91AC: u32 = 0x011f_91ac;
const TARGET_011AD840: u32 = 0x011a_d840;
const PASS_0X2F6: u32 = 0x2f6;
/// `BSRenderedTexture::StopOffscreen` (engine map, `00b6b260`, `thiscall`), the two target
/// set-ups `00b6b890` and `00b6b8d0` (unit `run_00b6b330`, `cdecl`, two words), `00b6b100`
/// (unnamed, no arguments) and `00b6b790` (unit `run_00b6b330`, no arguments),
/// `BSShaderProperty::RenderPass::RenderPass` (`00ba8ec0`, `cdecl`: the pass, the geometry,
/// the shader number and three words), `BSBatchRenderer::RenderPassImmediately` (`00b994f0`,
/// `cdecl`: the pass, the shader number, three words) and
/// `BSShaderProperty::RenderPass::~RenderPass` (`00ba8cd0`, `thiscall`) (engine map).
const STOP_OFFSCREEN_00B6B260: u32 = 0x00b6_b260;
const TARGETS_00B6B890: u32 = 0x00b6_b890;
const TARGETS_00B6B8D0: u32 = 0x00b6_b8d0;
const CURRENT_TARGET_00B6B100: u32 = 0x00b6_b100;
const RESTORE_TARGET_00B6B790: u32 = 0x00b6_b790;
const RENDER_PASS_CONSTRUCT_00BA8EC0: u32 = 0x00ba_8ec0;
const RENDER_PASS_IMMEDIATELY_00B994F0: u32 = 0x00b9_94f0;
const RENDER_PASS_DESTRUCT_00BA8CD0: u32 = 0x00ba_8cd0;

// Translated from 00c03d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03d80 - the C++ exception frame (FS:[0] chain, unwind state 0 to 1 for the texture reference and the render pass) is not translated.
/// `ImageSpaceShader::Render` (Xbox PDB), `thiscall` on the effect part (+0x6c of the shader),
/// four words: the geometry, a word it does not read, the parameter (`01202410` when null) and
/// a byte. Holds a reference to the first texture's rendered texture (`GetRenderedTexture`)
/// for the call; puts the shader in the geometry's +0xc0; calls the shader's slots +0x50
/// (geometry) and +0x148 (parameter) and `SetVSConstants`. Then the clear and the target:
/// without a rendered texture and with the byte `011f9426` set, clears (Direct3D renderer
/// slots +0xb4 with a zero colour and +0xac with `011a9bd0`) with flags 1; else with flag bit
/// 0 of the parameter's +0x28 the same with the parameter's colour (+0x2c). Unless the
/// renderer `011f4748` has a non-zero +0x200: with a rendered texture `00b6b8d0` (flags,
/// `StopOffscreen` of the texture), without one `00b6b890` (flags, 0). Then a target pointer:
/// the texture manager's +0x308 when the texture is the first word of `011f91ac`'s object,
/// else null, or `011ad840` without a texture and with `011f9426` set when the renderer's
/// slots +0xc8 and +0xcc answer the same; it goes to the renderer's slot +0x190 when its
/// +0x200 is 1 or 2 and its byte +0x208 is 1. Then shader slot +0xf4 (0x2f6), the render pass
/// built (`00ba8ec0`: geometry, 0x2f6, 1, 0, 0) and drawn (`00b994f0`: 0x2f6, 0, 0, 0); with a
/// rendered texture and the byte set, `00b6b790` when `00b6b100` differs from the renderer's
/// slot +0xc8; with flag bit 0 the zero colour goes to slot +0xac; the pass is destroyed and
/// the reference released.
pub fn image_space_shader_render(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    _unused_1: u32,
    param: u32,
    restore: u8,
) {
    let param = if param != 0 {
        param
    } else {
        DEFAULT_PARAM_01202410
    };
    let first = e.mem.u32(e.mem.u32(this.addr() + 0x2c));
    let texture = e.call(TEXTURE_GET_RENDERED_00BA3770, &args![first]).u32();
    if texture != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    let shader = this.addr().wrapping_sub(0x6c);
    e.mem.set_u32(geometry + 0xc0, shader);
    e.vcall(shader, 0x50, &args![geometry]);
    e.vcall(shader, 0x148, &args![param]);
    image_space_shader_set_vs_constants(e, Ptr::new(shader));
    // The frame's locals: the zero colour (four floats) at +0 and the render pass at +0x10.
    e.with_stack(0x20, |e, frame| {
        let color = frame.addr();
        let pass = color + 0x10;
        for offset in [0u32, 4, 8, 0xc] {
            e.mem.set_u32(color + offset, 0);
        }
        let mut flags = e.mem.u32(param + 0x28);
        let cleared_black = texture == 0 && e.global::<u8>(FLAG_011F9426) != 0;
        // `true` where the code reaches the `00b6b890` call (0x00c03e65), `false` where it
        // goes on with `00b6b8d0` or to the target pointer.
        let plain_targets = if cleared_black {
            let renderer = e.global::<u32>(RENDERER_011F9508);
            e.vcall(renderer, 0xb4, &args![color]);
            flags = 1;
            let renderer = e.global::<u32>(RENDERER_011F9508);
            e.vcall(renderer, 0xac, &args![CLEAR_COLOR_011A9BD0]);
            true
        } else {
            if flags & 1 != 0 {
                let renderer = e.global::<u32>(RENDERER_011F9508);
                e.vcall(renderer, 0xb4, &args![color]);
                let renderer = e.global::<u32>(RENDERER_011F9508);
                e.vcall(renderer, 0xac, &args![param + 0x2c]);
            }
            texture == 0
        };
        let renderer = e.global::<u32>(RENDERER_011F4748);
        if e.mem.u32(renderer + 0x200) == 0 {
            if plain_targets {
                e.call(TARGETS_00B6B890, &args![flags, 0u32]);
            } else {
                let stopped = e.call(STOP_OFFSCREEN_00B6B260, &args![texture]).u32();
                e.call(TARGETS_00B6B8D0, &args![flags, stopped]);
            }
        }
        let renderer = e.global::<u32>(RENDERER_011F4748);
        let manager = e.global::<u32>(GLOBAL_011F91AC);
        let mut target = if texture == e.mem.u32(manager) {
            manager + 0x308
        } else {
            0
        };
        if texture == 0 && e.global::<u8>(FLAG_011F9426) != 0 {
            let current = e.vcall(renderer, 0xc8, &args![]).u32();
            let other = e.vcall(renderer, 0xcc, &args![]).u32();
            if current == other {
                target = TARGET_011AD840;
            }
        }
        let renderer = e.global::<u32>(RENDERER_011F4748);
        let mode = e.mem.u32(renderer + 0x200);
        if (mode == 1 || mode == 2) && e.mem.u8(renderer + 0x208) == 1 {
            e.vcall(renderer, 0x190, &args![target]);
        }
        e.vcall(shader, 0xf4, &args![PASS_0X2F6]);
        e.call(
            RENDER_PASS_CONSTRUCT_00BA8EC0,
            &args![pass, geometry, PASS_0X2F6, 1u32, 0u32, 0u32],
        );
        e.call(
            RENDER_PASS_IMMEDIATELY_00B994F0,
            &args![pass, PASS_0X2F6, 0u32, 0u32, 0u32],
        );
        if texture != 0 && restore != 0 {
            let renderer = e.global::<u32>(RENDERER_011F4748);
            let current = e.vcall(renderer, 0xc8, &args![]).u32();
            let target = e.call(CURRENT_TARGET_00B6B100, &args![]).u32();
            if target != current {
                e.call(RESTORE_TARGET_00B6B790, &args![]);
            }
        }
        if e.mem.u8(param + 0x28) & 1 != 0 {
            let renderer = e.global::<u32>(RENDERER_011F9508);
            e.vcall(renderer, 0xac, &args![color]);
        }
        e.call(RENDER_PASS_DESTRUCT_00BA8CD0, &args![pass]);
    });
    if texture != 0 {
        release_reference(e, texture);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_3a20, image_space_shader_destructor(Ptr)),
        entry!(0x00c0_3ae0, fn_00c03ae0(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00c0_3c80, image_space_shader_set_vs_constants(Ptr)),
        entry!(0x00c0_3d70, fn_00c03d70(Ptr, u32)),
        entry!(
            0x00c0_3d80,
            image_space_shader_render(Ptr, u32, u32, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::IMPORT_INTERLOCKED_DECREMENT;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// An object at a fresh address whose vtable's slot `offset` is `base + offset` for each
    /// given offset (each registered as a double answering 0).
    fn with_slots(e: &mut Engine, vtable: u32, base: u32, offsets: &[u32]) -> u32 {
        let top = offsets.iter().max().copied().unwrap_or(0);
        let mut slots = vec![0u32; (top / 4 + 1) as usize];
        for offset in offsets {
            slots[(offset / 4) as usize] = base + offset;
            e.register(base + offset, |_, _| Ret::default());
        }
        e.put_vtable(vtable, &slots);
        let object = e.mem.alloc(0x210);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Answers `value` from `addr`.
    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    #[test]
    fn fn_00c03a20_releases_both_shaders_and_runs_the_bases() {
        let mut e = rig();
        e.register(EFFECT_DESTRUCT_00BA3D30, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT_00BE0F10, |_, _| Ret::default());
        let this = e.mem.alloc(0xd0);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 2);
        e.mem.set_u32(this + 0xc4, vertex);
        e.mem.set_u32(this + 0xc8, pixel);
        e.call(0x00c0_3a20, &args![this]);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BC8F8);
        assert_eq!(e.mem.u32(this + 0x6c), EFFECT_PART_VTABLE_010B0318);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![pixel + 4], vec![vertex + 4]]
        );
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex]]);
        assert_eq!(e.mem.u32(this + 0xc4), vertex);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this + 0x6c]]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT_00BE0F10), vec![vec![this]]);
        // Null pointers: nothing released.
        e.mem.set_u32(this + 0xc4, 0);
        e.mem.set_u32(this + 0xc8, 0);
        e.call(0x00c0_3a20, &args![this]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 2);
    }

    /// The rig with `MakePath`, the formatter and the compiles doubled (answering `vertex` and
    /// `pixel`).
    fn loader(vertex: u32, pixel: u32) -> Engine {
        let mut e = rig();
        e.register(MAKE_PATH, |_, _| Ret::default());
        e.register(FORMAT_00406D00, |_, _| Ret::default());
        answer(&mut e, COMPILE_VERTEX_SHADER, vertex);
        answer(&mut e, COMPILE_PIXEL_SHADER, pixel);
        e
    }

    #[test]
    fn fn_00c03ae0_compiles_both_shaders() {
        let mut e = loader(0, 0);
        let vertex = object(&mut e, 0);
        let pixel = object(&mut e, 0);
        answer(&mut e, COMPILE_VERTEX_SHADER, vertex);
        answer(&mut e, COMPILE_PIXEL_SHADER, pixel);
        let this = e.mem.alloc(0xd0);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0xc8, old);
        let vertex_source = e.mem.alloc(0x10);
        let pixel_source = e.mem.alloc(0x10);
        e.mem.set_u32(vertex_source, 0x5000);
        e.mem.set_u32(pixel_source, 0x6000);
        e.call(
            0x00c0_3ae0,
            &args![
                this,
                vertex_source,
                pixel_source,
                0x7000u32,
                0x8000u32,
                0x9000u32
            ],
        );
        let makes = calls(&e, MAKE_PATH);
        assert_eq!(makes.len(), 2);
        assert_eq!((makes[0][0], makes[1][0]), (0x5000, 0x6000));
        let path = makes[0][1];
        let formats = calls(&e, FORMAT_00406D00);
        let name = formats[0][0];
        assert_eq!(path, name + 0x104);
        assert_eq!(
            formats,
            vec![
                vec![name, 0x104, VERTEX_NAME_FORMAT_010BCA58, 0x7000],
                vec![name, 0x104, PIXEL_NAME_FORMAT_010BCA4C, 0x7000],
            ]
        );
        assert_eq!(
            calls(&e, COMPILE_VERTEX_SHADER),
            vec![vec![this, path, vertex_source + 4, 0x8000, name]]
        );
        assert_eq!(
            calls(&e, COMPILE_PIXEL_SHADER),
            vec![vec![this, path, pixel_source + 4, 0x9000, name]]
        );
        assert_eq!(e.mem.u32(this + 0xc4), vertex);
        assert_eq!(e.mem.u32(this + 0xc8), pixel);
        assert_eq!(e.mem.u32(vertex + 4), 1);
        assert_eq!(e.mem.u32(pixel + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
    }

    #[test]
    fn fn_00c03ae0_without_a_pixel_profile_clears_the_pixel_shader() {
        let mut e = loader(0, 0);
        let this = e.mem.alloc(0xd0);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0xc8, old);
        let source = e.mem.alloc(0x10);
        e.call(
            0x00c0_3ae0,
            &args![this, source, source, 0x7000u32, 0x8000u32, 0u32],
        );
        assert_eq!(calls(&e, MAKE_PATH).len(), 1);
        assert!(calls(&e, COMPILE_PIXEL_SHADER).is_empty());
        assert_eq!(e.mem.u32(this + 0xc4), 0);
        assert_eq!(e.mem.u32(this + 0xc8), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
    }

    const TEXTURE_SLOTS: u32 = 0x0e30_0000;
    const RENDERER_SLOTS: u32 = 0x0e31_0000;
    const GROUP_SLOTS: u32 = 0x0e32_0000;

    /// A renderer (`011f4748`) whose slot +0xc8 answers a group whose `GetBuffer` answers a
    /// 640 x 480 buffer.
    fn renderer_rig(e: &mut Engine) -> u32 {
        e.map(0x011f_4000, 0x1000);
        let renderer = with_slots(e, 0x0e33_0000, RENDERER_SLOTS, &[0xc8, 0xcc, 0x190]);
        e.set_global(RENDERER_011F4748, renderer);
        let group = with_slots(e, 0x0e33_1000, GROUP_SLOTS, &[0xb0]);
        let buffer = e.mem.alloc(0x14);
        e.mem.set_u32(buffer + 8, 640);
        e.mem.set_u32(buffer + 0xc, 480);
        answer(e, RENDERER_SLOTS + 0xc8, group);
        answer(e, GROUP_SLOTS + 0xb0, buffer);
        renderer
    }

    #[test]
    fn fn_00c03c80_gives_each_texture_its_size() {
        let mut e = rig();
        renderer_rig(&mut e);
        e.register(SET_TEX_OFFSET_00B8A6E0, |_, _| Ret::default());
        let texture = with_slots(&mut e, 0x0e33_2000, TEXTURE_SLOTS, &[0x94, 0x98]);
        answer(&mut e, TEXTURE_SLOTS + 0x94, 256);
        answer(&mut e, TEXTURE_SLOTS + 0x98, 128);
        // Entry 0 has a texture, entry 1's `GetTexture` answers null, entry 2 is null.
        e.register_double(TEXTURE_GET_TEXTURE_00BA3780, move |_, a| {
            (if a[0] == 0x4000 { texture } else { 0 }).into_ret()
        });
        let shader = e.mem.alloc(0xd0);
        let entries = e.mem.alloc(0xc);
        e.mem.set_u32(entries, 0x4000);
        e.mem.set_u32(entries + 4, 0x4100);
        e.mem.set_u32(shader + 0x98, entries);
        e.mem.set_u16(shader + 0x9e, 3);
        e.mem.set_u32(shader + 0xcc, 0x0e34_0000);
        e.call(0x00c0_3c80, &args![shader]);
        assert_eq!(
            calls(&e, SET_TEX_OFFSET_00B8A6E0),
            vec![
                vec![0x0e34_0000, 0, 256, 128],
                vec![0x0e34_0000, 1, 640, 480],
                vec![0x0e34_0000, 2, 640, 480],
            ]
        );
        assert_eq!(
            calls(&e, TEXTURE_GET_TEXTURE_00BA3780),
            vec![vec![0x4000], vec![0x4100]]
        );
        assert_eq!(calls(&e, GROUP_SLOTS + 0xb0).len(), 4);
    }

    #[test]
    fn fn_00c03c80_without_a_parameter_or_a_target_buffer() {
        let mut e = rig();
        renderer_rig(&mut e);
        e.register(SET_TEX_OFFSET_00B8A6E0, |_, _| Ret::default());
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |_, _| Ret::default());
        let shader = e.mem.alloc(0xd0);
        let entries = e.mem.alloc(4);
        e.mem.set_u32(shader + 0x98, entries);
        e.mem.set_u16(shader + 0x9e, 1);
        e.call(0x00c0_3c80, &args![shader]);
        assert!(calls(&e, SET_TEX_OFFSET_00B8A6E0).is_empty());
        assert!(calls(&e, RENDERER_SLOTS + 0xc8).is_empty());
        // A group without a buffer: width and height 0.
        e.mem.set_u32(shader + 0xcc, 0x0e34_0000);
        answer(&mut e, GROUP_SLOTS + 0xb0, 0);
        e.call(0x00c0_3c80, &args![shader]);
        assert_eq!(
            calls(&e, SET_TEX_OFFSET_00B8A6E0),
            vec![vec![0x0e34_0000, 0, 0, 0]]
        );
        // No group at all, and an empty array.
        answer(&mut e, RENDERER_SLOTS + 0xc8, 0);
        e.call(0x00c0_3c80, &args![shader]);
        assert_eq!(
            calls(&e, SET_TEX_OFFSET_00B8A6E0)[1],
            vec![0x0e34_0000, 0, 0, 0]
        );
        e.mem.set_u16(shader + 0x9e, 0);
        e.call(0x00c0_3c80, &args![shader]);
        assert_eq!(calls(&e, SET_TEX_OFFSET_00B8A6E0).len(), 2);
    }

    #[test]
    fn fn_00c03d70_copies_the_word_at_0x5c() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x5c, 0x1234_5678);
        e.call(0x00c0_3d70, &args![this, 9u32]);
        assert_eq!(e.global::<u32>(GLOBAL_0126F74C), 0x1234_5678);
    }

    const SHADER_SLOTS: u32 = 0x0e35_0000;
    const D3D_SLOTS: u32 = 0x0e36_0000;

    /// What `Render` needs: the shader (no textures for `SetVSConstants`), its effect part's
    /// first texture, the parameter with `flags` and a colour, the geometry, the renderers
    /// and the manager word, and every callee doubled. Answers the engine, the effect part,
    /// the geometry, the parameter and the shader.
    fn render_rig(rendered: u32, flags: u32) -> (Engine, u32, u32, u32, u32) {
        let mut e = rig();
        let renderer = renderer_rig(&mut e);
        e.map(0x011f_9000, 0x1000);
        let d3d = with_slots(&mut e, 0x0e33_3000, D3D_SLOTS, &[0xac, 0xb4]);
        e.set_global(RENDERER_011F9508, d3d);
        let manager = e.mem.alloc(0x310);
        e.set_global(GLOBAL_011F91AC, manager);
        e.mem.set_u32(renderer + 0x200, 0);
        let shader = with_slots(&mut e, 0x0e33_4000, SHADER_SLOTS, &[0x50, 0xf4, 0x148]);
        let textures = e.mem.alloc(4);
        e.mem.set_u32(textures, 0x4000);
        e.mem.set_u32(shader + 0x6c + 0x2c, textures);
        answer(&mut e, TEXTURE_GET_RENDERED_00BA3770, rendered);
        for callee in [
            TARGETS_00B6B890,
            TARGETS_00B6B8D0,
            RESTORE_TARGET_00B6B790,
            RENDER_PASS_CONSTRUCT_00BA8EC0,
            RENDER_PASS_IMMEDIATELY_00B994F0,
            RENDER_PASS_DESTRUCT_00BA8CD0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        answer(&mut e, STOP_OFFSCREEN_00B6B260, 0x0e37_0000);
        answer(&mut e, CURRENT_TARGET_00B6B100, 0x0e37_0100);
        let param = e.mem.alloc(0x40);
        e.mem.set_u32(param + 0x28, flags);
        let geometry = e.mem.alloc(0x100);
        (e, shader + 0x6c, geometry, param, shader)
    }

    #[test]
    fn fn_00c03d80_clears_to_black_without_a_rendered_texture() {
        let (mut e, this, geometry, param, shader) = render_rig(0, 0);
        e.set_global(FLAG_011F9426, 1u8);
        let renderer = e.global::<u32>(RENDERER_011F4748);
        e.mem.set_u32(renderer + 0x200, 1);
        e.mem.set_u8(renderer + 0x208, 1);
        // The renderer's slots +0xc8 and +0xcc answer the same group.
        let group = e.call(RENDERER_SLOTS + 0xc8, &args![renderer]).u32();
        answer(&mut e, RENDERER_SLOTS + 0xcc, group);
        e.call(0x00c0_3d80, &args![this, geometry, 0u32, param, 1u32]);
        assert_eq!(e.mem.u32(geometry + 0xc0), shader);
        assert_eq!(calls(&e, SHADER_SLOTS + 0x50), vec![vec![shader, geometry]]);
        assert_eq!(calls(&e, SHADER_SLOTS + 0x148), vec![vec![shader, param]]);
        let d3d = e.global::<u32>(RENDERER_011F9508);
        let clears = calls(&e, D3D_SLOTS + 0xb4);
        assert_eq!(clears.len(), 1);
        let color = clears[0][1];
        assert_eq!(
            calls(&e, D3D_SLOTS + 0xac),
            vec![vec![d3d, CLEAR_COLOR_011A9BD0]]
        );
        // +0x200 is not 0: no target set-up.
        assert!(calls(&e, TARGETS_00B6B890).is_empty());
        assert_eq!(
            calls(&e, RENDERER_SLOTS + 0x190),
            vec![vec![renderer, TARGET_011AD840]]
        );
        assert_eq!(calls(&e, SHADER_SLOTS + 0xf4), vec![vec![shader, 0x2f6]]);
        let pass = color + 0x10;
        assert_eq!(
            calls(&e, RENDER_PASS_CONSTRUCT_00BA8EC0),
            vec![vec![pass, geometry, 0x2f6, 1, 0, 0]]
        );
        assert_eq!(
            calls(&e, RENDER_PASS_IMMEDIATELY_00B994F0),
            vec![vec![pass, 0x2f6, 0, 0, 0]]
        );
        assert_eq!(calls(&e, RENDER_PASS_DESTRUCT_00BA8CD0), vec![vec![pass]]);
        assert!(calls(&e, CURRENT_TARGET_00B6B100).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00c03d80_renders_into_a_rendered_texture() {
        let (mut e, this, geometry, param, _) = render_rig(0, 1);
        let texture = object(&mut e, 1);
        answer(&mut e, TEXTURE_GET_RENDERED_00BA3770, texture);
        let manager = e.global::<u32>(GLOBAL_011F91AC);
        e.mem.set_u32(manager, texture);
        e.call(0x00c0_3d80, &args![this, geometry, 0u32, param, 1u32]);
        let d3d = e.global::<u32>(RENDERER_011F9508);
        let color = calls(&e, D3D_SLOTS + 0xb4)[0][1];
        assert_eq!(
            calls(&e, D3D_SLOTS + 0xac),
            vec![vec![d3d, param + 0x2c], vec![d3d, color]]
        );
        assert_eq!(calls(&e, STOP_OFFSCREEN_00B6B260), vec![vec![texture]]);
        assert_eq!(calls(&e, TARGETS_00B6B8D0), vec![vec![1, 0x0e37_0000]]);
        assert!(calls(&e, TARGETS_00B6B890).is_empty());
        // Mode 0: the target pointer (manager + 0x308) is not handed over.
        assert!(calls(&e, RENDERER_SLOTS + 0x190).is_empty());
        assert_eq!(calls(&e, CURRENT_TARGET_00B6B100).len(), 1);
        assert_eq!(calls(&e, RESTORE_TARGET_00B6B790).len(), 1);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![texture + 4]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![texture + 4]]
        );
        assert_eq!(e.mem.u32(texture + 4), 1);
    }

    #[test]
    fn fn_00c03d80_uses_the_default_parameter_and_plain_targets() {
        let (mut e, this, geometry, _, _) = render_rig(0, 0);
        e.map(0x0120_2000, 0x1000);
        e.call(0x00c0_3d80, &args![this, geometry, 0u32, 0u32, 0u32]);
        let shader = this - 0x6c;
        assert_eq!(
            calls(&e, SHADER_SLOTS + 0x148),
            vec![vec![shader, DEFAULT_PARAM_01202410]]
        );
        assert!(calls(&e, D3D_SLOTS + 0xb4).is_empty());
        assert_eq!(calls(&e, TARGETS_00B6B890), vec![vec![0, 0]]);
        assert!(calls(&e, RENDERER_SLOTS + 0xcc).is_empty());
    }
}
