//! The game's fonts: `.fnt` glyph tables and their `.tex` pictures, read
//! and adjusted the way FalloutNV.exe's font loader does (`00a15320`).
//!
//! A `.fnt` file is 14632 bytes: the line height (f32), how many pictures
//! (u32, at most 8), eight (u32, 32-byte name) slots, then 256 glyphs of
//! 56 bytes: the picture (u32), the texture coordinates of the top-left,
//! top-right, bottom-left and bottom-right corners (f32 pairs), then width,
//! height, left kerning, right kerning and baseline (f32; the baseline
//! measured down from the glyph's top). Pictures are
//! `textures\fonts\<name>.tex`: u32 width, u32 height, then RGBA bytes.

/// The `[Fonts] sFontFile_N` defaults (the strings at `00fb37f0` ..
/// `00fb3c50`), used when the INI files say nothing. Font 9 has no
/// default in the exe.
pub const DEFAULT_FONT_FILES: [&str; 8] = [
    "Data\\Fonts\\Glow_Monofonto_Large.fnt",
    "Data\\Fonts\\Monofonto_Large.fnt",
    "Data\\Fonts\\Glow_Monofonto_Medium.fnt",
    "Data\\Fonts\\Monofonto_VeryLarge02_Dialogs2.fnt",
    "Data\\Fonts\\Fixedsys_Comp_uniform_width.fnt",
    "Data\\Fonts\\Glow_Monofonto_VL_dialogs.fnt",
    "Data\\Fonts\\Baked-in_Monofonto_Large.fnt",
    "Data\\Fonts\\Glow_Futura_Caps_Large.fnt",
];

/// Size of a `.fnt` file.
pub const FNT_SIZE: usize = 296 + 256 * GLYPH_SIZE;
const GLYPH_SIZE: usize = 56;

/// One character's picture and spacing.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Glyph {
    /// Which of the font's pictures it's in.
    pub texture: u32,
    /// Texture coordinates: top-left, top-right, bottom-left, bottom-right.
    pub uv: [[f32; 2]; 4],
    pub width: f32,
    pub height: f32,
    pub kern_left: f32,
    pub kern_right: f32,
    /// From the glyph's top down to the line it sits on.
    pub baseline: f32,
}

/// A loaded font.
#[derive(Debug, Clone)]
pub struct Font {
    /// The file's line height.
    pub line_height: f32,
    /// Its pictures' names (without folder or extension).
    pub textures: Vec<String>,
    pub glyphs: Vec<Glyph>,
    /// The loader's largest "line height - baseline + height" over all
    /// glyphs (the font object's `+0x2c`): text sits `2 × (line height -
    /// this)` above a text tile's origin.
    pub descent: f32,
    /// The loader's smallest "baseline - height" (at most 0; the font
    /// object's `+0x30`): how far the lowest glyph reaches below its line.
    pub lowest: f32,
}

/// What can go wrong reading a font.
#[derive(Debug, Clone, PartialEq)]
pub enum FontError {
    WrongSize(usize),
    TooManyTextures(u32),
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontError::WrongSize(n) => write!(f, "a .fnt file is {FNT_SIZE} bytes, this is {n}"),
            FontError::TooManyTextures(n) => write!(f, "{n} font pictures; the game allows 8"),
        }
    }
}

impl std::error::Error for FontError {}

fn f32_at(b: &[u8], at: usize) -> f32 {
    f32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

impl Font {
    /// Reads a `.fnt` file and adjusts it as the game's loader does
    /// (`00a15320`):
    ///
    /// * D = the largest (line height - baseline + height), H = the largest
    ///   height, m = the smallest (baseline - height), at most 0, over all
    ///   256 glyphs as stored;
    /// * the space (32) swaps its width and right kerning, and gets height
    ///   H and baseline m + H; 0xA0 takes the space's width, right
    ///   kerning, height and baseline;
    /// * glyph 0 gets width 0, right kerning 0, height H, baseline m + H
    ///   and no texture coordinates; 127 takes 124's sizes.
    pub fn parse(bytes: &[u8]) -> Result<Font, FontError> {
        if bytes.len() != FNT_SIZE {
            return Err(FontError::WrongSize(bytes.len()));
        }
        let line_height = f32_at(bytes, 0);
        let count = u32_at(bytes, 4);
        if count > 8 {
            return Err(FontError::TooManyTextures(count));
        }
        // A name is read as the game reads it, up to its first zero byte:
        // one 32 characters or longer runs on into the next slot (font 6's
        // `Glow_Monofonto_VL_Dialogs_0_Lod_A` has 33).
        let textures = (0..count as usize)
            .map(|i| {
                let name = &bytes[8 + i * 36 + 4..296];
                let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
                String::from_utf8_lossy(&name[..end]).into_owned()
            })
            .collect();
        let mut glyphs: Vec<Glyph> = (0..256)
            .map(|c| {
                let at = 296 + c * GLYPH_SIZE;
                let f = |i: usize| f32_at(bytes, at + 4 * i);
                Glyph {
                    texture: u32_at(bytes, at),
                    uv: [[f(1), f(2)], [f(3), f(4)], [f(5), f(6)], [f(7), f(8)]],
                    width: f(9),
                    height: f(10),
                    kern_left: f(11),
                    kern_right: f(12),
                    baseline: f(13),
                }
            })
            .collect();
        let mut descent = 0.0f32;
        let mut tallest = 0.0f32;
        let mut lowest = 0.0f32;
        for g in &glyphs {
            descent = descent.max(line_height - g.baseline + g.height);
            tallest = tallest.max(g.height);
            lowest = lowest.min(g.baseline - g.height);
        }
        let space = &mut glyphs[32];
        std::mem::swap(&mut space.width, &mut space.kern_right);
        space.height = tallest;
        space.baseline = lowest + tallest;
        let space = glyphs[32];
        let nbsp = &mut glyphs[0xA0];
        nbsp.width = space.width;
        nbsp.kern_right = space.kern_right;
        nbsp.height = space.height;
        nbsp.baseline = space.baseline;
        let bar = glyphs[124];
        let del = &mut glyphs[127];
        del.width = bar.width;
        del.height = bar.height;
        del.kern_left = bar.kern_left;
        del.kern_right = bar.kern_right;
        del.baseline = bar.baseline;
        let zero = &mut glyphs[0];
        zero.width = 0.0;
        zero.kern_right = 0.0;
        zero.height = tallest;
        zero.baseline = lowest + tallest;
        zero.uv = [[0.0; 2]; 4];
        Ok(Font {
            line_height,
            textures,
            glyphs,
            descent,
            lowest,
        })
    }

    /// The glyph a character code draws with: the curly quotes 0x91..0x94
    /// as ' and " (`00a122b0`).
    pub fn glyph(&self, code: u8) -> &Glyph {
        &self.glyphs[usize::from(straight_quotes(code))]
    }

    /// Where a font's picture is loaded from (`"TEXTURES\\FONTS\\%s.TEX"`,
    /// `00a15320`).
    pub fn texture_path(name: &str) -> String {
        format!("textures\\fonts\\{name}.tex")
    }
}

/// Curly quotes drawn straight (`00a122b0`).
pub fn straight_quotes(code: u8) -> u8 {
    match code {
        0x91 | 0x92 => b'\'',
        0x93 | 0x94 => b'"',
        other => other,
    }
}

/// The extra space between lines some fonts get (`00a1b3a0`): fonts 2 and
/// 6: 4, font 3: 6, font 7: 2, others 0. `font` counts from 1.
pub fn extra_line_gap(font: usize) -> f32 {
    match font {
        2 | 6 => 4.0,
        3 => 6.0,
        7 => 2.0,
        _ => 0.0,
    }
}

/// A `.tex` font picture: width, height and its RGBA bytes.
pub fn read_tex(bytes: &[u8]) -> Option<(u32, u32, &[u8])> {
    if bytes.len() < 8 {
        return None;
    }
    let w = u32_at(bytes, 0);
    let h = u32_at(bytes, 4);
    let size = (w as usize).checked_mul(h as usize)?.checked_mul(4)?;
    let pixels = bytes.get(8..8 + size)?;
    Some((w, h, pixels))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A font file built from scratch: line height 31, one picture,
    /// glyphs given as (code, width, height, left, right, baseline).
    pub fn font_bytes(line_height: f32, glyphs: &[(u8, f32, f32, f32, f32, f32)]) -> Vec<u8> {
        let mut b = vec![0u8; FNT_SIZE];
        b[0..4].copy_from_slice(&line_height.to_le_bytes());
        b[4..8].copy_from_slice(&1u32.to_le_bytes());
        b[8..12].copy_from_slice(&1u32.to_le_bytes());
        b[12..18].copy_from_slice(b"Test_0");
        for &(code, w, h, l, r, base) in glyphs {
            let at = 296 + usize::from(code) * GLYPH_SIZE;
            let mut put =
                |i: usize, v: f32| b[at + 4 * i..at + 4 * i + 4].copy_from_slice(&v.to_le_bytes());
            put(1, 0.1);
            put(2, 0.2);
            put(9, w);
            put(10, h);
            put(11, l);
            put(12, r);
            put(13, base);
        }
        b
    }

    #[test]
    fn the_loaders_adjustments() {
        let bytes = font_bytes(
            31.0,
            &[
                (b' ', 0.0, 0.0, 0.0, 14.0, 0.0),
                (b'A', 18.0, 30.0, 0.0, -2.0, 25.0),
                (b'g', 19.0, 28.0, 0.0, -2.0, 18.0),
                (b'|', 5.0, 33.0, 1.0, 2.0, 27.0),
            ],
        );
        let font = Font::parse(&bytes).unwrap();
        assert_eq!(font.line_height, 31.0);
        assert_eq!(font.textures, ["Test_0"]);
        let a = font.glyph(b'A');
        assert_eq!(
            (a.width, a.height, a.kern_right, a.baseline),
            (18.0, 30.0, -2.0, 25.0)
        );
        assert_eq!(a.uv[0], [0.1, 0.2]);
        // D: 'g' gives 31 - 18 + 28 = 41.
        assert_eq!(font.descent, 41.0);
        // The space: width and right kerning swapped, height = tallest
        // (33), baseline = lowest (18 - 28 = -10) + 33.
        let s = font.glyph(b' ');
        assert_eq!(
            (s.width, s.kern_right, s.height, s.baseline),
            (14.0, 0.0, 33.0, 23.0)
        );
        assert_eq!(font.glyph(0xA0).width, 14.0);
        assert_eq!(font.glyph(127).height, 33.0);
        assert_eq!(font.glyph(0).uv, [[0.0; 2]; 4]);
        assert_eq!(font.glyph(0x92), font.glyph(b'\''));
    }

    #[test]
    fn long_picture_names_run_into_the_next_slot() {
        let mut bytes = font_bytes(34.0, &[(b'A', 18.0, 30.0, 0.0, -2.0, 25.0)]);
        let name = b"Glow_Monofonto_VL_Dialogs_0_Lod_A";
        bytes[12..12 + name.len()].copy_from_slice(name);
        let font = Font::parse(&bytes).unwrap();
        assert_eq!(font.textures, ["Glow_Monofonto_VL_Dialogs_0_Lod_A"]);
    }

    #[test]
    fn wrong_sizes_and_pictures() {
        assert_eq!(Font::parse(&[0; 10]).unwrap_err(), FontError::WrongSize(10));
        let mut tex = vec![0u8; 8 + 2 * 2 * 4];
        tex[0] = 2;
        tex[4] = 2;
        assert_eq!(
            read_tex(&tex).map(|(w, h, p)| (w, h, p.len())),
            Some((2, 2, 16))
        );
        assert!(read_tex(&tex[..10]).is_none());
        assert_eq!(extra_line_gap(3), 6.0);
    }
}
