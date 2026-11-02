//! Tag skills (`menus\chargen\char_gen_menu.xml`, class `CharGenMenu`
//! 1048, vtable `01071bb4` in FalloutNV.exe), as a script's
//! `SetTagSkills count [preselect]` opens it (`005d7350` → `00753420(1,
//! count, 0, preselect == 1)`). The same class has an attributes mode (0:
//! `SetSPECIALPoints`, `AddSPECIALPoints`, `ShowChargenMenu`), which New
//! Vegas's opening doesn't use; only the skills mode is here. Read from the
//! code (`00753420` and the functions after it):
//!
//! * opening (`00753420`): fails without tile ids 0–6 (`00753770`);
//!   `_OnlyAdd` on the menu; Reset (5) `sReset`, Done (6) `sDone`; the
//!   list's `y` 65; the skills (`00753b10`): actor values 32 + i for i < 14
//!   but those the exe's actor value table flags 0x2000 (Big Guns), each a
//!   `CGM_SelectItemTemplate` line with id 9 and `_BaseValue` the player's
//!   permanent value; one of the player's tag skills shows it less
//!   `fAVDTagSkillBonus`, and with "preselect" is picked (`_selected`,
//!   `_AddedValue` the bonus, a point used unless "only add"); sorted by
//!   name; `_CurrPoints`, `_MaxPoints`, the counter; the first line's
//!   picture and description shown.
//! * a line's number (`007545a0`): `_ValueString` = base + extra + added,
//!   kept within 0..100.
//! * the counter (`00753d20`): the title (id 0) `sSkillsTitle` ("SKILLS:  %d/%d
//!   Selected") with the points used and the most; what's left: none hides
//!   the counter (id 4), else `sSkillsCount` ("Tag %d Skill") with it, and
//!   "s" after it when it isn't 1.
//! * clicks (`00753ea0`): a line (9) picked again is let go (not one of the
//!   player's tags with "only add"); another is picked while points are
//!   left; `_AddedValue` the bonus or 0, sound 3, the count; Reset (5) lets
//!   them all go (keeping the player's tags with "only add"), sound 1 when
//!   that changed the count; Done (6): the player's tag slots cleared and the
//!   picked skills put in them in the list's order (`007537b0`, `00754530`).
//! * the pointer on a line (`00753870`): its picture and description when
//!   it isn't the line already shown, sound 4.
//!
//! Not here: the controller's buttons (`00754150` special 9/10, `007542d0`),
//! the first-time help message (`00718630(0xe, …)`).

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\chargen\\char_gen_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1048;
/// A skill line's id.
pub const SKILL_ID: i32 = 9;
/// The list's `y` in the skills mode (`0102243c`).
pub const SKILLS_LIST_Y: f32 = 65.0;

/// A skill the menu lists.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Skill {
    pub av: u16,
    pub name: String,
    /// The player's permanent value, whole.
    pub base: i32,
    /// One of the player's tag skills.
    pub tagged: bool,
    pub icon: String,
    pub description: String,
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Done: the tag skills picked, in the list's order.
    Close { tags: Vec<u16> },
}

/// The tag skills menu.
#[derive(Debug)]
pub struct CharGenMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..6 (`+0x2c`).
    pub tiles: [Option<TileId>; 7],
    /// The list (`+0x58`).
    pub list: ListBox,
    pub skills: Vec<Skill>,
    /// Points used (`+0x48`) and the most (`+0x4c`).
    pub points: i32,
    pub most: i32,
    /// "Only add" (`+0x54`): the player's tags can't be let go.
    pub only_add: bool,
    /// `fAVDTagSkillBonus`.
    pub bonus: f32,
    /// The listindex whose information is shown (`+0x50`).
    shown: Option<i32>,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

fn custom(ui: &mut Ui, name: &str) -> i32 {
    ui.names.lookup_or_add(name).unwrap_or(0)
}

impl CharGenMenu {
    pub fn new(menu: TileId) -> CharGenMenu {
        CharGenMenu {
            menu,
            tiles: [None; 7],
            list: ListBox::default(),
            skills: Vec::new(),
            points: 0,
            most: 0,
            only_add: false,
            bonus: 0.0,
            shown: None,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    /// Opens it in the skills mode (`00753420(1, …)`): `most` skills to
    /// tag, the player's tags picked to start with when `preselect`.
    pub fn open(
        &mut self,
        ui: &mut Ui,
        most: i32,
        only_add: bool,
        preselect: bool,
        bonus: f32,
        skills: Vec<Skill>,
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.most = most;
        self.points = 0;
        self.only_add = only_add;
        self.bonus = bonus;
        let only = custom(ui, "_OnlyAdd");
        ui.set_number(self.menu, only, if only_add { 1.0 } else { 0.0 });
        let text = |ui: &Ui, n: &str| ui.setting_text(n).unwrap_or_default();
        for (id, name) in [(5, "sReset"), (6, "sDone")] {
            if let Some(tile) = self.tile(id) {
                let s = text(ui, name);
                ui.set_text(tile, t::STRING, &s);
            }
        }
        self.skills = skills;
        self.fill(ui, preselect);
        let (curr, max) = (custom(ui, "_CurrPoints"), custom(ui, "_MaxPoints"));
        ui.set_number(self.menu, curr, self.points as f32);
        ui.set_number(self.menu, max, most as f32);
        self.counter(ui);
        if let Some(first) = self.list.item_at(ui, 0) {
            self.show_info(ui, first);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    /// The lines (`00753b10`).
    fn fill(&mut self, ui: &mut Ui, preselect: bool) {
        if let Some(list) = self.tile(1) {
            self.list = ListBox::new(ui, list, "CGM_SelectItemTemplate");
            ui.set_number(list, t::Y, SKILLS_LIST_Y);
        }
        let (base, added, sel) = (
            custom(ui, "_BaseValue"),
            custom(ui, "_AddedValue"),
            custom(ui, "_selected"),
        );
        for i in 0..self.skills.len() {
            let s = self.skills[i].clone();
            let Some(tile) = self.list.add(ui, self.menu, i as i32, Some(&s.name)) else {
                continue;
            };
            ui.set_number(tile, t::ID, SKILL_ID as f32);
            ui.set_number(tile, base, s.base as f32);
            if s.tagged {
                ui.set_number(tile, base, s.base as f32 - self.bonus);
                if preselect {
                    ui.set_number(tile, sel, 1.0);
                    ui.set_number(tile, added, self.bonus);
                    if !self.only_add {
                        self.points += 1;
                    }
                }
            }
            Self::display(ui, tile);
        }
        let names: Vec<String> = self
            .skills
            .iter()
            .map(|s| s.name.to_ascii_lowercase())
            .collect();
        let values: Vec<(TileId, i32)> =
            self.list.items.iter().map(|i| (i.tile, i.value)).collect();
        let name_of = |tile: TileId| {
            values
                .iter()
                .find(|(t_, _)| *t_ == tile)
                .and_then(|(_, v)| names.get(*v as usize))
                .cloned()
                .unwrap_or_default()
        };
        self.list.sort(ui, &|_, a, b| name_of(a) < name_of(b));
    }

    /// `_ValueString` (`007545a0`).
    fn display(ui: &mut Ui, tile: TileId) {
        let number = |ui: &mut Ui, name: &str| {
            let id = custom(ui, name);
            let v = ui.number(tile, id);
            if v == crate::names::NOT_FOUND {
                0.0
            } else {
                v
            }
        };
        let v = (number(ui, "_BaseValue") + number(ui, "_ExtraValue") + number(ui, "_AddedValue"))
            as i32;
        let s = custom(ui, "_ValueString");
        ui.set_string(tile, s, &v.clamp(0, 100).to_string());
    }

    /// The title and counter (`00753d20`).
    fn counter(&mut self, ui: &mut Ui) {
        let text = |ui: &Ui, n: &str| ui.setting_text(n).unwrap_or_default();
        if let Some(title) = self.tile(0) {
            let s = text(ui, "sSkillsTitle")
                .replacen("%d", &self.points.to_string(), 1)
                .replacen("%d", &self.most.to_string(), 1);
            ui.set_text(title, t::STRING, &s);
        }
        let Some(counter) = self.tile(4) else {
            return;
        };
        let left = self.most - self.points;
        if left == 0 {
            ui.set_number(counter, t::VISIBLE, 0.0);
        } else {
            let mut s = text(ui, "sSkillsCount").replacen("%d", &left.to_string(), 1);
            if left != 1 {
                s.push('s');
            }
            ui.set_text(counter, t::STRING, &s);
            ui.set_number(counter, t::VISIBLE, 1.0);
        }
        ui.refresh();
    }

    fn selected(ui: &mut Ui, tile: TileId) -> bool {
        let sel = custom(ui, "_selected");
        let v = ui.number(tile, sel);
        v != 0.0 && v != crate::names::NOT_FOUND
    }

    fn skill_of(&self, tile: TileId) -> Option<&Skill> {
        self.list
            .value_of(tile)
            .and_then(|v| self.skills.get(v as usize))
    }

    /// A line's picture and description (`00753870`).
    fn show_info(&mut self, ui: &mut Ui, line: TileId) {
        let Some(s) = self.skill_of(line).cloned() else {
            return;
        };
        if let Some(icon) = self.tile(2) {
            ui.set_string(icon, t::FILENAME, &s.icon);
            ui.set_number(icon, t::VISIBLE, 1.0);
        }
        if let Some(text) = self.tile(3) {
            ui.set_text(text, t::STRING, &s.description);
            ui.set_number(text, t::VISIBLE, 1.0);
        }
    }

    fn set_points(&mut self, ui: &mut Ui) {
        let curr = custom(ui, "_CurrPoints");
        ui.set_number(self.menu, curr, self.points as f32);
    }

    /// Done (`007537b0`): the picked skills by listindex.
    fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        let tags = self
            .list
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .filter(|l| Self::selected(ui, l.tile))
            .filter_map(|l| self.skills.get(l.value as usize).map(|s| s.av))
            .collect();
        self.requests.push(Request::Close { tags });
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for CharGenMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..7).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `00753ea0`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        let (sel, added) = (custom(ui, "_selected"), custom(ui, "_AddedValue"));
        match id {
            5 => {
                let before = self.points;
                self.points = 0;
                for line in self.list.items.clone() {
                    if Self::selected(ui, line.tile) {
                        let keep =
                            self.only_add && self.skill_of(line.tile).is_some_and(|s| s.tagged);
                        if !keep {
                            ui.set_number(line.tile, sel, 0.0);
                            ui.set_number(line.tile, added, 0.0);
                        }
                    }
                    Self::display(ui, line.tile);
                }
                self.set_points(ui);
                self.counter(ui);
                if self.points != before {
                    if let Some(s) = menu::menu_sound(1) {
                        self.sounds.push(s.to_string());
                    }
                }
            }
            6 => self.close(ui),
            SKILL_ID => {
                let Some(line) = tile else {
                    return;
                };
                let was = Self::selected(ui, line);
                if !was && self.points >= self.most {
                    return;
                }
                if was && self.only_add && self.skill_of(line).is_some_and(|s| s.tagged) {
                    return;
                }
                ui.set_number(line, sel, if was { 0.0 } else { 1.0 });
                ui.set_number(line, added, if was { 0.0 } else { self.bonus });
                if let Some(s) = menu::menu_sound(3) {
                    self.sounds.push(s.to_string());
                }
                self.points += if was { -1 } else { 1 };
                self.set_points(ui);
                Self::display(ui, line);
                self.counter(ui);
            }
            _ => {}
        }
        ui.refresh();
    }

    /// `00753870`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id != SKILL_ID {
            return;
        }
        let index = ui.number(tile, t::LISTINDEX) as i32;
        if self.shown == Some(index) {
            return;
        }
        self.show_info(ui, tile);
        if let Some(s) = menu::menu_sound(4) {
            self.sounds.push(s.to_string());
        }
        self.shown = Some(index);
        ui.refresh();
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.list]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn skill(av: u16, name: &str, base: i32, tagged: bool) -> Skill {
        Skill {
            av,
            name: name.into(),
            base,
            tagged,
            icon: format!("{name}.dds"),
            description: format!("About {name}."),
        }
    }

    fn opened(only_add: bool, preselect: bool) -> (Ui, CharGenMenu) {
        let mut ui = test_support::ui();
        let mut m = CharGenMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::char_gen_menu(), &mut m);
        let skills = vec![
            skill(43, "Speech", 30, true),
            skill(32, "Barter", 15, false),
            skill(41, "Guns", 30, true),
            skill(40, "Science", 15, false),
        ];
        assert!(m.open(&mut ui, 3, only_add, preselect, 15.0, skills));
        (ui, m)
    }

    fn line(ui: &mut Ui, m: &CharGenMenu, text: &str) -> TileId {
        m.list
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .find(|l| ui.string(l.tile, t::STRING).as_deref() == Some(text))
            .unwrap()
            .tile
    }

    /// `00753420`, `00753b10`, `00753d20`, `00753ea0`, `007537b0`: the
    /// player's tags picked to start with (shown less the bonus, which
    /// picking adds), the title and counter, at most three, Reset, Done in
    /// the list's order.
    #[test]
    fn tagging_skills() {
        let (mut ui, mut m) = opened(false, true);
        let value = ui.names.lookup("_ValueString").unwrap();
        let names: Vec<String> = m
            .list
            .shown_items(&mut ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect();
        assert_eq!(names, ["Barter", "Guns", "Science", "Speech"]);
        let (title, counter) = (m.tiles[0].unwrap(), m.tiles[4].unwrap());
        assert_eq!(
            ui.string(title, t::STRING).as_deref(),
            Some("SKILLS:  2/3 Selected")
        );
        assert_eq!(
            ui.string(counter, t::STRING).as_deref(),
            Some("Tag 1 Skill")
        );
        // The first line's information shows from the start.
        assert_eq!(
            ui.string(m.tiles[3].unwrap(), t::STRING).as_deref(),
            Some("About Barter.")
        );
        let guns = line(&mut ui, &m, "Guns");
        assert_eq!(ui.string(guns, value).as_deref(), Some("30"));
        let science = line(&mut ui, &m, "Science");
        m.click(&mut ui, SKILL_ID, Some(science), 0.0);
        assert_eq!(ui.string(science, value).as_deref(), Some("30"));
        assert_eq!(ui.number(counter, t::VISIBLE), 0.0);
        assert_eq!(ui.number(m.tiles[6].unwrap(), t::TARGET), 1.0);
        // A fourth is one too many; letting Guns go shows it untagged.
        let barter = line(&mut ui, &m, "Barter");
        m.click(&mut ui, SKILL_ID, Some(barter), 0.0);
        assert_eq!(m.points, 3);
        m.click(&mut ui, SKILL_ID, Some(guns), 0.0);
        assert_eq!(ui.string(guns, value).as_deref(), Some("15"));
        assert_eq!(
            ui.string(counter, t::STRING).as_deref(),
            Some("Tag 1 Skill")
        );
        m.click(&mut ui, SKILL_ID, Some(barter), 0.0);
        m.click(&mut ui, 6, None, 0.0);
        assert_eq!(
            m.requests,
            [Request::Close {
                tags: vec![32, 40, 43]
            }]
        );
        // Reset lets everything go, with sound 1.
        let (mut ui, mut m) = opened(false, true);
        m.click(&mut ui, 5, None, 0.0);
        assert_eq!(m.points, 0);
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuOK"));
        assert_eq!(
            ui.string(m.tiles[4].unwrap(), t::STRING).as_deref(),
            Some("Tag 3 Skills")
        );
        // "Only add": the player's tags stay and cost nothing.
        let (mut ui, mut m) = opened(true, true);
        assert_eq!(m.points, 0);
        let guns = line(&mut ui, &m, "Guns");
        m.click(&mut ui, SKILL_ID, Some(guns), 0.0);
        assert_eq!(ui.string(guns, value).as_deref(), Some("30"));
    }
}
