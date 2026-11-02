//! `BSMenu/font.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is compiled without optimization: every local lives on the stack
//! and every `float` passes through `FLD`/`FSTP`, so the translations quiet
//! signalling NaNs (`x87_quiet`) wherever the code stores a `float` it loaded.
//! Locals the code passes by address (a cursor position, an out parameter)
//! are game memory (`e.with_stack`).
//!
//! The functions with a C++ exception frame (`FS:[0]`) or the `/GS` cookie
//! check are marked incomplete: the unwinding itself is not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, BSStringT, NiTListItem, NiTPointerList};

/// `MemoryManager` singleton (`011f6238`), `this` of the allocator calls.
pub(crate) const MEMORY_MANAGER: u32 = 0x011f_6238;

/// `MemoryManager::Allocate` (`00aa3e40`): `(this, size)`.
pub(crate) const ALLOCATE: u32 = 0x00aa_3e40;

/// `MemoryManager::Deallocate` (`00aa4060`): `(this, block)`.
pub(crate) const DEALLOCATE: u32 = 0x00aa_4060;

/// `MemoryManager` reallocation (`00aa4150`): `(this, block, size)`, returns
/// the new block.
pub(crate) const REALLOCATE: u32 = 0x00aa_4150;

/// `_strlen` (`00ec6130`, cdecl).
pub(crate) const STRLEN: u32 = 0x00ec_6130;

/// `_strcpy_s` (`00ec65a6`, cdecl): `(dest, size, source)`.
pub(crate) const STRCPY_S: u32 = 0x00ec_65a6;

/// The CRT `fmod` entry (`00ec9130`, cdecl): two `double`s, result in ST0.
pub(crate) const FMOD: u32 = 0x00ec_9130;

/// `_ftol2` (`00ec62c0`): truncates ST0 to an integer (ST0 is a leading
/// `f64` argument in the uniform form).
pub(crate) const FTOL: u32 = 0x00ec_62c0;

/// `` `eh vector constructor iterator' `` (`00ec782f`, cdecl):
/// `(array, element size, count, constructor, destructor)`.
pub(crate) const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;

/// `` `eh vector destructor iterator' `` (`00ec5fce`, cdecl):
/// `(array, element size, count, destructor)`.
pub(crate) const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;

/// The `NiPointer` destructor the font's texture pointers are destroyed with
/// (`0045cec0`).
pub(crate) const NI_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;

/// `NiPointer` assignment (`0066b0d0`, thiscall on the pointer slot).
pub(crate) const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;

/// `BSStringT<char>` set (`004037f0`): `(this, text, length)`.
pub(crate) const BS_STRING_SET: u32 = 0x0040_37f0;

/// `NiBound::ComputeFromData` (`00a7ee30`, Xbox PDB name): `(bound, vertex
/// count, vertices)`.
pub(crate) const NI_BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;

/// `NiGeometryData::SetConsistency` (`00a67050`, Xbox PDB name).
pub(crate) const NI_GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;

/// `NiTListBase<..>::AddNodeTail` (`00a11db0`, Xbox PDB name).
pub(crate) const NI_TLIST_ADD_NODE_TAIL: u32 = 0x00a1_1db0;

/// The constructor of the `Font::TextData` the text functions fill
/// (`00759330`): `(this, width, height, line start, line end, separator)`.
pub(crate) const TEXT_DATA_CONSTRUCTOR: u32 = 0x0075_9330;

/// `Font::TextData::~TextData` (`007593e0`, Xbox PDB name).
pub(crate) const TEXT_DATA_DESTRUCTOR: u32 = 0x0075_93e0;

/// The `FontManager` pointer (`FontManager::pInstance`, Xbox PDB).
pub(crate) const FONT_MANAGER_INSTANCE: u32 = 0x011f_33f8;

/// A block pointer of this unit that the `Font` destructor frees and clears
/// (`011f33fc`), and the word after it that it also clears (`011f3400`).
pub(crate) const FONT_STATIC_BLOCK: u32 = 0x011f_33fc;

pub(crate) const FONT_STATIC_BLOCK_WORD: u32 = 0x011f_3400;

/// `0.0` (`double`, `01012060`).
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `FLT_MAX` (`float`, `01016970`).
pub(crate) const FLOAT_MAX: u32 = 0x0101_6970;

/// `2.0` (`double`, `01011590`).
pub(crate) const TWO_DOUBLE: u32 = 0x0101_1590;

/// The tab width as a `float` (`0102f0f8`, 75.0 in the exe).
pub(crate) const TAB_WIDTH_FLOAT: u32 = 0x0102_f0f8;

/// The tab width as a `double` (`010731a8`, 75.0 in the exe).
pub(crate) const TAB_WIDTH_DOUBLE: u32 = 0x0107_31a8;

/// Offset of the first `FontLetter` in `FontData` (Xbox PDB `pFontLetters`).
pub(crate) const FONT_LETTERS: u32 = 0x128;

/// Size of one `FontLetter` (Xbox PDB).
pub(crate) const FONT_LETTER_SIZE: u32 = 0x38;

layout! {
    /// `Font` (Xbox PDB), 0x54 bytes; the offsets are the ones the PC
    /// constructor (`00a12020`) writes.
    pub struct Font: 0x54 {
        /// `iRefCount` (Xbox PDB).
        0x00 iRefCount: u16,
        /// `pFontFile` (Xbox PDB): the file name, allocated.
        0x04 pFontFile: u32,
        /// `iFontNum` (Xbox PDB).
        0x08 iFontNum: i32,
        /// `pTextureData` (Xbox PDB): eight `NiPointer<NiTexturingProperty>`
        /// from here.
        0x0C pTextureData: u32,
        /// `fFontHeight` (Xbox PDB).
        0x2C fFontHeight: f32,
        /// `fMaxDrop` (Xbox PDB).
        0x30 fMaxDrop: f32,
        /// `iLineOverlap` (Xbox PDB).
        0x34 iLineOverlap: i32,
        /// `pFontData` (Xbox PDB): the [`FontData`].
        0x38 pFontData: u32,
        /// `IconAtlasTextureName` (Xbox PDB), a [`BSStringT`].
        0x3C IconAtlasTextureName: Inline<BSStringT>,
        /// `ButtonIcons` (Xbox PDB), a `BSSimpleArray<ButtonIcon>`
        /// ([`BSSimpleArray`]; its buffer at +0x48, size at +0x4C).
        0x44 ButtonIcons: Inline<BSSimpleArray>,
    }

    /// `FontData` (Xbox PDB), the font file's contents: base line, texture
    /// count, eight texture file names, then 256 [`FontLetter`]s from +0x128.
    pub struct FontData: 0x3928 {
        /// `fBaseLine` (Xbox PDB).
        0x00 fBaseLine: f32,
        /// `iTextureCount` (Xbox PDB).
        0x04 iTextureCount: i32,
    }

    /// `FontLetter` (Xbox PDB), 0x38 bytes.
    pub struct FontLetter: 0x38 {
        /// `iTextureIndex` (Xbox PDB).
        0x00 iTextureIndex: i32,
        /// `fWidth` (Xbox PDB).
        0x24 fWidth: f32,
        /// `fHeight` (Xbox PDB).
        0x28 fHeight: f32,
        /// `fLeadingEdge` (Xbox PDB).
        0x2C fLeadingEdge: f32,
        /// `fSpacing` (Xbox PDB).
        0x30 fSpacing: f32,
        /// `fTopEdge` (Xbox PDB).
        0x34 fTopEdge: f32,
    }

    /// `Font::TextData` (Xbox PDB), 0x28 bytes: the prepared text and its
    /// measurements.
    pub struct TextData: 0x28 {
        /// `xNewText` (Xbox PDB), a [`BSStringT`].
        0x00 xNewText: Inline<BSStringT>,
        /// `iWidth` (Xbox PDB).
        0x08 iWidth: i32,
        /// `iHeight` (Xbox PDB).
        0x0C iHeight: i32,
        /// `iLineStart` (Xbox PDB).
        0x10 iLineStart: i32,
        /// `iLineEnd` (Xbox PDB).
        0x14 iLineEnd: i32,
        /// `iCharCount` (Xbox PDB).
        0x18 iCharCount: i32,
        /// `cLineSep` (Xbox PDB).
        0x1C cLineSep: u8,
        /// `xLineWidths` (Xbox PDB), a [`BSSimpleList`] of `int`.
        0x20 xLineWidths: Inline<BSSimpleList>,
    }
}

/// Address of letter `c` of the font data at `font_data`.
fn letter(font_data: u32, c: u8) -> u32 {
    font_data
        .wrapping_add(FONT_LETTERS)
        .wrapping_add(c as u32 * FONT_LETTER_SIZE)
}

/// The byte the code reads as `text[text ? i : 0]` (the compiler's
/// `NEG/SBB/AND` index guard).
fn guarded_byte(e: &Engine, text: u32, i: u32) -> u8 {
    let index = if text != 0 { i } else { 0 };
    e.mem.u8(text.wrapping_add(index))
}

/// The width of line `line` in a `BSSimpleList<int>` of line widths: the
/// item `line` nodes from the head, or -1 past the end.
fn line_width(e: &Engine, list: u32, line: i32) -> i32 {
    let mut node = list;
    let mut i = 0;
    while i < line && node != 0 {
        node = e.mem.u32(node + 4);
        i += 1;
    }
    if node != 0 {
        e.mem.i32(node)
    } else {
        -1
    }
}

/// The ordered "above" the x87 comparisons test (`FCOMP` with `TEST
/// AH,0x41`): false for below, equal and unordered (NaN).
fn x87_above(a: f64, b: f64) -> bool {
    a > b
}

/// `_ftol2` on an x87 value.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// Runs `f` with the per-thread allocation word at `TLS + 0x2b4` set to
/// `value`, restoring the old value afterwards.
fn with_tls_scope<R>(e: &mut Engine, value: u32, f: impl FnOnce(&mut Engine) -> R) -> R {
    let at = e.tls() + 0x2b4;
    let saved = e.mem.u32(at);
    e.mem.set_u32(at, value);
    let r = f(e);
    let at = e.tls() + 0x2b4;
    e.mem.set_u32(at, saved);
    r
}

/// `NiBound::ComputeFromData` on the bound of a geometry's model data
/// (`NiGeometry` +0xB8, data +0x10), with its vertex count (+0x08) and
/// vertices (+0x20).
fn compute_geometry_bound(e: &mut Engine, geometry: u32) {
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = e.mem.u32(data + 0x20);
    let count = e.mem.u16(data + 8);
    e.call(
        NI_BOUND_COMPUTE_FROM_DATA,
        &args![data + 0x10, count as u32, vertices],
    );
}

/// The x start of line `line` for `alignment` 4 (right: minus the width) or
/// 2 (centered: minus half the width, rounded toward zero); 0 otherwise.
fn aligned_line_start(e: &Engine, widths: u32, alignment: i32, line: i32) -> Option<i32> {
    match alignment {
        4 => Some(line_width(e, widths, line).wrapping_neg()),
        2 => Some((line_width(e, widths, line) / 2).wrapping_neg()),
        _ => None,
    }
}

/// `_memset` (`00ec61c0`, cdecl).
pub(crate) const MEMSET: u32 = 0x00ec_61c0;

/// `sprintf` (`00ec623a`, cdecl, variable arguments).
pub(crate) const SPRINTF: u32 = 0x00ec_623a;

/// `strcpy` (`00ec6370`, cdecl).
pub(crate) const STRCPY: u32 = 0x00ec_6370;

/// `memmove` (`00ec7230`, cdecl).
pub(crate) const MEMMOVE: u32 = 0x00ec_7230;

/// A `sprintf_s`-style formatter (`00406d00`, cdecl): `(buffer, size,
/// format, ...)`.
pub(crate) const FORMAT_S: u32 = 0x0040_6d00;

/// `Interface::FindTextReplacementString` (`007070c0`, Xbox PDB name,
/// cdecl): `(name, out, out size, flag)`.
pub(crate) const FIND_TEXT_REPLACEMENT_STRING: u32 = 0x0070_70c0;

/// `Interface::TestConstantForGameSettings` (`007073d0`, Xbox PDB name,
/// cdecl): `(name, out)`.
pub(crate) const TEST_CONSTANT_FOR_GAME_SETTINGS: u32 = 0x0070_73d0;

/// Appends `*value` to a `BSSimpleList<int>` (`00af25b0`, thiscall on the
/// list): fills the head item when it is empty, else a new node at the end.
pub(crate) const SIMPLE_LIST_ADD_TAIL: u32 = 0x00af_25b0;

/// `"%s"` (`01019f08`).
pub(crate) const PERCENT_S: u32 = 0x0101_9f08;

/// `"glow_%s"` (`010950a8`).
pub(crate) const GLOW_FORMAT: u32 = 0x0109_50a8;

/// Moved to the core (the Havok crate calls it too); here under its old path.
pub(crate) use engine_core::x87::fistp;

/// `_ftol2` of a letter's `fWidth + fSpacing`.
fn letter_advance(e: &mut Engine, letter: u32) -> i32 {
    let advance = e.mem.f32(letter + 0x24) as f64 + e.mem.f32(letter + 0x30) as f64;
    ftol(e, advance)
}

/// The letter of the signed byte `c` (the code indexes with `MOVSX` here).
fn signed_letter(font_data: u32, c: i8) -> u32 {
    font_data
        .wrapping_add(FONT_LETTERS)
        .wrapping_add((c as i32 as u32).wrapping_mul(FONT_LETTER_SIZE))
}

/// `PrepText`'s line-break bookkeeping: appends the line width at `width_at`
/// to the text data's line widths and keeps the widest.
fn finish_line(e: &mut Engine, lines_list: u32, width_at: u32, widest: &mut i32) {
    e.call(SIMPLE_LIST_ADD_TAIL, &args![lines_list, width_at]);
    let width = e.mem.i32(width_at);
    *widest = if *widest > width { *widest } else { width };
}

/// The four `float`s `fn_00a14170` fills: the first word's constant
/// (`01034294`, 60.0 in the exe) and the second and fourth words' pairs for
/// `a`/`b`/`x`/`y` names (`01053114`, `0104f754`), `ls`/`lt`/`rs`/`rt`
/// names (`010950b0`, `0104f408`) and other names (`0104eb20`, `0101e578`).
const ICON_FIRST: u32 = 0x0103_4294;

const ICON_FACE_SECOND: u32 = 0x0105_3114;

const ICON_FACE_FOURTH: u32 = 0x0104_f754;

const ICON_SHOULDER_SECOND: u32 = 0x0109_50b0;

const ICON_SHOULDER_FOURTH: u32 = 0x0104_f408;

const ICON_OTHER_SECOND: u32 = 0x0104_eb20;

const ICON_OTHER_FOURTH: u32 = 0x0101_e578;

/// The `double` adjustments for fonts 1 and 7: `01018c00` (2.5 in the exe),
/// `01021928` (3.0), `01020998` (5.0), `01020758` (10.0).
const ICON_ADJUST_SECOND: u32 = 0x0101_8c00;

const ICON_ADJUST_THIRD_FONT_1: u32 = 0x0102_1928;

const ICON_ADJUST_FOURTH: u32 = 0x0102_0998;

const ICON_ADJUST_THIRD_FONT_7: u32 = 0x0102_0758;

/// `NiTriShapeData::GetTriList` (`00a7b850`, Xbox PDB name, thiscall on the
/// data): the triangle index list.
pub(crate) const NI_TRI_SHAPE_DATA_GET_TRI_LIST: u32 = 0x00a7_b850;

/// Writes three `float`s, each through an `FLD`/`FSTP` pair.
fn store_point(e: &mut Engine, at: u32, point: [f32; 3]) {
    for (k, v) in point.iter().enumerate() {
        e.mem.set_f32(at + 4 * k as u32, x87_quiet(*v));
    }
}

/// Writes the two triangles of quad `index` (vertices `4 * index` to
/// `4 * index + 3`) into the triangle list of `data`.
fn store_quad_triangles(e: &mut Engine, data: u32, index: u32) {
    let base = index.wrapping_mul(4);
    let list = e.call(NI_TRI_SHAPE_DATA_GET_TRI_LIST, &args![data]).u32();
    let at = list.wrapping_add(index.wrapping_mul(6).wrapping_mul(2));
    for (k, add) in [0u32, 1, 2, 2, 1, 3].iter().enumerate() {
        e.mem
            .set_u16(at + 2 * k as u32, base.wrapping_add(*add) as u16);
    }
}

/// Gamebryo's array allocation (`00aa1070`, cdecl): `(size)`.
pub(crate) const NI_ALLOC: u32 = 0x00aa_1070;

/// Gamebryo's object allocation (`00aa13e0`, cdecl): `(size)`.
pub(crate) const NI_NEW: u32 = 0x00aa_13e0;

/// `BSScissorTriShape::BSScissorTriShape` (`00c5cf30`, Xbox PDB name).
pub(crate) const BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR: u32 = 0x00c5_cf30;

/// `NiFixedString` from a C string (`00a5b690`, cdecl).
pub(crate) const NI_FIXED_STRING_CREATE: u32 = 0x00a5_b690;

/// Sets an object's name from an `NiFixedString` (`00a5b950`, thiscall).
pub(crate) const NI_OBJECT_NET_SET_NAME: u32 = 0x00a5_b950;

/// The pooled empty string handle the release is skipped for (`0109b220`).
pub(crate) const NI_FIXED_STRING_EMPTY: u32 = 0x0109_b220;

/// The `InterlockedDecrement` import slot (`00fdf1c4`).
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `"Interface: Text"` (`010950b4`).
pub(crate) const INTERFACE_TEXT_NAME: u32 = 0x0109_50b4;

/// `TileShaderProperty::TileShaderProperty` (`00bb7c30`, Xbox PDB name).
pub(crate) const TILE_SHADER_PROPERTY_CONSTRUCTOR: u32 = 0x00bb_7c30;

/// `NiAVObject::AttachProperty` (`00439410`, Xbox PDB name).
pub(crate) const NI_AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;

/// `BSShaderManager::PrepareObject` (`00b57e30`, Xbox PDB name, cdecl).
pub(crate) const BS_SHADER_MANAGER_PREPARE_OBJECT: u32 = 0x00b5_7e30;

/// `TileShaderProperty::SetTileTexture` (`00bb7a10`, Xbox PDB name).
pub(crate) const TILE_SHADER_PROPERTY_SET_TILE_TEXTURE: u32 = 0x00bb_7a10;

/// `NiAVObject::UpdateProperties` (`00a5a040`, Xbox PDB name).
pub(crate) const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;

/// `Interface::AttachDefaultAlphaProperty` (`00706d50`, Xbox PDB name,
/// cdecl).
pub(crate) const INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY: u32 = 0x0070_6d50;

/// The size `new T[count]` asks for: `count * size`, or 0xFFFFFFFF when the
/// product overflows (`MUL; SETO; NEG; OR`).
pub(crate) fn array_bytes(count: u32, size: u32) -> u32 {
    let product = count as u64 * size as u64;
    if product > u32::MAX as u64 {
        u32::MAX
    } else {
        product as u32
    }
}

/// Names `object` "Interface: Text" through a temporary `NiFixedString`
/// (created, passed by address, released unless it is the empty handle).
fn set_interface_text_name(e: &mut Engine, object: u32) {
    let handle = e
        .call(NI_FIXED_STRING_CREATE, &args![INTERFACE_TEXT_NAME])
        .u32();
    release_after_use(e, handle, |e, local| {
        e.call(NI_OBJECT_NET_SET_NAME, &args![object, local]);
    });
}

/// An `NiFixedString` temporary: `handle` in a stack local passed to `f` by
/// address, then released (`InterlockedDecrement` on the count 8 bytes
/// before the text) unless it is the empty handle (`0109b220`).
pub(crate) fn release_after_use(e: &mut Engine, handle: u32, f: impl FnOnce(&mut Engine, Ptr)) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), handle);
        f(e, local);
        let handle = e.mem.u32(local.addr());
        if handle != e.global::<u32>(NI_FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
}

/// The texture of the font's first texturing property: its first map's
/// texture (`NiTexturingProperty` +0x20 maps, map +0x08), or 0.
fn font_texture(e: &Engine, this: Ptr<Font>) -> u32 {
    let property = e.mem.u32(this.addr() + 0x0c);
    let maps = e.mem.u32(property + 0x20);
    let map = e.mem.u32(maps);
    if map != 0 {
        e.mem.u32(map + 8)
    } else {
        0
    }
}

/// The tile shader property part shared by `MakeTriShape` and
/// `MakeIconsTriShape`: a new `TileShaderProperty` (0xB0 bytes, 0 when the
/// allocation fails) attached to `shape`.
fn attach_tile_shader(e: &mut Engine, shape: u32) -> u32 {
    let block = e.call(NI_NEW, &args![0xb0u32]).u32();
    let property = if block != 0 {
        e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    } else {
        0
    };
    e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![shape, property]);
    property
}

/// A new `BSScissorTriShape` (0xD4 bytes, 0 when the allocation fails) for
/// `quads` quads: 4 vertices and texture coordinates per quad and 2
/// triangles per quad.
fn new_quad_shape(e: &mut Engine, quads: u32) -> u32 {
    let vertex_count = quads.wrapping_mul(4);
    // The `new NiPoint3[]` and `new NiPoint2[]` element loops construct
    // nothing (trivial constructors).
    let vertices = e
        .call(
            ALLOCATE,
            &args![MEMORY_MANAGER, array_bytes(vertex_count, 0x0c)],
        )
        .u32();
    let uvs = e
        .call(
            ALLOCATE,
            &args![MEMORY_MANAGER, array_bytes(vertex_count, 8)],
        )
        .u32();
    let triangles = e
        .call(NI_ALLOC, &args![quads.wrapping_mul(6).wrapping_shl(1)])
        .u32();
    let block = e.call(NI_NEW, &args![0xd4u32]).u32();
    if block != 0 {
        e.call(
            BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR,
            &args![
                block,
                vertex_count & 0xffff,
                vertices,
                0u32,
                0u32,
                uvs,
                1u32,
                0u32,
                quads.wrapping_shl(1),
                triangles,
                0u32,
                0u32,
                0u32,
                0u32
            ],
        )
        .u32()
    } else {
        0
    }
}

/// `"Interface: TextIcons"` (`010950c4`).
pub(crate) const INTERFACE_TEXT_ICONS_NAME: u32 = 0x0109_50c4;

/// The color the icon geometry's shader property gets (`011a9be0`, four
/// words).
pub(crate) const ICON_SHADER_COLOR: u32 = 0x011a_9be0;

/// `_strncpy_s` (`00ec8d5f`, cdecl): `(dest, size, source, count)`.
pub(crate) const STRNCPY_S: u32 = 0x00ec_8d5f;

/// `"Data\Textures\%s"` (`0104a64c`).
pub(crate) const TEXTURES_PATH_FORMAT: u32 = 0x0104_a64c;

/// `NiSourceTexture::Create` (`00a5fd70`, Xbox PDB name, cdecl):
/// `(name, format preferences, load directly, srgb)`.
pub(crate) const NI_SOURCE_TEXTURE_CREATE: u32 = 0x00a5_fd70;

/// The format preferences passed to `NiSourceTexture::Create` (`011a9598`).
pub(crate) const TEXTURE_FORMAT_PREFS: u32 = 0x011a_9598;

/// `FileFinder::GetFile_ov2` (`00afdf20`, Xbox PDB name, cdecl): `(path, 0,
/// 0x4000, 2)`, a file object or 0.
pub(crate) const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;

/// The message printer the load errors go through (`005b5e40`, cdecl,
/// variable arguments).
pub(crate) const PRINT_MESSAGE: u32 = 0x005b_5e40;

/// `"DEFAULT: Font file not found: %s. \n"` (`01095164`).
pub(crate) const FONT_NOT_FOUND_FORMAT: u32 = 0x0109_5164;

/// The "too many font textures" message (`01095118`).
pub(crate) const TOO_MANY_TEXTURES_FORMAT: u32 = 0x0109_5118;

/// The font texture "not found" message (`010950dc`).
pub(crate) const TEXTURE_NOT_FOUND_FORMAT: u32 = 0x0109_50dc;

/// `"TEXTURES\FONTS\%s.TEX"` (`01095100`).
pub(crate) const FONT_TEXTURE_PATH_FORMAT: u32 = 0x0109_5100;

/// `NiPixelData::NiPixelData_ov2` (`00a7c190`, Xbox PDB name): `(this,
/// width, height, format, mipmap levels, faces)`.
pub(crate) const NI_PIXEL_DATA_CONSTRUCTOR: u32 = 0x00a7_c190;

/// The pixel format the font textures use (`011aa2a0`).
pub(crate) const FONT_PIXEL_FORMAT: u32 = 0x011a_a2a0;

/// The constructor (in `NiMain/nitexturingproperty.cpp`, `00a6abb0`) that
/// makes a font texture from `(pixel data, name, format preferences)`.
pub(crate) const TEXTURE_FROM_PIXEL_DATA: u32 = 0x00a6_abb0;

/// The call made on each new font texture with 1 (`0060aeb0`, thiscall).
pub(crate) const FONT_TEXTURE_SETUP: u32 = 0x0060_aeb0;

/// Reads `size` bytes into `buffer` through the file object's read function
/// (the pointer at +8, cdecl `(file, buffer, size, &count, 1)` with count 1)
/// and advances its position (+4) by the result.
fn file_read(e: &mut Engine, file: u32, buffer: u32, size: u32) {
    let read = e.mem.u32(file + 8);
    let n = e.with_stack(4, |e, one| {
        e.mem.set_u32(one.addr(), 1);
        e.call(read, &args![file, buffer, size, one, 1u32]).u32()
    });
    let pos = e.mem.u32(file + 4).wrapping_add(n);
    e.mem.set_u32(file + 4, pos);
}

/// Deletes a file object (virtual slot 0 with 1) when it is not null.
fn delete_file(e: &mut Engine, file: u32) {
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
}

/// `Font::Load`'s per-letter maxima: the font height grows to
/// `(base line - top edge) + height` (`FCOMPP`, `TEST AH,0x05`: kept unless
/// the candidate is below), the tallest letter, and the drop shrinks to
/// `top edge - height` (`TEST AH,0x41`: replaced unless the candidate is
/// above).
fn load_letter_metrics(e: &mut Engine, this: Ptr<Font>, tallest: &mut f32) {
    for i in 0..0x100u32 {
        let data = e.get(this, Font::pFontData);
        let at = data.wrapping_add(i * 0x38);
        let reach = (e.mem.f32(data) as f64 - e.mem.f32(at + 0x15c) as f64) as f32;
        let reach = (reach as f64 + e.mem.f32(at + 0x150) as f64) as f32;
        let height = x87_quiet(e.get(this, Font::fFontHeight));
        let keep = (reach as f64) < height as f64;
        e.set(this, Font::fFontHeight, if keep { height } else { reach });

        let data = e.get(this, Font::pFontData);
        let letter_height = x87_quiet(e.mem.f32(data.wrapping_add(i * 0x38) + 0x150));
        let keep = (letter_height as f64) < *tallest as f64;
        *tallest = x87_quiet(if keep { *tallest } else { letter_height });

        let drop = x87_quiet(e.get(this, Font::fMaxDrop));
        let data = e.get(this, Font::pFontData);
        let at = data.wrapping_add(i * 0x38);
        let candidate = (e.mem.f32(at + 0x15c) as f64 - e.mem.f32(at + 0x150) as f64) as f32;
        let replace = !x87_above(candidate as f64, drop as f64);
        e.set(this, Font::fMaxDrop, if replace { candidate } else { drop });
    }
}

/// Copies the `float` at `from` to `to` (both offsets in the font data)
/// through `FLD`/`FSTP`.
fn copy_data_float(e: &mut Engine, data: u32, from: u32, to: u32) {
    let v = x87_quiet(e.mem.f32(data + from));
    e.mem.set_f32(data + to, v);
}

/// Loads font texture `index` (`TEXTURES\FONTS\<name>.TEX`: width and
/// height, then the 32-bit pixels) into texture slot `index`. False when
/// the file is missing or not open (the message is printed).
fn load_font_texture(e: &mut Engine, this: Ptr<Font>, index: u32) -> bool {
    // Stack locals: +0 path (0x100), +0x100 width and height, +0x108 the
    // format preferences (three words).
    e.with_stack(0x114, |e, locals| {
        let path = locals.addr();
        let header = path + 0x100;
        let prefs = path + 0x108;
        let data = e.get(this, Font::pFontData);
        let texture_name = data
            .wrapping_add(0x0c)
            .wrapping_add(index.wrapping_mul(0x24));
        e.call(
            FORMAT_S,
            &args![path, 0x100u32, FONT_TEXTURE_PATH_FORMAT, texture_name],
        );
        let file = e
            .call(FILE_FINDER_GET_FILE, &args![path, 0u32, 0x4000u32, 2u32])
            .u32();
        if file == 0 || e.mem.u8(file + 0x2c) == 0 {
            e.call(PRINT_MESSAGE, &args![TEXTURE_NOT_FOUND_FORMAT, path]);
            delete_file(e, file);
            return false;
        }
        file_read(e, file, header, 8);
        e.mem.set_u32(prefs, 6);
        e.mem.set_u32(prefs + 4, 3);
        e.mem.set_u32(prefs + 8, 2);
        let block = e.call(NI_NEW, &args![0x74u32]).u32();
        let pixels = if block != 0 {
            let width = e.mem.u32(header);
            let height = e.mem.u32(header + 4);
            e.call(
                NI_PIXEL_DATA_CONSTRUCTOR,
                &args![block, width, height, FONT_PIXEL_FORMAT, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let size = e
            .mem
            .u32(header)
            .wrapping_mul(e.mem.u32(header + 4))
            .wrapping_shl(2);
        // `pixels + offsets[0] + 0 * offsets[faces]`: the second word is
        // read and multiplied by zero.
        let offsets = e.mem.u32(pixels + 0x5c);
        let faces = e.mem.u32(pixels + 0x60);
        let _ = e.mem.u32(offsets.wrapping_add(faces.wrapping_mul(4)));
        let buffer = e.mem.u32(pixels + 0x50).wrapping_add(e.mem.u32(offsets));
        file_read(e, file, buffer, size);
        e.mem.set_u8(pixels + 0x70, 1);
        let block = e.call(NI_NEW, &args![0x30u32]).u32();
        let texture = if block != 0 {
            let file_name = e.get(this, Font::pFontFile);
            let handle = if file_name == 0 {
                e.global::<u32>(NI_FIXED_STRING_EMPTY)
            } else {
                e.call(NI_FIXED_STRING_CREATE, &args![file_name]).u32()
            };
            let mut texture = 0;
            release_after_use(e, handle, |e, local| {
                texture = e
                    .call(TEXTURE_FROM_PIXEL_DATA, &args![block, pixels, local, prefs])
                    .u32();
            });
            texture
        } else {
            0
        };
        delete_file(e, file);
        e.call(FONT_TEXTURE_SETUP, &args![texture, 1u32]);
        let slot = this.addr() + 0x0c + index.wrapping_mul(4);
        e.call(NI_POINTER_ASSIGN, &args![slot, texture]);
        true
    })
}

/// `" "` (`01020770`), the string the line extraction sets first.
pub(crate) const SPACE_STRING: u32 = 0x0102_0770;

/// The length of a [`BSStringT`]-shaped string: `strlen` when the stored
/// length is 0xFFFF.
fn string_length(e: &mut Engine, text: Ptr<BSStringT>) -> u32 {
    if e.get(text, BSStringT::sLen) == 0xffff {
        let s = e.get(text, BSStringT::pString);
        e.call(STRLEN, &args![s]).u32()
    } else {
        e.get(text, BSStringT::sLen) as u32
    }
}

/// `BSStringT<char>::operator+=` (`00404820`, Xbox PDB name): `(this, text)`.
pub(crate) const BS_STRING_APPEND: u32 = 0x0040_4820;

/// `"\n\n"` (`01095188`).
pub(crate) const TWO_NEWLINES: u32 = 0x0109_5188;

/// `"\n"` (`01012648`).
pub(crate) const ONE_NEWLINE: u32 = 0x0101_2648;

/// The address of `text[text ? i : 0]` for a [`BSStringT`].
fn guarded_at(e: &Engine, text: Ptr<BSStringT>, i: u32) -> u32 {
    let s = e.get(text, BSStringT::pString);
    s.wrapping_add(if s != 0 { i } else { 0 })
}

layout! {
    /// `FontManager` (Xbox PDB), 0x24 bytes: eight `Font` pointers and the
    /// new-fonts flag.
    pub struct FontManager: 0x24 {
        /// `pFont[0]` (Xbox PDB); font `n` is at `+4 * n`.
        0x00 pFont: u32,
        /// `bUseNewFonts` (Xbox PDB).
        0x20 bUseNewFonts: u8,
    }
}

/// `FontManager::sFont_1_Filename` (Xbox PDB) .. `sFont_8_Filename`: 12-byte
/// settings from `011f3414`, each with its string value at +4.
pub(crate) const FONT_FILENAME_SETTINGS: u32 = 0x011f_3414;

/// The fatal error reporter (`0040fbe0`, cdecl).
pub(crate) const FATAL_ERROR: u32 = 0x0040_fbe0;

/// `"Missing all font data. The program will be terminated."`
/// (`0109518c`).
pub(crate) const MISSING_FONT_DATA_MESSAGE: u32 = 0x0109_518c;

/// `memcpy_s` (`00ec7c66`, cdecl): `(dest, size, source, count)`.
pub(crate) const MEMCPY_S: u32 = 0x00ec_7c66;

/// `BSStringT<char>` copy assignment (`004047f0`): `(this, source)`.
pub(crate) const BS_STRING_ASSIGN: u32 = 0x0040_47f0;

/// `"Font::CollectTo() -- Found GameSetting w/o ending ; in HTML text."`
/// (`010951c8`).
pub(crate) const COLLECT_TO_NO_SEMICOLON: u32 = 0x0109_51c8;

layout! {
    /// `FontManager::TextDoc` (Xbox PDB), 0x18 bytes: the pages and their
    /// size, and the current page number.
    pub struct TextDoc: 0x18 {
        /// `xPages` (Xbox PDB), an [`NiTPointerList`] of `TextPage *`.
        0x00 xPages: Inline<NiTPointerList>,
        /// `iPageWidth` (Xbox PDB).
        0x0C iPageWidth: i32,
        /// `iPageHeight` (Xbox PDB).
        0x10 iPageHeight: i32,
        /// `iPageNum` (Xbox PDB).
        0x14 iPageNum: i32,
    }

    /// `FontManager::TextPage` (Xbox PDB), 0x44 bytes.
    pub struct TextPage: 0x44 {
        /// `xLines` (Xbox PDB), an [`NiTPointerList`] of `TextLine *`.
        0x00 xLines: Inline<NiTPointerList>,
        /// `iWidth` (Xbox PDB).
        0x0C iWidth: i32,
        /// `iHeight` (Xbox PDB).
        0x10 iHeight: i32,
        /// `iPageWidth` (Xbox PDB).
        0x14 iPageWidth: i32,
        /// `iPageHeight` (Xbox PDB).
        0x18 iPageHeight: i32,
        /// `iLastFontHeight` (Xbox PDB).
        0x1C iLastFontHeight: i32,
        /// `pCharsPerFont[0]` (Xbox PDB); font `n` at `+4 * n`, eight.
        0x20 pCharsPerFont: i32,
        /// `pDoc` (Xbox PDB).
        0x40 pDoc: u32,
    }

    /// `FontManager::TextLine` (Xbox PDB), 0x30 bytes.
    pub struct TextLine: 0x30 {
        /// `xChars` (Xbox PDB), an [`NiTPointerList`] of `CharData *`.
        0x00 xChars: Inline<NiTPointerList>,
        /// `iWidth` (Xbox PDB).
        0x0C iWidth: i32,
        /// `iNextSpacing` (Xbox PDB).
        0x10 iNextSpacing: i32,
        /// `iRise` (Xbox PDB).
        0x14 iRise: i32,
        /// `iDrop` (Xbox PDB).
        0x18 iDrop: i32,
        /// `iSkippedSpace` (Xbox PDB).
        0x1C iSkippedSpace: i32,
        /// `iJustify` (Xbox PDB).
        0x20 iJustify: i32,
        /// `iPageWidth` (Xbox PDB).
        0x24 iPageWidth: i32,
        /// `iY` (Xbox PDB).
        0x28 iY: i32,
        /// `pPage` (Xbox PDB).
        0x2C pPage: u32,
    }
}

/// Appends `*item` to an `NiTPointerList` (`00ae8bf0`, thiscall on the
/// list; the engine map's name for the folded body is
/// `BSAudioManager::GetFromCache`).
pub(crate) const POINTER_LIST_ADD_TAIL: u32 = 0x00ae_8bf0;

/// The texture atlas manager pointer `AddTextIcon` passes (`011f336c`).
pub(crate) const TEXTURE_ATLAS_MANAGER: u32 = 0x011f_336c;

/// `Tile::GetTextureAtlasInfo` (`00a06dd0`, Xbox PDB name, cdecl):
/// `(manager, name, atlas name out, texture rectangle out)`.
pub(crate) const TILE_GET_TEXTURE_ATLAS_INFO: u32 = 0x00a0_6dd0;

/// The button icon array's slot reservation (`00a1c130`, thiscall on the
/// array): returns the new element's index.
pub(crate) const BUTTON_ICONS_NEW_INDEX: u32 = 0x00a1_c130;

/// The button icon array's element construction (`00a1c1f0`, thiscall on
/// the array): `(element, count)`.
pub(crate) const BUTTON_ICONS_CONSTRUCT: u32 = 0x00a1_c1f0;

/// The `InterlockedIncrement` import slot (`00fdf1c0`).
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// The line paddings: `0101e570` (4.0 in the exe) for fonts 2 and 6,
/// `01050c48` (6.0) for font 3, `010162c0` (2.0) for font 7.
pub(crate) const PADDING_FONTS_2_6: u32 = 0x0101_e570;

pub(crate) const PADDING_FONT_3: u32 = 0x0105_0c48;

pub(crate) const PADDING_FONT_7: u32 = 0x0101_62c0;

/// `BSStringT<char>` assignment from another string (`00438470`): `(this,
/// source)`.
pub(crate) const BS_STRING_ASSIGN_FROM: u32 = 0x0043_8470;

/// The default character color: red `01095268`, green `01095264`, blue
/// `01095260` (alpha 1).
pub(crate) const DEFAULT_RED: u32 = 0x0109_5268;

pub(crate) const DEFAULT_GREEN: u32 = 0x0109_5264;

pub(crate) const DEFAULT_BLUE: u32 = 0x0109_5260;

layout! {
    /// `FontManager::CharData` (Xbox PDB), 0x38 bytes.
    pub struct CharData: 0x38 {
        /// `iFontIndex` (Xbox PDB).
        0x00 iFontIndex: i32,
        /// `cChar` (Xbox PDB).
        0x04 cChar: u8,
        /// `xColor` (Xbox PDB), an `NiColorA` (four floats).
        0x08 xColor: u32,
        /// `iJustification` (Xbox PDB).
        0x18 iJustification: i32,
        /// `xFilename` (Xbox PDB), a [`BSStringT`].
        0x1C xFilename: Inline<BSStringT>,
        /// `iWidth` (Xbox PDB).
        0x24 iWidth: i32,
        /// `iRise` (Xbox PDB).
        0x28 iRise: i32,
        /// `iDrop` (Xbox PDB).
        0x2C iDrop: i32,
        /// `iLeadingEdge` (Xbox PDB).
        0x30 iLeadingEdge: i32,
        /// `iX` (Xbox PDB).
        0x34 iX: i32,
    }
}

/// The font `index` of the font manager.
fn manager_font(e: &Engine, index: i32) -> u32 {
    let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
    e.mem
        .u32(manager.wrapping_add((index as u32).wrapping_mul(4)))
}

/// `FCOMP` against `0.0` with `TEST AH,0x44; JNP`: true only for an exact
/// (ordered) zero.
fn x87_is_zero(value: f64) -> bool {
    value == 0.0
}

/// `NiTPointerListBase<..>::RemoveHead` (`00ae7f90`, thiscall on the list):
/// unlinks the first item and returns its element.
pub(crate) const POINTER_LIST_REMOVE_HEAD: u32 = 0x00ae_7f90;

/// The `NiTPointerList` destructor (`00ae8370`, thiscall on the list).
pub(crate) const POINTER_LIST_DESTRUCTOR: u32 = 0x00ae_8370;

/// `NiTPointerListBase<..>::RemoveAll` (`00a1c2d0`, thiscall on the list).
pub(crate) const POINTER_LIST_REMOVE_ALL: u32 = 0x00a1_c2d0;

/// Removes and deletes every element of the list at `list` (each destroyed
/// by `destroy` when not null, then freed), then destroys the list.
fn delete_list_elements(e: &mut Engine, list: u32, destroy: fn(&mut Engine, u32)) {
    while e.mem.u32(list + 8) != 0 {
        let element = e.call(POINTER_LIST_REMOVE_HEAD, &args![list]).u32();
        if element != 0 {
            destroy(e, element);
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, element]);
        }
    }
    e.call(POINTER_LIST_DESTRUCTOR, &args![list]);
}

/// Clears the head, tail and count of the `NiTPointerList` at `list` (count
/// first, as the constructors write them).
fn clear_pointer_list(e: &mut Engine, list: u32) {
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

/// The `BSSimpleArray<ButtonIcon, 1024>` vtable (`0109527c`).
pub(crate) const BUTTON_ICON_ARRAY_VTABLE: u32 = 0x0109_527c;

/// The point `CalculateStringDimensions` starts from and returns for bad
/// arguments (`011f426c`, three floats).
pub(crate) const DIMENSIONS_START: u32 = 0x011f_426c;

/// `fWidth`, `fLeadingEdge` and `fSpacing` of the letter at `at` added in
/// the code's order: `(lead + width) + spacing`.
fn letter_extent(e: &Engine, at: u32) -> f32 {
    ((e.mem.f32(at + 0x2c) as f64 + e.mem.f32(at + 0x24) as f64) + e.mem.f32(at + 0x30) as f64)
        as f32
}

/// The larger of `current` and `candidate` as `FCOMPP; TEST AH,0x05; JP`
/// picks it: `candidate` unless it is below `current` (so also on NaN).
fn x87_keep_larger(current: f32, candidate: f32) -> f32 {
    if (candidate as f64) < current as f64 {
        x87_quiet(current)
    } else {
        x87_quiet(candidate)
    }
}

/// `value` plus the `-` letter's leading edge, width and spacing, added one
/// at a time as `CalculateStringDimensions` does (`+0xA04`, `+0x9FC`,
/// `+0xA08` from the first letter).
fn add_dash_extent(e: &Engine, letters: u32, value: f64) -> f64 {
    ((value + e.mem.f32(letters + 0xa04) as f64) + e.mem.f32(letters + 0x9fc) as f64)
        + e.mem.f32(letters + 0xa08) as f64
}

layout! {
    /// `FontManager::TextData` (Xbox PDB), 0x40 bytes: the request and the
    /// results of `FontManager::PrepText` (named so to tell it from
    /// `Font::TextData`, [`TextData`]).
    pub struct ManagerTextData: 0x40 {
        /// `iDefaultFont` (Xbox PDB).
        0x00 iDefaultFont: i32,
        /// `iJustification` (Xbox PDB).
        0x04 iJustification: i32,
        /// `xColor` (Xbox PDB), an `NiColorA` (four floats).
        0x08 xColor: u32,
        /// `cLineSep` (Xbox PDB).
        0x18 cLineSep: u8,
        /// `iWidth` (Xbox PDB).
        0x1C iWidth: i32,
        /// `iHeight` (Xbox PDB).
        0x20 iHeight: i32,
        /// `iPageNum` (Xbox PDB).
        0x24 iPageNum: i32,
        /// `iLines` (Xbox PDB).
        0x28 iLines: i32,
        /// `iNumPages` (Xbox PDB).
        0x2C iNumPages: i32,
        /// `iNumLines` (Xbox PDB).
        0x30 iNumLines: i32,
        /// `bIsHypertext` (Xbox PDB).
        0x34 bIsHypertext: u8,
        /// `xNewText` (Xbox PDB), a [`BSStringT`].
        0x38 xNewText: Inline<BSStringT>,
    }
}

/// A new `TextDoc` (0x18 bytes; 0 when the allocation fails) with no pages
/// and the page size and number of `info`.
fn new_text_doc(e: &mut Engine, info: Ptr<ManagerTextData>) -> Ptr<TextDoc> {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x18u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let page_num = e.get(info, ManagerTextData::iPageNum);
    let height = e.get(info, ManagerTextData::iHeight);
    let width = e.get(info, ManagerTextData::iWidth);
    ni_tpointer_list_text_page_ni_tpointer_list(e, Ptr::new(block));
    let doc: Ptr<TextDoc> = Ptr::new(block);
    e.set(doc, TextDoc::iPageWidth, width);
    e.set(doc, TextDoc::iPageHeight, height);
    e.set(doc, TextDoc::iPageNum, page_num);
    doc
}

/// The page measurements both text preparations store: the number of
/// pages, then for the current page the total line height (rise plus
/// skipped space), the widest line and the line count.
fn store_page_measurements(e: &mut Engine, info: Ptr<ManagerTextData>, doc: Ptr<TextDoc>) {
    let pages = e.mem.u32(doc.addr() + 8) as i32;
    e.set(info, ManagerTextData::iNumPages, pages);
    let page = font_manager_text_doc_current_page(e, doc);
    if page == 0 {
        return;
    }
    e.set(info, ManagerTextData::iHeight, 0);
    e.set(info, ManagerTextData::iWidth, 0);
    let mut node = e.mem.u32(page);
    while node != 0 {
        let line = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        let height = e
            .mem
            .i32(line + 0x14)
            .wrapping_add(e.mem.i32(line + 0x1c))
            .wrapping_add(e.get(info, ManagerTextData::iHeight));
        e.set(info, ManagerTextData::iHeight, height);
        let line_width = e.mem.i32(line + 0x0c);
        let width = e.get(info, ManagerTextData::iWidth);
        e.set(
            info,
            ManagerTextData::iWidth,
            if width > line_width {
                width
            } else {
                line_width
            },
        );
    }
    let lines = e.mem.i32(page + 8);
    e.set(info, ManagerTextData::iNumLines, lines);
}

/// `toupper` (`00eca7f4`, cdecl).
pub(crate) const TOUPPER: u32 = 0x00ec_a7f4;

/// `_stricmp` (`00ec68e4`, cdecl).
pub(crate) const STRICMP: u32 = 0x00ec_68e4;

/// `atol` (`00eca6d3`, cdecl).
pub(crate) const ATOL: u32 = 0x00ec_a6d3;

/// `sscanf` (`00eca4a6`, cdecl, variable arguments).
pub(crate) const SSCANF: u32 = 0x00ec_a4a6;

/// `BSStringT<char>::StrCmp` (`00408a80`): `(this, text, ignore case)`, 0
/// when equal.
pub(crate) const BS_STRING_STR_CMP: u32 = 0x0040_8a80;

/// `BSStringT<char>::operator+=` with another string (`007e0d80`, Xbox PDB
/// name `operator+=` (ov2)): `(this, &source)`.
pub(crate) const BS_STRING_APPEND_STRING: u32 = 0x007e_0d80;

/// `""` (`01011584`).
pub(crate) const EMPTY_TEXT: u32 = 0x0101_1584;

/// `"true"` (`0102fd2c`).
pub(crate) const TRUE_TEXT: u32 = 0x0102_fd2c;

/// `"%i"` (`01020774`).
pub(crate) const PERCENT_I: u32 = 0x0102_0774;

/// `255.0` (`double`, `0101e568`).
pub(crate) const COLOR_SCALE: u32 = 0x0101_e568;

/// The hypertext tag and attribute names: `"IMG"`, `"SRC"`, `"WIDTH"`,
/// `"HEIGHT"`, `"DIV"`, `"ALIGN"`, `"LEFT"`, `"CENTER"`, `"RIGHT"`,
/// `"FONT"`, `"FACE"`, `"COLOR"`, `"BR"`, `"P"`, `"HR"`, `"/FONT"`.
pub(crate) const TAG_IMG: u32 = 0x0109_525c;

pub(crate) const ATTR_SRC: u32 = 0x0109_5258;

pub(crate) const ATTR_WIDTH: u32 = 0x0109_5250;

pub(crate) const ATTR_HEIGHT: u32 = 0x0109_5248;

pub(crate) const TAG_DIV: u32 = 0x0109_5244;

pub(crate) const ATTR_ALIGN: u32 = 0x0109_523c;

pub(crate) const VALUE_LEFT: u32 = 0x0109_0cb8;

pub(crate) const VALUE_CENTER: u32 = 0x0109_5234;

pub(crate) const VALUE_RIGHT: u32 = 0x0109_0cb0;

pub(crate) const TAG_FONT: u32 = 0x0109_522c;

pub(crate) const ATTR_FACE: u32 = 0x0109_5224;

pub(crate) const ATTR_COLOR: u32 = 0x0109_521c;

pub(crate) const TAG_BR: u32 = 0x0109_5218;

pub(crate) const TAG_P: u32 = 0x0106_3e34;

pub(crate) const TAG_HR: u32 = 0x0109_5214;

pub(crate) const TAG_END_FONT: u32 = 0x0109_520c;

/// `PrepHypertext`'s stack frame: the parse position, the stopping type and
/// character `CollectTo` writes, five strings (text run, tag, attribute
/// name, attribute value, collected text), a temporary string and the
/// current `CharData`.
struct Hypertext {
    this: Ptr<FontManager>,
    buffer: u32,
    pos: u32,
    kind: u32,
    character: u32,
    run: Ptr<BSStringT>,
    tag: Ptr<BSStringT>,
    name: Ptr<BSStringT>,
    value: Ptr<BSStringT>,
    collected: Ptr<BSStringT>,
    temporary: Ptr<BSStringT>,
    data: Ptr<CharData>,
    doc: Ptr<TextDoc>,
    pending_image: bool,
    skipped: i32,
    new_page: u8,
    icons: u32,
}

impl Hypertext {
    fn kind(&self, e: &Engine) -> u32 {
        e.mem.u32(self.kind)
    }

    /// `CollectTo` into the temporary string, which is then emptied (the
    /// result is not kept).
    fn skip(&self, e: &mut Engine, stop: u32, accept: u32) {
        font_manager_collect_to(
            e,
            self.this,
            self.temporary,
            self.buffer,
            Ptr::new(self.pos),
            stop,
            accept,
            Ptr::new(self.kind),
            Ptr::new(self.character),
            0,
        );
        e.call(BS_STRING_SET, &args![self.temporary, 0u32, 0u32]);
    }

    /// `CollectTo` (collecting) assigned to `into`.
    fn collect(&self, e: &mut Engine, into: Ptr<BSStringT>, stop: u32) {
        let r = font_manager_collect_to(
            e,
            self.this,
            self.temporary,
            self.buffer,
            Ptr::new(self.pos),
            stop,
            0,
            Ptr::new(self.kind),
            Ptr::new(self.character),
            1,
        );
        e.call(BS_STRING_ASSIGN_FROM, &args![into, r]);
        e.call(BS_STRING_SET, &args![self.temporary, 0u32, 0u32]);
    }

    /// Upper-cases a string in place (`toupper` on each signed character).
    fn upper(&self, e: &mut Engine, string: Ptr<BSStringT>) {
        let mut k: u32 = 0;
        while k < string_length(e, string) {
            let at = guarded_at(e, string, k);
            let c = e.mem.i8(at);
            let upper = e.call(TOUPPER, &args![c as i32]).u32();
            e.mem.set_u8(at, upper as u8);
            k += 1;
        }
    }

    fn equals(&self, e: &mut Engine, string: Ptr<BSStringT>, text: u32) -> bool {
        e.call(BS_STRING_STR_CMP, &args![string, text, 1u32]).u32() == 0
    }

    /// Adds a copy of the current character to the document with the
    /// skipped space and new-page flag, which are then cleared.
    fn add_copy(&mut self, e: &mut Engine) {
        let copy = font_manager_char_data_copy(e, self.data);
        font_manager_text_doc_add_char(
            e,
            self.doc,
            copy.addr(),
            self.skipped as u32,
            self.new_page,
        );
        self.skipped = 0;
        self.new_page = 0;
    }

    /// Adds the pending image (character 0) and empties the file name.
    fn flush_image(&mut self, e: &mut Engine) {
        self.pending_image = false;
        e.set(self.data, CharData::cChar, 0);
        self.add_copy(e);
        let file = self.data.at(CharData::xFilename);
        e.call(BS_STRING_SET, &args![file, EMPTY_TEXT, 0u32]);
    }

    /// The value of hexadecimal digit `index` of the attribute value
    /// (`toupper` twice, as the code calls it).
    fn hex_digit(&self, e: &mut Engine, index: u32) -> i32 {
        let at = guarded_at(e, self.value, index);
        let c = e.mem.i8(at) as i32;
        let upper = e.call(TOUPPER, &args![c]).i32();
        let at = guarded_at(e, self.value, index);
        let c = e.mem.i8(at) as i32;
        let again = e.call(TOUPPER, &args![c]).i32();
        if upper < 0x41 {
            again.wrapping_sub(0x30)
        } else {
            again.wrapping_sub(0x37)
        }
    }

    /// A color component from digits `index` and `index + 1`, clamped to
    /// 0..=255.
    fn color_component(&self, e: &mut Engine, index: u32) -> i32 {
        let high = self.hex_digit(e, index).wrapping_shl(4);
        let value = high.wrapping_add(self.hex_digit(e, index + 1));
        let value = if value > 0 { value } else { 0 };
        if value < 0xff {
            value
        } else {
            0xff
        }
    }

    /// One attribute of a tag (the tag and attribute names upper-cased,
    /// the value collected).
    fn apply_attribute(&mut self, e: &mut Engine) {
        if self.equals(e, self.tag, TAG_IMG) {
            self.pending_image = true;
            if self.equals(e, self.name, ATTR_SRC) {
                let file = self.data.at(CharData::xFilename);
                e.call(BS_STRING_ASSIGN_FROM, &args![file, self.value]);
            } else if self.equals(e, self.name, ATTR_WIDTH) {
                let s = e.get(self.value, BSStringT::pString);
                let w = self.data.addr() + 0x24;
                e.call(SSCANF, &args![s, PERCENT_I, w]);
            } else if self.equals(e, self.name, ATTR_HEIGHT) {
                let s = e.get(self.value, BSStringT::pString);
                let h = self.data.addr() + 0x28;
                e.call(SSCANF, &args![s, PERCENT_I, h]);
                e.set(self.data, CharData::iDrop, 0);
            }
        } else if self.pending_image {
            self.flush_image(e);
        }
        if self.equals(e, self.tag, TAG_DIV) {
            self.skipped = self.skipped.wrapping_add(1);
            if self.equals(e, self.name, ATTR_ALIGN) {
                if self.equals(e, self.value, VALUE_LEFT) {
                    e.set(self.data, CharData::iJustification, 1);
                } else if self.equals(e, self.value, VALUE_CENTER) {
                    e.set(self.data, CharData::iJustification, 2);
                } else if self.equals(e, self.value, VALUE_RIGHT) {
                    e.set(self.data, CharData::iJustification, 4);
                }
            }
        } else if self.equals(e, self.tag, TAG_FONT) {
            if self.equals(e, self.name, ATTR_FACE) {
                for i in 0..8i32 {
                    let font = e.mem.u32(self.this.addr() + 4 * i as u32);
                    let file = e.mem.u32(font + 4);
                    let s = e.get(self.value, BSStringT::pString);
                    let mut matched = e.call(STRICMP, &args![s, file]).u32() == 0;
                    if !matched {
                        let s = e.get(self.value, BSStringT::pString);
                        matched = i.wrapping_add(1) == e.call(ATOL, &args![s]).i32();
                    }
                    if matched {
                        e.set(self.data, CharData::iFontIndex, i);
                        let c = e.get(self.data, CharData::cChar);
                        font_manager_char_data_set_char(e, self.data, c);
                        break;
                    }
                }
            }
            if self.equals(e, self.name, ATTR_COLOR) && string_length(e, self.value) == 6 {
                let red = self.color_component(e, 0);
                let green = self.color_component(e, 2);
                let blue = self.color_component(e, 4);
                let scale: f64 = e.global(COLOR_SCALE);
                let color = [
                    (red as f64 / scale) as f32,
                    (green as f64 / scale) as f32,
                    (blue as f64 / scale) as f32,
                    1.0,
                ];
                for (k, v) in color.iter().enumerate() {
                    e.mem.set_f32(self.data.addr() + 8 + 4 * k as u32, *v);
                }
            }
        }
    }
}

/// True when the `CharData` at `data` is text: no file name, or an empty
/// one.
fn is_text_char(e: &Engine, data: u32) -> bool {
    let file = e.mem.u32(data + 0x1c);
    file == 0 || e.mem.u8(file) == 0
}

/// Counts one more character of the `CharData`'s font on `page`.
fn count_font_char(e: &mut Engine, page: u32, data: u32) {
    let font = e.mem.u32(data);
    let at = page.wrapping_add(0x20).wrapping_add(font.wrapping_mul(4));
    let n = e.mem.i32(at).wrapping_add(1);
    e.mem.set_i32(at, n);
}

/// The per-font line rise a new line starts with (`011a709c`, one `int`
/// per font index).
pub(crate) const FONT_LINE_RISES: u32 = 0x011a_709c;

/// `NiTPointerList::RemoveTail` (`00ae8010`, thiscall on the list): unlinks
/// the last item and returns its element.
pub(crate) const POINTER_LIST_REMOVE_TAIL: u32 = 0x00ae_8010;

/// `NiTPointerList::AddHead` (`00a0c9f0`, thiscall on the list): `(&item)`.
pub(crate) const POINTER_LIST_ADD_HEAD: u32 = 0x00a0_c9f0;

/// A new `TextLine` (0x30 bytes; 0 when the allocation fails) on the same
/// page as `line`, starting with `char_data`, its rise set from the
/// character's font (`011a709c`).
fn new_line_after(e: &mut Engine, line: Ptr<TextLine>, char_data: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x30u32]).u32();
    let new_line = if block != 0 {
        let page = e.get(line, TextLine::pPage);
        let width = e.get(line, TextLine::iPageWidth);
        fn_00a1bd40(e, Ptr::new(block), Ptr::new(page), char_data, 0, width).addr()
    } else {
        0
    };
    let font = e.mem.u32(char_data);
    let rise: i32 = e.global(FONT_LINE_RISES.wrapping_add(font.wrapping_mul(4)));
    e.mem.set_i32(new_line + 0x14, rise);
    new_line
}

/// Re-lays the line from its first character: `iX` = width so far plus the
/// leading edge, width grows by each character's width.
fn relayout_line(e: &mut Engine, line: Ptr<TextLine>) {
    let mut node = e.mem.u32(line.addr());
    while node != 0 {
        let c = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        let x = e
            .get(line, TextLine::iWidth)
            .wrapping_add(e.mem.i32(c + 0x30));
        e.mem.set_i32(c + 0x34, x);
        let width = e
            .get(line, TextLine::iWidth)
            .wrapping_add(e.mem.i32(c + 0x24));
        e.set(line, TextLine::iWidth, width);
    }
}

/// `line.field = max(line.field, value)` the way the code compares
/// (`CMP field, value; JLE` takes `value`).
fn keep_max(e: &mut Engine, line: Ptr<TextLine>, field: Field<TextLine, i32>, value: i32) {
    let current = e.get(line, field);
    e.set(line, field, if current > value { current } else { value });
}

/// `TileImage::AddToTesTextures` (`00a1fa20`, Xbox PDB name, cdecl):
/// `(path, &scale, &texture, float, NiPointer by value)`.
pub(crate) const TILE_IMAGE_ADD_TO_TES_TEXTURES: u32 = 0x00a1_fa20;

/// `BSShaderManager::GetShader` (`00b55560`, Xbox PDB name, cdecl).
pub(crate) const BS_SHADER_MANAGER_GET_SHADER: u32 = 0x00b5_5560;

/// The color of the text geometry `TextDoc::Render` makes: red `01095274`,
/// green `01095270`, blue `0109526c` (alpha 1).
pub(crate) const RENDER_RED: u32 = 0x0109_5274;

pub(crate) const RENDER_GREEN: u32 = 0x0109_5270;

pub(crate) const RENDER_BLUE: u32 = 0x0109_526c;

/// `NiAVObject` update with an `NiUpdateData` (`00a59c60`, thiscall).
pub(crate) const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;

/// `NiTPointerList` node insertion at the head (`00a1c270`, thiscall on the
/// list): `(node)`.
pub(crate) const POINTER_LIST_ADD_NODE_HEAD: u32 = 0x00a1_c270;

/// A texture dimension as `TextDoc`'s image code scales it:
/// `(scale * size) / dimension`, the dimension an unsigned word.
fn scaled_uv(scale: f32, size: i32, dimension: u32) -> f32 {
    ((scale as f64 * size as f64) / dimension as f64) as f32
}

/// Releases the reference `texture` holds (the count at +4; at zero the
/// object deletes itself through virtual slot 4).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
}

/// Finishes a rendered geometry: its bound, its position at the start
/// point (`011f426c`), its properties and an update (an `NiUpdateData` of
/// time 0 and cleared flags), then attaches it to `out` (virtual slot
/// 0xDC, with 1).
fn attach_rendered(e: &mut Engine, out: u32, shape: u32) {
    compute_geometry_bound(e, shape);
    for k in 0..3 {
        let v: u32 = e.global(DIMENSIONS_START + 4 * k);
        e.mem.set_u32(shape + 0x58 + 4 * k, v);
    }
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![shape]);
    e.with_stack(0x0c, |e, update| {
        e.mem.set_f32(update.addr(), 0.0);
        for k in 4..9 {
            e.mem.set_u8(update.addr() + k, 0);
        }
        e.call(NI_AV_OBJECT_UPDATE, &args![shape, update]);
    });
    e.vcall(out, 0xdc, &args![shape, 1u32]);
}

// Translated from 00a11c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<DFALL<NiTListItem<NiTriShape*>>, NiTriShape*>::AddTail`
/// (Xbox PDB): allocates a 12-byte list item, stores `*element` in it and
/// links it at the tail (`AddNodeTail`).
pub fn ni_tpointer_list_base_add_tail(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) {
    let item: Ptr<NiTListItem> = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x0cu32]).ptr();
    let value = e.mem.u32(element.addr());
    e.set(item, NiTListItem::m_element, value);
    e.call(NI_TLIST_ADD_NODE_TAIL, &args![this, item]);
}

// Translated from 00a12020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::Font` (Xbox PDB): constructs the texture pointers, the icon atlas
/// name and the button icon array, then (with the per-thread allocation word
/// set to 0xC) clears the reference count, file name and font data; with a
/// non-empty `filename` it copies the name, stores `font_num`, and loads the
/// font when `load` is set. Returns `this`.
// Translation incomplete: 00a12020 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_font(
    e: &mut Engine,
    this: Ptr<Font>,
    font_num: i32,
    filename: u32,
    load: u8,
) -> Ptr<Font> {
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            this.addr() + 0x0c,
            4u32,
            8u32,
            0x00a1_b410u32,
            NI_POINTER_DESTRUCTOR
        ],
    );
    let name = this.at(Font::IconAtlasTextureName);
    e.set(name, BSStringT::pString, 0);
    e.set(name, BSStringT::sLen, 0);
    e.set(name, BSStringT::sMaxLen, 0);
    let icons = this.at(Font::ButtonIcons);
    fn_00a1bef0(e, icons);
    with_tls_scope(e, 0x0c, |e| {
        e.set(this, Font::pFontFile, 0);
        e.set(this, Font::iRefCount, 0);
        e.set(this, Font::pFontData, 0);
        if filename != 0 {
            let len = e.call(STRLEN, &args![filename]).u32();
            if len != 0 {
                let copy = e.call(ALLOCATE, &args![MEMORY_MANAGER, len + 1]).u32();
                e.set(this, Font::pFontFile, copy);
                e.call(STRCPY_S, &args![copy, len + 1, filename]);
            }
            e.set(this, Font::iFontNum, font_num);
            if load != 0 {
                font_load(e, this);
            }
        }
    });
    this
}

// Translated from 00a121e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Font` destructor: frees and clears this unit's static block
/// (`011f33fc`, and the word `011f3400`), releases the textures and the
/// font data (`00a12300`), frees the file name, destroys the button icon
/// array, empties the icon atlas name and destroys the texture pointers.
// Translation incomplete: 00a121e0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a121e0(e: &mut Engine, this: Ptr<Font>) {
    let block: u32 = e.global(FONT_STATIC_BLOCK);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    e.set_global(FONT_STATIC_BLOCK_WORD, 0u32);
    e.set_global(FONT_STATIC_BLOCK, 0u32);
    fn_00a12300(e, this);
    let file = e.get(this, Font::pFontFile);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, file]);
    let icons = this.at(Font::ButtonIcons);
    bs_simple_array_button_icon_destructor(e, icons);
    let name = this.at(Font::IconAtlasTextureName);
    e.call(BS_STRING_SET, &args![name, 0u32, 0u32]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![this.addr() + 0x0c, 4u32, 8u32, NI_POINTER_DESTRUCTOR],
    );
}

// Translated from 00a122b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::ConvertCharacter` (Xbox PDB): turns the typographic quotes 0x91 and
/// 0x92 into `'` and 0x93 and 0x94 into `"`, in place.
pub fn font_convert_character(e: &mut Engine, c: Ptr) {
    match e.mem.u8(c.addr()) {
        0x91 | 0x92 => e.mem.set_u8(c.addr(), b'\''),
        0x93 | 0x94 => e.mem.set_u8(c.addr(), b'"'),
        _ => {}
    }
}

// Translated from 00a12300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the font's textures (clears the first `iTextureCount` texture
/// pointers) and frees the font data.
pub fn fn_00a12300(e: &mut Engine, this: Ptr<Font>) {
    let mut i: i32 = 0;
    loop {
        let data = e.get(this, Font::pFontData);
        if data == 0 || i >= e.mem.i32(data + 4) {
            break;
        }
        let slot = this.addr() + 0x0c + 4 * i as u32;
        e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
        i += 1;
    }
    let data = e.get(this, Font::pFontData);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, data]);
    e.set(this, Font::pFontData, 0);
}

// Translated from 00a12370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::GetLineOffsets` (Xbox PDB): the start of a line for the alignment
/// in the low four bits of `flags`. `*offset` holds the line's width limit on
/// entry (`FLT_MAX` when not above 0, NaN included) and receives the x
/// offset: minus half the measured width for 2 (centered), minus the width
/// for 4 (right), else 0. The measurement is
/// `FontManager::CalculateStringDimensions` of `text` from line `line`.
pub fn font_get_line_offsets(
    e: &mut Engine,
    this: Ptr<Font>,
    text: u32,
    offset: Ptr,
    _unused_3: u32,
    flags: u32,
    line: u32,
) {
    let entry = e.mem.f32(offset.addr());
    let zero: f64 = e.global(ZERO_DOUBLE);
    // FCOMP; TEST AH,0x41: below, equal or unordered.
    let not_above = !x87_above(entry as f64, zero);
    let limit = if not_above {
        x87_quiet(e.global::<f32>(FLOAT_MAX))
    } else {
        x87_quiet(entry)
    };
    // The measured width (the first float of the dimensions).
    let measure = |e: &mut Engine| {
        let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
        let font_num = e.get(this, Font::iFontNum);
        e.with_stack(0x0c, |e, result| {
            let r = font_manager_calculate_string_dimensions(
                e,
                Ptr::new(manager),
                result,
                text,
                font_num,
                limit,
                line as i32,
            );
            e.mem.f32(r.addr())
        })
    };
    match flags & 0xf {
        2 => {
            let width = measure(e);
            let two: f64 = e.global(TWO_DOUBLE);
            e.mem.set_f32(offset.addr(), (-(width as f64) / two) as f32);
        }
        4 => {
            let width = measure(e);
            e.mem.set_f32(offset.addr(), x87_quiet(-width));
        }
        _ => e.mem.set_f32(offset.addr(), 0.0),
    }
}

// Translated from 00a12460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::MakeString` (Xbox PDB): builds the string geometry of `text` (a
/// [`BSStringT`]-shaped pointer and length; length 0xFFFF means "measure
/// with `strlen`") and returns it, or 0 for an empty string, a font without
/// data, or a string whose first character is the terminator.
///
/// The cursor starts at `x` plus the alignment offset of the first line
/// (`GetLineOffsets` with `*width` as the limit). With `raise` set, `y` is
/// lowered by twice the space letter's height above the base line. The
/// geometry (`MakeTriShape` for the character count) is placed at
/// `(x, z, y)`. Each character: 3 drops the icon rectangle, a tab advances
/// the cursor to the next multiple of the tab width, a newline restarts the
/// cursor at the next line's offset and moves `y` down by the base line;
/// the character (typographic quotes converted) is added (`AddChar`) with
/// the current rectangle, `*width` becomes the widest cursor travel so far
/// (`_ftol2`), and 2 starts using the rectangle (0, 0, 1, 1). Finally the
/// geometry's bound is computed from its vertices.
#[allow(clippy::too_many_arguments)]
pub fn font_make_string(
    e: &mut Engine,
    this: Ptr<Font>,
    x: f32,
    y: f32,
    z: f32,
    text: Ptr<BSStringT>,
    width: Ptr,
    flags: u32,
    shape_arg: u32,
    raise: u8,
    shape_flag: u8,
) -> u32 {
    let length = |e: &mut Engine| -> u32 {
        if e.get(text, BSStringT::sLen) == 0xffff {
            let s = e.get(text, BSStringT::pString);
            e.call(STRLEN, &args![s]).u32()
        } else {
            e.get(text, BSStringT::sLen) as u32
        }
    };
    if length(e) == 0 || e.get(this, Font::pFontData) == 0 {
        return 0;
    }
    // Stack locals: +0 the pen position (x, then `z`, then `y`: the point
    // `AddChar` reads), +0xc line offset, +0x10 unused out, +0x14 rectangle
    // (four floats), +0x24 character.
    e.with_stack(0x28, |e, locals| {
        let cursor = locals.addr();
        let depth_at = cursor + 4;
        let line_y_at = cursor + 8;
        let offset = cursor + 0x0c;
        let unused = cursor + 0x10;
        let rect = cursor + 0x14;
        let character = cursor + 0x24;

        e.mem.set_f32(offset, e.mem.i32(width.addr()) as f32);
        let s = e.get(text, BSStringT::pString);
        font_get_line_offsets(e, this, s, Ptr::new(offset), unused, flags, 0);
        e.mem
            .set_f32(cursor, (x as f64 + e.mem.f32(offset) as f64) as f32);
        e.mem.set_f32(line_y_at, x87_quiet(y));
        e.mem.set_f32(depth_at, x87_quiet(z));
        if raise != 0 {
            let data = e.get(this, Font::pFontData);
            let rise = e.mem.f32(data + 0x850) as f64 - e.mem.f32(data) as f64;
            let line_y = e.mem.f32(line_y_at) as f64 - (rise + rise);
            e.mem.set_f32(line_y_at, line_y as f32);
        }

        let mut count: u32 = 0;
        loop {
            let len = length(e);
            if count >= len {
                break;
            }
            let s = e.get(text, BSStringT::pString);
            if guarded_byte(e, s, count) == 0 {
                break;
            }
            count += 1;
        }
        if count == 0 {
            return 0;
        }

        let shape = font_make_tri_shape(e, this, count, shape_arg, shape_flag);
        e.mem.set_f32(shape + 0x58, x87_quiet(x));
        let depth = x87_quiet(e.mem.f32(depth_at));
        e.mem.set_f32(shape + 0x5c, depth);
        let line_y = x87_quiet(e.mem.f32(line_y_at));
        e.mem.set_f32(shape + 0x60, line_y);
        let mut current_rect: u32 = 0;
        e.mem.set_f32(rect, 0.0);
        e.mem.set_f32(rect + 4, 0.0);
        e.mem.set_f32(rect + 8, 1.0);
        e.mem.set_f32(rect + 0x0c, 1.0);
        e.mem.set_i32(width.addr(), 0);
        let start = x87_quiet(e.mem.f32(cursor));
        let mut index: u32 = 0;
        let mut i: u32 = 0;
        loop {
            let s = e.get(text, BSStringT::pString);
            let c = guarded_byte(e, s, i);
            if c == 0 {
                break;
            }
            if c == 3 {
                current_rect = 0;
            }
            if c == b'\t' {
                let at = x87_quiet(e.mem.f32(cursor));
                let tab = e.global::<f32>(TAB_WIDTH_FLOAT) as f64;
                let rem = e.call(FMOD, &args![at as f64, tab]).f64() as f32;
                let tab_double: f64 = e.global(TAB_WIDTH_DOUBLE);
                let next = (tab_double - rem as f64) + e.mem.f32(cursor) as f64;
                e.mem.set_f32(cursor, next as f32);
            } else if c == b'\n' {
                e.mem.set_f32(offset, e.mem.i32(width.addr()) as f32);
                let s = e.get(text, BSStringT::pString);
                font_get_line_offsets(e, this, s, Ptr::new(offset), unused, flags, i + 1);
                e.mem.set_f32(cursor, x87_quiet(e.mem.f32(offset)));
                let data = e.get(this, Font::pFontData);
                let line_y = e.mem.f32(line_y_at) as f64 - e.mem.f32(data) as f64;
                e.mem.set_f32(line_y_at, line_y as f32);
            }
            let s = e.get(text, BSStringT::pString);
            let c = guarded_byte(e, s, i);
            e.mem.set_u8(character, c);
            font_convert_character(e, Ptr::new(character));
            if e.mem.u8(character) as i32 > 0x100 {
                e.mem.set_u8(character, 0);
            }
            let data = e.get(this, Font::pFontData);
            let char_letter = letter(data, e.mem.u8(character));
            font_add_char(
                e,
                this,
                char_letter,
                index,
                shape,
                Ptr::new(cursor),
                current_rect,
            );
            index += 1;
            let widest = e.mem.i32(width.addr());
            let moved = e.mem.f32(cursor) as f64 - start as f64;
            let travel = ftol(e, moved);
            e.mem
                .set_i32(width.addr(), if travel > widest { travel } else { widest });
            let s = e.get(text, BSStringT::pString);
            if guarded_byte(e, s, i) == 2 {
                current_rect = rect;
            }
            i += 1;
        }
        compute_geometry_bound(e, shape);
        shape
    })
}

// Translated from 00a12880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::CreateText` (Xbox PDB): prepares `*text` (`PrepText` into a
/// `Font::TextData` built from `*width`, `*height` (0 means unlimited),
/// `line_start`, `line_end` (0 means unlimited) and `separator`), returns its
/// measured width and height through `width` and `height`, and builds the
/// string geometry into `*shape_out` (`MakeTriShape` for the prepared
/// character count) and, when the font has button icons, the icon geometry
/// into `*icons_out` (`MakeIconsTriShape`, consistency 0x4000).
///
/// Both are placed at `(0, 0, y)`, `y` twice the base line above the font
/// height. Each line starts at the alignment offset of its measured width
/// (`alignment` 4 right, 2 centered); at `separator` the next line starts
/// one base line plus the line padding lower; a tab advances to the next
/// tab stop. Character 1 adds the next button icon (when the font has any),
/// every other character a letter (`AddChar`); `*width` becomes the widest
/// cursor travel. Then both bounds are computed, the button icons cleared
/// and the text data destroyed.
// Translation incomplete: 00a12880 - C++ exception unwinding (FS:[0] frame) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn font_create_text(
    e: &mut Engine,
    this: Ptr<Font>,
    text: Ptr,
    width: Ptr,
    height: Ptr,
    line_start: i32,
    line_end: i32,
    alignment: i32,
    separator: u8,
    shape_arg: u32,
    shape_out: Ptr,
    icons_out: Ptr,
) {
    if e.mem.i32(height.addr()) == 0 {
        e.mem.set_i32(height.addr(), 0x7fff_ffff);
    }
    let line_end = if line_end == 0 { 0x7fff_ffff } else { line_end };
    let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
    let font_num = e.get(this, Font::iFontNum);
    let padding = font_manager_get_line_padding(e, Ptr::new(manager), font_num);
    // Stack locals: +0 the TextData, +0x28 the pen position (x, then 0,
    // then `y`: the point `AddChar` and `AddIcon` read).
    e.with_stack(0x34, |e, locals| {
        let data_ptr: Ptr<TextData> = locals.cast();
        let cursor = locals.addr() + 0x28;
        let w = e.mem.i32(width.addr());
        let h = e.mem.i32(height.addr());
        e.call(
            TEXT_DATA_CONSTRUCTOR,
            &args![data_ptr, w, h, line_start, line_end, separator as u32],
        );
        let source = e.mem.u32(text.addr());
        font_prep_text(e, this, source, data_ptr);
        let measured_width = e.get(data_ptr, TextData::iWidth);
        e.mem.set_i32(width.addr(), measured_width);
        let measured_height = e.get(data_ptr, TextData::iHeight);
        e.mem.set_i32(height.addr(), measured_height);
        let widths = data_ptr.addr() + 0x20;

        let mut line: i32 = 0;
        let first = aligned_line_start(e, widths, alignment, 0).unwrap_or(0);
        e.mem.set_f32(cursor, first as f32);
        let font_data = e.get(this, Font::pFontData);
        let top = e.mem.f32(font_data) as f64 - e.get(this, Font::fFontHeight) as f64;
        let y_at = cursor + 8;
        e.mem.set_f32(y_at, (top + top) as f32);
        e.mem.set_f32(cursor + 4, 0.0);
        let count = e.get(data_ptr, TextData::iCharCount) as u32;
        let shape = font_make_tri_shape(e, this, count, shape_arg, 1);
        e.mem.set_u32(shape_out.addr(), shape);
        let position = [
            0.0f32,
            x87_quiet(e.mem.f32(cursor + 4)),
            x87_quiet(e.mem.f32(y_at)),
        ];
        let shape = e.mem.u32(shape_out.addr());
        for (k, v) in position.iter().enumerate() {
            e.mem.set_f32(shape + 0x58 + 4 * k as u32, *v);
        }
        let icon_count = e.mem.u32(this.addr() + 0x4c);
        if icon_count != 0 {
            let icons = font_make_icons_tri_shape(e, this);
            e.mem.set_u32(icons_out.addr(), icons);
            let icons = e.mem.u32(icons_out.addr());
            for (k, v) in position.iter().enumerate() {
                e.mem.set_f32(icons + 0x58 + 4 * k as u32, *v);
            }
            let icons = e.mem.u32(icons_out.addr());
            let data = e.mem.u32(icons + 0xb8);
            e.call(NI_GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0x4000u32]);
        }
        let start = x87_quiet(e.mem.f32(cursor));
        let mut char_index: u32 = 0;
        let mut icon_index: u32 = 0;
        let mut i: u32 = 0;
        loop {
            let s = e.mem.u32(data_ptr.addr());
            let c = guarded_byte(e, s, i);
            if c == 0 {
                break;
            }
            if c == separator {
                line += 1;
                e.mem.set_f32(cursor, 0.0);
                if let Some(x) = aligned_line_start(e, widths, alignment, line) {
                    e.mem.set_f32(cursor, x as f32);
                }
                let font_data = e.get(this, Font::pFontData);
                let step = e.mem.f32(font_data) as f64 + padding as f64;
                let y = e.mem.f32(y_at) as f64 - step;
                e.mem.set_f32(y_at, y as f32);
            } else if c == b'\t' {
                let at = x87_quiet(e.mem.f32(cursor));
                let tab = e.global::<f32>(TAB_WIDTH_FLOAT) as f64;
                let rem = e.call(FMOD, &args![at as f64, tab]).f64() as f32;
                let tab_double: f64 = e.global(TAB_WIDTH_DOUBLE);
                let next = (tab_double - rem as f64) + e.mem.f32(cursor) as f64;
                e.mem.set_f32(cursor, next as f32);
            }
            let s = e.mem.u32(data_ptr.addr());
            let raw = guarded_byte(e, s, i);
            let ch = e.with_stack(4, |e, p| {
                e.mem.set_u8(p.addr(), raw);
                font_convert_character(e, p);
                e.mem.u8(p.addr())
            });
            let ch = if ch as i32 > 0x100 { 0 } else { ch };
            if ch == 1 {
                if e.mem.u32(this.addr() + 0x4c) > 0 {
                    let icons = e.mem.u32(icons_out.addr());
                    font_add_icon(e, this, icon_index, icons, Ptr::new(cursor));
                    icon_index += 1;
                }
            } else {
                let font_data = e.get(this, Font::pFontData);
                let shape = e.mem.u32(shape_out.addr());
                font_add_char(
                    e,
                    this,
                    letter(font_data, ch),
                    char_index,
                    shape,
                    Ptr::new(cursor),
                    shape_arg,
                );
                char_index += 1;
            }
            let widest = e.mem.i32(width.addr());
            let moved = e.mem.f32(cursor) as f64 - start as f64;
            let travel = ftol(e, moved);
            e.mem
                .set_i32(width.addr(), if travel > widest { travel } else { widest });
            i += 1;
        }
        let shape = e.mem.u32(shape_out.addr());
        compute_geometry_bound(e, shape);
        let icons = e.mem.u32(icons_out.addr());
        if icons != 0 {
            compute_geometry_bound(e, icons);
        }
        let button_icons = this.at(Font::ButtonIcons);
        bs_simple_array_button_icon_clear(e, button_icons, 1);
        e.call(TEXT_DATA_DESTRUCTOR, &args![data_ptr]);
    });
}

// Translated from 00a12fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::PrepText` (Xbox PDB): lays out `text` into `data` (a
/// `Font::TextData`): replaces `&name;` entities, wraps lines at
/// `data.iWidth`, cuts at `data.iLineEnd` lines, keeps lines
/// `iLineStart..iLineEnd` (by `'\n'`), and stores the text, the widest line,
/// the height (rounded), the line count and the character count. Nothing
/// happens for a null `text`.
///
/// Entities: after `&` (or `&-`, which passes flag 0 instead of 1) up to 127
/// characters name a replacement (`Interface::FindTextReplacementString`,
/// then `Interface::TestConstantForGameSettings`); a replacement ending in
/// `\` names a button icon after its first `\` (prefixed `glow_` for font
/// 7, added with `AddTextIcon`) that becomes character 1. Without a
/// replacement the `&` stays.
///
/// Wrapping: each character advances the line by its letter's width plus
/// spacing (character 1 takes the next button icon's width and spacing
/// into its letter); a tab advances to the next multiple of 75; a space or
/// `~` (a soft hyphen, not kept) marks a break. A line that gets too wide
/// breaks at the last mark (at a `~` with an inserted `-`), or is
/// hyphenated before the current character. Each finished line's width is
/// appended to `data.xLineWidths`.
// Translation incomplete: 00a12fb0 - /GS security cookie check (00ec408c) is not translated.
pub fn font_prep_text(e: &mut Engine, this: Ptr<Font>, text: u32, data: Ptr<TextData>) {
    if text == 0 {
        return;
    }
    if e.get(data, TextData::iWidth) <= 0 {
        e.set(data, TextData::iWidth, 0x7fff_ffff);
    }
    if e.get(data, TextData::iHeight) <= 0 {
        e.set(data, TextData::iHeight, 0x7fff_ffff);
    }
    if e.get(data, TextData::iLineEnd) <= 0 {
        e.set(data, TextData::iLineEnd, 0x7fff_ffff);
    }
    let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
    let font_num = e.get(this, Font::iFontNum);
    let padding = font_manager_get_line_padding(e, Ptr::new(manager), font_num);
    // Stack locals: +0 line width, +4 width at the break mark, +8 the
    // entity name (0x84), +0x8c the replacement (0x400), +0x48c the icon
    // name (0x108), +0x594 a copy of it (0x104), +0x698 four floats the
    // code clears and never reads, +0x6a8 the character.
    e.with_stack(0x6ac, |e, locals| {
        let width_at = locals.addr();
        let break_width_at = width_at + 4;
        let name = width_at + 8;
        let out = width_at + 0x8c;
        let icon_name = width_at + 0x48c;
        let icon_copy = width_at + 0x594;
        let cleared = width_at + 0x698;
        let character = width_at + 0x6a8;
        let lines_list = data.addr() + 0x20;

        let mut wrap_pos: u32 = 0;
        let mut break_full_width: i32 = 0;
        let mut widest: i32 = 0;
        let font_data = e.get(this, Font::pFontData);
        let mut height = x87_quiet(e.mem.f32(font_data + 0x850));
        let mut lines: i32 = 1;
        let mut len = e.call(STRLEN, &args![text]).u32();
        let line_end = e.get(data, TextData::iLineEnd);
        let size = len.wrapping_add(4);
        let mut source = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.call(MEMSET, &args![source, 0u32, size]);
        let size = len.wrapping_add(4);
        let mut dest = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.call(MEMSET, &args![dest, 0u32, size]);
        e.call(SPRINTF, &args![source, PERCENT_S, text]);
        let mut pos: u32 = 0;
        let mut capacity = len.wrapping_add(4);
        let mut hyphen = false;
        e.mem.set_u8(out, 0);
        let mut replaced = false;

        // Pass 1: entities.
        let mut i: u32 = 0;
        while i < len {
            if e.mem.u8(source.wrapping_add(i)) != b'&' {
                let c = e.mem.u8(source.wrapping_add(i));
                e.mem.set_u8(dest.wrapping_add(pos), c);
                pos = pos.wrapping_add(1);
                i = i.wrapping_add(1);
                continue;
            }
            let mut j: i32 = 0;
            let mut skip: i32 = 1;
            let mut flag: u8 = 1;
            if e.mem.u8(source.wrapping_add(i).wrapping_add(1)) == b'-' {
                flag = 0;
                skip = 2;
            }
            let separator = e.get(data, TextData::cLineSep);
            loop {
                let at = source.wrapping_add(i).wrapping_add(j as u32);
                if e.mem.u8(at.wrapping_add(skip as u32)) == 0 || j >= 0x7f {
                    break;
                }
                let c = e.mem.u8(at);
                if c == b';' || c == b'\n' || c == separator {
                    break;
                }
                let copied = e.mem.u8(at.wrapping_add(skip as u32));
                e.mem.set_u8(name.wrapping_add(j as u32), copied);
                j += 1;
            }
            let end = if j != 0 { j - skip } else { j };
            e.mem.set_u8(name.wrapping_add(end as u32), 0);
            let mut consumed = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
            if e.mem.u8(source.wrapping_add(i).wrapping_add(j as u32)) == b';' {
                consumed = consumed.wrapping_add(skip as u32);
            }
            let found = e
                .call(
                    FIND_TEXT_REPLACEMENT_STRING,
                    &args![name, out, 0x400u32, flag as u32],
                )
                .bool()
                || e.call(TEST_CONSTANT_FOR_GAME_SETTINGS, &args![name, out])
                    .bool();
            if found {
                let mut out_len = e.call(STRLEN, &args![out]).u32();
                let mut growth = out_len.wrapping_sub(consumed) as i32;
                if e.mem.u8(out.wrapping_add(out_len).wrapping_sub(1)) == b'\\' {
                    for k in 0..4 {
                        e.mem.set_f32(cleared + 4 * k, 0.0);
                    }
                    let mut k: u32 = 0;
                    while e.mem.u8(out.wrapping_add(k)) != b'\\' {
                        k = k.wrapping_add(1);
                    }
                    e.mem.set_u8(icon_name, 0);
                    e.call(
                        STRCPY,
                        &args![icon_name, out.wrapping_add(k).wrapping_add(1)],
                    );
                    let n = e.call(STRLEN, &args![icon_name]).u32();
                    e.mem.set_u8(icon_name.wrapping_add(n).wrapping_sub(1), 0);
                    if e.get(this, Font::iFontNum) == 7 {
                        e.call(STRCPY_S, &args![icon_copy, 0x104u32, icon_name]);
                        e.call(
                            FORMAT_S,
                            &args![icon_name, 0x104u32, GLOW_FORMAT, icon_copy],
                        );
                    }
                    font_add_text_icon(e, this, icon_name);
                    e.mem.set_u8(out.wrapping_add(k), 1);
                    e.mem.set_u8(out.wrapping_add(k).wrapping_add(1), 0);
                    out_len = e.call(STRLEN, &args![out]).u32();
                    growth = out_len.wrapping_sub(consumed) as i32;
                }
                if growth > 0 {
                    capacity = capacity.wrapping_add(growth as u32);
                    dest = e
                        .call(REALLOCATE, &args![MEMORY_MANAGER, dest, capacity])
                        .u32();
                }
                for k in 0..out_len {
                    let c = e.mem.u8(out.wrapping_add(k));
                    e.mem.set_u8(dest.wrapping_add(pos), c);
                    pos = pos.wrapping_add(1);
                }
                i = i.wrapping_sub(1).wrapping_add(consumed);
            } else {
                let c = e.mem.u8(source.wrapping_add(i));
                e.mem.set_u8(dest.wrapping_add(pos), c);
                pos = pos.wrapping_add(1);
            }
            replaced = true;
            i = i.wrapping_add(1);
        }
        e.mem.set_u8(dest.wrapping_add(pos), 0);
        if replaced {
            len = pos;
            source = e
                .call(
                    REALLOCATE,
                    &args![MEMORY_MANAGER, source, len.wrapping_add(4)],
                )
                .u32();
            e.call(STRCPY, &args![source, dest]);
        }

        // Pass 2: wrapping.
        e.mem.set_u8(dest, 0);
        pos = 0;
        let mut icon_index: u32 = 0;
        let separator = e.get(data, TextData::cLineSep);
        // FLD base line; FADD padding; FADD height.
        let next_line = |e: &Engine, height: f32| -> f32 {
            let font_data = e.mem.u32(this.addr() + 0x38);
            ((e.mem.f32(font_data) as f64 + padding as f64) + height as f64) as f32
        };
        let mut j: u32 = 0;
        while j < len && e.mem.u8(source.wrapping_add(j)) != 0 {
            let c = e.mem.u8(source.wrapping_add(j));
            if c == separator {
                e.mem.set_u8(dest.wrapping_add(pos), separator);
                pos = pos.wrapping_add(1);
                if pos >= capacity {
                    dest = e
                        .call(
                            REALLOCATE,
                            &args![MEMORY_MANAGER, dest, pos.wrapping_add(4)],
                        )
                        .u32();
                    capacity = pos.wrapping_add(4);
                }
                height = next_line(e, height);
                finish_line(e, lines_list, width_at, &mut widest);
                e.mem.set_i32(width_at, 0);
                wrap_pos = 0;
                lines = lines.wrapping_add(1);
            } else if c == b'\t' {
                let width = e.mem.i32(width_at);
                let next = (75i32.wrapping_sub(width.wrapping_rem(75))).wrapping_add(width);
                e.mem.set_i32(width_at, next);
                j = j.wrapping_add(1);
                continue;
            } else {
                e.mem.set_u8(character, c);
                font_convert_character(e, Ptr::new(character));
                let mut ch = e.mem.u8(character);
                let font_data = e.get(this, Font::pFontData);
                let current = letter(font_data, ch);
                if ch == 1 {
                    let count = e.mem.u32(this.addr() + 0x4c);
                    if icon_index < count {
                        let icons = e.mem.u32(this.addr() + 0x48);
                        let icon = icons.wrapping_add(icon_index << 5);
                        let w = x87_quiet(e.mem.f32(icon));
                        e.mem.set_f32(current + 0x24, w);
                        let s = x87_quiet(e.mem.f32(icon.wrapping_add(0x0c)));
                        e.mem.set_f32(current + 0x30, s);
                    }
                    icon_index = icon_index.wrapping_add(1);
                }
                let advance = letter_advance(e, current);
                let width = advance.wrapping_add(e.mem.i32(width_at));
                e.mem.set_i32(width_at, width);
                if ch == b' ' {
                    wrap_pos = pos;
                    let advance = letter_advance(e, current);
                    e.mem.set_i32(break_width_at, width.wrapping_sub(advance));
                    break_full_width = width;
                    hyphen = false;
                } else if ch == b'~' {
                    wrap_pos = pos;
                    e.mem.set_i32(break_width_at, width);
                    break_full_width = width;
                    hyphen = true;
                    let advance = letter_advance(e, current);
                    e.mem.set_i32(width_at, width.wrapping_sub(advance));
                }
                if e.mem.i32(width_at) > e.get(data, TextData::iWidth) {
                    if wrap_pos != 0 && !hyphen {
                        if wrap_pos == pos {
                            ch = separator;
                        } else {
                            e.mem.set_u8(dest.wrapping_add(wrap_pos), separator);
                        }
                        height = next_line(e, height);
                        finish_line(e, lines_list, break_width_at, &mut widest);
                        wrap_pos = 0;
                        lines = lines.wrapping_add(1);
                        let width = e.mem.i32(width_at).wrapping_sub(break_full_width);
                        e.mem.set_i32(width_at, width);
                    } else {
                        let before_at = if wrap_pos != 0 {
                            // Soft hyphen: "-\n" inserted at the mark.
                            hyphen = false;
                            capacity = capacity.wrapping_add(4);
                            dest = e
                                .call(REALLOCATE, &args![MEMORY_MANAGER, dest, capacity])
                                .u32();
                            e.call(
                                MEMMOVE,
                                &args![
                                    dest.wrapping_add(wrap_pos).wrapping_add(2),
                                    dest.wrapping_add(wrap_pos),
                                    pos.wrapping_sub(wrap_pos)
                                ],
                            );
                            e.mem
                                .set_u8(dest.wrapping_add(wrap_pos).wrapping_add(1), b'\n');
                            e.mem.set_u8(dest.wrapping_add(wrap_pos), b'-');
                            pos = pos.wrapping_add(2);
                            // The code reads `dest[wrap_pos - 1]` after
                            // clearing `wrap_pos`: the byte before the buffer.
                            dest.wrapping_sub(1)
                        } else {
                            // No mark: the last character moves to the next
                            // line after "-\n".
                            if pos.wrapping_add(3) >= capacity {
                                dest = e
                                    .call(
                                        REALLOCATE,
                                        &args![MEMORY_MANAGER, dest, pos.wrapping_add(6)],
                                    )
                                    .u32();
                                capacity = pos.wrapping_add(6);
                            }
                            let at = dest.wrapping_add(pos);
                            let c0 = e.mem.u8(at);
                            e.mem.set_u8(at.wrapping_add(2), c0);
                            let c1 = e.mem.u8(at.wrapping_sub(1));
                            e.mem.set_u8(at.wrapping_add(1), c1);
                            e.mem.set_u8(at, b'\n');
                            e.mem.set_u8(at.wrapping_sub(1), b'-');
                            pos = pos.wrapping_add(2);
                            dest.wrapping_add(pos).wrapping_sub(1)
                        };
                        let font_data = e.get(this, Font::pFontData);
                        let dash = font_data.wrapping_add(0xb00);
                        height = next_line(e, height);
                        let advance = letter_advance(e, dash);
                        let width = e.mem.i32(width_at).wrapping_sub(advance);
                        e.mem.set_i32(width_at, width);
                        finish_line(e, lines_list, width_at, &mut widest);
                        wrap_pos = 0;
                        lines = lines.wrapping_add(1);
                        let font_data = e.get(this, Font::pFontData);
                        let before = e.mem.i8(before_at);
                        let width = letter_advance(e, signed_letter(font_data, before));
                        e.mem.set_i32(width_at, width);
                        let font_data = e.get(this, Font::pFontData);
                        let advance = letter_advance(e, letter(font_data, ch));
                        let width = advance.wrapping_add(e.mem.i32(width_at));
                        e.mem.set_i32(width_at, width);
                    }
                }
                if ch != b'~' {
                    e.mem.set_u8(dest.wrapping_add(pos), ch);
                    pos = pos.wrapping_add(1);
                }
                if pos >= capacity {
                    dest = e
                        .call(
                            REALLOCATE,
                            &args![MEMORY_MANAGER, dest, pos.wrapping_add(4)],
                        )
                        .u32();
                    capacity = pos.wrapping_add(4);
                }
            }
            if line_end > 0 && lines > line_end && pos != 0 {
                while e.mem.u8(dest.wrapping_add(pos)) != separator {
                    pos = pos.wrapping_sub(1);
                }
                e.mem.set_u8(dest.wrapping_add(pos), 0);
                let font_data = e.get(this, Font::pFontData);
                let step = e.mem.f32(font_data) as f64 + padding as f64;
                height = (height as f64 - step) as f32;
                break;
            }
            j = j.wrapping_add(1);
        }

        let line_start = e.get(data, TextData::iLineStart) as u32;
        if e.mem.u8(dest) != 0 && line_start != 0 {
            let mut kept: u32 = 0;
            let mut line: u32 = 0;
            let mut k: u32 = 0;
            while k < pos {
                let last = e.get(data, TextData::iLineEnd) as u32;
                if line >= line_start && line < last {
                    let c = e.mem.u8(dest.wrapping_add(k));
                    e.mem.set_u8(dest.wrapping_add(kept), c);
                    kept = kept.wrapping_add(1);
                }
                if e.mem.u8(dest.wrapping_add(k)) == b'\n' {
                    line = line.wrapping_add(1);
                }
                k += 1;
            }
            e.mem.set_u8(dest.wrapping_add(kept), 0);
            pos = kept;
        }
        if e.mem.u8(dest) == 0 {
            e.mem.set_u8(dest, b' ');
            e.mem.set_u8(dest + 1, 0);
            pos = 1;
            lines = 1;
            let font_data = e.get(this, Font::pFontData);
            height = x87_quiet(e.mem.f32(font_data + 0x850));
            let space_width = e.mem.f32(font_data + 0x84c) as f64;
            let width = ftol(e, space_width);
            e.mem.set_i32(width_at, width);
        }
        finish_line(e, lines_list, width_at, &mut widest);
        e.mem.set_u8(dest.wrapping_add(pos), 0);
        e.call(BS_STRING_SET, &args![data, dest, 0u32]);
        e.set(data, TextData::iWidth, widest);
        e.set(data, TextData::iHeight, fistp(height as f64));
        e.set(data, TextData::iLineStart, 0);
        e.set(data, TextData::iLineEnd, lines);
        e.set(data, TextData::iCharCount, pos as i32);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, source]);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, dest]);
    });
}

// Translated from 00a14170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the four `float`s at `out` for the button icon named `name`: the
/// first from a constant, the third from the font height (`+0x2C`), the
/// second and fourth from a pair chosen by the name (`a`, `b`, `x`, `y`;
/// `l` or `r` followed by `s` or `t`; anything else). Font 1 then lowers the
/// second, third and fourth by 2.5, 3 and 5; font 7 raises the second by 2.5
/// and lowers the third and fourth by 10 and 5 (constants read from the
/// exe).
pub fn fn_00a14170(e: &mut Engine, this: Ptr<Font>, out: Ptr, name: u32) {
    let out = out.addr();
    let first = x87_quiet(e.global::<f32>(ICON_FIRST));
    e.mem.set_f32(out, first);
    let height = x87_quiet(e.get(this, Font::fFontHeight));
    e.mem.set_f32(out + 8, height);
    let c0 = e.mem.u8(name);
    let (second, fourth) = if matches!(c0, b'a' | b'b' | b'x' | b'y') {
        (ICON_FACE_SECOND, ICON_FACE_FOURTH)
    } else if matches!(c0, b'l' | b'r') && matches!(e.mem.u8(name.wrapping_add(1)), b's' | b't') {
        (ICON_SHOULDER_SECOND, ICON_SHOULDER_FOURTH)
    } else {
        (ICON_OTHER_SECOND, ICON_OTHER_FOURTH)
    };
    let second = x87_quiet(e.global::<f32>(second));
    e.mem.set_f32(out + 4, second);
    let fourth = x87_quiet(e.global::<f32>(fourth));
    e.mem.set_f32(out + 0x0c, fourth);
    let adjust = |e: &mut Engine, at: u32, by: u32, add: bool| {
        let value = e.mem.f32(at) as f64;
        let by: f64 = e.global(by);
        let r = if add { value + by } else { value - by };
        e.mem.set_f32(at, r as f32);
    };
    match e.get(this, Font::iFontNum) {
        1 => {
            adjust(e, out + 4, ICON_ADJUST_SECOND, false);
            adjust(e, out + 8, ICON_ADJUST_THIRD_FONT_1, false);
            adjust(e, out + 0x0c, ICON_ADJUST_FOURTH, false);
        }
        7 => {
            adjust(e, out + 4, ICON_ADJUST_SECOND, true);
            adjust(e, out + 8, ICON_ADJUST_THIRD_FONT_7, false);
            adjust(e, out + 0x0c, ICON_ADJUST_FOURTH, false);
        }
        _ => {}
    }
}

// Translated from 00a142d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::AddChar` (Xbox PDB): writes quad `index` of the string geometry
/// `shape` for `letter` (a `FontLetter`) at the pen position `pos` (three
/// `float`s): moves the pen by the leading edge, places the four corners
/// (left/right by the width, top at `pos.z` plus the top edge, bottom one
/// height lower), copies the letter's four texture coordinates, writes the
/// two triangles, and advances the pen by the width plus the spacing (no
/// spacing when the width is not above 0, NaN included). The fifth word is
/// not read.
pub fn font_add_char(
    e: &mut Engine,
    _this: Ptr<Font>,
    letter: u32,
    index: u32,
    shape: u32,
    pos: Ptr,
    _unused_5: u32,
) {
    let pos = pos.addr();
    let x = e.mem.f32(pos) as f64 + e.mem.f32(letter + 0x2c) as f64;
    e.mem.set_f32(pos, x as f32);
    let base = index.wrapping_mul(4);
    let data = e.mem.u32(shape + 0xb8);
    let vertices = e.mem.u32(data + 0x20).wrapping_add(base.wrapping_mul(0x0c));
    let width = |e: &Engine| e.mem.f32(letter + 0x24) as f64;
    let top = |e: &Engine| (e.mem.f32(letter + 0x34) as f64 + e.mem.f32(pos + 8) as f64) as f32;
    let bottom = |e: &Engine| {
        let top = e.mem.f32(letter + 0x34) as f64 + e.mem.f32(pos + 8) as f64;
        (top - e.mem.f32(letter + 0x28) as f64) as f32
    };
    let corner = |e: &Engine, right: bool, low: bool| -> [f32; 3] {
        let x = if right {
            (width(e) + e.mem.f32(pos) as f64) as f32
        } else {
            x87_quiet(e.mem.f32(pos))
        };
        let y = x87_quiet(e.mem.f32(pos + 4));
        let z = if low { bottom(e) } else { top(e) };
        [x, y, z]
    };
    let c0 = corner(e, false, false);
    store_point(e, vertices, c0);
    let c1 = corner(e, false, true);
    store_point(e, vertices.wrapping_add(0x0c), c1);
    let c2 = corner(e, true, false);
    store_point(e, vertices.wrapping_add(0x18), c2);
    let c3 = corner(e, true, true);
    store_point(e, vertices.wrapping_add(0x24), c3);

    let uv = e.mem.u32(data + 0x2c).wrapping_add(base.wrapping_mul(8));
    for (k, from) in [0x04u32, 0x08, 0x14, 0x18, 0x0c, 0x10, 0x1c, 0x20]
        .iter()
        .enumerate()
    {
        let v = x87_quiet(e.mem.f32(letter + from));
        e.mem.set_f32(uv.wrapping_add(4 * k as u32), v);
    }
    store_quad_triangles(e, data, index);

    let letter_width = e.mem.f32(letter + 0x24);
    let zero: f64 = e.global(ZERO_DOUBLE);
    // FCOMP; TEST AH,0x41: the spacing only when the width is above 0.
    let spacing = if letter_width as f64 > zero {
        x87_quiet(e.mem.f32(letter + 0x30))
    } else {
        0.0
    };
    let x = (letter_width as f64 + spacing as f64) + e.mem.f32(pos) as f64;
    e.mem.set_f32(pos, x as f32);
}

// Translated from 00a14650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::AddIcon` (Xbox PDB): writes quad `index` of the icon geometry
/// `shape` for button icon `index` (`ButtonIcon`, 0x20 bytes: width, x
/// offset, z offset, spacing, texture rectangle) at the pen position `pos`:
/// a square of the icon's width from `pos.x` plus the x offset and
/// `pos.z` plus the z offset downwards, the rectangle's corners as texture
/// coordinates, two triangles, and the pen advanced by the width plus the
/// spacing.
pub fn font_add_icon(e: &mut Engine, this: Ptr<Font>, index: u32, shape: u32, pos: Ptr) {
    let pos = pos.addr();
    let base = index.wrapping_mul(4);
    let icon = |e: &Engine| e.mem.u32(this.addr() + 0x48).wrapping_add(index << 5);
    let at = icon(e);
    let size = x87_quiet(e.mem.f32(at));
    let at = icon(e);
    let top = (e.mem.f32(pos + 8) as f64 + e.mem.f32(at + 8) as f64) as f32;
    let at = icon(e);
    let left = (e.mem.f32(pos) as f64 + e.mem.f32(at + 4) as f64) as f32;
    let data = e.mem.u32(shape + 0xb8);
    let vertices = e.mem.u32(data + 0x20).wrapping_add(base.wrapping_mul(0x0c));
    let right = (size as f64 + left as f64) as f32;
    let bottom = (top as f64 - size as f64) as f32;
    let y = x87_quiet(e.mem.f32(pos + 4));
    store_point(e, vertices, [left, y, top]);
    let y = x87_quiet(e.mem.f32(pos + 4));
    store_point(e, vertices.wrapping_add(0x0c), [left, y, bottom]);
    let y = x87_quiet(e.mem.f32(pos + 4));
    store_point(e, vertices.wrapping_add(0x18), [right, y, top]);
    let y = x87_quiet(e.mem.f32(pos + 4));
    store_point(e, vertices.wrapping_add(0x24), [right, y, bottom]);

    let data = e.mem.u32(shape + 0xb8);
    let uv = e.mem.u32(data + 0x2c).wrapping_add(base.wrapping_mul(8));
    for (k, from) in [0x10u32, 0x18, 0x10, 0x1c, 0x14, 0x18, 0x14, 0x1c]
        .iter()
        .enumerate()
    {
        let at = icon(e);
        let v = x87_quiet(e.mem.f32(at.wrapping_add(*from)));
        e.mem.set_f32(uv.wrapping_add(4 * k as u32), v);
    }
    store_quad_triangles(e, data, index);
    let at = icon(e);
    let x = (size as f64 + e.mem.f32(at + 0x0c) as f64) + e.mem.f32(pos) as f64;
    e.mem.set_f32(pos, x as f32);
}

// Translated from 00a14a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::MakeTriShape` (Xbox PDB): a new `BSScissorTriShape` for `count`
/// characters (`new_quad_shape`), its data marked (+0x38 = 0, +0x39 = 1),
/// named "Interface: Text", with consistency 0x4000, a new
/// `TileShaderProperty` whose color (+0x68, four words) is copied from
/// `color`, prepared by the shader manager when `prepare` is set, textured
/// with the font's first texture, its properties updated and the default
/// alpha property attached.
// Translation incomplete: 00a14a20 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_make_tri_shape(
    e: &mut Engine,
    this: Ptr<Font>,
    count: u32,
    color: u32,
    prepare: u8,
) -> u32 {
    let shape = new_quad_shape(e, count);
    let data = e.mem.u32(shape + 0xb8);
    e.mem.set_u8(data + 0x38, 0);
    e.mem.set_u8(data + 0x39, 1);
    set_interface_text_name(e, shape);
    let data = e.mem.u32(shape + 0xb8);
    e.call(NI_GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0x4000u32]);
    let property = attach_tile_shader(e, shape);
    for k in 0..4 {
        let v = e.mem.u32(color + 4 * k);
        e.mem.set_u32(property + 0x68 + 4 * k, v);
    }
    if prepare != 0 {
        e.call(BS_SHADER_MANAGER_PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    }
    let texture = font_texture(e, this);
    e.call(
        TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
        &args![property, texture],
    );
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![shape]);
    e.call(INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY, &args![shape]);
    shape
}

// Translated from 00a14da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::MakeIconsTriShape` (Xbox PDB): a new `BSScissorTriShape` with one
/// quad per button icon of the font, built like `MakeTriShape`'s but named
/// "Interface: TextIcons", its shader color from `011a9be0`, always
/// prepared by the shader manager, and textured with
/// `Data\Textures\<IconAtlasTextureName>` (`NiSourceTexture::Create`).
// Translation incomplete: 00a14da0 - C++ exception unwinding (FS:[0] frame) and the /GS security cookie check (00ec408c) are not translated.
pub fn font_make_icons_tri_shape(e: &mut Engine, this: Ptr<Font>) -> u32 {
    let count = e.mem.u32(this.addr() + 0x4c);
    let shape = new_quad_shape(e, count);
    let data = e.mem.u32(shape + 0xb8);
    e.mem.set_u8(data + 0x38, 0);
    e.mem.set_u8(data + 0x39, 1);
    let handle = e
        .call(NI_FIXED_STRING_CREATE, &args![INTERFACE_TEXT_ICONS_NAME])
        .u32();
    release_after_use(e, handle, |e, local| {
        e.call(NI_OBJECT_NET_SET_NAME, &args![shape, local]);
    });
    let data = e.mem.u32(shape + 0xb8);
    e.call(NI_GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0x4000u32]);
    let property = attach_tile_shader(e, shape);
    for k in 0..4 {
        let v: u32 = e.global(ICON_SHADER_COLOR + 4 * k);
        e.mem.set_u32(property + 0x68 + 4 * k, v);
    }
    e.call(BS_SHADER_MANAGER_PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    e.with_stack(0x208, |e, buffers| {
        let name = buffers.addr();
        let path = name + 0x104;
        let atlas = e.mem.u32(this.addr() + 0x3c);
        e.call(STRNCPY_S, &args![name, 0x104u32, atlas, 0x103u32]);
        e.call(FORMAT_S, &args![path, 0x104u32, TEXTURES_PATH_FORMAT, name]);
        let handle = e.call(NI_FIXED_STRING_CREATE, &args![path]).u32();
        release_after_use(e, handle, |e, local| {
            let texture = e
                .call(
                    NI_SOURCE_TEXTURE_CREATE,
                    &args![local, TEXTURE_FORMAT_PREFS, 1u32, 0u32],
                )
                .u32();
            e.call(
                TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
                &args![property, texture],
            );
        });
    });
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![shape]);
    e.call(INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY, &args![shape]);
    shape
}

// Translated from 00a152d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases one reference of the font: when the count is not 0 it drops
/// by one, and reaching 0 releases the textures and font data
/// (`00a12300`).
pub fn fn_00a152d0(e: &mut Engine, this: Ptr<Font>) {
    if e.get(this, Font::iRefCount) != 0 {
        let count = e.get(this, Font::iRefCount).wrapping_sub(1);
        e.set(this, Font::iRefCount, count);
        if e.get(this, Font::iRefCount) == 0 {
            fn_00a12300(e, this);
        }
    }
}

// Translated from 00a15320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::Load` (Xbox PDB): with the per-thread allocation word at 0xC,
/// loads an unreferenced font with a file name: reads the file
/// (`FileFinder::GetFile_ov2`) into a new 0x3928-byte `FontData`, computes
/// the font height, the tallest letter and the deepest drop over the 256
/// letters, swaps the space letter's width (+0x84C) with its spacing
/// (+0x858) and gives it the tallest height and `drop + tallest` as top
/// edge, copies the space letter's metrics to letter 0xA0 (+0x244C..) and
/// letter 0x7A's to 0x7D (+0x1C6C.. to +0x1D14..), clears letter 0's
/// metrics and texture coordinates (giving it the same height and top
/// edge), then loads every texture. Then the reference count goes up by
/// one. A missing file, more than 8 textures or a missing texture prints a
/// message and returns without counting the reference.
// Translation incomplete: 00a15320 - C++ exception unwinding (FS:[0] frame) and the /GS security cookie check (00ec408c) are not translated.
pub fn font_load(e: &mut Engine, this: Ptr<Font>) {
    let at = e.tls() + 0x2b4;
    let saved = e.mem.u32(at);
    e.mem.set_u32(at, 0x0c);
    let restore = |e: &mut Engine| {
        let at = e.tls() + 0x2b4;
        e.mem.set_u32(at, saved);
    };
    if e.get(this, Font::iRefCount) == 0 && e.get(this, Font::pFontFile) != 0 {
        let file_name = e.get(this, Font::pFontFile);
        let file = e
            .call(
                FILE_FINDER_GET_FILE,
                &args![file_name, 0u32, 0x4000u32, 2u32],
            )
            .u32();
        if file == 0 || e.mem.u8(file + 0x2c) == 0 {
            let file_name = e.get(this, Font::pFontFile);
            e.call(PRINT_MESSAGE, &args![FONT_NOT_FOUND_FORMAT, file_name]);
            delete_file(e, file);
            restore(e);
            return;
        }
        let data = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x3928u32]).u32();
        e.set(this, Font::pFontData, data);
        let size = e.vcall(file, 0x28, &args![]).u32();
        let data = e.get(this, Font::pFontData);
        file_read(e, file, data, size);
        delete_file(e, file);
        e.set(this, Font::fFontHeight, 0.0);
        let mut tallest = 0.0f32;
        e.set(this, Font::fMaxDrop, 0.0);
        load_letter_metrics(e, this, &mut tallest);

        let data = e.get(this, Font::pFontData);
        let space_width = x87_quiet(e.mem.f32(data + 0x84c));
        copy_data_float(e, data, 0x858, 0x84c);
        e.mem.set_f32(data + 0x858, x87_quiet(space_width));
        e.mem.set_f32(data + 0x850, x87_quiet(tallest));
        let top = (e.get(this, Font::fMaxDrop) as f64 + tallest as f64) as f32;
        e.mem.set_f32(data + 0x85c, top);
        for (from, to) in [
            (0x84c, 0x244c),
            (0x858, 0x2458),
            (0x850, 0x2450),
            (0x85c, 0x245c),
            (0x1c6c, 0x1d14),
            (0x1c74, 0x1d1c),
            (0x1c78, 0x1d20),
            (0x1c70, 0x1d18),
            (0x1c7c, 0x1d24),
        ] {
            copy_data_float(e, data, from, to);
        }
        e.mem.set_f32(data + 0x14c, 0.0);
        e.mem.set_f32(data + 0x158, 0.0);
        e.mem.set_f32(data + 0x150, x87_quiet(tallest));
        let top = (e.get(this, Font::fMaxDrop) as f64 + tallest as f64) as f32;
        e.mem.set_f32(data + 0x15c, top);
        for off in [0x12c, 0x134, 0x13c, 0x144, 0x130, 0x138, 0x140, 0x148] {
            e.mem.set_f32(data + off, 0.0);
        }
        let count = e.mem.i32(data + 4);
        if count > 8 {
            let file_name = e.get(this, Font::pFontFile);
            e.call(
                PRINT_MESSAGE,
                &args![TOO_MANY_TEXTURES_FORMAT, file_name, 8u32, count],
            );
            restore(e);
            return;
        }
        let mut i: i32 = 0;
        loop {
            let data = e.get(this, Font::pFontData);
            if i >= e.mem.i32(data + 4) {
                break;
            }
            if !load_font_texture(e, this, i as u32) {
                restore(e);
                return;
            }
            i += 1;
        }
    }
    let count = e.get(this, Font::iRefCount).wrapping_add(1);
    e.set(this, Font::iRefCount, count);
    restore(e);
}

// Translated from 00a15ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps only line `line` (1-based, lines end at `'\n'`) of the string
/// `text`: nothing changes when the line starts at 0 and has no end, the
/// string becomes `" "` when there is no such line, else it is set to `" "`
/// and then to the
/// line's characters (copied into a temporary block). `this` is not read.
pub fn fn_00a15ee0(e: &mut Engine, _this: u32, text: Ptr<BSStringT>, line: i32) {
    let mut current: i32 = 1;
    let mut start: u32 = u32::MAX;
    let mut end: u32 = u32::MAX;
    let len = string_length(e, text);
    let mut i: u32 = 0;
    while i < len {
        let s = e.get(text, BSStringT::pString);
        if guarded_byte(e, s, i) == 0 {
            break;
        }
        if current == line && start == u32::MAX {
            start = i;
        }
        let s = e.get(text, BSStringT::pString);
        if guarded_byte(e, s, i) == b'\n' {
            current = current.wrapping_add(1);
        }
        if current == line.wrapping_add(1) && end == u32::MAX {
            end = i;
            break;
        }
        i += 1;
    }
    if start == 0 && end == u32::MAX {
        return;
    }
    if start == u32::MAX {
        e.call(BS_STRING_SET, &args![text, SPACE_STRING, 0u32]);
        return;
    }
    if end == u32::MAX {
        end = len;
    }
    let count = end.wrapping_sub(start);
    let size = count.wrapping_add(1);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(MEMSET, &args![block, 0u32, size]);
    let mut k: u32 = 0;
    while k < count {
        let s = e.get(text, BSStringT::pString);
        let c = guarded_byte(e, s, k.wrapping_add(start));
        e.mem.set_u8(block.wrapping_add(k), c);
        k += 1;
    }
    e.mem.set_u8(block.wrapping_add(k), 0);
    e.call(BS_STRING_SET, &args![text, SPACE_STRING, 0u32]);
    e.call(BS_STRING_SET, &args![text, block, 0u32]);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a160c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::LineBreakText` (Xbox PDB): breaks `text` in place into lines no
/// wider than `*max_width` (0 means 999999) by putting `separator` at the
/// last space of a line that gets too wide, or (without a space) by
/// inserting `-` and `separator` before the current character (growing the
/// string by `"\n"` or `"\n\n"` first when it is full); newlines become
/// `separator`, tabs advance to the next multiple of 75. `*lines` gives the
/// line limit on entry (0 or less: none); at the limit the text is cut at
/// the last separator. Returns the line count and stores the widest line in
/// `*max_width` and the height (rounded letter heights) in `*lines`. 0 for
/// a null string or a font without data; the fourth word is not read.
pub fn font_line_break_text(
    e: &mut Engine,
    this: Ptr<Font>,
    text: Ptr<BSStringT>,
    max_width: Ptr,
    lines: Ptr,
    _unused_4: u32,
    separator: u8,
) -> i32 {
    if e.get(text, BSStringT::pString) == 0 || e.get(this, Font::pFontData) == 0 {
        return 0;
    }
    let base_line = |e: &mut Engine| {
        let data = e.get(this, Font::pFontData);
        let value = e.mem.f32(data) as f64;
        ftol(e, value)
    };
    let mut mark: u32 = 0;
    let mut width_at_mark: i32 = 0;
    let mut width: i32 = 0;
    let mut widest: i32 = 0;
    let data = e.get(this, Font::pFontData);
    let space_height = e.mem.f32(data + 0x850) as f64;
    let mut height = ftol(e, space_height);
    let mut count: i32 = 0;
    let len = string_length(e, text);
    let max_lines = e.mem.i32(lines.addr());
    if e.mem.i32(max_width.addr()) == 0 {
        e.mem.set_i32(max_width.addr(), 999_999);
    }
    let mut i: u32 = 0;
    while i < len {
        let at = guarded_at(e, text, i);
        if e.mem.u8(at) == 0 {
            break;
        }
        let c = e.mem.u8(at);
        if c == b'\t' {
            width = (75i32.wrapping_sub(width.wrapping_rem(75))).wrapping_add(width);
        } else if c == b'\n' {
            let at = guarded_at(e, text, i);
            e.mem.set_u8(at, separator);
            height = base_line(e).wrapping_add(height);
            widest = if widest > width { widest } else { width };
            width = 0;
            mark = 0;
            count = count.wrapping_add(1);
        } else {
            if c == b' ' {
                mark = i;
            }
            let ch = e.with_stack(4, |e, p| {
                let at = guarded_at(e, text, i);
                e.mem.set_u8(p.addr(), e.mem.u8(at));
                font_convert_character(e, p);
                e.mem.u8(p.addr())
            });
            let data = e.get(this, Font::pFontData);
            let current = letter(data, ch);
            width = letter_advance(e, current).wrapping_add(width);
            if mark == i {
                width_at_mark = width;
            }
            if width > e.mem.i32(max_width.addr()) {
                if mark != 0 {
                    let at = guarded_at(e, text, mark);
                    e.mem.set_u8(at, separator);
                    height = base_line(e).wrapping_add(height);
                    widest = if widest > width_at_mark {
                        widest
                    } else {
                        width_at_mark
                    };
                    mark = 0;
                    count = count.wrapping_add(1);
                    width = width.wrapping_sub(width_at_mark);
                } else {
                    let length = string_length(e, text);
                    let capacity = e.get(text, BSStringT::sMaxLen) as u32;
                    if length >= capacity {
                        let length = string_length(e, text);
                        let capacity = e.get(text, BSStringT::sMaxLen) as u32;
                        if length.wrapping_add(1) >= capacity {
                            e.call(BS_STRING_APPEND, &args![text, TWO_NEWLINES]);
                            let n = string_length(e, text);
                            let at = guarded_at(e, text, n.wrapping_sub(1));
                            e.mem.set_u8(at, 0);
                            let n = string_length(e, text);
                            let at = guarded_at(e, text, n.wrapping_sub(2));
                            e.mem.set_u8(at, 0);
                        } else {
                            e.call(BS_STRING_APPEND, &args![text, ONE_NEWLINE]);
                            let n = string_length(e, text);
                            let at = guarded_at(e, text, n.wrapping_sub(1));
                            e.mem.set_u8(at, 0);
                        }
                    }
                    let mut k = len.wrapping_add(2);
                    while k > i {
                        let from = guarded_at(e, text, k.wrapping_sub(2));
                        let c = e.mem.u8(from);
                        let to = guarded_at(e, text, k);
                        e.mem.set_u8(to, c);
                        k = k.wrapping_sub(1);
                    }
                    let at = guarded_at(e, text, i.wrapping_sub(1));
                    e.mem.set_u8(at, b'-');
                    let at = guarded_at(e, text, i);
                    e.mem.set_u8(at, separator);
                    height = base_line(e).wrapping_add(height);
                    let without = width.wrapping_sub(letter_advance(e, current));
                    widest = if widest > without { widest } else { without };
                    mark = 0;
                    count = count.wrapping_add(1);
                    width = 0;
                }
            }
        }
        if max_lines > 0 && count >= max_lines {
            loop {
                let at = guarded_at(e, text, i);
                if e.mem.u8(at) == separator {
                    break;
                }
                i = i.wrapping_sub(1);
            }
            let at = guarded_at(e, text, i);
            e.mem.set_u8(at, 0);
            height = height.wrapping_sub(base_line(e));
            break;
        }
        i = i.wrapping_add(1);
    }
    let first = e.get(text, BSStringT::pString);
    if e.mem.u8(first) != 0 && i >= len {
        count = count.wrapping_add(1);
    }
    widest = if widest > width { widest } else { width };
    e.mem.set_i32(max_width.addr(), widest);
    e.mem.set_i32(lines.addr(), height);
    count
}

// Translated from 00a16700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `string` to the part of `text` from its `index`-th `separator`
/// (the separator included; the whole text for 0, or when there are fewer)
/// up to the next separator, by terminating `text` there for the copy and
/// restoring the separator byte afterwards. `this` is not read.
pub fn fn_00a16700(
    e: &mut Engine,
    _this: u32,
    text: u32,
    string: Ptr<BSStringT>,
    index: i32,
    separator: u8,
) {
    let mut start = text;
    let mut count: i32 = 0;
    let mut i: u32 = 0;
    while e.mem.u8(text.wrapping_add(i)) != 0 && count != index {
        if e.mem.u8(text.wrapping_add(i)) == separator {
            start = text.wrapping_add(i);
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    while e.mem.u8(text.wrapping_add(i)) != 0 && e.mem.u8(text.wrapping_add(i)) != separator {
        i = i.wrapping_add(1);
    }
    e.mem.set_u8(text.wrapping_add(i), 0);
    e.call(BS_STRING_SET, &args![string, start, 0u32]);
    e.mem.set_u8(text.wrapping_add(i), separator);
}

// Translated from 00a167c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::Init` (Xbox PDB): creates the font manager (0x24 bytes,
/// `FontManager::FontManager`) when there is none; a failed allocation
/// leaves the pointer 0.
// Translation incomplete: 00a167c0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_init(e: &mut Engine) {
    if e.global::<u32>(FONT_MANAGER_INSTANCE) == 0 {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x24u32]).u32();
        let manager = if block != 0 {
            font_manager_font_manager(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(FONT_MANAGER_INSTANCE, manager);
    }
}

// Translated from 00a16850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the font manager (`00a16d20`, then the block is freed) and
/// clears the pointer, when there is one.
pub fn fn_00a16850(e: &mut Engine) {
    let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
    if manager != 0 {
        fn_00a16d20(e, Ptr::new(manager));
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, manager]);
        e.set_global(FONT_MANAGER_INSTANCE, 0u32);
    }
}

// Translated from 00a168b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::FontManager` (Xbox PDB): with the per-thread allocation
/// word at 0xC, creates and loads fonts 1 to 8 (`Font::Font` on a 0x54-byte
/// block, file name from `sFont_<n>_Filename`; 0 when the allocation fails),
/// clears `bUseNewFonts`, and reports a fatal error when none of the eight
/// has font data (each font's data pointer is read, null fonts included).
/// Returns `this`.
// Translation incomplete: 00a168b0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_font_manager(e: &mut Engine, this: Ptr<FontManager>) -> Ptr<FontManager> {
    with_tls_scope(e, 0x0c, |e| {
        for n in 1..=8u32 {
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x54u32]).u32();
            let font = if block != 0 {
                let setting = FONT_FILENAME_SETTINGS + 0x0c * (n - 1);
                let filename: u32 = e.global(setting + 4);
                font_font(e, Ptr::new(block), n as i32, filename, 1).addr()
            } else {
                0
            };
            e.mem.set_u32(this.addr() + 4 * (n - 1), font);
        }
        e.set(this, FontManager::bUseNewFonts, 0);
        let mut found = false;
        for i in 0..8u32 {
            let font = e.mem.u32(this.addr() + 4 * i);
            if e.mem.u32(font.wrapping_add(0x38)) != 0 {
                found = true;
                break;
            }
        }
        if !found {
            e.call(FATAL_ERROR, &args![MISSING_FONT_DATA_MESSAGE]);
        }
    });
    this
}

// Translated from 00a16d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FontManager` destructor: destroys and frees each of the eight fonts
/// (`00a121e0`) and clears its pointer.
pub fn fn_00a16d20(e: &mut Engine, this: Ptr<FontManager>) {
    for i in 0..8u32 {
        let at = this.addr() + 4 * i;
        let font = e.mem.u32(at);
        if font != 0 {
            fn_00a121e0(e, Ptr::new(font));
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, font]);
        }
        e.mem.set_u32(at, 0);
    }
}

// Translated from 00a16da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::GetCharType` (Xbox PDB): the hypertext class of `c`: 0x20
/// for the terminator, 8 for `"` and `'`, 1 for `<` and `{`, 0x10 for `=`,
/// 2 for `>` and `}`, 4 for any other character up to the space (signed,
/// so 0x80 and above too), else 0 (the exe's jump tables at `00a16e04` and
/// `00a16e1c`).
pub fn font_manager_get_char_type(e: &mut Engine, _this: Ptr<FontManager>, c: i8) -> u32 {
    let _ = e;
    match c as u8 {
        0 => 0x20,
        b'"' | b'\'' => 8,
        b'<' | b'{' => 1,
        b'=' => 0x10,
        b'>' | b'}' => 2,
        _ => {
            if c as i32 > 0x20 {
                0
            } else {
                4
            }
        }
    }
}

// Translated from 00a16ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CollectTo` (Xbox PDB): reads hypertext from `text` at
/// `*pos` into the string `result` (returned) until a character whose type
/// (`GetCharType`) is the terminator (`*type_out` = 0x20), in `stop` (taken:
/// `*pos` moves past it), or (with `accept` non-zero) not in `accept`;
/// `*char_out` gets each character read and `*type_out` the stopping type.
/// Newlines and carriage returns are skipped. With `collect` set, the other
/// characters are gathered in a 0x800-byte buffer (appended to the result
/// every 2000 characters, the buffer not cleared), and `&name;` entities
/// are replaced (`Interface::FindTextReplacementString`, flag 0): a
/// replacement starting with `\` names a button icon of font 1
/// (`AddTextIcon`) that becomes character 1, an empty one keeps the entity
/// text; an entity without `;` prints a message and is skipped.
// Translation incomplete: 00a16ea0 - C++ exception unwinding (FS:[0] frame) and the /GS security cookie check (00ec408c) are not translated.
#[allow(clippy::too_many_arguments)]
pub fn font_manager_collect_to(
    e: &mut Engine,
    this: Ptr<FontManager>,
    result: Ptr<BSStringT>,
    text: u32,
    pos: Ptr,
    stop: u32,
    accept: u32,
    type_out: Ptr,
    char_out: Ptr,
    collect: u8,
) -> Ptr<BSStringT> {
    // Stack locals: +0 the string (8 bytes), +8 the buffer (0x800), +0x808
    // the replacement (0x104).
    e.with_stack(0x90c, |e, locals| {
        let string: Ptr<BSStringT> = locals.cast();
        let buffer = locals.addr() + 8;
        let out = locals.addr() + 0x808;
        e.set(string, BSStringT::pString, 0);
        e.set(string, BSStringT::sLen, 0);
        e.set(string, BSStringT::sMaxLen, 0);
        let mut count: u32 = 0;
        e.call(MEMSET, &args![buffer, 0u32, 0x800u32]);
        let get_pos = |e: &Engine| e.mem.u32(pos.addr());
        let advance = |e: &mut Engine, by: u32| {
            let p = e.mem.u32(pos.addr()).wrapping_add(by);
            e.mem.set_u32(pos.addr(), p);
        };
        loop {
            let c = e.mem.u8(text.wrapping_add(get_pos(e)));
            e.mem.set_u8(char_out.addr(), c);
            let c = e.mem.u8(char_out.addr());
            let kind = font_manager_get_char_type(e, this, c as i8);
            if kind & 0x20 != 0 {
                e.mem.set_u32(type_out.addr(), 0x20);
                break;
            }
            if kind & stop != 0 {
                e.mem.set_u32(type_out.addr(), kind);
                advance(e, 1);
                break;
            }
            if accept != 0 && kind & accept == 0 {
                let c = e.mem.u8(text.wrapping_add(get_pos(e)));
                e.mem.set_u8(char_out.addr(), c);
                let c = e.mem.u8(char_out.addr());
                let kind = font_manager_get_char_type(e, this, c as i8);
                e.mem.set_u32(type_out.addr(), kind);
                break;
            }
            let c = e.mem.u8(char_out.addr());
            if c == b'\n' || c == b'\r' {
                advance(e, 1);
                continue;
            }
            if collect != 0 {
                let at = text.wrapping_add(get_pos(e));
                if e.mem.u8(at) == b'&' {
                    let mut n: u32 = 1;
                    loop {
                        let p = get_pos(e).wrapping_add(n);
                        let next = e.mem.u8(text.wrapping_add(p));
                        if font_manager_get_char_type(e, this, next as i8) != 0 {
                            break;
                        }
                        if e.mem.u8(text.wrapping_add(p).wrapping_sub(1)) == b';' {
                            break;
                        }
                        n = n.wrapping_add(1);
                    }
                    let p = get_pos(e).wrapping_add(n);
                    if e.mem.u8(text.wrapping_add(p).wrapping_sub(1)) != b';' {
                        e.call(
                            PRINT_MESSAGE,
                            &args![0x12u32, COLLECT_TO_NO_SEMICOLON, 0u32],
                        );
                    } else {
                        let name = e
                            .call(ALLOCATE, &args![MEMORY_MANAGER, n.wrapping_add(1)])
                            .u32();
                        let source = text.wrapping_add(get_pos(e));
                        e.call(STRNCPY_S, &args![name, n.wrapping_add(1), source, n]);
                        e.mem.set_u8(name.wrapping_add(n), 0);
                        e.mem.set_u8(out, 0);
                        e.call(
                            FIND_TEXT_REPLACEMENT_STRING,
                            &args![name, out, 0x104u32, 0u32],
                        );
                        let first = e.mem.u8(out);
                        if first == b'\\' {
                            let len = e.call(STRLEN, &args![out]).u32();
                            e.mem.set_u8(out.wrapping_add(len).wrapping_sub(1), 0);
                            let font = e.mem.u32(this.addr());
                            font_add_text_icon(e, Ptr::new(font), out + 1);
                            e.mem.set_u8(buffer.wrapping_add(count), 1);
                            count = count.wrapping_add(1);
                        } else if first != 0 {
                            let len = e.call(STRLEN, &args![out]).u32();
                            e.call(MEMCPY_S, &args![buffer.wrapping_add(count), len, out, len]);
                            count = count.wrapping_add(len);
                        } else if n != 0 {
                            e.call(MEMCPY_S, &args![buffer.wrapping_add(count), n, name, n]);
                            count = count.wrapping_add(n);
                        }
                        let p = n.wrapping_add(get_pos(e)).wrapping_sub(1);
                        e.mem.set_u32(pos.addr(), p);
                        e.call(DEALLOCATE, &args![MEMORY_MANAGER, name]);
                    }
                } else {
                    let c = e.mem.u8(at);
                    e.mem.set_u8(buffer.wrapping_add(count), c);
                    count = count.wrapping_add(1);
                    if count as i32 > 2000 {
                        e.call(BS_STRING_APPEND, &args![string, buffer]);
                        count = 0;
                    }
                }
            }
            advance(e, 1);
        }
        e.call(BS_STRING_APPEND, &args![string, buffer]);
        e.call(BS_STRING_ASSIGN, &args![result, string]);
        e.call(BS_STRING_SET, &args![string, 0u32, 0u32]);
    });
    result
}

// Translated from 00a18f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CreateText` (Xbox PDB): copies the string passed by value
/// (`text` and `text_lengths`, its two words), prepares it into a text
/// document (`PrepText` with `info`), renders the document into `out`
/// (`TextDoc::Render`) and destroys it; then empties the copy and the
/// argument string. Returns true.
// Translation incomplete: 00a18f00 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_create_text(
    e: &mut Engine,
    this: Ptr<FontManager>,
    text: u32,
    text_lengths: u32,
    info: u32,
    out: u32,
) -> bool {
    // Stack: +0 the argument string, +8 the copy.
    e.with_stack(0x10, |e, locals| {
        let argument: Ptr<BSStringT> = locals.cast();
        let copy: Ptr<BSStringT> = Ptr::new(locals.addr() + 8);
        e.mem.set_u32(argument.addr(), text);
        e.mem.set_u32(argument.addr() + 4, text_lengths);
        e.call(BS_STRING_ASSIGN, &args![copy, argument]);
        let doc = font_manager_prep_text(e, this, copy, info);
        if !doc.is_null() {
            font_manager_text_doc_render(e, doc, out, info);
            fn_00a1b990(e, doc);
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, doc]);
        }
        e.call(BS_STRING_SET, &args![copy, 0u32, 0u32]);
        e.call(BS_STRING_SET, &args![argument, 0u32, 0u32]);
    });
    true
}

// Translated from 00a18ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::TextDoc::CurrentPage` (Xbox PDB): page `iPageNum` of the
/// document, or 0 when there are no pages or not that many.
pub fn font_manager_text_doc_current_page(e: &mut Engine, this: Ptr<TextDoc>) -> u32 {
    let pages = this.at(TextDoc::xPages);
    if e.get(pages, NiTPointerList::m_uiCount) == 0 {
        return 0;
    }
    let mut remaining = e.get(this, TextDoc::iPageNum);
    let mut node = e.get(pages, NiTPointerList::m_pkHead);
    while node != 0 {
        let before = remaining;
        remaining = remaining.wrapping_sub(1);
        if before == 0 {
            break;
        }
        node = e.mem.u32(node);
    }
    if node != 0 {
        e.mem.u32(node + 8)
    } else {
        0
    }
}

// Translated from 00a19a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::TextDoc::AddChar` (Xbox PDB): adds `char_data` to the last
/// page (`00a19c00`, with `skipped`), or, when there is no page yet or
/// `new_page` is set, to a new page (0x44 bytes, `00a1bc70` with the
/// document's page size); a page it gets back is appended to the pages.
// Translation incomplete: 00a19a10 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_text_doc_add_char(
    e: &mut Engine,
    this: Ptr<TextDoc>,
    char_data: u32,
    skipped: u32,
    new_page: u8,
) {
    let pages = this.at(TextDoc::xPages);
    let page = if e.get(pages, NiTPointerList::m_uiCount) != 0 && new_page == 0 {
        let tail = e.get(pages, NiTPointerList::m_pkTail);
        let last = e.mem.u32(tail + 8);
        fn_00a19c00(e, Ptr::new(last), char_data, skipped)
    } else {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x44u32]).u32();
        if block != 0 {
            let width = e.get(this, TextDoc::iPageWidth);
            let height = e.get(this, TextDoc::iPageHeight);
            fn_00a1bc70(e, Ptr::new(block), this, char_data, width, height).addr()
        } else {
            0
        }
    };
    if page != 0 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), page);
            e.call(POINTER_LIST_ADD_TAIL, &args![this, local]);
        });
    }
}

// Translated from 00a19b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::TextPage::GetCharCountForFont` (Xbox PDB): the number of
/// characters on the page in font `font` that are not images (no file name
/// or an empty one).
pub fn font_manager_text_page_get_char_count_for_font(
    e: &mut Engine,
    this: Ptr<TextPage>,
    font: i32,
) -> i32 {
    let mut count: i32 = 0;
    let mut line_node = e.mem.u32(this.addr());
    while line_node != 0 {
        let line = e.mem.u32(line_node + 8);
        line_node = e.mem.u32(line_node);
        if line == 0 {
            continue;
        }
        let mut char_node = e.mem.u32(line);
        while char_node != 0 {
            let data = e.mem.u32(char_node + 8);
            char_node = e.mem.u32(char_node);
            if data == 0 {
                continue;
            }
            let file = e.mem.u32(data + 0x1c);
            let no_file = file == 0 || e.mem.u8(file) == 0;
            if no_file && e.mem.i32(data) == font {
                count = count.wrapping_add(1);
            }
        }
    }
    count
}

// Translated from 00a1aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::AddTextIcon` (Xbox PDB): adds a button icon for the texture named
/// `name`: its atlas name (into `IconAtlasTextureName`) and texture
/// rectangle (`Tile::GetTextureAtlasInfo`), its size from the up to three
/// characters after the last `_` before a `.` (`00a14170`), stored in a new
/// element of the button icon array.
pub fn font_add_text_icon(e: &mut Engine, this: Ptr<Font>, name: u32) {
    // Stack locals: +0 the short name (4 bytes), +4 the ButtonIcon (0x20
    // bytes; its texture rectangle at +0x14).
    e.with_stack(0x24, |e, locals| {
        let short = locals.addr();
        let icon = short + 4;
        let rect = icon + 0x10;
        for k in 0..4 {
            e.mem.set_f32(rect + 4 * k, 0.0);
        }
        let manager: u32 = e.global(TEXTURE_ATLAS_MANAGER);
        let atlas_name = this.addr() + 0x3c;
        e.call(
            TILE_GET_TEXTURE_ATLAS_INFO,
            &args![manager, name, atlas_name, rect],
        );
        let mut k = e.call(STRLEN, &args![name]).u32().wrapping_sub(1);
        while e.mem.u8(name.wrapping_add(k)) != b'_' {
            k = k.wrapping_sub(1);
        }
        let mut j: u32 = 0;
        while j < 3 {
            let c = e
                .mem
                .u8(name.wrapping_add(k).wrapping_add(j).wrapping_add(1));
            if c == b'.' {
                break;
            }
            e.mem.set_u8(short + j, c);
            j += 1;
        }
        e.mem.set_u8(short + j, 0);
        fn_00a14170(e, this, Ptr::new(icon), short);
        let array = this.addr() + 0x44;
        let index = e.call(BUTTON_ICONS_NEW_INDEX, &args![array]).u32();
        let element = e.mem.u32(array + 4).wrapping_add(index.wrapping_shl(5));
        e.call(BUTTON_ICONS_CONSTRUCT, &args![array, element, 1u32]);
        let element = e.mem.u32(array + 4).wrapping_add(index.wrapping_shl(5));
        let bytes = e.mem.bytes(icon, 0x20);
        e.mem.write(element, &bytes);
    });
}

// Translated from 00a1b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::GetLinePadding` (Xbox PDB): the extra space between lines
/// of font `font`: 4 for fonts 2 and 6, 6 for font 3, 2 for font 7, else 0
/// (constants read from the exe).
pub fn font_manager_get_line_padding(e: &mut Engine, _this: Ptr<FontManager>, font: i32) -> f32 {
    let at = match font {
        2 | 6 => PADDING_FONTS_2_6,
        3 => PADDING_FONT_3,
        7 => PADDING_FONT_7,
        _ => return 0.0,
    };
    x87_quiet(e.global::<f32>(at))
}

// Translated from 00a1b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` default constructor: clears the pointer (the reference
/// count increment that follows for a non-null pointer never runs).
pub fn fn_00a1b410(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00a1b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CharData::CharData` (Xbox PDB): font `font` clamped to
/// 0..=8, character `c`, color `(red, green, blue, alpha)` (bit patterns),
/// justification, an empty file name, then from the font's letter `c`:
/// `iWidth` = leading edge plus spacing (0 when twice the width is exactly
/// 0), `iRise` = the base line, `iDrop` = height minus top edge (each
/// `_ftol2`), leading edge and x 0.
// Translation incomplete: 00a1b450 - C++ exception unwinding (FS:[0] frame) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn font_manager_char_data_char_data(
    e: &mut Engine,
    this: Ptr<CharData>,
    font: i32,
    c: u8,
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
    justification: i32,
) -> Ptr<CharData> {
    let color = this.addr() + 8;
    for k in 0..4 {
        e.mem.set_f32(color + 4 * k, 0.0);
    }
    let name = this.at(CharData::xFilename);
    e.set(name, BSStringT::pString, 0);
    e.set(name, BSStringT::sLen, 0);
    e.set(name, BSStringT::sMaxLen, 0);
    let clamped = if font < 8 { font } else { 8 };
    e.set(
        this,
        CharData::iFontIndex,
        if clamped < 0 { 0 } else { clamped },
    );
    e.set(this, CharData::cChar, c);
    for (k, v) in [red, green, blue, alpha].iter().enumerate() {
        e.mem.set_u32(color + 4 * k as u32, *v);
    }
    e.set(this, CharData::iJustification, justification);
    e.call(BS_STRING_SET, &args![name, 0u32, 0u32]);
    let index = e.get(this, CharData::iFontIndex);
    let font_data = e.mem.u32(manager_font(e, index) + 0x38);
    let l = letter(font_data, e.get(this, CharData::cChar));
    let width = e.mem.f32(l + 0x24) as f64;
    let advance = if x87_is_zero(width + width) {
        0.0
    } else {
        e.mem.f32(l + 0x2c) as f64 + e.mem.f32(l + 0x30) as f64
    };
    let w = ftol(e, advance);
    e.set(this, CharData::iWidth, w);
    let index = e.get(this, CharData::iFontIndex);
    let font_data = e.mem.u32(manager_font(e, index) + 0x38);
    let base = e.mem.f32(font_data) as f64;
    let rise = ftol(e, base);
    e.set(this, CharData::iRise, rise);
    let drop = e.mem.f32(l + 0x28) as f64 - e.mem.f32(l + 0x34) as f64;
    let drop = ftol(e, drop);
    e.set(this, CharData::iDrop, drop);
    e.set(this, CharData::iLeadingEdge, 0);
    e.set(this, CharData::iX, 0);
    this
}

// Translated from 00a1b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CharData::Copy` (Xbox PDB): a new `CharData` (0x38 bytes)
/// constructed from this one's font, character, color and justification,
/// with its file name, width, rise, drop and x copied and leading edge 0.
// Translation incomplete: 00a1b660 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_char_data_copy(e: &mut Engine, this: Ptr<CharData>) -> Ptr<CharData> {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x38u32]).u32();
    let copy: Ptr<CharData> = if block != 0 {
        let color = this.addr() + 8;
        let words = [
            e.mem.u32(color),
            e.mem.u32(color + 4),
            e.mem.u32(color + 8),
            e.mem.u32(color + 12),
        ];
        let font = e.get(this, CharData::iFontIndex);
        let c = e.get(this, CharData::cChar);
        let justification = e.get(this, CharData::iJustification);
        font_manager_char_data_char_data(
            e,
            Ptr::new(block),
            font,
            c,
            words[0],
            words[1],
            words[2],
            words[3],
            justification,
        )
    } else {
        Ptr::NULL
    };
    e.call(
        BS_STRING_ASSIGN_FROM,
        &args![copy.addr() + 0x1c, this.addr() + 0x1c],
    );
    for f in [
        CharData::iWidth,
        CharData::iRise,
        CharData::iDrop,
        CharData::iX,
    ] {
        let v = e.get(this, f);
        e.set(copy, f, v);
    }
    e.set(copy, CharData::iLeadingEdge, 0);
    copy
}

// Translated from 00a1b770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CharData::RevertToDefault` (Xbox PDB): font 0, the default
/// color (from the exe, alpha 1), justification 1, an empty file name, and
/// the character set to a space (`SetChar`).
pub fn font_manager_char_data_revert_to_default(e: &mut Engine, this: Ptr<CharData>) {
    e.set(this, CharData::iFontIndex, 0);
    let color = [
        x87_quiet(e.global::<f32>(DEFAULT_RED)),
        x87_quiet(e.global::<f32>(DEFAULT_GREEN)),
        x87_quiet(e.global::<f32>(DEFAULT_BLUE)),
        1.0,
    ];
    for (k, v) in color.iter().enumerate() {
        e.mem.set_f32(this.addr() + 8 + 4 * k as u32, *v);
    }
    e.set(this, CharData::iJustification, 1);
    let name = this.at(CharData::xFilename);
    e.call(BS_STRING_SET, &args![name, 0u32, 0u32]);
    font_manager_char_data_set_char(e, this, b' ');
}

// Translated from 00a1b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CharData::SetChar` (Xbox PDB): sets the character; when
/// the character is text (no file name or an empty one) its metrics come
/// from the font's letter: `iWidth` = width plus (unless the width is
/// exactly 0) spacing and leading edge, `iRise` = the base line, `iDrop` =
/// minus the font's maximum drop (each `_ftol2`), leading edge 0. `iX`
/// becomes 0 in every case.
pub fn font_manager_char_data_set_char(e: &mut Engine, this: Ptr<CharData>, c: u8) {
    e.set(this, CharData::cChar, c);
    let file = e.mem.u32(this.addr() + 0x1c);
    let no_file = file == 0 || e.mem.u8(file) == 0;
    if no_file {
        let index = e.get(this, CharData::iFontIndex);
        let font_data = e.mem.u32(manager_font(e, index) + 0x38);
        let l = letter(font_data, e.get(this, CharData::cChar));
        let extra = if x87_is_zero(e.mem.f32(l + 0x24) as f64) {
            0.0
        } else {
            e.mem.f32(l + 0x30) as f64 + e.mem.f32(l + 0x2c) as f64
        };
        let width = e.mem.f32(l + 0x24) as f64 + extra;
        let width = ftol(e, width);
        e.set(this, CharData::iWidth, width);
        let index = e.get(this, CharData::iFontIndex);
        let font_data = e.mem.u32(manager_font(e, index) + 0x38);
        let base = e.mem.f32(font_data) as f64;
        let rise = ftol(e, base);
        e.set(this, CharData::iRise, rise);
        let index = e.get(this, CharData::iFontIndex);
        let font = manager_font(e, index);
        let drop = x87_quiet(e.mem.f32(font + 0x30));
        let drop = ftol(e, -(drop as f64));
        e.set(this, CharData::iDrop, drop);
        e.set(this, CharData::iLeadingEdge, 0);
    }
    e.set(this, CharData::iX, 0);
}

// Translated from 00a1b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList<FontManager::TextPage *>::NiTPointerList` (Xbox PDB):
/// an empty list (count, head and tail 0). Returns `this`.
pub fn ni_tpointer_list_text_page_ni_tpointer_list(
    e: &mut Engine,
    this: Ptr<NiTPointerList>,
) -> Ptr<NiTPointerList> {
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
    this
}

// Translated from 00a1b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CharData` destructor: empties the file name.
pub fn fn_00a1b970(e: &mut Engine, this: Ptr<CharData>) {
    let name = this.at(CharData::xFilename);
    e.call(BS_STRING_SET, &args![name, 0u32, 0u32]);
}

// Translated from 00a1b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TextDoc` destructor: deletes every page (`00a1ba50`) and destroys
/// the page list.
// Translation incomplete: 00a1b990 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1b990(e: &mut Engine, this: Ptr<TextDoc>) {
    delete_list_elements(e, this.addr(), |e, page| fn_00a1ba50(e, Ptr::new(page)));
}

// Translated from 00a1ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TextPage` destructor: deletes every line (`00a1bb10`) and destroys
/// the line list.
// Translation incomplete: 00a1ba50 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1ba50(e: &mut Engine, this: Ptr<TextPage>) {
    delete_list_elements(e, this.addr(), |e, line| fn_00a1bb10(e, Ptr::new(line)));
}

// Translated from 00a1bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TextLine` destructor: deletes every character (its file name
/// emptied) and destroys the character list.
// Translation incomplete: 00a1bb10 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1bb10(e: &mut Engine, this: Ptr<TextLine>) {
    delete_list_elements(e, this.addr(), |e, data| {
        e.call(BS_STRING_SET, &args![data + 0x1c, 0u32, 0u32]);
    });
}

// Translated from 00a1bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTPointerList` destructor wrapper (`00ae8370`).
// Translation incomplete: 00a1bbd0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1bbd0(e: &mut Engine, this: Ptr<NiTPointerList>) {
    e.call(POINTER_LIST_DESTRUCTOR, &args![this]);
}

// Translated from 00a1bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTPointerList` destructor that removes all items (`00a1c2d0`).
// Translation incomplete: 00a1bc20 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1bc20(e: &mut Engine, this: Ptr<NiTPointerList>) {
    e.call(POINTER_LIST_REMOVE_ALL, &args![this]);
}

// Translated from 00a1bc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TextPage` constructor: no lines, height 0, the page size, the
/// document, no characters per font, last font height 0x23, then the first
/// character added (`00a19c00`, nothing skipped). Returns `this`.
// Translation incomplete: 00a1bc70 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1bc70(
    e: &mut Engine,
    this: Ptr<TextPage>,
    doc: Ptr<TextDoc>,
    char_data: u32,
    page_width: i32,
    page_height: i32,
) -> Ptr<TextPage> {
    clear_pointer_list(e, this.addr());
    e.set(this, TextPage::iHeight, 0);
    e.set(this, TextPage::iPageWidth, page_width);
    e.set(this, TextPage::iPageHeight, page_height);
    e.set(this, TextPage::pDoc, doc.addr());
    for i in 0..8 {
        e.mem.set_u32(this.addr() + 0x20 + 4 * i, 0);
    }
    e.set(this, TextPage::iLastFontHeight, 0x23);
    fn_00a19c00(e, this, char_data, 0);
    this
}

// Translated from 00a1bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TextLine` constructor: no characters, width 0, the character's
/// justification, the page width, the page, spacing, rise and drop 0, the
/// skipped space, then the first character added (`00a19f70`, nothing
/// skipped). Returns `this`.
// Translation incomplete: 00a1bd40 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1bd40(
    e: &mut Engine,
    this: Ptr<TextLine>,
    page: Ptr<TextPage>,
    char_data: u32,
    skipped: i32,
    page_width: i32,
) -> Ptr<TextLine> {
    clear_pointer_list(e, this.addr());
    e.set(this, TextLine::iWidth, 0);
    let justify = e.mem.i32(char_data + 0x18);
    e.set(this, TextLine::iJustify, justify);
    e.set(this, TextLine::iPageWidth, page_width);
    e.set(this, TextLine::pPage, page.addr());
    e.set(this, TextLine::iNextSpacing, 0);
    e.set(this, TextLine::iRise, 0);
    e.set(this, TextLine::iDrop, 0);
    e.set(this, TextLine::iSkippedSpace, skipped);
    fn_00a19f70(e, this, char_data, 0);
    this
}

// Translated from 00a1be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TextPage` constructor from a finished line: height = the line's drop
/// plus rise, the page size, the document, the line appended (and its page
/// set to this one), last font height 0x23, no characters per font.
/// Returns `this`.
// Translation incomplete: 00a1be10 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a1be10(
    e: &mut Engine,
    this: Ptr<TextPage>,
    doc: Ptr<TextDoc>,
    line: Ptr<TextLine>,
    page_width: i32,
    page_height: i32,
) -> Ptr<TextPage> {
    clear_pointer_list(e, this.addr());
    let height = e
        .get(line, TextLine::iDrop)
        .wrapping_add(e.get(line, TextLine::iRise));
    e.set(this, TextPage::iHeight, height);
    e.set(this, TextPage::iPageWidth, page_width);
    e.set(this, TextPage::iPageHeight, page_height);
    e.set(this, TextPage::pDoc, doc.addr());
    let line_at = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), line.addr());
        e.call(POINTER_LIST_ADD_TAIL, &args![this, local]);
        e.mem.u32(local.addr())
    });
    e.set(this, TextPage::iLastFontHeight, 0x23);
    e.mem.set_u32(line_at + 0x2c, this.addr());
    for i in 0..8 {
        e.mem.set_u32(this.addr() + 0x20 + 4 * i, 0);
    }
    this
}

// Translated from 00a1bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<ButtonIcon, 1024>` constructor: its vtable and an
/// empty buffer. (The code that reserves an initial size is compiled for a
/// size of 0 and never allocates.) Returns `this`.
pub fn fn_00a1bef0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), BUTTON_ICON_ARRAY_VTABLE);
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    this
}

// Translated from 00a1bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ButtonIcon,1024>::~BSSimpleArray` (Xbox PDB): restores its
/// vtable and clears the array, freeing the buffer.
pub fn bs_simple_array_button_icon_destructor(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), BUTTON_ICON_ARRAY_VTABLE);
    bs_simple_array_button_icon_clear(e, this, 1);
}

// Translated from 00a1bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ButtonIcon,1024>::Clear` (Xbox PDB): with a buffer, sets
/// the size to 0; with `free` set it first frees the buffer (virtual slot
/// 8) and clears the buffer and the reserved size. (The element destructor
/// loop does nothing for `ButtonIcon`.)
pub fn bs_simple_array_button_icon_clear(e: &mut Engine, this: Ptr<BSSimpleArray>, free: u8) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        return;
    }
    if free != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.vcall(this.addr(), 8, &args![buffer]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 00a1c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ButtonIcon,1024>::_Allocate` (Xbox PDB): a block for
/// `count` 32-byte icons.
pub fn bs_simple_array_button_icon_allocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    count: u32,
) -> u32 {
    e.call(ALLOCATE, &args![MEMORY_MANAGER, count.wrapping_shl(5)])
        .u32()
}

// Translated from 00a1c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ButtonIcon,1024>::_Reallocate` (Xbox PDB): resizes
/// `block` to `count` 32-byte icons and returns the reallocation's result
/// (left in EAX).
pub fn bs_simple_array_button_icon_reallocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    block: u32,
    count: u32,
) -> u32 {
    e.call(
        REALLOCATE,
        &args![MEMORY_MANAGER, block, count.wrapping_shl(5)],
    )
    .u32()
}

// Translated from 00a1c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList::RemoveHead` (a copy in this unit): unlinks the first
/// item (the new head's previous link, or the tail when the list becomes
/// empty, cleared), frees it and returns its element.
pub fn fn_00a1c080(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let node = e.get(this, NiTPointerList::m_pkHead);
    let next = e.mem.u32(node);
    e.set(this, NiTPointerList::m_pkHead, next);
    let head = e.get(this, NiTPointerList::m_pkHead);
    if head != 0 {
        e.mem.set_u32(head + 4, 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
    }
    let element = e.mem.u32(node + 8);
    e.mem.set_u32(node + 8, 0);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, node]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

// Translated from 00a1c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ButtonIcon,1024>::`scalar deleting destructor'` (Xbox
/// PDB): destroys the array and frees it when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_simple_array_button_icon_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    bs_simple_array_button_icon_destructor(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a1b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::CalculateStringDimensions` (Xbox PDB): measures `text`
/// from character `start` in font `font` (1 to 8) wrapped at `max_width`,
/// and writes `(widest line, height, line count)` to `result` (returned).
/// Each character advances by its letter's leading edge, width and spacing
/// (a tab to the next tab stop, a `~` not at all); a space or newline marks
/// a break, a `~` marks a break after an inserted `-`. A line breaks when it
/// gets wider than `max_width` (an ordered comparison) or at a newline:
/// at the last mark, or (with no mark) before the current character with a
/// `-`; the height grows by the line padding plus the base line. For a font
/// out of range or a null text the result is the start point (`011f426c`).
pub fn font_manager_calculate_string_dimensions(
    e: &mut Engine,
    this: Ptr<FontManager>,
    result: Ptr,
    text: u32,
    font: i32,
    max_width: f32,
    start: i32,
) -> Ptr {
    let out = result.addr();
    if !(1..=8).contains(&font) || text == 0 {
        for k in 0..3 {
            let v: u32 = e.global(DIMENSIONS_START + 4 * k);
            e.mem.set_u32(out + 4 * k, v);
        }
        return result;
    }
    let start_point = |e: &Engine, k: u32| e.global::<f32>(DIMENSIONS_START + 4 * k);
    let mut widest = start_point(e, 0);
    // The start point's height and depth are copied too; both are replaced
    // below (the height by the space letter's, the depth by the line count).
    let len = e.call(STRLEN, &args![text]).i32();
    let font_ptr = |e: &Engine| {
        e.mem.u32(
            this.addr()
                .wrapping_add((font as u32).wrapping_mul(4))
                .wrapping_sub(4),
        )
    };
    let letters = e.mem.u32(font_ptr(e) + 0x38).wrapping_add(FONT_LETTERS);
    let mut break_x = 0.0f32;
    let mut x = 0.0f32;
    let padding = font_manager_get_line_padding(e, this, font);
    let mut previous = 0.0f32;
    let mut hyphen = false;
    let mut lines: i32 = 1;
    let mut height = x87_quiet(e.mem.f32(letters + 0x728));
    let mut i = start;
    while i < len {
        let c = e.with_stack(4, |e, p| {
            e.mem
                .set_u8(p.addr(), e.mem.u8(text.wrapping_add(i as u32)));
            font_convert_character(e, p);
            e.mem.u8(p.addr())
        });
        let mut advance = letter_extent(e, letters.wrapping_add(c as u32 * FONT_LETTER_SIZE));
        match c {
            b'\t' => {
                let tab = e.global::<f32>(TAB_WIDTH_FLOAT) as f64;
                let rem = e.call(FMOD, &args![x as f64, tab]).f64() as f32;
                let tab_double: f64 = e.global(TAB_WIDTH_DOUBLE);
                advance = (tab_double - rem as f64) as f32;
            }
            b'\n' | b' ' => {
                break_x = x87_quiet(x);
                hyphen = false;
            }
            b'~' => {
                break_x = add_dash_extent(e, letters, x as f64) as f32;
                hyphen = true;
            }
            _ => {}
        }
        if c != b'~' {
            x = (x as f64 + advance as f64) as f32;
        }
        // FCOMPP of max_width against x; TEST AH,0x05; JNP: only an ordered
        // "max_width below x" breaks, besides a newline.
        if (max_width as f64) < x as f64 || c == b'\n' {
            if !x87_above(break_x as f64, 0.0) {
                let without = (x as f64 - advance as f64) - previous as f64;
                break_x = add_dash_extent(e, letters, without) as f32;
                x = (advance as f64 + previous as f64) as f32;
            } else {
                x = (x as f64 - break_x as f64) as f32;
                if !hyphen {
                    if c == b'\n' {
                        x = 0.0;
                    } else {
                        let space = (e.mem.f32(letters + 0x72c) as f64
                            + e.mem.f32(letters + 0x724) as f64)
                            + e.mem.f32(letters + 0x730) as f64;
                        x = (x as f64 - space) as f32;
                    }
                }
            }
            widest = x87_keep_larger(widest, break_x);
            let base = e.mem.f32(e.mem.u32(font_ptr(e) + 0x38)) as f64;
            height = ((padding as f64 + base) + height as f64) as f32;
            break_x = 0.0;
            lines = lines.wrapping_add(1);
        }
        previous = x87_quiet(advance);
        i = i.wrapping_add(1);
    }
    widest = x87_keep_larger(widest, x);
    e.mem.set_f32(out, widest);
    e.mem.set_f32(out + 4, height);
    e.mem.set_f32(out + 8, lines as f32);
    result
}

// Translated from 00a18a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::PrepText` (Xbox PDB): lays `text` out into a new text
/// document (returned; 0 for a null string). Width and height below 2 mean
/// unlimited. Hypertext goes through `PrepHypertext`; plain text gets a
/// document whose characters (from a `CharData` reset to the defaults and
/// then given `info`'s font, color and justification) are added one by one
/// (typographic quotes converted; tabs and characters from the space up;
/// separators counted as skipped space before the next character). An
/// empty string becomes `" "`; `info.xNewText` gets the text. Then
/// `info` gets the page count, and for the current page its height, width
/// and line count.
// Translation incomplete: 00a18a30 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_prep_text(
    e: &mut Engine,
    this: Ptr<FontManager>,
    text: Ptr<BSStringT>,
    info: u32,
) -> Ptr<TextDoc> {
    let info: Ptr<ManagerTextData> = Ptr::new(info);
    if e.get(text, BSStringT::pString) == 0 {
        return Ptr::NULL;
    }
    if e.get(info, ManagerTextData::iWidth) < 2 {
        e.set(info, ManagerTextData::iWidth, 0x7fff_ffff);
    }
    if e.get(info, ManagerTextData::iHeight) < 2 {
        e.set(info, ManagerTextData::iHeight, 0x7fff_ffff);
    }
    let hyper = font_manager_prep_hypertext(e, this, text, info);
    if !hyper.is_null() {
        return hyper;
    }
    let doc = new_text_doc(e, info);
    e.with_stack(0x3c, |e, locals| {
        let data: Ptr<CharData> = locals.cast();
        let character = locals.addr() + 0x38;
        let red = x87_quiet(e.global::<f32>(DEFAULT_RED)).to_bits();
        let green = x87_quiet(e.global::<f32>(DEFAULT_GREEN)).to_bits();
        let blue = x87_quiet(e.global::<f32>(DEFAULT_BLUE)).to_bits();
        font_manager_char_data_char_data(e, data, 0, b' ', red, green, blue, 1.0f32.to_bits(), 1);
        font_manager_char_data_revert_to_default(e, data);
        let font = e.get(info, ManagerTextData::iDefaultFont);
        e.set(data, CharData::iFontIndex, font);
        for k in 0..4 {
            let v = e.mem.u32(info.addr() + 8 + 4 * k);
            e.mem.set_u32(data.addr() + 8 + 4 * k, v);
        }
        let justification = e.get(info, ManagerTextData::iJustification);
        e.set(data, CharData::iJustification, justification);
        if string_length(e, text) == 0 {
            e.call(BS_STRING_SET, &args![text, SPACE_STRING, 0u32]);
        }
        let new_text = info.at(ManagerTextData::xNewText);
        e.call(BS_STRING_ASSIGN_FROM, &args![new_text, text]);
        let mut skipped: i32 = 0;
        let mut i: u32 = 0;
        while i < string_length(e, text) {
            let at = guarded_at(e, text, i);
            e.mem.set_u8(character, e.mem.u8(at));
            font_convert_character(e, Ptr::new(character));
            let c = e.mem.u8(character);
            let separator = e.get(info, ManagerTextData::cLineSep) as i8 as i32;
            if c as i32 == separator {
                skipped = skipped.wrapping_add(1);
            } else if c == b'\t' || c >= 0x20 {
                font_manager_char_data_set_char(e, data, c);
                let copy = font_manager_char_data_copy(e, data);
                font_manager_text_doc_add_char(e, doc, copy.addr(), skipped as u32, 0);
                skipped = 0;
            }
            i += 1;
        }
        e.set(info, ManagerTextData::bIsHypertext, 0);
        e.set(info, ManagerTextData::iNumPages, 0);
        e.set(info, ManagerTextData::iWidth, 0);
        e.set(info, ManagerTextData::iHeight, 0);
        e.set(info, ManagerTextData::iNumLines, 0);
        if !doc.is_null() {
            store_page_measurements(e, info, doc);
        }
        fn_00a1b970(e, data);
    });
    doc
}

// Translated from 00a17390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::PrepHypertext` (Xbox PDB): when `text` starts (after
/// white space) with a tag, lays it out as hypertext into a new text
/// document (returned; else 0). Text runs between tags are added character
/// by character (character 1 takes the next button icon's width and
/// spacing from font 1) and collected into the result text, which replaces
/// `text`. Tags are upper-cased with their attributes (`"true"` for an
/// attribute without value, quoted values kept as written, others
/// upper-cased): `IMG` (`SRC`, `WIDTH`, `HEIGHT`) adds an image character
/// before the next tag or text, `DIV` (one skipped line per attribute;
/// `ALIGN` `LEFT`/`CENTER`/`RIGHT`), `FONT` (`FACE` by file name or number,
/// `COLOR` as six hex digits), `BR` (one skipped line), `P` (two), `HR`
/// (two, or a new page when the height is limited) and `/FONT` (back to
/// the defaults). `info` then gets the hypertext flag, the page count and
/// the current page's measurements.
// Translation incomplete: 00a17390 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_prep_hypertext(
    e: &mut Engine,
    this: Ptr<FontManager>,
    text: Ptr<BSStringT>,
    info: Ptr<ManagerTextData>,
) -> Ptr<TextDoc> {
    let len = string_length(e, text);
    let size = len.wrapping_add(1);
    let buffer = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(MEMSET, &args![buffer, 0u32, size]);
    let source = e.get(text, BSStringT::pString);
    e.call(STRCPY, &args![buffer, source]);
    e.with_stack(0x74, |e, locals| {
        let base = locals.addr();
        let string = |off: u32| -> Ptr<BSStringT> { Ptr::new(base + off) };
        let mut h = Hypertext {
            this,
            buffer,
            pos: base,
            kind: base + 4,
            character: base + 8,
            run: string(0x0c),
            tag: string(0x14),
            name: string(0x1c),
            value: string(0x24),
            collected: string(0x2c),
            temporary: string(0x34),
            data: Ptr::new(base + 0x3c),
            doc: Ptr::NULL,
            pending_image: false,
            skipped: 0,
            new_page: 0,
            icons: 0,
        };
        for s in [h.run, h.tag, h.name, h.value, h.collected] {
            e.set(s, BSStringT::pString, 0);
            e.set(s, BSStringT::sLen, 0);
            e.set(s, BSStringT::sMaxLen, 0);
        }
        let red = x87_quiet(e.global::<f32>(DEFAULT_RED)).to_bits();
        let green = x87_quiet(e.global::<f32>(DEFAULT_GREEN)).to_bits();
        let blue = x87_quiet(e.global::<f32>(DEFAULT_BLUE)).to_bits();
        font_manager_char_data_char_data(e, h.data, 0, b' ', red, green, blue, 1.0f32.to_bits(), 1);
        h.skip(e, 0, 4);
        if h.kind(e) == 1 {
            h.doc = new_text_doc(e, info);
            e.mem.set_u32(h.pos, 0);
            loop {
                h.collect(e, h.run, 1);
                if e.get(h.run, BSStringT::pString) != 0 {
                    if h.pending_image {
                        h.flush_image(e);
                    }
                    e.call(BS_STRING_APPEND_STRING, &args![h.collected, h.run]);
                    let mut k: u32 = 0;
                    while k < string_length(e, h.run) {
                        let at = guarded_at(e, h.run, k);
                        let c = e.mem.u8(at);
                        font_manager_char_data_set_char(e, h.data, c);
                        if e.get(h.data, CharData::cChar) == 1 {
                            let font = e.mem.u32(this.addr());
                            let icon = e.mem.u32(font + 0x48).wrapping_add(h.icons.wrapping_shl(5));
                            let advance =
                                e.mem.f32(icon) as f64 + e.mem.f32(icon.wrapping_add(0x0c)) as f64;
                            let width = ftol(e, advance);
                            e.set(h.data, CharData::iWidth, width);
                            h.icons = h.icons.wrapping_add(1);
                        }
                        h.add_copy(e);
                        k += 1;
                    }
                }
                if h.kind(e) & 0x20 != 0 {
                    break;
                }
                h.collect(e, h.tag, 6);
                h.upper(e, h.tag);
                if h.kind(e) & 0x20 != 0 {
                    break;
                }
                if h.kind(e) & 4 != 0 {
                    loop {
                        h.skip(e, 0, 4);
                        if h.kind(e) & 0x20 != 0 || h.kind(e) & 2 != 0 {
                            break;
                        }
                        h.collect(e, h.name, 0x16);
                        h.upper(e, h.name);
                        if h.kind(e) & 0x20 != 0 || h.kind(e) & 2 != 0 {
                            break;
                        }
                        if h.kind(e) & 4 != 0 {
                            e.call(BS_STRING_SET, &args![h.value, TRUE_TEXT, 0u32]);
                        } else {
                            h.skip(e, 0, 4);
                            if h.kind(e) & 0x20 != 0 || h.kind(e) & 2 != 0 {
                                break;
                            }
                            if h.kind(e) == 8 {
                                let p = e.mem.u32(h.pos).wrapping_add(1);
                                e.mem.set_u32(h.pos, p);
                                h.collect(e, h.value, 0x0a);
                                let p = e.mem.u32(h.pos);
                                let c = e.mem.u8(buffer.wrapping_add(p));
                                e.mem.set_u8(h.character, c);
                                e.mem.set_u32(h.pos, p.wrapping_add(1));
                                let kind = font_manager_get_char_type(e, this, c as i8);
                                e.mem.set_u32(h.kind, kind);
                            } else {
                                h.collect(e, h.value, 6);
                                h.upper(e, h.value);
                            }
                        }
                        h.apply_attribute(e);
                        if h.kind(e) & 0x20 != 0 || h.kind(e) & 2 != 0 {
                            break;
                        }
                    }
                }
                if h.equals(e, h.tag, TAG_BR) {
                    h.skipped = h.skipped.wrapping_add(1);
                } else if h.equals(e, h.tag, TAG_P) {
                    h.skipped = h.skipped.wrapping_add(2);
                } else if h.equals(e, h.tag, TAG_HR) {
                    if e.get(info, ManagerTextData::iHeight) == 0x7fff_ffff {
                        h.skipped = h.skipped.wrapping_add(2);
                    } else {
                        h.new_page = 1;
                        h.skipped = 0;
                    }
                } else if h.equals(e, h.tag, TAG_END_FONT) {
                    font_manager_char_data_revert_to_default(e, h.data);
                }
            }
            e.call(BS_STRING_ASSIGN_FROM, &args![text, h.collected]);
        }
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
        if !h.doc.is_null() {
            e.set(info, ManagerTextData::bIsHypertext, 1);
            store_page_measurements(e, info, h.doc);
        }
        let file = h.data.at(CharData::xFilename);
        e.call(BS_STRING_SET, &args![file, 0u32, 0u32]);
        for s in [h.collected, h.value, h.name, h.tag, h.run] {
            e.call(BS_STRING_SET, &args![s, 0u32, 0u32]);
        }
        h.doc
    })
}

// Translated from 00a19c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `char_data` to the page (the `TextPage` method `TextDoc::AddChar`
/// calls): to its last line (`00a19f70`), or, with no line yet or after
/// skipped lines, to a new line (0x30 bytes, `00a1bd40`) whose skipped
/// space is the last font height for each skipped line after the first. A
/// text character updates the page's last font height (base line plus
/// height below the top edge, each `_ftol2`). A line coming back is
/// appended when it fits the page height, else it starts a new page (0x44
/// bytes, `00a1be10`) that is returned; the character is counted for its
/// font on the page it ends up on. Returns the new page, else 0.
// Translation incomplete: 00a19c00 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a19c00(e: &mut Engine, this: Ptr<TextPage>, char_data: u32, skipped: u32) -> u32 {
    let page = this.addr();
    let mut skipped = skipped as i32;
    let mut space: i32 = 0;
    while skipped > 1 {
        space = space.wrapping_add(e.get(this, TextPage::iLastFontHeight));
        skipped -= 1;
    }
    let line = if e.mem.u32(page + 8) != 0 && skipped == 0 {
        let tail = e.mem.u32(page + 4);
        let last = e.mem.u32(tail + 8);
        fn_00a19f70(e, Ptr::new(last), char_data, 0)
    } else {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x30u32]).u32();
        if block != 0 {
            let width = e.get(this, TextPage::iPageWidth);
            fn_00a1bd40(e, Ptr::new(block), this, char_data, space, width).addr()
        } else {
            0
        }
    };
    if char_data != 0 && is_text_char(e, char_data) {
        let font = e.mem.i32(char_data);
        let font_data = e.mem.u32(manager_font(e, font) + 0x38);
        let l = letter(font_data, e.mem.u8(char_data + 4));
        let base = e.mem.f32(font_data) as f64;
        let rise = ftol(e, base);
        let below = e.mem.f32(l + 0x28) as f64 - e.mem.f32(l + 0x34) as f64;
        let drop = ftol(e, below);
        e.set(this, TextPage::iLastFontHeight, rise.wrapping_add(drop));
    }
    if line == 0 {
        if char_data != 0 && is_text_char(e, char_data) {
            count_font_char(e, page, char_data);
        }
        return 0;
    }
    let mut target = page;
    let line_height = e
        .get(this, TextPage::iHeight)
        .wrapping_add(e.mem.i32(line + 0x14))
        .wrapping_add(e.mem.i32(line + 0x1c));
    if line_height > e.get(this, TextPage::iPageHeight) {
        e.mem.set_i32(line + 0x1c, 0);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x44u32]).u32();
        let new_page = if block != 0 {
            let doc = e.get(this, TextPage::pDoc);
            let width = e.get(this, TextPage::iPageWidth);
            let height = e.get(this, TextPage::iPageHeight);
            fn_00a1be10(
                e,
                Ptr::new(block),
                Ptr::new(doc),
                Ptr::new(line),
                width,
                height,
            )
            .addr()
        } else {
            0
        };
        target = new_page;
        let rise = e.mem.i32(line + 0x14);
        e.mem.set_i32(target + 0x10, rise);
        e.mem.set_u32(line + 0x2c, target);
    } else {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), line);
            e.call(POINTER_LIST_ADD_TAIL, &args![page, local]);
        });
        let height = e
            .mem
            .i32(line + 0x14)
            .wrapping_add(e.mem.i32(line + 0x1c))
            .wrapping_add(e.get(this, TextPage::iHeight));
        e.set(this, TextPage::iHeight, height);
    }
    if is_text_char(e, char_data) {
        count_font_char(e, target, char_data);
    }
    if target != page {
        target
    } else {
        0
    }
}

// Translated from 00a19f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `char_data` to the line (the `TextLine` method pages call), at the
/// end or, with `at_head`, at the start (positions re-laid). A character
/// that does not fit the page width starts a new line (returned): a space
/// becomes character 0 and starts it; otherwise the words after the last
/// space move to the new line (the space itself is deleted), or without a
/// space the characters that do not fit with a `-` move and a `-` ends this
/// line. A text character joining a line of another font raises the line's
/// rise; the drop (and an image's rise) only grows. The page's width grows
/// to the line's. Returns the new line, else 0.
// Translation incomplete: 00a19f70 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_00a19f70(e: &mut Engine, this: Ptr<TextLine>, char_data: u32, at_head: u8) -> u32 {
    let list = this.addr();
    let char_width = e.mem.i32(char_data + 0x24);
    if e.get(this, TextLine::iWidth).wrapping_add(char_width) > e.get(this, TextLine::iPageWidth) {
        if e.mem.u8(char_data + 4) == b' ' {
            font_manager_char_data_set_char(e, Ptr::new(char_data), 0);
            return new_line_after(e, this, char_data);
        }
        let mut node = e.mem.u32(list + 4);
        while node != 0 && e.mem.u8(e.mem.u32(node + 8) + 4) != b' ' {
            node = e.mem.u32(node + 4);
        }
        let new_line = new_line_after(e, this, char_data);
        if node != 0 {
            let mut c = e.call(POINTER_LIST_REMOVE_TAIL, &args![list]).u32();
            while e.mem.u32(list + 8) != 0 && e.mem.u8(c + 4) != b' ' {
                fn_00a19f70(e, Ptr::new(new_line), c, 1);
                c = e.call(POINTER_LIST_REMOVE_TAIL, &args![list]).u32();
                let width = e
                    .get(this, TextLine::iWidth)
                    .wrapping_sub(e.mem.i32(c + 0x24));
                e.set(this, TextLine::iWidth, width);
            }
            if c != 0 {
                e.call(BS_STRING_SET, &args![c + 0x1c, 0u32, 0u32]);
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, c]);
            }
        } else {
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x38u32]).u32();
            let dash = if block != 0 {
                let red = x87_quiet(e.global::<f32>(DEFAULT_RED)).to_bits();
                let green = x87_quiet(e.global::<f32>(DEFAULT_GREEN)).to_bits();
                let blue = x87_quiet(e.global::<f32>(DEFAULT_BLUE)).to_bits();
                font_manager_char_data_char_data(
                    e,
                    Ptr::new(block),
                    0,
                    b'-',
                    red,
                    green,
                    blue,
                    1.0f32.to_bits(),
                    1,
                )
                .addr()
            } else {
                0
            };
            let mut moved: i32 = 0;
            while e.mem.u32(list + 8) != 0
                && e.get(this, TextLine::iWidth)
                    .wrapping_sub(moved)
                    .wrapping_add(e.mem.i32(dash + 0x24))
                    > e.get(this, TextLine::iPageWidth)
            {
                let c = e.call(POINTER_LIST_REMOVE_TAIL, &args![list]).u32();
                fn_00a19f70(e, Ptr::new(new_line), c, 1);
                moved = moved.wrapping_add(e.mem.i32(c + 0x24));
            }
            let width = e.get(this, TextLine::iWidth).wrapping_sub(moved);
            e.set(this, TextLine::iWidth, width);
            fn_00a19f70(e, this, dash, 0);
        }
        return new_line;
    }
    if is_text_char(e, char_data) {
        let mut font_change = false;
        if e.mem.u32(list + 8) == 0 {
            let font = e.mem.u32(char_data);
            let rise: i32 = e.global(FONT_LINE_RISES.wrapping_add(font.wrapping_mul(4)));
            e.set(this, TextLine::iRise, rise);
        }
        if at_head != 0 {
            let first = if e.mem.u32(list + 8) != 0 {
                e.mem.u32(e.mem.u32(list) + 8)
            } else {
                0
            };
            if first != 0 && e.mem.u32(char_data) != e.mem.u32(first) {
                font_change = true;
            }
            let lead = e.mem.i32(char_data + 0x30);
            e.mem.set_i32(char_data + 0x34, lead);
            e.set(this, TextLine::iWidth, e.mem.i32(char_data + 0x24));
            relayout_line(e, this);
        } else {
            let last = if e.mem.u32(list + 8) != 0 {
                e.mem.u32(e.mem.u32(list + 4) + 8)
            } else {
                0
            };
            if last != 0 && e.mem.u32(char_data) != e.mem.u32(last) {
                font_change = true;
            }
            let x = e
                .get(this, TextLine::iWidth)
                .wrapping_add(e.mem.i32(char_data + 0x30));
            e.mem.set_i32(char_data + 0x34, x);
            let width = e
                .get(this, TextLine::iWidth)
                .wrapping_add(e.mem.i32(char_data + 0x24));
            e.set(this, TextLine::iWidth, width);
        }
        if font_change {
            let value = e.mem.i32(char_data + 0x28);
            keep_max(e, this, TextLine::iRise, value);
        }
        let value = e.mem.i32(char_data + 0x2c);
        keep_max(e, this, TextLine::iDrop, value);
    } else {
        if at_head != 0 {
            e.mem.set_i32(char_data + 0x34, 0);
            e.set(this, TextLine::iWidth, e.mem.i32(char_data + 0x24));
            relayout_line(e, this);
        } else {
            let x = e.get(this, TextLine::iWidth);
            e.mem.set_i32(char_data + 0x34, x);
            let width = x.wrapping_add(e.mem.i32(char_data + 0x24));
            e.set(this, TextLine::iWidth, width);
        }
        let value = e.mem.i32(char_data + 0x28);
        keep_max(e, this, TextLine::iRise, value);
        let value = e.mem.i32(char_data + 0x2c);
        keep_max(e, this, TextLine::iDrop, value);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), char_data);
        let add = if at_head != 0 {
            POINTER_LIST_ADD_HEAD
        } else {
            POINTER_LIST_ADD_TAIL
        };
        e.call(add, &args![list, local]);
    });
    let page = e.get(this, TextLine::pPage);
    let line_width = e.get(this, TextLine::iWidth);
    let page_width = e.mem.i32(page + 0x0c);
    e.mem.set_i32(
        page + 0x0c,
        if page_width > line_width {
            page_width
        } else {
            line_width
        },
    );
    0
}

// Translated from 00a1a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::MakeImage` (Xbox PDB): an image quad for the file named by
/// the string passed by value (`text` and `text_lengths`): corners at
/// `(x, y, z)` to `(x + width, y, z + height)`, the path's `/` turned into
/// `\`, the texture from `TileImage::AddToTesTextures` (with its scale)
/// whose texture coordinates are the scaled size over the texture's
/// dimensions (virtual slots 0x94 and 0x98), a new `BSScissorTriShape`
/// with a `TileShaderProperty` textured with it and the shader from
/// `BSShaderManager::GetShader(0x20)` at +0xC0. The argument string is
/// emptied. Returns the shape.
// Translation incomplete: 00a1a800 - C++ exception unwinding (FS:[0] frame) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn font_manager_make_image(
    e: &mut Engine,
    _this: Ptr<FontManager>,
    text: u32,
    text_lengths: u32,
    x: f32,
    y: f32,
    z: f32,
    width: i32,
    height: i32,
) -> u32 {
    e.with_stack(0x10, |e, locals| {
        let argument: Ptr<BSStringT> = locals.cast();
        e.mem.set_u32(argument.addr(), text);
        e.mem.set_u32(argument.addr() + 4, text_lengths);
        let scale_at = locals.addr() + 8;
        let texture_at = locals.addr() + 12;
        let vertices = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x30u32]).u32();
        let uvs = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x20u32]).u32();
        let triangles = e.call(NI_ALLOC, &args![0x0cu32]).u32();
        let top = (height as f64 + z as f64) as f32;
        let right = (width as f64 + x as f64) as f32;
        store_point(e, vertices, [x, y, top]);
        store_point(e, vertices + 0x0c, [x, y, z]);
        store_point(e, vertices + 0x18, [right, y, top]);
        store_point(e, vertices + 0x24, [right, y, z]);
        for (k, v) in [0.0f32, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(uvs + 4 * k as u32, *v);
        }
        let mut i: u32 = 0;
        while i < string_length(e, argument) {
            let at = guarded_at(e, argument, i);
            if e.mem.u8(at) == b'/' {
                e.mem.set_u8(at, b'\\');
            }
            i += 1;
        }
        e.mem.set_f32(scale_at, 1.0);
        let mut texture: u32 = 0;
        let path = e.get(argument, BSStringT::pString);
        if path != 0 && e.mem.u8(path) != 0 {
            e.mem.set_u32(texture_at, 0);
            let held = e.mem.u32(texture_at);
            if held != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![held + 4]);
            }
            // The NiPointer passed by value: null, so no reference is taken.
            let path = e.get(argument, BSStringT::pString);
            e.call(
                TILE_IMAGE_ADD_TO_TES_TEXTURES,
                &args![path, scale_at, texture_at, 0.0f32, 0u32],
            );
            texture = e.mem.u32(texture_at);
            release_reference(e, texture);
        }
        if texture != 0 {
            let scale = e.mem.f32(scale_at);
            e.mem.set_f32(uvs, 0.0);
            e.mem.set_f32(uvs + 4, 0.0);
            let h = e.vcall(texture, 0x98, &args![]).u32();
            e.mem.set_f32(uvs + 8, 0.0);
            e.mem.set_f32(uvs + 0x0c, scaled_uv(scale, height, h));
            let w = e.vcall(texture, 0x94, &args![]).u32();
            e.mem.set_f32(uvs + 0x10, scaled_uv(scale, width, w));
            e.mem.set_f32(uvs + 0x14, 0.0);
            let w = e.vcall(texture, 0x94, &args![]).u32();
            let u = scaled_uv(scale, width, w);
            let h = e.vcall(texture, 0x98, &args![]).u32();
            let v = scaled_uv(scale, height, h);
            e.mem.set_f32(uvs + 0x18, u);
            e.mem.set_f32(uvs + 0x1c, v);
        }
        for (k, v) in [0u16, 1, 2, 2, 1, 3].iter().enumerate() {
            e.mem.set_u16(triangles + 2 * k as u32, *v);
        }
        let block = e.call(NI_NEW, &args![0xd4u32]).u32();
        let shape = if block != 0 {
            e.call(
                BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR,
                &args![
                    block, 4u32, vertices, 0u32, 0u32, uvs, 1u32, 0u32, 2u32, triangles, 0u32,
                    0u32, 0u32, 0u32
                ],
            )
            .u32()
        } else {
            0
        };
        let block = e.call(NI_NEW, &args![0xb0u32]).u32();
        let property = if block != 0 {
            e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
                .u32()
        } else {
            0
        };
        e.call(
            TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
            &args![property, texture],
        );
        e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![shape, property]);
        let shader = e.call(BS_SHADER_MANAGER_GET_SHADER, &args![0x20u32]).u32();
        e.mem.set_u32(shape + 0xc0, shader);
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![shape]);
        e.call(BS_STRING_SET, &args![argument, 0u32, 0u32]);
        shape
    })
}

// Translated from 00a19060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FontManager::TextDoc::Render` (Xbox PDB): renders the current page into
/// the node `out`: one string geometry per font with characters on the
/// page (`MakeTriShape`, the render color), the icon geometry of font 1
/// when it has button icons, and an image geometry per image character
/// (`MakeImage`). Lines (while `info.iLines` is above 0) go down by their
/// rise and skipped space; each starts at x 0, or for hypertext at the
/// justification offset in the page width (4 right, 2 centered). Then
/// every geometry is finished and attached, font 1's button icons are
/// cleared and the geometry list is emptied.
// Translation incomplete: 00a19060 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn font_manager_text_doc_render(e: &mut Engine, this: Ptr<TextDoc>, out: u32, info: u32) {
    let page = font_manager_text_doc_current_page(e, this);
    if page == 0 {
        return;
    }
    let info: Ptr<ManagerTextData> = Ptr::new(info);
    // Stack locals: +0 eight per-font character counters, +0x20 eight
    // per-font shapes, +0x40 the geometry list (head, tail, count), +0x4c
    // the render color, +0x5c a pen position, +0x68 a shape slot.
    e.with_stack(0x6c, |e, locals| {
        let counters = locals.addr();
        let shapes = counters + 0x20;
        let list = counters + 0x40;
        let color = counters + 0x4c;
        let pen = counters + 0x5c;
        let slot = counters + 0x68;
        let mut icon_index: u32 = 0;
        for f in 0..8u32 {
            e.mem.set_u32(counters + 4 * f, 0);
            e.mem.set_u32(shapes + 4 * f, 0);
            let count = font_manager_text_page_get_char_count_for_font(e, Ptr::new(page), f as i32);
            if count > 0 {
                e.mem.set_f32(color, x87_quiet(e.global::<f32>(RENDER_RED)));
                e.mem
                    .set_f32(color + 4, x87_quiet(e.global::<f32>(RENDER_GREEN)));
                e.mem
                    .set_f32(color + 8, x87_quiet(e.global::<f32>(RENDER_BLUE)));
                e.mem.set_f32(color + 12, 1.0);
                let font = manager_font(e, f as i32);
                let shape = font_make_tri_shape(e, Ptr::new(font), count as u32, color, 1);
                e.mem.set_u32(shapes + 4 * f, shape);
                for k in 0..3 {
                    let v: u32 = e.global(DIMENSIONS_START + 4 * k);
                    e.mem.set_u32(shape + 0x58 + 4 * k, v);
                }
            }
        }
        clear_pointer_list(e, list);
        let mut icons: u32 = 0;
        let font = manager_font(e, 0);
        if e.mem.u32(font + 0x4c) != 0 {
            let font = manager_font(e, 0);
            icons = font_make_icons_tri_shape(e, Ptr::new(font));
            for k in 0..3 {
                let v: u32 = e.global(DIMENSIONS_START + 4 * k);
                e.mem.set_u32(icons + 0x58 + 4 * k, v);
            }
            let node = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x0cu32]).u32();
            e.mem.set_u32(node + 8, icons);
            e.call(POINTER_LIST_ADD_NODE_HEAD, &args![list, node]);
        }
        let mut z: f32 = e.global(DIMENSIONS_START + 8);
        let mut line_node = e.mem.u32(page);
        while line_node != 0 && 0 < e.get(info, ManagerTextData::iLines) {
            let line = e.mem.u32(line_node + 8);
            line_node = e.mem.u32(line_node);
            let mut x = 0.0f32;
            let mut width: i32 = 0;
            if e.get(info, ManagerTextData::bIsHypertext) != 0 {
                width = e.get(this, TextDoc::iPageWidth);
            }
            let justify = e.mem.i32(line + 0x20);
            if justify == 4 {
                x = width.wrapping_sub(e.mem.i32(line + 0x0c)) as f32;
            }
            if justify == 2 {
                x = (width.wrapping_sub(e.mem.i32(line + 0x0c)) / 2) as f32;
            }
            let y = 0.0f32;
            let drop = e.mem.i32(line + 0x14).wrapping_add(e.mem.i32(line + 0x1c));
            z = (z as f64 - drop as f64) as f32;
            let mut char_node = e.mem.u32(line);
            while char_node != 0 {
                let c = e.mem.u32(char_node + 8);
                char_node = e.mem.u32(char_node);
                let index = e.mem.i32(c);
                let font = manager_font(e, index);
                let pen_x = (e.mem.i32(c + 0x34) as f64 + x as f64) as f32;
                e.mem.set_f32(pen, pen_x);
                e.mem.set_f32(pen + 4, y);
                e.mem.set_f32(pen + 8, z);
                if is_text_char(e, c) {
                    if e.mem.u8(c + 4) == 1 {
                        let first = manager_font(e, 0);
                        font_add_icon(e, Ptr::new(first), icon_index, icons, Ptr::new(pen));
                        icon_index = icon_index.wrapping_add(1);
                    } else {
                        let counter = counters.wrapping_add((index as u32).wrapping_mul(4));
                        let n = e.mem.u32(counter);
                        e.mem.set_u32(counter, n.wrapping_add(1));
                        let shape = e
                            .mem
                            .u32(shapes.wrapping_add((index as u32).wrapping_mul(4)));
                        let font_data = e.mem.u32(font + 0x38);
                        let l = letter(font_data, e.mem.u8(c + 4));
                        font_add_char(e, Ptr::new(font), l, n, shape, Ptr::new(pen), c + 8);
                    }
                } else {
                    let image = e.with_stack(8, |e, name| {
                        e.call(BS_STRING_ASSIGN, &args![name, c + 0x1c]);
                        let words = [e.mem.u32(name.addr()), e.mem.u32(name.addr() + 4)];
                        let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
                        let image_width = e.mem.i32(c + 0x24);
                        let extent = e.mem.i32(c + 0x28).wrapping_add(e.mem.i32(c + 0x2c));
                        font_manager_make_image(
                            e,
                            Ptr::new(manager),
                            words[0],
                            words[1],
                            pen_x,
                            y,
                            z,
                            image_width,
                            extent,
                        )
                    });
                    e.mem.set_u32(slot, image);
                    if image != 0 {
                        ni_tpointer_list_base_add_tail(e, Ptr::new(list), Ptr::new(slot));
                    }
                }
            }
        }
        for f in 0..8u32 {
            let shape = e.mem.u32(shapes + 4 * f);
            if shape != 0 {
                attach_rendered(e, out, shape);
            }
        }
        while e.mem.u32(list + 8) != 0 {
            let shape = fn_00a1c080(e, Ptr::new(list));
            attach_rendered(e, out, shape);
        }
        let first = manager_font(e, 0);
        bs_simple_array_button_icon_clear(e, Ptr::new(first + 0x44), 1);
        e.call(POINTER_LIST_REMOVE_ALL, &args![list]);
    });
}

/// `Setting` base vtable, stored first by the font-file setting initializers.
const SETTING_BASE_VTABLE: u32 = 0x0101_2120;
/// The setting's own vtable, stored after the name and value.
const INI_SETTING_VTABLE: u32 = 0x0101_7720;
/// The getter of the settings collection singleton (`0044f570`, no arguments).
const SETTING_COLLECTION_GETTER: u32 = 0x0044_f570;
/// The settings collection singleton pointer; its virtual at slot +4 adds a setting.
const SETTING_COLLECTION: u32 = 0x011f_96a0;
/// `_atexit` (`00ec658f`, cdecl).
const ATEXIT: u32 = 0x00ec_658f;

/// The common body of the font-file setting initializers: the base vtable at +0, the name
/// at +8, the value at +4 (in the exe's store order), the setting's vtable, then the
/// collection getter, the collection's add (slot +4) with the setting, and `_atexit` with its
/// destructor.
fn init_font_file_setting(e: &mut Engine, setting: u32, name: u32, value: u32, destructor: u32) {
    e.set_global::<u32>(setting, SETTING_BASE_VTABLE);
    e.set_global::<u32>(setting + 8, name);
    e.set_global::<u32>(setting + 4, value);
    e.set_global::<u32>(setting, INI_SETTING_VTABLE);
    e.call(SETTING_COLLECTION_GETTER, &args![]);
    let collection = e.global::<u32>(SETTING_COLLECTION);
    e.vcall(collection, 4, &args![setting]);
    e.call(ATEXIT, &args![destructor]);
}

// Translated from 00fb37f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb37f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1dfea, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_1_Filename_` (engine map): the setting at
/// `011f3414` named `"sFontFile_1:Fonts"` (`010952b0`) with the value
/// `"Data\Fonts\Glow_Monofonto_Large.fnt"` (`0109528c`), added to the settings collection;
/// its destructor `00fda3c0` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_1_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS,
        0x0109_52b0,
        0x0109_528c,
        0x00fd_a3c0,
    );
}

// Translated from 00fb3890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3890 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e01a, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_2_Filename_` (engine map): the setting at
/// `011f3420` named `"sFontFile_2:Fonts"` (`010952e4`) with the value
/// `"Data\Fonts\Monofonto_Large.fnt"` (`010952c4`), added to the settings collection;
/// its destructor `00fda440` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_2_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c,
        0x0109_52e4,
        0x0109_52c4,
        0x00fd_a440,
    );
}

// Translated from 00fb3930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3930 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e04a, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_3_Filename_` (engine map): the setting at
/// `011f342c` named `"sFontFile_3:Fonts"` (`01095320`) with the value
/// `"Data\Fonts\Glow_Monofonto_Medium.fnt"` (`010952f8`), added to the settings collection;
/// its destructor `00fda4c0` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_3_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 2,
        0x0109_5320,
        0x0109_52f8,
        0x00fd_a4c0,
    );
}

// Translated from 00fb39d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb39d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e07a, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_4_Filename_` (engine map): the setting at
/// `011f3438` named `"sFontFile_4:Fonts"` (`01095364`) with the value
/// `"Data\Fonts\Monofonto_VeryLarge02_Dialogs2.fnt"` (`01095334`), added to the settings collection;
/// its destructor `00fda540` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_4_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 3,
        0x0109_5364,
        0x0109_5334,
        0x00fd_a540,
    );
}

// Translated from 00fb3a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3a70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e0aa, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_5_Filename_` (engine map): the setting at
/// `011f3444` named `"sFontFile_5:Fonts"` (`010953a4`) with the value
/// `"Data\Fonts\Fixedsys_Comp_uniform_width.fnt"` (`01095378`), added to the settings collection;
/// its destructor `00fda5c0` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_5_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 4,
        0x0109_53a4,
        0x0109_5378,
        0x00fd_a5c0,
    );
}

// Translated from 00fb3b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3b10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e0da, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_6_Filename_` (engine map): the setting at
/// `011f3450` named `"sFontFile_6:Fonts"` (`010953e4`) with the value
/// `"Data\Fonts\Glow_Monofonto_VL_dialogs.fnt"` (`010953b8`), added to the settings collection;
/// its destructor `00fda640` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_6_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 5,
        0x0109_53e4,
        0x0109_53b8,
        0x00fd_a640,
    );
}

// Translated from 00fb3bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3bb0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e10a, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_7_Filename_` (engine map): the setting at
/// `011f345c` named `"sFontFile_7:Fonts"` (`01095420`) with the value
/// `"Data\Fonts\Baked-in_Monofonto_Large.fnt"` (`010953f8`), added to the settings collection;
/// its destructor `00fda6c0` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_7_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 6,
        0x0109_5420,
        0x0109_53f8,
        0x00fd_a6c0,
    );
}

// Translated from 00fb3c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00fb3c50 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1e13a, state variable) is not translated
/// `_dynamic_initializer_for_FontManager::sFont_8_Filename_` (engine map): the setting at
/// `011f3468` named `"sFontFile_8:Fonts"` (`0109545c`) with the value
/// `"Data\Fonts\Glow_Futura_Caps_Large.fnt"` (`01095434`), added to the settings collection;
/// its destructor `00fda740` goes to `_atexit`.
pub fn dynamic_initializer_for_font_manager_s_font_8_filename(e: &mut Engine) {
    init_font_file_setting(
        e,
        FONT_FILENAME_SETTINGS + 0x0c * 7,
        0x0109_545c,
        0x0109_5434,
        0x00fd_a740,
    );
}

/// The buffer change of the 0x20-byte item arrays (`00a1c340`, `BSMenu/run_00a1c340`,
/// `thiscall`, two words: new reserve and item count).
const ITEM_ARRAY_CHANGE_BUFFER_00A1C340: u32 = 0x00a1_c340;

// Translated from 00a1c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` append slot for 0x20-byte items (no map name; called from
/// `AddTextIcon`, `Render` and `00a1bef0`): when the array is full (`iSize` equals
/// `iReservedSize`), an empty array allocates 4 items (vtable +0x04) and records the
/// reserve, otherwise the reserve doubles (up to 0x400 items) or grows by 0x400 through
/// `00a1c340`, which then records it here. Then `iSize` grows by one; answers the index of
/// the new last item.
pub fn fn_00a1c130(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    let full = e.get(this, BSSimpleArray::iSize) == e.get(this, BSSimpleArray::iReservedSize);
    if full {
        let reserved = e.get(this, BSSimpleArray::iReservedSize);
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 0x04, &args![4u32]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, 4);
        } else {
            let grown = if reserved > 0x400 {
                reserved.wrapping_add(0x400)
            } else {
                reserved << 1
            };
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(ITEM_ARRAY_CHANGE_BUFFER_00A1C340, &args![this, grown, size]);
            e.set(this, BSSimpleArray::iReservedSize, grown);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    e.get(this, BSSimpleArray::iSize).wrapping_sub(1)
}

// Translated from 00a1c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` item construction for 0x20-byte items (no map name; `thiscall`, `this`
/// not read, called from `AddTextIcon`): for each of the `count` items from `items` (when
/// the item's address is not 0) stores 0.0 in the four floats at item `+0x10..+0x1C`.
pub fn fn_00a1c1f0(e: &mut Engine, _this: Ptr, items: u32, count: u32) {
    for i in 0..count {
        let item = (i << 5).wrapping_add(items);
        if item != 0 {
            for off in [0x10u32, 0x14, 0x18, 0x1c] {
                e.mem.set_f32(item.wrapping_add(off), 0.0);
            }
        }
    }
}

// Translated from 00a1c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTListBase::AddNodeHead` shape (no map name; `thiscall`, one word, called from
/// `AddTextIcon`): puts `node` in front of the list: its previous link cleared, its next
/// link the old head, which (or, for an empty list, the tail) then points back at it; the
/// head becomes `node` and the count grows by one.
pub fn fn_00a1c270(e: &mut Engine, this: Ptr<NiTPointerList>, node: Ptr<NiTListItem>) {
    e.set(node, NiTListItem::m_pkPrev, 0);
    let head = e.get(this, NiTPointerList::m_pkHead);
    e.set(node, NiTListItem::m_pkNext, head);
    if e.get(this, NiTPointerList::m_pkHead) != 0 {
        let head = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkHead));
        e.set(head, NiTListItem::m_pkPrev, node.addr());
    } else {
        e.set(this, NiTPointerList::m_pkTail, node.addr());
    }
    e.set(this, NiTPointerList::m_pkHead, node.addr());
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(this, NiTPointerList::m_uiCount, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1_c130, fn_00a1c130(Ptr<BSSimpleArray>) -> u32),
        entry!(0x00a1_c1f0, fn_00a1c1f0(Ptr, u32, u32)),
        entry!(
            0x00a1_c270,
            fn_00a1c270(Ptr<NiTPointerList>, Ptr<NiTListItem>)
        ),
        entry!(
            0x00fb37f0,
            dynamic_initializer_for_font_manager_s_font_1_filename()
        ),
        entry!(
            0x00fb3890,
            dynamic_initializer_for_font_manager_s_font_2_filename()
        ),
        entry!(
            0x00fb3930,
            dynamic_initializer_for_font_manager_s_font_3_filename()
        ),
        entry!(
            0x00fb39d0,
            dynamic_initializer_for_font_manager_s_font_4_filename()
        ),
        entry!(
            0x00fb3a70,
            dynamic_initializer_for_font_manager_s_font_5_filename()
        ),
        entry!(
            0x00fb3b10,
            dynamic_initializer_for_font_manager_s_font_6_filename()
        ),
        entry!(
            0x00fb3bb0,
            dynamic_initializer_for_font_manager_s_font_7_filename()
        ),
        entry!(
            0x00fb3c50,
            dynamic_initializer_for_font_manager_s_font_8_filename()
        ),
        entry!(
            0x00a11c00,
            ni_tpointer_list_base_add_tail(Ptr<NiTPointerList>, Ptr)
        ),
        entry!(0x00a12020, font_font(Ptr<Font>, i32, u32, u8) -> Ptr<Font>),
        entry!(0x00a121e0, fn_00a121e0(Ptr<Font>)),
        entry!(0x00a122b0, font_convert_character(Ptr)),
        entry!(0x00a12300, fn_00a12300(Ptr<Font>)),
        entry!(
            0x00a12370,
            font_get_line_offsets(Ptr<Font>, u32, Ptr, u32, u32, u32)
        ),
        entry!(
            0x00a12460,
            font_make_string(
                Ptr<Font>,
                f32,
                f32,
                f32,
                Ptr<BSStringT>,
                Ptr,
                u32,
                u32,
                u8,
                u8,
            ) -> u32
        ),
        entry!(
            0x00a12880,
            font_create_text(Ptr<Font>, Ptr, Ptr, Ptr, i32, i32, i32, u8, u32, Ptr, Ptr)
        ),
        entry!(0x00a12fb0, font_prep_text(Ptr<Font>, u32, Ptr<TextData>)),
        entry!(0x00a14170, fn_00a14170(Ptr<Font>, Ptr, u32)),
        entry!(
            0x00a142d0,
            font_add_char(Ptr<Font>, u32, u32, u32, Ptr, u32)
        ),
        entry!(0x00a14650, font_add_icon(Ptr<Font>, u32, u32, Ptr)),
        entry!(
            0x00a14a20,
            font_make_tri_shape(Ptr<Font>, u32, u32, u8) -> u32
        ),
        entry!(0x00a14da0, font_make_icons_tri_shape(Ptr<Font>) -> u32),
        entry!(0x00a152d0, fn_00a152d0(Ptr<Font>)),
        entry!(0x00a15320, font_load(Ptr<Font>)),
        entry!(0x00a15ee0, fn_00a15ee0(u32, Ptr<BSStringT>, i32)),
        entry!(
            0x00a160c0,
            font_line_break_text(Ptr<Font>, Ptr<BSStringT>, Ptr, Ptr, u32, u8) -> i32
        ),
        entry!(0x00a16700, fn_00a16700(u32, u32, Ptr<BSStringT>, i32, u8)),
        entry!(0x00a167c0, font_manager_init()),
        entry!(0x00a16850, fn_00a16850()),
        entry!(
            0x00a168b0,
            font_manager_font_manager(Ptr<FontManager>) -> Ptr<FontManager>
        ),
        entry!(0x00a16d20, fn_00a16d20(Ptr<FontManager>)),
        entry!(
            0x00a16da0,
            font_manager_get_char_type(Ptr<FontManager>, i8) -> u32
        ),
        entry!(
            0x00a16ea0,
            font_manager_collect_to(
                Ptr<FontManager>,
                Ptr<BSStringT>,
                u32,
                Ptr,
                u32,
                u32,
                Ptr,
                Ptr,
                u8,
            ) -> Ptr<BSStringT>
        ),
        entry!(
            0x00a17390,
            font_manager_prep_hypertext(
                Ptr<FontManager>,
                Ptr<BSStringT>,
                Ptr<ManagerTextData>,
            ) -> Ptr<TextDoc>
        ),
        entry!(
            0x00a18a30,
            font_manager_prep_text(Ptr<FontManager>, Ptr<BSStringT>, u32) -> Ptr<TextDoc>
        ),
        entry!(
            0x00a18f00,
            font_manager_create_text(Ptr<FontManager>, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00a18ff0,
            font_manager_text_doc_current_page(Ptr<TextDoc>) -> u32
        ),
        entry!(
            0x00a19060,
            font_manager_text_doc_render(Ptr<TextDoc>, u32, u32)
        ),
        entry!(
            0x00a19a10,
            font_manager_text_doc_add_char(Ptr<TextDoc>, u32, u32, u8)
        ),
        entry!(
            0x00a19b00,
            font_manager_text_page_get_char_count_for_font(Ptr<TextPage>, i32) -> i32
        ),
        entry!(0x00a19c00, fn_00a19c00(Ptr<TextPage>, u32, u32) -> u32),
        entry!(0x00a19f70, fn_00a19f70(Ptr<TextLine>, u32, u8) -> u32),
        entry!(
            0x00a1a800,
            font_manager_make_image(Ptr<FontManager>, u32, u32, f32, f32, f32, i32, i32) -> u32
        ),
        entry!(0x00a1aee0, font_add_text_icon(Ptr<Font>, u32)),
        entry!(
            0x00a1b020,
            font_manager_calculate_string_dimensions(
                Ptr<FontManager>,
                Ptr,
                u32,
                i32,
                f32,
                i32,
            ) -> Ptr
        ),
        entry!(
            0x00a1b3a0,
            font_manager_get_line_padding(Ptr<FontManager>, i32) -> f32
        ),
        entry!(0x00a1b410, fn_00a1b410(Ptr)),
        entry!(
            0x00a1b450,
            font_manager_char_data_char_data(
                Ptr<CharData>,
                i32,
                u8,
                u32,
                u32,
                u32,
                u32,
                i32,
            ) -> Ptr<CharData>
        ),
        entry!(
            0x00a1b660,
            font_manager_char_data_copy(Ptr<CharData>) -> Ptr<CharData>
        ),
        entry!(
            0x00a1b770,
            font_manager_char_data_revert_to_default(Ptr<CharData>)
        ),
        entry!(
            0x00a1b7f0,
            font_manager_char_data_set_char(Ptr<CharData>, u8)
        ),
        entry!(
            0x00a1b940,
            ni_tpointer_list_text_page_ni_tpointer_list(Ptr<NiTPointerList>) -> Ptr<NiTPointerList>
        ),
        entry!(0x00a1b970, fn_00a1b970(Ptr<CharData>)),
        entry!(0x00a1b990, fn_00a1b990(Ptr<TextDoc>)),
        entry!(0x00a1ba50, fn_00a1ba50(Ptr<TextPage>)),
        entry!(0x00a1bb10, fn_00a1bb10(Ptr<TextLine>)),
        entry!(0x00a1bbd0, fn_00a1bbd0(Ptr<NiTPointerList>)),
        entry!(0x00a1bc20, fn_00a1bc20(Ptr<NiTPointerList>)),
        entry!(
            0x00a1bc70,
            fn_00a1bc70(Ptr<TextPage>, Ptr<TextDoc>, u32, i32, i32) -> Ptr<TextPage>
        ),
        entry!(
            0x00a1bd40,
            fn_00a1bd40(Ptr<TextLine>, Ptr<TextPage>, u32, i32, i32) -> Ptr<TextLine>
        ),
        entry!(
            0x00a1be10,
            fn_00a1be10(Ptr<TextPage>, Ptr<TextDoc>, Ptr<TextLine>, i32, i32) -> Ptr<TextPage>
        ),
        entry!(
            0x00a1bef0,
            fn_00a1bef0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00a1bf80,
            bs_simple_array_button_icon_destructor(Ptr<BSSimpleArray>)
        ),
        entry!(
            0x00a1bfa0,
            bs_simple_array_button_icon_clear(Ptr<BSSimpleArray>, u8)
        ),
        entry!(
            0x00a1c020,
            bs_simple_array_button_icon_allocate(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(
            0x00a1c050,
            bs_simple_array_button_icon_reallocate(Ptr<BSSimpleArray>, u32, u32) -> u32
        ),
        entry!(0x00a1c080, fn_00a1c080(Ptr<NiTPointerList>) -> u32),
        entry!(
            0x00a1c100,
            bs_simple_array_button_icon_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ret;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn first_arg(_: &mut Engine, a: &[u32]) -> Ret {
        ret(a[0])
    }

    fn alloc_arg0(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.alloc(a[0]))
    }

    fn f64_arg(a: &[u32], i: usize) -> f64 {
        f64::from_bits(a[i] as u64 | (a[i + 1] as u64) << 32)
    }

    fn c_len(e: &Engine, s: u32) -> u32 {
        e.mem.cstr(s).len() as u32
    }

    // CRT stand-ins.
    fn strlen_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(c_len(e, a[0]))
    }
    fn memset_double(e: &mut Engine, a: &[u32]) -> Ret {
        for k in 0..a[2] {
            e.mem.set_u8(a[0] + k, a[1] as u8);
        }
        ret(a[0])
    }
    fn strcpy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = e.mem.cstr(a[1]);
        e.mem.set_cstr(a[0], &s);
        ret(a[0])
    }
    fn strcpy_s_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = e.mem.cstr(a[2]);
        e.mem.set_cstr(a[0], &s);
        ret(0)
    }
    fn strncpy_s_double(e: &mut Engine, a: &[u32]) -> Ret {
        let mut s = e.mem.cstr(a[2]);
        s.truncate(a[3] as usize);
        e.mem.set_cstr(a[0], &s);
        ret(0)
    }
    fn memmove_double(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0], &bytes);
        ret(a[0])
    }
    fn memcpy_s_double(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[2], a[3]);
        e.mem.write(a[0], &bytes);
        ret(0)
    }
    /// `sprintf`/`sprintf_s` stand-in: the format with each `%s` replaced by
    /// the next string argument.
    fn format(e: &Engine, fmt: u32, args: &[u32]) -> Vec<u8> {
        let f = e.mem.cstr(fmt);
        let mut out = Vec::new();
        let mut next = 0;
        let mut i = 0;
        while i < f.len() {
            if f[i] == b'%' && i + 1 < f.len() && f[i + 1] == b's' {
                out.extend(e.mem.cstr(args[next]));
                next += 1;
                i += 2;
            } else {
                out.push(f[i]);
                i += 1;
            }
        }
        out
    }
    fn sprintf_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = format(e, a[1], &a[2..]);
        e.mem.set_cstr(a[0], &s);
        ret(s.len() as u32)
    }
    fn format_s_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = format(e, a[2], &a[3..]);
        e.mem.set_cstr(a[0], &s);
        ret(s.len() as u32)
    }
    fn toupper_double(_: &mut Engine, a: &[u32]) -> Ret {
        ret((a[0] as u8).to_ascii_uppercase() as u32)
    }
    fn stricmp_double(e: &mut Engine, a: &[u32]) -> Ret {
        let x = e.mem.cstr(a[0]).to_ascii_uppercase();
        let y = e.mem.cstr(a[1]).to_ascii_uppercase();
        ret((x != y) as u32)
    }
    fn parse_int(s: &[u8]) -> i32 {
        std::str::from_utf8(s)
            .ok()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or(0)
    }
    fn atol_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = e.mem.cstr(a[0]);
        ret(parse_int(&s) as u32)
    }
    fn sscanf_double(e: &mut Engine, a: &[u32]) -> Ret {
        let s = e.mem.cstr(a[0]);
        e.mem.set_i32(a[2], parse_int(&s));
        ret(1)
    }
    fn fmod_double(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            st0: f64_arg(a, 0) % f64_arg(a, 2),
            ..Ret::default()
        }
    }
    fn ftol_double(_: &mut Engine, a: &[u32]) -> Ret {
        ret(f64_arg(a, 0).trunc() as i32 as u32)
    }
    fn realloc_double(e: &mut Engine, a: &[u32]) -> Ret {
        let block = e.mem.alloc(a[2]);
        if a[1] != 0 {
            let old = e.mem.block_size(a[1]).unwrap_or(0).min(a[2]);
            let bytes = e.mem.bytes(a[1], old);
            e.mem.write(block, &bytes);
            e.mem.free(a[1]);
        }
        ret(block)
    }

    // BSStringT stand-ins (the old text is not freed).
    fn string_set(e: &mut Engine, this: u32, text: u32, len: u32) {
        if text == 0 {
            e.mem.set_u32(this, 0);
            e.mem.set_u16(this + 4, 0);
            e.mem.set_u16(this + 6, 0);
            return;
        }
        let n = if len != 0 { len } else { c_len(e, text) };
        let bytes = e.mem.bytes(text, n);
        let p = e.mem.alloc(n + 1);
        e.mem.set_cstr(p, &bytes);
        e.mem.set_u32(this, p);
        e.mem.set_u16(this + 4, n as u16);
        e.mem.set_u16(this + 6, n as u16);
    }
    fn bs_set_double(e: &mut Engine, a: &[u32]) -> Ret {
        string_set(e, a[0], a[1], a[2]);
        ret(a[0])
    }
    fn bs_assign_double(e: &mut Engine, a: &[u32]) -> Ret {
        let text = e.mem.u32(a[1]);
        string_set(e, a[0], text, 0);
        ret(a[0])
    }
    fn bs_append_text(e: &mut Engine, this: u32, more: Vec<u8>) {
        let old = e.mem.u32(this);
        let mut s = if old != 0 { e.mem.cstr(old) } else { vec![] };
        s.extend(more);
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, &s);
        string_set(e, this, p, 0);
    }
    fn bs_append_double(e: &mut Engine, a: &[u32]) -> Ret {
        let more = e.mem.cstr(a[1]);
        bs_append_text(e, a[0], more);
        ret(a[0])
    }
    fn bs_append_string_double(e: &mut Engine, a: &[u32]) -> Ret {
        let p = e.mem.u32(a[1]);
        if p != 0 {
            let more = e.mem.cstr(p);
            bs_append_text(e, a[0], more);
        }
        ret(a[0])
    }
    fn bs_str_cmp_double(e: &mut Engine, a: &[u32]) -> Ret {
        let p = e.mem.u32(a[0]);
        let x = if p != 0 { e.mem.cstr(p) } else { vec![] };
        let y = e.mem.cstr(a[1]);
        ret((!x.eq_ignore_ascii_case(&y)) as u32)
    }

    // NiTPointerList stand-ins: head, tail, count; node next, previous,
    // element.
    fn link_tail(e: &mut Engine, list: u32, node: u32) {
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, tail);
        if tail != 0 {
            e.mem.set_u32(tail, node);
        } else {
            e.mem.set_u32(list, node);
        }
        e.mem.set_u32(list + 4, node);
        let n = e.mem.u32(list + 8) + 1;
        e.mem.set_u32(list + 8, n);
    }
    fn link_head(e: &mut Engine, list: u32, node: u32) {
        let head = e.mem.u32(list);
        e.mem.set_u32(node, head);
        e.mem.set_u32(node + 4, 0);
        if head != 0 {
            e.mem.set_u32(head + 4, node);
        } else {
            e.mem.set_u32(list + 4, node);
        }
        e.mem.set_u32(list, node);
        let n = e.mem.u32(list + 8) + 1;
        e.mem.set_u32(list + 8, n);
    }
    fn list_add_tail_double(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.alloc(12);
        let item = e.mem.u32(a[1]);
        e.mem.set_u32(node + 8, item);
        link_tail(e, a[0], node);
        ret(node)
    }
    fn list_add_head_double(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.alloc(12);
        let item = e.mem.u32(a[1]);
        e.mem.set_u32(node + 8, item);
        link_head(e, a[0], node);
        ret(node)
    }
    fn list_add_node_tail_double(e: &mut Engine, a: &[u32]) -> Ret {
        link_tail(e, a[0], a[1]);
        ret(0)
    }
    fn list_add_node_head_double(e: &mut Engine, a: &[u32]) -> Ret {
        link_head(e, a[0], a[1]);
        ret(0)
    }
    fn list_remove_head_double(e: &mut Engine, a: &[u32]) -> Ret {
        let list = a[0];
        let node = e.mem.u32(list);
        let next = e.mem.u32(node);
        e.mem.set_u32(list, next);
        if next != 0 {
            e.mem.set_u32(next + 4, 0);
        } else {
            e.mem.set_u32(list + 4, 0);
        }
        let n = e.mem.u32(list + 8) - 1;
        e.mem.set_u32(list + 8, n);
        let element = e.mem.u32(node + 8);
        e.mem.free(node);
        ret(element)
    }
    fn list_remove_tail_double(e: &mut Engine, a: &[u32]) -> Ret {
        let list = a[0];
        let node = e.mem.u32(list + 4);
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(list + 4, previous);
        if previous != 0 {
            e.mem.set_u32(previous, 0);
        } else {
            e.mem.set_u32(list, 0);
        }
        let n = e.mem.u32(list + 8) - 1;
        e.mem.set_u32(list + 8, n);
        let element = e.mem.u32(node + 8);
        e.mem.free(node);
        ret(element)
    }
    fn list_clear_double(e: &mut Engine, a: &[u32]) -> Ret {
        while e.mem.u32(a[0] + 8) != 0 {
            list_remove_head_double(e, a);
        }
        ret(0)
    }
    fn simple_list_add_tail_double(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[1]);
        if value != 0 {
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, value);
            } else {
                let n = e.mem.alloc(8);
                e.mem.set_u32(n, value);
                e.mem.set_u32(node + 4, n);
            }
        }
        ret(0)
    }
    fn text_data_constructor_double(e: &mut Engine, a: &[u32]) -> Ret {
        let this = a[0];
        for k in 0..0x28 {
            e.mem.set_u8(this + k, 0);
        }
        e.mem.set_u32(this + 8, a[1]);
        e.mem.set_u32(this + 0x0c, a[2]);
        e.mem.set_u32(this + 0x10, a[3]);
        e.mem.set_u32(this + 0x14, a[4]);
        e.mem.set_u8(this + 0x1c, a[5] as u8);
        ret(this)
    }

    /// The `BSScissorTriShape` stand-in: a shape whose model data (+0xB8)
    /// holds the vertex count (+0x08), vertices (+0x20), texture
    /// coordinates (+0x2C) and triangles (+0x30, returned by `GetTriList`).
    fn scissor_constructor_double(e: &mut Engine, a: &[u32]) -> Ret {
        let shape = a[0];
        let data = e.mem.alloc(0x40);
        e.mem.set_u16(data + 8, a[1] as u16);
        e.mem.set_u32(data + 0x20, a[2]);
        e.mem.set_u32(data + 0x2c, a[5]);
        e.mem.set_u32(data + 0x30, a[9]);
        e.mem.set_u32(shape + 0xb8, data);
        ret(shape)
    }
    fn tri_list_double(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 0x30))
    }

    fn map_page(e: &mut Engine, addr: u32) {
        if !e.mem.is_mapped(addr) {
            e.map(addr & !0xfff, 0x1000);
        }
    }
    fn set_f32_at(e: &mut Engine, addr: u32, v: f32) {
        map_page(e, addr);
        e.mem.set_f32(addr, v);
    }
    fn set_f64_at(e: &mut Engine, addr: u32, v: f64) {
        map_page(e, addr);
        e.mem.set_f64(addr, v);
    }
    fn set_u32_at(e: &mut Engine, addr: u32, v: u32) {
        map_page(e, addr);
        e.mem.set_u32(addr, v);
    }
    fn set_text_at(e: &mut Engine, addr: u32, s: &[u8]) {
        map_page(e, addr);
        e.mem.set_cstr(addr, s);
    }
    fn cstr(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        p
    }
    fn bs_string(e: &mut Engine, s: &[u8]) -> Ptr<BSStringT> {
        let p: Ptr<BSStringT> = e.new_object();
        let text = cstr(e, s);
        e.set(p, BSStringT::pString, text);
        e.set(p, BSStringT::sLen, s.len() as u16);
        e.set(p, BSStringT::sMaxLen, s.len() as u16);
        p
    }
    fn string_text(e: &Engine, p: Ptr<BSStringT>) -> Vec<u8> {
        let s = e.get(p, BSStringT::pString);
        if s == 0 {
            vec![]
        } else {
            e.mem.cstr(s)
        }
    }
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with the CRT, string, list and allocation callees doubled,
    /// the globals of this unit mapped, and the constants the code reads.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let doubles: [(u32, AbiFn); 38] = [
            (STRLEN, strlen_double),
            (MEMSET, memset_double),
            (STRCPY, strcpy_double),
            (STRCPY_S, strcpy_s_double),
            (STRNCPY_S, strncpy_s_double),
            (MEMMOVE, memmove_double),
            (MEMCPY_S, memcpy_s_double),
            (SPRINTF, sprintf_double),
            (FORMAT_S, format_s_double),
            (TOUPPER, toupper_double),
            (STRICMP, stricmp_double),
            (ATOL, atol_double),
            (SSCANF, sscanf_double),
            (FMOD, fmod_double),
            (FTOL, ftol_double),
            (REALLOCATE, realloc_double),
            (NI_ALLOC, alloc_arg0),
            (NI_NEW, alloc_arg0),
            (BS_STRING_SET, bs_set_double),
            (BS_STRING_ASSIGN, bs_assign_double),
            (BS_STRING_ASSIGN_FROM, bs_assign_double),
            (BS_STRING_APPEND, bs_append_double),
            (BS_STRING_APPEND_STRING, bs_append_string_double),
            (BS_STRING_STR_CMP, bs_str_cmp_double),
            (POINTER_LIST_ADD_TAIL, list_add_tail_double),
            (POINTER_LIST_ADD_HEAD, list_add_head_double),
            (NI_TLIST_ADD_NODE_TAIL, list_add_node_tail_double),
            (POINTER_LIST_ADD_NODE_HEAD, list_add_node_head_double),
            (POINTER_LIST_REMOVE_HEAD, list_remove_head_double),
            (POINTER_LIST_REMOVE_TAIL, list_remove_tail_double),
            (POINTER_LIST_DESTRUCTOR, list_clear_double),
            (POINTER_LIST_REMOVE_ALL, list_clear_double),
            (SIMPLE_LIST_ADD_TAIL, simple_list_add_tail_double),
            (TEXT_DATA_CONSTRUCTOR, text_data_constructor_double),
            (TEXT_DATA_DESTRUCTOR, nothing),
            (INTERLOCKED_DECREMENT, nothing),
            (INTERLOCKED_INCREMENT, nothing),
            (PRINT_MESSAGE, nothing),
        ];
        for (addr, f) in doubles {
            e.register(addr, f);
        }
        e.map(0x011f_3000, 0x2000);
        set_f64_at(&mut e, ZERO_DOUBLE, 0.0);
        set_f32_at(&mut e, FLOAT_MAX, f32::MAX);
        set_f64_at(&mut e, TWO_DOUBLE, 2.0);
        set_f32_at(&mut e, TAB_WIDTH_FLOAT, 75.0);
        set_f64_at(&mut e, TAB_WIDTH_DOUBLE, 75.0);
        set_text_at(&mut e, SPACE_STRING, b" ");
        set_text_at(&mut e, EMPTY_TEXT, b"");
        set_text_at(&mut e, PERCENT_S, b"%s");
        set_u32_at(&mut e, NI_FIXED_STRING_EMPTY, 0);
        e.call_log = Some(vec![]);
        e
    }

    /// The geometry callees of `MakeTriShape`, `MakeIconsTriShape` and
    /// `MakeImage` doubled.
    fn geometry(e: &mut Engine) {
        let doubles: [(u32, AbiFn); 14] = [
            (BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR, scissor_constructor_double),
            (NI_TRI_SHAPE_DATA_GET_TRI_LIST, tri_list_double),
            (NI_FIXED_STRING_CREATE, |_, _| ret(0x0000_1234)),
            (NI_OBJECT_NET_SET_NAME, nothing),
            (NI_GEOMETRY_DATA_SET_CONSISTENCY, nothing),
            (TILE_SHADER_PROPERTY_CONSTRUCTOR, first_arg),
            (NI_AV_OBJECT_ATTACH_PROPERTY, nothing),
            (BS_SHADER_MANAGER_PREPARE_OBJECT, nothing),
            (TILE_SHADER_PROPERTY_SET_TILE_TEXTURE, nothing),
            (NI_AV_OBJECT_UPDATE_PROPERTIES, nothing),
            (INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY, nothing),
            (NI_BOUND_COMPUTE_FROM_DATA, nothing),
            (NI_SOURCE_TEXTURE_CREATE, |_, _| ret(0x0000_5678)),
            (NI_AV_OBJECT_UPDATE, nothing),
        ];
        for (addr, f) in doubles {
            e.register(addr, f);
        }
        map_page(e, ICON_SHADER_COLOR);
    }

    /// Sets letter `c`'s width, height, leading edge, spacing and top edge.
    fn set_letter(e: &mut Engine, data: u32, c: u8, metrics: [f32; 5]) {
        let l = letter(data, c);
        for (off, v) in [0x24u32, 0x28, 0x2c, 0x30, 0x34].iter().zip(metrics) {
            e.mem.set_f32(l + off, v);
        }
    }

    /// A font with font data (base line 10, every printable letter 10 wide
    /// and 12 high, top edge 2, no leading edge or spacing) and a first
    /// texture property whose first map holds texture 0x9000.
    fn font(e: &mut Engine) -> Ptr<Font> {
        let font: Ptr<Font> = e.new_object();
        let data = e.mem.alloc(0x3928);
        e.mem.set_f32(data, 10.0);
        for c in 0x20..0x80u8 {
            set_letter(e, data, c, [10.0, 12.0, 0.0, 0.0, 2.0]);
        }
        e.set(font, Font::pFontData, data);
        e.set(font, Font::iFontNum, 1);
        let property = e.mem.alloc(0x30);
        let maps = e.mem.alloc(4);
        let map = e.mem.alloc(0x0c);
        e.mem.set_u32(map + 8, 0x9000);
        e.mem.set_u32(maps, map);
        e.mem.set_u32(property + 0x20, maps);
        e.mem.set_u32(font.addr() + 0x0c, property);
        font
    }

    /// A font manager (the global) whose eight fonts are `font`.
    fn manager(e: &mut Engine, font: Ptr<Font>) -> Ptr<FontManager> {
        let manager: Ptr<FontManager> = e.new_object();
        for i in 0..8 {
            e.mem.set_u32(manager.addr() + 4 * i, font.addr());
        }
        e.set_global(FONT_MANAGER_INSTANCE, manager.addr());
        manager
    }

    /// The tag names and default colors the text manager compares and
    /// reads, and the per-font line rises (9 each).
    fn manager_constants(e: &mut Engine) {
        for (addr, s) in [
            (TAG_IMG, &b"IMG"[..]),
            (ATTR_SRC, b"SRC"),
            (ATTR_WIDTH, b"WIDTH"),
            (ATTR_HEIGHT, b"HEIGHT"),
            (TAG_DIV, b"DIV"),
            (ATTR_ALIGN, b"ALIGN"),
            (VALUE_LEFT, b"LEFT"),
            (VALUE_CENTER, b"CENTER"),
            (VALUE_RIGHT, b"RIGHT"),
            (TAG_FONT, b"FONT"),
            (ATTR_FACE, b"FACE"),
            (ATTR_COLOR, b"COLOR"),
            (TAG_BR, b"BR"),
            (TAG_P, b"P"),
            (TAG_HR, b"HR"),
            (TAG_END_FONT, b"/FONT"),
            (TRUE_TEXT, b"true"),
            (PERCENT_I, b"%i"),
        ] {
            set_text_at(e, addr, s);
        }
        for (addr, v) in [
            (DEFAULT_RED, 0.5f32),
            (DEFAULT_GREEN, 0.25),
            (DEFAULT_BLUE, 0.125),
            (RENDER_RED, 0.75),
            (RENDER_GREEN, 0.5),
            (RENDER_BLUE, 0.25),
        ] {
            set_f32_at(e, addr, v);
        }
        set_f64_at(e, COLOR_SCALE, 255.0);
        for i in 0..9 {
            set_u32_at(e, FONT_LINE_RISES + 4 * i, 9);
        }
    }

    /// A text manager setup: engine, geometry doubles, a font and the
    /// manager.
    fn text_setup() -> (Engine, Ptr<Font>, Ptr<FontManager>) {
        let mut e = engine();
        geometry(&mut e);
        manager_constants(&mut e);
        let f = font(&mut e);
        let m = manager(&mut e, f);
        (e, f, m)
    }

    fn manager_info(e: &mut Engine) -> Ptr<ManagerTextData> {
        let info: Ptr<ManagerTextData> = e.new_object();
        e.set(info, ManagerTextData::iWidth, 1000);
        e.set(info, ManagerTextData::iHeight, 1000);
        e.set(info, ManagerTextData::iLines, 5);
        e.set(info, ManagerTextData::cLineSep, b'\n');
        info
    }

    /// A geometry shape with room for `quads` quads, as the scissor shape
    /// double builds it.
    fn quad_shape(e: &mut Engine, quads: u32) -> u32 {
        let shape = e.mem.alloc(0xd4);
        let vertices = e.mem.alloc(quads * 0x30);
        let uvs = e.mem.alloc(quads * 0x20);
        let triangles = e.mem.alloc(quads * 12);
        scissor_constructor_double(
            e,
            &[
                shape,
                quads * 4,
                vertices,
                0,
                0,
                uvs,
                1,
                0,
                quads * 2,
                triangles,
            ],
        );
        shape
    }

    fn vertex(e: &Engine, shape: u32, k: u32) -> [f32; 3] {
        let data = e.mem.u32(shape + 0xb8);
        let v = e.mem.u32(data + 0x20) + 0x0c * k;
        [e.mem.f32(v), e.mem.f32(v + 4), e.mem.f32(v + 8)]
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|k| e.mem.f32(at + 4 * k)).collect()
    }

    fn char_data(e: &mut Engine, font: i32, c: u8) -> u32 {
        let data: Ptr<CharData> = e.new_object();
        font_manager_char_data_char_data(e, data, font, c, 0, 0, 0, 0, 1);
        font_manager_char_data_set_char(e, data, c);
        data.addr()
    }

    fn list_elements(e: &Engine, list: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = e.mem.u32(list);
        while node != 0 {
            out.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    #[test]
    fn add_tail_links_a_new_item_holding_the_element() {
        let mut e = engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x77);
        ni_tpointer_list_base_add_tail(&mut e, list, Ptr::new(element));
        let head = e.get(list, NiTPointerList::m_pkHead);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        assert_eq!(e.mem.u32(head + 8), 0x77);
        assert_eq!(e.mem.block_size(head), Some(16));
        assert_eq!(
            calls(&e, NI_TLIST_ADD_NODE_TAIL),
            vec![vec![list.addr(), head]]
        );
    }

    #[test]
    fn font_constructor_copies_the_name_and_loads_when_asked() {
        let mut e = engine();
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, nothing);
        e.register(FILE_FINDER_GET_FILE, |_, _| ret(0));
        let font: Ptr<Font> = e.new_object();
        let name = cstr(&mut e, b"Fonts\\a.fnt");
        let tls = e.tls() + 0x2b4;
        e.mem.set_u32(tls, 5);
        assert_eq!(font_font(&mut e, font, 3, name, 1), font);
        let copy = e.get(font, Font::pFontFile);
        assert_ne!(copy, name);
        assert_eq!(e.mem.cstr(copy), b"Fonts\\a.fnt");
        assert_eq!(e.get(font, Font::iFontNum), 3);
        assert_eq!(e.mem.u32(font.addr() + 0x44), BUTTON_ICON_ARRAY_VTABLE);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                font.addr() + 0x0c,
                4,
                8,
                0x00a1_b410,
                NI_POINTER_DESTRUCTOR
            ]]
        );
        // `Load` ran and failed to find the file: the reference stays 0.
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE)[0][0], copy);
        assert_eq!(e.get(font, Font::iRefCount), 0);
        assert_eq!(e.mem.u32(tls), 5);

        let other: Ptr<Font> = e.new_object();
        e.set(other, Font::iFontNum, 9);
        font_font(&mut e, other, 4, 0, 1);
        assert_eq!(e.get(other, Font::iFontNum), 9);
        assert_eq!(e.get(other, Font::pFontFile), 0);
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE).len(), 1);
    }

    #[test]
    fn font_destructor_frees_the_static_block_and_the_name() {
        let mut e = engine();
        e.register(VECTOR_DESTRUCTOR_ITERATOR, nothing);
        let font: Ptr<Font> = e.new_object();
        let block = e.mem.alloc(16);
        e.set_global(FONT_STATIC_BLOCK, block);
        e.set_global(FONT_STATIC_BLOCK_WORD, 9u32);
        let file = cstr(&mut e, b"x");
        e.set(font, Font::pFontFile, file);
        fn_00a121e0(&mut e, font);
        assert_eq!(e.global::<u32>(FONT_STATIC_BLOCK), 0);
        assert_eq!(e.global::<u32>(FONT_STATIC_BLOCK_WORD), 0);
        assert_eq!(e.mem.block_size(block), None);
        assert_eq!(e.mem.block_size(file), None);
        assert_eq!(e.mem.u32(font.addr() + 0x44), BUTTON_ICON_ARRAY_VTABLE);
        assert_eq!(
            calls(&e, BS_STRING_SET),
            vec![vec![font.addr() + 0x3c, 0, 0]]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![font.addr() + 0x0c, 4, 8, NI_POINTER_DESTRUCTOR]]
        );
    }

    #[test]
    fn convert_character_replaces_typographic_quotes() {
        let mut e = engine();
        let c = e.mem.alloc(1);
        for (from, to) in [
            (0x91u8, b'\''),
            (0x92, b'\''),
            (0x93, b'"'),
            (0x94, b'"'),
            (b'a', b'a'),
        ] {
            e.mem.set_u8(c, from);
            font_convert_character(&mut e, Ptr::new(c));
            assert_eq!(e.mem.u8(c), to);
        }
    }

    #[test]
    fn release_textures_clears_each_texture_and_frees_the_data() {
        let mut e = engine();
        e.register(NI_POINTER_ASSIGN, nothing);
        let font = font(&mut e);
        let data = e.get(font, Font::pFontData);
        e.mem.set_i32(data + 4, 2);
        fn_00a12300(&mut e, font);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![font.addr() + 0x0c, 0], vec![font.addr() + 0x10, 0]]
        );
        assert_eq!(e.get(font, Font::pFontData), 0);
        assert_eq!(e.mem.block_size(data), None);
    }

    #[test]
    fn line_offsets_center_right_and_left() {
        let mut e = engine();
        let font: Ptr<Font> = e.new_object();
        manager(&mut e, font);
        // Font number 0 is out of range: the measurement is the start point.
        e.mem.set_f32(DIMENSIONS_START, 40.0);
        let offset = e.mem.alloc(4);
        let text = cstr(&mut e, b"abc");
        e.mem.set_f32(offset, 0.0);
        font_get_line_offsets(&mut e, font, text, Ptr::new(offset), 0, 0x12, 0);
        assert_eq!(e.mem.f32(offset), -20.0);
        e.mem.set_f32(offset, 5.0);
        font_get_line_offsets(&mut e, font, text, Ptr::new(offset), 0, 4, 0);
        assert_eq!(e.mem.f32(offset), -40.0);
        e.mem.set_f32(offset, 5.0);
        font_get_line_offsets(&mut e, font, text, Ptr::new(offset), 0, 1, 0);
        assert_eq!(e.mem.f32(offset), 0.0);
    }

    #[test]
    fn make_string_places_the_letters_and_measures_the_width() {
        let (mut e, font, _) = text_setup();
        let text = bs_string(&mut e, b"AB");
        let width = e.mem.alloc(4);
        let color = e.mem.alloc(16);
        let shape = font_make_string(
            &mut e,
            font,
            5.0,
            100.0,
            1.0,
            text,
            Ptr::new(width),
            0,
            color,
            0,
            1,
        );
        assert_ne!(shape, 0);
        assert_eq!(floats(&e, shape + 0x58, 3), vec![5.0, 1.0, 100.0]);
        assert_eq!(vertex(&e, shape, 0), [5.0, 1.0, 102.0]);
        assert_eq!(vertex(&e, shape, 1), [5.0, 1.0, 90.0]);
        assert_eq!(vertex(&e, shape, 6), [25.0, 1.0, 102.0]);
        assert_eq!(e.mem.i32(width), 20);
        let data = e.mem.u32(shape + 0xb8);
        let triangles = e.mem.u32(data + 0x30);
        let t: Vec<u16> = (0..12).map(|k| e.mem.u16(triangles + 2 * k)).collect();
        assert_eq!(t, vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7]);
        assert_eq!(calls(&e, NI_BOUND_COMPUTE_FROM_DATA)[0][1], 8);

        // Raised: two space heights above the base line lower the line.
        let raised = font_make_string(
            &mut e,
            font,
            0.0,
            100.0,
            0.0,
            text,
            Ptr::new(width),
            0,
            color,
            1,
            1,
        );
        assert_eq!(e.mem.f32(raised + 0x60), 96.0);

        let empty = bs_string(&mut e, b"");
        assert_eq!(
            font_make_string(
                &mut e,
                font,
                0.0,
                0.0,
                0.0,
                empty,
                Ptr::new(width),
                0,
                color,
                0,
                0
            ),
            0
        );
    }

    #[test]
    fn create_text_prepares_measures_and_builds_the_string() {
        let (mut e, font, _) = text_setup();
        let source = cstr(&mut e, b"AB");
        let text = e.mem.alloc(4);
        e.mem.set_u32(text, source);
        let width = e.mem.alloc(4);
        let height = e.mem.alloc(4);
        let shape_out = e.mem.alloc(4);
        let icons_out = e.mem.alloc(4);
        let color = e.mem.alloc(16);
        font_create_text(
            &mut e,
            font,
            Ptr::new(text),
            Ptr::new(width),
            Ptr::new(height),
            0,
            0,
            0,
            b'\n',
            color,
            Ptr::new(shape_out),
            Ptr::new(icons_out),
        );
        assert_eq!(e.mem.i32(width), 20);
        assert_eq!(e.mem.i32(height), 12);
        let shape = e.mem.u32(shape_out);
        assert_ne!(shape, 0);
        assert_eq!(e.mem.u32(icons_out), 0);
        assert_eq!(floats(&e, shape + 0x58, 3), vec![0.0, 0.0, 20.0]);
        assert_eq!(vertex(&e, shape, 0), [0.0, 0.0, 22.0]);
        assert_eq!(vertex(&e, shape, 4), [10.0, 0.0, 22.0]);
        assert_eq!(calls(&e, TEXT_DATA_DESTRUCTOR).len(), 1);
        let constructed = &calls(&e, TEXT_DATA_CONSTRUCTOR)[0];
        assert_eq!(
            &constructed[1..],
            &[0, 0x7fff_ffff, 0, 0x7fff_ffff, b'\n' as u32]
        );
    }

    fn find_replacement_double(e: &mut Engine, a: &[u32]) -> Ret {
        let name = e.mem.cstr(a[0]);
        if name == b"n" || name == b"&n;" {
            e.mem.set_cstr(a[1], b"Q");
            ret(1)
        } else {
            ret(0)
        }
    }

    #[test]
    fn prep_text_wraps_at_spaces_and_replaces_entities() {
        let (mut e, font, _) = text_setup();
        e.register(FIND_TEXT_REPLACEMENT_STRING, find_replacement_double);
        e.register(TEST_CONSTANT_FOR_GAME_SETTINGS, |_, _| ret(0));
        let data: Ptr<TextData> = e.new_object();
        e.set(data, TextData::iWidth, 25);
        e.set(data, TextData::cLineSep, b'\n');
        let text = cstr(&mut e, b"ab cd");
        font_prep_text(&mut e, font, text, data);
        assert_eq!(string_text(&e, data.cast()), b"ab\ncd");
        assert_eq!(e.get(data, TextData::iWidth), 20);
        assert_eq!(e.get(data, TextData::iHeight), 22);
        assert_eq!(e.get(data, TextData::iLineEnd), 2);
        assert_eq!(e.get(data, TextData::iCharCount), 5);
        let widths = data.addr() + 0x20;
        assert_eq!(e.mem.i32(widths), 20);
        assert_eq!(e.mem.i32(e.mem.u32(widths + 4)), 20);

        let data: Ptr<TextData> = e.new_object();
        e.set(data, TextData::cLineSep, b'\n');
        let text = cstr(&mut e, b"x&n;y");
        font_prep_text(&mut e, font, text, data);
        assert_eq!(string_text(&e, data.cast()), b"xQy");
        assert_eq!(e.get(data, TextData::iWidth), 30);
        assert_eq!(e.get(data, TextData::iCharCount), 3);

        // A null text changes nothing.
        let untouched: Ptr<TextData> = e.new_object();
        font_prep_text(&mut e, font, 0, untouched);
        assert_eq!(e.get(untouched, TextData::iWidth), 0);
    }

    fn icon_constants(e: &mut Engine) {
        for (addr, v) in [
            (ICON_FIRST, 60.0f32),
            (ICON_FACE_SECOND, -15.0),
            (ICON_FACE_FOURTH, -30.0),
            (ICON_SHOULDER_SECOND, -12.5),
            (ICON_SHOULDER_FOURTH, -25.0),
            (ICON_OTHER_SECOND, -10.0),
            (ICON_OTHER_FOURTH, -20.0),
        ] {
            set_f32_at(e, addr, v);
        }
        for (addr, v) in [
            (ICON_ADJUST_SECOND, 2.5f64),
            (ICON_ADJUST_THIRD_FONT_1, 3.0),
            (ICON_ADJUST_FOURTH, 5.0),
            (ICON_ADJUST_THIRD_FONT_7, 10.0),
        ] {
            set_f64_at(e, addr, v);
        }
    }

    #[test]
    fn icon_size_depends_on_the_button_name_and_font() {
        let mut e = engine();
        icon_constants(&mut e);
        let font: Ptr<Font> = e.new_object();
        e.set(font, Font::fFontHeight, 20.0);
        let out = e.mem.alloc(16);
        e.set(font, Font::iFontNum, 1);
        let name = cstr(&mut e, b"a");
        fn_00a14170(&mut e, font, Ptr::new(out), name);
        assert_eq!(floats(&e, out, 4), vec![60.0, -17.5, 17.0, -35.0]);
        e.set(font, Font::iFontNum, 7);
        let name = cstr(&mut e, b"lt");
        fn_00a14170(&mut e, font, Ptr::new(out), name);
        assert_eq!(floats(&e, out, 4), vec![60.0, -10.0, 10.0, -30.0]);
        e.set(font, Font::iFontNum, 2);
        let name = cstr(&mut e, b"lb");
        fn_00a14170(&mut e, font, Ptr::new(out), name);
        assert_eq!(floats(&e, out, 4), vec![60.0, -10.0, 20.0, -20.0]);
    }

    #[test]
    fn add_char_writes_a_quad_and_advances_the_pen() {
        let mut e = engine();
        geometry(&mut e);
        let font: Ptr<Font> = e.new_object();
        let shape = quad_shape(&mut e, 2);
        let l = e.mem.alloc(0x38);
        for k in 1..=8u32 {
            e.mem.set_f32(l + 4 * k, k as f32);
        }
        for (off, v) in [
            (0x24u32, 8.0f32),
            (0x28, 12.0),
            (0x2c, 1.0),
            (0x30, 2.0),
            (0x34, 3.0),
        ] {
            e.mem.set_f32(l + off, v);
        }
        let pen = e.mem.alloc(12);
        e.mem.set_f32(pen, 10.0);
        e.mem.set_f32(pen + 4, 5.0);
        e.mem.set_f32(pen + 8, 100.0);
        font_add_char(&mut e, font, l, 1, shape, Ptr::new(pen), 0);
        assert_eq!(vertex(&e, shape, 4), [11.0, 5.0, 103.0]);
        assert_eq!(vertex(&e, shape, 5), [11.0, 5.0, 91.0]);
        assert_eq!(vertex(&e, shape, 6), [19.0, 5.0, 103.0]);
        assert_eq!(vertex(&e, shape, 7), [19.0, 5.0, 91.0]);
        let data = e.mem.u32(shape + 0xb8);
        let uv = e.mem.u32(data + 0x2c) + 0x20;
        assert_eq!(
            floats(&e, uv, 8),
            vec![1.0, 2.0, 5.0, 6.0, 3.0, 4.0, 7.0, 8.0]
        );
        let triangles = e.mem.u32(data + 0x30) + 12;
        let t: Vec<u16> = (0..6).map(|k| e.mem.u16(triangles + 2 * k)).collect();
        assert_eq!(t, vec![4, 5, 6, 6, 5, 7]);
        assert_eq!(e.mem.f32(pen), 21.0);

        // A letter without width gets no spacing.
        e.mem.set_f32(l + 0x24, 0.0);
        font_add_char(&mut e, font, l, 0, shape, Ptr::new(pen), 0);
        assert_eq!(e.mem.f32(pen), 22.0);
    }

    #[test]
    fn add_icon_writes_a_square_with_the_icon_rectangle() {
        let mut e = engine();
        geometry(&mut e);
        let font: Ptr<Font> = e.new_object();
        let icons = e.mem.alloc(0x20);
        for (k, v) in [16.0f32, 2.0, 4.0, 3.0, 0.1, 0.2, 0.3, 0.4]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(icons + 4 * k as u32, *v);
        }
        e.mem.set_u32(font.addr() + 0x48, icons);
        let shape = quad_shape(&mut e, 1);
        let pen = e.mem.alloc(12);
        e.mem.set_f32(pen, 10.0);
        e.mem.set_f32(pen + 4, 5.0);
        e.mem.set_f32(pen + 8, 100.0);
        font_add_icon(&mut e, font, 0, shape, Ptr::new(pen));
        assert_eq!(vertex(&e, shape, 0), [12.0, 5.0, 104.0]);
        assert_eq!(vertex(&e, shape, 1), [12.0, 5.0, 88.0]);
        assert_eq!(vertex(&e, shape, 2), [28.0, 5.0, 104.0]);
        assert_eq!(vertex(&e, shape, 3), [28.0, 5.0, 88.0]);
        let data = e.mem.u32(shape + 0xb8);
        let uv = e.mem.u32(data + 0x2c);
        assert_eq!(
            floats(&e, uv, 8),
            vec![0.1, 0.3, 0.1, 0.4, 0.2, 0.3, 0.2, 0.4]
        );
        assert_eq!(e.mem.f32(pen), 29.0);
    }

    #[test]
    fn make_tri_shape_builds_a_textured_shape() {
        let mut e = engine();
        geometry(&mut e);
        let font = font(&mut e);
        let color = e.mem.alloc(16);
        for k in 0..4 {
            e.mem.set_u32(color + 4 * k, 0x100 + k);
        }
        let shape = font_make_tri_shape(&mut e, font, 3, color, 1);
        let ctor = &calls(&e, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR)[0];
        assert_eq!(ctor[0], shape);
        assert_eq!(ctor[1], 12);
        assert_eq!(e.mem.block_size(ctor[2]), Some(0x90));
        assert_eq!(e.mem.block_size(ctor[5]), Some(0x60));
        assert_eq!(ctor[8], 6);
        assert_eq!(calls(&e, NI_ALLOC), vec![vec![36]]);
        let data = e.mem.u32(shape + 0xb8);
        assert_eq!((e.mem.u8(data + 0x38), e.mem.u8(data + 0x39)), (0, 1));
        assert_eq!(
            calls(&e, NI_FIXED_STRING_CREATE),
            vec![vec![INTERFACE_TEXT_NAME]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x1234 - 8]]);
        let property = calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY)[0][1];
        let words: Vec<u32> = (0..4).map(|k| e.mem.u32(property + 0x68 + 4 * k)).collect();
        assert_eq!(words, vec![0x100, 0x101, 0x102, 0x103]);
        assert_eq!(
            calls(&e, BS_SHADER_MANAGER_PREPARE_OBJECT),
            vec![vec![shape, 0, 0]]
        );
        assert_eq!(
            calls(&e, TILE_SHADER_PROPERTY_SET_TILE_TEXTURE),
            vec![vec![property, 0x9000]]
        );
        assert_eq!(
            calls(&e, INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY),
            vec![vec![shape]]
        );
        font_make_tri_shape(&mut e, font, 1, color, 0);
        assert_eq!(calls(&e, BS_SHADER_MANAGER_PREPARE_OBJECT).len(), 1);
    }

    #[test]
    fn make_icons_tri_shape_textures_with_the_atlas() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        geometry(&mut e);
        let names = Rc::new(RefCell::new(Vec::new()));
        let seen = names.clone();
        e.register_double(NI_FIXED_STRING_CREATE, move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[0]));
            ret(0x1234)
        });
        set_text_at(&mut e, INTERFACE_TEXT_ICONS_NAME, b"Interface: TextIcons");
        set_text_at(&mut e, TEXTURES_PATH_FORMAT, b"Data\\Textures\\%s");
        for k in 0..4 {
            e.mem.set_u32(ICON_SHADER_COLOR + 4 * k, 0x200 + k);
        }
        let font = font(&mut e);
        e.mem.set_u32(font.addr() + 0x4c, 2);
        let atlas = cstr(&mut e, b"ui\\icons.dds");
        e.mem.set_u32(font.addr() + 0x3c, atlas);
        let shape = font_make_icons_tri_shape(&mut e, font);
        assert_eq!(calls(&e, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR)[0][1], 8);
        assert_eq!(
            *names.borrow(),
            vec![
                b"Interface: TextIcons".to_vec(),
                b"Data\\Textures\\ui\\icons.dds".to_vec()
            ]
        );
        let property = calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY)[0][1];
        let words: Vec<u32> = (0..4).map(|k| e.mem.u32(property + 0x68 + 4 * k)).collect();
        assert_eq!(words, vec![0x200, 0x201, 0x202, 0x203]);
        let create = &calls(&e, NI_SOURCE_TEXTURE_CREATE)[0];
        assert_eq!(&create[1..], &[TEXTURE_FORMAT_PREFS, 1, 0]);
        assert_eq!(
            calls(&e, TILE_SHADER_PROPERTY_SET_TILE_TEXTURE),
            vec![vec![property, 0x5678]]
        );
        assert_eq!(
            calls(&e, BS_SHADER_MANAGER_PREPARE_OBJECT),
            vec![vec![shape, 0, 0]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn release_reference_frees_the_data_at_zero() {
        let mut e = engine();
        e.register(NI_POINTER_ASSIGN, nothing);
        let font = font(&mut e);
        e.set(font, Font::iRefCount, 2);
        fn_00a152d0(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 1);
        assert_ne!(e.get(font, Font::pFontData), 0);
        fn_00a152d0(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 0);
        assert_eq!(e.get(font, Font::pFontData), 0);
        fn_00a152d0(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 0);
    }

    /// A file object for `Font::Load`: open (+0x2C), its read function at +8
    /// copying from `content`, its size (virtual slot 0x28) and its deleting
    /// destructor (slot 0).
    fn font_file(e: &mut Engine, content: u32, size: u32) -> u32 {
        const READ: u32 = 0x7000_0008;
        const SIZE: u32 = 0x7000_0028;
        const DELETE: u32 = 0x7000_0000;
        let file = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable, DELETE);
        e.mem.set_u32(vtable + 0x28, SIZE);
        e.mem.set_u32(file, vtable);
        e.mem.set_u32(file + 8, READ);
        e.mem.set_u8(file + 0x2c, 1);
        e.register(DELETE, nothing);
        e.register_double(SIZE, move |_, _| ret(size));
        e.register_double(READ, move |e, a| {
            let bytes = e.mem.bytes(content, a[2]);
            e.mem.write(a[1], &bytes);
            ret(a[2])
        });
        file
    }

    #[test]
    fn load_reads_the_font_data_and_derives_the_metrics() {
        let mut e = engine();
        let content = e.mem.alloc(0x3928);
        e.mem.set_f32(content, 10.0);
        let a = content + 0x128 + 0x41 * 0x38;
        e.mem.set_f32(a + 0x28, 20.0);
        e.mem.set_f32(a + 0x34, 15.0);
        e.mem.set_f32(content + 0x84c, 7.0);
        e.mem.set_f32(content + 0x858, 1.0);
        let file = font_file(&mut e, content, 0x3928);
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| ret(file));
        let font: Ptr<Font> = e.new_object();
        let name = cstr(&mut e, b"a.fnt");
        e.set(font, Font::pFontFile, name);
        let tls = e.tls() + 0x2b4;
        e.mem.set_u32(tls, 3);
        font_load(&mut e, font);
        assert_eq!(e.mem.u32(tls), 3);
        assert_eq!(e.get(font, Font::iRefCount), 1);
        let data = e.get(font, Font::pFontData);
        assert_eq!(e.mem.block_size(data), Some(0x3928));
        assert_eq!(e.get(font, Font::fFontHeight), 15.0);
        assert_eq!(e.get(font, Font::fMaxDrop), -5.0);
        assert_eq!(floats(&e, data + 0x84c, 5), vec![1.0, 20.0, 0.0, 7.0, 15.0]);
        assert_eq!(e.mem.f32(data + 0x2450), 20.0);
        assert_eq!(e.mem.f32(data + 0x150), 20.0);
        assert_eq!(e.mem.f32(data + 0x15c), 15.0);
        assert_eq!(e.mem.u32(file + 4), 0x3928);
        assert_eq!(calls(&e, 0x7000_0000), vec![vec![file, 1]]);

        // Loaded fonts only count references.
        font_load(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 2);
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE).len(), 1);
    }

    #[test]
    fn load_stops_on_a_missing_file_or_too_many_textures() {
        let mut e = engine();
        e.register(FILE_FINDER_GET_FILE, |_, _| ret(0));
        let font: Ptr<Font> = e.new_object();
        let name = cstr(&mut e, b"a.fnt");
        e.set(font, Font::pFontFile, name);
        font_load(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![FONT_NOT_FOUND_FORMAT, name]]
        );

        let content = e.mem.alloc(0x3928);
        e.mem.set_i32(content + 4, 9);
        let file = font_file(&mut e, content, 0x3928);
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| ret(file));
        font_load(&mut e, font);
        assert_eq!(e.get(font, Font::iRefCount), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE)[1],
            vec![TOO_MANY_TEXTURES_FORMAT, name, 8, 9]
        );
    }

    #[test]
    fn line_extraction_keeps_one_line() {
        let mut e = engine();
        let text = bs_string(&mut e, b"one\ntwo\nthree");
        fn_00a15ee0(&mut e, 0, text, 2);
        assert_eq!(string_text(&e, text), b"two");
        assert_eq!(calls(&e, BS_STRING_SET).len(), 2);
        let missing = bs_string(&mut e, b"one");
        fn_00a15ee0(&mut e, 0, missing, 5);
        assert_eq!(string_text(&e, missing), b" ");
        let single = bs_string(&mut e, b"abc");
        fn_00a15ee0(&mut e, 0, single, 1);
        assert_eq!(string_text(&e, single), b"abc");
        assert_eq!(calls(&e, BS_STRING_SET).len(), 3);
    }

    #[test]
    fn line_break_text_breaks_at_the_last_space() {
        let mut e = engine();
        let font = font(&mut e);
        let text = bs_string(&mut e, b"aa aa");
        let width = e.mem.alloc(4);
        e.mem.set_i32(width, 25);
        let lines = e.mem.alloc(4);
        let count = font_line_break_text(
            &mut e,
            font,
            text,
            Ptr::new(width),
            Ptr::new(lines),
            0,
            b'\n',
        );
        assert_eq!(count, 2);
        assert_eq!(string_text(&e, text), b"aa\naa");
        assert_eq!(e.mem.i32(width), 30);
        assert_eq!(e.mem.i32(lines), 22);

        let none: Ptr<BSStringT> = e.new_object();
        assert_eq!(
            font_line_break_text(
                &mut e,
                font,
                none,
                Ptr::new(width),
                Ptr::new(lines),
                0,
                b'\n'
            ),
            0
        );
    }

    #[test]
    fn separated_part_is_copied_with_its_separator() {
        let mut e = engine();
        let text = cstr(&mut e, b"a|bc|d");
        let string: Ptr<BSStringT> = e.new_object();
        fn_00a16700(&mut e, 0, text, string, 1, b'|');
        assert_eq!(string_text(&e, string), b"|bc");
        assert_eq!(e.mem.cstr(text), b"a|bc|d");
        fn_00a16700(&mut e, 0, text, string, 0, b'|');
        assert_eq!(string_text(&e, string), b"a");
    }

    fn manager_doubles(e: &mut Engine) {
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, nothing);
        e.register(VECTOR_DESTRUCTOR_ITERATOR, nothing);
        e.register(FILE_FINDER_GET_FILE, |_, _| ret(0));
        e.register(FATAL_ERROR, nothing);
    }

    #[test]
    fn init_creates_the_manager_once() {
        let mut e = engine();
        manager_doubles(&mut e);
        font_manager_init(&mut e);
        let manager: u32 = e.global(FONT_MANAGER_INSTANCE);
        assert_eq!(e.mem.block_size(manager), Some(0x28));
        assert_ne!(e.mem.u32(manager), 0);
        assert_eq!(calls(&e, FATAL_ERROR).len(), 1);
        font_manager_init(&mut e);
        assert_eq!(e.global::<u32>(FONT_MANAGER_INSTANCE), manager);
    }

    #[test]
    fn manager_shutdown_destroys_and_clears_the_manager() {
        let mut e = engine();
        manager_doubles(&mut e);
        let manager = e.mem.alloc(0x24);
        let font = e.mem.alloc(0x54);
        e.mem.set_u32(manager + 8, font);
        e.set_global(FONT_MANAGER_INSTANCE, manager);
        fn_00a16850(&mut e);
        assert_eq!(e.global::<u32>(FONT_MANAGER_INSTANCE), 0);
        assert_eq!(e.mem.block_size(manager), None);
        assert_eq!(e.mem.block_size(font), None);
        fn_00a16850(&mut e);
    }

    #[test]
    fn manager_constructor_creates_the_eight_fonts() {
        let mut e = engine();
        manager_doubles(&mut e);
        for n in 0..8u32 {
            let name = cstr(&mut e, format!("font{n}.fnt").as_bytes());
            e.mem.set_u32(FONT_FILENAME_SETTINGS + 0x0c * n + 4, name);
        }
        let manager: Ptr<FontManager> = e.new_object();
        e.set(manager, FontManager::bUseNewFonts, 1);
        assert_eq!(font_manager_font_manager(&mut e, manager), manager);
        for n in 0..8u32 {
            let font: Ptr<Font> = Ptr::new(e.mem.u32(manager.addr() + 4 * n));
            assert_eq!(e.get(font, Font::iFontNum), n as i32 + 1);
            let file = e.get(font, Font::pFontFile);
            assert_eq!(e.mem.cstr(file), format!("font{n}.fnt").as_bytes());
        }
        assert_eq!(e.get(manager, FontManager::bUseNewFonts), 0);
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE).len(), 8);
        assert_eq!(
            calls(&e, FATAL_ERROR),
            vec![vec![MISSING_FONT_DATA_MESSAGE]]
        );
    }

    #[test]
    fn manager_destructor_frees_every_font() {
        let mut e = engine();
        manager_doubles(&mut e);
        let manager: Ptr<FontManager> = e.new_object();
        let first = e.mem.alloc(0x54);
        let last = e.mem.alloc(0x54);
        e.mem.set_u32(manager.addr(), first);
        e.mem.set_u32(manager.addr() + 0x1c, last);
        fn_00a16d20(&mut e, manager);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(last), None);
        assert_eq!(e.mem.u32(manager.addr()), 0);
        assert_eq!(e.mem.u32(manager.addr() + 0x1c), 0);
        assert_eq!(calls(&e, VECTOR_DESTRUCTOR_ITERATOR).len(), 2);
    }

    #[test]
    fn char_types_follow_the_tables() {
        let mut e = engine();
        let m: Ptr<FontManager> = Ptr::NULL;
        let cases = [
            (0i8, 0x20u32),
            (b'"' as i8, 8),
            (b'\'' as i8, 8),
            (b'<' as i8, 1),
            (b'{' as i8, 1),
            (b'=' as i8, 0x10),
            (b'>' as i8, 2),
            (b'}' as i8, 2),
            (b' ' as i8, 4),
            (b'\n' as i8, 4),
            (-0x70, 4),
            (b'a' as i8, 0),
            (b'|' as i8, 0),
        ];
        for (c, kind) in cases {
            assert_eq!(font_manager_get_char_type(&mut e, m, c), kind, "{c}");
        }
    }

    #[test]
    fn collect_to_gathers_text_up_to_a_stop() {
        let (mut e, _, m) = text_setup();
        e.register(FIND_TEXT_REPLACEMENT_STRING, find_replacement_double);
        let buffers = e.mem.alloc(16);
        let pos = buffers;
        let kind = buffers + 4;
        let character = buffers + 8;
        let result: Ptr<BSStringT> = e.new_object();
        let text = cstr(&mut e, b"ab<c");
        let r = font_manager_collect_to(
            &mut e,
            m,
            result,
            text,
            Ptr::new(pos),
            1,
            0,
            Ptr::new(kind),
            Ptr::new(character),
            1,
        );
        assert_eq!(r, result);
        assert_eq!(string_text(&e, result), b"ab");
        assert_eq!(
            (e.mem.u32(pos), e.mem.u32(kind), e.mem.u8(character)),
            (3, 1, b'<')
        );

        // Accepting white space: the first other character stops, uncounted.
        let text = cstr(&mut e, b"  x");
        e.mem.set_u32(pos, 0);
        font_manager_collect_to(
            &mut e,
            m,
            result,
            text,
            Ptr::new(pos),
            0,
            4,
            Ptr::new(kind),
            Ptr::new(character),
            0,
        );
        assert_eq!(
            (e.mem.u32(pos), e.mem.u32(kind), e.mem.u8(character)),
            (2, 0, b'x')
        );

        // Entities are replaced; the terminator ends with type 0x20.
        let text = cstr(&mut e, b"&n;z");
        e.mem.set_u32(pos, 0);
        font_manager_collect_to(
            &mut e,
            m,
            result,
            text,
            Ptr::new(pos),
            1,
            0,
            Ptr::new(kind),
            Ptr::new(character),
            1,
        );
        assert_eq!(string_text(&e, result), b"Qz");
        assert_eq!((e.mem.u32(pos), e.mem.u32(kind)), (4, 0x20));
    }

    #[test]
    fn prep_hypertext_lays_out_tagged_text() {
        let (mut e, _, m) = text_setup();
        let info = manager_info(&mut e);
        let text = bs_string(&mut e, b"<BR>ab");
        let doc = font_manager_prep_hypertext(&mut e, m, text, info);
        assert!(!doc.is_null());
        assert_eq!(string_text(&e, text), b"ab");
        assert_eq!(e.get(info, ManagerTextData::bIsHypertext), 1);
        assert_eq!(e.get(info, ManagerTextData::iNumPages), 1);
        assert_eq!(e.get(info, ManagerTextData::iNumLines), 1);
        assert_eq!(e.get(info, ManagerTextData::iWidth), 20);
        assert_eq!(e.get(info, ManagerTextData::iHeight), 9);

        // Plain text is not hypertext.
        let plain = bs_string(&mut e, b"ab");
        let info = manager_info(&mut e);
        assert!(font_manager_prep_hypertext(&mut e, m, plain, info).is_null());
        assert_eq!(e.get(info, ManagerTextData::bIsHypertext), 0);
    }

    #[test]
    fn prep_hypertext_applies_font_colors_and_images() {
        let (mut e, _, m) = text_setup();
        let info = manager_info(&mut e);
        let text = bs_string(
            &mut e,
            b"<FONT COLOR=\"FF0080\">a</FONT><IMG SRC=x WIDTH=7>b",
        );
        let doc = font_manager_prep_hypertext(&mut e, m, text, info);
        assert_eq!(string_text(&e, text), b"ab");
        let page = font_manager_text_doc_current_page(&mut e, doc);
        let line = list_elements(&e, page)[0];
        let chars = list_elements(&e, line);
        assert_eq!(chars.len(), 3);
        assert_eq!(
            floats(&e, chars[0] + 8, 4),
            vec![1.0, 0.0, (128.0f64 / 255.0) as f32, 1.0]
        );
        assert_eq!(e.mem.u8(chars[1] + 4), 0);
        assert_eq!(e.mem.cstr(e.mem.u32(chars[1] + 0x1c)), b"X");
        assert_eq!(e.mem.i32(chars[1] + 0x24), 7);
        assert_eq!(floats(&e, chars[2] + 8, 4), vec![0.5, 0.25, 0.125, 1.0]);
    }

    #[test]
    fn manager_prep_text_adds_plain_characters() {
        let (mut e, _, m) = text_setup();
        let info = manager_info(&mut e);
        let text = bs_string(&mut e, b"a\nb");
        let doc = font_manager_prep_text(&mut e, m, text, info.addr());
        assert!(!doc.is_null());
        assert_eq!(e.get(info, ManagerTextData::bIsHypertext), 0);
        assert_eq!(e.get(info, ManagerTextData::iNumPages), 1);
        assert_eq!(e.get(info, ManagerTextData::iNumLines), 2);
        assert_eq!(e.get(info, ManagerTextData::iWidth), 10);
        assert_eq!(e.get(info, ManagerTextData::iHeight), 18);
        assert_eq!(string_text(&e, info.at(ManagerTextData::xNewText)), b"a\nb");

        let none: Ptr<BSStringT> = e.new_object();
        assert!(font_manager_prep_text(&mut e, m, none, info.addr()).is_null());
    }

    /// A node whose `AttachChild` (virtual slot 0xDC) is recorded.
    fn out_node(e: &mut Engine) -> u32 {
        const ATTACH: u32 = 0x7000_00dc;
        let node = e.mem.alloc(8);
        let vtable = e.mem.alloc(0xe0);
        e.mem.set_u32(vtable + 0xdc, ATTACH);
        e.mem.set_u32(node, vtable);
        e.register(ATTACH, nothing);
        node
    }

    #[test]
    fn manager_create_text_renders_into_the_node() {
        let (mut e, _, m) = text_setup();
        let info = manager_info(&mut e);
        let out = out_node(&mut e);
        let source = cstr(&mut e, b"a");
        let created = font_manager_create_text(&mut e, m, source, 1 | 1 << 16, info.addr(), out);
        assert!(created);
        let attached = calls(&e, 0x7000_00dc);
        assert_eq!(attached.len(), 1);
        assert_eq!(attached[0][0], out);
        assert_eq!(attached[0][2], 1);
    }

    #[test]
    fn current_page_follows_the_page_number() {
        let mut e = engine();
        let doc: Ptr<TextDoc> = e.new_object();
        assert_eq!(font_manager_text_doc_current_page(&mut e, doc), 0);
        for page in [0x111u32, 0x222] {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, page);
            list_add_tail_double(&mut e, &[doc.addr(), slot]);
        }
        e.set(doc, TextDoc::iPageNum, 1);
        assert_eq!(font_manager_text_doc_current_page(&mut e, doc), 0x222);
        e.set(doc, TextDoc::iPageNum, 2);
        assert_eq!(font_manager_text_doc_current_page(&mut e, doc), 0);
    }

    #[test]
    fn render_places_the_characters_of_each_line() {
        let (mut e, _, m) = text_setup();
        let info = manager_info(&mut e);
        let text = bs_string(&mut e, b"ab");
        let doc = font_manager_prep_text(&mut e, m, text, info.addr());
        let out = out_node(&mut e);
        font_manager_text_doc_render(&mut e, doc, out, info.addr());
        let shape = calls(&e, 0x7000_00dc)[0][1];
        assert_eq!(calls(&e, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR)[0][1], 8);
        assert_eq!(vertex(&e, shape, 0), [0.0, 0.0, -7.0]);
        assert_eq!(vertex(&e, shape, 4), [10.0, 0.0, -7.0]);
        let property = calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY)[0][1];
        assert_eq!(floats(&e, property + 0x68, 4), vec![0.75, 0.5, 0.25, 1.0]);
        assert_eq!(calls(&e, NI_AV_OBJECT_UPDATE).len(), 1);

        // No lines are drawn without a line count.
        e.set(info, ManagerTextData::iLines, 0);
        let out = out_node(&mut e);
        font_manager_text_doc_render(&mut e, doc, out, info.addr());
        assert_eq!(calls(&e, 0x7000_00dc).len(), 2);
    }

    #[test]
    fn doc_add_char_starts_pages() {
        let (mut e, _, _) = text_setup();
        let doc: Ptr<TextDoc> = e.new_object();
        e.set(doc, TextDoc::iPageWidth, 100);
        e.set(doc, TextDoc::iPageHeight, 100);
        let a = char_data(&mut e, 0, b'a');
        font_manager_text_doc_add_char(&mut e, doc, a, 0, 0);
        assert_eq!(e.mem.u32(doc.addr() + 8), 1);
        let b = char_data(&mut e, 0, b'b');
        font_manager_text_doc_add_char(&mut e, doc, b, 0, 0);
        assert_eq!(e.mem.u32(doc.addr() + 8), 1);
        let c = char_data(&mut e, 0, b'c');
        font_manager_text_doc_add_char(&mut e, doc, c, 0, 1);
        assert_eq!(e.mem.u32(doc.addr() + 8), 2);
        let pages = list_elements(&e, doc.addr());
        assert_eq!(e.mem.u32(pages[1] + 0x40), doc.addr());
    }

    #[test]
    fn char_count_skips_images_and_other_fonts() {
        let (mut e, _, _) = text_setup();
        let page: Ptr<TextPage> = e.new_object();
        let line = e.mem.alloc(0x30);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, line);
        list_add_tail_double(&mut e, &[page.addr(), slot]);
        let image = char_data(&mut e, 0, b'i');
        let file = cstr(&mut e, b"x.dds");
        e.mem.set_u32(image + 0x1c, file);
        for data in [
            char_data(&mut e, 0, b'a'),
            char_data(&mut e, 1, b'b'),
            image,
        ] {
            e.mem.set_u32(slot, data);
            list_add_tail_double(&mut e, &[line, slot]);
        }
        assert_eq!(
            font_manager_text_page_get_char_count_for_font(&mut e, page, 0),
            1
        );
        assert_eq!(
            font_manager_text_page_get_char_count_for_font(&mut e, page, 1),
            1
        );
        assert_eq!(
            font_manager_text_page_get_char_count_for_font(&mut e, page, 2),
            0
        );
    }

    #[test]
    fn page_add_char_fills_lines_and_overflows_to_a_new_page() {
        let (mut e, _, _) = text_setup();
        let doc: Ptr<TextDoc> = e.new_object();
        let page: Ptr<TextPage> = e.new_object();
        let a = char_data(&mut e, 0, b'a');
        fn_00a1bc70(&mut e, page, doc, a, 100, 60);
        let b = char_data(&mut e, 0, b'b');
        assert_eq!(fn_00a19c00(&mut e, page, b, 0), 0);
        assert_eq!(e.mem.u32(page.addr() + 8), 1);
        assert_eq!(e.mem.i32(page.addr() + 0x20), 2);
        assert_eq!(e.get(page, TextPage::iLastFontHeight), 20);
        assert_eq!(e.get(page, TextPage::iWidth), 20);

        // Two lines down: one last font height of space.
        let c = char_data(&mut e, 0, b'c');
        assert_eq!(fn_00a19c00(&mut e, page, c, 2), 0);
        let lines = list_elements(&e, page.addr());
        assert_eq!(lines.len(), 2);
        assert_eq!(e.mem.i32(lines[1] + 0x1c), 20);
        assert_eq!(e.get(page, TextPage::iHeight), 9 + 9 + 20);

        // The next line does not fit the page height.
        let d = char_data(&mut e, 0, b'd');
        let new_page = fn_00a19c00(&mut e, page, d, 3);
        assert_ne!(new_page, 0);
        let moved = list_elements(&e, new_page)[0];
        assert_eq!(e.mem.i32(moved + 0x1c), 0);
        assert_eq!(e.mem.u32(moved + 0x2c), new_page);
        assert_eq!(e.mem.i32(new_page + 0x20), 1);
    }

    fn line_on_page(e: &mut Engine, width: i32) -> (Ptr<TextPage>, Ptr<TextLine>) {
        let page: Ptr<TextPage> = e.new_object();
        let line: Ptr<TextLine> = e.new_object();
        e.set(line, TextLine::iPageWidth, width);
        e.set(line, TextLine::pPage, page.addr());
        (page, line)
    }

    #[test]
    fn line_add_char_lays_out_and_wraps_words() {
        let (mut e, _, _) = text_setup();
        let (page, line) = line_on_page(&mut e, 35);
        let chars: Vec<u32> = b"a b".iter().map(|c| char_data(&mut e, 0, *c)).collect();
        for c in &chars {
            assert_eq!(fn_00a19f70(&mut e, line, *c, 0), 0);
        }
        assert_eq!(e.mem.i32(chars[2] + 0x34), 20);
        assert_eq!(e.get(line, TextLine::iWidth), 30);
        assert_eq!(e.get(line, TextLine::iRise), 9);
        assert_eq!(e.get(line, TextLine::iDrop), 0);
        assert_eq!(e.get(page, TextPage::iWidth), 30);

        let c = char_data(&mut e, 0, b'c');
        let new_line = fn_00a19f70(&mut e, line, c, 0);
        assert_ne!(new_line, 0);
        assert_eq!(list_elements(&e, line.addr()), vec![chars[0]]);
        assert_eq!(e.get(line, TextLine::iWidth), 20);
        assert_eq!(list_elements(&e, new_line), vec![chars[2], c]);
        assert_eq!(e.mem.i32(c + 0x34), 10);
        assert_eq!(e.mem.block_size(chars[1]), None);
    }

    #[test]
    fn line_add_char_hyphenates_without_a_space() {
        let (mut e, _, _) = text_setup();
        let (_, line) = line_on_page(&mut e, 25);
        for c in *b"ab" {
            let data = char_data(&mut e, 0, c);
            fn_00a19f70(&mut e, line, data, 0);
        }
        let c = char_data(&mut e, 0, b'c');
        let new_line = fn_00a19f70(&mut e, line, c, 0);
        assert_eq!(list_elements(&e, new_line), vec![c]);
        let kept = list_elements(&e, line.addr());
        assert_eq!(kept.len(), 3);
        assert_eq!(e.mem.u8(kept[2] + 4), b'-');

        // A space that does not fit starts the next line as character 0.
        let space = char_data(&mut e, 0, b' ');
        let next = fn_00a19f70(&mut e, line, space, 0);
        assert_eq!(list_elements(&e, next), vec![space]);
        assert_eq!(e.mem.u8(space + 4), 0);
    }

    #[test]
    fn make_image_maps_the_texture_onto_a_quad() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        geometry(&mut e);
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(1));
        e.register(BS_SHADER_MANAGER_GET_SHADER, |_, _| ret(0x4444));
        let texture = e.mem.alloc(8);
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(vtable + 0x94, 0x7000_0094);
        e.mem.set_u32(vtable + 0x98, 0x7000_0098);
        e.mem.set_u32(texture, vtable);
        e.register(0x7000_0094, |_, _| ret(64));
        e.register(0x7000_0098, |_, _| ret(32));
        let paths = Rc::new(RefCell::new(Vec::new()));
        let seen = paths.clone();
        e.register_double(TILE_IMAGE_ADD_TO_TES_TEXTURES, move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[0]));
            e.mem.set_f32(a[1], 2.0);
            e.mem.set_u32(a[2], texture);
            ret(0)
        });
        let name = cstr(&mut e, b"a/b.dds");
        let shape =
            font_manager_make_image(&mut e, Ptr::NULL, name, 7 | 7 << 16, 1.0, 2.0, 3.0, 16, 8);
        assert_eq!(*paths.borrow(), vec![b"a\\b.dds".to_vec()]);
        assert_eq!(vertex(&e, shape, 0), [1.0, 2.0, 11.0]);
        assert_eq!(vertex(&e, shape, 1), [1.0, 2.0, 3.0]);
        assert_eq!(vertex(&e, shape, 2), [17.0, 2.0, 11.0]);
        assert_eq!(vertex(&e, shape, 3), [17.0, 2.0, 3.0]);
        let data = e.mem.u32(shape + 0xb8);
        let uv = e.mem.u32(data + 0x2c);
        assert_eq!(
            floats(&e, uv, 8),
            vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.5, 0.5]
        );
        assert_eq!(e.mem.u32(shape + 0xc0), 0x4444);
        let property = calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY)[0][1];
        assert_eq!(
            calls(&e, TILE_SHADER_PROPERTY_SET_TILE_TEXTURE),
            vec![vec![property, texture]]
        );
        assert_eq!(calls(&e, BS_STRING_SET).last().unwrap()[1..], [0, 0]);
    }

    #[test]
    fn add_text_icon_stores_a_sized_icon() {
        let mut e = engine();
        icon_constants(&mut e);
        e.register(TILE_GET_TEXTURE_ATLAS_INFO, |e, a| {
            for (k, v) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * k as u32, *v);
            }
            ret(1)
        });
        e.register(BUTTON_ICONS_NEW_INDEX, |_, _| ret(1));
        e.register(BUTTON_ICONS_CONSTRUCT, nothing);
        let font: Ptr<Font> = e.new_object();
        e.set(font, Font::iFontNum, 2);
        e.set(font, Font::fFontHeight, 20.0);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(font.addr() + 0x48, buffer);
        let name = cstr(&mut e, b"xbox_a.dds");
        font_add_text_icon(&mut e, font, name);
        assert_eq!(
            floats(&e, buffer + 0x20, 8),
            vec![60.0, -15.0, 20.0, -30.0, 0.1, 0.2, 0.3, 0.4]
        );
        assert_eq!(
            calls(&e, BUTTON_ICONS_CONSTRUCT),
            vec![vec![font.addr() + 0x44, buffer + 0x20, 1]]
        );
        assert_eq!(
            calls(&e, TILE_GET_TEXTURE_ATLAS_INFO)[0][2],
            font.addr() + 0x3c
        );
    }

    #[test]
    fn string_dimensions_wrap_and_count_lines() {
        let (mut e, _, m) = text_setup();
        let result = e.mem.alloc(12);
        let text = cstr(&mut e, b"aaa a");
        font_manager_calculate_string_dimensions(&mut e, m, Ptr::new(result), text, 1, 25.0, 0);
        assert_eq!(floats(&e, result, 3), vec![20.0, 32.0, 3.0]);
        e.mem.set_f32(DIMENSIONS_START, 4.0);
        font_manager_calculate_string_dimensions(&mut e, m, Ptr::new(result), text, 9, 25.0, 0);
        assert_eq!(floats(&e, result, 3), vec![4.0, 0.0, 0.0]);
        font_manager_calculate_string_dimensions(&mut e, m, Ptr::new(result), 0, 1, 25.0, 0);
        assert_eq!(e.mem.f32(result), 4.0);
    }

    #[test]
    fn line_padding_per_font() {
        let mut e = engine();
        set_f32_at(&mut e, PADDING_FONTS_2_6, 4.0);
        set_f32_at(&mut e, PADDING_FONT_3, 6.0);
        set_f32_at(&mut e, PADDING_FONT_7, 2.0);
        let m: Ptr<FontManager> = Ptr::NULL;
        let got: Vec<f32> = (0..9)
            .map(|f| font_manager_get_line_padding(&mut e, m, f))
            .collect();
        assert_eq!(got, vec![0.0, 0.0, 4.0, 6.0, 0.0, 0.0, 4.0, 2.0, 0.0]);
    }

    #[test]
    fn pointer_constructor_clears_the_pointer() {
        let mut e = engine();
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, 0x1234);
        fn_00a1b410(&mut e, Ptr::new(p));
        assert_eq!(e.mem.u32(p), 0);
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn char_data_constructor_takes_the_letter_metrics() {
        let (mut e, f, _) = text_setup();
        let data = e.get(f, Font::pFontData);
        set_letter(&mut e, data, b'a', [10.0, 12.0, 1.0, 2.0, 2.0]);
        let c: Ptr<CharData> = e.new_object();
        let r = font_manager_char_data_char_data(&mut e, c, -3, b'a', 1, 2, 3, 4, 5);
        assert_eq!(r, c);
        assert_eq!(e.get(c, CharData::iFontIndex), 0);
        assert_eq!(e.get(c, CharData::cChar), b'a');
        let words: Vec<u32> = (0..4).map(|k| e.mem.u32(c.addr() + 8 + 4 * k)).collect();
        assert_eq!(words, vec![1, 2, 3, 4]);
        assert_eq!(e.get(c, CharData::iJustification), 5);
        assert_eq!(e.get(c, CharData::iWidth), 3);
        assert_eq!(e.get(c, CharData::iRise), 10);
        assert_eq!(e.get(c, CharData::iDrop), 10);
        // A letter without width has no width.
        set_letter(&mut e, data, b'b', [0.0, 12.0, 1.0, 2.0, 2.0]);
        font_manager_char_data_char_data(&mut e, c, 3, b'b', 0, 0, 0, 0, 1);
        assert_eq!(e.get(c, CharData::iFontIndex), 3);
        assert_eq!(e.get(c, CharData::iWidth), 0);
    }

    #[test]
    fn char_data_copy_keeps_the_metrics_and_file() {
        let (mut e, _, _) = text_setup();
        let source: Ptr<CharData> = Ptr::new(char_data(&mut e, 2, b'q'));
        let file = cstr(&mut e, b"x.dds");
        e.mem.set_u32(source.addr() + 0x1c, file);
        e.set(source, CharData::iWidth, 31);
        e.set(source, CharData::iX, 7);
        e.set(source, CharData::iLeadingEdge, 4);
        let copy = font_manager_char_data_copy(&mut e, source);
        assert_ne!(copy, source);
        assert_eq!(e.get(copy, CharData::iFontIndex), 2);
        assert_eq!(e.get(copy, CharData::cChar), b'q');
        assert_eq!(e.get(copy, CharData::iWidth), 31);
        assert_eq!(e.get(copy, CharData::iX), 7);
        assert_eq!(e.get(copy, CharData::iLeadingEdge), 0);
        assert_eq!(e.mem.cstr(e.mem.u32(copy.addr() + 0x1c)), b"x.dds");
    }

    #[test]
    fn revert_to_default_resets_font_color_and_character() {
        let (mut e, _, _) = text_setup();
        let c: Ptr<CharData> = Ptr::new(char_data(&mut e, 3, b'q'));
        e.set(c, CharData::iJustification, 4);
        font_manager_char_data_revert_to_default(&mut e, c);
        assert_eq!(e.get(c, CharData::iFontIndex), 0);
        assert_eq!(floats(&e, c.addr() + 8, 4), vec![0.5, 0.25, 0.125, 1.0]);
        assert_eq!(e.get(c, CharData::iJustification), 1);
        assert_eq!(e.get(c, CharData::cChar), b' ');
        assert_eq!(e.get(c, CharData::iWidth), 10);
    }

    #[test]
    fn set_char_updates_text_metrics_only() {
        let (mut e, f, _) = text_setup();
        e.set(f, Font::fMaxDrop, -5.0);
        let data = e.get(f, Font::pFontData);
        set_letter(&mut e, data, b'w', [8.0, 12.0, 1.0, 2.0, 2.0]);
        let c: Ptr<CharData> = e.new_object();
        e.set(c, CharData::iX, 9);
        font_manager_char_data_set_char(&mut e, c, b'w');
        assert_eq!(e.get(c, CharData::iWidth), 11);
        assert_eq!(e.get(c, CharData::iRise), 10);
        assert_eq!(e.get(c, CharData::iDrop), 5);
        assert_eq!(e.get(c, CharData::iX), 0);
        let file = cstr(&mut e, b"x.dds");
        e.mem.set_u32(c.addr() + 0x1c, file);
        e.set(c, CharData::iX, 9);
        font_manager_char_data_set_char(&mut e, c, b'a');
        assert_eq!(e.get(c, CharData::cChar), b'a');
        assert_eq!(e.get(c, CharData::iWidth), 11);
        assert_eq!(e.get(c, CharData::iX), 0);
    }

    #[test]
    fn page_list_constructor_empties_the_list() {
        let mut e = engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        for k in 0..3 {
            e.mem.set_u32(list.addr() + 4 * k, 5);
        }
        assert_eq!(
            ni_tpointer_list_text_page_ni_tpointer_list(&mut e, list),
            list
        );
        assert_eq!(e.mem.bytes(list.addr(), 12), vec![0; 12]);
    }

    #[test]
    fn char_data_destructor_empties_the_file_name() {
        let mut e = engine();
        let c: Ptr<CharData> = e.new_object();
        fn_00a1b970(&mut e, c);
        assert_eq!(calls(&e, BS_STRING_SET), vec![vec![c.addr() + 0x1c, 0, 0]]);
    }

    /// A document with one page holding one line of two characters.
    fn small_doc(e: &mut Engine) -> (u32, u32, u32, [u32; 2]) {
        let doc = e.mem.alloc(0x18);
        let page = e.mem.alloc(0x44);
        let line = e.mem.alloc(0x30);
        let chars = [e.mem.alloc(0x38), e.mem.alloc(0x38)];
        let slot = e.mem.alloc(4);
        for (list, item) in [
            (doc, page),
            (page, line),
            (line, chars[0]),
            (line, chars[1]),
        ] {
            e.mem.set_u32(slot, item);
            list_add_tail_double(e, &[list, slot]);
        }
        (doc, page, line, chars)
    }

    #[test]
    fn doc_destructor_deletes_pages_lines_and_characters() {
        let mut e = engine();
        let (doc, page, line, chars) = small_doc(&mut e);
        fn_00a1b990(&mut e, Ptr::new(doc));
        for block in [page, line, chars[0], chars[1]] {
            assert_eq!(e.mem.block_size(block), None);
        }
        assert_eq!(e.mem.u32(doc + 8), 0);
    }

    #[test]
    fn page_destructor_deletes_its_lines() {
        let mut e = engine();
        let (_, page, line, chars) = small_doc(&mut e);
        fn_00a1ba50(&mut e, Ptr::new(page));
        assert_eq!(e.mem.block_size(line), None);
        assert_eq!(e.mem.block_size(chars[1]), None);
        assert_eq!(e.mem.u32(page + 8), 0);
    }

    #[test]
    fn line_destructor_deletes_its_characters() {
        let mut e = engine();
        let (_, _, line, chars) = small_doc(&mut e);
        fn_00a1bb10(&mut e, Ptr::new(line));
        assert_eq!(e.mem.block_size(chars[0]), None);
        assert_eq!(calls(&e, BS_STRING_SET).len(), 2);
        assert_eq!(calls(&e, POINTER_LIST_DESTRUCTOR), vec![vec![line]]);
    }

    #[test]
    fn list_destructor_wrappers_call_the_list_code() {
        let mut e = engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        fn_00a1bbd0(&mut e, list);
        fn_00a1bc20(&mut e, list);
        assert_eq!(calls(&e, POINTER_LIST_DESTRUCTOR), vec![vec![list.addr()]]);
        assert_eq!(calls(&e, POINTER_LIST_REMOVE_ALL), vec![vec![list.addr()]]);
    }

    #[test]
    fn page_constructor_starts_with_one_line() {
        let (mut e, _, _) = text_setup();
        let doc: Ptr<TextDoc> = e.new_object();
        let page: Ptr<TextPage> = e.new_object();
        let a = char_data(&mut e, 0, b'a');
        assert_eq!(fn_00a1bc70(&mut e, page, doc, a, 100, 200), page);
        assert_eq!(e.get(page, TextPage::iPageWidth), 100);
        assert_eq!(e.get(page, TextPage::iPageHeight), 200);
        assert_eq!(e.get(page, TextPage::pDoc), doc.addr());
        assert_eq!(e.get(page, TextPage::iLastFontHeight), 20);
        assert_eq!(e.get(page, TextPage::iHeight), 9);
        let line = list_elements(&e, page.addr())[0];
        assert_eq!(list_elements(&e, line), vec![a]);
    }

    #[test]
    fn line_constructor_takes_the_justification_and_first_character() {
        let (mut e, _, _) = text_setup();
        let (page, _) = line_on_page(&mut e, 0);
        let line: Ptr<TextLine> = e.new_object();
        let a = char_data(&mut e, 0, b'a');
        e.mem.set_i32(a + 0x18, 4);
        assert_eq!(fn_00a1bd40(&mut e, line, page, a, 3, 100), line);
        assert_eq!(e.get(line, TextLine::iJustify), 4);
        assert_eq!(e.get(line, TextLine::iSkippedSpace), 3);
        assert_eq!(e.get(line, TextLine::iPageWidth), 100);
        assert_eq!(e.get(line, TextLine::pPage), page.addr());
        assert_eq!(e.get(line, TextLine::iWidth), 10);
        assert_eq!(list_elements(&e, line.addr()), vec![a]);
    }

    #[test]
    fn page_from_line_takes_the_line() {
        let mut e = engine();
        let doc: Ptr<TextDoc> = e.new_object();
        let page: Ptr<TextPage> = e.new_object();
        let line: Ptr<TextLine> = e.new_object();
        e.set(line, TextLine::iRise, 9);
        e.set(line, TextLine::iDrop, 4);
        assert_eq!(fn_00a1be10(&mut e, page, doc, line, 50, 60), page);
        assert_eq!(e.get(page, TextPage::iHeight), 13);
        assert_eq!(e.get(page, TextPage::iLastFontHeight), 0x23);
        assert_eq!(e.get(line, TextLine::pPage), page.addr());
        assert_eq!(list_elements(&e, page.addr()), vec![line.addr()]);
    }

    #[test]
    fn button_icon_array_lifecycle() {
        let mut e = engine();
        const FREE: u32 = 0x7000_0108;
        let array: Ptr<BSSimpleArray> = e.new_object();
        assert_eq!(fn_00a1bef0(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), BUTTON_ICON_ARRAY_VTABLE);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, FREE);
        e.mem.set_u32(array.addr(), vtable);
        e.register(FREE, nothing);
        e.set(array, BSSimpleArray::pBuffer, 0x500);
        e.set(array, BSSimpleArray::iSize, 3);
        e.set(array, BSSimpleArray::iReservedSize, 4);
        bs_simple_array_button_icon_clear(&mut e, array, 0);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x500);
        bs_simple_array_button_icon_clear(&mut e, array, 1);
        assert_eq!(calls(&e, FREE), vec![vec![array.addr(), 0x500]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn button_icon_array_destructors() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        bs_simple_array_button_icon_destructor(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), BUTTON_ICON_ARRAY_VTABLE);
        assert_eq!(
            bs_simple_array_button_icon_scalar_deleting_destructor(&mut e, array, 0),
            array
        );
        assert!(e.mem.block_size(array.addr()).is_some());
        bs_simple_array_button_icon_scalar_deleting_destructor(&mut e, array, 1);
        assert_eq!(e.mem.block_size(array.addr()), None);
    }

    #[test]
    fn button_icon_array_allocation() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = Ptr::NULL;
        let block = bs_simple_array_button_icon_allocate(&mut e, array, 3);
        assert_eq!(e.mem.block_size(block), Some(0x60));
        e.mem.set_u32(block, 0xabc);
        let grown = bs_simple_array_button_icon_reallocate(&mut e, array, block, 5);
        assert_eq!(e.mem.block_size(grown), Some(0xa0));
        assert_eq!(e.mem.u32(grown), 0xabc);
    }

    #[test]
    fn remove_head_unlinks_and_returns_the_element() {
        let mut e = engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        let slot = e.mem.alloc(4);
        for item in [0x11u32, 0x22] {
            e.mem.set_u32(slot, item);
            list_add_tail_double(&mut e, &[list.addr(), slot]);
        }
        let first = e.get(list, NiTPointerList::m_pkHead);
        assert_eq!(fn_00a1c080(&mut e, list), 0x11);
        assert_eq!(e.mem.block_size(first), None);
        let second = e.get(list, NiTPointerList::m_pkHead);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        assert_eq!(fn_00a1c080(&mut e, list), 0x22);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn run_initializer(addr: u32, setting: u32, name: u32, value: u32, destructor: u32) {
        let mut e = Engine::new();
        e.map(setting, 0x0c);
        e.map(SETTING_COLLECTION, 4);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x00aa_0004);
        let collection = e.mem.alloc(8);
        e.mem.set_u32(collection, vtable);
        e.set_global::<u32>(SETTING_COLLECTION, collection);
        e.register(SETTING_COLLECTION_GETTER, |_, _| Ret::default());
        e.register(0x00aa_0004, |e, a| {
            // The setting is complete when it is added.
            assert_eq!(e.mem.u32(a[1]), INI_SETTING_VTABLE);
            Ret::default()
        });
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(addr, &args![]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        assert_eq!(
            log,
            vec![
                (SETTING_COLLECTION_GETTER, vec![]),
                (0x00aa_0004, vec![collection, setting]),
                (ATEXIT, vec![destructor]),
            ]
        );
        assert_eq!(e.mem.u32(setting), INI_SETTING_VTABLE);
        assert_eq!(e.mem.u32(setting + 4), value);
        assert_eq!(e.mem.u32(setting + 8), name);
    }

    #[test]
    fn font_1_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb37f0,
            0x011f_3414,
            0x0109_52b0,
            0x0109_528c,
            0x00fd_a3c0,
        );
    }

    #[test]
    fn font_2_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3890,
            0x011f_3420,
            0x0109_52e4,
            0x0109_52c4,
            0x00fd_a440,
        );
    }

    #[test]
    fn font_3_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3930,
            0x011f_342c,
            0x0109_5320,
            0x0109_52f8,
            0x00fd_a4c0,
        );
    }

    #[test]
    fn font_4_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb39d0,
            0x011f_3438,
            0x0109_5364,
            0x0109_5334,
            0x00fd_a540,
        );
    }

    #[test]
    fn font_5_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3a70,
            0x011f_3444,
            0x0109_53a4,
            0x0109_5378,
            0x00fd_a5c0,
        );
    }

    #[test]
    fn font_6_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3b10,
            0x011f_3450,
            0x0109_53e4,
            0x0109_53b8,
            0x00fd_a640,
        );
    }

    #[test]
    fn font_7_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3bb0,
            0x011f_345c,
            0x0109_5420,
            0x0109_53f8,
            0x00fd_a6c0,
        );
    }

    #[test]
    fn font_8_filename_initializer_builds_and_registers_the_setting() {
        run_initializer(
            0x00fb3c50,
            0x011f_3468,
            0x0109_545c,
            0x0109_5434,
            0x00fd_a740,
        );
    }

    /// All eight `sFontFile_N:Fonts` initializers (`00fb37f0` + 0xA0 * n) are registered,
    /// so the start-up table runs each one and `FontManager::FontManager` finds every
    /// font's file name (fonts 2 to 7 were missing: font 7's data stayed null).
    #[test]
    fn all_eight_font_filename_initializers_are_registered() {
        let table = funcs();
        for n in 0..8u32 {
            let addr = 0x00fb_37f0 + 0xa0 * n;
            assert!(
                table.iter().any(|(a, _)| *a == addr),
                "initializer {addr:08x} missing"
            );
        }
    }
}

#[cfg(test)]
mod n083_tests {
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
    fn append_slot_grows_a_full_array() {
        let mut e = Engine::new();
        let a = e.new_object::<BSSimpleArray>();
        e.put_vtable(0x0300_0000, &[0, 0x0300_1004]);
        e.mem.set_u32(a.addr(), 0x0300_0000);
        e.register(0x0300_1004, |_, _| Ret {
            eax: 0x0500_0000,
            ..Ret::default()
        });
        e.register(ITEM_ARRAY_CHANGE_BUFFER_00A1C340, |_, _| Ret::default());
        // Empty: four items allocated.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a1_c130, &args![a]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0300_1004), [vec![a.addr(), 4]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0x0500_0000);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 1);
        // Room left: only the size grows.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a1_c130, &args![a]).u32(), 1);
        assert!(calls_to(&e, ITEM_ARRAY_CHANGE_BUFFER_00A1C340).is_empty());
        // Full at 0x400: doubles.
        e.set(a, BSSimpleArray::iSize, 0x400);
        e.set(a, BSSimpleArray::iReservedSize, 0x400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a1_c130, &args![a]).u32(), 0x400);
        assert_eq!(
            calls_to(&e, ITEM_ARRAY_CHANGE_BUFFER_00A1C340),
            [vec![a.addr(), 0x800, 0x400]]
        );
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 0x800);
        // Full above 0x400: grows by 0x400.
        e.set(a, BSSimpleArray::iSize, 0x800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a1_c130, &args![a]).u32(), 0x800);
        assert_eq!(
            calls_to(&e, ITEM_ARRAY_CHANGE_BUFFER_00A1C340),
            [vec![a.addr(), 0xc00, 0x800]]
        );
        assert_eq!(e.get(a, BSSimpleArray::iSize), 0x801);
    }

    #[test]
    fn item_construction_clears_the_four_floats() {
        let mut e = Engine::new();
        let items = e.mem.alloc(0x60);
        for off in (0..0x60).step_by(4) {
            e.mem.set_u32(items + off, 0xffff_ffff);
        }
        e.call(0x00a1_c1f0, &args![0u32, items, 2u32]);
        for item in [items, items + 0x20] {
            for off in (0..0x10).step_by(4) {
                assert_eq!(e.mem.u32(item + off), 0xffff_ffff);
            }
            for off in (0x10..0x20).step_by(4) {
                assert_eq!(e.mem.u32(item + off), 0);
            }
        }
        assert_eq!(e.mem.u32(items + 0x50), 0xffff_ffff);
    }

    #[test]
    fn add_node_head_links_the_node_in_front() {
        let mut e = Engine::new();
        let list = e.new_object::<NiTPointerList>();
        let first = e.new_object::<NiTListItem>();
        let second = e.new_object::<NiTListItem>();
        e.set(first, NiTListItem::m_pkPrev, 0x55);
        e.call(0x00a1_c270, &args![list, first]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), first.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), first.addr());
        assert_eq!(e.get(first, NiTListItem::m_pkPrev), 0);
        assert_eq!(e.get(first, NiTListItem::m_pkNext), 0);
        e.call(0x00a1_c270, &args![list, second]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), second.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), first.addr());
        assert_eq!(e.get(second, NiTListItem::m_pkNext), first.addr());
        assert_eq!(e.get(first, NiTListItem::m_pkPrev), second.addr());
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
    }
}
