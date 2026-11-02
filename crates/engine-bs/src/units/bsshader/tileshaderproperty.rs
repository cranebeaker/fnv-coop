//! `BSShader/tileshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Ghidra has no function at `00bb7b60` (vtable slot +0x4c, `LoadBinary`, of the vtable at
//! `010b9d28`); its translation follows the instruction bytes read from the executable.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::stream_read_word;

layout! {
    /// `TileShaderProperty` (Xbox PDB) on PC, 0xb0 bytes: the PDB's fields 8 bytes lower
    /// (the PC `BSShaderProperty` is 0x60 bytes, the PDB's 0x68), as the constructor
    /// (`00bb7c30`) writes them.
    pub struct TileShaderProperty: 0xb0 {
        /// `spTileTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x60 spTileTexture: u32,
        /// `spAlphaMap` (Xbox PDB): `NiPointer<NiTexture>`.
        0x64 spAlphaMap: u32,
        /// `TintColor` (Xbox PDB): `NiColorA`, red.
        0x68 TintColor_r: u32,
        0x6c TintColor_g: u32,
        0x70 TintColor_b: u32,
        0x74 TintColor_a: u32,
        /// `fAlpha` (Xbox PDB).
        0x78 fAlpha: u32,
        /// `fTexScrollX` (Xbox PDB).
        0x7c fTexScrollX: u32,
        /// `fTexScrollY` (Xbox PDB).
        0x80 fTexScrollY: u32,
        /// `fTexScaleX` (Xbox PDB).
        0x84 fTexScaleX: u32,
        /// `fTexScaleY` (Xbox PDB).
        0x88 fTexScaleY: u32,
        /// `TexClampMode` (Xbox PDB).
        0x8c TexClampMode: u32,
        /// `bNoBlend` (Xbox PDB).
        0x90 bNoBlend: u8,
        /// `bUseTexScroll` (Xbox PDB).
        0x91 bUseTexScroll: u8,
        /// `bUseVertexColors` (Xbox PDB).
        0x92 bUseVertexColors: u8,
        /// `bUseTexturingProperty` (Xbox PDB).
        0x93 bUseTexturingProperty: u8,
        /// `TextureFilename.pString` (Xbox PDB, a `BSStringT<char>`).
        0x94 TextureFilename_pString: u32,
        /// `TextureFilename.sLen`.
        0x98 TextureFilename_sLen: u16,
        /// `TextureFilename.sMaxLen`.
        0x9a TextureFilename_sMaxLen: u16,
        /// `bUseScissor` (Xbox PDB), after the 16-byte `ScissorRect` at +0x9c.
        0xac bUseScissor: u8,
    }
}

/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InterlockedIncrement` (KERNEL32 import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `BSShaderProperty::BSShaderProperty` (`00ba8e50`, `thiscall`).
const BS_SHADER_PROPERTY_CONSTRUCTOR: u32 = 0x00ba_8e50;
/// The vtable the constructor installs (`010b9d28`).
const TILE_SHADER_PROPERTY_VTABLE: u32 = 0x010b_9d28;
/// The shader property's own `LoadBinary` (`00ba92c0`, `thiscall`, one word: the stream).
const SHADER_PROPERTY_LOAD_BINARY: u32 = 0x00ba_92c0;
/// `NiStream::LoadCString` (engine map, `00a64ce0`, `thiscall` on the stream, one word: the
/// address of the string pointer to fill).
const NI_STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
/// The string assignment `004037f0` (`thiscall` on a `BSStringT`, two words: the text and 0).
const BS_STRING_SET_004037F0: u32 = 0x0040_37f0;
/// The free `00aa10f0` (`cdecl`, one word).
const FREE_00AA10F0: u32 = 0x00aa_10f0;

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00bb7a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileShaderProperty::SetTileTexture` (engine map; the body matches): the `NiPointer`
/// assignment of `spTileTexture`. Nothing for the texture it already holds; otherwise the old
/// one is released and the new one (when non-null) gets a reference.
pub fn tile_shader_property_set_tile_texture(
    e: &mut Engine,
    this: Ptr<TileShaderProperty>,
    texture: u32,
) {
    let old = e.get(this, TileShaderProperty::spTileTexture);
    if old == texture {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    e.set(this, TileShaderProperty::spTileTexture, texture);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture.wrapping_add(4)]);
    }
}

// Translated from 00bb7b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's `LoadBinary` (vtable slot +0x4c of `010b9d28`): `thiscall`, one
/// word (the stream). The shader property's load (`00ba92c0`), one word into `TexClampMode`
/// through the stream's file, then the texture file name: `NiStream::LoadCString` into a
/// local (cleared first; the exe reuses the argument's stack slot), assigned to
/// `TextureFilename` (`004037f0(text, 0)`), and the loaded text freed (`00aa10f0`).
pub fn fn_00bb7b60(e: &mut Engine, this: Ptr<TileShaderProperty>, stream: u32) {
    e.call(SHADER_PROPERTY_LOAD_BINARY, &args![this, stream]);
    stream_read_word(e, stream, this.addr().wrapping_add(0x8c));
    e.with_stack(4, |e, text| {
        e.mem.set_u32(text.addr(), 0);
        e.call(NI_STREAM_LOAD_C_STRING, &args![stream, text]);
        let loaded = e.mem.u32(text.addr());
        let name = this.addr().wrapping_add(0x94);
        e.call(BS_STRING_SET_004037F0, &args![name, loaded, 0u32]);
        let loaded = e.mem.u32(text.addr());
        e.call(FREE_00AA10F0, &args![loaded]);
    });
}

// Translated from 00bb7c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileShaderProperty::TileShaderProperty` (engine map; the body matches): `thiscall`, one
/// word (`bUseTexturingProperty`, its low byte). The shader property's constructor
/// (`00ba8e50`), the vtable `010b9d28`, both textures null (each released first when set,
/// which cannot happen right after the clear), the tint colour, `fAlpha` and both texture
/// scales 1.0, both scrolls 0.0, `TexClampMode` 3, the flags cleared, an empty file name,
/// `bUseScissor` 0, and on the shader property `iShader` 0x20 and `iShaderPropertyType`
/// 0x13. Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7c30 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn tile_shader_property_tile_shader_property(
    e: &mut Engine,
    this: Ptr<TileShaderProperty>,
    use_texturing_property: u8,
) -> Ptr<TileShaderProperty> {
    e.call(BS_SHADER_PROPERTY_CONSTRUCTOR, &args![this]);
    let zero = 0.0f32.to_bits();
    let one = 1.0f32.to_bits();
    e.mem.set_u32(this.addr(), TILE_SHADER_PROPERTY_VTABLE);
    e.set(this, TileShaderProperty::spTileTexture, 0);
    e.set(this, TileShaderProperty::spAlphaMap, 0);
    e.set(this, TileShaderProperty::TintColor_r, zero);
    e.set(this, TileShaderProperty::TintColor_g, zero);
    e.set(this, TileShaderProperty::TintColor_b, zero);
    e.set(this, TileShaderProperty::TintColor_a, zero);
    e.set(this, TileShaderProperty::TextureFilename_pString, 0);
    e.set(this, TileShaderProperty::TextureFilename_sLen, 0);
    e.set(this, TileShaderProperty::TextureFilename_sMaxLen, 0);
    let texture = e.get(this, TileShaderProperty::spTileTexture);
    if texture != 0 {
        release_reference(e, texture);
        e.set(this, TileShaderProperty::spTileTexture, 0);
    }
    let alpha_map = e.get(this, TileShaderProperty::spAlphaMap);
    if alpha_map != 0 {
        release_reference(e, alpha_map);
        e.set(this, TileShaderProperty::spAlphaMap, 0);
    }
    e.set(this, TileShaderProperty::TintColor_r, one);
    e.set(this, TileShaderProperty::TexClampMode, 3);
    e.set(this, TileShaderProperty::TintColor_g, one);
    e.set(
        this,
        TileShaderProperty::bUseTexturingProperty,
        use_texturing_property,
    );
    e.set(this, TileShaderProperty::TintColor_b, one);
    e.set(this, TileShaderProperty::bNoBlend, 0);
    e.set(this, TileShaderProperty::TintColor_a, one);
    e.set(this, TileShaderProperty::bUseTexScroll, 0);
    e.set(this, TileShaderProperty::fAlpha, one);
    // BSShaderProperty::iShader (+0x58) and iShaderPropertyType (+0x1c).
    e.mem.set_u32(this.addr().wrapping_add(0x58), 0x20);
    e.mem.set_u32(this.addr().wrapping_add(0x1c), 0x13);
    e.set(this, TileShaderProperty::fTexScrollY, zero);
    e.set(this, TileShaderProperty::bUseScissor, 0);
    e.set(this, TileShaderProperty::fTexScrollX, zero);
    e.set(this, TileShaderProperty::bUseVertexColors, 0);
    e.set(this, TileShaderProperty::fTexScaleY, one);
    e.set(this, TileShaderProperty::fTexScaleX, one);
    this
}

/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, thiscall: a word, `RET 4`)
/// and `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, thiscall on the array, `RET
/// 0x20`: geometry, pass number, then six words), engine map names.
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The base member copy (`00ba8b10`, thiscall: destination and a word, `RET 8`), the base
/// `SaveBinary` (`00ba9380`, thiscall: stream) and `00a64160` (thiscall on the stream: a
/// C string, the file name's text).
const BASE_COPY_MEMBERS: u32 = 0x00ba_8b10;
const BASE_SAVE_BINARY: u32 = 0x00ba_9380;
const STREAM_SAVE_C_STRING_00A64160: u32 = 0x00a6_4160;

// Translated from 00bb7a60 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `TileShaderProperty::GetRenderPasses` (the vtable's slot +0xa0, `GetRenderPasses` in the
/// Xbox PDB; `thiscall`, `RET 0x18`: the geometry, a pass word, an unread word, the pass
/// type and two unread words). Makes the render pass list when needed
/// (`00ba95e0` with 1); when it is empty, adds pass 0x42 for the geometry (`00ba9ee0` with
/// 1 and five zeros), marks the last pass's `eAccumulationHint` byte (+6) 1 and stores
/// `pass_type << 8 | pass_word` in `iLastRenderPassState`. Answers the list.
pub fn tile_shader_property_get_render_passes(
    e: &mut Engine,
    this: Ptr<TileShaderProperty>,
    geometry: u32,
    pass_word: u32,
    _unused_3: u32,
    pass_type: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    use crate::units::bsshader::bsshaderproperty::BSShaderProperty;
    let property = Ptr::<BSShaderProperty>::new(this.addr());
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 1u32]);
    let list = e.get(property, BSShaderProperty::pRenderPassList);
    // `RenderPassArray::iSize` (+0x10) and its buffer (+4).
    if e.mem.u32(list.wrapping_add(0x10)) != 0 {
        return list;
    }
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, 0x42u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let list = e.get(property, BSShaderProperty::pRenderPassList);
    let count = e.mem.u32(list.wrapping_add(0x10));
    let buffer = e.mem.u32(list.wrapping_add(4));
    let last = e
        .mem
        .u32(buffer.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
    let key = (pass_type << 8) | pass_word;
    e.mem.set_u8(last.wrapping_add(6), 1);
    let list = e.get(property, BSShaderProperty::pRenderPassList);
    e.set(property, BSShaderProperty::iLastRenderPassState, key as i32);
    list
}

// Translated from 00bb7ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's member copy (unnamed in the engine map; called by the copy
/// at `00bb7e70`; thiscall, `RET 8`: destination and a word): the base copy (`00ba8b10`), the
/// `NiPointer` assignment of `spTileTexture` (inlined: nothing for the same texture, else the
/// old one released and the new one referenced), then the tint colour, `TexClampMode`,
/// `fAlpha` (through `FLD`/`FSTP`, quieting a signalling NaN), `bUseTexturingProperty` and
/// `bNoBlend`.
pub fn fn_00bb7ac0(
    e: &mut Engine,
    this: Ptr<TileShaderProperty>,
    destination: Ptr<TileShaderProperty>,
    word: u32,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, destination, word]);
    let old = e.get(destination, TileShaderProperty::spTileTexture);
    if old != e.get(this, TileShaderProperty::spTileTexture) {
        if old != 0 {
            release_reference(e, old);
        }
        let texture = e.get(this, TileShaderProperty::spTileTexture);
        e.set(destination, TileShaderProperty::spTileTexture, texture);
        if texture != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![texture.wrapping_add(4)]);
        }
    }
    for field in [
        TileShaderProperty::TintColor_r,
        TileShaderProperty::TintColor_g,
        TileShaderProperty::TintColor_b,
        TileShaderProperty::TintColor_a,
        TileShaderProperty::TexClampMode,
    ] {
        let value = e.get(this, field);
        e.set(destination, field, value);
    }
    let alpha = x87_quiet_bits(e.get(this, TileShaderProperty::fAlpha));
    e.set(destination, TileShaderProperty::fAlpha, alpha);
    let texturing = e.get(this, TileShaderProperty::bUseTexturingProperty);
    e.set(
        destination,
        TileShaderProperty::bUseTexturingProperty,
        texturing,
    );
    let no_blend = e.get(this, TileShaderProperty::bNoBlend);
    e.set(destination, TileShaderProperty::bNoBlend, no_blend);
}

// Translated from 00bb7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileShaderProperty::SaveBinary` (the vtable's slot +0x58; thiscall, `RET 4`): the base
/// `SaveBinary` (`00ba9380`), `TexClampMode` (4 bytes, the element size in the argument's
/// stack word) through the stream's output (+0x250, write function at +0xc), then the texture
/// file name's text through `00a64160`.
pub fn tile_shader_property_save_binary(
    e: &mut Engine,
    this: Ptr<TileShaderProperty>,
    stream: u32,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let output = e.mem.u32(stream.wrapping_add(0x250));
    let write = e.mem.u32(output.wrapping_add(0xc));
    let wrote = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(
            write,
            &args![output, this.addr().wrapping_add(0x8c), 4u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(output.wrapping_add(4)).wrapping_add(wrote);
    e.mem.set_u32(output.wrapping_add(4), position);
    let text = e.get(this, TileShaderProperty::TextureFilename_pString);
    e.call(STREAM_SAVE_C_STRING_00A64160, &args![stream, text]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bb_7a60,
            tile_shader_property_get_render_passes(
                Ptr<TileShaderProperty>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00bb_7ac0,
            fn_00bb7ac0(Ptr<TileShaderProperty>, Ptr<TileShaderProperty>, u32)
        ),
        entry!(
            0x00bb_7be0,
            tile_shader_property_save_binary(Ptr<TileShaderProperty>, u32)
        ),
        entry!(
            0x00bb_7a10,
            tile_shader_property_set_tile_texture(Ptr<TileShaderProperty>, u32)
        ),
        entry!(0x00bb_7b60, fn_00bb7b60(Ptr<TileShaderProperty>, u32)),
        entry!(
            0x00bb_7c30,
            tile_shader_property_tile_shader_property(
                Ptr<TileShaderProperty>,
                u8,
            ) -> Ptr<TileShaderProperty>
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn texture(e: &mut Engine, references: u32) -> u32 {
        let t = e.mem.alloc(8);
        e.mem.set_u32(t, 0x0f00_1000);
        e.mem.set_u32(t + 4, references);
        t
    }

    #[test]
    fn set_tile_texture_moves_the_reference() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        let first = texture(&mut e, 1);
        let second = texture(&mut e, 5);
        e.call(0x00bb_7a10, &args![this, first]);
        assert_eq!((e.mem.u32(this + 0x60), e.mem.u32(first + 4)), (first, 2));
        e.call(0x00bb_7a10, &args![this, first]);
        assert_eq!(e.mem.u32(first + 4), 2);
        e.mem.set_u32(first + 4, 1);
        e.call(0x00bb_7a10, &args![this, second]);
        assert_eq!(e.mem.u32(this + 0x60), second);
        assert_eq!(e.mem.u32(second + 4), 6);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![first]]);
        e.call(0x00bb_7a10, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(second + 4), 5);
    }

    #[test]
    fn load_binary_reads_clamp_mode_and_file_name() {
        let mut e = rig();
        e.register(SHADER_PROPERTY_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x0f00_0100, |e, a| {
            e.mem.set_u32(a[1], 2);
            4u32.into_ret()
        });
        e.register(NI_STREAM_LOAD_C_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x9000);
            Ret::default()
        });
        e.register(BS_STRING_SET_004037F0, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0f00_0100);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0xb0);
        e.call(0x00bb_7b60, &args![this, stream]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x8c), 2);
        assert_eq!(e.mem.u32(file + 4), 4);
        assert_eq!(calls(&e, NI_STREAM_LOAD_C_STRING)[0][0], stream);
        assert_eq!(
            calls(&e, BS_STRING_SET_004037F0),
            vec![vec![this + 0x94, 0x9000, 0]]
        );
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x9000]]);
    }

    #[test]
    fn constructor_sets_the_tile_defaults() {
        let mut e = rig();
        e.register(BS_SHADER_PROPERTY_CONSTRUCTOR, |e, a| {
            // Leave garbage where the tile constructor must write.
            for off in (0x60..0xb0).step_by(4) {
                e.mem.set_u32(a[0] + off, 0xdead_beef);
            }
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xb0);
        assert_eq!(e.call(0x00bb_7c30, &args![this, 0x101u32]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_PROPERTY_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), TILE_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        for off in [0x68, 0x6c, 0x70, 0x74, 0x78, 0x84, 0x88] {
            assert_eq!(e.mem.f32(this + off), 1.0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x7c), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 3);
        assert_eq!(e.mem.bytes(this + 0x90, 4), vec![0, 0, 0, 1]);
        assert_eq!(e.mem.u32(this + 0x94), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0);
        assert_eq!(e.mem.u8(this + 0xac), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0x20);
        assert_eq!(e.mem.u32(this + 0x1c), 0x13);
        assert!(calls(&e, INTERLOCKED_DECREMENT).is_empty());
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_render_passes_adds_the_tile_pass_once() {
        let mut e = Engine::new();
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        // The add appends a fresh 0x10-byte pass to the array.
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            let pass = e.mem.alloc(0x10);
            let count = e.mem.u32(a[0] + 0x10);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * count, pass);
            e.mem.set_u32(a[0] + 0x10, count + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        let list = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, buffer);
        e.mem.set_u32(this + 0x3c, list);
        e.call_log = Some(vec![]);
        let words = args![this, 0x5000u32, 7u32, 1u32, 3u32, 2u32, 9u32];
        assert_eq!(e.call(0x00bb_7a60, &words).u32(), list);
        assert_eq!(
            calls_to(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
            vec![vec![this, 1]]
        );
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, 0x5000, 0x42, 1, 0, 0, 0, 0, 0]]
        );
        let pass = e.mem.u32(buffer);
        assert_eq!(e.mem.u8(pass + 6), 1);
        assert_eq!(e.mem.u32(this + 0x38), 0x307);
        // A list with passes already is answered as it is.
        e.mem.set_u32(this + 0x38, 0);
        assert_eq!(e.call(0x00bb_7a60, &words).u32(), list);
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        assert_eq!(e.mem.u32(this + 0x38), 0);
    }

    #[test]
    fn copy_members_assigns_the_texture_and_copies_the_fields() {
        const DELETE: u32 = 0x03d0_0000;
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE);
        let texture = |e: &mut Engine| {
            let texture = e.mem.alloc(8);
            e.mem.set_u32(texture, vtable);
            e.mem.set_u32(texture + 4, 1);
            texture
        };
        let new = texture(&mut e);
        let old = texture(&mut e);
        let this = e.mem.alloc(0xb0);
        let target = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x60, new);
        e.mem.set_u32(target + 0x60, old);
        for (offset, value) in [(0x68u32, 1u32), (0x6c, 2), (0x70, 3), (0x74, 4), (0x8c, 5)] {
            e.mem.set_u32(this + offset, value);
        }
        e.mem.set_u32(this + 0x78, 0x7f80_0001);
        e.mem.set_u32(this + 0x7c, 0x1234);
        e.mem.set_u8(this + 0x93, 1);
        e.mem.set_u8(this + 0x90, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bb_7ac0, &args![this, target, 6u32]);
        assert_eq!(calls_to(&e, BASE_COPY_MEMBERS), vec![vec![this, target, 6]]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(target + 0x60), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        for (offset, value) in [(0x68u32, 1u32), (0x6c, 2), (0x70, 3), (0x74, 4), (0x8c, 5)] {
            assert_eq!(e.mem.u32(target + offset), value);
        }
        assert_eq!(e.mem.u32(target + 0x78), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target + 0x7c), 0);
        assert_eq!((e.mem.u8(target + 0x93), e.mem.u8(target + 0x90)), (1, 1));
        // The same texture again: no reference change.
        e.call(0x00bb_7ac0, &args![this, target, 6u32]);
        assert_eq!(e.mem.u32(new + 4), 2);
    }

    #[test]
    fn save_binary_writes_the_clamp_mode_and_the_file_name() {
        const WRITE: u32 = 0x03d1_0000;
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_C_STRING_00A64160, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, output);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x94, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00bb_7be0, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls_to(&e, WRITE);
        assert_eq!((writes[0][1], writes[0][2]), (this + 0x8c, 4));
        assert_eq!(e.mem.u32(output + 4), 4);
        assert_eq!(
            calls_to(&e, STREAM_SAVE_C_STRING_00A64160),
            vec![vec![stream, 0x6000]]
        );
    }
}
