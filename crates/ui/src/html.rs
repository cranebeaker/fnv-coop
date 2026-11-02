//! Text tiles with `ishtml` set: the game's own small HTML layout, read
//! from FalloutNV.exe 1.4.0.525. A text tile whose `ishtml` trait is above
//! 0 (`00a21af0`, the `TileText` update) is laid out by `00a18a30`: text
//! starting with `<` (after any whitespace) goes through the parser
//! `00a17390`; other text is laid out by the same engine one character at
//! a time (`00a18a30`'s own loop). In vanilla only the tutorial menu sets
//! `ishtml` (`007e9060`, on a message text starting with `<`) and only
//! `book_menu.xml`'s three page texts have it in a file (the book menu
//! isn't reached in New Vegas); the one vanilla text written in it is
//! `HelpHealingLimbs` (and its Xbox copy).
//!
//! The parser (`00a17390`) reads tags, attributes and the text between
//! them with `Font::CollectTo` (`00a16ea0`) and its character classes
//! (`00a16da0`); tag and attribute names, and values not in quotes, are
//! upper-cased. It knows `BR` (a line break), `P` (two), `HR` (two, or a
//! new page when the tile has a `wraplimit`), `DIV` (a line break for each
//! attribute; `ALIGN` `LEFT`, `CENTER`, `RIGHT`), `FONT` (`FACE` a font's
//! file or number 1 to 8, `COLOR` six hex digits), `/FONT` (back to font
//! 1, the default colour, left aligned) and `IMG` (`SRC`, `WIDTH`,
//! `HEIGHT`); anything else is ignored. `&name;` in text, names and values
//! is the game setting or control `name` (`007070c0`), kept as it is when
//! there's none; a `&` with no `;` is dropped.
//!
//! The layout (`00a19a10`, `00a19c00`, `00a19f70`) puts each character (an
//! "element") on a line, lines on pages; `wraplimit` is a page's height
//! and `pagenum` the page shown, `pagecount` the number of pages. The
//! drawing (`00a19060`) places each line a line height (and its gap) below
//! the last, aligned in the wrap width. The colour of `COLOR` goes into the
//! glyphs' vertices, which the tile shaders (`TILE1000`, `TILE1002`) don't
//! read for colour: on screen the text keeps the tile's colour.

use crate::font::{straight_quotes, Font, DEFAULT_FONT_FILES};
use crate::text::GlyphQuad;

/// Each font's line height as the layout reads it (`011a709c`, indexed by
/// font from 0; fonts 6 to 8 have 0).
pub const LINE_HEIGHTS: [i32; 8] = [35, 35, 30, 60, 60, 0, 0, 0];

/// The colour elements start with (117, 59, 33).
pub const DEFAULT_COLOR: [f32; 4] = [117.0 / 255.0, 59.0 / 255.0, 33.0 / 255.0, 1.0];

/// Alignments (`DIV ALIGN`; the justification numbers).
pub const LEFT: i32 = 1;
pub const CENTER: i32 = 2;
pub const RIGHT: i32 = 4;

/// "No limit" for a wrap width or page height.
pub const UNLIMITED: i32 = i32::MAX;

/// One character or picture (the game's 0x38-byte element; also the
/// parser's current style).
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// Font index from 0 (font 1 is 0).
    pub font: usize,
    pub code: u8,
    pub color: [f32; 4],
    pub align: i32,
    /// An `IMG`'s picture (empty for a character).
    pub picture: String,
    /// How far it moves the line on: trunc(width + left and right
    /// kerning), 0 for a glyph with no width.
    pub advance: i32,
    /// trunc(the font's line height).
    pub height: i32,
    /// trunc(-(the font's lowest baseline - height)).
    pub depth: i32,
    /// Where it starts on its line.
    pub x: i32,
}

fn font_at(fonts: &[Option<Font>], font: usize) -> Option<&Font> {
    fonts.get(font).and_then(|f| f.as_ref())
}

impl Element {
    /// A new element (`00a1b450`): the font index is kept within 0 to 8,
    /// the sizes come from the glyph.
    pub fn new(fonts: &[Option<Font>], font: usize, code: u8, color: [f32; 4], align: i32) -> Self {
        let mut e = Element {
            font: font.min(8),
            code,
            color,
            align,
            picture: String::new(),
            advance: 0,
            height: 0,
            depth: 0,
            x: 0,
        };
        e.measure(fonts);
        e
    }

    fn measure(&mut self, fonts: &[Option<Font>]) {
        let Some(f) = font_at(fonts, self.font) else {
            self.advance = 0;
            self.height = 0;
            self.depth = 0;
            return;
        };
        let g = &f.glyphs[usize::from(self.code)];
        let extra = if g.width == 0.0 {
            0.0
        } else {
            f64::from(g.kern_right) + f64::from(g.kern_left)
        };
        self.advance = (f64::from(g.width) + extra) as i32;
        self.height = f.line_height as i32;
        self.depth = -f.lowest as i32;
    }

    /// Changes the character (`00a1b7f0`): the sizes follow unless it's a
    /// picture.
    pub fn set_code(&mut self, fonts: &[Option<Font>], code: u8) {
        self.code = code;
        if self.picture.is_empty() {
            self.measure(fonts);
        }
        self.x = 0;
    }

    /// `/FONT` (`00a1b770`): font 1, the default colour, left aligned, no
    /// picture, a space.
    fn reset(&mut self, fonts: &[Option<Font>]) {
        self.font = 0;
        self.color = DEFAULT_COLOR;
        self.align = LEFT;
        self.picture.clear();
        self.set_code(fonts, b' ');
    }

    fn is_glyph(&self) -> bool {
        self.picture.is_empty()
    }
}

/// A line of elements (`00a1bd40`).
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub elements: Vec<Element>,
    /// How wide it is (with the game's arithmetic; see [`Line::add`]).
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    /// Space above it from extra line breaks.
    pub gap: i32,
    /// Its first element's alignment.
    pub align: i32,
    wrap: i32,
}

impl Line {
    fn new(fonts: &[Option<Font>], first: Element, gap: i32, wrap: i32) -> Line {
        let mut line = Line {
            elements: Vec::new(),
            width: 0,
            height: 0,
            depth: 0,
            gap,
            align: first.align,
            wrap,
        };
        if first.advance > wrap {
            // Wider than the wrap width on its own: the game's add would
            // start another line with it, and so on without end; here it
            // stays on this one.
            let mut first = first;
            first.x = 0;
            line.width = first.advance;
            line.height = Line::line_height(first.font).max(first.height);
            line.depth = first.depth;
            line.elements.push(first);
            return line;
        }
        let _ = line.add(fonts, first, false);
        line
    }

    fn line_height(font: usize) -> i32 {
        LINE_HEIGHTS.get(font).copied().unwrap_or(0)
    }

    /// Adds an element at the end (or the front, when words move down)
    /// (`00a19f70`). Past the wrap width it starts a new line, returned:
    ///
    /// * a space becomes character 0 (no width) at the new line's start;
    /// * otherwise, with a space on this line, the characters after the
    ///   last space move down in front of it and the space goes; this
    ///   line's width then loses all but its last character's width (the
    ///   game takes off the widths of the characters it looks at after
    ///   the first);
    /// * with no space, characters move down until a hyphen (font 1) fits
    ///   and the hyphen ends this line.
    ///
    /// The new line's height is its font's line height. A line's height
    /// grows to a character's height where the font changes, and to a
    /// picture's.
    fn add(&mut self, fonts: &[Option<Font>], mut e: Element, front: bool) -> Option<Line> {
        if self.wrap.saturating_sub(e.advance) < self.width {
            let font = e.font;
            if e.code == b' ' {
                e.set_code(fonts, 0);
                let mut next = Line::new(fonts, e, 0, self.wrap);
                next.height = Line::line_height(font);
                return Some(next);
            }
            let mut next = Line::new(fonts, e, 0, self.wrap);
            next.height = Line::line_height(font);
            if !self.elements.iter().any(|e| e.code == b' ') {
                let hyphen = Element::new(fonts, 0, b'-', DEFAULT_COLOR, LEFT);
                let mut moved = 0;
                while !self.elements.is_empty()
                    && self.wrap.saturating_sub(hyphen.advance) < self.width - moved
                {
                    let last = self.elements.pop().expect("not empty");
                    moved += last.advance;
                    let _ = next.add(fonts, last, true);
                }
                self.width -= moved;
                let _ = self.add(fonts, hyphen, false);
            } else {
                let mut last = self.elements.pop().expect("has a space");
                while !self.elements.is_empty() && last.code != b' ' {
                    let _ = next.add(fonts, last, true);
                    last = self.elements.pop().expect("not empty");
                    self.width -= last.advance;
                }
            }
            return Some(next);
        }
        if e.is_glyph() {
            let mut font_changed = false;
            if self.elements.is_empty() {
                self.height = Line::line_height(e.font);
            }
            if front {
                if let Some(first) = self.elements.first() {
                    font_changed = e.font != first.font;
                }
            } else if let Some(last) = self.elements.last() {
                font_changed = e.font != last.font;
            }
            if font_changed {
                self.height = self.height.max(e.height);
            }
        } else {
            self.height = self.height.max(e.height);
        }
        self.depth = self.depth.max(e.depth);
        if front {
            e.x = 0;
            self.width = e.advance;
            for other in &mut self.elements {
                other.x = self.width;
                self.width += other.advance;
            }
            self.elements.insert(0, e);
        } else {
            e.x = self.width;
            self.width += e.advance;
            self.elements.push(e);
        }
        None
    }
}

/// A page of lines (`00a1bc70`).
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub lines: Vec<Line>,
    /// The height its lines take.
    pub used: i32,
    wrap: i32,
    limit: i32,
    /// The last character's line height plus its height below its
    /// baseline: what each extra line break adds.
    last: i32,
}

impl Page {
    fn new(wrap: i32, limit: i32) -> Page {
        Page {
            lines: Vec::new(),
            used: 0,
            wrap,
            limit,
            last: 35,
        }
    }

    /// Adds an element after `breaks` line breaks (`00a19c00`): with any
    /// (or none yet) it starts a new line, `breaks - 1` times the last
    /// character's height lower. A line that doesn't fit the page's height
    /// starts a new page, returned.
    fn add(&mut self, fonts: &[Option<Font>], e: Element, breaks: i32) -> Option<Page> {
        let gap = self.last * (breaks - 1).max(0);
        let glyph_height = if e.is_glyph() {
            font_at(fonts, e.font).map(|f| {
                let g = &f.glyphs[usize::from(e.code)];
                f.line_height as i32 + (g.height - g.baseline) as i32
            })
        } else {
            None
        };
        let line = if self.lines.is_empty() || breaks != 0 {
            Some(Line::new(fonts, e, gap, self.wrap))
        } else {
            let last = self.lines.last_mut().expect("not empty");
            last.add(fonts, e, false)
        };
        if let Some(h) = glyph_height {
            self.last = h;
        }
        let mut line = line?;
        if self.limit.saturating_sub(line.height + line.gap) < self.used {
            line.gap = 0;
            let mut page = Page::new(self.wrap, self.limit);
            page.used = line.height;
            page.lines.push(line);
            return Some(page);
        }
        self.used += line.height + line.gap;
        self.lines.push(line);
        None
    }
}

/// Laid-out HTML text: its pages.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub pages: Vec<Page>,
    pub wrap: i32,
    pub limit: i32,
    /// Whether the parser made it (alignment is then within the wrap
    /// width), rather than the one-character-at-a-time fallback.
    pub parsed: bool,
}

impl Layout {
    fn new(wrap: i32, limit: i32, parsed: bool) -> Layout {
        Layout {
            pages: Vec::new(),
            wrap,
            limit,
            parsed,
        }
    }

    /// `00a19a10`: onto the last page, or a new one (the first, or after
    /// `HR`), dropping the line breaks.
    fn append(&mut self, fonts: &[Option<Font>], e: Element, breaks: i32, new_page: bool) {
        if self.pages.is_empty() || new_page {
            let mut page = Page::new(self.wrap, self.limit);
            let _ = page.add(fonts, e, 0);
            self.pages.push(page);
        } else if let Some(page) = self
            .pages
            .last_mut()
            .expect("not empty")
            .add(fonts, e, breaks)
        {
            self.pages.push(page);
        }
    }

    /// The size the tile takes (written back as its `width` and
    /// `height`): the widest line and the lines' heights and gaps on page
    /// `page`; with no such page, the wrap width and page height
    /// themselves (the game leaves them there).
    pub fn size(&self, page: i32) -> (i32, i32) {
        match usize::try_from(page).ok().and_then(|p| self.pages.get(p)) {
            Some(p) => (
                p.lines.iter().map(|l| l.width).max().unwrap_or(0),
                p.lines.iter().map(|l| l.height + l.gap).sum(),
            ),
            None => (self.wrap, self.limit),
        }
    }
}

/// A character's class (`00a16da0`): 0x20 the end, 1 `<` or `{`, 2 `>` or
/// `}`, 4 below `!` (signed, so bytes from 0x80 too), 8 a quote, 0x10
/// `=`, 0 anything else.
pub fn class(c: u8) -> u32 {
    match c {
        0 => 0x20,
        b'"' | b'\'' => 8,
        b'<' | b'{' => 1,
        b'=' => 0x10,
        b'>' | b'}' => 2,
        c if (c as i8) < 0x21 => 4,
        _ => 0,
    }
}

struct Reader<'a> {
    text: &'a [u8],
    pos: usize,
    setting: &'a dyn Fn(&str) -> Option<String>,
}

impl Reader<'_> {
    fn at(&self, i: usize) -> u8 {
        self.text.get(i).copied().unwrap_or(0)
    }

    /// `Font::CollectTo` (`00a16ea0`): reads on until a character whose
    /// class is in `stop` (taken) or, with `keep`, one whose class isn't
    /// in `keep` (left); line ends are skipped. With `collect` the
    /// characters are kept, `&name;` replaced. Returns the text and the
    /// class it stopped at.
    fn collect(&mut self, stop: u32, keep: u32, collect: bool) -> (Vec<u8>, u32) {
        let mut out = Vec::new();
        loop {
            let c = self.at(self.pos);
            let k = class(c);
            if k & 0x20 != 0 {
                return (out, 0x20);
            }
            if k & stop != 0 {
                self.pos += 1;
                return (out, k);
            }
            if keep != 0 && k & keep == 0 {
                return (out, k);
            }
            if c == b'\n' || c == b'\r' {
                self.pos += 1;
                continue;
            }
            if collect {
                if c == b'&' {
                    let mut n = 1;
                    while class(self.at(self.pos + n)) == 0 && self.at(self.pos + n - 1) != b';' {
                        n += 1;
                    }
                    if self.at(self.pos + n - 1) == b';' {
                        let entity = &self.text[self.pos..self.pos + n];
                        match self.entity(entity) {
                            // A picture for the text (a control's button):
                            // not drawn here.
                            Some(v) if v.starts_with('\\') => {}
                            Some(v) if !v.is_empty() => out.extend_from_slice(v.as_bytes()),
                            _ => out.extend_from_slice(entity),
                        }
                        self.pos += n - 1;
                    }
                } else {
                    out.push(c);
                }
            }
            self.pos += 1;
        }
    }

    /// What `&name;` stands for (`007070c0`; `007073d0` takes off the `&`,
    /// a `-` after it and the `;`).
    fn entity(&self, entity: &[u8]) -> Option<String> {
        let mut name = entity.strip_prefix(b"&").unwrap_or(entity);
        name = name.strip_prefix(b"-").unwrap_or(name);
        name = name.strip_suffix(b";").unwrap_or(name);
        let name = String::from_utf8_lossy(name);
        (self.setting)(&name)
    }
}

/// `sscanf("%i")`: decimal, `0x` hex or `0` octal, after spaces and a
/// sign; `None` when there's no number.
fn scan_int(s: &[u8]) -> Option<i32> {
    let s = std::str::from_utf8(s).ok()?.trim_start();
    let (neg, s) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let (radix, digits) = if s.len() > 1 && (s.starts_with("0x") || s.starts_with("0X")) {
        (16, &s[2..])
    } else if s.starts_with('0') {
        (8, s)
    } else {
        (10, s)
    };
    let end = digits
        .find(|c: char| !c.is_digit(radix))
        .unwrap_or(digits.len());
    if end == 0 {
        return None;
    }
    let v = i64::from_str_radix(&digits[..end], radix).ok()?;
    Some(if neg { -v } else { v } as i32)
}

/// `atoi`.
fn atoi(s: &[u8]) -> i32 {
    let s = String::from_utf8_lossy(s);
    let s = s.trim_start();
    let (neg, s) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let v: i64 = s[..end].parse().unwrap_or(0);
    (if neg { -v } else { v }) as i32
}

/// A hex digit as the parser reads it: below `A` as a digit, else from
/// `A` (upper-cased); a pair is then kept within 0 to 255.
fn hex_pair(hi: u8, lo: u8) -> f32 {
    let digit = |c: u8| -> i32 {
        let c = i32::from(c.to_ascii_uppercase());
        if c < 0x41 {
            c - 0x30
        } else {
            c - 0x37
        }
    };
    (digit(hi) * 16 + digit(lo)).clamp(0, 255) as f32 / 255.0
}

fn upper(v: &mut [u8]) {
    v.make_ascii_uppercase();
}

/// Parses `text` and lays it out (`00a17390`), or `None` when it doesn't
/// start (after whitespace) with `<` or `{`. `wrap` is the wrap width and
/// `limit` the page height (both [`UNLIMITED`] for none);
/// `setting` gives a game setting's or control's text.
pub fn parse(
    fonts: &[Option<Font>],
    text: &[u8],
    wrap: i32,
    limit: i32,
    setting: &dyn Fn(&str) -> Option<String>,
) -> Option<Layout> {
    let mut r = Reader {
        text,
        pos: 0,
        setting,
    };
    let (_, first) = r.collect(0, 4, false);
    if first != 1 {
        return None;
    }
    let mut layout = Layout::new(wrap, limit, true);
    r.pos = 0;
    let mut style = Element::new(fonts, 0, b' ', DEFAULT_COLOR, LEFT);
    let mut breaks = 0;
    let mut new_page = false;
    let mut picture_waiting = false;
    let flush_picture = |layout: &mut Layout,
                         style: &mut Element,
                         breaks: &mut i32,
                         new_page: &mut bool,
                         waiting: &mut bool| {
        *waiting = false;
        style.code = 0;
        layout.append(fonts, style.clone(), *breaks, *new_page);
        *breaks = 0;
        *new_page = false;
        style.picture.clear();
    };
    loop {
        let (run, mut kind) = r.collect(1, 0, true);
        if !run.is_empty() {
            if picture_waiting {
                flush_picture(
                    &mut layout,
                    &mut style,
                    &mut breaks,
                    &mut new_page,
                    &mut picture_waiting,
                );
            }
            for &c in &run {
                style.set_code(fonts, c);
                layout.append(fonts, style.clone(), breaks, new_page);
                breaks = 0;
                new_page = false;
            }
        }
        if kind & 0x20 != 0 {
            break;
        }
        let (mut tag, k) = r.collect(6, 0, true);
        kind = k;
        upper(&mut tag);
        if kind & 0x20 != 0 {
            break;
        }
        if kind & 4 != 0 {
            loop {
                let (_, k) = r.collect(0, 4, false);
                if k & 0x22 != 0 {
                    break;
                }
                let (mut name, k) = r.collect(0x16, 0, true);
                kind = k;
                upper(&mut name);
                if kind & 0x22 != 0 {
                    break;
                }
                let value = if kind & 4 == 0 {
                    let (_, k) = r.collect(0, 4, false);
                    kind = k;
                    if kind & 0x22 != 0 {
                        break;
                    }
                    if kind == 8 {
                        r.pos += 1;
                        let (v, _) = r.collect(10, 0, true);
                        let after = r.at(r.pos);
                        r.pos += 1;
                        kind = class(after);
                        v
                    } else {
                        let (mut v, k) = r.collect(6, 0, true);
                        kind = k;
                        upper(&mut v);
                        v
                    }
                } else {
                    b"true".to_vec()
                };
                if tag == b"IMG" {
                    picture_waiting = true;
                    match name.as_slice() {
                        b"SRC" => style.picture = String::from_utf8_lossy(&value).into_owned(),
                        b"WIDTH" => {
                            if let Some(v) = scan_int(&value) {
                                style.advance = v;
                            }
                        }
                        b"HEIGHT" => {
                            if let Some(v) = scan_int(&value) {
                                style.height = v;
                            }
                            style.depth = 0;
                        }
                        _ => {}
                    }
                } else if picture_waiting {
                    flush_picture(
                        &mut layout,
                        &mut style,
                        &mut breaks,
                        &mut new_page,
                        &mut picture_waiting,
                    );
                }
                if tag == b"DIV" {
                    breaks += 1;
                    if name == b"ALIGN" {
                        match value.as_slice() {
                            b"LEFT" => style.align = LEFT,
                            b"CENTER" => style.align = CENTER,
                            b"RIGHT" => style.align = RIGHT,
                            _ => {}
                        }
                    }
                } else if tag == b"FONT" {
                    if name == b"FACE" {
                        for (i, file) in DEFAULT_FONT_FILES.iter().enumerate() {
                            if value.eq_ignore_ascii_case(file.as_bytes())
                                || atoi(&value) == i as i32 + 1
                            {
                                style.font = i;
                                let code = style.code;
                                style.set_code(fonts, code);
                                break;
                            }
                        }
                    }
                    if name == b"COLOR" && value.len() == 6 {
                        style.color = [
                            hex_pair(value[0], value[1]),
                            hex_pair(value[2], value[3]),
                            hex_pair(value[4], value[5]),
                            1.0,
                        ];
                    }
                }
                if kind & 0x22 != 0 {
                    break;
                }
            }
        }
        match tag.as_slice() {
            b"BR" => breaks += 1,
            b"P" => breaks += 2,
            b"HR" => {
                if limit == UNLIMITED {
                    breaks += 2;
                } else {
                    new_page = true;
                    breaks = 0;
                }
            }
            b"/FONT" => style.reset(fonts),
            _ => {}
        }
    }
    Some(layout)
}

/// Text that isn't HTML in an `ishtml` tile (`00a18a30`'s own loop): each
/// character in the tile's font, colour and justification (curly quotes
/// straight), `\n` a line break, tabs and characters from space up;
/// no text is one space; no settings put in.
pub fn plain(
    fonts: &[Option<Font>],
    text: &[u8],
    font: usize,
    justify: i32,
    wrap: i32,
    limit: i32,
) -> Layout {
    let mut layout = Layout::new(wrap, limit, false);
    let mut style = Element::new(fonts, 0, b' ', DEFAULT_COLOR, LEFT);
    style.font = font.min(8);
    style.align = justify;
    let text: &[u8] = if text.is_empty() { b" " } else { text };
    let mut breaks = 0;
    for &raw in text {
        let c = straight_quotes(raw);
        if c == b'\n' {
            breaks += 1;
        } else if c == b'\t' || c >= 0x20 {
            style.set_code(fonts, c);
            layout.append(fonts, style.clone(), breaks, false);
            breaks = 0;
        }
    }
    layout
}

/// A picture in the text: where (x, y, width, height from the tile's
/// origin, y down) and its file.
#[derive(Debug, Clone, PartialEq)]
pub struct Picture {
    pub file: String,
    pub rect: [f32; 4],
}

/// The glyphs and pictures of page `page` (`00a19060`): each line sits its
/// height and gap below the one before (the first below the tile's top);
/// right-aligned lines end at the wrap width (0 for the fallback's),
/// centred ones sit in the middle; each glyph is placed as the plain
/// text's are (`00a142d0`: left kerning, then up from the line by its
/// baseline); a picture stands on the line, its height and depth tall.
/// Each glyph comes with its font (from 1).
pub fn draw(
    fonts: &[Option<Font>],
    layout: &Layout,
    page: i32,
) -> (Vec<(usize, GlyphQuad)>, Vec<Picture>) {
    let mut quads = Vec::new();
    let mut pictures = Vec::new();
    let Some(page) = usize::try_from(page).ok().and_then(|p| layout.pages.get(p)) else {
        return (quads, pictures);
    };
    let base = if layout.parsed { layout.wrap } else { 0 };
    let mut z = 0.0f32;
    for line in &page.lines {
        let start = match line.align {
            RIGHT => base.saturating_sub(line.width),
            CENTER => base.saturating_sub(line.width) / 2,
            _ => 0,
        } as f32;
        z -= (line.height + line.gap) as f32;
        for e in &line.elements {
            let x = e.x as f32 + start;
            if !e.picture.is_empty() {
                let h = (e.height + e.depth) as f32;
                pictures.push(Picture {
                    file: e.picture.clone(),
                    rect: [x, -(z + h), e.advance as f32, h],
                });
                continue;
            }
            let Some(f) = font_at(fonts, e.font) else {
                continue;
            };
            let g = &f.glyphs[usize::from(e.code)];
            let left = x + g.kern_left;
            let top = z + g.baseline;
            quads.push((
                e.font + 1,
                GlyphQuad {
                    texture: g.texture,
                    left,
                    right: left + g.width,
                    top: -top,
                    bottom: -(top - g.height),
                    uv: g.uv,
                },
            ));
        }
    }
    (quads, pictures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::tests::font_bytes;

    /// Font 1 in miniature: line height 31, letters 17 wide with right
    /// kerning -2 (advance 15), the space 14, '-' 11 (advance 9),
    /// baselines 18, 'g' reaching 10 below (lowest -10).
    fn font1() -> Font {
        let mut glyphs = vec![
            (b' ', 0.0, 0.0, 0.0, 14.0, 0.0),
            (b'-', 11.0, 10.0, 0.0, -2.0, 12.0),
            (b'.', 5.0, 5.0, 0.0, -2.0, 5.0),
            (b'g', 17.0, 28.0, 0.0, -2.0, 18.0),
        ];
        for c in b'A'..=b'Z' {
            glyphs.push((c, 17.0, 23.0, 0.0, -2.0, 18.0));
        }
        Font::parse(&font_bytes(31.0, &glyphs)).unwrap()
    }

    /// Font 3: line height 40, letters 20 wide (advance 20).
    fn font3() -> Font {
        let mut glyphs = vec![(b' ', 0.0, 0.0, 0.0, 10.0, 0.0)];
        for c in b'A'..=b'Z' {
            glyphs.push((c, 20.0, 30.0, 0.0, 0.0, 30.0));
        }
        Font::parse(&font_bytes(40.0, &glyphs)).unwrap()
    }

    fn fonts() -> Vec<Option<Font>> {
        let mut f = vec![None; 8];
        f[0] = Some(font1());
        f[2] = Some(font3());
        f
    }

    fn none(_: &str) -> Option<String> {
        None
    }

    fn lines(l: &Layout, page: usize) -> Vec<String> {
        l.pages[page]
            .lines
            .iter()
            .map(|line| line.elements.iter().map(|e| e.code as char).collect())
            .collect()
    }

    #[test]
    fn only_text_starting_with_a_tag_is_parsed() {
        let f = fonts();
        assert!(parse(&f, b"Plain text", 500, UNLIMITED, &none).is_none());
        assert!(parse(&f, b"", 500, UNLIMITED, &none).is_none());
        // Whitespace first is skipped for the test, then laid out.
        let l = parse(&f, b" \r\n<p>AB", 500, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), [" ", "AB"]);
        assert!(l.parsed);
    }

    #[test]
    fn breaks_paragraphs_and_their_gaps() {
        let f = fonts();
        let l = parse(&f, b"<div>\r\nAB<br>CD<p>EF", 500, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["AB", "CD", "EF"]);
        let p = &l.pages[0];
        // Font 1's line height from the table (35); advance 15 a letter.
        assert_eq!(
            p.lines
                .iter()
                .map(|l| (l.width, l.height))
                .collect::<Vec<_>>(),
            [(30, 35), (30, 35), (30, 35)]
        );
        // A paragraph's second break adds the last letter's line height
        // (31) plus its height below the baseline (23 - 18).
        assert_eq!(p.lines[2].gap, 36);
        assert_eq!(p.lines[1].gap, 0);
        assert_eq!(l.size(0), (30, 35 * 3 + 36));
        // A `DIV` breaks once per attribute (none here).
        let l = parse(&f, b"<div>AB<div align=center>CD", 500, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["AB", "CD"]);
        assert_eq!(l.pages[0].lines[1].align, CENTER);
    }

    #[test]
    fn words_wrap_at_spaces_and_long_ones_get_a_hyphen() {
        let f = fonts();
        // A space past the wrap width (40) becomes a character 0 starting
        // the next line.
        let l = parse(&f, b"<p>AB CD EF", 40, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["AB", "\0CD", "\0EF"]);
        // A letter past it (60) takes its word down: "AB C" is 59 wide;
        // the line keeps C's width (the game's arithmetic takes off only
        // the space's).
        let l = parse(&f, b"<p>AB CDE", 60, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["AB", "CDE"]);
        assert_eq!(l.pages[0].lines[0].width, 45);
        // No space: letters move down until a hyphen (9) fits.
        let l = parse(&f, b"<p>ABCDEF", 60, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["ABC-", "DEF"]);
        assert_eq!(l.pages[0].lines[0].width, 45 + 9);
    }

    #[test]
    fn fonts_colours_alignment_and_settings() {
        let f = fonts();
        let setting = |n: &str| (n == "sName").then(|| "XY".to_string());
        let l = parse(
            &f,
            b"<div align=\"right\"><font face=3 color=FF8000>A&sName;</font>B &nope; C & D",
            500,
            UNLIMITED,
            &setting,
        )
        .unwrap();
        let p = &l.pages[0];
        let text: String = p.lines[0].elements.iter().map(|e| e.code as char).collect();
        // A setting's text goes in, an unknown one stays; a lone & goes.
        assert_eq!(text, "AXYB &nope; C  D");
        let a = &p.lines[0].elements[0];
        assert_eq!((a.font, a.advance, a.height), (2, 20, 40));
        assert_eq!(a.color, [1.0, 128.0 / 255.0, 0.0, 1.0]);
        // Quoted values aren't upper-cased: "right" isn't RIGHT.
        assert_eq!(a.align, LEFT);
        // After `/FONT`, font 1 and the default colour again.
        let b = &p.lines[0].elements[3];
        assert_eq!((b.font, b.color), (0, DEFAULT_COLOR));
        // The line starts at font 3's table height (30) and grows to font
        // 1's own line height (31) where the font changes.
        assert_eq!(p.lines[0].height, 31);
        let l = parse(&f, b"<div align=right>AB", 500, UNLIMITED, &none).unwrap();
        let (quads, _) = draw(&f, &l, 0);
        // Right-aligned within the wrap width: 500 - 30.
        assert_eq!(quads[0].1.left, 470.0);
        assert_eq!(quads[0].0, 1);
    }

    #[test]
    fn pages_from_the_page_height_and_rules() {
        let f = fonts();
        // Pages 80 tall: two 35-high lines each.
        let l = parse(&f, b"<p>A<br>B<br>C<br>D<br>E", 500, 80, &none).unwrap();
        assert_eq!(l.pages.len(), 3);
        assert_eq!(lines(&l, 1), ["C", "D"]);
        assert_eq!(l.size(1), (15, 70));
        // No such page: the wrap width and page height.
        assert_eq!(l.size(5), (500, 80));
        // `HR` is two breaks without a page height, else a new page.
        let l = parse(&f, b"<p>A<hr>B", 500, UNLIMITED, &none).unwrap();
        assert_eq!(lines(&l, 0), ["A", "B"]);
        let l = parse(&f, b"<p>A<hr>B", 500, 400, &none).unwrap();
        assert_eq!(l.pages.len(), 2);
    }

    #[test]
    fn pictures_take_their_size() {
        let f = fonts();
        let l = parse(
            &f,
            b"<p>A<img src=\"x/y.dds\" width=40 height=0x20>B",
            500,
            UNLIMITED,
            &none,
        )
        .unwrap();
        let p = &l.pages[0];
        assert_eq!(p.lines.len(), 1);
        let pic = &p.lines[0].elements[1];
        assert_eq!(
            (pic.code, pic.advance, pic.height, pic.depth),
            (0, 40, 32, 0)
        );
        assert_eq!(p.lines[0].elements[2].x, 55);
        let (_, pictures) = draw(&f, &l, 0);
        assert_eq!(pictures[0].file, "x/y.dds");
        // Standing on the line: the line is 35 down, the picture 32 tall.
        assert_eq!(pictures[0].rect, [15.0, 3.0, 40.0, 32.0]);
    }

    #[test]
    fn glyphs_sit_on_their_lines() {
        let f = fonts();
        let l = parse(&f, b"<p>A<br>g", 500, UNLIMITED, &none).unwrap();
        let (quads, _) = draw(&f, &l, 0);
        // Line 1's base is 35 down; 'A' reaches 18 above it.
        assert_eq!((quads[0].1.top, quads[0].1.bottom), (17.0, 40.0));
        // Line 2 another 35 down; 'g' from 18 above to 10 below.
        assert_eq!((quads[1].1.top, quads[1].1.bottom), (52.0, 80.0));
    }

    #[test]
    fn the_fallback_lays_out_each_character() {
        let f = fonts();
        let l = plain(&f, b"AB\nC\x93", 2, RIGHT, 500, UNLIMITED);
        assert!(!l.parsed);
        let p = &l.pages[0];
        assert_eq!(p.lines.len(), 2);
        assert_eq!(p.lines[1].elements[1].code, b'"');
        assert_eq!(p.lines[0].elements[0].font, 2);
        // Right-justified from 0, as the plain text is.
        let (quads, _) = draw(&f, &l, 0);
        assert_eq!(quads[0].1.left, -40.0);
        assert_eq!(
            plain(&f, b"", 0, LEFT, 500, UNLIMITED).pages[0].lines[0].width,
            14
        );
    }

    #[test]
    fn character_classes_and_numbers() {
        assert_eq!(class(0), 0x20);
        assert_eq!(class(b'{'), 1);
        assert_eq!(class(b'}'), 2);
        assert_eq!(class(b'\t'), 4);
        assert_eq!(class(0xE9), 4);
        assert_eq!(class(b'a'), 0);
        assert_eq!(scan_int(b"0x20"), Some(32));
        assert_eq!(scan_int(b"010"), Some(8));
        assert_eq!(scan_int(b"-7px"), Some(-7));
        assert_eq!(scan_int(b"px"), None);
        assert_eq!(atoi(b"3x"), 3);
        assert_eq!(hex_pair(b'Z', b'Z'), 255.0 / 255.0);
    }
}
