//! The message box (`menus\message_menu.xml`, class `MessageMenu` 1001,
//! vtable `0107566c` in FalloutNV.exe): a script's `ShowMessage` of a
//! message box (`005b4630`), and the game's own questions. Read from the
//! code:
//!
//! * a box is queued by `007a8e60` (the `ShowMessageBox` wrappers
//!   `00703e80` / `00703f10` / `00704010`) with its text, icon, title,
//!   sound, the number its first button answers with, its kind, its buttons
//!   and some layout values; a box that waits for an answer goes before the
//!   queued ones that don't; the menu shows the first;
//! * buttons: the letter after a `&` in a button is its key (the `&` taken
//!   out); a button that is a single space keeps its number but isn't
//!   shown (`ShowMessage` makes the buttons whose conditions fail into
//!   that); a box with no button left gets "OK";
//! * filling it (`007a92e0`, the tiles from the disassembly): the text in
//!   `MM_MessageText`, the title in `MM_Title` (hidden without one), the
//!   buttons as `MM_ButtonTemplate` items of the list `MM_ButtonList` (each
//!   with `id` 7), the icon in `MM_MessageIcon`; then sizes: the button list
//!   as high as its items and as wide as the widest button's text + 80
//!   (both kept under the file's limits), the text wrapped at
//!   `_MaxMenuWidth` less 2 × `_horbuf` (and 120 with an icon), one line
//!   centred and more left-justified (the title too), the box as wide as the
//!   widest of title (+ 100), text (+ 2 × `_horbuf`, + 120 with an icon) and
//!   buttons (+ 2 × `_horbuf`), then kept between `_MinMenuWidth` and
//!   `_MaxMenuWidth`; its background as opaque as `fMenuBackgroundOpacity`
//!   (`fPopUpBackgroundOpacity` when another menu is open);
//! * a click on a button (`007aa070`, id 7) stores its number for
//!   `GetButtonPressed` (`00703fa0` reads it once, then -1) and shows the
//!   next box or closes; a typed key clicks the button with that letter
//!   (`007aa110`), and the Escape control the last button (unless the box is
//!   a script's, kind 0x17).

use std::collections::VecDeque;

use crate::list::{font_for, set_keeping_operators, ListBox};
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::text;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\message_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1001;
/// The template its buttons are made from.
pub const BUTTON_TEMPLATE: &str = "MM_ButtonTemplate";
/// The kind a script's message box has (`005b4630` passes 0x17).
pub const SCRIPT_KIND: i32 = 0x17;
/// The sound a box opens with when it names none.
pub const OPEN_SOUND: &str = "UIPopUpMessageWindow";
/// The button a box without any gets (`0103934c`).
pub const DEFAULT_BUTTON: &str = "OK";

/// A message box (the game's 0x50-byte record, `007a8e60`).
#[derive(Debug, Clone, PartialEq)]
pub struct MessageBox {
    /// `+0x0c`.
    pub text: String,
    /// `+0x00`: a picture to show beside the text.
    pub icon: Option<String>,
    /// `+0x08`.
    pub title: Option<String>,
    /// `+0x30`: the sound it opens with (its editor ID).
    pub sound: Option<String>,
    /// `+0x1c`: the number the first button answers with.
    pub first_number: i32,
    /// `+0x2c`.
    pub kind: i32,
    /// `+0x14`: the buttons in order; empty strings keep a number but show
    /// nothing.
    pub buttons: Vec<String>,
    /// `+0x20`: the buttons' key letters (one per button that has a `&`).
    pub shortcuts: Vec<u8>,
    /// `+0x38`: whether it waits for an answer (every caller in the game
    /// passes true).
    pub modal: bool,
    /// `+0x48`: its background's alpha (0..255).
    pub background_alpha: f32,
    /// `+0x4c`: a width in pixels (0: as wide as it needs).
    pub width: u32,
    /// `+0x28`, the callback: who hears the answer (`None`: a script's
    /// `GetButtonPressed`; else the menu that asked, by a number of the
    /// caller's choosing).
    pub owner: Option<u32>,
}

impl MessageBox {
    /// A box as `007a8e60` makes it from its buttons: each `&` taken out
    /// and the letter after it noted as the button's key; a button that is
    /// exactly one space becomes an empty (hidden) one.
    /// `background_alpha` is the menus' `_background_fill_alpha`.
    pub fn new(
        text: &str,
        title: Option<&str>,
        buttons: &[&str],
        kind: i32,
        background_alpha: f32,
    ) -> MessageBox {
        let mut list = Vec::new();
        let mut shortcuts = Vec::new();
        for &b in buttons {
            let mut b = b.to_string();
            if let Some(at) = b.find('&') {
                if let Some(&c) = b.as_bytes().get(at + 1) {
                    shortcuts.push(c);
                }
                b.remove(at);
            }
            list.push(if b == " " { String::new() } else { b });
        }
        MessageBox {
            text: text.to_string(),
            icon: None,
            title: title.filter(|t| !t.is_empty()).map(str::to_string),
            sound: None,
            first_number: 0,
            kind,
            buttons: list,
            shortcuts,
            modal: true,
            background_alpha,
            width: 0,
            owner: None,
        }
    }

    /// What a script's `ShowMessage` of a message box makes (`005b4630`):
    /// the text (with the values put in), the message's `FULL` name as the
    /// title, up to ten buttons with the ones whose conditions fail as
    /// blanks (`None`), "OK" when none is left, kind 0x17.
    pub fn script(
        text: &str,
        title: Option<&str>,
        buttons: &[Option<String>],
        background_alpha: f32,
    ) -> MessageBox {
        let mut list: Vec<String> = buttons
            .iter()
            .take(10)
            .map(|b| b.clone().unwrap_or_else(|| " ".to_string()))
            .collect();
        if buttons.iter().take(10).all(Option::is_none) {
            if list.is_empty() {
                list.push(DEFAULT_BUTTON.to_string());
            } else {
                list[0] = DEFAULT_BUTTON.to_string();
            }
        }
        let refs: Vec<&str> = list.iter().map(String::as_str).collect();
        MessageBox::new(text, title, &refs, SCRIPT_KIND, background_alpha)
    }
}

/// The message box menu.
#[derive(Debug)]
pub struct MessageMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..5 (`007a8ab0`): `MM_MainRect`, `MM_Title`,
    /// `MM_MessageIcon`, `MM_MessageText`, `MM_ButtonList`, `MM_Background`.
    pub tiles: [Option<TileId>; 6],
    pub list: ListBox,
    /// The boxes waiting; the first is on screen.
    pub queue: VecDeque<MessageBox>,
    /// The button last pressed (the interface manager's `+0xe4`) and the
    /// box's owner.
    pub pressed: Option<i32>,
    pub pressed_owner: Option<u32>,
    /// Sounds for the caller to play (editor IDs).
    pub sounds: Vec<String>,
    /// Closed: the caller takes the menu off the screen.
    pub closed: bool,
}

impl MessageMenu {
    /// A message menu for a menu tile loaded with [`menu::load`]; `tiles`
    /// filled by [`MenuCode::set_tile`].
    pub fn new(menu: TileId) -> MessageMenu {
        MessageMenu {
            menu,
            tiles: [None; 6],
            list: ListBox::default(),
            queue: VecDeque::new(),
            pressed: None,
            pressed_owner: None,
            sounds: Vec::new(),
            closed: false,
        }
    }

    /// Queues a box (`007a8e60`): one that waits for an answer goes before
    /// the first queued box that doesn't.
    pub fn push(&mut self, b: MessageBox) {
        if b.modal {
            if let Some(at) = self.queue.iter().skip(1).position(|q| !q.modal) {
                self.queue.insert(at + 1, b);
                return;
            }
        }
        self.queue.push_back(b);
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles[id]
    }

    /// Opens the menu (`007a8ba0`): the button list set up, the first box
    /// filled in, the background `fPopUpBackgroundOpacity` × 255 when another
    /// menu was open (`popup`), the menu shown.
    pub fn open(&mut self, ui: &mut Ui, popup: Option<f32>) {
        if let Some(list) = self.tile(4) {
            self.list = ListBox::new(ui, list, BUTTON_TEMPLATE);
        }
        self.fill(ui);
        if let (Some(opacity), Some(bg)) = (popup, self.tile(5)) {
            ui.set_number(bg, t::ALPHA, opacity * 255.0);
        }
        ui.refresh();
    }

    /// Fills the menu in for the first queued box (`007a92e0`).
    pub fn fill(&mut self, ui: &mut Ui) {
        let Some(b) = self.queue.front().cloned() else {
            return;
        };
        let menu = self.menu;
        let custom = |ui: &mut Ui, n: &str| ui.names.lookup_or_add(n).unwrap_or(0);
        let horbuf_id = custom(ui, "_horbuf");
        let hb = ui.number(menu, horbuf_id);
        self.sounds
            .push(b.sound.clone().unwrap_or_else(|| OPEN_SOUND.to_string()));
        let measure = |ui: &Ui, font: i32, s: &str, wrap: f32| {
            font_for(ui, font)
                .map(|f| text::measure(&f, font as usize, s.as_bytes(), wrap, 0))
                .unwrap_or(text::Measured {
                    width: 0.0,
                    height: 0.0,
                    lines: 0,
                })
        };
        if let Some(text_tile) = self.tile(3) {
            // An empty text is no text at all (the game's empty strings).
            ui.set_text(text_tile, t::STRING, &b.text);
        }
        // The title (font 6), measured with the box's width or its own
        // wrap width (none: no limit), plus 100.
        let mut title_w = 0.0f32;
        if let Some(title) = self.tile(1) {
            match &b.title {
                Some(s) => {
                    ui.set_string(title, t::STRING, s);
                    ui.set_number(title, t::VISIBLE, 1.0);
                    let mut wrap = if b.width != 0 {
                        b.width as f32
                    } else {
                        ui.number(title, t::WRAPWIDTH)
                    };
                    if wrap == 0.0 {
                        wrap = f32::MAX;
                    }
                    title_w =
                        measure(ui, 6, s, wrap).width + if b.width == 0 { 100.0 } else { 0.0 };
                }
                None => ui.set_number(title, t::VISIBLE, 0.0),
            }
        }
        // The buttons.
        self.list.clear(ui);
        let mut widest = 0i32;
        let mut height = 0.0f32;
        for (i, label) in b.buttons.iter().enumerate() {
            if label.is_empty() {
                continue;
            }
            let Some(tile) = self
                .list
                .add(ui, menu, b.first_number + i as i32, Some(label))
            else {
                continue;
            };
            ui.set_number(tile, t::ID, 7.0);
            if let Some(label_tile) = ui.find_below(tile, "ListItemText") {
                let mut wrap = if b.width != 0 {
                    b.width as f32
                } else {
                    ui.number(label_tile, t::WRAPWIDTH)
                };
                if wrap == 0.0 {
                    wrap = f32::MAX;
                }
                let s = ui.string(label_tile, t::STRING).unwrap_or_default();
                widest = widest.max(measure(ui, 2, &s, wrap).width as i32);
            }
            height += ui.number(tile, t::HEIGHT);
        }
        // The icon.
        let mut text_w = 0.0f32;
        let mut icon_shown = false;
        if let Some(icon) = self.tile(2) {
            match &b.icon {
                Some(path) if !path.is_empty() => {
                    ui.set_string(icon, t::FILENAME, path);
                    ui.set_number(icon, t::VISIBLE, 1.0);
                    icon_shown = true;
                    text_w = if b.width == 0 {
                        text_w + 120.0
                    } else {
                        (text_w + 120.0).min(b.width as f32)
                    };
                }
                _ => ui.set_number(icon, t::VISIBLE, 0.0),
            }
        }
        let mut buttons_w = widest as f32 + 80.0;
        if b.width != 0 {
            buttons_w = buttons_w.min(b.width as f32);
        }
        if let Some(list) = self.tile(4) {
            set_keeping_operators(ui, list, t::HEIGHT, height);
            set_keeping_operators(ui, list, t::WIDTH, buttons_w);
        }
        self.list.refresh(ui);
        buttons_w += 2.0 * hb;
        if b.width != 0 {
            buttons_w = buttons_w.min(b.width as f32);
        }
        // The text: wrapped at the widest the box may be.
        let screen_w = ui.screen_size.width();
        let pixels_w = ui.screen_size.width_px as f32;
        let max_id = custom(ui, "_MaxMenuWidth");
        let min_id = custom(ui, "_MinMenuWidth");
        let max_w = if b.width == 0 {
            ui.number(menu, max_id)
        } else {
            b.width as f32 / pixels_w * screen_w
        };
        let mut wrap = max_w - 2.0 * hb;
        if icon_shown {
            wrap -= 120.0;
        }
        let mut lines = 1;
        if let Some(text_tile) = self.tile(3) {
            ui.set_number(text_tile, t::WRAPWIDTH, wrap);
            let wrap = ui.number(text_tile, t::WRAPWIDTH);
            // No text measures as nothing (`00a1b020` with no string).
            let m = if b.text.is_empty() {
                text::Measured {
                    width: 0.0,
                    height: 0.0,
                    lines: 0,
                }
            } else {
                measure(ui, 2, &b.text, wrap)
            };
            text_w += 2.0 * hb + m.width;
            lines = m.lines;
        }
        // One line centred, more left-justified (`007a9b8a`).
        let justify = if lines as f32 <= 1.0 {
            text::CENTER
        } else {
            text::LEFT
        };
        for id in [1, 3] {
            if let Some(tile) = self.tile(id) {
                ui.set_number(tile, t::JUSTIFY, justify as f32);
            }
        }
        let width = if b.width == 0 {
            let w = title_w.max(text_w).max(buttons_w);
            let w = w.min(ui.number(menu, max_id));
            w.max(ui.number(menu, min_id))
        } else {
            let w = b.width as f32 / pixels_w * screen_w;
            w.min(screen_w - 2.0 * ui.screen_size.safe_x)
        };
        if let Some(main) = self.tile(0) {
            ui.set_number(main, t::WIDTH, width);
            ui.set_number(main, t::ALPHA, if b.modal { 1.0 } else { 0.0 });
        }
        ui.set_number(menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(menu, t::VISIBLE, 1.0);
        if let Some(bg) = self.tile(5) {
            ui.set_number(bg, t::ALPHA, b.background_alpha);
        }
        ui.refresh();
    }

    /// The pressed button's number, once (`00703fa0`), for a script's box.
    pub fn take_pressed(&mut self) -> Option<i32> {
        self.take_pressed_for(None)
    }

    /// The pressed button's number, once, when the box answered was
    /// owner's (its callback's).
    pub fn take_pressed_for(&mut self, owner: Option<u32>) -> Option<i32> {
        if self.pressed.is_some() && self.pressed_owner == owner {
            self.pressed_owner = None;
            return self.pressed.take();
        }
        None
    }

    /// The box on screen answered or dropped: the next one, or the menu
    /// closes (`007aa530`, `007a8df0`).
    fn next(&mut self, ui: &mut Ui) {
        self.queue.pop_front();
        if self.queue.is_empty() {
            ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
            self.closed = true;
        } else {
            self.fill(ui);
        }
    }
}

impl MenuCode for MessageMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..6).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007aa070`: a button (id 7): its number is the answer.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        let tile = match tile {
            Some(t_) => Some(t_),
            None if (0..6).contains(&id) => self.tiles[id as usize],
            None => None,
        };
        if id != 7 {
            return;
        }
        let Some(tile) = tile else {
            return;
        };
        if let Some(n) = self.list.value_of(tile) {
            self.pressed = Some(n);
            self.pressed_owner = self.queue.front().and_then(|b| b.owner);
            self.next(ui);
        }
    }

    /// `007aa110`: the Escape control clicks the last button (not for a
    /// script's box); a key that is a button's letter clicks that button.
    fn key(&mut self, ui: &mut Ui, code: u32, now: f64) -> bool {
        let Some(b) = self.queue.front() else {
            return false;
        };
        if code == menu::key::ESCAPE_TO_MESSAGE && b.kind != SCRIPT_KIND {
            let last = self.list.items.len() as i32 - 1;
            let tile = self.list.item_at(ui, last);
            self.click(ui, 7, tile, now);
            return false;
        }
        if code < 0x100 {
            let c = (code as u8).to_ascii_uppercase();
            if let Some(i) = b.shortcuts.iter().position(|s| s.to_ascii_uppercase() == c) {
                let tile = self.list.item_at(ui, i as i32);
                self.click(ui, 7, tile, now);
            }
        }
        false
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.list]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buttons_keys_and_blanks() {
        let b = MessageBox::new("Pick", None, &["&Yes", "No", " "], 0, 204.0);
        assert_eq!(b.buttons, ["Yes", "No", ""]);
        assert_eq!(b.shortcuts, b"Y");
        // A script's box: failed buttons are blanks that keep their number;
        // none left gives "OK".
        let s = MessageBox::script("Who?", Some("Title"), &[None, Some("Ma'am".into())], 204.0);
        assert_eq!(s.buttons, ["", "Ma'am"]);
        assert_eq!(s.kind, SCRIPT_KIND);
        assert_eq!(s.title.as_deref(), Some("Title"));
        let none = MessageBox::script("Hi", Some(""), &[None, None], 204.0);
        assert_eq!(none.buttons, ["OK", ""]);
        assert_eq!(none.title, None);
        let empty = MessageBox::script("Hi", None, &[], 204.0);
        assert_eq!(empty.buttons, ["OK"]);
    }

    #[test]
    fn boxes_that_wait_go_first() {
        let mut m = MessageMenu::new(0);
        let mut a = MessageBox::new("a", None, &["OK"], 0, 0.0);
        a.modal = false;
        let mut b = a.clone();
        b.text = "b".into();
        let c = MessageBox::new("c", None, &["OK"], 0, 0.0);
        m.push(a);
        m.push(b);
        m.push(c);
        // The first stays on screen; the waiting one goes before "b".
        let order: Vec<&str> = m.queue.iter().map(|q| q.text.as_str()).collect();
        assert_eq!(order, ["a", "c", "b"]);
    }

    use crate::menus::test_support;

    fn opened(b: MessageBox) -> (Ui, MessageMenu) {
        let mut ui = test_support::ui();
        let mut m = MessageMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::message_menu(), &mut m);
        m.push(b);
        m.open(&mut ui, None);
        (ui, m)
    }

    /// `007a92e0`'s sizes with the test font (letters 10 wide, the space
    /// 5): the buttons as wide as the widest text + 80 and each as high as
    /// its text (20) + 20; one line of text centred; the box as wide as the
    /// widest part, at least `_MinMenuWidth`.
    #[test]
    fn filling_the_box_as_the_game_measures_it() {
        let (mut ui, m) = opened(MessageBox::new("Hello", None, &["Yes", "No"], 0, 200.0));
        let list = m.tiles[4].unwrap();
        assert_eq!(ui.number(list, t::WIDTH), 30.0 + 80.0);
        assert_eq!(ui.number(list, t::HEIGHT), 80.0);
        // Each button numbered and given id 7.
        let first = m.list.items[0].tile;
        assert_eq!(ui.number(first, t::ID), 7.0);
        assert_eq!(m.list.value_of(m.list.items[1].tile), Some(1));
        let text = m.tiles[3].unwrap();
        assert_eq!(ui.number(text, t::JUSTIFY), crate::text::CENTER as f32);
        assert_eq!(ui.number(text, t::WRAPWIDTH), 700.0 - 80.0);
        // Text 2 × 40 + 50 = 130, buttons 110 + 80 = 190: 200 at least.
        let main = m.tiles[0].unwrap();
        assert_eq!(ui.number(main, t::WIDTH), 200.0);
        assert_eq!(ui.number(m.tiles[5].unwrap(), t::ALPHA), 200.0);
        // No title: hidden.
        assert_eq!(ui.number(m.tiles[1].unwrap(), t::VISIBLE), 0.0);
        assert_eq!(m.sounds, [OPEN_SOUND]);
        // Text of more than one line is left-justified, and wider.
        let long = "word ".repeat(30);
        let (mut ui, m) = opened(MessageBox::new(&long, Some("Title"), &["OK"], 0, 200.0));
        let text = m.tiles[3].unwrap();
        assert_eq!(ui.number(text, t::JUSTIFY), crate::text::LEFT as f32);
        assert_eq!(
            ui.number(m.tiles[1].unwrap(), t::JUSTIFY),
            crate::text::LEFT as f32
        );
        assert_eq!(ui.number(m.tiles[1].unwrap(), t::VISIBLE), 1.0);
        let main = m.tiles[0].unwrap();
        assert!(ui.number(main, t::WIDTH) > 600.0);
        assert!(ui.number(main, t::WIDTH) <= 700.0);
    }

    /// A box a menu asked (its callback, `+0x28`) answers that menu, not
    /// `GetButtonPressed`.
    #[test]
    fn a_menus_own_box_answers_that_menu() {
        let mut b = MessageBox::new("Cancel transaction?", None, &["Yes", "No"], 0x17, 200.0);
        b.owner = Some(7);
        let (mut ui, mut m) = opened(b);
        let first = m.list.items[0].tile;
        m.click(&mut ui, 7, Some(first), 0.0);
        assert_eq!(m.take_pressed(), None);
        assert_eq!(m.take_pressed_for(Some(7)), Some(0));
        assert_eq!(m.take_pressed_for(Some(7)), None);
    }

    #[test]
    fn an_empty_text_is_no_text() {
        let (mut ui, m) = opened(MessageBox::new("", None, &["OK"], 0, 200.0));
        assert_eq!(ui.string(m.tiles[3].unwrap(), t::STRING), None);
    }

    #[test]
    fn clicks_keys_and_escape_answer() {
        let (mut ui, mut m) = opened(MessageBox::new("Pick", None, &["&Yes", "&No"], 0, 200.0));
        let second = m.list.items[1].tile;
        m.click(&mut ui, 7, Some(second), 0.0);
        assert_eq!(m.take_pressed(), Some(1));
        assert_eq!(m.take_pressed(), None);
        assert!(m.closed);
        // A key letter.
        let (mut ui, mut m) = opened(MessageBox::new("Pick", None, &["&Yes", "&No"], 0, 200.0));
        m.key(&mut ui, u32::from(b'y'), 0.0);
        assert_eq!(m.take_pressed(), Some(0));
        // Escape: the last button, not for a script's box.
        let (mut ui, mut m) = opened(MessageBox::new("Pick", None, &["A", "B", "C"], 0, 200.0));
        m.key(&mut ui, menu::key::ESCAPE_TO_MESSAGE, 0.0);
        assert_eq!(m.take_pressed(), Some(2));
        let script = MessageBox::script("Pick", None, &[Some("A".into())], 200.0);
        let (mut ui, mut m) = opened(script);
        m.key(&mut ui, menu::key::ESCAPE_TO_MESSAGE, 0.0);
        assert_eq!(m.take_pressed(), None);
        // Other ids do nothing; the next box fills the menu.
        let (mut ui, mut m) = opened(MessageBox::new("one", None, &["OK"], 0, 200.0));
        m.push(MessageBox::new("two", None, &["OK"], 0, 200.0));
        m.click(&mut ui, 3, None, 0.0);
        assert_eq!(m.take_pressed(), None);
        let tile = m.list.items[0].tile;
        m.click(&mut ui, 7, Some(tile), 0.0);
        assert!(!m.closed);
        assert_eq!(
            ui.string(m.tiles[3].unwrap(), t::STRING).as_deref(),
            Some("two")
        );
    }
}
