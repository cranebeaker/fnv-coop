//! `BSMenu/tiletext.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TileText` is the text tile: its model is an `NiNode` holding the text
//! geometry that `Font::CreateText` builds, or (for hypertext, or when the
//! font manager uses the new fonts) what `FontManager::CreateText` renders.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::font::{
    Font, FontManager, ManagerTextData, BS_STRING_ASSIGN, NI_AV_OBJECT_UPDATE_PROPERTIES, NI_NEW,
};
use crate::units::bsmenu::tile::{
    Tile, TRAIT_ALPHA, TRAIT_BLUE, TRAIT_BRIGHTNESS, TRAIT_CLIPS, TRAIT_DEPTH, TRAIT_FONT,
    TRAIT_GREEN, TRAIT_HEIGHT, TRAIT_IS_HTML, TRAIT_JUSTIFY, TRAIT_LINE_GAP, TRAIT_PAGE_NUM,
    TRAIT_RED, TRAIT_STRING, TRAIT_VISIBLE, TRAIT_WIDTH, TRAIT_WRAP_LIMIT, TRAIT_WRAP_LINES,
    TRAIT_WRAP_WIDTH, TRAIT_X, TRAIT_Y,
};

layout! {
    /// `TileText` (Xbox PDB), 0x4C bytes on PC as on the Xbox.
    pub struct TileText: 0x4c {
        /// `fTextWidth` (Xbox PDB).
        0x38 fTextWidth: f32,
        /// `xOffset` (Xbox PDB): `NiPoint3`.
        0x3C xOffset: f32,
        /// `bIsHyper` (Xbox PDB).
        0x48 bIsHyper: bool,
    }
}

/// `Tile::Init` (`009ff600`, Xbox PDB name).
const TILE_INIT: u32 = 0x009f_f600;
/// Sets a float trait (`00a012d0`) and a string trait (`00a01350`)
/// (thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::SetNeedsUpdate` (`00a074d0`) and `Tile::AddNeedsUpdate`
/// (`00a07530`, Xbox PDB names).
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::GetParentModel` (`00a089c0`), `Tile::IsTrue` (`00a01230`),
/// `Tile::GetString` (`00a011f0`) and the float value of a trait
/// (`00a011b0`, `ST0`).
const TILE_GET_PARENT_MODEL: u32 = 0x00a0_89c0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::Extra::Extra` (`00a1f610`, Xbox PDB name, in `tilerect.cpp`).
const TILE_EXTRA_CONSTRUCTOR: u32 = 0x00a1_f610;
/// `Font::CreateText` (`00a12880`) and `FontManager::CreateText`
/// (`00a18f00`) (Xbox PDB names).
const FONT_CREATE_TEXT: u32 = 0x00a1_2880;
const FONT_MANAGER_CREATE_TEXT: u32 = 0x00a1_8f00;
/// `FontManager`'s instance (`011f33f8`).
const FONT_MANAGER: u32 = 0x011f_33f8;
/// `BSStringT<char>`: construction from a C string (`0040c0e0`, thiscall)
/// and `Set(text, length)` (`004037f0`).
const BS_STRING_CONSTRUCT: u32 = 0x0040_c0e0;
const BS_STRING_SET: u32 = 0x0040_37f0;
/// The `NiNode` constructor (`00a5ecb0`) and the `NiPointer` assignment
/// (`0066b0d0`).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiObjectNET::AddExtraData` (`00a5bca0`, Xbox PDB name `_ov2`).
const NI_OBJECT_NET_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `Interface::GetInterfaceRoot` (`00706d30`), `GetScreenWidth`
/// (`00706e00`) and `SetNeedToUpdate` (`00707930`) (Xbox PDB names,
/// cdecl).
const INTERFACE_GET_INTERFACE_ROOT: u32 = 0x0070_6d30;
const INTERFACE_GET_SCREEN_WIDTH: u32 = 0x0070_6e00;
const INTERFACE_SET_NEED_TO_UPDATE: u32 = 0x0070_7930;
/// `strcmp` (`00ec6da0`, cdecl).
const STRCMP: u32 = 0x00ec_6da0;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// The `NiNode` virtual slot that attaches a child (`+0xDC`, second word 1).
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;

/// `255.0f` (`01023cd8`): `Init`'s brightness, colour and alpha.
const FULL: u32 = 0x0102_3cd8;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `" "` (`01020770`): the text string's start value.
const SPACE: u32 = 0x0102_0770;
/// `0.0` (`01012060`), `255.0` (`0101e568`) and `-0.008` (`0106f290`)
/// (`double`): the depth step of the text geometry.
const ZERO: u32 = 0x0101_2060;
const ALPHA_SCALE: u32 = 0x0101_e568;
const DEPTH_STEP: u32 = 0x0106_f290;
/// The three `float`s the font manager's parameter block starts its colour
/// from (`01095268`, `01095264`, `01095260`); the code copies them into
/// locals it never reads again.
const DEFAULT_COLOR: [u32; 3] = [0x0109_5268, 0x0109_5264, 0x0109_5260];
/// Trait `0xFD5`: the number of pages the font manager laid out (no name
/// in `crate::units::bsmenu::tile`).
const TRAIT_PAGE_COUNT: i32 = 0xfd5;
/// The line separator (`'\n'`).
const LINE_SEPARATOR: u32 = 10;

/// `_ftol2_sse` on the value in `ST0`.
fn truncate(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// A trait's float value in `ST0`.
fn trait_value(e: &mut Engine, this: Ptr<TileText>, trait_id: i32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![this, trait_id]).f64()
}

// Translated from 00a21a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::Init` (Xbox PDB): `Tile::Init` with the same three words, an
/// empty string trait, brightness, red, green, blue and alpha the float at
/// `01023cd8`, visible 1 (each with flag word 1), and the update bits 3.
pub fn tile_text_init(
    e: &mut Engine,
    this: Ptr<TileText>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.call(TILE_INIT, &args![this, parent, name, set_parent_arg]);
    e.call(
        TILE_SET_STRING,
        &args![this, TRAIT_STRING, EMPTY_STRING, 1u32],
    );
    for trait_id in [
        TRAIT_BRIGHTNESS,
        TRAIT_RED,
        TRAIT_GREEN,
        TRAIT_BLUE,
        TRAIT_ALPHA,
    ] {
        let full = x87_quiet_bits(e.global::<u32>(FULL));
        e.call(TILE_SET_FLOAT, &args![this, trait_id, full, 1u32]);
    }
    e.call(TILE_SET_FLOAT, &args![this, TRAIT_VISIBLE, 1.0f32, 1u32]);
    e.call(TILE_SET_NEEDS_UPDATE, &args![this, 3u32]);
}

/// Places a text shape at `(x, depth * -0.008, y)` (its local translation,
/// `NiAVObject` +0x58) and attaches it to the tile's model.
fn place_text_shape(e: &mut Engine, this: Ptr<TileText>, shape: u32, x: u32, y: u32) {
    let depth = trait_value(e, this, TRAIT_DEPTH);
    let z = x87_store32(x87_mul(depth, e.global::<f64>(DEPTH_STEP)));
    let translation = [x87_quiet_bits(x), x87_quiet_bits(z), x87_quiet_bits(y)];
    for (i, word) in translation.iter().enumerate() {
        e.mem.set_u32(shape + 0x58 + 4 * i as u32, *word);
    }
    let model = e.get(this.cast::<Tile>(), Tile::spModel);
    e.vcall(model.addr(), NODE_ATTACH_CHILD_SLOT, &args![shape, 1u32]);
}

/// The `0 .. 0xFFFF` clamp the inlined `BSStringT` constructor applies to
/// its zero lengths (the `0xFFFF` branch never runs for 0).
fn clamped_length(length: u32) -> u16 {
    if length > 0xffff {
        0xffff
    } else {
        length as u16
    }
}

// Translated from 00a21af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::MakeNode` (Xbox PDB): reads x, y (negated), wrap width,
/// wrap limit, wrap lines, line gap, justification, font (at least 1),
/// page, hypertext (`> 0`), alpha and brightness. Without the font
/// (`FontManager` font `font - 1`) or its font data, returns 0. Otherwise
/// starts the text as `" "` replaced by the string trait when that is not
/// empty, takes the screen width for a wrap width below 1 and no limit for
/// wrap lines below 1, and raises the font's base line (font data +0) by
/// the line gap while the text is built. The colour is white with alpha
/// `alpha / 255` (0 for alpha 0).
///
/// * Hypertext, or a font manager with the new fonts (+0x20): the model is
///   a new `NiNode`, and `FontManager::CreateText` renders the text into it
///   from a `FontManager::TextData` block (font index, justification,
///   colour, line separator `'\n'`, width and height (0 means none),
///   page, lines, hypertext); when that fails the function returns 0
///   (without restoring the base line). Otherwise the laid-out width and
///   height are kept, the page count becomes trait `0xFD5` and the block's
///   hypertext flag `bIsHyper`.
/// * Otherwise: the model is a new `NiNode`, `Font::CreateText` builds the
///   text and icon shapes (width and height through pointers); `xOffset`
///   takes the text shape's translation, and each shape is placed at
///   `(x, depth * -0.008, y)` and attached to the model.
///
/// Then the base line is restored, the width and height traits set to the
/// width and height, the updates 9 (and the scissor update `0x100` for a
/// clipping tile) queued, the model attached to the nearest ancestor's
/// model or the interface root, its flags word's bit 0 set for a hidden
/// tile and cleared otherwise, its properties updated, the interface told
/// it needs an update and the tile's `Tile::Extra` added. Returns the
/// model.
// Translation incomplete: 00a21af0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_text_make_node(e: &mut Engine, this: Ptr<TileText>) -> Ptr {
    let tile = this.cast::<Tile>();
    let mut parent = e.call(TILE_GET_PARENT_MODEL, &args![this]).u32();
    let x = x87_store32(trait_value(e, this, TRAIT_X));
    let y = x87_store32(-trait_value(e, this, TRAIT_Y));
    let value = trait_value(e, this, TRAIT_WRAP_WIDTH);
    let wrap_width = truncate(e, value);
    let value = trait_value(e, this, TRAIT_WRAP_LIMIT);
    let wrap_limit = truncate(e, value);
    let value = trait_value(e, this, TRAIT_WRAP_LINES);
    let mut wrap_lines = truncate(e, value);
    let value = trait_value(e, this, TRAIT_LINE_GAP);
    let line_gap = truncate(e, value);
    let value = trait_value(e, this, TRAIT_JUSTIFY);
    let justify = truncate(e, value);
    let value = trait_value(e, this, TRAIT_FONT);
    let font = truncate(e, value);
    let font_index = (if font > 1 { font } else { 1 }) - 1;
    let manager = Ptr::<FontManager>::new(e.global::<u32>(FONT_MANAGER));
    let font_object = if manager.is_null() {
        Ptr::<Font>::NULL
    } else {
        Ptr::new(
            e.mem.u32(
                manager
                    .addr()
                    .wrapping_add((font_index as u32).wrapping_mul(4)),
            ),
        )
    };
    let value = trait_value(e, this, TRAIT_PAGE_NUM);
    let page = truncate(e, value);
    let is_html = trait_value(e, this, TRAIT_IS_HTML) > e.global::<f64>(ZERO);
    let alpha = x87_store32(trait_value(e, this, TRAIT_ALPHA));
    // The brightness is read into a local the code does not use.
    x87_store32(trait_value(e, this, TRAIT_BRIGHTNESS));
    let scaled_alpha = if x87_load32(alpha) == e.global::<f64>(ZERO) {
        0.0
    } else {
        x87_div(x87_load32(alpha), e.global::<f64>(ALPHA_SCALE))
    };
    let color = [
        1.0f32.to_bits(),
        1.0f32.to_bits(),
        1.0f32.to_bits(),
        x87_store32(scaled_alpha),
    ];
    if font_object.is_null() || e.get(font_object, Font::pFontData) == 0 {
        return Ptr::NULL;
    }

    // Stack: +0 the text, +8 the colour, +0x18 the wrap width, +0x1C the
    // wrap limit, +0x20 and +0x24 the shapes, +0x28 the font manager's
    // block (0x40 bytes), +0x68 the by-value string argument.
    e.with_stack(0x70, |e, locals| {
        let text = locals.addr();
        let color_at = text + 8;
        let width_at = text + 0x18;
        let limit_at = text + 0x1c;
        let shape_at = text + 0x20;
        let icons_at = text + 0x24;
        let block = Ptr::<ManagerTextData>::new(text + 0x28);
        let argument = text + 0x68;
        for (i, word) in color.iter().enumerate() {
            e.mem.set_u32(color_at + 4 * i as u32, *word);
        }
        e.mem.set_i32(width_at, wrap_width);
        e.mem.set_i32(limit_at, wrap_limit);

        e.call(BS_STRING_CONSTRUCT, &args![text, SPACE]);
        let string = e.call(TILE_GET_STRING, &args![this, TRAIT_STRING]).u32();
        if string != 0 && e.mem.i8(string) != 0 {
            e.call(BS_STRING_SET, &args![text, string, 0u32]);
        }
        if e.mem.i32(width_at) < 1 {
            let width = e.call(INTERFACE_GET_SCREEN_WIDTH, &args![]).f64();
            let width = truncate(e, width);
            e.mem.set_i32(width_at, width);
        }
        if wrap_lines < 1 {
            wrap_lines = 0x7fff_ffff;
        }
        // `FontData::fBaseLine` (font data +0).
        let data = e.get(font_object, Font::pFontData);
        let base_line = if data != 0 {
            x87_load32(e.mem.u32(data))
        } else {
            0.0
        };
        let base_line = truncate(e, base_line);
        let data = e.get(font_object, Font::pFontData);
        if data != 0 {
            let raised = x87_store32(f64::from(base_line.wrapping_add(line_gap)));
            e.mem.set_u32(data, raised);
        }

        let new_fonts = e.mem.u8(e.global::<u32>(FONT_MANAGER) + 0x20) != 0;
        if is_html || new_fonts {
            let node_block = e.call(NI_NEW, &args![0xacu32]).u32();
            let node = if node_block == 0 {
                0
            } else {
                e.call(NI_NODE_CONSTRUCTOR, &args![node_block, 0u32]).u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), node]);
            // The parameter block's inlined constructor.
            for address in DEFAULT_COLOR {
                x87_quiet_bits(e.global::<u32>(address));
            }
            let colour = block.addr() + ManagerTextData::xColor.off;
            for i in 0..4 {
                e.mem.set_f32(colour + 4 * i, 0.0);
            }
            let new_text = block.addr() + ManagerTextData::xNewText.off;
            e.mem.set_u32(new_text, 0);
            e.mem
                .set_u16(new_text + 4, clamped_length(std::hint::black_box(0)));
            e.mem
                .set_u16(new_text + 6, clamped_length(std::hint::black_box(0)));
            e.set(block, ManagerTextData::iDefaultFont, 0);
            e.set(block, ManagerTextData::iJustification, 1);
            e.set(block, ManagerTextData::cLineSep, LINE_SEPARATOR as u8);
            e.set(block, ManagerTextData::iWidth, 0x7fff_ffff);
            e.set(block, ManagerTextData::iHeight, 0x7fff_ffff);
            e.set(block, ManagerTextData::iPageNum, 0);
            e.set(block, ManagerTextData::iNumPages, 0);
            e.set(block, ManagerTextData::iNumLines, 0);
            e.set(block, ManagerTextData::bIsHypertext, 0);
            // The request.
            e.set(block, ManagerTextData::iDefaultFont, font_index);
            let width = e.mem.i32(width_at);
            let width = if width != 0 { width } else { 0x7fff_ffff };
            e.set(block, ManagerTextData::iWidth, width);
            e.set(block, ManagerTextData::iJustification, justify);
            for i in 0..4 {
                let word = e.mem.u32(color_at + 4 * i);
                e.mem.set_u32(colour + 4 * i, word);
            }
            let limit = e.mem.i32(limit_at);
            let limit = if limit != 0 { limit } else { 0x7fff_ffff };
            e.set(block, ManagerTextData::iHeight, limit);
            e.set(block, ManagerTextData::iPageNum, page);
            e.set(block, ManagerTextData::iLines, wrap_lines);
            e.set(block, ManagerTextData::bIsHypertext, is_html as u8);

            let model = e.get(tile, Tile::spModel);
            e.call(BS_STRING_ASSIGN, &args![argument, text]);
            let words = (e.mem.u32(argument), e.mem.u32(argument + 4));
            let manager = e.global::<u32>(FONT_MANAGER);
            let made = e
                .call(
                    FONT_MANAGER_CREATE_TEXT,
                    &args![manager, words.0, words.1, block, model],
                )
                .bool();
            if !made {
                e.call(BS_STRING_SET, &args![new_text, 0u32, 0u32]);
                e.call(BS_STRING_SET, &args![text, 0u32, 0u32]);
                return Ptr::NULL;
            }
            let width = e.get(block, ManagerTextData::iWidth);
            e.mem.set_i32(width_at, width);
            let height = e.get(block, ManagerTextData::iHeight);
            e.mem.set_i32(limit_at, height);
            let pages = e.get(block, ManagerTextData::iNumPages);
            e.call(
                TILE_SET_FLOAT,
                &args![this, TRAIT_PAGE_COUNT, x87_store32(f64::from(pages)), 1u32],
            );
            let hyper = e.get(block, ManagerTextData::bIsHypertext);
            e.mem.set_u8(this.addr() + TileText::bIsHyper.off, hyper);
            e.call(BS_STRING_SET, &args![new_text, 0u32, 0u32]);
        } else {
            e.mem.set_u32(shape_at, 0);
            e.mem.set_u32(icons_at, 0);
            let node_block = e.call(NI_NEW, &args![0xacu32]).u32();
            let node = if node_block == 0 {
                0
            } else {
                e.call(NI_NODE_CONSTRUCTOR, &args![node_block, 0u32]).u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), node]);
            e.call(
                FONT_CREATE_TEXT,
                &args![
                    font_object,
                    text,
                    width_at,
                    limit_at,
                    0u32,
                    wrap_lines,
                    justify,
                    LINE_SEPARATOR,
                    color_at,
                    shape_at,
                    icons_at
                ],
            );
            let shape = e.mem.u32(shape_at);
            for i in 0..3 {
                let word = e.mem.u32(shape + 0x58 + 4 * i);
                e.mem
                    .set_u32(this.addr() + TileText::xOffset.off + 4 * i, word);
            }
            place_text_shape(e, this, shape, x, y);
            let icons = e.mem.u32(icons_at);
            if icons != 0 {
                place_text_shape(e, this, icons, x, y);
            }
        }

        let data = e.get(font_object, Font::pFontData);
        if data != 0 {
            e.mem.set_u32(data, x87_store32(f64::from(base_line)));
        }
        let width = e.mem.i32(width_at);
        e.call(
            TILE_SET_FLOAT,
            &args![this, TRAIT_WIDTH, x87_store32(f64::from(width)), 1u32],
        );
        let height = e.mem.i32(limit_at);
        e.call(
            TILE_SET_FLOAT,
            &args![this, TRAIT_HEIGHT, x87_store32(f64::from(height)), 1u32],
        );
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 9u32]);
        if e.call(TILE_IS_TRUE, &args![this, TRAIT_CLIPS]).bool() {
            e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x100u32]);
        }
        if parent == 0 {
            parent = e.call(INTERFACE_GET_INTERFACE_ROOT, &args![]).u32();
        }
        let model = e.get(tile, Tile::spModel);
        e.vcall(parent, NODE_ATTACH_CHILD_SLOT, &args![model, 1u32]);
        // `NiAVObject` flags word (+0x30): bit 0 set while the tile is hidden.
        let model = e.get(tile, Tile::spModel);
        let visible = e.call(TILE_IS_TRUE, &args![this, TRAIT_VISIBLE]).bool();
        let flags = e.mem.u32(model.addr() + 0x30);
        if !visible {
            e.mem.set_u32(model.addr() + 0x30, flags | 1);
        } else {
            e.mem.set_u32(model.addr() + 0x30, flags & !1);
        }
        let model = e.get(tile, Tile::spModel);
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![model]);
        e.call(INTERFACE_SET_NEED_TO_UPDATE, &args![1u32]);
        let extra_block = e.call(NI_NEW, &args![0x14u32]).u32();
        let extra = if extra_block == 0 {
            0
        } else {
            let model = e.get(tile, Tile::spModel);
            e.call(TILE_EXTRA_CONSTRUCTOR, &args![extra_block, this, model])
                .u32()
        };
        let model = e.get(tile, Tile::spModel);
        e.call(NI_OBJECT_NET_ADD_EXTRA_DATA, &args![model, extra]);
        let model = e.get(tile, Tile::spModel);
        e.call(BS_STRING_SET, &args![text, 0u32, 0u32]);
        model
    })
}

// Translated from 00a225b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::PostParse` (Xbox PDB): for the string trait, queues the
/// create update (2) and returns `this` when the tile has no string or
/// `strcmp(previous, string)` is 0; for the justification, when `value`
/// differs from the current justification (NaN included). Returns 0
/// otherwise.
pub fn tile_text_post_parse(
    e: &mut Engine,
    this: Ptr<TileText>,
    trait_id: i32,
    value: f32,
    previous: Ptr,
) -> Ptr<TileText> {
    if trait_id == TRAIT_STRING {
        let string = e.call(TILE_GET_STRING, &args![this, TRAIT_STRING]).u32();
        if string == 0 || e.call(STRCMP, &args![previous, string]).u32() == 0 {
            e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 2u32]);
            return this;
        }
    } else if trait_id == TRAIT_JUSTIFY {
        let current = trait_value(e, this, TRAIT_JUSTIFY);
        if x87_load32(value.to_bits()) != current {
            e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 2u32]);
            return this;
        }
    }
    Ptr::NULL
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a2_1a10,
            tile_text_init(Ptr<TileText>, Ptr<Tile>, Ptr, u32)
        ),
        entry!(0x00a2_1af0, tile_text_make_node(Ptr<TileText>) -> Ptr),
        entry!(
            0x00a2_25b0,
            tile_text_post_parse(Ptr<TileText>, i32, f32, Ptr) -> Ptr<TileText>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ATTACH: u32 = 0x0000_1dc0;
    const NODE_VTABLE: u32 = 0x0003_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn tile_text_init_sets_the_default_traits() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TILE_INIT,
                TILE_SET_FLOAT,
                TILE_SET_STRING,
                TILE_SET_NEEDS_UPDATE,
            ],
        );
        e.map(FULL, 4);
        e.set_global(FULL, 255.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00a2_1a10, &args![0x5000u32, 1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, TILE_INIT), vec![vec![0x5000, 1, 2, 3]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5000, 0xfc4, EMPTY_STRING, 1]]
        );
        let full = 255.0f32.to_bits();
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x5000, 0xff5, full, 1],
                vec![0x5000, 0xfb2, full, 1],
                vec![0x5000, 0xfb3, full, 1],
                vec![0x5000, 0xfb4, full, 1],
                vec![0x5000, 0xfa9, full, 1],
                vec![0x5000, 0xfa3, 1.0f32.to_bits(), 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_NEEDS_UPDATE), vec![vec![0x5000, 3]]);
    }

    /// The traits `MakeNode` reads, by id.
    fn traits(id: u32, html: f64, font: f64) -> f64 {
        match id {
            0xfa1 => 10.0,
            0xfa2 => 20.0,
            0xfba => 0.0,
            0xfbb => 50.0,
            0xfbc => 0.0,
            0xff7 => 3.0,
            0xfb7 => 2.0,
            0xfb9 => font,
            0xfbd => 1.0,
            0xfbe => html,
            0xfa9 => 127.5,
            0xff5 => 255.0,
            0xfad => 5.0,
            _ => panic!("trait {id:x}"),
        }
    }

    struct Setup {
        e: Engine,
        this: Ptr<TileText>,
        data: u32,
    }

    fn make_node_engine(html: f64, new_fonts: bool, font: f64) -> Setup {
        let mut e = Engine::new();
        for (address, value) in [(ZERO, 0.0f64), (ALPHA_SCALE, 255.0), (DEPTH_STEP, -0.008)] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(0x0109_5260, 12);
        e.map(FONT_MANAGER, 4);
        let manager = e.mem.alloc(0x24);
        e.set_global(FONT_MANAGER, manager);
        e.mem.set_u8(manager + 0x20, new_fonts as u8);
        let font_object = e.mem.alloc(0x54);
        let data = e.mem.alloc(0x10);
        e.mem.set_f32(data, 12.0);
        e.mem.set_u32(font_object + 0x38, data);
        e.mem.set_u32(manager + 4, font_object);
        e.register_double(TILE_GET_FLOAT, move |_, a| Ret {
            st0: traits(a[1], html, font),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_SCREEN_WIDTH, |_, _| Ret {
            st0: 1280.0,
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_NODE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], NODE_VTABLE);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(TILE_GET_STRING, |_, _| Ret {
            eax: 0x0000_6000,
            ..Ret::default()
        });
        e.map(0x0000_6000, 8);
        e.mem.set_cstr(0x0000_6000, b"Hello");
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfa3) as u32,
            ..Ret::default()
        });
        e.register(TILE_GET_PARENT_MODEL, |_, _| Ret::default());
        e.register(INTERFACE_GET_INTERFACE_ROOT, |_, _| Ret {
            eax: 0x0004_0000,
            ..Ret::default()
        });
        e.register(TILE_EXTRA_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                ATTACH,
                BS_STRING_CONSTRUCT,
                BS_STRING_SET,
                BS_STRING_ASSIGN,
                TILE_SET_FLOAT,
                TILE_ADD_NEEDS_UPDATE,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                INTERFACE_SET_NEED_TO_UPDATE,
                NI_OBJECT_NET_ADD_EXTRA_DATA,
            ],
        );
        let mut slots = vec![0u32; 0x38];
        slots[(NODE_ATTACH_CHILD_SLOT / 4) as usize] = ATTACH;
        e.put_vtable(NODE_VTABLE, &slots);
        e.map(0x0004_0000, 4);
        e.mem.set_u32(0x0004_0000, NODE_VTABLE);
        let this = e.new_object::<TileText>();
        Setup { e, this, data }
    }

    #[test]
    fn tile_text_make_node_without_a_font_answers_null() {
        let Setup { mut e, this, .. } = make_node_engine(0.0, false, 5.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_1af0, &args![this]).u32(), 0);
        assert!(calls_to(&e, BS_STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn tile_text_make_node_builds_font_shapes() {
        let Setup { mut e, this, data } = make_node_engine(0.0, false, 2.0);
        let raised = Rc::new(RefCell::new(0.0f32));
        let seen = raised.clone();
        e.register_double(FONT_CREATE_TEXT, move |e, a| {
            *seen.borrow_mut() = e.mem.f32(e.mem.u32(a[0] + 0x38));
            // Measured size, a text shape and an icon shape.
            e.mem.set_i32(a[2], 400);
            e.mem.set_i32(a[3], 30);
            let shape = e.mem.alloc(0x70);
            e.mem.set_f32(shape + 0x58, 1.5);
            let icons = e.mem.alloc(0x70);
            e.mem.set_u32(a[9], shape);
            e.mem.set_u32(a[10], icons);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let model = e.call(0x00a2_1af0, &args![this]).u32();
        assert_ne!(model, 0);
        let create = calls_to(&e, FONT_CREATE_TEXT)[0].clone();
        assert_eq!(create[4..8], [0, 0x7fff_ffff, 2, 10]);
        let color: Vec<f32> = (0..4).map(|i| e.mem.f32(create[8] + 4 * i)).collect();
        assert_eq!(color, vec![1.0, 1.0, 1.0, 0.5]);
        // The base line is raised by the line gap during the build only.
        assert_eq!(*raised.borrow(), 15.0);
        assert_eq!(e.mem.f32(data), 12.0);
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.5);
        let attached = calls_to(&e, ATTACH);
        assert_eq!(attached.len(), 3);
        let shape = attached[0][1];
        let placed: Vec<f32> = (0..3).map(|i| e.mem.f32(shape + 0x58 + 4 * i)).collect();
        assert_eq!(placed, vec![10.0, (5.0f64 * -0.008) as f32, -20.0]);
        assert_eq!(attached[2], vec![0x0004_0000, model, 1]);
        let t = this.addr();
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![t, 0xfb1, 400.0f32.to_bits(), 1],
                vec![t, 0xfb0, 30.0f32.to_bits(), 1]
            ]
        );
        assert_eq!(calls_to(&e, TILE_ADD_NEEDS_UPDATE), vec![vec![t, 9]]);
        assert_eq!(e.mem.u32(model + 0x30) & 1, 0);
        assert_eq!(
            calls_to(&e, BS_STRING_SET),
            vec![vec![create[1], 0x6000, 0], vec![create[1], 0, 0]]
        );
    }

    #[test]
    fn tile_text_make_node_renders_hypertext_through_the_manager() {
        let Setup { mut e, this, data } = make_node_engine(1.0, false, 2.0);
        e.register(FONT_MANAGER_CREATE_TEXT, |e, a| {
            let block = Ptr::<ManagerTextData>::new(a[3]);
            assert_eq!(e.get(block, ManagerTextData::iDefaultFont), 1);
            assert_eq!(e.get(block, ManagerTextData::iJustification), 2);
            assert_eq!(e.get(block, ManagerTextData::iWidth), 1280);
            assert_eq!(e.get(block, ManagerTextData::iHeight), 50);
            assert_eq!(e.get(block, ManagerTextData::iPageNum), 1);
            assert_eq!(e.get(block, ManagerTextData::iLines), 0x7fff_ffff);
            assert_eq!(e.get(block, ManagerTextData::cLineSep), 10);
            assert_eq!(e.get(block, ManagerTextData::bIsHypertext), 1);
            assert_eq!(e.mem.f32(a[3] + 0x14), 0.5);
            e.set(block, ManagerTextData::iWidth, 640);
            e.set(block, ManagerTextData::iHeight, 90);
            e.set(block, ManagerTextData::iNumPages, 3);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let model = e.call(0x00a2_1af0, &args![this]).u32();
        assert_ne!(model, 0);
        assert!(calls_to(&e, FONT_CREATE_TEXT).is_empty());
        let made = calls_to(&e, FONT_MANAGER_CREATE_TEXT);
        assert_eq!(made[0][4], model);
        let t = this.addr();
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![t, 0xfd5, 3.0f32.to_bits(), 1],
                vec![t, 0xfb1, 640.0f32.to_bits(), 1],
                vec![t, 0xfb0, 90.0f32.to_bits(), 1]
            ]
        );
        assert_eq!(e.mem.u8(t + 0x48), 1);
        assert_eq!(e.mem.f32(data), 12.0);
    }

    #[test]
    fn tile_text_make_node_stops_when_the_manager_fails() {
        let Setup { mut e, this, data } = make_node_engine(0.0, true, 2.0);
        e.register(FONT_MANAGER_CREATE_TEXT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_1af0, &args![this]).u32(), 0);
        // The base line stays raised; nothing is attached.
        assert_eq!(e.mem.f32(data), 15.0);
        assert!(calls_to(&e, ATTACH).is_empty());
        assert_eq!(calls_to(&e, BS_STRING_SET).len(), 3);
    }

    #[test]
    fn tile_text_post_parse_queues_creation() {
        let mut e = Engine::new();
        quiet(&mut e, &[TILE_ADD_NEEDS_UPDATE]);
        let strings = Rc::new(RefCell::new(0u32));
        let current = strings.clone();
        e.register_double(TILE_GET_STRING, move |_, _| Ret {
            eax: *current.borrow(),
            ..Ret::default()
        });
        e.register(STRCMP, |e, a| Ret {
            eax: (e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32,
            ..Ret::default()
        });
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_cstr(a, b"x");
        e.mem.set_cstr(b, b"y");
        let this = 0x5000u32;
        let post = |e: &mut Engine, trait_id: u32, value: f32, previous: u32| {
            e.call(0x00a2_25b0, &args![this, trait_id, value, previous])
                .u32()
        };
        assert_eq!(post(&mut e, 0xfc4, 0.0, a), this);
        *strings.borrow_mut() = a;
        assert_eq!(post(&mut e, 0xfc4, 0.0, a), this);
        assert_eq!(post(&mut e, 0xfc4, 0.0, b), 0);
        assert_eq!(post(&mut e, 0xfb7, 2.0, 0), 0);
        assert_eq!(post(&mut e, 0xfb7, 1.0, 0), this);
        assert_eq!(post(&mut e, 0xfb7, f32::NAN, 0), this);
        assert_eq!(post(&mut e, 0xfa1, 1.0, 0), 0);
    }
}
