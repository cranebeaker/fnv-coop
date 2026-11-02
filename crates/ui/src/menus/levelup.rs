//! Levelling up (`menus\levelup_menu.xml`, class `LevelUpMenu` 1027 in
//! FalloutNV.exe): a page of skills to share the level's points among,
//! then (on perk levels) a page of perks to pick one from. Read from the
//! code (`00784c80` and the functions after it):
//!
//! * opening (`00784c80`): fails without tile ids 0–9 (`00785190`); the
//!   title (id 0) `sLevelUpTitleText` with the level ("WELCOME TO LEVEL
//!   %d"), Reset (6) `sReset`, Continue (7) `_Title_0` `sContinue` and
//!   `_Title_1` `sDone` (the file shows the second on the last page), Back
//!   (8) `sBack`; page 0's most points the level's skill points (after perk
//!   entry 10), page 1's 1; with no perk listed `_EndPage` 0 (the file
//!   says 1); filled; page 0.
//! * the skills (`00785540`): one line each (`LUM_SkillTemplate`, id 11)
//!   with `_AddedValue` 0 and `_BaseValue` the player's permanent value,
//!   sorted by name, case ignored (`007da270`); page 0's most is cut to
//!   what fits under 100 in all of them (and not below 0). The perks
//!   (`LUM_PerkTemplate`, id 12): "name (rank)" for a further rank of a
//!   perk held, `_Rank` the rank it would be.
//! * a page (`00785830`): past `_EndPage` the menu closes; else
//!   `_CurrentPage`, `_CurrPoints`, `_MaxPoints`, the counter; page 0 the
//!   skills' numbers (`007866e0`: `_DisplayString` = base + extra + added,
//!   kept within 0..100); page 1 each perk's `_TextAlpha` (`007864e0`: 255
//!   when its conditions pass and the player's level reaches its own, else
//!   128) and the perks sorted (`007865d0`: the brighter first, then by
//!   level, then by name).
//! * the counter (`00785990`, id 5): what's left of the page's points;
//!   none: hidden; one: `sLevelUpSkillCounter` / `sLevelUpPerkCounter`;
//!   more: `sLevelUpSkillCounterPl` / `sLevelUpPerkCounterPl` with the
//!   number.
//! * clicks (`00785aa0`): an arrow (13 less, 14 more; the file shows them
//!   while something can be taken back or given), when shown, takes or
//!   gives one of the page's points, and changes the skill's base value at
//!   once by that (× `iSkillPointsTagSkillMult` for the player's tag
//!   skills); a perk (12) picked again is let go, a bright one is picked
//!   alone; Reset (6) gives every point back (sound 1 when any were given)
//!   or lets the perk go; Continue (7) the next page, Back (8) the one
//!   before, both clearing the button's `mouseover`.
//! * closing (`007851d0`): the perk picked is given (`00786570`: its next
//!   rank); the skill points were given as they were clicked.
//! * the pointer on a skill (`00785280`): its picture (the actor value's
//!   `ICON`) in `LUM_SelectionIcon`, its description in
//!   `LUM_SelectionText`, a weapon skill's badge (`0066ea20`) in
//!   `stats_icon_badge`; on a perk: its picture (shown when its path has 2
//!   or more letters), the requirements text; sound 4 (`UIMenuFocus`). On
//!   an arrow: its skill chosen and shown.
//! * Right and Left (`00785ed0`) on the skills page click the chosen
//!   skill's arrows (with their click sound) when shown.
//! * Shown with `MUSSuccess` (`00785e20`).
//!
//! Not here: a perk's own skill bonuses shown while it's picked
//! (`007867a0`, `00786240`: `_ExtraValue`, `_OverflowValue`), the
//! controller's buttons (special codes 9, 10 and the item chosen at a
//! page's start, `00786a50`).

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\levelup_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1027;
/// A skill line's id, a perk line's, the arrows'.
pub const SKILL_ID: i32 = 11;
pub const PERK_ID: i32 = 12;
pub const LESS_ID: i32 = 13;
pub const MORE_ID: i32 = 14;
/// The sound it's shown with (`00785e20`).
pub const OPEN_SOUND: &str = "MUSSuccess";

/// The weapon skills' badges (`0066ea20`), by actor value.
pub fn skill_badge(av: u16) -> Option<&'static str> {
    Some(match av {
        0x21 => "Interface\\Icons\\TypeIcons\\weap_skill_icon_big_guns.dds",
        0x22 => "Interface\\Icons\\TypeIcons\\weap_skill_icon_energy.dds",
        0x23 => "Interface\\Icons\\TypeIcons\\weap_skill_icon_explosives.dds",
        0x26 => "Interface\\Icons\\TypeIcons\\weap_skill_icon_melee.dds",
        0x29 => "Interface\\Icons\\TypeIcons\\weap_skill_icon_sm_arms.dds",
        0x2D => "Interface\\Icons\\TypeIcons\\weap_skill_icon_unarmed.dds",
        _ => return None,
    })
}

/// A skill the page lists.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Skill {
    /// Its actor value.
    pub av: u16,
    pub name: String,
    /// The player's permanent value, whole.
    pub base: i32,
    /// One of the player's tag skills.
    pub tagged: bool,
    /// The actor value's `ICON` and `DESC`.
    pub icon: String,
    pub description: String,
}

/// A perk the page lists.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Perk {
    pub form: u32,
    pub name: String,
    /// The rank it would be (1 for a perk not held) and how many it has.
    pub rank: u8,
    pub ranks: u8,
    pub min_level: u8,
    /// Its conditions pass and the player's level reaches its own (asked
    /// again whenever the perk page is shown: the skill points just given
    /// count).
    pub available: bool,
    pub icon: String,
    /// The information text (`005ebac0`).
    pub text: String,
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// The player's base value of a skill changes by this much now.
    Skill { av: u16, by: i32 },
    /// The menu closed; the perk picked (its next rank is given).
    Close { perk: Option<u32> },
}

/// The level-up menu.
#[derive(Debug)]
pub struct LevelUpMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..9 (`+0x2c`).
    pub tiles: [Option<TileId>; 10],
    /// The skills' list (`+0x64`) and the perks' (`+0x94`).
    pub skill_list: ListBox,
    pub perk_list: ListBox,
    pub skills: Vec<Skill>,
    pub perks: Vec<Perk>,
    /// The page (`+0x28`), each page's points given (`+0x54`) and its most
    /// (`+0x5c`).
    pub page: usize,
    pub points: [i32; 2],
    pub most: [i32; 2],
    /// `iSkillPointsTagSkillMult`.
    pub tag_mult: i32,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

fn custom(ui: &mut Ui, name: &str) -> i32 {
    ui.names.lookup_or_add(name).unwrap_or(0)
}

/// The perks' lines (`00785540`, `007e7030`): "name (rank)" for a further
/// rank of a perk held, the line's `id`, `_Rank` the rank it would be.
pub(crate) fn add_perk_lines(
    ui: &mut Ui,
    menu: TileId,
    list: &mut ListBox,
    perks: &[Perk],
    id: i32,
) {
    let rank_id = custom(ui, "_Rank");
    for (i, p) in perks.iter().enumerate() {
        let name = if p.ranks > 1 && p.rank > 1 {
            format!("{} ({})", p.name, p.rank)
        } else {
            p.name.clone()
        };
        if let Some(tile) = list.add(ui, menu, i as i32, Some(&name)) {
            ui.set_number(tile, t::ID, id as f32);
            ui.set_number(tile, rank_id, f32::from(p.rank));
        }
    }
}

/// Each perk line's `_TextAlpha` (`007864e0`, `007e7670`: 255 when it can
/// be picked, else 128), then the lines sorted (`007865d0`, `007e7740`):
/// the brighter first, then by level, then by name.
pub(crate) fn brighten_and_sort(ui: &mut Ui, list: &mut ListBox, perks: &[Perk]) {
    let alpha = custom(ui, "_TextAlpha");
    for line in list.items.clone() {
        let bright = perks.get(line.value as usize).is_some_and(|p| p.available);
        ui.set_number(line.tile, alpha, if bright { 255.0 } else { 128.0 });
    }
    let levels: Vec<(TileId, u8)> = list
        .items
        .iter()
        .map(|l| {
            (
                l.tile,
                perks.get(l.value as usize).map_or(0, |p| p.min_level),
            )
        })
        .collect();
    let level_of = |tile: TileId| {
        levels
            .iter()
            .find(|(t_, _)| *t_ == tile)
            .map_or(0, |(_, l)| *l)
    };
    list.sort(ui, &|ui, a, b| {
        let (aa, ab) = (ui.number(a, alpha), ui.number(b, alpha));
        if aa != ab {
            return aa > ab;
        }
        let (la, lb) = (level_of(a), level_of(b));
        if la != lb {
            return la < lb;
        }
        let sa = ui
            .string(a, t::STRING)
            .unwrap_or_default()
            .to_ascii_lowercase();
        let sb = ui
            .string(b, t::STRING)
            .unwrap_or_default()
            .to_ascii_lowercase();
        sa < sb
    });
}

/// The lines picked (`_selected`, `007878d0`).
pub(crate) fn picked(ui: &mut Ui, list: &ListBox) -> Vec<TileId> {
    let sel = custom(ui, "_selected");
    list.items
        .iter()
        .filter(|l| {
            let v = ui.number(l.tile, sel);
            v != 0.0 && v != crate::names::NOT_FOUND
        })
        .map(|l| l.tile)
        .collect()
}

impl LevelUpMenu {
    pub fn new(menu: TileId) -> LevelUpMenu {
        LevelUpMenu {
            menu,
            tiles: [None; 10],
            skill_list: ListBox::default(),
            perk_list: ListBox::default(),
            skills: Vec::new(),
            perks: Vec::new(),
            page: 0,
            points: [0, 0],
            most: [0, 1],
            tag_mult: 1,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    /// Opens it (`00784c80`) for `level` with `points` skill points to
    /// give; `perks` empty when this level gives none. False without its
    /// tiles.
    pub fn open(
        &mut self,
        ui: &mut Ui,
        level: u16,
        points: i32,
        skills: Vec<Skill>,
        perks: Vec<Perk>,
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        let text = |ui: &Ui, n: &str| ui.setting_text(n).unwrap_or_default();
        if let Some(title) = self.tile(0) {
            let f = text(ui, "sLevelUpTitleText");
            ui.set_text(title, t::STRING, &f.replacen("%d", &level.to_string(), 1));
        }
        if let Some(reset) = self.tile(6) {
            let s = text(ui, "sReset");
            ui.set_text(reset, t::STRING, &s);
        }
        if let Some(cont) = self.tile(7) {
            let (a, b) = (custom(ui, "_Title_0"), custom(ui, "_Title_1"));
            let (s0, s1) = (text(ui, "sContinue"), text(ui, "sDone"));
            ui.set_string(cont, a, &s0);
            ui.set_string(cont, b, &s1);
        }
        if let Some(back) = self.tile(8) {
            let s = text(ui, "sBack");
            ui.set_text(back, t::STRING, &s);
        }
        self.points = [0, 0];
        self.most = [points, 1];
        if perks.is_empty() {
            let end = custom(ui, "_EndPage");
            ui.set_number(self.menu, end, 0.0);
        }
        self.skills = skills;
        self.perks = perks;
        self.fill(ui);
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.show_page(ui, 0);
        self.sounds.push(OPEN_SOUND.to_string());
        true
    }

    /// The lists (`00785540`).
    fn fill(&mut self, ui: &mut Ui) {
        if let Some(list) = self.tile(1) {
            self.skill_list = ListBox::new(ui, list, "LUM_SkillTemplate");
        }
        let (added, base) = (custom(ui, "_AddedValue"), custom(ui, "_BaseValue"));
        let mut sum = 0;
        for i in 0..self.skills.len() {
            let s = self.skills[i].clone();
            if let Some(tile) = self.skill_list.add(ui, self.menu, i as i32, Some(&s.name)) {
                ui.set_number(tile, t::ID, SKILL_ID as f32);
                ui.set_number(tile, added, 0.0);
                ui.set_number(tile, base, s.base as f32);
            }
            sum += s.base;
        }
        // By name, case ignored.
        let names: Vec<String> = self
            .skills
            .iter()
            .map(|s| s.name.to_ascii_lowercase())
            .collect();
        let values: Vec<(TileId, i32)> = self
            .skill_list
            .items
            .iter()
            .map(|i| (i.tile, i.value))
            .collect();
        let name_of = |tile: TileId| {
            values
                .iter()
                .find(|(t_, _)| *t_ == tile)
                .and_then(|(_, v)| names.get(*v as usize))
                .cloned()
                .unwrap_or_default()
        };
        self.skill_list.sort(ui, &|_, a, b| name_of(a) < name_of(b));
        let fit = self.skills.len() as i32 * 100 - sum;
        self.most[0] = self.most[0].min(fit).max(0);
        if let Some(list) = self.tile(2) {
            self.perk_list = ListBox::new(ui, list, "LUM_PerkTemplate");
        }
        add_perk_lines(ui, self.menu, &mut self.perk_list, &self.perks, PERK_ID);
    }

    /// A page (`00785830`): past the last the menu closes.
    pub fn show_page(&mut self, ui: &mut Ui, page: i32) {
        let end = custom(ui, "_EndPage");
        if page as f32 > ui.number(self.menu, end) {
            self.close(ui);
            return;
        }
        // The file shows Back only past page 0.
        let Ok(page) = usize::try_from(page) else {
            return;
        };
        self.page = page;
        let (cur, curr, most) = (
            custom(ui, "_CurrentPage"),
            custom(ui, "_CurrPoints"),
            custom(ui, "_MaxPoints"),
        );
        ui.set_number(self.menu, cur, page as f32);
        ui.set_number(self.menu, curr, self.points[page] as f32);
        ui.set_number(self.menu, most, self.most[page] as f32);
        self.counter(ui);
        if page == 0 {
            for line in self.skill_list.items.clone() {
                Self::display(ui, line.tile);
            }
            self.skill_list.select(ui, None);
        } else {
            brighten_and_sort(ui, &mut self.perk_list, &self.perks);
            self.perk_list.select(ui, None);
        }
        ui.refresh();
    }

    /// A skill line's number (`007866e0`).
    fn display(ui: &mut Ui, tile: TileId) {
        let (b, e, a) = (
            custom(ui, "_BaseValue"),
            custom(ui, "_ExtraValue"),
            custom(ui, "_AddedValue"),
        );
        let number = |ui: &mut Ui, id: i32| {
            let v = ui.number(tile, id);
            if v == crate::names::NOT_FOUND {
                0.0
            } else {
                v
            }
        };
        let v = (number(ui, b) + number(ui, e) + number(ui, a)) as i32;
        let s = custom(ui, "_DisplayString");
        ui.set_string(tile, s, &v.clamp(0, 100).to_string());
    }

    /// `_CurrPoints` for the page.
    fn set_points(&mut self, ui: &mut Ui) {
        let curr = custom(ui, "_CurrPoints");
        ui.set_number(self.menu, curr, self.points[self.page] as f32);
    }

    /// The counter (`00785990`).
    fn counter(&mut self, ui: &mut Ui) {
        let Some(counter) = self.tile(5) else {
            return;
        };
        let left = self.most[self.page] - self.points[self.page];
        let skills = self.page == 0;
        let setting = |ui: &Ui, n: &str| ui.setting_text(n).unwrap_or_default();
        match left {
            0 => ui.set_number(counter, t::VISIBLE, 0.0),
            1 => {
                let s = setting(
                    ui,
                    if skills {
                        "sLevelUpSkillCounter"
                    } else {
                        "sLevelUpPerkCounter"
                    },
                );
                ui.set_text(counter, t::STRING, &s);
                ui.set_number(counter, t::VISIBLE, 1.0);
            }
            n => {
                let f = setting(
                    ui,
                    if skills {
                        "sLevelUpSkillCounterPl"
                    } else {
                        "sLevelUpPerkCounterPl"
                    },
                );
                ui.set_text(counter, t::STRING, &f.replacen("%d", &n.to_string(), 1));
                ui.set_number(counter, t::VISIBLE, 1.0);
            }
        }
        ui.refresh();
    }

    fn picked_perks(&self, ui: &mut Ui) -> Vec<TileId> {
        picked(ui, &self.perk_list)
    }

    /// Closes it (`007851d0`).
    fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        let perk = self
            .picked_perks(ui)
            .first()
            .and_then(|&tile| self.perk_list.value_of(tile))
            .and_then(|v| self.perks.get(v as usize))
            .map(|p| p.form);
        self.requests.push(Request::Close { perk });
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }

    /// An arrow clicked (`00785aa0` cases 0xd, 0xe).
    fn arrow(&mut self, ui: &mut Ui, arrow: TileId, more: bool) {
        if ui.number(arrow, t::VISIBLE) == 0.0 {
            return;
        }
        let Some(line) = ui.tiles[arrow].parent else {
            return;
        };
        let Some(skill) = self
            .skill_list
            .value_of(line)
            .and_then(|v| self.skills.get(v as usize))
            .cloned()
        else {
            return;
        };
        let mut by = if more { 1 } else { -1 };
        self.points[self.page] += by;
        self.set_points(ui);
        if skill.tagged {
            by *= self.tag_mult;
        }
        let added = custom(ui, "_AddedValue");
        let v = ui.number(line, added) + by as f32;
        ui.set_number(line, added, v);
        self.requests.push(Request::Skill { av: skill.av, by });
        Self::display(ui, line);
        self.counter(ui);
    }

    /// A perk clicked (`00785aa0` case 0xc).
    fn perk(&mut self, ui: &mut Ui, line: TileId) {
        let (sel, alpha) = (custom(ui, "_selected"), custom(ui, "_TextAlpha"));
        if self.picked_perks(ui).contains(&line) {
            ui.set_number(line, sel, 0.0);
        } else if ui.number(line, alpha) == 255.0 {
            for other in self.picked_perks(ui) {
                ui.set_number(other, sel, 0.0);
            }
            ui.set_number(line, sel, 1.0);
        }
        self.points[self.page] = self.picked_perks(ui).len() as i32;
        self.set_points(ui);
        self.counter(ui);
    }

    /// Reset (`00785aa0` case 6, `00786170`, `00786200`).
    fn reset(&mut self, ui: &mut Ui) {
        if self.points[self.page] > 0 {
            if let Some(s) = menu::menu_sound(1) {
                self.sounds.push(s.to_string());
            }
        }
        self.points[self.page] = 0;
        self.set_points(ui);
        if self.page == 0 {
            let added = custom(ui, "_AddedValue");
            for line in self.skill_list.items.clone() {
                let n = ui.number(line.tile, added) as i32;
                if n != 0 {
                    if let Some(s) = self.skills.get(line.value as usize) {
                        self.requests.push(Request::Skill { av: s.av, by: -n });
                    }
                }
                ui.set_number(line.tile, added, 0.0);
                Self::display(ui, line.tile);
            }
        } else {
            let sel = custom(ui, "_selected");
            for line in self.picked_perks(ui) {
                ui.set_number(line, sel, 0.0);
            }
        }
        self.counter(ui);
    }

    /// A line's child with an id.
    fn child_with_id(ui: &mut Ui, line: TileId, id: i32) -> Option<TileId> {
        let children = ui.tiles[line].children.clone();
        children
            .into_iter()
            .find(|&c| ui.has(c, t::ID) && ui.number(c, t::ID) as i32 == id)
    }
}

impl MenuCode for LevelUpMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..10).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        match id {
            6 => self.reset(ui),
            7 | 8 => {
                let page = self.page as i32 + if id == 7 { 1 } else { -1 };
                self.show_page(ui, page);
                if let Some(button) = tile {
                    ui.set_number(button, t::MOUSEOVER, 0.0);
                }
            }
            PERK_ID => {
                if let Some(line) = tile {
                    self.perk(ui, line);
                }
            }
            LESS_ID | MORE_ID => {
                if let Some(arrow) = tile {
                    self.arrow(ui, arrow, id == MORE_ID);
                }
            }
            _ => {}
        }
        ui.refresh();
    }

    /// `00785280`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        let (icon, text, badge) = (self.tile(3), self.tile(4), self.tile(9));
        match id {
            SKILL_ID => {
                let Some(s) = self
                    .skill_list
                    .value_of(tile)
                    .and_then(|v| self.skills.get(v as usize))
                    .cloned()
                else {
                    return;
                };
                if let Some(i) = icon {
                    ui.set_string(i, t::FILENAME, &s.icon);
                    ui.set_number(i, t::VISIBLE, 1.0);
                }
                if let Some(x) = text {
                    ui.set_text(x, t::STRING, &s.description);
                    ui.set_number(x, t::VISIBLE, 1.0);
                }
                if let Some(b) = badge {
                    match skill_badge(s.av) {
                        Some(path) => {
                            ui.set_number(b, t::VISIBLE, 1.0);
                            ui.set_string(b, t::FILENAME, path);
                        }
                        None => ui.set_number(b, t::VISIBLE, 0.0),
                    }
                }
                if let Some(snd) = menu::menu_sound(4) {
                    self.sounds.push(snd.to_string());
                }
            }
            PERK_ID => {
                let Some(p) = self
                    .perk_list
                    .value_of(tile)
                    .and_then(|v| self.perks.get(v as usize))
                    .cloned()
                else {
                    return;
                };
                if let Some(i) = icon {
                    ui.set_string(i, t::FILENAME, &p.icon);
                    let shown = p.icon.len() >= 2;
                    ui.set_number(i, t::VISIBLE, if shown { 1.0 } else { 0.0 });
                }
                if let Some(x) = text {
                    ui.set_text(x, t::STRING, &p.text);
                    ui.set_number(x, t::VISIBLE, 1.0);
                }
                if let Some(b) = badge {
                    ui.set_number(b, t::VISIBLE, 0.0);
                }
                if let Some(snd) = menu::menu_sound(4) {
                    self.sounds.push(snd.to_string());
                }
            }
            LESS_ID | MORE_ID => {
                if let Some(line) = ui.tiles[tile].parent {
                    self.skill_list.select(ui, Some(line));
                    self.mouseover(ui, SKILL_ID, line);
                }
            }
            _ => {}
        }
        ui.refresh();
    }

    /// `00785ed0`: Right and Left click the chosen skill's arrows.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if code != menu::special::LEFT && code != menu::special::RIGHT {
            return false;
        }
        if self.page != 0 {
            return false;
        }
        let Some(line) = self.skill_list.selected else {
            return false;
        };
        let id = if code == menu::special::RIGHT {
            MORE_ID
        } else {
            LESS_ID
        };
        if let Some(arrow) = Self::child_with_id(ui, line, id) {
            if ui.number(arrow, t::VISIBLE) != 0.0 {
                if let Some(s) = ui.string(arrow, t::CLICKSOUND) {
                    let s = s.trim().to_string();
                    if !s.is_empty() {
                        self.sounds.push(s);
                    }
                }
                self.click(ui, id, Some(arrow), now);
            }
        }
        true
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.skill_list, &mut self.perk_list]
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
            icon: format!("Interface\\Icons\\{name}.dds"),
            description: format!("About {name}."),
        }
    }

    fn perk(form: u32, name: &str, rank: u8, ranks: u8, min_level: u8, available: bool) -> Perk {
        Perk {
            form,
            name: name.into(),
            rank,
            ranks,
            min_level,
            available,
            icon: String::new(),
            text: format!("Req: {name}"),
        }
    }

    fn opened(points: i32, skills: Vec<Skill>, perks: Vec<Perk>) -> (Ui, LevelUpMenu) {
        let mut ui = test_support::ui();
        let mut m = LevelUpMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::levelup_menu(), &mut m);
        assert!(m.open(&mut ui, 4, points, skills, perks));
        (ui, m)
    }

    fn texts(ui: &mut Ui, list: &ListBox) -> Vec<String> {
        list.shown_items(ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect()
    }

    fn line(ui: &mut Ui, list: &ListBox, text: &str) -> TileId {
        list.shown_items(ui, 0, i32::MAX)
            .into_iter()
            .find(|l| ui.string(l.tile, t::STRING).as_deref() == Some(text))
            .unwrap()
            .tile
    }

    fn shown(ui: &mut Ui, line: TileId, id: i32) -> Option<TileId> {
        LevelUpMenu::child_with_id(ui, line, id).filter(|&a| ui.number(a, t::VISIBLE) != 0.0)
    }

    /// `00784c80`, `00785540`, `00785830`, `00785990`: the title, the
    /// skills by name with their numbers, the counter, the arrows the file
    /// shows; the most points cut to what fits under 100.
    #[test]
    fn opening_on_the_skills_page() {
        let skills = vec![
            skill(40, "Science", 98, false),
            skill(32, "barter", 15, false),
            skill(43, "Speech", 99, true),
        ];
        let (mut ui, m) = opened(13, skills, vec![perk(0x900, "Toughness", 1, 2, 2, true)]);
        assert_eq!(
            ui.string(m.tiles[0].unwrap(), t::STRING).as_deref(),
            Some("WELCOME TO LEVEL 4")
        );
        assert_eq!(
            texts(&mut ui, &m.skill_list),
            ["barter", "Science", "Speech"]
        );
        // 300 − (98 + 15 + 99) = 88 fits; 13 asked.
        assert_eq!(m.most, [13, 1]);
        assert_eq!(
            ui.string(m.tiles[5].unwrap(), t::STRING).as_deref(),
            Some("ASSIGN 13 SKILL POINTS")
        );
        let science = line(&mut ui, &m.skill_list, "Science");
        let display = ui.names.lookup("_DisplayString").unwrap();
        assert_eq!(ui.string(science, display).as_deref(), Some("98"));
        assert!(shown(&mut ui, science, MORE_ID).is_some());
        assert!(shown(&mut ui, science, LESS_ID).is_none());
        assert_eq!(m.sounds, [OPEN_SOUND]);
        // A level whose points don't fit.
        let (_, m) = opened(13, vec![skill(40, "Science", 95, false)], vec![]);
        assert_eq!(m.most[0], 5);
    }

    /// `00785aa0`: the arrows give and take points (a tag skill's count
    /// `iSkillPointsTagSkillMult` times) and change the skill at once;
    /// the counter follows; Reset gives them all back.
    #[test]
    fn giving_points() {
        let skills = vec![
            skill(40, "Science", 50, false),
            skill(43, "Speech", 30, true),
        ];
        let (mut ui, mut m) = opened(2, skills, vec![]);
        m.tag_mult = 2;
        let science = line(&mut ui, &m.skill_list, "Science");
        let more = shown(&mut ui, science, MORE_ID).unwrap();
        m.click(&mut ui, MORE_ID, Some(more), 0.0);
        let display = ui.names.lookup("_DisplayString").unwrap();
        assert_eq!(ui.string(science, display).as_deref(), Some("51"));
        assert_eq!(
            ui.string(m.tiles[5].unwrap(), t::STRING).as_deref(),
            Some("ASSIGN 1 SKILL POINT")
        );
        assert!(shown(&mut ui, science, LESS_ID).is_some());
        let speech = line(&mut ui, &m.skill_list, "Speech");
        let more = shown(&mut ui, speech, MORE_ID).unwrap();
        m.click(&mut ui, MORE_ID, Some(more), 0.0);
        assert_eq!(ui.string(speech, display).as_deref(), Some("32"));
        assert_eq!(
            m.requests,
            [
                Request::Skill { av: 40, by: 1 },
                Request::Skill { av: 43, by: 2 }
            ]
        );
        // All given: no counter, no more arrows, Continue usable.
        assert_eq!(ui.number(m.tiles[5].unwrap(), t::VISIBLE), 0.0);
        assert!(shown(&mut ui, science, MORE_ID).is_none());
        assert_eq!(ui.number(m.tiles[7].unwrap(), t::TARGET), 1.0);
        // Left on the chosen skill takes one back.
        m.skill_list.select(&mut ui, Some(science));
        assert!(m.special_key(&mut ui, menu::special::LEFT, 0.0));
        assert_eq!(ui.string(science, display).as_deref(), Some("50"));
        assert_eq!(m.points[0], 1);
        // Reset: everything back, with sound 1.
        m.requests.clear();
        m.click(&mut ui, 6, None, 0.0);
        assert_eq!(m.requests, [Request::Skill { av: 43, by: -2 }]);
        assert_eq!(m.points[0], 0);
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuOK"));
        assert_eq!(ui.string(speech, display).as_deref(), Some("30"));
    }

    /// `00785830`, `007864e0`, `007865d0`, `00785aa0` case 0xc: the perks
    /// page sorts the bright ones first, then by level and name; only a
    /// bright perk can be picked, one at a time; Done closes with it.
    #[test]
    fn picking_a_perk() {
        let perks = vec![
            perk(0x901, "Toughness", 2, 2, 2, true),
            perk(0x902, "Comprehension", 1, 1, 4, true),
            perk(0x903, "Better Criticals", 1, 1, 16, false),
            perk(0x904, "Educated", 1, 1, 4, true),
        ];
        let (mut ui, mut m) = opened(0, vec![skill(40, "Science", 100, false)], perks);
        // Nothing to give on page 0: on to the perks.
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.page, 1);
        assert_eq!(
            texts(&mut ui, &m.perk_list),
            [
                "Toughness (2)",
                "Comprehension",
                "Educated",
                "Better Criticals"
            ]
        );
        let alpha = ui.names.lookup("_TextAlpha").unwrap();
        let dim = line(&mut ui, &m.perk_list, "Better Criticals");
        assert_eq!(ui.number(dim, alpha), 128.0);
        assert_eq!(
            ui.string(m.tiles[5].unwrap(), t::STRING).as_deref(),
            Some("CHOOSE 1 PERK")
        );
        m.click(&mut ui, PERK_ID, Some(dim), 0.0);
        assert_eq!(m.points[1], 0);
        let educated = line(&mut ui, &m.perk_list, "Educated");
        m.click(&mut ui, PERK_ID, Some(educated), 0.0);
        let tough = line(&mut ui, &m.perk_list, "Toughness (2)");
        m.click(&mut ui, PERK_ID, Some(tough), 0.0);
        assert_eq!(m.points[1], 1);
        assert_eq!(m.picked_perks(&mut ui), [tough]);
        // The pointer on a perk shows its text.
        m.mouseover(&mut ui, PERK_ID, tough);
        assert_eq!(
            ui.string(m.tiles[4].unwrap(), t::STRING).as_deref(),
            Some("Req: Toughness")
        );
        assert_eq!(ui.number(m.tiles[3].unwrap(), t::VISIBLE), 0.0);
        // Back, then on, then Done.
        m.click(&mut ui, 8, None, 0.0);
        assert_eq!(m.page, 0);
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.picked_perks(&mut ui), [tough]);
        m.click(&mut ui, 7, None, 0.0);
        assert!(m.closed);
        assert_eq!(
            m.requests.last(),
            Some(&Request::Close { perk: Some(0x901) })
        );
    }

    /// No perk this level: `_EndPage` 0, Continue closes after the skills.
    #[test]
    fn a_level_without_a_perk() {
        let (mut ui, mut m) = opened(1, vec![skill(40, "Science", 10, false)], vec![]);
        let cont = m.tiles[7].unwrap();
        // The button says "Done" on the last page.
        assert_eq!(ui.string(cont, t::STRING).as_deref(), Some("Done"));
        let science = line(&mut ui, &m.skill_list, "Science");
        m.mouseover(&mut ui, SKILL_ID, science);
        assert_eq!(
            ui.string(m.tiles[4].unwrap(), t::STRING).as_deref(),
            Some("About Science.")
        );
        assert_eq!(ui.number(m.tiles[9].unwrap(), t::VISIBLE), 0.0);
        let more = shown(&mut ui, science, MORE_ID).unwrap();
        m.click(&mut ui, MORE_ID, Some(more), 0.0);
        m.click(&mut ui, 7, Some(cont), 0.0);
        assert!(m.closed);
        assert_eq!(m.requests.last(), Some(&Request::Close { perk: None }));
        assert_eq!(
            skill_badge(0x29),
            Some("Interface\\Icons\\TypeIcons\\weap_skill_icon_sm_arms.dds")
        );
    }
}
