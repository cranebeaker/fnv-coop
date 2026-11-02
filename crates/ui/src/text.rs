//! Laying text out and placing its glyphs as FalloutNV.exe does: the line
//! breaking of `00a12fb0` (whole-number widths, breaks at spaces, `~` as a
//! soft hyphen, forced hyphens), then the glyphs of `00a12880` and
//! `00a142d0`. Checked against a recorded frame of the game's HUD: "HP",
//! "CND" and "2/8" come out at the same vertex positions.

use crate::font::{straight_quotes, Font};

/// Justification numbers (`&left;` 1, `&center;` 2, `&right;` 4).
pub const LEFT: i32 = 1;
pub const CENTER: i32 = 2;
pub const RIGHT: i32 = 4;

/// Text broken into lines, and its size.
#[derive(Debug, Clone, PartialEq)]
pub struct Prepared {
    /// The text with the line breaks put in (tabs dropped).
    pub text: Vec<u8>,
    /// Each line's width, as the breaking measured it.
    pub line_widths: Vec<i32>,
    pub width: i32,
    pub height: i32,
    pub lines: i32,
}

fn advance(font: &Font, code: u8) -> i32 {
    let g = font.glyph(code);
    (g.width + g.kern_right) as i32
}

/// Breaks `text` into lines no wider than `wrap_width` (`< 1`: no limit),
/// keeping at most `max_lines` (`< 1`: no limit) (`00a12fb0`):
///
/// * each character adds trunc(width + right kerning) (curly quotes as
///   straight ones); a tab moves on to the next multiple of 75 and isn't
///   kept; `\n` ends the line;
/// * a space or `~` is a break point (`~` isn't drawn); past the wrap
///   width the line ends at the last break point (that space becomes the
///   line end and doesn't count), or, with none, the previous character
///   moves down with a `-` put in its place;
/// * the height starts at the space's height and each further line adds
///   the line height plus `extra_gap`; lines past `max_lines` are dropped;
///   no text at all lays out as one space.
///
/// `line_height` is the font's, as the caller set it for this text (the
/// text tile truncates it and adds its `linegap`).
pub fn prepare(
    font: &Font,
    line_height: f32,
    extra_gap: f32,
    text: &[u8],
    wrap_width: i32,
    max_lines: i32,
) -> Prepared {
    let wrap = if wrap_width < 1 { i32::MAX } else { wrap_width };
    let max_lines = if max_lines < 1 { i32::MAX } else { max_lines };
    let step = line_height + extra_gap;
    let mut out: Vec<u8> = Vec::with_capacity(text.len() + 4);
    let mut widths: Vec<i32> = Vec::new();
    let mut height = font.glyph(b' ').height;
    let mut lines = 1;
    let mut x = 0i32;
    let mut widest = 0i32;
    // The last break point (an index in `out`; 0 is "none", as in the game).
    let mut brk = 0usize;
    let mut before_break = 0i32;
    let mut at_break = 0i32;
    let mut soft = false;
    for &raw in text {
        if raw == 0 {
            break;
        }
        if raw == b'\n' {
            out.push(b'\n');
            height += step;
            widths.push(x);
            widest = widest.max(x);
            x = 0;
            brk = 0;
            lines += 1;
        } else if raw == b'\t' {
            x += 75 - x % 75;
            continue;
        } else {
            let mut c = straight_quotes(raw);
            x += advance(font, c);
            if c == b' ' {
                brk = out.len();
                before_break = x - advance(font, b' ');
                at_break = x;
                soft = false;
            } else if c == b'~' {
                brk = out.len();
                soft = true;
                before_break = x;
                at_break = x;
                x -= advance(font, b'~');
            }
            if wrap < x {
                if brk == 0 {
                    // No break point: the previous character moves to the
                    // next line and a hyphen takes its place.
                    let n = out.len();
                    let prev = if n > 0 { out[n - 1] } else { 0 };
                    if n > 0 {
                        out[n - 1] = b'-';
                    }
                    out.push(b'\n');
                    out.push(prev);
                    height += step;
                    // The game's arithmetic: the width so far less one
                    // hyphen (right for fonts whose advances are equal).
                    x -= advance(font, b'-');
                    widths.push(x);
                    widest = widest.max(x);
                    lines += 1;
                    x = advance(font, prev) + advance(font, c);
                } else if !soft {
                    if brk == out.len() {
                        c = b'\n';
                    } else {
                        out[brk] = b'\n';
                    }
                    height += step;
                    widths.push(before_break);
                    widest = widest.max(before_break);
                    brk = 0;
                    lines += 1;
                    x -= at_break;
                } else {
                    soft = false;
                    out.splice(brk..brk, *b"-\n");
                    height += step;
                    x -= advance(font, b'-');
                    widths.push(x);
                    widest = widest.max(x);
                    brk = 0;
                    lines += 1;
                    // The game reads the byte before its buffer here; taken
                    // as glyph 0 (no width): a guess.
                    x = advance(font, 0) + advance(font, c);
                }
            }
            if c != b'~' {
                out.push(c);
            }
        }
        // Past the last line allowed: cut back to the last line break.
        // (The game leaves its line count one too high and still counts
        // the cut line's width below; both kept.)
        if lines > max_lines && !out.is_empty() {
            while let Some(&b) = out.last() {
                out.pop();
                if b == b'\n' {
                    break;
                }
            }
            height -= step;
            break;
        }
    }
    if out.is_empty() {
        out.push(b' ');
        lines = 1;
        height = font.glyph(b' ').height;
        x = advance(font, b' ');
    }
    widths.push(x);
    widest = widest.max(x);
    Prepared {
        text: out,
        line_widths: widths,
        width: widest,
        // `ROUND`: to nearest, halves to even.
        height: round_half_even(height),
        lines,
    }
}

/// Puts game settings into text before it's laid out (`00a12fb0`'s first
/// pass): `&-sName;` (and `&sName;`) becomes the setting's text. The name
/// is read the way the game reads it: from after `&` or `&-` while the
/// character that many places back isn't `;` or a line end, then
/// shortened by that many characters, so a name ending the text loses its
/// last characters (as in the game). Names the setting lookup doesn't
/// know leave the `&` as it is. (The game asks `007070c0` first: the
/// actions' settings read as their bindings' names, which `game::new_ui`
/// puts in the settings, `controls`; the player's name and race aren't.)
pub fn substitute(text: &[u8], setting: &dyn Fn(&str) -> Option<String>) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    let at = |i: usize| text.get(i).copied().unwrap_or(0);
    let mut i = 0usize;
    while i < text.len() {
        if text[i] != b'&' {
            out.push(text[i]);
            i += 1;
            continue;
        }
        let off = if at(i + 1) == b'-' { 2 } else { 1 };
        let mut name = Vec::new();
        let mut k = 0usize;
        while at(i + k + off) != 0 && k < 0x7f && at(i + k) != b';' && at(i + k) != b'\n' {
            name.push(at(i + k + off));
            k += 1;
        }
        let len = if k == 0 { 0 } else { k.saturating_sub(off) };
        name.truncate(len);
        let mut skip = name.len() + 1;
        if at(i + k) == b';' {
            skip += off;
        }
        let name = String::from_utf8_lossy(&name).into_owned();
        match setting(&name) {
            Some(value) if !name.is_empty() => {
                out.extend_from_slice(value.as_bytes());
                i += skip;
            }
            _ => {
                out.push(b'&');
                i += 1;
            }
        }
    }
    out
}

/// x87 rounding (to nearest, halves to even).
pub fn round_half_even(v: f32) -> i32 {
    let r = v.round();
    if (v - v.trunc()).abs() == 0.5 && (r as i64) % 2 != 0 {
        (r - v.signum()) as i32
    } else {
        r as i32
    }
}

/// Text measured the way the menus' code measures it before laying a menu
/// out ("Test Height", `00a1b020`): its widest line, its height and its
/// number of lines.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measured {
    pub width: f32,
    pub height: f32,
    pub lines: i32,
}

/// Measures `text` from byte `start` in a font for a wrap width (`00a1b020`,
/// read from the code). Unlike the text tiles' own layout (`prepare`,
/// whole-number advances) it works in floats:
///
/// * each character advances by left kerning + width + right kerning (curly
///   quotes as straight ones); a tab by 75 less the line so far modulo 75;
/// * a space or a line break notes the width so far as the break point; `~`
///   notes it plus a hyphen's advance and marks the break as soft (and isn't
///   counted);
/// * past the wrap width, or at a line break, the line ends: with no break
///   point noted the previous character moves down (the line taking its and
///   this character's width less a hyphen), else the new line keeps what
///   follows the break point, less the space's advance (nothing after a line
///   break; a soft break keeps the hyphen's share); the widest line is the
///   widest break point; each new line adds the font's line height and its
///   extra gap (`extra_line_gap`) to the height, which starts at the
///   space's height;
/// * the width is the widest line or the last one, whichever is wider.
///
/// `font_index` counts from 1; outside 1 to 8 the game measures nothing.
pub fn measure(
    font: &Font,
    font_index: usize,
    text: &[u8],
    wrap_width: f32,
    start: usize,
) -> Measured {
    if !(1..=8).contains(&font_index) {
        return Measured {
            width: 0.0,
            height: 0.0,
            lines: 0,
        };
    }
    let advance = |c: u8| {
        let g = font.glyph(c);
        g.kern_left + g.width + g.kern_right
    };
    let hyphen = advance(b'-');
    let space = advance(b' ');
    let extra = crate::font::extra_line_gap(font_index);
    let mut widest = 0.0f32;
    let mut x = 0.0f32;
    let mut brk = 0.0f32;
    let mut prev = 0.0f32;
    let mut soft = false;
    let mut lines = 1;
    let mut height = font.glyph(b' ').height;
    for &c in text.iter().skip(start) {
        if c == 0 {
            break;
        }
        let mut adv = advance(c);
        match c {
            b'\t' => adv = 75.0 - x % 75.0,
            b'\n' | b' ' => {
                brk = x;
                soft = false;
            }
            b'~' => {
                brk = x + hyphen;
                soft = true;
            }
            _ => {}
        }
        if c != b'~' {
            x += adv;
        }
        if wrap_width < x || c == b'\n' {
            if brk <= 0.0 {
                brk = (x - adv) - prev + hyphen;
                x = adv + prev;
            } else {
                x -= brk;
                if !soft {
                    if c == b'\n' {
                        x = 0.0;
                    } else {
                        x -= space;
                    }
                }
            }
            widest = widest.max(brk);
            height += extra + font.line_height;
            brk = 0.0;
            lines += 1;
        }
        prev = adv;
    }
    Measured {
        width: widest.max(x),
        height,
        lines,
    }
}

/// One glyph to draw: a rectangle relative to the text tile's origin (x
/// right, y down, menu units) and its texture coordinates (top-left,
/// top-right, bottom-left, bottom-right).
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphQuad {
    pub texture: u32,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub uv: [[f32; 2]; 4],
}

/// The glyphs of prepared text (`00a12880`, `00a142d0`). Lines start at x
/// 0 (left), -width (right, 4) or -(width / 2) (centre, 2, whole
/// numbers); each glyph moves the pen by its left kerning, sits from the
/// pen line up by its baseline, and the pen then moves by width + right
/// kerning (no right kerning for glyphs without width). The first line's
/// pen is 2 × (line height - the font's descent) above the origin; each
/// line moves down by line height + `extra_gap`. Also returns the width the
/// game writes back: the widest of the breaking's width and how far the
/// pen got from the first line's start.
pub fn glyphs(
    font: &Font,
    line_height: f32,
    extra_gap: f32,
    prepared: &Prepared,
    justify: i32,
) -> (Vec<GlyphQuad>, i32) {
    let start = |line: usize| -> f32 {
        let w = prepared.line_widths.get(line).copied().unwrap_or(-1);
        match justify {
            RIGHT => -w as f32,
            CENTER => -(w / 2) as f32,
            _ => 0.0,
        }
    };
    let mut quads = Vec::with_capacity(prepared.text.len());
    let mut line = 0usize;
    let mut x = start(0);
    let first = x;
    let mut z = 2.0 * (line_height - font.descent);
    let mut width = prepared.width;
    for &c in &prepared.text {
        if c == b'\n' {
            line += 1;
            x = start(line);
            z -= line_height + extra_gap;
        }
        let g = font.glyph(c);
        x += g.kern_left;
        let top = z + g.baseline;
        quads.push(GlyphQuad {
            texture: g.texture,
            left: x,
            right: x + g.width,
            top: -top,
            bottom: -(top - g.height),
            uv: g.uv,
        });
        x += g.width + if g.width > 0.0 { g.kern_right } else { 0.0 };
        width = width.max((x - first) as i32);
    }
    (quads, width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::tests::font_bytes;

    /// Font 7 of the game (Baked-in Monofonto Large) in miniature: line
    /// height 36, every letter 25 wide with right kerning -10, baseline
    /// 30; '/' 19 wide; a descender making the descent 48; the space 13.
    fn font7() -> Font {
        let mut glyphs = vec![
            (b' ', 0.0, 0.0, 0.0, 13.0, 0.0),
            (b'/', 19.0, 38.0, 0.0, -10.0, 30.0),
        ];
        for c in *b"HPCN28A-~" {
            glyphs.push((c, 25.0, 36.0, 0.0, -10.0, 30.0));
        }
        glyphs.push((b'D', 25.0, 35.0, 0.0, -10.0, 30.0));
        glyphs.push((b'g', 25.0, 42.0, 0.0, -10.0, 30.0));
        Font::parse(&font_bytes(36.0, &glyphs)).unwrap()
    }

    #[test]
    fn right_justified_labels_match_the_recorded_hud() {
        let font = font7();
        let p = prepare(&font, 36.0, 2.0, b"HP", 0, 0);
        assert_eq!((p.width, p.lines), (30, 1));
        let (q, _) = glyphs(&font, 36.0, 2.0, &p, RIGHT);
        // The recording: H from -30 to -5, P from -15 to 10, from 6
        // above the origin (z 6) to 30 below.
        assert_eq!(
            (q[0].left, q[0].right, q[0].top, q[0].bottom),
            (-30.0, -5.0, -6.0, 30.0)
        );
        assert_eq!((q[1].left, q[1].right), (-15.0, 10.0));
        // "2/8": '/' advances 9.
        let p = prepare(&font, 36.0, 2.0, b"2/8", 0, 0);
        assert_eq!(p.width, 39);
        let (q, _) = glyphs(&font, 36.0, 2.0, &p, RIGHT);
        let lefts: Vec<f32> = q.iter().map(|g| g.left).collect();
        assert_eq!(lefts, [-39.0, -24.0, -15.0]);
        assert_eq!((q[1].top, q[1].bottom), (-6.0, 32.0));
    }

    #[test]
    fn breaking_at_spaces_and_counting_lines() {
        let font = font7();
        // Each letter 15: "HP HP HP" with room for 5 characters.
        let p = prepare(&font, 36.0, 0.0, b"HP HP HP", 75, 0);
        assert_eq!(p.text, b"HP HP\nHP");
        // 15 + 15 + 13 (the space) + 15 + 15.
        assert_eq!(p.line_widths, [73, 30]);
        assert_eq!(p.lines, 2);
        // The space's height (the tallest glyph, 42) plus one line.
        assert_eq!(p.height, 42 + 36);
        let p = prepare(&font, 36.0, 0.0, b"HP HP HP", 75, 1);
        assert_eq!(p.text, b"HP HP");
        assert_eq!(p.height, 42);
        assert_eq!(p.width, 73);
        // Centred: lines start at -(width / 2).
        let p = prepare(&font, 36.0, 0.0, b"H\nHP", 0, 0);
        let (q, _) = glyphs(&font, 36.0, 0.0, &p, CENTER);
        assert_eq!(q[0].left, -7.0);
        assert_eq!(q[2].left, -15.0);
        // The second line sits a line height lower.
        assert_eq!(q[2].top - q[0].top, 36.0);
    }

    #[test]
    fn forced_hyphens_tabs_and_empty_text() {
        let font = font7();
        // Too long for 40 with no space: each overflow moves the previous
        // letter down behind a hyphen.
        let p = prepare(&font, 36.0, 0.0, b"HPHP", 40, 0);
        assert_eq!(p.text, b"H-\nP-\nHP");
        let p = prepare(&font, 36.0, 0.0, b"H\tP", 0, 0);
        assert_eq!(p.text, b"HP");
        assert_eq!(p.width, 90);
        let p = prepare(&font, 36.0, 0.0, b"", 0, 0);
        assert_eq!((p.text.as_slice(), p.width, p.height), (&b" "[..], 13, 42));
    }

    /// `00a1b020`: float advances, breaks at spaces with the space taken
    /// off the new line, a forced break moving the previous character down,
    /// a line height plus the font's extra gap per line.
    #[test]
    fn measuring_as_the_menus_code_does() {
        let font = font7();
        // H and P advance 15, the space 13 (font 7's extra gap is 2).
        let m = measure(&font, 7, b"HP HP", 1000.0, 0);
        assert_eq!((m.width, m.lines), (73.0, 1));
        assert_eq!(m.height, font.glyph(b' ').height);
        // Wrapped at 40: "HP" then "HP"; the widest line is the break point.
        let m = measure(&font, 7, b"HP HP", 40.0, 0);
        assert_eq!((m.width, m.lines), (30.0, 2));
        assert_eq!(m.height, font.glyph(b' ').height + 36.0 + 2.0);
        // No break point: the previous letter moves down with this one
        // (the new line starts 30 wide), and again at the fourth.
        let m = measure(&font, 7, b"HPHP", 40.0, 0);
        assert_eq!((m.width, m.lines), (30.0, 3));
        // A line break, and starting part-way through.
        let m = measure(&font, 7, b"HP\nH", 1000.0, 0);
        assert_eq!((m.width, m.lines), (30.0, 2));
        assert_eq!(measure(&font, 7, b"HP\nH", 1000.0, 3).width, 15.0);
        // Fonts outside 1 to 8 measure nothing.
        assert_eq!(measure(&font, 0, b"HP", 1000.0, 0).lines, 0);
    }

    #[test]
    fn settings_in_text() {
        let setting = |n: &str| (n == "sStatsXP").then(|| "XP".to_string());
        assert_eq!(substitute(b"+10 &-sStatsXP; now", &setting), b"+10 XP now");
        assert_eq!(substitute(b"a & b", &setting), b"a & b");
        // At the very end the game's reading loses the name's last
        // characters, so it isn't found.
        assert_eq!(substitute(b"&-sStatsXP;", &setting), b"&-sStatsXP;");
    }

    #[test]
    fn halves_round_to_even() {
        assert_eq!(round_half_even(2.5), 2);
        assert_eq!(round_half_even(3.5), 4);
        assert_eq!(round_half_even(-2.5), -2);
        assert_eq!(round_half_even(2.4), 2);
    }
}
