//! A terminal's own screen (`menus\computers_menu.xml`, class
//! `ComputersMenu` 1057, vtable `01072004` in FalloutNV.exe). What its
//! items do is the world's (`world::terminal`); this is the menu's side,
//! read from the code:
//!
//! * opening (`00757b70`): ids 0–15 must be there; the list (id 1) uses
//!   `computers_file_template`; `UIHackingFanHumLP` plays; a terminal that
//!   isn't unlocked (it was hacked, or the player holds its password note)
//!   first logs on (`00758ad0`), else the screen shows at once.
//! * every line appears through one queue (`00759da0`, typed in order by
//!   `007598d0`), each at the rate set when it was queued: menus and notes
//!   `1000 / iComputersDisplayRateMenus` (`…Notes`) ms a character (150 a
//!   second: 6 ms, whole milliseconds), the player's input `1000 /
//!   iHackingInputRate` (50 ms). Lines being typed play
//!   `UIHackingCharScroll`.
//! * the logon (ids 9–12): "WELCOME TO ROBCO INDUSTRIES (TM) TERMLINK"
//!   (`sHackingIntro01`), "> LOGON ADMIN" (`sComputersLogon`), "ENTER
//!   PASSWORD NOW" (`sHackingHeader2`), "> " and one `*` a letter of the
//!   hacking game's words for this terminal; the input lines play
//!   `UIHackingCharSingle` a character and `UIHackingCharEnter` when done,
//!   every line ends with 0.5 s; 2 s after the last the screen shows
//!   (clicking the background shows it at once).
//! * the screen (`00758d30`): the logon's lines hidden; the header
//!   `sComputersHeader1` (id 3), `sComputersHeader2` (4) and the server
//!   `sTerminalServerText1`–`10` by the record's server type (0), each
//!   centred on the screen, typed first; then the record's `DESC` (id 7;
//!   the separator, 15, shows once it's typed), each listed item as "> "
//!   and its text (an item's `_enabled` 0 until every line is typed), "> "
//!   and `sComputersBack` in a sub-menu, and the prompt "> " (8).
//! * picking an item (`00757f70`): `UIHackingCharEnter`; the world runs it
//!   (its script, its note); its `RNAM` is typed beside the prompt (id 5)
//!   and hidden after `iComputersResultDisplayTimeout` s (5); a sub-menu is
//!   filled in its place; a note shows: text in the display zone (2, its
//!   text 14) a page at a time (`007590c0`), a picture in the zone
//!   (`00759470`), a sound played (`00759030`), a voice note nothing; with
//!   no note, flag 0x02 fills the list again.
//! * a click on the zone (`00757f70` case 2): the next page, or back to the
//!   list (each item typed again, `00759560`; flag 0x02 fills it again).
//! * leaving (special code 10, `007583f0`): out of a note, else back a
//!   screen ("Back" too, `00758a80`); from the first the menu closes.

use std::collections::VecDeque;

use super::typed::Line;
use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\computers_menu.xml";
/// Its class number (`00757a10`).
pub const CLASS: i32 = 1057;
/// The menu's fade out as the player leaves a terminal or the hacking
/// game (`menufade` set to 0.75 at `00757ea0`, `00766aa0`; the generic
/// fade, `ui::fade`, then runs it).
pub const LEAVE_FADE: f32 = 0.75;
/// The list's template (`00757b70`).
const TEMPLATE: &str = "computers_file_template";

const SERVER: usize = 0;
const LIST: usize = 1;
const ZONE: usize = 2;
const HEADER_1: usize = 3;
const HEADER_2: usize = 4;
const RESULT: usize = 5;
const CURSOR: usize = 6;
const WELCOME: usize = 7;
const PROMPT: usize = 8;
const LOGON: [usize; 4] = [9, 10, 11, 12];
const BACKGROUND: usize = 13;
const ZONE_TEXT: usize = 14;
const SEPARATOR: usize = 15;

const USER0: i32 = t::USER0;

/// A character's width (the headers are centred by it).
const CHAR: f32 = 17.0;
/// What starts a line of the player's (`01072118`).
const PROMPT_TEXT: &str = "> ";

/// The code that leaves (`007583f0`).
pub const LEAVE: i32 = 10;

/// The sounds (`007577f0`; the character sound by name where it's played).
pub mod sound {
    pub const FAN: &str = "UIHackingFanHumLP";
    pub const SCROLL: &str = "UIHackingCharScroll";
    pub const ENTER: &str = "UIHackingCharEnter";
    pub const SINGLE: &str = "UIHackingCharSingle";
    /// Leaving (`007ffe40`, from `00757ea0` and the hacking menu's
    /// `00766aa0`; the power button's `007ffd50` too).
    pub const POWER_DOWN: &str = "OBJComputerTerminalPowerDown";
}

/// The rates (`iComputersDisplayRateMenus`, `…Notes`, `iHackingInputRate`,
/// characters a second; `iComputersResultDisplayTimeout` s; exe defaults,
/// none in the game's data).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rates {
    pub menus: u32,
    pub notes: u32,
    pub input: u32,
    pub result_timeout: u32,
}

impl Default for Rates {
    fn default() -> Rates {
        Rates {
            menus: 150,
            notes: 150,
            input: 20,
            result_timeout: 5,
        }
    }
}

impl Rates {
    pub const NAMES: [&'static str; 4] = [
        "iComputersDisplayRateMenus",
        "iComputersDisplayRateNotes",
        "iHackingInputRate",
        "iComputersResultDisplayTimeout",
    ];
}

/// Milliseconds a character: `1000 / rate`, whole; 1000 for a rate of 0.
fn per_char(rate: u32) -> f64 {
    1000u32.checked_div(rate).map_or(1000.0, f64::from)
}

/// One screen of a terminal: what the world says it lists now.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Screen {
    /// The terminal record (the menu hands it back in its requests).
    pub terminal: u32,
    /// Its `DESC`.
    pub welcome: String,
    /// Its server type (`DNAM` byte 2).
    pub server: u8,
    /// The items listed: their places in the record and their text.
    pub items: Vec<(usize, String)>,
}

/// What a note shows.
#[derive(Debug, Clone, PartialEq)]
pub enum NoteView {
    Text(String),
    Image(String),
    /// A sound note's sound (played; nothing shown).
    Sound(u32),
    /// A voice note (the menu does nothing).
    Voice,
}

/// What picking an item did, as the world worked it out.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Used {
    /// Its `RNAM`.
    pub result: Option<String>,
    pub note: Option<NoteView>,
    /// Its sub-menu, to go into.
    pub submenu: Option<Screen>,
    /// Flag 0x02: the screen filled again (now, with no note; after the
    /// note, with one).
    pub redraw: Option<Screen>,
    /// The item's script said `ForceTerminalBack`: back a screen first
    /// (`005dc4e0` runs `00758a80` from inside the script).
    pub back: bool,
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    Sound(&'static str, f64),
    Keep(&'static str),
    Hum,
    /// A sound record (a sound note).
    PlayForm(u32),
    /// The player picked this item of this terminal: run it and answer
    /// with [`ComputersMenu::used`].
    Use {
        terminal: u32,
        item: usize,
    },
    /// Fill this terminal's screen again (back from a sub-menu, or after a
    /// note with flag 0x02): answer with [`ComputersMenu::fill`].
    Fill {
        terminal: u32,
    },
    /// Which of these items' conditions pass now (back from a note):
    /// answer with [`ComputersMenu::retype`].
    Retype {
        terminal: u32,
    },
    Close,
    /// The player left (`00757ea0`): [`sound::POWER_DOWN`] plays and the
    /// rendered terminal fades out (`007ffaf0`); after [`Request::Close`].
    Leave,
}

/// Where the menu is (`+0xb0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Logon,
    Waiting,
    Screen,
}

/// What the menu opens on.
pub struct Setup {
    pub screen: Screen,
    /// For a terminal that isn't unlocked: the hacking game's word length
    /// (the asterisks of the logon).
    pub logon: Option<usize>,
    pub rates: Rates,
}

pub struct ComputersMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; 16],
    pub list: ListBox,
    pub stage: Stage,
    /// The screens gone into, the current one last (`+0x98`).
    stack: Vec<Screen>,
    queue: VecDeque<Line>,
    /// The queue's pause (`011d93d4`) and the logon's 2 s (`011d93c4`).
    hold: f64,
    wait_until: f64,
    /// The rate new lines get (`0119f278`).
    rate: f64,
    rates: Rates,
    /// Every line typed: the items get enabled (`+0xc1`).
    dirty: bool,
    /// The result's end (`+0xa0`).
    result_until: f64,
    /// A text note's pages: what's left (`+0xbc`).
    pages: Option<String>,
    /// Flag 0x02 on the note's item (`011d93c0`).
    redraw_after_note: bool,
    /// Items picked (the item's `+0x74` & 4), by terminal and place.
    visited: Vec<(u32, usize)>,
    blink_at: f64,
    now: f64,
    pub requests: Vec<Request>,
    pub closed: bool,
}

fn setting(ui: &Ui, name: &str) -> String {
    ui.setting_text(name).unwrap_or_default()
}

impl ComputersMenu {
    pub fn new(menu: TileId) -> ComputersMenu {
        ComputersMenu {
            menu,
            tiles: [None; 16],
            list: ListBox::default(),
            stage: Stage::Screen,
            stack: Vec::new(),
            queue: VecDeque::new(),
            hold: 0.0,
            wait_until: 0.0,
            rate: 6.0,
            rates: Rates::default(),
            dirty: false,
            result_until: 0.0,
            pages: None,
            redraw_after_note: false,
            visited: Vec::new(),
            blink_at: 0.0,
            now: 0.0,
            requests: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    fn set_visible(&self, ui: &mut Ui, id: usize, on: bool) {
        if let Some(t) = self.tile(id) {
            ui.set_number(t, t::VISIBLE, f32::from(on));
        }
    }

    /// Queues a line into a tile's trait, at `at` or the end (`00759da0`).
    fn queue_line(&mut self, text: &str, tile: Option<TileId>, string: i32, at: Option<usize>) {
        let mut line = Line::new(tile, text, self.rate);
        line.string = string;
        match at {
            Some(i) if i <= self.queue.len() => self.queue.insert(i, line),
            _ => self.queue.push_back(line),
        }
    }

    /// Every queued line at once (`00759f50`).
    fn finish_queue(&mut self, ui: &mut Ui) {
        for l in self.queue.iter_mut() {
            l.finish(ui);
        }
    }

    /// Opens it (`00757b70`); false without its tiles.
    pub fn open(&mut self, ui: &mut Ui, setup: Setup) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.rates = setup.rates;
        if let Some(list) = self.tile(LIST) {
            self.list = ListBox::new(ui, list, TEMPLATE);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.requests.push(Request::Hum);
        self.stack.push(setup.screen);
        match setup.logon {
            Some(letters) => self.logon(ui, letters),
            None => self.show_screen(ui),
        }
        ui.refresh();
        true
    }

    /// The logon (`00758ad0`).
    fn logon(&mut self, ui: &mut Ui, letters: usize) {
        self.stage = Stage::Logon;
        let menus = per_char(self.rates.menus);
        let input = per_char(self.rates.input);
        let lines = [
            (setting(ui, "sHackingIntro01"), menus),
            (
                format!("{PROMPT_TEXT}{}", setting(ui, "sComputersLogon")),
                input,
            ),
            (setting(ui, "sHackingHeader2"), menus),
            (format!("{PROMPT_TEXT}{}", "*".repeat(letters)), input),
        ];
        for (id, (text, ms)) in LOGON.iter().zip(lines) {
            self.rate = ms;
            let tile = self.tile(*id);
            self.queue_line(&text, tile, t::STRING, None);
        }
        self.rate = input;
    }

    /// The screen (`00758d30`).
    fn show_screen(&mut self, ui: &mut Ui) {
        if let Some(b) = self.tile(BACKGROUND) {
            ui.set_number(b, t::TARGET, 0.0);
        }
        self.finish_queue(ui);
        for id in LOGON {
            self.set_visible(ui, id, false);
        }
        self.fill_list(ui);
        let server = self.stack.last().map_or(0, |s| s.server);
        let server_text = if server < 10 {
            setting(ui, &format!("sTerminalServerText{}", server + 1))
        } else {
            String::new()
        };
        let lines = [
            (HEADER_1, setting(ui, "sComputersHeader1")),
            (HEADER_2, setting(ui, "sComputersHeader2")),
            (SERVER, server_text),
        ];
        for (at, (id, text)) in lines.into_iter().enumerate() {
            let tile = self.tile(id);
            self.queue_line(&text, tile, t::STRING, Some(at));
            if let Some(tile) = tile {
                let width = ui.tiles[tile]
                    .parent
                    .map_or(0.0, |p| ui.number(p, t::WIDTH));
                ui.set_number(tile, t::X, (width - (text.len() as f32 * CHAR)) / 2.0);
            }
        }
        let prompt = self.tile(PROMPT);
        self.queue_line(PROMPT_TEXT, prompt, t::STRING, None);
        self.stage = Stage::Screen;
    }

    /// The list from the current screen (`007586e0`).
    fn fill_list(&mut self, ui: &mut Ui) {
        let Some(screen) = self.stack.last().cloned() else {
            return;
        };
        self.finish_queue(ui);
        self.list.clear(ui);
        self.rate = per_char(self.rates.menus);
        if !screen.welcome.is_empty() {
            let welcome = self.tile(WELCOME);
            if let Some(w) = welcome {
                ui.set_number(w, t::VISIBLE, 0.0);
            }
            self.queue_line(&screen.welcome, welcome, t::STRING, None);
        }
        let enabled = ui.names.lookup_or_add("_enabled");
        for (index, text) in &screen.items {
            let Some(item) = self.list.add(ui, self.menu, *index as i32, None) else {
                continue;
            };
            if let Some(e) = enabled {
                ui.set_number(item, e, 0.0);
            }
            let visited = self.visited.contains(&(screen.terminal, *index));
            ui.set_number(item, 4054, if visited { 2.0 } else { 1.0 });
            let mut line = Line::new(Some(item), &format!("{PROMPT_TEXT}{text}"), self.rate);
            line.string = USER0;
            self.queue.push_back(line);
        }
        if self.stack.len() > 1 {
            if let Some(item) = self.list.add(ui, self.menu, -1, None) {
                if let Some(e) = enabled {
                    ui.set_number(item, e, 0.0);
                }
                let text = format!("{PROMPT_TEXT}{}", setting(ui, "sComputersBack"));
                let mut line = Line::new(Some(item), &text, self.rate);
                line.string = USER0;
                self.queue.push_back(line);
            }
        }
        self.dirty = true;
        ui.refresh();
    }

    /// The world's answer to [`Request::Fill`] (and the screen again after a
    /// flag 0x02 item): the current screen's list again.
    pub fn fill(&mut self, ui: &mut Ui, screen: Screen) {
        if let Some(top) = self.stack.last_mut() {
            *top = screen;
        }
        self.fill_list(ui);
    }

    /// The world's answer to [`Request::Retype`] (`00759560` for each item):
    /// the items whose conditions pass typed again.
    pub fn retype(&mut self, ui: &mut Ui, passing: &[usize]) {
        let Some(screen) = self.stack.last().cloned() else {
            return;
        };
        self.rate = per_char(self.rates.menus);
        let enabled = ui.names.lookup_or_add("_enabled");
        let items: Vec<(TileId, i32)> = self.list.items.iter().map(|i| (i.tile, i.value)).collect();
        for (tile, value) in items {
            if let Some(e) = enabled {
                ui.set_number(tile, e, 0.0);
            }
            let text = if value < 0 {
                Some(setting(ui, "sComputersBack"))
            } else if passing.contains(&(value as usize)) {
                screen
                    .items
                    .iter()
                    .find(|(i, _)| *i == value as usize)
                    .map(|(_, t)| t.clone())
            } else {
                None
            };
            let mut xdefault = 1.0;
            if value >= 0 {
                if let Some(p) = self
                    .visited
                    .iter()
                    .position(|v| *v == (screen.terminal, value as usize))
                {
                    if text.is_some() {
                        xdefault = 2.0;
                        self.visited.remove(p);
                    }
                }
            }
            ui.set_number(tile, 4054, xdefault);
            if let Some(text) = text {
                ui.set_string(tile, USER0, "");
                let mut line = Line::new(Some(tile), &format!("{PROMPT_TEXT}{text}"), self.rate);
                line.string = USER0;
                self.queue.push_back(line);
            }
        }
        self.dirty = true;
        ui.refresh();
    }

    /// The world's answer to [`Request::Use`] (the rest of `00757f70`).
    pub fn used(&mut self, ui: &mut Ui, used: Used) {
        if used.back {
            self.back(ui);
            if self.closed {
                return;
            }
        }
        if let Some(result) = used.result.filter(|r| !r.is_empty()) {
            self.rate = 0.0;
            let tile = self.tile(RESULT);
            self.queue_line(&result, tile, t::STRING, Some(0));
            self.result_until = self.now + f64::from(self.rates.result_timeout) * 1000.0;
        }
        if let Some(sub) = used.submenu {
            self.stack.push(sub);
            self.fill_list(ui);
            return;
        }
        match used.note {
            None => {
                if let Some(screen) = used.redraw {
                    self.fill(ui, screen);
                }
            }
            Some(note) => {
                self.redraw_after_note = used.redraw.is_some();
                match note {
                    NoteView::Text(text) => {
                        self.pages = Some(text);
                        self.show_page(ui);
                    }
                    NoteView::Image(path) => self.show_image(ui, &path),
                    NoteView::Sound(form) => self.requests.push(Request::PlayForm(form)),
                    NoteView::Voice => {}
                }
            }
        }
        ui.refresh();
    }

    /// A page of a text note (`007590c0`).
    fn show_page(&mut self, ui: &mut Ui) {
        self.finish_queue(ui);
        self.rate = per_char(self.rates.notes);
        self.set_visible(ui, LIST, false);
        if let Some(z) = self.tile(ZONE) {
            ui.set_number(z, t::ALPHA, 0.0);
            ui.set_number(z, t::VISIBLE, 1.0);
        }
        let Some(text) = self.pages.take() else {
            return;
        };
        let (page, rest) = self.page_of(ui, &text);
        let zone_text = self.tile(ZONE_TEXT);
        self.queue_line(&page, zone_text, t::STRING, None);
        self.pages = (!rest.is_empty()).then_some(rest);
        ui.refresh();
    }

    /// As much of `text` as a page holds (`007590c0`): the zone text's
    /// `wraplines` less the welcome's lines, + 1, broken as the game breaks
    /// lines (`00a12fb0`).
    fn page_of(&self, ui: &mut Ui, text: &str) -> (String, String) {
        let Some(zt) = self.tile(ZONE_TEXT) else {
            return (text.to_string(), String::new());
        };
        let wrap_lines = ui.number(zt, t::WRAPLINES);
        let wrap_width = ui.number(zt, t::WRAPWIDTH) as i32;
        let welcome_height = self.tile(WELCOME).map_or(0.0, |w| ui.number(w, t::HEIGHT));
        let font_index = ui.number(zt, t::FONT).max(1.0) as usize;
        let Some(font) = crate::list::font_for(ui, font_index as i32) else {
            return (text.to_string(), String::new());
        };
        let line_height = font.line_height.trunc();
        let lines = if line_height > 0.0 {
            (wrap_lines - welcome_height / line_height) as i32 + 1
        } else {
            wrap_lines as i32
        };
        let bytes = text.as_bytes();
        // The longest start of the text that breaks into that many lines.
        let fits = |n: usize| {
            crate::text::prepare(&font, line_height, 0.0, &bytes[..n], wrap_width, 0).lines
                <= lines.max(1)
        };
        if fits(bytes.len()) {
            return (text.to_string(), String::new());
        }
        let (mut lo, mut hi) = (0, bytes.len());
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if fits(mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        // Back to the last break, as the line breaking would end there.
        let cut = bytes[..lo]
            .iter()
            .rposition(|&c| c == b' ' || c == b'\n')
            .map_or(lo, |p| p + 1);
        let cut = if cut == 0 { lo } else { cut };
        (
            String::from_utf8_lossy(&bytes[..cut]).into_owned(),
            String::from_utf8_lossy(&bytes[cut..]).into_owned(),
        )
    }

    /// A picture note (`00759470`).
    fn show_image(&mut self, ui: &mut Ui, path: &str) {
        self.finish_queue(ui);
        self.set_visible(ui, LIST, false);
        if let Some(z) = self.tile(ZONE) {
            ui.set_number(z, t::ALPHA, 255.0);
            ui.set_number(z, t::VISIBLE, 1.0);
            ui.set_string(z, t::FILENAME, path);
        }
        self.set_visible(ui, ZONE_TEXT, false);
        self.pages = None;
    }

    /// Out of a note (`00757f70` case 2 with no page left).
    fn close_note(&mut self, ui: &mut Ui) {
        self.set_visible(ui, ZONE, false);
        self.set_visible(ui, ZONE_TEXT, false);
        self.set_visible(ui, LIST, true);
        let terminal = self.stack.last().map_or(0, |s| s.terminal);
        if self.redraw_after_note {
            self.requests.push(Request::Fill { terminal });
        } else {
            self.requests.push(Request::Retype { terminal });
        }
        self.dirty = true;
    }

    /// Back a screen (`00758a80`); out of the first, the menu closes.
    fn back(&mut self, ui: &mut Ui) {
        self.stack.pop();
        match self.stack.last() {
            Some(s) => {
                let terminal = s.terminal;
                self.requests.push(Request::Fill { terminal });
            }
            None => self.close(ui),
        }
    }

    /// Closes it as the player leaves (`00757ea0`): it fades out over
    /// 0.75 s (`menufade`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        ui.set_number(self.menu, t::MENUFADE, LEAVE_FADE);
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
        self.requests.push(Request::Close);
        self.requests.push(Request::Leave);
    }

    /// Every frame (`00758470`).
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        if self.closed {
            return;
        }
        self.now = now;
        self.step_queue(ui, now);
        self.blink(ui, now);
        if self.queue.is_empty() {
            match self.stage {
                Stage::Logon => {
                    self.wait_until = now + 2000.0;
                    self.stage = Stage::Waiting;
                }
                Stage::Waiting if now > self.wait_until => self.show_screen(ui),
                Stage::Screen if self.dirty => {
                    if let Some(e) = ui.names.lookup_or_add("_enabled") {
                        for i in &self.list.items {
                            ui.set_number(i.tile, e, 1.0);
                        }
                    }
                    self.dirty = false;
                }
                _ => {}
            }
        }
        if self.result_until != 0.0 && now > self.result_until {
            self.result_until = 0.0;
            self.set_visible(ui, RESULT, false);
        }
        ui.refresh();
    }

    /// The queue (`007598d0`).
    fn step_queue(&mut self, ui: &mut Ui, now: f64) {
        loop {
            if self.queue.is_empty() || now <= self.hold {
                return;
            }
            let head = &self.queue[0];
            if !head.done {
                break;
            }
            if self.stage == Stage::Logon {
                if head.text.starts_with('>') {
                    self.requests.push(Request::Sound(sound::ENTER, 0.0));
                }
                self.hold = now + 500.0;
            } else if head.tile.is_some() && head.tile == self.tile(WELCOME) {
                self.set_visible(ui, SEPARATOR, true);
            }
            self.queue.pop_front();
        }
        let input = self.stage == Stage::Logon && self.queue[0].text.starts_with('>');
        let head = &mut self.queue[0];
        let n = head.step(ui, now);
        if input {
            let ms = head.ms;
            for k in 0..n {
                self.requests
                    .push(Request::Sound(sound::SINGLE, ms * k as f64 / n as f64));
            }
        } else {
            self.requests.push(Request::Keep(sound::SCROLL));
        }
        let tile = self.queue[0].tile;
        if let Some(tile) = tile {
            self.cursor_after(ui, tile);
        }
    }

    /// The cursor after a line's text (`007598d0`'s end).
    fn cursor_after(&self, ui: &mut Ui, tile: TileId) {
        let Some(cursor) = self.tile(CURSOR) else {
            return;
        };
        let text = if ui.tiles[tile].kind == crate::names::kind::TEXT {
            tile
        } else {
            match ui.find_below(tile, "computers_file_template_text") {
                Some(t) => t,
                None => return,
            }
        };
        let width = ui.layout(text).map_or(0.0, |l| l.width as f32);
        let top = ui.tiles[cursor].parent;
        let (mut x, mut y) = (width, 0.0);
        let mut at = Some(text);
        while let Some(a) = at {
            if Some(a) == top {
                break;
            }
            x += ui.number(a, t::X);
            y += ui.number(a, t::Y);
            at = ui.tiles[a].parent;
        }
        ui.set_number(cursor, t::X, x);
        ui.set_number(cursor, t::Y, y);
        ui.set_number(cursor, t::VISIBLE, 1.0);
    }

    /// The cursor's blinking (`007585a0`).
    fn blink(&mut self, ui: &mut Ui, now: f64) {
        let Some(cursor) = self.tile(CURSOR) else {
            return;
        };
        if now > self.blink_at {
            let lit = ui.number(cursor, t::VISIBLE) == 1.0;
            ui.set_number(cursor, t::VISIBLE, if lit { 0.0 } else { 1.0 });
            let interval = ui
                .names
                .lookup("_blink_interval")
                .map_or(400.0, |id| ui.number(cursor, id));
            self.blink_at = now + f64::from(interval);
        }
    }
}

impl MenuCode for ComputersMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..16).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.list]
    }

    /// `00757f70`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        if self.closed {
            return;
        }
        if id == ZONE as i32 {
            self.finish_queue(ui);
            if self.pages.is_some() {
                self.show_page(ui);
            } else {
                self.close_note(ui);
            }
            self.requests.push(Request::Sound(sound::ENTER, 0.0));
            ui.refresh();
            return;
        }
        if id == BACKGROUND as i32 && self.stage != Stage::Screen {
            self.show_screen(ui);
            ui.refresh();
            return;
        }
        let Some(tile) = tile else {
            return;
        };
        let Some(value) = self.list.value_of(tile) else {
            return;
        };
        let enabled = ui.names.lookup("_enabled");
        if enabled.is_some_and(|e| ui.number(tile, e) == 0.0) {
            return;
        }
        self.requests.push(Request::Sound(sound::ENTER, 0.0));
        if value < 0 {
            self.back(ui);
            return;
        }
        let terminal = self.stack.last().map_or(0, |s| s.terminal);
        if !self.visited.contains(&(terminal, value as usize)) {
            self.visited.push((terminal, value as usize));
        }
        self.requests.push(Request::Use {
            terminal,
            item: value as usize,
        });
    }

    /// `007583f0`: code 10 leaves a note, else goes back a screen.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if code != LEAVE {
            return false;
        }
        let in_note = self
            .tile(ZONE)
            .is_some_and(|z| ui.number(z, t::VISIBLE) != 0.0);
        if in_note {
            self.pages = None;
            self.click(ui, ZONE as i32, None, now);
        } else {
            self.back(ui);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn screen(terminal: u32, items: &[(usize, &str)]) -> Screen {
        Screen {
            terminal,
            welcome: "Welcome, USER".into(),
            server: 1,
            items: items.iter().map(|(i, t)| (*i, t.to_string())).collect(),
        }
    }

    fn opened(logon: Option<usize>) -> (Ui, ComputersMenu) {
        let mut ui = test_support::ui();
        let mut m = ComputersMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::computers_menu(), &mut m);
        let setup = Setup {
            screen: screen(0xA5D, &[(0, "Disengage Lock"), (1, "Read memo")]),
            logon,
            rates: Rates::default(),
        };
        assert!(m.open(&mut ui, setup));
        (ui, m)
    }

    fn run(ui: &mut Ui, m: &mut ComputersMenu, from: f64, until: f64) -> f64 {
        let mut now = from;
        while now < until {
            m.update(ui, now);
            now += 16.0;
        }
        now
    }

    fn text(ui: &mut Ui, m: &ComputersMenu, id: usize) -> String {
        m.tile(id)
            .and_then(|t| ui.string(t, t::STRING))
            .unwrap_or_default()
    }

    fn items(ui: &mut Ui, m: &ComputersMenu) -> Vec<String> {
        m.list
            .items
            .iter()
            .map(|i| ui.string(i.tile, USER0).unwrap_or_default())
            .collect()
    }

    /// `00758d30`, `007586e0`: the header, server and welcome, then the
    /// items typed and enabled.
    #[test]
    fn the_screen_types_out() {
        let (mut ui, mut m) = opened(None);
        assert_eq!(m.stage, Stage::Screen);
        assert!(m.requests.contains(&Request::Hum));
        run(&mut ui, &mut m, 1.0, 3000.0);
        assert_eq!(
            text(&mut ui, &m, HEADER_1),
            "ROBCO INDUSTRIES UNIFIED OPERATING SYSTEM"
        );
        assert_eq!(text(&mut ui, &m, SERVER), "-Server 2-");
        assert_eq!(text(&mut ui, &m, WELCOME), "Welcome, USER");
        assert_eq!(items(&mut ui, &m), vec!["> Disengage Lock", "> Read memo"]);
        let e = ui.names.lookup("_enabled").unwrap();
        assert!(m.list.items.iter().all(|i| ui.number(i.tile, e) == 1.0));
        assert_eq!(text(&mut ui, &m, PROMPT), "> ");
    }

    /// `00758ad0`: the logon with the password's asterisks, then the screen
    /// 2 s after; a click on the background skips it.
    #[test]
    fn the_logon_comes_first_after_a_hack() {
        let (mut ui, mut m) = opened(Some(7));
        assert_eq!(m.stage, Stage::Logon);
        let now = run(&mut ui, &mut m, 1.0, 3500.0);
        assert_eq!(text(&mut ui, &m, LOGON[1]), "> LOGON ADMIN");
        assert_eq!(text(&mut ui, &m, LOGON[3]), "> *******");
        assert!(m.requests.contains(&Request::Sound(sound::ENTER, 0.0)));
        run(&mut ui, &mut m, now, now + 2500.0);
        assert_eq!(m.stage, Stage::Screen);
        let (mut ui, mut m) = opened(Some(7));
        m.update(&mut ui, 1.0);
        m.click(&mut ui, BACKGROUND as i32, None, 1.0);
        assert_eq!(m.stage, Stage::Screen);
    }

    /// `00757f70`: an item asks the world; a text note pages; leaving goes
    /// back out of the note, then out of the menu.
    #[test]
    fn items_notes_and_leaving() {
        let (mut ui, mut m) = opened(None);
        let now = run(&mut ui, &mut m, 1.0, 3000.0);
        m.requests.clear();
        let second = m.list.items[1].tile;
        m.click(&mut ui, -1, Some(second), now);
        assert!(m.requests.contains(&Request::Use {
            terminal: 0xA5D,
            item: 1
        }));
        let long: String = (0..60)
            .map(|i| format!("Line {i} of the memo.\n"))
            .collect();
        m.used(
            &mut ui,
            Used {
                result: Some("Opening...".into()),
                note: Some(NoteView::Text(long)),
                ..Used::default()
            },
        );
        let now = run(&mut ui, &mut m, now, now + 3000.0);
        let zone = m.tile(ZONE).unwrap();
        assert_eq!(ui.number(zone, t::VISIBLE), 1.0);
        let page1 = text(&mut ui, &m, ZONE_TEXT);
        assert!(page1.starts_with("Line 0 of the memo."));
        assert!(!page1.contains("Line 59"));
        assert!(m.pages.is_some());
        // The next page.
        m.click(&mut ui, ZONE as i32, None, now);
        let now = run(&mut ui, &mut m, now, now + 3000.0);
        assert_ne!(text(&mut ui, &m, ZONE_TEXT), page1);
        // Leaving shuts the note and asks for the items again.
        m.requests.clear();
        assert!(m.special_key(&mut ui, LEAVE, now));
        assert_eq!(ui.number(zone, t::VISIBLE), 0.0);
        assert!(m.requests.contains(&Request::Retype { terminal: 0xA5D }));
        m.retype(&mut ui, &[0, 1]);
        // The result goes after its time.
        run(&mut ui, &mut m, now, now + 6000.0);
        let result = m.tile(RESULT).unwrap();
        assert_eq!(ui.number(result, t::VISIBLE), 0.0);
        // Leaving the first screen closes the menu.
        m.special_key(&mut ui, LEAVE, now + 6000.0);
        assert!(m.closed);
        // It fades out over 0.75 s (`00757ea0`).
        assert_eq!(ui.number(m.menu, t::MENUFADE), LEAVE_FADE);
        let close = m.requests.iter().position(|r| *r == Request::Close);
        let leave = m.requests.iter().position(|r| *r == Request::Leave);
        assert!(close.is_some() && leave > close);
    }

    /// A sub-menu, then "Back" (`00758a80`).
    #[test]
    fn submenus_and_back() {
        let (mut ui, mut m) = opened(None);
        let now = run(&mut ui, &mut m, 1.0, 3000.0);
        let first = m.list.items[0].tile;
        m.click(&mut ui, -1, Some(first), now);
        m.used(
            &mut ui,
            Used {
                submenu: Some(screen(0xB00, &[(0, "Logs")])),
                ..Used::default()
            },
        );
        let now = run(&mut ui, &mut m, now, now + 3000.0);
        assert_eq!(items(&mut ui, &m), vec!["> Logs", "> Back"]);
        m.requests.clear();
        let back = m.list.items[1].tile;
        m.click(&mut ui, -1, Some(back), now);
        assert!(m.requests.contains(&Request::Fill { terminal: 0xA5D }));
        assert!(!m.closed);
        // An item whose script says `ForceTerminalBack`: back first; from
        // the first screen the menu closes.
        m.fill(&mut ui, screen(0xA5D, &[(0, "Disengage Lock")]));
        m.used(
            &mut ui,
            Used {
                back: true,
                ..Used::default()
            },
        );
        assert!(m.closed);
    }
}
