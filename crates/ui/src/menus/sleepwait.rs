//! Waiting and sleeping (`menus\sleep_wait_menu.xml`, class
//! `SleepWaitMenu` 1012, vtable `010763ac` in FalloutNV.exe; findings
//! `menus.md` §11). The rules of what the hours do are the world's
//! (`world::living::sleep`); this is the menu's own code:
//!
//! * tiles by `id` (`007a8ab0`, the message menu's `SetTile`): 0
//!   `SWM_HowManyText`, 1 `SWM_Scrollbar` (the `scrollbar_horiz.xml`
//!   prefab: 24 items, 1 visible, `_current_value` 0..23 worked out by the
//!   file from its arrows, pages, the wheel and dragging the marker), 2
//!   `SWM_HoursChosen`, 3 `SWM_CurrentTime`, 4 `SWM_WaitButton`, 5
//!   `SWM_CancelButton`; the file's `_PCButton_W` clicks Wait, `_PCButton_E`
//!   Cancel;
//! * opening (`007bfc30(sleeping)`): `UIPopUpMessageGeneral`; the word is
//!   `sSleep` or `sWait`; tile 0 = "`sHowManyWait` word?", tile 4 = the word
//!   with its first letter upper-cased, tile 2 = "1 `sHour`", tile 3 = the
//!   time line, tile 5 = `sCancel`; `SWM_Background`'s alpha =
//!   `fMenuBackgroundOpacity` × 255; shown;
//! * the time line (`007c0000`): "%s, %s, %d:%02d %s" with the weekday's
//!   name (`sDaySunday`…, by [`TimeLine::weekday`]), the date, the 12-hour
//!   clock's hour and minutes, `sAMTime` before 12:00, else `sPMTime`;
//! * every frame (`007c03a0`): when the bar's `_current_value` has
//!   changed, `UIMenuFocus` (sound 4) if Wait can be used, and tile 2 =
//!   "%d %s" with the value + 1 and `sHour` (value 0) or `sHours`;
//! * Wait (`007c0220`, id 4): `UIMenuOK`; an autosave when the INI says so
//!   (`00850a40`); for a sleep the fade to black (`00700960`); the menu is
//!   now waiting; Wait's and the bar's `target` go off; the world gets the
//!   hours (the value truncated + 1) and whether it's a sleep
//!   (`005c1a00`; Times Slept, followers: the world's); Cancel (id 5):
//!   `UIMenuCancel`, hours and flag cleared, closed (`007c01b0`: a sleep's
//!   fade released, `6002` set, the fade-out);
//! * Page Up / Page Down (`007c04d0`, specials 15 / 16) with Wait usable:
//!   `_current_value` −2 / +2 (its operators kept) and `_SetInCode` poked;
//! * while waiting (`007c0580`, each frame): the Rest control (15, T)
//!   pressed sets a flag that's never cleared; with it set and the control
//!   held, or the hours over, Cancel is clicked unless a message box is
//!   showing; else the frame's seconds add up and at 1.0 an hour passes
//!   (the world's), the bar's `_current_value` = hours left − 1 (operators
//!   dropped) with `_SetInCode` poked, and the time line is written again.

use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\sleep_wait_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1012;
/// The sound it opens with.
pub const OPEN_SOUND: &str = "UIPopUpMessageGeneral";
/// What the page keys move the bar by (`007c04d0`: `0104a494` −2.0 for
/// Page Up, `010162c0` 2.0 for Page Down).
pub const PAGE_STEP: f32 = 2.0;

/// The clock as the time line shows it: the weekday (Sunday 0), the date
/// as the game writes it ("10.19.81"), the 12-hour clock's hour and
/// minutes and whether it's PM (`world::living::sleep::Clock` works them
/// out as the game does).
#[derive(Debug, Clone, PartialEq)]
pub struct TimeLine {
    pub weekday: usize,
    pub date: String,
    pub hour: i32,
    pub minute: i32,
    pub pm: bool,
}

/// What the menu asks the world to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Wait or Sleep accepted: the hours chosen and whether it's a sleep
    /// (`005c1a00(hours, sleeping)`).
    Begin { hours: u32, sleeping: bool },
    /// An autosave first (`00850a40`), when the INI's `bSaveOnRest` /
    /// `bSaveOnWait` says so.
    Autosave,
    /// A sleep: the screen fades to black (`00700960`).
    FadeToBlack,
    /// Cancelled, or the hours over: hours and the sleeping flag cleared
    /// (`005c1a00(0, 0)`).
    Cancel,
    /// A sleep's menu closed: the fade back (`007010e0(0, 0)`).
    FadeBack,
}

/// What a frame while the menu is open asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Frame {
    Nothing,
    /// A real second has passed: an hour passes in the world; then
    /// [`SleepWaitMenu::hour_passed`].
    HourPasses,
    /// Cancel was clicked (the requests say so).
    Cancelled,
}

/// The weekday names' text settings, Sunday first (the exe's table at
/// `011895b8`).
const WEEKDAYS: [&str; 7] = [
    "sDaySunday",
    "sDayMonday",
    "sDayTuesday",
    "sDayWednesday",
    "sDayThursday",
    "sDayFriday",
    "sDaySaturday",
];

/// The sleep/wait menu.
#[derive(Debug)]
pub struct SleepWaitMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..5.
    pub tiles: [Option<TileId>; 6],
    /// Sleeping (+0x45) rather than waiting.
    pub sleeping: bool,
    /// Waiting the hours out (+0x44, from Wait's click).
    pub waiting: bool,
    /// The Rest control pressed since the menu opened (+0x46).
    rest_pressed: bool,
    /// Real seconds since the last hour (+0x40).
    timer: f32,
    /// The bar's value when tile 2 was last written (+0x48).
    last_value: f32,
    /// Whether Wait autosaves (the INI, read by the caller).
    pub autosave: bool,
    pub requests: Vec<Request>,
    /// Sounds for the caller to play (editor IDs).
    pub sounds: Vec<String>,
    pub closed: bool,
}

impl SleepWaitMenu {
    pub fn new(menu: TileId) -> SleepWaitMenu {
        SleepWaitMenu {
            menu,
            tiles: [None; 6],
            sleeping: false,
            waiting: false,
            rest_pressed: false,
            // The constructor (`007bfb30`) zeroes the timer, the waiting
            // flag and the hours' text value.
            timer: 0.0,
            last_value: 0.0,
            autosave: true,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn text(ui: &Ui, name: &str) -> String {
        ui.setting_text(name).unwrap_or_default()
    }

    /// Opens it (`007bfc30`) in sleep or wait mode; `background_alpha` is
    /// `fMenuBackgroundOpacity` × 255. False when the file lacks a tile the
    /// code needs (`007abdb0`: "Sleep Wait Menu Creation Failed").
    pub fn open(
        &mut self,
        ui: &mut Ui,
        sleeping: bool,
        clock: &TimeLine,
        background_alpha: f32,
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.sounds.push(OPEN_SOUND.to_string());
        self.sleeping = sleeping;
        let word = Self::text(ui, if sleeping { "sSleep" } else { "sWait" });
        let prompt = format!("{} {word}?", Self::text(ui, "sHowManyWait"));
        self.set(ui, 0, &prompt);
        // `toupper` on the first character.
        let mut button: Vec<char> = word.chars().collect();
        if let Some(c) = button.first_mut() {
            *c = c.to_ascii_uppercase();
        }
        let button: String = button.into_iter().collect();
        self.set(ui, 4, &button);
        let hour = format!("1 {}", Self::text(ui, "sHour"));
        self.set(ui, 2, &hour);
        self.set_time(ui, clock);
        let cancel = Self::text(ui, "sCancel");
        self.set(ui, 5, &cancel);
        if let Some(bg) = ui.find(self.menu, "SWM_Background") {
            ui.set_number(bg, t::ALPHA, background_alpha);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    fn set(&self, ui: &mut Ui, id: usize, text: &str) {
        if let Some(tile) = self.tiles[id] {
            ui.set_string(tile, t::STRING, text);
        }
    }

    /// The time line (`007c0000`) onto tile 3.
    pub fn set_time(&self, ui: &mut Ui, clock: &TimeLine) {
        let weekday = match WEEKDAYS.get(clock.weekday) {
            Some(name) => Self::text(ui, name),
            None => "Bad Day".to_string(),
        };
        let half = Self::text(ui, if clock.pm { "sPMTime" } else { "sAMTime" });
        let line = format!(
            "{weekday}, {}, {}:{:02} {half}",
            clock.date, clock.hour, clock.minute
        );
        self.set(ui, 3, &line);
    }

    /// The bar's `_current_value`.
    pub fn value(&self, ui: &mut Ui) -> f32 {
        match (self.tiles[1], ui.names.lookup("_current_value")) {
            (Some(bar), Some(id)) => ui.number(bar, id),
            _ => 0.0,
        }
    }

    /// The hours the bar stands for (the value truncated + 1).
    pub fn hours(&self, ui: &mut Ui) -> u32 {
        (self.value(ui) as i32 + 1).max(0) as u32
    }

    fn wait_usable(&self, ui: &mut Ui) -> bool {
        self.tiles[4].is_some_and(|wait| ui.number(wait, t::TARGET) != 0.0)
    }

    /// Every frame (`007c03a0`): the hours' text follows the bar.
    pub fn update(&mut self, ui: &mut Ui) {
        let value = self.value(ui);
        if value == self.last_value {
            return;
        }
        if self.wait_usable(ui) {
            if let Some(s) = menu::menu_sound(4) {
                self.sounds.push(s.to_string());
            }
        }
        self.last_value = value;
        self.write_hours(ui, value);
    }

    fn write_hours(&self, ui: &mut Ui, value: f32) {
        let unit = Self::text(ui, if value <= 0.0 { "sHour" } else { "sHours" });
        // `00406d90`: the float truncated to a whole number.
        let text = format!("{} {unit}", (value + 1.0) as i32);
        self.set(ui, 2, &text);
    }

    /// An hour has passed in the world (`007c0580`'s end): the bar shows
    /// the hours left (`_current_value` = left − 1, its operators dropped,
    /// `_SetInCode` poked) and the time line is written again.
    pub fn hour_passed(&mut self, ui: &mut Ui, hours_left: i32, clock: &TimeLine) {
        if let Some(bar) = self.tiles[1] {
            if let Some(current) = ui.names.lookup("_current_value") {
                ui.set_number(bar, current, (hours_left - 1) as f32);
            }
            if let Some(poke) = ui.names.lookup("_SetInCode") {
                ui.set_number(bar, poke, 1.0);
            }
        }
        self.set_time(ui, clock);
        ui.refresh();
    }

    /// A frame while the menu is open (`007c0580`): `rest_pressed` and
    /// `rest_held` are the Rest control (T) this frame, `hours_left` the
    /// world's, `box_showing` whether a message box is up (then nothing
    /// closes). `dt` is real seconds.
    pub fn frame(
        &mut self,
        ui: &mut Ui,
        dt: f32,
        rest_pressed: bool,
        rest_held: bool,
        hours_left: i32,
        box_showing: bool,
    ) -> Frame {
        if rest_pressed {
            self.rest_pressed = true;
        }
        let hours_over = self.waiting && hours_left < 1;
        if ((self.rest_pressed && rest_held) || hours_over) && !box_showing {
            self.click(ui, 5, None, 0.0);
            return Frame::Cancelled;
        }
        if !self.waiting {
            return Frame::Nothing;
        }
        self.timer += dt;
        if self.timer < 1.0 {
            return Frame::Nothing;
        }
        self.timer = 0.0;
        Frame::HourPasses
    }

    fn close(&mut self, ui: &mut Ui) {
        if self.sleeping {
            self.requests.push(Request::FadeBack);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for SleepWaitMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..6).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        match id {
            4 => {
                if let Some(s) = menu::menu_sound(1) {
                    self.sounds.push(s.to_string());
                }
                if self.autosave {
                    self.requests.push(Request::Autosave);
                }
                if self.sleeping {
                    self.requests.push(Request::FadeToBlack);
                }
                self.waiting = true;
                for id in [4, 1] {
                    if let Some(tile) = self.tiles[id] {
                        ui.set_number(tile, t::TARGET, 0.0);
                    }
                }
                let hours = self.hours(ui);
                self.requests.push(Request::Begin {
                    hours,
                    sleeping: self.sleeping,
                });
                ui.refresh();
            }
            5 => {
                if let Some(s) = menu::menu_sound(2) {
                    self.sounds.push(s.to_string());
                }
                self.requests.push(Request::Cancel);
                self.close(ui);
            }
            _ => {}
        }
    }

    fn special_key(&mut self, ui: &mut Ui, code: i32, _now: f64) -> bool {
        if !self.wait_usable(ui) {
            return false;
        }
        let by = match code {
            menu::special::PAGE_UP => -PAGE_STEP,
            menu::special::PAGE_DOWN => PAGE_STEP,
            _ => return false,
        };
        let Some(bar) = self.tiles[1] else {
            return false;
        };
        let value = self.value(ui) + by;
        if let Some(current) = ui.names.lookup("_current_value") {
            ui.set_base(bar, current, value);
        }
        if let Some(poke) = ui.names.lookup("_SetInCode") {
            ui.set_number(bar, poke, 1.0);
        }
        ui.refresh();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn clock() -> TimeLine {
        TimeLine {
            weekday: 1,
            date: "10.19.81".to_string(),
            hour: 2,
            minute: 30,
            pm: true,
        }
    }

    fn opened(sleeping: bool) -> (Ui, SleepWaitMenu) {
        let mut ui = test_support::ui();
        let mut m = SleepWaitMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::sleep_wait_menu(), &mut m);
        assert!(m.open(&mut ui, sleeping, &clock(), 0.8 * 255.0));
        (ui, m)
    }

    fn text(ui: &mut Ui, m: &SleepWaitMenu, id: usize) -> String {
        ui.string(m.tiles[id].unwrap(), t::STRING)
            .unwrap_or_default()
    }

    /// `007bfc30`: the texts, the background, the sound.
    #[test]
    fn opening_writes_the_texts() {
        let (mut ui, m) = opened(false);
        assert_eq!(text(&mut ui, &m, 0), "How long would you like to wait?");
        assert_eq!(text(&mut ui, &m, 4), "Wait");
        assert_eq!(text(&mut ui, &m, 2), "1 hour");
        assert_eq!(text(&mut ui, &m, 3), "Monday, 10.19.81, 2:30 PM");
        assert_eq!(text(&mut ui, &m, 5), "Cancel");
        assert_eq!(m.sounds, [OPEN_SOUND]);
        let bg = ui.find(m.menu, "SWM_Background").unwrap();
        assert_eq!(ui.number(bg, t::ALPHA), 204.0);
        assert_eq!(ui.number(m.menu, t::VISIBLE), 1.0);
        let (mut ui, m) = opened(true);
        assert_eq!(text(&mut ui, &m, 0), "How long would you like to sleep?");
        assert_eq!(text(&mut ui, &m, 4), "Sleep");
        // A missing tile: creation fails.
        let mut ui = test_support::ui();
        let mut m = SleepWaitMenu::new(0);
        m.menu = test_support::load(&mut ui, "<menu name=\"SleepWaitMenu\"><class>&SleepWaitMenu;</class><text name=\"SWM_HowManyText\"><id>0</id></text></menu>", &mut m);
        assert!(!m.open(&mut ui, false, &clock(), 204.0));
    }

    /// `007c03a0`: the hours' text follows the bar, with the focus sound
    /// while Wait can be used; `007c04d0`: the page keys move it by 2.
    #[test]
    fn the_bar_sets_the_hours() {
        let (mut ui, mut m) = opened(false);
        let bar = m.tiles[1].unwrap();
        let current = ui.names.lookup("_current_value").unwrap();
        m.update(&mut ui);
        assert_eq!(m.sounds.len(), 1);
        ui.set_base(bar, current, 4.0);
        ui.refresh();
        m.update(&mut ui);
        assert_eq!(text(&mut ui, &m, 2), "5 hours");
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuFocus"));
        assert!(m.special_key(&mut ui, menu::special::PAGE_DOWN, 0.0));
        assert_eq!(m.value(&mut ui), 6.0);
        assert!(m.special_key(&mut ui, menu::special::PAGE_UP, 0.0));
        assert!(m.special_key(&mut ui, menu::special::PAGE_UP, 0.0));
        assert!(m.special_key(&mut ui, menu::special::PAGE_UP, 0.0));
        // Kept within 0..23 by the file.
        assert_eq!(m.value(&mut ui), 0.0);
        m.update(&mut ui);
        assert_eq!(text(&mut ui, &m, 2), "1 hour");
        for _ in 0..20 {
            m.special_key(&mut ui, menu::special::PAGE_DOWN, 0.0);
        }
        assert_eq!(m.value(&mut ui), 23.0);
        assert_eq!(m.hours(&mut ui), 24);
        // The file's own arrows move it too (the interface clicks them,
        // pulsing `clicked`).
        let left = ui.find(m.menu, "scrollbar_horiz_left").unwrap();
        let menu_tile = m.menu;
        menu::Interface::default().click(&mut ui, menu_tile, &mut m, left, 0.0);
        assert_eq!(m.value(&mut ui), 22.0);
        m.update(&mut ui);
        assert_eq!(text(&mut ui, &m, 2), "23 hours");
    }

    /// `007c0220`: Wait asks the world for the hours (and the autosave and
    /// a sleep's fade), Wait and the bar can't be used; `007c0580`: an hour
    /// a second until the hours are over, then Cancel; the Rest control
    /// cancels; `007c01b0`: a sleep's menu releases the fade.
    #[test]
    fn waiting_the_hours_out() {
        let (mut ui, mut m) = opened(true);
        let bar = m.tiles[1].unwrap();
        let current = ui.names.lookup("_current_value").unwrap();
        ui.set_base(bar, current, 2.0);
        ui.refresh();
        m.click(&mut ui, 4, None, 0.0);
        assert_eq!(
            m.requests,
            [
                Request::Autosave,
                Request::FadeToBlack,
                Request::Begin {
                    hours: 3,
                    sleeping: true
                }
            ]
        );
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuOK"));
        assert_eq!(ui.number(m.tiles[4].unwrap(), t::TARGET), 0.0);
        assert_eq!(ui.number(bar, t::TARGET), 0.0);
        assert!(!m.special_key(&mut ui, menu::special::PAGE_DOWN, 0.0));
        m.requests.clear();
        // Half a second: nothing; a whole one: an hour.
        assert_eq!(
            m.frame(&mut ui, 0.5, false, false, 3, false),
            Frame::Nothing
        );
        assert_eq!(
            m.frame(&mut ui, 0.6, false, false, 3, false),
            Frame::HourPasses
        );
        m.hour_passed(&mut ui, 2, &clock());
        assert_eq!(m.value(&mut ui), 1.0);
        m.update(&mut ui);
        assert_eq!(text(&mut ui, &m, 2), "2 hours");
        // No focus sound now that Wait can't be used.
        assert_eq!(m.sounds.len(), 2);
        assert_eq!(
            m.frame(&mut ui, 1.0, false, false, 2, false),
            Frame::HourPasses
        );
        m.hour_passed(&mut ui, 1, &clock());
        assert_eq!(
            m.frame(&mut ui, 1.0, false, false, 1, false),
            Frame::HourPasses
        );
        m.hour_passed(&mut ui, 0, &clock());
        // The hours over: Cancel is clicked, unless a message box is up.
        assert_eq!(m.frame(&mut ui, 0.1, false, false, 0, true), Frame::Nothing);
        assert!(!m.closed);
        assert_eq!(
            m.frame(&mut ui, 0.1, false, false, 0, false),
            Frame::Cancelled
        );
        assert!(m.closed);
        assert_eq!(m.requests, [Request::Cancel, Request::FadeBack]);
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuCancel"));
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 1.0);

        // The Rest control: pressed and held, Cancel; a wait's menu has no
        // fade to release.
        let (mut ui, mut m) = opened(false);
        assert_eq!(
            m.frame(&mut ui, 0.1, true, true, 0, false),
            Frame::Cancelled
        );
        assert_eq!(m.requests, [Request::Cancel]);
        // Not autosaving: no request.
        let (mut ui, mut m) = opened(false);
        m.autosave = false;
        m.click(&mut ui, 4, None, 0.0);
        assert_eq!(
            m.requests,
            [Request::Begin {
                hours: 1,
                sleeping: false
            }]
        );
    }
}
