//! Hacking (`menus\hacking_menu.xml`, class `HackingMenu` 1055, vtable
//! `010728f4` in FalloutNV.exe): the terminal's word game on screen. The
//! rules (the words, the screen, the choices) are `world::hacking`'s; this
//! is the menu's side, read from the code (names from the Xbox 360
//! prototype's symbols, Xbox PDB):
//!
//! * opening (`HackingMenu::Create`, `00765b80`): every id 0–7 must be
//!   there; 6 and 7 get `sHackingLockout3` and `4`. A locked-out terminal
//!   shows them at once (the menu's `user2`) and nothing else. Otherwise the
//!   menu's `user0` is the prompt ">", the entry line (id 5) types at
//!   `iHackingInputRate`, and 14 intro lines (`hacking_intro_template`
//!   under id 0) at y 0, 60, 120, 180, 240, 270, 330, 360, 390, 420, 450,
//!   480, 510, 570 (`iYInterval` 30): "SECURITY RESET...", then
//!   `sHackingIntro01`–`13`, the 3rd, 5th, 6th and last typed as the
//!   player's input (">" before them, `iHackingInputRate`), the rest as the
//!   machine's (`iHackingOutputRate`). The fan hum starts.
//! * a typed line (`0076b300`; typing `006ffea0`, a new text `006ffda0`,
//!   finishing `00700110`): every `1000 / rate` ms (rounded, at least 1;
//!   1000 below a rate of 2) as many characters as the time since the
//!   last step covers, the line shown as it starts; done the step after
//!   the last character. A flashing line turns off and on every
//!   `iHackingFlashOffDuration` / `OnDuration` ms.
//! * the intro (`00769830`): the first line not done types; an input line
//!   first shows its ">" and waits 1 s, plays `UIHackingCharSingle` for
//!   each character it types and `UIHackingCharEnter` when done, then
//!   0.5 s; machine lines play `UIHackingCharScroll`. "SECURITY RESET..."
//!   shows only within the retry time (`dwRetryTime`: the menu last closed
//!   less than `iHackingRetryMilliseconds` ago). When all are done the
//!   screen is made; a click makes it at once, outside the retry time.
//! * the screen (`MakePasswordFile`, `00768aa0`): the intro lines go; under
//!   id 1 `sHackingHeader`, `sHackingHeader2` and "n `sHackingHeader3`"
//!   at y 0, 30, 90 (machine rate); the guess boxes (`hacking_guess_template`,
//!   one per attempt) right of the attempts line (id 1's `user1`, `user2`)
//!   appear after it; the 34 lines (`hacking_password_file_template` under
//!   id 2, `listindex` the row, `user0` the column) at
//!   `iHackingDumpRate`, one after another (`UIHackingCharScroll`); id 0
//!   hidden. A click finishes them all.
//! * playing (`00767c90`, `00769b50`): the pointer on a line picks a word,
//!   brackets or a character (`world::hacking::Game::selection`); the entry
//!   line types it (one `UIHackingCharSingle` a character); id 2's `user0`
//!   –`user2` and `user3`–`user5` show it highlighted (two parts for a word
//!   running onto the next line), `user1` and `user4` −1 for none. A click
//!   on a line (`DoClick`, `00766b80`) plays `UIHackingCharEnter`, chooses
//!   (`Game::choose`), adds the log lines (`hacking_password_log_template`
//!   under id 3, the newest at the bottom, `00767ef0`), rewrites the
//!   attempts line and the guess boxes, and `UIHackingPassBad` /
//!   `UIHackingPassGood` follow 250 ms later. After a click the pointer
//!   picks nothing until it leaves the letters.
//! * one attempt left: id 1's second line becomes `sHackingWarning` and
//!   flashes; refilled, it goes back.
//! * locked out (`0076a3f0`): the screen scrolls up `iYInterval` every
//!   50 ms (the menu's `user1`, `UIHackingCharScroll`) until it's gone, then
//!   ids 6 and 7 show (the menu's `user2`).
//! * the password (`0076a510`): 3 s later the terminal's own menu takes
//!   over (`TransitionToComputersMenu`).
//! * the cursor (id 4, `0076a9d0`, `0076ac10`, `0076ab40`): after the line
//!   typing, then after the entry line; blinking every `_blink_interval` ms.
//! * leaving (special code 10, `00767b80`; `00766aa0`): the menu closes;
//!   the retry time runs from then.

use world::grass::Twister;
use world::hacking::{Game, Outcome, Selection, FILE_CHARS, LINE_CHARS, ROWS};

use super::computers::LEAVE_FADE;
use super::typed::Line;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\hacking_menu.xml";
/// Its class number (`00765870`).
pub const CLASS: i32 = 1055;

const INTRO: usize = 0;
const HEADER: usize = 1;
const SCREEN: usize = 2;
const LOG: usize = 3;
const CURSOR: usize = 4;
const ENTRY: usize = 5;
const LOCKED_1: usize = 6;
const LOCKED_2: usize = 7;

const USER0: i32 = t::USER0;
const USER1: i32 = t::USER0 + 1;
const USER2: i32 = t::USER0 + 2;
const USER3: i32 = t::USER0 + 3;
const USER4: i32 = t::USER0 + 4;
const USER5: i32 = t::USER0 + 5;

/// A character's width on the screen (`0x11` throughout the code).
const CHAR: f32 = 17.0;
/// `iYStart`, `iYInterval` (the constructor's, `007654f0`).
const Y_START: f32 = 0.0;
const Y_INTERVAL: f32 = 30.0;
/// The address and space before a line's characters.
const PREFIX: usize = 7;
/// The intro lines' rows (× `iYInterval`) and whether each is input.
const INTRO_LINES: [(f32, bool); 14] = [
    (0.0, false),
    (2.0, false),
    (4.0, true),
    (6.0, false),
    (8.0, true),
    (9.0, true),
    (11.0, false),
    (12.0, false),
    (13.0, false),
    (14.0, false),
    (15.0, false),
    (16.0, false),
    (17.0, false),
    (19.0, true),
];
/// Where the screen's lines start among the typed lines.
const FIRST_FILE_LINE: usize = 4;

/// The sounds (`007654f0` loads them; the character sound by name where
/// it's played).
pub mod sound {
    pub const FAN: &str = "UIHackingFanHumLP";
    pub const GOOD: &str = "UIHackingPassGood";
    pub const BAD: &str = "UIHackingPassBad";
    pub const SCROLL: &str = "UIHackingCharScroll";
    pub const ENTER: &str = "UIHackingCharEnter";
    pub const SINGLE: &str = "UIHackingCharSingle";
}

/// The rates and durations (`iHacking…`; exe defaults, none set in the
/// game's data).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rates {
    /// Characters a second: the machine's, the player's, the screen's.
    pub output: i32,
    pub input: i32,
    pub dump: i32,
    /// The warning's flashing, ms.
    pub flash_on: i32,
    pub flash_off: i32,
}

impl Default for Rates {
    fn default() -> Rates {
        Rates {
            output: 67,
            input: 20,
            dump: 500,
            flash_on: 750,
            flash_off: 500,
        }
    }
}

impl Rates {
    /// The settings' names, in the fields' order.
    pub const NAMES: [&'static str; 5] = [
        "iHackingOutputRate",
        "iHackingInputRate",
        "iHackingDumpRate",
        "iHackingFlashOnDuration",
        "iHackingFlashOffDuration",
    ];
}

/// Milliseconds a character (`0076b300`).
fn per_char(rate: i32) -> f64 {
    if rate < 2 {
        1000.0
    } else {
        let q = 1000.0 / rate as f32;
        f64::from(q.trunc() + if q - q.trunc() >= 0.5 { 1.0 } else { 0.0 }).max(1.0)
    }
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// A sound, after a delay in ms (`00ad8870`; 0 for `00ad8830`).
    Sound(&'static str, f64),
    /// A sound the code plays again every frame something types (left
    /// playing if it still is).
    Keep(&'static str),
    /// The fan's hum, looping while the menu is open.
    Hum,
    /// The character sounds still waiting stop (`00ad8d10` on the list).
    StopTyping,
    /// The password: the terminal is hacked (now), and its menu should
    /// open (with [`Request::Close`] 3 s later).
    Granted,
    /// Out of attempts: the terminal locks the player out.
    LockedOut,
    /// The menu closed; the terminal's menu opens if `Granted` came.
    Close,
    /// The player left (`00766aa0`): the power-down sound plays and the
    /// rendered terminal fades out; after [`Request::Close`].
    Leave,
}

/// Where the menu is (`eStage`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Intro,
    Dump,
    Playing,
    LockingOut,
    Done,
}

/// What the menu opens on.
pub struct Setup {
    /// The session; `None` for a locked-out terminal.
    pub game: Option<Game>,
    /// The game's generator for the choices.
    pub rng: Twister,
    /// The address column's base (see `world::hacking::PasswordFile::address`).
    pub address: u16,
    pub rates: Rates,
    /// Until when (ms) the retry time runs (`dwRetryTime`).
    pub retry_until: f64,
}

pub struct HackingMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; 8],
    pub game: Option<Game>,
    rng: Twister,
    pub stage: Stage,
    /// The typed lines (`IOLines`, 46 places).
    lines: Vec<Option<Line>>,
    /// The entry line (`xEntryText`).
    entry: Option<Line>,
    guesses: Vec<TileId>,
    log: Vec<TileId>,
    address: u16,
    rates: Rates,
    retry_until: f64,
    /// The intro's pause (`011d969c`).
    wait: f64,
    /// `dwAccessTime`: the lockout's next scroll, or when the terminal opens.
    access_time: f64,
    /// `iLastOffset`, `pLastTarget`, `bSuppressEntry`.
    last_offset: i32,
    last_target: Option<TileId>,
    suppress: bool,
    /// The cursor's next blink.
    blink_at: f64,
    /// The time of the last update, ms (clicks and keys keep to it).
    now: f64,
    pub requests: Vec<Request>,
    pub closed: bool,
}

fn setting(ui: &Ui, name: &str) -> String {
    ui.setting_text(name).unwrap_or_default()
}

impl HackingMenu {
    pub fn new(menu: TileId) -> HackingMenu {
        HackingMenu {
            menu,
            tiles: [None; 8],
            game: None,
            rng: Twister::seeded(1),
            stage: Stage::Intro,
            lines: vec![None; 46],
            entry: None,
            guesses: Vec::new(),
            log: Vec::new(),
            address: 0,
            rates: Rates::default(),
            retry_until: 0.0,
            wait: 0.0,
            access_time: 0.0,
            last_offset: -1,
            last_target: None,
            suppress: false,
            blink_at: 0.0,
            now: 0.0,
            requests: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    /// Opens it (`00765b80`); false without its tiles.
    pub fn open(&mut self, ui: &mut Ui, setup: Setup) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.rng = setup.rng;
        self.address = setup.address;
        self.rates = setup.rates;
        self.retry_until = setup.retry_until;
        for (id, name) in [
            (LOCKED_1, "sHackingLockout3"),
            (LOCKED_2, "sHackingLockout4"),
        ] {
            let s = setting(ui, name);
            if let Some(tile) = self.tile(id) {
                ui.set_string(tile, t::STRING, &s);
            }
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        let Some(game) = setup.game else {
            ui.set_number(self.menu, USER2, 1.0);
            self.stage = Stage::Done;
            if let Some(c) = self.tile(CURSOR) {
                ui.set_number(c, t::VISIBLE, 0.0);
            }
            ui.refresh();
            return true;
        };
        self.game = Some(game);
        ui.set_string(self.menu, USER0, ">");
        let entry = self.tile(ENTRY);
        self.entry = Some(Line::new(entry, "", per_char(self.rates.input)));
        let intro = self.tile(INTRO).unwrap_or(self.menu);
        let texts: Vec<String> = std::iter::once("sHackingSecurityReset".to_string())
            .chain((1..=13).map(|i| format!("sHackingIntro{i:02}")))
            .collect();
        for (i, ((row, input), name)) in INTRO_LINES.iter().zip(&texts).enumerate() {
            let text = setting(ui, name);
            let rate = if *input {
                self.rates.input
            } else {
                self.rates.output
            };
            let line = self.make_line(
                ui,
                intro,
                "hacking_intro_template",
                &text,
                Some(Y_START + row * Y_INTERVAL),
                rate,
                *input,
            );
            self.lines[i] = Some(line);
        }
        self.requests.push(Request::Hum);
        ui.refresh();
        true
    }

    /// A typed line from a template (`0076b300`).
    #[allow(clippy::too_many_arguments)]
    fn make_line(
        &mut self,
        ui: &mut Ui,
        parent: TileId,
        template: &str,
        text: &str,
        y: Option<f32>,
        rate: i32,
        input: bool,
    ) -> Line {
        let tile = ui.instantiate(self.menu, parent, template);
        let mut line = Line::new(tile, text, per_char(rate));
        let left = ui.number(self.menu, USER3);
        if input {
            let intro = self.tile(INTRO).unwrap_or(self.menu);
            line.prompt = ui.instantiate(self.menu, intro, "hacking_intro_template");
            if let Some(p) = line.prompt {
                ui.set_string(p, t::STRING, ">");
            }
        }
        if let (Some(y), Some(tile)) = (y, tile) {
            ui.set_number(tile, t::Y, y);
            ui.set_number(tile, t::X, left);
            if let Some(p) = line.prompt {
                ui.set_number(p, t::X, left);
                ui.set_number(p, t::Y, y);
                ui.set_number(tile, t::X, left + CHAR);
            }
        }
        line
    }

    /// The screen (`00768aa0`).
    fn make_screen(&mut self, ui: &mut Ui) {
        for line in self.lines.iter_mut() {
            if let Some(l) = line.take() {
                for tile in [l.tile, l.prompt].into_iter().flatten() {
                    ui.remove(tile);
                }
            }
        }
        let Some(game) = self.game.clone() else {
            return;
        };
        let header = self.tile(HEADER).unwrap_or(self.menu);
        let attempts = format!("{} {}", game.attempts, setting(ui, "sHackingHeader3"));
        for (i, (text, row)) in [
            (setting(ui, "sHackingHeader"), 0.0),
            (setting(ui, "sHackingHeader2"), 1.0),
            (attempts.clone(), 3.0),
        ]
        .into_iter()
        .enumerate()
        {
            let y = Y_START + row * Y_INTERVAL;
            let line = self.make_line(
                ui,
                header,
                "hacking_intro_template",
                &text,
                Some(y),
                self.rates.output,
                false,
            );
            self.lines[i] = Some(line);
        }
        if let Some(tile) = self.lines[2].as_ref().and_then(|l| l.tile) {
            let x = ui.number(tile, t::X) + CHAR * attempts.len() as f32;
            let y = ui.number(tile, t::Y);
            ui.set_number(header, USER1, x);
            ui.set_number(header, USER2, y);
        }
        // The guess boxes appear with this one (`007694f0`).
        let mut boxes = Line::new(None, "", 1.0);
        boxes.shows = Some((header, USER0));
        self.lines[3] = Some(boxes);
        for _ in 0..game.max_attempts {
            self.add_guess(ui);
        }
        let screen = self.tile(SCREEN).unwrap_or(self.menu);
        for k in 0..FILE_CHARS / LINE_CHARS {
            let text = format!(
                "{} {}",
                world::hacking::PasswordFile::address(self.address, k),
                String::from_utf8_lossy(game.line(k))
            );
            let line = self.make_line(
                ui,
                screen,
                "hacking_password_file_template",
                &text,
                None,
                self.rates.dump,
                false,
            );
            if let Some(tile) = line.tile {
                ui.set_number(tile, t::LISTINDEX, (k % ROWS) as f32);
                ui.set_number(tile, USER0, (k / ROWS) as f32);
            }
            self.lines[FIRST_FILE_LINE + k] = Some(line);
        }
        if let Some(intro) = self.tile(INTRO) {
            ui.set_number(intro, t::VISIBLE, 0.0);
        }
        self.stage = Stage::Dump;
        ui.refresh();
    }

    /// One more guess box (`0076ad50`).
    fn add_guess(&mut self, ui: &mut Ui) {
        let header = self.tile(HEADER).unwrap_or(self.menu);
        if let Some(b) = ui.instantiate(self.menu, header, "hacking_guess_template") {
            ui.set_number(b, t::LISTINDEX, self.guesses.len() as f32);
            self.guesses.push(b);
        }
    }

    /// The screen lines' tiles, in order.
    fn file_line(&self, k: usize) -> Option<TileId> {
        self.lines
            .get(FIRST_FILE_LINE + k)
            .and_then(|l| l.as_ref())
            .and_then(|l| l.tile)
    }

    /// The cursor after a line's text (`0076ac10`).
    fn cursor_after(&self, ui: &mut Ui, tile: TileId) {
        let Some(cursor) = self.tile(CURSOR) else {
            return;
        };
        let len = ui
            .string(tile, t::STRING)
            .filter(|s| !s.starts_with(' '))
            .map_or(0, |s| s.len());
        let mut x = ui.number(tile, t::X) + CHAR * len as f32;
        let mut y = ui.number(tile, t::Y);
        let top = ui.tiles[cursor].parent;
        let mut at = ui.tiles[tile].parent;
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
    }

    /// The cursor after the entry (`0076ab40`), or at its line (`00767c90`
    /// case 1's end) with `y`.
    fn cursor_at_entry(&self, ui: &mut Ui, y: bool) {
        let (Some(cursor), Some(entry)) = (self.tile(CURSOR), self.tile(ENTRY)) else {
            return;
        };
        let len = if ui.number(entry, t::VISIBLE) != 0.0 {
            ui.string(entry, t::STRING).map_or(0, |s| s.len())
        } else {
            0
        };
        let top = ui.tiles[cursor].parent;
        let (mut sx, mut sy) = (0.0, 0.0);
        let mut at = Some(entry);
        while let Some(a) = at {
            if Some(a) == top {
                break;
            }
            sx += ui.number(a, t::X);
            sy += ui.number(a, t::Y);
            at = ui.tiles[a].parent;
        }
        ui.set_number(cursor, t::X, CHAR * len as f32 + sx);
        if y {
            ui.set_number(cursor, t::Y, sy);
        }
    }

    /// Every frame (`00767c90`): `over` is the tile under the pointer and
    /// `pointer_x` its x in menu units.
    pub fn update(&mut self, ui: &mut Ui, now: f64, over: Option<TileId>, pointer_x: Option<f32>) {
        if self.closed {
            return;
        }
        self.now = now;
        if let Some(e) = self.entry.as_mut() {
            e.step(ui, now);
        }
        self.blink(ui, now);
        match self.stage {
            Stage::Intro => self.intro(ui, now),
            Stage::Dump => {
                let next = (0..self.lines.len())
                    .find(|&i| self.lines[i].as_ref().is_some_and(|l| !l.done));
                match next {
                    Some(i) => {
                        self.requests.push(Request::Keep(sound::SCROLL));
                        if let Some(l) = self.lines[i].as_mut() {
                            l.step(ui, now);
                        }
                        if let Some(tile) = self.lines[i].as_ref().and_then(|l| l.tile) {
                            self.cursor_after(ui, tile);
                        }
                    }
                    None => {
                        self.cursor_at_entry(ui, true);
                        self.stage = Stage::Playing;
                    }
                }
            }
            Stage::Playing => {
                self.cursor_at_entry(ui, false);
                if self.game.as_ref().is_some_and(|g| g.attempts == 1) {
                    if let Some(l) = self.lines[1].as_mut() {
                        l.step(ui, now);
                    }
                }
                let screen = self.tile(SCREEN);
                if let Some(tile) = over.filter(|&o| ui.tiles[o].parent == screen) {
                    self.hover(ui, tile, pointer_x);
                }
            }
            Stage::LockingOut => {
                self.cursor_at_entry(ui, false);
                self.scroll(ui, now);
            }
            Stage::Done => {
                self.cursor_at_entry(ui, false);
                let granted = self
                    .game
                    .as_ref()
                    .is_some_and(|g| g.outcome == Some(Outcome::Granted));
                if granted && now > self.access_time {
                    self.close(ui, now);
                }
            }
        }
        ui.refresh();
    }

    /// The intro (`00769830`).
    fn intro(&mut self, ui: &mut Ui, now: f64) {
        // "SECURITY RESET..." only within the retry time.
        if let Some(tile) = self.lines[0].as_ref().and_then(|l| l.tile) {
            let shown = now <= self.retry_until;
            ui.set_number(tile, t::VISIBLE, f32::from(shown));
        }
        let Some(i) =
            (0..INTRO_LINES.len()).find(|&i| self.lines[i].as_ref().is_some_and(|l| !l.done))
        else {
            if now > self.wait {
                self.make_screen(ui);
            }
            return;
        };
        let Some(line) = self.lines[i].as_mut() else {
            return;
        };
        match line.prompt {
            None => {
                if now > self.wait {
                    line.step(ui, now);
                    self.requests.push(Request::Keep(sound::SCROLL));
                }
            }
            Some(prompt) => {
                if ui.number(prompt, t::VISIBLE) == 0.0 {
                    ui.set_number(prompt, t::VISIBLE, 1.0);
                    self.wait = now + 1000.0;
                }
                if now > self.wait {
                    let ms = per_char(self.rates.input);
                    let n = line.step(ui, now);
                    for k in 0..n {
                        self.requests
                            .push(Request::Sound(sound::SINGLE, k as f64 * ms / n as f64));
                    }
                    if line.done {
                        self.requests.push(Request::Sound(sound::ENTER, 0.0));
                        self.wait = now + 500.0;
                    }
                }
            }
        }
        // (Typing shows line 0 again while it types, as the game's does.)
        if let Some(tile) = self.lines[i].as_ref().and_then(|l| l.tile) {
            self.cursor_after(ui, tile);
        }
    }

    /// The cursor's blinking (`0076a9d0`).
    fn blink(&mut self, ui: &mut Ui, now: f64) {
        let Some(cursor) = self.tile(CURSOR) else {
            return;
        };
        if self.stage == Stage::Done && ui.number(self.menu, USER2) != 0.0 {
            return;
        }
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

    /// The lockout's scroll (`0076a3f0`).
    fn scroll(&mut self, ui: &mut Ui, now: f64) {
        if now > self.access_time {
            let steps = if self.access_time == 0.0 {
                1.0
            } else {
                ((now - self.access_time) / 50.0).floor() + 1.0
            };
            let up = ui.number(self.menu, USER1) + Y_INTERVAL * steps as f32;
            ui.set_number(self.menu, USER1, up);
            self.access_time = now + 50.0;
            self.requests.push(Request::Keep(sound::SCROLL));
        }
        // The depth rect (the screen's parent) gone above the top.
        let depth = self.tile(SCREEN).and_then(|s| ui.tiles[s].parent);
        if let Some(d) = depth {
            let (_, y) = ui.screen_position(d);
            if y + ui.number(d, t::HEIGHT) < 0.0 {
                ui.set_number(self.menu, USER2, 1.0);
                self.stage = Stage::Done;
            }
        }
    }

    /// The character under the pointer on a line (`00769700`): its index
    /// in the line's text, or -1.
    fn char_under(ui: &mut Ui, tile: TileId, pointer_x: Option<f32>) -> i32 {
        let Some(px) = pointer_x else {
            return -1;
        };
        let (x, _) = ui.screen_position(tile);
        let w = ui.number(tile, t::WIDTH);
        if px < x || px > x + w {
            return -1;
        }
        ((px - x) / CHAR) as i32
    }

    /// The pointer on a screen line (`00769b50`).
    fn hover(&mut self, ui: &mut Ui, tile: TileId, pointer_x: Option<f32>) {
        let offset = Self::char_under(ui, tile, pointer_x) - PREFIX as i32;
        if offset == self.last_offset && Some(tile) == self.last_target {
            return;
        }
        if offset < 0 && self.last_offset < 0 {
            return;
        }
        self.last_offset = offset;
        self.last_target = Some(tile);
        if offset < 0 || offset >= LINE_CHARS as i32 {
            self.clear_entry(ui);
            self.suppress = false;
            return;
        }
        let row = ui.number(tile, t::LISTINDEX) as usize;
        let col = ui.number(tile, USER0) as usize;
        let at = (col * ROWS + row) * LINE_CHARS + offset as usize;
        let Some(game) = self.game.as_ref() else {
            return;
        };
        if at >= FILE_CHARS {
            return;
        }
        if self.suppress {
            let letter = game.chars[at].is_ascii_alphabetic();
            self.clear_entry(ui);
            if !letter {
                self.suppress = false;
                self.last_target = None;
            }
            return;
        }
        let selection = game.selection(at);
        let text = String::from_utf8_lossy(&game.text(selection)).into_owned();
        let start = match selection {
            Selection::Word(k) => game.positions[k],
            Selection::Brackets { start, .. } => start,
            Selection::Char(at) => at,
        };
        if let Some(entry) = self.entry.as_mut() {
            if entry.text != text {
                entry.set_text(&text);
                self.requests.push(Request::StopTyping);
                let ms = entry.ms;
                for k in 0..text.len() {
                    self.requests
                        .push(Request::Sound(sound::SINGLE, k as f64 * ms));
                }
            }
        }
        self.highlight(ui, start, &text);
    }

    /// The highlight: id 2's `user0`–`user2` the first part, `user3`–`user5`
    /// the rest on the next line (the code's floating texts).
    fn highlight(&self, ui: &mut Ui, start: usize, text: &str) {
        let Some(screen) = self.tile(SCREEN) else {
            return;
        };
        let line = start / LINE_CHARS;
        let first = (LINE_CHARS - start % LINE_CHARS).min(text.len());
        let place = |ui: &mut Ui, k: usize, offset: usize| -> Option<(f32, f32)> {
            let tile = self.file_line(k)?;
            Some((
                ui.number(tile, t::X) + CHAR * (PREFIX + offset) as f32,
                ui.number(tile, t::Y),
            ))
        };
        if let Some((x, y)) = place(ui, line, start % LINE_CHARS) {
            ui.set_string(screen, USER0, &text[..first]);
            ui.set_number(screen, USER1, x);
            ui.set_number(screen, USER2, y);
        }
        match (first < text.len())
            .then(|| place(ui, line + 1, 0))
            .flatten()
        {
            Some((x, y)) => {
                ui.set_string(screen, USER3, &text[first..]);
                ui.set_number(screen, USER4, x);
                ui.set_number(screen, USER5, y);
            }
            None => ui.set_number(screen, USER4, -1.0),
        }
    }

    /// The entry and the highlight cleared (`00768020`).
    fn clear_entry(&mut self, ui: &mut Ui) {
        if let Some(e) = self.entry.as_mut() {
            e.set_text("");
            if let Some(tile) = e.tile {
                ui.set_string(tile, t::STRING, "");
            }
        }
        if let Some(screen) = self.tile(SCREEN) {
            ui.set_number(screen, USER1, -1.0);
            ui.set_number(screen, USER4, -1.0);
        }
    }

    /// Log lines, the newest at the bottom (`00767ef0`).
    fn add_log(&mut self, ui: &mut Ui, lines: &[String]) {
        let n = lines.len();
        for &tile in &self.log {
            let i = ui.number(tile, t::LISTINDEX);
            ui.set_number(tile, t::LISTINDEX, i + n as f32);
        }
        let log = self.tile(LOG).unwrap_or(self.menu);
        for (i, text) in lines.iter().enumerate() {
            if let Some(tile) = ui.instantiate(self.menu, log, "hacking_password_log_template") {
                ui.set_number(tile, t::LISTINDEX, (n - i - 1) as f32);
                ui.set_string(tile, t::STRING, text);
                self.log.push(tile);
            }
        }
    }

    /// A choice on the screen (`00766b80`, stage 2).
    fn choose(&mut self, ui: &mut Ui, now: f64) {
        let Some(entry) = self.entry.as_mut() else {
            return;
        };
        entry.finish(ui);
        let picked = entry.text.clone();
        if picked.is_empty() {
            return;
        }
        // Where the pointer is: the bracket's start.
        let Some(at) = self.pointer_at(ui) else {
            return;
        };
        self.requests.push(Request::Sound(sound::ENTER, 0.0));
        let texts = |name: &str| ui.setting_text(name).unwrap_or_default();
        let Some(game) = self.game.as_mut() else {
            return;
        };
        let before = game.attempts;
        let choice = game.choose(at, &mut self.rng, &texts);
        let attempts = game.attempts;
        let max = game.max_attempts;
        let outcome = choice.outcome;
        self.add_log(ui, &choice.lines);
        match outcome {
            Some(Outcome::Granted) => {
                self.requests.push(Request::Sound(sound::GOOD, 250.0));
                self.access_time = now + 3000.0;
                self.stage = Stage::Done;
                self.requests.push(Request::Granted);
            }
            Some(Outcome::LockedOut) => {
                self.requests.push(Request::Sound(sound::BAD, 250.0));
                self.stage = Stage::LockingOut;
                self.requests.push(Request::LockedOut);
            }
            None if attempts < before => {
                self.requests.push(Request::Sound(sound::BAD, 250.0));
            }
            None => {}
        }
        if attempts < before {
            if let Some(b) = self.guesses.pop() {
                ui.remove(b);
            }
        }
        // Refilled: back to one a guess (`0076ad50` until `iTotGuesses`).
        if attempts > before {
            while (self.guesses.len() as u32) < max {
                self.add_guess(ui);
            }
        }
        match choice.warning {
            Some(true) => {
                let text = setting(ui, "sHackingWarning");
                if let Some(l) = self.lines[1].as_mut() {
                    l.set_text(&text);
                    l.finish(ui);
                    l.flash = Some((
                        f64::from(self.rates.flash_on),
                        f64::from(self.rates.flash_off),
                        0.0,
                    ));
                }
            }
            Some(false) => {
                let text = setting(ui, "sHackingHeader2");
                if let Some(l) = self.lines[1].as_mut() {
                    l.set_text(&text);
                    l.flash = None;
                    l.finish(ui);
                }
            }
            None => {}
        }
        // The screen's lines again (a dud turns to dots).
        if let Some(game) = self.game.clone() {
            for k in 0..FILE_CHARS / LINE_CHARS {
                if let Some(tile) = self.file_line(k) {
                    let text = format!(
                        "{} {}",
                        world::hacking::PasswordFile::address(self.address, k),
                        String::from_utf8_lossy(game.line(k))
                    );
                    if let Some(l) = self.lines[FIRST_FILE_LINE + k].as_mut() {
                        l.text = text.clone();
                        l.typed = Some(text.len());
                    }
                    ui.set_string(tile, t::STRING, &text);
                }
            }
        }
        let header3 = setting(ui, "sHackingHeader3");
        if let Some(l) = self.lines[2].as_mut() {
            l.set_text(&format!("{attempts} {header3}"));
            l.finish(ui);
        }
        self.clear_entry(ui);
        self.suppress = true;
        ui.refresh();
    }

    /// The screen position under the pointer now (the click's
    /// `line × 12 − 7 + character`).
    fn pointer_at(&self, ui: &mut Ui) -> Option<usize> {
        let tile = self.last_target?;
        let offset = self.last_offset;
        if !(0..LINE_CHARS as i32).contains(&offset) {
            return None;
        }
        let row = ui.number(tile, t::LISTINDEX) as usize;
        let col = ui.number(tile, USER0) as usize;
        Some((col * ROWS + row) * LINE_CHARS + offset as usize)
    }

    /// Leaving (`00766aa0`): the menu closes; the retry time is the
    /// caller's (`iHackingRetryMilliseconds` from now, the destructor's).
    pub fn close(&mut self, ui: &mut Ui, _now: f64) {
        if self.closed {
            return;
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
        self.requests.push(Request::Close);
    }
}

impl MenuCode for HackingMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..8).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `00766b80`. The time is the last update's.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        let now = self.now;
        let tile = tile.or_else(|| usize::try_from(id).ok().and_then(|i| self.tile(i)));
        match self.stage {
            Stage::Intro => {
                if self.retry_until < now {
                    self.make_screen(ui);
                }
            }
            Stage::Dump => {
                for l in self.lines.iter_mut().flatten() {
                    l.finish(ui);
                }
                ui.refresh();
            }
            Stage::Playing => {
                let screen = self.tile(SCREEN);
                if tile.is_some_and(|t| ui.tiles[t].parent == screen) {
                    self.choose(ui, now);
                }
            }
            _ => {}
        }
    }

    /// `00767ff0`: the pointer off a tile.
    fn unmouseover(&mut self, ui: &mut Ui, _id: i32, _tile: TileId) {
        self.clear_entry(ui);
        self.suppress = false;
        self.last_target = None;
    }

    /// `00767b80`: code 10 leaves.
    fn special_key(&mut self, ui: &mut Ui, code: i32, _now: f64) -> bool {
        if code == LEAVE {
            self.stage = Stage::Done;
            if !self.closed {
                // `00766aa0`: the menu fades out over 0.75 s (`menufade`).
                ui.set_number(self.menu, t::MENUFADE, LEAVE_FADE);
                self.close(ui, self.now);
                self.requests.push(Request::Leave);
            }
            return true;
        }
        false
    }
}

/// The code that leaves the menu (`00767b80`).
pub const LEAVE: i32 = 10;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;
    use world::hacking::Game;

    /// A game on a screen laid out by hand: "SPRING" on line 0, "STRING"
    /// running from line 1 onto line 2, brackets on line 3.
    fn game() -> Game {
        let mut chars = vec![b'.'; FILE_CHARS];
        chars[2..8].copy_from_slice(b"SPRING");
        chars[20..26].copy_from_slice(b"STRING");
        chars[36..40].copy_from_slice(b"<..>");
        Game {
            length: 6,
            words: vec![b"SPRING".to_vec(), b"STRING".to_vec()],
            positions: vec![2, 20],
            password: b"STRING".to_vec(),
            chars,
            attempts: 4,
            max_attempts: 4,
            used_reset: true,
            used_brackets: Vec::new(),
            outcome: None,
        }
    }

    fn opened(game: Option<Game>, retry_until: f64) -> (Ui, HackingMenu) {
        let mut ui = test_support::ui();
        let mut m = HackingMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::hacking_menu(), &mut m);
        let setup = Setup {
            game,
            rng: Twister::seeded(1),
            address: 0xF4A0,
            rates: Rates::default(),
            retry_until,
        };
        assert!(m.open(&mut ui, setup));
        (ui, m)
    }

    fn text(ui: &mut Ui, tile: Option<TileId>) -> String {
        tile.and_then(|t| ui.string(t, t::STRING))
            .unwrap_or_default()
    }

    /// Runs frames every `step` ms until `until` or the stage changes.
    fn run(ui: &mut Ui, m: &mut HackingMenu, from: f64, until: f64, step: f64) -> f64 {
        let stage = m.stage;
        let mut now = from;
        while now < until && m.stage == stage {
            m.update(ui, now, None, None);
            now += step;
        }
        now
    }

    /// `00765b80`, `00769830`: the intro types line by line, the input
    /// lines with ">" and a pause, then the screen comes.
    #[test]
    fn the_intro_types_then_the_screen_dumps() {
        let (mut ui, mut m) = opened(Some(game()), 0.0);
        assert_eq!(m.stage, Stage::Intro);
        assert!(m.requests.contains(&Request::Hum));
        // A little later the first lines are typing.
        let now = run(&mut ui, &mut m, 1.0, 1500.0, 16.0);
        let first = m.lines[1].as_ref().and_then(|l| l.tile);
        assert_eq!(
            text(&mut ui, first),
            "WELCOME TO ROBCO INDUSTRIES (TM) TERMLINK"
        );
        // "SECURITY RESET..." typed but hidden outside the retry time.
        let reset = m.lines[0].as_ref().and_then(|l| l.tile).unwrap();
        assert_eq!(ui.number(reset, t::VISIBLE), 0.0);
        // The rest, then the screen.
        let now = run(&mut ui, &mut m, now, 60_000.0, 16.0);
        assert_eq!(m.stage, Stage::Dump);
        assert!(m.requests.contains(&Request::Sound(sound::ENTER, 0.0)));
        let now = run(&mut ui, &mut m, now, 60_000.0, 16.0);
        assert_eq!(m.stage, Stage::Playing);
        let header: Vec<String> = (0..3)
            .map(|i| text(&mut ui, m.lines[i].as_ref().and_then(|l| l.tile)))
            .collect();
        assert_eq!(
            header,
            vec![
                "ROBCO INDUSTRIES (TM) TERMLINK PROTOCOL",
                "ENTER PASSWORD NOW",
                "4 ATTEMPT(S) LEFT:"
            ]
        );
        assert_eq!(m.guesses.len(), 4);
        assert_eq!(text(&mut ui, m.file_line(0)), "0xF4A0 ..SPRING....");
        assert_eq!(text(&mut ui, m.file_line(1)), "0xF4AC ........STRI");
        let _ = now;
    }

    /// A click in the intro outside the retry time makes the screen; in
    /// the dump finishes it.
    #[test]
    fn clicks_skip_the_intro_and_the_dump() {
        let (mut ui, mut m) = opened(Some(game()), 0.0);
        m.update(&mut ui, 10.0, None, None);
        m.click(&mut ui, 0, None, 20.0);
        assert_eq!(m.stage, Stage::Dump);
        m.update(&mut ui, 30.0, None, None);
        m.click(&mut ui, 2, None, 30.0);
        assert_eq!(text(&mut ui, m.file_line(33)).len(), 19);
        // Within the retry time the intro can't be skipped.
        let (mut ui, mut m) = opened(Some(game()), 100_000.0);
        m.update(&mut ui, 10.0, None, None);
        m.click(&mut ui, 0, None, 20.0);
        assert_eq!(m.stage, Stage::Intro);
        let reset = m.lines[0].as_ref().and_then(|l| l.tile).unwrap();
        assert_eq!(ui.number(reset, t::VISIBLE), 1.0);
    }

    fn playing() -> (Ui, HackingMenu, f64) {
        let (mut ui, mut m) = opened(Some(game()), 0.0);
        m.update(&mut ui, 1.0, None, None);
        m.click(&mut ui, 0, None, 1.0);
        m.update(&mut ui, 2.0, None, None);
        m.click(&mut ui, 2, None, 2.0);
        let now = run(&mut ui, &mut m, 3.0, 10_000.0, 16.0);
        assert_eq!(m.stage, Stage::Playing);
        m.requests.clear();
        (ui, m, now)
    }

    /// The pointer x over a line's character.
    fn x_of(ui: &mut Ui, m: &HackingMenu, line: usize, offset: usize) -> f32 {
        let tile = m.file_line(line).unwrap();
        let (x, _) = ui.screen_position(tile);
        x + CHAR * (PREFIX + offset) as f32 + 1.0
    }

    /// `00769b50`: a word (here running onto the next line) is picked
    /// whole, typed on the entry line, and highlighted in two parts.
    #[test]
    fn the_pointer_picks_and_highlights() {
        let (mut ui, mut m, now) = playing();
        let line1 = m.file_line(1).unwrap();
        let x = x_of(&mut ui, &m, 1, 9);
        m.update(&mut ui, now, Some(line1), Some(x));
        let screen = m.tile(SCREEN).unwrap();
        assert_eq!(ui.string(screen, USER0).as_deref(), Some("STRI"));
        assert_eq!(ui.string(screen, USER3).as_deref(), Some("NG"));
        assert!(ui.number(screen, USER4) >= 0.0);
        let sounds = m
            .requests
            .iter()
            .filter(|r| matches!(r, Request::Sound(sound::SINGLE, _)))
            .count();
        assert_eq!(sounds, 6);
        let later = run(&mut ui, &mut m, now + 16.0, now + 1000.0, 16.0);
        assert_eq!(text(&mut ui, m.tile(ENTRY)), "STRING");
        let _ = later;
    }

    /// `00766b80`: a wrong word costs an attempt and a guess box; the
    /// password opens the terminal 3 s later.
    #[test]
    fn choosing_wrong_then_right() {
        let (mut ui, mut m, now) = playing();
        let line0 = m.file_line(0).unwrap();
        let x = x_of(&mut ui, &m, 0, 3);
        m.update(&mut ui, now, Some(line0), Some(x));
        m.click(&mut ui, -1, Some(line0), now + 1.0);
        assert_eq!(m.game.as_ref().unwrap().attempts, 3);
        assert_eq!(m.guesses.len(), 3);
        assert!(m.requests.contains(&Request::Sound(sound::BAD, 250.0)));
        let log: Vec<String> = m
            .log
            .iter()
            .map(|&t| ui.string(t, t::STRING).unwrap_or_default())
            .collect();
        assert_eq!(log, vec![">SPRING", ">Entry denied", ">5/6 correct."]);
        assert_eq!(
            text(&mut ui, m.lines[2].as_ref().and_then(|l| l.tile)),
            "3 ATTEMPT(S) LEFT:"
        );
        // The pointer has to leave the letters before it picks again.
        m.update(&mut ui, now + 20.0, Some(line0), Some(x + 1.0));
        assert_eq!(text(&mut ui, m.tile(ENTRY)), "");
        let line2 = m.file_line(2).unwrap();
        let x2 = x_of(&mut ui, &m, 2, 6);
        m.update(&mut ui, now + 40.0, Some(line2), Some(x2));
        let line1 = m.file_line(1).unwrap();
        let x1 = x_of(&mut ui, &m, 1, 9);
        m.update(&mut ui, now + 80.0, Some(line1), Some(x1));
        m.click(&mut ui, -1, Some(line1), now + 100.0);
        assert_eq!(m.stage, Stage::Done);
        assert!(m.requests.contains(&Request::Granted));
        run(&mut ui, &mut m, now + 200.0, now + 3200.0, 16.0);
        assert!(m.closed);
        assert!(m.requests.contains(&Request::Close));
        // Handed over to the terminal's menu, not left: no power down.
        assert!(!m.requests.contains(&Request::Leave));
    }

    /// Down to the last attempt the header warns and flashes; at none the
    /// screen scrolls away and the lockout shows.
    #[test]
    fn the_warning_and_the_lockout() {
        let (mut ui, mut m, now) = playing();
        m.game.as_mut().unwrap().attempts = 2;
        let line0 = m.file_line(0).unwrap();
        let x = x_of(&mut ui, &m, 0, 3);
        m.update(&mut ui, now, Some(line0), Some(x));
        m.click(&mut ui, -1, Some(line0), now + 1.0);
        let warning = m.lines[1].as_ref().and_then(|l| l.tile);
        assert_eq!(text(&mut ui, warning), "!!! WARNING: LOCKOUT IMMINENT !!!");
        let mut seen = std::collections::BTreeSet::new();
        let mut t = now + 2.0;
        while t < now + 3000.0 {
            m.update(&mut ui, t, None, None);
            seen.insert(ui.number(warning.unwrap(), t::VISIBLE) as i32);
            t += 16.0;
        }
        assert_eq!(seen.len(), 2, "it flashes");
        // The last attempt: a bracket's single character still counts.
        let line3 = m.file_line(3).unwrap();
        m.unmouseover(&mut ui, 0, line0);
        let x3 = x_of(&mut ui, &m, 3, 1);
        m.update(&mut ui, t, Some(line3), Some(x3));
        m.click(&mut ui, -1, Some(line3), t + 1.0);
        assert_eq!(m.stage, Stage::LockingOut);
        assert!(m.requests.contains(&Request::LockedOut));
        run(&mut ui, &mut m, t + 2.0, t + 10_000.0, 16.0);
        assert_eq!(m.stage, Stage::Done);
        assert_eq!(ui.number(m.menu, USER2), 1.0);
    }

    /// A locked-out terminal opens on the lockout; code 10 leaves.
    #[test]
    fn locked_out_and_leaving() {
        let (mut ui, mut m) = opened(None, 0.0);
        assert_eq!(m.stage, Stage::Done);
        assert_eq!(ui.number(m.menu, USER2), 1.0);
        assert_eq!(text(&mut ui, m.tile(LOCKED_1)), "TERMINAL LOCKED");
        assert!(m.special_key(&mut ui, LEAVE, 1.0));
        assert!(m.closed);
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 1.0);
        // Leaving fades it out over 0.75 s (`00766aa0`).
        assert_eq!(ui.number(m.menu, t::MENUFADE), LEAVE_FADE);
        let close = m.requests.iter().position(|r| *r == Request::Close);
        let leave = m.requests.iter().position(|r| *r == Request::Leave);
        assert!(close.is_some() && leave > close);
        // Only once.
        assert!(m.special_key(&mut ui, LEAVE, 2.0));
        assert_eq!(
            m.requests.iter().filter(|r| **r == Request::Leave).count(),
            1
        );
    }
}
