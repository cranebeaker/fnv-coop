//! `BSShader/run_00bae980` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of `SkyShaderProperty` the linker placed after `skyshaderproperty.cpp`: the
//! class's vtable is `010b8ce0` (stored by the constructor `00baeb80`; its `GetRTTI`
//! answers `012005b8`, which the static initializer at `00fba440` names
//! `"SkyShaderProperty"`). The PC object is 0x90 bytes (the allocation in `00baee40`); the
//! Xbox PDB's 0x98 has two more `BSShaderProperty` words, so the class's own fields sit 8
//! bytes lower on PC.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// `SkyShaderProperty`'s vtable (`010b8ce0`).
const VTABLE: u32 = 0x010b_8ce0;
/// `SkyShaderProperty::ms_RTTI` (`012005b8`).
const RTTI: u32 = 0x0120_05b8;
/// The `u16` counter that numbers the cloud layers as they load (`012005b4`): a property of
/// sky object type 3 takes its value as `usCloudLayer` and counts it up.
const CLOUD_LAYER_COUNTER: u32 = 0x0120_05b4;
/// The `NiColorA` the constructor copies into `VertexColor` (`011a9be0`, four `float`s).
const DEFAULT_VERTEX_COLOR: u32 = 0x011a_9be0;
/// `BSShaderProperty::BSShaderProperty` (engine map, `00ba8e50`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00ba_8e50;
/// The base class's destructor body `00baa0e0` (`thiscall`, no stack words).
const BASE_DESTRUCT: u32 = 0x00ba_a0e0;
/// The base class's `LoadBinary` `00ba92c0` (`thiscall`: the stream).
const BASE_LOAD_BINARY: u32 = 0x00ba_92c0;
/// The stream's string read `00a64ce0` (`thiscall`: the address of the word that receives
/// the allocated string).
const STREAM_READ_STRING: u32 = 0x00a6_4ce0;
/// The `BSStringT<char>` set `004037f0` (`thiscall`: the text, a length word).
const STRING_SET: u32 = 0x0040_37f0;
/// The Ni free `00aa10f0` (`cdecl`, the block).
const NI_FREE: u32 = 0x00aa_10f0;
/// The Ni allocation `00aa13e0` (`cdecl`, size) and the sized free `00aa1460` (`cdecl`: block,
/// size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

layout! {
    /// `SkyShaderProperty` (Xbox PDB), 0x90 bytes on PC.
    pub struct SkyShaderProperty: 0x90 {
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `VertexColor` (Xbox PDB): an `NiColorA` (four `float`s, +0x60 to +0x6c).
        0x60 VertexColor: f32,
        /// `spBaseMap` (Xbox PDB): `NiPointer<NiTexture>`.
        0x70 spBaseMap: u32,
        /// `TextureFilename` (Xbox PDB): a `BSStringT<char>`.
        0x74 TextureFilename: Inline<BSStringT>,
        /// `TextureClampMode` (Xbox PDB).
        0x7C TextureClampMode: u32,
        /// `spBlendTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x80 spBlendTexture: u32,
        /// `fBlendValue` (Xbox PDB).
        0x84 fBlendValue: f32,
        /// `usCloudLayer` (Xbox PDB).
        0x88 usCloudLayer: u16,
        /// `soType` (Xbox PDB): the sky object type (3 for clouds by `LoadBinary`'s use).
        0x8C soType: u32,
    }
}

/// A reference-counted object released: `InterlockedDecrement` on its count at +4, and
/// deleted through its vtable slot +4 when that answers 0.
fn release_reference(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One read through the stream's reader (`+0x24c`): its function at `+8` called with (reader,
/// destination, 4, the address of the count word, 1) and the answer added to the reader's
/// `+4`.
fn stream_read_word(e: &mut Engine, stream: u32, dest: u32, count: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read = e.mem.u32(reader + 8);
    let done = e.call(read, &args![reader, dest, 4u32, count, 1u32]).u32();
    let total = e.mem.u32(reader + 4).wrapping_add(done);
    e.mem.set_u32(reader + 4, total);
}

// Translated from 00baea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty::LoadBinary` (vtable slot +0x4c, the Xbox PDB's `LoadBinary` slot;
/// Ghidra made no function of it): `thiscall`, one word (the stream). The base `LoadBinary`
/// `00ba92c0`, four bytes into the clamp mode, the texture file name (the stream's string
/// read `00a64ce0` into the argument's stack slot, cleared first; set into
/// `TextureFilename` with length word 0; the read string freed through `00aa10f0`), four
/// bytes into `soType`; a cloud property (`soType` 3) takes the cloud layer counter's value
/// as `usCloudLayer` and counts it up. The count word handed to the reader is a local holding
/// 4.
pub fn sky_shader_property_load_binary(e: &mut Engine, this: Ptr<SkyShaderProperty>, stream: u32) {
    let base = this.addr();
    e.call(BASE_LOAD_BINARY, &args![base, stream]);
    e.with_stack(8, |e, frame| {
        let count = frame.addr();
        let slot = count + 4;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(count, 4);
        stream_read_word(e, stream, base + 0x7c, count);
        e.mem.set_u32(slot, 0);
        e.call(STREAM_READ_STRING, &args![stream, slot]);
        let text = e.mem.u32(slot);
        e.call(STRING_SET, &args![base + 0x74, text, 0u32]);
        let text = e.mem.u32(slot);
        e.call(NI_FREE, &args![text]);
        e.mem.set_u32(count, 4);
        stream_read_word(e, stream, base + 0x8c, count);
    });
    if e.get(this, SkyShaderProperty::soType) == 3 {
        let layer = e.global::<u16>(CLOUD_LAYER_COUNTER);
        e.set(this, SkyShaderProperty::usCloudLayer, layer);
        e.set_global(CLOUD_LAYER_COUNTER, layer.wrapping_add(1));
    }
}

// Translated from 00baeb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s constructor (unnamed in the engine map; it stores the class's
/// vtable; `thiscall`, no stack words): the base constructor, the vtable, `VertexColor`
/// zeroed, no base map, an empty file name, clamp mode 3, no blend texture, blend value 0,
/// cloud layer 0, `soType` 8, `VertexColor` copied from `011a9be0`, property type 0xd.
///
/// The compiler kept the `NiPointer` release of the blend texture's old value after storing
/// 0 into it; that value is the 0 it just stored, so the release never runs. It is
/// translated as the code has it.
// Translation incomplete: 00baeb80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the members) is not translated.
pub fn fn_00baeb80(e: &mut Engine, this: Ptr<SkyShaderProperty>) -> Ptr<SkyShaderProperty> {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE);
    for offset in [0x60u32, 0x64, 0x68, 0x6c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x70, 0);
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u16(base + 0x78, 0);
    e.mem.set_u16(base + 0x7a, 0);
    e.mem.set_u32(base + 0x7c, 3);
    e.mem.set_u32(base + 0x80, 0);
    // The old blend texture as the compiler holds it: the 0 just stored.
    let old_blend = 0u32;
    if old_blend != 0 {
        release_reference(e, old_blend);
        e.mem.set_u32(base + 0x80, 0);
    }
    e.mem.set_f32(base + 0x84, 0.0);
    e.mem.set_u16(base + 0x88, 0);
    e.mem.set_u32(base + 0x8c, 8);
    for word in 0..4 {
        let value = e.mem.u32(DEFAULT_VERTEX_COLOR + 4 * word);
        e.mem.set_u32(base + 0x60 + 4 * word, value);
    }
    e.mem.set_u32(base + 0x1c, 0xd);
    this
}

// Translated from 00baec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty::GetRTTI` (vtable slot +8; Ghidra made no function of it): `&ms_RTTI`
/// (`012005b8`).
pub fn sky_shader_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI
}

// Translated from 00baec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s destructor body (unnamed in the engine map; the scalar deleting
/// destructor `00baeeb0` runs it; `thiscall`, no stack words): the vtable; the blend texture
/// released and cleared, blend value 0, `soType` 8, cloud layer 0; the base map released
/// and cleared; the blend texture's `NiPointer` destructor (null by then); the file name's
/// text freed through the memory manager (`00aa4060`) when set and the string emptied; the
/// base map's `NiPointer` destructor (null by then); the base destructor `00baa0e0`.
// Translation incomplete: 00baec90 - the C++ exception frame (FS:[0] chain, unwind states 3 to 0 for the members) is not translated.
pub fn fn_00baec90(e: &mut Engine, this: Ptr<SkyShaderProperty>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    let blend = e.mem.u32(base + 0x80);
    if blend != 0 {
        release_reference(e, blend);
        e.mem.set_u32(base + 0x80, 0);
    }
    e.mem.set_f32(base + 0x84, 0.0);
    e.mem.set_u32(base + 0x8c, 8);
    e.mem.set_u16(base + 0x88, 0);
    let map = e.mem.u32(base + 0x70);
    if map != 0 {
        release_reference(e, map);
        e.mem.set_u32(base + 0x70, 0);
    }
    let blend = e.mem.u32(base + 0x80);
    if blend != 0 {
        release_reference(e, blend);
    }
    let text = e.mem.u32(base + 0x74);
    if text != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, text]);
    }
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u16(base + 0x7a, 0);
    e.mem.set_u16(base + 0x78, 0);
    let map = e.mem.u32(base + 0x70);
    if map != 0 {
        release_reference(e, map);
    }
    e.call(BASE_DESTRUCT, &args![base]);
}

// Translated from 00baee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words; `SkyShaderProperty`'s factory function by
/// its body): allocates 0x90 bytes and answers a property constructed there (`00baeb80`), 0
/// when the allocation fails.
// Translation incomplete: 00baee40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00baee40(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00baeb80(e, Ptr::new(block)).addr()
}

// Translated from 00baeeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty`'s scalar deleting destructor (vtable slot 0; unnamed in the engine
/// map): the destructor body `00baec90`, then the 0x90 bytes freed through `00aa1460` when
/// `flags & 1`. Answers `this`.
pub fn fn_00baeeb0(
    e: &mut Engine,
    this: Ptr<SkyShaderProperty>,
    flags: u32,
) -> Ptr<SkyShaderProperty> {
    fn_00baec90(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x90u32]);
    }
    this
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `BSShaderProperty`'s member copy (`00ba8b10`, `thiscall`, two words: the clone and the
/// cloning process) and `SaveBinary` (`00ba9380`, `thiscall`, one word: the stream).
const BASE_COPY_MEMBERS: u32 = 0x00ba_8b10;
const BASE_SAVE_BINARY: u32 = 0x00ba_9380;
/// The stream's string writer `00a64160` (`thiscall` on the stream, one word: the C string).
const STREAM_SAVE_STRING: u32 = 0x00a6_4160;
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), the
/// `NiGetViewerString` overloads (engine map; `cdecl`): `00a7d330` (a name), `_ov7` `00a7d4d0`
/// (a label and an integer), `_ov9` `00a7d550` (a label and a float), `_ov10` `00a7d5a0` (a
/// label and a pointer), `_ov11` `00a7d5e0` (a label and a string); the string array's grow
/// (`0096ad30`) and set (`0096ae90`), `thiscall`; `BSUtilities::GetTextureName` (`00c4bb40`,
/// `cdecl`, one word); the formatter `00406d00` (`cdecl`).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
const GET_TEXTURE_NAME: u32 = 0x00c4_bb40;
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// `_ftol2_sse` (`00ec62c0`, the value in ST0: a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// The viewer-string labels: `"SOType"` (`010b8de8`), `"Blend Texture"` (`010b8dd8`),
/// `"Blend Value"` (`010b8dcc`), `"base texture"` (`010ae79c`), `"Blend Color"` (`010b8db4`),
/// the format `"%d %d %d %d"` (`010b8dc0`) and `""` (`01011584`).
const SO_TYPE_LABEL: u32 = 0x010b_8de8;
const BLEND_TEXTURE_LABEL: u32 = 0x010b_8dd8;
const BLEND_VALUE_LABEL: u32 = 0x010b_8dcc;
const BASE_TEXTURE_LABEL: u32 = 0x010a_e79c;
const BLEND_COLOR_LABEL: u32 = 0x010b_8db4;
const COLOR_FORMAT: u32 = 0x010b_8dc0;
const EMPTY_TEXT: u32 = 0x0101_1584;
/// The `double` 255 (`0101e568`) the colour channels are scaled by.
const CHANNEL_SCALE_0101E568: u32 = 0x0101_e568;

/// An `NiPointer` assignment from this property's slot to the clone's, as the copy inlines it.
fn copy_pointer(e: &mut Engine, source: u32, target: u32) {
    let old = e.mem.u32(target);
    if old == e.mem.u32(source) {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    let new = e.mem.u32(source);
    e.mem.set_u32(target, new);
    if new != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

// Translated from 00bae980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SkyShaderProperty`'s member copy into a clone (`thiscall`, two
/// words: the clone and the cloning process). The base copy (`00ba8b10`); the blend texture
/// assigned (an `NiPointer`); `fBlendValue` (`FLD`/`FSTP`) and `soType` copied; the base map
/// assigned; `TextureClampMode` copied (the file name and the cloud layer are not).
pub fn fn_00bae980(e: &mut Engine, this: Ptr<SkyShaderProperty>, clone: u32, process: u32) {
    let base = this.addr();
    e.call(BASE_COPY_MEMBERS, &args![base, clone, process]);
    copy_pointer(e, base + 0x80, clone.wrapping_add(0x80));
    let blend = x87_quiet_bits(e.mem.u32(base + 0x84));
    e.mem.set_u32(clone.wrapping_add(0x84), blend);
    let kind = e.mem.u32(base + 0x8c);
    e.mem.set_u32(clone.wrapping_add(0x8c), kind);
    copy_pointer(e, base + 0x70, clone.wrapping_add(0x70));
    let mode = e.mem.u32(base + 0x7c);
    e.mem.set_u32(clone.wrapping_add(0x7c), mode);
}

/// One write through the stream's writer (`+0x250`): its function at `+0xc` (`cdecl`) called
/// with (writer, source, 4, the address of a count word holding 4, 1) and the answer added to
/// the writer's `+4`.
fn stream_write_word(e: &mut Engine, stream: u32, source: u32) {
    e.with_stack(4, |e, count| {
        let writer = e.mem.u32(stream + 0x250);
        let write = e.mem.u32(writer + 0xc);
        e.mem.set_u32(count.addr(), 4);
        let done = e
            .call(write, &args![writer, source, 4u32, count, 1u32])
            .u32();
        let total = e.mem.u32(writer + 4).wrapping_add(done);
        e.mem.set_u32(writer + 4, total);
    });
}

// Translated from 00baeb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SkyShaderProperty::SaveBinary` (vtable slot +0x58, the Xbox
/// PDB's `SaveBinary` slot; `thiscall`, one word: the stream), the mirror of
/// `sky_shader_property_load_binary`: the base `SaveBinary` (`00ba9380`), four bytes from the
/// clamp mode, the file name (`00a64160` with `TextureFilename`'s characters), four bytes from
/// `soType`.
pub fn fn_00baeb00(e: &mut Engine, this: Ptr<SkyShaderProperty>, stream: u32) {
    let base = this.addr();
    e.call(BASE_SAVE_BINARY, &args![base, stream]);
    stream_write_word(e, stream, base + 0x7c);
    let name = e.mem.u32(base + 0x74);
    e.call(STREAM_SAVE_STRING, &args![stream, name]);
    stream_write_word(e, stream, base + 0x8c);
}

// Translated from 00baec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0xd0 of `SkyShaderProperty`'s vtable `010b8ce0`
/// (`SetClampNI` in the Xbox PDB's slot list; `thiscall`, one word): stores the word in
/// `TextureClampMode`.
pub fn fn_00baec80(e: &mut Engine, this: Ptr<SkyShaderProperty>, mode: u32) {
    e.set(this, SkyShaderProperty::TextureClampMode, mode);
}

// Translated from 00baedc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SkyShaderProperty`'s vtable slot +0x48 (`CreateClone` in the
/// Xbox PDB's slot list; `thiscall`, one word: the cloning process). 0x90 bytes from `00aa13e0`
/// constructed by `fn_00baeb80` (null when the allocation gave none) get this property's
/// members (`fn_00bae980`). Answers the clone.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00baedc0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00baedc0(e: &mut Engine, this: Ptr<SkyShaderProperty>, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        fn_00baeb80(e, Ptr::new(block)).addr()
    };
    fn_00bae980(e, this, clone, process);
    clone
}

/// Adds `text` at the end of the viewer-string array: when `m_usSize` is not below
/// `m_usMaxSize` it grows to `m_usSize + m_usGrowBy`, then the text is set at `m_usSize`
/// through the stack word `element`.
fn add_viewer_string(e: &mut Engine, strings: u32, element: u32, text: u32) {
    e.mem.set_u32(element, text);
    let size = u32::from(e.mem.u16(strings + 0xa));
    let capacity = u32::from(e.mem.u16(strings + 8));
    if size >= capacity {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(
            ARRAY_SET_SIZE_0096AD30,
            &args![strings, grow.wrapping_add(size)],
        );
    }
    e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, element]);
}

// Translated from 00baeee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SkyShaderProperty`'s vtable slot +0x60 (`GetViewerStrings` in
/// the Xbox PDB's slot list; `thiscall`, one word: the string array). After
/// `BSShaderProperty::GetViewerStrings` it adds, each at the array's end: the class name (the
/// name word of `012005b8`), `"SOType"` with `soType`, `"Blend Texture"` with the blend
/// texture's name (`+8`; `""` without a texture or name), `"Blend Value"` with `fBlendValue`,
/// `"base texture"` with the base map's texture name (`BSUtilities::GetTextureName`) or, when
/// that is null, the base map as a pointer, and for a cloud (`soType` 3) `"Blend Color"` with
/// `VertexColor`'s channels times 255 (each truncated by `_ftol2`) formatted `"%d %d %d %d"`
/// (red, green, blue, alpha) into a 0x100-byte stack buffer.
///
/// The /GS cookie check is not translated.
// Translation incomplete: 00baeee0 - the /GS cookie check (00ec408c) of its stack buffer is not translated
pub fn fn_00baeee0(e: &mut Engine, this: Ptr<SkyShaderProperty>, strings: u32) {
    let base = this.addr();
    e.call(BASE_GET_VIEWER_STRINGS, &args![base, strings]);
    e.with_stack(0x104, |e, frame| {
        let element = frame.addr();
        let buffer = frame.addr() + 4;
        let name = e.mem.u32(RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add_viewer_string(e, strings, element, text);

        let kind = e.mem.u32(base + 0x8c);
        let text = e.call(VIEWER_STRING_INT, &args![SO_TYPE_LABEL, kind]).u32();
        add_viewer_string(e, strings, element, text);

        let blend = e.mem.u32(base + 0x80);
        let blend_name = if blend != 0 { e.mem.u32(blend + 8) } else { 0 };
        let blend_name = if blend_name != 0 {
            blend_name
        } else {
            EMPTY_TEXT
        };
        let text = e
            .call(VIEWER_STRING_TEXT, &args![BLEND_TEXTURE_LABEL, blend_name])
            .u32();
        add_viewer_string(e, strings, element, text);

        let value = x87_quiet_bits(e.mem.u32(base + 0x84));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![BLEND_VALUE_LABEL, value])
            .u32();
        add_viewer_string(e, strings, element, text);

        let map = e.mem.u32(base + 0x70);
        let map_name = e.call(GET_TEXTURE_NAME, &args![map]).u32();
        let text = if map_name != 0 {
            e.call(VIEWER_STRING_TEXT, &args![BASE_TEXTURE_LABEL, map_name])
                .u32()
        } else {
            let map = e.mem.u32(base + 0x70);
            e.call(VIEWER_STRING_POINTER, &args![BASE_TEXTURE_LABEL, map])
                .u32()
        };
        add_viewer_string(e, strings, element, text);

        if e.mem.u32(base + 0x8c) == 3 {
            let scale: f64 = e.mem.f64(CHANNEL_SCALE_0101E568);
            let alpha = x87_mul(x87_load32(e.mem.u32(base + 0x6c)), scale);
            let alpha = e.call(FTOL, &args![alpha]).u32();
            let blue = x87_mul(x87_load32(e.mem.u32(base + 0x68)), scale);
            let blue = e.call(FTOL, &args![blue]).u32();
            let green = x87_mul(x87_load32(e.mem.u32(base + 0x64)), scale);
            let green = e.call(FTOL, &args![green]).u32();
            let red = x87_mul(scale, x87_m32(e.mem.u32(base + 0x60)));
            let red = e.call(FTOL, &args![red]).u32();
            e.call(
                FORMAT_00406D00,
                &args![buffer, 0x100u32, COLOR_FORMAT, red, green, blue, alpha],
            );
            let text = e
                .call(VIEWER_STRING_TEXT, &args![BLEND_COLOR_LABEL, buffer])
                .u32();
            add_viewer_string(e, strings, element, text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_e980, fn_00bae980(Ptr<SkyShaderProperty>, u32, u32)),
        entry!(0x00ba_eb00, fn_00baeb00(Ptr<SkyShaderProperty>, u32)),
        entry!(0x00ba_ec80, fn_00baec80(Ptr<SkyShaderProperty>, u32)),
        entry!(0x00ba_edc0, fn_00baedc0(Ptr<SkyShaderProperty>, u32) -> u32),
        entry!(0x00ba_eee0, fn_00baeee0(Ptr<SkyShaderProperty>, u32)),
        entry!(
            0x00ba_ea50,
            sky_shader_property_load_binary(Ptr<SkyShaderProperty>, u32)
        ),
        entry!(
            0x00ba_eb80,
            fn_00baeb80(Ptr<SkyShaderProperty>) -> Ptr<SkyShaderProperty>
        ),
        entry!(0x00ba_ec70, sky_shader_property_get_rtti(Ptr) -> u32),
        entry!(0x00ba_ec90, fn_00baec90(Ptr<SkyShaderProperty>)),
        entry!(0x00ba_ee40, fn_00baee40() -> u32),
        entry!(
            0x00ba_eeb0,
            fn_00baeeb0(Ptr<SkyShaderProperty>, u32) -> Ptr<SkyShaderProperty>
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// `InterlockedDecrement` as a plain memory operation (only `Engine::with_exe` registers
    /// the platform's).
    fn interlocked(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
    }

    /// A reference-counted object whose delete slot (+4) is `delete`.
    fn counted(e: &mut Engine, count: u32, delete: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, delete);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn load_rig(so_type: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_LOAD_BINARY, STRING_SET, NI_FREE]);
        e.register(STREAM_READ_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x00ab_cd00);
            Ret::default()
        });
        let mut values = vec![so_type, 2u32];
        e.register_double(0x0000_2220, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], values.pop().unwrap());
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let reader = e.mem.alloc(12);
        e.mem.set_u32(reader + 8, 0x0000_2220);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        e.map(CLOUD_LAYER_COUNTER, 2);
        e.set_global(CLOUD_LAYER_COUNTER, 7u16);
        let this = e.mem.alloc(0x90);
        (e, this, stream)
    }

    #[test]
    fn load_binary_numbers_only_cloud_layers() {
        let (mut e, this, stream) = load_rig(3);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ea50, &args![this, stream]);
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + 0x7c), 2);
        assert_eq!(e.mem.u32(this + 0x8c), 3);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![this + 0x74, 0x00ab_cd00, 0]]
        );
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x00ab_cd00]]);
        let reader = e.mem.u32(stream + 0x24c);
        assert_eq!(e.mem.u32(reader + 4), 8);
        assert_eq!(e.mem.u16(this + 0x88), 7);
        assert_eq!(e.global::<u16>(CLOUD_LAYER_COUNTER), 8);

        let (mut e, this, stream) = load_rig(4);
        e.call(0x00ba_ea50, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(e.global::<u16>(CLOUD_LAYER_COUNTER), 7);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.map(DEFAULT_VERTEX_COLOR, 16);
        for word in 0..4u32 {
            e.mem
                .set_u32(DEFAULT_VERTEX_COLOR + 4 * word, 0x3f80_0000 + word);
        }
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x80, 0x1234);
        assert_eq!(e.call(0x00ba_eb80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        for word in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x60 + 4 * word), 0x3f80_0000 + word);
        }
        assert_eq!(e.mem.u32(this + 0x7c), 3);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 8);
        assert_eq!(e.mem.u32(this + 0x1c), 0xd);
    }

    #[test]
    fn rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ba_ec70, &args![0u32]).u32(), RTTI);
    }

    #[test]
    fn destructor_releases_both_textures_and_frees_the_name() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, MEMORY_FREE, 0x0000_1110]);
        interlocked(&mut e);
        let this = e.mem.alloc(0x90);
        let map = counted(&mut e, 1, 0x0000_1110);
        let blend = counted(&mut e, 2, 0x0000_1110);
        e.mem.set_u32(this + 0x70, map);
        e.mem.set_u32(this + 0x80, blend);
        e.mem.set_u32(this + 0x74, 0x7770);
        e.mem.set_u16(this + 0x88, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ec90, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(calls(&e, 0x0000_1110), vec![vec![map]]);
        assert_eq!(e.mem.u32(blend + 4), 1);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 8);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0x7770]]);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x00ba_ec90, &args![this]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn factory_constructs_into_the_allocation() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.map(DEFAULT_VERTEX_COLOR, 16);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ba_ee40, &args![]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x90]]);
        assert_eq!(e.mem.u32(made), VTABLE);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ba_ee40, &args![]).u32(), 0);
        assert_eq!(calls(&e, BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_DESTRUCT, NI_FREE_SIZED]);
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x00ba_eeb0, &args![this, flags]).u32(), this);
        }
        assert_eq!(calls(&e, BASE_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0x90]; 2]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    const STUB: AbiFn = |_, _| Ret::default();

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        e.register(0x0f00_0004, STUB);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x10);
        e.mem.set_u32(o, 0x0f00_1000);
        e.mem.set_u32(o + 4, 1);
        o
    }

    #[test]
    fn fn_00bae980_copies_the_members() {
        let mut e = rig();
        e.register(BASE_COPY_MEMBERS, STUB);
        let this = e.new_object::<SkyShaderProperty>();
        let clone = e.new_object::<SkyShaderProperty>();
        let (blend, map, old) = (object(&mut e), object(&mut e), object(&mut e));
        e.set(this, SkyShaderProperty::spBlendTexture, blend);
        e.set(this, SkyShaderProperty::spBaseMap, map);
        e.set(clone, SkyShaderProperty::spBaseMap, old);
        e.mem.set_u32(this.addr() + 0x84, 0x7f80_0001);
        e.set(this, SkyShaderProperty::soType, 3);
        e.set(this, SkyShaderProperty::TextureClampMode, 2);
        e.call(0x00ba_e980, &args![this, clone, 0x44u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x44]]
        );
        assert_eq!(e.get(clone, SkyShaderProperty::spBlendTexture), blend);
        assert_eq!(e.get(clone, SkyShaderProperty::spBaseMap), map);
        assert_eq!((e.mem.u32(blend + 4), e.mem.u32(map + 4)), (2, 2));
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(clone.addr() + 0x84), 0x7fc0_0001);
        assert_eq!(e.get(clone, SkyShaderProperty::soType), 3);
        assert_eq!(e.get(clone, SkyShaderProperty::TextureClampMode), 2);
    }

    #[test]
    fn fn_00baeb00_saves_mode_name_and_type() {
        const WRITE: u32 = 0x0f00_0200;
        let mut e = rig();
        e.register(BASE_SAVE_BINARY, STUB);
        e.register(STREAM_SAVE_STRING, STUB);
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            a[2].into_ret()
        });
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.new_object::<SkyShaderProperty>();
        e.mem.set_u32(this.addr() + 0x74, 0x5150);
        e.call(0x00ba_eb00, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this.addr(), stream]]);
        let writes = calls(&e, WRITE);
        assert_eq!(writes.len(), 2);
        assert_eq!(&writes[0][..3], &[writer, this.addr() + 0x7c, 4]);
        assert_eq!(&writes[1][..3], &[writer, this.addr() + 0x8c, 4]);
        assert_eq!(calls(&e, STREAM_SAVE_STRING), vec![vec![stream, 0x5150]]);
        assert_eq!(e.mem.u32(writer + 4), 8);
    }

    #[test]
    fn fn_00baec80_sets_the_clamp_mode() {
        let mut e = rig();
        let this = e.new_object::<SkyShaderProperty>();
        e.call(0x00ba_ec80, &args![this, 6u32]);
        assert_eq!(e.get(this, SkyShaderProperty::TextureClampMode), 6);
    }

    #[test]
    fn fn_00baedc0_constructs_and_copies() {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        for addr in [BASE_CONSTRUCT, BASE_COPY_MEMBERS] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.new_object::<SkyShaderProperty>();
        e.set(this, SkyShaderProperty::TextureClampMode, 1);
        let clone = e.call(0x00ba_edc0, &args![this, 0x77u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x7c), 1);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
    }

    #[test]
    fn fn_00baeee0_adds_the_strings() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(CHANNEL_SCALE_0101E568, 255.0);
        e.mem.set_u32(RTTI, 0x0123_0000);
        e.register(BASE_GET_VIEWER_STRINGS, STUB);
        for (addr, tag) in [
            (VIEWER_STRING_NAME, 1u32),
            (VIEWER_STRING_INT, 2),
            (VIEWER_STRING_FLOAT, 3),
            (VIEWER_STRING_POINTER, 4),
            (VIEWER_STRING_TEXT, 5),
        ] {
            e.register_double(addr, move |_, _| tag.into_ret());
        }
        e.register(GET_TEXTURE_NAME, |_, _| 0u32.into_ret());
        e.register(ARRAY_SET_SIZE_0096AD30, STUB);
        e.register(ARRAY_SET_AT_0096AE90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x10 + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(FORMAT_00406D00, STUB);
        let strings = e.mem.alloc(0x40);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 4);
        let this = e.new_object::<SkyShaderProperty>();
        let map = object(&mut e);
        e.set(this, SkyShaderProperty::spBaseMap, map);
        e.mem.set_f32(this.addr() + 0x84, 0.5);
        e.set(this, SkyShaderProperty::soType, 3);
        for (i, v) in [1.0f32, 0.5, 0.0, 1.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0x60 + 4 * i as u32, *v);
        }
        e.call(0x00ba_eee0, &args![this, strings]);
        let added: Vec<u32> = (0..6).map(|i| e.mem.u32(strings + 0x10 + 4 * i)).collect();
        assert_eq!(added, vec![1, 2, 5, 3, 4, 5]);
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0123_0000]]);
        assert_eq!(calls(&e, VIEWER_STRING_INT), vec![vec![SO_TYPE_LABEL, 3]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT)[0],
            vec![BLEND_TEXTURE_LABEL, EMPTY_TEXT]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT),
            vec![vec![BLEND_VALUE_LABEL, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_POINTER),
            vec![vec![BASE_TEXTURE_LABEL, map]]
        );
        let format = calls(&e, FORMAT_00406D00);
        assert_eq!(&format[0][1..], &[0x100, COLOR_FORMAT, 255, 127, 0, 255]);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT)[1],
            vec![BLEND_COLOR_LABEL, format[0][0]]
        );
        // The array grew once it was full.
        assert_eq!(calls(&e, ARRAY_SET_SIZE_0096AD30)[0], vec![strings, 6]);
    }
}
