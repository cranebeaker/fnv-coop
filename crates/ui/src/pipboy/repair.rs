//! The Pip-Boy's repair screen (`menus\repair_menu.xml`, class
//! `RepairMenu` 1035, vtable `01075c5c` in FalloutNV.exe): ITEMS' Repair
//! button (id 8, R) on the chosen item opens it (`00780140` case 8: the
//! `UIMenuMode` sound, `007b7020(item)`); the item is mended with another
//! of the player's things. The rules are `world::repair`'s. Read from the
//! code:
//!
//! * opening (`007b7020`): the condition card (id 4) `user0` the item's
//!   condition, shown; its figure card (5) (`world::repair::shown_stat`);
//!   the picture (3); the list (1, `RM_RepairListTemplate`, `007b6aa0`):
//!   the item first where its kind comes (marked `_IsRepairItem`, the
//!   brackets, id 2, on it), then every one of each thing that can mend
//!   it; each line (`007b57f0`): id 0xe, shown, its meter (0xf) condition ×
//!   0.6 wide, the worn mark (its last child), `user0` the item's
//!   condition after mending with it (`world::repair::mended_condition`);
//!   sorted (`007b5950`): the marked one first, then the best condition
//!   first, the worn first on a tie; the player's Repair (card 0's
//!   `_Value`); the first line chosen only with a pad (`004b71d0`,
//!   `00715860`), then `DoEnter` with the list's choice.
//! * a line chosen (`007b6120`): the repair button (11, shown only with a
//!   controller) reads "`sMaintainItem`" for a line's thing at 75% or more
//!   (armour 50%), else "`sRepairItem`"; on the item itself, with others to
//!   choose from, "`sSelectItemToRepair`" (8, its alpha pulsing 255 to 64
//!   every 2 s, `007b6970`) instead of the repaired card (9), the button dim
//!   (`_line_alpha` 128); on another, the button bright, the repaired card
//!   shown with that line's `user0` and its figure (10), "+n%" (6) and
//!   "+n" (7): the differences, at least 0, rounded. (The "skill needed"
//!   and "can't repair past" lines need a line's `user1`, which nothing
//!   sets.)
//! * a click on a line, or Enter on the chosen one (`007b5b40` case 0xe):
//!   the item mended with the chosen line (`world::repair::repair_with`,
//!   `UIRepairWeapon`) when the clicked line's text (its first child,
//!   `007b5370`) has alpha 255, the chosen line isn't the item itself and
//!   the repaired card (9) shows. The used line leaves the list, the choice
//!   moving to the line after it (before it when it was the last;
//!   `0071a5c0` → `0071aaa0`); then closed (a click on Cancel) when the
//!   item reaches 99% or only it is left, else the lines are worked out
//!   again and sorted without being made again (`007b7020(0)`) and
//!   `DoEnter` runs with the choice. E, Cancel (12, case 0xc):
//!   `UIMenuMode`, back to ITEMS.
//! * the mouse (`DoEnter` `007b6120`, `DoLeave` `007b6a00`, vtable slots
//!   0x10 and 0x14; `DoWheelMove` is the base menu's, so the wheel only
//!   moves the list's scroll bar): the pointer onto a line makes it the
//!   list's choice (the interface, `00717e70`), then `DoEnter`: when the
//!   chosen line's `listindex` isn't the one the scroll knob last turned
//!   for (`011da7d8`, a global, 0 at start and kept between openings) the
//!   knob turns a notch with `UIPipBoyScroll` (`007f8610(0, ±
//!   fScrollKnobIncrement, fScrollKnobRate, 1)`, + for a later line), then
//!   the line's details as above. The keys reach it the same way; the
//!   lines' own `mouseoversound` is `&nosound;`. Off a line (`DoLeave`,
//!   after the list lets its choice go, `00717ef0`): the repaired card (9)
//!   hidden, and with more than one line "choose an item" (8) too.
//! * the brackets (`007b6aa0`): on the item's line by operators, the left
//!   bracket's height the line's less 1, its y the line's (no animation).
//!
//! Not here: the switch between the two menus.

use super::{by_id, text, trait_id, Action, Key};
use crate::anim::Animations;
use crate::listbox::ListBox;
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::repair::Stat;

/// The menu's file.
pub const FILE: &str = "menus\\repair_menu.xml";
/// The list's template.
pub const TEMPLATE: &str = "RM_RepairListTemplate";
/// A line's id and its meter's.
pub const LINE_ID: i32 = 0xe;
pub const METER_ID: i32 = 0xf;
/// The Cancel button.
pub const CANCEL: i32 = 12;
/// The sounds: a repair, the menu changing.
pub const REPAIR_SOUND: &str = "UIRepairWeapon";
pub const MENU_SOUND: &str = "UIMenuMode";

/// One line: the chosen item, or one thing that can mend it.
#[derive(Debug, Clone, PartialEq)]
pub struct RepairRow {
    pub form: u32,
    pub name: String,
    /// Its condition in percent.
    pub condition: f32,
    /// Armour (the "Maintain" line's 50%; weapons 75%).
    pub armour: bool,
    pub equipped: bool,
    /// The chosen item itself.
    pub chosen: bool,
    /// The item's condition after mending with it (0..1).
    pub mends_to: f32,
    /// The item's figure at that condition.
    pub stat_after: Option<Stat>,
}

/// What the screen shows: the chosen item and its lines, in the player's
/// things' order (the menu sorts them).
#[derive(Debug, Clone, PartialEq)]
pub struct RepairInput {
    pub chosen: u32,
    /// Its condition in percent.
    pub condition: f32,
    pub icon: Option<String>,
    /// The player's Repair.
    pub skill: i32,
    /// The item's figure now.
    pub stat: Option<Stat>,
    pub rows: Vec<RepairRow>,
}

/// Whether two lines are the same (a line found again in the list made
/// anew): the item's own, or one of the same kind, worn or not, at the
/// same condition.
fn same_line(a: &RepairRow, b: &RepairRow) -> bool {
    a.chosen == b.chosen
        && (a.chosen
            || (a.form == b.form
                && a.equipped == b.equipped
                && (a.condition - b.condition).abs() < 1e-3))
}

/// `007b5950`: < 0 puts `a` first.
pub fn compare_rows(a: &RepairRow, b: &RepairRow) -> i32 {
    use std::cmp::Ordering;
    match (a.chosen, b.chosen) {
        (true, false) => -1,
        (false, true) => 1,
        _ => match b.condition.partial_cmp(&a.condition) {
            Some(Ordering::Greater) => 1,
            Some(Ordering::Less) => -1,
            _ if a.equipped => -1,
            _ if b.equipped => 1,
            _ => 0,
        },
    }
}

/// The game's list sort (`007653f0`: a Shell sort, gaps 1, 4, 13 …) with
/// [`compare_rows`].
fn sort_rows(rows: &mut [RepairRow]) {
    let n = rows.len();
    let mut gap = 1usize;
    while n > 0 && gap <= (n - 1) / 9 {
        gap = gap * 3 + 1;
    }
    while gap > 0 {
        for i in gap..n {
            let it = rows[i].clone();
            let mut j = i;
            while j >= gap && compare_rows(&it, &rows[j - gap]) < 0 {
                rows[j] = rows[j - gap].clone();
                j -= gap;
            }
            rows[j] = it;
        }
        gap /= 3;
    }
}

/// The repair screen.
pub struct RepairMenu {
    pub menu: TileId,
    pub list: ListBox,
    /// The lines, in the list's order.
    pub rows: Vec<RepairRow>,
    pub input: Option<RepairInput>,
    anims: Animations,
    now: f64,
    /// The chosen line's `listindex` the scroll knob last turned for
    /// (`011da7d8`: a global, 0 at start, kept from one opening to the
    /// next).
    pub knob_index: i32,
}

impl RepairMenu {
    /// Reads the menu (hidden).
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<RepairMenu, String> {
        let menu = super::load_menu(ui, FILE, read)?;
        let list_tile = by_id(ui, menu, 1).unwrap_or(menu);
        ui.set_number(menu, t::VISIBLE, 0.0);
        Ok(RepairMenu {
            menu,
            list: ListBox::new(menu, list_tile, TEMPLATE),
            rows: Vec::new(),
            input: None,
            anims: Animations::default(),
            now: 0.0,
            knob_index: 0,
        })
    }

    fn tile(&self, ui: &Ui, id: i32) -> Option<TileId> {
        by_id(ui, self.menu, id)
    }

    /// A figure card's title and value ("--" without one).
    fn stat_card(ui: &mut Ui, card: TileId, stat: Option<&Stat>) {
        let title = trait_id(ui, "_Title");
        let value = trait_id(ui, "_Value");
        let (setting, v) = match stat {
            Some(s) => (s.title, s.value),
            None => ("sInventoryDamageResistance", None),
        };
        let s = text(ui, setting);
        ui.set_string(card, title, &s);
        match v {
            Some(v) => ui.set_number(card, value, v as f32),
            None => ui.set_string(card, value, "--"),
        }
    }

    /// Opens it on an item (`007b7020` with the item) and shows it, the
    /// first line chosen only with a pad, then `DoEnter`. Returns what the
    /// game should do (the scroll knob).
    pub fn open(&mut self, ui: &mut Ui, input: RepairInput, pad: bool) -> Vec<Action> {
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        if let Some(icon) = self.tile(ui, 3) {
            match &input.icon {
                Some(path) => {
                    ui.set_string(icon, t::FILENAME, path);
                    ui.set_number(icon, t::VISIBLE, 1.0);
                }
                None => ui.set_number(icon, t::VISIBLE, 0.0),
            }
        }
        self.fill(ui, input);
        if pad {
            self.list.select(ui, Some(0));
        }
        self.entered(ui)
    }

    /// After a repair (`007b5b40`): the used line (the chosen one) leaves
    /// the list, the choice moving to the line after it, or to the one
    /// before when it was the last (`0071a5c0` → `0071aaa0`); the lines are
    /// worked out again and sorted (`007b7020(0)`), then `DoEnter` with the
    /// choice. Returns what the game should do (the scroll knob).
    pub fn refill(&mut self, ui: &mut Ui, input: RepairInput) -> Vec<Action> {
        let next = self.list.selected.and_then(|i| {
            let j = if i + 1 < self.rows.len() {
                i + 1
            } else {
                i.checked_sub(1)?
            };
            self.rows.get(j).cloned()
        });
        self.fill(ui, input);
        // Found again by kind, wear and condition; a kind's condition is
        // kept per holder here (not per instance), so a repair moves the
        // spare's too: then the first other line of its kind and wear.
        let at = next.and_then(|n| {
            self.rows.iter().position(|r| same_line(r, &n)).or_else(|| {
                self.rows.iter().position(|r| {
                    !n.chosen && !r.chosen && r.form == n.form && r.equipped == n.equipped
                })
            })
        });
        self.list.choose(ui, at);
        self.entered(ui)
    }

    /// Makes the lines (`007b6aa0`, `007b57f0`, sorted by `007b5950`) and
    /// fills the cards, nothing chosen.
    fn fill(&mut self, ui: &mut Ui, input: RepairInput) {
        if let Some(card) = self.tile(ui, 4) {
            ui.set_number(card, t::USER0, input.condition / 100.0);
            ui.set_number(card, t::VISIBLE, 1.0);
        }
        if let Some(card) = self.tile(ui, 5) {
            Self::stat_card(ui, card, input.stat.as_ref());
        }
        self.list.clear(ui);
        let mut rows = input.rows.clone();
        sort_rows(&mut rows);
        let marked = trait_id(ui, "_IsRepairItem");
        for row in &rows {
            let Some(tile) = self.list.add(ui, Some(&row.name)) else {
                continue;
            };
            ui.set_number(tile, t::ID, LINE_ID as f32);
            let index = ui.number(tile, t::LISTINDEX);
            ui.set_number(tile, t::VISIBLE, if index >= 0.0 { 1.0 } else { 0.0 });
            let children = ui.tiles[tile].children.clone();
            if let Some(&meter) = children
                .iter()
                .find(|&&c| ui.number(c, t::ID) as i32 == METER_ID)
            {
                ui.set_number(meter, t::WIDTH, row.condition * 0.6);
            }
            if row.equipped {
                if let Some(&mark) = children.last() {
                    ui.set_number(mark, t::VISIBLE, 1.0);
                }
            }
            ui.set_number(tile, t::USER0, row.mends_to);
            if row.chosen {
                ui.set_number(tile, marked, 1.0);
                // The brackets on it (`007b6aa0`: the left bracket's
                // height `copy` the line's then `sub` 1, its y `copy` the
                // line's; the right bracket copies both in the file).
                if let Some(bracket) = self.tile(ui, 2) {
                    ui.link(bracket, t::Y, tile, t::Y);
                    ui.link(bracket, t::HEIGHT, tile, t::HEIGHT);
                    ui.add_action(
                        bracket,
                        t::HEIGHT,
                        crate::names::op::SUB,
                        crate::tile::Operand::Constant(1.0),
                    );
                }
            }
        }
        self.rows = rows;
        if let Some(card) = self.tile(ui, 0) {
            let value = trait_id(ui, "_Value");
            ui.set_number(card, value, input.skill as f32);
        }
        self.input = Some(input);
        ui.refresh();
    }

    /// `DoEnter` on a line (`007b6120` case 0xe) with the list's choice as
    /// it is: the scroll knob turns when the chosen line's `listindex`
    /// (none chosen: 0) isn't the one it last turned for, then the line's
    /// details.
    fn entered(&mut self, ui: &mut Ui) -> Vec<Action> {
        let mut out = Vec::new();
        let index = self
            .list
            .selected
            .and_then(|i| self.list.rows.get(i).copied())
            .map_or(0, |tile| ui.number(tile, t::LISTINDEX) as i32);
        if index != self.knob_index {
            out.push(Action::Sound("UIPipBoyScroll".into()));
            out.push(Action::ScrollKnob {
                down: self.knob_index < index,
            });
            self.knob_index = index;
        }
        self.chosen_line(ui);
        out
    }

    /// The pointer onto a tile: a line becomes the list's choice (the
    /// interface's list box, `00717e70`), then `DoEnter` (`007b6120`).
    pub fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) -> Vec<Action> {
        if id != LINE_ID {
            return Vec::new();
        }
        let Some(index) = self.list.index_of(tile) else {
            return Vec::new();
        };
        self.list.choose(ui, Some(index));
        self.entered(ui)
    }

    /// The pointer off a tile: the list lets its choice go when it was
    /// that line (`00717ef0`), then `DoLeave` (`007b6a00` case 0xe): the
    /// repaired card (9) hidden, and with more than one line "choose an
    /// item" (8) too.
    pub fn unmouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        let index = self.list.index_of(tile);
        if index.is_some() && self.list.selected == index {
            self.list.choose(ui, None);
        }
        if id != LINE_ID {
            return;
        }
        if let Some(fixed) = self.tile(ui, 9) {
            ui.set_number(fixed, t::VISIBLE, 0.0);
        }
        if self.list.rows.len() > 1 {
            if let Some(choose) = self.tile(ui, 8) {
                ui.set_number(choose, t::VISIBLE, 0.0);
            }
        }
        ui.refresh();
    }

    /// A click on a line (`007b5b40` case 0xe; Enter clicks the chosen
    /// line): mended with the chosen line when the clicked line's text
    /// (`007b5370`: its first child) has alpha 255, the chosen line isn't
    /// the item and the repaired card (9) shows. Cancel (case 0xc) is
    /// [`super::Pipboy::cancel_repair`]'s.
    pub fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>) -> Vec<Action> {
        let mut out = Vec::new();
        let Some(tile) = tile.filter(|_| id == LINE_ID) else {
            return out;
        };
        let bright = ui.tiles[tile]
            .children
            .first()
            .copied()
            .is_some_and(|text| ui.number(text, t::ALPHA) >= 255.0);
        let shown = self
            .tile(ui, 9)
            .is_some_and(|fixed| ui.number(fixed, t::VISIBLE) != 0.0);
        let (Some(row), Some(input)) = (
            self.list.selected.and_then(|i| self.rows.get(i)),
            self.input.as_ref(),
        ) else {
            return out;
        };
        if bright && !row.chosen && shown {
            out.push(Action::Repair {
                chosen: input.chosen,
                part: row.form,
            });
            out.push(Action::Sound(REPAIR_SOUND.into()));
        }
        out
    }

    /// The line chosen now (`007b6120`).
    fn chosen_line(&mut self, ui: &mut Ui) {
        let Some(row) = self.list.selected.and_then(|i| self.rows.get(i)).cloned() else {
            return;
        };
        let line_alpha = trait_id(ui, "_line_alpha");
        let button = self.tile(ui, 11);
        if let Some(b) = button {
            let top = if row.armour { 50.0 } else { 75.0 };
            let s = if row.condition >= top {
                text(ui, "sMaintainItem")
            } else {
                text(ui, "sRepairItem")
            };
            ui.set_string(b, t::STRING, &s);
        }
        let (choose, fixed) = (self.tile(ui, 8), self.tile(ui, 9));
        if row.chosen {
            if self.rows.len() > 1 {
                if let (Some(c), Some(f)) = (choose, fixed) {
                    let s = text(ui, "sSelectItemToRepair");
                    ui.set_string(c, t::STRING, &s);
                    ui.set_number(f, t::VISIBLE, 0.0);
                    ui.set_number(c, t::VISIBLE, 1.0);
                    if !self.anims.moving(c, t::ALPHA) {
                        self.anims.pulse(c, t::ALPHA, 255.0, 64.0, 2.0, self.now);
                    }
                }
            }
            if let Some(b) = button {
                ui.set_number(b, line_alpha, 128.0);
            }
            ui.refresh();
            return;
        }
        if let Some(b) = button {
            ui.set_number(b, line_alpha, 255.0);
        }
        if let Some(f) = fixed {
            ui.set_number(f, t::USER0, row.mends_to);
            ui.set_number(f, t::VISIBLE, 1.0);
        }
        if let Some(c) = choose {
            ui.set_number(c, t::VISIBLE, 0.0);
        }
        if let Some(card) = self.tile(ui, 10) {
            Self::stat_card(ui, card, row.stat_after.as_ref());
        }
        let better = |a: f32, b: f32| ((a - b).max(0.0) + 0.5) as i32;
        let now = self.input.as_ref().map_or(0.0, |i| i.condition / 100.0);
        if let Some(t6) = self.tile(ui, 6) {
            let n = better(row.mends_to * 100.0, now * 100.0);
            ui.set_string(t6, t::STRING, &format!("+{n}%"));
        }
        if let Some(t7) = self.tile(ui, 7) {
            let value = |s: Option<&Stat>| s.and_then(|s| s.value).unwrap_or(0) as f32;
            let before = value(self.input.as_ref().and_then(|i| i.stat.as_ref()));
            let n = better(value(row.stat_after.as_ref()), before);
            ui.set_string(t7, t::STRING, &format!("+{n}"));
        }
        ui.refresh();
    }

    /// Hides it.
    pub fn hide(&mut self, ui: &mut Ui) {
        ui.set_number(self.menu, t::VISIBLE, 0.0);
        if let Some(c) = self.tile(ui, 8) {
            self.anims.stop(c, t::ALPHA);
        }
        self.input = None;
        ui.refresh();
    }

    /// One frame: the pulsing text.
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        self.now = now;
        self.anims.step(ui, now);
    }

    /// A key: Up and Down choose a line (then `DoEnter`: the knob), Enter
    /// clicks the chosen line (`007b5b40` case 0xe). Returns what the game
    /// should do; Cancel is [`super::Pipboy::cancel_repair`]'s.
    pub fn key(&mut self, ui: &mut Ui, key: Key) -> Vec<Action> {
        match key {
            Key::Up | Key::Down => {
                let before = self.list.selected;
                self.list.step(ui, if key == Key::Down { 1 } else { -1 });
                if self.list.selected == before {
                    return Vec::new();
                }
                self.entered(ui)
            }
            Key::Activate => {
                let tile = self
                    .list
                    .selected
                    .and_then(|i| self.list.rows.get(i).copied());
                self.click(ui, LINE_ID, tile)
            }
            _ => Vec::new(),
        }
    }

    /// Whether, after a repair, it closes (`007b5b40`): the item at 99% or
    /// more, or only it left to choose.
    pub fn done_after(input: &RepairInput) -> bool {
        input.condition >= 99.0 || input.rows.iter().all(|r| r.chosen)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::pipboy::tests::ui;

    /// `repair_menu.xml` cut down to the tiles the code finds by `id`, the
    /// list laid out for the pointer: lines 30 high from y 75.
    pub(crate) const MENU: &str = r#"<menu name="RepairMenu"><locus>&true;</locus>
      <_PCButton_E> RM_CancelButton </_PCButton_E>
      <rect name="RM_Headline_PlayerSkillInfo"><id>0</id><_Title></_Title><_Value></_Value></rect>
      <hotrect name="RM_RepairList"><id>1</id><x>0</x><y>75</y><width>430</width><height>468</height>
        <locus>&true;</locus><target>&false;</target><wheelable>&true;</wheelable>
        <_scroll_delta>0</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
        <rect name="RM_LeftBracket"><id>2</id><y>0</y><height>0</height></rect>
      </hotrect>
      <image name="RM_ItemIcon"><id>3</id></image>
      <rect name="RM_BrokenItemHealth"><id>4</id><user0>0</user0><visible>&false;</visible></rect>
      <rect name="RM_BrokenItemStat"><id>5</id><_Title></_Title><_Value></_Value></rect>
      <text name="RM_HealthImprovementText"><id>6</id><string></string></text>
      <text name="RM_StatImprovementText"><id>7</id><string></string></text>
      <text name="RM_ChooseItemText"><id>8</id><string></string><visible>&false;</visible></text>
      <rect name="RM_FixedItemHealth"><id>9</id><user0>0</user0><visible>&false;</visible></rect>
      <rect name="RM_FixedItemStat"><id>10</id><_Title></_Title><_Value></_Value></rect>
      <image name="RM_RepairButton"><id>11</id><string></string><_line_alpha>255</_line_alpha></image>
      <image name="RM_CancelButton"><id>12</id><x>900</x><y>700</y><width>50</width><height>20</height>
        <visible>&true;</visible><target>&true;</target></image>
      <template name="RM_RepairListTemplate"><hotrect name="RM_RepairListTemplateRect"><height>30</height>
        <width>393</width><y><copy src="me()" trait="_y"/></y><target>&true;</target>
        <_IsRepairItem>&false;</_IsRepairItem><user0>0</user0>
        <text name="ListItemText"><string><copy src="parent()" trait="string"/></string></text>
        <image name="RM_Template_Meter"><id>15</id><width>0</width></image>
        <image name="RM_Template_ItemMarker"><visible>&false;</visible></image>
      </hotrect></template>
    </menu>"#;

    pub(crate) fn menu(ui: &mut Ui) -> RepairMenu {
        RepairMenu::load(ui, &mut |p| (p == FILE).then(|| MENU.as_bytes().to_vec())).unwrap()
    }

    fn stat(v: i32) -> Option<Stat> {
        Some(Stat {
            title: "sInventoryDamage",
            value: Some(v),
        })
    }

    fn row(form: u32, condition: f32, chosen: bool, mends_to: f32, after: i32) -> RepairRow {
        RepairRow {
            form,
            name: format!("Thing {form:X}"),
            condition,
            armour: false,
            equipped: false,
            chosen,
            mends_to,
            stat_after: stat(after),
        }
    }

    pub(crate) fn input() -> RepairInput {
        RepairInput {
            chosen: 0x10,
            condition: 30.0,
            icon: None,
            skill: 15,
            stat: stat(6),
            rows: vec![
                row(0x20, 40.0, false, 0.45, 8),
                row(0x10, 30.0, true, 0.3875, 7),
                row(0x30, 80.0, false, 0.85, 9),
            ],
        }
    }

    fn text_of(ui: &mut Ui, m: &RepairMenu, id: i32) -> String {
        let tile = by_id(ui, m.menu, id).unwrap();
        ui.string(tile, t::STRING).unwrap_or_default()
    }

    /// `007b7020`, `007b57f0`, `007b5950`, `007b6120`: the chosen item
    /// first, then the best condition first; on it, "choose an item"; on
    /// another, what mending with it does. (With a pad: the first line
    /// chosen as it opens.)
    #[test]
    fn lines_and_what_each_does() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        assert!(m.open(&mut ui, input(), true).is_empty());
        let forms: Vec<u32> = m.rows.iter().map(|r| r.form).collect();
        assert_eq!(forms, vec![0x10, 0x30, 0x20]);
        let marked = ui.names.lookup("_IsRepairItem").unwrap();
        assert_eq!(ui.number(m.list.rows[0], marked), 1.0);
        let value = ui.names.lookup("_Value").unwrap();
        let skill = by_id(&ui, m.menu, 0).unwrap();
        assert_eq!(ui.number(skill, value), 15.0);
        let broken = by_id(&ui, m.menu, 4).unwrap();
        assert!((ui.number(broken, t::USER0) - 0.3).abs() < 1e-6);
        assert_eq!(text_of(&mut ui, &m, 8), "CHOOSE ITEM TO REPAIR WITH");
        assert_eq!(text_of(&mut ui, &m, 11), "Repair");
        let fixed = by_id(&ui, m.menu, 9).unwrap();
        assert_eq!(ui.number(fixed, t::VISIBLE), 0.0);

        // The next line: `DoEnter`, the knob a notch on with its click.
        let actions = m.key(&mut ui, Key::Down);
        assert_eq!(
            actions,
            vec![
                Action::Sound("UIPipBoyScroll".into()),
                Action::ScrollKnob { down: true }
            ]
        );
        assert_eq!(m.knob_index, 1);
        assert_eq!(ui.number(fixed, t::VISIBLE), 1.0);
        assert!((ui.number(fixed, t::USER0) - 0.85).abs() < 1e-6);
        assert_eq!(text_of(&mut ui, &m, 6), "+55%");
        assert_eq!(text_of(&mut ui, &m, 7), "+3");
        // A line's thing at 80%: "Maintain".
        assert_eq!(text_of(&mut ui, &m, 11), "Maintain");
        let actions = m.key(&mut ui, Key::Activate);
        assert_eq!(
            actions,
            vec![
                Action::Repair {
                    chosen: 0x10,
                    part: 0x30
                },
                Action::Sound("UIRepairWeapon".into())
            ]
        );
    }

    /// `007b7020` without a pad: nothing chosen as it opens, so neither card
    /// shows and Enter does nothing; Down chooses the first line, the item
    /// itself, which can't mend itself (`007b5b40`: the chosen line isn't
    /// the item).
    #[test]
    fn without_a_pad_nothing_is_chosen_at_first() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        assert!(m.open(&mut ui, input(), false).is_empty());
        assert_eq!(m.list.selected, None);
        let (choose, fixed) = (
            by_id(&ui, m.menu, 8).unwrap(),
            by_id(&ui, m.menu, 9).unwrap(),
        );
        assert_eq!(ui.number(choose, t::VISIBLE), 0.0);
        assert_eq!(ui.number(fixed, t::VISIBLE), 0.0);
        assert!(m.key(&mut ui, Key::Activate).is_empty());
        // The first line's listindex is 0, as the knob's: no turn.
        assert!(m.key(&mut ui, Key::Down).is_empty());
        assert_eq!(m.list.selected, Some(0));
        assert_eq!(ui.number(choose, t::VISIBLE), 1.0);
        assert!(m.key(&mut ui, Key::Activate).is_empty());
    }

    /// The pointer (`007b6120`, `007b6a00`, `007b5b40`): onto a line it's
    /// chosen and shown, the knob turning; a click mends with it; a line
    /// whose text isn't at full alpha can't be clicked; off it the cards
    /// hide and the choice goes.
    #[test]
    fn the_pointer_chooses_and_clicks_lines() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        m.open(&mut ui, input(), false);
        let (choose, fixed) = (
            by_id(&ui, m.menu, 8).unwrap(),
            by_id(&ui, m.menu, 9).unwrap(),
        );
        let line = m.list.rows[2];
        let out = m.mouseover(&mut ui, LINE_ID, line);
        assert_eq!(
            out,
            vec![
                Action::Sound("UIPipBoyScroll".into()),
                Action::ScrollKnob { down: true }
            ]
        );
        assert_eq!(m.list.selected, Some(2));
        assert_eq!(ui.number(fixed, t::VISIBLE), 1.0);
        assert!((ui.number(fixed, t::USER0) - 0.45).abs() < 1e-6);
        assert_eq!(
            m.click(&mut ui, LINE_ID, Some(line)),
            vec![
                Action::Repair {
                    chosen: 0x10,
                    part: 0x20
                },
                Action::Sound("UIRepairWeapon".into())
            ]
        );
        let text = ui.tiles[line].children[0];
        ui.set_number(text, t::ALPHA, 128.0);
        assert!(m.click(&mut ui, LINE_ID, Some(line)).is_empty());
        ui.set_number(text, t::ALPHA, 255.0);
        // Off it: the card hidden (and "choose an item", more than one
        // line), nothing chosen, so a click does nothing.
        m.unmouseover(&mut ui, LINE_ID, line);
        assert_eq!(m.list.selected, None);
        assert_eq!(ui.number(fixed, t::VISIBLE), 0.0);
        assert_eq!(ui.number(choose, t::VISIBLE), 0.0);
        // Back to the item's own line: the knob the other way.
        let out = m.mouseover(&mut ui, LINE_ID, m.list.rows[0]);
        assert_eq!(
            out,
            vec![
                Action::Sound("UIPipBoyScroll".into()),
                Action::ScrollKnob { down: false }
            ]
        );
        assert_eq!(ui.number(choose, t::VISIBLE), 1.0);
        let first = m.list.rows[0];
        assert!(m.click(&mut ui, LINE_ID, Some(first)).is_empty());
    }

    /// After a repair (`007b5b40` → `0071a5c0`, `007b7020(0)`): the used
    /// line gone, the choice on the line that came after it, found again in
    /// the new order; the used line the last: the one before it.
    #[test]
    fn after_a_repair_the_next_line_is_chosen() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        m.open(&mut ui, input(), true);
        m.key(&mut ui, Key::Down);
        assert_eq!(m.rows[1].form, 0x30);
        // Mended with 0x30: it's gone, the item at 85%.
        let mut after = input();
        after.condition = 85.0;
        after.rows.retain(|r| r.form != 0x30);
        for r in after.rows.iter_mut().filter(|r| r.chosen) {
            r.condition = 85.0;
        }
        m.refill(&mut ui, after.clone());
        let forms: Vec<u32> = m.rows.iter().map(|r| r.form).collect();
        assert_eq!(forms, vec![0x10, 0x20]);
        assert_eq!(m.list.selected, Some(1));
        // The next line's condition moved too (a kind's condition is kept
        // per holder here): found again by its kind and wear.
        let mut m2 = menu(&mut ui);
        m2.open(&mut ui, input(), true);
        m2.key(&mut ui, Key::Down);
        let mut moved = input();
        moved.rows.retain(|r| r.form != 0x30);
        for r in moved.rows.iter_mut().filter(|r| !r.chosen) {
            r.condition = 55.0;
        }
        m2.refill(&mut ui, moved);
        assert_eq!(m2.list.selected, Some(1));
        assert_eq!(m2.rows[1].form, 0x20);
        // Mended with the last line, 0x20: the one before it, the item.
        let mut last = after;
        last.rows.retain(|r| r.form != 0x20);
        last.rows.push(row(0x40, 20.0, false, 0.9, 9));
        m.refill(&mut ui, last);
        assert_eq!(m.rows[m.list.selected.unwrap()].form, 0x10);
    }

    /// `007b6aa0`: the brackets on the item's line, the height 1 less.
    #[test]
    fn the_brackets_follow_the_items_line() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        m.open(&mut ui, input(), false);
        ui.refresh();
        let bracket = by_id(&ui, m.menu, 2).unwrap();
        let line = m.list.rows[0];
        let h = ui.number(line, t::HEIGHT);
        assert_eq!(h, 30.0);
        assert_eq!(ui.number(bracket, t::HEIGHT), h - 1.0);
        assert_eq!(ui.number(bracket, t::Y), ui.number(line, t::Y));
    }

    /// `007b5b40`: after a repair it closes at 99% or with only the item
    /// left.
    #[test]
    fn when_it_closes() {
        let mut i = input();
        assert!(!RepairMenu::done_after(&i));
        i.condition = 99.0;
        assert!(RepairMenu::done_after(&i));
        let mut i = input();
        i.rows.retain(|r| r.chosen);
        assert!(RepairMenu::done_after(&i));
    }
}
