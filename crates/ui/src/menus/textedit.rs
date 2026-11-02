//! Typing a name (`menus\dialog\texteditmenu.xml`, class `TextEditMenu`
//! 1051, vtable `01070034` in FalloutNV.exe), as the player's name is asked
//! for (`ShowNameMenu`, `GetPlayerName`: `005da540` → the name menu
//! `007ab690` → `007e6320(sEnterName, the player's name, 007ab820)`). Read
//! from the code:
//!
//! * opening (`007e6320`): the prompt (id 2), the text (id 0) the name so
//!   far; the longest the text may measure is the text tile's `wrapwidth`
//!   less 5 (`00716aa0`); typing on, the cursor at the end, and the first
//!   key replacing the whole text (`007e6580(1)`); OK (id 1) `sOk`.
//! * what's shown (`007e6700`, `007170a0`): the text with the cursor put in
//!   at its place, character 127 or "|" by turns every 500 ms (`00717050`),
//!   " " when that leaves nothing; OK can be used when the name passes
//!   (`007ab820`: something besides spaces, no "\" or "~", and every
//!   character drawn by fonts 1 and 7 — a glyph of no width isn't).
//! * keys (`007e6620`, `00716b00`): Enter presses OK when it can be used;
//!   Backspace and Delete take out the character before / at the cursor
//!   (or everything, while the first-key replacing is on); Left, Right,
//!   Home, End move the cursor; Page Up/Down and the arrows only end the
//!   replacing; Tab is ignored; any other character (replacing everything
//!   first when that's on) goes in at the cursor if the text still measures
//!   no wider than the most (font 1, `00717230`).
//! * OK (`007e66a0`): the text is the answer; the name menu then sets the
//!   player's name (`007ab9a0`).

use crate::menu::{key, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\dialog\\texteditmenu.xml";
/// Its class number.
pub const CLASS: i32 = 1051;
/// How long the cursor shows each way (`00717050`), in milliseconds.
pub const BLINK_MS: f64 = 500.0;

/// The name menu's test (`007ab820`): at least one character that isn't a
/// space, no "\" or "~", and every character (curly quotes as straight
/// ones) drawn with some width by fonts 1 and 7.
pub fn valid_name(ui: &Ui, text: &[u8]) -> bool {
    let mut seen = false;
    for &c in text {
        let c = crate::font::straight_quotes(c);
        if c == b'\\' || c == b'~' {
            return false;
        }
        for index in [0usize, 6] {
            if let Some(Some(font)) = ui.fonts.get(index) {
                if font.glyphs[usize::from(c)].width == 0.0 {
                    return false;
                }
                seen |= c != b' ';
            }
        }
    }
    seen
}

/// The text edit menu.
#[derive(Debug)]
pub struct TextEditMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..2 (`+0x28`): the text, OK, the prompt.
    pub tiles: [Option<TileId>; 3],
    /// The text (Windows-1252 bytes), the cursor (`+0x10`).
    pub text: Vec<u8>,
    pub cursor: usize,
    /// Typing on (`+0x21`), the cursor's turn (`+0x20`), the first key
    /// replacing everything (`+0x22`).
    pub active: bool,
    pub blink: bool,
    pub replace: bool,
    /// The widest the text may measure (`+0x14`), -1 for any.
    pub most_width: i32,
    last_blink: f64,
    /// The answer, once OK is pressed.
    pub answer: Option<String>,
    pub closed: bool,
}

impl TextEditMenu {
    pub fn new(menu: TileId) -> TextEditMenu {
        TextEditMenu {
            menu,
            tiles: [None; 3],
            text: Vec::new(),
            cursor: 0,
            active: false,
            blink: false,
            replace: false,
            most_width: -1,
            last_blink: 0.0,
            answer: None,
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    /// Opens it (`007e6320`) with a prompt and the text so far.
    pub fn open(&mut self, ui: &mut Ui, prompt: &str, text: &str, now_ms: f64) -> bool {
        let (Some(text_tile), Some(ok), Some(prompt_tile)) =
            (self.tile(0), self.tile(1), self.tile(2))
        else {
            return false;
        };
        ui.set_text(prompt_tile, t::STRING, prompt);
        ui.set_text(text_tile, t::STRING, text);
        let wrap = ui.number(text_tile, t::WRAPWIDTH) as i32;
        self.most_width = wrap - 5;
        self.text = text.bytes().collect();
        self.active = true;
        self.cursor = self.text.len();
        self.replace = true;
        self.last_blink = now_ms;
        let s = ui.setting_text("sOk").unwrap_or_default();
        ui.set_text(ok, t::STRING, &s);
        self.show(ui);
        ui.set_number(self.menu, crate::menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    /// The text with the cursor (`007170a0`), and OK's `target`
    /// (`007e6700`).
    fn show(&mut self, ui: &mut Ui) {
        let mut shown: Vec<u8> = Vec::new();
        for i in 0..=self.text.len() {
            if self.active && i == self.cursor {
                shown.push(if self.blink { b'|' } else { 127 });
            }
            if let Some(&c) = self.text.get(i) {
                shown.push(c);
            }
        }
        if shown.is_empty() {
            shown.push(b' ');
        }
        if let Some(tile) = self.tile(0) {
            let s: String = shown.iter().map(|&b| char::from(b)).collect();
            ui.set_text(tile, t::STRING, &s);
        }
        if let Some(ok) = self.tile(1) {
            let valid = valid_name(ui, &self.text);
            ui.set_number(ok, t::TARGET, if valid { 1.0 } else { 0.0 });
        }
        ui.refresh();
    }

    /// Every frame (`007e65f0`): the cursor's turn changes every 500 ms.
    pub fn update(&mut self, ui: &mut Ui, now_ms: f64) {
        if !self.active {
            return;
        }
        if now_ms - self.last_blink > BLINK_MS {
            self.blink = !self.blink;
            self.last_blink = now_ms;
        }
        self.show(ui);
    }

    /// Whether the text measures no wider than the most (`00717230`).
    fn fits(&self, ui: &Ui, text: &[u8]) -> bool {
        if self.most_width == -1 {
            return true;
        }
        let Some(Some(font)) = ui.fonts.first() else {
            return true;
        };
        let m = crate::text::measure(font, 1, text, f32::MAX, 0);
        (m.width as i32) <= self.most_width
    }

    /// A key while typing (`00716b00`).
    fn edit(&mut self, ui: &Ui, code: u32) {
        let clear = |s: &mut Self| {
            s.text.clear();
            s.cursor = 0;
            s.replace = false;
        };
        match code {
            key::BACKSPACE => {
                if self.replace {
                    clear(self);
                } else if self.cursor > 0 {
                    self.text.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            key::LEFT => {
                self.cursor = self.cursor.saturating_sub(1);
                self.replace = false;
            }
            key::RIGHT => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                }
                self.replace = false;
            }
            key::HOME => {
                self.cursor = 0;
                self.replace = false;
            }
            key::END => {
                self.cursor = self.text.len();
                self.replace = false;
            }
            key::DELETE => {
                if self.replace {
                    clear(self);
                } else if self.cursor < self.text.len() {
                    self.text.remove(self.cursor);
                }
            }
            key::ENTER => {
                if self.replace {
                    clear(self);
                } else {
                    self.active = false;
                }
            }
            key::UP | key::DOWN | key::PAGE_UP | key::PAGE_DOWN => self.replace = false,
            9 => {}
            c if c < 0x100 => {
                if self.replace {
                    clear(self);
                }
                let mut next = self.text.clone();
                next.insert(self.cursor, c as u8);
                if self.fits(ui, &next) {
                    self.text = next;
                    self.cursor += 1;
                }
            }
            _ => {}
        }
    }
}

impl MenuCode for TextEditMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..3).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007e66a0`: OK gives the text as the answer and closes.
    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        if id != 1 || self.closed {
            return;
        }
        let s: String = self.text.iter().map(|&b| char::from(b)).collect();
        self.answer = Some(s);
        ui.set_number(self.menu, crate::menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }

    /// `007e6620`.
    fn key(&mut self, ui: &mut Ui, code: u32, now: f64) -> bool {
        if code == key::ENTER {
            let usable = self
                .tile(1)
                .is_some_and(|ok| ui.number(ok, t::TARGET) != 0.0);
            if usable {
                self.click(ui, 1, self.tile(1), now);
            }
            return usable;
        }
        if !self.active {
            return false;
        }
        self.edit(ui, code);
        self.show(ui);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn opened(name: &str) -> (Ui, TextEditMenu) {
        let mut ui = test_support::ui();
        let mut m = TextEditMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::text_edit_menu(), &mut m);
        assert!(m.open(&mut ui, "Enter character name.", name, 0.0));
        (ui, m)
    }

    fn shown(ui: &mut Ui, m: &TextEditMenu) -> String {
        ui.string(m.tiles[0].unwrap(), t::STRING)
            .unwrap_or_default()
    }

    /// `007e6320`, `007170a0`, `00716b00`, `007e6620`, `007ab820`: the
    /// first key replaces the name, the cursor shows by turns, editing keys,
    /// the width limit, OK only for a valid name.
    #[test]
    fn typing_a_name() {
        let (mut ui, mut m) = opened("Courier");
        assert_eq!(shown(&mut ui, &m), "Courier\u{7f}");
        let ok = m.tiles[1].unwrap();
        assert_eq!(ui.number(ok, t::TARGET), 1.0);
        assert_eq!(ui.string(ok, t::STRING).as_deref(), Some("Ok"));
        m.update(&mut ui, 501.0);
        assert_eq!(shown(&mut ui, &m), "Courier|");
        // The first key replaces everything.
        for c in b"Al" {
            assert!(m.key(&mut ui, u32::from(*c), 0.0));
        }
        assert_eq!(m.text, b"Al");
        m.key(&mut ui, key::LEFT, 0.0);
        m.key(&mut ui, u32::from(b'x'), 0.0);
        assert_eq!(m.text, b"Axl");
        m.key(&mut ui, key::BACKSPACE, 0.0);
        m.key(&mut ui, key::HOME, 0.0);
        m.key(&mut ui, key::DELETE, 0.0);
        assert_eq!(m.text, b"l");
        // Only spaces, or a backslash, can't be a name.
        m.key(&mut ui, key::DELETE, 0.0);
        m.key(&mut ui, u32::from(b' '), 0.0);
        assert_eq!(ui.number(ok, t::TARGET), 0.0);
        m.key(&mut ui, u32::from(b'\\'), 0.0);
        assert_eq!(ui.number(ok, t::TARGET), 0.0);
        assert!(!m.key(&mut ui, key::ENTER, 0.0));
        m.key(&mut ui, key::BACKSPACE, 0.0);
        // The test font's letters are 10 wide: 245 fits 24 of them.
        for _ in 0..30 {
            m.key(&mut ui, u32::from(b'a'), 0.0);
        }
        assert_eq!(m.text.len(), 25);
        assert!(m.key(&mut ui, key::ENTER, 0.0));
        assert!(m.closed);
        assert_eq!(m.answer.as_deref().map(str::len), Some(25));
    }
}
