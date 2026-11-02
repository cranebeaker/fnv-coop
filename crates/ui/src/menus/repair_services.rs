//! Merchants' repairs (`menus\repair_services_menu.xml`, class
//! `RepairServicesMenu` 1058, vtable `01075db4` in FalloutNV.exe), opened
//! by `ShowRepairMenu` on a vendor. The rules (what's listed, costs, what
//! the vendor mends to) are `world::repair`'s. Read from the code:
//!
//! * opening (`007b7570(vendor)`): the 16 tiles (ids 0 to 15, or it
//!   fails), the stat cards' `_Value` set 1 then 0 (`00a01290`), the
//!   vendor's Repair kept (`+0x9c`, `0066ef20`), the list (id 2,
//!   `RSM_RepairListTemplate`), filled; with `[General] sLanguage`
//!   "ENGLISH" the title, the empty list's text, the condition cards'
//!   titles and the buttons get the strings the file already gives them.
//! * filling (`007b8b30`): the total 0; a line per thing
//!   (`world::repair::service_lines`, `007b7b40`), its name; each line
//!   (`007b7920`): id 0x11, `_RepairCost` its cost (added to the total),
//!   alpha 255 when it costs something and the player can pay it, else
//!   128, on its text, its meter (id 0x12) and its last child (the worn
//!   mark), `_CanRepair` alpha 255, the meter's width condition × 0.6, the
//!   mark shown when it's worn; sorted (`007b7c40`); the caps line
//!   "`sInventoryCaps`   n" ("n+" at `iCapsLimit`, 1000000), the vendor's
//!   "`sRepairSkill`   n", Repair All "`sRepairAllItems`" with the total,
//!   usable when the total is above 0 and the player can pay it.
//! * the pointer on a line (`007b82f0`): the repair button usable as the
//!   line is, the picture (`sMissingImage` without one) and the stats
//!   shown: the condition card's `user0` the condition, the stat card the
//!   item's figure (`world::repair::shown_stat`); when the vendor mends to
//!   more (both to thousandths), the repaired card shown with the target
//!   and the figure there, "+n%" and "+n" (the differences, at least 0,
//!   rounded), the cost "`sRepairCost`" bright when it can be paid (alpha
//!   255, else 128); else the error "`sCantRepairPastMax`" with the target
//!   in percent and the repaired card hidden. Off it (`007b8ae0`): the
//!   button usable, the picture and the stats hidden.
//! * a click (`007b7d80`): a line that can be paid for is repaired and
//!   paid (`UIRepairWeapon`), the list filled again; one that can't,
//!   `UIVATSInsufficientAP`. Repair All (14) repairs every line that can
//!   be paid for and pays the total, `UIRepairWeapon`, filled again. Done
//!   (15) closes it.
//!
//! Not here: the dialogue topic handed back on closing (`007b78e0`:
//! `0061a2d0(5, 5)` to the dialogue menu), the menu's fade.

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::repair::{compare_service_lines, ServiceLine, Stat};

/// The menu's file.
pub const FILE: &str = "menus\\repair_services_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1058;
/// Its list's template.
pub const LIST_TEMPLATE: &str = "RSM_RepairListTemplate";
/// A line's id (`007b7920`).
pub const LINE_ID: i32 = 0x11;
/// A line's meter (`RSM_Template_Meter`).
pub const METER_ID: i32 = 0x12;
/// Repair All and Done.
pub const REPAIR_ALL: i32 = 14;
pub const DONE: i32 = 15;
/// How many tile ids the menu needs (`007b78a0`: 0 to 15).
pub const TILE_COUNT: usize = 16;
/// `iCapsLimit`'s default.
pub const CAPS_LIMIT: i32 = 1_000_000;

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Mend one thing for its cost (`007b7f70`, `008924e0`).
    Repair { form: u32, cost: i32 },
    /// Mend every thing that can be paid for, and pay the total.
    RepairAll { forms: Vec<u32>, total: i32 },
    /// Close it.
    Close,
}

/// The merchant's repair menu.
#[derive(Debug)]
pub struct RepairServicesMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    pub list: ListBox,
    /// The lines, by the list items' values.
    pub lines: Vec<ServiceLine>,
    /// The vendor's Repair (`+0x9c`).
    pub skill: i32,
    /// What the vendor mends to, 0..1.
    pub target: f32,
    /// The player's caps.
    pub caps: i32,
    pub caps_limit: i32,
    /// Every line's cost (`+0x98`).
    pub total: i32,
    /// The line under the pointer.
    selected: Option<usize>,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

fn custom(ui: &mut Ui, name: &str) -> i32 {
    ui.names.lookup_or_add(name).unwrap_or(0)
}

/// `sCantRepairPastMax`'s and the like's `%d` / `%.0f` / `%s`, in order.
fn printf(format: &str, values: &[String]) -> String {
    let mut out = String::new();
    let mut values = values.iter();
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        if chars.peek() == Some(&'%') {
            chars.next();
            out.push('%');
            continue;
        }
        // Flags, width and precision, then the conversion.
        while chars.peek().is_some_and(|c| !c.is_ascii_alphabetic()) {
            chars.next();
        }
        chars.next();
        if let Some(v) = values.next() {
            out.push_str(v);
        }
    }
    out
}

impl RepairServicesMenu {
    pub fn new(menu: TileId) -> RepairServicesMenu {
        RepairServicesMenu {
            menu,
            tiles: [None; TILE_COUNT],
            list: ListBox::default(),
            lines: Vec::new(),
            skill: 0,
            target: 0.0,
            caps: 0,
            caps_limit: CAPS_LIMIT,
            total: 0,
            selected: None,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: i32) -> Option<TileId> {
        self.tiles.get(id as usize).copied().flatten()
    }

    /// Opens it (`007b7570`); false when the file lacks one of its tiles.
    pub fn open(
        &mut self,
        ui: &mut Ui,
        skill: i32,
        target: f32,
        lines: Vec<ServiceLine>,
        caps: i32,
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        let value = custom(ui, "_Value");
        for id in [6, 11] {
            if let Some(card) = self.tile(id) {
                ui.set_number(card, value, 0.0);
            }
        }
        self.skill = skill;
        self.target = target;
        if let Some(list) = self.tile(2) {
            self.list = ListBox::new(ui, list, LIST_TEMPLATE);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.fill(ui, lines, caps);
        true
    }

    /// Fills the list (`007b8b30`) with the player's things and caps now.
    pub fn fill(&mut self, ui: &mut Ui, lines: Vec<ServiceLine>, caps: i32) {
        self.list.clear(ui);
        self.selected = None;
        self.total = 0;
        self.caps = caps;
        self.lines = lines;
        let alpha_of = |l: &ServiceLine| {
            if l.cost > 0 && l.cost <= caps {
                255.0
            } else {
                128.0
            }
        };
        let cost = custom(ui, "_RepairCost");
        let can = custom(ui, "_CanRepair");
        for i in 0..self.lines.len() {
            let line = self.lines[i].clone();
            let Some(tile) = self.list.add(ui, self.menu, i as i32, Some(&line.name)) else {
                continue;
            };
            ui.set_number(tile, t::ID, LINE_ID as f32);
            ui.set_number(tile, cost, line.cost as f32);
            self.total += line.cost;
            let alpha = alpha_of(&line);
            let children = ui.tiles[tile].children.clone();
            let meter = children
                .iter()
                .copied()
                .find(|&c| ui.number(c, t::ID) as i32 == METER_ID);
            let first = children.first().copied();
            let last = children.last().copied();
            for c in [last, meter, first].into_iter().flatten() {
                ui.set_number(c, t::ALPHA, alpha);
            }
            let index = ui.number(tile, t::LISTINDEX);
            ui.set_number(tile, t::VISIBLE, if index >= 0.0 { 1.0 } else { 0.0 });
            if let Some(m) = meter {
                ui.set_number(m, t::WIDTH, line.condition * 0.6);
            }
            ui.set_number(tile, can, if alpha == 255.0 { 1.0 } else { 0.0 });
            if line.equipped {
                if let Some(mark) = last {
                    ui.set_number(mark, t::VISIBLE, 1.0);
                }
            }
        }
        // `007b7c40` over the lines the items stand for.
        let by_tile: std::collections::HashMap<TileId, usize> = self
            .list
            .items
            .iter()
            .map(|i| (i.tile, i.value as usize))
            .collect();
        let lines = self.lines.clone();
        self.list.sort(ui, &|_, a, b| {
            let line = |tile: TileId| by_tile.get(&tile).and_then(|&i| lines.get(i));
            match (line(a), line(b)) {
                (Some(x), Some(y)) => compare_service_lines(x, y) < 0,
                _ => false,
            }
        });
        self.labels(ui);
        ui.refresh();
    }

    /// The caps line, the vendor's skill and Repair All (`007b8b30`'s
    /// end).
    fn labels(&mut self, ui: &mut Ui) {
        let text = |ui: &Ui, name: &str| ui.setting_text(name).unwrap_or_default();
        if let Some(caps) = self.tile(1) {
            let label = text(ui, "sInventoryCaps");
            let s = if self.caps > self.caps_limit {
                format!("{label}   {}+", self.caps_limit)
            } else {
                format!("{label}   {}", self.caps)
            };
            ui.set_text(caps, t::STRING, &s);
        }
        if let Some(skill) = self.tile(0) {
            let s = format!("{}   {}", text(ui, "sRepairSkill"), self.skill);
            ui.set_text(skill, t::STRING, &s);
        }
        if let Some(all) = self.tile(REPAIR_ALL) {
            let s = printf(&text(ui, "sRepairAllItems"), &[self.total.to_string()]);
            ui.set_text(all, t::STRING, &s);
            let usable = self.total > 0 && self.total <= self.caps;
            ui.set_number(all, t::TARGET, if usable { 1.0 } else { 0.0 });
        }
    }

    /// The line a list item stands for.
    fn line_of(&self, tile: TileId) -> Option<&ServiceLine> {
        let v = self.list.value_of(tile)?;
        self.lines.get(v as usize)
    }

    /// A stat card's title and value.
    fn stat_card(ui: &mut Ui, card: TileId, stat: Option<&Stat>) {
        let title = custom(ui, "_Title");
        let value = custom(ui, "_Value");
        let Some(stat) = stat else {
            return;
        };
        let s = ui.setting_text(stat.title).unwrap_or_default();
        ui.set_string(card, title, &s);
        match stat.value {
            Some(v) => ui.set_number(card, value, v as f32),
            None => ui.set_string(card, value, "--"),
        }
    }

    /// The stats for the line under the pointer (`007b82f0`).
    fn show_stats(&mut self, ui: &mut Ui, tile: TileId) {
        let Some(line) = self.line_of(tile).cloned() else {
            return;
        };
        let can = custom(ui, "_CanRepair");
        let usable = ui.number(tile, can);
        if let Some(button) = self.tile(13) {
            ui.set_number(button, t::TARGET, usable);
        }
        if let Some(stats) = self.tile(4) {
            ui.set_number(stats, t::VISIBLE, 1.0);
        }
        if let Some(icon) = self.tile(3) {
            let path = if line.icon.is_empty() {
                ui.setting_text("sMissingImage").unwrap_or_default()
            } else {
                line.icon.clone()
            };
            ui.set_string(icon, t::FILENAME, &path);
            ui.set_number(icon, t::VISIBLE, 1.0);
        }
        let condition = line.condition / 100.0;
        if let Some(card) = self.tile(5) {
            ui.set_number(card, t::USER0, condition);
        }
        if let Some(card) = self.tile(6) {
            Self::stat_card(ui, card, line.stat.as_ref());
        }
        let round = |v: f32| world::barter::round_to(v, 0.001);
        let Some(fixed) = self.tile(10) else {
            return;
        };
        if round(condition) >= round(self.target) {
            if let Some(error) = self.tile(9) {
                let s = printf(
                    &ui.setting_text("sCantRepairPastMax").unwrap_or_default(),
                    &[format!("{:.0}", self.target * 100.0), "%".to_string()],
                );
                ui.set_text(error, t::STRING, &s);
            }
            ui.set_number(fixed, t::VISIBLE, 0.0);
            ui.refresh();
            return;
        }
        ui.set_number(fixed, t::USER0, self.target);
        ui.set_number(fixed, t::VISIBLE, 1.0);
        if let Some(card) = self.tile(11) {
            Self::stat_card(ui, card, line.stat_after.as_ref());
        }
        let better = |a: f32, b: f32| (((a - b).max(0.0)) + 0.5) as i32;
        if let Some(text) = self.tile(7) {
            let n = better(self.target * 100.0, condition * 100.0);
            ui.set_text(text, t::STRING, &format!("+{n}%"));
        }
        if let Some(text) = self.tile(8) {
            let value = |s: &Option<Stat>| s.as_ref().and_then(|s| s.value).unwrap_or(0) as f32;
            let n = better(value(&line.stat_after), value(&line.stat));
            ui.set_text(text, t::STRING, &format!("+{n}"));
        }
        if let Some(text) = self.tile(12) {
            let cost = custom(ui, "_RepairCost");
            let c = ui.number(tile, cost) as i32;
            let s = printf(
                &ui.setting_text("sRepairCost").unwrap_or_default(),
                &[c.to_string()],
            );
            ui.set_text(text, t::STRING, &s);
            ui.set_number(text, t::ALPHA, if usable != 0.0 { 255.0 } else { 128.0 });
        }
        ui.refresh();
    }

    /// Closes it.
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        self.requests.push(Request::Close);
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for RepairServicesMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..TILE_COUNT as i32).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007b7d80`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        match id {
            REPAIR_ALL => {
                let forms: Vec<u32> = self
                    .lines
                    .iter()
                    .filter(|l| l.can_repair)
                    .map(|l| l.item.0)
                    .collect();
                self.requests.push(Request::RepairAll {
                    forms,
                    total: self.total,
                });
                self.sounds.push(world::repair::REPAIR_SOUND.to_string());
            }
            DONE => self.close(ui),
            LINE_ID => {
                let Some(tile) = tile.or(self.list.selected) else {
                    return;
                };
                let can = custom(ui, "_CanRepair");
                if ui.number(tile, can) == 0.0 {
                    self.sounds.push(world::repair::CANT_SOUND.to_string());
                    return;
                }
                let Some(line) = self.line_of(tile).cloned() else {
                    return;
                };
                let cost = custom(ui, "_RepairCost");
                let c = ui.number(tile, cost) as i32;
                self.requests.push(Request::Repair {
                    form: line.item.0,
                    cost: c,
                });
                self.sounds.push(world::repair::REPAIR_SOUND.to_string());
            }
            _ => {}
        }
    }

    /// `007b82f0`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id == LINE_ID {
            self.selected = self.list.value_of(tile).map(|v| v as usize);
            self.show_stats(ui, tile);
        }
    }

    /// `007b8ae0`.
    fn unmouseover(&mut self, ui: &mut Ui, id: i32, _tile: TileId) {
        if id != LINE_ID {
            return;
        }
        if let Some(button) = self.tile(13) {
            ui.set_number(button, t::TARGET, 1.0);
        }
        for id in [3, 4] {
            if let Some(tile) = self.tile(id) {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
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
    use esm::FormId;

    fn line(form: u32, name: &str, condition: f32, cost: i32, caps: i32) -> ServiceLine {
        ServiceLine {
            item: FormId(form),
            name: name.into(),
            count: 1,
            form_type: 0x28,
            equipped: false,
            condition,
            health: condition,
            cost,
            can_repair: cost > 0 && cost <= caps,
            icon: String::new(),
            stat: Some(Stat {
                title: "sInventoryDamage",
                value: Some(10),
            }),
            stat_after: Some(Stat {
                title: "sInventoryDamage",
                value: Some(14),
            }),
        }
    }

    fn opened(lines: Vec<ServiceLine>, caps: i32) -> (Ui, RepairServicesMenu) {
        let mut ui = test_support::ui();
        let mut m = RepairServicesMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::repair_services_menu(), &mut m);
        assert!(m.open(&mut ui, 50, 0.7, lines, caps));
        (ui, m)
    }

    fn texts(ui: &mut Ui, m: &RepairServicesMenu) -> Vec<String> {
        m.list
            .shown_items(ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect()
    }

    fn row(ui: &mut Ui, m: &RepairServicesMenu, text: &str) -> TileId {
        m.list
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .find(|l| ui.string(l.tile, t::STRING).as_deref() == Some(text))
            .unwrap()
            .tile
    }

    /// `007b8b30`, `007b7920`, `007b7c40`: the lines in their order, each
    /// with its cost and brightness, the meter's width, the labels and
    /// Repair All.
    #[test]
    fn filling() {
        let lines = vec![
            line(1, "Pistol", 30.0, 80, 50),
            line(2, "Revolver", 50.0, 20, 50),
            line(3, "Rifle", 80.0, 0, 50),
        ];
        let (mut ui, m) = opened(lines, 50);
        assert_eq!(texts(&mut ui, &m), vec!["Revolver", "Pistol", "Rifle"]);
        let pistol = row(&mut ui, &m, "Pistol");
        let text = ui.tiles[pistol].children[0];
        assert_eq!(ui.number(text, t::ALPHA), 128.0);
        let can = ui.names.lookup("_CanRepair").unwrap();
        assert_eq!(ui.number(pistol, can), 0.0);
        let revolver = row(&mut ui, &m, "Revolver");
        assert_eq!(ui.number(revolver, can), 1.0);
        let meter = ui.tiles[revolver].children[2];
        assert!((ui.number(meter, t::WIDTH) - 30.0).abs() < 1e-4);
        assert_eq!(m.total, 100);
        let s = |ui: &mut Ui, id: i32| ui.string(m.tiles[id as usize].unwrap(), t::STRING);
        assert_eq!(s(&mut ui, 0).as_deref(), Some("Repair Skill   50"));
        assert_eq!(s(&mut ui, 1).as_deref(), Some("Caps   50"));
        assert_eq!(s(&mut ui, REPAIR_ALL).as_deref(), Some("Repair All (100)"));
        // 100 caps owed, 50 held: Repair All can't be used.
        assert_eq!(ui.number(m.tiles[14].unwrap(), t::TARGET), 0.0);
    }

    /// `007b82f0`, `007b8ae0`: the pointer on a line shows its stats and
    /// what the vendor makes of it; past the target, the error.
    #[test]
    fn the_stats() {
        let lines = vec![
            line(1, "Pistol", 30.0, 80, 100),
            line(3, "Rifle", 80.0, 0, 100),
        ];
        let (mut ui, mut m) = opened(lines, 100);
        let pistol = row(&mut ui, &m, "Pistol");
        m.mouseover(&mut ui, LINE_ID, pistol);
        let tiles = m.tiles;
        let tile = |id: usize| tiles[id].unwrap();
        assert_eq!(ui.number(tile(4), t::VISIBLE), 1.0);
        assert!((ui.number(tile(5), t::USER0) - 0.3).abs() < 1e-6);
        assert!((ui.number(tile(10), t::USER0) - 0.7).abs() < 1e-6);
        assert_eq!(ui.number(tile(10), t::VISIBLE), 1.0);
        assert_eq!(ui.string(tile(7), t::STRING).as_deref(), Some("+40%"));
        assert_eq!(ui.string(tile(8), t::STRING).as_deref(), Some("+4"));
        assert_eq!(
            ui.string(tile(12), t::STRING).as_deref(),
            Some("Cost: 80 caps")
        );
        assert_eq!(ui.number(tile(12), t::ALPHA), 255.0);
        let value = ui.names.lookup("_Value").unwrap();
        assert_eq!(ui.number(tile(6), value), 10.0);
        assert_eq!(ui.number(tile(11), value), 14.0);
        m.unmouseover(&mut ui, LINE_ID, pistol);
        assert_eq!(ui.number(tile(4), t::VISIBLE), 0.0);

        let rifle = row(&mut ui, &m, "Rifle");
        m.mouseover(&mut ui, LINE_ID, rifle);
        assert_eq!(ui.number(tile(10), t::VISIBLE), 0.0);
        assert_eq!(ui.number(tile(9), t::VISIBLE), 1.0);
        assert_eq!(
            ui.string(tile(9), t::STRING).as_deref(),
            Some("CANNOT REPAIR PAST 70%")
        );
        assert_eq!(ui.number(tile(13), t::TARGET), 0.0);
    }

    /// `007b7d80`: a line that can be paid for asks for its repair; one
    /// that can't only sounds; Repair All asks for every one that can;
    /// Done closes.
    #[test]
    fn clicks() {
        let lines = vec![
            line(1, "Pistol", 30.0, 80, 50),
            line(2, "Revolver", 50.0, 20, 50),
        ];
        let (mut ui, mut m) = opened(lines, 50);
        let pistol = row(&mut ui, &m, "Pistol");
        m.click(&mut ui, LINE_ID, Some(pistol), 0.0);
        assert!(m.requests.is_empty());
        assert_eq!(m.sounds, vec!["UIVATSInsufficientAP"]);
        let revolver = row(&mut ui, &m, "Revolver");
        m.click(&mut ui, LINE_ID, Some(revolver), 0.0);
        assert_eq!(m.requests, vec![Request::Repair { form: 2, cost: 20 }]);
        m.requests.clear();
        m.click(&mut ui, REPAIR_ALL, None, 0.0);
        assert_eq!(
            m.requests,
            vec![Request::RepairAll {
                forms: vec![2],
                total: 100
            }]
        );
        m.requests.clear();
        m.click(&mut ui, DONE, None, 0.0);
        assert_eq!(m.requests, vec![Request::Close]);
        assert!(m.closed);
    }
}
