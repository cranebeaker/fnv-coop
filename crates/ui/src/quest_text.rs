//! The HUD's quest text (`QuestReminder` in `hud_main_menu.xml`), read from
//! FalloutNV.exe 1.4.0.525 with the Xbox 360 prototype's names
//! (`HUDMainMenu`, `QuestUpdateManager`; Xbox PDB). Two parts:
//!
//! - **Quest names** (`QuestAdded`): "Quest added", "Quest completed",
//!   "Quest FAILED" or a custom title (a place discovered: "You have
//!   discovered") above the quest's name, which appears in capitals one
//!   letter at a time (`HUDMainMenu::UpdateQuestText`, `0077a650`; each
//!   letter fading in and out by `0077c170`).
//! - **Objectives** (`QuestStages`): an objective shown, or completed
//!   ("COMPLETED: …"), with an empty or filled box, up to three lines
//!   coming in one after another, each held and then faded out while the
//!   rest scroll up (`0077b430`).
//!
//! What's queued comes from the game's state (`world::quest_text`, the
//! objectives' events); the viewer turns it into [`Notice`]s and
//! [`Objective`]s.

use std::collections::VecDeque;

use crate::anim::Animations;
use crate::names::t;
use crate::tile::{TileId, Ui};

/// Nothing new shows until 2 seconds after the last quest or objective was
/// queued (`0077a650`: `0x7d0` ms since `011d96ac`, which `0077a480` and
/// `0077a5b0` stamp).
pub const QUEUE_WAIT_MS: f64 = 2000.0;

/// How many letter tiles `QuestAdded` has besides the title (`0076bfe0`
/// makes 64 text tiles; the first is the title).
pub const LETTERS: usize = 63;

/// How many objective lines `QuestStages` has (32 texts, 32 boxes).
pub const OBJECTIVE_LINES: usize = 32;

/// The empty box of an objective shown and the filled one of a completed
/// one (`0077b430`).
pub const BOX_EMPTY: &str = "Interface\\Shared\\Marker\\glow_square_small.dds";
pub const BOX_FILLED: &str = "Interface\\Shared\\Marker\\glow_square_filled_small.dds";

/// The game settings the quest text is timed by (their defaults are the
/// exe's).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    /// `fQuestCinematicCharacterFadeInDelay`: between letters.
    pub letter_delay: f32,
    /// `fQuestCinematicCharacterRemain`: how long each stays.
    pub letter_remain: f32,
    /// `fQuestCinematicCharacterFadeIn`, `…FadeOut`.
    pub letter_fade_in: f32,
    pub letter_fade_out: f32,
    /// `fQuestCinematicObjectiveFadeInDelay`: between lines.
    pub objective_delay: f32,
    /// `fQuestCinematicObjectivePauseTime`: how long the top line holds.
    pub objective_pause: f32,
    /// `fQuestCinematicObjectiveFadeIn`, `…FadeOut`, `…ScrollTime`.
    pub objective_fade_in: f32,
    pub objective_fade_out: f32,
    pub objective_scroll: f32,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing {
            letter_delay: 0.1,
            letter_remain: 4.0,
            letter_fade_in: 2.0,
            letter_fade_out: 2.0,
            objective_delay: 1.0,
            objective_pause: 3.0,
            objective_fade_in: 0.5,
            objective_fade_out: 0.5,
            objective_scroll: 0.5,
        }
    }
}

/// `QuestUpdateManager::HUD_QUEST_UPDATE_TYPE` (Xbox PDB).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Added,
    Completed,
    Failed,
    Custom,
}

/// A queued quest name (`QuestUpdateManager::QuestUpdate`).
#[derive(Debug, Clone, PartialEq)]
pub struct Notice {
    pub kind: Kind,
    /// A custom notice's title (the others' come from settings).
    pub title: String,
    /// The quest's name, or a custom notice's subtitle.
    pub subtitle: String,
    /// 0 first, 1 normal, 2 last (custom notices; quests' are taken as
    /// normal: `0077a480` doesn't set it).
    pub priority: i32,
    /// 0 left, 1 centre, 2 right (custom notices).
    pub justification: i32,
    pub title_font: Option<i32>,
    pub subtitle_font: Option<i32>,
    /// A custom notice's sound.
    pub sound: String,
}

impl Notice {
    /// A quest added, completed or failed, with its name.
    pub fn quest(kind: Kind, name: &str) -> Notice {
        Notice {
            kind,
            title: String::new(),
            subtitle: name.to_string(),
            priority: 1,
            justification: 0,
            title_font: None,
            subtitle_font: None,
            sound: String::new(),
        }
    }
}

/// A queued objective line (`0077a5b0`: its text, completed, a reminder).
#[derive(Debug, Clone, PartialEq)]
pub struct Objective {
    pub text: String,
    pub completed: bool,
    pub reminder: bool,
}

/// What the game's state lets show this frame (`0077a650`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Gate {
    /// Quest names: no menu up, or the dialogue menu (1009) is (or
    /// `00703d50`'s case), and not in V.A.T.S. (`011f2250` + 8).
    pub names: bool,
    /// Objectives: the character generation menu (1048) isn't up.
    pub objectives: bool,
}

/// The quest text's tiles and queues.
#[derive(Debug, Clone)]
pub struct QuestText {
    pub reminder: TileId,
    pub stages: TileId,
    pub added: TileId,
    /// `QuestAdded`'s text tiles in the game's list order: the title, then
    /// the letters.
    pub added_tiles: Vec<TileId>,
    /// `QuestStages`' lines in the game's list order (its children are
    /// the 32 texts, then the 32 boxes; the game adds a child at the head
    /// of the list, `00a087d0`, so the last made comes first).
    pub texts: Vec<TileId>,
    pub boxes: Vec<TileId>,
    /// `QuestUpdateManager::xQuestNames` (Xbox PDB), sorted by priority.
    pub names: Vec<Notice>,
    /// The objective lines waiting (`HUDMainMenu` +0x258).
    pub objectives: VecDeque<Objective>,
    /// When a quest or objective was last queued, in milliseconds.
    last_queued: Option<f64>,
    /// `HUDMainMenu` +0x260 (names shown) and +0x261 (objectives).
    names_flag: bool,
    objectives_flag: bool,
    wait_complete: i32,
    hold_time: i32,
    is_reminder: i32,
}

/// The brightness the HUD gives these texts (`00774800` kind 0: 255).
const BRIGHT: f32 = 255.0;

impl QuestText {
    /// Places `QuestReminder` and makes its tiles as `0076bfe0` does: at
    /// (2 × safe x + 15, 2 × safe y + 170); 64 `template_justify_left_text`
    /// in `QuestAdded`, 32 `template_filled_checkbox` and 32
    /// `template_justify_left_text` in `QuestStages`.
    pub fn create(ui: &mut Ui, menu: TileId) -> Option<QuestText> {
        let screen = ui.screen_size;
        let (sx, sy) = (screen.safe_x as i32, screen.safe_y as i32);
        let reminder = ui.find(menu, "QuestReminder")?;
        ui.set_number(reminder, t::X, (sx * 2 + 15) as f32);
        ui.set_number(reminder, t::Y, (sy * 2 + 170) as f32);
        let stages = ui.find(reminder, "QuestStages")?;
        let added = ui.find(reminder, "QuestAdded")?;
        let mut added_tiles = Vec::new();
        for _ in 0..LETTERS + 1 {
            added_tiles.push(ui.instantiate(menu, added, "template_justify_left_text")?);
        }
        let mut boxes = Vec::new();
        for _ in 0..OBJECTIVE_LINES {
            boxes.push(ui.instantiate(menu, stages, "template_filled_checkbox")?);
        }
        let mut texts = Vec::new();
        for _ in 0..OBJECTIVE_LINES {
            texts.push(ui.instantiate(menu, stages, "template_justify_left_text")?);
        }
        // The game's list order: the last made first.
        added_tiles.reverse();
        boxes.reverse();
        texts.reverse();
        let mut name = |n: &str| ui.names.lookup_or_add(n).unwrap_or(0);
        let wait_complete = name("_WaitComplete");
        let hold_time = name("_HoldTime");
        let is_reminder = name("_IsReminder");
        Some(QuestText {
            reminder,
            stages,
            added,
            added_tiles,
            texts,
            boxes,
            names: Vec::new(),
            objectives: VecDeque::new(),
            last_queued: None,
            names_flag: false,
            objectives_flag: false,
            wait_complete,
            hold_time,
            is_reminder,
        })
    }

    /// Queues a quest's name or custom text. A notice with priority 0
    /// goes first (`0076baa0`, `AddHead`), the others at the end
    /// (`0076bad0`), the list then sorted by priority (`0076bb00`; kept
    /// in order where equal). Quests' updates stamp the queue's time
    /// (`0077a480`); custom text doesn't (`0076b960`).
    pub fn queue(&mut self, notice: Notice, now_ms: f64) {
        if notice.kind != Kind::Custom {
            self.last_queued = Some(now_ms);
        }
        if notice.priority == 0 {
            self.names.insert(0, notice);
        } else {
            self.names.push(notice);
        }
        self.names.sort_by_key(|n| n.priority);
    }

    /// Queues an objective line (`0077a5b0`, which stamps the time).
    pub fn queue_objective(&mut self, objective: Objective, now_ms: f64) {
        self.last_queued = Some(now_ms);
        self.objectives.push_front(objective);
    }

    fn any_visible(ui: &mut Ui, tiles: &[TileId]) -> bool {
        tiles.iter().any(|&t| ui.number(t, t::VISIBLE) != 0.0)
    }

    /// Whether quest names are up (`007735d0`): the flag, while any of
    /// `QuestAdded`'s texts shows (the flag drops when none does).
    fn names_showing(&mut self, ui: &mut Ui) -> bool {
        if self.names_flag {
            if Self::any_visible(ui, &self.added_tiles) {
                return true;
            }
            self.names_flag = false;
        }
        false
    }

    /// Whether objective lines are up (`00773650`).
    fn objectives_showing(&mut self, ui: &mut Ui) -> bool {
        if self.objectives_flag {
            let tiles: Vec<TileId> = self.texts.iter().chain(&self.boxes).copied().collect();
            if Self::any_visible(ui, &tiles) {
                return true;
            }
            self.objectives_flag = false;
        }
        false
    }

    /// One frame of `0077a650` (the HUD calls it each frame): the next
    /// quest name when none is up, then the letters' fades, or the
    /// objective lines. `now` in seconds; `opacity` is `fHudOpacity`.
    /// The sounds to play (editor IDs).
    pub fn update(
        &mut self,
        ui: &mut Ui,
        anims: &mut Animations,
        now: f64,
        opacity: f32,
        gate: Gate,
        timing: &Timing,
    ) -> Vec<String> {
        let mut sounds = Vec::new();
        let ms = now * 1000.0;
        if self
            .last_queued
            .is_some_and(|last| ms - last < QUEUE_WAIT_MS)
        {
            return sounds;
        }
        if !self.names_showing(ui) && !self.names.is_empty() {
            // Objectives up, their first line still seen: the name waits.
            let blocked = self.objectives_showing(ui) && ui.number(self.texts[0], t::ALPHA) != 0.0;
            if !blocked && gate.names {
                let notice = self.names.remove(0);
                sounds.push(self.show_name(ui, &notice, now, timing));
                self.animate_names(ui, anims, now, opacity, timing);
            }
        }
        if !self.names_showing(ui) {
            if gate.objectives
                && (self.objectives_showing(ui) || !self.objectives.is_empty())
                && !self.names_showing(ui)
            {
                self.update_objectives(ui, anims, now, opacity, timing, &mut sounds);
            }
        } else {
            self.animate_names(ui, anims, now, opacity, timing);
        }
        sounds
    }

    fn measure(ui: &mut Ui, text: &str, font: i32, wrap: f32) -> crate::text::Measured {
        let none = crate::text::Measured {
            width: 0.0,
            height: 0.0,
            lines: 0,
        };
        if !(1..=8).contains(&font) {
            return none;
        }
        match ui.fonts.get(font as usize - 1).cloned().flatten() {
            Some(f) => crate::text::measure(&f, font as usize, text.as_bytes(), wrap, 0),
            None => none,
        }
    }

    /// Puts a quest name up (`0077a650`): the title, then the subtitle in
    /// capitals, a letter to a tile, each due a delay after the last.
    /// Returns its sound.
    fn show_name(&mut self, ui: &mut Ui, notice: &Notice, now: f64, timing: &Timing) -> String {
        // Translated from 0077a650 (decompiled, FalloutNV.exe 1.4.0.525)
        self.names_flag = true;
        let sound = match notice.kind {
            Kind::Added => "UIPopUpQuestNew".to_string(),
            Kind::Custom => notice.sound.clone(),
            _ => "UIPopUpQuestComplete".to_string(),
        };
        let start = (now * 1000.0) as u32;
        let setting = |ui: &Ui, n: &str, d: &str| ui.setting_text(n).unwrap_or_else(|| d.into());
        let title_text = match notice.kind {
            Kind::Added => setting(ui, "sQuestAddedText", "Quest added"),
            Kind::Failed => setting(ui, "sQuestFailed", "Quest FAILED"),
            Kind::Completed => setting(ui, "sQuestCompletedText", "Quest completed"),
            Kind::Custom => notice.title.clone(),
        };
        let custom = notice.kind == Kind::Custom;
        let title = self.added_tiles[0];
        ui.set_string(title, t::STRING, &title_text);
        ui.set_number(title, t::VISIBLE, 1.0);
        ui.set_number(title, t::ALPHA, 0.0);
        let title_font = if custom {
            notice.title_font.unwrap_or(7)
        } else {
            7
        };
        ui.set_number(title, t::FONT, title_font as f32);
        let wrap = ui.number(title, t::WRAPWIDTH);
        let font = ui.number(title, t::FONT) as i32;
        let m = Self::measure(ui, &title_text, font, wrap);
        let title_width = m.width as i32;
        ui.set_number(title, t::Y, -(m.height + 15.0));
        // The space between the safe zone's edges.
        let screen = ui.screen_size;
        let room = screen.width() as i32 - screen.safe_x as i32 * 2;
        let mut factor = 0.0f32;
        if custom {
            let justify = match notice.justification {
                1 => {
                    factor = 0.5;
                    crate::text::CENTER
                }
                2 => {
                    factor = 1.0;
                    crate::text::RIGHT
                }
                _ => crate::text::LEFT,
            };
            ui.set_number(title, t::X, (room - title_width / 2) as f32 * factor);
            ui.set_number(title, t::JUSTIFY, justify as f32);
        }
        let remain = f64::from(timing.letter_remain) * 1000.0;
        ui.set_number(title, t::USER0, start as f32);
        ui.set_number(title, t::USER0 + 1, (start as f64 + remain) as f32);
        ui.set_number(title, t::BRIGHTNESS, BRIGHT);
        if notice.kind == Kind::Completed {
            self.hide_objectives(ui);
        }
        // The subtitle in capitals; a second line from the last space up
        // to its 33rd character when it's 32 long or more.
        let subtitle: Vec<u8> = notice
            .subtitle
            .bytes()
            .take(0x103)
            .map(|c| c.to_ascii_uppercase())
            .collect();
        let mut brk = 32usize;
        if subtitle.len() >= 32 {
            for i in (0..=32).rev() {
                if subtitle.get(i) == Some(&b' ') {
                    brk = i;
                    break;
                }
            }
        }
        let sub_font = if custom {
            notice.subtitle_font.unwrap_or(8)
        } else {
            8
        };
        let whole = Self::measure(ui, &String::from_utf8_lossy(&subtitle), sub_font, f32::MAX);
        let line_start = ((room as f32 - (factor + 1.0) * whole.width) * factor) as i32;
        let mut x = line_start;
        let mut due = start;
        let delay = (f64::from(timing.letter_delay) * 1000.0) as u32;
        let mut letters = self.added_tiles.iter().skip(1).copied();
        for (i, &c) in subtitle.iter().enumerate() {
            let Some(tile) = letters.next() else {
                continue;
            };
            if brk + 1 == i {
                x = line_start;
            }
            let letter = String::from_utf8_lossy(&[c]).into_owned();
            ui.set_number(tile, t::FONT, sub_font as f32);
            ui.set_string(tile, t::STRING, &letter);
            ui.set_number(tile, t::VISIBLE, 1.0);
            ui.set_number(tile, t::ALPHA, 0.0);
            ui.set_number(tile, t::X, x as f32);
            let wrap = ui.number(tile, t::WRAPWIDTH);
            let font = ui.number(tile, t::FONT) as i32;
            let m = Self::measure(ui, &letter, font, wrap);
            ui.set_number(tile, t::Y, if i > brk { m.height + 5.0 } else { 0.0 });
            due = due.wrapping_add(delay);
            ui.set_number(tile, t::USER0, due as f32);
            ui.set_number(tile, t::USER0 + 1, (due as f64 + remain) as f32);
            ui.set_number(tile, t::BRIGHTNESS, BRIGHT);
            x += m.width as i32;
        }
        sound
    }

    /// `0077f5e0`, when a quest is completed: every objective line hidden
    /// and the waiting ones dropped.
    fn hide_objectives(&mut self, ui: &mut Ui) {
        for &tile in self.texts.iter().chain(&self.boxes) {
            ui.set_number(tile, t::VISIBLE, 0.0);
            ui.set_number(tile, t::ALPHA, 0.0);
            ui.set_number(tile, self.wait_complete, 0.0);
            ui.set_number(tile, self.hold_time, 0.0);
        }
        self.objectives.clear();
    }

    /// The letters' fades (`0077c170`): objective lines kept out of sight;
    /// a letter due (its `user0`) fades in to the HUD's opacity over
    /// `fQuestCinematicCharacterFadeIn`, one past its time (`user1`) fades
    /// out from at least 175 over `…FadeOut`; one faded out is hidden.
    fn animate_names(
        &mut self,
        ui: &mut Ui,
        anims: &mut Animations,
        now: f64,
        opacity: f32,
        timing: &Timing,
    ) {
        // Translated from 0077c170 (decompiled, FalloutNV.exe 1.4.0.525)
        if ui.number(self.added, t::CHILDCOUNT) <= 0.0 {
            return;
        }
        for &tile in self.texts.iter().chain(&self.boxes) {
            ui.set_number(tile, t::ALPHA, 0.0);
            ui.set_number(tile, self.wait_complete, 0.0);
        }
        let ms = (now * 1000.0) as u32 as f32;
        for &tile in &self.added_tiles {
            let due = ui.number(tile, t::USER0);
            if due > 0.0 && due < ms {
                anims.start(
                    tile,
                    t::ALPHA,
                    0.0,
                    opacity * 255.0,
                    timing.letter_fade_in,
                    now,
                );
                ui.set_number(tile, t::USER0, 0.0);
                continue;
            }
            let end = ui.number(tile, t::USER0 + 1);
            if end > 0.0 && end < ms {
                let alpha = ui.number(tile, t::ALPHA);
                let from = if alpha <= 175.0 { 175.0 } else { alpha };
                anims.start(tile, t::ALPHA, from, 0.0, timing.letter_fade_out, now);
                ui.set_number(tile, t::USER0 + 1, 0.0);
                continue;
            }
            if anims.target(ui, tile, t::ALPHA) == 0.0
                && ui.number(tile, t::ALPHA) == 0.0
                && ui.number(tile, t::USER0 + 1) == 0.0
            {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
    }

    /// The objective lines (`0077b430`).
    fn update_objectives(
        &mut self,
        ui: &mut Ui,
        anims: &mut Animations,
        now: f64,
        opacity: f32,
        timing: &Timing,
        sounds: &mut Vec<String>,
    ) {
        // Translated from 0077b430 (decompiled, FalloutNV.exe 1.4.0.525)
        let hud_alpha = opacity * 255.0;
        let menu_width = ui.screen_size.width() as i32;
        let mut placed = false;
        if !self.objectives_showing(ui) {
            // New lines fill the slots from the last up; a reminder and a
            // real update don't share the lines.
            let mut slot = OBJECTIVE_LINES;
            let mut k = 0;
            while k < self.objectives.len() && slot > 0 {
                let showing = self.objectives_showing(ui);
                let first_reminder = showing && ui.number(self.texts[0], self.is_reminder) != 0.0;
                let e = self.objectives[k].clone();
                if showing && first_reminder != e.reminder {
                    k += 1;
                    continue;
                }
                self.objectives_flag = true;
                placed = true;
                slot -= 1;
                let (text, check) = (self.texts[slot], self.boxes[slot]);
                if e.completed {
                    let done = ui
                        .setting_text("sHUDQuestCompleted")
                        .unwrap_or_else(|| "COMPLETED: ".into());
                    ui.set_string(text, t::STRING, &format!("{done}{}", e.text));
                    ui.set_number(text, self.is_reminder, f32::from(u8::from(e.reminder)));
                    ui.set_number(text, t::USER0 + 3, 1.0);
                    ui.set_string(check, t::FILENAME, BOX_FILLED);
                } else {
                    ui.set_string(text, t::STRING, &e.text);
                    ui.set_number(text, t::USER0 + 3, 0.0);
                    ui.set_number(text, self.is_reminder, f32::from(u8::from(e.reminder)));
                    ui.set_string(check, t::FILENAME, BOX_EMPTY);
                }
                let x = ui.number(text, t::X);
                ui.set_number(check, t::X, x - 20.0);
                ui.set_number(check, t::VISIBLE, 1.0);
                ui.set_number(text, t::FONT, 7.0);
                ui.set_number(text, t::VISIBLE, 1.0);
                ui.set_number(text, t::ALPHA, 0.0);
                ui.set_number(text, t::WRAPWIDTH, (menu_width / 3) as f32);
                ui.set_number(text, t::BRIGHTNESS, BRIGHT);
                sounds.push("UIQuestUpdate".to_string());
                self.objectives.remove(k);
            }
        }
        if placed {
            // The lines shown, one under another (the box 3 lower).
            let mut y = 0i32;
            for j in 0..OBJECTIVE_LINES {
                let (text, check) = (self.texts[j], self.boxes[j]);
                if ui.number(text, t::VISIBLE) == 0.0 {
                    continue;
                }
                ui.set_number(text, t::Y, y as f32);
                ui.set_number(check, t::Y, (y + 3) as f32);
                let wrap = ui.number(text, t::WRAPWIDTH);
                let font = ui.number(text, t::FONT) as i32;
                let s = ui.string(text, t::STRING).unwrap_or_default();
                y += Self::measure(ui, &s, font, wrap).height as i32;
            }
        }
        // The first four lines shown.
        let mut lines: Vec<(TileId, TileId)> = Vec::new();
        for j in 0..OBJECTIVE_LINES {
            if lines.len() >= 4 {
                break;
            }
            if ui.number(self.texts[j], t::VISIBLE) != 0.0 {
                lines.push((self.texts[j], self.boxes[j]));
            }
        }
        let Some(&(top, top_box)) = lines.first() else {
            return;
        };
        let (wait, hold) = (self.wait_complete, self.hold_time);
        let w = ui.number(top, wait);
        if w == 0.0 {
            // In: the top line fades in; its wait runs three delays, the
            // second and third lines' one and two.
            ui.set_number(top, wait, 0.0);
            anims.start(top, t::ALPHA, 0.0, hud_alpha, 0.5, now);
            anims.start(top, wait, 0.0, 1.0, timing.objective_delay * 3.0, now);
            anims.start(top_box, t::ALPHA, 0.0, hud_alpha, 0.5, now);
            if let Some(&(second, _)) = lines.get(1) {
                anims.start(second, wait, 0.0, 1.0, timing.objective_delay, now);
                if let Some(&(third, _)) = lines.get(2) {
                    anims.start(third, wait, 0.0, 1.0, timing.objective_delay * 2.0, now);
                }
            }
        } else if w == 1.0 {
            // Held, then faded out.
            let h = ui.number(top, hold);
            if h == 0.0 {
                ui.set_number(top, hold, 0.0);
                anims.start(top, hold, 0.0, 1.0, timing.objective_pause, now);
            } else if h == 1.0 {
                anims.start(
                    top,
                    t::ALPHA,
                    hud_alpha,
                    0.0,
                    timing.objective_fade_out,
                    now,
                );
                ui.set_number(top, wait, 2.0);
                anims.start(
                    top_box,
                    t::ALPHA,
                    hud_alpha,
                    0.0,
                    timing.objective_fade_out,
                    now,
                );
            }
        } else if anims.done(top, t::ALPHA, now) && ui.number(top, t::ALPHA) == 0.0 {
            // Gone: the rest scroll up by the gap to the next line (every
            // line but the list's first tile), the fourth may come in.
            if let Some(&(second, _)) = lines.get(1) {
                let dy = (ui.number(second, t::Y) - ui.number(top, t::Y)) as i32;
                let tiles: Vec<TileId> = self.texts.iter().chain(&self.boxes).copied().collect();
                for &tile in tiles.iter().skip(1) {
                    let y = ui.number(tile, t::Y);
                    anims.start(tile, t::Y, y, y - dy as f32, timing.objective_scroll, now);
                }
            }
            if let Some(&(fourth, _)) = lines.get(3) {
                ui.set_number(fourth, wait, 1.0);
            }
            ui.set_number(top, t::VISIBLE, 0.0);
            ui.set_number(top_box, t::VISIBLE, 0.0);
            ui.set_number(top, wait, 0.0);
        }
        // The second and third lines fade in once their wait has run.
        for &(text, check) in lines.iter().skip(1).take(2) {
            if ui.number(text, wait) == 1.0 && ui.number(text, t::ALPHA) == 0.0 {
                anims.start(
                    text,
                    t::ALPHA,
                    0.0,
                    hud_alpha,
                    timing.objective_fade_in,
                    now,
                );
                anims.start(
                    check,
                    t::ALPHA,
                    0.0,
                    hud_alpha,
                    timing.objective_fade_in,
                    now,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The HUD's quest reminder, cut down to what the code reads.
    const MENU: &str = r#"<menu name="HUDMainMenu"><locus>&true;</locus>
      <template name="template_justify_left_text"><text name="justify_left_text"><justify>&left;</justify><font> 7 </font><wrapwidth>2048</wrapwidth><alpha>0</alpha><visible>&false;</visible></text></template>
      <rect name="QuestReminder"><width>400</width><height>200</height><locus>&true;</locus>
        <rect name="QuestStages"><width>100</width><height>50</height><y>40</y><locus>&true;</locus></rect>
        <template name="template_filled_checkbox"><image name="filled_checkbox"><visible>&false;</visible><alpha>0</alpha><width>26</width><height>26</height><x>-20</x><y>3</y></image></template>
        <rect name="QuestAdded"><width>100</width><height>150</height><y>80</y><locus>&true;</locus></rect>
      </rect></menu>"#;

    fn quest_text() -> (Ui, QuestText) {
        // Every glyph 10 wide and 20 high.
        let mut ui = crate::menus::test_support::ui();
        let menu = ui.load_menu(MENU.as_bytes(), &mut |_| None).unwrap();
        let q = QuestText::create(&mut ui, menu).unwrap();
        (ui, q)
    }

    const GATE: Gate = Gate {
        names: true,
        objectives: true,
    };

    #[test]
    fn placed_with_its_tiles() {
        let (mut ui, q) = quest_text();
        // 2 x 15 + 15, 2 x 15 + 170.
        assert_eq!(ui.number(q.reminder, t::X), 45.0);
        assert_eq!(ui.number(q.reminder, t::Y), 200.0);
        assert_eq!(q.added_tiles.len(), 64);
        assert_eq!((q.texts.len(), q.boxes.len()), (32, 32));
    }

    #[test]
    fn a_quest_added_spells_its_name_a_letter_at_a_time() {
        let (mut ui, mut q) = quest_text();
        let mut anims = Animations::default();
        let timing = Timing::default();
        q.queue(Notice::quest(Kind::Added, "Ghost town"), 100.0 * 1000.0);
        // Nothing for 2 seconds after it's queued.
        assert!(q
            .update(&mut ui, &mut anims, 101.0, 1.0, GATE, &timing)
            .is_empty());
        let sounds = q.update(&mut ui, &mut anims, 102.5, 1.0, GATE, &timing);
        assert_eq!(sounds, ["UIPopUpQuestNew"]);
        let title = q.added_tiles[0];
        assert_eq!(ui.string(title, t::STRING).as_deref(), Some("Quest added"));
        // Above the letters by its height (20) and 15; due now, gone 4 s on.
        assert_eq!(ui.number(title, t::Y), -35.0);
        assert_eq!(ui.number(title, t::USER0), 102500.0);
        assert_eq!(ui.number(title, t::USER0 + 1), 106500.0);
        // In capitals, one letter to a tile, 10 apart, each 100 ms later.
        let letters: Vec<TileId> = q.added_tiles[1..11].to_vec();
        let text: String = letters
            .iter()
            .map(|&l| ui.string(l, t::STRING).unwrap_or_default())
            .collect();
        assert_eq!(text, "GHOST TOWN");
        assert_eq!(ui.number(letters[3], t::X), 30.0);
        assert_eq!(ui.number(letters[3], t::USER0), 102900.0);
        assert_eq!(ui.number(letters[3], t::FONT), 8.0);
        assert_eq!(ui.number(q.added_tiles[11], t::VISIBLE), 0.0);
        // A letter due fades in over 2 seconds; one past its time fades
        // out; faded out, it's hidden.
        q.update(&mut ui, &mut anims, 103.0, 1.0, GATE, &timing);
        assert!(anims.moving(letters[3], t::ALPHA));
        assert_eq!(ui.number(letters[3], t::USER0), 0.0);
        anims.step(&mut ui, 105.0);
        assert_eq!(ui.number(letters[3], t::ALPHA), 255.0);
        q.update(&mut ui, &mut anims, 107.0, 1.0, GATE, &timing);
        assert_eq!(anims.target(&mut ui, letters[3], t::ALPHA), 0.0);
        anims.step(&mut ui, 110.0);
        q.update(&mut ui, &mut anims, 110.0, 1.0, GATE, &timing);
        assert_eq!(ui.number(letters[3], t::VISIBLE), 0.0);
    }

    #[test]
    fn names_wait_for_the_one_up_and_long_names_take_two_lines() {
        let (mut ui, mut q) = quest_text();
        let mut anims = Animations::default();
        let timing = Timing::default();
        let long = "A quest with a rather long name indeed";
        q.queue(Notice::quest(Kind::Completed, long), 0.0);
        q.queue(Notice::quest(Kind::Failed, "Other"), 0.0);
        let s = q.update(&mut ui, &mut anims, 10.0, 1.0, GATE, &timing);
        assert_eq!(s, ["UIPopUpQuestComplete"]);
        assert_eq!(
            ui.string(q.added_tiles[0], t::STRING).as_deref(),
            Some("Quest completed")
        );
        // The last space at or before the 33rd character (index 31)
        // breaks the line: the next letters start again at 0, a line
        // lower (the letter's height and 5).
        let after = q.added_tiles[1 + 32];
        assert_eq!(ui.string(after, t::STRING).as_deref(), Some("I"));
        assert_eq!(ui.number(after, t::X), 0.0);
        assert_eq!(ui.number(after, t::Y), 25.0);
        assert_eq!(ui.number(q.added_tiles[1 + 31], t::Y), 0.0);
        // 25 letters 10 wide and 6 spaces 5 wide before it.
        assert_eq!(ui.number(q.added_tiles[1 + 31], t::X), 280.0);
        // The second waits while the first is up.
        assert!(q
            .update(&mut ui, &mut anims, 11.0, 1.0, GATE, &timing)
            .is_empty());
        // Held back while the game state doesn't let it show.
        for &l in &q.added_tiles.clone() {
            ui.set_number(l, t::VISIBLE, 0.0);
        }
        let shut = Gate {
            names: false,
            objectives: true,
        };
        assert!(q
            .update(&mut ui, &mut anims, 30.0, 1.0, shut, &timing)
            .is_empty());
        assert_eq!(
            q.update(&mut ui, &mut anims, 31.0, 1.0, GATE, &timing),
            ["UIPopUpQuestComplete"]
        );
        assert_eq!(
            ui.string(q.added_tiles[0], t::STRING).as_deref(),
            Some("Quest FAILED")
        );
    }

    #[test]
    fn a_place_discovered_and_custom_text_first() {
        let (mut ui, mut q) = quest_text();
        let mut anims = Animations::default();
        let timing = Timing::default();
        let custom = |title: &str, priority: i32| Notice {
            kind: Kind::Custom,
            title: title.to_string(),
            subtitle: "Goodsprings".to_string(),
            priority,
            justification: 0,
            title_font: None,
            subtitle_font: None,
            sound: "UIPopUpQuestNew".to_string(),
        };
        q.queue(custom("You have discovered", 1), 0.0);
        // Priority 0 goes first.
        q.queue(custom("First", 0), 0.0);
        assert_eq!(q.names[0].title, "First");
        q.names.remove(0);
        // Custom text doesn't hold the queue back.
        let s = q.update(&mut ui, &mut anims, 0.5, 1.0, GATE, &timing);
        assert_eq!(s, ["UIPopUpQuestNew"]);
        let title = q.added_tiles[0];
        assert_eq!(
            ui.string(title, t::STRING).as_deref(),
            Some("You have discovered")
        );
        // Left: x 0, justified left.
        assert_eq!(ui.number(title, t::X), 0.0);
        assert_eq!(ui.number(title, t::JUSTIFY), 1.0);
        assert_eq!(ui.string(q.added_tiles[1], t::STRING).as_deref(), Some("G"));
    }

    #[test]
    fn objective_lines_come_in_hold_and_scroll_up() {
        let (mut ui, mut q) = quest_text();
        let mut anims = Animations::default();
        let timing = Timing::default();
        let line = |text: &str, completed: bool| Objective {
            text: text.to_string(),
            completed,
            reminder: false,
        };
        q.queue_objective(line("Find the man", false), 0.0);
        q.queue_objective(line("Talk to Sunny", true), 0.0);
        let s = q.update(&mut ui, &mut anims, 10.0, 1.0, GATE, &timing);
        assert_eq!(s, ["UIQuestUpdate", "UIQuestUpdate"]);
        // The newest takes the last line; the older shows above it.
        let (a, b) = (q.texts[30], q.texts[31]);
        assert_eq!(ui.string(a, t::STRING).as_deref(), Some("Find the man"));
        assert_eq!(
            ui.string(b, t::STRING).as_deref(),
            Some("COMPLETED: Talk to Sunny")
        );
        assert_eq!(
            ui.string(q.boxes[31], t::FILENAME).as_deref(),
            Some(BOX_FILLED)
        );
        assert_eq!(
            ui.string(q.boxes[30], t::FILENAME).as_deref(),
            Some(BOX_EMPTY)
        );
        assert_eq!((ui.number(a, t::Y), ui.number(b, t::Y)), (0.0, 20.0));
        assert_eq!(ui.number(q.boxes[31], t::Y), 23.0);
        // The top line fades in; the second after one delay.
        assert!(anims.moving(a, t::ALPHA));
        assert!(!anims.moving(b, t::ALPHA));
        anims.step(&mut ui, 11.0);
        q.update(&mut ui, &mut anims, 11.0, 1.0, GATE, &timing);
        assert!(anims.moving(b, t::ALPHA));
        // The top line's wait (3 s), hold (3 s), fade out (0.5 s); then
        // the rest move up by its height.
        anims.step(&mut ui, 13.0);
        q.update(&mut ui, &mut anims, 13.0, 1.0, GATE, &timing);
        anims.step(&mut ui, 16.0);
        q.update(&mut ui, &mut anims, 16.0, 1.0, GATE, &timing);
        assert_eq!(anims.target(&mut ui, a, t::ALPHA), 0.0);
        anims.step(&mut ui, 17.0);
        q.update(&mut ui, &mut anims, 17.0, 1.0, GATE, &timing);
        assert_eq!(ui.number(a, t::VISIBLE), 0.0);
        anims.step(&mut ui, 18.0);
        assert_eq!(ui.number(b, t::Y), 0.0);
    }

    #[test]
    fn a_quest_completed_clears_the_objectives() {
        let (mut ui, mut q) = quest_text();
        let mut anims = Animations::default();
        let timing = Timing::default();
        q.queue_objective(
            Objective {
                text: "Done".into(),
                completed: false,
                reminder: false,
            },
            0.0,
        );
        q.queue(Notice::quest(Kind::Completed, "Q"), 0.0);
        q.update(&mut ui, &mut anims, 5.0, 1.0, GATE, &timing);
        assert!(q.objectives.is_empty());
        assert_eq!(ui.number(q.texts[31], t::VISIBLE), 0.0);
    }
}
